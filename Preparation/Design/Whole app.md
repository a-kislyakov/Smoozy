# Whole app

*The Design decisions that hold across the whole app: the window, where things live, the conventions each screen follows, and the words and typography everything is written to — together with the parts of Smoozy that sit on no screen at all, which are the notifications, the update box and the prompt after a crash. Each screen's own document leans on this one and does not restate it.*

*2026-10-27 — Removed the withdrawn paid-access text and its obsolete references under the Creator’s free-and-open-source decision of 2026-10-04 (wave 5); retained protections and evidence keep their force. No auditor has seen this removal.*

**Where the numbers are.** This document and the screen documents beside it hold the decisions and the reasoning. They hold no colours, sizes or spacings — those live in one place, the token block at the top of [the mockup](Mockup/Smoozy.html), and [Design values](Design%20values.md) says what each one means. So a decision has one home and a number has another, and neither can drift into the other.

## The window

**Smoozy is one window, and it never scrolls as a whole.** It fills the screen it is given, and everything inside it fits. Where there is more content than room, the pane holding it scrolls on its own — the file list, the queue, a panel's body — and everything around that pane stays where it was.

That is a decision about what kind of thing Smoozy is. A page scrolls; software does not. Label: **Creator-stated** (2026-08-10).

**Nothing already drawn moves when something appears or disappears above it after the page is up.** A line that can go keeps its space when it goes, or sits below the controls rather than above them. Otherwise a click aimed at one field lands in the one that moved under the pointer. *Added 2026-09-27, out of step 10c's seventh pass on Sesterce: picking the Blender on Render Settings took a line away above the fields, they moved up about 30 pixels, and a click meant for Samples typed into Resolution. This is Design conforming to Architecture under Rule 10: no architecture answer touches layout.*

**A press already on its way is not lost to a redraw:** a page that redraws while a control is held down keeps that control, and the press lands where it began. *Added 2026-09-27, out of step 10c's eighth pass: a press on Render just after typing into a field did nothing, most likely because the field's value was committed on the press and the page was drawn again before the release — read in the page, not yet proved. Design conforming to Architecture under Rule 10; no architecture answer touches it.*

**Text being typed is not lost to a redraw either:** a page that redraws while a field has the focus keeps what is typed in it, not yet committed, and where the caret was. *Added 2026-09-28, out of step 10c's ninth pass, where a job name typed into Render Settings was gone 0.3 s later with the caret still in the field. Design conforming to Architecture under Rule 10; no architecture answer touches it.*

**Where the user had scrolled is not lost to a redraw either:** a list drawn again with the same things in it keeps its place, and a list that gains or loses rows keeps the row the user was looking at in view. *Added 2026-09-30, out of step 10c's eleventh pass, where a Files list receiving a thousand-file upload jumped to its top at every file. Design conforming to Architecture under Rule 10; no architecture answer touches it.*

**A row in Files is one line, and each line of a row in the queue or History stays one line.** A cell that does not fit ends in an ellipsis and shows whole on hover, and a row's icon and its name sit on that line together. *Added 2026-10-03, the Creator's note 9 on the test copy, where a long Kind wrapped and the name fell out of line with its icon. Nothing had said whether a row may wrap. Settled at Design by the build runner; no architecture answer touches it. No auditor has seen it.*

**Smoozy draws its own dropdowns.** A pressed one opens a list drawn as the right-click menu is, with the picked row in bold, never the system's own list; a pick closes it and leaves the control as it looked before it was pressed, and the keys work as in any list. *Added 2026-10-08, the Creator's fifth notes, note 2: the list "looks like it doesn't belong in this design, and when I have picked something, the dropdown appears to be selected." Settled at Design; no architecture answer touches it. No auditor has seen it.* **A list with no room below it opens upward, so the window's edge never cuts it off** — the rule that keeps the right-click menu inside the window. *Added 2026-10-18, the Creator's thirteenth notes, note 1: "It should be smart enough to understand that this list should be expanded upwards or something like this." No auditor has seen it.* The Queue entry's Telegram list, at the very foot of the entry, is held to it: *"Just make sure the lists expand upward, since they are at the very bottom."* *Added 2026-10-31, the Creator's twenty-fifth notes, note 1; already so in the app, and now tested. No auditor has seen it.*

**The layout is horizontal first.** Panes sit beside each other rather than stacking into a tall column. Narrowing the window drops the least important pane rather than reflowing everything into a strip.

**Every step answers to Smoozy's own window, not to the screen.** That is what the layout actually has to fit inside — a window at half the width of a large monitor has the same problem as a small monitor, and the first decision on this page is that Smoozy is one window that never scrolls as a whole. Label: **Creator-stated** (2026-08-13).

**Four steps, in the order of what the user can most afford to lose, and then Smoozy stops.**

1. **The preview column goes first.** It is the one thing on a job's entry that is a glance rather than work, and everything on it is a click away. It moves under the main column rather than vanishing.
2. **Then the server list folds its hardware under its work column, and the file list drops *Kind*.** Both are the column a user scanning a list reads last. The settings panel drops to two columns here as well, and the entry screen's two cards go one above the other.
3. **Then the rail narrows, the list narrows with it, the file list drops *Changed*, and Settings' own section list narrows.**
4. **Then the rail loses its words and keeps every place.** An icon at the top for All servers, badges alone for the servers, icons for Queue, History, Transfers, Keys and Settings, each with its name on hover. The rail and the list narrow again, and the side gutter narrows with them — once, at the last step, for everything at the same moment.

**Nothing in that fourth step leaves, and that is the point.** The rail is the only way to reach a server — [Servers](Servers.md) settles that the list screen compares them while the rail moves between them — so a plan that narrowed the rail until it could not hold a name and then stopped there left the user unable to pick one at all. A badge is already the thing that says *which server*, so a rail of badges is the rail doing its one job in a quarter of the width. Label: **Creator-stated** (2026-08-13).

**That rests on the badges in use being distinct, and Smoozy's own pick keeps them so up to eight.** It takes the first of the eight nobody is using, so eight servers added one after another have eight different badges. Where the user has set two servers to the same hue, or holds more than eight, two rows carry the same badge and the collapsed rail is a set of names on hover for those two. That is the user's own choice and it stays theirs.

**Every seam between two panes drags.** The rail against everything beside it, a list against the entry beside it, and a job entry's main column against its preview column. A desktop application whose panes are a fixed size reads as a web page, and the first decision on this page is that Smoozy is not a page. Label: **Creator-stated** (2026-08-23).

**Three rules hold it together, and none of them is optional.**

- **Each pane has a smallest and a largest useful width, and the drag stops at both.** A pane cannot be dragged into uselessness and then have to be hunted for; and past its largest width a pane is only taking room from the thing it sits beside.
- **The narrowing steps still govern underneath.** A dragged width is the user's own, and each step above re-declares the width it needs, so from that step's width down the step wins and the user's width is set aside — not destroyed. Widen the window again and their width comes back. From the third step down nothing drags at all: the rail and the list have both taken the widths the app declares for them, so there is nothing left on the line that is the user's to move. The badge rail a step further down is the app's decision about what fits rather than the user's, in the same way.
- **Double-clicking a seam puts it back where it started.** That is the way out of a mess, and it is the only reset there is.

**The rail's width is one width everywhere**, because the rail is on every screen and a rail that changed width between screens would be a moving target. **A list's width is one width for the queue and history both** — they are the same shape of screen, and a user who sets it on one means it on the other.

**The widths are remembered between runs**, on the user's own computer, with everything else they have set — one value for each seam, written when a drag ends.

**Below that Smoozy says the window is too small rather than folding.** A fifth step would mean reflowing into a single tall strip, which is what a page does — and the first decision on this screen is that Smoozy is not a page. So there is a floor, it is stated on screen, and it names the width. What that floor is in pixels lives with the other numbers, in the mockup.

## The window's own chrome

**Smoozy draws its own title bar on all three systems, and it is one bar at one height everywhere.** The name sits at the left, the transfer speed at the right, and the bar is what the user drags the window by.

**The window buttons are each system's own, and none of the three is drawn the same way.** Label: **Creator-stated** (2026-09-02) — *whatever needs to happen for everything to look OS-native.* What each system actually does is **documented**, from that system's own publications; the sources file carries them.

**On a Mac, macOS draws them.** The window keeps the system frame and Smoozy's page fills it, so the three buttons are the ones every Mac user knows — their own place, colours and order — and the window comes with the system's rounded corners and its shadow. Smoozy draws none of its own.

**On Windows, Smoozy draws them and they go at the right.** Windows reserves a corner for them and puts close in the upper right; the frame is given up, so drawing them is Smoozy's job and Microsoft publishes what they should look like — minimise, maximise and close, each a full-height plate rather than a small shape, each responding to hover and press. The corners come back for nothing: an undecorated window on Windows 11 is given rounded corners by the system when the window keeps its shadow.

**On Linux the desktop decides, and Smoozy asks it rather than guessing.** There is no single Linux arrangement, and that is the finding rather than a gap: the desktop publishes a setting saying **which buttons to show and on which side**, with a colon separating the left corner from the right. **Its own default is a close button alone, on the right** — not three buttons. So Smoozy reads that setting and draws what it says, on the side it says. Where nothing can be read, it draws the three at the right.

**The buttons Smoozy draws are named *Minimise*, *Maximise* and *Close***, on Windows and Linux alike — the name a screen reader says and the tooltip each one shows. They are the standard words under Rule 9, and the three Microsoft's own pages name (the sources file carries them). A Mac's buttons are macOS's own and carry its names. *(Added 2026-10-24, at Build step 12, piece P1. The code had used these three words since Build step 1, as stand-ins waiting for this sentence.)*

**While the window is maximised, the middle one is named *Restore***, as desktop apps name it, and *Maximise* again once it is not. *(Added 2026-10-25, the build runner's ruling under the Creator's standing delegation, the small owed fixes after the eighteenth notes: it read* Maximise *on a maximised window. No auditor has seen it.)*

**Why not one arrangement everywhere.** It was three grey circles on the left on all three, which is macOS's arrangement worn by two systems that do not use it. The alternative to asking each system is picking one and being wrong twice.

**Two costs are carried rather than hidden.** With the Mac's frame kept, the shell publishes that a window cannot be dragged while it is not the front window — one extra click on a window not yet touched. And **the colour Windows uses behind the close button when the pointer is on it is the system's own and is not published**, so Smoozy's is an approximation and may not match a given machine exactly.

**One thing is honest to say plainly: none of the Windows or Linux drawing can be seen working.** This project has no Windows computer and no Linux desktop, and renting one to answer a question is ruled out. Both are built from each system's own documentation and stay unproved until somebody runs them.

*(Settled 2026-09-02, after the first window was built and looked wrong. The drawing shows three grey circles and had never said whose they were, which was which, or where they go on a system that is not a Mac.)*

## The menu bar

**There is one, and it is deliberately small.** On a Mac it is not Smoozy's to decide: the operating system puts a menu bar at the top of the screen whether the app fills it or not, so the only real question is what goes in it. On Windows and Linux the same menus sit behind one button in the title bar, and that arrangement is Smoozy's own choice rather than something either system forces. Label: **documented** — checked on all three systems from each platform's own publications; see [Sources](Sources/Whole%20app.sources.md).

**It holds only what has no other home.** Every action in Smoozy lives on the thing it acts on, and a menu bar repeating them would be a second copy of the whole interface — two copies of a decision, which is the thing this project's conventions exist to prevent. So there is no **File** menu, because Smoozy opens nothing and saves nothing of its own; and no menu of server actions, because connecting, setting up and removing belong to the server's row.

Four menus, and no fifth:

- **Smoozy** — About, Settings, Check for updates, **Report a problem…**, Hide, Quit. Settings is here as well as in the rail, because that is where a desktop user reaches for it.
- **Edit** — cut, copy, paste, select all. **There is no Undo**, and its absence is the decision: nothing Smoozy does to a server can be taken back, and a permanently grey Undo says that more slowly than an empty space does. **A focused text field keeps the operating system's own undo while it is being edited** — the missing item is about Smoozy's acts, not about typing.
- **Go** — All servers, Queue, History, Transfers and Keys, each with a number, and up one folder. **The servers themselves are not in it**, so the rail stays the only way to reach one and the numbers do not change when a server is added. Settings is not in it either: it is the one rail place reached from the Smoozy menu above, with a key of its own rather than a number. Up one folder is here as well as on the File Browser, because that is where a desktop user reaches for it.
- **Window** — the operating system's own.

**There is no Help menu, and this document argued for one until 2026-08-25.** The argument ran that reporting is about something the app did wrong rather than about the app itself, so it belonged under Help. **The [spec](../Specs/Reporting%20a%20problem.md) had already settled it the other way** — *"In the Smoozy menu, always"*, two places and no third, Creator-stated — and this stage does not overturn a spec. Reporting is now in the Smoozy menu, and with it gone there was nothing left for a Help menu to hold: the only other item it ever had was a *Smoozy help* that was wired to nothing and described in no document anywhere. So the menu itself goes. Label: **Creator-stated** (2026-08-25).

**That is the whole of what changed.** The item's own behaviour, its wording and the box it opens are untouched; only which menu carries it moved. The other of the spec's two places — beside a failure that has room for it — is unaffected.

**Cut, copy and paste act on whatever has focus.** [File Browser](File%20Browser.md) gives those three to a server's files, any text field wants them too, and **the selected text in the failure panel below is the third claimant** — copy is the whole reason that panel's text is selectable, and pasting Blender's own words into a search is what a user actually does with them. They stay one set of three items either way, so the focused thing decides and nothing has to be chosen between.

## Selecting text

**Text is selectable only where a person would paste it somewhere else**: what a server or Blender printed, and a value shown exactly as Smoozy holds it — a path, a server's address, a key's fingerprint or public half, a job's file name, a bot's username, About's version, the report's file name and address. Everything else — headings, labels, names on rows, badges, buttons, statuses, counts, figures and Smoozy's own sentences — is not, so a drag or a Shift-click never paints it. A text field always is.

**Three exceptions, and only three.** **Nothing a click acts on is selectable, whatever it holds** — a row that opens or is chosen, a link, a path's clickable parts; where a value on such a row is worth copying, the row's menu carries Copy, as *Copy address* does. **A failure's sentences are selectable wherever it is shown whole**, never on its one-line form on a row: an error sentence is what people paste into a search or a message. **The report's address selects whole on one click.**

**The pointer is the arrow over text that cannot be selected and the text cursor over text that can.** Copy still acts on whatever has focus (above, and `Desktop app and delivery` K7), so nothing about the Edit menu changes.

*Added 2026-10-24, Creator-stated, the eighteenth notes, note 2: "Some text in Smoozy is off limits for selection. Like, the name of the server for example — I don't need it to be selectable at all." The rule, its three exceptions and the screen-by-screen list are the scout's (`Audits/Build/Step 10c/The Creator's eighteenth notes, note 2 — what text should be selectable.md`), taken whole on the build runner's ruling under the Creator's standing delegation. No Specs or Architecture answer says which text may be selected, so it enters here; Design conforming to Architecture under Rule 10. The drawing does not carry it: its stylesheet is compared line for line with the app's, so the app keeps the rule in a sheet of its own, as the scout lays out. No auditor has seen it.*

## Keyboard shortcuts

**A desktop app is judged on these, and they come in two kinds.**

**The whole app's** are in the menu bar, which is where a desktop user looks to find one: the places in the rail, Settings, quit, and the clipboard three.

**A screen's own** — a key that deletes the chosen file, one that collapses a panel, one that renames — belong to that screen and are settled in its own document, not here. They are not in the menu bar, because a menu holding them would be the second copy this design has just refused.

**Three rules bind every screen's own set, and they are here because nine documents each inventing them would not agree.**

**A screen's own key is never a key that changes something, except delete.** Renaming, deleting, collapsing a panel and moving between rows are the shapes a screen may bind. Sending, submitting and stopping a render have no key on any screen. The difference is what delete does and they do not: it acts on a row the user has chosen, and it puts a box in front of them before anything happens.

**Delete is the exception and it is not really one**, because it opens the box rather than doing the thing. Every screen that has a delete binds the delete key to it, and every one of them then asks exactly as the control does.

**Escape closes the thing on top, one layer at a time**, and nothing else. A box, then a panel, then a menu — never a screen, because the rail is where a screen is left.

**A box that asks for typing opens with the caret in its first field**, so the user types at once. Escape still closes it. *Added 2026-10-06, Creator-stated, from the Creator's third notes on the test copy (note 1): "the first input field should be active right away."*

**On a panel, Escape collapses rather than ends.** The panel goes to the strip along the bottom and keeps counting, and pressing the strip brings it back exactly as it was. **A panel's work is never stopped by a key** — stopping something is a deliberate act with a control behind it. This binds every panel on every screen.

**Return presses the box's own affirmative control where the box has one**, and never where the affirmative control is a red one. A box whose answer is *Delete* takes no Return.

**And each screen's set is listed in its own document, under that screen's own *Keyboard shortcuts* heading**, so a reader of one screen finds its keys without reading this one, and this document does not carry a second copy of them.

## Where things live

**The speed transfers are actually running at sits in the title bar**, so it is readable from every screen rather than only on Transfers, and it is there only while something is moving. The [Whole app spec](../Specs/Whole%20app.md) asks for that reading so a speed limit can be judged against something real, and leaves where it sits to this stage. **This is the whole app's reading of that number** — [Transfers](Transfers.md) and [Settings](Settings.md) each carry one of their own beside the thing it belongs to. Label: **settled at Design** (2026-08-24).

**Down the left is a rail for navigation, with each server’s menu available on right-click or Shift+F10.** *Changed 2026-10-23 for seventeenth note 16; the former sentence was “Down the left is a rail, and it is navigation only.” The menu’s rules live in [Servers](Servers.md).* It holds, in order: the servers, then the places that belong to no server — Queue, History, Transfers — then Keys and Settings.

**A server is a place you go into, and it has faces.** Selecting one opens it, and its own screen carries tabs: **Files** and **Set up**. Everything that happens on a server happens inside it. This follows the [File Browser spec](../Specs/File%20Browser.md), which settles that one server is browsed at a time — so the rail is what moves the user between servers, and an upload lands wherever they are standing.

**The rail cannot be the server list.** The [Servers spec](../Specs/Servers.md) puts each server's GPUs, its memory and its current work into the list, because that is where the user chooses which server a job runs on and the choice cannot be undone afterwards. None of that fits in a rail. So **All servers** is a screen of its own, reached from the top of the rail, and the rail carries names alone.

**Smoozy opens on All servers where no server has been added, and on Queue otherwise.** With no server there is nothing a queue could hold, and adding one is the first thing to do. Servers that are all disconnected still open on Queue. *Added 2026-10-03, the Creator's note 1 on the test copy: "opening the app for the first time or when there are no servers present should land the user in All Servers." Settled at Design by the build runner; no architecture answer touches it. No auditor has seen it.*

**The queue and history are two screens, not one.** They empty differently: the queue is blank on a fresh launch until a server is connected, while history is there straight away — see the [Queue & History spec](../Specs/Queue%20&%20History.md). One list would have to explain that difference every time.

## Panels over a server

**Two things open as a panel over a server rather than as a screen: a job's render settings, and a `.blend`'s dependency check.** Each covers the server's own screen, and each collapses to a strip along the bottom that keeps counting. Pressing the strip brings the panel back exactly as it was. *(Noted 2026-10-18, the twelfth notes, note 8: except where the other panel has since been opened on a different `.blend` — then the strip opens on that file, by the paragraph on moving both panels below.)* Both can be down at once, and they stack.

**While one panel is open, the other waits as its strip at the very bottom, under the open one; pressing it swaps them, and the one that was open becomes the strip.** A panel never opened over that server still has its strip while the other is open; it reads its name alone, and pressing it opens that panel on the open panel's file without starting anything. *(Changed 2026-10-10 on the Creator's seventh notes, note 2; it said a panel never opened over that server had no strip, and the head sentence said the other waited only where it was open over the same server too. No auditor has seen it.)* *Added 2026-10-08, Creator-stated, the fifth notes, note 3: "if dependency check is open, render settings should be this strip." Nothing had said what the other panel does while one is open. No auditor has seen it.*

**A strip carries no mark**: amber means work under way or something needing the user's eye, and a mark that is always there means neither. The strip's title and figures say what the panel holds. *Added 2026-10-17, the Creator's tenth notes, note 5: "Why does render settings ALWAYS have a yellow dot on it?" The drawing had put a mark on Render settings' strip alone, with no sentence giving it a meaning. No auditor has seen it.*

**The file's name in a panel's title is a link.** It collapses the panel and shows Files on that file's folder with the file chosen, so a user who has left that folder finds the file again. Its hover reads *Show in Files*. A file that has gone leaves its folder open with nothing chosen, as Files does for any row it cannot find. The strip is unchanged. *Added 2026-10-12, Creator-stated, the Creator's ninth notes (2026-10-11), note 5: "the filename at the top of both render settings and check dependencies should be a link that highlights the blend file in file browser and collapses either panel."*

**Opening Render Settings or the dependency check on a different `.blend` moves both panels to that file.** The other panel lets go of the file it was on and waits as its strip, reading its name alone, ready to open on the new file. Choosing a row in Files moves neither; only opening one does. Uploads the check had started carry on in Transfers. **Letting go loses nothing, exactly as Render Settings opened on a different `.blend` loses nothing**: the panel is a fresh page for the new file, and what was typed for the old one stays with that file — the [spec](../Specs/Render%20Settings.md)'s *"A `.blend` the user set up, left, and came back to still carries what they typed into it"*, which the [architecture](../Architecture/Render%20Settings.md) J6 keeps by keying the answers to the file rather than to the visit. Opening either panel on the old file again finds it as it was left. *Added 2026-10-18, Creator-stated, the twelfth notes, note 8; the last two sentences are the build runner's ruling of the same day. No auditor has seen it.*

**Collapsing is a chevron pinned to the panel's top corner, not a control in the header's flow.** A word for it spent a control's worth of width saying what the shape says; and laid out with the title it wrapped underneath on a narrow panel, which is the one place a way out must never be. So it is pinned, and the header wraps around it. Label: **Creator-stated** (2026-08-13).

**They are the same kind of thing on purpose.** Setting a render up and checking what a file is missing are one piece of work on one file, and the user moves between them constantly — the check is reached from the settings, and finding a texture missing sends them back. Two panels of the same kind over the same server makes that one gesture rather than navigation.

**Neither can block.** A dependency check reaches down another level every time a linked `.blend` arrives, and a search over a folder holding everything the user has ever made can run for minutes. Collapsing is what lets them go and do something else.

**A panel belongs to its server and does not follow the user.** Going to another server, or to the queue, leaves both panels where they were; coming back finds them unchanged. ~~Where a server has something of the user's open on it, its rail row says so. Label: **Creator-stated** (2026-08-10).~~ **STRUCK 2026-10-04 by the Creator's own word**, note 19 on the test copy — *"The yellow indicator next to the server — what does it even mean? I think it has to reflect what's happening with the server"*. The rail's dot now says what the server is doing (*The server badge*, below), and an open panel is said by its own strip along the bottom of that server's screen, which already keeps counting.

**A link from another screen that opens a server's Files — *Files on the server*, *Writing to* — collapses any panel open over that server to its strip, so the folder asked for is what shows; the strip brings the panel back as it was.** *Added 2026-10-07, the Creator's fourth notes, note 3: a *Writing to* press showed the Render Settings panel they had submitted from, still open over that server. A hole: nothing said what an open panel does when another screen sends the user to that server's Files. No auditor has seen it.*

## The server badge

**Every server has a colour, chosen when it is added, and it never changes.** It means one thing only: *which server*. It carries nothing about connection, work, or health.

It appears wherever that server does — the rail, the server list, the server's own screen, the queue's group headings, history rows, transfer rows. A job in history is placed by its colour before its name is read. Label: **Creator-stated** (2026-08-10).

**A badge is a squircle, and every state in Smoozy is a circle.** The connect dot, the status dot, the mark saying a server is not updating — all round. So shape says whether a colour is identity or state, before the colour itself is read. Label: **Creator-stated** (2026-08-11).

**That the two shapes can be told apart at the size a badge is drawn has not been checked.** Label: **an assumption** — see [Sources](Sources/Whole%20app.sources.md).

**That shape is what frees the palette.** The rule used to be that a badge could hold no colour carrying meaning, which left green, red and amber out and squeezed eight hues into one narrow arc of the wheel — near neighbours that could not be told apart at ten pixels, which is the one job a badge has. With shape carrying the difference, colour no longer has to, and **the eight sit 45 degrees apart around the whole wheel**: red, gold, green, jade, cyan, blue, violet, magenta. A green squircle cannot be misread as a green circle, because nothing else in Smoozy is a squircle.

**They are saturated, not muted**, for the same reason.

**And they are the same eight in light and dark.** Every other colour in Smoozy carries a value for each theme; these do not, because they mean *which server* and a hue that shifted between themes would be a second thing to learn about a mark whose whole job is to be recognised at ten pixels. [Design values](Design%20values.md) holds the ruling. Label: **Creator-stated** (2026-08-13).

**The colour arrives already picked and the user can change it.** Smoozy takes the first of the eight nobody is using; the user can override it with any of the eight, including one already in use. [Servers](Servers.md) settles how that reads in the Add dialog and why the pick is made for them.

## How state is shown

Because the badge is identity, state is carried elsewhere, and each kind of state has one home.

**Connection is the connect control itself.** It is filled green and tinted when connected, hollow when not, amber while Smoozy is repairing a dropped connection, and red and unavailable where the server is **not** connected and needs a new key — which is the [Servers spec](../Specs/Servers.md)'s rule that connect is withheld rather than offered and certain to fail. The control both shows the state and changes it.

**A server that is connected and needs a new key reads connected.** The spec allows both at once, and the control says what the connection is doing rather than what the next one will fail to do. The key is said elsewhere on the row — [Servers](Servers.md) settles where.

**In the rail, where there is no room for a control, a server that is not connected reads dim.** No colour, no second badge — the whole row dims, **badge included**.

**The badge has to dim, and it took the collapsed rail to show why.** The dimming used to be the label's alone, which works while there is a label; with the rail down to badges there is none, and a disconnected server read exactly like a connected one. Dimming identity is not the badge meaning something new — it is the row being dim, and the badge is now the whole row. Label: **Creator-stated** (2026-08-13).

~~**And the mark saying something of the user's is open sits on the badge there, not beside it.**~~ **The rail's one dot says what the server is doing. Changed 2026-10-04, Creator-stated** (their note 19 on the test copy, replacing their word of 2026-08-10 above): **green while connected and idle, amber and pulsing while a job renders on it, and none while it is not connected**, where the row dims as above. The pulse is the one a rendering job already has. The dot keeps the place the open-panel mark had: in a rail of names it is a dot at the end of the row; in a rail of badges there is no room for a second shape, and two blobs side by side pushed the badge off centre and read as one lopsided thing. So it moves onto the badge's top corner, with a ring of the rail's own ground keeping the two apart — a circle on a squircle, which is the shape rule already doing its job.

**Work is words, never a colour.** What a server is doing — rendering a named job, idle, nothing known — is written in the list beside it. **The rail's dot is the one exception, on the Creator's word of 2026-10-04**: the rail has no room for words, and its dot says idle, rendering, or reconnecting — the last drawn as the connect control's own reconnecting dot, its word the server list's own *Reconnecting…*; the server list still says it in words. *(Changed 2026-10-10 on the Creator's seventh notes, note 10; it said the dot said only idle or rendering, so a reconnecting server read as connected. No auditor has seen it.)*

**The rail's *Queue* has a dot of its own while any job in the queue reads *Needs your answer*.** One small dot just after the word *Queue*, in the amber those words are written in, and still. It goes the moment no job reads that. No count: the queue says how many. In a rail of icons it moves onto the Queue icon's top corner, with the same ring of the rail's ground a server's dot has there. Its name on hover becomes *Queue — needs your answer*. **It cannot be read as a server rendering**: a server's dot sits at the end of a server's row, in the lighter amber, and pulses; this one sits by the word *Queue*, in the darker amber, and never moves. It is the second exception to *work is words*, for the first one's reason: the rail has no room for words. **It can arrive late.** It comes from the same queue reading as the servers' dots, and that reading repeats only while Queue, the server list or a server's own page is on screen, so on History, Transfers, Keys or Settings a question can wait a while before the dot shows it. *Added 2026-10-31, the Creator's twenty-fourth notes, note 2: "Queue button in the left pane should visually indicate that something needs the user's attention." Settled at Design by the build runner; no Specs or Architecture sentence says what the rail shows. No auditor has seen it.*

**Anything Smoozy is showing from memory rather than from the server reads as such**: greyed, and saying when it was last seen. A job that was rendering when Smoozy last looked says plainly that it is **not updating**, because the readings that make a render look like it is still running would otherwise read as live. That is the [Queue & History spec](../Specs/Queue%20&%20History.md)'s requirement, and it holds anywhere a stale reading is shown.

## The file's value against the user's

**Every setting read from a `.blend` is shown in italic grey. Every value the user typed is upright and bold, and carries a small cross that hands it back.** One glance says which settings the user has taken over and which the file still owns.

That convention has to reach places italic cannot. A checkbox has no type to set, so **a View Layer's label takes the weight instead** — upright and bold where the user pinned it, italic where it is only a reading of one frame.

On a job's entry, where the settings are settled and read at a glance rather than worked in, the same convention holds and the file's own value is named beside the user's.

## Borrowed from Blender

Three things are taken from Blender's own visual vocabulary, and each means in Smoozy exactly what it means in Blender. They are borrowed for meaning, never for decoration. Label: **Creator-stated** (2026-08-10) — see [Sources](Sources/Whole%20app.sources.md).

- **The keyframe diamond.** A View Layer whose renderability is animated in the file carries one — solid where it is keyed on the frame being read, hollow where it is animated but not keyed there. This answers a problem the [Render Settings spec](../Specs/Render%20Settings.md) spends a section on: the checkbox shows one frame and the job does something else. An artist reads the diamond without being told.
- **The rejected-value field.** A number Smoozy will not accept turns the field red, exactly as Blender does. The note beside it says what would otherwise have happened — a resolution outside 1 to 32767 is silently corrected by Blender and charged for.
- **Linked-library blue.** A row in the dependency check that is itself another `.blend` is coloured, because those are the rows hiding an unknown amount beneath them.

**What is deliberately not borrowed is Blender's orange selection and its dark grey.** Those are Blender's identity rather than its vocabulary, and Smoozy is meant to stay out of the way. Smoozy uses no Blender logo or mark of any kind; the Blender Foundation was asked and declined. Label: **Creator-stated** (2026-08-10) — see [Sources](Sources/Whole%20app.sources.md).

## Colour

Colour is reserved and means the same thing everywhere.

- **Green** — connected, or a job that finished.
- **Amber** — work in progress, or something needing the user's eye.

**A colour that has to work as text and as a filled shape has two values, and that is a rule rather than an exception.** A status dot wants the traffic light's own yellow and text at that value cannot be read; a progress bar wants a light blue and a link at that value is too pale. So amber and the accent each carry a text value and a fill value. They mean the same thing — [Design values](Design%20values.md) holds both.
- **Red** — broken, refused, or destructive.
- **Blue** — the accent: a link, a chosen thing, whole-job progress, and a transfer's progress. A transfer is one reading of overall progress, so it takes the colour overall progress already has.
- **Grey** — nothing happening.
- **The eight badge hues** — which server, and nothing else.

**A value Smoozy cannot confirm carries no colour of its own.** It is shown by the italic face and the dash, which is the same convention that puts a setting read from the file in italic above. [Design values](Design%20values.md) holds the ruling.

**The frame in hand is yellow and the whole job is blue, and the blue is a light one.** The pair was tried in one colour family and the whole job came back to the accent: the two bars answer different questions, and a lighter blue is far enough from the yellow to be read at a glance without being the heavy navy the accent uses for text. **The same two colours are used wherever the pair appears** — the two-pixel bars on a queue row and the seven-pixel bars on the entry — so a row and its entry never disagree. Label: **Creator-stated** (2026-08-13).

A finished job's ending is coloured so it is read without being read: green finished, red failed, grey stopped by the user, amber abandoned. The fifth — an ending Smoozy never learned — is not an ending and takes no colour of its own; [Queue & History](Queue%20&%20History.md) settles how it reads.

## Light and dark

**Smoozy follows the operating system's setting and has no theme of its own.** Label: **Creator-stated** (2026-08-10). It is meant to sit beside Blender without competing with it, and a switch of its own would be one more thing to decide.

## Where a failure appears

The [Whole app spec](../Specs/Whole%20app.md) settles this and the design carries it: **a failure appears on the thing that failed and stays until the user is done with it**, never as a box in the middle of the screen. A server's failed connection sits on that server's row. A file that would not upload sits on that file's transfer.

**The one exception is the one the spec already names** — where the user asked for something and is standing there waiting on the answer. A delete confirmation, an already-exists question, a changed identity: the user cannot go on until they answer, so the answer is where they are. Everything else stays on its row.

**A failure is a plain sentence first**, with whatever the server or Blender actually said behind a link rather than in front of the user.

**A failure also goes when the condition it describes plainly ends**, and not only when the user clears it — the connection restored however it came back, the folder taking files again. The [spec](../Specs/Whole%20app.md) settles it, and the reason is a design one: leaving the message up would have Smoozy saying the connection is down on a server that reads as connected, which is the app being wrong on screen about its own state. **A job's recorded ending is not a condition and never clears this way.**

**And a failure goes when the thing it sits on is gone** — a server removed, a history entry deleted. That is the [spec](../Specs/Whole%20app.md)'s third way one ends, and it asks nothing of the user: there is nothing left for the message to attach to.

**A failure that will keep happening for the same reason is said once, not once per file.** The [spec](../Specs/Whole%20app.md) settles that too: a hundred files failing to land in one full folder is one fact, and Smoozy does not repeat it a hundred times. **Where the failures are rows rather than one message, it reads as one statement above the group**, with each row keeping its own state and its own *Retry* — so the fact is said once and the acts stay where they can be taken.

**One cause on a failed render is named rather than guessed at**, and the [spec](../Specs/Whole%20app.md)'s own carve-out holds the rule. Where that lands on a job's entry is [Queue & History](Queue%20&%20History.md)'s.

## The failure shape

**A failure has one shape wherever it attaches, and this is where it is stated.** A red dot, one plain sentence saying what did not happen, then — only where Smoozy actually knows — a second sentence saying what to do about it, then the controls: whatever act would try again, the raw text where there is any, and a cross that clears it. Nothing else, in that order. The thing it attaches to decides only how much room it has, never what it is made of.

**Where the surface is a single line of a list row, the shape collapses, and what survives is fixed in its order and in what may be added to it rather than fixed at a count.** The red dot, then the plain sentence, then the cross where the failure describes something still true — a job's recorded ending has no cross, so its collapsed row is two parts rather than three. The second sentence, the raw text and reporting are what a line has no room for, and they wait for the surface the row opens onto rather than being squeezed in — a row that grew to hold them would shift everything under it down the column. **And where the act that would try again is a control the row already carries, the failure does not repeat it**: the row's own control is the way back, and a second copy inside the failure would read as a different act.

**Two additions to the shape are permitted, and only two.**

**The first.** Where a failure sits in a box the user is standing in front of, the screen that owns the box may add its own way past — the act the user came to do, offered beside the retry, as [Servers](Servers.md) does when a tidy-up will not run and removing the server anyway stays available. A user who asked for something is not trapped by a failure in the middle of it.

**The second, from 2026-08-25, and it is the last.** **Report a problem…**, after the raw text and before the cross, quiet, on any failure that has room for it. The [spec](../Specs/Reporting%20a%20problem.md) requires it; *Reporting a problem* below settles why it is quiet and last, and why the collapsed row does not carry it at all.

**Those are the only two additions anywhere**, so *nothing else, in that order* holds everywhere else.

**The cross is only on a failure that describes something still true**, and this is the distinction the spec draws itself. A connection that is down and a folder that refused a write are live conditions: the user clears them, or they clear themselves when the condition ends. **A job's recorded ending is not a condition and carries no cross at all** — freeing the disk does not put back the frames that were never rendered, so a failed job stays failed and there is nothing to clear.

**The raw text is always one control away, and the control names whose text it is** — *Blender output*, *What the server said*. One panel serves every one of them, because they all show the same kind of thing: exactly what was printed, unchanged, with nothing read out of it for a cause. A render has two streams and they are kept apart, so it has two tabs; anything with one stream has none. **A tab may say how many lines its own stream holds.** Counting the lines in a stream that is already kept separate is not reading them for anything: the line is drawn at what is taken out of the text **as a cause**, and a count is not one.

**That panel never shows an empty box.** What a tab's record can be is four things and it says which: the text; *the render printed nothing on this stream*, in that stream's own words, and there is nothing further to read; *partly recorded*, with how much was kept and why the rest went; and *no output was recorded*. That is the architecture's closed set (*[Desktop app and delivery](../Architecture/Desktop%20app%20and%20delivery.md)* M10), one outcome per stream for each attempt at the job, and **none of the four is an empty box** — which is the clause's own reason for having four rather than three. The last two are different facts and a user who cannot tell them apart learns nothing from either — which is the whole reason the spec asks for the plain sentence to say it.

**There is a fifth sentence, and it is none of those four.** Where the render filled the server's own disk, the outcome for a stream can be written late, so there is a record that has been given no outcome yet. The tab says exactly that: *not recorded yet*. **It is never worded as *collected***, which would claim there is a record to open when there is not, and it is never worded as *no output was recorded*, which is a different fact resting on a different record. That is the architecture's (*Desktop app and delivery* M10a). Whether the unrecorded outcome reaches the user at all is also the architecture's (*[Rented-server runtime](../Architecture/Rented-server%20runtime.md)* S14), and the shape here holds either way, because every reading is a sentence and none of them is an empty box. Label: **settled at Design** (2026-08-23), the fourth outcome added 2026-09-02 and the fifth on the same date.

**Where a render's error stream came back empty, the control and the panel stay, and that stream's own tab says so in a sentence.** This document said until 2026-09-02 that such a render had printed nothing at all and carried no control and no panel, and both halves of that were wrong. The [spec](../Specs/Whole%20app.md) settles the ending it was written for — the machine's own memory running out, which kills Blender 4.5 and 5.0 — as one where the error stream is left empty *while the ordinary stream runs on to a sampling line and then stops*. So there are words, and they are on the other stream. And the architecture holds that an empty panel is never what the user meets: each of the outcomes above is a sentence, so no outcome is a reason to take the control away.

**The two tabs therefore take two different outcomes of the one set** (*Desktop app and delivery* M13a). **Errors** says *Blender wrote nothing here.* — the [spec](../Specs/Whole%20app.md)'s own claim, worded to the stream it is about. *(Reworded 2026-10-11, the Creator's eighth notes, note 6: it read *The render reported no errors. There is nothing further to read.* Blender 5 writes its errors to its ordinary output (`Desktop app and delivery` M13), so an empty **Errors** tab says only that the stream is empty. An empty **Output** tab likewise reads *The render printed no output.* alone, without the second sentence.)* **Output** holds the render's own lines, under its own name, ending at the sampling line the render stopped on. Each tab is read on its own, so the sentence in one says nothing about what is in the other.

**The render's own lines stay in their own tab, and nothing is taken out of them.** The architecture bounds it (*Desktop app and delivery* M13b): no line of the ordinary output is lifted out — no last line, no excerpt, no highlight — nothing of it is ordered or weighted, and nothing composed from it goes where the reason is said. Those lines are what the render printed before it stopped rather than why it stopped, and setting the last of them under the plain sentence would name a cause by placement that Smoozy does not have. The entry's own sentence names none and points the user at Blender's output, saying only that it may say why *(narrowed 2026-10-11, the Creator's eighth notes, note 6: it said none of the output says why the job ended, which Smoozy cannot know without reading it)*; [Queue & History](Queue%20&%20History.md) settles how that reads.

**What the old paragraph was protecting is exactly what the outcomes deliver.** A user must never be left working out whether Smoozy failed to collect the words or there were none — *the stream was empty* and *nothing was recorded* rest on two different records (*Desktop app and delivery* M11), and the panel says which one it is. Taking the control away avoided the empty box by removing the sentence that answers the question. Label: **settled at Design** (2026-09-02).

**It offers no way to save the text, and it carries Copy**: the copy icon in the words' bottom-right corner, shown only when the open tab holds printed words, which [Queue & History](Queue%20&%20History.md) settles under *The rest of the entry*. The text is selectable as well (*Selecting text*, above). *(Corrected 2026-10-24, the eighteenth notes, note 2, from the scout's finding. It read "**It offers no way to save or copy the text**, and that is deliberate rather than missed: no spec grants either, the text is selectable, and the menu bar already carries copy." The panel has carried Copy since the Creator's fourth notes, note 2. No auditor has seen it.)*

**The panel opens at the end.** While the reader is at the end, new lines keep it there; scrolling up more than a line lets go; scrolling back to the end holds it again. That holds for a finished job's panel too, where the end is where the render stopped. *Added 2026-10-03, the Creator's note 12 on the test copy: "the entire thing should automatically scroll to the end until the user deliberately scrolls up." Design conforming to Architecture under Rule 10; no architecture answer touches it. No auditor has seen it.*

**One place lifts lines from a render's output, and it is not here.** A rendering entry's *Blender output* group shows the ordinary output's newest three lines while the job renders ([Queue & History](Queue%20&%20History.md), *A rendering entry*) — `Desktop app and delivery` M13b's one narrowing, made because a rendering entry has no failure for the lines to stand beside. Everything above about a failure stands. *Added 2026-10-03; no auditor has seen it.*

## Reporting a problem

The [spec](../Specs/Reporting%20a%20problem.md) of 2026-08-25 gives the user a way to tell the Creator what went wrong: Smoozy writes a file, opens the folder it landed in, and names an address to attach it to. Smoozy sends nothing itself. What that looks like is settled here, and the spec's two markers naming this stage are closed by it. Label: **settled at Design** (2026-08-25).

### It is the second addition the failure shape permits, and now the last

*The failure shape* above permits two additions and no more, and this is the second — the spec required it rather than this stage choosing it. The first was spent on a screen offering the act the user came to do. *Nothing else, in that order* still holds for everything after these two.

**It goes last, and it is quiet.** The order on a failure is: whatever act would try again, the raw text, **Report a problem…**, the cross. The retry is what the user wants almost every time and it stays the loud one; reporting is the rare case and takes the same weight as the raw-text control beside it. **A loud control here would be actively wrong** — a connection dropping because the user shut their laptop is the commonest failure in the app and is nobody's bug, and a report invited on every one of them buries the ones that matter.

**On a failure collapsed onto a single line of a list, it does not appear at all.** Reporting is one of the parts a single line has no room for, and it waits for the surface the row opens onto exactly as the second sentence and the raw text do — the collapsed form is settled with the shape above. **This is the shape's existing rule reaching a new part, not a rule of its own.**

### The box, in two readings

**Asking.** The title says what is being done. One field for the user's own sentence, and it is the only thing they fill in — the spec makes it the whole point, since for a download that silently landed nowhere the user's sentence *is* the bug. One line says what Smoozy will put in the file, in the user's terms: its log, the servers' replies, and the names of files, folders, scenes, servers and users. **That line says what is in the file rather than showing it**, which is the Creator's ruling of 2026-08-25 — the user does not need to read it, and the file is plain text in a folder they chose, so anyone who wants to look already can. Then the folder it will be saved in, with a way to change it, and the loud control: **Save report**.

**Saved.** The folder has opened behind the box, so the file is in front of the user rather than somewhere they must go and find. The box says the file's name and the address to attach it to, and the address is the only thing on it worth reading twice, so it is the one thing drawn large. Then **Close**.

**Where it was opened from a failure, the box names that failure** in one line under the title, so the user does not have to describe what Smoozy already knows.

### After a crash

**Smoozy raises it at the next launch**, which the spec grants as a named exception to failures appearing on the thing that failed. It is a box in the middle of the screen and the user was not waiting on it, which is exactly what makes it an exception rather than a shape this stage invented.

**It says nothing was lost, and that is the most important line on it.** A crash and a clean close leave Smoozy in the same state — that is settled in the architecture and it is deliberate — so a prompt that only said *Smoozy did not close properly* would frighten a user about work that is perfectly safe. It says both: the run before this one ended unexpectedly, and nothing was lost.

**Two controls, and the quiet one is not a refusal.** **Report it…** opens the box above with the crash already named. **Not now** closes it.

**Nothing is offered that switches the prompt off for good, and nothing has to be.** Smoozy raises it for a crash it has not already raised it for, and it does not ask again about one the user has declined. So a version that crashes on every run puts the box up once rather than at every launch, and a genuinely new crash still speaks up. Label: **Creator-stated** (2026-09-02).

## Anything that isn't instant shows that it's underway

The [Whole app spec](../Specs/Whole%20app.md) makes this an app-wide promise like the failure rule above: whenever the user triggers something that takes a noticeable moment, Smoozy shows that it is happening rather than sitting there looking as though the click did nothing. It holds wherever it is true, not only where a screen's own document remembered to say so.

**There are three shapes, and what the user can still do decides which.** That is the whole rule, and it is meant to be applied without judgement.

- **On the control**, where the control the user pressed is the thing that is working and everything else on the screen still works. The control keeps its place and its width, loses its label to a turning mark, and cannot be pressed until it finishes. *Connect*, *Read again*, *Check dependencies…*. **It is not dimmed.** Not being pressable is carried by the control not answering; a fade on top of that says *unavailable* twice, and on a filled button it washes the turning mark out to almost nothing. Added 2026-09-02, after the drawing turned out to have two of these, one faded and one not.
- **On the row**, where the work belongs to one row of a list and the user will come back to it. A transfer, a queue entry, a setup step. The row says how far along in its own status column.
- **Over the pane**, where nothing on that pane can be used until the work finishes. A file listing being read, a dependency search running. Nothing else on the screen is covered.

**Where two look possible, the narrowest that is true wins**, because a shape that covers more than it has to takes away work the user could have been doing.

**One thing never carries two of them.** A server Smoozy is repairing already says so on its row, so its Connect control keeps its amber — which is state rather than an underway mark — and what it does not additionally do is lose its label or spin. That is a different rule from the one above: the narrowest true one settles which shape to use where more than one could apply, and this one stops a single thing wearing two at once.

**And where the control that started the work is gone by the time the work runs, the reading it produced carries the mark itself.** *Check for updates…* is a menu item, and the menu closes when it is pressed, so there is no control left to lose its label; the line in About that will answer it takes the underway mark instead. Label: **settled at Design** (2026-08-24).

**Anything that shows it is underway says how far along where it can, and a shape that never becomes a figure is the app saying it cannot.** That shape is itself the admission and no words are owed anywhere for it — a control that has lost its label has nowhere to put any, and *Connect* is a case where nothing could say how far along it is. What the rule forbids is showing nothing at all while work is happening. Label: **settled at Design** (2026-08-23).

## Do what can be done, and say what can't

Where Smoozy can carry out part of what the user asked but not all of it, it does the part it can and says plainly what it could not. It does not abandon the whole action because one piece failed, and it does not stay quiet about the piece that did. That is the [Whole app spec](../Specs/Whole%20app.md)'s, and it is written here because [Settings](Settings.md) and [File Browser](File%20Browser.md) both already lean on it as a rule this document holds.

## Being asked for a password

Some acts on a server are refused because the login Smoozy connected with is not permitted to carry them out. **There is one dialog for all of them, and it names the step that was refused** rather than asking for a password out of nowhere. A password box arriving with no reason given is a thing a user is right to refuse, so the reason comes first.

**The choices about keeping the password sit under the field**, because that is the moment the user is deciding it. What those choices are, and why, is the [Whole app spec](../Specs/Whole%20app.md)'s under *Security*; what is settled here is only that they are in this dialog and that they sit under the field.

**Where the computer has nowhere to keep a password, the choice to remember it for good is absent rather than greyed — and it is the only one that goes.** The [architecture](../Architecture/Security%20and%20licence.md) composes the offer from what the computer can do: two choices where it has nowhere to keep one, three otherwise. Asking each time and keeping it while this connection lasts both still work — the second needs no store at all — so both stay. A greyed choice reads as something the user could turn on if they worked out how; an absent one says the computer has no place for it.

**The dialog asks again only where nothing was remembered.** Where a password was remembered and the system would not hand it back — locked, refused, or the store cannot be reached — Smoozy does not ask for it and does not forget it: it reports that it could not reach what it remembered, and offers the action again. That is the architecture's rule (*[Desktop app and delivery](../Architecture/Desktop%20app%20and%20delivery.md)* S2), and the reason is on its face: asking would have the user type a secret the computer still holds.

**A system's own question is the system's**, and Smoozy draws none of it — not the box, not the field, not the wording. What Smoozy draws is what it needs the answer for, and what it does with the answer afterwards.

## Words

**Everything below sits under [Rule 9](../Groundwork/Rules.md), which binds every word the user reads: use the standard software term, never a metaphor and never a phrase that sounds warm.** *Downloaded*, not *came home*. *First frame rendered*, not *first frame is back*. The rules here are the project's own additions on top of it.

**Three of them, and they are absolute rather than preferred.** Label: **Creator-stated** (2026-08-10).

**Moving a file is *download* or *upload*, and nothing else.** No file comes home, comes down, goes up, is fetched, or arrives. Where a sentence seems to need a different verb, the sentence is doing too much rather than the rule needing an exception.

**The rented computer is a *server*. The user's own is *your computer*.** Never *machine*, whichever of the two is meant — and that is the reason: a word that can mean either end of a transfer is the last word an interface should use.

**Nothing invented for this project's own use ever reaches a screen.** *Renderlet* is the example: it exists so that agents writing these documents agree on what a piece of work is, and no Blender user has ever heard it. A progress reading says a percentage and, where the count is not simply frames, says why in the user's own terms.

**A heading names the thing, and is never a sentence about it.** *Notifications*, not *What Smoozy tells you while you are working elsewhere*. *Download folder*, not *Where downloads land*. A heading is scanned rather than read, and a sentence in that position has to be read before it can be skipped. Label: **Creator-stated** (2026-08-11).

**A box's own title is the one exception, and it may be a sentence.** *Smoozy 2.5.0 is ready to install*, *This step needs admin access*. A box is read rather than scanned — the user is standing in front of it and has nothing else to do until they answer — so its title says what is being asked. Section headings and field labels take the rule as stated. Label: **settled at Design** (2026-08-24).

**Where a piece of text explains rather than informs, it is cut.** A line earns its place if it changes what the user would do. Reassurance, restating what is already on screen, and explaining Smoozy's reasoning all fail that test.

## Typography

**A non-breaking space goes before every em dash**, so a dash never begins a line.

**No orphans**: the last two words of any piece of prose are bound together, so a sentence never ends with one word alone on a line.

Both hold everywhere the user reads, tooltips and button labels included. Label: **Creator-stated** (2026-08-10).

**Neither can be held by remembering it.** Nobody counts dashes while designing a screen, and both rules have already been broken by the person applying them. So they are applied by whatever draws the text rather than by hand, at one point every piece of text passes through on its way to a surface — which is what makes them hold in tooltips, button labels and menu items that no layout engine draws. How that point is built is the [architecture](../Architecture/Desktop%20app%20and%20delivery.md)'s to arrange; what is settled here is that it must be. Label: **settled at Architecture** (2026-08-23).

**A message of more than one sentence is handed over one sentence at a time, and that is the screen author's own job.** The point binds the last two words of whatever it is given and does not look for sentence ends inside it, so several sentences given together get the orphan rule once, at the end of the last one. A failure's message running to two sentences is the ordinary case, and it wants the rule on each. It is left to whoever writes the screen deliberately: a point that hunted for sentence ends would bind the wrong two words wherever it guessed wrong.

**Both rules are for prose alone.** The server's own words, paths, file names, hosts, ports and anything the user can copy reach the screen untouched, because a non-breaking space inside a path is a path that does not exist. So *everywhere the user reads*, above, means everywhere the user reads prose.

**A number and its unit are never split across two lines.** In the Creator's words: *"Every numeric value followed by a unit should be connected with a non-breaking space. So, 47 GB should never become: 47 / GB."* Label: **Creator-stated** (2026-09-28). **This one is not for prose alone**: it holds for every figure Smoozy writes with a unit after it — a size, a speed, a length of time, a price, and a count with what it counts, *240 frames*, *5 files* — in a sentence, a column, a button, a tooltip or a notification.

**It is held where the figure is written, in one place, and not by the typography point.** The same reason as the two rules above: nobody counts units while designing a screen. The [architecture](../Architecture/Desktop%20app%20and%20delivery.md) gives the point two substitutions and nothing else (X2), and a figure reaches the screen as a value by a route that substitutes nothing (X9), so the space is a non-breaking one from the moment the figure and its unit are put together. **The server's own words, paths and file names still reach the screen untouched** (X7), even where a figure and a unit sit inside them. *Added 2026-09-28, watching step 10c's tenth pass. Design conforming to Architecture under Rule 10.*

## Type and spacing

**Smoozy uses the operating system's own interface font.** A desktop app that brings its own type reads as a web page. Numbers that line up in columns — sizes, frame counts, memory, hosts and ports — are set in the system's fixed-width face so they can be compared down a column.

**A field is the width of what it holds.** A number field is short, a path field is long. This is not tidiness: a control that hands a setting back sits immediately beside its value, and a field spanning the window puts that control a mouse-drag away.

## About

**About shows the version, the copyright notice and these words:** *Smoozy is free software. You may share and modify it under the GNU General Public License, version 3 or later. Smoozy comes with no warranty.* The copyright name is the Creator’s, as on the website’s About page.

**Licence** opens the full licence inside Smoozy from the copy it carries; **Third-party licences** opens the bundled notices inside Smoozy. Both work without a connection. **Source code** opens the public source repository through the fixed-address opener. **Donate** appears only after the Creator chooses an address; it opens that fixed address. There is no donation prompt anywhere. The version and update readings stay in this same About box.

*Build detail, 2026-10-27, wave 4; no auditor has seen it.* The copyright line reads *Copyright © 2026 Alexander Kislyakov*. The three links sit below the notice. Licence and Third-party licences show their complete, selectable text in a scrolling view in the same About box; Close returns to About, and Escape does the same. Opening Source code keeps About open; if the system refuses the request, About says *Could not open the source code page.* in the ordinary failure shape. No Donate link is shown without an address. Third-party notices use the headings *Packages*, *Declared licence*, *Source*, *Repository* and *Authors*, followed by package values and complete licence texts; these legal texts remain verbatim.

*Added 2026-10-26, carrying the Creator’s decision of 2026-10-04 through Specs and Architecture. The drawing landed on 2026-10-27; no auditor has seen this text.*

## Checking for updates

**The Smoozy menu carries *Check for updates*, and it stays.** Label: **Creator-stated** (2026-08-23). It was taken out earlier the same day on the ground that no spec grants the user an update action, and the Creator put it back. **The spec does grant it**: [Whole app](../Specs/Whole%20app.md), under *Getting the app, and updates*, says the user may also ask for a check themselves — the same act reached a second way rather than a thing they have to do, with nothing promised differently if they never use it. It blocks nothing and it contradicts nothing.

**It does its work in About, and there is no second box.** About is where the version already lives and where its update check belongs, so putting the check anywhere else would give one subject two homes. Five readings, one box: the version and when it was last checked; *Checking…* while it runs; a new version found, which on Mac and Windows says *Smoozy will restart to install it.* and on Linux leads to the update box; a check that could not be reached; and a found update that could not be installed. Both failures take the ordinary failure shape and name no cause.

**While jobs are running, a new version waits, and About is where the user can install it anyway.** The third reading then takes a second form: *Smoozy 2.5.0 is ready. Installs when no jobs or transfers are running.* It carries **Install now**, and above the button, read before it is pressed, one sentence says what that does: *Installing now restarts Smoozy. Jobs keep rendering on their servers. Uploads and downloads in progress stop.* On Linux the button reads *Install now…* and leads to the update box, which is where the password is explained. **It is a form of the third reading, as the Linux form is, and not a sixth.** Pressing it installs that one version; nothing is remembered, and the next version waits the same way. Label: **Creator-stated** (2026-09-28): *"The user should be able to override this block, but by default — updating only happens with no running jobs."* *Added 2026-09-28. Design conforming to Architecture under Rule 10: `Desktop app and delivery` Z31 and Z32.*

**About is also the only thing in the app that names the version at all**, which matters because Smoozy updates itself and says nothing when it does.

## Notifications

**A notification says the thing and stops, and clicking one opens what it is about.** The system draws the notification; what is settled here is only those two.

**There are four kinds of notification, and [Settings](Settings.md) has four switches — one per kind.** The [Whole app spec](../Specs/Whole%20app.md) names the four. A batch is not a fifth: it is the shape the finished-job kind takes when the jobs were submitted together, so a user who silences finished jobs silences batches with them.

- **A job finished** — *Kitchen — final finished* / *240 frames in 6 hours 2 minutes · RunPod 4090*. Two figures, because those are the two a user wants before deciding whether to go and look. The frame comes with it where the job has one and where the system takes it. **Where a batch of scenes was submitted together, this kind speaks once for the whole batch** — *Product spins finished* / *4 scenes, 480 frames in 3 hours 14 minutes · Vast 5060* — and the scene count is what tells it from a single job.
- **A job failed or was abandoned** — *Kitchen — final v3 failed* / *at frame 61 of 240 · RunPod 4090*, and *Kitchen — clay abandoned* / *after two interruptions, at frame 132 of 240 · Old 4080*. **It names where it stopped and nothing about why.** What Smoozy knows about the cause is on the entry, behind the entry's own control, and a notification is not a place to start reading.
- **A connection could not be restored** — *Old 4080 disconnected* / *Could not reconnect.* No cause named, because three quite different things look identical from here.
- **An idle server** — *Vast 5060 is idle* / *No jobs for 30 minutes.* This one is about money, so it says how long rather than only that it happened.

**Clicking brings Smoozy forward and goes to the thing it is about** — a job's entry, a server's row. It goes where the user would have had to go anyway, so no notification is a dead end. **None of them says what to do next**, which is the failure rule already holding here: Smoozy says what to do only where it knows. Label: **settled at Design** (2026-08-23).

## Updating

**Smoozy keeps itself up to date, and what the user meets while it does differs by system.**

**On Mac and Windows, nothing.** Smoozy fetches and installs, then restarts itself with the new version (`Desktop app and delivery` Z21). Nothing is announced, nothing is asked, and there is no list of what changed — the spec's promise is that updating is not something the user has to think about, and a message about it is thinking about it. **Smoozy announces nothing there, and whether either operating system puts up something of its own on a version the app fetched and started is not established** — the [spec](../Specs/Whole%20app.md) leaves that open.

**About says what happens.** After a new version is found on Mac or Windows, the words after *Smoozy 2.5.0 is ready.* are *Smoozy will restart to install it.* If a found update cannot be installed, About says *Update installation failed.* in the ordinary failure shape; *Could not reach the update service.* is reserved for a check that could not be reached. *Changed 2026-10-25, step 11 D5b, Rules 9 and 10: the words follow `Desktop app and delivery` Z21’s restart and distinguish installation from reaching the service. No auditor has seen it.*

**On Linux, one box, once per version, and it is there because of what comes after it.** A new version arrives as a package and installing one needs the computer password. The system draws that request, not Smoozy — but a password box arriving out of nowhere is a thing a user is right to refuse, so Smoozy says what it is for first: *Smoozy 2.5.0 is ready to install*, *Your system asks for your computer password to install.*, and that *Not now* keeps this version until the next release. **Not now and Install, and Not now is not a failure** — the spec says nothing nags them, so nothing asks again until the next version. *Words changed 2026-11-01 (was *that the password is the system's own request and Smoozy never sees it*; the screen read *Your system will ask for your computer password when you install; Smoozy never sees it.*): Creator-stated, the second words sweep. No auditor has seen it.*

**A failed install is not a decline.** About says *Update installation failed.*, and the box may be offered again for the same version. *Added 2026-10-26, step 11 D6b, on the build runner's ruling under the Creator's standing delegation: only Not now — or Escape, or quitting with the box open, or declining the system's request — keeps the box away until the next version. No auditor has seen it.*

**On Linux, a new version that does not start is said, and so is the way back.** Smoozy puts the previous version back by itself (`Desktop app and delivery` Z21). Where the system will not let it do that without asking, Smoozy opens, with nothing else in the window, one box: *Smoozy 2.5.0 could not start*; *Your system asks for your computer password to reinstall.*; *Reinstall the previous version to open Smoozy.* **Quit and Reinstall**, and the system asks only once Reinstall is pressed, the box being where the password is explained. Reinstall takes the underway shape while it runs. **If the request is declined or the reinstall fails, the same box says *Reinstall failed.* in the failure shape**, and Reinstall is the way to try again; Quit leaves it for the next start, which opens the same box. Where only the last of the work is left and no password is needed, the password sentence is not shown. On success the previous version opens and nothing more is said. *Added 2026-10-26, step 11 D6b: Smoozy went quiet at both moments — it disappeared when the rollback could not finish, and at the next start it asked the system with nothing on screen and closed again with nothing on screen if the request was declined (Rule 8). Rules 9 and 10: the box takes the moment `Desktop app and delivery`'s D6b line gives it. No auditor has seen it.* *Words changed 2026-11-01 (was *Your system will ask for your computer password when you reinstall; Smoozy never sees it.*): Creator-stated, the second words sweep. No auditor has seen it.*

**On every system, nothing is installed while a job is running or a transfer is under way.** A job waiting in line or waiting on the user's answer counts. The new version waits and goes in by itself once nothing is running, and nothing is announced about the wait; on Linux the box is not offered until then. About is the one place the wait shows, and where the user can install anyway (*Checking for updates*, above). *Added 2026-09-28, on the Creator's ruling of that day. Design conforming to Architecture under Rule 10: `Desktop app and delivery` Z31.*

**Smoozy on each server follows the app, and waits the same way.** Nothing is announced about it. The server's row is the one place it shows — while it waits for jobs, while it runs, if it fails, and where a server is too old or too new to work with — and where the user can have it done anyway ([Servers](Servers.md), *A row*). *Added 2026-09-28, on the same ruling carried to servers. Design conforming to Architecture under Rule 10: `Desktop app and delivery` V11 to V15.*

**The only lasting trace of any of it is About**, which names the version and when an update check last completed, as a moment and never as an age. Label: **settled at Design** (2026-08-23). *(Corrected 2026-10-24 by step 11's plan pass, Design conforming to Architecture under Rule 10. It read ~~which names the version and when it was installed~~. No value records an install moment — `State, identity and recovery` I4 holds the last check's moment and the declined version, nothing else — and `Desktop app and delivery` Z23a has About show a moment, never an age.)*

## What this document does not settle

Each screen's own document settles its layout, its rows and its wording. This one settles only what they share. The screens are: [Servers](Servers.md), [File Browser](File%20Browser.md), [Render Settings](Render%20Settings.md), [Dependency Upload & Remap](Dependency%20Upload%20&%20Remap.md), [Queue & History](Queue%20&%20History.md), [Transfers](Transfers.md), [Server Setup](Server%20Setup.md), [Keys](Keys.md) and [Settings](Settings.md).

**Rendering has no document of its own, and that is deliberate.** It owns no screen: submitting sits at the bottom of [Render Settings](Render%20Settings.md), and its other two actions — submitting a job again, and carrying one on from where it stopped — sit on an entry in [Queue & History](Queue%20&%20History.md). So its Design lands in those two documents rather than in a third that would describe no place. What it is owed is tracked in whichever of the two carries it.

**A switch and a checkbox are different things, and the difference is kept.** A switch turns one thing on or off; a checkbox picks out of a set. [Settings](Settings.md) is where switches first appear, and the rule holds everywhere afterwards.

**Rendering and Telegram are drawn rather than written, and have no documents of their own.** Their screens, words and reasoning live in [the mockup](Mockup/Smoozy.html), on the Creator’s ruling of 2026-08-23.

**Rendering's own paragraph above still holds and is not an exception to this**: it owns no screen, so what is drawn for it sits on the settings panel and on a queue entry, not on a screen of its own.
