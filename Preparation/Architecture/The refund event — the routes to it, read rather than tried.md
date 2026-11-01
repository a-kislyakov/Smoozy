# The refund event — the routes to it, read rather than tried

*What Lemon Squeezy's own documentation establishes, on 2026-08-17, about the two routes that could deliver an `order_refunded` body inside the Creator's existing store: the dashboard's event simulation, and a test-mode order followed by a refund of that test order. What each route would cost, and which of Licence service (`Preparation/Architecture/Licence service.md`, withdrawn) §15 item 5's three remainders each would actually close.*

An **input** to the stage, like [The rented machine](The%20rented%20machine.md), [The host itself](The%20host%20itself%20—%20the%20licence%20platform%20run%20rather%20than%20read.md) and [The seller itself](The%20seller%20itself%20—%20the%20store%20observed%20rather%20than%20read.md): nothing here is a decision or a plan. Rule 1's plain-English rule is suspended here, per the stage's exception.

**Nothing was run.** No request to `api.lemonsqueezy.com`, no checkout, no webhook registration, no licence key read, no dashboard session, no authentication of any kind, and `~/.smoozy-keys/` was not opened. Every claim below is a quotation from a page at `docs.lemonsqueezy.com`, retrieved anonymously over HTTPS on **2026-08-17**. This document is the companion to [The seller itself](The%20seller%20itself%20—%20the%20store%20observed%20rather%20than%20read.md), which observed the store and reports item 5 as **not posable**; this one asks only whether a route to posing it exists on the vendor's own pages.

**The bar this document is written to.** Where a route rests on something the vendor states, the sentence is quoted. Where it rests on something the vendor **does not** state, the silence is reported as a finding with the search that established it, because for Route A a silence is the whole answer. Where a step is an inference from two statements rather than a statement, it is labelled **inferred** and never quoted as though the vendor said it.

---

## Retrieval, and the set that was read

**Label: read 2026-08-17.**

The site's own navigation was crawled to closure from four seeds (`/help/getting-started/test-mode`, `/api`, `/help`, `/guides`), yielding **264 internal addresses**: **231 returned HTTP 200**, **32 returned HTTP 308** section-index redirects, and **1 returned HTTP 404** (`/api/affiliates`). Every silence claimed below was checked against the text of all 264 retrieved bodies, not against the pages this document happens to quote.

[Lemon Squeezy — the licence service read against what Access needs](Lemon%20Squeezy%20—%20the%20licence%20service%20read%20against%20what%20Access%20needs.md) records that `docs.lemonsqueezy.com` answers **403** to an agent's page-fetching tool and **200** to direct HTTPS retrieval of the identical address. **That warning still stands and this reading did not re-test it** — direct HTTPS retrieval was used throughout and no other route was attempted. Its enumeration of 2026-08-16 counted 266 addresses and 232 HTTP 200s; this one counted 264 and 231, from different seeds. **Neither count is the size of the documentation set**; each is the closure of one crawl, and the two agree to within two addresses.

Nothing outside `docs.lemonsqueezy.com` is cited. No forum, no search result, no SDK repository, no recollection.

---

## The three remainders, restated exactly

Licence service §15 item 5 leaves three things unresolved, in these words:

> which fields the delivered `order_refunded` body actually carries, since no payload example for that event exists anywhere in the set; how many times the event fires; and the vendor's own `partial_refund` order status against its only worked partial refund returning *"status": "paid"*, which no page reconciles.

They are referred to below as **R1** (delivered fields), **R2** (firing count) and **R3** (the `partial_refund` disagreement).

**All three premises were re-checked against the whole set on 2026-08-17 and all three hold.**

**R1's premise holds.** The vendor's *Example Payloads* page carries **four** examples and `order_refunded` is not among them: *"order_created — Returns an Order object"*, *"subscription_created"*, *"subscription_cancelled"*, *"subscription_paused"* (`https://docs.lemonsqueezy.com/help/webhooks/example-payloads`, read 2026-08-17; the page's own *On this page* index lists those four and nothing else). No other page in the 264 carries an `order_refunded` body.

**And the one order body the vendor does publish for a webhook is narrower than the Order object.** The `order_created` example on that page carries `"refunded": null` and **omits** `refunded_amount`, `refunded_amount_usd`, `refunded_amount_formatted`, `setup_fee`, `tax_inclusive`, `affiliate_id` and `referral_amount` — every one of which [The seller itself](The%20seller%20itself%20—%20the%20store%20observed%20rather%20than%20read.md) found present on a real order read from `GET /v1/orders/100005`. **So the vendor's only published webhook order body is not the Order object its own webhook page promises**, and three of the fields §9's full-against-partial rule touches are missing from it. That is R1 with a sharper edge than §15 gives it: it is not merely that no refunded example exists, it is that the one order example that does exist would fail the rule.

**R2's premise holds.** Nothing anywhere states how many times `order_refunded` fires. The event's whole documented description is one line: *"order_refunded | Order object | Occurs when a full or partial refund is made on an order."* (`https://docs.lemonsqueezy.com/help/webhooks/event-types`, read 2026-08-17.) Searching all 264 bodies for `how many times`, `fires again`, `more than once`, `multiple refunds` and `second refund` returned nothing on this subject.

**R3's premise holds, exactly.** `partial_refund` appears in the Order object's status enum — *"The status of the order. One of: pending, failed, paid, refunded, partial_refund, fraudulent"* (`https://docs.lemonsqueezy.com/api/orders/the-order-object`, read 2026-08-17) — and the only worked partial refund in the set applies `"amount": 100` against a `"total": 1199` and returns *`"status": "paid"`*, *`"refunded": false`*, *`"refunded_at": null`*, `"refunded_amount": 100` (`https://docs.lemonsqueezy.com/api/orders/issue-refund`, read 2026-08-17). **`partial_refund` occurs in exactly two places in all 264 bodies** — that enum, and the subscription invoice object's *"partial_refund - The invoice was paid but has since been partially refunded"*. Nothing reconciles the enum with the example.

**One thing found while checking R3 that dates §9's rule and was not read before.** The Order object page states *"refunded — Has the value true if the order has been fully refunded"*, and the API changelog dates that meaning to **August 7th, 2024**, alongside *"Added refunded_amount, refunded_amount_usd and refunded_amount_formatted parameters to Order objects and Subscription invoice objects"* (`https://docs.lemonsqueezy.com/api/getting-started/changelog`, read 2026-08-17). **The changelog's own sentence about the Order object is an incomplete sentence in the page source**: the list item reads, in full, *"The `refunded` property of Order objects now only"* and stops. The parallel item completes: *"The `refunded` property of Subscription invoice objects now only reflects whether the invoice has been fully refunded"*. So the meaning §9 relies on is stated on the Order object page and **dated by a changelog entry that does not finish its own sentence**. It also explains the `order_created` example's `"refunded": null` — that example predates the change.

---

## Route A — the dashboard's own event simulation

### It exists, and one page says `order_refunded` is among its events

**Label: documented, read 2026-08-17.**

> When test mode is enabled, you can set up and test a full webhook integration. Lemon Squeezy will send webhooks for each event that happens in your test mode store just like it would in live mode.
>
> For more manual testing, you can simulate webhook events for any test mode order or subscription. Simply choose an event you'd like to receive a webhook for and send it manually to your app from your Lemon Squeezy admin.
>
> The following events can be simulated:
>
> **Subscription events** — `subscription_created`, `subscription_updated`, `subscription_cancelled`, `subscription_resumed`, `subscription_expired`, `subscription_paused`, `subscription_unpaused`, `subscription_plan_changed`
>
> **Order events** — `order_created`, `order_refunded`

— *Simulate Webhook Events*, `https://docs.lemonsqueezy.com/help/webhooks/simulate-webhook-events`, read 2026-08-17

**Ten events, `order_refunded` among them, dashboard-only.** *"from your Lemon Squeezy admin"* — no API endpoint for it exists in the 264 addresses, and the [Webhooks API](https://docs.lemonsqueezy.com/api/webhooks/create-webhook) exposes create, retrieve, list, update and delete and nothing that sends.

### Three other places in the same documentation say the feature is subscriptions only

**Label: documented, read 2026-08-17. This is a disagreement inside the vendor's own set and it goes to whether Route A exists at all for an order.**

> Additionally you can manually simulate individual subscription webhooks from the Lemon Squeezy dashboard.

— *Testing & Going Live*, `https://docs.lemonsqueezy.com/guides/developer-guide/testing-going-live`, read 2026-08-17

> **Simulate subscription webhooks in "Test mode"**
>
> We know building an API integration can be difficult, especially when you need to test subscription webhook events that happen in the future and can't be easily tested via the checkout.
>
> This is why we built a webhook simulation feature. When in "Test mode", you can manually trigger certain subscription events from your Lemon Squeezy account.
>
> Once you have some subscriptions in your test mode account (you can create these by purchasing products through the test mode checkout) you will see a "Simulate event" option in each subscription's side panel. Simply select the event you want to trigger and a matching event will be sent to your endpoint.

— *Sync with webhooks*, `https://docs.lemonsqueezy.com/guides/developer-guide/webhooks`, read 2026-08-17

> You can now manually simulate subscription events from within your Dashboard.

— *API Changelog*, entry dated **January 9th, 2023**, `https://docs.lemonsqueezy.com/api/getting-started/changelog`, read 2026-08-17

**What that costs Route A.** The only page describing **where the control lives** puts it *"in each subscription's side panel"*, and the only page listing order events is the help page above. **No page in the set says an order's side panel carries a Simulate control.** So the Creator may open a test order in the dashboard and find no such option, and nothing readable settles it in advance. This is a risk to Route A's existence, not merely to its fidelity, and it is the vendor contradicting itself on two pages read the same day — the same fault [The seller itself](The%20seller%20itself%20—%20the%20store%20observed%20rather%20than%20read.md) found on the *Event Types* page, where `subscription_plan_changed` is missing from a list that calls itself complete while appearing by name on the simulation page.

### What a simulated body contains: the vendor does not say

**Label: documented silence, established across all 264 retrieved bodies on 2026-08-17.**

**Nothing anywhere states the contents of a simulated body.** There is no simulated payload example of any event. Searching every retrieved body for `simulated payload`, `simulated webhook`, `simulated event`, `simulation payload` and `real webhook` returned **zero** occurrences of any of them.

**The nearest the vendor comes, twice, and neither is a contents statement.**

- *"a matching event will be sent to your endpoint"* (*Sync with webhooks*, address above). **"Matching" describes the event you selected**, not a body matching a delivered one. It is the sentence immediately after *"Simply select the event you want to trigger"*.
- *"To test subscription renewal events (`subscription_payment_*`), you need at least one renewal to have occurred on a subscription. **We generate simulation webhooks based on this renewal data.**"* (same page). **This is the only statement in the set about where a simulated body's contents come from**, and it is confined to subscription renewals: the simulated body is generated from data the store already holds. It says nothing about orders and nothing about equality with a delivered body.

### Whether a simulated body is identical to a delivered one: the vendor is silent, and that is the answer

**Label: documented silence, established across all 264 retrieved bodies on 2026-08-17. This is the finding that decides what Route A is worth.**

**No page in Lemon Squeezy's documentation states, in any words, whether a simulated webhook body is identical to a delivered one.** `identical` occurs **nowhere** in any of the 264 bodies. `same payload`, `same as a real` and `same data` occur nowhere.

**Two sentences look like fidelity claims and are not.** Both are about **real events in test mode**, which is Route B, and neither is about simulation:

- *"Lemon Squeezy will send webhooks for each event that happens in your test mode store just like it would in live mode."* — *Simulate Webhook Events*, address above. This is the paragraph **before** the simulation paragraph, and its subject is *"each event that happens in your test mode store"*.
- *"In test mode it's possible to test a full webhook integration. Webhooks are sent for all events just as they are in live mode."* — *Testing & Going Live*, address above. Its next sentence is *"Additionally you can manually simulate…"*, which marks simulation as a separate thing that the *"just as they are"* claim does not cover.

**So the fidelity of a simulated body rests on nothing the vendor has written.** A body obtained by simulation would establish which field names appear in something the seller calls an `order_refunded` body, and would establish nothing about what a real refund delivers.

### And the deeper problem with Route A, which the silence is not even the worst of

**Label: inferred from two documented statements, 2026-08-17. Named because it is decisive and because it is an inference rather than a quotation.**

Simulation is offered *"for any test mode order"* and the body it would carry is an Order object. **The twenty test orders in the Creator's store all read `refunded: false`, `refunded_at: null`, `refunded_amount: 0`, `status: "paid"`** ([The seller itself](The%20seller%20itself%20—%20the%20store%20observed%20rather%20than%20read.md), observed 2026-08-17). A simulation changes no record — no page claims it does, and the feature is described as a send rather than as an action on an order.

**So an `order_refunded` simulated against an unrefunded order would carry an order whose refund fields are all in the unrefunded state.** §9's rule reads `refunded` and corroborates on `refunded_amount` against `total`. Route A would hand it a body saying the order was not refunded. **It answers which fields are present and cannot answer what any of them say after a refund**, which is the half of R1 that the rule actually needs.

That is not a silence in the documentation. It follows from what the documentation does say, and it holds whether or not the simulated body is faithful.

### What Route A would cost

- **The Creator's own hands in the dashboard of a live store.** Reaching the admin means signing in to an account holding live data and moving the test toggle. **No agent on this stage does that** — the bar is absolute and is why [The seller itself](The%20seller%20itself%20—%20the%20store%20observed%20rather%20than%20read.md) records this route as not attempted.
- **A registered webhook, or nothing at all** (see the log below). A webhook can be created and deleted by API with the test-mode key, which the companion document did twenty times with the account proved identical afterwards.
- **No permanent data.** No order, no licence key, no customer, no refund. Nothing written to any pre-existing record.
- **Clicks:** sign in, toggle test mode, open a test order, find and use a Simulate control that may not exist for orders, choose `order_refunded`, read the result.

**Route A is cheap and reversible. It is also nearly empty.**

---

## Route B — a test-mode order through an ordinary checkout, then a refund of that test order

Each step below is established separately, because the route fails if any one of them is not available.

### Step 1 — a test order can be placed through an ordinary checkout against a product that already exists

**Label: documented, read 2026-08-17. No API product creation is involved anywhere in it.**

> In test mode, you can use dummy customer details and test card numbers to experience the checkout process from a customer's perspective. To make a test purchase:
>
> **Make sure test mode is enabled in the Dashboard** …
>
> **Make sure you have a "published" product in test mode** — Don't worry, this won't be visible to the public, only to you.
>
> **"Share" or "Preview" the product** — This will generate a test checkout link that you can use to test the checkout process.
>
> **Complete the checkout** — Do not use real card details for payment testing, as this may be interpreted as fraudulent activity. Instead, use the test credit cards listed below. …
>
> **Confirm that the order was successful 🥳** — You should receive an order receipt email and the order should be visible in your Lemon Squeezy Dashboard.

— *Test Mode*, `https://docs.lemonsqueezy.com/help/getting-started/test-mode`, read 2026-08-17

**The product is one that already exists and is published.** The Creator's store holds exactly that: product `100013`, `status: "published"`, `test_mode: true`, with licence keys behind it ([The seller itself](The%20seller%20itself%20—%20the%20store%20observed%20rather%20than%20read.md)). **The step needs no product to be created**, which is what closes the blockage that stopped item 5 — the companion document's Route two failed because *"the API cannot create one"*, and this route does not need one created.

**And the checkout URL need not come from the dashboard.** *"You can also get checkout URLs from the API by creating a checkout."* (*Sharing Products*, `https://docs.lemonsqueezy.com/help/products/sharing-products`, read 2026-08-17.) The endpoint exists: *"POST /v1/checkouts — Creates a unique checkout for a specific variant with specified attributes"* (`https://docs.lemonsqueezy.com/api/checkouts/create-checkout`, read 2026-08-17), returning an object whose `url` is *"The unique URL to access the checkout"* and whose `expires_at` is *"An ISO 8601 formatted date-time string indicating when the checkout expires. Can be null if the checkout is perpetual."* (`https://docs.lemonsqueezy.com/api/checkouts/the-checkout-object`, read 2026-08-17.)

**That matters for the bars.** A checkout for the existing variant can be produced with the test-mode key alone — no dashboard, no live-mode sign-in, no touching the product record. **Checkouts are the one thing in this route the API can create**, and [The seller itself](The%20seller%20itself%20—%20the%20store%20observed%20rather%20than%20read.md)'s list of what the API cannot create — products, variants, orders, licence keys — does not contradict this and did not include checkouts.

**Two things about the checkout the route inherits.** *"As soon as a checkout URL (/checkout/buy/...) is opened in a browser, it is converted to a "cart" (/checkout/?cart=...). These cart URLs should not be shared because they are designed to be single-use and unique to a customer."* (*Sharing Products*, address above.) And *"Order receipt emails in test mode will be sent to you and your team members, regardless of what you enter as the customer email address during checkout."* (*Test Mode*, address above) — so the receipt goes to the Creator, not to whatever address is typed.

**The one step no agent performs.** Completing the checkout means entering a card number — *"Visa: 4242 4242 4242 4242"*, with *"A valid future date for expiration (e.g. 12/35)"* and *"Any three-digit CVC (e.g. 123)"* (*Test Mode*, address above). **Entering card digits into a field is not an agent's action on this project**, whatever the digits are. This step is the Creator's hands, and it is the only step in Route B that cannot be handed to a script.

### Step 2 — a test order yields a licence key

**Label: documented, read 2026-08-17, from two pages neither of which states it on its own.**

> Testing subscriptions and license keys is also supported. File downloads are disabled for all test mode purchases.

— *Test Mode*, address above

> When the "Generate License Keys" setting is enabled, a unique license key will be generated for each customer that purchases the product.

— *Generating License Keys*, `https://docs.lemonsqueezy.com/help/licensing/generating-license-keys`, read 2026-08-17

**The second sentence carries no test-mode qualification and the whole page carries none** — `test` does not occur in its body. So licence key generation on a test purchase rests on the first page's *"license keys is also supported"* plus the second page's unqualified *"each customer that purchases"*. **Label the join inferred**, not quoted: no single sentence says *a test order generates a licence key*.

**It is also corroborated by the store rather than by a page.** The Creator's store holds **eleven licence keys, all `test_mode: true`**, behind twenty test orders of product `100013` ([The seller itself](The%20seller%20itself%20—%20the%20store%20observed%20rather%20than%20read.md)). Test purchases of that exact product have already produced test licence keys eleven times.

**One documented consequence to note and discard.** *"File downloads are disabled for all test mode purchases."* The route wants a licence key, not a file, so this costs nothing here.

### Step 3 — the test order can be refunded, and the vendor never says so in those words

**Label: documented for orders generally; a documented silence on test orders specifically. Read 2026-08-17.**

**Two routes exist and neither is qualified by mode.**

> Orders can be refunded partially or in full at any time via the action menu on the Orders page.

— *Refund Order*, `https://docs.lemonsqueezy.com/help/orders/refund-order`, read 2026-08-17 — that sentence is most of the page

> **POST /v1/orders/:id/refund** — Issues a partial or full refund for an order.
>
> `amount` Optional — Optional. The amount that you want to refund for an order. If not included full refund will be issued.
>
> Returns an Order object with updated refunded attributes.

— *Issue a Refund*, `https://docs.lemonsqueezy.com/api/orders/issue-refund`, read 2026-08-17

> If you do choose to offer refunds, they can be issued at any time via the Lemon Squeezy dashboard.

— *Refunds and Chargebacks*, `https://docs.lemonsqueezy.com/help/payments/refunds-chargebacks`, read 2026-08-17

**Nothing in the set says a test-mode order can be refunded, and nothing says it cannot.** Searching all 264 bodies for `refund a test`, `test order` and `refunding test` returned **zero** occurrences of any. The phrase `test order` does not appear anywhere in the documentation at all.

**What carries the step instead is an inference from two documented statements, and it is labelled as one.** *"API keys created in test mode will interact with test mode store data only."* (*Test Mode*, address above.) So a test-mode key calling `POST /v1/orders/:id/refund` with a test order's identifier acts on test data. Beside it: *"When Test mode is active, your Lemon Squeezy account changes into a test environment competely separate from your live store, with separate products, customers and purchases."* (*Testing & Going Live*, address above — the misspelling is the page's.) **Label: inferred.** The vendor does not state that the refund endpoint accepts test orders, and this route's third step would be the first thing to fail if it does not.

**And a second silence that the Creator must be told, because it is about their money.** **No page states what refunding a test-mode order does to a payout.** *Refunds and Chargebacks* says *"The refunded amount (minus our platform fee) will be deducted from your next payout"* without distinguishing modes, and the *"competely separate"* sentence is the only thing standing against it. **And what that sentence separates is products, customers and purchases — it does not name payouts. Added 2026-08-22: the quotation stood in this paragraph with no note of what it omits, so the silence is real rather than covered.** A payouts page was looked for in the help set on 2026-08-20 and there is none at the obvious address; the docs site publishes no sitemap. The store's own record reports `"total_sales":0,"total_revenue":0` under the test-mode key ([The seller itself](The%20seller%20itself%20—%20the%20store%20observed%20rather%20than%20read.md)), and the twenty test orders were paid with test cards, so no real money is behind the order that would be refunded. **The inference is strong and it is still an inference**, and it is about a store with live data on the other side of a toggle.

### Step 4 — a real refund fires the real `order_refunded` webhook

**Label: documented, read 2026-08-17. This is the step Route A cannot have and the reason Route B is worth more.**

> `order_refunded` — Order object — Occurs when a full or partial refund is made on an order.

— *Event Types*, `https://docs.lemonsqueezy.com/help/webhooks/event-types`, read 2026-08-17

> Lemon Squeezy will send webhooks for each event that happens in your test mode store just like it would in live mode.

— *Simulate Webhook Events*, address above

> In test mode it's possible to test a full webhook integration. Webhooks are sent for all events just as they are in live mode.

— *Testing & Going Live*, address above

> Test and live mode webhooks are kept separate within your account, meaning a test mode webhook will only be triggered for test mode data and vice versa.

— *Simulate Webhook Events*, address above

**A refund is an event that happens in the store.** So the body delivered is a delivered body, by the vendor's own claim about test mode, and the fidelity question that empties Route A does not arise. The last quotation is also the isolation guarantee: a test-mode webhook cannot carry a live customer's data, which is what made the companion document's twenty registrations permissible.

### Step 5 — the delivered body can be read out of the dashboard, and the firings counted

**Label: documented, read 2026-08-17. This was not read before and it changes what Route B costs.**

> **Viewing webhook events** — Recent webhooks sent from your store are logged on the webhooks settings page. You can see which events have been sent, the full payloads of each request and also resend webhooks if you need to.

— *Webhooks*, `https://docs.lemonsqueezy.com/help/webhooks`, read 2026-08-17

> If your app has not returned a HTTP 200 response by the fourth attempt, the event will not be tried again automatically, though you can re-send recent webhooks from your webhooks settings.

— *Sync with webhooks*, address above

**Three things follow, and the first two are what make Route B answerable rather than merely permissible.**

**The delivered body is readable without a receiver that works.** *"the full payloads of each request"* are on the settings page. **Label the next part inferred:** the log records what was sent, so a registration pointing at an address that answers nothing should still leave the payload readable — no page states that a non-200 response suppresses the log entry, and the retry sentence implies the send happens regardless of what answers. **R1 therefore does not require standing up an endpoint at all**, which removes the last piece of outside infrastructure from this route.

**The firing count is countable.** *"You can see which events have been sent"*. One refund, then the log; a second partial refund, then the log again.

**A delivered body can be re-sent.** *"you can re-send recent webhooks"* — so one refund's body is not a single chance.

### Step 6 — the test order and the test licence key cannot be deleted, and the vendor never mentions the subject

**Label: documented silence plus a documented endpoint list, established across all 264 retrieved bodies and the API navigation on 2026-08-17. This is the cost the Creator must be told before anyone clicks anything.**

**Nothing in Lemon Squeezy's documentation discusses deleting, clearing, resetting or removing test data.** Searching all 264 retrieved bodies returned **zero** occurrences of `delete test`, `delete a test`, `remove a test`, `cannot be deleted`, `start over` and `fresh start`, and zero matches for any pattern of the form *delete/remove/clear/reset/wipe/purge … test*. The only mentions of `test data` in the whole set are the isolation sentence quoted above and one line on the Zapier page. **There is no "clear test data" feature documented, and no statement that test data is permanent either.** The vendor simply does not raise it.

**What the API navigation exposes, enumerated from the site itself on 2026-08-17:**

| Resource | Documented endpoints | Create? | Delete? |
|---|---|---|---|
| Orders | the order object, retrieve, list, generate invoice, **issue a refund** | **no** | **no** |
| License keys | the object, retrieve, **update**, list | **no** | **no** |
| Products | the object, retrieve, list | **no** | **no** |
| Variants | the object, retrieve, list | **no** | **no** |
| Order items | the object, retrieve, list | **no** | **no** |
| Checkouts | the object, **create**, retrieve, list | **yes** | **no** |
| Webhooks | the object, **create**, retrieve, list, **update**, **delete** | **yes** | **yes** |

**Webhooks are the only resource in that list with a delete.** So everything Route B brings into existence except the checkout and the webhook is permanent.

**What Route B would leave behind, permanently, on the test side of the Creator's store:**

- **one test order** — the twenty-first, `test_mode: true`, and after step 3 **permanently marked refunded**
- **one test licence key** — the twelfth, `test_mode: true`, belonging to a refunded order
- **one order item**, and **a customer record** if the address typed at checkout is not one of the four already there
- **one checkout record**, which no endpoint deletes and which `expires_at` can be set to retire

**What it would not touch.** The twenty existing orders, the eleven existing licence keys, the product, the variant, the three files, and everything on the live side of the toggle. **The refund is issued against an order that did not exist before**, and that is checkable mechanically against the 10:00Z inventory in [The seller itself](The%20seller%20itself%20—%20the%20store%20observed%20rather%20than%20read.md) before any refund is sent. **A refund is irreversible and no page offers a route back**, so that check is the whole guard on this route.

**How much that permanence actually costs.** The store already holds twenty test orders whose buyer addresses are `buyer-1@example.invalid`, `buyer-2@example.invalid`, `buyer-3@example.invalid` and `buyer-4@example.invalid`, eleven test licence keys, and a variant reading `status: "pending"`. **A twenty-first test order in that store is not a new kind of clutter**, and it is invisible to the public: the companion document observed that the published test product is absent from the live storefront and that its own buy link answers `302` away from a checkout on the live domain. **The cost is real, permanent, and small**, and it is the Creator's store to decide about.

---

## What each route would actually close

| | **R1** — which fields the delivered body carries | **R2** — how many times the event fires | **R3** — `partial_refund` against `"status": "paid"` |
|---|---|---|---|
| **Route A** — simulate `order_refunded` | **Field names only, and unverifiably.** The vendor never says a simulated body equals a delivered one, and the simulated body would carry an **unrefunded** order — so it cannot report what any refund field says after a refund. **The half §9's rule needs stays open.** | **No.** A simulation is one deliberate send. It says nothing about what a refund causes. | **No.** A simulation changes no order record, so no status transition occurs to observe. |
| **Route B** — real test order, real refund | **Yes, outright.** The genuine delivered body, readable from the webhook log, for a full refund and for a partial one. | **Partly.** The count for the refunds actually performed is readable from the log — one full refund, a first partial, a second partial. **A general rule is not obtainable**; this would be one store on one day, like everything in the companion document. | **Yes.** A real partial refund shows whether `status` becomes `partial_refund` or stays `"paid"`, and whether `refunded` stays false while `refunded_amount` rises. A real full refund shows the other end. |

**And Route B would close two things outside item 5 that nothing else can reach.** [The seller itself](The%20seller%20itself%20—%20the%20store%20observed%20rather%20than%20read.md) records both as not posable, for the same reason — the states do not exist in the store and making one means writing to a pre-existing record:

- **The unauthenticated validate response for a key whose order has been refunded.** After step 3, the tester owns exactly such a key. That is the question §15 item 6's remainder turns on, and the one with a stated cost: *"nothing observed says whether a refund changes `status` or `valid` at all"*. If it does, **Row 237's day-long hole closes outright** rather than being narrowed; if it does not, the reconciliation path is confirmed as necessary rather than assumed. Either answer is worth having.
- **The unauthenticated validate response for a `disabled` key.** `PATCH /v1/license-keys/:id` exists — *"Update a license key"* — and the tester's own key can be disabled with it, without going near the Creator's eleven.

**Neither of those needs a second refund, a second order, or a single extra click beyond the route already described.**

---

## What both routes leave silent whatever happens

- **Whether `order_refunded` ever fires more than once for one order, as a rule.** Only the vendor could state that and it does not.
- **Whether `dispute_created` and `dispute_resolved` fire, and what they deliver.** Both are absent from every one of the 264 pages, and the companion document found the store accepts both. **Causing a dispute is unresolved rather than impossible, and Route B reaches it. Corrected 2026-08-22: this bullet closed *"A dispute cannot be caused."*** The vendor's *Test Mode* page names Stripe's testing page as the full list of test cards its own checkout supports, and that page documents `4000000000000259` for a fraudulent dispute (both read 2026-08-20) — so **one extra checkout at step 1, entering that number instead of `4242 4242 4242 4242` with `dispute_created` registered, puts this inside Route B** rather than leaving it silent whatever happens. The cost is one more permanent test order and one more permanent test licence key.
- **What refunding a test order does to a payout.** Named above; no page addresses it.
- **Whether the simulate control exists for an order at all.** Only the dashboard settles it, and the vendor's own pages disagree.
- **The ordinary limit on all of it.** Every quotation above is worth its date and no more. Read 2026-08-17.

---

## Recommendation to the runner

**Put Route B to the Creator. Do not put Route A to them as a route in its own right.**

**Route A is not worth doing on its own, and the reason is not the silence.** Even a perfectly faithful simulated body would carry an order that has not been refunded, so it can report which field names appear and nothing about what they say after a refund — which is the only half of R1 that §9's rule needs. It closes R2 and R3 not at all. Against that it needs a live-mode dashboard sign-in, which is the one thing this stage's bars refuse outright. **A route that costs the highest-risk action available and closes nothing is not a trade worth offering.**

**Route B is worth offering, and it is close.** Every step is established from the vendor's own pages, the two weakest joins are labelled inferred rather than quoted, and the payoff is larger than item 5: R1 outright, R3 outright, most of R2, **plus the refunded-key and disabled-key validate responses that the companion document could not pose at all** — and that last one is the only item in this batch with a named cost to the user, since it decides whether Row 237's day-long hole closes or stays. The permanent price is one test order and one test licence key on the test side of a store that already holds twenty and eleven, invisible to the public, and never any of the Creator's existing records. **The single guard that matters is that the refund goes only to an order identifier absent from the 10:00Z inventory**, because a refund cannot be undone.

**Two things go to the Creator with it, and they are not decoration.** The step no agent performs is entering the test card, so the checkout is their hands. And the vendor nowhere states that refunding a test order has no effect on a payout — the inference is strong, the store reports zero revenue under the test key, and it remains an inference about an account with live data on the other side of the toggle. **They should be told that in those terms and left to rule.** **And one line goes with it, added 2026-08-22:** the vendor's separation sentence covers products, customers and purchases and **does not name payouts**, so the silence is real; the two facts that make the risk small are the test cards behind the twenty orders and the zero revenue the store reports under the test key.

**If they decline, item 5 stays exactly where it is and that is a defensible place.** §15 already records it as breaking *"nothing outright"*: each remainder is covered by the daily reconciliation, a refunded key goes down that path rather than the immediate one, and the rule reads no contested field. **The honest summary is that Route B is cheap and worth doing, and that declining it costs a bounded, named, already-written-down delay rather than a promise.** Nothing here justifies a spend, a plan change, or a second seller account — and the Creator has already ruled the last of those out.

---

*Written by Claude Opus 5 (Claude Code), a reading agent on the architecture runner's brief, 2026-08-17. Nothing was run: no request to the seller's API, no checkout, no webhook registration, no key read, no dashboard session, and `~/.smoozy-keys/` was not opened.*
