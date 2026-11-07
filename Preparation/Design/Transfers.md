# Transfers

*What the one window holding every upload and download looks like, and how an auto-download is told from a request made at a moment. The conventions it follows are the [whole-app Design](Whole%20app.md)'s and are not restated here.*

## Where it sits

**A screen in the rail**, below the queue and history, because a transfer belongs to no one server — several servers can be moving files at once.

**Connections.** It serves the [Download spec](../Specs/Download.md) and the transfer half of the [File Browser spec](../Specs/File%20Browser.md), which settles that uploads and downloads are watched in one place. Uploads are started in [File Browser](File%20Browser.md) and by [Dependency Upload & Remap](Dependency%20Upload%20&%20Remap.md); an auto-download is switched on in [Render Settings](Render%20Settings.md) or from an entry in [Queue & History](Queue%20&%20History.md).


## Sending one download somewhere other than the standing folder

**The [spec](../Specs/Download.md) grants it and nothing had ever drawn it**, which was found on 2026-08-25 when the markers that spec hands Design were read one by one. [Settings](Settings.md) already tells the user in as many words that a single download can be sent elsewhere without changing the standing folder, so the promise was on screen with nothing behind it.

**It is a second act beside the first, never a question asked of every download.** The [spec](../Specs/Download.md) is firm that Smoozy does not ask where to put each download — the standing folder is the answer unless the user says otherwise — so the ordinary press stays one press and the variant is a control of its own, marked with the ellipsis that means a picker opens.

**Where it sits:** *Download to…* under *Download* in each of the file browser's three right-click menus, and *Download all to…* beside *Download all* in the header of a queue or history entry. **The file browser's toolbar keeps one Download and gains nothing**, because a strip of icons cannot carry a variant and the same file's right-click menu is one press away.

**Nothing on the picker offers to make the choice stick.** No *remember this*, no *make this my download folder*. The [spec](../Specs/Download.md) says the standing setting is untouched, and a tick that quietly changed it would be exactly the surprise the line in [Settings](Settings.md) exists to prevent. Label: **settled at Design** (2026-08-25).

## A download meeting files already there

**The already-exists question was drawn once, for an upload, and the download direction was never drawn at all** — found the same day and from the same read. The [spec](../Specs/Download.md) makes it a manual-download question in as many words.

**It is the same box with one sentence changed, not a screen of its own.** The question is the same question and the four answers are the same four — replace, keep both, skip existing, stop — so a second box would be a second copy of one decision. What the direction changes is the sentence naming what is already there: *40 of these 300 files are already in ~/Renders/kitchen*, the user's own folder rather than the server's.

**Two things that belong to an upload are absent rather than greyed.** The live-job warning, which is about the machine's own work and cannot apply to files arriving on the user's own computer; and the line saying an upload was interrupted, which is what puts *Skip existing* forward — a download has no such record, so nothing is put forward and the four answers sit level. **That makes the download reading the plainest of the three**, which is right: with nothing to say about the collision beyond the count, Smoozy says the count and stops. Label: **settled at Design** (2026-08-25).

**Where nothing is already there, the box says nothing about it.** A download that stops only for a file that may be incomplete, or for something on the server it could not read, has no count and no *Replace*, *Keep both* or *Skip existing*: it says what it has to say, with *Download* and *Stop*. A download where nothing at all could be read does not start, and says so. *Added 2026-10-01, from step 10c's Vast pass, where one file Smoozy could not read put up* 0 files are already there *with all four answers. Design conforming to Architecture under Rule 10: `Download` M6 stops only for a collision, and J10 makes the rest the user's go-ahead. No auditor has seen it. — Claude Opus 5.5.*

## Four kinds, told apart by a tag

**Running and waiting transfers come first; finished transfers follow. Within each group, newest started comes first, across every server and kind.** Each request’s header shows *Started 21 Oct 2026, 14:22:08*, in the computer’s local time. This is when that upload, manual download, dependency upload or auto-download arrangement first starts, not when a file arrives or the connection returns. An auto-download keeps its place through waits, new files and reconnections; switching it off and starting a new arrangement gives it a new start. Filtering does not reorder. Older rows with no saved start follow dated rows in their group and read *Start time not recorded*. Equal times use the saved desktop order; no server clock or separate table row number decides.

*Creator-stated, seventeenth note 8, 2026-10-21: “it’s very unclear in what order what happened there and why they are sorted the way they are.” The runner chose newest started within the existing two groups, with the start shown.* *Added 2026-10-23, the build runner’s ruling under the Creator’s standing delegation, from the seventeenth-notes design brief. No auditor has seen it.*

**Auto-download**, **Upload**, **Download**, and the dependency check's own — one list, with a tag on each rather than four separate lists. Four lists would make the user look in four places for one answer.

**Every transfer carries its server's badge and name.** The list spans servers, so a row that does not say which one it belongs to answers nothing.

**The rows alternate ground.** A request carries several lines of its own — the header, the bar, where it is going, then a file each — so without it one request's last file reads as the next request's first. The queue list already works this way and for the same reason. Label: **Creator-stated** (2026-08-13).

**The direction is an arrow on the tag, read before the word is.** Up for anything going to a server, down for anything coming back. Four tags in four colours still take a moment to tell apart; an arrow does not. Label: **Creator-stated** (2026-08-11).

**The list can be narrowed to one server, not reordered.** Every server is the default, because a transfer belongs to no one server. Narrowing is for when the user is watching one machine, and an empty result says so rather than showing a blank screen. Label: **Creator-stated** (2026-08-11).

## An auto-download is not a request, and reads differently

The [Download spec](../Specs/Download.md) draws that line hard: a download the user starts copies files that already exist, while an auto-download waits for files that do not exist yet. So:

- **It has no end until the job ends.** Its bar runs against what the user asked for rather than against a fixed set.
- **It says what it is narrowed to and where the files land.** Where the files land was fixed when it was switched on and is not asked again while the arrangement runs — the [architecture](../Architecture/Download.md) R11 has switching it off and on reset it and ask once more, which is the one way it moves; what it is narrowed to can still be changed from the job's entry in [Queue & History](Queue%20&%20History.md), and so can the switch itself. This screen shows the arrangement and does not change it — the choosing happens in [Render Settings](Render%20Settings.md) or on the entry.
- **It is turned off, not stopped.** Stopping is what happens to one transfer; turning off is what ends an auto-download.
- **It reads against what the user asked for, not against everything the job writes.** A deliberately narrowed download would otherwise report itself as permanently unfinished.
- **A first-frame check and its full range are one auto-download row**, as they are one entry in [Queue & History](Queue%20&%20History.md). The row is the full range's: its name, its start, its *Turn off* and *Open the job*. Its count holds the check's file beside the range's frames, so a check of one frame and a range of 6 reads *7 / 7* when all are home. A check whose range is gone keeps its own row. Before this the pair was two rows of one name. *Added 2026-10-31, the twenty-sixth notes, note 4 — the Creator: "some jobs appear to be duplicates". No auditor has seen it.*

**Where nothing moves for a minute with a connection standing, it says so**, in the amber of a waiting line: *Download stalled. Use Download all on the job.* A file in flight is moving, however long it takes to land. The line goes at the next file that lands. *Added 2026-11-03 ([architecture](../Architecture/Download.md) E2b): *downloading 0 / 2* stood for 38 minutes with no bytes moving and nothing said. No auditor has seen it.* *Words changed 2026-11-06 (was *Nothing is moving. Try Download all on the job.*): the last words read before the release. No auditor has seen it.*

**Its bar is drawn in files, and the reading beside it counts files** — *412 of 720 files*, the unit *When the job ended badly* below already uses. An upload's and a manual download's bars are drawn in bytes, over a total known before the first byte moves ([File Browser architecture](../Architecture/File%20Browser.md) U11 and [Download architecture](../Architecture/Download.md) M16). An auto-download has no such total and cannot have one: the files it waits for have not been rendered yet, so nothing knows how large they will be — which is why the [architecture](../Architecture/Download.md) N8 gives the arrangement no byte total at all. Label: **settled at Design** (2026-09-02).

**The four tags are one colour family of their own, and not one of the reserved five.** Which way a transfer is going is not a meaning the reserved colours carry, and the tags used to borrow three of them — green for an upload on the one screen where green already means finished, the unconfirmed grey for a download, the linked-library blue for a dependency check. They are now one neutral hue at four lightnesses, running from most emphasised to least — auto-download, upload, download, then the dependency check — which is darkest to lightest on a light ground and the other way round on a dark one, with the arrow carrying direction. [Design values](Design%20values.md) holds the tokens. Label: **Creator-stated** (2026-08-13).

**The progress bars are untouched by that** and keep the colours they had. Label: **Creator-stated** (2026-08-13).

**It is called *auto-download*, never *standing*.** A label has to say what the thing does, and *standing* said nothing to anyone who had not read this document. Label: **Creator-stated** (2026-08-11).

**An upload request that never started reads *Not started*.** A request asked for while the connection was down, whose walk never began, has that status in place of *— did not finish*, no bar and no Stop. A request whose question is open reads as under way; one stopped at that question keeps *— did not finish*, as does any request that began and then stopped. The upload’s first sentence stays as it is. Added 2026-11-04, the build runner’s ruling for the film’s fault 3.

## A request shows the whole and the files under it

**The whole** — how many of how many, a bar, and where they are going.

**The bar is the blue the whole-job bar uses**, not the yellow of a frame in hand. A transfer is one reading of overall progress against a known total, which is the same question the whole-job bar answers — so it takes the same colour. Label: **Creator-stated** (2026-08-13).

**The files** — a few at a time, each with its own small bar. A file waiting its turn says so, because only a set number run at once per server and the user does not set that number.

**A download of a folder is on Transfers from the moment it is asked for.** Until Smoozy knows what is in the folder, its row names the folder and reads *Counting files…*, with *Stop*. Then it reads as any request. *Added 2026-10-01, from step 10c's twelfth pass, where a thousand-file download showed nothing for three minutes. Design conforming to Architecture under Rule 10: `Download` M1 writes the request once the enumeration is complete, so the row before it is held for the run and never written down.*

**A file that failed says why, carries its own Retry, and does not hold up the rest.**

**It takes the whole-app failure shape**, on the row: the red dot, one plain sentence saying what did not happen, then *Retry*, then whatever the server said behind the control that names whose words they are, then the cross that clears it. That is the order the [whole-app Design](Whole%20app.md) fixes, and the cross is the part that must survive wherever the shape is collapsed. The sentence is Smoozy's own, in ordinary words; the server's words are behind the control and never in front of the user. Where the server said nothing, that control is absent rather than opening an empty panel.

~~**Where the refusal was a permission refusal on an upload or a dependency check, *Try again* carries the admin-access offer.** [File Browser](File%20Browser.md) owns that answer and its three outcomes, including the machine where there is no route to one and the row says so and stops. An ordinary *Try again* on a refused write would retry the same refused write for ever.~~ **Corrected 2026-09-27: an upload's permission refusal carries neither the admin-access offer nor *Retry*, and the row says only what happened.** Out of step 10c's fix piece 7; no auditor of this document has seen it. There is no elevated upload to offer. The [File Browser architecture](../Architecture/File%20Browser.md) P6a says why: the acts admin access can retry are a listing and the machine's own remove, move, copy or make-directory, and no transfer is among them. An upload's bytes move on a channel that connects as the login, and the part-sent file sits inside the folder being written to, so a folder the login cannot write to refuses both. ~~**A dependency check's refused files are uploads too, so the same holds there.**~~ **Narrowed later the same day, 2026-09-27: a dependency check's refused files are uploads too, so they carry no admin-access offer either — but their plain *Retry* is not this screen's to take away.** On this screen their rows already carry none and point back at the check panel (*The dependency check's own transfers*, below). On that panel each such file keeps its own *Retry*, because the [Dependency Upload & Remap spec](../Specs/Dependency%20Upload%20&%20Remap.md) has every file that could not be sent tried again on its own, and the [Dependency Upload & Remap architecture](../Architecture/Dependency%20Upload%20&%20Remap.md) A11 answers that without exception: a retry is a new transfer, and one that fails again leaves the row as it was. P6a rules out only the elevated retry. Conformed under Rule 10; no auditor of this document has seen it. The struck sentence's last reason still stands, and it now rules out the plain *Retry* as well: it would retry the same refused write for ever. So the row takes the failure shape without its retry — the red dot, the sentence, the server's own words behind the control, the cross — and stops. Design conforms to Architecture here under Rule 10.

**A download's permission refusal carries no such offer, and the row says only what happened.** Narrowed 2026-09-06, out of the code round on `Download`; no auditor of this document has seen it. This sentence used to cover all four kinds of transfer, and there is no elevated download to offer: the [Download architecture](../Architecture/Download.md) M5 rules one out on two separate grounds — elevation runs on the command channel and never inside the part of the connection a file's bytes move on, and copying a refused file somewhere readable would change the machine, which downloading never does — so a user who granted admin access here would watch nothing change. The reasoning above is upload-side reasoning: a refused write retried for ever is a real trap, while a download's permission refusal is a **read** the login may not make, and there is nothing behind the offer to make it succeed. So the row takes the failure shape and stops there, with the server's own words behind the control as always. The elevated **listing** is still reachable in [File Browser](File%20Browser.md), which is where a user goes looking when they think they should be able to see a file.

A file Smoozy downloaded but could not put in place on this computer reads **Download failed.** with the reason behind *Details*. A request with a file that failed never reads *done*; it reads *— did not finish* with its count. *Added 2026-10-01, from step 10c's thirteenth pass, where a* Replace *that wrote nothing read* done.

Nor does a download stopped at the already-there question, or one that met something on the server it could not read. *Added 2026-10-01, from step 10c's Vast pass, where a download stopped at that question read* 0 files into … · done. *No auditor has seen it. — Claude Opus 5.5.*

Nor does an upload stopped at the already-there question, or one that ended before its look at the server's folder came back — it reads *— did not finish* with what it asked for. *Added 2026-10-02, from step 10c's check before 0.1.7, where a file dragged in over a connection that was gone read* 0 files into /home/ubuntu · done. *No auditor has seen it. — Claude Opus 5.5.*

**Stop sits on the request.** Both specs give the user one in either direction and both make it act on the whole request — the files already finished stay where they landed, the ones still waiting are never sent, and the one in flight is stopped. So it sits in the request's header beside the count, where an auto-download carries *Turn off* instead. Label: **Creator-stated** (2026-08-13).

**Once there is nothing left to stop, that same place carries the way back to the thing the transfer was about.** A finished download reads *Files on your computer*, a dependency upload reads *Open dependency check*, and an auto-download whose job has ended reads *Open the job*. It is one control in one place, replacing Stop rather than sitting beside it — a header with an empty space where the control was is worse than one carrying the way back.

**Where what a download brought is gone, *Files on your computer* says so and opens nothing in its place**: *Folder not found.*, or ***File not found.*** where the download was one file. *Added 2026-10-18 by the build runner, from piece B of the Creator's thirteenth notes (note 4; `Design/Queue & History`'s sentence of the same day). No auditor has seen it.*

**The way back goes to the thing as it stands, and starts nothing.** *Open dependency check* shows the check's rows as they are and does not read the file again; where Smoozy holds no check for that file, as after a restart, the panel's *nothing was read* state shows, with ~~*Check again*~~ *Check dependencies* *(changed 2026-10-18, the Creator's thirteenth notes, note 2: nothing has run, so* again *is false; [Dependency Upload & Remap](Dependency%20Upload%20&%20Remap.md#the-search-folders-on-this-panel))*. *Files on your computer* opens the folder the download went to with what it brought selected: the job's folder for a whole job, or the one file or folder picked; where several things were picked, it opens that folder with nothing selected. *Added 2026-10-03, the Creator's notes 8 and 15 on the test copy: "When clicking Open the check from transfers I see an empty list", and "Files on your computer … doesn't select the folder itself, so I have to guess which one it is." Design conforming to Architecture under Rule 10: `Dependency Upload & Remap` G17, `Download` H5 and H7, `Desktop app and delivery` K21. No auditor has seen it.*

**And a single transfer can be stopped too, which reverses what this document used to say.** It read *"There is no Stop on a single file"* — on the ground that stopping one file of three hundred is not a thing the user asked for, and a control that looked like it would be read as one — and that is no longer the ruling. The [Download spec](../Specs/Download.md) has said since 2026-07-22 that a user who wants one transfer gone immediately *"can stop it by hand, like any other transfer"*, and the two ran side by side for a week before anyone noticed. **The spec wins and this screen owes the control.** Label: **Creator-stated** (2026-08-20).

**How it looks is settled just below**, and the old reasoning's risk is what settles it: a per-file control sitting on a request of three hundred rows would be read as the way to stop the download, so the two Stops must not read as competing versions of one control.

### What Stop says before it does it

**It asks, and the box names what is already across.** Stopping a request of three hundred files is not one act with one outcome: the files that landed stay landed, the ones still waiting are never sent, and the files in flight are thrown away part-sent — files, because this screen sends a few at a time, up to four on an upload and two on a download. So the box says those three things in that order, with the counts, and names each file that is moving at that moment: *104 files already there: kept. 206 will not be uploaded: not started. textures/wood_oak.png and textures/bark_rough.png: uploading now — partial upload discarded.* However many are in flight is how many it names — never one by assumption.

**The last of the three is the one worth the box.** The other two are what a user would guess; that the files being uploaded lose what has already been sent is the part they would not, and on a 2 GB cache it is the part that costs.

**It stays on a row waiting for a repaired connection**, which the [spec](../Specs/File%20Browser.md) requires. The request is still Smoozy's to stop even while nothing is crossing — and the box there says one thing differently: nothing is in flight, so nothing is thrown away.

### Stopping one file

**The control is in the row's own right-click menu, never on the row.** That is what keeps it from reading as the way to stop the request: a control sitting on three hundred rows invites the user to press the nearest one, and the two Stops must not read as competing versions of one control. The request's Stop stays where it is, in the header, as the visible one. The menu holds **Stop** and nothing else, and it opens the way a file's menu opens in [File Browser](File%20Browser.md) — right-click gives what acts on the thing clicked, and the row carries no menu control of its own. A file with nothing left to stop opens no menu at all rather than an empty one.

**It asks nothing.** One file's worth of part-sent bytes is not a decision the user needs walking through, and the request's own Stop is where the consequences live.

**A file already finished has no Stop**, because there is nothing to stop — it is not greyed, it is absent, which is the same rule the whole app follows for a control that cannot apply.

### The dependency check's own transfers

**They are one kind among the four and sit in the one list**, which is the [spec](../Specs/File%20Browser.md)'s own rule: uploads and downloads are watched in one place. Four lists would make the user look in four places for one answer, and a fifth would be worse.

**What tells them apart is the tag and what the request is called.** The tag reads **Dependencies** and is the least emphasised of the four; the request is named after the `.blend` rather than after a folder — *For `kitchen_v14.blend`* — because that is what the user asked for and where they would go to see it.

**A dependency upload is named for its file**: *For* in the secondary ink at the ordinary weight, the file's name as the name — so the two never read as one. *Added 2026-10-18, the Creator's fourteenth notes, note 4: "for and the blend name look too similar. Italics for something maybe? Or even a color?" Neither: italics mean a value Smoozy cannot confirm ([Design values](Design%20values.md)), and every colour here already carries a meaning. No auditor has seen it.*

**Its rows are the same rows**, with one difference: a file that failed here says which row it belongs to on the check panel, and the way back is to that panel rather than a plain Retry. The check owns the retry — the [spec](../Specs/Dependency%20Upload%20&%20Remap.md) has each failure tried again on its own without re-running the check — so this screen points at it rather than doing it twice.

### What happens to a finished request

**It stays, quiet, until Smoozy closes.** A request that vanished as it finished would take with it the one thing a user comes to this screen for afterwards: whether it actually all got there, and where it went.

**Finished requests collapse to one line each** — the count, the destination, and when — and sit below whatever is still running. Nothing running is ever pushed down the list by something that has finished.

**The finished line's ending word takes the colour a job's ending takes in History: green where every file asked for arrived, red where one failed, grey where the user stopped it or where an arrangement only ended.** The word is not dimmed; the rest of the line still is. *Added 2026-10-10 on the Creator's seventh notes, note 9. No auditor has seen it.* **Red also where the transfer ended because its line was cut or its server's period ended, not by the user** — the Creator, 2026-10-10, asked whether such a transfer should show red: *"Yes, it should."* Grey is only the user's own stop and an arrangement that only ended. *(Runner's ruling, 2026-10-10, after piece Z5b: a transfer whose server's period the user ended — *Disconnect*, saving an edit, removing the row — also ends red, since it did not finish and the user did not stop it; only *Stop* and Smoozy closing are grey.)*

**They go on closing, and the line along the bottom already says so.** There is no list of past transfers: [Queue & History](Queue%20&%20History.md) is where a job's outputs are found afterwards, and a finished transfer is not a record of anything.

### What a past sitting left behind

**A request that never finished does show here on the next sitting — as a leftover, with a remove control.** The [architecture](../Architecture/Download.md) M13 and the [File Browser architecture](../Architecture/File%20Browser.md) U8 both put it on this screen by name: an upload or download that ended with the app leaves part-sent material staged on disk, and the user is told it is there and can remove it. **Which disk is not the same in the two directions.** Staged material always sits beside where the file was going — the [Server protocol and transfers architecture](../Architecture/Server%20protocol%20and%20transfers.md) S1 — so an upload's leftover is on the server, in the folder the files were being sent to, and a download's is on the user's own computer, inside the folder the files were coming to (S28). The row says which, because *on this computer* said of an upload promises a user pressing Remove that their own disk is about to be freed when it is not. Remove clears the staged material wherever it actually sits. No new page exists for this — the transfer window is where transfers already live. **A leftover is not a past transfer**: it is an unfinished one whose staged files still take up space, which is the whole reason it is worth a line. It does not carry on by itself — closing Smoozy stopped it — and it offers no resume: uploading again is asked for where uploads start, in [File Browser](File%20Browser.md).

**An upload's leftover says the files are there and never says how much room they take.** The [File Browser architecture](../Architecture/File%20Browser.md) U8 settles it and gives the reason: the only two figures Smoozy durably holds are the total the finished files were expected to come to, and an accounted boundary that the [Server protocol and transfers architecture](../Architecture/Server%20protocol%20and%20transfers.md) S10a shows can sit either above or below what is really staged. Neither is a truthful reading of the space used, and neither may be relabelled as one. The real size is the server's own to report, and this line is read with that server not connected, so it cannot be asked either. **So the line reads *Partial files still use space on the server*, with the server named** — the files being there is what earns the Remove control, and a wrong number beside it would be worse than none.

**Whether a download's leftover can carry a size is not settled, here or anywhere.** Its staged files are on the user's own computer, so the size could in principle be read off the disk — but nothing composes that reading: the [architecture](../Architecture/Download.md) M13a presents local staging with its destination folder and a Remove control and names no size, and the same two record figures are no more truthful on this side than on the server's. So this document rules nothing about a figure there. **Until something does, both directions read the same and neither carries one**, a download's naming its folder where an upload's names its server. That is the reading that changes least and cannot be wrong. Label: **settled at Design** (2026-09-02).

**A finished request does not collapse where it carries an amber line.** That line is the whole reason the request is worth reading after it ends, so it keeps its lines and its links. Two requests carry one. The first left something alone, and its amber line names the files it did not overwrite. The second is the one whose job ended badly, which *When the job ended badly* below describes — the frames that may be incomplete, and, where a reset split them, the two folders the files are in.

## Waiting for the connection to come back

**A transfer the connection cut is not a transfer that failed, and the two must not look alike.** The [File Browser](../Specs/File%20Browser.md) and [Download](../Specs/Download.md) specs both settle that Smoozy carries an upload or a manual download on where **Servers** repairs the connection, without the user asking again. So the row is still working, and it is going to finish.

**It reads amber and says what it is waiting for**, not red. Amber is work in progress in the [whole-app Design](Whole%20app.md), and this is work in progress that happens to be standing still. The line is the same one the server's own row carries — *Reconnecting…*, with when the next attempt falls — so the user sees the same fact in both places and does not have to join them up.

**It carries no Retry**, because there is nothing to try: the retry is Smoozy's and is already happening. A control offering to do what is being done anyway is what makes a user press it repeatedly.

**A failed transfer keeps its red and its Retry.** The difference between the two rows is the whole point: one is Smoozy still working, the other is Smoozy stopped and waiting for the user.

**And the bar does not go back to nothing.** What already crossed stays crossed in the ordinary case — both specs say nothing is sent twice — so the row keeps its progress while it waits. **But the held bar is not what tells the user this is a pause: the row's words are.** The [architecture](../Architecture/Download.md) M11a has two cases where a transfer starts over from zero while a figure still stands, so a row that let the bar say *pause* would say the opposite of the truth exactly there. Each transfer carries a mark saying whether it carries on or starts over, the row reads the mark, and the words say which it is.

## When a download meets trouble

**It says what to do, once, and keeps trying.** A full folder, a folder made read-only, a drive unplugged — every one of them is something the user can put right, and the [Download spec](../Specs/Download.md) settles that the arrangement never switches itself off. So the row says the cause and the fix, and stays.

**It says it once rather than once per file**, which is the [whole-app Design](Whole%20app.md)'s rule and not this screen's, so it is not answered a second time here.

**And it goes when the folder takes files again**, without the user clearing it. That is the same document's rule for a failure whose condition has plainly ended.

**A file it passed over is said on the request, with the list one click away.** The [Download spec](../Specs/Download.md) has an auto-download leave alone anything it did not put there and tell the user which files those were — the one case the spec calls the outcome that would destroy work unseen. So the request carries one amber line saying how many were already there and were not overwritten, and a link that names them. The count is on the row because it is the part the user has to notice; the names are behind the link because they are the part they only sometimes want. Label: **Creator-stated** (2026-08-13).

### When the job ended badly

**The bar reads *ended*, never *complete*, and that is the [architecture](../Architecture/Download.md) E1's own word.** An arrangement finishes its bookkeeping and stops; it never judges the render. So the reading is arrivals against the whole — short where short — and the word is *ended* whether the job finished, failed, was stopped or was abandoned.

**It says the count and lets the count say the rest.** *Ended — 412 of 720 files.* A bar that read *complete* at 412 of 720 would be the arrangement claiming something about the render, which is exactly what it must not do. *(Since 2026-10-19, under Rule 10: once a job has ended short of a clean finish, the whole is the files the ending found on the server, not the asked range's — the [architecture](../Architecture/Download.md) N6. A job stopped with 156 files on the server reads* Ended — 156 of 156 files *once they are all down, and a count short of that is files that did not arrive. Out of the live pass on Vast, where a job stopped at 52 frames read* ended 100 / 450*. No auditor has seen it.)*

**One line rides with it where the job ended badly**, and it is the [architecture](../Architecture/Download.md) E1 step 2's: the frames that were being written when the job stopped were downloaded and may be incomplete. They are named on the request, the same way files it left alone are named — the count on the row, the names one click away.

**Where both folders hold part of it, both are named.** A destination reset part-way leaves the arrangement's files in two places, and the request says which is which rather than naming the latest one and letting the user hunt.

### A file that changed while it was going up

**The row says the file changed while it was uploading and offers the new version — not *Retry*.** The [architecture](../Architecture/File%20Browser.md) K8 stops that transfer the moment the file moves under it, and what was staged is never published. *Retry* would send whatever is there now under the name of the thing the user asked for, which is a different file arriving with no sign that it is different.

**So the control reads *Upload again*.** One press, and it is a fresh transfer of what the file is now — which the row says outright, because the user may have saved something they did not mean to send.

**The row is amber and not red.** Nothing failed: Smoozy noticed something and stopped rather than landing half of one file and half of another. Red would put it beside the transfers that need the user to fix something.

**And it says which file**, because on a request of three hundred this is the one row that is about the user's own act.

### A part-sent file that can be carried on from

**The row keeps its bar where it stood.** What crossed and was checked stays, which is both specs' promise for the ordinary case. **The bar is not what says this is a pause, though — the row's words are**, for the reason given under *Waiting for the connection to come back*: the mark says whether the transfer carries on or starts over, and the row reads the mark, never the held figure.

**It says how much is already across**, in the same words the bar means: *118 of 312 MB uploaded* (*downloaded* on a download). That is a statement about this file and not about the request, so it sits on the file's own row.

**It carries on by itself and says so rather than offering a control.** The [architecture](../Architecture/File%20Browser.md) K1 and K2 have it resume on the repaired connection without the user asking, so the row reads as work in progress that happens to be standing still — the same amber the reconnecting rows carry. A control here would offer to do what is already being done.

### While the part already sent is being checked

**A resumed file checks what is already staged before another byte crosses, and its figure does not move while that runs.** This is the third state the [architecture](../Architecture/Download.md) M11a describes, and it sits *inside* the carrying-on state above rather than beside it: nothing started over, so what already crossed still counts — but the check reads the staged part back, writes nothing, and moves no figure until it is done. On a file nearly all the way across at 4 GB it takes about twelve minutes.

**Twelve minutes of an unchanged number needs a sentence, so the row carries one.** Its line reads *Verifying the 3.9 GB already uploaded — this can take several minutes.*, with *already downloaded* on a download. Without it the user reads a stuck transfer and starts looking for something to press. The amount named is how much is being checked, which the [Server protocol and transfers architecture](../Architecture/Server%20protocol%20and%20transfers.md) G4b supplies beside the count.

**The row is amber and carries no control**, the same as the carrying-on row above and for the same reason: Smoozy is doing the work and nothing is asked of the user. It is neither a stall nor a failure, and nothing on this screen may show it as either. **And it shows no time left while the check runs**, which could only be worked out from a figure that is standing still. Label: **settled at Design** (2026-09-02).

### A part-sent file that starts over

**Where the part already sent does not check out on the repaired connection, nothing of it is reused — and the row says so.** The [Server protocol and transfers architecture](../Architecture/Server%20protocol%20and%20transfers.md) S12 step 3 checks the staged piece before carrying on; where the check fails, step 5 sends the file again from the start. The row stays amber — Smoozy is doing the work and nothing is asked of the user — its bar restarts with the fresh transfer, and its line reads *Could not resume — uploading from the beginning*. A bar left standing there would say a pause where the bytes are in fact going again.

**A fresh sitting is the one case where nothing carries on by itself** — closing Smoozy stopped the request, and what it staged shows as a leftover, settled under *What a past sitting left behind*. Uploading again is asked for in [File Browser](File%20Browser.md), whose already-exists box puts *Skip existing* forward with the reason.

## Along the bottom

The rule people get wrong, in one line: **closing Smoozy stops transfers; auto-downloads resume on the next connection, other transfers do not.** *Words changed 2026-11-01 (was *uploads and downloads do not*; the screen reads *Auto-downloads resume on the next connection. Other transfers do not.*): Creator-stated, the second words sweep. No auditor has seen it.*

**The line names all three kinds because naming two of them is worse than naming none.** It used to say *an upload does not*, which left the download unmentioned — and a reader takes an unmentioned kind for the one that resumes. The [Download spec](../Specs/Download.md) is explicit that a download the user started does not come back on its own: reopening Smoozy does not quietly resume it, and the user asks again if they still want it. Label: **Creator-stated** (2026-08-13).

That line is about **closing**, and it is the one place the kinds still part. A connection merely dropping now carries every kind on, which is the section above — so the line has to say *closing* and not *losing the connection*, or it says the opposite of what happens.

## Keyboard shortcuts

**None of its own.** Everything on this screen stops something that is running, and none of that is a key press. The whole-app rules are in [Whole app](Whole%20app.md).

## What this document left open, and how it was answered

Every marker this document once carried is closed, and the answers are kept together here rather than folded back into the body. **Nothing below is still open** — the heading used to say *Left for later*, which was true when the markers were markers and stopped being true the day they were answered.


- The two speed caps sit in [Settings](Settings.md), each with what is actually running beside it. This screen carries the live reading and a link there.
