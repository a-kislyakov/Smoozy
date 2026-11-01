# The manuals — keys, secrets, and what cannot be told apart

What the OpenSSH, Apple, Microsoft, freedesktop.org and desktop-project manuals answered on 2026-08-16 about SSH keys, key fingerprints, and where an operating system will hold a secret for you. An **input** to the stage, like [Batch A](Batch%20A%20—%20the%20shape%20throwaway.md) and [One box](One%20box%20—%20the%20Telegram%20capture%20and%20the%20port%20that%20cannot%20be%20chosen.md): nothing here is a decision, a plan, or a recommendation about what the application should do.

It answers nine facts — 32, 33, 34, 35, 36, 37, 38, 39 and 76 — assembled from four findings files, each independently citation-checked on the same day. Every claim carries a label: **documented**, **observed**, **inferred**, or **unresolved**. The labels are carried through exactly as the checks left them; where a check relabelled something from documented to inferred, it is inferred here. Rule 1's plain-English rule is suspended, per the stage's exception.

---

## Three limits, before anything else

**One. No Windows machine has run any of this, and none is rented today.** A Windows machine *can* be rented — this project has rented four, on a Kamatera account that allows one machine at a time — so the Windows silence is **untried rather than untestable**. **Every Windows machine this project has read reported a Server edition**, not the desktop edition its image was named for, so a question about a desktop edition is still out of reach of what has been rented, and any Windows answer must name the edition it was taken on. Every Windows answer below is documentation alone. Nothing in the Windows material has been confirmed against a real installation, and none of it is labelled observed.

**Two. Everything observed was observed on one machine, on one build.** macOS 26.5.2 (build 25F84), arm64 (Apple silicon), `OpenSSH_10.2p1, LibreSSL 3.3.6`, `/usr/bin/ssh-keygen`. A record of what a command printed supports a claim about what that command printed; a general statement is a step beyond it, and where the findings took that step the checks narrowed it back. Linux was not observed at all — the only machine available is a Mac, so every Linux claim is documentation-only too.

**Three. Some sources are old, and some refused to serve.**

- Two of the Windows sources are archived pages considerably older than the rest. **`hh994565`** (*Cached and Stored Credentials Technical Overview*) carries `is_archived: true` and `ms.date 2016-08-31`, and sits in the Windows Server 2012 R2 and 2012 previous-versions set — it is the only source for the single most load-bearing Windows sentence in Fact 38. **`ms995355`** (*Windows Data Protection*) is dated **October 2001**, is flagged `is_archived: true`, and introduces the Password Reset Disk as "Windows XP has a new feature".
- Three documentation sites refused automated retrieval on 2026-08-16, and all three refusals were re-tested and reproduce: **`docs.redhat.com`** returns HTTP 403 with a 503-byte body; **`docs.fedoraproject.org`** returns HTTP 200 whose body is an Anubis proof-of-work interstitial ("Making sure you're not a bot!", "Anubis version 1.25.0-1.fc44") and no documentation content; **`packages.debian.org`** redirects to `/.internal/challenge.html`. No claim here rests on any of them. Canonical's Ubuntu Server documentation is therefore the *only* Linux distribution documentation that could be read at all.

**And a version caveat that touches every OpenSSH answer.** `man.openbsd.org` serves **OpenBSD-current**, which runs ahead of what shipping operating systems contain. The current pages document an `mldsa44-ed25519` key type that the installed OpenSSH 10.2p1 does not offer as a `-t` argument. Treat the OpenBSD manual as the definition of the convention, never as the list of key types a user's installation actually has. Where a documented list and an observed list differ, both are given below.

---

## Fact 32 — which SSH key types are current, which a supported OpenSSH accepts, and which is the sensible modern default

### Documented

**The key types `ssh-keygen` can generate.**

> "Specifies the type of key to create. The possible values are “ecdsa”, “ecdsa-sk”, “ed25519 (the default),” “ed25519-sk”, “mldsa44-ed25519” or “rsa”. This flag may also be used to specify the desired signature type when signing certificates using an RSA CA key. The available RSA signature variants are “ssh-rsa” (SHA1 signatures, not recommended), “rsa-sha2-256”, and “rsa-sha2-512” (the default for RSA keys)."
> — ssh-keygen(1), `-t`

The odd placement of the closing quote around “ed25519 (the default),” is how the page itself renders; it is reproduced rather than corrected. The SYNOPSIS gives the same set as `[-t ecdsa | ecdsa-sk | ed25519 | ed25519-sk | mldsa44-ed25519 | rsa]`.

**The default when no type is given.**

> "The type of key to be generated is specified with the -t option. If invoked without any arguments, ssh-keygen will generate an Ed25519 key."
> — ssh-keygen(1), DESCRIPTION

**Sizes per type.**

> "Specifies the number of bits in the key to create. For RSA keys, the minimum size is 1024 bits and the default is 3072 bits. Generally, 3072 bits is considered sufficient. For ECDSA keys, the -b flag determines the key length by selecting from one of three elliptic curve sizes: 256, 384 or 521 bits. Attempting to use bit lengths other than these three values for ECDSA keys will fail. ECDSA-SK, Ed25519 and Ed25519-SK keys have a fixed length and the -b flag will be ignored."
> — ssh-keygen(1), `-b bits`

**What the client will use for public key authentication, by default.**

> "Specifies the signature algorithms that will be used for public key authentication as a comma-separated list of patterns. […] The default for this option is:"
>
> "ssh-ed25519-cert-v01@openssh.com, ecdsa-sha2-nistp256-cert-v01@openssh.com, ecdsa-sha2-nistp384-cert-v01@openssh.com, ecdsa-sha2-nistp521-cert-v01@openssh.com, sk-ssh-ed25519-cert-v01@openssh.com, sk-ecdsa-sha2-nistp256-cert-v01@openssh.com, webauthn-sk-ecdsa-sha2-nistp256-cert-v01@openssh.com, rsa-sha2-512-cert-v01@openssh.com, rsa-sha2-256-cert-v01@openssh.com, ssh-mldsa44-ed25519-cert-v01@openssh.com, ssh-ed25519, ecdsa-sha2-nistp256,ecdsa-sha2-nistp384,ecdsa-sha2-nistp521, sk-ssh-ed25519@openssh.com, sk-ecdsa-sha2-nistp256@openssh.com, webauthn-sk-ecdsa-sha2-nistp256@openssh.com, rsa-sha2-512,rsa-sha2-256, ssh-mldsa44-ed25519@openssh.com"
>
> "The list of available signature algorithms may also be obtained using "ssh -Q PubkeyAcceptedAlgorithms"."
> — ssh_config(5), `PubkeyAcceptedAlgorithms`

**What the server will accept, by default.** sshd_config(5) `PubkeyAcceptedAlgorithms` carries the identical default list, worded "Specifies the signature algorithms that will be accepted for public key authentication as a list of comma-separated patterns."

**What key types may appear in `authorized_keys`.**

> "Public keys consist of the following space-separated fields: options, keytype, base64-encoded key, comment. The options field is optional. The supported key types are:"
>
> "sk-ecdsa-sha2-nistp256@openssh.com / ecdsa-sha2-nistp256 / ecdsa-sha2-nistp384 / ecdsa-sha2-nistp521 / sk-ssh-ed25519@openssh.com / ssh-ed25519 / ssh-mldsa44-ed25519@openssh.com / ssh-rsa"
> — sshd(8), AUTHORIZED_KEYS FILE FORMAT (the eight types render as a list; joined with `/` here)

**DSA — an absence, not a statement.** The strings `ssh-dss`, `id_dsa` and `ssh_host_dsa` occur **zero times** in each of ssh-keygen(1), ssh(1), ssh_config(5), sshd(8), sshd_config(5) and ssh-agent(1) as served on 2026-08-16, verified by string search over the fetched pages and re-verified by the check. DSA is therefore not a documented key type in the current manual. **Corrected 2026-08-21: OpenSSH does state that DSA was removed, just not on a manual page.** `https://www.openssh.org/txt/release-10.0`, read 2026-08-20: *"OpenSSH 10.0 was released on 2025-04-09."* and *"This release removes support for the weak DSA signature algorithm, completing the deprecation process that began in 2015 (when DSA was disabled by default) and repeatedly warned over the last 12 months."* And `https://www.openssh.org/txt/release-7.0`, read the same day: *"OpenSSH 7.0 was released on 2015-08-11."*, *"Support for ssh-dss, ssh-dss-cert-* host and user keys is disabled by default at run-time."* **So DSA was present-but-off from August 2015 and gone from April 2025.** Label: **documented**. The manual's silence is a consequence of the removal rather than the only evidence about it.

### Observed — OpenSSH 10.2p1, 2026-08-16

```
$ ssh-keygen -q -N '' -C 'test@local' -f <tmp>/default </dev/null
$ cut -d' ' -f1 <tmp>/default.pub
ssh-ed25519
```

Confirms the documented default type on the installed version.

```
$ ssh -Q key
ssh-ed25519
ssh-ed25519-cert-v01@openssh.com
sk-ssh-ed25519@openssh.com
sk-ssh-ed25519-cert-v01@openssh.com
ecdsa-sha2-nistp256
ecdsa-sha2-nistp256-cert-v01@openssh.com
ecdsa-sha2-nistp384
ecdsa-sha2-nistp384-cert-v01@openssh.com
ecdsa-sha2-nistp521
ecdsa-sha2-nistp521-cert-v01@openssh.com
sk-ecdsa-sha2-nistp256@openssh.com
sk-ecdsa-sha2-nistp256-cert-v01@openssh.com
ssh-rsa
ssh-rsa-cert-v01@openssh.com
```

```
$ ssh -Q PubkeyAcceptedAlgorithms
ssh-ed25519
ssh-ed25519-cert-v01@openssh.com
sk-ssh-ed25519@openssh.com
sk-ssh-ed25519-cert-v01@openssh.com
ecdsa-sha2-nistp256
ecdsa-sha2-nistp256-cert-v01@openssh.com
ecdsa-sha2-nistp384
ecdsa-sha2-nistp384-cert-v01@openssh.com
ecdsa-sha2-nistp521
ecdsa-sha2-nistp521-cert-v01@openssh.com
sk-ecdsa-sha2-nistp256@openssh.com
sk-ecdsa-sha2-nistp256-cert-v01@openssh.com
webauthn-sk-ecdsa-sha2-nistp256@openssh.com
ssh-rsa
ssh-rsa-cert-v01@openssh.com
rsa-sha2-256
rsa-sha2-256-cert-v01@openssh.com
rsa-sha2-512
rsa-sha2-512-cert-v01@openssh.com
```

```
$ ssh-keygen -t badtype -f /dev/null
unknown key type badtype
```

(exit 255, recorded by the check; the findings file did not record the status.)

The installed 10.2p1 build offers **no** `mldsa44-ed25519` / `ssh-mldsa44-ed25519@openssh.com` and **no** DSA. Its own installed manual page renders the `-t` list without ML-DSA:

```
$ man ssh-keygen   (local page, OpenSSH 10.2p1)
     -t ecdsa | ecdsa-sk | ed25519 | ed25519-sk | rsa
             Specifies the type of key to create.  The possible values are "ecdsa", "ecdsa-sk",
             "ed25519 (the default)," "ed25519-sk", or "rsa".
```

This local page is an observation of the installed version, not a man.openbsd.org citation.

### On "the sensible modern default" — what is documented, and what is only judgement

**Documented:** the manual states which type `ssh-keygen` picks when given no arguments (Ed25519), and labels RSA-with-SHA1 signatures "not recommended". **The manual does not rank the types against one another and does not call any of them "recommended", "best" or "modern".** A search for `recommend` across the six pages returns no recommendation attached to any key type; the only key-related hit is the `-t` entry's "(SHA1 signatures, not recommended)" against `ssh-rsa`.

The findings file drew a **judgement** from that, expressly fenced off from the documentation: that `ed25519` is the type to generate. Its four grounds are each documented or observed above — it is the tool's own no-argument default; it has a fixed length so there is no size parameter to get wrong; it is present in the default `PubkeyAcceptedAlgorithms` of both client and server and in the `authorized_keys` supported list; and it exists in both the current documentation and the installed 10.2p1 build, whereas `mldsa44-ed25519` exists only in the former. That last clause is the operative one for portability: **a key type documented on man.openbsd.org is not necessarily present in the OpenSSH the user actually has.** The judgement is recorded here as what the day produced; it is not a decision taken by this document.

### Unresolved for Fact 32

- **From which OpenSSH version onward each key type is available.** The manual pages read carry no OpenSSH release-version marker against any key type. They carry exactly one release-version marker of any kind — ssh_config(5) `UpdateHostKeys`: "Presently, only sshd(8) from OpenSSH 6.8 and greater support the "hostkeys@openssh.com" protocol extension used to inform the client of all the server's hostkeys." — and it concerns host-key rotation, not key types. **What would settle it:** the OpenSSH release notes or `ChangeLog` for the specific versions, or querying `ssh -Q key` on each target machine. The reliable runtime test is `ssh -Q key` on the machine in question, not the manual.
- **Whether any given remote host restricts `PubkeyAcceptedAlgorithms` away from the default.** Per-host configuration; cannot be established from the manual.
- **Whether the installed 10.2p1 build can *use* an `mldsa44-ed25519` key it did not generate.** The usage banner settles only what `-t` will create. Sent back — see below.

---

## Fact 33 — where each of Mac, Windows and Linux keeps its SSH keys, and whether each has exactly one such place

### The convention, documented, and common to all three

`ssh(1)`, FILES:

> `~/.ssh/`
> This directory is the default location for all user-specific configuration and authentication information. There is no general requirement to keep the entire contents of this directory secret, but the recommended permissions are read/write/execute for the user, and not accessible by others.

The key files themselves, same section:

> `~/.ssh/id_ecdsa` `~/.ssh/id_ecdsa_sk` `~/.ssh/id_ed25519` `~/.ssh/id_ed25519_sk` `~/.ssh/id_mldsa44_ed25519` `~/.ssh/id_rsa`
> Contains the private key for authentication. These files contain sensitive data and should be readable by the user but not accessible by others (read/write/execute). ssh will simply ignore a private key file if it is accessible by others.

> `~/.ssh/id_ecdsa.pub` … `~/.ssh/id_rsa.pub`
> Contains the public key for authentication. These files are not sensitive and can (but need not) be readable by anyone.

The word used in the `~/.ssh/` FILES entry is **default**; *only* does not appear in it. (The page uses "only" elsewhere for other purposes — e.g. `/etc/hosts.equiv`, "It should only be writable by root." — so this holds for that entry, not for the manual as a whole.)

### The one-place premise is false as stated

The documentation names a conventional default location and, in the same breath, documents **three mechanisms** by which a usable private key lives somewhere else, plus a fourth point about the path itself. The premise should be read as: `~/.ssh` is the default location an application scans; it is not the only place a usable key can be.

**1. `IdentityFile` names the file the identity is read from, and there can be many of them.** ssh_config(5):

> Specifies a file from which the user's ECDSA, authenticator-hosted ECDSA, Ed25519, authenticator-hosted Ed25519 or RSA authentication identity is read. … The default is `~/.ssh/id_rsa`, `~/.ssh/id_ecdsa`, `~/.ssh/id_ecdsa_sk`, `~/.ssh/id_ed25519`, `~/.ssh/id_ed25519_sk` and `~/.ssh/id_mldsa44_ed25519`. … Arguments to IdentityFile may use the tilde syntax to refer to a user's home directory or the tokens described in the TOKENS section. Alternately an argument of none may be used to indicate no identity files should be loaded.

> It is possible to have multiple identity files specified in configuration files; all these identities will be tried in sequence. Multiple IdentityFile directives will add to the list of identities tried (this behaviour differs from that of other configuration directives).

`ssh(1)` says the same about the command line:

> It is possible to have multiple `-i` options (and multiple identities specified in configuration files).

The multiplicity half is **documented** outright. That a listed key can live *anywhere* is **inferred**: the manual states no restriction on the path, which is not the same as stating there is none. Everything downstream that needs "a listed key can live outside `~/.ssh`" rests on the absence of a stated restriction.

**2. The private key may not exist on disk at all — the agent may hold it.** Documented. `ssh(1)`, `-i`:

> You can also specify a public key file to use the corresponding private key that is loaded in ssh-agent(1) when the private key file is not present locally.

`ssh-agent(1)`:

> ssh-agent is a program to hold private keys used for public key authentication.

**3. The key may live on a hardware token, reached through PKCS#11.** Documented. ssh_config(5):

> `PKCS11Provider` — Specifies which PKCS#11 provider to use or none to indicate that no provider should be used (the default). The argument to this keyword is a path to the PKCS#11 shared library ssh(1) should use to communicate with a PKCS#11 token providing keys for user authentication.

**4. The path is written with a leading tilde, and configuration may also reach the home directory through a token.** ssh_config(5) says arguments to `IdentityFile` "may use the tilde syntax to refer to a user's home directory or the tokens described in the TOKENS section", and TOKENS defines `%d` as:

> Local user's home directory.

**How that home directory is arrived at is not in the documentation.** It is settled here only for macOS, only negatively, by the observation below.

### macOS

The location is the same `~/.ssh`. Apple ships the OpenSSH manuals and its consumer documentation defers to them rather than restating the location.

**Observed** — Apple-shipped `ssh(1)` manual, `MANWIDTH=90 man ssh | col -bx | sed -n '/^FILES/,/^AUTHORS/p'`:

```
     ~/.ssh/
             This directory is the default location for all user-specific configuration
             and authentication information.  There is no general requirement to keep the
             entire contents of this directory secret, but the recommended permissions are
             read/write/execute for the user, and not accessible by others.
...
     ~/.ssh/id_ecdsa
     ~/.ssh/id_ecdsa_sk
     ~/.ssh/id_ed25519
     ~/.ssh/id_ed25519_sk
     ~/.ssh/id_rsa
             Contains the private key for authentication.
```

The Apple-shipped manual lists **five** private-key names, not the six on man.openbsd.org — there is no `id_mldsa44_ed25519` entry.

**Documented (Apple):** Apple's own web documentation for macOS does not state where keys are kept. The relevant Apple Support page covers only turning on the server side, and refers the reader to the manual pages:

> Turn on Remote Login to access your Mac from another computer using SSH (Secure Shell Protocol) or SFTP (SSH File Transfer Protocol).

and, in its See also list:

> man page for ssh

The whole page was searched: no occurrence of `.ssh`, `ssh-keygen` or `key pair`.

**A macOS-specific second place — for the passphrase, not the key.** The key file still lives in `~/.ssh`; what moves into the Keychain is the passphrase.

**Observed** — `MANWIDTH=90 man ssh_config | col -bx`, at `UseKeychain`:

```
     UseKeychain
             On macOS, specifies whether the system should search for passphrases in the
             user's keychain when attempting to use a particular key. When the passphrase
             is provided by the user, this option also specifies whether the passphrase
             should be stored into the keychain once it has been verified to be correct.
             The argument must be "yes" or "no".  The default is "no".  Incompatible with
             PKCS11Provider.
```

**Observed** — `MANWIDTH=90 man ssh-add | col -bx`:

```
     --apple-use-keychain
             When adding identities, each passphrase will also be stored in the user's
             keychain.  When removing identities with -d, each passphrase will be removed
             from it.

     --apple-load-keychain
             Add identities to the agent using any passphrase stored in the user's
             keychain.
```

Neither option is present upstream — the string `keychain` occurs **zero times** in the served ssh_config(5) and zero times in the served ssh-add(1). That Apple authored them is **inferred**, from that absence plus the `apple-` prefix and the macOS-specific wording; no source read says so.

**Observed, and load-bearing** — on macOS the default key path is **not** resolved from `$HOME`. Command: `HOME=<scratch>/fakehome ssh-keygen -t ed25519 -N "" </dev/null`. First two lines:

```
Generating public/private ed25519 key pair.
Enter file in which to save the key (/Users/artist/.ssh/id_ed25519):
```

The offered default was the real account home despite `$HOME` pointing elsewhere. The command exited 1 on end-of-file at the prompt and wrote nothing. **Consequence: do not assume that reading `$HOME` reproduces what `ssh` and `ssh-keygen` will do.** *What is observed is the negative half only.* "Resolved from the password database" names a mechanism the observation cannot distinguish from any other non-`$HOME` source, and was cut as **inferred**. The check did not re-run this command: on end-of-file the offered default could, on a build behaving differently, be accepted and written into the real `~/.ssh`, which the check may not modify.

### Windows — documentation only, not confirmed against a real installation

The location is `%USERPROFILE%\.ssh`. Microsoft shows it three ways — twice as the literal path `C:\Users\username/.ssh/`, once as `$env:USERPROFILE\.ssh`. **Answered 2026-08-21, and the caution turns out to have been right for a better reason.** Microsoft's own page, `https://learn.microsoft.com/en-us/windows/deployment/usmt/usmt-recognized-environment-variables`, page dated 2025-01-29, read 2026-08-20, carries two rows: *USERPROFILE* — *"Same as **CSIDL_PROFILE**."*, and *CSIDL_PROFILE* — *"The user's profile folder. A typical path is `C:\Users\<username>`."* **So Microsoft does equate the variable with the profile folder, and in the same breath calls the literal path *typical* rather than the path.** The two are the same **directory**; the **string** is not guaranteed. **The rule that follows: expand the variable, never hard-code the path.** *(This does not settle the separate open question of which of `$HOME` and `%USERPROFILE%` wins on Windows when they disagree, which still needs a Windows machine.)*

The default offered by `ssh-keygen.exe`:

> Enter file in which to save the key (C:\Users\username/.ssh/id_ecdsa):

and the results:

> Your identification has been saved in C:\Users\username/.ssh/id_ecdsa.
> Your public key has been saved in C:\Users\username/.ssh/id_ecdsa.pub.

and in its `ssh-add` example:

> ssh-add $env:USERPROFILE\.ssh\id_ecdsa

Microsoft's own transcript shows the mixed separator `C:\Users\username/.ssh/id_ecdsa` — backslash to the profile, forward slashes below it. *That path comparison must normalise separators is an application ruling drawn from it, not something the page says.*

The client's per-user configuration is in the same directory:

> In Windows, the OpenSSH Client reads configuration data from %userprofile%\.ssh\config by default.

and, in the list of ways to configure the client:

> Via a user's configuration file at %userprofile%\.ssh\config.
> Via the system-wide configuration file at %programdata%\ssh\ssh_config.

**Windows is explicitly not one place.** Microsoft documents a second, machine-wide location:

> Because there's no user associated with the sshd service, the host keys are stored under C:\ProgramData\ssh.

and for the administrator variant of `authorized_keys`:

> The default is .ssh/authorized_keys. If you don't specify an absolute path, OpenSSH looks for the file relative to your home directory, such as C:\Users\username. If the user belongs to the administrator group, %programdata%/ssh/administrators_authorized_keys is used instead.

`C:\ProgramData\ssh` is where Microsoft says the **host keys** are stored, along with server configuration — not the user's client identities. **Answered 2026-08-21: that page does state it.** Microsoft, *OpenSSH Server Configuration for Windows*, `https://learn.microsoft.com/en-us/windows-server/administration/openssh/openssh-server-configuration`, page dated 2025-08-05, read 2026-08-20, under **HostKey**: *"The defaults are: `#HostKey __PROGRAMDATA__/ssh/ssh_host_rsa_key`, `#HostKey __PROGRAMDATA__/ssh/ssh_host_dsa_key`, `#HostKey __PROGRAMDATA__/ssh/ssh_host_ecdsa_key`, `#HostKey __PROGRAMDATA__/ssh/ssh_host_ed25519_key`"* and *"If the defaults aren't present, sshd automatically generates them on a service start."* **`HostKey` names a private key file — that is what the directive is for — and none of the four paths carries `.pub`.** So the consequence this paragraph feared is real rather than inferred: **any sweep for private keys on a Windows machine will find server host keys under `%programdata%\ssh`**, and they may not even have existed before the service first started.

### Linux — documentation only

The location is `~/.ssh`, per the OpenSSH manuals above, and Canonical states it outright for Ubuntu:

> By default, the public key is saved in the file `~/.ssh/id_<algorithm>.pub`, while `~/.ssh/id_<algorithm>` is the private key.

and, for the per-user configuration file:

> Since you can have multiple SSH-keys, you can configure which to use for which target machine in `~/.ssh/config` (see ssh_config(5)).

Only one distribution's documentation could be read at all, so "no distribution documentation contradicts it" means Canonical's. Distribution-specific **server** configuration lives in a different tree (`/etc/ssh/sshd_config.d/` on Ubuntu, quoted under Fact 39) — a different tree from `~/.ssh`, and the server side; the user's key location is unchanged.

### Unresolved for Fact 33

- **Whether any shipping OS documents an alternative default user key directory.** Nothing found does. The variation documented is by explicit configuration (`IdentityFile`, `-i`, `Include`), by agent, and by PKCS#11 token — not by a competing default directory.
- **Windows: whether `$HOME`/`%USERPROFILE%` or the profile path from the account database wins when they disagree.** The macOS observation shows OpenSSH does not use `$HOME` there; Microsoft's documentation says "your home directory" without defining how it is resolved, and no Microsoft page found settles it. **What would settle it:** running `ssh-keygen -t ed25519` with `$env:USERPROFILE` and `$env:HOME` set to different directories on a real Windows installation and recording the offered default path.
- **Windows: keys held by non-Microsoft SSH implementations** — PuTTY's `.ppk` files, or a separate `~/.ssh` inside a WSL distribution. Microsoft's OpenSSH documentation does not address them; no Microsoft page found rules on whether an application should look. **What would settle it:** a project decision on scope, not a fact — but if such keys must be found, their format and location need their own fact.
- **CLOSED 2026-08-21 at the level that matters, and it is not a macOS question.** **Answered 2026-08-21, and wider than macOS.** `~` and `%d` mean *the local user's home directory* — a property of the account — and the environment variable has a different syntax. `ssh_config(5)` as installed, TOKENS, read 2026-08-20: `%d    Local user's home directory.`; and on the same page, *"Arguments to some keywords can be expanded at runtime from environment variables on the client by enclosing them in `${}`, for example `${HOME}/.ssh` would refer to the user's .ssh directory."* **Run on the Creator's Mac, 2026-08-20**, with `$HOME` set to `/tmp/fakehome`: `ssh -G` still reported `userknownhostsfile /Users/artist/.ssh/known_hosts …`, `-o "UserKnownHostsFile=%d/KNOWNHOSTS"` gave `/Users/artist/KNOWNHOSTS`, and `-o "IdentityAgent=~/AGENTSOCK"` gave `/Users/artist/AGENTSOCK`. A negative would have been `/tmp/fakehome/…` in any of the three. **Nothing in that wording is macOS-specific**, so narrowing this to macOS made a portable behaviour look local — a worse error than the over-claim it was correcting. **What is genuinely not established is the mechanism** — that the resolution goes through `getpwuid` specifically — and that is a claim about how rather than about what, which this document's own wording says only matters *if the mechanism matters*.

### The consequence for deleting a key

Because a listed key can be a file whose path came from `IdentityFile` or `-i` and can live outside `~/.ssh`, and because a listed identity can exist only inside the agent with no file behind it, any "delete key" that unlinks files must (a) know, per key, whether a file backs it, and (b) know the file's real path rather than assuming `~/.ssh/<name>`. **The documentation does not support a rule of the form "the key's file is `~/.ssh/<name>`".**

---

## Fact 34 — what else ordinarily lives in that location, and how a key file is told from anything else there

### What the documentation says lives in `~/.ssh`

From `ssh(1)` FILES, verbatim:

| Path | Verbatim description |
|---|---|
| `~/.ssh/authorized_keys` | "Lists the public keys (ECDSA, Ed25519, RSA) that can be used for logging in as this user." |
| `~/.ssh/config` | "This is the per-user configuration file." |
| `~/.ssh/environment` | "Contains additional definitions for environment variables" |
| `~/.ssh/known_hosts` | "Contains a list of host keys for all hosts the user has logged into that are not already in the systemwide list of known host keys." |
| `~/.ssh/rc` | "Commands in this file are executed by ssh when the user logs in, just before the user's shell (or command) is started." |

Four more classes of resident, documented elsewhere in the manuals:

**Certificates.** `ssh(1)`, `-i`:

> If no certificates have been explicitly specified by the CertificateFile directive, ssh will also try to load certificate information from the filename obtained by appending `-cert.pub` to identity filenames.

**Arbitrary included configuration fragments.** `ssh_config(5)`, `Include`:

> Include the specified configuration file(s). Multiple pathnames may be specified and each pathname may contain glob(7) wildcards … Files without absolute paths are assumed to be in `~/.ssh` if included in a user configuration file or `/etc/ssh` if included from the system configuration file.

A file with any relative pathname named in a user `Include` therefore resolves inside `~/.ssh`, so files with arbitrary names can be configuration fragments there. That *no* filename is excluded is **inferred** — the page states no constraint on the name, which is not the same as stating there is none. The consequence drawn below — that no closed list of `~/.ssh` residents exists — survives on the narrower reading.

**Agent sockets.** `ssh-agent(1)`, `-a`:

> Bind the agent to the Unix-domain socket bind_address. The default is to create a socket at a random path matching `$HOME/.ssh/agent/s.*`.

So where an agent has been started and has taken that default, `~/.ssh` contains a **subdirectory** (`agent/`) holding **sockets**, not regular files. *The quoted sentence describes the default `ssh-agent` takes when run with no `-a`, not what the directory ordinarily holds.* Confirmed present in the Apple-shipped manual (**observed**, `MANWIDTH=90 man ssh-agent | col -bx`):

```
     -a bind_address
             Bind the agent to the UNIX-domain socket bind_address.  The default is to
             create a socket at a random path matching $HOME/.ssh/agent/s.*.
```

**A second authorized-keys file.** `sshd(8)`:

> if this option is not specified, the default is `~/.ssh/authorized_keys` and `~/.ssh/authorized_keys2`.

**Windows additions.** Microsoft documents `config` at `%userprofile%\.ssh\config`, and gives `authorized_keys` a **home-relative** default rather than a `%USERPROFILE%` one: "The default is .ssh/authorized_keys. If you don't specify an absolute path, OpenSSH looks for the file relative to your home directory, such as C:\Users\username." Microsoft's naming rule for the pair:

> The .pub file is the public key, and the file without an extension is the private key

That rule is Microsoft's, and it is **weaker than it looks**: `config`, `environment`, `rc`, `known_hosts` and `authorized_keys` also have no extension. **It is not usable as a discriminator.**

### How a key file is told from anything else — the formats

**Public key files and `authorized_keys` share one line format.** `sshd(8)`, AUTHORIZED_KEYS FILE FORMAT:

> Each line of the file contains one key (empty lines and lines starting with a '#' are ignored as comments). Public keys consist of the following space-separated fields: options, keytype, base64-encoded key, comment. The options field is optional. The supported key types are: `sk-ecdsa-sha2-nistp256@openssh.com` `ecdsa-sha2-nistp256` `ecdsa-sha2-nistp384` `ecdsa-sha2-nistp521` `sk-ssh-ed25519@openssh.com` `ssh-ed25519` `ssh-mldsa44-ed25519@openssh.com` `ssh-rsa`

> The comment field is not used for anything (but may be convenient for the user to identify the key).

**`known_hosts` uses a closely related format.** `sshd(8)`, SSH_KNOWN_HOSTS FILE FORMAT:

> Each line in these files contains the following fields: marker (optional), hostnames, keytype, base64-encoded key, comment. The fields are separated by spaces.

**This is the trap.** Side by side as `sshd(8)` gives them: `authorized_keys` is `options (optional), keytype, base64-encoded key, comment`; `known_hosts` is `marker (optional), hostnames, keytype, base64-encoded key, comment`. Both are space-separated fields around a keytype token and a base64 blob; `known_hosts` adds the mandatory `hostnames` field, and its optional leading field is a `marker`, not `options` — the optional leading field differs in kind as well as position. **Inferred** from that: any parser that scans for a known keytype token and a base64 blob will read `known_hosts` **and** `authorized_keys` as key files. Confirmed by observation for `known_hosts` below; **not run for `authorized_keys`**, and no manual says it.

**Private key files.** `ssh-keygen(1)`:

> ssh-keygen will by default write keys in an OpenSSH-specific format. This format is preferred as it offers better protection for keys at rest as well as allowing storage of key comments within the private key file itself.

> It is still possible for ssh-keygen to write the previously-used PEM format private keys using the `-m` flag.

The manuals do not print the file's header line (`BEGIN OPENSSH` occurs zero times in the served ssh-keygen(1), ssh(1) and sshd(8)). **Observed**, on a throwaway key in a scratch directory since deleted:

```
=== private first line ===
-----BEGIN OPENSSH PRIVATE KEY-----
=== private last line ===
-----END OPENSSH PRIVATE KEY-----
```

and the matching public file, in full:

```
ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIJ3gVKMADI8jAxk88EDy1e4aHy7fw/65f4i+D5etngaj smoozy-test@example
```

### Observed — the discriminator: ask `ssh-keygen`, do not parse

macOS 26.5.2, `OpenSSH_10.2p1`. `ssh-keygen -l -f <path>` against a scratch directory containing a `config`, a one-line `known_hosts`, a plain text file, an empty file, and a key pair. Exact outputs:

```
### config : exit=255
<path>/config is not a public key file.
### known_hosts : exit=0
256 SHA256:+DiY3wvvV6TuJJhbpZisF/zLDA0zPMSvHdkr4UvCOqU github.com (ED25519)
### notes.txt : exit=255
<path>/notes.txt is not a public key file.
### empty : exit=255
<path>/empty is not a public key file.
### k1.pub : exit=0
256 SHA256:eu60QIDeNFacTTTHcbXJt4Z1HntBdYupFsJE65kv1wU smoozy-test@example (ED25519)
### k1 : exit=0
256 SHA256:eu60QIDeNFacTTTHcbXJt4Z1HntBdYupFsJE65kv1wU smoozy-test@example (ED25519)
```

Every exit status and every message reproduced exactly on re-run. **One step is missing from the record and was sent back:** how the `known_hosts` fixture was built. The line recorded is `github.com`'s real ED25519 host key, and the findings state that nothing under the user's `~/.ssh` was read, so its provenance is unstated. The check's re-run used a `known_hosts` line built from a throwaway key pair generated in the scratch directory, and `ssh-keygen -l -f` on it returned exit 0 in the same way.

What this establishes:

1. **`ssh-keygen -l -f` is a sound negative test.** Non-key files fail with exit 255 and `… is not a public key file.` — no false positives on `config`, plain text, or an empty file.
2. **It is not a sound positive test on its own: `known_hosts` passes.** `-l` fingerprints the host keys inside it and exits 0. Because `known_hosts` ordinarily lives in the same directory, a scan treating "exit 0" as "this is a key" will present the user's `known_hosts` as a key. That `authorized_keys` behaves the same way is **inferred** from the shared line format — it was not run, and no manual says it. **Both filenames must be excluded by name before fingerprinting.**
3. An unencrypted private key with no `.pub` sibling still fingerprints correctly.

**Encrypted private keys: fingerprint works, comment does not.** **Observed** — a key generated with a passphrase, its `.pub` sibling moved aside:

```
### -l on encrypted private key, no .pub sibling:
exit=0
256 SHA256:BSMD8QZ/CPVgUL1WkISG0CbOMdza7mxDSi97tXCC3zQ no comment (ED25519)
```

The real comment was `enc@example`. A public key file with a genuinely empty comment field reports the same string:

```
### -l on public key with no comment:
exit=0
256 SHA256:eu60QIDeNFacTTTHcbXJt4Z1HntBdYupFsJE65kv1wU no comment (ED25519)
```

So **`no comment` is ambiguous** between "encrypted, comment not readable" and "there is no comment". Both results reproduced exactly. How the comment-less public key file was produced is missing from the record and was sent back; the check's re-run produced one by keeping the first two space-separated fields of the generated `.pub` file, with the fingerprint unchanged.

> **Carried over from Fact 35, and it corrects this file.** The findings for Fact 34 said the comment "is stored inside the encrypted portion". The check on the Fact 35 material struck that same claim there: the observation shows only that the comment is *not recoverable* from an encrypted private key file without the passphrase, not *where in the file it sits*. ssh-keygen(1) DESCRIPTION does document "allowing storage of key comments within the private key file itself", which places the comment in the file; neither it nor the observation places it inside the encrypted portion. **The narrower reading is the one that stands: not recoverable, location unestablished.**

**Detecting whether a private key has a passphrase, without asking the user.** **Observed**:

```
### -y -P '' on UNENCRYPTED key k1:
exit=0
ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIJ3gVKMADI8jAxk88EDy1e4aHy7fw/65f4i+D5etngaj smoozy-test@example
### -y -P '' on ENCRYPTED key k2:
exit=255
Load key "<path>/k2": incorrect passphrase supplied to decrypt private key
```

Both reproduced exactly. **The Fact 34 findings read this as a general rule** — exit 0 means no passphrase, exit 255 with that message means passphrase-protected. **The check on the Fact 35 material narrowed exactly that generalisation** to the two keys run there, since exit 255 also arises from other causes, and sent the question back. Fact 37's route-A table (below) bears directly on it: on that build, `ssh-keygen -y` returns **exit 255** for a malformed file (`invalid format`), a missing file, and a key file whose permissions are too open (`bad permissions`), as well as for a wrong passphrase. **So the exit status alone does not carry the answer; the message text does the work, and the message text is undocumented.**

**A genuinely broken key reads differently from a non-key.** **Observed** — a truncated file still carrying the OpenSSH private-key header:

```
### -l on TRUNCATED/corrupt private key:
exit=255
@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@
@         WARNING: UNPROTECTED PRIVATE KEY FILE!          @
@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@
Permissions 0644 for '<path>/truncated_key' are too open.
It is required that your private key files are NOT accessible by others.
This private key will be ignored.
<path>/truncated_key is not a key file.
```

All seven lines and the exit status reproduced exactly. Two things to note. First, the failure message is `is not a key file.` — **not** `is not a public key file.` — so the wording differs depending on whether the file looked like a private key. Second, a permissions warning fires *before* the parse result, and this is a real condition an application will meet, since `ssh(1)` states it will "simply ignore a private key file if it is accessible by others". **Two steps are missing from the record and were sent back:** how the file was truncated, and at what mode. The mode is load-bearing — the banner and the `Permissions 0644 … are too open.` line appear only because the file was `0644`, which the record never states. The re-run wrote the first three lines of a generated private key to a new file under the default umask, which yielded `0644`.

### What the fact is really asking

A file that was never a key will **not** be presented as a broken key, provided the application:

1. skips the names the documentation reserves for non-keys — `config`, `known_hosts`, `environment` and `rc` (`ssh(1)` FILES), `authorized_keys` and `authorized_keys2` (`sshd(8)`), and the `agent/` subdirectory (`ssh-agent(1)`); and `known_hosts.old` as well, **which is now observed rather than assumed** — the tool prints the name in full on standard output (2026-08-21 entry below);
2. skips anything that is not a regular file (the agent sockets);
3. treats `ssh-keygen -l -f` exit 255 with `is not a public key file.` / `is not a key file.` as "not a key, say nothing" rather than "broken key";
4. reserves the "broken key" presentation for files that *did* look like keys — a file carrying the `-----BEGIN OPENSSH PRIVATE KEY-----` header, or a `.pub` sibling of a private key, that still fails to parse.

### Unresolved for Fact 34

- **ANSWERED 2026-08-21 by running it on a scratch file.** `ssh-keygen(1)`'s `-R` still does not document a backup file name, which is why this was open — **but the tool announces it.** Run on the Creator's Mac, 2026-08-20, on a `known_hosts` built from a throwaway key in a scratch directory, with the Creator's own `~/.ssh` never opened: `ssh-keygen -R testhost.invalid -f <scratch>/known_hosts` printed *"# Host testhost.invalid found: line 1"*, *"<scratch>/known_hosts updated."*, *"Original contents retained as <scratch>/known_hosts.old"*, exit 0 — and afterwards `known_hosts.old` held both original lines while `known_hosts` held only the survivor. Label: **observed**, not documented. **The exclusion of `known_hosts.old` from the key-file discriminator now rests on an observation rather than on practice.** *(The original reason for not running it — avoiding any operation touching a real `known_hosts` — never applied to a scratch file.)* **What is still not documented is the name**, so a future OpenSSH could change it and nothing would announce that in advance. *(This item ended **What would settle it:** running `ssh-keygen -R <host> -f <copy of a known_hosts file>` in a scratch directory and recording what files exist afterwards — *"Not run, to avoid any operation touching a real `known_hosts`"* — until 2026-08-22. That is exactly the run recorded four sentences above, made on 2026-08-20 on a scratch file, which is why the item is answered.)*
- **The exhaustive list of what may appear in `~/.ssh`.** Because `Include` puts arbitrary filenames there by design, no closed list exists. Tolerate unknown filenames; do not enumerate known ones.
- **Whether the two failure messages (`is not a public key file.` / `is not a key file.`) are stable across OpenSSH versions and platforms.** Observed on OpenSSH 10.2p1 on macOS only; in no manual. **What would settle it:** the same probes on the Windows and Linux OpenSSH builds targeted. Until then, do not match on message text — match on exit status, and use the text only for logging. *(Note the tension with the point above: on this build the exit status alone does not separate the causes, and the text is the only thing that does.)*
- **Whether `authorized_keys` fingerprints at exit 0 the way `known_hosts` does.** Asserted by analogy; never run.

---

## Fact 35 — what can be read out of a private key file without its passphrase, and whether a connection attempt can be made that cannot fall back to asking for one

**This fact has two halves and they answer differently.** Half A is answered well. Half B is not answered at all in its decisive part.

### Half A — what is readable without the passphrase

#### Documented

> "Show fingerprint of specified public key file. ssh-keygen will try to find the matching public key file and prints its fingerprint. If combined with -v, a visual ASCII art representation of the key is supplied with the fingerprint."
> — ssh-keygen(1), `-l`

> "This option will read a private OpenSSH format file and print an OpenSSH public key to stdout."
> — ssh-keygen(1), `-y`

> "Show the bubblebabble digest of specified private or public key file."
> — ssh-keygen(1), `-B`

> "ssh-keygen will by default write keys in an OpenSSH-specific format. This format is preferred as it offers better protection for keys at rest as well as allowing storage of key comments within the private key file itself. The key comment may be useful to help identify the key. The comment is initialized to “user@host” when the key is created, but can be changed using the -c option."
> — ssh-keygen(1), DESCRIPTION

> "Provides the (old) passphrase."
> — ssh-keygen(1), `-P passphrase`

> "Specifies the cipher to use for encryption when writing an OpenSSH-format private key file. The list of available ciphers may be obtained using "ssh -Q cipher". The default is “aes256-ctr”."
> — ssh-keygen(1), `-Z cipher`

> "When saving a private key, this option specifies the number of KDF (key derivation function, currently bcrypt_pbkdf(3)) rounds used. Higher numbers result in slower passphrase verification and increased resistance to brute-force password cracking (should the keys be stolen). The default is 16 rounds."
> — ssh-keygen(1), `-a rounds`

**A contradiction inside the manual, worth flagging because it will mislead anyone who reads only the FILES section.** ssh-keygen(1) FILES says the passphrase "will be used to encrypt the private part of this file using 128-bit AES"; ssh(1) FILES says "using AES-128"; ssh-keygen(1) `-Z` says the default cipher is `aes256-ctr` — which is what was observed. All three located verbatim. Treat `-Z` as the accurate statement. *(That instruction is a judgement, not something the manual says.)*

#### Observed — OpenSSH 10.2p1, 2026-08-16

Setup: `ssh-keygen -q -t ed25519 -N 'hunter2hunter2' -C 'locked@local' -f <tmp>/locked`

**Type, bit size and fingerprint are readable from an encrypted private key file with no passphrase, and remain readable when the `.pub` file does not exist:**

```
$ ssh-keygen -l -f <tmp>/locked
256 SHA256:xf2aWlFVtkFboUcrRZJSx4714aLzi+CUehbueUqD22Y locked@local (ED25519)

$ mv <tmp>/locked.pub <tmp>/locked.pub.hidden
$ ssh-keygen -l -f <tmp>/locked
256 SHA256:xf2aWlFVtkFboUcrRZJSx4714aLzi+CUehbueUqD22Y no comment (ED25519)
```

Two things. First, the second invocation succeeds with no `.pub` present and with no prompt — so the type, size and fingerprint come out of the private key file itself, not out of a sidecar. Second, the **comment does not**: `locked@local` while the `.pub` existed, `no comment` once it was moved aside. **So the comment is not recoverable from the encrypted private key file without the passphrase, whereas the public key material is.** *Where in the file the comment sits is not established — see the note under Fact 34.*

This **exceeds what ssh-keygen(1) `-l` claims** ("will try to find the matching public key file"). The documented text describes only the sidecar path; the observed behaviour also reads the public half out of the private file.

**Whether the file is passphrase-protected at all** can be determined without a prompt by supplying an empty passphrase explicitly:

```
$ ssh-keygen -y -P '' -f <tmp>/locked
Load key "<tmp>/locked": incorrect passphrase supplied to decrypt private key
exit=255

$ ssh-keygen -y -P '' -f <tmp>/default
ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIMgZNDXp5dVkf0of6hWTqZkH...
exit=0
```

**On the two keys tested**, exit 0 came from the key without a passphrase (and yielded the public key as a side effect); exit 255 with `incorrect passphrase supplied to decrypt private key` came from the key with one. *This was written as a general rule and narrowed by the check: two keys were tested. Whether the mapping holds generally is not established — and Fact 37's route-A table shows exit 255 arising from at least three other causes on the same build.*

Without `-P ''` the same command prompts even with stdin closed and no TTY:

```
$ ssh-keygen -y -f <tmp>/locked </dev/null
Enter passphrase for "<tmp>/locked": Load key "<tmp>/locked": incorrect passphrase supplied to decrypt private key
exit=255
```

**The same determination can be made by parsing the file**, without invoking `ssh-keygen` at all. Strip the `-----BEGIN/END OPENSSH PRIVATE KEY-----` armour lines and base64-decode the remainder: the container begins with the literal `openssh-key-v1\0`, followed by a length-prefixed (4-byte big-endian) cipher name and a length-prefixed KDF name. Reading those two fields on the four throwaway keys gave:

```
default      ('none', 'none')
locked       ('aes256-ctr', 'bcrypt')
plain_rsa    ('none', 'none')
plain_ecdsa  ('none', 'none')
```

Of the four, only `locked` had a passphrase, and only `locked` gave anything other than `none`/`none`. *This was written as a rule ("anything else means passphrase-protected") and narrowed: only one encrypted key was in the set, so "anything else" rests on a single data point.* This container layout is **not documented** on any of the six manual pages — string search for `openssh-key-v1` returns count zero in each. It is an observation of the file bytes on 10.2p1.

> **A gap in the record, sent back.** The keys `plain_rsa`, `plain_ecdsa` and `<tmp>/ca` are used in observations here and in Facts 36 and 76, but their `ssh-keygen` invocations are recorded nowhere. What the recorded output does fix: `plain_rsa` is a 3072-bit RSA key with comment `rsa@local`, `plain_ecdsa` a 256-bit ECDSA key with comment `ec@local`. **The type of `<tmp>/ca` is not recoverable from anything written down.**

#### Unresolved — Half A

- **The manual does not state that the public half of a key is stored unencrypted inside an OpenSSH-format private key file.** The observed behaviour of `ssh-keygen -l` on a `.pub`-less encrypted key demonstrates it on 10.2p1, but the manual does not promise it, and it is not promised for the legacy PEM format (`-m PEM`), which was not tested. **What would settle it:** the OpenSSH source (`sshkey.c` / `PROTOCOL.key`), or repeating the `-l` observation on each targeted OpenSSH version and key format.
- **The manual documents no command whose stated purpose is "report whether this key has a passphrase".** The `-P ''` exit-status technique is observed behaviour, not documented contract.

### Half B — whether a connection attempt can be made that cannot fall back to asking for a passphrase

#### Documented

The one documented switch is `BatchMode`:

> "If set to yes, user interaction such as password prompts and host key confirmation requests will be disabled. This option is useful in scripts and other batch jobs where no user is present to interact with ssh(1). The argument must be yes or no (the default)."
> — ssh_config(5), `BatchMode`

`BatchMode` is mentioned **nowhere else** in ssh(1), ssh_config(5), sshd(8), sshd_config(5), ssh-keygen(1) or ssh-agent(1) as served on 2026-08-16 — one hit across the six pages, the entry above. So no other entry can qualify it.

On where a passphrase is read from:

> "If ssh needs a passphrase, it will read the passphrase from the current terminal if it was run from a terminal. If ssh does not have a terminal associated with it but DISPLAY and SSH_ASKPASS are set, it will execute the program specified by SSH_ASKPASS and open an X11 window to read the passphrase. This is particularly useful when calling ssh from a .xsession or related script. (Note that on some machines it may be necessary to redirect the input from /dev/null to make this work.)"
> — ssh(1), ENVIRONMENT, `SSH_ASKPASS`

> "Allows further control over the use of an askpass program. If this variable is set to “never” then ssh will never attempt to use one. If it is set to “prefer”, then ssh will prefer to use the askpass program instead of the TTY when requesting passwords. Finally, if the variable is set to “force”, then the askpass program will be used for all passphrase input regardless of whether DISPLAY is set."
> — ssh(1), ENVIRONMENT, `SSH_ASKPASS_REQUIRE`

**Documented, positively:** `SSH_ASKPASS_REQUIRE=never` closes off the askpass-program route entirely — "If this variable is set to “never” then ssh will never attempt to use one."

**Inferred, not documented:** that the terminal route applies *only* when ssh was run from a terminal. The manual states the conditional — ssh "will read the passphrase from the current terminal if it was run from a terminal" — and separately describes the askpass path for when ssh "does not have a terminal associated with it". Converting that pair into an exclusive "only" is a step the reader supplies. *(The findings stated this as something the manual "establishes, positively"; the check relabelled the second half inferred.)*

#### Half B — the documentation gap stands; the behaviour was settled by a run on 2026-08-20

**The manual does not state that `BatchMode=yes` suppresses the private-key passphrase prompt.** Its wording enumerates "password prompts and host key confirmation requests". A private-key passphrase is neither a password nor a host key confirmation; "user interaction such as" is open-ended but names neither. **The manual therefore does not settle whether a `BatchMode=yes` connection using an encrypted key fails outright or prompts.** This is the findings file's own most valuable result, and it holds.

This was **not** verified by observation either, deliberately: verifying it requires an SSH server to authenticate against, and producing one would have meant starting a listening `sshd`, outside the throwaway-key-files scope of that task.

**What would settle it, in order of decisiveness:**

1. The OpenSSH source: whether `read_passphrase()` is called with the `RP_ALLOW_STDIN` / batch-mode suppression flag on the private-key load path (`sshconnect2.c` / `readpass.c`).
2. A direct test against a real SSH server: `ssh -o BatchMode=yes -o PreferredAuthentications=publickey -i <encrypted key> user@host` from a non-TTY context with `SSH_ASKPASS_REQUIRE=never` set, observing whether it prompts or exits non-zero.

**A belt-and-braces construction — inferred, not documented.** Even without settling point 1, the two passphrase-input routes the manual describes can both be closed: run ssh with no controlling terminal (so the "read the passphrase from the current terminal if it was run from a terminal" path is unavailable), with `SSH_ASKPASS_REQUIRE=never` (so, quoting ssh(1), "ssh will never attempt to use one"), and with `BatchMode=yes`. **This is an argument assembled from two documented statements about *where* passphrases are read from. The manual does not itself state the conclusion; it does not state that these two are the *only* routes; and it does not state what ssh does when it needs a passphrase and neither route is available.** *(The findings headed this "documented to hold regardless" and asserted "no interactive prompt can succeed"; both were relabelled inferred and the exhaustiveness disclaimed.)*

**Route B in Fact 37 bears on this and does not close it.** Its default-level table records `encrypted key, BatchMode=yes, no passphrase source` → `Permission denied (publickey).`, exit 255 — that is, on that build and against that throwaway server, the connection failed rather than prompting. That run was against a server the check could not repeat, and it establishes what happened once, not what `BatchMode` guarantees.

**Corrected 2026-08-22. This heading read *Unresolved — Half B, and this is the important gap* and this paragraph ended *The question stays open*, and both went stale on 2026-08-20.** The behaviour was settled that day by the second of the two tests this entry itself named: on the Creator's own Mac, against a throwaway server on the loopback address, **with a terminal deliberately attached** — because with no terminal the test proves nothing — an encrypted key with batch mode on produced **no prompt at all** and refused at exit 255, against two controls that both prompted or authenticated as expected. A negative would have been the prompt appearing exactly as in the control. **What stays open is the documentation half only**: `ssh_config(5)` still names only *"password prompts and host key confirmation requests"*, and a passphrase is neither — so the construction is observed on one build rather than guaranteed. `Facts.md` Fact 35 is the answer of record and `Security and licence` §4 is where the label came off.

---

## Fact 36 — how the public file and a certificate file are named relative to a private key file

### Documented

**Public key file: the private key path with `.pub` appended.**

> "Normally this program generates the key and asks for a file in which to store the private key. The public key is stored in a file with the same name but “.pub” appended."
> — ssh-keygen(1), DESCRIPTION

The FILES section pairs them explicitly: private `~/.ssh/id_ecdsa`, `~/.ssh/id_ecdsa_sk`, `~/.ssh/id_ed25519`, `~/.ssh/id_ed25519_sk`, `~/.ssh/id_mldsa44_ed25519`, `~/.ssh/id_rsa`; public `~/.ssh/id_ecdsa.pub`, `~/.ssh/id_ecdsa_sk.pub`, `~/.ssh/id_ed25519.pub`, `~/.ssh/id_ed25519_sk.pub`, `~/.ssh/id_mldsa44_ed25519.pub`, `~/.ssh/id_rsa.pub`.

**Certificate file: `-cert.pub` appended to the private key path.** Stated three times, in three places:

> "If no certificates have been explicitly specified by CertificateFile, ssh(1) will try to load certificate information from the filename obtained by appending -cert.pub to the path of a specified IdentityFile."
> — ssh_config(5), `IdentityFile`

> "If no certificates have been explicitly specified by the CertificateFile directive, ssh will also try to load certificate information from the filename obtained by appending -cert.pub to identity filenames."
> — ssh(1), `-i identity_file`

> "The resultant certificate will be placed in /path/to/user_key-cert.pub."
> and
> "The host certificate will be output to /path/to/host_key-cert.pub."
> — ssh-keygen(1), CERTIFICATES, following the examples `ssh-keygen -s /path/to/ca_key -I id -n user /path/to/user_key.pub` and `ssh-keygen -s /path/to/ca_key -I id -h -n foo.example.org /path/to/host_key.pub`

In those CERTIFICATES examples the **input** is the `.pub` file (`user_key.pub`) and the **output drops that `.pub` before appending**: `user_key-cert.pub`, not `user_key.pub-cert.pub`. **So relative to a private key at path `P`: public is `P.pub`, certificate is `P-cert.pub`.**

The convention is overridable on the client side:

> "Specifies a file from which the user's certificate is read. A corresponding private key must be provided separately in order to use this certificate either from an IdentityFile directive or -i flag to ssh(1), via ssh-agent(1), or via a PKCS11Provider or SecurityKeyProvider."
> — ssh_config(5), `CertificateFile`

### Observed — OpenSSH 10.2p1, 2026-08-16

```
$ ssh-keygen -s <tmp>/ca -I testid -n testuser <tmp>/plain_rsa.pub
Signed user key <tmp>/plain_rsa-cert.pub: id "testid" serial 0 for testuser valid forever
$ ls <tmp> | grep cert
plain_rsa-cert.pub
```

Confirms the `.pub` → `-cert.pub` transformation on the installed version; repeated by the check with the same result. **The generation of `<tmp>/ca` is recorded nowhere**, and its key type is not recoverable — see the gap noted under Fact 35 Half A.

### Unresolved for Fact 36

- **What happens when the private key file has a suffix of its own** (e.g. `mykey.pem`) — whether the derived names are `mykey.pem.pub` / `mykey.pem-cert.pub` or something else. The documented rule is plain string appending to the private key path, which implies the former, but no example in the manual exercises it. **What would settle it:** an observation on a key whose filename carries an extension, or the OpenSSH source.
- **Whether the `.pub` file is required to be present for authentication when the private key is present.** The `-i` text addresses only the converse case: a public key file may be specified "to use the corresponding private key that is loaded in ssh-agent(1) when the private key file is not present locally."

---

## Fact 37 — whether a wrong passphrase can be told apart from every other reason a key could not be used or a connection refused

**Two routes, answered separately and differently.** Route **A** is the local private-key load/decrypt with a supplied passphrase, no connection. Route **B** is a connection where the key requires a passphrase. Route A was fully re-run by the check on the same machine and matched exactly. **Route B, the `ssh-add` route, and the rate-limit observation could not be repeated at all** — they need an agent, a listening socket and a server — and are judged from their descriptions alone. That distinction runs through everything below.

### Documented — exit statuses

`ssh(1)` — EXIT STATUS, in full:

> "ssh exits with the exit status of the remote command or with 255 if an error occurred."

That is the **entire** documented exit-status contract for the client. There is no documented mapping from failure cause to exit code: the page names one value, 255, for "an error". The six causes that matter here — wrong passphrase, unreadable key, rejected key, refused TCP connection, changed host key, DNS failure — were each **observed** to give 255; the page itself names none of them.

`ssh-add(1)` — EXIT STATUS, in full:

> "Exit status is 0 on success, 1 if the specified command fails, and 2 if ssh-add is unable to contact the authentication agent."

Only one cause gets its own code: **agent unreachable (2)**. The page describes 1 only as "the specified command fails"; it says nothing about passphrases or malformed key files, so what either produces is **undocumented**. *(The findings said "Everything else, including a wrong passphrase and a malformed key file, is 1" — not carried by the page, and contradicted by the file's own observation that a wrong passphrase produced no exit status at all.)*

`ssh-keygen(1)` — **has no EXIT STATUS section at all.** The only exit-status statements in the page are scoped to three unrelated operations. For `-Y check-novalidate`:

> "Successful testing of the signature is signalled by ssh-keygen returning a zero exit status."

For `-Y verify`:

> "Successful verification by an authorized signer is signalled by ssh-keygen returning a zero exit status."

And for `-Q` against a KRL:

> "ssh-keygen will exit with a non-zero exit status. A zero exit status will only be returned if no key was revoked."

**Nothing in `ssh-keygen(1)` documents what it returns when a key fails to decrypt.**

### Documented — error text, and the absence of it

**None of the diagnostic strings relied on below appears in any of the seven manual pages searched.** Verified by searching the full text of `ssh(1)`, `ssh-keygen(1)`, `ssh-add(1)`, `ssh-agent(1)`, `ssh_config(5)`, `sshd(8)` and `sshd_config(5)` for each of: `incorrect passphrase`, `Permission denied`, `invalid format`, `Host key verification`, `Connection refused`, `Could not resolve` — and, added by the check, `bad permissions`, `too open`, `Connection closed by`, `Enter passphrase for`, `no pubkey loaded`, `bad passphrase given`, `Operation timed out`, `Identity added`, `Error connecting to agent`, `sign_and_send_pubkey`, `Offering public key`, `Authentications that can continue`. **Zero hits in every page for every one of them.**

Two qualifications. The one `bad passphrase` hit in `ssh-keygen(1)` is prose advising against weak passphrases, not an error message. One near-miss: the token `kex_exchange_identification` **does** appear in `ssh_config(5)` and `sshd_config(5)`, but as a source-function name inside the `LogVerbose` example, not as a documented message.

**Consequence: every error string relied on below is undocumented and so carries no documented stability guarantee across versions.** They are implementation output, not interface. *(Absence of documentation is absence of a guarantee, not a promise of change — the findings' "unstable across versions by contract" was softened to this.)*

### Documented — what the manual does say about diagnosing authentication

`ssh(1)`:

> "The server may inform the client of errors that prevented public key authentication from succeeding after authentication completes using a different method. These may be viewed by increasing the LogLevel to DEBUG or higher (e.g. by using the -v flag)."

`ssh(1)`, on `-v`:

> "Verbose mode. Causes ssh to print debugging messages about its progress. This is helpful in debugging connection, authentication, and configuration problems. Multiple -v options increase the verbosity. The maximum is 3."

`ssh_config(5)`, on `LogLevel`:

> "Gives the verbosity level that is used when logging messages from ssh(1). The possible values are: QUIET, FATAL, ERROR, INFO, VERBOSE, DEBUG, DEBUG1, DEBUG2, and DEBUG3. The default is INFO. DEBUG and DEBUG1 are equivalent. DEBUG2 and DEBUG3 each specify higher levels of verbose output."

So the one diagnostic route the documentation points at is **raising the log level and reading prose** — and the `ssh(1)` sentence scopes even that narrowly, to errors the *server* supplies, viewable only after authentication completes by some other method. Neither an exit code nor a structured error is offered anywhere. *(The findings called this "the documented mechanism for telling authentication failures apart", which is wider than the sentence it rests on.)*

### Documented — host key change

`ssh(1)`:

> "If a host's identification ever changes, ssh warns about this and disables password authentication to prevent server spoofing or man-in-the-middle attacks, which could otherwise be used to circumvent the encryption. The StrictHostKeyChecking option can be used to control logins to machines whose host key is not known or has changed."

`ssh_config(5)`, on `StrictHostKeyChecking`:

> "If this flag is set to ask (the default), new host keys will be added to the user known host files only after the user has confirmed that is what they really want to do, and ssh will refuse to connect to hosts whose host key has changed. The host keys of known hosts will be verified automatically in all cases."

A changed host key is documented as producing a *warning* and a refusal. **The warning's text is not documented** — `IDENTIFICATION HAS CHANGED` and `Host key verification` both return zero hits across all seven pages.

### Documented — passphrase input and retry count

`ssh-add(1)`:

> "If any file requires a passphrase, ssh-add asks for the passphrase from the user. The passphrase is read from the user's tty. ssh-add retries the last passphrase if multiple identity files are given."

`ssh_config(5)`, `NumberOfPasswordPrompts`:

> "Specifies the number of password prompts before giving up. The argument to this keyword must be an integer. The default is 3."

Note the entry says *password* prompts and says nothing about private-key passphrase prompts. `BatchMode` and the two `SSH_ASKPASS*` entries are quoted under Fact 35 Half B. `ssh-keygen(1)` supplies the one non-interactive passphrase input, `-P passphrase` — "Provides the (old) passphrase." — and the operation used as the load/decrypt probe throughout route A, `-y` — "This option will read a private OpenSSH format file and print an OpenSSH public key to stdout."

**Documentation alone does not settle Fact 37.** It establishes that no mapping from failure cause to exit code is documented for `ssh` or `ssh-keygen` (`ssh-add` documents exactly one such code, 2, for an unreachable agent), and it points at raised log levels as the diagnostic route it knows about, but **it never states what that output contains for any failure cause.**

### Observed — the environment

- `ssh -V` → `OpenSSH_10.2p1, LibreSSL 3.3.6`; macOS 26.5.2 build 25F84, arm64.
- Server side: the same install's `/usr/sbin/sshd`, run as an unprivileged throwaway instance bound to `127.0.0.1:22022` with its own host key, config and `authorized_keys` inside a temporary directory. No system service was enabled or altered; no host belonging to the user or a third party was contacted. **The `sshd` command line, the `sshd_config` contents beyond the three keywords named below, the host key type, the account name and how `authorized_keys` was populated are all unrecorded** — section 2.3 cannot be repeated from its description without inventing the server.
- All keys were throwaways in a temporary directory; `HOME` was redirected and `-F /dev/null` passed for every `ssh`/`ssh-add` invocation, so the user's `~/.ssh` was never read, listed, written or printed. *How the user's running agent was queried to verify the throwaway fingerprint's absence is not recorded.*
- Passphrases were fed non-interactively through a throwaway `SSH_ASKPASS` helper with `SSH_ASKPASS_REQUIRE=force`, or through `ssh-keygen -P`.
- Key set: `k_enc` (Ed25519, passphrase `correct horse`), `k_plain` (Ed25519, no passphrase), `k_pem` (RSA-2048 in legacy PEM format, passphrase `correct horse`), plus deliberately damaged files. **What the damage was is unrecorded**; the check invented recipes that each reproduced its row exactly, and they are recorded as one set that works — not necessarily the writer's own: `k_garbage` = 200 bytes from `/dev/urandom`; `k_trunc` = the first 120 bytes of `k_enc`; `k_corrupt` = `k_enc` with the first character of the first body line replaced by `@`; `k_badb64` = the line `not base64 at all !!!` between valid BEGIN/END OPENSSH PRIVATE KEY lines; `k_empty` = a zero-length file; `k_pubonly` = a copy of `k_enc.pub`.

### Observed — route A, local load/decrypt (`ssh-keygen -y`)

`ssh-keygen -y -P <passphrase> -f <file>`, stdin `/dev/null`, `SSH_ASKPASS_REQUIRE=never`. **Every row of this table, the permission case below it and all three bit flips were re-run independently by the check on the same machine and the same build — seventeen `ssh-keygen -y` invocations in all. Every string and every exit code matched.**

| Case | stderr/stdout | exit |
|---|---|---|
| encrypted key, correct passphrase | public key printed on stdout | 0 |
| encrypted key, **wrong** passphrase | `Load key "…/k_enc": incorrect passphrase supplied to decrypt private key` | 255 |
| encrypted key, `-P ''` (empty) | `Load key "…/k_enc": incorrect passphrase supplied to decrypt private key` | 255 |
| encrypted key, no `-P`, no tty | `Enter passphrase for "…/k_enc": ` then `Load key "…/k_enc": incorrect passphrase supplied to decrypt private key` | 255 |
| unencrypted key, no `-P` | public key printed | 0 |
| unencrypted key, **wrong** `-P` supplied | public key printed — **the surplus passphrase is ignored, not an error** | 0 |
| garbage file (not a key) | `Load key "…/k_garbage": invalid format` | 255 |
| truncated OpenSSH key | `Load key "…/k_trunc": invalid format` | 255 |
| key with mangled base64 armour | `Load key "…/k_corrupt": invalid format` | 255 |
| non-base64 body between valid BEGIN/END lines | `Load key "…/k_badb64": invalid format` | 255 |
| zero-length file | `Load key "…/k_empty": invalid format` | 255 |
| a **public** key file passed as the private key | `Load key "…/k_pubonly": invalid format` | 255 |
| file does not exist | `…/k_missing: No such file or directory` | 255 |
| encrypted **PEM/RSA** key, correct passphrase | public key printed | 0 |
| encrypted **PEM/RSA** key, wrong passphrase | `Load key "…/k_pem": incorrect passphrase supplied to decrypt private key` | 255 |

**The file-permission gate pre-empts every other diagnosis.** With mode `0644` on the key file, `ssh-keygen -y` never reaches the passphrase and instead prints:

```
@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@
@         WARNING: UNPROTECTED PRIVATE KEY FILE!          @
@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@
Permissions 0644 for '…/k_enc' are too open.
It is required that your private key files are NOT accessible by others.
This private key will be ignored.
Load key "…/k_enc": bad permissions
```

exit 255. `bad permissions` is a **third distinct terminal string** alongside `incorrect passphrase supplied to decrypt private key` and `invalid format`.

#### The critical negative result — "incorrect passphrase" does not mean the passphrase was wrong

The `k_enc` file written by this build names `aes256-ctr` in its cipher field and `bcrypt` in its KDF field, and its encrypted section is 144 bytes, read directly out of the file while locating the blob. **To locate the encrypted section:** base64-decode the body, skip the `openssh-key-v1\0` magic, then the cipher, KDF and KDF-options strings, the key count and the public-key string; the next length-prefixed string is the encrypted section. Offsets tested were its first byte, its first byte plus half its length, and its last byte.

Flipping a single bit inside that section and then supplying the **correct** passphrase gave:

| Bit flipped at | stderr | exit |
|---|---|---|
| first byte of the encrypted blob | `Load key "…": incorrect passphrase supplied to decrypt private key` | 255 |
| one interior offset (blob start + half its length) | **public key printed, identical to the intact key** | **0** |
| last byte of the encrypted blob | `Load key "…": incorrect passphrase supplied to decrypt private key` | 255 |

So on this build:

- **`incorrect passphrase supplied to decrypt private key` is emitted in at least two different situations:** a genuinely wrong passphrase, and the **correct** passphrase against a key whose encrypted section has one flipped bit at its first or last byte. **The string does not establish that the passphrase was wrong. It is a strong hint, not proof.**
- A flip at the one interior offset tested was **not detected at all** — exit 0 and output identical to the intact key. That the offset falls inside the private-key field, and that the field is not re-verified, is an **inference** from the observation, not a documented behaviour.

*(The findings asserted that the format has "no authentication tag" and that "integrity is inferred from plaintext check bytes and padding after decryption", and that the message *means* "the decrypted plaintext failed its self-consistency check". None of that is observable in these runs or present in any cited page; the format specification was outside the permitted sources. All of it was cut and the mechanism is now an open question.)*

**What a negative result would have looked like** for route A: distinct exit codes per cause, or `incorrect passphrase` appearing only when the passphrase was genuinely wrong, or the bit-flip cases producing a distinct `corrupt key` message. **None of those occurred.**

### Observed — route A′, local load into an agent (`ssh-add`)

Throwaway `ssh-agent` with its own socket; keys as above. **None of this was repeated** — an agent and a listening socket were outside what the check was permitted to start, and the one row needing neither (agent unreachable) was refused by the session's permissions. **Judged from the description alone.** The `ssh-add` command lines and the source of the `SSH_ASKPASS` helper are unrecorded; for the hang below, the helper's exact behaviour *is* the experiment, so a repeater must invent the central piece.

| Case | output | exit |
|---|---|---|
| correct passphrase | `Identity added: …/k_enc (throwaway)` | 0 |
| **wrong** passphrase, askpass supplying it | had not terminated after 12 seconds — see below | — |
| malformed key file | `Error loading key "…/k_garbage": invalid format` | 1 |
| file does not exist | `…/k_missing: No such file or directory` | 1 |
| encrypted key, no passphrase source at all | `Enter passphrase for …/k_enc: ` and nothing else | 1 |
| agent unreachable | `Error connecting to agent: No such file or directory` | 2 |

**The wrong-passphrase case is a behavioural trap.** With an `SSH_ASKPASS` helper that keeps returning the same wrong passphrase, `ssh-add` **kept retrying for as long as it was watched**: an instrumented **59 askpass invocations in 12 seconds, process still alive** when killed, and no diagnostic on stdout or stderr in that time. *(The findings said "never terminates" / "retried without bound" — a claim about all time drawn from twelve seconds; narrowed. How the invocations were counted is unrecorded.)* `ssh-add(1)` documents no retry limit — its only retry sentence is "ssh-add retries the last passphrase if multiple identity files are given", which is not a limit. `NumberOfPasswordPrompts` is an `ssh_config(5)` keyword; **that it does not apply to `ssh-add` is an inference**, not something either page states — `ssh_config(5)` says its contents are what "ssh(1) obtains", and `ssh-add(1)` cites `ssh_config(5)` only for default `known_hosts` file names.

**What a negative result would have looked like:** `ssh-add` exiting 1 after a bounded number of attempts with an `incorrect passphrase` message. **It did neither.**

### Observed — route B, connection where the key requires a passphrase

Client options held constant: `-F /dev/null -o UserKnownHostsFile=<temp> -o IdentitiesOnly=yes -o IdentityAgent=none -o ConnectTimeout=5`. Throwaway `sshd` at `127.0.0.1:22022` with `PubkeyAuthentication yes`, `PasswordAuthentication no`, `PerSourcePenalties no`. **None of this was repeated** — every row needs a connection to a listening server. **Four rows cannot be repeated from their own description even with a server in hand:** the success row does not give the remote command that printed `REMOTE_OK` and exited 7; the "host key changed" row does not say how the key was changed; and the two "TCP accepted then closed" rows do not say what was listening on ports 22031 and 22032.

At **default LogLevel (INFO)**:

| Case | stderr | exit |
|---|---|---|
| correct passphrase, key authorised, remote `exit 7` | `REMOTE_OK` on stdout | **7** (remote command's status) |
| **wrong passphrase** | `artist@127.0.0.1: Permission denied (publickey).` | 255 |
| **empty passphrase** supplied for an encrypted key | `artist@127.0.0.1: Permission denied (publickey).` | 255 |
| encrypted key, `BatchMode=yes`, no passphrase source | `artist@127.0.0.1: Permission denied (publickey).` | 255 |
| valid unencrypted key **not in `authorized_keys`** | `artist@127.0.0.1: Permission denied (publickey).` | 255 |
| **wrong account name**, key otherwise fine | `nosuchuser22@127.0.0.1: Permission denied (publickey).` | 255 |
| malformed file passed as `IdentityFile` | `Connection closed by 127.0.0.1 port 22022` | 255 |
| nonexistent `IdentityFile` | `Warning: Identity file …/k_missing not accessible: No such file or directory.` then `Connection closed by 127.0.0.1 port 22022` | 255 |
| host key **changed** | full `WARNING: REMOTE HOST IDENTIFICATION HAS CHANGED!` banner, then `Host key for [127.0.0.1]:22022 has changed and you have requested strict checking.` then `Host key verification failed.` | 255 |
| host key **unknown**, `BatchMode=yes` | `Host key verification failed.` (banner absent) | 255 |
| TCP port closed | `ssh: connect to host 127.0.0.1 port 22023: Connection refused` | 255 |
| host does not answer (192.0.2.1, `ConnectTimeout=5`) | `ssh: connect to host 192.0.2.1 port 22: Operation timed out` | 255 |
| hostname does not resolve | `ssh: Could not resolve hostname no-such-host-abcxyz.invalid: nodename nor servname provided, or not known` | 255 |
| TCP accepted then closed immediately | `Connection closed by 127.0.0.1 port 22031` | 255 |
| TCP open, non-SSH service (HTTP banner) | `Connection closed by 127.0.0.1 port 22032` | 255 |

**At default log level, on this build and this server configuration, five materially different causes collapsed into the single string `Permission denied (publickey).` and exit 255:** wrong passphrase, empty passphrase, no passphrase available, key not authorised on the server, wrong account name. **In these runs they were not distinguishable.** *(This is the load-bearing conclusion of the whole finding and was written as a general property of the client; it rests on one run of each case against one throwaway server on one build.)*

**At `-vv` (LogLevel DEBUG2) they separate.** Exact discriminating lines observed:

| Cause | discriminating debug line |
|---|---|
| wrong passphrase | `debug2: bad passphrase given, try again...` — emitted **3 times**, then `debug2: we did not send a packet, disable method` |
| no passphrase available (no tty, no askpass) | `debug1: read_passphrase: can't open /dev/tty: Device not configured` then `debug2: no passphrase given, try next key` |
| key not authorised on server | neither of the above; `debug1: Offering public key: …` is followed by a second `debug1: Authentications that can continue: publickey` and **no** `sign_and_send_pubkey` line |
| malformed key file | `debug1: no pubkey loaded from …/k_garbage` (contrast: a loadable key gives `debug1: loaded pubkey from …: ED25519 SHA256:…`) |

**`-v` alone is not enough.** Instrumented count of the string `bad passphrase` in client output: **0 occurrences at `-v`, 3 occurrences at `-vv`.** Of the three verbosity settings sampled — default, `-v`, `-vv` — the wrong-passphrase signal appeared only at `-vv`. This matters because `ssh(1)` only advises "increasing the LogLevel to DEBUG or higher (e.g. by using the -v flag)", and one `-v` did not carry it. *("Lives at DEBUG2 and nowhere lower" covered levels never sampled — `VERBOSE`, and `LogLevel` set directly rather than via `-v` flags; narrowed to the three settings run. "`-v` is DEBUG1" was stated as documented and is an **inference**: `ssh(1)` says only "Multiple -v options increase the verbosity. The maximum is 3", and `ssh_config(5)` ties DEBUG1 to no count of flags.)*

**The retry count is observable and bounded here, unlike `ssh-add`:** instrumented askpass invocations for one `ssh` run with a wrong passphrase = **3**. The client then prints `Permission denied (publickey).` and exits 255. That count equals the documented default of `NumberOfPasswordPrompts`, but **the keyword was never varied, so the two threes are not established as connected** — and the keyword's documented text speaks of *password* prompts, not private-key passphrase prompts.

**What a negative result would have looked like** for route B: a distinct exit code, or the `incorrect passphrase supplied to decrypt private key` string surfacing at default log level the way it does in route A, or `bad passphrase given` appearing at `-v`. **None occurred.**

### Observed — server-side rate limiting corrupts the diagnosis

Observed accidentally and then confirmed from the throwaway `sshd` log. With default `PerSourcePenalties`, after an **unrecorded** number of failed authentications ("a handful") from one source address the server logged:

```
srclimit_penalise: 127.0.0.1/32: activating ipv4 penalty of 20 seconds for penalty: failed authentication
drop connection #0 from [127.0.0.1]:63141 on [127.0.0.1]:22022 penalty: failed authentication
```

and every subsequent client attempt — including ones with the **correct** passphrase — failed with:

```
kex_exchange_identification: Connection closed by remote host
Connection closed by 127.0.0.1 port 22022
```

exit 255. **That client-side text reads as a server-side or network fault.** So repeated passphrase retries against a real server change the failure mode from "authentication" to "looks like the server broke", and nothing in the default-level client output identified a penalty as the cause. *Neither the number of failed authentications that triggered the penalty, nor where the server log was written, nor the server's `LogLevel`, is recorded — this observation cannot be re-induced from its own description.*

Two qualifications the findings did not carry:

- **The two cases are not textually identical in these runs.** The route B row for a listener that accepted TCP and closed immediately records only `Connection closed by 127.0.0.1 port 22031`, with **no** `kex_exchange_identification:` line; the penalty case has that line. Whether it reliably separates a penalty from an ordinary drop was not tested, and **the two observations sit in tension. Recorded, not resolved.**
- **The server side is documented, even though the client-side symptom is not.** `ssh(1)` says nothing about penalties, but `sshd_config(5)` documents the mechanism at length:

  > "PerSourcePenalties — Controls penalties for various conditions that may represent attacks on sshd(8). If a penalty is enforced against a client then its source address and any others in the same network, as defined by PerSourceNetBlockSize, will be refused connection for a period. … Penalties are enabled by default with the default settings listed below but may disabled using the no keyword."

  with `authfail:duration` "how long to refuse clients that disconnect after making one or more unsuccessful authentication attempts (default: 5s)", `min:duration` "the minimum penalty that must accrue before enforcement begins (default: 15s)" and `max:duration` (default 10m). **What is undocumented is what the *client* sees when it happens** — and `-vv` was not run for this case, so even the absence of a client-side signal is established only at the default level.

### Failure reason against available signal

Route A = local load/decrypt (`ssh-keygen -y`, `ssh-add`). Route B = connection.

| Failure reason | Route A signal | Route B signal (default LogLevel) | Route B signal (`-vv`) | Genuinely distinguishable? |
|---|---|---|---|---|
| Passphrase wrong | `incorrect passphrase supplied to decrypt private key`, exit 255 (`ssh-keygen`); `ssh-add` **kept retrying for the 12s watched, no message, no exit** | `Permission denied (publickey).`, exit 255 | `debug2: bad passphrase given, try again...` ×3 | **Route A: inferable, not proof** — the correct passphrase against a key with one flipped bit at either end of its encrypted section yields the identical string. **Route B: no** at default; **yes** at `-vv` via an undocumented debug string |
| Passphrase not supplied / empty | identical to "wrong" — `incorrect passphrase supplied…` | `Permission denied (publickey).` | `debug2: no passphrase given, try next key` (+ `read_passphrase: can't open /dev/tty`) | **Route A: no** — collapses into wrong-passphrase. **Route B: yes at `-vv` only** |
| Key file is not a key / malformed / truncated / empty / public-key-as-private | `invalid format`, exit 255 (`ssh-keygen`); `Error loading key "…": invalid format`, exit 1 (`ssh-add`) | `Connection closed by <host> port <n>`, exit 255 — same as a server fault | `debug1: no pubkey loaded from <file>` | **Route A: yes**, clean and reliable. **Route B: no** at default, **yes** at `-vv` |
| Key file silently corrupted in its encrypted body | `incorrect passphrase supplied…` (first and last byte tested) **or exit 0 with output identical to the intact key** (one interior offset tested) | not tested; expected to follow route A | not tested | **No.** At the offsets tested it either masquerades as a wrong passphrase or is missed entirely |
| Key file permissions too open | `bad permissions`, exit 255 — pre-empts passphrase handling entirely | **not tested on route B** | **not tested on route B** | **Route A: yes.** Route B: unknown — no run recorded |
| Key file missing | `<path>: No such file or directory`, exit 255 / exit 1 | `Warning: Identity file <path> not accessible: No such file or directory.` — printed at default level | same | **Yes**, both routes |
| Key not authorised on server | n/a — cannot be seen locally | `Permission denied (publickey).`, exit 255 | `Offering public key: …` followed by `Authentications that can continue: publickey` and no `sign_and_send_pubkey` | **No** at default; at `-vv` only by *absence* of the passphrase markers plus a rejection pattern — the weakest discriminator in this table |
| Account name wrong | n/a | `Permission denied (publickey).`, exit 255 | same as "not authorised" | **No.** Not separable from "key not authorised" by any observed client signal |
| Host key changed | n/a | `WARNING: REMOTE HOST IDENTIFICATION HAS CHANGED!` banner + `Host key for <host> has changed and you have requested strict checking.` + `Host key verification failed.`, exit 255 | same | **Yes** — but only via the banner text; the final line `Host key verification failed.` alone is shared with "host key unknown in BatchMode" |
| Host key unknown, no interaction possible | n/a | `Host key verification failed.`, exit 255, **no banner** | same | Distinguishable from "changed" only by the banner's absence |
| Server refused the connection (TCP) | n/a | `ssh: connect to host <h> port <p>: Connection refused`, exit 255 | same | **Yes** |
| Server accepted TCP then dropped (rate-limit penalty, non-SSH service, immediate close) | n/a | `Connection closed by <host> port <n>`, sometimes preceded by `kex_exchange_identification: Connection closed by remote host`, exit 255 | same | **No** — a `PerSourcePenalties` lockout caused by earlier wrong passphrases is indistinguishable from a broken server or a wrong port |
| Machine did not answer | n/a | `ssh: connect to host <h> port <p>: Operation timed out`, exit 255 (with `ConnectTimeout` set) | same | **Yes** |
| Name does not resolve | n/a | `ssh: Could not resolve hostname <h>: nodename nor servname provided, or not known`, exit 255 | same | **Yes** |

**Cross-cutting: no exit status observed here separated one failure cause from another.** `ssh` returned 255 for every failure on route B, which is also what `ssh(1)` documents for any error; `ssh-keygen` returned 255 for every one of the seventeen failing cases on route A; `ssh-add` returned 1 for the four failing cases observed and 2 for an unreachable agent, and **returned nothing at all for a wrong passphrase, having not exited.** **All discrimination rests on parsing undocumented message text.**

### Unresolved for Fact 37

1. **Portability of every string above.** All observed on `OpenSSH_10.2p1 / LibreSSL 3.3.6` on macOS 26.5.2 arm64 — one build, one platform. None of these strings is documented anywhere in the OpenSSH manual pages, so they carry no stability guarantee. **What would settle it:** running the same matrix against each OpenSSH version the application is expected to ship with or find on a user's machine, and against any non-OpenSSH client library it might use instead (a Go, Rust or Python SSH library has its own error taxonomy, which may well be better — typed errors rather than text — but that is a different fact and was not investigated).
2. **Whether `incorrect passphrase supplied to decrypt private key` can ever be trusted as proof.** Observed: emitted both for a genuinely wrong passphrase and for a correct passphrase against a corrupted key. It is a *probabilistic* signal. Nothing observed or documented bounds how often the corrupt case occurs. **CLOSED on the underlying question, 2026-08-21: the format offers no way to separate the two.** **Answered 2026-08-21 from OpenSSH's own `PROTOCOL.key`**, which ships in every source release and is the project's own publication — read 2026-08-20 at `https://raw.githubusercontent.com/openssh/openssh-portable/master/PROTOCOL.key`, the mirror OpenSSH names on `https://www.openssh.org/portable.html`. The container is *"a header, a list of public keys, and an encrypted list of matching private keys"* under `AUTH_MAGIC "openssh-key-v1"`, and the whole integrity story is one sentence: *"Before the key is encrypted, a random integer is assigned to both checkint fields so successful decryption can be quickly checked by verifying that both checkint fields hold the same value."* **There is no message authentication code over the encrypted blob and the format names none.** So a wrong passphrase, a corruption landing on either checkint field, and a corruption in the padding are **the same event to the decoder**, and a corruption in the middle of the key material is invisible — which is exactly what this document's own bit-flip table observed. **Choosing an authenticated cipher does not rescue the distinction at the level of the message**: keys written with `-Z aes256-gcm@openssh.com` and `-Z chacha20-poly1305@openssh.com` both produced the identical `incorrect passphrase supplied to decrypt private key` on a wrong passphrase, and both read back cleanly with the right one (Creator's Mac, 2026-08-20). **The rule this settles: `incorrect passphrase` is a strong hint and never proof, and the app must never phrase it to the user as *you typed it wrong*.** *(This was parked because `PROTOCOL.key` "is not a manual page". It is OpenSSH's own publication and is the same authority — `Method.md` now says so.)* What is still open is only how **often** the corrupt case occurs, which nothing bounds.
3. **The exact boundary of undetected corruption.** One interior bit-flip produced exit 0 with correct output; the mechanism is an inference from three data points. **What would settle it:** a sweep flipping every byte offset in the encrypted blob and recording the outcome per offset.
4. **`ssh-add`'s retry limit on a wrong passphrase.** Observed unbounded over the window watched (59 attempts in 12s, still running). Not established whether it is truly infinite or merely very large, nor whether it differs when reading from a real tty rather than an `SSH_ASKPASS` helper. Not documented in `ssh-add(1)`. **CLOSED 2026-08-21.** **Answered 2026-08-21 from OpenSSH's own source.** `https://raw.githubusercontent.com/openssh/openssh-portable/master/ssh-add.c`, read 2026-08-20, the passphrase loop in `add_file()` is `for (;;)` with **no counter**. It has exactly two exits: an **empty** passphrase, which jumps to `fail_load`; and a successful parse, which breaks. A wrong but non-empty passphrase falls through and goes round again. The file was fetched twice with different questions and gave the same lines. **So the retry is unbounded, and the consequence belongs in the architecture: the app must never hand `ssh-add` a passphrase it is unsure of, because it will not come back.** *(The live confirmation was attempted and refused — the session's permissions would not start an `ssh-agent` or bind a listening socket, which is the same wall the original Fact 37 check hit. The source settles it without the run; if the observation is ever wanted too it costs about two minutes in a session permitted to start a local agent.)*
5. **Network-down as distinct from host-not-answering.** Only "no answer within `ConnectTimeout`" and "name does not resolve" were produced. The no-route-to-host case (`ENETUNREACH`/`EHOSTUNREACH`) was not induced, because doing so would have required altering this machine's networking. Not established what string it produces or whether it is separable from a timeout.
6. **Whether the `-vv` discriminators survive an agent in the path.** All route B tests used `IdentityAgent=none` and `IdentitiesOnly=yes`. With an agent holding candidate keys the client tries agent keys before file keys, and it was not established whether the `bad passphrase given` / `no passphrase given` markers remain unambiguous when several identities are attempted in one connection. **What would settle it:** repeating route B with a populated agent and multiple `IdentityFile` entries.
7. **Server-side causes generally.** The client cannot see `authorized_keys` problems, `AllowUsers`/`DenyUsers` denials, account expiry, or `PerSourcePenalties` state — the last of which *is* documented server-side in `sshd_config(5)`, just not in anything the client emits. All arrive as `Permission denied (publickey).` or `Connection closed by <host>`. The only documented escape hatch is `ssh(1)`'s note that the server "may inform the client of errors that prevented public key authentication from succeeding **after authentication completes using a different method**" — which requires a second, working authentication method, and so is unavailable when public key is the only method. Not established whether any signal reaches the client in a key-only configuration; nothing observed suggests one does.
8. **Whether a `PerSourcePenalties` lockout is separable from an ordinary immediate close.** The two runs differ: the penalty case carried a `kex_exchange_identification: Connection closed by remote host` line, the plain immediate-close case did not. The findings called them indistinguishable anyway. **What would settle it:** running both cases side by side, at default level and at `-vv`, several times each.
9. **Whether the three askpass invocations on route B come from `NumberOfPasswordPrompts`.** Observed 3; the keyword's default is 3; the keyword was never varied and its documented text is about password prompts. **What would settle it:** set it to 1 and to 5 and re-count.
10. **What route B does with a key file whose permissions are too open.** The findings' summary table originally stated an outcome for this at both log levels; **no such run exists anywhere.** **What would settle it:** run it, at default level and at `-vv`.
11. **Whether the wrong-passphrase marker appears at `LogLevel VERBOSE`.** Only default, `-v` and `-vv` were sampled, so the level between INFO and DEBUG1 is untested, as is setting `LogLevel` directly rather than through `-v` flags.

---

## Fact 38 — what each of Mac, Windows and Linux offers for holding a secret on the user's own computer; what the user must do to grant it; what they meet when it is refused, moved or lost; and what using it costs them

**Four questions, three operating systems.** macOS is documented and partly observed; **Windows is documentation alone with no machine behind it**; Linux is documentation alone, and its documentation deliberately declines to guarantee the two things that matter most. Erasure and the session-only case are carried under each system because they answer differently on each.

### macOS — what the system offers

**There are two distinct stores behind one API, and the distinction decides almost every answer.** Documented:

> "macOS has three keychain APIs: … **Keychain**. … **SecKeychain**. … **SecItem**. …
> macOS has two keychain implementations:
> - File-based keychain
> - Data protection keychain"
> — TN3137, *On Mac keychain APIs and implementations*

> "The Keychain and SecKeychain APIs always target the file-based keychain. The SecItem API can target either implementation. **It defaults to targeting the file-based keychain.** To target the data protection keychain, set the `kSecUseDataProtectionKeychain` attribute or the `kSecAttrSynchronizable` attribute to true."
> — TN3137

What the keychain is, in Apple's words:

> "Securely store small chunks of data on behalf of the user." … "giving your app a mechanism to store small bits of user data in an encrypted database called a keychain."
> — Keychain services

> "keychain services handles data encryption and storage (including data attributes) in a keychain, which is an encrypted database stored on disk. Later, authorized processes use keychain services to find the item and decrypt its data."
> — Keychain items

Documented guarantees (data protection keychain):

> "Keychain items are encrypted using two different AES-256-GCM keys: a table key (metadata) and a per-row key (secret key). Keychain metadata (all attributes other than kSecValue) is encrypted with the metadata key to speed searches, and the secret value (kSecValueData) is encrypted with the secret key. The metadata key is protected by the Secure Enclave but is cached in the Application Processor to allow fast queries of the keychain. The secret key always requires a round trip through the Secure Enclave. The keychain is implemented as a SQLite database, stored on the file system. There is only one database, and the securityd daemon determines which keychain items each process or app can access."
> — Apple Platform Security, *Keychain data protection*

> "On devices with macOS, Data Protection isn't used directly to enforce these guarantees."
> — same page

**That last sentence is the boundary of the guarantee on the Mac.** The class structure (`kSecAttrAccessibleWhenUnlocked` and friends) is described for the Data Protection world, and Apple states plainly that macOS does not enforce those guarantees the same way. Apple's macOS-specific statement is weaker, and is about the login keychain being tied to the login password.

**What it protects against, as documented:** reading the secret value off disk without the keys, and access by processes other than those authorised — `securityd` arbitration by entitlements for the data protection keychain, ACLs for the file-based keychain.

**What it does not protect against: no Apple page read for this fact states a threat model in negative form.** Apple documents what is encrypted, with which keys, and who arbitrates access; it nowhere states what the keychain fails to stop. The limits readable off the positive statements: access on the file-based keychain is governed only by ACLs, and the user can make a caller permanently and silently trusted by clicking "Always Allow"; the keychain password "is set to match their login password" when the user is created. **Unresolved: an explicit negative threat model.** *(The findings added "so anything that yields the login password yields the keychain" — cut, because no Apple page read says it; the source says only that the two passwords are set to match and are kept in step.)*

**Item size limits are not stated on any Apple page read here. Unresolved.**

### macOS — how an application reaches it

`SecItemAdd(_:_:)`, `SecItemCopyMatching(_:_:)`, `SecItemUpdate(_:_:)`, `SecItemDelete(_:)` in the Security framework. *(The findings said these are "present on every stock macOS installation. No third-party dependency." — cut; no Apple page read states it. What is on the record is narrower and **observed**: the Security framework loaded and these four functions resolved on the machine below without anything being installed.)*

> "Adds one or more items to a keychain." … "Keychain services takes care of encrypting this data if the item is secret, namely when it's one of the password types or involves a private key."
> — `SecItemAdd`

> "`SecItemAdd` blocks the calling thread, so it can cause your app's UI to hang if called from the main thread. Instead, call `SecItemAdd` from a background dispatch queue or `async` function"
> — same page. **The same "Performance considerations" paragraph appears verbatim for `SecItemCopyMatching`, `SecItemUpdate` and `SecItemDelete`.** This matters: the blocked call observed below is exactly this behaviour.

Reaching the **data protection** keychain has a code-signing precondition:

> "macOS builds the list of data protection keychain access groups available to your program from its code signing entitlements. … **These entitlements must be authorized by a provisioning profile. Your program needs an app-like bundle structure in which to embed that profile.** This is standard for app and app extensions but not for command-line tools."
> — TN3137

> "The data protection keychain is only available in a user login context. You can't use it, for example, from a `launchd` daemon."
> — TN3137

> "Xcode automatically adds the `application-identifier` entitlement (or the `com.apple.application-identifier` entitlement in macOS) to every app during code signing, formed as the team identifier (team ID) plus the bundle identifier (bundle ID)." … "Because app IDs are unique across all apps, and because the app ID is stored in an entitlement protected by code signing, no other app can use it, therefore no other app is in this group."
> — Sharing access to keychain items among a collection of apps

**Reaching the file-based keychain needs no entitlement (observed) but brings ACLs and their dialog.**

**Unresolved: whether a Developer ID–signed, non-App-Store app that ships without an embedded provisioning profile can use the data protection keychain.** TN3137 says the entitlements "must be authorized by a provisioning profile"; no page read states the outcome for a Developer ID app with no profile. **What would settle it:** signing a minimal Developer-ID app both ways and calling `SecItemAdd` with `kSecUseDataProtectionKeychain`.

### macOS — what the user must do to grant access

**Data protection keychain: nothing is documented for the *sharing* case,** and that is the only case a page read here covers:

> "This kind of sharing doesn't require interaction with or permission from the user, but limits sharing to apps that are delivered by a single development team."
> — Sharing access to keychain items among a collection of apps

*(The findings generalised this to "Access is decided by entitlements at code-signing time, not by the user at runtime"; the quote is scoped to sharing items among apps from one team, and no page read says the same of every data-protection access.)*

Unless the item is created with an access-control object demanding user presence:

> "Impose this restriction by supplying a `SecAccessControl` instance as the value for the `kSecAttrAccessControl` attribute when creating a keychain item." … "you typically use the `userPresence` option … to let the system choose a mechanism"
> — Restricting keychain item accessibility

**File-based keychain: the user may be asked, and this is the answer that costs the most.**

> "When an app attempts to access a keychain item for a particular purpose—like using a private key to sign a document—the system looks for an entry in the item's ACL containing the operation. If there's no entry that lists the operation, then the system denies access and it's up to the calling app to try something else or to notify the user.
> If there is an entry that lists the operation, the system checks whether the calling app is among the entry's trusted apps. If so, the system grants access. **Otherwise, the system prompts the user for confirmation. The user may choose to Deny, Allow, or Always Allow the access.** In the latter case, the system adds the app to the list of trusted apps for that entry, enabling the app to gain access in the future without prompting the user again."
> — Access control lists

The user-facing wording of that dialog:

> "When you connect to an app, server, or other item that's protected by a password, the item tries to retrieve the password from your keychain. Keychain Access requests your permission to provide the password to the app or server. Choose one of the following options: **Allow Once**: Grant access this time only. You are asked again the next time the app or server needs the password. **Always Allow**: Let the app or server retrieve the password from your keychain without any further authorization or notice from you. **Deny**: Prevent the app or server from retrieving the password from your keychain. You need to enter the password manually."
> — Keychain Access user guide, kyca1243

And the recurrence condition, which is the one that bites when the application is updated or otherwise modified:

> "**If an app is changed or infected by a virus after being granted access to your keychain, the app is no longer trusted, and you must grant access again.** A dialog asks if you want to change the keychain to allow access. This dialog appears if you recently updated your system software or the app, or if the app has been modified."
> — kyca1331

*(The findings read this as biting "on every app update". The same page says three lines further down: "You must sometimes reauthorize updated apps." — **sometimes**, not every time.)*

The keychain itself may also be locked, in which case an unlock dialog appears:

> "Certain keychain services functions that require the presence of a keychain automatically display a Keychain Not Found dialog if there is none. Functions that require the keychain to be unlocked automatically display the Unlock Keychain dialog."
> — `SecKeychainSetUserInteractionAllowed(_:)`

> "Your keychain may be locked automatically if your computer has been inactive for a period of time or your user password and keychain password are out of sync."
> — kyca1242

Interaction can be suppressed programmatically, converting the dialog into an error:

> "If you are writing an application that must run unattended on a server, you may wish to disable the user interface so that any subsequent keychain calls that normally bring up the unlock UI will instead return immediately with an `errSecInteractionRequired` result)."
> — `SecKeychainSetUserInteractionAllowed(_:)`

**Observed, and it does not match that phrasing:** with interaction disallowed, the ACL case returned `errSecAuthFailed` (−25293), **not** `errSecInteractionRequired`.

### macOS — what happens when it is refused, moved or lost

Everything returns an `OSStatus`. The relevant codes, with Apple's own one-line descriptions:

| Code | Apple's words |
| --- | --- |
| `errSecUserCanceled` (−128) | "User canceled the operation." |
| `errSecItemNotFound` (−25300) | "The item cannot be found." |
| `errSecInteractionNotAllowed` (−25308) | "Interaction with the Security Server is not allowed." |
| `errSecInteractionRequired` | "User interaction is required." |
| `errSecAuthFailed` (−25293) | "Authorization and/or authentication failed." |
| `errSecMissingEntitlement` (−34018) | "A required entitlement is missing." |
| `errSecDuplicateItem` | "The item already exists." |
| `errSecNoSuchKeychain` | "The keychain does not exist." |
| `errSecInvalidKeychain` | "The keychain is not valid." |

> **ANSWERED 2026-08-21: the mapping is published by Apple, shipped by Apple, and was on the disk the probes ran on.** Apple's own SDK header `SecBase.h`, at `/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/System/Library/Frameworks/Security.framework/Headers/SecBase.h` (also at `/Library/Developer/CommandLineTools/SDKs/MacOSX26.sdk/…`, dated 25 Apr 2026), read 2026-08-20: `errSecMissingEntitlement = -34018`, `errSecNotAvailable = -25291`, `errSecReadOnly = -25292`, `errSecAuthFailed = -25293`, `errSecNoSuchKeychain = -25294`, `errSecInvalidKeychain = -25295`, `errSecDuplicateItem = -25299`, `errSecItemNotFound = -25300`, `errSecInteractionNotAllowed = -25308` — **and the header's one-line comments are word for word the descriptions already quoted from Apple's web pages.** Confirmed a second way by Apple's own lookup tool on the Creator's Mac, 2026-08-20: `/usr/bin/security error -25293` → *"The user name or passphrase you entered is not correct."*, `-25300` → *"The specified item could not be found in the keychain."*, `-34018` → *"A required entitlement isn't present."*, `-25308` → *"User interaction is not allowed."*, against a control `-999999` → *"unknown error -999999"*. Label: **documented**. *(This is the clearest case in this document of the sourcing rule looking outward when the authority was on the local disk — the result-code web pages do give only the symbol and the description.)* *(This paragraph ended **Sent back.** until 2026-08-22. The item was answered on 2026-08-21, above, and sent-back item 17 is struck.)*

**Can "the user refused" be told apart from "the store is unavailable" and from "the item is not there"? On macOS, yes, and this is the strongest of the three platforms on this point** — with the caveats attached:

- **refused by the user** → `errSecUserCanceled`, whose documented meaning is "User canceled the operation." *No page read here connects this code to a **Deny** click in the keychain ACL dialog.*
- **item absent** → `errSecItemNotFound`.
- **store absent / not usable** → `errSecInteractionRequired` when the keychain is locked and UI has been disallowed (stated on the `SecKeychainSetUserInteractionAllowed` page); and, **by their descriptions alone**, `errSecNoSuchKeychain` ("The keychain does not exist.") and `errSecInvalidKeychain` ("The keychain is not valid."). *No page read states the conditions under which those two, or `errSecInteractionNotAllowed`, are returned.*
- **caller not permitted** → `errSecMissingEntitlement` for the data protection keychain — its Apple page scopes the error narrowly: "The method returns this error when you specify an access group to which your app doesn't belong" — or, **observed**, `errSecAuthFailed` for the file-based ACL with interaction disallowed. *The data-protection leg rests on the disputed observation below.*

**Unresolved: that the file-based path returns `errSecUserCanceled` specifically when the user clicks Deny in the ACL dialog.** No page read states the code for a Deny click, and the probe could not reach that branch without a human at the screen. **What would settle it:** run the call with interaction allowed and click Deny.

**Moved or lost. Two documented ways the secret dies:**

> "Your keychain password lets you automatically log in to apps and services stored in Keychain Access. When you create a user, their keychain password is set to match their login password. When you change your login password, the keychain password is automatically updated to match it. … The keychain password is available only to the user and not the administrator. **If the login password is reset by the administrator, the keychain password is not changed.** The user is asked to reset the keychain password during the next login. **If the login password was reset because the user forgot the old password, the keys, passwords, and other information in the keychain won't be accessible, and the user will need to create a new keychain.**"
> — mchlp1086

> "At login, if your keychain password somehow differs from your user password, it doesn't automatically unlock, and you're asked to enter the keychain's password. It's recommended that you enter your previous user password. **If this doesn't unlock the keychain, the keychain automatically resets.** … **Resetting the default keychain deletes all the passwords saved in the keychain** … If your user password was reset because you forgot your password, and you can't provide the old password for the keychain, you won't be able to access the information in the old keychain, and a new blank keychain is created."
> — kyca2429

**New machine:** items marked `ThisDeviceOnly` do not come across.

> "Items with this attribute do not migrate to a new device. Thus, after restoring from a backup of a different device, these items will not be present."
> — `kSecAttrAccessibleWhenUnlockedThisDeviceOnly`

> "If the attribute ends with the string `ThisDeviceOnly`, the item can be restored to the same device that created a backup, but it isn't migrated when restoring another device's backup data."
> — Restricting keychain item accessibility

The opposite choice puts the secret on every device the user owns:

> "Updating or deleting items using the `kSecAttrSynchronizable` key affects all copies of the item, not just the one on your local device."
> — `kSecAttrSynchronizable`

And the constraint that couples these:

> "You can use this attribute for macOS keychain items only if you also set a value of `true` for the `kSecUseDataProtectionKeychain` key, the `kSecAttrSynchronizable` key, or both."
> — `kSecAttrAccessible`

So `kSecAttrAccessible` is honoured on macOS only in the data protection world.

### macOS — session-only, never touching disk

**No macOS keychain facility for this was found.** The accessibility values documented are "When Passcode Set", "When Unlocked", "After First Unlock", "Always" — all describe *when a stored item is readable*, none describes an item that is never stored. Apple's own description of the keychain is "an encrypted database stored on disk". **There is no documented equivalent of Windows' `CRED_PERSIST_SESSION` or the Secret Service's session collection.**

*That a session-only secret therefore has to be process memory held by the application itself is the writer's conclusion from that absence, not a statement in any source read.*

### macOS — erasure, and whether erasure is confirmable

> "Deletes items that match a search query." … "The query dictionary for delete can't contain item return result keys, because `SecItemDelete(_:)` only returns a status." … "**By default, this function deletes all items matching the specified query.**"
> — `SecItemDelete(_:)`

Erasure is confirmable in the sense that the delete returns a status and a subsequent query returns `errSecItemNotFound`. **Observed end-to-end** below.

### Windows — documentation only, unconfirmed against a real installation

No Windows machine has run it, and none is rented today. Everything in this section is what Microsoft's documentation states. **None of it has been run.**

**Two separate facilities, at different levels.**

**(a) Credential Manager** — a store, reached through `CredWriteW` / `CredReadW` / `CredDeleteW` in `Advapi32.dll`.

> "The **CredWrite** function creates a new credential or modifies an existing credential in the user's credential set. The new credential is associated with the logon session of the current token. The token must not have the user's security identifier (SID) disabled."
> — `CredWriteW`

> "**CRED_TYPE_GENERIC** … The credential is a generic credential. The credential will not be used by any particular authentication package. **The credential will be stored securely but has no other significant characteristics.**"
> — `CREDENTIAL` structure

That sentence is the **entire documented guarantee** for the credential type an application would use for its own secrets. What "stored securely" means is documented elsewhere:

> "Users may choose to save passwords in Windows by using an application or through the Credential Manager Control Panel applet. **These credentials are stored on the hard disk drive and protected by using the Data Protection Application Programming Interface (DPAPI). Any program running as that user will be able to access credentials in this store.**"
> — hh994565, *Cached and Stored Credentials Technical Overview*, section "Credential Manager store". **This page carries `is_archived: true` and `ms.date 2016-08-31`, and sits in the Windows Server 2012 R2 and 2012 previous-versions set. It is the only source for that load-bearing sentence.**

> "Credential Manager uses the Credential Locker, formerly known as Windows Vault, for secure storage of user names and passwords."
> — same page

**This is the load-bearing difference from macOS.** Microsoft documents **no per-application isolation** for the Credential Manager store: any program running as that user can read it. No page read here documents an ACL, an entitlement, or a system prompt on `CredWrite` / `CredRead` / `CredDelete`.

*The findings added "There is also no documented mechanism by which the user is ever asked" — cut as over-broad. The `CREDENTIAL` structure page documents one prompting path, under `CRED_FLAGS_PROMPT_NOW`:* "If Type is CRED_TYPE_DOMAIN_PASSWORD or CRED_TYPE_DOMAIN_CERTIFICATE, an authentication package always fails an authentication attempt when using credentials marked as CRED_FLAGS_PROMPT_NOW. The application (typically through the key ring UI) prompts the user for the password." *That is the application prompting, and it does not apply to `CRED_TYPE_GENERIC` — but it is a documented mechanism, and the absolute wording did not survive it.*

**(b) DPAPI** — `CryptProtectData` / `CryptUnprotectData` in `Crypt32.dll`. **This is protection, not storage.**

> "Typically, only a user with the same logon credential as the user who encrypted the data can decrypt the data. In addition, the encryption and decryption usually must be done on the same computer."
> — `CryptProtectData`

> "An important point to remember is that DPAPI merely applies cryptographic protection to the data. **It does not store any of the protected data; therefore applications calling DPAPI must implement their own storage of the protected data.**"
> — ms995355, *Windows Data Protection*

> "A small drawback to using the logon password is that all applications running under the same user can access any protected data that they know about. … To counteract this, DPAPI allows an application to use an additional secret when protecting data. This additional secret is then required to unprotect the data. … Technically, this 'secret' should be called secondary entropy."
> — same page

**So the documented ceiling on Windows is: protection against another user and against another machine, not against another program of the same user.** Optional entropy raises the bar between applications of the same user but is itself something the application must keep.

**Sizes:**

> "The size, in bytes, of the **CredentialBlob** member. This member cannot be larger than **CRED_MAX_CREDENTIAL_BLOB_SIZE** (5*512) bytes."
> — `CREDENTIAL` structure — that is **2560 bytes per credential**, arithmetic on the quoted expression.

> "This member cannot be longer than **CRED_MAX_GENERIC_TARGET_NAME_LENGTH** (32767) characters." (for `CRED_TYPE_GENERIC` target names) — same page.

> "If the **Type** member is **CRED_TYPE_GENERIC**, this member is defined by the application." (the `CredentialBlob`) — same page.

**Naming convention Microsoft asks for:**

> "If the **Type** is CRED_TYPE_GENERIC, this member should identify the service that uses the credential in addition to the actual target. Microsoft suggests the name be prefixed by the name of the company implementing the service."
> — same page

### Windows — how an application reaches it

`Advapi32.dll` (`CredWriteW`, `CredReadW`, `CredDeleteW`) and `Crypt32.dll` (`CryptProtectData`, `CryptUnprotectData`) — the DLL named in the Requirements table on each page cited, each table giving "Minimum supported client: Windows XP" and "Minimum supported server: Windows Server 2003". *(The findings said both are "present on stock Windows … No installation, no dependency, no runtime" — cut. A Requirements table states the minimum supported OS version, not that the DLL ships with every installation. `CredEnumerate` and `CredFree` were also removed from the list: no page for either was read.)*

For `Crypt32.dll` alone, a separate sentence does carry presence:

> "The public DPAPI interfaces are part of Crypt32.dll and are available for any user process that has loaded it. This DLL is part of CryptoAPI; **application developers can assume that all Windows systems have this DLL available.**"
> — ms995355

Nothing read carries the same for `Advapi32.dll`. **That was challenged on 2026-08-20 on the ground that Microsoft publishes a reference page for the library; it does not.** There is no such page — the library appears only as a Requirements-table row on the pages of the functions it exports, and such a row states a minimum supported version rather than presence, which is exactly what this document already worked out. **The nearest first-party statement is real, weaker, and archived**, and it is recorded here as partial support rather than as the sentence that is missing: *Server Core Functions by DLL*, filed under *previous-versions*, opens *"This list contains, by the associated DLL, the APIs supported in full by Server Core."* and lists `advapi32.dll` with hundreds of function rows. **Server Core is the most reduced Windows installation Microsoft ships, so the library is present there** — but that is one page about one edition, and it is nothing like the `Crypt32.dll` sentence quoted above. **Settling it is one line on any Windows installation**, and better still one credential-writing call, which settles presence and reachability together; it rides in the standing Windows batch.

### Windows — what the user must do to grant access

**Nothing, on either facility, in the documented default path.** No page read here documents any user-facing prompt for `CredWrite`, `CredRead`, `CredDelete`, or for `CryptProtectData` / `CryptUnprotectData` called with a NULL prompt structure. The word "prompt" does not occur at all on the `CredWriteW`, `CredReadW` or `CredDeleteW` pages.

The one prompting mechanism that exists is being removed:

> "**Important** — The prompt-based flow controlled by this parameter is deprecated and will be removed in **February 2027**. Passing **NULL** or a struct with `dwPromptFlags` set to 0 will use the non-interactive path for new operations. However, **operations on data originally protected with the PromptStruct flow will fail.**"
> — `CryptProtectData`, and an identical note on `CryptUnprotectData`. *Vendor future tense, on a page read 2026-08-16.*

There is also a flag that turns any would-be UI into an error:

> "**CRYPTPROTECT_UI_FORBIDDEN** — This flag is used for remote situations where presenting a user interface (UI) is not an option. When this flag is set and a UI is specified for either the protect or unprotect operation, the operation fails and GetLastError returns the **ERROR_PASSWORD_RESTRICTION** code."
> — same page

### Windows — what happens when it is refused, moved or lost

Return values are `BOOL` plus `GetLastError()`. `CredRead`:

> "- ERROR_NOT_FOUND — No credential exists with the specified *TargetName*.
> - ERROR_NO_SUCH_LOGON_SESSION — The logon session does not exist or there is no credential set associated with this logon session. Network logon sessions do not have an associated credential set.
> - ERROR_INVALID_FLAGS — A flag that is not valid was specified for the *Flags* parameter."

`CredDelete` returns the same three. `CredWrite` adds `ERROR_INVALID_PARAMETER`, `ERROR_BAD_USERNAME`, `ERROR_NOT_FOUND` and smart-card errors.

**Can "refused" be told apart from "unavailable" from "not there"?**

- **not there** → `ERROR_NOT_FOUND`. Clean.
- **store unavailable** → `ERROR_NO_SUCH_LOGON_SESSION` is the only documented candidate, and its documented meaning is narrow ("the logon session does not exist or there is no credential set associated with this logon session").
- **the user refused** → **no such state is documented for `CRED_TYPE_GENERIC`.** None of the three `Cred*` pages read documents a prompt on the generic path, so none documents a refusal. The nearest analogue is `ERROR_PASSWORD_RESTRICTION` from `CRYPTPROTECT_UI_FORBIDDEN`, which reports "UI was needed and forbidden", not "the user said no". *(Scoped to `CRED_TYPE_GENERIC`: see the `CRED_FLAGS_PROMPT_NOW` note above.)*

**For DPAPI failures, Microsoft warns the error code is not reliable:**

> "The Message Authentication Code (MAC) hash added to the encrypted data is used to detect whether the encrypted data was altered in any way. However, **the specific error code returned when tampering is detected may vary depending on the nature of the corruption. The function may return ERROR_INVALID_DATA, ERROR_INVALID_PARAMETER, or in some cases may succeed with corrupted output. Applications should not rely on a specific error code to detect data tampering.** For robust tamper detection, consider implementing additional integrity checks at the application level."
> — `CryptUnprotectData`

**That is a direct architectural consequence: on Windows, a DPAPI blob that fails to decrypt cannot be classified reliably from the error alone.**

**Moved or lost — documented in detail, and better than on macOS:**

> "DPAPI initially generates a strong key called a MasterKey, which is protected by the user's password. … the encrypted MasterKey and HMAC, salt, iteration count, and backup MasterKey are all stored in a MasterKey file, which resides in the user's profile directory."
> — ms995355

Password change survives:

> "First, DPAPI hooks into the password-changing module and **when a user's password is changed, all MasterKeys are re-encrypted under the new password.** Second, the system keeps a 'Credential History' file in the user's profile directory. When a user changes his or her password, the old password is added to the top of this file and then the file is encrypted by the new password."
> — same page

Password *reset* has recovery paths, unlike macOS:

> "When a computer is a member of a domain, DPAPI has a backup mechanism to allow unprotection of the data. … The client encrypts the MasterKey with the Domain Controller public key."
> — same page

> "Well, Windows XP has a new feature: the Password Reset Disk (PRD). A user can create a PRD from the Control Panel at any time, and it then allows the user to reset their password when they forget it. **When the password is reset by the PRD, all of the user's MasterKeys are re-encrypted under the new password.**"
> — same page. **Note the vintage:** the source presents the PRD as a then-new Windows XP feature, in an article dated October 2001 and flagged archived, not as a standing property of current Windows.

**Unresolved: what happens to DPAPI-protected data and to Credential Manager items when a local (non-domain) account's password is reset by an administrator without a Password Reset Disk.** The article describes only the change path, the domain backup path and the PRD path. It does not state the outcome for an administrative reset on a standalone machine. **Nor does it cover Microsoft-account or Windows Hello logins, which post-date it.** **What would settle it:** current Microsoft documentation on DPAPI master-key recovery for local and Microsoft accounts, or a test on a real installation.

**New machine** — `CredWrite`'s persistence values decide it:

> "**CRED_PERSIST_LOCAL_MACHINE** — The credential persists for all subsequent logon sessions on this same computer. It is visible to other logon sessions of this same user on this same computer and **not visible to logon sessions for this user on other computers.**"
> "**CRED_PERSIST_ENTERPRISE** — The credential persists for all subsequent logon sessions on this same computer. It is visible to other logon sessions of this same user on this same computer **and to logon sessions for this user on other computers.** This option can be implemented as locally persisted credential if the administrator or user configures the user account to not have roam-able state. For instance, if the user has no roaming profile, the credential will only persist locally."
> — `CREDENTIAL` structure

And DPAPI, independently:

> "decryption usually can only be done on the computer where the data was encrypted. **However, a user with a roaming profile can decrypt the data from another computer on the network.**"
> — `CryptProtectData`

### Windows — session-only, never touching disk

There is a documented session persistence class:

> "**CRED_PERSIST_SESSION** — The credential persists for the life of the logon session. It will not be visible to other logon sessions of this same user. **It will not exist after this user logs off and back on.**"
> — `CREDENTIAL` structure

**Two cautions.** First, its documented lifetime is the **logon session**, not the application. It survives the application closing and dies at logoff; "dies when the application closes" is therefore not what it provides, and the application would still have to delete it on exit. Second, **unresolved: whether `CRED_PERSIST_SESSION` credentials ever touch disk.** No Microsoft page read states they are memory-only, and the Cached and Stored Credentials overview says of the Credential Manager store generally that credentials "are stored on the hard disk drive and protected by using the Data Protection Application Programming Interface (DPAPI)" without distinguishing persistence classes. **What would settle it:** an explicit Microsoft statement about `CRED_PERSIST_SESSION` storage, or observation on a real installation.

For a genuinely memory-only secret Microsoft points elsewhere:

> "To encrypt memory for temporary use in the same process or across processes, call the `CryptProtectMemory` function."
> — `CryptProtectData`

And a same-security-context helper for credential strings:

> "The **CredProtect** function encrypts the specified credentials so that only the current security context can decrypt them." … "Note that the output of the **CredProtect** function is not integrity protected, so if the output is modified, the CredUnprotect function is not updated and may produce incorrect results."
> — `CredProtectW`

### Windows — erasure, and whether erasure is confirmable

> "The **CredDelete** function deletes a credential from the user's credential set." … Returns TRUE on success; `ERROR_NOT_FOUND` — "There is no credential with the specified *TargetName*."
> — `CredDeleteW`

Confirmable **to the extent that the delete returns TRUE or FALSE**. *That a following `CredRead` returns `ERROR_NOT_FOUND` is **inference**, not citation — no source read states the sequence, and no Windows machine has run it.* DPAPI blobs, being application-stored, are erased by the application deleting its own file.

### Linux — there is no single answer, and the specification says so

The only cross-desktop contract is a **draft** specification that mandates remarkably little.

> "The Secret Service API allows client applications to store secrets securely in a service running in the user's login session.
> The secrets are usually stored in an encrypted manner by the service. The service may need to be unlocked by the user before the secrets become available for retrieval by client applications.
> The Secret Service stores a secret along with a set of lookup attributes. The attributes can be used to look up and retrieve a secret at a later date. **The lookup attributes are not treated as secret material, and the service may choose not to encrypt attributes when storing them to disk.**
> This API was designed by GNOME and KDE developers with the goal of having a common way to store secrets. Its predecessors are the desktop-specific APIs used by GNOME Keyring and KWallet."
> — Secret Service API **Draft**, ch01

The specification explicitly declines to guarantee the two things that matter most here:

> "**This specification does not mandate the use of master passwords to lock a collection of secrets.** The service may choose to implement any method for locking secrets.
> **This specification does not mandate any form of access control.** The service may choose to allow certain applications to access a keyring, and others.
> [TODO: complete]"
> — ch10

Chapter 11, "Notes for Service Implementors", reads **in its entirety**:

> "[TODO: complete]"

**So what the secret is protected against on Linux is not specified. It is whatever the installed implementation does.** That is a fact to carry, not to smooth over.

Attributes — the things an application searches by, e.g. which machine a host key belongs to — are documented as not secret:

> "It's important to remember that attributes are not part of the secret. **Services implementing this API will probably store attributes in an unencrypted manner** in order to support simple and effecient lookups."
> — lookup-attributes *(the source's own misspelling preserved)*

Secrets crossing the bus:

> "Since this is a D-Bus API, the data in all method calls and other accesses in this API will go through multiple processes, and may be cached arbitrarily by the OS or elsewhere.
> The Secrets API has provision to encrypt secrets while in transit between the service and the client application. **The encryption is not envisioned to withstand man in the middle attacks, or other active attacks.** It is envisioned to minimize storage of plain text secrets in memory and prevent plain text storage of secrets in a swap file or other caching mechanism.
> Many client applications may choose not to make use of the provisions to encrypt secrets in transit."
> — transfer-secrets

The transport choices are the `plain` algorithm and `dh-ietf1024-sha256-aes128-cbc-pkcs7`, negotiated in `OpenSession()`; "The `plain` algorithm is almost always supported."

### Linux — how an application reaches it

D-Bus, on the session bus, at these object paths:

> "`/org/freedesktop/secrets` — The object path for the service.
> `/org/freedesktop/secrets/collection/xxxx` — The object path for a collection …
> `/org/freedesktop/secrets/aliases/default` — The default collection for client applications to store secrets is available under this object path in addition to its real object path."
> — object-paths

Interfaces: `org.freedesktop.Secret.Service`, `.Collection`, `.Item`, `.Session`, `.Prompt`.

**The well-known bus name is not stated anywhere in the specification pages read.** It appears in an implementation's own documentation:

> ```
> [D-BUS Service]
> Name=org.freedesktop.secrets
> Exec=/usr/bin/keepassxc
> ```
> — KeePassXC User Guide, Secret Service Integration

The usual client library is GNOME's libsecret, which is a **client, not a store**:

> "Secret – 1. Secret Service D-Bus client library. Version 0.21.3."
> — libsecret index

> "The package name is `libsecret-1`."
> — libsecret, Using

**Is a store present on a stock installation? Answered 2026-08-21, on the vendor's own artefact.** The two desktop implementations document start-up at desktop login, and no source states what runs outside a desktop session — **but the distribution publishes what its server image contains, and it contains none of them.** Canonical's own published package manifest for the Ubuntu Server install image, `https://releases.ubuntu.com/24.04/ubuntu-24.04.3-live-server-amd64.manifest`, read 2026-08-20: **no `gnome-keyring` line, no `ksecretservice`, no `kwalletmanager`, and no `libsecret-1-0`** — not even the client library a caller would ask with. `dbus` **is** there, six lines including `dbus-user-session` and `dbus-session-bus-common`. **So on the stock server image of the distribution family this project actually rents, a session bus exists and nothing can answer on it for `org.freedesktop.secrets`.** That is the vendor's own artefact rather than an opinion about the world; a negative result would have been a `gnome-keyring` line. *(The cut sentence — "on a headless machine there is usually none" — is restored in this narrower and better form: it is a statement about a named published image rather than about headless machines in general.)*

GNOME's daemon:

> "The **gnome-keyring-daemon** is a service that stores your passwords and secrets. **It is normally started automatically when a user logs into a desktop session.**
> The **gnome-keyring-daemon** implements the DBus Secret Service API"
> — gnome-keyring-daemon man page source

KDE's, which changed shape recently:

> "This framework contains 3 main components:
> * Interface to KWallet, the safe desktop-wide storage for passwords on KDE work spaces.
> * **kwalletd exposes the KWallet API and proxies it to Secret Service, it can use ksecretd or any other Secret Service provider**
> * **ksecretd exposes the Secret Service DBus API** and is used to safely store the passwords on KDE work spaces."
> — KWallet README

> "ksecretd is the codebase that used to be kwalletd, but now exposes only the Secret Service API and it can be replaced with any other Secret Service provider."
> — same

A third implementation the user may install instead, with entirely different manners:

> "This feature allows KeePassXC to act as a Secret Service provider over DBus. … **Only one secret service provider can be enabled at a time. You may have to disable other providers, such as GNOME Keyring or KWallet, to use KeePassXC as a secret service provider.** … Open KeePassXC → Tools → Settings → Secret Service Integration → check Enable KeePassXC Freedesktop.org Secret Service Integration."
> — KeePassXC User Guide

Per desktop environment:

| Environment | What provides the Secret Service | Source |
| --- | --- | --- |
| GNOME | `gnome-keyring-daemon`, started automatically at desktop login | gnome-keyring man page |
| KDE Plasma (current) | `ksecretd`; `kwalletd` proxies the old KWallet API onto Secret Service | KWallet README |
| KDE Plasma (older) | `kwalletd`, which gained `org.freedesktop.secrets` support — **not established from KDE's own documentation which release first shipped it**; the KWallet README describes only the current split | — |
| Any, by user choice | KeePassXC, opt-in, exclusive | KeePassXC user guide |
| Headless / minimal window manager / server | **Not established.** The specification's own words are "a service running in the user's login session"; no page read here states either that a service exists by default outside a desktop session or that none does | Secret Service spec ch01 |

libsecret does document a non-D-Bus fallback backend, though only in passing:

> "The current implementation of file backend uses an encryption key derived from the user's login password. Security wise this not an ideal situation. Because, the entire security of the file backend relies on the user's login password (single point of failure)."
> — libsecret, TPM2

**Unresolved: how the libsecret file backend is selected at runtime, and whether it is available in a normal (non-Flatpak) build.** The rendered documentation read here describes the backend interface (`SecretBackend`) and the TPM extension, but no page read states the selection mechanism. **What would settle it:** libsecret's own documentation of its backend selection, or its source.

### Linux — what the user must do to grant access

The specification's answer is: whatever the implementation decides, delivered through a prompt object.

> "In order to complete various operations, such as unlocking a collection, the service may need to prompt the user for additional information, such as a master password.
> **The prompts are displayed by the service on behalf of the client application.** Operations that require a prompt to complete will return a prompt object. The client application must then call the `Prompt()` method of the prompt object to display the prompt. Client applications can use the `window-id` argument to display the prompt attached to their application window."
> — prompts

> "**Either the user or the client application can dismiss a prompt. In this case the operation that required the additional information is cancelled.** … The `Completed` signal will be emitted with its `dismissed` argument set to `TRUE`."
> — same

Locking is not something the client may assume away:

> "**A client application should always be ready to unlock the items for the secrets it needs**, or objects it must modify. It must not assume that an item is already unlocked for whatever reason.
> **A service may lock previously unlocked items for any reason at any time.** Usually this is done in response to user actions, timeouts, or external actions (such as the computer sleeping). The inherent race conditions present due to this are unavoidable, and must be handled gracefully."
> — unlocking

> "A service may choose to unlock items or collections just for a single client application. Alternatively the service may choose to allow any client application to access items or collections unlocked by another client application."
> — same

**In practice, per implementation.**

**GNOME** — the login keyring is unlocked by PAM with the login password:

> "`-l`, `--login` — This argument tells the daemon it is being run by PAM. It reads all of stdin (including any newlines) as a login password and does not complete actual initialization. … This option implies `--unlock`."
> "`--unlock` — Read a password from stdin, and use it to unlock the login keyring or create it if the login keyring does not exist."
> — gnome-keyring-daemon man page source

> "When a user logs into GNOME, the keyrings are locked and a master keyring password has to be provided in order to unlock each of them."
> — gnome-keyring, keyring-intro

**KDE** — same shape, with an explicit warning that it is not always configured:

> "By default a wallet named kdewallet will be used to store your passwords. **This wallet is secured by your login password and will automatically be opened at login, if kwallet_pam is installed and properly configured. On certain distros (e.g. Archlinux) kwallet_pam is not installed by default.** Other wallets have to be opened manually."
> — The KWallet Handbook

> "**A wallet is by default closed, which means that you must supply a password to open it. Once the wallet is opened, the contents can be read by any user process, so this may be a security issue.**"
> — same

**KeePassXC** — prompts on access when access confirmation is on, and prompts on *every* access for `secret-tool`:

> "If you enabled confirmation prior to access, you will see the following dialog … **When applications use secret-tool and you have access confirmation enabled, then you will be prompted each time credentials are requested. This is due to secret-tool obtaining a new process id each time it is run.**"
> — KeePassXC User Guide. *The source attributes prompting on every access to `secret-tool` getting a new process id per run; it does not say a long-running application is prompted on every access.*

> "Desktop notifications when a secret is retrieved and access confirmation dialogs."
> — same

### Linux — what happens when it is refused, moved or lost

The specification defines exactly **three** errors:

> "15.1 `org.freedesktop.Secret.Error.IsLocked` — The object must be unlocked before this action can be carried out.
> 15.2 `org.freedesktop.Secret.Error.NoSession` — The session does not exist.
> 15.3 `org.freedesktop.Secret.Error.NoSuchObject` — No such item or collection exists."
> — errors

libsecret's error enum, which is what a client actually sees through that library — **all nine members**:

> "**None of the errors are appropriate for display to the user.** It is up to the application to handle them appropriately."
> `SECRET_ERROR_PROTOCOL` "Received an invalid data or message from the Secret Service."
> `SECRET_ERROR_IS_LOCKED` "The item or collection is locked and the operation cannot be performed."
> `SECRET_ERROR_NO_SUCH_OBJECT` "No such item or collection found in the Secret Service."
> `SECRET_ERROR_ALREADY_EXISTS` "A relevant item or collection already exists."
> `SECRET_ERROR_INVALID_FILE_FORMAT` "The file format is not valid."
> `SECRET_ERROR_MISMATCHED_SCHEMA` "The xdg:schema attribute of the table does not match the schema name."
> `SECRET_ERROR_NO_MATCHING_ATTRIBUTE` "Attribute contained in table not found in corresponding schema."
> `SECRET_ERROR_WRONG_TYPE` "Attribute could not be parsed according to its type reported in the table's schema."
> `SECRET_ERROR_EMPTY_TABLE` "Attribute list passed to secret_attributes_validate has no elements to validate."
> — libsecret, Error enum

Search results are split into locked and unlocked, so "present but locked" is distinguishable from "absent":

> "The matched items will be returned in two sets. The `unlocked` return value will contain the object paths of all the items that are not locked. The `locked` return value will contain object paths of items that are locked, which can be unlocked if desired."
> — lookup-attributes

Absence, through the simple API, is not an error at all:

> "**If no secret is found then `NULL` is returned.**"
> — `secret_password_lookup_sync`

**Can "refused" be told apart from "unavailable" from "not there"? Partially:**

- **not there** → `NoSuchObject` / `SECRET_ERROR_NO_SUCH_OBJECT`, or a `NULL` return from the simple lookup. Distinguishable.
- **locked** → `IsLocked` / `SECRET_ERROR_IS_LOCKED`, or presence in the `locked` set. Distinguishable.
- **refused by the user** → the `Completed` signal with `dismissed` set to `TRUE`. Distinguishable, **but only if the application drives the `Prompt` object itself** rather than relying on a helper.
- **store unavailable** (no service running at all) → **not covered by the specification's error set, and not covered by libsecret's error enum**, and that much is still right. What arrives is a D-Bus-level failure to reach the name. **Narrowed 2026-08-21: this said the error was named nowhere. It is named.** **And the error is named, one layer down.** D-Bus's own protocol reference, `https://dbus.freedesktop.org/doc/api/html/group__DBusProtocol.html`, read 2026-08-20: `DBUS_ERROR_SERVICE_UNKNOWN` — *"The bus doesn't know how to launch a service to supply the bus name you wanted."*; `DBUS_ERROR_NAME_HAS_NO_OWNER` — *"The bus name you referenced doesn't exist (i.e. no application owns it)."* GIO's own reference, `https://docs.gtk.org/gio/error.DBusError.html`, read 2026-08-20, carries the same two as `G_DBUS_ERROR_SERVICE_UNKNOWN` (2) and `G_DBUS_ERROR_NAME_HAS_NO_OWNER` (3) — and GIO is the enum a libsecret client actually receives. The D-Bus specification, read the same day, on a call to a name nobody owns: *"Unless a message has the flag `NO_AUTO_START`, if the destination name does not exist then a program to own the destination name will be started (activated) before the message is delivered. … The message will be held until the new program is successfully started or has failed to start; in case of failure, an error will be returned."* **So it is one of two documented outcomes**, not an undocumented void: `ServiceUnknown` where the bus has no way to launch anything for the name — the stock-server case — and `NameHasNoOwner` where a name is referenced with no owner and no activation. **What is left, and it is narrow:** which of the two a client sees in a given situation, and what arrives when there is **no session bus at all** rather than no provider on it. **That wants a free virtual machine on the Creator's own Mac** — a minimal Ubuntu Server install, one libsecret call, record the error. **It was booked against a rented headless box and does not need one.** **It remains the single most likely condition on a machine without a desktop.** *(Two pre-correction clauses stood here until 2026-08-22, both contradicting the narrowing above them, which is 2026-08-21's. The first was **What would settle it:** the D-Bus reference implementation's documented activation errors plus a test on a headless Linux box — those errors are read and quoted above, and the residue that is left needs the free local virtual machine, not a headless box. The second was *"and it is precisely the one no read source names"* — it is named.)*

**Moved or lost.** Where the implementations tie the store to the login password, the consequences follow: GNOME's login keyring is unlocked by the login password via PAM, KDE's `kdewallet` "is secured by your login password". **Unresolved from GNOME's or KDE's own documentation read here: what happens to the stored secrets when the login password is changed by a route that does not go through PAM, or is reset by an administrator.**

KDE documents what its file is protected with, on the Blowfish path — the handbook offers two, "Blowfish encryption" and "GPG encryption":

> "KWallet saves this sensitive data for you in a strongly encrypted file, accessible by all applications, and protected with a master password that you define. The data is encrypted with the Blowfish symmetric block cipher algorithm, the algorithm key is derived from the SHA-1 hash of the password, with a key length of 156 bits (20 bytes)."
> — The KWallet Handbook

**New machine:** nothing in the specification or in the implementations' documentation read here provides any migration or sync. *(That "the secret stays on the machine" was cut — a positive statement no source carries; what the sources support is only the absence of documented migration in the pages read.)*

### Linux — session-only, never touching disk

**Linux is the only one of the three with a documented, first-class facility for this.**

> "An alias to the session collection, **which will be cleared when the user ends the session.** This can be passed to `secret_password_store()`, `secret_collection_for_alias()` or similar functions."
> — libsecret, `SECRET_COLLECTION_SESSION`

> "Use `SECRET_COLLECTION_SESSION` to store the password in the session collection, **which doesn't get stored across login sessions.**"
> — `secret_password_store_sync`

And the GNOME implementation states the disk property outright:

> "The program can manage several keyrings, each with its own master password, and there is also **a session keyring which is never stored to disk, but forgotten when the session ends.**"
> — gnome-keyring README

> "There is also a special 'session' keyring **which is not stored on disk and goes away when you log out.**"
> — gnome-keyring, keyring-intro

**Two limits, both material.** First, as on Windows, the documented lifetime is the **login session**, not the application; "dies when the application closes" is still the application's job. Second, **unresolved: whether KDE's `ksecretd` and KeePassXC implement the `session` collection alias, and with what semantics.** The alias mechanism is in the specification, which names only `default` as an example, but the memory-only guarantee is documented **only by GNOME**. **What would settle it:** KDE's and KeePassXC's own documentation of the `session` alias, or a test against each daemon.

### Linux — erasure, and whether erasure is confirmable

> "An item can be deleted by calling the `Delete()` method on the Item interface. **When deleting an item, the service may need to prompt the user** for additional information. In this case, a prompt object is returned. It must be acted upon in order for the item to be deleted."
> — ch03

Through libsecret:

> "Remove unlocked matching passwords from the secret service. … **All unlocked items that match the attributes will be deleted.**" Return value: "**Whether the any passwords were removed.**"
> — `secret_password_clear_sync` *(the source's own grammatical slip preserved)*

So erasure is confirmable — the call reports whether anything was removed — **but note the word *unlocked*: a locked item is not deleted by that call.** Erasing reliably means unlocking first, which may prompt.

### Observed — macOS only

Machine: the Apple-silicon Mac this work ran on, macOS as installed, user session active. Throwaway service names only. **No existing keychain item was read, listed, dumped, searched or printed, and no SSH key was touched.**

> **A gap in the record, sent back.** No macOS version is recorded for any of these runs, and no interpreter is named for the `ctypes` harness beyond "Python 3". Both are needed for the runs to be repeatable, and the disagreement below may turn on the second. The check's own repeat runs were on macOS 26.5.2 build 25F84, arm64, using `/usr/bin/python3` — recorded as the check's observation, not backfilled into the originals.

**Create and read back, from the binary that created it (file-based keychain).**

```
$ /usr/bin/security find-generic-password -s smoozy-architecture-probe-2026-08-16
exit 44
stdout: ''
stderr: 'security: SecKeychainSearchCopyNext: The specified item could not be found in the keychain.\n'

$ /usr/bin/security add-generic-password -a probe -s smoozy-architecture-probe-2026-08-16 -w probe-value-not-a-real-secret -U
exit 0
stdout: ''
stderr: ''

$ /usr/bin/security find-generic-password -s smoozy-architecture-probe-2026-08-16 -w
exit 0
stdout: 'probe-value-not-a-real-secret\n'
stderr: ''
```

**No dialog, no delay.** The binary that wrote the item read it back silently. `security` places the item in the login keychain, confirmed by the delete output below. Repeated by the check under a throwaway service name with the same three results and no dialog.

**Read the same item from a *different* binary, interaction disallowed.** Python 3 via `ctypes`, calling `SecKeychainSetUserInteractionAllowed(false)` then `SecItemCopyMatching` for the same service name:

```
SecKeychainSetUserInteractionAllowed(false) -> 0
file-based SecItemCopyMatching (interaction disallowed) OSStatus = -25293
data-protection SecItemCopyMatching OSStatus = -25300
```

Two things follow. First, **a binary other than the one that created the item is refused, and with UI suppressed the refusal surfaces as `errSecAuthFailed` — not as `errSecInteractionNotAllowed`, which is what the documentation's phrasing would lead one to expect.** *(The findings said "a binary that is not on the item's ACL"; the item's ACL was never read, so ACL membership is **inferred** from the outcome.)* Second, **the item written by `security` is invisible to a data-protection-keychain query: the two stores are genuinely separate.** Both status codes reproduced exactly.

#### The disagreement — write to the data protection keychain from a process without the entitlement

**This observation was run twice and came back differently the second time. Both runs are recorded. Neither is adjudicated. The behaviour is not established.**

**Ruled 2026-08-31: permanently unresolved.** The two runs cannot be told apart on the record kept — the interpreter, its code signature and the macOS build were never written down for the first run — so no side is picked, and **it will not be run again.** **What breaks:** this project keeps a written statement that it does not know whether a process without a keychain-access-group entitlement can write to the data protection keychain on macOS. Because that store is not used ([Security and licence](Security%20and%20licence.md) S5), nothing built depends on the answer; the cost is to the surrounding sentences only. **The guard, and it is the whole cost: no later reader may quietly recover the struck claim that every unentitled write is refused.** The observation that refuted it — a write that returned 0 and left an item that had to be cleaned up — stands on its own, whatever caused the two runs to differ. Not knowing the cause is not grounds for restoring the claim.

*The original run,* with the same `ctypes` harness, `kSecUseDataProtectionKeychain = true`, `kSecAttrAccessible = kSecAttrAccessibleWhenUnlockedThisDeviceOnly`, service `smoozy-architecture-probe-dp-2026-08-16`:

```
SecItemAdd data-protection keychain OSStatus = -34018
SecItemCopyMatching data-protection OSStatus = -25300
SecItemDelete data-protection OSStatus = -34018
SecItemDelete again (should be notFound) OSStatus = -34018
```

Read as: the data protection keychain refused both the write and the delete from a process lacking a keychain-access-group entitlement, while a *read* query returned plain `errSecItemNotFound`. Nothing was written, so nothing needed cleaning up.

*The repeat run,* 2026-08-16, on macOS 26.5.2 build 25F84, arm64, using `/usr/bin/python3` and `ctypes`, with `kSecUseDataProtectionKeychain = true`, `kSecAttrAccessible = kSecAttrAccessibleWhenUnlockedThisDeviceOnly`, `kSecClass = kSecClassGenericPassword`, `kSecValueData` set, **no `kSecAttrAccessGroup` specified**, service `smoozy-check38-throwaway-dp-2026-08-16`:

```
SecKeychainSetUserInteractionAllowed(false) -> 0
file-based SecItemCopyMatching (interaction disallowed) OSStatus = -25293
data-protection SecItemCopyMatching OSStatus = -25300
SecItemAdd data-protection keychain OSStatus = 0
SecItemDelete data-protection OSStatus = 0
```

and a follow-up query confirmed the probe item was written and then removed:

```
verify dp probe gone, SecItemCopyMatching OSStatus = -25300
SecItemDelete again OSStatus = -25300
```

**The file-based and read results agree exactly between the two runs; the write and the delete do not.** In the repeat, the write to the data protection keychain **succeeded** from a process with no keychain-access-group entitlement, and the item had to be cleaned up — the opposite of "nothing was written". **The behaviour is not established.** **What would settle it:** running it again with the interpreter, its code signature, and the macOS version all recorded, and, if the results still differ, with the difference between the two harnesses identified. Note that Apple's own `errSecMissingEntitlement` page scopes the error narrowly: "The method returns this error when you specify an access group to which your app doesn't belong."

**Everything that leaned on the original result has been unhooked from it:** the "observed consequence of missing that entitlement", the data-protection leg of the "caller not permitted" bullet, and the claim that without an entitlement "every write returns `errSecMissingEntitlement`".

#### The prompt — a call that hung

The same `SecItemCopyMatching` as above, but with user interaction left at its default (allowed):

```
calling SecItemCopyMatching with user interaction ALLOWED (default)...
[no further output]
```

**The call never returned. The process was killed by timeout at 45 seconds** (harness exit code 143, SIGTERM). Immediately afterwards:

```
$ ps ax -o pid,comm | grep -i SecurityAgent
37472 /System/Library/Frameworks/Security.framework/Versions/A/MachServices/SecurityAgent.bundle/Contents/MacOS/SecurityAgent
```

**Observed:** the call did not return within 45 seconds and was killed; a process named `SecurityAgent`, at a path inside `Security.framework`, was running immediately afterwards. **Inferred, not observed:** that a dialog was on screen — this agent had no view of the display and did not see it. **Also inferred:** that `SecurityAgent` is the component that displays keychain access dialogs, and that the call was waiting on a human.

*(The findings claimed the call "blocked indefinitely" and that "it blocks until a human answers". Neither is established: no human answered, no dialog was seen, and the call was terminated rather than allowed to complete, so whether it would ever have returned — on an answer, on a timeout, or not at all — is unknown. The run was not repeated: repeating it would put a system dialog on the user's screen and cannot complete inside a minute.)*

**What the documentation does carry, independently of this run:** an application that makes this call on its main thread hangs its own interface, exactly as `SecItemCopyMatching`'s "Performance considerations" warns.

#### Erasure, and verification of erasure

```
$ /usr/bin/security delete-generic-password -s smoozy-architecture-probe-2026-08-16
password has been deleted.
keychain: "/Users/artist/Library/Keychains/login.keychain-db"
version: 512
class: "genp"
attributes:
    0x00000007 <blob>="smoozy-architecture-probe-2026-08-16"
    "acct"<blob>="probe"
    "cdat"<timedate>="20260816143104Z\000"
    "svce"<blob>="smoozy-architecture-probe-2026-08-16"
    [remaining attributes NULL]
exit 0

$ /usr/bin/security find-generic-password -s smoozy-architecture-probe-2026-08-16
security: SecKeychainSearchCopyNext: The specified item could not be found in the keychain.
exit 44
```

**Deleted, and the deletion verified**, and reproduced by the check. The deletion echoed the item's attributes (not its secret) — **erasure on macOS is confirmable both by the delete's own output and by the subsequent not-found.**

### Unresolved for Fact 38

1. **macOS** — an explicit negative threat model for the keychain in Apple's own words. Apple states what it encrypts and who arbitrates, never what it fails to stop.
2. **macOS** — the maximum size of a keychain item's data.
3. **macOS** — whether a Developer ID–signed app shipped without an embedded provisioning profile can use the data protection keychain. *Settled by:* signing a minimal app both ways and calling `SecItemAdd` with `kSecUseDataProtectionKeychain`.
4. **macOS** — the `OSStatus` returned when a user clicks **Deny** in the ACL dialog. *Settled by:* re-running the blocked call with a human at the screen.
5. **CLOSED 2026-08-21.** The numerals are in Apple's own shipped SDK header `SecBase.h`, read 2026-08-20 on the Creator's Mac, and confirmed by `/usr/bin/security error` with a working control. The mapping is documented. See the paragraph above.
6. **macOS — PERMANENTLY UNRESOLVED, ruled 2026-08-31.** Whether `SecItemAdd` against the data protection keychain succeeds or returns `errSecMissingEntitlement` from a process without a keychain-access-group entitlement. **Two runs on the same machine disagree, and nothing recorded distinguishes them** — the interpreter, its code signature and the macOS build were not written down for the first run. **It will not be re-run and no side is picked.** Nothing rests on it (S5). **Guard:** the struck claim that every unentitled write is refused must never be quietly recovered — one write succeeded, and that observation stands whatever caused the difference.
7. **macOS** — what the blocked `SecItemCopyMatching` was actually waiting on, and whether it ever returns. Observed only that it did not return inside 45 seconds. *Settled by:* running it with a human at the screen who can report whether a dialog appeared and what happened on Allow, on Deny, and on leaving it untouched.
8. **macOS** — the version and build the probes ran on, and the identity of the Python 3 interpreter used. Neither is recorded, so none of the observed section is repeatable from its own description.
9. **Windows** — whether `CRED_PERSIST_SESSION` credentials ever touch disk. *Settled by:* an explicit Microsoft statement, or observation on a real installation.
10. **Windows** — the fate of DPAPI-protected data and Credential Manager items after an administrative password reset on a standalone (non-domain) machine with no Password Reset Disk; and the whole picture for Microsoft-account and Windows Hello logins, which post-date the DPAPI article.
11. **Windows** — whether the "Any program running as that user will be able to access credentials in this store" statement still holds for current Windows. It is quoted from an archived Windows Server 2012 R2 / 2012 page, and it is the single most load-bearing Windows sentence in this fact. *Settled by:* a current Microsoft statement on Credential Manager isolation.
12. **Windows** — *everything above*, in the sense that no Windows claim has been run against a real installation. No Windows machine has run it, and none is rented today.
13. **Linux — NARROWED 2026-08-21.** Neither the specification's three errors nor libsecret's nine-member enum names it, and that stands — but D-Bus and GIO both do: `ServiceUnknown` and `NameHasNoOwner`, quoted with their addresses in the store-unavailable entry. **What is left is which of the two arrives in a given situation, and what arrives with no session bus at all.** *Settled by:* a free virtual machine on the Creator's own Mac, one libsecret call. **No rental.**
14. **Linux — CLOSED 2026-08-21 on the vendor's own manifest.** Ubuntu's published server-image package list carries no provider and not even the client library. The question of what runs on a machine with no desktop session is answered for the image this project rents; a machine somebody has added a provider to is not this question.
15. **Linux** — which KDE release first exposed `org.freedesktop.secrets`, from KDE's own documentation.
16. **Linux** — whether `ksecretd` and KeePassXC implement the `session` collection alias, and with what disk guarantee. Documented by GNOME only.
17. **Linux** — how libsecret's file backend is selected at runtime and whether it is available in a normal build.
18. **Linux** — what happens to gnome-keyring and KWallet stores when the login password is changed outside PAM or reset by an administrator.

---

## Fact 39 — what each system offers for generating an SSH key pair, and whether it is present without the application installing anything

**The tool is the same on all three: `ssh-keygen`, from OpenSSH.** What differs is whether it is present by default.

### The interface — documented, and common to all three

> "ssh-keygen generates, manages and converts authentication keys for ssh(1)."

> "The type of key to be generated is specified with the `-t` option. If invoked without any arguments, ssh-keygen will generate an Ed25519 key."

> "Normally this program generates the key and asks for a file in which to store the private key. The public key is stored in a file with the same name but ".pub" appended. The program also asks for a passphrase. The passphrase may be empty to indicate no passphrase (host keys must have an empty passphrase), or it may be a string of arbitrary length."

> "The comment is initialized to "user@host" when the key is created, but can be changed using the `-c` option."

The options that make it non-interactive, and two more that bear on it, verbatim from the same page:

| Option | Verbatim text |
|---|---|
| `-t ecdsa \| ecdsa-sk \| ed25519 \| ed25519-sk \| mldsa44-ed25519 \| rsa` | "Specifies the type of key to create. The possible values are "ecdsa", "ecdsa-sk", "ed25519 (the default)," "ed25519-sk", "mldsa44-ed25519" or "rsa"." |
| `-C comment` | "Provides a new comment." |
| `-N new_passphrase` | "Provides the new passphrase." |
| `-f filename` | "Specifies the filename of the key file." |
| `-b bits` | "Specifies the number of bits in the key to create. For RSA keys, the minimum size is 1024 bits and the default is 3072 bits. … ECDSA-SK, Ed25519 and Ed25519-SK keys have a fixed length and the `-b` flag will be ignored." |
| `-q` | "Silence ssh-keygen." |

**The type list contains no `dsa`.** See the Windows discrepancy below.

**Passphrase handling for the "optional" case.** The DESCRIPTION says an empty passphrase means no passphrase, and `-N` "Provides the new passphrase." **That `-N ""` is therefore the way to generate an unprotected key non-interactively is *inferred* from those two sentences — the page never says `-N` accepts an empty string — and it is *observed* to work on this machine.** *(The findings labelled this documented; the check relabelled it, because everything downstream treats the two differently.)* `-N` takes the passphrase as a command-line argument, and the page offers no alternative on standard input — `ssh-keygen(1)` mentions standard input only for the signing options, and its whole ENVIRONMENT section is `SSH_SK_PROVIDER`. **But the passphrase can be kept off the command line entirely, through `SSH_ASKPASS` with `SSH_ASKPASS_REQUIRE=force`.** Run on the Creator's Mac, OpenSSH 10.2p1, 2026-08-20, with a two-line askpass helper in a scratch directory, **no `-N` and no `-P` anywhere on the command line**, standard input from `/dev/null`: `env -u DISPLAY SSH_ASKPASS=<scratch>/ask.sh SSH_ASKPASS_REQUIRE=force FAKEPASS='correct horse' ssh-keygen -q -t ed25519 -f <scratch>/k1 -C test@local` exited 0 and wrote the pair, and reading the key back through the same helper printed its public half at exit 0. **Three negative controls:** the wrong passphrase through the same helper gave `incorrect passphrase supplied to decrypt private key`, exit 255; askpass turned off with no terminal gave the interactive prompt and then the same failure; and `-P ''` asserted on the command line failed too — so the key is genuinely protected and the helper is genuinely what supplied the value. Label: **observed, not documented** — this page names neither variable. **`Keys` G3 builds the generator's passphrase delivery on this route, so a passphrase in `argv` is closed rather than merely deprecated.** *(Corrected 2026-08-22, and the ruling is 2026-08-21's: this sentence ended **which is a security consideration the architecture must rule on separately**, and it had already been ruled — the observation reached `Facts.md` Fact 39 and `Keys` §G3 that day and never reached this page, leaving a reader here at a hazard with no route out.)* *(That `-N` puts the passphrase in the process argument list follows from `-N` being a command-line option, which is the reader's step, not the manual's sentence.)*

Changing a passphrase later:

> `-p` — "Requests changing the passphrase of a private key file instead of creating a new private key. The program will prompt for the file containing the private key, for the old passphrase, and twice for the new passphrase."

and `-P passphrase` — "Provides the (old) passphrase."

**Generating a key does not put the public half anywhere it will be used.** ssh-keygen(1) FILES, on the `.pub` entry:

> "The contents of this file should be added to `~/.ssh/authorized_keys` on all machines where the user wishes to log in using public key authentication."

*(The findings originally cited a different FILES sentence — "This file is not automatically accessed by ssh-keygen but it is offered as the default file for the private key." — which is verbatim on the page but is about `ssh-keygen` not reading the default private key file of its own accord, and says nothing about what happens to a key after generation. The `.pub` sentence is the one that carries the claim.)*

### macOS — present, no installation required. Confirmed by observation.

**Observed** — macOS 26.5.2 (build 25F84), arm64:

```
$ sw_vers
ProductName:		macOS
ProductVersion:		26.5.2
BuildVersion:		25F84

$ which -a ssh-keygen ssh ssh-add ssh-agent
/usr/bin/ssh-keygen
/usr/bin/ssh
/usr/bin/ssh-add
/usr/bin/ssh-agent

$ ssh -V
OpenSSH_10.2p1, LibreSSL 3.3.6
```

`ssh-keygen` is at `/usr/bin/ssh-keygen`; nothing was installed to put it there. *("Part of the base system" was cut — nothing in the record distinguishes a base-system binary from any other binary at that path.)*

**Observed** — the type list this build will generate, from the usage banner (`ssh-keygen -V` used to force the usage text; it emits `ssh-keygen: option requires an argument -- V` first, then the block below):

```
usage: ssh-keygen [-q] [-a rounds] [-b bits] [-C comment] [-f output_keyfile]
                  [-m format] [-N new_passphrase] [-O option]
                  [-t ecdsa | ecdsa-sk | ed25519 | ed25519-sk | rsa]
                  [-w provider] [-Z cipher]
```

**`mldsa44-ed25519` is absent**, though the current OpenBSD manual lists it. This is the concrete proof that **the generatable type list must be read from the installed binary.** The banner reproduced character for character on re-run. *What it settles is what `-t` will create on this build; whether the build can **read** such a key is a separate question, sent back.*

**Observed** — a non-interactive generation in a scratch directory since deleted. Quoted as far as the fingerprint line; the real output continues with an eleven-line `The key's randomart image is:` block, omitted here:

```
$ ssh-keygen -t ed25519 -C "smoozy-test@example" -N "" -f <path>/k1
Generating public/private ed25519 key pair.
Your identification has been saved in <path>/k1
Your public key has been saved in <path>/k1.pub
The key fingerprint is:
SHA256:eu60QIDeNFacTTTHcbXJt4Z1HntBdYupFsJE65kv1wU smoozy-test@example
```

Resulting files and modes:

```
-rw-------@ 1 artist  wheel  411 Aug 16 16:28 k1
-rw-r--r--@ 1 artist  wheel  101 Aug 16 16:28 k1.pub
```

**`ssh-keygen` set `0600` on the private key and `0644` on the public key itself; the application does not need to chmod afterwards.** The `-C` comment appears verbatim in the fingerprint line and in the `.pub` file's third field. The re-run reproduced the modes, both file sizes (411 and 101 bytes) and the comment in both places exactly.

**Observed, and load-bearing** — `ssh-keygen` does **not** create missing parent directories:

```
$ ssh-keygen -q -t ed25519 -N "" -f <path>/nodir/sub/k3
exit=1
Saving key "<path>/nodir/sub/k3" failed: No such file or directory
```

**The application must create the target directory itself, with correct permissions, before generating into a path that does not yet exist.**

### Windows — `ssh-keygen.exe` ships in the OpenSSH **client**, which Microsoft's manufacturing documentation lists as preinstalled in a Windows image. Documentation only.

> **This heading said the opposite until 2026-08-21, and everything under it is corrected with it.** The reading below was taken from a table headed *SSH install state* which **does not distinguish the client from the server**, has no Windows 11 row, and whose Windows Server 2025 entry — *"Installed but not enabled"* — describes a state only a **service** can be in. The correction was the Creator's challenge, applied to `Facts.md` Fact 39 on 2026-08-20, and this file was not brought forward with it. **Two things follow.** The current source is Microsoft's *Available features on demand* page, `ms.date` 2026-02-20, monikers windows-10 **and windows-11**, read 2026-08-20: **OpenSSH Client sits under its own heading *Preinstalled FODs***, defined as features that *"come preinstalled in a Windows image"*, availability *"Windows 10, version 1709 and later"* — while **OpenSSH Server sits in that page's *FODs that are not preinstalled* section**, with *"Recommendation: Don't include this Feature on Demand on your image."* **And the page the table below came from no longer carries it**: that address now reads *"Get started with OpenSSH Server for Windows"*, `ms.date` 2025-09-04, split by version pivot, with no install-state table at all and one default-install sentence that is about the server — *"Beginning with Windows Server 2025, OpenSSH is now installed by default."* **The table below is kept as the record of what was read, not as a current source.**

Microsoft names the tool:

> "ssh-keygen generates, manages and converts authentication keys for SSH"

Availability, from the same page:

> "Beginning with Windows 10 build 1809 and Windows Server 2019, OpenSSH is available as a feature on demand."

> "The following table lists the default install state of OpenSSH for Windows on different Windows releases."

| Windows release | SSH install state |
|---|---|
| Windows Server 2019 | Not installed, install and enable using optional features |
| Windows Server 2022 | Not installed, install and enable using optional features |
| Windows Server 2025 | Installed but not enabled |
| Windows 10 build 1809 + | Not installed, install and enable using optional features |

(table reproduced verbatim; the page has no fifth row)

Installation, when absent:

> \# Install the OpenSSH Client
> Add-WindowsCapability -Online -Name OpenSSH.Client~~~~0.0.1.0

and the presence check:

> Get-WindowsCapability -Online | Where-Object Name -like 'OpenSSH*'

**On Windows the application must not assume `ssh-keygen.exe` is present — and the reason changed on 2026-08-21, though the rule did not.** The rule used to rest on the tool being absent by default, which is false: the OpenSSH **client** is preinstalled in a Windows image by Microsoft's own manufacturing documentation. **What the rule now rests on is that the manufacturing page speaks about the *shipped image*, and an OEM or enterprise image can have the capability removed** — so the tool is ordinarily present and cannot be assumed. It must probe for it and, if absent, tell the user how to add the OpenSSH Client optional feature — **and it must still never silently run the install command**, which is an administrative act. Microsoft's own wording: under Prerequisites, "An account that is a member of the built-in Administrators group."; and immediately before the install cmdlets, "Run PowerShell as an Administrator." *(The findings said the cmdlet "requires an elevated session per the same page"; the page never says that of the cmdlet. Corrected to what it does say.)*

Generation behaviour, per Microsoft:

> "To generate key files by using the ECDSA algorithm, run the following command in a PowerShell or Command Prompt window on your client:
> `ssh-keygen -t ecdsa`"

> "If you don't specify an algorithm, Ed25519 is used."

> "Next, you're prompted to use a passphrase to encrypt your private key files. In general, we don't recommend using an empty passphrase, because the passphrase works with the key file to provide two-factor authentication."

> "Enter passphrase (empty for no passphrase):
> Enter same passphrase again:"

**A documented contradiction, recorded rather than resolved.** Microsoft's key-management page says:

> "You can specify the following key-generation algorithms:
> Digital Signature Algorithm (DSA)
> Rivest–Shamir–Adleman (RSA)
> Elliptic Curve Digital Signature Algorithm (ECDSA)
> Ed25519"

`ssh-keygen(1)` on man.openbsd.org lists **no `dsa`** under `-t`, and the OpenSSH 10.2p1 binary observed on macOS does not offer it either. **Explained 2026-08-21, and the explanation is version skew rather than a fork difference.** OpenSSH removed DSA in 10.0 on 2025-04-09, having disabled it by default in 7.0 on 2015-08-11 (its own release notes, read 2026-08-20; quoted at the DSA entry above). **Microsoft's own current page shows the skew from the other side**: the `HostKey` defaults it publishes, on a page dated 2025-08-05, still list `ssh_host_dsa_key`. A Windows build carrying DSA-era defaults months after OpenSSH removed DSA upstream is what version skew looks like. **Whether the Windows `ssh-keygen.exe` actually accepts `-t dsa` is still not established** and needs a Windows machine, or Microsoft naming the OpenSSH version its build is based on. **The ruling under this does not change either way: the application must not offer a type merely because a documentation page lists it.**

Microsoft does not document `-C` or `-N` for the Windows build anywhere found. Since the Windows port is a fork of OpenSSH — Microsoft states "The OpenSSH open source project is managed by developers at the OpenBSD Project. The Microsoft fork of this project is in GitHub." — the `ssh-keygen(1)` options above are **expected** to apply, but **that expectation is not confirmed by Microsoft documentation and not confirmed against a real installation.**

### Linux — `ssh-keygen` ships in the client package, whose priority is the one Debian defines as installed by default. Documentation only.

> **This heading said the documentation makes no default-install claim, and that was false — corrected 2026-08-21, in the same act as the Windows half above and for the same reason.** The pages read were about the **server**, and an install command is what documentation gives whether or not the thing is already there. **What the packaging actually says**, read 2026-08-20: Debian's own package database lists `/usr/bin/ssh-keygen` in **`openssh-client`**, whose description names *"secure shell (SSH) client, for secure access to remote machines"* and enumerates `ssh`, `scp`, `sftp`, `ssh-agent`, `ssh-add`, **`ssh-keygen`**, `ssh-keyscan`, `ssh-copy-id`. **The packaging files for the openssh source in both Debian and Ubuntu carry `Priority: standard`**, with no override on the client binary, so the client takes it. **And Debian Policy defines that priority as the default selection:** `standard` is *"a reasonably small but not too limited character-mode system. This is what will be installed by default if the user doesn't select anything else."*, against `optional`, which the same page defines as the priority a package carries *"unless a package should be installed by default on standard Debian systems"*. **A negative result would have been a priority of `optional`.** It is not optional.

The tool and its options are the OpenSSH ones above, and Canonical names the same tool:

> "To generate a key pair, run the following command:
> `ssh-keygen -t ed25519`"

On availability, Canonical's own documentation gives an install command rather than asserting the client is preinstalled:

> "To install the OpenSSH client applications on your Ubuntu system, use this command at a terminal prompt:
> `sudo apt install openssh-client`"

> "To install the OpenSSH server application, and related support files, use this command at a terminal prompt:
> `sudo apt install openssh-server`"

Ubuntu-specific, and the same page documents Ubuntu's split configuration layout: "you can modify the main configuration file at /etc/ssh/sshd_config or add modular configuration snippets to the /etc/ssh/sshd_config.d/ directory." That affects the server side only, not key locations.

**Probe, do not assume — and the reason changed on 2026-08-21 while the instruction did not.** This ruling used to rest on *no document found states that `ssh-keygen` is present on a given Linux installation*, which is false for Debian and Ubuntu: their own packaging puts it in a package whose priority Debian defines as the default selection. **What the instruction now rests on is the difference between an archive's default selection and a particular machine**: a priority is a claim about what is installed when the user chooses nothing, not a guarantee about the computer in front of them, and a minimal container image or a trimmed appliance can lack it. The application must check for the binary and report its absence rather than fail obscurely. Since the application's target Linux is a *rented server* it connects **to**, and generation happens on the user's own machine, this matters only for users running the application itself on Linux.

### Unresolved for Fact 39

- **Whether Windows 11 is installed-by-default — CLOSED, 2026-08-20, by reading a different page.** This item observed correctly that the install-state table had no Windows 11 row and did not separate the Client capability from the Server capability; what it did not do was look elsewhere. Microsoft's *Available features on demand* page carries monikers windows-10 **and windows-11**, lists **OpenSSH Client under *Preinstalled FODs*** — *"come preinstalled in a Windows image"*, *"Windows 10, version 1709 and later"* — and puts **OpenSSH Server in the not-preinstalled section**. **So both of this item's complaints are answered by the same page**: Windows 11 is covered, and the client is separated from the server. **What is still not confirmed against a real installation is nothing this item asked**, but it remains true that no Windows machine has run it; the probe stands for the OEM-image reason rather than for this one.
- **Whether the Windows `ssh-keygen.exe` accepts `-C`, `-N`, `-t dsa`, and `-N ""`, and what permissions/ACLs it sets on the generated files.** Not documented by Microsoft; not testable here. **What would settle it:** running `ssh-keygen -t ed25519 -C "x@y" -N "" -f <scratch path>` and `ssh-keygen -Q key` on a real Windows installation and recording exit status, output, and the resulting ACLs via `icacls`.
- **Which Linux distributions ship `ssh-keygen` in their default desktop image — narrowed to the ones genuinely unread, 2026-08-21.** **Debian and Ubuntu are answered** by their own packaging: the client package carries `Priority: standard`, which Debian Policy defines as what is installed when the user selects nothing. What this item used to say — that Canonical's page gives an install command and makes no default-install claim — was reasoning from the wrong page, since an install command is not a statement of absence. **Red Hat and Fedora remain unread**: their documentation sites both refused automated retrieval, so no verbatim quotation from either could be taken and none is offered. **What would settle it:** reading those pages from a normal browser, or running `command -v ssh-keygen` on a clean installation of each target distribution.
- **The exact fallback behaviour when the private key file already exists.** Not probed, to avoid any overwrite test near real key material. **What would settle it:** generating twice to the same scratch path and recording the prompt and exit status.

---

## Fact 76 — what a key fingerprint is, which form is taken, whether it is stable across systems and key types, and whether two distinct keys can share one

### Documented

**Which hash, and the default.** The same statement appears on four pages:

> "Specifies the hash algorithm used when displaying key fingerprints. Valid options are: “md5” and “sha256”. The default is “sha256”."
> — ssh-keygen(1), `-E fingerprint_hash`

> "Specifies the hash algorithm used when displaying key fingerprints. Valid options are: “md5” and “sha256”. The default is “sha256”."
> — ssh-agent(1), `-E fingerprint_hash`

> "Specifies the hash algorithm used when displaying key fingerprints. Valid options are: md5 and sha256 (the default)."
> — ssh_config(5), `FingerprintHash`

> "Specifies the hash algorithm used when logging key fingerprints. Valid options are: md5 and sha256. The default is sha256."
> — sshd_config(5), `FingerprintHash`

**How a fingerprint is obtained and what it is for.**

> "When connecting to a server for the first time, a fingerprint of the server's public key is presented to the user (unless the option StrictHostKeyChecking has been disabled). Fingerprints can be determined using ssh-keygen(1):"
>
> "$ ssh-keygen -l -f /etc/ssh/ssh_host_rsa_key"
>
> "If the fingerprint is already known, it can be matched and the key can be accepted or rejected. If only legacy (MD5) fingerprints for the server are available, the ssh-keygen(1) -E option may be used to downgrade the fingerprint algorithm to match."
> — ssh(1), VERIFYING HOST KEYS

The words **"legacy"** and **"downgrade"** applied to MD5 are the manual's own, both in that one sentence; **that sentence is the only place any of the six pages compares the two fingerprint hashes.** Verified by string search for `md5`: the only other occurrences are the four `FingerprintHash`/`-E` option entries, which merely list it, and the `MACs` lists in ssh_config(5)/sshd_config(5), which are about message authentication codes, not fingerprints.

**A related but distinct construct that must not be confused with a fingerprint.** ssh(1) describes "random art" / `VisualHostKey`, and explicitly says it is not proof of identity:

> "Because these patterns are not unambiguous however, a pattern that looks similar to the pattern remembered only gives a good probability that the host key is the same, not guaranteed proof."
> — ssh(1), VERIFYING HOST KEYS

**This hedge is stated about the ASCII-art rendering, not about the fingerprint string.**

### Observed — OpenSSH 10.2p1, 2026-08-16

Fingerprints across key types, default hash:

```
$ ssh-keygen -l -f <tmp>/default
256 SHA256:Fd5lBZcFN0tswUHujbaBKYKRCth3UFWK5ovK+EQLf54 test@local (ED25519)
$ ssh-keygen -l -f <tmp>/plain_rsa
3072 SHA256:ULexf4Lilu7z7G0GnGoduVWgeaTTiTNbsoLdGbXoc8E rsa@local (RSA)
$ ssh-keygen -l -f <tmp>/plain_ecdsa
256 SHA256:jOELHfxRcnnazD7WqTjwk13B4aP6KPd46as7Hweu1bI ec@local (ECDSA)
```

Same keys with `-E md5`:

```
$ ssh-keygen -l -E md5 -f <tmp>/default
256 MD5:a8:50:f8:ba:3f:b0:a4:eb:7a:a3:b1:49:5d:77:41:5f test@local (ED25519)
$ ssh-keygen -l -E md5 -f <tmp>/plain_rsa
3072 MD5:d8:05:07:70:89:1f:6a:53:5b:bc:a7:37:f4:71:05:99 rsa@local (RSA)
```

**The output line is:** bit size, then `SHA256:` + unpadded base64 (or `MD5:` + colon-separated lowercase hex), then comment, then key type in parentheses.

**The same string is produced from the private and from the public file:**

```
$ ssh-keygen -l -f <tmp>/plain_rsa
3072 SHA256:ULexf4Lilu7z7G0GnGoduVWgeaTTiTNbsoLdGbXoc8E rsa@local (RSA)
$ ssh-keygen -l -f <tmp>/plain_rsa.pub
3072 SHA256:ULexf4Lilu7z7G0GnGoduVWgeaTTiTNbsoLdGbXoc8E rsa@local (RSA)
```

**What the fingerprint is computed over** was determined by **reconstruction, not from the manual**. Taking the second whitespace-separated field of the `.pub` line (the base64 key blob), base64-decoding it to raw bytes, and hashing those bytes reproduces both fingerprint forms exactly:

```
$ python3 -c "
import base64,hashlib
blob=base64.b64decode(open('plain_rsa.pub').read().split()[1])
print('SHA256:'+base64.b64encode(hashlib.sha256(blob).digest()).decode().rstrip('='))
print('MD5:'+':'.join('%02x'%b for b in hashlib.md5(blob).digest()))
"
SHA256:ULexf4Lilu7z7G0GnGoduVWgeaTTiTNbsoLdGbXoc8E
MD5:d8:05:07:70:89:1f:6a:53:5b:bc:a7:37:f4:71:05:99
```

**So, observed: the fingerprint is a hash of the wire-format public key blob** — the decoded bytes of the base64 field, which include the key-type string and the key parameters — rendered as unpadded base64 for SHA-256 and colon-hex for MD5. **It is not a hash of the file, not a hash of the base64 text, and not a hash of the comment.**

Two consequences follow directly: **the fingerprint does not depend on the comment** (changing the comment does not change it), and **it does not depend on the file's line endings or trailing whitespace.** The check exercised both on 10.2p1: after `ssh-keygen -c -C 'changed@local' -P ''` the `SHA256:` field was unchanged with the comment now `changed@local`, and appending three spaces and a CRLF to the `.pub` file left it unchanged again.

### Unresolved for Fact 76

- **Whether the fingerprint is stable across operating systems. The manual does not address this at all.** What the manual *does* establish is one ingredient: `sha256` is the documented default in all four places the hash is selectable — ssh-keygen(1) `-E`, ssh-agent(1) `-E`, ssh_config(5) `FingerprintHash`, sshd_config(5) `FingerprintHash` — **on the OpenBSD-current pages read; those pages say nothing about other platforms' builds.** The reconstruction shows the input is the decoded bytes of the base64 field of the `.pub` file, **but it ran on one platform and shows nothing about others.** No cross-OS observation was made. **What would settle it:** running `ssh-keygen -l -f` on the same `.pub` file on Windows and Linux builds of OpenSSH and comparing the strings; or the OpenSSH source. **One way the *displayed* string is documented to differ without the key differing:** a non-default `FingerprintHash` / `-E md5`. Compare fingerprints only after fixing the hash explicitly (`-E sha256` / `FingerprintHash sha256`) rather than relying on the default.

  **A second cause of divergence: an OpenSSH old enough to predate the SHA-256 default.** **Restored 2026-08-21.** It had been cut for want of a manual page, and OpenSSH publishes it in its own release notes — `https://www.openssh.org/txt/release-6.8`, read 2026-08-20: *"OpenSSH 6.8 was released on 2015-03-18."*, *"Add FingerprintHash option to ssh(1) and sshd(8), and equivalent command-line flags to the other tools to control algorithm used for key fingerprints."*, *"The default changes from MD5 to SHA256 and format from hex to base64."*, *"Fingerprints now have the hash algorithm prepended. An example of the new format: SHA256:mVPwvezndPv/ARoIadVY98vAC0g+P/5633yTC4d/wXE"* **So every OpenSSH before 6.8 defaulted to MD5 hex, and 6.8 moved the default to SHA-256 base64.** Label: **documented**. *(The cut's stated ground was that the manual pages do not carry it — true and irrelevant. Release notes are the project's own publication and are the same authority; `Method.md` now says so.)*

- **Whether the fingerprint is stable across key types.** Observed: the same `SHA256:`-prefixed, unpadded-base64 form is produced for Ed25519, RSA and ECDSA, and **in the observations recorded here the base64 field is 43 characters in every case**, so the observed string length did not vary by type. **The manual does not state this.** *(The findings reasoned from "the SHA-256 digest is fixed-length regardless of key type" — a property of the hash function that no cited source states; replaced with the character count actually present in the recorded observations.)* Note that the key type does affect the *rest* of the output line — the leading bit count and the trailing `(ED25519)` / `(RSA)` / `(ECDSA)` — so **any parser must take the `SHA256:...` field specifically, not the whole line.**

- **Whether two distinct keys can ever share one fingerprint. The manual does not state this, anywhere, for either hash.** It says nothing about collision resistance, uniqueness, or the probability of two keys hashing alike. String search over the six pages: `collision` count zero in all six; the `uniqu*` hits are unrelated. **The only uniqueness hedge on any of the six pages is the one about *random art* patterns being "not unambiguous", which is a statement about the ASCII-art rendering and cannot be transferred to the fingerprint string.** **No answer is offered here by reasoning from properties of hash functions, because that would be a guess dressed as a fact.** **What would settle it:** the cryptographic literature on SHA-256 and MD5 collision resistance, and the OpenSSH source for how the digest is constructed — neither of which is OpenSSH's own documentation and neither of which was consulted.

---

## What the user is made to do, or made to risk

Drawn from the Fact 38 material, stated flatly, per operating system, in three terms: **a place to look**, **something to type**, **a thing that can be lost**. No recommendation is made here.

### macOS

**A place to look.** Keychain Access, in /Applications/Utilities — Apple Platform Security: "It can be accessed by opening the Keychain Access app in /Applications/Utilities." The file-based login keychain and the data protection keychain are both shown, the latter as "Local Items" or "iCloud Keychain". Apple documents deleting a **whole keychain** from that app. *(That every item is visible there item by item, and that the user can delete an individual item an application wrote, was cut: TN3137 says the keychain list shows the file-based keychains and the data protection keychain; no page read says the rest.)*

**Something to type.** Two occasions, both real:

- *The keychain access dialog.* If the application targets the file-based keychain, a read by an untrusted caller puts a dialog in front of the user, answered with Allow Once, Always Allow, or Deny. Apple states that trust is lost when the app "is changed or infected by a virus", and that the dialog "appears if you recently updated your system software or the app, or if the app has been modified" — with the qualification, on the same page, that "You must sometimes reauthorize updated apps". **Observed as a call that did not return within 45 seconds and had to be killed.** *(That it blocks for as long as the dialog goes unanswered was cut — no dialog was seen and the call was killed, not answered.)*
- *The keychain unlock dialog.* If the login keychain has locked — inactivity, sleep, or password out of sync — the user types the keychain password before the read completes.

If the application targets the data protection keychain instead, **neither dialog is documented** — at the price of shipping an entitlement authorised by a provisioning profile inside an app bundle. *(What that costs if the entitlement is absent is the disputed observation: two runs disagree, and the question is open.)*

**A thing that can be lost.** Yes, and **macOS is the harshest of the three here.** If an administrator resets the login password, or the user forgets it and resets it, the keychain does not follow: "the keys, passwords, and other information in the keychain won't be accessible, and the user will need to create a new keychain". A failed unlock at login can reset the keychain outright, and "Resetting the default keychain deletes all the passwords saved in the keychain". On a new machine, items stored `ThisDeviceOnly` are simply not there. **No page read here documents a domain escrow or a recovery-disk equivalent**; what Apple does document for a deleted keychain is that "if you know the keychain's password, you can recover the information" and that the file can be restored from Time Machine, and, separately, that `kSecAttrSynchronizable` makes updates and deletes affect "all copies of the item, not just the one on your local device".

**Session-only.** No OS facility for it is documented. *(That it must therefore be held in the application's own memory is the writer's conclusion from that absence, not a source statement.)*

### Windows — documentation only, not confirmed on a real installation

**A place to look.** The Credential Manager control panel applet. Microsoft documents that "Users may choose to save passwords in Windows by using an application or through the Credential Manager Control Panel applet." *(No page read here says the store's contents are visible or editable there.)*

**Something to type.** **Nothing documented, on the `CRED_TYPE_GENERIC` path.** No documentation read here describes any prompt for `CredWrite`, `CredRead`, `CredDelete`, or for DPAPI called without a prompt structure. The one interactive mechanism DPAPI had is deprecated and, in Microsoft's words, "will be removed in **February 2027**", with the pointed warning that "operations on data originally protected with the PromptStruct flow will fail". **Windows is the quiet one on this path.** *(That the user is never asked, first time or any time, was cut — `CRED_FLAGS_PROMPT_NOW` documents an application-driven prompt for the two domain credential types.)*

**The price of that quiet** is the sentence in Microsoft's own overview: "**Any program running as that user will be able to access credentials in this store.**" There is **no per-application boundary**. Any other program the user runs can read what the application stored. No page read here documents a Windows counterpart to macOS's ACL dialog or entitlement check; optional DPAPI entropy is the nearest thing the sources name, and it is a secret the application must itself keep somewhere. **Note its source is an archived Windows Server 2012 R2 / 2012 page.**

**A thing that can be lost.** Less easily than on macOS. A normal password change re-encrypts all master keys, and a Credential History file lets DPAPI walk back through old passwords. A domain machine escrows the master key to a domain controller; a standalone machine can recover through a Password Reset Disk — **introduced on that page as "Windows XP has a new feature", in an article dated October 2001 and flagged archived.** But: an administrative reset on a standalone machine with no PRD is **not established** by the documentation read here, and neither are Microsoft accounts or Windows Hello, which post-date the article. On a new machine the secret is gone unless the account has a roaming profile and the credential was written `CRED_PERSIST_ENTERPRISE` — and even then, "if the user has no roaming profile, the credential will only persist locally."

Also: **2560 bytes per credential blob**, and a **DPAPI decrypt failure that cannot be reliably classified from its error code.**

**Session-only.** `CRED_PERSIST_SESSION` exists and dies at logoff, but its documented lifetime is the logon session, not the application, and **whether it touches disk is not established.** For in-memory encryption Microsoft points at `CryptProtectMemory`: "To encrypt memory for temporary use in the same process or across processes, call the `CryptProtectMemory` function."

### Linux

**A place to look.** Whichever the user has: `seahorse` (named by the gnome-keyring-daemon man page and by the KeePassXC guide), KWallet Manager (the KDE handbook), or the KeePassXC window. **The application cannot know which, and one of them may not exist at all.**

**Something to type.** Anywhere from nothing to a password on every single access:

- GNOME with PAM configured, KDE with `kwallet_pam` configured: **nothing.** The keyring or wallet unlocks with the login password at login.
- KDE where `kwallet_pam` is not installed — KDE names Arch Linux explicitly — the wallet is closed by default and the user types the wallet password to open it.
- KeePassXC as the provider: the user must first enable the integration by hand, may have to disable the other provider first, must have the database unlocked, and with access confirmation on sees a confirmation dialog — for `secret-tool`, "prompted each time credentials are requested", which the guide attributes to `secret-tool` "obtaining a new process id each time it is run".
- Any implementation, at any time: "A service may lock previously unlocked items for any reason at any time."

**A thing that can be lost.** The store is local, **with no documented migration to a new machine.** The specification guarantees **no access control at all**, so on a shared login another program of the same user may read the secret — KDE says so outright: "Once the wallet is opened, the contents can be read by any user process, so this may be a security issue." What happens to the stored secrets when the login password is reset out from under the keyring is **not established** by either project's own documentation.

**And the thing that can be absent.** This is Linux's distinctive cost and it belongs in the same list: the two desktop implementations document start-up at *desktop login*, and **no source read here says what runs, or fails to run, on a machine with no desktop session** — a headless box, a bare window manager, a remote shell. **The error the application receives when no provider is running is named by neither the specification nor libsecret's error enum.** **Answered 2026-08-21 in a narrower and better form.** Canonical publishes what its server image contains, and `https://releases.ubuntu.com/24.04/ubuntu-24.04.3-live-server-amd64.manifest` (read 2026-08-20) carries **no `gnome-keyring`, no `ksecretservice`, no `kwalletmanager` and no `libsecret-1-0`**, while `dbus` is present. **So on that named image a session bus exists and nothing can answer for `org.freedesktop.secrets`.** And the error **is** named, one layer down: D-Bus's `ServiceUnknown` and `NameHasNoOwner`, the same two in GIO's enum, which is what a libsecret client receives (addresses in the store-unavailable entry above). *(The cut sentence was a positive claim about headless machines in general; this one is about a published image.)*

**Session-only.** The only one of the three with a documented, memory-only facility — the `session` collection, "never stored to disk, but forgotten when the session ends" (GNOME). Its guarantee is documented by GNOME only; whether `ksecretd` and KeePassXC honour it is **not established**.

---

## Questions sent back

Everything the four citation checks could not settle and returned to be asked again. Each fact's own unresolved items stay in its section above; these are the questions raised *by the checking*.

### From the check of Facts 32, 35, 36 and 76

1. **ANSWERED 2026-08-21 and struck.** From OpenSSH's own release-6.8 notes: MD5 hex was the default before 6.8 (2015-03-18), SHA-256 base64 from it. Restored to the divergence bullet above with the quotation and address. *(It was parked because release notes "are not manual pages". They are the same authority.)*
2. **How were `plain_rsa`, `plain_ecdsa` and `<tmp>/ca` generated?** Their `ssh-keygen` invocations appear nowhere, though observations in Facts 35, 36 and 76 depend on them. **`<tmp>/ca`'s key type is not recoverable from anything written down.**
3. **Does the `-P ''` exit-status technique distinguish only "has a passphrase" from "has none", or does exit 255 also arise from unreadable or malformed key files?** Two keys were tested, one of each kind. *(Fact 37's route-A table bears on this and points to yes — `invalid format`, missing file and `bad permissions` all give 255 — but it is a different sitting and the question was sent back regardless.)*
4. **Whether `BatchMode=yes` suppresses the private-key passphrase prompt.** Unchanged and still open. Needs the OpenSSH source or a test against a real server.

### From the check of Facts 33, 34 and 39

5. **Where the `known_hosts` fixture in the Fact 34 probe came from.** The recorded line is `github.com`'s ED25519 host key, and the findings state nothing under the user's `~/.ssh` was read. The observation cannot be repeated from its own description until that step is written down. The behaviour it demonstrates reproduces with a fixture built from a throwaway key, so what is in question is the record, not the result.
6. **Whether `ssh-keygen -y` without `-P ""` prompts on the terminal.** Asserted in the Fact 34 findings, absent from `ssh-keygen(1)`, and not recorded there as an observation. Needs either a citation or a run with its command, output and exit status written down. *(Fact 35's material does record such a run — `Enter passphrase for "<tmp>/locked": …`, exit 255, stdin `/dev/null`, no TTY — but again in a different sitting.)*
7. **Whether `authorized_keys` fingerprints at exit 0 the way `known_hosts` does.** Asserted by analogy; never run. One probe would settle it.
8. **Whether the installed OpenSSH 10.2p1 can *use* an `mldsa44-ed25519` key it did not generate.** The usage banner settles only what `-t` will create. If it matters what key types the build can *read*, that is a separate probe (`ssh-keygen -l -f` against such a key, or `ssh -Q key`).
9. **ANSWERED 2026-08-21 and struck.** `~` and `%d` are OpenSSH's *local user's home directory*, documented in `ssh_config(5)`'s TOKENS, and `${HOME}` is a separate documented syntax for the variable — so this is OpenSSH's behaviour rather than macOS's. Run with a control on the Creator's Mac; see the Fact 33 entry. *(This pointed at **the Fact 35 entry** until 2026-08-22; the entry it means is in Fact 33's unresolved list.)*
10. **ANSWERED 2026-08-21 and struck.** Microsoft's USMT environment-variable page equates `USERPROFILE` with `CSIDL_PROFILE`, *"The user's profile folder"*, and calls `C:\Users\<username>` *"a typical path"*. Same directory, unguaranteed string — expand the variable. The separate `$HOME`-versus-`%USERPROFILE%` question is untouched.

### From the check of Fact 37

11. **ANSWERED 2026-08-21 and struck.** `PROTOCOL.key` — OpenSSH's own format file — gives the container's only integrity signal as two matching `checkint` fields, with no message authentication code. **The format offers no way to separate them**, and an authenticated cipher does not change the message. See the Fact 37 entry.
12. **Whether a `PerSourcePenalties` lockout is textually separable from an ordinary immediate close.** The two runs differ by the presence of a `kex_exchange_identification:` line, which the findings then called indistinguishable. Needs both cases re-run side by side, at default level and at `-vv`.
13. **Whether the three askpass invocations on route B are `NumberOfPasswordPrompts` or a coincidence.** Settled by setting the keyword to some other value and re-counting.
14. **ANSWERED 2026-08-21 and struck.** `ssh-add.c`'s `add_file()` loop is `for (;;)` with no counter and two exits — an empty passphrase, or a successful parse. Unbounded, from OpenSSH's own source.
15. **What route B does with a mode-0644 key file**, at default level and at `-vv`. The summary table asserted an answer nobody had run.
16. **Whether the wrong-passphrase marker appears at `LogLevel VERBOSE`** — the level between INFO and DEBUG1 was never sampled, nor was `LogLevel` set directly rather than through `-v` flags.

### From the check of Fact 38

17. **ANSWERED 2026-08-21 and struck.** Apple's shipped `SecBase.h` publishes the whole mapping, and `/usr/bin/security error` resolves each number with a control that fails on an unknown one.
18. **Whether `SecItemAdd` against the data protection keychain succeeds or returns `errSecMissingEntitlement` from an unentitled process.** **Two runs on the same machine disagree.** Needs running again with the interpreter, its code signature and the macOS version recorded — and, if it still differs, with the difference between the two harnesses identified.
19. **What the blocked `SecItemCopyMatching` was waiting on, and whether it ever returns.** Needs a human at the screen who can report whether a dialog appeared and what happens on Allow, on Deny, and on leaving it untouched. **This is the claim that decides how the file-based path is described, and it is currently carried by an inference.**
20. **The macOS version and the Python 3 interpreter behind the observed section.** Without them none of those observations is repeatable from its own description.
21. **REDUCED 2026-08-21 to one thing, and it does not need a rented machine.** Whether a provider runs on a stock server image is **answered**: Canonical's own published manifest lists none, and no client library either. What remains is which D-Bus error arrives — `ServiceUnknown` or `NameHasNoOwner` — and what arrives with no session bus at all; a free virtual machine on the Creator's Mac settles both. *(The findings' original sentence was a guess dressed as a fact and was right to be cut; the manifest is the fact.)*
22. **Whether "Any program running as that user will be able to access credentials in this store" still holds for current Windows.** The load-bearing Windows sentence in the whole fact, and its only source is an archived Windows Server 2012 R2 / 2012 page.

---

## Two places where the four findings files disagreed with each other

Recorded because assembly is where such things are found, and neither check saw them — each checked one file.

1. **Where the comment sits in an encrypted private key file.** The Fact 34 material states the comment "is stored inside the encrypted portion". The check of the Fact 35 material struck that same claim there, on the ground that the observation shows only what is and is not *recoverable*, not where in the file the comment sits. **The narrower reading stands throughout this document: not recoverable without the passphrase, location unestablished.**
2. **What `ssh-keygen -y -P ''` exit statuses mean.** The Fact 34 material states the rule generally — exit 0 means no passphrase, exit 255 with that message means passphrase-protected. The check of the Fact 35 material narrowed exactly that generalisation to the two keys run there and sent the question back. **The narrowed reading stands**, and Fact 37's route-A table shows exit 255 arising from at least three further causes on the same build, separated only by the message text — which is undocumented.

Neither disagreement is adjudicated beyond taking the narrower of the two readings in each case, which is what both checks did within their own files.
