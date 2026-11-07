# Sources — Dependency Upload & Remap

*Where the outside-world facts [Dependency Upload & Remap](../Dependency%20Upload%20&%20Remap.md) labels in place came from — the exact words, the addresses, the dates read, and what each does not cover. A decision Smoozy made gets no entry.*

**What is deliberately not here.** The document's binding section routes two whole classes, so neither is re-cited: **machine claims** to the frozen evidence documents in `Preparation/Architecture/`, and **Blender facts** to `Preparation/Blender Research/Across topics/What has to travel with a file.md` and the comparison questions that document and `Preparation/Specs/Sources/Dependency Upload & Remap.sources.md` name. Sections 2 and 3 below are pointers to those owners rather than second citations. **The routing does not reach the waiver**, because Section 1 quotes nine outside publications directly. **All nine now carry an address**; the four that did not were read on 2026-08-23 and are recorded at 1.6 to 1.9.

**And the Blender routing carries a named exception, which the binding sentence used to swallow.** Three of the facts this document leans on hardest — **Facts 99, 100 and 101** at G27, M8, M9, M9a, M9b and **M9d** (added 2026-09-02, the per-pair *established / not established* field on the marker, which reports from those two facts and from nothing else) — come from a rented-machine batch and not from the joined answer or any comparison question; the batch's own words are *"None of this went to Blender Research, per the Creator's standing instruction of 2026-08-22, and none of it is a Blender Research question"*. They are **machine evidence, labelled observed**, and are cited at 2.14 to 2.16 with that route's own limits. **Repaired in the body 2026-08-23**: the binding sentence now names the exception instead of stating a rule the three break.

Claims consumed from other Architecture plan documents — `Job files and outputs`, `Server protocol and transfers`, `State, identity and recovery`, `Desktop app and delivery`, `File Browser`, `Rented-server runtime` — carry their citations in those documents and are named here only where this document restates an outside fact in its own words.

---

## 1. Outside publications read directly

### 1.1 OpenSSH's own `PROTOCOL` — which filexfer revision is in play

**Quoted at [Q5](../Dependency%20Upload%20&%20Remap.md#21-already-on-the-machine) route 1.** Label: **documented**, first-party — the file that opens *"This documents OpenSSH's deviations and extensions to the published SSH protocol."*

**Address:** `https://raw.githubusercontent.com/openssh/openssh-portable/master/PROTOCOL`, the same file `Sources/Server protocol and transfers.sources.md` and Facts.md Fact 93 cite. **Read for this sentence 2026-08-23**, and the sentence as published is *"Note that OpenSSH's sftp and sftp-server implement revision 3 of the SSH filexfer protocol described in:"*.

**The reading date is new and the quote was widened to the published sentence — 2026-08-23.** No date was recorded anywhere in this project for this sentence: it appears in `Server protocol and transfers` §7.4, in this document, and in Facts.md Fact 21's addendum, all dated 2026-08-21 in their own added-on notes and none of them saying when the file was read for it. **The publication and the document agree** — what the document carried was a faithful substring of the published sentence, and the body now quotes the sentence whole. The project separately records the file read **2026-08-20** (§4.3) and **2026-08-22** (§4.1) for other sentences. **Not covered:** which revision an arbitrary rental's own server implements — see 2.13 for what was observed on four of them.

### 1.2 `draft-ietf-secsh-filexfer-02` — the version-3 attribute structure

**Used at [Q5](../Dependency%20Upload%20&%20Remap.md#21-already-on-the-machine) route 1 and [§12 item 19](../Dependency%20Upload%20&%20Remap.md#12-unresolved).** Label: **documented**.

**Address:** `https://datatracker.ietf.org/doc/id/draft-ietf-secsh-filexfer-02.txt`, the same address `Sources/Server protocol and transfers.sources.md` carries, where it is recorded read **2026-08-22** for §6.9 and §6.10. **Read for the attribute structure 2026-08-23**, section 5 *File Attributes*.

**Quoted words supplied 2026-08-23; until then the claim was paraphrase in three documents and quoted in none** (this one, `Server protocol and transfers` §7.4, Facts.md Fact 21). Section 5 opens *"A new compound data type is defined for encoding file attributes"* and gives the structure as, in full and in order:

```
uint32   flags
uint64   size           present only if flag SSH_FILEXFER_ATTR_SIZE
uint32   uid            present only if flag SSH_FILEXFER_ATTR_UIDGID
uint32   gid            present only if flag SSH_FILEXFER_ATTR_UIDGID
uint32   permissions    present only if flag SSH_FILEXFER_ATTR_PERMISSIONS
uint32   atime          present only if flag SSH_FILEXFER_ACMODTIME
uint32   mtime          present only if flag SSH_FILEXFER_ACMODTIME
uint32   extended_count present only if flag SSH_FILEXFER_ATTR_EXTENDED
string   extended_type
string   extended_data
```

with *"The `flags' specify which of the fields are present. Those fields for which the corresponding flag is not set are not present (not included in the packet)."* and *"The `atime' and `mtime' contain the access and modification times of the files, respectively. They are represented as seconds from Jan 1, 1970 in UTC."*

**The publication agrees with what the document carried.** The listed fields are exactly the document's *flags, size, owner and group, permissions, access and modification times, and extensions*; there is no inode, no file identifier and no change time in the structure; and `mtime` is a `uint32` of seconds, which is the document's 32-bit count of seconds. Nothing was corrected — what was owed was the words. **Not covered:** the structure's `extended_type`/`extended_data` tail is open by design, so nothing here bounds what a server may carry under an extension it announces; and this is the specification rather than any server's behaviour.

### 1.3 `[MS-FSCC]` §2.4.35 — FileNormalizedNameInformation

**Quoted at [F14a](../Dependency%20Upload%20&%20Remap.md#43-the-exact-match-and-the-near-miss--row-166), address and date carried in the document itself.** `https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-fscc/20bcadba-808c-4880-b757-4af93e41edf6`, read **2026-08-21**:

- *"A normalized name is an absolute pathname where each short name component has been replaced with the corresponding long name component, and each name component uses the exact letter casing stored on disk."*
- Footnote 139 to the same section: *"This information class is implemented on ReFS and NTFS file systems. Other file systems return STATUS_INVALID_DEVICE_REQUEST."*

Label: **documented**, the vendor's own published protocol specification. Also carried as Fact 121. **Not covered:** nobody has run the call — [§12 item 21](../Dependency%20Upload%20&%20Remap.md#12-unresolved); whether the returned absolute pathname carries a volume prefix, which is A4's open question and [§12 item 3](../Dependency%20Upload%20&%20Remap.md#12-unresolved)'s; NTFS's own folding rule, undocumented on every page read; and every volume that is neither NTFS nor ReFS.

### 1.4 `minwinbase.h` — the `FILE_INFO_BY_HANDLE_CLASS` enumeration

**Cited at [F14a](../Dependency%20Upload%20&%20Remap.md#43-the-exact-match-and-the-near-miss--row-166)** as the Win32 surface, `GetFileInformationByHandleEx` with `FileNormalizedNameInfo`. `https://learn.microsoft.com/en-us/windows/win32/api/minwinbase/ne-minwinbase-file_info_by_handle_class`, read **2026-08-21**. Label: **documented**.

**Nothing is quoted from it and that is the point of citing it:** the page carries the class and gives it no description of its own, which is why thirteen manual and reference pages missed it (Fact 121). **Not covered:** the page establishes the class exists on the Win32 surface and nothing about behaviour; behaviour is 1.3's.

### 1.5 The Linux kernel's own `admin-guide/ext4`

**Quoted at [F14a](../Dependency%20Upload%20&%20Remap.md#43-the-exact-match-and-the-near-miss--row-166), address and date carried in the document itself.** `https://docs.kernel.org/admin-guide/ext4.html`, read **2026-08-21**:

- *"The case-awareness is name-preserving on the disk, meaning that the file name provided by userspace is a byte-per-byte match to what is actually written in the disk."*
- *"The Unicode normalization format used by the kernel is thus an internal representation, and not exposed to the userspace nor to the disk, with the important exception of disk hashes, used on large case-insensitive directories with DX feature."*

Label: **documented**, the operating system's own documentation. Also carried as Fact 122. Settles the F8/F14 contradiction in F8's favour **for ext4 and for no other filesystem**. **Not covered:** every folding filesystem that is not ext4; which filesystem a distribution presents; and the unrun observation — [§12 items 11 and 21](../Dependency%20Upload%20&%20Remap.md#12-unresolved).

### 1.6 Microsoft, *Naming Files, Paths, and Namespaces*

**Quoted at [F14](../Dependency%20Upload%20&%20Remap.md#43-the-exact-match-and-the-near-miss--row-166):** *"On newer file systems, such as NTFS, exFAT, UDFS, and FAT32, Windows stores the long file names on disk in Unicode, which means that the original long file name is always preserved."* Label: **documented**.

**Address, recorded 2026-08-23:** `https://learn.microsoft.com/en-us/windows/win32/fileio/naming-a-file`, `ms.date` **2024-08-28**, page title *Naming Files, Paths, and Namespaces*. Originally read **2026-08-20** — F14's own *"three more read on 2026-08-20"*, and `Preparation/Groundwork/Open Threads.md` carries the same sentence from that verification round — **with no address recorded anywhere under `Preparation/` until it was read again on 2026-08-23 for this entry**. **The publication and the document agree**: the sentence is word for word what F14 carries, in the page's *Short vs. Long Names* section, and the `ms.date` matches.

**Not covered, in the document's own words:** the page states the name is preserved and does not state *and its capitalisation*, which is why F14's sentence stands and F14a takes exactness from 1.3's normalized name instead. The page's own *Naming Conventions* section says the opposite way round about lookups — *"Do not assume case sensitivity"* — which is about comparison and not about what a call returns.

### 1.7 `WIN32_FIND_DATAW` (`minwinbase.h`) — the file-enumeration structure's Remarks

**Quoted at [F14](../Dependency%20Upload%20&%20Remap.md#43-the-exact-match-and-the-near-miss--row-166):** *"If a file has a long file name, the complete name appears in the **cFileName** member, and the 8.3 format truncated version of the name appears in the **cAlternateFileName** member."* Label: **documented**.

**Publication identified and addressed 2026-08-23:** `https://learn.microsoft.com/en-us/windows/win32/api/minwinbase/ns-minwinbase-win32_find_dataw`, `ms.date` **2018-12-05**, page title *WIN32_FIND_DATAW (minwinbase.h)*. Originally read **2026-08-20** with 1.6. **The document named it only as *"the enumeration structure's own Remarks"* — no title, no address, no `ms.date`, and two member names as the only identifying detail**, which no later reader could have followed; the structure was found from those two member names on 2026-08-23 and the body now names it. **The publication and the document agree**: the Remarks sentence is word for word what F14 carries.

**Not covered:** what casing the enumeration returns, which F14a is explicitly written not to rest on. Two further Remarks clauses the document does not use and which do not bear on it: `cAlternateFileName` is a `NULL` string when `FindFirstFileEx` was called with `FindExInfoBasic`, and the structure has an ANSI twin, `WIN32_FIND_DATAA`, on the same header.

### 1.8 Microsoft, *Case Sensitivity* (Windows Subsystem for Linux)

**Cited at [F14a](../Dependency%20Upload%20&%20Remap.md#43-the-exact-match-and-the-near-miss--row-166)** as read and answering nothing. Label: **documented**. Nothing is quoted from it.

**Address, recorded 2026-08-23:** `https://learn.microsoft.com/en-us/windows/wsl/case-sensitivity`, `ms.date` **2021-09-27**, page title *Case Sensitivity*. Originally read **2026-08-21**, with no address recorded anywhere under `Preparation/` until the page was read again on 2026-08-23 for this entry. **Re-read confirms the document's characterisation**: the page covers case-**insensitivity**, the per-directory flag set with `fsutil.exe file setCaseSensitiveInfo`, the `system.wsl_case_sensitive` extended attribute and the `wsl.conf` mount options, and **says nothing anywhere about what casing a filesystem call returns**. Recorded so it is not read a third time.

### 1.9 Microsoft, archived *Configuring case sensitivity for file and folder names*

**Quoted at [F14a](../Dependency%20Upload%20&%20Remap.md#43-the-exact-match-and-the-near-miss--row-166):** `ntfscase={upper | lower | preserve}` with *"The ntfscase option sets the case sensitivity for the NTFS file system. The default case sensitivity is preserve (preserve case)."* Label: **documented**.

**Address, recorded 2026-08-23:** `https://learn.microsoft.com/en-us/previous-versions/windows/it-pro/windows-server-2003/cc783185(v=ws.10)`, `ms.date` **2009-10-08**, page title *Configuring case sensitivity for file and folder names*, marked archived and applying to Windows Server 2003 R2. Originally read **2026-08-21**, with no address recorded anywhere under `Preparation/` until the page was read again on 2026-08-23 for this entry. **The publication and the document agree** — the body's quote was the second of the two sentences and has been widened to both, since the first is what ties the option to NTFS. A near neighbour with almost the same title exists at `.../windows-server-2008-R2-and-2008/cc725747(v=ws.11)`; the title, the `ms.date` and the wording all match the ws.10 page, which is the one recorded.

The option belongs to Server for NFS and decides the case **that server** returns. **Not covered:** it says nothing about what a local Windows call returns, and the document uses it only in the opposite direction — a serving layer may hand back a folded spelling.

---

## 2. Machine evidence — pointers to this stage's input documents

Each names the document that owns the citation, the words this document uses, and what the observation does not cover. **One box** is [One box with Blender on it](../One%20box%20with%20Blender%20on%20it%20—%20the%20reading,%20the%20listing,%20and%20the%20names%20that%20do%20not%20come%20back.md); **Six machines**, [Two more boxes](../Two%20more%20boxes%20—%20the%20volume,%20the%20full%20disk,%20and%20a%20day%20left%20running.md), [The last two boxes](../The%20last%20two%20boxes%20—%20the%20hook%20that%20cannot%20be%20read%20back,%20the%20full%20disk,%20and%20the%20unit%20that%20kills%20a%20render.md), [The multi-provider batch](../The%20multi-provider%20batch%20—%20the%20surface%20that%20did%20not%20vary,%20the%20Debian%20route%20that%20exists,%20and%20the%20cards%20a%20slice%20cannot%20see.md), [The rented machine](../The%20rented%20machine.md) and [The user's own computer](../The%20user's%20own%20computer%20—%20where%20a%20download%20lands,%20what%20it%20will%20take,%20what%20it%20will%20show,%20and%20how%20the%20app%20replaces%20itself.md) are named in full.

### 2.1 Fact 18 — nothing on the machine distinguishes a part-arrived file from a whole one

**Owner:** [Six machines](../Six%20machines%20—%20the%20container,%20the%20webhook%20and%20the%20ready%20action.md), Fact 18, 2026-08-15/16. **Used by** the binding section, Q1, Q10, A14, V15 and Section 10's placement row — the single observation this module's whole record-not-presence rule rests on.

**The words this document uses:** killed and dropped transfers leave bytes at the final name with *"identical mode, permissions, extended attributes, inode flags, allocation and file-type output to a whole file"*; an interrupted overwrite left a **40,000,000-byte hybrid**, 10,444,800 new bytes spliced onto 29,555,200 old ones — *"Neither version, plausible size, no marker"*.

**Not covered:** one batch, one provider, Linux at the far end; damage caused by the disk rather than by an interruption is Fact 72's and is not this; nothing on Windows.

### 2.2 Fact 21 — the four-value identity

**Owner:** Six machines, Fact 21, 2026-08-15/16. **Used by** Q5 route 1, K7, M22f and V10.

**The words:** *"yes, from size, inode, modification time and change time together"* — a save-by-rename changes the inode, and a hand-restored modification time still moves the change time, which cannot be set back. Posed live: a byte changed mid-fetch, the copy returned 0 and delivered a different file.

**Not covered, and it is load-bearing here:** the entry records no channel. Two of the four values are absent from the file-transfer surface (1.2), so the reading is a command-channel one on unestablished ground — [§12 item 19](../Dependency%20Upload%20&%20Remap.md#12-unresolved) and 4.2 below. Re-posed on `overlay` in the multi-provider batch's Test 23 and not on any other rented filesystem.

### 2.3 Fact 15 — the contents comparison and what it costs

**Owner:** Six machines, Fact 15, 2026-08-15/16, on a real **669,966,622-byte** `.blend`. **Used by** Q2, Q3 and Q4.

**The words:** machine warm md5 0.89 s, sha256 2.24 s, sha512 1.53 s; Mac md5 1.26 s, sha256 2.22 s; every digest identical at both ends. `openssl dgst -sha256` **0.46 s** against `sha256sum`'s **2.24 s**, same file, same box. And the clause Q4 quotes: an end-to-end check at about a gigabyte a second is available *"on processors with the instructions that make it fast, which both ends here had and a rental may not."*

**Not covered:** which programs an arbitrary rental carries — the reason Q3 puts the digest in the package's own code; and the Mac's own implementation gap, which the entry asserts in direction only and gives no figures for. **The two Mac figures are two digest functions and not two implementations of one, and Q2 was repaired to say so on 2026-08-23** — see 5.1.

### 2.4 Fact 5 — the reading, and what it costs

**Owner:** One box §1 and §1.5, 2026-08-17, in **4.5.9, 5.0.1 and 5.2.0**. **Used by** G1 and G5.

**The words:** one SSH command, a named Blender already on the machine, headless, no window and no display server, the reading returned as one document between `SMOOZY_READ_START` and `SMOOZY_READ_END` sentinels; *"About 3.2 to 3.8 seconds end to end including a fresh connection; the reading itself 0.09 to 0.12 seconds"*.

**Not covered:** **5.1 was not installed and nothing there speaks for it** (G4, [§12 item 5](../Dependency%20Upload%20&%20Remap.md#12-unresolved)); one container on `overlayfs`, one provider, one scene; and **whether a read disturbs a running render is not answered** — five reads during a render all succeeded and the render finished whole, but three testers shared four cores at load average 2.5 to 6.5 and the loads cannot be separated (One box §1.6; [§12 item 16](../Dependency%20Upload%20&%20Remap.md#12-unresolved)). Every timing in that document is an upper bound.

### 2.5 One box §1.2 — what each way of listing reaches

**Owner:** One box §1.2. **Used by** G6.

**The words:** the two bulk means miss every write location; the walk over Blender's own marks on its settings reaches all of them and misses the text path the bulk means catch. **Not covered:** run on the rented Linux machine alone, one fixture; the joined answer and the spec's own sources file own the wider version of this claim at *Reading a file 5, 9* (3.2 below).

**Two corrections, both 2026-08-31, out of the repaired Blender research.** **First, *the two bulk means miss every write location* is no longer what the research carries** — it is now *they reach an ocean modifier's cache path and nothing else recorded for writing*, with what separates that one from the rest not established. Label: **observed in a test**. **Safe in the direction G6 uses it** — the walk is taken and the bulk means are refused as a route, and one reachable write place does not make them sufficient — **but the exception has to be carried**, because *miss every write location* is now a false statement of what the research holds. `Job files and outputs`' sources file carries the same correction from its own side. **Second, the difference between the two listing routes is wider than this entry says, and it bites the gathering rather than the check.** **Where a library points at a picture of its own, the map reaches through and the flat list does not** — so **a job gathered through the flat list leaves behind everything inside a linked library**. Label: **observed in a test**, from ***Dependencies 20*** through [4.5](#45-what-an-artists-ordinary-save-does-to-a-reference-they-never-touched--added-2026-08-29-answered-2026-08-30-entry-closed-2026-08-31). **What breaks:** any gathering step that walks the flat list would upload a file whose linked library's own references never travelled, and **this document is silent on which route the gathering walks** — G6 names the map for the *check* and no clause names a route for the *gather*. **Nothing is decided here**; which route the gathering takes is the document's.

### 2.6 Fact 77 — the rewrite routes

**Owner:** One box §2, 2026-08-17. **Used by** M5, M6, M14, M19 and M9c.

**The words:** the per-setting route works in all three exercised versions and reaches every recorded location in the fixture, write locations included, read back off disk by a separate process; the one-pass route is **absent in 4.5.9** and in 5.0.1 and 5.2.0 *rewrites the images, the text and the library and silently leaves every write location alone* — nothing raised, nothing printed, the save succeeding. The property carrying a File Output node's folder is `base_path` in 4.5.9 and `directory` in 5.0.1 and 5.2.0. Two traps: saving remaps paths relative by default, and from 5.0 adding an output to a File Output node rebuilds its socket list. The artist's file was digested before and after all seven runs and never changed, with two no-op runs as the control.

**Not covered, and M19 rests on it:** **5.1 untested**, and whole kinds never posed — a font, a sound, a volume, a point cache, a bake folder, a geometry-nodes bake folder, a sequencer proxy folder, and a path in a custom property. That limit list is M19's seven plus the custom property, and M19 labels the rewrite of those seven **inferred** ([§12 item 2](../Dependency%20Upload%20&%20Remap.md#12-unresolved)).

### 2.7 Fact 29 — what a name and a whole path may be on the machine

**Owner:** One box §7 and Six machines, Fact 29. **Used by** A5, and by `Job files and outputs` N5, N6 and N13, which own the claim A5 restates.

**The words:** all 33 hostile names — newline, tab, emoji, Cyrillic, Arabic, CJK, decomposed Unicode — reached the machine byte-for-byte; the machine's limits are **255 bytes per name** and a path limit of **4096** bounding the string handed to a system call rather than the tree; a write over the limit is refused at open with `Bad message`, which names nothing.

**Not covered:** one rented filesystem; the failures on the return leg are the Mac's and are Fact 24's, not this module's; nothing on Windows.

### 2.8 Fact 1 — the writability test, and the disk-full negative

**Owner:** [The last two boxes](../The%20last%20two%20boxes%20—%20the%20hook%20that%20cannot%20be%20read%20back,%20the%20full%20disk,%20and%20the%20unit%20that%20kills%20a%20render.md). **Used by** the binding section and M12.

**The words:** *"On the container, `os.access(W_OK)` answered `True` on a filesystem with literally zero bytes free, while a real one-byte write to the same three areas failed with errno 28 every time."* And the disk-full form of the re-run: after space is freed the container heals by itself while the systemd unit did not recover merely because space was freed.

**Not covered:** two boxes on one day; the reboot and crash halves are answered yes on both machine kinds and the disk-full half only on those two.

### 2.9 Fact 78 — publication by one rename, at every interruption boundary

**Owner:** The last two boxes §8, 2026-08-18, on **ext4, xfs and overlayfs**, every death a real forcible kill. **Used by** M16 and Section 10.

**The words:** no part-arrived bytes at a final name at any boundary, on any of the three.

**Not covered:** the rollback boundary could not be posed; the `pointer` and `pointer-done` boundaries are told apart only by reading the pointer; one run each per filesystem; nothing here is about the desktop end.

### 2.10 Fact 6 — where things sit and what survives

**Owner:** [Two more boxes](../Two%20more%20boxes%20—%20the%20volume,%20the%20full%20disk,%20and%20a%20day%20left%20running.md), 2026-08-16/17. **Used by** A1 and A13.

**The words:** everything on both disks survived a stop and a start with **markers byte-identical**, the volume remounted at the same place and the folders intact; a rental's second place to write arrives *"root:root 755"* and refuses the non-root login until a setup step hands it over.

**Not covered, and A13 and [§12 item 4](../Dependency%20Upload%20&%20Remap.md#12-unresolved) say so:** *"the machine's own unattended housekeeping over days was not posed"*, and neither was the recycle a container does. The volume there was a quota on one physical disk — see 2.11.

### 2.11 Two real block devices — the answered half of §12 item 4

**Owner:** The multi-provider batch, 2026-08-22. **Used by** [§12 item 4](../Dependency%20Upload%20&%20Remap.md#12-unresolved).

**The words:** one seller ships two real block devices as standard, `/dev/vda1` at 97 GB on `/` and `/dev/vdb` at 787 GB on `/ephemeral`. **Not covered:** nothing was tested about whether that large mount survives a stop, a start or a restart, and its name is the seller's claim rather than an observation.

### 2.12 Test 24 — the rented filesystems are case-sensitive

**Owner:** The multi-provider batch, Test 24, 2026-08-22, on three machines across three sellers. **Used by** G12's machine half.

**The words:** `casefile.txt` and `CASEFILE.TXT` held different contents and listed as two entries; `stat` on a mixed-case form returned not-found; both rented filesystems are case-sensitive and neither folds.

**Not covered, quoted in G12:** *"a folding directory returning the name it was given was not posed, because no folding directory was found to pose it on"*. The folding question is 1.5's for ext4 and nobody's elsewhere.

### 2.13 Protocol version 3 announced on four rentals

**Owner:** One box §3.1 (2026-08-17) and The multi-provider batch group 1 (2026-08-22, machines A, C and D across three sellers). **Used by** Q5 route 1's *"the revision in play"*.

**The words:** protocol version 3 on all three multi-provider machines — the batch's own surface table reads *"Protocol version announced | 3"* — and the extension set, announced limits and session cap byte-identical across them; the announced extension set varied only in extensions Smoozy does not use.

**This entry is now what Q5 route 1's observed half rests on, from 2026-08-23.** The `Remote version: 3` session that clause used to name is struck (4.3), and the sentence re-points here: **protocol version 3 announced by every rental posed, four boxes across three sellers**. **Not covered:** these are rented servers rather than anything Smoozy ships, which is why the body says *announced by every rental posed* and claims nothing wider; the 32,768-byte fallback branch has never been reached; four boxes, three sellers, Linux only.

### 2.14 Fact 100 — the Blender-application entry's recognition mark

**Owner:** The multi-provider batch, Fact 100, 2026-08-22, on machine A in **4.5.12 and 5.2.0**. **Used by** G27 and Row 171.

**The words:** on a file saved by each Blender from its own factory startup touching nothing, `bpy.utils.blend_paths()` returned **two entries**, both the same path into the application's own datafiles, each carried by a Material — `Dots Stroke` and `Material`, `id_name` `MADots Stroke` and `MAMaterial` — arriving as a **`library_weak_reference`**; `bpy.data.libraries` was empty and nothing in `images`, `libraries`, `sounds`, `fonts`, `movieclips`, `cache_files`, `volumes` or `texts` carried the path. `bpy.ops.file.report_missing_files()` printed `Info: No missing files` for a path that would exist on no other machine.

**Not covered:** two versions, one machine, one factory-startup file; nothing establishes the mark on a file a user made, only that a user's own reference always holds a datablock.

### 2.15 Fact 99 — the image kind needs no reload, and the `has_data` trap

**Owner:** The multi-provider batch, Fact 99, 2026-08-22, machine A, 4.5.12 and 5.2.0. **Used by** M9, M9a, M9b and [§12 item 1](../Dependency%20Upload%20&%20Remap.md#12-unresolved).

**The words:** the fixture's recorded locations were **absent at load**; original pure red, alternate pure green; as loaded `[0.741, 0.400, 0.745]` — the missing-texture colour; after assigning the new path with no reload `[0.145, 0.557, 0.000]`; after `image.reload()` unchanged; a freshly loaded datablock in the same slot as control. `image.has_data` reported **`False`** throughout while the render read the file correctly.

**Not covered, and M9b carries it:** **cache file** — the datablock had no user after the import and was dropped on save; **sound** — assigned and read back correctly, but a sound has no render to observe; **font, movie clip and volume** — not posed. Those five are unanswered and are not claimed.

### 2.16 Fact 101 — a library that never loaded is populated by assign and reload

**Owner:** The multi-provider batch, Fact 101, 2026-08-22, both versions. **Used by** M8 and closes [§12 item 17](../Dependency%20Upload%20&%20Remap.md#12-unresolved).

**The words:** at load the library was genuinely unresolved — `Warning: Unable to open '…/orig/lib.blend': No such file or directory`, `Info: Cannot find lib`, `LIB: Object: 'LinkedThing' missing`, `is_missing=True`, the linked object an `EMPTY` with `data = None`. After assigning the path with no reload: `('LinkedThing', None, -1, -1)`. After `library.reload()`: `('LinkedThing', 'Cone', 4, 4)` and `is_missing=False`.

**Not covered:** one machine, two versions, one library; the relocate command was not posed in this form.

### 2.17 Fact 16 — the stored-name reading on macOS

**Owner:** [The user's own computer](../The%20user's%20own%20computer%20—%20where%20a%20download%20lands,%20what%20it%20will%20take,%20what%20it%20will%20show,%20and%20how%20the%20app%20replaces%20itself.md) §3, the reading batch of 2026-08-18. **Used by** F9, F10, F12 and F14.

**The words:** `getattrlist(ATTR_CMN_NAME)`, `fcntl(F_GETPATH)` and the C `realpath(3)` return the name **as stored**, and comparing those bytes against the bytes asked for is the whole test; **APFS stores names byte for byte** and normalises only its lookup hash; folding is Unicode **full** folding, predicting all fifteen observed pairs where simple folding mispredicts three. The trap F9 quotes: *"any canonicaliser that manipulates strings rather than calling the system"* — the first run reported `realpath` echoing back what it was given, an artefact of one language's own pure-string implementation, caught by the citation check.

**Not covered:** one Mac, one case-insensitive APFS volume at default settings; which Unicode version macOS folds by; a case-sensitive APFS volume, HFS+ and network mounts; and the whole Windows half, which 1.3 and 1.6 to 1.9 narrow and do not close. **A5's 765-byte figure is not this fact's**: it is Fact 24's, answered the same day by the same batch in the same document, and A5 was corrected to say so on 2026-08-23 — see 5.2 and 2.19.

### 2.18 The space in a folder name

**Owner:** [The rented machine](../The%20rented%20machine.md). **Used by** A8.

**The words:** the folder name has a space in it, and that space broke the copy **three different ways in three tools** — one wanting the space escaped with a backslash, one splitting the plainly quoted path in two.

**Not covered:** one box, three tools; it is a quoting-level finding about tools and says nothing about what the filesystem accepts, which is 2.7's.

---

### 2.19 Fact 24 — the 765-byte macOS name

**Owner:** [The user's own computer](../The%20user's%20own%20computer%20—%20where%20a%20download%20lands,%20what%20it%20will%20take,%20what%20it%20will%20show,%20and%20how%20the%20app%20replaces%20itself.md) §3, the same reading batch of 2026-08-18 that answered Fact 16. **Used by** A5. **Added 2026-08-23**, the figure having been carried under Fact 16's number until then.

**The words:** Windows and macOS both count **255 UTF-16 code units** while **Linux stands alone with the binding limit** of 255 bytes; the macOS figure is *"measured, not documented"*, and Apple's own two manual pages contradict each other — `pathconf(2)` says bytes, `getattrlist(2)` says characters — *"with a 765-byte name accepted and a 128-character name refused"*.

**Not covered:** one Mac, one volume, one measurement; the Windows half is still owed and the fact says so. `Download` §15 item 8 and `Desktop app and delivery` C5 cite the same figure under the same number.

## 3. Blender — pointers to the joined answer and the comparisons this document names

**Nothing here is re-derived and nothing is assembled from topics.** The joined answer is [What has to travel with a file](../../Blender%20Research/Across%20topics/What%20has%20to%20travel%20with%20a%20file.md); the per-claim citations live there and in `Preparation/Specs/Sources/Dependency Upload & Remap.sources.md`. Each row below names the comparison the document itself names. **Eight clauses named none until 2026-08-23; every one of them now does, and the *Gap* column records what was added and where the name came from.** No comparison file was opened to fill them: each name is the one `Preparation/Specs/Sources/Dependency Upload & Remap.sources.md` or the joined answer already gives for that claim, which is the only route [Method](../Method.md) allows.

| Claim, and where it is used | What the document names | Gap |
|---|---|---|
| A recorded location is exempt from the rule that a file's text is all written one way — *"real disks hold names that are not"* (G3, G3a, §12 item 18) | ***Dependencies 9***, quoted | — |
| A linked file present with the named thing inside it gone: the name is the whole identity, *"with only a count of missing libraries telling a lost file from a lost item inside a file that loaded"*, what is left keeps the name and the library and holds nothing, and the render still writes a frame (G6a, G6b, §12 item 20) | ***Dependencies 8, 11***, through the joined answer, quoted | — |
| An import node's path takes no part in 4.5's path housekeeping and takes part in all of it from 5.0; `//` inside a **linked** node group's import node is measured from the library file; packing never carries an imported model (G8) | ***Node setups 1, 2***, through the joined answer | — |
| Blender's one-library rule — two recordings differing only in how the path is written are one library, two recordings reaching one file by two names are a library each (G11) | ***Dependencies 19***, through the spec | — |
| Scene attribution needs a walk over the record of what uses what; no single call does it for anything but an object, and the obvious shortcut comes back empty (G19) | ***Reading a file 7***, through the joined answer | — |
| A missed set member goes to the substitute colour on its own while the rest of the picture is unharmed (G22) | ***Images 9***, through the joined answer | — |
| The Blender-application entry is listed among the file's own paths in all four versions and Report Missing Files does not name it (G26) | ***Dependencies 1*** and ***Reading a file 5***, both quoted | — |
| Assigning a library's path *"changes what is recorded at once but leaves the old data in place until the library is told to reload"*, and *"Whether writing a path reloads anything other than a library"* is Not established (M8, M9c, §12 item 17) | ***Reading a file 5***, quoted both ways | — |
| A rewrite shows in the running Blender and never reaches the file on disk unless something saves it (M1, M2) | `Job files and outputs` **5.1 D5**, which owns it; its evidence is Fact 77 (2.6) | — |
| The padding of a frame-number run is a floor rather than a ceiling (G22) | `Job files and outputs` **I3**, named as the recipe fact's owner | — |
| Forms that stand for something are never resolved as text; a curly-brace expression is allowed only in the two write places; `~` and an environment variable are stored exactly as written and never expanded (G9) | ***Dependencies 4*** for one reference meaning many files, ***Dependencies 5*** for the ways a location can be written and the two forms Blender never works out | **Named at the clause 2026-08-23**; both names are the spec's own sources file's |
| A wrongly capitalised library location fails on Linux and quietly succeeds on the Mac, leaving two libraries where one was meant (G12, Mac half) | ***Dependencies 19***, which carries that pair in those words and says it settles those two machines and not the kind; ***Dependencies 8*** for the general rule that capitals are the disk's decision. The machine half is 2.12 | **Named at the clause 2026-08-23 — and the name this row used to offer was wrong.** *Reading a file 10* is Blender's own **search** minding capitals, a different claim; the library pair is *Dependencies 19*'s |
| A chain is recorded to at least nineteen libraries deep; a relative location inside a linked file is measured from that file's own folder; the readable chain stops at the first absent file (G13, G14) | ***Dependencies 13*** for the depth and the measured-from-the-library rule, ***Reading a file 8*** for the chain stopping at the first absent file | **Named at the clause 2026-08-23**; both are the spec's own sources file's. *Node setups 2*, which this row used to list, is G8's import-node claim and not this one |
| 5.2 alone can be asked to expand a set, and for one level of an unopened file's own links; 4.5, 5.0 and 5.1 hand back the written location (G14, G21) | ***Dependencies 4*** for the set expansion, ***Reading a file 8*** for the one-level listing of an unopened file's links | **Named at the clause 2026-08-23.** *Dependencies 4* also records that it does not cover how Blender decides which files belong to a sequence, which is G22's own stated limit |
| Blender's own search and repair stand behind nothing, for four reasons — exact capitals, largest file on disk with nothing said about a choice, the wider form silently repointing unbroken references, and **the form a repaired location is written back in not being established** (F5, M21) | ***Reading a file 10***, which the spec's own sources file names for all four | **Named at both clauses 2026-08-23. The fourth reason was corrected 2026-09-02** — it read *a relative location written back "although every version's manual says recoveries are made absolute"*, and that claim is withdrawn: there is **no disagreement between the comparisons and none with the manuals** (section 7 below) |
| ~~Whether Blender's exact-match strictness is Blender's rule or the disk's is unresolved~~ **SETTLED — it is Blender's own rule, not the disk's** (F15) | ***Reading a file 10***, which used to record the unresolved and now settles it | **Named at the clause 2026-08-23. Settled 2026-08-31, out of the repaired Blender research** — this row said the question was unresolved and that the step to *Blender's own rule* did not follow; the repaired question makes that step, so the row's caveat is spent. Label: **observed in a test**. **Nothing was decided on it and nothing moves** — F15 says so itself, calling it unresolved and building on it nowhere — so what changes is the label alone, and F15's own text is where the *unresolved* must come out |
| The seven findings about what a touched file does to a running render (V8) | ***Reading a file 11*** for six of them, ***Node setups 1*** for the imported model re-read only where its recorded time moved by a whole second, which that question settles **on the rented Linux machine alone** | **Named at the clause 2026-08-23**; both are the spec's own sources file's |
| From 5.0, packing linked data leaves two entries with one recorded location (G11) | ***Dependencies 19***, which carries packing as the deliberate exception to its own one-library rule | **Named at the clause 2026-08-23.** This row had no name at all before; it is the same comparison the rest of G11 already cited |
| The kinds a file can point at, and no version's sources claiming the list is complete (G6, G7, §12 item 13) | ***Dependencies 1***, through G26's citation and the spec's own standing unresolved | — |
| A packed picture is unaffected by its file on disk (G6, the sentence on what is carried inside the file) | ***Dependencies 3***, the spec's own sources file's name for packing item by item | **Named at the clause 2026-09-28.** The property that marks each kind packed is quoted at the code site from Blender's published reference, per version, and is not repeated here |
| Every new scene carries `//` as its bake output path (G6, the bake output exception); a still picture with a `#` in its name opens by that name (G9) | **Not the Blender research.** Observed at the desk during Build — the first in 4.5.12 and 5.2.2, the second in 5.2.2 — and recorded in `Audits/Build/Step 10c/The splash scenes at the desk — what was found.md` | **Added 2026-09-28.** One Mac, those versions only. Neither is leaned on for more than why every file met the bake output case and that a single-file picture's row can be found |
| A render with a cache or bake absent does not always work the simulation out again — a physics cache marked baked with its files missing leaves the object unmoving, a missing geometry-nodes Bake node folder leaves the baked object out, a geometry-nodes simulation never baked stays on its first frame; a placed physics copy is read only with the disk-cache switch left alone and its own stem, a placed geometry-nodes bake only with its folder set as the file loads; opening a Replay fluid file writes frame 1 beside the `.blend` (M22, M22b, §12 item 7, G6) | **Not the Blender research.** Observed at the desk during Build, 2026-10-02, and recorded in `Audits/Build/Step 10c/Simulation files at the desk — what was seen.md`, `Audits/Build/Step 10c/Does the redirect cut the render off from bakes — what was seen.md` and `Audits/Build/Step 10c/The simulation fixes — what each piece is, which files it owns, and in what order.md` | **Added 2026-10-02.** One Mac, from the command line, Blender 4.5.12 and 5.2.2 (the simulation never baked in all four versions; the Replay opening in 4.5.12 only). The spec's own sources file carries the first half; `Job files and outputs`' sources entries 38a to 38c carry the rest |

**What the whole of section 3 does not cover, in the research's own limits:** every rewrite and listing test was run **on the rented Linux machine alone**; nothing on Windows; nothing drawn in a window on Linux; the path-housekeeping test used one image as its only control; and **finding is not the same as rewriting** — for the six kinds only the mark walk reaches, rewriting is marked inferred in every version and no test closed it, which is M19's own inference and [§12 item 2](../Dependency%20Upload%20&%20Remap.md#12-unresolved).

---

## 4. Claims with no source in this project

### 4.1 Two files on one computer cannot sit at the same whole location

**Used by A4 and A6 — the landing rule's whole ground, and [A6](../Dependency%20Upload%20&%20Remap.md#52-the-mirror-rule--row-169) points at this file by name for it.** Label: **assumption**. **No source was found.**

**Where it was looked for:** `Preparation/Specs/Sources/Dependency Upload & Remap.sources.md`, `Preparation/Architecture/Facts.md` in full, the six evidence documents named in section 2, `Preparation/Groundwork/` and `Preparation/Architecture/Job files and outputs.md`. Nothing states it and nothing tests it.

**What breaks if it is wrong:** two different files land on one mirror name and the module's central promise fails silently. **What holds without it, and A6 states the argument:** the two ways it is genuinely stressed — the same location holding a different file later, and a second computer of the user's uploading to the same machine — both land on a mirror location that is already occupied, where Q5's contents comparison reads the difference and Q8's replace-or-stop warning fires before the write. Section 13 records the check against Method's escalations.

### 4.2 Which program answers a four-value reading on an arbitrary rental

**Used by Q5 route 1, K7, M22f and V10 — the record fast path.** Label: **unresolved**. **No source was found**, and naming a program would be inventing one.

**Where it was looked for:** Facts 21, 93 and 7; One box §3 and §5; the multi-provider batch group 1; `File Browser` Section 13 item 9, which is where this document routes it, and which records the same question as its own unestablished ground. The file-transfer surface is ruled out by 1.2. **What breaks:** nothing at the level of the promise — where the reading is unavailable the fast path does not fire and the read-through runs. **What it costs:** a builder reaching for the ordinary listing silently gets two of the four values and a coarser clock on one of those. [§12 item 19](../Dependency%20Upload%20&%20Remap.md#12-unresolved).

### 4.3 The session that negotiated `Remote version: 3` — **struck from the body 2026-08-23**

**Was used by Q5 route 1** — *"a session against this project's own shipped server negotiated `Remote version: 3`"*, labelled **observed** by the body. **No document under `Preparation/` records that session**: no machine, no end, no client, no date. The claim stood only in this document and in `Server protocol and transfers` §7.4.

**Why it was struck rather than relabelled.** An observation cannot be invented, and nothing in this project holds one. Relabelling it *unresolved* would have left a sentence claiming a session happened; the honest repair is that the claim goes and the two things this project does hold take its place.

**What replaced it, both recorded.** **Documented** — OpenSSH's own `PROTOCOL` sentence, now with an address and a reading date (1.1). **Observed** — protocol version 3 announced by every rental posed, four boxes across three sellers (2.13), which the body now cites in as many words. **Nothing downstream moves**: revision 3 was already established both ways, and Q5 route 1's argument was never about which server the session ran against.

**Routed 2026-08-23 and carried the same day at that document's S13.** `Server protocol and transfers` §7.4 carried the same unrecorded session — *"confirmed by a session negotiating `Remote version: 3`"* — and struck it the same day on these same two recorded grounds, keeping the struck words inside its own strike note. **Nothing is owed there.** It is that document's clause; [Section 11](../Dependency%20Upload%20&%20Remap.md#11-what-this-document-supplies-and-to-which-document) names it and this document does not touch it. **Corrected 2026-08-29: this entry read *"Routed, not repaired here … the same replacement is owed there"*, recording as a standing debt something discharged on the day it was routed.**

### 4.4 Four Microsoft publications with no address on record — **closed 2026-08-23**

1.6, 1.7, 1.8 and 1.9 were quoted or cited with a title and a `ms.date` and **no address anywhere under `Preparation/`**; 1.7 carried no title either and was identifiable only by two member names. **All four were read at their own publications on 2026-08-23 and all four now carry an address**, recorded at 1.6 to 1.9 and, for the two the body quotes inline, in F14 and F14a themselves.

- 1.6 *Naming Files, Paths, and Namespaces* — `https://learn.microsoft.com/en-us/windows/win32/fileio/naming-a-file`, `ms.date` 2024-08-28.
- 1.7 the enumeration structure — identified as **`WIN32_FIND_DATAW` (`minwinbase.h`)**, `https://learn.microsoft.com/en-us/windows/win32/api/minwinbase/ns-minwinbase-win32_find_dataw`, `ms.date` 2018-12-05, found from the two member names.
- 1.8 *Case Sensitivity* (WSL) — `https://learn.microsoft.com/en-us/windows/wsl/case-sensitivity`, `ms.date` 2021-09-27.
- 1.9 archived *Configuring case sensitivity for file and folder names* — `https://learn.microsoft.com/en-us/previous-versions/windows/it-pro/windows-server-2003/cc783185(v=ws.10)`, `ms.date` 2009-10-08.

**No publication contradicted what the document carried.** Every quoted sentence is word for word as published and every `ms.date` matches. One quote was **widened rather than corrected** — 1.9's, from *"The default case sensitivity is preserve (preserve case)"* to the two-sentence form that ties the option to NTFS. **What remains not covered is unchanged**: F14a still does not rest on 1.6's or 1.7's content, exactness comes from 1.3's normalized name, and 1.8 and 1.9 still answer nothing.

### 4.5 What an artist's ordinary save does to a reference they never touched — **added 2026-08-29, ANSWERED 2026-08-30, entry closed 2026-08-31**

**The answer, and it is now a source rather than a gap.** **Fact 128**, in `Preparation/Architecture/Facts.md`, **answered 2026-08-30 in the negative**: a plain save does **not** leave an untouched reference's recorded bytes alone. Label: **observed in a test** — settled by test and not by reading, on the Creator's Mac, a rented Linux machine and a rented Windows machine, across 4.5, 5.0, 5.1 and 5.2 matched build for build. **The answer of record is `Preparation/Blender Research/Comparison/Dependencies.md` question 20**, registered as ***Dependencies 20***, and that text moved on 2026-08-30 under the round's sweeps, so any earlier quotation of it is stale; `Facts.md`'s Fact 128 entry carries it for this stage.

**What it establishes, in the terms the body uses.** Almost every recorded place comes back character for character. **Two things move.** The **separator** is re-spelled toward the opening machine's own convention when the file is *opened*, and the plain save writes that in — backslash to forward slash on Mac and Linux, and on Windows every forward slash to a backslash but for a handful of places, nothing else in the text moving. And a **data-block with no users and no fake user is dropped at the save**, taking its recorded place with it, in every version on all three machines. The answer states in its own words that **a whole-file comparison or a checksum says nothing about whether a recorded place moved, and only the path text can be compared.** It also records that the two ways of listing a file's recorded places **return different collections**, one reaching through a linked library where the other does not.

**Where it is used, after the repair of 2026-08-31.** [M4](../Dependency%20Upload%20&%20Remap.md#62-what-the-remap-material-says) and [M4a](../Dependency%20Upload%20&%20Remap.md#62-what-the-remap-material-says) — pairs composed and matched on path text with separators normalised to forward slashes and nothing else altered; [M8](../Dependency%20Upload%20&%20Remap.md#63-how-the-pairs-take-effect-in-the-running-blender) and [M8a](../Dependency%20Upload%20&%20Remap.md#63-how-the-pairs-take-effect-in-the-running-blender) — the same matching, both shapes recorded, the dropped data-block named as a known and benign cause of *not applied*, and the checksum consequence written where a builder meets it; [G6](../Dependency%20Upload%20&%20Remap.md#32-what-the-reading-enumerates--rows-159-and-160) — `bpy.data.file_path_map` named as the route the check walks, `bpy.utils.blend_paths` not equivalent to it and never substituted; and [§12 item 22](../Dependency%20Upload%20&%20Remap.md#12-unresolved), which closes on it.

**What the answer does not cover, carried with it:** every Windows result behind it is a **Server edition**, no desktop edition having been reached; a **cache file, a movie clip and a movie strip** were never in the test file; a place on a **network share** written the way Windows writes one is untested; and **whether a save leaves a linked library untouched was run in one version alone**. §12 item 22 carries the same list on the plan side.

**The route's own exception, stated because the binding sentence would otherwise swallow it.** This is a Blender fact reached through the comparison, so it sits inside section 3's routing rather than beside Facts 99, 100 and 101 — but the comparison question behind it, ***Dependencies 20***, was registered on 2026-08-29 and is therefore **not one of the questions the joined answer or the Specs sources file name**. It is named here instead.

*(Everything from here to the end of 4.5 is the entry as first written, kept rather than deleted, because what it says breaks is what the answer bears on. It described a gap that is now filled; read it as history.)* ~~**Used by [M8](../Dependency%20Upload%20&%20Remap.md#63-how-the-pairs-take-effect-in-the-running-blender)'s narrow *reference re-pointed* reading and by [§12 item 22](../Dependency%20Upload%20&%20Remap.md#12-unresolved).** Label: **unresolved**, and it is **Fact 128** in `Preparation/Architecture/Facts.md`, numbered 2026-08-23 and **NOT ANSWERED**. **No source was found**, and none of section 3's Blender pointers reaches it.~~

**Where it was looked for:** `Preparation/Blender Research/Across topics/The map.md` and the joined answer [What has to travel with a file](../../Blender%20Research/Across%20topics/What%20has%20to%20travel%20with%20a%20file.md), the comparison questions section 3 names, `Preparation/Specs/Sources/Dependency Upload & Remap.sources.md`, and `Preparation/Architecture/Facts.md` in full. Nothing states what a plain save does to a reference the artist never edited; the register's own entry says the same and records that it is a **Blender question belonging to Blender Research rather than to this stage**. The one adjacent thing this project holds is Fact 77's save-time trap, which M14 carries: a save remaps paths relative by default unless the save-time relative-remapping is turned off. **That is Smoozy's own save rather than the artist's, and it is the ground for doubting rather than an answer** — the register's own entry says the same.

**What breaks if it goes the wrong way:** M4 composes every pair on the recorded location's expected bytes and M8 matches on those bytes across exactly the re-upload `Specs/Whole app` step 11 promises, so a save that re-spells references makes **every pair of every re-uploaded file miss at once**, all reported *not applied*, with the render finishing at success and nothing pointing where it should. **What holds without it:** the marker, which makes the miss readable after the fact, and a check re-run over the re-uploaded file, which re-reads the bytes and re-composes the pairs.

~~**What would close it:** the **Blender Research route, asked blind** — no rental and no machine, the register's own answering route for this fact.~~ **That is the route it was closed by, on 2026-08-30.**

**Why this entry exists.** The claim was carried in the body with no source entry anywhere in this file — not in section 3's Blender pointer table, not here, and not in section 3's closing list of what it does not cover — which is exactly 4.1's shape: a claim the document leans on with no source, where the honest thing is to say where it was looked for.

---

## 5. Two places the label and the evidence did not line up — **both repaired 2026-08-23**

**Recorded here as history, because both readings were stale rather than wrong about the evidence, and the repair was to the ground rather than to the conclusion.**

### 5.1 Q2's *"depending on the implementation"* — repaired

[Q2](../Dependency%20Upload%20&%20Remap.md#2-telling-two-files-apart-by-what-is-inside-them--row-168) used to give the desktop-side digest cost as *"about 1.26 to 2.22 seconds on the Creator's Mac depending on the implementation"*. **Fact 15's two Mac figures are md5 at 1.26 s and sha256 at 2.22 s** — two different digest functions, not two implementations of one — and Q1 fixes the function to the single one `Server protocol and transfers` S23 fixes per protocol version, so **one of those figures is the cost and the other is not, and there is no range**. Fact 15 measures the five-fold implementation gap **on the machine** (`openssl dgst -sha256` 0.46 s against `sha256sum`'s 2.24 s, Q3) and asserts a gap in the same direction on the Mac without recording figures for it, so **no observation bounds what a particular desktop implementation costs**.

**Q2 now says exactly that.** The conclusion survived the repair: no promise anywhere names a rate, which is Q4, so nothing else moves.

### 5.2 A5's attribution of the 765-byte name — repaired

[A5](../Dependency%20Upload%20&%20Remap.md#52-the-mirror-rule--row-169) used to credit the 765-byte macOS name to *"Fact 16's batch"*. The register records the figure under **Fact 24** (2.19), and `Download` §15 item 8 and `Desktop app and delivery` C5 both cite it as Fact 24's. Both facts were answered on 2026-08-18 by the same reading batch in the same document, so **the batch was right and the number was not**. A5 now names Fact 24 and the document it sits in. The figure itself is unchanged, so A5's byte arithmetic and its *could not be sent* outcome stand as written.

## 6. The letter-case question, settled — **added 2026-09-01, out of the repaired Blender research**

**F15 read *unresolved, checked and standing* until this date, and it is answered.** ***Reading a file 10*** now carries the run and, more to the point, the control that makes it a test: with only an upper-case name on disk and a lower-case one recorded, **neither machine recovered it**, while the control run with the matching name recovered it on both. The sentence that decides between the two readings is that question's own: *"The Creator's Mac gave that answer even though its own file system does not treat capital letters in a file name as significant"* — asked directly for a file by a name in capitals when the name on disk is in small letters, that Mac finds it and the Linux machine does not. **So it is Blender's own rule and not the file system's.**

**Label: observed in a test**, on the Creator's Mac and on a rented Linux machine, in all four versions. **Windows is untried and is the only part left**, and it is the interesting part — a file system that preserves capital letters without treating them as significant. **It will not be tried**: the Creator ruled on 2026-08-31 that there will be no further research and no further rented machine, so the Windows half is permanently unresolved.

**Nothing in the answer document moves.** F15 existed to record that the repair does not rescue a wrong spelling **either way**, which is the spec's own closing of the question; the answer arriving on the side the document already behaved as though it were on changes no behaviour and no other clause. **What moved is the label**, and that matters on its own — a reader meeting *unresolved* at F15 would go looking for a question that is answered.

---

## 7. The search's written-back form — a withdrawn claim, **added 2026-09-02**

**[M21](../Dependency%20Upload%20&%20Remap.md#66-the-reach-across-kinds--row-176) carried, until this date, an accusation against Blender's manuals, and it is withdrawn.** The fourth of M21's four reasons read *"it writes back a relative location although every version's manual says recoveries are made absolute — a documented promise the running program does not keep, in all four versions on both machines"*, and the section 3 row above repeated it. **Section 5's shape applies: the reading was stale rather than wrong about the evidence, and the repair is to the ground rather than to the conclusion.**

**Where the withdrawal is recorded.** `Preparation/Specs/Sources/Dependency Upload & Remap.sources.md`, dated **2026-09-01**, read **2026-09-02**, under the heading *"~~One disagreement between two comparisons.~~ THERE IS NO DISAGREEMENT — corrected 2026-09-01, after the research was repaired"*: *"On the form Blender's own search writes a repaired location back in, **Dependencies 8** and **Dependencies 12** report a full location into the search folder while **Reading a file 6** and **Reading a file 10** report the relative form — and the two sides were repairing different kinds of location. Dependencies' five paths were all written out in full and all five came back in full; Reading a file's runs repaired only paths written in Blender's own shorthand and got the shorthand back. **Each got back what it handed in.** **Neither ever repaired the other kind, so whether the search keeps the form it found has never been asked** — which is a genuinely open question and a much smaller one than a contradiction between two records."* `Preparation/Specs/Dependency Upload & Remap.md` carries the same correction in the spec's own words, and adds that *"the manual's promise is neither kept nor broken on this evidence"*. **Label: observed, and narrower than it reads** — the spec's own label.

**What it does not cover, and this is the whole of what is left.** **Whether the search keeps the form it found was never asked**, on any machine, in any version. It is **permanently unresolved** — the Creator ruled on 2026-08-31 that there will be no further Blender research and no further rented machine — and it is carried at [§12 item 25](../Dependency%20Upload%20&%20Remap.md#12-unresolved) *(filed as item 23 on 2026-09-02 and renumbered the same day, that number being already in use)*.

**Nothing in the answer document moves except the sentence itself.** M21's conclusion is that Blender's own search and repair stand behind nothing here, and that conclusion never rested on the manual being broken — it rests on the other three reasons and on the search's written-back form being unpredictable, which is what the evidence supports either way. **The standing instruction is unchanged**: Smoozy writes the location it means and reads it back rather than assuming its form.

**Why it is recorded rather than quietly deleted.** The struck claim says a first-party manual is wrong, which is the strongest thing this project ever says about a publication; a reader meeting it and then finding the spec's correction has no way to tell which is later without this entry.

## 8. A baked physics cache still reads as baked with its folder absent — **added 2026-10-02**

**[G6](../Dependency%20Upload%20&%20Remap.md#32-what-the-reading-enumerates--rows-159-and-160)'s default-cache-folder exception rests on it**: the check reads the machine's copy of the `.blend`, and a file uploaded alone has its cache folder only on the user's computer. **Observed at the desk, 2026-10-02, in 4.5.12 and 5.2.2**, by the check's own script run in background Blender: `made-kind-4-cloth-disk-cache.blend` and `cloth-long-name.blend`, each copied alone into an empty folder, still read their cloth's point cache as baked, and the check listed `//blendcache_<file name>/` for each; `cloth-unbaked-stale.blend`, beside a folder of 48 old files, listed none. Recorded in `Audits/Build/Step 10c/After 0.1.8, the checks' leftovers — what landed.md`, item 1.

## 9. A geometry-nodes bake folder's template, from 5.3 — **added 2026-11-02**

**[G9a](../Dependency%20Upload%20&%20Remap.md#32-what-the-reading-enumerates--rows-159-and-160) rests on it.** *Dependencies 5* and *Node setups 1* carry the release note, the API's mark and one bake each. **Observed at the desk, 2026-11-02**, each Blender headless on the Creator's Mac, by the operator and the shape *Findings/5.3/Dependencies.md* records: 5.2.3 `e13b728c6880`, 5.3.0 Beta `5d8151335b6b`, and 5.4.0 Alpha `3f4216e1bad1` for `{blend_name}`, `{scene_name}` and the linked case alone. A Bake node made by a script starts with no items, as *Findings/5.3/Node setups.md* says, so each was given one Geometry item.

| The modifier's bake folder | 5.3.0 Beta | 5.2.3 |
| --- | --- | --- |
| `//bakes/{blend_name}/` in `t.blend` | baked into `bakes/t/` | baked into a folder called `bakes/{blend_name}/` |
| `//bakes/{{x}}/` | `bakes/{x}/` | `bakes/{{x}}/` |
| `//bakes/{scene_name}/`, `//bakes/{fps}/`, `//bakes/{hero}/`, `//bakes/{blend_name/`, `//bakes/{blend_name}_{scene_name}/` | *Cannot determine bake location on disk. Falling back to packed bake.*, no folder, the operator still `{'FINISHED'}`, and the setting and its Disk target unchanged | a folder of those very bytes |

**The linked case, 5.3.0 Beta and 5.4.0 Alpha** (`a_linked_files_blend_name_is_the_file_blender_opened`). `L.blend` baked under `//bakes/{blend_name}/` and linked into `T.blend`, the Bake node cut off from its source so the object has geometry only where the bake is read: `L.blend` opened alone has its 8 vertices with the bake in `bakes/L/`; linked into `T.blend` it has none with the bake there, and 8 with the bake in `<L's folder>/bakes/T/`.

**Through the render worker's driver, on 5.2.3, 5.3.0 Beta and 5.4.0 Alpha** (`a_bake_folders_template_lands_where_the_render_looks`; 64 pixels, Cycles on the processor; the same cut-off Bake node). Bake folder absent: no cube. The folder uploaded beside the `.blend`: the cube. Sent, and copied under the job's folder by the instruction the desktop composes: the cube, with `geometry-nodes-bake/<id>/…` in the job's folder and no brace anywhere under it. On 5.3 and 5.4 a bake that fell back to packing: the cube, with no folder sent. **So the load-time route `Job files and outputs` D1a observed in 4.5.12 and 5.2.2 holds in 5.3.0 Beta and 5.4.0 Alpha.**

**From the check to the mirror, with each Blender's own reading** (`a_template_in_a_bake_folder_is_found_and_sent_at_the_desk`). The folder is found at `bakes/t` on 5.3 and 5.4, and at `bakes/{blend_name}` on 5.2.3; it lands whole at that folder's mirror; a copy is owed keyed by `//bakes/{blend_name}/`. An Import SPZ node's `//splat.spz` reads as found on 5.3 and 5.4 through the check's own walk.

**Not covered:** Linux, a graphics card, a linked file's bake through the driver (where the load-time handler sets a linked object's setting), a project's own variables.

## 10. A library's relative location as Blender reads it — **added 2026-11-02**

G6d. Label: **observed in a test**, on the Creator's Mac, with no window, by the builder holding `Audits/Build/Blender 5.3/Brief — what the real files found.md`. `cowboi_storytools.blend` (CC-BY) in a deep scratch folder: `Library.filepath` reads `//../../../data/cowboi/…/cowboi.blend` in 5.3.0 Beta. The same bytes at `/tmp/smzq/c.blend`: `//../../data/cowboi/…/cowboi.blend`, while the file's picture keeps `//../../../../gp3/…` as written. On the rented machine, at `/root/scenes`, the page read `//../../data/…` (`Audits/Build/Blender 5.3/The live pass — RunPod — what happened.md`, new fault 2). The desk test `a_librarys_location_is_read_as_the_file_writes_it` holds the same in the Blenders it names. `zstandard` imports in Blender's own Python in 4.5.12, 5.0.1, 5.1.2, 5.2.3, the 5.3.0 Beta and the 5.4.0 Alpha on this Mac; **a Linux build was not asked**, and where it is missing a compressed file falls back to what Blender read.

## 11. A file's reading, as the libraries below it land (G17b)

Observed at the desk, 2026-11-03, on the Mac, by `a_files_reading_grows_as_its_libraries_land_at_the_desk` in `App/Dependency Upload & Remap — machine/tests/packed_linked_data_at_the_desk.rs`, in 4.5.12, 5.0.1, 5.1.2, 5.2, 5.3.0 Beta and 5.4.0 Alpha. A file links a library, which links a second, which links a third holding a picture. All three taken away, then brought back one at a time, the top one first, the check's own script reading the top file each time: each library that is there brings the next one's Library reference into the top file's reading, an absent one is named and nothing below it is, and each reference's location, measured from the top file's own folder, is where that library really sits. With all three there, the third's picture is in the top file's reading too.

## 12. A text block a file's own start script makes (M14a)

Observed at the desk, 2026-11-03, on the Mac, on Blender 5.3.0 Beta `5d8151335b6b`, by `a_file_named_by_what_it_links_is_fixed_at_the_desk` and `a_real_files_pairs_are_tried_at_the_desk` in `App/Dependency Upload & Remap — machine/tests/packed_linked_data_at_the_desk.rs`. A file that links a library that links the file: read through the check's own script, the second file reaches the first and the first does not list itself; both are fixed through M14's own script, every pair applied, exit 0. The second Sesterce pass's top file, with the 61 pairs that pass's record holds for it, opened from the pack on the desk: 60 applied and one not, a `Text` pair whose location ends in the name of the folder above the file, `data_blocks` 0. The same file opened twice in that Blender with `-b --factory-startup`: 9 text blocks and none named for its folder; with `--enable-autoexec` as well, 10, the tenth being that block. So the block is made at load by the file's own script, which the reading runs and the fixing does not.
