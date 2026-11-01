# Sources — The manuals — keys, secrets, and what cannot be told apart

*Where the outside-world facts in [The manuals](../The%20manuals%20—%20keys,%20secrets,%20and%20what%20cannot%20be%20told%20apart.md) came from. The document carries each fact's label in place; this file carries the citation behind it, and what it does not cover. **Every address below was read on 2026-08-16**, and every one was re-fetched on the same day by the citation check that repaired the findings.*

---

## What no citation here covers

Three limits apply to every entry and are not repeated under each.

- **No Windows machine has run it and none is scheduled.** Every Microsoft citation is documentation only. None supports a claim about behaviour on a real installation.
- **Every observation was made on one machine, on one build** — macOS 26.5.2 (build 25F84), arm64, `OpenSSH_10.2p1, LibreSSL 3.3.6`. A transcript supports a claim about what that command printed on that build; it does not support a general statement, and where the findings made one the checks narrowed it back. **Linux was never observed.**
- **`man.openbsd.org` serves OpenBSD-current**, which runs ahead of shipping operating systems. No manual page citation supports a claim about what any particular user's OpenSSH contains.

---

## OpenSSH manual pages — `man.openbsd.org`

Used for Facts 32, 33, 34, 35, 36, 37, 39 and 76. All seven returned HTTP 200. Each page served without a version selector is the **OpenBSD-current** rendering; the footer date as served on 2026-08-16 is given, because a citation without a version cannot be re-checked later.

| Page | Address | Footer as served | Facts |
|---|---|---|---|
| ssh(1) | https://man.openbsd.org/ssh.1 | OpenBSD-current, August 4, 2026 | 32, 33, 34, 35, 36, 37, 76 |
| ssh-keygen(1) | https://man.openbsd.org/ssh-keygen.1 | OpenBSD-current, August 7, 2026 | 32, 34, 35, 36, 37, 39, 76 |
| ssh-add(1) | https://man.openbsd.org/ssh-add.1 | OpenBSD-current, July 11, 2026 | 33 (absence of the Apple options upstream), 37 |
| ssh-agent(1) | https://man.openbsd.org/ssh-agent.1 | OpenBSD-current, May 27, 2026 | 33, 34, 37, 76 |
| ssh_config(5) | https://man.openbsd.org/ssh_config.5 | OpenBSD-current, July 11, 2026 | 32, 33, 34, 35, 36, 37, 76 |
| sshd(8) | https://man.openbsd.org/sshd.8 | OpenBSD-current, August 15, 2026 | 32, 34, 37 |
| sshd_config(5) | https://man.openbsd.org/sshd_config.5 | OpenBSD-current, August 8, 2026 | 32, 37, 76 |

**What these citations do not cover.**

- **No version history.** These pages carry no OpenSSH release-version marker against any key type, any error string, or any fingerprint default. The one release-version marker of any kind across the six pages read for Fact 32 is ssh_config(5) `UpdateHostKeys` — "Presently, only sshd(8) from OpenSSH 6.8 and greater support the "hostkeys@openssh.com" protocol extension…" — which concerns host-key rotation. Nothing here dates a feature.
- **No error strings.** None of the diagnostic strings Fact 37 relies on appears in any of the seven pages. Eighteen strings were searched across all seven and returned zero hits in every page. The pages therefore support the *absence* of documentation for those strings, and nothing about their stability. The one near-miss — `kex_exchange_identification` appearing in ssh_config(5) and sshd_config(5) — is a source-function name inside the `LogVerbose` example, not a documented message.
- **No private-key file layout.** `openssh-key-v1` returns count zero across all six pages read for Fact 35. The container layout described there is an observation of file bytes, not a citation. The OpenSSH private-key **format specification** was outside the permitted sources and was not consulted; it is what Fact 37's central open question needs.
- **No DSA statement.** `ssh-dss`, `id_dsa` and `ssh_host_dsa` return count zero in each page. That supports "DSA is not a documented key type"; it does **not** support "DSA was removed", which no page states.
- **Nothing about collisions or portability of fingerprints.** `collision` returns count zero in all six pages read for Fact 76; no page makes any portability statement about fingerprints. Fact 76's uniqueness question is not answerable from these citations at all.
- **A contradiction inside the manual itself.** ssh-keygen(1) FILES says a passphrase encrypts the private part "using 128-bit AES" and ssh(1) FILES says "using AES-128", while ssh-keygen(1) `-Z` says the default cipher is `aes256-ctr`. All three are verbatim on their pages. The instruction in the document to treat `-Z` as accurate is a judgement, not a citation.

---

## Apple — macOS SSH key location and Keychain integration (Fact 33)

- <https://support.apple.com/guide/mac-help/allow-a-remote-computer-to-access-your-mac-mchlp1066/mac> — read 2026-08-16, HTTP 200.

  **What it does not cover:** it does not state where SSH keys are kept. The whole page was searched: no occurrence of `.ssh`, `ssh-keygen` or `key pair`. Its content is System Settings → Sharing → Remote Login plus how to connect in from another computer, and its "See also" list defers to "man page for ssh". It is cited only for that deferral.

- **The Apple-shipped `ssh(1)`, `ssh_config(5)`, `ssh-add(1)` and `ssh-agent(1)` manual pages on the running system** — read from `/usr/bin/man` on macOS 26.5.2 (build 25F84), arm64, 2026-08-16. Label: **observed**.

  **What they do not cover:** they are the pages of one installed build. The Apple-shipped `ssh(1)` lists five private-key names where the OpenBSD-current page lists six; that difference is evidence about this build, not about macOS generally or about other versions. That Apple authored the `UseKeychain`, `--apple-use-keychain` and `--apple-load-keychain` options is **inferred** from their absence upstream (`keychain` occurs zero times in the served ssh_config(5) and ssh-add(1)) plus the `apple-` prefix; **no source read says so.**

---

## Microsoft — OpenSSH for Windows (Facts 33, 34, 39)

All four returned HTTP 200. Two now redirect and were followed.

| Address | Redirects to | Facts |
|---|---|---|
| https://learn.microsoft.com/en-us/windows-server/administration/openssh/openssh_overview | `.../administration/OpenSSH/openssh-overview` | 39 |
| https://learn.microsoft.com/en-us/windows-server/administration/openssh/openssh_install_firstuse | — (no redirect) | 39 |
| https://learn.microsoft.com/en-us/windows-server/administration/openssh/openssh_keymanagement | — (no redirect) | 33, 34, 39 |
| https://learn.microsoft.com/en-us/windows-server/administration/openssh/openssh_server_configuration | `.../administration/OpenSSH/openssh-server-configuration` | 33, 34 |

**What these citations do not cover.**

- **Nothing observed.** No claim drawn from them has been run on a Windows machine.
- **They do not define the Windows home directory.** No page read states that `%USERPROFILE%` expands to `C:\Users\<username>`; both forms appear, the equation between them does not. Nor does any page state how "your home directory" is resolved when `$HOME` and `%USERPROFILE%` disagree.
- **They do not say what is in `C:\ProgramData\ssh`.** Microsoft says "the host keys are stored under C:\ProgramData\ssh". That a host key stored there includes a private half is the reader's step; no page read states it.
- **They do not cover Windows 11's install state.** Windows 11 appears in the applies-to banner and is **not a row** in the install-state table, which also does not separate the Client capability from the Server capability.
- **They do not document `-C` or `-N` for the Windows build.** The only `ssh-keygen` invocation shown anywhere across the four pages is `ssh-keygen -t ecdsa`. That the OpenSSH options apply because the Windows port is a fork is an expectation, not a citation.
- **The DSA listing is uncorroborated and possibly stale.** `openssh_keymanagement` lists DSA among the key-generation algorithms; `ssh-keygen(1)` lists no `dsa` under `-t` and the observed 10.2p1 binary does not offer it. The contradiction is recorded, not resolved.
- **They do not cover non-Microsoft SSH implementations** — PuTTY `.ppk` files or a `~/.ssh` inside WSL are mentioned on no page read.

---

## Canonical — Ubuntu Server OpenSSH (Facts 33, 39)

- <https://documentation.ubuntu.com/server/how-to/security/openssh-server/> — read 2026-08-16, HTTP 200 after a redirect to <https://ubuntu.com/server/docs/how-to/security/openssh-server/>.

  **What it does not cover:** it is **one distribution**. It states the key location affirmatively and names `ssh-keygen -t ed25519`, but it gives install commands (`sudo apt install openssh-client`) rather than asserting the client is preinstalled — so it settles nothing about whether the tool is present without installing anything, on Ubuntu or anywhere else. Its `/etc/ssh/sshd_config.d/` material is server-side and says nothing about key locations. Nothing here supports a statement about Linux in general.

## Linux distribution documentation that refused to serve

Recorded so they are not silently re-attempted. All three refusals were re-tested on 2026-08-16 and all three reproduce. **No claim in the document rests on any of them, and their absence is why "no distribution documentation contradicts it" means Canonical's alone.**

- `docs.redhat.com` — HTTP 403 with a 503-byte body, to `https://docs.redhat.com/en/documentation/red_hat_enterprise_linux/9/html/configuring_basic_system_settings/index`.
- `docs.fedoraproject.org` — HTTP 200 whose body is an Anubis proof-of-work interstitial ("Making sure you're not a bot!", "Anubis version 1.25.0-1.fc44"); no documentation content served.
- `packages.debian.org` — `https://packages.debian.org/stable/openssh-client` redirects to `https://packages.debian.org/.internal/challenge.html?original=%2fstable%2fopenssh-client`.

---

# Fact 38 — OS secret storage

Every address below was read on 2026-08-16 and every one was fetched again by the citation check the same day: **no dead addresses, no moved pages.** Two method notes. Apple's `developer.apple.com/documentation/...` pages are a single-page application and were read through their backing endpoint `developer.apple.com/tutorials/data/documentation/....json`, which carries the same prose and the same inline code identifiers. The `gitlab.gnome.org` raw paths refuse a plain browser request with HTTP 406 and were read through the project's GitLab API on the `main` branch.

## Apple — macOS keychain

Developer documentation:

- <https://developer.apple.com/documentation/technotes/tn3137-on-mac-keychains> (TN3137, *On Mac keychain APIs and implementations*)
- <https://developer.apple.com/documentation/security/keychain-services>
- <https://developer.apple.com/documentation/security/keychain-items>
- <https://developer.apple.com/documentation/security/keychains>
- <https://developer.apple.com/documentation/security/access-control-lists>
- <https://developer.apple.com/documentation/security/secitemadd(_:_:)>
- <https://developer.apple.com/documentation/security/secitemcopymatching(_:_:)>
- <https://developer.apple.com/documentation/security/secitemdelete(_:)>
- <https://developer.apple.com/documentation/security/seckeychainsetuserinteractionallowed(_:)>
- <https://developer.apple.com/documentation/security/ksecattraccessible>
- <https://developer.apple.com/documentation/security/ksecattraccessiblewhenunlockedthisdeviceonly>
- <https://developer.apple.com/documentation/security/ksecattrsynchronizable>
- <https://developer.apple.com/documentation/security/ksecusedataprotectionkeychain>
- <https://developer.apple.com/documentation/security/restricting-keychain-item-accessibility>
- <https://developer.apple.com/documentation/security/sharing-access-to-keychain-items-among-a-collection-of-apps>
- <https://developer.apple.com/documentation/security/security-framework-result-codes> and the individual `errSec*` pages linked from it

Platform Security and the Keychain Access user guide:

- <https://support.apple.com/guide/security/keychain-data-protection-secb0694df1a/web>
- <https://support.apple.com/guide/keychain-access/about-your-keychain-password-mchlp1086/mac>
- <https://support.apple.com/guide/keychain-access/if-youre-asked-for-access-to-your-keychain-kyca1243/mac>
- <https://support.apple.com/guide/keychain-access/if-a-trusted-app-asks-for-keychain-access-kyca1331/mac>
- <https://support.apple.com/guide/keychain-access/if-you-need-to-update-your-keychain-password-kyca2429/mac>
- <https://support.apple.com/guide/keychain-access/mac-keychain-password-kyca1242/mac>
- <https://support.apple.com/guide/keychain-access/delete-a-keychain-kyca2423/mac>

**What these citations do not cover.**

- **No negative threat model.** Apple documents what is encrypted, with which keys, and who arbitrates access. A search across every page listed for "does not protect", "doesn't protect", "not protect against" and "threat model" returns nothing. Nothing here supports a statement about what the keychain fails to stop.
- **No item size limit.** A search for "maximum size", "size limit" and "bytes" across all fetched Apple pages returns nothing on item size.
- **No numeric `OSStatus` values.** The result-code list and each individual `errSec*` page carry the **symbol and a one-line description only**; no numeral appears in the fetched page data for any of them. The numbers in the document's table are retained because −25293, −25300 and −34018 were observed as returned values, but **no cited authority maps a number to a symbol.**
- **No conditions for three of the codes.** No page read states when `errSecNoSuchKeychain`, `errSecInvalidKeychain` or `errSecInteractionNotAllowed` are returned — only what they mean. Only `errSecInteractionRequired` has a source stating its condition.
- **No code for a Deny click.** No page read states the `OSStatus` returned when the user clicks Deny in the ACL dialog.
- **No presence claim.** No page states that the Security framework or its four `SecItem*` functions are present on every stock macOS installation. What is on the record is narrower and observed: they resolved on the one machine without anything being installed.
- **`sharing-access-to-keychain-items` is scoped to sharing.** Its "doesn't require interaction with or permission from the user" sentence is about sharing items among apps from one development team. It does not carry a general statement about every data-protection access.
- **`kyca1331` says "sometimes".** It documents that the dialog "appears if you recently updated your system software or the app, or if the app has been modified", and on the same page that "You must sometimes reauthorize updated apps". It does not support "after every update".
- **`kyca2423` documents deleting a whole keychain**, not an individual item, and documents two recovery routes for a deleted keychain — knowing its password, and restoring from Time Machine. No page read says iCloud Keychain recovers a lost login keychain.
- **The `errSecMissingEntitlement` page scopes itself:** "The method returns this error when you specify an access group to which your app doesn't belong."

**Observed on macOS, kept separate from all of the above.** Transcripts from `/usr/bin/security` and from a Python 3 `ctypes` harness against the Security framework, run 2026-08-16 on the Apple-silicon Mac under throwaway service names. **No existing keychain item was read, listed, dumped, searched or printed.**

**What the observations do not cover.** No macOS version and no interpreter identity is recorded for the original runs, so none of them is repeatable from its own description; the check's own repeats were on macOS 26.5.2 build 25F84, arm64, with `/usr/bin/python3`. **One observation was run twice and disagreed** — a `SecItemAdd` against the data protection keychain from a process with no keychain-access-group entitlement returned `errSecMissingEntitlement` the first time and `0` the second. **Both runs are recorded in the document and neither is adjudicated; the behaviour is unresolved.** One observation could not be repeated at all — the call that hung — because repeating it would put a system dialog on the user's screen; it establishes that the call did not return within 45 seconds and that a `SecurityAgent` process was running afterwards, and **nothing more**: that a dialog was on screen, that `SecurityAgent` is what displays it, and that the call waits on a human are all **inferred**.

## Microsoft — Credential Manager and DPAPI

- <https://learn.microsoft.com/en-us/windows/win32/api/wincred/nf-wincred-credwritew>
- <https://learn.microsoft.com/en-us/windows/win32/api/wincred/nf-wincred-credreadw>
- <https://learn.microsoft.com/en-us/windows/win32/api/wincred/nf-wincred-creddeletew>
- <https://learn.microsoft.com/en-us/windows/win32/api/wincred/ns-wincred-credentiala>
- <https://learn.microsoft.com/en-us/windows/win32/api/wincred/nf-wincred-credprotectw>
- <https://learn.microsoft.com/en-us/windows/win32/api/dpapi/nf-dpapi-cryptprotectdata>
- <https://learn.microsoft.com/en-us/windows/win32/api/dpapi/nf-dpapi-cryptunprotectdata>
- <https://learn.microsoft.com/en-us/previous-versions/ms995355(v=msdn.10)> — *Windows Data Protection*
- <https://learn.microsoft.com/en-us/previous-versions/windows/it-pro/windows-server-2012-r2-and-2012/hh994565(v=ws.11)> — *Cached and Stored Credentials Technical Overview*

**Two of these are archived pages considerably older than the rest, and they carry the heaviest claims in the section.**

- **`hh994565`** carries `is_archived: true` and `ms.date 2016-08-31`, and sits in the Windows Server 2012 R2 and 2012 previous-versions set. **It is the only source for "Any program running as that user will be able to access credentials in this store"** — the single most load-bearing Windows sentence in Fact 38. Whether it still holds for current Windows is unresolved.
- **`ms995355`** is dated **October 2001** (byline "NAI Labs, Network Associates, Inc. October 2001") and is flagged `is_archived: true`. It introduces the Password Reset Disk as "Well, Windows XP has a new feature". It is the source for the whole MasterKey, Credential History, domain-escrow and PRD material.

**What these citations do not cover.**

- **Nothing observed.** No Windows machine; none of it has been run.
- **They do not state that `Advapi32.dll` ships with every installation.** The Requirements table gives "Minimum supported client: Windows XP" and "Minimum supported server: Windows Server 2003" — a minimum supported version, not a presence guarantee. Only `Crypt32.dll` has a sentence that carries presence, and only for that DLL.
- **No page for `CredEnumerate` or `CredFree` was read**; neither is cited.
- **They do not cover an administrative password reset on a standalone machine with no PRD**, nor Microsoft-account or Windows Hello logins, which post-date the DPAPI article.
- **They do not say whether `CRED_PERSIST_SESSION` credentials touch disk.** No page distinguishes persistence classes on that point.
- **They document no prompt on the `CRED_TYPE_GENERIC` path.** The word "prompt" does not occur at all on `CredWriteW`, `CredReadW` or `CredDeleteW`. One prompting path *is* documented — `CRED_FLAGS_PROMPT_NOW`, application-driven, for `CRED_TYPE_DOMAIN_PASSWORD` and `CRED_TYPE_DOMAIN_CERTIFICATE` only — which is why the document does not say the user is never asked.
- **`CryptUnprotectData` disclaims its own error codes:** "Applications should not rely on a specific error code to detect data tampering." No classification of a DPAPI decrypt failure is supportable from these pages.
- **The February 2027 removal is the vendor's future tense** on a page read 2026-08-16, not a completed fact.

## freedesktop.org — Secret Service API (Draft)

- <https://specifications.freedesktop.org/secret-service-spec/latest/> — chapters ch01, ch02, ch03, aliases, lookup-attributes, sessions, transfer-secrets, unlocking, prompts, ch10, ch11, object-paths, interfaces, errors.

**What it does not cover — and this is the point of citing it.** The document is titled "Secret Service API **Draft**". It states outright that it "does not mandate the use of master passwords to lock a collection of secrets" and "does not mandate any form of access control", and both statements are followed by "[TODO: complete]". Chapter 11, "Notes for Service Implementors", reads in its entirety "[TODO: complete]". **So the specification supports no claim about what a secret is protected against on Linux.** It defines exactly three errors, and **none of them covers "no service is running"**. It does not state the well-known bus name anywhere across the fourteen pages read — neither "bus name" nor "well-known name" occurs, and `org.freedesktop.secrets` appears only as an object path. Its aliases chapter names only `default` as an example, so it does not establish that any implementation honours the `session` alias.

## The implementations

- <https://gnome.pages.gitlab.gnome.org/libsecret/> — index, `const.COLLECTION_SESSION`, `enum.Error`, `func.password_store_sync`, `func.password_lookup_sync`, `func.password_clear_sync`, `libsecret-simple-api`, `libsecret-using`, `iface.Backend`, `libsecret-tpm2`
- <https://gitlab.gnome.org/GNOME/gnome-keyring/> — `README`, `docs/keyring-intro.txt`, `docs/gnome-keyring-daemon.xml` (fetched raw through the project's GitLab API on `main`)
- <https://invent.kde.org/frameworks/kwallet/-/blob/master/README.md>
- <https://docs.kde.org/trunk_kf6/en/kwalletmanager/kwalletmanager/introduction.html> — *The KWallet Handbook*
- <https://keepassxc.org/docs/KeePassXC_UserGuide>

**What these citations do not cover.**

- **Nothing observed.** No Linux machine was available; every Linux claim is documentation only.
- **They do not say what runs on a machine with no desktop session.** gnome-keyring's man page says the daemon "is normally started automatically when a user logs into a desktop session"; the KWallet Handbook says the wallet opens at login "if kwallet_pam is installed and properly configured". **Neither states what exists, or fails to exist, headless.** That is the single most likely condition for a machine without a desktop and it is precisely the one no source names.
- **They do not name the error a client gets when no provider is running.** libsecret's error enum has nine members and none covers it; the specification's three errors do not either.
- **libsecret is a client, not a store**, and its documentation read here does not state how its file backend is selected at runtime or whether it is available in a normal (non-Flatpak) build.
- **The `session` collection's memory-only guarantee is GNOME's alone.** Whether `ksecretd` and KeePassXC implement the alias, and with what disk guarantee, is stated by neither project.
- **The KWallet README describes only the current `kwalletd` / `ksecretd` split**, so it does not date when `org.freedesktop.secrets` first appeared in KDE.
- **Neither GNOME's nor KDE's documentation read here covers a login-password change outside PAM, or an administrative reset.**
- **The KWallet Handbook offers two storage options** — "Blowfish encryption" and "GPG encryption". Only the Blowfish paragraph is quoted; nothing here describes the GPG path.
- **KeePassXC's every-access prompting is scoped to `secret-tool`**, which the guide attributes to `secret-tool` "obtaining a new process id each time it is run". It does not say a long-running application is prompted on every access.

