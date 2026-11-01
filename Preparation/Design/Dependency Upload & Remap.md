# Dependency Upload & Remap

*What the dependency check looks like, what each row carries, and what the user can do to one. The conventions it follows are the [whole-app Design](Whole%20app.md)'s and are not restated here.*

## Where it sits

**A panel over the server**, the twin of [Render Settings](Render%20Settings.md), collapsing to a strip that keeps counting.

**While the check's uploads are running or waiting, the strip reads *Uploading 4 of 12* in place of its figures**: the files of this check uploaded so far, of all it is sending, read from what [Transfers](Transfers.md) lists and never counted a second time. *Added 2026-10-02. **Creator-stated**, note 6 on 0.1.8: "When dependencies for a given job are being uploaded, there should be some indication of that where the user would start the render job." [Render Settings](Render%20Settings.md) carries the same count by Render.*

**Otherwise the strip shows the panel's summary sentence** ([Above the rows](#above-the-rows)) — *Nothing is missing. 2 files to upload, 90 MB.*, or *2 failed · 3 missing · …* — so a collapsed check still gives its answer. Its *figures* above are that sentence. *Changed 2026-10-18, the dependency check's fresh look, approved by the Creator on 2026-10-18 with both its decisions answered yes (`Audits/Build/Step 10c/The dependency check, a fresh look — the proposal.md`, from the Creator's twelfth notes, note 5). Until then the strip read* 6 of 12 found · 2.2 GB to upload · 3 not found. *No auditor has seen it.*

**Collapsing is not a convenience here, it is required.** The check reaches down another level every time a missing linked `.blend` arrives, a search over a large folder runs for minutes, and its uploads run in [Transfers](Transfers.md). A panel that had to stay open would stop the user working.

**Connections.** It serves the [Dependency Upload & Remap spec](../Specs/Dependency%20Upload%20&%20Remap.md). It is reached from [Render Settings](Render%20Settings.md) and from a `.blend` in [File Browser](File%20Browser.md). Its uploads appear in [Transfers](Transfers.md).

## Above the rows

**The first line under the title is the summary sentence, and it is the answer to the whole check.** Where nothing is open it says so, then what will upload: *Nothing is missing. 2 files to upload, 90 MB.* Where something is open it names that first, worst first, in the order of the groups below, and leaves out every zero: *2 failed · 3 missing · 1 not verified · 344 files to upload, 2.2 GB · 128 MB already on the server.* While the check is still reading, it ends *, found so far*. **It never says *Nothing is missing* while any row is failed, missing or not verified, or while the check is still reading.** The size already on the server stays in it, so a finished check still says what is there: *Nothing is missing. 90 MB already on the server.* Ignored rows are not in it, since ignoring a row is the user's answer. Its figures keep the colours the foot's had: failed and missing red, not verified amber, already on the server green. Collapsed, the strip shows this same sentence. *Added 2026-10-18, the dependency check's fresh look, approved by the Creator on 2026-10-18 with both its decisions answered yes (`Audits/Build/Step 10c/The dependency check, a fresh look — the proposal.md`, from the Creator's twelfth notes, note 5); the figures were along the bottom until then. No auditor has seen it.*

**The summary has room around it.** A clear gap under the title's divider, a wider gap between the summary and the line under it (*Check again*, or the first statement), and a clear gap above the search line, so the answer reads as the head of the panel rather than squeezed between two borders. The statements under the summary keep their own close spacing, and the rows and groups below keep theirs. *Added 2026-10-28, Creator-stated, the twentieth notes, note 6: "It's a little too tight there." The three gaps judged together in the drawing, runner-ruled; drawn as `chk-onserver` (the Creator's screenshot: one file, nothing missing, nothing searched) and checked on `chk-folders`. No auditor has seen it.*

**The search says what it is doing and can be stopped** — how many of the user's folders are done, with a Stop beside it. A search folder Smoozy could not search is named rather than passed over silently, in [Settings](Settings.md)' own words for that state rather than in fresh ones: that list row owns the folder's state.

**Where Smoozy has no record of where the file came from on this computer, the check says so at the top, once:** *Original location unknown. Searched in your search folders; matches apply to this server only.* The rows then read as they always do. *Added 2026-10-01, from step 10c's twelfth pass, where a file uploaded before its server was set up checked as 58 rows of Not found with no reason given. Design conforming to Architecture under Rule 10: `Dependency Upload & Remap` K5 asks for this limit to be said "in as many words". Corrected the same day: it said relative paths are looked for only on the server, and the check does not search the server for one. A relative path in such a file has no folder on this computer to be measured from (K5, F27), so the search of the user's search folders is what finds it, by name (F3), and K5 does not take it out of that search.* *Words changed 2026-11-01 (was *Original location unknown: relative paths are searched for in your search folders, and matches are kept for this server only.*): Creator-stated, the second words sweep. No auditor has seen it.*

**Search folders is reachable from here as well as from Settings**, because this is where the user finds out they need one.

**The check is called *Check dependencies*, not *Check for missing files*.** A dependency is what the industry calls it and what the [spec](../Specs/Dependency%20Upload%20&%20Remap.md) and the [Glossary](../Groundwork/Glossary.md) call it; *missing files* described the outcome the user hopes not to have rather than the thing being checked. Label: **Creator-stated** (2026-08-13).

**Where a shift was worked out, it is stated once at the top** — the old root, the new root, how many rows it resolved, and one action that undoes the whole thing. And **every row it resolved says so**, so undoing it is a decision the user can see the shape of.

## A row

**A row names the file once.** In order:

- **The name, then its kind** — *studio_white_soft_light_02.exr* · *Image*, *rig.blend* · *Linked .blend*, *NeueHaas.otf* · *Font*, *smoke* · *Cache*. A row that stands for a set says so and how many files: *Sequence · 240 files*, *Cache · 340 files*.
- **One location: the folder on this computer it will upload from.** Where nothing was found, and where a file uploaded before has no copy left on this computer, it reads *The file says:* and the folder as the `.blend` writes it. **Hovering the location shows the file's own text for it**, exactly as the `.blend` writes it — *//../Documents/Blender/HDRi/studio_white_soft_light_02.exr*. So a file found where the `.blend` says shows that place once, written out in full.
- **Notes, only where there is something to say** — one line each, wrapping, every one that applies, in this order:
  1. a change since the check ran, in the new state's colour: *File no longer found*, *Ready to upload*, *Ignored*;
  2. a failure's own reason, in red: *Upload failed: permission denied*, *Uploaded; remap failed*;
  3. how it was found, where that is not where the file says: *Found in search folder HDRI. The file says: //../old/hdri/* · *Found by your path change from /work to /Archive* · *You chose this file* · *Only the case differs: metallic.png* · *3 matches in Archive; arbitrary pick* · *Also in Textures, later in your search folders* · *You rejected wall_plaster.png*. The last two of those stay apart on purpose: one is the user's folder order deciding, the other is Smoozy picking among equals;
  4. what the row stands for: *Referenced 4 times* ([below](#when-more-than-one-reference-points-at-one-thing)), and on a missing linked `.blend`, *Uploading it may add more rows*;
  5. *Not used by any scene*, only where Smoozy's reading finished for every reference the row stands for and reached no scene — the [spec](../Specs/Dependency%20Upload%20&%20Remap.md)'s one exception to saying nothing about scenes. It acts on nothing. **Never on a row the reading's walk cannot reach a scene from at all** — a linked `.blend`, a shader node's script (an OSL script, an IES profile), or something the scene itself owns (its bake folder, rigid-body cache, sequencer strips and proxies) — since there *no scene reached* would say nothing true. *Added 2026-10-18, the build runner's ruling on the rebuild's ask; no auditor has seen it.*
- **The size.**
- **At most one visible action, and only where the state needs one**: *Locate…* on a missing row, *Accept* where only the capitals differ, *Use it* where the user rejected the match, *Verify*, *Retry*, *Restore*. A row Smoozy will upload, or has on the server already, shows none.
- **⋯**, holding the rest: *Locate…* (point the row at a different file), *Ignore*, *Restore*, and *Reject match* where Smoozy guessed.

**There are no column headings and no checkbox.** Rows are chosen the way a file manager chooses them ([Acting on many rows](#acting-on-many-rows)), so nothing on a row reads as *tick to upload*.

*Rewritten 2026-10-18, the dependency check's fresh look, approved by the Creator on 2026-10-18 with both its decisions answered yes (`Audits/Build/Step 10c/The dependency check, a fresh look — the proposal.md`, from the Creator's twelfth notes, note 5). The row named the location twice whenever the file was found where the `.blend` says, which is the usual case, and nothing said what a row was. Until then this section read:* ~~Three columns — **what the file records**, **what Smoozy found** and **the size** — and the actions. There is no *State* column: the group heading says the state.~~ ~~**What the file records** carries the location as written inside the `.blend`, and nothing beneath it.~~ ~~**What Smoozy found** carries the location and, beneath it, whatever qualifies it: that a match differs only in capitals, that a name was found in more than one place and this pick was arbitrary, that a row stands for a whole folder sent as one, or that a missing linked `.blend` may add more rows once it is placed.~~ *No auditor has seen it.*

*(Until 2026-10-08 the row also listed, under the location, every scene using it, or* scene not known*. The Creator dropped that line as noise (sixth notes, note 3), and the [spec](../Specs/Dependency%20Upload%20&%20Remap.md) changed with it. The one scene note above came back on 2026-10-18.)*

**A row that is itself another `.blend` is coloured** — the linked-library blue from the [whole-app Design](Whole%20app.md) — because those are the rows hiding an unknown amount beneath them. *(From 2026-10-18 the colour is on its name.)*

## The seven states

Seven, and the [spec](../Specs/Dependency%20Upload%20&%20Remap.md) splits them where this table does: five say what the check found, two say what happened after the sending started. Each row carries the action that fits it.

**The seven take the reserved colours, not seven of their own.** [Design values](Design%20values.md) owns what a colour means and a screen may not use one to mean something new, so where no reserved colour says what a state is, the heading's word says it and the group takes none.

**The screen has its own names for them, and the spec keeps its own** — the same way the spec says *processor* where a screen says *CPU* (Rule 9). The two failures share one heading, *Failed*, and each row keeps its own reason as a note.

| On screen | The spec's state | Colour | Visible action on the row |
| --- | --- | --- | --- |
| Failed | Could not be sent | `--bad` | Retry |
| Failed | Could not be fixed | `--bad` | Retry |
| Missing | Not found | `--bad`, something the user must act on | Locate… — *Accept* where only the capitals differ, *Use it* where the user rejected the match |
| Uploaded before, not verified | Sent before, not confirmed | `--busy`, its dot `--busy-fill`, something the user should look at | Verify |
| To upload | Found on your computer | none — the word alone | — |
| Already on the server | Already on the machine | `--ok`, nothing left to do | — |
| Ignored | Set aside | `--idle`, nothing happening | Restore |

**A row whose upload is already on its way says *Uploading* among its notes**, in `--busy`, work in progress, from the moment its send is running or waiting in [Transfers](Transfers.md) until the send ends, when the row takes the state its ending gives — *Already on the server*, or *Failed* with its reason. It stays under *To upload*, because nothing has landed yet, and it carries no action. The panel reads this from what Transfers is sending, file by file, every time the panel is shown — so coming back to the check from Transfers shows which files are already going. *Added 2026-10-24, Creator-stated, the eighteenth notes, note 6: "When I click upload in dependency checker and then come back to it from transfers it's still telling me to upload what is being uploaded at that moment. It has to know what is already en route via transfers, to avoid accidental duplicate uploads." The build runner's ruling under the Creator's standing delegation; the drawing's state is `chk-sending`. No auditor has seen it.*

*Changed 2026-10-18, the dependency check's fresh look, approved by the Creator on 2026-10-18 with both its decisions answered yes (`Audits/Build/Step 10c/The dependency check, a fresh look — the proposal.md`, from the Creator's twelfth notes, note 5): *Verify* replaced *Confirm*, which read as accepting a claim; *Ignore* replaced *Set aside*. The table until then, kept for its words:*

| State | Colour | Action on the row |
| --- | --- | --- |
| ~~Found on your computer~~ | ~~none — the word alone~~ | ~~Locate…~~ |
| ~~Already on the server~~ | ~~`--ok`, nothing left to do~~ | ~~—~~ |
| ~~Uploaded before, not confirmed~~ | ~~`--busy`, its dot `--busy-fill`, something the user should look at~~ | ~~Confirm~~ |
| ~~Not found~~ | ~~`--bad`, something the user must act on~~ | ~~Locate…~~ |
| ~~Set aside~~ | ~~`--idle`, nothing happening~~ | ~~Restore~~ |
| ~~Not uploaded~~ | ~~`--bad`~~ | ~~Try again~~ |
| ~~Could not be fixed~~ | ~~`--bad`~~ | ~~Try again~~ |

*No auditor has seen it.*

**The last two are failures of the upload rather than of the search**, so they take `--bad` and the whole-app failure shape.

**There is no State column; the group heading says the state**, with its dot, its colour and how many rows — ~~*● Not found 3*~~ *● Missing 3*. A row whose state changed after the check ran stays in its group and says its new state ~~on a small line under *What Smoozy found*~~ as its first note, in that state's colour. **The colour says what the user has to do**: red where they must act, amber where they should look, green where nothing is left, and no colour where Smoozy will upload the file itself. It sits on the group heading, its dot and ~~the word under *What Smoozy found*~~ the row's notes; ~~at the foot, *not found* and *failed* are red above zero and *already on the server* is green~~ in the summary sentence, *failed* and *missing* are red, *not verified* amber and *already on the server* green. *(Changed 2026-10-18 with the row and the foot; No auditor has seen it.)* A check with no red and no amber needs nothing before Upload. Label: **Creator-stated** (2026-10-08, sixth notes, note 4). **If seven genuinely distinct colours are ever wanted, they enter as named tokens in [Design values](Design%20values.md) first**, the way the four transfer tags did — that is a decision made in the values document and never on a screen.

**Where several fail for one reason, the [whole-app Design](Whole%20app.md)'s once rule holds** and this panel does not answer it separately: one statement above the group, with each row keeping its own state and its own *Retry*. Each is tried again **on its own**, without re-running the check or re-sending what already arrived — which the [spec](../Specs/Dependency%20Upload%20&%20Remap.md) requires.

**A file the server refused to write for permission is ~~*Not uploaded*~~ *Failed* *(the screen's name from 2026-10-18)* like any other, and keeps its *Retry* — with no admin-access offer on the row or on the statement above it.** The [architecture](../Architecture/Dependency%20Upload%20&%20Remap.md) A11 gives every row that could not be sent a retry on its own, and the [File Browser architecture](../Architecture/File%20Browser.md) P6a gives admin access no upload to retry. *(Added 2026-09-27 under Rule 10, so that this document and [Transfers](Transfers.md) say the same thing about these files; no auditor has seen it.)*

**Accepting a capitals-only match reads as taking a proposal, not as finding a file.** ~~The **Accept** sits on the qualifier under *What Smoozy found*, and the qualifier says what the difference is — *found as `METallic.png`, which differs only in capitals*.~~ **Accept** is the row's one visible action, and its note says what the difference is — *Only the case differs: metallic.png*. *(Changed 2026-10-18 with the row; No auditor has seen it.)* Over a selection the same control accepts each row's own candidate one at a time, which the [architecture](../Architecture/Dependency%20Upload%20&%20Remap.md) F24 requires, and a selected row with no proposal is untouched. The count on the control says how many will be accepted, and the bar says how many are chosen apart from it, so a selection of two hundred rows with four proposals does not read as two hundred acceptances.

**Capitals differing is not a state, and this table used to say it was.** The [spec](../Specs/Dependency%20Upload%20&%20Remap.md) matches exactly, capitals included, so a name differing only in case is **not found** — and what Smoozy shows beside it is a proposal rather than a match. So it is a ~~qualifier under *What Smoozy found*~~ note, as the row description above already has it, and the **Accept** that goes with it sits on that row rather than on the state. *(Changed 2026-10-18 with the row.)* Making it an eighth state said Smoozy had found the file, which is exactly what the spec says it has not. Label: **Creator-stated** (2026-08-13).

**The rest of the actions sit behind the three dots on each row**: pointing it at a file (*Locate…*), ~~setting it aside~~ *Ignore*, bringing it back (*Restore*), and *Reject match*. *(Changed 2026-10-18: the screen's word is* Ignore*.)*

## How the rows are grouped and sorted

**Grouped by state, in the order of what the user has to do about them**: *Failed*, *Missing*, *Uploaded before, not verified*, *To upload*, *Already on the server*, *Ignored*. The failures sit above all the rest while they exist, because they are the only rows where something has already gone wrong. **A heading's count is of rows; a count of what will upload is of files**, so a folder sent as one is one row and every file in it.

**Already on the server is folded away, with its size on its heading** — *● Already on the server 1 · 128 MB* — because nothing is left to do there, and it opens like the group below.

*Changed 2026-10-18, the dependency check's fresh look, approved by the Creator on 2026-10-18 with both its decisions answered yes (`Audits/Build/Step 10c/The dependency check, a fresh look — the proposal.md`, from the Creator's twelfth notes, note 5). Until then the order read:* ~~Not found first, then found on your computer, then sent before and not confirmed, then already on the server. The two failures — could not be sent, could not be fixed — sit above all of them while they exist~~*, and Already on the server was open. No auditor has seen it.*

**~~Set-aside~~ Ignored rows sit at the bottom, in their own group, collapsed.** They are the rows the user has already answered, and the [spec](../Specs/Dependency%20Upload%20&%20Remap.md) is firm that setting one aside is the user saying they know and do not care. Keeping them among the rest would make a check that is done read as a check with work left in it. The group says how many, and opens.

**Inside a group, by the location the file records**, so the folders of a project stay together and a shift's effect reads as a block rather than as scattered rows.

**A row that is itself another `.blend` sorts to the head of its group.** Those are the rows hiding an unknown amount beneath them, and the [spec](../Specs/Dependency%20Upload%20&%20Remap.md) says one of them may put many more rows on the screen — so a user reading a short list needs the ones that may lengthen it at the top rather than buried.

**Nothing re-sorts under the user's hand as they work.** A row whose state changes stays where it is, marked, until the check is run again. A list that reshuffles as rows are resolved loses the user's place at exactly the moment they are working through it.

## When more than one reference points at one thing

**It is ~~the fifth qualifier~~ a note and it belongs with the others.** The [architecture](../Architecture/Dependency%20Upload%20&%20Remap.md) G10 groups references into one row where they resolve to the same file and keeps the record beneath per reference — so the row is one thing and the references behind it are several, which is a fact the user can act on: locating it once fixes all of them.

**It reads ~~*used by 4 references*, under what Smoozy found~~ *Referenced 4 times***, with the other notes. *(Changed 2026-10-18, the dependency check's fresh look, approved by the Creator on 2026-10-18 with both its decisions answered yes (`Audits/Build/Step 10c/The dependency check, a fresh look — the proposal.md`, from the Creator's twelfth notes, note 5); No auditor has seen it.)* It is not a state and not a warning; it is what the row stands for.

**And it says the count rather than listing them.** The references differ by the text inside the file, which is not something an artist recognises, and the [architecture](../Architecture/Dependency%20Upload%20&%20Remap.md) G10's own example is two spellings of one name — so a list would show near-identical lines and answer nothing. The count says what matters: this one act covers more than one.

## Acting on many rows

**Rows can be selected and acted on together.** A file with two hundred missing textures is not one the user ~~sets aside~~ ignores a row at a time.

**Rows are chosen the way a file manager chooses them**: a click chooses a row, Shift-click a range, and Cmd-click (Ctrl-click on Windows) adds a row or takes one away. A chosen row takes the selected ground. **Pressing a row's own button or its ⋯ never changes what is chosen.** There is no checkbox, so nothing reads as *tick to upload*, and the user can still act on 30 rows out of 200. **This is the first place in Smoozy where several rows are chosen**; choosing several in Files is parked.

**Once two or more rows are chosen, a bar appears above the rows**: *20 selected · Accept 3 · Choose a folder… · Ignore · Reject match*. It says how many are chosen apart from how many *Accept* will take, and it shows only the acts that apply to at least one chosen row.

**Four acts sit over a selection**: **Accept**, carrying the count of how many of the selected rows have a proposal; **Choose a folder…**, which is what pointing at a file becomes over many rows at once — it points the chosen rows at one folder now and does not add a search folder; ~~**Set aside**~~ **Ignore**; and **Reject match**.

*Changed 2026-10-18, the dependency check's fresh look, approved by the Creator on 2026-10-18 with both its decisions answered yes (`Audits/Build/Step 10c/The dependency check, a fresh look — the proposal.md`, from the Creator's twelfth notes, note 5), decision 2: rows were chosen with a checkbox each, which read as choosing what uploads. No auditor has seen it.*

**Restore is not one of them.** It acts on ~~set-aside~~ ignored rows, and those sit in their own collapsed group at the bottom rather than in the list a selection is made from.

## Along the bottom

**Only the button**, naming the number of files and their size rather than saying *Send*: *Upload 2 files (90 MB)*, or *Upload the 4 found so far* while the check is still reading. **It uploads what is under *To upload*, and nothing else.** The figures are the summary sentence's, at the top. **It counts and sends only the rows not already on their way**: with 2 of 5 files uploading, it reads *Upload 3 files (60 MB)*, and where every row under *To upload* is on its way the button is not there and the strip's *Uploading 4 of 12* says what is happening. **A second press can never send a file twice**: what goes is decided from what Transfers is sending at the moment of the press, not from what the panel last drew. *(Added 2026-10-24, the eighteenth notes, note 6; `Dependency Upload & Remap` A9a. No auditor has seen it.)*

The spec argues the whole module from size, so the size goes on the control the user presses when deciding whether to start now or leave it overnight.

*Changed 2026-10-18, the dependency check's fresh look, approved by the Creator on 2026-10-18 with both its decisions answered yes (`Audits/Build/Step 10c/The dependency check, a fresh look — the proposal.md`, from the Creator's twelfth notes, note 5). Until then it read:* ~~**How much will upload, how much is already on the server, how many were not found, and how many failed** — then the action, naming the number of files rather than saying *Send*.~~ ~~The spec argues the whole module from size, so the number goes where the user decides whether to start now or leave it overnight.~~ *No auditor has seen it.*

## This panel's own overwrite warning

**It is replace or stop, and never the four answers the file browser offers.** The [spec](../Specs/Dependency%20Upload%20&%20Remap.md) closes the other two by name and the [architecture](../Architecture/Dependency%20Upload%20&%20Remap.md) Q8 keeps them closed: the landing rule sends a file to the mirror of its own location and nowhere else, so *keep both* has no second place to put it, and *skip existing* would leave the reference pointing at a file that is not the one it means.

**It fires on a different reading from the browser's, and the wording says which.** The browser compares names and places; this one has read the file at the landing place and found the contents differ. So the line says that outright — *a different file is already at that location* — rather than *already there*, which is the browser's weaker claim and would make two different checks look the same.

**Over a set row it widens to the folder** and names two things: the members a send would write over, and the members standing in the mirrored folder that the local folder does not hold. The [architecture](../Architecture/Dependency%20Upload%20&%20Remap.md) Q8 is explicit that a send onto a folder holding more than the row means is the same harm by a quieter route, so the second half is not a detail.

**And the live-job warning here is the browser's box, not a second one.** The [architecture](../Architecture/Dependency%20Upload%20&%20Remap.md) V3 gives the wording to [File Browser](File%20Browser.md) for all three warnings, and one warning in two shapes would be two things for the user to learn. It appears above the replace-or-stop choice, the same way it appears above the browser's four.

## When the file could not be read at all

**No rows, and the panel says why with the version named.** The [architecture](../Architecture/Dependency%20Upload%20&%20Remap.md) G3 is exact: a run that returned nothing is *could not be read* and never an empty list, and an empty answer is never inferred from a missing one.

**So the panel must not show an empty list, and that is the whole point of the marker.** An empty check reads as *nothing is missing*, which is the most dangerous wrong answer this screen can give — the user submits, and the render looks for its pieces on a computer that is not there.

**It takes the whole-app failure shape**, attached to the check itself.

**The read ends four ways, not one, and they are the four [Render Settings](Render%20Settings.md) sets out.** Both screens run a Blender on the server to read a `.blend`, so both get the same four answers, and this panel does not list them again.

**Only one of the four carries a second sentence** — the ending where that Blender could not open the file, which is the one Smoozy knows what to do about: pick another Blender and check again. The version named there is Smoozy's own record of which build it ran. The other three say what happened and stop. Telling a user their file cannot be opened when the connection dropped, or when Smoozy stopped the read, would send them to change version for a reason that was never the file's.

**The list area says what it is rather than standing empty.** One line where the rows would be: *File not read; dependencies unknown.*

## When the connection drops mid-check

**The sitting stands and what was found stays on screen.** The [architecture](../Architecture/Dependency%20Upload%20&%20Remap.md) G16 makes every finding and every decision a durable write at the moment it lands, so nothing already decided is lost and nothing is re-decided when the check runs again.

**What changes is one band at the top saying the check is incomplete**, and it says which half is missing: *Connection lost. Check incomplete.* The [spec](../Specs/Dependency%20Upload%20&%20Remap.md) asks for it to be said plainly, and *plainly* here means the user must not read a short list as a complete one.

**The rows keep their actions where the action does not need the server.** Locating a file on the user's own computer, ~~setting a row aside~~ ignoring a row, bringing one back and undoing a guess all still work — they are decisions about the user's own machine, and the [architecture](../Architecture/Dependency%20Upload%20&%20Remap.md) K4 records them the moment they are taken. Sending, ~~confirming~~ verifying and trying again go unavailable, because all three need the machine.

**And the upload control says what it is uploading** — *Upload the 4 found so far* — because a control naming a final number on an incomplete check is the same lie the empty list would tell, one number smaller. The band above carries the rest; ~~the figures beside it are not repeated with a caution each~~ the summary sentence ends *, found so far* rather than repeating a caution on each figure. *(Changed 2026-10-18 with the foot.)*

## A guess the user rejected

**The row says so and offers it back, and that is the whole of it.** The [architecture](../Architecture/Dependency%20Upload%20&%20Remap.md) K4 keeps a rejected guess so Smoozy does not put it forward again by itself and so the user can reinstate it — two promises that need one line and one control.

**It ~~reads under *What Smoozy found*, where the proposal used to be~~ is a note**: ~~*you rejected this match*, with **Use it** beside it~~ *You rejected METallic.png*, with **Use it** as the row's one visible action. *(Changed 2026-10-18, the dependency check's fresh look, approved by the Creator on 2026-10-18 with both its decisions answered yes (`Audits/Build/Step 10c/The dependency check, a fresh look — the proposal.md`, from the Creator's twelfth notes, note 5); No auditor has seen it.)* The row's state goes back to what it was without the guess — not found — so the row is honest about where it stands while the offer sits beside it.

**Smoozy never puts it forward again on its own**, which is why the line is needed at all: without it, a user who rejected a match and came back a week later would see a plain ~~*not found*~~ *Missing* row and no sign that Smoozy had ever had a candidate for it.

**Undoing a guess over many rows uses the same words.** The [architecture](../Architecture/Dependency%20Upload%20&%20Remap.md) F24 and F25 write one rejection per row and take the inverse over the same selection, so nothing here is a bulk shape of its own.

## While it is still reading below a linked file

**The panel keeps working and says which file it is inside.** A linked `.blend` that has just been located or sent opens a tree nobody has read yet, so the check goes on running under a list the user is already looking at.

**It is the header's own line, not a row and not a veil.** *Reading files linked from `props.blend` — progress unknown* — with the underway mark on it — because the rows already on screen are true and stay usable, which is the whole-app rule for anything that leaves the screen usable.

**~~The count along the bottom~~ The summary sentence carries the same caution as the dropped-connection band**, ending *, found so far*, and the button reads *Upload the 4 found so far*: it is a total of what has been found so far, and more may arrive. *(Changed 2026-10-18 with the foot.)* The [spec](../Specs/Dependency%20Upload%20&%20Remap.md) is explicit that one linked file can stand for a hundred, so a number that reads as final here would be wrong by any amount.

**And the rows that arrive go into their groups without moving what is above them.** That is the same no-reshuffle rule the sort follows, and it matters more here, because rows are arriving while the user is reading.

**While the file itself is being read, the header's line says *Reading the file…* with the underway mark, and no table, no tools and no foot are drawn until its reading lands.** An empty table there is the empty list [above](#when-the-file-could-not-be-read-at-all) calls the most dangerous wrong answer, shown for a few seconds on every check. *Added 2026-10-03, the Creator's note 8 on the test copy; nothing covered the root being read. A hole a fault exposed, ruled by the build runner; no auditor has seen it.*

## The search folders on this panel

**Two lines. The first names the folders searched; the second adds one.** The list, its order and what the order decides still belong to [Settings](Settings.md): the first line shows the folders this search used, in Settings' order, and nothing here edits them.

**The first line names them by their last folder name** — *Searched Work, Assets, Textures.* — and past three says how many more: *Searched Work, Assets, Textures and 2 more.* The full path shows on hover. ~~*The matches came from …* follows it where a pick came from one, which is what the row's own qualifier needs.~~ **Each folder name is a link, drawn as the page's other links, and opens that folder in the computer's own file manager** — the folders are on the user's computer, so Smoozy's Files, which shows the server, cannot show them. ***The matches came from …* is gone.** *Changed 2026-10-04, the Creator's note 5 on the test copy: "the folders should be links … And they should look like links of course. The matches came from — we can drop that entirely." The link opens the folder the same way a job's downloaded files are opened on the user's computer from its entry. Settled at Design by the build runner; no auditor has seen it.*

***Check again* sits on that line at all times**, not only where the file could not be read, **and it is a button, the same small button the failure shapes carry** *(it was drawn as a link until 2026-10-03, the Creator's note 6 on the test copy: "Check again in dependencies doesn't look like a button at all")*. It reads the file again and searches the folders Settings now lists. Answers the user gave stay, which is the [architecture](../Architecture/Dependency%20Upload%20&%20Remap.md) K4's rule. It is the one refresh: it covers a folder added in Settings, a texture moved on disk and a changed `.blend`. *(2026-10-18: "at all times" means wherever a check has run. Where none has, there is no folders line — the next paragraph.)*

**Where no check of this file has run since Smoozy was opened, there is no folders line**, because nothing was searched: the panel says *File not read; dependencies unknown.*, and under it one button, ***Check dependencies***, which starts the check. *Check again* is only for a check that ran. *Added 2026-10-18, the Creator's thirteenth notes, note 2: "it's only "again" when it has been done before". No auditor has seen it.*

**Where Settings' list is no longer the list this search used, the line says *Search folders changed since this search.*** with *Check again* beside it. That is a comparison when the panel is drawn. **Nothing searches by itself**: a search over large folders runs for minutes, and starting one the user did not ask for is the wrong default — except once after an upload ends, which [the spec](../Specs/Dependency%20Upload%20&%20Remap.md#checking-a-file) asks for. *(The exception added 2026-10-17, the Creator's tenth notes, note 4. No auditor has seen it.)*

**The second is a way to add a folder without leaving the check**, because the moment a user learns their folders are wrong is the moment a row reads ~~*not found*~~ *Missing* — sending them to Settings and back to lose the sitting would be the wrong answer to the right realisation. Its box says it in one sentence: *Adds it to your search folders in Settings and searches it for missing files.*

**The box lists the search folders already in Settings first**, full paths, one line each, in Settings' order, to read and not to edit, with *Change them in Settings* opening Settings' list; then *Choose a folder…*. So the user sees what is already searched before adding to it, without leaving the check. *Added 2026-10-03, the Creator's note 7 on the test copy: "Search folders... should also give me a list of what I already have added. I don't have to need to go to settings to look it up." Under Rule 10, F1's one list is shown and not changed; no auditor has seen it.*

**Adding one re-runs the search over the rows that are still not found, and nothing else.** Rows the user has already answered by hand are not re-decided, which is K4's own rule: a located answer is a durable decision, not a guess to be revisited.

***Add* closes the box, and the panel says once what it did**: *Added Assets. 4 found, 3 still ~~not found~~ missing.*, or *Added Assets. Nothing more found.* *(The screen's word became* missing *on 2026-10-18, with the states.)* The new name is then in the first line. The folder goes into Settings' list in the order that list already defines.

*Rewritten 2026-10-02. **Creator-stated**, note 5 on 0.1.8: "Adding a search folder to the dependency checker is confusing. Have I added it? … And I think there should be a button to refresh the check or something." The Creator's want overrides this section's earlier "neither is the list": the first line now names the folders. Nothing said what follows Add, and the box stayed open, empty. Under Rule 10: F1's one list, K4's durable answers and the existing Check again are used unchanged.*

## What is deliberately absent

**The location a `.blend` records pointing into Blender itself is not a row.** There is nothing for the user to do with it, and leaving it in would mean the check never reads as complete — which is the [spec](../Specs/Dependency%20Upload%20&%20Remap.md)'s own reasoning.

## Keyboard shortcuts

**None of its own beyond choosing rows**: Shift-click and Cmd-click (Ctrl-click on Windows), as [above](#acting-on-many-rows). ~~Its rows are chosen with checkboxes rather than as a single selection, so~~ Return and the delete key have no one thing to act on, and every act here is one the panel already offers on the row or on the bar. *(Changed 2026-10-18, the dependency check's fresh look, approved by the Creator on 2026-10-18 with both its decisions answered yes (`Audits/Build/Step 10c/The dependency check, a fresh look — the proposal.md`, from the Creator's twelfth notes, note 5), decision 2; No auditor has seen it.)* The whole-app rules are in [Whole app](Whole%20app.md).

## What this document left open, and how it was answered

Every marker this document once carried is closed, and the answers are kept together here rather than folded back into the body. **Nothing below is still open** — the heading used to say *Left for later*, which was true when the markers were markers and stopped being true the day they were answered.


- **Settled at Design (2026-08-23), drawn in the mockup.** The statements sit in a block under the header, above the rows: what could not be read, the sitting that was cut short, the folders that were searched, a linked file whose named item is gone, and a file that was itself sent up as a dependency before — that last one the [spec](../Specs/Dependency%20Upload%20&%20Remap.md)'s own: *Uploaded before as a dependency. Its paths still point to your computer.* The linked-file one reads *`props.blend` is here, but data linked from it is missing. Blender does not say which.* — no control, nothing to dismiss, and the honest admission of what Blender withholds, which is the [spec](../Specs/Dependency%20Upload%20&%20Remap.md)'s own requirement.
- **A capitals-only match is accepted per row, and a selection accepts many of them one at a time.** Each row names its own candidate, and **no act over a selection ever changes what counts as a match** — byte equality decides what is found and the fold decides only what is offered, so a bulk accept is many acceptances and never a rule that folds. Label: **settled at Architecture** (2026-08-23). How the acceptance reads on the screen is still this document's.
- **Settled at Design (2026-08-23).** The folders are managed and ordered in [Settings](Settings.md), which settles the list, the order and what the order decides; this panel shows two lines and never the list — how many were searched with the one a pick came from, and a way to add a folder without leaving the check. **Corrected 2026-10-02:** the first line now names the folders searched, with *Check again* beside it ([The search folders on this panel](#the-search-folders-on-this-panel)), on the Creator's note 5 on 0.1.8; the list is still Settings' alone.
