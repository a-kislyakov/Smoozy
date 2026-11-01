# Download

*What the Download module does, in plain terms: how files come back from a server to the user's own computer — both the ones the user asks for and the render outputs that arrive on their own as a job produces them. Not one word about how it's built.*

## Where Download fits

Download owns everything that travels from a server back to the user's machine. Nothing else in Smoozy brings files back to the user's computer.

There are two ways in, and the difference between them is **whether the files exist yet**:

- **A manual download** copies files that are already sitting on the server. It is a finite set, asked for in one moment, and the user knows what they asked for.
- **Auto-download** waits for files that do not exist yet. It is switched on for one job, and brings that job's outputs home as they are written, for as long as the job keeps producing them.

They are not the same action with two triggers. One copies files that are already there; the other is a standing arrangement attached to a job. Everything below that differs between them differs for that reason.

**Connections.** The [module map](Whole%20app.md) owns how Download is wired to the rest of the app. That covers its dependence on a connection from **Servers** for everything it does. It covers the places a download is started from: **File Browser**, where the user picks files, and the queue and history in **Queue & History**. It covers what **Queue & History** tells it about a job, and that the queue is one of the places the user reaches the auto-download toggle. It covers that an auto-download arrangement is most often made at the moment a job is submitted, which is **Rendering**'s action. And it covers what it takes from **Render Settings**, whose reading of a `.blend` is where the count of a job's expected output files comes from.

**One thing it leans on rather than owns.** What a job actually writes — how many files, in which formats, into which folders — is settled in [What a job writes](What%20a%20job%20writes.md), because this module, **Render Settings**, **Queue & History** and **Rendering** all promise into it and none of them owns it. Everything below about counts, formats and the folder auto-download watches takes its answer from there.

## What can be downloaded

Anything on the server the user's login is allowed to read — files, folders, and any mix of the two, in any number. Render outputs are the common case, not the limit: the user has a file browser and can bring down whatever they can see. Label: **Creator-stated** (2026-07-22).

Where the server will not let that login read something, Smoozy brings down what it can and says plainly what it could not — see *do what can be done, and say what can't* in [Whole app](Whole%20app.md).

**A file Smoozy cannot treat as finished still comes down when the user asks for it, and Smoozy says so first.** A frame can be sitting on the machine part-written — a render still working on it, or a write that ran out of room — and what makes a file one Smoozy cannot treat as finished is [What a job writes](What%20a%20job%20writes.md)'s. Where the user picks such a file out by hand, Smoozy says it cannot vouch for that file and brings it anyway if they go ahead: the machine is theirs, refusing would be deciding for them, and saying nothing would hand them a broken frame to composite with. **The promise is that Smoozy says what it cannot vouch for, not that it can always tell** — neither a file's presence nor what the machine reported proves a frame finished, which that same document settles, so this reaches exactly as far as what Smoozy can actually judge. Auto-download is the other way round, and *Auto-download* below says why.

Downloading never changes anything on the machine. Smoozy copies; it does not move, and it does not tidy up after itself. What came down is also still up there until the user removes it by some other means. Label: **Creator-stated** (2026-07-22).

## Where files land

Smoozy puts downloaded files in the folder the operating system uses for downloads. The user can point Smoozy at a different folder instead — set once, in one place, and used for everything from then on. Label: **Creator-stated** (2026-07-22), following how web browsers behave.

That standing choice can be **overridden for a single download**: when the user asks for something, they can send that one thing somewhere else without changing the setting. Smoozy does not ask where to put each download; the user sends a single download elsewhere only when they choose to, and the standing setting is untouched.

**An auto-download arrangement takes its destination when it is switched on** — the standing folder, or somewhere chosen at that moment — and keeps it for the life of the job. Nothing can be asked of a user who is not there, so the choice is made once, in advance, like everything else about an auto-download arrangement.

A downloaded folder arrives as a folder — the structure the files had on the server is the structure they have when they land.

**A whole job's outputs come home inside the job's own folder, whichever way they were asked for — unless the user moves the folder part-way through, and where they do, Smoozy says so at the time and shows both places afterwards.** Every job's outputs are written into a folder of that job's own on the machine, which [Queue & History](Queue%20&%20History.md) owns, and that folder is recreated where the files land. So two jobs' outputs no more mix on the user's disk than they do on the server's, and a job re-run after an interruption comes home to the folder its first attempt used. It holds for an auto-download arrangement and for a whole-job download the user starts by hand from a queue or history entry alike — both bring the job's outputs, so both bring the folder.

**A whole-job download started by hand is an ordinary manual download**, so it meets the already-exists question below like any other. What it is landing on is that job's own folder, so it collides only with what an earlier download of the same job left there — never with another job's frames that happen to share a name.

## The unit is the file, the request is still one thing

Pulling down a folder of three hundred textures is **three hundred transfers**, not one. That is deliberate: the user can see exactly which file is moving, which one is slow, and which one failed, rather than watching a single bar that hides all of it. Label: **Creator-stated** (2026-07-22).

But the user asked for **one thing**, and that stays true. However many files a request turns into, the user can still see how far the request as a whole has got. Both are owed: the detail and the whole.

**A set number of transfers run at once per server, and the rest wait their turn.** The user does not set that number. What it should be depends on the server, the connection and the size of the files — none of which the user is in a position to judge better than the app — so it is Smoozy's to decide, and Architecture's to pick. Label: **Creator-stated** (2026-07-22), following how web browsers behave. Auto-download transfers take priority over manual download transfers in the server's transfer queue. Label: **Creator-stated** (2026-07-22).

## When the file is already there

This is Download's version of the whole-app promise that local files are never silently overwritten (see [Whole app](Whole%20app.md)); what follows is what that means for a download in particular.

Before a manual download starts, Smoozy checks what it is about to land on. If any of the files it is bringing down already exist where they would go, it stops and asks — **once for the whole request**, not once per file.

What it says includes **how many** already exist. "Forty of these three hundred are already here" is a different decision from "one of them is", and the user cannot judge it without the number.

The four answers are:

- **Replace** the existing copies with the arriving ones.
- **Keep both** — it applies a non-colliding name to the arriving folder, leaving internal file names intact. Nothing already on the user's disk is touched. Label: **Creator-stated** (2026-07-22).
- **Skip existing** — fetch only the files that are not already present at the destination.
- **Stop** the whole download.

Smoozy compares by **name and place only**. It does not look inside the files, and it does not compare their dates or sizes to guess which one the user wants. Guessing wrong overwrites work; asking plainly does not. Label: **Creator-stated** (2026-07-22), following how the macOS Finder behaves.

When a folder is part of the request, this is judged file by file, not folder against folder: a folder coming down into a same-named folder merges, and only the files that share a name and place inside actually collide. **Replace** touches just those; **Keep both** sets the whole arriving folder aside under a non-colliding name. The [File Browser spec](File%20Browser.md) sets this out in full, and it works the same in both directions.

## Stopping a download, and losing the app

The user can stop a download that is under way.

**A part-arrived file is not left behind looking finished.** Whatever happens to a transfer that does not complete — stopped by the user, cut off by a lost connection, or ended by the app closing — Smoozy does not leave a half-copied file sitting on the user's disk as though it were the real thing. Exactly what becomes of it is Architecture's; that the user is never handed a broken file that looks complete is settled here. A file that arrives whole but damaged is the same harm reached another way, and the whole-app promise that a file is either correct or plainly not there covers both — see [Whole app](Whole%20app.md).

**One thing does appear briefly at the finished name, and it is empty.** As a file lands, an empty file carrying its exact name appears in the destination folder for a moment, and the arriving file replaces it. That is how Smoozy claims the name — asking the computer whether the name is free rather than working it out, which is also what stops anything else taking the name in the instant before the file lands. **Where Smoozy is stopped at that exact moment, the empty file is left behind**, and Smoozy removes it the next time it runs and not before. So a user who looks in that folder in between finds a file with the right name holding nothing. Label: **Creator-stated** (2026-08-18). It is not a part-arrived file and the promise above is unaffected: nothing half-copied is ever there, and nothing there is ever mistaken for the real thing by Smoozy itself.

**Closing Smoozy stops transfers.** This is unlike the render queue, which lives on the server and carries on with nobody watching (see [Queue & History](Queue%20&%20History.md)). A transfer runs between the server and the user's computer, so it needs Smoozy to be there.

**A manual download interrupted by a lost connection carries on where Servers repairs the connection.** The transfers the drop interrupted pick up where they stopped, without the user asking again. Label: **Creator-stated** (2026-08-12). It lasts only as long as the app is open, and only where the file on the server has not changed.

**The reason that used to stand here is set aside, and is written down rather than deleted.** It was that the user asked for something at a moment, and that moment has passed — so Smoozy does not renew a request on their behalf. The argument that beat it is the same one that settled the matter for an upload in the [File Browser spec](File%20Browser.md) on the same day: Smoozy is holding a live connection it repaired on its own and not using it. The two are visible in the same transfer window, so they now give the same answer for the same event rather than two different ones.

**Closing Smoozy is different, and there a manual download does not restart on its own.** Reopening Smoozy does not quietly resume it; the user asks again if they still want it. Auto-download behaves differently even then, and the next section says why. The line between the two is whether Smoozy is still there: a connection it repairs is a connection it already had, where an app being closed and opened is the user starting again.

## Auto-download

Auto-download is **opt-in** and belongs to one job. It is off unless the user turns it on. When it is on, that job's outputs come back to the user's computer as they are written, rather than waiting for the job to be over.

**It is normally switched on before the job is submitted**, at the moment the user sets the job going — which is **Rendering**'s action. That is the ordinary way in, and the toggle is Download's wherever it is reached from.

Switching it on or off **afterwards**, for a job waiting its turn or one already rendering, is settled in [Queue & History](Queue%20&%20History.md), which also owns the queue as one of the places the user reaches the toggle.

**Switching it on mid-render brings back what has already been written.** A job that is forty frames in when the user turns auto-download on does not start from frame forty-one. The frames already sitting on the server come down too. Anything else hands the user an incomplete set of frames. Label: **Creator-stated** (2026-07-22).

**Switching it off ends the arrangement, but does not reverse it.** Nothing new is fetched from that moment on. The transfers already under way are allowed to finish rather than being cut off part-way: only a set number run at once, so that is a few files, not many, and cutting them off would throw away work already half done. A user who wants one gone immediately can stop it by hand, like any other transfer. The toggle ends the arrangement; stopping a transfer ends that transfer. They are two controls doing two jobs, not competing versions of one.

**What has already come home stays.** Smoozy never takes files back off the user's computer, whatever becomes of the arrangement that brought them. That, and the transfers under way being let finish above, are both Label: **Creator-stated** (2026-07-22).

**Switching it on again catches up.** Everything written while the arrangement was off comes back, by the same rule as switching it on mid-render. Changing their mind about the narrowing costs the user nothing. Switching auto-download off and on resets both the destination and the narrowing — the user chooses both again when switching it back on. Label: **Creator-stated** (2026-07-22). **Changing the destination part-way through does cost something, and Smoozy says so at that moment**: what has already come home stays where it went, only what is still to come lands in the new folder, and the job shows both places afterwards. Label: **Creator-stated** (2026-08-20).

**It watches the job's output folder.** Every job's outputs are written into a folder of that job's own, which is what gives auto-download one place to watch and one place to pull from. That folder, the identity it is named for, and the optional label the user can give it are all properties of the job: they belong to [Queue & History](Queue%20&%20History.md), which owns what a job is and states what that folder is. Download reads the folder it is pointed at and changes nothing about how it is arranged.

**That one place holds even when the `.blend` asks for others.** A file can be set up to write anywhere on the machine, in any number of places, and none of it escapes the job's folder — Smoozy redirects all of it there, keeping the arrangement the artist built. See [What a job writes](What%20a%20job%20writes.md). Without that, auto-download would have no single place to watch and would silently miss files.

**A file appearing in that folder is not yet a file to bring home.** A frame's file can be caught part-written, and an empty file can be sitting at the finished frame's place from the moment that frame's work begins — both settled in [What a job writes](What%20a%20job%20writes.md), which hands the problem here. So the arrangement leaves a new file where it is until it can be treated as finished, and fetches it then. How Smoozy tells a finished file from one still being written is Architecture's.

**That reaches as far as Smoozy can judge, and no further — the same promise the manual half makes.** A write that runs out of room leaves the frame at its own final name, gets Blender's ordinary saved line, and opens reporting the full picture size, so nothing about the file tells it apart from a finished one. Label: **observed** (all four versions, both machines) — see [Sources](Sources/Download.sources.md). So an arrangement can bring such a file down believing it finished, and **Smoozy does not claim it can always tell**. What is promised is that Smoozy says what it cannot vouch for, which is exactly what *What can be downloaded* above promises for a file the user picks by hand.

**Where it says so is the job's entry, not the moment.** Nobody is there to be asked or told while an arrangement runs, so the saying happens where the job is read afterwards: the count coming up short, and the line naming the path in what the machine printed, which [Queue & History](Queue%20&%20History.md) shows. Both are described below. This is the one place the whole-app promise that a file is whole and correct or plainly not there (see [Whole app](Whole%20app.md)) reaches only as far as Smoozy can judge, and it is stated here rather than left to be discovered. **Auto-download is not a check run on the user's behalf and is not written as one** — it fetches what looks finished, and says what it could not vouch for.

**It does not have to bring back everything.** One render can produce several files for the same frame, in different formats and in several folders at once — see [What a job writes](What%20a%20job%20writes.md). An artist who only wants a quick look should not have to drag heavy files across an entire animation to get it. So the user can narrow an auto-download arrangement to the kinds of output they actually want. Label: **Creator-stated** (2026-07-22).

Three things go with that:

- **Everything is the default.** A user who never thinks about this gets all of their work. Narrowing is something the user does on purpose; it is never something Smoozy decides for them.
- **The choice is made when the arrangement is made**, alongside the destination and for the same reason — there is nobody to ask later.
- **Narrowing costs nothing permanent.** Whatever the arrangement passed over is still sitting on the server, and can be pulled down by hand from history at any time, for as long as the server exists. So narrowing is a safe decision — the user is choosing what comes home now, not what they are allowed to keep.

**The choice is by file format, and goes no finer.** The user picks which formats come back — not passes, not view layers, not any other cut of what a job writes. Format is what separates a file worth glancing at from a file worth compositing, and that is the whole of what this choice is for. Label: **Creator-stated** (2026-07-22).

**Format is the cut because it is the one the artist already holds.** A user knows what their own file writes and in what formats; a job's **main output** is Smoozy's reckoning about their file (see [What a job writes](What%20a%20job%20writes.md)), and offering that as the cut instead would ask them to trust a judgment rather than state a fact. Label: **Creator-stated** (2026-07-28).

The formats offered are the ones **this job will actually produce**, read from the job the same way its file count is. The user is never shown a format that is not coming, and a job producing only one format has nothing to narrow.

**That list is everything the job writes, not the scene's own format.** A file's compositor can write formats the scene never mentions, so a job whose scene writes PNG while its nodes write EXR offers both. What the list is made of is settled in [What a job writes](What%20a%20job%20writes.md).

**Where the file has not been read, there is no list, and the arrangement covers everything.** A job can be submitted without the user ever running Pre-load Settings, and the formats come from reading the file — so an arrangement made at that moment has nothing to narrow down to, and takes the default of everything the job writes. Nothing is lost: the file is read when the job starts, and the arrangement can be narrowed after that from the queue, where its toggle already lives. No pre-load is forced before submission for the sake of this. See the [Render Settings spec](Render%20Settings.md), which owns the reading.

What a job produces is the product of **two** things: the formats the file asks for, and what the Blender version rendering it is capable of delivering. Neither alone decides it. A server can hold more than one version (see the [Server Setup spec](Server%20Setup.md)), so the same file is not guaranteed to produce the same set of files on every one of them — which is why this cannot be a fixed list Smoozy carries around.

**The list does differ between versions, and how it differs is known.** Blender 4.5, 5.0, 5.1 and 5.2 all offer the same two families — still pictures written straight out, and video written through FFmpeg — and three differences are real. Blender 5.1 and 5.2 can write AVIF; Blender 4.5 and 5.0 cannot, and refuse it when asked. From Blender 5.0 the multi-layer EXR is picked through a control of its own, which has to be set before the format itself, where Blender 4.5 has the format list alone. And what a written file records about its own colour space was added version by version, which changes nothing about which formats can be written. Label: **documented and observed** — see [Sources](Sources/Download.sources.md).

**What happens when a file asks for a format the version rendering it cannot deliver is still open**, and not for want of looking: every copy of Blender the project asked has every optional format built into it, on both the machines it could test, so the case was never produced. One Blender that came with a rented machine was never asked anything, and that machine is gone. Label: **permanently unresolved** — see [Sources](Sources/Download.sources.md).

**It will not now be answered.** Settling it needs a copy of Blender built without one of the optional formats, which is a controlled Blender test, and the Creator ruled on 2026-08-31 that there will be no further Blender research and no further rented machine. *(This paragraph used to point the reader at [Open Threads](../Groundwork/Open%20Threads.md) for the route. That document has no entry on the subject and never had one, so the route is written out here instead.)* What holds without the answer is settled at Architecture — see the [Download architecture](../Architecture/Download.md), its section on Row 214, where the promise this module makes stands unchanged: entries the version cannot deliver never arrive, and the set of files ends short and says so rather than waiting for ever. **What breaks:** a file asking for a format its version cannot deliver produces something other than what the user asked for, or nothing, and this module does not know which.

**An arrangement lands in the job's folder like any other whole-job download** — see *Where files land* above, which owns that rule. When a re-run's outputs arrive at that same folder, auto-download follows the same collision rule as ongoing downloads: it fetches only what it has not already brought, and where it finds a file it did not put there (like those from the first attempt), it leaves it alone and says so.

**Auto-download does not ask the already-exists question, because it knows what it has already brought.** It keeps track of what has come home for its job and fetches only what has not. That is what lets it pick up after a lost connection, catch up when switched on part-way through, and carry on after being switched off and on again: each time it is answering *what is here that I have not taken yet*, never *shall I replace this*. The already-exists question in the section above is a manual-download question.

**Where it finds a file it did not put there, it leaves it alone and says so.** A file can be sitting at the destination without this arrangement having brought it — for example, because the user pulled some of that job's outputs down by hand first. Auto-download does not write over it, and does not stop either: it passes over that one file, carries on with the rest, and tells the user which files it left behind. It runs with nobody watching, and quietly overwriting something the user put there by hand is the one outcome that would destroy work unseen. This is *do what can be done, and say what can't* (see [Whole app](Whole%20app.md)). Label: **Creator-stated** (2026-07-22). (Note: This means auto-download warns about early frames the user manually downloaded from the same job. Whether auto-download should recognise files from the same job to avoid this warning is a product decision.)

**The user can see how much of the job has come home.** How many output files a job will produce is knowable when the job is submitted (or once rendering starts, if the user skipped pre-loading the file), so auto-download can say how far along it is against the whole, rather than only which file is moving. Where the user has narrowed the arrangement, the whole is **what they asked for**, not everything the job writes — otherwise a deliberately narrowed download would report itself as permanently unfinished. That count comes from reading the `.blend`, which is **Render Settings**' routine, and it is a property of the *job* rather than of the file — the frame range, the view layers and the rest are what decide it, and they are fixed at submission along with the other settings. That the count is knowable at all is a Blender fact, labelled **Creator-stated** (2026-07-22) — see [Sources](Sources/Download.sources.md).

**The count is not one file per frame.** A job of a hundred frames can come to several hundred files, or to fewer than a hundred where some of its frames produce nothing. What the reckoning is made of is settled in [What a job writes](What%20a%20job%20writes.md), including the two cases that bend it — a job whose output is one video file, which counts as one file with the number stated as possibly growing, and a file built on Blender 4.5 in a way that lets its outputs change from frame to frame.

**That count is now doing more work than it was written for, and this module should know it.** A file output node's write that runs out of room does not fail the job: the render carries on, reaches its last frame and ends at a **successful** status, with the lost file's own saved line printed as though it had been written in full. Label: **observed** (the same in all four versions, on both machines) — see [Sources](Sources/Download.sources.md). So none of the failure machinery fires and the job reads as finished. **The count against what has arrived does not reliably catch it either**, and that is worth stating rather than assuming: a write that ran out of room leaves a file at the right name, and [What a job writes](What%20a%20job%20writes.md) says plainly that Smoozy cannot tell such a file from a finished one. Where the write was refused before it started, nothing at all is left and the count does come up short — except on the rented Linux machine, which is the kind Smoozy renders on, where a volume with no room left leaves a file of zero bytes at the right name, so the count does not catch that one either. That fact is [What a job writes](What%20a%20job%20writes.md)'s. **The one signal that is always there is a line naming the path in what the machine printed**, which [Queue & History](Queue%20&%20History.md) already shows the user. So where an auto-download arrangement finishes short of its count, that is worth saying plainly rather than rounding up to done — and it is a number that happens to catch some of these rather than a check Smoozy runs on the user's behalf.

## When auto-download meets trouble

**A lost connection stops it, and loses nothing.** Smoozy cannot read a server it is not connected to, so auto-download stops when the connection does. The outputs are unharmed — they are on the server, and the render carries on there whether Smoozy is watching or not. The next time the user connects to that server, auto-download picks up everything that landed while they were away. Label: **Creator-stated** (2026-07-22).

**A repaired connection is no longer the place the two kinds part.** A manual download interrupted by the same drop also carries on where Servers repairs the connection — see *Stopping a download, and losing the app* above. What still separates them is the app being closed, below.

That sits inside the explicit-connect model the [Servers spec](Servers.md) sets out rather than being an exception to it. Nothing reaches out on its own: the arrangement is work the user already asked for, and the connection it resumes on is one the user opened.

**Closing Smoozy is the same story.** Transfers stop, and auto-download resumes on the next connection — because it is an arrangement attached to a job, not a request made at a moment. (The arrangement itself, including its destination and narrowing choices, is remembered locally on the user's computer. This does not conflict with [Queue & History](Queue%20&%20History.md)'s rule that Smoozy keeps no lasting copy of a machine's state: a download arrangement is a local intention, not a cached copy of the server's queue.) That is the one place manual and automatic downloading behave differently for the same event, and it follows from what each one is.

**A job that ends badly still gives up what it made.** When a job fails, is stopped by the user, or is abandoned after being interrupted twice, whatever it managed to write still comes down. *Words changed 2026-10-28 (was *is given up*): Creator-stated, the final words sweep. No auditor has seen it.* Sixty frames of a hundred are sixty frames the user paid a rented machine to produce, and there is no reason to leave them behind. The endings themselves are [Queue & History](Queue%20&%20History.md)'s; what Download does about each of them is the same. Label: **Creator-stated** (2026-07-22).

**Where the server itself went down, what it made may be less than what it reported making.** A machine reset while a frame was being written was found to leave **no frame's contents at all** — including a frame Blender had reported saved seconds earlier. Label: **observed**, on one rented Linux machine; macOS untested — see [Sources](Sources/Download.sources.md). So this promise is that whatever is on the server comes down, not that everything the job reported comes down. **A file of zero bytes is not a frame and is not brought down**, which is [What a job writes](What%20a%20job%20writes.md)'s rule and is where the emptied frames land. What that leaves is a job whose outputs arrive short of what its entry says it rendered, and the reading below already says *ended* rather than complete.

**And the user is told when those outputs are home**, the same as for a job that finished. What a badly-ended job managed to write is the part most worth telling them about, and leaving it to be discovered would make the user go looking for files they were never told had arrived.

**The reading of how much has come home says *ended*, not complete.** That reading is measured against everything the job was going to produce, and a job that ends badly never produces it. So the arrangement brings down what the job actually wrote and then reads as ended — rather than sitting part-way, which says more is still coming, or reading as complete, which is untrue. Bringing back everything a job produced and bringing back everything it was going to produce are two different things, and only the first one is promised here.

**A cancelled job takes its arrangement with it.** A job cancelled while it was still waiting never renders and never writes anything, and it leaves no trace anywhere in Smoozy (see [Queue & History](Queue%20&%20History.md)). There is nothing for the arrangement to bring back, and nothing left of the arrangement either.

**A job being over does not mean auto-download is over.** The render finishing and the files being home are two different moments, and the second one comes later. Auto-download is done when the folder has arrived, not when the job left the queue.

**When it cannot write to its destination.** How the trouble reaches the user is settled by the whole-app concern *how a failure reaches the user* (see [Whole app](Whole%20app.md)): a full or unwritable folder is a cause Smoozy genuinely knows, so it says what to do about it, and it says it once rather than once per file.

**The arrangement keeps trying, and never switches itself off.** A full disk, a folder made read-only, a destination folder that has been deleted, a drive that is not plugged in — every one of them is something the user can put right, and the moment they do, the arrangement carries on and brings down everything written while it could not, by the same rule as switching it on again. Nothing is lost in the meantime: the files are on the machine, and the machine is not asked to hold anything for this. Switching itself off would avoid no harm and cost the user the one thing they came back for — they would find nothing downloaded and the arrangement quietly dead, needing to be set up again from memory. Label: **Creator-stated** (2026-07-30).

It says the trouble once, not once per attempt. Where the destination starts working and later fails again, that is a fresh piece of trouble and is said again. How often it tries is Architecture's; that it does not give up is settled here. The only thing that ends an arrangement early is the server being removed, below.

**A removed server ends the arrangement.** If the server the arrangement belongs to is removed from Smoozy (see [Servers](Servers.md)), there is nothing left to connect to. The arrangement ends permanently.

## What Download does not do

- It does not send anything **up** to a server. User-driven uploading is **File Browser**'s; sending a `.blend`'s missing assets is **Dependency Upload & Remap**'s.
- It does not browse a server's files or let the user pick them — that is **File Browser**.
- It does not connect to a server. Everything here needs a connection from **Servers**, and it never reaches a server the user has not connected.
- It does not decide what a job renders, where a job writes its outputs, or what a job is called. Those are properties of the job.
- It does not find out on its own that a job exists, or that it has started or ended. **Queue & History** tells it.
- It does not delete, move, or tidy anything on the server, before or after bringing files down.
- It does not retry a render, reorder the queue, or change anything about a job.

## Design-level detail left for Design

Each of these was handed to Design. **Read against the Design documents on 2026-08-25**: five are answered and now say where, and two were never designed at all and keep their markers. This is not an exhaustive list — the body hands Design further appearance and wording calls where they arise.

- **How the transfer window lays out uploads and downloads, and shows each transfer and each whole request.** Answered in [Transfers](../Design/Transfers.md), which is the whole of that screen. That uploads and downloads are watched in one place is settled in the [File Browser spec](File%20Browser.md).
- **How an auto-download arrangement is shown as a different kind of thing from a manual download.** Answered in [Transfers](../Design/Transfers.md), under *An auto-download is not a request, and reads differently* — its own four tags, turned off rather than stopped, and what it says about where the files land.
- **How the already-exists question is worded and presented for a download, including how the count is shown.** **Found undrawn on 2026-08-25 and drawn the same day.** [Transfers](../Design/Transfers.md), under *A download meeting files already there*: the same box as an upload's with one sentence changed, since the question and its four answers are the same and a second box would be a second copy of one decision. The live-job warning and the interrupted-upload line are absent rather than greyed, so nothing is put forward and the four answers sit level.
- **How a request made of many files shows both its individual transfers and its progress as a whole.** Answered in [Transfers](../Design/Transfers.md) — a request with its files under it, its own progress, and what a finished one collapses to.
- **How a download is started from history, and from the file browser.** Answered: *Download all* in the header of a queue or history entry, settled in [Queue & History](../Design/Queue%20&%20History.md), and *Download* on the toolbar and in all three right-click menus, settled in [File Browser](../Design/File%20Browser.md).
- **How a single download is sent somewhere other than the standing folder, without disturbing the setting.** **Found undrawn on 2026-08-25 and drawn the same day.** [Transfers](../Design/Transfers.md), under *Sending one download somewhere other than the standing folder*: a second control beside the first, never a question asked of every download — *Download to…* in each of the file browser's three right-click menus, and *Download all to…* in a queue or history entry's header. The picker offers nothing that makes the choice stick, because the standing setting is untouched.
- **How the narrowing choice is offered when auto-download is switched on, and how a narrowed arrangement reads afterwards.** Answered: the panel that narrows a download, drawn over the render settings page, and [Transfers](../Design/Transfers.md), where a narrowed arrangement says what it is narrowed to and reads against what the user asked for rather than against everything the job writes.
