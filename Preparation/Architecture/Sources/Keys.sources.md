# Sources — Keys

*Where every outside-world claim in [Keys](../Keys.md) came from. The document carries each claim's label in place; this file carries the citation behind it — the pointer to the input document that owns it, or the publication's exact words with its address and the date read, or the plain record that no source exists — and what each one does **not** cover. **Decisions Smoozy made carry no entry here.** Entries run in the document's own order, Section 1 to Section 15.*

Rule 1's plain-English rule is suspended here, per the Architecture stage's exception.

---

## What no entry below repeats

Four limits apply to everything in this file and are not restated under each entry.

- **No Windows machine has run it, and none is rented today.** Every Windows citation is documentation alone. Nothing drawn from one has been run on a real installation, and nothing below is labelled observed on Windows.
- **No Linux computer exists on this project either.** Every Linux claim is documentation alone, from the packaging and the desktop-project publications named where they are used.
- **Every observation is one build on one machine.** The Creator's own Mac — macOS 26.5.2 (build 25F84), arm64, `/usr/bin/ssh-keygen`, the system's own build, reporting `OpenSSH_10.2p1, LibreSSL 3.3.6` (`ssh -V`). **Two entries below are on a later build of the same machine and say so in place** — see the second machine record.
- **`man.openbsd.org` serves OpenBSD-current**, which runs ahead of shipping systems. No manual-page citation supports a claim about the OpenSSH any user actually has — which is the ground of [T2](../Keys.md#2-the-supported-types-the-offered-set-and-the-default-row-241)'s runtime reading and of [G6](../Keys.md#63-the-generator-probed-never-assumed-row-248)'s probe.

## The machine every observation made for this document was made on

The Creator's own Mac, as above. The throwaway runs written up below were made on **2026-08-20**, inside a temporary folder under the session's scratchpad area; nothing was written outside it, nothing pre-existing was touched (`~/.ssh` was read once, by directory listing only, to confirm the version probe had written nothing), and the folder was deleted when the runs ended. **Each entry says what a negative result would have looked like.**

**The second machine record — the same Mac, a later build, 2026-08-23.** Two entries below (the `SSH_ASKPASS_REQUIRE` matrix under Section 1, and the PEM/PKCS8 entry under Section 4) were **run for the first time on 2026-08-23**, because the runs the document described had left no record anywhere in `Preparation/`. **The machine had moved on since 2026-08-20 and the difference is stated rather than smoothed:** the Creator's own Mac, now **macOS 26.6.2 (build 25G83)**, arm64, `/usr/bin/ssh-keygen`, the system's own build, reporting **`OpenSSH_10.3p1, LibreSSL 3.3.6`** (`ssh -V`) — **a later OpenSSH than the `OpenSSH_10.2p1` every other observation in this file was taken on.** Both runs were driven from a short script in the session's scratchpad area, each invocation in **its own session with no controlling terminal** (`/dev/tty` unopenable from the child, checked before the runs) and **standard input on `/dev/null`**; every key involved was made inside that folder, nothing outside it was written, `~/.ssh` was not read or touched at all, and the folder was deleted when the runs ended. **These two entries are new records, not recovered ones**, and neither reproduces a transcript that ever existed in the project.

**The third machine record — the same Mac, the same later build, 2026-09-02.** Two entries under Section 2 (the **generation timing** behind [T2b](../Keys.md#2-the-supported-types-the-offered-set-and-the-default-row-241) and the **`ssh -Q key-plain` output** behind its three grounds) were run on **2026-09-02**, for the same reason the second machine record exists: the figures T2b carried had **no date, no machine, no command and no label anywhere**, and the sitting that wrote them recorded that nothing was run. **The machine:** the Creator's own Mac, **macOS 26.6.2 (build 25G83)**, arm64, `/usr/bin/ssh-keygen` and `/usr/bin/ssh`, the system's own builds, reporting **`OpenSSH_10.3p1, LibreSSL 3.3.6`** (`ssh -V`) — the same build as the second machine record and a later one than the rest of this file. Every key made was written into the session's own scratchpad folder under `-C smoozy-probe`, `~/.ssh` was neither read nor touched, nothing outside that folder was written, nothing was installed and nothing was rented. **One departure from the second record is stated rather than smoothed:** the three throwaway pairs were made and the deletion of the folder was **refused by the permission system**, so they remain in the session scratchpad — outside `Preparation/`, outside the user's own key locations, and named to the runner. **These are new records, not recovered ones.**

**The input documents this file points to** are [The manuals — keys, secrets, and what cannot be told apart](../The%20manuals%20—%20keys,%20secrets,%20and%20what%20cannot%20be%20told%20apart.md) (Facts 32 to 39 and 76), [The computer itself — its name, its identity, and its clock](../The%20computer%20itself%20—%20its%20name,%20its%20identity,%20and%20its%20clock.md) (Fact 12) and [Six machines — the container, the webhook and the ready action](../Six%20machines%20—%20the%20container,%20the%20webhook%20and%20the%20ready%20action.md) (Fact 60). Where the project holds an address for a publication those documents quote, it is repeated here so this file can be checked without opening theirs.

---

# Section 1 — what binds this before anything is decided

## §1 — what is readable from an encrypted private key without the passphrase (the private half's rule, inherited)

**Pointer.** [The manuals](../The%20manuals%20—%20keys,%20secrets,%20and%20what%20cannot%20be%20told%20apart.md), **Fact 35 Half A**, documented and observed: type, bit size and fingerprint are readable from an encrypted private key file with no passphrase and **stay readable with the `.pub` deleted**; the **comment is not recoverable** without it — `locked@local` while the `.pub` existed, `no comment` once it was moved aside.

**Does not cover:** where in the file the comment sits. That fact's own narrowing: the observation shows the comment is *not recoverable*, not *where it is stored*, and `ssh-keygen(1)`'s DESCRIPTION places it *"within the private key file itself"* and no further. It does not cover the legacy PEM format, which was not tested there — that is [U8](../Keys.md#44-the-protection-reading-and-the-availability-answer-row-245)'s entry below. Two keys, one build.

## §1 — a successful fingerprint is not proof of a key

**Pointer.** [The manuals](../The%20manuals%20—%20keys,%20secrets,%20and%20what%20cannot%20be%20told%20apart.md), **Fact 34**, observed: `ssh-keygen -l -f` rejects `config`, plain text and empty files at exit 255 with *"… is not a public key file."*, but **accepts `known_hosts` at exit 0 and fingerprints it**, because a `known_hosts` line differs from a public key line only by a leading hostname field. Every exit status and message reproduced on re-run.

**Does not cover:** `authorized_keys`, which that fact asserted **by analogy and never ran**. It was run for this document — see the observed `authorized_keys` entry under Section 4 below.

## §1 — no secret is ever a process argument or an environment value

**Pointer.** [Six machines](../Six%20machines%20—%20the%20container,%20the%20webhook%20and%20the%20ready%20action.md), **Fact 60**, observed on a rental: from a second ordinary login, another login's process **"is visible with its owner and command line"**.

**Does not cover:** the user's own computer, where no comparable reading was taken. `Security and licence` §13 rules 1 and 2 apply the same rule in both places, and nothing in this document waits on the second reading.

## §1 — the askpass route and the requirement variable, as `ssh(1)` documents them

**Direct publication, held by an input document and repeated here with its address.** `ssh(1)`, OpenSSH's own manual at **`https://man.openbsd.org/ssh.1`**, footer as served *OpenBSD-current, August 4, 2026*, read **2026-08-16** ([The manuals](../The%20manuals%20—%20keys,%20secrets,%20and%20what%20cannot%20be%20told%20apart.md), Fact 35 Half B, and that document's own [companion](The%20manuals%20—%20keys,%20secrets,%20and%20what%20cannot%20be%20told%20apart.sources.md) for the address). ENVIRONMENT, `SSH_ASKPASS`:

> "If ssh needs a passphrase, it will read the passphrase from the current terminal if it was run from a terminal. If ssh does not have a terminal associated with it but DISPLAY and SSH_ASKPASS are set, it will execute the program specified by SSH_ASKPASS and open an X11 window to read the passphrase."

ENVIRONMENT, `SSH_ASKPASS_REQUIRE`:

> "Allows further control over the use of an askpass program. If this variable is set to “never” then ssh will never attempt to use one. If it is set to “prefer”, then ssh will prefer to use the askpass program instead of the TTY when requesting passwords. Finally, if the variable is set to “force”, then the askpass program will be used for all passphrase input regardless of whether DISPLAY is set."

**Does not cover, and this is the whole of Section 1's caution:** the page is `ssh(1)`. **It says nothing about `ssh-keygen`**, which is the tool every promptless read in this document invokes — see the `ssh-keygen(1)` entry immediately below. The same fact records that the *exclusivity* of the two input routes is **inferred, not documented**: the manual states the terminal route's condition and describes the askpass route separately, and never says those two are the only ways a passphrase can be supplied, nor what the tool does when both are closed.

## §1 — `ssh-keygen(1)` documents no askpass behaviour at all

**Direct publication.** `ssh-keygen(1)`, OpenSSH's own manual. **Address:** `https://man.openbsd.org/ssh-keygen.1` — **read 2026-08-20.** The page serves OpenBSD-current and runs ahead of shipping systems (Fact 32's standing caution; the type table below shows it doing so).

The **ENVIRONMENT section holds a single entry**, `SSH_SK_PROVIDER`: *"Specifies a path to a library that will be used when loading any FIDO authenticator-hosted keys, overriding the default of using the built-in USB HID support."* **Neither `SSH_ASKPASS` nor `SSH_ASKPASS_REQUIRE` appears anywhere on the page.** On prompting, the DESCRIPTION says only: *"The program also asks for a passphrase."*

**Does not cover:** what the tool actually does with either variable, in either direction. The absence is an absence of documentation and not a documented absence — which is why both directions are carried as observations below, and why [Section 13](../Keys.md#13-unresolved) item 3 stays open on the documentation gap after the behaviour was run.

## §1 — the disabling direction: `SSH_ASKPASS_REQUIRE=never` against `ssh-keygen`

**Observed — and this is a NEW record, run on 2026-08-23, not a recovered one.** The four-way run the document described from 2026-08-20 had **no transcript, table or record anywhere under `Preparation/`**: not in this file, not in [Facts](../Facts.md) — whose **Fact 115** records only the `force` **generation** run, not this `-y` matrix — and not in [The manuals](../The%20manuals%20—%20keys,%20secrets,%20and%20what%20cannot%20be%20told%20apart.md), whose Fact 35 Half B material is about `ssh(1)` and a real server rather than about `ssh-keygen`. **Nothing was invented to fill it. It was run again**, on the machine described in the second machine record above — the Creator's Mac, **macOS 26.6.2 (build 25G83)**, **`OpenSSH_10.3p1`**, a later build than the rest of this file's observations.

**What was run.** One encrypted `ed25519` key (`ssh-keygen -t ed25519 -C matrix -N 'correct horse'`) and a two-line askpass helper that **appends a line to a log file every time it is invoked** and prints the correct passphrase. Then `ssh-keygen -y -f <key>` five times, each invocation in **its own session with no controlling terminal** and **standard input on `/dev/null`**, with `SSH_ASKPASS` always naming the helper and only the two variables under test moving. The child's terminal state was checked before the runs: opening `/dev/tty` from a child started the same way failed with *"Device not configured"*.

| # | `DISPLAY` | `SSH_ASKPASS_REQUIRE` | exit | askpass calls | what came back |
|---|---|---|---|---|---|
| 1 | `:0` | *unset* | **0** | **1** | public key printed; log line `CALLED: Enter passphrase for "<path>": ` |
| 2 | `:0` | `never` | **255** | **0** | stderr `Enter passphrase for "<path>": Load key "<path>": incorrect passphrase supplied to decrypt private key` |
| 3 | `:0` | `prefer` | **0** | **1** | public key printed; same log line |
| 4 | *unset* | `force` | **0** | **1** | public key printed; same log line |
| 5 | *unset* | *unset* | **255** | **0** | same stderr as case 2 |

**Cases 1 and 2 differ in one variable and each is the other's control.** No case hung — each run was under a twenty-second kill timer that never fired, and every exit status above is the tool's own.

**One thing the document did not have, recorded rather than smoothed.** `never` closes the **helper** and not the terminal route: in cases 2 and 5 the tool still wrote its prompt string to **standard error** and then read standard input, and because that input was `/dev/null` it took the empty read as an **empty passphrase** and ended at `incorrect passphrase supplied to decrypt private key` rather than at a distinct *no passphrase available*. **The refusal is still an exit status and still promptless in the only sense the construction needs** — nothing waits, nothing is put to a user, and [U6](../Keys.md#42-usable-composed-row-245) parses no diagnostic string — but the exit-255 text on a refused read is the same text a wrong passphrase gives, which is why nothing is ever read from it.

**A negative result would have been:** the helper logging a call in case 2 or case 5 (the requirement doing nothing), or any case hanging until the kill timer fired instead of exiting (the failure mode this whole construction exists to prevent). Neither occurred.

**What this does not cover.** **One build, macOS only, and a *different* build from the rest of this file** — `OpenSSH_10.3p1` here against `OpenSSH_10.2p1` everywhere else, so the two halves of the construction are observed on two builds rather than one, and neither is confirmed on the other's. `ssh-keygen(1)` still documents **no** askpass behaviour at all (the entry above), so this remains behaviour without a contract; Windows and Linux are unobserved. [Section 13](../Keys.md#13-unresolved) item 3 stays open on exactly that.

## §1 — with a controlling terminal, `never` does not make the read promptless

**Observed, 2026-09-18**, by step 10b's first code round's implementer (the runner's R2), on the Creator's Mac, `OpenSSH_10.3p1`; recorded at `Audits/Build/Keys/Decisions record.md` and in the head of `App/Desktop app and delivery/src/keys/readings.rs`, and written into [§1](../Keys.md#1-what-binds-this-before-anything-is-decided) on 2026-09-18 by that step's loose ends. `ssh-keygen -y -f` on a passphrase-protected key, run **from a pseudo-terminal**, standard input closed, `SSH_ASKPASS_REQUIRE=never`: the prompt was written **on the terminal** and the tool was **still waiting six seconds later**. The same run with `-P ""` exited 255 at once; on a key with no passphrase `-P ""` changed nothing.

**What it adds to the entry above.** Every run there was in a session with no controlling terminal. With one, the tool reads the passphrase from the terminal rather than from standard input, so closing standard input does not end the read.

**What this does not cover.** One build, macOS only. Windows and Linux are unobserved.

## §1 and G3a — a channel path the system will not take

**Observed, 2026-09-18**, at step 10b's audit round's loose ends, on the Creator's Mac, in a development copy of Smoozy started with its data folder moved: with Smoozy's own temporary area at a path of 107 bytes, the passphrase channel's path came to 116 bytes, the channel could not be opened, and the box's *Connect* with the right passphrase reached no machine; the same copy with its area at 94 bytes (and the channel's name shortened) connected, and so did one under a short folder. The limit the code enforces is not a number written down: it is the length of the path field in the system's own socket address, less the byte that ends a path, read from the system's definition when the application is built.

**What this does not cover.** macOS only; the Linux field is read the same way and never run against. The exact macOS limit is not recorded here as a number and nothing in the plan depends on one.

## §1 — the `-P` flag is a command-line value, which is why it is closed to a real passphrase

**Direct publication.** `ssh-keygen(1)`, address and date as above: `-N new_passphrase` — *"Provides the new passphrase."* — and `-P passphrase` — *"Provides the (old) passphrase."*

**Does not cover:** that a value in `argv` is readable by another login, which is Fact 60's entry above rather than this page's. The page offers no alternative on standard input — `ssh-keygen(1)` mentions standard input only for the signing options.

---

# Section 2 — the supported types, the offered set and the default (Row 241)

## T1 — the documented `-t` set, and which of it is file-backed

**Direct publication.** `ssh-keygen(1)`, address and date as above, `-t`:

> "Specifies the type of key to create. The possible values are “ecdsa”, “ecdsa-sk”, “ed25519 (the default),” “ed25519-sk”, “mldsa44-ed25519” or “rsa”. This flag may also be used to specify the desired signature type when signing certificates using an RSA CA key. The available RSA signature variants are “ssh-rsa” (SHA1 signatures, not recommended), “rsa-sha2-256”, and “rsa-sha2-512” (the default for RSA keys)."

The odd placement of the closing quote around “ed25519 (the default),” is how the page itself renders; it is reproduced rather than corrected. **Six values, of which the two carrying `-sk` are authenticator-hosted**, leaving the four plain file-backed types [T1](../Keys.md#2-the-supported-types-the-offered-set-and-the-default-row-241) counts.

**Does not cover:** any shipping build's list — the observed 10.2p1 binary's own installed page renders the `-t` list **without** `mldsa44-ed25519` ([The manuals](../The%20manuals%20—%20keys,%20secrets,%20and%20what%20cannot%20be%20told%20apart.md), Fact 32, observed). It also carries **no OpenSSH release-version marker against any key type**, so it dates nothing.

## T1 — DSA is gone rather than merely undocumented

**Direct publication, held by an input document and repeated here with its address.** OpenSSH's own release notes, read **2026-08-20** ([The manuals](../The%20manuals%20—%20keys,%20secrets,%20and%20what%20cannot%20be%20told%20apart.md), Fact 32, corrected 2026-08-21). `https://www.openssh.org/txt/release-10.0`: *"This release removes support for the weak DSA signature algorithm, completing the deprecation process that began in 2015 (when DSA was disabled by default) and repeatedly warned over the last 12 months."* and `https://www.openssh.org/txt/release-7.0`: *"Support for ssh-dss, ssh-dss-cert-\* host and user keys is disabled by default at run-time."*

**Does not cover:** Microsoft's fork, whose own current key-management page still lists DSA among the algorithms a user may specify and whose published `HostKey` defaults still name `ssh_host_dsa_key`. Whether the Windows `ssh-keygen.exe` accepts `-t dsa` is **established by nothing read**; the probe of [T2](../Keys.md#2-the-supported-types-the-offered-set-and-the-default-row-241) is what settles it per machine.

## Observed — the type probe by attempted generation (Keys [T2](../Keys.md#2-the-supported-types-the-offered-set-and-the-default-row-241), and [T3](../Keys.md#2-the-supported-types-the-offered-set-and-the-default-row-241)'s enrollment result)

`ssh-keygen -t <type> -N "" -C probe -f <tmp>/probe_<type>`, standard input closed, per candidate:

| `-t` | exit | evidence |
|---|---|---|
| `ed25519` | 0 | pair written, modes 0600/0644 |
| `rsa` | 0 | pair written |
| `ecdsa` | 0 | pair written |
| `dsa` | 255 | `unknown key type dsa`; **no files left** |
| `ed25519-sk` | 255 | `Generating public/private ed25519-sk key pair.` then `Key enrollment failed: invalid format` (no authenticator present); **no files left** |
| `ecdsa-sk` | 255 | same shape as `ed25519-sk`; **no files left** |
| `nosuchtype` | 255 | `unknown key type nosuchtype` |

A negative result for the probe mechanism would have been a failed attempt leaving files behind (a probe that dirties the location) or a success for a type the binary does not support. **The 0600/0644 modes in the first row are also [G10](../Keys.md#65-the-parent-directory-and-the-modes-row-248)'s observation.**

**What this does not cover:** one build. The `-sk` failures are **with no authenticator attached**, which is the case the probe exists to reflect and **not a claim about those types themselves** — [T3](../Keys.md#2-the-supported-types-the-offered-set-and-the-default-row-241)'s exclusion of them is a decision and carries no entry. `mldsa44-ed25519` (documented on the page above) **was not attempted**, so its absence from this build is Fact 32's observation and not this run's.

## T3 — a key can exist with no file at all

**Pointer.** [The manuals](../The%20manuals%20—%20keys,%20secrets,%20and%20what%20cannot%20be%20told%20apart.md), **Fact 33**, documented. `ssh(1)`, `-i`: *"You can also specify a public key file to use the corresponding private key that is loaded in ssh-agent(1) when the private key file is not present locally."*; `ssh-agent(1)`: *"ssh-agent is a program to hold private keys used for public key authentication."*; and `ssh_config(5)`, `PKCS11Provider`: *"a path to the PKCS#11 shared library ssh(1) should use to communicate with a PKCS#11 token providing keys for user authentication."*

**Does not cover:** anything about how such a key is reached, enrolled or driven. This is the shape [L5](../Keys.md#3-the-key-locations-and-the-one-place-premise-row-242) states as a boundary, not a capability this document plans for.

## T4 — the no-argument default, and the one thing the manual ranks

**Direct publication.** `ssh-keygen(1)`, address and date as above, DESCRIPTION: *"The type of key to be generated is specified with the -t option. If invoked without any arguments, ssh-keygen will generate an Ed25519 key."* The only ranking language anywhere on the page attached to a key type is in the `-t` entry quoted above: *"“ssh-rsa” (SHA1 signatures, not recommended)"*.

**Does not cover, and [T4](../Keys.md#2-the-supported-types-the-offered-set-and-the-default-row-241) labels itself accordingly:** the manual **does not rank the types against one another and calls none of them recommended, best or modern**. Fact 32 records the search: `recommend` across the six pages returns no recommendation attached to any key type. *The sensible modern default* is a judgement, and the document carries it as one.

## Observed — what the type probe costs, timed ([T2b](../Keys.md#2-the-supported-types-the-offered-set-and-the-default-row-241))

**Observed on this build — and this is a NEW record, run on 2026-09-02, not a recovered one.** [T2b](../Keys.md#2-the-supported-types-the-offered-set-and-the-default-row-241) was written earlier the same day carrying *"0.03 s, 0.03 s and 0.51 s"* with no date, no machine, no build, no command and no label, and this file's own gathered-gaps item 7 said in as many words that **no key check had been timed anywhere on this project**. The sitting's own decisions record ends *"Nothing was looked up, nothing was rented, nothing was spent."* **Nothing was invented to fill it and the figures were not defended. The probe was run**, on the machine in the third machine record above.

`ssh-keygen -q -t <type> -N "" -C smoozy-probe -f <scratch>/k_<type>`, three runs of each type, the pair removed between runs, wall time taken around the invocation alone:

| `-t` | runs, seconds | what was written |
|---|---|---|
| `ed25519` | 0.034, 0.031, 0.031 | 256-bit pair |
| `ecdsa` | 0.032, 0.031, 0.031 | 256-bit pair |
| `rsa` | 0.278, 0.240, 0.173 | 3072-bit pair, the build's own default |

**The two small figures reproduce the old ones; the 0.51 s does not**, and no run of this build came near it. **`rsa` is not constant work** — a prime search is a search — so the spread is what is carried and T2b states no single number for it. A tenth run was attempted and refused by the permission system, so the sample is three per type and is stated as three.

**What this does not cover:** one build on one machine, an idle machine, three runs per type, and **nothing about Windows or Linux**, where the same probe has never been run. It says nothing about the cost of a *key check* ([Section 13](../Keys.md#13-unresolved) item 16), which is a different act on a key that already exists — the gathered-gaps item below is narrowed rather than closed. And the timing measures the whole invocation, process start included, which is what T2b is about.

## Observed and documented — what `ssh -Q key-plain` actually answers ([T2b](../Keys.md#2-the-supported-types-the-offered-set-and-the-default-row-241)'s three grounds)

**T2b declined the documented instant alternative on three grounds and said *"each is established rather than supposed"* while citing nothing.** Both halves are here.

**Documented.** `ssh(1)`, the manual page shipped with the operating system, read with `man ssh` on the machine in the third machine record, **2026-09-02**. Its `-Q` entry, verbatim in the parts the grounds turn on:

> "Queries for the algorithms supported by one of the following features: cipher (supported symmetric ciphers), … key (key types), key-ca-sign (valid CA signature algorithms for certificates), key-cert (certificate key types), key-plain (non-certificate key types), key-sig (all key types and signature algorithms) …"

**The word the first ground rests on is the page's own: *algorithms*.**

**Observed on this build.** `ssh -Q key-plain`, same machine, same date, printed exactly seven lines in this order:

> ssh-ed25519
> sk-ssh-ed25519@openssh.com
> ecdsa-sha2-nistp256
> ecdsa-sha2-nistp384
> ecdsa-sha2-nistp521
> sk-ecdsa-sha2-nistp256@openssh.com
> ssh-rsa

**All three grounds are read straight off that output.** *Not one of the seven is a value `-t` accepts* — the `-t` set is `ecdsa`, `ecdsa-sk`, `ed25519`, `ed25519-sk`, `mldsa44-ed25519`, `rsa` (the T1 entry above), so a mapping rule would be needed and no publication states one. *One accepted type is split into three curves* — `nistp256`, `nistp384`, `nistp521`. *The two authenticator-hosted types [T3](../Keys.md#2-the-supported-types-the-offered-set-and-the-default-row-241) excludes are in the list* — the second and sixth lines, both `sk-`. And the third ground needs no run at all: `-Q` is a flag of **`ssh`**, documented on the client's own page, so it cannot answer anything about the **generator**'s presence, which is [G6](../Keys.md#63-the-generator-probed-never-assumed-row-248)'s second job for the same probe.

**What this does not cover:** one build on one machine. The Windows fork's `ssh.exe` was not run and its output is established by nothing — which is exactly why [T2](../Keys.md#2-the-supported-types-the-offered-set-and-the-default-row-241)'s probe reads the binary in front of it rather than any list. Whether `mldsa44-ed25519` would appear on a build that offers it was not tested; it is absent from this one, consistent with Fact 32.

---

# Section 3 — the key locations, and the one-place premise (Row 242)

## L1 — the per-user default directory on each system

**Pointer.** [The manuals](../The%20manuals%20—%20keys,%20secrets,%20and%20what%20cannot%20be%20told%20apart.md), **Fact 33**, documented. `ssh(1)` FILES: *"`~/.ssh/` — This directory is the default location for all user-specific configuration and authentication information. There is no general requirement to keep the entire contents of this directory secret, but the recommended permissions are read/write/execute for the user, and not accessible by others."* The word in that entry is **default**; *only* does not appear in it. macOS: the same `~/.ssh`, confirmed against the Apple-shipped `ssh(1)` on the running system (observed). Linux: the same, and Canonical states it outright — *"By default, the public key is saved in the file `~/.ssh/id_<algorithm>.pub`, while `~/.ssh/id_<algorithm>` is the private key."* Windows: `%USERPROFILE%\.ssh`, Microsoft, documentation alone.

**Does not cover, and the Windows half carries an extra narrowing:** Microsoft equates `%USERPROFILE%` with the profile folder and calls the literal path *"A typical path is `C:\Users\<username>`"* — **the directory is the same, the string is not guaranteed**, so the variable is expanded and the path never hard-coded. **Which of `$HOME` and `%USERPROFILE%` wins on Windows when they disagree is settled by nothing read** and needs a Windows machine. On macOS the offered default path was observed **not** to come from `$HOME`; the mechanism behind that is inferred and unestablished, and nothing in this document reads either variable — [G9](../Keys.md#65-the-parent-directory-and-the-modes-row-248) always supplies the path itself.

## L2 — `C:\ProgramData\ssh` is the machine's host-key and server-configuration directory

**Direct publications, held by an input document and repeated here with their addresses.** Both read **2026-08-20** ([The manuals](../The%20manuals%20—%20keys,%20secrets,%20and%20what%20cannot%20be%20told%20apart.md), Fact 33, and [Facts](../Facts.md), Fact 33).

- Microsoft, *Key-Based Authentication in OpenSSH for Windows*, `https://learn.microsoft.com/en-us/windows-server/administration/openssh/openssh_keymanagement`: *"Because there's no user associated with the `sshd` service, the host keys are stored under C:\ProgramData\ssh."*, with `administrators_authorized_keys` in the same directory, while a user's own key is offered at `C:\Users\<name>\.ssh`.
- Microsoft, *OpenSSH Server Configuration for Windows*, `https://learn.microsoft.com/en-us/windows-server/administration/openssh/openssh-server-configuration`, page dated 2025-08-05, under **HostKey**: *"The defaults are: `#HostKey __PROGRAMDATA__/ssh/ssh_host_rsa_key`, `#HostKey __PROGRAMDATA__/ssh/ssh_host_dsa_key`, `#HostKey __PROGRAMDATA__/ssh/ssh_host_ecdsa_key`, `#HostKey __PROGRAMDATA__/ssh/ssh_host_ed25519_key`"* and *"If the defaults aren't present, sshd automatically generates them on a service start."*
- OpenSSH, `sshd_config(5)` at `https://man.openbsd.org/sshd_config#HostKey`, read the same day: a `HostKey` *"Specifies a file containing a private host key used by SSH."*

**None of the four default paths carries `.pub`**, so private key files do live there and they are the machine's.

**Does not cover:** the directory's default access control, which is unread and which nothing in [L2](../Keys.md#3-the-key-locations-and-the-one-place-premise-row-242) depends on. Nothing here has been confirmed against a real Windows installation.

## L3 — the one-place premise is false

**Pointer.** [The manuals](../The%20manuals%20—%20keys,%20secrets,%20and%20what%20cannot%20be%20told%20apart.md), **Fact 33**. Four documented routes put a usable key outside the default directory: `IdentityFile` — *"Specifies a file from which the user's ECDSA, authenticator-hosted ECDSA, Ed25519, authenticator-hosted Ed25519 or RSA authentication identity is read"*, and *"It is possible to have multiple identity files specified in configuration files; all these identities will be tried in sequence"*; `ssh(1)`'s *"It is possible to have multiple `-i` options"*; the agent; and a PKCS#11 token. That input document draws the consequence in the words this document inherits: **"any 'delete key' that unlinks files must (a) know, per key, whether a file backs it, and (b) know the file's real path rather than assuming `~/.ssh/<name>`"**, and *"The documentation does not support a rule of the form 'the key's file is `~/.ssh/<name>`'."*

**Does not cover:** that a key can live *anywhere*, which that fact labels **inferred** — the manual states **no restriction** on the path, which is not the same as stating there is none.

---

# Section 4 — what is a key, what *usable* is, and how a list gets its answers (Rows 245, 253, 423)

## U2 — what else ordinarily lives in a key location, and why no filename list closes it

**Pointer.** [The manuals](../The%20manuals%20—%20keys,%20secrets,%20and%20what%20cannot%20be%20told%20apart.md), **Fact 34**, documented. `ssh(1)` FILES gives `authorized_keys`, `config`, `environment`, `known_hosts` and `rc`; `sshd(8)` adds `authorized_keys2`; `ssh(1)`'s `-i` adds certificates — *"ssh will also try to load certificate information from the filename obtained by appending `-cert.pub` to identity filenames"*; `ssh-agent(1)`'s `-a` adds a subdirectory of sockets — *"The default is to create a socket at a random path matching `$HOME/.ssh/agent/s.*`."*; and `ssh_config(5)`'s `Include` puts **arbitrary filenames** there — *"Files without absolute paths are assumed to be in `~/.ssh` if included in a user configuration file"*. **So no closed filename list is possible.**

**Does not cover:** an exhaustive inventory, which that fact says outright does not exist. That *no* filename is excluded by `Include` is **inferred** — the page states no constraint on the name. Microsoft's own naming rule — *"The .pub file is the public key, and the file without an extension is the private key"* — is recorded there as **not usable as a discriminator**, since `config`, `environment`, `rc`, `known_hosts` and `authorized_keys` also have no extension.

## U2 — the private-key armour line

**Pointer.** [The manuals](../The%20manuals%20—%20keys,%20secrets,%20and%20what%20cannot%20be%20told%20apart.md), **Fact 34**, observed on a throwaway key: first line `-----BEGIN OPENSSH PRIVATE KEY-----`, last line `-----END OPENSSH PRIVATE KEY-----`. The manuals do not print it: *"`BEGIN OPENSSH` occurs zero times in the served ssh-keygen(1), ssh(1) and sshd(8)"*. The legacy alternative is documented only as a format — `ssh-keygen(1)`: *"It is still possible for ssh-keygen to write the previously-used PEM format private keys using the `-m` flag."* The PEM and PKCS8 armour labels themselves are the RFC entry under U8 below.

**Does not cover:** any armour line's stability across builds, which no manual page states.

## U2 — `ssh-keygen -l -f` fingerprints `authorized_keys` and `known_hosts` at exit 0 (Keys [U2](../Keys.md#41-what-in-a-key-location-is-listed-as-a-key-row-245))

An `authorized_keys` file was composed in the temporary folder from the generated public key: one plain line, and one line carrying an options prefix (`no-agent-forwarding,command="/bin/true" ssh-ed25519 …`). `ssh-keygen -l -f` on it exited **0** and printed a fingerprint line **for each of the two lines**, options-prefixed included. The same reading on a composed `known_hosts` line (`example.invalid ssh-ed25519 …`) also exited 0 and fingerprinted it. A negative result would have been exit 255 on either file, which would have made the fingerprint read a usable discriminator; it is not.

**What this does not cover:** one build. Fact 34's fuller file inventory was not re-run here. **This run is what turns Fact 34's `authorized_keys` half from an assertion by analogy into an observation** — [Section 13](../Keys.md#13-unresolved) item 13, and `Facts.md` Fact 34 carries it back.

## U4 clause 5 — the permission gate on macOS and Linux

**Pointer, two facts.** [The manuals](../The%20manuals%20—%20keys,%20secrets,%20and%20what%20cannot%20be%20told%20apart.md).

- **Documented, Fact 33.** `ssh(1)` FILES, on the private key files: *"These files contain sensitive data and should be readable by the user but not accessible by others (read/write/execute). ssh will simply ignore a private key file if it is accessible by others."*
- **Observed, Fact 37.** With mode `0644` on the key file, `ssh-keygen -y` **never reaches the passphrase**: it prints the `WARNING: UNPROTECTED PRIVATE KEY FILE!` banner, *"Permissions 0644 for '…/k_enc' are too open."*, *"This private key will be ignored."* and `Load key "…/k_enc": bad permissions`, exit 255. `bad permissions` is a third distinct terminal string beside `incorrect passphrase supplied to decrypt private key` and `invalid format`.

**Does not cover:** which modes short of 0644 trip it, and any system but the one observed. **Route B — a connection with a too-open key file — was never run at all**, at either log level (Fact 37's unresolved 10).

**The label the document now carries, restated here because a citation has to say what kind of claim it is backing.** [U4](../Keys.md#42-usable-composed-row-245) clause 5 is **the documented recommendation applied as a rule**, not an observed rule: the documented half gives the direction and no line, the observed half gives one point on it — mode 0644 — and **every mode short of 0644 is refused by Smoozy on the recommendation alone.** The document carries the label in place, and what it costs rides with [Section 13](../Keys.md#13-unresolved) item 8 rather than as an item of its own. **What would settle it:** one key at mode 0640, one load, on a machine that already exists.

## U4 clause 5 — the Windows form of the gate

**Direct publication, and it is the weaker of the two halves.** The Windows port's own project wiki, **`https://github.com/PowerShell/Win32-OpenSSH/wiki/Security-protection-of-various-files-in-Win32-OpenSSH`**, read **2026-08-20** (the address is carried in [Keys](../Keys.md#13-unresolved) item 8): *"User's private keys are user's credentials. To prevent unauthorized access to these files, private keys need to be owned by the user and no other user should have access to user's key files."*

**Flagged as weaker than a Learn page, and it is the best found.** Microsoft's own key-management page — `https://learn.microsoft.com/en-us/windows-server/administration/openssh/openssh_keymanagement`, `ms.date: 2025-10-03`, `updated_at: 2026-04-21`, read 2026-08-20 — **says nothing about the client refusing a private key at all**; its only access-control statements are about the **server**, host keys and `administrators_authorized_keys`. The wiki's own *"Permissions for … are too open"* passage is likewise about a host key on the server.

**Does not cover the thing the gate would need: that the Windows client *refuses* a key whose access control is loose.** That is stated by nothing read, which is why [U4](../Keys.md#42-usable-composed-row-245) clause 5 is **not applied on Windows** and why no verdict is invented there. What would settle it is in [Section 13](../Keys.md#13-unresolved) item 8: one Windows installation, one private key granting a second local account, one connection attempt.

**The reading route is spent and the running route is with the Creator — stated 2026-09-02.** Both pages above were read and neither says the client refuses such a key, so **no further reading will close this** and no citation is coming. What is left is a run, and it needs a Windows installation: **not a rental** — the Creator ruled on 2026-08-31 that no further machine is rented to answer a question — but a guest on the Creator's own Mac, which `What Build inherits` item 4 records as having **no hypervisor installed at all**. So the citation side of this entry is **closed with nothing to add**, and the observation side is unresolved on one decision that is the Creator's own. [Keys](../Keys.md#13-unresolved) §13 item 8 carries what breaks and what holds; nothing here is pending and no space is left for an answer.

## U6 — no diagnostic string of the tool's is ever parsed

**Pointer.** [The manuals](../The%20manuals%20—%20keys,%20secrets,%20and%20what%20cannot%20be%20told%20apart.md), **Fact 37**, documented as an absence: *"no OpenSSH manual page documents a single one of the diagnostic strings the tools emit"* — eighteen strings searched across all seven pages, zero hits in every page. `ssh(1)`'s entire exit-status contract is *"ssh exits with the exit status of the remote command or with 255 if an error occurred."*, and **`ssh-keygen(1)` has no EXIT STATUS section at all**.

**Does not cover:** the strings' stability, which nothing guarantees. That is the ground for [U6](../Keys.md#42-usable-composed-row-245)'s closed reason set being Smoozy's own reading of which clause failed — a decision, carrying no entry.

## U7 — the fingerprint read works from the private file alone, WITH THE `.pub` MOVED ASIDE, and only then

**This heading said *from the private file alone* without that qualifier until 2026-09-03**, and the clause it supported said the same. **U7b overturned both.** The citation below is exactly what it always was; what was wrong was the generalisation drawn from it.

**Pointer.** [The manuals](../The%20manuals%20—%20keys,%20secrets,%20and%20what%20cannot%20be%20told%20apart.md), **Fact 35 Half A**, observed: `ssh-keygen -l -f` on an encrypted private key returned the fingerprint at exit 0 **with the `.pub` moved aside** and **with no prompt**. The document notes this **exceeds what `ssh-keygen(1)`'s `-l` claims** — the page describes only *"will try to find the matching public key file"*.

**Does not cover — and this is the correction of 2026-09-03: the ordinary case, where a `.pub` is sitting beside the key.** The observation was made in the one arrangement where no `.pub` existed, so it says nothing about which half `-l -f` reads when one does. See the next entry.

**Does not cover, and Fact 35's own unresolved says so:** *"The manual does not state that the public half of a key is stored unencrypted inside an OpenSSH-format private key file."* It is not promised for the legacy PEM format, **which was not tested there** — the PEM result is the Section 13 item 6 entry below.

## U7b and F1 — which invocation reads which half

**Observed at Build, and it is a new record rather than a recovered one.** Run on the Creator's Mac, **macOS 26.6.2**, **`OpenSSH_10.3p1`**, **2026-09-03**, by the code being written against [W7a](../Keys.md#95-the-copy-half-row-247)'s own words — a test that passed the comparison it was written to fail.

Two keys `a` and `b` were generated. `b`'s public half was copied over `a.pub`.

- **`ssh-keygen -l -E sha256 -f a` printed `b`'s fingerprint and `b`'s comment.** With `a.pub` removed, the same call printed `a`'s own. **So `-l -f` is a public-half read wherever a `.pub` exists**, and a private-half read only in the arrangement the entry above happens to have tested.
- **`ssh-keygen -y -f a`, with `b.pub` still standing in for `a.pub`, printed `a`'s own public line**, whose fingerprint is `a`'s. **So `-y -f` derives the public half from the private key itself and is uninfluenced by the file beside it.**

**Does not cover:** any build but the one observed, and any system but this Mac; `ssh-keygen(1)` documents no such distinction for `-l`, which is what let the wrong invocation stand in the plan from the day it was written. **It also does not cover the encrypted case at all**: `-y -f` on a key whose passphrase is not available ends at exit 255 with the tool's own refusal (promptless under §1's construction), which is why the encrypted-key case belongs to [U7](../Keys.md#43-the-readings-behind-the-clauses)'s public-half fallback and not to [F1](../Keys.md#51-where-the-fingerprint-is-read-from-row-246).

## U7 — the fingerprint is identical from the private file and the public one

**Pointer.** [The manuals](../The%20manuals%20—%20keys,%20secrets,%20and%20what%20cannot%20be%20told%20apart.md), **Fact 76**, observed: `ssh-keygen -l -f` on `plain_rsa` and on `plain_rsa.pub` printed the same `SHA256:` field.

**Does not cover:** any build but the one observed; the manual states nothing about it.

## U8 — the `openssh-key-v1` container read

**Pointer, with all three narrowings carried.** [The manuals](../The%20manuals%20—%20keys,%20secrets,%20and%20what%20cannot%20be%20told%20apart.md), **Fact 35**, observed: strip the armour, base64-decode, and the container begins with the literal `openssh-key-v1\0` followed by a length-prefixed cipher name and a length-prefixed KDF name. On the four throwaway keys — `default ('none','none')`, `locked ('aes256-ctr','bcrypt')`, `plain_rsa ('none','none')`, `plain_ecdsa ('none','none')`.

**Does not cover, and the three narrowings are the reason [U8](../Keys.md#44-the-protection-reading-and-the-availability-answer-row-245) keeps the probe as confirmation:** one build; **four keys of which only one was encrypted**, so *anything other than `none`/`none` means protected* rests on a single data point; and the container layout is **documented on no manual page** — `openssh-key-v1` returns count zero across all six pages searched — so it is an observation of file bytes, not a citation.

## U8 — the `-P ''` probe and its confounders

**Pointer.** [The manuals](../The%20manuals%20—%20keys,%20secrets,%20and%20what%20cannot%20be%20told%20apart.md), **Facts 35 and 37**, observed: `ssh-keygen -y -P '' -f <path>` exits 0 on a key with no passphrase and 255 with `incorrect passphrase supplied to decrypt private key` on a key with one. Fact 37's route-A table records **exit 255 arising from at least four other causes on the same build** — `invalid format` for a garbage file, a truncated key, a mangled armour, a non-base64 body, a zero-length file and a public key passed as a private one; `No such file or directory` for a missing file; and `bad permissions` for a too-open one.

**Does not cover:** the exit status as an answer on its own. Fact 35's own narrowing: the mapping was written as a general rule and **narrowed to the two keys tested**. That is exactly why [U8](../Keys.md#44-the-protection-reading-and-the-availability-answer-row-245) takes the yes-or-no from the container and uses the probe only to confirm.

## U8 — the legacy PEM and PKCS8 header answer (Keys [U8](../Keys.md#44-the-protection-reading-and-the-availability-answer-row-245))

**Direct publications.** Read **2026-08-20**; the addresses are held in the project at [Security and licence's companion](Security%20and%20licence.sources.md), which supplied them on 2026-08-23 and re-read the words at the publications.

- **RFC 1421 §4.6.1.1 and §4.6.1.3**, at **`https://www.rfc-editor.org/rfc/rfc1421.txt`**: *"The \"Proc-Type:\" encapsulated header field, required for all PEM messages, identifies the type of processing performed on the transmitted message."* and *"the field is required for all messages specified as \"ENCRYPTED\" in the \"Proc-Type:\" field."*
- **RFC 7468 §10 and §11**, *Textual Encodings of PKIX, PKCS, and CMS Structures*, April 2015, at **`https://www.rfc-editor.org/rfc/rfc7468.txt`**: unencrypted key structures *"are encoded using the \"PRIVATE KEY\" label"* and encrypted ones *"are encoded using the \"ENCRYPTED PRIVATE KEY\" label"*. The full sentences at the publication are *"Unencrypted PKCS #8 Private Key Information Syntax structures (PrivateKeyInfo), renamed to Asymmetric Key Packages (OneAsymmetricKey), are encoded using the \"PRIVATE KEY\" label."* (§10) and *"Encrypted PKCS #8 Private Key Information Syntax structures (EncryptedPrivateKeyInfo), called the same in [RFC5958], are encoded using the \"ENCRYPTED PRIVATE KEY\" label."* (§11).

**Observed on 2026-08-23**, on the machine in the second machine record — the same four keys re-made with `-m PEM` and `-m PKCS8`, with and without a passphrase (2048-bit RSA in every case, since neither legacy format takes an `ed25519` key), each read in its own session with standard input on `/dev/null`. **The first three lines of each file, verbatim:**

| key | line 1 | line 2 | line 3 |
|---|---|---|---|
| plain PEM | `-----BEGIN RSA PRIVATE KEY-----` | `MIIEpQIBAAKCAQEA6v06qZVs…` (base64 body) | base64 body |
| encrypted PEM | `-----BEGIN RSA PRIVATE KEY-----` | `Proc-Type: 4,ENCRYPTED` | `DEK-Info: AES-128-CBC,358065EA3F63CD0F306742B88BCBDD8E` |
| plain PKCS8 | `-----BEGIN PRIVATE KEY-----` | `MIIEvgIBADANBgkqhkiG9w0B…` (base64 body) | base64 body |
| encrypted PKCS8 | `-----BEGIN ENCRYPTED PRIVATE KEY-----` | `MIIFHzBJBgkqhkiG9w0BBQ0w…` (base64 body) | base64 body |

**This is what forced [U8](../Keys.md#44-the-protection-reading-and-the-availability-answer-row-245) to be written as two reads rather than one, corrected 2026-08-23.** The two PEM keys carry the **identical first line**, so line 1 decides nothing for PEM and line 2 is the whole answer; the two PKCS8 keys differ on **line 1 alone**, and their line 2 is base64 body, so line 2 decides nothing for PKCS8. The document's earlier *"the answer is in plain text on the file's own second line"* was true of the PEM half only, and this file recorded that overreach before it was repaired.

**A negative result would have been:** `Proc-Type:` appearing on the plain PEM key, or the two PKCS8 armour labels coming back identical — either would have left the header read unable to separate protected from unprotected without the probe.

**Does not cover:** one build, one Mac, four keys, RSA only, and **only the two formats `ssh-keygen -m` writes** — no key produced by any other tool was read. Fingerprinting a PEM key without its `.pub` is a separate result and is the next entry.

## Section 13 item 6 — the PEM fingerprint refusal

**Observed — and this is a NEW record, run on 2026-08-23, not a recovered one.** Neither of the two runs [Keys](../Keys.md#13-unresolved) item 6 describes was recorded anywhere under `Preparation/`: [Security and licence's companion](Security%20and%20licence.sources.md) names the first outright as *"a separate result, and `Keys`' to own"* — and this file did not hold it. Documents searched: this file, `Facts.md`, all three input documents, `Security and licence.md` and its companion, and `Rows/Keys.md`. **Nothing was invented to fill it. Both were run again**, on the machine in the second machine record — **macOS 26.6.2 (build 25G83)**, **`OpenSSH_10.3p1`** — over the four keys of the entry above plus an encrypted and a plain `openssh-key-v1` key, every invocation in its own session with standard input on `/dev/null`.

**`ssh-keygen -l -f <private>`, with the `.pub` moved aside for the duration and put back after:**

| key | exit | what came back |
|---|---|---|
| plain PEM | 0 | `2048 SHA256:nNMx0YdmerIwatf7cEDAZt5Bl8GhbOGgDLHnuhm5tK4 no comment (RSA)` |
| **encrypted PEM** | **255** | `<path> is not a key file.` |
| plain PKCS8 | 0 | `2048 SHA256:FNfzqQSQNuynNwcDVr5iuMHZ08lBGGPKcpR4ji2QE14 no comment (RSA)` |
| **encrypted PKCS8** | **255** | `<path> is not a key file.` |
| **encrypted `openssh-key-v1`** | **0** | `2048 SHA256:Qdg1pEQvQLfFALER6Psmtj3t55YhBawplzZoatyf0XU no comment (RSA)` |
| plain `openssh-key-v1` | 0 | `2048 SHA256:/0FlrKOS9IaJo7u63SoCi63uBVBFr86DPCOU1bR9qU4 v1_plain (RSA)` |

**The same six keys with the `.pub` present: every one exits 0 and prints its fingerprint**, the encrypted PEM key included — `2048 SHA256:sfn7EIuTb7N/oijV/3X4TNp7f0i3+vOXJbxW8z7omq0 pem_enc (RSA)`. **That is the pair [U7](../Keys.md#43-the-readings-behind-the-clauses)'s fallback rests on.** Note the comment field in the first table: with the `.pub` aside the readable keys print `no comment`, which is Fact 35 Half A's *the comment is not recoverable* seen again.

**`ssh-keygen -y -P '' -f <private>`, `.pub` moved aside** — [U8](../Keys.md#44-the-protection-reading-and-the-availability-answer-row-245)'s confirming probe: **exit 0** for plain PEM, plain PKCS8 and plain `openssh-key-v1`; **exit 255** with `Load key "<path>": incorrect passphrase supplied to decrypt private key` for encrypted PEM, encrypted PKCS8 and encrypted `openssh-key-v1`. **Six for six, and no askpass helper was in the environment for any of them**, so every refusal above is a refusal and not a suppressed question.

**A negative result would have been:** the encrypted PEM or PKCS8 key **prompting** rather than refusing (which is what the document feared and what would have broken the promptless construction), or the encrypted `openssh-key-v1` key refusing alongside them (which would have taken [F1](../Keys.md#51-where-the-fingerprint-is-read-from-row-246)'s reading down for every key Smoozy makes), or the `-P ''` probe disagreeing with the header read on any of the six.

**What this does not cover.** One build, macOS only, **and a later build than the rest of this file** — `OpenSSH_10.3p1` against `OpenSSH_10.2p1`. RSA only. `ssh-keygen`'s own `-m` output only — no key written by any other tool was read. **And the refusal message is not a contract**: `<path> is not a key file.` is documented on no manual page (the Fact 37 entry above), which is why [U6](../Keys.md#42-usable-composed-row-245) reads the failed clause and never the string. [Section 13](../Keys.md#13-unresolved) item 6's stated cost is unchanged by the re-run: a PEM key with no `.pub` shows *unreadable* rather than usable-and-protected.

## U9 — reaching the store can raise a system prompt; reading the local record cannot

**Pointer.** [The manuals](../The%20manuals%20—%20keys,%20secrets,%20and%20what%20cannot%20be%20told%20apart.md), **Fact 38**, macOS, documented: access on the **file-based keychain** is governed by per-item access control lists, so a caller not on the list triggers a system dialog, and Apple documents that the grant is lost when the application is changed or updated.

**Does not cover:** Windows, where **no prompt is documented on the generic-credential path at all**, and Linux, where the Secret Service specification *"does not mandate any form of access control"*. [U9](../Keys.md#44-the-protection-reading-and-the-availability-answer-row-245)'s rule is written to the worst of the three and needs no per-system reading.

---

# Section 5 — the fingerprint (Rows 246, 293, 429)

## F1 to F3 — what a fingerprint is and what it is taken over

**Pointer.** [The manuals](../The%20manuals%20—%20keys,%20secrets,%20and%20what%20cannot%20be%20told%20apart.md), **Fact 76**. Documented on four pages — `ssh-keygen(1)` `-E`, `ssh-agent(1)` `-E`, `ssh_config(5)` `FingerprintHash`, `sshd_config(5)` `FingerprintHash` — *"Valid options are: “md5” and “sha256”. The default is “sha256”."* Observed: the printed line is bit size, then `SHA256:` plus unpadded base64 (or `MD5:` plus colon-hex), then comment, then key type in parentheses. **What the digest is taken over was determined by reconstruction, not from the manual**: base64-decoding the second field of the `.pub` line and hashing those raw bytes reproduced both printed forms exactly — so it is **independent of the comment and of file whitespace**, both exercised (`ssh-keygen -c -C 'changed@local' -P ''` left the `SHA256:` field unchanged; appending three spaces and a CRLF to the `.pub` left it unchanged).

**Does not cover:** any documented statement of the digest input — the reconstruction is the source. One platform; **no cross-operating-system observation was made**. `ssh(1)`'s own hedge about *"random art"* patterns being *"not unambiguous"* is about the ASCII-art rendering and **cannot be transferred to the fingerprint string**.

## F5 and F8 — MD5 collisions are cheap

**Direct publication.** RFC 6151, *Updated Security Considerations for the MD5 Message-Digest and the HMAC-MD5 Algorithms*, March 2011, at **`https://www.rfc-editor.org/rfc/rfc6151.txt`** — the address is held in the project at [Security and licence's companion](Security%20and%20licence.sources.md), which supplied it on 2026-08-23 and confirmed both sentences word for word at the publication. First read **2026-08-20**. §2.1, *Collision Resistance*: *"The attacks presented in [KLIM2006] can find MD5 collision in about one minute on a standard notebook PC (Intel Pentium, 1.6GHz)."* §2, *Security Considerations*: *"MD5 is no longer acceptable where collision resistance is required such as digital signatures."*

**Does not cover:** SHA-256 — see the next entry.

## F8 — that no SHA-256 collision exists

**UNRESOLVED, and permanently so.** **No first-party sentence says a SHA-256 collision does not exist**, because an absence of published collisions is not a citable statement, and neither [Keys](../Keys.md) nor this file asserts it. Fact 76 says the same in its own words: *"No answer is offered here by reasoning from properties of hash functions, because that would be a guess dressed as a fact."* Documents searched for one: `Facts.md`, all three input documents, every companion under `Sources/`, and `Groundwork/Open Threads.md` — the last of which carries the question as **item 8a, Due: the Creator**.

**Breaks:** [F4](../Keys.md#53-the-second-reference-new-location-against-replacement-row-246) folds two colliding keys into one entry and the store holds one passphrase against a key it does not belong to, wherever [F5](../Keys.md#53-the-second-reference-new-location-against-replacement-row-246)'s blob comparison cannot run; Row 293's world-side clause is asserted by nothing. **Holds:** every mechanism in Section 5, which functions identically under either answer, and F5's partial detection.

## F8 — the hash is a selectable setting

**Direct publication, held by an input document.** `ssh_config(5)`, `FingerprintHash`, at `https://man.openbsd.org/ssh_config.5`, footer as served *OpenBSD-current, July 11, 2026*, read 2026-08-16: *"Specifies the hash algorithm used when displaying key fingerprints. Valid options are: md5 and sha256 (the default)."*

**Does not cover:** any shipping build's default. Those pages say nothing about other platforms' builds — which is why [F8](../Keys.md#54-the-identity-claim-row-293) names the hash at every call rather than relying on the default.

## F8 — cross-system stability

**Direct publication, plus a pointer to the reconstruction.** RFC 4253, *The Secure Shell (SSH) Transport Layer Protocol*, January 2006, **§6.6 *Public Key Algorithms***, at **`https://www.rfc-editor.org/rfc/rfc4253.txt`** — the address is held in the project at [Security and licence's companion](Security%20and%20licence.sources.md), supplied and re-read there on 2026-08-23. The two quoted strings — *"string certificate or public key format identifier"* and *"byte[n] key/certificate data"* — are **two lines of the section's own format block**, `string    certificate or public key format identifier` and `byte[n]   key/certificate data`, quoted with the block's internal spacing collapsed. The hand recomputation reproducing the tool's printed line in both hash forms is [The manuals](../The%20manuals%20—%20keys,%20secrets,%20and%20what%20cannot%20be%20told%20apart.md)'s, Fact 76, observed.

**Does not cover:** any second machine. The recomputation was done on the same one Mac; **no fingerprint has ever been taken on a Windows or Linux build on this project.** What the RFC settles is that the *input* is protocol-fixed, leaving nothing for a system to vary — an argument, not an observation.

---

# Section 6 — making a key (Rows 248, 243, 249)

## G1 and G2 — the generation flags

**Direct publication.** `ssh-keygen(1)`, address and date as at Section 1. DESCRIPTION: *"ssh-keygen generates, manages and converts authentication keys for ssh(1)."* and *"Normally this program generates the key and asks for a file in which to store the private key. The public key is stored in a file with the same name but ".pub" appended. The program also asks for a passphrase. The passphrase may be empty to indicate no passphrase (host keys must have an empty passphrase), or it may be a string of arbitrary length."* Flags: `-C comment` — *"Provides a new comment."*; `-f filename` — *"Specifies the filename of the key file."*; `-t` and `-N` as quoted above.

**Does not cover, and [The manuals](../The%20manuals%20—%20keys,%20secrets,%20and%20what%20cannot%20be%20told%20apart.md) Fact 39 relabels it:** that **`-N ""` is the way to generate an unprotected key non-interactively is *inferred* from two sentences — the page never says `-N` accepts an empty string** — and is *observed* to work on this build, in the probe table above. Microsoft documents neither `-C` nor `-N` for the Windows build anywhere found; the options are **expected** to carry to the fork and that expectation is confirmed by nothing.

## Observed — the askpass route at key generation (Keys [G3](../Keys.md#62-the-passphrase-route-row-248), and [G1](../Keys.md#61-the-one-act-and-the-tool-row-248)'s comment handling)

**What was run.** An executable askpass script that logs each invocation and prints a fixed passphrase; then, with standard input closed (`< /dev/null`) and the environment carrying `SSH_ASKPASS=<script>` and `SSH_ASKPASS_REQUIRE=force`:

```
ssh-keygen -t ed25519 -C "Test key — комментарий с пробелами" -f <tmp>/key_askpass
```

**What came back.** Exit 0. The askpass script was invoked **exactly twice**, with prompts (logged verbatim): `Enter passphrase for "<tmp>/key_askpass" (empty for no passphrase): ` and `Enter same passphrase again: `. The private and public files were written (`key_askpass` mode 0600, 529 bytes; `key_askpass.pub` mode 0644, 149 bytes), and the fingerprint line carried the comment verbatim, spaces, apostrophe and Cyrillic included: `SHA256:Q75f5RAh0f3psw6yDYFmWmxFDaWIiCwdG1RTS7bb0Sc Test key — комментарий с пробелами`. **That last is [G1](../Keys.md#61-the-one-act-and-the-tool-row-248)'s observation as well as G3's** — an arbitrary comment passed as one argument survives into the `.pub` and the fingerprint line unchanged.

**That the passphrase was genuinely applied.** `ssh-keygen -y -P "" -f <tmp>/key_askpass` exited 255 with `Load key "<tmp>/key_askpass": incorrect passphrase supplied to decrypt private key`. A load with the correct passphrase supplied through the same askpass route (`ssh-keygen -y -f <tmp>/key_askpass`, no `-P`) exited 0, invoked askpass **once**, and printed a public key byte-identical to `key_askpass.pub`'s type-and-blob fields.

**A negative result would have been:** the run hanging or exiting non-zero without invoking the script (askpass not honoured for generation); or exit 0 with the `-P ""` probe then succeeding (a key silently written unprotected). Neither occurred.

**What this does not cover.** One build, macOS only. **Windows and Linux are unobserved for this behaviour**, and no manual page read documents it anywhere — the route is observation, not contract. This is [Facts](../Facts.md)' **Fact 115**, its Mac half and nothing more; [Section 13](../Keys.md#13-unresolved) item 3 carries what breaks.

## Observed — a wrong passphrase over the askpass load is asked once, not looped (Keys [S5](../Keys.md#83-a-passphrase-that-stops-working-row-250))

`ssh-keygen -y -f <tmp>/key_askpass` with an askpass script printing a wrong value: exit 255, askpass invoked **exactly once**, message `Load key "<tmp>/key_askpass": incorrect passphrase supplied to decrypt private key`. A negative result would have been repeated invocations, the shape `ssh-add` was observed to produce elsewhere (Fact 37's 59 invocations in 12 seconds). What this does not cover: `ssh-keygen -y` alone, this build alone; nothing is claimed for other tools.

## Observed — `ssh-keygen` refuses an existing `-f` target with no terminal input (Keys [G8](../Keys.md#64-the-filename-scheme-and-the-collision-row-243))

With `<tmp>/key_existing` present (a copy of the generated pair), the same generation invocation as above against that path, askpass available and standard input closed: exit **1**, output `<path> already exists.` then `Overwrite (y/n)? `, the existing file **byte-unchanged** (checksum compared before and after), and the askpass script never invoked for the question. A negative result would have been the file replaced, or the run hanging.

**What this does not cover:** one build; **no manual page read documents the no-terminal behaviour of this prompt.** [Section 13](../Keys.md#13-unresolved) item 4 carries it. **This is also the one observation standing behind Section 1's *standard input closed turns a would-be question into a failed read rather than a hang*** — the question printed and the process exited rather than waiting.

## Observed — the generator's presence on this system (Keys [G6](../Keys.md#63-the-generator-probed-never-assumed-row-248))

`which ssh-keygen` returned `/usr/bin/ssh-keygen` with nothing installed by this project. Covers this Mac alone.

## Documented — the OpenSSH **Client** is a preinstalled Feature on Demand on Windows, and the **Server** is not (Keys [G6](../Keys.md#63-the-generator-probed-never-assumed-row-248))

**Read by the architecture runner on 2026-08-20**, on the Creator's challenge to Fact 39's Windows half, and it **overturns the reading that reached them as an escalation.**

**Address:** `https://learn.microsoft.com/en-us/windows-hardware/manufacture/desktop/features-on-demand-non-language-fod` — *Available features on demand*, `ms.date` **2026-02-20**, updated **2026-02-23**, monikers `windows-10` and `windows-11`.

The page defines its own first section: *"**Preinstalled FODs** — The following Features on Demand come preinstalled in a Windows image. Don't remove these FODs from the Windows image"*. **The OpenSSH client is listed inside that section:**

> ### OpenSSH client
> The OpenSSH client Feature on Demand enables the use of OpenSSH on a Windows PC.
> **Feature**: OpenSSH Client
> - **Capability name**: OpenSSH.Client~~~~0.0.1.0
> - **Availability**: Windows 10, version 1709 and later

**The OpenSSH Server is listed in a different section of the same page** — *"FODs that are not preinstalled"* — carrying *"**Recommendation:** Don't include this Feature on Demand on your image."*

**`ssh-keygen.exe` ships in the client.** Microsoft's own overview page lists the commands *"OpenSSH for Windows has the following commands built in"* and puts `ssh-keygen` among them beside `ssh`, with `sshd` named separately as *"the SSH server component"*.

**The contradiction between two Microsoft pages, recorded rather than smoothed.** The overview page — `https://learn.microsoft.com/en-us/windows-server/administration/openssh/openssh_overview`, `ms.date` **2025-02-20**, updated **2025-05-01**, re-read 2026-08-20 and still carrying the table verbatim — gives *"Windows 10 build 1809 + | Not installed, install and enable using optional features"* under a column headed **"SSH install state"**, with **no Windows 11 row at all**. That table does not distinguish the client from the server, and its Windows Server 2025 row reads *"Installed but not enabled"* — a state only the server service has. **The newer and more specific page is the one relied on**, and the older table is read as being about OpenSSH the server feature rather than about `ssh-keygen`'s presence.

**What this does not cover, and it is why the probe stays.** The page is Microsoft's **manufacturing** documentation and speaks about what comes preinstalled **in a Windows image** — its own wording — so an OEM or enterprise image with the capability removed is possible, which the page's *"Don't remove these FODs"* implies rather than forbids. **No Windows machine has run it**, so nothing here is confirmed against a real installation, and the presence check remains the `Get-WindowsCapability` call Microsoft's own install page documents.

**The elevation half, in Microsoft's own words**, from the install page: under Prerequisites, *"An account that is a member of the built-in Administrators group."*, and immediately before the install cmdlets, *"Run PowerShell as an Administrator."* **The page never says the cmdlet itself requires an elevated session** — Fact 39 records that correction — and [G6](../Keys.md#63-the-generator-probed-never-assumed-row-248)'s rule that Smoozy never runs the install command is Smoozy's own, not this page's.

## G6 — the Linux half

**Direct publications, read 2026-08-23, and this entry previously carried the words with no address anywhere in `Preparation/` for any of the three.** The claim is [Facts](../Facts.md)' **Fact 39** as **corrected 2026-08-21**, with [The manuals](../The%20manuals%20—%20keys,%20secrets,%20and%20what%20cannot%20be%20told%20apart.md)'s Linux section: `/usr/bin/ssh-keygen` is a file of **`openssh-client`**, whose priority is `standard`, which Debian Policy defines as installed by default. **Each of the three publications was opened and the words read against what the project carried. All three match; nothing is corrected.**

- **The package database, for the binary's home.** Debian's own package pages, **`https://packages.debian.org/trixie/amd64/openssh-client/filelist`**, headed *"File list of package openssh-client in trixie of architecture amd64"*, read 2026-08-23: the list carries the line **`/usr/bin/ssh-keygen`**.
- **The packaging files, for the priority.** Debian: openssh's own `debian/control` as Debian publishes it, **`https://sources.debian.org/data/main/o/openssh/1%3A10.0p1-7%2Bdeb13u4/debian/control`** (browsable at `https://sources.debian.org/src/openssh/`), read 2026-08-23 — the source stanza's third line is **`Priority: standard`**. Ubuntu: **`https://git.launchpad.net/ubuntu/+source/openssh/plain/debian/control?h=applied/ubuntu/devel`**, read the same day — the source stanza again reads **`Priority: standard`**. **In both files `openssh-client` is the only binary package with no `Priority:` field of its own**, so it alone inherits `standard`; every other binary stanza in the file — `openssh-server`, `ssh`, `openssh-sftp-server`, `openssh-client-gssapi`, `openssh-server-gssapi`, `openssh-tests`, `ssh-askpass-gnome` and the two `-udeb`s — is overridden to **`Priority: optional`**. That is a sharper reading than *"with no override on that binary"*: the override list is what makes the inheritance deliberate rather than incidental.
- **Debian Policy, for what the priority means.** *Debian Policy Manual* **v4.7.4.1**, **§2.5 Priorities**, at **`https://www.debian.org/doc/debian-policy/ch-archive.html`**, read 2026-08-23. `standard`: *"These packages provide a reasonably small but not too limited character-mode system. This is what will be installed by default if the user doesn't select anything else. It doesn't include many large applications."* — the project's quotation is the first two of those three sentences and the third is added here for completeness; the page renders the apostrophes as typographic ones. `optional`: *"This is the default priority for the majority of the archive. Unless a package should be installed by default on standard Debian systems, it should have a priority of optional."*

**A negative result would have been** a `Priority:` field on the `openssh-client` stanza itself reading `optional`, or a Policy definition of `standard` that said nothing about default installation. Neither.

**[G6](../Keys.md#63-the-generator-probed-never-assumed-row-248) said *"On Linux no default claim exists (Fact 39)"* — the reading before that correction — and was repaired on 2026-08-23.** The **rule was never affected** — probe, never assume — but its stated ground was wrong: what the rule rests on is that an archive's default selection is not a guarantee about the machine in front of you, and a minimal container image or a trimmed appliance can lack the binary.

**Does not cover.** **Red Hat and Fedora**, whose documentation sites refused automated retrieval and remain unread; no verbatim quotation from either exists on this project, and no default claim of any kind is made for them. **No Linux computer exists on this project**, so none of this has been confirmed against a running installation — a priority is a claim about the archive, not about a disk. The Debian control file read is **trixie's `1:10.0p1-7+deb13u4`** and the Ubuntu one is the **development series**; older and newer suites were not checked, and neither was any derivative beyond Ubuntu. **One thing recorded rather than smoothed:** `packages.debian.org` is listed in this project as one of the three sites that refused automated retrieval, and on **2026-08-23 it answered normally** — the refusal was not reproduced. `Facts.md` is not edited from here.

## G9 — macOS's `ssh-keygen` does not create a missing parent directory

**Pointer.** [Facts](../Facts.md), **Fact 39**, observed, and **challenged on 2026-08-20 with the challenge failing**, so the clause stands on four observations rather than one. The claim put to it was that the tool creates `~/.ssh` at 0700 and prints *"Created directory"* when the target sits inside it. Posed four ways on the Creator's Mac — target inside a missing directory literally named `.ssh`, target two levels deep in missing directories, target one level deep in a differently-named directory, and the path typed at the tool's own interactive prompt rather than given with `-f`. **All four failed at exit 1 with `No such file or directory` and created nothing, and the phrase *"Created directory"* never appeared.**

**Does not cover:** the tool's **own default** path on an account with no `.ssh`, which is untested — the tool takes that default from the password database rather than from `HOME` (observed), and [G9](../Keys.md#65-the-parent-directory-and-the-modes-row-248) always supplies the path itself. One build; Windows and Linux unobserved.

## G9 — the mode the directory is created at

**Pointer.** [The manuals](../The%20manuals%20—%20keys,%20secrets,%20and%20what%20cannot%20be%20told%20apart.md), **Fact 33**, documented. `ssh(1)` FILES, `~/.ssh/`: *"the recommended permissions are read/write/execute for the user, and not accessible by others."*

**Does not cover:** any requirement — the page says *recommended*. That Smoozy creates the directory at 0700 is Smoozy's decision taken against that recommendation and carries no entry of its own.

## G10 — the modes the tool sets on the files it writes

**Pointer plus this document's own run.** [Facts](../Facts.md), **Fact 39**: macOS's `ssh-keygen` *"sets the file modes itself — observed 0600 private and 0644 public on 2026-08-20"*. The same values were observed in the probe table above and in the askpass generation run (`key_askpass` 0600, `key_askpass.pub` 0644).

**Does not cover:** Windows or Linux. **What permissions or ACLs the Windows `ssh-keygen.exe` sets is documented by Microsoft nowhere and is untested** (Fact 39's unresolved).

## G11 — the comment's documented default form

**Direct publication.** `ssh-keygen(1)`, address and date as at Section 1, DESCRIPTION: *"The key comment may be useful to help identify the key. The comment is initialized to “user@host” when the key is created, but can be changed using the -c option."*

**Does not cover:** how either half of `user@host` is obtained, which is Fact 12's entry below. **That Smoozy adopts the shape as a pre-fill is Smoozy's own choice** and carries no entry.

## G11 and N2 — a computer has more than one name, and the login account name is not one thing either

**Pointer.** [The computer itself — its name, its identity, and its clock](../The%20computer%20itself%20—%20its%20name,%20its%20identity,%20and%20its%20clock.md), **Fact 12**. macOS has three names by Apple's own `scutil(8)` — `ComputerName`, `LocalHostName`, `HostName` — and on the observed machine `HostName` was unset, so `gethostname(3)` falls through to a **fourth** string that is neither of the two actually set. Windows has the NetBIOS-era name derived from the DNS host name by re-encoding and truncation, and **two Microsoft pages contradict each other** on whether a name may exceed 15 characters. Linux has systemd's pretty, static and transient forms. **The login account name:** on the observed Mac `id -un` and `logname` **disagreed**, `getlogin()` returning `root` at effective uid 501, so a program reading the account name that way can get the wrong account.

**Does not cover, and [N2](../Keys.md#7-the-later-reading-of-is-this-name-the-default-one-row-249) is written so that it need not:** **steadiness across a reinstall or a hardware change is documented by nobody, on any of the three** — that fact's largest unresolved cluster — and the rename clause is documentation-only everywhere, macOS included, since observing it would have meant renaming the Creator's computer. [N1](../Keys.md#7-the-later-reading-of-is-this-name-the-default-one-row-249)'s recorded bit makes that gap non-load-bearing: nothing re-derives yesterday's default.

---

# Section 8 — the remembered passphrase (Rows 244, 250, 315)

## 8.1 — the store on each system

**Pointer.** [The manuals](../The%20manuals%20—%20keys,%20secrets,%20and%20what%20cannot%20be%20told%20apart.md), **Fact 38**, documented. [Keys](../Keys.md#81-the-citation-clause-by-clause-row-244) reaches these through `Security and licence` §3.1 to §3.3 and restates none of them; the world-side ground is this fact.

- **macOS.** Apple's TN3137: *"macOS has two keychain implementations: File-based keychain / Data protection keychain"* and *"The SecItem API can target either implementation. **It defaults to targeting the file-based keychain.**"* Reached through `SecItemAdd`, `SecItemCopyMatching`, `SecItemUpdate`, `SecItemDelete`. Access on the file-based keychain is governed by ACLs.
- **Windows.** Credential Manager through `CredWriteW` / `CredReadW` / `CredDeleteW` in `Advapi32.dll`. `CREDENTIAL`: *"**CRED_TYPE_GENERIC** … The credential will be stored securely but has no other significant characteristics."* and *"**CRED_PERSIST_LOCAL_MACHINE** — The credential persists for all subsequent logon sessions on this same computer."* The documented ceiling is Microsoft's own sentence: *"These credentials are stored on the hard disk drive and protected by using the Data Protection Application Programming Interface (DPAPI). **Any program running as that user will be able to access credentials in this store.**"*
- **Linux.** The Secret Service API on the session D-Bus at `/org/freedesktop/secrets`, through GNOME's libsecret, which is *"a client, not a store"*. The specification is a **draft** and declines the two guarantees that matter: *"**This specification does not mandate the use of master passwords to lock a collection of secrets.**"* and *"**This specification does not mandate any form of access control.**"*, with Chapter 11 reading in its entirety *"[TODO: complete]"*.

**Does not cover:** Windows and Linux behaviour, neither of which has ever been run on this project. **The single load-bearing Windows sentence above comes from an archived page** — `hh994565`, *Cached and Stored Credentials Technical Overview*, `is_archived: true`, `ms.date 2016-08-31` — and the DPAPI page beside it, `ms995355`, *Windows Data Protection*, is dated **October 2001** and is also archived. **[Keys](../Keys.md#13-unresolved) item 2 named one archived page and there are two; it names both from 2026-08-23**, with the second identified as where `Security and licence` §3.2's DPAPI half comes from. Nothing read carries a presence statement for `Advapi32.dll` of the kind `Crypt32.dll` has.

## 8.1 — the recorded disagreement inside Fact 38

**Pointer.** [The manuals](../The%20manuals%20—%20keys,%20secrets,%20and%20what%20cannot%20be%20told%20apart.md) and [Facts](../Facts.md), **Fact 38**, observed on macOS: a keychain write that returned a **missing-entitlement error on the first run returned success on the same day's repeat**. That behaviour is **not established**, both runs stand side by side, and three claims that leaned on the first were cut.

**Does not cover:** which of the two is the machine's real behaviour. [Keys](../Keys.md#13-unresolved) item 14 carries it forward as ground that Row 244's citation rests on, disagreement included.

## S4 and S5 — what a local load separates and a connection does not

**Pointer.** [The manuals](../The%20manuals%20—%20keys,%20secrets,%20and%20what%20cannot%20be%20told%20apart.md), **Fact 37**, observed. Locally: a wrong passphrase gives `incorrect passphrase supplied to decrypt private key`; malformed, truncated, empty and public-as-private all give `invalid format`; too-open permissions give `bad permissions` and pre-empt the passphrase entirely. Over a connection at default log level: **a wrong passphrase, an empty passphrase, no passphrase available, a key not authorised and a wrong account name all collapse into `Permission denied (publickey).` at exit 255**, separating only at `-vv`.

**Does not cover:** the message strings as a contract — no manual documents any of them. Raising to `-vv` does separate them, on one build, and those levels print key paths and identity material.

## S5 — even the local signal is not proof

**Pointer.** [The manuals](../The%20manuals%20—%20keys,%20secrets,%20and%20what%20cannot%20be%20told%20apart.md), **Fact 37**, observed: the **correct** passphrase against a key with one flipped bit at the first or last byte of its encrypted section produced the identical `incorrect passphrase` message, and a flip at one interior offset produced **exit 0 with output identical to the intact key**.

**Does not cover:** how often either case arises, which nothing bounds. The underlying question was closed from OpenSSH's own publication — `PROTOCOL.key`, read 2026-08-20 at `https://raw.githubusercontent.com/openssh/openssh-portable/master/PROTOCOL.key`: *"Before the key is encrypted, a random integer is assigned to both checkint fields so successful decryption can be quickly checked by verifying that both checkint fields hold the same value."*, and **there is no message authentication code over the encrypted blob and the format names none** — so a wrong passphrase and a corruption landing on a checkint field are the same event to the decoder. [S5](../Keys.md#83-a-passphrase-that-stops-working-row-250) accepts the cost as one re-entry.

## S6 — nothing retries, and why the tool's own retrying matters

**Pointer.** [The manuals](../The%20manuals%20—%20keys,%20secrets,%20and%20what%20cannot%20be%20told%20apart.md), **Fact 37**. An `ssh-add` fed a wrong passphrase by an askpass helper made **59 askpass invocations in 12 seconds and was still running when killed**, with no diagnostic; `ssh-add(1)` documents no retry limit. **Closed from the source, 2026-08-21**: `https://raw.githubusercontent.com/openssh/openssh-portable/master/ssh-add.c`, read 2026-08-20 — the passphrase loop in `add_file()` is `for (;;)` with **no counter**, exiting only on an empty passphrase or a successful parse, so **the retry is unbounded**.

**Does not cover:** any other tool. `ssh` itself made three and stopped, with the source's own narrowing that *"the keyword was never varied, so the two threes are not established as connected"*. `ssh-add` is invoked nowhere in this document.

## S6 — the source penalty that corrupts the diagnosis

**Pointer.** [The manuals](../The%20manuals%20—%20keys,%20secrets,%20and%20what%20cannot%20be%20told%20apart.md), **Fact 37**, observed: with default `PerSourcePenalties`, after an **unrecorded** number of failed authentications from one source address, every subsequent client attempt — **including ones with the correct passphrase** — failed with `kex_exchange_identification: Connection closed by remote host`, exit 255, **which reads as a server-side or network fault**. The server side is documented in `sshd_config(5)`; the client-side symptom is not.

**Does not cover:** the threshold, the penalty's duration, or the server's log level — the observation *"cannot be re-induced from its own description"*. Whether a penalty is separable from an ordinary immediate close was not tested and the two observations sit in tension.

## S8 to S12 — a computer with nowhere to remember

**Pointer.** [The manuals](../The%20manuals%20—%20keys,%20secrets,%20and%20what%20cannot%20be%20told%20apart.md), **Fact 38**, Linux, documentation alone: **there is no single answer**, availability depends on which desktop is installed, the draft mandates no access control, and it is the only one of the three with a documented memory-only collection.

**Does not cover:** which error a Secret Service client receives when no provider is running, which `Security and licence` §14 item 7 carries as narrowed and open. **Linux was never observed on this project.** [S8](../Keys.md#84-a-protected-key-on-a-computer-with-nowhere-to-remember-row-315)'s *store standing* value and the once-only notice are `Security and licence` §3.3's decisions and carry no entry.

---

# Section 9 — ownership, delete, remove and the warning (Rows 251, 247, 254, 252)

## W1, W3 and W4 — canonical path resolution

**ASSUMPTION — no source in this project establishes it.**

[W1](../Keys.md#91-ownership-is-a-derivation-from-the-canonical-path-row-251) rests on the platform's own full path resolution — *"symbolic links followed, `.` and `..` collapsed, the same resolution applied to both sides"* — being available through the platform file adapter on all three systems, and [W4](../Keys.md#93-the-indirect-routes-row-251) rests on a symbolic link inside the location resolving to its target outside it. **No fact in the register asks this, no input document answers it, and no companion under `Sources/` cites it.** Documents searched: `Facts.md`, all three input documents, `Preparation/Architecture/The user's own computer — where a download lands, what it will take, what it will show, and how the app replaces itself.md`, `Desktop app and delivery.md` and its companion, `File Browser.md` and its companion, and `Server Setup.sources.md` — the last of which cites `chown`'s symbolic-link handling **on a rented Linux machine**, which is a different question on a different computer.

**The assumption covers those two things and no longer a third.** It read as three — links, `.` and `..` — with a **case-only difference** silently inside *the same location written two ways*. **That is not an assumption and it is not established either way; it is a stated question, and it is the next entry.**

**Breaks:** [W1](../Keys.md#91-ownership-is-a-derivation-from-the-canonical-path-row-251)'s managed-or-borrowed derivation, and with it which branch a removal takes — the wrong branch being the unrecoverable one, destroying a file Smoozy promised only to borrow. **Holds:** the derivation's shape, which compares two paths under one resolution whatever that resolution turns out to be, and the confirmation naming files by full path before either act.

## W1a — what a canonical resolution does with a case-only difference

**A STATED QUESTION, established by nothing this project has read, and answered here in neither direction.**

**The question.** On macOS and Windows, a case-only difference is the commonest way of writing one path two ways — which is [W3](../Keys.md#92-the-pointed-at-file-that-lies-in-the-location-row-251)'s own phrase, *"The same location written two ways"*. **Whether any of the three platforms' canonical path resolution answers such a difference — folding it, preserving it, or answering per volume — is stated by nothing read on this project.** The same search as the entry above was made and found nothing: no fact asks it, no input document answers it, and the two publications that touch path resolution anywhere in this file (`ssh(1)`'s FILES entry and the Windows `%USERPROFILE%` note) say nothing about case at all. **The nearest thing the project holds is not an answer:** `Desktop app and delivery` F10a's third bullet supplies *reading a name back as the filesystem stores it*, and its §7 C10 puts *is this name already taken* to the filesystem as an act rather than modelling it — **both are the shape of asking, not the answer**, which is why [W1a](../Keys.md#91-ownership-is-a-derivation-from-the-canonical-path-row-251) asks rather than deciding.

**Breaks, and it breaks in two opposite directions, which is why neither may be guessed.**

- **If the comparison ends up byte-for-byte** where the volume does not distinguish case: a key genuinely sitting in the usual key location reads **borrowed**, against the spec's *whatever sits in the operating system's usual key location*, *regardless of who created them* — the user is offered *Remove from Smoozy* for a key they expect to be able to delete.
- **If it ends up case-folding** where the volume does distinguish case: a **borrowed** key reads managed, and Delete destroys a file Smoozy promised only to borrow — Row 251's unrecoverable branch, the one the row exists to prevent.

**Holds:** the shape of [W1a](../Keys.md#91-ownership-is-a-derivation-from-the-canonical-path-row-251), which asks the receiving filesystem rather than modelling a platform, and is therefore written the same way whatever the answer turns out to be; and the confirmation naming files by full path before either act.

**What would settle it:** two paths differing only in case put through the same resolution the adapter would use, on a case-insensitive volume and on a case-sensitive one. The macOS half needs only the machine this project already has; the Windows half needs the installation [Section 13](../Keys.md#13-unresolved) item 8 already asks for, and Linux the one item 2 already names.

## W4 — a hard link is invisible to path resolution

**ASSUMPTION — no source in this project establishes it**, on the same search as the entry above. [Keys](../Keys.md#93-the-indirect-routes-row-251) states it and immediately declines to defend it: *"that is the filesystem's own semantics, outside what Smoozy can see, and the confirmation names paths rather than promising the bytes are unrecoverable."*

**Breaks:** nothing this document promises — the promise is deliberately written to the paths named rather than to the bytes. **Holds:** the confirmation's wording, which is what carries the limit to the user.

## W5 — the companion naming rule

**Pointer.** [The manuals](../The%20manuals%20—%20keys,%20secrets,%20and%20what%20cannot%20be%20told%20apart.md), **Fact 36**, documented in three places and confirmed by observation. `ssh-keygen(1)` DESCRIPTION: *"The public key is stored in a file with the same name but “.pub” appended."* `ssh_config(5)` `IdentityFile`: *"ssh(1) will try to load certificate information from the filename obtained by appending -cert.pub to the path of a specified IdentityFile."* `ssh-keygen(1)` CERTIFICATES: *"The resultant certificate will be placed in /path/to/user_key-cert.pub."* — the **input** to that example being `user_key.pub`, so **the `.pub` is dropped rather than doubled**. Observed on this build: `ssh-keygen -s <tmp>/ca -I testid -n testuser <tmp>/plain_rsa.pub` produced `plain_rsa-cert.pub`.

**Does not cover:** a private key file carrying a suffix of its own — whether `mykey.pem` yields `mykey.pem.pub` is implied by plain string appending and **exercised by no example in the manual**. Nor whether the `.pub` is required for authentication when the private key is present. The generation of `<tmp>/ca` is recorded nowhere and its key type is not recoverable.

## W7 — why copy does not reconstruct a missing public half

**Pointer.** [The manuals](../The%20manuals%20—%20keys,%20secrets,%20and%20what%20cannot%20be%20told%20apart.md), **Fact 35**. `ssh-keygen(1)`, `-y`: *"This option will read a private OpenSSH format file and print an OpenSSH public key to stdout."*, and observed: without `-P` the same command **prompts even with standard input closed and no terminal** — `ssh-keygen -y -f <tmp>/locked </dev/null` printed `Enter passphrase for "<tmp>/locked": ` before failing.

**Does not cover:** the same command's behaviour under the askpass-disabling construction, which is Section 1's unresolved entry above. [W7](../Keys.md#95-the-copy-half-row-247)'s rule does not depend on it: the control is unavailable rather than attempted.

---

# What nothing in this project establishes, gathered

Each of these is stated in [Keys](../Keys.md) with its label, and each is here because no source for it exists anywhere under `Preparation/`.

1. **CLEARED 2026-08-23 by running it again.** It read: *the `SSH_ASKPASS_REQUIRE=never` matrix against `ssh-keygen -y`* — the four-way run Section 1 and [Section 13](../Keys.md#13-unresolved) item 3 describe — *no transcript anywhere*. There is one now, at the Section 1 entry above: **a new record on `OpenSSH_10.3p1` / macOS 26.6.2, not a recovered one**, reproducing every clause the document claimed and adding one it did not have.
2. **CLEARED 2026-08-23 by running it again.** It read: *the PEM and PKCS8 fingerprint refusal, and the `-P ''` probe against those four keys* — [Section 13](../Keys.md#13-unresolved) item 6's runs — *no transcript anywhere, and the companion that would otherwise hold them names this document as their owner*. Both are recorded at the Section 4 entry above, **new records on the same later build**, reproducing every result the document claimed.
3. **That no SHA-256 fingerprint collision exists.** No first-party sentence says so; [Open Threads](../../Groundwork/Open%20Threads.md) item 8a carries it as the Creator's.
4. **Canonical path resolution on the user's own computer, and hard-link invisibility** — [W1](../Keys.md#91-ownership-is-a-derivation-from-the-canonical-path-row-251), [W3](../Keys.md#92-the-pointed-at-file-that-lies-in-the-location-row-251), [W4](../Keys.md#93-the-indirect-routes-row-251). No fact asks it. **And, separately from the assumption, what such a resolution does with a case-only difference** — [W1a](../Keys.md#91-ownership-is-a-derivation-from-the-canonical-path-row-251), carried above as a stated question with both failure directions, because a guess in either direction is a wrong app.
5. **Every Windows half of this document.** The key location, the second location's contents, the store, the generator's presence, the permission gate's form, the identity readings, and what `ssh-keygen.exe` accepts and sets — all documentation, none run.
6. **Every Linux half.** The key location, the store, the generator's presence — all documentation, none run, and Red Hat and Fedora unread entirely.
7. **How long a key check takes**, on any machine — [Section 13](../Keys.md#13-unresolved) item 16. No key check has been timed anywhere on this project. *(Narrowed 2026-09-02: what **was** timed that day is the **type probe's generations** — Section 2's timing entry, three runs of each type on this Mac — which is a different act on keys that do not yet exist. The check itself, over a key already on disk, is still untimed everywhere, and no figure in this file may be read across to it.)*
8. **Which modes short of 0644 the client refuses on macOS and Linux** — [U4](../Keys.md#42-usable-composed-row-245) clause 5, whose rule about group and other is the documented recommendation applied to every unobserved mode. Carried at [Section 13](../Keys.md#13-unresolved) item 8 rather than as an item of its own.
9. **Where the user's home directory comes from, and where Smoozy's own temporary area is** — [L1a](../Keys.md#3-the-key-locations-and-the-one-place-premise-row-242) and [T2](../Keys.md#2-the-supported-types-the-offered-set-and-the-default-row-241). Neither is a fact about the world so much as a reading no document supplies; both are routed at [Section 13](../Keys.md#13-unresolved) item 20. **What *is* a fact about the world, and is cited above, is that reading `$HOME` is not established to reproduce what the tools do** (the L1 entry).
