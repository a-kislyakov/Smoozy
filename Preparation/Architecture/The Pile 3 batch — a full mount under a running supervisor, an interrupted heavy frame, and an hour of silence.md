# The Pile 3 batch — a full mount under a running supervisor, an interrupted heavy frame, and an hour of silence

What one rented machine did when the three Pile 3 questions that wanted a rental were put to it, together with Blender's crash file, which rode the same rental free.

**This is a frozen snapshot of one machine on one day.** Everything below was observed on **2026-08-25** on a single container rented from Vast.ai and destroyed the same day. It is an **input to** the Architecture stage, not part of it: nothing here is a decision, a recommendation or a plan. A different rented machine may differ in every particular. The raw record, with every command and every line the machine printed, is `Audits/Architecture/The Pile 3 batch of 2026-08-25/Findings — the rented container.md`.

**Every claim here is observed.** Where a claim is bounded by what was actually exercised, the bound is stated rather than rounded off.

---

## 1. The machine

| | |
|---|---|
| Provider and instance | Vast.ai, instance **48661846**, from offer 45604096 |
| Rented / destroyed | 2026-08-25, 14:40 to 16:13 UTC — 93 minutes at **$0.0361/hour**, about six cents |
| Operating system | Ubuntu 22.04.4 LTS, kernel 6.8.0-47-generic, x86_64 |
| Card | NVIDIA RTX A2000 12GB, driver 550.107.02 |
| Processor | AMD Ryzen 5 5600X, 6 cores |
| Root filesystem | `overlay`, 40 GB |
| Blender | 4.5.3 LTS |
| Machine kind | **Container.** PID 1 is the provider's own launch script, `/bin/sh -c while [ ! -e /.launch ]; do sleep 1; done; bash /.launch` |

**The systemd binary is present — `systemd 249` — and is not init.** So **every finding here is the container kind**, and none of it is evidence about a systemd machine. That bound matters most in §2.

**The container permits no mounting of any kind.** A loopback mount and a tmpfs mount were both attempted and both refused (`Operation not permitted`, `permission denied`). On this machine kind Smoozy's state mount **is** the container's own disk, so §2 was run by filling that disk — the real condition rather than a proxy for it.

---

## 2. A running supervisor whose state mount fills under it — **U1**

**What was run.** A stand-in supervisor with the shape Smoozy's has — a long-lived process holding a command endpoint on the state mount, a records store written temp-file-then-`fsync`-then-rename, an append-only journal, and a one-second heartbeat counter proving the process is still running its own loop. It was started, given a baseline, and **left running throughout**. The disk was then filled to **genuine zero**, not near-zero: `fallocate` to within 2 MiB, then 4 KiB writes until `ENOSPC`, then single-byte writes until `ENOSPC`.

**It is not Smoozy's code and it is not from `App/`.**

### 2.1 The supervisor survives

**Observed.** Same process throughout — pid unchanged, run identifier unchanged, uptime continuous, heartbeat counting from 135 beats to 227 across the whole full-disk period. It never exited, never restarted, and nothing restarted it.

**This is the opposite end of the case Batch E recorded.** That batch found a supervisor **starting** on a full mount cannot bind its endpoint at all, because creating the endpoint needs a directory entry. **Starting and continuing are two different answers**, and only the first was known.

### 2.2 Reads stay truthful and refused writes change nothing

**Observed.** The line, the journal size and the health reading all answered correctly while the mount was at zero. Every write needing a new block or a new directory entry was refused with `errno 28`, **naming the exact file**, and left the store exactly as it was — the held job's attempt count did not advance, and the record that could not be created was not created.

### 2.3 **An append to an already-open file goes on succeeding on a full mount, and that is new**

**Observed.** With the mount at genuine zero, the journal accepted **seventy-six further events**. It grew from 110 bytes to **exactly 4096 — one filesystem block — and only then refused.**

The reason is ordinary. An append that fits inside the slack of a file's already-allocated last block needs no new block, so it needs no free space. A record write needs a new file and a new directory entry, so it does.

**So *the mount is full* and *the journal refuses* are not the same moment.** There is a window of up to one filesystem block in which the record store already refuses and the journal still accepts. On this machine that window was 4 KiB, about seventy-six events.

**Batch E observed the opposite on its own box** — its journal did not grow at all. Both are true: that journal happened to sit on a block boundary and this one did not. **Which of the two a real machine meets is decided by where the journal's last write happened to land, and nothing controls that.**

### 2.4 The writability test lies, on a second machine

**Observed.** `os.access(W_OK)` answered **True** on a mount at genuine zero, confirming Batch E's finding on a different box. A writability check is not a substitute for attempting the write.

### 2.5 Recovery is immediate and needs nothing

**Observed.** The moment space returned, the same process — no restart, no new run identifier — accepted the write that had just been refused, advanced the attempt count, appended to the journal and listed both records. **On the container kind, `Rented-server runtime` S14's hold-pending-and-re-attempt shape works exactly as written.**

### 2.6 What this does not answer

**The systemd kind, entirely.** `Rented-server runtime` §16 item 12's recorded break — the start limiter giving up and the unit not recovering when space is freed — is about a **systemd** machine and about a supervisor **starting**. Nothing here touches either half of it.

---

## 3. A heavy frame's latency to honour an interrupt — **U8**, and **Fact 106**

**What was run.** A deliberately expensive scene: Cycles on the card, 16384 samples, adaptive sampling off, denoising off, 24 bounces, 1920×1080 at 100%, thirty-six subdivision-5 icospheres with transmissive materials. Blender's own estimate for the one frame was **ten and a half hours**, at a sample batch of sixteen every **37.3 seconds**. Four interrupts were sent at different points, each timed with nanosecond stamps immediately before the signal and immediately after the process left.

### 3.1 The measurements

| the frame had been rendering | latency to exit |
|---|---|
| **12 min 27 s** | **2.666 s** |
| 1 min 55 s | 13.437 s |
| 2 min 32 s | 29.946 s |
| 2 min 33 s | 8.142 s |

Blender's own words on the interrupt: `Sent an internal break event. Press ^C again to kill Blender`, then the batch in flight completing, then `Cancelled`, then `Blender quit`. Exit code **1**.

### 3.2 The mechanism, and it is the whole answer

**Observed.** **The latency is the time left in the current sample batch and nothing else.** Blender announces the break at once, finishes the batch already in flight, then cancels. All four measurements fall inside one batch of 37.3 seconds and are spread across it the way a signal arriving at an arbitrary moment would be.

**How deep the frame is does not matter.** The deepest interrupt of the four produced the **fastest** exit, because the signal landed near the end of a batch. **What a small fast scene changes is the batch length, not the mechanism** — which is the direct answer to the concern that every stop measurement on record had been taken on a small scene.

### 3.3 What this does and does not license

**~~`Rendering` K9's 60-second deadline is confirmed at the width exercised.~~** *(Annotated 2026-08-31 rather than rewritten: this is an observation record and what it observed is unchanged. **The deadline is now 120 seconds.** A second body of measurement — *Frames 15* — records longest stop waits of sixty-five to seventy-three seconds in all four versions on a much smaller scene, which is above the sixty this line confirmed, and the two bodies are not reconciled. `Rendering` K9 and its §9 item 7 carry both.)* The worst case on this scene is one batch — 37.3 seconds — leaving about 23 seconds of headroom, and the worst of four real measurements was 29.9 seconds.

**The headroom is a property of the scene, not of Blender**, and that is the part a reader must not round off. Nothing observed here says a sample batch cannot take longer than 60 seconds; a scene with a higher per-sample cost would push it there, and then K10's premature sentence becomes the ordinary case on a stop that is working. **The assumption stands at this width and no wider.**

### 3.4 An interrupted frame leaves nothing

**Observed, on all four runs.** The output folder was never created — no part-written image, no empty file, nothing. Consistent with what `Queue & History` already records for an interrupt, now seen on a heavy frame too.

---

## 4. An hour of silence on a provider's SSH port mapping — **U2**

**What was run.** A marker was written, the endpoint recorded, and **the machine was touched by nothing at all for 3623 seconds** — no connection, no keep-alive, no probe. The rental stayed running throughout.

### 4.1 It survived

**Observed.** The same host and the same port reached the **same machine** after an hour of complete silence, on the first packet. Proved three ways: the marker on disk was the one written before the silence; the machine's boot identifier was unchanged, so it had not rebooted; and §2's supervisor was still the same pid with an unbroken heartbeat of 5393 beats against 5401 seconds of uptime.

### 4.2 What it does not establish

**It is one hour, on one provider, on one rental, and it sets no upper bound.** What it shows is that an hour of silence is **not enough** to lose the mapping. It says nothing about a longer silence, nothing about a rental that stops and restarts, and nothing about `Servers` §16 item 5's separate open half — whether a seller later hands the same address to a **different** machine.

---

## 5. Blender's crash file and the machine's environment — **Fact 131**

**What was run.** Two unmistakable marker values were placed in the environment — one shaped like a licence key, one shaped like a private path — and Blender 4.5.3 was crashed with a null dereference from Python. Twice: once with no scene, once with a real scene loaded.

### 5.1 It carries no environment values

**Observed.** Neither marker appears anywhere in the file, and the file contains no assignment-shaped text at all. The file is **1775 bytes** and holds **three sections and nothing else**:

- a version header naming the Blender version, commit date and hash;
- `# backtrace` — module paths and hex addresses, some with symbol names, all of them either the Blender installation's own or a system library's;
- `# Python backtrace` — source file paths and line numbers.

### 5.2 The contents never name the user's scene and the file name always does

**Observed.** With `heavy.blend` open, Blender wrote **`/tmp/heavy.crash.txt`**; with no scene it wrote `/tmp/blender.crash.txt`. The contents were the same size and the same three sections both times, and a search for the scene's name inside the file found nothing.

**So the leak, such as it is, is the file's name.** What the backtrace sections carry is paths belonging to the Blender installation, to system libraries, and — where a script is running — to that script.

---

## 6. What was spent, and what was left behind

**About six cents.** The account was verified empty before the rental and confirmed empty immediately after the destroy. **Nothing pre-existing in the account was touched**, including the throwaway key `smoozy-arch-batch` that the Architecture board records as wanting removal by hand.

---

*2026-08-25 — Claude Opus 5 (Claude Code) — the architecture runner, the Pile 3 batch.*
