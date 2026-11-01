# The seller itself — the store observed rather than read

*What the Creator's own Lemon Squeezy store did on 2026-08-17 when three of the questions Licence service (`Preparation/Architecture/Licence service.md`, withdrawn) §15 records as unresolved — items 5, 6 and 9 — were put to it with a test-mode API key, together with the account inventory taken before and after.*

An **input** to the stage, like [The rented machine](The%20rented%20machine.md), [Batch A](Batch%20A%20—%20the%20shape%20throwaway.md), [Six machines](Six%20machines%20—%20the%20container,%20the%20webhook%20and%20the%20ready%20action.md) and [The host itself](The%20host%20itself%20—%20the%20licence%20platform%20run%20rather%20than%20read.md): nothing here is a decision or a plan. Rule 1's plain-English rule is suspended here, per the stage's exception.

**This is the first time anything in this project has observed the seller rather than read about it.** [The host itself](The%20host%20itself%20—%20the%20licence%20platform%20run%20rather%20than%20read.md) closes with *"Anything about the seller. No account, no key, no product, no webhook."* This document is its companion and covers exactly that gap — but only the part of it that a read-only key and a deletable object could reach. **Two of the three questions are answered, one of them in a way that contradicts what was read; the third could not be posed at all**, and the section that says so says what it would take.

**Every entry carries the line saying what a negative result would have looked like** — and where a question could not be posed, it says that instead, because a negative-result line written for a test nobody ran would be the worst thing in this document. Where a question was posed, it was posed with a deliberate control that produced the negative: a licence key that does not exist, returning a body with no `meta` object at all; sixty-eight event names refused with one identical sentence, against the twenty accepted; and a second inventory of the whole store run against the first.

---

## Six limits, before anything else

**One — this is one account on one day, and it is the Creator's own.** A different account, a different plan, a different country or a different month may differ in every particular. Read it as *this is what that store did*, never as *this is what Lemon Squeezy does*.

**Two — the store is on the `free` plan and was in live mode throughout.** Everything read below is the test-mode side of it, seen through a test-mode key. The live side was never read, and the live-mode key in `~/.smoozy-keys/lemonsqueezy.txt` was never opened, never used and never tested.

**Three — nothing pre-existing was written to, refunded, disabled, renamed or deleted.** The proof is the two inventories below. Everything the store held at 10:00Z it held at 10:12Z, down to the `updated_at` timestamp of all eleven licence keys.

**Four — nothing was bought, no card of any kind was entered, no checkout was completed, no subscription was started and no plan was changed.** Nothing cost anything anywhere. The only money-shaped thing attempted was reading a refund field on an order that was already not refunded.

**Five — three questions were brought here and only two of them could be put to the store.** Item 6 is answered on its positive side and could not be posed on its negative side. Item 5 could not be posed at all, in any of its three parts. Item 9 is answered, and its answer is the one that matters most in this document. Each is set out where it belongs, not gathered into an excuse.

**Six — the only thing outside the Creator's account that this work used was a throwaway HTTP endpoint at `webhook.site`**, created anonymously by `POST https://webhook.site/token` at 10:07:26Z as UUID `00000000-0000-0000-0000-000000000000`, used as the destination of every webhook registered below, and destroyed by `DELETE https://webhook.site/token/00000000-…` at 10:12Z, returning HTTP 204. **It received nothing.** Its request log, queried at the end, reported `total: 0`. No account was created for it, it cost nothing, and after deletion its address returns a 404 page.

**Publication substitutions.** Credentials, personal names, country and buyer addresses are redacted or replaced with examples. Account and object identifiers use consistent placeholders; field names, response shapes and observed conclusions are kept.

---

## 0. The toggle and the key's world

**Label: observed 2026-08-17, in an earlier sitting, not re-run by this work.**

The vendor documents only that *"API keys created in test mode will interact with test mode store data only."* (`https://docs.lemonsqueezy.com/help/getting-started/test-mode`, read 2026-08-17.) **Nothing anywhere in the vendor's pages says what the dashboard's live/test toggle does to a key that already exists.**

So it was posed. The two read-only calls `GET /v1/users/me` and `GET /v1/stores` were run with the test-mode key **while the store was in test mode**, and then run again, identically, **after the store had been switched back to live mode**. Both times HTTP 200, both times `meta.test_mode: true`, the same store id and the same test-mode readings. The conclusion drawn was that **the toggle and the key's world are independent**, and that live-mode data are not visible to this key at all.

**Three limits ride with that sitting:** one account, one day, the Creator's own; only two endpoints were called and both were reads; and nothing was created, written or refunded anywhere.

### Whether this sitting agrees

**The conclusion: yes, and more strongly than two endpoints could show. The evidence for it: partly no, and the correction matters to anyone who tries to re-run the check.**

The store was in **live mode for the whole of this sitting** and the toggle was never touched. `GET /v1/users/me` was called at the start and again at the end, and both times returned `{"meta":{"test_mode":true},…}`. Every other call in this document was made against a live-mode store and returned test-mode data.

**But `GET /v1/stores` carries no `test_mode` field at all** — not in `meta`, not on the store object, on the list endpoint or the single-store endpoint. Its `meta` carries pagination and nothing else:

```
{"meta":{"page":{"currentPage":1,"from":1,"lastPage":1,"perPage":10,"to":1,"total":1}},…}
```

and `GET /v1/stores/100012` carries no `meta` key whatsoever. **So the verification step as written — *confirm `meta.test_mode` is `true` on both* — cannot be completed on `/v1/stores` by anybody, in either mode.** Whatever the earlier sitting compared across the toggle, it was not that field on that endpoint.

**What is there instead is better, and it is the thing to check in future.** Every *object* the key returns carries its own `test_mode` attribute, and every one of them was `true`: one product, one variant, twenty orders, twenty order items, eleven licence keys, three files, four customers, and every webhook created and deleted below. Against that:

- the store reports `"total_sales":0,"total_revenue":0,"thirty_day_sales":0,"thirty_day_revenue":0` — **on the live side of an account**;
- the twenty orders carry four buyer addresses between them and every one is obvious rubbish — `buyer-1@example.invalid`, `buyer-2@example.invalid`, `buyer-3@example.invalid`, `buyer-4@example.invalid`;
- the store's `plan` reads `"free"`.

**What a negative would have looked like:** a single object anywhere carrying `test_mode: false`, or a non-zero sales figure, or an order with a real customer address. None appeared in any of the twelve collections listed. **So the earlier sitting's conclusion holds and this sitting extends it**: the key saw test-mode data only, on a store that was in live mode the entire time, across every collection the API exposes rather than across two endpoints.

---

## The store, before anything was created

Read at **2026-08-17 10:00Z**, with the test-mode key. Store `100012`, name `Smoozy`, slug `smoozy`, domain `store.smoozy.app`, country `[redacted]`, currency `EUR`, created 2026-02-25. User `100002`.

| Collection | Total | Identifiers |
|---|---|---|
| Products | **1** | `100013` — "Smoozy", `status: "published"`, `test_mode: true`, price 2000 |
| Variants | **1** | `100003` — "Default", `status: "pending"`, `test_mode: true` |
| Orders | **20** | all `status: "paid"`, all `refunded: false`, all `refunded_at: null`, all `refunded_amount: 0`, all `test_mode: true`; four buyer addresses between them, every one obvious rubbish — `buyer-1@example.invalid`, `buyer-2@example.invalid`, `buyer-3@example.invalid`, `buyer-4@example.invalid` |
| Order items | **20** | across four products — `100014`, `100015`, `100013`, `100016` |
| Licence keys | **11** | 8 `inactive`, 3 `active`, **none `disabled`**, none with an expiry, all `activation_limit: 2` |
| Licence key instances | **5** | named `host-a.local`, `[redacted]`, `DESKTOP-EXAMPLE-A`, `DESKTOP-EXAMPLE-B`, `192.0.2.4` |
| Webhooks | **0** | — |
| Subscriptions | **0** | — |
| Discounts | **0** | — |
| Files | **3** | `Smoozy_1.0.6_aarch64.dmg`, `Smoozy_1.0.6_x64.dmg`, `Smoozy_1.0.6_x64_en-US.msi`, all attached to variant `100003` |
| Customers | **4** | — |
| Checkouts | **0** | — |

**Three things in that table decide most of what follows.**

**No order in the store is refunded, and no licence key in it is disabled.** That is why item 6's negative half and the whole of item 5 could not be posed by reading: the states those questions ask about do not exist in this store, and making one means writing to something that was already there.

**The store already holds a test-mode product of the Creator's own, published, with three files and twenty test orders behind it.** It is `100013`. It was listed and never opened for writing: no checkout was created against it, no order was placed against it, no field of it was changed.

**And a product can disappear while the orders that name it stay.** The twenty order items name four products; the product list holds one. `GET /v1/products/100014`, `/100015` and `/100016` each answer

```
{"jsonapi":{"version":"1.0"},"errors":[{"status":"404","title":"Not Found"}]}
```

with HTTP 404, while their order items sit in the store carrying those identifiers. **This bears on Licence service §8's *"It verifies the store and product the key belongs to."*** A licence key's `meta.product_id` can name a product that the authenticated API no longer has, so a product check written as *fetch the product and compare* fails where a check written as *compare the identifier against the one this adapter expects* does not. **It was not observed on a licence key** — all eleven belong to `100013`, which still exists — so this is a hazard shown on order items, not a validate response anybody saw.

## Whether a test object can reach a live customer, settled before any write

**This had to be settled before a single write, and it was settled three ways.**

**Documented.** The vendor's test-mode page, read 2026-08-17, on making a test purchase: *"Make sure you have a "published" product in test mode"* — *"Don't worry, this won't be visible to the public, only to you."* And on the crossing: *"Products created in test mode won't automatically transfer to live mode when your store is activated. However, you can easily copy them, including discounts, to live mode using the "Copy to Live Mode" option in the product or discount menu."* **So the only documented route from test to live is a deliberate menu action in the dashboard**, and nothing in this work went near the dashboard.

**Documented, for webhooks specifically.** *"Test and live mode webhooks are kept separate within your account, meaning a test mode webhook will only be triggered for test mode data and vice versa."* (`https://docs.lemonsqueezy.com/help/webhooks/simulate-webhook-events`, read 2026-08-17.) Every webhook created below read back `test_mode: true`, so none of them could have carried a live customer's data to the throwaway endpoint even for the seconds it existed.

**Observed, twice.** The live storefront `https://store.smoozy.app/` was fetched: its entire body is the store name and *"Powered by Lemon Squeezy"*. **The published test-mode product `100013` is not listed on it.** And the buy link that product's own API record carries, `https://store.smoozy.app/checkout/buy/00000000-0000-0000-0000-000000000000`, answers

```
HTTP/2 302
location: https://store.smoozy.app/checkout
```

— **it redirects away rather than rendering a checkout.** A member of the public following a test-mode product's own purchase link on the live domain does not reach a purchasable page.

**What a negative would have looked like:** the product listed on the storefront, or the buy link returning 200 with a checkout form. Neither. **This is the observation that made the webhook work permissible**, and it is also why no product was created even after it: the bound it clears is visibility, not deletability, and deletability is what stopped the rest.

## What was then created, and what it was for

**Twenty webhook registrations and nothing else.** Every one pointed at the throwaway endpoint, every one carried a single event name, and every one was deleted. No product, no variant, no order, no licence key, no customer, no discount and no checkout was created, because **the API cannot create any of those** (see item 5 below) and because an order, once made, cannot be removed by any route this work had.

---

## 1. What the unauthenticated validate response carries — §15 item 6

**Answered: yes. `meta.order_id` is present on the unauthenticated response, and beside it five more identifiers, three names and the buyer's email address.**

### What was posed

```
POST https://api.lemonsqueezy.com/v1/licenses/validate
Accept: application/json
Content-Type: application/x-www-form-urlencoded
license_key=<the key>
```

**No `Authorization` header on any of the four calls.** Four calls: a pre-existing key whose status is `active`, a pre-existing key whose status is `inactive`, a key that does not exist, and a call with no `license_key` parameter at all. A fifth call added an `instance_id` that does not exist. **Every one is a read; the store's own record proves it** — all eleven licence keys carry the same `updated_at` after this work as before it, and the two that were validated read `2026-03-20T09:35:55` and `2026-05-05T11:06:28` at both inventories.

### A — a key whose status is `active` (licence key `100008`, order `100005`)

HTTP 200:

```
{"valid":true,"error":null,"license_key":{"id":100008,"status":"active","key":"[test-mode licence key, redacted]","activation_limit":2,"activation_usage":2,"created_at":"2026-03-20T10:58:19.000000Z","expires_at":null,"test_mode":true},"instance":null,"meta":{"store_id":100012,"order_id":100005,"order_item_id":100010,"variant_id":100003,"variant_name":"Default","product_id":100013,"product_name":"Smoozy","customer_id":100007,"customer_name":"[redacted]","customer_email":"buyer-1@example.invalid"}}
```

### B — a key whose status is `inactive`, never activated (licence key `100009`, order `100006`)

HTTP 200:

```
{"valid":true,"error":null,"license_key":{"id":100009,"status":"inactive","key":"[test-mode licence key, redacted]","activation_limit":2,"activation_usage":0,"created_at":"2026-03-20T09:35:55.000000Z","expires_at":null,"test_mode":true},"instance":null,"meta":{"store_id":100012,"order_id":100006,"order_item_id":100011,"variant_id":100003,"variant_name":"Default","product_id":100013,"product_name":"Smoozy","customer_id":100007,"customer_name":"[redacted]","customer_email":"buyer-1@example.invalid"}}
```

### C — the control: a key that does not exist

`license_key=00000000-0000-0000-0000-000000000000`, HTTP **404**:

```
{"valid":false,"error":"license_key not found."}
```

### D — the second control: no `license_key` parameter

HTTP **422**:

```
{"message":"The license key field is required.","errors":{"license_key":["The license key field is required."]}}
```

### E — a real key with an `instance_id` that does not exist

HTTP **404**:

```
{"valid":false,"error":"instance_id not found."}
```

### What this settles, and the five things beside it

**Item 6 is answered.** Licence service §15 item 6 asks *"Whether the seller's unauthenticated validate response carries the order identifier."* **It does — `meta.order_id`, on both good keys, matching the order the authenticated API attributes the key to.** §9's one-day hole in Row 237's immediacy therefore closes on its mechanical side: the adapter's single first-presentation call already returns the order the key belongs to, so nothing further is needed to identify which order to check. **What that call does *not* return is anything about the order's refund state**, which is the next paragraph's problem and item 5's.

**Five things came back that no page read had recorded, and a builder needs all five.**

1. **`license_key.test_mode` is in the response and is in no documented example.** The unauthenticated call therefore distinguishes a test-mode key from a live one, without a credential. §8's first presentation can refuse a test key outright rather than admitting one.
2. **The count field has a different name on this API than on the other one.** The unauthenticated response calls it `activation_usage`; the authenticated *License Key Object* for the very same key calls it `instances_count` (observed on both, values agreeing at 2 and 0). §7 holds slots itself and reads neither, but the pair is exactly the sort of thing an adapter gets wrong once.
3. **`valid: true` against `status: "inactive"`.** §8's rule — *"It reads `status`, not `valid`"*, with `active` and `inactive` both good — is confirmed rather than corrected, and the vendor's own License API page says why: *"inactive - The license key is valid but has no activations."*
4. **A non-recognition carries no `meta` object at all.** Body C has four fields and no order, no product, no store. So *not issued by the seller* and *issued but refunded* are not two values of one field; they are two different response shapes, and only one of them can be reasoned about.
5. **Two different error shapes come out of one endpoint.** C and E return `{"valid":…,"error":…}`; D returns `{"message":…,"errors":{…}}` with no `valid` key at all. **§8's rule that *"a shape the adapter cannot read is not a non-recognition"* now has a real second shape behind it**, produced by nothing worse than a malformed request.

**And one trap, from E.** The key in E is good and the response says `"valid":false`. **Passing an `instance_id` the seller does not know turns a valid licence into an invalid answer**, with a 404 and no `meta`. §8 never activates anything at the seller and so holds no instance identifiers, which is what keeps this from mattering — but it only holds while that stays true.

**The response also carries the customer's name and email address on an unauthenticated call**, confirming by observation what §8 records and refuses to read: *"It never sends the customer's email or name anywhere, though the vendor's response carries both."* It carries both, to anybody holding the key.

**What a negative would have looked like:** a `meta` object with no `order_id`, or no `meta` object on a good key. **Control C produced exactly that shape** — a body with no `meta` at all — which is what makes A and B a test rather than a formality.

### What could not be posed here, and what it would take

**The response for a `disabled` key, and the response for a key whose order has been refunded.** Neither state exists anywhere in this store: all eleven keys read `disabled: false`, and all twenty orders read `refunded: false`. Producing either means writing to a record that was already there, which the bounds forbid outright.

**What it would take:** a licence key of the tester's own, which requires an order of the tester's own, which requires a product — and the API can create none of the three. See item 5 immediately below; the blockage is the same one.

**So the shape of a refunded key's validate response is unobserved and remains so.** §9 does not rest on it — the lock rests on Smoozy's own record and the reconciliation reads the seller authenticated — but item 6's *"such a key is accepted until the next reconciliation pass"* cannot be shown to close from the unauthenticated call alone, because **nothing observed says whether a refund changes `status` or `valid` at all.** The order identifier is there; what the seller says about that order on this endpoint is not.

---

## 2. The delivered `order_refunded` body — §15 item 5

**Not posed. No part of it. This section says why, exactly what stopped it, and what would be needed.**

### Why

A delivered `order_refunded` body requires a refund, and a refund requires an order. There are two routes to one and both are closed.

**Route one — refund an order that is already in the store.** All twenty are the Creator's own. The endpoint exists (`POST /v1/orders/{id}/refund`, listed as *"Issue a refund"* in the vendor's API navigation). **The bound is absolute: nothing pre-existing is touched.** A refund is a write to a record that was already there, and it is irreversible. Not attempted.

**Route two — make an order of this work's own, refund it, delete both.** **The API cannot create one, and could not delete it if it could.** The vendor's own API navigation (`https://docs.lemonsqueezy.com/api`, read 2026-08-17) lists, in full:

- Products: *"The product object / Retrieve a product / List all products"* — **no create, no delete**
- Variants: *"The variant object / Retrieve a variant / List all variants"* — **no create, no delete**
- Orders: *"The order object / Retrieve an order / List all orders / Generate order invoice / Issue a refund"* — **no create, no delete**
- License Keys: *"The license key object / Retrieve a license key / Update a license key / List all license keys"* — **no create, no delete**

So a licence key can only be brought into existence by a purchase of a product, and **once it exists neither it nor its order can be removed by any API route.** **Corrected 2026-08-22: this sentence read *"a purchase of a product that only the dashboard can make"*.** The API does create checkouts — `POST /v1/checkouts`, *"Creates a unique checkout for a specific variant with specified attributes"* (`https://docs.lemonsqueezy.com/api/checkouts/create-checkout`, read 2026-08-20), four entries below Checkouts in the same navigation the list above is enumerated from — and a checkout URL for an already-published variant can be made with a test-mode key and nothing else. [The refund event](The%20refund%20event%20—%20the%20routes%20to%20it,%20read%20rather%20than%20tried.md) Step 1 already states it. **What the API cannot create is products, variants, orders and licence keys**, which is the list above, and **the bound that stopped this work is permanence rather than the dashboard.** Creating one would have left a permanent order and a permanent key in the Creator's store that this work could not take back, which the inventory bound forbids in as many words. Not attempted.

**Route three — the vendor's own manual simulation — is dashboard-only.** The page the brief quotes says, in full: *"For more manual testing, you can simulate webhook events for any test mode order or subscription. Simply choose an event you'd like to receive a webhook for and send it manually to your app from your Lemon Squeezy admin."* **`from your Lemon Squeezy admin`** — there is no API for it, and the ten events it lists are reachable no other way. Reaching the admin means signing into the dashboard of a store with live data and moving its test toggle. Outside this work's bounds. Not attempted.

**What a negative would have looked like** is not a meaningful question for a test that was never run, and saying otherwise would be the worst thing this document could do. **Item 5 stands exactly where §15 leaves it**, in all three of its parts: which fields the delivered body carries, how many times the event fires, and the `partial_refund` status against the worked example's `"status": "paid"`.

### What was established without a refund, and what it is worth

**Three things, and none of them is the delivered body.**

**One — `order_refunded` is subscribable on this store, and was subscribed to.** Webhook `100023` was created against it, HTTP 201, read back with `"events":["order_refunded"]` and `"test_mode":true`, and deleted. So nothing about the event name or the registration is in doubt; only what arrives.

**Two — the Order object as this store's API returns it, whole.** The vendor states the delivered body is this object: *"The webhook body will be a JSON:API resource object that relates to the event (e.g. "order" events will be sent an order object …)"*. **That statement is not tested here** — this is the object read from `GET /v1/orders/100005`, not a body any webhook delivered — and it must be labelled that way wherever it is used. With that said, here is a real order from this store, complete:

```
{"jsonapi":{"version":"1.0"},"links":{"self":"https:\/\/api.lemonsqueezy.com\/v1\/orders\/100005"},"data":{"type":"orders","id":"100005","attributes":{"store_id":100012,"customer_id":100007,"affiliate_id":null,"referral_amount":null,"identifier":"00000000-0000-0000-0000-000000000000","order_number":100001,"user_name":"[redacted]","user_email":"buyer-1@example.invalid","currency":"EUR","currency_rate":"1.15447046","tax_name":"VAT","tax_rate":"0.00","tax_inclusive":false,"status":"paid","status_formatted":"Paid","refunded":false,"refunded_at":null,"subtotal":2000,"discount_total":0,"tax":0,"setup_fee":0,"total":2000,"refunded_amount":0,"subtotal_usd":2309,"discount_total_usd":0,"tax_usd":0,"setup_fee_usd":0,"total_usd":2309,"refunded_amount_usd":0,"subtotal_formatted":"€20.00","discount_total_formatted":"€0.00","tax_formatted":"€0.00","setup_fee_formatted":"€0.00","total_formatted":"€20.00","refunded_amount_formatted":"€0.00","first_order_item":{"id":100010,"order_id":100005,"product_id":100013,"variant_id":100003,"price_id":100004,"product_name":"Smoozy","variant_name":"Default","price":2000,"quantity":1,"created_at":"2026-03-20T10:58:19.000000Z","updated_at":"2026-03-20T10:58:19.000000Z","test_mode":true},"urls":{"receipt":"https:\/\/app.lemonsqueezy.com\/my-orders\/00000000-0000-0000-0000-000000000000?expires=1786983037&signature=[redacted]"},"created_at":"2026-03-20T10:58:17.000000Z","updated_at":"2026-03-20T10:59:19.000000Z","test_mode":true},"relationships":{"store":{…},"customer":{…},"affiliate":{…},"order-items":{…},"subscriptions":{…},"license-keys":{"links":{"related":"https:\/\/api.lemonsqueezy.com\/v1\/orders\/100005\/license-keys","self":"https:\/\/api.lemonsqueezy.com\/v1\/orders\/100005\/relationships\/license-keys"}},"discount-redemptions":{…}},"links":{"self":"https:\/\/api.lemonsqueezy.com\/v1\/orders\/100005"}}}
```

*(the seven `relationships` entries other than `license-keys` are elided only because each is the same two-link pair; identifiers and personal fields use the publication substitutions above)*

**All five fields §9's full-against-partial rule touches are present on a real order in this store** — `status`, `refunded`, `refunded_at`, `refunded_amount`, `total` — and so are `refunded_amount_usd` and `refunded_amount_formatted`, which no page read enumerates. **What is still unobserved is what any of them say after a refund**, which is the whole of item 5's first part.

**And one field the reading did not have.** `urls.receipt` carries a signed, expiring link straight into the customer's own order page at `app.lemonsqueezy.com/my-orders/…`. §8 sends a user to `app.lemonsqueezy.com/my-orders` and has them fetch a magic link by email; **this is a second route to the same page that the seller already puts inside the order record**. It is not the service's to use — the service never holds an order object for a user, only a digest — but it is worth knowing that the reference §8 keeps opaque has a customer-facing address attached to it at the seller.

**Three — the order-to-key relationship is a live link, not just a documented parameter.** `relationships.license-keys.links.related` on the order above resolves to `/v1/orders/100005/license-keys`. §9's *"From an order to a key, the chain is documented end to end"* rests on the `order_id` filter; the store also exposes it as a relationship on the order itself.

### What it would take

**Any one of these, and none is this work's to arrange.**

- **A Lemon Squeezy account that is not the Creator's.** A throwaway store would put every one of item 5's three parts inside reach — create a licensed product, buy it with the vendor's own test card `4242 4242 4242 4242`, refund it in full, refund another in part, count the deliveries. Nothing in the bounds would be near it.
- **Or the Creator creating one scratch test-mode product in this store and accepting that the resulting test order and licence key stay in test-mode data for good**, since no API route removes either. That is a decision about the Creator's own store and it is theirs, not an agent's.
- **Or the Creator, in the dashboard with test mode on, using *simulate webhook events* against one of the twenty existing test orders**, pointed at any endpoint. That one changes no record at all and would answer the "which fields" part — though not the "how many times" part, since a simulation is one deliberate send.

---

## 3. The events this store will actually accept — §15 item 9

**Answered, and it contradicts what was read. The store accepts two dispute events. The vendor's pages document neither.**

Licence service §15 item 9 records: *"No chargeback, dispute or charge-reversal event exists in the vendor's set, confirmed across all 232 pages."* §9 repeats it: *"No chargeback, dispute or charge-reversal event exists anywhere in the vendor's seventeen-event list."* **The pages are still as the reading found them. The store is not.**

### What was posed

For each candidate name, one registration:

```
POST https://api.lemonsqueezy.com/v1/webhooks
Authorization: Bearer <test-mode key>
Accept: application/vnd.api+json
Content-Type: application/vnd.api+json

{"data":{"type":"webhooks","attributes":{
  "url":"https://webhook.site/00000000-0000-0000-0000-000000000000",
  "events":["<candidate>"],
  "secret":"zzsmoozyfact9secret123456"},
 "relationships":{"store":{"data":{"type":"stores","id":"100012"}}}}}
```

**Eighty-eight distinct names were tried. Twenty were accepted with HTTP 201 and a webhook object. Sixty-eight were refused with HTTP 422 and one identical body:**

```
{"jsonapi":{"version":"1.0"},"errors":[{"detail":"The selected events.0 is invalid.","source":{"pointer":"\/data\/attributes\/events\/0"},"status":"422","title":"Unprocessable Entity"}]}
```

### The twenty the store accepted

The seventeen the vendor's *Event Types* page lists — `order_created`, `order_refunded`, `customer_updated`, `subscription_created`, `subscription_updated`, `subscription_cancelled`, `subscription_resumed`, `subscription_expired`, `subscription_paused`, `subscription_unpaused`, `subscription_payment_success`, `subscription_payment_failed`, `subscription_payment_recovered`, `subscription_payment_refunded`, `license_key_created`, `license_key_updated`, `affiliate_activated` — **and three more it does not:**

| Event name | On *Event Types*? | Webhook created | Read back as |
|---|---|---|---|
| **`dispute_created`** | **no** | `100017`, 10:07:38Z | `{"store_id":100012,"url":"https:\/\/webhook.site\/00000000-…","events":["dispute_created"],"last_sent_at":null,"created_at":"2026-08-17T10:07:38.000000Z","updated_at":"2026-08-17T10:07:38.000000Z","test_mode":true}` |
| **`dispute_resolved`** | **no** | `100018`, 10:08:04Z | `{"store_id":100012,"url":"https:\/\/webhook.site\/00000000-…","events":["dispute_resolved"],"last_sent_at":null,"created_at":"2026-08-17T10:08:04.000000Z","updated_at":"2026-08-17T10:08:04.000000Z","test_mode":true}` |
| **`subscription_plan_changed`** | **no** | `100036`, 10:09:04Z | `{"store_id":100012,"url":"https:\/\/webhook.site\/00000000-…","events":["subscription_plan_changed"],"last_sent_at":null,"created_at":"2026-08-17T10:09:04.000000Z","updated_at":"2026-08-17T10:09:04.000000Z","test_mode":true}` |

`dispute_created`'s creation response in full, HTTP 201:

```
{"jsonapi":{"version":"1.0"},"links":{"self":"https:\/\/api.lemonsqueezy.com\/v1\/webhooks\/100017"},"data":{"type":"webhooks","id":"100017","attributes":{"store_id":100012,"url":"https:\/\/webhook.site\/00000000-0000-0000-0000-000000000000","events":["dispute_created"],"last_sent_at":null,"created_at":"2026-08-17T10:07:38.000000Z","updated_at":"2026-08-17T10:07:38.000000Z","test_mode":true},"relationships":{"store":{"links":{"related":"https:\/\/api.lemonsqueezy.com\/v1\/webhooks\/100017\/store","self":"https:\/\/api.lemonsqueezy.com\/v1\/webhooks\/100017\/relationships\/store"}}},"links":{"self":"https:\/\/api.lemonsqueezy.com\/v1\/webhooks\/100017"}}}
```

**The registration is not a formality that was silently dropped.** `GET /v1/webhooks/100017`, issued afterwards, returned the same `"events":["dispute_created"]`, and the store's webhook list at 10:11Z carried all twenty registrations with their event arrays intact.

### The control, and it is what makes this a test

**Sixty-eight names were refused with the identical sentence**, including every other spelling of the same idea that could be thought of: `order_chargeback`, `chargeback_created`, `chargeback`, `order_chargeback_created`, `chargeback_resolved`, `dispute`, `dispute_opened`, `dispute_open`, `dispute_updated`, `dispute_closed`, `dispute_won`, `dispute_lost`, `dispute_lost_final`, `dispute_deleted`, `dispute_accepted`, `dispute_reversed`, `dispute_refunded`, `dispute_reopened`, `dispute_pending`, `dispute_needs_response`, `dispute_under_review`, `dispute_charge_refunded`, `dispute_alert`, `dispute_inquiry`, `dispute_evidence_submitted`, `disputes_created`, `order_disputed`, `order_dispute_created`, `payment_disputed`, `charge_disputed`, `refund_created`, `refund_updated`, `refund_issued`, `refund_completed`, `order_refund`, `order_partially_refunded`, `order_fully_refunded`, `order_updated`, `order_deleted`, `order_paid`, `order_pending`, `order_failed`, `customer_created`, `customer_deleted`, `product_created`, `variant_created`, `license_key_deleted`, `license_key_disabled`, `license_key_expired`, `license_key_activated`, `license_key_deactivated`, `affiliate_created`, `affiliate_updated`, `affiliate_deactivated`, `affiliate_payout_created`, `payout_created`, `usage_record_created`, `discount_redemption_created`, `store_updated`, `subscription_deleted`, `subscription_past_due`, `subscription_payment_pending`, `subscription_trial_started`, `subscription_trial_ended`, `order_receipt_sent`, `email_subscriber_created`, `dispute_created_test`, and the deliberate nonsense `zz_not_an_event`.

**What a negative would have looked like:** `dispute_created` refused with the same 422 as `order_chargeback` and `dispute_opened`. **Sixty-eight names produced exactly that**, two of them one character away from an accepted one. That is what makes the two acceptances an observation rather than a slip.

### The limit on this answer, and it is a real one

**The endpoint does not enumerate its allowed values.** Every refusal is the same sentence: *"The selected events.0 is invalid."* No page lists them either — the vendor's *Event Types* page opens *"This is the full list of webhook events available in Lemon Squeezy"* and then lists seventeen, three of which are wrong by omission.

**So twenty is a floor, not the list.** A twenty-first name that nobody guessed would have looked exactly like the sixty-eight refusals. Anything downstream that needs *the complete set* still needs a route that enumerates, and this work found none.

**One corroboration that the omission is the page's and not the store's.** `subscription_plan_changed` is absent from *Event Types* and **present, by name, on the vendor's own *Simulate webhook events* page**, in its list of simulable subscription events. So the vendor contradicts itself about what its own event set contains, on two pages read the same day, and the store agrees with the second one. The two dispute events are on neither page.

### What this changes for `Licence service`, and what it does not

**It changes item 9's premise and it does not answer item 9's question.**

- §15 item 9 and §9's *Chargebacks* paragraph both rest on *no such event exists*. **On this store, two do**, so the reason a chargeback produces no signal is not that the seller has no event for it.
- §9's reasons table gives **charged back** as *"a seller refund event the seller attributes to a dispute. **Nothing documented says any event carries that attribution**, so unless one does, a chargeback reaches the record as *refunded* and this reason is never set."* **That is now the wrong reason for the right caution.** An event named for a dispute exists; whether it fires, what it delivers, and whether it names an order or a key are all unobserved.

**What is unobserved, and cannot be observed from this store.** Whether `dispute_created` or `dispute_resolved` ever fire; whether they fire in test mode at all; what object either delivers; whether that object reaches an order and therefore a key; and whether they are available on every plan or only some. **Whether a dispute can be caused is unresolved rather than impossible. Corrected 2026-08-22: this read *"**A dispute cannot be caused** — it is a customer's bank's act, there is no test card documented for one, and no simulation is offered for either name."*** In live mode it is the customer's bank's act, and no simulation is offered for either name. **But a test card for one is documented, by the vendor's own link.** Its *Test Mode* page closes its six-card list with *"See the full list of supported test cards"*, whose href is `https://docs.stripe.com/testing#cards` (read 2026-08-20), and that page's *Simulate a dispute* section carries `4000000000000259` — *"With default account settings, charge succeeds, only to be disputed as fraudulent."* **The route to trying it is one extra checkout inside [The refund event](The%20refund%20event%20—%20the%20routes%20to%20it,%20read%20rather%20than%20tried.md)'s Route B**, entering that number instead of `4242 4242 4242 4242`, with `dispute_created` registered. Registering for them costs nothing and risks nothing, which is a decision §9 can now take; relying on them is not something this work licenses.

---

## Where this disagrees with what was read

**Five places, gathered here because the runner needs them in one list. No input document is edited by this one.**

| # | What the reading records | What was observed |
|---|---|---|
| 1 | Licence service §15 item 9 and §9 — *"No chargeback, dispute or charge-reversal event exists in the vendor's set, confirmed across all 232 pages"* | **The pages still carry none. The store accepts two: `dispute_created` and `dispute_resolved`**, both registered, both read back, against sixty-eight other names refused with one identical sentence. [Item 9](#3-the-events-this-store-will-actually-accept--15-item-9) |
| 2 | Licence service §15 item 6 — *"no page states whether the unauthenticated response includes it"* | **It includes it.** `meta.order_id`, plus `store_id`, `order_item_id`, `variant_id`, `product_id`, `customer_id` and the customer's name and email. The vendor's own *Validate a license key* example also shows `meta.order_id`, so the page the item calls silent is not. [Item 6](#1-what-the-unauthenticated-validate-response-carries--15-item-6) |
| 3 | The earlier sitting of 2026-08-17 — the toggle check was made by confirming *"`meta.test_mode: true`"* on `GET /v1/users/me` **and** `GET /v1/stores` | **`GET /v1/stores` carries no `test_mode` field at all**, in either shape, on the list or the single store. The conclusion holds and is stronger than that check; the check itself cannot be run as described. [Section 0](#0-the-toggle-and-the-keys-world) |
| 4 | The vendor's *Event Types* page — *"This is the full list of webhook events available in Lemon Squeezy"*, seventeen rows | **Three accepted names are missing from it**, and one of the three, `subscription_plan_changed`, is listed on the vendor's own *Simulate webhook events* page. The vendor contradicts itself. [Item 9](#3-the-events-this-store-will-actually-accept--15-item-9) |
| 5 | Licence service §8 — the seller's response *"carries both"* the customer's name and email, which the adapter refuses to read | **Confirmed, and it is worse than the reading implies: it is the *unauthenticated* response that carries them**, so anyone holding a licence key can read the buyer's name and address without a credential. §8's refusal to read them is unchanged; what changes is that the exposure is not behind the service's own key. [Item 6](#1-what-the-unauthenticated-validate-response-carries--15-item-6) |

**And two places where what was read held exactly.** §8's *"It reads `status`, not `valid`"* with `inactive` counted good, which the vendor's License API page states in the same terms and the store returned; and the documented *"60 requests per minute"* on the License API, which is on the page and was nowhere near approached — **five calls were made to that API in total**, so nothing here tests the limit and nothing here should be read as testing it.

---

## The store, after everything was deleted

Every webhook created for this work was deleted, each one re-read first and matched against the throwaway address before the delete was sent, with the script refusing any registration whose URL did not match:

```
deleted 100036 -> [HTTP 204]     deleted 100026 -> [HTTP 204]
deleted 100034 -> [HTTP 204]     deleted 100027 -> [HTTP 204]
deleted 100035 -> [HTTP 204]     deleted 100024 -> [HTTP 204]
deleted 100033 -> [HTTP 204]     deleted 100025 -> [HTTP 204]
deleted 100032 -> [HTTP 204]     deleted 100022 -> [HTTP 204]
deleted 100030 -> [HTTP 204]     deleted 100023 -> [HTTP 204]
deleted 100031 -> [HTTP 204]     deleted 100021 -> [HTTP 204]
deleted 100028 -> [HTTP 204]     deleted 100019 -> [HTTP 204]
deleted 100029 -> [HTTP 204]     deleted 100020 -> [HTTP 204]
                                 deleted 100018 -> [HTTP 204]
                                 deleted 100017 -> [HTTP 204]
```

The same inventory was then re-run at **2026-08-17 10:11:54Z**, against the one taken at **10:00Z** before anything was created.

| Collection | Before | After |
|---|---|---|
| Products | 1 (`100013`) | **1 (`100013`)** |
| Variants | 1 (`100003`) | **1 (`100003`)** |
| Orders | 20 | **20**, the same twenty identifiers |
| Order items | 20 | **20**, the same twenty identifiers |
| Licence keys | 11 | **11**, the same eleven identifiers |
| Licence key instances | 5 | **5**, the same five identifiers |
| **Webhooks** | **0** | **0** |
| Subscriptions | 0 | **0** |
| Discounts | 0 | **0** |
| Files | 3 | **3**, the same three identifiers |
| Customers | 4 | **4**, the same four identifiers |
| Checkouts | 0 | **0** |

**The two are identical, collection by collection and identifier by identifier.**

**And the stronger check, because a validate call could in principle have touched a key.** All eleven licence keys read the same `status`, the same `disabled: false`, the same `instances_count` and the same `updated_at` after this work as before it — including the two that were validated, whose `updated_at` still read `2026-03-20T09:35:55.000000Z` and `2026-05-05T11:06:28.000000Z`. **The unauthenticated validate endpoint changed nothing about the keys it was pointed at.**

**No order was refunded, no key was disabled, no product or variant or file was altered, and the store's live side was never opened.** The throwaway endpoint was destroyed and never received a request.

---

*Written by Claude Opus 5 (Claude Code), an agent on the architecture runner's brief, 2026-08-17.*
