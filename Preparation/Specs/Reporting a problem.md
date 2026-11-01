# Reporting a problem

*What a user does when Smoozy has gone wrong and they want the Creator to know, and what Smoozy has been keeping that makes such a report worth reading. Not one word about how it's built.*

*2026-10-27 — Removed the withdrawn paid-access text and its obsolete references under the Creator’s free-and-open-source decision of 2026-10-04 (wave 5); retained protections and evidence keep their force. No auditor has seen this removal.*

## Why this is its own document

Every module in this project says what the user is told when something goes wrong. Not one of them says what the **Creator** learns. A bug can come out of any part of the app, so the answer belongs to no module; and it is too big for one line among the whole-app concerns, because three separate things have to be settled — what Smoozy keeps, what a report is made of, and when Smoozy offers to make one.

So they are settled here, once, and the [Whole app spec](Whole%20app.md) points at this document.

**This is a want, not a hole.** Smoozy's three promises are all about the user, and none of them breaks because the Creator cannot see what went wrong. It is here because the Creator asked for it on 2026-08-24 and settled its shape the same day. The investigation behind it — what the project already had, what it did not, and every option considered — is in [Diagnostics — what reaches the Creator when a user hits a bug](../Groundwork/Diagnostics%20—%20what%20reaches%20the%20Creator%20when%20a%20user%20hits%20a%20bug.md).

## The whole act, in one run through

An artist sends a job up on Tuesday. It renders, the frames come home, and on Saturday they plug in the drive and there is nothing on it. Every number on the history entry says all 240 files arrived. Nothing failed anywhere; there is no red dot to press.

They open the Smoozy menu and pick **Report a problem**. Smoozy asks for one sentence in their own words, and they type *the frames aren't on the drive but Smoozy says they arrived*. Smoozy writes a single file, offers a place to put it, opens the folder it landed in so the file is in front of them, and tells them to attach it to a mail to **hello@smoozy.app**. They send it from their own mail account.

**Smoozy sends nothing anywhere.** The user attaches the file and sends it themselves, from a place they opened and hold — so nothing of theirs leaves their computer except by their own act, and the promise that everything of the user's stays on their own computer is untouched.

**Smoozy points at the file and never at a mail program.** It does not try to hand the file to whatever the computer uses for mail, already attached. Label: **Creator-stated** (2026-08-25) — the Creator uses no mail program at all and reads mail in a provider's own pages in a browser, and is not unusual in that. A route built on a mail program being set up would do nothing for those users, and doing nothing is worse than pointing at a file, which works for everybody. **So this is the floor and also the ceiling**, and nobody should propose the other route again.

**What Smoozy is doing for them is the part that would otherwise be impossible.** The Creator's requirement, stated 2026-08-24: *"I don't want users to have to find a file somewhere on their hard drive by themselves and copy certain parts of it. It has to be easy for them."* So the user never hunts for a file, never decides which part of it matters, and is never asked to make the bug happen again in order to report it.

**The file is not put in front of them to read, and does not need to be.** It is plain text, it is in a folder they chose and Smoozy just opened, so anyone who wants to look already can. Smoozy does not make them look. Label: **Creator-stated** (2026-08-25) — *"they don't need it to be readable to them"*. The consent question that would otherwise ride on this is answered by the act itself: the user is the one who attaches and sends.

**The address is a mailbox and nothing more.** No ticket, no account, no registration, and no promise about how fast anyone answers. It exists so that a user holding a description of a bug has somewhere to send it, and so that the Creator can write back and ask one question — which nothing in Smoozy could otherwise do.

## What Smoozy keeps, so that there is anything to send

Smoozy keeps a running record of what it did — **the diary**. It is Smoozy's own, on the user's own computer, and the user is never asked about it, never has to switch it on, and does not meet it as a feature. It exists so that when a report is made there is something in it.

**One rolling record, capped at 20 MB, oldest lines dropped as it fills.** Label: **Creator-stated** (2026-08-24) — the size cap is the Creator's shape; the number is the runner's and is theirs to overrule. It is fixed rather than read off the disk, so no free-space reading decides anything.

**Why a size and not a job, a run or a count.** Retention tied to jobs loses everything that happened before a job existed — and the artist who crashes three times while getting a fresh machine render-ready has submitted nothing at all. A single rolling record has no such hole: there is nothing to key on and no case that falls outside it. 20 MB is negligible beside what this product already moves, where one scene file runs to gigabytes, and it holds weeks of ordinary use in plain text.

**What goes into it.**

**What Smoozy did, in order, with times.** Its own acts, the ones a user could name: connected, read a file, started a job, sent a batch, published a download. Not its internals — a record of those is unreadable to the Creator and unshowable to anyone.

**What Smoozy was told.** The endings and readings that came back from a machine, and that machine's own Blender version and Linux family, which nothing else in Smoozy holds. Smoozy's acts and the machine's answers together are the thing neither side has alone. **But never on one clock.** The user's computer clock is undefended, so every moment says which clock it came from, and the machine's own order comes from its journal's own counting rather than from time.

*Why the Blender version earns its place:* a render that ended having printed nothing is a known Blender behaviour on two of the four supported versions and a possible Smoozy bug on the other two (see [Whole app](Whole%20app.md), under *How a failure reaches the user*). Without the version the Creator cannot tell which, on a record built to let him find Smoozy's bugs.

**What failed** — the failure as Smoozy already put it to the user, in the same words.

**Smoozy's own version, and which operating system the user is on.** Smoozy's update check already tells its own hosting both of these on every start, so nothing new about the user is learned by writing them down.

**What Smoozy assumed and found wrong.** The valuable one. A dozen places in the plan expect a fault of this kind to be caught while the app is being built, and nothing anywhere answers for one turning up while the app is running. These are Smoozy's own bugs rather than the user's or the machine's, and today they are silent everywhere.

**What Smoozy managed to say on its way down, where it got the chance.** A failure record exists for a connection dropping or a write refused; by its nature none exists for the case where Smoozy itself died. Without this the Creator learns only that a run ended during an upload — not whether it was the transfer, the machine running out of memory, or the user force-quitting, which leave the identical absence of a mark. **Its limit is stated rather than hedged:** a process that is killed outright runs no code of its own, so nothing is written, and a force-quit and the machine running out of memory stay indistinguishable from each other and from any other death. What this catches is the narrower case of a fault Smoozy sees coming and survives long enough to write down. Label: **unresolved** — exactly what an operating system permits a dying process to write is an outside fact and needs a source before it becomes a promise.

**What it never holds.** Nothing from the secret store: no passphrase, no key of any kind, no fingerprint. And never the contents of the user’s files.

**What it does hold that is the user's**, and this is deliberate: file paths, folder names, scene names, machine names, usernames. That is what makes a report worth reading. The user is the one who sends it, so nothing goes anywhere on their behalf that they did not send.

## A report is a slice of the diary, not the diary

Label: **Creator-stated** (2026-08-24), and this is what keeps a mail attachment small.

- Reporting a **failed job** takes that job's own acts and the session around them.
- Reporting a **crash** takes the session that died.
- Reporting **nothing in particular** — the Saturday drive above — takes the current session and the one before it.

**Three things keep it small enough to attach:** it is a slice and not the whole record; it is plain text; and where a slice would still be too large, Smoozy drops from the middle, where the least is happening, and **says how much it dropped**. That is the behaviour Smoozy already has for a machine's output that would not fit.

**What must never happen is a quiet trim.** A report that silently left things out is one the Creator reads as complete. Whatever was dropped is said in the file itself, so the statement travels with it.

**And it says what it could not include.** *Do what can be done, and say what can't* holds here too. A report made with no server reachable holds none of the machine's words, and it says which of three things is true — absent, unreachable, or never collected. Collapsing those costs the Creator a round trip back through the user, which is the one thing a report is meant to avoid.

**Where a server is reachable, the report also takes what that machine holds about the job** — its journal's tail and its note on the job. **Blender's own crash file, which the machine writes where Blender died, is the obvious further thing to take**, and a backtrace is exactly what tells a Blender fault from a Smoozy one. Label: **unresolved** — whether such a file carries values out of the machine's own environment is not established, and capturing those into any record is forbidden elsewhere. Until that is answered the crash file is not taken.

## Where the user is offered this

Two places, and no third. Label: **Creator-stated** (2026-08-25).

**On any failure the user is looking at.** The report belongs beside the thing that went wrong, because that is where the user is and that is when they care. A failure has one shape everywhere it attaches, so this adds one thing to that shape rather than a route of its own. What it looks like there, and what it does to a failure that has collapsed onto a single line of a list, is Design's. Label: **settled at Design** (2026-08-25) — it is the second and last addition the failure shape permits, quiet and after the raw text; a failure collapsed onto one line does not carry it, by the rule that already keeps the second sentence and the raw text off a row.

**In the Smoozy menu, always.** Because the largest class of bug leaves no failure anywhere: the download that says it arrived, the render that came out wrong, the app that is simply slow. Nobody presses a red dot for those, and a route that only exists beside a failure would never catch one.

## After a crash, Smoozy speaks up

A user who has to remember, days later, to go and report something will not. So Smoozy raises it itself.

**Smoozy writes a mark at the moment the user asks to quit.** If the next launch finds no mark, the run before it ended badly, and Smoozy says so and offers to report it.

**This is a named exception to two things this project has settled**, granted by the Creator on 2026-08-24 — *"Smoozy may speak up, yes."* [Whole app](Whole%20app.md) says a failure appears on the thing that failed, that Smoozy does not interrupt with a box the user is not waiting on, and that there is no separate place failures are collected. A run that crashed yesterday is not a thing on any screen, so a launch-time prompt is refused by both halves. It is written down here as the exception it is rather than slipped in, and it reaches no further: the crash of the previous run, at the next launch, and nothing else. Label: **settled at Design** (2026-08-25) — a box with two controls, whose most important line says nothing was lost, because a crash and a clean close leave Smoozy in the same state and a prompt that did not say so would frighten a user about work that is safe.

**One thing this cannot reach, and it is said rather than hidden.** On Mac and Linux, a new version of Smoozy that will not start at all is rolled back by nothing — and an app that never runs writes nothing and offers nothing. That case stays exactly as unreachable as it was.

## Where this stops

**Smoozy never sends anything, to anyone, ever.** Not the diary, not a report, not a count of anything. There is no telemetry in this app and this document does not introduce any. Everything that leaves the user's computer leaves because the user attached it to their own mail and pressed send.

**The user is never asked to reproduce a bug** in order to report it, to find a file, or to judge which part of anything matters.

**An address to write to is not an account and not a registration.** [Whole app](Whole%20app.md) promises no email, no account and no registration, and that promise is about getting and using Smoozy — there is still nothing to sign up for, nothing held about the user anywhere, and no mailbox of theirs in Smoozy's hands. Label: **Creator-stated** (2026-08-24) — *"My promise about no email was about registering to use Smoozy, which the user doesn't need to do."*

**And the address is a standing commitment of the Creator's**, not a feature of the app: it has to be read and answered for as long as Smoozy is out there. That cost is the Creator's and is accepted. Label: **Creator-stated** (2026-08-25).
