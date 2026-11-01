# The computer itself — its name, its identity, and its clock

What Apple's, Microsoft's, systemd's, the Linux man-pages project's, the kernel's and the Open Group's own documentation answered on **2026-08-16** about three things a program might ask of the machine it is running on: whether the machine can be recognised again from away, what the machine and its login account are called, and whether elapsed time can be read in a way that winding the clock back does not defeat.

An **input** to the stage, like the batches before it. Nothing here is a decision, a plan or a recommendation about what the application should do. Where a route is closed, that is recorded as a fact; which route to take is not recorded, because that is not this document's business.

It answers three facts — **11**, **12** and **13** — assembled from three findings files, each independently citation-checked on the same day by a fresh agent who re-fetched every address, re-read every manual page with `man`, and re-ran every observation on the same machine. Every claim carries a label, and there are only four: **documented**, **observed**, **inferred**, **unresolved**. The labels are carried through exactly as the checks left them. Where a check relabelled something from documented to inferred, it is inferred here. Where a check cut a claim, it does not appear here at all. **"Inferred" means the vendor is silent, and silence is not denial** — that distinction is load-bearing throughout and downstream readers must treat the two differently.

The claim-by-claim verdicts live in the run's trail, `Audits/Architecture/The computer itself/`. Everything else a builder would need is reproduced here in full: every load-bearing quote, every address, every observed value's format. Values that identify the machine everything was observed on are `[redacted]`; only formats are recorded. Rule 1's plain-English rule is suspended, per the stage's exception — a paraphrase would be ambiguous to the thing that will build from this.

---

## The limits, before anything else

### One. No Windows machine and no Linux computer has run any of this — a Windows machine can be rented, and every one this project has read reported a Server edition

**Every Windows claim and every Linux claim in this whole document is documentation alone.** Nothing in any Windows section and nothing in any Linux section was executed, tested or observed. No Windows or Linux claim anywhere below is labelled observed, and none should be read as if it were. This applies to all three facts without exception, and it is not repeated at every turn.

### Two. Everything observed was observed on one machine

macOS **[redacted]**, build **[redacted]**, arm64, **[redacted]**, model identifier `[redacted]`. All observations were made as **uid 501** (non-root), with no `sudo`, in a non-interactive shell with no controlling terminal, on 2026-08-16. The machine is Apple silicon and unrepresentative of Intel Macs; Intel Macs are documentation-only here too.

A control test establishes that a privilege failure is visible in this environment rather than swallowed by a permissive sandbox:

```
$ head -c 20 /etc/master.passwd
head: /etc/master.passwd: Permission denied
$ ls -l /etc/master.passwd
-rw-------  1 root  wheel  10004 Jun 25 04:29 /etc/master.passwd
```

Reproduced identically by the check, byte size and timestamp included. So the "no privilege required" observations below are not an artefact of the environment: an actually-root-only read does fail, and fails visibly.

### Three. Forgeability was never tested, on any system

Nobody attempted to make one machine report another's identifier. Apple states in prose that I/O Registry properties are spoofable and that `gethostuuid`'s inputs "can themselves be modified"; Microsoft states that a local administrator can spoof a software KSP as a TPM KSP; systemd documents a supported kernel-command-line override of the machine ID. **All three are documented assertions, not demonstrations.** What would settle it: three disposable machines, one per operating system, permission to modify them, and a controlled attempt to make machine B report machine A's identifier.

### Four. Three further things were deliberately not done

- **Nothing on the machine was changed.** Nothing installed, renamed or configured. No `scutil --set`, no System Settings edit, no clock set, no time-synchronisation setting touched. Consequently the **rename** half of Fact 12's steadiness question is documentation-only on all three systems including macOS, and **the whole of Fact 13's central question — what happens to a reading when the system time is set backwards — is documentation-only on every system including this one.** No claim below about a wound-back clock is observed.
- **No Secure Enclave key was created**, even a non-permanent one, and the keychain was not touched.
- **TCG's own specification PDFs** (EK Credential Profile, TPM 2.0 Library) were not fetched on 2026-08-16, and **when the fetch was attempted on 2026-08-20 the site refused it with HTTP 403** — so this one is no longer a deliberate omission and it is recorded among the refusals in §5. *Corrected 2026-08-22; this bullet previously said only that they "were not fetched", which reads as a choice.* Every TPM claim below comes from Microsoft Learn, the Linux kernel's documentation site, TCG's published reference implementation, or the tpm2-software project's own files, and each says which.

### Five. Sources that refused to serve

Recorded so nobody silently re-attempts them, and because two of them refused only one of the two agents.

- `https://www.kernel.org/doc/Documentation/ABI/testing/sysfs-class-dmi-id` — **HTTP 404**, reproduced exactly (nginx 404, 146 bytes).
- `https://docs.kernel.org/ABI/testing/sysfs-class-dmi-id` — **HTTP 404**, reproduced exactly.
- `https://raw.githubusercontent.com/torvalds/linux/master/Documentation/ABI/testing/sysfs-class-dmi-id` — **HTTP 404**; the file does not exist upstream. Reproduced.
- `https://docs.kernel.org/admin-guide/abi-testing.html` — HTTP 200, **3,270,635 bytes**, fetched and searched. The strings `product_uuid`, `/sys/class/dmi`, `dmi-id` and `dmi/id` each occur **zero** times in it. Reproduced to the byte, all four counts zero.
- → **Conclusion, hedged as the check left it: the Linux kernel appears to publish no prose ABI documentation for `/sys/class/dmi/id/*`.** This is an absence established by four failed lookups, not by a kernel statement. What stands in for it is the kernel's own *source*, flagged as source-not-prose where it is used.
- `https://trustedcomputinggroup.org/resource/tpm-library-specification/`, and the TPM 2.0 Library Specification PDFs linked under it — **HTTP 403**, on a plain fetch and again with a browser user-agent string, 2026-08-20. This is the address Unresolved 11 item 16 and U6 both name as what would settle the TPM questions, and it is not reachable from here; what stands in for it is TCG's own published reference implementation of the TPM, which is. *Added 2026-08-22: the refusal happened on 2026-08-20 and was written into the closing summary but never into this section, which the closing summary said it had been.*
- `developer.apple.com` **HTML is not readable by the fetch tool** — the pages render client-side and a fetch returns only the `<title>`. The check reproduced this on both a documentation page and `support.apple.com/en-us/102547`. Apple's API text below was taken instead from Apple's own documentation JSON endpoint under `developer.apple.com/tutorials/data/…`, which is the same documentation served as data — the JSON carries that page's own `identifier.url`, so it is not a third party. Addresses are given per quote.
- **No current Microsoft Learn reference page could be found for the "duplicate name exists on the network" behaviour.** Searching `learn.microsoft.com` surfaces Microsoft Q&A threads, archived MSDN/TechNet forum threads, and one archived Microsoft-authored event-message page from a retired documentation set (`learn.microsoft.com/en-us/previous-versions/ms841516`) — none of which meets the bar. RFC 1001's own text is quoted instead. **Microsoft's naming-conventions page does not designate RFC 1001 as governing**: it links RFC 1001 and RFC 1002 once, under NetBIOS names ▸ Special characters ▸ Period, "For more information about NetBIOS scopes".
- **Windows local (non-domain) account-name character and length rules were not found** in Microsoft's own reference documentation. The domain-side `sAMAccountName` rules were, and are quoted.

### Six. Where the two runs disagreed, both stand, and neither is adjudicated

Seven readings came back differently on the second run. Each is a real finding, recorded side by side. In no case was a winner picked, because the checker was not present for the first run.

| # | Reading | First run | Re-run |
|---|---|---|---|
| 1 | `https://www.freedesktop.org/software/systemd/man/latest/machine-id.html` (Fact 11) | **HTTP 418**, anti-bot interstitial titled `Checking you are not a bot`; `WebFetch` **403 Forbidden**, no body | **HTTP 200**, full rendered manual, `<title>machine-id</title>`, systemd 261.2 |
| 2 | `…/sd_id128_get_machine.html` (Fact 11) | **HTTP 418**, same interstitial | **HTTP 200**, full page |
| 3 | `https://git.kernel.org/…/plain/Documentation/ABI/testing/sysfs-class-dmi-id` | **HTTP 200 but the body was an anti-bot challenge**, title `Making sure you're not a bot!`, containing an `anubis_challenge` JSON block; no document content served | **HTTP 404** (2,928 bytes of cgit's own not-found page) — both outcomes agree that no document content was served |
| 4 | `https://git.savannah.nongnu.org/cgit/dmidecode.git/plain/man/dmidecode.8` | **HTTP 502**; the `dmidecode(8)` man page was not read, and no claim rests on it | **301 → HTTP 200** at `cgit.git.savannah.nongnu.org`, 10,984 bytes of real roff (`.TH DMIDECODE 8 "June 2025"`). The page is reachable; the source is not in fact unavailable |
| 5 | `scutil --get HostName` exit status (Fact 12) | exit status **0** | exit status **1**. Confined to the one preference that is unset; `ComputerName` and `LocalHostName` returned 0 in both runs, and the strings, the absence of a prompt, the absence of elevation and uid 501 all reproduced exactly |
| 6 | `DCDevice` token byte-count (Fact 11) | `token byte-count=2212` | **2213**, on each of two calls. The first run's own later observation also records 2213 twice, so that file was internally inconsistent by one byte. **The practical reading: the token's length is not a fixed property and should not be cited as one** |
| 7 | The six `freedesktop.org` addresses used for Fact 13 | **HTTP 200**, no refusals recorded | **HTTP 418** with an anti-bot interstitial (`<title>Checking you are not a bot</title>`, a "goaway" meta-refresh challenge) to a command-line fetch. The check did not attempt to defeat the challenge; it read the six pages in a real browser instead, where they served normally and every quotation verified word for word at systemd 261.2 |

Two Fact 13 observations also differed in *tightness* rather than in outcome, and are recorded in place in the Fact 13 Observed section below: the spread across four clocks over a five-second sleep (~10 µs versus ~78 µs), and the value of `kern.monotoniclock_offset_usecs` (`1669110948808898` versus `1669110948769134`, a change of −39,764 µs ≈ −0.0398 s). Both are recorded and neither is adjudicated.

---

## Fact 11 — an identity that survives being taken away

*Whether each supported operating system offers something that identifies a computer again from away, how far it survives a reinstall, a move and a person with full control of the machine, and what it takes to read it.*

### Documented — macOS

#### What an ordinary application can read

**`IOPlatformSerialNumber`** — an I/O Registry property of the `IOPlatformExpertDevice` node.

> "However, starting with Panther, the serial number is presented in the property IOPlatformSerialNumber of the IOPlatformExpertDevice node in the I/O Registry. The I/O Registry accessor APIs in IOKit.framework can be used to retrieve this property from an application."
> — Apple, *Technical Note TN1103: Uniquely Identifying a Macintosh Computer*, `https://developer.apple.com/library/archive/technotes/tn1103/_index.html`, read 2026-08-16.

That technote is archived and carries Apple's own banner: "This document is no longer being updated." Its revision history's last entry is `2011-07-18` ("Minor formatting changes"). The constant is still shipped and still listed in current documentation: `kIOPlatformSerialNumberKey`, declared `var kIOPlatformSerialNumberKey: String { get }`, macOS 10.3+, at `https://developer.apple.com/documentation/iokit/kioplatformserialnumberkey` (read 2026-08-16 via the corresponding `.json`). **Apple's current page for this constant carries an empty abstract** — `abstract` is `null` in the JSON, with no discussion section at all. No description of what it identifies or guarantees.

**`IOPlatformUUID`** — `kIOPlatformUUIDKey`, declared `var kIOPlatformUUIDKey: String { get }`, macOS 10.5+, at `https://developer.apple.com/documentation/iokit/kioplatformuuidkey` (read 2026-08-16 via the corresponding `.json`). **Its abstract is also empty.** Apple documents that the constant exists and nothing about what it means, how stable it is, or whether it can be changed.

**`gethostuuid(2)`** — a system call, documented in Apple's own manual page shipped on macOS [redacted] (`man 2 gethostuuid`, read 2026-08-16, page dated `November 5, 2008`):

> "gethostuuid - return a unique identifier for the current machine"

> "The gethostuuid() function returns a 16-byte uuid_t specified by id, that uniquely identifies the current machine.  Be aware that the hardware identifiers that gethostuuid() uses to generate the UUID can themselves be modified."

**Primary built-in MAC address** — TN1103:

> "The Ethernet MAC (media access control) address for the primary built-in Ethernet (or Wi-Fi, on systems without Ethernet ports, such as MacBook Air) interface is the closest fit to these requirements available on Macintosh computers."

**The antecedent matters and the check restored it.** "These requirements" are TN1103's own **Digital Rights Management** requirements, stated three sentences earlier: "The identifier must be unique… The identifier must be stable… The identifier should not be easily changed." Apple is calling the MAC address the closest fit **to a DRM identifier's requirements**, not endorsing it generally.

**None of the four is documented as requiring elevated privilege or as prompting the user.** This is an absence claim, measured: TN1103 was searched for `privilege|administrator|admin|root user|sudo|elevat` — eight hits, none of them a privilege requirement for reading an identifier (they concern asset-tracking administrators, the `lladdr` admin user, and disabling mDNSResponder). The two constant pages have no prose at all. The `gethostuuid` man page documents only `[EFAULT]` and `[EWOULDBLOCK]`. Apple documents no privilege requirement either way.

#### What Apple documents about stability

**Serial number, hardware replacement and irrecoverability** — TN1103, under its heading "Caveats for using the serial number":

> "It is possible for a system to lose its serial number so that it will no longer appear either in System Profiler or the I/O Registry. Repairing a system by swapping hardware components is one reason this can happen. Apple does not document the specific details of how a machine can lose its serial number. Once the serial number has been lost there is no means to restore it to the machine. Apple does not guarantee that all future systems will have a software-readable serial number."

And on its format:

> "Important:  Developers should not make any assumptions about the format of the serial number such as its length or what characters it may contain."

**MAC address and board replacement** — TN1103:

> "The primary MAC address is associated with a particular system main logic board. So while the address is stable, it will change should the system's main logic board need to be replaced."

**Reinstalling the operating system.** **No Apple statement was found** about whether `IOPlatformSerialNumber`, `IOPlatformUUID` or `gethostuuid()` survive an OS reinstall. Nothing on TN1103, on either constant page, or on the man page addresses reinstall. See Unresolved.

**Restoring onto different hardware, and migration.** Apple documents this only for its attestation-adjacent keys, not for the identifiers above:

> "The keys that you generate remain valid through regular app updates, but don't survive app reinstallation, device migration, or restoration of a device from a backup. In these cases, you need to start the process from the beginning and generate a new key."
> — Apple, *Establishing your app's integrity*, `https://developer.apple.com/documentation/devicecheck/establishing-your-app-s-integrity`, read 2026-08-16.

> "Note that the metric can grow if a user reinstalls your app, restores from a backup, or transfers a device to another user. For privacy reasons, App Attest keys stored on device don't survive these events, forcing your app to generate a new key on the same device."
> — Apple, *Assessing fraud risk*, `https://developer.apple.com/documentation/devicecheck/assessing-fraud-risk`, read 2026-08-16.

And for Managed Device Attestation's hardware-bound keys:

> "These hardware-bound keys are automatically removed when erasing or restoring a device. Because the keys are removed, any configuration profiles relying on those keys won't work after a restore."
> — Apple, *Managed Device Attestation for Apple devices*, `https://support.apple.com/guide/deployment/managed-device-attestation-dep28afbde6a/web`, read 2026-08-16 (page footer: "Published Date: July 29, 2024").

**Moving to a different network — Apple documents nothing either way.** The check cut a claim here that had asserted a positive documented finding of network-independence; no Apple page read states that. What is on the page is one adjacent statement, about netbooting affecting *which* built-in port is primary (TN1103: "Netbooting introduces a wrinkle with systems with multiple built-in Ethernet ports"). A gloss reading "not applicable to current single-NIC Macs" was cut with it: Apple does not say it, and this project has no survey of current Mac network hardware. **What stands is only the absence: no statement was found that any of these identifiers is derived from network state, and none that any of them is independent of it.**

#### Forgeability, in Apple's own words

**Documented: any I/O Registry property can be spoofed by knowledgeable persons.** Apple's word is **"spoofed"** and the actor Apple names is **"knowledgeable persons"**. The word *administrator* appears in Apple's text at exactly one place — the `ifconfig lladdr` sentence below — and that sentence is about the `getifaddrs` and CFUUID routes, **not** about the I/O Registry properties. So it is **not documented** that an administrator specifically can rewrite `IOPlatformSerialNumber` or `IOPlatformUUID`.

TN1103, on the MAC address route:

> "On Panther, the ifconfig utility supports the lladdr option to set a locally-administered MAC address for an interface. Both getifaddrs and the CFUUID APIs will return the locally-administered MAC address if one exists. A DRM product that relied on these APIs would be vulnerable to an admin user using ifconfig to specify the same MAC address on multiple machines."

> "Finally, it is possible for the built-in MAC address (or any I/O Registry property for that matter) to be spoofed by knowledgeable persons. If a more robust level of security is required, such as in cases where the licensed assets have a high monetary value, developers may want to investigate the numerous hardware security tokens that are available."

That parenthesis — **"or any I/O Registry property for that matter"** — reaches `IOPlatformSerialNumber` by Apple's own words, since TN1103 itself calls the serial number a property "in the I/O Registry". **Its reach to `IOPlatformUUID` is inferred, not documented**: `IOPlatformUUID` is **never mentioned in TN1103 at all** — zero occurrences in the whole document, checked. Treating it as covered rests on `kIOPlatformUUIDKey` being an I/O Registry key, which Apple's constant page states without any prose and which the observation below confirms on this machine — not on anything TN1103 says.

`gethostuuid(2)`'s man page says something adjacent about its inputs — "the hardware identifiers that gethostuuid() uses to generate the UUID can themselves be modified" — and **note that it does not say by whom, or at what privilege**.

The `lladdr` capability is still documented on this machine's own `ifconfig(8)` man page (read 2026-08-16):

> "     ether   Another name for the lladdr parameter."

> "     lladdr addr
>              Set the link-level address on an interface.  This can be used to
>              e.g. set a new MAC address on an ethernet interface, though the
>              mechanism used is not ethernet-specific."

#### The hardware-backed routes Apple documents, and what each demands

**Route 1 — App Attest (`DCAppAttestService`). Documented, hardware-backed, and documented as unavailable on Mac.**

The mechanism is exactly the shape this fact asks about:

> "You can't rely on your app's logic to perform security checks on itself because a compromised app can falsify the results. Instead, you use the shared instance of the DCAppAttestService class in your app to create a hardware-based, cryptographic key that uses Apple servers to certify that the key belongs to a valid instance of your app."

> "The device automatically stores the associated private key in the Secure Enclave, from where the App Attest service can use it to create signatures, but from where no process can ever directly read or modify it, ensuring its security."
> — *Establishing your app's integrity*, address above, read 2026-08-16.

Its availability badge lists `macOS 11.0+`. **The property that gates it says otherwise:**

> "If you read isSupported from an app running on a Mac device, the value is false. This includes Mac Catalyst apps, and iOS or iPadOS apps running on Apple silicon."
> — Apple, *DCAppAttestService.isSupported*, `https://developer.apple.com/documentation/devicecheck/dcappattestservice/issupported`, read 2026-08-16 (via the corresponding `.json`).

Requirements it would impose if it were available: "To use the App Attest service, your app must have an App ID that you register on the [Apple Developer] website" (*Establishing your app's integrity*); a server-side component to hold challenges and verify attestation objects; and, for the fraud metric, a DeviceCheck-enabled key: "Be sure to enable the DeviceCheck service when requesting the cryptographic key **that you use to encrypt the App Attest token**" (*Assessing fraud risk*) — the emphasised object is what scopes the requirement, and was restored by the check after being trimmed away.

**A correction the check made and which matters for anyone reaching for endpoints.** Two base URLs were originally attributed to *Establishing your app's integrity*. **Neither string appears on that page.** Both are on ***Assessing fraud risk***, and they are not App Attest's endpoints in general — they are the base URLs for the `attestationData` **receipt-refresh** call that maintains the fraud-risk metric. Apple's sentence in full: "Use the base URL of `https://data-development.appattest.apple.com` shown in the example above for testing. This URL accesses the sandbox environment that your app uses during development. To work with apps that you've distributed through the App Store, TestFlight, or with an Enterprise Developer certificate, use a base URL of `https://data.appattest.apple.com` instead." (*Assessing fraud risk*, read 2026-08-16.)

**Route 2 — DeviceCheck (`DCDevice`). Available on macOS, but Apple documents no forgery-resistance property for it.**

`DCDevice`: "A representation of a device that provides a unique, authenticated token." Availability includes `macOS 10.15+`. (`https://developer.apple.com/documentation/devicecheck/dcdevice`, read 2026-08-16.)

> "Generates a token that identifies the current device."

> "`token`:  An ephemeral token that identifies the current device."

> "Your server uses the generated token in its requests to get or set the persistent bits for the current device. You should treat the token you receive in the completion block as single-use."

> "Note: The app you use to generate the token must be associated with your developer account; otherwise, the generation request fails."
> — Apple, *DCDevice.generateToken(completionHandler:)*, `https://developer.apple.com/documentation/devicecheck/dcdevice/generatetoken(completionhandler:)`, read 2026-08-16.

What Apple's server side offers, per *Accessing and modifying per-device data* (`https://developer.apple.com/documentation/devicecheck/accessing-and-modifying-per-device-data`, read 2026-08-16): "Use a token from your app to query and modify two per-device binary digits stored on an Apple server." Three endpoints — `/v1/query_two_bits`, `/v1/update_two_bits`, `/v1/validate_device_token` — under `https://api.devicecheck.apple.com` (production) or `https://api.development.devicecheck.apple.com` (development). The query response fields are `bit0`, `bit1` and `last_update_time` ("The date of the last modification, in YYYY-MM format"). Authentication is a JWT: "The authentication key must use the ES256 algorithm and be in the Base 64 URL–encoded JSON web token format", created by "the same steps you use to create a provider authentication token for Apple Push Notification service (APNs)".

**Cost and account requirements, documented:** a developer account (the `generateToken` note above), and an APNs-style ES256 signing key obtained from that account. Apple's DeviceCheck pages **do not** state a distribution-channel restriction for `DCDevice`, unlike App Attest's production URL note — measured: the `DCDevice`, `generateToken`, `DCDevice.isSupported` and per-device-data pages carry zero occurrences of "App Store", "TestFlight" or "Enterprise".

**What Apple does *not* document about `DCDevice`.** Apple does not say the token is Secure-Enclave-backed, does not say it is unforgeable, and does not describe what `validate_device_token` actually proves. The word "authenticated" in the class abstract is the strongest claim, and it is unexplained. **Treat "DCDevice resists forgery by the machine's administrator" as undocumented — not documented-false, and not documented-true.**

This is a claim about what a vendor does *not* say, so the check measured it and its scope is written in. The DeviceCheck framework's own index lists eleven documentation pages — `DCDevice`, `DCDevice.isSupported`, `generateToken(completionHandler:)`, `DCAppAttestService`, `DCAppAttestService.isSupported`, `DCError`, `DCErrorDomain`, *Establishing your app's integrity*, *Validating apps that connect to your server*, *Assessing fraud risk*, *Attestation object validation guide*, and *Accessing and modifying per-device data*. All were fetched (HTTP 200) and searched on 2026-08-16. Across the whole set: **"Secure Enclave" occurs exactly once**, on *Establishing your app's integrity*, and it is about App Attest keys, not `DCDevice`. **"unforgeable", "tamper", "spoof" and "forge" occur zero times anywhere in the set.** The per-device-data page's only words about validation are "To validate a device, you create a JSON payload…". The absence claim stands, over that scope.

**Route 3 — Managed Device Attestation. Unforgeable by design in Apple's own words, and out of reach of a website-downloaded app by inference.**

> "Managed Device Attestation provides strong evidence about which properties of a device can be used as part of a trust evaluation. This cryptographic declaration of device properties is based on the security of the Secure Enclave and the Apple attestation servers."

> "Managed device attestation helps protect against the following threats: A compromised device lying about its properties … A compromised device sending a different device's identifiers … Private key extraction for use on a rogue device"

> "This attestation provides strong assurances that the properties of the device (for example, the serial number) are legitimate and not spoofed."
> — *Managed Device Attestation for Apple devices*, address above, read 2026-08-16.

A fourth line originally sat with those three and was moved by the check, because it comes from a different section: "In the last two cases, the Apple attestation servers refuse to issue an attestation for properties that they can't verify." It is verbatim, but it belongs to the page's later **"Handle failed attestations"** section, and "the last two cases" refers to the last two entries in a list of *reasons an attestation request may fail* — "The device hardware or software may be compromised" and "The device isn't genuine Apple hardware". Placed under the threat list it read as though "the last two cases" meant the last two threats. With its real antecedent, Apple's point is narrower: when an attestation *fails* for those two reasons, the servers refuse rather than attest. The same section adds: "There is no trustworthy way for the device management service to know the exact cause of a failed attestation."

What it requires, in Apple's words on the same page:

- Hardware: "Mac computers: With Apple silicon." (iPhone/iPad/Apple TV need A11 Bionic or later.)
- Operating system: "a feature for devices with iOS 16, iPadOS 16.1, **macOS 14**, tvOS 16, or later."
- A **device management service** (MDM) and/or an **ACME-issuing Certification Authority**: "An organization's issuing Certification Authority (CA) ACME service can request an attestation of the enrolling device's properties"; alternatively "a device management service can issue a DeviceInformation query requesting a DevicePropertiesAttestation property."
- A server-side verifier: "The device management service can then validate the response by evaluating that the certificate chain is rooted with the expected Apple Certificate Authority (available from the Apple Private PKI Repository)".
- A rate limit: "usage is limited currently to one DeviceInformation attestation per device every seven days."
- A privacy carve-out: "Serial number and UDID are both omitted when using User Enrollment to protect the user's privacy."

**Apple documents only two request paths, and both are organisational.** Among what ACME payload attestation validates, Apple lists "The device is managed by the organization's device management service."

**Inferred-closed, not documented-closed.** The original wording — "There is no documented path by which an application the user downloaded from a website, not enrolled in any MDM, obtains a Managed Device Attestation" — shut the door harder than Apple does. **Apple nowhere states that no other path exists, and nowhere addresses an unenrolled application.** The door is shut here by the vendor describing only enterprise scenarios, which is not the same as the vendor denying the other one. Silence is not denial.

#### What it takes to read each thing in practice — macOS

- **`IOPlatformSerialNumber`:** `IOServiceGetMatchingService(kIOMasterPortDefault, IOServiceMatching("IOPlatformExpertDevice"))` then `IORegistryEntryCreateCFProperty(platformExpert, CFSTR(kIOPlatformSerialNumberKey), kCFAllocatorDefault, 0)` — the exact call sequence given in TN1103's Listing 1. **Narrowed by the check:** that listing is titled "Reading the **IOPlatformSerialNumber** property" and reads only the serial; applying the same sequence to `IOPlatformUUID` is this project's extension, not TN1103's. Command-line reader: `ioreg -rd1 -c IOPlatformExpertDevice` — **this comes from the observation below, not from TN1103, which never mentions `ioreg`.**
- **`gethostuuid()`:** `#include <unistd.h>`; `int gethostuuid(uuid_t id, const struct timespec *wait);` — returns 0 on success, −1 on error; documented errors `[EFAULT]` and `[EWOULDBLOCK]`.
- **MAC address:** TN1103 names three routes — the I/O Registry (sample code `GetPrimaryMACAddress`), `getifaddrs`, and the CFUUID APIs — and rules the latter two out for identity purposes because they return a locally-administered address if one has been set: "Neither the getifaddrs nor the CFUUID methods are suitable for DRM purposes".
- **`DCDevice`:** `DCDevice.current.generateToken { token, error in … }`, framework `DeviceCheck`.

### Documented — Windows

*(Documentation only. No Windows machine has run it. Nothing here was run.)*

#### What an ordinary application can read

**`Win32_ComputerSystemProduct.UUID`** (WMI, `root\CIMV2`):

> "UUID
> Data type: string
> Access type: Read-only
> Qualifiers: MappingStrings ("SMBIOS|Type 1|UUID")
> Universally unique identifier (UUID) for this product. A UUID is a 128-bit identifier that is guaranteed to be different from other generated UUIDs. If a UUID is not available, a UUID of all zeros is used.
> This value comes from the UUID member of the System Information structure in the SMBIOS information."
> — Microsoft, *Win32_ComputerSystemProduct class*, `https://learn.microsoft.com/en-us/windows/win32/cimwin32prov/win32-computersystemproduct`, read 2026-08-16.

**`Win32_BIOS.SerialNumber`**:

> "SerialNumber
> Data type: string
> Access type: Read-only
> Qualifiers: Maxlen (64), Mappingstrings ("MIF.DMTF|ComponentID|001.4")
> Assigned serial number of the software element.
> This property is inherited from CIM_SoftwareElement."
> — Microsoft, *Win32_BIOS class*, `https://learn.microsoft.com/en-us/windows/win32/cimwin32prov/win32-bios`, read 2026-08-16.

Note that Microsoft's own text for this property says "software element" — it is inherited boilerplate, not a hardware-identity guarantee, and the trailing inheritance sentence confirms that reading.

**`Windows.System.Profile.SystemIdentification.GetSystemIdForPublisher()`** — the strongest documented ordinary-application identifier, and the one with an explicit stability contract:

> "Gets an identifier value for the system based on the app publisher ID."

> "The identifier returned by this method is specific to the app publisher on the current device."

> "The ID has the following characteristics:
> Unique for each system
> On any particular system, all apps by the same publisher will get the same value for this ID (for all users). Conversely, apps by different publishers on the same system will obtain different IDs.
> Can be created offline
> Persists across restarts, reinstalls and upgrades of Windows, including clean installs (please see below for exceptions)
> Persists across most hardware modifications
> Available in OneCore"

> "The method will first attempt to use the Trusted Platform Module (TPM), if present, to get an ID. If a TPM is not present, the method will try to get an ID from the Unified Extensible Firmware Interface (UEFI). If neither of these sources is available, this method will return an ID that is backed by the Windows registry. In the case of the Windows registry, the ID will not satisfy all the above guarantees. For example, if a system does not have a TPM or UEFI support, and thus an ID was obtained from the registry, a clean install of Windows will result in a new, different ID being returned. Callers of this method should refer to the Source property of the returned SystemIdentificationInfo to determine where the ID was obtained from in order to understand the guarantees provided."
> — Microsoft, *SystemIdentification.GetSystemIdForPublisher Method*, `https://learn.microsoft.com/en-us/uwp/api/windows.system.profile.systemidentification.getsystemidforpublisher`, read 2026-08-16.

**The clause a website-distributed binary must read carefully**, from the class page:

> "Note
> The system identification string is created using the calling app's publisher ID and an identifier derived from the system's firmware/hardware. The system identification string will be identical for all unpackaged (Win32) apps and all packaged apps with no publisher ID."
> — Microsoft, *SystemIdentification Class*, `https://learn.microsoft.com/en-us/uwp/api/windows.system.profile.systemidentification`, read 2026-08-16. Requirements on that page: "Device family — Windows 10 Anniversary Edition (introduced in 10.0.14393.0)"; "API contract — Windows.Foundation.UniversalApiContract (introduced in v3.0)".

**Corrected 2026-08-21, and the route is open rather than closed.** This paragraph said the vendor shuts the door in its own words. **It reads the Note backwards.** The Note names two inputs — the publisher ID and *"an identifier derived from the system's firmware/hardware"* — and then says the **result** is the same **for a class of apps**. It says the apps are not told apart. **It does not say the machines are not told apart, and the second input is what tells machines apart.** The method page on this same route says the opposite in as many words — *"Unique for each system"* — and the class page's own abstract is *"Provides information to uniquely identify the system on which the app is running."* The sibling method's page settles the direction: *"The identifier returned by this method is specific to a user on the current device and allows for correlation of usage across different applications running on the same device"* (Microsoft, *SystemIdentification.GetSystemIdForUser(User) Method*, `https://learn.microsoft.com/en-us/uwp/api/windows.system.profile.systemidentification.getsystemidforuser`, read 2026-08-20) — **the whole axis of the publisher ID is which apps share the value.** **So what an unpackaged Win32 app actually loses is app-scoping**: every other unpackaged program on that machine computes the same string. That is a privacy and anti-correlation loss, not a loss of availability, and this use needs neither. **This route belongs with the open routes and not beside App Attest's genuine documented closure.** The standing caveat is unchanged — documentation only, never observed. Unresolved 11 is undisturbed: what `Source` an unpackaged binary receives is still not stated.

**And a sibling method this document never mentioned.** `GetSystemIdForUser(User)` is on the class page's Methods table. **It is not a route for this project** — Microsoft's own page on it says *"If you do not need to obtain an ID that is shared across apps built by different publishers, you should not use this method"*, and it requires a restricted capability needing Microsoft's approval — but its absence here was a gap. Noted 2026-08-21. Note also that `GetSystemIdForPublisher` carries **no App capabilities row** in its requirements table where `GetSystemIdForUser` carries *"userSystemId"*, so this document's *no documented privilege requirement* is right.

The `Source` property values are enumerated:

> "None 0 — There is no module present on the device for generating a unique system ID.
> Tpm 1 — The ID was generated by a Trusted Platform Module (TPM).
> Uefi 2 — The ID was generated by a Unified Extensible Firmware Interface (UEFI).
> Registry 3 — The ID was generated from the system registry."
> — Microsoft, *SystemIdentificationSource Enum*, `https://learn.microsoft.com/en-us/uwp/api/windows.system.profile.systemidentificationsource`, read 2026-08-16. Requirements: "Windows 10 Anniversary Edition (introduced in 10.0.14393.0)"; `Registry` was added in Windows version 1709 / SDK 16299.

**`EasClientDeviceInformation.Id`** — documented and package-scoped. A "reinstall-fragile" label was **cut** by the check: package-scoping is documented (the hash includes Package Family Name), but **nothing on that page says anything about reinstall, in either direction**.

> "UWP apps only. The Id property represents the DeviceId using the GUID truncated from the first 16 bytes of the SHA256 hash of the MachineID, User SID, and Package Family Name where the MachineID uses the SID of the local users group."
> — Microsoft, `https://learn.microsoft.com/en-us/uwp/api/windows.security.exchangeactivesyncprovisioning.easclientdeviceinformation.id`, read 2026-08-16.

**`HardwareIdentification.GetPackageSpecificToken(IBuffer)` — the ASHWID.** The one Windows identifier documented end-to-end as a *signed* value for a *remote* service:

> "Gets a hardware identifier (ASHWID) that represents the current hardware. The returned ASHWID will be different for each application package. In other words, this API will return different identifiers when called by two apps from different packages. It will return the same identifier when called by two apps that are part of the same package."

> "The cryptographic nonce is optional. The nonce is recommended when ASHWID needs to be verified on the cloud against replay attacks."

> "The GetPackageSpecificToken is designed for use by professional developers who have experience with licensing content and with sufficient expertise in cryptography"
> — Microsoft, `https://learn.microsoft.com/en-us/uwp/api/windows.system.profile.hardwareidentification.getpackagespecifictoken`, read 2026-08-16.

**`MachineGuid`** (`HKLM\SOFTWARE\Microsoft\Cryptography\MachineGuid`) — **no Microsoft Learn page documenting it was found**, by either agent. No forum, blog or search-snippet substitute was used, which would have been inadmissible. See Unresolved.

#### What Microsoft documents about stability

For **ASHWID**, Microsoft's own guidance page is unusually explicit (Microsoft, *Guidance on using the App Specific Hardware ID (ASHWID) to implement per-device app logic*, `https://learn.microsoft.com/en-us/previous-versions/windows/apps/jj553431(v=win.10)`, read 2026-08-16):

> "For a given app/package, the ASHWID is not affected by:
> OS re-installs
> A push button reset
> OS SKU upgrade
> Version updates for an app
> Changing users on the same device"

> "An ASHWID is generated using the components that make up the device. Each component, when present, contributes to a section of the returned ASHIWD byte stream. Nine components are listed below:
> CPU ID of the processor
> Size of the memory
> Serial number of the disk device
> Network adapter (such as NIC MAC address)
> Audio adapter
> Docking station
> Bluetooth address
> Mobile broadband device ID
> BIOS"

> "The components that make up a device can change. This change is referred to as 'hardware drift'. The ASHWID from a given device may change for a variety of reasons depending on when it is queried: Users may decide to upgrade or augment their existing devices resulting in a change in the components that affect the ASHWID. Users may temporarily connect peripherals to their devices that add to the list of components. Power management devices (for slates and devices running on ARM processors) might switch off certain hardware components to conserve battery life."

> "Hardware drift is the reason why the app should not consume the ASHWID byte stream as is. If a few hardware components change or are switched off, the API returns a different ASHWID. The app risks the possibility of incorrectly identifying the same device as a new device."

Microsoft's documented remedy is a weighted score computed **on the server**, with the threshold left to the developer: the worked illustration is explicitly "within the cloud service", "The computed score must pass the threshold set by the cloud component to be identified as the same device", and "No explicit recommendations are made for the value of the weights."

For **`GetSystemIdForPublisher`**, the stability contract is the bullet list quoted above: persists across "restarts, reinstalls and upgrades of Windows, including clean installs" and "most hardware modifications". When `Source` is `Registry`, Microsoft states plainly that the ID "will not satisfy all the above guarantees" and gives the example that "a clean install of Windows will result in a new, different ID being returned."

**Inferred, not documented:** that the contract holds "only when `Source` is `Tpm` or `Uefi`". Microsoft documents the registry case breaking the guarantees; it does **not** enumerate which bullets survive under `Tpm` versus `Uefi`, and does not affirm that the full contract holds for both. The clean Tpm/Uefi-versus-Registry dichotomy is this project's reading of the page, not Microsoft's statement.

For **`Win32_ComputerSystemProduct.UUID`** and **`Win32_BIOS.SerialNumber`**, Microsoft documents **no stability contract at all** across reinstall, network move, hardware replacement or backup restore — verified on both pages. The only statement bearing on availability is "If a UUID is not available, a UUID of all zeros is used."

**Moving the machine to a different network:** no Microsoft statement was found tying any of the above to network configuration, except that the ASHWID's component list includes "Network adapter (such as NIC MAC address)" — so an adapter change, not a network change, shifts one ASHWID component.

**Restoring a backup onto different hardware:** not documented for any of these. See Unresolved.

#### Forgeability, and the routes an administrator cannot forge

**Microsoft states outright that the software path is spoofable by a local administrator.** From *TPM Key Attestation* (`https://learn.microsoft.com/en-us/windows-server/identity/ad-ds/manage/component-updates/tpm-key-attestation`, read 2026-08-16 — note this page's own banner: "This content is written by a Microsoft customer support engineer"):

> "There was no guarantee that a key is actually protected by a TPM (someone can easily spoof a software KSP as a TPM KSP with local administrator credentials)."

**This is the strongest administrator-forgeability statement anywhere in the batch, and it is a genuine vendor statement, not an inference.**

The documented unforgeable root is the TPM's Endorsement Key:

> "EK — Endorsement Key. This is an asymmetric key contained inside the TPM (injected at manufacturing time). The EK is unique for every TPM and can identify it. The EK can't be changed or removed."

> "EKCert — EK Certificate. A TPM manufacturer-issued certificate for EKPub. Not all TPMs have EKCert."
> — same page, read 2026-08-16.

> "A trusted application can use TPM only if the TPM contains an endorsement key, which is an RSA key pair. The private half of the key pair is held inside the TPM and it's never revealed or accessible outside the TPM."

> "TPM key attestation allows a certification authority to verify that a private key is protected by a TPM and that the TPM is one that the certification authority trusts. Endorsement keys proven valid are used to bind the user identity to a device."
> — Microsoft, *TPM fundamentals*, `https://learn.microsoft.com/en-us/windows/security/hardware-security/tpm/tpm-fundamentals`, read 2026-08-16 ("Last updated on 2025-08-15").

**Route 1 — AD CS TPM key attestation.** Requires an **enterprise** Certification Authority, in Microsoft's own words: "TPM key attestation is not supported for a standalone CA." A "and a domain" half was **cut as inferred**: that page never states a domain requirement; the closest it comes is describing one trust model as validating nothing "other than the user's domain credentials". Enterprise CA: **documented**. Domain: **inferred**. From the same page:

> "Note
> This feature requires Windows 8.1/Windows Server 2012 R2.
> TPM key attestation for third-party smart card KSPs is not supported. Microsoft Platform Crypto Provider KSP must be used.
> TPM key attestation only works for RSA keys.
> TPM key attestation is not supported for a standalone CA.
> TPM key attestation does not support non-persistent certificate processing."

Three trust models are documented, and the strongest requires per-device enrolment by an administrator:

> "Trust based on EKPub: The enterprise CA validates that the EKPub provided as part of the certificate request appears in an administrator-managed list of allowed EKPubs. This list is expressed as a directory of files where the name of each file in this directory is the SHA-2 hash of the allowed EKPub. This option offers the highest assurance level but requires more administrative effort, because each device is individually identified."

> "Trust based on EKCert: The enterprise CA validates the EKCert chain that is provided as part of the certificate request against an administrator-managed list of acceptable EK cert chains. The acceptable chains are defined per-manufacturer… Note that in this mode, TPMs in use in the environment must contain EKCerts."

The same page adds "The administrator must obtain the EKPub for each device that will need TPM-attested certificates". And note the weakest model, which offers no forgery resistance at all: "Trust based on user credential: The enterprise CA trusts the user-provided EKPub as part of the certificate request and no validation is performed other than the user's domain credentials."

**Route 2 — Azure Attestation (TPM attestation).** A Microsoft-run remote attestation service. **Cost and setup requirements were cut as unsourced**: an earlier claim that it required "an Azure subscription and an attestation provider with a policy" does not survive, because the cited page contains **zero** occurrences of "subscription", "attestation provider", "paid" and "pricing" (checked). Those requirements may well be real; this page does not carry them, and no other source was consulted for them. From `https://learn.microsoft.com/en-us/azure/attestation/tpm-attestation-concepts`, read 2026-08-16:

> "Every TPM ships with a unique asymmetric key called the endorsement key (EK). This key is burned by the manufacturer."

> "A device proves to the CA that the key for which the certificate is being requested is cryptographically bound to the EKPub and that the TPM owns the EKPriv."

> "At boot, an attestation identity key is generated. It's used to provide cryptographic proof to the attestation service that the TPM in use was issued a certificate after EK validation was performed."

> "Relying parties can perform an attestation against Azure Attestation, which can be used to validate measurements made during the boot process."

That page's emphasis is on **boot integrity**, not on device identity per se; the identity element is the EK/AIK relationship established by a CA the relying party trusts. It also documents a Linux path: "Linux systems follow a similar boot process to Windows, and with TPM attestation the protection profile can be extended to beyond boot into the kernel as well using Integrity Measurement Architecture(IMA)."

**Route 3 — ASHWID signature validation.** Of the routes read here, the one that is (a) available to an application rather than an enterprise, and (b) verifiable server-side without a CA of your own. A superlative — "the only documented route" — was **softened by the check** to "of the routes read here": that was this project's comparison across pages, not a Microsoft statement, and nothing in the sources rules out a fourth route. From the ASHWID guidance page:

> "The returned ASHWID is signed. This provides a means for the app to verify that the returned ASHWID came from Windows. Passing in a shifting nonce as input protects against replay attacks."

> "By providing a different nonce every time, the cloud has the ability to verify that the returned ASHWID actually came from Windows and was not replayed by the user."

Its documented server-side workflow:

> "Verify the trustworthiness and the validity of the certificate. This step assumes that the 'Microsoft Assurance Designation Root 2011' root certificate is trusted on the cloud system. The root certificate needs to be manually added to the trusted root store on Windows-based servers."

> "Check to make sure the leaf certificate has the '1.3.6.1.4.1.311.10.5.40' EKU."

> "Signature blob is the signed blob of the SHA1 hash of the concatenation of the nonce (when present) and the ASHWID."

**The requirement that decides whether this route is open:** it is `HardwareIdentification.GetPackageSpecificToken`, and Microsoft documents the ID as *package*-specific — "The returned ASHWID will be different for each application package… It will return the same identifier when called by two apps that are part of the same package." The guidance page frames the whole feature around Store apps: it "provides a means for **Windows Store apps** to generate an ASHWID". **No Microsoft statement was found** on whether an unpackaged Win32 binary distributed from a website can call it, or what it would return. See Unresolved.

#### What it takes to read each thing in practice — Windows

- **`Win32_ComputerSystemProduct` / `Win32_BIOS`:** WMI queries against `root\CIMV2`, provider `CIMWin32`. Microsoft documents no privilege requirement on either page. See Unresolved.
- **`SystemIdentification.GetSystemIdForPublisher()`:** a WinRT static method; returns `SystemIdentificationInfo`; requires build 10.0.14393.0 and `UniversalApiContract` v3.0. No documented privilege requirement, and it is documented to work offline ("Can be created offline").
- **`HardwareIdentification.GetPackageSpecificToken(IBuffer nonce)`:** WinRT static method returning `HardwareToken` (which carries `.Id`, `.Signature`, `.Certificate`). No documented privilege requirement. Note the class caveat: "This class is not agile, which means that you need to consider its threading model and marshaling behavior." — **attribution corrected by the check:** that sentence is **not** on the method page, it is on the `HardwareIdentification` **class** page, `https://learn.microsoft.com/en-us/uwp/api/windows.system.profile.hardwareidentification` (read 2026-08-16).
- **TPM access from an application:** TPM Base Services. "The Trusted Platform Module (TPM) Base Services (TBS) feature centralizes TPM access across applications. The TBS feature runs as a system service in Windows Server 2008, Windows Vista, or newer operating systems. It provides services as an API exposed through remote procedure calls (RPC)." — `https://learn.microsoft.com/en-us/windows/win32/tbs/tpm-base-services-portal`, read 2026-08-16 ("Last updated on 2020-08-19"). Microsoft's own steer on that page is away from raw TBS: "The TPM can be used for key storage operations. However, developers are encouraged to use the Key Storage APIs for these scenarios instead."
- **Reading the EK:** `Get-TpmEndorsementKeyInfo` — "The Get-TpmEndorsementKeyInfo cmdlet gets information about the endorsement public key and certificates of the Trusted Platform Module (TPM)." (`https://learn.microsoft.com/en-us/powershell/module/trustedplatformmodule/get-tpmendorsementkeyinfo`, read 2026-08-16). Documented output fields: `IsPresent`, `PublicKey`, `PublicKeyHash`, `ManufacturerCertificates`. The page states no privilege requirement; see Unresolved.
- **Stability between two reads:** not documented for any Windows identifier here, except by implication for ASHWID — "Unless the underlying hardware has changed, two calls from the same app will result in identical ASHWIDs."

### Documented — Linux

*(Documentation only. There is no Linux machine of any kind available to this project. Nothing here was run.)*

#### What an ordinary application can read

**`/etc/machine-id`** — the systemd machine ID, from systemd's own `machine-id(5)` manual. Both runs read this page, one from the systemd repository's DocBook source and one from the live rendered freedesktop.org page at systemd 261.2; every quotation matched verbatim across both.

> "The /etc/machine-id file contains the unique machine ID of the local system that is set during installation or boot. The machine ID is a single newline-terminated, hexadecimal, 32-character, lowercase ID. When decoded from hexadecimal, this corresponds to a 16-byte/128-bit value. This ID may not be all zeros."

> "The machine ID is usually generated from a random source during system installation or first boot and stays constant for all subsequent boots. Optionally, for stateless systems, it is generated during runtime during early boot if necessary."

**The clause that governs whether an application may send it anywhere:**

> "This ID uniquely identifies the host. It should be considered "confidential", and must not be exposed in untrusted environments, in particular on the network. If a stable unique identifier that is tied to the machine is needed for some application, the machine ID or any part of it must not be used directly. Instead the machine ID should be hashed with a cryptographic, keyed hash function, using a fixed, application-specific key. That way the ID will be properly unique, and derived in a constant way from the machine ID but there will be no way to retrieve the original machine ID from the application-specific one. The sd_id128_get_machine_app_specific(3) API provides an implementation of such an algorithm."

**`sd_id128_get_machine()` / `sd_id128_get_machine_app_specific()`** — from systemd's `sd_id128_get_machine(3)`:

> "sd_id128_get_machine() returns the machine ID of the executing host. This reads and parses the machine-id5 file. This function caches the machine ID internally to make retrieving the machine ID a cheap operation. This ID may be used wherever a unique identifier for the local system is needed. However, it is recommended to use this ID as-is only in trusted environments. In untrusted environments it is recommended to derive an application specific ID from this machine ID, in an irreversible (cryptographically secure) way."

> "sd_id128_get_app_specific() returns a machine ID that is a combination of the base and app_id parameters. Internally, this function calculates HMAC-SHA256 of the app_id parameter keyed by the base parameter, and truncates this result to fit in sd_id128_t and turns it into a valid Variant 1 Version 4 UUID, in accordance with RFC 4122. Neither of the two input parameters can be calculated from the output parameter ret."

> "This function will return the same application-specific ID for each combination of machine ID and application ID."

Signature: `int sd_id128_get_machine_app_specific(sd_id128_t app_id, sd_id128_t *ret);` — returns 0 on success or a negative errno. Documented errors include `-ENOENT` ("when /etc/machine-id is missing"), `-ENOMEDIUM` ("when /etc/machine-id is empty or all zeros"), `-ENOPKG` ("when the content of /etc/machine-id is uninitialized"), `-EUCLEAN` ("when the configured value has invalid format"), and `-EPERM` ("Requested information could not be retrieved because of insufficient permissions").

**D-Bus machine ID** — from the D-Bus Specification (`https://dbus.freedesktop.org/doc/dbus-specification.html`, read 2026-08-16; this host served both agents normally, unlike `www.freedesktop.org`):

> "The UUID must contain 128 bits of data and be hex-encoded. The hex-encoded string may not contain hyphens or other non-hex-digit characters, and it must be exactly 32 characters long."

> "On receipt of the METHOD_CALL message org.freedesktop.DBus.Peer.GetMachineId, an application should reply with a METHOD_RETURN containing a hex-encoded UUID representing the identity of the machine the process is running on. This UUID must be the same for all processes on a single system at least until that system next reboots. It should be the same across reboots if possible, but this is not always possible to implement and is not guaranteed."

> "On Unix, implementations should try to read the machine ID from /var/lib/dbus/machine-id and /etc/machine-id."

> "The UUID is intended to be per-instance-of-the-operating-system, so may represent a virtual machine running on a hypervisor, rather than a physical machine."

**`/proc/sys/kernel/random/boot_id`** — via `sd_id128_get_boot()`: "This reads and parses the /proc/sys/kernel/random/boot_id file exposed by the kernel. It is randomly generated early at boot and is unique for every running kernel instance." Useless for identifying a machine across reboots by construction, but recorded because it is the other ID systemd exposes.

**`/sys/class/dmi/id/product_uuid`** — the SMBIOS system UUID. **The kernel publishes no prose ABI documentation for this path that either agent could reach** (see Limits). What the kernel's own *source* shows (`https://raw.githubusercontent.com/torvalds/linux/master/drivers/firmware/dmi-id.c`, read 2026-08-16 — **this is source code, not prose documentation, and is flagged as such**; verbatim, lines 49, 50, 56 and 61 of the current file):

```
DEFINE_DMI_ATTR_WITH_SHOW(product_serial,	0400, DMI_PRODUCT_SERIAL);
DEFINE_DMI_ATTR_WITH_SHOW(product_uuid,		0400, DMI_PRODUCT_UUID);
DEFINE_DMI_ATTR_WITH_SHOW(board_serial,		0400, DMI_BOARD_SERIAL);
DEFINE_DMI_ATTR_WITH_SHOW(chassis_serial,	0400, DMI_CHASSIS_SERIAL);
```

Mode `0400` is owner-read-only, and on the face of the source an unprivileged process cannot read `product_uuid`, `product_serial`, `board_serial` or `chassis_serial`. **Inferred, and already flagged as such:** the mode comes from source, but "sysfs attributes are owned by root" is cited to nothing. **Treat this as source-derived, not documented.** It could not be confirmed against kernel prose documentation and could not be tested. See Unresolved.

#### What the vendors document about stability

**Reinstalling the operating system — inferred.** systemd's `machine-id(5)` says "The machine ID is usually generated from a random source during system installation or first boot" and "For normal operating system installations, where a custom image is created for a specific machine, /etc/machine-id should be populated during installation." That a reinstall is therefore a fresh generation, and that the machine ID does **not** survive it, is read out of the quoted generation rule. **systemd does not say so in those words**, so this carries the **inferred** label, not the documented one.

**Replacing hardware and moving networks — documented:**

> "The machine ID does not change based on local or network configuration or when hardware is replaced. Due to this and its greater length, it is a more useful replacement for the gethostid(3) call that POSIX specifies."

A comparative superlative — that this is "the strongest stability statement any of the three systems makes" — was **cut**. It is a judgement, not a source claim, and a contestable one: Microsoft's `GetSystemIdForPublisher` contract claims persistence across "restarts, reinstalls and upgrades of Windows, including clean installs" and "most hardware modifications", which is at least as strong on its face.

**Cloning an image — documented in detail:**

> "For operating system images which are created once and used on multiple machines, for example for containers or in the cloud, /etc/machine-id should be either missing or an empty file in the generic file system image… An ID will be generated during boot and saved to this file if possible."

And the boot-time fallback order, which shows what the ID can be derived from when the file is absent:

> "If this file is empty or missing, systemd will attempt to use the D-Bus machine ID from /var/lib/dbus/machine-id, the value of the kernel command line option container_uuid, the KVM DMI product_uuid or the devicetree vm,uuid (on KVM systems), the Xen hypervisor uuid, and finally a randomly generated UUID. systemd.machine_id=firmware can be set to generate the machine ID from the firmware."

**Restoring a backup onto different hardware — inferred, not documented.** systemd's rule is "If /etc/machine-id already contains a valid machine-id, this is not a first boot." Read across to a backup, a byte-for-byte restore onto different hardware would carry `/etc/machine-id` with it unchanged, and the same machine ID would appear on two machines. The quoted rule is verbatim, but it is a **first-boot-detection** rule. systemd documents the *image-cloning* case explicitly and **never addresses backup restore**, nor states the two-machines consequence. This is this project's step, not systemd's.

**Stability of the SMBIOS `product_uuid`** across any of these events: not documented anywhere either agent could reach.

#### Forgeability, and the routes an administrator cannot forge

**systemd documents, in the machine ID's own manual, that it is a plain file whose value can be dictated at boot:**

> "The machine ID may be set, for example when network booting, with the systemd.machine_id= kernel command line parameter or by passing the option --machine-id= to systemd. An ID specified in this manner has higher priority and will be used instead of the ID stored in /etc/machine-id."

That is a documented, first-class, supported mechanism for a person with control of the machine to make the machine ID be any 128-bit value they choose. **No forgery is required; the override is a feature.** This is the cleanest documented-forgeability claim in the batch. systemd nowhere claims the machine ID resists an adversary; it claims the opposite by asking that it be treated as "confidential" and hashed before leaving the host.

**No Linux equivalent of DeviceCheck or Managed Device Attestation was found in the documentation read.** A flat universal negative — that no such thing "exists", and that "there is no vendor-run service that a Linux application can ask 'is this really a distinct machine?', because there is no vendor" — was **narrowed by the check**: it had no source behind it and claimed more than the findings' own unresolved item conceded. The defensible form is the one stated here: none was found in the documentation read.

What that documentation does describe is a TPM-based route the application developer must build and operate entirely themselves:

- The unforgeable root is the same EK described in the Windows section, which is a property of the TPM chip rather than of any operating system. **Inferred:** Microsoft's `tpm-fundamentals` page is Windows-scoped; carrying its EK description across to a TPM running under Linux is this project's step. Reasonable, given the EK is a hardware property, but it is a step, and no Linux-side source was read for it.
- Retrieving the EK certificate on Linux is documented by the tpm2-software project's own manual for `tpm2_getekcertificate` (`https://raw.githubusercontent.com/tpm2-software/tpm2-tools/master/man/tpm2_getekcertificate.1.md`, read 2026-08-16):

  > "**tpm2_getekcertificate**(1) - Retrieve the endorsement key certificate. The certificate is present either on the TCG specified TPM NV indices OR on the TPM manufacturer's endorsement certificate hosting server."

  Its documented lookup order is: "NV-Index: Default search location when **ARGUMENT** is not specified", then manufacturer-specific servers (the page names Intel's EK-certificate server and re-certification server for manufacturer `INTC`), then "Generic or other EK-certificate-server: Search location when **ARGUMENT** specifies the EK certificate web hosting address."

- **Inferred, not documented:** that on Linux the burden therefore **appears to be** a TPM 2.0 that has an EKCert at all (Microsoft: "Not all TPMs have EKCert"), the manufacturer's root and intermediate certificates obtained out of band, and a server-side verifier written and run by the developer. This is a synthesis across two sources; neither states the "server-side verifier written and run by the developer" requirement in its own words.
- Azure Attestation documents a Linux path, but its subject is boot and kernel integrity via IMA, not device identity. A "paid" descriptor was **cut**: the cited page carries no cost, pricing or subscription statement. "Linux systems follow a similar boot process to Windows, and with TPM attestation the protection profile can be extended to beyond boot into the kernel as well using Integrity Measurement Architecture(IMA)." (`tpm-attestation-concepts`, address above, read 2026-08-16.)

#### What it takes to read each thing in practice — Linux

- **`/etc/machine-id`:** an ordinary file read, or `sd_id128_get_machine()` / `sd_id128_get_machine_app_specific()` from `#include <systemd/sd-id128.h>`, linking `libsystemd`. Documented as cached: "This function caches the machine ID internally to make retrieving the machine ID a cheap operation." **Inferred, not documented:** that two reads within a process therefore return the same value by construction — systemd documents caching as a performance property, not as a stability guarantee. `-EPERM` is a documented failure mode.
- **D-Bus machine ID:** the `org.freedesktop.DBus.Peer.GetMachineId` method call, or reading `/var/lib/dbus/machine-id`. Command-line generator and reader `dbus-uuidgen` (`https://dbus.freedesktop.org/doc/dbus-uuidgen.1.html`, fetched HTTP 200 by both agents on 2026-08-16).
- **`/sys/class/dmi/id/product_uuid`:** a file read. Per the kernel source quoted above, mode `0400` — root only, source-derived rather than documented. `dmidecode(8)` would be the other route; **its manual page was not read by the first run (HTTP 502) and no claim rests on it.** The check reached it (301 → HTTP 200, `.TH DMIDECODE 8 "June 2025"`), so the source is not in fact unavailable, but nothing downstream changes.
- **TPM device access: an ordinary user is not in the required group by default.** From the tpm2-tss project's own shipped udev rules (`https://raw.githubusercontent.com/tpm2-software/tpm2-tss/master/dist/tpm-udev.rules`, read 2026-08-16 — **a shipped configuration file, not prose documentation, carrying the same source-not-prose flag as the kernel's `dmi-id.c`**):

```
# tpm devices can only be accessed by the tss user but the tss
# group members can access tpmrm devices
KERNEL=="tpm[0-9]*", TAG+="systemd", MODE="0660", OWNER="tss"
KERNEL=="tpmrm[0-9]*", TAG+="systemd", MODE="0660", GROUP="tss"
KERNEL=="tcm[0-9]*", TAG+="systemd", MODE="0660", OWNER="tss"
KERNEL=="tcmrm[0-9]*", TAG+="systemd", MODE="0660", GROUP="tss"
```

  `/dev/tpm0` is owned by user `tss` with mode `0660`; `/dev/tpmrm0` is group `tss` with mode `0660`. An application run by an ordinary desktop user reaches neither by default — reaching the resource-manager device `/dev/tpmrm0` requires being added to group `tss`, an administrative act. **Narrowed by the check:** group membership does **not** open both. The rules' own comment says "tpm devices can only be accessed by the **tss user** but the tss **group** members can access **tpmrm** devices", and the `tpm[0-9]*` rule sets `OWNER="tss"` with no `GROUP=`, so joining group `tss` opens `/dev/tpmrm0` but **not** `/dev/tpm0`.

### Observed — Fact 11

All on the machine named in the Limits, on 2026-08-16, as uid 501, with no `sudo` and no state change. **Nothing was observed for Windows or Linux, because no such machine exists here.** Every observation below was re-run independently by the check; where a figure differed, both are shown.

**O-1. `IOPlatformExpertDevice` properties are readable by a non-root user with no prompt.**
Command: `ioreg -rd1 -c IOPlatformExpertDevice`, run as uid 501.
Result: exit status 0; both `IOPlatformSerialNumber` and `IOPlatformUUID` present. Values `[redacted]`.
*What a negative result would have looked like:* `Permission denied`, an empty match set, a non-zero exit status, or a blocking authorisation dialog. None occurred; the command completed non-interactively. Reproduced: `exit=0`, one match each, non-interactive.

**O-2. Format of the two identifiers (values never recorded).**
- `IOPlatformUUID` — length 36; character set `{0-9, A-F, '-'}`; hyphen-separated group lengths `[8, 4, 4, 4, 12]`; every non-hyphen character an uppercase hex digit, verified programmatically (all characters ∈ `0123456789ABCDEF-`). Canonical uppercase UUID shape.
- `IOPlatformSerialNumber` — length 10; character set `{0-9, A-Z}`; no separators. Apple's TN1103 explicitly warns against relying on this: "Developers should not make any assumptions about the format of the serial number such as its length or what characters it may contain."

*What a negative result would have looked like:* a differing length, lowercase characters, non-hex characters in the UUID, or a different group split. Reproduced exactly.

**O-3. Two consecutive reads return the same value.**
Method: read `IOPlatformExpertDevice` twice in one process, compare strings.
Result: `IOPlatformUUID read1==read2: True`; `IOPlatformSerialNumber read1==read2: True`.
*What a negative result would have looked like:* `False` for either — a per-read or per-process value rather than a stable one. Reproduced.

**O-4. `system_profiler SPHardwareDataType` exposes a third identifier: a Provisioning UDID.**
Command: `system_profiler SPHardwareDataType`, uid 501, no prompt, exit 0.
Hardware fields (identifying values redacted): `Model Name: MacBook Pro`; `Model Identifier: [redacted]`; `Chip: [redacted]`; `System Firmware Version: [redacted]`; `OS Loader Version: [redacted]`; `Activation Lock Status: Enabled`.
Identifier formats (values `[redacted]`):
- `Serial Number (system)` — length 10, no separators (matches O-2's `IOPlatformSerialNumber` format).
- `Hardware UUID` — length 36, groups `[8, 4, 4, 4, 12]`, uppercase hex only.
- `Provisioning UDID` — length **25**, a **single hyphen** splitting it into groups `[8, 16]`, uppercase hex only.

*What a negative result would have looked like:* the `Provisioning UDID` row absent, or a UDID of a different length or shape. All reproduced.

A parenthetical claiming the Provisioning UDID "is absent on Intel Macs, or a 40-character UDID of the older shape" was **cut**: unsourced, and a statement about other machines dressed as a note on this one, with no Intel Mac available to this project.

**O-5. App Attest reports itself unsupported on this Mac — Apple's documentation confirmed against the machine.**
Method: compiled a small Swift program against the `DeviceCheck` framework (`swiftc -O dcheck.swift -framework DeviceCheck`) in the scratchpad and ran it. Nothing was installed; the binary lives outside the project.
Output, verbatim:

```
DCAppAttestService.shared.isSupported = false
DCDevice.current.isSupported = true
generateToken OK: token byte-count=2212 (value withheld)
done
```

*What a negative result would have looked like:* `DCAppAttestService.shared.isSupported = true`, which would have contradicted Apple's documented "If you read isSupported from an app running on a Mac device, the value is false."

The check wrote and compiled its own probe and ran it as uid 501. **The two `isSupported` lines reproduced exactly.** The byte-count did not — see the disagreements table in the Limits: 2212 on the first run, **2213** on each of two calls in the re-run, with the first run's own O-7 also recording 2213 twice. Both recorded, neither adjudicated. **The token's length is not a fixed property and should not be cited as one.**

**O-6. `DCDevice` works on macOS, from a binary signed by no developer and with no application bundle.**
The probe that produced O-5 has no application bundle, no `Info.plist`, no bundle identifier, no entitlements and no Developer ID. Its signature, per `codesign -dvvv`:

```
Identifier=dcheck
Format=Mach-O thin (arm64)
CodeDirectory v=20400 size=543 flags=0x20002(adhoc,linker-signed) hashes=14+0 location=embedded
Signature=adhoc
```

`DCDevice.current.generateToken` nevertheless returned a token with no error.
*What a negative result would have looked like:* a non-nil `NSError` from `generateToken` — which is exactly what Apple's note leads one to expect ("The app you use to generate the token must be associated with your developer account; otherwise, the generation request fails"). It did not fail.

The check reproduced this from its own probe, adding `Info.plist=not bound`, `TeamIdentifier=not set`, `Sealed Resources=none`, `Internal requirements=none`, and `codesign -d --entitlements -` returning no entitlements; `generateToken` returned a token with a nil error, twice. **The observation stands.**

**Scope of this observation:** it shows only that the *client-side* call succeeds. It does **not** show that Apple's server would accept the token, because accepting it requires a DeviceCheck-enabled ES256 key from a developer account, which this project does not have. Whether `https://api.devicecheck.apple.com/v1/validate_device_token` would honour a token from an ad-hoc-signed binary is **untested and unresolved**. And what Apple documents the token is *for* is narrower than device identity: it is "ephemeral", identifies "the current device", should be treated "as single-use", and exists so "Your server uses the generated token in its requests to get or set the persistent bits for the current device" — addressing two per-device bits on Apple's server, not proving hardware identity to a relying party.

**O-7. The `DCDevice` token is different on every call.**
Method: called `generateToken` twice in one process and compared the returned `Data`.
Output, verbatim: `token1.count=2213 token2.count=2213 identical=false`, and their SHA-256 digests differ.
*What a negative result would have looked like:* `identical=true`, which would have made the token a directly comparable device fingerprint rather than the single-use blob Apple documents. Reproduced.

**O-8. `gethostuuid` has a manual page on this machine but no Swift or Objective-C documentation page.**
`man 2 gethostuuid` returns the page quoted above. `man 3 gethostuuid` returns `No manual entry for gethostuuid`. The declaration reaches applications through `#include <unistd.h>`, which at line **670** of the macOS 26 SDK's `unistd.h` reads `#include <gethostuuid.h>`.
*What a negative result would have looked like:* no man page in any section, or no such include in the SDK header. Reproduced exactly, line number included.

**Not observed, deliberately:** `gethostuuid()` was not called, the primary MAC address was not read, no Secure Enclave key was created, and no identifier was changed.

### Unresolved — Fact 11

**Cross-cutting**

1. **Forgeability was never tested on any system.** See the Limits. *What would settle it:* three disposable machines, one per operating system, permission to modify them, and a controlled attempt to make machine B report machine A's identifier.
2. **The Linux kernel's prose ABI documentation for `/sys/class/dmi/id/*` could not be located.** The `0400` mode is taken from `drivers/firmware/dmi-id.c` — the kernel's own source, but not its documentation. **CLOSED 2026-08-21 from the kernel's own source, and it is wider than this entry had it.** `https://git.kernel.org/pub/scm/linux/kernel/git/torvalds/linux.git/plain/drivers/firmware/dmi-id.c`, read 2026-08-20: `DEFINE_DMI_ATTR_WITH_SHOW(product_serial, 0400, DMI_PRODUCT_SERIAL);`, `product_uuid` 0400, `board_serial` 0400, `chassis_serial` 0400 — and `bios_vendor` and **every** other attribute in that block (`bios_version`, `bios_date`, `sys_vendor`, `bios_release`, `ec_firmware_release`, `product_name`, `product_version`, `product_sku`, `product_family`, `board_vendor`, `board_name`, `board_version`, `board_asset_tag`, `chassis_vendor`, `chassis_type`, `chassis_version`, `chassis_asset_tag`) at **0444**. **Exactly four are root-only and they are the four identity-bearing ones; the rest are open to anyone.** The mode bits answer the non-root question without a machine. *(The prose ABI documentation is still unreached; what closed is the question, not the wish for a second source.)*

**macOS**

3. **Whether `IOPlatformUUID` and `IOPlatformSerialNumber` survive an OS reinstall is undocumented by Apple.** Both constants have **empty abstracts** on developer.apple.com. Apple documents only one way these can be lost — "Repairing a system by swapping hardware components" — and says "Apple does not document the specific details of how a machine can lose its serial number." *What would settle it:* an Apple statement, or a reinstall on a disposable Mac with a before-and-after read.
4. **Whether the Provisioning UDID is documented at all.** It appears in `system_profiler` output (O-4); no Apple page defining it, its derivation or its stability was found. *What would settle it:* an Apple developer or deployment documentation page naming "Provisioning UDID".
5. **Whether a `DCDevice` token resists forgery by the machine's administrator is undocumented.** Apple calls it "a unique, authenticated token" and offers a `validate_device_token` endpoint, but never states what the validation proves, never says the token is Secure-Enclave-backed, and never claims tamper-resistance. *What would settle it:* an Apple statement about the token's binding, or an Apple Developer Program membership plus a controlled test.
6. **Whether Apple's DeviceCheck server accepts tokens from a non-App-Store, Developer-ID-signed or ad-hoc-signed Mac binary.** O-6 shows the client call succeeds; the server round trip was never made. *What would settle it:* an Apple Developer Program membership (cost not established from the documentation read here), a DeviceCheck-enabled ES256 key, and a call to `/v1/validate_device_token`.
7. **Whether the two DeviceCheck bits are per-device-per-developer-account, and how long they persist.** Apple documents `last_update_time` "in YYYY-MM format" and a "Bit State Not Found" 200 response, but no retention policy, and no statement on whether the bits survive an OS reinstall or a logic-board replacement. *What would settle it:* an Apple retention statement.

**Windows**

8. **`HKLM\SOFTWARE\Microsoft\Cryptography\MachineGuid` is undocumented on Microsoft Learn**, as far as either agent's searches reached. Any claim about it — its stability, its regeneration on reinstall, its readability — would be unsourced. *What would settle it:* a Microsoft Learn page naming it.
9. **Whether an unpackaged Win32 binary distributed from a website can call `HardwareIdentification.GetPackageSpecificToken`, and what it returns if so.** The API is documented as *package*-specific and the guidance page is framed around Store apps. This matters more than anything else on Windows, because ASHWID is the only Windows route documented as producing a *signed* value a remote server can authenticate. *What would settle it:* a Microsoft statement on unpackaged callers, or a test on a Windows machine.
10. **Whether `Win32_ComputerSystemProduct.UUID`, `Win32_BIOS.SerialNumber` and `Get-TpmEndorsementKeyInfo` require administrator privilege.** None of the three Microsoft pages states a privilege requirement in either direction. *What would settle it:* running each as a standard user on Windows.
11. **What `GetSystemIdForPublisher` actually returns on a machine with a TPM versus one without, and whether the `Registry`-sourced value is the one an unpackaged Win32 app receives.** The class note ("identical for all unpackaged (Win32) apps") and the `Source` property interact in a way Microsoft does not spell out. *What would settle it:* reading `SystemIdentificationInfo.Source` on a Windows machine from an unpackaged binary.
12. **Whether any Windows identifier survives a backup restored onto different hardware.** Not documented for any of them; ASHWID's component list makes it near-certain that ASHWID would change, but Microsoft does not say so.

**Linux**

13. **Whether a TPM EK certificate is present on typical consumer Linux hardware.** Microsoft states "Not all TPMs have EKCert"; the tpm2-tools manual documents manufacturer-hosted fallbacks for Intel only. Without an EKCert and without the manufacturer's chain, the only TPM trust model remaining *among those read here* is the enterprise one — an administrator-curated EKPub allowlist — which this project has no route to. **That last clause is inferred**, from the three trust models Microsoft documents on a Windows-scoped page; it was softened by the check from an assertion of fact sitting inside an open question. *What would settle it:* a survey of EKCert presence across manufacturers from TCG or manufacturer documentation, and a Linux machine to check.
14. **CLOSED 2026-08-21 from systemd's own source. It is world-readable, and systemd says why in a comment.** `https://raw.githubusercontent.com/systemd/systemd/main/src/shared/machine-id-setup.c`, read 2026-08-20: *"/* We create this 0444, to indicate that this isn't really something you should ever modify. Of course, since the file will be owned by root it doesn't matter much, but maybe people look. */"* followed by `openat(etc_fd, "machine-id", O_CREAT|O_EXCL|O_RDWR|O_NOFOLLOW|O_CLOEXEC, 0444)`. **And systemd's *confidential* is intent rather than mode** — `machine-id(5)`, read the same day: *"This ID uniquely identifies the host. It should be considered "confidential", and must not be exposed in untrusted environments, in particular on the network."* **Confidential in intent, 0444 in fact**, which is exactly the gap this entry was right to be suspicious of. *(No machine needed; `ls -l` on a borrowed box was never the only route.)*
15. **Whether any Linux-native remote attestation service exists that a consumer application could use without operating its own verifier.** None was found in the documentation read; Azure Attestation is the only named service, it is Microsoft's, and it is aimed at boot integrity. *What would settle it:* a survey of Linux Foundation or distribution vendor documentation, which was not undertaken.

**Not consulted at all**

16. **TCG's own specifications** (TPM 2.0 Library, EK Credential Profile) were not fetched on 2026-08-16, and **the fetch was attempted on 2026-08-20 and refused with HTTP 403**, on a plain fetch and again with a browser user-agent string — see Limits §5. Everything said above about EK and EKCert semantics is second-hand from Microsoft Learn and the tpm2-software project's manuals, not from the standard's own text. *What would settle it:* the standard's own text obtained by some route `trustedcomputinggroup.org` does not refuse; **fetching the PDFs directly is not that route, because it was tried.** *(Corrected 2026-08-22; this entry previously said the specifications "were not fetched" and named "fetching the TCG PDFs directly" as the remedy, which reads as free.)*

---

## Fact 12 — what the computer and the login account are called

*What each supported operating system calls the computer and the login account, how a program reads them, how steady each is across a rename, a network move, a reinstall and a hardware change, and whether two computers can carry the same computer name.*

### Documented — macOS

#### How many names a Mac has, and what each is for

Apple documents **three** distinct names, in the `scutil(8)` manual page shipped with macOS.

> `--get pref`
>  Retrieves the specified preference.  The current value will be reported on standard output.
>
>  Supported preferences include:
>
>        ComputerName   The user-friendly name for the system.
>
>        LocalHostName  The local (Bonjour) host name.
>
>        HostName       The name associated with hostname(1) and gethostname(3).

— `man scutil` on this machine (macOS [redacted], build [redacted]); page header `SCUTIL(8) System Manager's Manual`, dated `January 23, 2008`. Read 2026-08-16 by both agents.

So: **ComputerName** is the display name, **LocalHostName** is the Bonjour/mDNS label, and **HostName** is the preference `scutil` documents as *associated with* `hostname(1)` and `gethostname(3)`. **Narrowed by the check:** an earlier reading said HostName "is the POSIX hostname that `gethostname(3)` returns". The man page says only "associated with", and observation O4 below shows that when `HostName` is unset, `gethostname(3)` returns a different string entirely — so the tighter reading is contradicted by this batch's own observation.

Apple states the derivation between the first two, and the `.local` suffix:

> The local hostname, or local network name, is displayed at the bottom of the Sharing settings window. It identifies your Mac to Bonjour-compatible services.

> The local hostname is your computer’s name with .local added, and any spaces are replaced with hyphens. For example, if your computer’s name is My Computer, your local hostname is My-Computer.local. Local hostnames aren’t case sensitive, so my-computer.local is the same as My-Computer.local.

— `https://support.apple.com/guide/mac-help/change-computers-local-hostname-mac-mchlp2322/mac` (macOS Tahoe 26 edition), read 2026-08-16.

Note the wording: Apple's sentence describes the *local hostname* as the computer's name plus `.local`, but the value stored in the `LocalHostName` preference does **not** carry the `.local` suffix — see Observed, where `LocalHostName` is the bare label and the `.local` appears only on the POSIX hostname.

Where the ComputerName is not usable as a Bonjour label at all, Apple documents a fixed fallback:

> If your computer name is not recognized by Bonjour, the local network name is Macintosh.local.

— same address, read 2026-08-16.

#### How a program reads each name

Apple's System Configuration framework exposes each name as its own function.

> `func SCDynamicStoreCopyComputerName(_ store: SCDynamicStore?, _ nameEncoding: UnsafeMutablePointer<CFStringEncoding>?) -> CFString?`
>
> Returns the current computer name.
>
> [`store`] The dynamic store session that should be used for communication with the server. Pass `NULL` to use a temporary session.
>
> [`nameEncoding`] A pointer to memory that, on output, is filled with the encoding associated with the computer or host name, if it is non-`NULL`.
>
> The current computer name, or `NULL` if the name has not been set or if an error occurred. You must release the return value.

— `https://developer.apple.com/tutorials/data/documentation/systemconfiguration/scdynamicstorecopycomputername(_:_:).json` — the documentation data for `https://developer.apple.com/documentation/systemconfiguration/scdynamicstorecopycomputername(_:_:)`. Read 2026-08-16.

> `func SCDynamicStoreCopyLocalHostName(_ store: SCDynamicStore?) -> CFString?`
>
> Returns the current local host name.
>
> Returns the current local host name, or `NULL` if the name has not been set or if an error occurred. You must release the return value.

— `https://developer.apple.com/tutorials/data/documentation/systemconfiguration/scdynamicstorecopylocalhostname(_:).json` — the documentation data for `https://developer.apple.com/documentation/systemconfiguration/scdynamicstorecopylocalhostname(_:)`. Read 2026-08-16.

**Inferred, not documented:** that the `nameEncoding` out-parameter on the ComputerName call means Apple treats the ComputerName as text that need not be ASCII. Apple's page says only that the pointer is "filled with the encoding associated with the computer or host name". It nowhere says the name need not be ASCII — that step is this project's reading, and it was originally sitting unmarked under a heading that says Documented.

The third name, `HostName`, is read by the POSIX call named in the `scutil` man page:

> The `gethostname()` function shall return the standard host name for the current machine. The `namelen` argument shall specify the size of the array pointed to by the `name` argument. The returned name shall be null-terminated, except that if `namelen` is an insufficient length to hold the host name, then the returned name shall be truncated and it is unspecified whether the returned name is null-terminated.
> Host names are limited to {HOST_NAME_MAX} bytes.

— The Open Group Base Specifications Issue 8, IEEE Std 1003.1-2024, `https://pubs.opengroup.org/onlinepubs/9799919799/functions/gethostname.html`. Read 2026-08-16.

**Privilege.** Apple documents elevation for *writing*, and says nothing about elevation for reading:

> The `--set` option requires super-user access.

— `man scutil` on this machine, read 2026-08-16. No corresponding statement exists for `--get`, and **neither of the two** `SCDynamicStoreCopy…` pages cited above states a privilege requirement — both were searched for privilege, root, sudo, entitlement and permission language, with no hits. (An earlier "none of the three" was **corrected**: there are only two, because **Apple publishes no `SCDynamicStoreCopy…` call for `HostName`**, which is why this fact reaches for `gethostname(3)` instead.) Reading was observed unprivileged and without a prompt — see Observed.

#### The login account's name

Apple distinguishes an account name from a display name, and documents that they need not match:

> Update the “User name” field — not the “User” field — to match the new name that you gave to your home folder in the previous section.

> Optional: Update the “Full name” field. The full name does not need to match the user name, and can contain spaces. You will be able to use either the user name or the full name when logging in or making changes that require your account name and password.

— `https://support.apple.com/en-us/102547` ("Change the name of your macOS user account and home folder"), read 2026-08-16. That page renders its body client-side; the quotes were taken from the raw HTML of the same address, retrieved directly.

The POSIX text that macOS inherits distinguishes three *different* account names a process can read, which is the crux of "what a program reads to get it":

> Three names associated with the current process can be determined: `getpwuid(geteuid())` shall return the name associated with the effective user ID of the process; `getlogin()` shall return the name associated with the current login activity; and `getpwuid(getuid())` shall return the name associated with the real user ID of the process.

> The `getlogin()` function shall return a pointer to a string containing the user name associated by the login activity with the controlling terminal of the current process. If `getlogin()` returns a non-null pointer, then that pointer points to the name that the user logged in under, even if there are several login names with the same user ID.

> [ENXIO] The calling process has no controlling terminal.

— The Open Group Base Specifications Issue 8, IEEE Std 1003.1-2024, `https://pubs.opengroup.org/onlinepubs/9799919799/functions/getlogin.html`. Read 2026-08-16. One note for a later reader: the three-names paragraph is in the **APPLICATION USAGE** section, which the standard marks informative; the controlling-terminal sentence is normative DESCRIPTION.

That `getlogin()` is defined in terms of the *controlling terminal* rather than the process's own user ID is the basis **in POSIX** on which these two readings *can* differ. **Narrowed by the check** from "the documented reason these two readings can disagree": POSIX documents that the two are defined against different things; it does not document that this is the reason any particular pair of readings disagreed.

**Corrected 2026-08-22: this sentence went on "— and they did differ here; see Observed", and that is not why they differed on the machine observed. Darwin does not define `getlogin()` against the controlling terminal at all**, and Apple says so in its own manual page: `man getlogin`, read on the Creator's Mac 2026-08-20, gives the name as *"the login name of the user associated with the current session, as previously set by setlogin()"* and, in BUGS, *"The current implementation (using setlogin()) allows getlogin to succeed even when the process has no controlling terminal."* The kernel matches — `getlogin` copies a string held on the **session** (`bsd/kern/kern_prot.c`), `setlogin` writes it and is superuser-only, and a new session inherits it from its parent's (`bsd/kern/kern_proc.c`), all read 2026-08-20. **The lineage is the session, not the terminal.** The POSIX quotations above stand as POSIX's text; what does not follow from them is the account of this machine's divergence, which is in the Observed entry.

#### Collisions

Apple documents that two Macs can hold the same ComputerName, and what macOS does about it:

> If your Mac has the exact name of another Mac on your local network, a number is added to the local network name.

— `https://support.apple.com/guide/mac-help/change-computers-local-hostname-mac-mchlp2322/mac`, read 2026-08-16.

Note what that sentence does and does not say: the *local network name* gets a number. The ComputerName itself is not documented as changing. So two Macs on one link can display the same ComputerName while the names they advertise on the link differ. **Narrowed by the check** from "…while carrying different LocalHostNames": Apple's sentence is about the local network name, and whether the stored `LocalHostName` preference is what gets renumbered, or whether the numbering lives only in the advertised Bonjour name, **is not documented on that page**.

The underlying mechanism is the mDNS specification Apple's Bonjour implements:

> To remedy this problem, this document allows any computer user to elect to give their computers link-local Multicast DNS host names of the form: "single-dns-label.local.".  For example, a laptop computer may answer to the name "MyComputer.local.".  Any computer user is granted the authority to name their computer this way, provided that the chosen host name is not already in use on that link.  Having named their computer this way, the user has the authority to continue utilizing that name until such time as a name conflict occurs on the link that is not resolved in the user's favor.  If this happens, the computer (or its human user) MUST cease using the name, and SHOULD attempt to allocate a new unique name for use on that link.

> A common example of a resource record type that is intended to be unique, not shared between hosts, is the address record that maps a host's name to its IP address.  Should a host witness another host announce an address record with the same name but a different IP address, then that is considered inconsistent, and that address record is considered to be in conflict.

> The recommended course of action after probing and failing is as follows:
>
>    1. Programmatically change the resource record name in an attempt to find a new name that is unique.  This could be done by adding some further identifying information (e.g., the model name of the hardware) if it is not already present in the name, or appending the digit "2" to the name, or incrementing a number at the end of the name if one is already present.

— RFC 6762, "Multicast DNS", Sections 3 and 9, `https://www.rfc-editor.org/rfc/rfc6762.txt`. Read 2026-08-16. Section attribution verified: the "To remedy this problem" passage is in §3, both conflict passages in §9.

Two points matter, and they carry different labels.

**Inferred, not documented:** `.local` names "are meaningful only on the link where they originate" (RFC 6762 §3), from which it follows that two Macs on different networks with identical names never collide and never renumber. The quoted §3 sentence is about the *scope of the names*, not about where conflict detection happens, and **the RFC nowhere states the never-collide conclusion.**

**Documented:** the renumbering is the *loser's*, so which machine keeps the plain name is decided by the probing tie-break, not by anything stable. RFC 6762 §9: "The protocol used in the Probing phase will determine a winner and a loser, and the loser MUST cease using the name, and reconfigure."

#### Characters and length

For the **ComputerName**, Apple documents no character restriction and no length limit anywhere located across the pages cited. The `SCDynamicStoreCopyComputerName` signature carries a `CFStringEncoding` out-parameter, from which this project *infers* that the name is not necessarily ASCII — same relabelling as above; Apple documents the out-parameter, not the conclusion. Observed on this machine: it is not ASCII.

For the **LocalHostName**, the governing limits are the mDNS ones:

> Multicast DNS names may be up to 255 bytes long (in the on-the-wire message format), not counting the terminating zero byte at the end.
>
> "Domain Names - Implementation and Specification" [RFC1035] says:
>
>    Various objects and parameters in the DNS have size limits.  They are listed below.  Some could be easily changed, others are more fundamental.
>
>    labels          63 octets or less
>
>    names           255 octets or less

> all names in Multicast DNS MUST be encoded as precomposed UTF-8 [RFC3629] "Net-Unicode" [RFC5198] text.

> For names that are restricted to US-ASCII [RFC0020] letters, digits, and hyphens, the UTF-8 encoding is identical to the US-ASCII encoding, so this is entirely compatible with existing host names.

— RFC 6762, Appendix C and Section 16, `https://www.rfc-editor.org/rfc/rfc6762.txt`. Read 2026-08-16. Attribution verified: the 255-byte passage and the nested RFC 1035 excerpt are in Appendix C; the Net-Unicode and US-ASCII sentences are in §16, Multicast DNS Character Set.

For the **HostName**, the POSIX limit is `{HOST_NAME_MAX}` (quoted above); the observed value of that constant on this machine is in Observed.

#### Steadiness — macOS

- **Rename.** Documentation-only, per the Limits. Apple documents that a user can change the ComputerName in System Settings ▸ General ▸ About and the LocalHostName in Sharing settings, and documents the derivation between them. Apple does **not** state whether editing one silently rewrites the other; that is unresolved.
- **Network move.** Apple documents nothing about the ComputerName or LocalHostName changing on a network move. What the mDNS specification does document is that the *conflict* is per-link (RFC 6762 §3, with the inference note above), so moving a Mac to a link where its name is already taken is exactly the situation in which the loser "MUST cease using the name". Whether that renumbering persists after leaving the link is unresolved.
- **Reinstall.** Not found in Apple's documentation. Unresolved.
- **Hardware change.** Not found in Apple's documentation. Unresolved.
- **Derived from what.** The LocalHostName is documented as derived from the ComputerName ("your computer’s name with .local added, and any spaces are replaced with hyphens"). **Nothing in Apple's documentation derives the ComputerName from hardware, from a serial number, or from an account.**

### Documented — Windows

*(Documentation only. No Windows machine was used.)*

#### How many names, and what each is for

Microsoft documents a computer as carrying a **NetBIOS name** and a **DNS host name**, plus a **DNS domain name** and the **fully qualified DNS name** that combines the last two.

> DNS names consist of one or more components separated by a period (for example, msdn.microsoft.com). Each component can be up to 63 bytes. Each name can be up to 255 bytes total. DNS names are represented in the UTF-8 character set or Unicode. The name is not case sensitive.

> A computer is uniquely identified by its fully qualified DNS name, which consists of its DNS host name and the name of the DNS domain to which it is assigned.

> NetBIOS names consist of up to 15 bytes of OEM characters including letters, digits, hyphens, and periods. Some characters are specific to the character set. NetBIOS names are typically represented in the OEM character set. The OEM character set depends on the locale. Some OEM character sets represent certain characters as two bytes. NetBIOS names, by convention, are represented in uppercase where the translation algorithm from lowercase to uppercase is OEM character set dependent.

> By convention, the NetBIOS name and the DNS host name are interdependent. When you modify the DNS name, the NetBIOS name is also updated. The NetBIOS name is the OEM representation of the DNS host name up to MAX\_COMPUTERNAME\_LENGTH characters. If you set a DNS host name of more than MAX\_COMPUTERNAME\_LENGTH characters, the NetBIOS name is set to a truncated version of the DNS host name. Otherwise, the whole DNS host name is translated into the OEM NetBIOS name. Warning: If you modify the NetBIOS name so that it is not a truncated mapping of the DNS name, you will break applications that use functions such as DnsHostnameToComputerName which rely on this convention.

— `https://learn.microsoft.com/en-us/windows/win32/sysinfo/computer-names`, read 2026-08-16.

So the NetBIOS name is documented as **derived from** the DNS host name by OEM re-encoding and truncation — and the two can be forced out of step deliberately, which Microsoft labels a warning.

#### How a program reads them

> Retrieves a NetBIOS or DNS name associated with the local computer. The names are established at system startup, when the system reads them from the registry.

> ```cpp
> BOOL GetComputerNameExW(
>   [in]      COMPUTER_NAME_FORMAT NameType,
>   [out]     LPWSTR               lpBuffer,
>   [in, out] LPDWORD              nSize
> );
> ```

> **ComputerNameNetBIOS** | The NetBIOS name of the local computer.
> **ComputerNameDnsHostname** | The DNS host name of the local computer.
> **ComputerNameDnsDomain** | The name of the DNS domain assigned to the local computer.
> **ComputerNameDnsFullyQualified** | The fully qualified DNS name that uniquely identifies the local computer. This name is a combination of the DNS host name and the DNS domain name, using the form *HostName*.*DomainName*.

> The length of the name may be greater than MAX\_COMPUTERNAME\_LENGTH characters because DNS allows longer names. To ensure that this buffer is large enough, set this parameter to **NULL** and use the required buffer size returned in the *lpnSize* parameter.

> If group policy is not set for the local machine, the **GetComputerNameEx** function retrieves the NetBIOS or DNS names established at system startup. If group policy is set, the function returns the primary domain name set by group policy. Name changes made by the SetComputerName or SetComputerNameEx functions do not take effect until the user restarts the computer.

> If the local computer is not configured to use DNS names, **GetComputerNameEx** will not return DNS information.

— `https://learn.microsoft.com/en-us/windows/win32/api/sysinfoapi/nf-sysinfoapi-getcomputernameexw`, read 2026-08-16. Library `Kernel32.lib`, DLL `Kernel32.dll`.

**A trim the check flagged:** the four `ComputerName…` rows above are presented as the cells' content and are not. Each cell carries a further sentence — for example `ComputerNameNetBIOS` reads in full "The NetBIOS name of the local computer. If the local computer is a node in a cluster, lpBuffer receives the NetBIOS name of the cluster virtual server." Not meaning-changing here, since clusters are discussed immediately below.

The same enumeration also has `ComputerNamePhysicalNetBIOS`, `ComputerNamePhysicalDnsHostname`, `ComputerNamePhysicalDnsDomain` and `ComputerNamePhysicalDnsFullyQualified`, documented as returning the local computer's own name rather than a cluster virtual server's name when the machine is a cluster node. A sentence claiming that "on a non-clustered desktop the two families are documented to return the same thing" was **cut**: neither the `GetComputerNameEx` page nor the `COMPUTER_NAME_FORMAT` page says anything about what the two families do on a machine that is not a cluster node. Probably true; not carried by the cited page — and, this being a desktop-only project, that was the load-bearing half.

**Privilege.** Microsoft's `GetComputerNameEx` page states no privilege requirement and no prompt; verified, the page has no privilege language. Elevation is documented only for the write side:

> The process that calls the **SetComputerNameEx** function must have administrator privileges on the local computer.

> Sets a new NetBIOS or DNS name for the local computer. Name changes made by **SetComputerNameEx** do not take effect until the user restarts the computer.

> The new name. The name cannot include control characters, leading or trailing spaces, or any of the following characters: " / \ [ ] : | < > + = ; , ?

— `https://learn.microsoft.com/en-us/windows/win32/api/sysinfoapi/nf-sysinfoapi-setcomputernameexw`, read 2026-08-16.

#### The login account's name

> Retrieves the name of the user associated with the current thread.

> A pointer to the buffer to receive the user's logon name. If this buffer is not large enough to contain the entire user name, the function fails. A buffer size of (UNLEN + 1) characters will hold the maximum length user name including the terminating null character. UNLEN is defined in Lmcons.h.

> If the current thread is impersonating another client, the **GetUserName** function returns the user name of the client that the thread is impersonating.

> If **GetUserName** is called from a process that is running under the "NETWORK SERVICE" account, the string returned in *lpBuffer* may be different depending on the version of Windows. On Windows XP, the "NETWORK SERVICE" string is returned. On Windows Vista, the “<HOSTNAME>$” string is returned.

— `https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-getusernamew`, read 2026-08-16. Library `Advapi32.lib`.

The display name is a *separate format*, retrieved by a different call:

> Retrieves the name of the user or other security principal associated with the calling thread. You can specify the format of the returned name.

> The format of the name. This parameter is a value from the EXTENDED_NAME_FORMAT enumeration type. It cannot be **NameUnknown**. If the user account is not in a domain, only **NameSamCompatible** is supported.

> **ERROR\_NO\_SUCH\_DOMAIN** | The domain controller is not available to perform the lookup
> **ERROR\_NONE\_MAPPED** | The user name is not available in the specified format.

— `https://learn.microsoft.com/en-us/windows/win32/api/secext/nf-secext-getusernameexw`, read 2026-08-16. Library `Secur32.lib`.

> `NameSamCompatible` Value: *2* A legacy account name (for example, Engineering\JSmith). The domain-only version includes trailing backslashes (\).
>
> `NameDisplay` Value: *3* A "friendly" display name (for example, Jeff Smith). The display name is not necessarily the defining relative distinguished name (RDN).
>
> `NameUserPrincipal` Value: *8* The user principal name (for example, someone@example.com).

— `https://learn.microsoft.com/en-us/windows/win32/api/secext/ne-secext-extended_name_format`, read 2026-08-16.

That combination is the answer to "are the displayed name and the underlying account name the same thing" on Windows: they are documented as different formats of the same principal, and on a machine that is not domain-joined **the friendly display name is documented as unavailable through this API** — "If the user account is not in a domain, only **NameSamCompatible** is supported."

The account name's own limits, on the domain side:

> The logon name used to support clients and servers running earlier versions of the operating system, such as Windows NT 4.0, Windows 95, Windows 98, and LAN Manager.
>
> This attribute must be 20 characters or fewer to support earlier clients, and cannot contain any of these characters:
>
> - "/ \ [ ] : ; | = , + \* ? < >

> | Size | 20 characters or fewer. |
> | Update Frequency | This value should be assigned when the account record is created, and should not change. |

— `https://learn.microsoft.com/en-us/windows/win32/adschema/a-samaccountname`, read 2026-08-16.

#### Characters and length

> - **Allowed characters:** NetBIOS computer names can contain all alphanumeric characters except for the extended characters that appear in the following **Disallowed characters** list. Names can contain a period, but names can't start with a period. As the NetBIOS computer name is the same as the DNS host name, the DNS host name restrictions also apply.

> - **Disallowed characters:** NetBIOS computer names can't contain the following characters: backslash (\) slash (/) colon (:) asterisk (\*) question mark (?) quotation mark (") less than sign (<) greater than sign (>) vertical bar (|) comma (,)

> - **Character case**: Although NetBIOS is case-sensitive, Windows uses only uppercase names, and in practice, the names aren't case-sensitive.

> - **Name length rules:** Minimum name length: One character. Maximum name length: 15 characters. Note: The 16th character of a NetBIOS computer name is reserved for identifying the functionality that is installed on the registered network device.

> - **Allowed characters:** DNS names can contain only alphabetic characters (A-Z), numeric characters (0-9), the minus sign (-), and the period (.). Period characters are allowed only when they're used to delimit the components of domain style names.

> - The DNS host name registration process substitutes a hyphen (-) character for invalid characters.

> - Windows doesn't permit computer names that exceed 15 characters, and you can't specify a DNS host name that differs from the NetBIOS host name.

> - Computers that are members of an Active Directory domain can't have names that contain only numerals. This is a DNS restriction.

> - **Additional rules for DNS host names:** All characters preserve their case formatting except for American Standard Code for Information Interchange (ASCII) characters. The first character in a DNS host name must be alphabetic or numeric. The last character must not be a minus sign or a period.

— `https://learn.microsoft.com/en-us/troubleshoot/windows-server/active-directory/naming-conventions-for-computer-domain-site-ou`, read 2026-08-16. (Original KB 909264, confirmed on the page.) One trim to note: the 15-character sentence continues on the page with "However, you might create host headers for a website that's hosted on a computer. In that case, the host headers are subject to this rule." — dropped, and not meaning-changing for the use made of it.

**A tension between two Microsoft pages, both quoted here**: the Win32 `GetComputerNameEx` page says "The length of the name may be greater than MAX_COMPUTERNAME_LENGTH characters because DNS allows longer names", while the naming-conventions page says "Windows doesn't permit computer names that exceed 15 characters". Both are Microsoft's own words; the API is documented to tolerate longer DNS names than Windows is documented to let you configure.

#### Collisions and the default name

Microsoft documents that Windows **generates a random computer name** when none is supplied, and that the user is not asked for one:

> **Note** In Windows 10, users can no longer enter a computer name during OOBE as the name is auto-generated. To set a default computer name pre-OOBE, OEMs can configure `ComputerName` in the Unattend.xml file and specify a name for the computer. After OOBE, end users can change this default computer name after OOBE by changing it in the **System Properties** page.

> If `ComputerName` is not specified, a random computer name is generated.

> If `ComputerName` set to an asterisk (\*) or is included but empty (""), Windows creates a random 15-character name using up to 7 characters from FullName and Organization, then a dash, then more random characters.

> `ComputerName` is a string with a maximum length of 15 bytes of content:
> - `ComputerName` can use ASCII characters (1 byte each) and/or multi-byte characters such as Kanji, so long as you do not exceed 15 bytes of content.
> - `ComputerName` cannot use spaces or any of the following characters: { | } ~ [ \ ] ^ ' : ; < = > ? @ ! " # $ % ` ( ) + / . , \* &, or contain any spaces.
> - `ComputerName` cannot use some non-standard characters, such as emoji.

> Computer names that cannot be validated through the DnsValidateName function cannot be used, for example, computer names that only contain numbers (0-9).

— `https://learn.microsoft.com/en-us/windows-hardware/customize/desktop/unattend/microsoft-windows-shell-setup-computername`, read 2026-08-16.

Two computers **can** carry the same Windows computer name. The documented routes, with their labels:

1. **A user simply choosing it — documented.** Microsoft's guidance is "Use a unique name for every computer in your organization" (`https://learn.microsoft.com/en-us/troubleshoot/windows-server/active-directory/naming-conventions-for-computer-domain-site-ou`, DNS host names ▸ Best practices, read 2026-08-16) — advice, not enforcement — and `SetComputerNameEx` documents no already-in-use check, only a character check.
2. **A vendor default — the quote is documented, the consequence is inferred.** Microsoft documents that an OEM "can configure `ComputerName` in the Unattend.xml file and specify a name for the computer". That a fixed literal there would yield identical names on every unit of that model is **this project's reasoning; Microsoft does not state it.**
3. **Not from the out-of-box default, when it is left alone** — "If `ComputerName` is not specified, a random computer name is generated". The `*` form is documented as "up to 7 characters from FullName and Organization, then a dash, then more random characters"; that two users with the same full name would **still differ in the random tail is inferred** — Microsoft documents the name as random, not as collision-free.

What happens when two do, on the NetBIOS side, from the NetBIOS specification itself:

> A unique name should be held by only one station at a time.  However, duplicates ("name conflicts") may arise due to errors.

> Name conflict exists when a unique name has been claimed by more than one node on a NetBIOS network.  B, M, and NBNS nodes may detect a name conflict.  The detection mechanism used by B and M nodes is active only during name discovery.  The NBNS may detect conflict at any time it verifies the consistency of its name database.

> The period in which B and M nodes examine responses is limited by a conflict timer, CONFLICT_TIMER.  The accuracy or duration of this timer is not crucial: the NetBIOS system will continue to operate even with persistent name conflicts.

> Any end-node that receives NAME CONFLICT DEMAND is required to update its "local name table" to reflect that the name is in conflict.

> Logically, a marked name does not exist on that node.  This means that the node should not defend the name (for name claim purposes), should not respond to a name discovery requests for that name, nor should the node send name refresh messages for that name.  Furthermore, it can no longer be used by that node for any session establishment or sending or receiving datagrams.  Existing sessions are not affected at the time a name is marked as being in conflict.

> The only valid user function against a marked name is DELETE NAME.  Any other user NetBIOS function returns immediately with an error code of "NAME CONFLICT".

— RFC 1001, "Protocol Standard for a NetBIOS Service on a TCP/UDP Transport: Concepts and Methods", Sections 5.1 and 15.1.3.5, `https://www.rfc-editor.org/rfc/rfc1001.txt`. Read 2026-08-16. Section attribution verified.

Contrast with macOS: **NetBIOS marks a conflicting name dead on any node that receives the demand, and every function against it errors; RFC 1001 documents no renumbering.** The system is not documented as repairing itself into two distinct names the way mDNS does. **Narrowed by the check** from "marks the loser's name dead" and "does not renumber it": RFC 1001 has no winner and no loser at this layer — there is no probing tie-break — and the absence of renumbering in the text was stated as a positive fact about the protocol.

#### Steadiness — Windows

- **Rename.** Documented, and documented as *deferred*: "Name changes made by **SetComputerNameEx** do not take effect until the user restarts the computer." Also documented as *coupled*: renaming the DNS host name rewrites the NetBIOS name automatically ("When you modify the DNS name, the NetBIOS name is also updated").
- **Network move.** Microsoft documents the name as read from the registry at startup — "The names are established at system startup, when the system reads them from the registry" — not from the network. What a network move can change is what `GetComputerNameEx` returns for the *domain* and *fully qualified* formats: "If group policy is set, the function returns the primary domain name set by group policy", and "If the local computer is not configured to use DNS names, **GetComputerNameEx** will not return DNS information." So the host-name portion is documented as local and stable; the domain and FQDN portions are documented as network-dependent.
- **Reinstall.** Microsoft documents that a fresh install with no `ComputerName` specified generates a *random* name. That is documentation of the install-time behaviour, not of survival. **"The name does not survive a reinstall" is the reading that follows from it; Microsoft does not say it in those words** — see Unresolved.
- **Hardware change.** Not found in Microsoft's documentation. Unresolved.
- **Derived from what.** The NetBIOS name is documented as derived from the DNS host name. The random default is documented as derived from `FullName` and `Organization` plus random characters. **Nothing documents derivation from hardware.**

### Documented — Linux

*(Documentation only. No Linux machine of any kind was available.)*

#### How many names, and what each is for

systemd documents **three**, by exactly those names:

> systemd-hostnamed.service(8) and this tool distinguish three different hostnames: the high-level "pretty" hostname which might include all kinds of special characters (e.g. "Lennart's Laptop"), the "static" hostname which is the user-configured hostname (e.g. "lennarts-laptop"), and the transient hostname which is a fallback value received from network configuration (e.g. "node12345678"). If a static hostname is set to a valid value, then the transient hostname is not used.

> Note that the pretty hostname has little restrictions on the characters and length used, while the static and transient hostnames are limited to the usually accepted characters of Internet domain names, and 64 characters at maximum (the latter being a Linux limitation).

— `https://www.freedesktop.org/software/systemd/man/latest/hostnamectl.html` (systemd 261.2), read 2026-08-16. This host refused the fetch tool with **HTTP 403 Forbidden, response body not retrieved**, for both agents, on all three freedesktop addresses used in this fact; the same addresses returned **HTTP 200** to a direct `curl` from this machine. The quotes are from those same addresses, retrieved directly. This is not a secondary source; it is the same page.

The relationship between pretty and static is documented as a one-way simplification:

> By default, this will alter the pretty, the static, and the transient hostname alike; however, if one or more of --static, --transient, --pretty are used, only the selected hostnames are changed. If the pretty hostname is being set, and static or transient are being set as well, the specified hostname will be simplified in regards to the character set used before the latter are updated. This is done by removing special characters and spaces. This ensures that the pretty and the static hostname are always closely related while still following the validity rules of the specific name. This simplification of the hostname string is not done if only the transient and/or static hostnames are set, and the pretty hostname is left untouched.

— same address, read 2026-08-16.

Note how closely this mirrors the macOS ComputerName → LocalHostName derivation, and how it differs: on Linux the simplification is documented to happen **only** when the pretty name is the one being set, so the two can be driven out of step deliberately.

The precedence between the static, transient and fallback forms:

> The /etc/hostname file configures the name of the local system. Unless overridden as described in the next section, systemd(1) will set this hostname during boot using the sethostname(2) system call.

> Effectively, the static hostname has higher priority than a transient hostname, which has higher priority than the fallback hostname. Transient hostnames are equivalent, so setting a new transient hostname causes the previous transient hostname to be forgotten. The hostname specified on the kernel command line is like a transient hostname, with the exception that it has higher priority when the machine boots. Also note that those are the semantics implemented by systemd tools, but other programs may also set the hostname.

> Otherwise, a fallback hostname configured at compilation time will be used ("localhost").

— `https://www.freedesktop.org/software/systemd/man/latest/hostname.html` (systemd 261.2), read 2026-08-16.

#### How a program reads each

The static, actual and pretty forms are separate D-Bus properties:

> The service exposes the following interfaces on the bus: node /org/freedesktop/hostname1 { interface org.freedesktop.hostname1 { methods: SetHostname(in s hostname, in b interactive); SetStaticHostname(in s hostname, in b interactive); SetPrettyHostname(in s hostname, in b interactive); …

> The StaticHostname property exposes the "static" hostname configured in /etc/hostname. It is not always in sync with the current hostname as returned by the gethostname(3) system call. If no static hostname is configured this property will be the empty string.

> The Hostname property exposes the actual hostname configured in the kernel via sethostname(2). It can be different from the static hostname. This property is never empty.

> The PrettyHostname property exposes the pretty hostname which is a free-form UTF-8 hostname for presentation to the user. User interfaces should ensure that the pretty hostname and the static hostname stay in sync. E.g. when the former is "Lennart's Computer" the latter should be "lennarts-computer". If no pretty hostname is set this setting will be the empty string. Applications should then find a suitable fallback, such as the dynamic hostname.

> The HostnameSource property exposes the origin of the currently configured hostname. One of "static" (set from /etc/hostname), "transient" (a non-permanent hostname from an external source), "default" (the value from os-release or the compiled-in fallback).

— `https://www.freedesktop.org/software/systemd/man/latest/org.freedesktop.hostname1.html`, read 2026-08-16.

`HostnameSource` is the one property here that directly answers "how steady is this" at runtime: it reports whether the name in force came from a file, from the network, or from a fallback.

The kernel-level read:

> These system calls are used to access or to change the system hostname. More precisely, they operate on the hostname associated with the calling process's UTS namespace.

> `gethostname()` returns the null-terminated hostname in the character array `name`, which has a size of `size` bytes. If the null-terminated hostname is too large to fit, then the name is truncated, and no error is returned

> **EPERM** For `sethostname()`, the caller did not have the **CAP_SYS_ADMIN** capability in the user namespace associated with its UTS namespace

> SUSv2 guarantees that "Host names are limited to 255 bytes". POSIX.1 guarantees that "Host names (not including the terminating null byte) are limited to **HOST_NAME_MAX** bytes". On Linux, **HOST_NAME_MAX** is defined with the value 64, which has been the limit since Linux 1.0 (earlier kernels imposed a limit of 8 bytes).

> The GNU C library does not employ the `gethostname()` system call; instead, it implements `gethostname()` as a library function that calls `uname(2)` and copies up to `size` bytes from the returned `nodename` field into `name`.

— Linux man-pages project, `gethostname(2)`, `https://git.kernel.org/pub/scm/docs/man-pages/man-pages.git/plain/man/man2/gethostname.2`. Read 2026-08-16.

**Inferred from the UTS-namespace clause, not documented:** that inside a container the hostname a program reads would be the container's, not the host's, and that nothing in the reading would distinguish the two. The man page says the calls operate on the hostname associated with the calling process's UTS namespace; "container", and the claim that nothing in the reading distinguishes the two, are this project's extension of that sentence, and no Linux machine exists here to check it.

**Privilege.** Reading is unprivileged; writing is gated:

> Changing a hostname using this interface is authenticated via polkit.

— `https://www.freedesktop.org/software/systemd/man/latest/org.freedesktop.hostname1.html`, read 2026-08-16.

> **--no-ask-password** Do not query the user for authentication for privileged operations.

— `https://www.freedesktop.org/software/systemd/man/latest/hostnamectl.html`, read 2026-08-16.

So on Linux the *write* path is documented to prompt (polkit) unless suppressed; no prompt is documented for the read path. `sethostname(2)` at the syscall level requires `CAP_SYS_ADMIN`; **no prompt is documented at that level either** — narrowed by the check from "does not prompt at all", which, obvious though it is, is reasoning rather than documentation, with no Linux machine here to observe it.

#### The login account's name

The account name is the `passwd` login name; the display name is the GECOS field of the same record:

> The comment field, also known as the gecos field, is used by various system utilities, such as finger(1). The use of an ampersand here will be replaced by the capitalised login name when the field is used or displayed by such system utilities.

> Unprivileged users may write arbitrary non-ASCII characters in the GECOS field, notably including C1 control characters. When reading this file, use the C locale. Otherwise, the contents may look different than they are.

— shadow-utils, `passwd(5)`, `https://raw.githubusercontent.com/shadow-maint/shadow/master/man/passwd.5.xml`. Read 2026-08-16.

That warning is worth flagging on its own: **the Linux display name is documented as writable by an unprivileged user and as able to contain C1 control characters.**

Account-name rules:

> Usernames may contain only lower and upper case letters, digits, underscores, dashes, or dots. They can end with a dollar sign. Dashes are not allowed at the beginning of the username. Fully numeric usernames and usernames . or .. are also disallowed. It is not recommended to use usernames beginning with . character as their home directories will be hidden in the ls output.

> Usernames may only be up to 256 characters long.

— shadow-utils, `useradd(8)`, CAVEATS, `https://raw.githubusercontent.com/shadow-maint/shadow/master/man/useradd.8.xml`. Read 2026-08-16.

How a program reads them — **inferred:** that the same three-way POSIX distinction quoted under macOS applies here. No Linux-side source is cited for it, and no Linux machine exists here to confirm it. Plus:

> `getlogin()` returns a pointer to a string containing the name of the user logged in on the controlling terminal of the process, or a null pointer if this information cannot be determined.

> `cuserid()` returns a pointer to a string containing a username associated with the effective user ID of the process.

> These functions let your program identify positively the user who is running (`cuserid()`) or the user who logged in this session (`getlogin()`). (These can differ when set-user-ID programs are involved.)

> For most purposes, it is more useful to use the environment variable **LOGNAME** to find out who the user is. This is more flexible precisely because the user can set **LOGNAME** arbitrarily.

— Linux man-pages project, `getlogin(3)`, `https://git.kernel.org/pub/scm/docs/man-pages/man-pages.git/plain/man/man3/getlogin.3`. Read 2026-08-16.

The man-pages project's own words say the environment variable is settable arbitrarily by the user. Read that as documentation that **`LOGNAME` is not an identity**, whatever else it is.

#### Characters and length

> The hostname should be composed of up to 64 7-bit ASCII lower-case alphanumeric characters or hyphens forming a valid DNS domain name. It is strongly recommended that this name contains only a single DNS label, i.e does not contain any dots. This recommendation reflects both usual expectations of applications, and the fact that the Linux kernel limits the length of the system hostname to 64 (i.e. close to the maximum DNS label length of 63) rather than 255 (the maximum DNS domain name length). When applied, invalid characters will be filtered out in an attempt to make the name valid, but obviously it is recommended to use a valid name and not rely on this filtering.

— `https://www.freedesktop.org/software/systemd/man/latest/hostname.html`, read 2026-08-16.

> The static and transient hostnames must each be either a single DNS label (a string composed of 7-bit ASCII lower-case characters and no spaces or dots, limited to the format allowed for DNS domain name labels), or a sequence of such labels separated by single dots that forms a valid DNS FQDN. The hostname must be at most 64 characters, which is a Linux limitation (DNS allows longer names).

— `https://www.freedesktop.org/software/systemd/man/latest/hostnamectl.html`, read 2026-08-16.

And, independent of systemd:

> Each element of the hostname must be from 1 to 63 characters long and the entire hostname, including the dots, can be at most 253 characters long. Valid characters for hostnames are ASCII(7) letters from `a` to `z`, the digits from `0` to `9`, and the hyphen (-). A hostname may not start nor end with a hyphen.

— Linux man-pages project, `hostname(7)`, `https://git.kernel.org/pub/scm/docs/man-pages/man-pages.git/plain/man/man7/hostname.7`. Read 2026-08-16.

The pretty hostname's limits are the absence of limits: "the pretty hostname has little restrictions on the characters and length used" (`hostnamectl`), and it is documented as "a free-form UTF-8 hostname for presentation to the user" (`hostname1`).

#### Collisions and defaults

Two Linux computers can trivially carry the same hostname. systemd documents a compiled-in default that is *identical on every machine that has no other source*:

> Otherwise, a fallback hostname configured at compilation time will be used ("localhost").

— `https://www.freedesktop.org/software/systemd/man/latest/hostname.html`, read 2026-08-16.

That is a **compile-time** default shared by every machine that reaches the fallback. **Narrowed by the check** from "a vendor-set default shared by every affected machine — the strongest collision case of the three systems": what is documented is a compile-time fallback, which is not a vendor's in the sense the Windows OEM case uses the word, and no source ranks the three systems against each other.

systemd also documents an *opt-in* uniquifier. **Inferred, and a comparison no source makes:** that it appears to be the only per-machine differentiator in any of the three systems' hostname documentation.

> If the question mark character "?" appears in the hostname, it is automatically substituted by a hexadecimal character derived from the machine-id(5) when applied, securely and deterministically by cryptographic hashing. Example: "foobar-????-????" will automatically expand to "foobar-92a9-061c" or similar, depending on the local machine ID.

— `https://www.freedesktop.org/software/systemd/man/latest/hostname.html` and `https://www.freedesktop.org/software/systemd/man/latest/hostnamectl.html`, read 2026-08-16. It genuinely appears on both pages.

Note that this is opt-in: the `?` must be present in the configured name. It is not applied to names that do not contain one.

**Nothing in systemd's or the kernel's documentation describes hostname-conflict detection on a network.** No mechanism equivalent to mDNS probing or NetBIOS NAME CONFLICT DEMAND is documented on any of the systemd or kernel man pages consulted here. A Linux machine running an mDNS responder is subject to RFC 6762's rules, quoted under macOS, but that is the responder's behaviour, not the hostname's. **Narrowed by the check** from "Linux does not document": the evidence is four pages — three systemd, one kernel man page — and with no Linux machine available, the honest claim is about the pages actually read.

#### Steadiness — Linux

- **Rename.** Documented: `hostnamectl hostname NAME` changes pretty, static and transient together by default; `/etc/hostname` holds the static form across boots. Whether the change takes effect without a restart is documented implicitly (`SetHostname` sets the kernel hostname live) but not stated as such — see Unresolved.
- **Network move.** Documented, and this is the one system where the network can rewrite the name: "a transient hostname may be set during runtime, for example based on information in a DHCP lease" (`https://www.freedesktop.org/software/systemd/man/latest/hostname.html`, read 2026-08-16). Documented as subordinate to a configured static name — "If a static hostname is set to a valid value, then the transient hostname is not used" — so a machine with `/etc/hostname` set does not take a name from the network, and one without it **may**. (Narrowed by the check from "does": systemd says a transient hostname *may* be set during runtime, and only where NetworkManager or systemd-networkd is doing it.)
- **Reinstall.** Not stated in systemd's documentation. The static hostname lives in `/etc/hostname`, which is documented; the consequence for a reinstall is not. Unresolved.
- **Hardware change.** Not stated. Unresolved. Note the one derived case: a hostname containing `?` is documented as derived from the machine ID, so its steadiness under a hardware change is the machine ID's steadiness — which is Fact 11's question, not this one's.
- **Derived from what.** The static hostname is documented as derivable from the pretty hostname by simplification; a `?` in the name is documented as derived from the machine ID by cryptographic hashing; the fallback is compiled in.

### Observed — Fact 12

All commands run on the machine named in the Limits, on 2026-08-16, as an unprivileged user, in a non-interactive shell. **Nothing was changed.** Every value that identifies this machine or its owner is `[redacted]`; lengths, character classes and the relationships between values are reported instead. Every observation was re-run independently by the check.

**O1 — Platform.**

```
$ sw_vers
ProductName:		macOS
ProductVersion:		[redacted]
BuildVersion:		[redacted]
$ uname -m
arm64
```

*A negative result would have looked like:* a different `ProductVersion`, or `x86_64`. Re-run identical, byte for byte.

**O2 — Three names exist and two of them are set.**

```
$ scutil --get ComputerName
[redacted]
$ scutil --get LocalHostName
[redacted]
$ scutil --get HostName
HostName: not set
```

`ComputerName` and `LocalHostName` each returned a value. `HostName` returned the literal string `HostName: not set`. No password prompt, no elevation, effective UID 501. Re-run identical on all of that.

**The exit status differs between the two runs**, recorded side by side, neither adjudicated:

| Command | First run | Re-run |
|---|---|---|
| `scutil --get ComputerName` | exit status 0 | exit status 0 |
| `scutil --get LocalHostName` | exit status 0 | exit status 0 |
| `scutil --get HostName` | exit status 0 | **exit status 1** |

The difference is confined to the one preference that is unset.

*A negative result would have looked like:* `scutil --get ComputerName` returning `not set` too, or any of the three producing a permission error or a password prompt. The `HostName` line is itself the negative outcome for one of the three names, which is why it is worth stating: **on this machine, the POSIX `HostName` preference is unset.** (Narrowed by the check from "unset on a stock Mac that has never had it set by hand" — a general rule about Macs drawn from one machine, which additionally asserted this machine's configuration history, something no command here establishes.)

*Control:* `scutil --get NoSuchPref` printed the tool's `usage:` block rather than a value, confirming `--get` distinguishes a real preference from a made-up one and is not echoing whatever it is handed. Reproduced.

**O3 — Format of each name.** Measured with Python 3 over the returned strings; values never printed.

The display name contained non-ASCII punctuation; the local host name was ASCII-only and differed from it. Exact lengths and letter counts are redacted.

*A negative result would have looked like:* an ASCII-only display name or identical names. The distinction reproduced, including U+2019.

The **U+2019** is the single most consequential thing observed here. **This machine's user-facing computer name contains a non-ASCII character, and `LocalHostName` is not a byte-for-byte copy of it.** (A clause reading "set through nothing but ordinary macOS setup" was **cut**: the character reproduces, but the name's provenance is not something any of these commands can establish.)

**O4 — The three readings are not three copies of one string.**

```
ComputerName == LocalHostName          ->  false
hostname == sysctl -n kern.hostname    ->  true
uname -n == hostname                   ->  true
hostname == LocalHostName + ".local"   ->  true
```

*A negative result would have looked like:* `hostname == LocalHostName + ".local"` returning false — which is what would happen if `HostName` had been set, since `scutil` documents `HostName` as "the name associated with hostname(1) and gethostname(3)". With `HostName` unset (O2), the POSIX hostname falls through to the Bonjour name plus the mDNS suffix, and that is what was measured. All four comparisons reproduced.

So on this machine, of the three names Apple documents, **two are set and one is unset, and the unset one's consumer silently reports a fourth string derived from the second.** A program calling `gethostname(3)` here gets `LocalHostName` with `.local` appended — not `ComputerName`, and not the bare `LocalHostName` either. The first run inferred this from the `hostname`, `uname -n` and `sysctl` command-line tools; **the check additionally called `gethostname(3)` itself, through libc, and got the same string, equal to `LocalHostName + ".local"`.** The claim now rests on the call it is about rather than on tools that wrap it.

**O5 — `LocalHostName` is consistent with derivation from `ComputerName` by character-class filtering.** Four candidate transformations were applied to `ComputerName` and compared against `LocalHostName`:

```
CN with spaces->hyphens                                      == LHN  ->  false
CN with non-ASCII dropped, then spaces->hyphens              == LHN  ->  true
CN with spaces->hyphens, then non-[A-Za-z0-9-] dropped       == LHN  ->  true
   (exact lengths redacted)
LHN matches ^[A-Za-z0-9-]+$                                  ->  true
CN  matches ^[A-Za-z0-9-]+$                                  ->  false
```

*A negative result would have looked like:* every transformation returning false, which would mean `LocalHostName` had been set independently rather than derived. The first line **is** a negative — the naive space-to-hyphen rule that Apple's support page describes does not by itself reproduce the value, because it leaves the U+2019 in place. The rule that does reproduce it also drops every character outside `[A-Za-z0-9-]`. All five comparison lines reproduced exactly.

Two further points, with exact name details redacted:

- **Case is preserved.** `LocalHostName` retains uppercase letters. macOS did not lowercase the derived label, even though Apple documents that "Local hostnames aren’t case sensitive". The stored value keeps the case; the comparison ignores it.
- **The lengths tell the story on their own.** The non-ASCII punctuation was removed and spaces were substituted rather than removed. This is consistent with the documented derivation plus one undocumented step.

**Framing narrowed by the check, from "is derived" to "is consistent with derivation".** What a single static reading can show is that two candidate transformations reproduce the stored value and two do not — not that macOS computed one name from the other, nor that it would do so again. Settling that needs a rename, which this batch forbids.

**O6 — Reading the names needs no privilege and produces no prompt.**

```
$ id -u
501
```

Every reading in O2–O5 was taken at effective UID 501, in a non-interactive shell with no controlling terminal available for a prompt. No `sudo`, no authentication dialog, no polkit-style agent, no delay.
*A negative result would have looked like:* any of the `scutil --get` calls returning a non-zero exit status with a permission message, or blocking on a password prompt. Re-run: `id -u` → 501, `tty` → "not a tty".

**O7 — The account name, and its two competing readings.**

```
id -un              -> [redacted]   (length redacted)
$USER               -> [redacted]   (length redacted)   equals id -un: yes
basename $HOME      -> [redacted]   (length redacted)   equals id -un: yes
logname             -> root         (4 characters)   equals id -un: NO
```

`logname` — which is `getlogin()` — returned the literal string **`root`**, while `id -un` (`getpwuid(getuid())`) returned the account name, at effective UID 501. **The two disagreed.** Reproduced identically, including `logname` → `root`; macOS `logname(1)` lists `getlogin(2)` under SEE ALSO, and the check additionally called `getlogin(3)` directly through libc, which returned `root` — the same string.

*A negative result would have looked like:* `logname` returning the same string as `id -un`. It did not. (A clause reading "which is what happens in an ordinary interactive login shell" was **cut**: no interactive login shell was run, in either pass, so it asserted an outcome for a case nobody observed.)

**Corrected 2026-08-21: this said the POSIX distinction made concrete — `getlogin()` defined against *the controlling terminal of the current process*. Darwin does not do that, and Apple's own manual says so.** `man getlogin`, read on the Creator's Mac 2026-08-20: the name is *"the login name of the user associated with the current session, as previously set by setlogin()"*, *"inherited by all processes descended from the login shell"*, and in BUGS *"The current implementation (using setlogin()) allows getlogin to succeed even when the process has no controlling terminal."* The kernel matches: `getlogin` copies a string held on the **session** (`bsd/kern/kern_prot.c`), `setlogin` writes it and is superuser-only, and a new session inherits it from its parent's (`bsd/kern/kern_proc.c`), all read 2026-08-20. **No terminal is consulted.** **So the explanation is no longer inferred:** the value is `root` because some ancestor session had its login name set to `root` by a privileged process and it was inherited down. That is this entry's own *terminal lineage runs through a privileged launcher*, minus the terminal — **the lineage is the session**, and *with no controlling terminal* stops being surprising. *(Also refuted: `getlogin()` is not reading the audit user ID — `getauid()` returned auid 501 at the same moment `getlogin()` returned `root`.)* What was observed is that `getlogin()` returned `root` while the process ran at UID 501 with no controlling terminal; **nothing recorded identifies a privileged launcher or a terminal lineage**, and the mechanism was originally stated as fact. What the observation does license, unchanged: **any program that reads the account name via `getlogin()`/`logname` can get a name that is not the logged-in user's** — here, a name that belongs to a different account entirely.

**O8 — The display name is a different string from the account name.**

```
dscl . -read /Users/[redacted] RealName  -> [redacted]  (19 characters, contains a space)
id -F                                    -> [redacted]  (19 characters)
RealName == id -F                        -> true
RealName == id -un                       -> false
```

*A negative result would have looked like:* `RealName` equalling the account name, which would mean macOS keeps one name rather than two. Reproduced. Confirms Apple's documented split: the "Full name" and the "User name" are separate fields, the full name contains a space, and the two are not interchangeable as strings even though Apple documents both as usable at the login window.

**O9 — Length limits in force on this machine.**

```
$ getconf HOST_NAME_MAX            -> 255
$ getconf _POSIX_HOST_NAME_MAX     -> 255
$ getconf LOGIN_NAME_MAX           -> 255
$ getconf _POSIX_LOGIN_NAME_MAX    -> 9
```

And from the macOS SDK headers on this machine (`/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/param.h`):

```
line 103:  #define MAXHOSTNAMELEN  256             /* max hostname size */
line  97:  #define MAXLOGNAME      255             /* max login name length */
```

All reproduced, including the line numbers.

*A negative result would have looked like:* `HOST_NAME_MAX` reporting 64, as the Linux man-pages document for Linux. It reports 255. **The host-name length limit is not the same number on macOS as on Linux** — the Linux figure being documentation only, since no Linux machine exists for this project. And the POSIX minimum guarantee for a login name on this machine is 9 **bytes of storage including the terminating null**, while the actual limit is 255 — so a program that sized a buffer from `_POSIX_LOGIN_NAME_MAX` holds at most **8** characters, and **would** truncate the 9-character name observed in O7, by one character.

> {\_POSIX\_LOGIN\_NAME\_MAX} The size of the storage required for a login name, in bytes (including the terminating null). Value: 9

> {HOST\_NAME\_MAX} Maximum length of a host name (not including the terminating null) as returned from the gethostname() function.

— The Open Group Base Specifications Issue 8, IEEE Std 1003.1-2024, `https://pubs.opengroup.org/onlinepubs/9799919799/basedefs/limits.h.html`. Read 2026-08-16.

**The arithmetic above is the check's correction; the first run's was off by one.** It had said such a buffer "would truncate the 9-character name observed in O7 by exactly nothing, and would truncate a 10-character one". The `getconf` and header readings all reproduced exactly; only the conclusion drawn from them was wrong. Note the asymmetry the quoted definitions expose: **the login-name constant counts the null, the host-name constant does not.**

**O10 — What was *not* observed.** The rename, the network move, the reinstall and the hardware change were **not** observed, because each requires changing this machine. Specifically: a rename would require `sudo scutil --set ComputerName …` or a System Settings edit, then re-reading all three names to see which moved; a network move would require attaching this machine to a different network and, to test the collision case, a second Mac carrying the same name on that link; a reinstall and a hardware change are not performable at all. The check performed none of them either.

### Unresolved — Fact 12

1. **Whether editing macOS's `ComputerName` rewrites `LocalHostName`, and in which direction.** The derivation is documented and the measurement in O5 is consistent with it, but Apple does not document whether the derivation is re-applied on every change, applied once at setup, or broken permanently once `LocalHostName` is edited by hand. *What would settle it:* on a machine that may be changed, set `ComputerName` and re-read `LocalHostName`; then set `LocalHostName` by hand, set `ComputerName` again, and re-read. Three readings, three states.
2. **Why macOS preserves case in the derived `LocalHostName`**, when Apple's own example (`My Computer` → `My-Computer.local`) preserves it but the documentation calls local hostnames case-insensitive. Observed, not documented. *What would settle it:* an Apple statement on the stored form; none was located.
3. **Whether the macOS `ComputerName` survives a macOS reinstall, an erase-and-install, or a Migration Assistant transfer.** No Apple documentation located. *What would settle it:* Apple documentation of where the preference is stored and which install paths preserve it, or a controlled reinstall.
4. **Whether the macOS `ComputerName` has any documented length limit or character restriction.** None located. Observed to permit at least one non-ASCII character at 23 characters. *What would settle it:* an Apple statement; the `CFStringEncoding` out-parameter implies non-ASCII is expected but does not bound it.
5. **Whether a macOS Bonjour renumbering — the appended number — persists after the machine leaves the conflicting link.** Apple documents the renumbering; neither Apple nor RFC 6762 documents its lifetime. RFC 6762 §3 says `.local` names are "meaningful only on the link where they originate", which bears on it but does not answer it. *What would settle it:* two Macs with identical names on one link, then separating them and re-reading.
6. **Whether the Windows computer name survives a Windows reinstall.** Microsoft documents that a fresh install generates a random name when none is supplied; it does not document what happens to an existing name across an in-place upgrade or repair install. *What would settle it:* Microsoft documentation of the upgrade path's handling of the `ComputerName` registry value, or a Windows machine.
7. **Whether the Windows computer name survives a hardware change.** Nothing located. The name is documented as read from the registry at startup, which is a disk-side store, but Microsoft does not state the hardware-change case. *What would settle it:* Microsoft documentation, or a Windows machine.
8. **Windows local (non-domain) account-name character and length rules.** The domain `sAMAccountName` rules were found and are quoted; the local-account case was not. `UNLEN` is referenced by `GetUserName` as the buffer size but its value is not stated on that page. *What would settle it:* a Microsoft Learn page for local account naming, or the value of `UNLEN` from `Lmcons.h`.
9. **Whether the Windows friendly display name is obtainable at all on a machine that is not domain-joined.** `GetUserNameEx` documents "If the user account is not in a domain, only **NameSamCompatible** is supported", which reads as *no*, but Microsoft does not say what a local machine's "Full name" field maps to instead. *What would settle it:* Microsoft documentation of the local-account path, or a Windows machine.
10. **The official Microsoft statement of what a user sees when two Windows computers share a name.** Only RFC 1001's specification text was obtainable; the Microsoft-hosted results were Q&A threads, archived forum threads, and one archived event-message page from a retired documentation set, none of which meets the bar. *What would settle it:* a current Microsoft Learn reference or troubleshooting page for the duplicate-name condition, or the NetBT event-log documentation. **Checked again 2026-08-21 and it stays open, and a plausible reason for closing it was refuted.** The suggestion that NetBIOS over TCP/IP is disabled by default on a modern Windows 11 desktop and being deprecated is not supported by Microsoft's own registers: its deprecated-features page (read 2026-08-20, `ms.date: 2025-09-09`) carries **no row** for NetBIOS, NBT-NS, WINS, LLMNR or mDNS while listing Computer Browser, Remote Mailslots and NTLM — so the register is maintained and NetBIOS is not on it; its removed-features page likewise. **The one Microsoft page found that states a default says the opposite**: *NetbiosOptions*, `"1 | Specifies that NetBIOS is enabled. This is the default value if DHCP is not available."* — though that page's own metadata is `ms.date: 2017-05-02` and it is about an unattended-install setting, so it is not a statement about Windows 11 today either. **And the mDNS half is not established**: the only first-party sentence found is Microsoft recommending DNS-SD and mDNS as alternatives for *service discovery*, which is not a statement that Windows runs a host-name responder that probes and renumbers. **So neither the protocol table nor the suggested replacement is established as current Windows behaviour**, and this stays parked. *A search for exactly this has now failed twice, so a Windows machine is the likelier route.*
11. **Whether a Linux hostname change takes effect without a reboot.** systemd documents `SetHostname` as setting the kernel hostname and `/etc/hostname` as the boot-time source, but does not state the immediacy in those words. *What would settle it:* a Linux machine, or an explicit systemd statement.
12. **Whether the Linux static hostname survives a distribution reinstall.** `/etc/hostname` is documented as the store; the reinstall consequence is not documented. *What would settle it:* a Linux machine, or distribution documentation — which would be a different vendor's documentation for each distribution, and so is not one answer.
13. **Whether any of the three systems documents a hostname as derived from hardware.** None found on any of the three. The only documented derivation from a machine-specific value anywhere is systemd's opt-in `?` substitution from the machine ID, and the machine ID's own steadiness is Fact 11's question.

---

## Fact 13 — elapsed time a wound-back clock does not defeat

*What each supported operating system offers as a reading of elapsed time that winding the computer's clock back does not defeat, and whether any of it survives the app being closed for weeks.*

**The central question is documentation-only on every system, including this one.** Changing this machine's clock, disabling time synchronisation and altering any system setting were all forbidden. Therefore **no claim below about what happens to a reading when the system time is set backwards is observed.** Every such claim is documented or unresolved. Observing it would take: a machine that is not the Creator's, a recorded before-reading of each clock, `sudo systemsetup -setusingnetworktime off` followed by `sudo date` (or `settimeofday`) to a point days in the past, an after-reading of each clock, a reboot, and a further reading — with the machine's original time restored afterwards from an external source. That is a destructive test on a disposable machine.

Two further scoping notes. Apple's shipped manual pages are used here **as Apple documentation** — they are Apple's own text, with a path on this machine rather than a public address, and they are recorded as `documented` with the exact `man` invocation, not as `observed`, because the words are Apple's and not this machine's behaviour. And **TPM's own specification was not fetched**; Microsoft's own pages do not document the TPM clock beyond a one-line test description, quoted below.

### Documented — macOS

#### The clocks the system offers

Apple's shipped manual page for `clock_gettime`, read on this machine with `MANWIDTH=100 man 3 clock_gettime` (macOS [redacted], build [redacted]):

> "`CLOCK_REALTIME` the system's real time (i.e. wall time) clock, expressed as the amount of time since the Epoch. This is the same as the value returned by gettimeofday(2)."

> "`CLOCK_MONOTONIC` clock that increments monotonically, tracking the time since an arbitrary point, and will continue to increment while the system is asleep."

> "`CLOCK_MONOTONIC_RAW` clock that increments monotonically, tracking the time since an arbitrary point like CLOCK_MONOTONIC. However, this clock is unaffected by frequency or time adjustments. It should not be compared to other system time sources."

> "`CLOCK_UPTIME_RAW` clock that increments monotonically, in the same manner as CLOCK_MONOTONIC_RAW, but that does not increment while the system is asleep. The returned value is identical to the result of mach_absolute_time() after the appropriate mach_timebase conversion is applied."

> "Only the `CLOCK_REALTIME` clock can be set, and only the superuser may do so."

**Note on naming, and it is the single easiest mistake in this fact.** Darwin's `CLOCK_MONOTONIC_RAW` is the *sleep-inclusive* clock and `CLOCK_UPTIME_RAW` is the *sleep-exclusive* one. **This is the reverse of the Linux convention** (see the Linux subsection).

**Both halves are documented, but on two different Apple pages, and the check corrected the citation.** Only `CLOCK_UPTIME_RAW`'s sleep-**exclusiveness** is stated on the `clock_gettime(3)` page above — "does not increment while the system is asleep". That page does **not** state that `CLOCK_MONOTONIC_RAW` includes sleep; it says only "tracking the time since an arbitrary point **like CLOCK_MONOTONIC**", and it is `CLOCK_MONOTONIC` that carries the explicit "will continue to increment while the system is asleep". What documents the sleep-inclusive half is Apple's `mach_continuous_time` page, quoted next.

Apple's own developer documentation, `https://developer.apple.com/tutorials/data/documentation/kernel/1462446-mach_absolute_time.json` (title `mach_absolute_time`), read 2026-08-16:

> "Returns current value of a clock that increments monotonically in tick units (starting at an arbitrary point), this clock does not increment while the system is asleep."

> "Prefer to use the equivalent clock_gettime_nsec_np(CLOCK_UPTIME_RAW) in nanoseconds."

`https://developer.apple.com/tutorials/data/documentation/kernel/1646199-mach_continuous_time.json` (title `mach_continuous_time`), read 2026-08-16:

> "Returns current value of a clock that increments monotonically in tick units (starting at an arbitrary point), including while the system is asleep."

> "Prefer to use the equivalent clock_gettime_nsec_np(CLOCK_MONOTONIC_RAW) in nanoseconds."

`https://developer.apple.com/tutorials/data/documentation/foundation/processinfo/systemuptime.json`, read 2026-08-16:

> "The amount of time the system has been awake since the last time it was restarted."

The same page carries Apple's fingerprinting notice, quoted because it is a documented constraint on using the reading at all:

> "This API has the potential of being misused to access device signals to try to identify the device or user, also known as fingerprinting. Regardless of whether a user gives your app permission to track, fingerprinting is not allowed. When you use this API in your app or third-party SDK (an SDK not provided by Apple), declare your usage and the reason for using the API in your app or third-party SDK's `PrivacyInfo.xcprivacy` file."

**The identical notice appears on the `mach_absolute_time` page** — exact-match confirmed on both.

**What Apple's documentation does NOT say.** None of the four pages above states whether `CLOCK_MONOTONIC`, `CLOCK_MONOTONIC_RAW` or `CLOCK_UPTIME_RAW` reset to zero at boot, and none states in words what setting the system time does to them. `CLOCK_MONOTONIC_RAW` is said to be "unaffected by frequency or time adjustments"; `CLOCK_MONOTONIC` and `CLOCK_UPTIME_RAW` carry no such sentence. **Apple's documentation is silent on whether `CLOCK_MONOTONIC` and `CLOCK_UPTIME_RAW` are affected by `settimeofday`.** It is also silent on whether any of them survives a reboot. Verified by search of all four pages. Silence, not denial.

#### Setting the clock, and who may

Apple's shipped `man 2 gettimeofday` (macOS [redacted]):

> "Only the super-user may set the time of day or time zone. If the system securelevel is greater than 1 (see launchd(8) ), the time may only be advanced. This limitation is imposed to prevent a malicious super-user from setting arbitrary time stamps on files. The system time can still be adjusted backwards using the adjtime(2) system call even when the system is secure."

> "[EPERM] A user other than the super-user attempted to set the time."

This is the only Apple sentence found that constrains a superuser from moving the clock backwards — and it is conditional on `kern.securelevel > 1`. On this machine `kern.securelevel` is `0` (observed below), so the constraint does not apply, and the documentation itself notes `adjtime(2)` remains available even when it does.

Apple's shipped `man 8 systemsetup` documents the user-facing controls:

> "-setdate mm:dd:yy  Use this command to set the current month, day, and year."

> "-settime hh:mm:ss  Sets the current time. The provided time argument should be in 24-hour format."

> "-setusingnetworktime on | off  Sets whether using network time is on or off."

So the vendor documents both a command to set the clock and a command to switch off network time, on the same page.

#### What survives a reboot

Apple's shipped `man 8 sysctl` lists the readable kernel variables in a three-column table (name / type / changeable). The relevant rows read verbatim:

> `kern.boottime                            struct        no`

> `kern.clockrate                           struct        no`

"no" in the third column is that page's marking for not changeable via `sysctl`; the page states: "The changeable column indicates whether a process with appropriate privilege can change the value."

**A claim here was outright wrong and the check rewrote it.** The original read: "Apple's `man 3 sysctl` does not document `KERN_BOOTTIME` — a grep of the whole page for 'boottime' returns matches only in unrelated context and there is no descriptive entry. So the meaning of `kern.boottime` … is undocumented by Apple." That grep does not reproduce. Apple's shipped `man 3 sysctl` **does** document `kern.boottime`, in the `CTL_KERN` table:

> `kern.boottime                struct timeval          no`

and again as a descriptive entry:

> "`kern.boottime`  A struct timeval structure is returned.  This structure contains the time that the system was booted."

So Apple documents what `kern.boottime` *means*. What Apple does **not** document is how it is maintained — specifically **whether it is stored at boot or derived from the wall clock**, which is the property that matters here. That narrower gap is real and is carried to Unresolved.

`man 5 utmpx` (macOS [redacted]):

> "The <utmpx.h> header defines the structures and functions for logging user. Currently logged in users are tracked in /var/run/utmpx."

`man 1 last` (macOS [redacted]) declares its data file as:

> "/var/log/utx.log  login data base"

— a file which does not exist on this machine (observed below). **Narrowed twice by the check.** First: what is established is that the documented file is absent while `last` still returns records; that the binary "disagrees" with the page is an inference, not something either the page or the observation states. Second, "undocumented" is too strong — the page **does** document a source, both in FILES and in the `-f` option ("Read the file file instead of the default, /var/log/utx.log"). **What is unknown is where the shipped binary actually reads from on this machine, the documented file being absent.**

#### The scheduled-task facility

Apple's shipped `man 5 launchd.plist` (macOS [redacted]):

> "`StartInterval <integer>`  This optional key causes the job to be started every N seconds. If the system is asleep during the time of the next scheduled interval firing, that interval will be missed due to shortcomings in kqueue(3). If the job is running during an interval firing, that interval firing will likewise be missed."

> "`StartCalendarInterval <dictionary of integers or array of dictionaries of integers>`  This optional key causes the job to be started every calendar interval as specified. Missing arguments are considered to be wildcard. The semantics are similar to crontab(5) in how firing dates are specified."

> "Unlike cron which skips job invocations when the computer is asleep, launchd will start the job the next time the computer wakes up. If multiple intervals transpire before the computer is woken, those events will be coalesced into one event upon wake from sleep."

`StartCalendarInterval` is a **wall-clock** facility (minute/hour/day/weekday/month keys — Apple's page says the semantics "are similar to crontab(5) in how firing dates are specified"). **Inferred, not documented:** that a wound-back clock therefore delays its firing. Apple documents the crontab-like firing dates and says nothing about a wound-back clock. `StartInterval` is documented as unreliable across sleep. **Neither is documented as recording elapsed time; they are triggers, not readings.**

#### File timestamps

Apple's shipped `man 2 utimes`:

> "If times is non-NULL, it is assumed to point to an array of two timeval structures. The access time is set to the value of the first element, and the modification time is set to the value of the second element. The caller must be the owner of the file or be the super-user."

> "In either case, the inode-change-time of the file is set to the current time."

> "[EPERM]  The times argument is not NULL and the calling process's effective user ID does not match the owner of the file and is not the super-user."

So on macOS the **owner of a file** — not merely an administrator — may set that file's access and modification times. That privilege is **documented outright**, and it is worth stressing that it sits with the owner, not only with an administrator. **Inferred from silence, not documented:** that the times may be set backwards as well as forwards. Apple's page states no such thing — it simply imposes no constraint on the values, and nothing on the page contradicts the reading.

The inode-change-time (`ctime`) is documented as always being set to the current time and is not settable through this call; Apple's page does not say whether any other interface can set it. **Apple's `utimes(2)` page is silent on setting `ctime` and on setting APFS birth time** — narrowed by the check from "Apple's documentation is silent", which was a universal negative over documentation not read.

#### Anything hardware-backed, or outside the operating system's own time

Apple Platform Security, "The Secure Enclave", `https://support.apple.com/guide/security/secure-enclave-sec59b0b31ff/web`, read 2026-08-16 (HTTP 200). Every use of the word "clock" on the page is about attack resistance or power monitoring, not timekeeping. (The count was **corrected**: there are four occurrences, not the two quoted — the two below plus the section heading "Power and clock monitors". All four are about attack resistance or power monitoring, so the substance of the claim holds; the count did not.)

> "It's designed to operate efficiently at a lower clock speed that helps to protect it against clock and power attacks."

> "If the monitors detect an illegal operating point, the clocks in the Secure Enclave automatically stop and don't restart until the next SoC reset."

**Apple's Secure Enclave chapter is silent on any trusted clock, secure timestamp, or monotonic counter available to third-party applications. It documents no time source at all.** Verified on re-check: the page contains no occurrence of "monotonic" or "timestamp"; the counters it does document are passcode-attempt counters in Secure Storage lockboxes, not time.

**A wider claim was cut.** The original closed with "There is no Apple-documented hardware time reading that an ordinary application may take." That is a universal negative over the whole of Apple's documentation drawn from one chapter of one guide — and it contradicts this fact's own unresolved item, which says whether some other Apple document describes such a counter is "not settled". The two cannot both stand. What the fetched page supports is the silence stated above, and nothing wider.

`man sntp` (macOS [redacted]) describes only a client that *sets* time from a network server:

> "sntp - A very Simple Network Time Protocol client program"

It offers no elapsed-time reading; the page documents `-S` ("Use `clock_settime(2)` to set the system clock…") and `-s` ("Use `adjtime(2)` to slew the system clock…").

#### How a program reads each thing on macOS

| Reading | Call | Privilege | Prompts |
| --- | --- | --- | --- |
| Wall clock | `clock_gettime(CLOCK_REALTIME)` / `gettimeofday(2)` | none | no |
| Sleep-inclusive monotonic | `clock_gettime(CLOCK_MONOTONIC_RAW)` or `mach_continuous_time()` | none | no |
| Sleep-exclusive monotonic | `clock_gettime(CLOCK_UPTIME_RAW)` or `mach_absolute_time()` | none | no |
| Awake-time since restart | `ProcessInfo.systemUptime` | none | no; but requires a `PrivacyInfo.xcprivacy` declaration per Apple's notice above |
| Boot instant | `sysctl kern.boottime` (`KERN_BOOTTIME`) | none (observed readable as uid 501) | no |
| Setting the wall clock | `settimeofday(2)` | super-user | — |

### Documented — Linux

*(Documentation alone. No Linux machine exists for this project.)*

#### The clocks the system offers

The Linux man-pages project, `clock_gettime(2)`, `https://man7.org/linux/man-pages/man2/clock_gettime.2.html`, read 2026-08-16:

> "`CLOCK_REALTIME`  A settable system-wide clock that measures real (i.e., wall-clock) time. Setting this clock requires appropriate privileges. This clock is affected by discontinuous jumps in the system time (e.g., if the system administrator manually changes the clock), and by frequency adjustments performed by NTP and similar applications via adjtime(3), adjtimex(2), clock_adjtime(2), and ntp_adjtime(3)."

> "`CLOCK_MONOTONIC`  A nonsettable system-wide clock that represents monotonic time since—as described by POSIX—'some unspecified point in the past'. On Linux, that point corresponds to the number of seconds that the system has been running since it was booted."

> "The `CLOCK_MONOTONIC` clock is not affected by discontinuous jumps in the system time (e.g., if the system administrator manually changes the clock), but is affected by frequency adjustments. This clock does not count time that the system is suspended. All CLOCK_MONOTONIC variants guarantee that the time returned by consecutive calls will not go backwards, but successive calls may—depending on the architecture—return identical (not-increased) time values."

> "`CLOCK_MONOTONIC_RAW` (since Linux 2.6.28; Linux-specific)  Similar to CLOCK_MONOTONIC, but provides access to a raw hardware-based time that is not subject to frequency adjustments. This clock does not count time that the system is suspended."

> "`CLOCK_BOOTTIME` (since Linux 2.6.39; Linux-specific)  A nonsettable system-wide clock that is identical to CLOCK_MONOTONIC, except that it also includes any time that the system is suspended. This allows applications to get a suspend-aware monotonic clock without having to deal with the complications of CLOCK_REALTIME, which may have discontinuities if the time is changed using settimeofday(2) or similar."

The same page also records a bound that matters:

> "A call to clock_settime() with a clockid of CLOCK_REALTIME attempted to set the time to a value less than the current value of the CLOCK_MONOTONIC clock."

— that is the documented `EINVAL` condition. It bounds how far back the wall clock may be set *within a single boot*: not below the current `CLOCK_MONOTONIC` value. **It says nothing across reboots.** A further sentence — that on a recently-booted machine `CLOCK_MONOTONIC` is small, "so the bound is weak" — is **the agent's assessment, not a statement on the page**, though the arithmetic behind it is plainly right.

**Linux's suspend-inclusive clock is `CLOCK_BOOTTIME`; its `CLOCK_MONOTONIC_RAW` is suspend-exclusive. This is the opposite of Darwin's naming.**

POSIX, The Open Group Base Specifications Issue 8, `clock_getres`/`clock_gettime`/`clock_settime`, `https://pubs.opengroup.org/onlinepubs/9799919799/functions/clock_getres.html`, read 2026-08-16:

> "All implementations shall support a clock_id of CLOCK_MONOTONIC defined in <time.h>. This clock represents the monotonic clock for the system. For this clock, the value returned by clock_gettime() represents the amount of time (in seconds and nanoseconds) since an unspecified point in the past (for example, system start-up time, or the Epoch). This point does not change after system start-up time. The value of the CLOCK_MONOTONIC clock cannot be set via clock_settime(). This function shall fail if it is invoked with a clock_id argument of CLOCK_MONOTONIC."

> "Appropriate privileges to set a particular clock are implementation-defined."

#### Who may rewind

`capabilities(7)`, `https://man7.org/linux/man-pages/man7/capabilities.7.html`, read 2026-08-16:

> "`CAP_SYS_TIME`  Set system clock (settimeofday(2), stime(2), adjtimex(2)); set real-time (hardware) clock."

`settimeofday(2)`, `https://man7.org/linux/man-pages/man2/settimeofday.2.html`, read 2026-08-16:

> "`EPERM`  The calling process has insufficient privilege to call settimeofday(); under Linux the CAP_SYS_TIME capability is required."

**Documented, in those words:** the same capability that sets the system clock also sets **the hardware clock**. Nothing in **the Linux pages read for this question** places the RTC out of an administrator's reach — narrowed by the check from "Nothing in the Linux documentation", which was a universal negative over all Linux documentation.

`timedatectl(1)` (systemd 261.2), `https://www.freedesktop.org/software/systemd/man/latest/timedatectl.html`, read 2026-08-16:

> "`set-time [TIME]`  Set the system clock to the specified timestamp. This will also update the RTC time accordingly."

> "`set-ntp [BOOL]`  Takes a boolean argument. Controls whether network time synchronization is active and enabled (if available)."

So the vendor documents, in one place, a single command that sets both the system clock and the hardware clock, and another that turns network synchronisation off.

#### What survives a reboot

`proc_uptime(5)`, `https://man7.org/linux/man-pages/man5/proc_uptime.5.html`, read 2026-08-16:

> "`/proc/uptime`  This file contains two numbers (values in seconds): the uptime of the system (including time spent in suspend) and the amount of time spent in the idle process."

This is a since-boot reading; it does not survive a reboot.

`proc_stat(5)`, `https://man7.org/linux/man-pages/man5/proc_stat.5.html`, read 2026-08-16:

> "`btime 769041601`  boot time, in seconds since the Epoch, 1970-01-01 00:00:00 +0000 (UTC)."

`btime` is documented as "seconds since the Epoch" — that is, in wall-clock terms. **That it is therefore only as trustworthy as the wall clock is inference from the units, not a statement on the page.**

`utmp(5)`, `https://man7.org/linux/man-pages/man5/utmp.5.html`, read 2026-08-16, documents record types that are precisely a log of boots and of clock changes:

> "`#define BOOT_TIME     2 /* Time of system boot (in ut_tv) */`"

> "`#define NEW_TIME      3 /* Time after system clock change (in ut_tv) */`"

> "`#define OLD_TIME      4 /* Time before system clock change (in ut_tv) */`"

> "The wtmp file records all logins and logouts. Its format is exactly like utmp except that a null username indicates a logout on the associated terminal. Furthermore, the terminal name ~ with username shutdown or reboot indicates a system shutdown or reboot and the pair of terminal names |/} logs the old/new system time when date(1) changes it. wtmp is maintained by login(1), init(1), and some versions of getty(8) (e.g., mingetty(8) or agetty(8)). **None of these programs creates the file, so if it is removed, record-keeping is turned off.**"

That last sentence is the vendor stating outright that the record can be destroyed by deleting a file. `last(1)`, `https://man7.org/linux/man-pages/man1/last.1.html`, read 2026-08-16, confirms it reads that same file:

> "last looks through the file wtmp (which records all logins/logouts) and prints information about connect times of users."

> "`wtmp`  The system wide login record file. See wtmp(5) for further details."

`journald.conf(5)`, `https://www.freedesktop.org/software/systemd/man/latest/journald.conf.html`, read 2026-08-16, on whether logs persist at all:

> "`Storage=`  Controls where to store journal data. One of 'volatile', 'persistent', 'auto' and 'none'. If 'volatile', journal log data will be stored only in memory, i.e. below the /run/log/journal hierarchy … 'auto' behaves like 'persistent' if the /var/log/journal directory exists, and 'volatile' otherwise (the existence of the directory controls the storage mode). 'none' turns off all storage, all log data received will be dropped".

So on Linux, whether any log survives a reboot at all is a per-distribution configuration choice, documented as **switchable** — the page adds that the default is "determined at compilation time", which is what makes it per-distribution.

**"Destroyable" was corrected out.** Switchable is documented; destroyable is not — and the same page says close to the opposite about switching: "Note that when this option is changed to 'volatile', existing persistent data **is not removed**." What `Storage=none` documents is that future log data is dropped, not that existing data is destroyed.

`journalctl(1)`, `https://www.freedesktop.org/software/systemd/man/latest/journalctl.html`, read 2026-08-16:

> "`--list-boots`  Show a tabular list of boot numbers (relative to the current boot), their IDs, and the timestamps of the first and last message pertaining to the boot."

#### The one Linux mechanism explicitly designed against a backwards clock

This belongs to systemd rather than to the kernel. (An opening line calling it "the most directly relevant documented thing on any of the three systems" is **the agent's judgement, not a claim any source carries**; it is kept only as an explicitly flagged assessment, and a reader weighing the three systems should form their own.)

`systemd(1)` (systemd 261.2), section "System clock epoch", `https://www.freedesktop.org/software/systemd/man/latest/systemd.html`, read 2026-08-16:

> "When systemd is started or restarted, it may set the system clock to the 'epoch'. This mechanism is used to ensure that the system clock remains somewhat reasonably initialized and roughly monotonic across reboots, in case no battery-backed local RTC is available or it does not work correctly."

> "The epoch is the lowest date above which the system clock time is assumed to be set correctly. When initializing, the local clock is advanced to the epoch if it was set to a lower value. As a special case, if the local clock is sufficiently far in the future (by default 15 years, but this can be configured at build time), the hardware clock is assumed to be broken, and the system clock is rewound to the epoch."

> "The epoch is set to the highest of: the build time of systemd, the modification time ('mtime') of /usr/lib/clock-epoch, and the modification time of /var/lib/systemd/timesync/clock."

`systemd-timesyncd.service(8)`, `https://www.freedesktop.org/software/systemd/man/latest/systemd-timesyncd.service.html`, read 2026-08-16:

> "It also saves the local time to disk every time the clock has been synchronized and uses this to possibly advance the system realtime clock on subsequent reboots to ensure it (roughly) monotonically advances even if the system lacks a battery-buffered RTC chip."

> "`/var/lib/systemd/timesync/clock`  The modification time ('mtime') of this file is updated on each successful NTP synchronization or after each SaveIntervalSec= time interval, as specified in timesyncd.conf(5). If present, the modification time of this file is used for the epoch by systemd(1) and systemd-timesyncd.service."

**What an administrator can do to it — and this is where two claims were repaired.** The epoch is documented as the mtime of two ordinary files. `utimes`-class calls set file mtimes; on Linux the file's owner or a privileged process may do so (see the macOS `utimes(2)` quote for the equivalent BSD rule — the Linux `utimensat(2)` page was not fetched, so **treat the Linux privilege rule for setting mtimes as unresolved**). Nothing in systemd's documentation claims the epoch files are protected against being rewound. **systemd's documentation is silent on whether an administrator can lower the epoch.**

- **Cut:** "Both files are under root-owned system directories." No cited page states the ownership or mode of `/usr/lib/clock-epoch` or `/var/lib/systemd/timesync/clock`, and no Linux machine exists on which it was observed — the claim rested on the paths looking like system paths.
- **Relabelled inferred:** that "by its own description lowering the mtime of those files would lower the epoch". That is drawn from the epoch being "the highest of" three mtimes — systemd never says it, and the `systemd-timesyncd.service(8)` page cuts across it: "systemd-timesyncd will advance the clock when it is started and notices that the system clock is before the modification time of /var/lib/systemd/timesync/clock."

#### The scheduled-task facility

`systemd.timer(5)`, `https://www.freedesktop.org/software/systemd/man/latest/systemd.timer.html`, read 2026-08-16:

> "`Persistent=`  Takes a boolean argument. If true, the time when the service unit was last triggered is stored on disk. When the timer is activated, the service unit is triggered immediately if it would have been triggered at least once during the time when the timer was inactive. Such triggering is nonetheless subject to the delay imposed by RandomizedDelaySec=. This is useful to catch up on missed runs of the service when the system was powered down. Note that this setting only has an effect on timers configured with OnCalendar=. Defaults to false."

> "Use `systemctl clean --what=state …` on the timer unit to remove the timestamp file maintained by this option from disk."

The vendor documents the erasure command on the same page as the feature. And `Persistent=` only applies to `OnCalendar=` timers, which are wall-clock timers.

On the monotonic timers:

> "These are monotonic timers, independent of wall-clock time and timezones. If the computer is temporarily suspended, the monotonic clock generally pauses, too. Note that if `WakeSystem=` is used, a different monotonic clock is selected that continues to advance while the system is suspended and thus can be used as the trigger to resume the system."

> "`OnBootSec=`  Defines a timer relative to when the machine was booted up."

So: `OnBootSec=` is wall-clock-proof but boot-relative; `OnCalendar=` with `Persistent=` survives being powered down but is wall-clock-based and its state file is documented as removable.

#### The hardware clock

`rtc(4)`, `https://man7.org/linux/man-pages/man4/rtc.4.html`, read 2026-08-16:

> "This is the interface to drivers for real-time clocks (RTCs)."

> "Most computers have one or more hardware clocks which record the current 'wall clock' time. These are called 'Real Time Clocks' (RTCs). One of these usually has battery backup power so that it tracks the time even while the computer is turned off."

> "A key difference between an RTC and the system clock is that RTCs run even when the system is in a low power state (including 'off'), and the system clock can't. Until it is initialized, the system clock can only report time since system boot, not since the POSIX Epoch. So at boot time, and after resuming from a system low power state, the system clock will often be set to the current wall clock time using an RTC."

util-linux, `hwclock(8)`, `https://man7.org/linux/man-pages/man8/hwclock.8.html`, read 2026-08-16:

> "hwclock is an administration tool for the time clocks. It can: display the Hardware Clock time; set the Hardware Clock to a specified time; set the Hardware Clock from the System Clock; set the System Clock from the Hardware Clock; compensate for Hardware Clock drift; correct the System Clock timescale; set the kernel's timezone, NTP timescale, and epoch (Alpha only); and predict future Hardware Clock values based on its drift rate."

> "`-s, --hctosys`  Set the System Clock from the Hardware Clock."

`rtc(4)` documents the RTC as recording "the current 'wall clock' time" — a wall clock, not an elapsed-time counter — and `hwclock(8)` is a tool for setting it. **That it therefore "offers nothing that a wound-back clock does not defeat" is the agent's inference from that, and no page states it.**

#### How a program reads each thing on Linux

| Reading | Call | Privilege | Prompts |
| --- | --- | --- | --- |
| Wall clock | `clock_gettime(CLOCK_REALTIME)` | none | no |
| Since-boot, suspend-exclusive | `clock_gettime(CLOCK_MONOTONIC)` | none | no |
| Since-boot, suspend-inclusive | `clock_gettime(CLOCK_BOOTTIME)` | none | no |
| Uptime including suspend | read `/proc/uptime` | none — 0444 by kernel registration (U13) | no |
| Boot instant (wall clock) | read `btime` from `/proc/stat` | none — 0444 by kernel registration (U13) | no |
| Boot history | `last`, reading `wtmp` | not documented on `last(1)`; `wtmp` readability is per-distribution | no |
| Setting the clock | `settimeofday(2)` / `timedatectl set-time` | `CAP_SYS_TIME` | not documented |

**Unresolved for Linux, restated here because the table's `wtmp` row depends on it:** whether `wtmp` is readable by an unprivileged process on a typical desktop distribution. `last(1)` does not state a privilege requirement, and **absence of a statement is not a grant.** Its ownership and mode are set by the distribution rather than by the kernel, so there is no single vendor to quote and no first-party route to close it; it wants `ls -l /var/log/wtmp` in a guest of the distribution actually targeted, or that distribution's own packaging.

*Corrected 2026-08-22. Two changes, both from the same finding.* The two privilege cells above read **"none documented"**, and this paragraph covered `/proc/uptime` and `/proc/stat` alongside `wtmp` and ended **"There is no machine on which to test."** Both `/proc` files were closed on 2026-08-21 from the kernel's own source — registered with mode zero, and mode zero means read-for-everyone — and no machine was needed. The evidence and the addresses are in U13.

### Documented — Windows

*(Documentation alone. No Windows machine has run it.)*

#### The clocks the system offers

`GetTickCount64`, `https://learn.microsoft.com/en-us/windows/win32/api/sysinfoapi/nf-sysinfoapi-gettickcount64`, read 2026-08-16:

> "Retrieves the number of milliseconds that have elapsed since the system was started."

> "To obtain the time the system has spent in the working state since it was started, use the QueryUnbiasedInterruptTime function."

**Microsoft's page for `GetTickCount64` is silent on whether it is affected by setting the system time, and silent on whether it includes sleep.** Verified by search: zero occurrences of "sleep", "hibernat" or "suspend"; the page's only mention of adjustment is about *resolution* and `GetSystemTimeAdjustment`. The contrast it draws with `QueryUnbiasedInterruptTime` ("the time the system has spent in the working state") **implies** `GetTickCount64` is biased, i.e. includes sleep — but Microsoft does not say so on this page.

**Microsoft does say it on the conceptual page beside the function page.** *Interrupt Time*, `https://learn.microsoft.com/en-us/windows/win32/sysinfo/interrupt-time`, read 2026-08-20:

> "To retrieve elapsed time that accounts for sleep or hibernation, use the **GetTickCount** or **GetTickCount64** function, or use the System Up Time performance counter."

**So the sleep half is stated, in Microsoft's own words, on another page of Microsoft's own documentation. What is not stated anywhere is the clock-change half** — see U8. *Corrected 2026-08-22: this passage ended "so it is recorded as **implied, not stated**", of both halves at once. The sleep half was closed on 2026-08-21 in U8, and the page that closes it was cited nowhere in this section, so a reader working from the Windows section alone was told "implied" for something this document settles.*

`QueryUnbiasedInterruptTime`, `https://learn.microsoft.com/en-us/windows/win32/api/realtimeapiset/nf-realtimeapiset-queryunbiasedinterrupttime`, read 2026-08-16:

> "Gets the current unbiased interrupt-time count, in units of 100 nanoseconds. The unbiased interrupt-time count does not include time the system spends in sleep or hibernation."

> "The interrupt-time count begins at zero when the system starts and is incremented at each clock interrupt by the length of a clock tick."

> "The interrupt-time count retrieved by the QueryUnbiasedInterruptTime function reflects only the time that the system is in the working state. Therefore, the interrupt-time count is not 'biased' by time the system spends in sleep or hibernation. The system uses biased interrupt time for some operations, such as ensuring that relative timers that would have expired during sleep expire immediately upon waking."

> "**Unlike system time, the interrupt-time count is not subject to adjustments by users or the Windows time service.**"

`QueryInterruptTime`, `https://learn.microsoft.com/en-us/windows/win32/api/realtimeapiset/nf-realtimeapiset-queryinterrupttime`, read 2026-08-16:

> "Gets the current interrupt-time count."

> "The interrupt-time count begins at zero when the system starts and is incremented at each clock interrupt by the length of a clock tick."

> "Unlike system time, the interrupt-time count is not subject to adjustments by users or the Windows time service."

Note that `QueryInterruptTime`'s own page does **not** state that it includes sleep; the statement that Windows keeps a "biased" interrupt time that does is on the `QueryUnbiasedInterruptTime` page. **Microsoft's `QueryInterruptTime` page is silent on suspend** — verified: zero occurrences of "sleep", "hibernat" or "suspend" in the body; "unbiased" appears only as "See also" links.

**The conceptual page answers it by defining the word the two functions are named against.** *Interrupt Time*, address and date above:

> "Unbiased interrupt-time means that only time that the system is in the working state is counted—therefore, the interrupt-time count is not "biased" by time the system spends in sleep or hibernation."

**So the biased count — the one `QueryInterruptTime` returns — includes sleep**, and this is stated rather than inferred; see U9. *Added 2026-08-22: this passage recorded the function page's own silence and stopped there, while the page that removes the silence was cited nowhere in this section.*

`QueryPerformanceCounter`, documented in "Acquiring high-resolution time stamps", `https://learn.microsoft.com/en-us/windows/win32/sysinfo/acquiring-high-resolution-time-stamps`, read 2026-08-16, is the clearest statement Microsoft makes anywhere on this question:

> "QueryPerformanceCounter reads the performance counter and returns the total number of ticks that have occurred since the Windows operating system was started, **including the time when the machine was in a sleep state such as standby, hibernate, or connected standby.**"

> "**Is QPC affected by daylight savings time, leap seconds, time zones, or system time changes made by the administrator?** — No. QPC is completely independent of the system time and UTC."

> "QPC is a difference clock and isn't synchronized to an external time epoch or reference."

> "**How often does QPC roll over?** — Not less than 100 years from the most recent system boot, and potentially longer based on the underlying hardware timer used."

> "The frequency of the performance counter is fixed at system boot".

The same page also documents the accuracy limit that bounds any long measurement:

> "Crystal oscillators that are used in personal computers and servers are typically manufactured with a frequency tolerance of ± 30 to 50 parts per million, and rarely, crystals can be off by as much as 500 ppm."

with a table row "1 week | ± 6.08 seconds" for ±10 ppm. At ±50 ppm the corresponding weekly figure is about ±30 seconds — **arithmetic the agent's, not Microsoft's**, and correct (6.08 × 5 = 30.4).

**So on Windows there is a documented, explicitly time-change-proof, sleep-inclusive counter: QPC. It resets at boot.** All three properties are on the cited page.

#### Who may rewind

`SetSystemTime`, `https://learn.microsoft.com/en-us/windows/win32/api/sysinfoapi/nf-sysinfoapi-setsystemtime`, read 2026-08-16:

> "The calling process must have the SE_SYSTEMTIME_NAME privilege. This privilege is disabled by default. The SetSystemTime function enables the SE_SYSTEMTIME_NAME privilege before changing the system time and disables the privilege before returning."

"Change the system time — security policy setting", `https://learn.microsoft.com/en-us/previous-versions/windows/it-pro/windows-10/security/threat-protection/security-policy-settings/change-the-system-time`, read 2026-08-16:

> "This policy setting determines which users can adjust the time on the device's internal clock. This right allows the computer user to change the date and time associated with records in the event logs, database transactions, and the file system."

> "Constant: SeSystemtimePrivilege"

> "By default, members of the Administrators and Local Service groups have this right on workstations and servers."

> "Users who can change the time on a computer could cause several problems. For example: Time stamps on event log entries could be made inaccurate; Time stamps on files and folders that are created or modified could be incorrect".

**Microsoft states plainly that the holder of this right — by default any Administrator — can make event-log timestamps and file timestamps inaccurate. This is documented, in those words, on Microsoft's own page.** That this "disposes of both of those as evidence on Windows" is **the conclusion drawn from the documented sentence, not part of it** — reasonable, but the agent's, and flagged as such.

#### The Windows Time service, and a documented backwards-correction limit

"Windows Time Service Tools and Settings", `https://learn.microsoft.com/en-us/windows-server/networking/windows-time-service/windows-time-service-tools-and-settings`, read 2026-08-16:

> "The Windows Time service (W32Time) synchronizes the date and time for all computers managed by Active Directory Domain Services (AD DS)."

> "`MaxNegPhaseCorrection`  Specifies the largest negative time correction, in seconds, that the service makes. If the service determines that a change larger than this value is required, it logs an event instead. The value 0xFFFFFFFF is a special case. This value means that the service always corrects the time. The default value for domain members is 0xFFFFFFFF (hexadecimal). The default value for domain controllers is 172,800 (48 hrs). **The default value for stand-alone clients and servers is 54,000 (15 hrs).**"

> "`TimeJumpAuditOffset`  Indicates the time jump audit threshold, in seconds. The offset is stored as an unsigned integer. If the time service adjusts the local clock by setting the clock directly, and the time correction is more than this value, the time service logs an audit event."

> "`EventLogFlags`  Controls which events the time service logs. 0x1: Time jump. 0x2: Source change."

> "`UtilizeSslTimeData`  Indicates whether W32Time uses multiple SSL timestamps to seed a clock that's grossly inaccurate. A value of 1 indicates multiple SSL timestamps are used. The UtilizeSslTimeData registry value refers to the secure time seeding feature. … The default out-of-box value for all Windows versions with this feature is 1 except for Windows Server 2025, where the default value is 0."

Two things follow, both documented. First, on a stand-alone (non-domain) machine, W32Time will **not by itself** pull a badly-wrong clock back more than 15 hours; it logs an event instead. Second, all of these are registry values under `HKLM\SYSTEM\CurrentControlSet\Services\W32Time` — the page names "HKLM\System\CurrentControlSet\Services\W32Time (the root key for W32Time)" and says W32Time "stores several configuration properties as registry entries" — and the same page documents `w32tm /unregister`:

> "`/unregister`  Unregisters W32Time and removes all its configuration information from the registry."

**An administrator can therefore switch the whole mechanism off, per Microsoft's own page — and this one is documented rather than inferred:** the same page states "Membership in the local Administrators group is required to run this tool locally."

#### What survives a reboot

`Win32_OperatingSystem`, `https://learn.microsoft.com/en-us/windows/win32/cimwin32prov/win32-operatingsystem`, read 2026-08-16:

> "`LastBootUpTime`  Data type: datetime. Access type: Read-only. Date and time the operating system was last restarted."

> "`LocalDateTime`  Data type: datetime. Access type: Read-only. Operating system version of the local date and time-of-day."

Both are `datetime`, `Access type: Read-only` — expressed in wall-clock terms. `LastBootUpTime` is documented as read-only through WMI, but Microsoft does not say it is derived from anything other than the system time, and the "Change the system time" page above says the right lets a user change "the date and time associated with records in the event logs, database transactions, and the file system". **Microsoft's `Win32_OperatingSystem` page is silent on what `LastBootUpTime` reads when the clock has been moved** — narrowed by the check from "Microsoft's documentation is silent", a universal over documentation not read.

`SetFileTime`, `https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-setfiletime`, read 2026-08-16:

> "Sets the date and time that the specified file or directory was created, last accessed, or last modified."

> "A handle to the file or directory. The handle must have been created using the CreateFile function with the **FILE_WRITE_ATTRIBUTES** access right."

So on Windows, file creation time itself is settable by anyone with `FILE_WRITE_ATTRIBUTES` on the file — which is more than macOS `utimes(2)` grants, since that page documents `ctime` as always set to the current time and not settable through the call.

**A claim was rewritten here.** The original read "Windows has no equivalent of the unsettable `ctime` that the macOS `utimes(2)` page documents." **No cited Microsoft page says Windows lacks such a field** — it was inferred from `SetFileTime` naming three settable times and saying nothing about a fourth. That is silence, not denial, and from a single API page. What the two pages jointly support is the comparison stated above: Windows documents creation time as settable, macOS documents `ctime` as not settable through `utimes`. **Whether Windows has some other unsettable timestamp is not established here.**

Taken with the "Change the system time" page above, **file timestamps carry little evidential weight on Windows** — softened by the check from "are worthless as evidence on Windows", an absolute Microsoft does not state; it is the agent's conclusion from the two documented facts, and a fair one, but not a quotation.

#### The scheduled-task facility

"Task Scheduler for developers", `https://learn.microsoft.com/en-us/windows/win32/taskschd/task-scheduler-start-page`, read 2026-08-16:

> "The Task Scheduler enables you to automatically perform routine tasks on a chosen computer. The Task Scheduler does so by monitoring whatever criteria you choose (referred to as triggers) and then executing the tasks when those criteria are met."

> "You can schedule a task to execute in response to these triggers: … At a specific time. At a specific time on a daily schedule. … When the system is booted. When a user logs on."

`ITaskSettings::get_StartWhenAvailable`, `https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nf-taskschd-itasksettings-get_startwhenavailable`, read 2026-08-16:

> "Gets or sets a Boolean value that indicates that the Task Scheduler can start the task at any time after its scheduled time has passed."

> "This property applies only to time-based tasks with an end boundary or time-based tasks that are set to repeat infinitely."

> "Tasks that are started after the scheduled time has passed (because of the StartWhenAvailable property being set to True) are queued in the Task Scheduler service's queue of tasks and they are started after a delay. The default delay is 10 minutes."

This is Windows' equivalent of systemd's `Persistent=`: a missed wall-clock trigger fires late. **It is a trigger, not a reading, and it is wall-clock based.**

#### Anything hardware-backed

Microsoft's TPM Base Services page fetched, `Tbsi_Get_TCG_Log`, `https://learn.microsoft.com/en-us/windows/win32/api/tbs/nf-tbs-tbsi_get_tcg_log`, read 2026-08-16, documents only the boot-configuration log:

> "Retrieves the most recent Windows Boot Configuration Log (WBCL), also referred to as a TCG log."

It contains **no mention of a TPM clock or any time facility** — verified, zero occurrences of "clock" on that page.

The one Microsoft page that acknowledges a TPM clock exists is a hardware-certification test description, "TPM 2.0 - Clock Tests", `https://learn.microsoft.com/en-us/windows-hardware/test/hlk/testref/74027550-10b3-46aa-975a-65e9d355ee2f`, read 2026-08-16, whose entire substantive content on the subject is one sentence:

> "This manual test validates Trusted Platform Module (TPM) internal clock consistency."

That is the whole of it — verified on re-check: the word "clock" appears on that page only in its title, its heading, and that one sentence. **Microsoft documents that a TPM internal clock exists and is certification-tested for consistency, and on the two TPM pages read here documents nothing about what it is, whether it advances while the machine is off, whether it can be moved backwards, or how a Win32 application reads it.** (Narrowed by the check to the pages actually fetched; as originally written it was a universal negative over Microsoft's documentation.)

#### How a program reads each thing on Windows

| Reading | Call | Privilege | Prompts |
| --- | --- | --- | --- |
| Wall clock | `GetSystemTimeAsFileTime` / `GetSystemTimePreciseAsFileTime` | none documented | no |
| Ticks since start | `GetTickCount64` | none documented | no |
| Interrupt time | `QueryInterruptTime` | none documented | no |
| Interrupt time excluding sleep | `QueryUnbiasedInterruptTime` | none documented | no |
| Performance counter (time-change-proof, sleep-inclusive) | `QueryPerformanceCounter` + `QueryPerformanceFrequency` | none documented | no |
| Boot instant | WMI `Win32_OperatingSystem.LastBootUpTime` | not documented on that page | no |
| Setting the clock | `SetSystemTime` | `SE_SYSTEMTIME_NAME` (Administrators, Local Service by default) | not documented |
| Setting file times | `SetFileTime` | `FILE_WRITE_ATTRIBUTES` on the file | no |

### Observed — Fact 13

All on the machine named in the Limits, as uid 501, on 2026-08-16. **Nothing was changed.** No clock was set, no time-synchronisation setting touched, no system setting altered. Where a reading identifies the machine, the format is recorded and the value is `[redacted]`.

**O1 — Which clocks this machine exposes, and what each returns.** Command: `/usr/bin/python3` calling `time.clock_gettime` for each named constant, plus `ctypes` into `/usr/lib/libSystem.B.dylib`.

```
CLOCK_REALTIME (id=0) -> 1786897216.239109993
CLOCK_MONOTONIC (id=6) -> 2965890.832733000
CLOCK_MONOTONIC_RAW (id=4) -> 2965891.562168083
CLOCK_MONOTONIC_RAW_APPROX -> not exposed by this python build
CLOCK_UPTIME_RAW (id=8) -> 2026434.523998583
CLOCK_UPTIME_RAW_APPROX -> not exposed by this python build
CLOCK_PROCESS_CPUTIME_ID (id=12) -> 0.047314000
CLOCK_THREAD_CPUTIME_ID (id=16) -> 0.049062541
CLOCK_BOOTTIME -> not exposed by this python build
mach_absolute_time() -> 48634428584711
mach_continuous_time() -> 71181397501036
mach_timebase_info -> numer=125 denom=3
```

*A negative result would have looked like:* `clock_gettime` raising `OSError` with `EINVAL` for a constant, or `mach_continuous_time` being absent from `libSystem` (an `AttributeError`). Neither happened. `CLOCK_BOOTTIME` genuinely is not present — **the Linux name is not a Darwin clock** — and the two `_APPROX` variants are absent from this Python build, not necessarily from the operating system.

Reproduced: same identifiers, same three absent constants, `mach_timebase_info numer=125 denom=3`. The re-run's own values: `CLOCK_REALTIME` 1786898546.657, `CLOCK_MONOTONIC` 2967221.251, `CLOCK_MONOTONIC_RAW` 2967222.026, `CLOCK_UPTIME_RAW` 2027764.988, `mach_absolute_time` 48666359709124, `mach_continuous_time` 71213328625306.

**O2 — Two monotonic identities hold; a third claimed identity does not.**

```
mach_absolute_time in seconds   = 2026475.246
mach_continuous_time in seconds = 2965932.284
CLOCK_UPTIME_RAW seconds        = 2026475.246
CLOCK_MONOTONIC_RAW seconds     = 2965932.284
CLOCK_REALTIME minus kern.boottime sec = 2965931.965
```

(`mach` values converted with the observed timebase 125/3 ns per tick.)

**Two** identities hold to the precision of separate calls: `mach_absolute_time == CLOCK_UPTIME_RAW` (re-run: 4.1 µs apart) and `mach_continuous_time == CLOCK_MONOTONIC_RAW` (re-run: 2.5 µs apart). The first confirms Apple's `clock_gettime(3)` sentence about `CLOCK_UPTIME_RAW` word for word.

*A negative result would have looked like:* either difference exceeding the microseconds of call-to-call drift — for instance `CLOCK_MONOTONIC_RAW` being awake-only, which would have made it equal to `CLOCK_UPTIME_RAW` and put it ten days from the boottime arithmetic. Neither did.

**The third identity is withdrawn.** The first run claimed `CLOCK_REALTIME − kern.boottime == CLOCK_MONOTONIC_RAW`, called all three "identities [that] hold to the precision of separate calls", and concluded that "the machine's sleep-inclusive monotonic clock and its `kern.boottime` are two views of one quantity." **No run supports that.**

| | first run | re-run |
| --- | --- | --- |
| `CLOCK_MONOTONIC_RAW` | 2965932.284 | 2967222.031 |
| `CLOCK_REALTIME − kern.boottime` | 2965931.965 | 2967221.255 |
| difference | **0.319 s** | **0.775668 s** |
| the other two identities, same run | agree to ~0.001 s | agree to 2–4 µs |

Both runs are recorded. **This said neither is adjudicated; ruled 2026-08-31: the re-run's figure is the one that stands.** An earlier run disagreed and the later one is taken — 0.775668 s, stable to 2 µs across six tightly interleaved rounds, against one unrepeated 0.319 s sample. **It will not be run again.** **Nothing rests on it:** `Access` T5 and `Security and licence` L11 both state outright that nothing there rests on any monotonic reading or on any identity between two clock sources, and no document was found that still inherits the withdrawn identity. The ruling did not require choosing between the two runs in any case, because **neither shows equality** — they disagree about the size of the gap, not about whether there is one. In the first run's own printed numbers the third difference is 0.319 s while the other two agree to about a millisecond, a gap three hundred times larger — and 0.319 s already exceeds the "~0.3 s" threshold this observation itself named as a negative result. In the re-run the difference was 0.775668 s and stable to 2 µs across six tightly interleaved rounds (−0.775670, −0.775668, −0.775668, −0.775668, −0.775668, −0.775668) — a fixed offset is a real difference between two quantities, not sampling scatter.

The re-run also showed what that quantity does track on this machine: **`CLOCK_MONOTONIC`**, matched to under a microsecond in every one of those six rounds. The residual against `CLOCK_MONOTONIC_RAW` equalled the `CLOCK_MONOTONIC_RAW − CLOCK_MONOTONIC` gap exactly (+0.775668 s) — which is the frequency adjustment Apple documents `CLOCK_MONOTONIC_RAW` as being free of.

**What survives is far weaker than an identity: at one moment, `CLOCK_REALTIME − kern.boottime` and `CLOCK_MONOTONIC_RAW` agreed to within about a second, out of 2.97 million seconds** — and not with the clock the original named. The "two views of one quantity" conclusion is cut, and the "~0.3 s of call-to-call drift" figure with it, since the same run's other two identities agreed to microseconds. **Anything downstream that rested on the sleep-inclusive clock and `kern.boottime` being one quantity needs re-deciding.**

The relationship between `CLOCK_REALTIME − kern.boottime` and any monotonic clock is not documented anywhere by Apple.

**O3 — Elapsed, awake, and asleep on this machine.**

```
CLOCK_MONOTONIC_RAW (continuous) = 2966262.7 s = 34.33 days
CLOCK_UPTIME_RAW    (awake only) = 2026805.7 s = 23.46 days
difference (time asleep)         = 939457.0 s = 10.87 days
```

`uptime` reported `up 34 days,  7:51`. `sysctl kern.boottime` reported `[boot timestamp redacted]`.

This is a real, large separation: on a laptop that has been up 34 days, the awake-only counter is 11 days short of wall-clock elapsed time. **On this machine, over this 34-day stretch, the choice between `CLOCK_UPTIME_RAW` and `CLOCK_MONOTONIC_RAW` was worth about a third of the elapsed period.**

**Narrowed by the check:** the original stated this as a rule about macOS. What was measured is one laptop's sleep habit over one stretch, twice. The size of the gap on any other machine follows from how much that machine sleeps, which nothing here observes.

Reproduced exactly — re-run gave continuous 2967222.0 s, awake-only 2027765.0 s, difference 939457.0 s, the sleep total identical; `uptime` "up 34 days, 8:15" against "34 days, 7:51", consistent with the later hour; `kern.boottime` byte-identical.

*A negative result would have looked like:* the difference being zero or near-zero, which is what a machine that never sleeps would show, and which would have made the distinction untestable here.

**O4 — Two readings of the same clock advance consistently with each other.** Over a `time.sleep(5)`:

```
delta CLOCK_REALTIME       = 5.005046
delta CLOCK_MONOTONIC      = 5.005049
delta CLOCK_MONOTONIC_RAW  = 5.005056
delta CLOCK_UPTIME_RAW     = 5.005055
```

All four advance together over five seconds while the machine is awake and the clock is undisturbed.

*A negative result would have looked like:* one clock's delta differing from the others by far more than call overhead — a clock frozen at 0.000000, or `CLOCK_REALTIME` jumping because NTP stepped it mid-test.

**The runs differ on tightness**, both recorded, neither adjudicated:

| | first run | re-run |
| --- | --- | --- |
| `CLOCK_REALTIME` | 5.005046 | 5.010023 |
| `CLOCK_MONOTONIC` | 5.005049 | 5.010049 |
| `CLOCK_MONOTONIC_RAW` | 5.005056 | 5.010099 |
| `CLOCK_UPTIME_RAW` | 5.005055 | 5.010101 |
| spread | ~10 µs | **~78 µs** |

The finding that all four agree stands in both runs. The original's "to about 10 microseconds" was a single-run figure stated as a property, and is narrowed to the plain agreement.

**O5 — The monotonic clock does not go backwards.** 200,000 consecutive reads of `CLOCK_MONOTONIC_RAW`:

```
samples=200000  backward_steps=0
```

*A negative result would have looked like:* `backward_steps` greater than zero. Reproduced exactly.

**O6 — A non-settable clock rejects being set, as an unprivileged user.**

```
RESULT: clock_settime(CLOCK_MONOTONIC_RAW) failed: errno=22 Invalid argument
uid = 501
```

The value offered was the clock's own current reading, so no change could have occurred either way. `errno 22` is `EINVAL`, not `EPERM`: **the kernel rejected the *clock*, not the *privilege*.** Reproduced — `rc=-1, errno=22 (Invalid argument), uid=501`; `CLOCK_UPTIME_RAW` and `CLOCK_MONOTONIC` return `EINVAL` likewise.

**Relabelled upward by the check.** The original called this "one small piece of direct evidence", as though observation were its only support. Apple documents the distinction outright, in the `clock_gettime(3)` ERRORS section: "[EINVAL] `clock_id` specifies a clock that isn't settable" against "[EPERM] A user other than the superuser attempted to set the time." **This belongs under documented, with the observation confirming it — not as an observed inference standing alone.**

*A negative result would have looked like:* the call returning success, or failing with `EPERM` (errno 1), which would have meant the clock is settable by a superuser.

**`clock_settime(CLOCK_REALTIME)` was not attempted.** Even offering the current value would have been a change to the machine's clock.

**O7 — What this machine records about its own boot.**

```
kern.boottime: [boot timestamp redacted]
who -b: system boot [redacted]
last reboot:
  reboot time    [redacted]
  shutdown time  Mon Jul 13 10:25
  reboot time    [redacted]
  reboot time    [redacted]
  wtmp begins [redacted]
```

`last` reports boots from **earlier boots as well as the most recent one. Some record of prior boots therefore survives a reboot on macOS.** But:

```
/var/log/utx.log             absent
/var/log/utx.lastlogin       absent
/var/log/wtmp                absent
/var/run/utmpx               EXISTS 5652 bytes  -rw-r--r--  2026-08-16 17:27:34
```

Apple's own `man 1 last` names `/var/log/utx.log`, which does not exist. `/var/run/utmpx` is 5652 bytes, and `sizeof(struct utmpx)` on this machine is **640** bytes — measured by compiling against the shipped SDK header (`_UTX_USERSIZE` 256, `_UTX_IDSIZE` 4, `_UTX_LINESIZE` 32, `_UTX_HOSTSIZE` 256, plus `pid_t`, `short`, `struct timeval` and `ut_pad[16]`). 5652 ÷ 640 = 8.83: **that file does not hold a whole number of records at all.** Meanwhile `last` with no arguments produced 83 lines. **Where macOS `last` gets its history is unresolved.** `strings -a /usr/bin/last | grep '^/var'` returned nothing, so the paths are not recoverable from the binary without privileged tracing.

**Both numbers in the original were wrong and the check rewrote this.** It had read: "`struct utmpx` on Darwin is 628 bytes and 5652/628 = 9 exactly, which is consistent with the nine boot/shutdown/session records `last reboot` displays — but `last` with no arguments produced 83 lines, more than nine records can account for." The arithmetic 628 × 9 = 5652 is correct in itself, but **neither figure is a quantity this machine produces**: the struct measures 640 bytes, not 628, and `last reboot` displays **four** records — three reboots and one shutdown — not nine, as its own output block above shows. The rest reproduced exactly: 5652 bytes, mode and owner unchanged, 83 lines, empty `strings` result. **The underlying point survives and is in fact stronger, since 5652 bytes is not a whole number of records either way.**

*A negative result would have looked like:* `last reboot` showing only the most recent boot, i.e. no pre-reboot history at all. It did not.

`/var/run/utmpx` is `-rw-r--r-- root:daemon`. **An administrator with `sudo` can overwrite it. That is not a documented fact; it is what the mode bits say.**

**O8 — An undocumented persistent counter exists on this machine.**

```
kern.monotonicclock: 117786329
kern.monotonicclock_usecs: 117786329181976 48635910607924
kern.monotonicclock_rate_usecs: 30
kern.monotoniclock_offset_usecs: 1669110948808898
```

117,786,329 seconds is about **3.73 years** — far longer than the 34-day uptime. The offset, 1,669,110,948.808898, read as a Unix time is 2022-11-22. Offset plus counter equals the current `CLOCK_REALTIME` to within a second. Sampled twice, three seconds apart:

```
117786550796020 48641229345613
1669110948808898
117786553810577 48641301694978
1669110948808898
```

The counter advanced 3,014,557 µs (3.01 s) while the offset stayed byte-identical **over those three seconds**.

So macOS maintains a counter whose value spans multiple boots. **Whether it is a genuinely persisted continuous counter or simply `CLOCK_REALTIME − offset` recomputed on each read cannot be distinguished without moving the clock**, which was forbidden. The arithmetic identity (offset + counter = wall clock) is equally consistent with both. `sysctl -a` exposes it to an unprivileged user; **Apple documents none of these four keys** — they appear in neither `man 8 sysctl`'s table nor `man 3 sysctl`, verified on both pages.

*A negative result would have looked like:* the counter reading a value close to the 34-day uptime, or the offset changing between samples seconds apart. Neither happened **at the timescale actually sampled.**

Reproduced: 117787669 s (3.732 years), rate key 30, offset resolving to 2022-11-22T09:55:48 UTC, offset + counter within 0.007 s of `CLOCK_REALTIME`, and 3,021,393 µs of advance over a three-second sample with the offset byte-identical across it. **But the offset is not the constant the original took it for, and the runs differ:**

| | `kern.monotoniclock_offset_usecs` |
| --- | --- |
| first run | 1669110948**808898** |
| re-run | 1669110948**769134** |
| change | −39,764 µs ≈ **−0.0398 s** |

Both values are recorded and neither is adjudicated. What follows either way is that **the offset is constant over seconds and moves over hours** — so the original's unqualified "the offset stayed byte-identical", and the negative-result clause "or the offset changing between samples. Neither happened", hold only at the timescale actually sampled. This matters because O8's reasoning and the unresolved item that follows from it both lean on the offset being fixed.

**O9 — Sleep and wake are recorded, for the most recent pair.**

```
kern.sleeptime: [redacted]
kern.waketime: [redacted]
kern.wake_abs_time: [redacted]
kern.sleep_abs_time: [redacted]
kern.wakereason: [redacted]
```

These expose a **single** sleep/wake pair — the most recent one. They are expressed both in wall-clock seconds and in mach absolute ticks, which is notable: `kern.wake_abs_time` and `kern.sleep_abs_time` are in the sleep-exclusive counter and so are not wall-clock-derived — their magnitude, ≈4.86×10¹³, matches `mach_absolute_time` (4.867×10¹³) rather than `mach_continuous_time` (≈7.12×10¹³).

Reproduced byte-identical; the machine had not slept since. **Two repairs.** *Narrowed:* "a single pair, not a history" — one pair is what these keys expose; that no history is kept anywhere else is inference, not something this observation reaches. *Cut:* "They do not survive a reboot." **That was never observed** — no reboot was performed — and it is not documented, since Apple documents none of these keys. It was an inference sitting inside an Observed section in the voice of something seen. **Whether these keys survive a reboot is unknown.**

*A negative result would have looked like:* these keys being absent, or `kern.sleeptime` being later than `kern.waketime`.

**O10 — The unified log does not reach back seven days on this machine.**

```
/var/db/diagnostics/Persist — 51 files, ~10.4 MB each
 oldest: 0000000000004bd2.tracev3   Aug 13 09:23:47 2026
 newest: 0000000000004c04.tracev3   Aug 16 18:24:35 2026
```

Just over **three days** of retained log on a machine that has been running for 34. Reproduced — 51 files, same oldest file and timestamp, newest still being written, 506 MB in total.

*A negative result would have looked like:* the oldest Persist file being dated more than seven days back, which would have made the unified log a candidate seven-day record. It is not, at least on this machine at this log volume.

**Cut by the check:** "The log rotates by volume, not by age." No cited source states that, and the observation does not show it — uniformly sized files are equally consistent with size-capped files rotated on any policy, including one keyed to age. **What retention policy the unified log applies is not established here**; what is established is the three days of coverage.

**O11 — This machine is not in a state where the documented "advance only" constraint applies.**

```
kern.securelevel: 0
```

Apple's `gettimeofday(2)` says time "may only be advanced" when securelevel is greater than 1. At 0 it may be moved freely by the superuser. *A negative result would have looked like:* a value of 2 or more. Reproduced.

**O12 — An unprivileged process can read the boot time.** Running as uid 501, `sysctl -n kern.boottime` returned `[boot timestamp redacted]` with no prompt and no error. *A negative result would have looked like:* `sysctl: unknown oid` or a permission error. Reproduced.

**Not observed, and why.** Anything about a wound-back clock — forbidden. Anything on Windows or Linux — no such machine. File-timestamp manipulation — demonstrating that `utimes` can backdate a file requires creating a file, which was forbidden; Apple's `man 2 utimes` states the rule and is quoted above. Reboot behaviour — rebooting the Creator's machine is a change.

### Unresolved — Fact 13

**U1 — NARROWED to the two raw counters. What setting the clock backwards does to each Darwin clock.** Apple's documentation says only that `CLOCK_MONOTONIC_RAW` is "unaffected by frequency or time adjustments". It is silent on `CLOCK_UPTIME_RAW`. **`CLOCK_MONOTONIC` and `kern.boottime` are no longer part of this question — they are answered in U2 below** from Apple's own published kernel source: `CLOCK_MONOTONIC` is defined as the wall clock minus the stored boot time, and setting the wall clock moves the stored boot time by exactly the same delta in both directions, so the two move together. *Settles what remains:* on a disposable Mac, record `CLOCK_MONOTONIC_RAW` and `CLOCK_UPTIME_RAW`; disable network time; set the clock back seven days; re-read; reboot; re-read. *(Corrected 2026-08-22. This entry said Apple "is silent on `CLOCK_MONOTONIC` and `CLOCK_UPTIME_RAW`" and asked for **all four clocks and `kern.boottime`** on a disposable Mac. U2 was closed on 2026-08-21 and states four lines below that U1's Darwin half narrows to the two raw counters; the narrowing was never made here, so this entry went on asking for a machine to establish what the entry beneath it had closed.)*

**U2 — Whether `kern.boottime` moves when the clock moves.** *(The original opened by claiming O2 shows `CLOCK_REALTIME − kern.boottime` equals `CLOCK_MONOTONIC_RAW` **exactly**. It does not — see O2: the two runs differ by 0.319 s and 0.775668 s respectively, and no run shows equality. The question is worth keeping; the evidence claimed for it is not.)* On this machine `CLOCK_REALTIME − kern.boottime` tracked `CLOCK_MONOTONIC` closely at the moment of the re-run, and sat within about a second of `CLOCK_MONOTONIC_RAW`. Two structures are consistent with that: `boottime` is stored at boot and the closeness reflects an undisturbed clock, or `boottime` is derived from the wall clock and moves with it. **CLOSED 2026-08-21: it is the second, from Apple's own published kernel source, and no disposable Mac is needed.** Apple documents it in code rather than in prose. `kern.boottime`'s sysctl handler calls `boottime_timeval()` → `clock_get_boottime_microtime()`, which returns the variable `clock_boottime` (`bsd/kern/kern_sysctl.c`, `bsd/kern/kern_time.c`, `osfmk/kern/clock.c`). And `settimeofday()` → `setthetime()` → `clock_set_calendar_microtime()`, whose own header comment reads *"Also adjusts the boottime to keep the value consistent"* and whose body does it in both directions — *"Adjust the boottime based on the delta."*, `TIME_ADD(clock_boottime, …)` moving forwards and `TIME_SUB(clock_boottime, …)` moving backwards, **by exactly the delta**. All from `https://raw.githubusercontent.com/apple-oss-distributions/xnu/main/…`, read 2026-08-20. *(Version caveat: that is the `main` branch; the Creator's running kernel is `xnu-12377.121.10`, for which Apple has published no tag — a fetch returned 404 — and the nearest reachable tag, `xnu-11215.41.3`, carries the identical comment at the identical line. The mechanism is not new.)*

   **The consequence, and it is stronger than this entry supposed.** `kern.boottime` carries **no resistance to a wound-back clock at all**: an administrator who steps the clock back a week gets a `kern.boottime` a week earlier and `now − kern.boottime` unchanged. **`mach_continuous_time` and `mach_absolute_time` are the ones that do not move**, because nothing in that function touches them. **Anything downstream that rested on the sleep-inclusive clock and `kern.boottime` being one quantity is re-decided by that sentence rather than left parked.**

   **And it explains O2's unexplained gap.** `CLOCK_MONOTONIC` on Darwin is *defined* as the wall clock minus the stored boot time — `bsd/…`'s `clock_gettime.c` computes exactly that — so it would be exact; `CLOCK_MONOTONIC_RAW` is a separate tick counter that drifts from it. **The 0.319 s and 0.775668 s are drift between two different clocks, not measurement noise**, and U17's *the arbitrary point is boot to within about a second on this machine* is safe. **U1's Darwin half narrows to the two raw counters**, which the same source answers.

**U3 — What `kern.monotonicclock` actually is.** See O8. Undocumented by Apple; spans 3.7 years; offset constant across samples **seconds apart, but observed to move by about 40 ms between readings roughly an hour apart**. *Settles it:* the U1 test, watching whether `kern.monotonicclock` moves when the wall clock does and whether `kern.monotoniclock_offset_usecs` is adjusted to compensate — plus, separately, locating the file or NVRAM variable that persists the offset across reboots, which needs root and so needs a machine that is not the Creator's.

**U4 — Where macOS `last` reads its boot history.** Apple's `man 1 last` names a file that does not exist. See O7. *Settles it:* `sudo fs_usage -f filesys last` on a disposable Mac, or `sudo dtruss last`, capturing the paths opened. Both need root and a machine that is not the Creator's.

**U5 — Whether that history file is protected in any way.** If it is `/var/run/utmpx`, it is `root:daemon 0644` and any administrator can rewrite it. If it is something else, unknown. Follows from U4.

**U6 — NARROWED to the reach-from-Windows half; the semantics are answered.** Microsoft acknowledges a "TPM internal clock" in a single certification-test sentence and documents nothing else — no API, no persistence semantics, no rewind semantics. The TPM Base Services page fetched documents only the boot log. **The semantics came instead from TCG's own published reference implementation of the TPM and the tpm2-tools manual pages**, quoted with their addresses in the hardware-floor paragraph of the closing summary: Clock is held in non-volatile storage and survives a power cycle; `TPM2_ClockSet` returns `TPM_RCS_VALUE` for a value below the current one, so Clock can only be advanced, and the refusal is in the command rather than in an implementation habit; `tpm2_gettime(1)` provides a signed, remotely verifiable copy; Clock counts time the TPM has been powered on rather than wall time. **And the qualification: *never moves backwards* holds only where `safe` is set.** Clock is written to non-volatile storage every so often rather than on every tick, so after a power loss that was not an orderly shutdown it can come back lower by up to that interval, and the TPM says so by clearing `safe`. **The route this entry named as what settles it is not reachable**: `trustedcomputinggroup.org` returned HTTP 403 on a plain fetch and again with a browser user-agent, 2026-08-20 — Limits §5. *Settles what remains:* whether an ordinary Windows program can reach Clock at all. Microsoft documents `Tbsip_Submit_Command` for desktop apps **with no privilege statement anywhere on the page**, silence in both directions, so it wants a Windows machine running `TPM2_ReadClock` through TBS, first as a standard user and then elevated. *(Corrected 2026-08-22. This entry read open in full and named as what settles it "fetching the … 'TPM 2.0 Library Specification, Part 1: Architecture' and 'Part 3: Commands' from `trustedcomputinggroup.org` … it was not fetched." The semantics were answered on 2026-08-21 and written into the closing summary; this entry was passed over then, while U8, U9, U13 and U14 in the same list all took their markers that day.)*

**U7 — Whether Apple silicon exposes any equivalent hardware monotonic counter.** The Apple Platform Security guide's Secure Enclave chapter documents no time source at all (quoted above). Whether some other Apple document mentions a monotonic counter or replay-protected counter available to applications is **not settled** — the guide has many chapters and one was read. *Settles it:* reading the whole of `https://support.apple.com/guide/security/` and the `CryptoTokenKit` / `Security` framework reference for any documented monotonic counter or trusted timestamp.

**U8 — NARROWED 2026-08-21 to one half; the sleep half is closed.** Microsoft states the sleep behaviour on the conceptual page beside the function page — *Interrupt Time*, `https://learn.microsoft.com/en-us/windows/win32/sysinfo/interrupt-time`, read 2026-08-20: *"To retrieve elapsed time that accounts for sleep or hibernation, use the **GetTickCount** or **GetTickCount64** function, or use the System Up Time performance counter."* **So `GetTickCount64` includes sleep, in Microsoft's own words.** **What is still not stated is the clock-change half.** The same page says the interrupt-time count *"is not subject to adjustments by users or the Windows time service"*, and the `GetTickCount64` page groups the two counters together — *"the interrupt-time count and tick count are advanced by approximately 49 days"* on checked builds — but **nowhere does Microsoft say in words that `GetTickCount64` is unaffected by setting the clock.** Strongly implied, not stated. *Settles it:* a Windows machine, or a Microsoft page that states it.

**U9 — CLOSED 2026-08-21 by Microsoft's own definition of the word.** *Interrupt Time*, address and date above: *"Unbiased interrupt-time means that only time that the system is in the working state is counted—therefore, the interrupt-time count is not "biased" by time the system spends in sleep or hibernation."* **The biased one — `QueryInterruptTime` — therefore includes sleep**, by the definition of the term the unbiased variant is named against.

**U10 — Whether Windows records anything that survives a reboot and an administrator.** Every Windows record found — event log timestamps, file timestamps, `LastBootUpTime` — is stated or implied by Microsoft to be wall-clock derived, and Microsoft's own "Change the system time" page says the right makes event-log and file timestamps inaccurate. **No Microsoft-documented Windows reading was found that survives both a reboot and an administrator.** *Settles it:* a Microsoft page documenting a persistent monotonic counter, or the TPM path in U6 — **which is now answered on the semantics and open only on whether an ordinary Windows program can reach the TPM's Clock, so what this entry is waiting on is a Windows machine and not a specification.** *(Corrected 2026-08-22; this pointed at U6 while U6 still read open in full and named a fetch that returns HTTP 403.)*

**U11 — Windows Event IDs 6013 (system uptime), 12 and 13 (Kernel-General boot/shutdown), and 1 (Kernel-General system time change).** These are widely used in practice but no Microsoft Learn page documenting them was found, and no non-vendor source was cited. *Settles it:* a Microsoft Learn page for the Kernel-General or EventLog provider event manifests.

**U12 — Windows software-licensing grace-period state (`sppsvc`, `slmgr /rearm`).** A Microsoft facility with its own notion of elapsed time and its own rearm limit. Not investigated. *Settles it:* Microsoft's own documentation for the Software Protection Platform and `slmgr`.

**U13 — TWO THIRDS CLOSED 2026-08-21 from the kernel's own source; the `wtmp` half stays open.** Both `/proc` files are registered with mode zero — `proc_create_single("uptime", 0, NULL, uptime_proc_show);` (`fs/proc/uptime.c`) and `proc_create("stat", 0, NULL, &stat_proc_ops);` (`fs/proc/stat.c`) — and mode zero means read-for-everyone: `if ((mode & S_IALLUGO) == 0) mode |= S_IRUGO;` (`fs/proc/generic.c`). All three read 2026-08-20 from `git.kernel.org`. **`wtmp` is not settled and no vendor can settle it**: its ownership and mode are set by the distribution rather than by the kernel, so there is no single authority to quote. *Settles the rest:* `ls -l /var/log/wtmp` in a guest of the distribution actually targeted, or that distribution's own packaging.

**U14 — CLOSED 2026-08-21, and the rule is the same as the BSD one already quoted.** `utimensat(2)`, from the Linux man-pages project's own source at `https://raw.githubusercontent.com/mkerrisk/man-pages/master/man2/utimensat.2`, read 2026-08-20, the `EPERM` entry: setting a timestamp to a value other than the current time fails where *"the caller's effective user ID does not match the owner of file, and the caller is not privileged (Linux: does not have the CAP_FOWNER capability)"* or the file is append-only or immutable. **So root, or anything holding `CAP_FOWNER`, can set any mtime to any value, up or down.** **U15's consequence half does not follow from this and stays inferred** — that lowering those mtimes lowers systemd's epoch is still a step nobody has taken. The privilege half is settled; the consequence half is not.

**U15 — Whether systemd's epoch can be lowered by an administrator.** systemd's documentation defines the epoch as the highest of three mtimes and is silent on protecting them. Lowering all three would appear to lower the epoch, but **systemd does not say so**, and the timesyncd page adds behaviour that cuts across it. *Settles it:* a Linux machine, or a systemd page that addresses it.

**U16 — Whether macOS has any analogue of systemd's epoch advance.** Nothing in the Apple documentation read describes macOS refusing a low clock at boot or advancing it to a floor. *Settles it:* the U1 test — set the clock back a year, reboot, observe whether macOS restores it without a network.

**U17 — Whether `CLOCK_MONOTONIC_RAW` resets at boot on macOS.** Apple's documentation says only "an arbitrary point". *(The original called O2's identity with `wall − kern.boottime` strong evidence that the arbitrary point is boot. That identity does not hold — see O2.)* A weaker reading survives and is enough for the purpose: `CLOCK_MONOTONIC_RAW` sat within about a second of `wall − kern.boottime` out of 2.97 million seconds, in both runs, so **the arbitrary point is boot to within about a second on this machine.** That is still one machine and one boot. *Settles it:* a reboot with a before-and-after reading.

---

## The three questions the batch turns on

Gathered in one place because they are what everything downstream leans on. Each is stated as the evidence leaves it, including where that is a negative or an inference from silence. **Where a route is closed by the vendor's own words, it says so; where it is closed only by the vendor never mentioning the case, it says that instead. Silence is not denial.**

### One — is there an identity a person with full administrative control cannot forge, and what does each such route demand?

**Nothing was tested.** Forgeability was never demonstrated or refuted on any system. Everything below is what the vendors say, sorted by whether they say it.

**Documented that the ordinary routes can be forged, in the vendor's own words — three statements.**

- Microsoft, on the software path: **"There was no guarantee that a key is actually protected by a TPM (someone can easily spoof a software KSP as a TPM KSP with local administrator credentials)."** This is the only place in the batch where a vendor names *administrator* and *spoof* in one sentence about its own mechanism. Its page carries the banner "This content is written by a Microsoft customer support engineer".
- systemd, on the machine ID: **"The machine ID may be set … with the systemd.machine_id= kernel command line parameter or by passing the option --machine-id= to systemd. An ID specified in this manner has higher priority and will be used instead of the ID stored in /etc/machine-id."** No forgery is required; the override is a supported feature, documented in the identifier's own manual. systemd nowhere claims the machine ID resists an adversary — it claims the opposite, by asking that it be treated as "confidential" and hashed before leaving the host.
- Apple, on the MAC-address route only: "A DRM product that relied on these APIs would be vulnerable to an **admin user** using ifconfig to specify the same MAC address on multiple machines." That sentence is about `getifaddrs` and the CFUUID APIs.

**Documented more weakly for the I/O Registry, and inferred for one property.** Apple's word is "spoofed" and its actor is "knowledgeable persons": "it is possible for the built-in MAC address (or any I/O Registry property for that matter) to be spoofed by knowledgeable persons." That reaches `IOPlatformSerialNumber` by Apple's own words. **Its reach to `IOPlatformUUID` is inferred** — TN1103 never mentions `IOPlatformUUID` at all, zero occurrences in the whole document. And `gethostuuid(2)` says its inputs "can themselves be modified" **without saying by whom or at what privilege**.

**Undocumented — neither true nor false.** Whether a `DCDevice` token resists forgery by the machine's administrator. Apple calls it "a unique, authenticated token", offers a `validate_device_token` endpoint, and never explains what the validation proves. Measured over the DeviceCheck framework's own eleven-page index: **"unforgeable", "tamper", "spoof" and "forge" occur zero times**, and the single "Secure Enclave" mention belongs to App Attest, not to `DCDevice`. Treat as undocumented, not documented-false and not documented-true.

**The routes that are documented as resisting forgery, and what each costs.**

| Route | System | What the vendor claims | What it demands | Open to a website-downloaded app? |
|---|---|---|---|---|
| **Managed Device Attestation** | macOS | "strong assurances that the properties of the device (for example, the serial number) are legitimate and not spoofed", "based on the security of the Secure Enclave and the Apple attestation servers" | Apple silicon Mac; macOS 14+; an MDM service and/or an ACME-issuing CA; a server-side verifier validating a chain rooted in Apple's CA; one `DeviceInformation` attestation per device per seven days | **Inferred-closed.** Apple documents only two request paths and both are organisational. **Apple nowhere states that no other path exists and nowhere addresses an unenrolled application.** |
| **App Attest** | macOS | a "hardware-based, cryptographic key" whose private half sits in the Secure Enclave, "from where no process can ever directly read or modify it" | Would require a registered App ID and a server-side verifier | **Documented-closed on Mac**, by the gating property itself: "If you read isSupported from an app running on a Mac device, the value is false." |
| **AD CS TPM key attestation** | Windows | EK-rooted: "The EK is unique for every TPM and can identify it. The EK can't be changed or removed"; the private half "is never revealed or accessible outside the TPM" | **Documented:** an *enterprise* Certification Authority — "TPM key attestation is not supported for a standalone CA"; RSA keys only; Microsoft Platform Crypto Provider KSP. **Inferred:** a domain. The strongest trust model needs an administrator-curated per-device EKPub allowlist | Closed to a consumer application by the enterprise-CA requirement |
| **Azure Attestation** | Windows and Linux | EK/AIK-based proof to a Microsoft-run service | The cited page's emphasis is **boot integrity**, not device identity. **No cost, subscription or provider requirement is on the page** — an earlier claim of those was cut as unsourced | Not established |
| **ASHWID signature validation** | Windows | "The returned ASHWID is signed. This provides a means for the app to verify that the returned ASHWID came from Windows"; a nonce protects against replay | A server-side verifier trusting "Microsoft Assurance Designation Root 2011", checking EKU `1.3.6.1.4.1.311.10.5.40`, and a weighted-score threshold of the developer's own choosing to absorb hardware drift | **Unresolved.** The API is documented as *package*-specific and the guidance page is framed around Store apps. **No Microsoft statement was found on whether an unpackaged Win32 binary can call it, or what it would return.** This is the most consequential open question on Windows |
| **TPM EK on Linux** | Linux | The EK itself is the same hardware property; **carrying Microsoft's Windows-scoped description across to a TPM under Linux is inferred** | **Inferred:** a TPM 2.0 that has an EKCert at all ("Not all TPMs have EKCert"), the manufacturer's chain obtained out of band, and a server-side verifier written and run by the developer. Also `/dev/tpmrm0` requires group `tss` membership, an administrative act — and joining that group does **not** open `/dev/tpm0` | **No Linux equivalent of DeviceCheck or Managed Device Attestation was found in the documentation read.** That is a not-found, not a proof of non-existence — an earlier universal negative was cut |

**And one route documented as closed for a different reason.** On Windows, `GetSystemIdForPublisher` carries the strongest ordinary-application stability contract of anything in the batch — but Microsoft states the outcome for the case that matters directly: **"The system identification string will be identical for all unpackaged (Win32) apps and all packaged apps with no publisher ID."** **Corrected 2026-08-21: that sentence read Microsoft's Note backwards and this is not a closed route.** The Note says a class of **apps** is not told apart; it does not say **machines** are not, and the method page states *"Unique for each system"*. What an unpackaged Win32 app loses is app-scoping — a privacy loss this use does not need — not availability. **It moves to the open routes.** Documentation only; never observed; what `Source` such a caller receives is Unresolved 11 and still open.

**One observation bears on all of this and is worth restating.** `DCDevice.current.generateToken` returned a token, twice, **from a binary with no application bundle, no `Info.plist`, no bundle identifier, no entitlements and no Developer ID** — `Signature=adhoc`, `flags=0x20002(adhoc,linker-signed)`, `TeamIdentifier=not set` — despite Apple's note that "The app you use to generate the token must be associated with your developer account; otherwise, the generation request fails." Reproduced by the check from its own probe. **Its scope is the client call only.** Whether Apple's server would honour such a token is untested, and what Apple documents the token is *for* is addressing two per-device bits on Apple's server, not proving hardware identity to a relying party.

### Two — can two computers carry the same name?

**Yes, on all three systems.** The short answer: on macOS and Windows it takes a user choosing it or a vendor setting it; on Linux it is the documented state of any two machines that reach the compiled-in fallback.

| | Can two carry the same name? | Documented default | What the system does when two do |
|---|---|---|---|
| **macOS** | **Yes** — a user can type any name, and Apple documents the case explicitly | `Macintosh.local` as the Bonjour fallback when the computer name is unusable | Appends a number to the **local network name** only; the `ComputerName` is not documented as changing. Per-link. Which machine keeps the plain name is decided by the mDNS probing tie-break: "The protocol used in the Probing phase will determine a winner and a loser, and the loser MUST cease using the name, and reconfigure." |
| **Windows** | **Yes** — by a user choosing it (Microsoft's "Use a unique name for every computer in your organization" is advice, not enforcement, and `SetComputerNameEx` documents no already-in-use check), or by an OEM setting a fixed `ComputerName` in Unattend.xml. **Not** from the out-of-box default, which is documented as random | A random 15-character name; the `*` form uses "up to 7 characters from FullName and Organization, then a dash, then more random characters" | **This cell describes the NetBIOS protocol, not Windows 11 today** — labelled 2026-08-21, because it reads as a current statement about the system and the document could not source one (Unresolved 12 #10). NetBIOS marks the name as in conflict on **any node that receives the demand**; "Logically, a marked name does not exist on that node", and every NetBIOS function against it returns `NAME CONFLICT`. **RFC 1001 documents no renumbering** — there is no winner and no loser at this layer, and no probing tie-break |
| **Linux** | **Yes** — the compiled-in fallback is `localhost` on every machine that reaches it | `localhost` (compile-time fallback) | **Nothing documented at the hostname layer on the pages consulted.** An mDNS responder on the machine follows RFC 6762's rules, but that is the responder's behaviour, not the hostname's |

**Three labels to carry with that table.** The OEM route on Windows is documented as a quote and **inferred as a consequence** — that a fixed literal yields identical names on every unit of a model is this project's reasoning, not Microsoft's. That two users with the same full name "still differ in the random tail" is likewise **inferred**: Microsoft documents the name as random, not as collision-free. And on Linux, what is documented is only what happens when no source supplies a name — **nothing in systemd's pages says a fresh installation leaves `/etc/hostname` unset**, so "two fresh machines" was narrowed to "two machines that reach the compiled-in fallback".

**Two related negatives, both real.** First: **no system documents a hostname as derived from hardware.** The only documented derivation from a machine-specific value anywhere in the batch is systemd's opt-in `?` substitution from the machine ID — and that is opt-in, requiring a literal `?` in the configured name. Second, on macOS the three names are not three copies of one string: on the machine observed, `ComputerName` is 23 characters and contains a **U+2019**, `LocalHostName` is a 22-character ASCII label, and `gethostname(3)` returns a **fourth** 28-character string equal to `LocalHostName + ".local"` — confirmed by calling `gethostname(3)` itself, not merely the tools that wrap it. Which reading a program takes changes the answer.

**And the account name has the same problem.** `logname` — `getlogin()` — returned the literal string **`root`** while `id -un` returned the account name, at effective UID 501, in both runs, and confirmed by calling `getlogin(3)` directly through libc. POSIX documents the two as defined against different things: `getlogin()` against *the controlling terminal of the current process*, `getpwuid(getuid())` against the process's user ID. **Any program that reads the account name via `getlogin()`/`logname` can get a name that is not the logged-in user's.** **Corrected 2026-08-21: POSIX may define them that way; Darwin does not, and Apple says so in its own manual.** `man getlogin` on the Creator's Mac, read 2026-08-20: *"The getlogin() routine returns the login name of the user associated with the current session, as previously set by setlogin(). The name is normally associated with a login shell at the time a session is created, and is inherited by all processes descended from the login shell. (This is true even if some of those processes assume another user ID, for example when su(1) is used.)"* and, in BUGS: *"In earlier versions of the system, getlogin() failed unless the process was associated with a login terminal. The current implementation (using setlogin()) allows getlogin to succeed even when the process has no controlling terminal."* Apple's kernel does exactly that — `getlogin` copies a string held on the **session** (`bsd/kern/kern_prot.c`), `setlogin` writes it and is superuser-only, and a new session inherits it from its parent's (`bsd/kern/kern_proc.c`); all read 2026-08-20. **No terminal is consulted anywhere in that path.** So the mechanism is no longer inferred: the value is `root` because some ancestor session had its login name set to `root` by a privileged process and it was inherited down. **That is this document's own *terminal lineage runs through a privileged launcher*, minus the terminal — the lineage is the session.** *(One thing refuted with it: `getlogin()` does **not** read the audit user ID. On the Creator's Mac `getauid()` returned auid 501 at the same moment `getlogin()` returned `root`.)* **The practical conclusion is unchanged**: a program reading the account name through `getlogin()` or `logname` can get a name belonging to a different account.

### Three — does anything survive both a reboot and an administrator?

**Nothing in this batch was shown to, and that is corrected 2026-08-21 in one place: the TPM's Clock.** It survives a reboot by living in non-volatile storage, and it refuses to go backwards for an administrator holding owner authorisation — **provided `safe` is set**, which an unclean power loss clears. The evidence and the qualification are with the hardware-floor paragraph below. **What is still not shown is that a Windows application can reach it**, which is a Windows machine and nothing else. Everything else in this negative stands, and it is a negative arrived at three different ways, and the ways differ in strength.

**What survives a reboot but not an administrator.**

- **macOS:** *some* record of prior boots survives — `last` reported boots from 12 and 19 June on a machine that last booted 13 July, observed twice. But **where `last` reads that history is unresolved**: Apple's own `man 1 last` names `/var/log/utx.log`, which does not exist on this machine, and `/var/run/utmpx` at 5652 bytes is not a whole number of 640-byte records. If the source is `/var/run/utmpx`, it is `root:daemon 0644` and **an administrator with `sudo` can overwrite it — which is what the mode bits say, not a documented fact.** Separately, an **undocumented** counter `kern.monotonicclock` spans about 3.73 years across multiple boots, readable unprivileged; **whether it is genuinely persisted or simply `CLOCK_REALTIME − offset` recomputed on each read cannot be distinguished without moving the clock.** Apple documents none of its four keys.
- **Linux:** `wtmp` records boots and, by their own record types, clock changes (`NEW_TIME`, `OLD_TIME`). The vendor states outright that the record can be destroyed by deleting a file: **"None of these programs creates the file, so if it is removed, record-keeping is turned off."** Whether the journal persists at all is a per-distribution compile-time choice (`Storage=`). systemd's clock epoch is the one mechanism explicitly designed against a backwards clock — "the local clock is advanced to the epoch if it was set to a lower value" — but the epoch is **the mtime of two ordinary files**, and **systemd's documentation is silent on whether an administrator can lower it.** That lowering those mtimes would lower the epoch is **inferred**, and the timesyncd page cuts across it. An earlier claim that both files sit in root-owned directories was **cut**: no page states it, and no machine existed to observe it.
- **Windows:** **no Microsoft-documented reading was found that survives both a reboot and an administrator.** Every record found is stated or implied to be wall-clock derived, and Microsoft's own page says the right to change the system time "allows the computer user to change the date and time associated with records in the event logs, database transactions, and the file system", with the explicit consequences that "Time stamps on event log entries could be made inaccurate" and "Time stamps on files and folders that are created or modified could be incorrect". `w32tm /unregister` "removes all its configuration information from the registry", and the same page states "Membership in the local Administrators group is required to run this tool locally" — so switching the correction mechanism off is **documented, not inferred**.

**What resists a wound-back clock but not a reboot.** This is the clean, documented answer on Windows and the shape of the answer everywhere:

- **`QueryPerformanceCounter`** — Microsoft's clearest statement in the batch. "returns the total number of ticks that have occurred since the Windows operating system was started, **including the time when the machine was in a sleep state such as standby, hibernate, or connected standby**", and "**Is QPC affected by daylight savings time, leap seconds, time zones, or system time changes made by the administrator? — No. QPC is completely independent of the system time and UTC.**" **It resets at boot** ("The frequency of the performance counter is fixed at system boot"), and over a week its crystal tolerance bounds the reading at roughly ±6 seconds at ±10 ppm, or about ±30 seconds at ±50 ppm.
- **Darwin's monotonic clocks** are structurally unsettable, and this is **documented, not merely observed**: `clock_gettime(3)` distinguishes "[EINVAL] `clock_id` specifies a clock that isn't settable" from "[EPERM] A user other than the superuser attempted to set the time", and the observation confirmed it — `clock_settime(CLOCK_MONOTONIC_RAW)` failed with `errno=22 EINVAL` at uid 501, as did `CLOCK_UPTIME_RAW` and `CLOCK_MONOTONIC`. But **Apple's documentation is silent on what setting the system time does to `CLOCK_MONOTONIC` and `CLOCK_UPTIME_RAW`, and silent on whether any of them survives a reboot.** Silence, not denial.
- **Linux's `CLOCK_MONOTONIC`** is documented as "not affected by discontinuous jumps in the system time (e.g., if the system administrator manually changes the clock), but is affected by frequency adjustments", and `CLOCK_BOOTTIME` as identical but suspend-inclusive. The one documented bound on rewinding — `clock_settime` failing with `EINVAL` when asked to set below `CLOCK_MONOTONIC` — holds **within a single boot only; the page says nothing across reboots.**

**Two naming traps that will bite a builder, both documented.** Darwin's `CLOCK_MONOTONIC_RAW` is the **sleep-inclusive** clock and `CLOCK_UPTIME_RAW` the sleep-exclusive one; Linux's `CLOCK_BOOTTIME` is the suspend-inclusive one and its `CLOCK_MONOTONIC_RAW` is suspend-exclusive. **The convention is reversed between the two systems.** And `CLOCK_BOOTTIME` is not a Darwin clock at all — it was absent from the observed machine's list. The size of the mistake is not theoretical: on the machine observed, over 34 days, the two Darwin clocks differed by **10.87 days**, about a third of the elapsed period. (That figure is one laptop's sleep habit over one stretch, measured twice — not a property of macOS.)

**And the hardware floor, on both systems that have one.** Apple's Secure Enclave chapter **documents no time source at all** — zero occurrences of "monotonic" or "timestamp"; the counters it documents are passcode-attempt counters. Whether some other Apple document describes such a counter is **not settled**, so no wider negative is asserted. Microsoft acknowledges a **TPM internal clock** in exactly one certification-test sentence — "This manual test validates Trusted Platform Module (TPM) internal clock consistency" — and, on the two TPM pages read, documents nothing about what it is. **Answered 2026-08-21 from TCG's own reference implementation and the tool manuals, and it changes the batch's headline negative.** *(TCG's own site refused: `https://trustedcomputinggroup.org/resource/tpm-library-specification/` and the PDFs under it returned HTTP 403 on a plain fetch and again with a browser user-agent — recorded in Limits §5 so nobody re-attempts it.)* **What Clock is.** `tpm2_readclock(1)` — *"Reads the current TPMS_TIME_INFO structure from the TPM"* — describes `clock` as *"64 bit value of time TPM has been powered on since last TPM2_Clear in ms"* and `safe` as *"boolean yes|no value that no value of Clock greater than the current value of Clock has been previously reported by the TPM"* (`https://raw.githubusercontent.com/tpm2-software/tpm2-tools/master/man/tpm2_readclock.1.md`, read 2026-08-20). **It can only be advanced**: `tpm2_setclock(1)` is *"Sets the clock on the TPM to a time in the **future**"*, and the refusal is in the command itself rather than in a habit — TCG's reference implementation, as hosted by Microsoft, `TPM2_ClockSet` returns `TPM_RCS_VALUE` where *"in->newTime … < go.clock"* (`https://raw.githubusercontent.com/microsoft/ms-tpm-20-ref/main/TPMCmd/tpm/src/command/ClockTimer/ClockSet.c`, read 2026-08-20). **And there is a signed, remotely verifiable form**: `tpm2_gettime(1)` *"Provides a signed copy of the current time and clock from the TPM"*. **The qualification, and it is the important half.** Clock is written to non-volatile storage every so often rather than on every tick, and after a power loss that was not an orderly shutdown it can come back **lower** than last reported, by up to that interval — the TPM says so by clearing `safe`: *"If the previous cycle is orderly shut down, the value of the safe bit the same as previously saved. Otherwise, it is not safe."* (`.../ms-tpm-20-ref/.../subsystem/Time.c`, read 2026-08-20). **So *never moves backwards* holds only where `safe` is set, and any use of this as an elapsed-time floor must read `safe` and treat a cleared one as unusable.** **It also counts powered-on time rather than wall time**, so it will not count weeks the machine spent switched off. **Still open, and it is a Windows machine and nothing else:** whether an ordinary Windows program can reach it. Microsoft documents `Tbsip_Submit_Command` — *"Submits a Trusted Platform Module (TPM) command to TPM Base Services (TBS) for processing"*, requirements *"Windows Vista [desktop apps only]"* — **with no privilege statement anywhere on the page** (read 2026-08-20). Silence in both directions. *Settles it:* running `TPM2_ReadClock` through TBS on a Windows machine, first as a standard user and then elevated.
