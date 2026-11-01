# What 5.3 changed

This is the list of Blender research questions whose answer for Blender 5.3 is not simply 5.2's, with every plan and every code file that names each one, so Build knows where to look. Every 5.3 result behind it is the Blender 5.3.0 Beta built 2026-10-16, run on the Creator's Mac and on nothing else. Where a line here and its comparison disagree, the comparison is right.

## How to read it

- **Three kinds of entry.** *Differs*: 5.3 answers differently from 5.2, by a test set against Blender 5.2.2 on the same Mac or against 5.2's own Mac record, or by 5.3's release notes. *Not established for 5.3*: 5.2 had an answer and 5.3 was not asked, or what it was asked cannot settle it. *An earlier reading reopened*: the 5.3 work, most often a same-day 5.2.2 run that did not repeat 5.2's record, unsettled or narrowed what was recorded for 4.5 to 5.2, so a plan built on the older reading needs a look.
- **One gap is on no entry because it is on all of them.** Nothing in 5.3 ran on Linux, on Windows, on a graphics card other than the Mac's own, or on more than one card. Wherever a plan leans on a Linux or NVIDIA result, that result is for 4.5 to 5.2 alone.
- **What is left out.** Questions where 5.3 answers as 5.2 does. Of the 179 questions (Add-ons 9 is retired), 123 are on the list: 74 differ, 38 are not established for 5.3, and 11 are earlier readings reopened.
- **How the citations were found.** Each question's name — topic and number, as a whole word — was searched in `Preparation/Specs/`, `Preparation/Design/`, `Preparation/Architecture/`, `Preparation/Build/` and `App/`, as the files stood on 2026-10-18. A list such as *Frames 5 and 12*, or a range such as *The compositor 1–7*, counts for each question in it. Signature files are left out, and so are a few lines where the words are ordinary talk about frames. Design and Build name none of the listed questions. Entries run from the most-cited down, counting files, plans and code together.
- **Where in a file.** `§` is a numbered section or a labelled part of that file; a row file names its rows; italics are the start of a heading; *line* is given where no heading stands above the citation. Code gives line numbers.
- **Joined answers** are the documents in `Preparation/Blender Research/Across topics/` that name the question. The map is left out, since it holds only question names.

## The list, most-cited first

### Frames 13 — differs

A File Output node fed from the compositor group's own input writes in 5.3 and not in 5.2.2 on the Mac. Reports 1 and The compositor 2 test 5.3's saving switch; Reports 11 records its refusal of a multi-layer format from a script until the media type is set. This topic did not try the JPEG preview copy in 5.3; Images 8 records it. A kernel reset is untried in 5.3, and whether a TIFF cut short mid-write opens is unsettled.

- **Joined answers:** *Running Blender with nobody there*, *Two renders in one folder*, *What the research cannot say*, *When a render ends*.
- **Plans (18):** Architecture/Facts.md (Fact 44); Architecture/Job files and outputs.md (§10.7); Architecture/Rendering.md (§3.2); Architecture/Rows/Queue & History.md (Rows 54, 58, 72, 73); Architecture/Rows/Rendering.md (Rows 17, 20); Architecture/Rows/Telegram.md (Row 272); Architecture/Rows/What a job writes.md (Rows 133, 138); Architecture/Sources/Download.sources.md (*A part-written file opens, and nothing…*); Architecture/Sources/Job files and outputs.sources.md (§8); Architecture/Sources/Queue & History.sources.md (§3); Architecture/Sources/Rendering.sources.md (§B10); Architecture/Sources/Rented-server runtime.sources.md (line 479); Architecture/Sources/State, identity and recovery.sources.md (Section 1.3, Section 6); Architecture/Telegram.md (§16); Specs/Sources/Download.sources.md (*Added when the second and third…*); Specs/Sources/Queue & History.sources.md (*Answered by the Blender research*, *Added when the second and third…*); Specs/Sources/Rendering.sources.md (*Added when the second and third…*, *Added when the carry-on figure was…*); Specs/Sources/What a job writes.sources.md (*Frames, view layers and the re-run*).
- **Code (2):** App/Job files and outputs — machine/src/reckoning.rs (lines 665, 703); App/Rendering/src/kind.rs (line 21).

### Frames 7 — not established for 5.3

What a person's own key or click does in 5.3 is untried, since Blender's simulated events were used. The frame-list answers are the same in 5.3.

- **Joined answers:** *What a file can be asked before it renders*, *What can be set from outside*, *What the research cannot say*, *While a render runs*.
- **Plans (16):** Architecture/Facts.md (Fact 127); Architecture/Job files and outputs.md (§10.1); Architecture/Render Settings.md (§2.4, §3); Architecture/Rendering.md (§1.6); Architecture/Rows/Design — Render Settings.md (Row 437); Architecture/Rows/Queue & History.md (Row 72); Architecture/Rows/Render Settings.md (Row 146); Architecture/Rows/Rendering.md (Row 14); Architecture/Rows/What a job writes.md (Row 133); Architecture/Sources/Job files and outputs.sources.md (§5); Architecture/Sources/Queue & History.sources.md (§3); Architecture/Sources/Render Settings.sources.md (§B21); Architecture/Sources/Rendering.sources.md (§B7); Specs/Sources/Queue & History.sources.md (*Answered by the Blender research*); Specs/Sources/Render Settings.sources.md (*Facts from Blender Research*); Specs/Sources/What a job writes.sources.md (*Frames, view layers and the re-run*).
- **Code (4):** App/Render Settings — machine/src/frames.rs (line 6); App/Render Settings — machine/src/refusals.rs (line 196); App/Rendering — machine/src/frames.rs (lines 16, 20, 28, 60, 62, 122); App/Rendering — machine/src/spellings.rs (lines 313, 322).

### Images 9 — differs

5.3 reports missing textures differently, against 5.2.2: EEVEE names the file, an empty slot prints nothing, and an unreadable file gets one line. The 5.2.2 run put some earlier small number gaps down to how those runs measured. Whether 5.3 keeps 5.2's texture-cache additions is not established.

- **Joined answers:** *What a render really did*, *What can be set from outside*, *What has to travel with a file*.
- **Plans (16):** Architecture/Dependency Upload & Remap.md (§3.6); Architecture/Render Settings.md (§6.1); Architecture/Rendering.md (§1.7); Architecture/Rows/Dependency Upload & Remap.md (Row 164); Architecture/Rows/Queue & History.md (Row 85); Architecture/Rows/Render Settings.md (Row 151); Architecture/Rows/Rendering.md (Row 12); Architecture/Sources/Dependency Upload & Remap.sources.md (§3); Architecture/Sources/Render Settings.sources.md (§B17); Architecture/Sources/Rendering.sources.md (§B8); Specs/Sources/Dependency Upload & Remap.sources.md (*From Blender Research*); Specs/Sources/Queue & History.sources.md (*Still open*, *Added when the second and third…*, *Added from the cross-sweep of the…*); Specs/Sources/Render Settings.sources.md (*Facts from Blender Research*); Specs/Sources/Rendering.sources.md (*Added when the Blender Research cross-sweep…*); Specs/Sources/Telegram.sources.md (line 49); Specs/Sources/What a job writes.sources.md (*Pictures, passes and formats*).
- **Code (1):** App/Render Settings — machine/src/engine.rs (line 7).

### The machine 5 — an earlier reading reopened

The original 4.5 to 5.2 Mac runs that showed no card in use are unresolved: a missed card is possible. A script changing devices inside the run took effect in 5.3 and 5.2.2 on the Mac. The answer to the question is the same in all five.

- **Joined answers:** *Running Blender with nobody there*, *What a render really did*, *What can be set from outside*, *What the research cannot say*.
- **Plans (13):** Architecture/Facts.md (Fact 114, Fact 46); Architecture/Render Settings.md (§8.1); Architecture/Rendering.md (§1.3, §1.8, §9); Architecture/Rows/Render Settings.md (Rows 157, 158); Architecture/Rows/Rendering.md (Row 8); Architecture/Server Setup.md (§2.1); Architecture/Servers.md (§8.3); Architecture/Sources/Render Settings.sources.md (§B4, §B24); Architecture/Sources/Rendering.sources.md (§B3, §B4, §B9, §M13, §P5, §U4, §U16, §C12); Architecture/Sources/Server Setup.sources.md (Fact 46); Architecture/Sources/Servers.sources.md (§6.15, §6.16); Specs/Sources/Render Settings.sources.md (*Facts from Blender Research*); Specs/Sources/Rendering.sources.md (lines 5, 9, 10, 11, 15).
- **Code (2):** App/Rendering — machine/src/spellings.rs (lines 382, 388, 408, 546, 560); App/Rendering — machine/src/technology.rs (line 9).

### Dependencies 17 — not established for 5.3

Whether 5.3 has the point-cache compression setting was not tested.

- **Joined answers:** *Two renders in one folder*, *What a file can be asked before it renders*, *What can be set from outside*, *What has to travel with a file*, *What the research cannot say*, *When a render ends*, *Where a render's files go*, *While a render runs*.
- **Plans (14):** Architecture/Facts.md (Fact 77, *Named by a sweep, and not…*); Architecture/Job files and outputs.md (§4.2, §5.3); Architecture/One box with Blender on it — the reading, the listing, and the names that do not come back.md (line 151); Architecture/Rendering.md (§1.3); Architecture/Rows/Queue & History.md (Row 72); Architecture/Rows/Render Settings.md (Rows 140, 152); Architecture/Rows/Rendering.md (Rows 6, 13); Architecture/Rows/What a job writes.md (Rows 119, 127); Architecture/Sources/Job files and outputs.sources.md (§3, §4, §10); Architecture/Sources/Queue & History.sources.md (§3); Architecture/Sources/Render Settings.sources.md (§B1); Architecture/Sources/Rendering.sources.md (§B3); Specs/Sources/Dependency Upload & Remap.sources.md (*From Blender Research*); Specs/Sources/What a job writes.sources.md (*Paths, names, and the redirect*).
- **Code (1):** App/Rendering — machine/src/spellings.rs (lines 428, 435).

### Settings 16 — differs

EEVEE has a time limit in 5.3 alone; the tested Cycles sampling rules agree. Reports 11's same-Mac control establishes a change in Cycles' printed ending: 5.3 omits the closing zero-sample line under adaptive sampling and the time limit, while 5.2.2 prints it. Cancellation remains unresolved across versions.

- **Joined answers:** *What a file can be asked before it renders*, *What a render really did*, *What can be set from outside*, *What the research cannot say*, *When a render ends*, *While a render runs*.
- **Plans (12):** Architecture/Facts.md (*Named by a sweep, and not…*, Fact 127); Architecture/Render Settings.md (*What binds this before anything is…*, §1.1, §2.4, §5, §9.2, §13); Architecture/Rendering.md (§1.3); Architecture/Rows/Design — Render Settings.md (Rows 369, 437); Architecture/Rows/Queue & History.md (Row 62); Architecture/Rows/Render Settings.md (Rows 146, 147, 150, 152); Architecture/Rows/Rendering.md (Row 6); Architecture/Sources/Job files and outputs.sources.md (§8); Architecture/Sources/Queue & History.sources.md (§3); Architecture/Sources/Render Settings.sources.md (§B1, §B13, §B14, §B15, §B23a, §B29, §U10, §U20); Architecture/Sources/Rendering.sources.md (§B3); Specs/Sources/Render Settings.sources.md (*Facts from Blender Research*).
- **Code (2):** App/Render Settings — machine/src/samples.rs (lines 9, 119, 150); App/Render Settings — machine/src/spellings.rs (line 170).

### What actually ran 1 — differs

The hardware ray-tracing line in Cycles' device report is indented differently in 5.3 than in 5.2.2; nothing else in the block differs.

- **Joined answers:** *What a render really did*, *What the research cannot say*.
- **Plans (10):** Architecture/Rendering.md (§1.2, §1.3, §1.8, §9); Architecture/Rows/Queue & History.md (Row 69); Architecture/Rows/Rendering.md (Row 10); Architecture/Sources/Queue & History.sources.md (§3); Architecture/Sources/Rendering.sources.md (§B2, §B3, §U8, §U9); Architecture/Sources/Rented-server runtime.sources.md (line 473); Architecture/Sources/Server Setup.sources.md (*The device line a finished job…*); Specs/Sources/Queue & History.sources.md (*Added when the second and third…*); Specs/Sources/Rendering.sources.md (*Added when the second and third…*); Specs/Sources/Server Setup.sources.md (*Added when the second and third…*).
- **Code (4):** App/Queue & History — machine/src/acting/reader.rs (lines 79, 469); App/Rendering — machine/src/line.rs (line 71); App/Rendering — machine/src/spellings.rs (lines 228, 255, 293); App/Server Setup/src/statements.rs (line 360).

### Reading a file 9 — an earlier reading reopened

The command that swaps an image's file ran with no window in 5.3 and 5.2.2 once handed an Image editor; the earlier no-window refusals had none handed to them, so they do not show it cannot run. The unsaved-file substitution was not asked of 5.3.

- **Joined answers:** *Running Blender with nobody there*, *What a file can be asked before it renders*, *What can be set from outside*, *What has to travel with a file*.
- **Plans (12):** Architecture/Facts.md (Fact 77, Fact 116); Architecture/Job files and outputs.md (§5.1, §5.5); Architecture/One box with Blender on it — the reading, the listing, and the names that do not come back.md (line 151); Architecture/Rows/Dependency Upload & Remap.md (Rows 162, 174, 175, 176); Architecture/Rows/Render Settings.md (Row 152); Architecture/Rows/Rendering.md (Rows 6, 13); Architecture/Rows/What a job writes.md (Rows 118, 119, 120, 313); Architecture/Sources/Dependency Upload & Remap.sources.md (§2.5); Architecture/Sources/Job files and outputs.sources.md (§3, §4); Architecture/Sources/Render Settings.sources.md (§B4); Specs/Sources/Dependency Upload & Remap.sources.md (*From Blender Research*); Specs/Sources/What a job writes.sources.md (*Paths, names, and the redirect*).
- **Code:** none.

### The machine 7 — differs

Render Audio's file size, EEVEE's missing-texture words and the automatic-execution dialog's controls change in 5.3, against 5.2.2 on the Mac; by release note, no Dock icon in background mode and viewport renders paused during a final render. Whether a hang-up ends a background render is not settled: 5.3 and 5.2.2 ended, the earlier runs did not.

- **Joined answers:** *Running Blender with nobody there*, *Two renders in one folder*, *What can be set from outside*, *What the research cannot say*, *When a render ends*, *While a render runs*.
- **Plans (11):** Architecture/Render Settings.md (§4.1); Architecture/Rendering.md (§1.1, §1.3, §3.3); Architecture/Rows/Queue & History.md (Row 67); Architecture/Rows/Rendering.md (Rows 18, 19); Architecture/Sources/Queue & History.sources.md (§3); Architecture/Sources/Render Settings.sources.md (§B12, §P2); Architecture/Sources/Rendering.sources.md (§B1, §B4, §B11, §P4); Specs/Rendering.md (*The file's own scripts run*); Specs/Sources/Queue & History.sources.md (*Added when the second and third…*); Specs/Sources/Rendering.sources.md (*Added when the second and third…*); Specs/Sources/What a job writes.sources.md (*Pictures, passes and formats*).
- **Code (1):** App/Rendering — machine/src/spellings.rs (lines 147, 157).

### Settings 17 — not established for 5.3

A relative output path with no file loaded, which fails in 5.0 to 5.2, was not tried in 5.3.

- **Joined answers:** *Running Blender with nobody there*, *Two renders in one folder*, *What a file can be asked before it renders*, *What a render really did*, *What can be set from outside*, *What has to travel with a file*, *What the research cannot say*, *When a render ends*, *Where a render's files go*.
- **Plans (7):** Architecture/Facts.md (Fact 127); Architecture/Render Settings.md (*What binds this before anything is…*, §5, §9.2, §13); Architecture/Rendering.md (*What binds this before anything is…*, §1.3, §1.4, §1.8, §9); Architecture/Rows/Design — Render Settings.md (Row 366); Architecture/Sources/Render Settings.sources.md (§B9, §B28, §B29, §U18); Architecture/Sources/Rendering.sources.md (§B13, §U17, §C12); Architecture/What Build inherits.md (*What is still owed at the…*).
- **Code (4):** App/Render Settings — machine/src/samples.rs (line 170); App/Render Settings — machine/src/spellings.rs (lines 2, 21, 52, 55, 110, 220, 344, 367); App/Rendering — machine/src/line.rs (line 23); App/Rendering — machine/src/spellings.rs (lines 10, 38).

### Settings 1 — differs

5.3 adds one launch argument, about not falling back to another drawing backend, read from help text only; none of the render arguments changes.

- **Joined answers:** *What a file can be asked before it renders*, *What a render really did*, *What can be set from outside*, *What has to travel with a file*, *When a render ends*.
- **Plans (8):** Architecture/Facts.md (*Named by a sweep, and not…*); Architecture/Render Settings.md (§4, §4.2, §5, §13); Architecture/Rendering.md (§1.1, §1.4, §1.6); Architecture/Rows/Render Settings.md (Rows 147, 149, 150, 152); Architecture/Rows/Rendering.md (Row 1); Architecture/Sources/Render Settings.sources.md (§B1, §B2, §B4, §B7, §B9, §B11, §U18); Architecture/Sources/Rendering.sources.md (§B3, §B5, §B6); Specs/Sources/Render Settings.sources.md (*Facts from Blender Research*, *Facts owned elsewhere*).
- **Code (2):** App/Rendering — machine/src/material.rs (line 599); App/Rendering — machine/src/spellings.rs (lines 269, 275, 279, 283, 288, 294, 311, 322).

### What a render demands 3 — not established for 5.3

The multi-layer format name was not tried in this topic's 5.3 runs (Reports 11 records 5.3 refusing it from a script; Settings 17 records the command-line route working), and 5.3 was not offered EEVEE's old name here (Images 9 records it refused from a script).

- **Joined answers:** *What a render really did*, *What can be set from outside*, *What the research cannot say*.
- **Plans (9):** Architecture/Facts.md (*Named by a sweep, and not…*); Architecture/Render Settings.md (§4, §8.1); Architecture/Rendering.md (§1.1, §1.3, §1.4, §1.6, §9); Architecture/Rows/Render Settings.md (Rows 146, 147, 150, 151, 152, 157); Architecture/Rows/Rendering.md (Rows 1, 6, 9, 22); Architecture/Sources/Render Settings.sources.md (§B1, §B2, §B3, §B4, §B11, §B24); Architecture/Sources/Rendering.sources.md (§B3, §B4, §B5, §B6, §U16); Specs/Sources/Render Settings.sources.md (*Facts owned elsewhere*); Specs/Sources/Rendering.sources.md (*Added when the second and third…*, line 13).
- **Code (1):** App/Rendering — machine/src/spellings.rs (lines 275, 283).

### Settings 13 — not established for 5.3

Whether 5.3 accepts EEVEE's old name on the command line was not tried; Images 9 records its refusal from a script. What a render demands 1 and 4 establish Cycles' feature-set setting as absent in 5.3 on the Mac, as in 5.0 to 5.2.

- **Joined answers:** *Running Blender with nobody there*, *What a file can be asked before it renders*, *What a render really did*, *What can be set from outside*, *What has to travel with a file*, *What the research cannot say*.
- **Plans (9):** Architecture/Render Settings.md (§1.4, §6.1, §6.2); Architecture/Rendering.md (§1.7); Architecture/Rows/Render Settings.md (Rows 142, 151, 152, 154); Architecture/Rows/Rendering.md (Row 12); Architecture/Rows/Server Setup.md (Row 94); Architecture/Sources/Render Settings.sources.md (§B5, §B17, §B18, §B19, §U23); Architecture/Sources/Rendering.sources.md (§B8); Specs/Sources/Render Settings.sources.md (*Facts from Blender Research*); Specs/Sources/Rendering.sources.md (*Added when the Blender Research cross-sweep…*).
- **Code (1):** App/Render Settings — machine/src/engine.rs (line 7).

### Frames 8 — differs

Opening the Frame Range panel by name crashed 5.2 (5.2.0 on Linux, 5.2.2 on the Mac) and not 5.3 on the Mac.

- **Joined answers:** *What a file can be asked before it renders*, *While a render runs*.
- **Plans (8):** Architecture/Render Settings.md (*What binds this before anything is…*, §2.4, §3, §4.1); Architecture/Rows/Design — Render Settings.md (Row 437); Architecture/Rows/Queue & History.md (Row 72); Architecture/Rows/Render Settings.md (Row 140); Architecture/Sources/Queue & History.sources.md (§3); Architecture/Sources/Render Settings.sources.md (§B22, §B27); Specs/Sources/Queue & History.sources.md (*Answered by the Blender research*); Specs/Sources/Render Settings.sources.md (*Facts from Blender Research*).
- **Code (2):** App/Render Settings — machine/src/frames.rs (lines 176, 200); App/Render Settings — machine/src/refusals.rs (line 201).

### Reports 9 — not established for 5.3

The 5.3 Render menu's sequencer entries are not settled, whether 5.3's printing of a render engine's own status text is new is not established, and a person's Escape was not tried.

- **Joined answers:** *Running Blender with nobody there*, *What can be set from outside*, *What the research cannot say*, *While a render runs*.
- **Plans (9):** Architecture/Rendering.md (§1.1); Architecture/Rows/Queue & History.md (Rows 60, 61); Architecture/Rows/Render Settings.md (Row 152); Architecture/Rows/Rendering.md (Row 11); Architecture/Sources/Queue & History.sources.md (§3); Architecture/Sources/Rendering.sources.md (§B1); Specs/Sources/Queue & History.sources.md (*Added from the cross-sweep of the…*); Specs/Sources/Render Settings.sources.md (*Facts from Blender Research*); Specs/Sources/Rendering.sources.md (*Added when the Blender Research cross-sweep…*).
- **Code (1):** App/Rendering — machine/src/line.rs (line 10).

### Dependencies 10 — an earlier reading reopened

A connected shader-script node wrote its compiled file beside the source in 5.3 and 5.2.2 on the Mac. Earlier runs finding no compiled file did not record a connected node, so they do not establish that Blender cannot write it.

- **Joined answers:** *What has to travel with a file*, *Where a render's files go*.
- **Plans (9):** Architecture/Dependency Upload & Remap.md (§3.2); Architecture/Facts.md (Fact 116); Architecture/Job files and outputs.md (§2.4, §5.5); Architecture/Rows/Dependency Upload & Remap.md (Row 174); Architecture/Rows/Rendering.md (Row 13); Architecture/Rows/What a job writes.md (Rows 118, 313); Architecture/Sources/Job files and outputs.sources.md (§4); Specs/Sources/Dependency Upload & Remap.sources.md (*From Blender Research*); Specs/Sources/What a job writes.sources.md (*Paths, names, and the redirect*).
- **Code (1):** App/Dependency Upload & Remap — machine/src/check_script.rs (line 646).

### Writing 9 — differs

5.3's release notes add project variables for File Output paths. This topic did not check the path-walking options; Reading a file 5 and 9 record them in 5.3.

- **Joined answers:** *What can be set from outside*, *Where a render's files go*.
- **Plans (9):** Architecture/Job files and outputs.md (§5.1, §5.2); Architecture/Rows/Dependency Upload & Remap.md (Row 174); Architecture/Rows/Rendering.md (Row 6); Architecture/Rows/What a job writes.md (Rows 118, 120); Architecture/Sources/Job files and outputs.sources.md (§4, §10); Specs/Sources/Dependency Upload & Remap.sources.md (*From Blender Research*); Specs/Sources/Download.sources.md (line 11); Specs/Sources/Rendering.sources.md (line 16); Specs/Sources/What a job writes.sources.md (*Paths, names, and the redirect*).
- **Code:** none.

### Frames 11 — not established for 5.3

Whether Escape stops a 5.3 render is not established: Blender's simulated Escape did not, in 5.2.2 either.

- **Joined answers:** *Running Blender with nobody there*, *When a render ends*.
- **Plans (9):** Architecture/Rows/Queue & History.md (Row 58); Architecture/Rows/Telegram.md (Row 272); Architecture/Sources/Job files and outputs.sources.md (§8); Architecture/Sources/Telegram.sources.md (§8); Architecture/Telegram.md (§16); Specs/Sources/Queue & History.sources.md (*Answered by the Blender research*, *Added when the second and third…*); Specs/Sources/Rendering.sources.md (*Added when the second and third…*, lines 17, 18); Specs/Sources/Telegram.sources.md (line 67); Specs/Sources/What a job writes.sources.md (*Frames, view layers and the re-run*).
- **Code:** none.

### Reports 11 — differs

Under adaptive sampling and the time limit, 5.3 omits the closing zero-sample line that 5.2.2 prints on the same Mac; cancellation had no matched control. The Sample Count pass is absent here and present under Images 10's different setup; what decides this is unresolved. The script-set multi-layer format refusal also occurs in 5.2.2, so it is not a 5.3 change.

- **Joined answers:** *What a render really did*, *What can be set from outside*, *When a render ends*, *While a render runs*.
- **Plans (8):** Architecture/Facts.md (Fact 127); Architecture/Render Settings.md (§5); Architecture/Rows/Queue & History.md (Rows 61, 62); Architecture/Rows/Render Settings.md (Row 150); Architecture/Sources/Queue & History.sources.md (§3); Architecture/Sources/Render Settings.sources.md (§B16); Architecture/Sources/Rendering.sources.md (§B5); Specs/Sources/Queue & History.sources.md (*Still open*, *Added from the cross-sweep of the…*).
- **Code (1):** App/Render Settings — machine/src/samples.rs (line 10).

### Dependencies 5 — differs

5.3 adds project variables, by release note, and a template in a geometry-nodes bake folder, tested on the Mac.

- **Joined answers:** *What has to travel with a file*.
- **Plans (7):** Architecture/Dependency Upload & Remap.md (§3.2); Architecture/Job files and outputs.md (§7.1, §14); Architecture/Rows/What a job writes.md (Row 121); Architecture/Sources/Dependency Upload & Remap.sources.md (§3); Architecture/Sources/Job files and outputs.sources.md (§5); Specs/Sources/Dependency Upload & Remap.sources.md (*From the Creator*, *From Blender Research*); Specs/Sources/What a job writes.sources.md (*Paths, names, and the redirect*).
- **Code (2):** App/Job files and outputs — machine/src/reading.rs (line 339); App/Job files and outputs — machine/src/redirect.rs (line 606).

### Reading a file 3 — differs

By 5.3's release notes, the compositor renders scenes referenced inside node groups.

- **Joined answers:** *What a file can be asked before it renders*.
- **Plans (7):** Architecture/Facts.md (*Named by a sweep, and not…*); Architecture/Render Settings.md (*What binds this before anything is…*, §1.4, §4, §4.1, §4.3, §9.2); Architecture/Rows/Render Settings.md (Rows 140, 142, 147, 149); Architecture/Sources/Queue & History.sources.md (§4); Architecture/Sources/Render Settings.sources.md (§B5, §B7, §B8, §B9, §B10, §B11, *What this test does not cover*, §U7, §U8); Specs/Sources/Queue & History.sources.md (*Still open*); Specs/Sources/Render Settings.sources.md (*Facts from Blender Research*).
- **Code (1):** App/Render Settings — machine/src/view_layers.rs (lines 8, 197).

### Reports 3 — differs

5.3 omits the closing zero-sample line in the controlled early-convergence, time-limit and completed tiled-frame tests; cancellation had no matched control (Reports 11). What the Status Bar percentage tracks across frames in 5.3 is not established.

- **Joined answers:** *What the research cannot say*, *While a render runs*.
- **Plans (7):** Architecture/Facts.md (Fact 127); Architecture/Queue & History.md (§5); Architecture/Render Settings.md (§2.4); Architecture/Rows/Queue & History.md (Rows 60, 61, 62, 65); Architecture/Sources/Queue & History.sources.md (§3); Architecture/Sources/Render Settings.sources.md (§B23a); Specs/Sources/Queue & History.sources.md (*Answered by the Blender research*).
- **Code (1):** App/Queue & History — machine/src/acting/reader.rs (lines 31, 45, 48, 74, 149, 363, 399, 450, 452, 456, 459, 462, 498, 516).

### Frames 15 — not established for 5.3

Whether a person's Escape stops a 5.3 render is not established: Blender's simulated Escape stopped nothing in 5.2.2 or 5.3.

- **Joined answers:** *Running Blender with nobody there*, *Two renders in one folder*, *What a file can be asked before it renders*, *What a render really did*, *What can be set from outside*, *What has to travel with a file*, *What the research cannot say*, *When a render ends*, *Where a render's files go*, *While a render runs*.
- **Plans (7):** Architecture/Facts.md (Fact 106); Architecture/Rendering.md (§1.8, §3.3, §9); Architecture/Sources/Queue & History.sources.md (§3); Architecture/Sources/Rendering.sources.md (§U1, §C13); Architecture/Sources/Server Setup.sources.md (line 566); Architecture/The Pile 3 batch — a full mount under a running supervisor, an interrupted heavy frame, and an hour of silence.md (§3.3); Architecture/What Build inherits.md (*Blender: the ceiling, the two vocabularies,…*).
- **Code (1):** App/Rendering — machine/src/stop.rs (lines 20, 43).

### Reports 2 — differs

For the no-camera failure, 5.3's command-line route uses the window's wording, while its script-started route keeps the earlier wording. A key-started failure's reports list was not compared with 5.2.

- **Joined answers:** *Running Blender with nobody there*, *What the research cannot say*, *When a render ends*.
- **Plans (8):** Architecture/Rendering.md (§1.2); Architecture/Rows/Queue & History.md (Row 68); Architecture/Rows/Server Setup.md (Row 94); Architecture/Sources/Queue & History.sources.md (§3); Architecture/Sources/Rendering.sources.md (§B2); Architecture/Sources/Rented-server runtime.sources.md (line 475); Specs/Sources/Queue & History.sources.md (*Answered by the Blender research*); Specs/Sources/Rendering.sources.md (lines 14, 23).
- **Code:** none.

### Frames 5 — not established for 5.3

5.3's stopped videos were read only by Blender's own reader; the earlier Mac files were also played in QuickTime.

- **Joined answers:** *What the research cannot say*, *When a render ends*, *Where a render's files go*, *While a render runs*.
- **Plans (7):** Architecture/Rows/Queue & History.md (Row 74); Architecture/Rows/What a job writes.md (Rows 129, 138); Architecture/Sources/Download.sources.md (*The video destination's file size while…*); Architecture/Sources/Job files and outputs.sources.md (§5, §8); Architecture/Sources/Queue & History.sources.md (§3); Specs/Sources/Queue & History.sources.md (*Answered by the Blender research*); Specs/Sources/What a job writes.sources.md (*Frames, view layers and the re-run*).
- **Code:** none.

### Writing 13 — differs

5.3's list of launch facts a script can read adds two names, set against 5.2's record.

- **Joined answers:** *Running Blender with nobody there*, *Two renders in one folder*, *What a file can be asked before it renders*, *What a render really did*, *What can be set from outside*, *What has to travel with a file*, *What the research cannot say*, *Where a render's files go*.
- **Plans (5):** Architecture/Job files and outputs.md (§5.2, §14); Architecture/Rendering.md (§9); Architecture/Sources/Job files and outputs.sources.md (§10); Architecture/Sources/Rendering.sources.md (§U3a, §B25); Architecture/What Build inherits.md (*Blender: the ceiling, the two vocabularies,…*).
- **Code (2):** App/Job files and outputs — machine/src/redirect.rs (line 350); App/Rendering — machine/src/spellings.rs (lines 153, 448, 453).

### File output nodes 13 — differs

In 5.3 a File Output node writes only from a compositor effect switched on for render.

- **Joined answers:** *What a file can be asked before it renders*, *What the research cannot say*, *When a render ends*, *Where a render's files go*.
- **Plans (7):** Architecture/Job files and outputs.md (§14); Architecture/Rows/Rendering.md (Row 17); Architecture/Rows/What a job writes.md (Row 124); Architecture/Sources/Download.sources.md (*The frame order, and each frame's…*); Architecture/Sources/Job files and outputs.sources.md (§3, §10); Specs/Sources/Rendering.sources.md (*Added when the second and third…*); Specs/Sources/What a job writes.sources.md (*The two routes, and whether either…*, *What a file output node can…*, *Frames, view layers and the re-run*, *Starting a render*).
- **Code:** none.

### Images 1 — differs

5.3 adds a compositor effects stack and changes the default group, keeping the older setting with a warning.

- **Joined answers:** *What a file can be asked before it renders*, *Where a render's files go*.
- **Plans (7):** Architecture/One box with Blender on it — the reading, the listing, and the names that do not come back.md (line 551); Architecture/Rows/Queue & History.md (Row 85); Architecture/Rows/What a job writes.md (Rows 128, 130, 132); Architecture/Sources/Job files and outputs.sources.md (§6); Architecture/Sources/Queue & History.sources.md (§3); Specs/Sources/Queue & History.sources.md (*Answered by the Blender research*); Specs/Sources/What a job writes.sources.md (*The two routes, and whether either…*, *Frames, view layers and the re-run*, *Pictures, passes and formats*).
- **Code:** none.

### Writing 6 — not established for 5.3

A person's key press was not tried in 5.3, and its viewport menu names were read from code only. Choosing a video format directly is refused in 5.2.2 and 5.3, where the earlier runs conflict.

- **Joined answers:** *Running Blender with nobody there*, *Where a render's files go*.
- **Plans (6):** Architecture/Rendering.md (§1.6); Architecture/Rows/Rendering.md (Row 14); Architecture/Rows/What a job writes.md (Row 138); Architecture/Sources/Rendering.sources.md (§B7); Specs/Sources/Rendering.sources.md (line 26); Specs/Sources/What a job writes.sources.md (*Starting a render*).
- **Code (1):** App/Rendering — machine/src/frames.rs (lines 9, 11, 25).

### The machine 4 — not established for 5.3

5.3's final Metal progress marker was not recorded, and the removed-driver warning was not tried in 5.3.

- **Joined answers:** *Running Blender with nobody there*, *What a render really did*, *What can be set from outside*, *What the research cannot say*, *When a render ends*.
- **Plans (7):** Architecture/Rendering.md (§1.8); Architecture/Rows/Reporting a problem.md (Row 463); Architecture/Rows/Server Setup.md (Row 89); Architecture/Sources/Rendering.sources.md (§B9); Architecture/Sources/Server Setup.sources.md (*A machine rendering on the processor…*); Specs/Sources/Rendering.sources.md (lines 13, 14); Specs/Sources/Server Setup.sources.md (*Facts about Blender*).
- **Code:** none.

### Reading a file 11 — an earlier reading reopened

The sequencer's reload commands and the shader-script update ran with no window in 5.3 and 5.2.2 once handed the right editor; the earlier refusals in 5.0 to 5.2.0 do not show they cannot. A mistyped option's error was not asked of 5.3.

- **Joined answers:** *Running Blender with nobody there*, *What has to travel with a file*, *What the research cannot say*.
- **Plans (7):** Architecture/Dependency Upload & Remap.md (§7.2); Architecture/Rows/Dependency Upload & Remap.md (Row 173); Architecture/Rows/File Browser.md (Row 195); Architecture/Sources/Dependency Upload & Remap.sources.md (§3); Architecture/Sources/File Browser.sources.md (§G2); Specs/Sources/Dependency Upload & Remap.sources.md (*From Blender Research*); Specs/Sources/File Browser.sources.md (*Added when the second and third…*).
- **Code:** none.

### Images 3 — differs

The 5.3 beta adds two build options that are not picture formats; its release notes announce better multi-layer OpenEXR reading and writing with lossy DWA compression, untested.

- **Joined answers:** *What a file can be asked before it renders*, *What can be set from outside*, *Where a render's files go*.
- **Plans (6):** Architecture/Rendering.md (§1.4); Architecture/Sources/Rendering.sources.md (§B5, §C8); Architecture/Telegram.md (§12); Specs/Sources/Download.sources.md (line 9); Specs/Sources/Telegram.sources.md (line 45); Specs/Sources/What a job writes.sources.md (*Pictures, passes and formats*).
- **Code (1):** App/Desktop app and delivery/src/surface/render_settings.rs (line 6974).

### Frames 4 — not established for 5.3

Two Blenders sharing one output folder were not run in 5.3.

- **Joined answers:** *Two renders in one folder*, *What can be set from outside*, *What the research cannot say*, *When a render ends*, *While a render runs*.
- **Plans (5):** Architecture/Rows/Queue & History.md (Rows 60, 72); Architecture/Rows/What a job writes.md (Row 133); Architecture/Sources/Queue & History.sources.md (§3); Specs/Sources/Queue & History.sources.md (*Answered by the Blender research*); Specs/Sources/What a job writes.sources.md (*Frames, view layers and the re-run*).
- **Code (1):** App/Rendering — machine/src/spellings.rs (lines 426, 435).

### Writing 11 — differs

The tested uncompressed OpenEXR is one byte longer in 5.3, against 5.2.2. Two Blenders writing together was not tried in 5.3, and the two-gigabyte split was not run in this topic's 5.3 runs (Frames 5 holds a 5.3 run on the Mac).

- **Joined answers:** *Two renders in one folder*, *What the research cannot say*, *When a render ends*, *Where a render's files go*, *While a render runs*.
- **Plans (6):** Architecture/Download.md (§4.1); Architecture/Rows/Queue & History.md (Rows 54, 72, 74); Architecture/Rows/What a job writes.md (Row 133); Architecture/Sources/Download.sources.md (*The no-overlap inference — REWRITTEN 2026-09-01:…*, *The no-overlap inference — an assumption,…*); Architecture/Sources/Job files and outputs.sources.md (§8); Architecture/Sources/Queue & History.sources.md (§3, §4).
- **Code:** none.

### Builds 9 — not established for 5.3

A cut-off copy of 5.3 was redone cleanly, but whether the result starts was not established.

- **Joined answers:** *When a render ends*.
- **Plans (6):** Architecture/Rows/Queue & History.md (Row 67); Architecture/Rows/Server Setup.md (Rows 95, 101, 114); Architecture/Server Setup.md (§4.3); Architecture/Sources/Queue & History.sources.md (§3); Architecture/Sources/Server Setup.sources.md (*The interrupted install re-run over the…*, line 577); Specs/Sources/Server Setup.sources.md (*Facts about Blender*, *Facts about rented machines, not about…*).
- **Code:** none.

### Settings 14 — differs

5.3 offers four more render presets than 5.2.2 on the Mac; the accepted range is unchanged.

- **Joined answers:** *What a file can be asked before it renders*.
- **Plans (5):** Architecture/Coverage.md (Row 366); Architecture/Render Settings.md (§1.1, §5, §13); Architecture/Sources/Render Settings.sources.md (§B23, §U18); Specs/Sources/Render Settings.sources.md (*Facts owned elsewhere*); Specs/Sources/Rendering.sources.md (*Added when Simplify was ruled onto…*).
- **Code (1):** App/Render Settings — machine/src/refusals.rs (line 26).

### Reading a file 1 — differs

A compressed save's opening bytes show readable text in 5.3 and not in 5.2.2 on the Mac; 5.2.0's case is disputed; 5.3's previews were asked another way.

- **Joined answers:** *What a file can be asked before it renders*, *What the research cannot say*.
- **Plans (6):** Architecture/File Browser.md (*What binds this before anything is…*, §1.3); Architecture/Rows/File Browser.md (Row 182); Architecture/Rows/Render Settings.md (Row 140); Architecture/Sources/File Browser.sources.md (§B3, *K. Assumptions and unresolved, gathered*); Specs/Sources/Dependency Upload & Remap.sources.md (*From the Creator*); Specs/Sources/File Browser.sources.md (line 6).
- **Code:** none.

### Reports 5 — differs

The opening list includes this question in the closing zero-sample line omitted in 5.3's controlled early-convergence, time-limit and tiled-frame tests (Reports 11 and 7). Whether the tooltip and render window's estimate disagree in 5.3 was not caught.

- **Joined answers:** *While a render runs*.
- **Plans (5):** Architecture/Queue & History.md (§5); Architecture/Rows/Design — Queue & History.md (Row 432); Architecture/Rows/Queue & History.md (Row 62); Architecture/Sources/Queue & History.sources.md (§3); Specs/Sources/Queue & History.sources.md (*Answered by the Blender research*).
- **Code (1):** App/Queue & History — machine/src/acting/reader.rs (lines 65, 158).

### Reports 6 — not established for 5.3

The graphics-card memory readout was never seen in 5.3.

- **Joined answers:** *What a render really did*, *What the research cannot say*, *When a render ends*, *While a render runs*.
- **Plans (6):** Architecture/Rows/Queue & History.md (Row 66); Architecture/Rows/Servers.md (Row 27); Architecture/Sources/Queue & History.sources.md (§3); Architecture/Sources/Servers.sources.md (§6.15); Specs/Sources/Queue & History.sources.md (*Answered by the Blender research*); Specs/Sources/Servers.sources.md (line 22).
- **Code:** none.

### Reading a file 4 — differs

By 5.3's release notes, bake-folder templates and project variables are added. Neither was tested in this topic's runs; Dependencies 5 and Node setups 1 test a bake-folder template in 5.3.

- **Joined answers:** *What a file can be asked before it renders*, *Where a render's files go*.
- **Plans (5):** Architecture/Job files and outputs.md (§7.1); Architecture/Rows/Render Settings.md (Row 140); Architecture/Rows/What a job writes.md (Row 127); Architecture/Sources/Job files and outputs.sources.md (§5); Specs/Sources/Dependency Upload & Remap.sources.md (*From Blender Research*).
- **Code (1):** App/Rendering — machine/src/spellings.rs (lines 444, 453).

### Reading a file 8 — differs

By 5.3's release notes, only directly linked packed data is saved; untested.

- **Joined answers:** *What has to travel with a file*.
- **Plans (6):** Architecture/Dependency Upload & Remap.md (§3.4); Architecture/Rows/Dependency Upload & Remap.md (Row 162); Architecture/Rows/File Browser.md (Row 195); Architecture/Sources/Dependency Upload & Remap.sources.md (§3); Specs/Sources/Dependency Upload & Remap.sources.md (*From Blender Research*); Specs/Sources/File Browser.sources.md (line 24).
- **Code:** none.

### Reports 1 — differs

The tested Cycles scene with its camera deleted ends a command-line render in 5.3 reporting success, writing nothing and running none of the render's event scripts, where 5.2.2 on the Mac failed it. An unwritable folder was not run here in 5.3. The compositor 2 records a File Output node writing both frames with saving off in 5.3 on the Mac, while the scene's own output wrote nothing.

- **Joined answers:** *What a render really did*, *What the research cannot say*, *When a render ends*.
- **Plans (6):** Architecture/Facts.md (*Named by a sweep, and not…*); Architecture/Rows/Queue & History.md (Row 67); Architecture/Rows/Reporting a problem.md (Row 463); Architecture/Sources/Queue & History.sources.md (§3); Specs/Sources/Queue & History.sources.md (*Answered by the Blender research*); Specs/Sources/Rendering.sources.md (line 23).
- **Code:** none.

### Frames 2 — not established for 5.3

Whether a person's Escape stops a 5.3 render is not established; Blender's simulated Escape stopped nothing in 5.3 or 5.2.2 on the Mac. The tested ways of resuming still start a fresh render.

- **Joined answers:** *When a render ends*.
- **Plans (6):** Architecture/Rows/Queue & History.md (Row 71); Architecture/Rows/Telegram.md (Row 275); Architecture/Sources/Job files and outputs.sources.md (§8); Architecture/Sources/Queue & History.sources.md (§3); Specs/Sources/Queue & History.sources.md (*Answered by the Blender research*); Specs/Sources/Telegram.sources.md (line 65).
- **Code:** none.

### Settings 10 — differs

5.3's release notes record their own compatibility breaks. 5.2.2 opened a 5.3 file with a warning and rendered it. 5.3 opening a newer file is not established.

- **Joined answers:** *What a file can be asked before it renders*.
- **Plans (4):** Architecture/Render Settings.md (*What binds this before anything is…*, §7.2); Architecture/Rows/Render Settings.md (Row 156); Architecture/Sources/Render Settings.sources.md (§B25, §U1); Specs/Sources/Render Settings.sources.md (*Facts from Blender Research*).
- **Code (1):** App/Render Settings/src/version.rs (line 297).

### Reports 12 — differs

The camera-deleted scene ends reporting success in 5.3, where 5.2.2 failed it. A hang-up signal, sent to 5.3 alone, kills it. An unwritable folder was not run in 5.3.

- **Joined answers:** *Running Blender with nobody there*, *Two renders in one folder*, *What a file can be asked before it renders*, *What a render really did*, *What can be set from outside*, *What has to travel with a file*, *What the research cannot say*, *When a render ends*, *Where a render's files go*, *While a render runs*.
- **Plans (3):** Architecture/Rendering.md (§1.8); Architecture/Sources/Rendering.sources.md (§C12); Architecture/What Build inherits.md (*Blender: the ceiling, the two vocabularies,…*).
- **Code (2):** App/Rendering — machine/src/spellings.rs (lines 10, 180, 188, 288, 294, 299, 306); App/Rendering/src/arriving_later.rs (line 194).

### Builds 6 — not established for 5.3

Copying a Blender over a running one was not tried for 5.3.

- **Joined answers:** *Two renders in one folder*, *What the research cannot say*, *When a render ends*, *While a render runs*.
- **Plans (5):** Architecture/Rows/Queue & History.md (Rows 60, 67, 68); Architecture/Rows/Server Setup.md (Row 95); Architecture/Sources/Queue & History.sources.md (§3); Architecture/Sources/Server Setup.sources.md (*Installing beside a running render, and…*); Specs/Sources/Server Setup.sources.md (*Facts about Blender*, line 7).
- **Code:** none.

### Add-ons 10 — an earlier reading reopened

On the Mac, 5.2.2 and 5.3 both draw no row for four shipped add-ons in the window's Add-ons list, where the earlier Linux runs drew every one; whether the machine or the build decides is open.

- **Joined answers:** *Running Blender with nobody there*.
- **Plans (4):** Architecture/Rows/Server Setup.md (Rows 99, 100); Architecture/Server Setup.md (§6); Architecture/Sources/Server Setup.sources.md (lines 571, 573); Specs/Sources/Server Setup.sources.md (*Facts about Blender*, *The add-on questions — where they…*).
- **Code (1):** App/Server Setup — machine/src/addons.rs (line 212).

### Node setups 1 — differs

5.3 adds an Import node for SPZ files, which 5.2.2 lacks, and templates in bake folders. The earlier one-file bake result is not a rule: 5.2.2 wrote two files when given a geometry item.

- **Joined answers:** *What a render really did*, *What has to travel with a file*, *What the research cannot say*.
- **Plans (5):** Architecture/Dependency Upload & Remap.md (§3.2, §7.2); Architecture/Rows/Dependency Upload & Remap.md (Rows 160, 173); Architecture/Rows/Rendering.md (Row 13); Architecture/Sources/Dependency Upload & Remap.sources.md (§3); Specs/Sources/Dependency Upload & Remap.sources.md (*From Blender Research*, *Added when three wordings were corrected…*).
- **Code:** none.

### Dependencies 12 — differs

EEVEE's message for a missing file changes in 5.3 and names the file, against 5.2.2 on the Mac; a new missing-strip icon rests on 5.3's release notes.

- **Joined answers:** *What a render really did*, *What has to travel with a file*, *What the research cannot say*, *When a render ends*.
- **Plans (5):** Architecture/Dependency Upload & Remap.md (§6.6); Architecture/Rows/Dependency Upload & Remap.md (Row 174); Architecture/Rows/Rendering.md (Row 13); Architecture/Sources/Dependency Upload & Remap.sources.md (§7); Specs/Sources/Dependency Upload & Remap.sources.md (*From Blender Research*).
- **Code:** none.

### Dependencies 20 — an earlier reading reopened

A save that packs files while some places reach nothing refused in the earlier runs and wrote the file in 5.3 and 5.2.2, so that is not settled. 5.3's release notes add support for keeping a file unchanged on open and save; its reach is unknown.

- **Joined answers:** *Running Blender with nobody there*, *Two renders in one folder*, *What a file can be asked before it renders*, *What can be set from outside*, *What has to travel with a file*, *What the research cannot say*, *Where a render's files go*.
- **Plans (4):** Architecture/Dependency Upload & Remap.md (§3.2, §6.3); Architecture/Facts.md (Fact 128, *Where the register stands*); Architecture/Sources/Dependency Upload & Remap.sources.md (§2.5, §4.5); Architecture/What Build inherits.md (*Facts with no answer at all*).
- **Code (1):** App/Dependency Upload & Remap/src/check.rs (line 124).

### Dependencies 9 — not established for 5.3

The move to another computer was not made in 5.3; the round trip on the Mac came back the same.

- **Joined answers:** *What has to travel with a file*.
- **Plans (4):** Architecture/Dependency Upload & Remap.md (§3.1, §12); Architecture/Job files and outputs.md (§4.2); Architecture/Sources/Dependency Upload & Remap.sources.md (§3); Specs/Sources/Dependency Upload & Remap.sources.md (*From Blender Research*).
- **Code (1):** App/Rendering — machine/src/spellings.rs (lines 446, 453).

### Images 2 — differs

Three changes in 5.3, against 5.2.2 on the Mac: EEVEE's line names the missing file, an empty image slot prints nothing, and an unreadable file gets one line only.

- **Joined answers:** *What a render really did*, *What has to travel with a file*.
- **Plans (5):** Architecture/Rows/Queue & History.md (Row 85); Architecture/Sources/Queue & History.sources.md (§4); Specs/Sources/Queue & History.sources.md (*Still open*, *Answered by the Blender research*, *Added from the cross-sweep of the…*); Specs/Sources/Telegram.sources.md (line 51); Specs/Sources/What a job writes.sources.md (*Pictures, passes and formats*).
- **Code:** none.

### Builds 10 — not established for 5.3

Nothing about unpacking 5.3 was tested or read: its published files, checksums, archives, disk image or Python-module build.

- **Joined answers:** *Running Blender with nobody there*, *Two renders in one folder*, *What a file can be asked before it renders*, *What can be set from outside*, *What has to travel with a file*, *What the research cannot say*, *Where a render's files go*.
- **Plans (2):** Architecture/Server Setup.md (§5.2); Architecture/Sources/Server Setup.sources.md (*xz absent, lspci absent, and the…*).
- **Code (3):** App/Server Setup — machine/src/acting/steps.rs (line 74); App/Server Setup — machine/src/blender.rs (line 675); App/Server Setup — machine/src/ready_action.rs (line 119).

### Frames 9 — differs

5.3's default compositor tree, by release note a Group Input joined to a Group Output, put the first view layer's picture in the frame's file. Whether 5.3 offers AVIF was not recorded in this topic's runs (Images 3 records it in 5.3's list). A render with only the second view layer crashed in 5.3 and 5.2.2, unlike the earlier 5.2 runs.

- **Joined answers:** *What a file can be asked before it renders*, *Where a render's files go*.
- **Plans (5):** Architecture/Rows/Queue & History.md (Row 85); Architecture/Rows/What a job writes.md (Row 128); Architecture/Sources/Queue & History.sources.md (§3); Specs/Sources/Queue & History.sources.md (*Answered by the Blender research*); Specs/Sources/What a job writes.sources.md (*Frames, view layers and the re-run*).
- **Code:** none.

### Images 5 — differs

A File Output node fed from Group Input writes in 5.3 and not in 5.2.2 on the Mac; 5.3's multi-view reading is the Mac's alone and stands apart from the earlier Linux one.

- **Joined answers:** *What a file can be asked before it renders*, *Where a render's files go*.
- **Plans (5):** Architecture/Job files and outputs.md (§7.2); Architecture/One box with Blender on it — the reading, the listing, and the names that do not come back.md (line 551); Architecture/Rows/What a job writes.md (Row 132); Architecture/Sources/Job files and outputs.sources.md (§10); Specs/Sources/What a job writes.sources.md (*Pictures, passes and formats*, *Added 2026-09-01, after the research was…*).
- **Code:** none.

### Images 6 — differs

5.3 adds an sRGB ICC profile, and its PNG gains a colour-description block, by release note and a Mac check; the announced colour interop change is untested.

- **Joined answers:** *What a render really did*.
- **Plans (4):** Architecture/One box with Blender on it — the reading, the listing, and the names that do not come back.md (line 551); Architecture/Rows/What a job writes.md (Row 132); Specs/Sources/Download.sources.md (line 9); Specs/Sources/What a job writes.sources.md (*Pictures, passes and formats*).
- **Code (1):** App/Job files and outputs — machine/src/reading_script.rs (line 865).

### The compositor 4 — an earlier reading reopened

Repeated Scene-strip renders in one session kept an earlier composited picture in 5.3 and 5.2.2 on the Mac even as switches changed; the earlier 5.2 runs followed the switches. Fresh Blender runs follow them. This narrows the earlier reading without establishing a version difference.

- **Joined answers:** *What a file can be asked before it renders*, *What has to travel with a file*.
- **Plans (5):** Architecture/Rows/Design — Render Settings.md (Row 492); Architecture/Rows/What a job writes.md (Row 125); Architecture/Sources/Job files and outputs.sources.md (§3); Specs/Sources/Render Settings.sources.md (*Facts from Blender Research*); Specs/Sources/What a job writes.sources.md (*The compositor and the scenes around…*).
- **Code:** none.

### The machine 6 — differs

Cycles' record for each card holds more values in 5.3 than in 5.2.2 on the Mac; the launch option that lists drawing devices named none in 5.3 on the Mac.

- **Joined answers:** *Running Blender with nobody there*, *What can be set from outside*.
- **Plans (5):** Architecture/Facts.md (Fact 46); Architecture/Rows/Servers.md (Row 27); Architecture/Sources/Servers.sources.md (§6.15); Specs/Sources/Rendering.sources.md (line 12); Specs/Sources/Servers.sources.md (line 22).
- **Code:** none.

### What actually ran 5 — differs

5.3's list of log categories changes against 5.2.2, and 5.3 ships an OpenEXR reader for scripts that reads a written file's header.

- **Joined answers:** *Running Blender with nobody there*, *What a render really did*, *While a render runs*.
- **Plans (5):** Architecture/Rendering.md (§1.2); Architecture/Rows/Queue & History.md (Row 60); Architecture/Rows/Server Setup.md (Row 94); Architecture/Sources/Queue & History.sources.md (§3); Architecture/Sources/Rendering.sources.md (§B2).
- **Code:** none.

### Writing 12 — differs

Failure lines name different writing steps in 5.3 and 5.2.2. The render cache's failure reason, lost in the older 5.1 and 5.2 Mac runs, is named by both. The permissions case is untested in 5.3.

- **Joined answers:** *What the research cannot say*, *When a render ends*, *Where a render's files go*.
- **Plans (5):** Architecture/Rows/Queue & History.md (Row 67); Architecture/Rows/What a job writes.md (Row 135); Architecture/Sources/Queue & History.sources.md (§3); Architecture/Sources/Rented-server runtime.sources.md (line 475); Specs/Sources/What a job writes.sources.md (*Paths, names, and the redirect*).
- **Code:** none.

### Builds 5 — not established for 5.3

That two copies of 5.3 share one settings folder is inferred, not tested; 4.5 to 5.2 were tested.

- **Joined answers:** *Two renders in one folder*.
- **Plans (4):** Architecture/Rows/Server Setup.md (Rows 95, 96, 97, 101); Architecture/Rows/Telegram.md (Row 268); Architecture/Sources/Server Setup.sources.md (lines 574, 577); Specs/Sources/Server Setup.sources.md (*Facts about Blender*, *Where the research and the Creator…*, *Facts about rented machines, not about…*, *Added when the second and third…*).
- **Code:** none.

### Reports 7 — differs

A tiled frame's lines stop on the last tile's full count in 5.3, where 5.2.2 printed one more zero-sample line.

- **Joined answers:** *What a render really did*, *What the research cannot say*, *When a render ends*, *While a render runs*.
- **Plans (3):** Architecture/Rows/Queue & History.md (Rows 60, 62, 68); Architecture/Sources/Queue & History.sources.md (§3); Specs/Sources/Queue & History.sources.md (*Answered by the Blender research*).
- **Code (1):** App/Queue & History — machine/src/acting/reader.rs (lines 42, 381, 446, 448).

### Reports 8 — differs

With both logging switches given, 5.3 prints more lines than 5.2.2 on the same scene, and its category list changes.

- **Joined answers:** *What the research cannot say*, *While a render runs*.
- **Plans (2):** Architecture/Rows/Queue & History.md (Rows 60, 62); Architecture/Sources/Queue & History.sources.md (§3).
- **Code (2):** App/Queue & History — machine/src/acting/reader.rs (lines 72, 77, 399, 450, 467); App/Queue & History/src/bars.rs (line 240).

### Add-ons 12 — an earlier reading reopened

An install run by a script that fires at each frame, with a window open, finished in the earlier 5.2.0 run and failed in 5.2.2 and 5.3, so that case is not settled. An install fired from a second Blender was not tried in 5.3.

- **Joined answers:** *Two renders in one folder*, *What the research cannot say*, *While a render runs*.
- **Plans (3):** Architecture/Rows/Server Setup.md (Row 111); Architecture/Sources/Server Setup.sources.md (*Installing beside a running render, and…*); Specs/Sources/Server Setup.sources.md (*Facts about Blender*, *Added when the second and third…*).
- **Code (1):** App/Rendering — machine/src/spellings.rs (lines 153, 157, 290, 294).

### Reports 4 — differs

At information level 5.3 logs one more line than 5.2.2, and its list of log categories changes.

- **Joined answers:** *While a render runs*.
- **Plans (4):** Architecture/Rows/Queue & History.md (Row 60); Architecture/Sources/Queue & History.sources.md (§3); Specs/Sources/Queue & History.sources.md (*Answered by the Blender research*); Specs/Sources/Rendering.sources.md (line 14).
- **Code:** none.

### Builds 2 — not established for 5.3

None of the routes for listing and fetching builds was tried for 5.3.

- **Joined answers:** none.
- **Plans (4):** Architecture/Rows/Server Setup.md (Row 92); Architecture/Sources/Server Setup.sources.md (*The shop-front listing, read per channel…*); Specs/Sources/Server Setup.sources.md (*Facts about Blender*); Specs/Sources/Servers.sources.md (line 10).
- **Code:** none.

### Dependencies 13 — differs

By 5.3's release notes, which packed linked items count as direct, and so are written, changes; untested.

- **Joined answers:** *What has to travel with a file*.
- **Plans (4):** Architecture/Dependency Upload & Remap.md (§3.3, §3.4); Architecture/Rows/Dependency Upload & Remap.md (Row 162); Architecture/Sources/Dependency Upload & Remap.sources.md (§3); Specs/Sources/Dependency Upload & Remap.sources.md (*From the Creator*, *From Blender Research*).
- **Code:** none.

### Reports 10 — not established for 5.3

Nothing answers what 5.3 does when the machine's memory runs out while it samples.

- **Joined answers:** *What the research cannot say*, *When a render ends*.
- **Plans (4):** Architecture/Rows/Design — Queue & History.md (Row 393); Architecture/Rows/Queue & History.md (Rows 67, 68); Architecture/Sources/Queue & History.sources.md (§3); Specs/Sources/Queue & History.sources.md (*Answered by the Blender research*).
- **Code:** none.

### The machine 1 — differs

By 5.3's release notes, Vulkan becomes the default drawing system on Windows (not ARM) and Linux, and its minimum is lowered. 5.3's start-up on those systems, and a window request with no display, were not tried.

- **Joined answers:** *Running Blender with nobody there*, *What the research cannot say*.
- **Plans (4):** Architecture/Rendering.md (§1.1); Architecture/Rows/Server Setup.md (Rows 88, 94); Architecture/Sources/Rendering.sources.md (§B1); Specs/Sources/Server Setup.sources.md (*Facts about Blender*).
- **Code:** none.

### The machine 2 — differs

5.3 raises the stated NVIDIA driver minimum for CUDA and OptiX, with its manual and Cycles' file giving different figures; Cycles' Metal line names a higher macOS; by release note RTX Spark is supported on Windows 11.

- **Joined answers:** *Running Blender with nobody there*, *What a render really did*, *What the research cannot say*, *When a render ends*.
- **Plans (4):** Architecture/Rows/Server Setup.md (Rows 88, 89); Architecture/Sources/Rendering.sources.md (§P6); Architecture/Sources/Server Setup.sources.md (*A machine rendering on the processor…*); Specs/Sources/Server Setup.sources.md (*Facts about Blender*, line 7).
- **Code:** none.

### What a render demands 5 — not established for 5.3

Whether 5.3 changes a 4.5 file's saved tiling values on loading was not tried.

- **Joined answers:** *What a file can be asked before it renders*, *What a render really did*, *What can be set from outside*, *While a render runs*.
- **Plans (4):** Architecture/Render Settings.md (§1.4); Architecture/Rows/File Browser.md (Row 182); Architecture/Rows/Render Settings.md (Row 142); Architecture/Sources/Render Settings.sources.md (§B5, §B26).
- **Code:** none.

### Dependencies 15 — differs

5.3's bundled colour configuration has one more colour space and four more settings, against 5.2.2 on the Mac; by its release notes a project can choose the configuration.

- **Joined answers:** *What a render really did*, *What has to travel with a file*.
- **Plans (3):** Architecture/Queue & History.md (*What binds this before anything is…*, §8); Architecture/Sources/Queue & History.sources.md (§3); Specs/Sources/Dependency Upload & Remap.sources.md (*From Blender Research*).
- **Code (1):** App/Queue & History — machine/src/acting/reader.rs (lines 89, 871).

### Reading a file 2 — not established for 5.3

Reading or saving a file while another Blender renders was not tried in 5.3.

- **Joined answers:** *Two renders in one folder*, *What a file can be asked before it renders*, *What the research cannot say*.
- **Plans (4):** Architecture/Render Settings.md (§10, §13); Architecture/Rows/Render Settings.md (Row 140); Architecture/Rows/Telegram.md (Row 268); Architecture/Sources/Render Settings.sources.md (§B6, §U22).
- **Code:** none.

### Node setups 2 — differs

By 5.3's release notes, the rule for directly linked packed data changes; untested.

- **Joined answers:** *What has to travel with a file*.
- **Plans (4):** Architecture/Dependency Upload & Remap.md (§3.2); Architecture/Rows/Dependency Upload & Remap.md (Row 160); Architecture/Sources/Dependency Upload & Remap.sources.md (§3); Specs/Sources/Dependency Upload & Remap.sources.md (*From Blender Research*).
- **Code:** none.

### The compositor 1 — differs

In 5.3 a scene's compositor is a stack of effects, and the default tree is a Group Input joined to a Group Output; by release note the viewport compositor is on by default, untested. With no final output node 5.3 wrote a transparent black frame, not shown to be a change.

- **Joined answers:** *What a file can be asked before it renders*, *What the research cannot say*, *When a render ends*, *Where a render's files go*.
- **Plans (4):** Architecture/Rows/Design — Render Settings.md (Row 492); Architecture/Rows/What a job writes.md (Row 124); Architecture/Sources/Job files and outputs.sources.md (§3); Specs/Sources/What a job writes.sources.md (*The two routes, and whether either…*).
- **Code:** none.

### Builds 3 — differs

The 5.3 beta reports one more launch argument, two more names a script can read, two more build options and a different answer on whether it is portable, set against 5.2.2 or 5.2's own Mac record.

- **Joined answers:** *What a file can be asked before it renders*, *What the research cannot say*.
- **Plans (4):** Architecture/Rows/Render Settings.md (Row 155); Architecture/Rows/Server Setup.md (Row 96); Architecture/Sources/Server Setup.sources.md (*Asking a build what it is…*); Specs/Sources/Server Setup.sources.md (*Facts about Blender*).
- **Code:** none.

### Dependencies 14 — differs

By 5.3's release notes, scenes referenced inside compositor node groups are rendered; 5.3 was not traced.

- **Joined answers:** *What has to travel with a file*, *What the research cannot say*.
- **Plans (4):** Architecture/Rows/File Browser.md (Row 195); Architecture/Sources/File Browser.sources.md (§G2); Specs/Sources/Dependency Upload & Remap.sources.md (*From Blender Research*); Specs/Sources/File Browser.sources.md (line 26).
- **Code:** none.

### File output nodes 6 — differs

5.3's release notes report a change to a fresh node's starting file name; the earlier untouched defaults are not established here.

- **Joined answers:** *What a file can be asked before it renders*, *Where a render's files go*.
- **Plans (4):** Architecture/Job files and outputs.md (§5.2); Architecture/Rows/What a job writes.md (Row 128); Architecture/Sources/Job files and outputs.sources.md (§10); Specs/Sources/What a job writes.sources.md (*What a file output node can…*).
- **Code:** none.

### Settings 6 — not established for 5.3

Rendering with no screen was not tried in 5.3.

- **Joined answers:** *Running Blender with nobody there*, *What the research cannot say*, *When a render ends*, *While a render runs*.
- **Plans (4):** Architecture/Rendering.md (§1.1); Architecture/Rows/Server Setup.md (Row 94); Architecture/Sources/Rendering.sources.md (§B1); Specs/Sources/Render Settings.sources.md (*Facts from Blender Research*).
- **Code:** none.

### Writing 2 — differs

The built-in defaults of two File Output node settings are on in 5.3 and off in 5.2.2 on the Mac. A fresh 5.3 node starts with both on; earlier fresh nodes' starting values were not read.

- **Joined answers:** *What a file can be asked before it renders*, *Where a render's files go*.
- **Plans (4):** Architecture/Job files and outputs.md (§7.1); Architecture/Rows/What a job writes.md (Row 121); Architecture/Sources/Job files and outputs.sources.md (§5); Specs/Sources/What a job writes.sources.md (*The two routes, and whether either…*, *Paths, names, and the redirect*).
- **Code:** none.

### Frames 3 — not established for 5.3

Finished frames survived the tested 5.3 stops on the Mac, but a person's Escape was not tried; the earlier Mac runs had that test. Blender's simulated Escape did not stop the render.

- **Joined answers:** *Two renders in one folder*, *What the research cannot say*, *When a render ends*, *While a render runs*.
- **Plans (3):** Architecture/Rows/Queue & History.md (Row 60); Architecture/Sources/Queue & History.sources.md (§3); Specs/Sources/Queue & History.sources.md (*Answered by the Blender research*).
- **Code (1):** App/Queue & History — machine/src/acting/reader.rs (lines 69, 462, 467).

### Builds 8 — not established for 5.3

Removing a Blender while it renders was not tested for 5.3.

- **Joined answers:** *Two renders in one folder*, *When a render ends*.
- **Plans (3):** Architecture/Rows/Server Setup.md (Rows 95, 96, 97, 101); Architecture/Sources/Server Setup.sources.md (*Installing beside a running render, and…*, line 577); Specs/Sources/Server Setup.sources.md (*Facts about Blender*, *Where the research and the Creator…*).
- **Code:** none.

### What a render demands 7 — differs

By 5.3's release notes, Vulkan becomes the default on Windows (not ARM) and Linux, and RTX Spark is supported on Windows 11. 5.3's NVIDIA driver minimum differs between its manual and Cycles' file. The smaller method-to-method picture gaps in 5.3 are not set against the earlier versions.

- **Joined answers:** *Running Blender with nobody there*, *Two renders in one folder*, *What a file can be asked before it renders*, *What a render really did*, *What can be set from outside*, *What has to travel with a file*, *What the research cannot say*, *When a render ends*, *Where a render's files go*, *While a render runs*.
- **Plans (3):** Architecture/Facts.md (Fact 107); Architecture/Rendering.md (Row 12, §1.8, §9); Architecture/Sources/Rendering.sources.md (§B15, §M17, §P6, §C12).
- **Code:** none.

### Add-ons 11 — not established for 5.3

One Cycles hardware ray-tracing setting reads on in 5.3 on the Mac, but no earlier Mac value was recorded, so 5.3 is not set against the others.

- **Joined answers:** *What has to travel with a file*.
- **Plans (3):** Architecture/Rows/Server Setup.md (Rows 98, 100); Architecture/Sources/Server Setup.sources.md (lines 568, 574); Specs/Sources/Server Setup.sources.md (*Facts about Blender*, *The add-on questions — where they…*, line 7).
- **Code:** none.

### Add-ons 14 — not established for 5.3

For 5.3, crossing between versions rests on its documents, not a test; a copied 5.3 never started, so a second copy and a portable folder are not settled.

- **Joined answers:** *What has to travel with a file*.
- **Plans (3):** Architecture/Rows/Server Setup.md (Rows 98, 100, 101); Architecture/Sources/Server Setup.sources.md (lines 570, 576); Specs/Sources/Server Setup.sources.md (*Added when the second and third…*).
- **Code:** none.

### Add-ons 4 — not established for 5.3

Carrying a set-up add-on to another kind of computer was not tried in 5.3, and one Cycles hardware ray-tracing setting's starting value is not placed for 5.3.

- **Joined answers:** *What a render really did*, *What has to travel with a file*.
- **Plans (3):** Architecture/Rows/Server Setup.md (Row 98); Architecture/Sources/Server Setup.sources.md (lines 567, 568, 574); Specs/Sources/Server Setup.sources.md (*Facts about Blender*, *The add-on questions — where they…*).
- **Code:** none.

### Add-ons 8 — differs

5.3 adds two repository settings and a line to generated listings, through Add-ons 5's same-Mac control. Copying an installed add-on to another computer was not tried in 5.3.

- **Joined answers:** *What has to travel with a file*.
- **Plans (3):** Architecture/Rows/Server Setup.md (Rows 97, 98); Architecture/Sources/Server Setup.sources.md (lines 565, 567); Specs/Sources/Server Setup.sources.md (*Facts about Blender*, *The add-on questions — where they…*).
- **Code:** none.

### File output nodes 15 — differs

5.3's failure lines for an OpenEXR write that cannot finish name a different part of Blender than 5.2.2's on the Mac. Because 5.2.2 named the reason, whether 5.1 and 5.2 do is not fixed.

- **Joined answers:** *Running Blender with nobody there*, *Two renders in one folder*, *What the research cannot say*, *When a render ends*, *Where a render's files go*, *While a render runs*.
- **Plans (3):** Architecture/Rows/What a job writes.md (Rows 135, 136); Specs/Sources/Download.sources.md (*Added when the second and third…*); Specs/Sources/What a job writes.sources.md (*What a file output node can…*, *Paths, names, and the redirect*).
- **Code:** none.

### Add-ons 13 — not established for 5.3

Whether 5.3 has two launch options for system folders, and what one logging level adds in 5.3, were not established; a difference in how a legacy add-on's details read back is not shown to be a version change.

- **Joined answers:** *What a render really did*.
- **Plans (3):** Architecture/Rows/Server Setup.md (Rows 99, 100); Architecture/Sources/Server Setup.sources.md (lines 569, 571); Specs/Sources/Server Setup.sources.md (*Facts about Blender*, *Added when the second and third…*).
- **Code:** none.

### Writing 14 — differs

The comparison also lists here the OpenEXR file one byte longer in 5.3, under Writing 11; the order of writes is the same in all five.

- **Joined answers:** *Running Blender with nobody there*, *Two renders in one folder*, *What a render really did*, *What can be set from outside*, *What has to travel with a file*, *What the research cannot say*, *When a render ends*, *Where a render's files go*, *While a render runs*.
- **Plans (2):** Architecture/Download.md (§4.1, §15); Architecture/Sources/Download.sources.md (*The no-overlap inference — REWRITTEN 2026-09-01:…*).
- **Code (1):** App/Download/src/vouching.rs (line 67).

### Add-ons 3 — differs

By 5.3's release notes, a background run from the command line no longer shows Blender's icon in the macOS Dock; no test looked. For 5.3, installing on a machine with no display server rests on its documents, not a test.

- **Joined answers:** *Running Blender with nobody there*, *What can be set from outside*, *What the research cannot say*.
- **Plans (3):** Architecture/Rows/Server Setup.md (Rows 94, 97); Architecture/Sources/Server Setup.sources.md (line 566); Specs/Sources/Server Setup.sources.md (*Facts about Blender*, *The add-on questions — where they…*).
- **Code:** none.

### Add-ons 5 — differs

5.3's extension command offers one more action, each repository carries two more settings, and a generated repository listing carries one more line; each was checked against 5.2.2 on the Mac.

- **Joined answers:** *Running Blender with nobody there*, *What can be set from outside*.
- **Plans (3):** Architecture/Rows/Server Setup.md (Row 97); Architecture/Sources/Server Setup.sources.md (line 565); Specs/Sources/Server Setup.sources.md (*Facts about Blender*, *Where the research and the Creator…*, *The add-on questions — where they…*).
- **Code:** none.

### File output nodes 14 — differs

5.3's effect render switch can be keyed or driven for its own frame; changed by a per-frame script, it takes effect one frame late. The scene's Compositing switch also reaches the node one frame late in 5.3; 5.2.2 on the same Mac wrote every frame.

- **Joined answers:** *What a file can be asked before it renders*.
- **Plans (3):** Architecture/Rows/What a job writes.md (Rows 124, 128); Architecture/Sources/Job files and outputs.sources.md (§3); Specs/Sources/What a job writes.sources.md (*The two routes, and whether either…*, *What a file output node can…*).
- **Code:** none.

### Frames 1 — not established for 5.3

A kernel reset was not tried in 5.3; a real power cut was tried in no version.

- **Joined answers:** *When a render ends*.
- **Plans (3):** Architecture/Rows/Queue & History.md (Rows 71, 73); Architecture/Sources/Queue & History.sources.md (§3); Specs/Sources/Queue & History.sources.md (*Answered by the Blender research*).
- **Code:** none.

### Builds 1 — differs

The oldest macOS the application names is higher in the 5.3 beta, on the Mac. An older Mac, and 5.3's published files, were not checked.

- **Joined answers:** none.
- **Plans (3):** Architecture/Rows/Server Setup.md (Row 92); Specs/Sources/Server Setup.sources.md (*Facts about Blender*); Specs/Sources/Servers.sources.md (line 8).
- **Code:** none.

### Frames 14 — not established for 5.3

Two renders started at once were not run in 5.3.

- **Joined answers:** *Two renders in one folder*, *What the research cannot say*, *When a render ends*.
- **Plans (2):** Architecture/Rows/Queue & History.md (Row 72); Architecture/Sources/Queue & History.sources.md (§3).
- **Code (1):** App/Rendering — machine/src/spellings.rs (line 435).

### The compositor 3 — differs

In 5.3 a scene points at its node groups through its own stack of effects.

- **Joined answers:** *What a file can be asked before it renders*, *What can be set from outside*.
- **Plans (3):** Architecture/Rows/Design — Render Settings.md (Row 492); Architecture/Rows/What a job writes.md (Row 125); Specs/Sources/What a job writes.sources.md (*The compositor and the scenes around…*).
- **Code:** none.

### The compositor 7 — not established for 5.3

Whether 5.3's multi-layer OpenEXR carries the extra pixel-aspect channel was not read.

- **Joined answers:** *Running Blender with nobody there*, *Where a render's files go*.
- **Plans (3):** Architecture/Job files and outputs.md (§8.1, §14); Architecture/Rows/Design — Render Settings.md (Row 492); Architecture/Sources/Job files and outputs.sources.md (§3, §10).
- **Code:** none.

### Dependencies 18 — an earlier reading reopened

Opening a file whose fluid cache folder is gone remade the folder in 4.5 to 5.2 and did not in 5.3 or 5.2.2, so that is not settled. The two 5.2 readings of a missing ocean bake disagree. A 4.5 cache was not tried in 5.3.

- **Joined answers:** *What a render really did*, *What has to travel with a file*.
- **Plans (3):** Architecture/Rows/Dependency Upload & Remap.md (Row 312); Specs/Sources/Dependency Upload & Remap.sources.md (*From Blender Research*, *Added when three wordings were corrected…*); Specs/Sources/Rendering.sources.md (*Added 2026-10-02 — a simulation never…*).
- **Code:** none.

### Dependencies 6 — not established for 5.3

No 5.3 file was carried to another computer.

- **Joined answers:** *What has to travel with a file*, *What the research cannot say*.
- **Plans (3):** Architecture/Rows/What a job writes.md (Row 121); Specs/Sources/Dependency Upload & Remap.sources.md (*From Blender Research*); Specs/Sources/What a job writes.sources.md (*Paths, names, and the redirect*).
- **Code:** none.

### Settings 8 — differs

By 5.3's release notes, the compositor renders scenes referenced inside node groups; 5.3's Scene-strip view-layer setting is not established.

- **Joined answers:** *What a file can be asked before it renders*.
- **Plans (3):** Architecture/Rows/Design — Render Settings.md (Row 492); Architecture/Rows/Render Settings.md (Row 140); Specs/Sources/Render Settings.sources.md (*Facts from Blender Research*).
- **Code:** none.

### The compositor 5 — differs

5.3's default tree brings the render in through Group Input. Enabled-pass sockets, precision and the fuller socket display in 5.3 are not established.

- **Joined answers:** *Where a render's files go*.
- **Plans (3):** Architecture/Rows/Design — Render Settings.md (Row 492); Architecture/Rows/What a job writes.md (Row 130); Specs/Sources/What a job writes.sources.md (*The compositor and the scenes around…*).
- **Code:** none.

### The compositor 8 — differs

A render with a window no longer fills the Viewer node's image in 5.3, by release note and the Mac test. A compositor tree accepts more node types in 5.3 than in 5.2.2. Whether 5.3 keeps 5.2's rounding result is open.

- **Joined answers:** *Running Blender with nobody there*.
- **Plans (3):** Architecture/Rows/What a job writes.md (Row 130); Architecture/Sources/Job files and outputs.sources.md (§6); Specs/Sources/What a job writes.sources.md (*The compositor and the scenes around…*).
- **Code:** none.

### What actually ran 4 — differs

The 5.3 build reports two more build options than 5.2.2. Switching off saved output was not tried in this topic's 5.3 runs (The compositor 2 and Reports 1 record it in 5.3).

- **Joined answers:** *What a render really did*, *What can be set from outside*, *When a render ends*.
- **Plans (3):** Architecture/Rows/Render Settings.md (Row 157); Architecture/Rows/Rendering.md (Row 9); Architecture/Sources/Render Settings.sources.md (§B24).
- **Code:** none.

### Images 8 — differs

The smaller copy written by a File Output node fed from the group's input appears in 5.3 and not in 5.2.2.

- **Joined answers:** *Running Blender with nobody there*, *Where a render's files go*.
- **Plans (2):** Architecture/One box with Blender on it — the reading, the listing, and the names that do not come back.md (§6); Specs/Sources/Telegram.sources.md (line 47).
- **Code (1):** App/Job files and outputs — machine/src/derivatives.rs (lines 128, 137, 255, 1011).

### Writing 5 — differs

Two failure lines name different writing steps in 5.3 and 5.2.2. The full-disk reason the older 5.1 and 5.2 Mac runs lost is named by 5.2.2 too, so that is not a 5.3 change. Refusals by a folder's own permissions were not tested in 5.3.

- **Joined answers:** *What the research cannot say*, *When a render ends*.
- **Plans (2):** Specs/Sources/Download.sources.md (*Added when the second and third…*); Specs/Sources/What a job writes.sources.md (*What a file output node can…*, *Paths, names, and the redirect*).
- **Code:** none.

### Settings 19 — differs

5.3 adds view-layer denoising settings for Cycles and EEVEE, a viewport upscale quality setting, and by release note a DLSS viewport denoiser. OptiX in 5.3 is untested.

- **Joined answers:** *Running Blender with nobody there*, *Two renders in one folder*, *What a file can be asked before it renders*, *What a render really did*, *What can be set from outside*, *What has to travel with a file*, *What the research cannot say*, *When a render ends*, *Where a render's files go*.
- **Plans (2):** Architecture/Job files and outputs.md (§8.3); Architecture/Sources/Job files and outputs.sources.md (§10).
- **Code:** none.

### Settings 2 — differs

EEVEE gains a time limit in 5.3, documented and checked against 5.2.2 on the Mac.

- **Joined answers:** *What a render really did*.
- **Plans (2):** Architecture/Facts.md (Fact 141); Specs/Sources/Render Settings.sources.md (*Facts from Blender Research*).
- **Code:** none.

### File output nodes 11 — not established for 5.3

5.3's format menu was not read in the window; the no-video answer held on the Mac.

- **Joined answers:** *What a file can be asked before it renders*, *Where a render's files go*.
- **Plans (2):** Architecture/Rows/What a job writes.md (Row 129); Specs/Sources/What a job writes.sources.md (*What a file output node can…*).
- **Code:** none.

### File output nodes 12 — differs

By 5.3's release notes, the default compositor tree takes the render in through Group Input rather than a Render Layers node; feeding a File Output node from it was not tried under this question.

- **Joined answers:** *What a file can be asked before it renders*, *Where a render's files go*.
- **Plans (2):** Architecture/Rows/What a job writes.md (Row 128); Specs/Sources/What a job writes.sources.md (*What a file output node can…*).
- **Code:** none.

### What a render demands 1 — differs

5.3 adds four setting names, against 5.2.2 on the Mac.

- **Joined answers:** *What a render really did*, *What the research cannot say*, *While a render runs*.
- **Plans (2):** Specs/Sources/Render Settings.sources.md (*Facts owned elsewhere*); Specs/Sources/Rendering.sources.md (*Added when Simplify was ruled onto…*).
- **Code:** none.

### Add-ons 2 — not established for 5.3

This topic did not offer 5.3 EEVEE's old name; Images 9 records its refusal from a script. Whether 5.3 accepts it on the command line is untried.

- **Joined answers:** *Running Blender with nobody there*.
- **Plans (1):** Specs/Sources/Server Setup.sources.md (*Facts about Blender*, *The add-on questions — where they…*).
- **Code:** none.

### Dependencies 2 — differs

5.3's release notes add asset libraries from a project, from an extension and from a server that asks for a login; none is a library a file points at.

- **Joined answers:** *What has to travel with a file*.
- **Plans (1):** Specs/Sources/Dependency Upload & Remap.sources.md (*From the Creator*, *From Blender Research*).
- **Code:** none.

### File output nodes 16 — an earlier reading reopened

The older windowed sequencer-command cancellations do not establish a version difference: 5.2.2 and 5.3 ran it on the same Mac once the sequencer scene was set. Saving off through a Scene strip is untried; The compositor 2 records nodes writing with saving off without a strip in 5.2 and 5.3.

- **Joined answers:** *Running Blender with nobody there*, *Two renders in one folder*, *What a file can be asked before it renders*, *What can be set from outside*, *What has to travel with a file*, *What the research cannot say*, *Where a render's files go*, *While a render runs*.
- **Plans (1):** Architecture/What Build inherits.md (*Blender: the ceiling, the two vocabularies,…*).
- **Code:** none.

### Reports 13 — differs

EEVEE's missing-texture line changes and names the file in 5.3, against 5.2.2. How quiet mode treats missing-texture progress is not settled across versions.

- **Joined answers:** *Running Blender with nobody there*, *Two renders in one folder*, *What a render really did*, *What can be set from outside*, *What has to travel with a file*, *What the research cannot say*, *When a render ends*, *Where a render's files go*, *While a render runs*.
- **Plans (1):** Architecture/What Build inherits.md (*Blender: the ceiling, the two vocabularies,…*).
- **Code:** none.

### File output nodes 5 — differs

5.3's release notes add project path variables, and a 5.3 test agrees; a project-backed root was not tested.

- **Joined answers:** *What the research cannot say*, *Where a render's files go*.
- **Plans (1):** Specs/Sources/What a job writes.sources.md (*What a file output node can…*).
- **Code:** none.

### Settings 7 — differs

By 5.3's release notes, EEVEE renders panoramic cameras; untested, and its limitations page disagrees.

- **Joined answers:** *What can be set from outside*.
- **Plans (1):** Specs/Sources/Render Settings.sources.md (*Facts from Blender Research*).
- **Code:** none.

### File output nodes 9 — not established for 5.3

5.3's format menu as drawn in the window was not read.

- **Joined answers:** *What a file can be asked before it renders*, *Where a render's files go*.
- **Plans:** none.
- **Code (1):** App/Desktop app and delivery/src/surface/render_settings.rs (line 6974).

## On the list, and named by no plan and no code

These change nothing in the plans or the code as they stood on 2026-10-18. The joined answers that name them are given.

- **Frames 17** — not established for 5.3. 5.3's leftovers were read only by Blender's own reader. Joined answers: *Running Blender with nobody there*, *Two renders in one folder*, *What a file can be asked before it renders*, *What can be set from outside*, *What has to travel with a file*, *When a render ends*, *Where a render's files go*.
- **Images 10** — differs. A File Output node fed from Group Input writes in 5.3 and not in the 5.2.2 Mac control; earlier 5.0 to 5.2 results remain partly unsettled. 5.3 adds the ICC profile and colour-description block. The Sample Count pass is written here and absent in Reports 11's different setup; the cause is unsettled. Joined answers: *What a render really did*, *What has to travel with a file*.
- **Settings 3** — differs. EEVEE's own settings gain a time limit in 5.3, against 5.2.2 on the Mac. Joined answers: *What a file can be asked before it renders*, *What the research cannot say*.
- **What actually ran 3** — an earlier reading reopened. The earlier runs did not establish that the requested add-on was absent, so the answer for a genuinely absent add-on in 4.5 to 5.2.0 remains open. 5.3 and 5.2.2 name an absent add-on and carry on. Joined answers: *What a render really did*.
- **Writing 3** — differs. By 5.3's release notes, project variables are added; their use inside a project is untested. Joined answers: *What a file can be asked before it renders*, *Where a render's files go*.
