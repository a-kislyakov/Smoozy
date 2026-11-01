# Sources — Servers (Architecture)

*Where every outside-world fact [Preparation/Architecture/Servers.md](../Servers.md) leans on came from. The document carries each claim's label in place; this file carries the citation — the owning document or publication, the exact words, the address and date where there is one, and what each source does **not** cover. **Decisions Smoozy made get no entry here**, per [Conventions](../../Groundwork/Conventions.md), *Outside facts are cited in a companion sources file*.*

**Three shapes, and there is no fourth.**

1. **A pointer to an input document of this stage** — one of the frozen observation documents beside `Servers.md`. The entry names that document, quotes the fact in that document's own words, and states the observation's own limit. This is the commonest shape here, because this document leans almost entirely on rented-machine observations across several sellers.
2. **A direct publication** — a manual, a protocol document, a vendor's own reference, a release note. The entry carries the exact words, the address and the date read.
3. **No source in this project** — recorded as an assumption or as unresolved, with what breaks if it is wrong and which documents were looked in. **This is a permitted outcome and there are several below.**

**The reach of every rented-machine entry, stated once.** [Servers](../Servers.md) §16 item 15 says it of itself and every entry below inherits it: *"Every claim about a rented machine in this document is one to six boxes on one to three days, across up to three sellers"*, and the input documents' own instruction is *"Read it as *this is what that box did*, never as *this is what a rented machine does*."* Individual entries add their own limits on top of that; none of them relaxes it. **That figure was widened from *one or two boxes on one or two days* on 2026-08-23**, the old wording having understated the nine-rental, three-seller batch of 2026-08-22 that several entries below rest on.

**What the input documents are, and their dates.** [The rented machine](../The%20rented%20machine.md) (one box, 2026-08-13/14); [Batch A](../Batch%20A%20—%20the%20shape%20throwaway.md) (2026-08-13); [Two more boxes](../Two%20more%20boxes%20—%20the%20volume,%20the%20full%20disk,%20and%20a%20day%20left%20running.md) (2026-08-16); [Six machines](../Six%20machines%20—%20the%20container,%20the%20webhook%20and%20the%20ready%20action.md) (six rentals, 2026-08-15/16); [One box — the Telegram capture](../One%20box%20—%20the%20Telegram%20capture%20and%20the%20port%20that%20cannot%20be%20chosen.md) (2026-08-16); [One box with Blender on it](../One%20box%20with%20Blender%20on%20it%20—%20the%20reading,%20the%20listing,%20and%20the%20names%20that%20do%20not%20come%20back.md) (2026-08-17); [The host key](../The%20host%20key%20—%20obtained%20without%20authenticating,%20and%20whose%20it%20turns%20out%20to%20be.md) (2026-08-17); [The manuals](../The%20manuals%20—%20keys,%20secrets,%20and%20what%20cannot%20be%20told%20apart.md) (2026-08-16, documentation and one Mac); [The last two boxes](../The%20last%20two%20boxes%20—%20the%20hook%20that%20cannot%20be%20read%20back,%20the%20full%20disk,%20and%20the%20unit%20that%20kills%20a%20render.md) (2026-08-18); [The multi-provider batch](../The%20multi-provider%20batch%20—%20the%20surface%20that%20did%20not%20vary,%20the%20Debian%20route%20that%20exists,%20and%20the%20cards%20a%20slice%20cannot%20see.md) (nine rentals, three sellers, 2026-08-22); [The Pile 3 batch](../The%20Pile%203%20batch%20—%20a%20full%20mount%20under%20a%20running%20supervisor,%20an%20interrupted%20heavy%20frame,%20and%20an%20hour%20of%20silence.md) (one rental, 2026-08-25) — **added 2026-08-29**, this list having been written before [1.7](#17-the-providers-address-mapping-survives-an-hour-of-complete-silence)'s observation entered the document.

---

# 1. What binds this before anything is decided

## 1.1 Fact 57 — how a connection's ending reaches the side still holding it

**Pointer.** [Six machines](../Six%20machines%20—%20the%20container,%20the%20webhook%20and%20the%20ready%20action.md), Fact 57. Label: **observed**. Used by the head bullet 1, [K2](../Servers.md#21-the-break-reporting-mechanism), [K3](../Servers.md#21-the-break-reporting-mechanism), [K5](../Servers.md#21-the-break-reporting-mechanism), [M2](../Servers.md#41-the-mark-taken-rather-than-decided), and §16 item 12.

The document's own table, in its own words:

| | | |
|---|---|---|
| **Powered off from the provider's console** | **told** | **same second** |
| The caller's line pulled, keepalives on | detected | 56 s |
| The caller's line pulled, keepalives off | **not detected** | **silent 21 m 45 s** |

> "A provider's stop is an orderly shutdown, not a power cut — the machine's own service was alive long enough to close its sockets politely, so **both a session with keepalives and one without learned in the same second and the setting made no difference.**"

> "So the danger is never the machine going away; it is the caller's own connection failing. Only a broken path needs keepalives, and without them a caller can sit for twenty minutes believing a dead connection is a slow one."

Two side-findings from the same fact, both used here:

> "A ten-minute silent gap made a connection **unrecoverable even after the path was restored** — almost certainly because the provider's address translation forgets faster than the protocol does, though **the actual timeout was not established**."

> "And **one provider answers no ping at all** while its connection works perfectly — so **any ping-based health check would report it dead.**"

**Not covered, in that document's own words:** *"the genuinely abrupt case. A real power cut would look like the pulled-line row, and **no rented machine can pose it.**"* And §16 item 12's second half, which this document carries: **every observation ran through a provider's proxy**, so what was seen when a machine went away is what the proxy did. The ten-minute unrecoverability has **no measured timeout** — [K5](../Servers.md#21-the-break-reporting-mechanism) rests on the observation and not on a number.

**One further reading of the same event, from a second document.** [One box — the Telegram capture](../One%20box%20—%20the%20Telegram%20capture%20and%20the%20port%20that%20cannot%20be%20chosen.md): three holders of one connection *"All three ended in the same second, 13:09:30"*, each with `Connection to ssh7.vast.ai closed by remote host.` and exit 255 — *"the close was announced, so every holder learned of it by itself and immediately, without needing to use the connection first."* This is the announced-ending half on a second box and a second day. **Not covered:** the same proxy limit.

## 1.2 The machine had not learnt inside the watched window, and the manual and kernel defaults bound it

**Pointer.** [Six machines](../Six%20machines%20—%20the%20container,%20the%20webhook%20and%20the%20ready%20action.md), Fact 57, the correction of 2026-08-21. Label: **observed**, with the ceiling **documented**. Used by head bullet 1.

The input document's own sentence: *"**The machine had not learnt within the 5 minutes 10 seconds it was watched** — it held the socket open with the job still running."* Its correction note — *"this said *never learns at all*, which the manual of the observed software contradicts"* — is the same correction `Servers.md` carries, and the window it names is its own: *"The window in `findings-hyperstack.md` runs from the blackhole at 21:07:40 to the socket inspection at 21:12:50. **Nobody watched for two hours, so the run measured what the defaults already predict.**"*

**Not covered:** anything past 5 min 10 s. The observation measured nothing about the two-hour ceiling, and `Servers.md`'s conclusion — *"two hours, or fifteen minutes, is no use to a caller either"* — does not need it to.

## 1.3 `sshd_config(5)` — `TCPKeepAlive`'s default, and `ClientAliveInterval`'s

**Direct publication.** OpenSSH's own `sshd_config(5)`, the manual page shipped on the Creator's Mac, **macOS 26.5 page dated 2025-10-04**, read **2026-08-20**. Label: **documented**, first-party. Used by head bullets 1 and 2, and quoted at second hand from [Six machines](../Six%20machines%20—%20the%20container,%20the%20webhook%20and%20the%20ready%20action.md) Fact 57 and [The rented machine](../The%20rented%20machine.md) §5, which are where `Servers.md` takes it from.

> "The default is yes (to send TCP keepalive messages), and the server will notice if the network goes down or the client host crashes."
> — `TCPKeepAlive`

> "The default is 0, indicating that these messages will not be sent to the client."
> — `ClientAliveInterval`

**On the address.** The page has no web address in this project; the address is the file on the machine. [Server protocol and transfers](../Sources/Server%20protocol%20and%20transfers.sources.md) records where the two copies live on that machine — `/Applications/Xcode.app/…/MacOSX.sdk/usr/share/man/man5/sshd_config.5`, dated **October 4, 2025**, which is the date this document records, and the system copy `/usr/share/man/man5/sshd_config.5`, dated **March 28, 2026**, carrying the quoted paragraphs byte-identically for `MaxSessions`. **Neither reading was re-taken for this file.**

**Not covered:** what any given rental's own configuration sets. The manual states a default, and a default is a setting somebody can change — which is exactly why the box's own values were read separately ([1.4](#14-the-box-own-keepalive-settings-read-rather-than-assumed)).

## 1.4 The box's own keepalive settings, read rather than assumed

**Pointer.** [The rented machine](../The%20rented%20machine.md) §5, as `Servers.md` head bullet 2 cites it, with the correction of 2026-08-20 in that document. Label: **observed**. Used by head bullet 2 and [K2](../Servers.md#21-the-break-reporting-mechanism).

What was read on the box: `clientaliveinterval 0`, `clientalivecountmax 3`, `tcpkeepalive yes`, and the box's own configuration file carrying `#ClientAliveInterval 0` **commented out** — so the value in force is OpenSSH's shipped default rather than the provider's choice. The document's own conclusion, quoted in the body: *"So keeping an idle connection open is still the caller's job, and the reason is the one that is off rather than a machine that sends nothing at all."*

**Not covered:** one box, one image, one provider. Nothing here says a second rental's `sshd` is configured the same way, and the design does not depend on it — [K2](../Servers.md#21-the-break-reporting-mechanism)'s caller-side keepalive is what covers the case whatever the machine sends.

## 1.5 The kernel's own keepalive and retry ceilings

**Direct publication. The address gap is closed 2026-08-23 by reading the page itself.** Label: **documented**, first-party. Used by head bullet 1's ceiling argument.

**The publication.** The Linux kernel's own networking parameter reference, `ip-sysctl`, `https://docs.kernel.org/networking/ip-sysctl.html`, read **2026-08-23**. [Six machines](../Six%20machines%20—%20the%20container,%20the%20webhook%20and%20the%20ready%20action.md) Fact 57 names the publication as *"kernel.org `ip-sysctl`"* and its own date read as **2026-08-20**; that reading recorded no address, and this entry supplies one.

**Read against the words the body already carried, and all four hold exactly.** Quoted from the page, each under its own parameter:

> "How often TCP sends out keepalive messages when keepalive is enabled. Default: 2hours."
> — `tcp_keepalive_time`

> "How many keepalive probes TCP sends out, until it decides that the connection is broken. Default value: 9."
> — `tcp_keepalive_probes`

> "How frequently the probes are send out. Multiplied by tcp_keepalive_probes it is time to kill not responding connection, after probes started. Default value: 75sec i.e. connection will be aborted after ~11 minutes of retries."
> — `tcp_keepalive_intvl`

> "This value influences the timeout of an alive TCP connection, when RTO retransmissions remain unacknowledged. Given a value of N, a hypothetical TCP connection following exponential backoff with an initial RTO of TCP_RTO_MIN would retransmit N times before killing the connection at the (N+1)th RTO. The default value of 15 yields a hypothetical timeout of 924.6 seconds and is a lower bound for the effective timeout."
> — `tcp_retries2`

**Nothing the body carries is contradicted.** *"Default: 2hours"*, `tcp_keepalive_probes` 9 at `tcp_keepalive_intvl` 75 s, *"connection will be aborted after ~11 minutes of retries"* and *"yields a hypothetical timeout of 924.6 seconds and is a lower bound"* are all the page's own words. **One attribution is worth stating** because the body's sentence runs the four together: the *~11 minutes* clause belongs to `tcp_keepalive_intvl`, not to `tcp_retries2`, and the page's `tcp_retries2` sentence continues *"for the effective timeout"*.

**Not covered:** the kernel's documented defaults, which are what a distribution or an operator may change; nothing here was read off the observed box. And the page is a live document with no version stamp on the parameters quoted, which is why the date read sits with it. **Not exercised:** these are Linux kernel parameters and the machine this was read on is a Mac, so no run was available at no cost.

**What breaks if the quotation is wrong:** nothing built. Head bullet 1's own last sentence is *"Nothing below changes"* — the numbers are cited to say that the machine's own eventual notice is far too slow to be useful, and [K3](../Servers.md#21-the-break-reporting-mechanism)'s figures are derived from three constraints of which none is this one.

## 1.6 The container recycle — what it destroys, what comes back, and what the connection shows

**Pointer.** [Six machines](../Six%20machines%20—%20the%20container,%20the%20webhook%20and%20the%20ready%20action.md), *Recycle*. Label: **observed**. Used by head bullet 3, [H20](../Servers.md#84-the-installation-identity-over-all-of-it), [T5](../Servers.md#121-what-the-period-owes-a-transfer) and §16 item 5.

> "A Smoozy that reconnects and finds its address answering has no signal that its whole installation is gone. It must check for its own state, not for reachability"

with the recycle destroying everything Smoozy installed and returning on the same address and port in about **90 seconds**, against **15–20** for a reboot.

**Not covered:** one provider, one rental kind. The port half of the same finding is answered differently on a second box — [The last two boxes](../The%20last%20two%20boxes%20—%20the%20hook%20that%20cannot%20be%20read%20back,%20the%20full%20disk,%20and%20the%20unit%20that%20kills%20a%20render.md) §17 records a recycle on a virtual machine where *"The recycle changed the connection's port and not its address"* (15840 became 15820). **`Servers.md` head bullet 3 says *the same address and port*, which is the container observation; on the machine kind the port moved.** Nothing in this document keys on either, which is why the difference costs nothing here.

## 1.7 The provider's address mapping survives an hour of complete silence

**Pointer.** [The Pile 3 batch](../The%20Pile%203%20batch%20—%20a%20full%20mount%20under%20a%20running%20supervisor,%20an%20interrupted%20heavy%20frame,%20and%20an%20hour%20of%20silence.md) §4, **2026-08-25**. Label: **observed**. Used by [K5a](../Servers.md#21-the-break-reporting-mechanism), which is what gives [N6](../Servers.md#32-the-standing-repair-which-never-ends-while-the-app-is-open) a real address to build to. **Entry added 2026-08-29**: K5a was written after this file and its citation stood in the body alone.

> "the machine was touched by nothing at all for 3623 seconds — no connection, no keep-alive, no probe"

answering on the same host and the same port on the first packet, and the same machine — the marker written before the silence, the boot identifier unchanged, a process started before the silence still alive.

**Not covered, in the observation's own words:** *"It is one hour, on one provider, on one rental, and it sets no upper bound."* Nothing about a longer silence, nothing about a rental that stops and restarts, and nothing about §16 item 5's remaining half — whether a seller later hands the same address to a different machine.

---

# 2. Section 1.4 — the additional connection at a low session cap

## 2.1 `sshd_config(5)` — `MaxSessions` is per connection, may be 1, and defaults to 10

**Direct publication, cited at second hand.** OpenSSH's own `sshd_config(5)` on the Creator's Mac, **macOS 26.5 page dated October 4, 2025**, read **2026-08-17** for this document's use of it. Label: **documented**, first-party. Used by [1.4](../Servers.md#14-the-additional-connection-and-it-is-the-only-one-a-period-may-hold)'s constraint, [S14](../Servers.md#14-the-additional-connection-and-it-is-the-only-one-a-period-may-hold) item 1, [S15](../Servers.md#14-the-additional-connection-and-it-is-the-only-one-a-period-may-hold), and §16 item 16.

> "Specifies the maximum number of open shell, login or subsystem (e.g. sftp) sessions permitted per network connection.  Multiple sessions may be established by clients that support connection multiplexing.  Setting MaxSessions to 1 will effectively disable session multiplexing, whereas setting it to 0 will prevent all shell, login and subsystem sessions while still permitting forwarding.  The default is 10."

**This document does not own the citation.** `Servers.md` takes the cap from `Server protocol and transfers` [T2](../Server%20protocol%20and%20transfers.md#61-a-transfer-is-a-channel-and-the-channel-is-independent-of-the-command-channel), and that document's own companion, [Server protocol and transfers.sources.md](Server%20protocol%20and%20transfers.sources.md), carries the page, the two copies on that machine, and their differing page dates. The entry here exists so a builder reading `Servers.md` alone can find it.

**Not covered:** what any given rental sets. Four rentals on record observed a cap of **10**; a cap of 1 and a cap of 0 are configurations the manual documents rather than machines anybody rented.

## 2.2 Fact 93 and Fact 112 — the cap is per connection, and a second connection gets its own ten

**Pointer.** [One box with Blender on it](../One%20box%20with%20Blender%20on%20it%20—%20the%20reading,%20the%20listing,%20and%20the%20names%20that%20do%20not%20come%20back.md) §3.3, on a real rental, **2026-08-17**. Label: **observed**. Used by §16 item 16, which is what makes [S14](../Servers.md#14-the-additional-connection-and-it-is-the-only-one-a-period-may-hold)'s whole remedy stand on an observed footing rather than an assumed one.

> "The first ten succeeding in the same run is the control. **The cap is per connection** — a second connection gets its own ten."

The run itself, in the same section: the box's own dump gave `maxsessions 10`, fourteen concurrent sessions were opened on **one** multiplexed connection, and the record reads `sessions that succeeded: 10 of 14`. **§16 item 16 was repaired to those words on 2026-08-23**, having carried a one-sentence condensation of the whole passage stated as a quotation of the fact; nothing in either form went beyond what was observed and no answer moved.

The same section carries a trap this document inherits without restating: *"the refusal never says *too many sessions*. It says `Session open refused by peer` and then appends the actively misleading `Host key verification failed.` Anything matching on that text chases a host-key bug that is not there."*

**Not covered:** one machine, one seller, one day, at a cap of 10. **Nothing observed a machine at `MaxSessions 1` or `0` at all**, so [S14](../Servers.md#14-the-additional-connection-and-it-is-the-only-one-a-period-may-hold) and [S15](../Servers.md#14-the-additional-connection-and-it-is-the-only-one-a-period-may-hold) are written against the manual's documented configurations and not against a box.

## 2.3 Fact 25's sharing result — a second machine buys about 20%

**Pointer.** [Six machines](../Six%20machines%20—%20the%20container,%20the%20webhook%20and%20the%20ready%20action.md), Fact 25. Label: **observed**. Used by [S14](../Servers.md#14-the-additional-connection-and-it-is-the-only-one-a-period-may-hold) item 7 as the reason one additional connection is a cap rather than a pool.

> "One machine alone gave 13.9–15.2 MB/s; a second machine alone gave 14.7–15.0. **Both at once gave 16.5–18.3.** A whole second rented machine bought about **20%, not 100%**, and each machine's own throughput roughly halved."

> "So the limit is global — the home line — and not per server. A pool that scales with machine count measures the wrong thing."

**Not covered, in that document's own words:** *"one home line on one evening over Wi-Fi. Two of the three rentals were the same provider."*

---

# 3. Section 2 — holding the connection up

## 3.1 K3's second constraint — the `findings-connection.md` quotations struck, and what the constraint now rests on

**Struck 2026-08-23, and replaced by a pointer.** Used by [K3](../Servers.md#21-the-break-reporting-mechanism)'s second constraint.

**What was struck, recorded here so the deletion is not silent.** The body quoted a file by name and printed two verbatim round-trip strings:

> ~~`findings-connection.md`, 2026-08-15/16 — "round-trip min/avg/max/stddev = 43.692/74.688/109.417/27.773 ms" to the United Arab Emirates box and "75.249/107.895/163.903/36.218 ms" to Stuttgart~~

and, in its correction of 2026-08-21, a third string from the same named file:

> ~~"Connection setup is a flat 1.0–1.4 s to either Vast box and 1.9–4.1 s to Hyperstack, independent of file size."~~

**Why they were struck.** Every file under `Preparation/` was looked in. The string `43.692` occurred in `Servers.md` and nowhere else; so did `Connection setup is a flat`; so did `findings-connection`. **No document named `findings-connection.md` exists under `Preparation/`**, and [Six machines](../Six%20machines%20—%20the%20container,%20the%20webhook%20and%20the%20ready%20action.md) Fact 25 — the register's own home for that fact — carries throughput, concurrency and sharing figures and **no round-trip latency at all**. These are **observations**, not readings: no publication can be gone to for them, and this stage does not re-run a rental to recover a number. So they are struck rather than left standing on a trail a builder cannot open.

**What the constraint rests on instead — pointer.** [Six machines](../Six%20machines%20—%20the%20container,%20the%20webhook%20and%20the%20ready%20action.md), Fact 25, 2026-08-15/16. Label: **observed**.

> "most of the gain is hiding a flat 1.0–1.4 s connection setup rather than adding bandwidth."

**That is the register's own sentence for the same figure the struck quotation gave**, and it is the one half of the three strings that has a home in the project. It is used as what it is — **per-stream connection setup, which contains a round trip and is an upper bound on one** — and never as a round trip. The Hyperstack 1.9–4.1 s half is not in the project and is not carried.

**What the document loses.** The round-trip number, and with it the two-orders-of-magnitude margin the struck bullet claimed. **What it keeps:** the constraint itself, on a margin of more than ten times rather than a hundred — 1.0–1.4 s against a 15-second interval — which is the same conclusion on a narrower and sourced figure. **Nothing built rests on either**: no figure in K3 is derived from these numbers, and K3's own text says so — *"none is any one machine's measured cost."*

**Not covered, in Fact 25's own words:** *"one home line on one evening over Wi-Fi. Two of the three rentals were the same provider."*

## 3.2 The negotiated maximum write

**Pointer, to an answer document rather than to an observation document.** `Server protocol and transfers` [§1.1](../Server%20protocol%20and%20transfers.md#11-what-the-surface-is-made-of-and-it-is-two-mechanisms-rather-than-one), which records the reading: **observed on the caller's own end**, macOS 26.5, `OpenSSH_10.2p1, LibreSSL 3.3.6`, **2026-08-17** — the server reported *"max-packet 262,144, max-read 261,120, max-write 261,120, max-open-handles 1,048,571"*. Label: **observed**. Used by [K3](../Servers.md#21-the-break-reporting-mechanism)'s third constraint.

**Not covered, in that document's own words:** *"This was observed on the caller's end and not on any rented machine"*, which is its §12 item 3's standing ground.

## 3.3 Fact 111 — the chosen cadence produced no false break under a saturated link

**Pointer.** [The multi-provider batch](../The%20multi-provider%20batch%20—%20the%20surface%20that%20did%20not%20vary,%20the%20Debian%20route%20that%20exists,%20and%20the%20cards%20a%20slice%20cannot%20see.md), Test 20, machine **G** (Vast container, Norway), **2026-08-22**. Label: **observed**. Used by §16 item 11, which is where `Servers.md` records the answer.

> "Posed on machine **G** with the connection carrying **the interval `Servers` K3 chose — 15 seconds** — while a **419 430 400-byte** file was moved at the link's full speed in both directions, with a watcher checking every second whether the transport had been declared dead while bytes were still moving."

> "**Upload: 680.52 s.** **Download: 108.07 s.** **A second full download immediately after: 97.28 s.**" — "**`connection still active after all of it: True`**" — "**`false breaks reported while bytes were moving: NONE`**"

> "A negative would have been a break declared on a healthy connection under load … **It did not happen across eleven minutes of continuous saturated transfer.**"

**Not covered, and the body states it as the limit it is:** **the count was never exercised.** No break was declared, so the number of unanswered keepalives that declares one was never reached. **The interval is observed; the count of three is this document's choice** against [K3](../Servers.md#21-the-break-reporting-mechanism)'s three constraints. One box, one seller, one day.

---

# 4. Sections 3 and 4 — repair, and the restart's reconnect policy

## 4.1 The restart return spread — 9 to 90 seconds across three machines, and a recycle at about 90

**Pointer, through an answer document to its observations.** `Server Setup` [T6](../Server%20Setup.md#8-the-restart) and its §5, which state the constraint this document is bound by; the observations are [Six machines](../Six%20machines%20—%20the%20container,%20the%20webhook%20and%20the%20ready%20action.md) (Fact 80's eight measurements on three machines, and the *Recycle* finding at about 90 seconds). Label: **observed**. Used by [N4](../Servers.md#31-a-run-and-the-two-figures-in-it), [M4](../Servers.md#42-the-reconnect-policy-for-a-marked-period) and [M5](../Servers.md#42-the-reconnect-policy-for-a-marked-period).

`Server Setup`'s own words, which is the sentence `Servers.md` quotes: *"Return cost was measured at 9 to 90 seconds across three machines, and a container recycle at about 90 — anything budgeting a fixed wait is wrong on most machines."* Its §5 gives the count: *"Restart cost varied **9 to 90 seconds** across eight measurements on three machines, so **nothing here budgets a fixed wait**."*

**Not covered:** three machines, two sellers, one restart mechanism each. The spread is a **floor and a top**, and `Server Setup` T6 places a requirement on this document rather than a number: *"no figure is derived from any one machine's observed cost."* [N4](../Servers.md#31-a-run-and-the-two-figures-in-it)'s own third bullet is that requirement kept.

## 4.2 Fact 51 — a machine restarted over the connection Smoozy already holds

**Pointer.** [Batch A](../Batch%20A%20—%20the%20shape%20throwaway.md), Fact 51, **2026-08-13**. Label: **observed**. Used by [Section 4](../Servers.md#4-the-restart-server-setup-caused-row-32)'s constraint.

> "`systemctl reboot` issued over the connection Smoozy already held. The connection reported `Connection to 31.59.39.104 closed by remote host.` and the machine **answered again in about ten seconds**, with a new boot id. The connection could be re-established at the same address and port."

**Not covered:** one machine, one kind, one day, one restart. Ten seconds is a single measurement and is **not** what any figure in [4.2](../Servers.md#42-the-reconnect-policy-for-a-marked-period) is derived from — the spread of [4.1](#41-the-restart-return-spread--9-to-90-seconds-across-three-machines-and-a-recycle-at-about-90) is.

## 4.3 Q19's cadence argument — how soon a machine's own supervisor is back

**Pointer, through an answer document.** `Rented-server runtime` [K10](../Rented-server%20runtime.md#13-on-a-container) — the container hook *"ran on both container restarts — twelve seconds after the first request and five after the second"* — and its systemd half, from [Six machines](../Six%20machines%20—%20the%20container,%20the%20webhook%20and%20the%20ready%20action.md): *"It returned **6.71 seconds** into the new boot, as uid 1001, with nobody logged in, resuming from where it stopped rather than starting over"*, with Hyperstack reproducing it at *"six seconds after boot, four heartbeats before any login."* Label: **observed**. Used by [Q19](../Servers.md#10-the-first-connection-in-order-rows-36-and-35) to show that a reconnection arriving before the supervisor is back is the ordinary case rather than a corner.

**Not covered:** two container restarts on one box and one systemd return on each of two boxes. The numbers establish that the window exists and is short; [Q19](../Servers.md#10-the-first-connection-in-order-rows-36-and-35)'s re-take is bounded by settling rather than by any of them.

---

# 5. Section 6 — the store

## 5.1 The remembered host key carries no installation identity — Fact 83 as the reason nothing is keyed on it

**Pointer, to three documents, because the finding widened twice.** Label: **observed**. Used by [D11](../Servers.md#63-the-remembered-host-key-row-37), [Q10](../Servers.md#102-the-three-paths-and-the-two-reads-that-close-the-gap-between-them) and §16 item 5. The full citation is at [8.4](#84-fact-83--whose-host-key-it-is-and-what-an-unchanged-one-means) below; it is named here so [D11](../Servers.md#63-the-remembered-host-key-row-37) is not read as resting on nothing.

## 5.2 Nothing modifies the user's own SSH configuration

**No entry — this is Smoozy's decision.** `Security and licence` §7 owns it and [D1](../Servers.md#61-one-durable-store-and-one-row-per-server) cites it. Recorded here only so a later reader does not go looking for a source that should not exist.

---

# 6. Section 8 — the hardware reading

## 6.1 Fact 46 — the count trap

**Pointer.** [Six machines](../Six%20machines%20—%20the%20container,%20the%20webhook%20and%20the%20ready%20action.md), Fact 46, on **a two-card box with no Blender, no screen and no display server**, 2026-08-15/16. Label: **observed**. Used by [H5](../Servers.md#82-what-the-reading-is-made-of-and-every-part-of-it-is-a-trap-closed).

> "`nvidia-smi --query-gpu=count --format=csv,noheader` returns:
> ```
> 2
> 2
> ```
> **The value once per card.** A caller reading the first line is right by luck; one parsing the whole output as an integer gets `22`. **It passes every test ever written on a one-card machine.** Safe readings are `nvidia-smi -L | wc -l` or counting rows of any per-card query."

**Not covered:** one image, one driver, one card model, two cards. The trap is a property of that query's output shape on that build; nothing establishes it for a different vendor's tool, and Fact 3 (a card make other than NVIDIA) is unanswered.

## 6.2 Fact 46 — a card's identity, and the make as a keyable value

**Pointer.** Same fact and box. Label: **observed**. Used by [H6](../Servers.md#82-what-the-reading-is-made-of-and-every-part-of-it-is-a-trap-closed) and [H7](../Servers.md#82-what-the-reading-is-made-of-and-every-part-of-it-is-a-trap-closed).

> "Identity needs more than a name. Both cards are the same model, so **the UUID and bus location are what tell them apart** — a distinction that cannot arise with one card. `lspci` listed them **without a model name** (`NVIDIA Corporation Device [10de:2684]`), so on that box it gave a count and a PCI id, not an identity."

The input document corrects its own reading of that last clause on 2026-08-20, and **`Servers.md` [H6](../Servers.md#82-what-the-reading-is-made-of-and-every-part-of-it-is-a-trap-closed) carries the correction in place from 2026-08-23**, having until then quoted only the first half: *"That is this image's own ID database being older than the card, not the tool declining to name it"* — the PCI ID Repository gives vendor `10de` device `2684` as *"AD102 [GeForce RTX 4090]"* (`https://pci-ids.ucw.cz/read/PC/10de/2684`, read 2026-08-20), and `lspci(8)` documents the names as coming from a `pci.ids` file that goes stale. **The design does not change** — identity from `nvidia-smi`, presence from `lspci -d 10de:` — which is the direction [H6](../Servers.md#82-what-the-reading-is-made-of-and-every-part-of-it-is-a-trap-closed) and [H11](../Servers.md#82-what-the-reading-is-made-of-and-every-part-of-it-is-a-trap-closed) already take.

**Not covered:** two cards of one model on one box. **`Rendering` §8's requirement that the make be a value a table can be keyed on is a requirement, not an observation** — nothing establishes that every make a rental can hold has a row.

## 6.3 A card's index, UUID and bus location survive a restart

**Pointer.** [The multi-provider batch](../The%20multi-provider%20batch%20—%20the%20surface%20that%20did%20not%20vary,%20the%20Debian%20route%20that%20exists,%20and%20the%20cards%20a%20slice%20cannot%20see.md), Test 29, machine **C** (Hyperstack KVM virtual machine, Debian 12), **2026-08-22**. Label: **observed**. Used by [H6](../Servers.md#82-what-the-reading-is-made-of-and-every-part-of-it-is-a-trap-closed)'s last sentence.

> "Machine C, across the reboot: `0, NVIDIA L40, GPU-36e29c4f-d527-a4d7-01bf-5d42626be0e7, 00000000:00:07.0` — index, UUID and bus location **all unchanged**. **A negative would have been the ordering changing.** It did not, on one card."

**Not covered, in that document's own words: *on one card***. `Servers.md` [H6](../Servers.md#82-what-the-reading-is-made-of-and-every-part-of-it-is-a-trap-closed) said *"observed across a reboot on a virtual machine"* and did not say *on one card*; **it carries the scope from 2026-08-23.** The observation cannot speak to whether an ordering is stable across a restart on a multi-card box, which is the case where it would matter, and no multi-card box was restarted. **Nothing in `Servers.md` is keyed on a card's index**, which is what makes that gap cost nothing.

## 6.4 A count taken below the driver's own query counts the host's cards

**Pointer.** [The multi-provider batch](../The%20multi-provider%20batch%20—%20the%20surface%20that%20did%20not%20vary,%20the%20Debian%20route%20that%20exists,%20and%20the%20cards%20a%20slice%20cannot%20see.md), Group 7, two hosts of four cards each rented as two-card slices, **2026-08-22**. Label: **observed**. Used by [H5a](../Servers.md#82-what-the-reading-is-made-of-and-every-part-of-it-is-a-trap-closed) and [H6](../Servers.md#82-what-the-reading-is-made-of-and-every-part-of-it-is-a-trap-closed)'s vindication note.

| Reading | **E** — 2 of 4 GTX 1660 SUPER | **F** — 2 of 4 RTX 5090 |
|---|---|---|
| `nvidia-smi -L` | **2** — ours | **2** — ours |
| `nvidia-smi --query-gpu=index` | **2** | **2** |
| `/proc/driver/nvidia/gpus` | **4** — two UUIDs not ours | **4** — two UUIDs not ours |

> "**`nvidia-smi` was honest on both boxes and nothing below it was.** … **A card count taken below `nvidia-smi` counts the host's cards.** This project's own machine-A probe used `ls /proc/driver/nvidia/gpus | wc -l`, which agreed there only because that box had one card and one tenant."

> "**What [Servers] H5 and H6 already chose turns out to be the only pair that survives this**: identity from `nvidia-smi`'s UUID and bus location, presence from `lspci`. The first is vindicated. **The second has no route on a Vast container, where `lspci` is absent on all three boxes.**"

**Not covered:** two hosts, one seller, container rentals only, both four-card hosts taken as two-card slices. Nothing establishes how a **virtual machine** sold as a slice behaves, and nothing was posed on a slice of a host with a different card count.

## 6.5 The device-node route is inconsistent between hosts and its numbering is not the query's

**Pointer.** Same document, same group. Label: **observed**. Used by [H5b](../Servers.md#82-what-the-reading-is-made-of-and-every-part-of-it-is-a-trap-closed).

> "E exposed four nodes for a two-card rental; F exposed two. And on F the kernel's `Device Minor` values for **our** cards were **2** and **0** — so `/dev/nvidia0` and `/dev/nvidia2` are ours, and the minor number has no relationship to `nvidia-smi`'s index 0 and 1. **Anything that identifies a card by device-node number, or that assumes node *n* is index *n*, is wrong on this box.**"

**Not covered:** two boxes on one seller. The prohibition [H5b](../Servers.md#82-what-the-reading-is-made-of-and-every-part-of-it-is-a-trap-closed) writes is wider than the observation — it forbids the route everywhere, on the ground that a route wrong on two observed boxes cannot be trusted on an unobserved one. That is a decision resting on the observation and not a further fact.

## 6.6 Per-card memory has exactly one route

**Pointer.** [Six machines](../Six%20machines%20—%20the%20container,%20the%20webhook%20and%20the%20ready%20action.md), Fact 46. Label: **observed**. Used by [H8](../Servers.md#82-what-the-reading-is-made-of-and-every-part-of-it-is-a-trap-closed).

> "**Per-card memory has exactly one route, and it is `nvidia-smi`.** `/proc/driver/nvidia/gpus/*/information` gives model, UUID and bus location and **no memory figure at all**. So where `nvidia-smi` is absent, count and identity survive and **per-card memory does not**."

**One further reading from the same fact, not carried in `Servers.md` and recorded here because it bounds what the figure means:** *"**483 MiB of each card was already held by the driver at idle**, so free is not total."* [H6](../Servers.md#82-what-the-reading-is-made-of-and-every-part-of-it-is-a-trap-closed) reads **total** memory, so nothing built depends on it.

**Not covered:** one box, one driver version.

## 6.7 Nothing reads a card's live memory use — the one reading on record, and its limit

**Pointer.** [The multi-provider batch](../The%20multi-provider%20batch%20—%20the%20surface%20that%20did%20not%20vary,%20the%20Debian%20route%20that%20exists,%20and%20the%20cards%20a%20slice%20cannot%20see.md), Test 26. Label: **observed**. Used by [H7a](../Servers.md#82-what-the-reading-is-made-of-and-every-part-of-it-is-a-trap-closed).

> "On both E and F, `memory.used` was read three times at ten-second intervals with nothing running: **E held steady at 1 MiB on both cards; F held steady at 2 MiB on both.** `--query-compute-apps` returned an empty list on both. **A negative would have been used memory moving while Smoozy was doing nothing.** It did not move."

**Not covered, and the input document says so itself:** *"the cards `nvidia-smi` shows are the two we hold, so a neighbouring tenant's work on the host's *other* two cards could not have appeared in this reading whether it existed or not. **What is established is that the two cards we held reported only our own use. Whether a card can ever be shared under us was not posed and this test could not pose it.**"* **No clause of `Servers.md` takes this reading**, which is why [H7a](../Servers.md#82-what-the-reading-is-made-of-and-every-part-of-it-is-a-trap-closed) exists only to stop the steady figures being read as evidence of exclusivity.

## 6.8 The machine's own memory is the container's allocation where there is one

**Pointer, to two documents.** [Six machines](../Six%20machines%20—%20the%20container,%20the%20webhook%20and%20the%20ready%20action.md), Fact 46, and its reproduction at [The last two boxes](../The%20last%20two%20boxes%20—%20the%20hook%20that%20cannot%20be%20read%20back,%20the%20full%20disk,%20and%20the%20unit%20that%20kills%20a%20render.md) §14, **2026-08-18**. Label: **observed**. Used by [H9](../Servers.md#82-what-the-reading-is-made-of-and-every-part-of-it-is-a-trap-closed).

Six machines: *"`nproc` said 4 and `/proc/meminfo` said 15.57 GiB, while the real allocations were **`cpu.max` 3.84 CPUs and `memory.max` 14.95 GiB**. They nearly coincided only because that rental held all of its host's GPUs."* And the provider's own documentation quoted there: *"a 1-GPU instance on a 4-GPU machine gets 25% of CPU baseline"*, so *"**On a 1-of-8 rental, `/proc/meminfo` would overstate usable memory eightfold. The reading Smoozy must take on a container is the cgroup's, falling back to `/proc` only when it reads the literal `max`.**"*

The last two boxes §14, which is where the fallback's trigger was observed: *"Exactly the figures the earlier batch found, reproduced. The control-group interface is the version-2 one; the version-1 paths are absent. **The high-water reading answers the literal *max*, so a reader taking that instead falls back to the wrong figure.** The throwaway took the right ones and named its source."*

**Not covered:** two containers on one seller. The eightfold figure is the provider's own documented allocation rule and not a reading anybody took. The same section records that *"the provider's own API reported the processor count as `4` and `4.0` effective, **disagreeing with the control group's 3.84**"* — so a seller's own figure is not a substitute for the machine's.

## 6.9 On a four-card host's slice the version-1 control-group paths were empty and the plain readings were the host's

**Pointer.** [The multi-provider batch](../The%20multi-provider%20batch%20—%20the%20surface%20that%20did%20not%20vary,%20the%20Debian%20route%20that%20exists,%20and%20the%20cards%20a%20slice%20cannot%20see.md), Group 7 finding 3. Label: **observed**. Used by [H9a](../Servers.md#82-what-the-reading-is-made-of-and-every-part-of-it-is-a-trap-closed).

> "E was sold as a 20-core slice and `nproc` reported **40**, with `/proc/cpuinfo` listing 40 processors and `free` reporting the host's 64 GiB. **No cgroup quota was readable** to correct it (`cpu.cfs_quota_us` and `memory.limit_in_bytes` both empty — the host is on cgroup v2 and the v1 paths do not exist). So a reading of *how much machine do I have* taken the obvious way is the host's answer, and nothing inside the rental contradicts it."

**Not covered, and this is the limit that matters most for [H9a](../Servers.md#82-what-the-reading-is-made-of-and-every-part-of-it-is-a-trap-closed):** **the observation records the version-1 paths as empty and records no reading of the version-2 paths on those two boxes.** The group's table says only *"cgroup device list | not readable"*. So *the cgroup answered nothing at all* is `Servers.md`'s reading of the observation rather than the observation's own words — see the note at [11.3](#113-h9as-third-outcome-says-more-than-the-observation-records) below. Two boxes, one seller, container rentals only.

## 6.10 Fact 28 — free space is a choice of mount

**Pointer.** [Two more boxes](../Two%20more%20boxes%20—%20the%20volume,%20the%20full%20disk,%20and%20a%20day%20left%20running.md), Fact 28, **2026-08-16**. Label: **observed**. Used by [H10](../Servers.md#82-what-the-reading-is-made-of-and-every-part-of-it-is-a-trap-closed).

> "a plain `df` returned 51 filesystems, 48 of them `tmpfs` — including one per CPU thermal-throttle folder — and of the three that are not, one is **the host's own 32 GiB disk bind-mounted in to deliver the driver files, showing 20 GiB free that the rental may not use**. So the free-space question is not a reading but a choice of which mount to read, and total size, biggest, and sum all pick the wrong one."

> "the root-reserved blocks, which is why the available figure and not the free figure is the one to read"

**Not covered:** one container. `Server protocol and transfers` F3 carries the reserve's own limit — *"On one machine `Reserved block count: 0` and the two agreed; that is one box, not a rule."* The **mechanism** of the per-path reading is `Server protocol and transfers` [F1 to F3](../Server%20protocol%20and%20transfers.md#91-a-reading-is-a-choice-of-mount-and-the-mount-is-the-one-the-write-was-attempted-on)' and is not re-cited here.

## 6.11 Fact 46 — the card states, and the misclassification two of three providers shipped

**Pointer.** [Six machines](../Six%20machines%20—%20the%20container,%20the%20webhook%20and%20the%20ready%20action.md), Fact 46. Label: **observed**. Used by [H11](../Servers.md#82-what-the-reading-is-made-of-and-every-part-of-it-is-a-trap-closed).

> "**Three states exist, not two, and all three were seen.** SESTERCE and HYPERSTACK both arrived with a card in `lspci` and **no driver**, where `nvidia-smi` is simply absent — **byte-identical to what a cardless machine gives.** A machine with a driver but no card gives a third answer, `No devices were found`, exit 6, seen for real on SESTERCE in the window between unloading `nouveau` and binding `nvidia`."

> "**Anything concluding "this machine has no GPU" from `nvidia-smi` failing will misclassify a perfectly good rented A6000 as cardless — and on two of these three providers that is not a corner case, it is what they shipped.**"

> "`lspci -d 10de:` returns an empty result with exit 0 for an absent vendor and a populated line for a present one."

**Not covered, and the input document says the fact is not closed:** *"**a genuinely cardless rental was never posed.** All four attempts used a proxy — vendor filters returning empty, a GPU-less container view, or argument from what a cardless machine would necessarily show."* [One box — the Telegram capture](../One%20box%20—%20the%20Telegram%20capture%20and%20the%20port%20that%20cannot%20be%20chosen.md) adds why it stays unposed there: *"A filter for CPU-only machines is rejected by the API, and no offer with no GPU and real memory came back from any search run here"*, narrowed on 2026-08-22 to *"the search has no machine-kind filter to ask the question with."* **That narrowing struck the wording `Servers.md` §16 item 2 carried** — *"unposable on Vast, which sells no cardless offer"* — **which the body was brought into line with on 2026-08-23**, and [Facts.md](../Facts.md) Fact 46 records the strike in terms: *"(This said *which sells no cardless offer* until 2026-08-22 — what was observed is what could be asked for, not what the provider sells.)"* **What is established is that the question could not be asked, not that the offer does not exist.** **So [H11](../Servers.md#82-what-the-reading-is-made-of-and-every-part-of-it-is-a-trap-closed)'s *empty* row — the one row that concludes *no card* — rests on the tool's documented behaviour and on no cardless box.** §16 item 2 is that limit carried in the body.

## 6.12 `lspci` is absent from the container images observed

**Pointer.** [The multi-provider batch](../The%20multi-provider%20batch%20—%20the%20surface%20that%20did%20not%20vary,%20the%20Debian%20route%20that%20exists,%20and%20the%20cards%20a%20slice%20cannot%20see.md), Group 7's table and Group 1's Test 6 (*the programs are a property of the image*). Label: **observed**. Used by [H11](../Servers.md#82-what-the-reading-is-made-of-and-every-part-of-it-is-a-trap-closed)'s fifth row, [H12a](../Servers.md#82-what-the-reading-is-made-of-and-every-part-of-it-is-a-trap-closed) and [H12b](../Servers.md#82-what-the-reading-is-made-of-and-every-part-of-it-is-a-trap-closed).

The batch records `lspci` **absent** on all three Vast containers (**A**, **E**, **F**) and present on the Hyperstack virtual machine (**C**), with `pciutils` not in the image.

**Not covered:** three container images from one seller and one virtual machine from another. **[H12b](../Servers.md#82-what-the-reading-is-made-of-and-every-part-of-it-is-a-trap-closed)'s case — a machine that is not a container and has no `lspci` either — is unobserved**, and the body says so.

## 6.13 Fact 46 — on a container the bus scan is the host's unfiltered view

**Pointer, to two documents.** [Six machines](../Six%20machines%20—%20the%20container,%20the%20webhook%20and%20the%20ready%20action.md) Fact 46, and [The last two boxes](../The%20last%20two%20boxes%20—%20the%20hook%20that%20cannot%20be%20read%20back,%20the%20full%20disk,%20and%20the%20unit%20that%20kills%20a%20render.md) §14. Label: **observed**. Used by [H12](../Servers.md#82-what-the-reading-is-made-of-and-every-part-of-it-is-a-trap-closed).

Six machines: *"**in a view that genuinely has no cards, `/proc/driver/nvidia` and the PCI sysfs scan lie**: both are inherited from the host kernel and still reported two NVIDIA devices in a container that could touch neither."*

The last two boxes §14: *"**the bus scan is unmistakably the host's unfiltered view**, listing the host's own storage controller, its audio device and its network card, none of which belong to this container. **So it would lie on a multi-card host; only the arithmetic consequence is invisible here.**"*

**Not covered:** the arithmetic consequence was invisible on those boxes because each host had one card; [6.4](#64-a-count-taken-below-the-drivers-own-query-counts-the-hosts-cards) is where it became visible, and there the scan did not lie — **it did not exist**. [H12a](../Servers.md#82-what-the-reading-is-made-of-and-every-part-of-it-is-a-trap-closed) is that pair of different failures with one outcome.

## 6.14 Fact 46 — counting display devices is forbidden

**Pointer.** [Six machines](../Six%20machines%20—%20the%20container,%20the%20webhook%20and%20the%20ready%20action.md), Fact 46. Label: **observed**. Used by [H13](../Servers.md#82-what-the-reading-is-made-of-and-every-part-of-it-is-a-trap-closed).

> "Counting `/sys/class/drm/card*` reports **two cards on a one-card machine**, because both SESTERCE and HYPERSTACK carry an **emulated Cirrus console adapter** that is a display device and not a render card — so vendor id `0x10de` is the discriminator, never device class."

**Not covered:** two boxes. The emulated adapter is a property of those images; nothing establishes that every rental carries one, and nothing needs it to — the prohibition costs nothing where no adapter exists.

## 6.15 The Blender route to a hardware reading, considered and declined

**Blender facts are not re-derived here.** `Servers.md` [H16](../Servers.md#83-where-the-reading-is-taken-and-why-no-blender-is-in-it) leans on Blender in two places and **names no joined answer and no comparison question for either**. The project does name them, in the Specs-stage companion this document's own scope descends from — [Preparation/Specs/Sources/Servers.sources.md](../../Specs/Sources/Servers.sources.md):

- *"A Blender already on a machine can be asked which cards it sees and can count them, with no window and on a machine with no display server"* — **The machine 6**, in the [comparison](../../Blender%20Research/Comparison/The%20machine.md), *"the same in all four versions"*.
- *"The one route by which the research ever saw a card's whole memory needs Blender's window and has never been seen with more than one card"* — **Reports 6**, in the [comparison](../../Blender%20Research/Comparison/Reports.md).

Label: **documented** through the Blender Research comparison, per [Rule 2](../../Groundwork/Rules.md). **Nothing is assembled here out of several topics**, and neither comparison question is restated.

**Not covered:** both readings are about what a Blender on a machine can be asked. Neither says anything about a machine with no Blender on it, which is the ordinary case at the moment a server is added and is [H16](../Servers.md#83-where-the-reading-is-taken-and-why-no-blender-is-in-it)'s first reason for declining the route.

**And a condition on the first of the two, carried 2026-08-31 out of the repaired Blender research.** **Every reading of a machine's cards on record was taken after a refresh was asked for; asked cold, a four-card machine and a cardless one give the same answer** — the repaired *The machine 5* records the windowless runs coming back **empty** in all four versions with an untouched settings folder, and empty again under factory settings, filling only once a refresh was asked for. Label: **observed in a test**. **Nothing breaks: [H16](../Servers.md#83-where-the-reading-is-taken-and-why-no-blender-is-in-it) declines the route, and the repair strengthens its third reason** — a reading of the list alone was already unsafe there, and it is now unsafe in a second way. `Facts.md` Fact 46 is where the pointer should gain **The machine 5**, which is that register's and is not edited here.

**Carried into the document 2026-09-02, as [H16](../Servers.md#83-where-the-reading-is-taken-and-why-no-blender-is-in-it)'s fourth reason.** The route to it is the map's own arrival question *what does a machine answer when it is asked, cold, which cards it has*, and the joined answer it names, [Running Blender with nobody there](../../Blender%20Research/Across%20topics/Running%20Blender%20with%20nobody%20there.md): *"A rented machine asked what cards it has, before anything else, says none — and that is not the machine having none."* **It is the strongest of H16's four reasons because it defeats the reading on a machine that has cards**, and it would have fed the one row of [H11](../Servers.md#82-what-the-reading-is-made-of-and-every-part-of-it-is-a-trap-closed)'s table that concludes *no card*. The safe rule for anything that does read the list is `Server Setup` R1a's — refresh first, and never conclude *no cards* from an unrefreshed empty one.

## 6.16 Test 27 — Blender enlists no card unless it is told to

**Pointer.** [The multi-provider batch](../The%20multi-provider%20batch%20—%20the%20surface%20that%20did%20not%20vary,%20the%20Debian%20route%20that%20exists,%20and%20the%20cards%20a%20slice%20cannot%20see.md), Test 27, machines **E** and **F**, both Blender versions, **2026-08-22**. Label: **observed** — a reading taken on a rented machine by this stage, not a Blender Research finding, and it is named as such. Used by [H16](../Servers.md#83-where-the-reading-is-taken-and-why-no-blender-is-in-it)'s third reason.

> "From `--factory-startup`, `bpy.context.preferences.addons["cycles"].preferences.compute_device_type` arrives as **`NONE`**, and a Cycles render therefore runs **on the processor**. The individual devices arrive with `use = True` — both CUDA entries and both OptiX entries — **so a reading of the device list alone suggests every card is enabled, and it is the backend selector that is off.**"

**A conflict, stated rather than resolved. Added 2026-08-31, out of the repaired Blender research.** **The repaired *The machine 5* says the list itself came back *empty*** in all four versions in the windowless runs, with an untouched settings folder, and empty again under factory settings, filling only once a refresh was asked for. **This test's own record does not say whether its script called a refresh**, so the two cannot be reconciled from anything this project holds. Per the Architecture method, it is **said and not resolved**: no clause is rewritten on it here, and [H16](../Servers.md#83-where-the-reading-is-taken-and-why-no-blender-is-in-it) declines the route either way. `Rendering`'s sources file M13 carries the same conflict from its own side.

**Carried into the document 2026-09-02, and it stays unresolved there. [Section 16](../Servers.md#16-unresolved) item 24** now names both records, says neither is doubted, says nothing written down reconciles them, and says **no observation will settle it** — the Creator ruled on 2026-08-31 that there will be no further Blender research and no further rented machine. **Nothing here is decided by it**: H16's third and fourth reasons are each independently sufficient, so the decline survives whichever record is right. `Server Setup` §13 item 25 carries the same conflict and states the safe reading for anything that does read a device list.

**Not covered:** two boxes on one seller, both containers, two Blender versions. The same test carries a second caution `Servers.md` does not use — *"`preferences.get_device_types()` returned `['NONE', 'CUDA', 'OPTIX', 'HIP', 'ONEAPI']` on machines with NVIDIA cards and no AMD hardware … **The list is what the build was compiled with, not what works on the machine.**"* The launch-side consequence is `Rendering` L21's and is not this document's.

---

# 7. Section 9 — the key's three moments

## 7.1 Fact 35 half A — what is readable from an encrypted private key with no passphrase

**Pointer.** [The manuals](../The%20manuals%20—%20keys,%20secrets,%20and%20what%20cannot%20be%20told%20apart.md), Fact 35 half A, **observed on one Mac, macOS 26.5.2 (build 25F84), `OpenSSH_10.2p1, LibreSSL 3.3.6`, 2026-08-16**, with the documented half from `ssh-keygen(1)` at `man.openbsd.org`. Label: **documented and observed**. Used by [P5](../Servers.md#91-before-a-connection-row-33).

> "**Type, bit size and fingerprint are readable from an encrypted private key file with no passphrase, and remain readable when the `.pub` file does not exist**"

> "**Whether the file is passphrase-protected at all** can be determined without a prompt by supplying an empty passphrase explicitly" — `ssh-keygen -y -P ''` giving exit 0 on an unprotected key and exit 255 with `incorrect passphrase supplied to decrypt private key` on a protected one; and, without invoking the tool at all, the container's own cipher and KDF name fields read from the decoded file.

**Not covered, and the input document narrows its own findings twice:** *"**On the two keys tested**, exit 0 came from the key without a passphrase … *This was written as a general rule and narrowed by the check: two keys were tested. Whether the mapping holds generally is not established — and Fact 37's route-A table shows exit 255 arising from at least three other causes on the same build.*"* The byte-parsing route rests on *"a single data point"* — one encrypted key in a set of four — and *"This container layout is **not documented** on any of the six manual pages"*. **One machine, one build, one operating system.** Which route is read as the yes-or-no, and its three narrowings, are `Security and licence` §6's and are not re-decided here.

## 7.2 Fact 35 half B — batch mode and the passphrase prompt

**Pointer.** [The manuals](../The%20manuals%20—%20keys,%20secrets,%20and%20what%20cannot%20be%20told%20apart.md), Fact 35 half B, as corrected **2026-08-22**. Label: **observed** for the behaviour, **the documentation gap stands**. Used by [P6](../Servers.md#91-before-a-connection-row-33) and §16 item 3.

The documented half, `ssh_config(5)` at `man.openbsd.org`, read 2026-08-16:

> "If set to yes, user interaction such as password prompts and host key confirmation requests will be disabled. This option is useful in scripts and other batch jobs where no user is present to interact with ssh(1). The argument must be yes or no (the default)."
> — `BatchMode`

The input document's own reading of that: *"A private-key passphrase is neither a password nor a host key confirmation; "user interaction such as" is open-ended but names neither. **The manual therefore does not settle whether a `BatchMode=yes` connection using an encrypted key fails outright or prompts.**"*

The run that settled the behaviour, **2026-08-20**: *"on the Creator's own Mac, against a throwaway server on the loopback address, **with a terminal deliberately attached** — because with no terminal the test proves nothing — an encrypted key with batch mode on produced **no prompt at all** and refused at exit 255, against two controls that both prompted or authenticated as expected. A negative would have been the prompt appearing exactly as in the control."*

**Not covered, in that document's own words:** *"**What stays open is the documentation half only** … so the construction is observed on one build rather than guaranteed."* One machine, one build, one throwaway server. **Nothing in [9.1](../Servers.md#91-before-a-connection-row-33) rests on it** — P1 to P3 keep the promise by not making the attempt.

## 7.3 Fact 58 — an open connection survives its key file going away

**Pointer.** [Six machines](../Six%20machines%20—%20the%20container,%20the%20webhook%20and%20the%20ready%20action.md), Fact 58. Label: **observed**. Used by [P9](../Servers.md#92-during-a-connection-row-34).

> "**Answered: yes.** A copy of the key was made, a session opened with the copy, and the copy deleted; **the live session kept working, and a fresh connection with the missing key failed** — proving the key gated authentication and the survival is because the session no longer needs the file. The real key was never touched."

**Not covered:** deletion only. The fact as registered asks about the file being *"deleted, moved, or changed under it"*; what was posed is deletion of a copy. **[P8](../Servers.md#92-during-a-connection-row-34) does not rest on the observation** — it rests on `Security and licence` §6's arrangement, under which nothing after authentication reads the key file at all, and P9 is the confirmation rather than the ground.

## 7.4 Fact 76 — what a fingerprint is, which hash, and the collision clause

**Pointer, plus two direct publications.** [The manuals](../The%20manuals%20—%20keys,%20secrets,%20and%20what%20cannot%20be%20told%20apart.md), Fact 76, on `man.openbsd.org` pages read 2026-08-16 and one Mac at `OpenSSH_10.2p1`; the reframing of 2026-08-20 is recorded at [Facts.md](../Facts.md) Fact 76 and at `Security and licence` §6 and `Keys` F8. Label: **documented and observed**. Used by [P16](../Servers.md#93-at-the-list-row-46) and §16 item 4.

Documented, the same sentence on four pages: *"Specifies the hash algorithm used when displaying key fingerprints. Valid options are: “md5” and “sha256”. The default is “sha256”."* — `ssh-keygen(1) -E`, `ssh-agent(1) -E`, and in the `FingerprintHash` form on `ssh_config(5)` and `sshd_config(5)`.

Cross-system stability, from the protocol rather than from any system: **RFC 4253 §6.6**, `https://www.rfc-editor.org/rfc/rfc4253.txt` — a public key's encoding is *"string    certificate or public key format identifier"* followed by *"byte[n]   key/certificate data"*. Recorded at [Facts.md](../Facts.md) Fact 76, which adds the hand recomputation: *"the base64 field of a key file was decoded and hashed directly, and both the SHA-256 and the MD5 forms **reproduced the tool's printed line exactly**, at 32 and 16 digest bytes."*

The collision half, and why it was the wrong question: **RFC 6151**, `https://www.rfc-editor.org/rfc/rfc6151.txt`, read **2026-08-20** — *"The attacks presented in [KLIM2006] can find MD5 collision in about one minute on a standard notebook PC (Intel Pentium, 1.6GHz)."* and *"MD5 is no longer acceptable where collision resistance is required such as digital signatures."*

**Not covered.** The manual pages read are **OpenBSD-current** and *"say nothing about other platforms' builds"*. The hand recomputation ran on **one machine**. **No first-party sentence says a SHA-256 collision does not exist**, because an absence of published collisions is not a citable statement — the input documents say so in terms, and whether Row 293's world-side clause may close on that basis is the Creator's, parked at [Open Threads](../../Groundwork/Open%20Threads.md) item 8a. **What `Servers.md` owns is holding the value and handing it over**; the recognition and the pinned hash are `Keys` F8's and `Security and licence` §6's.

## 7.5 The three systems' secret stores

**Pointer.** [The manuals](../The%20manuals%20—%20keys,%20secrets,%20and%20what%20cannot%20be%20told%20apart.md), Fact 38 — macOS observed on one Mac, **Windows and Linux documentation only**. Label: **documented**, with the macOS half **observed**. Used by [Section 17](../Servers.md#17-moments-that-reached-the-creator)'s last entry, where the ruling of 2026-08-16 is recorded as spent.

The input document's own first limit: *"**No Windows machine has run any of this, and none is rented today.** … **Every Windows machine this project has read reported a Server edition** … Every Windows answer below is documentation alone."* And its third: three Linux documentation sites refused automated retrieval on 2026-08-16, so *"Canonical's Ubuntu Server documentation is therefore the *only* Linux distribution documentation that could be read at all."*

**Not covered:** everything Windows and everything Linux is unconfirmed against a real installation. **Nothing in `Servers.md` rests on the detail** — the store is `Security and licence` §3's and the adapter is `Desktop app and delivery` §9's; this document holds a subject reference and no value.

---

# 8. Section 10 — the first connection

## 8.1 SSH's own host-trust default, and `accept-new`

**Direct publication, cited through a Specs-stage companion.** OpenSSH's `ssh(1)` and `ssh_config(5)` at **man.openbsd.org**, retrieved **2026-07-17**, recorded at [Preparation/Specs/Sources/Servers.sources.md](../../Specs/Sources/Servers.sources.md). Label: **documented**. Used by [Q3](../Servers.md#101-host-trust-comes-first-and-the-scan-decides-row-36).

> "That same `StrictHostKeyChecking` documentation records the default as **asking** the user before a new host identity is accepted, and `accept-new` as the setting that accepts new identities without asking while still refusing changed ones."

`ssh(1)`'s own sentence, quoted in [The manuals](../The%20manuals%20—%20keys,%20secrets,%20and%20what%20cannot%20be%20told%20apart.md): *"If a host's identification ever changes, ssh warns about this and disables password authentication to prevent server spoofing or man-in-the-middle attacks, which could otherwise be used to circumvent the encryption. The StrictHostKeyChecking option can be used to control logins to machines whose host key is not known or has changed."*

**Not covered, and the Specs companion says it:** *"This citation covers SSH's own behaviour only. Smoozy's host-identity policy is a decision, settled in the body of the spec."* Silent first trust plus a user-decided replacement of a **changed** identity is Creator-stated (2026-07-17) and earns no entry.

## 8.2 Fact 82 — the offered key is obtainable and comparable with no credential

**Pointer.** [The host key](../The%20host%20key%20—%20obtained%20without%20authenticating,%20and%20whose%20it%20turns%20out%20to%20be.md), Fact 82, **2026-08-17**, from the Creator's Mac (`OpenSSH_10.2p1`) against one Vast rental. Label: **documented and observed**. Used by [Q4](../Servers.md#101-host-trust-comes-first-and-the-scan-decides-row-36) and, through `Server protocol and transfers` A5, by [Section 10](../Servers.md#10-the-first-connection-in-order-rows-36-and-35)'s ordering.

Documented, `ssh-keyscan(1)`, macOS 26.5 man page dated October 4, 2025, read 2026-08-17:

> "For scanning, one does not need login access to the machines that are being scanned, nor does the scanning process involve any encryption."

Observed:

> "`ssh-keyscan -p 28452 ssh3.vast.ai` exited **0**, wrote nothing to stderr, and returned three keys — `ssh-rsa`, `ecdsa-sha2-nistp256` and `ssh-ed25519` … No identity file was supplied and none was asked for."

> "**Two independent scans, minutes apart, returned byte-identical key material.** So the scan is a reading of something stable, not a per-connection value."

**The words `Servers.md` prints were repaired 2026-08-23.** [Q4](../Servers.md#101-host-trust-comes-first-and-the-scan-decides-row-36) quoted the fact as *"exited 0, offered no identity, was asked for none, and returned byte-identical material on two scans minutes apart"*, and **that exact sentence is not in the input document** — it is a condensation of the passages above, stated as a quotation of the fact. Q4 now quotes the observation's own words and nothing else; **nothing said in either form goes beyond what was observed**, so no answer moved. **The same condensation stands in `Server protocol and transfers`**, whose clause is that document's to repair and is not touched here.

**Not covered, in the input document's own words:** *"The scan was posed against one server on one provider from one caller. Whether a scan can be defeated — a middle that answers the scan honestly and the session dishonestly, or the reverse — was not posed and is not answerable from a single vantage point."* And: *"`ssh-keyscan(1)` has no EXIT STATUS section at all"* — the exit codes are observed on one version and documented nowhere. §16 item 5 carries the residual, which [Q7](../Servers.md#102-the-three-paths-and-the-two-reads-that-close-the-gap-between-them)'s throwaway file narrows to the window between the two reads and does not close.

## 8.3 Fact 37 — no OpenSSH manual documents the diagnostic strings

**Pointer.** [The manuals](../The%20manuals%20—%20keys,%20secrets,%20and%20what%20cannot%20be%20told%20apart.md), Fact 37, and [The host key](../The%20host%20key%20—%20obtained%20without%20authenticating,%20and%20whose%20it%20turns%20out%20to%20be.md), which states it as its own premise. Label: **documented — an established absence**. Used by [Q5](../Servers.md#101-host-trust-comes-first-and-the-scan-decides-row-36), [Q12](../Servers.md#103-the-not-linux-check-row-35), and by [Section 16](../Servers.md#16-unresolved) item 1's argument.

The host key document's own statement: *"Fact 37 established that **no OpenSSH manual documents a single one of the diagnostic strings the tools emit**"*, with the thirteen-line changed-host-key warning printed verbatim there and its two limits recorded: *"**`255` is ssh's generic failure status and distinguishes nothing.** No exit status separates a host-key mismatch from a refused credential, an unreachable address or a timeout. Only the text above does … so a text match on them is a version change away from breaking silently."*

**Not covered:** six manual pages on one date at `man.openbsd.org`, and one build's observed output. An absence in a manual is evidence that nothing is promised, not evidence that a string will change.

## 8.4 Fact 83 — whose host key it is, and what an unchanged one means

**Pointer, to three documents, because the answer widened twice.** Label: **observed**. Used by [D11](../Servers.md#63-the-remembered-host-key-row-37), [Q10](../Servers.md#102-the-three-paths-and-the-two-reads-that-close-the-gap-between-them) and §16 item 5.

**First, on a container, 2026-08-17** — [The host key](../The%20host%20key%20—%20obtained%20without%20authenticating,%20and%20whose%20it%20turns%20out%20to%20be.md). The German box was destroyed and *"a fresh rental taken on the same physical machine, `141036`"*: a beyond-doubt different container — different hostname, `/root/day.jsonl` gone with everything else Smoozy had put there — reached at a different proxy and port, presenting **byte-identical** host keys with the same comment and the same file date. *"The host key survived the rental it belonged to."*

**Second, on containers across two hosts, 2026-08-22** — [The multi-provider batch](../The%20multi-provider%20batch%20—%20the%20surface%20that%20did%20not%20vary,%20the%20Debian%20route%20that%20exists,%20and%20the%20cards%20a%20slice%20cannot%20see.md), Test 19. The same offer re-rented moved the proxy port (33510 → 34412) and returned **all three host keys identical**, comment `root@buildkitsandbox`; a **different** machine on the **same image** presented **three different keys**. *"**So the keys are neither the proxy's nor the image's: they are generated once on the host and reused by whatever the host rents out next.** They survive the destruction of the rental that first saw them."*

**Third, on a virtual machine, 2026-08-18** — [The last two boxes](../The%20last%20two%20boxes%20—%20the%20hook%20that%20cannot%20be%20read%20back,%20the%20full%20disk,%20and%20the%20unit%20that%20kills%20a%20render.md) §17. Four host keys commented `root@ubuntu`, key files dated **2025-11-06 20:55:50** to the nanosecond; after a recycle that removed both logins, Blender, the throwaway and the lingering flag, *"**identical. Every fingerprint, every file size, every timestamp to the nanosecond.**"* — *"this provider's image does not generate host keys at first boot. It ships them."*

**The retraction this replaced, recorded because `Servers.md` [Q10](../Servers.md#102-the-three-paths-and-the-two-reads-that-close-the-gap-between-them) is written against it.** The host key document's *generated at first boot* reading was *"an inference from a log line, not a reading"* — cloud-init prints the fingerprints of whatever keys are present — and that document now carries the correction in place.

**Not covered.** Two sellers, four boxes. **Still open, in the batch's own words:** *"whether the seller ever hands the same proxy port to a different machine, which would need a much longer watch than this batch could run."* And, from the host key document: *"whether a machine's keys ever change without the hardware changing"*, and *"a virtual machine whose keys were read at rental and compared against a second rental of the same image on different hardware — the test Test 19 ran for containers and nobody has run for machines."*

## 8.5 Fact 59, macOS half — the SSH version banner on a macOS host

**Direct observation on the Creator's own computer.** Label: **observed in a test**. Used by [Q15](../Servers.md#103-the-not-linux-check-row-35) and §16 item 1. **This entry stood in this file before this brief and is kept.**

**The machine:** the Creator's own Mac, `ProductName: macOS`, `ProductVersion: 26.5.2`, `BuildVersion: 25F84`. Read **2026-08-19** at 19:18 UTC.

**What was done.** The connection was opened to the local SSH server already listening on port 22 and the first line it announced was read, with nothing sent and nothing authenticated.

**What came back, verbatim:**

```
SSH-2.0-OpenSSH_10.2
```

**What it establishes, and it is a negative result.** The banner a macOS machine announces carries **no operating-system marker of any kind** — no vendor name, no system name, nothing but the protocol version and the implementation's own version. Against it, the rented Ubuntu box on this project's record announced `SSH-2.0-OpenSSH_8.9p1 Ubuntu-3ubuntu0.16` ([The host key](../The%20host%20key%20—%20obtained%20without%20authenticating,%20and%20whose%20it%20turns%20out%20to%20be.md), which records it as that box's own sshd announcement). **So a bare banner is what a macOS machine gives and is also what a Linux build carrying no vendor suffix would give, and the banner cannot separate the two.** The banner route to *how Smoozy tells* is therefore ruled out by observation rather than only by Fact 37's caution.

**What a negative result would have looked like:** a banner naming the system — `…OpenSSH_10.2 macOS` or similar — which would have made the banner a usable discriminator and would have closed Fact 59's macOS half outright.

**Not covered, and this is the larger half.** **What a macOS machine answers to a command over an authenticated connection was not posed.** Posing it needs a public key written into the Creator's own `~/.ssh/authorized_keys`, which is a change to their own SSH configuration — the same thing `Security and licence` §7 forbids Smoozy from doing, and which this stage's rule that nothing pre-existing is touched also forbids. So the reading `Servers` actually takes — the machine naming its own operating system over the command channel — is **unobserved over an authenticated connection to a machine that is not Linux**. **Corrected 2026-08-22: this said the reading is unobserved on any machine that is not Linux, and it said the Windows half remains not posable at all. Both are wrong.**

**The macOS value was read directly, and this is the observation.** On the same Mac, on **2026-08-20**, `/usr/bin/uname -s` answered `Darwin` at exit 0, and `/usr/sbin/sysctl kern.ostype` answered `kern.ostype: Darwin`. **One reading, one machine, one release**, and it is not evidence about macOS releases in general: `uname(1)` (macOS 26.5 man page dated November 13, 2020, read the same day) documents only *"-s      Write the name of the operating system implementation to standard output."*, and its STANDARDS section points at POSIX, which fixes the flag and not the string. **It needed no authentication**, because the value a macOS machine names is the same whether the shell was reached over SSH or not — so the authenticated route named above bounds the *connection*, not this half of the question. **What it does not settle is which command [Q12](../Servers.md#103-the-not-linux-check-row-35) takes**, which the design never names, so no reading closes Fact 59 until it is.

## 8.6 Fact 59, Windows half — the marker in Microsoft's own fork

**Direct publications, all of them addressed as of 2026-08-23.** Label: **documented**, first-party. Used by [Q15](../Servers.md#103-the-not-linux-check-row-35) and §16 item 1. Recorded in the register at [Facts.md](../Facts.md) Fact 59, read **2026-08-20**.

**Addressed and complete:**

- Microsoft names its own fork. Its OpenSSH overview page calls `https://github.com/PowerShell/openssh-portable` *"The Microsoft fork of this project"*. The page is Microsoft's OpenSSH overview, `https://learn.microsoft.com/en-us/windows-server/administration/openssh/openssh_overview`, whose `ms.date` and re-read date are recorded at [Keys.sources.md](Keys.sources.md) (**2025-02-20**, updated **2025-05-01**, re-read 2026-08-20).
- The identification string reaches the wire before authentication. **RFC 4253 §4.2**, `https://www.rfc-editor.org/rfc/rfc4253.txt`: the string is sent *"When the connection has been established"*, with key exchange beginning after.
- The default shell, and OpenSSH's arrival on Windows. Microsoft, *OpenSSH Server Configuration for Windows*, `https://learn.microsoft.com/en-us/windows-server/administration/openssh/openssh-server-configuration`, page dated **2025-08-05**, read **2026-08-20** — the address and page date are recorded at [The manuals](../The%20manuals%20—%20keys,%20secrets,%20and%20what%20cannot%20be%20told%20apart.md) for a different quotation from the same page. The two sentences this document uses: *"OpenSSH is open-source and was added to Windows Server and Windows client operating systems starting with Windows Server 2019 and Windows 10 (build 1809)"* and *"The initial default in Windows is the Windows command prompt (cmd.exe)"*.

**The two source files, addressed 2026-08-23 by reading them in the fork itself.** The body quotes two files inside that repository; the repository root was the only address the project held, and these are the file-level addresses. Both re-read **2026-08-23** on the `latestw_all` branch, which is the branch the Windows build is made from.

- **The version string.** `https://github.com/PowerShell/openssh-portable/blob/latestw_all/version.h`, whose own header line is `/* $OpenBSD: version.h,v 1.105 2025/04/09 07:00:21 djm Exp $ */`. The file's first four lines, verbatim:

> `#define SSH_WINDOWS_VERSION "OpenSSH_for_Windows_10.0"`
> `#define SSH_WINDOWS_BANNER " Win32-OpenSSH-GitHub"`
> `#define SSH_VERSION	SSH_WINDOWS_VERSION SSH_WINDOWS_BANNER`

- **The identification-string composition.** `https://github.com/PowerShell/openssh-portable/blob/latestw_all/kex.c`, in `kex_exchange_identification`, under the comment `/* Prepare and send our banner */`:

> `if ((r = sshbuf_putf(our_version, "SSH-%d.%d-%s%s%s\r\n",`
> `    PROTOCOL_MAJOR_2, PROTOCOL_MINOR_2, SSH_VERSION,`

**Read against what the body carried, and both hold exactly.** `SSH_WINDOWS_VERSION` is `"OpenSSH_for_Windows_10.0"` and the format string is `"SSH-%d.%d-%s%s%s\r\n"` with `SSH_VERSION` as its third argument, as recorded on 2026-08-20. Label: **documented**, first-party. **Two things the reading adds rather than corrects:** `SSH_VERSION` is `SSH_WINDOWS_VERSION` concatenated with `SSH_WINDOWS_BANNER`, so the marker reaches the wire ahead of the GitHub-specific suffix rather than inside it — which is exactly the reading §16 item 1 rests its *very likely in both builds* on; and `SSH_WINDOWS_BANNER` reading `" Win32-OpenSSH-GitHub"` is the branch identifying itself as the GitHub build, which is the register's own ground for calling the in-box build unread.

**Not covered:** a branch is a moving address. What was read is the state of `latestw_all` on 2026-08-23, and the version line above is the only version stamp either file carries. **Not exercised:** no Windows machine was used, so neither file could be compiled or its banner read off a running server.

**Not covered, and the register says it in terms.** *"One clause is genuinely unread and it is the only part wanting a machine."* What was read is the branch behind the **GitHub** release of the Windows build — its own banner constant says `Win32-OpenSSH-GitHub` — and Microsoft distinguishes that from the **in-box** build shipped with Windows, which its own troubleshooting page says *"often lags behind the latest Win32-OpenSSH releases that are available on GitHub"*. *"The marker sits in the Windows version macro rather than in the GitHub-specific suffix, so it is very likely present in both; **very likely is not this project's bar.**"* **What would settle it:** one TCP connection to a Windows machine with the in-box server feature enabled, reading the first line, with no login and no change to the machine. **PERMANENTLY UNRESOLVED, 2026-09-02.** No such machine exists on this project and none is coming: the Creator ruled on 2026-08-31 that there will be no further rented machine for answering a question. **This is not a pending citation and must not be read as one** — the reading is not late, it is not going to be taken, and nothing here is to be left open against it. [Servers](../Servers.md#16-unresolved) §16 item 1 carries the same permanence with what breaks and what holds.

**And no Windows machine ran any of this.** [The manuals](../The%20manuals%20—%20keys,%20secrets,%20and%20what%20cannot%20be%20told%20apart.md)'s first limit binds every Windows claim in this document: *"No Windows machine has run any of this, and none is rented today."*

---

# 9. Section 11 — the machine's work

## 9.1 Fact 60 — seen yes, stopped no, against a real render

**Pointer.** [The last two boxes](../The%20last%20two%20boxes%20—%20the%20hook%20that%20cannot%20be%20read%20back,%20the%20full%20disk,%20and%20the%20unit%20that%20kills%20a%20render.md) §18, on a virtual machine, **2026-08-18**. Label: **observed**. Used by [W9](../Servers.md#113-an-offer-for-work-the-current-login-cannot-stop), [W12](../Servers.md#114-a-username-edit-and-the-work-it-leaves-row-40) and [W13](../Servers.md#114-a-username-edit-and-the-work-it-leaves-row-40).

> "A real 400-frame Cycles render, 44 threads, seen from a second ordinary login on the virtual machine. **Seen: everything.** … **Stopped: nothing, every time.** Polite and forcible stops against both the render and the worker were refused with `Operation not permitted`; a pattern kill of the render was refused; a pattern kill of every process of that login was refused seven times over. **All 44 threads survived and the render kept going.** **The existence probe is refused too**, so a second login cannot even test liveness by signal. And the endpoint holds. From the second login the endpoint's directory was `Permission denied`, and the client binary itself was `Permission denied`."

**Not covered:** one machine kind, one operating system, one pair of ordinary logins, one day. Nothing was posed with an elevated second login, which is a different question and one Smoozy never has.

## 9.2 Fact 84 — the endpoint cannot be lied to about who is calling

**Pointer.** [The host key](../The%20host%20key%20—%20obtained%20without%20authenticating,%20and%20whose%20it%20turns%20out%20to%20be.md), Fact 84, posed on **both machine kinds**, 2026-08-17. Label: **observed**. Used by [W13](../Servers.md#114-a-username-edit-and-the-work-it-leaves-row-40).

> "**A `uid 0` claim reaching the receiver as `uid 0` would have been the negative result.** It was refused outright. So a caller can attach only its own true identity, and attaching nothing yields the same truth — **the endpoint cannot be lied to about who is calling**, on either machine kind."

**Not covered:** two boxes, one seller, one kernel family. The rule the mechanism serves — *"The endpoint is reachable only by the login Smoozy holds"* — is `Security and licence` §8's and is cited rather than restated.

## 9.3 Nothing is ever found by process search

**Pointer, through the stage's shape document to its observation.** [The shape](../The%20shape.md) and `Rented-server runtime` X6, on [Batch A](../Batch%20A%20—%20the%20shape%20throwaway.md)'s observation that a pattern selected more than one process. Label: **observed**. Used by [Section 11](../Servers.md#11-the-machines-work-rows-42-43-40-44-and-317)'s constraint and by [W13](../Servers.md#114-a-username-edit-and-the-work-it-leaves-row-40)'s second bullet.

The completing half is [The host key](../The%20host%20key%20—%20obtained%20without%20authenticating,%20and%20whose%20it%20turns%20out%20to%20be.md), Fact 91: *"a recorded identifier alone **is** a pattern with extra steps, because identifiers are re-handed-out; the identifier **paired with the kernel-set start time** is not"* — demonstrated by forcing a reuse on a virtual machine and getting `exists=True   matches=False`.

**Not covered, in that document's own words:** *"The pairing was demonstrated across a forced reuse **within one boot**."* A container could not pose it at all — three routes, three refusals.

## 9.4 A detached render survives the connection dropping; a non-detached one does not

**Pointer.** [The rented machine](../The%20rented%20machine.md). Label: **observed**. Used by [W27](../Servers.md#116-disconnecting-does-not-stop-the-machines-work-row-44).

> "A render started detached survives the connection dropping"

with its parent the system itself, still writing frames after the connection was closed and a fresh one opened; and *"A render **not** detached dies when the connection drops"*, ended at once with the frame in flight lost entirely.

**Not covered:** one box, one day, one launch arrangement each. The same document records that **nothing on that machine would bring work back by itself**. The launch is `Rendering`'s and the machine-side arrangement is `Rented-server runtime`'s; this document neither chooses nor performs either.

## 9.5 What each way of stopping a render leaves of the last image

**Blender fact, named through the comparison question the project already carries.** `Servers.md` [W19a](../Servers.md#115-stop-and-clear-and-what-it-says-when-it-only-partly-worked-row-42) quotes `Job files and outputs` C33; that document's companion routes it to **Frames 12** in the [comparison](../../Blender%20Research/Comparison/Frames.md), through [Preparation/Specs/Sources/What a job writes.sources.md](../../Specs/Sources/What%20a%20job%20writes.sources.md). Label: **observed**. Used by [W19a](../Servers.md#115-stop-and-clear-and-what-it-says-when-it-only-partly-worked-row-42)'s fourth and sixth bullets.

The words `Servers.md` carries, in `Job files and outputs` C33's own form: *"An **interrupt** has never left a short image — not in one run, by anybody, in any version, on either machine, across hundreds of runs. A **polite request to close** and an **outright kill** each left the last image short in all eighty of their runs when the stop was aimed at the moment the file appeared, and **on Linux every short file was exactly zero bytes**."*

**Not covered, and the Specs companion states the limit precisely.** The eighty-run sets *"aimed each signal within about a thousandth of a second of the file appearing"*; two smaller earlier sets stopped at no particular moment and *"finding nothing part-written is what chance predicts rather than what Blender promises"*. **So the interrupt's clean record is a record of runs, not a promise** — which is exactly why [W19a](../Servers.md#115-stop-and-clear-and-what-it-says-when-it-only-partly-worked-row-42) says the frame being rendered **may not arrive** and never that it *is lost*. **Nothing here is assembled out of several topics**, and no Blender answer is derived in this file.

---

# 10. Section 13 — the connection identity

**The four entries below stood in this file before this brief and are kept as written, with their limits completed.** They are the reading that closed Fact 61.

## 10.1 Vast.ai's own networking page — port mapping, and several containers on one address

*(Fact 61, container half; Row 24.)*

**Source:** Vast.ai's own documentation, *Networking & Ports* — `https://docs.vast.ai/documentation/instances/connect/networking`. Read **2026-08-19**. Label: **documented**.

**What it answered, in its own words:**

> "Each open internal port (such as 22 or 8080 etc) is mapped to a *random* external port on the machine's (usually shared) public IP address."

> "In essence each docker instance gets a fraction of a public IP address based on a subset of ports."

> "Vast.ai docker instances have full internet access, but generally do not have unique IP addresses."

The page also describes an environment value per requested internal port carrying the external port that maps to it, and a per-instance limit of 64 open ports.

**So one rented machine handing out several containers, each reached on its own port at one shared public address, is the provider's documented ordinary arrangement rather than an assumption.** That is the second half of Fact 61 and it is what the Servers spec's duplicate check — host and port together naming what Smoozy is reaching — rests on.

**Not covered:** the page names **no proxy and no jump host** as a requirement for reaching an instance; it says nothing about any other provider; and it is one page on one date, which for a provider is the reason the date is recorded.

## 10.2 Vast.ai's own SSH page — the connection's own shape

*(Fact 61, gateway half; Row 24.)*

**Source:** Vast.ai's own documentation, the SSH and SCP guide — `https://docs.vast.ai/instances/sshscp`. Read **2026-08-19**. Label: **documented**.

**What it answered.** The connection command shown is a single SSH invocation with a host, a non-standard port and a username — the example given is `ssh -p 20544 root@142.214.185.187` — with a key added to the account. The page states that *"Vast offers both proxy (default) and direct connection methods for SSH"*, with the proxy route working *"on all machines"* and the direct route *"faster and more reliable"*. **Neither route is described as needing its own credentials or a second login step**; both are one host, one port, one username and one key.

**Not covered:** the page does not print the proxy route's own hostnames, and the exact proxy address is shown in the provider's own interface per instance rather than in the documentation. Nothing on it names a jump host, a bastion or a gateway.

## 10.3 RunPod's own SSH page — the connection's own shape, and the mapped port

*(Fact 61, both halves; Row 24.)*

**Source:** RunPod's own documentation, *Connect to a Pod with SSH* — `https://docs.runpod.io/pods/configuration/use-ssh`. Read **2026-08-19**. Label: **documented**.

**What it answered, in its own words:**

> `ssh root@[POD_IP_ADDRESS] -p [SSH_PORT] -i [PATH_TO_SSH_KEY]`

> "For full SSH capabilities, including SCP and SFTP for file transfers, you need to rent an instance that supports a public IP address."

> "While SSH operates on port 22 within your Pod, Runpod assigns a different external port for access."

So the connection is a host, a mapped external port, a username and a key — four fields — and the mapped-port arrangement is documented here too. The page describes a basic SSH route as proxied through the provider's own systems and **names no proxy credential, no jump host and no gateway needing its own authentication**.

**Not covered:** what the proxied route's own address is; any date other than 2026-08-19.

## 10.4 Hyperstack's own SSH page — a direct connection on port 22

*(Fact 61, gateway half; Row 24.)*

**Source:** Hyperstack's own documentation, *Secure Shell (SSH) Access* — `https://docs.hyperstack.cloud/docs/network/ssh/`. Read **2026-08-19**. Label: **documented**.

**What it answered, in its own words:**

> `ssh -i [path_to_ssh_key] ubuntu@[vm_ip_address]`

> "For SSH access, a VM must have a public IP address assigned, as this allows the VM to be reachable over the internet"

> "Hyperstack adds an inbound firewall rule allowing traffic from any IP (0.0.0.0/0) on port 22 via TCP"

So this provider's machines are reached **directly, at the standard port**, with a public address, a username and a key. **The page names no jump host, no bastion, no gateway and no proxy.**

**Not covered:** any provider other than this one; any date other than 2026-08-19; and whether the same holds for that provider's other product lines.

**And the third quoted sentence was falsified on a rental of this same provider three days later.** [The multi-provider batch](../The%20multi-provider%20batch%20—%20the%20surface%20that%20did%20not%20vary,%20the%20Debian%20route%20that%20exists,%20and%20the%20cards%20a%20slice%20cannot%20see.md), 2026-08-22: *"Hyperstack delivered machine C with a floating address and **no inbound SSH** — every login attempt timed out until a security-group rule was created through the seller's own API."* **So the page's *Hyperstack adds an inbound firewall rule* did not hold on the one machine of that seller this project has rented.** Nothing Fact 61 rests on moves — the four fields still reached the machine once the port was open, and no gateway with its own credentials appeared — but **the page is not evidence that a machine of this seller arrives reachable**, and [10.7](#107-the-closed-port-delivery-and-which-seller-it-was) is the observation. **`Servers.md` records it in place from 2026-08-23**, at [C7a](../Servers.md#132-what-the-reading-does-not-cover), which names the falsified sentence, keeps [C1](../Servers.md#131-four-fields-and-fact-61-is-closed) and [C2](../Servers.md#131-four-fields-and-fact-61-is-closed) standing on what the rental confirmed, and forbids reading anything above as a claim that a provider's machine arrives reachable.

## 10.5 What the three providers together do and do not settle

*(Fact 61.)*

**Answered:** at the three providers read on 2026-08-19, **the four fields reach the machine** — host, port, username, key — and **no gateway needing its own details appears in any of their documentation**. Where a proxy route exists it is an address and a port rather than a second set of credentials. And **one machine handing out several containers each on its own port is documented at two of the three**, which is the half the Servers spec previously carried as an assumption.

**Not covered, and stated as the limit it is:** three providers on one date. A fourth provider this project has rented from was not read. Nothing here speaks to a machine at a corporate or academic host reached through a bastion, which is the shape a fifth field would exist for. Provider pages change, which is why the date sits with each entry. [Section 13.2](../Servers.md#132-what-the-reading-does-not-cover) and §16 item 8 carry it in the body.

## 10.6 `ssh_config(5)` — the default port is 22

**Direct publication, cited through a Specs-stage companion.** OpenSSH's `ssh_config(5)` under the `Port` option, **man.openbsd.org**, retrieved **2026-07-17**, recorded at [Preparation/Specs/Sources/Servers.sources.md](../../Specs/Sources/Servers.sources.md): *"The default is 22."* Label: **documented**. Used by [C4](../Servers.md#131-four-fields-and-fact-61-is-closed).

**Not covered:** the manual's default, not any machine's configuration. That a mapped port is the number the user reads off a provider's interface is the arrangement [10.1](#101-vastais-own-networking-page--port-mapping-and-several-containers-on-one-address) and [10.3](#103-runpods-own-ssh-page--the-connections-own-shape-and-the-mapped-port) document; that Smoozy starts the field at 22 is Smoozy's decision and earns no entry.

## 10.7 The closed-port delivery, and which seller it was

**Pointer.** [The multi-provider batch](../The%20multi-provider%20batch%20—%20the%20surface%20that%20did%20not%20vary,%20the%20Debian%20route%20that%20exists,%20and%20the%20cards%20a%20slice%20cannot%20see.md), *What the three sellers turned out to be*, **2026-08-22**. Label: **observed**. Used by §16 item 8's addition.

> "Hyperstack delivered machine C with a floating address and **no inbound SSH** — every login attempt timed out until a security-group rule was created through the seller's own API (`POST /core/virtual-machines/{id}/sg-rules`, `direction ingress`, `protocol tcp`, ports 22–22). **This is a setup step that exists on one seller and not the others**, it is performed through the seller's own interface rather than over the connection, and **Smoozy cannot perform it** — it has no route to a seller's control plane. **A machine that cannot be reached at all is not distinguishable, from Smoozy's side, from a machine that is down.**"

Two further findings from the same passage that bound how much this can be read as a rule: *"How long a machine takes to become usable varies by an order of magnitude"* — about 40 seconds, about 90 seconds and about 7 minutes across the three sellers — and *"**Anything that waits a fixed period for a machine to come up will be wrong on at least one of these three.**"*

**Which seller it was, and the body now says.** §16 item 8's addition called this *"A fifth provider … read on 2026-08-22"* adding *"a shape none of the four had"*, and **both halves were wrong; corrected in the body 2026-08-23**. **The observation is Hyperstack's**, and Hyperstack is one of the three whose own documentation closed Fact 61 on 2026-08-19 ([10.4](#104-hyperstacks-own-ssh-page--a-direct-connection-on-port-22)) — so no fifth provider was read. The seller genuinely unseen before that batch is **Sesterce**, which *"delivered a directly reachable address"*. So the shape belongs to a provider already read, and it contradicts that provider's own page rather than adding a case outside the reading.

**Not covered:** three sellers on one day, one machine each for the reachability observation. Nothing establishes how common the closed-port delivery is, and the remedy — a sentence in the user's own words about a provider's own firewall — is `Preparation/Specs/Servers.md`'s to grant.

## 10.8 The four providers' own connection pages — the forms of the line, and the one that carries no files

*(§13.3 PL3, and PL2's options after the destination. Added 2026-10-07.)*

**Source:** each provider's own documentation, read **2026-10-07** by the builder of step 10c's second notes, piece 15. Label: **documented**, first-party, each for its own provider on that date. The pages' text was saved beside that builder's scratch and is not kept in the project.

**Vast.ai — *SSH Connection*, `https://docs.vast.ai/guides/instances/connect/ssh`** (the address read on 2026-08-19 in [10.2](#102-vastais-own-ssh-page--the-connections-own-shape), `https://docs.vast.ai/instances/sshscp`, now leads here). Its words:

> "Vast offers both proxy (default) and direct connection methods for SSH:"

> "**Proxy SSH**: Works on all machines, slower for data transfer, uses Vast proxy server"

> "**Direct SSH**: Requires machines with open ports, faster and more reliable, preferred method"

The line it shows is `ssh -p 50983 root@154.64.230.50 -L 8080:localhost:8080`. **Two forms, and neither is said to be unable to carry files**: the proxy is called slower for data transfer, which is a statement about transfer, not its absence. The page does not print what a proxy line looks like.

**Runpod — *Connect to a Pod with SSH*, `https://docs.runpod.io/pods/configuration/use-ssh`.** The page's own table gives *Basic SSH* the value **No** under *SCP/SFTP support*, and *Full SSH (public IP)* **Yes**. Its words:

> "All Pods provide a basic SSH connection that is proxied through Runpod's systems. This method does not support commands like SCP (Secure Copy Protocol) or SFTP (SSH File Transfer Protocol)."

> "In the Pod's **Connect** tab, copy the command listed under **SSH**. It should look something like this: `ssh 8y5rumuyb50m78-6441103b@ssh.runpod.io -i ~/.ssh/id_ed25519`"

> "For full SSH capabilities, including SCP and SFTP for file transfers, you need to rent an instance that supports a public IP address and ensure an SSH daemon is running within your Pod."

> "In the Pod's **Connect** tab, copy the command listed under **SSH over exposed TCP**. It should look something like this: `ssh root@213.173.108.12 -p 17445 -i ~/.ssh/id_ed25519`"

and of that second line's shape, `ssh root@[POD_IP_ADDRESS] -p [SSH_PORT] -i [PATH_TO_SSH_KEY]`: *"`[SSH_PORT]`: The designated public SSH port for your Pod."* **So this provider documents a form that cannot carry files — the proxied one, whose host in the provider's own example is `ssh.runpod.io` — and names its direct form as the one under *SSH over exposed TCP*.** And **its direct line puts the port option after the destination**, which is the provider's own documented line for reaching the machine. The page spells the provider *Runpod*.

**Hyperstack — *Secure Shell (SSH) Access*, `https://docs.hyperstack.cloud/docs/network/ssh/`** (the page of [10.4](#104-hyperstacks-own-ssh-page--a-direct-connection-on-port-22), rewritten since). Its words:

> "The VM's **Networking** tab shows a ready-to-copy **Connect over SSH** command, already filled in with your VM's public IP and the default username for its OS image."

> "The command leaves out `-i [path_to_ssh_key]`, so add that yourself if your private key is not in `~/.ssh`."

and the line `ssh -i [path_to_ssh_key] ubuntu@[vm_ip_address]`. **One form, with no port option**, and nothing on the page about a relay.

**Sesterce — *Terminal connection*, `https://docs.sesterce.com/compute-instances/terminal-connection`.** Its words: *"You can then copy your SSH Command to connect the server!"* The command itself appears only in a screenshot of the provider's console, which reads `ssh sesterce@216.81.248.60` under *Command*. The same provider's tutorial *Expose AI model from Hugging Face using vLLM* (`https://docs.sesterce.com/tutorials/expose-ai-model-from-hugging-face-using-vllm`) says: *"When your VM instance is launched, you'll be able to get ssh command to connect into it, like `ssh sesterce@<IP_MACHINE>`."* **One form, with no port option**, and nothing about a relay.

**What the four together settle.** **One provider of the four, Runpod, documents a form of its line that cannot carry files**, and the only thing its own words give to tell that form apart is its host, `ssh.runpod.io`. Vast's proxy form is documented as carrying data, more slowly. Hyperstack and Sesterce document one direct form each. **Two of the four lines carry no port option**, so PL2's port 22 is the ordinary case and not an edge.

**Not covered:** these four providers on this one date; Vast's proxy line itself, which the page does not print; whether every Runpod proxied line uses exactly `ssh.runpod.io` (the page says the line *"should look something like this"*); and the providers' consoles, which were not opened — what was read is their documentation of them.

## 10.9 `ssh(1)` on the desk Mac — the destination, the login option, the port option, and which options take a value

*(§13.3 PL2. Added 2026-10-07.)*

**Source:** the `ssh(1)` manual page on the desk Mac (macOS 26.6.2, `OpenSSH_10.3p1`), read with `man ssh` on **2026-10-07**. Label: **documented**.

The synopsis, which is where PL2 says the value-taking options are read from:

> `ssh [-46AaCfGgKkMNnqsTtVvXxYy] [-B bind_interface] [-b bind_address] [-c cipher_spec] [-D [bind_address:]port] [-E log_file] [-e escape_char] [-F configfile] [-I pkcs11] [-i identity_file] [-J destination] [-L address] [-l login_name] [-m mac_spec] [-O ctl_cmd] [-o option] [-P tag] [-p port] [-R address] [-S ctl_path] [-W host:port] [-w local_tun[:remote_tun]] destination [command [argument ...]]`

So **the options that take a value are `B b c D E e F I i J L l m O o P p R S W w`**, and the letters inside the first bracket take none. The destination:

> "ssh connects and logs into the specified destination, which may be specified as either [user@]hostname or a URI of the form ssh://[user@]hostname[:port]."

> "-l login_name — Specifies the user to log in as on the remote machine."

> "-p port — Port to connect to on the remote host."

> "-i identity_file — Selects a file from which the identity (private key) for public key authentication is read."

**Not covered:** the manual does not say which wins where a line gives a username both before an `@` and with `-l`, or a port both in an `ssh://` destination and with `-p`; nor whether a value may be written joined to its letter (`-p2222`); nor whether options may follow the destination, which the synopsis does not show and Runpod's own documented line does ([10.8](#108-the-four-providers-own-connection-pages--the-forms-of-the-line-and-the-one-that-carries-no-files)). **What Smoozy does in each of those cases is its own decision**, written at PL2, and the user sees the filled fields before *Add*. An option given through `-o` is not read at all.

---

# 11. What has no source in this project, or reads wider than its source

*Every item here is shape 3 or a limit worth naming. None of them stopped the document.*

## 11.1 The round-trip figures behind [K3](../Servers.md#21-the-break-reporting-mechanism) — struck

**Closed 2026-08-23 by striking them.** See [3.1](#31-k3s-second-constraint--the-findings-connectionmd-quotations-struck-and-what-the-constraint-now-rests-on), which keeps the struck strings, why they were struck, and what the constraint stands on instead. `findings-connection.md` is not under `Preparation/` and no figure of it appears anywhere else in the project; the constraint now cites [Six machines](../Six%20machines%20—%20the%20container,%20the%20webhook%20and%20the%20ready%20action.md) Fact 25's own connection-setup sentence. **What the document loses is the round-trip number.**

## 11.2 The kernel keepalive and retry defaults — closed

**Closed 2026-08-23.** See [1.5](#15-the-kernels-own-keepalive-and-retry-ceilings). The page was read at `https://docs.kernel.org/networking/ip-sysctl.html` and **every word the body carried is the page's own**; nothing was contradicted.

## 11.3 `H9a`'s third outcome is this document's construction, and the body now says so

**Repaired in the body 2026-08-23.** [H9a](../Servers.md#82-what-the-reading-is-made-of-and-every-part-of-it-is-a-trap-closed) stated that on the two-card slice *"the cgroup answered nothing at all"* and built a third outcome — *no route at all — neither pair present or both empty* — on it. **What [The multi-provider batch](../The%20multi-provider%20batch%20—%20the%20surface%20that%20did%20not%20vary,%20the%20Debian%20route%20that%20exists,%20and%20the%20cards%20a%20slice%20cannot%20see.md) records is narrower**: the **version-1** paths were empty and do not exist because *"the host is on cgroup v2"*, and *"No cgroup quota was readable"*. **The document records no reading of the version-2 paths — `cpu.max` and `memory.max` — on machines E or F**, so the observation is consistent with all three outcomes and demonstrates none. The body now carries that scope in place and labels the third row as this document's own rule. **The two-shape attempt is documented practice; the third outcome is a construction, and it is safe in the direction it errs** — a stated absence never substitutes the host's figure. Nothing built breaks either way.

## 11.4 The banner claim's reach on Windows — addressed, and one clause still unread

See [8.6](#86-fact-59-windows-half--the-marker-in-microsofts-own-fork). **All four supports carry an address as of 2026-08-23**, the two source files having been read in the fork itself. **What remains unread is the register's own clause**: what was read is the branch behind the GitHub release of the Windows build, whose own `SSH_WINDOWS_BANNER` says `" Win32-OpenSSH-GitHub"`, and the **in-box** build shipped with Windows has never been connected to **and never will be — permanently unresolved, 2026-09-02, on the Creator's ruling of 2026-08-31 that no further machine is rented to answer a question.** **Breaks:** nothing built — [Q15](../Servers.md#103-the-not-linux-check-row-35) takes no decision on the banner at all, and whether to use it is left open to Design and to `Preparation/Specs/Servers.md`.

## 11.5 The *no card* conclusion rests on no cardless machine

See [6.11](#611-fact-46--the-card-states-and-the-misclassification-two-of-three-providers-shipped). [H11](../Servers.md#82-what-the-reading-is-made-of-and-every-part-of-it-is-a-trap-closed)'s *empty* row is the one row that concludes *no card*, and **no rental with no card has ever been posed** — the question cannot be asked on the seller this project rents from, whose search has no CPU-only filter. **That is what could be asked for and not what the seller stocks**, per the narrowing of 2026-08-22 carried into §16 item 2 on 2026-08-23.

## 11.6 Fact 44 — what a machine losing power outright leaves

**No source, and none obtainable today.** Label: **unresolved**. Recorded at [Facts.md](../Facts.md) Fact 44: *"**Posable:** **not at all today.** … The blocker is what is installed rather than what exists: the Creator's Mac has no hypervisor and no container runtime installed at all, and installing free virtualisation software is the Creator's own act."* Used by §16 item 13. **Breaks:** nothing this document owns; it is named because [W28](../Servers.md#116-disconnecting-does-not-stop-the-machines-work-row-44)'s *resumes following* reads a durable cursor and a durable queue.

## 11.7 Fact 17's residuals

**Pointer.** [Six machines](../Six%20machines%20—%20the%20container,%20the%20webhook%20and%20the%20ready%20action.md), Fact 17. Label: **observed**. Used by [T4](../Servers.md#121-what-the-period-owes-a-transfer) and §16 item 14.

> "**the flag that holds a partial off the final name accepted the flag and resumed nothing on the sending direction on one box**, re-sending in full."

> "A hard kill produced it on one box and, on the other, left rsync's own **randomly named temporary file** in the destination instead — a name Smoozy cannot predict. **So it is not deterministic by signal alone**; the source's own guess is that it turns on which side noticed the disconnect first, and that is a guess rather than a finding."

**Not covered:** two boxes, one tool on the sending direction. **Neither residual is about a name Smoozy chose**, which is why `Server protocol and transfers` S4 and S5 make the case not arise.

## 11.8 The container's silent no-ops

**Pointer.** [Six machines](../Six%20machines%20—%20the%20container,%20the%20webhook%20and%20the%20ready%20action.md), Fact 1's narrow clause. Label: **observed**. Used by §16 item 15's quotation.

> "**Answered: yes on both kinds of machine, but by completely different arrangements — and on a container every route a builder would reach for first is a silent no-op.**"

**Not covered:** two machine kinds on two sellers. The consequence §16 item 15 draws — that no later document may read a table in `Servers.md` as a description of rented machines — is a bound this document places on its readers rather than a further fact.

## 11.9 [K2](../Servers.md#21-the-break-reporting-mechanism)'s first route is evidenced on a provider *stop* and never on a *destroy*

**Added 2026-09-02**, out of the scenario replay *The machine is destroyed*, finding 4. Carried in the body as [§16 item 27](../Servers.md#16-unresolved).

**Pointer.** [Six machines](../Six%20machines%20—%20the%20container,%20the%20webhook%20and%20the%20ready%20action.md), Fact 57. Label: **observed**.

> "A provider's stop is an orderly shutdown, not a power cut."

**What the observation covers:** a machine closing politely and a machine powered off from a provider's console, both told to the caller in the same second. **What it does not cover: a machine being *destroyed*.** No test, no reading and no register fact poses that act, and the two are not the same act — a stop leaves the rental standing, a destroy ends it. Under the Creator's ruling of 2026-08-31 no further machine is rented to answer a question, so **this is permanently unresolved rather than a question awaiting a run.**

**What rests on it.** Everything hanging off a break being declared at all: [N1](../Servers.md#31-a-run-and-the-two-figures-in-it)'s repair, [N2a](../Servers.md#31-a-run-and-the-two-figures-in-it)'s condition at 105 seconds, [H1a](../Servers.md#81-three-states-and-the-cheap-implementation-has-two)'s and [W41b](../Servers.md#118-the-machine-runs-and-bills-until-it-is-destroyed-by-hand)'s fall to *last seen*, and `Queue & History` C7's not-updating mark. **What holds without it:** both of K2's routes are built, so a destroy that fires neither the announced ending nor an early keepalive failure still reaches [K3](../Servers.md#21-the-break-reporting-mechanism)'s window between 45 and 60 seconds; nothing anywhere reads which route ran.

## 11.10 Nothing establishes that a night-long idle connection, or a computer waking from sleep, produces a break this mechanism can see

**Added 2026-09-02**, out of the scenario replay *The user queues multiple jobs overnight*, finding 7. Carried in the body as [§16 item 28](../Servers.md#16-unresolved), beside the desktop-clock half at [§16 item 21](../Servers.md#16-unresolved).

**Nothing is quoted here because nothing was read or run.** [1.5](#15-the-kernels-own-keepalive-and-retry-ceilings) carries the kernel's own keepalive and retry ceilings, which are what [K3](../Servers.md#21-the-break-reporting-mechanism)'s count sits above; **no observation anywhere in this project holds a connection open for hours and then asks what it reports**, and none suspends the computer at either end. Label: **unresolved**.

**What breaks if a long-idle or slept connection fails to report.** The failure is a **half-open** connection — the far end gone, the near end never told — which the mechanism cannot report because there is nothing to report. `Queue & History` C7 keys its stamp and its not-updating statement on the moment this document declares the break, so with no declaration a queue reading taken hours earlier goes on being shown as live, which C6 calls the failure the promise exists to prevent. `Desktop app and delivery` P10d's sequence ends at the same declaration and so does [W41b](../Servers.md#118-the-machine-runs-and-bills-until-it-is-destroyed-by-hand)'s *live* state.

**What holds without it.** [K3](../Servers.md#21-the-break-reporting-mechanism)'s keepalive count is written for exactly a connection that stops answering without saying so, and it runs on the primary connection whether or not anything else is using it; the repair is attempt-driven ([N8](../Servers.md#32-the-standing-repair-which-never-ends-while-the-app-is-open)) and picks the machine up within about three minutes of it returning; and nothing durable is written from a machine reading, so a stale one costs a screen and never a record.

**What would close it.** Item 21's throwaway on the Creator's own Mac would answer the sleep half; nothing anybody may now run answers the long-idle half. **Permanently unresolved**, on the ruling of 2026-08-31.

---

## 11.11 What tells a machine with its own kernel from a system container — H9a's split absence row

**Added 2026-09-21, with the R12 change to [H9a](../Servers.md#82-what-the-reading-is-made-of-and-every-part-of-it-is-a-trap-closed).** Label: **documented.** `systemd-detect-virt(1)`, `https://man7.org/linux/man-pages/man1/systemd-detect-virt.1.html`, read 2026-09-21, the page carrying *systemd 262~devel* and dated 2026-08-04. The option: *"--container: Only detects container virtualization (i.e. shared kernel virtualization)."* The exit: *"If a virtualization technology is detected, 0 is returned, a non-zero code otherwise."* The page's container identifiers are `openvz`, `lxc`, `lxc-libvirt`, `systemd-nspawn`, `docker`, `podman`, `rkt`, `wsl`, `proot` and `pouch`. **The same option and the same two sentences were read earlier by the builders of `Rented-server runtime` (2026-09-04) and of this module (2026-09-03)**, and their citations stand in the code at the sites.

**The first process's command name** — `/proc/1/comm` — rests on `proc_pid_comm(5)`, read 2026-09-04 by the builder of `Server Setup` and quoted at `App/Server Setup/src/readings.rs` beside `THE_FIRST_PROCESSS_NAME`: *"This file exposes the process's comm value—that is, the command name associated with the process."* **Not re-read here**; it is the reading `Server Setup` already takes for K1's second row, and H9a takes the same one so the two place a machine alike.

**Not covered, and it is the limit H9a names.** The list is the one on a development page of the current systemd. **An older systemd on a rental may name fewer technologies, and no systemd names a technology it has not been taught.** A container of a technology the tool on that machine does not name, with no marker file and systemd as its first process, reads as a machine with its own kernel. **No machine of that shape is on record.** `Rented-server runtime` §16 item 33 carries it.

# 12. Section 11.8 — the runaway rental, and the running-time reading

**Added 2026-09-02 at the stage-close gate repair.** [11.8](../Servers.md#118-the-machine-runs-and-bills-until-it-is-destroyed-by-hand) was written earlier the same day and this file, edited that morning for [H16](../Servers.md#83-where-the-reading-is-taken-and-why-no-blender-is-in-it), carried no section for it — while [W40](../Servers.md#118-the-machine-runs-and-bills-until-it-is-destroyed-by-hand) asserts in this document's own voice that a rented machine goes on being billed until somebody destroys the rental by hand, and [W42](../Servers.md#118-the-machine-runs-and-bills-until-it-is-destroyed-by-hand) restates an observation with no label. **Both are entered here and neither is re-quoted from scratch: the citations have one home and it is `Rented-server runtime`'s.**

## 12.1 That nothing Smoozy places on a machine can end its rental — W40

**Pointer.** [Rented-server runtime](../Rented-server%20runtime.md#12-health-readings) **V13**, whose citations are entered whole at [that document's companion](Rented-server%20runtime.sources.md), *2.16 — Vast.ai's own instance pages and `pid_namespaces(7)`*, both read **2026-08-20**. Label: **documented**, first-party, on two independent grounds neither of which carries the claim alone.

**The seller's half.** Vast.ai's create-instance reference, `https://docs.vast.ai/api/create-instance`, **establishes by absence** — it lists the request body in full and carries no duration field, so a cap sent on creation is an unknown field; the companion show-instance page carries a duration as a **read-only** number of seconds. And Vast.ai's Instances FAQ, `https://docs.vast.ai/documentation/reference/faq/instances`, under *"What does 'Lifetime' mean on my instance?"*:

> "This rental end date is locked when you rent and cannot be changed by the host. When the rental end date is reached, the rental contract expires and the instance is stopped."

**The kernel's half.** `pid_namespaces(7)`, `https://man7.org/linux/man-pages/man7/pid_namespaces.7.html`:

> "Only signals for which the "init" process has established a signal handler can be sent to the "init" process by other members of the PID namespace."

> "SIGKILL or SIGSTOP are treated exceptionally: these signals are forcibly delivered when sent from an ancestor PID namespace. Neither of these signals can be caught by the "init" process."

**So a watchdog inside the container cannot end the rental**, and a cap asked for on creation is not a field the seller accepts — which is the whole of what W40's statement to the user rests on. **The withdrawal rides with it:** that companion records a watchdog whose kill *cannot land*, whose command was never recorded, and whose outcome was *"written in the future tense and never checked"*; nothing here rests on it.

**Not covered:** **one seller for the cap** — the kernel rule is general, the provider rule is Vast's alone, and no other provider's terms were read for it. And the billing question that document leaves open: *what a container whose first process has exited is charged while the instance is neither stopped nor destroyed*, which wants a page Vast has not published or a live rental left in that state. **Under the Creator's ruling of 2026-08-31 neither is coming**, and W40 is written to what is established rather than to what a further reading might have added.

## 12.2 The container's uptime is the host's — W42

**Pointer.** [The last two boxes](../The%20last%20two%20boxes%20—%20the%20hook%20that%20cannot%20be%20read%20back,%20the%20full%20disk,%20and%20the%20unit%20that%20kills%20a%20render.md), its host-inherited-readings paragraph, entered at [Rented-server runtime's companion](Rented-server%20runtime.sources.md) under V13. Label: **observed in a test**. Its words:

> "Free-memory reporting shows **4 GiB of swap** that is the host's, and the uptime reading gives the host's 8.2 days. **Both would lie to a reader.**"

**The container was created that morning**, which is what makes 8.2 days the host's figure and not a slow clock. **W42 now carries the label in place**, which it did not until this entry was written.

**Not covered, and it is the limit W42 is written inside:** **one container on one seller**, and **the swap half is not carried across** — W42 is about the running-time reading alone, and no clause in this document reads free memory. Nothing here establishes what any other container kind reports, so W42's rule is a kind test followed by a stated absence rather than a claim about a number.

---

*First written 2026-08-23 against the brief for this file, when nothing was looked up outside `Preparation/` and three entries carried no address of their own. **Revised the same day against the repair brief, and those three are closed.** [1.5](#15-the-kernels-own-keepalive-and-retry-ceilings) and [8.6](#86-fact-59-windows-half--the-marker-in-microsofts-own-fork) were addressed by reading the publications themselves — the kernel's own `ip-sysctl` page and the two files in Microsoft's own fork — and every word the document carried was found in them unchanged. [3.1](#31-k3s-second-constraint--the-findings-connectionmd-quotations-struck-and-what-the-constraint-now-rests-on) had no publication to go to, being an observation whose only record is outside `Preparation/`; **its quotations are struck and the entry keeps them, why they went, and what the constraint stands on instead.***
