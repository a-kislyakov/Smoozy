# The host key — obtained without authenticating, and whose it turns out to be

What the German rental and the Creator's own Mac answered on 2026-08-17, in the sitting that fetched that box's day record. An **input** to the stage, like the others: nothing here is a decision or a plan.

Every claim is either **observed**, with what was run and what came back, or **documented**, with the exact words and the address. Where something was not established, it says so. Rule 1's plain-English rule is suspended here, per the stage's exception.

**Why it exists.** Facts 82 and 83 were filed out of `Security and licence` on 2026-08-16 and both ride free on any rental. Fact 82 is load-bearing: Fact 37 established that **no OpenSSH manual documents a single one of the diagnostic strings the tools emit**, so if a host key cannot be obtained and compared before connecting, the only routes left are parsing text a version change can break silently, or accepting whatever key is offered, connecting anyway, and warning the user after the fact. **Corrected 2026-08-21: this route was described as *authenticating to an impostor* or *proving Smoozy's identity to an impostor*, and that cannot happen.** SSH checks the host key inside key exchange, before authentication is even asked for — RFC 4253 §8 step 3, read 2026-08-20: *"C verifies that K_S really is the host key for S (e.g., using certificates or a local database)."*, and §10 makes `ssh-userauth` a service requested *"After the key exchange"*; RFC 4252's abstract puts the authentication protocol *"on top of the SSH transport layer protocol"*. **Run with a control on the Creator's Mac, `OpenSSH_10.2p1`, 2026-08-20:** with a deliberately wrong stored key the client exited 255 at *"Host key verification failed."* and the whole trace contains no `userauth`, no `SERVICE_ACCEPT`, no key offered; with the true key in the same scratch file the trace reaches *"debug2: service_accept: ssh-userauth"*. **No credential can go first.** **What the bad route actually is** — accept whatever key is offered, connect anyway, and warn the user after the fact, which is a weaker and different harm. **So Fact 82 is not load-bearing in the way this paragraph claimed** — that word comes down a notch. The route the project chose is unaffected. Fact 83 asks whose key it is, at a provider that fronts SSH with its own address.

**The sitting ran in two halves and the join matters.** In the first, the Vast API was unreachable to the agent doing the work, so nothing could be rented or destroyed and the work was confined to a box already running plus the Creator's own Mac. That was enough to answer Fact 82 and to answer Fact 83's proxy half by a comparison the fact had not proposed — which then opened a sharper question. **In the second, the Creator supplied the keys folder**, and the rest was posed properly: the German box destroyed, a fresh rental on the same physical machine, and a genuine virtual machine. **All four facts are now answered.** The order is kept here because two of the answers read differently once you know a second rental settled them.

**Five rentals were taken and all five destroyed**, the account verified empty afterwards — no instances, no volumes. **The whole batch cost under five cents.** Two of the five were lost to provider faults recorded at the end.

---

## The machines

**The box that was already running.** Vast.ai instance `47868453`, reached through `ssh3.vast.ai` port `28452` as root, in Germany, at about `$0.031/hr` — the box left running since 2026-08-16 13:27 UTC to reach the *over a day* halves of Facts 62 and 65. Ubuntu 22.04.5 LTS, kernel `7.0.0-29-generic`, **PID 1 is `sh`** and `systemd-detect-virt` reports **`docker`** — a container rental. Its sshd announces itself as **`SSH-2.0-OpenSSH_8.9p1 Ubuntu-3ubuntu0.16`**. Hostname `a1a84bc7fa93`. Capability set `CapPrm`, `CapEff` and `CapBnd` all **`00000000a80405fb`**, which does not include `CAP_SYS_ADMIN`. `pid_max` is **4194304**.

**The second rental on the same machine.** Vast.ai instance `47947488`, offer `46997840`, **machine `141036` again** — the same physical machine — at `$0.028/hr`, alive 13:44–13:57 UTC. Reached at **`ssh9.vast.ai` port `27488`**. Hostname `149e28227de9`. Also a container, same kernel `7.0.0-29-generic`, same capability set.

**The virtual machine.** Vast.ai instance `47947626`, offer `36959937`, machine `21141`, in Quebec, at `$0.068/hr`, alive 13:47–13:57 UTC. Rented from the provider's own **Ubuntu 22.04 VM** template, whose image resolved to `docker.io/vastai/kvm:cuda-12.9.1-auto`. `systemd-detect-virt` reports **`kvm`**, PID 1 is **`systemd`**, kernel `6.8.0-59-generic`, capability set **`000001ffffffffff`** — every capability — and `/proc` mounted read-write. Reached at **`184.144.255.144` port `32961`**, its direct mapping, because its proxy port never forwarded.

**A third container, for one reading only.** Instance `47947078`, machine `55691`, in Quebec, alive about five minutes: its host keys are the second data point on where keys come from.

**The caller.** The Creator's own Mac, macOS 26.5, **`OpenSSH_10.2p1, LibreSSL 3.3.6`**. All scanning and comparing was done from here, across an ordinary home line, between 13:01 and 13:57 UTC on 2026-08-17.

---

## Fact 82 — the key can be obtained and compared with no connection that proves anything to anybody

### What the documentation sanctions

**`ssh-keyscan(1)`, macOS 26.5 man page dated October 4, 2025, read 2026-08-17.** The mechanism's own words, on needing no credential:

> For scanning, one does not need login access to the machines that are being scanned, nor does the scanning process involve any encryption.

The same page states the positive use outright, which is the one Smoozy needs:

> On the other hand, if the security model allows such a risk, ssh-keyscan can help in the detection of tampered keyfiles or man in the middle attacks which have begun after the ssh_known_hosts file was created.

And its own **EXAMPLES** section documents the comparison as a text operation on the scanned output rather than as anything ssh does:

> Find all hosts from the file ssh_hosts which have new or different keys from those in the sorted file ssh_known_hosts:
>
> `$ ssh-keyscan -t rsa,ecdsa,ed25519 -f ssh_hosts | sort -u - ssh_known_hosts | diff ssh_known_hosts -`

`ssh-keygen(1)`, same system, documents the lookup half:

> **-F** hostname | [hostname]:port — Search for the specified hostname (with optional port number) in a known_hosts file, listing any occurrences found.

### What was observed

**A scan returns every key type with no credential offered, and nothing on stderr.** `ssh-keyscan -p 28452 ssh3.vast.ai` exited **0**, wrote nothing to stderr, and returned three keys — `ssh-rsa`, `ecdsa-sha2-nistp256` and `ssh-ed25519` — each preceded by a banner comment naming the server version. No identity file was supplied and none was asked for.

Their fingerprints, as `ssh-keygen -lf` reported them:

```
3072 SHA256:GUlE6238zSGM7SIdXqmJBkkpBqPL1d8vHqtR+4CAOSo [ssh3.vast.ai]:28452 (RSA)
256 SHA256:/JYhPs3y83tBMq2vqPdMFfntV+XItblLH3oOkcTo59Y [ssh3.vast.ai]:28452 (ECDSA)
256 SHA256:mO080LZj5p4sF1ay8FN0vhK8UXXBCob7NZ4mS2fY+SE [ssh3.vast.ai]:28452 (ED25519)
```

**A single type can be asked for.** `-t ssh-ed25519` returned that one key alone, which is what lets a comparison be made against one remembered value rather than against a set.

**Two independent scans, minutes apart, returned byte-identical key material.** So the scan is a reading of something stable, not a per-connection value.

**The comparison separates a real key from a false one without ssh.** A throwaway ed25519 keypair was generated and pinned as though it were the host's; comparing the third field of the scanned line against the impostor's key material **did not match**, while comparing it against the stored good record **did**. That is the whole check, and it is a string comparison on the output of a program that never authenticated.

**`ssh-keygen -F` answers the stored-record half by exit status.** Present: exit **0**, with the matching lines printed. Absent — the same host at a port never recorded: exit **1** and no output.

**Nothing there is a diagnostic string.** The comparison is on key material, and the decision is the caller's own.

### What a mismatch yields, and the limit that matters

**Where a connection *is* attempted against a wrong stored record, ssh refuses and says so at length.** With `StrictHostKeyChecking=yes`, `BatchMode=yes` and a `UserKnownHostsFile` holding the impostor key, the attempt exited **255** and wrote thirteen lines to stderr, verbatim:

```
@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@
@    WARNING: REMOTE HOST IDENTIFICATION HAS CHANGED!     @
@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@
IT IS POSSIBLE THAT SOMEONE IS DOING SOMETHING NASTY!
Someone could be eavesdropping on you right now (man-in-the-middle attack)!
It is also possible that a host key has just been changed.
The fingerprint for the ED25519 key sent by the remote host is
SHA256:mO080LZj5p4sF1ay8FN0vhK8UXXBCob7NZ4mS2fY+SE.
Please contact your system administrator.
Add correct host key in <path> to get rid of this message.
Offending ED25519 key in <path>:1
Host key for [ssh3.vast.ai]:28452 has changed and you have requested strict checking.
Host key verification failed.
```

**Two limits on that, and both are Fact 37's shape again.**

**`255` is ssh's generic failure status and distinguishes nothing.** No exit status separates a host-key mismatch from a refused credential, an unreachable address or a timeout. Only the text above does, and Fact 37 established that no OpenSSH manual documents these strings — so a text match on them is a version change away from breaking silently.

**`ssh-keyscan(1)` has no EXIT STATUS section at all.** The `0` on a successful scan and the `1` on a port with no server were both observed, on this version, on this system; the manual states neither. A negative result would have been a scan that exited 0 with no keys, making success and failure indistinguishable — that is not what happened, but nothing documents that it will not.

### What this settles for the plan

**Neither of the two bad routes Fact 82 feared is forced.** Smoozy obtains the offered key with `ssh-keyscan`, compares the key material against what it remembers itself, and decides — **before** any connection that would present Smoozy's own credential to whatever is at the far end. Nothing is parsed, and no authentication precedes the warning. Where the comparison fails, the user is warned on Smoozy's own words rather than on OpenSSH's.

**What is not established.** The scan was posed against one server on one provider from one caller. Whether a scan can be defeated — a middle that answers the scan honestly and the session dishonestly, or the reverse — was not posed and is not answerable from a single vantage point. And the exit statuses above are observed rather than documented.

---

## Fact 83 — the key is the machine's, and that is not the end of it

### The comparison that replaced two rentals

**The three keys offered through the proxy are byte-for-byte the machine's own host-key files.** Read on the box itself, `/etc/ssh/ssh_host_rsa_key.pub`, `/etc/ssh/ssh_host_ecdsa_key.pub` and `/etc/ssh/ssh_host_ed25519_key.pub` hold exactly the key material the scan returned from `ssh3.vast.ai:28452`, and `ssh-keygen -lf` on the box gives the same three fingerprints listed above.

**So the proxy forwards the connection rather than terminating it.** On this rental, at this provider, the remembered host key belongs to the rented machine and not to `sshN.vast.ai`. The half of Fact 83 that worried about a proxy substituting its own identity is answered **no**.

**One honest limit on that.** The machine's own files were read over an authenticated SSH session through the same proxy. A middle performing a full man-in-the-middle on both the scan and the session would present a consistent lie, and nothing observed rules that out. What is ruled out is the ordinary case the fact was filed against — a proxy that terminates SSH with a key of its own, which would have shown up as a mismatch between the scan and the files.

### The sharper question this opened, which nobody had asked

**The keys were not made for this rental, and three readings say so.**

- All three carry the comment **`root@f077078e58b1`**, while this box's hostname is **`a1a84bc7fa93`**.
- All six key files are dated **2026-08-04 08:14:13 UTC**.
- The rental itself started **2026-08-16 13:27:02 UTC** (`/proc/1` mtime), and the physical host booted **2026-08-06 16:06:27 UTC** (`uptime -s`).

**So the host key predates the rental by twelve days and the host's own boot by two.** Nothing in this rental generated it. `/etc/ssh` holds no regeneration hook, and the key files were never rewritten — their modification and change times are both that 2026-08-04 stamp.

**Why that matters more than the proxy question did.** Row 36's promise is that a machine is recognised again, and that a **changed** identity warns the user. If the same key material comes back on a different rental — because it is baked into an image, or generated once per physical machine at provisioning — then:

- **The warning never fires when the machine is genuinely replaced**, which is the case it exists for. The board already records that a container rental's **recycle** destroys everything Smoozy installed and hands back a provider default on the same address and port, with nothing in the connection to reveal it. A host key dated before the rental cannot reveal it either.
- **And it may fire when nothing bad happened**, if a later rental lands on different hardware or a different image while the user believes they are returning to their server.

**It was then settled the same day, and the answer is the negative one.** The Creator supplied provider access; the German box was destroyed and **a fresh rental taken on the same physical machine, `141036`**, from the same offer `46997840`.

### The negative result, in full

**The new rental is beyond doubt a different container.** Hostname **`149e28227de9`** against the old box's `a1a84bc7fa93`. Container start **2026-08-17 13:44:45 UTC** against 2026-08-16 13:27:02. **`/root/day.jsonl` — the twenty-four-hour record the old rental had been writing — is gone**, along with everything else Smoozy had put there. The physical host had not rebooted between the two (`uptime -s` is 2026-08-06 16:06:27 in both).

**It reaches a different address on a different port.** The old box was `ssh3.vast.ai:28452`; the new one is **`ssh9.vast.ai:27488`**. Same machine, different proxy, different port.

**And it presents byte-identical host keys.** All three fingerprints match the destroyed box exactly:

```
3072 SHA256:GUlE6238zSGM7SIdXqmJBkkpBqPL1d8vHqtR+4CAOSo  (RSA)
256  SHA256:/JYhPs3y83tBMq2vqPdMFfntV+XItblLH3oOkcTo59Y  (ECDSA)
256  SHA256:mO080LZj5p4sF1ay8FN0vhK8UXXBCob7NZ4mS2fY+SE  (ED25519)
```

Same comment `root@f077078e58b1`, same file date **2026-08-04 08:14:13 UTC**. **The host key survived the rental it belonged to.**

**A second machine confirms the mechanism.** A container rented on machine `55691` carried host keys commented **`root@buildkitsandbox`** and dated **2026-08-07 12:36:32 UTC** — a container-image build artefact, and different key material again. **The mechanism this inferred is wrong and Test 19 disproved it on 2026-08-22 — corrected here the same day, four sections above where the correction already stood.** It read *So the keys are baked when an image is built, and a rental inherits whatever build its machine holds*, which predicts one image giving one set of keys. **A container's keys are the host's**: the same offer re-rented gave three identical keys with the proxy port moved, and a different machine on the same image gave three different ones. The `root@buildkitsandbox` comment is a build artefact of the image the *host* was built from and says nothing about which host serves a rental. See §17 below. Nothing generates a key for a rental, ever — **on the two providers observed. Scope added 2026-08-22: the sentence stood unqualified, and that is the scope it has always had in practice.**

### What this does to Row 36

**The changed-identity warning cannot fire on the event it exists for.** A user whose rental is gone and who rents again on the same hardware gets **the same host key** and no warning — while everything Smoozy installed has been destroyed. That is the same blind spot the container recycle already has, and this is a second, independent door into it.

**It also fires when nothing is wrong.** Re-renting on different hardware changes the key, so a user deliberately taking a new server meets a warning about identity having changed.

**So the harm Fact 83 named is real and arrives by neither of the routes it guessed.** The proxy is honest; the key is the machine's. The key is simply **not a name for the thing Smoozy needs to recognise** — not the rental, and not the installation. **What can tell the two apart is the installation identity** that [State, identity and recovery](State,%20identity%20and%20recovery.md) already introduced for the recycle case, and this observation is a second argument for it.

**One thing is not established:** whether a machine's keys ever change without the hardware changing — a host reprovisioning or rebuilding its image would presumably do it, and nothing here observed that.

**The proxy address is not a stable name either**, which nothing had asked. `ssh3` became `ssh9`, and the port changed, for the same machine across two rentals hours apart.

---

## Fact 84 — the mechanism is peer credentials, and it survives the container

**The endpoint.** An `AF_UNIX` stream socket at `/run/smoozy-fact84/endpoint`, held by a process running as **root**, deliberately left mode **`0666`** so the test was about credentials rather than about file permissions. Fact 84's first layer — the endpoint and its directory owned by the supervisor's login and permitting that login alone — was already observed where Fact 1's narrow clause was posed and is not re-posed here.

**The caller.** An ordinary login `f84user` created for the test, `uid=1000(f84user) gid=1000(f84user)`, running a small client under `runuser`.

**What the kernel told the server.** On the accepted connection, `getsockopt(SOL_SOCKET, SO_PEERCRED)` returned:

```
pid=1628 uid=1000 gid=1000
```

and `uid 1000` resolved to login **`f84user`** — the caller's own login, correctly named. The caller's independent view of itself agreed exactly: `client: pid=1628 uid=1000 gid=1000`.

**It survives the container's reduced capability set.** The container holds `CapPrm`/`CapEff`/`CapBnd` `00000000a80405fb`, which does not include `CAP_SYS_ADMIN`, and the mechanism worked unchanged. **So Fact 84's second half is answered for a container rental.**

**One thing fell out of it that belongs to Fact 91.** By the time the server looked up the pid the kernel had just reported, the caller had exited and **`/proc/1628` was already gone** — `head` and `grep` on `/proc/1628/status` both returned `No such file or directory`, exit status 2. The reported pid is a stale handle the instant the caller finishes, which is exactly the hazard Fact 91 exists to close.

### The virtual-machine half — identical

**Posed on a genuine virtual machine the same day**, once provider access arrived: a Vast KVM rental where `systemd-detect-virt` reports **`kvm`**, PID 1 is **`systemd`**, the capability set is the full **`000001ffffffffff`**, and `/proc` is mounted read-write. The same test, same script: the server was told

```
pid=1643 uid=1002 gid=1002
```

for a caller running as `f84user` (uid 1002), and the caller's own view agreed exactly. **So the mechanism behaves identically on both machine kinds, and Fact 84's paired requirement is met.** The stale-identifier observation repeated too — `/proc/1643` was gone by the time the server looked.

### The forgery half — the endpoint may trust what it is told, as well as what it asks

**Posed on both machine kinds**, and the answer is the same on each. An unprivileged caller (`f84user`) attached `SCM_CREDENTIALS` to a datagram and tried three claims:

| what was claimed | what the kernel did |
|---|---|
| `uid 0`, own identifier | **refused — `errno 1 Operation not permitted`** |
| own uid, own identifier | accepted |
| own uid, **identifier 1** | **refused — `errno 1 Operation not permitted`** |
| nothing attached at all | sent |

**And what the receiver actually got, in every case that arrived, was the truth**: `pid=1758 uid=1002 (f84user) gid=1002` on the virtual machine, `pid=501 uid=1000 (f84user) gid=1000` on the container — including the datagram that attached no claim, because the kernel fills the credentials in itself.

**A `uid 0` claim reaching the receiver as `uid 0` would have been the negative result.** It was refused outright. So a caller can attach only its own true identity, and attaching nothing yields the same truth — **the endpoint cannot be lied to about who is calling**, on either machine kind.

---

## Fact 91 — answered: the pair is the identifier and the kernel-set start time

**The mechanism.** A group's identity recorded together with **the start time of its leader, field 22 of `/proc/<identifier>/stat`** — written by the kernel at fork, in clock ticks since boot. A supervisor validates by reading that field again at the recorded identifier and comparing.

**It cannot be chosen by the process it describes.** `/proc/self/stat` is mode `-r--r--r--` and a write to it was refused with **`errno 22 Invalid argument`** on both machine kinds. There is no route by which a process presents a start time of its choosing.

### The reuse case, forced on the virtual machine

**A container cannot pose this and that is itself worth recording.** On the container rental, `pid_max` is **4194304**, so waiting for the identifier space to wrap is not a test anybody can run in a sitting; writing `/proc/sys/kernel/ns_last_pid` was refused with **`errno 30 Read-only file system`**, because `/proc/sys` is mounted read-only there; and making a namespace of one's own to do it inside failed with **`unshare: unshare failed: Operation not permitted`**. Three routes, three refusals.

**On the virtual machine it worked, and the identifier was genuinely re-handed-out.** A group leader was recorded as the pair **(identifier 1493, start time 19186)** and killed; validating after it ended gave `exists=False`. Then `/proc/sys/kernel/ns_last_pid` was written, and **the next process created received identifier 1493 — the very same one**. Validating the recorded pair against that new, unrelated process:

```
exists=True   matches=False
```

**So the pair separates them.** The identifier is present again and the recorded start time does not match it, which is exactly the discrimination Fact 91 asked for. **A later, unrelated process validating against the recorded pair would have been the negative result**, and it did not happen.

### What this settles, and the one thing it does not

**`The shape`'s prohibition is completed rather than contradicted.** That document forbids finding or stopping a worker by executable name, command-line pattern or process search, on Batch A's observation that a pattern selected more than one process. This supplies the missing half: a recorded identifier alone **is** a pattern with extra steps, because identifiers are re-handed-out; the identifier **paired with the kernel-set start time** is not, because the later holder cannot present it and cannot forge it.

**What is not established.** The pairing was demonstrated across a forced reuse **within one boot**. Start time is measured in ticks since boot, so it is meaningful only against the same boot — which is why the run identifier `State, identity and recovery` already records at start is what settles the reboot, container-restart and recycle cases, and this mechanism is not asked to. That division was the document's before this test and is unchanged by it.

---

## Two provider faults, recorded because both cost a rental

**A rental can report `success, running` while being unreachable, and nothing in the report says so.** Instance `47947049`, on machine `141036`, came up with `actual_status: running` and `status_msg: "success, running ubuntu_22.04/ssh"`. Its proxy port **accepted a connection and immediately closed it** — `write (ssh7.vast.ai): Broken pipe` from `ssh-keyscan`, `Connection refused` from `ssh` moments earlier. Its instance record offered no usable direct mapping (`direct_port_start 65535`, `direct_port_end -1`), and the machine's own `machine_dir_ssh_port` refused the login. Forty polls over ten minutes never opened it. It was destroyed and re-rented from the same offer, which worked. **That is the fourth rental this project has lost to a container that never became reachable**, and it is why a rental is checked rather than assumed.

**Asking for a virtual machine by adding a flag is silently ignored.** The create call was sent with `"vm": true` on an offer advertising `vms_enabled`; it returned `{"success": true}` and delivered **a container** — `systemd-detect-virt: docker`, the ordinary reduced capability set. **This is the same shape as the contract cap** that `Two more boxes` records: the provider accepts a parameter, reports success, and does nothing with it.

**What a virtual machine actually needs.** The provider's own **template**, by its hash. Its image tag is the placeholder `@vastai-automatic-tag`, which the provider resolves; **supplying the image by hand instead defeats it** and the instance dies with `Error response from daemon: manifest for vastai/kvm:latest not found`. And `runtype` and `use_ssh` may not both be sent — `{"success":false,"error":"invalid_args","msg":"runtype and use_ssh cannot both be set"}`.

**A virtual machine also takes minutes rather than seconds**, and its guest boots inside the rental: cloud-init finished 12.4 seconds after the guest started, but the guest only started some minutes after the instance reported running, and **its proxy port never forwarded at all** — the direct mapping in the instance record's `ports` field was the only way in.

**One more thing, unasked and relevant to Fact 83. CORRECTED 2026-08-22 — this paragraph drew a conclusion its evidence does not support, and the opposite is observed.**

**What it used to say:** *"The virtual machine **generated its own host keys at first boot**, commented `root@ubuntu`, and cloud-init printed their fingerprints to the instance log. So the baked-in-at-image-build behaviour that breaks Row 36 is a property of **container** rentals, not of rented machines in general. Whether a virtual machine's keys survive its own recycle was not posed."*

**What was actually observed here** is that cloud-init printed the fingerprints to the instance log. **Cloud-init prints the fingerprints of whatever host keys are present**, and it prints them whether it generated them or found them. **Nothing here read the key files' timestamps, and nothing here compared the keys against another rental of the same image.** So *generated at first boot* was an inference from a log line, not a reading.

**[The last two boxes](The%20last%20two%20boxes%20—%20the%20hook%20that%20cannot%20be%20read%20back,%20the%20full%20disk,%20and%20the%20unit%20that%20kills%20a%20render.md) §17 posed it properly and the answer is the reverse.** That box offered four host keys, every one commented `root@ubuntu`, with **key files all dated 2025-11-06 20:55:50 to the nanosecond** — months before the rental existed — and after a recycle that removed both logins, Blender, the throwaway and the lingering flag, **every fingerprint, every file size and every timestamp was identical**. Its own words: *"this provider's image does not generate host keys at first boot. It ships them."*

**So the corrected finding is: the ship-them-in-the-image behaviour is not a property of container rentals as against machine rentals.** It was observed on this provider's machine image. **What is a property of the container kind, established separately on 2026-08-22, is something else again** — [The multi-provider batch](The%20multi-provider%20batch%20%E2%80%94%20the%20surface%20that%20did%20not%20vary,%20the%20Debian%20route%20that%20exists,%20and%20the%20cards%20a%20slice%20cannot%20see.md) Test 19 established that a container's keys are neither the image's nor the proxy's but **the host's**: the same offer re-rented gave three identical keys, and a different machine on the same image gave three different ones.

**What this leaves genuinely unposed** is a virtual machine whose keys were read at rental and compared against a second rental of the same image on different hardware — the test Test 19 ran for containers and nobody has run for machines.
