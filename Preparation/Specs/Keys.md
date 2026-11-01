# Keys

*What the Keys module does, in plain terms: how the user creates and manages the SSH keys Smoozy uses to reach servers. Not one word about how it's built.*

## Where Keys fits

The user rarely opens Keys on purpose. An SSH key (the standard way one machine logs into another over a network) is made for a provider once — on a given computer — and then forgotten. So most key work happens **inside the Add Server dialog**: the user is adding a server, needs a key for it, and makes one right there. Keys is not a place they return to on a schedule.

There is still a small standalone keys view for the occasional housekeeping. It shows the keys Smoozy knows about, which is every key found in the operating system's usual key location plus every key the user has pointed Smoozy at. From there the user can make a new key, copy a key's public half, point at a key kept elsewhere, point an unusable key at its new location or at a replacement, clear a key's remembered passphrase, delete a managed key, or remove a referenced key from Smoozy.

**Connections.** Keys hands keys to Servers — the Add Server and Edit Server dialogs pick which key a server uses — and is most often opened from one of those dialogs. Keys also reads from Servers which saved servers rely on a given key, so the delete/remove warning can name them. The full wiring between modules is left on the [module map](Whole%20app.md), which also holds that Keys works only on the local machine and never talks to a server itself, and which settles who supplies the passphrase when a server's key is protected by one. One consequence of the local-machine limit falls to the user: giving the public key to the provider is their own step, listed under [What Keys does not do](#what-keys-does-not-do).

When Keys is opened from the Add Server or Edit Server dialog, the key the user settles on is **handed back to the dialog and selected** for that server. That key may be one they have just made, one they have pointed Smoozy at from elsewhere, or one already in the list. Pointing at a key kept elsewhere goes through Keys in both cases: the dialog opens Keys to do it, rather than picking the file itself. If the user cancels the key work, they return to the dialog with what they'd already filled in still there. The rest of the hand-off is the [module map](Whole%20app.md)'s and is settled there: unusable keys are never offered, the server keeps the key's fingerprint rather than its name or its file, and where the key has a passphrase Keys supplies it while Servers makes the connection.

## Making a key

Wherever it's started — from a server dialog or from the standalone view — creating a key is the same. The user gives:

- **A name.** This is the key's *comment* — the human-readable tag SSH keeps to identify a key. By default it is the name the user goes by on this computer, joined to the computer's own name, and it travels with the public half. *(documented — see [Sources](Sources/Keys.sources.md))* The public half is the half that gets put on the machine the user wants to log in to. *(documented — see [Sources](Sources/Keys.sources.md))* For Smoozy that machine is the rented server, so the public half is the one the user later hands over.

  Smoozy **suggests naming the key after the provider it is being made for**. A key is made for a provider once and then used for every machine rented there, so the name is the only thing that later says which key belongs where — the fingerprint tells two keys apart but says nothing about which provider holds which. Without that, the user picks between keys in the Add Server dialog with nothing to go on, and a wrong pick is not caught until the connection fails. The suggestion is meant to be prominent rather than tucked away, and it stays a suggestion: the user can type anything, and the default stands if they leave it. This is one of the places Smoozy speaks in provider terms without knowing any provider (see the [module map](Whole%20app.md)). Label: **Creator-stated** (2026-07-27). Settled at Design (2026-08-23): the name field leads the dialog and the suggestion sits under it as a sentence rather than a rule — *Name it after the provider you are renting from* — with the default already in the field.
- **A passphrase, optionally.** A key can be left without one, or protected with a passphrase (see [Passphrases](#passphrases)).
- **The key type.** Smoozy picks a sensible modern default, so the user can just accept it; an advanced choice lets them pick a different type.

Smoozy generates the private and public halves and stores them where the operating system keeps SSH keys. Each operating system Smoozy supports keeps its keys in its own place (the supported platforms are listed in the [module map](Whole%20app.md)). The exact locations, and the premise that each supported OS has just one of them, are outside facts to be confirmed and sourced in Architecture per Rule 2 — not asserted here. The user can then **copy the public half to the clipboard** to paste at their provider.

**On a computer that does not have what Smoozy needs, Smoozy makes no key and checks none, and says so.** Making a key and checking one both rest on a small program that comes with the operating system. Almost every computer has it; a few do not — a stripped-down Linux, or a work machine built to somebody else's recipe. Where it is missing, Smoozy says so plainly and once, where the user is looking: it makes nothing, checks nothing, and **does not call the keys the user already has broken**, because they are not. The user is not asked to install anything, and may not be allowed to. What is said, and where, is Design's. Label: **Creator-stated** (2026-08-25).

The name the user types sets both the comment and the key's filename: a key named *hyperstack* is stored as `hyperstack` and `hyperstack.pub`. Label: **Creator-stated** (2026-10-08, sixth notes, note 6). Keys already made are not renamed. The rest of the filename scheme is an Architecture detail, but one rule holds at this level: a key file is a local file, so the whole-app promise that Smoozy never silently overwrites one covers it (see [Whole app](Whole%20app.md)). How a collision is resolved is part of the filename scheme left to Architecture. Settled at Design (2026-08-23): it is never shown, because the scheme takes the next free number rather than asking; the file's name appears on the row and at the end of the made-key dialog.

## Using a key that already exists

A key is **usable** when it is a supported type that Smoozy can read and verify. A key is **unusable** when its file is missing, unreadable, not a supported type, or no longer the same key Smoozy knew. How each view treats an unusable key depends on where the user is looking.

The user may already have keys. Ones the operating system holds in its usual place show up in Keys on their own. For a key kept somewhere else, the user **points Smoozy at it** by selecting its **private key file** — the private half is the one needed to connect, so that's what the user points at *(documented — see [Sources](Sources/Keys.sources.md))* — and Smoozy references it where it sits: it does not move or copy the file.

For any key Smoozy knows — whether found in the usual place or pointed at — copying the public half stays available when the public half is present alongside the private one. When only the private file is present, Smoozy does not go looking for a public half that isn't there, and copy stays unavailable until a public file is present. **For a key with no passphrase that is the whole of what the missing public file costs: the key still works and only the copy is gone. For a key with a passphrase it costs more — Smoozy reads a protected key's fingerprint from the public file, so with that file missing it cannot read the key at all, and the key is unusable until the public file is beside it.** Smoozy says which of the two it is met, rather than reporting a fault in a file that has none. *(Narrowed on 2026-09-03, out of the `Keys` code round 1, on the consolidator's ruling after the question was put to the Creator and handed back. No promise is added and nothing new is built; what changes is that a promise which was always false for this one case now says so. No auditor has seen this change.)*

Pointing only adds a key Smoozy can use now, while the usual-location list shows everything found there, usable or not, so broken managed keys are visible rather than silently absent. If the user points at something that isn't a usable key, Smoozy says so plainly and does not add it.

A key's file can change outside Smoozy at any time. It can be moved, deleted, become unreadable, or be swapped for a different key. A referenced key sits outside Smoozy, so this happens to referenced keys more than to any other kind.

So **Smoozy checks a key whenever something asks for the check** (these are the only things that ask). It does not watch keys in the background. The standalone keys view asks whenever it is on screen, so an unusable key shows as unusable before it is relied on. The server list asks whenever it is on screen too. And Servers asks again when it connects, using the check that Keys supplies — Keys provides the check, Servers runs it at that moment.

Where the computer is missing what the check needs, no key is marked unusable on that account — the keys are fine and the check could not run (see [Making a key](#making-a-key)).

When a key becomes unusable, that mark stops a different key being used in its place without the user knowing. How each view treats the mark differs:
- In the **standalone keys view**, an unusable key is shown as unusable rather than hidden.
- The **Add Server dialog** never offers unusable keys.
- The **Edit Server dialog** offers no *other* unusable key, but may keep the server's current key selected even when unusable, as settled in the Servers spec.

For an unusable key, Smoozy offers to point at the key's new location, or at a replacement key:
- **New location:** Must be the same key (the fingerprint matches).
- **Replacement:** A different key (new fingerprint). The remembered passphrase does not carry across, and servers still holding the old fingerprint show as needing a new key per the Servers spec. Settled at Design (2026-08-23): it is one of the two ways out on an unusable key's own row, and it says what it costs before it is pressed — the remembered passphrase goes, and every server holding the old key needs a new one.

Existing keys protected with a passphrase are supported the same as ones Smoozy made itself — **on macOS and Linux, and where the key's public file sits beside it**. Two limits sit under that, and both are stated here rather than met as a surprise:

- **A key with a passphrase needs its public file beside it**, as [above](#using-a-key-that-already-exists). Without it Smoozy cannot read the key at all.
- **On Windows, Smoozy cannot make a key with a passphrase and cannot use one.** The way a passphrase is handed to the computer's own SSH tools has never been arranged on Windows, and Smoozy refuses rather than guessing at it, so the act is reported as failed. Everything that needs no passphrase works there as it does anywhere: keys are listed, checked and made, and a key with no passphrase connects.

*(Both qualifiers added on 2026-09-03, out of the `Keys` code round 1, on the consolidator's ruling after the questions were put to the Creator and handed back. **Neither is a promise that anything is coming**, and nothing new is built: what changes is that a sentence which was already false on Windows and for that one class of key now says so. No auditor has seen this change.)*

## Telling keys apart

A key's name is its comment, and it can repeat — two keys made on one machine that both take the default name carry the same name, though the second one's files take a number — and Smoozy never changes it after creation (see [What Keys does not do](#what-keys-does-not-do)). So the name alone can't reliably identify a key.

Smoozy also shows each key's **fingerprint** — a short code effectively unique to every key — so keys that share a name can still be told apart, by both the user and Smoozy. *(documented — see [Sources](Sources/Keys.sources.md))*

While Keys knows a key, it keeps that key's fingerprint and uses it to tell the key apart and detect a swapped file. It drops the fingerprint only when the key is deleted or removed from Smoozy.

The fingerprint is also what identifies a key as **the same key over time** — including one that was removed from Smoozy entirely and later pointed at again. After a key has left Keys, only a saved server that still holds the fingerprint can recognise the key when it is pointed at again (see Servers) — Keys itself cannot.

If the user points at a key whose fingerprint Smoozy already knows, Smoozy refuses the second reference with a plain message and keeps one entry.

If a new or newly referenced key would share the name of one Smoozy already knows but has a **different fingerprint**, Smoozy points that out, since the two are different keys under one name.

When the shared name is the default one, Smoozy stays quiet: this assumes two keys sharing the default name are two ordinary keys made on one machine, not a clash the user needs to hear about — and Smoozy cannot know that for certain.

Settled at Design (2026-08-23): the fingerprint sits on the row in the fixed-width face and stays there while a key is unusable, being the only thing that can say whether a file the user points at is the same key; the duplicate-name warning is a statement when the second key arrives, naming the other key's fingerprint, and never a refusal.

## Passphrases

A passphrase is optional. **Smoozy remembers one the moment it is entered, wherever it is entered — including as the key is being made.** A user who sets a passphrase on a new key is not asked for it again at the first connection — **wherever this computer has somewhere to remember, and the remembering goes through**. Where the computer has nowhere, the [Whole app spec](Whole%20app.md)'s security concern sets that out per system; where the remembering fails, nothing is held, and the first connection asks exactly as it does for any passphrase Smoozy is not holding. *(Qualified in two steps on 2026-08-29 by the architecture runner, on the Creator's delegation; **no promise is added and no mechanism is invented.** The sentence was flat where the arrangement has always been conditional in two cases: a computer with nowhere to remember, where a connection asks every time for the life of the installation, and a save that fails, where the passphrase is simply not held. The first step added the pointer this section had never carried; the second added the failed save, which the pointer does not reach and which the architecture states outright. No auditor has seen either change.)* On a connect the user starts, if the key needs a passphrase Keys is not holding, the user is asked and Keys keeps what they enter; Keys supplies the held passphrase when Servers connects. Smoozy **remembers it, and keeps remembering it across restarts**, so they aren't asked again — it stays remembered until the key is deleted, or removed from Smoozy. The user can also **clear a remembered passphrase** at any time, so the next connection asks for it fresh. Remembering always, and until the passphrase is cleared — with no "don't remember this" option at the moment it's entered — is a deliberate choice to keep the flow simple, not an oversight.

**A remembered passphrase belongs to the key, not to where its file sits.** Keys tells one key from another by fingerprint (see [Telling keys apart](#telling-keys-apart)), and the passphrase is held against that. Everything else follows from it:

- **A key's file going missing does not cost the passphrase.** The key shows as unusable, and Smoozy keeps what it remembers, so a drive unplugged for an afternoon costs the user nothing. When the file comes back — in the same place, or somewhere else the user points Smoozy at — it is the same key, and the passphrase is still there.
- **A different key means a different passphrase.** Where the fingerprint changes, the remembered passphrase goes: it belonged to the key that left. That is already the rule when the user points an unusable key at a **replacement** (see [Using a key that already exists](#using-a-key-that-already-exists)), and it holds the same way for a key **with no passphrase** quietly swapped for another at the same place. **A key that has a passphrase is the one case Smoozy cannot see the swap in**: reading such a key's fingerprint needs its public file, and a public file says nothing about which private key is sitting next to it, so the row goes on showing the key Smoozy knew. What happens instead is that the remembered passphrase stays until the next connection, that connection fails to open the key that arrived, and Smoozy then forgets the passphrase and asks for it fresh — the rule below. So the passphrase is still not kept against a key it does not belong to; what is lost is the row saying a swap happened, at the cost of one failed connection. *(Narrowed on 2026-09-03, out of the `Keys` code round 1, on the consolidator's ruling after the question was put to the Creator and handed back. **Nothing is built and nothing new is promised**: the sentence described a check that could never run for a key with a passphrase, which is the only kind of key a remembered passphrase ever exists for. No auditor has seen this change.)*
- **A passphrase that stops working is forgotten, not retried.** The user can change a key's own passphrase outside Smoozy, which Keys never does itself (see [What Keys does not do](#what-keys-does-not-do)). When a remembered passphrase no longer unlocks its key, Smoozy drops it and asks for it fresh rather than failing again with the same stale one.

Label: **Creator-stated** (2026-07-27).

## Deleting a key

Delete does different things depending on the kind of key, because Smoozy owns some key files and only borrows others. What decides ownership is location, not who made the key: Smoozy treats whatever sits in the operating system's usual key location as the keys it manages, regardless of who created them, and treats anything it only references as borrowed.

- For a key sitting in the operating system's usual place (including any Smoozy generated there), deleting removes the actual key files — the private half, the public half, and a certificate file sitting beside it when one is present, whoever made the key (an extra file that a trusted authority signs to vouch for a key) *(documented — see [Sources](Sources/Keys.sources.md))* — after Smoozy asks the user to confirm.
- For a key Smoozy only **references** (one the user pointed it at, kept elsewhere), Smoozy offers **"remove from Smoozy"**: it drops the reference and forgets the key, but leaves the files exactly where they sit, untouched. This asks for no confirmation of its own, because nothing is destroyed — though the warning below still applies if a saved server relies on the key.

If the key is one that a saved server uses to connect, Smoozy **warns the user and names the servers that rely on it**, but lets them go ahead if they choose. That warning covers only the uses Smoozy knows about — its own saved servers. A key may also be used by other tools, machines, or accounts that Smoozy can't see, so removing or deleting it can have effects beyond what the warning lists. What those servers then do once the key is gone is settled in the [Servers spec](Servers.md).

## What Keys does not do

- It does not rename keys. A key's name is set once, at creation, and not changed afterwards.
- It does not add, change, or remove the passphrase on an existing key. Clearing a *remembered* passphrase is a different thing — that only forgets Smoozy's saved copy, and the key's own passphrase is never changed. Like the no-rename stance above, this is a deliberate exclusion, not a gap.
- It does not move or copy a referenced key file — it references the file where it sits.
- It does not produce certificates for the keys it generates.
- It does not give the public key to the provider — that's the user's own step.
- It does not connect to servers; that belongs to **Servers**.

## Design-level detail left for Design

The body of this specification holds further Design markers; this section is not the full set.

Settled at Design (2026-08-11 onward) — see [the Design document](../Design/Keys.md), which carries the row, its controls and their wording.
