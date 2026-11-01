# File Browser

*What a server's files look like, how they are managed, and what the user meets when uploading onto something already there or deleting something in use. The conventions it follows are the [whole-app Design](Whole%20app.md)'s and are not restated here.*

## Where it sits

**It is a server's main face**, the first of that server's two tabs, and it fills the server's screen. That follows from the [File Browser spec](../Specs/File%20Browser.md): one server is browsed at a time, and an upload lands wherever the user is standing, so moving between servers is the rail's job rather than a control inside the browser.

**Connections.** It serves the [File Browser spec](../Specs/File%20Browser.md). A double-click on a `.blend`, or *Render settings…* with one chosen, opens [Render Settings](Render%20Settings.md) as a panel over it. *(Corrected 2026-10-06: this said choosing a `.blend` opened it.)* It shares its screen with [Server Setup](Server%20Setup.md), the other tab. Transfers it starts are watched in [Transfers](Transfers.md).

## The list

**A path along the top**, each part of it clickable, with the actions beneath.

**The file manager's eight are icons; Smoozy's two are words.** Upload, download, new folder, rename, cut, copy, paste and delete are the same eight in every file manager an artist has ever used, so the shape is read faster than the word — and eight words across the top spends most of the row's width on controls rather than on files. **Render settings…** and **Check dependencies…** keep their words, because they are Smoozy's own and nobody has an icon for them. Label: **Creator-stated** (2026-08-13).

**Every icon carries its word on hover and on keyboard focus.** An icon is a reminder, never a first explanation, and the one time a user has to hunt is the time they are new.

**Four things per file**, which is what the [File Browser spec](../Specs/File%20Browser.md) settles Smoozy knows: name, kind, size, and when it last changed. A `.blend` carries a fifth — the Blender version it was saved with — as a small tag on its name rather than a column, since only one kind of file has one.

**A folder's Size shows a dash until the user asks.** *Calculate size* on the folder's menu puts *Calculating…* in that row's Size cell, then the total in the same form a file's size takes — *12.4 GB*. Where some folder inside could not be read, the cell says *at least 12.4 GB*. Hovering a total shows what it counted — *1,204 files, 87 folders* — and on *at least*, *Some folders could not be read.* as a second line. A count that fails says *Failed* in the cell, with the reason on hover. The figure stays until the folder's parent is listed again — opening the folder and coming back, or any act that reads the parent again — and then the dash returns. A sort by size uses the figure where a folder has one; an uncounted folder sorts as before. Added 2026-10-30, Creator-stated, the twenty-third notes, note 2: *"let's add a button in the right-click menu to calculate a folder's size."* Bounded by the [architecture](../Architecture/File%20Browser.md)'s L24 to L28. No auditor has seen it.

**Every row carries an icon, and a folder is solid where a file is outlined.** That contrast is what separates the two before either shape is read; the shape itself only says which kind of file. Label: **Creator-stated** (2026-08-11).

**Five icons, and the ones that matter are not pages.** A folder, a plain page, a solid cube for a `.blend`, a landscape frame for an image, and a page with a zip for an archive. Anything else is the plain page.

**A page with a small drawing inside is a smudge at sixteen pixels, and two of them are the same smudge.** That was the first attempt and it failed exactly there: the `.blend` and the image could not be told apart. So the kinds the user actually picks between get a silhouette of their own rather than a shared page outline. Label: **Creator-stated** (2026-08-11).

**The cube is the one icon that is not a page at all**, which is right: a `.blend` is the only file in the list the user acts on differently, and its row is the only one with two actions of its own.

**They are drawn in grey, never in colour, and in the theme's own grey in light and dark alike.** Colour is reserved in Smoozy and a row of coloured icons would spend it on nothing. The [whole-app Design](Whole%20app.md) owns that rule and this is the place it would most easily be broken.

**The `.blend` icon is a plain cube, and it is generic on purpose.** Smoozy uses no Blender logo or mark of any kind — the Blender Foundation was asked and declined, which the [whole-app Design](Whole%20app.md) records. A cube says *3D file* to an artist without borrowing anything.

**The headers sort.** Free disk on the server sits along the bottom, where the spec puts it. **A sort reorders the rows Smoozy already holds and never reads the folder from the server again**, as *Show hidden files* does not (the [architecture](../Architecture/File%20Browser.md) L14 and L21): the list keeps its place in the folder, its selection and its count, and only the order changes. *(Added 2026-10-29, Creator-stated, the twenty-first notes, note 2: "Changing file ordering in the file browser seemingly reloads the folder in which the user is … simply rearranging stuff already loaded on screen shouldn't reload the folder altogether." Design conforming to L14's cache and L21's rule under Rule 10. No drawing state: nothing on the screen changes but the order. No auditor has seen it.)*

**The bottom bar carries two more things beside the free disk**: a standing line saying where an upload will land — *Uploads go to `kitchen/`* — and a count of what is in the folder. An upload lands where the user is standing, so that answer changes as they browse, and the drop zone and the moved-browser line each give it at the moment it is asked. *(Corrected 2026-10-06: this also named the upload confirmation, struck from* Uploading *on 2026-10-04.)* The standing line gives it on a quiet screen, before anything is asked.

**The bottom bar also carries *Show hidden files*, a checkbox, unticked until the user ticks it.** The [File Browser spec](../Specs/File%20Browser.md) settles what it does: names starting with a period leave the list, Smoozy's own staging folders with them, and nothing else changes. ~~**It sits beside the count**, because the count is the thing it changes,~~ **It sits at the bar's right end, so nothing beside it moves it as the figures change from folder to folder**, and not in the toolbar, which holds acts on files. **Ticked, it shows a tick mark on its box**, as every other checkbox in Smoozy does. *Changed 2026-10-03, the Creator's note 2 on the test copy: "Show hidden files should not jump around … A regular checkmark would be better." It sat beside the count, and the count and free disk change width per folder. Settled at Design by the build runner; no auditor has seen it.* **The count is of what the list shows**, and *1 selected* likewise: unticking the box takes a hidden name out of the selection. Label: **Creator-stated** (2026-10-01) for the checkbox and its default; its place was settled at Design on 2026-10-02.

**With the box ticked, a hidden row is drawn grey.** A row whose name starts with a period draws its icon, its name and its details in the quiet ink, so it reads as hidden at a glance, as a file manager draws it. Smoozy's staging folder is one of them and keeps its kind, *Smoozy staging folder*. **Nothing else about the row changes**: it is chosen, opened, renamed, moved and deleted as any other, and chosen it takes the ordinary highlight. A row that carries a refusal keeps the refusal's own look. *(Added 2026-10-29, Creator-stated, the twenty-first notes, note 3: "Hidden files should be grayed out, like they are in Finder, for example." A want touching only the look, so it enters at Design. Drawn as `fb-hidden`. No auditor has seen it.)*

**It is a checkbox because the Creator asked for one**, though it turns one thing on or off, which the [whole-app Design](Whole%20app.md) would make a switch. File managers draw this exact control as a checkbox, and so does the delete box's *Don't warn me* line, so it is the familiar form.

**While any transfer to or from this server is running or waiting, the bottom bar says how many — *3 transfers* — as a link to [Transfers](Transfers.md).** It counts every kind, as one number, and says nothing when there are none. No figure and no bar: Transfers carries those. An upload started here therefore says nothing above the list while it runs, and says there only how it ended. *Added 2026-10-02, the Creator's note 10 on 0.1.7: "some indication that a transfer is happening (at this very server of course) is definitely needed." Under Rule 10: the count is read from what Transfers lists for this server and never counted a second time.*

**The line above the list belongs to the server it was said on.** Going to another server clears it. An upload's or an act's ending that comes while the user is on another server is shown on that server's line the next time its Files is shown, once, and nowhere else. Only the newest is kept, and not past closing Smoozy; Transfers still lists the upload. *Added 2026-10-02: a hole a fault exposed. **Creator-stated**, notes 2 and 3 on 0.1.8: "Uploaded 1 file into /root. — this information is displayed on a server the file wasn't uploaded to", and "Uploading 1 file into /home/ubuntu. — … this message does not reflect reality." Nothing said how long the line lasts, and Smoozy said something false (Rule 8).*

**It also belongs to the folder and the act that made it.** Opening another folder clears it, and a new act's line replaces it. Reading the same folder again after an act keeps it. *Added 2026-10-03, the build runner's ruling (c), under the Creator's standing delegation of 2026-08-31: the Vast pass before 0.1.9 watched the line follow the user into other folders and stay up while a different act ran. No auditor has seen it.*

**A folder the login cannot read stays in the list.** Smoozy cannot tell before it is opened whether the login can read into it — the [architecture](../Architecture/File%20Browser.md) P2a rules out the per-entry probe and settles that a free one would not answer the question either — so nothing marks it in advance. Opening it is what produces the answer: the row says the login cannot read the folder, and where the machine allows one, the offer to try again with admin access is beside it. It is not silently absent: the spec's rule is to show what can be shown and say plainly what could not.

**The greying comes with the refusal rather than standing before it, and says the folder cannot be opened and nothing more.** It is not the whole-app grey for a reading taken from memory, and it is not the failure. **The note and the offer beside it are the failure, and they take the whole-app failure shape**: the red dot, one plain sentence saying the login cannot read the folder, and *Retry with admin access* as the act that would try again.

**The same offer follows a refused action, not only a folder that could not be read.** The [spec](../Specs/File%20Browser.md) gives both the same two responses, and this document had it on the listing alone. A delete, a rename, a move or a new folder the login is not permitted to make says so where it was attempted, with *Retry with admin access* beside it.

**Whether the offer appears at all is decided per machine and at the moment of the refusal.** The [architecture](../Architecture/File%20Browser.md) P5 settles five cases — the four in its table, and one more stated beside it — and three of them offer nothing. Passwordless elevation offers it and asks nothing; a systemd machine offers it and the password is asked at that moment. **A container under a login that is not its root offers nothing at all**, because the route needs a password Smoozy does not have and never will — there the refusal says the login cannot do it on this machine, and stops. **A refusal met by a login that is already root offers nothing either, and takes those same words**: there is nothing above root to try, so the sentence is true as it stands. Where root really is root the case does not arise, a permission refusal not being possible there; where it holds less than root, the refusal is what the user gets and the words fit it for the same reason as the container. **And a machine Smoozy cannot place offers nothing either**: the refusal is shown as it stands, with nothing added, because nothing is established about what admin access would do on that machine and the container's sentence would name a wrong reason. An offer that could not work is worse than none. **This paragraph counted four until 2026-09-02**, which is the table's own number and left out the case sitting beside the table.

~~**A refused write on an upload surfaces on the transfer row**, which is [Transfers](Transfers.md)'s surface, and its *Try again* carries the same offer where the machine allows one. That is the one place this answer lands on another screen, and it lands there because that is where the user is looking when it happens.~~ **Corrected 2026-09-27: a refused write on an upload surfaces on the transfer row, which is [Transfers](Transfers.md)'s surface, and carries no offer.** Out of step 10c's fix piece 7; no auditor of this document has seen it. The [architecture](../Architecture/File%20Browser.md) P6a has no elevated upload: the offer is attached to the acts P6 names — a listing and the machine's own remove, move, copy or make-directory — and never to a transfer, so a write refused on an upload is told as it stands. **So this answer never lands on another screen.** Design conforms to Architecture here under Rule 10.

## Acting on files

**Two groups, and the split is what they touch.** Upload and Download sit together, because both cross the wire and neither changes what is on the server. Everything else — new folder, rename, cut, copy, paste, delete — acts on the server's own files.

**The second group is itself divided three ways**: making and renaming, then the clipboard three, then delete on its own. That is what every file manager does, and it keeps delete — the one act the server cannot undo — from sitting hard against paste.

**There is no Duplicate, and it was taken out rather than never offered.** Copy and paste into the same folder is the same act in one more press, and at sixteen pixels a duplicate icon and a copy icon are the same drawing — so it cost a slot in the toolbar to offer a shortcut the user could not tell from the thing it shortened. Label: **Creator-stated** (2026-08-13).

**Cut, copy and paste rather than *move to* and *copy to*.** A pasteboard holds until something replaces it, so one copy can be pasted into several folders in turn. A cut is let go of once it is pasted. Paste carries a count of what is on it. Label: **Creator-stated** (2026-08-10).

**How many files are chosen is said at the far end of the toolbar** — *1 selected*. What is on the pasteboard is not said again there: the Paste control's own count is its one home, and the two sit a few controls apart in the same row.

**Rows are chosen the way a file manager chooses them.** A click chooses one row alone; Shift-click chooses the range from the last row clicked, in the order the list draws them; Cmd-click (Ctrl-click on Windows and Linux) adds a row or takes it away. Shift with ↑ or ↓ widens the choice by one row. The toolbar says how many: *3 selected*. **Delete, Download, Cut and Copy act on every chosen row**, and the delete box names how many. **Rename acts on one row and is greyed while several are chosen; Render settings… and Check dependencies… are offered only while exactly one `.blend` is chosen.** A right-click on a chosen row acts on all of them; on a row outside the choice it chooses that row alone first. A double-click chooses its row alone and opens it; Return does nothing while several are chosen. *Added 2026-10-18, Creator-stated, the thirteenth notes, note 6: "Why can't I ctrl/cmd+click to select multiple files in the file browser? Same for shift-clicking." The same choosing as the dependency check's rows. Under Rule 10, `File Browser` L22 and P9 already declare acts on several files. No auditor has seen it.*

**Paste has to remember which it was.** The [File Browser spec](../Specs/File%20Browser.md) gives a colliding move *Replace* or *Cancel*, and a colliding copy *Replace*, *Keep both* or *Cancel* — so a paste after a cut and a paste after a copy do not offer the same answers.

**Delete is the one in red**, because it is the one the server cannot undo.

**What the toolbar offers changes with what is chosen, and only a `.blend` changes it.** ~~Choosing one puts **Render settings…** and **Check dependencies…** at the head of the toolbar, before Upload and Download.~~ Choosing one puts **Render settings…** and **Check dependencies…** at the end of the toolbar, after Delete, so the file manager's own controls never move when a `.blend` is chosen. *Changed 2026-10-08, Creator-stated, the fifth notes, note 1: "so that the latter don't move."* *(2026-10-18: several rows chosen also change it — Rename greys, and the two `.blend` actions show only for exactly one `.blend`; see* Rows are chosen the way a file manager chooses them*.)* Choosing anything else takes them away rather than greying them: a control that cannot apply to what is selected is not a control, it is a question. Label: **Creator-stated** (2026-08-13).

**One click chooses any row — a folder, a file or a `.blend` — so the toolbar, the keys and right-click act on it; a double-click opens it**: a folder opens, and a `.blend` opens its settings, as *Render settings…* does, and a picture opens over the window *(added 2026-10-31: see* Looking at a picture*, below)*. **A click on the list's empty space lets the choice go.** Both Creator-stated 2026-10-06, the third notes on the test copy: *"Clicking a blend file should not automatically open render settings"* (note 7) and *"clicking in empty space should deselect everything"* (note 6). *Changed 2026-10-12, Creator-stated, the Creator's eighth notes (2026-10-11), note 2: "let's do opening stuff double-click for everything. Single-click should just select an item in the file browser." The sentence began* One click chooses a `.blend` like any other file*, and one click on a folder opened it.* The path along the top and *Up* are not rows and stay one click.

~~**They go first because that is what the browser was opened for.** Every other control there belongs to a file manager; these two are why Smoozy has a file browser at all, and putting them after Paste would bury the only two actions that lead anywhere.~~ **They go last so nothing else moves when they appear**, and being the toolbar's only words they are still found at a glance. *Changed 2026-10-08 with line 73, the fifth notes, note 1.*

**Right-click gives what acts on the thing clicked.** *(2026-10-18: on a row inside a choice of several, what is chosen is the thing clicked, and the menu acts on all of it; Rename is greyed there.)* Three menus: a folder, an ordinary file, and a `.blend`. All three carry download, cut, copy, rename and delete. **Only the folder menu carries paste**, a folder being the one thing clicked that a paste can go into. **Upload and new folder are on no menu at all**, because both act on the folder being browsed rather than on the thing clicked, and a menu item about somewhere else is not what a right-click is for. The `.blend` menu puts the same two at the top, these two being why Smoozy has a file browser at all. *(Reason restated 2026-10-08, when the toolbar's own reason changed; the menu is unchanged.)*

**The folder menu carries *Calculate size*, in a group of its own right under *Download* and *Download to…*.** Finder's own words; it is not a transfer, so it does not join their group. On a choice of several, it counts each chosen folder, each row filling in on its own; chosen files keep the size they already show. On a folder showing *Calculating…* the item is greyed. On a folder already counted, it counts again. Like every act that needs the server, it is greyed while the listing is stale. **Where the server cannot count** (the architecture's L28), the item is greyed and says *Set up Smoozy on this server to calculate folder sizes.* on hover, or *Update* in place of *Set up* where Smoozy there is older than the app — the words Copy uses. Added 2026-10-30, Creator-stated, the twenty-third notes, note 2. No auditor has seen it.

**Two items came off the folder menu.** *Open* went because a double-click already opens it *(changed 2026-10-12 with line 75, the Creator's eighth notes, note 2; it read* clicking the folder already opens it*)*, and a menu item for the thing a click does is a menu item nobody reads. *New folder inside* went because the toolbar already makes a folder wherever the user is standing, and two ways to make a folder is one more than the question deserves. Label: **Creator-stated** (2026-08-13).

**What the screen says around an act.** Added 2026-10-02, out of step 10c's map of Files' owed acts, ruled by the build runner; no auditor of this document has seen it. None of it meets an [architecture](../Architecture/File%20Browser.md) clause beyond the ones it names.

- **A new folder's row starts named *New folder*, selected, so typing replaces it.** Where the list already shows that name, the row says so as the user types, the same hint a rename gets.
- **A row being typed into keeps the row's height.** Its field takes the list's own font and size, sits exactly where the name sits and as wide as the name column, and shows it has the focus by a one-pixel accent border, never the system's glow. The same holds for a rename. *Added 2026-10-03, the Creator's note 4 on the test copy ("I think there's something very wrong with this design"); the plain typing row had never been drawn. No auditor has seen it.*
- **A press anywhere outside the row being typed puts it back as it was, as Escape does.** *Added 2026-10-06, Creator-stated, the third notes on the test copy, note 8: "Clicking in empty space … should discard the attempted renaming."*
- **A control pressed with nothing chosen says so in one line**: *Select a file or folder to rename.*, and the same for cut, copy and delete. The delete key with nothing chosen does nothing.
- **A long act is told on the line above the list**, where the moved-browser line sits: what is under way and how far it has got — file by file for many files, nothing in between for one large copy (the architecture's M7) — and then how it ended: what was done, what was refused, what was not attempted. The list is read again at the end.
- **A name the row refuses before trying** (the architecture's M9: empty, a `/`, `.` or `..`, too long, or Smoozy's staging folder's name) keeps the row in editing, red, with one plain line under it, as a taken name does.
- **A rename to a name starting with a period, with *Show hidden files* off,** takes the row out of the list, so the line above the list says it was renamed and is now hidden.
- **A paste into itself.** A folder pasted into itself, or into a folder inside it, is refused in one line before anything is sent. A cut pasted back into the folder it came from does nothing, and the line says *Already in this folder.* A copy pasted into its own folder asks the ordinary question, and *Keep both* is the duplicate.
- **Where the server cannot copy files on itself** (the architecture's M7), Copy says *Set up Smoozy on this server to copy files.*, and a move between disks says *Set up Smoozy on this server to move files between disks.* Where Smoozy there is older than the app, *Update* replaces *Set up*.

## Uploading

**One Upload control, not two.** Files and folders go up together where the user picks them together.

**Dragging onto the list uploads into the folder being browsed.** The list takes a dashed border and says where the files will land and how many there are. Drag-and-drop is not in the [File Browser spec](../Specs/File%20Browser.md), and does not need to be — the spec leaves Design *how uploading is started*, and this is a how.

**Where the system file picker cannot take files and folders in one go, the same Upload control asks which kind first.** That is one control either way: the fallback is a question inside Upload, never a second control beside it. **On Windows the picker cannot** — its dialog offers a choice of folders rather than files, and never both at once. **On Linux the shared picker the desktop itself provides cannot either**, and which of the two a Linux Smoozy actually meets — that shared one, or a dialog the interface toolkit draws on its own — is not settled, so Upload asks the question there too. **On macOS it looks as though it can**, but that rests on two documented settings that no page names together, so it is checked once at Build rather than assumed. Label: **settled at Architecture** (2026-08-23). **It was checked, and it can** ([architecture](../Architecture/File%20Browser.md) H3, observed 2026-09-04): **on a Mac, Upload opens one panel that takes files and folders together, and asks nothing first.** On Windows and Linux the question stays, worded for that computer. *Added 2026-10-04, the Creator's note 2 on the test copy: on their Mac the question said the picker could not take both, which is not true there.*

**What a folder upload will do is shown before it goes, and it is a count rather than a tree.** The [architecture](../Architecture/File%20Browser.md) H3a settles what Smoozy actually knows at the moment of dropping: paths and nothing else — no sizes, no counts, no walk of what is inside. So a tree drawn at that moment would be drawn from nothing.

~~**What is shown is what was picked and where it lands**, in the confirmation before the upload starts: the folders by name, the loose files as a count, and the destination. *`textures/` and `sim/`, and 4 files, into `kitchen/`.*~~ **STRUCK 2026-10-04: there is no confirmation. A pick in the system panel, or a drop on the list, starts the upload.** Choosing in the panel is already the decision, and a drag already shows where the files land while it is held. The already-exists question after the walk is unchanged, and Transfers lists the upload from its first moment. Label: **Creator-stated** (2026-10-04), the Creator's note 2 on the test copy: *"There's still a message before an upload starts, do we really need that?"*

**The full count arrives after the walk, not before it**, because the walk runs once the upload has started *(read "once the user has confirmed" until the confirmation was struck, 2026-10-04)*. That is where the already-exists question already lives, and it is where the real number appears. Nothing here promises a figure earlier than Smoozy has one. **Nothing explains any of this** *(read "The confirmation does not explain any of this: it names what goes and where, and nothing about how" until the confirmation was struck, 2026-10-04)*. *Creator-stated 2026-10-02, note 9 on 0.1.7: "Uploading is just uploading, there's no need to explain how exactly it happens."*

**The structure is kept as it was picked**, which the [spec](../Specs/File%20Browser.md) settles: a folder goes up as a folder. So the user's own arrangement is what lands *(this sentence also said the confirmation names the folders rather than their contents, until the confirmation was struck, 2026-10-04)*.

## When the files are already there

**Asked once for the whole upload, leading with how many.** "42 of these 300 files are already in `kitchen/`" is a different decision from one file, and the [File Browser spec](../Specs/File%20Browser.md) requires the number. The colliding names are listed beneath, scrollable, so the user can see which before choosing.

**One line says what the comparison was**, and it stops there: compared by name and location only. Saying afterwards that Smoozy has not compared their contents was the same fact told twice.

**Four answers**: Replace, Keep both, Skip existing, Stop.

**One line above them is a separate warning**, and it is the one that matters: where some of the colliding files were placed by the dependency check, replacing them puts back files pointing at the user's own computer and the render will not find them. It offers to send those through that check instead.

**The live-job warning sits above all four answers, not beside one of them.** The [spec](../Specs/File%20Browser.md) puts it ahead of the answers, and the reason is what it is: it is about the machine's own work, and every one of the four answers can disturb it. Under an answer it would read as a property of that answer.

**Skip existing is put forward, but only where Smoozy knows why.** The [architecture](../Architecture/File%20Browser.md) K1 and K2 have an interrupted upload carry on by itself across a repair, so the ordinary interrupted case never comes back to this box at all. What does come back is an upload the user starts again in a **fresh sitting**, which K3 forbids resuming on its own — and there Smoozy holds the earlier request's record and can say so.

**So the box says why the answer is put forward, in one line.** *Upload interrupted — last file uploaded on 12 Aug. Skip existing uploads only what is missing.* Without the line, a pre-picked answer is Smoozy deciding for the user with no reason given, and the one thing that makes it right is a fact only Smoozy holds. **The moment named is when a file last finished uploading, not when the upload stopped** — nothing is written down at the moment an upload stops, so that moment is the only one Smoozy honestly holds. The two are the same day except where an upload stops after midnight.

**Where there is no such record, nothing is put forward** and the four answers sit level. Smoozy does not guess that a collision means an interrupted upload.

## Renaming into a name that is taken

**It is refused in place, on the row, and the refusal comes from the machine rather than from the list.** The [architecture](../Architecture/File%20Browser.md) M4 is firm about which reading answers: the listing is a cache and may be minutes old, so a refusal composed from it would say a name is taken when the machine holds nothing under it. The rename is attempted and the filesystem's own answer is what reports the collision.

**The row stays in editing with the typed name still in it**, red, with one line under it: *Name already in use.* Nothing is replaced and nothing is lost — the user picks another name and presses again.

**A warning may come earlier, from the list, and it is a warning and not a refusal.** Where the cached listing already shows the name, the row can say so as the user types. That is a hint about what Smoozy last saw; the refusal above is the machine's answer, and only the second one stops anything.

## The browser opened so the user can point at a file

**It is the ordinary browser with nothing withheld, and the [architecture](../Architecture/File%20Browser.md) §12 is explicit about that.** Two callers open it — a job whose `.blend` has gone from a machine still connected, and a re-submission whose own machine is gone. Either way the user points at a file and the file goes back to whoever asked.

**What marks it is a bar along the top, not a different screen.** It says who is asking and for what — *Pick the `.blend` for Kitchen — final* — with a way out beside it. Everything below the bar is the browser exactly as it always is: the same toolbar, the same menus, uploading, deleting, all of it.

**Moving to another server from inside it is allowed and is not an accident.** The [architecture](../Architecture/File%20Browser.md) §12 confirms the ordinary between-machines move stays available, and the pick carries the server it was made on. So a user who opened it for one machine and picked on another gets what they picked, and nothing assumes otherwise.

**Picking a `.blend` gives it to whoever asked, rather than opening [Render Settings](Render%20Settings.md).** That is the whole difference from an ordinary double-click and the one thing the bar has to make true: the caller asked for a file, and the file goes to the caller.

**It ends three ways and all three are on the bar.** The user picks a file; the user presses the way out, and the caller is told nothing was picked; or the user leaves the browser for another screen, which is the same as pressing the way out. Nothing is left half-open behind them.

## When another module moves the browser to a different server

**One machine is browsed at a time, so *show outputs* on a job that ran elsewhere moves the browser.** The [spec](../Specs/File%20Browser.md) asks for that move to be obvious, and gives the reason: an upload lands where the user is browsing, so the move changes where the next file they send would go.

**The badge and the name change, and a line says why.** The server's colour is already the app's answer to *which machine*, and it changes as the browser moves. Above the list, one line: *Moved to Hetzner box to show this job's outputs.* It clears when the user next changes folder by hand.

**Nothing already going is disturbed**, and the line does not claim otherwise. The [spec](../Specs/File%20Browser.md) settles that an upload under way keeps going to the machine it was sent to; only the next one lands here. The line says the second half only where an upload is actually in flight, because on a quiet browser it would be answering a question nobody asked.

**A machine that is not connected is not browsed and the action is not offered.** Smoozy never opens a connection to do this, which is the spec's own rule.

**Where *show outputs* lands on a job's folder that is not there and the job has saved no frames, Files says so in place of the list:** *Does not exist yet: /…/beauty. No frames saved.* Anywhere else a missing folder reads as `File Browser` P3 has it. *Added 2026-10-07, Creator-stated, the fourth notes, note 3: "It has to tell the user that this folder doesn't exist yet because no files have been saved so far." Rule 10: the words still name the missing part of the path and open no other folder. No auditor has seen it.*


## The live-job warning, on all four acts

**The [spec](../Specs/File%20Browser.md) fires it before a delete, a move, a rename and an overwrite, and this document had settled only the delete.** The other three take the same box and the same words. Nothing about it is per act, because nothing about it can be.

**It is uniform across files and different across jobs**, which is the [architecture](../Architecture/File%20Browser.md) W5a's bound stated exactly. A job that is rendering and a job that is waiting are told apart, because Smoozy knows which is which. What a touched file does to a running render is **not** modelled — the research reaches a scene's material picture and a linked `.blend` and nothing else — so the warning covers every touch the same way and promises nothing about what the touch does.

**So the sentence is a *may* and never a description.** *A running render may finish with wrong frames.* **This document said *with the wrong picture in it* until 2026-08-23**, which names one of several behaviours as the consequence — a per-file claim the [architecture](../Architecture/File%20Browser.md) W5a forbids. Corrected under [Rule 10](../Groundwork/Rules.md).

**It names every job that holds the file**, and where Smoozy's set for a job is only the `.blend` and its folder it says so rather than implying it checked more. That is the [spec](../Specs/File%20Browser.md)'s own smaller-set clause arriving at its widest.

**It cannot be switched off, on any of the four.** Creator-stated, and there is no suppression row for it.

## Deleting

**Two warnings of different weight, in one box, and the difference is the design problem.**

The **live-job warning** comes first, in red, naming the jobs and telling the truth about each — a running render may finish with wrong frames. It is the same warning the three other acts carry and is set out above. It cannot be switched off.

The **permanence warning** follows, quieter: *This cannot be undone.* It can be switched off.

**So the checkbox sits under the permanence warning and says in one line that it does not cover the other.** Without that, switching off "warn me about deleting" would read as switching off both.

## Disturbing a file nothing live is using

**There is a second warning with no live job behind it, and it is quieter for a reason.** The [architecture](../Architecture/File%20Browser.md) W1 fires it from the local record rather than from any job: deleting something the dependency check found other references pointing at, or a fixed `.blend` that would be left pointing at nothing. No render is at risk this second; what is at risk is the next one.

**It is one line in the same box, under the live-job warning where both are there.** Where there is no live job it is the whole box. It names what depends on the file and says the check would have to run again.

**It is [Dependency Upload & Remap](Dependency%20Upload%20&%20Remap.md)'s trigger and this screen's wording**, which is why it says what the file is *for* rather than what the record is: *3 other references point at this file* reads to an artist; a record naming a machine location does not.

**And it never claims the file is in use.** Nothing here reads the machine — the [architecture](../Architecture/File%20Browser.md) W1 answers it from the local record before any write — so a record outliving a file the user removed with their own tools produces no line at all, and a line that does appear is about what points at the file rather than about what is happening to it.

## Looking at a picture

*Added 2026-10-31, the Creator's seventeenth notes, note 11: "Can we include an image viewer in the file browser? Just a viewer, nothing else. Because I feel like simply double-clicking an image and instantly seeing a preview is something people would surely expect." Asked whether Files should have an image viewer, they said "Yes to both your questions" (2026-10-30). PNG and JPEG only and the 100 MB cap, ruled by the build runner 2026-10-31. The read is the [architecture](../Architecture/File%20Browser.md)'s L29. No auditor has seen it.*

**A double-click or Return on a picture opens it over the whole window**, the way Queue's large preview opens: centred, as large as the window allows with a 24-pixel margin on every side, the picture fitted whole. A picture is a row with the picture icon. No menu item: a double-click already opens it, the reason *Open* came off the folder menu.

**The box's top line names the file, then its size and pixel size, then *Download* and the close button**: *kitchen_0002.png* · *3.9 MB · 1920 × 1080*. *Download* brings the file home through Download, as it does from the list. No *Not colour-accurate*: this is the file itself, not a small copy.

**‹ and › under the picture, and ← and → on the keyboard, step to the previous and next picture in the folder**, in the list's order, so a run of frames is looked through in order. Rows that are not pictures are skipped; pictures Smoozy cannot show are not. At either end the arrow greys. The row chosen in the list follows, so closing leaves the last picture chosen.

**Escape, the close button or a click outside closes it.**

**While the picture is read, the box says *Loading…* where it goes.** Stepping on stops a read the user has left.

**Only PNG and JPEG are shown.** Any other picture opens the same box with one line where the picture goes, naming its format: *Can't preview OpenEXR files.* — or TIFF, Targa, HDR or WebP. The top line gives its size alone. *Download* is right there. *Words changed 2026-11-01 (was *Smoozy can't show OpenEXR files.*): Creator-stated, the second words sweep. No auditor has seen it.*

**A file over 100 MB is not read**: *Too large to show (over 100 MB).* in the same place. *Words changed 2026-11-01 (was *Too large to show. Smoozy shows pictures up to 100 MB.*): Creator-stated, the second words sweep. No auditor has seen it.*

**A PNG or JPEG that does not open reads *Could not show this file.*; a read that fails reads *Could not read the file.*, with the reason under it and *Retry*.** Nothing retries by itself.

**While the connection is down the rows stop responding**, so no viewer opens. One already open keeps its picture; stepping shows *Not connected.* where the picture goes, and *Download* greys.

**A video opens nothing**, as before.

## Keyboard shortcuts

**Return opens what is chosen. Backspace goes up a folder. The delete key deletes, asking exactly as the control does. F2 renames.** The clipboard three are the menu bar's own set acting on what has focus, which is this screen's files when the list has it.

**↑ and ↓ move the chosen row.** *(Corrected 2026-10-06: this said Shift with either widens the choice, the way a click and a Shift-click already choose. Files chooses one row and has no Shift-click, so there was nothing to widen; Shift waits for choosing several rows, which is parked in Open Threads.)* **Shift with ↑ or ↓ widens the choice by one row**, the way a Shift-click chooses a range. *(Restored 2026-10-18 with choosing several rows, the thirteenth notes, note 6.)* **Return does what a double-click does: it opens a chosen folder, a chosen `.blend`'s settings or a chosen picture, and does nothing on any other file.** *(The picture added 2026-10-31.)* *(2026-10-18: and nothing while several rows are chosen.)* *(Changed 2026-10-12 with line 75, the Creator's eighth notes, note 2, so the keyboard and the mouse keep one rule; it read* Return opens a chosen folder and does nothing on a file, since Files has no act that opens a file.*)* **On a Mac the keys are the Mac's own**: the key a Mac keyboard labels *delete* deletes, asking exactly as the control does, and **⌘↑ goes up a folder**, as the Mac's own file manager does — on a Mac that key reports itself as Backspace, so the line above sent it up a folder and left nothing on most Mac keyboards that deletes. **Elsewhere the keys are as written above.** No key acts under a box, in a field being typed in, or while a panel is open. *Added 2026-10-04, the Creator's note 4 on the test copy: "Hotkeys and arrow navigation in the file browser — for some reason this has been completely forgotten." Arrows were in no plan; Return was drawn and never built; the Mac's delete key was a fault. Settled at Design by the build runner; no auditor has seen it.*

**Nothing that crosses the wire has a key.** Upload, download and the two `.blend` actions are all pressed deliberately, and a key pressed by accident that starts a 4 GB upload is the one mistake this screen must not make. *(Corrected 2026-10-12 by the build runner, found by the builder of the eighth notes' piece E2: since Return does what a double-click does, Return opens a chosen `.blend`'s settings, which asks the server for its Blender list. That is a reading, not an act a mistake can make costly, so it is the one exception. Upload, download and* Check dependencies… *still have no key. No auditor has seen it.)* *(Added 2026-10-31: Return on a picture reads it to show it — a reading too, capped at 100 MB, and nothing lands — so it is the second exception. No auditor has seen it.)* The whole-app rules are in [Whole app](Whole%20app.md).

## While the connection is down

**The listing stays and says it is offline**, with when it was last updated. Throwing away what the user was reading helps nobody, and they may still want a name off it.

**Everything needing the server goes unavailable rather than failing on a click** — the controls dim, the rows stop responding, and the free-space figure says it is what was last seen. That matches what the server itself reads as, so the two parts of the app do not disagree.

***Show hidden files* still works**, because it only changes what is shown from what Smoozy already has. The [architecture](../Architecture/File%20Browser.md) L21 settles that it never reads the server.

## What this document left open, and how it was answered

**Nothing, and nothing was ever parked here.** Every marker this document carried was closed in the body where it stood, so this section has been an empty heading rather than a list. It is kept so the shape matches the other screen documents, and it says so rather than sitting blank.

