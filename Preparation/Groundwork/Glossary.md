# Glossary

The agreed names for this project, in one place. Only terms that come up often and could otherwise cause confusion are listed here — not every word we ever use. Grouped by kind. New terms get added as they earn their place.

*2026-10-27 — Removed the withdrawn paid-access text and its obsolete references under the Creator’s free-and-open-source decision of 2026-10-04 (wave 5); retained protections and evidence keep their force. No auditor has seen this removal.*

## People

### Creator
The human who directs this project and has the final say — over what gets built, what gets audited, the pace of the work, and (per Rule 2) how Blender behaves. A Blender professional, not a coder.

## Stages

### Stage
One of the six big blocks of work the project moves through, in order: Groundwork → Specs → Blender Research → Design → Architecture → Build. A stage is a period of work over time, not a place. What a stage *produces* is what gets stored in a folder — the stage itself isn't stored anywhere. (The order the stages run in, and *why*, is laid out in Start Here — kept there alone so the two can't drift apart.)

### Groundwork
Stage 1. Where we settle the project's foundations: names, rules, folder structure, and conventions.

### Specs
Stage 2. A plain-English description of *what* the app does — with no mention of how it's built. No stack, no code, no technical how. This is also where the app's **feature** modules are first identified, named, and scoped — the capabilities a user actually gets. Shared, technical modules are *not* named here; they emerge during Architecture.

### Blender Research
Stage 3. Gathering the true facts about Blender that the app needs. Blender's official documentation is the preferred source; a fact may also come from the Creator or from a controlled test, but every fact carries a label saying which (see Rule 2). Never from an agent's training data, general knowledge, or the open web.

### Design
Stage 4. What the app looks like and how it feels to use — the screens, the layout, the visual language, the finer interactions.

### Architecture
Stage 5. The plan for *how* the app actually does what the Specs describe.

### Build
Stage 6, the final stage. Making the actual app. Begins with a plan — what to construct and in what order — then the construction itself. Build is the only stage that produces code, not just documents.

## Folders

### Preparation
The root folder that holds every planning document — all Groundwork, Specs, Blender Research, Design, Architecture, and Build-plan files, plus the scenarios. Contains no code.

### App
The root folder that holds the actual application code, produced during the Build stage.

### Audits
The root folder that holds every audit trail — the audits, verdicts, and decisions records produced when a Subject is checked. Kept separate because it grows for the life of the project and never concludes.

Where things go, in one line: planning documents → Preparation, code → App, audit trails → Audits.

## Structure

### Module
A self-contained part of the app, with clear edges — something you can add, change, or remove on its own. Two kinds: *feature modules* (a capability, like File Browser or Rendering) and *shared modules* (common plumbing many features lean on, like talking to the server). Feature modules are named in Specs; shared modules emerge in Architecture, because they're a technical answer, not a product promise. Each module's plan lives in Preparation, its code in App, its audit trail in Audits.

## Modules

Smoozy's **feature modules** — the capabilities the user gets. Their canonical names are recorded here. How the modules connect, and the scope of any module whose own spec isn't written yet, lives in the [module map](../Specs/Whole%20app.md); once a module has its own spec, that spec owns its full scope.

### Keys
SSH key management.

### Servers
A server's connection identity.

### File Browser
Browsing a server's files and sending files up to it.

### Server Setup
Getting a bare machine ready to render — installing Blender, supplying the add-ons a job needs, and whatever else the machine needs.

### Render Settings
The render settings for a chosen `.blend`, and the routine that reads the file.

### Dependency Upload & Remap
Supplying and re-linking a `.blend`'s missing assets.

### Rendering
Running a render job on a server.

### Queue & History
The shared job queue and the record of finished jobs.

### Download
Bringing files back from a server to the user's machine.

### Telegram
The Telegram bot for following and driving renders from a phone.

## Machines and connections

Two words from outside Smoozy that the specs lean on constantly. Both are ordinary industry terms, defined here so no document has to gloss them again.

### GPU
The graphics card in a machine — what Blender renders on. A machine may have one or several, and Smoozy gives all of them to whatever job is rendering. Where anything says *graphics card*, it means the same thing; **GPU** is the name used throughout.

### SSH
The standard way one machine logs into another over a network and runs things on it. It is how Smoozy reaches every server: connecting, browsing files, installing software, rendering and moving files all happen over it. Smoozy connects with a **key** rather than a password (see [Keys](../Specs/Keys.md)), and the details it needs to reach a machine are what a server's connection identity holds (see [Servers](../Specs/Servers.md)).

## Blender

The Blender concepts Smoozy has to reason about by name. Each is a fact about how Blender works, stated by the Creator — a Blender professional — under Rule 2, and the citations sit in [Glossary.sources.md](Sources/Glossary.sources.md). They do not all stand the same way, and each entry says which it is.

### Scene
A self-contained part of a `.blend` file — in effect a mini-`.blend` inside it, with its own frame range, its own view layers, and its own render settings. One `.blend` can hold several. Scenes may share dependencies, but otherwise each stands on its own. Label: **Creator-stated** (2026-07-24).

### Dependency
Something a `.blend` needs to render that is not inside it — a file it points at rather than holds. Which kinds there are, and how they are supplied and re-linked, belong to the [Dependency Upload & Remap spec](../Specs/Dependency%20Upload%20&%20Remap.md). Label: **Creator-stated** (2026-07-28).

### Frame
A single point in time within a scene. The frames a job is asked to consider are its **frame range** — but being in the range is only an invitation, not a promise that anything is produced there (see Renderlet). Frames are **scene-local**: frame 1 of one scene and frame 1 of another are different points that produce different Outputs, so a frame is only pinned down together with its scene. Label: **Creator-stated** (2026-07-24).

### View layer
A slice of a scene that can be rendered on its own. A single frame can have zero, one, or many view layers active on it. Whether a view layer renders is **animatable**: the same layer can render on one frame, be skipped on the next, and render again on a third. Label: **Creator-stated** (2026-07-24).

### Renderlet
The atom of render work: **one view layer, on one frame, within one scene** — the coordinates *(scene, frame, view layer)*. It is the smallest thing that actually gets rendered, and it is not the same as a frame, a view layer, an output, or a scene. Because a frame's active view layers are decided per frame, a single frame yields **zero** renderlets (nothing active, so nothing rendered and no Output), one, or many.

"Renderlet" is Smoozy's own name for this unit — Blender has no single word for it. The name is a Smoozy decision; the unit it names is a Blender fact. Plain "render" was set aside as too generic to survive a long conversation without being misread. Label: **Creator-stated** (2026-07-24).

### Output
What finished work lands on disk as. An output belongs to a **frame**, not to a renderlet: a frame's outputs are written to disk only once **every** view layer active on that frame has finished, so many renderlets can feed one output, and a frame with no active view layers produces none. Label: **Creator-stated** (2026-07-24).

### Regular output path
The one place, set on the scene, where a render writes its frames. It has nothing to do with the compositor. Named this way because a `.blend` has a second, quite separate way of writing files — see File output node. Label: **Creator-stated** (2026-07-28), and to be confirmed at Blender Research.

### Compositor
A set of nodes inside a `.blend` that takes the render and does something with it before anything is saved. Label: **Creator-stated** (2026-07-28), and to be confirmed at Blender Research.

### File output node
A node in the compositor that writes files of its own, to wherever it is told, in any number. Ordinary work in real files, not a power user's trick. What it can do, and what Smoozy does about it, is [What a job writes](../Specs/What%20a%20job%20writes.md)'s. Label: **Creator-stated** (2026-07-28), and to be confirmed at Blender Research with more rigour than anything else in this project, because its behaviour changes from version to version.

### Beauty pass, utility pass, diffuse colour pass
The kinds of picture a render produces. The **beauty pass** is the finished picture an artist looks at; Blender's own name for it is **Combined**, the same in every version, which is how it is told from the rest. A **utility pass** is a separate slice of the render kept for later work — a shadow, reflection, mist, depth or ambient-occlusion pass, among others — and most say nothing on their own about whether the render came out right. The **diffuse colour pass** is one of them, named separately only because two specs reach for it by name.

It was named separately for a reason that no longer holds: that it is the one utility pass which still shows whether a scene's textures loaded. Label: **Creator-stated** (2026-07-28), and **not confirmed** — the research established nothing of the kind for any pass, having only ever looked at the finished picture. What it did establish is that a missing or unreadable texture comes out as flat magenta in the picture itself, so the beauty pass answers that question without a second pass being needed. The claim about this pass is left unresolved rather than overturned, since nobody tested it. The facts are owned by the [Queue & History sources](../Specs/Sources/Queue%20&%20History.sources.md) and cited there at **Images 1** and **Images 2**; the first two terms are **documented and observed**.

## Jobs

Names Smoozy coined for things that belong to a job rather than to Blender. Each is a naming decision, not a sourced fact.

### Main output
The one place a job's most important work is written. What it is and how it is found live in [What a job writes](../Specs/What%20a%20job%20writes.md). Label: **Creator-stated** (2026-07-28).

### Working format, viewing format
The two kinds of image file a render is written as. A **working format** holds more light than a screen can show and is meant to be carried on working with rather than looked at. A **viewing format** opens as a picture anywhere. The split decides two things that pull opposite ways — see [What a job writes](../Specs/What%20a%20job%20writes.md), which also carries the open question of which formats fall on which side. Label: **Creator-stated** (2026-07-28).

## Reporting a problem

### Diary
Smoozy's own running record of what it did, kept on the user's own computer. It is Smoozy's, not the user's: they never switch it on and never meet it as a feature. What it holds, how long it is kept and what a report takes out of it live in [Reporting a problem](../Specs/Reporting%20a%20problem.md). Label: **Creator-stated** (2026-08-24).

### Report
The file Smoozy writes when the user asks to report a problem — a **slice** of the diary and never the whole of it, plus one sentence in the user's own words. The user attaches it to their own mail; Smoozy sends nothing. See [Reporting a problem](../Specs/Reporting%20a%20problem.md). Label: **Creator-stated** (2026-08-24).

## Auditing

### Subject
The thing being audited — a document, a chunk of code, a seam, a scenario, or a set of findings. One word for all of them, so the audit process can talk about any.

### Set of findings
One Blender Research topic, with its questions answered for every Blender version, read together. A kind of Subject, audited like any other. Not one version on its own: what a round catches is a habit of the researcher's, and habits repeat across versions.

### Seam
One connection or one thread that several documents have to agree about — the fit between them, rather than any one of them. A kind of Subject, audited like any other. Two shapes: a *pair seam* (two modules that connect, plus what the module map says about the pair) and a *thread seam* (one behaviour running through many documents, owned by none of them).

### Scenario
One person doing one piece of real work with the app, written out step by step and played through the documents from start to finish. A kind of Subject, audited like any other. Written before the round, never invented by the auditor, and fixed once written.

### Altitude
What counts as an answer in a scenario round: the stage the round is played at, and what a document is not expected to have settled yet. Set by the round rather than by the scenario, since the same scenario is played at more than one stage.

### Runner
Unqualified, **runner** always means the agent that runs one audit round. A stage can have a runner of its own, and it is always named for its stage — the **researcher-runner** carries one Blender Research topic from an empty question sheet to a checked comparison, and the **architecture runner** carries the Architecture stage. Neither audits anything, and each is set out in its own stage's method. A stage runner may also run that stage's audit rounds — the architecture runner does — since neither role ever audits, which is all the separation the audit runner needs.

The agent that runs an audit round from start to finish. It does no auditing itself. It walks the Creator through the round step by step and does all the filing: it works out the round number, creates the folder, records the starting commit and audit boundary, fills every role from the standing roster, hands over or launches each brief, checks each output landed, and closes the round at the end by checking the implementer's work against the Verdict. The Creator makes only the calls that are theirs. Architecture is the exception: its method opens one discovery round by readiness and closes it by bounded verification because the Creator does not read that stage.

### Verdict
The outcome of one audit round: the consolidated list of what to fix, plus what was rejected and why. The Creator approves it in an ordinary round; Architecture uses its stage waiver unless a finding has to return to Specs. One per round.

### Decisions record
The running, append-only memory of everything that has happened to one Subject: what each audit round fixed and rejected, with reasons, and every change made to the Subject *between* rounds — which no auditor has yet seen (Rule 7). Read by the consolidator, never by the auditors, so settled points aren't re-litigated and unexamined additions don't pass unnoticed. One per Subject.

### Audit folder
The folder holding everything for one audited Subject: the individual audits, each round's verdict, and the decisions record. Versions aren't kept here — git holds the history.

---
