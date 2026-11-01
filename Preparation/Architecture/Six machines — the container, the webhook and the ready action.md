# Six machines — the container, the webhook and the ready action

What six rented machines answered on 2026-08-15 and 2026-08-16, across Batches B, C and D. An **input** to the stage, like [The rented machine](The%20rented%20machine.md) and [Batch A](Batch%20A%20—%20the%20shape%20throwaway.md): nothing here is a decision or a plan.

**This is a frozen snapshot of six machines on two days**, and all six were destroyed at the end of it. A different rental may differ in every particular. Read it as *this is what those boxes did*, never as *this is what a rented machine does*. Where two of the six disagreed, both answers are recorded rather than reconciled.

Every claim is **observed**, with what was run and what came back. Where something was not established, it says so. Rule 1's plain-English rule is suspended here, per the stage's exception.

**Why six at once.** The Creator supplied five machines already running and authorised a sixth to be rented for one question. That made possible three things no single rental has ever allowed this project: a genuine **locked container** beside genuine **virtual machines**, so the same question could be put to both; a machine with **a card and no driver**, which is what settles what a provider actually leaves undone; and **more than one rental running at the same time** against one home line.

---

## The machines

Six boxes, three providers. The differences between them are the point, so they are set out in full.

### VAST-1 — the non-root and restart box

Vast.ai, reached at `31.59.39.104` port `31384` — a NAT-mapped high port. **Unmapped ports in the range answer `Connection closed`, not `Connection refused`**, so a wrong-port probe looks like a half-open endpoint.

Ubuntu 22.04.5, kernel `6.8.0-59-generic`, systemd 249 as PID 1, `systemd-detect-virt` → `kvm`. AMD Ryzen 7 3700X presented as **15 single-thread vCPUs** (15 sockets, one core each). 12 GiB memory, **no swap**. One real disk `vda` 130 G, ext4 `/` with 118 G free; **`vdb` is a 366 KB iso9660 cloud-init seed, not storage**. Memory-backed: `/dev/shm` 6.1 G, `/run` 2.5 G. **`/tmp` and `/var/tmp` both sit on the root disk.** One RTX 3080, 10240 MiB, driver **575.51.03**, installed by apt and DKMS with the packages **held**. Docker plus `nvidia-container-toolkit`. No Blender. Clock UTC, NTP synced.

**Four logins, and the differences matter.** `root` is **real root**. `user` (uid 1000) is the genuine ordinary provider account and **its `authorized_keys` is byte-identical to root's** — so the Creator's key opens both. `ubuntu` (uid 1001) has passwordless sudo but a **0-byte `authorized_keys`**, so it cannot be used at all. `smoozy2` (uid 1002) was created for Fact 60. **`user` is in the `docker` group**, which is silent root-equivalence.

### VAST-2 — the transfers box

Vast.ai at `195.5.177.123` port `15852`. Ubuntu 22.04.5, kernel `6.8.0-87-generic`, KVM guest. **5 cores** of a Threadripper PRO 3945WX, 24 GB, no swap, one 130 G ext4 disk. One RTX 4000 Ada, 20475 MiB, driver 580.105.08.

**Not a bare server.** A **KDE Plasma desktop is running** — Xorg, kwin, plasmashell and a screen locker were already holding **301 MiB of card memory** before anything was done to it — alongside selkies-gstreamer, Sunshine, wine-staging, Pinokio, Tomcat and Chrome. The Blender already present is an **official 5.0.0 Linux tarball unpacked into `/opt/blender-5.0.0-linux-x64` on 24 November 2025**, symlinked into `/usr/bin` and `/bin` with a desktop entry copied in; **no package manager knows about it**. Three more Blenders were added for Fact 45.

### VAST-3 — the workhorse

Vast.ai at `212.83.33.54` port `41010`, root. Ubuntu 22.04.5, kernel `6.8.0-59-generic`, KVM. AMD Ryzen Threadripper PRO 3975WX presented as **20 single-core sockets**. 60 GiB memory, **no swap**. One disk, 130 G ext4, 118 G free; the same 366 KB `vdb` decoy. **No second usable disk and no memory-backed scratch folder**, which differs from the box on earlier record.

**Two NVIDIA GeForce RTX 4090s**, 24564 MiB each, driver **580.95.05**, CUDA 13.0, using the **open** kernel module. `nvidia-smi topo -m` reports `PHB` between them — **no NVLink**. Persistence mode disabled, compute mode default, ECC disabled on both. No screen, no display server. **`unattended-upgrades` disabled and inactive**, unlike the box on earlier record whose kernel changed underneath it. `ulimit -n` 1024.

Five Blenders were placed on it five different ways for Fact 45: 4.5.12 unpacked into a plain folder, 3.0.1 through apt, 5.2.0 as a snap, 5.2.0 in a folder nobody would look in, and a copy of 4.5.12 **with its binary renamed** and reached by symlink.

### SESTERCE — the card with no driver

`38.80.122.52` port 22, login `sesterce` uid 1001, **non-root with passwordless sudo**, SSH key only. Hostname `shadecloud`. Ubuntu 22.04.5, kernel `6.8.0-40-generic`, systemd 249. `systemd-detect-virt` → `kvm`; vendor `OpenStack Foundation`, model `OpenStack Nova`. **Legacy BIOS, no Secure Boot** — `mokutil --sb-state` → `EFI variables are not supported on this system`, so kernel modules need no signing here.

28 logical CPUs, AMD EPYC 7763, 2 sockets × 14 cores, 2 NUMA nodes. 56.98 GiB memory, **no swap**. One 100 G disk, ext4, **`Reserved block count: 0`**. `/dev/shm` **29 G**. One RTX A6000 at `00:06.0`, plus **an emulated Cirrus Logic console adapter at `00:02.0`**.

**As delivered there was no proprietary driver at all** — no `nvidia-smi`, no `/dev/nvidia*`, no `/proc/driver/nvidia` — **but `nouveau` was loaded and bound to the card**. `nvidia-container-toolkit` was pre-installed **with no driver under it**. No Blender, no `unzip`, no `dkms`, no `nvcc`.

**The provider set `UsePAM no` in `sshd_config`**, and it is the most consequential choice on the box: an SSH login creates no logind session, so `XDG_RUNTIME_DIR` is unset, `/run/user/1001` does not exist, `systemctl --user` cannot reach a bus, and **the login never appears in `who` or `utmp`**.

### HYPERSTACK — the webhook box

`38.128.232.130`, login `ubuntu` uid 1000, non-root with passwordless sudo; **root login is refused by the provider**. Ubuntu 24.04.2, kernel `6.11.0-26-generic` at the start and `7.0.0-28-generic` after the driver work pulled a new one, KVM. 28 cores, 56 GB, one 100 G ext4 disk. No Docker. An RTX A6000 present with **no driver installed**, and **`nouveau` loaded** — the same delivered state as Sesterce.

Private address `10.0.0.184` behind floating address `38.128.232.130`. **Inbound is closed by default**: only port 22 was reachable, and the block is an OpenStack security group the machine itself cannot change.

### VAST-CONTAINER — the locked container

Vast.ai instance `47818861`, image `ubuntu:22.04`, **$0.0350/hr**, listed as Spain — **and a listed location is not evidence**, since one of the other rentals was listed in Spain and reported the United Arab Emirates from the box itself. Reached **only** through Vast's proxy at `ssh8.vast.ai` port `18860`; the rental had **no direct ports at all**. The proxy works because the container holds its own **outbound reverse tunnel** back to Vast.

**A container, not a virtual machine.** `systemd-detect-virt` → `docker`; `/.dockerenv` present; the root filesystem is a Docker `overlay`; the hostname is the container id; and **PID 1 is `/bin/sh -c while [ ! -e /.launch ]; do sleep 1; done; bash /.launch`** — not an init system. The kernel is the host's `5.15.0-187-generic` and **`/lib/modules` does not exist**.

Intel Core i3-3240, `nproc` 4 but **`cpu.max` 3.84 CPUs**; `/proc/meminfo` 15.57 GiB but **`memory.max` 14.95 GiB**. 30 G overlay. One GTX 1080 Ti, 11264 MiB, driver **580.173.02**, whose user-space half arrives as **about 35 read-only bind mounts** from the host's disk.

**Three of these rentals took finding.** The first jammed on a Vast proxy port a stale tunnel was holding, retrying `Error: remote port forwarding failed for listen port 17916` indefinitely. The second sat pulling a six-gigabyte CUDA image and never produced a log. Both were destroyed. **The third used a bare `ubuntu:22.04` image, on the reasoning that a Vast container gets its driver injected from the host rather than from the image** — which the machine then confirmed.

### One thing about two of them

The Sesterce box is at `38.80.122.52` in a region it calls `CAN-Montreal-2`; **Hyperstack's own published address ranges include `38.80.122.0/24`**, and the Hyperstack box is at `38.128.232.130`, another range on the same list. Sesterce's API carries a `cloudProvider` field on every create and a `cloud` object on every offer, and its live offers listing returned **0 container offers, 254 virtual-machine and 18 bare-metal** on 2026-08-15.

**So Sesterce appears to be a broker, and this rental appears to be physically a Hyperstack machine.** Both boxes arriving with an A6000 bound to `nouveau` and no driver is consistent with that. **They should not be counted as two independent providers** for any fact about hardware, drivers or networking. This is inference from published address ranges, not something either provider states.

---

## Fact 48 — whether a rented machine can be a locked container whose drivers are already in place and which Smoozy cannot change

**Answered: yes, and the *cannot change* half holds under every route tried.**

**One seller's own account of its own instances.** **Narrowed 2026-08-23; this headline read *Such rentals are the norm, not a corner case*, which stated Vast.ai's claim about Vast.ai's own instances as a property of rentals.** The paragraph below this one already records **0 container offers on Sesterce** and no container product on Hyperstack at this batch's own reading, and the three-seller batch of 2026-08-22 then delivered **virtual machines on the other two sellers** ([The multi-provider batch](The%20multi-provider%20batch%20%E2%80%94%20the%20surface%20that%20did%20not%20vary,%20the%20Debian%20route%20that%20exists,%20and%20the%20cards%20a%20slice%20cannot%20see.md)). **Only the headline is narrowed: every reading in this fact is this container's own and does not move.** Vast.ai's own documentation, read 2026-08-15: *"Instances are almost always Docker containers; a small subset are virtual machines."* (`docs.vast.ai/guides/concepts.md`) and *"The standard and most common way to run instances on Vast.ai is using Docker containers."* (`.../templates/introduction.md`). The three Vast boxes in this batch were **not** containers because they were rented from **VM templates** — all three use `docker.io/vastai/kvm:*` images. **The quotation this sentence used to carry — *"VM images are packaged as Docker images from the `docker.io/vastai/kvm` repository."* — is on no page of the seller's, and is replaced here on 2026-08-23 by the page's own words.** `https://docs.vast.ai/guides/templates/introduction`, read 2026-08-23: *"In addition to standard Docker container templates, we also offer Virtual Machine (VM) templates. These launch a full virtual machine environment rather than a docker container."* and *"You can edit VM templates just like regular templates, but you should not change the docker image field. Only the images we distribute from `docker.io/vastai/kvm` will work."* **The substance is unchanged** — the templates these three were rented from launch a virtual machine rather than a container, and their images come from that one repository.

Sesterce carries `deploymentType` values `vm`, `container` and `baremetal` in its API, but its live offers listing on 2026-08-15 returned **0 container offers** against 254 virtual-machine and 18 bare-metal. Hyperstack appears to sell no container product; its renter picks an image *"including variants with NVIDIA drivers and the CUDA toolkit pre-installed"*, which is the opposite arrangement.

### What root is allowed to do inside one

Root in the container is **not** a user namespace: `/proc/self/ns/user` is the kernel's initial namespace `4026531837`, the same as PID 1, and `uid_map` is the identity map `0 0 4294967295`. **Confinement is by capability dropping plus seccomp.** It holds **13 of 41 capabilities** — `CapPrm/CapEff/CapBnd = 00000000a80405fb`, with `CapInh` and `CapAmb` both zero. **The 28 missing** include **`CAP_SYS_MODULE`**, `CAP_SYS_ADMIN`, `CAP_SYS_RAWIO`, **`CAP_SYS_PTRACE`**, `CAP_SYS_BOOT`, `CAP_SYSLOG`, `CAP_NET_ADMIN`, `CAP_NET_RAW`.

That set is **Docker's stock default minus `CAP_NET_RAW`**. Seccomp is on with one filter. `/sys` is read-only; five paths under `/proc` are read-only — `bus`, `fs`, `irq`, `sys` and `sysrq-trigger`; `/proc/kcore`, `/proc/keys`, `/proc/timer_list` and `/proc/interrupts` are masked with `/dev/null`; `/proc/acpi`, `/proc/asound`, `/proc/scsi` and `/sys/firmware` are masked with empty tmpfs.

**Privileged behaviour is absent.** `umount` → `must be superuser to unmount`; `unshare -m` → `Operation not permitted`; `mount --bind` → `permission denied`; `dmesg` → `read kernel buffer failed: Operation not permitted`. `chroot /` succeeds, and buys nothing.

### Four routes to the driver, all closed

**Installing a driver package** fails on the bind mounts rather than on permissions:

```
dpkg: error processing archive .../libnvidia-compute-580_...deb (--unpack):
 unable to make backup link of './usr/lib/x86_64-linux-gnu/libcuda.so.580.173.02' before installing new version: Invalid cross-device link
E: Sub-process /usr/bin/dpkg returned an error code (1)
```

`nvidia-smi` afterwards is byte-identical. **The failure also poisons apt** — a later unrelated install failed with `E: Unmet dependencies`, and recovery needed `dpkg --remove --force-all` plus `apt-get --fix-broken install`.

**Loading or unloading a kernel module** returns `EPERM` on every route, including the raw syscalls with `kmod` bypassed:

```
init_module(fake image)      ret=-1 errno=1 (Operation not permitted)
delete_module(nvidia_drm)    ret=-1 errno=1 (Operation not permitted)
finit_module(/dev/null)      ret=-1 errno=1 (Operation not permitted)
```

**This is the sharpest evidence in the batch, and it is deliberate.** `init_module` was handed a malformed image — `\x7fELF` followed by 60 zero bytes. A caller **holding** `CAP_SYS_MODULE` gets `ENOEXEC` or `EINVAL` from that image, because the capability check runs first. **So `EPERM` proves the capability is absent rather than the image bad.** The unload targets were chosen the same way: `bluetooth` and `nvidia_drm` both show **0 users** in `lsmod`, so `EBUSY` was not available as an excuse.

**Writing to `/proc/driver/nvidia`** gives `EACCES` on every writable file tried — `registry`, `suspend`, and the per-card `registry`. The errno matters: `EROFS` elsewhere is *the mount* refusing, while **`EACCES` here comes from the NVIDIA driver's own write handler**, on a read-write filesystem, on a root-owned file the container "owns". `/proc/driver/nvidia/params` lists over twenty tunables — **all readable, none settable.**

**Replacing the user-space libraries in place is impossible** — the driver files are read-only bind mounts, giving `EROFS` on write, `Device or resource busy` on `rm` and `mv`, and `must be superuser` on `umount`.

### The one gap, and it is dangerous

**The `.so.1` symlinks everything actually opens are ordinary files in the container's writable overlay, not bind mounts.** So the user-space half **can** be shadowed by anything controlling `LD_LIBRARY_PATH` or those symlinks. Pointing the loader at 390.157 user space against the host's 580 kernel module:

```
cuInit -> 999
cuDriverGetVersion -> 0 version = 9010
```

`cuInit` fails, **but `cuDriverGetVersion` succeeds and reports CUDA 9.1** — the library's own idea, with no reference to the kernel module. **A version read from the CUDA library is not evidence the pairing works.**

Blender under that mismatch:

```
CUDA cuInit: Unknown error
DEVICE: Intel Core i3-3240 CPU @ 3.40GHz | CPU | use = False
scene.cycles.device = GPU
...
Fra:1 ... Time:02:22.84 ... | Scene, ViewLayer | Finished
Saved: '/root/out_ldpath_0001.png'
exit=0
```

**The card vanished from the device list, the device setting still said `GPU`, and Blender printed one line, rendered on the processor, and exited 0.** 128 samples took 2:23 against 1:29 on the card. **A broken driver pairing produces a slow, quiet, apparently successful render — not a refusal.**

**This also answers half of Fact 4, which the register records as uncovered** — the driver *mismatched* rather than removed. The Rendering spec's loud refusal holds for a **removed** driver and **not** for a mismatched one, which is the state an unattended upgrade produces.

Two practical notes. A symlink-only swap without `LD_LIBRARY_PATH` will not load at all. And **`ldconfig` silently reverts such a swap** by rescanning and recreating the symlink — and almost every `apt` operation fires an `ldconfig` trigger.

### One command escapes the container entirely

```
$ nvidia-smi -pm 0
Disabled persistence mode for GPU 00000000:01:00.0.  All done.
$ nvidia-smi -pm 1
Enabled persistence mode via daemon for GPU 00000000:01:00.0.  All done.
```

**"via daemon" is the giveaway** — `/run/nvidia-persistenced/socket` is bind-mounted in, so the request is served by the **host's** `nvidia-persistenced`. **A container-side command changes a host-wide driver setting.** Every other control operation is refused: `-pl` and `-c` give `Insufficient Permissions`, `-r` gives `GPU Reset couldn't run because GPU 00000000:01:00.0 is the primary GPU.`

### What a driver fault means on a container

**The machine is lost until the provider acts.** Reset refused, module reload refused, suspend refused. **And a restart does not help**: across two container restarts the host's uptime rose monotonically — `481887.72` → `481963.96` → `482447.24` — with the module table and NVRM string unchanged, so **the host never rebooted and the driver was never reloaded**. A container restart kills the container's processes, which does free leaked GPU contexts, but cannot touch a wedged kernel module.

**For Smoozy: on a container rental, "restart the machine" is not a recovery action for a driver fault. A card that has stopped answering is terminal for that server.**

### What the container sees of the host

More leaks than expected: **the host's entire module list** via `/proc/modules` (128 modules, including `bluetooth`, `kvm_intel`, `r8169`, `btrfs`), **its full PCI bus**, **its display outputs**, its kernel version, its disk sizes through `df`, and its uptime. Other tenants are **not** visible — separate PID and network namespaces, `dmesg` refused, `/proc/kcore` masked.

The card was not shared here, and the provider states *"GPUs are exclusive resources, they are never shared between multiple users at the same time."* **Exclusivity cannot be verified from inside** — another tenant's process sits in a different PID namespace and `nvidia-smi` in a container cannot name it.

**Not covered.** No actual card hang or Xid fault was induced — what is established is that *if* one occurs, none of the four repair routes is open. Untested: a multi-card rental where reset might succeed on a non-primary card; MIG; a rental sold as privileged; Entrypoint launch mode. **The observed capability set is Docker's default; a provider running `--privileged`, or adding `CAP_SYS_ADMIN`, would answer this fact differently, and this test cannot rule that out.**

---

## Fact 1's narrow clause — registering a boot return under the non-root login Smoozy holds

**Answered: yes on both kinds of machine, but by completely different arrangements — and on a container every route a builder would reach for first is a silent no-op.**

Batch A answered the plain form as real root on a virtual machine. This clause needed a genuine non-root login, and it got three: two on virtual machines and one on a container.

### On a virtual machine — a user service plus lingering

On Sesterce, everything ran as uid 1001 with **no sudo except the reboot itself**.

The provider's `UsePAM no` is the obstacle: an SSH login creates no logind session, so `systemctl --user` cannot reach a bus, `/run/user/1001` does not exist, and `loginctl show-user` reports the user *"is not logged in or lingering"*.

**`loginctl enable-linger` with no argument fails** — `Could not enable linger: No such device or address` — because "self" is derived from a session that does not exist. **By name it succeeds, unprivileged:**

```
$ loginctl enable-linger sesterce
exit=0
$ loginctl show-user sesterce -p Linger -p State
State=lingering
Linger=yes
```

That is polkit granting `set-self-linger` to an unprivileged caller. **Hyperstack independently established why**, which matters because it turns Sesterce's biggest stated worry into a testable property: `pkaction` shows `org.freedesktop.login1.set-self-linger` carries **`implicit any: yes`** in the systemd project's shipped policy. **That policy was read on one machine, 24.04; what both machines agree on is the behaviour, not the reading.**

With a unit under `~/.config/systemd/user/` and `enable --now`, the machine was rebooted with nobody connected. It came back in 21 seconds with a new boot id, and the program's own record reads:

```
=== START 2026-08-15T21:00:24Z ===
  boot_id=1383ab02-...  uptime_at_start=1822.60s
  logged_in_users=[]  loginctl_sessions=[]
  resumed_queue_position=0
=== START 2026-08-15T21:01:20Z ===
  boot_id=c80317b5-...  uptime_at_start=6.71s
  logged_in_users=[]  loginctl_sessions=[]
  resumed_queue_position=4
```

**It returned 6.71 seconds into the new boot, as uid 1001, with nobody logged in, resuming from where it stopped rather than starting over.** The system side confirms it was started by uid 0 at boot rather than by a login, and `loginctl list-sessions` reported `No sessions.` Hyperstack reproduced it independently: service started **six seconds** after boot, four heartbeats before any login.

**The negative control failed on demand, which is what makes this a test.** With the unit still `enabled` and only lingering turned off, the program **did not come back** — and *"no log line anywhere says why"*. Turning lingering off also killed the running program immediately rather than at the next boot. Hyperstack proved the same with an 89-second no-login window and zero heartbeats. **`enable` alone is not enough, and its failure is silent.**

`crontab @reboot` also works on a virtual machine — 6.77 seconds into the boot, uid 1001, no sessions — **but it launches once and forgets**: no restart, no status, no supervision. A fallback, not an equal. It carries its own trap, **whose mechanism is corrected here on 2026-08-23 against the page itself, this sentence having said that a job containing the character *runs, does nothing, and cron logs it as a successful command***. `crontab(5)` as Debian ships it, read 2026-08-23 at `https://manpages.debian.org/trixie/cron/crontab.5.en.html`: *"The entire command portion of the line, up to a newline or % character, will be executed by /usr/bin/sh"*, and *"Percent-signs (%) in the command, unless escaped with backslash (\\), will be changed into newline characters, and all data after the first % will be sent to the command as standard input."* **So an unescaped `%` truncates the command at that point and the truncated prefix runs**, with everything after the `%` handed to it as standard input. Nothing refuses the entry and nothing reports a fault: what runs is a shortened command, and its own ending is what cron records. **Documented, not observed** — no entry in this batch carried a `%`.

**Sesterce's cron needed no such help** — no allow or deny list, so any login may use it.

### On a container — none of that exists, and two routes lie

PID 1 is `/bin/sh -c while [ ! -e /.launch ]; …`, and there is no systemd. So:

```
$ systemctl status
System has not been booted with systemd as init system (PID 1). Can't operate.
$ loginctl enable-linger root
System has not been booted with systemd as init system (PID 1). Can't operate.
```

**But:**

```
$ systemctl enable ssh
Synchronizing state of ssh.service with SysV service script with /lib/systemd/systemd-sysv-install.
exit=0
$ ls -la /etc/systemd/system/multi-user.target.wants/
lrwxrwxrwx 1 root root 31 Aug 15 21:57 ssh.service -> /lib/systemd/system/ssh.service
```

**`systemctl enable` reports success, writes the symlink, exits 0 — and the unit will never run.**

**`@reboot` cron never fires either.** Three separate fresh starts of the daemon, including with a trivial job, gave zero, while a `* * * * *` entry in the *same* crontab fired every minute throughout. The likely reason — **flagged as reasoning, not observation** — is that Debian's cron consults `/proc/uptime` to decide whether the machine has just booted, and `/proc/uptime` in a container is the **host's**, here 5.6 days.

**So a builder gets two silent no-ops where a mechanism is expected.**

**The one route that works is the provider's own hook**, `/root/onstart.sh`, which Vast documents: *"Add to `/root/onstart.sh` for persistence across restarts."* It ran on both container restarts — twelve seconds after the first request and five after the second — and launched a supervisor running as the non-root login:

```
SMOOZY SUPERVISOR STARTED 2026-08-15T22:18:02Z pid=78 ppid=71 uid=1000 user=smoozy
```

**Installing that hook requires root.** From the non-root account, `/root/onstart.sh` and `/etc/rc.local` are both `Permission denied`, and `sudo -n` demands a password. **So a non-root login has, on its own, no boot-registration route at all on a container.** The clause is answerable only as *root installs the hook, and the hook drops privilege*.

**And a non-root cron job on a container needs one edit first**, which is where the trap lives rather than on a virtual machine. `pam_loginuid.so` is `required` in `/etc/pam.d/cron`. **The mechanism sentence written here was inverted and is corrected 2026-08-20**: it said reading `/proc/self/loginuid` gives `EPERM`, and that is not producible on this kernel. The file is mode 0644 (`REG("loginuid", S_IWUSR|S_IRUGO, …)`, `fs/proc/base.c` at tag v5.15, the kernel these boxes ran) and the read path has no permission or capability check at all — its only error is `-ESRCH`. **`EPERM` comes from the write**, in `audit_set_loginuid_perm` (`kernel/audit.c`), where the capability is `CAP_AUDIT_CONTROL` — and **only when the loginuid is already set**, since that function's first line returns 0 with no privilege check when it is unset. `pam_loginuid(8)` says the module *"sets the loginuid process attribute"*: sets, not reads. **This record does not say which of the two was observed**, so the causal clause about the missing capability is dropped rather than kept. None of the practical result changes: the cron session failed, the job was dropped, and the edit made is the ordinary fix. **The session fails and the job is dropped with no visible error.** The mechanism cannot arise on a virtual machine, which holds the full capability set.

### The two answers are coherent, and the pair is the point

They differ because the machines differ, and the mechanism was observed rather than inferred — systemd is PID 1 on one and absent on the other. **A document written only on the virtual-machine answer would fail silently on a container**, because both routes it would name report success and do nothing there.

So what the spine documents inherit is **two arrangements and a detection step**, never one:

- **On a systemd machine:** a user unit under `~/.config/systemd/user/` **plus `loginctl enable-linger <name>`**. No sudo needed. Returns in 6–7 seconds with nobody logged in, resuming rather than restarting. `enable` alone fails silently. On a `UsePAM no` provider, `XDG_RUNTIME_DIR` must be exported by hand and the bare `enable-linger` fails.
- **On a container:** `/root/onstart.sh` and nothing else. Root installs it, the hook drops privilege. **`systemctl enable` must never be the registration step.**
- **Tell them apart** with `systemd-detect-virt` and `/.dockerenv` before choosing.

**Not covered, and all three are load-bearing.** **Crash return was tested on neither kind** — both machines did clean reboots only, and the fact's own wording says *"after a reboot **or a crash**"*. The container's **recycle** case, answered separately below, changes what "the filesystem carries everything forward" is worth. And **whether the lingering permission holds on a provider whose policy differs** is narrowed but not closed: one machine called it the single most fragile assumption in the positive answer, and the other, having found the shipped policy that grants it, said plainly that **this is a per-image fact rather than a Linux one and should be re-read on each provider rather than assumed.** It is one command to check, and a ready action should check it rather than trust it.

---

## Recycle — what a container rental does that a virtual machine does not

**Answered on 2026-08-15, and it is the sharpest hazard in the batch.**

The container agent had posed only the **reboot** form of restart. Vast documents a second: *"Destroys and recreates container in place (from newly pulled image) without losing GPU priority."* That was then run deliberately, through `PUT /api/v0/instances/recycle/47818861/` → `{"success": true}`.

**What a negative would have looked like:** the machine returning with its hook, its Blender, its created login and its markers intact — recycle behaving like the reboot already tested.

Before, at 22:52:35 — `/root/onstart.sh` 825 bytes dated 22:17, Blender present, `smoozy` uid 1000 present, markers in `/root`, `/tmp` and `/etc`, two crontabs, 362 packages. **After, at 22:54:02 — about 90 seconds later, same address, same proxy port:**

```
onstart.sh:      -rw-r--r-- 1 root root 82 Aug 15 22:53 /root/onstart.sh
blender:         No such file or directory
smoozy user:     id: 'smoozy': no such user
markers:         all three — No such file or directory
crontabs:        /var/spool/cron/crontabs/: No such file or directory
dpkg count:      222
nvidia:          NVIDIA GeForce GTX 1080 Ti, 580.173.02
```

**Everything Smoozy installed was destroyed**, down to the `cron` package and the edit to `/etc/pam.d/cron`, with the package count back to exactly what the image shipped with.

**The boot hook was not deleted but replaced** — `/root/onstart.sh` exists again as an **82-byte provider default dated at the recycle moment**. **So a check for "does my hook exist" passes while the hook does nothing Smoozy asked for.**

**The card and driver are untouched**, consistent with the driver belonging to the host and never having been the container's.

**And nothing in the connection distinguishes a recycled machine from a rebooted one** — same address, same port, reachable in 90 seconds against 15–20 for a reboot. **A Smoozy that reconnects and finds its address answering has no signal that its whole installation is gone. It must check for its own state, not for reachability.**

**Not covered:** one recycle, one container, one provider. The **stop/start** form was not posed — it releases the GPU, and the provider says it may not be reclaimable. Whether recycle re-pulls a *changed* image, and so silently changes the operating system underneath Smoozy, was not tested; the tag was unchanged here. Nothing was running at the moment of recycle, so what it does to work in flight is unobserved.

---

## Making a machine ready — Facts 47, 80 and 54

### Fact 47 — what a machine needs beyond a graphics driver before a render will run

**Answered on four machines, and there is no single answer: it is a property of the image the provider ships, not of rented machines.** The four results were **nothing**, **two things**, **four things** and **seven things**. That spread is the finding.

- **VAST-3, which had the driver and a fat provider image: nothing.** A render ran on both card technologies with only an unpacked Blender tarball added. Every library Blender links was already present, and `libnvoptix.so.1` and `libcuda.so.1` **come with the driver package rather than a toolkit**, which is why no CUDA toolkit was ever needed on any of these machines.
- **The container, on a bare `ubuntu:22.04`: two.** An `xz` decompressor, and nine X11/OpenGL libraries — plus the 33 packages apt pulls in behind them, 178 MB.
- **SESTERCE: four.** Three libraries the driver install did not supply, Blender itself, an unpack tool, and about 3.8 GB of disk.
- **HYPERSTACK: seven**, including two traps of its own — `ubuntu-drivers install --gpgpu` **does not install `nvidia-smi`**, and it **omits `libnvoptix`**, so OptiX fails with `Error: Found no Cycles device of the specified type` while CUDA works.

**All four agree on the mechanism, and that is what a ready action must be built on.** The blocker is the X11 and OpenGL shared-library chain, and **`libgl1` is needed even for a background render**, because the executable links `libGL.so.1` at load time — the failure is at launch, not at render.

**And Blender names only the first missing library.** From the container:

```
$ ./blender --version
./blender: error while loading shared libraries: libX11.so.6: cannot open shared object file: No such file or directory
exit=127
$ ldd ./blender | grep "not found"
	libX11.so.6 => not found
	libXrender.so.1 => not found
	libXfixes.so.3 => not found
	libXi.so.6 => not found
	libxkbcommon.so.0 => not found
	libSM.so.6 => not found
	libICE.so.6 => not found
	libXext.so.6 => not found
	libGL.so.1 => not found
```

**Fixing them one at a time takes as many rounds as there are libraries. A ready action must read the whole list at once and never work from the launch error.** SESTERCE needed three rounds' worth and HYPERSTACK five before this was understood.

**One trap that hides the problem rather than showing it.** On SESTERCE the NVIDIA metapackage dragged in a whole X stack — `xserver-xorg-core`, `libgl1`, `libegl1`, `libsm6`, `libice6`, mesa, 76 packages — and **incidentally supplied six of the nine missing libraries**. Had the driver not gone in first, five would have been missing rather than three. **So a ready action must install these explicitly and never rely on the driver's dependencies.**

**Not covered:** four images on three providers. The mechanism is settled; the list is not, and cannot be, since it is per image. **And one of the four broke the fact's own precondition** — it requires an untouched machine and says the question must be asked first, and on HYPERSTACK it was asked sixth. That agent flagged it itself. The other three did ask it first, so the answer stands, but **a strict reading would want HYPERSTACK's re-posed on a box where it genuinely ran first.**

### Fact 80 — whether the driver work can require a restart

**Answered: yes on both machines that posed it, by two different mechanisms — and they disagree about the signal, which is the useful part.**

**SESTERCE needed a restart because the provider left the card bound to `nouveau`.** The install finished at exit 0 in 85 seconds, **printed no reboot demand**, and left the card unusable:

```
$ ls -l /var/run/reboot-required*
ls: cannot access '/var/run/reboot-required*': No such file or directory
$ nvidia-smi
NVIDIA-SMI has failed because it couldn't communicate with the NVIDIA driver...
EXIT=9
```

The kernel says exactly why: `NVRM: GPU 0000:00:06.0 is already bound to nouveau.` The package **did** install the blacklist that fixes it at next boot — into `/lib/modprobe.d`, not `/etc/modprobe.d` — which has no effect on the running kernel.

**HYPERSTACK needed one for a different reason** — the install pulled a whole new kernel and **did** print its own demand: `Restarting the system to load the new kernel will not be handled automatically, so you should consider rebooting.` Before rebooting, `nvidia-smi` exited 9 and `modprobe nvidia` gave `FATAL: Module nvidia not found in directory /lib/modules/6.11.0-26-generic`.

**So Ubuntu's own reboot-required flag is not a reliable signal — it was never set on the machine that needed a restart most.** The rule that survives both: **run `nvidia-smi` after installing and restart only if it fails.** Not unconditionally, and never from the package manager's word.

**Avoiding the restart by hand was possible on SESTERCE and left the machine worse.** Unloading `nouveau` rebound the card, but `nvidia-smi` then gave **the cardless answer** (`No devices were found`, exit 6). A full unload and reload brought it up — and it **broke again on the very next command** (`Failed to initialize NVML: Unknown Error`), because the reload left device nodes with the wrong majors. The state then oscillated, working and inconsistent at the same time. **No ready action should ship in that condition, and no reading taken in it can be trusted — including card detection.**

**NVIDIA's own documentation**, read 2026-08-15 at `us.download.nvidia.com/XFree86/Linux-x86_64/570.211.01/README/commonproblems.html`: *"You will need to reboot your system after adding these configuration directives in order for them to take effect."* and, on the prerequisite, *"If you're installing on a system that is set up to use the Nouveau driver, then you should first disable it before attempting to install the NVIDIA driver."* **Both alternatives NVIDIA offers are boot-time changes. It offers no route that avoids a restart.** **That quotation answers a different question and this conclusion does not follow from it — corrected 2026-08-20.** It sits at the end of NVIDIA's answer to *"How do I prevent Nouveau from loading and performing a kernel modeset?"* and is about the two lines written into a modprobe configuration file taking effect at the next boot. **The sentence immediately before it says the opposite in NVIDIA's own words**: *"Without the kernel modeset, it is possible to unload Nouveau's kernel module, in the event that it is accidentally or intentionally loaded."* (same page, re-read 2026-08-20). **The decision does not change** — run the card tool after installing and restart only if it fails, which is sound on its own merits and costs 9 to 28 seconds on these boxes — and neither does the observation. **What changes is the reason given, and the flat claim that no reading taken in that state can be trusted**, which is written as a permanent property of what is an ordinary repairable state: the repair NVIDIA does document is that `nvidia-modprobe` creates the device files, and only *"when ... the NVIDIA character device files do not yet exist"*, so a stale file that does exist is not replaced. **The repair itself is undocumented and unposed here**, and wants a Linux machine with an NVIDIA card.

**Restart cost varies by an order of magnitude between machines, and that is the useful part.** Measured eight times across three: **9, 11 and 12 seconds** on HYPERSTACK, the fastest in the batch; **21, 22 and 28** on SESTERCE; **85 to 90** on VAST-2. Anything that budgets a fixed wait for a machine to return will be wrong on most of them.

**Bare card to a rendered image: about 4 minutes 15 seconds on SESTERCE — six commands and one reboot — and 6 minutes 27 seconds on HYPERSTACK, whose own list ran to seven items and five failed launches. Nothing on either needed a human.**

**Not covered, and it is a real risk.** **Secure Boot.** SESTERCE had legacy BIOS with no EFI, and its build logged `EFI variables are not supported on this system` when it tried to sign the module. **On a Secure Boot rental that step could fail outright, or demand a human enrol a key at the console — which Smoozy cannot do over SSH.** This fact as worded does not cover it, and it wants a number and a machine of its own. Also untested: driver *upgrades* and *removals*, and the `.run` installer route.

### Fact 54 — reaching Blender's downloads and the extensions site

**Answered: yes on every machine that posed it, and the fetch happens on the machine.** VAST-3 pulled 360 MB and 366 MB in 10.3 and 9.9 seconds — **about 37 MB/s on the machine's own line**, with nothing passing through the Mac. SESTERCE reached six blender.org hosts, all 200, and fetched a 335 MB archive **in 2 seconds**.

Both ran deliberate negative controls in the same session — `curl: (6) Could not resolve host` against an invalid name, and `curl: (28) ... Connection timed out` against a closed port — so the test could fail.

**One thing worth carrying:** the downloads honour byte ranges — `http_code=206` against a range request, returning the archive's real first bytes, `fd 37 7a 58 5a 00`, which is the XZ magic number — so a resumable fetch is available from a rental.

**And one trap in reading the index.** A plain alphabetical sort of the release listing picks **4.2.9** as the newest when **4.2.23** exists. Anything Smoozy writes that reads that index must sort by version, not by string.

**Not covered:** one moment on each machine; reach over hours or days untested; no proxy or egress-filtered box tested; an actual extension package was never fetched, only the listing.

---

## The cards — Facts 46, 3 and 43

### Fact 46 — reading the cards and the machine's own memory

**Answered for count, per-card identity and per-card memory, on a two-card box with no Blender, no screen and no display server — which is the form the fact asks for. The no-card half is not answered.**

**The count reads three independent ways** — `nvidia-smi -L`, `/proc/driver/nvidia/gpus`, and a PCI vendor scan in sysfs — all saying two.

**One trap, and it is exactly the several-card trap the fact exists to catch.** `nvidia-smi --query-gpu=count --format=csv,noheader` returns:

```
2
2
```

**The value once per card.** A caller reading the first line is right by luck; one parsing the whole output as an integer gets `22`. **It passes every test ever written on a one-card machine.** Safe readings are `nvidia-smi -L | wc -l` or counting rows of any per-card query.

**Identity needs more than a name.** Both cards are the same model, so **the UUID and bus location are what tell them apart** — a distinction that cannot arise with one card. `lspci` listed them **without a model name** (`NVIDIA Corporation Device [10de:2684]`), so on that box it gave a count and a PCI id, not an identity. **That is this image's own ID database being older than the card, not the tool declining to name it — corrected 2026-08-20.** The PCI ID Repository, the file `lspci` reads, gives vendor `10de` device `2684` as *"AD102 [GeForce RTX 4090]"* with a discussion entry dated 2022-06-22 (`https://pci-ids.ucw.cz/read/PC/10de/2684`, read 2026-08-20), and `lspci(8)` documents the names as coming from a `pci.ids` file that goes stale, with `update-pciids(8)` to refresh it. **The tell is in the observation itself**: the vendor half resolved to *NVIDIA Corporation* from the same lookup on the same run. **The design does not change** — identity comes from `nvidia-smi` and presence from `lspci -d 10de:`, which is the right way round regardless.

**Per-card memory has exactly one route, and it is `nvidia-smi`.** `/proc/driver/nvidia/gpus/*/information` gives model, UUID and bus location and **no memory figure at all**. So where `nvidia-smi` is absent, count and identity survive and **per-card memory does not**. Also: **483 MiB of each card was already held by the driver at idle**, so free is not total.

**Three states exist, not two, and all three were seen.** SESTERCE and HYPERSTACK both arrived with a card in `lspci` and **no driver**, where `nvidia-smi` is simply absent — **byte-identical to what a cardless machine gives.** A machine with a driver but no card gives a third answer, `No devices were found`, exit 6, seen for real on SESTERCE in the window between unloading `nouveau` and binding `nvidia`.

**So the discrimination the fact asks for is possible, but only by reading PCI.** `lspci -d 10de:` returns an empty result with exit 0 for an absent vendor and a populated line for a present one. **Anything concluding "this machine has no GPU" from `nvidia-smi` failing will misclassify a perfectly good rented A6000 as cardless — and on two of these three providers that is not a corner case, it is what they shipped.**

**Two further traps.** Counting `/sys/class/drm/card*` reports **two cards on a one-card machine**, because both SESTERCE and HYPERSTACK carry an **emulated Cirrus console adapter** that is a display device and not a render card — so vendor id `0x10de` is the discriminator, never device class. And **in a view that genuinely has no cards, `/proc/driver/nvidia` and the PCI sysfs scan lie**: both are inherited from the host kernel and still reported two NVIDIA devices in a container that could touch neither.

**One route exists only in the pre-driver window.** `nouveau` prints the frame buffer size at boot — `nouveau 0000:00:06.0: fb: 49140 MiB GDDR6` — and that number **later matched `nvidia-smi` exactly**. It needs `adm` group membership to read (`kernel.dmesg_restrict = 1`), and it **vanishes once the proprietary driver is installed and `nouveau` is blacklisted**. Convenient for telling a user what a machine has before installing anything; absent on a machine that arrives with `nouveau` already blacklisted.

**On a container, the machine's own memory reading is wrong.** `nproc` said 4 and `/proc/meminfo` said 15.57 GiB, while the real allocations were **`cpu.max` 3.84 CPUs and `memory.max` 14.95 GiB**. They nearly coincided only because that rental held all of its host's GPUs. Vast's own documentation: *"a 1-GPU instance on a 4-GPU machine gets 25% of CPU baseline"* and *"Instances can temporarily use additional RAM above their allocation … but doing so risks having processes killed by the OOM killer."* **On a 1-of-8 rental, `/proc/meminfo` would overstate usable memory eightfold. The reading Smoozy must take on a container is the cgroup's, falling back to `/proc` only when it reads the literal `max`.**

**Not covered, and this fact is not closed:** **a genuinely cardless rental was never posed.** All four attempts used a proxy — vendor filters returning empty, a GPU-less container view, or argument from what a cardless machine would necessarily show. It needs the cheap cardless box Batch B2 always specified.

### Fact 3 — which card-technology names a build accepts

**Answered for the NVIDIA make, and the two routes disagree with each other.**

`blender --help` prints, identically in 4.5.12 and 5.2.0: *"Valid options are: 'CPU' 'CUDA' 'OPTIX' 'HIP' 'ONEAPI' 'METAL'."* **That list is wrong about this build** — the setting itself takes five names and refuses `METAL` outright: `enum "METAL" not found in ('NONE', 'CUDA', 'OPTIX', 'HIP', 'ONEAPI')`. **`--help` prints one list for every platform.**

**"Accepted" and "usable" are different questions.** `HIP` and `ONEAPI` are accepted by the setting and find nothing; only the device list separates them. **Both cards read `use True` with nobody having switched anything on**, and **the processor appears inside each technology's own device list**, so counting `get_devices_for_type('OPTIX')` gives 3 on a two-card box, not 2.

**At launch, with a real render, there are two distinguishable refusals:**

| `--cycles-device` | exit | wrote a file | printed |
|---|---|---|---|
| `CPU`, `CUDA`, `OPTIX`, `OPTIX+CPU` | 0 | yes | ordinary progress |
| `HIP`, `ONEAPI`, `METAL` | **1** | **nothing** | `Error: Found no Cycles device of the specified type` |
| `BOGUS` | **1** | **nothing** | `Error: BOGUS is not a valid Cycles device.` |

**A name the build does not know is distinguishable from one it knows but no hardware answers to. What is *not* distinguishable is a broken driver from absent hardware** — both give `Found no Cycles device of the specified type`, the same line the purged-driver run produced on the earlier record.

**`METAL` deserves its own note: `--help` advertises it, the Python setting refuses it, and the command line accepts it as a valid name then finds no device.** Three routes, three different answers, one build.

**Not covered:** NVIDIA only, as the fact allows. AMD, Intel and Apple hardware are unposed and want another rental.

### Fact 43 — reading a card's live memory use

**Answered, and the several-card case the register records as never established is now established.**

With a render pinned to card 0 by `CUDA_VISIBLE_DEVICES=0`:

```
0, GPU-77ea3507-..., RTX 4090, 24564 MiB, 2801 MiB, 21281 MiB, 97 %, 50 %, 44, 229.47 W
1, GPU-efef9623-..., RTX 4090, 24564 MiB,    0 MiB, 24081 MiB,  0 %,  0 %, 34,   7.80 W

gpu_uuid, pid, process_name, used_gpu_memory [MiB]
GPU-77ea3507-..., 7963, /opt/blender-plain/blender-4.5.12-linux-x64/blender, 2792 MiB
```

The columns are card, identity, model, total, used and free memory, then **processor utilisation and memory-bus utilisation**, temperature and power draw.

**Three separate things read per card rather than as a total** — memory used, utilisation, and **which process is on which card**, named by the card's UUID and the process's own path and pid. **The idle card reading zero at the same instant rules out every way this could have been a machine-wide figure.** It is live, not cached: both cards read zero after the render stopped.

**Two figures, not one, and they differ.** Card-level `2801 MiB` against process-level `2792 MiB` at the same instant — **9 MiB of driver context not attributed to the process.** Anything comparing "what my render is using" against "what the card has" must know which it is reading.

**It works unprivileged**, because `/dev/nvidia*` are `crw-rw-rw-`.

**Not covered:** two renders on two cards at once, and one render spanning both, were **not** posed — the obvious next several-card questions. Whether a non-root user can see *another* user's process in the compute-apps list is untested, and it matters for attribution.

---

## Driving Blender — Facts 5, 77, 45 and 71

### Fact 5 — opening a `.blend` and handing a reading back

**Answered for all three payloads, in both installed versions, posed beside a render already running, each read taking about one second.**

The file was built for the purpose: two scenes with one pulling the other in, three view layers with one **keyframed** off at frame 10 and on at 20, a compositor with a File Output node, non-default settings throughout, and external references of four kinds.

**Payload 1, the file's recorded locations.** A flat list returns three; the locations it **does not** name — both scenes' output paths, both bake paths, the File Output node's path, and a custom property — each need going to directly. **An unused image data-block appears with an empty path**, which a naive caller would treat as a location.

**Payload 2, every value the Render Settings spec shows.** All present and identical in both versions, including the per-view-layer sample override reading 7 against the scene's 33 — so **the spec's override warning has a source**. The spec's hardest claim holds: where the view-layer switch is keyframed, **one read answers the question for any frame later asked about**, because the whole curve comes back and can be evaluated at any frame.

**Payload 3, destinations, compositor tree, gating conditions and per-frame set.** All four answered. The per-frame set **ends at 49 and never renders 50**, exactly the step behaviour the spec describes.

**And this is where the versions part company, severely.** Reading **the same file on disk**, 4.5.12 saw six compositor nodes and 5.2.0 saw eight — Composite became Group Output, `MixRGB` became `ShaderNodeMix`, and the links were rewired, all from version-conversion on load. **So a reading of a file's compositor tree is a reading taken by a particular Blender, not a property of the file.** Anything comparing a stored tree against a fresh one, or showing the user "the compositor in your file", must say which version read it.

Two version differences a caller must handle by name: the File Output path is `base_path` under `scene.node_tree` in 4.5 and `directory` under `scene.compositing_node_group` in 5.2, and slots need `file_slots[].path` against `file_output_items[].name`. **The first 5.2 run failed outright** with `AttributeError: 'CompositorNodeOutputFile' object has no attribute 'file_slots'`, which is the honest evidence this is real rather than something worked around quietly. **A reader asking for one name gets nothing in the other version, silently.**

**Beside a running render**, the reads cost about one second each and the render kept its pace — `Sample 69633/400000` advancing normally across them. **The rental imposed no visible limit.**

**Not covered:** one file, built by the agent. A file with a sequencer, sound, fonts, packed data, a driver on the view-layer switch or geometry-node imports is untested. Many simultaneous reads, a read with both cards busy, and a very large `.blend` where one second would not be the answer — all untested.

### Fact 77 — rewriting a `.blend`'s recorded locations so the change reaches disk

**Answered: yes, on both routes, in both versions present, with the change reaching the file on disk — and only if the file is saved.**

**The two routes are the project's own research, not the agent's invention**, taken from *What has to travel with a file* and the comparison questions it names. The one-pass visitor the research records as present from 5.0 was confirmed **by probe** to exist in 5.2.0 and be absent in 4.5.12.

**The wholesale sweep** reached three locations and **missed seven** — both scenes' output paths, both bake paths, the File Output node's path and the custom property — **exactly the seven-miss shape the research records**, confirmed on this machine in both versions. **The mark walk** reached eight and **stuck seven to disk**, including **every write location the sweep missed**, and found the compositor path in both versions **without needing to know its name** — which is the argument for the mark walk over a hand-written property list. What it missed was the text data-block's path and the custom property, **exactly the two misses the research records**.

**So the two routes together cover everything except the custom property** — the research's own conclusion, now confirmed by test on a rented box rather than inherited.

**The control that makes this a test.** Same sweep, not saved:

```
"make_paths_absolute_result": "{'FINISHED'}",
"after_in_memory": { "image/LoadedTex": "/root/w/45-nosave/tex/disktex.png", ... },
"saved": false,
"after_reopen_from_disk": { "image/LoadedTex": "//tex/disktex.png", ... },
"reached_disk": {}
```

**The operator reports `FINISHED`, the value reads back changed inside the running Blender, and the file on disk is untouched.** That is what makes the never-write-the-artist's-file promise keepable — **and it is a failure invisible to anything that trusts the operator's return value.**

**The one-pass route reaches exactly what the sweep reaches and misses exactly what the sweep misses.** It is a nicer sweep — it names the data-block each path belongs to — but **not a substitute for the mark walk**, so the two-routes-together requirement holds in 5.2 as much as in 4.5.

**One trap nobody warned about.** An image carries **two** marked path properties, `filepath` and `filepath_raw`, aliases of one another, and a naive walk rewrote the same image twice, producing `/mnt/relocated/mnt/relocated/tex/disktex.png` on disk. **A mark walk must know that some marked properties are two views of one location, or it double-applies its prefix.**

**Not covered:** the sweep was run in the **absolute direction only**. `Make Paths Relative` was not run. **5.0 and 5.1 are untested** — only 4.5 and 5.2, which satisfies the fact as written but not "every supported version" if those count separately. No rewrite was tested against a file linked from another file.

### Fact 45 — searching a machine for every Blender on it

**Answered: partly, and the failure is the one the fact predicts, plus one worse.**

Five Blenders were planted five ways, so every route could be checked against known ground truth.

| Placement | PATH | apt | snap | desktop | file search | content probe |
|---|---|---|---|---|---|---|
| plain unpacked folder | no | no | no | no | **yes** | yes |
| through apt | yes | yes | no | yes | yes | — |
| as a snap | yes | no | yes | yes | yes | — |
| a folder nobody would look in | no | no | no | no | **yes** | yes |
| **binary renamed** | no | no | no | no | **NO** | **yes** |

**The fact's prediction is confirmed exactly: a plain unpacked folder is invisible to every route but a file search.** And **a renamed binary defeats the file search too** — only running candidates and reading their version output found it. That probe is a heuristic, not a search: it would miss a small build and costs an execution per candidate. **The whole-disk file search cost 5.3 seconds** on a 126 GB disk 12% full.

**Two traps in asking a candidate its version.** The apt build **prints two lines of colour-management noise before its version**, so anything reading "the first line" gets `Color management: using fallback mode for management` and no version — every build must be parsed by finding the line beginning `Blender `, never by position. And **deduplication by real path is wrong for snaps**: `/snap/bin/blender` resolves to `/usr/bin/snap`, so a dedup keyed on the resolved path would collapse every snap into one entry and could name the snap launcher as a Blender. **The version each path reports is the reliable discriminator.**

**One more trap in the search itself:** restricting it to a single filesystem, which is the ordinary way to keep such a search cheap, **silently loses the snap**, because a snap is mounted from a loop device. So the cheap form of the only route that finds everything is also the form that misses one placement.

**A ceiling no route lifts:** a non-root login found three of four on the other machine and was refused the fourth outright.

**A real instance of a case the specs care about turned up by accident** — the apt build is **3.0.1, below the floor**, which the Render Settings spec says must appear in the list and never be the automatic pick.

**Not covered:** flatpak was not installed on either box, so that route is untested rather than absent in general. Neither box tested a non-root login **against a renamed binary**. A Blender inside a container, on a network mount, or in a user's home directory is untested.

### Fact 71 — making a small viewable copy of a frame, and what it costs

**Answered: yes, and the cost that matters is memory, not time.**

| | 1920×1080, 33 MB EXR | 10000×10000, 1.6 GB EXR |
|---|---|---|
| wall, warm | **0.47 s** | **1.18 s** |
| wall, cold page cache | not posed | **5.54 s** |
| reading the frame | 0.024 s | 0.571 s warm, **4.479 s cold** |
| scaling | 0.006 s | 0.162 s |
| writing the copy | 0.075 s | 0.076 s |
| **peak memory** | **337 MB** | **3.40 GB** |
| copy produced | 512×288, 5,426 bytes | 512×512, 7,184 bytes |

**Blender's own start-up is most of the small case** — 0.47 s wall against 0.105 s of actual work — so **the floor is about half a second per invocation whatever the frame is.** The large case is dominated by **reading** the file, and by memory: **3.40 GB peak for a 1.6 GB file**, because the frame is held uncompressed as float. On a 24 GB machine that is four such previews at once before trouble; on a smaller rental it is a real ceiling.

**The route is the research's**, which also supplied the control that produced a negative: the research records that the saving call writes in the **scene's** format whatever the path says. Run deliberately with the scene left on OpenEXR and the path ending `.jpg`, the output carried OpenEXR's magic number — **a file called `.jpg` that is an OpenEXR inside**, which is what Smoozy would ship to a preview pane if it set the path and forgot the format.

**Not covered:** only OpenEXR sources and JPEG and PNG outputs. **The cost of doing this while a render is using the card and the cores was not measured** — every figure is from an idle machine.

---

## What the connection carries — Facts 25, 75, 17, 15 and 72

### Fact 25 — what the link carries, and whether the limit is per server

**Answered in full, including the sharing half, and it corrects the project's own record by about tenfold.**

`The rented machine.md` gives **1.3 MB/s down and 0.48 up**. Measured against the same home line: **12–15 MB/s down and 1.2–1.3 up**. **Down is 11–12 times up, and that asymmetry is the dominant fact about this connection.** That record should stop being treated as the ceiling.

**Concurrency against one machine:** the knee is at **two streams** for download — one to two buys 53%, two to four buys 3%, four to eight buys 11% — and around **four** for upload, where the uplink saturates near 1.85–1.9 MB/s. **Two to four is the entire useful range**, and most of the gain is hiding a flat 1.0–1.4 s connection setup rather than adding bandwidth.

**The sharing half is the decisive result.** One machine alone gave 13.9–15.2 MB/s; a second machine alone gave 14.7–15.0. **Both at once gave 16.5–18.3.** A whole second rented machine bought about **20%, not 100%**, and each machine's own throughput roughly halved.

**So the limit is global — the home line — and not per server. A pool that scales with machine count measures the wrong thing.**

**But a per-server number is still needed, for a different reason.** One provider **could not fill its share of any budget** — 4.24 MB/s down at best and 0.35 up, dropping long fetches intermittently. With three machines running, **the slowest set the finish time**: 112 seconds for a job whose fast half was done in 28, with the line idle for 80. And on the upload direction, uncoordinated fan-out was **actively worse** than one machine at a time: 1.54 MB/s to one box, 1.24 across two, 0.99 across three.

**Not covered:** one home line on one evening over Wi-Fi. Two of the three rentals were the same provider.

### Fact 75 — whether one speed limit can hold across machines

**Answered: yes to both halves, with a catch that decides the design.**

Caps hold in both directions and across machines — 409 KB/s against a requested 400 on concurrent uploads to two boxes, and 402.8 and 405.5 KB/s on concurrent downloads against 9.44 MB/s uncapped on the identical transfer.

**The catch:** the limit is **per invocation**, so dividing a total budget statically only holds while every transfer is still running. With uneven sizes, a two-way 400+400 split against an 800 KB/s budget achieved **421.9 KB/s** — **half the budget forfeited the moment the small transfer finished.** A single shared budget in one process, same work, achieved **820 KB/s** and cut 21.85 seconds to 11.24.

**So a total cap needs one shared budget, not per-transfer caps.**

**Not covered:** the shared-budget mechanism was tested on uploads only, and **the download case differs materially** — a receiver enforces a cap by not reading, so buffering can let the wire rate overshoot.

### Fact 17 — resuming a part-sent transfer, both directions

**Answered: yes in both directions. The fetching direction was posed on one machine with two tools; the sending direction on two providers, with one tool.**

Resume was exact — 119,424,000 bytes where 157,286,400 minus 37,862,400 was owed, and 436,786,462 where 669,966,622 minus 233,180,160 was owed — with matching checksums. On a second provider, an interrupted transfer resumed in **12.85 seconds against a from-scratch control of 30.97**.

**The partial can be held off the final name**, which is the clause the spec leans on.

**Two conditions worth carrying.** **Apple's `rsync` now accepts every resume flag but one**, unlike the project's earlier record — but **the flag that holds a partial off the final name accepted the flag and resumed nothing on the sending direction on one box**, re-sending in full.

**And the partial directory cannot be relied on after a crash, though the reason is not settled.** A graceful interrupt produced it on both boxes. A hard kill produced it on one box and, on the other, left rsync's own **randomly named temporary file** in the destination instead — a name Smoozy cannot predict. **So it is not deterministic by signal alone**; the source's own guess is that it turns on which side noticed the disconnect first, and that is a guess rather than a finding. **What Smoozy can rely on is the partial directory after an interruption it performs itself.**

### Fact 15 — telling two files apart by their contents, and what it costs

**Answered with a genuinely large asset, as the fact requires** — a real 669,966,622-byte `.blend`.

Machine, warm: md5 0.89 s, sha256 2.24 s, sha512 1.53 s; a plain read took 0.34 s cold, about 1.97 GB/s. Mac: md5 1.26 s, sha256 2.22 s. **Every digest identical at both ends.**

**The finding worth keeping is a five-fold difference within one machine**: `openssl dgst -sha256` took **0.46 s** against `sha256sum`'s **2.24 s** — same file, same box. **The cost is processor, not disk**, and the same gap exists on the Mac in the same direction. A Smoozy wanting its own end-to-end check can have one at about a gigabyte a second if it picks the right tool — **on processors with the instructions that make it fast, which both ends here had and a rental may not.**

### Fact 72 — whether a transfer already tells a whole arrival from a damaged one

**Answered: partly, and the split is sharp.**

**Damage in the pipe is caught for free and hard.** With a relay flipping exactly one bit — and a control run first, flip disabled, proving the relay faithful — the connection died in both directions: `message authentication code incorrect` downloading, `Broken pipe` uploading. **But the failure is a dropped connection, not a "file is corrupt" message**, so it is indistinguishable from an ordinary network drop without reading the exit text. **And it leaves the same nameless partial under the final name.** The machine's own log said **nothing at all** — so Smoozy cannot learn of an integrity failure by reading the rental's log.

**Damage at rest is invisible.** A file with four bytes overwritten in place **and its modification time put back** — so size and time both match a whole arrival — travelled through `scp` perfectly at exit 0. And a default `rsync` **called it done**: 48 bytes sent, exit 0, file still corrupted, and **with a change summary requested it printed nothing at all**.

**The check need not be built, but it must be asked for.** Apple's `rsync` **does** support checksum comparison — the warning about it refusing resume flags does not extend to these — and it caught the four-byte wound, repaired it by sending **36,314 bytes**, and reported it machine-readably. The user's own end has nothing else: `sftp`'s 33 commands include **no checksum, verify, hash or digest command of any kind**, and `scp` has nothing either.

**Not covered:** compression was never on, only the default cipher suite was exercised, and corruption caused by the **disk** was not posed. Nothing was posed against Windows.

---

## What a transfer and a restart leave — Facts 18, 21, 29, 55 and 51

### Fact 18 — what a stopped transfer leaves, and whether anything distinguishes it

**Answered: it repeats, and it is worse than the record says.**

Killed and dropped transfers leave bytes at the final name with **identical mode, permissions, extended attributes, inode flags, allocation and file-type output** to a whole file. **Nothing on the machine distinguishes them.**

**And the new case is sharper than the old one.** An interrupted **overwrite** is not truncated: 40,000,000 bytes of one file overwritten by a killed transfer of another left a **40,000,000-byte hybrid** — 10,444,800 new bytes spliced onto 29,555,200 old ones. **Neither version, plausible size, no marker.**

**So anything relying on "the size looks wrong" is defeated.** The whole promise rests on Smoozy's own record, and the shape has to carry it.

### Fact 21 — telling that a file on the server has not changed since a transfer began

**Answered: yes, from size, inode, modification time and change time together.** An append moves size and modification time; a same-length overwrite moves modification time only; **a save-by-rename changes the inode**; and **a hand-restored modification time still moves the change time, which cannot be set back.**

Posed live: a byte was changed mid-fetch, **the copy returned 0 and delivered a different file**, and only the timestamps revealed it.

### Fact 29 — what names and paths survive between two filesystems

**Answered, and the failures are all on the Mac's side.**

All 33 hostile names — newline, tab, emoji, Cyrillic, Arabic, CJK, decomposed Unicode — **reached the machine byte-for-byte** by both transfer tools. The machine's limits: 255 **bytes** per name (85 CJK characters), case-sensitive, and it accepts non-UTF-8 sequences the Mac refuses. A path limit of 4096 bounds **the string handed to a system call, not the tree** — a 10,257-character path exists and ordinary tools walk it while a stat call fails.

**Coming back is where it breaks.** `Capitals.txt` and `capitals.txt`, and the composed and decomposed spellings of the same accented name, **each collapsed to one file on the Mac — one file's name carrying the other's contents, with no error.** That is a property of **this** Mac's disk, which ignores capitals by default; it would not happen on one set up to respect them, and Smoozy cannot assume either. And one tool **aborted the entire transfer** on the non-UTF-8 name where the other skipped it. The Mac's own archive tool broke it differently again: 32 sidecar files and a "File name too long" failure.

**Two operational notes:** Blender refuses a 256-byte output name at exit 1, and **giving two jobs the same label is refused by a plain make-directory call at exit 1, but the make-parents form accepts it silently — so the second job overwrites the first.**

### Fact 55 — what a restart stops, posed against a real render

**Answered against a real Cycles render, which matters, because the stand-in answer was incomplete.**

A frame here is about **16.8 seconds: ~15.4 sampling, ~1.4 writing**. Both feared outcomes are real, and which one you get depends on where the interrupt lands.

**Interrupted during the write**, a partial sits under the frame's real final name — 52,514,816 bytes carrying OpenEXR's real magic number, not sparse, **with nothing to mark it**. **So Blender does not write to a temporary name and rename**, which settles the open question the stand-in run could not. **Interrupted during sampling — about 92% of a frame — there is nothing at the path at all**: no partial and no zero-byte file. **There was no placeholder either, but that is a setting rather than a property** — the switch that writes one was off in every run here, as it has been in every script this project has run.

**A polite stop behaves identically to a hard kill.** Exit 143, partial left in place, no removal, no note — **and that is the signal a shutdown sends**, so it settles what a restart does mid-write.

**Two real machine restarts, both with a render in flight, both landed in sampling** despite the second being aimed deliberately at a write — **because the shutdown latency varied from 9 seconds to 6.4 between two runs minutes apart.** So a restart has roughly an **8% chance of leaving a partial and a 92% chance of leaving nothing**, and it cannot be aimed reliably even on purpose.

**Nothing announces the death.** The system log records only that it sent a termination signal to remaining processes, naming neither Blender nor the render nor its output. **The only evidence a render was killed rather than finished is the absence of the frames it had not written** — and, in the 8% case, a partial that looks like a finished small file.

**Not covered:** one output format and one picture size. **A dependency check in flight was not posed at all — that third clause of the fact is still open.**

### Fact 51 — restarting a machine over the connection Smoozy holds

**Answered: yes, but only with elevated access.**

**The ordinary non-root login cannot do it**: `Failed to reboot system via logind: Interactive authentication required.` **An SSH session is not a local seat, so the permission system refuses.** It needs a password, a route to root, or the root login.

Issued as root, the machine came back **in about 20 seconds at the same address and port**, with a new boot identifier proving a real reboot, the disk intact, and **the driver module reloading automatically with no intervention**. On another machine the same operation took **85 to 90 seconds**. **The address never changed on any machine.**

---

## Reading a machine — Facts 6, 7, 19, 20, 28 and 78

### Fact 6 — where writes sit, and what survives

**Answered, and the disk-choice half comes back "no choice here" on every one of the six.** All five virtual machines were single-disk. **Three of them carried a decoy** — a 366 KB device that is not storage but appears in any listing and must be filtered.

**The container is a separate case and a worse one.** It has one real filesystem of its own, but its `df` also reports **the host's disks**, mounted at paths like `/etc/hosts` and the driver binaries. **Anything reading free space by walking `df`, or by asking about a path that happens to be one of the bind-mounted driver files, gets the host's figures rather than its own.** The correct reading is against the actual output folder — which bears on Fact 28 as much as on this one.

**Writes to disk survive a reboot. `/tmp` does not.** The machine's own housekeeping **wipes `/tmp` at every boot** regardless of age, while **`/var/tmp` is not cleaned at boot and has a documented thirty-day expiry underneath — corrected 2026-08-20; this record said it was *never cleaned at all*, and a reboot test cannot tell "never" from "after thirty days"**. Ubuntu's own source for its jammy systemd package (`https://git.launchpad.net/ubuntu/+source/systemd/plain/tmpfiles.d/tmp.conf?h=ubuntu/jammy`, read 2026-08-20, identical to upstream at v249) carries `q /tmp 1777 root root 10d` and `q /var/tmp 1777 root root 30d`; `tmpfiles.d(5)` gives the age field as *"If a file or directory is older than the current time minus the age field, it is deleted."*; and the cleaner runs without being enabled — the shipped `timers.target.wants/systemd-tmpfiles-clean.timer` symlink, `OnBootSec=15min`, `OnUnitActiveSec=1d`. **On a rental that lives hours this costs nothing; it matters because the next sentence nominates `/var/tmp` as the durable scratch area**, and anything later put there to persist inherits an expiry nobody wrote down — read from the machine's configuration and from the system's own manual, then verified across a real reboot with markers in three places. **So `/var/tmp` is the only temporary area that both survives a reboot and sits on disk**, which also makes it an easy place to leave one rental's data for the next tenant.

### Fact 7 — what listing a folder yields, and which failures can be told apart

**Answered, and all four fields come back for every kind of entry**, with the time to nanosecond precision on the machine's own clock in UTC.

**The three failures are distinguishable — but only by reading the message, never the exit status.** *May not read* and *gone* both give exit 2 from a listing and exit 1 from a stat; only the text differs. *Did not answer* is a transport failure at exit 255 with no listing at all — **but exit 255 alone does not settle it**, since a remote command returning 255 gives the same number with clean stderr. **It is the accompanying text that separates them, not the number** — and detecting it at all needs keepalives switched on, or the caller hangs indefinitely.

**The two machines disagreed about the file-transfer route, and both were right about different cases.** Settled directly in one session: where the refusal happens at **readdir** — the directory reachable but unreadable — it says `Permission denied` and the cases are distinguishable; where it happens at **stat**, because the login cannot traverse a parent, it says `not found`, **byte-identical to a path that never existed**. **The stat case is the ordinary shape of "may not read it", so a client scraping that output cannot reliably tell the two apart.**

**One trap the fact does not name: size can be a lie.** A 500 GB sparse file reports 500 GB to a listing and occupies nothing; only the block count reveals it. **Any size shown to a user, any will-it-fit check and any transfer estimate must decide which number it means.**

### Fact 19 — noticing a change while staying connected

**Answered for both payloads, and it is free.** A render took 43.64 seconds watched against controls of 43.47 and 43.79, and the watcher cost 0.08 seconds of processor over 55. A file arriving by transfer is **indistinguishable in event shape** from a frame being written.

**One trap: a watch on a file follows the inode**, so a save-by-rename kills it silently, and a replaced folder gives one ignored-event and then nothing.

### Fact 20 — telling that a file is finished rather than still being written

**Answered by two routes at a large picture size, as the fact insists** — 10000×10000, where the window is 1.2–1.6 seconds rather than a fraction of one.

**And the clause the first run named as most likely to break did break, silently.** Watching the writing process works as root and **fails completely across a user boundary**: a non-root reader watching a root-written file got **160,151 refusals, zero matches, and "held by nobody" from an 18 MB file all the way to 1.6 GB** — the same answer the route gives for a finished file. **A Smoozy applying the rule would have called that frame finished at 18 MB**, having paid the full cost of the scan to get the wrong answer. The same-user control found 42 matches, proving it is the boundary and not the route.

**Two related tools fail the same way and lie quietly** — both return exit 0 and no rows for a file very much held open. **Not an error; an empty answer that reads exactly like "nobody has it".**

**The other route is unaffected**, because it watches the folder rather than the process. **So only one of the two routes survives a non-root Smoozy, and it is the one that cannot answer for a file finished before Smoozy connected.**

**And size stability is a trap**: a frozen transfer sat at 17,233,920 bytes for 11.1 seconds while unfinished.

**Not covered.** The non-root reader was a real non-root **process**, obtained without a separate login, so **this does not separately prove a non-root login can connect at all** on that box. Only one of the two ordinary accounts was tried. And **a `/proc` mounted to hide other users' processes, which some providers do, would hide the process listing too** and was not posed — on this machine nothing was hidden, so a non-root login could at least see *that* a render was running, only not which files it held.

### Fact 28 — reading free space, and what a reading must exclude

**Answered: yes, unprivileged.** A true reading must exclude memory-backed filesystems — **one machine advertised 29 GB free that was memory, not disk** — read-only package mounts that are permanently 100% full, the boot partition, the same filesystem counted twice, and **the root-reserved blocks**, which is why the available figure and not the free figure is the one to read.

**A fifth exclusion the fact does not name: apparent file size.** The 500 GB sparse file sat in the measured folder and moved the free-space reading by nothing. **Free space must never be computed by subtracting listed file sizes.**

**A sixth exclusion, on a container only: the host's disks.** A container's free-space listing also reports filesystems belonging to the host, mounted at paths like `/etc/hosts` and the driver binaries — **so a reading taken by walking that listing, or by asking about a path that happens to be one of them, is the host's figure and not the machine's.** Read against the actual output folder.


**A refused folder can be seen taking files again** — verified both by permissions and by genuinely filling a disk until writes failed with real errors, then freeing it.

**But the probe that says so must be a real write.** On a full filesystem, the writability test **returns writable**, and **creating an empty file succeeds** — an empty inode needs no data blocks. Only writing at least one byte fails. **And a failed copy left a zero-byte file behind**, which is exactly the ambiguity that matters: **a file of size zero is indistinguishable between "still being written", "the disk filled" and "the render wrote nothing".**

### Fact 78 — what the filesystem guarantees for publication and a pointer switch

**Answered on ext4.** Replacing a file by rename is atomic — the content became the whole new string and **the inode changed**, a whole-file swap that was never partial — and the system's own manual says so: *"If newpath already exists, it will be atomically replaced, so that there is no point at which another process attempting to access newpath will find it missing."* A symlink pointer switches atomically by the same means, and two names can be exchanged atomically.

**So publication into a final name cannot expose part-arrived bytes.** But **no single operation updates two independent files atomically** — the matching pointer-and-registration pair is a design obligation, built on single-rename atomicity plus ordering, not a filesystem primitive.

**Not covered:** no crash injection at the rename boundary; durability rests on the journal and an explicit directory sync.

---

## The rest of the endings — Facts 57, 58, 60 and 62

### Fact 57 — whether the side still holding a connection learns of its end

**Answered all three ways, and the spread is large.** The third way — the machine powered off from the provider's console — was posed by the runner using the provider's API, because nothing inside a machine can switch that machine off.

| How it ended | How the caller learned | How long |
|---|---|---|
| The machine closed it politely | told | same second |
| **Powered off from the provider's console** | **told** | **same second** |
| The caller's line pulled, keepalives on | detected | 56 s |
| The caller's line pulled, keepalives off | **not detected** | **silent 21 m 45 s** |

**A provider's stop is an orderly shutdown, not a power cut** — the machine's own service was alive long enough to close its sockets politely, so **both a session with keepalives and one without learned in the same second and the setting made no difference.**

**So the danger is never the machine going away; it is the caller's own connection failing.** Only a broken path needs keepalives, and without them a caller can sit for twenty minutes believing a dead connection is a slow one.

**The machine had not learnt within the 5 minutes 10 seconds it was watched** — it held the socket open with the job still running. **Corrected 2026-08-21: this said *never learns at all*, which the manual of the observed software contradicts.** That box ran `tcpkeepalive yes`, and `sshd_config(5)` (macOS 26.5 man page dated 2025-10-04, read on the Creator's Mac 2026-08-20) says *"The default is yes (to send TCP keepalive messages), and the server will notice if the network goes down or the client host crashes."* The kernel's own defaults set the ceiling — kernel.org `ip-sysctl`, read 2026-08-20: `tcp_keepalive_time` *"Default: 2hours"*, then 9 probes at 75 s, *"connection will be aborted after ~11 minutes of retries"*; and `tcp_retries2` *"yields a hypothetical timeout of 924.6 seconds and is a lower bound"* where the machine has anything to send. The window in `findings-hyperstack.md` runs from the blackhole at 21:07:40 to the socket inspection at 21:12:50. **Nobody watched for two hours, so the run measured what the defaults already predict.**

**Two side-findings.** A ten-minute silent gap made a connection **unrecoverable even after the path was restored** — almost certainly because the provider's address translation forgets faster than the protocol does, though **the actual timeout was not established**. And **one provider answers no ping at all** while its connection works perfectly — so **any ping-based health check would report it dead.**

**Not covered:** the genuinely abrupt case. A real power cut would look like the pulled-line row, and **no rented machine can pose it.**

### Fact 58 — whether an open connection survives its key file being deleted

**Answered: yes.** A copy of the key was made, a session opened with the copy, and the copy deleted; **the live session kept working, and a fresh connection with the missing key failed** — proving the key gated authentication and the survival is because the session no longer needs the file. The real key was never touched.

### Fact 60 — a render started under one login, seen and stopped from another

**Answered: seen yes, stopped no.** From a second ordinary login, the process is visible with its owner and command line, but both a polite and a forceful stop are refused: `Operation not permitted`. The same login then stopped it cleanly.

**So Smoozy can watch another login's render across a re-login, but to stop it must be the same account or hold elevated access.**

**Not covered, and stated plainly: no real Blender render was used.** That box had none, and the driver work that would have supplied one was blocked. A stand-in process is signal-identical to a render, but this leaves open whether a real Blender's child processes or its hold on the card change the answer. **The machines that did have Blender were not used for this**, so it should be re-posed on the next batch that has one.

### Fact 62 — reaching Telegram

**Answered yes on all six machines, across three providers and four countries** — every one answering a deliberately invalid token with Telegram's own rejection, which only the service itself can produce. **Four of the six also ran a negative control** that failed as expected; two did not, and their results rest on the rejection alone.

**One thing worth carrying:** on two machines the name lookup for Telegram **returned an IPv6 address first, or the probe read only one entry of the list**, while the machine had no global IPv6 and connected over IPv4. **The sentence written here said the lookup returned *only* an IPv6 address, and that is false and self-defeating — corrected 2026-08-20**: the name publishes both (`dig` on 2026-08-20 returned an `A` record and an `AAAA` record), and if only an IPv6 address had come back there would have been nothing for the transfer tool to fall back **to**, which the same sentence says it did. **What those two boxes actually saw is not settled** — the command was never recorded — and it rides free on any later rental. The transfer tool coped by falling back. **A client that resolves and then connects strictly on the first address family would fail there.**

**Not covered on any machine:** the *goes on reaching it* half. The longest window achieved was 45 minutes, on the container, across two restarts.

---

## Telegram — Facts 63 and 65

### Fact 63 — whether a webhook can replace the exclusive receiving slot

**Answered: yes, and a rented machine can be what one needs — with no domain name, no DNS entry, no certificate authority and nothing bought.**

**Telegram's own requirements**, read 2026-08-15 from its Bot API documentation: a public address, one of four ports, modern transport security, and a certificate whose name matches the address — **and the guide explicitly permits a bare address as that name for a self-signed certificate.**

**Both halves were then proved on a real machine.** The webhook registered against a self-signed certificate on a bare address — `{"ok":true,"result":true,"description":"Webhook was set"}` — and **Telegram genuinely connected**, from an address inside one of the exact two ranges its documentation names, completing the handshake against that self-signed certificate and holding the connection for sixty seconds before closing cleanly. **So the certificate is accepted in the real handshake and not merely at registration.**

**The inbound port was the whole obstacle, and it is a procurement question rather than a machine one.** On first run every port but 22 timed out and a packet capture on the machine caught **zero packets** — the block was the provider's security group, not the machine, which had been listening correctly all along. **Opening it took one API call from outside, applied to a running machine with no reboot**, after which the same capture caught the handshake arriving.

**But this must not be generalised.** Vast.ai **maps ports only at rental time**, does not let them be opened afterwards, and gave this batch's container **no spare port at all**. **So whether a rental can host a webhook is decided when it is rented.**

**Three findings that matter more than the yes.**

**Registration succeeding proves nothing.** Telegram accepted a registration for a completely unreachable address. Only asking it afterwards reveals the truth — and **the healthy and unreachable answers are byte-identical except that the unreachable one carries an error timestamp and message.** Address, certificate flag, connection limit and even the pending count are the same for both. **The health test is the presence of that error field**, which appeared within sixteen seconds. **That is one observation of one probe**, and Telegram appears to make it once on registration — no retry was seen over the following fourteen minutes, so nothing here says how quickly a webhook that breaks *later* would be reported.

**Exclusivity is confirmed word for word:** `409 Conflict: can't use getUpdates method while webhook is active; use deleteWebhook to delete the webhook first`. Every other method keeps working. **But that same 409 code also covers a different failure** — two pollers competing — **distinguishable only by the description text.**

**Not covered: the payload was never observed.** The Update body, the header Telegram sends with it, the retry cadence and the behaviour under load are all unposed. A correctly-shaped message was sent from the Mac to show what the receiving side does, and is **labelled in the trail as a simulation and not as Telegram traffic**.

**Why it was never observed is itself a finding, and it is not the obvious one.** Every reading during the run showed no updates waiting and an empty result from the ordinary receiving call, which was first written up as *no message ever reached the bot*. **That was wrong.** The Creator confirmed afterwards that they had sent many messages to that bot, and that Telegram showed them as **read** — which for a bot means an update was **consumed**, not that it never arrived.

**So the updates existed and something in this test consumed them without capturing them.** Which thing is not established: the receiving call was made several times during the run, and the webhook was registered and later deleted. Any of those consumes updates, and none of them recorded a body.

**That is worth more to the design than the payload would have been.** The Telegram module rests on several machines holding one token, and this is a live demonstration of the hazard underneath that: **an update belongs to whoever takes it first, and a holder that takes one removes it for every other holder, silently and with nothing left behind to show it happened.** A second holder polling for its own reasons — a leftover test, a stale server, a second copy of Smoozy — is enough to make messages disappear for the machine that was supposed to act on them.

**And whether updates queued before a webhook exists are delivered or lost is still open**, and now cannot be answered from this run at all, since the queue was emptied by something other than the endpoint. Telegram's own text says updates are held *"not longer than 24 hours"*, which implies delivery on registration — **but that is their documentation, not this machine's behaviour.**

**Closing both wants one rental with an open inbound port, and a rule for next time: nothing else may hold the token while the capture runs.**

### Fact 65 — whether Telegram's pace holds over hours rather than a minute

**Not answered, and it should be treated as unposed rather than thinly answered.** **The sending side the fact actually names was never exercised at all**, because no conversation with the bot ever existed, and thirty-eight minutes stood in for "hours and days".

**What was measured is worth keeping even so, as a floor rather than an answer.** Read calls sustained **6,360 requests in three unbroken minutes, every one answered**, with a burst reaching **91 a second**. The one refusal seen anywhere was on registering a webhook, and it **carried a wait-this-long field** rather than simply failing — which is the shape Smoozy would have to obey. **None of that is the sending rate**, which is the number the module actually needs, and it must not be read as one.

---

## Two things about the batch itself

**Two rentals were destroyed without being used.** One jammed on a proxy port a stale tunnel was holding, retrying `Error: remote port forwarding failed for listen port 17916` indefinitely. One sat pulling a six-gigabyte image and never produced a log at all. **A container rental takes finding**, and the one that worked used a bare `ubuntu:22.04` image on the reasoning — which the machine then confirmed — that a container gets its driver injected from the host rather than from the image.

**A per-instance key on the container could read the whole account.** It is described as per-instance, and it listed **every instance on the account** with models, prices and images, and could restart them. **Anything that gets code execution on a container rental gets an account-wide inventory read for free**, so it is a secret in its own right.

