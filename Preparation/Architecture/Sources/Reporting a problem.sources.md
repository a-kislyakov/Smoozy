# Sources — Reporting a problem

*Where every outside-world claim in [Reporting a problem](../Reporting%20a%20problem.md) came from. The document carries each claim's label in place; this file carries the citation behind it — the pointer to the input document that owns it, the pointer to the companion file that already holds it, the publication's exact words with its address and the date read, or the plain record that no source exists. **Decisions Smoozy made carry no entry here.** Entries run in the document's own order where the order means anything.*

Rule 1's plain-English rule is suspended here, per the Architecture stage's exception.

**Why this file was written on 2026-09-02, five weeks after the document.** [Reporting a problem](../Reporting%20a%20problem.md) was the one answer document in this stage with no companion sources file, and its own first signature says why: *"no outside publication cited directly, so no sources file"*. That was true of the writing and false of the document — every operating-system claim in it is an outside fact taken from an input document, and the input document does not carry an address for most of them. [Download's companion](Download.sources.md) had already reported the gap from the other side at its Fact 132 entry — *"`Reporting a problem` has no companion sources file at the time of writing, which is that document's to answer"* — and a checker verifying the Windows half of [V7](../Reporting%20a%20problem.md#7-writing-the-file-and-putting-it-in-front-of-the-user) had nowhere it was permitted to go. **Nothing in the document was changed to write this file.**

**Shapes used.** Section 1 is **pointers** into this stage's frozen input documents, which own the citation. Section 2 is **pointers into another document's companion file**, where the fact already has a home and copying it would give it two. Section 3 is **publications read for this file on 2026-09-02**, each with its exact words, its address and that date. Section 4 is **no source in this project**, recorded because none exists.

---

## What no entry below repeats

Four limits apply to everything in this file and are not restated under each entry.

- **No Windows machine has run any of this, and none was rented for it.** Every Windows claim in the document is documentation alone. This holds even where Section 3 supplies a first-party address: reading a page is not running a system. **PERMANENTLY SO, recorded 2026-09-02**: the Creator ruled on **2026-08-31** that no further machine is rented to answer a question, and the four Windows machines this project did rent are destroyed. This limit is not waiting on anything and must never be read as pending.
- **No Linux desktop computer exists on this project.** The Linux machines it rents are headless servers, not a user's own computer. Every Linux claim about the desktop side is documentation alone. **PERMANENTLY SO, recorded 2026-09-02, on the same ruling.** No Linux desktop has ever existed here and none is coming. **Guard for both limits — nothing below may ever be re-labelled *observed* on the strength of an address**, and where a claim is repeated elsewhere it keeps the label it was given here.
- **Every observation the document rests on was made on the Creator's own Mac**, and the input document's own limit is the document's: [Section 12](../Reporting%20a%20problem.md#12-unresolved) item 2 carries it in the body and no clause below is written as though a Windows or Linux desktop reading had been seen.
- **Machine-side facts are not desktop facts.** Where an entry below is a reading of a rented Linux server — the crash file, the rental's own settings — it says so, and nothing carries it across to the user's computer.

## The machine every reading made for this file was made on

**The Creator's own Mac, macOS 26.6.2, build 25G83, arm64** — confirmed at the machine on 2026-09-02 with `sw_vers` and `uname -m`. One reading below is of a manual page shipped on that machine and was made with `man`; the rest were made at the publications' own addresses. **Nothing was written outside the session's scratchpad area, nothing was installed, nothing was rented, and nothing pre-existing was touched.**

**The input document this file points to most** is [The three desktop systems read again — dying, revealing, resolving, and what the shell does not expose](../The%20three%20desktop%20systems%20read%20again%20—%20dying,%20revealing,%20resolving,%20and%20what%20the%20shell%20does%20not%20expose.md), the answer of record for Facts 129, 130 and 132, read 2026-08-25. **Its own reading rule binds every pointer below**: where a findings file and its citation check disagreed, the check won, and only the reversed form is carried. **Do not open the trail behind it** (`Audits/Architecture/The reading batch of 2026-08-25/`); that document says so itself.

---

# Section 1 — pointers into this stage's frozen input documents

## Fact 129 — what a dying process may run and write ([3](../Reporting%20a%20problem.md#3-what-goes-into-the-diary) J8, J9; [9](../Reporting%20a%20problem.md#9-after-a-crash) C6)

**Owner:** the input document **§1**. Label as it labels it: **observed** on the Mac, **documentation alone** on Windows and Linux.

The four claims J8 and J9 lean on, in the owner's words. **The endings that let a process run code:** on macOS *"Any signal you install a handler for"* plus the ordinary Quit path, *"not a signal at all but an Apple Event landing in `applicationShouldTerminate:` then `applicationWillTerminate:`"*; on Windows the console handler receiving `CTRL_C_EVENT`, `CTRL_BREAK_EVENT`, `CTRL_CLOSE_EVENT`, and the windowed messages `WM_QUERYENDSESSION`, `WM_ENDSESSION`, `WM_CLOSE`; on Linux *"Any signal you install a handler for"*. **The endings that give nothing:** `SIGKILL` and `SIGSTOP` refused on macOS and Linux, `TerminateProcess` on Windows, and macOS **sudden termination**, in Apple's own words, a process *"may be killed by the operating system without any notification or event being sent to the process first"*. **The buffering result, which is the most load-bearing thing in the fact:** a handler writing with a buffered stream left **0 bytes — the line vanished** — while a handler doing `write(2)` then `F_FULLFSYNC` left the line, *"and the exit status proves the handler fired in both cases"*; run, and **reproduced by the citation check with its own program**. **Out-of-memory on Linux:** *"the killer uses `SIGKILL`, so nothing runs, ever."*

**Used by** J8 whole, J9's three run-nothing endings and its force-quit width, and C6's *may hold no line from the moment of death*.

**Not covered, and two of these narrow the document rather than the fact.** The macOS force-quit ground is a **proxy at the width exercised** — `NSRunningApplication.forceTerminate()` against a **responsive** application; *"The Option-Command-Escape panel and Activity Monitor's Force Quit were never driven"* and *"a wedged application may not get the same treatment"* ([Section 12](../Reporting%20a%20problem.md#12-unresolved) item 3 carries it). **The Windows console half is narrower than J8 reads**: the owner's own correction records that the logoff and shutdown console events are received *"only by services"*, because *"Interactive applications are terminated at logoff, so they are not present when the system sends this signal"* — so the Windows endings J8 can actually rely on are the three `CTRL_` events and the three window messages, not the logoff pair. And **whether the forced-down bytes survive a real power loss is untestable without pulling power**; Apple's own page warns some drives ignore the request. Section 4's second entry carries that. **One further claim the owner asserts is not the owner's to assert and is not carried here** — that a force-quit's signal carries no origin. It has no publication behind it anywhere in this project, and the two the last entry of Section 3 records say the opposite; J9 was corrected on 2026-09-02 and no longer rests on it.

## Fact 130 — whether the next launch can tell the last run died ([2](../Reporting%20a%20problem.md#2-the-run-identity-and-the-run-mark) U6)

**Owner:** the input document **§2**. Label: **documented** on all three, with the macOS half **observed**.

The sentence U6 quotes is the owner's own verdict: every system-supplied route *"either misses the endings that matter most, or is off by default, or needs privileges a plain downloaded binary does not have"*, and *"the mark the application writes at startup and clears at a clean exit is not a fallback here; it is the only mechanism that covers every ending."*

The three routes U6 names as read about and deliberately not built on, as the owner carries them. **MetricKit** — `MXAppExitMetric` and `MXCrashDiagnostic`, readable unelevated at launch, overturning the word *nothing* but not the advice, because it gives cumulative counts over a delivery window rather than a verdict about one previous run, with Apple's own caveat *"Metrics are not guaranteed to be delivered, but can be expected atleast once per day when conditions permit."* and a forced quit landing under *"exited normally, or was gracefully terminated by the system"*. **The Windows recovery callback** — fires only *"If the application encounters an unhandled exception or becomes unresponsive"*, and **none of the three Windows routes fires on `TerminateProcess`**. **The Linux kernel log** — the out-of-memory kill is in it, but *"reading it is not a right an unelevated app has everywhere"*.

**Used by** U6's second half, and by [2](../Reporting%20a%20problem.md#2-the-run-identity-and-the-run-mark) U2's reason for keeping Smoozy's own mark at all.

**Not covered, in the owner's own closing line:** whether any Windows event-log entry names a process killed by `TerminateProcess`, and whether an unelevated application can read it — *"a Windows computer and a run"*. **And the macOS reversal must not be met in its old form**: the findings file said *"nothing does"*, and that is false; U6's *only mechanism that covers every ending* is the surviving claim and is not the same claim.

## Fact 132 — showing a folder, or a named file inside its folder ([7](../Reporting%20a%20problem.md#7-writing-the-file-and-putting-it-in-front-of-the-user) V2, V7)

**Owner:** the input document **§3**. Label: **documented** on all three, **observed** on the Mac, **documentation alone** on Windows and Linux.

The claims V7 leans on, as the owner carries them. **The act exists on all three and all three report failure to the caller** — macOS `selectFile:inFileViewerRootedAtPath:`, *"returns a status"*, false for a missing file, a missing root directory and a missing folder, observed on this Mac and reproduced by the check with an independently written probe; Windows `SHOpenFolderAndSelectItems`, *"with a count of zero to mean 'open the parent folder and select that item'"*; Linux `org.freedesktop.portal.OpenURI.OpenDirectory`, with its own published fallback. **The macOS variant to avoid** — `activateFileViewerSelectingURLs:`, *"Returns nothing. Not a run result — the header declares it to return nothing, so there is no outcome that could have counted as failure."* **What macOS requires of the application** — *"No bundle, no property list, no entitlements, and no Developer ID signature."* **What Linux must not assume** — `xdg-open` *"ships in a separate project and must be declared as a dependency"*, and *"no file manager can be assumed"*. **And the shell reaches none of it** — its browser-opening call *"refuses the `file` scheme by name"*, in a validator *"byte-identical across every current release checked"*, and *"the call returns nothing at all"*.

**V2's own half is the owner's *Save-a-new-file* section**, read and not built on: the save-a-new-file route is *"Published on all three, and the shell exposes it"*, with the portal's `current_name` option — *"Suggested name of the file."* **The caveat V2 does not need but a builder might:** the portal's **folder** suggestion carries *"The portal implementation is free to ignore this option."*, while the **name** carries no such caveat.

**The Linux reversal is stated as a reversal and must not be met in its old form.** The findings file said there is no published route and that the older bus interface is the only one; both are false, and only the portal form is carried.

**The Windows requirement is the one thing in this fact the owner quotes without an address, and it is read directly in Section 3 below.** The owner's words are *"`SHOpenFolderAndSelectItems` **requires** the component system to be initialised — 'Not doing so causes … to fail'"*; V7 turns that into *"which its page requires rather than suggests"*. The page, its address and its exact sentence are the first entry of Section 3.

**Not covered, in the owner's own closing line:** *"whether the reveal actually brought a window forward with the right row highlighted, which was never checked"*; which file managers besides GNOME's implement the older interface; what a macOS sandbox would change; and *"everything on Windows and Linux, which was read and never run"*. [Section 12](../Reporting%20a%20problem.md#12-unresolved) item 2 carries the last of these.

## Fact 131 — Blender's crash file and the machine's environment ([6](../Reporting%20a%20problem.md#6-what-a-reachable-machine-adds) G5)

**Owner:** [The Pile 3 batch — a full mount under a running supervisor, an interrupted heavy frame, and an hour of silence](../The%20Pile%203%20batch%20—%20a%20full%20mount%20under%20a%20running%20supervisor,%20an%20interrupted%20heavy%20frame,%20and%20an%20hour%20of%20silence.md) **§5**, registered at Fact 131 in [Facts](../Facts.md). Label: **observed in a test**, 2026-08-25, on Blender 4.5.3 on a rented Ubuntu 22.04 container.

The words G5 rests on: two unmistakable marker values were planted in the environment — one shaped like a licence key, one like a private path — and Blender was crashed twice, once with no scene and once with a real scene open. **Neither marker appears anywhere in either file, and neither file contains assignment-shaped text of any kind.** Both files are 1775 bytes and hold three sections and nothing else: a version header; a `# backtrace` of module paths and hex addresses, every path either the Blender installation's own or a system library's; and a `# Python backtrace` of source paths and line numbers. **With `heavy.blend` open Blender wrote `heavy.crash.txt`** — the user's scene name in the file's *name* and never in its contents.

**A negative result would have been** either marker appearing in either file, which would have kept `Security and licence` §13 rule 2's bar in force over the file and left Row 463 unresolved with the crash file untaken, exactly as the spec's standing instruction required.

**Used by** G5 whole, its first bound (the name carries the scene name), and G5's closing sentence that §13 rule 2 is kept rather than bent.

**Not covered:** one Blender version, one Linux rental, two induced crashes. **Nothing here is a reading of a crash file written on the user's own computer**, and nothing in the document takes one.

## The Blender facts about *when* a crash file exists ([6](../Reporting%20a%20problem.md#6-what-a-reachable-machine-adds) G5's third bound)

**Named by the document as the research's, and not re-derived here.** The route is the fixed one and was walked when the fact was named: `Preparation/Blender Research/Across topics/The map.md`, then `When a render ends.md`'s *Whether a crash file is written* section, then the one comparison question it names. The citations are the research's own and are carried in this project at [Facts](../Facts.md) Fact 131 and at [Rows/Reporting a problem](../Rows/Reporting%20a%20problem.md); **this entry points there rather than keeping a third copy.**

The three settled claims, in the research's words as those two carry them: *"A Blender made to crash writes a crash file and says so, at 139, in all four versions: on Linux in the temporary folder under the loaded file's name, on the Mac in that machine's own temporary folder."*; *"Anything printed immediately before the crash never reaches the ordinary output."*; and *"Running the machine's own memory out writes no crash file at all, at either of the two statuses that ending gives"*.

**Not covered, and the document says so in place:** *"Whether one is written when a Blender is replaced under a running render is not settled — two runs of one test on one machine disagree and both stand."* G5's third bound is written to that — the absence of a crash file is never evidence about the ending. Under the Creator's ruling of 2026-08-31 there is no further Blender research, so this stays unsettled rather than pending.

---

# Section 2 — facts whose home is another document's companion file

Each of these is an outside fact the document leans on and does not own. **The citation has one home and it is named rather than copied.**

## The per-user application-data location, per system ([1](../Reporting%20a%20problem.md#1-the-diary) R1, R6)

**Home:** [Desktop app and delivery's companion](Desktop%20app%20and%20delivery.sources.md), *Section 10 — the application identity adapter*, which owns the platform call that names the location; that file's Section 5 note says outright that the location is entered there and nowhere else. R1 applies `Desktop app and delivery` **L3**'s rule and re-decides nothing: the path comes from each system's own call and is never assembled from the login account name.

**Not covered:** nothing in this document reads or composes such a path itself, so its whole exposure is L3's.

## The durable-write calls on macOS and Linux ([1](../Reporting%20a%20problem.md#1-the-diary) R5, [3](../Reporting%20a%20problem.md#3-what-goes-into-the-diary) J8)

**Home:** [State, identity and recovery's companion](State,%20identity%20and%20recovery.sources.md), its `fsync(2)` entries for Linux and for macOS. The macOS entry carries Apple's own words — *"Mac OS X provides the F_FULLFSYNC fcntl. The F_FULLFSYNC fcntl asks the drive to flush all buffered data to permanent storage."*, with the page's own admission that plain `fsync` is not enough because *"the drive itself may not physically write the data to the platters for quite some time"* — read on this same Mac, and the Linux entry carries the kernel manual's. **J8 inherits both and adds nothing to either.**

**Not covered, and it is why Section 3 carries a Windows entry:** that companion records Windows as unresolved for this, so J8's `FlushFileBuffers` had no home anywhere. It is read directly below.

## The atomic publication of a composed file ([7](../Reporting%20a%20problem.md#7-writing-the-file-and-putting-it-in-front-of-the-user) V6)

**Home:** Fact 79, held by [Desktop app and delivery](../Desktop%20app%20and%20delivery.md) **F16** with its citations in [that document's companion](Desktop%20app%20and%20delivery.sources.md) — macOS `rename(2)`'s crash guarantee, Linux `rename(2)`'s atomic replacement, and Windows' documented **absence** of a crash guarantee for either of its two routes, with `MoveFileExW` and `ReplaceFileW` addressed there. [Download's companion](Download.sources.md) carries the macOS address again for its own reason and says why. **V6 uses F15's one-step publication as supplied and decides nothing about it.**

**Not covered:** the Windows third of V6 inherits F16's negative — no crash guarantee is documented — and the document does not claim one.

## Where a proposed folder comes from, per system ([7](../Reporting%20a%20problem.md#7-writing-the-file-and-putting-it-in-front-of-the-user) V4)

**Home:** Fact 22, owned by [The user's own computer](../The%20user's%20own%20computer%20—%20where%20a%20download%20lands,%20what%20it%20will%20take,%20what%20it%20will%20show,%20and%20how%20the%20app%20replaces%20itself.md) §1 and entered at [Download's companion](Download.sources.md) under *Fact 22*, consumed by both documents through `Desktop app and delivery` §8 W1–W7 rather than directly. The three claims V4 rides on are that one's: *"The three systems are three different mechanisms, not three spellings of one"*; *"The documented Linux fallback is `$HOME`, not `$HOME/Downloads`"*; and Apple's instruction that an app *"must not write files to the preceding directories unless explicitly directed to do so by the user"*, which is what V4's *the user's press is the explicit direction* answers.

**Not covered:** V4's *where the route returns nothing, nothing is proposed* is W5's rule applied, not a reading, and carries no citation.

## The Linux reveal route's address, and the shell's refusal ([7](../Reporting%20a%20problem.md#7-writing-the-file-and-putting-it-in-front-of-the-user) V7)

**Home:** [Desktop app and delivery's companion](Desktop%20app%20and%20delivery.sources.md), the tenth sitting's Fact 132 entry, which carries the portal's own words — *"Asks to open the directory containing a local file or directory represented by `fd` in the file browser, possibly selecting it. This is implemented using the `org.freedesktop.FileManager1.ShowItems()` D-Bus API and if that service doesn't exist falls back to the `OpenURI` method"* — with both addresses, `https://flatpak.github.io/xdg-desktop-portal/docs/doc-org.freedesktop.portal.OpenURI.html` and the project's own shipped interface definition, read 2026-08-25; the `xdg-open` scope sentences; and the shell's `file`-scheme refusal. **The service those citations back is `Desktop app and delivery` §18.6 K20–K24, which V7 consumes.** This entry points there and keeps no second copy.

**Not covered:** everything on Linux here is read and never run, and no file manager is assumed — the fallback is the portal's published behaviour, not the application's error handling.

## The rental's own settings hold secrets in plain text ([4](../Reporting%20a%20problem.md#4-what-it-never-holds-and-what-it-holds-that-is-the-users) N2, fourth bullet)

**Home:** [Security and licence's companion](Security%20and%20licence.sources.md), pointing at [The rented machine](../The%20rented%20machine.md), *What identifies the rental*: the provider's own settings **"hold secrets in plain text — an access key for the provider's own service, and a password for a web page it runs"**, with the finding's own conclusion that **"Anything that reads the machine's settings and writes them into a log would be writing those out too."** Label: **observed**. That is the ground under `Security and licence` §13 rule 2, which N2 cites and does not re-decide.

**Not covered:** it is a reading of a **rented** machine and says nothing about the user's own computer's environment.

## What the diary's protection rests on, and why nothing encrypts it ([1](../Reporting%20a%20problem.md#1-the-diary) R6)

**Home:** [Security and licence's companion](Security%20and%20licence.sources.md), in **two** entries and not one. The Secret Service specification's own words — **"The lookup attributes are not treated as secret material, and the service may choose not to encrypt attributes when storing them to disk"** — are that file's **§3.0 rule 1** entry, headed *lookup attributes are not secret material*; the *nowhere to remember* case that follows from a machine with no working store is its **§3.3** entries, which own what a Linux store does and what its specification declines to mandate. R6 cites `Security and licence` §9's argument whole and remakes none of it. *(Corrected 2026-09-02 at the stage-close gate repair: this named §3.3 for both halves, and the quoted sentence lives at §3.0 rule 1 — the right section number for the other half, in the wrong place for this one.)*

**Not covered:** R6's claim is that the operating system's own file permissions are sufficient **because the diary holds no secret value by construction** (N1–N2). The sufficiency is a decision on that ground, not a reading, and carries no entry of its own.

## The taken check is the filesystem's own exclusive create ([7](../Reporting%20a%20problem.md#7-writing-the-file-and-putting-it-in-front-of-the-user) V5)

**Added 2026-09-02 at the stage-close gate repair**, where V5 was found stating an outside-world mechanism whose home exists in this stage and was not pointed at — the same shape V4, V6 and V7 already have entries for.

**Home:** [Desktop app and delivery](../Desktop%20app%20and%20delivery.md) **§7.3 C10**, in its own words: *"The question* is this name already taken *is put to the filesystem as an exclusive create of the final name — create if absent, fail if present — and the created entry is the reservation publication renames over."* The readings underneath it are that document's, entered at [its companion](Desktop%20app%20and%20delivery.sources.md), *Section 7 — the receiving-filesystem collision judgement*: the folding facts C11 rests on — macOS full folding observed, NTFS's rule **undocumented on every page read** — and the input document's own warning that the trap is *"any canonicaliser that manipulates strings rather than calling the system"*, which is the ground for asking the filesystem instead of modelling it.

**V5 consumes the mechanism and decides nothing about it.** What is V5's own is the branch taken on *taken* — recompose with a non-colliding suffix and judge again, rather than prompt — which is Row 303's appending branch and a decision, not a reading.

**Not covered:** C12's *no bytes are published and nothing is overwritten* is documented across all three systems and **run on none but the Mac**; C13's declared zero-byte reservation window and C22's every-publication-is-a-replace consequence are that document's unresolved items 30 and 4, and V5 inherits both without adding to either.

---

# Section 3 — publications read for this file on 2026-09-02

Each entry below is the thing's own documentation, read at the address given, on the date given. **Every one of them existed in the project as a quotation with no address**, or as a claim the input document made without naming which of a publisher's pages carries it.

## The Windows requirement — `SHOpenFolderAndSelectItems` must have the component system initialised ([7](../Reporting%20a%20problem.md#7-writing-the-file-and-putting-it-in-front-of-the-user) V7)

**Direct publication, and this is the entry the file was written for.** Microsoft, *SHOpenFolderAndSelectItems function (shlobj_core.h)*, `https://learn.microsoft.com/en-us/windows/win32/api/shlobj_core/nf-shlobj_core-shopenfolderandselectitems`, page `ms.date` **2018-12-05**, `updated_at` **2025-07-01**, read **2026-09-02**. Label: **documented**, first-party.

Remarks, in full and verbatim:

> "CoInitialize or CoInitializeEx must be called before using **SHOpenFolderAndSelectItems**. Not doing so causes **SHOpenFolderAndSelectItems** to fail."

**V7's wording is exact rather than generous.** *"which its page requires rather than suggests"* is supported twice over on the page — the modal is *must*, and the consequence of not doing it is stated as failure rather than as a risk. The contrast V7 draws is the input document's and is with `ShellExecute`, whose own page says it *"should be initialized"*; that second page is not read here and the contrast stays the input document's.

**Three more things the page states that V7 and [12](../Reporting%20a%20problem.md#12-unresolved) item 2 depend on.** The return value: *"If this function succeeds, it returns **S_OK**. Otherwise, it returns an **HRESULT** error code."* — which is what makes the Windows third of V8's *the reveal's outcome is learned* true rather than assumed. The count-of-zero form the input document names: *"If cidl is zero, then pidlFolder must point to a fully specified ITEMIDLIST describing a single item to select. This function opens the parent folder and selects that item."* And the requirements the build output needs: minimum supported client *"Windows XP [desktop apps only]"*, header `shlobj_core.h` (include `Shlobj.h`), library `Shell32.lib`, DLL `Shell32.dll (version 6.0 or later)` — all present on an ordinary install, which is why the input document's *unelevated side-by-side install works* is a documentation claim and not a hope.

**Not covered:** the page says nothing about whether the reveal brings a window forward or highlights the right row, and **nothing on it has been run** — no Windows machine has executed this call on this project. The page is a contract, not an observation.

## macOS — the reveal call selects the symlink, and returns a status ([7](../Reporting%20a%20problem.md#7-writing-the-file-and-putting-it-in-front-of-the-user) V7)

**Direct publication.** Apple, *NSWorkspace `selectFile(_:inFileViewerRootedAtPath:)`*, `https://developer.apple.com/documentation/appkit/nsworkspace/selectfile(_:infileviewerrootedatpath:)`, read **2026-09-02**. The page is served as data at `https://developer.apple.com/tutorials/data/documentation/appkit/nsworkspace/selectfile(_:infileviewerrootedatpath:).json`, which is where the words below were taken from; the rendered page carries the same content. Label: **documented**, first-party.

Discussion, verbatim:

> "In macOS 10.5 and later, this method does not follow symlinks when selecting the file. If the `fullPath` parameter contains any symlinks, this method selects the symlink instead of the file it targets. If you want to select the target file, use the `resolvingSymlinksInPath` method to resolve any symlinks before calling this method."

Return value, verbatim:

> "`true` if the file was successfully selected; otherwise, `false`."

And the parameter rule V7's *rooted at* half needs: *"If you specify a nonempty path string, this method opens a new file viewer. If you specify an empty string (`@""`), this method selects the file in the main viewer."*

**Both halves of V7's macOS clause are the page's own.** *"with symbolic links resolved before asking, since the call selects a symlink rather than its target"* is the discussion's instruction, not an inference; and *"the reveal call that returns a status"* is the return value. **Apple names the remedy as well as the trap**, which V7 does not say and a builder needs: resolve with `resolvingSymlinksInPath` before the call.

**Not covered:** the page says nothing about why the call would return false, so the input document's three observed false cases — missing file, missing root directory, missing folder — remain the observation and not the contract. And it says nothing about `activateFileViewerSelectingURLs:`, whose no-return declaration is that other page's; V7 refuses that variant on the input document's ground.

## Windows — the force-down call, and the one thing its page does not claim ([3](../Reporting%20a%20problem.md#3-what-goes-into-the-diary) J8)

**Direct publication.** Microsoft, *FlushFileBuffers function (fileapi.h)*, `https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-flushfilebuffers`, page `ms.date` **2018-12-05**, `updated_at` **2025-07-01**, read **2026-09-02**. Label: **documented**, first-party.

The description and the Remarks sentence J8 rests on, verbatim:

> "Flushes the buffers of a specified file and causes all buffered data to be written to a file."

> "Typically the WriteFile and WriteFileEx functions write data to an internal buffer that the operating system writes to a disk or communication pipe on a regular basis. The **FlushFileBuffers** function writes all the buffered information for a specified file to the device or pipe."

And the unbuffered alternative the input document names beside it, verbatim:

> "To open a file for unbuffered I/O, call the CreateFile function with the **FILE_FLAG_NO_BUFFERING** and **FILE_FLAG_WRITE_THROUGH** flags. This prevents the file contents from being cached and flushes the metadata to disk with each write."

**The negative the input document reported is confirmed by this reading rather than carried across.** The page's only mention of caching is *"Due to disk caching interactions within the system, the FlushFileBuffers function can be inefficient…"* — *within the system* being the operating system's cache. **Microsoft nowhere on this page claims the call defeats the drive's own hardware cache**, which is exactly what Apple's `fsync(2)` page states outright for `F_FULLFSYNC` and what the Linux `fsync(2)` page claims with its own older-kernel qualifier. **So J8's three calls are not three equal guarantees**: macOS's is documented down to the platter, Linux's is documented down to the platter with a qualifier, and Windows' is documented down to the device and no further. Nothing in the document turns on the difference — J8's promise is that the line is not sitting in a process's own buffer when the process dies — but a builder must not read the three as interchangeable.

**Not covered:** nothing on the page has been run, and the page's *unbuffered I/O instead* advice is about efficiency under many writes, which is not J8's case — J8 writes one line at one ending.

## Windows — `TerminateProcess` runs nothing, and which page actually says so ([3](../Reporting%20a%20problem.md#3-what-goes-into-the-diary) J9)

**Direct publications, two, and naming which is which is this entry's point.** The input document attributes to one unnamed page two sentences that live on two different Microsoft pages, so a checker looking for either at the obvious address finds one and not the other.

- **Microsoft, *TerminateProcess function (processthreadsapi.h)***, `https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-terminateprocess`, page `ms.date` **2025-12-30**, `updated_at` **2026-01-07**, read **2026-09-02**. Its words: *"The **TerminateProcess** function is used to unconditionally cause a process to exit."*; *"This function stops execution of all threads within the process and requests cancellation of all pending I/O."*; and **"A process cannot prevent itself from being terminated."**
- **Microsoft, *Terminating a Process***, `https://learn.microsoft.com/en-us/windows/win32/procthread/terminating-a-process`, page `ms.date` **2025-07-14**, `updated_at` **2025-07-16**, read **2026-09-02**. Its words, and this is where the input document's headline sentence actually is: **"If a process is terminated by TerminateProcess, all threads of the process are terminated immediately with no chance to run additional code. This means that the thread does not execute code in termination handler blocks. In addition, no attached DLLs are notified that the process is detaching."**

**J9's claim is supported and the attribution is repaired.** *"`TerminateProcess` running nothing on Windows"* is the second page's sentence word for word; *"A process cannot prevent itself from being terminated"* is the first page's. **The reference page as served today does not contain the *no chance to run additional code* string at all** — it says *"stops execution of all threads within the process"* instead — so a checker sent to the reference page alone would have found the input document's quotation missing and would have been wrong to call it dead.

**Not covered:** neither page has been run against, and neither says what Task Manager's *End task* does to a windowed application — the input document parks that as wanting a Windows computer.

## macOS — the two signals a handler cannot be installed for ([3](../Reporting%20a%20problem.md#3-what-goes-into-the-diary) J9)

**Direct publication, read on the machine.** `sigaction(2)`, the manual page shipped with the operating system, read with `man 2 sigaction` on the Creator's own Mac — **macOS 26.6.2, build 25G83, arm64** — on **2026-09-02**. Label: **documented**, first-party, read locally rather than at an address. Its words:

> "[EINVAL]  An attempt is made to ignore or supply a handler for SIGKILL or SIGSTOP."

> "The sa_mask field specified in act is not allowed to block SIGKILL or SIGSTOP.  Any attempt to do so will be silently ignored."

and, in the signal table, `SIGSTOP` — *"stop process  stop (cannot be caught or ignored)"*.

**This is J9's macOS third, which the input document asserted without a citation of its own** — its §1 table says *"the system refuses the handler outright"* and quotes only the Linux page. The refusal is an error the call returns, which is what *refused* means here.

**Not covered:** one macOS version on one machine. The page says nothing about **sudden termination**, which is the other macOS ending J9 folds into *any other silent death*; that remains the input document's Apple quotation and is not re-sourced here.

## Linux — the two signals that cannot be caught ([3](../Reporting%20a%20problem.md#3-what-goes-into-the-diary) J9)

**Direct publication.** `signal(7)`, the Linux man-pages project, `https://man7.org/linux/man-pages/man7/signal.7.html`, page footer *"Linux man-pages 6.18            2026-02-08"*, read **2026-09-02**. Label: **documented**, first-party. Its words:

> "The signals **SIGKILL** and **SIGSTOP** cannot be caught, blocked, or ignored."

**This is the one sentence in Fact 129's table the input document quotes with the page named**, and the address was recorded nowhere in `Preparation/`. It is the same publication and the same project this stage already cites for `rename(2)` and `wait(2)` at [State, identity and recovery's companion](State,%20identity%20and%20recovery.sources.md).

**Not covered:** the page is the manual, not the kernel; the out-of-memory killer's choice of signal is the next entry and is not on it.

## Linux — the out-of-memory killer uses `SIGKILL`, from the kernel's own source ([3](../Reporting%20a%20problem.md#3-what-goes-into-the-diary) J9)

**Direct publication, the vendor's own reference implementation, at a version-pinned address.** `mm/oom_kill.c` in the Linux kernel, read **2026-09-02** at **`https://raw.githubusercontent.com/torvalds/linux/v6.12/mm/oom_kill.c`** — the release tag `v6.12`, so the address does not move. In `__oom_kill_process`, lines 943 to 947, verbatim:

> 	/*
> 	 * We should send SIGKILL before granting access to memory reserves
> 	 * in order to prevent the OOM victim from depleting the memory
> 	 * reserves from the user space under its control.
> 	 */
> 	do_send_sig_info(SIGKILL, SEND_SIG_PRIV, victim, PIDTYPE_TGID);

and the same call again at line 987, for the victim's other thread groups.

**This closes the trail hazard the input document flagged against itself.** Its §1 records *"that source was cited at a branch-tip address, which moves. If the citation is meant to survive it wants a commit-pinned address."* A tag is what it wanted; this is it. **The claim is unchanged** — J9's *"Linux's out-of-memory killer uses `SIGKILL`, so that ending writes nothing, ever"* is the source line plus the previous entry's sentence, and nothing more.

**Not covered, and it is the input document's own note:** the kernel's **prose** documentation describes the killer without naming the signal — *"the name is only in the source"* — so no manual page backs this and none was found. **And the address is a mirror rather than the canonical one:** `git.kernel.org` refused the fetch on 2026-09-02 (HTTP 403). `torvalds/linux` is the kernel project's own published tree and the tag pins the content; a reader who can reach `git.kernel.org` should prefer `linux.git` at the same tag.

## Whether a delivered signal carries its sender — and it does, which is the opposite of what J9 asserted ([3](../Reporting%20a%20problem.md#3-what-goes-into-the-diary) J9)

**Direct publications, two, and this entry exists because the claim behind them was going the other way.** J9 read *"A force-quit's termination signal carries no origin"*, asserted flatly by the input document with **no publication anywhere in this project behind it**. It was looked for on 2026-09-02 and what the operating systems publish is the reverse.

- **The header shipped in the operating system.** Apple's `sys/signal.h`, read on the Creator's own Mac — **macOS 26.6.2, build 25G83, arm64** — on **2026-09-02**, at the shipped SDK path `/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/signal.h`. In the `siginfo_t` declaration, verbatim, comments and all:

  > 	pid_t   si_pid;                 /* sending process */
  > 	uid_t   si_uid;                 /* sender's ruid */

  Label: **documented**, first-party, read locally rather than at an address.

- **Linux, the manual page.** `sigaction(2)`, the Linux man-pages project, `https://man7.org/linux/man-pages/man2/sigaction.2.html`, page footer *"Linux man-pages 6.18            2026-02-08"*, read **2026-09-02**. Its `siginfo_t` listing carries *"pid_t    si_pid;       /* Sending process ID */"* and *"uid_t    si_uid;       /* Real user ID of sending process */"*, and the page states which senders fill them: **"Signals sent with kill(2) and sigqueue(3) fill in si_pid and si_uid."** Label: **documented**, first-party. The same publication and the same project this file already cites for `signal(7)`.

**So a handler installed with `SA_SIGINFO` can be handed the sending process's identifier, on both systems the clause was about.** The macOS `sigaction(2)` page read for the entry above says nothing either way — it names `si_code` and no other member — so the header is what settles the macOS half.

**What this changed in the document, and what it did not.** J9's **answer is unchanged**: the last line still records the signal the system delivered and never a claim of who sent it. What was corrected is the **ground** — it is a decision of this document, on J3's boundary (a sending process identifier is a run-time internal, and a number no report's reader can resolve) and on the fact that nothing in the spec's promise rests on it, rather than something the system withheld. **This is deliberately not a Section 4 entry.** The gate expected one, on the reading that no source existed; a source exists and says the opposite, and recording *no source* would have left a false claim standing under an honest-looking label.

**Not covered:** which signal any particular force-quit route actually sends, and what `si_code` it arrives with — neither page says, and no force-quit was driven for this reading. Nothing in the document turns on either, and [12](../Reporting%20a%20problem.md#12-unresolved) item 3 still carries the macOS force-quit residue at the width the check supports. **And nothing here was run**: no handler was installed and no signal was delivered for this entry. The Windows third of J9 is untouched — `TerminateProcess` delivers no signal at all, which is the entry above.

---

# Section 4 — no source in this project

## Whether an instance hold survives a process that died without running code ([12](../Reporting%20a%20problem.md#12-unresolved) item 12; [9](../Reporting%20a%20problem.md#9-after-a-crash) C1's first condition)

**Unresolved, and it is Rule 2 applied rather than a gap that can be closed by reading harder.** The document parks it in terms and guesses nothing: `Desktop app and delivery` **P2** states the one-instance rule and leaves the enforcing mechanism to Build, so there is no named mechanism whose documentation could be read. **What was looked in:** the input document, which answers three desktop questions and not this one; `Desktop app and delivery` and its companion, where P2 is a rule and not a call; and this document's own §12, which states the question and declines it.

**Breaks:** on a system where a stale hold can outlive the process, the launch after a real crash never becomes the ordinary running application, C1's first condition fails, and the crash the user just had is never raised — Row 479's *raised at the next launch* is not kept there.

**Holds without it:** the menu entry point (X2), whose slice is the current session and the one before; the crashed run's lines, still in the diary and still carrying its identity (U7); and C1's deliberate failure direction — a launch that establishes nothing opens nothing.

## Whether forced-down bytes survive a real power loss ([3](../Reporting%20a%20problem.md#3-what-goes-into-the-diary) J8)

**Unresolved, and untestable by any means this project has.** The input document's own closing line: it is *"untestable without pulling power — Apple's own page warns some drives ignore the request."* The Windows entry in Section 3 adds the second half of the same limit — Microsoft does not claim the drive's cache at all.

**Breaks:** nothing the document promises. J8's promise is that the last line is not lost **in the process's own buffer**, which is what the observed buffering result establishes; surviving the platter is not claimed anywhere in the document.

## The macOS force-quit ground, and the two systems nobody has run ([12](../Reporting%20a%20problem.md#12-unresolved) items 2 and 3)

**Recorded here because the document's own unresolved items are the honest form and no source can move them.** Item 3: what was exercised is the programmatic force-terminate call against a **responsive** application; the real Force Quit panel and a wedged application were never driven, and the input document names the hand test that would close it — **the Creator running the test application and force-quitting it by hand, two minutes' work**. Item 2: every Windows and every Linux claim in the document is documentation alone, and Section 3's addresses do not change that — reading Microsoft's page is not running Windows.

**What holds:** every clause names its system and its label, so nothing in the document reads as observed that was not.

## What carries no entry at all

**Named so a checker does not go looking.** The diary's 20 MB cap, the 5 MB report ceiling ([5](../Reporting%20a%20problem.md#5-a-report-is-a-slice) S3) and the 512-entry journal tail ([6](../Reporting%20a%20problem.md#6-what-a-reachable-machine-adds) G2) are Smoozy's own figures, fixed rather than read; [12](../Reporting%20a%20problem.md#12-unresolved) item 9 records them as decisions and the document says at each clause that no reading and no outside claim decides it. The seven line kinds, the three slices, the middle-drop, the per-part states, the run identity's rules, the two entry points and the launch prompt's placement outside the failure shape are all decisions. **A decision has no source and earns no citation** — Rule 2's own boundary — and the ones marked **Creator-stated** in the document carry that mark to say who decided, not where a fact came from.

---

*2026-09-02 — written at the twentieth sitting, from [Reporting a problem](../Reporting%20a%20problem.md) read whole, against the input documents it names, the companion files that already hold its facts, and the publications read on the date above.*

**Twenty-five entries, and ten publications in eight of them.** Five are pointers into this stage's frozen input documents; eight point at a citation whose home is another document's companion file; eight are publications read here on 2026-09-02 — ten publications, because the `TerminateProcess` entry carries two pages and the signal-origin entry carries two; four record that no source exists. *(Counts corrected and two entries added on 2026-09-02, later the same day, at the stage-close gate repair. The file's closing line had said* seven publications *where seven was the number of entries, and the signature line said* twenty-one entries *and then enumerated six, seven, seven and four for a file that held twenty-three with five pointers in Section 1. The two entries added are V5's exclusive create, whose home existed and was not pointed at, and the signal-origin reading, which found J9's asserted claim contradicted by both systems' own documentation.)*
