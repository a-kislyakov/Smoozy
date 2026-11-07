# Keys

*What the keys screen looks like, what each row carries, and what the user meets when making a key, pointing at one, or taking one away. The conventions it follows are the [whole-app Design](Whole%20app.md)'s and are not restated here.*

## Where it sits

**A screen in the rail, above Settings.** It is also opened from the Add Server and Edit Server dialogs, which is where the [Keys spec](../Specs/Keys.md) says most key work actually happens — the user is adding a server, needs a key, and makes one there.

**Connections.** It serves the [Keys spec](../Specs/Keys.md). It hands a key back to [Servers](Servers.md)' two dialogs, and reads from Servers which saved servers rely on a key so it can name them before one is taken away.

## The screen is a list and almost nothing else

**The spec is blunt that nobody comes here on purpose**: a key is made for a provider once and then forgotten. So this screen is not a workspace and gets no panes, no detail column and no filter. It is a list of keys with the two ways of adding one above it.

**Two controls at the top: Choose a key… and New key…** They are the two ways a key enters Smoozy and they are not the same act — one makes files, the other only remembers where files already are. The difference decides what deleting means later, so the two are never one control with a choice inside it.

**They are the same size, and only the fill says which is which.** Making a key is the loud one and pointing at an existing one is quiet, which the fill carries; a second difference in height and type read as a mistake rather than as emphasis. Label: **Creator-stated** (2026-08-13).

## A row

**Four things, and the fingerprint is one of them.** The name, the fingerprint, where the file sits, and which servers use it.

**The name is on top and it is not an identity.** The [spec](../Specs/Keys.md) is exact: a name is the key's comment, it can repeat, and Smoozy never changes it. Two keys made on one computer carry the same name. So the name is a label the user reads, and nothing on this screen is decided by it.

**The fingerprint sits under the name, in the fixed-width face, always.** It is what actually tells one key from another — for the user and for Smoozy — so it is not detail behind a control. Two rows reading *Studio MacBook Pro* with two different fingerprints is the case this screen exists to make readable.

**Where the file sits is the third line, and it is doing more work than it looks.** The [spec](../Specs/Keys.md) settles that ownership is decided by location and not by who made the key: whatever sits in the operating system's usual place is Smoozy's to delete, and anything else is only referenced. That is the difference between *Delete* and *Remove*, so the path is what the user reads before pressing either.

**Which servers use it is the fourth**, named rather than counted. *Used by RunPod 4090 and Vast 5060* is what makes taking a key away a decision; *2 servers* makes it a number.

**The row's controls sit in fixed slots, not in a run that ends where the row's text stops.** Copy at one x, the control that takes the key away at another, in every row — including the row that has no copy, where the slot is empty rather than closed up. That is the same arrangement the [Servers](Servers.md) list already uses, and for the same reason: a control that moves between rows is a control the eye has to find again each time.

**The list's own edges match the server list's**, header and footer included, because the two are the same kind of screen. They differed by four pixels, which is enough to read as wrong without being enough to name.

## The two kinds, said by the control and not by a tag

**A key Smoozy manages offers Delete. A key it only references offers Remove.** Those two words are the whole difference and the [spec](../Specs/Keys.md) gives them: one takes the files, the other drops the reference and leaves the files where they sit. Each says so on hover: Delete, *Deletes the key files from your .ssh folder.*; Remove, *Removes the key from Smoozy. Its files are not deleted.* Creator-stated 2026-10-08, sixth notes, note 7. *Changed 2026-10-12, Creator-stated, the Creator's eighth notes (2026-10-11), note 1: the reason is where the key sits. The lines were* Deletes the key files from your computer. *and* Removes the key from Smoozy. Its files stay where they are. *Words changed 2026-11-01 (was *Each says so on hover, with the reason, which is where the key sits* and *Removes the key from Smoozy. Its files are outside your .ssh folder and stay.*): Creator-stated, the second words sweep. No auditor has seen it.*

**Neither kind carries a badge saying which it is**, because the path already says it and the control already says it. A third way of saying the same thing is what a screen with three keys on it cannot afford.

## While a key is being checked

**The row draws first and its verdict arrives behind it.** The [architecture](../Architecture/Keys.md) U10a is firm that no list waits on a reading, so a key's usable-or-unusable answer is not there at the instant the row appears — and the rule that follows is the one that matters: **no verdict is drawn that has not been read.** A row is never shown usable while it is still being asked, and never unusable either.

**So the row shows everything the record holds** — the name, the fingerprint, the path and the *used by* line — and the one place the verdict would go carries the underway mark instead. That is the whole-app *underway on the control* shape, at its smallest.

**It is not a veil and not a spinner over the list.** Every other row is true and usable while one key is slow, and the [architecture](../Architecture/Keys.md) U10a has each answer arrive on its own, so a slow key holds up nothing.

**And it happens every time the screen is opened**, because nothing is cached and nothing is remembered — which is the [spec](../Specs/Keys.md)'s own rule and the reason a key that has quietly gone unusable is caught at all.

## An unusable key

**It stays in the list, dimmed, and says what is wrong in the row.** The [spec](../Specs/Keys.md) requires it to be shown rather than hidden — a key that vanished when its drive was unplugged would look like a key that was never there.

**The row carries the two ways out the spec gives, and they are not the same offer.** *Choose its new location* takes the same key, and Smoozy checks the fingerprint matches. *Choose a replacement* takes a different key, and the row says what that costs before it is pressed: the saved passphrase is cleared, and every server holding the old key needs a new one.

**The fingerprint stays on screen while the key is unusable.** It is the only thing that can say whether the file the user is about to point at is the same key.

## Which of this screen's bad answers is a failure

**Five of them, and they do not all take the same shape.** The [whole-app Design](Whole%20app.md) gives a failure one shape wherever it attaches, and this screen names which of its five take it.

- **An unusable key.** The message on the row takes the shape: the red dot, then one plain sentence saying what is wrong, then the two ways out — which are the shape's own controls, the acts that would try again.
- **A key file that could not be read**, and **a key whose permissions let others read it**. Both are refusals of the act the user just took, so both take the shape where that act happened. The permissions one is the case where Smoozy knows what to do and says it, naming the mend as the user's own.
- **A good key with the wrong fingerprint.** The same shape, and its second sentence is the other way out: *Choose a replacement* takes a different key deliberately, with what that costs said before it is pressed.
- **A remembered passphrase that stopped working is not a failure.** Nothing is broken and there is nothing to clear or try again — Smoozy has already forgotten the passphrase, and the box in front of the user is asking for it. It is one line of that box, which is where this document already puts it.

**None of the four carries a cross.** The shape puts one on a failure describing something still true, and each of these is — but clearing it is the one thing the row must not offer. An unusable key cleared would look like a key that was never there, which is the exact confusion the [spec](../Specs/Keys.md) requires the row to prevent. The way these end is the way the condition ends: point at the file's new place, point at a replacement, mend the permissions, and the message goes because the key is usable again. **So the cross's rule is unchanged and the cross is absent** — this is not a failure the user is allowed to dismiss, it is one they are allowed to fix, and the two ways out are what stand where a cross would.

**The row stays dimmed, and here dimming says one thing: this key cannot be chosen.** It is not the whole-app dim, which says a reading came from memory rather than from a server, and it is not the failure — the failure is the message on the row, in the shape above. Saying *cannot be chosen* is exactly what this screen needs, because in the mode where it was opened to pick a key an unusable key's choose control is absent rather than dimmed, and the dim is what says why.

## When there is no public key file beside the key

**The copy control is absent, not dimmed, and one line says why.** The [architecture](../Architecture/Keys.md) L4 looks for the public half beside the private file by name; where it is not there, there is nothing to copy and a control that copied nothing would be a control that does nothing.

**The line says what Smoozy would need and where.** *No public key file beside this key. The key itself works.* **It is the line for a key with no passphrase**, whose second sentence is true of that key and of no other — the protected case takes its own line, below. It does not offer to make one: the [spec](../Specs/Keys.md) keeps Smoozy from changing a key it only references, and writing a file next to somebody's private key is exactly that.

**For a key with no passphrase the fingerprint still reads and the row is still usable.** The [architecture](../Architecture/Keys.md) F1 reads such a key's fingerprint from the private file alone, so a missing public half costs the user the copy and nothing else — the row says that rather than reading as broken.

**For a key with a passphrase it is the other way round, and this screen has to say so.** The [architecture](../Architecture/Keys.md) U7b establishes that the private file's own read is refused for every protected key, and [U7](../Architecture/Keys.md)'s fallback reads the public file instead — so with no public file beside it **there is nothing left to read the fingerprint from, and the row is unusable**. The architecture keeps that verdict as *unreadable* and does not widen its closed set; what it adds is which situation produced it, so this screen can say the true thing rather than *the file could not be read*, which would send the user hunting for damage in a healthy file. **The line says the key has a passphrase, that there is no public key file beside it, and that bringing that file across is what fixes it.** **It is two sentences and they are these:** *Passphrase set and no .pub file beside it.* *Can't be checked without one.* *Words changed 2026-11-06 (was *This key has a passphrase and no public key file beside it.* *It cannot be checked until that file is beside it.*): the last words read before the release. It still names no act, for the reason below. No auditor has seen it.*

*(Ruled 2026-09-13 at Build step 10a, under the Creator's standing delegation of 2026-08-31, the Design stage being closed. It is the last of the three things this paragraph asks for and the other two were already here; what it adds is the wording, in the shape the paragraph above it already uses for the key with no passphrase. **It says what is missing and what ends the limit, and it names no act the user is told to take** — the file may be on another computer and Smoozy does not know. **It is not a fault of the key and does not read as one**, which is the whole reason the architecture carries the situation beside the verdict rather than letting the row say *the file could not be read*. The reasoning in full is in `Audits/Build/Step 10a/Piece 4 — what landed.md` and in `Audits/Build/Desktop app and delivery/Decisions record.md`. No auditor has seen this change.)*

*(Narrowed on 2026-09-03, out of the `Keys` code round 1. This paragraph read* The fingerprint still reads and the row is still usable *for every key, and cited U7 and F1 as reading from the private file alone — which was true while that read was `-l -f` and stopped being true when U7b named `-y -f`. **Design conforms to Architecture under [Rule 10](../Groundwork/Rules.md)**, so the paragraph narrows rather than the answer being reopened. No auditor has seen this change.)*

**The line stands on any key with no passphrase missing its public half, one Smoozy made included.** Smoozy wrote both halves once, and never touches the files afterwards — the [architecture](../Architecture/Keys.md) G10 — so nothing stops the public half being gone by the time anyone looks. *No public key file beside this key* is true of a made key and a referenced one alike, which is what makes it the honest wording rather than *Smoozy lost it*.

## When the user points at something that is not a key

**Two different refusals, and they must not read alike.** One is *this is not a key Smoozy can read*; the other is *this is a perfectly good key, and it is not the one this row is about*. The [spec](../Specs/Keys.md) allows the new-location path to refuse on the second, and a single message would leave the user hunting for a fault in a file that has none.

**Not a usable key** says what failed, from the [architecture](../Architecture/Keys.md) U6's own closed set: the file could not be read, or it is not a private key Smoozy recognises, or it is a key of a type Smoozy does not support, or its permissions let others read it — that last one naming the mend as the user's own, since Smoozy never widens or narrows a key file's permissions. The unsupported type is its own message and is never folded into *not a key*: a key of a type Smoozy does not support **is** a key, and saying otherwise sends the user hunting for a fault the file does not have.

**A good key with the wrong fingerprint** says exactly that: this is a key, and it is not the key this row was made for. It names both fingerprints, because on this one screen the fingerprint is the only thing that can tell two keys apart — and it points at the other way out: *Choose a replacement* takes a different key deliberately, with what that costs said before it is pressed.

**Neither is a dead end.** Both leave the row as it was, unusable and still in the list, with both ways out still on it.

## Where the passphrase is asked for

**On a connect the user started, and nowhere else.** The [spec](../Specs/Keys.md) puts the ask there, and the reason is that a passphrase asked for by something the user did not start is a question with no context — they would not know which key, or why now.

**So the box is the connection's, not this screen's.** It names the server being connected to and the key by name, takes the passphrase, and says nothing about how it will be kept. The [spec](../Specs/Keys.md) settles that Smoozy always remembers, so there is no choice to offer and no line explaining one.

**Asking again after a remembered one stops working is the same box with one line added.** The [architecture](../Architecture/Keys.md) S5 establishes that from a local load rather than from a failed connection, and erases the remembered one the moment it does — so the line says the saved passphrase no longer works and was cleared. Without that line the user retypes what they think is already stored and cannot tell whether anything changed.

**It never says why it stopped working**, because Smoozy does not know: the [architecture](../Architecture/Keys.md) S5 records that a correct passphrase against a damaged key gives the same answer as a wrong one. Guessing between them would be Smoozy telling the user their key is fine when it may not be.

**And an automatic attempt does not ask at all.** It finds nothing remembered and makes no attempt, which is the [spec](../Specs/Keys.md)'s own rule — the ask waits for the user to start a connect.

## A remembered passphrase

**One line on the row, only where there is one, with the control that clears it.** The [spec](../Specs/Keys.md) settles that Smoozy always remembers and keeps remembering, so there is nothing to choose here — the line reports and offers the one action.

**It says the passphrase is saved, never how.** How it is kept safely is Architecture's, and a screen that hinted at the answer would be making a promise this design cannot keep.

## On a computer with nowhere to keep a passphrase

**Smoozy says so once, where the user already is, and never again.** The [whole-app spec](../Specs/Whole%20app.md) promises that where a computer has nowhere to keep a secret, Smoozy asks each time it needs one *"and it says so once, so the user knows it is not going to learn rather than thinking it keeps forgetting."* The [architecture](../Architecture/Security%20and%20licence.md) S34 fixes the moment: the first time Smoozy finds out, once for the life of the installation — not once a run, which would be the forgetting the sentence exists to explain.

**The sentence:** *Passwords and passphrases cannot be saved on this computer.* It names the computer as the thing that cannot, because that is what will not change by waiting, and it says both words because the same fact covers a key's passphrase and a server's admin password.

**It goes with the act that found out, in that act's own place.** Finding out takes an act that needed a secret, and the sentence is a note in the box or dialog that act already shows — never a dialog of its own:

- **The passphrase box**, under the line naming the server being connected to.
- **The new-key dialog's last step**, under the line saying the public key is copied, where the key was made with a passphrase.
- **The admin access box** on Set up, above the password field. The choice to remember the password is already absent there ([Whole app](Whole%20app.md)); the sentence says why, the once it is said.

**Where no act was in front of the user** — Smoozy found out while repairing a connection on its own — **the sentence goes on that server's row in the All servers list**, beside the mark below, and stays there for as long as Smoozy stays open.

**The mark on a server Smoozy cannot reconnect on its own for want of a passphrase.** The [architecture](../Architecture/Keys.md) S10 has the server marked with what is wrong, bounded so that a permanent condition is never worded as a passing one, and the words are Design's:

- where the computer has nowhere to keep one, *No automatic reconnect: this computer cannot save passphrases.*
- otherwise, *No automatic reconnect until you connect: passphrase not saved.*

**It takes the row's extra line the way the gone-key mark does** ([Servers](Servers.md)): a failure standing on the row takes the line first, and the mark takes it back afterwards. A gone key is the bigger fact and takes the line from this mark. **The once-said sentence is not the mark and is drawn whether or not the mark has the line**, because it is said once and a line it lost to a failure would never be said at all.

## Where the passphrase cannot be handed to the SSH tools

**Every passphrase reaches the SSH tools down one channel Smoozy opens in its own data folder, and on a computer where the path to that folder is too long the channel cannot be opened.** The [architecture](../Architecture/Keys.md) G3a says so. The folder's path includes the account name, so this is a condition of the computer, it does not pass by trying again, and it stops every key with a passphrase — making one and using one alike.

**The sentence:** *Keys with a passphrase cannot be created or used: the path to Smoozy's data folder is too long.* It says what cannot be done, and why, and stops, in the shape of the Windows limit's sentence.

**It is said in place of the answer the act would otherwise give**: above the list in All servers where a connect met it, instead of the connect failure's own sentence — *Could not connect to Hyperstack.*, the server named, since 2026-10-03 ([Servers](Servers.md), *Adding a server*) — which would claim nothing true about why; on the server's Set up card where an *Add and set up* met it ([Server Setup](Server%20Setup.md), *This server*); and in the new-key dialog under *Could not create the key.* **It is not drawn in the mockup.**

**On Windows the channel is not built at all**, so the same limit holds there for every key with a passphrase, whatever the path — the [architecture](../Architecture/Keys.md) G3a again. **The Windows limit's sentence:** *Keys with a passphrase are not supported on Windows.* It names the limit and the system and stops; it names no act, because nothing inside Smoozy ends it. It is said in the same places as the sentence above, in its place. **It is not drawn in the mockup.** *(Added 2026-10-24, at Build step 12, piece P1. The words were ruled on 2026-09-13 at Build step 10a piece 4, under the Creator's standing delegation, and until this date stood only in the code, while the paragraph above already pointed at them.)*

## On a computer without the SSH tools

**Making a key and checking one both rest on the computer's own SSH tools, and a few computers do not have them.** The [spec](../Specs/Keys.md) has Smoozy make nothing and check nothing there, say so plainly and once, and never call the user's existing keys broken — they are not. The [architecture](../Architecture/Keys.md) U6a and G6 put one statement in each place the readings were needed: this screen and the server list.

**One line above the list, never a message per key.** *SSH tools not installed. Keys can't be created or checked without them.* *(Words changed 2026-11-06 (was *SSH tools not installed. Install them to create or check keys.*): the check after the last fix — the second sentence asked the user to install, which the spec forbids. No auditor has seen it.)* *(Words changed 2026-11-06 (was *SSH tools are not installed on this computer. No key can be created or checked until they are.*): the last words read before the release. No auditor has seen it.)* *(Changed 2026-10-28, Rule 9, the loose ends after the words sweep: the third sentence, *Your keys are not at fault.*, is dropped. Nothing in the line calls the keys broken, so it added nothing. No auditor has seen it.)* The cause is a property of the computer rather than of any key, so it is said once where the whole list is read — the whole-app rule for one cause over many rows. The same line, word for word, is the one the [Servers](Servers.md) list carries.

**"Without them" says when the limit ends; it does not tell the user to end it.** The [architecture](../Architecture/Keys.md) G6 wrote the phrase *until they are* and then flagged it, because the [spec](../Specs/Keys.md) forbids asking the user to install anything and the phrase could be read as doing exactly that; *without them* says the same and is held to the same judgement. It stays. *(Words changed 2026-11-06 (was *"Until they are"*, which the line no longer says): the check after the last fix. The judgement is unchanged. No auditor has seen it.)* It does not address the user and it names no act they could take, and nothing else on the screen offers one — *New key…* is unavailable with this line as its reason, and there is no install control anywhere near it. Taking the phrase out would make the line worse rather than safer: *Keys can't be created or checked*, stopped there, reads as a permanent limit of the app instead of a condition of this computer, which is not true. The two things the spec bans are asking the user to act and calling their keys broken; this line does neither. The judgement covers the server list's copy of the line as well. Label: **settled at Design** (2026-09-02).

**Every row shows what Smoozy remembers, and no verdict**, because no verdict was read. A key is never called unusable for the computer's missing tool, and no row carries a message of its own either — not even one saying the file is not there, which needs no tool to see. The [architecture](../Architecture/Keys.md) U6a and G6 allow neither: *not checked* is the absence of a verdict rather than a new kind of one, and no unusable reason may be composed for a key on such a computer. So the line above the list is the whole of what this screen says about the missing tools.

**New key… reads unavailable**, with the line above as its reason. Nobody is asked to install anything; the user may not be allowed to.

## Making a key

**A dialog, and the provider suggestion is the first thing in it.** The [spec](../Specs/Keys.md) asks Design to make that suggestion prominent, and the reason is worth the room: a key is made for a provider once and used for every machine rented there, so the name is the only thing that later says which key belongs where. A user who names it after their computer will pick blind in the Add Server dialog months later.

**So the name field leads, and the line under it is the suggestion rather than a rule.** *Use your provider's name.* The field is pre-filled with the default the spec describes, and the suggestion sits beneath as a sentence the user can ignore — which they can, since anything they type is accepted and the default stands if they leave it alone.

**A passphrase is one optional field, not a section.** It is a real choice and a short one.

**The key type is behind *Advanced* and closed.** The [spec](../Specs/Keys.md) settles that Smoozy picks a sensible modern default and the user can accept it, so putting the choice in front of everybody makes a decision out of something almost nobody has.

**Opened, it is the named types this computer can make and nothing else — no explanation of which is stronger.** The [architecture](../Architecture/Keys.md) T2 reads the offered set off the installed tool at that moment rather than off any fixed list, so the count is usually three and can be fewer — a single entry on some computers. It settles the default on a judgement it states and labels as one, rather than on any ranking. So this screen does not rank them either: an artist choosing here already knows why they are choosing, and a line telling them one is best would be Smoozy's opinion dressed as a fact.

**The default is marked as the default and nothing more.** *Default* beside it. Not *recommended*, which implies a comparison nobody made.

**A type the computer cannot make is not in the list.** The [architecture](../Architecture/Keys.md) checks the offered set against the machine at the moment the dialog needs it, so an unusable option is absent rather than shown and refused — the same rule the whole app follows.

**Where the usual one cannot be made, the next in the fixed order is the default** and one line says so, because a user who opened Advanced expecting the usual choice and found another selected is owed the reason.

**Then the public half is copied, and that is the last step of making a key rather than a separate errand.** The dialog ends on it, because the reason the user made a key at all is to paste it at a provider — and the [Servers](Servers.md) screen's empty-list guidance says the same thing in the same order.

## The list's order

**By name, and the name is not an identity — which is exactly why the order needs saying.** Two keys can share a name and the [spec](../Specs/Keys.md) allows it, so a list ordered by name puts them next to each other, which is the most useful place they can be: the only thing that separates them is the fingerprint, and side by side is where a fingerprint is worth reading.

**Nothing else sorts.** No sorting by type, by date or by state — a key list is short, and headers that sort are a file manager's answer to a problem this screen does not have.

**Unusable keys are not moved to the bottom.** They stay where the name puts them, dimmed. Moving them would make the list reorder itself as keys break and mend, and the [architecture](../Architecture/Keys.md) U10a has verdicts arriving one at a time after the draw — a list that re-sorted as they landed would move under the user's hand.

**The order does not change while a check is running**, for the same reason.

## Where two keys share a name

**Smoozy says so when the second one arrives, not on the list afterwards.** The [spec](../Specs/Keys.md) is exact about when: a new or newly referenced key sharing a name with a different fingerprint is pointed out; two keys sharing the *default* name are left alone, since that is two ordinary keys made on one computer.

**It is a statement, not a refusal.** Nothing is blocked. The line names the other key's fingerprint, which is the only thing that distinguishes them.

**Pointing at a key Smoozy already knows is different and is refused.** The [spec](../Specs/Keys.md) keeps one entry, so the message says the key is already here and names it, rather than adding a second row for one key.

## Taking a key away

**The servers are named before the confirmation, and the sentence after them is the honest one.** The warning covers the servers Smoozy knows about, and a key may be used by tools and computers Smoozy cannot see — so the box names the servers, then says *Other apps and computers may also use this key.* *Words changed 2026-11-06 (was *Only Smoozy's servers are listed.* before that sentence): the last words read before the release; the list itself says which servers are Smoozy's. No auditor has seen it.*

**Delete asks; Remove does not.** That follows the [spec](../Specs/Keys.md): deleting destroys files, removing drops a reference and leaves everything where it sits. Where a server relies on the key, Remove still shows the warning — nothing is destroyed, but something breaks.

## Opened from a server dialog

**The same list, with a Use this key control on each row and Cancel along the bottom.** The [spec](../Specs/Keys.md) settles that the key is handed back selected, and that cancelling returns the user to the dialog with what they had already filled in.

**Reached from a server dialog's New key…, the screen opens with the New key dialog already showing.** The link names that act, so it does it. The dialog's key field is Smoozy's own dropdown of the usable keys — each by name, with its fingerprint's start beside it in the fixed-width face — and a pick fills the field without leaving the dialog; its last row, *Open Keys…*, opens this screen in its picking mode. A key still being checked is listed and cannot be picked until it reads usable, and one that reads unusable leaves the list ([architecture](../Architecture/Keys.md) U10a). In the Edit dialog the server's own unusable key stays at the top of the list with the gone mark ([Servers](Servers.md), *The Edit dialog*). *Changed 2026-10-12, Creator-stated, the Creator's eighth notes (2026-10-11), note 3; the sentence was* The dialog's key field opens the list alone, as before. *The drawing already drew a dropdown in both server dialogs; this sentence now matches it.* *Added 2026-10-02, the Creator's note 1 on 0.1.7: "Pressing New Key... in the Add server dialog doesn't actually open the new key dialog." Under Rule 10: the [architecture](../Architecture/Keys.md) U11 hands the dialog's fields over the same way whichever control was pressed.*

**Unusable keys are not offered here.** The spec forbids it for the Add Server dialog, so in this mode they read as they do everywhere else and their choose control is absent rather than dimmed.

**The screen says which server it is choosing for, where anything names one.** A list of keys with no context is the same screen as the housekeeping one, and the user arrived at it in the middle of something. The next paragraph is the case where nothing does.

**Where nothing names the server, the line reads *Choosing a key* and stops.** Coming from Edit Server the saved row carries a label. Coming from Add Server the dialog's name, username and host may all still be empty, and the [architecture](../Architecture/Keys.md) U11 hands over the dialog's own fields as they stand and nothing where there is none — so no value naming that server exists anywhere. The line's job is the context above, and the act does that job on its own; a stand-in phrase would put words on screen standing for something that has no name yet, which is worse than stopping short. Label: **settled at Design** (2026-08-24).

## Keyboard shortcuts

**Return chooses the key where this screen was opened to pick one, and does nothing otherwise.** The delete key deletes or removes, whichever the chosen row offers, asking exactly as the control does. The whole-app rules are in [Whole app](Whole%20app.md).

## What this document left open, and how it was answered

Every marker this document once carried is closed, and the answers are kept together here rather than folded back into the body. **Nothing below is still open** — the heading used to say *Left for later*, which was true when the markers were markers and stopped being true the day they were answered.


- **Settled at Design (2026-08-23).** There is nothing to show: a second key of the same name takes the next free number rather than asking, so no collision ever reaches the user. The name appears where this document already draws the path — on the row, in the fixed-width face — and the made-key dialog ends by naming the file it wrote, which is the one moment the user has any reason to know it.
