# Settings — Blender 4.5, 5.0, 5.1, 5.2 and 5.3 side by side

Which settings a render uses, where each of them lives, and which of them can be set at the moment a render is started from outside Blender.

## The machines behind the tests

Six machines ran this topic's tests. The equipment section of `Preparation/Blender Research/Method.md` describes every one of them in full; these lines are only enough to place a result.

- **The Creator's Mac.** A MacBook Pro with Apple M1 Pro, its own 14-core Metal graphics, no second card and 16 GB of memory, with a screen. Questions 1–16 used Blender 4.5.12, 5.0.1, 5.1.2 and **5.2.0**; questions 17–19 used **5.2.1 LTS**, hash `9e2066aef7ef`, with the other three unchanged. Earlier tests report macOS 26.5.2; questions 18–19 and the 5.3 tests report **macOS 26.6.2**. Those system and build differences qualify the results. **Every 5.3 test ran here**, in Blender 5.3.0 Beta, hash `5d8151335b6b`, built 2026-10-16. Controls used **5.2.2 LTS**, hash `d13f752e3b9c`, on the same Mac. Blender windows were opened and photographed from a terminal; 5.3's were driven by simulated clicks and keys. Nobody tested typing into a field, dragging a control or hovering for a tooltip in any version.
- **The one-card rented Linux machine.** One NVIDIA GeForce RTX 4090, reached over SSH. It **has a display server of its own**, so a run there could be cut off from the display but was never on a machine that had none. It shares someone else's hardware, and up to four topics were being worked on it at once, so **nothing from it says anything about how long a run took or how much memory was free**.
- **The two-card rented Linux machine.** Two NVIDIA GeForce RTX 4090s and **no display server at all** — none was installed and none was started, and its absence is the thing being tested. Four tests only. Nothing in this topic needed its second graphics card.
- **The RTX 5060 rented Linux machine.** One NVIDIA GeForce RTX 5060, **no display server at all**. Questions 14 and 15 rest on this machine and on the Mac and on nothing else.
- **The RTX 4060 Ti rented Linux machine.** One NVIDIA GeForce RTX 4060 Ti, **no display server at all**. Question 16 rests on this machine and on the Mac and on nothing else. Its graphics driver changed part-way through, from 580.95.05 to 580.173.02, and one result under question 16 turns on that and says so.
- **The RTX 3060 rented Linux machine.** One NVIDIA GeForce RTX 3060 with 12 GB on driver 550.120, a container sharing someone else's hardware. Questions 18 and 19 rest on this machine and on the Mac and on nothing else. It had **no display server as rented**; one was installed on it in software for a single test, which asks only what a panel draws, and that test says so on its face — **a software-drawn display is no workstation's screen**. Its four Blenders were checked against the Mac's before anything was run and every build hash matched, so the two machines were running the same four Blenders.

**No machine speaks for another.** Where the same test was run on more than one, the answer below says so and names each. Where it was run on one, the answer says which. No result from one machine is ever written as though it covered another. **A difference between these machines is never a difference between Blender versions**, and where two of them behaved differently in the same version, that is written inside the answer it belongs to.

**"Both machines" means four different pairs, and each answer says which.** Under questions 1 to 13 it means the Creator's Mac and the one-card Linux machine, which are the two that ran this topic's full list; the two-card machine ran four tests only and is always named in so many words wherever it is the source. Under questions 14 and 15 it means the Creator's Mac and the RTX 5060 machine. Under question 16 it means the Creator's Mac and the RTX 4060 Ti machine. Under questions 18 and 19 it means the Creator's Mac and the RTX 3060 machine. **"Both machines" never includes Blender 5.3**, whose every result is from the Creator's Mac alone.

**Question 17 has no pair at all.** Every one of its runs was made on the Creator's Mac and on no other computer. **Nothing was rented for it**, so no Linux machine ran any part of it, and no Windows machine ran it there any more than anywhere else in this topic. **That bounds its runs and not its subject**: three of the six names it is about are exercised on Linux by questions 1, 14 and 16 and by **What a render demands 3**, on four different rented machines in all four earlier versions, so the Linux gap is much smaller than one machine would suggest. The head of question 17 sets out which parts really do rest on the Mac alone.

**No figure measured on one rented machine is ever set against a figure measured on another**, and none is below. They are different computers with different cards, different amounts of memory and different drivers.

**A 5.3 result is set against a 4.5 to 5.2 result only where both came from the Mac.** Where 4.5 to 5.2 have a Linux result and 5.3 has only a Mac one, the two are given side by side and never called the same or different: together they say something about Linux in the earlier four and macOS in 5.3, and nothing about macOS in the earlier four or Linux in 5.3. Every difference between 5.2 and 5.3 in the first list below rests on the same test run on the Mac in both, or on 5.3's own release notes or change log.

**Every Cycles render behind question 16 was on the processor, not on the graphics card**, in every version on both of that question's machines. A throwaway settings folder switches no graphics device on, and the test scene sets Cycles' device to the processor. Nothing under question 16 says anything about a render on a graphics card.

**Two things about the Mac, and both are about the same trap.** The Creator's own Blender was open and working on that machine throughout the second batch of question 16's tests, with a background helper and a BlenderKit client beside it; the machine's load average climbed from 2.54 to 23.77 while those ran. The first batch's Mac figures were taken with nothing else on the machine, and the rented Linux machine was idle throughout both batches. So **a timing that differs between the two machines in that batch is not an operating-system fact** — that load could explain it — and nothing below is built on one. Separately, **no timing taken on that Mac on the day questions 18 and 19 were run is clean**, because other work was driving Blender on it all day. Nothing under either of those two questions is a timing, and the one place a clock reading appears there says what it is worth. **Other programs were running on the Mac when the 5.3 tests were run as well**, so no 5.3 timing is compared with any other run's, and none below is.

**A result settles only the machines it ran on.** A result that came back the same in every version still settles the computers it was run on; whether it holds on any other is untested, and untested everywhere is not a difference between versions.

**Windows was never tried anywhere, in any version, on any question.** This is untested rather than impossible. The equipment section of `Preparation/Blender Research/Method.md` describes a route to Windows machines; its machines reported Server editions, so any result would need to name its edition. No Mac or Linux result here establishes a Windows answer.

**Blender 5.3 was run on no Linux machine, no Windows machine and no graphics card but the Mac's own.** So everything this document settles on a rented Linux machine or on an NVIDIA card is settled for 4.5 to 5.2 and untested in 5.3, and every 5.3 test result below is a macOS result.

---

## The answer in short

### Where Blender itself differs between versions

These are version differences on the machines named under each question, or documented additions and changes. A result on one machine does not establish it on another. Every 5.3 test was on the Creator's Mac. Release-note claims remain labelled untested where no test reached them.

- **2, 3, 16 — EEVEE gains a Time Limit in 5.3.** Documented change; properties and panel checked against 5.2.2 on the same Mac. Full answer under 2 and 16.
- **16 — the closing zero-sample line is absent in 5.3 under a time limit and adaptive sampling.** Reports 11 tested 5.2.2 beside 5.3 on the same Mac; cancellation remains unresolved. Full answer under 16.
- **13 — EEVEE's identifier changes at 5.0.** Acceptance of the old name depends on the route; this topic did not try it in 5.3, and Images 9 records 5.3 refusing it from a script. Full answer under 13.
- **13 — 4.5 ends badly on engine-list and bad-engine requests; 5.0–5.3 end cleanly.** The symptom varies by machine. Full answer under 13.
- **17 — 4.5 alone silently accepts misspelled Cycles setting names.** Tested on the Mac, 5.3 included. Full answer under 17.
- **17 — the declared view-layer switch default differs in 4.5; real layers start on in all five.** Tested on the Mac. Full answer under 17.
- **17 — a relative output path without a loaded file works in 4.5 and fails in 5.0–5.2.** Mac only; 5.3 untried. Full answer under 17.
- **17 — launch sub-commands increase from one to three in 5.2 and 5.3.** Mac only; none sets a render setting. Full answer under 17.
- **1 — 5.3 adds `--debug-gpu-backend-no-fallback`.** Help text compared with 5.2.2 on the same Mac; fallback behaviour untested. Full answer under 1.
- **6 — refused-window output and exit behaviour differ in 4.5–5.2.** Linux without a reachable display; 5.3 untried. Full answer under 6.
- **15 — ordinary render logs change shape at 5.0.** 5.3 prints the newer shape on the Mac. Full answer under 15.
- **9 — the compositor starts on CPU through 5.1 and GPU in 5.2 and 5.3.** Full answer under 9.
- **9 — Workbench's Render tab controls and some labels change between versions.** 5.3 matches the tested 5.2 device controls on the Mac. Full answer under 9.
- **13 — Cycles shows Feature Set and Device in 4.5, Device alone in 5.0–5.3.** The property itself is absent in 5.0–5.3, observed under What a render demands 1 and 4; 5.3 on the Mac alone. Full answer under 13.
- **16 — EEVEE obeys view-layer sample overrides from 5.0; 4.5 ignores them.** 5.3 obeys them on the Mac. Full answer under 16.
- **12 — a separate Sequencer Scene exists from 5.0.** Bare command-line renders ignore it, including in 5.3. Full answer under 12.
- **8 — a Scene strip gains a view-layer selector in 5.2.** Release note; 5.3's setting unestablished. Full answer under 8.
- **8 — the compositor renders scenes referenced inside node groups from 5.3.** Release note, untested. Full answer under 8.
- **8 — how a scene holds its compositor changes at 5.0.** The tested rendering result remains the same. Full answer under 8.
- **7 — EEVEE gains panoramic-camera rendering in 5.3.** Release note, untested; the limitations page disagrees. Full answer under 7.
- **10 — each version's release notes identify particular compatibility changes.** The general compatibility statement agrees. Full answer under 10.
- **10 — 4.5 prints extra warnings and errors while successfully opening a newer file.** Tested beside 5.0 and 5.1; 5.2.2 warns alone on the Mac; 5.3 untried. Full answer under 10.
- **11 — Python-environment launch arguments grow from one to two in 5.1.** 5.3 carries both on the Mac. Full answer under 11.
- **1 — `--gpu-device` and its no-fallback companion appear in 5.2 and 5.3.** Earlier-version check on Linux; 5.2.2 and 5.3 help checked on the Mac. Full answer under 1.
- **9 — HIP's missing-library warning is louder from 5.0.** Tested on Linux in 4.5–5.2; 5.3 untried there. Full answer under 9.
- **3 — EEVEE's settings block changes size across versions.** 5.3's extra property against 5.2.2 is Time Limit. Full answer under 3.
- **6 — `gpu.init()` is added in 5.2 and documented in 5.3.** No tested render needed it. Full answer under 6.
- **15 — the script render-operator route prints progress in 4.5 and none in 5.0–5.3.** Full answer under 15.
- **14 — Cycles' panel code reads resolution percentage from 5.0.** This does not change its accepted range. Full answer under 14.
- **14 — Cycles gains two Texture Resolution settings in 5.2, retained in 5.3.** Separate from render resolution percentage. Full answer under 14.
- **14 — 5.3 has four more render presets than 5.2.2 on the same Mac.** Full answer under 14.
- **5 — the Max Samples menu adds trailing dots to Add Driver in 5.1.** 5.3's Mac menu matches 5.2. Full answer under 5.
- **18 — 4.5's factory Grease Pencil count is 1; the other four hold 8.** Declared default is 8 in all five. Full answer under 18.
- **19 — Cycles view-layer denoising settings increase in 5.2 and again in 5.3.** Full answer under 19.
- **19 — 5.3 adds viewport upscale quality and a DLSS viewport denoiser.** Property checked against 5.2.2 on the Mac; DLSS release note, untested. Full answer under 19.
- **19 — denoising feature-pass number and order differ across versions.** 5.3 matches 5.2 on the Mac. Full answer under 19.
- **19 — node quality is a property in 4.5 and a Quality input in 5.2.2 and 5.3 on the Mac.** Input coverage in intervening versions is incomplete. Full answer under 19.
- **19 — EEVEE view-layer settings increase, gaining denoising settings in 5.3.** Change log and same-Mac control. Full answer under 19.
- **19 — OptiX tracing fails in 5.2.1 where 4.5–5.1 work on one RTX 3060 with driver 550.120.** Other machines and 5.3 untested. Full answer under 19.

### What is not established

These are gaps, not version differences. Each question gives the full limits and any results available for only some versions.

- **All questions — Windows in every version; Linux and other graphics cards in 5.3.** The 5.3 observations reach the Creator's Mac alone.
- **1 — Mac 5.2.0/5.2.1 support for `--gpu-device`; the all-layers-off result before 5.3.** Full answer under 1.
- **1, 5, 17 — view-layer right-click menus before 5.3; hovering tooltips in every version.** Full answers under 1 and 17.
- **2, 16 — whether a time-limited frame is denoised; what the render window shows before 5.3.** Full answer under 2.
- **5 — keyed EEVEE and view-layer counts rendered frame by frame before 5.3; keyed-field colours before 5.3.** Full answer under 5.
- **6 — screenless rendering in 5.3; headless Windows, a screenless Mac and a computer without graphics hardware in every version.** Full answer under 6.
- **6 — the unassigned/hidden-camera Workbench result outside the two tested 5.3 Mac files.** Full answer under 6.
- **7 — the exact panoramic projections added in 5.3.** Full answer under 7.
- **8 — frame mapping and chains deeper than two scenes; the 5.3 Scene-strip view-layer selector; sequencer-versus-host-compositor tests before 5.3.** Full answer under 8.
- **9 — GPU Compute on AMD or Intel; NVIDIA behaviour and the Linux HIP warning in 5.3.** Full answer under 9.
- **10 — 5.3 opening a newer file; exact counts of 4.5's extra compatibility messages.** Full answer under 10.
- **11 — Freestyle scripts without the enabling flag; registered text, explicit text and OSL tests before 5.3.** Full answer under 11.
- **12 — the active scene recorded by a file saved from Blender's window.** Full answer under 12.
- **13 — 5.3 acceptance of EEVEE's old name on the command line; the actual engine and warnings under that name in 5.0–5.2.** Full answer under 13.
- **13 — `--addons` enabling a third-party engine early enough.** Full answer under 13.
- **14, 15 — typed or dragged percentages, animation or drivers outside the range, and out-of-range values loaded from files.** Full answers under 14 and 15.
- **15 — reading settings without rendering before 5.3.** Full answer under 15.
- **16 — the automatic threshold; much older files' sample conversion; GPU and multi-layer time limits.** Full answer under 16.
- **16 — Layer Samples visibility and effect while hidden; per-pixel counts; the general combination and convergence rules; window-started renders beyond the 5.3 time-limit test.** Full answer under 16.
- **16 — trailing zero sample lines after cancellation across every version.** Full answer under 16.
- **17 — Linux coverage of two names and the Mac-only version differences; other routes, shared counts, other engines and scene selection for the instruction.** Full answer under 17.
- **17 — whether Workbench's fixed anti-aliasing choices count as a sample count; relative output without a file in 5.3.** Full answer under 17.
- **18 — sample counts driven or keyed outside the range, loaded out of range, typed or dragged.** Full answer under 18.
- **19 — AMD/Intel and multi-card denoising; OptiX settings and tracing in 5.3; the effect of added denoising settings on a picture.** Full answer under 19.
- **19 — Quality inputs in 5.0, 5.1, 5.2.1 and Linux 5.2.2/5.3; non-default node quality; viewport and EEVEE ray-tracing denoising in a final render; engine/node denoising interaction.** Full answer under 19.
- **19 — the dimmed checkbox on one machine across all five versions.** Earlier versions used Linux software drawing; 5.3 used the Mac. Full answer under 19.

### Where only the documentation differs

- **9 — where the Cycles Device control is documented.** The setting and tested overrides agree. Full answer under 9.
- **17 — API choice-type notation changes at 5.1, with a further Cycles type notation difference recorded in 5.2.** Full answer under 17.
- **14 — the same resolution-percentage range is printed differently from 5.1.** Full answer under 14.

---

## 1. Which of a render's settings can be settled at the moment a render is started from outside Blender, and which can only be changed by changing the file?

**How it is known.** The launch arguments are **documented**. Setting other values with a launch-time Python expression is **inferred** from those arguments and the property names, then **observed** in every version: 4.5–5.2 on the Mac and one-card Linux machine, 5.3 on the Mac. View-layer keyframes were also tested. The earlier Mac window checks were Creator-run; 5.3 used simulated input.

**Verdict: the same everywhere.**

The answer, once:

**Settled at launch, by an argument of its own — in all five versions.** A frame range (`-s` / `--frame-start` and `-e` / `--frame-end`), an interval to skip frames by (`-j` / `--frame-jump`), a list of separate frames (`-f` / `--render-frame`, which takes a comma list, a `..` range, and both mixed in one argument), and a render engine (`-E` / `--engine`). Alongside them, and not asked about: which scene (`-S`), the output path (`-o`), the image format (`-F`), whether the extension is added (`-x`), the number of threads (`-t`), and Cycles' own two, the render device and the statistics switch. All fifteen of these appear with the same names and short forms in every earlier version's help text on both machines. 5.3's help text, on the Mac, differs from 5.2.2's on the same Mac in one added argument and the version banner and nothing else, and that argument is none of these.

**No argument of its own — in all five versions.** A sample count, a resolution percentage, and whether a view layer renders. Every version searched its manual and found none; the help-text search was then run on each program itself — on both machines for 4.5 to 5.2 and on the Mac for 5.3 — looking for `sampl`, `resolution`, `percent` and `view layer`, and found no such argument in any of the five.

**But "only by changing the file" is too strong, in all five versions.** Blender runs a Python expression given at launch, and the test showed in every version that this settles all three of the settings with no argument: the expression printed `SET 8 25 False` and the render came out 50 by 25 pixels instead of 200 by 100 — in 4.5 to 5.2 on both machines and in 5.3 on the Mac. It works only when the expression comes after the file on the command line. Put before the file, the file load wipes it — the expression still printed its values, and the image still came out at the file's own size. That is the manual's own ordering rule, seen working, identically in all five versions.

**The arguments do not touch the file.** In all five versions — 4.5 to 5.2 on both machines, 5.3 on the Mac: after a render with arguments contradicting everything the file held, the file's size and modification time were unchanged and reading it back gave its own saved values.

**The frame jump applies to a whole-animation render and not to a frame list.** In all five versions: `-j 3` with `-f 1..10` wrote all ten frames; `-j 3` with `-s 1 -e 10 -a` wrote frames 1, 4, 7 and 10. The same five frame runs wrote the same file names into the same folders on both machines in 4.5 to 5.2, and on the Mac in 5.3.

**Whether a view layer renders can differ from frame to frame, and a render started from outside honours it.** In all five versions — 4.5 to 5.2 on both machines, 5.3 on the Mac: a view layer's switch took a keyframe, and rendering two frames wrote a file holding both layers at frame 1 and one layer at frame 2. The Creator confirmed the control side on the Mac in all four earlier versions — the switch has an animation dot beside it and takes a keyframe, and the row below it, "Render Single Layer", is refused one outright with the message `"use_single_layer" property cannot be animated`. In their own words, that is "consistent across all versions". The same refusal came back from the one-card Linux machine in all four. **5.3, on the Mac, did the same in its own window**, with I pressed over each row by simulated input: the "Use for Rendering" checkbox changed colour and its dot became a diamond, and "Render Single Layer" was refused in the status bar in the same words. The right-click menu on "Use for Rendering" was opened in 5.3 alone: it offers Insert Keyframe, Add Driver…, Open Drivers Editor, Add to Keying Set, Remove from Keying Set, Reset to Default Value, Copy to Selected (greyed), Copy Data Path, Copy Full Data Path, Copy as New Driver, Add to Quick Favorites, Assign Shortcut… and Online Manual.

**"Render Single Layer" is ignored from the command line, in all five versions.** Every version's manual says so, and the test watched it happen — on both machines for 4.5 to 5.2 and on the Mac for 5.3: with that setting turned on at launch, both view layers were still written.

**One result has no counterpart before 5.3.** In 5.3, on the Mac, a render with every view layer switched off rendered nothing and wrote nothing, printed one line, `ERROR All render layers are disabled`, and **exited 0**. Anything that judges a run by its exit status would read that as a success. Not tried in 4.5 to 5.2.

**Four notes, none of which changes the answer.**

*The view-layer switch has a different declared default in 4.5.* Real layers start on in all five tested versions. Question 17 holds the full distinction; it does not change the frame-by-frame result here.

*The multilayer file written by the view-layer test is not identical across versions.* Reading the layer names back, 4.5 gave two — `Layer2.Combined` and `ViewLayer.Combined` — while 5.0, 5.1 and 5.2 each gave four, the extra two named `Layer2.CombinedpixelAspectRatio` and `ViewLayer.CombinedpixelAspectRatio`. The same difference came back on both machines. The 4.5 files are smaller as well (5,301 and 4,200 bytes on the Mac, against roughly 8,400 and 4,400 for the other three there). **The extra two names are very likely not layers and not picture channels.** Question 17's runs, all on the Creator's Mac, met the same two names in the same three versions, from a tool that scans a file for readable text: there the channel's own name is `…Combined` and `pixelAspectRatio` is the next field of the file's header sitting hard against it with nothing between, so the tool runs the two into one word. Nothing dumped the file's structure to prove that, and whether the names read back here came from a reading of the same kind is not recorded — so it is the best account of them and not a settled one. **5.3 adds one reading of a different kind**: on the Mac, a tool that reports each part of the file found one part per view layer and, for each, the four channels `….Combined.R`, `G`, `B` and `A` and nothing else, and 5.2.2 given the same test on the same Mac wrote the same parts and the same names. That fits the account above; it does not show what the earlier reading saw. 5.3's files measured 8,363 bytes with both layers and 4,323 with one, on the Mac. **What is left unexplained is the 4.5 file size**, and none of it touches the answer above: the switch was honoured per frame in all five versions.

*The list of launch arguments as a whole grew — and it is not the same list on the two operating systems.* On the Mac the program's own help text runs to 434 lines in 4.5, 466 in 5.0, 498 in 5.1, 518 in 5.2 and 521 in 5.3; the three lines 5.3 adds are one argument, `--debug-gpu-backend-no-fallback`, set against 5.2.2's 518 on the same Mac. Its help says it fails instead of falling back to another GPU backend when the selected one is unsupported, and skips the GPU backend support check. This is **observed help text**, not a test of that fallback. On the one-card Linux machine the same command gives 447, 479, 505 and 525 for the four earlier versions; 5.3's was not read there. So the list grows from version to version, and in 4.5 to 5.2 it is longer on Linux than on macOS in every version. Arguments marked "(Linux only)" and "(Unix only)" appear there, and the ones marked "(Windows only)" are printed on Linux as well. On the Mac, 5.3's help prints eleven lines marked for Windows, for Linux, for both, for Vulkan or for debug builds, and no render argument is among them. The render arguments this question asks about are identical in all five versions.

*Blender 5.2 and 5.3 carry `--gpu-device`, and it is not an argument about where a render runs.* Comparing each version's own help text on the one-card Linux machine, 5.2 holds `--gpu-device` and `--gpu-device-no-fallback` and 4.5, 5.0 and 5.1 hold neither. Its own help says it overrides the graphics device from the user preferences for one run, that it only applies to the Vulkan backend, and that other backends print a warning and ignore it. **It names the card Blender *draws* with, not the card Cycles renders on, and it does not touch `--cycles-device`.** It was run, in all four earlier versions on two machines: in 5.2, `--gpu-device help` prints a numbered list of cards and exits, on a machine with no display server; on a windowless render, where the drawing backend is OpenGL, it prints a warning that it applies to Vulkan only, is ignored, and the render finishes at exit 0; asked for a device that is not there, with a window and Vulkan, it reports the device is unavailable. In 4.5 the argument does not exist and Blender takes it for a file name. That was settled under **The machine 5 and 6**, not here. **5.3's help text carries both arguments**, on the Mac, with the same words as 5.2.2's on the same Mac; 5.3 did not run them. What is still open is whether the Mac's 5.2.0 and 5.2.1 carry the argument — the Mac run searched their help text for five words only, and this is not one of them.

Two things touching this question differ between versions and are set out under question 13: the engine name `-E` accepts, and how 4.5 fails to end when given a name no engine bears.

---

## 2. Can a render be set to stop once the picture is clean enough, and can it be set to stop after a length of time rather than after a count?

**How it is known.** Cycles' adaptive sampling and Time Limit are **documented**, with defaults **observed** in Cycles' files. EEVEE's 5.3 Time Limit is **documented** as added. Whole-frame convergence is **inferred** from the description of individual pixels, then **observed** in tests. The absence of the other stopping controls was **not in the sources**, then checked in the running engines and their panels. Tests cover 4.5–5.2 on the named Mac/Linux machines and 5.3 on the Mac alone.

**Verdict: it differs** — in EEVEE alone.

Cycles has both, in all five versions. **EEVEE has a time limit in 5.3 and in no earlier version**, and a clean-enough stop in none. Workbench has neither, in any version.

**Stopping when the picture is clean enough** is Cycles' adaptive sampling with its noise threshold, which the manual describes as cutting short the sampling in areas that have become less noisy than the threshold. It is on by default in all five versions, with a threshold of 0.01.

**Stopping after a length of time** is Cycles' Time Limit, which "stops rendering if the time exceeds the limit, even if the desired sample count hasn't been reached yet". It is off by default in all five versions, and in all five the manual notes it excludes the work done before rendering starts.

**The time limit is applied to each frame, not to the whole job, and a frame cut short is still written.** In all four earlier versions, on the Mac: a three-frame render with a five-second limit took roughly three times the limit, not one, and all three images were saved. The same shape of test was then run in all four on the Mac and on an idle rented Linux machine — twenty-four runs of a three-frame render under a ten-second limit, three repeats per version on each machine. Every run wrote three image files, each frame took about ten seconds, and every run finished in about thirty-two seconds. Not one stopped the job after ten seconds. So this holds in all four, on macOS and on Linux. **5.3 did the same on the Mac**: under the five-second limit each of three frames stopped at `Sample 1025/16384` after about six and a third seconds and all three were written; under the ten-second limit, three runs of three frames each stopped every frame at `Sample 513/4096` after about ten and a half seconds, every run took about thirty-two seconds, and each wrote three pictures. No line in any 5.3 run names the limit.

**Adaptive sampling ends the whole frame early, not just the sampling of single pixels.** In all four earlier versions on both machines, on a plain, evenly lit, low-detail scene at 128 by 128 — chosen at that size so that the control run with adaptive sampling off finishes in seconds — with adaptive sampling on and the threshold at 0.1, the frame stopped at 48 of 2,048 samples. With it off, the same frame ran all 2,048. The two machines gave the same number, 48, in every version, and 5.3 gave 48 on the Mac. That is the point the manual leaves open, because it describes the decision per pixel and never says what ends the frame. **Observed in all four earlier versions**, from a comment in Cycles' kernel files: an unconverged pixel makes its neighbours keep sampling too, so the stopping is not purely per pixel either. Question 16 carries a further run of this at a different threshold.

**EEVEE's time limit, from 5.3.**

- 5.3's manual documents it on EEVEE's sampling page in the same words as Cycles': it stops rendering if the time exceeds the limit even if the sample count has not been reached, zero switches it off, and pre-render processing is not counted. Its API change log lists it as added between 5.2 and 5.3.
- Listing EEVEE's settings in 5.3 on the Mac finds it — `time_limit`, "Limit the render time. Zero disables the limit." The same listing in 5.2.2 on the same Mac does not, and the earlier versions' listings, on both machines, found nothing of the kind either.
- EEVEE's Sampling panel, photographed in 5.3's window on the Mac, has a Time Limit row under the render samples, reading `0 s`. The same panel in 5.2.2 on the same Mac holds the render samples and nothing else.
- **It works in a render started from outside.** On the Mac, an EEVEE render at 400 by 400 with Render Samples 4,096 and a two-second limit stopped at 275 of 4,096 a little after two seconds, saved its picture, exited 0, and printed nothing naming the limit. The same file with the limit set to 0 was still rendering, at 3,775 of 4,096, when a 25-second ceiling stopped it.

**Neither EEVEE nor Workbench has a clean-enough stop, in any version, and Workbench has no time limit either.** Listing every setting on both engines' blocks and searching for time, threshold, adaptive, noise, stop and limit found nothing in any of the four earlier versions, on both machines, and in 5.3, on the Mac, found the EEVEE time limit and nothing else. EEVEE's whole settings block was listed again in every earlier version on question 16's two machines and again held nothing resembling a time limit, a noise threshold or a minimum count. The nearest EEVEE has to a threshold is a Light Threshold, which decides which lights contribute. The Creator then photographed both engines' Sampling panels on the Mac in all four earlier versions and they are identical: EEVEE shows viewport samples, temporal reprojection, jittered shadows, render samples, shadow rays, steps, volume shadows, resolution and light threshold; Workbench shows two rows, a render count and a viewport method. Workbench's two rows were photographed again on the one-card Linux machine in all four and match. 5.3's photographs on the Mac show the same EEVEE panel with the Time Limit row added, and the same two Workbench rows, both of them menus. No control in Workbench, or in EEVEE before 5.3, would end a render early.

**What Blender's window shows when the limit ends a render — seen in 5.3 alone.** A Cycles render started from 5.3's window on the Mac under a three-second limit showed, while it ran, the frame, the elapsed time, the time remaining, memory and the sample count, with a progress bar in the status bar. After the limit ended it, the line showed the frame, the time and memory and no sample count, and the status bar was empty. Neither said a word about the limit. Not looked at in 4.5 to 5.2.

**A note, marked as not established.** How far past the five-second limit each version ran varied on the Mac: roughly 5.2 to 5.4 seconds per frame in 5.0 and 5.1, roughly 6.6 to 7.1 seconds in 4.5 and 5.2, and about 6.3 seconds in 5.3 on a busy machine. That is one timing run of one scene per version, and it is not enough to say the versions overshoot differently. What is established is the same in all five: the limit is per frame and the frame is still written.

**Not established in any version: whether a Cycles frame the limit cuts short is still denoised.** No version's sources say. In 5.3, on the Mac, a frame cut short with denoising on spent about a quarter of a second more between its last count and `Finished` than the same frame with denoising off, and nothing printed says what was done in that time.

The size of EEVEE's settings block in each version is under question 3; the one setting 5.3 has that 5.2.2 has not is the time limit above.

---

## 3. Which of a render's settings belong to a particular render engine rather than to every engine?

**How it is known.** The engine-specific groups and shared settings are **documented**; Cycles' registration is **observed** in its own files. Their coexistence and the link between Workbench and the display block are **inferred**, then **observed** in tests: 4.5–5.2 on the Mac and one-card Linux machine, 5.3 on the Mac.

**Verdict: the same everywhere.**

Blender keeps a separate block of settings for each engine, hanging off the scene, plus one shared block, in all five versions. Cycles' block is added by the Cycles add-on when it registers; EEVEE's and Workbench's are built in. The shared block holds the things that belong to no engine — the resolution and its percentage, the output path and format, the frame rate, the thread count, the border, the stamps, the compositing and sequencer switches, and the engine setting itself. The frame range and step sit on the scene beside it.

Every version's tests confirmed the same thing — on both machines in 4.5 to 5.2, on the Mac in 5.3: with a Blender started from a terminal and no window, all the blocks were reachable and each held its own saved values, whichever engine was selected. Workbench's block holds nine settings in every version.

**EEVEE's own settings block is not the same size in each version:** 55 settings in 4.5, 52 in 5.0, 54 in 5.1, 53 in 5.2 and 54 in 5.3, counted by the same test in each earlier version on both machines, which gave the same four numbers, and in 5.3 on the Mac. Which ones differ among the earlier four is recorded nowhere. **The one 5.3 has that 5.2 has not is EEVEE's time limit** — the same listing in 5.2.2 on the same Mac counts 53 and finds no time limit. It changes nothing about the rule this question asks about. It does change question 2 for 5.3: in 4.5 to 5.2 none of EEVEE's settings stops a render early, and in 5.3 the time limit does. The panel a person sees is identical in the four earlier versions; 5.3's adds that one row.

**One distinction that holds across the tested versions:** a setting can be drawn inside an engine's own panel and still be a shared one. The thread count and the thread mode are documented on Cycles' Performance page but belong to the shared block, not to Cycles. So does the compositor's device, which appears under whichever engine is chosen — the Creator saw it under Workbench in all four versions on the Mac, and the tab was photographed again in all four on the one-card Linux machine. In 5.3, texture cache and persistent data are further shared settings drawn in Cycles' own panels, and 5.3's Workbench tab, photographed on the Mac, shows the compositor's device as well.

**One difference found beside this answer, and it is under question 13:** the rows under Cycles' engine dropdown are not the same in 4.5 as in the other four. It does not change the rule this question asks about, which is identical in all five.



---

## 4. Does a file carry a separate sample count for each render engine?

**How it is known.** EEVEE's and Workbench's counts are **documented**; Cycles' is **observed** in its own files. The link from the display block to Workbench is **inferred**. Saving all counts together without disturbing them by an engine switch is **inferred** from their storage, then **observed** in every version: 4.5–5.2 on the Mac and one-card Linux machine, 5.3 on the Mac.

**Verdict: the same everywhere.**

Yes. Three separate counts, in three separate places on the same scene, in all five versions:

- **Cycles** — a number of samples per pixel, 4,096 by default.
- **EEVEE** — a number of samples per pixel for rendering, 64 by default.
- **Workbench** — not a number but a choice of anti-aliasing method whose options are sample counts, `8` by default, chosen from a fixed menu of off, single-pass, 5, 8, 11, 16 and 32.

Each engine also keeps a separate viewport count of its own, which is not what this question asks about.

**One file holds all three at once, they survive a save and a reload, and switching the engine disturbs none of them.** In all four earlier versions on both machines, and in 5.3 on the Mac: a file saved with Cycles at 123, EEVEE at 45 and Workbench at 32 came back with exactly those values, and reading them again after switching the engine three times gave the same three numbers every time. The only thing that read differently between versions was the engine's own name, which is question 13's business.

**One anti-aliasing count is not per engine, in all five versions.** The Grease Pencil final-render count is a single scene-wide value that every earlier engine's manual page documents in identical words. It was set to 3 and read back as 3 under all three engines, in all four earlier versions on both machines and in 5.3 on the Mac.

**The three controls have the shapes the sources describe**, seen by the Creator on the Mac in all four earlier versions and photographed in 5.3's window on the Mac: Cycles' is a plain number field beside its noise threshold, minimum samples and time limit; EEVEE's is a number field with a separate viewport one above it — and in 5.3 a time limit beside it, which is question 2's; Workbench's is a menu of fixed choices.

**One difference beside this answer, and it is under question 16.** A view layer can carry a sample count of its own that overrides the scene's. That override exists in all five versions and is one value shared between engines. What changed is who obeys it: from 5.0 onwards EEVEE honours view layer overrides and 4.5 does not, which 5.0's release notes state and which question 16 settled by test, 5.3 included. Cycles' own rule for what to do with an override — use it, bound it by the scene count, or ignore it — is in all five versions.

---

## 5. Can a sample count change from frame to frame?

**How it is known.** Keyframing sample counts is **inferred** at best from the general animation rules. EEVEE's count in 5.3 is also labelled **not in the sources**; Workbench and the view-layer count have no direct statement. The runtime answer is **observed**, from terminal and Mac window tests in all five versions, with Linux terminal tests for 4.5–5.2. No inference is presented as a documented sample-count rule.

**Verdict: the same everywhere.**

Yes for Cycles and EEVEE; no for Workbench; and a render started from outside reads the new value at each frame. Identical in all five versions — 4.5 to 5.2 on both machines, 5.3 on the Mac.

**Which counts take a keyframe.** In all five versions, asking for a keyframe on Cycles' count and on EEVEE's count both succeeded, and asking for one on Workbench's anti-aliasing choice was refused with the message `property "render_aa" not animatable`. In 5.3 a view layer's own count took one as well.

**A render honours it frame by frame.** In all five versions, a Cycles file with its maximum samples keyed to 4 at frame 1 and 512 at frame 2, rendered as an animation from a terminal, reported 4 samples for the first frame and 512 for the second. The count is read again at each frame, not once when the job starts — which is exactly the point every version's sources could not settle. **In 5.3, on the Mac, the same held for EEVEE's count**, keyed 4 then 64, **and for a view layer's own count**, keyed 5 then 100 under Cycles; neither was rendered keyed in 4.5 to 5.2.

**The panel has no dot to click, in the four earlier versions, and the menu does the job instead.** The Creator photographed Cycles' Sampling panel on the Mac in all four: no animation dot beside the noise threshold, the maximum samples, the minimum samples or the time limit. Right-clicking the maximum samples opens the same menu in all four, offering Insert Keyframe with `I` as its shortcut, Add Driver, and the drivers editor. The only difference between the four menus is that 5.1 and 5.2 write the driver entry with trailing dots and 4.5 and 5.0 do not. 5.3's menu, photographed in its window on the Mac, matches 5.2's entry for entry, trailing dots included. That panel was not photographed on the one-card Linux machine.

**How a keyed count is drawn — seen in 5.3 alone.** In 5.3's window on the Mac, with Max Samples keyed to 16 at frame 1 and 64 at frame 20, the field read 16 and was filled one colour at frame 1, read 39 and was filled another colour at frame 10, and read 64 in the first colour at frame 20. So between keys the count takes a value neither key holds. No render was made at a frame between keys.

---

## 6. Does every render engine render on a computer with no screen attached?

**How it is known.** The general command-line statement and EEVEE's Windows limitation are **documented**. The screenless answer for each engine is otherwise **not in the sources**. **Observed** tests settle 4.5–5.2 on the two-card Linux machine with no display server. The Mac, including every 5.3 test, settles only rendering with no Blender window on a machine that has a screen.

**Verdict: not established for every version.**

For 4.5 to 5.2: yes, on Linux, on a computer with no display server — every render engine each version has rendered there, on the graphics cards and on the processor, EEVEE included. All four did the same thing. **For 5.3 the screenless case was never tried**: the one machine it ran on has a screen, and there, with no window open, all three engines rendered. That is a gap in what 5.3 was run on, not a difference between versions. Three further parts of the question are still unanswered on every machine this topic has had, and all three are the same in all five versions.

**What the two-card machine settles, identically in all four earlier versions.** With no display server on the machine — the display variables empty, the session a plain terminal, no display process running, and the graphics cards present only as devices of the kernel — every engine each version has rendered a 64 by 64 image and exited 0, whether the render was aimed at the cards or at the processor, with no error and nothing printed about a display. Asking Cycles to report where it worked showed the card runs really were on the cards and the processor runs really were on the processor. Blender 4.5 has three engines there and rendered on all three; 5.0, 5.1 and 5.2 have three each and rendered on all three. So on Linux this is settled for those four — for a machine with **no display server**, which is the tested claim and the one used throughout below.

**What the four earlier versions also establish, identically, and where 5.3 stands beside them:**

- Their manuals say a render driven from the command line needs no graphical display, giving Linux without an X server as the example, and that a render can therefore be driven over a remote shell. 5.3's says the same.
- Their manuals say, of EEVEE alone, that headless rendering "is not supported on headless Windows systems", and that EEVEE only renders on the graphics card with no plan ever to render on the processor. 5.3's says the same two things. **Nothing in this project has ever established which device an EEVEE render used, on that machine or any other.** What is recorded is that on the RTX 3060 machine, whose display server is drawn by the processor, EEVEE frames took about ninety seconds each — `Frames 15` — and that six of this project's machines with a display had that display drawn in software. Whether such a machine counts as having a graphics card for EEVEE's purposes is a question nobody has asked — **The machine 2**. **There is a second record, and it is not the same shape.** On a machine with no display server at all, with the card's graphics library hidden on purpose and nothing else changed, every version answered that it was drawing with a software renderer and the render still finished, wrote its frame and exited 0, on the one timing recorded taking longer than the same render with the card's library — **The machine 7**. **Neither record establishes what EEVEE's own sampling ran on**, so neither settles the contradiction; the second one adds that it is the graphics library Blender is handed that moves this, and not whether the machine has a screen.
- Their manuals say nothing at all about Workbench and a screenless machine, and nothing about EEVEE on a screenless Mac or Linux machine. 5.3's sources are silent on Cycles and Workbench as well.
- No earlier version's manual defines what "headless" means — whether it is a machine with no monitor plugged in, no logged-in session, or no graphics card at all.
- With no window open, on macOS, all three engines rendered and wrote a 64 by 64 image with no error, in all four versions — **and in 5.3**, on the Mac, with no error or warning line and an exit status of 0 for each.
- With no window open, on the one-card Linux machine, the same three renders did the same thing in all four versions.
- **With the display taken out of the environment entirely on the one-card Linux machine**, so that Blender could reach no display at all, all three engines — Cycles, EEVEE and Workbench included — still rendered and wrote a 64 by 64 image and exited 0, in all four versions, with no error, no warning and nothing printed about a display.
- Asked for a window rather than a render with no display to open it on, Blender says so plainly and stops. All four name the same two back-ends and say neither could be reached, but they say it in their own version's shape: 4.5 prints the plain `GHOST: failed to initialize display for back-end(s): ['WAYLAND', 'X11']` and then `GHOST: unable to initialize, exiting!`, while 5.0, 5.1 and 5.2 print the same two lines as log lines under a `ghost.system` channel, the second marked `ERROR`. Blender 5.0, 5.1 and 5.2 then exited 1; Blender 4.5 never ended at all — one more case of its kind, which is under question 13.
- **Being cut off from a display and having no display server look the same from outside.** On the two-card machine the refusal above came back word for word as the one-card machine had given it when cut off from its display, with the same exit status in each version. One further thing showed there, and it is the versions differing rather than the machines, since it was one test on one machine in all four: 4.5 wrote its refusal to the error stream and the other three wrote theirs to the ordinary output stream. Blender 5.0 alone also repeated a warning about the memory checker fourteen times on the error stream.

None of the Linux results in that list has a 5.3 counterpart: 5.3 ran on no Linux machine.

**The unfreed-memory line is not a version marker, and must not be used as one.** In that same refused-window test on the machine with no display server, 5.0, 5.1 and 5.2 each added a line reporting memory left unfreed and 4.5 added none. That holds inside that test and nowhere else: 4.5 prints an unfreed-memory line elsewhere — question 10 below is an instance, in this topic's own runs on both machines — and the newer versions sometimes do not, while 5.3 printed one beside a refused Cycles device name, on the Mac (question 9). **Nothing reading Blender's output may treat the line's presence or absence as telling it which version is running**, and nothing should treat it as a failed render.

**An unassigned or hidden camera — tried in 5.3 alone, on the Mac.** The factory scene, a cube, camera and light, was saved on Workbench at 64 by 64. In one file the scene's camera was set to `None`, but the object `Camera` remained in the scene; a fresh Blender read the assignment back as `None`. In the other, the camera stayed assigned and its `hide_render` and `hide_viewport` were both set to `True`. Each was rendered with no window, factory settings, a throwaway settings folder, and `-F PNG -f 1`. Each exited 0, printed no camera complaint and wrote a PNG identical, channel for channel, to the ordinary Workbench render of the same factory scene: zero differing channels out of 16,384. **Observed for these two Workbench files only.** No run removed every camera object, tried another engine in this state, or tried it in 4.5 to 5.2.

**What is still not established.** For 5.3, the screenless case itself, as above. And three things in all five versions, none of them a difference between versions: a headless Windows machine, a computer with no graphics hardware at all, and a Mac with no display server. All four are in *What is not established* at the top. The computer with no graphics hardware is untried here rather than out of reach: a machine with no graphics card of any kind is on record — the Windows Server 2025 machine under **Frames 15**, which rendered on it — though it was driven over a remote-desktop screen and nothing records what drew it.

**One difference beside this answer, in *Where Blender itself differs between versions* at the top.** Blender 5.2's release notes add `gpu.init()`, a way for a script to start the graphics side of Blender when it is running with no window, and 5.3's API documentation offers it for the same use. No render needed it, on any machine: the tests rendered on every engine in 5.2 without calling it, including on the machine with no display server, and on every engine in 5.3 on the Mac without calling it.

---

## 7. Does a scene set up for one render engine come out the same in another?

**How it is known.** **Documented**, from the manuals and, for the 5.3 camera change, the release notes. No test compared an engine's look with another in any version.

**Verdict: the same everywhere.**

No. All five versions say the same thing in their own words: cameras, lights and materials are shared between EEVEE and Cycles, but some features work in only one of them, and EEVEE approximates where Cycles computes. All four earlier versions quote the same kinds of concrete limits — only two camera projections supported, lights limited to one colour with no node trees, spot size not softening the cone, only single scattering in volumes, a limited number of geometry-node attributes reaching a material. All five say Workbench is further away again: it does not use shader nodes at all, and the four earlier versions add that it is not meant to produce final images.

**The engine differences run both ways.** EEVEE-only nodes that do not work in Cycles are documented in 4.5 and 5.3; utility nodes incompatible with EEVEE are documented in 5.1; and a Cycles-only node is documented in 5.2. Uneven examples do not establish version changes. Engine settings are separate as question 3 describes. In 5.0 a material's output node is also documented as being aimable at one engine, allowing an intentional difference in the picture; whether that wording appears in another version is not established.

**One of the concrete limits moves in 5.3, by its release notes.** EEVEE can now render panoramic cameras. 5.3's EEVEE limitations page still says only perspective and orthographic projections are supported, so the documented claims disagree; the release note states a change, and that is what the first list carries. **Documented, untested.** Which panoramic projections this adds is not established here. This does not change the general answer: 5.3 still says materials may look different between engines and some nodes work in only one.

---

## 8. Can one scene be set up to take in another scene's rendered result, so that rendering the first renders the second?

**How it is known.** Both mechanisms are **documented**. A command-line render using the sequencer, and the fresh render's own settings, require **inference** from that description. Their behaviour is then **observed** in 4.5–5.2 on the Mac and one-card Linux machine and in 5.3 on the Mac.

**Verdict: the same everywhere.**

Yes, two ways, in all five versions.

**Through the compositor**, with a Render Layers node whose scene is set to the other scene. **Through the video sequencer**, with a Scene strip referencing the other scene. Both are ordinary contents of a file, and both work in a render started from a terminal with no window.

Four things came back identically in all four earlier versions, on both machines, and in 5.3 on the Mac:

- **Rendering the first scene does start a fresh render of the second.** The terminal named the second scene while the first was rendering.
- **The second scene's own engine and its own sample count are used**, not the host scene's: a Workbench scene pulling in a Cycles scene printed Cycles' progress and its eight samples.
- **Through the compositor, the second scene's own compositor does not run.** Comparing three images built from the same file, the first scene's output matched the second scene's raw render and not its composited one. Every version's manual hints at this by telling you to render to files first if you want the other scene's composited output; the test is what settles it. **Through a Scene strip it does** — the strip carries the other scene's composited picture, and that scene's File Output node writes, once per frame of the strip, governed by that scene's own Compositing switch (**The compositor 4**, **File output nodes 16**; on record for 4.5 to 5.2).
- **Two scenes pointed at each other do not hang.** With each scene's node aimed at the other, the render finished well inside a sixty-second cut-off — in 5.3 in under a second — exited cleanly, wrote an image, and rendered the second scene once. No error, and no sign of the chain repeating.

For the sequencer route, a Scene strip referencing another scene was rendered from the command line in all four earlier versions on both machines and in 5.3 on the Mac, and the referenced scene rendered with its own engine.

**What the sequencer does to the host scene's own render — tried in 5.3 alone.** On the Mac, a scene whose sequencer holds strips, with the sequencer switched on, wrote the sequencer's result and rendered none of its own view layers, and its compositor did not reach the picture even with compositing switched on. With the sequencer switched on and holding no strips, it rendered its 3D scene as an ordinary render. Not tried in 4.5 to 5.2.

**How a scene holds its compositor changes at 5.0.** Its compositor is reached one way in 4.5 and another from 5.0, including 5.3. The tests used each version's own route; the rendering result agrees.

**Three differences beside this answer, all in *Where Blender itself differs between versions* at the top.** The Sequencer Scene exists from 5.0 and not in 4.5 — and where it exists, a bare command-line render ignores it and renders the active scene, on both machines for 5.0 to 5.2 and on the Mac for 5.3; the full account of that is under question 12. A Scene strip's view layer selector is new in 5.2: Blender 5.2's release notes say scene strips now have one, set at first to the default view layer, and that release note is what carries the difference. Whether 5.3 carries the selector is not established. And 5.3's release notes say the compositor now renders scenes referenced inside node groups. That rests on the release note alone: no test in any version built such a node group.

**What no version settles**, and this is in *What is not established* at the top: how frame numbers line up between the two scenes, and whether a chain may go deeper than two. The test scenes were still in every version, so no frame mapping could be seen.

---

## 9. Does a file record whether it renders on the computer's graphics cards or on its processor?

**How it is known.** Cycles' setting is **observed** in its own files and its purpose is **documented**. Saving the choice per scene and applying the override are **inferred**, then **observed** by test. Workbench's lack of a choice was **not in the sources** for the earlier versions; in 5.3 it is labelled **observed** from API pages as well as left open. Runtime property lists and panel checks settle it on the tested machines: 4.5–5.2 on the Mac and one-card Linux machine, 5.3 on the Mac.

**Verdict: it differs** — in three details. Everything the question turns on is identical; what differs is a starting value, a set of controls, and one printed line. 5.3 sides with 5.2 on the first two, on the Mac, and was not tried on the third.

**What is the same in all five versions** — on both machines for 4.5 to 5.2 unless a line says otherwise, and on the Mac for 5.3:

- **Cycles records a two-way choice per scene: the processor, or "GPU Compute".** Nothing finer. Which technology and which cards are used is a setting of the machine, kept in the user preferences, not in the file.
- **The choice starts at the processor**, survives a save and a reload, and is held one scene at a time. A file saved with one scene set to the graphics card and another left alone came back exactly that way in all five versions.
- **A launch argument beats both the file and the machine's preferences.** Cycles' own help text says so, and the test watched it happen in all five versions — in 5.3, `METAL` beat a file saying `CPU`. The argument only works after a bare double dash. A device name that names no device stops the render with an error, exits 1 and writes nothing, in every version. **In 5.3 `GPU`, the file's own word for the card, is one such name**: refused as not a valid Cycles device, exit 1, no picture. Not tried in 4.5 to 5.2.
- **Without the double dash, the argument is taken as a file name — and the frame may already be done by then.** On the one-card Linux machine in 4.5 to 5.2, Blender rendered and saved the frame it was asked for, then reached the stray argument, printed `unknown argument, loading as file: --cycles-device` and `Cannot read file "…/--cycles-device": No such file or directory`, and ended at exit code 1. So an image existed and the run still reported failure, and the device name itself was ignored — the render went to the processor, as the file said. Seen in all four earlier versions on the one-card Linux machine, and in 5.3 on the Mac in the same shape.
- **A file asking for the graphics card when the machine has none to give renders on the processor, writes the image, and exits 0.** Two different ways of having none were tried in 4.5 to 5.2, one on each machine, and both behaved the same. On the Mac, with no graphics device enabled at all. On the one-card Linux machine, with the preferences set to a technology no card there can serve — `NONE`, `HIP` or `ONEAPI`. In every case the image was written and the run exited 0. 5.3, on the Mac, did the same with the preferences set to None and with them left at factory settings, which hold Metal with no device switched on, and printed nothing; only with Metal and its devices switched on did its statistics carry the line that marks a render off the processor. Anything that needs to know whether a render actually used the graphics card cannot learn it from the exit status or a warning, on any machine. The one thing Blender says about it differs by version and is below.
- **What technologies a machine offers is a fact about the computer, not about Blender or its version.** The Creator's Mac offers Metal and the processor, in every version, 5.3 included. The one-card rented Linux machine offers `NONE`, `CUDA`, `OPTIX`, `HIP` and `ONEAPI`, and **no Metal at all** — asking for Metal there is refused outright, in the same words in all four earlier versions. This is not a version difference and must not be read as one.
- **A technology named with no card behind it is refused in exactly the same words, however impossible it is.** On the one-card Linux machine, `--cycles-device HIP`, `ONEAPI` and `METAL` all stopped the render with `Found no Cycles device of the specified type`, exit code 1, no image — the same refusal for a technology this build supports but the machine cannot serve as for one this machine cannot have at all. Identical in all four earlier versions. 5.3, on the Mac, refused `CUDA`, `OPTIX`, `HIP` and `ONEAPI` in the same words, exit 1, no image.
- **On a machine with an NVIDIA card, `CUDA` and `OPTIX` both render on the card.** Asked for at launch on the one-card machine, both exited 0 and wrote the image, and Cycles' own statistics carried the line it prints only when the render is not on the processor. Chosen in the preferences with the file asking for GPU Compute, both again rendered on the card. Identical in all four earlier versions; **untried in 5.3**, which had no such card. `CPU` rendered on the processor, as asked. On the two-card machine, asking for `OPTIX` at launch put the work on graphics cards and asking for `CPU` put it on the processor, in all four earlier versions, and Cycles named which device it used in each case.
- **EEVEE has nothing to record**, because it only renders on the graphics card in the first place. **Workbench records nothing either**: listing every setting on its block found nine, none of them a device choice, in all five versions, and photographs of the whole Workbench Render Properties tab — the Creator's on the Mac and the agent's on the one-card Linux machine for 4.5 to 5.2, and 5.3's on the Mac — found no device control for Workbench anywhere in it.
- **Two device choices in the file belong to no engine**: the compositor's, and the denoise nodes'. They appear under whichever engine is chosen. **They are two different settings and only one of them differs between versions** — the compositor's own device is the one below; the denoise nodes' device starts at "automatic" in all five versions, and question 19 carries it with the rest of the denoising settings.

**What differs, version by version:**

- **The compositor's device on a brand-new scene.** In 4.5, 5.0 and 5.1 a new scene starts at the processor. In 5.2 and 5.3 it starts at the graphics card. Read back in all four earlier versions on both machines and matched by the Creator's photograph of the panel in each; read back in 5.3 on the Mac and matched by its photograph of Workbench's tab. Cycles' own device setting, above, starts at the processor in all five.
- **The Workbench Render Properties tab holds different controls.** Blender 4.5's has no Curves panel; 5.0's, 5.1's, 5.2's and 5.3's do — photographed in all four earlier versions on both machines and in 5.3 on the Mac. Three further points rest on the Mac's photographs alone: Blender 5.2's and 5.3's Performance panels add an Anisotropic Filtering row and a Precision row beside the compositor's device, which the other three do not show, and 5.2 and 5.3 name the colour control Display Emulation, reading Automatic, where 4.5, 5.0 and 5.1 name it Display Device. (Of the older three only 4.5's shot records what that control reads, and it is sRGB.) 5.3's tab matches 5.2's in every control that names a device.
- **What Blender prints when Cycles meets HIP with no AMD card.** On the one-card Linux machine, at the ordinary logging level, 5.0, 5.1 and 5.2 print one warning line, `WARNING HIPEW initialization failed: Error opening HIP dynamic library`, and 4.5 prints nothing. But 4.5 does print the same message at debug level, so **what differs between the versions is how loudly it is said, not whether**. The line appears whenever something makes Cycles look at its device list — writing the system-information file, a script that sets the device, a plain refresh — and not only when HIP is asked for; it does not appear in an ordinary render at factory settings in any version. Nothing at all is printed for `ONEAPI` or `METAL` in any version. This is one machine's answer in all four earlier versions, so it is the versions differing and not the machines. **Not tried in 5.3**, which ran on no Linux machine; on the Mac, which offers no HIP, 5.3 refused `HIP` with the same one error line as `CUDA`, which says nothing about this line.

**A difference in documentation only.** In 4.5 the Cycles render-settings index has no Device entry; the control is documented on the GPU Rendering page. In 5.1, 5.2 and 5.3 the index itself describes CPU and GPU Compute. Whether 5.0's index has that entry is not established. The saved setting and tested launch override agree across versions, so this page difference calls for no different handling of a render.

**Two notes, marked as not established.**

Cycles' device setting is quoted with an extra instruction attached in 5.0, 5.1, 5.2 and 5.3 — a call that runs when the value changes — which 4.5's quotation of the same setting does not carry. What that changes for a render, if anything, is recorded nowhere and no test touched it on any machine.

**What a machine with more than one card does with a named technology is not settled by anything in this topic.** On the two-card machine, asking for `OPTIX` gave a report naming both of its cards, in all four earlier versions. That is not enough on its own to say a named technology takes every card of its kind, because which cards the preferences had switched on was never recorded here, and nothing in the report says how the work was divided. It has been settled elsewhere in this research, by varying exactly that — **under The machine 5** — and the answer is that a named technology takes every card of its kind whatever the preferences said. It is a fact about a machine in any case, not about a Blender version: all four reported the same thing.

**What no version settles**, and this is in *What is not established* at the top: what a file's "GPU Compute" means on an AMD or an Intel graphics card. No machine the project has had carried one — Apple's own graphics on the Mac, NVIDIA cards on every rented machine — so what `HIP` and `ONEAPI` do when the card really is present is untried in every version.

---

## 10. What does Blender state about opening a file that was saved by a different version of Blender?

**How it is known.** The compatibility statement and release-note changes are **documented**. The warning is described for a window; its appearance in a terminal is **observed** by tests on both named machines in 4.5–5.2 and on the Mac between 5.2.2 and 5.3. Cycles' conversion code is **observed** as code; its runtime effect is **inferred** unless a test below says otherwise.

**Verdict: it differs** — not in the general statement, which is identical, but in the specific breaks each version records.

**What is the same in all five versions.** Every version's manual carries the same page, word for word where they quote it. It says Blender opens files from both older and newer versions; that opening older files is usually straightforward and that this will hold for at least a full release cycle; that opening a newer file should be expected to lose things, that a warning is shown in the interface, and that trying to overwrite such a file raises a confirmation; that a new major version can make a file completely unreadable by older Blenders, which then fail with a message naming the minimum version needed; that the last long-term release of the previous cycle is kept able to convert between the two; and that for a wide gap it helps to step through intermediate releases.

**And the tested behaviour is the same too.** A file saved by 5.2, opened with no window by 4.5, 5.0 and 5.1, printed `File written by newer Blender binary (502.44), expect loss of data!` in all three and then rendered and saved the image, exiting cleanly. Going the other way, 5.2 opened a file saved by 4.5 with no warning at all. The two machines agree on every part of that. Between 5.2 and 5.3, on the Mac: 5.2.2 given a file saved by 5.3 printed `WARNING File written by newer Blender binary (503.29), expect loss of data!` and rendered it, exit 0; 5.3 given a file saved by 5.2.2 printed nothing about versions and rendered it. So the warning the manual describes as shown "in the interface" does reach a terminal, and none of the older Blenders refused the file. The Creator confirmed the inside case in 4.5 on the Mac: the same sentence appears as a report box over the viewport and again in the status bar, nothing blocks the file from opening, and nothing has to be dismissed. **What 5.3 does with a file newer than itself is untried**: no newer Blender exists to save one.

**What differs, version by version.** Each version's own release notes record its own breaks, and these are statements about opening files across versions, so they are answers to this question:

- **Blender 4.5** says a file saved on a big-endian machine triggers a warning when opened, and that big-endian files are deprecated and will be removed in 5.0.
- **Blender 5.0** says big-endian support has been removed. It also says the file format itself changed — the file's header and the header of every block inside it — so that outside code reading these files has to be updated. It raised the longest data-block name to 255 bytes and says names will be cut short when such a file is opened in 4.5, and that linking data-blocks with longer names is not supported in 4.5 at all. It says data packed into a 5.0 file will open in 4.5 as ordinary local data. It also names two render-side breaks: a material property whose meaning had been inverted since 4.2 is fixed, so files saved from 5.0 will not bake the same lighting in older Blenders, and view layer overrides are newly supported so older files using them will render differently.
- **Blender 5.1** says node tools now need globally unique names, set automatically by opening and saving in 5.1, and that Grease Pencil fills were rebuilt with automatic conversion that some setups may need adjusting after.
- **Blender 5.2** says older Cycles files are converted automatically to a legacy Random Walk mode so that they keep matching, and that asset files holding Geometry Nodes tools must be re-saved to work properly.
- **Blender 5.3** says scenes using a Sky Texture with Sun Disc enabled will look different from earlier versions, because the sun disc now contributes to the lighting, and that switching Sun Disc off restores the earlier look. It says Spline IK chains are capped at 255 bones when a file is read, so that 5.3 can read a file from a later version that raises the limit without crashing. And it records initial support for leaving a file unchanged when it is opened and saved. Separately, 5.3's Cycles carries conversion code that rewrites some Cycles settings in files saved by 5.0.77 or older and passes over files saved by anything newer than 5.2.8 — read in Cycles' own files and not run, since no file old enough was at hand.

**A note on something observed in three versions and not the others.** When they opened the 5.2 file, 5.0 and 5.1 printed only the compatibility warning, and 5.2.2 given the 5.3 file on the Mac printed the warning line alone as well. Blender 4.5 printed the warning and then several lines of the form `Warning: region type 11 missing in space type "Action" (id: 12) - removing region`, at least one reading `regiondata free error`, and at the end `Error: Not freed memory blocks: 4`. It still rendered and exited cleanly. The Mac and the one-card Linux machine agree that 4.5 alone does this; the two-card machine never ran this test. The two runs on the one-card machine do not agree on how many such lines there are — one gave fourteen lines in all and the other thirteen `region type` warnings and two `regiondata free error` lines, so the count is unsettled, and the machine cannot be reached again. 5.3 has no newer file to open, so its behaviour on that case is not established. That gap does not undo the tested difference among the earlier versions. Anything that reads Blender's output for errors should know that 4.5 says a good deal more than the others when it opens a newer file, and that none of it stopped the render.

**One thing the Linux run printed that the Mac's record does not carry.** On Linux, 4.5 opening the 5.2 file also printed `Error: Engine 'BLENDER_EEVEE' not available for scene 'Scene' (an add-on may need to be installed or enabled)` — because the 5.2 file names the engine identifier 5.0 introduced, which 4.5 does not have (question 13). Blender 4.5 reported it, fell back to its own EEVEE and rendered anyway. Whether the Mac's run printed the same is not recorded, so nothing is claimed either way; this is a gap in one machine's record, not a difference between the machines.

---

## 11. Can a file rely on scripts stored inside it to render correctly, and what decides whether those scripts run when a render is started from outside Blender?

**How it is known.** The script-running controls are **documented**. The preference description and the disable argument's “default” wording leave a tension about starts with neither flag; the tests below settle it. Simple-expression exceptions are **inferred**, then **observed**. Tests cover 4.5–5.2 on the Mac and one-card Linux machine, and 5.3 on the Mac.

**Verdict: the same everywhere.**

Yes, a file can carry and depend on scripts, and what decides whether they run is the same in all five versions.

**What a file can carry**, in all five versions: text blocks marked to load on start, and driver expressions that compute values. Every version's manual says automatic execution is off by default and that some files need it to work properly.

**What decides**, in all five versions: a preference called Auto Run Python Scripts, which can be overridden per file when opening through the file browser, and which two launch arguments override — one to enable, one to disable. The file-browser route has no equivalent from outside; the preference is what applies there.

**With neither flag, the preference decides.** The test was run in all four earlier versions against two throwaway settings folders, on both machines, and in 5.3 on the Mac, and gave exactly the same six results every time: with the preference off, no flag blocked the script and the enable flag ran it; with the preference on, no flag ran it and the disable flag blocked it. So the "(default)" the arguments page prints beside the disable flag describes the factory state of that preference, not an unconditional rule. That settles the tension between the same two documented statements in all five versions.

**What a blocked script costs, in all five versions.** The render does not fail and the image is still written. The driver supplies 0 instead of its value, so the picture comes out wrong. And it is not silent: the line `BPY_driver_exec: restricted access disallows name '__import__', enable auto-execution to support` goes to the terminal, which is where the interface would have shown a dialog. A run can also find out for itself that this happened, without reading the terminal: `bpy.app.autoexec_fail` and `bpy.app.autoexec_fail_message` report it, in all four earlier versions on both machines — **The machine 7** — and in 5.3 on the Mac. Not every expression needs the permission — simple ones ran whatever the flag, in all five versions.

**One line the Linux runs printed that the Mac's record for 4.5 to 5.2 does not carry.** On the one-card Linux machine every blocked run printed a second line as well, `Error in PyDriver: expression failed: …`, in all four earlier versions. The Mac's record for those four holds only the first line. **5.3, on the Mac, printed both.** Whether the Mac printed the second in 4.5 to 5.2 is not recorded, so nothing is claimed either way — this is a gap in one machine's record rather than an established difference between the machines, and it is certainly not a difference between versions.

**Three more kinds of script — tried in 5.3 alone, on the Mac**, with the preference off:

- **A registered text block** ran under the enable flag only. With no flag and with the disable flag it was skipped, and Blender printed one line naming it: `scripts disabled for "…", skipping 'marker.py'`.
- **A text block named on the launch line with `--python-text`** ran under all three — no flag, the enable flag and the disable flag. Running a text block on purpose is not held back by the setting.
- **An Open Shading Language script stored in the file**, used by a material's script node, ran under all three as well: the cube came out the colour the script gives it, and with Open Shading Language switched off it did not. The setting that governs Python scripts does not govern these.

None of the three was tried in 4.5 to 5.2.

**One difference beside this answer, in *Where Blender itself differs between versions* at the top.** Blender 5.1, 5.2 and 5.3 carry two Python-environment launch arguments where 4.5 and 5.0 carry one. In 5.1, 5.2 and 5.3, one allows Python to use system environment variables and says it also enables the user environment; the other allows Python to use the user's own installed packages and says it disables full isolation. In 4.5 and 5.0 only the first exists, worded to cover both. Each program's own help text in all four earlier versions, on both machines, shows two arguments from 5.1 and one in the older two; 5.3's help text on the Mac carries both. This decides which Python modules a script can reach, not whether a file's own scripts are allowed to run.

**What no version settles**, and this is in *What is not established* at the top: whether Freestyle's line-style scripts run without the enabling flag. Every version's manual lists rendering with Freestyle among the routes that run even when automatic execution is off, on the grounds that they need a person to act — and a render started from outside involves no person. The test could not be written without guessing at the set-up, and more machines do not help that.

---

## 12. Can a render started from outside Blender be pointed at one named scene of a file, and what does it render when it is not told which?

**How it is known.** The naming argument is **documented**. Applying the active/current-scene wording to a command-line render is **inferred**, and which scene that means on loading is **not in the sources**. The runtime answer is **observed** in every version: 4.5–5.2 on the Mac and one-card Linux machine, 5.3 on the Mac.

**Verdict: the same everywhere.**

Yes, and three answers came back identically in all five versions — on both machines for 4.5 to 5.2, on the Mac for 5.3:

- **A render can be pointed at a named scene**, with `-S`.
- **Told nothing, it renders the scene that was showing when the file was saved.** A file with two scenes, saved with the second showing, rendered the second in every version.
- **A name no scene bears is reported and then ignored.** The terminal printed `Can't find scene: 'NoSuchScene' in file: '…'` and then rendered the same scene it would have rendered anyway, saved the image and exited cleanly, in every version. Anything that relies on a bad scene name failing will not get a failure.

**One thing the test does not settle, in any version, on any machine.** The test files were built and saved by a Blender with no window. Whether a file saved from the interface records the active scene the same way was not tried anywhere.

**One difference beside this answer, in *Where Blender itself differs between versions* at the top.** Blender 5.0, 5.1, 5.2 and 5.3 have a Sequencer Scene, held per workspace, which can differ from the active scene. What carries the difference is 5.0's release note, saying the sequencer stopped using the window's active scene and now uses a scene held per workspace — and 4.5's half is tested as well: on the one-card Linux machine, building the test file in 4.5 reported that a workspace has no Sequencer Scene at all, and setting the test up on the Mac found the same. In all four versions that have one, a bare command-line render ignored it and rendered the active scene — on both machines for 5.0 to 5.2, on the Mac for 5.3. There is no launch argument for it in any version. So a file can hold something in 5.0 and later that 4.5 cannot, and an unattended render behaves the same in all five regardless.

---

## 13. What decides the list of render engines a file can choose between, and can that list grow on a given computer?

**How it is known.** The bundled engines and third-party additions are **documented**; Cycles' registration is **observed** in its own files. The list being exactly what is registered at runtime is **inferred**, then **observed** in tests. Terminal checks cover the Mac and named Linux machines for 4.5–5.2 and the Mac for 5.3. The earlier 5.0 and 5.1 Mac dropdown labels share a Creator-supplied photograph with 5.2 rather than an independent photograph each.

**Verdict: it differs** — the rule is the same everywhere; the list itself is not, and neither is what happens when you ask for it.

**What is the same in all five versions** — on both machines for 4.5 to 5.2, on the Mac for 5.3:

- **What decides the list is what is registered in the running Blender.** Three engines come with it; more arrive as add-ons; an add-on can be turned on for a single run at launch, or installed and enabled for good.
- **Blender's own data cannot be read for the list.** Asked for the choices on the engine property, a factory-startup Blender prints exactly one entry, its own EEVEE, in every version. `-E help` is what prints the real list.
- **`-E help` prints identifiers, not the human labels.** The dropdown in Blender's window offers the labels — EEVEE, Workbench, Cycles, in that order — in all five versions on the Mac, and the engine control was photographed again on the one-card Linux machine in all four earlier versions, reading a human label rather than an identifier.
- **The list shrinks when an add-on is switched off, and the render goes ahead anyway.** With the Cycles add-on off, a file saved with Cycles printed `Engine 'CYCLES' not available for scene 'Scene' (an add-on may need to be installed or enabled)`, the scene's engine read back as EEVEE's identifier, the printed list dropped to two, and the render ran on EEVEE at EEVEE's own 64 samples rather than the file's Cycles setting of 4. Identical in all five versions. Anything that trusts a file's engine name to survive must not.
- **Blender's own flag for "more than one engine is available" reads true** in all five.

**The list grows by an engine a launch script registers — tried in 5.3 alone, on the Mac.** A script that registers an engine named `CUSTOM`, handed in with `--python` ahead of `-E help`, made the listing print it as a fourth engine, and Blender's window, opened with the same script, offered EEVEE, Workbench, Cycles and Custom in its Render Engine menu; the engine property's own list of choices, read from a script, still held one entry. A file saved with that engine and opened without the script printed the same `not available` line, read back as EEVEE and rendered on EEVEE, exit 0. Opened with the script registered first, it kept its engine name. Not tried in 4.5 to 5.2.

**What differs, version by version:**

- **EEVEE's identifier.** `-E help` prints `BLENDER_EEVEE_NEXT` in 4.5, and `BLENDER_EEVEE` in 5.0, 5.1, 5.2 and 5.3. Blender 5.0's release notes state the change, all three machines printed the same identifiers for the four earlier versions, and the Mac printed `BLENDER_EEVEE` for 5.3. Workbench's and Cycles' identifiers are the same in all five.
- **Which identifier a version will accept, which is not the same question as which one is listed — and it depends on the route.** There are two routes a program can use, and they do not agree:

  | | `-E` on the command line | `scene.render.engine` set from a script |
  |---|---|---|
  | 4.5 | `BLENDER_EEVEE_NEXT` only. `BLENDER_EEVEE` errors, writes nothing, then hangs | `BLENDER_EEVEE_NEXT` only. `BLENDER_EEVEE` is refused |
  | 5.0, 5.1, 5.2 | **Both** names accepted, exit 0, same file size | `BLENDER_EEVEE` only. `BLENDER_EEVEE_NEXT` is refused |
  | 5.3 | `BLENDER_EEVEE` accepted; the old name not tried | `BLENDER_EEVEE` accepted; the old name not tried here, and refused under **Images 9** |

  The command-line half is this topic's: on the two-card machine every earlier version was asked to render under each name in turn, and Blender 5.0, 5.1 and 5.2 accepted `BLENDER_EEVEE_NEXT` as readily as `BLENDER_EEVEE`, each writing an image of the same size, though only the new name is in their listing, while Blender 4.5 given `BLENDER_EEVEE` printed `Error: engine not found 'BLENDER_EEVEE'`, wrote nothing, and then hung. That was run on that one machine, in all four earlier versions. The script half was settled elsewhere in this research, under **Images 9**. 5.3's row comes from its own runs on the Mac, which used the new name by both routes and never the old one; Images 9 records 5.3 refusing the old name from a script. **The name that works everywhere, by every route, is the version's own** — `BLENDER_EEVEE_NEXT` in 4.5, `BLENDER_EEVEE` in 5.0, 5.1, 5.2 and 5.3. Use the version's own name and neither route can trip. Anything that passes an engine name at launch, or reads one back out of a file, has to know which version it is talking to.

  What the command-line half does not show is in *What is not established* at the top: whether a 5.0 to 5.2 run under the old name really renders in EEVEE, and whether it warns; and whether 5.3 takes the old name on the command line.
- **How 4.5 ends.** In 4.5.12, both `-E help` and a render given an engine name no engine bears fail to end properly; whether output appears depends on the machine — every time, with and without background mode and with and without a factory-startup run, while a plain version check and ordinary renders exit 0. In 5.0, 5.1, 5.2 and 5.3, `-E help` exits 0 and the bad engine name exits 1 after printing `Error: engine not found`. In every version, on every machine, no image was written for the bad name. **What differs between the versions is that 4.5 ends badly; what differs between the machines is the symptom.** On the Mac, 4.5 ended with `libc++abi: terminating` and exit code 134. On the one-card Linux machine, 4.5 did not end at all, had to be killed at sixty seconds for exit code 124, and printed nothing at all before hanging. On the two-card Linux machine, 4.5 also had to be killed for exit code 124, but it printed its full engine listing first. On the RTX 4060 Ti machine, `-E help` in 4.5 again printed nothing at all and had to be stopped by a sixty-second ceiling, using a third of a second of processor time in the whole minute — it was waiting, not working — where 5.0, 5.1 and 5.2 each answered in under a second; that is the one-card machine's symptom showing again on a fourth machine rather than a fifth symptom. The two Linux machines with no display server add a further case of the same kind: asked for a window with no display to open it on, 4.5 printed its refusal and then hung, where 5.0, 5.1 and 5.2 printed theirs and exited 1. Four machines agree that 4.5 ends badly and 5.0 to 5.2 do not; none of the symptoms is chosen between, and a reader should not expect any one of them in particular. 5.3 ended cleanly on the Mac, the one machine it ran on.
- **What sits under the dropdown**, from photographs on the Mac: Feature Set and Device in 4.5, Device alone in the Creator's shot covering 5.0, 5.1 and 5.2, and Device alone in 5.3's. Not photographed on either Linux machine. **The property `feature_set` is absent in 5.0 to 5.3**, observed by property sweeps **under What a render demands 1 and 4**: all four earlier versions on both machines, and 5.3 on the Mac alone. This topic did not test the property in 5.3. Its Supported/Experimental description does not match the tested build.

**What no version settles**, and this is in *What is not established* at the top: whether enabling an add-on at launch happens early enough for the engine argument to see it. No third-party engine was installed on any machine this topic used and installing one was not allowed on any of them, so that half is untested in all five versions everywhere; the script-registered engine above is the nearest 5.3 came. **What a file naming a missing third-party engine does is settled** — **Add-ons 7** built one for the purpose and ran it in all four earlier versions on two machines: Blender says one line, renders the frame in EEVEE at EEVEE's own sample count instead of the file's, writes a picture, and ends at 0. 5.3 did the same with the script's engine, above. The nearest case this topic could reach itself — Cycles itself switched off — is above.

---

## 14. What values can a render's resolution percentage be set to?

*The two machines behind 4.5 to 5.2's answer are the Creator's Mac and the rented Linux computer with the RTX 5060 and no display server, and "both machines" below means those two. 5.3 was run on the Mac alone.*

**How it is known.** The kind, printed range and starting value are **documented**; treating the printed pair as the outright limits is **inferred** where the notation is unexplained. The actual property limits, assigned values and installed Python files are **observed** by tests on the Mac and RTX 5060 machine in 4.5–5.2 and on the Mac in 5.3. Reading a dragging bound is separate from trying a drag.

**Verdict: the same everywhere.**

The answer, once. **Whole numbers from 1 to 32767 inclusive, and the setting starts at 100.** There is no zero, no negative number, and no fraction — 12.5 is not a value this setting will take. Values above 100 are meant as well as allowed; the manual says in all five versions that the control both reduces and increases the size of the picture.

**There are two ranges, not one, and Blender itself was asked for both.** In all four earlier versions on both machines, and in 5.3 on the Mac, reading the property out of a running Blender gave exactly the same ten lines:

- the kind is a whole number, and the starting value is 100 — which is the control, because that is what the documentation states, so the readout really did reach the property the documents describe;
- the outright limits are **1 and 32767** — the pair every version's API page prints;
- the limits for dragging are **1 and 100** — a narrower pair that no version's documentation carries anywhere;
- the step is 10, the value can be read and written, and **the property reports itself as not animatable**.

So the pair printed in the documentation is the outright pair. The declared dragging maximum is 100; nobody tested where an actual drag stops. The five running Blenders gave the same ten lines.

**Anything outside those limits is quietly pulled back to the nearest one, and anything that is not a whole number is refused.** In all five versions — 4.5 to 5.2 on both machines, 5.3 on the Mac: 0 and −50 both store 1; 32768 and 40000 both store 32767; a number too large for a machine whole number is refused outright with an error; and 50.7, 0.4, `"50"` and `"abc"` are all refused with an error naming the setting and the kind of value it expected. Nothing at all is printed for the four numbers that were pulled back. That the refusals printed on the same channel is the control: the channel does carry complaints, so the silence over the corrected numbers is a real result and not a channel that never speaks. The full account of that is question 15.

**Not every value in the range gives a picture.** A percentage inside the range can still ask for a picture too small to make: 1 per cent of 4 pixels by 4 was refused with `Image too small` and no file was written, in all five versions. The range is what the setting will hold, not what a render will always produce. Again, question 15.

**Where the setting lives is the same in all five versions** — on the scene's render settings, not on any render engine, and drawn in the panel by one line of Blender's own interface code, which labels it `%`. Render presets that ship with Blender set it: fourteen of them in every earlier version, each setting it to 100, and eighteen in 5.3.

**Three differences found beside this answer, all in *Where Blender itself differs between versions* at the top.** Cycles' own files read the percentage from 5.0 onwards at one line of their interface file — line 399 in 5.0 to 5.2, line 414 in 5.3 — and do not mention it at all in 4.5. Every Python file inside each installed Blender was searched, in all four earlier versions on both machines and in 5.3 on the Mac: Blender 5.0, 5.1, 5.2 and 5.3 each carry that one line, dividing the percentage by 100, and Blender 4.5's Cycles carries no mention of the setting anywhere. Everything else the search found — the panel line, the presets, the preview renderer — is the same in all four earlier versions, and in 5.3 the same but for the presets below. A control count showed the search was reaching Python files inside every install, so 4.5's silence is a real absence and not a search that found nothing anywhere. **This changes nothing about what values the setting takes**: where those lines exist they only decide the wording of an advisory label in a panel. Blender 5.2's Cycles carries two Texture Resolution settings that 4.5, 5.0 and 5.1 have not got — 5.2's release notes announce a new Texture Resolution percentage in the Simplify panel, and the same search found `texture_resolution` and `texture_resolution_render` in 5.2's Cycles files and nothing of the kind in 4.5, 5.0 or 5.1, on both machines. They are decimal scale factors running from 0.00001 to 1.0 and starting at 1.0 — **a different setting from the render's resolution percentage**, which is unchanged, and it is named here so that nobody meeting a second percentage-shaped setting from 5.2 on mistakes it for the first. 5.3's Cycles carries the same two settings, found by the same search on the Mac. And **5.3 ships four render presets 5.2 does not** — square pictures at 2K, 3K, 4K and 8K — each of which sets the percentage, as the others do; the same search in 5.2.2 on the same Mac found the other fourteen and not these, and that was the whole difference between the two.

**One difference in documentation only.** The 4.5 and 5.0 API pages print `int in [1, 32767], default 100`. The 5.1, 5.2 and 5.3 pages print `Percentage scale for render resolution (in [1, 32767], default 100)` with `Type: int` separately. Same numbers and starting value; a different presentation.

**What is not established, and it is the same in all five versions.** What the `%` field does when an out-of-range number is typed into it, where the slider stops when it is dragged, and whether a driver or an animated value could carry the setting outside its range — all three are in *What is not established* at the top. And Windows, as everywhere in this topic.

---

## 15. What does a render do when it is asked for a value outside what that setting accepts, and what does it report?

*On the same two machines named under question 14 for 4.5 to 5.2, and on the Mac alone for 5.3.*

**How it is known.** No version documents the answer for this percentage specifically. Applying the general integer-property correction rule is **inferred**; wrong-kind values, reports and the render's result are **not in the sources**. The answer is **observed** from assignment and render tests by both routes, on the Mac and RTX 5060 machine in 4.5–5.2 and on the Mac in 5.3.

**Verdict: it differs** — in what a render prints, not in what it does. Everything the value does is identical in all five versions. What Blender says while doing it is not.

**What is the same in all five versions** — on both machines for 4.5 to 5.2, on the Mac for 5.3:

- **The value is corrected the instant it is assigned, and nothing whatever is said about it.** Below the range stores 1; above it stores 32767. No error, no warning, no line of any kind. A control in the same run — assigning text, which is refused with a complaint on that same channel — proves the silence is real and not a channel that never speaks.
- **A fraction is refused, not rounded.** 50.7 and 0.4 are both refused with an error naming the setting. So is text, and so is a number too large for a machine whole number. This is the one point no version's sources settled, and it comes out the same in all five versions.
- **Whatever started Blender is not told.** With the switch that turns a script failure into a chosen exit status set to 42, a run that deliberately fails comes back 42 and a run that merely has its value corrected comes back 0. The control fired in every single run, in every version, so the 0 means the correction is genuinely not signalled and not that the switch never works. **Anything that judges a render by its exit status will never learn that the percentage it asked for was not the one used.**
- **The render then goes ahead at the corrected value, and the picture comes out at that size.** Measured out of the written files rather than worked out from the percentage: 1920 by 1080 at 200 per cent gives 3840 by 2160; at 33 per cent, 633 by 356; at a request of 0 per cent — stored as 1 — 19 by 10; and 4 by 4 at 32767 per cent gives 1310 by 1310. The same numbers in all five versions. The 200 per cent case is the control on the measuring, and it held everywhere.
- **A request that works out to less than one pixel is refused, and no file appears.** 4 by 4 at 1 per cent stopped with `Image too small` in all five versions. So a value inside the range can still make a render impossible — which is the second reading of this question, the one no version's sources answered.
- **A wildly large request does not refuse and does not warn — it simply starts.** Asked for 40000 per cent of 1920 by 1080, every version stored 32767 and began rendering a picture cut into 53,284 tiles. None finished; every run was still working when a time limit killed it — four minutes for 4.5 to 5.2, a minute and a half or a minute for 5.3 depending on the route — with hours of work estimated as remaining wherever an estimate was printed. Searching those runs for errors and warnings found none. **Nothing tells a caller that the number it asked for was cut down, and nothing tells it that the render it started cannot realistically finish.**
- **How the frame is started changes nothing about the value.** Both ordinary ways of rendering one frame with no window — Blender's own single-frame argument after the script, and the render operator called from inside the script — stored the same value and wrote a picture of the same size in every case, in all five versions.
- **The refusal under the operator route is the same in all five versions**, and it is named here so that nobody mistakes it for a difference. All five raise it into Python as an error inside a traceback, all five print a bare `Error: Image too small` beside that traceback, and the run still ends with status 0 — those runs did not carry the exit-status switch. The timestamped `reports | ERROR Image too small` line never appears under the operator route in any version; it belongs to the other route, and that is the change of line shape below.
- **Where the two machines differed at all, they differed in wording and in speed only.** One message about the limits of a machine whole number is written in hexadecimal on the Linux machine and in ordinary figures on the Mac, and 5.3 on the Mac wrote it in ordinary figures. The Mac got further through the enormous render in the time allowed. Neither is a fact about Blender.

**Opening a file with a Python line and no render argument renders nothing — tried in 5.3 alone.** On the Mac, a file opened with no window and a line that printed its settings, with no `-f` or `-a`, printed the settings, wrote nothing and exited 0. Not tried in 4.5 to 5.2.

**What differs, version by version — all of it in what is printed:**

- **The shape of the ordinary render log changed at 5.0.** In 4.5 the tested lines are plain: `Fra:1 Mem:8.51M (Peak 9.10M) | Time:00:00.00 | … | Sample 4/4`, `Error: …`, `Warning: …`, `Saved: '…'`. From 5.0 ordinary log lines carry elapsed time and a channel, for example `00:00.271  render | Rendering frame 1`, `00:00.269  reports | WARNING …`, `00:00.323  cycles | ERROR …`. A progress line reads `Fra: 1 | Mem: 1M | Sample 8/8`, beside an `Engine: Cycles` line absent in 4.5. The Image too small refusal takes the same split. **Observed on the tested machines**, with 5.3 on the Mac alone; bare Python errors under the operator route remain as described above.
- **Under the operator route, 5.0, 5.1, 5.2 and 5.3 print nothing at all while they work, and 4.5 prints its progress as usual.** There are two ordinary ways to render a single frame without a window: Blender's own single-frame argument placed after the script, and calling the render operator from inside the script. Both were run, for six different resolution settings, in all four earlier versions on both machines — ninety-six runs — and in 5.3 on the Mac. For a render that finishes, 5.0, 5.1 and 5.2 print twelve lines in total and not one progress line, against forty-six or more by the other route; 5.3 printed seven, among them Blender's `Saved:` line, and not one progress line, against fifty-four or more by the other route. How many lines there are turns on what each run's own script prints, so those totals are not compared. For a render left going a full minute, 5.0 to 5.2 print seven lines and 5.3 three — in each case the script's own — and nothing of Blender's. Blender's own `Saved:` line is still there among the lines of a finished render, which proves the output was being captured and the progress lines are genuinely absent. Blender 4.5 prints its full run of progress lines by either route. The refusal message differs the same way: under the operator route all five versions raise it into Python as an error, and 5.0, 5.1, 5.2 and 5.3 never print the report line their other route prints. This came back identically on both machines for the four earlier versions, run for run. **Anything that watches a render's output to see whether it is alive must know that, from 5.0 onwards, one of the two ways of starting a frame says nothing whatever while it works.**

**What is not established, and it is the same in all five versions.** Whether anything appears in the Info editor or the status bar when a value is corrected — the terminal half of that is answered above and the window half was never looked at; whether a driver or an animated value could carry the value out of range; and whether a file can hold a value outside the range. All three are in *What is not established* at the top. And Windows, as everywhere in this topic.

---

## 16. When a render is started from outside Blender and given a sample count of its own, what decides how many samples it actually takes, and what else has a say in when it stops?

*The two machines behind 4.5 to 5.2's answer are the Creator's Mac and the rented Linux computer with an RTX 4060 Ti and no display server, and "both machines" below means those two. 5.3 was run on the Mac alone, and every Cycles render behind its answer was on the processor too.*

**How it is known.** The Cycles sampling rules have both documented and tested support, kept separate below.

- **Documented in all five versions**, from each version's own manual and Python API: Cycles' Max Samples and what it means; adaptive sampling with its Noise Threshold and Min Samples; the Time Limit, with the note that pre-render work is excluded from it; the Sample Subset with its Offset and Length; the per-view-layer Samples override; the scene-level Layer Samples choice of Use, Bounded and Ignore; and EEVEE's Render Samples. In 5.3, EEVEE's time limit as well.
- **Observed in all five versions**, from Cycles' own files rather than from documentation, because Cycles is an add-on and its scene settings have no API page in any of the five: the name, kind, default and range of every one of those Cycles settings.
- **Inferred, and marked as such**: the step from "a view layer can carry a number of its own" to "that number beats the count the run was given". The test below settled that step in all five versions.
- **Not in the sources, in all four earlier versions**: that EEVEE and Workbench have no time limit and no early stop — every one of them records this as an absence on the page rather than as a statement; in 5.3 the absence is Workbench's alone. Also not in the sources in any of the five: the rule by which the several limits combine when more than one is in force — for the four earlier versions because the part of Cycles that runs a render session is missing from every one of their file snapshots, which hold the render kernel and utilities only.
- **Left open by every earlier version's sources, in the same words**: whether the Cycles Time Limit is spent on each frame or across a whole run; whether a frame ends once every pixel has converged; and how the limits behave together. 5.3's leave the first and the last open and infer the second.

All of that was settled by test, in all five versions, every run from a terminal with no window — on both machines for 4.5 to 5.2 except where an answer below names one, and on the Mac for 5.3.

**Verdict: it differs.** EEVEE obeys a view layer's own sample count from 5.0 and has a time limit in 5.3 alone. The tested Cycles sampling rules agree in all five versions. Its printed ending differs: **Reports 11** records a closing zero in 5.2.2 and none in 5.3 under a time limit and adaptive sampling, on the same Mac. Cancellation remains unresolved across versions.

**The answer, once.** A sample count is a **ceiling, not a quantity**. In Cycles, five things stand between the number a run is given and the number of samples a pixel actually gets. In EEVEE two things push the other way and take the number **above** what was given, and in 5.3 a time limit can stop it short. In Workbench there is no such number at all.

### Cycles sampling — on both machines for 4.5 to 5.2, on the Mac for 5.3

**Adaptive sampling turns the count into a maximum, and it is on unless somebody turns it off, at a threshold of 0.01.** Three renders of the same frame at Max Samples 4096 with no time limit: with adaptive sampling off it ran all 4,096 samples; with it on at the default threshold it still reported `Sample 4096/4096` but took roughly a quarter of the time; with it on at a loose threshold of 0.5 the whole frame stopped at `Sample 32/4096`. The same three counts in all four earlier versions on both machines, and in 5.3 on the Mac. So a frame does end before its maximum once its pixels have converged, rather than only single pixels stopping — and at the default threshold the skipping shows up in the clock and not in the count.

**The Cycles Time Limit is spent on each frame, not across the whole run, and a frame cut short is still written.** Question 2 above carries this topic's whole account of the time limit, including the twenty-four runs on the Mac and the idle Linux machine, and 5.3's three.

**Nothing announces that the time limit has fired.** In the tested cases the frame ended below its maximum and was still written. **Read the last non-zero sample count.** A trailing `Sample 0/N` is recorded before `Finished` in 4.5 to 5.2 and in interrupt captures from 4.5 and 5.0. It is not a result. Whether every cancellation prints it in every version is **not established**: some captures omit it, including the 5.1 and 5.2 cancellation captures, and 5.3's quoted final lines go straight from the break message to `Cancelled`. 5.3's early-convergence capture also shows no trailing zero. These omissions alone do not prove a version change. **Reports 11** ran 5.2.2 beside 5.3 on the same Mac, under the time limit and under adaptive sampling: 5.2.2 printed the closing zero and 5.3 did not. It ran no such pair for a cancellation.

**The picture size also matters.** The untiled runs behind the last-count reading used 320 by 240. On a tiled frame a zero can appear after a successful render: at 2049 by 2049 the last line of a run that exited 0 and wrote its picture reads `Rendered 3/4 Tiles, Sample 0/16` (**Reports 7**). No tiled frame that really stopped short has been captured, so that line alone does not tell a finished tiled picture from one cut short.

**A view layer's own count, and what Layer Samples does with it.** Six results, identical in all four earlier versions on both machines and in 5.3 on the Mac, with the scene's count supplied at launch rather than saved in the file. The number after each layer is the count that layer was given:

| Layer Samples | override 64, scene count 1024 | override 1024, scene count 256 |
|---|---|---|
| `Use` | first layer 1024, second layer **64** | first layer 256, second layer **1024** |
| `Bounded` | first layer 1024, second layer **64** | first layer 256, second layer **256** |
| `Ignore` | first layer 1024, second layer **1024** | first layer 256, second layer **256** |

So a view layer's own number stands in place of a count supplied from outside under `Use`, is capped by it under `Bounded`, and is thrown away under `Ignore`. The default is `Use` in all five versions. **This is the step the sources only inferred, and it holds.** A count handed to a render can be silently displaced by a number stored inside the file.

**Sample Subset decides how many samples are rendered, instead of the maximum.** With Max Samples 2048 and a subset of length 256, every version reported `Sample 256/256` and counted its progress against 256 rather than against 2048. Turning adaptive sampling on as well changed nothing — still 256. Adding a two-second time limit instead cut the subset short, in every version on both machines; how far it got is a clock reading and differs with the machine.

**The Cycles viewport sampling settings have no part in a render with no window.** Render sampling held fixed while all four viewport numbers were swung from one extreme to the other, twice, and the two pictures compared channel by channel. Both runs reported the same count and the comparison came back `PIXELS max_difference 0.0 channels_differing 0 of 307200` — eight times, once per version per machine, and once more in 5.3 on the Mac. Every version's sources had this as an inferred step; it is now observed.

**The settings travel with the file and nothing rewrites them on the way.** Eleven Cycles values and EEVEE's render sample count were set, saved and read back out of the saved file. The reply was identical in all four earlier versions on both machines and in 5.3 on the Mac: Max Samples 137, Time Limit 42.0, adaptive sampling off, adaptive threshold 0.005, Min Samples 9, Layer Samples `Bounded`, sample subset on, offset 3, length 11, the view layer's own Samples 64, EEVEE's Render Samples 77.

**A count supplied at launch is the number the render counts to — but only if it is supplied before the render argument.** A file holding 4,096 was given 32 at launch. With the instruction placed before the single-frame argument, every version reported `Sample 32/32`. With the same instruction placed after it, every version reported `Sample 4096/4096` and took between thirty-six and forty-three seconds on the rented Linux machine, and longer on the Mac, where 4.5 took a minute and a half, and 5.3 a minute and a half as well — rather than under a second, so the file's own number really was the one rendered. The counts are identical in all five versions; only those times differ.

**What else can stop a render started from outside.** Each of these was tried three ways on a 4,096-sample frame interrupted after eight seconds, in all four earlier versions on both machines and in 5.3 on the Mac.

- **An interrupt signal** stops it. The output includes its real count, a break message, `Cancelled` and `Blender quit`; it writes no file and exits 1. A trailing zero is present in some earlier captures and absent from others; its coverage is the open point above.
- **A terminate signal** stops it too, and says nothing. No further output, no file, and the shell reports exit status 143.
- **Turning off the abort handler changes nothing.** The same break message, the same `Cancelled`, the same exit status 1, the same absent file.

One thing worth knowing about the interrupt: in the first batch of tests the stop landed on the same **sample number** in every version on both machines, not at the same moment. The signal was sent at eight seconds; on the Mac the render stopped around twenty-two seconds and on the Linux machine around ten, and both stopped at `Sample 1025/4096`. That is a difference in machine speed, not in Blender. It is not perfectly repeatable, though: when the same eight runs were made again later, seven landed on `Sample 1025/4096` again and one — 5.0 on the busy Mac — landed on `Sample 2049/4096`. In 5.3 the interrupt landed on `Sample 736/4096`, a little under eight seconds in by Blender's own clock, on a busy Mac with the scene saved with adaptive sampling on; that turns on the clock and is not compared.

**EEVEE takes exactly the number it is given, except for two features that raise it.** Identical in all four earlier versions on both machines and in 5.3 on the Mac:

| | reported |
|---|---|
| Render Samples 64, with Cycles' Max Samples left at 4096 in the same file | 64 of 64 |
| Render Samples 64, motion blur on with five steps | 65 of 65 |
| Render Samples 16, depth of field with Jitter Camera on | 19 of 19 |
| Render Samples 16, depth of field with Jitter Camera off | 16 of 16 |

The two features are documented: Jitter Camera adds depth-of-field samples by a stated formula, and with motion blur "the render sample count is rounded up to the next multiple of steps". So the engine chosen decides which count is read — Cycles' 4,096 sitting in the same file had no effect on an EEVEE render — and in EEVEE the number given is a floor as much as a target. That EEVEE has no noise threshold and no minimum count in any version, and a time limit in 5.3 alone, is under question 2 and below.

**Workbench has no sample count to give.** In all four earlier versions on both machines and in 5.3 on the Mac the run printed no count of any kind — no `Sample n/N` line and no `Rendering n / N samples` line. Its one anti-aliasing setting holds one of a fixed list: off, single-pass, 5, 8, 11, 16 or 32, and it read 8. Cycles' 4,096 sitting in the same file changed nothing about the run.

**A command-line render ignores "Render Single Layer", so a second view layer's own sample count is in play that a person clicking Render would not have hit.** With that switch turned on and the first layer active, both view layers rendered anyway, at 256 and at 64, in all four earlier versions on both machines and in 5.3 on the Mac. Question 1 above establishes that every version's manual says this outright.

**The default Noise Threshold is not the automatic value Cycles uses at zero.** At the same maximum of 1,024, a threshold of 0.01 and a threshold of 0 both reached the maximum, but the second took roughly twice as long and made a different picture — between 23,558 and 25,357 of 307,200 channels differing, in every earlier version on both machines, with 5.2 lowest and 5.0 highest; 5.3 gave 23,511 on the Mac, a reading of its own and not compared. What the automatic threshold actually is, and whether it moves with the sample count, is settled nowhere; the cost of a single sample changes with the count on its own, so the runs at different maxima cannot be read against each other.

**One of the two automatic values is printed.** Set Min Samples to zero and Cycles works one out from the threshold. Blender 5.2 prints it — `Adaptive sampling: automatic min samples = 64` — under an informational logging level, captured under **What actually ran**. That logging level is untested for this value in the other four versions, rather than proved silent. The automatic **noise threshold** is still printed nowhere anyone has looked.

**A Min Samples set above Max Samples changes nothing about the count reached.** Both values were accepted as set — 8 and 64 — and the render reported `Sample 8/8`, exactly as the control run with Min Samples 0 did, within hundredths of a second. The maximum wins and the minimum is simply never reached. The same in all four earlier versions on both machines and in 5.3 on the Mac.

**Asking Cycles to print its statistics reports no sample count.** In all four earlier versions on both machines and in 5.3 on the Mac the run printed a block of mesh geometry memory, per-kernel timings and shader and object statistics, and no figure anywhere in it names a sample count.

**All five versions carry a Sample Count debug pass** on the Cycles view-layer settings. Nobody read the pass itself, so the spread of counts across one picture is still unmeasured.

**Cycles carries a version-conversion pass that can rewrite a stored sample count as a file loads**, read out of Cycles' own files. The eleven Cycles sampling values and EEVEE's render count above were saved and read back inside one version, which cannot exercise a pass that fires when a file crosses a version boundary. The test that can is under **What a render demands 5**: a file saved by 4.5 and opened in 5.0, 5.1 and 5.2 has its tiling switch turned on and its tile size changed from 256 to 8192, while its stored count of 11 comes back as 11 in all four, on both machines. So the pass is not dormant and it does rewrite a Cycles value — and across the only version boundary this project has crossed with it, it leaves the sample count alone. 5.3's Cycles carries such a pass too, read in its own files, which by its own code passes over files saved by anything newer than 5.2.8; 5.3 was not given an older file. The half that stays untestable, a file from a Blender old enough to have used squared samples, is in *What is not established* at the top.

### What differs, version by version

**A view layer's own sample count changes an EEVEE render in 5.0, 5.1, 5.2 and 5.3, and does nothing in 4.5.** With EEVEE's Render Samples at 256 and a second view layer carrying its own Samples of 8:

| | first layer | second layer |
|---|---|---|
| 4.5 | 256 of 256 | **256 of 256** |
| 5.0 | 256 of 256 | **8 of 8** |
| 5.1 | 256 of 256 | **8 of 8** |
| 5.2 | 256 of 256 | **8 of 8** |
| 5.3 | 256 of 256 | **8 of 8** |

**Two separate things carry this, and either would be enough.** One test, run in all four earlier versions on both machines, came back differently — and both machines found the same split, so it is the versions and not the operating systems; 5.3 gave 5.0 to 5.2's answer on the Mac. And Blender 5.0's release notes state the change in words, saying view layer overrides are newly supported and warning that older files using them will render differently. Blender 4.5's documents are silent on EEVEE and view-layer overrides, where 5.0's, 5.1's, 5.2's and 5.3's all quote a manual page saying that in EEVEE the override "is always used" — but that silence is not what carries the difference here; the release note and the test are. Anything working out how many samples an EEVEE render will take must know that a per-layer override is obeyed from 5.0 and ignored in 4.5.

Cycles obeys a view-layer override in all five versions, as the table further up shows. It is EEVEE alone that changed.

**EEVEE has a time limit in 5.3 and in no earlier version.** On the Mac, a 5.3 EEVEE render with Render Samples 4,096 and a two-second limit stopped at 275 of 4,096 and wrote its picture, printing nothing about the limit. 5.2.2 on the same Mac has no such setting, no earlier version's settings held one on either machine, and 5.3's API change log lists it as added between 5.2 and 5.3. So from 5.3 an EEVEE render, like a Cycles one, can end below the count it was given. Question 2 carries the full account.

### Two things about the machines, which are not differences between versions

**The EGL line, and three runs that stand together.** On the rented Linux machine, which has no display server at all, the first batch of tests recorded every EEVEE render printing three copies of an `EGL Error … EGL_BAD_MATCH` line before it started — in all four versions, with every render working and writing its image anyway. The tests used graphics drivers 580.95.05 and 580.173.02. Two later runs on the newer driver found **no EGL line at all**, in any of the four versions, with every render again working and writing its image. What all three runs agree on is the thing the answer rests on: an EEVEE render works on a machine with no display server and writes its picture either way. The Mac has a display server and never printed the line, so the question does not arise there.

**Blender 4.5 does not answer `-E help` on that rented machine**, which is the "4.5 ends badly" difference under question 13 showing again on a fourth machine. **Nothing in the answer to question 16 rests on it**: the engine identifiers were read off the Mac, where the command answers in every version, and the same 4.5 binary on the same machine renders perfectly well in both Cycles and EEVEE, as the tests above show.

### What is not established

Every one of these is in *What is not established* at the top, and not one of them is a difference between versions: the automatic noise threshold Cycles picks at zero; whether a count stored by a Blender old enough to have used squared samples is rewritten as the file loads; whether the Time Limit is counted the same way on a graphics card; whether it applies per view layer when a scene has several; whether the Layer Samples control is shown only when an override exists, and whether the setting still bites when it is hidden; how many samples different pixels of one picture actually took; the general rule combining the several limits, and how often the convergence check runs; what the interface says when a time limit ends a render early, in 4.5 to 5.2, 5.3's being under question 2; whether a render started inside Blender's own window differs, beyond the one 5.3 started; whether a frame the time limit cuts short is still denoised; and whether cancellations consistently print a trailing `Sample 0/N` line in every version. Each of those is the same silence in every version except where it names one. And Windows, as everywhere in this topic, and for 5.3 Linux and any graphics card.

**The missing tests are distinct.** Every Cycles time-limit run used the processor and one view layer, so neither a graphics-card timer nor a limit per layer is settled for Cycles; EEVEE's one time-limit run, in 5.3 on the Mac, is above. The Sampling panel was photographed without a layer override; nobody photographed it with one, and keeping a saved Layer Samples value does not prove its effect while hidden. A Sample Count pass exists, but nobody read its per-pixel values. The general order of the limits, frequency of convergence checks and width of the neighbour filter are not stated in the supplied sources.

**Very old files are a separate case.** Cycles' version-conversion code in 4.5 to 5.2 squares counts from files old enough to have used squared samples; another branch turns adaptive sampling off for files old enough never to have set it. These are **observed code rules**, with their load-time effect **inferred**, not tested on such a file. They require a file from Blender 3.0 or earlier, which none of these tests used.

---

## 17. When a render is started from outside and an instruction handed in at launch sets the resolution percentage, the sample count, or whether a view layer renders, what name does that instruction use for each of those three settings?

*One machine ran every part of this answer: the **Creator's Mac**, carrying Blender 4.5.12, 5.0.1, 5.1.2 and **5.2.1**, and for 5.3 Blender 5.3.0 Beta. **Nothing was rented for it**, and no Windows machine ran any of it — untried rather than untestable. So every run **below** is a macOS run and nothing here settles any other operating system on its own.*

***But for 4.5 to 5.2 the names themselves are not untried on Linux.** Four other questions set them from a launch-time Python expression on **four different rented Linux machines**, in all four versions, and got the values they asked for: **Settings 1** set the resolution percentage, a sample count and a view layer's own switch in one expression on the one-card machine — the render came out 50 by 25 instead of 200 by 100; **Settings 16** set a scene's sample count and a view layer's own override at launch on the RTX 4060 Ti machine; **Settings 14** read the resolution percentage out of the running program on the RTX 5060 machine; and **What a render demands 3** set a sample count from a start-up script and watched it reach the render. **So three of the six names in the table below are exercised on Linux by `Settings 1` alone, in all four versions, and `Settings 16` adds a view layer's own override.** The two nobody has set on Linux are EEVEE's `taa_render_samples` and Workbench's `render_aa`. In 5.3 none of the six has been set on Linux. What rests on the Mac alone is listed at the end of this answer.*

**How it is known.** The non-Cycles property names are **documented** in the API; Cycles' definitions are **observed** in its own files. **5.3 also documents `bpy.context.scene.cycles.samples` in an API example.** Joining a property to its scene path is otherwise **inferred**, except for the full resolution-percentage example. Using the names in launch-time Python is **inferred**, then **observed** in the tests below. No separate naming for a start without a window is stated outright.

**No dedicated launch argument for these settings is established by the sources' list.** That absence is **inferred** or labelled **not in the sources**; the built help listing and the launch tests below provide separate **observed** evidence.

**Every run below is observed on the Creator's Mac.** Tests used factory settings and the default scene — a cube, camera and light, 1920 by 1080 at 100 per cent, one view layer — except where a test changes them. The 5.3 window checks used simulated input and a throwaway settings folder. Nothing here tests what another add-on might do to these names.

**Verdict: it differs** — in what happens when a name is wrong, not in the names themselves. **The names are identical in all five versions.** What differs is that Blender 4.5 alone accepts a wrong name on Cycles' settings without a word. Three further differences came out of the same runs and are set out below beside it.

### The sentence worth more than the names

**Neither the exit status nor a readback separates a working instruction from a broken one. What Blender prints does — except in 4.5 on Cycles' settings, where nothing is printed either — and so does what lands on disk.** Three results carry that between them.

- **A wrong name does not stop the render and does not show in the exit status.** Three names were deliberately misspelled in one run in the four earlier versions, and one in each of three runs in 5.3, and the frame was rendered anyway. In all five versions Blender rendered it, saved the picture, and **exited 0**. Where the misspelling is refused, the refusal is on the terminal and nowhere else: there to be read by anything capturing Blender's output, invisible to anything watching only the exit status.
- **In Blender 4.5 the complaint is missing as well, on Cycles' settings.** There, a name Cycles' settings do not carry is accepted with no error of any kind and the value is gone the instant it is read back. 5.0, 5.1, 5.2 and 5.3 refuse it, and every other settings group refuses in all five versions. Full account below.
- **A correct name handed in before the file is loaded prints the right value back and has no effect.** A file saved at 100 per cent, the same instruction handed in twice: both orders printed the value set, and only one order changed the picture — 480 by 270 with the instruction after the file, 1920 by 1080 with it before, because loading the file replaced the scene and with it the value just written. **The readback does not even warn**, because at the moment of reading the value really was what had been set. The same in all five versions — 5.3's by question 1's before-and-after test, which printed the values set and wrote a picture at the file's own size.

So the exit status is 0 either way; the readback is the value asked for either way; and in 4.5 on Cycles' settings even the printed complaint is absent. **What tells a working instruction from a broken one is that complaint on the terminal where there is one, and otherwise the size of the written picture, the count Blender prints while it renders, and the channels in the file** — and every test below that shows a name works, shows it that way.

**One thing is the opposite of all this, and a builder has to know which they are looking at.** An unknown *switch* in Blender's own part of the launch line — before the bare double dash — is not swallowed. `--render-percentage 50` — the shape a switch for this setting would take — was handed in on the launch line of a file that would otherwise have rendered, in all four versions: Blender took it for a file name, could not read it, **wrote no picture at all, and exited 1**. 5.3 did the same. A wrong *name* inside a Python instruction stops nothing and shows in nothing. Two completely different behaviours, and the same in all five versions.

### The names, once, and they are the same in all five versions

| Setting | The name the instruction uses | Written out from a scene |
| --- | --- | --- |
| Resolution percentage | `resolution_percentage` | `scene.render.resolution_percentage` |
| Sample count — Cycles | `samples` | `scene.cycles.samples` |
| Sample count — EEVEE | `taa_render_samples` | `scene.eevee.taa_render_samples` |
| Sample count — Workbench, so far as it has one | `render_aa` | `scene.display.render_aa` |
| Sample count — one view layer's override | `samples` | `scene.view_layers["…"].samples` |
| Whether a view layer renders | `use` | `scene.view_layers["…"].use` |

Three things about that table.

**The bare word `samples` names two different settings**, and which one is meant depends entirely on what it is written after: a scene's Cycles count, or one view layer's override of it.

**A name does not depend on which engine is running.** All six were set in one expression, at one launch, in a run whose engine was Cycles, and every one read back as the value set with nothing refused — in all five versions.

**Every near-miss name exists in all five versions too, under the same name, with the same label, the same description and the same starting value**: `use_single_layer`, Cycles' `preview_samples`, EEVEE's `taa_samples`, Cycles' `use_layer_samples`, and Grease Pencil's `aa_samples`. `render_aa` offers the same seven choices in all five and starts at the same one. The names read identically from the running program in all five. The non-Cycles names are documented in API pages; Cycles' definitions come from its own files, with the 5.3 API example noted above. Uneven source coverage establishes no version difference.

### The name is not the word the interface shows, and that holds in all five versions

| Setting | The instruction says | The program's own label says | The panel says |
| --- | --- | --- | --- |
| Resolution percentage | `resolution_percentage` | Resolution Scale | `%` |
| Sample count — Cycles | `samples` | Samples | Max Samples |
| Sample count — EEVEE | `taa_render_samples` | Render Samples | `Samples` under `Render` (5.3) |
| Workbench's anti-aliasing | `render_aa` | Render Anti-Aliasing | `Render`, reading `8 Samples` (5.3) |
| Whether a view layer renders | `use` | Enabled | Use for Rendering |

**The label column was read out of the running program in all five versions and is identical in all five**, word for word. **The panel column is a photograph**, and it does not reach as far: the Format panel was photographed in all four versions and shows the same three rows in each, the third labelled with nothing but `%`; Cycles' Sampling panel and the View Layer panel were photographed in all four and read in 4.5.12 and 5.2.1 only. Where a panel is too narrow the label is cut, and the view-layer switch reads `Use for Rende…` on screen. **In 5.3 all five panels were photographed**, on the Mac: the Format, Sampling and View Layer panels show `%`, `Max Samples` and `Use for Rendering` as in the earlier versions; EEVEE's Sampling panel labels the render count `Samples` under a `Render` heading; and Workbench's shows a `Render` menu reading `8 Samples`.

**The manual is a fourth wording again, and it does not name every one of them.** Every version's manual calls the resolution percentage "Percentage" and the view-layer switch "Use for Rendering" — quoted from all five. For the Cycles count some versions' manuals were quoted as "Render (Max) Samples" and for EEVEE's as "Samples", but those two exact manual wordings are not established in all five. No version's manual gives any of these settings a name an instruction could use.

**Two of those wordings are worth keeping in mind.** For the resolution percentage there are four different words for one setting, and only one of them is the name an instruction may use. And **the panel writes one label, `Max Samples`, over two different Cycles settings** — the render's `samples` and the viewport's `preview_samples` — telling them apart only by which group they sit in.

### Each name, exercised — the same result in all five versions

Each test had a control run beside it, and the control is what makes the second run mean anything. 5.3's runs on the Mac gave every result below.

- **`resolution_percentage` changes the picture Blender writes.** 1920 by 1080 with no instruction, 480 by 270 with the value set to 25.
- **Cycles' `samples` changes the count Blender prints as the one it is working towards** — `/4096` with nothing set, `/3` with the instruction. Setting `preview_samples` to 200 in the same breath changed nothing about the render, so the two are different settings and the render's count is the one called `samples`.
- **EEVEE's `taa_render_samples` does the same for EEVEE** — `Rendering 64 / 64 samples` with nothing set, `Rendering 3 / 3 samples` with the instruction. Setting `taa_samples` to 200 alongside it changed nothing.
- **`scene.display.render_aa` is the route to Workbench's anti-aliasing, it takes the value as text and not as a number, and it changes the picture.** The number `8` is refused with a complaint that a piece of text was expected; the text `'8'` is accepted. Rendering the same frame at `'OFF'` and at `'32'` gives pictures differing by up to 0.243 on a nought-to-one channel scale — **the same figure to the last digit in all four earlier versions**, and the same to four places in 5.3 — while rendering twice at `'OFF'` gives pictures differing by exactly 0.0, which is what makes the first number a result rather than noise.
- **A view layer's `use` removes that layer from the file Blender writes.** The file was asked for as a multi-layer OpenEXR with `-F OPEN_EXR_MULTILAYER` on the launch line. With two layers on, that file carries both layers' channels; with one switched off, that layer's four channels are gone and only the other's remain.
- **A view layer's own `samples` beats the scene's count.** Scene count 64 and no override finishes at `Sample 64/64`; the same run with the override at 5 finishes at `Sample 5/5`. Blender reported the Layer Samples choice as `Use` in every version without being asked to set it. Question 16 above carries the full account of what each of the three Layer Samples choices does.

### No launch switch of Blender's own, and none of Cycles', in any of the five

Each version's whole help listing was captured and searched for `percent`, `sample`, `resolution`, `view layer`, `view-layer` and `viewlayer`. **Two hits in every version, and neither is a switch for any of the three** — one about how a screen is drawn, one about what is built when a file is opened. The listing runs to 434 lines in 4.5, 466 in 5.0, 498 in 5.1, 518 in 5.2 and 521 in 5.3, which are the same figures question 1 records for the Mac. **The built program offers no such switch.**

Cycles was asked the same question, with a control. A guessed `--cycles-samples 8` handed in after the double dash changed nothing in any version — the render worked towards its default of 4096, and Blender neither complained about the argument nor acted on it. **The control is what stops that being an empty result:** a documented Cycles argument handed in the same way made Cycles print statistics in every version — seven blocks in the four earlier versions, seven lines naming statistics in 5.3 — so arguments in that position genuinely are read, and a sample-count argument is absent rather than unreachable. Asking Cycles for a help listing of its own printed nothing, in any version.

### The near miss: the name that looks right and does nothing

**`use_single_layer` is set, reads back as set, and both view layers still come out** — in all five versions, when the render is started from outside. Blender's own description of it says as much, in the same words in all five: it affects rendering from the interface only and is ignored from the command line. **`use` is the name that governs whether a view layer renders from outside; `use_single_layer` is the one that looks like it and does not.** Question 1 above records that every version's manual says so outright.

### What differs, version by version

**A misspelled name on Cycles' settings is swallowed in Blender 4.5 and refused in the other four.** The same sweep was run in all five versions over all five settings groups:

| | Cycles' settings | render settings | view layer | scene display | EEVEE |
|---|---|---|---|---|---|
| 4.5.12 | **accepted, silently** | refused | refused | refused | refused |
| 5.0.1 | refused | refused | refused | refused | refused |
| 5.1.2 | refused | refused | refused | refused | refused |
| 5.2.1 | refused | refused | refused | refused | refused |
| 5.3.0 Beta | refused | refused | refused | refused | refused |

In 4.5 it is not one particular misspelling that slips through — two different invented names were both accepted, and the value is gone the moment it is read back, so what was written went nowhere. Everywhere else, and in every other version, Blender names the type and the missing name and says so on the terminal. **This is the exact shape of failure a program cannot detect**, and 4.5 is the one version where it can happen. Repeated tests on the same Mac agreed.

**The declared default of a view layer's on/off switch is `False` in 4.5 and `True` in 5.0, 5.1, 5.2 and 5.3.** Read out of the running program in all five versions, and each version's documentation says exactly what its own program says — so this is a difference in Blender and not in how the documentation was written. **It is the property's declared value and not what a view layer carries:** in all five versions the default scene's own layer reads on, and a layer made fresh reads on — so 4.5's `False` is not what a real layer starts at, and the other four's `True` is. The line is in the first list at the top so that nobody meeting the split in two versions' pages takes one of them for a mistake.

**A relative output path with no file loaded works in 4.5 and fails in 5.0, 5.1 and 5.2.** Asked to write to `//name` with no `.blend` loaded, 4.5 saved the picture into the current folder; 5.0, 5.1 and 5.2 each stopped with a render error saying the file could not be saved, and wrote nothing. Two runs per version, control and instruction, all consistent. This does not change which setting name works. With a file loaded the case does not arise, since the short form then means that file's own folder. **Not tried in 5.3.**

**Blender 5.2 and 5.3 offer three launch sub-commands where 4.5, 5.0 and 5.1 offer one.** `-c help` lists `extension` alone in the older three, and `asset_listing`, `extension` and `maketx` in 5.2 and 5.3, on the Mac. None names a resolution percentage, sample count or view layer.

### Documentation wording

**No page states outright that these names are independent of whether Blender has a window.** Using the documented name from a launch script is **inferred**, then **observed** in the tests on the named machines.

**The API pages change notation between 5.0 and 5.1.** In 4.5 and 5.0 a choice is written as `enum in ['BLENDER_EEVEE']` (4.5 uses its own engine identifier); in 5.1, 5.2 and 5.3 it is written as `Literal['BLENDER_EEVEE']`. Workbench's seven choices and starting value read identically from the running program despite the page's notation. This is a documentation difference, not a change in those values. The scene's Cycles block has a second notation difference: 4.5 prints `CyclesRenderSettings, (readonly)` and 5.2 prints `CyclesRenderSettings | None`. Whether 5.1 and 5.3 use either mark is not established; their quoted entries give the type without settling that part.

### What is not established

**What actually rests on the Creator's Mac alone** for 4.5 to 5.2, for the reason at the head of this question: **4.5's silent swallow of a misspelled name on Cycles' settings**; the **unknown-switch** result; the **relative output path with no file loaded**; the **count of launch sub-commands**; and the **label column** of the second table. Each of those is a macOS result and none has been tried on Linux. In 5.3 the whole of this answer rests on the Mac.

Every one of these is in *What is not established* at the top, and not one is a difference between versions: whether any of these names holds on **Windows**, which is where the real gap is; whether the two names no Linux run has exercised — EEVEE's `taa_render_samples` and Workbench's `render_aa` — hold there; whether these are the only names; a sample count belonging to a whole scene rather than to one engine; a sample-count name for any engine beyond Cycles, EEVEE and Workbench; whether Workbench's anti-aliasing choice counts as a sample count at all; and which scene an instruction should hang these names from. Reading a name off a hovering tooltip is unreached in all five as well. Reading one out of a control's right-click menu was done in 5.3 alone, on the Mac: the menu over Max Samples is headed `Samples` and the one over the view layer's switch `Use for Rendering` (questions 1 and 5). The relative output path with no file loaded is untried in 5.3.

**No single scene-wide count is established.** The sources give engine-specific names, and no runtime test searched for an additional shared count or another route to the same setting. No test reached another engine's sample-count name. Workbench's fixed anti-aliasing choices are observed; whether to call them a sample count is not settled by a test.

**Choosing the scene for an instruction remains open.** More than one scene access route is documented, but no rule here chooses between them or settles how each meets `-S`. These runs used the scene Blender hands the script by default.

---

## 18. What values will a render's sample count be set to, and what does a render do when it is given a value outside those?

*The two machines behind this answer are the Creator's Mac and the rented Linux computer with an RTX 3060, and "both machines" below means those two. The four Blenders on the rented machine were checked against the Mac's build for build before anything was run, and all four matched. 5.3 was run on the Mac alone.*

**How it is known.** Cycles' limits are **observed** in its own files. The other counts are **documented** where the API supplies their ranges. Applying the general correction rule to these particular counts is **inferred**; wrong-kind values and the resulting render are **not in the sources**. Runtime tests settle the results on the Mac and RTX 3060 machine in 4.5–5.2 and on the Mac in 5.3.

**Verdict: the same everywhere.**

### The sentence a builder needs, and it is a hazard

**A sample count outside the accepted range is silently corrected to the nearest limit, nothing whatever is reported, and the corrected number is what the render works to.** Ask Cycles for 99,999,999 samples and it stores 16,777,216 and settles in to render that many — no error, no warning, no line of any kind. Blender's own estimate of the time remaining ran between roughly nine and fourteen hours across the four earlier versions; the Mac was not a quiet machine that day, so those figures give the scale and nothing finer. 5.3's first estimate, on the Mac, read about four hours and forty minutes — a clock reading of its own, not compared. **Two homes were seen carrying the corrected figure, and neither is announced.** One is the setting itself, read straight back — ask Cycles for 99,999,999 and it reads back 16,777,216. The other is the ordinary progress line, as the number after the slash — but only where nothing else is setting that number. Question 16 above establishes two ordinary things that take it away: a sample subset makes the line count against the subset's length, and a view layer's own count makes it count against that. Neither was ever put on the same file as an out-of-range count, so what such a file prints has not been seen. A third home exists on record, and is untested for a corrected count: a written PNG carries `cycles.ViewLayer.samples`, read without Blender, identical in all four versions on both machines, and it is the maximum rather than the number reached (**Reports 11**). No run here read that field on a picture whose count had been corrected, so whether it carries the corrected figure has never been seen. The run at the other end of the range is the control on that: a count of 0, stored as 1, rendered its frame, saved it and exited 0.

**And the progress line is not always there.** Every run behind this answer was started from the command line. **Reports 3** records that a render started instead by calling Blender's render operator from a script prints **no progress line at all in 5.0, 5.1 and 5.2**, and prints the full progress in 4.5 alone, and question 15 above found 5.3 silent on that route too. So on that route, in four of the five versions, the progress line is not a place the corrected figure appears — and the read-back off the setting is the only one left.

Silent correction was observed in all four earlier versions on both machines and in 5.3 on the Mac, including assignments to the EEVEE, view-layer and Grease Pencil counts. The render results below are separate from those assignment tests.

**Each count is corrected against its own limits, and that is a trap worth knowing.** The same number is cut down for one engine and taken whole by another: 99,999,999 is above Cycles' ceiling and below EEVEE's, so Cycles works towards 16,777,216 samples and EEVEE towards 99,999,999. The capped tests did not finish those renders. Neither says a word. **The number a caller hands in tells it nothing about the render it has started.** What does tell it is the count Blender prints — **on a render started from the command line**; on the operator route in 5.0, 5.1, 5.2 and 5.3 nothing is printed, by **Reports 3** and question 15, and the value read back off the setting is the only reading left.

**The control is what makes the silence a result.** The same run printed two lines of its own on each output stream and made one deliberately wrong assignment, which was refused loudly. So the output was being captured and does carry complaints; the silence over a corrected number is a real silence.

### What each count will take

Read out of each running Blender with no file open, in all four earlier versions on both machines and in 5.3 on the Mac. **The reply was identical in every version and on every machine, apart from one line named below, which is not a limit.**

| The count | Accepted values | Starts at | Dragging is narrower |
|---|---|---|---|
| Cycles, final render | 1 to 16,777,216 | 4,096 | no |
| Cycles, viewport | 0 to 16,777,216, where 0 means unlimited | 1,024 | dragging stops at 1 |
| Cycles, sample subset offset | 0 to 16,777,216 | 0 | no |
| Cycles, sample subset length | 1 to 16,777,216 | 2,048 | no |
| Cycles, adaptive minimum, render and viewport alike | 0 to 4,096 | 0 | no |
| Cycles, path-guiding training samples | 0 to 2,147,483,647 | 128 | dragging stops at 1 |
| Cycles, viewport denoising start sample | 0 to 16,777,216 | 1 | no |
| EEVEE, final render | 1 to 2,147,483,647 | 64 | no |
| EEVEE, viewport | 0 to 2,147,483,647 | 16 | no |
| One view layer's override | 0 to 2,147,483,647, where 0 means no override | 0 | dragging stops at 10,000 |
| Grease Pencil's anti-aliasing count | 1 to 2,147,483,647 | 8 | dragging stops at 256 |
| Workbench's anti-aliasing | not a number: off, single-pass, 5, 8, 11, 16, 32 | 8 | — |

Three things about that table.

**Two declared dragging limits are absent from the documentation** — 10,000 for a view layer's override and 256 for the Grease Pencil count. They were read from the running program; nobody tried the drag itself.

**Cycles' path-guiding count does have a ceiling**, the whole-number ceiling, where its own declaration writes none.

**One line reads differently in 4.5 and it is not a limit.** The Grease Pencil count sits at 1 in 4.5's startup file and at 8 in the other four. **The declared starting value is 8 in all five**; what differs is what each version's startup file happens to hold, which is not a fact about the setting.

### What "inf" means, and that the documentation never says

Every version's Python API pages write the upper end of several of these ranges as the bare word **`inf`** rather than a number — the EEVEE counts, the view layer's override and the Grease Pencil count all print it. **It is 2,147,483,647**, established by asking each running Blender in all four earlier versions on both machines and in 5.3 on the Mac.

**Blender's own documentation explains it nowhere.** In each version, `inf` appears only ever inside a range and is never defined, expanded or given a meaning — not on the page that explains how to read the reference, not on the front page, not on the pages describing a whole-number setting's limits, not in the manual, and not in the release notes. It is not a rare corner either: in one version it fills over three and a half thousand ranges, and in another the notation appears in more than twelve hundred separate pages. **That is the documentary answer to the first half of this question, and it is the same in all four earlier versions.** In 5.3 the quoted property descriptions print `inf` without giving a numerical ceiling or explaining the word.

The nearest any version comes is one operator whose stated starting value is 2,147,483,647 while its stated range says `inf` — which shows only that `inf` is no smaller than that, not that it equals it. The running program settles the rest.

### A number too big to be a whole number is refused, not corrected

Two different things happen and the line between them is exact. **Above the setting's own ceiling but inside a whole number**, the value is silently corrected. **Past what a whole number can hold**, it is refused outright with an error naming the setting, and the value it already had stays. That is the same in all four earlier versions on both machines and in 5.3 on the Mac.

**One behaviour, written two ways.** The refusal prints the whole-number limits in ordinary figures on macOS, 5.3 included, and in a shorter number notation on Linux. It is the same refusal for the same reason; only the wording differs — and it is the same message question 15 records for the resolution percentage, met again here. **Anything that reads Blender's output to catch this has to match both shapes** — and a third is possible on Windows, which nothing has tried.

### A decimal, text, or nothing at all is refused outright

In all four earlier versions on both machines and in 5.3 on the Mac, a sample count given a decimal, a word, a piece of text that looks like a number, or nothing at all is **refused with an error naming the setting and the kind of value it expected**. Nothing is rounded and nothing is truncated. **The tested Boolean exception is `True`**: it is accepted and stores 1.

Workbench's anti-aliasing is a fixed list rather than a number, and it behaves the same way: a choice outside the list is refused, the refusal names every real choice, and matching a choice is case-sensitive.

### Where this leaves a caller

Three things, all of them the same in all five versions, and together they are the whole answer:

- **A count that is too big is taken, cut down, and rendered** — silently, and at whatever cost the corrected number carries.
- **A count that is the wrong kind of thing is refused** — loudly, on the terminal.
- **Read the stored value to check whether it was corrected.** The progress line can also show the corrected count when that route prints one and no subset or view-layer override replaces it. A silent exit status does not answer that question.

Every one of these results was got from a Blender started with no window and driven by a script handed in at launch, which is exactly the case Smoozy renders in.

**What is not established, and it is the same in all five versions.** A driver or a keyframe carrying a count out of range; what happens when a file holding such a count is loaded rather than assigned; what a field does when the number is typed or dragged; and Windows, and for 5.3 Linux. All four are in *What is not established* at the top.

---

## 19. What settings does a scene or a view layer carry that govern denoising, and what does each of them do?

*On the same two machines named under question 18 for 4.5 to 5.2, except where an answer names one of them alone, and on the Mac alone for 5.3.*

**How it is known.** Cycles' settings and defaults are **observed** in its own files; many descriptions are **documented** in the manual. The shared settings and EEVEE settings are **documented**, including the 5.3 view-layer additions; DLSS is a **documented**, untested release-note addition. How two switches combine, whether a control is dimmed, and whether hidden settings keep their values are **inferred**, then **observed** in tests. Prefilter and Quality under OptiX are settled by tests on the RTX 3060 machine for 4.5–5.2 only. Other tests use that machine and the Mac as named below; 5.3 uses the Mac alone.

**Verdict: it differs** — in several things. Everything about how denoising is switched on, what governs it, and what a render does with it is identical in every version, as far as each was tested. What differs is how many denoising settings a view layer carries — for Cycles in 5.2 and again in 5.3, and for EEVEE from 5.3; how many feature passes are stored and in what order; how the compositor's Denoise node carries a quality of its own; one setting and one denoiser that 5.3 adds for the viewport; and one thing about EEVEE's view layer in 4.5 that is not about denoising at all.

### Where the settings live

Five places in 4.5 to 5.2, and both machines gave the same list in every one of them; six in 5.3, read on the Mac:

- **Cycles, on the scene, for the final render** — six settings, in all five versions: whether to denoise at all (on to begin with); which denoiser; which extra passes the denoiser is fed (albedo and normal to begin with); the prefilter (accurate); the quality (high); and whether the denoising itself runs on the graphics card (**off** to begin with).
- **Cycles, on the scene, for the viewport** — the same family again with its own values, seven of them in 4.5 to 5.2 and eight in 5.3, including one the final render has no equivalent of, the sample at which viewport denoising begins. **5.3's eighth is an upscale quality** — Viewport Denoising Upscale Quality, balanced to begin with, chosen from none, quality, balanced, performance and ultra performance — which 5.2.2 on the same Mac has not got. Two of its starting values run the other way from the render's, in all five: viewport denoising is **off** to begin with where the render's is on, and denoising on the graphics card is **on** for the viewport where it is off for the render.
- **Cycles, on each view layer** — whether that layer is denoised (on to begin with), and whether the denoising feature passes are stored (off to begin with). **In 5.2, two more, and in 5.3 three more**; see below.
- **On the scene, whatever the engine** — three settings governing Denoise nodes in the compositor rather than the engine's own denoising: which device processes them, the quality used for a final render (high), and the quality used in the viewport (balanced). The same in all five.
- **EEVEE, on the scene** — one switch called "denoise using temporal reprojection", and four more inside EEVEE's ray-tracing options: denoise at all, spatial reuse, temporal accumulation, and a bilateral filter. All five start on in every version, and in the four earlier versions the ray tracing they belong to starts off.
- **EEVEE, on each view layer — in 5.3 alone** — two settings: Denoising Data, which delivers the denoising passes (off to begin with), and a roughness weighting for the albedo pass (on). See below.

**In 4.5 to 5.2 a view layer carries nothing about denoising outside Cycles**: EEVEE's own view-layer settings hold no denoising setting anywhere, and Workbench has none at all, so in those four the view-layer half of this question is answered by Cycles alone. **In 5.3 EEVEE's view layer carries the two above.** No version's record gives Workbench a denoising setting.

**The per-view-layer denoising switch is in no earlier version's manual.** It exists, it is drawn in the view layer's own panel, and every earlier version's manual lists that panel's contents without it. Its description comes from Cycles' own files in all five versions.

### Cycles denoises only when both switches are on

This combines two switches whose joint effect is **inferred** from their descriptions and **observed** in the tests below. In all four earlier versions on both machines, and in 5.3 on the Mac, the same frame was rendered in all four combinations of the scene's switch and the view layer's and the pictures compared channel by channel, with one picture against itself as the control:

- **Both on** gives a different picture from both off — some six thousand channels of 12,288.
- **The scene's switch alone** gives a picture identical, to the last channel, to both off.
- **The view layer's switch alone** gives a picture identical, to the last channel, to both off.
- **The control was zero** in every version on every machine.

So **either switch alone denoises nothing**. The "noisy image" pass appears under exactly the same condition — both on, and only both on. **Storing the denoising feature passes is gated by neither**: those passes appear whenever that setting is on, whatever the two switches say. All of that is the same in all five versions.

The two machines gave slightly different figures for the two non-zero comparisons, because two different processors trace slightly different noise. **The three zeros and the control zero are identical on both, in every version, and in 5.3.**

### The switch a person sees is dimmed, not cleared

With the scene's denoising off, the view layer's own denoising checkbox is drawn dimmed — grey label, muted box — and **the tick stays in it**. Turning the scene's switch back on restores it to full brightness. Nothing else on the screen changes: the two captures differ in one small block of pixels and nowhere else.

**Observed in all four earlier versions on the rented Linux machine, and in 5.3 on the Mac's own screen.** The Linux machine had no display server as rented and one was installed on it in software for this test. This observes software drawing only; it does not establish how another display would draw the control. 5.3's run on the Mac's own screen showed: one checkbox and its label dimmed, still ticked, and nothing else changed. Neither machine ran the other's versions, so the two halves stand side by side and are not set against each other.

### The prefilter and the quality settings do nothing under another denoiser — which no version's documentation says

**Two claims, and they must be kept apart.**

**What every version's manual says** is that these two settings are *"visible only when using OpenImageDenoise"*. That is a statement about what is drawn on a screen. Cycles' own interface file agrees and no more: it draws those two rows only under a test for the chosen denoiser. **No version's documentation says anything at all about what the settings do when another denoiser is in use** — not that they are ignored, not that they still apply, not that they are cleared.

**What the runs say** is separate and stronger. With OptiX chosen, moving the prefilter from none to accurate and the quality from fast to high changes **not one channel of 27,648** — in all four earlier versions. With OpenImageDenoise chosen, the same change moves about ten and a half thousand channels. Two identical runs compared against each other came back the same to the last channel, so a difference would have been seen.

**So the two settings are not merely hidden under another denoiser: they have no effect at all** — in 4.5 to 5.2. The documentation carries the first half and nothing more; the second half is settled by test and by nothing anybody could read.

That test needed two denoisers on one machine, so **it was run on the rented Linux machine alone**, in all four earlier versions. The Creator's Mac offers one denoiser and could not have run it, so **it is untested in 5.3**.

### The compositor's own denoise quality changes a render, under both engines

Switching Cycles' own denoising off and leaving a Denoise node as the only thing denoising, the scene's "quality for a final render" setting moved from fast to high changes the picture — **under Cycles and under EEVEE alike**, in all four earlier versions on both machines and in 5.3 on the Mac, against a control of zero. So it is engine-independent by test rather than by reasoning.

**One figure from that test must not be quoted as exact.** The count of differing channels is about nine thousand of 27,648 under Cycles and about three and a half thousand under EEVEE, and the Cycles count **is not reproducible in 4.5, 5.0 or 5.1** — run forty-eight times across the four earlier versions, it moved over two, three or four neighbouring values depending on which of those three it was, and **did not move at all in 5.2**, nor in 5.3's twelve rounds on the Mac. The wobble arrives with a rebuilt scene rather than with a repeated render: one scene file held still and rendered six times each way gave the same count every time, which was tried in 5.0 alone, and 5.3's twelve rounds all rendered one saved file. Through all forty-eight rounds the control stayed at zero and the largest difference between the two pictures never moved. **The verdict is untouched — a change of nine thousand channels against a control of nothing is not in doubt — but the last digit of that count is not a number to build on.**

### A file naming a denoiser this machine has not got still renders, and is still denoised

In 4.5 to 5.2 a file was made on the machine with an NVIDIA card, with its denoiser set to OptiX, and carried to the Mac, which offers no OptiX. 5.3 had no such machine, and the Mac refuses the name, so the same stored value was written into a file directly on the Mac; that is the one difference in the set-up. In all five versions:

- **The render goes ahead.** Blender does not refuse and does not stop; the picture is saved.
- **The picture is still denoised** — it differs from the same file with denoising switched off by nearly fourteen thousand channels of 27,648. Blender falls back to a denoiser the machine has rather than doing nothing.
- **The setting reads back as empty text.** A program reading the denoiser out of such a file gets nothing at all, not the name that was stored.
- **A warning is printed, and only when a script reads the setting.** Nothing about the mismatch appears in the render's own output, so a run that never asks says nothing whatever about it.

### What denoisers a machine offers is a fact about the machine

Asked in all four earlier versions on both machines, and **the answer is the same in all four versions and different on the two machines**: the Mac offers OpenImageDenoise alone, and the Linux machine with an NVIDIA card offers OptiX and OpenImageDenoise. **That difference follows the card**, as far as two machines can show, and no machine with an AMD or an Intel card was available. 5.3, on the Mac, offers OpenImageDenoise alone for the final render and Automatic and OpenImageDenoise for the viewport — no OptiX and no DLSS — and choosing Metal in the preferences changed nothing about it.

**AMD support is documented differently and untested here.** The 4.5 and 5.0 manuals tie GPU denoising to two named card generations; 5.1 says one generation “and newer”; 5.2 and 5.3 split the claim by operating system. For 5.3 the stated minimum is discrete Radeon RX 6000 on Linux and RX 7000 on Windows, each “and newer”. No test used an AMD card, so the actual support and any change between versions are not established.

Two things about that list correct what every earlier version's manual says, and they are the same correction in all four:

- **Having the card is enough.** Every version's manual ties OptiX to an NVIDIA card *configured in the preferences*. On the machine with such a card, OptiX was already offered before any device type had been chosen, and switching devices on changed nothing. Untested in 5.3, which had no such card.
- **"Automatic" is never a choice for the final render.** Every version's manual lists it under a heading covering both the viewport and the render. It is the first viewport choice on both machines and it is not a render choice on either, in any of the four — nor in 5.3 on the Mac. Cycles' own files are right here and the manuals are not.

### What differs, version by version

**The number of denoising settings a Cycles view layer carries: two in 4.5, 5.0 and 5.1, four in 5.2, five in 5.3.** Blender 4.5, 5.0 and 5.1 give a view layer two: whether to denoise it, and whether to store the feature passes. **Blender 5.2 gives it four**, adding one about following reflections and one about weighting the albedo by roughness, both starting on. Read in all four earlier versions on both machines, so the two are genuinely absent from the earlier three rather than merely unrecorded there. In 5.2 those two are drawn only when two developer preferences are switched on — but **the preference governs only whether they are drawn**: with both off, both settings read back their starting value and both accepted a new one. **5.3 has the same two**, and on the Mac they behaved the same way with the developer preferences off. **5.3 adds a fifth**: Store Denoising Motion Passes, "Store the denoising motion passes, for temporal denoisers", off to begin with. The same listing in 5.2.2 on the same Mac gives four and not this one. What any of the three extra settings does to a picture was never measured, in any version.

**5.3 adds one viewport denoising setting and one viewport denoiser.** The viewport upscale quality above is in 5.3 and not in 5.2.2 on the same Mac. And 5.3's release notes say a new DLSS denoiser is available for the viewport on NVIDIA cards, which Cycles' own files list as a viewport choice; the Mac does not offer it, so nothing ran it. Both belong to the viewport. That viewport denoising settings are ignored in a render with no window is **inferred**, not established by a test; the two additions were not tested for that effect.

**5.2 and 5.3 store five denoising feature passes where the other three store three, and the three are not in the same order everywhere.** With the storing setting on:

| | The feature passes stored |
|---|---|
| 4.5 | Denoising Normal, Denoising Albedo, Denoising Depth |
| 5.0 | Denoising Normal, Denoising Albedo, Denoising Depth |
| 5.1 | Denoising Albedo, Denoising Normal, Denoising Depth |
| 5.2 | Denoising Albedo, Denoising Specular Albedo, Denoising Normal, Denoising Roughness, Denoising Depth |
| 5.3 | Denoising Albedo, Denoising Specular Albedo, Denoising Normal, Denoising Roughness, Denoising Depth |

Read three times over in each version, the same every time — 5.3's on the Mac. **5.2's and 5.3's manuals name all five**; the earlier three's manuals name two of their three — Denoising Albedo and Denoising Normal — beside the image before denoising, and never Denoising Depth. **Anything picking a pass out by its position rather than by its name will pick the wrong one.** Which passes appear, and under what condition, does not change: the feature passes appear whenever the storing setting is on, in every version.

**How the compositor's Denoise node carries a quality of its own.** In 4.5 the node has a quality setting and it starts at "follow the scene". In 5.0, 5.1, 5.2.1 and 5.3 the tested node has no `quality` property. **In 5.3, on the Mac, the node carries a Quality input** among its six inputs — Image, Albedo, Normal, HDR, Prefilter and Quality — reading "Follow Scene" on a new node; and **5.2.2, run on the same Mac, carries the same six inputs**, Quality among them, reading the same. No test in 5.0, 5.1 or 5.2.1 checked the node's inputs — those runs checked only for a setting on the node and found none — so whether their node (or the node on Linux in 5.2.2 and 5.3) carries that input was not tested. **So the condition every version's documentation attaches to the two scene-level quality settings — that they apply "if the node's quality option is set to Follow Scene" — describes a real choice in 4.5, as a setting on the node, and in 5.2.2 and 5.3 on the Mac, as an input to it.** Every test behind the quality result above used a new node, so the choice started at Follow Scene throughout; what a render does with it set to anything else was not tried in any version.

**EEVEE's view layer carries three settings in 4.5, four in 5.0, 5.1 and 5.2, and six in 5.3.** In 5.0 to 5.2 the extra one is about ambient occlusion distance, and **none of the earlier versions' settings is about denoising**; that was read in all four on both machines. **5.3's two extra ones are both about denoising** — the Denoising Data and albedo-weighting settings under *Where the settings live*. 5.3's API change log lists both as added between 5.2 and 5.3, and the same listing in 5.2.2 on the same Mac gives four settings and neither of them. What they do to an EEVEE render was not tested.

**One difference bounded to a single machine, met beside another test.** On the rented computer with an RTX 3060 on driver 550.120, asking Cycles to trace the picture itself on the card under OptiX worked in 4.5, 5.0 and 5.1 and **failed outright in 5.2.1**, which reported that it could not load its OptiX kernel and wrote no image. **This is a result about one card on one driver and not a statement about Blender 5.2 anywhere** — whether it happens on any other machine is untested, in any version, and 5.3 had no NVIDIA card to try it on. It is recorded because a render farm choosing a driver would want to know it can happen. Nothing else in this answer depends on it: the prefilter test kept the tracing on the processor and only the denoising on the card, which is why it ran in all four earlier versions there.

### What is not established

Every one of these is in *What is not established* at the top: what denoisers an AMD or an Intel card offers, and whether the split by operating system that 5.2's and 5.3's manuals give for AMD is real; what denoising on the graphics card does when a machine has more than one; whether 5.2's two extra view-layer settings, or anything 5.3 adds, change a picture; whether the Denoise node of 5.0, 5.1 and 5.2.1 (or on Linux in 5.2.2 and 5.3) carries a Quality input, and what a render does when the node's quality choice is set to anything other than following the scene; whether EEVEE's ray-tracing denoising settings bear on a final render at all; whether Cycles' own denoising and a Denoise node interact when both are on; whether the prefilter and quality settings do anything under OptiX in 5.3; and Windows, and for 5.3 Linux and any NVIDIA card.

**Multiple graphics cards were never tested for denoising.** The GPU-denoising description speaks of devices in the plural; which cards it uses, and what happens when only some are enabled, remain open. A result from one NVIDIA card or the Mac's own graphics does not settle that case.

**One thing about names, and it is the same in all five versions.** Blender itself calls the temporal-reprojection switch "Viewport Denoising", and every earlier version's manual calls it "Temporal Reprojection", as does 5.3's Sampling panel. Both names are for one setting, and anything matching on the label has to know it.
