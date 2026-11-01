# Reading a file — Blender 4.5, 5.0, 5.1, 5.2 and 5.3 side by side

What a `.blend` file gives up to something reading it, what can be written back into one, and what changes between the five versions.

## The machines behind the tests

Four computers, each written out in full in the equipment section of `Preparation/Blender Research/Method.md`. This is only enough to tell them apart.

- **The Creator's Mac.** A MacBook Pro with an M1 Pro chip, Apple's own graphics, no second card, and a screen. Blender 4.5 to 5.2 were tested on it under macOS 26.5.2, in 4.5.12 LTS, 5.0.1, 5.1.2 and 5.2.0 LTS. Blender 5.3 was tested on it under macOS 26.6.2, in **5.3.0 Beta** built 2026-10-16. **5.2.2 LTS** controls also ran on this Mac.
- **A rented Linux machine with an RTX 4090**, twelve cores, and a display of its own — the **first** rented Linux machine in the equipment section. Questions 1 to 8 were tested on this one.
- **A rented Linux machine with an RTX 4080 SUPER**, twenty-four cores, and **no display server at all** — the **fourth** there. Question 9, and the test results that answer parts of questions 5 and 7, were run on this one.
- **A rented Linux machine with an RTX 5060**, twenty-three cores, and **no display server at all** — the **fifth** there. Questions 10 and 11 were run on this one.

**Those are three different rented computers, and no figure from one may ever be set against a figure from another.** Where an answer below says *both machines*, it means the Creator's Mac and whichever rented machine ran that question's tests. Every rented machine was shared with other work while it ran, and the Mac's runs did not measure it, so **nothing here says how long anything took or how much memory anything used.**

**Blender 5.3 was run on the Creator's Mac alone; no rented machine ran anything for it.** So wherever a sentence below says *all four versions*, *every version*, *the four* or *both machines* without naming 5.3, it means 4.5 to 5.2 and the machines they ran on. 5.3 is named wherever it is meant, and *all five* includes it. A 5.3 result is set against an earlier version's Mac result. Where the earlier versions were tested on a rented machine alone, a 5.3 result beside theirs rests on a different computer: an agreement there is said to cross machines, and a disagreement there would be not established, never a difference between versions. **A result marked 5.2.2 is that same-day control on the Mac**; a 5.2 result without the number came from 5.2.0.

Two standing limits. **A result settles only the tested machines and test conditions.** **Windows was never tested in this topic, and Linux was never tested for 5.3.** Neither gap is a version difference. No result here establishes another graphics-card make or a machine with several cards. The render device matters too: question 11's file-change renders used Cycles on the processor.

## The answer in short

Three lists — where Blender itself differs between versions, what is not established, and where only the documentation differs. Every question's full answer follows them, in order.

### Where Blender itself differs between versions

Release-note changes are **documented**. Test differences are **observed** only on the machines named under each question; 5.3 ran on the Mac alone in a beta build.

- **1 — compression became the saving default at 5.0.**
- **1 — file and block headers changed at 5.0.**
- **1 — compressed opening bytes differed in the saves tested; 5.2.0's account is disputed.** The same-Mac 5.2.2–5.3 test also differed.
- **1, 8 and 9 — the loader offers the file's version from 5.0 and its linked-file list from 5.2; its data categories also changed at 5.0.**
- **1 — preview tools split at 5.1 in 4.5–5.2.** 5.3's position under those tools is not established.
- **3 — the render switch's declared default and the route to its curve changed at 5.0.** The test also reported a changed EEVEE identifier.
- **3 — from 5.3, the compositor renders scenes referenced inside node groups.** Release note only.
- **4 and 9 — the File Output node and the scene's compositor-tree property changed at 5.0.**
- **4 — the File Output node's starting format changed at 5.0.**
- **4 — slashes in input names stopped working in 5.0 and were restored in 5.1.**
- **4 — the node's assembled-path panel appears from 5.0; its File Extensions switch appears from 5.2.**
- **4 — `{scene_name}` became a path variable at 5.0.**
- **4 — the documented render-output-path default changes to `//` at 5.2.** Untested.
- **4 — the switch to stop saving the scene's render output was added in 5.2 and is documented in 5.3.**
- **4 — 5.3 adds bake-folder templates and project variables.** Release notes; untested.
- **5, 9 and 11 — the one-pass path visitor was added in 5.0 and extended in 5.2.**
- **5 and 9 — the list of every data-block was added in 5.2.**
- **5 and 9 — writing already-relative paths into a library was fixed in 5.1.** 5.3 agrees on the Mac; earlier tests ran on Linux.
- **5 — 5.1 fixed clearing the external multi-resolution path when repacking.** Release note only; the path could not be set for a test.
- **6 — the window manager's readable reports list was added in 5.2.** It did not hold the missing-files report in the tests.
- **8 — loading messages gained timestamps and channels at 5.0.**
- **8 — archive libraries for packed linked data were added at 5.0.** Release note; untested.
- **8 — 5.3 saves only directly linked packed data.** Release note; untested.
- **9 — the sequencer change-path command's requirements changed at 5.0; its refusal wording changed again at 5.1.** Mac tests only.
- **11 — a changed sequencer image-strip file reaches the next render unaided in 4.5 and does not in 5.0–5.3.**
- **11 — the texture-cache settings and commands are absent before 5.2 and present in 5.2 and 5.3.**
- **11 — the error for a mistyped option differs between 5.0 and 5.1–5.2.** 5.3 was not asked.

### What is not established

These are gaps, not evidence of a version change. Each question holds its full limits.

- **All questions — Windows, and Linux for 5.3.** Other machines, graphics-card makes and multiple-card arrangements are not established by these tests either.
- **1 — what outside programs can read; Linux file-browser previews; 5.3 file-browser previews; preview-tool registration; and the disputed compressed opening bytes in 5.2.0.** Whether the asset-catalog and registration documentation changed is also unestablished.
- **2 — a general count of simultaneous Blenders, and reading or saving during a render in 5.3.** File-handle lifetime, shared temporary files and slowdown also remain open.
- **3 — drivers, scripts changing the switch during animation, other scenes reached through the compositor, and the window checks outside the Mac.** The switched-off-layer and direct-format tests were run only in 5.3; its hand-pressed warning was not tested.
- **4 — a complete list of write destinations, a script route to the sidebar's assembled path, the panel's Linux contents, and 5.3 bake templates and project-variable resolution in practice.**
- **5 and 9 — outside tools, the multi-resolution path, the absolute sweep's reach over the seven unchanged settings, and tests on the other operating system.**
- **5 — the reach of the write-only-to-the-open-file rule, and whether path assignment reloads data beyond the tested cases.**
- **6 — a missing-files report as data, the unexplained recovered Mac path, and the script-in-window and recovered-library results outside 5.3.**
- **7 — tracing references of every kind, large-file cost, termination in every case, and the excluded-collection, filtered-Outliner and dropped-driver results outside 5.3.**
- **8 — what the library parent property holds, missing non-library data, packed libraries' loader listing in practice, and the missing-chain Outliner result outside 5.3.**
- **9 — the image-replacing command with an editor but no window before 5.2.2, the sequencer command on Linux with its required inputs, and the image's own Data API path row.** The earlier versions' sweep counts and library-file warnings are also unestablished.
- **10 — the rule behind equal-size ties, search depth beyond eight folders, undo, search counts, sequences and UDIM sets, and preservation of both path forms in 4.5–5.2.**
- **10 and 11 — window warnings and messages beyond the window tests explicitly described.**
- **11 — graphics-card renders; same-second changes beyond the tested Mac cases; the edited-data reload option in 5.0–5.2; the named-library reload command; and the successful sequencer and shader-editor calls in 4.5–5.2.0.**
- **11 — the texture cache with automatic building disabled after a copy was made, outside 5.3; replacement size or format changes beyond the clip test; deletion; and whether Blender notices replacement of its own open file.**
- **11 — the conflicting results for packed-image reloads, changed shader scripts and Alembic reloads.** A documentation change for the shader auto-update switch is not established.

### Where only the documentation differs

- **4 — only 4.5's quoted release note warns that the finished-frame-path call can fail; the malformed-path test raised in all five.**
- **5 and 9 — the packed-path switch is described oppositely before 5.1, but includes packed paths in all five tested versions.**
- **5 and 9 — 4.5's listed argument form differs, but all five tested versions reject positional arguments.**


---

## 1. What does a `.blend` file give up without a Blender to open it, and what needs one?

**How it is known.** **Documented:** Zstandard compression, a stored preview and Blender-installed preview tools. A full account of what an outside program can read is **not in the sources**. The claim that everything beyond the preview and decompression requires Blender remains **inferred**, not established by that silence. The results below are **observed** in 4.5–5.2 on the Mac and Linux, and in 5.3 on the Mac, except where a narrower scope is named.

**The same in the tested cases.** Four things came back identically, in 4.5 to 5.2 on both machines and in 5.3 on the Mac:

- **The compression is an ordinary one.** Every version's manual says the algorithm is Zstandard and that outside tools can undo it. Handing a compressed file to an ordinary Zstandard tool produced, in every version, a file of exactly the same size and the same opening bytes as the same scene saved with compression switched off. Compared byte by byte, the two are **not** identical throughout — they first differ at byte 181 in 4.5, byte 394 in 5.0 and byte 65,970 in 5.1 and 5.2, read on the rented machine, and at byte 65,970 in 5.3 on the Mac, where a 5.2.2 control gave the same. Same size and same opening bytes, identical throughout no.
- **The file's layout is nowhere described.** All five versions searched for it — header, byte order, structure definitions, format specification — and found nothing.
- **Outside tooling is acknowledged and not described.** All five manuals say tooling that manipulates blend-files outside Blender already exists and name one. None says what it can read.
- **Peeking into a file without opening it gives that file's own local data only.** In every version the loader listed a local collection and returned an empty object list, because the object inside was itself linked from elsewhere. Handed a path that does not exist, it raised the same error in all five.

**A warning about file sizes.** The exact size of a compressed save is not stable — not between the two machines, and not between two saves on the same machine. Three runs of the same 4.5 save gave three different sizes. Nothing should be read from a compressed file's exact byte count.

**Where it differs, version by version.**

**Compression on an ordinary save.** In **4.5** the preference reads off and the `compress` argument's own default reads off, and a save with no compression argument produced a 434,663-byte file — byte for byte the size of the explicitly uncompressed save, on both machines. In **5.0, 5.1, 5.2 and 5.3** the preference reads on while the argument's default still reads off, and a save with no argument produced a compressed file, as did the explicitly compressed save. Blender 5.0's release notes state it: compression on saving is now enabled by default, and anyone wanting uncompressed files must change and re-save their preferences. **Confirmed in 4.5 to 5.2 on both machines and in 5.3 on the Mac.**

**The text a file starts with.** In **4.5**: `BLENDER-v405REND`. In **5.0**: `BLENDER17-01v0500REND`. In **5.1**: `…v0501…`. In **5.2**: `…v0502…`. In **5.3**: `…v0503…`. Blender 5.0's release notes say this change "affects the structure of the file header as well as the header of each block in the file. So any code that is parsing .blend files has to be updated." **Confirmed in 4.5 to 5.2 on both machines and in 5.3 on the Mac.**

**What a compressed file's first bytes show.** All the tested compressed saves begin with the four Zstandard marker bytes. Within the first 32 bytes, readable opening text follows in **4.5 and 5.0** on the Mac and Linux, and in **5.3** on the Mac. No readable text follows in **5.1** on either tested machine. **5.2.0 is disputed:** its printed Mac and Linux bytes show no readable text there, but another account of both runs says the opening text follows the marker. Neither account is discarded, so that point remains open for 5.2.0. A separate **5.2.2** test on the same Mac as 5.3 showed no readable text, using the same command. That comparison establishes a difference between those two tested builds on that Mac. Nothing here explains it or establishes what every compressed save will show. An ordinary Zstandard tool returned the plain opening text in all five versions.

**What the loader hands back about a file it peeks into.** In **4.5** there is no version attribute and no library-list attribute. In **5.0** and **5.1** there is a version attribute — it read `(5, 0, 119)` and `(5, 1, 30)` — and still no library list. In **5.2** and **5.3** there is both: the version read `(5, 2, 44)` and `(5, 3, 29)`, and the library list named the file that file links, with its recorded path and a flag saying whether it is an archive. Blender 5.2's release notes state the addition and show the same shape of result. The names of the data-block categories the loader offers also shifted between 4.5 and 5.0 — 4.5 carries a separate grease-pencil-version-three category and no annotations category, while 5.0 to 5.3 carry an annotations category and no version-three one; 5.3's list of categories is the same as 5.2's. **Confirmed in 4.5 to 5.2 on both machines, the same version numbers included, and in 5.3 on the Mac.**

**Whether a file saved with no window carries a preview picture. Three different tools were asked, and they are not one test run three times.**

- **On the Creator's Mac, for 4.5 to 5.2, the question was put to the Finder.** Eight files were put side by side, two per version — one saved by that version with no window, one opened in that version and saved again from the window. The two saved with no window by **4.5** and by **5.0** showed a plain grey document icon with the Blender logo on it. The two saved with no window by **5.1** and by **5.2** showed a picture of a grey cube, as did all four saved from the window.
- **On the rented Linux machine there is no Finder, so the question was put to Blender's own thumbnail tool**, the one it ships beside its program. Given the same eight files, built the same way, it **failed with an error code and made no picture** for the two saved with no window by 4.5 and by 5.0, and **succeeded and made a picture of a grey cube** for the two from 5.1 and 5.2 and for all four saved from a window.
- **For 5.3, on the Mac, the question was put to Blender's own preview loader**, asked from a Blender with no window, which reads the preview stored inside a file. A 5.3 file saved with no window held a 128 by 128 preview; one saved from the window held a 256 by 256 one; one saved with preview saving switched off held none. Written out as pictures, both previews show a grey cube. **So in 5.3 the preview is inside the file, saved with no window as well as with one.** The Finder was not reached for 5.3: macOS's Quick Look, asked from the terminal, never answered for any file, 5.2.2's included, and the thumbnail tool registered on that Mac was 5.2.2's and not 5.3's.

**The split falls in the same place under the first two tools, and that is as far as it goes.** A desktop file browser and a command-line tool are two different things asking two different questions, so this is a matching pattern and not a single result confirmed twice. **No file browser on Linux was ever looked at.** For 4.5 to 5.2, neither run establishes whether the preview is missing from the file or merely not shown. For 5.3 the reverse holds: the preview is in the file, and what a file browser shows is untried.

**Verdict: it differs.** Compression defaults, headers and loader access differ as described above. The preview tools split in 4.5–5.2; 5.3 was asked another way. Compressed opening bytes also differed in the tested saves, including the same-Mac 5.2.2–5.3 pair, but the 5.2.0 answer remains disputed.

**Not established.**

- **The compressed opening text in 5.2.0.** Its two accounts disagree, as above.
- **What a program that is not Blender could actually read out of a `.blend`.** Silent in all five, and the whole point of the question. Nothing on either machine changes that.
- **Whether a Linux desktop file browser shows a preview picture**, and whether registering Blender's thumbnail tool with one would make it. The tool was run directly from a command line.
- **Whether a file browser shows the preview of a 5.3 file.** Above.
- **Whether the asset-catalog or preview-registration documentation changed.** Not established. An asset catalog is documented as a separate text file beside the blend-file in 4.5 and 5.1; silence in the other findings does not establish its absence.
- **Preview-tool registration on Windows and Linux.** The 5.2 answer is inconsistent about whether its documentation covers those systems. 5.3 documents all-user Linux thumbnail installation. No registration test settled either system.

---

## 2. How many Blenders can be at work on one computer at the same time, and does one reading a file while another renders cause trouble?

**How it is known.** A maximum or recommended count and any rule about reading while another Blender renders are **not in the sources**. Multiple running instances are **documented** for 4.5, 5.0, 5.1 and 5.3. The 5.2 documentation answer is open; the tests do establish multiple running instances there. Concurrent reading and saving were **observed** in 4.5–5.2 on the Mac and Linux. **5.3 was not tested**: its Mac tests allowed one test Blender at a time.

**The count could not be tested on any machine, and no test could settle it.** Whatever number of Blenders a computer would start is a fact about that computer. It is a silence in what could be tried, in every version alike. The rented machine's limit on how many files one process may hold open at once was noted as 1,024 — a fact about that machine, not Blender.

**The second half is settled for 4.5 to 5.2, and settled the same in all four, on both machines.** A 60-frame render of a file was started with no window. While it ran:

- A second Blender opened that same file with no window, printed its path and its object names, and quit with an exit code of 0 while the render was still going. The render then finished with an exit code of 0, wrote all 60 frames, and printed no error and no warning at all.
- A third Blender opened a copy of the file, renamed an object in it and saved over it mid-render. The save reported that it had finished, the render still finished with an exit code of 0 and all 60 frames and no error or warning, and reopening the file afterwards showed the rename had taken.

That came back the same in 4.5, 5.0, 5.1 and 5.2, on the Mac and on the rented Linux machine alike. On the Mac it was run both when the second Blender was started after one finished frame and when it was started after two; on the rented machine the wait was on two frames and every poll printed its count. **Neither run says anything about how long any of it took.**

**Verdict: not established for every version** — because the first half was answered "not in the sources" in every version and no test could reach it, and because the second half was not tried in 5.3. **The second half gave the same result in 4.5–5.2 on the two tested machines.** For 5.3 it is not established either way.

**Not established.**

- **A number, of any kind.** Owed by every version, and unreachable by any test.
- **Whether one Blender reading a file while another renders it causes trouble in 5.3.** Not tried.
- **Whether Blender holds a `.blend` open after loading it or reads it once and lets go.** Not stated in any version, and no machine's tests probed it.
- **Whether two Blenders tread on each other in the temporary folder they both pick by default.** Not stated in any version. In 5.3 automatic saves are documented as named by the running Blender's process number. Keeping them apart follows by **inference**, not a test; other temporary files remain open. **Which environment variable names that folder is not settled either** — two pages of the same manual disagree about it, the same way in 4.5 and in 5.1, so it is not a difference between versions, and nobody tested it.
- **Whether a second or third Blender slows the render down.** No machine can say.

---

## 3. Can a single read of a file tell, for every view layer, whether that layer renders at any frame later asked about?

**How it is known.** The per-layer render switch and the command-line disregard of Render Single Layer are **documented**. Whether animation changes the switch and whether a render follows it frame by frame are **observed** below, in 4.5–5.2 on the Mac and Linux and in 5.3 on the Mac. The manual pictures do not establish whether either switch is animatable: the accounts disagree about their decorator dots, including within 5.1. The directly read animation marks and the tested window behaviour are given below.

**The same in the tested cases.** The behaviour that answers the question came back identically — in 4.5 to 5.2 on both machines and in 5.3 on the Mac:

- The switch can be keyframed from a script, and the curve lands on the scene with a data path naming the layer.
- Stepping the frame changes what the switch reads.
- **An animation render honours the keyframed value frame by frame.** With one layer switched on at frame 1 and off at frame 2, the file written for frame 1 held both layers and the file written for frame 2 held only the one still switched on. On the rented machine the render ran on an NVIDIA graphics card and came out the same way.
- **Blender itself marks the render switch as animatable and the render-single-layer switch as not.** Asked directly — in 4.5 to 5.2 on the rented machine, in 5.3 on the Mac — the first reads true and the second reads false. The two machines are not the same computer, and the answer agrees across them.
- **In Blender's own window on the Mac, pressing I over the render switch keyframes it.** In 4.5 to 5.2 the Creator did it by hand and reported the checkbox turning yellow and the dot beside it becoming a diamond, identical in all four; no picture was taken after the key was pressed, so that rests on the Creator's written words. In 5.3 a script pressed the key over the switch and the window was photographed afterwards: the checkbox drawn yellow-green, a diamond in place of the dot, and a curve on the scene. Over the render-single-layer switch no curve appeared, in 5.3. **What the window shows when the key is pressed is settled on the Mac alone.**

**What the window says over the render-single-layer switch.** The Creator, pressing the key by hand in 4.5 to 5.2, saw the message that that property cannot be animated. In 5.3 the scripted key press brought no message to the status bar in the three seconds before the window was photographed, and nothing on the terminal; whether a message showed and went in those three seconds is not established. The same scripted press in 5.2.2 on the same Mac came out the same as 5.3 in every particular. That gives no evidence of a version difference; it does not establish why the earlier hand press showed a message.

**Where it differs, version by version.**

**The switch's declared default.** In **4.5** it reads off. In **5.0, 5.1, 5.2 and 5.3** it reads on. Each version's own API text says the same as its probe — 4.5 to 5.2 on both machines, 5.3 on the Mac. The other switch's default reads off in all five.

**How you reach the animation curve.** In **4.5** an action's curves hang directly off the action, and reading them that way worked. In **5.0, 5.1, 5.2 and 5.3** that route does not exist — asking for it raised the same error, worded the same way — and the curve has to be reached by walking the action's layers, then its strips, then its channel bags, then its curves. **Confirmed in 4.5 to 5.2 on both machines and in 5.3 on the Mac.**

**What the compositor renders.** Blender 5.3's release notes record that, from 5.3, the compositor renders scenes referenced inside node groups. A Render Layers node names a scene and a view layer, so this is one more way a render can reach a view layer of another scene. No test touched it.

**Verdict: it differs** — in the switch's default value, in the route to its curve, and, by 5.3's release notes alone, in the compositor rendering scenes referenced inside node groups. What the question actually asks is answered the same way in the tested cases: 4.5 to 5.2 on two machines, 5.3 on the Mac.

**A compositor node aimed at a switched-off view layer — 5.3 alone.** In 5.3, on the Mac, a file was rendered whose compositor's only output was a Render Layers node aimed at a view layer whose render switch was off. The render named only the other view layer, and the node's picture came out empty. With the switch on, the render named both and the node's picture held what that view layer sees. So in 5.3 a Render Layers node does not make a switched-off view layer render. Cycles on the processor; untried in 4.5 to 5.2.

**Not established.**

- **Whether a driver, as opposed to a keyframe, on that switch can be resolved from one read.** Tested on no machine. Automatic running of Python drivers is documented as off by default for 5.0–5.2; that default is not established for 4.5 or 5.3 here.
- **Whether a script inside the file could change the switch as each frame comes round.** Untested.
- **What Blender's window shows when the switch is keyframed by hand, other than on the Mac.**
- **Whether a person pressing I by hand over the render-single-layer switch in 5.3 sees the message.** Above.
- **What a Render Layers node aimed at a switched-off view layer does, outside 5.3; and whether view layers of other scenes, reached through the compositor, render.** Neither was tried in 4.5 to 5.2, and the second was tried in no version.

**Two limits of the test set-up.** **Observed:** the render engine's own identifier: 4.5 reported `BLENDER_EEVEE_NEXT` and 5.0 to 5.3 reported `BLENDER_EEVEE`, 4.5 to 5.2 on both machines and 5.3 on the Mac. Second, the way multi-layer OpenEXR is asked for: 4.5's test printed one word, and 5.0's, 5.1's, 5.2's and 5.3's each printed the same two words. In 5.3 those two words came only after the media type had been set to multi-layer first: asked for directly as a file format, multi-layer OpenEXR was refused as not among the formats offered. No refusal is recorded in 4.5 to 5.2, and whether they refuse the same direct request is not established. Neither result settles the view-layer question.

---

## 4. Can the places a file is set up to write to be read out of it?

**How it is known.** The named output settings, the finished-frame-path call and stored template text are **documented**. Template resolution by that call is **observed**. The tests below ran in 4.5–5.2 on the Mac and Linux and in 5.3 on the Mac, except where a narrower scope is named. Release-note changes and untested documented defaults are identified separately.

**The same in the tested cases.** Five results came back identically, in 4.5 to 5.2 on both machines and in 5.3 on the Mac:

- **The stored string is the stored string.** A path holding `{blend_name}` reads back with the braces intact in all five; asking for the finished path for a given frame returned a path with the value worked in.
- **A malformed template makes that call raise** rather than return anything, in all five, with the same wording. **Documented:** 4.5's release note says the call used to be infallible and can fail on a bad template. No such warning about the call is established for the other versions; 5.3's manual warns about animation rendering instead. **Observed:** all five raised on the same malformed path, so this is a difference in the documented warning, not a tested version difference.
- **The output-path marker** reads true for the scene's render output path and false for an image's path, a library's path and a physics cache's path — the same four answers in all five versions.
- **The whole-file path lists do not carry output paths.** On a file whose render output path and whose File Output node were both set, neither the list of external paths nor the map of paths per data-block returned either of them, in any version, and in 5.3 the one-pass call did not visit them.
- **Every version's manual names the same two places that take path templates** — the scene's render output path and the compositor's File Output node — in the same words, 5.3's included. 5.3's release notes add a third, below.

**Where it differs, version by version.** Every one of these came back the same on both machines in 4.5 to 5.2; 5.3's half was run on the Mac.

**What the File Output node is made of.** In **4.5** the node carries a base path and a list of slots, each slot with its own sub-path; reading it back gave the base path `//out/` and slot paths `Image`, `passes/diffuse`, `beauty`. In **5.0, 5.1, 5.2 and 5.3** there is no base path at all; the node carries a directory, a file name and a list of items. Blender 5.0's release notes list the three properties removed and the three added.

**What a render writes from it, and why.** In **4.5** the render wrote one file per input — three of them, plus the scene's own output — and the slash in an input's name made a folder on disk. In **5.0 to 5.3** the same test wrote **one** file for the whole node, plus the scene's own output. **That is the node's starting format and not the node writing differently.** A freshly added File Output node starts on PNG in 4.5 and on Multi-Layer OpenEXR in 5.0, 5.1 and 5.2, and this test's 4.5 node was on plain PNG while the later nodes were on Multi-Layer EXR; in 5.3 the test set no format, and the node was on Multi-Layer EXR. The later node has a two-way media-type choice, Image or Multi-Layer EXR, which 4.5's node has no equivalent of. What a File Output node calls its file follows the node's mode: in multi-layer mode no input's name reaches the file, and in image mode it does. In 4.5 to 5.2 the node can do both, and in the later three it is the media type and not the version that switches between them. **Settled under File output nodes 1 and 6**; the starting value is the real version change, and it is in the list of behaviour differences above.

**Whether a slash in an input's name survives.** In **4.5** it survives, and makes a folder. In **5.0** it does not — an item asked for as `passes/diffuse` came back as `passes_diffuse`. In **5.1, 5.2 and 5.3** it survives again. Blender 5.1's release notes state it plainly: the node "now allows specifying directories in item names. This was possible in older versions of Blender and the functionality is now restored."

**`{scene_name}` in an output path.** In **4.5** it is an unknown variable; from **5.0**, 5.3 included, it resolves to the scene's name. Blender 5.0's release notes list it among new path variables.

**What the node's sidebar shows.** In **4.5** there is no Output Paths panel at all; the sidebar holds the base path, the format settings and a list of the inputs with a sub-path field. In **5.0, 5.1, 5.2 and 5.3** there is a section headed Output Paths. On the Mac the Creator opened it in 5.0 to 5.2, and in 5.3 a script opened it with simulated clicks. The node photographed was in Multi-Layer EXR mode, which writes one file for the whole node, and the panel held one row — in 5.3 the assembled path to the one EXR file, cut short in the middle by Blender to fit. Where a node writes one file per output the panel holds one row per output — settled under **File output nodes 10 and 17**. In **5.2** the panel also holds a File Extensions switch and a button to add to it, and 5.2's node carries a matching file-extension property that 4.5's, 5.0's and 5.1's do not. 5.3's node carries the same property, and its panel holds the switch; the picture ends just below the path line, so whether anything follows it is not shown. **On the rented machine the panel was seen but not opened** — it was closed in all three of the later versions, and nothing on that machine could reliably click it — so **the panel's presence is settled on both machines for 5.0 to 5.2, and its contents on the Mac alone.**

**Where the compositor's tree hangs.** In **4.5** the scene has a node-tree property and no compositing-node-group property. In **5.0 to 5.3** it is the other way round. Blender 5.0's release notes state the replacement. On the rented machine a script that chose its route by looking reported taking the old route in 4.5 and the new one in 5.0 to 5.2. In 5.3, every script that set the newer property printed a warning that it is expected to be removed in Blender 6.0. The earlier records show no such line, and whether the earlier versions print it is not established.

**The render output path's documented default.** Blender 4.5's, 5.0's and 5.1's API text gives it as empty. Blender 5.2's and 5.3's give it as `//`. This rests on each version's own documentation; no test on any machine read it.

**Whether the render output has to be written at all.** Blender **5.2** has a switch for it, and 5.2's release notes say "Saving render output can now be disabled." Its manual explains what happens when it is off: for an animation, the sequencer or the File Output nodes must supply the result, and if neither can, an error is raised. **Observed under The compositor**, in 4.5–5.2 on both machines: **asked for by name in 4.5, 5.0 and 5.1 the switch is simply absent**, and in 5.2, turning it off stops the scene's own frames being written even when a place is named on the command line, while a File Output node in the same tree keeps writing. Blender **5.3**'s API text documents the same switch — "Write frames to disk for animation renders", on by default — and no test in this topic reached it in 5.3; **The compositor 2** and **Reports 1** record 5.3 tests of it.

**Path templates in a bake folder, and a project's own variables — 5.3.** Blender 5.3's release notes add template support to bake paths in geometry nodes, and add a `{project_root}` variable and variables a project defines for itself. 5.3's API text marks both of a geometry-nodes modifier's bake-folder settings as taking templates. 5.3's manual still says templates work only in the render output path and the File Output node; the release notes explicitly record the addition. It is **inferred** that a path using a project's variables cannot be fully worked out from the file alone, because a project is identified by a `.blender_project` folder on disk. **None of it was tested in this topic.**

**Verdict: it differs.**

**Not established.**

- **A complete list of the places a file can be set up to write to.** No complete list is established. Different numbers of properties in the findings do not establish different capabilities in Blender.
- **What the Output Paths panel holds on Linux.** The panel is there in 5.0, 5.1 and 5.2 on both machines; its contents were read on the Mac only, because nothing on the rented machine could open a closed panel. 5.3's was seen on the Mac only.
- **Any way for a script to get the assembled final path the node's sidebar shows.** No route is established. The 5.2 and 5.3 property probes exposed no such value. In 4.5 the panel does not exist to have an equivalent of.
- **Whether a bake folder takes templates in practice in 5.3, and whether a path using a project's variables can be worked out from the file alone.** Above; untested.
- **Whether Windows' cap on file names bites here.** Blender 5.0's release notes note in passing that Windows file names are capped at 255 characters and that longer data-block names make that easier to hit; nothing was tested.

---

## 5. Can the locations a file records for the things it points at be read out and written back, and by what?

**How it is known.** The path lists, named path settings, bulk conversion commands and path helpers are **documented**. Whether an unmarked property accepts assignment, and whether a rewrite persists only after saving, need more than those descriptions; both were **observed** in the tests below. Basic image-path tests ran in 4.5–5.2 on the Mac and Linux; most later tests ran on Linux alone. All 5.3 tests ran on the Mac alone.

**The same in the tested cases.** Five results came back identically, in 4.5 to 5.2 on both machines and in 5.3 on the Mac:

- **Reading and writing a recorded path works, and the write sticks.** An image's recorded path was read, given a new value, saved, and read back after reopening the file — it held the new value in all five.
- **All of it works with no window at all**, in all five.
- **The list of operators under the file group is identical in all five versions**, forty of them, including the two that make paths absolute or relative and the four that pack and unpack.
- **The list of external paths carried one entry nothing in the file pointed at** — the Blender application's own brushes file — in all five versions.
- **A recorded path survives when the file it names is gone.** The path still reads back, and the missing flag on the image still reads false, in all five.

**"All paths to external files" does not cover every kind of recorded path.** A file was built holding **twenty-one kinds of recorded location**, written out in full from the drive root where the setting accepted it, and each way of listing and each way of rewriting was pointed at it. The flat list of external paths, the map of data-blocks to paths and the relative sweep reach the same sixteen readings and miss the same seven — the **render output path, the bake output path, the File Output node's path, the point cache, the geometry-nodes bake folder, the sequencer's proxy folder, and the custom property**. That the flat list and the map name the same set holds on this file, whose linked library holds nothing outside itself; where a library points at a picture of its own, the map reaches through the library and the flat list does not (**Dependencies 20**, on Windows). A walk over Blender's own marks on its settings finds nineteen of the twenty-one, including six of the seven the sweeps miss. **So no single means reaches everything, in any version**, and the sweeps and the mark walk together cover all twenty-one but the custom property. The absolute sweep restored the changed paths but left those seven already-absolute settings unchanged; whether it reaches them is untested. Question 9 gives the full coverage. **The result is identical in all five versions**, apart from the one-pass visitor, which 4.5 does not have. 4.5 to 5.2 ran it on the rented Linux machine alone; 5.3 ran the same file on the Mac alone — counting the same contents as twenty-three kinds, because it counts the multi-resolution modifier's path and the sequencer's proxy folder on their own — so the agreement crosses machines.

Two more things that settles:

- **A path that holds a template is left alone by everything.** Neither sweep touches the render output path or the File Output node's path when they hold `{blend_name}`, no listing means ever names them, and the braces survive in the stored text. The call that asks what a frame will be written to substitutes the value. Identical in all five versions.
- **Writing a path is not enough on its own, at least for a linked library.** Assigning a library's path changes what is recorded at once but leaves the old data in place until the library is told to reload. The relocate operator does both in one act and records the new path in relative form. Identical in all five versions, on a file linking one library, aimed at a file no library records. **Dependencies 19** ran the same call on a file linking two libraries and it moved nothing, in 4.5 to 5.2; what separates the two is not established, and neither is to be read as a general rule.

**Writing an image's path, and whether its picture follows — 5.3 alone.** In 5.3, on the Mac, four images whose pictures had been read were each pointed at another file a different way. Setting the image's path dropped its loaded picture, and the next read gave the new file's. Setting its raw path — documented as changing the path without refreshing — kept the old picture. Rewriting the path through the one-pass call kept the old picture with the call's default options, and gave the new one with its option for reloading edited data. Untried in 4.5 to 5.2.

**Where it differs, version by version.**

**A call that visits every recorded path in one pass, and can rewrite each one by returning a new string.** Absent in **4.5** — asking for it returns nothing and the map it would fill reports no such method. Present in **5.0, 5.1, 5.2 and 5.3**. **Confirmed by probe in 4.5 to 5.2 on both machines and in 5.3 on the Mac.** Blender 5.0's release notes state the addition and describe exactly that callback behaviour. Blender **5.2** adds three more options to it — visiting each tile of a tiled image, each frame of a sequence, and cache files such as Cycles' texture cache — together with an object that tells the caller whether the path it is looking at can be edited at all; **5.3** has the same. Blender 5.2's release notes state that addition and give render farms as the reason for it.

**A read-only list of every data-block in the file.** Absent in **4.5, 5.0 and 5.1**. Present in **5.2 and 5.3**. Confirmed by probe — 4.5 to 5.2 on both machines, 5.3 on the Mac. Blender 5.2's release notes state the addition.

**Writing a library out with everything made relative.** Handed a path that was **already** relative, **4.5 and 5.0 leave it exactly as it stands**, so after the write it points at a file that is not there; **5.1, 5.2 and 5.3 recalculate it** so it still points at the real file. Blender 5.1's release notes record this as a fix, and the same test run in every version is what establishes it. 4.5 to 5.2 ran on the rented Linux machine alone and 5.3 on the Mac alone. **The other fix 5.1's release notes record — an external multi-resolution displacement file's recorded path not being cleared when re-packing — could not be tested at all**, because the property would not take a value in any version.

**Verdict: it differs.**

**Which machines this rests on.** Most later tests ran on Linux alone in 4.5–5.2 and on the Mac alone in 5.3. One path-reading test ran on both earlier machines and differed only in the application path and the disk's treatment of capital letters, as under question 9. Neither is a version difference.

**Not established.**

- **Whether the rule that a Blender writes only to the file it has open gained an exception.** Blender 4.5's manual states the rule flat. Blender 5.0–5.2 state an exception for special asset files. The scope of 5.3's rule on its asset-library page is unclear. Whether any of that is a change in Blender or a change in the manual was not established, and 4.5's silence on those files is not evidence either way.
- **Whether writing a path reloads anything other than a library, outside 5.3**, and for anything other than an image in 5.3.
- **Whether the sweeps come out the same on the other machine** — on the Mac for 4.5 to 5.2, on Linux for 5.3.
- **Whether the multi-resolution modifier's own recorded path is reached by anything.**
- **Whether 5.3's answer holds on Linux and Windows.** Not established: the claim that Blender behaves this way on every supported system has no source or test beyond the Mac behind it.

---

## 6. Can anything be asked to report which of a file's references are missing, to search for them, and to rewrite the recorded locations?

**How it is known.** The report, search and rewrite commands and the flag for missing linked data are **documented**. A route to the report as data is **not established**. The results below are **observed** in 4.5–5.2 on the Mac and Linux and in 5.3 on the Mac, with each window result qualified separately.

**The same in the tested cases.** Six results came back identically, in 4.5 to 5.2 on both machines and in 5.3 on the Mac:

- **The report works with no window and names the file.** Calling it printed `Warning: Path '<path>' not found, from local data-block 'IMProbeLoaded'` and reported that it had finished, in all five. With two files missing, 5.3 printed one such line for each.
- **The search works with no window when it is handed a folder.** It found the moved file, rewrote the recorded path, and the change survived a save and a reopening — in all five. In 4.5 to 5.2 the path started in the relative form and came back in it every time; in 5.3 a path recorded in full came back in full and one recorded relative came back relative. Question 10 holds that.
- **Calling the search with a folder supplied opened no file browser** — in all five. Every version's manual says a file browser will show up; in every version, given a folder, none did. On the Mac in 4.5 to 5.2 it was typed into the Python Console inside Blender's window; on the rented machine the window count was read before and after and stayed at one; in 5.3 on the Mac it was called from a script inside a window, and the count stayed at one there too.
- **Chosen from the File menu in Blender's own window, the report appears at the bottom of the window**, naming the missing path and the data-block, with the same wording in all five apart from the folder in the path. The Creator read it by hand in 4.5 to 5.2 on the Mac; in 5.3 the menu was driven by simulated clicks on the Mac and the window photographed, showing a warning mark and the path in the status bar, while the terminal printed the same warning as a timestamped line. On the rented machine the operator was driven with a window open and Blender printed the missing path by name, but the message on screen could not be read there.
- **The flag on a plain image whose file is gone reads false** in all five — it marks stand-ins for missing linked data, not broken image paths.
- **The search covers a missing linked blend-file.** It rewrote a missing linked file's path along with five other kinds, in all five; that is under question 10. In 5.3 the report named the missing linked file as well, one line among six; untried in 4.5 to 5.2.

**Called from a script inside a window — 5.3 alone.** In 5.3, on the Mac, the report called from a script running inside Blender's window printed its warning on the terminal and nothing reached the status bar; chosen from the File menu in the same kind of window, it did reach the status bar. Untried that way in 4.5 to 5.2.

**A missing library after the search — 5.3 alone.** In 5.3, on the Mac, once the search had rewritten a missing linked file's path, the library and its collection still read as missing for the rest of that session. Saved and reopened, the library was found and its collection, object and mesh were whole. Untried in 4.5 to 5.2.

**The recovered path on each machine.** On the Mac in 4.5 to 5.2, the Creator's run from the Python Console in Blender's window recovered `//../15/tex_5.2/probe.png` in every version, 4.5 included, and it was not established why. On the rented machine the same test recovered `//search/deeper/probe.png` in every version — the plain relative path to where the file actually was. In 5.3 on the Mac, called from a script with no window and again inside a window, the search recovered `//search/deeper/probe.png`. The search folders were not established as matching, so these paths cannot establish either a version or an operating-system difference.

**What makes a file a possible match, and which one is chosen when two candidates share a name, is answered by test under question 10.**

**Where it differs, version by version.**

**Whether Blender's reports can be read as a list.** In **4.5, 5.0 and 5.1** the window manager has no such property at all — the test printed `NO SUCH PROPERTY` in each, on both machines. In **5.2 and 5.3** the property exists, and the test printed an empty list after the report had run. Blender 5.2's release notes state the addition: reports "are now exposed as a read-only list in WindowManager ID types". So the property is genuinely new in 5.2 and kept in 5.3 — **and it did not hold the missing-files report.** In 5.2 it was tried again on the rented machine **with a window open on that machine's own display, straight after the report ran**, and it was still empty; in 5.3 it was empty after the report was called from a script inside a window on the Mac. The tested calls yielded messages; no tested call yielded the report as a list.

**Verdict: it differs** — in that one respect. Everything the question actually asks for works the same way in all five, 4.5 to 5.2 on two machines and 5.3 on the Mac.

**Not established.**

- **Any route to the missing-file report as data.** None is established. The tested reports property stayed empty; that does not rule out every possible route.
- **Why the Mac's recovered path named a 5.2 folder in 4.5 to 5.2.** Recorded there, explained nowhere, and reproduced neither on the other machine, which searched a different folder, nor in 5.3 on the Mac.
- **Where the report goes when a script calls it inside a window, and what a missing library's data does after the search, outside 5.3.** Above.
- **Whether 5.3's answer holds on Linux and Windows.** Not established. The all-systems claim is unsupported, as under question 5.

---

## 7. Can something a file points at be traced to the scene that uses it?

**How it is known.** The map of direct users, an object's scenes and a library's users are **documented**. Following successive users to a scene is **inferred** from those parts and **observed** for the references tested below. The plain image test ran in 4.5–5.2 on the Mac and Linux; the awkward-reference tests ran on Linux alone. All 5.3 tests ran on the Mac alone.

**The same in the tested cases.** The test output is identical line for line — in 4.5 to 5.2 on both machines and in 5.3 on the Mac:

- From an image used by a material on a cube that sits in one of two scenes, the walk reached that scene in four steps — image to material, material to mesh, mesh to object, object to scene — and stopped cleanly two steps later.
- From a second image that nothing used, it reached nothing at all.
- **Asking the map directly for scene-type users of the image returned an empty set**, in all five. So the shortcut does not work and the walk is what is needed, everywhere.
- The map from data-blocks to their recorded paths named both images and their paths, in all five.
- The whole thing runs with no window, in all five.

**All four tested image-reference cases show up in that map.** A file was built holding one scene, one object, one material and four images. One image was used straight from the material's own nodes, as the control. One was used **only by an image node inside a node group**, which the material reaches through a group node. One was named **only as the target of a driver**. One was held **only by a custom property on the object** pointing at the data-block. The map of what uses what was asked, once, and walked outwards from each image. **All three of the awkward cases are visible, and the result is identical in all five versions, line for line.** The node-group one shows up through the group's own node tree; the driver one and the custom-property one show up through the object itself. Every image that is used at all reaches the scene. Asking the map directly for scene-type users returns an empty set for all four images, which is what the plainly-used image gave too — so the walk is still what is needed, and the shortcut still does not work. **4.5 to 5.2 ran this on the rented Linux machine alone and 5.3 on the Mac alone**, so the agreement crosses machines and no version has both.

**The driver adds no user.** In 5.3 every image in that file was given a fake user so that none could be dropped on saving, and each user count includes it. The image named only as a driver's target counts one — its fake user and nothing else — and the earlier four printed the same counts. In 5.3, built once without fake users, that image was not in the file when it was opened again. Untried that way in 4.5 to 5.2.

**Two more results, from 5.3 alone, on the Mac.**

- **An image used only in a collection excluded from its scene's view layer still traces to that scene.** The walk reached the scene through the collection. The exclusion shows only in the view layer's own setting and in whether the object reads as visible in that view layer, which it did not; the map of what uses what carries neither. So the map says what uses what, not what renders.
- **The Outliner's Scenes view, filtered by a linked object's name, shows it under the one scene that holds it.** The other scene does not appear in the filtered tree.

**Verdict: the same in the tested cases; not established generally.** The walk reached the scene from the images tested. The plain test ran on both machines in 4.5–5.2 and on the Mac in 5.3; the awkward references ran on Linux in 4.5–5.2 and on the Mac in 5.3. That agreement across machines does not settle either missing operating-system half, untested reference kinds or whether the walk always terminates. No single general trace-to-scene call is established.

**Not established.**

- **What the walk costs on a large file.** Two of the direct properties carry cost warnings in every version's documentation; the map itself carries none. No machine can say.
- **Whether the walk always terminates.** It did in every test in all five; nobody tried to make it loop.
- **Whether the awkward-reference result holds on the other machine** — on macOS for 4.5 to 5.2, on Linux for 5.3.
- **The excluded collection, the Outliner's filtered Scenes view and the dropped driver target, outside 5.3.** Untried.
- **Whether a reference that is not between data-blocks** — a compositor node naming another scene's view layer — shows up in the map. No version tested it.
- **Whether 5.3's answer holds on Linux and Windows.** Not established. The all-systems claim is unsupported, as under question 5.

---

## 8. Can a file's full chain of references be read while some of the referenced files are absent?

**How it is known.** Missing-library placeholders, their missing flag and a library's path and parent properties are **documented**. The chain stopping at an absent file is **inferred** from the documented parts and **observed** in the chain tested below: 4.5–5.2 on the Mac and Linux, 5.3 on the Mac.

**The same in the tested cases.** A three-deep chain was built in each version — one file holding a cube, a second linking that cube into a local collection, a third linking that collection — and opened three times. The results are identical in all five versions, 4.5 to 5.2 on each machine and 5.3 on the Mac:

- **With everything present**, both libraries are listed with their recorded paths, neither is marked missing, and the linked object is listed as indirectly linked from the far file.
- **With the far file gone**, both libraries are still listed, the far one is marked missing, the linked object is marked missing, and the list of external paths still holds a path whose file does not exist.
- **With the middle file gone instead**, only the middle file is listed, and it is marked missing. **The far file is named nowhere at all**, the object list is empty, and the list of external paths holds only the middle file. So the readable chain stops at the first absent file — in every version.
- **A library's parent property came back empty in every case in every version**, while Blender's own log said each library was linked directly. What that property holds is not documented here, and the tests give no reason to think it differs anywhere.
- **Opening a file with missing libraries succeeded** in all five and printed warnings naming the missing file and the missing data-block, with no window.
- **Asking the loader for a file that does not exist raised the same error** in all five.

**In Blender's window, 5.3 alone.** With the middle file gone, the Outliner's Blender File view in 5.3, on the Mac, listed the missing file under Libraries with a warning mark, showed the linked collection under the scene with the linked-data mark, and named the far file nowhere. Untried in 4.5 to 5.2.

**Where it differs, version by version.**

**How much of another file's own chain you can see without opening it.** In **4.5, 5.0 and 5.1** the loader offers no list of the libraries the peeked file itself links — asking for it returns nothing. In **5.2 and 5.3** it does: the test's peek returned the recorded path of the file that file links, together with a flag saying whether it is an archive. Blender 5.2's release notes state the addition and give the same shape of result. So in 5.2 and 5.3 one level of another file's chain can be read without loading it; in the other three it cannot. **Confirmed in 4.5 to 5.2 on both machines and in 5.3 on the Mac.** The rest of what the loader offers about a file it has not opened is under question 1.

**What Blender prints while loading.** In **4.5** the lines are plain: `Info: Read library: '<path>', '//<name>', parent '<direct>'` and `Warning: Unable to open '<path>': No such file or directory`. In **5.0 to 5.3** each line carries a timestamp and a channel: `00:00.267  reports          | Read library: …` and `… | WARNING Unable to open …`. Blender 5.0's release notes say most of Blender now uses one logging system, including operator reports and background render progress, and that the output format has changed. The wording of the messages themselves is otherwise the same in all five, including the summary line counting the missing libraries and data-blocks. **Confirmed in 4.5 to 5.2 on both machines and in 5.3 on the Mac.** Anything that reads Blender's output has to be written per version. **And it should not lean on the channel name:** in 5.3 the same loading lines carried the channel `reports` in runs with no window and `blend.readfile` in a run with a window, on the same Mac.

**One line that is not a version difference.** Blender 5.0 printed a line about memory not being freed as it quit after two of the rented machine's runs, and 4.5 and 5.1 printed something similar once each. Nothing was claimed from it, and nothing should be: the line comes and goes in every version, and **nothing reading Blender's output may treat its presence or absence as telling it which version is running.**

**Where linked data packed into a file is kept.** **Documented:** Blender **5.0's** release notes call the holding place for packed linked items "a new type of 'archive' libraries" owned by the real library. That establishes the archive arrangement as a 5.0 addition. The library-packing operator itself was **observed** in all five versions, 4.5–5.2 on both machines and 5.3 on the Mac. No test here examined an archive library.

**Which packed linked data a save writes.** Blender **5.3's** release notes say that, from 5.3, writing linked data writes only the packed items the file links directly; the indirectly linked ones are used only by other libraries, so writing them into the file is left out. That is a change in 5.3, recorded by its release notes. No test touched it.

**Verdict: it differs.**

**Not established.**

- **What a library's parent property actually holds.** No version's documentation describes it, and it came back empty in every case in every version on every machine, so the tests do not describe it either.
- **Whether a missing image or sound leaves anything equivalent to the stand-in a missing library leaves.** The stand-in text is written about libraries in all five; the plain-image test showed the missing flag reading false in all five. Nobody tested any other kind.
- **Whether packing a library's data changes what the one-level library list reports.** 5.3's API text says packed linked data-blocks are not listed in it. Untested in every version.
- **What a 5.3 save writes of indirectly linked packed data, in practice.** Above.
- **What the Outliner's Blender File view shows with a library missing mid-chain, outside 5.3.** Above.

---

## 9. What separate means exist for listing and rewriting the locations recorded in a `.blend` file, and which kinds of recorded location does each of those means reach?

**How it is known.** The named listing and rewriting means are **documented**, but an exact account of which path kinds each bulk means reaches is **not in the sources**. The results below are **observed**. The write, unpack and default-visitor behaviour therefore rests on tests, rather than the inferences available from their descriptions alone.

**The same in the tested cases, which cover most of the answer.** Unless a line says otherwise, every result here came back identically in all four of 4.5 to 5.2, and every one of them was run **on the rented Linux machine only**. 5.3 ran the same tests **on the Mac only**, and came back the same in every one of them; where it adds something, the line says so.

**What the means reach, put to the test.** A file was built holding **twenty-one kinds of recorded location**, written out in full from the drive root where the setting accepted it — an image, a sound, a font, a text file, a volume, a movie clip, an Alembic cache file, a linked library, the render output path, the bake output path, the compositor File Output node's path, a cloth point cache, a fluid cache folder, a geometry-nodes bake folder, a mesh-cache file, an ocean bake folder, an OSL script, an IES texture, a camera's own shader path, a sequencer strip folder with the sequencer's proxy folder, and a custom property on an object holding a path-shaped string. Then each way of listing and each way of rewriting was pointed at it. 5.3 tested the same kinds, counting the multi-resolution path and proxy folder separately for twenty-three. Its multi-resolution path stayed empty and its Alembic path was already relative.

- **The flat list of external paths names sixteen of the readings and misses seven.** (Sixteen and seven count individual settings read back by name, not the twenty-one kinds — two of the sixteen are two readings of the same image.) The seven it never names are the **render output path, the bake output path, the File Output node's path, the point cache, the geometry-nodes bake folder, the sequencer's proxy folder, and the custom property**. Its three switches change nothing about which kinds appear.
- **The map of data-blocks to their paths names the same set** — on this file, whose linked library holds nothing outside itself. Where a library points at a picture of its own, the map reaches through the library and the flat list does not, so the two are not the same set (**Dependencies 20**, on Windows). The map also tells you which data-block each path belongs to, and it never names a packed thing's path at all.
- **Make Paths Relative reaches the same sixteen readings and leaves seven alone. Make Paths Absolute puts the relative paths in that set into full form.** So the relative sweep misses exactly what the listing means miss. **The absolute sweep was run on the file the relative one had saved**, in which those seven were still written out in full, so its run shows that it changed none of them and not that it passed them by — in 5.3 as in the earlier four. Whether it reaches those settings when they hold relative paths is untried. In 5.3 each sweep also printed a count — sixteen files, fourteen changed by the relative sweep and sixteen by the absolute one, none failed; the earlier records show no such line, and whether the earlier versions print it is not established.
- **Walking Blender's own marks on its settings finds nineteen of the twenty-one** — including six of the seven the sweeps miss. It misses the custom property, which carries no mark and turns up only if custom properties are listed separately; and it misses the text data-block's path, which carries neither mark and whose kind reads as nothing — and that one the sweeps *do* reach.
- **So no single means reaches everything, in any version.** The sweeps and the mark walk together cover all twenty-one but the custom property.

**Path marks identify some recorded paths, but not all.** The render output path carries all three of Blender's path marks and reads as a file path. The text data-block's path carries none of them and reads as nothing. A camera's own shader path reads as a file path but carries neither of the other two marks. Identical in all five versions.

**The rest of what the tests settled, the same in all five.**

- **The "packed" switch adds a packed thing's path rather than removing it.** **Documented:** 4.5 and 5.0 describe the switch as skipping packed paths; 5.1–5.3 describe it as including them. **Observed:** it includes them in all five, so the earlier wording disagrees with its own version's tested behaviour. The plain list leaves it out; the list with the switch on holds it. The map never names it in any version. In 5.3 the one-pass call leaves it out with its default options and visits it with them emptied.
- **Unpacking into the file's own folder rewrites the recorded path**, to a folder beside the file. Unpacking to the original location leaves the recorded path exactly as it was.
- **The wider form of the missing-file search silently repoints links that are not broken.** Two folders each held a file of the same name, both links were proved healthy first, and the search pointed the second at the first folder's file.
- **A path holding a template is left alone by everything.** Neither sweep touches it, no listing means names it, the braces survive in the stored text, and the call that asks what a frame will be written to substitutes the value. Before the file has ever been saved, that call substitutes the word `Unsaved` — in 4.5 to 5.2; 5.3 was not asked that.
- **The write call's four remap choices do four different things**, and it may be pointed at a file that already exists — no error is raised and the file is replaced. In 5.3, opening each file the write call made printed a warning that it is a library file and an empty scene was loaded; the earlier records show no such line, and whether the earlier versions print it is not established.
- **Saving elsewhere with Remap Relative recalculates paths already recorded relative and leaves full-form ones alone** — and **leaves the render output path alone too**, even though that one was recorded relative.
- **Writing a library's own path is not the same act as relocating it.** Assigning the path changes what is recorded at once but leaves the old data in place until the library is told to reload. The relocate operator does both in one act and records the new path in **relative** form, not the full form it was handed — on a file linking one library, aimed at a file no library records. **Dependencies 19** ran the same call on a file linking two libraries and it moved nothing, in 4.5 to 5.2. What separates the two is not established — do not read either as a general rule.
- **The render-output argument on the command line sets the scene's own setting for that run.** An explicit save writes it into the file; without a save the file still holds its old value when reopened. The same is true of a bulk rewrite: it shows in the running Blender and never reaches the file on disk unless saved.
- **Turning a stored path into a real one gives a different answer depending on whether the library it came from is supplied.** Asked plainly, a linked image's path resolves against the open file and names a file that is not there; asked with its library, it resolves against the library and names the real file.
- **The write call refuses to be handed a plain no for its remap choice**, in all five tested versions. **The flat path list rejects positional arguments in all five.** The 4.5 API text lists ordinary arguments, while 5.0–5.3 mark them as name-only. The tested refusal is unchanged; only the listed argument form differs.

**Two results that came from the Mac alone.**

- **In 5.3 and 5.2.2 on the Mac, the image-replacing operator runs without a window when given an Image editor showing the image.** In 4.5 to 5.2, with a window on screen it finished and rewrote the recorded path, and with no window it refused, on the Mac and on the rented machine alike, when no Image editor was handed to it. In 5.3 it refused with nothing handed to it and finished once an Image editor showing the image was handed to it — with no window as well as with a window. A 5.2.2 control on the same Mac answered line for line as 5.3 did. Whether 4.5, 5.0, 5.1 and 5.2.0 run with no window when handed an Image editor is not established.
- **The Outliner's Data API view does show a path value, and shows it as a field rather than as flat text.** The view was reached and photographed with no clicking at all. With the tree opened two levels the file's own recorded path sits in a greyed text field, in all five. In 4.5 to 5.2, with it opened four levels, a data-block's own settings are drawn the same way, and rows named for a camera's shader path and an Alembic cache file's path were reached further down the tree, so path settings are listed under their data-blocks; 5.3's further-opened picture reached the data-blocks' names but not their own settings. **Assigning a new path through the same route the view uses worked** in all five: it raised nothing, read back as the new value, and still read as the new value after the file was saved, closed and opened again — in 5.3, saved one folder over, with the relative path recalculated, since saving remaps relative paths by default.

**Where it differs, version by version.**

**A call that visits every recorded path in one pass, and can rewrite each one by returning a new string.** Asked for it directly, **4.5 says it is not there**; **5.0, 5.1, 5.2 and 5.3** say it is. Where it exists it visited sixteen paths with its filters emptied and rewrote fifteen of them, the sixteenth being handed back unchanged by the test's own callback; in 5.3 it left the same seven kinds alone as the sweeps. Stated in full under question 5.

**What that call hands its callback, and what it will accept.** In **5.2 and 5.3** every visit carries three further yes-or-no marks — whether the path is read-only, whether it is a cache file, whether it was expanded from a tile or a frame — and three extra options are accepted by name. In **5.0** and **5.1** no such third piece of information arrives, and all three of those options are refused by name, with Blender listing the six it does accept. Blender 5.2's release notes state the addition and give render farms as the reason for it. **Not in 4.5 at all**, since the call is not there.

**A read-only list of every data-block in the file.** Absent in **4.5, 5.0 and 5.1**, present in **5.2 and 5.3**. Asked directly, in all five. Stated in full under question 5.

**The name of the File Output node's own path setting.** In **4.5** the node has a base path and no directory setting. In **5.0 to 5.3** it has a directory setting and no base path. Asked directly, in all five. This is one face of the node being rebuilt at 5.0, which question 4 records in full. The same shows up in where the setting is found: in 4.5 the walk reaches it through the scene's own node tree, and in the other four through the compositing node group — which is question 4's change of where the compositor's tree hangs, seen from another angle.

**Writing a library out with everything made relative.** Handed a path that was **already** relative, **4.5 and 5.0 leave it exactly as it stands** — so after the write it points at a file that is not there. **5.1, 5.2 and 5.3 recalculate it** so it still points at the real file. The other three remap choices came out identical in all five. **The same test ran in every version**, which is what makes this a difference in Blender rather than in the documentation; 4.5 to 5.2 ran it on the rented machine and 5.3 on the Mac.

**What the sequencer's change-path operator wants before it will run.** All five rewrite the recorded folder and the file listed in the strip once they run, but they do not ask for the same thing first. In **4.5** the operator answers cancelled until the file is named, and there is no sequencer scene in its context at all. In **5.0** it refuses with "context is incorrect" until the sequencer's own scene is put into the context. In **5.1, 5.2 and 5.3** it refuses with "Context missing sequencer scene" until the same thing is supplied. **What differs between 5.0 and the later versions is the wording of the refusal and nothing else** — all of them want the sequencer's own scene in the context, and none of them accepts the file's name on its own. What the operator demands changes once, between 4.5 and the other four. **Run on the Mac only**, in all five versions, both with a window and without one. **Like the image operator beside it, this one does not need a window**: given the file's name and the sequencer's scene, it ran with no window at all in every version. In 5.3, called with no relative-path setting, it wrote the new folder in the `//` form, and a 5.2.2 control by the same call did the same; which form 4.5 to 5.2.0 write by that call was not asked.

**What one Blender can list out of a file it has not opened.** In **5.2 and 5.3** the loader hands back the recorded paths of the blend-files that file links; in **4.5, 5.0 and 5.1** it offers no such thing. Blender 5.2's release notes state the addition. This is a means of listing recorded locations, so it belongs here as well as where it already stands, under questions 1 and 8, where the same probe was run in 4.5 to 5.2 on both machines and in 5.3 on the Mac.

**Verdict: it differs** — in seven particulars, and in every one of them 5.3 sides with 5.2. Six were established by running the same test in every version, each on the machine its own entry names; the seventh, what the loader lists about an unopened file, is carried over from questions 1 and 8. What the question mainly asks — which kinds of recorded location each means reaches — is answered **the same in all five**, apart from the one-pass visitor, which 4.5 does not have to answer for.

**Two things about the machines, so nobody reads either as a version difference.** The one test that was run on both machines in 4.5 to 5.2 printed twenty-five of its twenty-seven lines identically, character for character. The two that differ are **between the operating systems, not between the versions**: the flat path list holds one entry more on the Mac, a path into the Blender application's own folder, appearing only after the sweep and the save; and asked for its own image file with the name written in capitals — by asking the operating system directly, with none of Blender's own machinery in the question — **the Mac found it and the Linux machine did not, so the Mac's disk does not treat capitals as significant and the Linux machine's does**. 5.3's run of the same reading on the Mac found the capitalised name too, as the earlier Mac run did. **Each speaks only for its own machine.** Smoozy renders on the machine that is not the Mac, and that is the strict one: on it a name whose capitals do not match is not found.

**Not established.**

- **Whether the tests run on one machine alone come out the same on the other.** Nineteen of them ran on the rented machine alone in 4.5 to 5.2 and are untested on macOS there; 5.3 ran them on the Mac alone and is untested on Linux. The one test that was run on both in 4.5 to 5.2 agreed on twenty-five of its twenty-seven lines.
- **What a tool that is not Blender can list or rewrite.** Every version names Blender Asset Tracer in one quoted sentence and says nothing more. Out of the project's reach on purpose.
- **Whether the multi-resolution modifier's own recorded path is reached by anything.** The property would not take a value in any version — the assignment was accepted without complaint and the setting read back empty every time — so no test put a path there. That refusal is uniform and is not itself a difference between the versions.
- **What an image data-block's own path row looks like in the Outliner's Data API view.** The view shows path values as fields, but that row was not seen in any version. That does not establish that it is absent.
- **Whether the image-replacing operator runs with no window in 4.5, 5.0, 5.1 and 5.2.0 when handed an Image editor.** Above.
- **Whether the sequencer operator runs with no window on Linux when given what it wants.** It was only asked that way on the Mac. The rented machine's run supplied neither the file name nor the sequencer's scene, so its refusal there says nothing either way.
- **Whether any bulk means reaches a path held inside a custom property.** It does not, in any version — but nothing establishes whether that is deliberate.
- **Whether the sweeps' count line, and the write call's warning on opening its file, are printed by the earlier versions.** Above.
- **Whether the bulk sweep hits Windows' own wall.** Making paths relative across two Windows drives is documented as capable of failing; no test here reached it. Uniform in all five versions, and never a difference between them.

---

## 10. When a search for a missing reference finds more than one possible file, what makes a file a possible match, which one is chosen, and what location is written back?

**How it is known.** Find Missing Files and its recursive search are **documented**. Its matching and choice rules are **not in the sources**. All five versions' quoted manual text says recoveries are absolute; the tests below qualify that claim.

Everything else below is **observed**. Ten hand tests answered this question in 4.5 to 5.2, and **every one of them was run in all four versions on both machines**, the Creator's Mac and the rented Linux machine; the two machines agreed on every point. 5.3 ran the same kinds of test on the Mac. The command was called from a script in a Blender started with no window, with a fresh working folder, fresh settings and factory startup for every run.

**Verdict: the same in the tested cases; not established beyond those machines.** The 4.5–5.2 tests ran on the Mac and Linux; 5.3 ran on the Mac alone. No result establishes the matching or choice rules on Windows or in 5.3 on Linux.

**What makes a file a possible match: the file name, exactly.** A file of the same name in a sub-folder was accepted. A file with the same stem and a different extension was not. A file with a different name holding the same picture was not. Both refusals printed the same line as a folder holding nothing at all — the command saying it could not find the name it was looking for.

**Letter case had to match on the machines tested.** With only an upper-case name on disk and a lower-case one recorded, neither machine recovered it, in any version, while the control run with the matching name recovered it. **The Creator's Mac gave that answer, in all five, even though its own file system does not treat capital letters in a file name as significant** — asked directly for a file by a name written in capitals when the name on disk is in small letters, that Mac finds it and the Linux machine does not. So this is Blender's own rule and not the file system's. Windows is untried.

**Which one is chosen when several match: the largest file on disk.** Ten arrangements were run in each of 4.5 to 5.2 on each machine, and eight such arrangements in 5.3 on the Mac. Wherever the candidates differed in size, the recovery pointed at the biggest one — whichever sub-folder it sat in, whatever the sub-folders were called, whatever order the file system listed them in, and however deep it was buried. Swapping the two files between the folders swapped the answer with them. A file three folders down beat one at the top of the search folder on a single byte in 4.5 to 5.2, and on eleven bytes in 5.3. **Where two candidates were byte-for-byte the same size, the one in the folder named `aaa` was taken rather than the one in `zzz`**, and in 5.3 giving either file an older date made no difference — that is what happened, and nothing establishes the rule behind it.

**Nothing at all is said about there having been more than one.** No line about ambiguity appeared in any arrangement, in any version, on either machine. The control is what makes that worth recording: with nothing to find, the same terminal carried the "could not find" warning, so this is a silence about ambiguity and not a channel nobody was watching. **What appears in the Info editor or the status bar is a separate question and is untried everywhere.**

**The recovery rewrites the recorded location without reading the file.** Straight after a successful recovery, the image reported that it held no data; reading its pixels afterwards then returned the new file's colour. So the file is read when it is next wanted, not at the moment of the repair.

**The search goes at least eight folders deep.** A file eight folders below the chosen one was found, in every version. Nothing deeper was tried.

**It reaches every one of the six kinds that were tried.** One blend-file pointing at a broken image, sound, font, movie clip, linked blend-file and Alembic cache archive had **all six** recorded locations rewritten by a single run — the linked blend-file included, although the manual gives linked files a separate repair tool of their own. All six were repaired in 4.5–5.2 on both machines and in 5.3 on the Mac. With the search folder empty, the control printed one "could not find" line per kind and moved none of them. What a missing library's data does after its path is rewritten is under question 6.

**The wider form of the search rewrites locations that were not broken.** With the option off, only the broken reference moved. With it on, a reference whose file still existed was repointed as well, to a same-named file in the search folder.

**A packed resource's recorded location is rewritten too, and it stays packed.**

**Whether a recovery can be undone is not settled anywhere.** With no window the undo command refused in all five versions — even with a viewport context assembled by hand, and in 4.5 to 5.2 even after the undo system had been started by hand — and the recovered location stayed as the search had written it. What a person pressing undo at a window would get is untried in every version.

**One thing to keep apart: a document disagreeing with a machine.**

**The location written back keeps the form it was recorded in, at least in 5.3, and every manual says absolute.** In 4.5 to 5.2, on both machines, every path repaired in this topic had been recorded in the `//` form Blender uses for "relative to this blend-file", and every one came back in that form: `//orig/tex.png` became `//s_lower/deep/tex.png`. **Dependencies 8** ran the same command on paths written out in full from the drive root, and those came back written out in full. In 5.3, on the Mac, both forms were put to the same version: a path recorded in full came back in full and one recorded in the `//` form came back in that form, for a single image and again for six kinds at once. Every version's manual says that recoveries are made as absolute paths, as quoted in the findings.

**This is a document disagreeing with a machine, not one version disagreeing with another.** The manual is matched by the full-form repairs and contradicted by the `//` ones, in every version that was asked. In 5.3 the repair keeps the form it found. For 4.5 to 5.2 no single run put both forms to the same version, so that it keeps the form there is the pattern every run fits and not something one run showed. Preserving both forms is observed in 5.3 on the Mac. In 4.5–5.2 it remains an inference from separate tests; the relative-path repairs already show that the manual's absolute-only wording is not a safe general rule.

**The test pictures had different byte sizes.** The larger-candidate and exact-tie tests still gave the same answers at each set of sizes. Those test-input sizes establish nothing further about the recovery rule.

**Not established.**

- **Whether a differently-cased name matches on Windows** — that is, on a file system that preserves capital letters without treating them as significant. No Windows machine ran it. Uniform in all five versions and never a difference between them.
- **Whether undo reverses a recovery at a window**, and **what a panel shows** when a search finds several candidates. Both need eyes on a window; no machine gave one.
- **How deep the search really goes.** Eight folders is a floor, not a limit.
- **How matching works for image sequences and UDIM sets.** Untested here.
- **Whether the command reports how many references it fixed.** Nothing appeared in the terminal beyond the failures.
- **Whether a repaired location keeps the form it was recorded in, in 4.5 to 5.2.** It does in 5.3. In the earlier four, this topic's runs repaired only `//` paths and **Dependencies 8**'s only full ones.
- **Whether 5.3's answer holds on Linux.** Untried.

---

## 11. What happens when a file Blender has already read is changed or replaced on disk, and what has to happen before a later render uses the changed file?

**How it is known.** Explicit reload commands, changed-text detection and packed data ignoring the disk are **documented**. Automatic texture-cache regeneration is documented in the 5.2 and 5.3 findings. Whether a render re-reads other changed files needs a test; a conclusion from the existence of reload commands alone is **inferred**. The results below are **observed**.

Everything else below is **observed**. Eighteen hand tests answered this question in 4.5 to 5.2, and **every one of them was run in all four versions on both machines**, the Creator's Mac and the rented Linux machine; their conflicting results are stated below. 5.3 ran fourteen of the same kind on the Mac. The basic image scene was a plane filling the camera and taking its colour from one image, rendered by Cycles on the processor at eight pixels square, so a render could be read as a single colour; files were replaced by the operating system rather than by Blender. Unless a timing case below says otherwise, replacements were followed by two and a half clear seconds so a modification time recorded only to the second was plainly newer. These render results do not establish what graphics-card rendering does, on any make or number of cards. Where runs disagree, that disagreement remains open.

**The same in the tested cases.** In 4.5 to 5.2 on both machines and in 5.3 on the Mac:

**A changed image reaches the very next render inside a running Blender, with nothing done at all.** The first render was red, the file on disk was overwritten with a blue one, and the next render — nothing touched in between — was blue. A second swap was picked up the same way. It held in all five tested versions.

**A frame file of an image sequence behaves the same way, and the Auto Refresh setting makes no difference to it.** With the sequence proved to be following the timeline, an overwritten frame file reached the very next render — with the setting off and with it on, identically. Auto Refresh is about the frame changing, not the file changing, and turning it on changed none of the readings.

**A freshly started Blender read the changed files in the tested cases, with two exceptions.** A second Blender process opening the same blend-file after a swap rendered the new picture where the first had rendered the old one. This is **observed** for the files tested. Two exceptions, both under this question: an Alembic archive the importer gave no cache reader, and a texture-cache copy made in the same second as the change.

**The Persistent Data setting stops the change being picked up.** With it off, the changed image reached the second render. With it on, the second render showed the old picture — and still showed the old picture after the image was told to reload. The setting is present in all five versions and is off by default. **This setting stopped the changed image reaching the processor render in every tested version.**

**A linked blend-file behaves the other way round from an image.** Replaced on disk under a running Blender, it did **not** reach the next render. Telling the linked file's own data to reload brought the new version in. A freshly started Blender rendered the new version. The separate command for reloading a named linked file refused every way it was tried, in all five versions, complaining that what it was handed was not a library — in 5.3 it was handed the library's name, and then its name and its recorded path, and the second call also warned that the command's path setting does not take the `//` form; how that command is meant to be called is owed.

**A movie clip does not pick a change up on its own**, and the command that reloads it does re-read the file. The clip's recorded size stayed as it was after the file was replaced with one of a different size; the reload command refused with no clip editor and changed nothing, and with one assembled by hand it finished and the new size appeared.

**Blender does notice a changed external text file, and the conflict can be resolved with no window.** The flag that says the copy on disk differs from the one Blender holds was false while the two agreed and true once they did not, in every version. The text Blender held did not change until a reload was run; the reload command refused with no text editor and finished with one assembled by hand, and so did the command that resolves the conflict by reloading. **So the tested Blenders could settle that conflict with no window.** **One timing caveat:** in 5.3, and in a 5.2.2 control on the same Mac, a rewrite made within a fraction of a second of the text being loaded was not flagged; one made seconds later was.

**A packed image ignores the file on disk for the purposes of a render.** With the image packed, the render went on showing the old picture after the file was replaced, in every run of every version. The control with the image left unpacked went from old to new, which is what shows the swap really bit. *(What the reload command then does to a packed image is the one part of this that came out two ways; see below.)*

**A file replaced while a render is already running takes effect from a later frame onward.** In a twelve-frame animation whose image was replaced part-way through, the frames up to some point held the old picture and every frame after it held the new one. **No frame came out part old and part new**, in any of the eight runs of 4.5 to 5.2, nor in 5.3, where every pixel of every frame was read. Which frame the change lands on moved from run to run and from machine to machine — frames four to nine across the eight runs, frame eight in 5.3, and every frame new in a 5.3 run whose swap landed before the first frame — because it depends on how far the render had got when the swap landed. **That is timing, and not a difference between versions.**

**The blend-file itself, replaced on disk while Blender holds it open.** Blender went on holding what it had, with nothing done. Revert then gave back **the file as it stands on disk**, not the version Blender had been opened from, and it ran with no window in all five versions. A control revert with nothing replaced gave the original back. Whether Blender notices on its own that its own file changed is untried — it needs a window.

**How the reload commands behaved with no window in these tests.** The reloads built into an image's own data and a linked file's own data, and the revert command, all ran with no window, in all five. The command forms for image, movie clip, cache archive and text all refused unless an editor of the right kind was assembled by hand out of the areas a windowless Blender still keeps, and then they finished. For the image, the editor alone is not enough — the image has to be put into that editor before the command will run. **The command that updates a shader script node refused in every arrangement tried in 4.5 to 5.2.0**, in the calls recorded there. **In 5.3, and in a 5.2.2 control on the same Mac, it finished** once a Shader node editor showing the material's tree, with the script node, was handed to it, reporting that the shader compiled. Whether 4.5 to 5.2.0 run it when handed the same is not established. The sequencer's two commands are below.

**Where it differs, version by version.**

**A changed picture behind a Video Sequencer strip.** One image strip pointing at a file that was then replaced on disk, and the scene rendered again with nothing else done. In **4.5** the render showed the new picture, and both sequencer commands — reload strips, and refresh everything — ran with no window and reported finished. In **5.0, 5.1, 5.2 and 5.3** the render went on showing the old picture, and **both commands refused when nothing was handed to them, saying the context is incorrect**. In 4.5 to 5.2 the two machines agreed with each other version for version, so this is a difference in Blender and not in the operating system. In 5.0, 5.1 and 5.2.0 the commands refused in every arrangement tried. **In 5.3, and in a 5.2.2 control on the same Mac, both commands ran with no window once a Sequencer editor and the sequencer's own scene were handed to them, and the next render showed the new picture.** For a render driven from a script on a machine with no display, the practical shape of it is: in 4.5 a replaced strip file reaches the render by itself; from 5.0 it does not; and in 5.3, and in 5.2.2, handing the reload commands a Sequencer editor and the sequencer's scene brings it in. Whether the same route works in 5.0, 5.1 and 5.2.0 is not established.

**Cycles' cache of prepared texture copies.** Asked directly, the two settings and the two commands behind it were **absent in 4.5, 5.0 and 5.1 and present in 5.2 and 5.3** — 4.5 to 5.2 on both machines, 5.3 on the Mac. In both, the cache itself is on by default and the automatic build is off by default. With both switched on, a prepared copy of the image appeared in a folder beside it after the first render; the source image was then overwritten, and the next render **showed the new picture** and the prepared copy had been rebuilt — its recorded time had moved on. **One machine difference inside that result, and it changed nothing:** on the rented Linux machine the prepared copy also changed size, from 1597 bytes to 1594; on the Creator's Mac it stayed at 1692 bytes in 5.2 and at 1640 bytes in 5.3, and only the time moved. The Mac and Linux size patterns differ on those machines, but no operating-system cause is established. The two Mac sizes also came from different runs and do not establish a version change. Three further results, from the Mac:

- **A change in the same whole second is missed.** When the source image was replaced in the same whole second as the prepared copy had been made — an eight-by-eight render takes well under a second — the copy was not rebuilt and the next render showed the old picture, and so did a freshly started Blender. In 5.3 and in 5.2.2 alike. The copy's recorded time was a whole second, and the source image's time afterwards fell in that same second.
- **With the automatic build switched off after a copy was made**, and the source then changed, the next render showed the new picture and so did a fresh Blender's, and the out-of-date copy was left as it was. 5.3 only.
- **With the automatic build off from the start**, no folder of copies was made and the new picture showed. 5.3 only.

**The call that visits every recorded path in one pass** is **absent in 4.5 and present in 5.0, 5.1, 5.2 and 5.3**, asked directly — 4.5 to 5.2 on both machines, 5.3 on the Mac; it is stated in full under question 5. What this question adds is one thing, one answer for one version, and one silence. The thing: handed the same mistyped option name, **5.1 and 5.2 say what spelling they expected and 5.0 only says the keyword is invalid**, so the message differs between 5.0 and the two versions after it; 5.3 was not handed it. The answer: **in 5.3 the option named for reloading edited data reloads** — an image whose picture had been read and whose path was rewritten through the call with that option showed its new file at once, where the call's default options left the old picture in place; 5.3's documents name the option and say it reloads data when the path is edited. The silence: **what that option does in 5.0, 5.1 and 5.2 is owed.** Those tests used a misspelt keyword and errored. No documentation change is established by 5.3 naming the option.

**Verdict: it differs in the tested cases.** The sequencer's unaided update and the texture-cache machinery differ as above; the mistyped-keyword error also differs in 5.0–5.2. The 5.3 results settle only its tested Mac, and the render results are for the processor. The successful 5.3 and 5.2.2 editor-assisted commands do not establish a change from the earlier, differently called tests.

**Three things repeated runs left unsettled.**

The packed-image and Alembic runs agreed across 4.5–5.2 within each set-up. The shader-script repeat also differed between machines within the same version. None of these conflicting results establishes a version difference, and the disagreements remain open.

**Whether the reload built into an image's own data brings a changed file into a packed image.** Three runs stand side by side for 4.5 to 5.2. One result: after the reload, the render still showed the **old** picture. Another result: the render showed the **new** picture, with the image still packed. The paired saved-and-reopened and saved-without-reopening cases got the **new** picture in every version on both machines in both packed arms, and found that reopening made no difference at all, so that is not what separates the earlier two. 5.3's one run, on the Mac, ran the same two packed arms and got the **new** picture in both, the image still packed. **All the runs stand and none has been preferred.** What every run agrees on: with nothing done, a packed image's render shows the old picture, and the reload raises no error either way.

**Whether a changed external shader script reaches the very next render.** Three runs for 4.5 to 5.2. One result: the render showed the **new** colour, in all four versions on both machines. Another result: mixed — on its Mac 4.5 and 5.0 new, 5.1 and 5.2 old; on its Linux machine 4.5, 5.0 and 5.1 old and 5.2 new. **Its own two machines disagreed with each other inside the same version, so nothing in that run can be read as a version difference either.** A third run, with the compiled copy of the script watched and the clock measured: the **new** colour in all four versions on both machines, with the compiled copy rebuilt during that very render, and with a control showing both machines record a file's modification time to the nanosecond and the script's own recorded time had moved on by about three seconds. Two further controls held: a genuinely separate Blender rendered the new colour, and did so again after every compiled copy had been deleted. In **5.3** two runs on the Mac both gave the **new** colour at the very next render, with the compiled copy rebuilt during it — once with the script rewritten a fraction of a second after the compiled copy was made but across a whole-second boundary, once two and a half seconds after. **A 5.2.2 control on the same Mac points at timing:** with the script rewritten in the same whole second as the compiled copy, the next render kept the **old** colour and the copy was not rebuilt; rewritten two and a half seconds later, it showed the new colour. That fits every run whose timing is recorded; the mixed repeat did not record its timing, and 5.3 was not given a rewrite in the same second. **All the runs stand and none has been preferred.** What every run agrees on: the automatic-update switch is present in all five versions and off by default. The switch and its purpose are **documented** for 4.5; no change in the other versions' own documentation is established here.

**Whether an Alembic cache archive picks up a change on disk.** The 4.5–5.2 results disagree across different set-ups. One result: the archive was never picked up at any point — not on its own, not after the reload command was cancelled for want of a context, not after the same command finished with a context assembled by hand, not after the recorded location was written back to itself, not after the frame was stepped away and back. With the importer asked to add a cache reader, **the reload command with a context assembled by hand then did bring the bigger cube in**, and it stayed. The importer's default set-up produced an object carrying **no cache reader at all** — so nothing on it was tied to the archive — and a fresh Blender gave the same unchanged size. 5.3 on the Mac gave: with the importer's defaults the object carried no cache reader and nothing, not even a fresh Blender, showed the new archive; with the cache reader the reload with a context assembled by hand brought the bigger cube in, and a fresh Blender showed it. **Both earlier runs stand and neither has been preferred.** What every run agrees on: with nothing done, and with the reload command cancelled for want of a context, the old archive is what shows.

**Not established.**

- **Whether the processor-render results hold for graphics-card rendering**, including other makes and several cards. Untried here.
- **Whether a file can be replaced on disk at all while Blender holds it open, on Windows.** No Windows machine ran it. Uniform in all five versions.
- **Whether the changed-text warning banner is drawn**, and **whether anything appears in the Info editor or the status bar** when a file changes or a render begins. Both need a window; untried in all five versions.
- **What the option named for reloading edited data does, in 5.0, 5.1 and 5.2.** Above.
- **How the command for reloading a named linked file is meant to be called.** Above.
- **Whether the sequencer's reload commands, handed a Sequencer editor and the sequencer's scene, bring a changed strip file in, in 5.0, 5.1 and 5.2.0**; and **whether the shader-script update command runs there when handed a Shader node editor.** Both do in 5.3 and in 5.2.2.
- **Whether changes made inside the same whole second are missed outside the Mac, and in 4.5 to 5.1.** Above, for the texture cache, the shader script and the text file.
- **What the texture cache does with its automatic build switched off after a copy was made, in 5.2.** Above.
- **What happens when the replacement differs in size, format or colour depth from the file it replaces** — except for the movie clip, whose replacement was deliberately a different size, and where the reload showed the new size.
- **What happens if the file is deleted rather than changed.**
- **Whether Blender notices on its own that its own open blend-file has been replaced.** Needs a window.
- **Whether 5.3's answer holds on Linux.** Untried.
