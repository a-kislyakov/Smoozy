# Conventions

How we keep this project tidy and consistent. Breaking one of these makes a **mess**, not a disaster — the things that cause real damage live in the Rules. Written lean: a short statement each, an example where it helps.

---

**Planning documents live in Preparation, code lives in App, audit trails live in Audits.**

The three top folders never mix. Every planning document goes under `Preparation/`; the actual app code goes under `App/`; every audit trail goes under `Audits/`. Which one something belongs in depends on *what it is* — a plan, code, or a checking record — not on which stage or module made it.

One folder at the top is none of the three, and is not ours: **`Blender Sources/`**, holding Blender's own documentation as downloaded. It sits apart because a Blender researcher is allowed to read it and nothing else in the project, and keeping it away from `Preparation/` is what stops "read only your sources" from putting that researcher next to the planning documents it must never see. Its contents are kept out of git and can be downloaded again; only its folder note is tracked. Nothing of ours is ever written there.

Inside each folder, the organization differs:

- **Preparation** is organized by **stage**, then by **module** within it — e.g. `Preparation/Specs/File Browser.md`, `Preparation/Design/File Browser.md`.
  - Not everything is module-shaped. A stage folder may also hold a **`Whole app.md`** for material that spans the whole product (the complete user journey, the shared visual language, security, how failures behave), and **topic-shaped documents** where that's the natural cut — Blender Research falls into Blender topics, Design into screens and flows. A topic document says which modules it affects.
  - Some stages are global and hold no module documents at all. Groundwork is one.
  - One folder inside Preparation is not a stage: **`Preparation/Scenarios/`**, which holds the scenarios (see [Auditing](Auditing.md)). A scenario belongs to no stage — the same one is played against the Specs, then the Design, then the Architecture, then the built app — so filing it under any single stage would be wrong. A scenario is **fixed once written**: it is not revised as the documents it is played against change, because every round that has already played it judged that exact story. It sits in Preparation rather than Audits because it is an input to the checking, not a record of it, and it carries no judgment of any document.
- **App** is organized by **module** — e.g. `App/File Browser/`.
- **Audits** is organized by **Subject** — one folder per audited thing (see below).

---

**Keys and credentials live in `~/.smoozy-keys/`, outside the project, one file per service.**

Nothing secret ever enters the project tree — not a document, not a findings file, not a commit, not a chat message an agent writes. So the keys this project's work needs sit in a folder of the Creator's own, next to their home folder and nowhere near the repository: **`~/.smoozy-keys/`**, written `~/.smoozy-keys/` everywhere else. **This convention is the folder's one home in the project**; the folder's own `README.md` says what is currently in it and what an agent must not touch in each account, and that README is the thing to open, not this paragraph.

The shape is **one plain-text file per service, named for the service in lower case with no spaces** — `vast.txt`, `cloudflare.txt`, `lemonsqueezy.txt`. A space or a capital in the name breaks a shell command that a lower-case name would have survived, which is the only reason the form is fixed. Each file's first line says what the key is for and what is already the Creator's in that account. Nothing else belongs in the folder: it holds keys, not working files or fetched records.

**An agent reads only the one file its work needs.** Whoever hands out the work names that file; an agent does not browse the folder for keys it was not given. **An agent that finds its key missing says so and stops that piece of work** — it never carries on by guessing, and it never asks the Creator to paste the value into a chat when the folder is the route. **A key that has been through a chat stays in that chat's history for good and is worth rotating** once the work is done, so reading one into a conversation that does not need it is a cost with no benefit.

An agent that would echo a key in a machine's output, a log or a transcript **redacts it and says that it did**.

What agents may spend, and on whose behalf, is a different question and is not this convention's: the [Architecture method](../Architecture/Method.md) holds the Creator's ruling of 2026-08-17 and the five bounds on it.

---

**Audit trails are filed under `Audits/<Stage>/<Subject>/`.**

The stage is a folder, and the Subject sits inside it — so nothing collides: File Browser has a spec, a design, an architecture, and code, and each of those is a separate Subject with its own folder under its own stage.

```
Audits/Specs/File Browser/
├── Decisions record.md       ← running; spans every round
├── Round 1/
│   ├── Round info.md         ← starting commit, audit boundary, and why this round opened
│   ├── Claude Opus 4.8.md    ← one audit per model, named after the model
│   ├── GPT-5.md
│   └── Verdict.md
└── Round 2/
    └── …
```

A Subject's folder can hold more than rounds. Blender Research is the case, and it holds **no rounds at all** — its Subject is one **topic**, and it is checked by a series of single chats rather than by the four-role loop (see [Auditing](Auditing.md)). Everything is named for the model that did it, so a second or third pass sits beside the first instead of over it:

```
Audits/Blender Research/File output nodes/
├── Decisions record.md
├── Hand tests.md                    ← the one list, every version, ticked as it is run
├── Comparison trail.md              ← how the comparison was built, kept out of the comparison
├── Audit/
│   ├── 4.5 — Opus 5.md              ← one per version per pass
│   └── …
├── Citation check/
│   └── Opus 5.md                    ← one per pass, covering every version
└── Comparison check/
    └── Opus 5.md
```

Add a level where a stage would otherwise hold an unreadable flat list; don't add one by default.

The commit the Subject sat at when a round began is captured in that round's `Round info.md` when the runner opens the round, and echoed at the top of the Verdict — so it's fixed once, durably, before any auditing happens. The same file records the audit boundary and why the round was allowed to open, so a later role can tell what the auditors were asked to judge and why another round existed. A Blender Research chat has no round to open, so it writes the commit it read at the top of its own file instead. Versions are never archived by hand — git holds the history.

Every entry in the decisions record covers, at a minimum: the round number, the starting commit, what was changed, and what was rejected with the reason. Where there is no round — Blender Research — the entry names the step and the model instead of a round number.

The record also holds entries that belong to no round. When a Subject that has already been audited is changed **outside** a round — a new feature landing in a settled spec, per Rule 7 — the agent making the change adds a line saying what was added, when, and that no auditor has seen it. This is a completion condition, exactly like signing: the change isn't finished without it. Written by the agent, never left to the Creator. Read from the bottom, those lines are a running tally of how much unexamined material a document has taken on since its last round — which is how the Creator judges when the next one is due.

Audit documents are ordinary documents: the Verdict and the decisions record each have a companion signatures file, and every individual audit carries its auditor's sign-off. The same split holds in Blender Research, which has no Verdict: its decisions record has a companion, while an audit, a citation check, a comparison check and the hand-test list all carry their sign-offs inside, since each is one agent's own record rather than a document the project maintains.

If a Subject is renamed or moved, rename its audit folder to match — the decisions record keeps the history either way.

---

**Names follow the Glossary.**

When a thing has an agreed name in the [Glossary](Glossary.md), use exactly that name — same spelling, same capitalization. Write stages as `Stage 1`, `Stage 2`, … never "the first stage".

---

**Signposts point, never restate.**

A signpost is a file whose only job is to send the reader somewhere else — like the root `README`, `CLAUDE.md`, any equivalent stub a tool needs to find its way in, and the "About this folder" note in each folder. They point (at `Start Here`, or at what the folder holds) and never hold their own copy of the thing they point to, so there is only ever one version to keep up to date.

In particular, a folder note never carries a stage's **number** or its full definition — those live in `Start Here` and the Glossary, and repeating them means a single change must be chased through many files. A one-line label saying what lives in the folder is fine.

Signposts are not documents in the full sense: they are exempt from the Signatures rule below, and from the "documents live in Preparation" rule — a signpost may sit in any folder, including `App`, to orient it.

**There are several signposts at the root, saying the same thing, and that is deliberate.** Different tools read different files when an agent starts — `CLAUDE.md`, `AGENTS.md`, `GEMINI.md`, `.rules`, `.github/copilot-instructions.md`, `README.md` — and this project is worked on from several. Each carries the same short text, so an agent lands on the same instruction whichever door it comes through. A missing one is a silent hole, so keep them in step when any changes.

Each also opens with the **same warning to Blender researchers**: stop, ignore this file, read only your question sheet and `Blender Sources/`. The same warning repeats at the top of `Start Here`, and at the entrance to `Preparation/` and `Audits/`. That is restating, which this convention otherwise forbids — allowed here because it is a guardrail rather than a fact, it must fire wherever an agent lands rather than one click away, and the thing it prevents cannot be detected afterwards. Keep the wording identical; the detail behind it belongs to the [Blender Research method](../Blender%20Research/Method.md) alone.

---

**A link is written from the file it sits in, and checked before it is left.**

A relative link counts the folders from **the document doing the linking**, not from the project root and not from the folder a reader might imagine it in. A companion file in a `Sources/` or `Signatures/` subfolder is one level deeper than the document it belongs to, and its links need the extra `../` to match — that single mistake, repeated, broke three hundred links across this project before anyone noticed, because a wrong link looks exactly like a right one until it is followed.

Two habits keep it from happening again. **Follow every link you write**, once, before you finish. And where you are naming a document rather than sending a reader to it — inside an audit, a decisions record, a brief — **write the path from the project root as plain text and do not make it a link at all**; it cannot rot, and it says the same thing.

---

**Every fact has one home.**

Write each fact — a name, a rule, a module's scope — once, in the document that owns it, and point at it from anywhere else instead of copying it. Two copies of one fact drift: one gets updated, the other quietly goes stale, and nothing flags the disagreement. This is the signpost rule above, widened from whole files to the facts inside them — and it is structural, not a thing anyone has to remember: if a fact lives in only one place, there is no second copy to forget.

One exception: an **input snapshot** — a document that records what was said or decided at a single moment and is not maintained afterwards, like the [Product brief](../Specs/Product%20brief.md) — may restate things that a living document later owns. It is frozen, so it can't drift.

*Example:* a module's connections live in the [module map](../Specs/Whole%20app.md), a written module's full scope lives in its own spec, and the Glossary holds only its name.

---

**Mark every unfinished spot with `[PENDING: what's missing]`.**

Whenever something is left to be done, tag it in plain text, e.g. `[PENDING: one-line description of the app]`. Before the app goes to production, search the whole project for `[PENDING` and clear every one — this keeps unfinished work from hiding. (Checkboxes used as live trackers are not placeholders, and don't need this. A `[PENDING]` inside a frozen input snapshot is part of the record, not open work — the sweep skips it, and the living document that owns the work carries its own marker.)

---

**Sign your work.**

Every document has a **companion signatures file** — named `<document>.signatures.md`, sitting in a `Signatures/` subfolder next to the document itself. Each meaningful change adds one line there, in this form:

`date — model (tool) — what it did`

Every agent that meaningfully changes a document adds its own line to the companion file — oldest at top, newest at the bottom. A "meaningful change" is one that changes what the content *means*; a typo or spacing fix doesn't earn a line. Granularity is one line per sitting — several meaningful edits made in one sitting share one line, rather than each earning its own; separate sittings on the same day (say, two audit rounds) each add their own. Keep that line a **brief summary** of the sitting's work — its thrust, not an exhaustive list of every edit. The full per-change detail already lives in git; a long semicolon-chain only defeats the purpose, since it's unreadable and still can't pin a change to a moment the way git can. The date records when the work was done, which stays true forever, so it doesn't clash with Rule 6.

Signatures live outside the document rather than inside it so that the document contains only its own content. This also strengthens auditor independence: signatures record who changed what and when, which is audit-trail information that auditors should not see.

**A commit names the files it is committing, never the whole working folder.** Several chats work in this one folder at once, so whatever else is changed at that moment probably belongs to somebody else. This has gone wrong three times: a whole step was thrown away after two chats committed over each other, and on another day one agent's commit carried ten files belonging to a different stage. Stage by name every time, and where work has already been swept into somebody else's commit, say so rather than rewriting the history.

**Code is signed differently.** Its authorship lives in the version-control history instead of a signatures file — so **every commit message must name the model that did the work** (for example, a `Co-authored-by: Claude Opus 4.8` line). Never rely on a particular tool adding this for you: a commit's author is always the Creator, so if the model isn't named in the message, the trail is lost.

---

**Outside facts are cited in a companion sources file. Smoozy's own decisions are not.**

A claim about the world beyond Smoozy — how SSH behaves, what Blender does, how an operating system handles a file — carries its **label** in the body, where the claim is used, and its **citation** in a companion sources file named `<document>.sources.md`, sitting in a `Sources/` subfolder next to the document. The citation holds where the fact came from, when it was retrieved, and what it does *not* cover. That split keeps the document readable for everyone downstream who needs the claim but not the paper trail, while leaving the trail one click away for anyone checking it.

A **decision Smoozy made** gets no entry. It has no source to cite. Marking it **Creator-stated** in the body is the whole record — that says who settled it, not where a fact came from — and when it was settled is already in the document's signatures file, and in its decisions record once it has one. Copying a decision into a sources file gives it a second home — and, per *Every fact has one home* above, two copies drift. Nothing under a *Sources* heading should be something Smoozy chose.

*Example:* "the port field starts at 22, the standard SSH port" earns an entry, because SSH decided that. "Files land in the operating system's download folder unless the user says otherwise" does not, because Smoozy decided that.

A sources entry restates the label as part of describing its source — a citation has to say whether it is documented or an assumption. Where the two ever disagree, **the document wins**: it is the one that owns the claim, and the sources file only records where the claim came from.

A companion file — signatures or sources — is part of its document rather than a document in its own right, so it carries no companion files of its own.

---

**Every document opens with a one-line statement of its purpose.**

The first thing in any document says what it's for. This lets any reader — including an auditor — hold the document to what it was meant to do, without hunting for that intent elsewhere.

---

**Every module's plan states its connections.**

A module never stands alone. Its plan says plainly what it leans on and what it hands to others — for example: *Rendering needs a Blender from Server Setup and settings from Render Settings, and hands finished jobs to Queue & History.* This keeps the wiring between modules visible on paper, so changing one module tells you exactly what else to check.

**A clause marker is meaningless without its document.**

From Architecture onward, a long plan labels its clauses with a short marker — `L1`, `H12`, `Z21a`. **The prefix is meaningful inside that one document only**, chosen by whoever wrote it: in `File Browser`, `L` is listing, `P` permission, `U` upload request, `X` already-exists, `N` uploading, `O` pool, `M` file management, `W` warnings, `K` continuation, `H` picker. **`M1` is defined in eight different documents.** So a marker is always cited with the document that owns it — *`Servers` H11*, never *H11* — and a marker quoted alone is unrecoverable. Added 2026-08-22, after a reading found no legend anywhere in the project.

---
