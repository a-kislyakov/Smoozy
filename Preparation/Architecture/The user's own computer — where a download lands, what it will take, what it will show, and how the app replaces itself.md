# The user's own computer — where a download lands, what it will take, what it will show, and how the app replaces itself

*The answer of record for Facts 16, 22, 23, 24, 73, 74 and 96, read on 2026-08-18. Every claim here is either quoted from the thing's own documentation or observed on the Creator's Mac, and each carries its label in the trail. Nothing was rented and nothing was paid for.*

**Where the evidence is.** `Audits/Architecture/The user's own computer/` holds five findings files and five citation checks, one checker per findings file, each fetching every cited address itself and re-running every observation. That folder is the raw material; this document is what a later architecture document reads. **Do not open the trail to answer a question this file answers.**

**The one limit that runs through everything below, and it is not closed here.** **No Windows machine has run any of it**, and there is **no Linux desktop computer** either — the Linux machines this project rents are headless servers, not a user's own computer. A Windows machine can be rented and four have been, so the Windows half is **untried rather than untestable**; but **every Windows machine this project has read reported a Server edition**, not the desktop edition its image was named for, so a desktop-only question is still out of reach of what has been rented. So every Windows claim and every Linux claim below is **documentation alone**, and every observation was made on **macOS 26.5.2, build 25F84, arm64**. Row 282 already requires every answer touching the user's own computer to be written three times; this file supplies two of the three from documentation and one from observation, and says so at each point.

---

## 1. Where a download lands (Fact 22)

**The three systems are three different mechanisms, not three spellings of one.** A route written once against whichever system the writer had in mind is wrong on the other two.

| | macOS | Windows | Linux |
|---|---|---|---|
| The route | `FileManager.url(for: .downloadsDirectory, in: .userDomainMask, appropriateFor: nil, create:)` | `SHGetKnownFolderPath(FOLDERID_Downloads, 0, NULL, &p)`, then `CoTaskMemFree` | read `XDG_DOWNLOAD_DIR` from `${XDG_CONFIG_HOME:-$HOME/.config}/user-dirs.dirs` |
| Governed by | Apple's Foundation reference | Microsoft's Known Folders | **no specification at all** — the `xdg-user-dirs` tool and its manual pages |
| Relocatable by the user | not documented either way | **yes** — `SHSetKnownFolderPath`, User privileges for this per-user folder | **yes** — edit the file, or `xdg-user-dirs-update --set` |
| Does the route follow a relocation | nothing documented to follow; observation suggests the path is **composed** rather than looked up | **yes by default**; `KF_FLAG_DEFAULT_PATH` turns that off | yes by construction; **GLib caches and will not** |
| On-disk name localised | **no** — display name only, in Apple's own words | **no** — `KNOWNFOLDER_DEFINITION`'s `pszRelativePath` "refers to the physical file system path, and is not localized", while `pszLocalizedName` is a resource reference stored in the folder's `Desktop.ini`; `SHGetLocalizedName` hands back a resource module and an ID and never a path *(corrected 2026-08-22: this cell read "not documented; `SHGetLocalizedName` implies display-only")* | **yes** — genuinely translated, and the encoding is configurable |
| Can return nothing | the array may be empty | **yes** — `E_FAIL` or `E_INVALIDARG`, and failure with no path where verification fails | **yes** through GLib (`NULL`); the tool's own helper returns `$HOME` instead |
| If it does not exist | pass `create: true`, or create it | pass `KF_FLAG_CREATE`, or `KF_FLAG_DONT_VERIFY` to get the path anyway | the update tool **resets a missing downloads folder to `$HOME` at next login** |
| Permission needed | none documented unsandboxed; `com.apple.security.files.downloads.read-write` sandboxed | none for the lookup | none documented |

**Five things a builder must carry from that table.**

- **Never pass the Windows default-path flag.** Windows is the one system with a real relocation API whose default read follows the relocation. Asking for the default path defeats the user's own choice.
- **Linux has no downloads specification.** The XDG Base Directory Specification contains the string `download` **zero times**, and freedesktop's specification index carries no `user-dirs` entry. What governs it is a **tool**, not a standard.
- **The documented Linux fallback is `$HOME`, not `$HOME/Downloads`.** A Smoozy that assumes the latter proposes a folder the system does not use.
- **Linux is the only system where the on-disk name is genuinely translated.** A path assembled from the English word is wrong there and only there.
- **Apple forbids silent defaulting**, in its own words: an app "must not write files to the preceding directories unless explicitly directed to do so by the user". Proposing the folder is fine; writing into it unasked is not. That bears directly on whatever Design settles about the destination.

**One trap in the managed route:** `.NET`'s `Environment.SpecialFolder` has **no Downloads member**, and Microsoft's own Known Folder table records `CSIDL Equivalent: None` for it. There is no older equivalent to fall back on.

**Not answered, and what would close it:** whether the macOS route looks the location up or composes it (a Foundation call on a Mac, which needs a compiled binary). *(Corrected 2026-08-22, out of the first sanity sweep's verification round: this line also carried "whether Windows localises the on-disk name (a Windows computer)". Microsoft documents it and it wanted a page rather than a computer — `KNOWNFOLDER_DEFINITION` and the `FOLDERID_Downloads` entry, both read 2026-08-20; the answer is in the table above.)*

---

## 2. What a failed write reports, and what cannot be told apart (Fact 23)

**Judged at the moment Smoozy would actually meet each cause**, which is the only way this question means anything. All macOS values below are **observed on the Mac**; the Windows and Linux values are documentation.

### At `open()` — creating the frame file, on macOS

| Cause | What comes back |
|---|---|
| Full volume | `ENOSPC` (28) |
| Permission refused | `EACCES` (13) |
| Directory removed | `ENOENT` (2) |
| Volume detached | `ENOENT` (2) **where the mount point went with the volume**; **no error at all** where the mount point survived |

**A removed directory and a detached volume collapse into one value**, in the case where the mount point goes too. Smoozy cannot separate them from the error alone and **must not claim to**.

**And there is a third outcome, worse than a collapse.** Where the mount-point directory **survives** the detach, `open()` on the detached volume **succeeds**, and the frame lands on the underlying volume while Smoozy believes it wrote to the user's drive. **No error distinguishes that from a healthy write, anywhere.** Both conditions were reproduced twice each, and neither is adjudicated as the normal case — which mount-point behaviour a real user's drive gives is unobserved.

**This kills a test the findings file had proposed.** Telling the two causes apart by whether the mount point survived does not work, because in half the runs it survived. Struck rather than carried.

### At `write()` — mid-frame, on a descriptor already open, on macOS

| Cause | What comes back |
|---|---|
| Full volume | `ENOSPC` (28) |
| Permission refused | cannot arise — refusal happens at `open()` |
| Directory removed | **no error at all** — write, `fsync` and `close` all succeed |
| Volume detached | `EIO` (5) at `write`; `fsync` and `close` then return 0 |

**The third row is the dangerous finding in this whole batch.** A directory deleted underneath an open descriptor reports **success at every call** on macOS while the bytes go to an unlinked inode that vanishes at close. **Smoozy will report a frame written that does not exist.** The only defence is to **re-check the destination by path after closing** rather than trusting any return value. Reproduced line for line by the citation check.

**The detached-volume error is reported once and then lost** — `EIO` at the write, then success from `fsync` and `close`. An arrangement that only inspects the last call sees a clean finish.

### Two signals that are not error values

**Free space separates a full volume from a refusing one**, which confirms the direction of this project's existing rented-Linux finding: about 61 GB available on the volume that refused for permissions, about 1 MB on the full one.

**But it does not tell you a refusal has ended, and it never reads zero.** On the genuinely full volume the reading was **1,068 KB, not zero** — so **any check written as *available equals zero* will never fire**. Compare the reading against the volume's size. And only a write attempt establishes that a refusal is over.

### What is not established

- **Windows' detached-volume error is not established.** Six candidate codes exist and Microsoft's `WriteFile` page names none. **Linux's manual pages name none either.**
- **`ERROR_NOT_ENOUGH_QUOTA` (1816) is not a disk quota** on `WriteFile` — it is the process's working set. Anything reading it as a disk condition is wrong.
- **The quota error number differs between the two systems Smoozy can be observed on**: `EDQUOT` is **69 on macOS and 122 on Linux**, each from its own header.
- **A write to the sealed macOS system volume returned `EPERM` (1), not the `EROFS` (30) Apple's own `open(2)` documents.** Observed, unadjudicated, and recorded because a builder reading the manual page alone would get it wrong.
- **A full volume "leaves no file behind" only where it was already full at open time.** Where it filled *during* the write, a truncated file remained on every run of the byte loop, and an empty file on every run of the copy tool. The blanket form of that claim is not true.
- Quota behaviour, blocking writes, and the permission-refused case on a directory Smoozy itself created are documentation-only, each for a stated reason.

---

## 3. Which names the user's computer will take (Facts 16 and 24)

Frames and folders are made on a rented Linux machine and fetched to the user's own computer. **Linux permits names the other two will not take**, so this is checked before a transfer rather than during one.

### The length unit differs, and it is two units rather than three

- **Linux — 255 bytes** of UTF-8, from ext4's own kernel documentation and POSIX's own definition. **This confirms the project's existing finding**: a length check written in characters passes a name Linux refuses.
- **Windows — 255 UTF-16 code units**, counted as `WCHAR`s.
- **macOS — 255 UTF-16 code units**, **measured, not documented**. Windows and macOS are the same unit; **Linux stands alone**, and its byte limit is the binding one.

**Apple's own two manual pages contradict each other on this and both are wrong**: `pathconf(2)` says bytes, `getattrlist(2)` says characters. A 765-byte name was accepted and a 128-character name refused. `getattrlist(2)`'s derived ceiling of 766 is a correct bound for 255 UTF-16 code units, which is how the contradiction resolves.

### Getting the name as stored — the core of Fact 16

**On macOS a case or normalisation near-miss is detectable in one call.** `getattrlist(ATTR_CMN_NAME)`, `fcntl(F_GETPATH)`, and the C `realpath(3)` all return the name **as stored on disk**; comparing those bytes against the bytes asked for is the whole test. Apple's own `realpath(3)` documents `F_GETPATH` as the mechanism.

**The trap is not a particular call — it is any canonicaliser that manipulates strings.** The first run reported that `realpath` echoes back whatever capitalisation it was given; that was an artefact of one language's own pure-string implementation, which never calls the system routine. The citation check caught it. **A resolver that rejoins the strings it was handed cannot detect a near-miss**, whatever it is called.

**APFS stores names byte for byte as given** and normalises only its lookup hash, so names round-trip. **HFS+ rewrites them** to a decomposed form — documentation plus inference, never measured here.

**macOS folding is Unicode *full* folding**, tested against the folding table rather than asserted: it predicted all fifteen observed pairs with no mismatch, where simple folding mispredicts three. So `ß` and `ss` are one file, and `ﬀ` and `ff` are one file — **a one-character name and a two-character name can collide**.

### The three silent failures, and only one is on a Mac

1. **Windows silently strips trailing periods and spaces** from any path not prefixed with exactly `\\?\`. Documented by Microsoft. No error; the file lands under a different name. **This is the one that will bite**, because Linux produces such names happily.
2. **Windows silently collapses a reserved stem** — `NUL.png` "is equivalent to NUL", a device rather than a file. A frame written there is discarded.
3. **A Linux case-folding directory silently loses case-insensitive lookup** for a name not valid in the volume's encoding. The file is created and readable; only the folding stops.

**Everything else measured on the Mac is a visible hard error** — `ENAMETOOLONG`, `EILSEQ`, `EEXIST` — and visible errors are safe. **Nothing is claimed about what Windows returns**: no page read states it and nothing was measured.

### Two more measured limits on the Mac

- **macOS refuses invalid UTF-8 outright** with `EILSEQ`, which Apple documents nowhere. **Linux permits any byte but `/` and NUL.** So a rendered frame can carry a name that cannot be written to an APFS volume **at all**, and that must be caught before the transfer rather than during it.
- **A path length ceiling limits the pathname handed to one call, not the file's depth.** A fetched tree can go deeper than any absolute path macOS will accept while staying reachable by relative descent. Both runs hit the refusal at 1,041 bytes.
- A base character carries at most **31 combining marks** before `EILSEQ`. Measured, undocumented, one volume format.

### The largest hole

**The thirteen manual and reference pages read here documented no call guaranteed to return a file's on-disk capitalisation.** They included the file-find, long-path, handle-information and name-information routes; `GetFinalPathNameByHandleW`'s normalisation is documented only for the drive name and symbolic links. **Fact 121 later superseded the broader negative:** Microsoft's protocol specification documents `FileNormalizedNameInformation` as returning the exact letter casing stored on disk, bounded to NTFS and ReFS; other filesystems refuse the class, and the throwaway remains unrun.

**NTFS's case-folding rule is undocumented** on every page read — no character set, no Unicode version. exFAT's, by contrast, is fully specified as an on-disk table written at format time, limited to the basic plane.

---

## 4. What the user's computer will show as a notification (Fact 73)

**A plain downloaded binary cannot post a notification on any of the three systems without extra machinery.** That is the answer to the clause most likely to break Row 284's promise, and it bites on all three — but in each case the machinery exists and is ordinary.

**macOS.** The notification interface **requires a bundle identity**. Apple documents that precondition nowhere, and it is decisive: an unbundled executable is killed by an uncaught exception on the very first call, exit 134, with nothing to catch. Put the same executable inside a bundle with an identifier and it works; remove the identifier and it dies with a different exception. **An ad-hoc signature was enough** — no Developer ID, no notarization. Reproduced exactly by the citation check, including the distinct failure paths for *no bundle* and *bundle with no identifier*.

**Windows.** Microsoft says package identity is **not** required, but the current recommended route depends on a runtime package that an unpackaged app must deploy to the end user through its installer — and Microsoft's own page says this "can compromise the simplicity of self-contained deployment", offering a runtime probe rather than a guarantee. The older route needs a **Start-menu shortcut carrying an application identifier**: "Without a valid shortcut installed in the Start screen or in All Programs, you cannot raise a toast notification from a desktop app." Two further Windows facts matter: **an elevated app cannot post at all**, and the post **fails silently**.

**Linux.** **No identity precondition at all** in the freedesktop Desktop Notifications Specification v1.3 — the application name "can be blank" and the desktop entry is only a hint. What it needs is a session message bus and a process owning the notification name, and **the specification says outright that clients "must not assume that it is available."** So Linux is the one system where the promise can fail for a reason Smoozy cannot fix.

### Permission, and what happens without it

**Three different models.** macOS: the app must request authorization, the system draws the prompt, a denial is silent — the system "ignores any attempts to schedule local notifications" — and the state is readable back. Windows: no prompt and no request interface found; only a read-only setting whose every non-enabled value is a way of being switched off. Linux: **no permission model in the specification at all**, and therefore no denied state to read.

### The frame preview — the clause Row 284 rests on

- **macOS: yes.** An attachment, **JPEG, GIF or PNG only, 10 MB maximum, and it must be a file on disk** — no in-memory supply.
- **Windows: two Microsoft pages disagree and the disagreement is real.** One lists three image sources, two of which need a package identity an unpackaged app lacks; another documents a fourth, a local file path, "Supported only for desktop apps". **Neither page acknowledges the other.** Both were verified verbatim on both runs and the contradiction is deliberately left unresolved.
- **Linux: yes, and it is the only one that documents supplying pixels in memory** — raw image data, no stated size limit. **But there is no capability string for it**, so a client cannot ask in advance whether the server will render it, and hints may be ignored silently.

### The rest of what a notification can do

**Replace-by-identity works on all three** and is atomic on Linux by specification. *(Corrected 2026-08-18: the clause reading "which is what lets one repeating fact be said once" was a conclusion drawn here rather than a reading, and `Desktop app and delivery` Round 1 overturned it — a later failure resolving to a standing identity needs no post at all. Nothing observed or quoted changed; only the conclusion is withdrawn.)* **Withdrawal works on all three.** **Only Windows documents a true in-place update**, for progress data only, failing if the user dismissed it. **Grouping is documented on macOS and Windows and absent on Linux** — only an optional category hint. **No rate limit or coalescing is documented on any of the three**, on the pages read.

### The limits

**No notification was posted and no permission requested**, deliberately — the machine is the Creator's. So image rendering, banner placement, and the exact error returned when a request has been denied are **unobserved even on macOS**. All Windows and all Linux claims are documentation. Nine Microsoft addresses that search results still point at return 404 and are not cited; four of them redirect first and the redirect targets 404 as well.

---

## 5. How the app arrives and replaces itself (Fact 74)

**The promise is reachable on all three systems, and not by one shared mechanism.** That is the answer, and the first run got it wrong in the expensive direction — it reported that Windows had no documented route at all. The citation check refuted that with two current Microsoft pages. **What survives is narrower and different in kind.**

### Replacing itself while running

**macOS — documented yes.** Apple publishes a page about a software updater replacing a file that contains signed code. It names the wrong route — modifying in place, which "macOS caches information about the code's signature in the kernel" and which "can cause a hard-to-reproduce code-signing crash" — and the right one: write to a temporary file, then rename over the original. There is a supported API for the same operation with a **named backup for rollback**. **No prompt, no elevation and no administrator appears anywhere on that page**, confirmed by reading it end to end rather than a section.

**Observed on the Mac, both ways.** Rename-over, unlink, and move-aside-then-replace all succeed on a running executable and the process keeps running. And Apple's warning was **reproduced deliberately** with the hardened runtime enabled by an ad-hoc signature: in-place replacement left a signature the verifier called valid on disk and a binary that was killed at every launch, while write-then-rename launched and ran.

**Windows — the outcome is documented; the file operation is not.** Microsoft documents, of an unpackaged per-user executable, that it "downloads the update to a hidden folder without any user interaction", that it "verifies and installs it", that "If OneDrive is running, it stops and then restarts", and that users "don't need administrative rights to install the update". Microsoft also documents an interface a third party can call to update and relaunch, on a deployment technology needing no administrative rights. **So an unattended, unelevated, self-relaunching update of an unpackaged Windows app is documented.**

**What is not documented is replacing the image at its own path.** Every route Microsoft documents puts the new build **somewhere else** — one says outright that the app "is reinstalled from a new side-by-side folder". In-place replacement is blocked by everything Microsoft does document: deletion "fails if an application attempts to delete a file that has other handles open"; a rename "will still fail if a file with the same name already exists and is … a currently executing file"; the deferred-rename route "can be used only if the process is in the context of a user who belongs to the administrators group or the LocalSystem account"; and the restart and installer facilities resolve an in-use file by shutting the app down or asking for a reboot. **The Windows answer is side-by-side, not in-place.** Two conditions ride on the third-party interface: it "is not supported in .NET Core and .NET 5 and later versions", and on current runtimes "checking updates before the application starts is the only update option supported".

**Linux — documented yes at the kernel layer, on the thinnest documentation of the three.** The rename call documents that an existing path "will be atomically replaced, so that there is no point at which another process attempting to access newpath will find it missing" and that "Open file descriptors for oldpath are also unaffected"; opening a running executable for writing is refused with `ETXTBSY`, which is only the wrong approach. **The project-level half is weaker than it first looked**: the self-update library's page is copyrighted to 2022, three of the four claims first quoted from it appear nowhere on it, its promise never to overwrite before "all validation mechanisms report success" covers **integrity only** because it states signature validation is not implemented, and the flags it says to respect are ones it says still need defining. **No signing identity, no cost, no enrolment and no elevation appears on any Linux page read** — but the library **advises asking for consent** and by default writes under a new filename rather than overwriting.

### Signing, and the part that is not technical

**macOS.** Signing and notarization are both documented requirements for a download that runs without the system refusing it. Apple's notarization service explicitly removes the macOS dependency for that step, **but no Apple page read documents signing off a Mac** — so a Mac stays in the pipeline.

**Windows is where the cost lands, and it is an identity gate rather than an engineering problem.** Microsoft's managed signing programme is documented as open to individual developers **located in the United States or Canada only**; organizations need "a verifiable tax history of three or more years"; a paid subscription is mandatory; validation takes "from 1 to 20 business days" with three document-upload attempts before the door closes. **The two Microsoft country lists genuinely disagree with each other**, and that was reproduced. The fallback is an ordinary certificate at "$300–500/year".

**And a signature does not buy silence.** Microsoft's own words: "Even when signed, a newly created binary could still show a SmartScreen warning until its hash or publisher certificate accumulates sufficient evidence of positive reputation", taking "several weeks and hundreds of clean installs from a wide audience", with "no need (or mechanism) to manually submit a file for SmartScreen reputation review for consumer endpoints" *(quotation restored 2026-08-22: it stopped at "reputation review", and the dropped words are what scope the sentence — the same page names an enterprise submission portal for "internal or managed deployments", which a consumer download from Smoozy's own website is not)* — and **extended-validation certificates no longer bypass it**, on a page that asserts the change rather than merely being silent. **For a tool at this scale, which will never see hundreds of installs a week, Microsoft's documentation says the warning does not go away on its own.**

**Linux requires nothing.** No signing gate appears on any page read.

### Nothing about the user goes up, and that is documented

Apple states outright that its download checks "have never included the user's Apple Account or the identity of their device". Windows' signature-verification call can be made with a value documented as meaning "There is no interactive user". **So verification on both platforms is compatible with the no-account promise.**

### The one clause deliberately left open, and it bears on macOS

**Whether an app that arrived quarantined re-triggers the system's first-open approval for the copy it writes for itself is not documented on any first-party page read.** The checker attacked it and could not close it: every source that appeared to answer it was a forum, which the bar excludes. **If it does re-trigger, macOS users meet an approval dialog on every update** — which converts *updating is not something they have to remember* into exactly that. Named here rather than assumed either way. **What clearing that dialog costs is unestablished as well — corrected 2026-08-22, out of the first sanity sweep's verification round.** This sentence used to add *"and clearing it costs the user their login password"*, and no first-party page read says a password is involved: Apple's *Safely open apps on your Mac* (`https://support.apple.com/en-us/102445`, read 2026-08-20) describes the override for a **blocked** app as "click the Open Anyway button" and then "you can click Open", names no password, and is not the notarized first-open case. **And one observation on the Creator's Mac, 2026-08-20, which does not close it:** a file a program fetches over its own connection and writes with ordinary file calls carries `com.apple.provenance` and **not** `com.apple.quarantine`, and the attribute is not inherited from a quarantined parent bundle, while browser downloads in the same folder do carry it. Apple's own Gatekeeper page names a second mechanism on exactly this case — "Gatekeeper also tracks the provenance of files written by downloaded software" — so **the absent attribute is not an absent prompt.**

### What the check changed, recorded because it was expensive

**The first run's headline negative was wrong.** It reported that no Microsoft page documents an unpackaged Windows app updating itself, and concluded the packaged route was the only one — which, had it stood, would have been an escalation to the Creator about a promise that could not be kept. Two current Microsoft pages refute it. **Four quotations in the Linux section appeared nowhere at their cited addresses** — all paraphrases printed inside quotation marks — two more were truncated in ways that changed their force, and one was attributed to the wrong Apple page. Nine claims of 141 were struck.

---

## 6. Which shell can host it, and what one really weighs (Fact 96)

**This fact exists to serve a standing ruling and not to take a decision.** The Creator ruled on 2026-08-13 that the app should be relatively small with no fixed ceiling, that the comparison must be against **real release artifacts** on all three systems, that a route using the operating system's own web view or another materially smaller shell is preferred, and that **a route bundling a complete browser engine is not to be chosen merely because it keeps more promises**. What follows is the evidence. **The choice belongs to `Desktop app and delivery`.**

**Every byte figure below is a published asset's own recorded size**, read from the release listing's metadata on **2026-08-18**. Nothing was downloaded and nothing was measured by hand. **All 51 figures were independently re-read by the citation check and every one matched to the byte.**

### The figures, per system

*[Runner, 2026-08-18: the six candidates and the applications each figure was measured on were originally described here rather than named, on the reading that the names sat in the trail. That made this section unusable for the one decision it exists to serve — `Desktop app and delivery` could not name the route it chose, because everything under `Audits/` is outside this stage's reading. **The names are restored below from the findings file's own comparison tables**, which carry them per row with the artifact, the byte count, the version and the date read. No figure changed and no claim changed; what was missing was the name beside the number.]*

**The six candidates are Electron, Tauri, Wails, Neutralinojs, Qt 6 and Flutter.** Three of them — Tauri, Wails and Qt — publish **no end-user application artifact of their own**, so those figures are real applications built with them and the application is named beside each.

**Bundling a complete browser engine — Electron.** The shell alone, before one line of Smoozy: **122.1 MB** on macOS arm64, **144.4 MB** on Windows x64, **125.6 MB** on Linux x64. A real application built on it — YouTube Music v3.12.0 — lands at **124 MB / 287 MB / 142 MB**.

**Using the operating system's own web view.** **Tauri v2**, measured on Yaak v2026.6.0: **66.4 MB / 42.9 MB / 79.9 MB**. **Wails v2**, measured on October v1.12.2, gives the **smallest real figures in the whole comparison — 6.4 MB / 8.8 MB / 5.7 MB.** Both are compiled; the difference in the figures is the application rather than the language.

**Not web-based at all.** **Qt 6**, measured on KeePassXC 2.7.12: **34.4 MB / 35.9 MB / 49.2 MB**. **Flutter**, measured on LocalSend v1.18.0: **52.5 MB / 25.8 MB / 24.6 MB**.

**Neutralinojs ships a single 8.1 MB bundle covering all three systems**, and the per-system share cannot be got without downloading it — so it cannot be placed in an order against the other five.

### Three things that qualify those numbers, and all three are the projects' own words

- **A system-web-view artifact grows back toward browser-engine scale under two documented options.** One project states its Windows offline installer adds "~127MB" and its fixed-version option "~180MB". And a self-contained Linux bundle "bundles all dependencies", which is why one app's self-contained Linux artifact is **149 MB against its 80 MB package**.
- **The three system web views are three different engines** — the projects name them — one of which is self-updating, one frozen to the operating system version, and one whose picture the project itself calls "very hard to compile accurate information about", telling developers to check feature support per version. **That is the documented cost of the small shell.**
- **Installed and unpacked sizes are documented by none of the six.** That clause is open across the board.

### The negatives, which matter more than the sizes

- **Electron documents no automatic update on Linux at all.**
- **Wails documents no self-update anywhere**, which is Row 280's promise. It does document signing and notarization, contrary to the first run's claim. Every page of its current major version carries a notice that the next one is in beta.
- **Flutter's own Windows and Linux pages document store distribution rather than a signed downloadable binary**, which is the wrong shape for Smoozy.
- **Qt 6 is the only candidate whose licence could constrain a closed-source download**, and which of its modules fall under the GPL rather than the LGPL could not be resolved.
- **Only Tauri addresses cross-platform signing head-on**: macOS signing needs an Apple device, and cross-compiled Windows installers need an external tool and are "not tested as much". No other candidate addresses it on the pages read.

*[Runner, 2026-08-18: the five bullets above named no candidate when this file was frozen. The names are restored from the findings file, which states each against its own cited pages. Nothing else changed.]*

### Two hazards in the evidence itself

**A release listing is not frozen.** One asset was added to a listing days after publication. So a byte figure is durable and a claim of the form *that listing holds only these assets* is not.

**One same-application, same-day comparison of a bundled engine against a system web view — 209 MB against 80 MB — rests on a filename alone**, because no documentation explains those assets. It is labelled inferred and should not be leaned on.

---

## 7. What the whole batch leaves owed

**Almost all of it is one thing: no Windows machine was rented for any of it.** Between them these facts leave open the Windows on-disk capitalisation route, the NTFS folding rule, what Windows returns for a forbidden character, a reserved stem, an unpaired surrogate and an over-long component, and what a detached volume reports. **No reading has closed any of those, and one Windows computer closes them all.** *(Corrected 2026-08-22, out of the first sanity sweep's verification round: this list also carried "whether Windows localises the downloads folder" and the sentence read "None of it can be closed by reading, and all of it is closed by one Windows computer". That item was closed by reading — Microsoft states the physical path of a known folder is not localized (§1) — so the list loses one entry and the categorical loses its "cannot".)* [Open Threads](../Groundwork/Open%20Threads.md) already carries that with a Due line saying nothing schedules it.

**Two things want a Linux desktop** rather than a rented server: which filesystem a distribution presents by default, and whether the case-folding option behaves as its kernel documentation says.

**Three things want a Mac that may be changed** rather than the Creator's: a case-sensitive volume and an older filesystem for the name rules, a compiled Foundation call for two clauses, and posting a real notification.

**And one thing wants no machine at all and is still open**: whether a quarantined macOS app re-triggers first-open approval for the copy it writes for itself. It is the clause most likely to cost the update promise something the user meets, and no first-party page read answers it.

**The machine is not coming, so this section reads as permanent rather than pending. Added 2026-09-02.** On 2026-08-31 the Creator ruled that **no further rented machine will be taken to answer a question**, and the Windows machine, the Linux desktop and the second Mac above are each exactly that. **What that changes and what it does not.** It does not change one claim in this file: every reading here stands as read, and the reading route stays open — a first-party page that answers one of these closes it tomorrow, and the downloads-folder entry struck from the paragraph above on 2026-08-22 is this file's own worked example of that happening. **What it changes is the label.** Each item above was written as owed and read as scheduled; none of them is. They are **unresolved permanently**, and the three groups are not alike:

- **The Windows group — Facts 16, 23 and 24's Windows halves** (the on-disk capitalisation route, the NTFS folding rule, the return for a forbidden character, a reserved stem, an unpaired surrogate and an over-long component, and what a detached volume reports). **Breaks:** every Windows clause built on them is documentation alone with no observation behind it — `Desktop app and delivery` [§6](Desktop%20app%20and%20delivery.md) the platform file adapter's refusal classification, [§7](Desktop%20app%20and%20delivery.md) the receiving-filesystem collision judgement, and [6.6](Desktop%20app%20and%20delivery.md)'s folder-identity comparison — so Row 303 and the Rows that read it hold on one system's observation and two systems' pages. **What holds without it:** that document already writes each of them three times and already names the Windows half as unresolved rather than answered — Y3, Y5, F15a's identity-unavailable branch, F21c and C8 are the clauses that carry it, and none of them claims what Windows returns. **Nothing here is owed a second telling** and no clause changes; what is removed is the expectation that a machine will one day arrive and turn a documented claim into an observed one.
- **The Linux-desktop and second-Mac groups** are the same ruling on the same terms, and their breaks are already written where they are consumed. Neither is a rental this project would take and neither is scheduled.
- **The quarantine question is not in this group at all**, and the distinction is the point: it wants no machine, so the ruling does not reach it and **it stays answerable by reading**. It is the one item in this section a later reader should try again rather than write off.

[Open Threads](../Groundwork/Open%20Threads.md)'s Due line, which said nothing schedules the machine, is now true for a reason rather than by accident.
