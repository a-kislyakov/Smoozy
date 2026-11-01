# Architecture — the front door

*What is in this folder, which files are plans and which are evidence, and where to start when you have been told to build something.*

*2026-10-27 — Removed the withdrawn paid-access text and its obsolete references under the Creator’s free-and-open-source decision of 2026-10-04 (wave 5); retained protections and evidence keep their force. No auditor has seen this removal.*

**Read this before anything else here.** It is the equivalent of `Blender Research/Across topics/The map.md`, which is that stage's front door and works.

## The one thing to know first

**Forty-three files sit here. Eighteen are plans.** Twenty are **frozen evidence** — what a rented server, a desktop system or a live service actually did when somebody ran something against it. Four are the stage's own bookkeeping, and one, [What Build inherits](What%20Build%20inherits.md), is written for Build. An evidence document is not a design and must never be built from. It is the thing a design cites when it needs a fact about the world.

**Read [What Build inherits](What%20Build%20inherits.md) once, before the plans.** It states what this stage leaves open on purpose and what breaks if any of it turns out badly, so a builder meets those by reading rather than by discovering. It is stale wherever `Coverage.md`, `Facts.md` or this file disagrees with it, and it says so itself.

**You can tell them apart by their names, with one exception.** A plan carries a short module name — `Servers.md`, `Download.md`. Evidence carries a long title with an em dash saying what was found — `Two more boxes — the volume, the full disk, and a day left running.md`. **The exception is `The rented machine.md`**, which is 60 KB of frozen evidence with a plan-shaped name, linked from a dozen plans. Treat it as evidence.

**Do not read anything under `Audits/`, with one exception, and the exception is a single named file.** That folder is the audit trail, and the board in it is the runner's memory rather than a builder's.

**The exception: `Audits/Blender Research/Cross-sweep/Mechanism sweep/Downstream after the repairs — Claude Opus 5.md`.** [What Build inherits](What%20Build%20inherits.md) sends a builder to it by name, because the Blender comparisons and joined answers were repaired after the citation check that had ruled every downstream citation sound, and that file is the list of the sentences in this folder — and in the row files, the sources files and the specs — that no longer rest on what the research carries. **It has no other home**: it is a work list rather than an answer, and nothing under `Preparation/` holds it. Read that one file, and no other file under `Audits/`, however relevant its name looks.

## The eighteen plans

`The shape` comes first because it settles what all the others are.

| Document | What it settles |
|---|---|
| [The shape](The%20shape.md) | Where each component runs, the boundaries between them, and the complete set and order of the documents below. |
| [Security and licence](Security%20and%20licence.md) | The two deployment trust domains and retained crossings; how the five things Smoozy holds are kept, reached and erased on Mac, Windows and Linux; how a machine's identity is remembered. |
| [State, identity and recovery](State,%20identity%20and%20recovery.md) | Every durable and volatile store on both sides; what issues each identity and what it may never be made of; desired state against observed state; the event journal; what survives what. |
| [Server protocol and transfers](Server%20protocol%20and%20transfers.md) | What rides on the authenticated SSH connection for a machine's whole life; how a request is framed, negotiated and correlated so sending it twice does nothing twice; staging, resume, integrity and publication in both directions. |
| [Rented-server runtime](Rented-server%20runtime.md) | How the server package is placed, committed, rolled back, repaired and replaced; what the supervisor is and what restarts it; how every worker is started and stopped. |
| [Desktop app and delivery](Desktop%20app%20and%20delivery.md) | What the desktop application is made of and what shell it runs in; how modules reach the operating system and each other; every local write on each system; how the app is delivered and how it replaces itself. |
| [Servers](Servers.md) | What a connected period is; the closed list of moments a connection is opened and replaced; how a break is learned of without using the connection first; repair attempts and how long they persist. |
| [Server Setup](Server%20Setup.md) | How a rented machine is made able to render, and how that act is detected, ordered, recorded, interrupted, resumed and safely re-run; how Blender builds are listed, fetched, sent up, unpacked, matched and removed. |
| [Job files and outputs](Job%20files%20and%20outputs.md) | Where Smoozy's directories sit on a machine and what every path under them is called; what a job's immutable submission manifest carries and where it is published; every place a job would write. |
| [Queue & History](Queue%20&%20History.md) | The machine-local queue and the line in it; how a job's identity, folder and label are composed; held jobs; every act on the line, and progress. |
| [Rendering](Rendering.md) | The Blender launch contract — the route that reports, the command-line window, the per-version composition, the version confirmation, the detachment — and the attempt pipeline that runs it with nobody connected. |
| [Dependency Upload & Remap](Dependency%20Upload%20&%20Remap.md) | What the dependency check reads and what a row is; how two files are told apart by contents; the search over the user's own folders; where an asset is placed and how the remap is put in force. |
| [File Browser](File%20Browser.md) | How the browser lists a machine and what each listed fact is worth; how a permission refusal is told apart and retried elevated; what an upload request is and what survives a drop, an app close and a fresh sitting. |
| [Download](Download.md) | How files come back: the manual request and the auto-download arrangement; the record of what an arrangement has brought and what it survives; the watch on the job's output folder. |
| [Render Settings](Render%20Settings.md) | The settings half of the one recorded reading of a `.blend`; the override store; frame-list tidying; the view-layer pin. |
| [Keys](Keys.md) | How SSH keys are discovered, generated, validated, fingerprinted, copied and deleted on the user's computer, and how passphrases are kept and genuinely forgotten. |
| [Telegram](Telegram.md) | Which single machine asks Telegram for what the user sends and how the fleet shares one pinned slot; how a tap reaches the machine that owns the job; how the conversation is proved to be the user's. |
| [Reporting a problem](Reporting%20a%20problem.md) | The diary, the crash report, the machine-side journal tail, and the reveal action. Added to Specs on 2026-08-25 after The shape closed; belongs to no single module. |

## The twenty evidence documents — frozen, not plans

Each is what was observed, with the commands and the machines named. **Nothing here is a decision.** Where an evidence document and a plan disagree about what to build, the plan is what to build; where they disagree about what the world does, say so rather than choosing.

**Rented machines and what they did.** [The rented machine](The%20rented%20machine.md) *(short name, evidence — the exception noted above)* · [Batch A — the shape throwaway](Batch%20A%20—%20the%20shape%20throwaway.md) · [Six machines](Six%20machines%20—%20the%20container,%20the%20webhook%20and%20the%20ready%20action.md) · [Two more boxes](Two%20more%20boxes%20—%20the%20volume,%20the%20full%20disk,%20and%20a%20day%20left%20running.md) · [The last two boxes](The%20last%20two%20boxes%20—%20the%20hook%20that%20cannot%20be%20read%20back,%20the%20full%20disk,%20and%20the%20unit%20that%20kills%20a%20render.md) · [The multi-provider batch](The%20multi-provider%20batch%20—%20the%20surface%20that%20did%20not%20vary,%20the%20Debian%20route%20that%20exists,%20and%20the%20cards%20a%20slice%20cannot%20see.md) — nine rentals, three sellers, two machine kinds · [One box with Blender on it](One%20box%20with%20Blender%20on%20it%20—%20the%20reading,%20the%20listing,%20and%20the%20names%20that%20do%20not%20come%20back.md) · [One box — the Telegram capture](One%20box%20—%20the%20Telegram%20capture%20and%20the%20port%20that%20cannot%20be%20chosen.md) · [The host key](The%20host%20key%20—%20obtained%20without%20authenticating,%20and%20whose%20it%20turns%20out%20to%20be.md) · [The Pile 3 batch](The%20Pile%203%20batch%20—%20a%20full%20mount%20under%20a%20running%20supervisor,%20an%20interrupted%20heavy%20frame,%20and%20an%20hour%20of%20silence.md) — a full mount under a running supervisor, an interrupted heavy frame, and an hour of silence

**The user's own computer.** [The computer itself](The%20computer%20itself%20—%20its%20name,%20its%20identity,%20and%20its%20clock.md) · [The user's own computer](The%20user's%20own%20computer%20—%20where%20a%20download%20lands,%20what%20it%20will%20take,%20what%20it%20will%20show,%20and%20how%20the%20app%20replaces%20itself.md) · [The manuals](The%20manuals%20—%20keys,%20secrets,%20and%20what%20cannot%20be%20told%20apart.md) — keys and secrets on each system · [The three desktop systems read again](The%20three%20desktop%20systems%20read%20again%20—%20dying,%20revealing,%20resolving,%20and%20what%20the%20shell%20does%20not%20expose.md) — dying, revealing, resolving, and what the shell does not expose

**Telegram, run rather than read.** [The pinned line](The%20pinned%20line%20—%20what%20a%20conversation%20actually%20shows%20for%20a%20pin,%20and%20the%20filter%20that%20had%20been%20eating%20the%20bot's%20messages.md) — what a conversation actually shows for a pin, and the filter that had been eating the bot's messages

**The licence platform and the seller.** [Where the licence service runs](Where%20the%20licence%20service%20runs%20—%20the%20hosts%20read%20against%20what%20it%20must%20do.md) · [The host itself](The%20host%20itself%20—%20the%20licence%20platform%20run%20rather%20than%20read.md) — the platform run, not read · [Lemon Squeezy](Lemon%20Squeezy%20—%20the%20licence%20service%20read%20against%20what%20Access%20needs.md) · [The seller itself](The%20seller%20itself%20—%20the%20store%20observed%20rather%20than%20read.md) · [The refund event](The%20refund%20event%20—%20the%20routes%20to%20it,%20read%20rather%20than%20tried.md)

## The four bookkeeping files, and the one written for Build

[Method](Method.md) is the runner's instructions for the stage and is not a builder's document. [Coverage](Coverage.md) holds every promise this stage owed an answer to, quoted, with where the answer is — and the count. [Facts](Facts.md) holds the numbered facts the rows point at. This front door is the fourth. `Rows/`, `Sources/` and `Signatures/` hold the per-document row files, the citation companions and the writing signatures.

[What Build inherits](What%20Build%20inherits.md) is not bookkeeping and is not a plan. It is this stage's handover: what is left open on purpose, what breaks if it turns out badly, and which rulings' costs Build meets. It is written to be read once, at the start of Build, by an agent that has read nothing else of this stage's history — so read it before the plans, as the top of this file says.

## Where to start, by what you were told to build

`Coverage.md` is keyed by **quoted spec sentence**, so it only helps if you already know the spec's exact wording. This table is the other way in.

| You were told to build… | Start at |
|---|---|
| a whole module | that module's document above, then its `Rows/` file |
| how a job survives the app being closed | `Queue & History`, then `Rented-server runtime` (the supervisor) and `State, identity and recovery` |
| what happens when the connection drops | `Servers` — connected periods, breaks, repair |
| moving bytes either way, resume, integrity | `Server protocol and transfers` |
| starting a render | `Rendering` (the launch contract), then `Job files and outputs` (where it writes) |
| finding and sending up what a `.blend` needs | `Dependency Upload & Remap` |
| making a bare rented machine able to render | `Server Setup` |
| anything about a secret, a key or a trust boundary | `Security and licence` first, then `Keys` |
| any durable store, identity or journal | `State, identity and recovery` |
| the app's window, shell, install or update | `Desktop app and delivery` |
| notifications, or driving a job from a phone | `Telegram` |
| what runs where, or which document owns something | `The shape` |

## Reading order for the whole stage

Read the **spine** first, in this order. Each document's upstream dependencies have already been settled by the documents above it.

| Order | Document | Why it sits here |
|---|---|---|
| 1 | [The shape](The%20shape.md) | Defines where each component runs and what every other document is. |
| 2 | [Security and licence](Security%20and%20licence.md) | The two deployment trust domains, the five held things, and the constraints every store and message must honour. Nothing below can be judged without it. |
| 4 | [State, identity and recovery](State,%20identity%20and%20recovery.md) | Every durable and volatile store, every identity, desired-versus-observed state. Used by every module. |
| 5 | [Server protocol and transfers](Server%20protocol%20and%20transfers.md) | The SSH surface, request framing, transfer contract. Takes identities from 4; supplies primitives to every server-using module. |
| 6 | [Job files and outputs](Job%20files%20and%20outputs.md) | Directory layout, submission manifest, output redirect, publication contract. Takes transfer rules from 5 and identities from 4. |
| 7 | [Rented-server runtime](Rented-server%20runtime.md) | Package placement, supervisor, worker lifecycles, machine-local endpoint. Implements 4 over the protocol from 5 and reads file rules from 6. |
| 8 | [Desktop app and delivery](Desktop%20app%20and%20delivery.md) | The desktop shell, platform adapters, local writes, delivery and updates. Hosts every feature module; consumes 2, 4 and 5. |

The **feature modules** follow. Their mutual connections (seams) mean no linear order is perfect; the order below minimises forward references. Where a document cites one not yet read, the citation names the section and the reader meets it when that document comes.

| Order | Document | Why it sits here |
|---|---|---|
| 10 | [Keys](Keys.md) | Key discovery, generation, passphrases. Consumes spine only (2, 4, 8). Servers waits on it. |
| 11 | [Servers](Servers.md) | Connection lifecycle, reconnect, hardware readings. Takes keys from 10 and transport from 5. Every server-using module below enters through it. |
| 12 | [Server Setup](Server%20Setup.md) | Making a machine able to render. Needs 5, 6, 7 and 11; reads live-job use from 17 (forward reference). |
| 13 | [File Browser](File%20Browser.md) | Remote listing, uploads, permissions. Needs 5, 6, 8 and 11. Hands files to 14 and 15. |
| 14 | [Render Settings](Render%20Settings.md) | Settings reading, overrides, submission payload. Takes a `.blend` from 13, a Blender from 12, workers from 7. |
| 15 | [Dependency Upload & Remap](Dependency%20Upload%20%26%20Remap.md) | Dependency check, search, remap. Takes files from 13, Blender from 12, transfers from 5, storage from 6. |
| 16 | [Rendering](Rendering.md) | Launch contract, stop, re-run. The heaviest consumer: needs 5, 6, 7, 11, 12, 14 and 15, and parts of 17 and 19 (forward references). |
| 17 | [Queue & History](Queue%20%26%20History.md) | Machine-local queues, held jobs, history, previews. Takes jobs from 16 (mutual dependency). |
| 18 | [Download](Download.md) | Manual and automatic downloads. Takes transfers from 5, inventories from 6 and 14, job events from 17. |
| 19 | [Telegram](Telegram.md) | Bot, receiving slot, cross-machine routing. Runs in 7 after 12 installs it; reads 6 and 17; submits 16 commands. |
| 20 | [Reporting a problem](Reporting%20a%20problem.md) | Diary, crash report, reveal. Added to Specs after The shape closed. Consumes 2, 4 and 8. |

**Three mutual dependencies cross the feature modules and are read as seams, not as blockers.** Rendering (16) and Queue & History (17) each cite the other: Rendering hands jobs and events to Queue & History, while Queue & History supplies job identity and admission back. Rendering (16) and Telegram (19) have the same structure: Rendering exposes stop and re-run, while Telegram submits those commands and supplies phone interactions. Read Rendering first in both cases; the forward-referenced sections are named in its own opening and are self-contained when reached.

**One forward reference in the spine.** Desktop app and delivery (8) names feature modules it hosts — Keys, Servers, File Browser, Download, Queue & History, Reporting a problem — by the services it exposes to them. Those names are meaningful after the module is read, not before; reading 8 first is still correct because the services are the dependency direction.

## Markers, and how to cite one

Every plan opens with a Row → anchor → marker table, and its clauses are labelled `L1`, `H12`, `Z21a` and so on. **A marker's prefix is meaningful inside its own document only** — `M1` is defined in eight different documents — so **a marker is never quoted without naming its document**. [Conventions](../Groundwork/Conventions.md) carries the rule.

**CLOSED FOR TWO OF THE THREE, 2026-08-31.** `Security and licence` gained **156** clause markers and `Licence service` **130** — 286 in all, in the same convention `Server Setup`, `Rendering` and `Job files and outputs` use, a per-section prefix at the head of each top-level clause. **It was a labelling pass and nothing else**: every marked line was rebuilt by stripping its marker and compared byte for byte against the original, 156 for 156 and 130 for 130, with no mismatch. No wording, ordering, meaning or section number moved, and every heading is byte-identical, so **every existing section citation still resolves exactly as before.**

**Why it was worth doing.** A check finished the same day read all 504 coverage-row summaries against the clauses they cite and found **45 per cent had drifted**, most often a pointer that still resolves but no longer says what the summary claims. **A section-level pointer is the worst possible case of that**, because a section can grow indefinitely underneath one number and no pointer ever goes stale. Build reads rows.

~~**`State, identity and recovery` still has markers only in places, and that is now the whole of the gap.**~~ **CLOSED FOR THE THIRD, 2026-09-02.** That document gained **124** clause markers, in the same convention and by the same verification — every marked line rebuilt by stripping its marker and compared byte for byte against the original, 124 for 124, with no mismatch, and no wording, ordering, meaning or section number moved. **So all three are marked and the marker gap is closed.** Its pre-existing `I`, `D` and `E` series were left exactly as they stood; the ten sections that had none took a fresh prefix apiece. **And the roughly sixty section-level citations of the pair in `Rows/` and `Coverage.md` have not been upgraded** — the markers now exist for whoever does that as one deliberate pass, rather than half of them being upgraded and the rest left, which reads worse than a uniform state.

~~**Three documents are cited by section number alone and have no markers**: `Licence service`, `Security and licence`, and `State, identity and recovery` beyond a handful. `Security and licence` §3 is cited twenty-two times and spans hundreds of lines. That is a known gap, not a different convention.~~

## Blender

**No Blender fact is settled here and none may be looked up here.** Start at [Blender Research/Across topics/The map.md](../Blender%20Research/Across%20topics/The%20map.md), which lists the questions a builder arrives with and names the joined answer to read first for each. Read [What the research cannot say](../Blender%20Research/Across%20topics/What%20the%20research%20cannot%20say.md) before leaning on any Blender finding — it is the ceiling.

**One warning about the word *map*.** `Job files and outputs` P10 uses *the map* for its own remap pair table, so searching this folder for it returns the wrong thing. The Blender front door is the file linked above.
