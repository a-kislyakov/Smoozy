# Sources — Server Setup (Architecture)

*Where the outside-world facts read for [Preparation/Architecture/Server Setup.md](../Server%20Setup.md) came from. The document carries each claim's label in place; this file carries the citation, the exact words or values, the address, the date read, and what each source does **not** cover. Decisions Smoozy made get no entry here.*

**Three shapes, and there is no fourth.**

1. **A pointer to an input document of this stage** — a frozen observation document in `Preparation/Architecture/`, or the fact register that indexes it. The entry names that document, carries the fact's own words as it uses them, and says what the observation does not cover. This is the commonest shape here, because most of what this module rests on was read off rented machines by this project.
2. **A publication quoted directly** — a manual, a source file, a vendor's reference, a protocol specification. The entry carries the exact words, the address and the date read.
3. **No source in this project** — an assumption, or unresolved. The entry says what breaks if it is wrong and says plainly that no source was found, naming where the search went.

**Blender facts are not re-derived here.** Where the document leans on Blender, the entry names the joined answer or the comparison question already named for that claim — by this document, or by [Preparation/Specs/Sources/Server Setup.sources.md](../../Specs/Sources/Server%20Setup.sources.md), which is the project's own record of which Blender question each of this module's Blender claims came from. Nothing is assembled out of several topics, and no words of Blender's are copied in.

**Every machine claim carries the input documents' own ceiling**, which the document restates at *Do not read past the input documents*: a machine observation is a frozen snapshot of a named set of boxes on named days — *"Read it as this is what that box did, never as this is what a rented machine does."*

---

# Part 1 — What binds this before anything is decided

## Fact 51 — a rented machine restarts over the connection Smoozy already holds

**Shape 1, pointer.** [Facts](../Facts.md), Fact 51, answered **2026-08-13**; evidence of record [Batch A](../Batch%20A%20—%20the%20shape%20throwaway.md). Label: **observed in a test**. Used by the opening paragraph and by [T1](../Server%20Setup.md#8-the-restart).

**The fact's own words.** *"`systemctl reboot` over the connection Smoozy already held; the connection reported `closed by remote host` and the machine answered again in about ten seconds at the same address and port, with a new boot id and the disk intact."* The run itself, in the input document: *"The connection reported `Connection to 31.59.39.104 closed by remote host.` and the machine **answered again in about ten seconds**, with a new boot id. The connection could be re-established at the same address and port."*

**Not covered by the 2026-08-13 run:** another provider, another kind of rental, and whether the driver work in the ready action requires a restart at all (that is Fact 80, below). The register also records that the run had to be made **last** on its box, because a box that does not come back ends the batch.

**A second seller now agrees, and the entry below carries it** — *The multi-provider batch*, Test 17.

## Fact 51's narrowing — the restart is refused to an ordinary login

**Shape 1, pointer.** [Facts](../Facts.md), Fact 27, under *What it costs where it is absent*: restarting the machine is refused to an ordinary login — **`Failed to reboot system via logind: Interactive authentication required.`** Label: **observed in a test**. Used by the opening paragraph, which concludes from it that **the restart is an elevated act**.

**Not covered.** *"because an SSH session is not a local seat"* is the document's **reading** of that message, not a quotation from anything read: **no publication on `logind`, seats or sessions was read by this project**, and none is cited anywhere in `Preparation/`. Nothing turns on the reading — the observed refusal, and the elevated route [E3](../Server%20Setup.md#9-elevation-the-refusal-and-the-password) fixes, stand whatever the mechanism is. Label of the reading alone: **an assumption**.

## Fact 50 — a second ready action on a half-finished machine, answered NO

**Shape 1, pointer.** [The last two boxes](../The%20last%20two%20boxes%20—%20the%20hook%20that%20cannot%20be%20read%20back,%20the%20full%20disk,%20and%20the%20unit%20that%20kills%20a%20render.md) §16; [Facts](../Facts.md), Fact 50, answered **2026-08-18**. Label: **observed in a test**. Used by the opening paragraph and by all of [Section 4](../Server%20Setup.md#4-interruption-resumption-and-the-safe-re-run).

**What it answered, in the register's words.** *"A second ready action was not refused."* No error and no refusal — a second setup worker started and got its own recorded execution group *"while the machine's own state still named only the first run"*. It then *"walked all five boundaries from the beginning and, on reaching the end, cleared the machine-level state itself"*, leaving a machine that had genuinely completed two of five steps **marked ready** by a run nobody authorised. **The card was unharmed** — *"the harm this fact is about is the state, not the hardware"* — and the documented recovery behaved correctly: clearing under the original identity worked and a fresh run afterwards started cleanly with a new identity.

**Not covered.** The negative was taken against **a stub setup with no guard**, run last on a box being given up. It says nothing about a run whose admission point holds the state; that is exactly what [S4](../Server%20Setup.md#42-the-recovery-run) is built out of, and the register's five boundaries are the stub's rather than [Y4](../Server%20Setup.md#32-the-recorded-safe-boundaries)'s list.

## Fact 48 — the container rental, and one seller's claim about its own instances

**Shape 1, pointer, with the norm claim's own publication carried by the input document.** [Six machines](../Six%20machines%20—%20the%20container,%20the%20webhook%20and%20the%20ready%20action.md), *Fact 48*, on a container rented specifically to pose it, **2026-08-15/16**; [Facts](../Facts.md), Fact 48. Label: **observed in a test**, with the norm claim **documented**. Used by the opening paragraph, [M2](../Server%20Setup.md#1-the-machine-kind-and-what-a-setup-can-be-on-each), [Section 2](../Server%20Setup.md#2-the-ready-action)'s constraint line, [R3](../Server%20Setup.md#21-what-the-action-is-made-of), [R8](../Server%20Setup.md#24-the-shadowed-user-space-half), [T1](../Server%20Setup.md#8-the-restart) and [W1](../Server%20Setup.md#10-what-this-module-says-about-a-machine).

**The norm claim's publication, as the input document carries it.** Vast.ai's own documentation, read **2026-08-15**: *"Instances are almost always Docker containers; a small subset are virtual machines."* (`docs.vast.ai/guides/concepts.md`), and *"The standard and most common way to run instances on Vast.ai is using Docker containers."* (`.../templates/introduction.md`).

**The reach of that claim, narrowed in the document on 2026-08-23.** **It is one seller's own documentation about its own instances and covers nothing beyond that seller.** [W1](../Server%20Setup.md#10-what-this-module-says-about-a-machine) used to carry it as *"the commonest rental kind"*, a property of rentals; the three-seller batch of 2026-08-22 delivered **KVM virtual machines on the other two sellers** — machine C on Hyperstack and machine D on Sesterce — beside Vast's containers ([The multi-provider batch](../The%20multi-provider%20batch%20—%20the%20surface%20that%20did%20not%20vary,%20the%20Debian%20route%20that%20exists,%20and%20the%20cards%20a%20slice%20cannot%20see.md)). **Nothing in the document turns on which kind is commoner**: every rule is written per kind and the kind is read at every placement (K2), so the narrowing costs no clause.

**[Section 2](../Server%20Setup.md#2-the-ready-action)'s constraint line was narrowed the same way on 2026-08-23**, on the runner's ruling, W1's implementer having flagged it as outside its own findings. It read *"pretending otherwise would be wrong on the commonest rental kind"* and now reads *"wrong on the container kind, which is observed rather than assumed"*. **The constraint itself does not move** — what the ready action is made of differs by machine kind whichever kind is commoner, and Section 2 composes the action per kind either way. **What is added to this entry's *not covered*:** which machine kind is commoner across the rental market is **not established by anything this project holds**, and no clause in this document may be written against it.

**The opening paragraph was narrowed the same way on 2026-08-23**, on the runner's ruling, and this entry's heading with it. The paragraph read *"A container rental is the norm and nothing on it is open at all"* and now opens *"On a container rental nothing is open at all"*; the heading read *"the container rental, and it is the norm"*. **The container observations that paragraph carries are untouched** — the capability count, the four closed driver routes, the restart that is not a recovery action, and the non-answering card being terminal for that server are all that one rental's own, and none of them depends on how common the kind is.

**What the rental itself answered.** Confinement is capability dropping plus seccomp rather than a user namespace: root inside holds **13 of 41 capabilities** (`CapPrm/CapEff/CapBnd = 00000000a80405fb`), without `CAP_SYS_MODULE`, `CAP_SYS_ADMIN`, `CAP_SYS_RAWIO`, `CAP_SYS_PTRACE` or `CAP_SYS_BOOT`. **Four routes to the driver, all closed** — a driver package fails on the bind mounts (*"unable to make backup link ... Invalid cross-device link"*) and poisons apt afterwards; module load and unload return `EPERM` on every route including the raw syscalls; writing `/proc/driver/nvidia` gives `EACCES` from the driver's own write handler; replacing the user-space libraries in place gives `EROFS`. **A restart is not a recovery action there** — across two container restarts the host's uptime rose monotonically, so the host never rebooted and the driver was never reloaded.

**The one dangerous gap, which [R8](../Server%20Setup.md#24-the-shadowed-user-space-half) is written against.** The user-space library symlinks are ordinary files in the writable overlay, so the user-space half **can be shadowed**: under a deliberate mismatch Blender rendered on the processor and exited 0 with the device setting still reading `GPU`.

**Not covered.** *"No actual card hang was induced; what is established is that if one occurs, none of the four repair routes is open."* **The observed capability set is the container runtime's default**, and a provider running privileged, or adding `CAP_SYS_ADMIN`, would answer this fact differently. Whether one machine commonly hands out several containers is a separate fact, kept apart deliberately and not closed by this one.

## Fact 47 — what a rented machine needs beyond a driver

**Shape 1, pointer.** [Six machines](../Six%20machines%20—%20the%20container,%20the%20webhook%20and%20the%20ready%20action.md), *Fact 47*, four freshly rented machines, **2026-08-15/16**; [Facts](../Facts.md), Fact 47. Label: **observed in a test**. Used by the opening paragraph, [R1](../Server%20Setup.md#21-what-the-action-is-made-of) item 4 and [R5](../Server%20Setup.md#22-what-the-action-never-does).

**What it answered.** *"There is no single answer: it is a property of the image the provider ships, not of rented machines."* The four results were **nothing, two things, four things and seven things**; *"that spread is the finding."* All four agree on the mechanism: the blocker is the X11 and OpenGL shared-library chain, and **`libgl1` is needed even for a background render**, because the executable links `libGL.so.1` at load time — *"the failure is at launch, not at render."* **Blender names only the first missing library**, so *"a ready action must read the whole list at once, from `ldd`, and never work from the launch error."* One trap: on one machine the NVIDIA metapackage *"incidentally supplied six of the nine missing libraries"* — *"so a ready action must install these explicitly and never rely on the driver's dependencies."* No CUDA toolkit was ever needed on any machine; `libnvoptix.so.1` and `libcuda.so.1` come with the driver package.

**Two traps of one image's own, recorded beside it:** `ubuntu-drivers install --gpgpu` **does not install `nvidia-smi`** and **omits `libnvoptix`**, so OptiX fails with `Error: Found no Cycles device of the specified type` while CUDA works.

**Not covered.** The **list itself cannot be settled** — it is per image, and four further images on three providers are still owed. One of the four broke the fact's own precondition (asked sixth on an already-used machine rather than first), so *"a strict reading would want Hyperstack's re-posed."*

## Fact 80 — the driver work can require a restart, and the flag is not the signal

**Shape 1, pointer.** [Six machines](../Six%20machines%20—%20the%20container,%20the%20webhook%20and%20the%20ready%20action.md), *Fact 80*, two machines, **2026-08-15/16**; [Facts](../Facts.md), Fact 80. Label: **observed in a test**. Used by the opening paragraph, [R1](../Server%20Setup.md#21-what-the-action-is-made-of) items 5 and 6, [R5](../Server%20Setup.md#22-what-the-action-never-does), [Y5](../Server%20Setup.md#32-the-recorded-safe-boundaries) and [T6](../Server%20Setup.md#8-the-restart).

**What it answered.** Yes on both machines, *"by two different mechanisms — and they disagree about the signal, which is the useful part."* One needed a restart because the provider left the card bound to `nouveau`: the install finished at **exit 0 in 85 seconds, printed no reboot demand**, left `nvidia-smi` failing at exit 9, and the kernel said why — `NVRM: GPU 0000:00:06.0 is already bound to nouveau.` The other needed one because the install pulled a whole new kernel and **did** print its own demand. *"So Ubuntu's own reboot-required flag is not a reliable signal — it was never set on the machine that needed a restart most. The rule that survives both: run `nvidia-smi` after installing and restart only if it fails. Never unconditionally, and never on the package manager's word."*

**The unload-and-reload alternative, which [R5](../Server%20Setup.md#22-what-the-action-never-does) forbids.** A full module unload and reload brought the card up and it *"broke again on the very next command"* (`Failed to initialize NVML: Unknown Error`), the reload having left device nodes with the wrong majors; the state then oscillated, *"working and inconsistent at once."*

**Restart cost.** Measured eight times across three machines: **9, 11 and 12 seconds** on one, **21, 22 and 28** on another, **85 to 90** on a third. *"Anything that budgets a fixed wait for a machine to return will be wrong on most of them."*

**Not covered, and it is a correction the register carries.** NVIDIA's *"You will need to reboot your system after adding these configuration directives"* was **withdrawn as support for this conclusion on 2026-08-20** — it answers a different question, and *"the sentence immediately before it says the opposite in NVIDIA's own words"*. The decision does not change; the reason given does. The register also narrows *"no reading taken in that state can be trusted"*: it is written as a permanent property of what is an ordinary repairable state, it names the repair NVIDIA does document — `nvidia-modprobe` creates the character device files, and only *"when … the NVIDIA character device files do not yet exist"*, so a stale file that does exist is not replaced — and it records that **that repair is undocumented in operation and unposed here**. **The document used to repeat the untrusted-reading rule at its full width and was brought into line on 2026-08-23**: it now states the rule as what this document does with such a reading, carries the register's narrowing in place, and says why nothing below moves — [R5](../Server%20Setup.md#22-what-the-action-never-does) forbids the unload and reload outright, so no act of Smoozy's enters that state and none repairs it.

## Fact 27 — what elevated access a rental permits, at both weights

**Shape 1, pointer.** [The last two boxes](../The%20last%20two%20boxes%20—%20the%20hook%20that%20cannot%20be%20read%20back,%20the%20full%20disk,%20and%20the%20unit%20that%20kills%20a%20render.md) §10 for the ready-action weight, **2026-08-18**; [Six machines](../Six%20machines%20—%20the%20container,%20the%20webhook%20and%20the%20ready%20action.md) for the placement weight, **2026-08-15/16**; [Facts](../Facts.md), Fact 27. Label: **observed in a test**. Used by the opening paragraph, [M2](../Server%20Setup.md#1-the-machine-kind-and-what-a-setup-can-be-on-each)'s table, [M4](../Server%20Setup.md#1-the-machine-kind-and-what-a-setup-can-be-on-each), [E1](../Server%20Setup.md#9-elevation-the-refusal-and-the-password) and [E4](../Server%20Setup.md#9-elevation-the-refusal-and-the-password).

**On a systemd rental, in two halves.** *"No elevated access is needed for the placement at all"* — writing the user unit, enabling it, granting lingering to its own login, the linger read-back and the registration confirmation all ran unprivileged and all succeeded. *"Root is needed for everything about preparing the machine to render"*, and on that rental it was not hypothetical: the card arrived unusable and the repair was a driver package plus a reboot, refused outright to the supervisor's login. **Passwordless root exists on the image and is held by the cloud-init account rather than by the login Smoozy runs as** — `ubuntu ALL=(ALL) NOPASSWD:ALL` in the image's own drop-in — which is what [M4](../Server%20Setup.md#1-the-machine-kind-and-what-a-setup-can-be-on-each) refuses to treat as a route.

**On a container, nothing is open at all from a non-root login.** *"Elevation demands a password the supervisor's login does not have, switching to root fails authentication, the capability set is empty, and each of the three things a ready action needs is refused — installing software, installing the boot hook, and writing anywhere system."* The register corrected this on **2026-08-22** from *there is no elevation route at all from a non-root login* to *the route demands a password Smoozy does not hold*; the document is written on the corrected form.

**Not covered.** *"The permissions read inside the container are the ordinary ones for that kind of rental, and a provider handing out a container with more would answer differently; nothing here can rule that out."* Nothing in the fact says what a **different** image's sudoers policy grants.

## Fact 49 — a machine can be read for how far a half-finished setup got

**Shape 1, pointer.** [The last two boxes](../The%20last%20two%20boxes%20—%20the%20hook%20that%20cannot%20be%20read%20back,%20the%20full%20disk,%20and%20the%20unit%20that%20kills%20a%20render.md) §9, **2026-08-18**; [Facts](../Facts.md), Fact 49. Label: **observed in a test**. Used by the opening paragraph, [Y3](../Server%20Setup.md#32-the-recorded-safe-boundaries), [Y6](../Server%20Setup.md#32-the-recorded-safe-boundaries), [S1](../Server%20Setup.md#41-what-a-part-finished-setup-reports) and [S5](../Server%20Setup.md#42-the-recovery-run).

**What it answered.** Yes. *"Cut at a recorded safe boundary, the machine named the third of five by name with the moment it was reached"*, and the recorded outcome. *"Cut hard, mid-step rather than at a boundary, it named the second of five — which is right: it names the last boundary actually passed, not the one it died in."* A job submitted while the state stood **was never admitted**, before or after a reboot, and *"the state survives a reboot with no worker behind it"*, still naming its boundary and its moment.

**The one reading named wrong — the defunct-entry hole.** *"Immediately after the hard cut, with no setup worker alive at all, the machine reported the recorded pair as still validating; only the reboot, which reaped the entry, flipped it."* The cause recorded there is that the supervisor never waits on the workers it starts. [S5](../Server%20Setup.md#42-the-recovery-run) leans on the reading only in the direction that holds — it can report a dead group live, never a live one dead.

**Not covered.** A machine that recorded **nothing**, where the state cannot be worked out at all. The five boundaries were the stub's own; nothing here validates [Y4](../Server%20Setup.md#32-the-recorded-safe-boundaries)'s composed list.

## The observation ceiling this document quotes

**Shape 1, pointer.** The closing sentence of *What binds this* — *"Read it as this is what that box did, never as this is what a rented machine does"* — is the input documents' own standing caution, carried by [Six machines](../Six%20machines%20—%20the%20container,%20the%20webhook%20and%20the%20ready%20action.md), [The last two boxes](../The%20last%20two%20boxes%20—%20the%20hook%20that%20cannot%20be%20read%20back,%20the%20full%20disk,%20and%20the%20unit%20that%20kills%20a%20render.md), [Two more boxes](../Two%20more%20boxes%20—%20the%20volume,%20the%20full%20disk,%20and%20a%20day%20left%20running.md) and [The multi-provider batch](../The%20multi-provider%20batch%20—%20the%20surface%20that%20did%20not%20vary,%20the%20Debian%20route%20that%20exists,%20and%20the%20cards%20a%20slice%20cannot%20see.md). Label: **observed in a test**, per box.

**The two counts now agree, corrected 2026-08-23.** The document's binding sentence gave *one to six boxes on one to three days* — the width before the multi-provider batch of 2026-08-22 — while [Section 13](../Server%20Setup.md#13-unresolved) item 13 already gave the widened **one to nine boxes on one to four days, across three sellers**. The widening had reached the unresolved list and not the binding sentence; the binding sentence now carries item 13's count.

**Not covered:** the count is of boxes and days and says nothing about how representative they are, which is the whole point of the caution it introduces.

---

# Part 2 — Section 1: the machine kind and the distribution family

## `os-release(5)` — how a Linux machine says which distribution it is

**Shape 2, publication.** **Read 2026-08-22** at `https://manpages.debian.org/bookworm/systemd/os-release.5.en.html` (systemd 252.36-1~deb12u1, Debian bookworm). Label: **documented**. Supports [M1a](../Server%20Setup.md#1-the-machine-kind-and-what-a-setup-can-be-on-each), the distribution-family reading the Creator's ruling of 2026-08-22 made necessary.

**Where the file lives, and which one wins:**

- *"The /etc/os-release and /usr/lib/os-release files contain operating system identification data."*
- *"The file /etc/os-release takes precedence over /usr/lib/os-release. Applications should check for the former, and exclusively use its data if it exists, and only fall back to /usr/lib/os-release if it is missing. Applications should not read data from both files at the same time."*

**How it is read, and why it is never sourced as a shell script:**

- *"The format of os-release is a newline-separated list of environment-like shell-compatible variable assignments. It is possible to source the configuration from Bourne shell scripts, however, beyond mere variable assignments, no shell features are supported (this means variable expansion is explicitly not supported), allowing applications to read the file without implementing a shell compatible execution engine."*

**The two fields the family test uses:**

- `ID=` — *"A lower-case string (no spaces or other characters outside of 0–9, a–z, \".\", \"_\" and \"-\") identifying the operating system, excluding any version information and suitable for processing by scripts or usage in generated filenames. If not set, a default of \"ID=linux\" may be used."* Examples given: *"ID=fedora"*, *"ID=debian"*.
- `ID_LIKE=` — *"A space-separated list of operating system identifiers in the same syntax as the ID= setting. It should list identifiers of operating systems that are closely related to the local operating system in regards to packaging and programming interfaces, for example listing one or more OS identifiers the local OS is a derivative from."* And, on when to consult it: *"Build scripts and similar should check this variable if they need to identify the local operating system and the value of ID= is not recognized."* Its own worked example is the case M1a's withdrawn second arm turned on: *"For an operating system with \"ID=ubuntu\", an assignment of \"ID_LIKE=debian\" is appropriate."* The field is stated to be **optional**.

**What this carries for the document:** the family can be read from a documented, standard interface rather than probed; the precedence between the two paths is the standard's own rather than a choice; the fall-back from `ID` to `ID_LIKE` is the standard's own recommended practice for exactly this purpose; and the file is parsed as assignments rather than executed, which also keeps a machine-written value out of any command under `Server protocol and transfers` 1.1.

**Not covered:** this was read on no rental. That `ID` is absent on no ordinary rented image is **not** established — the standard makes both `ID` and `ID_LIKE` optional and gives `ID=linux` as a default that *"may be used"*, which is why M1a refuses an unreadable or unrecognised answer rather than defaulting it. Nothing here says what any particular provider's image carries. **The document's narrowing — reading `ID_LIKE` and refusing to admit on it — is Smoozy's decision and is not this standard's advice**; the document states that itself.

## What three sellers stock, read from their own image lists

**Shape 1, pointer.** [The multi-provider batch](../The%20multi-provider%20batch%20—%20the%20surface%20that%20did%20not%20vary,%20the%20Debian%20route%20that%20exists,%20and%20the%20cards%20a%20slice%20cannot%20see.md), *What the three sellers turned out to be*, image lists read **2026-08-22**. Label: **observed in a test**. Used by [M1a](../Server%20Setup.md#1-the-machine-kind-and-what-a-setup-can-be-on-each)'s last-but-two bullet.

**What it answered.** *"Sesterce sells no Debian at all"* — its image list, read from its own API on 2026-08-22, offers *"Ubuntu and AlmaLinux in every region, and Debian in none"*, and it delivered an Ubuntu machine. *"Hyperstack, by contrast, offers Ubuntu, Debian and AlmaLinux; Vast offers whatever image the renter names, which is why a Debian container is trivially available there and a Debian virtual machine is not."* The input document draws the same conclusion the document does: *"it is the first observation that a seller may not offer a family Smoozy supports"*, and nothing breaks, because the supported pair is what Smoozy prepares rather than what a seller stocks.

**Not covered:** one day, three sellers, and a list read rather than a rental attempted for each family. Whether a seller's list is the whole of what it rents is not established — the Whole app spec states the same caution for the sampled-offers figure, *"a sample of what it rents and not the whole of it"*.

## The Debian driver route, run end to end

**Shape 1, pointer.** [The multi-provider batch](../The%20multi-provider%20batch%20—%20the%20surface%20that%20did%20not%20vary,%20the%20Debian%20route%20that%20exists,%20and%20the%20cards%20a%20slice%20cannot%20see.md), *Test 13*, on a rented `Debian 12` machine with an L40 and no driver at all, **2026-08-22**. Label: **observed in a test**. Used by [M1b](../Server%20Setup.md#1-the-machine-kind-and-what-a-setup-can-be-on-each), [R1](../Server%20Setup.md#21-what-the-action-is-made-of) item 5 and [Section 13](../Server%20Setup.md#13-unresolved) item 22's closure.

**What it answered.** The route exists and is not Ubuntu's. **The image enables only `main`**, so `apt-cache policy nvidia-driver` returns `Candidate: (none)` — the driver is unreachable rather than merely uninstalled — until `contrib non-free non-free-firmware` are enabled. **`ubuntu-drivers-common` is unpackaged there**: `apt-cache policy` returned nothing at all. **`nvidia-detect`** names the package in its own words — `Detected NVIDIA GPUs:` … `Your card is supported by the default drivers.` … `It is recommended to install the` / `    nvidia-driver` / `package.` — and `apt-get install nvidia-driver` follows. **The restart is not optional on this family**: the running kernel was `6.1.0-31-amd64` and `linux-headers-6.1.0-31-amd64` **is not in the archive at all**, so the install pulled a newer kernel and DKMS built against `6.1.0-52-amd64`; until the boot, `nvidia-smi` reports `NVIDIA-SMI has failed because it couldn't communicate with the NVIDIA driver.` and `modprobe nvidia` reports `FATAL: Module nvidia-current not found in directory /lib/modules/6.1.0-31-amd64`. After the restart: kernel `6.1.0-52-amd64`, driver `535.261.03`, the card named and sized, `nvidia_uvm`, `nvidia_drm`, `nvidia_modeset` and `nvidia` all loaded. **149 seconds for the install plus one restart.**

**Not covered:** one image on one seller, one card. That `ubuntu-drivers-common`'s absence from Debian is permanent is **a reading of one image**, which [Section 13](../Server%20Setup.md#13-unresolved) item 22 states. Nothing here says what the route does on a card newer than the L40 — item 18.

## `unattended-upgrades` is the image's property, not the family's

**Shape 1, pointer.** [The multi-provider batch](../The%20multi-provider%20batch%20—%20the%20surface%20that%20did%20not%20vary,%20the%20Debian%20route%20that%20exists,%20and%20the%20cards%20a%20slice%20cannot%20see.md), *Test 14*, **2026-08-22**, both sides read the same day. Label: **observed in a test**. Used by [M1b](../Server%20Setup.md#1-the-machine-kind-and-what-a-setup-can-be-on-each)'s second paragraph and by [Section 13](../Server%20Setup.md#13-unresolved) item 23's answer.

**What it answered.** Hyperstack's `Debian 12 Plain` carries it: `dpkg -l unattended-upgrades` → `ii  unattended-upgrades 2.9.1+nmu3`; `/etc/apt/apt.conf.d/20auto-upgrades` reads `APT::Periodic::Update-Package-Lists "1";` and `APT::Periodic::Unattended-Upgrade "1";`; `apt-daily.timer` and `apt-daily-upgrade.timer` are both **enabled**, one **17 minutes from firing** when read. Vast's `ubuntu:24.04` container carries the opposite — not installed, no `20auto-upgrades` file, no timers, and no service manager to run them. The input document states the bound itself: *"this is a property of the `Debian 12 Plain` image Hyperstack ships, not a property of Debian."*

**Not covered:** two images on two sellers. Neither family may be read as carrying or not carrying it, which is why [S7b](../Server%20Setup.md#43-why-each-step-is-safe-to-run-again) probes the lock rather than assuming it.

## `tar` is Essential and `xz-utils` is not — **the address gap is closed**

**Shape 2, publications, four of them, read 2026-08-23.** Label: **documented**. Used by [R1](../Server%20Setup.md#21-what-the-action-is-made-of) item 3 for what *"is settled … on either side"*, and reused by [M1b](../Server%20Setup.md#1-the-machine-kind-and-what-a-setup-can-be-on-each): *"`tar`'s Essential status is Debian Policy's rather than Ubuntu's"*.

**The standard's own definition.** Debian Policy Manual **v4.7.4.1**, §3.8 *Essential packages*, `https://www.debian.org/doc/debian-policy/ch-binary.html#essential-packages`, read **2026-08-23**:

- *"Essential is defined as the minimal set of functionality that must be available and usable on the system at all times, even when packages are in the 'Unpacked' state."*
- *"Since these packages cannot be easily removed (one has to specify an extra force option to `dpkg` to do so), this flag must not be used unless absolutely necessary."*

**The package pages.** Read **2026-08-23**:

- `https://packages.ubuntu.com/noble/tar` — *"Package: tar (1.35+dfsg-3ubuntu0.4 and others) [security] [essential]"*. Its **Suggests** list carries `bzip2`, `ncompress`, `tar-doc`, `tar-scripts` and **`xz-utils`**; its **Depends** list does not.
- `https://packages.ubuntu.com/noble/xz-utils` — *"Package: xz-utils (5.6.1+really5.4.5-1ubuntu0.3 and others) [security]"*. **No `[essential]` tag.**
- `https://packages.debian.org/bookworm/tar` — *"Package: tar (1.34+dfsg-1.2+deb12u1) [essential]"*, the tag itself linking to the Policy anchor above; **Depends** `libacl1`, `libc6`, `libselinux1`; **Suggests** `bzip2`, `ncompress`, `tar-doc`, `tar-scripts`, **`xz-utils`**.
- `https://packages.debian.org/bookworm/xz-utils` — *"Package: xz-utils (5.4.1-1+deb12u1)"*, **no `[essential]` tag**.

**Every clause the document states held word for word on reading.** Nothing was corrected.

**How this entry read before.** *"No address and no date read exist anywhere in `Preparation/` for any of it"*, labelled **an assumption**, having searched this document, [Facts](../Facts.md), every input document, every other architecture document and sources file, the Specs and their sources files, and Groundwork. **`packages.debian.org` is recorded elsewhere in this project as a site that refuses retrieval; it did not refuse this read**, and both its pages answered.

**Not covered.** One Debian suite (bookworm) and one Ubuntu series (noble); Essential membership is a per-suite property and a different suite answers for itself. The Policy definition is the standard's and says nothing about what any **provider's image** actually carries — a rental is an image rather than an ordinary install, and [R1](../Server%20Setup.md#21-what-the-action-is-made-of) item 3's step probes rather than assumes either way. The `xz` half never rested on the packaging claim at all: `xz`'s absence was **observed** on a real image (entry below).

## `xz` absent, `lspci` absent, and the ten link libraries

**Shape 1, pointer.** [The multi-provider batch](../The%20multi-provider%20batch%20—%20the%20surface%20that%20did%20not%20vary,%20the%20Debian%20route%20that%20exists,%20and%20the%20cards%20a%20slice%20cannot%20see.md), *Test 6*, **2026-08-22**. Label: **observed in a test**. Used by [R1](../Server%20Setup.md#21-what-the-action-is-made-of) items 1, 3 and 4, and by [E7](../Server%20Setup.md#9-elevation-the-refusal-and-the-password)'s mapping requirement.

**What it answered.** **Absent from the image:** *"`lspci`, `cron` and `crontab`, `fuser`, `lsof`, `xz`, `zstd`, `bzip2`, `unzip`, `zip`, `aria2c`, `screen`, `needrestart`, `at`."* `lspci` was *"absent on every Vast container in this batch — A, E and F — and present on the Hyperstack virtual machine"*, so *"on a container it is not there, and the presence reading has no route unless something installs `pciutils` first."* Blender ships as `.tar.xz` and **`xz` is absent**, so the archive cannot be unpacked until `xz-utils` is installed. Blender also **links libraries the image does not carry** — the binary failed with `error while loading shared libraries: libX11.so.6: cannot open shared object file: No such file or directory` until `libx11-6 libxi6 libxxf86vm1 libxfixes3 libxrender1 libgl1 libegl1 libsm6 libice6 libxkbcommon0` were installed, **which took 17 seconds**. The input document draws Fact 47's own conclusion again: *"the list is a property of the image and cannot be fixed in advance."*

**Not covered:** three container images on one seller and one virtual machine on another. **The ten packages are the mapping's first entry rather than the mapping**, which is exactly what [R1](../Server%20Setup.md#21-what-the-action-is-made-of) item 4 says of them.

**A second, independent entry for both halves — *Builds 10*. Added 2026-09-02, used by [B7b](../Server%20Setup.md#52-a-build-the-user-points-at).** Source: question 10 of `Preparation/Blender Research/Comparison/Builds.md`, the joined answer for all four versions. Label: **observed**, one deliberately bare rented Linux machine, run in all four versions.

- **The tool.** *"On a machine holding GNU tar 1.34 and no `xz`, all four archives refused with the same message — tar could not run `xz` — at exit code 2, and nothing landed on disk in any of the four. Naming the compression outright made no difference."* One further package took the machine from 253 installed packages to 254 and all four then opened with no message at all. **The archive is a tar inside xz compression in all four**, read off the files' own first bytes.
- **The silence behind it.** Every version's manual documents that TAR must be used and 7-zip is not supported, and **no version's manual names the archive's format anywhere**, so what TAR must in turn decompress is stated in none of them. The comparison records the requirement as **incomplete in all four alike, not in one of them.**
- **The libraries.** With the archives open and nothing else installed, **none of the four would start**; each named `libX11.so.6` first and the machine could not satisfy **nine distinct libraries across ten lines, the same nine in every version**. Nine packages were asked for, the machine pulled in **forty-two**, and all four then started with every build hash matching the Creator's Mac. *"Nothing about the unpacking needed any of those libraries, and the program needs all of them."*
- **7-zip.** Two passes over each archive both reported success and exited cleanly in all four versions; the runnable mark is gone and every symbolic link has become a plain file — tar copies hold **71, 75, 77 and 81** links, 7-zip copies hold **none** — and putting the mark back by hand is not enough, the program then failing on one of its own libraries.

**Why the two entries differ in count, and that it is not a conflict.** This one names **nine** libraries and the multi-provider batch above names **ten packages**; the machines are different images on different sellers, which is the whole of the reason and is the sentence [R1](../Server%20Setup.md#21-what-the-action-is-made-of) item 4 already states. **Not covered:** Windows, untried in all four versions alike; and what any other provider's image already carries, which nothing measured.

## Fact 46 — reading cards, and the readings that lie on a container

**Shape 1, pointer.** [Six machines](../Six%20machines%20—%20the%20container,%20the%20webhook%20and%20the%20ready%20action.md), *Fact 46*, **2026-08-15/16**; extended by [The last two boxes](../The%20last%20two%20boxes%20—%20the%20hook%20that%20cannot%20be%20read%20back,%20the%20full%20disk,%20and%20the%20unit%20that%20kills%20a%20render.md) §14 (**2026-08-18**) and by [The multi-provider batch](../The%20multi-provider%20batch%20—%20the%20surface%20that%20did%20not%20vary,%20the%20Debian%20route%20that%20exists,%20and%20the%20cards%20a%20slice%20cannot%20see.md) on two multi-card hosts (**2026-08-22**); [Facts](../Facts.md), Fact 46. Label: **observed in a test**. Used by [R1](../Server%20Setup.md#21-what-the-action-is-made-of) item 1.

**What it answered, for this document's one use.** *"Three states exist, not two."* A card present with no driver makes `nvidia-smi` **absent, byte-identical to what a cardless machine gives**; a driver with no card gives `No devices were found`, exit 6. *"So the discrimination is possible only by reading PCI — `lspci -d 10de:`, empty with exit 0 for an absent vendor."* And *"in a view with genuinely no cards, `/proc/driver/nvidia` and the PCI sysfs scan lie"*, both inherited from the host kernel — confirmed on two multi-card hosts, where `/proc/driver/nvidia/gpus` reported four cards on a two-card slice, *"the extra two carrying UUIDs that are another tenant's"*, while **`nvidia-smi` was the only reading correct on all four boxes**.

**Not covered:** the no-card half **cannot be posed on the seller that was tried at all** — every no-GPU offer its search returns is a storage ask, and its own field list has no CPU-only filter. So *a machine with no card* has never been rented and read.

**The cold-machine condition, and it belongs to a different reading. Added 2026-09-02, out of the repaired Blender research.** [Running Blender with nobody there](../../Blender%20Research/Across%20topics/Running%20Blender%20with%20nobody%20there.md), *The machine 5* — reached by `The map`'s own arrival question *what does a machine answer when it is asked, cold, which cards it has*. In its own words: *"On a factory-fresh Blender that list is empty, in all four versions, with an untouched settings folder and again when Blender is started at factory settings; it fills only once a refresh has been asked for, the preferences panel has been drawn, or a preferences file saved earlier is read. **A rented machine asked what cards it has, before anything else, says none** — and that is not the machine having none."* Label: **observed in a test**. Used by [R1a](../Server%20Setup.md#21-what-the-action-is-made-of). **It bears on Blender's own device list and on nothing this document reads** — R1 item 1's bus scan and driver query are not that list — which is why R1a states a rule for any reading that does take it rather than changing a step. `Facts.md` Fact 46 carries the same pointer.

**And a conflict travels with it, stated and not resolved — [Section 13](../Server%20Setup.md#13-unresolved) item 25.** [The multi-provider batch](../The%20multi-provider%20batch%20—%20the%20surface%20that%20did%20not%20vary,%20the%20Debian%20route%20that%20exists,%20and%20the%20cards%20a%20slice%20cannot%20see.md) records the individual devices arriving with `use = True` from factory startup; *The machine 5* records the list arriving empty. Both **observed in a test**, neither doubted, **and the batch's own record does not say whether its script called a refresh.** The Creator's ruling of 2026-08-31 — no further Blender research and no further rented machine — means **no observation will settle it**. R1a's rule is correct under both readings, since both agree a refreshed list returns the machine's devices. `Servers`' companion carries the same conflict, as does `Rendering`'s at M13.

## Fact 6 — the mount that arrives `root:root`, and the second real device

**Shape 1, pointer.** [Two more boxes](../Two%20more%20boxes%20—%20the%20volume,%20the%20full%20disk,%20and%20a%20day%20left%20running.md), **2026-08-16**, for the volume; [The multi-provider batch](../The%20multi-provider%20batch%20—%20the%20surface%20that%20did%20not%20vary,%20the%20Debian%20route%20that%20exists,%20and%20the%20cards%20a%20slice%20cannot%20see.md), **2026-08-22**, for the second device; [Facts](../Facts.md), Fact 6. Label: **observed in a test**. Used by [R6](../Server%20Setup.md#23-what-the-placement-is-given) and [R7](../Server%20Setup.md#23-what-the-placement-is-given).

**What it answered.** An attached volume *"arrives mounted at the path the rental asks for and is a genuinely separate filesystem"* — the root filesystems report device 53 and the volume 2052 — *"and it arrives `root:root 755`, refusing the non-root login Smoozy holds until a setup step hands it over, after which that login writes and reads free space normally."* On that seller the volume *"is not a second disk"* but a quota on a partition of one. **On another seller two real virtio devices ship as standard** — `/dev/vda1` at 97 GB on `/` and `/dev/vdb` at 787 GB on `/ephemeral` — *"so a rental's second filesystem can be a genuine second device and not only a quota on the same disk. Both shapes are now observed and neither may be expected."*

**Not covered:** whether the large second mount survives a stop, a start or a restart **was not posed**; its name is the seller's own claim rather than an observation; survival across the machine's own housekeeping over days is untouched.

## Fact 28 — the free-space reading is a choice of mount

**Shape 1, pointer.** [Two more boxes](../Two%20more%20boxes%20—%20the%20volume,%20the%20full%20disk,%20and%20a%20day%20left%20running.md), **2026-08-16**; [Facts](../Facts.md), Fact 28. Label: **observed in a test**. Used by [R6](../Server%20Setup.md#23-what-the-placement-is-given)'s exclusion of memory-backed filesystems, read-only mounts and host bind-mounts.

**What it answered.** *"A plain `df` returned 51 filesystems, 48 of them `tmpfs`"* — including one per CPU thermal-throttle folder — *"and of the three that are not, one is the host's own 32 GiB disk bind-mounted in to deliver the driver files, showing 20 GiB free that the rental may not use. So the free-space question is not a reading but a choice of which mount to read, and total size, biggest, and sum all pick the wrong one."*

**Not covered:** one box. The document uses only the exclusion; the free-space **route** is `Server protocol and transfers` §9.1's and is cited there rather than here.

## `sudo(8)` — the validate, stdin and non-interactive forms

**Shape 2, publication.** `sudo(8)`, **Sudo 1.9.17p2**, manual page dated **June 7, 2025**, read **2026-08-19** on the Creator's Mac, macOS 26.5.2 (build 25F84). Label: **documented**. Used by [M3](../Server%20Setup.md#1-the-machine-kind-and-what-a-setup-can-be-on-each), [E3](../Server%20Setup.md#9-elevation-the-refusal-and-the-password), [E4](../Server%20Setup.md#9-elevation-the-refusal-and-the-password) and `Security and licence` §14 item 15's separable authentication result.

Exact words:

- `-v, --validate` — *"Update the user's cached credentials, authenticating the user if necessary. For the sudoers plugin, this extends the sudo timeout for another 5 minutes by default, but does not run a command. Not all security policies support cached credentials."*
- `-S, --stdin` — *"Write the prompt to the standard error and read the password from the standard input instead of using the terminal device."*
- `-n, --non-interactive` — *"Avoid prompting the user for input of any kind. If a password is required for the command to run, sudo will display an error message and exit."*
- EXIT VALUE — *"If there is an authentication failure, a configuration/permission problem, or if the given command cannot be executed, sudo exits with a value of 1."*

**What this carries for the document:** the validate form runs no command, so its failure cannot be an operation's — the separable authentication result [E4](../Server%20Setup.md#9-elevation-the-refusal-and-the-password) rests on. The exit value alone does **not** separate an authentication failure from a command that could not be executed, which is why the separation is by the validate-first order and never by the number.

**The rental's own release is now read too, and it moves one figure. Added 2026-08-23.** `sudo(8)` at **1.9.15p5**, the version machine A carried, as Ubuntu publishes it for the image that shipped it — `https://manpages.ubuntu.com/manpages/noble/man8/sudo.8.html` (page states `sudo 1.9.15p5-3ubuntu5.24.04.2`), read **2026-08-23**. Label: **documented**. Used by [M3](../Server%20Setup.md#1-the-machine-kind-and-what-a-setup-can-be-on-each), [E4](../Server%20Setup.md#9-elevation-the-refusal-and-the-password)'s rider and [Section 13](../Server%20Setup.md#13-unresolved) item 6.

- `-n, --non-interactive`, `-S, --stdin` and the EXIT VALUE sentence are **word for word the same** as the 1.9.17p2 page above, so [M3](../Server%20Setup.md#1-the-machine-kind-and-what-a-setup-can-be-on-each)'s probe and [E4](../Server%20Setup.md#9-elevation-the-refusal-and-the-password)'s separation hold at the release a rental actually ran. The EXIT VALUE entry carries one further sentence this document does not use: *"In the latter case, the error string is printed to the standard error."*
- `-v, --validate` **differs in one figure**: *"Update the user's cached credentials, authenticating the user if necessary. For the sudoers plugin, this extends the sudo timeout for another 15 minutes by default, but does not run a command."* — **fifteen minutes, not five.**

**Where the publication contradicted the document.** [E4](../Server%20Setup.md#9-elevation-the-refusal-and-the-password)'s rider read the observed 15-minute cache on machine A *"against the manual's documented five-minute default"*, as a per-machine divergence from documentation. **It is not a divergence at all** — it is exactly what that machine's own release documents. The document was corrected on 2026-08-23; **the rule it justified is unchanged** — the window is read per machine and never assumed from any one page's figure — and is now better founded, because two first-party pages of the same tool give two different defaults.

**Not covered:** the `sudo` a given rental ships in general — two releases have now been read, 1.9.17p2 on the Creator's Mac and 1.9.15p5 as Ubuntu 24.04 publishes it, and a rental on any other image answers for itself; whether a rental's policy supports cached credentials, which the 1.9.17p2 page states is not universal (*"Not all security policies support cached credentials"*); and the behaviour of any of the three forms on a rental, which the entry below covers by observation rather than by either page.

## sudo-rs, and Ubuntu's second elevating command — **the address gap is closed, and one attribution was wrong**

**Shape 2, two publications, read 2026-08-23.** Label: **documented**. Used by [Section 1](../Server%20Setup.md#1-the-machine-kind-and-what-a-setup-can-be-on-each)'s closing paragraph.

**The project's own manual, `https://raw.githubusercontent.com/trifectatechfoundation/sudo-rs/main/docs/man/sudo.8.md`, read 2026-08-23** — the manual source in sudo-rs's own repository:

- `-n, --non-interactive` — *"Avoid prompting the user for input of any kind. If any input is required for the command to run, sudo-rs will display an error message and exit."*
- `-v, --validate` — *"Update the session record for the current session, authenticating the user if necessary."*

**The project's own README, `https://raw.githubusercontent.com/trifectatechfoundation/sudo-rs/main/README.md`, read 2026-08-23**, under its *Ubuntu 25.10 (Questing Quokka) and 26.04 (Resolute Raccoon)* heading: *"sudo-rs is installed and enabled by default; you can control which sudo version is being used by running `update-alternatives --config sudo`"*. It also gives the packaged versions — *"The sudo-rs package in Ubuntu 25.10 is based on v0.2.8 with additional bug fixes backported from newer versions."* and *"The package in Ubuntu 26.04 LTS is based on v0.2.13 with additional bug fixes."*

**Canonical's own page, Ubuntu Server documentation, *sudo-rs*, `https://ubuntu.com/server/docs/reference/other-tools/sudo-rs/`, read 2026-08-23**: *"sudo.ws prompt for password says `[sudo] password for <USERNAME>`, whereas sudo-rs prompt says `[sudo: authenticate] <METHOD>:`."*

**Where the publication contradicted the document.** The document said the default install is *"which that project's own README states and Canonical's own page confirms"*. **The README states it; Canonical's page does not.** That page's only reference to the releases is *"This is a list of major differences as of the Ubuntu 25.10 and 26.04 releases, to help users upgrading to those."* — a statement about what the comparison covers, not about what is installed. **The document was corrected on 2026-08-23** and the default-install claim now stands on the README alone. The prompt-text comparison, which is the half anything at Build would break on, is Canonical's own and holds word for word.

**How this entry read before.** *"No address for any of the three exists anywhere in `Preparation/`"* — searched: this document, every architecture document and sources file, [Facts](../Facts.md), the input documents, the Specs and their sources files, and Groundwork, with the string `sudo-rs` occurring in this document alone. The words and a read date of **2026-08-20** were recorded and no address was; the reading behind that date is evidenced nowhere. The three quotations were re-read at the addresses above on **2026-08-23** and all three hold.

**Not covered.** The manual read is the project's own repository at `main` rather than the exact packaged version Ubuntu ships (v0.2.8 and v0.2.13 by the README's own account), so an option's wording at those tags is not what was read; **neither implementation has been run on a rental**, which the document says at [Section 13](../Server%20Setup.md#13-unresolved) item 6; and nothing here says what `update-alternatives` selects on any particular machine. **What the citation is carrying is a warning** — that anything at Build matching the prompt text breaks on Ubuntu 25.10 and later — and [E2](../Server%20Setup.md#9-elevation-the-refusal-and-the-password) matches no prompt text, so nothing in the mechanism rests on it.

## The elevation probe, the validate form and the three refusals, run on a rental

**Shape 1, pointer.** [The multi-provider batch](../The%20multi-provider%20batch%20—%20the%20surface%20that%20did%20not%20vary,%20the%20Debian%20route%20that%20exists,%20and%20the%20cards%20a%20slice%20cannot%20see.md), *Test 11*, machine A, sudo **1.9.15p5**, Ubuntu's stock sudoers, under a purpose-made non-root login in group `sudo`, **2026-08-22**. Label: **observed in a test**. Used by [M3](../Server%20Setup.md#1-the-machine-kind-and-what-a-setup-can-be-on-each), [E4](../Server%20Setup.md#9-elevation-the-refusal-and-the-password) and [Section 13](../Server%20Setup.md#13-unresolved) item 6's narrowing.

**What it answered.** `sudo -n -v` returned **`sudo: a password is required`** at exit 1. `sudo -S -v` with the right password prompted `[sudo] password for smoozytest:` and **exited 0 having run no command**. The three refusals **differ word for word and all exit 1**: wrong password ends *"`sudo: 1 incorrect password attempt`"*; no password at all gives *"`sudo: a password is required`"*; a login that is not an administrator gives *"`Sorry, user smoozynoadmin may not run sudo on a0e7a496ff47.`"* — **which names the host**, so any construction matching it as a constant breaks on the next machine. The credential cache on that image is **15 minutes** (`sudo -V`: *Authentication timestamp timeout: 15.0 minutes*), and `sudo -n id -u` inside the window returned `0`.

**What this entry is now carrying, corrected 2026-08-31.** It used to end *nothing here separates the two failures the document declines to separate*, and the document no longer declines both. `Security and licence` §13 rule 5 carries a **Creator-stated exception, dated 2026-08-25**, reaching the elevation tool alone: the wrong-password ending `sudo: 1 incorrect password attempt` is matched as an **exact observed constant**. **That exception is a decision and earns no citation; the strings it is drawn on are this observation's**, and the line it is drawn along is this observation's too — the not-an-administrator message **names the host**, so it is per-machine text and matches nothing on the next machine, while the wrong-password ending carries nothing of the machine. [E4](../Server%20Setup.md#9-elevation-the-refusal-and-the-password), [W1](../Server%20Setup.md#10-what-this-module-says-about-a-machine)'s *Refused* ending and [Section 13](../Server%20Setup.md#13-unresolved) item 6 were all repaired on 2026-08-31 onto that footing.

**Not covered.** One login on one image on one seller, at 1.9.15p5, on a container. **The delivered login could not be asked the question at all** — it arrives as root, `id -u` is 0 and `sudo -n true` returns 0 — which is why the run made a login to pose it with. Nothing here says what a **policy without cached credentials** does. **And the reach of the matched constant is exactly this observation's reach:** a release or a machine kind whose wording was never observed matches nothing, so the virtual-machine kind under a non-root login, and any `sudo` release but 1.9.15p5 under Ubuntu 24.04's stock sudoers, are outside it. **What breaks there:** an unmatched refusal keeps the password and Row 117's promise is weaker for those cases, which is what item 6 states.

## The probe's two forms, the driver check's streams, the staged file and the search's own area — two rented machines, 2026-09-04

**Shape 1, pointer.** `Audits/Build/Step 6 rental.md`, machine A (Hyperstack virtual machine, image `Debian 12 Plain`) and machine C (Hyperstack virtual machine, image `Ubuntu Server 22.04 LTS R570 CUDA 12.8`), each driven on the login its provider delivered, **2026-09-04**. Label: **observed in a controlled test**. Used by [M3](../Server%20Setup.md#1-the-machine-kind-and-what-a-setup-can-be-on-each)'s repair, [B10](../Server%20Setup.md#54-the-search)'s added bullet, and the code repairs of `App/Server Setup/` made the same day. **Both machines were destroyed the moment they had answered.**

**What it answered, in four parts.**

1. **The elevation probe's two forms, on both machines.** `sudo -n true` **exited 0**; `sudo -n -v` **exited 1** with `sudo: a password is required`. The sudoers rules were read on the machines: each login is granted **both** `(ALL : ALL) ALL` and `(ALL) NOPASSWD: ALL`, and validating meets the first.
2. **The driver check's streams**, on machine C with the driver installed and its kernel modules unloaded — the state a working-but-unbooted install is in. The tool ended at **9** and wrote its whole **145-byte** message, *"NVIDIA-SMI has failed because it couldn't communicate with the NVIDIA driver…"*, to **standard output**, with **nothing** on the complaint stream.
3. **The components step's staged file**, on machine A. After the step, `~/.smoozy-carriage/debian-sources` was still on the machine. The step itself worked and is safe to run twice: the elevated copy landed, the list update succeeded, and a second run answered *nothing to do*.
4. **The whole-disk search without the file-kind restriction**, on machine A: **0.84 seconds** on a 99 GB disk, no folder refused because the elevated retry reached them, **five paths returned, three of them directories**, and one of those three was **the `blender` area Smoozy's own placement had made**. A symbolic link was among the five.

**Not covered, and each limit is why the clauses stay where they do.**

- **Two images on one seller on one day.** Nothing here says what `sudo`, `nvidia-smi`, `find` or an image's sudoers do in general, and no publication was read for any of the four. **A per-command sudoers policy was never met**, so nothing here says a probe that succeeds means every later step runs without a password; `sudoers(5)`'s `NOPASSWD` tag is unread by this project and one rental carrying such a policy would settle it.
- **The driver observation is one tool in one failure state.** It does not say where that tool writes on any other image, in any other failure, or at any other version — which is why the reading now keeps **both** streams rather than swapping one for the other.
- **The search figures are one disk.** Which of the five paths a different machine would return, and whether a path other than Smoozy's own area is a directory, is not established here; only that Smoozy's own area was among them, and that path is one Smoozy composes rather than a fact about the machine.
- **No container was driven with this module's own code**, so every one of the four is a virtual-machine observation.

---

# Part 3 — Section 2: the ready action

## GNU coreutils, *chown invocation* — how `chown` treats a symbolic link

**Shape 2, publication.** GNU coreutils manual, *chown invocation* — `https://www.gnu.org/software/coreutils/manual/html_node/chown-invocation.html` — read **2026-08-22**. Label: **documented**. Used by [R7a](../Server%20Setup.md#23-what-the-placement-is-given), the planted-link carriage. Exact words:

- default and `--dereference` — *"Do not act on symbolic links themselves but rather on what they point to. This is the default when not operating recursively."*
- `-h, --no-dereference` — *"Act on symbolic links themselves instead of what they point to. This mode relies on the `lchown` system call."*
- `-H` — *"If --recursive (-R) is specified and a command line argument is a symbolic link to a directory, traverse it."*
- `-L` — *"In a recursive traversal, traverse every symbolic link to a directory that is encountered."*
- `-P` — *"Do not traverse any symbolic links. This is the default if none of -H, -L, or -P is specified."*

**What this carries for the document:** the planted-link carriage works through a plain `chown` because a command-line link is dereferenced by default, and works through a recursive `chown` **only with `-H`**, because recursion defaults to `-P` and would otherwise touch nothing on the target mount — which is why [R7a](../Server%20Setup.md#23-what-the-placement-is-given) requires the flag and keeps R7's real-write landing check.

**Not covered:** any rental's own `chown` — this is the GNU coreutils project's manual, read on no machine; a rental shipping a different implementation answers for itself, which is the same per-image ground `Server protocol and transfers` §12 items 3 and 11 record for every command-channel step. **Nothing in the manual says what a failed dereference reports**, which is why the exit status is never the landing evidence.

## Ubuntu Server's own driver command

**Shape 2, publication.** Ubuntu Server documentation, *Install NVIDIA drivers* — `https://ubuntu.com/server/docs/how-to/graphics/install-nvidia-drivers/`, read **2026-08-20**. Label: **documented**. Used by [R1](../Server%20Setup.md#21-what-the-action-is-made-of) item 5's Ubuntu arm and by [Section 13](../Server%20Setup.md#13-unresolved) item 18.

**Exact words.** `ubuntu-drivers install --gpgpu` is given as the documented and recommended command, *"which will install the driver that is considered the best match for your hardware"*.

**What the same page does not say, which item 18 rests on.** It *"stops its own example listing at driver 550 and nowhere states that the tool picks the open flavour correctly for Blackwell"* — a statement about the page's silence, taken from the same read.

**Not covered:** the tool exists only on Ubuntu — Canonical's own — and this page says nothing about any other family; `ubuntu-drivers-common`'s Debian absence is a reading of one image (entry above). Fact 47 records that this command on one machine **did not install `nvidia-smi`** and **omitted `libnvoptix`**, which the page does not mention.

## NVIDIA's own README — which flavour serves which card

**Shape 2, publication.** NVIDIA's README for driver 580.105.08, `https://download.nvidia.com/XFree86/Linux-x86_64/580.105.08/README/kernel_open.html`, read **2026-08-20**. Label: **documented**. Used by [R1](../Server%20Setup.md#21-what-the-action-is-made-of) item 5's prohibition on naming a flavour. Exact words:

- *"The proprietary flavor supports the GPU architectures Maxwell, Pascal, Volta, Turing, and later GPUs until Blackwell. Blackwell and later are only supported by the open kernel modules."*
- *"The open kernel modules cannot support GPUs before Turing, because the open kernel modules depend on the GPU System Processor (GSP) first introduced in Turing."*

**Both ends observed on this project's own rentals.** The pre-Turing end — [The last two boxes](../The%20last%20two%20boxes%20—%20the%20hook%20that%20cannot%20be%20read%20back,%20the%20full%20disk,%20and%20the%20unit%20that%20kills%20a%20render.md) §11, the kernel's own words: *"installed in this system is not supported by open nvidia.ko because it does not include the required GPU System Processor (GSP)"*, with `probe of 0000:00:07.0 failed with error -1` and `None of the NVIDIA devices were initialized.` The other end — [The rented machine](../The%20rented%20machine.md) §9, a box driven by `nvidia-driver-580-open` (`580.105.08-0ubuntu1`, with its DKMS and kernel-source packages). Label of both: **observed in a test**.

**Not covered:** this is one driver release's README; what a **different** release supports is not read. It says nothing about what any distribution's detection route actually picks, which is item 18's whole point.

## What the fetch and the unpack cost

**Shape 1, pointer.** [The multi-provider batch](../The%20multi-provider%20batch%20—%20the%20surface%20that%20did%20not%20vary,%20the%20Debian%20route%20that%20exists,%20and%20the%20cards%20a%20slice%20cannot%20see.md), *Test 15*, machine A, after `xz-utils` and the link libraries were in place, **2026-08-22**. Label: **observed in a test**. Used by [R1](../Server%20Setup.md#21-what-the-action-is-made-of) item 4's cost paragraph.

**What it answered.** Download **5 s** for 4.5.12 and **4 s** for 5.2.0; unpack **62 s** and **67 s** on four cores with single-threaded `xz`; the two archives were **377 902 364** and **384 441 228** bytes. On a machine with more cores both downloaded in parallel and both unpacked in **34 seconds in all**. **The unpack dominates the download by an order of magnitude.**

**Not covered:** two archives on two machines on one day, one seller, one link. Nothing here is a figure to budget with — it is what makes the step's cost the machine's processor rather than its link.

## Fact 81 — Secure Boot, unposable

**Shape 1, pointer, and the label is unresolved.** [Facts](../Facts.md), Fact 81, exposed by the six-machine batch **2026-08-16** and numbered by the runner the same day. Label: **unresolved**. Used by [R9](../Server%20Setup.md#25-secure-boot-and-the-machine-smoozy-cannot-make-ready) and [Section 13](../Server%20Setup.md#13-unresolved) item 1.

**What the register says.** *"None of the seven machines this project has rented had one; the Sesterce box had legacy BIOS with no EFI at all and its driver build logged `EFI variables are not supported on this system` while trying to sign the module, which is what raised the question."* The register names the failure mode the document's clause is written for: *"the ready action may need a human at the machine's console enrolling a key, which Smoozy cannot do over the connection it holds. If so, that is not an architecture choice but a machine Smoozy cannot make ready."*

**Not covered: everything about the behaviour itself.** Nothing was run on a machine with Secure Boot enforcing, and no operating-system or driver publication on module signing was read by this project. The clause is written on the register's own statement of the failure mode.

---

# Part 4 — Sections 3 and 4: the setup run, and running it again

## Fact 57 — whether a side learns its connection ended

**Shape 1, pointer, and the input document carries the publications.** [Six machines](../Six%20machines%20—%20the%20container,%20the%20webhook%20and%20the%20ready%20action.md), *Fact 57*, **2026-08-16**; [Facts](../Facts.md), Fact 57. Label: **observed in a test**, with the ceiling **documented**. Used by [Y7](../Server%20Setup.md#33-the-worker-the-driving-and-the-stop), [T4](../Server%20Setup.md#8-the-restart) and [Section 13](../Server%20Setup.md#13-unresolved) item 7.

**What it answered.** *"The machine had not learnt within the 5 minutes 10 seconds it was watched"* — it held the socket open with the job still running. **The register corrected *never learns at all* on 2026-08-21**, against the manual of the software the run observed. The publications, as the input document carries them:

- `sshd_config(5)`, macOS 26.5 man page dated **2025-10-04**, read on the Creator's Mac **2026-08-20**, on `TCPKeepAlive`: *"The default is yes (to send TCP keepalive messages), and the server will notice if the network goes down or the client host crashes."*
- the kernel's own `ip-sysctl` document, read **2026-08-20** by the input document and **re-read at its address on 2026-08-23** — `https://docs.kernel.org/networking/ip-sysctl.html`: `tcp_keepalive_time` *"How often TCP sends out keepalive messages when keepalive is enabled. Default: 2hours."*; `tcp_keepalive_probes` *"How many keepalive probes TCP sends out, until it decides that the connection is broken. Default value: 9."*; `tcp_keepalive_intvl` *"How frequently the probes are send out. Multiplied by tcp_keepalive_probes it is time to kill not responding connection, after probes started. Default value: 75sec i.e. connection will be aborted after ~11 minutes of retries."*; and `tcp_retries2` *"The default value of 15 yields a hypothetical timeout of 924.6 seconds and is a lower bound for the effective timeout."* **All three values the document uses hold word for word.** **The address is supplied here for the first time — 2026-08-23**; the document and Fact 57 identified this publication by site and name and no URL existed anywhere in `Preparation/`. **Nothing was corrected**; only the address was missing.

**The rest of the fact, which [T4](../Server%20Setup.md#8-the-restart) uses.** The caller's side: a machine closing politely and a machine powered off from the provider's console were both detected **in the same second**; the caller's own line pulled was detected in **56 seconds** with keepalives on and **not at all** — silent for 21 minutes 45 seconds — with them off.

**Not covered:** the machine side was watched for 5 min 10 s and nobody watched for two hours, so *"the run measured what those defaults already predict and nothing more."* The `ip-sysctl` document states defaults for the kernel as built, and says nothing about what any particular image's `sysctl` settings actually are — no rental's live values were read. **The partial address is closed: the URL is above, supplied 2026-08-23.** `Servers` and `Rented-server runtime` report the same gap from their own sides and repair their own; nothing here edits theirs.

## Fact 55 — what a restart stops

**Shape 1, pointer.** [Six machines](../Six%20machines%20—%20the%20container,%20the%20webhook%20and%20the%20ready%20action.md), *Fact 55 — what a restart stops, posed against a real render*, **2026-08-15/16**, for the render half; [Batch A](../Batch%20A%20—%20the%20shape%20throwaway.md) for the transfer half, **2026-08-13** — *"It left 4.7 GB of the 8 GB asked for, under the file's final name, with nothing marking it incomplete, and nothing restarted it. Its error stream was empty."* [Facts](../Facts.md), Fact 55. Label: **observed in a test** for the render and the transfer; **unresolved** for the dependency check. Used by [T7](../Server%20Setup.md#8-the-restart), [W3](../Server%20Setup.md#10-what-this-module-says-about-a-machine) and [Section 13](../Server%20Setup.md#13-unresolved) item 2.

**What it answered.** **Two real machine restarts with a Cycles render in flight**, taken over the connection as root rather than from the provider console. *"Both feared outcomes are real and which one arrives cannot be chosen."* A frame ran about 16.8 seconds — roughly 15.4 sampling, 1.4 writing. **Interrupted during the write, a partial sits under the frame's real final name with nothing to mark it** — 52,514,816 bytes carrying OpenEXR's real magic number, not sparse — *"which settles that Blender does not write to a temporary name and rename."* **Interrupted during sampling, about 92% of a frame, there is nothing at the path at all**: no partial and no zero-byte file. *"A polite stop behaves identically to a hard kill at exit 143, and that is the signal a shutdown sends."* Both restarts landed in sampling **despite the second being aimed deliberately at a write**, because the shutdown latency varied from 9 seconds to 6.4 between runs minutes apart — *"so a restart leaves a partial about 8% of the time and nothing about 92%, and cannot be aimed even on purpose."* **Nothing announces the death**: the system log records only that it sent a termination signal to remaining processes, *"naming neither Blender nor the render nor its output."* **The transfer half:** an 8 GB write in flight died and did not come back, leaving **4.7 GB under the file's final name with nothing marking it incomplete** and an empty error stream.

**Not covered, and it is the open clause.** *"A dependency check in flight was not posed at all"* — the fact's own last open line. Also not covered: **one output format and one picture size**.

## Fact 60 — another login's process is visible with its command line

**Shape 1, pointer.** [Six machines](../Six%20machines%20—%20the%20container,%20the%20webhook%20and%20the%20ready%20action.md), *Fact 60*, **2026-08-15/16**, re-posed against a real 400-frame Cycles render and closed at [The last two boxes](../The%20last%20two%20boxes%20—%20the%20hook%20that%20cannot%20be%20read%20back,%20the%20full%20disk,%20and%20the%20unit%20that%20kills%20a%20render.md) §18, **2026-08-18**; [Facts](../Facts.md), Fact 60. Label: **observed in a test**. Used by [E6](../Server%20Setup.md#9-elevation-the-refusal-and-the-password)'s rule that a password is never a command argument.

**What it answered, for this use.** From a second ordinary login, *"the process is visible with its owner and command line"* — on the re-posed run, *"both processes with owner and command line, the process name, the thread count, and even the start-time field"*. Stopping is refused: polite and forcible stops, a pattern kill of the render, and a pattern kill of every process of that login all answered `Operation not permitted`.

**Not covered:** the fact is about **seeing and stopping**; what it establishes for [E6](../Server%20Setup.md#9-elevation-the-refusal-and-the-password) is the visibility of a command line to another login on the same machine, and nothing about how long an argument persists anywhere else, or about a machine's own process accounting.

## `dpkg(1)` and apt's own source — the repair sequence, and the forced form that is forbidden

**Shape 2, publications, with one address carried by another document of the project.** Label: **documented**. Used by [S7](../Server%20Setup.md#43-why-each-step-is-safe-to-run-again) and by [Section 13](../Server%20Setup.md#13-unresolved) item 5.

**apt's own source**, `apt-pkg/deb/debsystem.cc`, read **2026-08-20** at `https://salsa.debian.org/apt-team/apt/-/raw/main/apt-pkg/deb/debsystem.cc`: `cmd = "dpkg --configure -a";` … `_error->Error(_("dpkg was interrupted, you must manually run '%s' to correct the problem. "), cmd)`. **So the first half of the repair is the instruction the machine itself prints in exactly the situation the rule fires on.**

**`dpkg(1)`**, Debian trixie, `https://manpages.debian.org/trixie/dpkg/dpkg.1.en.html`, **read 2026-08-23**, quoted by the document for three things — **all three verified word for word at that address on that date**:

- *"Configure a package which has been unpacked but not yet configured. If -a or --pending is given instead of package, all unpacked but unconfigured packages are configured."*
- on the force options — *"These options are mostly intended to be used by experts only. Using them without fully understanding their effects may break your whole system."*
- on what `all` turns on — `--force-all` *"Turns on (or off) all force options"*, among them `remove-essential`, *"Remove, even if the package is considered essential. … Removing them might cause the whole system to stop working, so use with caution."*, and `remove-protected`, *"Remove, even if the package is considered protected (since dpkg 1.20.1). … Removing them might cause the whole system to be unable to boot or lose required functionality to operate, so use with caution."* The document quotes the middle of each of those last two sentences; the whole of each is recorded here.

**How this entry read before.** The document gave **no address** for `dpkg(1)` and a read date of **2026-08-20** whose reading is evidenced nowhere. The project already carried an address for the same manual — the same one above, read **2026-08-22** by `Desktop app and delivery` and recorded in [its sources file](Desktop%20app%20and%20delivery.sources.md) — but that was a different read on a different date for different words, so it identified the publication and did not confirm these quotations. **It was a soft gap and it is now closed by reading the manual for these words.**

**What this carries for the document.** `dpkg --remove --force-all` was recorded once as an observed recovery (Fact 48's container case) and written here as the rule; the manual's own warnings are why it cannot be, *"and a forced removal opening every re-run sits directly against Row 114's promise"*.

**Not covered, and item 5 states it.** **The documented sequence has never been run by this project on any machine.** The correction of 2026-08-20 removed the only observation this step ever had. Nothing establishes what it does on a package system the documented repair cannot fix.

**PERMANENT FOR THIS STAGE, 2026-09-02, and the label is what protects it.** Item 5's own *Answerable: free* sentence is exact — the run needs no card, no rental and no money — but it needs **an Ubuntu machine, and this project runs none**; every Linux box it has held was a rental, and the Creator ruled on 2026-08-31 that there will be no further rental to answer a question. **So the label here stays *documented* for the life of this stage.** **It must not be upgraded by a single successful run.** A builder who interrupts a transaction, runs the sequence and finds it works has established that it worked once, on one machine, in one package state; what this entry lacks is the failing case, and that is the case the document's clause is written for.

## apt's frontend lock, and Ubuntu's automatic updates

**Shape 2, publications.** Label: **documented**. Used by [S7b](../Server%20Setup.md#43-why-each-step-is-safe-to-run-again) and by [Section 13](../Server%20Setup.md#13-unresolved) item 19.

- **Ubuntu Server documentation, *Automatic updates*, read 2026-08-20** at `https://ubuntu.com/server/docs/how-to/software/automatic-updates/`: *"unattended-upgrades may often run on system startup and thereby cause immediate activity and prevent other package operations from taking place at that time"*, naming the counter-productive case as *"VM images that are only started for some quick action"*.
- **apt's own source**, same address and date as the entry above: `DPkg::Lock::Timeout` **defaults to `0`** and its lock acquisition returns immediately at that value; the refusal reads *"Unable to acquire the dpkg frontend lock (%s), is another process using it?"*

**Beside them, observed.** [The rented machine](../The%20rented%20machine.md) §5 records `unattended-upgrades.service   loaded active running` on a real rental, and §9 D an unasked kernel upgrade. Label: **observed in a test**.

**Not covered.** *"The setting is documented in apt's source and in neither `apt.conf(5)` nor `apt-get(8)`"*, so its behaviour at a non-zero value is documented nowhere this project has read — which is why the bounded wait's **value** is item 19 and not a citation. The Ubuntu page is Canonical's and says nothing about Debian; the entry above establishes that the behaviour is the **image's** rather than either family's.

## The bounded wait, exercised at boot

**Shape 1, pointer.** [The multi-provider batch](../The%20multi-provider%20batch%20—%20the%20surface%20that%20did%20not%20vary,%20the%20Debian%20route%20that%20exists,%20and%20the%20cards%20a%20slice%20cannot%20see.md), *Test 12*, machine C **at 108 seconds of uptime**, **2026-08-22**. Label: **observed in a test**. Used by [S7c](../Server%20Setup.md#43-why-each-step-is-safe-to-run-again).

**What it answered.** `apt-get -o DPkg::Lock::Timeout=120 update` returned **0 in 31 seconds**, fetching 26.7 MB; `fuser /var/lib/dpkg/lock-frontend` reported the lock **not held** at that moment. *"A negative would have been the documented sequence failing to clear the lock, or waiting longer than a setup can reasonably wait. Neither happened on this box."*

**Not covered, in the input document's own words.** *"What this does not establish is the loaded case: the timers were enabled and one was 17 minutes away, so the run did not collide with a job actually holding the lock. The `DPkg::Lock::Timeout` route was exercised; the collision it exists for was not."* **Item 19's value stays unobserved.**

## The interrupted install re-run over the same folder — Blender

**Shape 1 into Blender research, and the document names the question itself.** [S7](../Server%20Setup.md#43-why-each-step-is-safe-to-run-again)'s tooling/unpack bullet cites ***Builds 9*** by name. The project's record of what that question carries is [Preparation/Specs/Sources/Server Setup.sources.md](../../Specs/Sources/Server%20Setup.sources.md): *"That an install cut off part-way can be run again over the same folder, and that the wrong unpacking tool breaks a Blender in silence."* Label: **observed** for the first, **documented and observed** for the second.

**Not covered, in that record's own words:** *"this was one Blender archive on one machine, and the Windows installer could not be tried at all. It settles the Blender half of the run-it-again promise and no more."* The document narrows further of its own accord: *"That observation covers the unpack half alone"* — the package-install half of the same step is covered by the repair precondition instead.

---

# Part 5 — Section 5: the Blender lifecycle

## The shop-front listing, read per channel (Fact 52, and Row 92's list)

**Shape 2, publication.** The Snap Store's own package-information endpoint for the Blender Foundation's package — `https://api.snapcraft.io/v2/snaps/info/blender`, requested with the header `Snap-Device-Series: 16`. Read **2026-08-19**; HTTP 200. Label: **observed**. Used by [B2](../Server%20Setup.md#51-the-list-and-the-fetch), [B3](../Server%20Setup.md#51-the-list-and-the-fetch), [B5](../Server%20Setup.md#51-the-list-and-the-fetch) and [B6a](../Server%20Setup.md#51-the-list-and-the-fetch).

**What it answered.** One entry per channel in a `channel-map`, each carrying the track, the risk, the version, the release moment and the download size, with the publisher given as `Blender Foundation`, username `blenderfoundation`, validation `verified`. The stable-risk channels on the day read, verbatim values:

- `latest/stable` → **5.2.0** (released 2026-07-14)
- `5.2lts/stable` → 5.2.0; `5.1/stable` → 5.1.2; `5.0/stable` → 5.0.1; `4.5lts/stable` → 4.5.12; `4.3/stable` → 4.3.1; `4.2lts/stable` → **4.2.23**; older tracks back to `2.79/stable`.
- Non-stable risks on the same day: `latest/candidate` → 5.1.2; `latest/edge` → **5.3.0** (released 2026-08-19); `5.2lts/candidate` → **5.2.1** (released 2026-08-19); `4.5lts/candidate` → 4.5.12; assorted `beta` entries on old tracks.

Long-term-support lines are marked in the track's own name (`4.2lts`, `4.5lts`, `5.2lts`); the unstable kinds are marked by the risk field. The default track is `latest`.

**Not covered:** whether *installing* through a named channel with the machine's snap tooling delivers that channel's version — the listing was read, nothing was installed; whether the endpoint's answer differs by requesting device or region; and any moment other than 2026-08-19. **Nothing establishes that a Blender installed through that shop-front carries card rendering**, which is the separate ground [Preparation/Specs/Sources/Server Setup.sources.md](../../Specs/Sources/Server%20Setup.sources.md) records under **The machine 3** and **Builds 2** and the reason nothing is fetched from it.

## Blender's own downloads, read against the listing (Fact 52)

**Shape 2, publication.** Blender's own release listing and folders — `https://download.blender.org/release/`, and under it `Blender5.2/`, `Blender5.1/`, `Blender5.0/`, `Blender4.5/`, `Blender4.3/`, `Blender4.2/`. Read **2026-08-19**; all HTTP 200. The listing is a plain directory index (`Index of /release/`) with one folder per release line from `Blender1.0/` to `Blender5.2/`. Label: **observed**. Used by [B4](../Server%20Setup.md#51-the-list-and-the-fetch) and [B5](../Server%20Setup.md#51-the-list-and-the-fetch).

**What it answered.** For every stable-channel version checked, the file exists at the derivable address `Blender<major.minor>/blender-<version>-linux-x64.tar.xz`: confirmed by direct request at 200 for 5.2.0, 5.1.2, 5.0.1, 4.3.1, and by folder listing for 4.5.0–4.5.12 and 4.2.20–4.2.23. The checksum is one file per release covering all platforms, named `blender-<version>.sha256` (observed beside `blender-5.2.0-linux-x64.tar.xz`; a per-file `…tar.xz.sha256` does **not** exist — 404). **The disagreement case is real and was observed the same day:** `Blender5.2/blender-5.2.1-linux-x64.tar.xz` answered **404** while the listing's `5.2lts/candidate` named 5.2.1, and no `Blender5.3/` folder exists while `latest/edge` named 5.3.0. So on the day read, the stable channels and the downloads agreed pair for pair, and the candidate and edge channels named versions the downloads do not carry. **Read again on 2026-10-02 with the same result** — see *The build service read again* below.

**Not covered:** any other day — the frequency of disagreement over time is unknown; platforms other than `linux-x64` beyond what the 5.2 folder listing shows; and whether the sha256 naming convention holds for every historic folder.

## Where daily builds sit, and blender.org's machine-readable listing (Fact 53)

**Shape 2, publication.** Blender's own build service — `https://builder.blender.org/download/daily/?format=json&v=1`. Read **2026-08-19**; HTTP 200, 81,331 bytes. Label: **observed**. Used by [B6](../Server%20Setup.md#51-the-list-and-the-fetch) and [B6a](../Server%20Setup.md#51-the-list-and-the-fetch)'s last bullet.

**What it answered.** A structured list, one entry per artefact, each carrying `version`, `risk_id` (`alpha`, `candidate`, `stable`), `branch`, `hash`, `platform`, `architecture`, `file_size`, `file_mtime`, `file_name` and a `url` on `cdn.builder.blender.org/download/daily/`. On the day read it carried 131 entries spanning 5.3.0-alpha (main), 5.2.1-candidate, 4.5.2-candidate, and stable rebuilds of 4.2.23, 4.3.2, 4.4.3, 4.5.12, 5.0.1 and 5.1.2, with the newest file moment 2026-08-19T05:47:59Z. **So blender.org's own build service does publish a listing a machine can read, and daily builds sit there.**

**One trap observed:** the listing also held a stale entry — `4.2.0-alpha+main`, file moment June 2024 — beside the current ones, so a reader must read `file_mtime` and must not assume every entry current.

**Not covered:** how long entries persist; whether the endpoint's shape is versioned or stable over time; and anything about the separate experimental/patch listings the same service may carry. ~~**Nothing in the document builds on it** — the list's source is the Creator's ruling of 2026-08-12, which is a decision and earns no entry.~~ **Corrected 2026-10-02: B2 and B4 build on it**, on the Creator's revision of that day, which is a decision and earns no entry. The reading they rest on is the next entry.

## The build service read again, and Blender's release folders read again (Fact 52, Fact 53) — 2026-10-02

**Shape 2, publication. Read 2026-10-02, about 09:15 UTC by the mapping agent and again at 09:37 UTC by the plan pass, the same answers both times.** Label: **observed**. Used by [B2](../Server%20Setup.md#51-the-list-and-the-fetch), [B4](../Server%20Setup.md#51-the-list-and-the-fetch), [B5](../Server%20Setup.md#51-the-list-and-the-fetch) and [B6](../Server%20Setup.md#51-the-list-and-the-fetch).

- **Blender's release folders.** `https://download.blender.org/release/` → 200, newest folder `Blender5.2/`. `https://download.blender.org/release/Blender5.2/` → 200, 31 entries: 5.2.0, 5.2.1 and 5.2.2 for each platform, with `.md5` and `.sha256`; **none named alpha, beta or rc.** `https://download.blender.org/release/Blender5.3/` → **404**, and `…/Blender5.3/blender-5.3.0.sha256` → **404** — the address Smoozy's machine composed for 5.3.0, which the Creator's note 7 met as *"the checksum file answered 404"*.
- **The build service's page.** `https://builder.blender.org/download/daily/` → 200. Its own words: *"The following builds have the latest features and bug fixes, but they can be unstable and break your files."* It names entries *"Blender 5.3.0 Alpha"* and, for an older Windows-only entry, *"Blender 4.5.2 Release Candidate"*.
- **Its machine-readable listing.** `https://builder.blender.org/download/daily/?format=json&v=1` → 200, 81,139 bytes, 131 entries, each with `app`, `architecture`, `bitness`, `branch`, `file_extension`, `file_mtime`, `file_name`, `file_size`, `hash`, `patch`, `platform`, `release_cycle`, `risk_id`, `url` and `version`. **`risk_id` values present: `alpha`, `candidate`, `stable`; no `beta` on this day.** The Linux `.tar.xz` entries were the `stable` rebuilds of 4.2.23, 4.3.2, 4.4.3, 4.5.14, 5.0.1, 5.1.2 and 5.2.2, and **one unfinished build: version `5.3.0`, `risk_id` `alpha`, `file_name` `blender-5.3.0-alpha+main.64d181010e0d-linux.x86_64-release.tar.xz`**, with a `url` under `https://cdn.builder.blender.org/download/daily/` — every entry's `url` is that base plus its `file_name`. The checksum is listed as an entry of its own, `file_extension` `sha256`. Stale Windows entries from July and August sat beside the current ones, as B6 already warned.
- **The alpha's archive.** At that `url`: HTTP 200, `content-length: 399290348`, `last-modified: Fri, 02 Oct 2026 02:36:58 GMT`, `accept-ranges: bytes`; a request for bytes 0 to 5 answered **206** with `fd 37 7a 58 5a 00`, the XZ magic number.
- **Its checksum.** `<that url>.sha256` → 200, **64 bytes: a bare hexadecimal digest, one line with no file name and no line ending** (`42ec085c…dfeca3`). A release folder's `blender-<version>.sha256` lists one digest per file by name ([B4](../Server%20Setup.md#51-the-list-and-the-fetch)'s first route), so the two shapes differ.
- **The shop-front the same morning.** `https://api.snapcraft.io/v2/snaps/info/blender` (Canonical's, quoted only as what Smoozy listed): its one unfinished entry at or above 4.5 that survived B2's narrowing was `latest/edge` at **5.3.0**, marked only by the risk `edge`.

**Not covered:** any other day; whether the build service ever carries a `beta`, which neither reading saw; the top folder inside the alpha's archive, which nobody listed (the builder lists it before trusting the unpack); **and what an unfinished build prints when asked its version**, which stays B1's *documented and untested*.

## The Filesystem Hierarchy Standard on `/proc` and `/sys` (B10) — 2026-10-02

**Shape 2, publication.** The Linux Foundation's *Filesystem Hierarchy Standard* 3.0, chapter 6, `https://refspecs.linuxfoundation.org/FHS_3.0/fhs/ch06.html`, read 2026-10-02. Label: **documented**. Used by [B10](../Server%20Setup.md#54-the-search).

**What it says.** §6.1.5: *"/proc : Kernel and process information virtual filesystem"*. §6.1.7: *"/sys : Kernel and system information virtual filesystem"*, *"the location where information about devices, drivers, and some kernel features is exposed."* So neither holds files a user put there, and a search that walks them counts the kernel's own entries as folders it could not read — 2,578 of 2,679 on Sesterce under an ordinary login (`Audits/Build/Step 10c/The release pass on 0.1.7 — Sesterce.md`).

**Not covered:** whether every rented image mounts both at those paths — the standard is what Linux systems follow, not a reading of each image — and other virtual mounts the search may still meet.

## Fact 54 — the machine's own line, byte ranges, and the sort trap

**Shape 1, pointer.** [Six machines](../Six%20machines%20—%20the%20container,%20the%20webhook%20and%20the%20ready%20action.md), *Fact 54*, **2026-08-15/16**; [Facts](../Facts.md), Fact 54. Label: **observed in a test**. Used by [B2](../Server%20Setup.md#51-the-list-and-the-fetch), [B4](../Server%20Setup.md#51-the-list-and-the-fetch), [A1](../Server%20Setup.md#6-add-ons) and [Section 13](../Server%20Setup.md#13-unresolved) items 3 and 12.

**What it answered.** Yes on every machine that posed it, **and the fetch happens on the machine**: one box pulled 360 MB and 366 MB in 10.3 and 9.9 seconds — *"about 37 MB/s on the machine's own line, with nothing passing through the Mac"* — and another reached six blender.org hosts, all 200, fetching a 335 MB archive in 2 seconds. Both ran deliberate negative controls in the same session (`curl: (6) Could not resolve host`, `curl: (28) ... Connection timed out`), *"so the test could fail."* **The downloads honour byte ranges** — `http_code=206`, returning the archive's real first bytes `fd 37 7a 58 5a 00`, the XZ magic number. **And a plain alphabetical sort of the release listing picks 4.2.9 as the newest when 4.2.23 exists.**

**Not covered:** *"one moment on each machine, reach over hours or days untested, no proxy or egress-filtered box tested, and an actual extension package was never fetched — only the listing."* That last is [A1](../Server%20Setup.md#6-add-ons)'s named gap and [Section 13](../Server%20Setup.md#13-unresolved) item 3.

## Fact 25 — the user's own line, in both directions, and the later reading that disagrees

**Shape 1, pointer. Added 2026-08-29**, the stage-close gate having found the uplink figure cited to Fact 54, which carries no uplink figure at all. [Six machines](../Six%20machines%20—%20the%20container,%20the%20webhook%20and%20the%20ready%20action.md), *Fact 25*, **2026-08-15/16**; [Facts](../Facts.md), Fact 25. Label: **observed in a test**. Used by [B4](../Server%20Setup.md#51-the-list-and-the-fetch) and [Section 14](../Server%20Setup.md#14-moments-that-reached-the-creator)'s twelfth escalation check.

**What it answered, in the register's words.** *"`The rented machine.md` gives 1.3 MB/s down and 0.48 up; measured against the same home line, **12–15 MB/s down and 1.2–1.3 up**. **Down is 11–12 times up, and that asymmetry is the dominant fact about this connection.**"*

**Not covered**, in the register's own words: *"one home line on one evening over Wi-Fi, and two of the three rentals were the same provider."*

**A later reading of that same line disagrees, and where anything is priced on it the slower governs.** `Server protocol and transfers` R17 measured **up 0.62 MB/s against down 3.88** on the Norway box, **2026-08-22** — about six to one against Fact 25's eleven to twelve — and that document settled the rule on 2026-08-23 at S12 step 3: *"a comparison priced at the more favourable of two readings of the same line is priced at the one that might not hold."* B4 carries both figures with their own facts; Section 14's twelfth check prices at the slower.

## Fact 45 — searching a machine for every Blender on it

**Shape 1, pointer.** [Six machines](../Six%20machines%20—%20the%20container,%20the%20webhook%20and%20the%20ready%20action.md), *Fact 45*, five Blenders planted five ways against known ground truth, **2026-08-15/16**; [Facts](../Facts.md), Fact 45. Label: **observed in a test**. Used by [B1](../Server%20Setup.md#5-the-blender-lifecycle), [B10](../Server%20Setup.md#54-the-search), [B11](../Server%20Setup.md#54-the-search), [B8a](../Server%20Setup.md#53-where-a-build-goes) and [B12a](../Server%20Setup.md#54-the-search).

**What it answered.** *"A plain unpacked folder is invisible to every route but a whole-disk file search"*, and *"a renamed binary defeats the file search too, found only by running each candidate and reading its version output"* — a probe that *"would miss a small build and costs an execution per candidate"*. **The whole-disk search cost 5.3 seconds on a 126 GB disk 12% full.**

**Three traps that break the obvious implementation.** *"The apt build prints two lines of colour-management noise before its version, so a reader taking the first line gets none — every build must be parsed by finding the line beginning `Blender `, never by position."* *"Deduplication by resolved path is wrong for snaps: `/snap/bin/blender` resolves to `/usr/bin/snap`, so it would collapse every snap into one entry and could name the snap launcher as a Blender — the version each path reports is the reliable discriminator."* *"Restricting the search to a single filesystem, the ordinary way to keep it cheap, silently loses the snap, which is mounted from a loop device."*

**The ceiling, which [B11](../Server%20Setup.md#54-the-search) is written on.** *"A non-root login found three of four on the other machine and was refused the fourth outright."* And one real below-floor build turned up by accident: the apt build is **3.0.1**.

**Not covered:** *"flatpak was not installed on either box; neither tested a non-root login against a renamed binary; a Blender inside a container, on a network mount, or in a user's home directory is untested."*

## Asking a build what it is — the `Blender ` line and its two marks

**Shape 1 into Blender research.** [B1](../Server%20Setup.md#5-the-blender-lifecycle)'s parsing rule is Fact 45's (entry above). The two further marks the same line carries — whether the build is long-term-support, and whether it is a finished release — are the Blender question ***Builds 3***, as [Preparation/Specs/Sources/Server Setup.sources.md](../../Specs/Sources/Server%20Setup.sources.md) names it for this exact claim: *"That a build reports its own version, its long-term-support marking, and whether it is a finished release."* Label: **documented and observed**, *"except the long-term-support marking, which is **observed** only — no version's documentation carries it, and all four were searched"* — which is exactly the split [B1](../Server%20Setup.md#5-the-blender-lifecycle) labels in place.

**Not covered, in that record's words:** *"what an unstable build reports, and Windows"* — the document says the same of itself, *"untested by this project, every build tested having been a finished release"*.

## Running Blender with nobody there — the streams, the time limit, the engine list

**Shape 1 into Blender research, and the document names the joined answer itself.** [B1](../Server%20Setup.md#5-the-blender-lifecycle) cites ***Running Blender with nobody there*** — [Preparation/Blender Research/Across topics/Running Blender with nobody there.md](../../Blender%20Research/Across%20topics/Running%20Blender%20with%20nobody%20there.md) — for three things it states: that both streams are captured always and *"an empty error stream is never read as a property of having no window"*; that *"Anything running 4.5 unattended needs a time limit and something outside Blender ready to kill it; the other three versions do not"*; and that *"Until it is settled, do not get a build's engine list that way on an unattended machine."*

**Nothing is added here.** The joined answer is where the reading stops, per the Blender Research map's own rule; the words above are the document's quotations of it, and the document applies the time limit to all four versions of its own accord because it cannot know a candidate's version before asking — which is a decision and earns no citation.

## Installing beside a running render, and deleting a build underneath one

**Shape 1 into Blender research, named by the project's own record.** Used by [Y1](../Server%20Setup.md#31-two-classes-of-setup-run)'s additive class, [W3](../Server%20Setup.md#10-what-this-module-says-about-a-machine)'s table and [B14](../Server%20Setup.md#55-taking-a-blender-off). [Preparation/Specs/Sources/Server Setup.sources.md](../../Specs/Sources/Server%20Setup.sources.md) names the questions:

- **Builds 6** — *"That installing a Blender beside a running render leaves that render alone, and that copying one over the folder a render is using kills it on Linux."* Label: **observed**, both. Not covered: *"Windows, an installer as against a plain copy, and why the two operating systems part — nothing in the research explains it."*
- **Builds 8** — *"What becomes of a render whose Blender is deleted underneath it: it carries on and finishes."* Label: **observed**. Not covered: *"Windows, where the sources say the deletion may be refused outright, and an uninstaller as against a plain deletion."* That is exactly the bound [B14](../Server%20Setup.md#55-taking-a-blender-off) states in place.
- **Add-ons 12** — installing an add-on does not disturb a render already running: *"every frame was compared picture against picture rather than file against file — not one value differed"*, sixty-four comparisons for one run alone. Not covered: *"whether an install is harmless in general rather than at the rate and in the manner these were fired"* — which is the limit [Y1](../Server%20Setup.md#31-two-classes-of-setup-run) carries in place.

  **And a write-location condition, carried 2026-08-31 out of the repaired Blender research, which [A6a](../Server%20Setup.md#6-add-ons) states in the document.** **Every install on record went into a user repository, and nothing was ever written into the folder a running Blender was running from** — which is the one write **Builds 6** records as killing a render on Linux. Label: **observed in a test**. **So *an add-on install disturbed no running render* is a result about writes into a user repository and is not evidence about a write into a Blender's own folder.** [A6](../Server%20Setup.md#6-add-ons) already fixes the version-numbered settings folder as the target, so nothing Smoozy does enters the untested case; **what breaks without the condition** is [W3](../Server%20Setup.md#10-what-this-module-says-about-a-machine)'s *no* row for an add-on install, which would otherwise be resting on a wider result than the research carries. **A free mitigation is named by the research and is not taken here** — pointing the system-extensions variable outside every Blender's own folder — because taking it is a decision and this file makes none.

**And the reason [A8](../Server%20Setup.md#6-add-ons) rests on, from the same record:** *"Blender keeps a lock on its own add-on store, and a Blender that is rendering does not hold it"*; with one Blender holding it, an install fired from another *"refuses in under a second at a failing status, naming the session that holds it, rather than waiting"*; and *"a Blender that is already running does not pick a newly installed add-on up by itself"*.

## POSIX `unlink()` — what a removal does to a process holding the file

**Shape 2, publication.** `unlink()`, The Open Group Base Specifications Issue 8, read **2026-08-20** at `https://pubs.opengroup.org/onlinepubs/9799919799/functions/unlink.html`. Label: **documented**. Used by [Section 13](../Server%20Setup.md#13-unresolved) item 17, which is what lets the package-manager half of a removal be answered without a machine. Exact words: *"If one or more processes have such a reference to the file when the last link is removed, the link shall be removed before unlink() returns, but the removal of the file contents shall be postponed until there are no such references to the file."*

**Not covered:** the **snap** route, where the build is a read-only mount rather than ordinary files and removal must unmount it. Nothing in the standard reaches that, and item 17 keeps it open rather than borrowing the answer.

## Canonical's *System snap directory* — the mount is read-only squashfs

**Shape 2, publication.** Canonical's own *System snap directory*, read **2026-08-20** at `https://snapcraft.io/docs/explanation/how-snaps-work/system-snap-directory/`, giving the mount as `squashfs (ro,nodev,relatime,x-gdu.hide)`. Label: **documented**. Used by [B15](../Server%20Setup.md#55-taking-a-blender-off), which concludes that a path under the snap mount *"cannot be deleted by hand at all"* — the mount refuses the deletion rather than leaving records behind — and by [B12a](../Server%20Setup.md#54-the-search)'s read-only-folder case and [Section 13](../Server%20Setup.md#13-unresolved) item 17.

**Not covered:** what an unmount does to a render running from that mount — item 17's open half. The page is Canonical's description of the directory and says nothing about removal against a live process.

## A hand-deleted managed build leaves the manager's records — **no source in this project**

**Shape 3.** [B15](../Server%20Setup.md#55-taking-a-blender-off) states that *"a path under the system package areas is the manager's; deleting its files by hand leaves the manager's records lying about the machine."* **No publication and no observation in `Preparation/` supports it.** Searched: this document, [Facts](../Facts.md) (Facts 45 and 48 are the two that touch a machine's package manager, and neither poses this), the input documents, the other architecture documents and their sources files, and the Specs sources. Label: **an assumption**.

**What breaks if it is wrong:** nothing mechanical. The rule it justifies — remove a managed build through its manager — is also justified independently for the snap route by the read-only mount above, and the removal is a route choice rather than a claim to the user. If a hand deletion left no stale records, the manager route would merely be unnecessary on that half.

---

# Part 6 — Section 6: add-ons

*Every entry in this part is shape 1 into Blender research, **with one exception, and it is the last bullet: Fact 56 is shape 2, a marketplace's own support pages read 2026-09-02** (noted here 2026-09-02, the entry having been shape 3 before that and this sentence never having covered it). The question numbers are the ones [Preparation/Specs/Sources/Server Setup.sources.md](../../Specs/Sources/Server%20Setup.sources.md) records for these exact claims; nothing is assembled across topics here, and no words of Blender's are copied in.*

- **Where an add-on may come from ([A1](../Server%20Setup.md#6-add-ons)).** **Add-ons 5**, with **Add-ons 8**: *"a list of repositories held in Blender's settings … A fresh Blender holds three, a repository's address may be a plain folder path, and a package installs by name from a repository or straight from a file — never from a web address."* Label: **documented and observed**. **Not covered:** *"a site somebody else runs. No version's sources name an address for one, so none could be tried, and that case alone stays unresolved."* The machine's own reach to the extensions site is Fact 54's, **for the listing only** — see item 3.
- **Installing with no screen ([A1](../Server%20Setup.md#6-add-ons), [A2](../Server%20Setup.md#6-add-ons)).** **Add-ons 3**. Label: **observed**. Not covered: *"a machine with no graphics hardware at all, and Windows."* **Corrected 2026-08-31, out of the repaired Blender research: a machine with no graphics hardware is *untried here* rather than out of reach — one is on record under Frames 15.** This entry, and the register's note at Fact 46 that the no-card case *cannot be posed on the seller that was tried at all*, together read as though no such machine existed anywhere; that is not what the research now carries. **Nothing moves:** *Add-ons 3* was not run on it, so the gap in this entry is unchanged in substance — what changes is that it is a question nobody asked rather than a question nobody could ask.
- **A copy carried from the user's own computer ([A2](../Server%20Setup.md#6-add-ons)).** **Add-ons 4 and 8**: *"the install and the switch-on travel and the plain part works; the part built for the first machine does not, and nothing announces it."* Label: **observed**. **Not covered:** *"Windows, and a copy in the direction Smoozy actually sends one — the test packed on Linux and opened on a Mac."* That is [Section 13](../Server%20Setup.md#13-unresolved) item 4's first residue, carried in the document in place.
- **The declared-part failure that arrives only at use ([A2](../Server%20Setup.md#6-add-ons), [A5](../Server%20Setup.md#6-add-ons)).** **Add-ons 11**, with **Add-ons 4**: an add-on *"may declare a part it is meant to carry, install with no warning, report success, be switched on, and not have it — with the failure arriving only when something uses it, and that run reporting success too."* Label: **observed**. Not covered: Windows.
- **Which switch-on route reports ([A4](../Server%20Setup.md#6-add-ons)).** **Add-ons 13**: one route *"succeeds for a working add-on and raises for a broken one, naming the fault, tried against three different kinds of breakage"*; the other *"hands back nothing rather than raising"*, and switching on at launch *"prints its complaint and still ends at success"*. The three limits the document carries are that record's: the switched-on list *"records what was asked for rather than what loaded"*; asking Blender to look again *"reports nothing about damage"*; and the installed listing *"says nothing about health and nothing about on or off"*. Label: **observed**, all four versions, both machines. Not covered: *"what Blender's own add-on error status actually holds, which no version documents at all; and Windows."*
- **What an install's ending is worth ([A5](../Server%20Setup.md#6-add-ons)).** **Add-ons 14**: a declared Blender-version mismatch is *"refused by name, nothing is installed … and the command reports success"*; a compiled part built for the wrong Python is *"refused at a failing status"*; *"the same declaration is ignored entirely for an older-style add-on … and every version's manual says the opposite."* Label: **observed**. **This is the record that makes the exit status unreadable in both directions**, which is [A5](../Server%20Setup.md#6-add-ons)'s whole rule.
- **What can be read back, and the hole in it ([A5](../Server%20Setup.md#6-add-ons), [A6](../Server%20Setup.md#6-add-ons)).** **Add-ons 10**, with **Add-ons 13**: a machine can be asked which add-ons it holds and which are switched on, with no screen. **Not covered, and the document carries it in place:** *"no single route gives a complete inventory … switched-off older-style add-ons are covered by no route, which is the shape an artist's own hand-placed copy takes"*, so *"Smoozy can be sure something is there and can never be sure something is absent."* That residue is [Section 13](../Server%20Setup.md#13-unresolved) item 4's second.

  **The negative was firmer in the answer document than here, and it is narrowed there rather than here. Added 2026-09-02, out of the downstream-citation pass (Open Threads 133).** *Add-ons 10* lists this as **not established** — whether any route gives one complete list including switched-off older-style add-ons — so *"covered by no route"* is a silence about what was found rather than a demonstrated absence. **[A5](../Server%20Setup.md#6-add-ons), [A6](../Server%20Setup.md#6-add-ons) and [Section 13](../Server%20Setup.md#13-unresolved) item 4 are narrowed to *no route is established*, 2026-09-02**, and nothing they decide changes, every one of them resting on the uncertainty already. **The quotation above is deliberately left exactly as it stands.** It is a verbatim quotation of `Preparation/Specs/Server Setup.md`, and correcting a quotation would put it out of step with the thing it quotes; the same words are quoted again in that spec's own sources file and twice in `Rows/Server Setup.md`. **The spec moves first and the three quotations re-sync with it in one sitting** — that is routed and is not this file's to do.
- **Which folder an add-on belongs to, and the Python split ([A6](../Server%20Setup.md#6-add-ons)).** **Add-ons 6**, with **Add-ons 4 and 11** for the Python change: *"what keeps versions apart is the folder's version-numbered name and nothing else, and that name carries two numbers rather than three, so patch releases share"*, and the Python is *"3.11 for Blender 4.5 and 5.0 and 3.13 for 5.1 and 5.2"*. Label: **documented and observed**. Not covered: Windows; two builds of one version sharing a folder is **Builds 5** instead — settled there on one machine, in one run, for one pair of builds of one version from two sources.

  **A bound on the sharing tests, carried 2026-08-31 out of the repaired Blender research.** **All five sharing tests used a plain-Python add-on with no bundled compiled module**, and **across the Python change the install-time check cannot fire, because the second version installs nothing.** Label: **observed in a test**. **What breaks:** any reading of these tests as evidence about what happens to an add-on **with** a compiled part when two versions share a folder across the 5.0-to-5.1 Python change — the sharing runs never posed it, and the install-time refusal **Add-ons 14** records for a compiled part built for the wrong Python is not reached on this path. **What holds:** [A6](../Server%20Setup.md#6-add-ons)'s own sentence, which states the consequence rather than the check — an add-on with a compiled part installed for one Python is not usable by the other even where they share a folder — and that sentence is not what the sharing tests were carrying.
- **A build removed leaves its settings folder ([B15](../Server%20Setup.md#55-taking-a-blender-off), [A6](../Server%20Setup.md#6-add-ons)).** **Builds 5**, with **Builds 8** and **Builds 9**: *"two Blenders of the same version share one settings folder, and … a removal leaves that folder behind."* Label: **documented and observed**. Not covered: *"Windows, and whether the cache folder is shared."*
- **What a render does when an add-on it needs is missing ([A9](../Server%20Setup.md#6-add-ons)). REVERSED 2026-08-31, out of the repaired Blender research.** **Add-ons 1** stands: a `.blend` keeps no record of the add-ons it needs, with one exception — *"a file records its engine by name, and a Blender lacking that engine's add-on says so on opening"* — and that exception is the one thing [A9](../Server%20Setup.md#6-add-ons) lets cross from Pre-load. **The second half of this entry quoted the old answer word for word — a missing engine *"draws one line and the render falls back … writes an image and reports success"* — and the repaired research reverses it outright.** **Add-ons 7** now records that **where the engine is named at launch and its add-on is absent, no image is written**: **5.0, 5.1 and 5.2 end at 1, and 4.5 does not end at all** — killed on the Mac at status 134, and on a rented Linux machine it never exits on its own. Label: **observed in a test**. [Render Settings.sources.md](Render%20Settings.sources.md) records that Smoozy names the engine on the launch line, which is the condition this finding is stated under. **What breaks:** every sentence resting on *a missing add-on produces a finished job with the wrong picture rather than a failure* — what the user is weighing is not that at all, and on 4.5 **a rented machine can be left running a job that never ends**, which is the sharper fact. The 4.5 case is the same shape [B1](../Server%20Setup.md#5-the-blender-lifecycle) already applies a time limit for, on *Running Blender with nobody there*'s own rule that anything running 4.5 unattended needs a limit and something outside Blender ready to kill it. **This was already wrong before the repairs; they surfaced it.** **Nothing is decided here** — what A9 and the Pre-load hand-off now say is the document's, and this entry states only what the research carries.
- **Fact 56 — whether a copied add-on carries licence material ([A3](../Server%20Setup.md#6-add-ons)). ANSWERED BY READING, 2026-09-02, and this entry changes shape with it: it was shape 3 and is now shape 2.** ~~**Shape 3 by the register's own finding.** [Facts](../Facts.md), Fact 56: *"Neither a machine nor Blender's documentation can answer it — nothing in any version's documentation describes pricing, purchase, keys, activation or entitlement"*; it is answerable only by reading a particular seller's own documentation, which nobody has read. Label: **unresolved**.~~ **That last clause is what stopped being true.** The register's negative half stands untouched — no machine and no version's Blender documentation can answer it — and the seller's own documentation has now been read.

  **Publication, and it is the marketplace whose rules bind the paid Blender add-ons this question is about: Superhive, formerly Blender Market.** Two of its own support pages, both read **2026-09-02**. Label: **documented**. **Neither ruling of 2026-08-31 reached this — no machine was used, nothing was rented and nothing was spent.**

  *Available Licensing Options*, `https://support.superhivemarket.com/article/54-available-licensing-options`: *"All scripts, add-ons, and other code-based products may be licensed as either GPL or MIT."* and *"No other licensing options are currently permitted for code-based products."* That page's own table gives, for GNU GPL 3.0 and for MIT alike, **Allow Redistribution: y**, **Allows Modification: y**, and **Requires License & Copyright Notice Be Included with Software: y**.

  *How to Make Your Blender Add-on GPL Compliant*, `https://support.superhivemarket.com/article/294-how-to-make-your-blender-addon-gpl-compliant`: *"Along with the other files in your add-on, you should include one named LICENSE"*, whose content *"should be the entire text of the GPL"*; *"every Python file in your add-on should also include a license notice at the top of the file"*; and *"The best thing to do is pack your add-on into a folder. That way, you can include a proper LICENSE, README and other helpful files"*.

  **What it settles, in the direction [A3](../Server%20Setup.md#6-add-ons) already stood.** By the seller's own instruction a set-up add-on's folder carries licence material twice over — a LICENSE file beside the code and a notice at the head of every Python file — so **a copy carries licence text and a copyright notice, not an entitlement**. Both permitted licences allow redistribution and modification, so packing the folder and installing it on a machine the user rents is inside the seller's own terms, and **neither licence requires a seller-issued key for the copy to work**. **A3 needs no change** — Smoozy does not ask for, read or check a licence key — and what the module *says* about licences now cites this reading instead of deferring to an unread question.

  **Not covered, and the residue is narrower than the question was.** Neither permitted licence forbids an add-on making a runtime activation check of its own, so **whether any individual add-on additionally stores a seller-issued key inside its folder is not established**; and this is one marketplace's rules on one date rather than every seller's. **What breaks:** what the module *says* about licences, not what it does. **The silence is removed by the failure being the add-on's own** — an add-on that will not run unactivated says so itself — and Smoozy claims nothing about licences it has not read.

  **The blind-rewording warning is honoured rather than spent.** [Facts](../Facts.md), Fact 56 carries it — the question *"was raised by an agent from its own knowledge and must be reworded blind before anyone is asked"* — and **nobody was asked anything here; a publication was read.** The warning stands for the day the residue above is put to a person.

---

# Part 7 — Sections 8 to 10, and what remains

## Ubuntu's detection tool is in a package a plain image may not have (R1 item 5 and M1b, 2026-09-21)

**Shape 2, direct.** Ubuntu's own package index, the file list of `ubuntu-drivers-common` for the two releases the second pass rented: `https://packages.ubuntu.com/noble/amd64/ubuntu-drivers-common/filelist` and `https://packages.ubuntu.com/jammy/amd64/ubuntu-drivers-common/filelist`, both read **2026-09-21**. Label: **documented**. **Both list `/usr/bin/ubuntu-drivers`** — beside `/usr/bin/gpu-manager`, `/usr/bin/nvidia-detector` and `/usr/bin/quirks-handler` — so the tool R1 item 5 calls is that package's.

**Shape 1, the observation that made it matter.** `Audits/Build/Step 10c/Piece 12, the second pass — Hyperstack.md`, fault 1, **2026-09-21**, on Hyperstack's plain `Ubuntu Server 24.04 LTS (Noble Numbat)` image: *"sudo: ubuntu-drivers: command not found"*; `dpkg-query` found no `ubuntu-drivers-common`, and `apt-cache policy` gave *"Installed: (none)"* with a candidate available. Label: **observed in a test**, one image.

**What this carries:** R1 item 5's and M1b's Ubuntu first step — the package that holds the tool is installed before the tool is called. **Not covered:** whether the image's package lists are always current enough to find the package (on the one image read they were); and Ubuntu's server page, which calls the tool and names no package for it.

## The package lists are refreshed before every install (S7d, R1 item 5 and M1b, 2026-09-22)

**Shape 2, direct.** apt's own manual, `apt-get(8)`, as Debian publishes it for trixie: `https://manpages.debian.org/trixie/apt/apt-get.8.en.html`, read **2026-09-22**. Label: **documented**. Under *update*: *"update is used to resynchronize the package index files from their sources. The indexes of available packages are fetched from the location(s) specified in /etc/apt/sources.list."* **Ubuntu's own copy of the same page, `https://manpages.ubuntu.com/manpages/noble/en/man8/apt-get.8.html`, answered *503 Service Unavailable* when it was asked the same day**, so the Debian copy is the one read; the page is apt's own and both distributions ship apt.

**Shape 2, direct.** apt's own source, `apt-private/private-install.cc` on the project's main branch: `https://salsa.debian.org/apt-team/apt/-/raw/main/apt-private/private-install.cc`, read **2026-09-22**. Label: **documented**. Twice, where a fetch of the archives fails: `_error->Error(_("Unable to fetch some archives, maybe run apt update or try with --fix-missing?"))`. **The machine on the third pass printed the same sentence with `apt-get update` in place of `apt update`** — the older wording of the release that image carries; the main branch is what was read.

**Shape 2, direct.** Ubuntu Server's own page *Install and manage packages*, `https://documentation.ubuntu.com/server/how-to/software/package-management/`, read **2026-09-22**. Label: **documented**. Under *Update the package index*: *"To update the local package index with the latest changes made in the repositories, and thereby access the most up-to-date version of the package you’re interested in, run the following: $ sudo apt update"*, before its *Install a package*.

**Shape 1, the observation that made it matter.** `Audits/Build/Step 10c/Piece 12, the third pass — Hyperstack.md`, **2026-09-22**, on Hyperstack's plain `Ubuntu Server 24.04 LTS (Noble Numbat)` image: the newest package list on the machine was dated 2025-06-15 and `update-success-stamp` read 2025-06-16; the install of `ubuntu-drivers-common` asked for `1:0.9.7.6ubuntu3.2`, the mirror answered *"404  Not Found"* for that file, and apt printed *"E: Unable to fetch some archives, maybe run apt-get update or try with --fix-missing?"*. The image's own automatic upgrade ran between the two attempts and did not refresh the lists. Label: **observed in a test**, one image.

**What this carries:** S7d — every package install of a setup run is preceded by `apt-get update` under S7b's bounded wait — and the corrections to R1 item 5 and M1b. **It replaces the *not covered* of the entry above**, which recorded that the one image read then had lists current enough. **Not covered:** that a refresh makes every install succeed (a mirror that is down fails the refresh); whether a refresh may precede S7's repair of an interrupted transaction, which no route issues and nobody has read; and what `DPkg::Lock::Timeout` does for the lists' own lock, which is S7b's item 19 and not this entry's.

## The driver check is in a package the driver install does not bring (R1 item 5, M1b and E7, 2026-09-23)

**Shape 2, direct.** Ubuntu Server's own page *NVIDIA drivers installation*, at its current address `https://documentation.ubuntu.com/server/how-to/graphics/install-nvidia-drivers/`, read **2026-09-23**. Label: **documented**. Under *Installing the drivers on servers and/or for computing purposes*, after `sudo ubuntu-drivers install --gpgpu` and its named-driver form, the page says: *"You will also want to install the following additional components:"* and prints `sudo apt install nvidia-utils-535-server`. **It is the same page the entry of 2026-08-20 in Part 7 above reads for the `--gpgpu` command itself** — that entry quoted the install and stopped one sentence short of this one. *(The old address `https://ubuntu.com/server/docs/how-to/graphics/install-nvidia-drivers/` now redirects to this one.)*

**Shape 2, direct.** Ubuntu's own package index, searched by file contents for the exact filename `nvidia-smi` in `noble` on `amd64`: `https://packages.ubuntu.com/search?searchon=contents&mode=exactfilename&suite=noble&arch=amd64&keywords=nvidia-smi`, read **2026-09-23**. Label: **documented**. *"Found 1 results."* — `/usr/bin/nvidia-smi`, in `nvidia-utils-470`, `nvidia-utils-470-server`, `nvidia-utils-535`, `nvidia-utils-535-server`, `nvidia-utils-550` and `nvidia-utils-550-server`. **No unversioned package, no virtual package and no `-open` variant publishes it**, which is why the branch has to be derived at all and why a trailing `-open` is dropped from the derived name.

**Shape 2, direct.** The same index, searched by package name for `nvidia-utils` in `noble-updates`: read **2026-09-23**. Label: **documented**. *"Found 22 matching packages"* — branches 460, 465, 470, 530, 535, 550, 565, 570, 575, 580, 590, 595 and 610, most of them in both the plain and the `-server` form. **This is the entry behind the refusal to name the branches at build time**: a fixed list here would be twenty-two flavours named in a clause that forbids naming one, and would go stale the day Canonical ships the next.

**Shape 2, direct.** The tool's own program text, `ubuntu-drivers` as Ubuntu publishes the source of `ubuntu-drivers-common`: `https://git.launchpad.net/ubuntu/+source/ubuntu-drivers-common/plain/ubuntu-drivers?h=ubuntu/noble-updates` and the same file at `?h=ubuntu/jammy-updates`, both read **2026-09-23**. Label: **documented**. `list` carries `--recommended`, whose own help reads *"Only show the recommended driver packages"*; where it is given, the command prints one line and stops (`break`), and the source's own comment at that print says *"This is just a space separated two item line"* and *"Such as \"nvidia-headless-no-dkms-470-server linux-modules-nvidia-470-server-generic\""*. **Both releases carry it identically.** And `list --gpgpu` and `install --gpgpu` draw from the same routine, `UbuntuDrivers.detect.system_gpgpu_driver_packages`, under the same comparator — which is what makes the tool's recommendation the install's own choice rather than a second opinion. **The install's own apt call is visible in the same file** and carries `--no-install-recommends`, so nothing the driver package merely recommends arrives with it.

**Shape 1, the observation that made it matter.** `Audits/Build/Step 10c/Piece 12, the fifth pass — Hyperstack.md`, fault 1, **2026-09-23**, on Hyperstack's plain `Ubuntu Server 24.04 LTS (Noble Numbat)` image with an RTX A6000: `ubuntu-drivers install --gpgpu` placed thirteen packages including `nvidia-headless-no-dkms-595-server-open` and `nvidia-compute-utils-595-server`; `/proc/driver/nvidia/version` read *NVRM version: NVIDIA UNIX Open Kernel Module for x86_64 595.91.07*, `/dev/nvidia0` was there and the card answered — and `apt-cache policy nvidia-utils-595-server` read *Installed: (none)*, with `find / -name nvidia-smi` finding none. Label: **observed in a test**, one image.

**And it was recorded twice before, under this clause, and built against neither time.** [Facts](../Facts.md) Fact 47, **2026-08-16**: on Hyperstack, *"`ubuntu-drivers install --gpgpu` does not install `nvidia-smi`"*. Part 7's entry of **2026-08-23** for the `--gpgpu` command carries the same sentence in its own *Not covered*. **The gap was never in the sources; it was between the sources and the clause.**

**What this carries:** R1 item 5's and M1b's Ubuntu fourth act — the driver support binaries installed after the driver, the publication's own additional-components step — and E7's fifth carriage, which is how the branch reaches the command without crossing Smoozy. **Not covered:** what the tool prints on a machine with two different cards, which nobody has posed; whether `--recommended` names the same package the install chose on a machine whose archive changed between the two commands (they run seconds apart in one act); and whether the derived package exists in every image's archive — where it does not, apt says so and the step ends in the machine's own words.

## Which package publishes each library Blender asks for (R1 item 4 and E7, 2026-09-22)

**Shape 2, direct.** Ubuntu's own package index, searched by file contents for an exact filename in `noble` on `amd64`: `https://packages.ubuntu.com/search?searchon=contents&mode=exactfilename&suite=noble&arch=amd64`, eleven searches, all read **2026-09-22**. Label: **documented**. Each answered with one package holding the file under `/usr/lib/x86_64-linux-gnu/`:

| the library `ldd` names | the package the index puts it in |
|---|---|
| `libX11.so.6` | `libx11-6` |
| `libXi.so.6` | `libxi6` |
| `libXxf86vm.so.1` | `libxxf86vm1` |
| `libXfixes.so.3` | `libxfixes3` |
| `libXrender.so.1` | `libxrender1` |
| `libGL.so.1` | `libgl1` |
| `libEGL.so.1` | `libegl1` |
| `libSM.so.6` | `libsm6` |
| `libICE.so.6` | `libice6` |
| `libxkbcommon.so.0` | `libxkbcommon0` |
| `libXext.so.6` | `libxext6` |

Two searches answered with a second, unrelated package as well and neither is the one taken: `libX11.so.6` is also under `nxagent` at `/usr/lib/x86_64-linux-gnu/nx/X11/`, and `libGL.so.1` under `primus-libs` at `/usr/lib/x86_64-linux-gnu/primus/`. **The package taken is the one holding the ordinary library path.**

**Shape 2, direct.** `libxext6`'s own file list, `https://packages.ubuntu.com/noble/amd64/libxext6/filelist`, read **2026-09-22**. Label: **documented**. It lists `/usr/lib/x86_64-linux-gnu/libXext.so.6` beside `libXext.so.6.4.0`.

**Shape 1, the observation that made it matter.** `Audits/Build/Step 10c/Piece 12, the fourth pass — Vast.md`, fault 1, **2026-09-22**, on a bare `ubuntu:24.04` container on Vast: `ldd` over a fetched Blender 5.2.2 gave ten not-found lines, **nine distinct** — `libX11.so.6` twice, plus `libXrender.so.1`, `libXfixes.so.3`, `libXi.so.6`, `libxkbcommon.so.0`, `libSM.so.6`, `libICE.so.6`, `libXext.so.6`, `libGL.so.1` — and the step refused because eight of the nine had no key in the mapping. Label: **observed in a test**, one image.

**What this carries:** R1 item 4's rule that an entry answers for every library its own packages publish, and the eleventh package name the entry gained. **Not covered:** whether these package names are the same on Debian, which this project installs on too and which was not searched; and whether some other image needs a package no entry names, which the step is built to refuse honestly rather than guess at.

## A library Blender opens by name, and the same packages on 22.04 (R1 item 4, 2026-10-11)

**Shape 1, the observation.** `Audits/Build/Step 10c/The Creator's eighth notes, notes 5 and 6 — what failed and why.md`, section 2: a render's error stream on a RunPod machine on *Base Ubuntu 22.04* (the Creator's word) said *Couldn't open libEGL.so.1*, and `ldd` over 5.2.2 on a bare `ubuntu:24.04` container on 2026-09-22 named nine libraries, `libEGL.so.1` not among them. Label: **observed**. That Blender opens it by name rather than linking it is an **inference** from the two.

**Shape 2, direct.** Ubuntu's own file list for each of the entry's eleven packages in `jammy` (22.04) on `amd64`, `https://packages.ubuntu.com/jammy/amd64/<package>/filelist`, all read **2026-10-11**. Label: **documented**. Every one exists under the same name as in `noble` and lists the same library under `/usr/lib/x86_64-linux-gnu/` that the table above gives it — `libegl1` lists `/usr/lib/x86_64-linux-gnu/libEGL.so.1` beside `libEGL.so.1.1.0`. **So the entry needs no second release's names.**

**Shape 2, direct.** The loader the lookup asks, on 22.04: `ld.so(8)` at `https://manpages.ubuntu.com/manpages/jammy/en/man8/ld.so.8.html`, read **2026-10-11**, carries the same two sentences `the_optix_library.rs` quotes from `noble` — *"A list of additional, user-specified, ELF shared objects to be loaded before all others"* and *"causes the program to list its dynamic dependencies, as if run by ldd(1), instead of running normally"* — and glibc 2.35's `elf/rtld.c` (`https://sourceware.org/git/?p=glibc.git;a=blob_plain;f=elf/rtld.c;hb=refs/tags/glibc-2.35`, read the same day) prints each object as `"\t%s => %s (0x%0*Zx)\n"` and a preload it cannot find as *"cannot be preloaded (%s): ignored."* Label: **documented**.

**Not covered:** whether the card can draw through EGL once the library is present, and whether its absence stopped the render of note 5.

## The swap up to the OptiX bar, and the fabric manager (R1 item 5 and E7, 2026-10-11)

**Shape 2, direct.** Ubuntu's own package index, read **2026-10-11**: `nvidia-driver-580-server-open` and `nvidia-driver-580-server` at `580.178.04-0ubuntu0.22.04.1` (jammy-updates, jammy security) and `580.178.04-0ubuntu0.24.04.1` (noble-updates, noble security), section *restricted* (`https://packages.ubuntu.com/search?keywords=nvidia-driver-580-server&searchon=names&suite=all&section=all`; the `-open` search the same way, read by the third piece). Each depends on `libnvidia-gl-580-server`, `nvidia-utils-580-server`, `nvidia-kernel-common-580-server` and its flavour's `nvidia-dkms-580-server…` (`https://packages.ubuntu.com/noble-updates/nvidia-driver-580-server` and `…/nvidia-driver-580-server-open`). `nvidia-fabricmanager-580` and `libnvidia-nscq-580` at the same two versions, section *multiverse*; the fabric manager's one dependency is `nvidia-kernel-common-580-server-580.178.04` (`https://packages.ubuntu.com/search?keywords=nvidia-fabricmanager-580&searchon=names&suite=all&section=all`, `…?keywords=libnvidia-nscq-580…`, `https://packages.ubuntu.com/noble-updates/nvidia-fabricmanager-580`, `https://packages.ubuntu.com/jammy-updates/nvidia-fabricmanager-580`). Label: **documented**.

**Shape 2, direct.** Ubuntu Server, *NVIDIA drivers installation*, `https://ubuntu.com/server/docs/how-to/graphics/install-nvidia-drivers/`, read **2026-10-11**: *Uninstalling the NVIDIA drivers* gives `sudo apt --purge remove '*nvidia*${DRIVER_BRANCH}*'`; under the servers heading, *"If your system comes with NVswitch hardware, then you will want to install Fabric Manager"*, `sudo apt install nvidia-fabricmanager-535 libnvidia-nscq-535`, the fabric manager matching the `-server` driver. Label: **documented**.

**Shape 2, direct.** NVIDIA's README for 580.105.08, `https://download.nvidia.com/XFree86/Linux-x86_64/580.105.08/README/kernel_open.html`, read **2026-10-11**: the loaded flavour shows in `/proc/driver/nvidia/version` as *"NVRM version: NVIDIA UNIX Open Kernel Module for x86_64"* (open) or *"NVRM version: NVIDIA UNIX x86_64 Kernel Module"* (proprietary); the flavour ranges are the ones R1 item 5 already quotes. Label: **documented**.

**Shape 2, direct.** `apt-get(8)`, apt 2.8.3, `https://manpages.ubuntu.com/manpages/noble/en/man8/apt-get.8.html`, read **2026-10-11**: `-s` *"No action; perform a simulation of events that would occur based on the current system state but do not actually change the system"*; *"apt-get returns zero on normal operation, decimal 100 on error"*; `--no-install-recommends` *"Do not consider recommended packages as a dependency for installing"*. Label: **documented**.

**Not covered:** that `apt` takes apt-get's `-y` and `-o` options, and that the headers package is `linux-headers-` followed by the running kernel's release (both **assumptions**, each failing before anything is removed or at the removal itself); whether a trial run with the old branch's packages still installed resolves on an image carrying NVIDIA's archive; whether *multiverse* is enabled on a given image; and an unversioned fabric manager package from NVIDIA's archive, which the removal's pattern does not reach. All are on the rental pass's watch-list in `Audits/Build/Step 10c/OptiX and the driver, piece O3b — what landed.md`.

## The restart, confirmed on a second seller

**Shape 1, pointer.** [The multi-provider batch](../The%20multi-provider%20batch%20—%20the%20surface%20that%20did%20not%20vary,%20the%20Debian%20route%20that%20exists,%20and%20the%20cards%20a%20slice%20cannot%20see.md), *Test 17*, **2026-08-22**. Label: **observed in a test**. Used by [T1](../Server%20Setup.md#8-the-restart)'s second paragraph and [T6](../Server%20Setup.md#8-the-restart).

**What it answered.** `sudo systemctl reboot` issued over the SSH connection already open **returned 0 and the connection dropped**; the box came back in **49 seconds**, and in **55** on a second restart later the same day — *"both inside the 9-to-90-second range already on record, which is why nothing budgets a fixed wait."* **The address was unchanged**, and **all three host keys were byte-identical** before and after, the input document recording each fingerprint.

**Not covered:** the same act **on a container**, where there is no service manager to ask — not posed, and [R3](../Server%20Setup.md#21-what-the-action-is-made-of) forbids it there on Fact 48's ground, so nothing needs the answer.

## The machine is on UTC and knows no time zone

**Shape 1, pointer.** [The rented machine](../The%20rented%20machine.md) §7: *"The machine is on UTC. Every file listing in this document shows a UTC time, because that is the only time the machine has. It has no idea what time zone anyone using it is in."* (`Time zone: Etc/UTC (UTC, +0000)`.) Label: **observed in a test**. Used by [Y6a](../Server%20Setup.md#32-the-recorded-safe-boundaries)'s zone-shift clause.

**Not covered:** one box. Nothing establishes that every rental is on UTC; [Y6a](../Server%20Setup.md#32-the-recorded-safe-boundaries) does not need it to be, because the shift uses the desktop's zone setting and reads none of the machine's clock, and `State, identity and recovery` D5 forbids the subtraction either way.

## A machine's own settings hold the provider's secrets in plain text

**Shape 1, pointer.** [The rented machine](../The%20rented%20machine.md): *"Those same settings also hold **secrets in plain text** — an access key for the provider's own service, and a password for a web page it runs. They are not quoted here. Anything that reads the machine's settings and writes them into a log would be writing those out too."* Label: **observed in a test**. Used by [E6](../Server%20Setup.md#9-elevation-the-refusal-and-the-password)'s prohibition on reading the machine's settings into any log.

**Not covered:** one box, one provider. The prohibition is Smoozy's own rule and stands whatever another image holds; this observation is the reason recorded for it.

## `systemd.unit(5)` — the start limiter is a unit setting

**Shape 2, publication; the address is carried by the project rather than by this document.** `systemd.unit(5)`, read **2026-08-20** at `https://www.freedesktop.org/software/systemd/man/latest/systemd.unit.html`, per [State, identity and recovery.sources.md](State,%20identity%20and%20recovery.sources.md) and [Desktop app and delivery.sources.md](Desktop%20app%20and%20delivery.sources.md), which record the same read. Label: **documented**. Used by [F1](../Server%20Setup.md#9b-the-user-facing-fallback-re-run) step 3.

**Exact words the document uses:** `StartLimitIntervalSec=` is a unit setting that *"may be set to 0 to disable any kind of rate limiting"*.

**The observation behind the step.** [The last two boxes](../The%20last%20two%20boxes%20—%20the%20hook%20that%20cannot%20be%20read%20back,%20the%20full%20disk,%20and%20the%20unit%20that%20kills%20a%20render.md) §1.3: on the virtual machine, after a full disk, the unit went to `failed` and *"it does not recover when space is freed"* — watched three minutes at thirty-second intervals — *"A plain start command was enough to recover it, but nothing on the machine performs one and a reboot does not either."* Label: **observed in a test**.

**Not covered:** *"what the server package's own placement writes"*. The observed unit was left at its defaults, which is why the step is not withdrawn on the strength of a setting nobody has chosen; whether the placement disables the limiter is `Rented-server runtime`'s and is not settled there. The same input document's ten-in-two-seconds figure **does not match** the documented default (`systemd-system.conf(5)`, read the same day: `DefaultStartLimitIntervalSec=` `10s`, `DefaultStartLimitBurst=` `5`) and *"wants re-reading before that number is repeated anywhere"* — this document repeats no such number.

## The device line a finished job carries

**Shape 1 into Blender research.** [W4](../Server%20Setup.md#10-what-this-module-says-about-a-machine) reads one thing from a finished job. The Blender question is ***What actually ran 1***, as [Preparation/Specs/Sources/Server Setup.sources.md](../../Specs/Sources/Server%20Setup.sources.md) names it: *"Yes, in every version, and every result behind it is Cycles' own report — whether an EEVEE or Workbench job names anything is not established … none of the four prints it unless asked."* Label: **observed; how you ask differs by version**.

**The bound this entry quoted as W4's is false, and it was the whole ground for trusting a line found on the stream. Corrected 2026-08-31, out of the repaired Blender research.** This entry carried, as the comparison's own words, that the line is *"a closing report, printed after the last sample and before the frame is written, so a render stopped part-way never reaches it"*. ***What actually ran 6*** now records that **after a stop, 5.0 and 5.1 print the whole closing block, 5.2 prints a cancelled line and no block, and 4.5 prints nothing** — and that **5.2's own published fix list calls that block on exit fake, with wrong numbers**. Label: **observed in a test**, with that version's own publication behind the last part. **So a device line observed on a run that did not finish is not evidence of anything.** **What breaks, and it is why this is not a wording change:** as [W4](../Server%20Setup.md#10-what-this-module-says-about-a-machine) stood, a fabricated closing block printed by a run that did not finish could be read as the job's devices, declared a setup fault, and offer a re-run of the ready action that can restart the machine. W4 and [W3a](../Server%20Setup.md#10-what-this-module-says-about-a-machine) were repaired the same day, gating the reading on the job's **recorded outcome** rather than on the line's presence; `Queue & History` E12 is the rule and this document states no second one.

**The limits that stand.** *"That timing established on the Linux machine alone"*. The settings read back afterwards *"do not answer it and can be shown to say the wrong thing outright"*. And the render-time layer from Blender 5.0 *"follows the scene's setting and pays no attention to what actually ran."*

**Not covered:** *"whether a card running out of memory announces the move into the machine's own memory … anything on Windows, on a card of another make, or on a machine with two cards."* **The route and the record on the machine are `Rendering` L26's** and are cited there rather than re-taken here.

## A machine rendering on the processor while reporting success

**Shape 1, pointer.** [Facts](../Facts.md), Fact 48 for the container mismatch — *"under a deliberate mismatch Blender rendered on the processor and exited 0 with the device setting still reading `GPU`"* — and [Preparation/Specs/Sources/Server Setup.sources.md](../../Specs/Sources/Server%20Setup.sources.md)'s **The machine 2 and 4** for the Blender half: *"a machine whose graphics driver is missing or wrong still renders, on the processor, and reports success."* Label: **observed**. Used by [R3](../Server%20Setup.md#21-what-the-action-is-made-of), [R4](../Server%20Setup.md#21-what-the-action-is-made-of), [W2](../Server%20Setup.md#10-what-this-module-says-about-a-machine) and [W4](../Server%20Setup.md#10-what-this-module-says-about-a-machine).

**Not covered:** *"whether a card that is present but below a stated driver floor behaves the same way, and Windows."* This is the entry behind the module's refusal to claim that a machine renders on its cards.

## A Blackwell card renders, and what is still not established about it

**Shape 1, pointer.** [The multi-provider batch](../The%20multi-provider%20batch%20—%20the%20surface%20that%20did%20not%20vary,%20the%20Debian%20route%20that%20exists,%20and%20the%20cards%20a%20slice%20cannot%20see.md), **2026-08-22**. Label: **observed in a test**. Used by [Section 13](../Server%20Setup.md#13-unresolved) item 18.

**What it answered.** Both Blender versions ran on a **Blackwell RTX 5090** under driver `595.71.05`, on both backends, at one card and at two, **with no CUDA toolkit installed** and no failure of any kind.

**Not covered, and it is what leaves item 18 open.** *"On those boxes the seller injected the driver and no detection route ran, so the question this item asks was not posed"* — what the distribution's own detection route installs for such a card is unestablished.

## The rentable-Linux evidence the Creator was shown

**Shape 3 in part.** [Section 14](../Server%20Setup.md#14-moments-that-reached-the-creator) records two figures behind the ruling of **2026-08-22**.

- **1,700 sampled offers on one provider, Ubuntu and nothing else, 20.04 through 26.04.** This exists in the project outside the document: [Whole app](../../Specs/Whole%20app.md)'s platform boundary — *"on 2026-08-22, 1,700 offers sampled from one provider across several orderings, which is a sample of what it rents and not the whole of it, were Ubuntu and nothing else"* — and [Open Threads](../../Groundwork/Open%20Threads.md) restates it. **No address, no method and no record of the query exist anywhere**, so it is a figure the project carries rather than a citation. Label: **observed**, uncited. **The document was corrected on 2026-08-23 in two ways**: it introduced these figures as *"what is actually rentable"*, dropping the bound the owning spec states in the same sentence — *"a sample of what it rents and not the whole of it"* — which is now restored and attributed; and it now says outright that no method, query or address for the sampling is recorded anywhere in `Preparation/`, so a reader knows the figure cannot be re-taken.
- **Seventeen images on a second provider, fifteen Ubuntu, one Debian and one AlmaLinux — struck 2026-08-23.** **That breakdown appeared nowhere in `Preparation/` but in this document, and no record of the run that produced it exists**, so it could be neither cited nor re-run. **What stands in its place is what the project holds**: [The multi-provider batch](../The%20multi-provider%20batch%20—%20the%20surface%20that%20did%20not%20vary,%20the%20Debian%20route%20that%20exists,%20and%20the%20cards%20a%20slice%20cannot%20see.md)'s *"Hyperstack, by contrast, offers **Ubuntu, Debian and AlmaLinux**"*, image lists read 2026-08-22 — which agrees in kind and carries no counts — and [Whole app](../../Specs/Whole%20app.md)'s *"a second provider offered Ubuntu, Debian and one other"*. Label of what remains: **observed in a test**, families only. **What the document loses by the strike is the proportion**: the second provider's list is known to hold all three families and not in what numbers, so the document can no longer say how thin the non-Ubuntu end of it was.

**What breaks if the figures are wrong:** nothing this document decides. **The ruling itself is the Creator's and is a decision** — Ubuntu and Debian, Creator-stated 2026-08-22, carried by [Server Setup](../../Specs/Server%20Setup.md) *Getting the machine ready* and by [Whole app](../../Specs/Whole%20app.md) — and a decision earns no citation. What the figures are is the ground the Creator was shown, and a ground nobody can re-read is worth recording as such.

**Not covered:** one day on each provider, and lists read rather than rentals attempted per family. Neither figure says what is rentable at any other moment, and the first is a sample by its own statement.

---

# Part 8 — Claims this file could find no source for

Gathered here as well as in place, because a shape-3 entry is the one a reader most needs to find.

**Four of these were address gaps and were closed on 2026-08-23 by reading the publications; they are kept in the list, marked, so a reader looking for the old gap finds where it went.**

1. **CLOSED 2026-08-23.** **`tar` is Essential and `xz-utils` is not**, with the Debian Policy definition and the package pages ([R1](../Server%20Setup.md#21-what-the-action-is-made-of) item 3, [M1b](../Server%20Setup.md#1-the-machine-kind-and-what-a-setup-can-be-on-each)) — was words recorded with **no address and no date anywhere in the project**. Debian Policy §3.8 and four package pages read and addressed; every clause held.
2. **CLOSED 2026-08-23.** **sudo-rs's two option descriptions and Canonical's prompt-text comparison** ([Section 1](../Server%20Setup.md#1-the-machine-kind-and-what-a-setup-can-be-on-each)'s closing paragraph) — was words and a read date of 2026-08-20 with **no address anywhere in the project**. The project's own manual and README and Canonical's page read and addressed; **one attribution was wrong and was corrected** — Canonical's page does not confirm the default install.
3. **CLOSED 2026-08-23.** **`dpkg(1)`'s four quotations** ([S7](../Server%20Setup.md#43-why-each-step-is-safe-to-run-again)) — was read 2026-08-20 with no address in this document. Read at `https://manpages.debian.org/trixie/dpkg/dpkg.1.en.html`; all four hold.
4. **CLOSED 2026-08-23.** **The kernel's `ip-sysctl`** ([Y7](../Server%20Setup.md#33-the-worker-the-driving-and-the-stop), item 7, by way of Fact 57) — was identified by site and document name with **no URL anywhere in the project**. Read at `https://docs.kernel.org/networking/ip-sysctl.html`; all three values hold. `Servers` and `Rented-server runtime` carry the same gap and repair their own.
5. **"an SSH session is not a local seat"** (the opening paragraph) — the document's reading of an observed refusal; no publication on it was read.
6. **"deleting a managed build's files by hand leaves the manager's records lying about the machine"** ([B15](../Server%20Setup.md#55-taking-a-blender-off)) — no publication and no observation.
7. **The seventeen-image breakdown** ([Section 14](../Server%20Setup.md#14-moments-that-reached-the-creator)) — **struck from the document 2026-08-23**: the families are observed, the counts were in this document alone and no record of the run existed. The 1,700-offer figure beside it stays, because the project does hold it, and is now stated with the owning spec's own bound.
8. **The repair sequence has never been run** ([S7](../Server%20Setup.md#43-why-each-step-is-safe-to-run-again), item 5) — documented in two publications, observed nowhere, and the one observation it ever had was of the forced form the rule now forbids.
9. **The frontend lock's bounded wait has no established value** ([S7b](../Server%20Setup.md#43-why-each-step-is-safe-to-run-again), item 19) — the setting appears in apt's source and in neither `apt.conf(5)` nor `apt-get(8)`, and the one run at boot did not meet the collision.
10. **What a manager-route removal does to a render already running from that build** ([B14](../Server%20Setup.md#55-taking-a-blender-off), item 17) — [Facts](../Facts.md) carries it as a numbered fact exposed by this document's own Round 1 on 2026-08-19; the package-manager half is answered by `unlink()` above and **the snap half is not**.
11. **Whether the elevated access separation can distinguish a wrong password from a refused login** ([E4](../Server%20Setup.md#9-elevation-the-refusal-and-the-password), item 6) — the three refusals **are** distinguishable word for word by observation. **Corrected 2026-08-31: the separation is no longer refused outright.** `Security and licence` §13 rule 5's prohibition on mining a tool's output gained a **Creator-stated exception on 2026-08-25**, reaching this tool alone, under which the wrong-password ending is matched as an exact observed constant — so the wrong-password case **is** told apart and the password is erased rather than kept. That rule and its exception are Smoozy's own and earn no entry; what is recorded here is that the observation above is what the exception's line is drawn on, and that every other refusal is still unmatched by that document's own bound. **The rental's own `sudo` release is no longer unread — 1.9.15p5 was read 2026-08-23** and the EXIT VALUE sentence the separation rests on is word for word the 1.9.17p2 page's.
12. **Secure Boot** ([R9](../Server%20Setup.md#25-secure-boot-and-the-machine-smoozy-cannot-make-ready), item 1) — unposable on the seven machines rented, and no publication on module signing read.
13. **Whether a copied add-on carries licence material** ([A3](../Server%20Setup.md#6-add-ons), item 4) — unresolved by the register's own finding, and not posable from here.
14. **Windows and macOS for every operation on the user's own computer** ([Section 13](../Server%20Setup.md#13-unresolved) item 11) — the Blender records' *Not covered* lines name Windows throughout, and the one packed-copy observation ran Linux-to-Mac, the reverse of Smoozy's direction.

---

*Written by Claude Opus 5, 2026-08-23, against the whole of [Server Setup](../Server%20Setup.md) from its first line to its last. Nothing was looked up on the first pass — every citation was one that already existed under `Preparation/`, and where the project holds no source the entry says so and names where it was searched. This file supersedes the six-entry version written to no brief; every entry that stood in it is carried forward whole, with its use, its label and its limit stated.*

*Repaired the same day by Claude Opus 5, to a numbered findings list. **The four address gaps that pass recorded rather than filled are filled** — the Essential packaging claims, sudo-rs and Canonical's prompt comparison, `dpkg(1)`, and the kernel's `ip-sysctl` — each by reading the publication itself and recording the address and today's date, and a fifth publication was read that no gap named: `sudo(8)` at **1.9.15p5**, the release a rental actually ran, against which this document had only the Creator's Mac's page at 1.9.17p2. **Two of those reads contradicted what the document carried and both were corrected**: Canonical's sudo-rs page does not confirm that Ubuntu 25.10 and 26.04 install sudo-rs by default — only the project's own README states it — and the 15-minute credential cache observed on a rental is that release's own documented default rather than a divergence from documentation.*

*Carried forward 2026-09-02 by Claude Opus 5 (Claude Code subagent), an implementer under the architecture runner, the twentieth sitting. **The Fact 56 entry in Part 6 changed shape, from *no source in this project* to a publication quoted directly**, on the two Superhive support pages the register read that day: a code product may be GPL or MIT and nothing else, both require the licence and copyright notice to travel with the software, and the seller's own compliance article asks for a LICENSE file in the add-on folder and a notice at the head of every Python file — so a copied add-on carries licence **text**, not an entitlement, and A3 needs no change. **Part 6's opening sentence now names that entry as its one exception**, which it never covered. **Nothing was rented, nothing was spent, no answer document was touched and no git command was run. No auditor has seen this text.***


## The shop-front read again, 2026-10-27

**Shape 2, publication.** The Snap Store's Blender Foundation package endpoint, `https://api.snapcraft.io/v2/snaps/info/blender`, requested with `Snap-Device-Series: 16`. Read **2026-10-27**, HTTP 200, first by the nineteenth-notes map and again by this builder. Label: **observed**. Used by `Server Setup` B3.

The `channel-map` named `latest/stable` version **5.2.2**, released `2026-09-15T11:03:13.935007+00:00`; `5.2lts/stable` version **5.2.3**, released `2026-10-22T08:50:33.847431+00:00`; and `latest/candidate` version **5.2.3**, released `2026-10-22T08:55:21.365037+00:00`. The default track's stable version was therefore older than another track's stable version.

**Not covered:** when channels will next change, or whether the named archives can be fetched. This reading concerns the listing only. *Added 2026-10-27; no auditor has seen it.*

## `apt-get(8)`, `dpkg(1)` and `apt(8)`, for R1 item 5's removal and its reading

`https://manpages.ubuntu.com/manpages/noble/man8/apt-get.8.html`, read **2026-11-03**, apt 2.8.3. `-s`: *"No action; perform a simulation of events that would occur based on the current system state but do not actually change the system."* *"Simulated runs print out a series of lines, each representing a dpkg operation: configure (Conf), remove (Remv) or unpack (Inst)."* On `install`: *"If a hyphen is appended to the package name (with no intervening space), the identified package will be removed if it is installed."* `apt-get returns zero on normal operation, decimal 100 on error.` Label: **documented**.

`https://manpages.ubuntu.com/manpages/noble/man1/dpkg.1.html`, read **2026-11-03**. `--force` thing `overwrite`: *"Overwrite one package's file with another's file."* Label: **documented**.

`https://manpages.ubuntu.com/manpages/noble/man8/apt.8.html`, read **2026-11-03**: its page *"only mentions the most used commands and options"* and does not list `-s`. So that `apt` takes `-s` is an **assumption**, labelled where it is used. That it takes `-y` and two `-o` on a glob removal is **observed**, the Sesterce pass's hand repair of 2026-11-02.

**Not covered:** whether a real removal on any other image meets the same file, and what apt prints for a name ending in a hyphen when no such package exists. *Added 2026-11-03; no auditor has seen it.*
