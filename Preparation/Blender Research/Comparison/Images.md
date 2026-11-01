# Images — Blender 4.5, 5.0, 5.1, 5.2 and 5.3 side by side

What pictures a render produces, what file formats they can be written in, what a written file carries about what produced it, and what comes out when a picture a file points at is missing.

## The machines behind the tests

**The Creator's Mac.** A MacBook Pro with an Apple M1 Pro chip, its own 14-core Metal graphics, 16 GB of memory and its own screen. The earlier tests used macOS 26.5.2 and Blender 4.5.12, 5.0.1, 5.1.2 and 5.2.0 LTS.

**The rented Linux machines.** Questions 1 to 8 used an Ubuntu machine with one NVIDIA GeForce RTX 4090 and a display. Question 9 used one with an RTX 5060 and no display server. Question 10 used one with two RTX 4000 Ada cards and a display server, but its graphics render used one card. These are separate machines; their figures cannot be compared as version results. The earlier findings identify the tested builds as 4.5.12, 5.0.1, 5.1.2 and 5.2.0 LTS.

**5.3 ran on the Mac alone**, under macOS 26.6.2, in **Blender 5.3.0 Beta**, built 2026-10-16, hash `5d8151335b6b`. Its controls used **5.2.2 LTS**, hash `d13f752e3b9c`, on that same Mac under the same conditions. A difference reproduced by those controls is a version difference on that Mac. Whether the released 5.3 behaves as the beta did is not established.

**The window checks have narrower coverage.** The Output panel under questions 3 and 8 was seen on the Mac only. In 4.5 to 5.2, multi-view files and the Image Editor's own View menu were checked on Linux only; in 5.3 they were checked on the Mac only. Those readings do not establish a match or difference between 5.3 and the earlier versions on one system. The Layer and Pass selectors were checked on both systems in the earlier four versions and on the Mac in 5.3.

**Every observation settles its tested machines only.** No Windows machine ran any version. No Linux machine or NVIDIA card ran 5.3. AMD, Intel and two-card rendering were not tested. OpenEXR tag names also depend on the reader: the Linux machine used exiftool 12.40, the earlier Mac tests 13.50, and the 5.3 tests 13.55.

---

## The answer in short

These lists give the short answers. Each numbered question below holds the evidence, conditions and remaining limits. Every 5.3 observation is from the Mac and the beta described above.

### Where Blender itself differs between versions

- **1 and 5** — `Emit` and `DiffCol` become `Emission` and `Diffuse Color` at 5.0, including 5.3; `Depth` stays the same.
- **1** — the 5.0 release note also renames `IndexMA` to `Material Index`; its written channel spelling is untested.
- **1 and 9** — the compositor's output node and scene settings change at 5.0; 5.3 adds an effects stack and changes the default group, while retaining the older setting with a warning.
- **2 and 9** — missing-texture console messages change at 5.0, 5.1 and 5.2; 5.3 changes EEVEE's file identification, empty-slot reporting and unreadable-file reporting, confirmed against 5.2.2 on the Mac.
- **2** — the AMD-library message is debug output in 4.5 and a warning from 5.0 in the earlier Linux tests; its 5.3 Linux behaviour is untested.
- **2, 4 and 7** — ordinary console lines gain timestamps and channels at 5.0, including 5.3.
- **3** — AVIF arrives at 5.1 and remains in 5.2 and 5.3; the tested 5.3 build also adds two non-image build options.
- **3** — Media Type and its required setting order arrive at 5.0, including 5.3.
- **3** — the format order and labels change at 5.0, and video gains a Color Space row; the panel observations are Mac-only, including 5.3.
- **3** — 5.3's release notes announce improved multi-layer OpenEXR reading and writing with lossy DWA compression; the precise effect is untested.
- **4** — MPEG-4 with FFV1 fails in 4.5 to 5.1 and writes a file in 5.2 and 5.3; the test ran in every version, with 5.3 on the Mac only.
- **1 and 5** — Asset Shelf is available in the Image Editor's View menu in 4.5 and greyed in 5.0 to 5.2 on Linux; 5.3's greyed Mac entry is not a same-system comparison with those readings.
- **5** — multi-layer OpenEXR changes from one part to several at 5.0, including 5.3, with Interleave restoring one part; multi-view naming changes at 5.0 on Linux, while 5.3's Mac reading stands separately.
- **5 and 8** — File Output naming and the fresh node's format change at 5.0, including 5.3; naming depends on the node's mode.
- **5, 8 and 10** — a File Output node fed from Group Input writes in 5.3 and not in 5.2.2 under the same Mac tests; the written recipe also produced no file in 5.0 to 5.2 on both earlier machines, with a conflicting result left open under question 10.
- **6** — the working colour space and OpenEXR `colorInteropID` arrive at 5.0, including 5.3.
- **6** — PNG gains `sRGB`, `gAMA` and `cHRM` blocks at 5.1, including 5.3.
- **6 and 10** — 5.3 adds an sRGB ICC profile, and its PNG also gains `cICP`; its release note and the Mac control support the change.
- **6** — 5.3's release notes announce improved display colour interop IDs in OpenEXR; the precise effect is untested.
- **7** — sequencer-scene render options arrive at 5.0 by release note; their presence in 5.2 and 5.3 is not established here.
- **8** — Scale node controls and the second copy's reported depth and floating-point status change at 5.0, including 5.3.
- **9** — EEVEE's engine name changes at 5.0, including 5.3; which names are accepted also depends on how they are supplied.
- **9** — disabling world nodes stops taking effect at 5.0, including 5.3, and the tested Volume object's dimensions change at the same boundary.
- **9** — colour-to-number conversion differs in the fifth decimal place between 4.5 and 5.0 onward, including 5.3; measured on the processor only, with nothing to build on the small gap.
- **9** — Workbench's alpha differs slightly between 4.5 and the later versions on the Mac only, including 5.3; Linux does not show that version split.
- **9** — 5.2's release notes add three file-enumeration options and name the new Cycles texture cache; their continued presence in 5.3 is not established here.
- **10** — OpenEXR gains an unconditional `Software` version attribute at 5.0, including 5.3; multi-part files keep it and the fourteen render fields in part 0 only.
- **10** — Render Time is absent in 4.5 and available on the processor from 5.0, including 5.3; it is absent from the tested graphics renders.
- **10** — peak-memory text changes from two decimals to whole numbers at 5.0, including 5.3.
- **10** — stereo movie names lose `_L` at 5.1, including 5.3.
- **10** — the tested MPEG-4 encoder line changes with the bundled FFmpeg library at 5.2; 5.3 matches 5.2.
- **10** — 5.2's release notes announce stereo and panoramic movie metadata export; the tested MPEG-4 files did not reveal the announced markings.

### What is not established

- **Every question** — Windows in every version; Linux and NVIDIA in 5.3; other graphics makes and two-card rendering; the released 5.3 rather than this beta.
- **1** — the full pass list for 5.2, untested written pass renamings, extra Cycles passes, pass-name translation, several 5.3 compositor effects, and the 9000-pixel graphics-compositor result across versions.
- **1 and 5** — the open View picker in 4.5 to 5.2, and same-system comparisons of 5.3's multi-view file and View menu against those earlier versions.
- **1 and 5** — the composited part in earlier versions; 5.3's `Composite.Combined` was measured on the Mac alone.
- **2 and 9** — which unreadable files 4.5 names, and whether the earlier graphics-texture line appears in every matching Mac run; captures and claims of its absence disagree.
- **2** — separate-pass texture results before 5.3, the output stream for 5.3's EEVEE message, and its Linux behaviour.
- **3** — untested command-line names, DDS output, the heading checkbox in earlier versions, and the precise effect of the announced 5.3 DWA improvement.
- **4** — a blend-file asking for a real format omitted from its Blender build; the official builds tested all carry the optional formats.
- **4** — refused pairings in earlier versions' windows, unknown format values from newer files, and inconsistent refusal totals for 4.5 to 5.1.
- **5** — other formats' ability to hold several pictures, Interleave's effect on naming, several view layers and views together, an older reader's chosen view, and earlier versions' per-part `view` attribute.
- **6 and 10** — ICC embedding in 4.5 to 5.2 under a configuration requesting it; the default tested files carried none.
- **6** — HDR output beyond the tested 5.3 8-bit PNG and AVIF, empty interop identifiers, readers lacking the writing colour space, the precise 5.3 interop improvement, pixel effects of treatment, the Output colour override and Wayland's effect on saved files.
- **7** — length limits, placeholder colours, a missing frame in a movie file, animated single-picture output, the player without a window in 5.2, and the later sequencer-scene options.
- **8** — Preview quality, its colour treatment before 5.3, an ordinary window still's hand-saved preview before 5.3, whether 5.3 requires the editor drawn before Save As, and other routes to background proxies.
- **9** — a tile waiting for memory, an image record without pixels, the missing-files report in a window, legacy Texture data-blocks, metallic, roughness before 5.3, 4.5's volume-node property, and the texture-cache additions' continued presence in 5.3.
- **10** — Sequencer-strips metadata, the File Output node's conflicting result in 5.0 to 5.2, and density in later OpenEXR parts in 5.0 and 5.1.
- **10** — Preview metadata across versions, Cycles fields per view layer in 5.0 to 5.2, Cryptomatte manifest differences, other video containers' metadata, and where the announced stereo and panoramic markings are written.

### Where only the documentation differs

- **1 and 5** — 4.5's manual says Z where the other quoted manuals say Depth; the tested file and menu name is `Depth` in all five versions.
- **3** — 4.5's format table groups rows that later manuals separate; the tested format choices are separate in all five versions.
- **9** — the names and locations of Cycles' missing-image values differ in the supplied files, but the tested surface substitution stays magenta; 5.3's source reading alone leaves that value open.
- **10** — the recorded peak-memory `ddM` fault note concerns 5.0 and 5.1; every tested build writes a real figure.

---

## 1. What kinds of picture can a render produce, and how can the finished picture be told apart from the others?

**How it is known.** The pass kinds and Combined's meaning are **documented**; Cycles' literal names were **observed in its files**. Some identifications are **inferred** from the sources: Combined as the finished picture in 4.5, the single output as the final picture in 5.2, and exact-name matching and denoising in 5.3. The file and menu results below are **observed in tests** on the Mac and Linux in 4.5 to 5.2, and on the Mac alone in 5.3. The compositor changes also have release notes.

**The same in all five.** A render produces a set of separate pictures called **passes**, one set per **view layer**, and — when stereo is on — one set per **view**. The finished picture is the pass named **Combined**, described in every version's manual as "the render output before any compositing is applied".

Three results came back identically in all five versions:

- With two view layers and a plain single-image output, **one file is written, not one per view layer**. Its centre carries the lit cube and its corner carries the world.
- With compositing switched on and a plainly different colour fed to the compositor's output, **what lands in the file is the compositor's result, not the raw Combined pass**. Measured at 32 by 32. The earlier tests used the compositor on the processor; 5.3 tested both compositor routes without establishing a comparison between compositor devices. At 9000 pixels wide with the compositor on the graphics card, Blender 5.1 wrote a black frame instead — neither the compositor's colour nor the render's — on two machines and two makes of card (*The compositor 1*).
- In the Image Editor, a person picks the finished picture by choosing `Combined` in the menu labelled `Pass`, within the view layer chosen in the menu labelled `Layer`.

**The Image Editor's menus, and the one that does not choose a picture.** With an ordinary multi-layer OpenEXR open, the Image Editor's header holds **two** picture-choosing menus in all five versions — `Layer` and `Pass` — and no third one at all. A third menu appears only for a file holding several views, and only with the stereo button switched off; that is question 5's ground. Separately, the Image Editor has its own menu called `View` in the header beside `Image`, and it is a different thing entirely: it holds `Toolbar`, `Sidebar`, `Tool Settings`, `Asset Shelf`, `Adjust Last Operation`, `Update Automatically`, `Show Metadata`, `Frame All`, `Center View to Cursor`, `Zoom` and `Area`, the same eleven entries in all five versions, and nothing in it chooses which picture is shown. `Center View to Cursor` is greyed out in all five; `Asset Shelf` is greyed out in 5.0, 5.1, 5.2 and 5.3 and not in 4.5. **That menu was photographed on the Linux machine only in the four earlier versions, and on the Mac only in 5.3**, so 5.3's reading is not set against theirs.

**Where it differs, version by version.**

**Blender 4.5.** Written into a multi-layer file, the passes are named `Combined`, `Depth`, `Normal`, `Mist`, `Emit`, `DiffCol`. The Image Editor's Pass menu shows the same six names. The compositor's final output node is the **Composite** node.

**Blender 5.0, 5.1 and 5.2.** The same six passes are named `Combined`, `Depth`, `Normal`, `Mist`, `Emission`, `Diffuse Color` — `Diffuse Color` with a space — both in the file and in the Pass menu. Blender 5.0's release notes state the change: "Many render passes were renamed to avoid obscure abbreviations." The compositor's final output node is the **Group Output** node; 5.0's release notes state that the Composite node was removed and replaced by it.

**Blender 5.3.** The pass names are 5.0's, in the file and in the Pass menu, and the final output node is still the Group Output. **What changed is how the compositor is reached.** 5.3's release notes: "The compositor now uses an effects stack instead of a single compositor node group", and "The default compositor node tree now uses a simple **Group Input** connected to a **Group Output** with no **Render Layers** node." In the test, a node group put into the stack — `scene.compositor_effects`, each entry holding its own `node_group` — reached the file, and so did the same group assigned to the older `scene.compositing_node_group`, which printed `DeprecationWarning: 'Scene.compositing_node_group' is expected to be removed in Blender 6.0`. The two routes are one thing seen two ways: with a group in the stack, the older setting reads back that group, and setting the older one with the stack empty leaves the stack holding one entry named `Scene Effect`. In a multi-layer file, 5.3 writes the composited picture as a part of its own — question 5.

**Depth differs in the manuals, not in the tested name.** The 4.5 manual calls it Z; the quoted 5.0, 5.1 and 5.3 manuals call it Depth. The 5.0 release note also announces Z to Depth. Yet the file channels and Pass menu read `Depth` in every tested version, including 4.5. No version-specific written depth name follows from those tests.

**A further documented rename at 5.0.** The release note changes `IndexMA` to `Material Index`. Its spelling in a written file was not tested; the announced rename does not depend on a test.

**Verdict: it differs.** Settled by the same tests run in the four earlier versions on the Creator's Mac and again on the rented Linux machine, which agreed on every pass name and every pixel, and by 5.0's own release notes; 5.3 by the same tests on the Mac alone and by its own release notes. Windows is untested everywhere, and Linux untested for 5.3.

**Small numbers wobble between 4.5 and the later versions, and nothing is read into it.** Several tests came back with pixel values differing in the fourth decimal place — a rendered centre pixel of 0.8039 in 4.5 against 0.8078 in the other four, and 0.2609 against 0.2599 in 5.0 to 5.2. Written file sizes also differ version by version for the same scene, and 5.2's OpenEXR files are consistently the largest of the first four (a 640 by 360 render came out at about 101,000 bytes in 5.2 against about 57,000 in the other three; 5.3's came out at about 96,000). None of that was investigated by anybody, and none of it changes what any test showed.

**Not established.**

- **The complete list of passes.** Not established for 5.2. The documented lists for 4.5, 5.0, 5.1 and 5.3 agree apart from the depth name and the Debug group: Sample Count alone in 4.5, Render Time and Sample Count in the other three. Question 10 gives the measured debug-pass results; the Sample Count channel is `Debug Sample Count` in all five.
- **The exact written material-index name.** The 5.0 release note establishes the rename from `IndexMA` to `Material Index`, and 5.3's manual lists Material Index. No index pass was rendered, so the written channel name in any version remains untested.
- **Extra Cycles passes.** Volume Scatter, Volume Transmit and Volume Majorant switches were observed in the supplied 5.2 and 5.3 files. The 5.3 files also name Noisy Shadow Catcher, Denoising Backward Motion, Denoising Specular Motion and three debug-build guiding passes. These were not rendered, and their presence in 4.5 to 5.1 is not established. The source coverage does not establish additions.
- **What the View picker holds when it is open.** Settled for 5.3 on the Mac — question 5. Its open contents remain unestablished in 4.5 to 5.2 on Linux.
- **The composited picture's name inside a multi-layer file**, which 5.3 writes as `Composite.Combined` and no earlier version was looked at for — question 5.
- **How 5.3's effects stack picks the final picture when it holds more than one effect.** Not in 5.3's sources; every 5.3 test used one effect.
- **The 9000-pixel graphics-compositor result across versions.** The 5.1 black-frame observation above does not establish the result in the other versions.
- **Whether the pass names change with the interface language.** 5.3's reading of Cycles' files finds the names wrapped in a marker its API documentation calls a no-op, and infers they stay English whatever the language. No version tested it.

---

## 2. Do any of the pictures a render produces show whether a scene's textures loaded, and if so, which?

**How it is known.** A texture-loading pass is **not in the sources** in any version. Magenta was **observed in Cycles' files**, but its visibility in a render is **inferred** from the source reading. In 5.3 those files alone leave the fallback open: the nodes receive their missing colour from outside the supplied code, and the compiled Open Shading Language shader defaults to black. The picture results below are **observed in tests** on the Mac and Linux in 4.5 to 5.2, and on the Mac alone in 5.3. Separate-pass results are 5.3 Mac observations only. The Sequencer's separate Show Missing Media option is **documented**.

**The same in all five.** The tested pictures behave alike. No documented pass reports texture loading; the findings establish no addition of such a pass. What a render shows is this, and the numbers came back the same in every version and on every machine that ran it:

- **File present** — the texture's own colour, in Cycles on the processor, Cycles on the Metal graphics, EEVEE and Workbench alike.
- **File missing** — magenta, in Cycles on both devices and in EEVEE. Workbench shows a dark magenta; with its lighting set to Flat as well as its colour to Texture it shows the same magenta as the others, which is question 9's ground.
- **File present but unreadable** — magenta, exactly as for a missing file, in every engine.
- **Image slot left empty** — this is the one case the engines part company on, and they part the same way in all five versions: Cycles still shows magenta, EEVEE shows black, Workbench shows plain grey.

Nothing in any version says **which** texture is missing from the picture alone. The magenta is one flat colour with no identity in it, and nothing distinguishes a missing texture from a genuinely magenta one.

**The card makes no difference to the picture.** The substituted magenta came back identical on the processor, on the NVIDIA card through CUDA, on the same card through OptiX, and on Apple's own Metal graphics, in every version and every case that machine ran. **Workbench is the one small departure**: its alpha reads `1.0` on the Linux machine and a shade under `1.0` on the Mac, in all four earlier versions, and 5.3's Mac figure is the Mac's figure again. That is a difference between the machines, not between the versions. The Mac's own figure differs slightly between 4.5 and the other four; question 9 gives both machines' figures in full.

**Where it differs, version by version.** Only in what Blender prints. The picture is the same everywhere; the console is not.

**Blender 4.5.** On a **Cycles** render, a missing picture prints nothing that separates it from the same render with the file in place: the frame is written, the run ends at 0, and standard error is empty. The picture's file name does appear, in the ordinary progress line `Updating Images | Loading <name>`, but that line is printed whether the file is there or not, in every version, so it identifies nothing. On an **EEVEE** render, 4.5 does print one line — `GPUTexture: Blender Texture Not Loaded!` — and prints it only when the texture is missing; it carries no error marking and does not name the file, so it says a texture is missing and never which one. That the line is tied to the missing file was shown on the Creator's Mac and on the Norway machine, in all four earlier versions, against a control run first in the same shape — with nothing missing, standard error is empty in every version. It lands on standard error in 4.5 and on standard output from 5.0, in different words. Cycles prints nothing in 4.5, captured against the same control. Settled under **Reports 2 and 13**. A file that is present but **corrupt** is named in 4.5, by `IMB_load_image_from_memory: unknown file-format (<path>)`, where a missing one is not. An OpenImageIO complaint is among 4.5's lines as well, and which of the cases printed it was never written down.

**Blender 5.0.** The file is named, in the new log format: `cycles | ERROR Image file '<path>' does not exist.` and `cycles | ERROR Image file '<path>' failed to load.` EEVEE's line is `gpu.texture | ERROR Failed to create GPU texture from Blender image`, which names nothing.

**Blender 5.1.** The same lines, without quotation marks around the path. The empty-slot case prints `Image file  does not exist.` with nothing between the words.

**Blender 5.2.** The same lines again, but graded down to `WARNING`, with an added summary line: `cycles | ERROR Failed to load 1 image files`. EEVEE's `gpu.texture` line is unchanged, and follows the `image.read` line for a corrupt file as well.

**Blender 5.3.** Cycles prints as 5.2 does: `cycles | WARNING Image file <path> does not exist.` then `cycles | ERROR Failed to load 1 image files`, and for an unreadable file the `WARNING … failed to load.` line twice, then the same count. **Three things changed**, each against 5.2.2 run on the same Mac under the same conditions with the same script:

- **EEVEE's line names the file** for the first time in any version: `image.gpu | ERROR Image file "<path>" does not exist`, the path in double quotation marks. 5.2.2 printed `gpu.texture | ERROR Failed to create GPU texture from Blender image` in its place.
- **An empty image slot prints nothing at all.** 5.2.2 printed `cycles | WARNING Image file  does not exist.` and the count.
- **For an unreadable file EEVEE prints only** `image.read | ERROR IMB_load_image_from_memory: unknown file-format (<path>)`, with no `gpu.texture` line after it.

OpenImageIO's closing "pending error message … Not a PNG file" block is printed after a corrupt file by both 5.2.2 and 5.3.

The Linux machine adds one line of its own in the earlier versions: a warning that Cycles could not load a library for AMD graphics, of which that machine has none. That warning is not about the version and not about the missing texture: it appears whenever something makes Cycles look at its device list — writing the system-information file, a script that sets the device, a plain refresh — which this test does, because it sets the device before rendering, and it does not appear in an ordinary render at factory settings in any version. Blender 4.5 prints the same message at debug level rather than as a warning, so what differs between the versions is how loudly it is said, not whether. **The `gpu.texture` line is not established as Linux-only.** It was captured on the Mac in 5.0, 5.1 and 5.2, despite claims that it was absent there; 5.2.2 also printed it on the Mac. Whether it appears in every matching run is not settled. This conflict does not establish a difference between operating systems.

**Verdict: it differs** — in what Blender prints, and in nothing else. Anything that watches Blender's output for a missing texture has to be written per version. Anything that looks at the picture does not. **Settled by the same test run in the four earlier versions on both machines, which agreed on all four cases in every engine; 5.3's changes were settled on the Mac alone, against 5.2.2.**

**A texture driving roughness, a mask or a bump** is covered under question 9.

**Not established.**

- **Which separate passes carry the magenta.** Every earlier version's test read the finished picture only, on both machines. **In 5.3, on the Mac**, with the texture feeding a Principled surface's Base Color: in Cycles the Combined, Diffuse Color, Noisy Image and Denoising Albedo passes swap the texture's green for magenta, the Denoising Specular Albedo, Denoising Normal and Denoising Depth passes are unchanged, and Denoising Roughness moved slightly (0.9478 to 0.9725); in EEVEE the Combined and Diffuse Color passes swap. Untested in 4.5 to 5.2.
- **Whether the earlier graphics-texture line appears in every matching Mac run.** Its recorded Mac presence conflicts with claims of its absence, as above.
- **Which output stream 5.3's EEVEE line lands on.** It was not captured apart from the rest.
- **Whether 5.3's three changes hold on Linux.** Run on the Mac only.
- **Graphics cards of other makes.** The result covers this Mac's processor and its own Metal graphics in all five versions, and the Linux machine's processor and its NVIDIA card used both ways in the four earlier ones. AMD graphics, Intel graphics and two cards at once remain untested.

---

## 3. Which file formats can Blender write a render out as?

**How it is known.** The format families and choices are **documented**. Their availability in one build is not a promise about every build; 5.3 labels that step **inferred**. Lists and actual writes are **observed in tests** on the Mac and Linux in 4.5 to 5.2, and on the Mac alone in 5.3. Output-panel observations are Mac-only. Release notes also carry the named additions below.

**The same in all five.** Every installed build reports every optional image format compiled in — OpenEXR, OpenJPEG, TIFF, WebP, Cineon, Radiance HDR and FFmpeg all true, and the AVI codec false. Two families are on offer: still images written directly, and video written through FFmpeg. 5.3's build adds two build options, `io_spz` and `opentimelineio`, and neither is an image format.

**And the same on both machines, in the four earlier versions.** The Linux builds report the same formats, the same list in the same order, the same Media Type answer and the same three video lists, in 4.5 to 5.2. Two or three build options do differ between the machines, and **not one of them is an image format**: audio libraries, and in 4.5 and 5.0 a virtual-reality option. So the download for the operating system a render actually happens on carries every format the Mac's build carries — in those four versions. **5.3's Linux download was never read.** The virtual-reality option is true on both machines in 5.1 and 5.2.

The video lists are identical in all five versions, both as Blender reports them and as the menus show them:

- **Containers**: MPEG-4, Matroska, WebM, then AVI, DV, Flash, MPEG-1, MPEG-2, Ogg, QuickTime.
- **Video codecs**: No Video, AV1, H.264, H.265 / HEVC, WebM / VP9, then DNxHD, DV, FFmpeg video codec #1, Flash Video, HuffYUV, MPEG-1, MPEG-2, MPEG-4 (divx), PNG, ProRes, QuickTime Animation, Theora.
- **Audio codecs**: No Audio, AAC, AC3, FLAC, MP2, MP3, Opus, PCM, Vorbis.

Every version's manual carries the same warning, in the same words, that seven formats are build-time options "not available on all systems": OpenEXR, OpenEXR MultiLayer, FFmpeg, Cineon, DPX, JPEG 2000 and WebP.

**Where it differs, version by version.**

**Blender 4.5.** Fifteen names, in this order: `BMP`, `IRIS`, `PNG`, `JPEG`, `JPEG2000`, `TARGA`, `TARGA_RAW`, `CINEON`, `DPX`, `OPEN_EXR_MULTILAYER`, `OPEN_EXR`, `HDR`, `TIFF`, `WEBP`, `FFMPEG`. **There is no AVIF**, and asking for it by name is refused with `Error: Format from '-F / --render-format' not known or not compiled in this release.` **There is no Media Type control** — not in the Python interface and not in the Output panel. The panel has one row, `File Format`, and its menu entries carry no extensions (`BMP`, `Iris`, `PNG`, …). Under Video the rows are Video Codec, Color Depth, Output Quality, Encoding Speed, Keyframe Interval, Max B-frames.

**Blender 5.0.** Fifteen names, reordered: `JPEG`, `OPEN_EXR`, `PNG`, `WEBP`, `BMP`, `CINEON`, `DPX`, `IRIS`, `JPEG2000`, `HDR`, `TARGA`, `TARGA_RAW`, `TIFF`, `OPEN_EXR_MULTILAYER`, `FFMPEG`. **Still no AVIF**, and it is refused the same way. **A Media Type control exists**, with three entries — Image, Multi-Layer EXR, Video — sitting above File Format in the panel, and it comes first: with Media Type on Image, the format setting will take only the still-image names, and `OPEN_EXR_MULTILAYER` and `FFMPEG` are not among its choices. Blender 5.0's release notes state the change: media type must be set before the format. The File Format menu entries now carry their extensions (`JPEG (.jpg)`, `PNG (.png)`, …). Under Video there is a new `Color Space` row.

**Blender 5.1, 5.2 and 5.3.** Sixteen names — the same fifteen as 5.0 with **`AVIF` at the head of the list**. AVIF works both ways: `-F AVIF` from a terminal writes a real AVIF file, and the panel offers `AVIF (.avif)` with Color, Color Depth (8 / 10 / 12) and Quality. Blender 5.1's release notes state the addition: "Avif: New file format support." Everything else is as 5.0 — in 5.3 too, where the Media Type rule, the menus' labels, the Color Space row and the three video menus were seen again on the Mac.

**The format table changes its layout without changing these choices.** The 4.5 manual groups Cineon and DPX in one row and gives Targa one row. The later manuals list Cineon, DPX, Targa and Targa Raw separately. The tested builds offer `CINEON`, `DPX`, `TARGA` and `TARGA_RAW` as four choices in all five versions: on both machines in the earlier four, and on the Mac in 5.3.

**A further documented change in 5.3.** Its release notes announce improved reading and writing of multi-layer OpenEXR with lossy DWA compression. That establishes a change, while its precise effect remains untested.

**Verdict: it differs.** Settled by the same tests run in the four earlier versions on both machines, which returned the same format list and the same AVIF result in every version, and in 5.3 on the Mac; and by 5.0's and 5.1's release notes. What a panel shows — the Media Type row, the File Format menu's labels, the Encoding section's menus and the Color Space row — was seen on the Mac alone.

**Two traps in this question, both settled.**

- **Blender 4.5's own release notes announce a Media Type control that 4.5 does not have.** They say the image format settings "now has a Media Type enum". Blender 4.5's Python interface reports no such setting, and the panel shows no such row. The test settles it: Media Type arrives at 5.0, not 4.5. Here the release-note claim conflicts with the tested 4.5 build, which has no such control.
- **Blender 5.1's, 5.2's and 5.3's manuals leave AVIF out of the command-line page entirely** — it appears in neither the always-available list nor the may-not-be-compiled-in list — and yet `-F AVIF` works in all three. That is a gap inside those versions' own documentation, not a difference between versions.

**What those two video menus offer and what the encoder will actually take are two different things** — the full account is under question 4.

**Not established.**

- **What a build on Windows carries**, in any version, and **what the Linux 5.3 download carries.** All four earlier builds on the Mac and on the Linux machine have everything, and so does 5.3's on the Mac; that still says nothing about the case the manual is warning about, which needs a build that lacks something.
- **The `-F` command-line name lists.** Blender 4.5's manual lists `AVIRAW` and `AVIJPEG` among the always-available names; 5.0's, 5.1's, 5.2's and 5.3's do not. Neither name appears in any version's Python format list, and nobody tried either from the command line in any version on any machine. Whether Blender changed or only the page did is untested.
- **`image_dds=True`.** Every version reports it as a build option while DDS is nowhere among the formats a render can be written as. Untouched by any test, in every version.
- **The Output heading checkbox across versions.** Present in the 5.2 and 5.3 Mac panel observations. Its presence or absence in 4.5 to 5.1 is not established; a claim that only 5.2 has it lacks matching observations of those earlier versions.
- **The precise effect of 5.3's DWA improvement.** The release notes establish that multi-layer OpenEXR reading and writing with lossy DWA compression improved; the tests do not establish which files or results change.

---

## 4. What happens when a file asks for a format that Blender cannot write?

**How it is known.** A blend-file requesting a real format omitted at build time is **not in the sources** in all five versions. Format limits and compatibility warnings are **documented**; 5.3 labels the step from incompatibility to failure **inferred**. The neighbouring failures below are **observed in tests** on the Mac and Linux in 4.5 to 5.2, and on the Mac alone in 5.3. The refused pairing in a window was tested in 5.3 on the Mac only.

**The question's own answer: not established anywhere.** A blend-file naming a real format omitted from the running build was not tested in any version. Every optional format is present in the official builds tested on the Mac and Linux in 4.5 to 5.2, and on the Mac in 5.3. The outcome needs a build lacking one of those formats; these observations say nothing about such a build.

**The same in all five.** Three neighbouring cases behave identically in every version:

- **A format name the command line will not take.** Blender prints one line — `Error: Format from '-F / --render-format' not known or not compiled in this release.` — then carries on and writes the frame in whatever format the file already held. The exit status is `0`. This is the same for a nonsense name and for a real format the version lacks (AVIF in 4.5 and 5.0).
- **The same request from Python.** Setting the format to an unknown name raises a `TypeError` whose message lists every name the setting will take. The setting keeps its old value and the render proceeds normally.
- **A picture size the codec refuses.** With the MPEG-4 container, the H.264 codec and a 101 by 101 picture, every version reports `Error: width not divisible by 2 (101x101)`. The earlier four also record `Couldn't initialize video codec: Generic error in an external library`; 5.3's generic codec lines were not tied to a particular encode. **No file is written, and the exit status is still `0`.** That last point holds in all five and matters for anything that watches exit codes.

**And a fourth case: a pairing Blender itself refuses.** The test assigns all 170 container-and-codec pairings and then really encodes three frames for every container with H.264, and for the MPEG-4 container with every codec. **The same script ran on the rented Linux machine and the Creator's Mac in 4.5 to 5.2, and on the Mac in 5.3. Both machines' printed refusals and successful file sizes agree in the earlier four.** 5.3's twenty-six Mac encodes match 5.2's named outcomes.

What came back, the same in all five versions except where said:

- **The setting refuses nothing.** Every container still offers all seventeen codecs after it is chosen, and all 170 pairings hold.
- **The encoder refuses plenty.** The wording that answers the question is `Error: Could not initialize streams, probably unsupported codec combination`, and it came back for the WebM container with H.264, and for the MPEG-4 container with No Video, Flash Video, HuffYUV, ProRes, QuickTime Animation and Theora — and, in 4.5, 5.0 and 5.1, with FFV1 as well. So eight refusals of that kind in those three versions and seven in 5.2 and 5.3. A file of zero or a few hundred bytes is left behind each time.
- **Three further kinds of refusal concern the picture rather than the pairing**, and are the same in all five: `Error: Render width has to be 720 pixels for DV!` for the DV container with H.264 and for the MPEG-4 container with the DV codec, `Error: Invalid tile sizing: frame not large enough to fit specified tile arrangement.` for AV1, and `Error: ff_frame_thread_encoder_init failed` for DNxHD.

**Where it differs.**

**In the shape of the lines.** In 4.5 the failure and the save are printed plain; from 5.0, 5.3 included, they carry a timestamp and a channel, as `00:00.265  video.write      | ERROR Couldn't initialize video codec: …` and `00:00.758  render           | Saved: '<path>'`. Blender 5.0's release notes state the change: most of Blender moved to one logging system and the output format changed. One line is the exception in every version: the `-F` format error is printed plain, outside the log format, in all five.

**And in one pairing.** **MPEG-4 with FFV1 is refused in 4.5, 5.0 and 5.1, with the same wording as the other refusals, and writes a real file in 5.2 and 5.3** — about four kilobytes of it for three frames. Every other refusal held unchanged across all five versions. This was settled by the same encode run in the four earlier versions on both machines, and in 5.3 on the Mac. The pairing difference rests on observation.

**Verdict: not established for every version**, for the question as asked. No version's sources answer it, and the case was not produced on any tested machine. The two differences recorded above — the console-line change at 5.0 and the FFV1 pairing at 5.2 — are real, each established by the same test run in every version, and neither is an answer to this question.

**Not established.**

- Whether the same lines and the same exit status come back on **Windows**, and — for 5.3 — on **Linux**. The Windows gap sits directly on the question, since builds differing between systems is the manual's own reason the situation exists.
- **What a refused pairing shows in Blender's window.** In 5.3, on the Mac, a WebM with H.264 animation render started from the window puts `Could not initialize streams, probably unsupported codec combination` in the status bar beside a red error mark, logs the same words as an `operator | ERROR` line, and leaves a 264-byte `.webm` behind — the same words and the same size as without a window. No earlier version was run from the window.
- **The total refusal count in 4.5, 5.0 and 5.1.** Their full encode outputs record twelve refusals; another account gives ten pairing refusals plus three size refusals. Those totals disagree. The named FFV1 outcome agrees.
- What a newer blend-file's unknown format value does when opened in an older version. Untested in all five.
- Whether another build omits a documented format. The tested official builds do not.

---

## 5. Can a format hold several pictures inside one file, and if so, how can the finished picture be told apart from the others inside it?

**How it is known.** OpenEXR holding layers, passes and views is **documented**. Exact written names are **not in the sources**, with naming steps **inferred** in 4.5 and 5.3. The names and layouts below are **observed in tests** on the Mac and Linux in 4.5 to 5.2, and on the Mac alone in 5.3. Multi-view files were tested on Linux alone in the earlier four and on the Mac alone in 5.3.

**The same in all five.** OpenEXR is a documented format for this, in two senses — several layers and passes in one file, and several views in one file, the latter documented in every version as OpenEXR's alone. Selecting it differs by version only in that it needs the Media Type set first from 5.0 (question 3).

The naming came back the same in every version:

- **Each channel is named `<view layer>.<pass>.<channel>`.** The finished picture of a view layer called `Beauty` is `Beauty.Combined.R`, `.G`, `.B` and `.A`. That is the name a program outside Blender needs, and it is the same in all five versions.
- **A layer written by a File Output node takes its item's name, spaces and all**, with no view-layer part in front of it — `MyBeautyLayer.R`, `second socket.R`. The same in all five.
- **A person at the screen sees the same names split in two**: a menu labelled `Layer` holding the view layers, and a menu labelled `Pass` holding the passes.
- **The Video Sequencer will not treat these files as multi-layer.** Every version's manual says it does not support them. Offered three multi-layer OpenEXR frames as a strip, every version wrote the frames out with no message about the format at all — the picture is there, and darker than the same scene's PNG frames put through the same route. The same on every machine that ran it.

**The third picture-choosing menu.** An ordinary multi-layer OpenEXR shows two menus. A multi-view file gains a third only with the stereo button off; it reads `left`. In 4.5 to 5.2 this was observed on Linux, where its tooltip is `Select View.`; its open contents remain unestablished. In 5.3 on the Mac the opened picker lists a greyed `View` label, then `right` and `left`. The file itself names both views.

**A file holding several views.** With the image settings left as they come, every version writes **two** files, one per view — in 5.3, `frame_0001_L.exr` and `frame_0001_R.exr`. Told to put both views in one file, every version writes **one** — and what is inside it differs by version, which is set out below. In the four earlier versions this was run on the rented Linux machine only, and the Mac never wrote a multi-view file; in 5.3 it was run on the Mac only.

**Where it differs, version by version.**

**Blender 4.5.** The file is a **single-part** OpenEXR (version field `0x00000002`). All thirty channels of both view layers sit in one part, and there is no per-part `name` attribute to read — the channel names are the only place the layer and pass are recorded. There is no `Interleave` setting; the Python property for it does not exist, and the Output panel does not offer it. The pass parts of the names are `Combined`, `Depth`, `Mist`, `Normal`, `Emit`, `DiffCol`. A File Output node writing such a file names it for the **frame number alone** — `0001.exr`. **With several views in one file, 4.5 writes one part whose channels carry the view — `Beauty.Combined.left.R` and so on — plus an attribute `multiView` listing `left` and `right`.** (Every version's multi-layer file also carries a `BlenderMultiChannel` attribute — question 10.)

**Blender 5.0, 5.1, 5.2 and 5.3.** The file is a **multi-part** OpenEXR (version field `0x00001002`), with one part per pass per view layer — twelve parts for the two view layers and six passes of the test. Each part carries its own `name` attribute reading `<view layer>.<pass>`, so with no compositor the finished picture is the part named `Beauty.Combined`. Each part also carries `colorInteropID`, and its value is not the same in every part: the parts holding colour passes read `lin_rec709_scene` and the parts holding data passes — Depth, Mist, Normal — read `data`. The `Software` attribute naming the Blender version is in **part 0 only** (question 10). Blender 5.0's release notes state the change: "Multilayer OpenEXR now saves multi-part files by default … The 'Interleave' option can be turned on to get the old behavior." That `Interleave` switch is present in all four, off by default, in the Python interface and in the Output panel; turned on, it brings the file back to one part with no `name` attribute and every channel in one list. The pass parts of the names are `Combined`, `Depth`, `Mist`, `Normal`, `Emission`, `Diffuse Color`. A File Output node writing such a file uses the name it was given — `fromnode_0001.exr`. **With several views in one file, 5.0 to 5.2 write one part per view per pass, named `Beauty.Combined-left` and `Beauty.Combined-right`, and the view is not in the channel names at all** — on the Linux machine. 5.3 wrote the same shape on the Mac, with each part also carrying a `view` attribute reading `left` or `right`; whether the earlier three write that attribute is not recorded, and the two machines are not set against each other.

**Blender 5.3 also writes the composited picture into the file.** With a compositor in use, a multi-layer render gains a part of its own, placed **first**, named **`Composite.Combined`**, holding the compositor's result and carrying the file's record of fields; the view layers' own Combined parts still hold the picture before the compositor. With Interleave on, it is the channels whose names begin `Composite.Combined.`. No earlier version was looked at for this — see *Not established*.

**Verdict: it differs.** The single-part-against-multi-part shape, the part names and the File Output node's naming were settled by the same test run in the four earlier versions on the Creator's Mac and again on the rented Linux machine, which agreed, and in 5.3 on the Mac. **The multi-view half was run on the Linux machine in the four earlier versions and on the Mac in 5.3**, so it settles each of those operating systems for those versions only.

**Not established.**

- **What the View picker holds when it is open, in 4.5 to 5.2.** Settled for 5.3 on the Mac. In the earlier four the picker is settled — it exists only for a file with several views, appears only with the stereo button off, is named `Select View.` and reads `left` — and its open contents are not.
- **The composited picture inside a multi-layer file, in 4.5 to 5.2.** Never looked for. 5.3's `Composite.Combined` is not evidence that the earlier versions lack it.
- **Whether 5.0 to 5.2 write a `view` attribute on each part of a multi-view file**, and whether 5.3 writes the same multi-view shape on Linux.
- **Whether any format other than OpenEXR can hold several pictures.** Every version's manual attributes the ability to OpenEXR alone and says nothing either way about the others. Untested everywhere.
- **How the Interleave switch changes what a file is called.** Present from 5.0. Turning it on brings a multi-part file back to one part (here and question 10); its effect on naming was never seen, on any machine.
- **Whether several view layers and several views can be combined in one file.** The multi-view tests used one view layer; the two-view-layer tests used one view.
- **Which view an older OpenEXR reader shows for a multi-view file.** 5.3's manual says such readers see one view and does not say which; untried.
- **Whether the same names come out on Windows.** They come out the same on macOS and, for the four earlier versions, on Linux.

---

## 6. Does a format that can hold more light than a screen can show also record the treatment it was made under?

**How it is known.** The view-treatment record is **not in the sources** in 4.5 to 5.2; 5.3 labels its absence on reloading **inferred**. Colour-space information is **documented** in 5.0 to 5.2. In 5.3, general colour-space writing is documented, its application to OpenEXR is inferred from the sources, and the ICC addition has a release note. Actual headers and reloaded settings below are **observed in tests** on the Mac and Linux in 4.5 to 5.2, and on the Mac alone in 5.3. HDR display files were tested only in 5.3 on the Mac.

**The same in all five.** **The treatment is not recorded.** Three OpenEXR files were written from each version, changing one thing at a time — one under AgX with no look, one under Standard, one under AgX with a Punchy look, exposure 2.0 and gamma 0.5. In every version the three headers came back as the same text apart from the date and the render time. No attribute in any version names a view transform, a look, an exposure or a gamma. There is no `chromaticities` attribute in any version.

Three further results are identical in all five:

- **Radiance HDR records nothing at all.** Its header is `#?RADIANCE`, `FORMAT=32-bit_rle_rgbe`, the picture size, and then the pixels.
- **Reopened files come back the same regardless of what they were made under.** Every OpenEXR made in the default working space loads as `Linear Rec.709` with View as Render off; the PNG loads as `sRGB`.
- **The metadata panel has no colour-management field.** Its fourteen render fields are the same in all five versions, measured in the running Blender — see question 10.

**Where it differs, version by version.** What the file records about the **colour space** — which is not the treatment — changed three times.

**Blender 4.5.** An OpenEXR carries **no colour-space attribute**. There is no `colorInteropID`. A PNG written by 4.5 has the blocks `IHDR, eXIf, oFFs, pHYs`, seven text entries and `IDAT` — **no `sRGB`, `gAMA` or `cHRM` block**. A blend-file has no working colour space at all; asking for one reports that this version has none.

**Blender 5.0.** An OpenEXR carries **`colorInteropID`**, reading `lin_rec709_scene` by default. Blender 5.0's release notes state the addition: "OpenEXR files now support Color Interop ID metadata, to automatically detect the file colorspace." The attribute follows the blend-file's **working colour space**, which is new in this version: changing the working space to ACEScg makes the attribute read `lin_ap1_scene`. A PNG still has no `sRGB`, `gAMA` or `cHRM` block.

**Blender 5.1 and 5.2.** OpenEXR behaves as in 5.0. A PNG now **does** carry colour blocks: `IHDR, sRGB, gAMA, cHRM, eXIf, …`, with `sRGB` reading `03` and identical `gAMA` and `cHRM` values in both versions. So a PNG written by these versions describes the display space it is in, and still says nothing about the view transform, the look, the exposure or the gamma. No PNG carries an ICC profile.

**Blender 5.3.** OpenEXR behaves as in 5.0, the ACEScg case included — and that file reopens as `ACEScg`. **A PNG now also carries an sRGB ICC profile and a `cICP` block**: `IHDR, cICP, iCCP, sRGB, gAMA, cHRM, eXIf, …`. The `cICP` block names BT.709 primaries, the sRGB curve and full range; the `iCCP` block holds a 334-byte profile named "Embedded Profile", described inside as `sRGB`. The same blocks come under AgX and under Standard alike. A 5.3 JPEG carries the same sRGB profile (question 10), and a 5.3 WebP is flagged as holding one. 5.3's release notes state the change: "Saving sRGB image files now includes an ICC profile." 5.2.2, run on the same Mac under the same conditions with the same script, wrote its PNG with no `cICP` and no `iCCP`. Still nothing names the view transform, the look, the exposure or the gamma.

**HDR pictures, in 5.3 alone.** 5.3's build offers six displays — `sRGB`, `Display P3`, `Rec.1886`, `Rec.2020`, `Rec.2100-PQ`, `Rec.2100-HLG` — and its HDR views carry a nits figure in their names. A picture written as an 8-bit PNG and an 8-bit AVIF under `Rec.2100-PQ` with the view `ACES 1.3 - HDR 1000 nits` records the encoding and nothing more: the PNG's `cICP` names BT.2020 / BT.2100 primaries and the PQ curve (SMPTE ST 2084), with **no** ICC profile, while still carrying the `sRGB`, `gAMA` and `cHRM` blocks; the AVIF's colour box names the same. Neither file holds a mastering-display or content-light-level block, the view's name, a nits figure or the look. 5.3's own manual says the nits figure in a view's name is used for mastering-display metadata; in these 8-bit files none was found.

**A further documented change in 5.3.** Its release notes announce improved support for display colour interop IDs in OpenEXR files and colour management. The tested identifiers do not reveal what improved.

**Verdict: it differs.** Settled by the same tests run in the four earlier versions on both machines, which wrote the same headers and the same blocks, and by 5.0's release notes; 5.3's profile by the same test against 5.2.2 on the Mac and by 5.3's release notes. Windows is untested, and Linux untested for 5.3.

**Not established.**

- **Whether an ICC profile is ever embedded by 4.5 to 5.2.** The 5.1, 5.2 and 5.3 manuals document embedding when the colour configuration requests a profile. No tested 4.5-to-5.2 file carried one, and no version tested a configuration requesting one.
- **What an HDR picture records beyond 5.3's 8-bit PNG and AVIF.** The 203-nits and 100-nits conventions are documented in 5.1's, 5.2's and 5.3's manuals for HDR pictures written as AVIF or PNG, and for HDR video; Those conventions are not established here for 4.5 or 5.0, and neither can write AVIF. Only 5.3 wrote an HDR picture, and only at 8 bits. Deeper files, HDR video, and every earlier version are untried. Whether Blender differs here or only the manuals do is untested.
- **What happens when a colour space has no interop identifier.** The 5.1 API documents that the identifier may be empty; untested in every version.
- **What a machine that does not have the colour space makes of the name in the file.** The attribute follows the writing machine's working space (5.0's release note and the ACEScg test). What a reading machine under a different colour configuration does with it was never tested in any version. **Dependencies 15** owns the configuration-on-the-computer half, and settles the equivalent for a blend-file's own stored name — the name is silently dropped — but nothing tests the file attribute.
- **The precise effect of 5.3's display colour interop improvement.** Its release notes establish improved support in OpenEXR files and colour management. The tested OpenEXR files carried the same `colorInteropID` values as 5.2's, so those tests do not identify the improvement.
- **Whether the picture itself differs under a different treatment.** The three files' headers matched; their pixel data lengths did not, but they came from three separate renders, so nothing is settled. Nobody compared the pixels, in any version.
- **Whether the Output panel's colour-management override is recorded in the file.** Untested everywhere.
- **The Linux Wayland sentence.** 5.3's manual says that on Linux under Wayland Blender switches between sRGB and gamma 2.2 encoding depending on whether an SDR or HDR display was chosen, and does not say whether that touches saved files. Untested; it needs a Linux machine running Wayland.

---

## 7. Can Blender turn a set of finished frames into a short moving picture?

**How it is known.** The frames-to-movie workflow is **documented** in all five versions. Joining the steps without a window was **inferred** in 5.3, then **observed in a test**. File and pixel results below were tested on the Mac and Linux in 4.5 to 5.2, and on the Mac alone in 5.3; menu observations are Mac-only.

**The same in all five.** The whole job works, and it works with no window open. Every version gave the same results:

- **Building the strip and encoding it both work headless.** Six finished PNG frames were made, the 3D scene was then changed beyond recognition, an image strip was built from the six saved files by a script in a windowless Blender, and `blender -b <file> -a` from a terminal wrote `out_0001-0006.mp4`, reported as ISO Media MP4, exit status `0`.
- **The pictures really come from the strip.** Rendered again to PNG with the Sequencer pipeline on, the frames are the saved ones; with it off, they are the changed 3D scene. Every version. The six Sequencer frames took a fraction of a second against about that each for the six 3D renders.
- **There is no command-line argument that turns a folder of frames into a movie by itself.** Pointed at a plain startup file, `-F FFMPEG … -a` writes a movie of the 3D scene in every version.
- **A frame whose file has gone missing comes out fully transparent black**, with nothing printed about it, in every version. All the other frames are written normally.
- **Multi-layer OpenEXR frames go through anyway**, with no message about the format, in every version — the picture is there and darker.
- **The length follows the frame rate.** Twenty-four frames wrote a one-second movie at 24 frames per second and a two-second movie at 12, in every version, with the same time scale recorded in the file.
- **The container and codec menus are the same** in all five versions, and the MPEG-4 container with the H.264 codec wrote a playable file in all five. What those menus offer and what the encoder will actually take are two different things — see question 4.
- **The animation player is documented as ignored with the background flag** in 4.5, 5.0, 5.1 and 5.3. This is not established for 5.2; that gap does not establish different behaviour.

**Where it differs, version by version.**

**Blender 4.5.** During the encode Blender prints `Append frame 1` through `Append frame 6`, on plain lines.

**Blender 5.0, 5.1, 5.2 and 5.3.** Blender prints `Rendering frame N`, `Executing sequencer` and `Video append frame N`, each on a timestamped log line. The `Executing sequencer` line is the one that shows plainly that the Sequencer pipeline ran.

**Blender 5.0 onward — sequencer-scene render options.** Blender 5.0's release notes state that when the sequencer scene differs from the active scene the Render menu gains "Render Sequencer Image" and "Render Sequencer Animation" entries. The render command's own options match: 4.5's takes `animation, write_still, use_viewport, layer, scene`; 5.0's and 5.1's take those plus `use_sequencer_scene`, `frame_start` and `frame_end`. **Whether 5.2 and 5.3 still have them is not established by these findings.**

**Verdict: it differs** — in what is printed, and in the sequencer-scene options added at 5.0. The job itself is identical in all five — on both machines in the four earlier versions, on the Mac in 5.3. Windows is untested, and Linux untested for 5.3.

**One small number, recorded rather than read into.** The multi-layer frames put through the Sequencer read `0.2603` at the centre in 5.3 against `0.2599` in 5.2, and 5.2.2 gave `0.2599` again on the same Mac. But each version rendered those frames itself before its Sequencer read them, so the check does not separate the render from the reading. The PNG frames through the same route came back identical in 5.2.2 and 5.3. Nothing should be built on it.

**Not established.**

- **How long "short" may be.** No documented or tested limit. Automatic splitting above 2 GiB is documented for 4.5 and 5.1; it is not established here for the other three. The 5.3 Mac panel offers Autosplit Output, but its threshold was not tested. Those gaps do not establish a version difference.
- **What colour a placeholder frame is.** Blender 4.5's manual says missing frames are "displayed as black"; 5.0's, 5.1's, 5.2's and 5.3's say "Placeholder frames are rendered in pink in the VSE preview". The test that was run used a strip naming a file that had been deleted, not the placeholder feature, and it came back fully transparent black in all five. So 4.5's manual and the other four's say different things about a case nobody tested. **This looks like a change and is not established as one.**
- **Whether any of this behaves the same on Windows**, or — for 5.3 — on Linux. Every version's manual says the video format itself is a build-time option not present on all systems; every build the project read carries it.
- **What a movie file, rather than a PNG frame, shows for a missing frame.** Untested everywhere.
- **Whether an animated single-picture format such as GIF can be written.** Searched for and not found in any version; every version's complete format list holds nothing of the kind, and 5.3's search for animated WebP and APNG found nothing either.

---

## 8. Can Blender be made to produce a smaller, plainly viewable copy of a picture a render produced?

**How it is known.** Preview, resize and save tools are **documented**. The Preview copy's smaller size is **not in the sources**; combining resize and write tools is **inferred** in 4.5, 5.1, 5.2 and 5.3. The results below are **observed in tests** on the Mac and Linux in 4.5 to 5.2, and on the Mac alone in 5.3. Preview colour and an ordinary window still's hand save were tested only in 5.3 on the Mac.

**The same in all five.** Yes — and by more than one route. Four results came back identically in every version:

- **The Preview copy.** With OpenEXR output and Preview switched on, a JPEG copy is written beside every frame, with the same stem. It has **the same number of pixels** as the main file and fewer bytes (roughly a third in most versions, a quarter in 5.2, about two-fifths in 5.3 — figures nobody looked into). It is written for an animation, for a single frame started from a terminal, and — see question 10 — for a still written to the output path from a Blender with its window open as well, which every version's manual's wording does not lead a reader to expect. **With PNG output the setting can be switched on and reads back as on, and no JPEG copy is written** — in every version. The Output panel offers a Preview checkbox for OpenEXR and for the multi-layer variant, and for nothing else, in every version. So every version's manual is right to document it under OpenEXR, and every version's Python interface is misleading in listing it on the general image-format settings.
- **A smaller second copy made during the render works, and works headless.** A Scale node at half size feeding a File Output node wrote a 32 by 24 second file beside the 64 by 48 main one, in every version.
- **Two Image Editor commands do not work without a window, in any version.** `bpy.ops.image.resize` and `bpy.ops.image.save_as` both refuse with `poll() failed, context is incorrect`, and in 4.5 to 5.2 `save_as` refuses even with a window until the Image Editor has been drawn once. In 5.3 the hand save succeeded with a drawn editor; whether drawing it first is required was not tested. The plain calls `Image.scale` and `Image.save_render` do the same job instead — with the catch, identical in every version, that `save_render` writes in the **scene's** output format whatever the path's ending says. A file named `.png` came out as an OpenEXR in all five until the scene's format was changed.
- **Proxies are not built during a background animation render.** With a strip set to build a 25% proxy and the file saved, every version wrote only the output frames and left no `BL_proxy` folder anywhere.

**Where it differs, version by version.**

**Blender 4.5.** The compositor's Scale node has a `space` setting offering `RELATIVE`, `ABSOLUTE`, `SCENE_SIZE` and `RENDER_SIZE`, and three input sockets: Image, X and Y. The File Output node writes its second copy as `small_0001.png` — the node's own file name and the frame number. Read back, that file reports 32 bits and not floating point.

**Blender 5.0, 5.1, 5.2 and 5.3.** The Scale node has no `space` property. Its eight inputs are Image, Type, X, Y, Frame Type, Interpolation, Extension X and Extension Y; Type starts as `Relative`. In image mode the File Output name combines the node's file name, item name and frame number without inserting a separator: `small_` plus `small` gives `small_small0001.png` in the same-Mac 5.2.2 and 5.3 tests. The file reads back as 128 bits and floating point. An automatic extra underscore before the frame number is not established as a difference between those versions.

**What the node calls its file follows the node's mode**, not the version alone: in multi-layer mode no item name reaches the file, and in image mode it does. The names above are image mode; question 5 gives the multi-layer names. And a freshly added node starts on PNG in 4.5 and on Multi-Layer OpenEXR in 5.0, 5.1, 5.2 and 5.3 — **that starting value is itself a version change**. The rule about the mode is settled under **File output nodes 1 and 6**.

**What the node is fed from matters in 5.0 to 5.2 and not in 5.3.** Fed from a Render Layers node, the second copy is written in every version from 5.0. Fed from the compositor group's own input — the only source 5.3's default compositor group has — 5.3 writes it and 5.2.2 writes nothing. Question 10 holds the full account.

**Verdict: it differs.** Settled by the same four tests run in the four earlier versions on the Creator's Mac and again on the rented Linux machine, which agreed, and in 5.3 on the Mac. Windows is untested, and Linux untested for 5.3.

**A stale page, worth knowing.** Blender 5.1's manual still documents the Scale node's `Space` setting with its Relative and Absolute options, and the node in 5.1 has no such setting. That is a mismatch inside one version, not a difference between versions.

**Not established.**

- **The JPEG quality the Preview copy is written at.** Unanswered in all five.
- **What colour treatment the Preview copy takes, in 4.5 to 5.2.** Settled for 5.3 on the Mac: **it takes the view transform**. Under the `False Color` view the OpenEXR's centre stayed grey and its JPEG copy came out green, as the same scene's PNG did. Untested in the four earlier versions.
- **What a still started from the window writes, in 4.5 to 5.2.** In 5.3, on the Mac, a still started the way the Render menu starts it writes nothing to the output path — no OpenEXR and no JPEG copy — until it is saved by hand from the Image Editor, and that hand save writes the OpenEXR alone, with no JPEG copy. That still renders are not saved by themselves is in 5.2's and 5.3's manuals; what the hand save leaves out was seen in 5.3 only.
- **Whether 5.3 requires the Image Editor drawn before Save As.** Its successful drawn-editor save does not test that requirement.
- **Whether proxies can be built without a window by some route other than a background animation render.** Untried everywhere.
- **Whether any of this behaves the same on Windows**, or — for 5.3 — on Linux.

---

## 9. What does a render produce where an image a file points at is missing, and that image is feeding something other than colour?

**How it is known.** The missing-material result is **not in the documentation** in any version. The four earlier versions **observe** a fixed magenta lookup in Cycles' files and **infer** the step from a missing disk file to that lookup, and its downstream effects. 5.3 **observes** a magenta constant but finds the Image Texture node's missing colour supplied by code outside the available files; the compiled Open Shading Language shader defaults to black. Its source reading leaves the value open. The results below are **observed in tests** on the Mac and Linux in 4.5 to 5.2, and on the Mac alone in 5.3, except where narrower coverage is stated.

**The supplied code changes its names and layout without changing the tested substitution.** The 4.5 and 5.0 files name four values `TEX_IMAGE_MISSING_R/G/B/A` in `source/util/texture.h`; 5.1, 5.2 and 5.3 name `IMAGE_MISSING_RGBA` in `source/util/types_image.h`. The 5.2 reading also describes the markers `KERNEL_IMAGE_NONE` and `KERNEL_TILE_LOAD_FAILED` in `source/kernel/device/gpu/image.h`; the 5.3 reading finds the shared lookup in `source/kernel/util/image.h` and the node-supplied fallback described above. These source observations do not establish a changed surface colour. In the Emission-only, unlit Standard-view test, all five give red 1, green 0, blue 1, alpha 1. Ordinary lighting changes the rendered numbers; that test measures the substitute alone.

**The measured 5.3 missing colour is magenta.** Source uncertainty does not turn that Mac observation into a documented guarantee.

### The same in all five

Every one of the following came back identically in 4.5, 5.0, 5.1, 5.2 and 5.3 — on both machines in the earlier four, on the Mac in 5.3 — unless the machine is named.

- **A missing picture gives magenta at the node's Colour output and 1 at its Alpha output.** With the picture there, the test plane rendered red 0, green 1, blue 0, which is the control showing the file was really being read; with the picture moved away it rendered red 1, green 0, blue 1. The Alpha output read about 0.25 with the picture there and exactly 1 with it gone — so what a cut-out mask would be handed is the fully opaque end. **Nobody rendered such a mask**, in any version, so "the surface stays solid rather than going invisible" is what the measured 1 implies and not something a test watched.
- **The graphics card and the processor agree**, in every version, on every machine.
- **The picture's Colour Space setting changes nothing.** `Non-Color` and the default gave the same numbers. That settles the second way the question can be read.
- **A colour plugged into a socket that takes one number becomes about 0.2848** — a single value, the same at every point, rather than any pattern. The exact figure differs slightly between 4.5 and the other four; see *Where it differs*.
- **The render finishes, writes its frame, and ends with an exit status of 0.** Standard error was empty in every run in every version where it was captured. Anything watching exit codes learns nothing from a missing picture.
- **A missing normal map is not a no-change.** A Normal Map node fed its own default colour of 0.5, 0.5, 1 renders exactly as no Normal Map node at all — mean difference 0.000000, largest 0.000000. A missing picture on the same input does not: it differs from the plain sphere by mean 0.031 to 0.033 of a channel in the earlier four (0.023 in 5.3, measured over all four channels). Those means are not a version comparison: the earlier measurement does not specify its channels. So the surface is shaded as though tilted, everywhere and by the same amount.
- **A missing bump map produces no bump.** The missing-picture sphere is indistinguishable from the sphere with nothing connected, against a control of 0.47 to 0.65 of a channel showing what the real picture does. This is the case that goes unnoticed: no magenta, no obvious wrongness, just missing detail.
- **A missing displacement map shrinks the object.** In the rebuilt test the outline held 1424 pixels untouched, about 870 with the picture missing, about 350 with nothing connected to the Displacement node, and about 3046 with the picture present. So the surface moves — inward — and it moves by the same amount everywhere, with no relief in it.
- **EEVEE substitutes the same magenta as Cycles**, and behaves the same way on the three spheres: the normal map changes, the bump does not (largest difference exactly 0.000000), the displacement shrinks.
- **Workbench substitutes the same magenta**, but only once its colour is set to Texture and its lighting to Flat. Without those two settings it renders the same grey whether the picture is there or not. **Both EEVEE and Workbench started and rendered on the rented Linux machine, which has no display server of any kind**, in the earlier four.
- **Five different kinds of absence give the same magenta**: the file moved away; the path pointing into a folder that does not exist; a file of zero bytes; a file named `.png` holding two hundred bytes of zeros; and a file the running user is not allowed to read. **One does not**: a working PNG whose name ends `.xyz` loads normally and gives the picture. So Blender goes by the file's contents, not its name. The permission case was run on the rented Linux machine alone in the earlier four, as an ordinary user, with its own control showing the same file readable gives the picture; in 5.3 it was run on the Mac, where reading the same file from the terminal failed with `Permission denied` just before the render.
- **One missing tile of a tiled picture affects only itself.** The half of the plane drawn from the tile that was still there stayed green; the half drawn from the tile whose file was gone went magenta.
- **One missing frame of a picture sequence affects only itself.** Frames 1 and 3 were unchanged; frame 2 went magenta.
- **A picture packed into the blend-file is unaffected by its file on disk being deleted.** It renders the picture. The same file unpacked, in the same version on the same machine, gives magenta — that is the control.
- **Box projection and the world's Environment Texture behave exactly as the plain node does.** Both went from the picture's green to magenta together.
- **Open Shading Language gives the same magenta**, and gives 0.2848 for the colour-into-a-number case in all five versions. The reason was read out of the compiled shader in one version: the three weights are fixed in the shader file rather than taken from the scene. The measurement itself is the same in all five.
- **The compositor gives black, not magenta.** An Image node whose file is missing, driving the factor of a mix, rendered 0.7152 with the picture there and 0.0 with it gone — in 5.3 by both of its compositor routes. Whatever the compositor substitutes, it is not the shading system's magenta.
- **A geometry-nodes Image Texture node gives zero, not magenta.** Driving a Set Position offset, the vertices moved with the picture present and sat on the untouched grid exactly with it gone, so the offset received was 0, 0, 0.
- **A Volume object whose OpenVDB file is gone draws nothing at all, and says nothing.** The frame matched a render with no Volume object in the scene, to every digit printed, in every version on every machine, and no run printed anything matching `ERROR`, `WARNING`, `Error` or `Traceback`.
- **`bpy.ops.file.report_missing_files()` works with no window.** It returns `{'FINISHED'}` either way and prints its report to the ordinary output: `Info: No missing files` where the file is there, and `Warning: Path '<path>' not found, from local data-block 'IM<name>'` where it is not. So a render farm can ask.
- **`has_data` never distinguishes a missing picture from a present one.** It read false before the render, after the operator and after a completed render, in both states, in every version. It is not a way to spot the condition.

**One more, in 5.3 alone.** **A missing picture driving roughness gives that same single number.** A Glossy surface whose Roughness was fed by a missing picture rendered identically, to every pixel, to the same surface with its Roughness typed in as 0.28483182191848755 — and the picture-present render nearly matches roughness typed as the picture's own 0.8 (largest channel difference 0.000003). Run on the Mac only; the earlier four measured the conversion but never rendered with it driving roughness.

### Where it differs, version by version

The differences below concern console output, settings, small measurements and a documented addition.

**What Blender prints.** This is the one that matters, and it is the same difference question 2 records; here it was measured again from a plain render command and from six kinds of absence.

- **Blender 4.5**, rendering in Cycles, prints nothing in any of the six states that separates it from the same render with the picture in place. Searching the whole of the run's output for *missing*, *cannot*, *can't*, *unable*, *error*, *warn*, *fail*, *not found*, *no such* and *invalid* matched one line, the one saying the frame was saved. That silence is a real result rather than an absence of looking: the picture-present run of the same command prints a banner, thirty-odd progress lines and a save line, and the frame written by the missing-picture run carries the magenta, so the file genuinely had not been read. **What 4.5 prints on an EEVEE render, and what it prints for a corrupt file, is under question 2.**
- **Blender 5.0** prints `cycles | ERROR Image file '<path>' does not exist.`, with single quotes round the path, on the ordinary output rather than the error output. Where the file is present but unreadable — zero bytes, rubbish contents, or no permission — it prints `cycles | ERROR Image file '<path>' failed to load.` instead.
- **Blender 5.1** prints the same two lines with no quotation marks round the path.
- **Blender 5.2 and 5.3** print `cycles | WARNING Image file <path> does not exist.` and add `cycles | ERROR Failed to load 1 image files`. Where the file is present but unreadable they print the `WARNING` line twice, followed by the same counting `ERROR`. In 5.3 a missing tile and a missing frame of a sequence each draw the same pair, naming the one file, and a packed picture and a geometry-nodes picture draw nothing. **On an EEVEE render 5.3 adds `image.gpu | ERROR Image file "<path>" does not exist`, naming the file** — question 2 has the comparison with 5.2.

**Which kinds of unreadable file 4.5 names is not settled.** Question 2 records a corrupt file named by `IMB_load_image_from_memory: unknown file-format (<path>)`; two of the six kinds of absence here — a file of zero bytes, and a file named `.png` holding two hundred bytes of zeros — drew no line at all. Nothing compared the two.

**The name of the EEVEE engine, and the route decides which name works.** Set from a script, each version takes only its own name — `BLENDER_EEVEE_NEXT` in 4.5, `BLENDER_EEVEE` in 5.0, 5.1, 5.2 and 5.3 — and refuses the other outright with a plain error listing the three engines it will take. That is the route this test used, in all five. Given as `-E` on the command line it is not the same: 4.5 takes `BLENDER_EEVEE_NEXT` only, and `BLENDER_EEVEE` errors, writes nothing and then hangs, while 5.0, 5.1 and 5.2 take **both** names, end at 0 and write a file of the same size. 5.3 was never given a name by `-E`. **The name that works everywhere, by every route tried, is the version's own** — `BLENDER_EEVEE_NEXT` in 4.5, `BLENDER_EEVEE` from 5.0.

**Three incidental settings.** `world.use_nodes = False` takes in 4.5 and does not take in 5.0, 5.1, 5.2 or 5.3. The earlier test's OpenVDB file gives dimensions of 3.5938 by 3.5938 by 4.8438 in 4.5 and 3.75 by 3.75 by 5.0 in 5.0 to 5.2. The 5.3 file baked from the same recipe has the same 57,570-byte size and gives the latter dimensions; its bytes were not compared with the earlier file. The compositor is reached through `scene.node_tree` in 4.5 and through `scene.compositing_node_group` from 5.0 — in 5.3 through its effects stack as well (question 1) — and from 5.0 the two node types the 4.5 test used, `CompositorNodeValToRGB` and `CompositorNodeComposite`, no longer exist and are replaced by `ShaderNodeValToRGB` and `NodeGroupOutput`.

**The number, in its fifth decimal place.** A colour plugged into a socket taking one number gives about 0.284849 in 4.5 and about 0.284832 in 5.0, 5.1, 5.2 and 5.3 (5.3: `0.28483182191848755`); the picture's own green through the same route gives about 0.715152 in 4.5 and about 0.715169 in the other four. In Blender 5.0 and 5.3, where both shading paths were measured, the figure is 0.2848318 through the ordinary path and 0.2848000 through Open Shading Language. The gap between 4.5 and the other four sits in the fifth decimal place; between the two machines the same figures differ only in the seventh. **It was measured on the processor** — the graphics card was run for the colour, not for this number. Nobody looked into why. Nothing should be built on it.

**A documented addition at 5.2.** Its release notes add three options to `bpy.data.file_path_foreach` so render farms and scripts can find all relevant image files, including the new Cycles texture cache. This establishes the addition at 5.2. It was not tested here, and continued availability in 5.3 is not established by these findings. It does not change the missing-picture results above.

**Verdict: it differs** — in what Blender prints, in what the EEVEE engine is called, in the incidental settings above, and in the documented 5.2 additions. **The tested missing-picture effects on the surface agree; some measured numbers differ as described above.** The observed results were settled by the same test run in the four earlier versions on the Creator's Mac and on the rented Linux machine, which agreed, and in 5.3 on the Mac. The 5.2 cache and enumeration additions rest on their release note. Windows is untested — no Windows machine ran it — and so is Linux for 5.3; those are silences, not differences.

### Where the two machines differ, which is not a difference between versions

- **Workbench's alpha and its magenta.** On the Mac the magenta reads 0.99755859375 and on the rented Linux machine 0.9990234375, in all four earlier versions, and 5.3 on the Mac reads 0.99755859375 too. The alpha reads 1.0 on the Linux machine in all four, and on the Mac reads 0.99755859375 in 4.5 and 0.998046875 in the other four, 5.3 included. The colour difference is between the machines. The alpha difference between 4.5 and the rest shows on the Mac only and not on Linux. It is a small version difference confined to that Mac, not an operating-system difference to handle per version. Nobody investigated it and nothing rests on it.
- **Last digits.** A handful of means and one or two largest-difference figures differ between the machines in their last digits. Every silhouette count matched.
- **The permission case** was run on the rented Linux machine alone in the earlier four, because everything on that machine otherwise runs as the owner of every file, and a permission test run that way would prove nothing; 5.3's was run on the Mac, as the Mac's own user. Both gave magenta.

### Small numbers — recorded rather than read into

Several sets of small numbers came out of these tests, and each would be easy to mistake for a finding.

**The bump residue.** "No bump at all" was not exactly zero in every earlier run. Against a control of 0.47 to 0.65 of a channel, the largest single-channel difference between the missing-picture sphere and the nothing-connected sphere was about 0.00003 in 4.5 and about 0.006 to 0.0125 in 5.0, 5.1 and 5.2. That looks like a version difference until you see the other figure from the same run: in 5.0, 5.1 and 5.2 the same residue of 0.0125, 0.0125 and 0.00625 appears between **two states that both have no bump in them at all**. And in 5.3, and in 5.2.2 on the same Mac from the same script, both figures are exactly zero. So the residue is not the missing picture doing anything, and not the version; it belongs to how that earlier run measured. In every version the conclusion is the same and it is not close: a missing bump map leaves the surface as it was.

**The pixel counts.** The displacement counts below apply only where no sphere touches the frame edge. Cycles gives 3045 present, 868 missing and 348 with the input disconnected in 4.5, 5.2 and 5.3; 5.0 and 5.1 give 3048, 874 and 350. The cause of those few pixels is not established. Cropped outlines do not give comparable counts. EEVEE's 5.3 and 5.2.2 counts differ by one or two pixels too.

**The volume figures.** The volume test's picture-present figures cannot be set beside 4.5's, because the scene was not the same: `world.use_nodes = False` took in 4.5 and did not in the others, so 4.5 rendered against black and the others against the world's default grey. Among 5.0, 5.1 and 5.2 the scene *was* the same, and the three figures still differ — a mean over the frame of 0.2371 in 5.0, 0.2387 in 5.1 and 0.2187 in 5.2, with the largest pixel 2.003, 2.003 and 1.776. Both machines gave those figures, and both runs of the test gave them. 5.3 gave a mean of 0.21876 and a largest pixel of 1.7764, and 5.2.2 on the same Mac from the same script gave exactly the same, so the gap from 5.2's recorded mean is how the runs averaged, not the version. Nobody looked into why 5.0 and 5.1 differ from each other, so it is recorded here rather than read as a version story. What is comparable, and what the test was for, is that in every version the missing-file render matched the no-volume render to every digit.

**The geometry-nodes offset with the picture present.** 5.3 moved every vertex by exactly 1.0 where 5.2 moved it by about 1.00002, and 5.2.2 gave about 1.00002 again on the same Mac. Each run made its own picture with its own Blender, so the check does not separate the picture from the version. The missing case — the one the question asks about — is exactly zero in both.

### Not established

- **Windows**, in any version, and **Linux and every non-Apple graphics card** for 5.3. Everything above settles macOS on Apple silicon in all five, and Linux on an NVIDIA card in the earlier four.
- **Whether a 5.x command-line run under the old EEVEE name really renders in EEVEE, and whether it warns.** Neither was captured, and the command-line acceptance rests on one run on one machine on a 64 by 64 scene. 5.3 was not run by that route.
- **Continued availability of the texture cache and enumeration options in 5.3.** The 5.2 additions are documented above; no test here checks them in any version, and 5.3's availability is not established.
- **A tile waiting for memory and an image record with no pixel data.** The supplied 5.2 and 5.3 Cycles files describe an average-colour fallback while a tile waits; 5.3 also describes transparent black when an image record has no pixels. Neither state was reached by a test. Those source readings do not establish version differences.
- **Where the "Report Missing Files" warning goes when there is a window.** Every version's manual says the Info editor's header, and nobody looked, in any version. What the command does with no window is settled above.
- **Metallic, and roughness before 5.3.** The conversion they rely on — a colour output into a socket that takes one number — was measured directly in all five versions. Only 5.3 rendered with it driving roughness; no version rendered metallic.
- **Graphics cards of other makes.** Apple's own graphics in all five, and one NVIDIA card in the earlier four. Other makes remain untested.
- **A missing picture reached through a legacy Texture data-block** — a Displace modifier, a force field — rather than through a shader node. Untested in all five.
- **Whether 4.5's Principled Volume node carries a `density_attribute` property.** Its absence was observed in 5.0 to 5.2; 4.5 and 5.3 are not established here.

---

## 10. What does a finished picture hold, in every part of the file, about what produced it?

**The question has two readings:** information anywhere in a file, and its repetition in each part of a file with several parts. Both are covered below.

**How it is known.** The render fields and switches are **documented**. Exact file names, unconditional Blender identification and repetition across parts are **not established by the documentation**. Burn-in reaching formats without fields and operation without a window are **inferred** from the sources in the earlier four; 5.3 labels video-field coverage **inferred**. The results below are **observed in tests** on the Mac and Linux in 4.5 to 5.2, and on the Mac alone in 5.3. Disagreements that the tests leave open are listed below.

**The measurements cover the tested files and settings below.** They do not settle every output setting or machine.

### The same in all five

Every one of the following came back identically in 4.5, 5.0, 5.1, 5.2 and 5.3 — on both machines in the earlier four, on the Mac in 5.3 — unless a machine is named.

- **The block of named fields reaches the file, and these are the names it carries.** Fourteen of them: `File`, `Note`, `Date`, `Marker`, `Time`, `Frame`, `FrameRange`, `Camera`, `Lens`, `Scene`, `Strip`, `RenderTime`, `Memory`, `Hostname`. They are separate named entries, not one lump. **All fourteen appear only because this test switched all fourteen on.** A render from an untouched blend-file carries the seven the factory list below leaves on — `File`, `Date`, `Time`, `Frame`, `Camera`, `Scene`, `RenderTime` — and no more (question 6's PNG). In an OpenEXR the same fourteen names are separate header attributes; in a JPEG they are separate comment markers, each one reading `Blender:<name>:<value>` — for example `Blender:Note:SMZNOTE-12345`. **That is the answer a program outside Blender needs, and it is the same in every version.**
- **What each value looks like as text.** The date reads `2026/08/12 06:23:49`; `Time` is a timecode, `00:00:00:07`; `FrameRange` is `7:9`; `Lens` is `37.50`; `RenderTime` is minutes and seconds, `00:00.04`, and **not** the timecode shape; `Memory` is a figure with `M` after it, whose shape is the one thing here that changes by version. `File` holds the **whole path** to the blend-file, not just its name, in a blend-file that has been saved; in one that never has, it reads `<untitled>` (question 6). `Strip` reads `<none>` when there is no strip.
- **Which formats keep the block.** PNG, JPEG and OpenEXR — single-layer and multi-layer — keep all fourteen. BMP, Iris, JPEG 2000, Targa, Targa Raw, Cineon, DPX, Radiance HDR, TIFF and **WebP** keep none, and AVIF keeps none in the three versions that have it. **Every version's manual marks WebP as carrying metadata, and no version writes any into it.** That is a fault shared by all five manuals, not a difference between them.
- **Nothing is said when a format cannot carry the block.** No warning, no error, no line in the console, in any version. The fields are simply absent.
- **The manual's warning about "a limited subset of fields" does not bite.** All fourteen appear in each of the three formats that keep any, in every version.
- **Cycles adds four fields nobody switched on.** `cycles.ViewLayer.samples`, `cycles.ViewLayer.total_time`, `cycles.ViewLayer.render_time` and `cycles.ViewLayer.synchronization_time` are in every file a Cycles render writes that keeps fields at all. **They survive with every switch set to off**, and they are absent from an otherwise identical EEVEE render. No tag in either file names an engine.
- **A multi-layer OpenEXR always carries `BlenderMultiChannel = Blender V2.55.1 and newer`**, whatever the switches say.
- **The fourteen render fields include neither an engine name nor a view transform.** Cycles does write its sample count in `cycles.<view layer>.samples`. The image structure records its dimensions; those are separate from the fourteen render fields.
- **Burning the text into the picture adds to the block rather than replacing it.** With the burn-in on, the picture carries a block at the top left (file, date, render time, peak memory, hostname, then the note on its own line with no label), a strip along the bottom (marker, timecode, frame, frame range, camera, lens, scene) and a `Strip` item at the top right — the same fields in the same places in all five — **and the file written in that same run still carries the whole block of named fields as well**. The burnt-in text reaches Targa, which keeps no fields at all. The labels drawn into the picture are not the names in the file: `Memory` is drawn as `Peak Memory`, and `Time` as `Timecode`.
- **A field a render engine adds for itself does not reach the written picture.** An engine registered for the test called `stamp_data_add_field` with a distinctive name and value; the call succeeded, and neither string appears anywhere in the picture that render wrote — while the scene's own fourteen, in the same file, do.
- **Saving a still by hand from the Image editor writes the same block.** The hand-saved file's list of tags is identical, name for name, to the file the render wrote itself.
- **A window makes no difference.** The same file rendered twice on the same machine, once with no window and once from a Blender with its window open, carries the same tags name for name; only the date and the render time differ, as they must.
- **A rendered MPEG-4 with H.264 carries none of the fourteen under any name — the only container and codec asked.** An MPEG-4 with H.264 holds only its colour lines (`nclx`, BT.709 three times, a limited range flag), a creation date and an encoder line. **So the Python documentation's uneven wording — five of the fourteen say "image/video metadata" and the rest say "image metadata" — is not borne out in any version.** None of the five reaches a movie either.
- **Cryptomatte's manifest goes into the picture's own header**, as four attributes per layer named `cryptomatte/<hash>/conversion`, `/hash`, `/manifest` and `/name`. So the manual's note that sidecar files are not supported is about a separate file; the identity information itself is in the picture.
- **Pixel density.** With the density set to 300: PNG and BMP carry it in both directions, TIFF carries it, OpenEXR carries one number, **JPEG carries none although every version's table ticks it**, and WebP carries none. With non-square pixels, PNG and BMP write the two directions separately, TIFF writes both as the across figure, and OpenEXR still writes one number.
- **The factory switches are the same list in all five.** On: date, time, render time, frame, camera, scene, filename, include labels. Off: frame range, memory, hostname, lens, marker, strip name, note, burn into image. **`use_stamp_memory` reads off on a fresh scene while the property's own stated default is on** — in all five versions. The manual's screenshot describes a fresh Blender and the stated default describes the property; both are right about different things.
- **No property in any version has an interface name beginning "Use Strip Me"**, which is what the manual's screenshot appears to show in all five. The panel's own drawing code, read out of the running Blender, is identical in the four earlier versions, and 5.3's list of controls matches it; in all five it offers two sequencer controls, both shown only when the Video Sequencer is in use.
- **The two note properties are different things in all five.** On the render settings, one is a switch and one is the text; on the scene, the same name carries a string described as a user-defined note. Neither documentation page is wrong.
- **The OpenEXR preview copy is written both ways.** With the Preview option on, a JPEG copy sat beside the EXR when the frame was rendered from a command line **and** when a still was rendered to the output path from a Blender with its window open, in all five versions. Every version's manual words this as something that happens for animations or for single frames from the command line, and that wording does not draw the line it seems to. (5.3's still started the way the Render menu starts it is under question 8.)
- **`Hostname` and `File` hold that machine's own name and path.** `Artists-Studio-MacBook.local` on the Mac and `ubuntu` on the Linux machine, each matching exactly what that machine's `hostname` command prints. **Those values identify the machine, not a version difference.**

### Where it differs, version by version

The observed differences below concern version identification, file parts, node output, passes, text and colour profiles.

**An OpenEXR names the Blender that wrote it, from 5.0.** Blender 4.5 writes no attribute naming Blender into a single-layer OpenEXR at all. Blender 5.0, 5.1, 5.2 and 5.3 write a `Software` attribute reading `Blender 5.0.1`, `Blender 5.1.2`, `Blender 5.2.0 LTS` and `Blender 5.3.0 Beta` — and they write it **with every metadata switch turned off**, so it cannot be switched away. No tested PNG, JPEG, WebP or AVIF records the Blender version. JPEG field comments do begin `Blender:`. This is the plainest answer to the question the sheet asks: from 5.0, an OpenEXR always says which Blender made it.

**Where the record sits inside a multi-layer OpenEXR.** Blender 4.5 has no Interleave setting, writes one part, and puts everything in it. Blender 5.0, 5.1, 5.2 and 5.3 write the same render as six parts, and **all fourteen fields, and the `Software` attribute, sit in part 0 and in no part after it**; turning Interleave on brings the file back to one part. So anything that reads a single part of such a file sees the record only if that part is the first.

**A File Output node fed from the compositor group's own input.** With the group interface made first, Media Type set before format and one linked item, 5.3 writes `smznodeSMZNODE.png` for a still and `smznodeSMZNODE0007.png` for an animation. Both compositor routes work, with a window and without, and create the missing folder. Under that recipe 5.0 to 5.2 produced no node file on the Mac and Linux; 5.2.2 also produced none in the matching Mac control. Fed from Render Layers instead, 5.0 to 5.3 write — questions 5 and 8. In 5.2.2 the source alone was changed to establish that distinction. The 5.3 default group uses Group Input. Where the file is written it holds the fourteen fields, with Camera and Lens `<none>`, RenderTime `00:00.00` and no Cycles fields. A conflicting 5.0-to-5.2 result remains open below. The 4.5 compositor uses the older route under question 9.

**Cycles' Render Time pass.** In 4.5 the setting does not exist — asking for it fails with a plain error. In 5.0, 5.1, 5.2 and 5.3 it exists, and a render on the processor writes a channel named `ViewLayer.Render Time.X` which a render on the graphics card does not. The card was Apple's own graphics through Metal on the Mac in 5.0 to 5.3 and one NVIDIA RTX 4000 Ada through OptiX on the Linux machine in 5.0 to 5.2, and the two agreed. The Sample Count pass is written in all five versions, on both devices — in 5.3 from the standard scene of these tests, a factory scene at 64 by 64 and one sample, with the pass and the multi-layer media type set from a script and the render started by that script. **Reports 11** records 5.0 to 5.3 writing no such channel from another setup: in 5.3 a saved scene with a maximum of 512 samples and adaptive sampling at 0.1, the pass set by an expression handed in at launch, and the frame rendered with `-f 1` from the command line. Which difference between the two setups decides whether the pass is written is not established. 5.3 calls its two settings `pass_debug_sample_count` and `pass_render_time` and its channel `ViewLayer.Debug Sample Count.X`.

**The peak-memory fault note does not describe the tested builds.** The recorded 5.0 and 5.1 release note describes peak-memory metadata displaying `ddM`, listed as backported to 5.0.1. The tested 5.0.1 and 5.1.2 builds are later than that fault, and every tested build writes a real figure. The absence of that note in the other versions does not establish different behaviour.

**The shape of the peak-memory figure.** Blender 4.5 writes it with two decimal places; 5.0, 5.1, 5.2 and 5.3 write a whole number. The amounts themselves are measurements and are not comparable version to version; the shape of the text is the difference. Nobody looked into why. **Nothing should be built on it**, beyond knowing that a program parsing that field has to accept both shapes.

**The name of a stereoscopic movie file.** The same side-by-side Stereo 3D render is written as `stereo_0007-0009_L.mp4` by 4.5 and 5.0, and as `stereo_0007-0009.mp4` by 5.1, 5.2 and 5.3. All five files are 128 × 64, so both eyes are in the picture in every version. Anything that predicts the file name of a stereo render has to be written per version.

**The encoder line in a rendered movie.** `Lavf61.7.100` in 4.5, 5.0 and 5.1; `Lavf62.12.100` in 5.2 and 5.3. That names the FFmpeg library the build carries. It identifies a difference in the tested MPEG-4 files, not a guarantee about every field in every movie.

**An sRGB ICC profile in the JPEG, from 5.3.** A 5.3 JPEG carries an sRGB ICC profile beside its fields, as its PNG does; question 6 holds the whole of it.

**A documented movie change at 5.2 whose markings were not found by the test.** Its release notes announce stereoscopic metadata export and panoramic camera metadata export for full equirectangle cameras. The tested side-by-side stereo and equirectangular MPEG-4 files in all five versions carried none of the seven searched markings (`st3d`, `sv3d`, `svhd`, `prji`, `proj`, `equi`, `ytmp`), and exiftool reported none. That limited search does not erase the announced addition: it leaves which output carries it unsettled. 5.3's test is Mac-only and does not establish the feature's continued presence there.

**Verdict: it differs** — in the eight observed items above and the documented movie addition at 5.2. **The fourteen field names and the formats keeping or dropping them agree.** Memory's text shape differs as described above. The first seven were settled by the same test run in the four earlier versions on the Creator's Mac and on the rented Linux machine, which agreed, and in 5.3 on the Mac; the File Output node's half against 5.2 by 5.2.2 on the Mac; the profile by 5.3's release notes and question 6's test. Windows is untested — no Windows machine ran it — and so is Linux for 5.3; those are silences, not differences.

### Not established

- **Windows**, in all five versions equally, and **Linux** for 5.3.
- **Sequencer-strips metadata is not settled.** In each of 4.5 to 5.2, a STRIPS output carried the foreground image's fields in one observation and no Blender fields in others, even with those fields still present in the source image. In 5.3 on the Mac it carried none. No established condition explains the conflicting outcomes. With SCENE, every tested version wrote the render's own fields, the foreground strip's name and no Cycles fields.
- **The File Output result in 5.0 to 5.2 is not fully settled.** A node fed from Group Input produced no file under the written recipe on the Mac and Linux, while a file was also observed under incompletely specified conditions. Render Layers-fed nodes do write in those versions. In 5.2.2 on the Mac, changing only the source established that distinction; no such isolated change was tested in 5.0 or 5.1. The 5.2.2-versus-5.3 Mac difference above holds for the specified Group Input recipe, not every possible setup.
- **Density in later OpenEXR parts is not settled for 5.0 and 5.1.** Parts 1 to 5 were observed both with thirteen attributes including `xDensity` and with twelve excluding it, on the tested Mac and Linux machines. 5.2 consistently carried it; 5.3 carried it on the Mac. Those conflicting readings inside 5.0 and 5.1 do not establish a version difference. The fourteen render fields were consistently confined to part 0.
- **Whether the OpenEXR preview copy carries the block across versions.** Documented for 5.0 only; no test compared the copy with its neighbouring file in any version. The other versions remain unestablished.
- **Whether Cycles' four fields are written once per view layer in 5.0 to 5.2.** Observed in 4.5's single-part file and in 5.3 on the Mac. In 5.3, part 0 holds eight fields for the two view layers, `cycles.AAALAYER.*` and `cycles.BBBLAYER.*`. Not established for the middle three versions.
- **Cryptomatte manifest differences are not established across versions.** A `"(null)"` versus `"World"` entry was recorded between 4.5 and 5.2, but its identification conflicts between the object and asset manifests. The tested 5.3 object manifest matches 5.2 on the Mac; 5.0 and 5.1 were not compared. The four attribute names per layer are established in every version.
- **Where the announced stereo and panoramic markings are written.** The tested MPEG-4 files did not reveal them; unsearched markings and other output settings remain open.
- **What any container other than MPEG-4 carries.** Only MPEG-4 with H.264 was written and read, in all five versions. Question 4 put three frames through every one of ten containers with H.264 — some of them refused, and some wrote real files — and not one of the nine besides MPEG-4 was ever read for what it carries.
- **Whether an ICC profile is embedded by 4.5 to 5.2 under a configuration that asks for one** — see question 6, which holds the whole of that gap.
- **Whether anything differs on a graphics card of another make.** The Render Time pass result covers Apple's own graphics in all five and one NVIDIA card in the earlier four. Other makes remain untested.
