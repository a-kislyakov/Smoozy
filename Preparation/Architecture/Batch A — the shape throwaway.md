# Batch A — the shape throwaway

What one rented machine answered on 2026-08-13, for the facts the Architecture stage's shape rests on. An **input** to the stage, like [The rented machine](The%20rented%20machine.md): nothing here is a decision or a plan.

**This is a frozen snapshot of one machine on one day**, and the machine was destroyed at the end of it. A different rental may differ in every particular. Read it as *this is what that box did*, never as *this is what a rented machine does*.

Every claim is **observed**, with what was run and what came back. Where something was not established, it says so.

## The machine

Rented from Vast.ai, reached over SSH on port 31567. One **NVIDIA GeForce RTX 3080** with 10240 MiB, driver **575.51.03**. An AMD Ryzen 7 3700X with 15 cores given to this machine, 12 GB of memory, 126 GB of disk with 119 GB free, ext4. Ubuntu on `systemd`, `systemd-detect-virt` reporting `kvm`. **No Blender installed**, no display server. `tmux`, `screen`, `nohup`, `setsid` and `systemd-run` present; `at`, `supervisord`, `runit` and `daemonize` all missing — the same split the earlier machine showed.

All work in `/root/smoozy-arch/`. The throwaway was a small program holding a queue in a file, doing one item every three seconds, logging each with its own process id and its parent's, and handling an interrupt by recording where it had got to.

---

## Fact 1 — whether a program Smoozy leaves on a rented machine can outlive the connection, hold a queue, and come back by itself

**Answered: yes, on all four clauses.**

**It outlives the connection that started it.** Started with `setsid nohup python3 worker.py > stdout.txt 2> stderr.txt < /dev/null &`, then the connection closed. From a fresh connection thirty-five seconds later:

```
   2337       1    2337 Ss         00:35 python3 worker.py
```

Its parent is **1**, the shell that started it (2332) no longer exists, and it is its own session leader. It had gone on working the whole time with nobody connected — `did job9` through `did job12` in the log, at three-second intervals.

**The line of work can be read back over a fresh connection.** `queue read back: 18 done, 181 left; next up: job19`.

**It comes back by itself after a reboot.** Arranged as an ordinary `systemd` unit with `Restart=always` and enabled at boot. Before the reboot: 19 done, 380 left, main process 3054, boot id `d1f9e7e1…`. The machine was rebooted with `systemctl reboot`, and **answered again in about ten seconds** with boot id `32d5f65b…` and `up 0 minutes`. The service was `active` with main process **600**, and the log read:

```
START sid=600 pgrp=600
did job22
did job23
```

**It resumed rather than starting over** — 23 done, 376 left, picking up from the queue file rather than from the beginning.

**The disk survived entirely.** Every file in the working folder was still there.

## Fact 2 — whether a thing Smoozy started can be reached over a connection and told to end

**Answered: yes.** The target was a process whose starting connection no longer existed. From a new connection, `kill -INT` against it: exit status 0, and the program's own record:

```
SIGNAL SIGINT received
EXITING cleanly, 18 done, 181 left
```
```
ended on SIGINT, 18 done, 181 left
```

**No work was lost.** The queue read 18 done and 181 left both before and after — it finished the item it was on and stopped at a boundary.

**The stop is not instant.** It takes effect on the program's next cycle: about **2.3 seconds** here, against a three-second cycle. A check made two seconds after sending the signal reported the program still running and would have been wrong. **Anything waiting on a stop needs a limit of its own**, which is what the joined research already said about waiting on a run to finish.

## Fact 68 — whether a stop can be issued by something on the machine, with nobody connected

**Answered: yes**, and the first attempt failed in a way worth keeping.

**The failure.** A detached script on the machine found its target with `pgrep -f "python3 worker.py"`, which matched **two** process ids. The variable held both, `kill` was handed a malformed argument, and the program went on working:

```
stopper: sent SIGINT to 2527
2660 at 23:48:12, nobody was connected
worker STILL RUNNING — the on-machine stop did not take
```

**So a stop must be aimed at a recorded identity, never at a pattern.** This is the same hazard the earlier machine recorded for `pgrep -f`, arriving through a different door.

**The success.** Redone against the identity the service manager itself holds:

```
stopper: SIGINT to recorded pid 2871 at 23:50:07, nobody connected, kill exit 0
SIGNAL SIGINT received
EXITING cleanly, 8 done, 391 left
```

Nobody was connected at the moment it happened.

## The constraint that matters most, and no fact asked for it

**The arrangement that makes work survive a reboot also undoes a deliberate stop.**

Two seconds after the clean stop above, `Restart=always` started a fresh process and the work carried on:

```
state now: active, MainPID now: 3054
did job9
did job10
```

The stop was honoured and then reversed, by the same mechanism that answers Fact 1's reboot clause. **Whatever holds work on a rented machine has to be able to tell a deliberate stop from a crash**, or Smoozy cannot both keep a queue alive and let the user stop a job. That is a requirement on the shape document, found by test rather than by argument.

## Fact 51 — restarting the machine over the connection

**Answered.** `systemctl reboot` issued over the connection Smoozy already held. The connection reported `Connection to 31.59.39.104 closed by remote host.` and the machine **answered again in about ten seconds**, with a new boot id. The connection could be re-established at the same address and port.

## Fact 55 — what a restart stops

**Answered for a transfer.** An 8 GB write was started detached and was in flight at the moment of the reboot. Afterwards:

```
-rw-r--r-- 1 root root 4773113856 Aug 12 23:50 inflight.bin
the transfer did not come back
```

**It left 4.7 GB of the 8 GB asked for, under the file's final name, with nothing marking it incomplete**, and nothing restarted it. Its error stream was empty — it said nothing about being cut off. This is the same shape the earlier machine found for a killed transfer, now confirmed for a restart.

## Fact 4 — whether a broken driver still produces a loud refusal when the technology is named at launch

**Answered: yes.** This is the case the register recorded as never run.

**First, the control, with the driver working.** Naming a technology the machine has hardware for renders and writes the file — `exit 0`, 18086 bytes. Naming one it has no hardware for refuses:

```
HIP   -> exit 1,   file none
RuntimeError: Error: Found no Cycles device of the specified type
```

**Then the driver was purged.** Two things stood in the way and both are findings of their own. `apt-get remove --purge "*nvidia*"` matched nothing useful, and removing the packages by name failed at exit 100:

```
E: Held packages were changed and -y was used without --allow-change-held-packages.
```

**The provider holds its driver packages.** They come off only with an explicit override. Separately, taking the kernel modules out with `rmmod` succeeded — `lsmod | grep -c nvidia` went from 5 to 0 — and `nvidia-smi` went on working immediately afterwards, because the modules load again on demand. **Anything checking whether a machine still has its driver has to ask something that uses it, not something that lists it.** That reproduces the earlier machine exactly.

**With the packages genuinely purged** (`nvidia-smi: command not found`, `libcuda.so.1` unknown to the linker), Blender 4.5.3 asked for OPTIX at launch:

```
exit=1  file=none
Error: Found no Cycles device of the specified type
DEVICES: [('AMD Ryzen 7 3700X 8-Core Processor', 'CPU')]
```

**It refused loudly, ended at a failing status, and wrote nothing.** So the decision the Rendering spec takes — naming the card technology when a render is meant to use the cards — does convert the quiet, expensive processor render into a refusal, and it does so with the driver broken and not only with the hardware absent.

**What this does not cover.** Only Blender 4.5 was installed, so 5.0, 5.1 and 5.2 are untried here. The driver was removed outright rather than mismatched, which is the other broken state and the one an unattended upgrade produces.

## The riders

**Fact 31 — a deleted file is not recoverable.** ext4, no `trash-cli`, no trash folder for root, no snapshotting filesystem. What is deleted is gone.

**Fact 62 — the machine reaches Telegram.** Three requests to `api.telegram.org` with a deliberately invalid token, each answering `HTTP 401` — the service answered and rejected the token, which is what proves reachability. Nothing about a longer period was tested.

**Fact 70, the plain half — both of a program's output streams can be captured, kept, read back with nobody connected, and searched.** A program printing to both and ending at status 3, started detached:

```
ordinary stream kept:   this went to the ordinary output|ordinary, second line|
complaints stream kept: this went to the complaints stream|
ending recorded:        3
```

The two streams stayed separate, the ending was recorded, and the kept output could be searched for a line. **Whether *the render printed nothing* can be told from *nothing was collected* was not tested** and remains Fact 70's own open half.

---

## What was not established on this machine

- **Facts 8 and 40 beyond their plain halves.** Both need something on the machine that takes a submission, which does not exist until the shape does.
- **Fact 4 in the other three Blender versions**, and with the driver **mismatched** rather than removed.
- **Fact 62 over a day** rather than over a minute.
- **Fact 58** — whether an open connection survives its key going away. Not run.
- **Fact 70's second half** — telling a silent render from uncollected output.
- **How long a stop may take at the worst**, rather than the one reading of about 2.3 seconds against a three-second cycle.

---

2026-08-13 — Claude Opus 5 (Claude Code) — Batch A, run and recorded by the architecture runner. The machine was destroyed afterwards.
