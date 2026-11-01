# Settings

*What Settings holds, why those four things and not others, and how each is shown. The conventions it follows are the [whole-app Design](Whole%20app.md)’s.*

*2026-10-27 — Removed the withdrawn paid-access text and its obsolete references under the Creator’s free-and-open-source decision of 2026-10-04 (wave 5); retained protections and evidence keep their force. No auditor has seen this removal.*

## Where it sits

**A screen at the bottom of the rail, below Keys.** It is also reached from the menu bar, which the [whole-app Design](Whole%20app.md) settles, because that is where a desktop user reaches for it.

**Connections.** It holds pieces belonging to four specs: the search folders from [Dependency Upload & Remap](../Specs/Dependency%20Upload%20&%20Remap.md), where downloads land from [Download](../Specs/Download.md), the speed caps and the notification switches from [Whole app](../Specs/Whole%20app.md), and the switch and bot from [Telegram](../Specs/Telegram.md). The [Transfers](Transfers.md) screen links here for the caps. The [Dependency Upload & Remap](Dependency%20Upload%20&%20Remap.md) panel does not link here at all: a folder is added there without leaving the check, and the list it joins is this one.

## What is in it, and what is not

**Four sections, listed down its own side: Transfers, Search folders, Warnings and notifications, Telegram.** One section shows at a time. The list is the only navigation inside the screen; nothing here scrolls past its neighbours.

**Everything in Settings belongs either to the user's own computer or to the whole app.** That is the test for what may live here, and it decides the awkward cases on its own. A remembered password belongs to one server, so it is cleared where that server is edited — which the [Whole app spec](../Specs/Whole%20app.md) already settles. A key belongs to Keys, which is its own place in the rail. Neither is here, and neither is an oversight.

**There is no General section**, because a section holding whatever did not fit is an admission that the other four were named badly.

## Transfers

**The download folder, then the two caps.** They sit together because both are about moving files, and the [Transfers](Transfers.md) screen points here for the caps.

**The folder is shown as a path with one control beside it**, and one line says the thing a user would otherwise get wrong: each download can use a different folder. That is the [Download spec](../Specs/Download.md)'s default-folder rule, and it is the only part of it a user could be surprised by.

**Each cap has what is actually running beside it.** The [Whole app spec](../Specs/Whole%20app.md) requires the reading and leaves its place to Design; here is its place, because a cap can only be judged against a real number. An empty field is no cap, which is also the default.

**One line says the caps are totals** — across every transfer and every server at once, not a limit applied to each one. That is the sentence the spec spends a paragraph on, and it is the difference between the caps and the transfer slots the user does not set.

**A cap takes effect on transfers already running, and the screen does not have to say so.** A change is picked up by everything in flight rather than only by the next transfer to start, so nothing on this section warns the user that a new cap waits. Label: **settled at Architecture** (2026-08-23).

## Search folders

**A numbered list the user orders, with what the order does and does not decide said underneath.** The [Dependency Upload & Remap spec](../Specs/Dependency%20Upload%20&%20Remap.md) is exact about this and the honesty is the point: the first folder with a match is used, so the order decides **which folder** a file comes from, never which file inside it. A user who thinks ordering fixes duplicates will be wrong at the worst moment.

**A folder Smoozy could not search stays on the list, marked, and the rest are searched anyway.** Removing it silently would lose the user's own choice; failing the whole search would break the whole-app rule that Smoozy does the part it can and says what it could not.

**The mark is one word — *could not be read*.** The reading behind it cannot tell a folder that is gone from one that moved from one that refused, so a word claiming the folder is absent claims more than Smoozy knows. **And the mark is the last search's, not the row's**: nothing reads a folder because this screen was opened, so a row carries the mark only where a search in this run met that folder, and the row shows when that search ran. **That is why the row carries no cross** — it is a record of what happened rather than a live condition, and the whole-app failure shape gives a record no cross. **This list row owns the state**, and the [Dependency Upload & Remap](Dependency%20Upload%20&%20Remap.md) panel names it in these words rather than its own. Label: **settled at Architecture** (2026-08-21), under the [architecture](../Architecture/Dependency%20Upload%20&%20Remap.md) F1a and F7.

**Each row carries a Remove control, and removing a marked folder is no different from removing any other.** The list is the user's own, so the row comes off and Smoozy says nothing further about the search that will now miss it.

**One line says subfolders are included**, which is what makes a five-row list enough and is the reason a user points at their work folder rather than at each project.

## Warnings and notifications

**The four notification kinds and the one control that restores warnings sit in one section**, because they are the same family: things Smoozy says to the user. They are not the same mechanism, and the section does not pretend otherwise — one is the operating system's, the other is in the app.

**Four switches, one per kind**, exactly the four the [Whole app spec](../Specs/Whole%20app.md) names. The finished-job switch says the frame comes **where the job has one**, since that is the one whose content is worth knowing before switching it off — and the qualifier is not decoration. The picture is absent four ways that are none of them faults: too large for the system to attach, dropped by a system that gives no signal that it dropped it, refused by a system whose own documentation contradicts itself, or not yet on this computer at the moment of posting, where the notification goes without it rather than waiting. **A flat claim would be wrong in ordinary use rather than in a rare one.** Label: **settled at Design** (2026-08-23), under [Rule 10](../Groundwork/Rules.md) and `Preparation/Architecture/Desktop app and delivery.md` T17a.

**One line says these reach the user only while Smoozy is open, and points at Telegram.** That is the gap the spec names, and it is the only place in the app where the user is standing in front of the fix.

**The restore control says how many are switched off, and says there is no way back one at a time.** The spec makes it all at once; saying so stops a user hunting for a list that does not exist.

## Telegram

**This section is the app's half of Telegram and no more**: the switch, the bot, the token, and what a job reports by default. What the conversation on the phone looks like is [Telegram](../Specs/Telegram.md)'s own ground and is not designed yet.

**Smoozy cannot make the bot, so the dialog says so and hands over the three steps.** Telegram only lets a person make one, from Telegram. Pretending otherwise would leave the user waiting for something that is never going to happen.

**The dialog starts with *Create a new bot* and *Use an existing bot*, equally visible, and nothing below them until one is picked.** No token field or Check bot button shows before that choice. Keep the computer instruction above it. The chosen route stays marked; choosing the other route replaces its instructions.

**Create a new bot shows only creation’s three steps:** open @BotFather and send /newbot; copy the two proposed names; paste the reply containing the token. Each name has the output panel’s copy icon directly beside it, labelled **Copy name** and **Copy username** on hover and focus; *Suggest another* follows the username’s icon. Say that the token lets Smoozy use the bot, then offer **Check bot**.

**Use an existing bot says why a new token is needed, asks for it, and states what it costs — and asks nothing else.** The reason comes first, in one line: *Your bot needs a new token, one no server is using.* Then two steps: *In Telegram, open @BotFather and revoke your bot’s token. You get a new one.* and *Paste the new token here.*, the field under it with *Nothing is saved until the bot is checked.* Below the steps, as a warning: *Servers on the old token stop reporting until you next connect to them.* Then **Check bot**, available at once. There is no tick-box: revoking is what makes the token one no server is using, which is Architecture S5’s boundary, and S6a asks for nothing more. The token-not-accepted and could-not-reach failures keep this route and its paste. While checking, the chosen route and paste cannot change.

**After the check, the returned bot identity decides what follows.** When this computer already holds proof for the same bot, the new token keeps that proof (P4): the result says *@<username> is already set up. No Start needed.* and *Servers get the new token when you next connect to them.*, then offers **Save token**, which delivers on the next connection, with no Start and no desktop listening. Without that proof, use the new token for check then Start; never receive on an old shared token. Nothing keeps a token through off/on.

*Drawn again 2026-10-24, Creator-stated on the first redraw: “Yeah, the bot menu is still not perfect.” The existing-bot route asked the user to replace their bot’s token and then to tick **No server has this token.** — a box that confirmed what replacing had just made true, and read as a riddle. The tick is gone, at Architecture S6a’s own clause, flagged there for the Creator; the route now says why in one line and names the cost as servers waiting for Smoozy’s next connection rather than a reconnection the user must do. The proposed names take the copy icon the Start step uses. No auditor has seen it. The four paragraphs replaced were:*

> **The dialog starts with *Make a new bot* and *Use an existing bot*, equally visible, and nothing below them until one is picked.** No token field, checkbox or Check bot button shows before that choice. Keep the computer instruction above it. The chosen route stays marked; choosing the other route replaces its instructions and clears the unshared-token confirmation.
>
> **Make a new bot shows only creation’s three steps:** open @BotFather and send /newbot; copy the two proposed names; paste the reply containing the token. Say that the token lets Smoozy use the bot, then offer **Check bot**. No checkbox belongs on this route: a newly made bot has not been placed on a server.
>
> **Use an existing bot shows the replacement instructions, the paste, then one confirmation.** Say *In @BotFather, replace this bot’s token before pasting it here. Servers using the old token cannot report until you reconnect them.* Label the field **Replacement token**. Below it, the checkbox says **No server has this token.** Its explanation is *This lets Smoozy receive your Start message.* This is the user’s confirmation of Architecture S5’s boundary, never proof read from Telegram. **Check bot** is unavailable until it is ticked; the same token-not-accepted and could-not-reach failures keep this route and its paste. While checking, the chosen route and paste cannot change. A retry uses the same check and confirmation. Editing the replacement token clears the confirmation and disables both Check bot and Try again until it is ticked again.
>
> **After the check, the returned bot identity decides what follows.** When this computer already holds proof for the same bot, a replacement token keeps that proof (P4): show the checked identity and **Save token**, then deliver on the next connection, with no Start, checkbox or desktop listening on that result step. Without that proof, use the fresh unshared token for check then Start; never receive on an old shared token. Nothing keeps a token through off/on.

*Redrawn 2026-10-23, Creator-stated: “why is there a token input field and the checkbox? What do they do before I even select an option? Has to be reworked.” The runner requires the choice alone first. The checkbox remains only in the existing-bot instructions, before the check as Architecture S6a requires; S5’s listening boundary and the same-bot proof shortcut stand. The paragraph replaced was:*

> **The dialog first offers *Create a bot* and *Use an existing bot*, equally visible.** Creation keeps the two proposed names, paste, check, link and Start. Existing-bot setup names the S5 boundary: *If this bot is used by a server, replace its token in @BotFather first. Those servers cannot report until you reconnect them.* Ask the user to confirm this token has not been given to a server before enabling the check-and-Start route. When the desktop already holds proof for the same bot, a replacement token keeps that proof (P4): check, save, then deliver on the next connection, with no Start and no desktop listening. On another computer or after switching off erased the proof, use the fresh replacement token for check then Start, never receive on the old shared token. The check’s returned bot identity decides which route applies; an unproved or unknown token is never silently assumed safe. Nothing keeps a token through off/on.

*Changed 2026-10-23 for fifteenth note 1, Creator-stated, 2026-10-21: “What if I, say, installed Smoozy on a different computer and just want to continue using my old bot? Well that doesn’t seem to be an option right now. Needs fixing.” The runner repairs the guidance within Architecture S5, rather than retaining an erased token. The former paragraph was:*

> **The dialog is a walkthrough, and it is drawn as one — the steps in order, with the parts Smoozy can do already done.** The [architecture](../Architecture/Telegram.md) S1 to S6 fix the order and this document does not re-decide it: two names ready to copy, the paste, the check that the bot answers, the link to the chat, then Start.

**Only after *Create a new bot* is picked, Smoozy offers two names because Telegram needs two**, and both sit ready to copy with the control beside each. Only the username has to be unique, so it is the one that can be taken and the only one with *Suggest another* beside it — Smoozy is only generating candidates and the user carries them over, so nothing here waits on Telegram to answer.

*Qualified 2026-10-23 for the Creator’s redraw above. Former opening: “Smoozy offers two names because Telegram needs two”. The names are absent before the choice and on the existing-bot route.*

**Each chosen route shows its own token field, and Smoozy takes the token out of whatever lands there.** The [architecture](../Architecture/Telegram.md) S2 finds it inside a longer paste as insurance rather than as the ordinary case, so creation asks for Telegram’s whole reply rather than for a token by shape; the existing-bot route asks for the new token. *(This read* the replacement token *until 2026-10-24, changed with the existing-bot route above.)*

*Changed 2026-10-23 for the Creator’s redraw above. Former paragraph:*

> **The token is one field the user pastes into, and Smoozy takes the token out of whatever lands there.** The [architecture](../Architecture/Telegram.md) S2 finds it inside a longer paste as insurance rather than as the ordinary case, so the step asks for Telegram's whole reply rather than for a token by shape.

**The check that the bot answers is its own step and its failure has two readings, not one.** The [architecture](../Architecture/Telegram.md) S3 separates them and the wording keeps them apart: **Telegram did not accept that token** sends the user back to the paste, while **Could not reach Telegram** is an ordinary failed action with the paste kept and the check offered again. Reading the second as the first would send a user to re-copy a token that was right.

**Nothing is stored until the check passes**, which the [architecture](../Architecture/Telegram.md) S4 requires, and the dialog says so at the paste — so a user who abandons it half-way knows nothing was kept.

**Where a new conversation proof is needed, then the link, and Start is a step rather than a courtesy.** The [architecture](../Architecture/Telegram.md) S6 needs the user's Start before the conversation can be proved, so the dialog says plainly that Smoozy is waiting for it and shows the reply when it arrives. A dialog that closed before Start would leave a bot nothing can send to. *Qualified 2026-10-23 for the existing-bot route above; its former opening was “Then the link, and Start is a step rather than a courtesy.” A same-bot replacement with standing proof skips this step.*

**On entering Start, copy only the checked bot’s username once, without `t.me/` or `@`.** Show the bot once as the link **t.me/<username>**, with the output panel’s copy icon directly beside it. No second name or visible Copy word appears. Its accessible label and hover/focus hint say **Copy bot name**. *Bot name copied. Paste it into Telegram’s search.* confirms success and says what the copy is for. *(This read* Bot name copied *until 2026-10-24: a person new to Smoozy met a copy with no reason given. The Creator’s own reason for it, fifteenth note 3: “the last thing the user has in the clipboard in that scenario is the token, while Smoozy is asking to find the bot in Telegram by name”.)* A failed clipboard write says *Could not copy the bot name. Use the copy icon.* in place, without blocking Start; pressing the icon retries and updates that same line. Refreshing or revisiting the same Start step does not copy again; a newly checked bot does. The returned username is the only source, never a proposed name. The drawing explains this action but never writes to the real clipboard.

*Redrawn 2026-10-23, Creator-stated: “the copy button should be right next to the t.me/botname and only copy the botname, without the t.me part”; “the botname without t.me gets copied into clipboard right away when this window appears”; “we can use a copy icon instead of the word Copy for this button.” The bare username replaces the former @username; the link is its only display. Former paragraph:*

> **On entering Start, copy the checked bot’s `@username` once.** Show that name with its own **Copy** beside the link; *Bot name copied* confirms success. A failed clipboard write says *Could not copy the bot name. Use Copy.* without blocking Start. Refreshing or revisiting the same Start step does not copy again; a newly checked bot does. The returned username is the only source, never a proposed name. The drawing explains this action but never writes to the real clipboard.

*Added 2026-10-23, Creator-stated, fifteenth note 3, 2026-10-21: “the bot’s name should be pre-loaded in the clipboard, and the dialog asking for it should have the name copy-able as well”. No auditor has seen it.*

**And one line sits at the top, not the bottom: do this on a computer where you use Telegram.** It is the [spec](../Specs/Telegram.md)'s own instruction, and it is at the top because it is the only thing in the dialog that can send the user somewhere else — telling them at the end would be telling them after they had tried. *(Until 2026-10-28 this read* do this on the computer where you can reach Telegram *and called it the spec's own sentence. It was not: `Specs/Telegram` § The bot asks for the instruction and holds no such sentence. Changed in step 13's words sweep, Rule 9.)*

**The token is held and never shown**, and can be replaced. One line says why it matters: whoever holds it can speak as the bot.

**One control changes the bot or its token: *Change bot or token…*, on the Bot row.** It opens the dialog above, from its first choice, and the checked bot's identity decides what follows, as it always has: the same bot keeps its proof and skips Start, a new bot needs Start. The Token row keeps its hidden token as information and has no control. *Added 2026-10-28, Creator-stated, the twentieth notes, note 5: "Use another bot and Replace in Telegram settings does the same thing. Is that by design? If it is, I think it has to be given a thought." Both opened the same dialog, so two labels promised a difference the first screen never made; runner-ruled. Until then the Bot row read* Use another bot… *and the Token row* Replace…*. No auditor has seen it.*

**The heading is *What a job reports*, not *by default*.** *By default* invited the question of what the other way is, which is the per-job choice on the settings page — and this section already says that in its last line. Label: **Creator-stated** (2026-08-13).

**What a job reports is three radios, not switches or boxes.** They are steps of one decision — everything, the pictures without the running commentary, or alerts only (endings and questions) — and a set of independent switches would invite combinations nobody wants. **One line names the floor**: *Endings and questions are always sent*, whichever is chosen. *(From 2026-10-28 it reads* All three send endings and questions.*, since a job set to* Off *on the [Render Settings](Render%20Settings.md) page sends neither, and* always *would say otherwise. The twentieth notes, note 4. No auditor has seen it.)* The choice here is the default; the [Render Settings](Render%20Settings.md) page settles it per job. *(That page offers a fourth choice, a picture every few minutes, which is not a default here: it carries minutes typed for that one job. Added 2026-10-19. No auditor has seen it.)*

**From 2026-10-31 the radios are five, the same five as the Render Settings list and in its order**: *Everything*, *Pictures only*, *A picture every [5] minutes*, *Alerts only*, *Off*. The third carries its minutes field inline, 5 to begin with, refused outside 1 to 60 the way the Render Settings field is. The floor line reads *All but Off send endings and questions.* The line under it reads *This is the default. Change it per job when you submit, or for all jobs with /reporting in Telegram.* A choice made from the phone with `/reporting` becomes this default (`Specs/Telegram`, *One choice for every job, from the phone*), so the radios show it the next time Smoozy has read a server that heard it; a click here afterwards stands until the next choice from the phone. *The Telegram discussion of 2026-10-31, Creator-stated: "The choice … should apply to everything and be the new default." Design conforming to Architecture `Telegram` V1a and V1d under Rule 10. Drawn in `set`'s Telegram section. No auditor has seen it.*

**Each enabled job’s start is reported under every reporting choice, and every job message keeps its Stop control with confirmation.** This exception is `Specs/Telegram`’s seventeenth-note start promise; the choices continue to narrow the other reports. *Added 2026-10-23; no auditor has seen it.* *(Under the three choices here. A job set to* Off *sends no start and no Stop: `Specs/Telegram`,* Off, for one job*. Added 2026-10-28, the twentieth notes, note 4. No auditor has seen it.)*

**The idle message has its own switch**, because the [Telegram spec](../Specs/Telegram.md) gives it one and it is the only one of the bot's messages about a server rather than about a job.

**Where servers have not been reached since the switch was changed, they are named.** A server Smoozy is connected to learns of the change at once; any other learns on its next connection and not before, so a server not yet reached may still be sending, or still be silent. Naming them is the only honest thing to show. *(Changed 2026-10-31; it said both specs were firm that every server learned only on its next connection. `Specs/Telegram` and Architecture `Telegram` L2 changed the same day, from the live Telegram pass's fault 11, where three connected servers kept sending after the switch went off. No auditor has seen it.)*

**A replaced token does the same thing, and so does being pointed at a new bot.** A connected server gets the new one at once, and any other on its next connection and not before, so until then a server that has not been reached is still speaking as the old bot. *(Changed 2026-10-31 with the paragraph above.)* The naming covers all three — the switch, a replacement token, a different bot — because in every one of them the state of a server Smoozy has not reached is unknown for the same reason. Label: **Creator-stated** (2026-08-13).

## What this screen closed

Six markers in the specs asked Design for something that lands here: the caps’ readings, the download folder, search folders and their order, notification switches, Telegram’s switch and bot setup, and reporting choices. The computer-naming marker is withdrawn.

**Telegram is drawn in full elsewhere, and this document holds its half that lands here.** Its phone conversation is in [the mockup](Mockup/Smoozy.html), on the Creator’s ruling of 2026-08-23. Notification wording and clicks belong to [Whole app](Whole%20app.md).

## Keyboard shortcuts

**None of its own.** The whole-app rules are in [Whole app](Whole%20app.md).

## What this document left open, and how it was answered

Every marker this document once carried is closed, and the answers are kept together here rather than folded back into the body. **Nothing below is still open** — the heading used to say *Left for later*, which was true when the markers were markers and stopped being true the day they were answered. Each bullet names the question it settles, which they did not when the questions were still above them.

- **Whether the control that restores switched-off warnings says how many it will restore.** **Settled at Design (2026-08-23).** It keeps the count. The [spec](../Specs/Whole%20app.md) grants no more than the control, and a count is not more: it is a reading of what the control will do, and without it the user cannot tell a control that restores nine warnings from one that restores none. Where none is switched off, the control is there and reads *none switched off* rather than going away — a control that appears only when it has work is a control nobody knows exists.
- **How a search folder is added, and what happens to one already on the list.** **Settled at Design (2026-08-23).** The system's own picker is the whole of it, and Smoozy draws none of it — the same rule the password prompt and the file picker already follow. A folder already on the list is not added twice and is not refused either: the list flashes the row it is already on, which answers the user's question without a box that says nothing they can act on. A folder inside one already on the list **is** added, because the order decides which wins and a user putting a narrower folder above a wider one is doing something deliberate.
- **How the search folders are put in order.** **Settled at Design (2026-08-23).** Drag is not alone: each row carries up and down controls as well as the drag handle, because the order decides which folder a search picks first and a list whose only way to reorder is dragging is a list some people cannot reorder. Dragging shows the row lifted and a line where it would land — not a gap opening, which makes the list jump as the pointer moves. The list is short and does not scroll while dragging.
- **What the Telegram section holds before Telegram is switched on.** **Settled at Design (2026-08-23).** The switch and one paragraph, and nothing else — no greyed bot row, no empty token field, no reporting choices. Those are all answers to questions the user has not been asked yet, and greying them puts four dead controls in front of somebody deciding whether they want this at all. The paragraph says what Telegram gets them — a job's progress and its finished frames on their phone — and that the user sets up the bot themselves, in Telegram. That last part belongs before the switch rather than after it: it is the one thing that might make somebody put this off until they are at the right computer.
