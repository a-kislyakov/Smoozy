# The pinned line — what a conversation actually shows for a pin, and the filter that had been eating the bot's messages

*An input document of the Architecture stage. It carries **Fact 102**, both halves, and one thing nobody posed. Run 2026-08-29 against the live bot this project already holds. **No machine was rented and nothing was spent.***

**Read it as *this is what that bot and that client did on that day*.** Every limit is stated at the end rather than left to be assumed.

## What was run, and on what

**The bot:** `smoozy_example_bot`, id `1234567890` — this project's own test bot, made 2026-08-12, its token in `~/.smoozy-keys/telegram.txt` since 2026-08-19. `getMe` returned the same id and username the record already held.

**The conversation:** a **private** conversation between that bot and the Creator's own Telegram account. **The account's numeric identifier and username are deliberately not written down here**; they are the Creator's, they are not needed by anything, and the observations below hold for any private conversation.

**The client that was looked at:** the Creator's own Telegram desktop application on macOS, in one window, on 2026-08-29. **This is the half no documentation answers**, and `Preparation/Architecture/Telegram.md` §21 item 17 says so in as many words — *"What is genuinely not established is what a Telegram client draws for that service message"*. It was answered by looking, not by reasoning, which is the failure that made item 17 be rewritten on 2026-08-20.

**Everything else** — the calls, their answers word for word, and what the bot received — was run from the Creator's Mac against Telegram's live service, and is quoted below.

**The runner ran the calls itself rather than briefing an agent**, because each step needed the Creator's eyes before the next could be composed. **Nothing here is an answer to a row**; the row it unblocks is answered separately and by somebody else.

## The test, and what a negative would have looked like

Three messages were sent, then pinned in order: **A and B with the notification suppressed**, exactly as `Telegram` B10's machinery pins, and **C with it not suppressed, as the control**. Then C was unpinned. Then a fourth, long, machinery-shaped message was sent, pinned suppressed, **edited**, and **pinned a second time while already pinned**.

**The control is what makes it a test.** A negative — the answer that would have cost this project a promise — is a suppressed pin drawing **nothing** in the conversation while the unsuppressed one drew a line: that would have made the whole *conversation stays quiet* paragraph, and the spec sentence Row 506 quotes, an account of a cost the user never meets. **That is not what happened.**

## Fact 102, first half — the update carries the pinned message whole

**Answered: yes.** Each pin produced its own update, and each update's `message` carried a **`pinned_message`** holding the entire original message — its id, its sender, its chat, its date and its text. Observed three times, once per pin. The three service messages took ids `149`, `150` and `151`, each with its own `date`, so **a pin creates a real message in the conversation's history** — which the data model documented and which is now observed.

**So `Telegram` B11's fallback — the holder reading the reported pin — is insurance rather than the ordinary route.** The holder can take the pinned message out of the update it already has.

## Fact 102, second half — what the conversation shows

**Answered, and the news is in the shape of the line rather than in whether there is one.**

**1. There is a line, and suppressing the notification does not remove it.** A and B, both pinned with the notification suppressed, drew exactly the same kind of line as the unsuppressed control C. **So `disable_notification` silences the notification and nothing else** — `Telegram` §21 item 17's ruling of 2026-08-25, until now reasoned, is now observed. *(Noted 2026-10-18, Creator-stated: a phone announced every pin made with `disable_notification` in a private chat, so a suppressed pin is not silent; `Telegram` B24 and Sources entry 41. No auditor has seen it.)*

**2. The line is not *X pinned a message*. It quotes the message.** What was drawn, three times, word for word:

> `smoozy_example pinned "Fact 102 test — message A"`

> `smoozy_example pinned "Fact 102 test — message B"`

> `smoozy_example pinned "Fact 102 test — message C"`

**The bot's own name and the pinned message's own text**, in quotation marks. `Preparation/Specs/Telegram.md`'s sentence — *"Telegram itself writes a short line into a conversation every time anything is pinned — X pinned a message"* — **describes something narrower than what the user meets.** The line is not anonymous about its content.

**3. The quote is truncated, at about seventy characters on this client.** The long message, 95 characters, drew:

> `smoozy_example pinned "ACK m3 job=7f2a1c9e attempt=4 frames=118-240 board=rev82 state=finish…"`

**69 characters of the message survived**, with the ellipsis inside the closing quotation mark.

**The cut is by the window's width and not by a character count.** Creator-stated, 2026-08-29, established by resizing the window and watching the line lengthen and shorten with it. **So there is no ceiling to quote here**: how much of a machinery message a user meets is a property of their window, and on a wide screen it is more than the sixty-nine characters this one window happened to show. Sixty-nine is a measurement of this window, not a limit of the client.

**4. The quote follows the message's later edits, and this is the finding that matters most.** The long message was pinned, **then edited**, and the drawn line quotes the **edited** text — `board=rev82 state=finished` — not the text that stood at the moment of pinning. **So the line is a live window onto the pinned message rather than a record of a past act.** `Telegram` B4 makes the board's heartbeat an **edit**, so a board that is pinned once and edited continuously has a line in the user's conversation whose visible text keeps changing.

**5. Re-pinning a message that is already pinned costs nothing.** The second pin of the same message produced **no update to the bot and no second line in the conversation**. The screen shows one line for that message, not two. **A line is written when a message goes from unpinned to pinned, and at no other time.**

**6. Editing a pinned message produces no update to the bot and no new line.** Observed on the same message.

**7. Unpinning produces no update at all, and leaves the line standing.** Three pins gave three updates; the unpin of C gave none. In the conversation, C's line *"smoozy_example pinned …"* is still there after the unpin — **what the unpin removed is the small pin marker drawn beside the message itself**, which A and B still carry and C no longer does. **The lines accumulate and nothing takes one away.**

## The phone, looked at the same day — and it shows one thing the desktop does not

**Added 2026-08-29, after the desktop half. The device the whole module exists for.** Telegram's Android client, an Android phone, same conversation, same messages.

**The lines are the same, and cut shorter.** `smoozy_example pinned "Fact 102 test — mess…"` — the same shape, the same quoting of the message's own text, truncated harder because the window is narrower. **This confirms the cut is by width on a second client**, and it removes the limit this document carried: what the phone draws is no longer unestablished.

**The pin markers behave the same**: A, B and the long machinery message carry one; C, unpinned, does not.

**And the phone puts the pinned message's text permanently at the top of the conversation.** A **Pinned Message** banner sits under the chat's title, above everything, showing one truncated line of the **current** pinned message — here `ACK m3 job=7f2a1c9e attempt=4 frames=…`. **It is not a line in the history that scrolls away. It is a header that is always there**, and what it shows is whatever is pinned right now.

**That matters more than the lines do**, and nothing had reached it. `Telegram` B1 keeps the board pinned as the steady state, so **in the built app that banner shows the board** — Smoozy's own machine-to-machine noticeboard — **at the top of the user's conversation, permanently, updating as B4 edits it.** The lines accumulate below and are read once; the banner is read every time the user opens the chat.

**One limit stays:** whether the desktop client has an equivalent header was not looked at, and the banner's own truncation was not measured against window width.

## What this costs, stated plainly and not smoothed

**The noise is far lower than `Preparation/Architecture/Rows/Telegram.md`'s Row 506 table assumes, and the reason is finding 5.** That table prices a line per acknowledgement, per gather reply, per announcement and per instruction proposal, from every machine. **A machine re-using one message that stays pinned writes one line ever**, not one per act. **What that turns on is the janitor**: `Telegram` B18's holder pops the stack by unpinning, and a machine whose message has been unpinned pays a fresh line the next time it pins. **Working out the real rate against B9 to B15 and B18 is the row's, not this document's**, and this document deliberately does not do it.

**What is worse than the spec says is the content, not the count.** A machinery message carries what one machine tells another, and **that text is quoted into the user's own conversation and keeps up with the edits.** The spec's *a short line — X pinned a message* prepares the user for an anonymous line. **What they get is Smoozy's internal chatter, live, and as much of it as their window is wide** — the cut being by width rather than by any count, so a wide window shows more rather than the same.

**Both halves are for the row and the Creator to weigh, and this document rules on neither.**

## And one thing nobody posed — the filter that had been eating the bot's messages

**The bot's token was carrying a leftover `allowed_updates` filter, and it had been silently discarding every ordinary message sent to the bot.** `getWebhookInfo` returned, word for word:

> `{"ok":true,"result":{"url":"","has_custom_certificate":false,"pending_update_count":0,"allowed_updates":["my_chat_member"]}}`

**So `getUpdates` answered `{"ok":true,"result":[]}` while the Creator had genuinely written to the bot.** Nothing was queued and nothing was delayed — the messages were dropped. The filter was reset by naming the wanted kinds on a `getUpdates` call, and the message arrived on the next attempt.

**Three things follow, and all three are the architecture's rather than this document's to act on.**

- **The setting lives on the token, not on a caller.** Every machine in a fleet shares one token, so **one machine setting this filter changes what every other machine can hear**, and nothing in the answer to an ordinary call reveals it. It is visible only by asking `getWebhookInfo`.
- **An empty update queue does not mean an empty conversation.** `Facts.md`'s Fact 129 entry records that *"The runner tried on 2026-08-19 and found the bot's update queue empty, so no conversation identifier was available"* — **the same reading this filter produces.** Whether that is what happened then is not established and is not claimed here; what is established is that an empty queue has a second cause nobody had recorded.
- **A message dropped this way is gone.** It is not the queued-updates case entry 18 observed surviving sixty-seven minutes; nothing was held.

**Where the filter came from is not established.** It names `my_chat_member` alone, which no clause of `Telegram` asks for.

## The limits, and every one of them is real

- ~~**One client, one day, one window.**~~ **Two clients, one day** — Telegram's desktop application on macOS and its Android client on the Creator's phone, which agree on the line's shape and differ only in how much width they have. **Nothing here says what the web client or an older version draws**, and Telegram publishes its data model rather than its clients' rendering, which is why this was looked at rather than read.
- **One private conversation, one account, one bot.** No group conversation was tried, and a group is where pinning rules differ.
- **The truncation is by width, which is settled** (Creator-stated, 2026-08-29, by resizing) — but **what a wide window shows was not measured**, only that it shows more. The sixty-nine characters are this window's figure.
- **Whether the phone raised a notification for the unsuppressed control was not reported**, so this document claims nothing about it. The `silent` flag's documented half already stands at the sources file's entry for it.
- **The unpin was made by the same bot that pinned.** An unpin by the user was not tried.
- **Nothing was deleted.** The conversation is left exactly as the test made it, so a later check can look at the same screen.

---

*2026-08-29 — run and written by the architecture runner, Claude Opus 5 (Claude Code), with the Creator at the client. **No auditor has seen this text.***
