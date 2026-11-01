# The last two boxes — the hook that cannot be read back, the full disk, and the unit that kills a render

What two rentals answered on 2026-08-18, run against a throwaway built from [Rented-server runtime](Rented-server%20runtime.md). An **input** to the stage, like [Batch A](Batch%20A%20—%20the%20shape%20throwaway.md), [Six machines](Six%20machines%20—%20the%20container,%20the%20webhook%20and%20the%20ready%20action.md), [Two more boxes](Two%20more%20boxes%20—%20the%20volume,%20the%20full%20disk,%20and%20a%20day%20left%20running.md) and [One box with Blender on it](One%20box%20with%20Blender%20on%20it%20—%20the%20reading,%20the%20listing,%20and%20the%20names%20that%20do%20not%20come%20back.md): nothing here is a decision or a plan.

**This is Batch E, and it is the stage's last machine work.** Both boxes are destroyed and the account was verified empty afterwards — no instances, no volumes. Whole-batch machine spend about **twenty cents**.

Every claim is **observed**, with what was run and what came back. Where something was not established, it says so. Rule 1's plain-English rule is suspended here, per the stage's exception.

**Read every figure as *this is what that box did on that day*, never as *this is what a rented machine does*.** Two boxes, one provider, one afternoon.

**A third strand ran beside the boxes and is [Section 12](#12-the-container-boot-hook-across-providers-read-rather-than-run) below: a reading of each container provider's own documentation.** Nothing in it was run, and it is dated 2026-08-18 like the rest.

---

## The two boxes

**The virtual machine.** Vast.ai virtual-machine rental, contract `48022355`, machine `55691`, **Quadro P4000**, Quebec, about `$0.069/hr`, **19 processors**, 128 GB on **ext4**, Ubuntu 22.04.5 LTS, kernel `6.8.0-59-generic`. Reached at `142.170.96.180` port 15840, **direct address only** — the provider's proxy route was not re-tested and nothing here says whether it works. `systemd-detect-virt` → `kvm`; `--container` → `none`, exit 1; no container marker file; first process `systemd`.

**The container.** Vast.ai container `48021183`, **GTX 1080 Ti**, Spain, about `$0.034/hr`, from a bare `ubuntu:22.04` image, Ubuntu 22.04.5. `systemd-detect-virt` → `docker`, exit 0; container marker file present; first process `sh`; root filesystem **overlayfs**.

**Both carried two non-root logins and a Blender.** The virtual machine had Blender **4.5.9 LTS** (`8bf95cbd38d1`) and **5.0.1** (`a3db93c5b259`); the container had **4.5.3 LTS** (`67807e1800cc`). Every render on both boxes was pinned to the **processor**, so no result below depends on a card.

**What stood up on them first**, and it went nowhere near `App/`: the placed package with its pointer and boot registration, the supervisor with its run identifier and endpoint, the durable stores, a render worker with stream capture, and a stub setup worker. It is a throwaway and it is gone with the boxes.

**Three process facts from the sitting, all of which cost time.** A first virtual machine (`48021177`, Hong Kong) was destroyed after twenty minutes still pulling its image and never reaching `running` — **the third rental this project has lost that way**. The replacement's **proxy route refused every connection while its direct address worked perfectly**, which is the mirror image of the six-machine batch's finding. And an instance API key was printed unredacted by the rental helper; it dies with the instance and both instances are gone.

**Three changes were made to the container before anything could be posed**, and each is a finding of its own: its `/etc/environment` ships `HOME=/root`, which `pam_env` applies at every login, so a fresh non-root login gets `HOME=/root` and cannot read its own start-up file — the line was removed and the original kept; `/root` was made traversable, without which nothing in [Section 2](#2-the-container-boot-hook-run-as-observed--and-a-placement-that-can-never-be-confirmed) could be posed at all; and the recurring-entry daemon was installed, which the image does not ship, so that the claim about it could be posed honestly rather than dodged.

---

## 1. Fact 1 at the promise level — the supervisor returns from a reboot and from a crash, and does not return from a full disk

### 1.1 A clean reboot, both machine kinds — answered yes

Posed with a job running, a job held, and the role worker enabled.

**On the virtual machine.** `sudo reboot`, back in about 16 seconds. With nobody logged in, the process table showed the user-level service manager, its authentication helper and the supervisor. Then: a **fresh run identifier**; the held job still `held` with `at_admission_point: false` and its hold **byte-identical**, `release_outcome: null` and its placed moment unchanged; the running job at `attempt_count: 2` with `"disposition": "re-run once"` journalled against the new run identifier, and attempt 1's capture outcomes recorded as `collection incomplete` / *"the worker never reported an end of stream"* / `written_by: reconciliation`; and the role worker restarted, journalling its start and reading forward from its durable cursor.

**On the container**, the same shape by a completely different route, confirmed over **two** reboots: fresh run identifier, phase still `verified`, hold untouched, the running job at attempt 2 with `re-run once`, and the role worker back and reading forward from cursor 80 to 86. **Reconciliation did not start the held job**, and every disposition was journalled.

**The two silent no-ops were re-confirmed across a real restart on the container**, and one of them is worse than the earlier evidence said. `systemctl --user enable` exits 0, creates the symlink, and `is-enabled` afterwards **answers `enabled`** — while the bus it would need is absent, and the probe file it should have produced never appeared after two restarts. **It does not merely report success at the moment it is run; it answers a later reader too.** The recurring boot entry installs, writes a real spool file, and nothing starts a daemon: its probe never appeared either.

**The asymmetry both branches turn on was seen on one machine.** On the container the systemd read-back gives `System has not been booted with systemd as init system (PID 1). Can't operate.` — a machine-set value refusing loudly — while the container read-back reads Smoozy's own bytes.

### 1.2 Crash return — posed for the first time, both kinds, answered yes

**On the virtual machine, by a real kernel panic.** With a 400-frame Cycles render running and a hold standing, the kernel's crash trigger was armed and fired; the machine reboots immediately on panic rather than hanging. Issued 12:39:07 UTC, reachable again about 16 seconds later. **Proof it was unclean is in the next boot's kernel log** — `EXT4-fs (vda1): INFO: recovery required on readonly filesystem`, `write access will be enabled during recovery`, `recovery complete` — and the previous boot logged no orderly journal stop at all.

After it came back, with nobody connected: fresh run identifier, the held job still `held` with its hold unchanged, the real render at `attempt_count: 2` with `re-run once` journalled, and the role worker restarted and reading forward.

**On the container, a clean restart and a crash turn out to be the same event, and that is provable rather than reasoned.** Its first process catches only two signals, so the runtime's polite stop is **discarded** and the forcible kill after the grace period is what ends everything — nothing inside is ever asked to stop. Downtime was about 9 seconds. The proof is in the record rather than in the argument: attempt 1's collection reads `written_by: reconciliation`, both streams `collection incomplete`, reason *"the worker never reported an end of stream"*. **So on this provider there is no clean container restart to contrast a crash with, and the crash rider is answered by the same two reboots.**

### 1.3 The disk-full form — answered **no**, on both kinds, and it is the batch's heaviest negative

**Every write is refused loudly and correctly, and nothing moves behind it.** On the virtual machine, with the state area's own recorded mount at a true zero — 126G of 126G, `free_space_on_state_mount` reading `0`, and the filesystem carrying **no root reserve** — all seven write commands answered identically with `OSError: [Errno 28] No space left on device`: submit, stop, hold, the role toggle, history removal, a setup run, and a release. Behind them the journal's next sequence was unchanged, the queue unchanged, the holds unchanged, and the held job still `held`. The container reproduced it exactly, naming the temporary file in the refusal, with the journal growing **0 bytes**.

**A writability test is not a write.** On the container, `os.access(W_OK)` answered **`True`** on a filesystem with literally zero bytes free, while a real one-byte write to the same three areas failed with errno 28 every time.

**Every read still answers truthfully with the disk at zero.** On the virtual machine: the health reading answered with free space `0` rather than a stale figure; the line answered 233 jobs; the note answered in full including an ending; the history answered 230 entries — before the refused writes and again after them. The container reproduced it: 14 jobs, a complete note, 10 history entries. **Reads do not write, on either kind.**

**But the supervisor cannot start on a full disk, and this is where the promise fails.**

On the **container** the first act is the endpoint bind, and creating the endpoint object needs a directory entry:

```
File ".../lib/supervisor.py", line 69, in _bind_endpoint
    s.bind(path)
OSError: [Errno 28] No space left on device
```

The keeper restarted it into a permanent crash loop. **It heals by itself the moment space returns**, with a fresh run identifier.

On the **virtual machine** the first act is the fresh run identifier, and it fails there:

```
OSError: [Errno 28] No space left on device
  File ".../lib/supervisor.py", line 126, in _write_run_identifier
smoozy-supervisor.service: Scheduled restart job, restart counter is at 10.
smoozy-supervisor.service: Start request repeated too quickly.
smoozy-supervisor.service: Failed with result 'exit-code'.
```

**Ten restarts inside about two seconds exhaust the service manager's default start limiter and the unit goes to `failed`.** A reboot with the disk still full did not help. **And it does not recover when space is freed:** the ballast was removed and the box watched for three minutes at thirty-second intervals — `is-active=failed`, endpoint answering no, at every one. A plain start command was enough to recover it, but nothing on the machine performs one and a reboot does not either.

**That difference between the kinds is real and it is narrower than this record first stated it, corrected 2026-08-20:** the container's keeper loops forever and heals; the systemd machine's start limiter gives up and waits for something outside it. **The *stays failed across a reboot* clause was never posed.** The one reboot performed here was with the disk **still full**, where the unit fails again for the same reason; the three-minute watch after the space was freed was performed **without** a reboot. Freeing the space and then rebooting is the case the claim was about and it was not run. **Nor is the limiter a property of systemd**: `systemd.unit(5)`, read 2026-08-20 at `https://www.freedesktop.org/software/systemd/man/latest/systemd.unit.html`, gives `StartLimitIntervalSec=` and `StartLimitBurst=` as unit settings and says the interval *"may be set to 0 to disable any kind of rate limiting"*. **And the ten-in-two-seconds figure does not match the documented default** — `systemd-system.conf(5)`, read the same day, gives `DefaultStartLimitIntervalSec=` as `10s` and `DefaultStartLimitBurst=` as `5`. Either the unit carried its own values or ten is a count of log lines rather than of starts; **the raw record wants re-reading before that number is repeated anywhere.**

**Which of the four named negatives fired: two of them.**

- *A stop reconciliation will not honour* — **no.** The stop write was refused loudly and no desired state was recorded, so there is no stop to dishonour and the caller was told.
- *An attempt count that does not advance* — **yes**, in its strongest form, and not because a write failed silently: reconciliation never runs at all, so the count stayed at 1 through the whole episode and the journal never moved.
- *An ending no reader learns* — **yes**, and it is the serious one. **The arrangement's own death is recorded nowhere Smoozy writes.** The run record still names the dead run. The only trace is the machine's own log, and with the disk full that log had `-- No entries --` for that boot. A reader asking the machine gets `could not reach the endpoint` and nothing that separates *given up permanently* from *briefly down*.
- *A hold left unreleased* — **no**, and this is the safe direction: the held job was still `held` with its hold unreleased at every point, and after recovery it was still `held` while the running job resumed.

---

## 2. The container boot hook, run as observed — and a placement that can never be confirmed

**This is the unchecked fact the sixth spine document's round parked, and the container answered it directly rather than by inference.** The provider's own boot script is readable on the machine, and the first process waits on it. The relevant lines, verbatim from the machine:

```
			if [ "$PWD" = "/" ]; then cd ~ ; fi ;
			touch /etc/rc.local;
			if [ ! -e ./onstart.sh ]; then
				echo '#!/bin/bash' > onstart.sh; ...
			fi;
			[ -e "/root/onstart.sh" ] || ln -s "$PWD/onstart.sh" "/root/onstart.sh";
			( ( ( bash /etc/rc.local 2>&1 >> /var/log/onstart.log; bash /root/onstart.sh |& tee -a /var/log/onstart.log ) & ) & );
```

- **The path is `/root/onstart.sh`**, which is the path the arrangement names.
- **It is run by handing the file to a shell**, not executed — the execute bit and the first line are irrelevant, and as shipped the file is mode 0644 and still runs.
- **The identity is root**, from the first process. Witnessed: `rc.local ran at 2026-08-18T13:36:13Z as uid=0 pid1=sh`.
- **It is backgrounded and detached**, so whatever it starts reparents to the first process.
- **It is not overwritten if it is already there**, so Smoozy's content survives.
- **`/etc/rc.local` is a second hook**, run first by the same line — and `/etc` is world-readable while `/root` is not.

**It runs again on every restart of an existing instance, proved twice and three independent ways.** After each reboot the process table showed **Smoozy's own hook line** running — the root step handing over to the non-root login, which starts the keeper, which starts the supervisor; a log file only the hook writes was created at the restart moment; and a witness file gained one line per reboot, at 13:36:13 and 13:39:09. **The whole arrangement came back only because the hook ran.**

### 2.1 The failure that was feared did not happen. The opposite one did, and it is permanent.

**`/root` is mode 0700 root:root, and the read-back is performed by the non-root placement.** So:

- The read before the root step failed: `container hook at /root/onstart.sh could not be read ([Errno 13] Permission denied: '/root/onstart.sh')`
- The root step installed the hook **byte-perfectly**.
- **The read after it failed with the identical message**, and the placement recorded `"phase": "failed before commit"`.

**On this provider, as both the provider and the arrangement ship them, a container placement can never be confirmed and never commits.** It is not the silent confirmation the last round predicted — it is a **permanent, loud refusal** whose message sends the operator round a loop that cannot terminate, because it tells them to run the root step they have just run. The cause is a directory permission neither side thinks about: the root step installs the hook without making it *reachable* by the login that must read it back.

**Two fixes exist and both were established on the machine.** Making `/root` traversable is enough — a named file inside it can then be read while listing the directory is still refused. Or the hook goes at `/etc/rc.local`, which the same provider line runs and whose directory is world-readable already, needing no permission change at all.

### 2.2 What could not be posed here

**What a recycle does to a hook Smoozy wrote.** The tester was forbidden to recycle and did not. Reading the provider's script alone suggests a recycle that resets the writable layer would restore the provider's two-line default, **but that is a reading of a script rather than an observation and is recorded as such.**

---

## 3. Fact 2 at the promise level — the stop reaches Blender itself, and the capture survives its own supervisor

### 3.1 The recorded render process is Blender, and never a shell

Checked by hand on both boxes. On the virtual machine the execution group is the worker (`comm=python3`), its child is the recorded target (`comm=blender`, 44 threads), and the launch material `exec`s itself away so nothing sits between them. The container gave the same shape.

### 3.2 The interrupt, under all three signals, on both kinds

| Signal | Machine | Time to recorded outcome | Blender exit | Recorded outcome |
|---|---|---|---|---|
| polite interrupt | virtual machine | ~3 s | `1` | left stopped |
| terminate | virtual machine | ~1 s | `-15` | left stopped |
| forcible kill | virtual machine | ~1 s | `-9` | left stopped |
| polite interrupt | container | ~14 s | `1` | left stopped |
| terminate | container | 1.9 s | `-15` | left stopped |
| forcible kill | container | 2.3 s | `-9` | left stopped |

**The signal went to the recorded render process every time, never to the worker and never to a shell.** The journal recorded the request, the issue and the outcome in order on both boxes, and **nothing restarted the render** — exactly one Blender throughout.

**The capability reduction touches none of it.** On the container the non-root login holds an empty effective capability set and even root is bounded without the seven capabilities named; signalling a process you own needs none of them, and reading the process's own state, its name and its start-time field all worked identically to ordinary Linux. **There is a reason to expect none:** the one relevant missing capability is the one that permits tracing another process, and nothing in the stop uses it — because the shape forbids finding anything by process search, nothing in the stop depends on a capability a container lacks.

### 3.3 The orphaned-worker half — and the first result was the throwaway's own unit

**First attempt, on the unit exactly as the throwaway writes it: the worker and Blender were both gone**, and reconciliation wrote `re-run once` rather than `resumed`.

**That is the unit and not the machine, and it was separated by three probes.** The unit carries no kill-mode line, so the service manager's default takes the whole control group down with the main process. Three scratch units, none of them the real one, each a supervisor starting a worker and then forcibly killed:

| Probe | Unit | Result |
|---|---|---|
| a — the throwaway's exact shape | default kill mode | **worker taken down with the supervisor** |
| b — the same plus one line | kill mode set to the process only | **worker survived**, reparented to the user service manager |
| c — the throwaway's shape, worker in its own transient scope | default kill mode | **worker survived** |

**So the machine permits a worker to outlive its supervisor two independent ways, both available to an unprivileged login with no elevated access.**

**With one line corrected, the case the whole capture arrangement exists for was seen working for the first time.** A real 100-frame Cycles render, the supervisor forcibly killed 25 seconds in with 82 frames already written and 393,216 bytes already in the ordinary sink:

- the worker and Blender both survived, the worker reparented to the user service manager
- the restarter brought a fresh supervisor up with a fresh run identifier
- the disposition was **`resumed`**, `attempt_count` still 1 — **no frame re-rendered**
- **the render finished under a supervisor it never met**, and its report was accepted: `written_by: the supervisor, from the worker's report`
- final counts `{"complaints": 0, "ordinary": 514253}`, exit status 0, both stream ends reported, 100 frames in the output folder

**514,253 bytes is byte for byte the same total as the identical render run with no interruption at all**, and Smoozy's own count matches the sink's size exactly.

**The unit is a finding in its own right.** The architecture says the registration is a user unit, enabled, with lingering granted, and says nothing about what that unit does to a worker when the supervisor dies. **Written exactly as described, it loses a paid render on every supervisor restart.**

### 3.4 The registration read-back is asymmetric, and nothing had noticed

**It refuses loudly when the unit file's bytes change.** Observed by accident when a placement ran against a unit file that had been rewritten: `"reason": "K15: registration content differs from the record Smoozy wrote (246 bytes on the machine, 268 recorded)"`, `"result": "failed before commit"`.

**It confirms happily when a supplementary configuration file changes what the unit actually does.** With the kill-mode correction in place as a drop-in — unit file digest unchanged, effective behaviour changed — the read-back answered `{"K15_confirmed": true, "detail": "content matches the record"}`.

**Both halves it compares are Smoozy's own**, so a third party changing the effective configuration is invisible to it. That is the container branch's failure shape arriving on the branch that was supposed to be the loud one.

---

## 4. Fact 68's full form — a stop with nobody connected — answered yes

A detached script scheduled the stop 90 seconds out and recorded the conditions at the moment it acted; the tester disconnected entirely and did not touch the box.

At the moment of the stop: the logged-in list was **empty**, and the only connection process was the listener. The stop went **through the ordinary command path**, wrote the desired state, and issued the signal to the recorded render process. Blender was alive before and gone after, and the recorded outcome afterwards was `left stopped` with counts and exit status.

**One honest limit, stated rather than smoothed over:** the machine's session manager still listed one session for the supervisor's login, with no seat and no terminal, because the detached script inherited it — not because anyone was connected.

**Thin: one scheduled stop.**

---

## 5. The forced identifier reuse, against the throwaway's own recorded pair — answered yes

**A finding came first and blocked the attempt.** The supervisor starts its workers and never waits on them, so a killed worker becomes a defunct entry that persists. Its start-time field still reads exactly what was recorded, so **the recorded pair validates for a process that no longer exists** — the rule that a pair which no longer validates means the group has ended never fires for a dead worker while its parent lives. **That is accidentally protective and it is also a hole: the machine believes a dead group is live.** It bit this test and it bit [Section 9](#9-fact-49--how-far-a-half-finished-setup-got--answered-yes) as well.

To pose the question the supervisor was killed so the service manager reaped the entry, with the operation record preserved verbatim first. Then, as root, the next-identifier control was set and an unrelated sleeping process took the identifier:

```
the pair the supervisor recorded at the moment the worker was started:
  {"identifier": 3873, "kernel_start_time": "62775", ...}
a new, unrelated process now holds identifier 3873 (forced on attempt 1)
  comm = sleep
  recorded kernel_start_time = 62775
  kernel_start_time now      = 68280
exists=True matches=False
group_validates(the throwaway's own recorded pair) = False
```

The recorded render-process pair behaved the same way. **A later, unrelated process validating against the recorded pair would have been the negative result, and it did not happen.**

**The container confirmed it cannot pose this at all, in the machine's own words rather than by assumption**: the identifier space is 4,194,304; the next-identifier control is on a read-only mount (`Read-only file system`, errno 30); and a namespace of one's own is refused (`unshare failed: Operation not permitted`) for all three forms tried. **The third refusal is the runtime's own restriction rather than the usual suspects** — the machine's own limits on user namespaces are generous and the unprivileged switch is on.

---

## 6. Fact 8 in full — 312 trials, and the window is about three milliseconds

The connection was cut mid-submission with the operation identity chosen at the desk before the request was sent.

**Round one — 82 trials, cold connections, cuts from 0.05 s to 1.0 s.** Every one answered *no record of that identity*, and the job was not in the queue. **That is not a test and the tester said so:** the handshake to this box costs about 1.6–1.9 seconds, so a cut at one second lands before the request has been sent at all.

**Round two — 120 trials over a warm connection, cuts 0.02 s to 0.40 s.** Every one answered *a recorded outcome*, job present. Consistent, and past the window.

**Round three — 110 trials, warm, cuts 0.000 s to 0.018 s.** This straddles it: every trial at 0.000–0.002 s answered *no record*; every trial from 0.004 s upward answered *a recorded outcome*; 0.003 s split six to four. **The transition is at about three milliseconds, and *still in hand* was never once returned** — the window in which a submission is accepted but has no recorded outcome is narrower than a shell can aim at.

**Two rows looked like a disagreement and are not.** Two identities answered *no record* while the job appeared in the queue read that followed; re-asked afterwards, both answered *a recorded outcome* with a job identity. **Killing the local end of a connection does not kill the remote command instantly**, so in 2 of 202 warm trials the submission landed between the two reads. The machine's answer was correct at the moment it was given. **The lesson is the tester's own harness and worth keeping: cutting a connection does not guarantee the request never lands.**

**No disagreement between the machine's answer and the queue in 312 trials.**

---

## 7. Fact 70's second half — the four collection outcomes, and the measurement nobody had taken

### 7.1 The outcomes — answered yes

All six cases were run and **read back with nobody connected**, from a detached script, except where noted.

| Case | Ordinary stream | Complaints stream |
|---|---|---|
| a render that prints nothing | `collected, and the stream was empty` — *"the marker passed and nothing followed it"* | `collected, and the stream was empty` — *"an end of stream and a count of zero; the complaints stream carries no marker"* |
| a real Blender render quiet on complaints | `collected`, 5,286 bytes | `collected, and the stream was empty`, 0 bytes |
| a render that prints | `collected`, 8,089 | `collected`, 3,692 |
| the render killed mid-stream | `collected`, exit `-9`, 1,446 | `collected`, 1,041 |
| **the worker killed mid-stream** | `collection incomplete` — *"the worker never reported an end of stream"*, counts null, `written_by: reconciliation` | the same |
| the sinks never made | `never established` — *"no capture record exists for the attempt"* | the same |
| a refused write to a sink | `collection incomplete` with `errno 28`, count stopped at 512 | `collected`, 3,692 — **the render was not touched** |

**Every count agreed exactly with what the sinks held.** *Collected and empty* was **not** written where the worker was killed, and *never established* is plainly distinct from *collected and empty*. The refused-write case is the one read taken with a connection open; every other was read with nobody connected.

### 7.2 The measurement — bytes per frame on a real animation

Nothing anywhere had measured this. Each run is a real Blender render of the stated frame count through the throwaway's own counting loop, on the virtual machine, divided by frames.

| Blender | Scene | Frames | **Ordinary B/frame** | **Complaints B/frame** | Totals |
|---|---|---|---|---|---|
| 4.5.9 LTS | Cycles 32 samples, 320×180 | 100 | **5,142.5** | **0.0** | 514,253 / 0 |
| 5.0.1 | the same scene | 100 | **3,713.0** | **0.0** | 371,302 / 0 |
| 4.5.9 LTS | the same, plus a texture whose file is absent | 100 | **5,284.5** | **0.0** | 528,447 / 0 |
| 5.0.1 | the same | 100 | **3,931.0** | **0.0** | 393,097 / 0 |
| 4.5.9 LTS | EEVEE Next, 640×360 | 20 | **425.6** | **0.0** | 8,513 / 0 |

**About 5 kB per frame on the ordinary stream under 4.5, about 3.7 kB under 5.0, and nothing at all on the complaints stream** for a scene that renders cleanly — roughly a 28% reduction across the version boundary for an identical scene.

**Two caveats, both the tester's own.** A scene with a deliberately missing file put **nothing** on the complaints stream in either version, so on this box a warning-heavy scene did not separate the streams — that is a Blender observation for this box on this day and a question owed elsewhere, not a statement about Blender. And the wall-clock gap between the two versions is almost certainly a warm cache on the second run rather than a version difference; it is reported because it is what the clock said.

**A mechanical note for whoever builds from this:** the throwaway's capture loop reads in 64 kB blocks, so sinks fill in 64 kB steps and a size read mid-render lags badly. Final counts are exact; intermediate ones are not.

**Thin: one render each, one scene family, processor-only, on a box that was otherwise idle.**

---

## 8. Fact 78 — the commit's interruption boundaries, on ext4, xfs and overlayfs

The virtual machine posed **ext4** (a second root of its own) and **xfs** (a file-backed loop, `crc=1 finobt=1`); the container posed **overlayfs**. Every death was a real forcible kill without unwinding.

### 8.1 The four commit boundaries — identical on all three filesystems

| Boundary | Phase record left at | Pointer | Versions on disk | Retry under the same identity |
|---|---|---|---|---|
| staging | `staging`, naming the staging location | v1 | v1 | completes → `verified` at v2 |
| pointer | `committing`, with the pre-commit run identifier | v1 | v1 + v2 | completes → `verified` at v2 |
| pointer-done | `committing` | v2 | v1 + v2 | completes → `verified` at v2 |
| pre-health | `committed, unverified` | v2 | v1 + v2 | ext4 and overlayfs: `verified`; xfs: `rolled back` |

**No part-arrived bytes at a final name, ever, on any of the three.** **The pointer always named exactly one of the two versions — never neither.** Every retry under the same identity resolved correctly. The container's checks returned empty mismatch, missing, unexpected-temporary and staging-leftover lists at every boundary, with the pointer target present in all cases.

**The one divergence is not a filesystem difference and is recorded as such.** The xfs pre-health retry came back `{"reason": "the supervisor did not answer", "result": "rolled back"}` with the pointer restored to v1 — which is the rollback path working correctly, restoring from the recorded previous version rather than re-reading the machine. **It is one run each and the tester attributes it to the health-exchange deadline on a box contending for one shared unit, not to xfs.**

**One precise caveat, from the container.** The `pointer` and `pointer-done` boundaries leave the *same* phase and are told apart only by reading the pointer. *"The retry can tell from the phase record which branch it is on"* is not literally true — it can tell from the phase record **plus the pointer**.

**A real rollback was reached honestly** on the container, with a version whose self-test passes and whose supervisor exits non-zero: `"rollback_reason": "the supervisor did not answer"`, `"last_error": "[Errno 111] Connection refused"`.

### 8.2 The hold record's publication — identical on all three

- **Interrupted before the rename:** the record is left under a temporary name, fsynced. **Nothing at a final name.** The machine answers with zero holds and zero placement entries in the journal. The caller receives nothing describing the protocol.
- **Interrupted after the rename and before the journal entry:** the record is complete at a final name, parses, and **carries its reason and its subject**. But **the hold stands with no journal entry** — a reader following the journal never learns it exists, while the hold silently blocks every package commit.

**So no hold ever reads as a hold with no reason.** The second case is the one the document's own machinery already names, arriving live on all three filesystems.

**And the temporary file left by the interrupted publication is never cleaned up.** It survived hours, several supervisor restarts and full reconciliation on the container, and survived on the virtual machine too. The throwaway's own sweep is bounded to the package staging area and does not reach it. **Litter, on all three.**

### 8.3 The rollback boundary — could not be posed, and each blockage is a finding

The route the throwaway documents is to mask the unit so the health exchange cannot be answered. **The image refuses it:** `Failed to mask unit: File ... already exists.` — the service manager will not mask a unit whose own file is present in the same directory. So the exchange succeeded and the transaction went to `verified`; the rollback branch was never entered.

Making the restarter start a sleeping process instead does make the exchange fail honestly, **but by then the test root would not enter a commit at all**, answering `deferred` and naming an open operation and an unreleased hold. Both were cleared and confirmed gone on disk, **and it stayed deferred anyway** — on a fresh identity and on the re-sent identity alike, still naming a hold that no longer existed.

**Three findings fall out of that, and they are worth more than the boundary that was lost.**

1. **The deferral reason a caller is shown is the one stored when the commit was first deferred, not a fresh reading.** It went on naming a hold long after that hold was gone, so anyone acting on the message chases something that does not exist.
2. **Once the phase record reached `deferred`, this box never left it** — neither a fresh identity nor the re-sent identity got past it.
3. **The escape route depends on an identity that may never have been kept.** The first deferral was caused by a run whose identity the throwaway's own installer generated internally; if the operator did not save that line, there was no documented way back and the machine could not take a new package version again.

**A real rollback was observed (the xfs pre-health retry) but never interrupted mid-flight.**

---

## 9. Fact 49 — how far a half-finished setup got — answered yes

Boundaries: packages, driver, blender, root-handover, done.

**Cut at a recorded safe boundary**, no signal was sent at all. Afterwards the machine reported the setup in progress, named the operation, and gave the **third of five boundaries by name with the moment it was reached**, with the recorded outcome `left stopped`.

**A hard cut**, killed mid-step rather than at a boundary, gave the **second of five** — which is right: it names the last boundary actually *passed*, not the one it died in.

**A job submitted while the setup state stood was never admitted**, staying queued before and after a reboot.

**The state survives a reboot with no worker behind it**, still naming its boundary and its moment, with the queued job still queued.

**One thing the machine got wrong, and it is the defunct-entry hole again.** Immediately after the hard cut, with **no setup worker alive at all**, the machine reported the recorded pair as still validating. Only the reboot, which reaped the entry, flipped it. **So between the cut and the next supervisor start, the machine says a dead setup group is live.**

---

## 10. Fact 27 at the ready-action weight — answered, and it differs by machine kind

**On the container: nothing is open at all.** As the supervisor's login: elevation demands a password it does not have; switching to root fails authentication; the capability set is empty. The three things a ready action needs are each refused — installing software (`are you root?`, exit 100), installing the boot hook (`cannot create regular file '/root/onstart.sh': Permission denied`), and writing anywhere system (`/usr/local/bin`, `/opt`, `/etc`, `/usr/bin`, `/root` all refused). **The container arrangement therefore requires a root step the arrangement itself cannot perform** — and on this provider that root step must do one thing more than it does now, per [2.1](#21-the-failure-that-was-feared-did-not-happen-the-opposite-one-did-and-it-is-permanent), or the placement never completes.

**On the virtual machine the answer is in two halves.**

*What the ready action does **not** need root for:* **the whole boot registration.** Writing the user unit, enabling it, granting lingering to its own login, the linger read-back and the registration confirmation all ran unprivileged and all succeeded. **On a systemd rental there is no elevated step in the placement at all** — unlike the container branch, where the hook install is root's.

*What it **does** need root for:* everything about preparing the machine to render. On this rental that was not hypothetical — see [Section 11](#11-the-rental-arrived-with-a-card-its-own-driver-could-not-open) — and it is refused outright to the supervisor's login. Also refused: reading the root home, writing the system configuration directory, and mounting, which the ready action needs if the state area is ever to go on its own filesystem.

*Is elevation present without a password?* **Not for the supervisor's login, and yes for another.** The supervisor's login has no route to root at all. But the image ships `ubuntu ALL=(ALL) NOPASSWD:ALL` in its own drop-in. **So passwordless root exists on this box, held by the cloud-init account rather than by the login Smoozy runs as** — the ready action's elevated half must be performed as that account, or as root over the connection the rental hands over, and privilege dropped for the placement.

**An image quirk worth recording:** every elevation attempt first printed `unable to resolve host ubuntu: Temporary failure in name resolution`, because the image's own hostname is not in its hosts file — which adds a name-lookup timeout to every privileged command.

### 10.1 The lingering grant — how it succeeds, and the boundary that refuses it

**Granting lingering succeeded unprivileged**, by name, run as the login itself, with the read-back answering `Linger=yes`.

**The image's own shipped policy reads as though it should be refused** — the by-name action demands administrator authentication in all three columns. **It is not refused, and the reason matters:** the machine routes a request whose target is the caller's own login to the *self* action, which the image allows outright. **The boundary was proved by asking for another login's:**

```
$ loginctl enable-linger tester2          # run as smoozy
Could not enable linger: Interactive authentication required.
```

**So the by-name grant works only because the name is the caller's own. A build that ever grants lingering for a different login than the one it is running as is refused on this image.** The earlier box's failure of the bare *self* form was a different failure; here the bare form returned success.

**The hazard the arrangement carries about the runtime directory did not bite:** the per-user runtime directory existed on every login, so the user-level control tool never failed before the read-back. Under lingering it persists with nobody connected.

---

## 11. The rental arrived with a card its own driver could not open

Nobody asked about this. **It sits on the ready action's path and it bears on Fact 27 and Fact 50.**

On arrival the card tool answered `NVIDIA-SMI has failed because it couldn't communicate with the NVIDIA driver.` The card is on the bus and a driver is installed, but it is the **open** kernel module variant, which cannot drive this generation of card. The kernel's own words: `installed in this system is not supported by open nvidia.ko because it does not include the required GPU System Processor (GSP)`, `probe of 0000:00:07.0 failed with error -1`, `None of the NVIDIA devices were initialized.` The card device node did not exist and loading the module by hand answered `No such device`.

**The repair was the proprietary driver package, rebuilt against both installed kernels, plus a reboot — about six minutes, root required throughout.** After it, the card answered normally.

**The recycle at the end restored the broken state**, so this is a property of the image and **every rental of this machine begins there.**

---

## 12. The container boot hook across providers, read rather than run

A reading of each provider's own documentation, done 2026-08-18. **Nothing was run.** Every claim below is quoted from the page at the address given, on the provider's own documentation site; where a page could not be retrieved, or where the documentation simply does not say, that is the finding.

### 12.1 The premise does not survive: it is two mechanisms wearing one name

- **A file at a path inside the container.** Documented by **Vast.ai alone**, at `/root/onstart.sh`: *"Place startup commands in `/root/onstart.sh`. This script runs automatically on container startup."* (https://docs.vast.ai/guides/reference/faq/instances, read 2026-08-18). **No other provider read documents any path at which a customer may leave a script for the provider to run.**
- **A stored field on the instance or template that replaces the image's start command.** This is **RunPod**'s (*"Customize the command that runs when your Pod starts. This overrides the default CMD instruction in your Docker container."* — https://docs.runpod.io/pods/templates/manage-templates), **Novita**'s (fields `command` and `entrypoint` — https://novita.ai/docs/api-reference/gpu-instance-create-instance.md), and **Paperspace Deployments**' (*"command to run on startup"* — https://docs.digitalocean.com/products/paperspace/deployments/reference/deployment-spec/), all read 2026-08-18. **There is no path.** A script written into the filesystem by a builder expecting the first shape would sit there unexecuted, **and nothing about writing it would fail.**

**And two of the providers named do not rent containers at all.** **Lambda** and **Hyperstack** rent virtual machines; a container is something the customer runs on one. Their equivalent is cloud-init, and Hyperstack states its timing outright: *"The script runs when the virtual machine first boots."* (https://docs.hyperstack.cloud/docs/virtual-machines/initialization-configuration/, read 2026-08-18).

| Provider | Rents a container? | Runs something at start? | Mechanism |
|---|---|---|---|
| Vast.ai | yes | yes | a file — `/root/onstart.sh` — plus a stored on-start field |
| RunPod | yes | yes | stored field only; overrides the image's start command |
| Novita AI | yes | yes | stored fields only — `command` and `entrypoint` |
| Paperspace Deployments | yes | yes | stored field only |
| Paperspace Notebooks | yes | not documented | — |
| Sesterce (container shape) | yes | not documented | — |
| Paperspace Machines | no — virtual machine | yes | assigned startup script, *"first boot or on every boot"* |
| Hyperstack | no — virtual machine | yes | cloud-init, *"when the virtual machine first boots"* |
| Lambda | no — virtual machine | yes | cloud-init user data at launch |
| SaladCloud | could not be read | — | — |

### 12.2 Where the silence falls, and it is total

**Whether the hook runs again on a restart or a recycle is unanswered by every single provider read, without exception, including Vast.** Not one page states, of a container, that the start-up hook runs on a subsequent start as well as the first. Vast documents that a reboot *"executes docker stop/start commands on the host machine"* and never connects it to the on-start script; RunPod documents resume and never connects it to the start command; Novita's start endpoint page carries no description at all.

**The identity the hook runs as is unanswered by every provider read.** Vast's path sits under `/root`, which is a hint and not a statement.

**Documented overwriting, and what becomes of a customer-written hook at a recycle, is unanswered by every provider read.** For the stored-field providers the question partly dissolves — the hook is configuration rather than a file. **For Vast, the one provider where the hook *is* a file and therefore the one where the question has teeth, it is not addressed.**

**SaladCloud refused every documentation fetch on 2026-08-18 with an HTTP 403, and no claim about it is recorded.** Search listings paraphrased those pages; a paraphrase is not the thing's own documentation.

### 12.3 The one documented fact that bears hardest

RunPod, of a Pod's own storage: *"The container disk provides temporary storage for the operating system and session data. It's created when a Pod launches and is cleared when the Pod stops."* against *"Data stored in the `/workspace` directory survives Pod stops and restarts"* (https://docs.runpod.io/pods/storage/types, read 2026-08-18). **So on RunPod a hook written as a file anywhere outside the persistent volume is erased by exactly the event a boot hook exists to survive** — and a read-back of Smoozy's own file at placement time would have confirmed it happily, minutes before the wipe.

**One avenue was tried and closed without a result.** Vast appears to have a second, remotely fetched provisioning mechanism whose timing may be documented; the page carrying the wording could not be reached on 2026-08-18. A search listing is not the thing's own documentation, **so no claim about it is recorded** — it is noted only so a later reader knows the avenue is open rather than closed by evidence.

---

## 13. Fact 92 — the clock over the length of a render — answered yes, with the qualifier that matters more than the number

**On the virtual machine**, the machine's own realtime clock was sampled once a second against its monotonic clock for the whole of a real 400-frame Cycles render:

```
samples: 1499        watched: 1499.6 s (25.0 minutes), 371 frames rendered inside it
band width: 0.0129 ms
largest single step between consecutive samples: 0.0124 ms
```

**No jump, no slew, nothing a reading would notice.**

**And the qualifier: nothing is disciplining that clock.** `NTP=no`, `NTPSynchronized=no`, server `n/a`, **packet count 0**, and the alternative time daemon absent. **Zero time-service packets, ever.** So the band is tight **because nothing is stepping it**, and it says nothing whatever about whether the clock is *correct* against the outside world — only that it does not lurch during a render. **A box that later starts synchronising is a different box for this question.**

**On the container the answer is the same and the reason is different.** It has **no clock of its own**, established four ways: its time namespace is the initial one with zero offset; nothing inside can discipline it (setting the date is refused even as root, the hardware clock is unreachable, and the capability that would permit it is absent from root's bounding set); no time daemon runs inside; and its monotonic and boot-time clocks read the **host's** uptime — 8.21 days, while the container was created that morning. **Both container restarts left the uptime and the boot identifier unchanged, so nothing inside can tell from them that it was restarted at all.**

Against an outside synchronised clock, 13 samples over 9.6 minutes spanning a live render gave a corrected offset spread of **0.039 s**, a largest step of 0.035 s, and drift within noise. **The container had nothing to do with that: it is the host's clock, and if the host stepped it the container would follow silently and could neither prevent nor detect it.**

---

## 14. Health readings on a container — reproduced, and the half that matters is still open

| | The machine's own | The container's allocation |
|---|---|---|
| memory | 15.57 GiB | **14.95 GiB** |
| processors | 4 | **3.84** |

Exactly the figures the earlier batch found, reproduced. The control-group interface is the version-2 one; the version-1 paths are absent. **The high-water reading answers the literal *max*, so a reader taking that instead falls back to the wrong figure.** The throwaway took the right ones and named its source.

**The card readings all agreed — and that is the trap passing, not the question closing.** The driver's own answer, two count-as-a-count routes, the kernel listing and the bus scan **all gave 1, because the host has exactly one card**. The mechanism is nonetheless established: **the bus scan is unmistakably the host's unfiltered view**, listing the host's own storage controller, its audio device and its network card, none of which belong to this container. **So it would lie on a multi-card host; only the arithmetic consequence is invisible here.** **This needs a multi-card host and must not be treated as closed.**

**Two host-inherited readings nobody asked about.** Free-memory reporting shows **4 GiB of swap** that is the host's, and the uptime reading gives the host's 8.2 days. **Both would lie to a reader.** And the provider's own API reported the processor count as `4` and `4.0` effective, **disagreeing with the control group's 3.84**.

---

## 15. Facts 40 and 41 — reading the machine, and removing a history entry

### 15.1 Fact 40 — answered yes, including the disk-full form

*The negative result, written first by each tester:* a read that disagrees with what the machine holds; or, with the state mount full, a read that returns success while quietly answering from nothing, or one that fails without saying which read failed or why.

Over the connection, on both boxes: the line answered every job with its state, its admission point, its operation and its unreleased hold; the note answered attempt count, ending with outcome and detail, re-run marks and label — **and `"ending": null` for a job that has not ended rather than inventing one**; the history answered its count and its entries.

**With the filesystem at a genuine zero, all of them answered normally and truthfully**, on both boxes — including free space as `0` rather than a stale figure — before the refused writes and again after them.

**Difference between the machine kinds: none either tester could point to.** These are reads of files the login owns over an endpoint the login owns.

### 15.2 Fact 41 — answered **in part**, and the same way on both boxes

*The negative result, written first:* the entry survives; or it goes but its streams and its note stay behind; or another job's material goes with it; or the removal reports success without the machine's copy ending.

**The removal reported `removed: true`, and the subject genuinely ended:** the history count dropped by one and the subject was absent; the history record, the note, the collection record and the queue entry were all gone; the removal was journalled; and **no other job's material was touched**.

**But the bytes stayed.** On both boxes, under the removed job's own identity, fully readable, while the machine reports nothing about it:

```
-rw------- 1 smoozy smoozy 9814 .../streams/job-<removed>/1/ordinary
-rw-rw-r-- 1 smoozy smoozy 49149 .../jobs/job-<removed>/frame_0001.png
-rwxr-xr-x 1 smoozy smoozy  268 .../job-records/job-<removed>/launch-attempt-1.sh
```

**The captured streams, the rendered frame and the launch material all survive.** Both testers independently, and both flagged that the throwaway's own expectation was that the streams end with the subject. **Thin: one removal each, not repeated.**

---

## 16. Fact 50 — the ready action run again on a half-finished machine — answered **no**

Run **last, on the box being given up**, against the machine left half-finished by [Section 9](#9-fact-49--how-far-a-half-finished-setup-got--answered-yes) — state standing, last boundary the third of five, two of five steps genuinely done.

*The negative result, written first:* the second run is admitted while the first run's state still stands and it re-does steps that are not safe to repeat; or it clears the first run's state without being asked, so the machine reads as ready when it is not.

**A second setup run was not refused.** No error, no refusal — a second setup worker started and got its own recorded execution group, while the machine's own state still named only the first run. **Two setup runs in flight at once, and the machine's state naming one of them.**

**And then it did the harm.** The unauthorised second run walked **all five boundaries from the beginning** and, on reaching the end, **cleared the machine-level state itself** — leaving a machine that had genuinely completed two of five steps marked **ready**, by a run nobody authorised, on the strength of a pass that repeated every step from the start.

**On a real machine those steps are package installation and driver installation** — exactly the work that arrived broken on this rental, per [Section 11](#11-the-rental-arrived-with-a-card-its-own-driver-could-not-open), and exactly the work that is not free to repeat.

**The documented recovery behaved correctly.** Clearing under the *original* identity answered `{"in_progress": false, "reconciliation": "ran"}`, and a fresh setup run afterwards started cleanly with a new identity.

**Did it leave the card not answering? No.** Checked after the second run and again after the clear, the card answered normally. **The card was unharmed; the harm the fact is about is the state, and it is real.**

---

## 17. The host keys and the recycle — answered, and the answer refutes the question's premise

The question was posed because a virtual machine generates its own host keys at first boot, so a container's answer cannot carry it.

**Before anything else was done, and again immediately before the recycle** — unchanged across five reboots and one kernel panic in between — the box offered four host keys, every one commented `root@ubuntu`, with key files all dated **2025-11-06 20:55:50** to the nanosecond.

**After the recycle**, on a changed port: **identical. Every fingerprint, every file size, every timestamp to the nanosecond.** The scanned public keys matched byte for byte too.

**The recycle was thorough in every other respect.** Both logins gone, Blender gone, the throwaway gone, the lingering flag gone, the repaired driver gone and the broken one back. **The host keys stayed.**

**So this provider's image does not generate host keys at first boot. It ships them** — generated months before this rental existed, with `root@ubuntu` baked into every comment. **An unchanged set after a recycle is the finding, not a failed test.**

**Two consequences.**

1. **A host key does not identify this machine.** Every rental of this image, on every host, presents the same four keys. Anything keying on them to recognise *the same machine* would also recognise every other machine from the same image.
2. **The recycle changed the connection's port and not its address** — 15840 became 15820, the address unchanged. **So a reconnection cannot key on the port either, and the address alone is not specific to an instance.** Whatever survives a recycle for identification purposes, it is neither the endpoint nor the host key.

---

## 18. Fact 60 re-posed against a real render — answered yes

A real 400-frame Cycles render, 44 threads, seen from a second ordinary login on the virtual machine.

**Seen: everything.** The worker and Blender both listed with owner and command line, the process name readable, the thread count readable, **and even the start-time field readable**.

**Stopped: nothing, every time.** Polite and forcible stops against both the render and the worker were refused with `Operation not permitted`; a pattern kill of the render was refused; a pattern kill of every process of that login was refused seven times over. **All 44 threads survived and the render kept going.**

**The existence probe is refused too**, so a second login cannot even test liveness by signal.

**And the endpoint holds.** From the second login the endpoint's directory was `Permission denied`, and the client binary itself was `Permission denied`.

**A real render's children do not change the answer.** The earlier stand-in result — seen yes, stopped no — stands unchanged.

---

## 19. What was not reached, and what is thin

**Not reached.**

- **Fact 78's rollback boundary** — blocked twice over, and each blockage is written up at [8.3](#83-the-rollback-boundary--could-not-be-posed-and-each-blockage-is-a-finding).
- **The card readings' *they lie* half** — needs a multi-card host. [14](#14-health-readings-on-a-container--reproduced-and-the-half-that-matters-is-still-open). **Not closed.**
- **What a recycle does to a hook Smoozy wrote** — forbidden on the container, not posed.
- **A power cut.** The kernel panic is the nearest thing this project has reached and it is not one; the filesystem needed journal recovery, and no claim beyond that is made.
- **The complaints stream under load.** Zero bytes in every real render, including a scene with a deliberately missing file. **A measured zero rather than a demonstrated capacity.**

**Thin, and named by the testers themselves.** Fact 78's boundaries are one run each per filesystem. The bytes-per-frame figures are one render each. Fact 68 rests on one scheduled stop. Fact 41 is one removal per box. The forced identifier reuse is one forced reuse, though it succeeded on the first attempt and the reading is unambiguous. Fact 8 is the opposite of thin at 312 trials, **and its first 82 were worthless and the tester says so.**

---

## 20. Things nobody asked about

- **The container's `/etc/environment` ships `HOME=/root`**, applied at every login, breaking every non-root login on that image.
- **The container's provider boot script is world-readable and handles the account's API key**, and the account key file is mode 0644 and is a **full account key** — it listed both instances on the account. **Anything running as any login on such a box can read it.**
- **The supervisor never reaps its workers**, so a killed worker persists as a defunct entry keeping its start-time field alive. It bit two tests. [Section 5](#5-the-forced-identifier-reuse-against-the-throwaways-own-recorded-pair--answered-yes), [Section 9](#9-fact-49--how-far-a-half-finished-setup-got--answered-yes).
- **A package commit blocked by holds or by an in-flight operation could not be cleared by a fresh identity** — it answered `deferred` and acted on nothing. Only re-sending the **same** identity moved it forward, and that is easy to mistake for a hang.
- **The throwaway's own instructions had two faults that produce false passes**, both worth knowing because both look like results: the hold-boundary instruction sets its variable on the client while the die point is in the supervisor, so it produces a clean-looking pass that poses nothing; and the frame-range form it gives is not one Blender parses.

---

## 21. Four Blender observations, and none of them is a claim about Blender

Each is what one box did on one day. **They are routed to Blender Research as questions and nothing here builds on them.**

1. **A polite interrupt does not stop at the current frame.** Under it, Blender abandoned the frame in hand and then **walked every remaining frame number doing no work** before quitting — all 200 frame numbers printed, 12 frames on disk, **82,823 bytes against about 9,813** for the same job terminated at frame 2. **Two consequences if it holds: time-to-stop grows with frames remaining rather than being constant, and a stopped job's byte count is badly skewed.**
2. **A malformed frame argument gives an error and exit status 0.** `Error: not a number '-f / --render-frame 1-200'.` with `exit_status: 0`, both streams collected, and an empty output folder. **A job that rendered nothing looks, by exit status alone, like a success.**
3. **A scene with a deliberately missing texture printed nothing on the complaints stream**, in both 4.5.9 and 5.0.1 — 0 bytes each. Whatever was said about the absent file went to the ordinary stream or was not said.
4. **Headless 4.5.9 ended on a polite interrupt with exit status 1**, not the conventional interrupt status, having finished the frame in hand rather than truncating it.
