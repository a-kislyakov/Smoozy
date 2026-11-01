# Sources — Download

*Where every outside-world fact [Download](../Download.md) leans on came from, what the owning document's own words are, and what each does not cover. A decision Smoozy made gets no entry.*

**Shapes used.** Most entries are **pointers** into this stage's frozen input documents, which own the citation. Blender entries name the joined answer or the comparison question the document itself already names, and nothing more. Three entries are **assumptions or unresolved**, recorded because no source for them exists in this project.

**Three addresses were read for this file on 2026-08-23** — Apple's `rename(2)`, `pathconf(2)` and `getattrlist(2)` — because the trails behind Facts 79 and 24 held the words and the date read and no address anywhere in `Preparation/`. Each is marked at its entry with what was read and what was found. **Everything else here is still a pointer**, and where an address belongs to another document's entry it is named as that document's rather than copied.

---

## 1. Pointers into this stage's frozen input documents

### Fact 18 — a part-arrived file is indistinguishable from a whole one on the machine

**Owner:** [Six machines](../Six%20machines%20—%20the%20container,%20the%20webhook%20and%20the%20ready%20action.md), *Fact 18*. Label: **observed**, 2026-08-15/16.

Its words: killed and dropped transfers leave bytes at the final name with *"identical mode, permissions, extended attributes, inode flags, allocation and file-type output"* to a whole file, and *"Nothing on the machine distinguishes them."* The interrupted overwrite: 40,000,000 bytes of one file overwritten by a killed transfer of another left a 40,000,000-byte hybrid, 10,444,800 new bytes spliced onto 29,555,200 old ones — *"Neither version, plausible size, no marker."*

**Used by** [Section 1](../Download.md#1-what-binds-this-before-anything-is-decided), J6's *presence is not proof*, M10's Row 208 answer, and R8/R9's refusal to ask the already-exists question of the destination.

**Not covered:** the input document is *"a frozen snapshot of six machines on two days"* and states its own reading rule — *"this is what those boxes did, never this is what a rented machine does."* It is a statement about the **rented** machine's filesystem; nothing in it is evidence about the user's own computer.

### Fact 20 — a file cannot be told finished from still-being-written over the connection

**Owner:** [Six machines](../Six%20machines%20—%20the%20container,%20the%20webhook%20and%20the%20ready%20action.md), *Fact 20*. Label: **observed**, 2026-08-15/16, posed at 10000×10000 where the write window is 1.2–1.6 seconds.

Its words: a non-root reader watching a root-written file got *"160,151 refusals, zero matches, and 'held by nobody' from an 18 MB file all the way to 1.6 GB"* — the same answer the route gives for a finished file; the same-user control found 42 matches, *"proving it is the boundary and not the route."* Two related tools *"return exit 0 and no rows for a file very much held open."* And *"size stability is a trap"*: a frozen transfer sat at 17,233,920 bytes for 11.1 seconds while unfinished.

**Used by** [Section 1](../Download.md#1-what-binds-this-before-anything-is-decided), [Section 4](../Download.md#4-telling-a-finished-file-from-one-still-being-written-rows-211-and-203)'s preamble (the process-watching route and the size-stability route both closed off), J6, and W6 (the rename lever the fact found, closed off by the spec).

**Not covered, in the owner's own *Not covered* line:** a `/proc` mounted to hide other users' processes was not posed; only one of the two ordinary accounts was tried; the non-root reader was a real non-root process rather than a separate login, so the fact does not separately prove a non-root login can connect at all on that box. [Section 15](../Download.md#15-unresolved) item 4 carries this.

### Fact 19 — a change on the machine can be noticed while connected, and what it costs

**Owner:** [Six machines](../Six%20machines%20—%20the%20container,%20the%20webhook%20and%20the%20ready%20action.md), *Fact 19*. Label: **observed**, 2026-08-15/16.

Its words: a render took 43.64 seconds watched against controls of 43.47 and 43.79, *"and the watcher cost 0.08 seconds of processor over 55"*; a file arriving by transfer is *"indistinguishable in event shape"* from a frame being written. The trap: *"a watch on a file follows the inode"*, so a save-by-rename kills it silently, and a replaced folder gives one ignored event and then nothing.

**Used by** W1's third bullet (the inode-following event tool, quoted), and nothing else. **The cost figure is no longer used: W1's fifth bullet cited it until 2026-08-23, and the citation was withdrawn** for the reason in the second half of the line below.

**Not covered:** the render was deliberately run on the processor, so the card case is unposed ([Section 15](../Download.md#15-unresolved) item 5). **And the measurement is of a machine-side event watcher**, which is the route W1 rejects; the mechanism W1 builds has no machine-side program at all, so the 0.08-second figure is a cost for a heavier route than the one taken and is not a measurement of this document's watch. **That second half is what withdrew the citation**, and W1's fifth bullet now says outright that no measured figure exists for what this walk costs the machine.

### Fact 7 — what listing a folder over the connection yields, and which failures separate

**Owner:** [One box with Blender on it](../One%20box%20with%20Blender%20on%20it%20—%20the%20reading,%20the%20listing,%20and%20the%20names%20that%20do%20not%20come%20back.md) §5, posed throughout under the non-root login. Label: **observed**, 2026-08-17.

Its words: all four fields come back in one reading on both routes — 5010 entries cost *"52 round trips, not 5010"* against 651 seconds for one stat per file, the measured figure *"18.22 s"* on the file-transfer route and about a quarter-second on the command channel; the attribute flag `0xf` is size, owner, permissions and times. The zero-length file *"reports `0` on both routes at no extra cost"*. On the file-transfer route the three failures carry distinct protocol statuses with controls either side, while the command channel shares exit 2 between refusal and absence. The sparse file reports its full apparent size with zero blocks allocated, and *"the file-transfer route carries no allocated-block count"*. And the finding W5 rests on: *"Partial output is delivered before the connection dies. A truncated listing is indistinguishable from a complete short one except by exit status."*

**Used by** [Section 1](../Download.md#1-what-binds-this-before-anything-is-decided), W1's second, fourth and fifth bullets, W5's discard rule, M2's enumeration, M3's per-route separation, and M17's sparse-size limit.

**Not covered, in the owner's own *Limits* line as carried by the register:** one container, `overlayfs`, the proxy route only, one unprivileged login shape with no groups, access lists or read-only mounts; the Linux form of the path-under-an-unreadable-parent case is unposed. [Section 15](../Download.md#15-unresolved) item 6 carries this.

### Fact 25 — what the link carries, and where concurrency stops helping

**Owner:** [Six machines](../Six%20machines%20—%20the%20container,%20the%20webhook%20and%20the%20ready%20action.md), *Fact 25*. Label: **observed**, 2026-08-15/16.

Its words: 12–15 MB/s down and 1.2–1.3 up against the same home line; *"Down is 11–12 times up, and that asymmetry is the dominant fact about this connection."* Concurrency against one machine: *"the knee is at two streams for download — one to two buys 53%, two to four buys 3%, four to eight buys 11%"*, and around four for upload. *"Two to four is the entire useful range"*, and most of the gain is hiding a flat 1.0–1.4 s connection setup rather than adding bandwidth.

**Used by** Q1, as one of its two reasons for the download-direction ceiling of two. *Corrected 2026-08-29 at the stage close: this line read "which is the whole reasoning for the download-direction ceiling of two", the ground Q1 itself withdrew on 2026-08-23 and which the paragraph below already records.*

**Not covered, in the owner's own *Not covered* line:** one home line, one evening, over Wi-Fi; two of the three rentals were the same provider. [Section 15](../Download.md#15-unresolved) item 7 carries this.

**One thing the document said that the owner does not, corrected 2026-08-23.** Q1 read the curve as *"the second channel buys half again and everything above it buys noise and setup-hiding"*, while the owner's reading of the same numbers is *"Two to four is the entire useful range"* and records four-to-eight at 11% rather than at noise. **The conclusion survived and its ground did not**: Q1 now quotes the owner's own reading, says the curve settles the second channel and not two against four, and names its second reason — two of the pool's four slots kept unavailable to the download direction so the slow upload direction is never starved — as what settles two against four. [Section 15](../Download.md#15-unresolved) item 7 carries the same split.

### Fact 55 — what a restart stops, against a real render

**Owner:** [Six machines](../Six%20machines%20—%20the%20container,%20the%20webhook%20and%20the%20ready%20action.md), *Fact 55 — what a restart stops, posed against a real render*. Label: **observed**, 2026-08-15/16, two real machine restarts with a render in flight.

Its words: *"A frame here is about 16.8 seconds: ~15.4 sampling, ~1.4 writing."* Interrupted during the write, a partial sits under the frame's real final name — 52,514,816 bytes carrying OpenEXR's real magic number, not sparse, *"with nothing to mark it"* — *"So Blender does not write to a temporary name and rename."* Interrupted during sampling, about 92% of a frame, *"there is nothing at the path at all"*. A polite stop behaves identically to a hard kill at exit 143, *"and that is the signal a shutdown sends."*

**Used by** W2's second constraint (10 seconds is short against a frame).

**Not covered:** one output format and one picture size, the fact's own line; a dependency check in flight was not posed. [Section 15](../Download.md#15-unresolved) item 16 carries this.

### Fact 28 — free space, and what a probe of a destination has to be

**Owners, two.** [Six machines](../Six%20machines%20—%20the%20container,%20the%20webhook%20and%20the%20ready%20action.md), *Fact 28*, and [Two more boxes](../Two%20more%20boxes%20—%20the%20volume,%20the%20full%20disk,%20and%20a%20day%20left%20running.md) §*Fact 28* for the more-than-one-disk clause. Label: **observed**, 2026-08-16.

The words D7 rests on: *"on a full filesystem, the writability test **returns writable**, and **creating an empty file succeeds** — an empty inode needs no data blocks. Only writing at least one byte fails."* And the words J9 rests on: *"a file of size zero is indistinguishable between 'still being written', 'the disk filled' and 'the render wrote nothing'."* A refused folder *"can be seen taking files again"*, verified both by permissions and by genuinely filling a disk until writes failed and then freeing it.

**Used by** D7's first bullet, J9's zero-byte mark, and [Section 1](../Download.md#1-what-binds-this-before-anything-is-decided).

**Not covered:** these are readings of a **rented Linux** machine. D7 applies the real-write rule on the user's own side, where the corresponding observation is Fact 23's; the register's own correction of 2026-08-21 records that the Linux run *"only ever posed the already-full case"* and that the partway-fill case is unposed on Linux.

### Fact 29 — names that reach the machine and do not come back

**Owner:** [One box with Blender on it](../One%20box%20with%20Blender%20on%20it%20—%20the%20reading,%20the%20listing,%20and%20the%20names%20that%20do%20not%20come%20back.md) §7.1. Label: **observed**, 2026-08-17.

Its words: *"Linux took every one. Seventeen went out and fourteen came back."* All three case-variant names resolve to one file holding the last payload fetched; the two Unicode spellings collapse the same way. *"Every fetch reported success — no error, no warning, no exception."* And the sentence the document carries at its own door: *"Frames rendered into distinct names on the rental can arrive on the user's computer as fewer files than were made, with nothing indicating anything was lost."* The control in the same sitting proves the loss is at the Mac end and not in transit.

**Used by** [Section 1](../Download.md#1-what-binds-this-before-anything-is-decided) and P6's second bullet (*cannot land*, the seventeen-to-fourteen loss arriving at this document's door).

**Not covered, in the owner's own words:** *"That is a property of **this** Mac's disk, which ignores capitals by default; it would not happen on one set up to respect them, and Smoozy cannot assume either."* One Mac, one case-insensitive volume at default settings; a case-sensitive volume was not posed. [Section 15](../Download.md#15-unresolved) item 15 carries this.

### Fact 23 — what a failed write to the destination reports on the user's own computer

**Owner:** [The user's own computer](../The%20user's%20own%20computer%20—%20where%20a%20download%20lands,%20what%20it%20will%20take,%20what%20it%20will%20show,%20and%20how%20the%20app%20replaces%20itself.md) §2. Label: **observed** on macOS 26.5.2, build 25F84, arm64, 2026-08-18; Windows and Linux values in that section are **documented** only.

Its words at `open()`: full volume `ENOSPC` (28), permission refused `EACCES` (13), directory removed `ENOENT` (2), volume detached `ENOENT` (2) *"where the mount point went with the volume"* and *"no error at all"* where the mount point survived. *"A removed directory and a detached volume collapse into one value … Smoozy cannot separate them from the error alone and must not claim to."* The proposed separating test is *"Struck rather than carried."* At `write()`: a directory deleted underneath an open descriptor *"reports success at every call"* and *"the only defence is to re-check the destination by path after closing"*; a detached volume gives `EIO` (5) once at the write, with `fsync` and `close` then returning 0. And the reading D3 and D5 rest on: *"On the genuinely full volume the reading was 1,068 KB, not zero — so any check written as available equals zero will never fire."*

**Used by** [Section 1](../Download.md#1-what-binds-this-before-anything-is-decided), D1's four classes, D2's narrowing of *a cause Smoozy genuinely knows*, D3's refusal of any threshold, D5's write-only withdrawal, and D7's probe.

**Not covered, in the owner's own *What is not established*:** Windows' detached-volume error — *"Six candidate codes exist and Microsoft's `WriteFile` page names none"* — and *"Linux's manual pages name none either"*; quota behaviour and blocking writes are documentation-only. [Section 15](../Download.md#15-unresolved) items 1, 2 and 3 carry these.

**The value the class list did not name, supplied 2026-08-23.** The detached-volume `EIO` at `write()` is recorded by the owner and by `Desktop app and delivery` F2, and F4's classifier — `ENOSPC`, `EACCES` and `ENOENT` at `open()`, `ENOSPC` alone at `write()` — named no class for it, so D1's list carried none either; the gap originated in F4 and [Section 15](../Download.md#15-unresolved) item 23 routed it to that document rather than filling it here. **It came back as F4a**, a fourth adapter class — *the write failed at the device*, cause unnamed — and D1 consumes it as supplied exactly as it consumes the other three, with item 23 closed. **The class carries this fact's own label and no wider one:** `EIO` is **observed** on the macOS run recorded above and is claimed for neither Windows nor Linux, whose detached-volume values are unestablished (items 1 and 2 of that section). The case stays bounded by F2, which makes the first error the outcome; by D2, which claims no cause it cannot separate and names no remedy where it names no cause; by the unit failing rather than recording *fetched*, so R9 never makes it permanent; and by D4's folder-path key, which needs no cause to raise the condition.

### Fact 24 — which names the user's computer will take

**Owner:** [The user's own computer](../The%20user's%20own%20computer%20—%20where%20a%20download%20lands,%20what%20it%20will%20take,%20what%20it%20will%20show,%20and%20how%20the%20app%20replaces%20itself.md) §3. Labels as that section marks them: Linux 255 bytes **documented** from ext4's own kernel documentation and POSIX's own definition; Windows 255 UTF-16 code units **documented**; macOS 255 UTF-16 code units **measured, not documented**.

Its words: *"Apple's own two manual pages contradict each other on this and both are wrong: `pathconf(2)` says bytes, `getattrlist(2)` says characters. A 765-byte name was accepted and a 128-character name refused."* The three silent failures: *"Windows silently strips trailing periods and spaces"* from any path not prefixed with exactly `\\?\`, documented by Microsoft, *"This is the one that will bite, because Linux produces such names happily"*; *"Windows silently collapses a reserved stem"* — `NUL.png` *"is equivalent to NUL"*; and a Linux case-folding directory silently losing case-insensitive lookup. *"macOS refuses invalid UTF-8 outright with `EILSEQ`, which Apple documents nowhere."* And the rule P8 rests on: *"a path length ceiling limits the pathname handed to one call, not the file's depth."*

**Used by** P4's name check, P8's per-file refusal, P10's trailing-period case, and P11's depth rule.

**Not covered:** *"Nothing is claimed about what Windows returns: no page read states it and nothing was measured."* The forbidden character, reserved stem, unpaired surrogate and over-long component cases all want a Windows computer this project does not have **and will not get: the Creator ruled on 2026-08-31 that no further machine is rented to answer a question, so these four are permanently unresolved rather than unmeasured — recorded here 2026-09-02.** [Section 15](../Download.md#15-unresolved) items 1 and 8 carry these, item 1 now with that permanence and with the correction that Fact 22's downloads-folder half left this list on 2026-08-22, answered by reading.

**The two Apple pages, read for this file 2026-08-23.** The input document and `Desktop app and delivery` C5 name them by call and quote them, and no address for either was recorded anywhere in `Preparation/`. Both were read at Apple's own publications and both carry the claim.

- **`pathconf(2)`**, Apple's own manual page, `https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/pathconf.2.html`, page footer *"4th Berkeley Distribution — June 4, 1993"*, read 2026-08-23. Its words: *"_PC_NAME_MAX — The maximum number of bytes in a file name."* and *"_PC_PATH_MAX — The maximum number of bytes in a pathname."* **Bytes, as the input document carries it.** The page shipped on the project's own Mac carries the same footer date and the same two lines.
- **`getattrlist(2)`**, Apple's own manual page source as published in `xnu`, `https://github.com/apple-oss-distributions/xnu/blob/main/bsd/man/man2/getattrlist.2`, its own `.Dd March 7, 2025`, read 2026-08-23. Its words: ATTR_CMN_NAME is *"An attrreference structure containing the name of the file system object as UTF-8 encoded, null terminated C string. The attribute data length will not be greater than NAME_MAX + 1 characters, which is NAME_MAX * 3 + 1 bytes (as one UTF-8-encoded character may take up to three bytes)."*, and `[ENAMETOOLONG]` is *"A component of a path name for getattrlist exceeded NAME_MAX characters, or an entire path name exceeded PATH_MAX characters."* **Characters, as the input document carries it, and the contradiction with `pathconf(2)` is confirmed rather than repaired.**
- **Why the second address is the source rather than the archive page.** Apple's archived HTML `getattrlist(2)` — `https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/getattrlist.2.html`, footer *"Darwin — October 14, 2004"* — is an older revision of the same page and **does not carry the word `characters` in ATTR_CMN_NAME at all**; it reads *"will not be greater than NAME_MAX + 1"* and stops. The claim is the shipped page's, and `xnu` is where Apple publishes that page with an address. Both were read; the older one is named here so the next reader is not sent to it.

**What none of this changes.** The macOS 255-UTF-16-code-unit figure stays **measured, not documented** — a 765-byte name accepted and a 128-character name refused is the input document's own measurement, and neither Apple page states the figure. **And no address is supplied here for the Microsoft trailing-period and reserved-stem pages or for ext4's kernel documentation**; those are `Desktop app and delivery` C5's and C7's citations and are named as that document's rather than copied.

### Fact 22 — where a download lands on each of three systems

**Owner:** [The user's own computer](../The%20user's%20own%20computer%20—%20where%20a%20download%20lands,%20what%20it%20will%20take,%20what%20it%20will%20show,%20and%20how%20the%20app%20replaces%20itself.md) §1, consumed by this document through [Desktop app and delivery](../Desktop%20app%20and%20delivery.md) §8 W1–W7 rather than directly. Label: **documented**, read 2026-08-18 and 2026-08-20, three routes on three systems.

The three claims H1–H3 lean on, in the owner's words: *"The three systems are three different mechanisms, not three spellings of one."* — *"Linux has no downloads specification. The XDG Base Directory Specification contains the string `download` zero times, and freedesktop's specification index carries no `user-dirs` entry. What governs it is a tool, not a standard."* — *"The documented Linux fallback is `$HOME`, not `$HOME/Downloads`. A Smoozy that assumes the latter proposes a folder the system does not use."* And the instruction H2 turns on: Apple's own words, that an app *"must not write files to the preceding directories unless explicitly directed to do so by the user"*.

**Used by** H1 (consumed whole), H2's explicit-direction rule, and H3's refusal to compose a path.

**Not covered:** whether the macOS route looks the location up or composes it — observation suggests it composes — which wants a compiled Foundation call on a Mac. `Desktop app and delivery` W6 owns it and H4 declines to re-carry it.

### Fact 79 — the platform operation that publishes a staged download into its final name

**No frozen input document owns this one.** It is held by [Desktop app and delivery](../Desktop%20app%20and%20delivery.md) F16 and its [Section 22](../Desktop%20app%20and%20delivery.md#22-unresolved) item 4, and in the register entry for Fact 79 in [Facts](../Facts.md). Its citations are that document's and belong in [its own companion file](Desktop%20app%20and%20delivery.sources.md); this entry points there rather than keeping a second copy of them.

The words, as F16 carries them. **macOS `rename(2)`**, documented, macOS 26.5, page dated June 3 2021, read 2026-08-17: *"The rename() system call guarantees that an instance of new will always exist, even if the system should crash in the middle of the operation."* **Linux `rename(2)`**, quoted in Fact 78's answer: *"If newpath already exists, it will be atomically replaced, so that there is no point at which another process attempting to access newpath will find it missing."* **Windows**, documented and negative: Microsoft documents `MoveFileExW` with `MOVEFILE_REPLACE_EXISTING` and `ReplaceFileW`, *"but no crash guarantee for either"*, and `ReplaceFileW` publishes a named failure state in which *"the replaced file no longer exists and the replacement file exists under its original name."*

**Used by** P5 (every publication create-intended, the reservation making the rename always a replace) and [Section 15](../Download.md#15-unresolved) item 18.

**The macOS page, read for this file 2026-08-23.** Apple's own manual page, `https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/rename.2.html`, header *"RENAME(2) — BSD System Calls Manual"*, read 2026-08-23. Its words, **exactly as F16 and this entry already carried them**: *"The rename() system call guarantees that an instance of new will always exist, even if the system should crash in the middle of the operation."*

**One difference, and it is a page date rather than a claim.** That archived page's footer is *"4.2 Berkeley Distribution — June 4, 1993"*, while F16 records the page as *"macOS 26.5, page dated June 3, 2021"*. The later date is the page shipped on macOS, which was checked on the project's own Mac on 2026-08-23 — footer *"BSD 4.2 — June 3, 2021"* — and carries that sentence word for word. **Two vintages, one sentence; nothing the document says moves.**

**The two Microsoft pages are not addressed here.** `MoveFileExW` and `ReplaceFileW` are `Desktop app and delivery` F16's citations, that document owns Fact 79, and its address for each belongs in its own companion file — copying one here would give the same citation two homes. This entry points at F16 for them, as it does for everything else in Fact 79.

**Not covered:** the throwaway route is unrun on all three systems and the Windows half cannot be posed until a Windows computer exists.

### Fact 132 — what each system offers an application for showing a folder, and whether the chosen shell exposes it

**Added 2026-08-29 at the stage close**, which found H7 and H8 resting on this claim with no entry here and no unresolved item, while `Reporting a problem` V7, its §11 and `The shape` §19 all named the gap by clause letter.

**No frozen input document owns this one.** It is held by [Reporting a problem](../Reporting%20a%20problem.md) **V7** and its [Section 12](../Reporting%20a%20problem.md#12-unresolved) item 5, answered in [The three desktop systems read again](../The%20three%20desktop%20systems%20read%20again%20—%20dying,%20revealing,%20resolving,%20and%20what%20the%20shell%20does%20not%20expose.md) §3, and registered at Fact 132 in [Facts](../Facts.md). **Its per-system citations are V7's and this entry points at V7 rather than keeping a second copy of them** — `Reporting a problem` has no companion sources file at the time of writing, which is that document's to answer and is reported rather than repaired here. Label: **documented**, answered 2026-08-25 on all three systems, tried on the Mac and **documentation alone on Windows and Linux**.

The two claims H7 and H8 lean on, as V7 carries them. **The act exists on all three** — macOS `selectFile:inFileViewerRootedAtPath:`, Windows `SHOpenFolderAndSelectItems`, and on Linux the desktop portal's `OpenDirectory` with its own published fallback — **and all three report failure to the caller**, which is what makes H8's *a place Smoozy cannot open* a reported outcome rather than a guess. And the negative: **the chosen shell reaches none of it**, its one browser-opening call refusing `file://` addresses by name and its open-a-thing call returning nothing at all, so the service is platform code beside the shell.

**Used by** H7's *one place, and it opens*, its unlettered paragraph naming the supplier, and H8's *cannot open* reading.

**Where the service is written, corrected 2026-08-29 at the re-gate.** This entry read *"the service does not exist yet — V7 declares it and routes it to `Desktop app and delivery` §21, where it is not yet written"* and left the debt in V7's list. **It was written on 2026-08-25, at `Desktop app and delivery` [§18.6](../Desktop%20app%20and%20delivery.md#186-the-folder-reveal-service) K20–K24** — the service and its two acts (K20), the per-system routes (K21), the outcome reaching the caller and naming `Download` H8's *cannot open* by clause letter (K22), the callers including the *show a job's downloaded files* control (K23), and what it does not answer (K24) — **and supplied to `Download` by that document's §21 *To `Download`* line of the same date**. `Reporting a problem` §12 item 5 is headed **CLOSED, 2026-08-25** and says so on its own first line, naming §18.6 as what closed `Download` H7's serviceless *"it opens"*; `The shape.md` records it in the past tense. §21 lists the service; §18.6 writes it, and pointing at the list rather than the writing is what stopped this being seen for four days. [H7](../Download.md#11-where-downloads-land-row-204) now names K20–K24.

**Not covered:** the three builds' packaging half of the question rides with V7. **And V7 itself still reads *"which does not exist yet and is routed, not written"***, which is where this entry's error was inherited from; it is that document's to correct and is routed at [Section 15](../Download.md#15-unresolved) item 27. Nothing in H5–H8 waits on the service either way — the set and its order are composed from records already kept.

---

## 2. Blender, named and not re-derived

Every Blender claim below is owned where the document already names it. Nothing is re-derived and nothing is assembled out of more than one topic.

### The frame order, and each frame's files written as that frame finishes

**Named by the document as** [What a job writes](../../Specs/What%20a%20job%20writes.md), whose own citation is in [its sources file](../../Specs/Sources/What%20a%20job%20writes.sources.md) against the comparison questions **Frames 10** (the order) and **File output nodes 13** (when a node writes). Label there: **observed, the same in every version, on both machines**.

The spec's own words: *"A typed list of frames is rendered in the order it was written, not sorted"*; and *"A node writes its files as each frame finishes, not in one lot at the end of the job."*

**Used by** J1's first two bullets, J2's *A range is never the order*, and J8.

**Not covered:** the sources file records that the node-writes-per-frame result carries two conditions — the node writes during a render only, and switching compositing off stops it writing at all.

### The placeholder's timing, and the Placeholders setting itself

**Named by the document as** `Preparation/Blender Research/Comparison/Writing.md` **question 11**, finding 5: *"the empty file for the next frame appears at the moment the previous frame's picture is complete"*, **both machines, all four versions**. **Corrected 2026-08-29 at the re-gate.** This entry recorded that J1 quoted the wording carried by the row file [Rows/Download](../Rows/Download.md) — *"at the instant the previous frame's picture completes"* — rather than the research's own; that string is in no research file, and J1 now quotes finding 5 exactly. **The substance never differed** and nothing resting on it moved. *(The row file carries the same paraphrase and is the runner's.)* The same question records that the Placeholders setting *"is off out of the box"* — **documented** in all four.

That the setting does not reach a file output node is `Preparation/Blender Research/Comparison/File output nodes.md` **question 10**, in its own words: *"A two-frame animation with placeholders on produced the node's files and the scene's files, and no placeholder for the node."*

**Used by** J1's second bullet, J4's two signal paths, and J6's second bullet.

**Not covered:** the placeholder timing is a folder watched at intervals — see the assumption entry below.

### A part-written file opens, and nothing may key on a size

**Named by the document as** [What a job writes](../../Specs/What%20a%20job%20writes.md), cited in [its sources file](../../Specs/Sources/What%20a%20job%20writes.sources.md) against **Frames 13**. Label: **observed**, all four versions, both machines.

The spec's words: *"a stopped TIFF, and sometimes a stopped JPEG, loads and reports the full picture size with real data in it, so it opens is never proof a frame finished"*; and *"How many bytes are left is a race with no pattern by version or machine, running from nothing to some tens of thousands, so nothing may key on a size."*

**Used by** [Section 4](../Download.md#4-telling-a-finished-file-from-one-still-being-written-rows-211-and-203)'s preamble, which is where the whole size closure and its single zero-byte exception are stated.

### A multi-layer file's format is the file's own

**Named by the document as** `What a job writes`' rule, already inside the recorded reading. That spec's words: set to write a multi-layer file, a node *"writes one file per frame holding every output as a layer of it, whatever the number of outputs"*. Label there: **documented, inferred and observed; differs by version**.

**Used by** N1's last sentence and N5's refusal of a name-suffix rule.

### The video destination's file size while a render runs

**Owned by** [Job files and outputs](../Job%20files%20and%20outputs.md) I19–I21, whose Blender ground is cited in [What a job writes' sources](../../Specs/Sources/What%20a%20job%20writes.sources.md) against **Frames 5 and 12** and **Writing 8**: the file *"appears within moments at a token size and its reported size does not move while the render runs"*, reaching its real size only at the close or a split. Label: **observed**; two limits travel with it — one container and one codec.

**Used by** J8's second paragraph, which is why a video destination's entry is never fetchable before the ending walk.

### Row 214 — a format the version rendering it cannot deliver

**Named by the document as** a Blender question, checked 2026-08-12 and standing. The owner is `Preparation/Blender Research/Comparison/Images.md` **question 4**, cited by [the Download spec's sources](../../Specs/Sources/Download.sources.md) and by [Rows/Download](../Rows/Download.md). Label: **unresolved** — not established for any version, for the same reason in all four.

The reason, in the spec's own words: *"every copy of Blender the project asked has every optional format built into it, on both the machines it could test, so the case was never produced."* A fifth Blender came with a rented machine's own image, was never asked anything, and that machine is destroyed — **Builds 7**. Settling it needs a Blender built without one of the optional formats.

**Used by** [Section 13](../Download.md#13-row-214-closed-unresolved) and E5.

**Not covered:** the right reading is *never asked*, not *cannot exist*. The document states this and does not read the silence as an answer.

---

## 3. No source in this project

### The no-overlap inference — **REWRITTEN 2026-09-01: it is a result on both machines now, at two different strengths, and one of the two quotations this section rested on no longer exists in its source**

**What was wrong here, and it is the shape a quote check catches and a citation check does not.** This section and `Download` §15 item 17 both quoted `While a render runs` for *"Nothing stands for a frame while it renders" rests on a folder watched at intervals, and the same question records a short-lived file falling between two samples* — **and that sentence is gone from that file**, repaired underneath both on 2026-08-30. Both carried the annotation *re-quoted 2026-08-29, no word moved*, which was true of a file that has since moved.

**What the repaired research says.** *Writing 14*'s verdict, in its own words: **every write belonging to one frame is finished before any write belonging to the next frame begins** — in all four versions, on the Creator's Mac and on a rented Linux machine, with no window and again with a window. **Two instruments, and that question insists they are two separate claims: *"Do not let one carry the other."*** The folder watched at intervals — the write made slow on purpose at about 190 milliseconds a frame on the Mac, the folder read every two milliseconds, **sixty-three separate readings inside one write on the Mac and thirty-one on Linux** — which settles macOS. And **every file call traced thread by thread on the rented Linux machine**, which, in *Writing 14*'s own words, *"settles Linux and says nothing whatever about macOS"*. **Address corrected 2026-09-02:** this line gave those words to `While a render runs`, whose own sentence is the shorter *"it settles Linux and says nothing about macOS"* (line 124, read 2026-09-02); the quoted form is `Preparation/Blender Research/Comparison/Writing.md` question 14 (line 629, read 2026-09-02). The sentence is real and unchanged in substance — the wrong file was named for it, which is the one thing a citation check is for and the reason this note is written rather than the quotation quietly swapped.

**The one quotation that survives, and it still bounds the sampled half.** *"The size a part-written file was first caught at is a property of how often the folder was looked at, not of Blender."* (`While a render runs`, on *Writing 11*.) A rule about instants between two samples is not something a sampled folder can show, and the Mac's half is sampled.

**And the neighbouring thing the trace settles outright:** *"there is no rename call anywhere in it, and every file is written under its final name from the first byte"* (*Writing 14*) — so no temporary name is ever mistakable for a finished frame.

**Label: observed in a test on both machines, by two instruments of different strength.** **The macOS half is permanently unresolved rather than pending** — the instrument that would close it is a thread-by-thread trace, which `While a render runs` records as having *"no equivalent on the Creator's Mac that does not need a system setting changed"*, and no agent may change one. Under the Creator's ruling of 2026-08-31 there is no further research.

**The document's own body was brought into line on 2026-09-02, and until then it was the last live copy of the dead quotation.** `Download` [§4.1](../Download.md#41-the-ground-both-mechanisms-stand-on)'s closing paragraph still read *"No research answer establishes it"* and *"nothing tested for one"*, still quoted the sentence about a short-lived file falling between two samples, and still carried the annotation *(Both re-quoted 2026-08-29 at the re-gate … No word moved.)* — so §15 item 17 and this section said one thing and the clause they describe said the opposite. It now states the verdict, keeps the two-instrument split, and keeps the sampled limit that survives. *(The original section follows.)*

### The no-overlap inference — an assumption, and the document marks it as one

**Assumption.** J1's closing paragraph states that the first write belonging to a later frame comes after every write of the earlier frame is complete, and says outright: *"No research answer establishes it."*

**What was looked in, and what was found.** `Preparation/Blender Research/Across topics/While a render runs.md`, the joined answer the document names, records the limit itself rather than the rule: *""Nothing stands for a frame while it renders" rests on a folder watched at intervals, and the same question records a short-lived file falling between two samples."*, and *"The size a part-written file was first caught at is a property of how often the folder was looked at, not of Blender."* *(The first re-quoted 2026-08-29 at the re-gate: the joined answer's inner double quotation marks had been written as single ones, which no plain-string search finds. No word moved, and the second was already exact here.)* **Both quotations are dead. Corrected 2026-08-31, out of the repaired Blender research.** The joined answers were repaired on 2026-08-30, after the check that had ruled every downstream citation sound, and **the sentence about a folder watched at intervals no longer exists in `While a render runs`** — the 2026-08-29 re-quote was against the text as it then stood, and that text has moved. **The instrument has been bettered: every file call is now traced thread by thread on Linux.** Label: **observed in a test**. **What that settles and what it does not: it settles naming, and it does not settle overlap.** So **the assumption above still stands, unweakened and unstrengthened** — nothing yet claims an absence of overlap and nothing tested for one — but the *what was looked in* paragraph must no longer rest on the interval-sampling sentence, because that is not what the research now says its limit is. **What is new and worth recording:** the traced instrument is a route that **could** answer [Section 15](../Download.md#15-unresolved) item 17's parked question, where the folder-watching instrument could not; whether to pose it is the runner's and nothing is decided here. Both lines route to *Writing 11*. `Preparation/Blender Research/Comparison/Writing.md` question 11 and `File output nodes.md` question 10 and 13 were the other places looked; none claims an absence of overlap and none tested for one. `Preparation/Groundwork/Open Threads.md` carries the same finding from `Download` Round 1, 2026-08-20, in the same terms.

**What breaks if it is wrong:** J3 judges frame N finished with a write to it still pending, every expected file present and non-zero, and the file is fetched torn, recorded *fetched*, and never fetched again under R9. This is the one place in the document where a wrong judgement fetches wrong rather than late. [Section 15](../Download.md#15-unresolved) item 17 carries it, with the question posed in the shape the research would need.

**What holds without it:** the three observations it is drawn from, each observed in all four versions on both machines; the ending walk, which judges by the ending and not by order; the zero-byte rule; and J2's refusal to run mechanism one on an order it could not read.

### Whether a volume identity can be read on the user's own computer — unresolved

**Unresolved.** [Section 15](../Download.md#15-unresolved) item 3 parks it beside the mount-point-survived detach: whether a volume identity can be read at all on Mac, Windows and Linux, which is what any detection of that case would rest on.

**What was looked in, and what was found.** The document's own statement is that *"no cited document carries it"*. `Desktop app and delivery` F15 carries a **file** identity per system with its citations — the `st_dev`/`st_ino` pair on macOS and Linux, the 128-bit file identifier with the volume serial number on Windows — which is not the same reading and is not offered as one. The only path-to-mount resolver anywhere in the project is `Server protocol and transfers` F11's machine-side reading, held unresolved at that document's §12 item 3.

**What breaks:** nothing detects the mount-point-survived detach, so a frame lands on the underlying volume, the account records *fetched*, and R9 makes that permanent.

### That real frames run longer than the one measured — an assumption

**Assumption.** W2's second constraint reads *"10 seconds is short against a frame: the one frame cost on record is about 16.8 seconds (Fact 55), and real frames run longer."*

**What was looked in, and what was found.** Fact 55 establishes one frame at about 16.8 seconds on one machine, at one output format and one picture size — its own *Not covered* line. Fact 71's driven writes are at 1920×1080 and are not renders timed for this. No document in `Preparation/` establishes a floor for a real user's frame cost, and none was found that states one.

**What breaks:** nothing fetched wrong. A scene whose frames finish faster than the 10-second cadence has several frames complete between two walks and its files come home a few frames behind rather than one — latency and not a judgement. [Section 15](../Download.md#15-unresolved) item 16 carries the fact's limits.

**What holds:** the cadence's other two constraints — measured from the end, and the duty-cycle bound — and [Section 4](../Download.md#4-telling-a-finished-file-from-one-still-being-written-rows-211-and-203)'s judgement, which is order-based and indifferent to what a frame costs.
