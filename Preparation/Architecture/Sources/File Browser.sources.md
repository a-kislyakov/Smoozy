# Sources — File Browser

*Where every outside-world fact [File Browser](../File%20Browser.md) leans on came from — the owning document, the exact words, the addresses, the dates, and what each does not cover. A decision Smoozy made gets no entry.*

**Three shapes and no fourth.** **(1) Owned by an input document of this stage** — a frozen observation document under `Preparation/Architecture/`, or [Facts.md](../Facts.md)'s register entry over it. The entry names that document, quotes the fact in the words the document uses, and says what the observation does not cover. **(2) A publication quoted directly** — the exact words, the address, the date read. **(3) No source in this project** — recorded as an assumption or unresolved, with what breaks and where it was looked for. **A shape-(3) entry becomes a shape-(2) entry the moment someone reads the publication and records the address**, which is what happened to entry **D4** on 2026-08-23; the sentence here used to read *Nothing here was looked up outside `Preparation/`*, and that stopped being true of this file the first time an entry carried an address and a date read.

**Blender facts are pointed at, never re-derived.** Where the document leans on Blender the entry names the joined answer or the comparison question the document itself names, and stops.

**Entries are in document order.** Where one fact serves several clauses it has one entry, and the clauses are named in it.

---

## A. What binds this before anything is decided

### A1. Nothing on the machine distinguishes a part-arrived file from a whole one (observed)

**Clauses:** the preamble's first bullet, [X10](../File%20Browser.md#44-the-partial-never-counted-row-197-and-row-191s-exclusion), [X11](../File%20Browser.md#44-the-partial-never-counted-row-197-and-row-191s-exclusion), [X13](../File%20Browser.md#44-the-partial-never-counted-row-197-and-row-191s-exclusion), [U9](../File%20Browser.md#34-the-live-half-the-end-at-close-and-the-fresh-sitting-rows-190-and-197).

**Owner:** [Six machines](../Six%20machines%20—%20the%20container,%20the%20webhook%20and%20the%20ready%20action.md), *What a transfer and a restart leave*, **Fact 18**; register entry [Facts.md](../Facts.md) Fact 18, **2026-08-15/16**. Its words: killed and dropped transfers leave bytes at the final name with *"identical mode, permissions, extended attributes, inode flags, allocation and file-type output"* to a whole file, and *"Nothing on the machine distinguishes them."* The overwrite case: 40,000,000 bytes of one file overwritten by a killed transfer of another left a **40,000,000-byte hybrid** — *"10,444,800 new bytes spliced onto 29,555,200 old ones"* — *"Neither version, plausible size, no marker."*

**Not covered:** the observation is six machines on two days in one batch; nothing distinguishes a **damaged** arrival from a whole one either, which is Fact 72's and not this one's (register split note); and no other transfer mechanism than the two tools that batch carried.

### A2. Seventeen hostile names out, fourteen back, and the loss is silent (observed)

**Clauses:** the preamble's second bullet, [N2](../File%20Browser.md#52-a-folder-as-a-folder-and-the-names-row-186), [X5](../File%20Browser.md#42-the-rule-collisions-are-judged-under-row-192), [P6](../File%20Browser.md#22-the-elevated-retry-row-180)'s newline case.

**Owner:** [One box with Blender on it](../One%20box%20with%20Blender%20on%20it%20—%20the%20reading,%20the%20listing,%20and%20the%20names%20that%20do%20not%20come%20back.md) **§7.1**, **2026-08-17**; register entry Fact 29. Its words: *"All seventeen hostile names were written on the rental successfully — newline, tab, colon, backslash, leading dash, emoji and a 255-character name among them. **Linux took every one. Seventeen went out and fourteen came back.**"* The three destroyed: `JOBFILE.png`, `JobFile.png`, `jobfile.png` held distinct on the rental, one file on the Mac *"holding the **last** payload fetched"*, and the composed/decomposed pair the same way. *"**Every fetch reported success** — no error, no warning, no exception."* The control in the same sitting proves *"the loss is at the Mac end and not in transit"*. *"Everything else survived byte for byte: spaces, leading and trailing spaces, Cyrillic, Japanese, emoji, colon, backslash, quote, newline, tab, leading dash, and the 255-character name."*

**Corroborated on three more boxes**, [The multi-provider batch](../The%20multi-provider%20batch%20%E2%80%94%20the%20surface%20that%20did%20not%20vary,%20the%20Debian%20route%20that%20exists,%20and%20the%20cards%20a%20slice%20cannot%20see.md), **2026-08-22**: fifteen name forms — a newline, a carriage return, a tab, a leading dash, a space, a non-UTF-8 byte sequence, a 250-character name, a capitals-only difference, a double quote and apostrophe, a backslash, an asterisk, a semicolon, a dollar sign, a format-specifier string, an astral emoji — *"All fifteen landed, listed and read back."*

**Not covered:** one Mac, *"one case-insensitive volume at default settings — **a case-sensitive volume would behave differently and was not posed.**"* The **down** direction's loss belongs to Row 303 and `Download`, and [N2](../File%20Browser.md#52-a-folder-as-a-folder-and-the-names-row-186) says so; nothing here is evidence about it. Windows is untouched.

### A3. The name and path refusals, and the make-parents silent collision (observed)

**Clauses:** the preamble's third bullet, [N3](../File%20Browser.md#52-a-folder-as-a-folder-and-the-names-row-186), [N4](../File%20Browser.md#52-a-folder-as-a-folder-and-the-names-row-186), [X9](../File%20Browser.md#43-keep-both-rows-192-and-186), [M9](../File%20Browser.md#75-new-folder-row-192s-quiet-path), [Section 13](../File%20Browser.md#13-unresolved) item 3.

**Owner:** [One box with Blender on it](../One%20box%20with%20Blender%20on%20it%20—%20the%20reading,%20the%20listing,%20and%20the%20names%20that%20do%20not%20come%20back.md) **§7**, **2026-08-17**. Its words: *"**Longest single name: 255 characters.** 256 was refused at open with `SSH_FX_BAD_MESSAGE (5) 'Bad message'`, as was every length up to 270. It matches the `f_namemax = 255` the extension reports. **Nothing was left behind.**"* And *"**Longest whole path: 4095.** 4096 was refused at open with the same status. Making a directory refused at path length 4240."* And *"the refusal for both over-limit cases is `Bad message`, which reads as protocol corruption rather than *name too long*. Anything surfacing that string to a user reports the wrong problem entirely. **Both refuse at open, before any byte is sent.**"* On two jobs given one label: the file-transfer channel refused the second directory creation with *"`SSH_FX_FAILURE (4) 'Failure'`, a **generic** code"*; the command channel *"names it exactly"* (`mkdir: cannot create directory … : File exists`, `rc=1`); *"the make-parents form returned **`rc=0`** against the already-existing folder"* — *"the silent collision path"*.

**The unit is bytes, and that half is the other batch's.** [Six machines](../Six%20machines%20—%20the%20container,%20the%20webhook%20and%20the%20ready%20action.md), Fact 29: *"255 **bytes** per name (85 CJK characters)"*, and *"A path limit of 4096 bounds **the string handed to a system call, not the tree**"*. [Facts.md](../Facts.md) Fact 29 records the correction outright — the 2026-08-17 tester posed the length in ASCII and reported characters, and *"A length check written in characters passes a name the machine refuses."*

**Corroborated**, [The multi-provider batch](../The%20multi-provider%20batch%20%E2%80%94%20the%20surface%20that%20did%20not%20vary,%20the%20Debian%20route%20that%20exists,%20and%20the%20cards%20a%20slice%20cannot%20see.md) **2026-08-22**, on three machines across three sellers: *"Name length | **255 accepted, 256 refused** `Bad message`"*, and `statvfs@openssh.com` reporting *"`f_namemax` **255**"* on all three.

**The document was brought into line with these words on 2026-08-23.** The accepted maximum is **255** and the refusal is at **256**; the accepted path is **4095** and the refusal is at **4096**. The preamble's third bullet, [N4](../File%20Browser.md#52-a-folder-as-a-folder-and-the-names-row-186) and [X9](../File%20Browser.md#43-keep-both-rows-192-and-186) all called 255 and 4095 *the observed refusals*, which states each bound as the refused length rather than as the accepted one; **a pre-flight implemented literally from that wording rejects a 255-byte name the machine accepts, and X9's trim would have shortened a stem to 254 bytes for no reason.** All three now say *accepted* and name 256 and 4096 as the refusals. This entry recorded the mismatch before the repair and is left saying so.

**Not covered:** four Linux rentals and no other kind of filesystem; the 4095 path figure is one box's and is Linux's, not universal ([Section 13](../File%20Browser.md#13-unresolved) item 3 and entry K1 below).

### A4. A listing yields all four fields in one reading, and what it costs (observed)

**Clauses:** [L1](../File%20Browser.md#11-one-reading-on-the-subsystem-route-rows-183-and-191), [L6](../File%20Browser.md#12-the-clock-and-the-size-row-183), [X1](../File%20Browser.md#41-the-walk-row-191), [P6](../File%20Browser.md#22-the-elevated-retry-row-180)'s command-channel figure, [Section 13](../File%20Browser.md#13-unresolved) item 11.

**Owner:** [One box with Blender on it](../One%20box%20with%20Blender%20on%20it%20—%20the%20reading,%20the%20listing,%20and%20the%20names%20that%20do%20not%20come%20back.md) **§5.1** and **§5.4**, **2026-08-17**; register entry Fact 7. Its words: *"All four fields come back in one reading on both routes, and the size costs no second round trip … a 15-entry listing cost **zero per-file stats**, and 5010 entries cost **52 round trips rather than 5010** — which would have been 651 seconds against the 18.22 measured. On the command channel the same four fields cost **about a quarter of a second for 5010 entries**."* And *"the zero-length file reports 0 on both routes at no extra cost."* Kind, apparent size, blocks allocated, epoch modification time and name all came back in one reading.

**Not covered:** one container on `overlayfs`, one provider, the proxy route only, one unprivileged login shape. The document's own limit on every timing: *"three testers shared four cores"* at load average 2.5 to 6.5, so *"Every timing here is an upper bound rather than a measurement of an idle machine."* [Section 13](../File%20Browser.md#13-unresolved) item 11 carries that.

### A5. Which clock the last-changed time is on (observed)

**Clauses:** [L4](../File%20Browser.md#12-the-clock-and-the-size-row-183).

**Owner:** [One box with Blender on it](../One%20box%20with%20Blender%20on%20it%20—%20the%20reading,%20the%20listing,%20and%20the%20names%20that%20do%20not%20come%20back.md) **§5.2**; register entry Fact 7: *"**No timezone is configured at all on this container** — the `+0000` is the C library's fallback in the absence of configuration, so a provider image that set one would render those times in its zone, and **the epoch form is the unambiguous one**. Nothing inside disciplines the clock. The two ends **agree to better than half a second and no more finely than that**, bounded by a ±0.3 s round trip. A hand-set time round-trips exactly at nanosecond resolution."*

**Not covered:** one image. A provider image carrying a timezone renders those times in its zone, which is exactly why [L4](../File%20Browser.md#12-the-clock-and-the-size-row-183) takes the epoch form. The container has no clock of its own; Fact 92 is unanswered at the length of a render.

### A6. The sparse file's lie, and which route can catch it (observed)

**Clauses:** [L5](../File%20Browser.md#12-the-clock-and-the-size-row-183).

**Owner:** [One box with Blender on it](../One%20box%20with%20Blender%20on%20it%20—%20the%20reading,%20the%20listing,%20and%20the%20names%20that%20do%20not%20come%20back.md) **§5.3**: a 500 GB sparse file on a 60 GB allocation reported `536870912000` with **0 blocks allocated**. *"**Detectable on the command-channel route**, as one extra field in the same reading. **Undetectable on the file-transfer route**, which carries no allocated-block count at all."*

**Not covered:** one filesystem, `overlayfs`; nothing about what any other route would report.

### A7. The three listing failures are separable on the subsystem route and not by exit status on the command channel (observed)

**Clauses:** [L3](../File%20Browser.md#11-one-reading-on-the-subsystem-route-rows-183-and-191), [P1](../File%20Browser.md#21-the-limit-and-the-walk-that-recovers-the-separation-row-181), [P2](../File%20Browser.md#21-the-limit-and-the-walk-that-recovers-the-separation-row-181), [P3](../File%20Browser.md#21-the-limit-and-the-walk-that-recovers-the-separation-row-181), [X2](../File%20Browser.md#41-the-walk-row-191), [Section 13](../File%20Browser.md#13-unresolved) item 1.

**Owner:** [One box with Blender on it](../One%20box%20with%20Blender%20on%20it%20—%20the%20reading,%20the%20listing,%20and%20the%20names%20that%20do%20not%20come%20back.md) **§5.5** and **§5.6**, **2026-08-17**; register entry Fact 7.

- **The trap first.** *"**Root lists the root-owned private folder at exit 0.** Posed as root this reads as success and proves nothing."* Everything else is the non-root login.
- **On the subsystem route, distinct statuses with a control either side.** *"CONTROL: sent status Success … CASE 1: opendir "…denied/" → sent status Permission denied … CASE 2: lstat ".../gone_folder" → sent status No such file … CONTROL AGAIN: sent status Success"*. *"The client's exit is 1 for both; **the protocol status distinguishes them**, and any library exposes it."*
- **On the command channel it is not separable by status.** *"`exit(denied)=2   exit(gone)=2   -> SAME: exit status alone CANNOT tell them apart`"*, and only the English message separates them, *"stable **here** only because the image carries `C`, `C.utf8` and `POSIX` and the language variable is unset."* **Corrected in that document 2026-08-22:** a `stat` and a file test on a path **under** an unreadable parent both exit 1 either way; only a walk of the parent components separates them, and *"a case not posed on this box."*
- **The machine did not answer:** all four shapes at exit **255** with a client-written message.
- **Partial output before the connection dies:** *"genuinely short (3 entries), uncut: 4 lines EXIT: 0 / cut (starts big, dies early): 4 lines EXIT: 255"* — *"Anything that treats *output arrived* as *listing complete* is wrong on this machine."*

**Not covered, and [Section 13](../File%20Browser.md#13-unresolved) item 1 carries it:** *"one container, `overlayfs`, the proxy route only, one unprivileged login shape with no groups, access lists or read-only mounts."* The Linux form of the path-under-an-unreadable-parent case is **unposed** (register entry Fact 7).

### A8. A full disk refuses one way, and nothing can be written afterwards (observed)

**Clauses:** the preamble's fifth bullet, [X11](../File%20Browser.md#44-the-partial-never-counted-row-197-and-row-191s-exclusion), [M6](../File%20Browser.md#73-move-row-192-and-the-partial-action), [L11](../File%20Browser.md#14-the-free-space-figure-row-184).

**Owner:** [Two more boxes](../Two%20more%20boxes%20—%20the%20volume,%20the%20full%20disk,%20and%20a%20day%20left%20running.md), **2026-08-16**; register entry Fact 28. Its words: every write past the fill was refused with *"`No space left on device`"* — a shell redirect, `cp`, and a Python write with an explicit flush and `fsync` that raised **errno 28**. On what a reading has to exclude: *"A plain `df` on this container returned **51 filesystems**. Forty-eight are `tmpfs`"*, and one of the three that are not is *"the **host's** disk, bind-mounted in to deliver the driver files … It is not the rental's storage and nothing may be written to it, but it looks exactly like a disk with 20 GiB free."*

**A part-written file at a final name.** [The rented machine](../The%20rented%20machine.md): the file `dd` was writing when the disk filled *"**stays on disk under its final name**"*, at the size it reached, with nothing marking it incomplete.

**Not covered, and the scope narrowing is the register's own:** the Linux run *"only ever posed the already-full case"* — the volume was filled first — so *"the partway-fill case is not posed on Linux at all"*, and Fact 23 on macOS left a truncated file on every byte-loop run. One volume, one container. [Section 13](../File%20Browser.md#13-unresolved) item 11 carries the one-volume limit.

### A9. Elevation differs by machine kind (observed)

**Clauses:** the preamble's sixth bullet, [P5](../File%20Browser.md#22-the-elevated-retry-row-180)'s four outcomes, [Section 13](../File%20Browser.md#13-unresolved) item 7.

**Owner:** [Six machines](../Six%20machines%20—%20the%20container,%20the%20webhook%20and%20the%20ready%20action.md), *The machines*, Facts 51 and 48, **2026-08-15/16**, and [The last two boxes](../The%20last%20two%20boxes%20—%20the%20hook%20that%20cannot%20be%20read%20back,%20the%20full%20disk,%20and%20the%20unit%20that%20kills%20a%20render.md) §10, **2026-08-18**; register entry Fact 27. Its words: *"**On a virtual-machine rental elevated access is ordinarily present and needs no password**"* — Sesterce uid 1001 non-root with passwordless sudo, Hyperstack uid 1000 the same with root login refused by the provider, Vast handing over real root with a second account holding passwordless sudo. *"One trap: the Vast ordinary account was **in the `docker` group**, which is silent root-equivalence."* And *"root inside a container is not root: it holds **13 of 41 capabilities** (`CapPrm/CapEff/CapBnd = 00000000a80405fb`), without `CAP_SYS_MODULE`, `CAP_SYS_ADMIN`, `CAP_SYS_PTRACE` or `CAP_SYS_BOOT`."*

**The container half as the register words it, and the document now does too** *(heading corrected 2026-08-29: it read "and the document does not", the pre-repair title, while the entry's own body records the repair made on 2026-08-23)*. [Facts.md](../Facts.md) Fact 27 was **corrected 2026-08-22**: *"On a container rental a non-root login's elevation route demands a password Smoozy does not hold — corrected 2026-08-22, where this said *there is no elevation route at all from a non-root login*"* — `sudo -n` demands a password, `/root/onstart.sh` and `/etc/rc.local` are both `Permission denied`, and switching to root fails authentication. The register adds: *"**Nothing downstream changes**: a password Smoozy does not have is as good as no route for the placement's purposes."* **Repaired in the document on 2026-08-23.** The preamble's sixth bullet and [P5](../File%20Browser.md#22-the-elevated-retry-row-180)'s second table row had both attributed the pre-correction sentence — *no elevation route at all* — to Fact 27, and P5's row additionally read *a password no password satisfies*, which the register never said. Both now carry the register's corrected sentence, the three refusals behind it, and the register's own follow-on — *"a password Smoozy does not have is as good as no route"* — which is what keeps the **outcome**, no offer, unchanged. **The conclusion survived; only its stated ground was stale.**

**Not covered, and [Section 13](../File%20Browser.md#13-unresolved) item 7 carries it:** provider images not surveyed; the non-interactive probe never run on a rental in `Server Setup` M3's exact form; the `docker`-group equivalence on one image.

**The elevating command's own documentation is not this document's.** `sudo(8)` 1.9.17p2's `-S` wording is cited at [Server Setup's companion](Server%20Setup.sources.md) and at [the spec's own](../../Specs/Sources/File%20Browser.sources.md); [P5](../File%20Browser.md#22-the-elevated-retry-row-180) names `Server Setup` §9's flow and quotes nothing, so no citation is repeated here.

### A10. Resume is exact in both directions, and the partial can be held off the final name (observed)

**Clauses:** the preamble's seventh bullet, [K4](../File%20Browser.md#92-carrying-on-inside-a-single-file-row-200), [K5](../File%20Browser.md#92-carrying-on-inside-a-single-file-row-200), [Section 14](../File%20Browser.md#14-moments-that-reached-the-creator)'s Row 200 entry.

**Owner:** [Six machines](../Six%20machines%20—%20the%20container,%20the%20webhook%20and%20the%20ready%20action.md), Fact 17, **2026-08-15/16**; register entry Fact 17. Its words: *"Resume was exact — 119,424,000 bytes where 157,286,400 minus 37,862,400 was owed, and 436,786,462 where 669,966,622 minus 233,180,160 was owed — with matching checksums."* On a second provider an interrupted transfer resumed in *"**12.85 seconds against a from-scratch control of 30.97**"*. *"**The part that arrived can be held off the final name**, which is the clause the spec leans on."*

**The two conditions the document declines, in the source's words:** Apple's `rsync` *"accepts every resume flag but one"*, and *"the flag that holds a partial off the final name accepted the flag and resumed nothing on the sending direction on one box"*; and *"the partial directory cannot be relied on after a crash"* — a hard kill left, on one box, *"rsync's own **randomly named temporary file** in the destination instead — a name Smoozy cannot predict."*

**Not covered:** the fetching direction was posed on one machine with two tools, the sending direction on two providers with one tool. Both conditions are about `rsync`; [K5](../File%20Browser.md#92-carrying-on-inside-a-single-file-row-200) states why neither reaches this document's route, and that statement is a decision rather than a fact.

### A11. Fact 30 — a file on the user's own computer changing while it is being read, posed on macOS (observed, 2026-08-20)

**What was posed, and on what.** The Creator's own Mac — macOS 26.5.2, an APFS volume at default settings — with a throwaway harness in a scratch directory outside the project tree. Blender 4.5.12 LTS and Blender 5.0.1, each launched by the throwaway itself with no window; **the Blender instance the Creator had open was not touched, and nothing pre-existing was written to** — the per-version `recent-files.txt` files were checked after the runs and none had moved. The register's own requirement was honoured: the writer was a real Blender save and a real Blender texture export, not a synthetic script write, because the swap-in case is what an application does.

**The harness.** A reader held one file handle open and read the file through slowly in chunks, computing a digest of what the handle delivered — the byte loop's own shape. Beside it, a poller re-read the file's **path identity** — size, modification time in nanoseconds, inode — every 100 ms, and watched the directory's entries. Mid-read, a separate headless Blender was launched to save over the file.

**Result one — a real `.blend` save is a swap-in, and it is one atomic transition at the path.** In both versions, Blender wrote a temporary `name.blend@` beside the file and renamed it over the final name. Observed on the 4.5.12 run (a 159,298,485-byte file): the temporary appeared in the directory, and at the swap the path identity changed in one step — size 159,298,485 → 159,299,150, inode 105231870 → 105231885, modification time moved — caught by the poller within one 100 ms interval, with the previous file left under the version-backup name `A45.blend1` still carrying the old inode. The 5.0.1 run (a 63,492,129-byte compressed save) behaved identically in shape: temporary `A50.blend@`, one atomic identity change, inode 105232062 → 105232066. **In both runs the held-open handle went on delivering the pre-save file, whole, its digest equal to the old file's** — the file the handle reads is no longer the file the name points at, which is the exact case the spec's sources feared would slip past detection, and it is visible at the path in every observed value.

**Result two — a real texture export writes in place, and size and inode both miss it.** Blender 4.5.12's own image save over an existing 13,931-byte PNG: the path identity moved **twice** — first to size 0 (a truncation, same inode 105231869), then back to size 13,931 with a new modification time, **the same inode and the byte-identical size throughout**. Only the modification time distinguishes the new file from the old at the path. The after-digest differed from the before-digest, proving the contents really changed; the held-open handle's digest equalled the old file's only because the differing bytes sat in the already-read prefix — in general such a read is a mixture of old prefix and new tail.

**The controls.** A run with no save produced zero identity-change events and three equal digests — so the watch can fail, and the test is a test. The swap was proved in the same run by the inode change plus the content change, with the old content still on disk under the backup name matching the handle's digest byte for byte.

**What a negative would have looked like:** a real save that moved none of size, modification time or inode as seen by a path reading — a change no poll could see. It did not occur; both save shapes moved the modification time at nanosecond resolution, and the `.blend` save moved all three values.

**What this does not cover.** One Mac, one APFS volume at default settings; Windows and Linux, which have no computer behind them in this project; applications other than Blender, whose save shapes were not posed; a writer that deliberately restores the modification time (observed possible on Linux in the Fact 72 batch), which no path identity catches and which the document's contents gate exists for; and clock or timestamp granularity on any other filesystem. The harness, its scripts and its raw JSON output were throwaway and are not kept; the figures above are transcribed from the run output of 2026-08-20.

---

## B. Section 1 — the listing

### B1. A directory listing that decodes entry names eagerly fails on a name the machine handles (observed)

**Clauses:** [L1a](../File%20Browser.md#11-one-reading-on-the-subsystem-route-rows-183-and-191), and through it [X1](../File%20Browser.md#41-the-walk-row-191)'s machine side.

**Owner:** [The multi-provider batch](../The%20multi-provider%20batch%20%E2%80%94%20the%20surface%20that%20did%20not%20vary,%20the%20Debian%20route%20that%20exists,%20and%20the%20cards%20a%20slice%20cannot%20see.md), **2026-08-22**. Its words: *"A directory listing that decodes entry names as UTF-8 **raises** on the non-UTF-8 name — `UnicodeDecodeError('utf-8', b'bad\xff\xfe.txt', 3, 4, 'invalid start byte')` — while the write, the read-back and the machine's own `ls` all handled it. **So the enumeration must carry names as bytes and decode them only for display**; a listing that decodes eagerly fails on a name Smoozy is otherwise able to move. This is a constraint on `File Browser`'s walk, observed rather than reasoned."* The document quotes the exception string character for character.

**Not covered:** it is a property of one decoding client, not of any server; three rentals across three sellers all wrote, listed and read the name back. `Server protocol and transfers` [§12 item 12](../Server%20protocol%20and%20transfers.md#12-unresolved) records it as the second in-Smoozy instance of the same hazard and names this document as its owner.

### B2. A newline in a file name splits a listing across lines on both routes (observed)

**Clauses:** [L2](../File%20Browser.md#11-one-reading-on-the-subsystem-route-rows-183-and-191), and [P6](../File%20Browser.md#22-the-elevated-retry-row-180)'s separator rule.

**Owner:** [One box with Blender on it](../One%20box%20with%20Blender%20on%20it%20—%20the%20reading,%20the%20listing,%20and%20the%20names%20that%20do%20not%20come%20back.md) **§10 item 1**, **2026-08-17**: *"**A newline in a file name splits a listing across lines** on both routes. Line-oriented parsing of a listing is unsafe."* The same section's item 2: *"**The file-transfer client's text output is lossy though the protocol is not** — no year or seconds for recent entries, no timezone ever, symlink targets dropped, and non-Latin names escaped into octal."*

**Not covered:** one box, one client at `OpenSSH_10.2p1` against one server at `8.9p1`. The finding is about **rendered text**, not about the protocol's own fields, which is exactly the distinction [L2](../File%20Browser.md#11-one-reading-on-the-subsystem-route-rows-183-and-191) turns on.

### B3. Row 182 — the `.blend` reading's Blender ground, pointed at rather than copied

**Clauses:** [L7](../File%20Browser.md#13-the-blend-version-row-182)–[L10](../File%20Browser.md#13-the-blend-version-row-182), and [Section 13](../File%20Browser.md#13-unresolved) item 2.

Everything the document leans on here is owned by the Blender Research comparison at **_Reading a file 1_**, reached through [What a file can be asked before it renders](../../Blender%20Research/Across%20topics/What%20a%20file%20can%20be%20asked%20before%20it%20renders.md), and by [the spec's own sources file](../../Specs/Sources/File%20Browser.sources.md), which already cites it for the spec. The pieces the document names:

- the two opening-text shapes, `BLENDER-v405REND` against `BLENDER17-01v0500REND` with the version digits moving — **observed**, the same in all four versions on both machines;
- **that a future header shape is a documented possibility rather than a guess** — [L8](../File%20Browser.md#13-the-blend-version-row-182)'s *5.0's release notes say outright that parsers have to be updated when the header changes* is *Reading a file 1*'s own reading of those release notes;
- the from-5.0 compression, and that outside tools can undo it — **documented**, the same question;
- that no version's documentation describes the file's inside;
- that nothing outside Blender was ever pointed at a `.blend` in the research.

They are named here only so this file lists every outside fact the document leans on; their citations live where their owners put them. **No Blender answer is assembled here out of several topics, and no topic document is quoted.**

**Not covered, and [Section 13](../File%20Browser.md#13-unresolved) item 2 carries it:** Windows was never tested; a future or unobserved header shape is untried; and what a program that is not Blender can read out of a `.blend` is the research's own open question.

### B4. The free-space figure's route and its shape (owned elsewhere, named here)

**Clauses:** [L11](../File%20Browser.md#14-the-free-space-figure-row-184)–[L13](../File%20Browser.md#14-the-free-space-figure-row-184).

The mechanism is `Server protocol and transfers` F1–F3 and F10/F16 and is that document's; what is outside-world here is the surface it rests on.

- **Both free-space routes exist on a rental, and the extension is announced.** [One box with Blender on it](../One%20box%20with%20Blender%20on%20it%20—%20the%20reading,%20the%20listing,%20and%20the%20names%20that%20do%20not%20come%20back.md) **§3.4**, 2026-08-17 (register Fact 93): *"**Both free-space routes are available under the non-root login**"* — and both reported the whole container overlay rather than a per-path allowance. [The multi-provider batch](../The%20multi-provider%20batch%20%E2%80%94%20the%20surface%20that%20did%20not%20vary,%20the%20Debian%20route%20that%20exists,%20and%20the%20cards%20a%20slice%20cannot%20see.md), 2026-08-22: *"`statvfs@openssh.com` | **succeeded**"* on all three machines.
- **Why the available figure and never the free figure.** The root-reserve ground is `mke2fs(8)`'s, cited in full at [Server protocol and transfers' companion](Server%20protocol%20and%20transfers.sources.md) — read at the e2fsprogs source tree, `https://git.kernel.org/pub/scm/fs/ext2/e2fsprogs.git/plain/misc/mke2fs.8.in`, read **2026-08-23**: *"Specify the percentage of the file system blocks reserved for the super-user … The default percentage is 5%."* Not repeated further here; this document consumes F3 and quotes nothing.
- **What a reading has to exclude** is entry **A8**'s: 51 filesystems, 48 of them `tmpfs`, and the host's bind-mounted disk.

**Not covered:** what any given rental's root filesystem was made with, including whether it is an ext filesystem at all — a container rental's root is `overlay`. No figure here is a statement about the machine, which is F16's rule and a decision.

---

## C. Section 2 — permission, and the elevated retry

### C1. The protocol's two status codes, quoted (documented)

**Clauses:** [P1](../File%20Browser.md#21-the-limit-and-the-walk-that-recovers-the-separation-row-181), and through it [P2](../File%20Browser.md#21-the-limit-and-the-walk-that-recovers-the-separation-row-181)–[P4](../File%20Browser.md#21-the-limit-and-the-walk-that-recovers-the-separation-row-181).

`draft-ietf-secsh-filexfer-02`, *SSH File Transfer Protocol* — the revision-3 specification the negotiated protocol names. Quoted at `Server protocol and transfers` [§1.3](../Server%20protocol%20and%20transfers.md#13-what-the-surface-does-not-carry), read **2026-08-20**: `SSH_FX_NO_SUCH_FILE 2`, *"returned when a reference is made to a file which should exist but doesn't"*, and `SSH_FX_PERMISSION_DENIED 3`, *"returned when the authenticated user does not have sufficient permissions to perform the operation."* **Address:** `https://datatracker.ietf.org/doc/id/draft-ietf-secsh-filexfer-02.txt`, the address that document's own companion records (read there 2026-08-22). [P1](../File%20Browser.md#21-the-limit-and-the-walk-that-recovers-the-separation-row-181) names both codes and quotes neither definition; the words above are the specification's, recorded here because the document gives none.

**Not covered:** what a server that is not OpenSSH's returns; the specification defines the codes and says nothing about which one any implementation chooses.

### C2. A real subsystem returning each code, with a control on each side (observed on the caller's own end)

**Clauses:** [P1](../File%20Browser.md#21-the-limit-and-the-walk-that-recovers-the-separation-row-181)'s *a real subsystem was made to return each of them with a control on 2026-08-20*.

**Owner:** `Server protocol and transfers` [§1.3](../Server%20protocol%20and%20transfers.md#13-what-the-surface-does-not-carry) and its [Sources companion](Server%20protocol%20and%20transfers.sources.md). The run was re-performed on **2026-08-23** on the Creator's Mac — macOS 26.6.2 build 25G83, `OpenSSH_10.3p1`, driving its own `/usr/libexec/sftp-server` directly over pipes. An attribute request on a file beneath an unreadable parent returned **status 3**; the same request on an absent path returned **status 2**; and the readable control returned attributes. **What a negative would have looked like:** the first two returning the same status, or either returning attributes. The same clause records that the byte-identical rendering belongs to the transfer **client program**, which expands the path before it asks, and that this collapse was reproduced on the same rig. Label: **observed**, on the caller's own end.

**Not covered, and this is the remaining limit:** no rented machine's file-transfer server has been posed at this stat case. `Server protocol and transfers` [§12](../Server%20protocol%20and%20transfers.md#12-unresolved) item 24 carries that narrower unresolved ground and what would break if a provider image answered differently.

**What the document may claim while resting on it, settled 2026-08-23.** [P1](../File%20Browser.md#21-the-limit-and-the-walk-that-recovers-the-separation-row-181) said *a real subsystem was made to return each of them with a control on 2026-08-20*, which let one thin run stand for both halves of the separation. It now splits them:

- the **readdir** half is Fact 7's own rental result (entry **A7**) — `opendir` on an unreadable folder sent **Permission denied**, `lstat` on an absent path sent **No such file**, with a successful reading either side — and that is the half [P2](../File%20Browser.md#21-the-limit-and-the-walk-that-recovers-the-separation-row-181) and [P3](../File%20Browser.md#21-the-limit-and-the-walk-that-recovers-the-separation-row-181)'s component walk runs on;
- the **stat-refusal** half — an attribute request on a file under an unreadable parent answering *refused* rather than *absent* — is observed on the caller's own end with its rig, controls and negative recorded, and Fact 7 does not pose it on a rental. Its command-channel twin is recorded in that document as *"a case not posed on this box"* (entry **A7**).

**What holds on an unposed rental:** [P3](../File%20Browser.md#21-the-limit-and-the-walk-that-recovers-the-separation-row-181)'s walk names the refused component from readdir statuses alone, and [P4](../File%20Browser.md#21-the-limit-and-the-walk-that-recovers-the-separation-row-181)'s residue message is written to be true either way. **What narrows is how early Smoozy knows which of the two it met on a path it was handed, not whether it ever knows.**

### C3. What a per-entry probe could and could not answer (no source — an assumption)

**Clauses:** [P2a](../File%20Browser.md#21-the-limit-and-the-walk-that-recovers-the-separation-row-181)'s third bullet — *"The question is *can **this login** read into this folder*, which mode bits do not answer: supplementary groups, access-control lists and the login's own privilege all bear on it."*

**No source for it exists anywhere in `Preparation/`.** Looked for in: [Facts.md](../Facts.md) (Facts 7 and 27 and their register entries), [One box with Blender on it](../One%20box%20with%20Blender%20on%20it%20—%20the%20reading,%20the%20listing,%20and%20the%20names%20that%20do%20not%20come%20back.md) §5.5, [Six machines](../Six%20machines%20—%20the%20container,%20the%20webhook%20and%20the%20ready%20action.md), [The multi-provider batch](../The%20multi-provider%20batch%20%E2%80%94%20the%20surface%20that%20did%20not%20vary,%20the%20Debian%20route%20that%20exists,%20and%20the%20cards%20a%20slice%20cannot%20see.md), every `Sources/` file under this folder, and the Specs sources. **The nearest thing the project holds is a silence rather than a statement:** Fact 7's own limit is *"one unprivileged login shape with no groups, access lists or read-only mounts"* — that says those shapes were **not posed**, not that they bear on the answer.

Label: **an assumption.** **What breaks if it is wrong:** nothing built. [P2a](../File%20Browser.md#21-the-limit-and-the-walk-that-recovers-the-separation-row-181)'s conclusion — no per-entry marking, the refusal met on the click — is carried independently by [L1](../File%20Browser.md#11-one-reading-on-the-subsystem-route-rows-183-and-191)'s and [X1](../File%20Browser.md#41-the-walk-row-191)'s measured prohibition on a stat per entry (entry **A4**: 52 round trips against 5010). If mode bits did answer the question, the design would still be the same design and the reason would be cost alone. **What is lost is one of two reasons, not the answer.**

### C4. The wrapped file-transfer server — OpenSSH's own three pages (documented, read 2026-08-23)

**Clauses:** [P6](../File%20Browser.md#22-the-elevated-retry-row-180)'s first paragraph.

Read at the pages themselves on 2026-08-23, in two forms that agree word for word: as shipped on the Creator's Mac — macOS 26.6.2, **OpenSSH_10.3p1, LibreSSL 3.3.6**, `man 8 sftp-server`, `man 5 sshd_config`, `man 1 sftp` — and at OpenBSD's own published manual.

- **The file-transfer server is an ordinary program.** *sftp-server(8)*, DESCRIPTION, first sentence: *"sftp-server is a program that speaks the server side of SFTP protocol to stdout and expects client requests from stdin."* Address: `https://man.openbsd.org/sftp-server.8`.
- **The subsystem is a command rather than a fixed thing.** *sshd_config(5)*, `Subsystem`: *"Configures an external subsystem (e.g. file transfer daemon).  Arguments should be a subsystem name and a command (with optional arguments) to execute upon subsystem request."* Address: `https://man.openbsd.org/sshd_config.5`.
- **The client will speak the protocol to a named program instead.** *sftp(1)*, `-D sftp_server_command`: *"Connect directly to a local sftp server (rather than via ssh(1)).  A command and arguments may be specified, for example "/path/sftp-server -el debug3".  This option may be useful in debugging the client and server."* Address: `https://man.openbsd.org/sftp.1`.

**All three match the document character for character**; P6 quotes the first whole, the second from *"a subsystem name"*, and the third to *"A command and arguments may be specified"*.

**Not covered:** anything about a rental. No page says whether OpenSSH passes a value that is not a bare path through the remote shell, whether a given rental carries both an elevating command and a file-transfer server at a knowable path, or whether P5's probe would reach it — Section 13 item 9's own subject, and the document says so. **That subject is permanently unresolved from 2026-09-02**, the Creator having ruled on 2026-08-31 that there will be no further rental to answer a question: no page can answer what an arbitrary rental carries, and no machine is coming to be asked. Section 13 items 9 and 16 state what that costs. `man.openbsd.org` serves OpenBSD-current and can run ahead of a shipping OpenSSH, which is why the shipped 10.3p1 pages were read beside it.

### C5. The separator byte no filename may hold (documented, read 2026-08-23)

**Clauses:** [P6](../File%20Browser.md#22-the-elevated-retry-row-180)'s separator paragraph, and [Section 13](../File%20Browser.md#13-unresolved) item 9's 2026-08-21 narrowing.

- **A filename may hold any byte but two.** POSIX.1-2024 (The Open Group Base Specifications Issue 8), **3.146 Filename**: *"A sequence of bytes consisting of 1 to {NAME_MAX} bytes used to name a file. The bytes composing the name shall not contain the <NUL> or <slash> characters."* Address: `https://pubs.opengroup.org/onlinepubs/9799919799/basedefs/V1_chap03.html`, read 2026-08-23. Exact as P6 quotes it; the section number is recorded here because the document gives none.
- **The channel carries that byte.** RFC 4251, **§5 Data Type Representations Used in the SSH Protocols**, on the `string` type: *"Arbitrary length binary string.  Strings are allowed to contain arbitrary binary data, including null characters and 8-bit characters."* Address: `https://www.rfc-editor.org/rfc/rfc4251.html`, read 2026-08-23. P6 quotes the second sentence, exact.
- **Programs on the Creator's own Mac already use it, documented.** `find(1)`, `-print0`, as shipped on macOS 26.6.2: *"This primary always evaluates to true.  It prints the pathname of the current file to standard output, followed by an ASCII NUL character (character code 0)."* And `xargs(1)`, `-0, --null`: *"Change xargs to expect NUL (``\0'') characters as separators, instead of spaces and newlines.  This is expected to be used in concert with the -print0 function in find(1)."* Address: the manual pages shipped on the disk, `man 1 find` and `man 1 xargs`, read 2026-08-23. P6's fragment is exact.

**Not covered:** whether an **arbitrary rental** offers any program that emits such a format — Section 13 item 9's own subject, and the document deliberately names no program. Nothing above is evidence about a program's behaviour on a machine Smoozy would be reading: three are definitions, and the fourth is a `find` shipped on a Mac.

### C6. P6's two runs — no observation record exists, and both are labelled unresolved (2026-08-23)

`Method`'s *a fact got by running something* asks for what was done, on which machine, what came back word for word, a line saying what a negative would have looked like, and a machine whose exact description is written down. **Two sentences in P6 reported a run and none of that is anywhere** — not in this file, not in `Facts.md`, not in any input document. Both are recorded here as unresolved rather than reconstructed, and P6 now says so at each place.

1. *"Posed on the Creator's Mac, 2026-08-20, with no network: a full protocol session came back through an arbitrary wrapping command standing in for the elevating one."* **No record was kept and none was found.** What it would need is the exact command, the OpenSSH version, the machine's description, the session's own words back, and the negative — a wrapping command through which no session comes back.
2. *"…which was run: a newline-bearing name came back whole with the boundary marker appearing nowhere else, and a four-field record held every boundary."* **Same: no record.** What it would need is the exact `find` and `xargs` invocation, the name used, the bytes back, and the negative — the same name splitting a record whose separator is a byte the name holds.

**What breaks, and it is bounded.** Neither claim is load-bearing. The first is used only to say the wrapped route is *not documented shut*, and P6 then states outright that reachability on a rental is unestablished and permanently so — Section 13 item 9 carries the permanence line and no batch is coming, the Creator having ruled on 2026-08-31 that there will be no further rental to answer a question *(corrected 2026-09-02: this read "and rides Section 13 item 9's batch")* — so what survives is OpenSSH's three sentences above and nothing more. The second is used only to say the separator hazard is a property of a choice of separator, and POSIX.1-2024's own filename definition together with RFC 4251 §5 carry that without any run. **What is lost is the demonstration, not the conclusion.** Re-posing either is free and local and neither needs a rental; until one is posed and written down, both stand as unresolved here.

### C7. No OpenSSH manual documents the diagnostic strings, and 255 is the whole exit contract (observed and documented)

**Clauses:** [P6](../File%20Browser.md#22-the-elevated-retry-row-180)'s output-discipline sentence — *"never an exit status alone where 255 is possible (Fact 7, Fact 37)"* — and, through `Server protocol and transfers` §1.1, every place this document refuses to mine a tool's text.

**Owner:** [The manuals — keys, secrets, and what cannot be told apart](../The%20manuals%20—%20keys,%20secrets,%20and%20what%20cannot%20be%20told%20apart.md), **2026-08-16**; register entry Fact 37. Its words: *"`ssh(1)`'s entire exit-status contract is "255 if an error occurred", `ssh-keygen(1)` has no EXIT STATUS section at all, and **no OpenSSH manual page documents a single one of the diagnostic strings the tools emit** — verified by searching seven pages. Every distinction therefore rests on parsing undocumented text."*

**Not covered:** *"All observed on one build against one throwaway server; nothing here is documented, so nothing is guaranteed stable."* The finding is about the caller's own tools, not about a rental's.

---

## D. Section 4 — already exists, the collision rule, and the partial

### D1. Every rental's filesystem told capitals apart (observed)

**Clauses:** [X5](../File%20Browser.md#42-the-rule-collisions-are-judged-under-row-192), and through it [X9](../File%20Browser.md#43-keep-both-rows-192-and-186), [M8](../File%20Browser.md#74-copy-rows-192-and-197s-shape-machine-locally), [M10](../File%20Browser.md#76-collisions-on-a-move-a-copy-and-a-rename-row-192), [W4](../File%20Browser.md#82-the-comparison-behind-the-warnings-rows-193-and-192).

**Owners, and there are now more of them than [X5](../File%20Browser.md#42-the-rule-collisions-are-judged-under-row-192) counts:**

- [The rented machine](../The%20rented%20machine.md): *"The filesystem **tells capitals apart** — `Case.txt` and `case.txt` are two different files."*
- [One box with Blender on it](../One%20box%20with%20Blender%20on%20it%20—%20the%20reading,%20the%20listing,%20and%20the%20names%20that%20do%20not%20come%20back.md) §7.1: the rental held `JOBFILE.png`, `JobFile.png` and `jobfile.png` as three distinct files, and *"The rental kept every variant distinct."*
- [Six machines](../Six%20machines%20—%20the%20container,%20the%20webhook%20and%20the%20ready%20action.md), Fact 29: the machine's limits are *"255 **bytes** per name (85 CJK characters), case-sensitive"*.
- [The multi-provider batch](../The%20multi-provider%20batch%20%E2%80%94%20the%20surface%20that%20did%20not%20vary,%20the%20Debian%20route%20that%20exists,%20and%20the%20cards%20a%20slice%20cannot%20see.md), 2026-08-22: *"Case sensitivity | **case-sensitive** on all three"*.

**Repaired in the document on 2026-08-23.** [X5](../File%20Browser.md#42-the-rule-collisions-are-judged-under-row-192) said *both boxes that were asked* against the four owners above, the last of them on three machines across three sellers. The clause understated its own evidence rather than overstating it, so **the answer did not move** — only the count of what stands behind it. X5 now names four observation documents and quotes the three-seller batch's *"case-sensitive on all three"*.

**Not covered:** every one of them is Linux. Nothing establishes that a rental's filesystem must be case-sensitive, which is exactly why [X7](../File%20Browser.md#42-the-rule-collisions-are-judged-under-row-192)'s create-intended branch exists; and the **receiving** side on the user's own computer is the opposite result (entry **A2**) and belongs to Row 303.

### D2. The exclusive-create route answers *is this name taken*, and the base rename refuses over an existing target (observed)

**Clauses:** [X6](../File%20Browser.md#42-the-rule-collisions-are-judged-under-row-192), [X7](../File%20Browser.md#42-the-rule-collisions-are-judged-under-row-192)'s create-intended branch, [M4](../File%20Browser.md#72-rename-row-192s-third-face), [M5](../File%20Browser.md#73-move-row-192-and-the-partial-action), [M10](../File%20Browser.md#76-collisions-on-a-move-a-copy-and-a-rename-row-192), [Section 13](../File%20Browser.md#13-unresolved) item 13.

**Owner one — the caller's own end.** `Server protocol and transfers` [S16](../Server%20protocol%20and%20transfers.md#75-publication-is-the-last-step-and-it-is-one-step), observed on the Creator's Mac, macOS 26.5, `OpenSSH_10.2p1`, **2026-08-17**, driving OpenSSH's own server subsystem directly over a pipe: *"The base `SSH_FXP_RENAME` over an existing target returned status **4, `Failure`**"*, while `posix-rename@openssh.com` onto an existing final file returned *"status **0, `Success`**"* with the published file's inode changed and the staging name gone.

**Owner two — three rentals across three sellers.** [The multi-provider batch](../The%20multi-provider%20batch%20%E2%80%94%20the%20surface%20that%20did%20not%20vary,%20the%20Debian%20route%20that%20exists,%20and%20the%20cards%20a%20slice%20cannot%20see.md), **2026-08-22**, surface table: *"Exclusive create over an existing path | **refused** (`Failure`)"*; *"Plain create over the same path — the control | **succeeded**"* — *"proves the refusal is exclusivity, not permission"*; *"Base rename over an existing name | **refused** (`Failure`)"*; *"`posix-rename@openssh.com` over an existing name | **succeeded**, target held the source bytes"*.

**Repaired in the document on 2026-08-23.** [Section 13](../File%20Browser.md#13-unresolved) item 13 read *"observed on one server — the caller's own"*, which is evidence four days older than the document's last dated correction: the multi-provider batch observed the same refusal on three rentals across three sellers on 2026-08-22, **with the exclusive-create control beside it in the same table** — exclusive create over an existing path refused, plain create over the same path succeeded, which is what proves the refusal is exclusivity rather than permission. Item 13 now names all four servers and both readings. The item's **breaks** clause survives and narrows: all four are OpenSSH servers at Linux rented ends, so the case left open is a **non-OpenSSH** file-transfer server, which is `Server protocol and transfers` §12 item 3's own configuration.

**Not covered:** four OpenSSH servers across two operating-system families, all Linux at the rented end; nothing about a non-OpenSSH file-transfer server, which `Server protocol and transfers` §12 item 3 keeps open.

### D3. `posix-rename@openssh.com` — what it performs, and that it is announced (documented and observed)

**Clauses:** [X7](../File%20Browser.md#42-the-rule-collisions-are-judged-under-row-192)'s replace-intended branch and its *rare case rather than the ordinary one* correction.

**Documented.** OpenSSH's own `PROTOCOL` file — the file that opens *"This documents OpenSSH's deviations and extensions to the published SSH protocol."* — Section 4.3, address `https://raw.githubusercontent.com/openssh/openssh-portable/master/PROTOCOL`, read **2026-08-20**: *"On receiving this request the server will perform the POSIX operation rename(oldpath, newpath) and will respond with a SSH_FXP_STATUS message. This extension is advertised in the SSH_FXP_VERSION hello with version \"1\"."*

**Observed.** [One box with Blender on it](../One%20box%20with%20Blender%20on%20it%20—%20the%20reading,%20the%20listing,%20and%20the%20names%20that%20do%20not%20come%20back.md) §3.1, **2026-08-17** (register Fact 93): the three extensions the protocol document rests on — POSIX-rename, fsync and statvfs — *"are all announced"*, in a set of eight, read raw off the wire and byte-identical under the non-root login. [The multi-provider batch](../The%20multi-provider%20batch%20%E2%80%94%20the%20surface%20that%20did%20not%20vary,%20the%20Debian%20route%20that%20exists,%20and%20the%20cards%20a%20slice%20cannot%20see.md), **2026-08-22**: announced on all three, and *"the announced extension set is 11 on the OpenSSH 9.x boxes and 8 on the 8.9p1 box"*, the shortfall being `copy-data`, `home-directory` and `users-groups-by-id@openssh.com`.

**Not covered:** four boxes, three sellers, Linux only. `Server protocol and transfers` §12 item 3 records the reachable configuration in which a provider image bans a request or serves a non-OpenSSH server — documented from `sftp-server(8)`'s `-P`, `-p` and `-R` flags, cited in that document's own companion.

### D4. When `posix-rename@openssh.com` entered OpenSSH — read at the publications, and one attribution refuted (documented, read 2026-08-23)

**Clauses:** [X7](../File%20Browser.md#42-the-rule-collisions-are-judged-under-row-192)'s age sentence, and [Section 13](../File%20Browser.md#13-unresolved) item 12's preserved *"OpenSSH has announced it since 4.9"*.

**What the document carried, and it had no address anywhere.** [X7](../File%20Browser.md#42-the-rule-collisions-are-judged-under-row-192) read *"as OpenSSH's own `PROTOCOL` file says it has been in every release since 4.9 (2008)"*. `Server protocol and transfers` [S17](../Server%20protocol%20and%20transfers.md#75-publication-is-the-last-step-and-it-is-one-step) and its §12 item 3 carried the same claim attributed to a different publication — *"by that project's own release announcement of 2008-03-31"* — also with no address and no date read. Looked for in: [Facts.md](../Facts.md) Fact 93 in full, [Server protocol and transfers' companion](Server%20protocol%20and%20transfers.sources.md), [Dependency Upload & Remap's companion](Dependency%20Upload%20&%20Remap.sources.md), every other `Sources/` file in this folder, the Specs sources, and every observation document named above. **Both publications were read on 2026-08-23 and they settle it. It is one claim with two attributions, not two claims — and one of the two attributions is refuted.**

- **`PROTOCOL` does not say it, and carries no release history at all.** OpenSSH's own `PROTOCOL` file, `https://raw.githubusercontent.com/openssh/openssh-portable/master/PROTOCOL`, read **2026-08-23**, section **4.3**: the whole of what it states about the extension's presence is *"This extension is advertised in the SSH_FXP_VERSION hello with version "1"."* — a wire-format version, not a release. **The file names no OpenSSH release anywhere**, and the string `4.9` occurs in it exactly once, as the section number of a different extension: *"4.9. sftp: Extension request "expand-path@openssh.com""*. **The publication is right and the document was wrong**, and [X7](../File%20Browser.md#42-the-rule-collisions-are-judged-under-row-192) is corrected to say so.
- **The release announcement does say it, and it is the right publication.** OpenSSH's own release notes for 4.9, `https://www.openssh.com/txt/release-4.9`, read **2026-08-23**: *"OpenSSH 4.9 was released on 2008-03-31."*, and under *Changes since OpenSSH 4.7* → *New features*: *"Added a protocol extension method "posix-rename@openssh.com" for sftp-server(8) to perform POSIX atomic rename() operations."*, followed on the page by the bug reference *"(bz#1400)"*. **So `Server protocol and transfers`' attribution is the correct one and its date is exact.**
- **But the claim as both documents worded it is still wider than either publication.** The announcement says the extension was **added** in 4.9. **No OpenSSH publication says it has been in *every release since*** — that sentence is an extrapolation neither page carries. What the two pages together do support is: added in 4.9 on 2008-03-31, and still documented in the current `PROTOCOL` file.

Label: **documented**, for *added in OpenSSH 4.9, released 2008-03-31, and still documented today*. **What breaks:** nothing built — the age sentence feeds only the *how likely is this branch* estimate. [X7](../File%20Browser.md#42-the-rule-collisions-are-judged-under-row-192)'s behaviour on an unannounced extension is written and works — create-intended publication is the base rename alone — and the observed ground (entry **D3**) establishes the extension present on four rentals across three sellers, including the oldest server on record at 8.9p1.

**Routed, not edited.** Read on 2026-08-23, `Server protocol and transfers`' [S17](../Server%20protocol%20and%20transfers.md#75-publication-is-the-last-step-and-it-is-one-step) follow-on paragraph already carries the same address, the same quoted words and the same finding — *"The release announcement is what carries the version history and OpenSSH's `PROTOCOL` file does not"* — reached independently on that document's own side, which is a second reading agreeing with this one. **Its [§12](../Server%20protocol%20and%20transfers.md#12-unresolved) items 3 and 7 still carry the claim with no address, and its companion still files it unresolved.** Those are that document's clauses and its own repair; nothing is edited there from here.

**Not covered:** neither page says anything about a rental, about a provider image that bans a request, or about a non-OpenSSH server — `Server protocol and transfers` §12 item 3's ground. `PROTOCOL` was read at the project's `master` branch, so it describes OpenSSH as it now stands and not any intervening release.

### D5. No folding rule is documented, on any filesystem the user's own computer may carry

**Clauses:** [X8](../File%20Browser.md#42-the-rule-collisions-are-judged-under-row-192), and [W4](../File%20Browser.md#82-the-comparison-behind-the-warnings-rows-193-and-192)'s *modelling the machine's folding rule* comparison.

**Owner:** [Facts.md](../Facts.md) **Fact 16**, over [The user's own computer](../The%20user's%20own%20computer%20—%20where%20a%20download%20lands,%20what%20it%20will%20take,%20what%20it%20will%20show,%20and%20how%20the%20app%20replaces%20itself.md) §3, **2026-08-18**, and **Fact 121**. Its words on what remains owed: *"**NTFS's folding rule is undocumented** on every page read, where exFAT's is fully specified."* On macOS: *"**APFS stores names byte for byte** and normalises only its lookup hash"*, and *"**Folding is Unicode *full* folding**"*, tested against the folding table.

**Not covered:** which Unicode version macOS folds by; a case-sensitive APFS volume, HFS+, and network mounts, all of which need a machine that may be changed; and Windows has no computer on this project. `Desktop app and delivery` C18 owns the limit [X8](../File%20Browser.md#42-the-rule-collisions-are-judged-under-row-192) names; nothing is decided here.

### D6. One tool aborted an entire transfer on a non-UTF-8 name (observed)

**Clauses:** [X9](../File%20Browser.md#43-keep-both-rows-192-and-186)'s trim-in-bytes reasoning, which quotes `Server protocol and transfers` §12 item 12 for it.

**Owner:** [Six machines](../Six%20machines%20—%20the%20container,%20the%20webhook%20and%20the%20ready%20action.md), Fact 29, **2026-08-15/16**: the machine *"accepts non-UTF-8 sequences the Mac refuses"*, and *"one tool **aborted the entire transfer** on the non-UTF-8 name where the other skipped it. The Mac's own archive tool broke it differently again: 32 sidecar files and a "File name too long" failure."*

**Not covered:** two transfer tools and one archive tool on one Mac; nothing about what Smoozy's own byte loop does, which is `Server protocol and transfers`' and is not a tool.

---

## E. Section 6 — the pool and the order

### E1. What the link carries, where the concurrency knee is, and what a second machine buys (observed)

**Clauses:** [O1](../File%20Browser.md#61-the-pool-row-188)–[O4](../File%20Browser.md#62-the-order-and-the-shares-row-189), [M3](../File%20Browser.md#71-delete-rows-194-and-181)'s hours-of-uplink figure, [K9](../File%20Browser.md#93-the-unchanged-file-row-201-and-fact-30)'s seconds-against-minutes comparison.

**Owner:** [Six machines](../Six%20machines%20—%20the%20container,%20the%20webhook%20and%20the%20ready%20action.md), *What the connection carries*, Fact 25, **2026-08-15/16**; register entry Fact 25. Its words: against the same home line, *"**12–15 MB/s down and 1.2–1.3 up**. **Down is 11–12 times up, and that asymmetry is the dominant fact about this connection.**"* On concurrency: *"the knee is at **two streams** for download — one to two buys 53%, two to four buys 3%, four to eight buys 11% — and around **four** for upload, where the uplink saturates near 1.85–1.9 MB/s. **Two to four is the entire useful range**, and most of the gain is hiding a flat 1.0–1.4 s connection setup rather than adding bandwidth."* On sharing: one machine 13.9–15.2 MB/s, a second alone 14.7–15.0, *"**Both at once gave 16.5–18.3.** A whole second rented machine bought about **20%, not 100%**"*. And on the upload direction, *"uncoordinated fan-out was **actively worse** than one machine at a time: 1.54 MB/s to one box, 1.24 across two, 0.99 across three."* One provider *"could not fill its share of any budget"* at 4.24 down and 0.35 up, and with three machines *"**the slowest set the finish time**"* — 112 seconds against a fast half done in 28.

**Not covered, in the source's own words:** *"one home line on one evening over Wi-Fi. Two of the three rentals were the same provider."* [Section 13](../File%20Browser.md#13-unresolved) item 11 carries the cost figures' one-box limit. The earlier record — 1.3 MB/s down and 0.48 up in [The rented machine](../The%20rented%20machine.md) — is superseded by this measurement and is recorded there as *"a floor, not a measurement of the link at its best"*.

### E2. The session cap is the server's, and it was 10 on every rental posed (documented and observed)

**Clauses:** [O1](../File%20Browser.md#61-the-pool-row-188)'s *T2 makes the session cap the server's* and its *at a cap of one* branch; [L7](../File%20Browser.md#13-the-blend-version-row-182)'s reason for riding the reading channel.

**Documented.** `sshd_config(5)`, `MaxSessions`, read on the shipped page **2026-08-23** and cited in full at [Server protocol and transfers' companion](Server%20protocol%20and%20transfers.sources.md): *"Specifies the maximum number of open shell, login or subsystem (e.g. sftp) sessions permitted per network connection.  Multiple sessions may be established by clients that support connection multiplexing.  Setting MaxSessions to 1 will effectively disable session multiplexing, whereas setting it to 0 will prevent all shell, login and subsystem sessions while still permitting forwarding.  The default is 10."*

**Observed.** [One box with Blender on it](../One%20box%20with%20Blender%20on%20it%20—%20the%20reading,%20the%20listing,%20and%20the%20names%20that%20do%20not%20come%20back.md) §3.3, 2026-08-17: the cap is 10 and per connection, *"not set in configuration, present only as a commented default, and then posed rather than trusted: ten of fourteen concurrent sessions on one connection succeeded, with the first ten as the control, and a second connection gets its own ten."* And *"**The refusal never says *too many sessions***"* — it appends `Host key verification failed.` [The multi-provider batch](../The%20multi-provider%20batch%20%E2%80%94%20the%20surface%20that%20did%20not%20vary,%20the%20Debian%20route%20that%20exists,%20and%20the%20cards%20a%20slice%20cannot%20see.md), 2026-08-22: *"Concurrent sessions on one connection | **capped at 10**"* and *"Parallel connections | **6 opened, no refusal**"* on all three.

**Not covered:** what any given rental's configuration sets, which is not readable before it is hit; the cap of 1 and of 0 are documented configurations and were observed nowhere.

---

## F. Section 7 — managing files

### F1. A file deleted on a rented machine is not recoverable on it (observed)

**Clauses:** [M1](../File%20Browser.md#71-delete-rows-194-and-181), [Section 13](../File%20Browser.md#13-unresolved) item 4.

**Owner:** [Batch A — the shape throwaway](../Batch%20A%20—%20the%20shape%20throwaway.md), *The riders*, **2026-08-13**; register entry Fact 31. Its words: *"**Fact 31 — a deleted file is not recoverable.** ext4, no `trash-cli`, no trash folder for root, no snapshotting filesystem. What is deleted is gone."* The machine: a Vast.ai rental, one RTX 3080, *"126 GB of disk with 119 GB free, ext4. Ubuntu on `systemd`"*.

**Not covered, and the document says so:** one machine, one filesystem. That document's own framing binds this: *"**This is a frozen snapshot of one machine on one day** … Read it as *this is what that box did*, never as *this is what a rented machine does*."* A rental with recovery tooling would make the warning's *is gone* overstated there — which is the spec's to soften, not this document's ([Section 13](../File%20Browser.md#13-unresolved) item 4, and [the spec's own sources file](../../Specs/Sources/File%20Browser.sources.md), which had this labelled an assumption before the fact was answered).

### F2. A rental can carry more than one filesystem (observed)

**Clauses:** [M6](../File%20Browser.md#73-move-row-192-and-the-partial-action)'s *the case is reachable on real rentals rather than hypothetical*.

**Owner:** [Two more boxes](../Two%20more%20boxes%20—%20the%20volume,%20the%20full%20disk,%20and%20a%20day%20left%20running.md), **2026-08-16**; register entry Fact 28's more-than-one-disk clause — a rental with a volume attached, the volume presenting at its own mount, free space reading true on it by root and by the non-root login, before and after the disk filled.

**Not covered:** one rental, one attached volume, one provider. Nothing establishes that a cross-filesystem move is common, only that the case exists; [M6](../File%20Browser.md#73-move-row-192-and-the-partial-action) declares the pair *"for whenever it is met"*.

### F3. Server-side copy is announced on some rentals and not on others (observed)

**Clauses:** [M7](../File%20Browser.md#74-copy-rows-192-and-197s-shape-machine-locally)'s *by the subsystem's copy-data extension where the connection announced it, and otherwise by the machine's own copy through the command channel*.

**Owner:** [One box with Blender on it](../One%20box%20with%20Blender%20on%20it%20—%20the%20reading,%20the%20listing,%20and%20the%20names%20that%20do%20not%20come%20back.md) §3.1, 2026-08-17 (register Fact 93): the rental's set of eight *"is smaller than the caller's eleven, and the shortfall matters: the rental lacks **server-side copy**, which the Mac announces."* [The multi-provider batch](../The%20multi-provider%20batch%20%E2%80%94%20the%20surface%20that%20did%20not%20vary,%20the%20Debian%20route%20that%20exists,%20and%20the%20cards%20a%20slice%20cannot%20see.md), 2026-08-22: eleven on the two OpenSSH 9.x boxes including `copy-data`, eight on the 8.9p1 box without it — *"**This is the first observation that the set varies at all on a rental**, and it is what makes capability negotiation load-bearing rather than a courtesy."*

**Not covered:** four boxes, three sellers. What the machine's own copy program is on an arbitrary rental is [Section 13](../File%20Browser.md#13-unresolved) item 9's ground and [M7](../File%20Browser.md#74-copy-rows-192-and-197s-shape-machine-locally) names none.

---

## G. Section 8 — the warnings

### G1. An alias makes one recorded location into two, in Blender's own reckoning

**Clauses:** [W4](../File%20Browser.md#82-the-comparison-behind-the-warnings-rows-193-and-192), [W1b](../File%20Browser.md#81-the-three-warning-readings-row-193)'s first limit, [Section 13](../File%20Browser.md#13-unresolved) item 5.

The document names the comparison question itself: ***Dependencies 19***, reached through [What has to travel with a file](../../Blender%20Research/Across%20topics/What%20has%20to%20travel%20with%20a%20file.md). That is the whole citation. **No Blender answer is assembled here**, and no finding document is read across.

**Not covered, as the comparison itself records:** the fifteen tests ran on the Creator's Mac and on the third rented Linux machine only; the second-mount route is a Linux result alone; everything about Windows is untested; and how two recorded library locations are compared is stated in no version's documentation, so the whole answer is by test.

### G2. What the live-job warning's evidence reaches — named nowhere in this document

**Clauses:** [W7](../File%20Browser.md#83-the-live-set-and-the-ownership-rule-row-195) — *"the evidence reaches a scene's material picture and a linked `.blend` and nothing else"*.

**The document names no comparison question and no joined answer for this sentence.** The citation behind it is owned by [the spec's own sources file](../../Specs/Sources/File%20Browser.sources.md), which carries it under two comparisons — ***Reading a file 11*** (what replacing a file a running render depends on does to that render) and ***Dependencies 14*** (rendering one scene, and a `.blend` linked as a library) — and records the not-covered list in full there, including *"what happens when the replacement differs in size, format or colour depth from the file it replaces"* and Windows. **Pointed at, not copied.**

The **decision** the sentence carries — that the warning stays wide, and that it is not dismissible — is Creator-stated (2026-08-12 and the spec's own) and earns no entry.

---

## H. Section 9 — continuation, and the unchanged file

### H1. Fact 30

Entry **A11** above, unchanged. [K6](../File%20Browser.md#93-the-unchanged-file-row-201-and-fact-30) consumes it whole and cites this file for the evidence, the controls and the negative.

### H2. What a read-through of a large file costs at each end (observed)

**Clauses:** [K9](../File%20Browser.md#93-the-unchanged-file-row-201-and-fact-30)'s *"about 1.3 to 2.2 seconds for a 670,000,000-byte file on the observed Mac"*, and [Section 14](../File%20Browser.md#14-moments-that-reached-the-creator)'s Row 201 costing.

**Owner:** [Six machines](../Six%20machines%20—%20the%20container,%20the%20webhook%20and%20the%20ready%20action.md), Fact 15, **2026-08-15/16**; register entry Fact 15. Its words, on a real 669,966,622-byte `.blend`: *"Mac: md5 1.26 s, sha256 2.22 s. **Every digest identical at both ends.**"* And the finding it calls the one worth keeping: *"`openssl dgst -sha256` took **0.46 s** against `sha256sum`'s **2.24 s** — same file, same box. **The cost is processor, not disk**."*

**Not covered, in the source's own words:** *"on processors with the instructions that make it fast, which both ends here had and a rental may not."* One Mac, one file, one sitting. The document's *"1.3 to 2.2 seconds"* is that Mac's md5 and sha256 figures read as a range; nothing establishes which digest a build uses.

### H3. Damage at rest is invisible, and a restored modification time defeats a stat (observed)

**Clauses:** [K9](../File%20Browser.md#93-the-unchanged-file-row-201-and-fact-30)'s *"a writer that restores modification times (Fact 72's observed shape)"*, and [K13](../File%20Browser.md#93-the-unchanged-file-row-201-and-fact-30)'s silent-carry-on shape.

**Owner:** [Six machines](../Six%20machines%20—%20the%20container,%20the%20webhook%20and%20the%20ready%20action.md), Fact 72, **2026-08-15/16**; register entry Fact 72. Its words: *"**Damage at rest is invisible.** A file with four bytes overwritten in place **and its modification time put back** — so size and time both match a whole arrival — travelled through `scp` perfectly at exit 0. And a default `rsync` **called it done**: 48 bytes sent, exit 0, file still corrupted."*

**And the fourth value the file-transfer surface cannot carry.** [Six machines](../Six%20machines%20—%20the%20container,%20the%20webhook%20and%20the%20ready%20action.md), Fact 21: *"a hand-restored modification time still moves the change time, which cannot be set back."* [K7](../File%20Browser.md#93-the-unchanged-file-row-201-and-fact-30)'s path identity on the **user's own computer** is size, modification time and file identifier and does not include the change time; nothing in this document claims it does.

**Not covered, in the source's own words:** *"compression was never on, only the default cipher suite was exercised, and corruption caused by the **disk** was not posed. Nothing was posed against Windows."*

### H4. The corrupt-file warning [K9](../File%20Browser.md#93-the-unchanged-file-row-201-and-fact-30) and Section 14 lean on is OpenSSH's own (documented)

**Clauses:** [Section 14](../File%20Browser.md#14-moments-that-reached-the-creator)'s Row 201 entry — *"the corrupt file OpenSSH itself warns about"*.

`sftp(1)`, the manual page shipped on the Creator's Mac, `/usr/share/man/man1/sftp.1`, page dated **December 6, 2024**, footer `macOS 26.6`, read **2026-08-23** — cited in full at [Server protocol and transfers' companion](Server%20protocol%20and%20transfers.sources.md). `get -a`: *"If the -a flag is specified, then attempt to resume partial transfers of existing files.  Note that resumption assumes that any partial copy of the local file matches the remote copy.  If the remote file contents differ from the partial local copy then the resultant file is likely to be corrupt."* `put -a` carries the mirror — *"Note that resumption assumes that any partial copy of the remote file matches the local copy."*

**Read again on the shipped page 2026-08-23** — `OpenSSH_10.3p1, LibreSSL 3.3.6` on macOS 26.6.2, page dated December 6, 2024, footer `macOS 26.6` — and the `put -a` wording is exact in full: *"If the -a flag is specified, then attempt to resume partial transfers of existing files.  Note that resumption assumes that any partial copy of the remote file matches the local copy.  If the local file contents differ from the remote local copy then the resultant file is likely to be corrupt."* The same page's command-line `-a` carries the short form: *"Attempt to continue interrupted transfers rather than overwriting existing partial or complete copies of files.  If the partial contents differ from those being transferred, then the resultant file is likely to be corrupt."*

**Repaired in the document on 2026-08-23.** [Section 14](../File%20Browser.md#14-moments-that-reached-the-creator)'s Row 201 entry read *"the alternative — trusting any filesystem's stat semantics — is the corrupt file OpenSSH itself warns about"*, and **that widened the page beyond what it says**: the warning is about a **resume that assumes a partial copy matches the other end**, and the page says nothing about what any filesystem's stat reports. The entry now names the assumption the page names and quotes `put -a`. **The upload direction is the one quoted**, because this document's transfers go up. **The conclusion did not move** — [K9](../File%20Browser.md#93-the-unchanged-file-row-201-and-fact-30)'s gate is kept for the same reason it always was.

**Not covered:** the page is about the standard client's own resume, which Smoozy does not run; it is cited for the failure it names, not for a mechanism.

### H5. K11 — the three directory-change instruments, named and not adopted (documented, read 2026-08-23)

K11 names these only to say that a standard instrument for *has anything landed in this folder* exists and is documented on all three systems, and that none of them asks the caller for a filename or a naming convention. **None is adopted and none was run.**

- **macOS.** Apple, *File System Events* (Core Services API collection), `https://developer.apple.com/documentation/coreservices/file_system_events`, read 2026-08-23 at the page's own documentation data — its abstract is *"Get notifications when the contents of a directory hierarchy change."*, exact as K11 quotes it. The guide that page links states the same at length: *"The file system events API provides a way for your application to ask for notification when the contents of a directory hierarchy are modified."* (`https://developer.apple.com/library/archive/documentation/Darwin/Conceptual/FSEvents_ProgGuide/Introduction/Introduction.html`).
- **Linux.** `inotify(7)`, read at the Linux man-pages project's own source, `https://git.kernel.org/pub/scm/docs/man-pages/man-pages.git/plain/man/man7/inotify.7`, 2026-08-23: *"When a directory is monitored, inotify will return events for the directory itself, and for files inside the directory."*, exact. The three events K11 names are on the same page — `IN_CREATE`: *"File/directory created in watched directory"*; `IN_MOVED_TO`: *"Generated for the directory containing the new filename when a file is renamed."*; `IN_CLOSE_WRITE`: *"File opened for writing was closed."*
- **Windows.** Microsoft, *ReadDirectoryChangesW*, `https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-readdirectorychangesw`, read 2026-08-23. `FILE_NOTIFY_CHANGE_FILE_NAME`: *"Any file name change in the watched directory or subtree causes a change notification wait operation to return. Changes include renaming, creating, or deleting a file."* — K11's fragment exact. K11's caution is the page's own Remarks: *"If the buffer overflows, ReadDirectoryChangesW will still return true, but the entire contents of the buffer are discarded and the lpBytesReturned parameter will be zero, which indicates that your buffer was too small to hold all of the changes that occurred."*, with *"In this case, you should compute the changes by enumerating the directory or subtree."*

**Not covered:** latency, behaviour under load, and anything at all about what a Blender save looks like at the path, which is Fact 30's and not these pages'. Each page establishes that the instrument exists and what it notifies on, and no more.

---

## I. Section 10 — the picker

### I1. Row 307 — whether a system file picker can take files and folders together (documented, read 2026-08-20)

- **Windows — no.** Documented — Microsoft, *FILEOPENDIALOGOPTIONS enumeration*, `https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/ne-shobjidl_core-_fileopendialogoptions`, read 2026-08-20: `FOS_PICKFOLDERS` — *"Present an Open dialog that offers a choice of folders rather than files."* The folder choice is a mode of the one dialog, offered **rather than** files; no option on the page offers both at once. **Not covered:** any newer dialog surface Microsoft documents elsewhere, and anything observed on a running Windows, which this project cannot pose.
- **Linux, the portal route — no.** Documented — freedesktop, *org.freedesktop.portal.FileChooser*, `https://flatpak.github.io/xdg-desktop-portal/docs/doc-org.freedesktop.portal.FileChooser.html`, read 2026-08-20: `OpenFile` — *"Asks to open one or more files"* — takes a `directory` option — *"Whether to select for folders instead of files. Default is to select files."* **Instead of**, a two-way switch. **Not covered:** toolkit dialogs outside the portal, and any desktop's own non-portal chooser.
- **macOS — two independent switches, and the combination is now observed.** Documented — Apple, *NSOpenPanel.canChooseFiles*, `https://developer.apple.com/documentation/appkit/nsopenpanel/canchoosefiles`, read 2026-08-20: *"A Boolean that indicates whether the user can choose files in the panel."* And *NSOpenPanel.canChooseDirectories*, `https://developer.apple.com/documentation/appkit/nsopenpanel/canchoosedirectories`, read 2026-08-20: *"A Boolean that indicates whether the user can choose directories in the panel."* Two properties of one panel, each documented, neither page naming the other or an exclusion. **That both may be on at once was an inference from the two pages until 2026-09-04, when the check was run — see I1a below** ([Section 13](../File%20Browser.md#13-unresolved) item 8, closed).

### I1a. Row 307, macOS — the combination put up and watched (observed in a controlled test, run 2026-09-04)

**Clause:** [H3](../File%20Browser.md#10-the-picker-row-307)'s *yes*, and with it [Section 13](../File%20Browser.md#13-unresolved) item 8's closure. **Label: observed in a controlled test** — not documented, because neither Apple page states the combination and the reading route was already taken and is above.

**What was run, so it can be run again.** On the Creator's own Mac, **macOS 26.6.2, build 25G83**, on **2026-09-04**, at a Build sitting. A small program was compiled outside the project and deleted afterwards; it made a scratch folder holding one ordinary file and one ordinary folder, raised an `NSOpenPanel` over that folder **three times** — file switch on and folder switch off, then off and on, then **both on** — with multiple selection allowed, read the two switches back off each live on-screen panel, recorded which items the panel asked its own delegate about, and **cancelled each panel itself**. Nothing was opened, nothing of the Creator's was touched, and nothing was left behind.

**What was seen, in the three runs.**

- **File switch only:** the panel reports the two switches as set, and asks about the ordinary file and the folder inside — **not** about the folder being browsed.
- **Folder switch only:** the switches as set, and it asks about the folder inside and the folder being browsed — **not** about the ordinary file.
- **Both on:** the panel keeps **both** switches on — neither is reset and nothing refuses — and it asks about **all three**: the ordinary file, the folder inside, and the folder being browsed. **The union of the two single-switch runs, from one panel.**

**Not covered, and it matters.** **No selection was made and Open was not pressed**, so this establishes that one panel accepts the combination and offers both kinds — it is not a measurement of what a mixed selection hands back. What the panel asks its delegate about is the panel's own behaviour and is not the same as a row being enabled on screen; the three runs are read against each other rather than singly, which is what makes the differential the evidence. **One machine, one macOS version**, so nothing here is a claim about macOS generally or about any earlier or later release. **Windows and the Linux portal route are untouched by it** — both are answered *no* above, from their own publications.

### I2. H3a — the drop facility on the **withdrawn** shell (documented, read 2026-08-21; superseded 2026-09-04 by I2a)

**Superseded, 2026-09-04, and kept rather than deleted.** The shell was settled as **Tauri v2** on 2026-09-02 (`Desktop app and delivery` H4 and H5b), so everything below is a true reading of a project this application does not use. **No clause of this document rests on it any more**; [H3a](../File%20Browser.md#10-the-picker-row-307) rests on **I2a** below, and [H4](../File%20Browser.md#10-the-picker-row-307)'s *dragging onto the list … takes any mix everywhere* was struck rather than re-sourced, because the chosen shell's reference does not carry the folder half ([Section 13](../File%20Browser.md#13-unresolved) item 19). It is kept because it is the citation behind `Facts.md` Fact 120, which is not withdrawn.

**Clauses, as they stood:** [H3a](../File%20Browser.md#10-the-picker-row-307)'s five bullets, and [H4](../File%20Browser.md#10-the-picker-row-307)'s *dragging onto the list … takes any mix everywhere*.

**Owner:** [Facts.md](../Facts.md) **Fact 120**, answered **2026-08-21** by reading the shell's own documentation, no rental and no machine. The addresses are the register's; `wails.io` returns 403 to a fetch, so the site's own documentation sources were read.

- **The capability, one option, off by default.** `https://raw.githubusercontent.com/wailsapp/wails/master/website/docs/reference/options.mdx`, read 2026-08-21: *"EnableFileDrop enables wails' drag and drop functionality that returns the dropped in files' absolute paths."* Default `false`. *"The event returns the coordinates of the drop and a file path slice."*
- **Folders are named by the project itself.** `https://raw.githubusercontent.com/wailsapp/wails/master/website/docs/reference/runtime/draganddrop.mdx`, read 2026-08-21: *"This part of the runtime handles dragging and dropping files and or folders in to the window."* And the handler's shape: *"Calls the callback function with the coordinates inside the window where the drag was released and a slice of absolute file paths."*
- **What arrives is paths and nothing else** — no sizes, no counts, no walk; the register states it outright, which is [H3a](../File%20Browser.md#10-the-picker-row-307)'s second bullet.
- **The drop target.** `CSSDropProperty` (default `--wails-drop-target`) and `CSSDropValue` (default `drop`) mark an element as a valid target, and the runtime adds `wails-drop-target-active` while the pointer is over it.
- **The separate switch.** `DisableWebViewDrop`, *"to prevent accidental opening of dragged in files in the webview"*.

**Not covered, in the register's own words:** *"The pages state no per-platform difference and this project has observed none"*, on any of the three; Wails' own version notice applies (`Desktop app and delivery` H10), and nothing here rests on a facility announced for the next major version. `Desktop app and delivery` [Section 22](../Desktop%20app%20and%20delivery.md#22-unresolved) carries both. **All three quotes match the document character for character.**

### I2a. H3a — the drop facility on the **chosen** shell, Tauri v2 (documented, read 2026-09-04)

**Clauses:** [H3a](../File%20Browser.md#10-the-picker-row-307)'s bullets. **Not** H4's *any mix everywhere*, which these pages do not support and which was struck.

**Owner:** no fact number. The reading was taken at a **Build sitting on 2026-09-04**, when a builder standing on H3a found it citing the withdrawn shell, and it is recorded in `Audits/Build/File Browser/Decisions record.md` and quoted at `App/File Browser/src/picker.rs`. No rental and no machine.

- **The event and its payload.** Tauri v2's JavaScript API reference, `WebviewWindow.onDragDropEvent`, `https://v2.tauri.app/reference/javascript/api/namespacewebviewwindow/`, read 2026-09-04: *"Listen to a file drop event. The listener is triggered when the user hovers the selected files on the webview, drops the files or cancels the operation."* The payload the page's own example reads is `paths`. **So *what arrives is paths and nothing else* holds on this shell too** — no sizes, no counts, no walk.
- **The Windows alternative, which is the one per-platform statement the reading carries.** `WebviewBuilder::disable_drag_drop_handler`, `https://docs.rs/tauri/2.11.5/tauri/webview/struct.WebviewBuilder.html`, read 2026-09-04: *"Disables the drag and drop handler. This is required to use HTML5 drag and drop APIs on the frontend on Windows."* So the shell's own handler and the page's own drag and drop are **alternatives rather than companions** on Windows, and a build that wants the paths keeps the handler enabled.

**Not covered, and it is the substantive gap:** **whether a folder may be dropped at all.** The page speaks of files throughout and states nothing about folders in either direction — unstated, not refused — so the withdrawn shell's *"files and or folders"* has no counterpart here and was not carried across. [Section 13](../File%20Browser.md#13-unresolved) item 19 holds what that breaks. **Also not covered:** how a drop target is marked, which the withdrawn shell published and this reading does not reach; any per-platform difference beyond the Windows statement above; and anything observed on a running machine, on any of the three. **Both quotes match the document character for character.**

---

## J. Section 13 — the unresolved entries whose ground is a publication

### J1. Section 13 item 3 — the name and path bounds are Linux's, not universal (documented, read 2026-08-23)

- **The standard makes them per-implementation and per-pathname.** POSIX.1-2024, `<limits.h>`, *Pathname Variable Values*: *"The values in the following list may be constants within an implementation or may vary from one pathname to another."* The minimum an implementation must accept is `{_POSIX_NAME_MAX}` **14** and `{_POSIX_PATH_MAX}` **256**, from the same page's *Minimum Values*. Address: `https://pubs.opengroup.org/onlinepubs/9799919799/basedefs/limits.h.html`, read 2026-08-23. Exact as item 3 carries it.
- **The two systems this project runs on disagree, and both figures are read from headers rather than from a manual.** **macOS:** `sys/syslimits.h` in the installed SDK, `/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/syslimits.h` (SDK 26.5 on macOS 26.6.2), read 2026-08-23 — `#define NAME_MAX 255 /* max bytes in a file name */` and `#define PATH_MAX 1024 /* max bytes in pathname */`. **Linux:** the kernel's own `include/uapi/linux/limits.h`, `https://git.kernel.org/pub/scm/linux/kernel/git/torvalds/linux.git/plain/include/uapi/linux/limits.h`, read 2026-08-23 — `#define NAME_MAX 255 /* # chars in a file name */` and `#define PATH_MAX 4096 /* # chars in a path name including nul */`, so 4095 usable, which is the figure N4, X9 and item 3 carry.

**One sentence the macOS header carries that the document does not, recorded rather than used:** beside `NAME_MAX` it notes *"Many filesystems (including HFS & APFS) may support names longer than `NAME_MAX` bytes."* Nothing here rests on it — N4's bound is enforced against the machine's limits, not the Mac's — and it is written down so a later reader does not read 255 as a bound on the source side.

**Not covered:** any actual rental's limits, which is exactly why item 3 carries these as bounds rather than as a law of rentals.

### J2. Section 13 item 6 — Windows' file times and identifiers, and Linux's ext4 granularity (documented, read 2026-08-23)

**Windows times.** Microsoft, *File Times*, `https://learn.microsoft.com/en-us/windows/win32/sysinfo/file-times`, read 2026-08-23:

- *"A *file time* is a 64-bit value that represents the number of 100-nanosecond intervals that have elapsed since 12:00 A.M. January 1, 1601 Coordinated Universal Time (UTC)."*
- *"Time stamps are updated at various times and for various reasons. The only guarantee about a file time stamp is that the file time is correctly reflected when the handle that makes the change is closed."*
- *"When writing to a file, the last write time is not fully updated until all handles that are used for writing are closed."*

All three exact as item 6 and K13 carry them. **The page's own shape is the basis for item 6's *the narrower repair does not work*:** it offers `GetFileTime` against a handle and `FindFirstFile`/`FindNextFile` against a directory as two ways of reading the same value, and draws no distinction between them anywhere.

**Windows identifiers.** Microsoft, *BY_HANDLE_FILE_INFORMATION*, `https://learn.microsoft.com/en-us/windows/win32/api/fileapi/ns-fileapi-by_handle_file_information`, read 2026-08-23: *"The identifier (low and high parts) and the volume serial number uniquely identify a file on a single computer."*; *"File IDs are not guaranteed to be unique over time, because file systems are free to reuse them."*; and on a replace — *"You can replace one file with another file without changing the file ID by using the ReplaceFile function. However, the file ID of the replacement file, not the replaced file, is retained as the file ID of the resulting file."* The 128-bit form and the named call are on the same page: *"The ReFS file system, introduced with Windows Server 2012, includes 128-bit file identifiers. To retrieve the 128-bit file identifier use the GetFileInformationByHandleEx function with **FileIdInfo** to retrieve the FILE_ID_INFO structure. The 64-bit identifier in this structure is not guaranteed to be unique on ReFS."* And *FILE_ID_INFO*, `https://learn.microsoft.com/en-us/windows/win32/api/winbase/ns-winbase-file_id_info`, read 2026-08-23: *"The 128-bit file identifier for the file. The file identifier and the volume serial number uniquely identify a file on a single computer."* All exact as item 6 carries them.

**Linux — and here the publication says more than the document did.** The kernel's own ext4 documentation, `Documentation/filesystems/ext4/inodes.rst`, `https://git.kernel.org/pub/scm/linux/kernel/git/torvalds/linux.git/plain/Documentation/filesystems/ext4/inodes.rst`, read 2026-08-23, gives the widening condition item 6 quotes — *"If the inode structure size ``sb->s_inode_size`` is larger than 128 bytes and the ``i_inode_extra`` field is large enough to encompass the respective ``i_[cma]time_extra`` field, the ctime, atime, and mtime inode fields are widened to 64 bits."*, with *"the upper 30 bits are used to provide nanosecond timestamp accuracy"* — **and the same document states the default outright, in its *Inode Size* section**: *"By default, ext4 inode records are 256 bytes, and (as of August 2019) the inode structure is 160 bytes (``i_extra_isize = 32``)."* **The document said the default inode size is documented nowhere. The publication is right and the document was wrong**, and item 6 and K13 are corrected to say so. `mke2fs.conf(5)`, read at e2fsprogs' own source, `https://git.kernel.org/pub/scm/fs/ext2/e2fsprogs.git/plain/misc/mke2fs.conf.5.in`, 2026-08-23, is quoted correctly by the document — *"This relation specifies the default inode size if the user does not specify one on the command line."* — and its example fragment sets `inode_size = 256` under `[defaults]` and under `ext4`, and `inode_size = 128` under `floppy`, which is the override case.

**Not covered:** any actual Linux machine. What ext4's documentation gives is the default for a filesystem `mke2fs` formatted without an override; `mke2fs.conf(5)`'s own `floppy` example shows an override exists, and a filesystem that is not ext4 is outside both pages. **And none of these pages touches the earliness half** — whether every real application save moves the path identity on Windows or Linux is Blender's and wanted a blind researcher, exactly as item 6 says. **PERMANENTLY UNRESOLVED, 2026-09-02:** the Creator ruled on 2026-08-31 that there will be **no further Blender research and no further rental to answer a question**, so no researcher is coming and no Windows machine is either. **The reading route was taken to its end first** — the granularity half was answered by reading on 2026-08-23 and is what most of this entry now is, and on Windows Microsoft's own *File Times* page states the negative outright rather than leaving it untested. **What that costs is `File Browser` §13 item 6's, restated as permanent there rather than duplicated here**: Row 201's third clause's saying and offering does not fire where a real save moves no path-visible value, while the no-corruption promise itself holds on all three systems because K9's gate is contents arithmetic.

### J3. Section 13 item 9 — why two of the four identity values have no route but the command channel (documented and observed)

**Clauses:** [Section 13](../File%20Browser.md#13-unresolved) item 9's *"Revision 3's attribute structure carries no inode and no change time"*, and the five stable-file-identity consumers it names at [X12](../File%20Browser.md#44-the-partial-never-counted-row-197-and-row-191s-exclusion) and [M7](../File%20Browser.md#74-copy-rows-192-and-197s-shape-machine-locally).

- **Which revision OpenSSH implements.** OpenSSH's own `PROTOCOL` file, `https://raw.githubusercontent.com/openssh/openssh-portable/master/PROTOCOL`, read **2026-08-23**: *"Note that OpenSSH's sftp and sftp-server implement revision 3 of the SSH filexfer protocol described in:"*
- **What revision 3's attribute structure holds.** `draft-ietf-secsh-filexfer-02`, section 5 *File Attributes*, address `https://datatracker.ietf.org/doc/id/draft-ietf-secsh-filexfer-02.txt`, read **2026-08-23** and quoted at `Dependency Upload & Remap` Q5 route 1: the fields are *"uint32   flags"*, *"uint64   size"*, *"uint32   uid"*, *"uint32   gid"*, *"uint32   permissions"*, *"uint32   atime"*, *"uint32   mtime"*, *"uint32   extended_count"*, *"string   extended_type"*, *"string   extended_data"*, with *"The `atime' and `mtime' contain the access and modification times of the files, respectively. They are represented as seconds from Jan 1, 1970 in UTC."* **So: no inode, no file identifier, no change time, and no sub-second granularity.**
- **Observed.** Protocol version **3** announced by every rental posed — four boxes across three sellers: [One box with Blender on it](../One%20box%20with%20Blender%20on%20it%20—%20the%20reading,%20the%20listing,%20and%20the%20names%20that%20do%20not%20come%20back.md) §3.1, 2026-08-17, read raw off the wire as `VERSION packet type=2 version=3`; [The multi-provider batch](../The%20multi-provider%20batch%20%E2%80%94%20the%20surface%20that%20did%20not%20vary,%20the%20Debian%20route%20that%20exists,%20and%20the%20cards%20a%20slice%20cannot%20see.md) group 1, 2026-08-22, machines A, C and D.

**Not covered:** these are other people's stock servers; that a server Smoozy has not met announces revision 3 is an expectation rather than an observation, which `Server protocol and transfers` §7.4 now states in its own words. **Which program answers a four-value reading on an arbitrary rental, and whether its format holds, is item 9's own subject and no source in this project answers it.**

### J4. Section 13 item 10 — a streaming Zstandard decode from a bounded prefix (documented, read 2026-09-04; and observed in a controlled test, run 2026-09-04)

**Clauses:** [L8](../File%20Browser.md#13-the-blend-version-row-182)'s attempt order and [L9](../File%20Browser.md#13-the-blend-version-row-182)'s grown prefix, and [Section 13](../File%20Browser.md#13-unresolved) item 10, which this entry closes. **The reading was taken at a Build sitting on 2026-09-04**, which is where the decoder was pinned; it is recorded here because the item's ground is a publication and this is that publication's home.

**The format's own specification.** RFC 8878, *Zstandard Compression and the 'application/zstd' Media Type*, `https://www.rfc-editor.org/rfc/rfc8878.html`, read 2026-09-04:

- §2: *"each block can be decompressed without waiting for its successor"*.
- §3.1.1.1: the frame header is *"a variable size, with a minimum of 2 bytes up to a maximum of 14 bytes depending on optional parameters"*, and the magic number is *"4 bytes, little-endian format. Value: 0xFD2FB528."*
- §3.1.1.2.4: *"The size of Block_Content is limited by Block_Maximum_Size, which is the smallest of: Window_Size [or] 128 KB"*.

Label: **documented**, first-party. **What it establishes is the format and not an implementation:** nothing in the format requires the whole frame before the first bytes, so L9's premise is sound at the format level.

**The library's own publication.** `ZSTD_decompressStream`, Zstandard's own manual, `https://facebook.github.io/zstd/zstd_manual.html`, read 2026-09-04: it *"tries to flush all data decoded immediately, respecting output buffer size"*, and `ZSTD_DStreamOutSize()` is the *"recommended size for output buffer. Guarantee to successfully flush at least one complete block in all circumstances."* Label: **documented**, first-party.

**And two implementations were run, because the item's own words are that the assumption is *not established for any particular implementation*.** Run 2026-09-04 at the desk, against frames whose first bytes were the observed opening text of a `.blend`, at three compression levels, on compressible and incompressible bodies:

- **The C library reached the opening text from 16 to 43 bytes of prefix at every level**, on every body.
- **A pure-Rust decoder needed between 208 bytes and the whole file — 8,388,810 bytes at the highest level.** The mechanism is its own and is stated in its own source: it hands out only what has left the frame's declared window, which its `can_collect` does by calling `can_drain_to_window_size` while the frame is unfinished. **The window is the writer's choice**, so that need is bounded by nothing this project can read.

Label: **observed in a controlled test**. **What was pinned on it** is the C library — `zstd` at `=0.13.3`, default features off, in `App/Cargo.toml` with the measurement written beside it. MIT by its own manifest; the C sources it builds are Meta's under the BSD licence.

**Not covered.** **What a `.blend` declares as its window is established by nothing here** — no version's documentation describes the file's inside (item 2), so the pure-Rust decoder's need was measured on frames this project wrote rather than on Blender's own. **Nothing was run on Windows or Linux**, and the pinned library builds C sources, which is the one part of this reading that could behave differently per system. **The measurement is of the first decompressed bytes and not of a whole decode**, which is all L8 asks for. **No Blender fact was looked for and none is claimed**: the opening text used is the one already observed and carried at **B3**.

---

## K. Assumptions and unresolved, gathered

Recorded once here so a reader sees the whole set. Each has its full entry above.

1. **A streaming Zstandard decode yields the first decompressed bytes from a bounded compressed prefix** ([L9](../File%20Browser.md#13-the-blend-version-row-182), [Section 13](../File%20Browser.md#13-unresolved) item 10). **Closed 2026-09-04 at a Build sitting — entry J4.** The format's own specification and the library's own manual were read, and two implementations were run: **the C library reaches the opening text from 16 to 43 bytes of prefix and a pure-Rust one can want the whole file**, so the assumption is a property of the implementation and the pinned one has it. **What breaks is still cost only** — the iteration fetches more input, and past the cap the version tag is blank, which is [L8](../File%20Browser.md#13-the-blend-version-row-182)'s honest answer either way. *(This read* **An assumption. No source in this project** *until 2026-09-04, and the search behind that sentence stands as a record of where it had been looked for: the Blender Research comparison at* Reading a file 1 *— which establishes that outside tools can undo the compression and nothing about any implementation's streaming behaviour —* [Facts.md](../Facts.md)*, and every* `Sources/` *file under this folder.)*
2. **Mode bits do not answer whether this login can read into a folder** ([P2a](../File%20Browser.md#21-the-limit-and-the-walk-that-recovers-the-separation-row-181)). **An assumption** — entry **C3**.
3. **When `posix-rename@openssh.com` entered OpenSSH** ([X7](../File%20Browser.md#42-the-rule-collisions-are-judged-under-row-192), [Section 13](../File%20Browser.md#13-unresolved) item 12). **Closed 2026-08-23 at the publications** — added in OpenSSH 4.9, released 2008-03-31, by that version's own release announcement; **the `PROTOCOL` attribution the document carried is refuted**, and *every release since* is stated by no publication — entry **D4**.
4. **P6's two runs** ([P6](../File%20Browser.md#22-the-elevated-retry-row-180)). **Unresolved, no observation record** — entry **C6**.
5. **The 2026-08-20 subsystem status run** ([P1](../File%20Browser.md#21-the-limit-and-the-walk-that-recovers-the-separation-row-181)). **Recorded but thin, and the document now claims only the stat half on it** — entry **C2**; the readdir half and every clause below are carried independently by Fact 7.
