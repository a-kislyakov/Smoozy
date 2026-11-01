# Two more boxes — the volume, the full disk, and a day left running

What two further rentals answered on 2026-08-16, after [One box](One%20box%20—%20the%20Telegram%20capture%20and%20the%20port%20that%20cannot%20be%20chosen.md). An **input** to the stage, like the others: nothing here is a decision or a plan.

**Frozen snapshots of two boxes.** The first was destroyed the same afternoon. The second was left running on purpose to reach a day; **its record was fetched on 2026-08-17 at 24.02 hours and its section is written**, so this document is complete as evidence. The instance itself was still alive at that point and its section says why.

Every claim is **observed**, with what was run and what came back. Where something was not established, it says so. Rule 1's plain-English rule is suspended here, per the stage's exception.

**Why they existed.** Seven rentals in a row had a single disk, so Fact 6's *does a rental offer a choice of disk at all* and Fact 28's *more than one disk* clause had never been posed. The runner ruled on 2026-08-16 that a volume may be attached to a rental, which is the only way to pose them on this provider. And Facts 62 and 65 both carry an *over hours and days* half that a two-hour sitting cannot reach, so a cheap box was left running to reach it.

---

## The volume box

**Vast.ai instance `47868527`, offer `45169474`, machine `52447`, at `$0.0489/hr` plus a 20 GB volume at `$0.00028/GB/hr`**, alive 13:28–13:34 UTC and destroyed. Listed as South Korea; **verified** host, reliability 99.68%. Image `ubuntu:22.04`.

Ubuntu 22.04.5 LTS, kernel `5.15.0-186-generic`, **PID 1 is `/bin/sh -c while [ ! -e /.launch ]; do sleep …`** — a container again. Intel Xeon E5-2630 v4 presented as **40 processors**, 65,731,712 kB memory, one **GTX 1660 SUPER, 6144 MiB, driver 580.159.03**. Root is a **16 GiB `overlay`**.

**A failed rental came first, and it is worth recording.** An earlier attempt on machine `140109` — a verified host at 99.14% — sat in `loading` for sixteen minutes on a bare `ubuntu:22.04`, its last log line an apt fetch, and never opened SSH. It was destroyed for `$0.0013`. **That is the third rental this project has lost to a container that never starts**, and it is why a rental is checked rather than assumed.

## Fact 6 — a rental does offer a choice of disk, and the choice is not a second disk

**A volume can be asked for at rental time and arrives mounted.** It was created against the same machine's volume offer, named in the instance request with a mount path, and the box came up with it in place at `/vol`.

**It is a separate filesystem, and a program can tell.** `/`, `/root`, `/tmp` and `/var` all report device number **53**; `/vol` reports **2052**; `/dev/shm` reports **122**. So the three placements the specs care about — a job's folder, Smoozy's asset area, a Blender install — can be put on either, and which one they are on is readable.

**But it is not a second disk.** `df` gives it as `/dev/sda4`, type **xfs**, size **20 GiB**. `lsblk` gives `sda` as a single **1.8 T** disk, and `sda4` as a **1.8 T** partition of it. **So the 20 GiB is a quota imposed on a slice of the same physical disk, not a device of its own** — the container sees separate free space, and shares whatever the disk itself can do. The disk-choice half of Fact 6 is answered; *the machine has more than one physical disk* was **not** posed and this box cannot answer it.

**It arrives owned by root, and an ordinary login cannot use it.** As delivered, `/vol` is `root:root 755`, and the non-root login made on the box was refused: `touch: cannot touch '/vol/.probe2': Permission denied`. After `mkdir /vol/smoozy` and `chown smoozy:smoozy`, the same login wrote to it and read its free space without trouble. **So a setup step has to hand the volume over; a rental with a volume is not usable by Smoozy's login as it stands.**

**Everything on both disks survived a stop and a start.** Markers were written to `/root` and to three places under `/vol`, the instance was stopped from the provider's console and started again. It reported `exited` within twenty seconds of the stop and was **running again about forty seconds after the start**, with **both markers byte-identical, the volume mounted at the same place, and the folders intact**. Fact 6's survival clause is answered for a stop-and-start; **the machine's own unattended housekeeping over days was not posed**, and neither was the recycle the previous batch found on a container.

## Fact 28 — free space reads true, and a naive reading is mostly noise

**Free space can be read over the connection, by root and by an ordinary login**, both through `df` and through the filesystem call underneath it, before and after the disk filled.

**What a reading has to exclude to be true, in numbers.** A plain `df` on this container returned **51 filesystems**. Forty-eight are `tmpfs` — `/dev`, `/dev/shm`, and dozens under `/proc` and `/sys`, including **one per CPU thermal-throttle folder**. Of the three that are not:

- `overlay` on `/` — 16 GiB, the machine's own storage.
- `/dev/sda4` on `/vol` — 20 GiB, the volume.
- **`/dev/sda2` on `/usr/bin/nvidia-smi` — 32 GiB, 35% used, 20 GiB free.** That is the **host's** disk, bind-mounted in to deliver the driver files. It is not the rental's storage and nothing may be written to it, but it looks exactly like a disk with 20 GiB free.

**So *the machine's free disk space* is not a reading, it is a choice of which mount to read.** Reading by total size, or by "the biggest", or by summing, all pick up something that is not the machine's.

**A full disk refuses in one way, everywhere.** The volume was filled with `dd`, which wrote exactly `21,474,836,480` bytes at 472 MB/s and then failed. Every further write was refused with **`No space left on device`** — a shell redirect, `cp`, and a Python write with an explicit flush and `fsync` that raised **errno 28**. **None of the refused writes left a file behind.** **Scope added 2026-08-21, because this sentence has been read wider than the run:** the volume was filled first, so **every write tested here opened onto an already-full volume**. The case where a write fills the disk partway through was never posed on this machine. On macOS it was, and it behaves differently — Fact 23: filling during a write left a truncated file on every byte-loop run and an empty file on every copy-tool run. **So no writer may read the absence of a file as evidence that nothing was written.**

**While full, the reading is unambiguous**: `df` reports `20G 20G 0 100%`, and the filesystem call reports `free=0 avail=0`.

**And a folder that refused a write can be seen to be taking files again.** Deleting the large file returned the free space at once — `0% used`, `avail` back to the full count — and a copy into the same folder that had just refused succeeded. **Both signals are available: the reading recovers, and the write succeeds.**

**One small thing observed in passing.** `su - smoozy` on this container reports `-bash: /root/.bash_profile: Permission denied`, so the ordinary login's environment is not cleanly its own. Recorded, not chased.

---

## The box left running

**Vast.ai instance `47868453`, offer `46997840`, machine `141036`, at `$0.0289/hr` with a 12 GB disk at `$0.133/GB/month`**, started 13:27 UTC on 2026-08-16 in **Germany** — a fourth location, and the cheapest reliable box on offer at 99.45%. A full day is about **75 cents** all in.

**A contract cap was asked for and silently refused, and that is a finding in its own right.** The rental was created with `duration` set to a 30-hour cap — **the literal value sent is not recorded, and the field is documented in seconds, so this record cannot say what was asked for**. The instance came up with an `end_date` of **2027-05-10**, the host's own availability. Setting `duration` again on the running instance returned **`{"success": true}`** and changed nothing. **So a Vast rental cannot be told to end itself, and the call that appears to do it reports success and does nothing** — the same shape as the boot-hook the previous batch found on a container.

**The reason, corrected 2026-08-20 against the provider's own documentation, and the conclusion above is unchanged.** `duration` **is not a parameter of the create-instance call at all** — Vast's create-instance reference, `https://docs.vast.ai/api/create-instance`, read 2026-08-20, lists the request body in full and does not carry it; `{"success": true}` is what an ignored unknown field looks like. `duration` exists only as a read-only number of seconds on the instance, per `https://docs.vast.ai/api-reference/instances/show-instance`. And Vast states the end date is not the tenant's to set — its Instances FAQ, `https://docs.vast.ai/documentation/reference/faq/instances`, read 2026-08-20, under *"What does "Lifetime" mean on my instance?"*: *"This rental end date is locked when you rent and cannot be changed by the host. When the rental end date is reached, the rental contract expires and the instance is stopped."* Its CLI guide, `https://docs.vast.ai/cli/commands`, names no duration, no end date and no scheduled termination. **So this is a property of the provider rather than of one call, and no value of any parameter settles it.**

**What was thought to stand in for it does not exist, corrected 2026-08-20.** The original record said a watchdog on the box would kill its own PID 1 at about 16:03 UTC on 2026-08-17, ending the container and the compute charge. **That kill cannot land.** The Linux man-pages project's `pid_namespaces(7)`, `https://man7.org/linux/man-pages/man7/pid_namespaces.7.html`, read 2026-08-20: *"Only signals for which the "init" process has established a signal handler can be sent to the "init" process by other members of the PID namespace."* and *"SIGKILL or SIGSTOP are treated exceptionally: these signals are forcibly delivered when sent from an ancestor PID namespace. Neither of these signals can be caught by the "init" process."* SIGKILL can never be caught, so no handler for it can ever exist, so it can never reach the namespace's init from inside; the forcible route runs from an ancestor namespace only, which is the provider's side. PID 1 on this box was `/bin/sh -c while [ ! -e /.launch ]; do sleep …`, a non-interactive shell in a wait loop — the case with the fewest handlers installed. [The last two boxes](The%20last%20two%20boxes%20—%20the%20hook%20that%20cannot%20be%20read%20back,%20the%20full%20disk,%20and%20the%20unit%20that%20kills%20a%20render.md) already recorded the same rule from the host's side. **The command the watchdog ran was never recorded**, so which no-op it was cannot be said; either way the container did not end by that route, and the outcome was written in the future tense and never checked.

**The billing half was unsourced and stays unsettled.** *Stops the compute charge whatever else happens* had no source. Vast's Instances FAQ covers stopped and destroyed instances — *"Stopped instances: Data persists, storage charges continue"*, *"Destroyed instances: All data is permanently deleted"* — and says nothing about a container whose first process has exited while the instance is neither. Settling it needs a page Vast has not published or one live rental left in that state and a look at the bill.

**So the runaway-rental hazard has no mitigation on the box at all.** The rental has no scheduled end and the watchdog was the whole of what stood in for one. **The instance still has to be destroyed by hand**, and that is now the only thing that ends either charge.

**What it is doing.** Every five minutes it calls `getMe` and sends a chat action — a real call down the sending path that delivers no message and rings nobody. Every six hours it sends one 1,465,514-byte 1920×1080 picture, times it, and deletes it a second later. One line per event, with hours-since-start on each.

**What it is for.** Fact 62's *goes on reaching it* over a day, and Fact 65's *hours rather than minutes*. Both have been open since the module's facts were first written.

**Its first readings, already worth having.** From Germany, `getMe` took **0.072 s** and the full-HD picture went in **0.578 s**. The same picture from the Oregon box took **1.4 s**, and `getMe` there took **0.46 s**. **So the picture-per-frame cost is dominated by where the machine is, not by Telegram** — a fact the Oregon numbers alone would have hidden.

### The day it reached — Facts 62 and 65, the *over a day* halves

**Fetched 2026-08-17 at 13:30 UTC, at `hours_in` 24.022.** The record is **589 lines** covering **24.02 hours** unbroken, from `2026-08-16T13:27:50Z` to `2026-08-17T13:29:05Z`.

**Nothing failed, at all.** Every one of the **289 `getMe`** calls and **289 `chatAction`** calls returned **HTTP 200** with `ok: true`. So did all **five** full-HD picture sends and all five deletions. There is not a single non-200, not a single `ok: false`, and no error description anywhere in the file. **A negative result would have been any refusal, any rate-limit reply, or a gap where the recorder tried and got nothing** — none is present.

**And nothing drifted.** Median round trip per four-hour block, across all six blocks and the twenty-fourth hour:

| hours | `getMe` median | `chatAction` median |
|---|---|---|
| 0–4 | 0.065 | 0.080 |
| 4–8 | 0.065 | 0.079 |
| 8–12 | 0.065 | 0.074 |
| 12–16 | 0.063 | 0.075 |
| 16–20 | 0.064 | 0.076 |
| 20–24 | 0.062 | 0.079 |
| past 24 | 0.066 | 0.140 |

`getMe` over the whole day: min **0.058**, median **0.064**, max **1.248**. `chatAction`: min **0.067**, median **0.078**, max **1.423**. **Both maxima fall in the 16–20 hour block and are isolated** — that block's medians are 0.064 and 0.076, unchanged from every other. So the service produced two slow answers in a day and no trend.

**The pictures held their pace too.** Five sends of the same 1,465,514-byte 1920×1080 picture, at 0, 6, 12, 18 and 24 hours: **0.578, 0.506, 0.485, 0.765 and 0.588 seconds**. Deletion took 0.073 to 0.213.

**No gap between consecutive events exceeded eight minutes** against a five-minute cadence, so the recorder never stalled and never lost the service.

**What this answers, precisely.**

**Fact 62 — answered.** A rented machine in Germany went on reaching Telegram continuously for **over a day**, with 578 calls answered and none refused. The *goes on reaching it* clause that had been open since the module's facts were written is closed.

**Fact 65 — answered for a steady low rate, and that is the honest limit.** The fact asks whether the pace holds over hours and days rather than over a minute, and over 24 hours it holds without drift. But **this box sent lightly**: two small calls every five minutes and one picture every six hours. **Sustained heavy sending over a day was not posed** — the burst behaviour is the previous batch's answer (sixty texts in 35.2 seconds, thirty simultaneous all accepted and serialised, ten pictures at 1.36–1.82 seconds each), and nothing here extends that to a day. So *a day at a light rate* is observed; *a day at a rendering job's rate* is not.

**One reading confirms the earlier finding rather than adding to it.** From Germany the picture cost **0.485–0.765 s** all day, against **1.4 s** for the same picture from the Oregon box. **The picture-per-frame cost is set by where the machine is, not by Telegram** — now measured over a day rather than over a sitting.

**The box was not destroyed.** The agent that fetched this record could not reach the provider's API — the route to the key was refused by the tooling it was working through — so the instance is still there. It was written here that its watchdog kills its own PID 1 at about **16:03 UTC on 2026-08-17**, ending the container and the compute charge; **that is wrong and is corrected above** — the kill cannot land inside a PID namespace, so nothing on the box ended anything. **The instance needs destroying by hand**, and that is the only act that ends either charge. **That is the only thing this document still owes**, and it is not a finding.
