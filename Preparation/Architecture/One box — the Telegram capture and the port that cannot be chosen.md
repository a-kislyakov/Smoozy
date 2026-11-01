# One box — the Telegram capture and the port that cannot be chosen

What one rented machine answered on 2026-08-16 about Telegram, and what it settled about hosting a webhook on a rental at all. An **input** to the stage, like [The rented machine](The%20rented%20machine.md), [Batch A](Batch%20A%20—%20the%20shape%20throwaway.md) and [Six machines](Six%20machines%20—%20the%20container,%20the%20webhook%20and%20the%20ready%20action.md): nothing here is a decision or a plan.

**This is a frozen snapshot of one box on one day**, and it was destroyed at the end of it. A different rental, and above all a different provider, may differ in every particular. Read it as *this is what that box did*, never as *this is what a rented machine does*.

Every claim is **observed**, with what was run and what came back. Where something was not established, it says so. Rule 1's plain-English rule is suspended here, per the stage's exception.

**Why this rental existed.** [Six machines](Six%20machines%20—%20the%20container,%20the%20webhook%20and%20the%20ready%20action.md) left the Telegram payload unobserved: a webhook was registered and connected to, but no Update body, no secret-token header, no retry cadence, and no answer on whether updates queued while a server is down survive. That run's own conclusion was that closing them needed one rental with an open inbound port **and a rule: nothing else may hold the token while the capture runs.** The rule was kept, in the only form that means anything: **every call that could consume an update was deliberate and is named here, with its time.** Four things ever took updates — one poller on the box, one probe from the Creator's Mac made *in order to* collide with it, the two-holder test, and the throwaway that took without confirming. Everything else done with the token — sending, deleting, reading the queue's depth, `getMe` — consumes nothing.

---

## The machine

**Vast.ai instance `47860785`, offer `47762973`, machine `146005`, at `$0.0485/hr`**, rented 2026-08-16 11:14 UTC and destroyed the same day. Listed as Oregon, US; the host is a **verified** one at 99% reliability. Image `ubuntu:22.04`, chosen because the previous batch found that a bare image is what a Vast container rental will actually start.

Ubuntu 22.04.5 LTS, kernel `6.8.0-137-generic`. **PID 1 is `/bin/sh -c while [ ! -e /.launch ]; do sleep …`** — a container, not an init system. Root filesystem is a 20 G `overlay`, and `/tmp` is on the same overlay. The host underneath is bare metal: `ProLiant DL360 Gen9`, Intel Xeon E5-2620 v4 presented as **32 processors**, 32,772,720 kB memory. One **NVIDIA GeForce GTX 1650, 4096 MiB, driver 580.173.02**, with `/dev/nvidia-modeset`, `/dev/nvidia-uvm` and `/dev/nvidia-uvm-tools` present. Clock UTC. The image as delivered has **no `ip` and no `ss`** — neither was ever installed, so nothing here reads the machine's own network state — and no `python3`, `curl` or `openssl` either; those three were installed by the start-up script.

**Two ways in, and both matter below.** SSH only through Vast's proxy at `ssh7.vast.ai` port `20784`. Separately, a public address `50.120.65.61` with four mapped inbound ports.

---

## Fact 63 — a Vast rental cannot host a Telegram webhook, and the reason is not the certificate

**The rental was asked for the four ports Telegram accepts.** The creation request carried `-p 443:443`, `-p 80:80`, `-p 88:88` and `-p 8443:8443`. **What came back was `45104`, `45181`, `45149` and `45191`.** The provider maps the container's port to an external port of its own choosing, from its own range, and the number is not negotiable.

**The ports are genuinely open to the internet.** A self-signed certificate was made on the box for `CN=50.120.65.61` with `subjectAltName=IP:50.120.65.61`, a Python HTTPS listener was started on container port 443, and the Creator's own Mac reached it from outside: `HTTP 200`, connect `0.22 s`, TLS handshake complete at `0.48 s`. The request is on record in the capture file, headers and all. The same address on port `443` itself answered nothing. **So the box has a real, reachable, encrypted inbound endpoint — on the wrong number.**

**Telegram refuses it in as many words.** `setWebhook` with `url=https://50.120.65.61:45104/hook`:

```
{"ok":false,"error_code":400,"description":"Bad Request: bad webhook: Webhook can be set up only on ports 80, 88, 443 or 8443"}
```

**So the webhook question is decided by the provider's port allocator, before the machine ever boots.** The previous batch found that Vast maps ports only at rental time and gave that container no spare port at all; this one had four spare ports and still could not be used. **A URL naming a Vast container's own mapped port cannot be a webhook address**, and no certificate and no image changes it, because the provider chooses the number. **Narrowed 2026-08-21: this said *a Vast container rental cannot be a webhook host, whatever it is rented as, and no setup step changes it*.** That reads a fact about one address as a fact about a machine. Telegram's own reference page constrains the **URL** — *"HTTPS URL to send updates to"* and Note 3's four ports — and states nothing about the machine running the bot's code, so a URL naming some other address on one of the four ports is excluded by nothing seen here. That the earlier Hyperstack box could host one stands unchanged — the difference is a provider that hands over the address and lets the machine choose its own port.

**Not covered:** a Vast **virtual machine** rental was not tried. Those are offered and are cheap, and whether a VM gets an address with ports of its own rather than the same mapping is unknown. Nothing here should be read as covering it.

**One rate limit was seen, on registration.** Repeating `setWebhook` immediately after the refusal returned `{"ok":false,"error_code":429,"description":"Too Many Requests: retry after 1","parameters":{"retry_after":1}}`. That matches what the previous batch saw: a refusal that carries a wait-this-long field rather than simply failing.

**What therefore remains unobserved, and now needs a different provider.** The webhook POST wrapper — the request headers, and the `X-Telegram-Bot-Api-Secret-Token` header in particular — and Telegram's retry cadence against a webhook that fails or goes quiet. Neither could be posed here at all. **They are not blocking**: everything below was answered on the receiving call instead, which is the route that works on any rental.

---

## What updates do when nobody is listening

**Five messages, queued for sixty-seven minutes, all survived.** The Creator sent `1` to `5` at 11:26:06–11:26:12 UTC. Nothing held the token: no webhook was registered, nothing polled. At 12:32:41 the queue was read without consuming it — `getWebhookInfo` reported `"pending_update_count":5`. At 12:32:54 a long-poll was started, and every one arrived in a single answer, in send order, with consecutive `update_id`s `457042769`–`457042773`, held between **4002 and 4008 seconds**.

**So a machine that is down does not cost the user their message.** The whole body is intact, `date` included — an update carries the moment it was sent, so a machine coming back can tell how stale what it is holding is.

**Not covered:** sixty-seven minutes is what this box observed, and the 24-hour edge was not approached. [Six machines](Six%20machines%20—%20the%20container,%20the%20webhook%20and%20the%20ready%20action.md) records Telegram's documentation as saying updates are kept *"not longer than 24 hours"*; **that documentation was not re-read in this sitting**, and nothing here observes it.

## The Update body, as delivered

Recorded byte for byte in the capture files. A private text message arrives as an `update_id` and a `message` holding `message_id`, a `from` and a `chat` — both carrying the same numeric id for a private conversation — `date` as a Unix second, and `text`. The `from` carried `is_bot`, `first_name`, `last_name`, `username`, `language_code` and `is_premium`.

**A button press arrives as a `callback_query`**, not a message: its own `id`, the `from` who pressed it, the **whole of the bot's original message** including its `reply_markup`, and `data` carrying exactly the string the bot put on the button — here `stop:job-43`. That is the shape a stop-from-a-phone would arrive in.

**Two presses of the same button, five seconds apart, arrived as two separate updates** — `457042778` and `457042779`, different callback ids, the same `data` — with **neither answered**. An earlier single press had raised the question of whether an unanswered press swallows the next one. It does not.

## Two holders do not race — they deadlock

This is the sharpest thing the sitting produced, and it corrects the reading the previous batch left behind.

Two pollers were run against the one token for forty seconds, three seconds out of step, each long-polling with a ten-second timeout, neither confirming anything. **Every single call on both sides failed**, fourteen in a row:

```
{"ok":false,"error_code":409,"description":"Conflict: terminated by other getUpdates request; make sure that only one bot instance is running"}
```

Each 409 came back in **3.45 to 4.94 seconds** — not on asking, but the moment the other side asked. Neither holder ever received an update. The first clean answer came at 12:35:44, after the second holder had stopped.

**So a second holder does not quietly steal the first one's work. It makes the bot deaf.** While two are polling, nothing reaches either of them, and the failure is loud and specific rather than silent. The hazard the module rests on is real but different in shape from the one recorded before: the silent theft case is a holder that takes updates **and confirms them**; two holders that merely poll cancel each other out and nothing is delivered at all.

**The same 409 code covers a different case** — a poll while a webhook is registered — and the previous batch recorded its wording. The two are told apart only by the description text.

## An update taken but not confirmed comes back

Three updates were taken with `getUpdates` and deliberately not confirmed. Asked again two seconds later with no offset, **the same three came back, identical, same ids**. Only when asked again past the last id (`offset=<last>+1`) did they go, and two further asks returned nothing.

**So a machine that grabs an update and dies before acting on it destroys nothing.** The update is lost only when something takes it *and* confirms it. That is what makes the two-holder hazard survivable and what any receiving arrangement has to be built on: **confirm after the work is done, never on receipt.**

## Fact 65 — the sending pace, as far as this box could push it

**Nothing was ever refused.** No `429` was seen on any send, in any shape tried.

- **Sixty text messages, one after another**: all accepted, `35.196 s` end to end, about **0.5 s per send**. That is the round trip and not a limit — `getMe` from the same box took 0.46 s all afternoon.
- **Thirty sends fired at the same instant**: all thirty accepted. Telegram **serialised** them — `message_id` 95 to 124 in strict order, with the answers arriving about 25–30 ms apart — and answered twenty-eight within `1.26 s`. The last two came back at `5.59 s` and `5.62 s`. **The service absorbs a burst by delaying it, not by refusing it**, at least at this depth.
- **Ten pictures the size of a frame preview** — a 1920×1080 PNG of `1,465,514` bytes: every one accepted, **1.36 to 1.82 s each**, steady, no slowdown across the ten.

**So a picture per frame costs about a second and a half of wall clock on this line, and the ceiling was not found.** What was *not* done is sustained sending over hours, which is the half Fact 65 actually names; this is minutes.

**The bot's own messages can be taken back.** All 100 test messages were deleted afterwards with `deleteMessage`, one call each, every one accepted. An id that is not there answers `Bad Request: message to delete not found` — so *deleted* and *never existed* are not told apart.

## Fact 62 — reaching Telegram, over a longer window

`getMe` was called every five minutes for the length of the sitting. Every call answered `ok`, round trip 0.46–0.48 s. The window achieved was under three hours. **The *over a day* half remains unobserved**, as it was after the last batch.

## Fact 67 — a replaced token cuts the old holder off instantly, and it cannot tell why

The Creator revoked the bot's token in BotFather while a poller on this box was long-polling with the old one.

**The open poll was cut, not left hanging.** The call that was in flight returned at 13:04:02 UTC after 13.92 seconds of a 25-second wait, reporting no updates. The next call, 0.47 seconds later, was refused, and every call after it.

**The refusal carries nothing.** Body in full, 58 bytes:

```
{"ok":false,"error_code":401,"description":"Unauthorized"}
```

**A token that never existed answers byte for byte the same.** So a machine still holding a replaced token knows only that it is out — it cannot tell *replaced* from *mistyped* from *bot deleted*, and nothing in the answer names a cause or a moment. Whatever tells a machine it has been retired has to come from Smoozy, not from Telegram.

**The bot survives the change whole.** Under the new token, `getMe` returns the same `id` `1234567890` and the same username. The queue is empty and **the update position the old holder had confirmed still holds** — nothing already taken came back. So revoking replaces who may act as the bot and nothing else.

---

## Fact 57, the provider-console way — what it looked like through a proxy

**Read the caveat before the result.** This box's SSH was reachable **only through Vast's own proxy** at `ssh7.vast.ai`. So what is recorded below is what the **proxy** did when the machine under it went away, not what a machine that has been powered off does to a connection held straight to it. A direct connection may well behave completely differently — silence rather than a close — and that case is still unposed.

Three connections were held open at once and the instance was then stopped from the provider's console at 13:09:29 UTC: one idle with **no keepalive from the caller** (`ServerAliveInterval=0`), one idle **with** a keepalive every 15 seconds, and one with output flowing every 5 seconds.

**All three ended in the same second, 13:09:30**, each with `Connection to ssh7.vast.ai closed by remote host.` and exit code `255`. The keepalive made no difference and neither did the traffic: the close was announced, so every holder learned of it by itself and immediately, without needing to use the connection first.

---

## Fact 46 — the cardless box could not be rented here, and that is the finding

Batch B2 has always specified the cheapest box the provider offers **with no card at all**, and the previous batch could not pose it. **Nothing this search returned is one.** **Narrowed 2026-08-22: this said *Vast.ai does not sell one*.** That reads one search and two refusals as a fact about the marketplace. Every offer the search returns with no GPU is a **storage** ask — `resource_type: "disk"`, `cpu_ram: 0`, `gpu_frac: 0.0` — and two attempts to rent one were refused outright:

```
error 404/3603: no_such_ask  Instance type by id 30421090 is not available.
```

A filter for CPU-only machines is rejected by the API, and no offer with no GPU and real memory came back from any search run here. **Narrowed 2026-08-22: this said *no offer with no GPU and real memory exists at any price*.** Two refused storage asks do not establish that, and the search has no machine-kind filter to ask the question with. **So the no-card half of Fact 46 cannot be closed on this provider at all**, and the batch that closes it must go elsewhere. This is worth knowing before another rental is bought to try.

---

## What this sitting did not touch

- The webhook POST wrapper and its secret-token header; Telegram's retry cadence against a failing endpoint. **Needs a provider that hands over a port.**
- A Vast **virtual machine** rental, and whether it is mapped the same way.
- Sustained sending over hours, and reach over a day.
- The 24-hour queue edge.
- Fact 66, still wanting a second Telegram account.
- **Fact 57 on a connection held straight to the machine**, with no proxy between.

---

## The rental, closed out

One instance, `47860785`, alive from 11:14 to 13:09 UTC on 2026-08-16, **destroyed** at the end of it. The account was checked immediately afterwards and held **no instances and no volumes**.

**The one rule this rental existed to keep was kept**, in the form set out at the top: every call that could take an update was deliberate and is on record with its time, and the previous batch's payload was lost to exactly the opposite. **The webhook listener never held the token at all** — no webhook was ever successfully registered, so it received nothing from Telegram and could consume nothing.
