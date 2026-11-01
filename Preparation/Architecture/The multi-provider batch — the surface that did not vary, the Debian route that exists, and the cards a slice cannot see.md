# The multi-provider batch — the surface that did not vary, the Debian route that exists, and the cards a slice cannot see

What the machine batch of 2026-08-22 answered. An **input** to the stage, like the others: nothing here is a decision or a plan. Every claim is **observed**, with what was run, on which machine, and what came back. Where something was not established, it says so, and it says what was looked at. Rule 1's plain-English rule is suspended here, per the stage's exception.

**On the title, so nobody is misled by it.** *The surface that did not vary* is true of **every reading Smoozy leans on** and is not true of the whole surface: the announced extension set varied by OpenSSH version, in three extensions Smoozy does not use. [Group 1](#group-1--the-surface-everything-rests-on-and-the-part-smoozy-leans-on-did-not-vary) states both halves exactly.

**Why it existed.** [Server protocol and transfers](Server%20protocol%20and%20transfers.md) §12 item 11 says it of its own document: *"Every claim about a rented machine in this document is one or two boxes on one or two days, and the input documents disagree with each other in places."* This batch is variance, not discovery — with three exceptions that are discovery: the Debian driver route ([Server Setup](Server%20Setup.md) §13 item 22), the card readings inside a rented slice, and Facts 99, 100, 101 and 116.

**Approved by the Creator on 2026-08-22** against approved per-seller limits, with the three Blender facts riding along. **Billing was measured as account-balance differences rather than quoted rates; the personal amounts are withheld**, at the Creator's instruction of the same day: *"the prices you see via the API are not final."*

---

## The machines

**Nine rentals, three sellers, and one of the nine never started.**

| | Seller | Instance | What it was | Kind | Alive (UTC) | What it was for |
|---|---|---|---|---|---|---|
| **A** | Vast.ai | `48388918`, machine `142136`, offer `44921058` | Tesla P4, New York US, 4 cores, 16 GiB, 60 GB disk, `ubuntu:24.04` | container | 11:07 → 15:13 | group 1, the disk, elevation, survival, file identity, the four Blender facts, and the endurance run |
| **C** | Hyperstack | `998523`, flavor `n3-L40x1`, CANADA-1 | NVIDIA L40 46 068 MiB, 28 vCPU, 58 GiB, 99 GB disk, image `Debian 12 Plain` | KVM virtual machine | 11:22 → 11:38 | **the Debian driver route**, the package lock at boot, the restart, boot registration, and the disk-full test that destroyed it |
| **E-fail** | Vast.ai | `48391417`, machine `29051` | 1 of 4 GTX 1660, United Kingdom | **never started** | 11:46 → 11:49 | intended as the one-card half of the Creator's pair |
| **E** | Vast.ai | `48391785`, machine `52447`, offer `45169470` | **2 of 4** GTX 1660 SUPER, South Korea, `ubuntu:22.04` | container | 11:53 → 12:01 | the card readings inside a slice; one card against two |
| **F** | Vast.ai | `48392256`, offer `48319438`, Slovakia | **2 of 4** RTX 5090 32 607 MiB, driver 595.71.05, `ubuntu:24.04` | container | 12:00 → 12:09 | the card an artist would actually rent; OptiX against CUDA; the slice question on a second host |
| **D** | Sesterce | `sf-6a8991c2f7048d5ef8ddfec7`, `AZ_22`, NOR-Oslo-3 | RTX 5090 32 607 MiB, driver 570.195.03, 12 vCPU, 120 GiB, 97 GB + a 787 GB second disk, `ubuntu22.04_cuda12.8_shade_os` | KVM virtual machine | 12:15 → 12:20 | a third seller, unseen; group 1 on a third OpenSSH; the same card under an older driver |
| **G** | Vast.ai | `48393510`, machine `54912`, offer `43555756` | RTX 3060, Norway, `ubuntu:22.04` | container | 12:39 → 12:52 | the keepalive under saturation, and throughput both ways |
| **G2** | Vast.ai | `48394413`, **the same machine `54912` re-rented** | RTX 3060, Norway, `ubuntu:22.04` | container | 12:53 → 12:57 | whether an address or a host key comes back |
| **H** | Vast.ai | `48394483`, machine `123545` | RTX 2060, South Korea, `ubuntu:22.04` | container | 12:57 → 12:59 | whether the host keys are the image's or the host's |

**One rental never started and it is recorded rather than skipped.** Instance `48391417` was taken from a `rentable: true` offer and ended at `Error: GPU error, unable to start instance.` with SSH refusing the key throughout. It was destroyed unused. **That is the fourth rental this project has lost to a box that never comes up**, and it is why a rental is checked rather than assumed.

**And it exposed how this seller sells a slice.** Machine `29051` advertises a one-card, a two-card and a four-card offer. **Taking the one-card slice made the other two `rentable: false` immediately** — they are alternative carvings of one host, not independent rentals. So the Creator's controlled one-card/two-card pair **cannot be rented simultaneously on one host**; it is a sequential test or it is two hosts.

---

## What the three sellers turned out to be, which is itself a finding

**Machine D existed to see what an unseen seller is.** What it turned out to be is recorded here rather than left in the batch's working notes.

**Sesterce sells no Debian at all, and that lands on a promise the Creator made the same day.** Its image list, read from its own API on 2026-08-22, offers **Ubuntu and AlmaLinux in every region, and Debian in none**. The Creator's ruling of 2026-08-22 promises **Ubuntu and Debian** as the rented-server Linux boundary. **Nothing breaks** — the boundary is what Smoozy supports, not what every seller stocks, and this seller stocks one of the two. **But it is the first observation that a seller may not offer a family Smoozy supports**, and `Server Setup`'s machine-refusal path is what meets it. Hyperstack, by contrast, offers **Ubuntu, Debian and AlmaLinux**; Vast offers whatever image the renter names, which is why a Debian container is trivially available there and a Debian *virtual machine* is not.

**A rental is not reachable until something opens the port, on one of the three.** Hyperstack delivered machine C with a floating address and **no inbound SSH** — every login attempt timed out until a security-group rule was created through the seller's own API (`POST /core/virtual-machines/{id}/sg-rules`, `direction ingress`, `protocol tcp`, ports 22–22). **This is a setup step that exists on one seller and not the others**, it is performed through the seller's own interface rather than over the connection, and **Smoozy cannot perform it** — it has no route to a seller's control plane. **A machine that cannot be reached at all is not distinguishable, from Smoozy's side, from a machine that is down.** Vast needs nothing of the kind because it reaches the container through its own proxy; Sesterce delivered a directly reachable address.

**How long a machine takes to become usable varies by an order of magnitude.**

| Seller | requested → reachable | billed from |
|---|---|---|
| Vast.ai | **about 40 seconds** | creation |
| Hyperstack | **about 90 seconds** | creation |
| Sesterce | **about 7 minutes** (`pending` 12:10:41 → active 12:15:02, reachable shortly after) | **when it went active**, so the pending time was free |

**Anything that waits a fixed period for a machine to come up will be wrong on at least one of these three.**

**And one seller's own documentation is wrong about its own creation call.** Sesterce documents `cloudProvider` as a required field and its offers list carries a cloud with both a `name` (`AZ_22`) and an internal `_id`. **Passing the name returns `500 Unexpected error occured`; passing the `_id` succeeds.** The documentation names neither the field's expected form nor the id. **Recorded because Rule 2's readable-source route assumes a provider's own documentation is right about its own API**, and here it was not — the working value was found by trying, not by reading.

---

## Group 1 — the surface everything rests on, and the part Smoozy leans on did not vary

Run against **three machines on three sellers**: **A** (Vast container, Ubuntu 24.04, OpenSSH `9.6p1 Ubuntu-3ubuntu13.18`, root login), **C** (Hyperstack virtual machine, Debian 12, OpenSSH `9.2p1 Debian-2+deb12u4`, non-root login) and **D** (Sesterce virtual machine, Ubuntu 22.04, OpenSSH **`8.9p1 Ubuntu-3ubuntu0.13`**, non-root login with passwordless elevation). Three sellers, two operating-system families, two machine kinds, two login kinds, **three OpenSSH versions**.

**Every load-bearing reading was identical on all three. One reading was not, and it is the batch's second real surface finding.**

| Reading | A, C and D | What was at stake |
|---|---|---|
| Protocol version announced | **3** | [1.1](Server%20protocol%20and%20transfers.md) assumes 3 |
| Extensions announced | **11 on A and C, 8 on D** | see below — the one reading that varied |
| `limits@openssh.com` max-packet | **262 144** | vs the 32 768 fallback |
| max-read / max-write | **261 120** each | R5's chunk bound |
| max-open-handles | **1 019** | — |
| Exclusive create over an existing path | **refused** (`Failure`) | `Rented-server runtime` G1b |
| Plain create over the same path — the control | **succeeded** | proves the refusal is exclusivity, not permission |
| Base rename over an existing name | **refused** (`Failure`) | why publication needs the extension |
| `posix-rename@openssh.com` over an existing name | **succeeded**, target held the source bytes | 7.5's publication step |
| `fsync@openssh.com` on an open handle | **succeeded** | durability |
| `statvfs@openssh.com` | **succeeded**, `f_namemax` **255** | the free-space reading |
| Filename round-trip, 15 forms | **15 of 15**, zero failures | admissibility |
| Case sensitivity | **case-sensitive** on all three | §12's recorded contradiction |
| Name length | **255 accepted, 256 refused** `Bad message` | — |
| Concurrent sessions on one connection | **capped at 10** | T5's lease |
| Parallel connections | **6 opened, no refusal** | the additional transfer connection |

### The extension set does vary, and the variance is in extensions Smoozy does not use

**A and C, both on OpenSSH 9.x, announced eleven** and in this order: `posix-rename@openssh.com`, `statvfs@openssh.com`, `fstatvfs@openssh.com`, `hardlink@openssh.com`, `fsync@openssh.com`, `lsetstat@openssh.com`, `limits@openssh.com`, `expand-path@openssh.com`, `copy-data`, `home-directory`, `users-groups-by-id@openssh.com`.

**D, on OpenSSH 8.9p1, announced eight** — the first eight of that list. **Missing: `copy-data`, `home-directory` and `users-groups-by-id@openssh.com`.**

**So capability negotiation is load-bearing rather than a courtesy, and this is the observation that proves it.** Every extension Smoozy actually leans on — `posix-rename` for publication, `statvfs` for free space, `fsync` for durability, `limits` for the chunk bound — was present on all three, and **the announced limits were byte-identical across all three** despite the version gap. **What varies is the tail, and Smoozy uses none of it.** An older but entirely ordinary rental is a real case rather than a hypothetical, and the rule that a capability not announced is never attempted is what makes that safe.

**A negative would have been:** a box announcing no `posix-rename`, or limits below the 32 768 the specification recommends, or a session cap of one. **None occurred on any of the three.** The **32 768 fallback branch of [1.2](Server%20protocol%20and%20transfers.md#12-capability-negotiation-and-it-is-load-bearing-rather-than-a-courtesy) item 2 has now been unreached on four boxes across three sellers**, and remains untested rather than disproved.

**The filenames that round-tripped**, written over the subsystem and read back byte-for-byte, then listed from the machine's own shell with `ls -b`: a newline (`a\nb.txt`), a carriage return, a tab, a leading dash (`-rf.txt`), a space, **a non-UTF-8 byte sequence** (`bad\377\376.txt`), a 250-character name, a name differing from another only in capitals, a double quote and an apostrophe together, a backslash, an asterisk, a semicolon, a dollar sign, a percent-and-format-specifier string (`pc%%s%d.txt`), and an emoji outside the basic plane. **All fifteen landed, listed and read back.** That is the evidence under the pre-hand-over admissibility rule's *values are never syntax*.

**One thing this exposed about a client rather than a server.** A directory listing that decodes entry names as UTF-8 **raises** on the non-UTF-8 name — `UnicodeDecodeError('utf-8', b'bad\xff\xfe.txt', 3, 4, 'invalid start byte')` — while the write, the read-back and the machine's own `ls` all handled it. **So the enumeration must carry names as bytes and decode them only for display**; a listing that decodes eagerly fails on a name Smoozy is otherwise able to move. This is a constraint on `File Browser`'s walk, observed rather than reasoned.

### Test 4 — a handle does not survive its channel, and the refusal is the handle's

Posed with **both channels opened before anything was closed**, so that closing one could not be confused with the connection going away.

- The handle read correctly on its own channel: `b'HANDLEDATA'`.
- The same handle on a **second channel of the same connection, already open**: refused, `Failure`.
- After the first channel was closed, the second channel: refused, `Failure`.
- A **freshly opened** third channel: refused, `Failure`.
- **The control**: the second channel then served `stat` on the same path and returned size 21. **So the channel was alive and it was the handle that was refused.**

**A negative would have been the reopened channel accepting the old handle.** It did not. This is the observation behind *an operation holding an open handle when the lease changes resumes by re-opening, and never by re-using* — which was written from reasoning and is now observed on a rented machine.

### Test 6 — the programs are a property of the image, and two absences are load-bearing

Machine A, `ubuntu:24.04` as the seller ships it, under the login Smoozy would run as.

**Present:** `sudo`, `su`, `flock`, `tar`, `gzip`, `curl`, `wget`, `systemd-detect-virt`, `nvidia-smi`, `lsblk`, `df`, `stat`, `sha256sum`, `md5sum`, `python3` (3.12.3), `systemctl`, `service`, `rsync`, `scp`, `sftp`, `ssh`, `sshd`, `tmux`, `nohup`, `setsid`, `ps`, `pkill`, `dpkg`, `apt`, `apt-get`, `getent`, `stdbuf`, `timeout`, `nice`, `ionice`, `truncate`, `fallocate`, `dd`, `sync`, `ln`, `readlink`, `realpath`, `mktemp`, `find`, `xargs`, `loginctl`.

**Absent:** **`lspci`**, **`cron` and `crontab`**, `fuser`, `lsof`, `xz`, `zstd`, `bzip2`, `unzip`, `zip`, `aria2c`, `screen`, `needrestart`, `at`.

**Two of those absences are not cosmetic.**

- **`lspci` is absent, and [Servers](Servers.md) H6 uses `lspci -d 10de:` for card presence.** It was absent on **every Vast container in this batch** — A, E and F — and present on the Hyperstack virtual machine. On a container it is not there, so the presence reading has no route unless something installs `pciutils` first.
- **`cron` and `crontab` are absent, and there is no service manager either** — `systemctl is-system-running` returns `offline` and PID 1 is `/bin/sh -c while [ ! -e /.launch ]; do sleep 1; done; bash /.launch`. **So on this container kind neither boot-registration route the architecture names exists as shipped.** What does exist is [below](#test-18--the-boot-registration-routes-are-not-the-same-on-the-two-kinds).

**And a third absence cost real time.** Blender ships as `.tar.xz` and **`xz` is absent**, so the archive cannot be unpacked until `xz-utils` is installed. Blender also **links libraries the image does not carry** — the binary failed with `error while loading shared libraries: libX11.so.6: cannot open shared object file: No such file or directory` until `libx11-6 libxi6 libxxf86vm1 libxfixes3 libxrender1 libgl1 libegl1 libsm6 libice6 libxkbcommon0` were installed, which took **17 seconds**. This is the same per-image spread the earlier batch recorded, met again: **the list is a property of the image and cannot be fixed in advance.**

---

## Group 2 — the disk, and the one result that would have mattered most

### Test 7 — the free-space reading describes the layer the bytes land on

Machine A, a container with a 60 GB allocation on a host holding much more.

`statvfs@openssh.com` on the job path returned `f_frsize` 4096, `f_blocks` 15 728 640, `f_bavail` 15 728 451 — **64 424 509 440 bytes total, which is exactly the 60 GiB rented**, not the host's 224 GB `/dev/sda3` nor the 32 GB `/dev/sda2`. The machine's own `df -B1 /root` agreed to the byte: `overlay 64424509440`. Root is `overlay` with `xino=off`; the quota is enforced by the host's XFS project quota (`prjquota` in the mount options).

**A negative would have been the reading describing a filesystem the job's bytes do not land on.** It did not. **Not posed:** whether a container whose writable layer is *not* quota-backed reports the host's disk — this box's was.

### Test 8 — the disk filled, and the two routes did opposite things

Machine **C**, deliberately, as the last work before it was destroyed. The 99 GB filesystem was filled with `fallocate` to leave about 41 MiB, and a **209 715 200-byte** file was pushed into it by both routes.

**Route 1 — Smoozy's own byte loop into a staging name Smoozy chose:**
- the write raised `Failure` after 42 823 680 bytes had been handed to it;
- the staged file held **41 930 752 bytes** at `route1.bin.smoozy-diskfull.part`;
- **the final name was absent.**

**Route 2 — the standard tool writing straight at the final name:**
- `scp` exited **1** and printed `scp: write remote "/home/debian/dfull/route2.bin": Failure` and `scp: failed to upload file … to …`;
- and it left **`route2.bin`, 261 120 bytes of 209 715 200, at the final name.**

**That is Row 281's exact failure arriving by the route that looks safest**, observed rather than argued: a partial file sitting at the name a user would take for a finished one. It is the strongest single piece of evidence in the batch for [7.5](Server%20protocol%20and%20transfers.md#75-publication-is-the-last-step-and-it-is-one-step) and for S5's *no tool ever chooses the staging name*.

**Two further readings from the same moment.**
- **`statvfs` read `0` free**, and `df` read `100%`. The free-space reading is honest at the boundary.
- **The machine's own log did say something**, unlike Fact 72's corruption case where it said nothing: `sftp-server[1053]: error: process_write: write "…route1.bin.smoozy-diskfull.part": No space left on device`, and the same line for route 2. **So a full disk is logged and damage in the pipe is not** — the two are not equally silent, and nothing should be built on the assumption that they are.

### A rental with two real block devices, which the register said still wanted hardware

**Machine D answered Fact 28's outstanding clause without being asked to.** That clause — *a box with more than one physical disk*, as distinct from the attached-volume answer — is listed in the register as still wanting hardware. **Sesterce delivers it as standard.**

```
/dev/vda1        97G   15G   82G  16% /            ext4
/dev/vda15      105M  6.1M   99M   6% /boot/efi    vfat
/dev/vdb        787G   28K  747G   1% /ephemeral   ext4
```

**Two separate block devices, not two partitions of one** — `vda` and `vdb`, different virtio devices, and the second is **eight times the size of the root filesystem** and empty on delivery. This is the shape the earlier volume box could not pose: that box's 20 GiB `/vol` turned out to be a quota on a slice of the same physical disk (`sda4`, a partition of the same `sda`), and this document recorded so.

**What it means for the placements.** Smoozy hands over one chosen root and creates its areas under it. On a machine like this, **the obvious root is the small one** — `/` is 97 GB and `/ephemeral` is 787 GB — so a job's folder placed by convention rather than by a free-space reading lands on the wrong disk by a factor of eight. **The free-space reading is what makes the right choice available**, and it reports per-mount correctly here.

**Its name is a warning in itself.** The mount is called `/ephemeral`, and **nothing was tested about whether it survives anything** — no stop, no start, no restart. **Not posed**, and the name is the seller's own claim rather than an observation. A placement decision that put a job's outputs there on size alone would be resting on an untested word.

### Test 9 — where the writable space is, on a container

`/` is `overlay`. `/dev/shm` is a 7 GiB `tmpfs`. `/etc/hosts`, `/etc/hostname` and `/etc/resolv.conf` are **individually bind-mounted from the host's `/dev/sda3`**, and `/usr/bin/nvidia-smi` and every NVIDIA library are **bind-mounted read-only from the host's `/dev/sda2`** — which is how the driver arrives without the image carrying it. **Writing to a path under one of those bind mounts would not be on the rented layer.** The survival half of this test was not posed on A; it was posed on the earlier volume box and is recorded there.

---

## Group 3 — setting a machine up

### Test 10 — what the detector says, and one machine the closed table has no row for

| | `systemd-detect-virt` | `-c` (container) | `-v` (vm) | `/.dockerenv` | PID 1 |
|---|---|---|---|---|---|
| **A** (Vast container) | `docker` | `docker` | **`vmware`** | present | `sh` |
| **C** (Hyperstack) | `kvm` | `none` | `kvm` | absent | `systemd` |

**Machine A is a container inside a VMware virtual machine, and both halves answer.** `systemd-detect-virt` alone says `docker`; `-v` says `vmware`; the DMI strings say `VMware, Inc.` / `VMware Virtual Platform` and the CPU carries the hypervisor flag. **A closed table keyed on one reading places this box as a container and is right; a table that expects the two readings to agree, or that treats `-v` as authoritative, places it as a virtual machine and is wrong.** This nesting was not on the record before.

### Test 11 — elevation, and the three refusals are distinguishable

Posed properly on machine A by making a non-root login, because **the delivered login is already root and no elevation question can be asked of it** (`id -u` = 0, `sudo -n true` returns 0, and a wrong password cannot even be offered).

Under the non-root login `smoozytest` (in group `sudo`), with sudo 1.9.15p5, sudoers as Ubuntu ships it (`Defaults env_reset, mail_badpass, secure_path, use_pty`; `%sudo ALL=(ALL:ALL) ALL`):

- **Right password**, `sudo -S -v`: prompt `[sudo] password for smoozytest:`, **exit 0**.
- **Wrong password**, word for word: `[sudo] password for smoozytest: Sorry, try again.` then `[sudo] password for smoozytest: ` then `sudo: no password was provided` then **`sudo: 1 incorrect password attempt`**, exit 1.
- **No password at all**, `sudo -n -v`: **`sudo: a password is required`**, exit 1.
- **Not an administrator** (`smoozynoadmin`, not in the group), right password offered: **`Sorry, user smoozynoadmin may not run sudo on a0e7a496ff47.`**, exit 1.

**A negative would have been a refusal indistinguishable from a wrong password.** It was not: the three differ in their exact text, and their digests differ. **This is the observation Fact 108's separable-authentication result needed and had not had on a rented machine** — [Security and licence](Security%20and%20licence.md) says outright that no fact in the register covered it.

**Two things that come with it.** The credential cache is **15 minutes** (`sudo -V`: *Authentication timestamp timeout: 15.0 minutes*), so one authentication does carry across later commands within that window — `sudo -n id -u` returned `0` after a successful validate. And the not-an-administrator message **names the host**, so it is per-machine text and must not be matched as a constant.

### Test 12 — the package lock at boot

**Machine C, at 108 seconds of uptime**, which is the race the architecture had never posed. `apt-get -o DPkg::Lock::Timeout=120 update` returned **0 in 31 seconds**, fetching 26.7 MB. `fuser /var/lib/dpkg/lock-frontend` reported the lock **not held** at that moment.

**A negative would have been the documented sequence failing to clear the lock, or waiting longer than a setup can reasonably wait.** Neither happened on this box. **What this does not establish** is the loaded case: the timers were enabled and one was 17 minutes away, so the run did not collide with a job actually holding the lock. The `DPkg::Lock::Timeout` route was exercised; the collision it exists for was not.

### Test 13 — the Debian driver route exists, and it is not the Ubuntu one

**This was the test named as able to stop the stage.** [Server Setup](Server%20Setup.md) §13 item 22 records that the Ubuntu route uses Canonical's own tool, that it does not exist on Debian, and that nothing establishes an equivalent; the Creator's ruling of 2026-08-22 promises both families. **A route exists. The stage is not stopped.**

**Machine C, `Debian 12 Plain` as Hyperstack ships it, an L40 with no driver: `nvidia-smi` absent, zero NVIDIA modules loaded, `dkms` absent.**

1. **`ubuntu-drivers` is absent and Debian does not package it.** `apt-cache policy ubuntu-drivers-common` returned nothing at all. Confirmed rather than assumed.
2. **The image enables only `main`.** `/etc/apt/sources.list.d/debian.sources` reads `Components: main` for `bookworm`, `bookworm-updates` and `bookworm-backports`, and `bookworm-security`. **So `apt-cache policy nvidia-driver` returned `Candidate: (none)`** — the driver is not merely uninstalled, it is unreachable.
3. **Enabling `contrib non-free non-free-firmware` and updating makes it reachable**: `nvidia-driver` candidate `535.261.03-1` from `bookworm/non-free`.
4. **Debian's own equivalent of Canonical's tool is `nvidia-detect`**, in `non-free`, candidate `535.261.03-1`. Run against this card it printed:

   > `Detected NVIDIA GPUs:` / `00:07.0 3D controller [0302]: NVIDIA Corporation AD102GL [L40] [10de:26b5] (rev a1)` / `Checking card:  NVIDIA Corporation AD102GL [L40] (rev a1)` / `Your card is supported by the default drivers.` / `Your card is also supported by the NVIDIA Linux Open GPU Kernel Module.` / `It is recommended to install the` / `    nvidia-driver` / `package.`

   **It names the package to install, which is exactly what the Ubuntu tool is used for.** That is the equivalent §13 item 22 says nothing establishes.
5. **A trap that is not in any manual and cost the run its first attempt.** The running kernel was `6.1.0-31-amd64` and **`linux-headers-6.1.0-31-amd64` is not in the archive** — `E: Unable to locate package`. Debian removes superseded kernel headers, so **the headers for the kernel a stale image booted may simply not exist**. Installing `nvidia-driver` resolved it by pulling a newer kernel: DKMS built against **`6.1.0-52-amd64`**, not against the running one.
6. **So a restart is required, and until it happens the route looks broken.** `nvidia-smi` after install: `NVIDIA-SMI has failed because it couldn't communicate with the NVIDIA driver.` `modprobe nvidia`: `FATAL: Module nvidia-current not found in directory /lib/modules/6.1.0-31-amd64`.
7. **After the restart it works.** Kernel `6.1.0-52-amd64`; `nvidia-smi` reports driver `535.261.03`, `NVIDIA L40`, `46068 MiB`; `nvidia_uvm`, `nvidia_drm`, `nvidia_modeset` and `nvidia` all loaded.

**Timing: 149 seconds for the driver install, plus the restart. Bare card to a working card query in about four minutes plus one reboot, with nothing asked of a human.**

**A negative would have been no route that ends in a working card query.** There is one. **What it costs the architecture is the restart**, which is why the whole-app boundary's carve-out matters here rather than being theoretical — see Test 17.

### Test 14 — Debian does carry the automatic-updates job

[Server Setup](Server%20Setup.md) §13 item 23 records that it is Ubuntu's own default and that Debian is not established to share it. **On this image it does.**

`dpkg -l unattended-upgrades` → `ii  unattended-upgrades 2.9.1+nmu3`. `/etc/apt/apt.conf.d/20auto-upgrades` reads `APT::Periodic::Update-Package-Lists "1";` and `APT::Periodic::Unattended-Upgrade "1";`. `apt-daily.timer` and `apt-daily-upgrade.timer` are both **enabled**, and `apt-daily-upgrade.timer` was **17 minutes from firing** when read.

**Stated exactly, because the distinction has bitten this project before: this is a property of the `Debian 12 Plain` image Hyperstack ships, not a property of Debian.** The same caution the earlier batch drew about the lingering permission applies — it is one image on one seller, and a setup step should read it rather than assume it either way. **Machine A's `ubuntu:24.04` container carried the opposite**: `unattended-upgrades` not installed (`un … <none>`), no `20auto-upgrades` file, no timers, no service manager to run them.

### Test 15 — installing a Blender, timed

Machine A, after `xz-utils` and the link libraries were in place.

| | 4.5.12 | 5.2.0 |
|---|---|---|
| download | **5 s**, 377 902 364 bytes | **4 s**, 384 441 228 bytes |
| unpack (`tar xf`, single-threaded xz, 4 cores) | **62 s** | **67 s** |

**The unpack dominates the download by an order of magnitude on a 4-core box.** On machine F (Slovakia, more cores) both archives downloaded **in parallel** and both unpacked in **34 seconds total**. So the cost is the machine's processor rather than its link, which is the opposite of what a transfer-shaped assumption would predict.

---

## Group 4 — survival

### Test 16 — a program outlives the connection, on the container kind

Machine A. A shell loop was started `setsid nohup … </dev/null >/dev/null 2>&1 &`, giving PID 855 as its own session leader (`sid == pid`). The connection was closed.

**On a new connection**: PID 855 still running, **reparented to PID 1** (`PPID 1`), `ELAPSED 00:18`, its log still being appended. `kill -TERM 855` was accepted, the process logged `SIGTERM received, ending cleanly` and exited within two seconds.

**A negative would have been the program dying with the session on the container kind, which is where it was most likely.** It did not. **This is the shape fact at the promise level rather than the throwaway level**, on a real rented container.

### Test 17 — the machine restarts over the connection Smoozy already holds

Machine C, `sudo systemctl reboot` issued over the SSH connection already open. **The command returned 0 and the connection dropped.**

- **Back in 49 seconds**, and on a second restart later the same box came back in **55 seconds**. Both inside the 9-to-90-second range already on record, which is why nothing budgets a fixed wait.
- **The address was the same**: `185.216.22.240`.
- **All three host keys were the same**, byte for byte, before and after: `SHA256:1IVT2IeXv+1KoL409JbzwVyHJYLOEfBNi4jeQw6IjyQ` (ed25519), `SHA256:n6JwIgzHwfMSY0fGkSfjAApe28xhJ8S/F/12DYGtveg` (RSA), `SHA256:onfu4Kbg0woN6KGZH7nEzB+7qiKW2WtmPkYHi+AXJFE` (ECDSA).

**So the exception the whole-app boundary carves out is reachable on this seller**, and Test 13's route needs it. **Not posed:** the same question on a container, where there is no service manager to ask.

### Test 18 — the boot-registration routes are not the same on the two kinds

**On the virtual machine (C), the service-manager route works.** A unit was written to `/etc/systemd/system/smoozy-probe.service` with `Restart=always` and `WantedBy=multi-user.target`, enabled and started. After the restart: **`enabled`, `active`, new MainPID 874 with PPID 1**, and its log shows the shutdown delivered `SIGTERM` cleanly before the reboot and the new instance starting 50 seconds later. **The plain detached process from Test 16 did not survive**, correctly — `old pid 1369: gone`.

**On the container (A) neither named route exists as shipped.** No service manager (`systemctl is-system-running` → `offline`), no `cron`, no `crontab`, no `at`. `/etc/rc.local` exists but nothing runs it. **What does exist is the seller's own hook**: PID 1 is a shell waiting on `/.launch`, and `/.launch` runs the container's provisioning and then `/root/onstart.sh`, which is the file the rental request's `onstart` field writes. **So on this kind the boot registration is the provider's own mechanism, which is per-seller and outside Smoozy's control**, rather than either route the architecture names. That is a gap between the two kinds, not a detail.

### Test 19 — the address is not handed to a different machine, but the host key is

**Posed on machine G** (Vast container, machine `54912`, Norway, `ubuntu:22.04`, offer `43555756`) by taking it, recording everything, destroying it, and **renting the same offer again**.

| | first rental (`48393510`) | second rental (`48394413`) |
|---|---|---|
| proxy address | `ssh3.vast.ai` | `ssh3.vast.ai` |
| proxy port | **33510** | **34412** |
| public address | `178.164.79.201` | `178.164.79.201` |
| RSA host key | `SHA256:zXD6ZHh/Z9Ohwz5bX0giDWoKRs3yryvaFNg1jFE7MRI` | **identical** |
| ECDSA | `SHA256:9GkF22snuRylDEtK6wAxvKV9NhqMrpIJW35MLCpojOo` | **identical** |
| ED25519 | `SHA256:jH2eaZgwf6L6pWGTbq2s36OMmuCPk22H7dCSm4Wmzz0` | **identical** |

**The old port stopped answering** the moment the first rental was destroyed — `ssh-keyscan` on `ssh3.vast.ai:33510` returned nothing. **The port moves with the contract**, so an address-and-port pair is not reused inside the window this test could watch.

**The test's stated negative was the same address and port answering with a different host key. What happened is the mirror of it: a different rental answering with the same host keys.**

**Whose keys they are, established rather than guessed.** Inside the second rental, `/etc/ssh/ssh_host_*_key.pub` held exactly those three fingerprints, with the comment **`root@buildkitsandbox`** — the image build sandbox rather than the machine. **A third rental was then taken to separate the two explanations**: machine `123545`, a **different machine in a different country reached through a different proxy node**, with the **same `ubuntu:22.04` image** — and it presented **three different keys** (`SHA256:QYq6wyHL3xCvgRgJVG5A+IengHFQgd3zPil8nYCWJ9c`, `SHA256:R3qPwjHXxMWPnJ/ue/u+m4vXxoqVpEFdaqqTGYuQYcE`, `SHA256:SJxlufOuz88AWXSzMriSJGfRtIs+DRDaUr7pPJNaEXA`).

**So the keys are neither the proxy's nor the image's: they are generated once on the host and reused by whatever the host rents out next.** They survive the destruction of the rental that first saw them.

**What this means for the architecture, and it is a vindication rather than a problem.** **A host key cannot distinguish *my machine, still mine* from *the same machine, re-rented after mine was destroyed*.** Smoozy's reconnection is gated on the installation identity rather than on reachability or on the key, and this is the observation that gate was written for — *"Nobody has ever watched it happen"* is no longer true of the half that could be watched cheaply. **Not posed:** whether the seller ever hands the same proxy port to a different machine, which would need a much longer watch than this batch could run.

### Test 29 — a card's identity and its position survive a restart

Machine C, across the reboot: `0, NVIDIA L40, GPU-36e29c4f-d527-a4d7-01bf-5d42626be0e7, 00000000:00:07.0` — index, UUID and bus location **all unchanged**. **A negative would have been the ordering changing.** It did not, on one card.

---

## Group 5 — the link and the clock

### Test 20 — the keepalive cadence produced no false break under a saturated link

**This is Fact 111.** Posed on machine **G** (Vast container, Norway) with the connection carrying **the interval `Servers` K3 chose — 15 seconds** — while a **419 430 400-byte** file was moved at the link's full speed in both directions, with a watcher checking every second whether the transport had been declared dead while bytes were still moving.

- **Upload: 680.52 s.** **Download: 108.07 s.** **A second full download immediately after: 97.28 s.**
- **`connection still active after all of it: True`.**
- **`false breaks reported while bytes were moving: NONE`.**

**A negative would have been a break declared on a healthy connection under load**, which would tear down a live connection and start a repair against a machine that never went away. **It did not happen across eleven minutes of continuous saturated transfer.** The two figures `Servers` K3 chose stand observed rather than assumed.

### Test 21 — throughput, and the constraint is the user's own link

Same box, same run.

| | bytes | seconds | rate |
|---|---|---|---|
| up | 419 430 400 | 680.52 | **0.62 MB/s** |
| down | 419 430 400 | 108.07 | **3.88 MB/s** |
| down again, straight after | 419 430 400 | 97.28 | **4.31 MB/s** |

**The round-trip digest matched the source exactly** (`bb1351f08f28c505…`), so the 400 MiB went up and came back byte-identical.

**Two things follow, and the first is the one that matters for every timing estimate in the architecture.** **The upload was six times slower than the download, and the ceiling is the desktop's uplink rather than the rented machine's link** — the seller advertised 580 Mbit up on this host, and 0.62 MB/s is about 5 Mbit. The endurance run was drawing its throttled 384 KB/s from the same uplink at the same time, which accounts for part of it but not most. **Anything that budgets an upload from the machine's advertised figure will be wrong by an order of magnitude.**

**And nothing throttles.** The second download was **faster** than the first, not slower, so no shaping appeared after several hundred megabytes. **A negative would have been a visible drop on the repeat.** There was none.

### Test 22 — the clock

Machine A, a container: **no time daemon at all** (`chronyd`, `ntpd`, `systemd-timesyncd` all absent), `timedatectl` unusable because there is no service manager. The clock nonetheless read correctly to the second against the client, clocksource `tsc`, timezone `Etc/UTC`. **It is the host's clock seen through the container**, which is why no daemon is needed and also why nothing inside can correct it.

Machine C, a virtual machine: `systemd-timesyncd` **active**, `System clock synchronized: yes`, `NTP service: active`, RTC in UTC. Across a reboot the clock stayed correct.

**A negative would have been a clock wrong at boot or drifting over the run.** Neither was seen. **The container case is the one to note**: the four-value file identity and every stamped record depend on a clock that, on that kind, nothing on the machine is maintaining.

---

## Group 6 — file identity

### Test 23 — the four values behave as the fast path assumes, on `overlay`

Machine A, on the rental's own overlay filesystem:

| Action | inode | size | mtime | ctime |
|---|---|---|---|---|
| file created | 135417817 | 6 | 1787397173 | 1787397173 |
| **renamed** | **135417817** (same) | 6 | 1787397173 (same) | **1787397175** (moved) |
| copied to a new name | **135417818** (new) | 6 | 1787397175 | 1787397175 |
| **modification time set back** to 2001-01-01 | 135417817 | 6 | **978307200** | **1787397175** (did not move back) |

**A rename keeps the inode and moves the change time. A hand-restored modification time does not move the change time back.** That is exactly what S13 leans on, now observed on an overlay filesystem rather than an ordinary one — the case the test called *the least predictable*.

### Test 24 — the filesystem is case-sensitive, and the recorded contradiction resolves one way

Two clauses in [Dependency Upload & Remap](Dependency%20Upload%20&%20Remap.md) say opposite things about this and neither carries a citation.

**On both machines, by both routes:** `casefile.txt` and `CASEFILE.TXT` written into one directory held **different contents** and listed as **two entries**; `stat` on `CaSeFiLe.TxT` returned not-found; and on the machine's own shell the directory listed both names. **Both rented filesystems are case-sensitive and neither folds.** So a folding directory returning the name it was given **was not posed**, because no folding directory was found to pose it on — which is stated rather than counted as an answer.

---

## Group 7 — more than one card, and the machine is not the machine

The Creator's own addition of 2026-08-22. The premise it was written from is right: **on this seller a rental is a slice of a host, and every card rule assumes the cards Smoozy can see are the cards Smoozy has.**

Posed on **two different hosts**, each holding **four** cards, each rented as a **two-card** slice.

| Reading | **E** — 2 of 4 GTX 1660 SUPER | **F** — 2 of 4 RTX 5090 |
|---|---|---|
| `nvidia-smi -L` | **2** — ours | **2** — ours |
| `nvidia-smi --query-gpu=index` | **2** | **2** |
| `/proc/driver/nvidia/gpus` | **4** — two UUIDs not ours | **4** — two UUIDs not ours |
| `/dev/nvidia*` device nodes | **4** (`nvidia0`–`nvidia3`) | **2** (`nvidia0`, `nvidia2`) |
| `nproc` | **40** — the whole host | 16 |
| memory | **64 191 MiB** — the whole host | — |
| `lspci` | **absent** | **absent** |
| cgroup device list | not readable | not readable |

**Three findings, and they are the point of the group.**

1. **`nvidia-smi` was honest on both boxes and nothing below it was.** On E the query returned the two cards at `04:00.0` and `84:00.0`, while `/proc/driver/nvidia/gpus` listed those two **plus** `03:00.0` (`GPU-bac235ec-…`) and `85:00.0` (`GPU-c93948c8-…`), which belong to another tenant. On F the query returned `e1:00.0` and `f1:00.0`, while `/proc` listed those plus `01:00.0` and `11:00.0`. **A card count taken below `nvidia-smi` counts the host's cards.** This project's own machine-A probe used `ls /proc/driver/nvidia/gpus | wc -l`, which agreed there only because that box had one card and one tenant.

2. **The device-node route is inconsistent between hosts and its numbering is not the query's.** E exposed four nodes for a two-card rental; F exposed two. And on F the kernel's `Device Minor` values for **our** cards were **2** and **0** — so `/dev/nvidia0` and `/dev/nvidia2` are ours, and the minor number has no relationship to `nvidia-smi`'s index 0 and 1. **Anything that identifies a card by device-node number, or that assumes node *n* is index *n*, is wrong on this box.**

3. **The processor and memory readings are the host's, not the slice's.** E was sold as a 20-core slice and `nproc` reported **40**, with `/proc/cpuinfo` listing 40 processors and `free` reporting the host's 64 GiB. **No cgroup quota was readable** to correct it (`cpu.cfs_quota_us` and `memory.limit_in_bytes` both empty — the host is on cgroup v2 and the v1 paths do not exist). So a reading of *how much machine do I have* taken the obvious way is the host's answer, and nothing inside the rental contradicts it.

**What [Servers](Servers.md) H5 and H6 already chose turns out to be the only pair that survives this**: identity from `nvidia-smi`'s UUID and bus location, presence from `lspci`. The first is vindicated. **The second has no route on a Vast container, where `lspci` is absent on all three boxes.**

### Test 26 — the memory reading, and what it did and did not show

On both E and F, `memory.used` was read three times at ten-second intervals with nothing running: **E held steady at 1 MiB on both cards; F held steady at 2 MiB on both.** `--query-compute-apps` returned an empty list on both.

**A negative would have been used memory moving while Smoozy was doing nothing.** It did not move. **But this is weaker than it looks and is stated as such:** the cards `nvidia-smi` shows are the two we hold, so a neighbouring tenant's work on the host's *other* two cards could not have appeared in this reading whether it existed or not. **What is established is that the two cards we held reported only our own use. Whether a card can ever be shared under us was not posed and this test could not pose it.**

### Test 27 — Blender enlists no card at all unless it is told to

**Observed on both E and F, and on both Blender versions.** From `--factory-startup`, `bpy.context.preferences.addons["cycles"].preferences.compute_device_type` arrives as **`NONE`**, and a Cycles render therefore runs **on the processor**.

The individual devices arrive with `use = True` — both CUDA entries and both OptiX entries — **so a reading of the device list alone suggests every card is enabled, and it is the backend selector that is off.** On machine E a 480×270, 400-sample render took **12.52 s** with the settings as they arrive and **3.01 s** once `compute_device_type` was set to `CUDA` and one card selected.

**[Rendering](Rendering.md) Row 8 promises every card on the machine. A headless render with the settings as they arrive uses none of them.** That is the negative the test was written to find, and it lands on the same ground as Fact 114: the device choice lives in the preferences rather than in the file.

**One more thing that must not be read as a capability list.** `preferences.get_device_types()` returned `['NONE', 'CUDA', 'OPTIX', 'HIP', 'ONEAPI']` on machines with NVIDIA cards and no AMD hardware, and Blender 5.2 printed `WARNING HIPEW initialization failed: Error opening HIP dynamic library` on the same box. **The list is what the build was compiled with, not what works on the machine.**

### Test 28 — the second card buys time, and the measurement had to be made honestly

**A first attempt measured nothing** — 480×270 at 400 samples gave 3.01 s on one card and 2.76 s on two, a difference swallowed by fixed costs. **That is recorded because it is the shape of a false negative**: at that size the answer would have read as *two cards take as long as one*, which is this test's stated negative, and it would have been wrong.

**Machine E, 960×540, 3000 samples, run twice each:**

| | run 1 | run 2 |
|---|---|---|
| one GTX 1660 SUPER | 27.19 s | 27.37 s |
| two GTX 1660 SUPER | 15.97 s | 16.05 s |

**1.70×.**

**Machine F, 1920×1080, 3000 samples, six subdivided meshes with a metallic material:**

| | Blender 4.5.12 | Blender 5.2.0 |
|---|---|---|
| CUDA, one RTX 5090 | 22.03 s | 21.82 s |
| CUDA, two RTX 5090 | **12.30 s** | **12.31 s** |
| OptiX, one RTX 5090 | 16.70 s | 16.51 s |
| OptiX, two RTX 5090 | **9.23 s** | **9.44 s** |

**So: the second card is worth about 1.8× on this scene, OptiX is worth about 1.3× over CUDA, and the two Blender versions agree to within a few per cent on every cell.** The allotment promise means something. **What is not established** is how any of this behaves on a scene that does not fit in one card's memory, which is where a second card stops being a straightforward multiplier.

**And the card the Creator asked about specifically behaves.** Both Blender versions drove a Blackwell RTX 5090 on driver `595.71.05` with **no CUDA toolkit installed** — only the driver the seller injects and the unpacked Blender — on both backends, at both card counts, with no failure of any kind.

---

## The endurance run — the Creator's own

**Machine A, from 11:16 UTC.** A **4 294 967 296-byte** fixture, built by the same deterministic algorithm on the Mac and on the machine and confirmed byte-identical at both ends — `sha256 8a60a4307867cd2511c525e121fc57215e61fb5321010793fa4f0f3dc813af9a`, `st_blocks × 512 == st_size` at both ends, so **allocated and not sparse**, as the run requires.

**It was moved by Smoozy's own byte loop** — a chunk bounded by the server's announced `max-write` of 261 120, written at caller-chosen offsets into a staging path Smoozy names, with a durable record updated every two seconds, resume verified by digest before a byte is added, publication by `posix-rename` as the last step. **Held near 384 KB/s deliberately**, which is what makes it hours rather than minutes, because everything below needs time to go wrong in.

### What the slow hold showed while nothing was going wrong

**Memory was flat.** Resident size settled at **64 416 KB** and did not move across more than an hour of continuous transfer — read repeatedly at one-minute intervals. **Open file descriptors held at 10** throughout. **The rate held at 384.0 KB/s to one decimal place** for the whole of that stretch. **A negative would have been a leak, a descriptor climb, or a drifting rate.** None appeared.

**One number did move, and it is explained rather than reported as a leak:** resident size rose to **95 600 KB** after a resume, because verifying a 2 GiB staged prefix reads it back through a prefetching buffer. It settled and stayed there.

### The two kinds of ending, and the record was wrong in opposite directions

**This is the most useful thing the run produced**, and it is a direct observation of what S10 says: *"The staged size read from the machine is what the bytes actually are; the record is what was accounted for. Where they differ, neither is taken on trust."*

**A hard kill — `SIGKILL` on the transfer, twice.**

| | record said | disk actually held | record was |
|---|---|---|---|
| at 37 MB | 37 552 128 | 37 847 040 | **294 912 behind** |
| at 2.14 GB (clean `SIGTERM`) | 2 140 700 672 | 2 140 798 976 | **98 304 behind** |

**The connection killed under a moving transfer** — the machine's own `sshd` session serving the transfer was killed from a second connection, at 13:02:35 UTC.

| | record said | disk actually held | record was |
|---|---|---|---|
| at 2.30 GB | 2 303 393 792 | 2 303 229 952 | **163 840 AHEAD** |

**So the record can be wrong in either direction, and which direction depends on how the transfer ended.** A kill leaves the record **behind** the disk, because bytes reached the far end after the last record write. A dropped connection leaves the record **ahead** of the disk, because **writes are pipelined** — the loop counts a chunk as written when it hands it over, and the connection died before some of those chunks landed.

**This is why the resume takes the smaller of the two and neither on its own.** Had it trusted the record after the drop, it would have written **163 840 bytes past the end of the staged file**, left a hole, and discovered it only from the end-to-end digest **after moving the remaining two gigabytes**. The `min` is not caution; it is the difference between a resume and two gigabytes of wasted transfer.

**And in every one of the three endings, the final name was absent.** Checked from the machine each time: `ls: cannot access '/root/end/up4g.bin': No such file or directory`. **Part-arrived bytes never sat at the final name.**

### Three failures of the harness itself, recorded because each is a lesson for the builder

**1. A dropped connection escaped the ending classification.** The drop arrived as `OSError: Socket is closed` raised out of the write call, propagated past the code that classifies the ending, and the process exited by traceback. **The durable record was left saying `moving`** rather than `interrupted`. `Server protocol and transfers` C23 requires the record of which ending occurred to be written **before** the channel is closed, precisely because 7.8 does opposite things for a stop and a drop — this is what it looks like when that rule is not followed: a reader cannot tell an interrupted transfer from one still in flight.

**2. Writing without pipelining costs an order of magnitude.** The first attempt ran at **99–102 KB/s against a 384 KB/s cap** and could not reach it. Each chunk waited a full round trip for its reply, so the achievable rate was one 261 120-byte chunk per round trip regardless of the cap. **With pipelining the same loop held 384.0 KB/s exactly.** R6 bounds the accounting interval; this is the separate point that **the write path must not be synchronous per chunk**, or the cap is not the binding constraint and the transfer is latency-bound instead.

**3. A cumulative rate governor makes a cap change unsafe in both directions, and this was measured twice.** The first governor accounted over the whole run — total bytes against total elapsed. Changing the cap mid-flight then settles the change against **every byte already moved**:

| governor | cap change | what the transfer actually did |
|---|---|---|
| **cumulative** | 384 → 512 | burst to about **1 000 KB/s**, roughly **double the new cap**, and stayed there for minutes |
| **cumulative** | 512 → 384 | fell to about **97 KB/s**, a **quarter of the new cap**, and stayed there |
| **five-second window** | 384 → 768 | **572.8 KB/s** across the minute the change landed in — the exact mean of old and new — then **704.0 KB/s** |
| **five-second window** | 768 → 256 | **338.1 KB/s** across the minute it landed in, then **257.1 KB/s** — the new cap |

**The burst is the serious half.** A rate cap exists to protect the user's own link, and a cumulative governor answers a raised cap by ignoring the cap entirely for a stretch proportional to how long the transfer has already run. **`Server protocol and transfers` 8.3 requires that a cap reach transfers already running. It does — but only if the accounting window is bounded, which R6 says of the consultation interval and which must also be true of the window the rate is measured over.** That is the correction this run earns: **bounding how often the budget is consulted is not the same as bounding what the rate is averaged over**, and only the second makes a cap change take effect now.

### How it ended, in both directions

**Upload, published 14:52:46 UTC.** The final stretch of the byte loop ran **5 122.8 s** at **379.7 KB/s** against a 384 cap. Then, in order:

- `fsync@openssh.com` on the staged file: **OK**.
- **source digest at rest** `8a60a4307867cd2511c525e121fc57215e61fb5321010793fa4f0f3dc813af9a`
- **pass-through digest** — the same value, computed in Smoozy's own loop as the bytes went by.
- **staged-at-rest digest** — the same value again, by reading all **4 294 967 296 bytes back over the subsystem** into Smoozy's own loop. **That read took 967.4 s**, about sixteen minutes at 4.44 MB/s.
- **the final name absent right up to publication: confirmed.**
- **published in 0.139 s** by `posix-rename`.
- **staging path gone afterwards: confirmed — the rename was the cleanup.**
- **The machine's own `sha256sum` of the published file returned the same digest**, which is a fourth independent computation by a program this project did not write.

**Download, published 15:12:11 UTC, uncapped.** 4 294 967 296 bytes in **1 066.4 s** — **4.03 MB/s**. Pass-through and staged-at-rest digests both matched, the final name was absent until publication, the rename published it, and the staging path was gone afterwards. **The file that came back is byte-identical to the one that went out**, checked on the Mac with `shasum -a 256` against the original fixture, and **no staging file was left at either end**.

**The asymmetry is the user's own link and not the machine's.** 0.62 MB/s up on the Norway box and 3.85 MB/s down here, same loop, same connection kind — about six to one. **Every duration estimate the product shows has to be direction-aware**, and an upload budgeted from a seller's advertised uplink will be wrong by an order of magnitude.

**Peak resident size across the whole run: 101 646 336 bytes on the upload and 117 178 368 on the download**, both dominated by the verification read-back rather than by the transfer.

### What a resume actually costs, which nothing had measured

**Verifying the staged prefix is read back over the wire and digested in Smoozy's own loop, and it is linear in what is already staged.**

| staged prefix | seconds | rate |
|---|---|---|
| 37 552 128 | 6.6 | 5.7 MB/s |
| 2 140 700 672 | **361.4** | 5.92 MB/s |

**Six minutes to resume a transfer that was half done.** The cost is paid on **every** resume, and it grows with progress — so a transfer interrupted near its end costs the most to resume, exactly when the least is left to send. At the observed 5.9 MB/s read-back and a 384 KB/s cap, **verifying a nearly-complete 4 GiB prefix would take about twelve minutes against a remaining transfer of seconds.**

**This is not an argument against S12** — the alternative is the unverified resume OpenSSH's own manual warns produces a file that *"is likely to be corrupt"* with no error and no marker. **It is a cost that was never written down**, and anything that shows the user a progress figure or estimates a completion time has to account for it, because during those minutes the transfer moves no bytes at all and its recorded offset does not change.

## The Blender facts

Run on machine A, in **Blender 4.5.12 LTS** (`hash 84afd5f785f7`, built 2026-07-21) and **Blender 5.2.0 LTS** (`hash fbe6228777e7`, built 2026-07-14), both `-b -noaudio --factory-startup`. **None of this went to Blender Research, per the Creator's standing instruction of 2026-08-22, and none of it is a Blender Research question**: each asks what Smoozy's own construction does to a file it wrote the launch material for, on a machine it rented.

### Fact 100 — the mark exists, and it is `library_weak_reference`

**Answered, and it is the same in both versions.**

A file was saved by each Blender from its own factory startup, touching nothing. In both, the flat path list Smoozy's enumeration reads — `bpy.utils.blend_paths()` — returned **two entries, both the same path into the application's own datafiles**:

- 4.5.12: `//../../bl/blender-4.5.12-linux-x64/4.5/datafiles/assets/brushes/essentials_brushes-gp_draw.blend`
- 5.2.0: `//../../bl/blender-5.2.0-linux-x64/5.2/datafiles/assets/brushes/essentials_brushes-gp_draw.blend`

**What identifies them as data:** each arrives as a **`library_weak_reference`** on an ordinary datablock — in both versions, on the two Materials `Dots Stroke` and `Material`, with `library_weak_reference.id_name` of `MADots Stroke` and `MAMaterial`. **`bpy.data.libraries` is empty**, and no datablock in any path-carrying collection (`images`, `libraries`, `sounds`, `fonts`, `movieclips`, `cache_files`, `volumes`, `texts`) carries that path — the only images present are the `VIEWER`-source `Render Result` and `Viewer Node`, both with empty paths.

**So the mark that separates it from every reference the user made is this: the path arrives only through `library_weak_reference` and has no datablock in any path-carrying collection.** A user's own reference always has one.

**Three sub-questions the fact asked, answered.** Which kind and property: any datablock, through `library_weak_reference.filepath`. Whether it is the library weak reference: **yes, exactly that**. Whether more than one can exist: **yes — two here, one per datablock, both naming the same file**, so a deduplicating reader sees one path and a per-datablock reader sees two. Whether the mark is stable across versions: **yes, 4.5.12 and 5.2.0 identical.**

**A negative would have been no mark that separates them**, in which case Row 171's promise could not be kept as written. **There is one. Row 171 can be kept.**

**And Blender's own report is confirmed useless here, as the fact predicted:** `bpy.ops.file.report_missing_files()` printed `Info: No missing files` in both versions, for a path that would exist on no other machine.

### Fact 99 — for the image kind, assigning the location is enough on its own

**Answered for the image kind, in both versions, by render rather than by a status flag.**

The fixture was built so that **its recorded locations were absent at load** — the folder they name was renamed away — which is the state every remapped file is in, and is the case the fact's own correction (Round 1 must-do D3) insists on. The original texture is pure red; the alternate, which exists, is pure green. Everything but the textured plane was hidden from the render, because otherwise an occluding object returns black and every reading means nothing — **which happened on the first attempt and is recorded here as the reason the isolation is there.**

| | 4.5.12 | 5.2.0 |
|---|---|---|
| as loaded, original absent | `[0.741, 0.400, 0.745]` — **the missing-texture colour** | same |
| **after assigning the new path, no reload** | `[0.145, 0.557, 0.000]` — **green** | same |
| after `image.reload()` | `[0.145, 0.557, 0.000]` | same |
| **control**: a freshly loaded datablock in the same slot | `[0.145, 0.557, 0.000]` | — |

**Assignment alone sufficed. The reload changed nothing, and the control proves the colour is what a correct read looks like.**

**One trap, and it is sharp.** Throughout all of this, `image.has_data` reported **`False`** — before the assignment, after it, and after the reload — while the render was reading the file correctly. **`has_data` is not the test of whether the render will find the file**, and anything that gates on it will report a working remap as broken.

**What was not posed, and why.** The **cache-file** kind was built into the fixture but the `CacheFile` datablock had no user after the Alembic import and was dropped on save, so it was not present at load to be tested. **Sound** was assigned and read back correctly (`strip snd … sound.filepath` updated) but a sound has no render to observe, so *is it in force for the render* cannot be asked of it. **Font**, **movie clip** and **volume** were not posed: Blender's default font is built-in and carries no path, and no external font, movie or VDB fixture was built. **Those five kinds are unanswered and are not claimed.**

### Fact 101 — a library that never loaded is populated by assign and reload

**Answered, positively, in both versions.**

At load the library was genuinely unresolved — `Warning: Unable to open '…/orig/lib.blend': No such file or directory`, `Info: Cannot find lib`, `LIB: Object: 'LinkedThing' missing`, `is_missing=True`, and the linked object present as an `EMPTY` with `data = None`. **That is the state the fact insists on: not a working library being re-pointed.**

- **After assigning the path, no reload:** `('LinkedThing', None, -1, -1)` — still nothing. **Assignment alone did not populate it.**
- **After `library.reload()`:** `('LinkedThing', 'Cone', 4, 4)` and `is_missing=False`. The mesh is the alternate library's cone, not the original's plane.

**So the library kind needs the reload and the image kind does not**, which is the per-kind difference Fact 99 was written to find.

### Fact 116 — one of the four is answered, and it answers with a silent failure first

**The four places are the bake output path, the physics point cache, the geometry-nodes bake folder, and the sequencer proxy folder.** A file was built in which **the artist gave none of them a path**, and the readings as saved were: point cache `use_external=False`, `use_disk_cache=False`, `filepath=''`, `name=''`; geometry-nodes `bake_target='PACKED'`, `bake_directory=''`; sequencer `proxy_storage='PER_STRIP'`, `proxy_dir=''`; bake output `'//'`.

**The physics point cache: answered, and the first answer was the negative.**

Assigning `point_cache.filepath` to the job folder and setting `use_disk_cache=True` — the obvious assignment — **was read back exactly as assigned**, the bake ran, **the render finished and reported success**, and the cache was written to **`blendcache_artist/` beside the `.blend`**: 250 files named `537068657265_0000NN_00.bphys`. **Nothing under the job folder but the frames.**

**That is precisely D22's observed silent case, reproduced deliberately: a render that finishes, reports success, and leaves the cache beside the `.blend`.** It is the negative this fact was written to be able to show, and the mechanism is worse than a refusal — **the value is stored and returned, and ignored.**

**What actually takes is four values together**, and with them the render honours the assignment in both versions **(corrected 2026-08-22: this said *three* and listed four; four is right, and `Job files and outputs` D21b-i is written on four)**:

```
point_cache.use_disk_cache = True
point_cache.use_external   = True        # without this, filepath is stored and ignored
point_cache.filepath       = <job folder>
point_cache.name           = <non-empty>  # the stem of every cache file
```

With those set, and the directory created first, both versions wrote `pointcache/smoozyjob_000001_00.bphys` … `_000003_00.bphys` **under the job folder and nothing beside the `.blend`.**

**The other three are NOT established, and the attempts are recorded rather than the conclusions.**

- **The geometry-nodes bake folder.** `bake_target` was set to `'DISK'` and `bake_directory` to a job path; both read back as assigned. `bpy.ops.object.geometry_node_bake_single` returned without raising for the modifier's one bake. **No bake folder appeared anywhere — not under the job folder and not beside the `.blend`** — and the saved file grew by 64 bytes. **The node group used a simulation zone that passes geometry through unchanged, and it appears to have produced nothing to bake.** So the assignment was never tested against a real bake. **What was looked at:** the modifier's own `bakes` collection, `bake_target`, `bake_directory`, and both output locations.
- **The sequencer proxy folder.** `proxy_storage` was set to `'PROJECT'` and `proxy_dir` to a job path, and both read back. **No proxy was ever built**, because the fixture has no movie strip to build one from. Untested.
- **The bake output path.** `render.bake.filepath` was assigned and read back. **No image bake was run**, so where it would have written is unobserved.

**So Fact 116 stands as: posed and answered for the physics point cache, in both versions, with the four-value condition and the silent-failure mode both demonstrated. Not established for the other three.** The general hazard the one answer exposes — **a location that is stored, returned on read, and ignored at write time** — is the thing the other three should be tested against when they are.

---

## What it cost

**Measured as account-balance differences**, at the Creator's instruction. Personal balances, spending totals and limits are withheld.

**Sesterce's bill came from its own `billing` record**, rather than a subtraction. It began billing when the instance went active and not when it was requested, so the pending interval cost nothing. Earlier records in the account were not touched or counted.

**On the price discrepancy the Creator caught.** On Vast the balance moved in step with the API's quoted `dph_total`, so on that seller the API figure was what was billed — machine A ran at a quoted `$0.0358/hr` including its 60 GB, and the balance bore that out. **The difference between the API's `$0.0233` for offer `44921058` and the `$0.0358` actually billed is the disk allocation**, which the offer's own `dph_total` does not include and the instance's does. That accounts for a gap of the size the Creator observed, though it is not proof that the web page's figure is the same arithmetic. **Hyperstack's spend is visible only as a credit difference**, which is why it is quoted that way.

**Every rental in this batch was destroyed and every account was checked empty afterwards.** Vast: `instances 0`, `volumes []`. Hyperstack: `instances 0`, `volumes 0`, `snapshots 0`. Sesterce: `instances 0`, volumes empty.

---

## What this document does not do

**It routes nothing.** Every finding above sits here and nowhere else. **The architecture documents these findings belong to have not been touched**, and at least three of the findings are **corrections rather than additions** — `lspci` for card presence in [Servers](Servers.md), the boot-registration route in [Rented-server runtime](Rented-server%20runtime.md), and the free reading of card counts below `nvidia-smi`. **A correction is the dangerous kind**, because the document it contradicts still reads perfectly well, and neither counted gate can see the contradiction: the gates match rows against sections, and a finding in an input document is neither.

**So the routing pass is owed and is named on the board.** Until it is done, this document is evidence that the stage has not yet acted on.
