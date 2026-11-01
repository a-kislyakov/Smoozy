# Sources — Security and licence (Architecture)

*Where every outside-world claim in [Security and licence](../Security%20and%20licence.md) comes from. The document carries each claim's label in place; this file carries the citation, the words as their owner states them, the date, and what the source does not cover. Decisions Smoozy made get no entry here.*

Rule 1's plain-English rule is suspended here, per the Architecture stage's exception.

**Three shapes only.** A **pointer** names the input document of this stage that owns the citation. A **direct publication** carries the exact words, the address and the date read, taken from this document or from elsewhere in `Preparation/`. An **assumption or unresolved** says plainly that no source was found and what breaks.

**Five direct publications and no more.** Four of them are quoted by this document and by nothing that precedes it — Apple's shipped `SecItem.h`, Microsoft's `CredEnumerate` reference, RFC 6151, and the PEM pair RFC 1421 / RFC 7468. The fifth, Telegram's token warning, is quoted here from a Specs-stage companion. Everything else in the document is a pointer or an unresolved.

**Four addresses were supplied on 2026-08-23 and the entries say so in place.** This file previously recorded that **no address for RFC 6151, RFC 7468, RFC 4253 or RFC 4252 existed anywhere in `Preparation/`** — document number, section and date read were the whole citation. Each of the four was read at the RFC Editor's own published text on **2026-08-23** and the address recorded. **The words already carried were checked against the publication and every one of them matches**; nothing in the document or in this file was corrected for a mismatch. A pointer entry stays a pointer where the input document of this stage still owns the reading — the supplied address is added to it and does not reclassify it.

---

## §1.2 — the caller's network address and location at the licence service

**Pointer.** [The host itself — the licence platform run rather than read](../The%20host%20itself%20—%20the%20licence%20platform%20run%20rather%20than%20read.md), the platform run rather than read on **2026-08-17**, routed through `Licence service` §14 as this document routes it. That document's Question 4 records: **"Bodies are not captured. The complete header set is, along with the caller's IP address, city, postcode, latitude, longitude, network operator and TLS fingerprint."** It records `"cf-connecting-ip"` on the observed request header set, and records that with the stored record turned off *"what remains of a console line is stripped of headers, IP and geolocation, keeping only the method, URL and path."*

**Does not cover:** the live request stream as against the stored record — the observation is of what a console line retains, not of what a token holder attached to the service sees in real time. `Licence service` §15 item 12 holds that as not established, and §1.2's *"who checked, and from where, is visible in real time"* sentence rests on that item rather than on an observation. It also covers one platform on one date; no other host was run.

## §1.2 — the seller's unauthenticated response carrying the customer's name and email

**Pointer.** [The seller itself — the store observed rather than read](../The%20seller%20itself%20—%20the%20store%20observed%20rather%20than%20read.md), Fact 89, the Creator's own Lemon Squeezy store observed on **2026-08-17** with a test-mode API key. Its words: **"The response also carries the customer's name and email address on an unauthenticated call"**, and, on the item it answers, *"it is the **unauthenticated** response that carries them, so anyone holding a licence key can read the buyer's name and address without a credential."* The observed body carries `customer_name` and `customer_email` inside `meta`.

**Does not cover:** the refund event's body, which §1.2 also names as carrying the customer's name and email. The seller document states outright that **"the shape of a refunded key's validate response is unobserved and remains so"** and that a refund was never posed at that endpoint; the refund event's own routes are [The refund event](../The%20refund%20event%20—%20the%20routes%20to%20it,%20read%20rather%20than%20tried.md)'s, read rather than tried. It also covers one store, one test-mode key, one date.

## §1.3 — a Vast container's mapped port is refused as a webhook address

**Pointer.** [One box — the Telegram capture and the port that cannot be chosen](../One%20box%20—%20the%20Telegram%20capture%20and%20the%20port%20that%20cannot%20be%20chosen.md), Fact 63, **2026-08-16**. A container rental asked for all four ports Telegram accepts was given `45104`, `45181`, `45149` and `45191`; the mapped port answered a self-signed HTTPS request from outside; Telegram refused the registration with **`Bad Request: bad webhook: Webhook can be set up only on ports 80, 88, 443 or 8443`**.

**Does not cover:** the machine. Telegram's page constrains the URL, which is the narrowing this document's §1.3 records as applied on 2026-08-21. **A Vast virtual-machine rental was not tried**, and a relay in front of such a container is not settled. Fact 63's own entry says so in those terms.

## §1.3 — a webhook that works on a second provider, with nothing bought

**Pointer.** [Six machines — the container, the webhook and the ready action](../Six%20machines%20—%20the%20container,%20the%20webhook%20and%20the%20ready%20action.md), Fact 63. Inbound was closed by default by an OpenStack security group the machine itself could not change; opening it there produced a working webhook with no domain name, no certificate authority and nothing bought.

**Does not cover:** any other provider's inbound arrangement, and it is what makes §1.3's boundary **provider-dependent** rather than a capability Smoozy may rely on.

## §1.3 — registration proves nothing about reachability, and one `409 Conflict` covers two causes

**Pointer.** [Six machines](../Six%20machines%20—%20the%20container,%20the%20webhook%20and%20the%20ready%20action.md), Fact 63, for the registration half: Telegram accepted a registration for a completely unreachable address, and the healthy and unreachable answers are byte-identical except for an error field that appeared within sixteen seconds. [One box](../One%20box%20—%20the%20Telegram%20capture%20and%20the%20port%20that%20cannot%20be%20chosen.md), section *Two holders do not race*, for the `409 Conflict` half — one status covering both *a poll while a webhook is registered* and *two pollers competing*, separable only by description text.

**Does not cover:** the description text itself as a contract. Telegram publishes no list of it, and nothing in the project treats the text as stable.

## §1.3 and §14 item 12 — the webhook POST wrapper, the secret-token header, the retry cadence

**Unresolved, and the register says so.** Fact 63's entry: **"Still not covered, and it now needs a different provider: the webhook POST wrapper, the `X-Telegram-Bot-Api-Secret-Token` header, and the retry cadence against a failing endpoint."** No page carrying them was read on this project, and no rental posed them.

**Breaks:** only the authentication of the one exceptional inbound endpoint, and only where a provider permits one at all, which is what §14 item 12 already states. `Telegram` inherits it.

## §2 — the Windows per-credential ceiling, and the two session-scoped facilities

**Pointer.** [The manuals — keys, secrets, and what cannot be told apart](../The%20manuals%20—%20keys,%20secrets,%20and%20what%20cannot%20be%20told%20apart.md), Fact 38, answered **2026-08-16**. It holds the `CREDENTIAL` structure's `CRED_MAX_CREDENTIAL_BLOB_SIZE` as `5*512`, which is the 2560 bytes §2 and §3.2 both name; and the two session facilities under its *Windows — session-only* and *Linux — session-only* headings, both documented against the **logon session** rather than the application.

**Does not cover:** whether Windows' session persistence touches disk at all — Fact 38 carries that as unresolved — and macOS, where no such facility is documented at all, which is an absence of documentation rather than a documented absence. Every Windows claim here is **documentation alone**: no Windows machine has run it.

## §3.0 rule 1 — lookup attributes are not secret material

**Pointer.** [The manuals](../The%20manuals%20—%20keys,%20secrets,%20and%20what%20cannot%20be%20told%20apart.md), Fact 38. The Secret Service specification's own words as that document quotes them: **"The lookup attributes are not treated as secret material, and the service may choose not to encrypt attributes when storing them to disk"**, with libsecret's own page repeating that implementations *"will probably store attributes in an unencrypted manner"*.

**Does not cover:** macOS and Windows. The reason for the opaque subject is Linux-specific and documented; on the other two the same rule is carried for consistency, not because a page says it.

## §3.0 rule 3 — every store call is made off the drawing thread

**Pointer.** [The manuals](../The%20manuals%20—%20keys,%20secrets,%20and%20what%20cannot%20be%20told%20apart.md), Fact 38. Documented, from Apple's own page: **"`SecItemAdd` blocks the calling thread, so it can cause your app's UI to hang if called from the main thread"**, the identical paragraph appearing on `SecItemCopyMatching`, `SecItemUpdate` and `SecItemDelete`. Observed on the Creator's Mac: a `SecItemCopyMatching` with user interaction left allowed **did not return within 45 seconds and was killed**. Linux's half is the specification's **"A service may lock previously unlocked items for any reason at any time."**

**Does not cover:** what the blocked call was waiting on, or whether it would ever have returned. The source itself calls that *"the claim that decides how the file-based path is described, and it is currently carried by an inference"* — §14 item 14. Rule 3 rests on the documentation alone and not on the blocked run.

## §3.0 — the Linux enumerate result arrives already split

**Pointer.** [The manuals](../The%20manuals%20—%20keys,%20secrets,%20and%20what%20cannot%20be%20told%20apart.md), Fact 38: **"The matched items will be returned in two sets. The `unlocked` return value will contain the object paths of all the items that are not locked. The `locked` return value will contain object paths of items that are locked, which can be unlocked if desired"**.

**Does not cover:** any implementation's behaviour. It is the draft specification's wording, and §3.3 already records that what a Linux store does is whatever the installed implementation does.

## §3.1 — the file-based keychain, its access lists, and its loss case

**Pointer.** [The manuals](../The%20manuals%20—%20keys,%20secrets,%20and%20what%20cannot%20be%20told%20apart.md), Fact 38. It owns every quoted sentence in §3.1 except the enumerate paragraph below: Apple's TN3137 — **"macOS has two keychain implementations: File-based keychain [and] Data protection keychain"** and *"The `SecItem` API can target either implementation. It defaults to targeting the file-based keychain."*; the data protection keychain's entitlement requirement, *"must be authorized by a provisioning profile"* and *"is only available in a user login context"*; the access-list sentence **"Otherwise, the system prompts the user for confirmation. The user may choose to Deny, Allow, or Always Allow the access."**; the post-update reauthorisation page, *"If an app is changed or infected by a virus after being granted access to your keychain, the app is no longer trusted"* and *"You must sometimes reauthorize updated apps"*; the reset case, **"If the login password was reset because the user forgot the old password, the keys, passwords, and other information in the keychain won't be accessible, and the user will need to create a new keychain."**; the synchronisation sentence *"Updating or deleting items using the `kSecAttrSynchronizable` key affects all copies of the item, not just the one on your local device."*; and the status symbols `errSecItemNotFound`, `errSecUserCanceled`, `errSecInteractionRequired`, `errSecNoSuchKeychain`, `errSecInvalidKeychain`, from Apple's one-line descriptions. Observed on the Creator's Mac, in the same fact: the Security framework loaded and all four entry points resolved with nothing installed; the binary that created an item read it back silently; a **different** binary reading the same item with user interaction disallowed was refused with `errSecAuthFailed`; and `SecItemDelete` followed by a query returning `errSecItemNotFound` end to end.

**Does not cover, and Fact 38 numbers each gap:** that a **Deny** click returns `errSecUserCanceled` specifically is stated by no page read (unresolved 4); whether a Developer ID application shipped without an embedded provisioning profile can reach the data protection keychain (unresolved 3); and two runs on one machine disagreeing about whether a write from a process without the entitlement succeeds or returns `errSecMissingEntitlement` (unresolved 6). That the refusal of the second binary was **ACL membership** rather than some other cause is inferred, since the item's ACL was never read. Which updates trigger the post-update dialog is stated by nothing — the page says *sometimes*. One Mac, one date.

## §3.1 — the numerals behind the five status symbols

**Direct publication, and this document owns the reading. Entry added 2026-08-23**, replacing this file's earlier line that *the numeric values of these symbols appear on no Apple page at all*. That line was a stale reading of Fact 38's unresolved 5, **closed 2026-08-21** in [The manuals](../The%20manuals%20—%20keys,%20secrets,%20and%20what%20cannot%20be%20told%20apart.md) on the same header.

**Source:** Apple's own shipped SDK header `SecBase.h`, on the Creator's Mac at `/Library/Developer/CommandLineTools/SDKs/MacOSX26.5.sdk/System/Library/Frameworks/Security.framework/Versions/A/Headers/SecBase.h`, **dated 25 April 2026 on disk** (65,186 bytes, owner `root:wheel`), read there **2026-08-23**. The same header ships at `/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.5.sdk/…`. Exact words for the five symbols §3.1 names, quoted from the `CF_ENUM(OSStatus)` block with the block's column spacing collapsed:

- **`errSecUserCanceled = -128, /* User canceled the operation. */`**
- **`errSecNoSuchKeychain = -25294, /* The specified keychain could not be found. */`**
- **`errSecInvalidKeychain = -25295, /* The specified keychain is not a valid keychain file. */`**
- **`errSecItemNotFound = -25300, /* The specified item could not be found in the keychain. */`**
- **`errSecInteractionRequired = -25315, /* User interaction is required, but is currently not allowed. */`**

**Confirmed a second way** on the same machine and the same day by Apple's own lookup tool: `/usr/bin/security error -128` → *"User canceled the operation."*, `-25294` → *"The specified keychain could not be found."*, `-25295` → *"The specified keychain is not a valid keychain file."*, `-25300` → *"The specified item could not be found in the keychain."*, `-25315` → *"User interaction is required, but is currently not allowed."* Control: `-999999` → *"unknown error -999999=fffffffffff0bdc1"*.

**One thing the publication carries that the input document's own paragraph does not:** [The manuals](../The%20manuals%20—%20keys,%20secrets,%20and%20what%20cannot%20be%20told%20apart.md)'s Fact 38 paragraph lists nine numerals and names neither `errSecUserCanceled` nor `errSecInteractionRequired`, which are two of the five §3.1 uses. Both are in the header, read above.

**Does not cover:** which status a **Deny** click produces, which is Fact 38's unresolved 4 and is a question about behaviour rather than about the mapping. A header is a declaration and not a behavioural contract, and **§3.1 maps by symbol and never by number regardless** — the numerals are recorded so that no reader takes the mapping for unpublished, not because any code path uses them. One SDK on one Mac.

## §3.1 — enumerate on macOS: `SecItemCopyMatching` with the match limit set to all

**Direct publication, and this document owns it.** Fact 38 struck the macOS enumerate candidate on the ground that no page naming it had been read; the header was then read on **2026-08-20**.

**Source:** Apple's own `SecItem.h`, as shipped on the Creator's Mac at `/Library/Developer/CommandLineTools/SDKs/MacOSX26.5.sdk/…/Security.framework/Versions/A/Headers/SecItem.h`, **dated 25 April 2026 on disk**, read **2026-08-20**. Exact words: **"By default, this function returns only the first match found. To obtain more than one matching item at a time, specify kSecMatchLimit with a value greater than 1. The result will be a CFArrayRef containing up to that number of matching items."** and **"kSecMatchLimitAll Specifies that an unlimited number of results may be returned; used as a value for the kSecMatchLimit dictionary key."**

**Observed** the same day, on a throwaway keychain file in a scratch area and never the Creator's login keychain: three items stored under one service name, the all-limit search returned **3** at status 0, the one-limit search returned a single dictionary, and a never-stored service returned `-25300` (`errSecItemNotFound`) as the control. A negative would have been the all-limit search returning one item, or the control returning a match.

**Does not cover:** any per-application boundary. **This enumerates by attribute, not by application** — the boundary is the service-name prefix Smoozy adopts, which is a naming convention and not an operating-system guarantee. One SDK header on one Mac; the shipping framework's behaviour on other macOS versions is untested, and the header is a declaration rather than a behavioural contract.

## §3.2 — Credential Manager: the store, the guarantee, the failure states, DPAPI and persistence

**Pointer.** [The manuals](../The%20manuals%20—%20keys,%20secrets,%20and%20what%20cannot%20be%20told%20apart.md), Fact 38, which owns every quoted sentence in §3.2 except the `CredEnumerate` paragraph below. `CRED_TYPE_GENERIC` through `CredWriteW`, `CredReadW` and `CredDeleteW` in `Advapi32.dll`, the DLL taken from the Requirements table on each page cited; the whole documented guarantee, **"The credential will be stored securely but has no other significant characteristics."**; what *stored securely* means, from the separate archived page dated **2016-08-31** — **"These credentials are stored on the hard disk drive and protected by using the Data Protection Application Programming Interface (DPAPI). Any program running as that user will be able to access credentials in this store."**; `ERROR_NOT_FOUND` and `ERROR_NO_SUCH_LOGON_SESSION`; DPAPI's **"merely applies cryptographic protection to the data. It does not store any of the protected data"**, its same-user reach, and *"the specific error code returned when tampering is detected may vary … Applications should not rely on a specific error code to detect data tampering"*; the three persistence values, `CRED_PERSIST_LOCAL_MACHINE` being *"visible to other logon sessions of this same user on this same computer and not visible to logon sessions for this user on other computers"*, `CRED_PERSIST_ENTERPRISE` visible *"to logon sessions for this user on other computers"*, and `CRED_PERSIST_SESSION` which *"will not exist after this user logs off and back on"*; and the naming guidance, **"Microsoft suggests the name be prefixed by the name of the company implementing the service"**.

**Does not cover:** any observation whatever. **No Windows machine has run it, and none is rented today** — Row 282's Windows limit, and §14 item 8. Whether the archived 2016 sentence still holds for current Windows is Fact 38's unresolved 11. That a following `CredRead` returns `ERROR_NOT_FOUND` after a `CredDelete` is **inference, not citation**, which is why §3.2 does not require the read-back it requires on macOS. And nothing read carries `Advapi32.dll`'s presence on a stock installation the way one page does for `Crypt32.dll`.

## §3.2 — enumerate on Windows: `CredEnumerate` with a name-prefix filter

**Direct publication, and this document owns it.** Fact 38 and its companion record **"No page for `CredEnumerate` or `CredFree` was read; neither is cited"** — a procedural strike, not a page read and found wanting. The page was read on **2026-08-20**.

**Source:** Microsoft's own reference at `https://learn.microsoft.com/en-us/windows/win32/api/wincred/nf-wincred-credenumeratew`, page metadata `updated_at: 2024-11-20`, read **2026-08-20**. Exact words: **"The CredEnumerate function enumerates the credentials from the user's credential set. The credential set used is the one associated with the logon session of the current token."** On the filter: **"Pointer to a null-terminated string that contains the filter for the returned credentials. Only credentials with a TargetName matching the filter will be returned. The filter specifies a name prefix followed by an asterisk. For instance, the filter \"FRED\\*\" will return all credentials with a TargetName beginning with the string \"FRED\"."** Its Requirements table gives Library `Advapi32.lib`, DLL `Advapi32.dll`.

**Does not cover:** anything observed — documentation alone, like the rest of §3.2. And it carries the same narrowing as the macOS entry point, doubly so here: the filter is a **name prefix and not a per-application boundary**, and §3.2 already records that Windows has no per-application boundary at all.

## §3.2 — what a Windows password change or reset does to the items

**Pointer.** [The manuals](../The%20manuals%20—%20keys,%20secrets,%20and%20what%20cannot%20be%20told%20apart.md), Fact 38: DPAPI master keys are re-encrypted on a password **change**, with domain and Password Reset Disk recovery paths for a reset.

**Does not cover, and it is Fact 38's unresolved 10:** an administrative reset on a standalone machine with no such disk, and the whole picture for Microsoft-account and Windows Hello logins. §3.2 needs no answer — every outcome presents as *not there* or *store unavailable*, both of which the contract handles.

## §3.3 — the Secret Service, what its specification declines to mandate, and the implementations

**Pointer.** [The manuals](../The%20manuals%20—%20keys,%20secrets,%20and%20what%20cannot%20be%20told%20apart.md), Fact 38, which owns every quoted sentence in §3.3. The API is a **draft** and states **"This specification does not mandate the use of master passwords to lock a collection of secrets"** and **"This specification does not mandate any form of access control."**; its "Notes for Service Implementors" chapter reads, in its entirety, **"[TODO: complete]"**; GNOME's daemon **"is normally started automatically when a user logs into a desktop session"**; KDE's `ksecretd` sits behind the same interface and KeePassXC can replace both exclusively; the locking instruction, **"A client application should always be ready to unlock the items for the secrets it needs … It must not assume that an item is already unlocked for whatever reason. A service may lock previously unlocked items for any reason at any time."**, with `SECRET_ERROR_IS_LOCKED` and the `locked` set; the prompt rule, that the `Completed` signal's `dismissed` argument is the only way **"Either the user or the client application can dismiss a prompt"** is distinguishable; `secret_password_clear` reporting whether anything was removed and deleting **unlocked** items only; and the transport warning that the encryption **"is not envisioned to withstand man in the middle attacks"** with **"The `plain` algorithm is almost always supported."**

**Does not cover:** anything observed. **No Linux computer exists on this project** — §14 item 9, which puts every Linux claim in this section under one label. And the session collection is documented as memory-only **by GNOME alone**; `ksecretd` and KeePassXC are Fact 38's unresolved 16, which is why §2's items 3b, 3c and 4b use process memory instead.

## §3.3 — the load-bearing Linux gap: no provider running

**Unresolved and narrowed, and the input document names both halves. Corrected 2026-08-23**, this entry having carried Fact 38's unresolved 13 and 14 as open after both moved on 2026-08-21.

**Fact 38's unresolved 13 — NARROWED 2026-08-21**, in [The manuals](../The%20manuals%20—%20keys,%20secrets,%20and%20what%20cannot%20be%20told%20apart.md)'s own words: **"Neither the specification's three errors nor libsecret's nine-member enum names it, and that stands — but D-Bus and GIO both do: `ServiceUnknown` and `NameHasNoOwner`, quoted with their addresses in the store-unavailable entry. What is left is which of the two arrives in a given situation, and what arrives with no session bus at all."**

**Fact 38's unresolved 14 — CLOSED 2026-08-21 on the vendor's own manifest**, same source: **"Ubuntu's published server-image package list carries no provider and not even the client library. The question of what runs on a machine with no desktop session is answered for the image this project rents; a machine somebody has added a provider to is not this question."**

**Breaks:** the accuracy of the *nowhere to remember* notice `Whole app` promises. §3.3's probe establishes that a round trip failed and **not which of the two it was**, and the document says outright it does not claim the probe separates them; §14 item 7 records the cost. What holds without it is §3.3's conservative rule — only a probe that has never succeeded in any run drives the notice.

## §3.3 — the store's standing, the once-only notice, and the third password choice

**No entry — these are Smoozy's own decisions.** The durable *store's standing* value with its three readings and one writer, its lifetime class, the moment the notice is shown, and composing the machine-password offer from the standing are Architecture's rulings against Row 316 and `Whole app`, not facts about any system. The only outside claim under them is the probe's own limit, which is the entry above.

## §4 — the askpass route, and the passphrase flags that are closed to it

**Pointer.** [The manuals](../The%20manuals%20—%20keys,%20secrets,%20and%20what%20cannot%20be%20told%20apart.md), Fact 35. `ssh(1)`: **"If ssh does not have a terminal associated with it but DISPLAY and SSH_ASKPASS are set, it will execute the program specified by SSH_ASKPASS"**, and `SSH_ASKPASS_REQUIRE`: **"if the variable is set to \"force\", then the askpass program will be used for all passphrase input regardless of whether DISPLAY is set"**. That `ssh-keygen`'s `-P` puts a passphrase on a command line is the same document's reading of `ssh-keygen(1)`, `-P passphrase` — *"Provides the (old) passphrase."*

**Does not cover:** `ssh-keygen`'s own honouring of the askpass route at key generation, which is a separate fact and is `Keys`' — its citation is [Keys](Keys.sources.md)' companion, observed on one build. Nothing read documents what the client does when it needs a passphrase and **every** input route is closed; §4 says outright that nothing rests on it.

## §4 — `BatchMode=yes` and the private-key passphrase prompt

**Pointer.** [The manuals](../The%20manuals%20—%20keys,%20secrets,%20and%20what%20cannot%20be%20told%20apart.md), Fact 35 Half B, run **2026-08-20** against a real server with a terminal deliberately attached and a control either side: the prompt appears twice without batch mode and does not appear at all with it, the server having accepted the key. The same document holds the documentation gap that remains: `BatchMode`'s own wording names only **"password prompts and host key confirmation requests"**, and *"The manual therefore does not settle whether a `BatchMode=yes` connection using an encrypted key fails outright or prompts."*

**Does not cover:** the manual, which is still silent — the **documentation** gap is real and only the **behaviour** gap closed. One build, one server, one date. §4 keeps the construction as the second line of defence and not the first.

## §4 and §13 rule 1 — another login's process is visible with its command line

**Pointer.** [Six machines](../Six%20machines%20—%20the%20container,%20the%20webhook%20and%20the%20ready%20action.md), Fact 60. From a second ordinary login, another login's process **"is visible with its owner and command line"**.

**Does not cover:** the user's own computer. No comparable fact is established there, and §4 says none is needed — the rule is the same in both places, so nothing waits on it.

## §4 — retry behaviour is the tool's, and it is unbounded on one route

**Pointer.** [The manuals](../The%20manuals%20—%20keys,%20secrets,%20and%20what%20cannot%20be%20told%20apart.md), Fact 37. An `ssh-add` fed a wrong passphrase by an askpass helper made **59 askpass invocations in 12 seconds and was still running when killed**, with no diagnostic; `ssh-add(1)` documents no retry limit. `ssh` itself made three and stopped, with the source's own narrowing: **"the keyword was never varied, so the two threes are not established as connected"**.

**Does not cover:** any other tool, and the connection between the two threes. One build.

## §4 and §13 rule 5 — what a local load separates and a connection does not

**Pointer.** [The manuals](../The%20manuals%20—%20keys,%20secrets,%20and%20what%20cannot%20be%20told%20apart.md), Fact 37. Locally, a wrong passphrase gives `incorrect passphrase supplied to decrypt private key` while malformed, truncated, empty and public-as-private all give `invalid format`. Over a connection at default log level, **wrong passphrase, empty passphrase, no passphrase available, key not authorised and wrong account name all collapse into `Permission denied (publickey).` and exit 255**. The same fact establishes that **no OpenSSH manual documents any of the diagnostic strings the tools emit**.

**Does not cover:** the message strings as a contract — that is the point of the fact. Raising to `-vv` does separate them, but that is observed on one build and those levels print key paths and identity material, which is why §13 rule 5 forbids it on an ordinary path.

## §4 — even the local signal is not proof

**Pointer.** [The manuals](../The%20manuals%20—%20keys,%20secrets,%20and%20what%20cannot%20be%20told%20apart.md), Fact 37. The **correct** passphrase against a one-bit-corrupted key produced the identical `incorrect passphrase` message, and a flip at one interior offset produced exit 0 with correct-looking output.

**Does not cover:** how often either case arises. §4 accepts the cost as one re-entry.

## §4 — the rate limit that corrupts the diagnosis

**Pointer.** [The manuals](../The%20manuals%20—%20keys,%20secrets,%20and%20what%20cannot%20be%20told%20apart.md), Fact 37. Repeated attempts against a real server tripped the default source penalty, after which attempts **with the correct passphrase** failed as a closed connection, client-side indistinguishable from a broken server.

**Does not cover:** the threshold, the penalty's duration, or any server whose configuration differs from the one observed.

## §4 and §14 item 15 — the separable authentication result from an elevation step

**Pointer, and the separation itself is answered.** [Facts](../Facts.md), **Fact 108**, numbered 2026-08-19 out of `Server Setup` and quoting §14 item 15 as the reason. **Answered on the caller's end 2026-08-19** from `sudo(8)` at **Sudo 1.9.17p2**, manual dated June 7 2025, read on the Creator's Mac — the citation is [Server Setup's companion](Server%20Setup.sources.md) and the validate form *"does not run a command"*. **Run on a rental 2026-08-22** — the multi-provider batch's machine A, `sudo` **1.9.15p5** with Ubuntu 24.04's stock sudoers, under a purpose-made non-root login because the delivered login is already root: `sudo -S -v` with the right password prompted and **exited 0 with no command run**; `sudo -n -v` gave `sudo: a password is required` at exit 1; the credential cache is **15 minutes** on that image. Evidence in [The multi-provider batch](../The%20multi-provider%20batch%20—%20the%20surface%20that%20did%20not%20vary,%20the%20Debian%20route%20that%20exists,%20and%20the%20cards%20a%20slice%20cannot%20see.md).

**What it covers on the second question, and this half changed on 2026-08-29.** The three refusals sit at exit 1 and differ only in their words — `sudo: 1 incorrect password attempt`, `sudo: a password is required`, and `Sorry, user … may not run sudo on <host>.` **They are observed, not documented**, on that one release. **The Creator ruled on 2026-08-25** that §13 rule 5 carries a named exception for this tool, so the wrong-password ending **is** matched as an exact constant and the failed validate carrying it **is** an authentication result. The third message **names the host** and is still never matched as a constant.

**Does not cover:** any release or machine kind other than the one observed — one rental, `sudo` 1.9.15p5, Ubuntu 24.04, 2026-08-22 — and **no manual page read names any of the three strings**. The virtual-machine kind under a non-root login is unposed: both virtual machines in that batch delivered passwordless elevation, which cannot pose a password refusal.

**Breaks:** on an unobserved release or kind no string matches, nothing is concluded, and the erase does not fire — so `Whole app`'s rule that a password that stops working is forgotten rather than retried is kept where the strings are observed and falls back to *keep and report* elsewhere. §4's safe direction holds throughout: an unmatched refusal is reported as a failed operation and forgets nothing.

**This entry read *Unresolved. No route was run and no manual read on this project establishes* the separation until 2026-08-23, and carried the Fact 108 correction as recorded-rather-than-fixed.** Both were repaired that day: the route is `Server Setup` E4's, it was run, and §4 and §14 item 15 say so in place. **Its *does not cover* paragraph then said outright that a failed validate does not separate the two causes, and that §13 rule 5 forbids mining the output — both written before the 2026-08-25 ruling and neither brought forward until 2026-08-29.**

## §13 rule 5 — the elevation tool's refusal strings, which the exception rests on

**Pointer, and the label is *observed* rather than *documented*. Entry added 2026-08-29; §13 rule 5's exception had none, and the claim it carried was labelled documented.** [Facts](../Facts.md), **Fact 108**, run on a rental **2026-08-22**: the multi-provider batch's machine A, `sudo` **1.9.15p5** with Ubuntu 24.04's stock sudoers (`Defaults env_reset, mail_badpass, secure_path, use_pty`; `%sudo ALL=(ALL:ALL) ALL`), under a purpose-made non-root login. Its words, all three at exit 1: wrong password — `Sorry, try again.` … `sudo: no password was provided` … **`sudo: 1 incorrect password attempt`**; no password offered — **`sudo: a password is required`**; not an administrator — **`Sorry, user smoozynoadmin may not run sudo on a0e7a496ff47.`** Evidence in [The multi-provider batch](../The%20multi-provider%20batch%20—%20the%20surface%20that%20did%20not%20vary,%20the%20Debian%20route%20that%20exists,%20and%20the%20cards%20a%20slice%20cannot%20see.md).

**Does not cover, and this is why the rule's own wording was corrected:** **no publication.** No `sudo` manual page read on this project names one of these strings, and the exception's original wording — *documented, in fixed wording, on both versions Ubuntu ships* — is supported by nothing in `Preparation/`: not by a manual, and not for a second Ubuntu release, since one image on one date is the whole of what was run. The **Creator's ruling of 2026-08-25** that the strings may be matched stands; **its ground is this observation.**

**Breaks:** the reach of the exception and nothing under it — §14 item 17. A release whose wording differs matches nothing, so the erase does not fire there and Smoozy claims nothing about the cause.

## §5 — erasure

**No entry.** The three-write order, the recorded debt, the sweep and its trigger, and the per-event erasure list are Smoozy's decisions against Rows 41 and 252. The only outside claims under them are Linux's unlock-before-delete, cited at §3.3 above, and macOS's confirmable delete, cited at §3.1.

## §5.4 — nothing puts a fingerprint or a host key on a screen

**No entry.** This is `Whole app`'s and `Servers`' promise, restated and extended by this document. A spec promise is not an outside fact.

## §6 — the one-place premise is false: where a private key can live

**Pointer.** [The manuals](../The%20manuals%20—%20keys,%20secrets,%20and%20what%20cannot%20be%20told%20apart.md), Fact 33. OpenSSH's manuals call `~/.ssh` the **default** location and never the only one; `IdentityFile` and `-i` can name any path and may appear many times; a key can exist only inside `ssh-agent` with no file behind it; a PKCS#11 token supplies keys with no file at all; and Windows has a second Microsoft-named location, `C:\ProgramData\ssh`, where Microsoft says the **host keys** and server configuration live, that page's `HostKey` defaults naming private key files, none of the four carrying `.pub`.

**Does not cover:** an exhaustive list of what may appear in a key location. The same fact records that because `Include` puts arbitrary filenames there by design, **no closed list exists**. §6's rule is the narrow one that follows: no code path may synthesise a private key path from a name.

## §6 — the fingerprint hash is selectable

**Pointer.** [The manuals](../The%20manuals%20—%20keys,%20secrets,%20and%20what%20cannot%20be%20told%20apart.md), Fact 76. `FingerprintHash` takes **"md5 and sha256 (the default)"** on both the client's and the server's configuration pages.

**Does not cover:** any shipping build's default. The pages are OpenBSD-current and say nothing about other platforms' builds — which is why §6 names the hash at every call rather than relying on the default.

## §6 and §14 item 1 — MD5 collisions are cheap

**Direct publication.** RFC 6151, *Updated Security Considerations for the MD5 Message-Digest and the HMAC-MD5 Algorithms*, March 2011, at **`https://www.rfc-editor.org/rfc/rfc6151.txt`**. First read **2026-08-20**; **address supplied and words re-read at the publication 2026-08-23**. Exact words: **"The attacks presented in [KLIM2006] can find MD5 collision in about one minute on a standard notebook PC (Intel Pentium, 1.6GHz)."** — §2.1, *Collision Resistance* — and **"MD5 is no longer acceptable where collision resistance is required such as digital signatures."** — §2, *Security Considerations*, in the opening paragraph ahead of the subsections.

**Where the citation sits in the project:** [Facts](../Facts.md), Fact 76's collision entry, and `Keys` F8, both carrying the same two sentences and the same date. **This entry recorded that no address for RFC 6151 existed anywhere in `Preparation/`** — document number and date read were the whole citation. It was supplied on 2026-08-23 from the RFC Editor's own published text. **Both sentences match the publication word for word**; the section attributions above are new and were not carried before.

**Does not cover:** SHA-256. **No first-party sentence says a SHA-256 collision does not exist**, because an absence of published collisions is not a citable statement, and this document does not assert it. Whether Row 293's *two keys Smoozy knows can never share one* may close on that basis is the Creator's — [Open Threads](../../Groundwork/Open%20Threads.md) item 8a.

## §6 — what is readable from an encrypted private key without the passphrase

**Pointer.** [The manuals](../The%20manuals%20—%20keys,%20secrets,%20and%20what%20cannot%20be%20told%20apart.md), Fact 35 Half A, documented and observed: type, bit size and fingerprint are readable from an encrypted private key file with no passphrase and stay readable with the `.pub` deleted; the **comment is not recoverable** without it.

**Does not cover:** key formats other than the ones tested, and the legacy PEM case, which is the entry two below. Fact 35's own narrowing records that **only two keys were tested**.

## §6 — the `-P ''` probe and its confounders

**Pointer.** [The manuals](../The%20manuals%20—%20keys,%20secrets,%20and%20what%20cannot%20be%20told%20apart.md), Facts 35 and 37. `ssh-keygen -y -P '' -f <path>` exits 0 on a key with no passphrase and 255 on a key with one; the same command exits 255 for a garbage file, a truncated key, a zero-length file, a public key passed as a private one, a missing file, and a key whose file permissions are too open, separated only by message text **no OpenSSH manual documents**.

**Does not cover:** the message text as a discriminator, which is exactly why §6 takes the yes-or-no from the container instead and uses the probe only to confirm.

## §6 — the `openssh-key-v1` container read

**Pointer.** [The manuals](../The%20manuals%20—%20keys,%20secrets,%20and%20what%20cannot%20be%20told%20apart.md), Fact 35. Strip the armour, base64-decode, read the length-prefixed cipher and KDF names; `none`/`none` is unprotected.

**Does not cover, and §6 carries all three narrowings:** it was observed on **one build, OpenSSH 10.2p1**, on **four keys of which only one was encrypted**, so the discriminating half rests on a single data point; the container layout is **documented on no OpenSSH manual page** — `openssh-key-v1` returns count zero on all six pages searched — and is an observation of the file bytes; and a legacy PEM key is not this container at all, which the next entry covers.

## §6 — the legacy PEM and PKCS8 header answer

**Direct publication.** Read **2026-08-20**; the citation as the project holds it is `Keys` U8's, and it is repeated here because §6 quotes it.

- **RFC 1421 §4.6.1.1 and §4.6.1.3**, at `https://www.rfc-editor.org/rfc/rfc1421.txt`: **"The \"Proc-Type:\" encapsulated header field, required for all PEM messages, identifies the type of processing performed on the transmitted message."** and **"the field is required for all messages specified as \"ENCRYPTED\" in the \"Proc-Type:\" field."**
- **RFC 7468 §10 and §11**, *Textual Encodings of PKIX, PKCS, and CMS Structures*, April 2015, read the same day: unencrypted key structures **"are encoded using the \"PRIVATE KEY\" label"** and encrypted ones **"are encoded using the \"ENCRYPTED PRIVATE KEY\" label"**. **Address supplied 2026-08-23 — this entry recorded that none existed in the project:** **`https://www.rfc-editor.org/rfc/rfc7468.txt`**, read there 2026-08-23. The two fragments are parts of one sentence each, and the full sentences at the publication are **"Unencrypted PKCS #8 Private Key Information Syntax structures (PrivateKeyInfo), renamed to Asymmetric Key Packages (OneAsymmetricKey), are encoded using the \"PRIVATE KEY\" label."** (§10) and **"Encrypted PKCS #8 Private Key Information Syntax structures (EncryptedPrivateKeyInfo), called the same in [RFC5958], are encoded using the \"ENCRYPTED PRIVATE KEY\" label."** (§11). **The fragments match; nothing is corrected.**

**Observed** on the Creator's Mac, four keys made with `-m PEM` and `-m PKCS8` with and without a passphrase: the encrypted PEM key carries `Proc-Type: 4,ENCRYPTED` and `DEK-Info: AES-128-CBC,…` on lines 2 and 3, the plain one carries neither, and the two PKCS8 armour lines differ from each other.

**Does not cover:** fingerprinting a PEM key without its `.pub`, which `Keys` records as refused at exit 255 rather than prompted — a separate result, and `Keys`' to own. One build, one Mac, four keys.

## §6 — the permission gate

**Pointer.** [The manuals](../The%20manuals%20—%20keys,%20secrets,%20and%20what%20cannot%20be%20told%20apart.md), Fact 37: a private key file with mode 0644 is rejected with `bad permissions` before the passphrase question is reached at all.

**Does not cover:** which modes short of 0644 trip it, and any system but the one observed.

## §6 and §7 — a successful fingerprint is not proof of a key

**Pointer.** [The manuals](../The%20manuals%20—%20keys,%20secrets,%20and%20what%20cannot%20be%20told%20apart.md), Fact 34. `ssh-keygen -l -f` correctly rejects `config`, plain text and empty files at exit 255, but **accepts `known_hosts` at exit 0 and fingerprints it**; every exit status and message reproduced on re-run.

**Does not cover:** `authorized_keys`, which is **inferred from the shared line format and never run**, and which no manual states. One build.

## §6 — what the fingerprint is taken over

**Pointer, with one citation the project holds only as words.** [The manuals](../The%20manuals%20—%20keys,%20secrets,%20and%20what%20cannot%20be%20told%20apart.md), Fact 76, observed on **one build, OpenSSH 10.2p1 on one Mac**: the printed form is `SHA256:` plus unpadded base64, and the digest is taken over the decoded wire-format public key blob — which the source itself records as **"determined by reconstruction, not from the manual"** — so it is independent of the comment and of file whitespace and identical from the private file or the public one. The cross-system half rests on **RFC 4253 §6.6**, whose words the project holds in [Facts](../Facts.md), Fact 76: the encoding is *"string certificate or public key format identifier"* followed by *"byte[n] key/certificate data"*. **Address and date supplied 2026-08-23 — this entry recorded that neither existed anywhere in `Preparation/`:** RFC 4253, *The Secure Shell (SSH) Transport Layer Protocol*, January 2006, at **`https://www.rfc-editor.org/rfc/rfc4253.txt`**, §6.6 *Public Key Algorithms*, read there **2026-08-23**. **The two quoted strings match**, and one thing about their form is worth recording so nobody looks for them as prose: they are two lines of the section's own format block — `string    certificate or public key format identifier` and `byte[n]   key/certificate data` — quoted with the block's internal spacing collapsed. The §8 and §10 citations of the same RFC are the entry under §7 below and carry the same address.

**Does not cover:** any documented statement of the digest input — the reconstruction is the source. The hand recomputation reproducing the tool's printed line in both hash forms was done on the same one Mac.

## §7 — `accept-new` is the nearest documented setting and does not deliver the accept path

**Pointer, to a Specs-stage companion rather than to an input document of this stage.** [Preparation/Specs/Sources/Servers.sources.md](../../Specs/Sources/Servers.sources.md), OpenSSH's `ssh(1)` and `ssh_config(5)` at man.openbsd.org, retrieved **2026-07-17**: the `StrictHostKeyChecking` documentation records the default as **asking** the user before a new host identity is accepted, and **`accept-new` as the setting that accepts new identities without asking while still refusing changed ones**. `ssh(1)`'s own sentence is also quoted in [The manuals](../The%20manuals%20—%20keys,%20secrets,%20and%20what%20cannot%20be%20told%20apart.md): *"If a host's identification ever changes, ssh warns about this and disables password authentication…"*

**Does not cover:** the accept-a-change path §7 needs, where a **changed** identity is deliberately replaced after the user says so. That is neither the default nor `accept-new`, and the Specs companion says the citation covers SSH's own behaviour only.

## §7 and §14 item 4 — the offered host key obtained without authenticating

**Pointer.** [The host key — obtained without authenticating, and whose it turns out to be](../The%20host%20key%20—%20obtained%20without%20authenticating,%20and%20whose%20it%20turns%20out%20to%20be.md), Fact 82, answered **2026-08-17**. `ssh-keyscan(1)`, macOS 26.5, page dated **October 4, 2025**, re-read **2026-08-20**: **"For scanning, one does not need login access to the machines that are being scanned, nor does the scanning process involve any encryption."** Observed against the German rental — `ssh-keyscan -p 28452 ssh3.vast.ai` exited **0**, wrote nothing to stderr, returned `ssh-rsa`, `ecdsa-sha2-nistp256` and `ssh-ed25519`, two independent scans byte-identical, and the comparison separated the real key from a generated impostor.

**Does not cover, and §7 keeps both limits:** `ssh-keyscan(1)` has **no EXIT STATUS section**, so its `0` and its `1` are observed and documented nowhere; and where a connection is attempted against a wrong stored record, ssh exits **255**, its generic status, which separates a host-key mismatch from nothing. One provider, one rental.

## §7 and §14 item 4 — the host key is checked inside key exchange, before authentication

**Pointer.** [The host key](../The%20host%20key%20—%20obtained%20without%20authenticating,%20and%20whose%20it%20turns%20out%20to%20be.md), which owns this citation in the same words this document uses. **RFC 4253 §8 step 3, read 2026-08-20:** **"C verifies that K_S really is the host key for S (e.g., using certificates or a local database)."**; §10 makes `ssh-userauth` a service requested **"After the key exchange"**; RFC 4252's abstract puts the authentication protocol **"on top of the SSH transport layer protocol"**. Run with a control on the Creator's Mac, `OpenSSH_10.2p1`, **2026-08-20**: with a deliberately wrong stored key the client exited 255 at *"Host key verification failed."* and the whole trace contains no `userauth`, no `SERVICE_ACCEPT`, no key offered; with the true key in the same scratch file the trace reaches *"debug2: service_accept: ssh-userauth"*.

**Addresses supplied 2026-08-23 — this entry recorded that none was held for either RFC in the project, the document number, the section and the date read being the whole citation.** RFC 4253, *The Secure Shell (SSH) Transport Layer Protocol*, January 2006, at **`https://www.rfc-editor.org/rfc/rfc4253.txt`**; RFC 4252, *The Secure Shell (SSH) Authentication Protocol*, January 2006, at **`https://www.rfc-editor.org/rfc/rfc4252.txt`**. Both read there **2026-08-23**, and **every quoted phrase matches**. §8 step 3 reads **"C verifies that K_S really is the host key for S (e.g., using certificates or a local database)."**; §10, *Service Request*, reads **"After the key exchange, the client requests a service."**, with `ssh-userauth` among the service names it reserves; and RFC 4252's abstract reads **"The SSH authentication protocol runs on top of the SSH transport layer protocol and provides a single authenticated tunnel for the SSH connection protocol."**, of which this document quotes the middle phrase.

**Does not cover:** the client's behaviour, which is the run rather than the RFC. One client build, one Mac.

## §7 and §14 item 5 — the host key at a provider that fronts SSH with its own proxy

**Pointer.** [The host key](../The%20host%20key%20—%20obtained%20without%20authenticating,%20and%20whose%20it%20turns%20out%20to%20be.md), an input document of this stage, and [Facts](../Facts.md), **Fact 83**, **ANSWERED IN FULL, 2026-08-17**. **This entry recorded the document's own status for it as stale until 2026-08-23** — §7 and §14 item 5 carried the question as open and said *"neither document records anything about host keys at all, and neither does any other input document"*, which was false of the very input document that answers it. **Both places were repaired on 2026-08-23** and now carry the answer and what it costs Row 36's reach.

What the input document establishes:

- **The proxy half is answered no.** The three keys offered through `ssh3.vast.ai:28452` are **byte-for-byte the machine's own `/etc/ssh/ssh_host_rsa_key.pub`, `ssh_host_ecdsa_key.pub` and `ssh_host_ed25519_key.pub`** — *"So the proxy forwards the connection rather than terminating it."*
- **The sharper answer is negative for Row 36.** The German box was destroyed and a fresh rental taken on the same physical machine `141036`; it reached a different proxy and port, `ssh9.vast.ai:27488`, and **presented byte-identical host keys** — *"The host key survived the rental it belonged to."* On the virtual-machine kind, [The last two boxes](../The%20last%20two%20boxes%20—%20the%20hook%20that%20cannot%20be%20read%20back,%20the%20full%20disk,%20and%20the%20unit%20that%20kills%20a%20render.md) §17 records four host keys identical across a recycle, shipped in the image and generated 2025-11-06. Test 19 of [The multi-provider batch](../The%20multi-provider%20batch%20—%20the%20surface%20that%20did%20not%20vary,%20the%20Debian%20route%20that%20exists,%20and%20the%20cards%20a%20slice%20cannot%20see.md), 2026-08-22, establishes that **a container's keys are the host's**. The register's conclusion: **"the key is simply not a name for the thing Smoozy needs to recognise — not the rental and not the installation."**
- **The honest limit that remains.** The machine's own files were read over an authenticated session through the same proxy, so a middle lying consistently on both would not show up. And whether a machine's keys ever change without the hardware changing is **not established**.

**Does not cover:** two providers only, and the *on every host* half of the image claim is an inference from one machine watched across a recycle rather than from two rentals on two hosts — marked as such on 2026-08-22.

## §8 — the login the supervisor runs as on a rented machine

**Pointer.** [Six machines](../Six%20machines%20—%20the%20container,%20the%20webhook%20and%20the%20ready%20action.md), Fact 1's narrow clause. On both machines where it was posed, the supervisor runs as an ordinary login rather than as root — **`uid=1000 user=smoozy`** on the container and **uid 1001** on the virtual machine — and on the container that is reached only because root installs the provider hook and the hook drops privilege: **"a non-root login has, on its own, no boot-registration route at all on a container"**.

**Does not cover:** four of the six machines, on which the narrow clause was not posed, and any provider not in that batch.

## §8 — peer credentials as the caller-identity mechanism

**Pointer.** [The host key](../The%20host%20key%20—%20obtained%20without%20authenticating,%20and%20whose%20it%20turns%20out%20to%20be.md) and [Facts](../Facts.md), Fact 84, answered **2026-08-17** on both machine kinds and both halves. `getsockopt(SOL_SOCKET, SO_PEERCRED)` on an `AF_UNIX` stream socket: a server told **`pid=1628 uid=1000 gid=1000`** for a caller running as an ordinary login, `uid 1000` resolving to that login, and the caller's own independent view agreeing exactly — on a container rental whose capability set **`00000000a80405fb`** does not include `CAP_SYS_ADMIN`, and identically on a genuine virtual machine with the full set. The endpoint was deliberately left mode `0666`, so what was observed is credentials rather than file permissions. An unprivileged caller attaching a `uid 0` claim was refused by the kernel with **`errno 1 Operation not permitted`**.

**Does not cover:** the machine's own root. §8 states that limit outright and no arrangement on a rented machine changes it. Two machines, one provider, one date.

## §8 and §11 — root on a rented machine, and what the provider holds

**Pointer.** [The rented machine](../The%20rented%20machine.md), *The edges of the rental*: on a Vast.ai virtual machine the login handed over **was real root**, holding every capability including `cap_sys_module`, `cap_sys_admin` and `cap_sys_boot`, and **"Root ignores file permissions"**. [Six machines](../Six%20machines%20—%20the%20container,%20the%20webhook%20and%20the%20ready%20action.md), Fact 48: on a container, root inside held **13 of 41** capabilities and was not in a user namespace, and even there could not be excluded from the container's own filesystem.

**Does not cover:** any provider that hands over a non-root login on a virtual machine. §8's and §11's claims are written as *on the rentals observed*.

## §9 — why the local record is not encrypted

**Pointer.** [The manuals](../The%20manuals%20—%20keys,%20secrets,%20and%20what%20cannot%20be%20told%20apart.md), Fact 38, for both quoted sentences: Windows' **"Any program running as that user will be able to access credentials in this store"**, and Linux's **"does not mandate any form of access control"**. macOS's per-application boundary is the access-list entry under §3.1.

**Does not cover:** anything about the record itself — that it is written in the per-user application-data location, protected by file permissions and nothing further, is Smoozy's decision and earns no citation.

## §10 — the bot token's stakes, in Telegram's own words

**Direct publication, quoted here from a Specs-stage companion.** [Preparation/Specs/Sources/Telegram.sources.md](../../Specs/Sources/Telegram.sources.md): Telegram's documentation was read at `core.telegram.org/bots/api`, `/bots`, `/bots/features`, `/bots/faq`, `/bots/webhooks` and `/bots/api-changelog`, **checked 2026-08-12**, and its security warning reads **"Everyone who has your token will have full control over your bot."**

**Does not cover:** more than one program holding one token. The same companion records that Telegram's documentation **"never addresses more than one program holding one token — not for sending, not for receiving"**, and that the only exclusivity it states is between `getUpdates` and a webhook.

## §10 — revoking a token, and what a retired machine can tell

**Pointer.** [One box](../One%20box%20—%20the%20Telegram%20capture%20and%20the%20port%20that%20cannot%20be%20chosen.md), Fact 67. Revoking cuts the old holder off immediately — an in-flight poll returned and every call after it was refused with **`{"ok":false,"error_code":401,"description":"Unauthorized"}`**, 58 bytes — and **"So revoking replaces who may act as the bot and nothing else."** On the other side: **"A token that never existed answers byte for byte the same. So a machine still holding a replaced token knows only that it is out — it cannot tell replaced from mistyped from bot deleted … Whatever tells a machine it has been retired has to come from Smoozy, not from Telegram."**

**Does not cover:** any behaviour after the observed window, and any second bot. One token, one date.

## §10 — what a rented disk keeps, and what a recycle destroys

**Pointer.** [The rented machine](../The%20rented%20machine.md): **"The record of a boot from November 2025 was still on the disk in August 2026"**, and a reboot and a destruction both carried out from the provider's own console. [Six machines](../Six%20machines%20—%20the%20container,%20the%20webhook%20and%20the%20ready%20action.md), Fact 6: `/var/tmp` **"is the only temporary area that both survives a reboot and sits on disk, which also makes it an easy place to leave one rental's data for the next tenant"**; and on a recycle, **"Everything Smoozy installed was destroyed, down to the `cron` package"**.

**Does not cover:** what a provider does with a disk after a rental ends, which §10 states is neither requestable nor observable by Smoozy. And **nothing in the connection distinguishes a recycled machine from a rebooted one**, which is the same batch's finding.

## §10 — the provider's own secret found on a rented machine

**Pointer.** [Six machines](../Six%20machines%20—%20the%20container,%20the%20webhook%20and%20the%20ready%20action.md): a Vast.ai per-instance key on the container **"listed every instance on the account with models, prices and images, and could restart them"**.

**Does not cover:** other providers. §10 names it only to fix the altitude — Smoozy neither puts it there nor can remove it.

## §12.1 — the local computer identity is forgeable on all three systems

**Pointer.** [The computer itself — its name, its identity, and its clock](../The%20computer%20itself%20—%20its%20name,%20its%20identity,%20and%20its%20clock.md), Fact 11, answered **2026-08-16**, which owns every vendor sentence §12.1 quotes: Microsoft's **"someone can easily spoof a software KSP as a TPM KSP with local administrator credentials"**; Apple's, that any I/O Registry property **"can be spoofed by knowledgeable persons"** and that `gethostuuid(2)`'s inputs **"can themselves be modified"**; systemd's, that `systemd.machine_id=` on the kernel command line **"has higher priority and will be used instead of the ID stored in /etc/machine-id"**. The same document owns `GetSystemIdForPublisher`'s two sentences — the Note that **"The system identification string will be identical for all unpackaged (Win32) apps"** and the method page's **"Unique for each system"** — which is the pair §12.1 records itself as having read backwards until 2026-08-21.

**Does not cover:** any test. **Forgeability itself was never tested on any system**; all three are documented assertions. And whether the identity survives a reinstall or a hardware change is **documented by nobody on any of the three** — §14 item 10, which is what the reinstall and snapshot rows of §12.4 rest on.

## §12.1 — the `DCDevice` lead

**Pointer.** [The computer itself](../The%20computer%20itself%20—%20its%20name,%20its%20identity,%20and%20its%20clock.md), Fact 11. `DCDevice.current.generateToken` returned a token, twice, from an ad-hoc-signed, bundle-less macOS binary with no entitlements and no Developer ID; the two tokens **differed**; and **"unforgeable", "tamper", "spoof" and "forge" occur zero times** across the DeviceCheck framework's eleven pages.

**Does not cover:** forgery resistance, and whether Apple's server would honour such a token — §14 item 11, undocumented and untested. §12.1 forbids anything downstream resting on it.

## §12.1 — the two obligations that attach to reading an identity

**Pointer.** [The computer itself](../The%20computer%20itself%20—%20its%20name,%20its%20identity,%20and%20its%20clock.md). systemd's own instruction: **"It should be considered \"confidential\", and must not be exposed in untrusted environments, in particular on the network. If a stable unique identifier that is tied to the machine is needed for some application, the machine ID or any part of it must not be used directly. Instead the machine ID should be hashed with a cryptographic, keyed hash function, using a fixed, application-specific key."**, with `sd_id128_get_machine_app_specific()` as the documented implementation. And Apple's declaration requirement: `ProcessInfo.systemUptime` and `mach_absolute_time` both carry the notice that the API **"has the potential of being misused to access device signals to try to identify the device or user, also known as fingerprinting"**, with usage to be declared in `PrivacyInfo.xcprivacy`.

**Does not cover:** which identifier `Access` chooses on each system, which is that document's under Row 223 and is constrained rather than settled here.

## §12.2 — the computer name is a string

**Pointer.** [The computer itself](../The%20computer%20itself%20—%20its%20name,%20its%20identity,%20and%20its%20clock.md), Fact 12. Two computers can carry the same computer name on all three systems, and no system documents a hostname as derived from hardware; on Linux the compiled-in fallback is `localhost`; macOS renumbers the **local network name only**, per link; NetBIOS marks the name in conflict and returns **`NAME CONFLICT`** with no renumbering; Linux documents nothing at the hostname layer.

**Also from the same document, observed:** `gethostname(3)` on macOS can return a **fourth** string that is neither of the two actually set, with `HostName` unset on the machine tested and `gethostname(3)` returning `LocalHostName` plus `.local`; and `logname`/`getlogin()` returned the literal `root` while `id -un` returned the real account name at effective uid 501.

**Does not cover:** Windows and Linux for the two reading traps — both were observed on one Mac. The name half of Fact 12 is documentation on all three.

## §12.3 — no clock reading survives both a reboot and an administrator

**Pointer.** [The computer itself](../The%20computer%20itself%20—%20its%20name,%20its%20identity,%20and%20its%20clock.md), Fact 13. `QueryPerformanceCounter` is **"completely independent of the system time and UTC"** and counts only **"since the Windows operating system was started"**; every cross-reboot record is wall-clock-derived; `SeSystemtimePrivilege`'s holder can make event-log and file timestamps inaccurate; `CAP_SYS_TIME` covers the hardware clock as well as the system clock; `utmp(5)` says of the boot record **"if it is removed, record-keeping is turned off"**; and systemd's clock epoch is Linux-only, distribution-dependent, and defined from file modification times systemd never claims are protected.

**Does not cover:** anything observed about a wound-back clock. **No claim about what a wound-back clock does to any reading is observed on any system**, deliberately — the brief forbade moving the Creator's clock. The three rules §12.3 draws from this are decisions about Smoozy's own service, not facts, and earn no citation.

## §12.3 — the two Darwin clock traps and the withdrawn identity

**Pointer.** [The computer itself](../The%20computer%20itself%20—%20its%20name,%20its%20identity,%20and%20its%20clock.md). Darwin's `CLOCK_MONOTONIC_RAW` is the sleep-**inclusive** clock and `CLOCK_UPTIME_RAW` the sleep-**exclusive** one, the reverse of the Linux convention — observed, the two differing by **10.87 days over 34** on one machine. And the claim that a Mac's sleep-inclusive monotonic clock and `kern.boottime` are two views of one quantity was **withdrawn**: neither run supports it and the two runs disagree.

**Does not cover:** any machine but the one measured. §12.3 states that anything which would have rested on the withdrawn identity must be re-decided rather than inherited.

## §12.4 — the attempt table

**No entry beyond the facts already cited.** Every *met at* and *held* cell is either a rule this document writes, which is a decision, or a consequence of Facts 11 and 13, whose citations are the three entries above. The two rows that name a rule of this document — the per-request value and the authenticated channel — are decisions and are cited to `Licence service` §5 and §7 for their mechanism, which is that document's.

## §13 rule 2 — the machine's own settings hold third-party secrets in the clear

**Pointer.** [The rented machine](../The%20rented%20machine.md), *What identifies the rental*: the provider's own settings **"hold secrets in plain text — an access key for the provider's own service, and a password for a web page it runs"**, with the finding's own conclusion that **"Anything that reads the machine's settings and writes them into a log would be writing those out too."**

**Does not cover:** other providers, and the secrets themselves, which that document deliberately does not quote.

---

## What no source in this project covers

Recorded so nobody downstream reads an absence as a settled negative.

**Items 2, 3, 4, 5 and 10 are permanently uncovered, 2026-09-02.** Each wants a machine this project has not got — a Windows computer, a Linux computer, a rented host reprovisioned — and the Creator ruled on **2026-08-31** that no further machine is rented to answer a question. **They are not pending and no later reading closes them.** **Guard, and it covers all five: an absence recorded here must never be quietly recovered as a settled negative, and no claim resting on one may ever be re-labelled *observed*.** The document's own §14 items 5, 8, 9, 10 and 17 carry what each one breaks.

1. **What the blocked `SecItemCopyMatching` was waiting on, and whether it ever returns** (§14 item 14). Observed only that it did not return within 45 seconds and was killed. Rule 5's deadline is a rule about how long Smoozy waits, not a claim about what the keychain does.
2. **Every Windows claim in §3.2** (§14 item 8). Documentation alone, one load-bearing sentence archived and dated 2016-08-31, and no Windows machine ran any of it.
3. **Every Linux claim in §3.3** (§14 item 9). Never observed; no Linux computer on this project.
4. **What error a Secret Service client receives when no provider is running, and whether any provider runs without a desktop session** (§14 item 7).
5. **Whether the local computer identity survives a reinstall or a hardware change** (§14 item 10). Documented by nobody, on any of the three.
6. **`DCDevice`'s forgery resistance** (§14 item 11). Undocumented and untested; nothing is built on it.
7. **Telegram's webhook retry cadence, and that alone** (§14 item 12). **Corrected 2026-09-02: this read *"The webhook POST wrapper, its secret-token header, and Telegram's retry cadence"*, and two of those three are covered — by Telegram's own Bot API page, read 2026-09-02, quoted in the §14 item 12 entry above.** The cadence is not, and the page is the reason: *"we will repeat the request and give up after a reasonable amount of attempts"* names no interval, no count and no deadline, so no reading will ever close it.
8. **That the elevation tool's refusal strings are documented anywhere, or fixed across releases** (§13 rule 5's exception, §14 items 15 and 17). **Narrowed twice.** **2026-08-23** — this line read *whether the elevation route yields a separable authentication result*, and that half is answered by Fact 108: the validate form runs no command, so its result is separable from the requested operation's own. **2026-08-29** — it then read *whether a failed elevation validate separates a wrong password from a login not permitted to elevate*, and **that is answered too, by the Creator's ruling of 2026-08-25**: §13 rule 5's exception matches the wrong-password ending as an exact constant, so the separation exists and §4's erase fires on it. **What no source covers is the ground under that exception** — the strings are **observed on one rental**, `sudo` 1.9.15p5 under Ubuntu 24.04, 2026-08-22, and no manual page read names one of them; a second Ubuntu release was never read or run. The virtual-machine kind under a non-root login stays unposed.
9. **That no SHA-256 collision exists.** No first-party sentence says so, and this document does not assert it.
10. **Whether a rented machine's host keys ever change without the hardware changing** (§14 item 5). **Added 2026-08-23**, when Fact 83 stopped being an open question here and became an answer with a named residual. Nothing observed on either provider posed a host reprovisioning or rebuilding its image, and the *on every host* half of the image reading is an inference from one machine watched across a recycle rather than from two rentals on two hosts.

---

## §14 item 12 — the webhook wrapper and the secret-token header

**Direct publication, read 2026-09-02.** Telegram's own *Bot API* page, `https://core.telegram.org/bots/api`, the page carrying **Bot API 10.3** and the date **August 24, 2026** at its head. Exact words, from `setWebhook`:

> "Whenever there is an update for the bot, we will send an HTTPS POST request to the specified URL, containing a JSON-serialized Update. In case of an unsuccessful request (a request with response HTTP status code different from 2XY), we will repeat the request and give up after a reasonable amount of attempts."

> "If you'd like to make sure that the webhook was set by you, you can specify secret data in the parameter secret_token. If specified, the request will contain a header “X-Telegram-Bot-Api-Secret-Token” with the secret token as content."

And from the `secret_token` parameter row of the same method:

> "A secret token to be sent in a header “X-Telegram-Bot-Api-Secret-Token” in every webhook request, 1-256 characters. Only characters A-Z, a-z, 0-9, _ and - are allowed. The header is useful to ensure that the request comes from a webhook set by you."

**What it covers.** The wrapper — an HTTPS POST carrying a JSON-serialized `Update` — and the authentication of the one exceptional inbound endpoint of §1.3, which is a header the caller sets at `setWebhook` and the platform then sends on every request. Both were carried as owed under Fact 63 and neither needed a machine.

**Does not cover: the retry cadence, and the page is why it never will be.** *"a reasonable amount of attempts"* names no interval, no count and no deadline, so no reading closes it; only a run against a deliberately failing endpoint would. **This is a reading rather than a run**: nothing was deployed, no webhook was registered, and no bot exists on this project.

**Routed, not written:** `Telegram.md` §21 item 5 and `Facts.md` Fact 63, which carry Fact 63 as owed in full and are not this document's to edit.
