# The three desktop systems read again — dying, revealing, resolving, and what the shell does not expose

*The answer of record for Facts 129, 130, 132, 133, 136, 137, 138 and 140, read on 2026-08-25. Every claim here is either quoted from the thing's own publication or run on the Creator's Mac, and each carries its label at the point it is made. Nothing was rented, nothing was installed, nothing was paid for, and no bot, notification or message was ever created or sent.*

**Where the evidence is.** `Audits/Architecture/The reading batch of 2026-08-25/` holds five findings files and, beside each, its own citation check — one checker per findings file, each fetching every cited address itself and re-running every observation on the same Mac. That folder is the raw material; this document is what a later architecture document reads. **Do not open the trail to answer a question this file answers.** *(One trail note for whoever audits that folder: it carries the citation checkers' brief but not the reading agents' brief, which was composed per agent and never written down there.)*

**Where a findings file and its check disagreed, the check won.** That is not a courtesy — it is why the checks were run, and in this batch it earned its place four times: it **reversed two stated negatives outright** (Linux can reveal a named file; Windows does publish a Help key), it **narrowed the batch's most load-bearing macOS finding** to what was actually exercised, and it **struck a claim of an unsigned binary that this machine cannot produce**. Every reversal is written below in its reversed form only. A reader must not meet the old form anywhere in this document.

**The one limit that runs through everything below, and it is not closed here.** **No Windows machine has run any of it**, and there is **no Linux desktop computer** either — the Linux machines this project rents are headless servers, not a user's own computer. A Windows machine can be rented and four have been, so the Windows half is **untried rather than untestable**; but **every Windows machine this project has read reported a Server edition**, not the desktop edition its image was named for, so a desktop-only question is still out of reach of what has been rented. So **every Windows claim and every Linux claim in this document was read rather than seen**, and every observation was made on **macOS 26.6.2, build 25G83, arm64**. Where a claim is observed on the Mac and documentation-only on the other two, that is said at the claim, not once at the top.

---

## The five things that will bite a builder, gathered

These are scattered across eight facts and every one of them is a place where doing the obvious thing produces a wrong result quietly. Three of the five were found by a checker rather than by the reading agent.

1. **Two macOS calls select the symbolic link rather than what it points at, and neither says so where you will look.** `NSWorkspace`'s reveal-in-Finder call: Apple's own page says *"this method does not follow symlinks when selecting the file. If the `fullPath` parameter contains any symlinks, this method selects the symlink instead of the file it targets"* (§3). And Apple's file-identity value does the same thing, which makes it **disagree with `stat`** about whether two paths are one folder (§8). Both are fixed by resolving symlinks first; neither is fixed by anything else.
2. **The Windows 11 system typeface ships no italic at all.** Segoe UI Variable has fifteen named instances and not one of them is italic, and its only axes are weight and optical size. If the Windows web view lands on it, the app's italic text is a machine shear (§5).
3. **Telegram publishes a per-account ceiling on how many bots one person may own** — `BOT_CREATE_LIMIT_EXCEEDED`, with a default limit and a higher one for paid accounts. For an app whose plan is that every user makes their own bot, that is material, and no reading agent noticed it (§4).
4. **A buffered write in a dying process loses everything.** The last line must be written with a raw write and forced down, not with a buffered writer — proved twice, and the exit status proves the handler ran in both cases (§1).
5. **The one route that reveals a file on Linux was declared not to exist, and does.** `org.freedesktop.portal.OpenURI.OpenDirectory` is published, versioned, and falls back automatically when no file manager owns the older interface (§3).

---

## 1. What a dying process may run and write (Fact 129)

**Answered on all three, with one qualification that travels with the headline.**

| | macOS | Windows *(documentation only)* | Linux *(documentation only)* |
|---|---|---|---|
| Endings that let you run code | Any signal you install a handler for; and the ordinary **Quit** path, which is not a signal at all but an Apple Event landing in `applicationShouldTerminate:` then `applicationWillTerminate:` | Console: `SetConsoleCtrlHandler` receiving `CTRL_C_EVENT`, `CTRL_BREAK_EVENT`, `CTRL_CLOSE_EVENT`. Windowed: `WM_QUERYENDSESSION`, `WM_ENDSESSION`, `WM_CLOSE`. Also the error-reporting **recovery callback**, on unhandled exception or hang | Any signal you install a handler for |
| Endings that give you nothing | `SIGKILL` and `SIGSTOP` — the system refuses the handler outright. Also **sudden termination**: Apple's own words, the process "may be killed by the operating system without any notification or event being sent to the process first" | `TerminateProcess` — "all threads of the process are terminated immediately with no chance to run additional code," no DLL is notified, and "A process cannot prevent itself from being terminated" | `signal(7)`: "The signals **SIGKILL** and **SIGSTOP** cannot be caught, blocked, or ignored" |
| What forces a write to the platter | **`fcntl(fd, F_FULLFSYNC, 0)`. `fsync` is documented as not enough** — it moves data to the drive, but "the drive itself may not physically write the data to the platters for quite some time" | **`FlushFileBuffers`**, or open with `FILE_FLAG_NO_BUFFERING` and `FILE_FLAG_WRITE_THROUGH`. Microsoft does **not** claim on that page that it defeats the drive's own cache — confirmed as a real negative; the page's only caching mention is "within the system", which is the operating system's cache | `fsync(2)`, which does claim the hardware cache — "This includes writing through or flushing a disk cache if present" — **but the same page qualifies it, and the qualifier matters**: "The fsync() implementations in older kernels and lesser used filesystems do not know how to flush disk caches. In these cases disk caches need to be disabled using hdparm(8) or sdparm(8) to guarantee safe operation" |
| Time limit on the handler | **None that this machine could find.** *(Run: a handler that slept 20 seconds completed all 20 — reproduced at 20.023 s. A refusing handler was still alive 301 seconds on — reproduced and extended past the reading agent's own 185 s.)* The one published macOS number is **two minutes** to answer `replyToApplicationShouldTerminate:` after returning `NSTerminateLater`, and that is scoped to the **logout / restart / shutdown** path only | Console, published: `CTRL_CLOSE_EVENT` **5000 ms**; `CTRL_LOGOFF_EVENT` and non-service `CTRL_SHUTDOWN_EVENT` **5000 ms**; service `CTRL_SHUTDOWN_EVENT` **20000 ms**; `CTRL_C` / `CTRL_BREAK` **no timeout**. **For windowed apps there is no published general budget** — see the correction below | **No kernel time limit** published for a handler on a plain process. Deadlines come from whoever is killing you, not from the kernel |

### The macOS headline, at the width the check supports

**`NSRunningApplication.forceTerminate()` delivers a catchable `SIGTERM` to a responsive application, and does not escalate.** *(Run, and independently rebuilt from scratch by the citation check: three runs, two launch routes including an application properly registered through LaunchServices with parent process 1. The handler ran, the line landed, the process exited 42. A refusing handler survived 301 seconds with nothing ever coming for it.)*

**That is the whole of what was exercised, and the qualifier must travel with it.** *(Corrected out of the citation check: the findings file stated this flat, as "force-quit is a catchable `SIGTERM`", in both places a reader takes the answer from.)* Two gaps neither side closed:

- **The Option-Command-Escape panel and Activity Monitor's Force Quit were never driven** — that needs interface automation which would have disturbed the Creator's own session. Apple's own header for these two methods says only "Attempts to quit the receiver normally" and "Attempts to force the receiver to quit", and **mentions no signal at all**, which is exactly why the behaviour had to be run. No publication ties the panel to the method.
- **Every run fired at a *responsive* application.** The Force Quit panel is aimed at a wedged one, and a wedged application may not get the same treatment.

**Normal quit and forced quit are two completely different doorways**, and that half is unqualified. *(Run, reproduced.)* Normal termination produced both delegate callbacks and **no signal**, exit 0. Forced termination produced **the signal and no callbacks**, exit 42.

**What the app cannot tell apart:** a forced quit's `SIGTERM` from any other `SIGTERM` — a script's, a parent's, `killall`'s. The signal carries no origin. And a kill that runs nothing is by construction invisible.

### The buffering result, which is the most practical thing in this fact

| Run | What the handler did | File afterwards |
|---|---|---|
| Buffered | `fprintf` to a fully-buffered stream, then exit | **0 bytes. The line vanished.** |
| Raw | `write(2)`, then `F_FULLFSYNC`, then exit | The line, plus `F_FULLFSYNC returned 0` |

*(Run, reproduced by the check with its own program.)* **The exit status proves the handler fired in both cases** — so the empty file is not a handler that failed to run; it is a handler that ran and whose bytes died in the buffer. **Write the last line with a raw write and force it down.**

### Out-of-memory, per system

- **macOS: partly published, and the reading agent's negative is now narrower.** Apple's jetsam article names "iOS, iPadOS, tvOS, visionOS, and watchOS" and **macOS is absent from that list** — confirmed, the word does not appear on the page. This machine nevertheless carries kernel settings named `kern.memorystatus.kill_on_sustained_pressure_count` (0), `..._window_s` (600) and `..._delay_ms` (500) — observed, not documented. **But Apple does publish that macOS applications can exit under memory pressure**, in the shipped MetricKit header: `MXBackgroundExitData.cumulativeMemoryPressureExitCount`, marked available from macOS 11.0, "Cumulative number of times the application exited due to memory pressure on the system." *(Found by the citation check; the findings file's "Apple publishes no prose about it" was too strong.)* What it does **not** say is what signal, or whether anything runs.
- **Windows: a notification, not a killing.** `CreateMemoryResourceNotification` exists so "the application can reduce its working set". **Nothing on that page says Windows terminates the application** — confirmed as a real negative. No Microsoft publication read describes a desktop equivalent of jetsam.
- **Linux: the killer uses `SIGKILL`, so nothing runs, ever.** The kernel's own source calls `do_send_sig_info(SIGKILL, SEND_SIG_PRIV, victim, PIDTYPE_TGID)`. Note that the kernel's *prose* documentation describes the killer without naming the signal — the name is only in the source. *(Trail hazard, from the check: that source was cited at a branch-tip address, which moves. If the citation is meant to survive it wants a commit-pinned address.)*

### Two Windows corrections, both out of the citation check

- **The logoff and shutdown console events do not reach an ordinary application.** Microsoft's own page: they are received **only by services**, because "Interactive applications are terminated at logoff, so they are not present when the system sends this signal." The findings file carried that caveat in its evidence but not in the table a reader uses.
- **There is no published general budget for a windowed application.** The five seconds on `WM_QUERYENDSESSION` is when the system *displays a screen naming the applications preventing shutdown* and lets the user terminate them — not a hard deadline for answering. The five- and thirty-second figures on the recovery-callback page are explicitly scoped to the case where an installer wants to update a component of your application. Neither page supports a general windowed-app budget, and the findings file's merged version of them is struck.

**One claim is struck outright as unsourced and unlabelled:** *"ordinary read access to the Application log is the normal default."* Microsoft's page says only that access "is restricted" and is granted per account. Nothing supports the rest.

**Not answered, and what would close it:** whether the real Force Quit panel behaves as the method does, and whether a *wedged* application is treated the same *(the Creator running the test app and force-quitting it by hand — two minutes' work, and it would turn a strong proxy into a direct observation)*; whether forced termination ever escalates beyond five minutes *(the same test left running for an hour)*; what macOS does to a process under real memory pressure *(a controlled run on a machine that is not the Creator's daily driver — this must not be done on that machine)*; what Task Manager's End task does to a **windowed** Windows application, which Microsoft publishes only for console processes *(a Windows computer)*; whether `FlushFileBuffers` defeats a drive's hardware cache, which Microsoft does not say the way Apple explicitly does *(a Microsoft page neither side found)*; and whether the bytes forced down on this Mac would truly survive a power loss, which is untestable without pulling power — Apple's own page warns some drives ignore the request.

---

## 2. Whether the next launch can tell the last run died (Fact 130)

**Answered on all three. The advice is the same on all three, and the reason differs.**

**The answer is: keep your own mark.** Every system-supplied route either misses the endings that matter most, or is off by default, or needs privileges a plain downloaded binary does not have. The mark the application writes at startup and clears at a clean exit is not a fallback here; it is the only mechanism that covers every ending.

**But "nothing tells you" is false on macOS, and that reversal is recorded because a builder must not meet the old form.** *(Reversed by the citation check; the findings file said "nothing does" and "Publications I looked at … found nothing.")*

### macOS — a route exists, is published, is unelevated, and still cannot see the ending that matters

- **Crash reports: yes, and readable in process.** *(Run, and re-run by the check with its own crasher.)* A null-pointer dereference produced an `.ips` report in `~/Library/Logs/DiagnosticReports` within seconds, mode `-rw-------`, owned by the running user, **read successfully by an unelevated process**. Apple's own crash-report article gives no programmatic route and no folder path — it describes opening Console by hand; the path came from the run.
- **But not for the endings that matter.** *(Run, reproduced.)* A kill produced **no report**. A plain termination signal with no handler produced **no report**. And the machine held **no** jetsam reports at all in either location, so that route could not even be sampled.
- **MetricKit is the route the findings file missed**, and it ships in Apple's own SDK on this machine — the same class of publication the batch already accepts elsewhere. `MXAppExitMetric` (available from macOS 11.0) publishes counts of how previous runs ended, foreground and background: normal, abnormal, bad access, illegal instruction, watchdog, memory resource limit, and — background only — memory pressure. `MXCrashDiagnostic` and `MXMetricManager.pastDiagnosticPayloads` (macOS 12.0) are readable by an ordinary unelevated application at launch, in process, with no Console and no elevation.
- **What MetricKit does and does not overturn.** It overturns the word *nothing*. It does **not** overturn the advice: it gives cumulative counts over a delivery window rather than a verdict about one specific previous run, and Apple's own caveat in the same header is *"Metrics are not guaranteed to be delivered, but can be expected atleast once per day when conditions permit."* A forced quit almost certainly lands under "exited normally, **or was gracefully terminated by the system**" — so the ending the app cares about stays invisible even here.

### Windows — three routes, each with a condition, and none covers the ending that matters *(documentation only)*

1. **Application Recovery and Restart.** The recovery callback fires only "If the application encounters an unhandled exception or becomes unresponsive." Restart happens only with the user's consent — "the application is not automatically restarted without the user's consent" — and only "if it has been running for a minimum of 60 seconds." Available to a plain unelevated binary; the page is marked "desktop apps only".
2. **Local crash dumps.** "This feature is not enabled by default. Enabling the feature requires administrator privileges." **Not available** on an unconfigured machine.
3. **The event log.** Access "is restricted" and granted per account. **Which event a given death writes is not established** — no page read says whether a forced termination produces any entry naming your process. Do not build on it.

**None of the three fires on `TerminateProcess`.** *(Footnote, from the check: `Windows.ApplicationModel.Activation.ApplicationExecutionState` is a published next-launch route whose `Terminated` member covers "Windows can terminate a suspended app if the system is running low on resources" — but it is packaged-app only, outside this app's shape, and the same table records that **End task in Task Manager produces `NotRunning`, not `Terminated`**. The Windows negative survives.)*

### Linux — nothing, and the negative was attacked and held *(documentation only)*

- **Core dumps do not answer it.** Neither the kill signal nor the plain termination signal is in the list of signals that dump core — confirmed against `signal(7)`'s own list. A dump tells you the process crashed; it says nothing about a forced quit or an out-of-memory kill.
- **The service-manager routes do not apply.** Restart policies and watchdog notifications concern units. This app is a plain downloaded binary the user launches themselves.
- **The out-of-memory kill is in the kernel log**, but reading it is not a right an unelevated app has everywhere — a system setting may forbid it, and journal access depends on group membership. A conditional route, not a reliable one.
- Nine publications were searched and the checker searched them again independently. **Nothing in any of them lets an ordinary application ask the system how its last run ended.**

**Not answered, and what would close it:** whether any Windows event-log entry names a process killed by `TerminateProcess`, and whether an unelevated app can read it *(a Windows computer and a run)*. Everything else in this fact is settled at the width stated.

---

## 3. Showing a folder, or a named file inside its folder, in the system's own file manager (Fact 132)

**Answered on all three — and the Linux answer is the opposite of the one the reading agent reached.**

| | The call that reveals a **named file** | The call that opens a **folder** | Does failure reach the caller? |
|---|---|---|---|
| **macOS** | `selectFile:inFileViewerRootedAtPath:` — returns a status | the same call with a nil file, or `openURL:` | **Yes.** *(Observed on this Mac; reproduced by the check with an independently written probe.)* False for a missing file, false for a missing root directory, false for a missing folder |
| **macOS, the one to avoid** | `activateFileViewerSelectingURLs:` | — | **No.** Returns nothing. Not a run result — the header declares it to return nothing, so there is no outcome that could have counted as failure |
| **macOS, command line** | `open -R` | `open` | **Yes.** *(Observed: exit 1 plus a message for a missing path, ten calls, reproduced exactly.)* **Not documented** — the manual page has no exit-status section at all |
| **Windows** *(docs only)* | `SHOpenFolderAndSelectItems`, with a count of zero to mean "open the parent folder and select that item" | `ShellExecute` on a folder path | **Yes**, per Microsoft: success or an error code from the first; greater than 32 or a named error from the second, with `GetLastError` for detail |
| **Linux** *(docs only)* | **`org.freedesktop.portal.OpenURI.OpenDirectory`**, or the older `org.freedesktop.FileManager1.ShowItems` underneath it | `xdg-open` on a folder path, or `FileManager1.ShowFolders` | **Yes** for the portal, in the ordinary portal way; **yes** for `xdg-open`, by exit code. **Not at the semantic level** for the bare bus interface — its methods have no out-arguments, so a file manager that accepts the call and shows nothing looks like success |

### The Linux reversal, stated as such

*(Reversed by the citation check. The findings file said there is "no published freedesktop specification for showing a folder or revealing a named file in a file manager" and that "the **only** route that selects a named file is the `org.freedesktop.FileManager1` D-Bus interface, which **no freedesktop specification publishes**". Both are false, against a page on a documentation site the same findings file cites for something else.)*

**The XDG Desktop Portal publishes a versioned, documented interface for exactly this job:**

> `OpenDirectory ( IN parent_window s, IN fd h, IN options a{sv}, OUT handle o )`
> "Asks to open the directory containing a local file or directory represented by `fd` in the file browser, possibly selecting it. This is implemented using the `org.freedesktop.FileManager1.ShowItems()` D-Bus API and if that service doesn't exist falls back to the `OpenURI` method on the directory of the file or directory."
> "The OpenDirectory method was introduced in version 3 of the OpenURI portal API."

— read 2026-08-25 at `https://flatpak.github.io/xdg-desktop-portal/docs/doc-org.freedesktop.portal.OpenURI.html`, and confirmed in the project's own shipped interface definition at `https://raw.githubusercontent.com/flatpak/xdg-desktop-portal/main/data/org.freedesktop.portal.OpenURI.xml`, method at line 145.

**Three consequences.** It is a **second route**, so "the only route" is wrong. It **falls back automatically** when nothing owns the older interface, which is published behaviour rather than the app's own error handling. And it **returns a handle and answers on a response signal**, so failure is learnable in the ordinary way rather than only as a raw bus error.

**And the reason that matters more than the citation:** the findings file's Linux warning rested on a D-Bus quote about *a service the bus knows how to start, which then fails to start*. It does not describe the case the warning is about — nothing owning the name and nothing activatable for it. Nothing on that page says what happens then, **which is exactly why the findings file could not find the name of the error it expected**. So *"failure is at least learnable"* was **not** solid for the no-file-manager case. The portal route sidesteps the question entirely.

### What is true about the older interface, and about the specification index

- **The freedesktop specification index really does list 33 specifications and none of them is a file-manager interface** — verified independently, character for character. But **the index itself says "We are not a formal standards body"**, and it does not host the D-Bus specification, AppStream or the authorization framework either; it only links out to them. **Absence from that index proves much less than the findings file made it prove.**
- **`xdg-open` really has no reveal-a-named-file capability of any kind.** A case-insensitive search of its complete source for the obvious words returns exactly one hit, on a line about desktop-environment detection — reproduced character for character, line number and all. Two scope sentences the findings file did not carry: it "supports file, ftp, http and https URLs", and it "is for use inside a desktop session only."
- **The older interface is real and is GNOME's shipped definition** — three methods, taking an array of addresses and a startup identifier. Its bus name is claimed from a service file that ships with GNOME's file manager, not with the base system. *(One framing correction: the comment calling the interface "official" sits above one added property, not two.)*
- **The freedesktop wiki page for that interface could not be read**, by either agent — an anti-scraping challenge, reproduced, not a transient error. Nothing is quoted from it.
- **No claim is made about any file manager other than GNOME's.** Three KDE addresses were tried and all returned nothing, and the checker could not resolve it either. That is a failure to locate a file, not evidence.

### Windows — the folklore and the contract

**Microsoft does not publish the `explorer.exe` command line in any reference documentation.** Confirmed twice, against the current commands reference and the archived one, both of whose alphabetical sections were read in full and contain no entry for it; and a restricted search returned only user-written question-and-answer threads, which this project does not accept. **Treat the `/select` form as folklore.** The documented call is the one that returns a status; a launched process would not.

**One difference between the two Windows calls that is not advisory:** `SHOpenFolderAndSelectItems` **requires** the component system to be initialised — "Not doing so causes … to fail" — while `ShellExecute` says it "should be initialized" as good practice, naming the single-threaded apartment as the type some shell extensions need.

### What macOS actually requires of the application, corrected

**No bundle, no property list, no entitlements, and no Developer ID signature.** *(Observed: a plain command-line binary, compiled with the tools already on this machine, reached the Finder and got true/false answers back for six calls. Reproduced by the check with its own probe, plus a seventh row — a good file with a non-empty root directory, which also succeeds.)*

***Struck: "no code signature of any kind."*** *(Not reproduced. The check inspected an identically built probe: it is signed — ad-hoc, applied by the linker. Every executable of this architecture carries at least that, and an unsigned one will not run at all on this hardware. So that observation could not have been made on this machine, and the conclusion that leaned on it — that the bundled-and-signed case is "strictly easier" — leans on a false premise. What survives, and is what a builder cares about: no bundle, no property list, no entitlements, no Developer ID.)*

***Corrected: "App Sandbox is opt-in" is not a general property of macOS.*** The findings file's own quotation from Apple's page says the opposite for one important path: *"When you create a new macOS app in Xcode, it receives the App Sandbox entitlement and a default set of capabilities."* It is opt-in for an app built the way this one will be built. What Apple's notarization page supports is the narrower and useful thing: **notarization does not require the sandbox** — the requirement list names the hardened runtime, a Developer ID certificate, a secure timestamp, an SDK floor, well-formed entitlements, and the absence of one debugging entitlement, and **the sandbox appears nowhere on it**. It does not support "a downloaded app is not sandboxed" in general.

### The shell reaches none of it

**Wails' runtime exposes nothing that reaches any system's file manager**, and its browser-opening call **refuses the `file` scheme by name**, in a validator that is byte-identical across every current release checked — six references, all verified genuinely distinct trees before the identical result was accepted. A bare filesystem path fares no better: it parses with an empty scheme, which the same line rejects.

**And the call returns nothing at all.** Both failure paths end in a log line, at both layers of the source and on all three platforms. So even if the scheme were permitted, the app could not learn that the open failed — which is precisely what it must be able to do. *(A second, independent instance of the same refusal, from the check: the portal's own general open-address method says "Note that `file://` URIs are explicitly not supported by this method."*)

**Two things about the shell's own category list must not be repeated.** *(Both from the citation check, and the second is confirmed twice over — see §6.)* The seven-name list is **a construction, not a quotation** — the page carries a lead-in and a bulleted list of links. And it is **not the complete set**: the same page documents five further top-level methods in no category, and the shell demonstrably ships whole runtime areas the list omits, including the notification runtime this document relies on in §7. **Do not use that list as evidence that the shell lacks something.**

### Save-a-new-file, where the app supplies the name

**Published on all three, and the shell exposes it.** macOS has a save panel with an editable filename field; Windows has a save dialog with a set-filename method; freedesktop's portal publishes a save-file method with a `current_name` option, "Suggested name of the file." The shell's own save dialog takes a default filename and, unlike the browser call, **returns an error**. On macOS the shell's own code passes that name straight to the property Apple documents.

**One caveat the findings file trimmed away** *(from the check)*: the portal's **folder** suggestion is a suggestion — the page continues, past where the quotation stopped, *"The portal implementation is free to ignore this option."* The **name** carries no such caveat, so the app-supplies-the-name half is safe and the folder half is not.

### What each build output needs

- **macOS, a notarized non-App-Store download: works unchanged.** An unbundled binary already reaches the Finder, so a bundled and signed one is not the harder case for any reason established here.
- **Windows, an unelevated side-by-side install: works.** These are ordinary user-context shell calls; the requirements name only "desktop apps only", a header and a library already present on every install. The one thing the app must do is initialise the component system itself.
- **Linux, a distribution package: works, with two things it must not assume.** `xdg-open` ships in a separate project and must be **declared as a dependency**, or the call fails. And **no file manager can be assumed** — but the portal's documented fallback softens this considerably compared with what the findings file concluded.

**Not answered, and what would close it:** what the sandbox would change for these calls — Apple's sandbox pages do not mention the Finder or these calls, though they do name "Sending Apple Events to arbitrary apps" among restricted activities and state that a sandboxed app "doesn't have unrestricted access to the user's home folder", which the findings file's flat "neither mentions" undersold *(Apple's file-access entitlement reference, or a run from inside a sandboxed bundle, which needs a signing identity)*; which file managers besides GNOME's implement the older interface *(each project's own shipped definition)*; the name of the error returned when nothing owns and nothing can activate the bus name *(the reference implementation's own error-name header — and note the portal route makes this much less important)*; whether the reveal actually brought a window forward with the right row highlighted, which was never checked *(a screenshot)*; and **everything on Windows and Linux, which was read and never run** *(a Windows computer and a Linux desktop)*.

---

## 4. What Telegram requires of a bot's name and username (Fact 133)

**Answered.** This fact is one publisher rather than three, and it is the best-sourced file in the batch: eleven addresses, all loaded, **and every quoted string is on the page it is attributed to** — including Telegram's own inconsistent dashes, reproduced per page. Do not re-check the quotations.

### The username — a published rule, and a validation contract

- **Characters:** letters, digits and underscores only.
- **Length:** **5–32 characters**, and the count **includes the `bot` suffix**.
- **Case:** usernames are **not** case sensitive.
- **Suffix:** the username **must end in `bot`** — stated exactly that broadly, not "`_bot`", not "`Bot`".
- **Permanence:** the username **cannot be changed later**. The only published way to free one is destructive.
- **Exception:** the suffix is what usernames "normally require"; collectible usernames bought elsewhere can be assigned to bots without it. Not a route the app can offer, but a reason not to write "all bot usernames end in bot" as a universal truth.

**It is published as a rule and better than that — as a validation contract.** Telegram instructs client developers to check the username locally before calling its server, and publishes the server errors for each way the rule can be broken, including a distinct error for a missing suffix. **The app can validate a candidate locally before the user ever opens the bot-making chat**, and should state the rule in Telegram's own wording rather than a paraphrase, so a future drift is visible on re-reading.

**One inference must not be presented as Telegram's own statement.** *(Corrected out of the check.)* Telegram never says the suffix check is case-insensitive. What the pages support is narrower and sufficient: **Telegram offers `TetrisBot` as a valid example, so an ending of `Bot` demonstrably satisfies the requirement.**

**And the coverage is uneven per bullet, which matters if the app cites a page.** Case insensitivity and permanence are published on the bot-features page **alone**. "Includes the suffix" is on the two protocol pages **and not** on the bot-features page. Nothing contradicts anything; but no single page carries all six bullets.

### The display name — a length, an error name, and nothing else

**Telegram publishes no character rule for a bot's display name anywhere.** *(This negative was attacked hardest and held — the checker searched a dedicated page on editing bot information that the findings file never opened, and it publishes no character rule and no length for the name either. Treat it as settled and do not pay to check it again.)*

The length bounds live only in the protocol references: **1–64 characters** at creation, **0–64** when renaming, where zero means "remove the dedicated name for this language". The only published signal about content is an error name with no explanation of what triggers it.

**So the app should state the 64-character ceiling and say nothing about permitted characters.**

### Telling a taken username from a free one — the headline holds, the reason changes

*(Corrected out of the check. The findings file's summary said the app cannot tell whether a username is free "unless it either (a) becomes a Telegram user-account client on MTProto, or (b) runs its own manager bot." Option (a) is not the constraint Telegram publishes, and the findings file's own body draws the line in the right place while its summary draws it in the wrong one.)*

**What no bot can get is *claimability* — "may this be used for a new bot".** That question is answered only by `bots.checkUsername`, whose own result block reads **"Only users can use this method"**, and whose prose twin says "Only users can invoke `bots.checkUsername` and `bots.createBot`". **That is the real limit and it survived attack.**

**What a bot *can* get is an occupancy answer, and it costs much less than the findings file said.** Telegram publishes that a bot may authenticate to the protocol on its bot token alone — "instead of using the standard login code flow, simply provide the bot token generated by @botfather", and "After successful authorization, you will be able to use most MTProto API methods, just as any normal user" — with the import method marked **"Both users and bots can use this method"**. And `contacts.resolveUsername` is likewise marked usable by bots and publishes `USERNAME_NOT_OCCUPIED`, "The provided username is not occupied." **No person logs in.** The only extra requirement Telegram publishes is a developer registration obtained once by filling in a form.

**But occupancy is not availability.** A username can be neither occupied nor free to take: a separate published error says "The specified username can be purchased" elsewhere. And even the real check is a snapshot — Telegram publishes that the answer can go stale between checking and creating.

**The alternative flow is real and is the file's best-sourced claim.** A bot the app owns can hand the user a link that opens Telegram's own bot-creation window with the username and display name pre-filled but editable; the manager bot is then notified and can fetch the new bot's token. **The publisher describes the flow as described, not something narrower** — every step is in Telegram's own words, and the checker found a worked example confirming the parameter shapes. In this route the availability check runs inside Telegram's own client on the user's own account, which is exactly where the users-only method is permitted.

### Three things the reading agent missed, and the first one bears on the plan

1. **There is a published ceiling on how many bots one account may own.** `BOT_CREATE_LIMIT_EXCEEDED`, described as firing "if the current user already owns the maximum allowed number of owned bots", governed by two published settings — a default and a higher one for paid accounts — with clients told to offer a delete-a-bot link when a paid user hits it. The same error is listed on the creation method. **For an app whose walkthrough asks every user to make their own bot, a per-account ceiling that a paid tier raises is material.**
2. **The manager-bot route has a prerequisite the findings file's quotes begin after.** The manager bot must have management mode switched on in the bot-making chat's own interface first; at the protocol level that is a flag on the account, toggled through the same chat, with its own published error for when it is missing.
3. **There are protocol twins of the token methods**, including an export-token call with a rotation flag, access-setting getters and setters, and an update. There is also a published access restriction that limits a managed bot to its owner, optionally plus a small number of named others.

**One trim to note:** the findings file's argument that Telegram publishes nothing machine-readable about failures stops one sentence before the page's mention of an optional structured error field carrying a migration target and a retry delay. Neither helps with username availability, so the conclusion survives — but the blanket form does not.

**Not answered, and what would close it:** the display name's permitted characters, which **nothing published will close** — the honest options are to enforce only the ceiling and let Telegram reject the rest, or for a later stage to test a rename against a bot the Creator already owns and record it as an observation rather than a rule; whether a bot may end up with an empty fallback name, since creation says a minimum of one and renaming says zero-means-remove and Telegram does not reconcile them; username edge cases inside the allowed character set — leading digits or underscores, consecutive underscores, a trailing underscore before the suffix — none published, with one catch-all error whose triggers are not enumerated; whether "Latin characters" means exactly the ASCII letters, since one page says that and another writes a range that is not even clean; how large the set of reserved or purchasable-but-unoccupied usernames is; any Bot API error string, of which Telegram publishes none and explicitly reserves the right to change the numeric code; rename rate limits, not published; and the bot-making chat's own prompt wording, which would require talking to that bot and was forbidden — and which, for the username, turned out to be the weaker source anyway.

---

## 5. Italics in the three system web views (Fact 136)

**Answered for macOS by four independent routes plus a fifth from the check. Answered for Linux as "the platform has no answer". Open for Windows — and the check moved the open question to a different place than the findings file left it.**

| | resolves to | true italic? | how we know |
|---|---|---|---|
| **macOS** `system-ui` | the system UI face | **Yes.** A designed italic ships with the system | **Observed on this Mac**, five ways |
| **macOS** `ui-monospace` | the system monospaced face | **Yes.** A designed italic ships with the system | **Observed on this Mac**, five ways |
| **Windows** `system-ui` *(docs only)* | **Not established.** No publication names the face | **The risk:** if it is **Segoe UI Variable**, there is no italic to find — fifteen named instances, none italic, and only weight and optical-size axes. If it is **Segoe UI**, six italics ship | Read |
| **Windows** `ui-monospace` *(docs only)* | **Not established** | Both plausible faces ship italics | Read |
| **Linux** `system-ui` *(docs only)* | **the desktop's own configured font** — no fixed face | **Unanswerable in general.** It is whatever the distribution and the user configured | Read, from the engine's own source |
| **Linux** `ui-monospace` *(docs only)* | **Not established.** The mapping function has no branch for it | — | Read, with the stronger claim explicitly refused |

**The one-line summary for the builder: macOS gives real italics for both families. Windows may give a machine shear for `system-ui`, and no publication settles which. Linux gives whatever the desktop is configured with, and the monospaced family may resolve to nothing at all. Do not assume an italic exists.**

### The macOS answer, and why it is trustworthy

**The decisive test is turning synthesis off.** If the italic was synthesized, disabling synthesis must collapse it to upright; if a real face was selected, disabling synthesis must change nothing. This was run in the real system web view — the exact class the app will embed, not a browser and not a substitute — and **rebuilt from scratch by the citation check**, which reported differing pixel counts rather than rounded averages.

| family | with synthesis off, against upright | verdict |
|---|---|---|
| `system-ui` | **89,555 pixels differ** | **unchanged → real italic** |
| `ui-monospace` | **71,027 pixels differ** | **unchanged → real italic** |
| a known-real italic face *(positive control)* | 50,997 differ | real |
| a known-real oblique face *(positive control)* | 78,787 differ | real |
| a face with no italic *(negative control)* | **0 pixels differ** | **collapsed exactly to upright → synthesized** |
| the rounded system face, which ships no italic *(negative control)* | **0 pixels differ** | **collapsed exactly to upright → synthesized** |

**The negative controls collapsed *exactly*** — byte-for-byte identical to their upright renders. That is what makes the test trustworthy: **the property demonstrably reaches the engine and the engine demonstrably obeys it**, so "unchanged" is a finding and not an artefact. A test that cannot fail is not a test; two controls made this one fail on demand.

Four further routes agree and none disagreed: the shipped font files themselves; the operating system's font machinery naming the italic faces and reporting an **identity matrix**, which is the tell that the slant lives in the outlines rather than in a transformation; the advance width changing for `system-ui` between upright and italic, which only a different face can do; and the check's own geometric shear measurement (below).

### The shear angle — the conclusion is right and the arithmetic was wrong

*(Corrected out of the check, which measured it properly rather than argue.)*

**The two engines do not shear at the same angle.** The findings file said so three times, giving 14.04° as "the same slant WebKit uses" and calling the reading and the run agreement "exact".

- **WebKit's own constant is 14**, and a subpixel measurement of the two synthesized families gives **13.99°** — the constant, measured from the outside.
- **14.036° is Blink's number**, the one that falls out of its quarter-skew.

Practically indistinguishable, and **nothing in the advice changes** — but they are not the same number, and the findings file's own coarse measurement could not have told: it swept on a grid and landed on the nearest grid point.

**The finer measurement is a fifth independent route to the macOS headline, and it is worth having:** the system UI face measures **12.03°** and a known real italic **11.01°** — nowhere near fourteen. It also retires a worry the findings file raised, that a known real oblique face fits a low-residual shear: it measures **11.97°** and is plainly not a fourteen-degree shear.

**One row of the findings file's evidence proves nothing and is struck.** *(No statable negative, from the check.)* Asking for a specific oblique angle behaved identically to plain italic — but so did **every** angle swept, including zero degrees and minus thirty. **The engine discards the requested angle when it synthesizes**, so that row carries no information about the number fourteen. It remains fine as evidence for the separate and smaller point it was also used for: that italic and oblique are matched as synonyms, and that the specification's own escape hatch — a strict engine refusing to synthesize for italic — is not one you can rely on. **If you want no shear, you must say so yourself.**

### The Windows gap — relocated, not closed

**No publication names the face the Windows web view resolves the system family to.** The negative holds; the checker looked independently and did not find one either. **But the findings file's account of *why* is wrong, and its "what would close it" is aimed at the wrong target.**

*(Reversed out of the check. The findings file presented a source excerpt that does not exist in that form — two separately guarded declarations welded into one block with four intervening lines silently removed — and concluded from it that "the system font family is *set into* Blink by the embedder. For WebView2 the embedder is Microsoft Edge, which is not published. So Chromium's source cannot answer it.")*

**On Windows, that setter is never called.** In the engine's own Windows font code it is marked unreachable — reaching it is a bug — and the getter beside it returns **the Windows menu font**, pushed in through a different call the engine exposes to embedders, with the project's own note in the source saying as much in plain words.

**So the Windows system family tracks the Windows *menu font* as the operating system reports it — a system-wide setting, not a private choice by the browser vendor.** That changes the question from "what does the browser vendor choose?" to "what face does Windows report as the menu font?", which is a question about Windows, answerable against Microsoft's own window-metrics documentation, **and answerable without a web view at all**.

**The risk the findings file flags is still real** and the label "not established" is still right. Only the reasoning and the next step need redoing.

*(One thing in the findings file's defence, confirmed: the other Windows file it searched genuinely is a dead end — it handles the CSS system-font keywords, not the generic family.)*

### Small sizes

**A synthesized oblique is easily distinguishable from upright at small sizes**, by measurement and by eye — at eleven pixels the synthesized obliques differ from their upright by *more* than the real italics do, and the letter shapes read as unmistakably slanted. There is no threshold below which the shear disappears.

**The more useful question next door was answered less rigorously, and its answer must carry that.** *(No statable negative, from the check.)* The findings file concluded that a synthesized oblique is **hard to tell from a real italic** at small sizes, and therefore that Windows users are unlikely to notice a fake. That is one agent's eyes on one specimen with nothing said about what would have counted as failure, and it is stated more firmly than the file's own admission — that the perceptual threshold is not established — warrants. **Carry it as a plausible expectation, not as a finding.**

**And Microsoft separately advises against italic in Windows interfaces at all**, in its own words: "Bold and Italic styles are not part of the Windows type ramp. Use Semibold instead of Bold for emphasis. Italic is excluded because it can reduce readability and legibility, particularly for people with dyslexia."

*(Two supporting details the findings file did not use, from the check: the same typography page carries an **axes table** giving the variable face's complete axis list, which is firmer evidence than inferring it from prose; and a **non-Latin table** on which Microsoft still names plain Segoe UI, italics included, as the interface font for several scripts — which bears directly on which face a Windows web view might land on.)*

**Not answered, and what would close it:** which face the Windows web view resolves the system family to — now reframed as *which face Windows reports as the menu font*, and closable either by Microsoft's own window-metrics documentation or by running the synthesis-off test in a real Windows web view, ten minutes' work for anyone with a Windows computer, which this project does not have; whether the monospaced family resolves to anything on the Linux engine, deliberately left as *not established* because the mapping function has no branch for it and the callers were not traced *(a Linux desktop, plus tracing)*; which face the system family resolves to on Linux in general, which **has no answer and no publication could give one** — it is a property of the platform; whether the Windows variable face might gain an italic later, its published list being dated 2022; and the perceptual threshold, which needs human testing with more than one pair of eyes rather than a pixel count.

---

## 6. Keyboard shortcut conventions and accelerators (Fact 137)

**Answered on all three, and the Windows row of the table gained four entries the reading agent had recorded as unpublished.**

| Command | macOS (Apple) | Windows (Microsoft) | Linux (GNOME / KDE) |
|---|---|---|---|
| Cut / Copy / Paste | Command-X / -C / -V | Ctrl+X / Ctrl+C (or Ctrl+Insert) / Ctrl+V (or Shift+Insert) | Ctrl+X / Ctrl+C / Ctrl+V (both) |
| Close window | Command-W | Ctrl+W ("Close"); Alt+F4 ("Close the active item, or exit the active app") | Ctrl+W (both) |
| Quit | Command-Q | **no app-level convention published**; the system's is Alt+F4 | Ctrl+Q (both) |
| Minimise | Command-M | **no app-level convention.** The nearest published thing is **F11 — "Maximize or minimize the active window"** *(added out of the check; the findings file's flat "not published" glossed over it)* | **not published** by either |
| Preferences / Settings | Command-Comma | **not published** | GNOME Ctrl+, — KDE Ctrl+Shift+, |
| **Help** | Shift-Command-Question mark | **F1 — and it *is* published** *(reversal, below)* | F1 (both) |
| Menu bar / mnemonics | — | **Alt + underlined letter**, **F10** to activate the menu bar, **Shift+F10** for a context menu *(all three added out of the check)* | GNOME F10 opens the primary or secondary menu |

### The Windows Help reversal

*(Reversed by the citation check. The findings file recorded, as one of its open items: "A Microsoft publication for F1 = Help. Searched; the current Windows Support shortcut list does not include F1, and the Windows-apps keyboard-accelerators design page does not either.")*

**Both halves of that search were true and were reproduced. Microsoft publishes it anyway, in its own Win32 reference**, on the `WM_HELP` message page:

> "Indicates that the user pressed the F1 key. If a menu is active when F1 is pressed, WM_HELP is sent to the window associated with the menu; otherwise, WM_HELP is sent to the window that has the keyboard focus."

— read 2026-08-25 at `https://learn.microsoft.com/en-us/windows/win32/shell/wm-help`. Minimum supported client Windows XP, desktop apps only.

That is Microsoft defining F1 as the application's Help key, routed to the application's own window, with a menu-aware rule. It is a reference page rather than a style guide, which is squarely within what this project accepts. **The open item is closed.**

**The failure that produced it is worth naming, because it will recur.** The findings file went looking, hit a question-and-answer site, **correctly refused it** — and then stopped, instead of asking where else the same vendor might publish the answer. Refusing the forum was right. Treating the forum as a signal that the vendor probably publishes it somewhere, and going to find where, is the step that was missed.

**And the three menu conventions added above matter more than usual for this app**, because on Windows and Linux it draws its five menus itself behind a title-bar button in the web view. Mnemonics, the menu-bar key and the context-menu key are behaviours it will have to imitate by hand or knowingly drop.

### What each system reserves

- **macOS — a table, not a prohibition.** Apple lists standard shortcuts, several of which are plainly the system's own — the search field, the app switcher, the force-quit panel, screen capture, the screen reader, keyboard access, input sources. Apple's stated rule around them is guidance: **"In general, don't repurpose standard keyboard shortcuts for custom actions."** **Apple publishes no list under the word "reserved"** — confirmed; the word does not appear on the page.
- **Windows — exactly two hard reservations, and no general list.** Both live in the hot-key registration reference: **"Keyboard shortcuts that involve the WINDOWS key are reserved for use by the operating system"** and **"The F12 key is reserved for use by the debugger at all times."** The design-guidance page a reader would expect to carry a reserved list **does not contain the word "reserved" at all** — confirmed by a search of its stripped text, and its thirty-row table has no Quit, Exit, Minimize, Preferences, Settings or Help row. What it says instead is that a **control can win**: with a text box focused, the copy accelerator only copies the selected text and app-defined accelerators are ignored.
- **Linux — GNOME publishes a list, freedesktop publishes one rule.** GNOME's is headed "Legacy System Reserved Shortcuts" and introduced as "The following shortcuts should be avoided by apps, since they are used by the system", covering the window-switching, window-menu and close combinations and the interrupt combination. freedesktop's own draft adds exactly one required rule, and the section really does contain only that one sentence: **"Applications should never use the Meta/Super key."** KDE says the same in its own words.
- **freedesktop publishes nothing that lists conventions.** Its index has 33 entries — verified independently, in order, character for character — of which only two touch keyboards and **both are marked DRAFT**: one defines a *notation* for writing a shortcut down, the other contains that single rule. Neither mentions cut, copy, paste, close, minimise, quit, preferences or help.

**Practical reading: none of the eight commands this app needs collides with anything any of the three vendors reserves.**

### Does the shell put an accelerator beside a menu item, and does it need a native menu bar?

**Yes it exposes one, and yes it only works with a native menu bar. The substance is confirmed and is on firmer ground than the findings file put it on — but the findings file's stated proof was not the proof it claimed, and one leg of it must not be reused.**

*(Corrected out of the check, in two ways.)*

- **"Confirmed twice over, in the shell's prose and in its source" is not what happened.** The prose nowhere says an accelerator requires a native menu bar. The prose establishes only that the accelerator is a field on a menu item; the source establishes that the field's only consumer is native menu construction. **Two different propositions, each proved once.**
- **The runtime index the findings file leaned on is not an inventory, and using it as evidence of absence is unsound.** The released version demonstrably ships whole runtime areas that list omits — notifications, screens, drag-and-drop, in both the code and the documentation — and the same findings file's own headline for the next fact is that notifications exist. **Discard that argument; keep the conclusion.**

**The argument that actually works is a whole-documentation search at the released version**, which the checker ran: the words *shortcut*, *accelerator*, *hotkey* and *keybinding* occur in exactly two files across the entire documentation set — the menus reference, and one community showcase entry about somebody else's application. **There is no keyboard page, no shortcut page and no hotkey facility.** In the source, the key type has exactly one consumer, and a menu is accepted at exactly three public entry points, all three of them the application menu.

**So on Windows and Linux, where this app draws its menus inside the web view, the accelerator beside each item is a label the app draws, and catching the keystroke is the web view's job — a key handler in the page. The shell publishes no route for it.**

**On macOS the dependency runs the other way**, and the shell says so outright: the native edit menu is what makes the ordinary clipboard keys work at all. **Dropping it costs the web view its cut, copy and paste keys.**

**What the accelerators can express:** any single character except the plus sign, which is spelled out; a named set of keys; and four modifiers. **There is no Meta/Super/Windows-key modifier** — and that is not merely an omission: the checker found it **deliberately commented out** in the source, both the constant and its map entry. Which happens to line up with what freedesktop and KDE tell applications not to use anyway.

***One quotation is inexact and is wrong against both of its sources.*** The findings file compressed the function-key range to "F1 through F39". The documentation table lists F1–F25 and F30–F39, **skipping four**; the parser that actually decides accepts **F1 through F35**, so four keys the documentation advertises will not parse. Low stakes for an app needing at most F1, but the compressed range is true of neither source.

**And the pre-built menu roles are macOS-only** — stated twice by the shell in its own words.

**Not answered, and what would close it:** whether cut, copy and paste work in the web view on **Windows and Linux** without a native menu. On macOS the shell states the dependency outright; for the other two it publishes nothing either way, and the web views presumably handle the clipboard keys in editable fields themselves — **but that is an assumption, not a read fact**, and it matters precisely because this app draws its own edit menu *(a statement from the shell or from either web view's own publication, or a run on either machine — neither of which this project has)*. Also open: whether the next major version of the shell changes the accelerator picture, deliberately not read since the chosen version is the current one. *(One place a Linux free-standing-shortcut question would actually go, which the findings file did not mention: freedesktop publishes a global-shortcuts **portal** interface — a registration mechanism, not a conventions list, so it leaves the table above standing.)*

*(Two smaller notes from the check, neither disturbing anything: the shell does ship a context-menu facility, driven by markup attributes and taking no menu structure and no accelerator — so "a menu is only ever the application menu" reads more safely as "the **Go menu structure** is only ever set as the application menu". And the checker confirmed the shell's own site refuses automated fetches, which is why its documentation was read from the source the site is built from.)*

---

## 7. How a notification activation gets back (Fact 138)

**Answered on all three for a *running* application. Open on Windows for a cold start. Undefined by publication on Linux for an exited one.**

| | macOS | Windows *(documentation only)* | Linux *(documentation only)* |
|---|---|---|---|
| Mechanism | The system calls a notification-centre delegate method with a response object | The notification platform activates a registered component and calls its activate method | The notification server emits an action-invoked signal on the session bus to the client that called it |
| What it requires of the app | A delegate assigned **before the app finishes launching**. **Apple publishes no bundle, signing or notarization requirement** — a negative checked across **nine** of Apple's own pages with zero matches. The shell's own code nevertheless refuses to start without a bundle identifier | An application identifier, a **Start-menu shortcut** carrying it and an activator identifier, and a registered local component server | A live session-bus connection, and a server advertising the actions capability. **Nothing else** — the sending application's name "can be blank", and the only identity hint is optional and cosmetic |
| Plain unelevated binary? | Bundled in practice, which the shell builds | **The registration is per-user** — Microsoft's own line writes under the current user, so no elevation. **The Start-menu shortcut is what a bare download lacks**; an unelevated app can create one in its own per-user folder, which Microsoft's own sample does, but it is an installed artefact the app must make | **Yes.** A session-bus connection is all it takes |
| Reaches a running app? | Yes | Yes | Yes |
| Reaches an app that has exited? | **Yes** — Apple documents it: "the system launches your app in the background and calls the delegate's … method" | **Yes** by Microsoft's mechanism, and it is the stated reason the component server exists — **but see the open item; the shell's own path to it is unproven** | **Not defined.** The specification describes a signal to a connected client and nothing else |
| Vendor documents the difference? | Yes | Yes, explicitly and at length | **No** — and that negative is airtight: nine chapters of the specification searched for every relevant word, with zero hits outside the one unrelated sense |

### macOS

Apple states the route three times over, and the one hard timing requirement is unambiguous: **the delegate must be assigned before the app finishes launching**, because "Assigning a delegate after the system calls these methods might cause you to miss incoming notifications."

**A bare click on the notification body is itself an action**, and Apple has a constant for it whose "delivery … doesn't require any special configuration of notification categories."

**Apple's silence on identity is a real negative, strengthened by the check.** The findings file searched four of Apple's pages; the checker searched **nine**, for bundle identifier, code signing, notarization, entitlements and property lists — **zero matches on all nine.** The bundle requirement in practice comes from the shell's own code, not from Apple's publications.

### Windows

**Microsoft's own reasoning, quoted, is why the whole mechanism exists:** notifications "can stay around for several days before they are seen by the user and processed. Because of this, you can't assume that your app will be running when the user responds to a notification. In order to get around this, you will need to create a local Component Object Model (COM) server that can be invoked when users interact with the toast."

Three requirements, all published: **a Start-menu shortcut** ("Without a valid shortcut installed in the Start screen or in All Programs, you cannot raise a toast notification from a desktop app"), **an application identifier on that shortcut**, and **an activator identifier with a component server registered for it** — Microsoft's own registration line writing **under the current user**, which is why no elevation is involved.

**Microsoft's current guidance for its own framework says something simpler** — that for unpackaged apps a single registration call "automatically sets up the COM server registration that allows Windows to launch your app when a notification is clicked", with no manual configuration — **but that is its own framework doing it, and this app's language does not use it.** The same page adds that a console app "must remain running to receive activation callbacks when notifications are clicked."

### Linux

- **The actions capability is advertised, and may be absent.** "Even if this cap is missing, actions may still be specified by the client, however the server is free to ignore them."
- **The specification twice warns against assuming a response.** "This functionality may not be implemented by the notification server, conforming clients should check if it is available before using it", and, attached to the signal itself, "Clients should not assume the server will generate this signal. Some servers may not support user interaction at all."
- **A plain click on the body is an action too, and has a reserved name** — "The default action (usually invoked by clicking the notification) should have a key named `"default"`."
- **There is a companion signal carrying a token for raising your own window**, with the same do-not-assume caveat attached.
- **What happens after the application exits is simply not in the specification.** The word *activation* appears only in the raise-a-window sense, never in the sense of launching a program. **Assume a notification click only ever reaches a running application on Linux.**

### The shell does expose all of it, in the released version

**This is the surprise of the batch and it is worth stating loudly**, because notifications are a headline feature of the *next* major version and it would be reasonable to expect the current one to have nothing. It has a full notifications runtime including the activation callback, present at both named released versions in the runtime package and in all three platform frontends — **verified independently by the checker in the project's own repository, at both tags**.

A bare body click is normalised across all three systems to one identifier. The response carries the notification's identity, the action, a category, the text of the notification, any typed reply, and arbitrary carried data, with per-field platform support marked — and two of those fields are not available everywhere.

How the shell meets each platform's requirements, read from its own source at the released version: on **macOS** it refuses to initialise without a bundle identifier, installs a delegate, and rewrites Apple's default-action identifier to its own; on **Windows** it does the unpackaged registration itself at startup, **entirely under the current user with no elevation and no installer**, writing the component server entry and storing a generated activator identifier under an application-identifier key, with the application identifier being simply the executable's filename; on **Linux** it connects to the session bus, matches the action-invoked and closed signals, and **always prepends the default action pair**, so a plain body click comes back as an action exactly as the specification's reserved name describes.

**Two of the shell's own pages contradict each other** and both were confirmed to say what they say: the guide says notifications are unsupported in the browser-side runtime; the reference publishes browser-side signatures for nearly every call while carving out the activation callback and telling you to receive it in the host language and re-emit it as an event. **That last pattern works regardless of which page is right.**

**Not answered, and what would close it:**

1. **Whether a cold-start activation reaches the shell's callback on Windows.** Microsoft's mechanism launches the executable when the app has exited, and the underlying library's own architecture note says the activation executable is "to be invoked when our application is not running". But the shell registers its class factory only while running, and **its Windows notification file contains no command-line parsing at all** — the whole file was read by both agents and the checker confirmed the only such parsing anywhere in the Windows frontend is in an unrelated single-instance file. **So the path from "user clicks a toast while the app is closed" to "the callback fires" is not established by any publication.** *What would close it:* a statement from the shell either way, or a run on a Windows machine. **Until then, design for activation reaching a running app and treat cold start as unproven.**
2. **Whether the registry-only registration the shell performs is a route Microsoft supports.** Microsoft publishes two routes and this is neither. **Microsoft publishes no documentation of that key layout at any live address** — checked across the application-identifier reference, the activator-property stub, both framework pages, Microsoft's own search, and Microsoft's own documentation source repository. ***One caveat the runner must hold:*** a Microsoft page on sending a local toast from other types of unpackaged app **once existed and now returns 404**, its older address redirects to an unrelated page, and its source file is gone from the repository. The checker could not read it and refused to infer its contents from a search snippet. **So the honest form of this negative is "Microsoft publishes no such documentation at any live address today", not "Microsoft never documented this."** And the application identifier being the bare executable filename is a collision risk nobody has ruled out.
3. **Whether macOS activation needs a *signed* or *notarised* app, as distinct from a bundled one.** Apple publishes no such requirement on nine pages. Whether an unsigned locally built bundle receives the callbacks **is not stated by Apple** — and **this project has a Mac, so this one is cheaply closable later.**
4. **Whether the Linux route survives the app exiting, and whether it needs a desktop-entry file.** Nothing in the specification will close it — this is per-server behaviour. The shell sends no desktop-entry hint. **The honest architectural answer is that a click reaches a running app and nothing else.**
5. **Whether the shell's browser-side notification bindings exist**, its two pages being in flat contradiction. *What would close it:* reading the browser-side notification file, which **exists in the repository — the checker confirmed it — and was not read by either agent.**

---

## 8. Telling two paths to be one folder (Fact 140)

**Answered on all three, and the answer is the same on all three: compare identity numbers from the file system, not text.**

| | the call | what you compare | when the folder does not exist |
|---|---|---|---|
| **macOS** | `stat` / `fstat` | device and file serial number | fails |
| | `realpath` | the canonical text | fails — "All components of `file_name` must exist" |
| | Apple's file-identity value | an opaque object | throws |
| **Windows** *(docs only)* | file information from an open handle | volume serial number plus file index | needs an open handle, so there is nothing to call it on |
| | the extended file-identity structure | volume serial number plus a 128-bit identifier | same |
| | the final-path call | the final path text | same |
| | the full-path call | the merged text | **succeeds** — "does not verify that the resulting path and file name are valid" |
| **Linux** *(docs only)* | `stat` / `statx` | device and inode, plus a mount identifier | fails |
| | `realpath` | the canonical text | fails |
| | the name-to-handle call | an opaque handle, designed for exactly this comparison | fails |

### The four rules that fall out

1. **Where both folders exist: compare identity numbers, live, at the moment you need the answer.**
2. **Never store an identity as a durable key.** Both POSIX and Microsoft say outright that these values are reused over time. The newest POSIX wording is explicit: "Over time, these file identities can be reused for different files" — an inode value after the last link is unlinked and the space freed, a device value after a filesystem is detached and another attached. Microsoft's kernel-side wording is blunter still: identifiers "are guaranteed to be unique only within a static file system."
3. **Where a folder does not exist yet — a path the user typed — no system will identify it.** You must fall back to text, and **say so in the interface rather than pretending**.
4. **Never rely on canonicalised text alone.** It is wrong in documented ways on all three systems, and one run on this Mac found a case where Apple's own recommended identity call disagrees with the plainest one.

### Why text is not enough, per system

- **macOS: no.** Canonicalisation resolves symbolic links, so it collapses that case correctly — but **it cannot see hard links**, and it fails outright on a path that does not exist.
- **Windows: no**, and Microsoft's own documentation makes the case. The full-path call is pure string arithmetic that verifies nothing. Two different texts name one folder through a **mounted folder** — Microsoft's own example: "you can then use either path … to access the volume" — a junction, a symbolic link, or a short name. And Microsoft warns that a volume "can have more than one volume GUID path", calling the term *unique volume name* "misleading".
- **Linux: no.** A **bind mount** makes "the same contents … accessible in two places" with two entirely unrelated canonical texts and **no marker to detect it** — "There is nowhere stored information that the filesystem has been attached by a 'bind' operation." Text cannot possibly see through that. And case-folding on the common filesystem is **per directory within one filesystem**, so no single rule about text comparison holds even on one machine. Windows has the same per-directory shape from the other side: case-insensitive by default, with an attribute that makes a directory case-sensitive, inherited by new directories beneath it.

### What the run on this Mac established

*(All observed on this Mac; every row reproduced by the citation check on fresh folders with fresh inodes.)*

- **Seven different spellings of one folder** — through a symbolic link, through a parent reference, in two wrong cases, with a trailing slash, with a doubled separator — **all gave one identity**, and canonicalisation gave one text.
- **Negative control:** two genuinely different folders compared different by both routes. **So the seven "same" results mean something.**
- **A hard link is where text is wrong and identity is right:** two different canonical texts, one object; identity says same, text says different. *(Demonstrated on a file, because macOS refuses to hard-link a directory — which matches Microsoft's parallel statement that hard links "can't reference directories, only files". The folder-level equivalent is the bind mount, which needs a Linux desktop.)*
- **Canonicalisation on macOS normalises the case back to the true on-disk spelling** — which the description of that call does not promise, and which the same page's variants section explains as a Darwin implementation detail. **Do not carry this behaviour to another system.**
- **A language runtime's "realpath" may not be the system call.** One invented an answer for a folder that does not exist, where the real call refuses. **A real trap, correctly flagged.**

### The disagreement that will bite

**Apple's own file-identity value disagrees with `stat` through a symbolic link, and Apple does not document its side of it.** *(Observed; reproduced exactly by the check, byte layout and all — including that the volume half read the same sixteen characters on a different day with a different set of files, which is what a volume identifier ought to do.)*

| path | Apple's identity value, against the real folder's |
|---|---|
| the folder itself | reference |
| a **symbolic link** to it | **not equal** — while `stat` says **same folder** |
| through a parent reference | equal |
| the wrong case | equal |
| an unrelated folder | not equal |
| a path that does not exist | **throws** |
| the symbolic link **after resolving symlinks** | **equal** |

The identifier decodes as an inode followed by a volume identifier, matching Apple's own header text — **but the resource-value lookup does not follow a trailing symbolic link, so it identifies the link, not the target.**

**This is the same trap Windows documents explicitly and Apple does not:** on Windows, whether your handle identifies the link or the target is decided **at open time** by a flag, not by the identity call, and the identity call inherits that choice. **For the builder: resolve symbolic links before asking, on both systems.**

### The published limits, per vendor

- **Microsoft.** *(Two corrections the findings file made to its own brief, both confirmed exactly by the check.)* Microsoft does **not** say file identifiers are "not supported on all filesystems" on the page in question — it says **"Support for file IDs is file system-specific."** And the caveat about the newer filesystem is **not** in that page's remarks: it sits in one member's description, saying the 64-bit identifier "is not guaranteed to be unique" there and pointing at the 128-bit structure. That 128-bit structure's own page **has no remarks section at all** and mentions neither filesystem — confirmed, including the negative half. *(One qualification the check adds: the brief's original phrasing was not invented — Microsoft publishes nearly that sentence in its file-sharing protocol glossary, which the same findings file goes on to cite twenty lines later. Nobody should re-litigate this.)* Microsoft also names identifiers that change: on the older filesystem an identifier can move when a directory is defragmented, and **renaming a file can change it** if the new name is longer.
- **The protocol specification publishes two "no usable identity" sentinels** — a filesystem without support must set the field to zero, and a file for which no unique identifier can be established must set it to all ones, both to be ignored. And its uniqueness language is **should**, not must.
- **Network filesystems.** Microsoft: the information call "may fail, return partial information, or full information" depending on the server; normalized-path queries are **not supported** by the file-sharing protocol; volume GUID paths "are not created for network shares". *(Added out of the check: the same information page carries a **support table** on which the current file-sharing protocol, its failover and scale-out variants, the cluster filesystem and the newer filesystem are **all "Yes"** — which pairs informatively with the open-by-identifier call's table, on which the same three protocol rows are **all "No"**. **You can ask for identity over the network; you just cannot open by it.**)* Linux's own client documentation is explicit both ways: using the server's inode numbers makes hard links spottable but "the server does not guarantee that the inode numbers are unique if multiple server side mounts are exported under a single share", and generating them client-side means "These inode numbers will vary after unmount or reboot". And handle translation is documented as unsupported on several filesystem kinds including network ones.
- **Two genuine documentation gaps, both attacked and both real.** The network-filesystem page for the other common protocol **says nothing about inode stability** — the checker confirmed the word *inode* appears on it **zero times**. And **Apple publishes nothing found on identity over network filesystems**, across five sources read on this machine; the nearest thing is a read-only-volume note about a different attribute. *(The checker did not independently sweep for the Apple gap and records it as not-independently-verified rather than confirmed.)*
- **Overlay filesystems are much less hazardous here than they look**, and this is the non-obvious claim the checker expected to break and did not. The instability warnings about device and inode values changing are **scoped to non-directory objects**; the kernel's own properties table shows a uniform device value as yes for directories in **every** configuration listed — four rows, checked. **Since this fact is about folders, that warning does not land.**

***One quotation is trimmed at exactly the wrong point and one provenance claim is false.*** *(Both from the check.)* An identity attribute is quoted without its deprecation: `getattrlist(2)` continues, immediately after the quoted text, that **`ATTR_CMN_OBJID` is deprecated starting with macOS 10.13, iOS 11.0, watchOS 4.0 and tvOS 11.0, and `ATTR_CMN_EXT_LINK_ID` should be used in its place**, and separately that "no relation between the two attributes should ever be implied" against `ATTR_CMN_FILEID`, the plain file-identifier attribute. **For a fact whose subject is identity, a deprecated identity attribute presented without its deprecation, and its named replacement left unmentioned, is a real omission — carry the replacement: `ATTR_CMN_EXT_LINK_ID`, not `ATTR_CMN_OBJID`.** *(The two names were written in here on 2026-08-31; this paragraph said only "a named replacement" and named nothing, so a builder could not act on it.)* And the blanket sentence that all the macOS manual pages "were read on this machine … each footer reads `macOS 26.6`" is **false for two of the four** — two carry much older footers. The pages do ship here and every quotation from them is verbatim; only the provenance sentence is wrong.

**Not answered, and what would close it:** **everything about Windows and Linux behaviour, which was read and never run** *(a Windows computer and a Linux desktop, running the equivalent of the Mac run: two paths to one folder through a symbolic link, a junction or bind mount, a mounted folder, and a differently-cased spelling, comparing identity against text)*; **whether the device value is preserved across a Linux bind mount**, documented nowhere in three pages read and only inferable — **and it matters, because the bind mount is the one case where two folder paths are genuinely one folder with no text relationship at all** *(a bind mount and one call on both paths)*; **Apple's position on identity over network filesystems**, a gap in Apple's documentation rather than in the reading *(a statement from Apple, or a run against a network share — free, and possible later on this Mac)*; the other network protocol's position on inode stability, where the page is silent and was **not** filled from memory; behaviour on a **case-sensitive** volume, every run above having been on a case-insensitive one — the case-normalising result would look different *(create such a volume and repeat; free on this Mac but it touches disk configuration, so it was not done)*; a hard-linked **folder** on macOS, which the system refuses; and whether the flag that errors on a multiply-linked object is a useful shortcut, documented but never exercised *(a short run, free on this machine)*.

---

## 9. What this batch leaves owed

**Two machines close most of it, and this project has neither.**

**A Windows computer** closes: what End task does to a windowed application; whether any event-log entry names a process killed outright and whether an unelevated app can read it; whether the app's italic text is a designed face or a machine shear; whether the shell's cold-start notification path works; whether the shell's registry-only registration works at all; whether the clipboard keys work in the web view without a native menu; and every identity behaviour in Fact 140 that was read rather than run. **Nothing schedules one.**

**A Linux desktop computer** closes: whether the monospaced generic resolves to anything in that web view; whether the device value survives a bind mount, which is the one case where two folder paths are one folder with no text relationship; the clipboard-keys question on that side; and every Fact 140 identity behaviour on that system.

**Four things want no machine at all and are still open.** A Microsoft page on whether the flush call defeats a drive's hardware cache. A Microsoft page documenting the registry layout the shell writes — **and the honest form of that negative is "no live address today", because a relevant page was retired and could not be read.** Which face Windows reports as its menu font, which the check relocated from an unanswerable question about a browser vendor into an answerable one about Windows. And Telegram's permitted characters for a display name, which **nothing published will ever close**.

**Three things are cheap on the Mac this project has**, and are named so they are not forgotten: force-quitting the test application from the real panel by hand, which would turn the batch's most load-bearing macOS finding from a strong proxy into a direct observation; whether an unsigned locally built bundle still receives notification callbacks; and a case-sensitive volume for the identity runs.

**One thing wants a machine that is not the Creator's**: a controlled memory-pressure run. It must not be done on the Creator's daily driver.
