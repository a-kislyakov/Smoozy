# Method

*How a promise made in a spec becomes a plan the app can be built from: what Architecture answers, in what order, how it is written, how it is checked, and the three places the Creator comes in.*

The clearest way to see this stage is to follow one promise all the way through it. So take a real one, from the [Rendering spec](../Specs/Rendering.md):

> Every place the job would write is brought under the job's folder.

Watch what becomes of that sentence.

## Architecture answers promises

The specs already say what Architecture owes. A few paragraphs under that promise, Rendering says outright that how the redirect is arranged is not established and that this is where it lands. It does the same four more times in the same document — the machine's cards, the route a render has to be started by, all of that surviving Smoozy being closed, and that a single job either reaches the machine whole or does not exist. A sixth, the remap being in force when the render loads the file, says only that it is *still open* and names no stage, which is the ordinary case: most of what this stage owes is never labelled. Rendering is not even the heaviest: Servers names Architecture eleven times and Queue & History ten.

So Architecture is not a fresh act of design. It is a list of answers to a list somebody already wrote and the Creator already approved. Everything below follows from that.

## Starting or resuming the stage

**"Let's carry on with architecture", or anything close to it, makes you the architecture runner.** That role is set out at the end of this document. Its first act in any chat is to read the board — `Audits/Architecture/Progress.md` — which says where everything stands and what the next thing is.

**What the board holds**, since it carries more here than the Blender Research board does. That stage is a run of rounds and its board reads as a story. This one is a long list worked in an order, so the board is short and answers four things: which document is being written now and what its exact next action is; which rows that document is answering; what is waiting on the Creator and since when; and what is waiting on a machine. Everything else — the rows themselves, the facts, the counts — lives in `Coverage.md`, and the board never restates it. If the two ever disagree, `Coverage.md` is right and the board is stale.

Where the board does not exist yet, this is the first chat: make it, then start at the next section.

## Every promise gets a row

The list is `Preparation/Architecture/Coverage.md`. Building it is the first work of the stage.

**It is built one document at a time, by one agent each.** The specs run to a hundred thousand words, and one agent reading all of them spreads itself thin and returns half a list — the same problem [Auditing](../Groundwork/Auditing.md) already names for a Subject that is too big. So each spec and each design document gets its own agent, all reading to the same bar.

**Three things earn a row.** The document says outright that something is Architecture's. Or it carries an unfinished-spot marker naming Architecture. Or it promises an outcome that cannot be reached without a choice that is not obvious, or that rests on an outside fact, or that needs something else to hold before it can be kept.

**That third test has a floor, and without it the list swallows the whole app.** Every spec opens by saying not one word about how it is built, so *this document does not say how* is true of nearly every sentence in it — an agent applying that alone would file a row for every promise Queue & History makes, hundreds of them, none of them architecture questions. Ordinary construction is **Build's**, exactly as most of how a screen is drawn is. A row is owed where there is a decision to take, a fact to get, or a dependency to satisfy. Nothing else.

**So the sweep is calibrated before it is spread.** One document is swept first, the runner reads the result against that bar, and the worked list goes to the other agents as the example. Twenty-four agents started cold from a written test alone will each read it differently, which is the fault the Blender Research method spends a paragraph preventing when it insists every version gets identical wording.

**A spec's *unresolved* is checked before it becomes a fact.** Several are stale — the label sits in a spec while the owning sources file and the Blender Research comparison say the question was answered. So a sweeping agent that meets one opens the sources file and the comparison before writing the row, and where the spec is out of date it says so rather than parking a question that would cost a rented machine to answer twice.

**Most markers name no stage, and the ones that do are the minority.** Of the two hundred-odd unfinished spots in the specs and the design, only about seventeen name Architecture; thirteen more sit under Server Setup's own heading saying what is not settled, naming nobody. So the third test above does most of the work, and an agent that only collects the named markers has swept nothing.

**The design documents owe far fewer rows than the specs**, and that is expected rather than a sign of a thin sweep. Design says what the user sees, and most of how a screen is drawn is Build's rather than Architecture's. What Design does hand over is the handful of things it says outright — the typography rules being applied by the thing that renders the text is the example.

**A row takes a number and keeps it for good**, never reused and never renumbered, exactly as a fact does. A row that turns out to be wrong or to belong to Build keeps its number with a note saying so, rather than vanishing and taking the count down with it.

**A row holds the promise quoted**, which document it came from, which facts it waits on, the constraints its answer must honour, and where its answer is. **The constraints live in the row rather than in the sweep's own trail**, because everything under `Audits/` is outside this stage's reading and an agent writing an answer must never have to open it to find what the answer has to survive. A sweep file records how a row was found; `Coverage.md` records what the answer owes. Quoted rather than described, and that is the important part. The checking below catches a promise with no answer. It cannot catch an answer to a promise somebody misread — so the promise sits on the page beside its answer, where a mismatch shows. **Where the answer is** means the sentences that keep it, clause by clause where the promise has clauses, rather than the document it sits in. That is what lets the gate below be a matching exercise rather than a judgment.

When every document has been swept, a fresh agent reads each one against the list and rules **both ways**: what was missed, and what does not belong. A check that can only add is a check that only ever grows the stage.

**Two things the sweep would otherwise pass over.** The [mockup](../Design/Mockup/Smoozy.html) is neither a spec nor a design document and is the single home of every design value, so it is swept like the rest — it may well owe no rows, and that is an answer. And of the architecture items in [Open Threads](../Groundwork/Open%20Threads.md), the ones that are promises or facts become rows and leave that file, while the ones that are parked **ideas** — rendering from a copy of the `.blend` is one — stay there with their Due lines, because a list of promises has nowhere to hold an idea.

**The sweep is not done once.** Design closed on 2026-08-25, and the three modules with no design document — `Access`, `Rendering` and `Telegram` — are **a settled arrangement rather than a debt**: the Creator ruled on 2026-08-23 that those three go through Design **drawn rather than written**, so for them the mockup is the plan and it is swept like any other source. Nothing further is owed from that quarter. A spec changes between rounds too, whenever a want lands in one under Rule 7 or a correction is carried in — and a stage close can bring a wave of them rather than one at a time. So: a document written after the sweep gets its own agent when it arrives, a swept document that changes is swept again by one agent and its rows added or reworded, and the whole sweep is re-run at the stage close beside the two gates. **What tells anyone a document changed is the gate's quote check** — a quote that no longer matches its source is a document that has moved, and that is the only trigger this has. Without that, the finish condition at the end of this document counts a list that stopped being true.

## The order the rows are answered in

Rows are not picked off in any order that suits. Three things come first, and the middle one is the whole stage's spine.

**A few facts, and only a few.** Most facts can wait until the row that needs them comes up. The ones that cannot are the ones that decide the shape below — whether a queue can live on a rented server with nobody connected, whether a running render can be reached and told to end. Answer those first, because a shape chosen without them is a guess.

Those two carry a trap. Neither can be tested properly before the shape exists, because there is nothing on the machine yet to keep a queue alive or to send a stop to. So they are answered twice: first at the plain level, with a throwaway, asking only whether a program can outlive a connection, come back after a reboot, and be reached and told to end — and again at the promise level once the shape is written. The first answer is enough to choose a shape and is not enough to close the row.

**The Telegram facts come before the Telegram work**, not during it. Open Threads says so outright for the two that hold that module up, and if either fails the module goes back to a different promise rather than being reinterpreted.

**Then the shape.** One document, before any other, settling what runs where: the app on the user's computer, whatever sits on each rented server, the piece of Telegram that lives there too, and the licence service that sits away from the user's computer. It also settles **what the other architecture documents will be**, because until that is decided there is nowhere for an answer to go.

**Then the spine, then the modules.** The spine is the few documents everything else leans on — what each piece remembers and what survives what, how the pieces talk, how the six things Smoozy holds are kept safe, how the app is built and updated. Our redirect promise lands in one of them. The module documents come last and most will be short, because the spine will already have answered them.

## Some rows wait on a fact

**Ours turns out not to, and finding that out is the first thing the row costs.** Rendering labels the redirect's central question unresolved — whether an output path can be imposed on a render without writing it into a file — and it is not. The research answered it on both machines in all four versions, and the sources file that owns it says so plainly; four specs still carry the stale label. What the row actually inherits is a version difference rather than a gap: the two values a file output node's path is held in are named one way in 4.5 and another from 5.0, so the answer is written per version.

**A Blender fact is looked up, never asked again here.** A row that appears to want a Blender question is a row nobody has looked up.

**Where it is looked up is fixed, and this stage does not get to choose.** Start at `Preparation/Blender Research/Across topics/The map.md`, which lists the questions a builder arrives with and names, for each, the joined answer to read first. Where an answer spans topics it has already been joined and checked; read that. Open a comparison question only where the map, a joined answer, or a spec's own sources file names it — that question, and no more of the file. Everything else in that folder, and everything under `Audits/`, is outside this stage's reading. **Nobody here assembles an answer out of several topics themselves**: the one time this project did, a claim that was true of its own test conditions and false as a general rule reached a spec. The [Blender Research method](../Blender%20Research/Method.md) owns the rule and why it is drawn this way.

What genuinely falls to this stage is everything Blender is not: the rented machine, the connection, the providers, the operating systems the app itself runs on. Rule 2 governs those exactly as it governs Blender — never from an agent's memory, never from whatever is lying around the open web, always labelled, and the citation in a companion sources file. They arrive two ways.

**A fact got by running something.** The answer says what was done, on which machine, and what came back word for word. Two rules hold, and both are the [Blender Research method](../Blender%20Research/Method.md)'s rather than restated here — *a test that cannot fail is not a test*, with the line in the entry saying what a negative result would have looked like, and *the machine must be one whose exact description is written down*. **A machine rented for this stage has its description written into one of two places, and they are the two a reader actually opens: the batch's own input document, where the whole batch is described in one table, or — for a rental taken to answer a single document's question — that document's companion sources file.** The rule is that a description is written down somewhere named, not that it is written down in any particular file. *(Corrected 2026-09-02, at the Architecture stage close. This sentence used to send the reader to the [Blender Research method](../Blender%20Research/Method.md)'s equipment section and say a rental for this stage is added there like any other. **Every Architecture rental's description had in fact gone to one of the two homes above**, and only the multi-provider batch's machine was ever added to that equipment section — deliberately unnumbered, because it is not equipment a blind hand test may use. Pushing this stage's rentals into that numbered sequence would put non-Blender-Research equipment into a blind run, so the sentence was brought to where the descriptions already are rather than the descriptions moved to the sentence.)*

**A fact got by reading.** The permitted source is **the thing's own documentation** — a provider's, a service's, an operating system's, a standard's, a library's — and nothing else: not a forum, not a search result, not an agent's recollection of how something works. The list is that wide because most of this stage's readable facts are not about providers at all; where each operating system keeps its SSH keys is one the Keys spec hands over by name, and the specs already source the default SSH port from OpenSSH's own manual. The answer carries the same three things a documented Blender finding carries: the exact words quoted, the address, and the date read. Fast-changing outside facts are the reason for the date; a provider's pricing page read in August says nothing about September.

**And a thing's own documentation can be wrong about the thing, which this route assumes it is not. Added 2026-08-22, observed.** A seller documented a field as required on its own creation call and its own offers list carried a value with both a name and an internal identifier; **passing the name returned `500 Unexpected error occured` and passing the identifier succeeded**, and the documentation named neither the expected form nor the identifier ([The multi-provider batch](The%20multi-provider%20batch%20—%20the%20surface%20that%20did%20not%20vary,%20the%20Debian%20route%20that%20exists,%20and%20the%20cards%20a%20slice%20cannot%20see.md)). **The working value was found by trying, not by reading.** **What follows is not a weakening of the rule** — the alternatives are a forum and an agent's memory, and both are worse. **It is that a documented fact about a thing that can be exercised is exercised where exercising it is free**, and where a reading and a run disagree, the run is what is recorded and the disagreement is recorded with it.

**A manual page is not the boundary of a project's documentation, and this stage has been reading a narrower shelf than the rule allows.** Added 2026-08-21, out of the verification round on the sanity sweep. **Release notes, protocol and format files, changelogs, published package manifests, a vendor's own reference implementation, and headers shipped in the operating system are the same authority as a manual page** — they are the thing's own publication. **The cost of not knowing this was measured**: two questions were parked as unanswerable *because the answer was not in a manual page*, and both fell in minutes — which OpenSSH version moved the default fingerprint hash off MD5 (its own release-6.8 notes), and whether the private-key format can separate a wrong passphrase from a corruption (its own `PROTOCOL.key`). A third was parked against a rented machine and answered by the distribution's own published image manifest, and a fourth was a symbol-to-number mapping sitting in a header on the disk the probes had already run on. **The rule that follows:** before a question is parked for want of a source, the thing's own publications are looked at, not only its manual. **And a parked question says which publication was looked at**, so the next reader can see what was not.

**A fact takes a number and keeps it for good**, never reused. One fact serves several promises, so it needs a name of its own that a row and a spec's sources file can both point at. The facts are listed in `Coverage.md` beside the promises, in their own section, and each row names the facts it waits on. The answers land in **input** documents alongside the architecture, each carrying its number. [The rented machine](The%20rented%20machine.md) is the first of them and sets the shape — it is frozen and carries no numbers of its own, so where it already answers a fact, the number and the pointer live in `Coverage.md`.

**Two things about renting itself, learned by doing it and easy to lose. Added 2026-08-22, out of an audit of the batch's routing.**

- **A rental is checked rather than assumed to have started.** One of the nine machines taken on 2026-08-22 came from an offer marked rentable and ended at `Error: GPU error, unable to start instance.`, refusing the key throughout; it was destroyed unused. **That is the fourth rental this project has lost to a box that never came up.** A batch that assumes its machines exist loses the questions riding on them and reports nothing rather than a negative.
- **On at least one seller a host's offers are alternative carvings of one machine, not independent rentals.** Taking a one-card slice made the same host's two-card and four-card offers unrentable immediately. **So a controlled pair that needs two sizes at once cannot be rented on one host** — it is a sequential test, or it is two hosts, and a batch that plans it as one rental plans something that cannot happen.

**The facts wanting a machine are batched, and asked for once per batch.** A runner that asks for a machine every time it meets such a fact stalls the stage over and over. The runner collects them as it goes and puts the whole list at once, saying what each one needs the machine to be — a card's size or the absence of a screen can decide whether a question is answerable at all.

**Agents act on the Creator's behalf with the Creator's keys, and do not ask each time.** Creator-stated, 2026-08-17, in their own words: *"I am 100% okay with stuff being done on my behalf."* That covers renting machines, paid services, a bot, deploying a throwaway — whatever a fact needs. It replaces the old assumption that only the Creator can rent a server, and with it the reason this stage stalled between sittings.

**Five things bound it, and none of them is a formality.**

- **Cheap by default.** The Creator asked for it plainly and this stage has already shown what it looks like: a whole day of machine work at low cost. Pick the cheapest thing that can answer the question, not the most convenient.
- **A rental is destroyed the moment it has answered**, and the account is checked empty afterwards — no instances, no volumes, no snapshots. Anything left running between sittings is on the board with the reason and the date it must go, and that is the exception rather than the habit. **That habit is the only protection there is, established 2026-08-20**: the provider publishes no way to cap a rental's life — `duration` is not a parameter of the create call and the end date is documented as locked at rental — and a watchdog inside the container cannot end it either, because the kernel forbids a process from killing its own namespace's init. Both are recorded in [Two more boxes](Two%20more%20boxes%20—%20the%20volume,%20the%20full%20disk,%20and%20a%20day%20left%20running.md). **A rental left running runs until somebody destroys it by hand.**
- **The batching above still holds.** It was never only about asking; it is about not paying for a machine that sits idle between questions.
- **Nothing that is already the Creator's is touched.** Their accounts hold things this project did not make. Everything pre-existing is read-only, whatever it is called, and an agent that finds itself near one stops and says so rather than working around it.
- **Money at a different scale still stops.** Upgrading a plan, starting a subscription, adding a payment method, or a spend beyond the small change this stage has run on so far — those are named and put to the Creator, not assumed under this ruling.

**Where the keys live, how they are named, and what an agent may do with one is a project-wide convention** and is written once, in [Conventions](../Groundwork/Conventions.md) — `~/.smoozy-keys/`, outside the repository, one file per service, read only the file the work needs, stop rather than guess where it is missing, redact it in any output. It is not restated here. **What is this stage's:** the runner points each agent at the one file it needs and nothing more, rather than telling it the folder exists and letting it look.

**Batched does not mean once for the stage.** Renting a machine, answering a batch, destroying it, and renting another a few days later for a different batch is fine and expected. Label: **Creator-stated** (2026-08-12). That matters for the order above: the two facts that decide the shape cannot wait for the sweep to finish collecting everything else, so they get their own machine, and the questions the sweep turns up later get theirs. What is being avoided is a machine billed by the hour sitting idle between questions, not a second rental.

**A tester is not kept blind to the project.** A Blender researcher is blind because it would otherwise hand this project's own guesses back as findings. These questions are about what Smoozy needs, and they cannot be asked without saying what Smoozy is. What replaces blindness is the evidence bar above: the answer is a machine's own words or a quoted page, not a claim. The one thing withheld is what the plan hopes the answer will be.

## The answer is written for the agent that will build from it

This is the rule the stage turns on, so it is written plainly.

**An architecture document is written for the agent that will build the app, and for nobody else.** Every line either names a promise and says how it is kept, or names a constraint that forced a choice. No restating the spec — the promise has one home and it is there. No explaining, no reassuring, no reasoning for a reader. A reason appears only where it is itself a constraint, and then in one line.

**It uses the exact technical word.** A plain-English paraphrase is ambiguous to the thing building from it, and ambiguity here becomes a wrong app. [Rule 1](../Groundwork/Rules.md) carves this out for the stage and sets the scope, which is narrow — this stage's own documents and their audit trail, and nothing else.

**It is not made readable for the Creator, deliberately.** They do not read this stage, and writing for them would make every document ten times longer for no gain. The specs are the readable half of the pair, and they are what drives this one.

## Which model writes it

**Where a document meets the test below, it is written by the strongest model available, and that model is ~~Fable~~ Claude Opus 5.** Label: **Creator-stated** (2026-08-18, and the name corrected by the Creator on 2026-08-31: *"If anything anywhere in the protocol says to use Fable — use Opus 5."*). **The ruling is the strongest model, not the name**, and the name was the only thing that moved. Everything else — every gate, every sweep, every citation check, every machine run, and the ordinary documents — runs on whatever the runner would have used anyway. This is not a general upgrade and it is not the roster in [Auditing](../Groundwork/Auditing.md), which covers the audit roles and is untouched.

**Writing is the only step here with nothing behind it.** An audit has three independent seats, then a consolidator, then two counted gates, then a bounded close — a weak audit is caught by the rest. One agent writes a document, and everything downstream inherits whatever it decided. The gates make that worse rather than better: they check that every promise has an answer and every section has a promise, and they say outright that they do not ask whether the answer is any good. So an answer that is complete, coherent and wrong passes everything this stage has except a machine.

**The test, and all three parts have to hold at once.**

- **The answer is invented rather than applied.** A module document that mostly cites spine answers already settled does not qualify, however long it is.
- **A wrong answer fails quietly.** The render succeeds and is wrong, or the app reports success over something lost. Where a wrong answer refuses loudly, the fault is found at Build for the price of finding it.
- **Several other things read it.** A record, an identity rule, a map — something a later document leans on rather than a decision that stays local.

**Where that lands, on this stage's own reading of the feature-module documents.** In order of need: `Dependency Upload & Remap`, because it is the one place where a wrong answer produces a finished render that is wrong and says nothing; `Queue & History`, because its faults need three things true at once and no static reading accumulates state; `Telegram`, because the transport itself fights the promises and two of its races are already demonstrated failures; `Server Setup`, because Fact 50 came back negative and the elevation and restart-and-resume answers differ by machine kind; and `Rendering`, whose per-version launch contract is where a mistake becomes a silent no-op. **Anything not on that list is put to the test rather than to the precedent** — a document arriving later can qualify, and one on the list can stop qualifying if the spine settles its hard part first.

**One place in the checking earns it too:** a seam round between two module documents, because a seam Subject is two documents read together and is the largest thing anybody reads at this stage. That is the same size argument that put DeepSeek V4 Pro in Kimi K3's seat.

**Where it buys nothing, and the record rather than an opinion says so.** The stage's most valuable catch was running the licence platform, which found a ceiling that granted thirty claims against two slots — no reading of any quality would have found it, and it cost nothing. The next were citation checks, which reversed one fact's headline and prevented a false escalation, and caught a result true of its own test conditions and false as a rule. Inside the rounds, the cheapest seat present raised the finding that killed the central clause of Row 286. And the runner's own bookkeeping has been the fault nine times, which is clerical. None of that is fixed by a stronger reader.

**How it is applied.** The runner names the model in that document's writing brief and nowhere else. Nothing about the gates, the round, the roster or the close changes.

## Trying it

A doubt about a spec is settled by argument. A doubt about how something is built can be settled by building a small throwaway and watching what it does. That is an ordinary step here, not a last resort.

It matters more than it looks. With the Creator out of this stage, it is the only thing left that can prove an agent wrong rather than out-argue it. And a wrong architecture surfaces at Build, where there is already code standing on it.

A throwaway is written to be thrown away. It goes nowhere near `App/`, it is not the start of the build, and what survives it is the finding, recorded the same way any other observed fact is.

## Checking, part one: two gates that are counted

Neither of these needs the Creator, and both are meant to give the same result whoever runs them.

**Traced.** Every row the document was meant to answer has an answer, and no section of the document stands without a row or a constraint behind it. A row with no answer is a gap. A section with no row is something being built that nobody asked for — which is Rule 7, catchable by counting for the first time in this project.

It is a matching exercise and not a judgment, and that is only true because a row points at the sentences that keep the promise rather than at the document holding them. Where a promise has clauses, each clause is matched. **What the gate does not ask is whether the answer is any good** — that is the auditors' work below, and a gate claiming to do it would be two agents guessing differently at the one check meant to be certain.

**And it checks the quote itself against the document it came from.** Everything here leans on that quote being faithful — the matching above, and Auditing's decision that an architecture document needs no seam round against the spec it serves, which rests on the spec's own words already being inside the Subject. Nothing else ever reads the two together. So the gate searches each row's quote in its source document, which is a search rather than a judgment, and a miss means one of two things that both need acting on: the quote was trimmed to the convenient half of a promise, or **the source document has changed** since the sweep.

It reads at the level of a section, never a sentence. Most of an architecture document is how an answer is made, and no spec promises that; a gate flagging every such line would flag everything and mean nothing. **The shape document is exempt from the second half**, because no spec promises a shape and no spec promises a list of documents — it is answered against the whole list rather than against rows.

**Tried.** Anything the document rests on that nobody has observed is either tested, or labelled unresolved with what breaks written down. That is Rule 2 as it already stands; the gate is that it is checked rather than hoped for.

**A fresh agent runs both, using the [Architecture Gates](../Groundwork/Templates/Architecture%20Gates.md) brief, over a document as soon as it is finished and before it is audited** — gaps found by counting are cheaper to fix than gaps found by three auditors. The architecture runner does not run them, because it keeps the list and would be marking its own work.

**They run twice more.** Once at a round's close, together with the same brief's bounded Verdict and repair check, because the implementer has just changed the document and a trace that passed before those edits is not the trace that stands after them. And once over everything at the stage close. The close check is not another audit: it checks only the Verdict, the changed passages and their immediate neighbours, and the two counted gates. It does not hunt untouched passages for new architecture findings.

## Checking, part two: the ordinary loop, almost without the Creator

Beyond the gates, an architecture document gets one discovery round for a material version, and [Auditing](../Groundwork/Auditing.md) owns what differs — readiness opening the round rather than the Creator picking it by feel, three auditors rather than five, no seam round against the spec or the design the document serves, the consolidator's middle pile becoming *this comes back to Specs*, the cap of two not applying to it, the written audit boundary, and the bounded close. Read it there; none of it is repeated here.

Two things about it belong to this stage rather than to that loop. **Seams between two architecture documents hold as usual**, since the shape document and the spine documents describe the same pieces from different sides. And **the traced gate is the nearest thing to the Creator's reading of the fix-it list** — it catches a promise that stopped being kept, which is the giveaway when a consolidator has quietly decided something that was theirs. It is not a substitute for their judgment, only for that one check; it catches neither an answer that is merely poor nor the second kind of escalation below, which is why that kind stops an agent outright.

Scenarios are replayed at this stage's altitude, as they are at every other.

**The architecture runner runs these rounds.** It is that role and the audit runner both, and nothing else appoints one — it does no auditing either way, which is all the separation the audit runner's job ever needed.

**The briefs this stage owes are its own, and the existing ones would break it.** Three sit in `Groundwork/Templates` beside the standing set, and the fourth is a branch inside one of them.

[Architecture Sweep](../Groundwork/Templates/Architecture%20Sweep.md), because building the list is the first work of the stage and twenty-four agents composed by hand will drift. It carries the three tests, the floor under the third, the stale-*unresolved* check, and where a Blender fact is looked up.

[Architecture Auditor](../Groundwork/Templates/Architecture%20Auditor.md): the rows and their quoted promises handed over, the standing template's clause making code-speak a defect replaced by Rule 1's exception, the leave-alone list required, and one question asked of every answer — does it put anything in front of the user that no spec says they get, a place to look, something to type, a thing that can be lost. That last one matters more than it reads: no gate catches the third escalation below, and without it the only thing that ever notices is the agent that wrote the answer. It also tells the auditor not to repeat the counting, since a gate has already done it and two agents guessing at one check is what that gate exists to prevent.

[Architecture Consolidator](../Groundwork/Templates/Architecture%20Consolidator.md): applying the concrete failure-path and audit-boundary tests before it sorts; recording the defect count without using it to schedule another round; no cap on the comes-back-to-Specs pile; and the Verdict written without a Creator gate where that pile is empty.

And a **runner branch** for an architecture Subject in the standing [Runner](../Groundwork/Templates/Runner.md) brief, beside the ones it already carries for a seam, a scenario and a set of findings.

A fifth was written when the sweep finished: [Architecture Both-ways Check](../Groundwork/Templates/Architecture%20Both-ways%20Check.md), for the pass that reads each document against the list and rules both ways. This document required that pass without naming a brief for it, and thirteen agents composed by hand drift — which is the argument that earned the sweep its own brief.

[Architecture Gates](../Groundwork/Templates/Architecture%20Gates.md) is the sixth. It gives the fresh checker one stable brief for the before-audit gates, the round-close gates and bounded repair verification, and the stage-close gates. Its prohibition on auditing untouched material is what keeps a close check from silently becoming another discovery round.

## The three places the Creator comes in

All three are the same thing seen from different sides: **Architecture has run into something only Specs can settle.**

**The architecture cannot keep a promise the spec makes.** Our redirect row does not end this way, since the research settled it. A row that might is Server Setup's restart: whether a rented server can be restarted over the connection Smoozy already holds is unresolved, and the whole-app boundary carves out one exception to Smoozy never touching a machine's power precisely so a setup step needing a restart does not stop and send the user to their provider. If it cannot be done, that promise cannot be kept, and what replaces it — sending the user away mid-setup — is the outcome the Creator's own ruling exists to prevent. So it is not an architecture decision at all. It is a change to what the user gets.

**Or a fact the promise cannot be kept without turns out to be unanswerable.** *Unresolved* is a safe word in a spec because Architecture is coming to settle it. Here there is no later stage that settles facts — the next one is code. So a row whose promise merely gets weaker still closes as unresolved with what breaks written down, and a row whose promise **cannot stand at all** without the missing fact is this escalation rather than a closeable row. The agent stops, and the Creator is shown what the user would get instead. Otherwise the stage finishes by counting, Build starts, and the hole is found with code already standing on it.

**Or the answer costs the user something the spec does not say they get.** The specs say this themselves, in the place they hand over the biggest single question in the stage. The [Whole app spec](../Specs/Whole%20app.md) settles that how the six things Smoozy holds are kept safely is Architecture's, and then: *where the answer costs the user anything — a place to look, something to type, a thing that can be lost — that comes back to Specs*. Every promise there is still kept, so the trace never breaks; the user simply meets something nobody wrote down. Open Threads says the same for a job's folder and for the Keys questions.

Either way it is a hole under Rule 7. The fix goes **up** into the spec, never sideways into a workaround, and only the Creator rules on a spec change.

Everything that reaches them is put as a moment: what the user would get, and what they would get instead. Never as a technical choice, because they cannot judge one and should not be asked to.

**What follows a ruling is work, and it is the runner's.** The spec is changed, which is a change to an audited document outside a round, so that spec's decisions record gets its line saying no auditor has seen it. The row is rewritten to quote the new promise. Every other row quoting that promise or resting on it is reopened, and any architecture document already written against the old wording goes back for the same treatment. Clearing an unfinished-spot marker in a spec or a design document is the runner's too, and it needs the same line. None of it is left to be noticed later.

## Stopping to confirm is waived for the stage

[Rule 4](../Groundwork/Rules.md) carries the waiver and its limits: the checking half goes, the unhurried half stays. So agents carry on without asking and commit as they go, and nothing here is licence to rush.

**Three moments still reach the Creator, and everything else is waived.** They are the section above, and an agent stops the moment it hits any of them. Opening and closing an ordinary architecture round follow the readiness and bounded-close tests without asking; a change to Specs still stops as the section above requires. Nothing else is put to the Creator.

**The fourth used to be the batched request for a rented server, and the ruling of 2026-08-17 took it away.** Agents act with the Creator's keys and do not ask. What replaces the request is a **telling**: the board says what was rented, what it cost, and that it is gone. Two things still reach them from that quarter — money at a different scale, which the section above lists, and anything that would touch what is already theirs.

**And the runner says when a fresh chat is wanted**, which is the rule at the end of this document rather than a moment of the Creator's. It is a statement, not a request, and the work carries on either way.

## Who drives it

One role carries the stage: the **architecture runner**. It writes no architecture, audits nothing, runs no gate, and rules on nothing. It picks what comes next in the order set out above, hands out briefs, files what comes back, keeps the list and the board true, and commits.

Everything in this document is its memory, not the Creator's. It reads the board first in every chat and updates it last, so a closed chat costs nothing and the work can move between tools.

**And it says when a fresh chat is wanted, rather than leaving the Creator to guess.** Creator-stated, 2026-08-17. The runner is the only one who can see a chat filling up, and the Creator cannot: they are the one who would carry the cost of guessing wrong in either direction — a chat run past its useful length, or a perfectly good one abandoned and its context rebuilt for nothing. So the moment the runner judges that the work should continue somewhere new, **it says so plainly and says why**, and it does that before the chat is in trouble rather than at the point of failure. It also says what the fresh chat needs to be told, which is normally nothing at all — the board is written precisely so that the answer is *read the board*. This is not a request for permission and it is not a stop: the runner names it, the Creator decides, and the work carries on either way.

The same holds for any agent this stage starts that finds itself in the same position.

**Three rules about working beside other chats, and none of them is theoretical.** The Creator runs several at once, and this project has already had one agent commit another's work and a branch left eleven commits behind. This stage is the worst placed of any for it: it is the longest, it fans out more agents at once than any research round, and Rule 4's checking half is waived so nobody is asked first.

- **Only the runner writes anything shared** — `Coverage.md`, the board, a decisions record, a signatures file. An agent it starts writes its own file and nothing else, so a sweep agent hands its rows back rather than appending them.
- **Only the runner commits, and never while an agent it started is running.** It pushes at the end of every sitting.
- **A commit names its files**, which the [Conventions](../Groundwork/Conventions.md) require of everyone and which matters most here.

## The one thing the Creator can read

Everything this stage produces is written past them on purpose, and stopping to confirm is waived, so weeks can pass with nothing they can check. One line stands against that, and it is the only handle they have.

**`Coverage.md` opens with a count, kept current**: how many rows there are, how many are answered, how many are unresolved **and the promise still holds**, how many are unresolved **and the promise does not**, and how many are waiting on the Creator or on a machine. Five numbers, no prose. The two kinds of unresolved are split because they are not the same news at all — one is a detail nobody could settle, the other is a promise with nothing under it, and a single number hides the second inside the first. The runner reports that line at the end of every sitting, whether or not anything moved — a count that has not changed in three sittings says something no summary would.

## When the stage is finished

**Every row in the list has an answer**, or says plainly that it is unresolved and what breaks. That is the whole test, and it is the rows rather than the markers — the rows are the complete set, and a marker naming Architecture is only one of the three ways a row gets made.

Beside it: the sweep has been re-run so the list is current, both gates have been run over everything, the seams between architecture documents have been checked, and the scenarios chosen for this stage have been replayed.

Then the stage is done, and it is done by counting rather than by anybody's feel.

**Passing that test is not the same as being done, and two things sit outside what it measures.** The test counts rows, and a row's **summary** is a paraphrase rather than a quotation — so a summary that has fallen behind the clause it points at breaks no quote and is invisible to the traced gate's quote check, and Build reads rows. And the test stops at this stage's own boundary: the Blender **comparisons** and **joined answers** that this stage's answers, sources files and row files cite were repaired after the citation check that had ruled every downstream citation sound, so every sentence resting on one is unverified against the research as it now stands. Neither is a defect in the test, which is the stage's own and is met. Both, with everything else found after the five items closed, are gathered at `Audits/Architecture/What this stage still owes — Claude Opus 5.md`, which the board points at and which is later than the board wherever the two disagree.
