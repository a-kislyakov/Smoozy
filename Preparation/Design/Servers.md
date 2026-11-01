# Servers

*What the server list looks like, what each row carries, and what the user meets when adding, removing or reconnecting a server. The conventions it follows — the badge, how connection is shown, the words — are the [whole-app Design](Whole%20app.md)'s and are not restated here.*

## Where it sits

**All servers is a screen, reached from the top of the rail.** The rail lists the servers by name for navigation; this screen is where they are compared. That split is settled in the [whole-app Design](Whole%20app.md).

**Connections.** It serves the [Servers spec](../Specs/Servers.md). Opening a server from here leads to [File Browser](File%20Browser.md) and [Server Setup](Server%20Setup.md), which are that server's two tabs. It shows work read from [Queue & History](Queue%20&%20History.md), and hardware the Servers spec owns.

## A row

Every row has the same shape, so nothing shifts down the column: **badge, name, address, and one extra line only where there is something to say.**

- **The badge** is the server's colour.
- **The name**, or `username@host` where the server has none. The port shows where it is the only thing telling two unnamed rows apart, which the [Servers spec](../Specs/Servers.md) requires.
- **The address**, in the fixed-width face.
- **The extra line**, which is a failure that is still standing, the mark saying the server needs a new key, or the mark saying Smoozy cannot reconnect the server on its own for want of its key's passphrase. A failure takes the line first and the gone-key mark takes it from the passphrase mark; the passphrase mark's words, and the once-said sentence that can stand on the row beside whichever has the line, are [Keys](Keys.md)'s, under *On a computer with nowhere to keep a passphrase*.

Beside that, in three columns: **what it is doing**, **what it has to render with**, and the actions.

**What it is doing** is words: *Rendering* and the job's name with how many are waiting, or *Idle*, or *Reconnecting…* with when the next attempt falls. Where Smoozy cannot say what the server is doing, the column takes the dash — the mark [Design values](Design%20values.md) gives app-wide for a value Smoozy cannot confirm. Never a colour.

**Where the reading holds jobs waiting and none rendering, the column says *Waiting* and how many — *Waiting*, *2 jobs* — in the same shape as *Rendering*, with the count where the job's name goes.** That is the server whose every job waits on the user, such as a first-frame check, and the reading says exactly that, so the dash would claim Smoozy cannot tell when it can. *Added 2026-09-28, out of step 10c's ninth-pass fix piece 2, which drew the dash here for want of a word. Design conforming to Architecture under Rule 10: the words are drawn from the [architecture](../Architecture/Queue%20&%20History.md)'s queue reading (A1) and nothing else, held jobs counted as the Queue counts them, and never from Smoozy's own record; a reading that is not updating still takes the dash (C7). It says nothing about W1's idle message, which may still count such a machine idle.*

**A server Smoozy restarted reads *Restarting*, and it is not a failure.** The [architecture](../Architecture/Servers.md) K8 makes it a presentation rather than a fourth state — the restart mark standing while the connection is down — and M3 suppresses the ordinary dropped-connection repair and its notification underneath it. So the row says *Restarting* and how long it has been waiting, in the ordinary face and not the failure one, because Smoozy took the machine down on purpose and knows it.

**It says nothing about how long is left.** The [architecture](../Architecture/Servers.md) M5 has machines return anywhere from nine to ninety seconds and says outright that any fixed wait is wrong on most of them, so a countdown would be a promise made out of nothing. The elapsed time is a fact; the remaining time is not.

**When Smoozy gives up, the row becomes an ordinary reconnecting server and says so once.** The [architecture](../Architecture/Servers.md) M6 and M7 clear the mark and hand the period back to the ordinary repair, which never stops while the app is open.

**What the line says is the [Whole app spec](../Specs/Whole%20app.md)'s and is not copied here**, including the half that keeps Smoozy from telling the user to try again or to check their own network. It claims nothing about why, which matters most in this case: a machine still starting cannot be told from one that will never answer.

**The line carries no retry, and that is the failure shape being met rather than broken.** The shape puts whatever act would try again inside the failure; this row already carries the connect control, so the line does not repeat it and the connect control is the way back. Nothing on the row is put forward as the fix, which the spec requires.

**And the line goes when the connection is made again, however that happened** — the user connecting, or Smoozy's own repair getting through. It does not wait to be cleared. That is the [whole-app Design](Whole%20app.md)'s rule for a failure whose condition has ended, and clearing it by hand is only one of the ways out.

**And a setup that stopped part-way is a line on the row too.** [Server Setup](Server%20Setup.md) reads that state fresh from the machine on every connection, so the row is reading rather than remembering: *Setup stopped part-way*, with the step list inside the server. A server with no connection says nothing about a setup at all.

**Where a server is connected and Smoozy's service on it does not answer, the row's extra line says *Smoozy isn't responding.* (shortened 2026-09-27 on the Creator's word: *"The wording is too long, I think it should be shorter"*; it was *Smoozy's service on this server is not responding.*), and what it is doing takes the dash rather than *Idle*, since no reading of its work stands.** *Added 2026-09-27, out of step 10c's seventh pass on RunPod, where a server after a restart read Idle with a general failure line; the words were owed from the eighth pass's fix map. The condition is the [architecture](../Architecture/Desktop%20app%20and%20delivery.md#12-the-automatic-supervisor-unavailable-repair-trigger)'s supervisor-unavailable condition (V1), whose words were left to Design. The line says the service is not answering and nothing more: it does not say the server will or will not start it again, because Smoozy cannot tell which kind of provider it is on (`Rented-server runtime` §16 item 16). Design conforming to Architecture under Rule 10.*

**Where Smoozy on a server is older or newer than the app, the row's extra line says which and what happens next, and never *Smoozy isn't responding*.** Smoozy on a server follows the app's own version and is updated when Smoozy connects and nothing is running there; nothing is said about that ordinary case beyond the line while it runs. The lines:

- **Waiting for its jobs.** *Smoozy on this server updates when its jobs finish.* with **Update now…** inside the line, since the act that would move it belongs inside it. The work column reads as ever. **Update now…** opens a box: *Update Smoozy on RunPod 4090 now?*, *Updating now stops its jobs first.*, then the removal box's own statement of what *stop and clear* does — *Clears the 2 waiting jobs, then stops the render. The current frame may not be saved; rendered frames stay on the server.* — then **Cancel** and **Stop jobs and update**. Where the stop and clear does not fully work, the box says so in the words the removal uses for the same result, and the line stays.
- **Updating.** *Updating Smoozy on this server…* in the ordinary face, and the work column takes the dash while it runs, since the service is being replaced and no reading of its work stands.
- **The update failed.** *Smoozy on this server could not be updated.* in the failure face, with the server's own words behind it as every failure carries them. It goes when a later update lands. The server keeps working on the version it had.
- **Too old, with jobs on it.** *Smoozy on this server is out of date. Reconnect after its jobs finish to update it.* No **Update now…**, because the stop cannot reach those jobs. The work column takes the dash.
- **Too old, and Smoozy cannot tell whether it has jobs.** *Out of date. Not updated: could not read its job list.* No **Update now…**, for the same reason as the line above. The work column takes the dash. *Words changed 2026-11-01 (was *Smoozy on this server is out of date and was not updated: its job list could not be read. Checked again on each connection.*): Creator-stated, the second words sweep. No auditor has seen it.*
- **Too new.** *This server has a newer version of Smoozy. Update Smoozy on this computer to use it.* The work column takes the dash.

*Added 2026-09-28, out of step 10c's plan pass for servers following the app's release. Design conforming to Architecture under Rule 10: the five conditions are `Desktop app and delivery` V13's, the override is V15's and its statement is `Servers` W19a's, already drawn for a removal; `Server protocol and transfers` P3b is why none of them is the not-responding line.*

*The sixth line, too old with no way to tell whether it has jobs, was added 2026-09-28, out of step 10c's transfer wave piece 9, on the build runner's ruling of that day. Design conforming to Architecture under Rule 10: `Desktop app and delivery` V11 holds a program it cannot talk to as one with work on it where `Server protocol and transfers` P3b's reading of its records cannot answer, and the line above would then say jobs stand when that is not known (Rule 8). It carries no **Update now…**, because V15 is not offered on too old: the stop cannot reach that program.*

**While a setup runs on a server, the column says *Setting up*.** It takes the place of *Idle* and the dash, never of *Rendering*, *Waiting* or *Restarting*. A row reading *Idle* while Smoozy installs a driver and a Blender there is the row saying nothing is happening when something is. *Added 2026-10-02, the Creator's note 3 on 0.1.7. Under Rule 10: the run is the [architecture](../Architecture/Server%20Setup.md)'s Y6 reading, which every connection already takes.*

**Idle carries no second line.** It said *nothing rendering, nothing waiting* beneath it, which is what idle means — the line explained the word above it and changed nothing the user would do. Label: **Creator-stated** (2026-08-13).

**What it has to render with** is the GPUs and their VRAM, then the server's RAM and its free disk — those words, since a screen uses the industry's. Where Smoozy is showing this from memory it is greyed and says when it was last seen. A server never connected shows nothing and says so rather than looking empty. No row shows Blender versions — the [Servers spec](../Specs/Servers.md) forbids it, because the user can install one without Smoozy knowing.

**The hardware column is the widest of the three, and the work column the narrowest.** Creator-stated (2026-09-28): *"The GPU information in the All servers window is very narrow. The status in the same window didn't need that much space — make room for the info to not overflow as much."* The work column is a word and a name, and keeps only the room its longest word needs — *Reconnecting…* — so it narrows no further near the fold. The hardware takes what the work column gave up, and a card line as the driver names it, *1 × NVIDIA GeForce RTX 4090, 24 GB*, sits on one line in the window Smoozy opens at. **Where the hardware folds under the work, that shared column is the wider of the two beside the name**, since the work is one short line and the hardware is the rest. **It holds from the narrowest window Smoozy allows to a large one**: nothing runs out of its column, and at the narrow end a long card line or a long address takes a second line. The widths are the drawing's. *Added 2026-09-28, watching step 10c's tenth pass. Design conforming to Architecture under Rule 10: no architecture answer touches layout.*

**The list carries a footer**, with how many servers there are and how many are connected.

**On a computer without the SSH tools, one line above the list says why no key is checked.** It is the [Keys](Keys.md) screen's statement, word for word — *SSH tools are not installed on this computer. No key can be created or checked until they are.* — because the rows' key marks rest on the same readings, which is the [architecture](../Architecture/Keys.md) U6a's rule. Said once, never per row, and no key mark pretends to a verdict nobody read.

## Actions on a row

**Clicking the row opens the server.** That is the ordinary thing to do with one, and it is why no row carries a *Set up* control: a control saying *Set up* in a list reads as an instruction that the server needs setting up, which is usually untrue.

**The connect control sits on the row and in the server pane’s heading**, and carries the connection state, as the [whole-app Design](Whole%20app.md) sets out. *Changed 2026-10-23; this began “The connect control sits on the row”. Creator-stated, seventeenth note 15: “An existing server that is not connected should have a connect button in the right pane when I click it.”* Where a server carries a setup it never got to run, the control says so — *Connect and set up* — which the [Servers spec](../Specs/Servers.md) asks for.

**Everything else is behind the three dots, and behind a right-click, which open the same menu.** There are five menus, one per shape a server can be in, and each shows the actions it cannot do rather than hiding them.

**A server Smoozy is repairing takes the connected menu, and its first slot reads *disconnect*.** The [spec](../Specs/Servers.md) keeps disconnecting available for as long as a repair is running — at any point in the repair, during an attempt or in either of the pauses *(corrected 2026-08-22 — this said whether the row reads Reconnecting… or sits disconnected between attempts, which was written on a spec sentence the Creator retired that day: the row now reads Reconnecting… for the whole repair)* — and taking it stops the attempts. So a repairing server is not a sixth shape; it is the connected menu, because the one thing the user needs there is the way out, and the disconnected menu offers connect, which is the opposite. Label: **Creator-stated** (2026-08-13).

**Every menu is the same five slots in the same order**, so a user who has disconnected a server twice does not have to read the menu a third time. Label: **Creator-stated** (2026-08-13).

1. **The connection** — connect, disconnect, or connect and set up. Always the first item, always exactly one item.
2. **Going into the server** — browse files, set up.
3. **What this shape can do about itself**, where it has anything — the two ways out of a gone key, or connecting without the saved setup and forgetting it.
4. **The server's own details** — rename, edit, copy address.
5. **Remove.**

**Copy address stays, and the spec's open question does not reach it.** The marker parked it against the [spec](../Specs/Servers.md)'s own question about a server action beyond connect, edit and remove. That question is about acts that *touch the machine* — a reboot the user asks for by hand is its example — and copying an address touches nothing, reaches nothing and grants no power the user did not already have; the address is on the row in front of them. So it is a convenience over text on screen, not a sixth thing Smoozy does to a server, and it is not waiting on anything.

**A slot with nothing in it is skipped, not padded**, and the order still holds. What matters is that nothing above a slot ever moves, so the position of every item is the same in all five menus.

**This cost the key actions their place at the top**, and that is the trade. They were first in the two menus that have them, which put connect third in those two and first everywhere else — the exact wandering this rule exists to stop. They now sit in slot 3, which is the same slot in every menu that has one.

The five:

- **Connected** — disconnect; browse and set up; rename, edit, copy address; remove.
- **Disconnected** — connect; browse and set up shown unavailable; rename, edit, copy address; remove.
- **Needs a new key** — connect shown unavailable; browse and set up shown unavailable; the two ways out the [Servers spec](../Specs/Servers.md) gives, a different key and fixing the key in Keys; rename, edit, copy address; remove.
- **Connected, key gone** — disconnect; browse and set up, both available; the same two ways out; rename, edit, copy address; remove.
- **Waiting to be set up** — connect and set up; browse and set up shown unavailable; connecting without the setup and forgetting it; rename, edit, copy address; remove.

**Rename… changes the name in place, on the row**, as a rename does in [File Browser](File%20Browser.md). Return saves and Escape cancels. An emptied name clears it, and the row shows `username@host`, as clearing it in Edit does. A name another server has gets Add's duplicate-name line under the field, which never blocks. It saves the name alone, the way Edit saves it, and carries none of Edit's warnings: the [architecture](../Architecture/Servers.md) W37 gives a rename none. *Added 2026-10-02. **Creator-stated**, note 1 on 0.1.8: "I feel like at least a rename operator should be present in the right click menu of a server." ~~The rail stays navigation only and gets no menu.~~ *Superseded 2026-10-23 by the rail-menu ruling for seventeenth note 16, below.**

## Dropping a setup the server never ran

**It is in the menu and not on the row, and the marker asked whether it should move.** It should not. The [architecture](../Architecture/Servers.md) B5 gives the bit three ways out — connecting, the user dropping it, and removing the server — and two of those are already on the row, where the connect control reads *Connect and set up*. Dropping is the third and it is the rare one: a user who ticked the option and has changed their mind.

**Putting it on the row would put a control for the rare case beside the control for the ordinary one**, on a list whose whole job is comparing servers at a glance. The menu's slot 3 is where every shape keeps what it can do about itself, and this is exactly that.

**What it says is what it costs, because the wording is the whole of it.** *Connect without setup* — one act, said as one act. Connecting and forgetting are not two choices here: the [architecture](../Architecture/Servers.md) B5 clears the bit on any connection and hands the remembered setup to Server Setup, so a plain connect carries the setup out, and the only way to connect without it is to drop it first. The menu item drops the setup and then connects, so the server connects **without** setting it up, and it says so. *Corrected 2026-10-03, ruled by the build runner under the Creator's standing delegation after the 38's piece 4 found the contradiction: this sentence said the item drops the setup without connecting, against its own words and "one act" above. The item's words won, as built.* *Corrected again 2026-10-03, the sentence before it: it said a user who connects has already dropped the setup, where B5 hands the setup on and the connection carries it out. Brought into line with the item as built and with the sentence after it; no auditor has seen it.*

## Connected and needing a new key at once

**The server pane’s heading repeats the row’s control, beside the address, above Files and Set up.** It uses the same command and availability: *Connect* disconnected; the control’s spinner during an initial attempt; *Connected* with the disconnect action; *Reconnecting…* through repair with disconnect available to stop attempts; *Connect and set up* for a remembered setup; unavailable *Connect* for a missing key, with its explanation. A restart reads *Restarting* as on the row. Failed connections say why beside the control and offer the same retry; pending actions cannot be started twice. Missing SSH tools withdraw connection exactly as on All servers. Selecting the server only opens its pane.

**Right-clicking a rail server opens that server’s All servers menu beside its rail row.** Shift+F10 or the Menu key on the focused row opens it too; arrows move between available items, Enter chooses, Escape or clicking outside closes and returns focus to that row. It reuses all five menu shapes, slot order, disabled items and actions, including Rename. It belongs to the clicked server even if another pane is open. State changes refresh it; removing its server closes it. A right-click opens no connection and changes no selected pane. Menu actions have the same confirmations and unavailable reasons as All servers.

*Added 2026-10-23, Creator-stated, seventeenth note 16: “Also, why aren’t there right-click menu items for servers?” The runner settled the rail as the place intended and kept the existing menu.*

**Those four are shapes, and one pair of them can both be true.** The [Servers spec](../Specs/Servers.md) says so plainly: a key can stop being usable while the user is connected — they delete it in Keys, or its file moves — and Smoozy leaves the connection alone. So the server is working and its next connection cannot be made. Label: **Creator-stated** (2026-08-13) as the design answer; the state itself is the spec's.

**Connection wins the control, the key takes the extra line.** The connect control reads connected, because it does: files browse, jobs run, the server is reachable. The row's extra line is where the key mark goes, in the same place a failure that is still standing would sit — which is what that line is for.

**The key mark is not a failure, and the row draws it as what it is.** It takes no red dot and no cross: nothing went wrong, and there is nothing to clear — the mark stands until the server is given a key, and only a key ends it. It is a statement about the server, in the ordinary face, and it sits in the failure's slot because that line is the row's one spare and not because it is a failure.

**Where a failure and the key mark are both true, the failure takes the line.** A server that is disconnected and whose key is gone has both — and the failure is the one that just happened, the one the whole-app shape gives a way out on, and the one that will go by itself when the condition ends. The key mark waits and takes the line back afterwards, since it is standing either way. Nothing is lost by waiting: the two ways out of a gone key are in the menu's slot 3, where every shape keeps what it can do about itself.

**The menu is the connected one with the two ways out added in slot 3.** Nothing is shown unavailable, because nothing is: disconnect, browse and set up all work. What is added is the pair from the needs-a-new-key menu, in the slot every menu keeps for what its own shape can do about itself.

**The line says what it costs rather than what happened.** *Key missing. Choose a new key to reconnect.* The user does not need telling that a key went missing; they need telling that the working connection in front of them is the last one they get.

**And it is the one case where a connected server can be dimmed nowhere.** In the rail there is no room for the mark, and a connected server reads bright by the [whole-app Design](Whole%20app.md)'s rule. It stays bright. Dimming a server that is connected to say something about its key would say the wrong thing with the strongest signal on the screen.

## No servers yet

The [Whole app spec](../Specs/Whole%20app.md) promises that whenever the list is empty, Smoozy lays out what to do and in what order. That promise lands on this screen.

**It is a numbered list, because it genuinely is a sequence** — and the order is the part that matters. The public half of the key goes to the provider **before** the server is rented, because the server is handed the key as it is made.

It says plainly that Smoozy rents nothing, and that the server must be Linux — and **which** Linux, because Smoozy only sets up Ubuntu and Debian machines. **Added 2026-08-22, on the Creator's ruling of the same day**, which is what the [Whole app spec](../Specs/Whole%20app.md) now promises: the user is told which kinds to rent in the same breath as being told to rent Linux, so they meet the boundary before the money is spent rather than after.

## Adding a server

**A form of five fields and two choices.** Host, port starting at 22, username, which key — a dropdown of the usable keys ([Keys](Keys.md), *Opened from a server dialog*) — and an optional name. *(The dropdown added 2026-10-12, Creator-stated, the Creator's eighth notes (2026-10-11), note 3.)* Then the server's colour. Then the two setup options.

**The provider's connection line is pasted into Host.** A paste that starts with `ssh` and a space fills Host, Port and Username from the line, and Host then holds the host alone; anything else pasted is a host, as before. The fields can be read and changed before *Add*, and nothing is saved until it is pressed. **A line missing a piece fills what it has and names what it lacks in one line under Host**, in the grey the form uses for a statement — *No username in that line.* or *No host in that line.* — never in red, since nothing the user typed is wrong. A line with no port fills 22 and says nothing, because that is what the line means. Nothing else in the line is mentioned. The field's own placeholder says the paste works: *Host, or paste the ssh line*. *Added 2026-10-04, on `Specs/Servers`' paragraph of that day. Design conforming to Architecture under Rule 10: `Servers` PL1–PL3. No auditor has seen it.*

**A line of a form that cannot carry files fills nothing, and the one line under Host says so in the same grey.** The provider reading of 2026-10-07 found one such form, Runpod's proxied line, so a paste of it leaves every field as it was and reads *This command can't transfer files. Paste the one under SSH over exposed TCP.* *SSH over exposed TCP* is the name Runpod's own console gives its direct line, so the user finds it where they copied the first. *Added 2026-10-07 by the builder of step 10c's second notes, piece 15, replacing the sentence that waited on that reading. Design conforming to Architecture under Rule 10: `Servers` PL3. No auditor has seen it.* *Words changed 2026-11-01 (was *Runpod's proxied line can't transfer files — paste the one under SSH over exposed TCP.*; the build runner's ruling: words the user reads never name a provider): Creator-stated, the second words sweep. No auditor has seen it.*

**The colour arrives already picked, and it is one no other server is using.** Smoozy takes the first of the eight nobody has taken; the user can change it to any of the eight, including one already in use. Label: **Creator-stated** (2026-08-13).

**That is the point of the badge working at all.** Its one job is *which server*, and two servers sharing a hue at ten pixels is exactly the failure the palette was rebuilt twice to avoid — so leaving the pick to a user who is halfway through adding a server is leaving it to the moment they care least. Choosing for them is not deciding for them: it is offering the answer they would have given.

**With more than eight servers there is nothing left to take.** Smoozy picks the hue used by the fewest servers rather than refusing to add one. Eight is enough for the way the Creator works — a machine per project, thrown away at the end — so this is the honest fallback rather than a case worth designing around.

**The possible-duplicate flag appears under the form as a plain statement**, naming the server it matches and saying that it may well be a different one. It never blocks, which is what makes the loose comparison the right one.

**The duplicate-name warning is a second, separate statement, and it must not read like the first.** They are checks on different things: one says two rows may be reaching the same machine, the other says two rows will be hard to tell apart in the rail. So the name one says exactly that — *Another server is already called Overnight box* — and nothing about addresses.

**It names the cost rather than the rule.** Two servers sharing a name are two identical-looking rows in a list whose whole job is telling servers apart, so the line says that in the sidebar only the badge tells them apart. That is the fact the user needs to decide with.

**Like the duplicate-address one, it never blocks**, and for the same reason: the user may well have two machines they think of by the same name, and Smoozy is not the judge of that.

**The two setup options say what ticking them does**: either connects immediately.

**Add and set up closes the dialog onto the new server's Set up tab**, where the connection and then the setup are shown as they happen ([Server Setup](Server%20Setup.md)). The user sees something happen the moment they press it. A plain add stays on the server list. *Added 2026-10-02, the Creator's note 3 on 0.1.7: "I expect Smoozy to show me that something is being done the very second I add it."*

**Where that connection is not made, the Set up tab says so** ([Server Setup](Server%20Setup.md), *This server*), **and the line above this list names the server**: *Could not connect to Hyperstack.* Above a list, *this server* names nothing. The line shows whenever the list is on screen, the newest for each server, until the next act on that server, and not past closing Smoozy. *Added 2026-10-03, the Creator's note 14 on the test copy; no auditor has seen it.*

**A required field left blank says so on the field, and not before it is left.** The line sits under the field, in the same red the page uses for a number it will not take, and it says what is missing rather than that something is: *Host is required.* A dialog that turned red as the user tabbed through it would be scolding them for not having typed yet.

**The Add control is not greyed while a field is blank.** Greying it leaves a user pressing nothing and looking for the reason; pressing it and being shown the two fields that are empty answers the question they actually have. Label: **Creator-stated** — the same reasoning the whole app uses for never hiding a control the user is looking for.

**The port is the exception and refuses in place**, like every other number the app will not take: it holds a whole number and says the range, because a port typed as text is a different mistake from a port left empty. **The range it says is 1 to 65535**, and it is a fact about the world rather than a Smoozy decision — the port space is 16 bits and 0 is reserved, so 1 is the lowest a server can be listening on. Label: **documented** for the space, **inferred** for the floor; see [Sources](Sources/Servers.sources.md). **The field is checking that a number is a port number at all**, not predicting that the connection will work: whether a particular machine answers on it is something Smoozy learns by trying.

## The Edit dialog

**It is the Add form with the same fields, and three things Add does not have.**

**It keeps an unusable key selected rather than clearing it.** A server needing a new key still shows the key it was added with, marked as missing. Clearing it would lose the one piece of evidence the user needs — *which* key this server was using — at the exact moment they are trying to replace it. The [Keys](Keys.md) screen owns what a gone key looks like and this dialog uses that mark.

**It warns before a host or port change when jobs are running on the old machine.** The [architecture](../Architecture/Servers.md) D13 drops everything Smoozy read from the old machine on such a change, and W34 composes the warning from the durable record — so it names **jobs Smoozy has not seen end**, and the wording carries that bound rather than claiming they are still running. It says the change points this server at a different machine, names those jobs, and says they stay there. It does not block, and it cannot be switched off.

**A username change carries the warning too, with a different lead.** The jobs stay on that machine, and Smoozy can no longer reach them once it logs in as someone else. The warning leads *This logs in as a different user.*, names the jobs Smoozy has not seen end in the same shape as the host warning, and ends *They will be unreachable.* Nothing in it says the work is still running (W34). Where the host or port changes as well, the host warning is the one shown, because leaving the jobs on another machine is the larger fact. A key change and a rename carry no warning (W37). The drawing's `sv-edit` state shows the host case.

**And it is where a remembered machine password is cleared.** The whole-app password prompt names this dialog as the place, so the control has to be here: one line saying a password is saved for this server, with a control that clears it. It is absent where none is saved, rather than greyed.

**One thing it does silently and says so afterwards.** The [architecture](../Architecture/Servers.md) D14 clears a remembered password without asking when the username changes, because a password belongs to a login. The dialog says the saved password is cleared, under the username field, as a plain statement, so a user who changes a username is not surprised to be asked for a password next time.

**And one more thing Add does not have: the server's fallback folder.** One optional field under the name, labelled *Fallback folder*, empty by default with *Smoozy's folder* as its placeholder, and one grey line under it: *Where jobs go when their output folder can't be used here.* The path is typed: nothing in the drawing picks a folder on a server, and a picker would be new surface. The dialog does not check it; whether it can be used is found at submission, and a job that cannot use it goes to Smoozy's folder and says so as any fallback does ([Queue & History](Queue%20&%20History.md)). Saving it alone warns about nothing and leaves the connection alone. **It is not on Add**: it names a place on that machine, which a user adding a server has usually not looked at yet, and Add stays the five fields a pasted line fills. *Added 2026-10-04, on the Creator's want of that date to choose the fallback folder per server. Design conforming to Architecture under Rule 10: `Servers` D3 and E3, `Job files and outputs` P23. No auditor has seen it.* **Corrected later on 2026-10-04, by the build runner's ruling: the dialog does check one thing.** A path that does not start with `/` is refused in place, as the port is, with one line under the field in the same red: *Enter a full path, starting with /.* — so a typo never sends jobs somewhere else. Everything else about the folder is still found at submission, and a job that cannot use it says so on its entry ([Queue & History](Queue%20&%20History.md)).

**And two things Add has that this does not: the colour, and the two setup options.** The badge is settled when the server is added and does not change afterwards, which the [whole-app Design](Whole%20app.md) owns; the setup options belong to adding a server rather than to editing one.

## Removing a server

**The box says the server is removed from Smoozy and nothing on the server changes** — *Removes the server from Smoozy. Nothing on the server changes.* — which is the part worth saying, since destroying it happens at the provider. What exactly is removed used to be listed out; the word *removes* covers it, and a user who has just asked to remove a server is not weighing whether the port number survives.

**Where the server has work on it, the two ways out are offered as statements rather than as buttons called Yes and No.** *Leave it running* says the render and queue keep running, untracked. *Stop and clear first* says the waiting jobs go, then the render stops, and names what that costs: the frame in hand may not be saved, and the frames already written stay on the server.

**A count of nought is left out, not said.** With nothing waiting, the warning reads *1 job is rendering on this server.* and *Stop and clear first* reads *Stops the render. The current frame may not be saved; rendered frames stay on the server.* With nothing rendering, the warning reads *2 jobs are waiting on this server.* and *Stop and clear first* reads *Clears the 2 waiting jobs.* The same holds for the warning when Smoozy is not connected, and in the **Update now…** box, which uses these words. *Added 2026-09-30, from step 10c's eleventh-pass fix piece 6: once the box took its counts from the server's reading, "Clears the 0 waiting jobs" could reach the screen. Design conforming to Architecture under Rule 10: `Servers` W19.*

Where the server is not connected, only *leave it running* is offered, and that warning can be turned off — which the [Servers spec](../Specs/Servers.md) allows because it rests on stale knowledge.

**And there the warning says that what it names is what Smoozy last knew.** The [architecture](../Architecture/Servers.md) W19 composes it from the last-known reading, *"saying so"*, so neither the warning nor *leave it running* states the machine's work as present fact: the warning reads *When last connected, 1 job was rendering and 2 were waiting*, and *leave it running* says that whatever is still running keeps running, untracked. A warning worded as though the connection were live would state as now what may have finished hours ago. *(Added 2026-09-17 from a code round, under Rule 10.)*

**The switch-off says what it covers, and what it covers is every disconnected server.** The [architecture](../Architecture/Servers.md) W20 puts it as one row in the one suppression table, so there is no per-server version of it to offer. The line reads *Don't warn me again*, and beneath it, that it covers every server Smoozy is not connected to and where it is restored — the same shape the dependency band uses, for the same reason: a switch whose reach is not stated is a switch the user turns off once and is surprised by later.

**The connected warning has no switch at all**, and the difference is stated rather than left to be noticed. That one is composed from a live reading, so it is never wrong; this one rests on what Smoozy last knew and may name work that finished hours ago, which is exactly what earns it a way out.

**Where Smoozy holds nothing about the server's work, there is no warning and none is fetched.** The [architecture](../Architecture/Servers.md) W19 is firm that Smoozy does not reach out to find out — a plain confirmation stands in its place. So the absence of a warning here never means *nothing is running*; it means Smoozy has nothing to say, and the confirmation does not pretend otherwise.

### When stop and clear does not work

**It is one request with two ordered halves, so there are three endings and not two.** The [architecture](../Architecture/Servers.md) W18 clears the waiting jobs first and stops the render second, both recorded before anything acts — so a connection dropping mid-way changes nothing, and the halves can part.

- **It could not begin at all.** *Could not stop and clear the jobs on this server.* *Nothing changed.* The lead names no cause: a dropped connection ends here, and so does a Smoozy on the server too old to answer the request, which Smoozy did reach (the [architecture](../Architecture/Servers.md) W21). The box takes the whole-app failure shape and offers the two ways on: *Retry*, or *Remove anyway*. *Changed 2026-09-29, out of step 10c's stop and clear plan pass: the drawn lead was* Smoozy could not reach RunPod 4090., *which is false of a server that refused. Design conforming to Architecture under Rule 10.*
- **The clear was refused, so the stop was never attempted.** That is W18's own rule and the wording says both halves of it — nothing was cleared, and because of that the render was not stopped either. Otherwise a user reads a refused clear and assumes the render came down.
- **It half-worked.** The waiting jobs went and the render did not stop. Smoozy says exactly that, and offers the same two ways on.

**All three name what actually happened rather than saying it failed.** The [spec](../Specs/Servers.md) asks for that in as many words, and it matters here more than most places: the difference between *your queue is empty and your render is still going* and *nothing happened* is the difference between a machine still costing money and one that is not.

**And removing anyway stays available in all three.** The user asked to remove the server; a tidy-up that would not run is not a reason to keep them from it.

## Keyboard shortcuts

**Return opens the chosen server. The delete key removes it, asking exactly as the control does.** Nothing else is bound: connecting, disconnecting and editing are all acts this screen asks about or opens a dialog for, and the whole-app rules in [Whole app](Whole%20app.md) keep keys off them.

## A changed identity

**No fingerprint is shown, anywhere.** That is the [Servers spec](../Specs/Servers.md)'s policy and the [Whole app spec](../Specs/Whole%20app.md)'s security concern agreeing: the identity means nothing to a 3D artist, and a screen nobody can act on is worse than no screen.

So the message says what it might be, admits Smoozy cannot tell which, and says plainly that **accepting trusts the server without verifying it.** Declining leaves it disconnected.

## What this document left open, and how it was answered

Every marker this document once carried is closed, and the answers are kept together here rather than folded back into the body. **Nothing below is still open** — the heading used to say *Left for later*, which was true when the markers were markers and stopped being true the day they were answered. Each bullet names the question it settles, which they did not when the questions were still above them.


- **Whether the guidance for an empty server list stays reachable once there are servers.** **Settled at Design (2026-08-23): it stays**, reachable from the Add Server dialog, which is the one place it is wanted a second time — a user who already has servers, renting from a provider they have not used before. Anywhere else it would be in the way.
