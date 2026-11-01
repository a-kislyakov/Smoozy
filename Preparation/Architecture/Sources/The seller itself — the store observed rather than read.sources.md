# Sources — The seller itself — the store observed rather than read

*Where the **documented** claims in [The seller itself](../The%20seller%20itself%20—%20the%20store%20observed%20rather%20than%20read.md) came from. That document carries each claim's label in place; this file carries the citation behind it, and what it does not cover. **Everything it establishes by running something is in the document itself and is not repeated here** — this file covers vendor pages only.*

---

## What no citation here covers

Four limits apply to every entry and are not repeated under each.

- **Every address below was read on 2026-08-17, once, and not re-fetched by a second agent.** No citation check has been run over this file. Each quote is worth its date and no more.
- **The vendor's help centre carries no dates of any kind.** Not a published date, not a last-updated date, not a version. A help-centre quote cannot be tied to a moment beyond the day it was read, and the same page may have said something else a week earlier. This is the same limit Licence service (`Preparation/Architecture/Licence service.md`, withdrawn) §15 item 10 already records for the seller.
- **These pages describe Lemon Squeezy in general. The document they support describes one store.** Where the two disagree — and in three places they do — the document records the disagreement and adjudicates nothing.
- **Every page was fetched over plain HTTPS with a browser user-agent.** `docs.lemonsqueezy.com` refuses a default fetcher with HTTP 403; that is a fetching detail, not a fact about the vendor.

---

## `docs.lemonsqueezy.com` — the five pages quoted

| Page | Address | HTTP | What it is quoted for |
|---|---|---|---|
| Test Mode | https://docs.lemonsqueezy.com/help/getting-started/test-mode | 200 | the key's world; test products not public; Copy to Live Mode |
| Simulate Webhook Events | https://docs.lemonsqueezy.com/help/webhooks/simulate-webhook-events | 200 | manual simulation is dashboard-only; test and live webhooks separate; the ten simulable events |
| Event Types | https://docs.lemonsqueezy.com/help/webhooks/event-types | 200 | the seventeen-event list said to be full |
| License API (overview) | https://docs.lemonsqueezy.com/api/license-api | 200 | the four licence key statuses; the rate limit; the request requirements |
| Validate a License Key | https://docs.lemonsqueezy.com/api/license-api/validate-license-key | 200 | the endpoint, its parameters, and its example response |
| API Reference (index) | https://docs.lemonsqueezy.com/api | 200 | the complete endpoint navigation, used for what the API cannot do |

All read **2026-08-17**.

---

## The quotes, in the document's order

### Test Mode — `https://docs.lemonsqueezy.com/help/getting-started/test-mode`

**Label: documented (read 2026-08-17).**

- On what a test-mode key sees — quoted in the document's Section 0, and already carried in the first lines of `~/.smoozy-keys/lemonsqueezy-test.txt`:

  > "API keys created in test mode will interact with test mode store data only. When you are ready to go live with your integration, make sure to create an API key in live mode and use that in your production application."

- On whether a published test-mode product is public — the load-bearing quote for the bound that had to clear before anything was written. It appears as a step in the page's *Checkout testing* list:

  > "Make sure you have a "published" product in test mode"
  > "Don't worry, this won't be visible to the public, only to you."

- On the only route from test mode to live mode:

  > "Products created in test mode won't automatically transfer to live mode when your store is activated. However, you can easily copy them, including discounts, to live mode using the "Copy to Live Mode" option in the product or discount menu."

- The test card the document names as what a purchase *would* have used, had one been made — none was:

  > "Visa: 4242 4242 4242 4242"

  and, on the same page, the warning that goes with it:

  > "Do not use real card details for payment testing, as this may be interpreted as fraudulent activity and may result in your store being suspended."

- Two further sentences from the same page, not quoted in the document but bearing on how far it can be read: *"Order receipt emails in test mode will be sent to you and your team members, regardless of what you enter as the customer email address during checkout"*, and *"File downloads are disabled for all test mode purchases."*

**What this page does not cover.** It says nothing about what the dashboard's live/test toggle does to a key that already exists — which is the silence Section 0 of the document exists to fill by observation. It says nothing about deleting test-mode data, and this work found no page that does.

### Simulate Webhook Events — `https://docs.lemonsqueezy.com/help/webhooks/simulate-webhook-events`

**Label: documented (read 2026-08-17).**

- On where simulation lives — the sentence that closed item 5:

  > "For more manual testing, you can simulate webhook events for any test mode order or subscription. Simply choose an event you'd like to receive a webhook for and send it manually to your app from your Lemon Squeezy admin."

- On mode separation, which is why a test-mode webhook could not have carried live data to a throwaway endpoint:

  > "Test and live mode webhooks are kept separate within your account, meaning a test mode webhook will only be triggered for test mode data and vice versa."

- The page's own list of what can be simulated, quoted for the one name in it that the *Event Types* page omits. Under *Subscription events*: `subscription_created`, `subscription_updated`, `subscription_cancelled`, `subscription_resumed`, `subscription_expired`, `subscription_paused`, `subscription_unpaused`, **`subscription_plan_changed`**. Under *Order events*: `order_created`, `order_refunded`.

  > "The following events can be simulated:"

**What this page does not cover.** It names no API for simulation and this work found none. It does not say whether a simulated `order_refunded` for an unrefunded order delivers `refunded: false` or refuses — which is one of the things the route would have had to establish.

### Event Types — `https://docs.lemonsqueezy.com/help/webhooks/event-types`

**Label: documented (read 2026-08-17).**

- The page's opening claim, which the document contradicts by observation:

  > "This is the full list of webhook events available in Lemon Squeezy."

  followed by seventeen rows: `order_created`, `order_refunded`, `customer_updated`, `subscription_created`, `subscription_updated`, `subscription_cancelled`, `subscription_resumed`, `subscription_expired`, `subscription_paused`, `subscription_unpaused`, `subscription_payment_success`, `subscription_payment_failed`, `subscription_payment_recovered`, `subscription_payment_refunded`, `license_key_created`, `license_key_updated`, `affiliate_activated`.

- The `order_refunded` row entire, quoted because Licence service §9 rests on it:

  > "order_refunded | Order object | Occurs when a full or partial refund is made on an order."

**What this page does not cover.** No dispute or chargeback event appears on it. `subscription_plan_changed` does not appear on it either, though the vendor's own *Simulate webhook events* page lists that name. The page carries no date.

### License API overview — `https://docs.lemonsqueezy.com/api/license-api`

**Label: documented (read 2026-08-17).**

- On the four statuses — quoted because it is the vendor's own statement of why `valid: true` and `status: "inactive"` are not a contradiction:

  > "inactive - The license key is valid but has no activations."
  > "active - The license key has one or more activations."
  > "expired - The license key's expiry date has passed, either because the related product had a defined license length or because the license's subscription has expired."
  > "disabled - The license key has been manually disabled from the Lemon Squeezy dashboard."

- On the rate limit Licence service §8 and §15 item 13 both lean on:

  > "API calls are rate limited to 60 requests per minute."

- On the request requirements, and on the API being separate:

  > "The License API is a separate API from the Lemon Squeezy API and has different requirements."
  > "All requests must include the Accept: application/json header."
  > "POST requests should be sent with the Content-Type: application/x-www-form-urlencoded header."

**What this page does not cover, and it is the reason the document calls the endpoint "unauthenticated" on observation rather than on the page.** The page's *Requests* section lists three requirements and **an `Authorization` header is not among them**; it never states in words that no credential is needed. The document's claim that the endpoint answers without one is **observed**, not documented — four calls were made with no `Authorization` header and all four were answered.

### Validate a License Key — `https://docs.lemonsqueezy.com/api/license-api/validate-license-key`

**Label: documented (read 2026-08-17).**

- The endpoint and its parameters:

  > "POST /v1/licenses/validate"
  > "Checks if a license key is valid and returns information about the license key and its associated instance."
  > "license_key — Required — The license key to validate."
  > "instance_id — If included, validate a license key instance, otherwise validate a license key. If no instance_id is provided, the response will contain "instance": null."

- On the `meta` object — the sentences that bear directly on Licence service §15 item 6:

  > "meta — Additional information about the license key, including the order, product, variant, and customer information."
  > "ID fields in the meta object can be used to fetch data related to the license key, such as relevant order/product information, from the Lemon Squeezy API. The meta object also contains convenience information from the license key, including product and variant names, and the customer's name and email address."

- **The page's own example response carries `"order_id": 2` inside `meta`.** Its `meta` block, as printed on the page:

  > `"meta": { "store_id": 1, "order_id": 2, "order_item_id": 3, "product_id": 4, "product_name": "Lemonade", "variant_id": 5, "variant_name": "Citrus Blast", "customer_id": 6, "customer_name": "John Doe", "customer_email": "…" }`

  **This matters to item 5's honesty.** §15 item 6 states that *"no page states whether the unauthenticated response includes it"*. On the page as read 2026-08-17, the example does include it. Whether the page changed between the reading of 2026-08-16 and this one cannot be established — the page carries no date — so the document records both the page and the observation and treats the observation as the answer.

**What this page does not cover.** Its example response does **not** carry the `test_mode` field that the real response carries, and its `meta` orders `product_id` before `variant_id` where the real response reverses them. It says nothing about what the response looks like for a `disabled` key, an `expired` key, or a key whose order has been refunded — the three shapes the document could not pose.

### API Reference index — `https://docs.lemonsqueezy.com/api`

**Label: documented (read 2026-08-17).**

Quoted for what it does **not** list, which is what stopped item 5. The navigation's own entries, verbatim and complete for the four resources that matter:

> "Products — The product object / Retrieve a product / List all products"
> "Variants — The variant object / Retrieve a variant / List all variants"
> "Orders — The order object / Retrieve an order / List all orders / Generate order invoice / Issue a refund"
> "License Keys — The license key object / Retrieve a license key / Update a license key / List all license keys"

Beside them, for contrast, the two resources that **do** carry a delete, which is why webhooks could be created and removed and nothing else could:

> "Discounts — The discount object / Create a discount / Retrieve a discount / Delete a discount / List all discounts"
> "Webhooks — The webhook object / Create a webhook / Retrieve a webhook / Update a webhook / Delete a webhook / List all webhooks"

And the same page on test mode:

> "You can build and test a full API integration with Lemon Squeezy using Test mode. Any API keys created in Test mode will interact with your test mode store data."

**What this page does not cover.** It is a navigation list, so an absence in it is evidence that no such page exists in the reference, not proof that no such route exists on the server. **No `POST /v1/products` and no `DELETE /v1/orders/{id}` was attempted** — a create that succeeded could not have been undone, which is precisely the bound that stopped the work.

---

## One address that is not the vendor's

**`webhook.site`.** `POST https://webhook.site/token` created an anonymous throwaway endpoint at 10:07:26Z on 2026-08-17; `GET https://webhook.site/token/{uuid}/requests` read its (empty) log; `DELETE https://webhook.site/token/{uuid}` destroyed it at 10:12Z with HTTP 204. **No fact in the document comes from that service's documentation** — it is a tool the work used, not a source it cites, and the vendor's own *Simulate webhook events* page happens to recommend it by name: *"To make it easier to check the data that is sent via webhooks, you can use a tool like webhook.site."* It received nothing.
