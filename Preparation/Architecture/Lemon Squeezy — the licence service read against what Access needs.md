# Lemon Squeezy — the licence service read against what Access needs

What Lemon Squeezy's own documentation stated on **2026-08-16** about Facts 9 and 10, clause by clause against Access (`Preparation/Specs/Access.md`, withdrawn). An **input** to the stage, like [The manuals](The%20manuals%20—%20keys,%20secrets,%20and%20what%20cannot%20be%20told%20apart.md) and [The computer itself](The%20computer%20itself%20—%20its%20name,%20its%20identity,%20and%20its%20clock.md): nothing here is a decision, a plan, or a recommendation. **The build-or-buy fork is not ruled here.**

Every claim carries a label: **documented**, **not stated**, or **disagreement**. Every documented claim carries the exact words, the address and the date read. A claim that could not be quoted is not here.

Rule 1's plain-English rule is suspended, per the stage's exception.

---

## Three limits, before anything else

**One. Nothing here was run. Every claim is a documented assertion, never an observation.** No account exists, no product exists, no key was issued, and not one request was made to `api.lemonsqueezy.com`. The distinction that [The computer itself](The%20computer%20itself%20—%20its%20name,%20its%20identity,%20and%20its%20clock.md) draws between documented and observed applies with full force: **every behaviour below is what the vendor says its service does**, and the three places where the vendor's own pages contradict each other or themselves are proof that this is not the same thing as knowing what it does.

**Two. This is one day's reading of a live, undated documentation set.** [Facts](Facts.md) records Fact 10 as "the fastest-changing fact in the register — a page read in August says nothing about September", and the vendor's own record confirms the set moves: the API changelog's most recent entry is dated **July 29th, 2026**, eighteen days before this reading, with **three dated entries carrying four changes inside the last seven months**. Separately, **not one help-centre page carries a date of any kind** — no published date, no revised date, no version — and the page HTML carries no `datePublished`, `dateModified`, `article:modified_time` or `lastmod` metadata either. So every help-centre claim below can be dated only by the day it was read, and nothing distinguishes a sentence written last month from one written in 2021.

**Three. Read here from the vendor's pages alone, and their retrieval is not uniform.** `docs.lemonsqueezy.com` returned **HTTP 403 Forbidden** to the agent's own page-fetching tool, with no body retrieved, and **HTTP 200** to direct HTTPS retrieval of the identical address — 251,596 bytes, an identical byte count with and without a browser user-agent string. Both routes were reproduced by the citation check. Extending the second route to the **entire documentation set — 232 pages** enumerated from the site's own navigation — returned **HTTP 200 on every one, zero failures**. The site serves no `robots.txt` and no `sitemap.xml`; both return the site's own 404 page body. Two consequences: **a later reader using the first route will wrongly conclude this documentation is unavailable, and it is fully available**; and nothing here rests on any source that is not `docs.lemonsqueezy.com`. Forums, wrappers, SDK repositories, comparison pages and search-result summaries were excluded by the brief and none is cited.

**A note on this document's own scope against that set.** This reading opened **35** of the 232 pages. The citation check of 2026-08-16 read all 232. Where a negative claim below was re-tested against the full set, it says so and names the wider search; where a quote below comes from a page this reading's first pass did not open, it says so and names the check that surfaced it. **Every such quote was afterwards confirmed on the live page on the same date**, so nothing here is carried on another agent's word alone.

---

## Fact 9 — what the licence service must be able to do

### Clause 1 — hold **two** computer slots against a key

**Documented, and it is held against the key alone.** The mechanism is a per-key activation count with a configurable ceiling.

> "You can control how long a license key will be considered “valid” and how many times it can be activated using the “License length” and “Activation limit” settings."
> — *Generating License Keys*, `https://docs.lemonsqueezy.com/help/licensing/generating-license-keys`, read 2026-08-16

> "Then, let’s imagine your product costs $9 and the activation limit is set to “1” so that the customer can only use your software product on one computer. If they try to activate the same license on more than one computer it won’t work as the license will have reached its activation limit."
> — same page

> "With this license, the software can be used on up to 3 computers before the license key can no longer be activated."
> — same page

> "Lemon Squeezy will not allow new license key activations once the activation limit has been reached."
> — *Validating License Keys With the License API*, `https://docs.lemonsqueezy.com/guides/tutorials/license-keys`, read 2026-08-16

The ceiling is an attribute of the key object and is writable:

> "activation_limit — A number or null. The activation limit of this license key. Assign null to set the activation limit to “unlimited”."
> — *Update a License Key*, `https://docs.lemonsqueezy.com/api/license-keys/update-license-key`, read 2026-08-16

> "activation_limit — The activation limit of this license key."
> "instances_count — A count of the number of instances this license key has been activated on."
> — *The License Key Object*, `https://docs.lemonsqueezy.com/api/license-keys/the-license-key-object`, read 2026-08-16

**The standing constraint is satisfied on this clause.** Activation takes the key string and a label, and nothing else:

> "license_key Required — The license key to activate."
> "instance_name Required — A label for the new instance to identify it in Lemon Squeezy."
> — *Activate a License Key*, `https://docs.lemonsqueezy.com/api/license-api/activate-license-key`, read 2026-08-16

No account, no registration and no email is required by the endpoint. **Setting the limit to 2 is a product-form setting the Creator makes once, not an act Smoozy performs.**

**What is narrower than the promise.** A slot is not held against a *computer*. It is held against an **activation event**:

> "Each license key can have one or many license key instances, which are created every time you activate the license key."
> — tutorial, address above

> "A new license key instance is created for each successful activation request, each with a unique ID (an instance is a recorded “use” of a license key)."
> — tutorial, address above

Nothing in any page states that two activation requests carrying the same `instance_name` are collapsed into one instance. **Confirmed across all 232 pages** by the citation check: `instance_name` occurs on exactly two pages, four times in total, described only as "A label for the new instance to identify it in Lemon Squeezy" and "it will be used to give the license key instance a name in your Lemon Squeezy dashboard" — no uniqueness constraint, no matching, no de-duplication anywhere. That is Fact 11's first question and is answered below.

**The vendor's prose pulls the other way and must not be read as a binding.** The help page talks in computers — "the customer can only use your software product on one computer", "the software can be used on up to 3 computers". The mechanism pages describe only per-activation-request instances, and **nothing in the documentation set binds an instance to a machine, matches on one, or de-duplicates by one.**

### Clause 2 — **release** one slot, on request from a computer that holds no slot, about a computer that may no longer exist

**Documented in half, and the half the spec exists for is the half that is not reachable from the desktop application alone.**

Release itself is documented, and it does free the slot:

> "Deactivate a license key instance."
> "license_key Required — The license key to deactivate."
> "instance_id Required — The instance ID returned when activating a license key."
> — *Deactivate a License Key*, `https://docs.lemonsqueezy.com/api/license-api/deactivate-license-key`, read 2026-08-16

> "Deactivating a license will delete the instance and reduce the activation_usage for the license key."
> — tutorial, address above

> "Note that there is no instance object in the response because it has been deleted."
> — tutorial, address above

**Nothing requires the released computer to answer, to be reachable, or to have been seen.** The endpoint takes two strings. Row 233's constraint that a dead laptop must not have to release itself is therefore met at the service, in the documented shape of the call.

**The blocking clause is `instance_id`.** The endpoint's own words say where that value comes from — "The instance ID returned when activating a license key" — and the tutorial states plainly that it is the activating client's job to keep it:

> "Make sure you save the instance.id value into your app as you will need this to validate and deactivate license key instances."
> — tutorial, address above

A third computer that never activated has never held the other two computers' `instance_id` values. **The License API returns no list of instances.** The `activate` and `validate` responses return `license_key`, at most one `instance`, and `meta`; `validate` returns an instance only when the caller already supplied its id:

> "instance_id — If included, validate a license key instance, otherwise validate a license key. If no instance_id is provided, the response will contain "instance": null."
> — *Validate a License Key*, `https://docs.lemonsqueezy.com/api/license-api/validate-license-key`, read 2026-08-16

The only documented enumeration of instances is on the **other** API:

> "GET /v1/license-key-instances — Returns a paginated list of license key instances."
> "license_key_id — Only return instances belonging to the license key with this ID."
> — *List All License Key Instances*, `https://docs.lemonsqueezy.com/api/license-key-instances/list-all-license-key-instances`, read 2026-08-16

and that API is authenticated, with a documented prohibition that closes the desktop route:

> "The Lemon Squeezy API uses API keys to authenticate requests."
> "Do not share your API keys in publicly accessible areas such as GitHub, client-side code, and so forth."
> "Generated API keys are valid for a year."
> — *Requests*, `https://docs.lemonsqueezy.com/api/getting-started/requests`, read 2026-08-16

**So the documented route to Row 233's release is complete only with a component that is not the user's copy of Smoozy.** The chain is: `POST /v1/licenses/validate` with the key alone returns `license_key.id`; `GET /v1/license-key-instances?filter[license_key_id]=<id>` with a `Bearer` API key returns the instances and their `identifier` values; `POST /v1/licenses/deactivate` with the key and one `identifier` frees the slot. The middle step is the one the vendor forbids putting in client-side code, and the third step's key-and-instance-id pair is documented to need no authentication at all. **The instance `identifier` is the same value as the activation-time `instance_id`:**

> "identifier — The unique identifier (UUID) for this instance. This is the instance_id returned when activating a license key."
> — *The License Key Instance Object*, `https://docs.lemonsqueezy.com/api/license-key-instances/the-license-key-instance-object`, read 2026-08-16

**Verdict: states something narrower.** Release from the computer that activated is documented and needs no authentication. Release *about another computer* is documented only through an API key the vendor says must not ship in a client.

### Clause 3 — report a key as **no longer valid, and why**

**States something narrower.** Both a boolean and a reason exist; the reason is coarse in one place and undocumented in the other, and the two are never connected.

The boolean:

> "valid — A boolean indicating whether the license was successfully validated."
> "error — An error message if the license could not be validated. This will be null if the license was validated successfully."
> — *Validate a License Key*, address above

The reason, as a status with four documented values:

> "inactive - The license key is valid but has no activations."
> "active - The license key has one or more activations."
> "expired - The license key’s expiry date has passed, either because the related product had a defined license length or because the license’s subscription has expired."
> "disabled - The license key has been manually disabled from the Lemon Squeezy dashboard."
> — *License API*, `https://docs.lemonsqueezy.com/api/license-api`, read 2026-08-16, and repeated verbatim in the tutorial

> "disabled — Has the value true if this license key has been disabled."
> — *The License Key Object*, address above

**The two fields are connected in two places, and neither closes the gap the rows need.** The vendor asserts the validity of one status outright — "inactive - The license key is valid but has no activations" (*License API*, address above) — and directs the implementer to read `status` rather than `valid` for the access decision:

> "When accessing license keys with the API, you’ll see a status value which you can use to determine if a customer should have access to your application."
> — tutorial, `https://docs.lemonsqueezy.com/guides/tutorials/license-keys`, read 2026-08-16

**The gap that survives is narrower and is the one Rows 227 and 237 need: no page states what the `valid` boolean returns for a `disabled` or an `expired` key.**

**The vendor's own worked example is a hard-coded illustration and evidences nothing about what the server does. Corrected 2026-08-22: this paragraph read *"And the vendor's own worked example shows the two fields disagreeing with the calendar"*, cited `"valid": true` and `"status": "active"` against an `expires_at` of `2022-01-24T14:15:07.000000Z` — *"more than four years in the past relative to the date this page was read"* — and closed *"what the examples do show is a long-expired key rendered both `active` and `valid`."*** On *Validate a License Key*, re-read 2026-08-20, that example's `created_at` is `2021-01-24T14:15:07` and its `expires_at` is `2022-01-24T14:15:07` — **the same instant one year later, to the second**, which is a hand-written illustration of a one-year licence frozen at the moment it was typed, not a response any server produced on the day the page was read. Its age against today's calendar is an artefact of the page having been written once. The tutorial's two validate examples have the same shape. **And the vendor documents the opposite rule, on the page this reading already quotes:** *"expired - The license key's expiry date has passed, either because the related product had a defined license length or because the license's subscription has expired."* No documented example anywhere in the set shows `expires_at` earlier than `created_at`.

**Two further things are not stated and both matter to Row 237.** **No page enumerates the `error` strings.** One example string exists in the whole documentation set — `"This license key has reached the activation limit."` on the activate page's error example — and no other; **confirmed across all 232 pages**, where a search for any quoted `"error": "…"` value returns exactly that one distinct string. And **the four statuses are the whole vocabulary of "why", and none of them says "refunded" or "charged back"**; `disabled`'s documented cause is a human act in the dashboard, which is Fact 10's problem below.

Row 227's last clause — "Smoozy does not guess at why, beyond saying what it was told" — has a documented `error` field to carry, and that field's contents are documented once by example.

### Clause 4 — tell apart: an invalid key; a licence already on two computers; a check that could not be reached

**States something narrower, and the separation rests on one undocumented string.**

The transport frame is documented:

> "The API will return a 4XX status code in the response if an error has occurred and an error field in the response body containing an explanation of the error."
> — *License API*, address above

and the status table on the same page reads, in full:

> "400 — An error occurred. See the error field for details."
> "404 — An item could not be found."
> "422 — A required field was invalid or missing."

**A licence already at its limit is documented, by example only:**

> `"activated": false, "error": "This license key has reached the activation limit."`
> — *Activate a License Key*, error response example, address above

**An invalid key is not documented at all.** No page states which status code or which `error` string an unknown, mistyped or non-existent key produces. `404 — An item could not be found` is the only candidate in the table and no page connects it to a key. **Confirmed across all 232 pages:** a search for `invalid key`, `unknown key`, `incorrect key`, `key not found` and `licence … not found` returns **zero** matches anywhere in the set. The silence is total.

**A check that could not be reached is not the vendor's to document and they do not.** The separation available is structural rather than stated: a documented refusal is a 4XX carrying a JSON body with an `error` field, and a check that could not be reached produces no HTTP response and no body. **That distinction is a property of HTTP, not a documented property of this service**, and it is the whole of what Row 227's third outcome has under it.

**There is a fourth outcome, and Access has only three.** The vendor documents two response shapes that are neither a 4XX-with-`error` nor an absent response:

> "If you exceed the limit you will receive a 429 Too Many Requests response."
> — *API Reference*, `https://docs.lemonsqueezy.com/api` — not opened by this reading's first pass; surfaced by the citation check and confirmed on the live page, 2026-08-16

> "Codes in the 5xx range indicate an error with our servers (these are rare)."
> — *Responses*, `https://docs.lemonsqueezy.com/api/getting-started/responses`, read 2026-08-16

The rate limit applies to the License API — its own page states the 60-per-minute figure — so **a 429 is a documented outcome of the very check Row 234 schedules**, and it is neither a refusal Row 227 wants reported nor an unreachable service. **The License API's own status table lists only 400, 404 and 422, so that page documents neither its own rate-limit refusal nor a server failure.**

**Verdict.** The two refusals are separable only by matching an `error` string that appears once, in an example, on one page. A vendor free to reword an example string can collapse *invalid key* and *already on two computers* into one indistinguishable refusal without changing any documented contract — which is the exact harm Row 227 names. And Row 227's three outcomes have to absorb a fourth that the service's own reference page does not describe.

### Clause 5 — hold **whether a given computer has already had a trial** — 7 days, one per computer, clock started when the user asks

**Not stated at all. Nothing in the documentation holds any record that is not bound to a purchase.**

Every record the service is documented to hold hangs off an order. The license key object's attributes are `store_id`, `customer_id`, `order_id`, `order_item_id`, `product_id`, `user_name`, `user_email`, `key`, `key_short`, `activation_limit`, `instances_count`, `disabled`, `status`, `status_formatted`, `expires_at`, `created_at`, `updated_at` — *The License Key Object*, address above. The instance object's are `license_key_id`, `identifier`, `name`, `created_at`, `updated_at` — *The License Key Instance Object*, address above. **Neither object carries a free-form or arbitrary-metadata field of any kind.**

The vendor's own "trial" is a billing trial on a subscription, not a per-device one:

> "If you want to offer free trials with Lemon Squeezy, it’s currently supported on all subscription products."
> "The free trial period offers full access to your subscription product before payments begin from your customer."
> — *Free Trials*, `https://docs.lemonsqueezy.com/help/products/free-trials`, read 2026-08-16

It is subscription-only, and Access states the licence is "bought once and is the user's forever. There is no subscription and nothing to renew" (Row 301).

**The one documented free-access route fails the standing constraint outright:**

> "Allow product access for free in return for the user’s email address."
> — *Lead Magnets*, `https://docs.lemonsqueezy.com/help/products/lead-magnets`, read 2026-08-16

A trial obtained that way is a registration inside the trial path. Under Fact 9's standing constraint that does not count.

**Verdict: not stated, and confirmed absent rather than merely unfound.** Nothing in Lemon Squeezy's documentation records anything against a computer that has not bought something. **This is the negative the citation check searched hardest, across all 232 pages:** every occurrence of `trial` (all bound to a subscription variant, a price object, or a checkout `skip_trial` flag — not one to a device); every occurrence of `custom_data` and `metadata` (all bound to a checkout, an order, or a subscription); the full attribute lists of both licence objects; and every page in the Products, Licensing, Checkout and Online Store sections. **There is no per-device record, no arbitrary-metadata field on any licence object, and no object in the documented model that is not an order's descendant.**

---

## The two questions Fact 11's answer forces

Fact 11 is answered negative on all three operating systems: every identifier an ordinary downloaded application can read is a value the operating system hands over on request, and each vendor documents that it can be changed or spoofed. Fact 12 settles that two computers can carry the same computer name on all three. Fact 13 settles that nothing on any of the three survives both a reboot and an administrator. So both questions below are asked of the service, because the user's computer cannot answer them.

### Does anything documented stop the same key being activated repeatedly under different computer names, or the same trial being taken again under a fresh one?

**No, and the documentation says why in its own words.**

`instance_name` is documented as a label and nothing more — "A label for the new instance to identify it in Lemon Squeezy" (*Activate a License Key*, address above). **No page states any uniqueness constraint on it, any matching on it, or any de-duplication by it — verified across all 232 pages, not only the 35 read here.** The tutorial states the opposite behaviour outright:

> "A new license key instance is created for each successful activation request, each with a unique ID"
> — tutorial, address above

So an activation under a fresh name is a fresh instance. **The only documented defence is the count** — "Lemon Squeezy will not allow new license key activations once the activation limit has been reached" — and the count is documented to be reduced by an unauthenticated call: "Deactivating a license will delete the instance and reduce the activation_usage for the license key" (tutorial, address above), where deactivate requires only `license_key` and `instance_id` and carries no `Authorization` header in any documented example.

**Two consequences, both documented rather than inferred.**

- **A slot is consumed per activation event, so an honest reinstall costs a slot.** Nothing documents a computer being recognised as one it has already seen. Row 240's promise that a locked Smoozy unlocks in place, and Row 229's key entered after a lock, both re-enter the key; whether that is a fresh activation or a re-validation against a saved `instance_id` is Architecture's to arrange, and the service offers no help distinguishing them.
- **The named ceiling is the only enforcement, and it is a count of events, not of computers.** Two computers can hold the two slots; so can one computer twice.

**On the trial half: there is nothing to take again, because nothing is taken.** No trial record exists at the service (clause 5).

### Does anything documented give the service its **own** record of when a trial started, independent of the clock on the user's computer?

**For a trial, no — there is no trial record.** For a **licence**, the service holds service-side timestamps and evaluates one of them itself, and that is the whole of what is available.

Service-side timestamps, all ISO 8601 and all set by the service:

> "created_at — An ISO 8601 formatted date-time string indicating when the object was created."
> "expires_at — An ISO 8601 formatted date-time string indicating when the license key expires. Can be null if the license key is perpetual."
> — *The License Key Object*, address above; `created_at` also on *The License Key Instance Object*, address above

Both `license_key.created_at` and `instance.created_at` are returned to an unauthenticated caller — by `activate`, and by `validate` when an `instance_id` is supplied. **The service also evaluates expiry itself**, which is the only documented place where its own clock reaches a verdict:

> "expired - The license key’s expiry date has passed, either because the related product had a defined license length or because the license’s subscription has expired."
> — *License API*, address above

**What is not there.** **No documented response carries the service's current time** — confirmed across all 232 pages, where a search for `server time`, `current time`, a clock-bearing `now`, or a documented `Date` header returns nothing. The unauthenticated License API returns no server clock in any documented field, so a client can read service-set timestamps but cannot compare them against a service-held "now" except by asking the service whether the key has expired. And `expires_at` is writable only through the authenticated API:

> "expires_at — An ISO 8601 formatted date-time string indicating when the license key expires. Can be null if the license key is perpetual."
> — *Update a License Key*, address above (`PATCH /v1/license-keys/:id`, `Authorization: Bearer {api_key}`)

**So a service-defended clock is documented to exist for a purchased key and to be settable only by something holding an API key that must not ship in the client.** Fact 13's finding that the trial clock has to be defended at the service or not at all meets, here, a service that defends a clock for keys it sold and holds nothing at all for a computer that has bought nothing.

---

## Fact 10 — the seller

### Taking payment

**Documented.** Lemon Squeezy is the merchant of record, not a gateway:

> "When a customer makes a purchase, they are buying from the merchant of record. The merchant of record is responsible for handling all payments and takes responsibility for everything relating to the purchase, including collecting sales tax, processing refunds and chargebacks, and ensuring PCI compliance."
> "Yes! Lemon Squeezy acts as a merchant of record."
> — *Merchant of Record*, `https://docs.lemonsqueezy.com/help/payments/merchant-of-record`, read 2026-08-16

> "With Lemon Squeezy, there is zero setup required and you can start accepting payments immediately."
> — *Payment Methods*, `https://docs.lemonsqueezy.com/help/checkout/payment-methods`, read 2026-08-16

The same page lists the accepted methods as cards ("Visa, Mastercard, American Express, Discover, Diners Club, JCB and China UnionPay from customers worldwide"), PayPal, Apple Pay ("Safari only"), Google Pay ("Chrome only"), Alipay, WeChat Pay, Cash App Pay and "Bank debits (e.g. ACH)", with the note "Some of these payment methods are only available on certain devices or in certain locations."

**Nothing about the purchase touches Smoozy**, which is what Row 225 requires.

### Delivering the key by email

**Documented, and a recovery route exists that Smoozy only has to point at (Row 226).**

> "When the “Generate License Keys” setting is enabled, a unique license key will be generated for each customer that purchases the product. The customer will be able to see their license key in their order receipt email and when viewing their order on the My Orders page."
> — *Generating License Keys*, address above

> "Each time a customer purchases a product with license keys enabled, a license key will be generated and emailed to the customer in their receipt. The customer will also find the license key via the My Orders page."
> — tutorial, address above

> "When an order is created the customer will be sent an email receipt with information about their order and a link to access the order content. Receipt emails can be re-sent at any time via the action menu on the Orders page."
> — *Resend Receipt*, `https://docs.lemonsqueezy.com/help/orders/resend-receipt`, read 2026-08-16

> "The URL for customers to access all of their orders is app.lemonsqueezy.com/my-orders."
> "Customers only need the email they used at checkout. We will send them a secure magic link to their email, which, when clicked, will automatically log them in."
> — *My Orders*, `https://docs.lemonsqueezy.com/help/online-store/my-orders`, read 2026-08-16

**This satisfies the standing constraint only because it is entirely outside the app.** The email and the magic-link login belong to the purchase and to the seller's own site; nothing about it puts an account, a registration or an email field inside Smoozy. Row 226's constraint that recovery is "something Smoozy points at rather than something it does" has a documented destination.

### Row 471 — the issued key's form, whether it is invariant, and `key_short` (read 2026-08-27)

**This is a later, narrow reading for Fact 10 and is not covered by the citation check of 2026-08-16.** Only Lemon Squeezy's live documentation was read. Nothing was run and no key was issued. The finding therefore distinguishes what the pages **show by example** from what they **state as a contract**.

**Documented — the examples show one form.** Lemon Squeezy states only that it generates a unique key:

> "When the “Generate License Keys” setting is enabled, a unique license key will be generated for each customer that purchases the product."
> — *Generating License Keys*, `https://docs.lemonsqueezy.com/help/licensing/generating-license-keys`, read 2026-08-27

The reference names the full-value field without defining its syntax:

> "key — The full license key."
> — *The License Key Object*, `https://docs.lemonsqueezy.com/api/license-keys/the-license-key-object`, read 2026-08-27

Its object example then shows:

> `"key": "80e15db5-c796-436b-850c-8f9c98a48abe"`
> — same page, read 2026-08-27

The License API's activate example independently shows:

> `-d "license_key=38b1460a-5104-4067-a91d-77b872934d51"`
> — *Activate a License Key*, `https://docs.lemonsqueezy.com/api/license-api/activate-license-key`, read 2026-08-27

Both shown values are 36 characters in an `8-4-4-4-12` grouping of lowercase hexadecimal characters. **That sentence describes the examples; it is not a documented rule for issued keys.** The same UUID-shaped example recurs on the validate, deactivate and tutorial pages, but those pages call it only a license key and nowhere call the key a UUID.

**Not stated — that this form is promised for every issued key.** No page read states that a license key is a UUID; fixes its length, grouping, character set or case; permits a seller-defined prefix; or promises that the generator will retain one format. The distinction is visible on one official page which names two complete values side by side:

> "license_key — The complete license key."
>
> "order_identifier — The unique order identifier in UUID format linked to the transaction."
> — *Link Variables*, `https://docs.lemonsqueezy.com/help/products/link-variables`, read 2026-08-27

The page explicitly promises UUID format for the order identifier and makes no corresponding statement for the license key. **The examples therefore do not establish an invariant.** Nothing documents the mockup's `SMZ-XXXX-XXXX-XXXX` prefix or grouping, and nothing states that a product may configure either.

**Documented — `key_short` is a per-key fragment, not a description of the key's form.** Its complete definition is:

> "A “short” representation of the license key, made up of the string “XXXX-” followed by the last 12 characters of the license key."
> — *The License Key Object*, `https://docs.lemonsqueezy.com/api/license-keys/the-license-key-object`, read 2026-08-27

and the example is:

> `"key_short": "XXXX-8f9c98a48abe"`
> — same page, read 2026-08-27

So `key_short` is a fixed literal `XXXX-` followed by **twelve characters copied from the real key**. It exposes those twelve characters. It does not state or preserve the full key's length, the number or positions of its separators, its prefix, or the characters before that suffix. It therefore cannot establish that every full key has the example's shape, and it is not the fixed shape of *remembered* that `Security and licence` §3 rule 6 permits: twelve displayed characters are part of the stored value.

**Not stated — any seller-neutral way for the desktop to discover a key format.** The documented License Key Object has `key` and `key_short`, but no format, length, grouping, prefix or character-set field. The License API's activate and validate responses document and show the full `key`; they do not document `key_short` or any format descriptor. The authenticated API is not a client-side escape:

> "The Lemon Squeezy API uses API keys to authenticate requests."
>
> "Do not share your API keys in publicly accessible areas such as GitHub, client-side code, and so forth."
> — *Requests*, `https://docs.lemonsqueezy.com/api/getting-started/requests`, read 2026-08-27

**Verdict for Row 471's fact.** Lemon Squeezy's current pages show UUID-shaped full-key examples, but do not promise that issued keys always have that form. They document no format-discovery route. `key_short` is seller-specific, per-key output which reveals the last twelve real characters and says nothing about the omitted part's shape. On the documented interface, Smoozy cannot make the desktop know a truthful full-key form without either hard-coding an inference from Lemon Squeezy's examples or adding a new seller-aware translation to its own service contract; the latter would be a Smoozy decision, not a vendor fact. **Fact 10 supplies no basis for the mockup's `SMZ-XXXX-XXXX-XXXX` placeholder or its `SMZ-••••-••••-••••` remembered reading.**

### Does a refund or a chargeback reach whatever the licence check reads?

**Not stated. This is the clause the brief said was most likely to be unstated, and it is unstated in every direction it was looked for.**

**What the refund pages say about license keys: nothing.**

> "Orders can be refunded partially or in full at any time via the action menu on the Orders page."
> "Refunds can take up to 10 days to appear on your customer’s statement."
> — *Refund Order*, `https://docs.lemonsqueezy.com/help/orders/refund-order`, read 2026-08-16 — the complete body of the page apart from its subtitle, one further sentence of refund policy, and a pointer to the page below

> "Lemon Squeezy allows sellers to set their own refund policies but reserves the right to issue refunds within 60 days to prevent chargebacks."
> "If you do choose to offer refunds, they can be issued at any time via the Lemon Squeezy dashboard. The refunded amount (minus our platform fee) will be deducted from your next payout."
> "That being said, Lemon Squeezy reserves the right to issue refunds within 60 days of purchase, at its own discretion, in order to prevent chargebacks. So if you choose to have a “no refunds” policy, be aware that customers can issue chargebacks at any time against their purchases."
> "Generally, Lemon Squeezy is responsible for handling any chargebacks made against your sales. In most cases, we will offer a full refund on your behalf and deduct the refunded amount (minus our platform fee) plus a $15 dispute fee from your next payout."
> "If your store has too many chargebacks, our team may intervene and suspend your account."
> — *Refunds and Chargebacks*, `https://docs.lemonsqueezy.com/help/payments/refunds-chargebacks`, read 2026-08-16

**Neither page contains the word "license".** The API's refund endpoint says only:

> "Issues a partial or full refund for an order."
> "Returns an Order object with updated refunded attributes."
> — *Issue a Refund*, `https://docs.lemonsqueezy.com/api/orders/issue-refund`, read 2026-08-16

**What the licence pages say about refunds: nothing.** The four documented statuses give one route to invalidity that is not time-based, and its documented cause is a human act:

> "disabled - The license key has been manually disabled from the Lemon Squeezy dashboard."
> — *License API*, address above

**The webhook list carries a refund event and no dispute event.** The complete list of seventeen events was read; the two that touch this clause are:

> "order_refunded | Order object | Occurs when a full or partial refund is made on an order."
> "license_key_updated | License key object | Occurs when a license key is updated."
> — *Event Types*, `https://docs.lemonsqueezy.com/help/webhooks/event-types`, read 2026-08-16

**No chargeback, dispute or charge-reversal event appears anywhere in that list — and the store accepts two the list does not carry. Corrected 2026-08-22: this sentence read *"There is no chargeback, dispute or charge-reversal event anywhere in that list"*, which a reader of this document alone takes as a statement about the seller rather than about the page.** [The seller itself](The%20seller%20itself%20—%20the%20store%20observed%20rather%20than%20read.md), 2026-08-17: `dispute_created` and `dispute_resolved` were registered against the Creator's own store and read back, against sixty-eight other spellings of the same idea refused with one identical sentence. Licence service (`Preparation/Architecture/Licence service.md`, withdrawn) §9 and §15 item 9 carry it. **What is unchanged is the page**, which still lists seventeen and neither of the two. The other fifteen are `order_created`, `customer_updated`, `subscription_created`, `subscription_updated`, `subscription_cancelled`, `subscription_resumed`, `subscription_expired`, `subscription_paused`, `subscription_unpaused`, `subscription_payment_success`, `subscription_payment_failed`, `subscription_payment_recovered`, `subscription_payment_refunded`, `license_key_created` and `affiliate_activated`. And `license_key_updated`'s description — "Occurs when a license key is updated" — **names no trigger**, so nothing states that a refund updates a key.

**Extended across all 232 pages by the citation check:** `chargeback`, `charge-back` and `dispute` appear on ten pages, none of them a licence-side signal — the two refund and merchant-of-record pages, *Refund Order*, *Payment Methods* ("Reduce fraud and disputes"), *Currencies* and *Payments* as navigation links, two affiliate pages where a referral can go in review on "an unresolved dispute", and the docs root. **No webhook event, no API field and no licence-side signal of a chargeback exists anywhere in the documentation set.**

**But a chargeback is documented to become a refund, and refunds do fire an event.** The two sentences have to be read together:

> "Generally, Lemon Squeezy is responsible for handling any chargebacks made against your sales. In most cases, we will offer a full refund on your behalf…"
> — *Refunds and Chargebacks*, address above

> "order_refunded | Order object | Occurs when a full or partial refund is made on an order."
> — *Event Types*, address above

**No page joins the two by name.** What is documented is that a chargeback produces a full refund "in most cases", and that `order_refunded` fires when a full or partial refund is made. **So a chargeback is in most cases learnable as a refund**, and the silence is that nothing states the join — not that no signal could exist.

**A disagreement between two of their own pages, on the one operation this clause would need.** Both are recorded; neither is picked.

> "It’s not currently possible to disable license keys using the API."
> "If you want to disable a license key (i.e. fully stop a customer from having access to your product), you can do this from your Lemon Squeezy dashboard. Go to Store » Licenses and click on the ... menu icon on the license you want to disable. Click on the “Disable key” option and confirm."
> — tutorial, `https://docs.lemonsqueezy.com/guides/tutorials/license-keys`, read 2026-08-16

against

> "disabled — If true, the license key will have “disabled” status."
> — *Update a License Key*, `https://docs.lemonsqueezy.com/api/license-keys/update-license-key`, read 2026-08-16, documenting `PATCH /v1/license-keys/:id`

**The disagreement was still live on the date of reading. Neither page is picked here.**

**And the whole revocation route is documented on the reference side.** The missing link is a filter on a page this reading opened:

> "order_id — Only return license keys belonging to the order with this ID."
> — *List All License Keys*, `https://docs.lemonsqueezy.com/api/license-keys/list-all-license-keys`, read 2026-08-16, documenting `GET /v1/license-keys`

The chain, every step documented and all three authenticated: `order_refunded` delivers an Order object → `GET /v1/license-keys?filter[order_id]=<id>` returns that order's keys → `PATCH /v1/license-keys/:id` with `"disabled": true`, which "will have “disabled” status." **Whether it works turns entirely on the contradiction above.**

**Three internal defects on the licence pages, recorded because they lower what those pages are worth as evidence.**

- *Update a License Key*'s "Returns updated License Key object" example body is a `subscription-items` object with `subscription_id`, `price_id`, `quantity` and `is_usage_based` attributes, and a `links.self` pointing at `https://api.lemonsqueezy.com/v1/subscription-item/1`. **The page's example shows no licence key at all.**
- *Validate a License Key* describes its own `license_key` return as **"The information about the license that was deactivated."** — on an endpoint that deactivates nothing.
- *Update a License Key* and the tutorial contradict each other on whether the API can disable a key, as above.

**Verdict on Fact 10's third clause: not stated.** The seller documents that it handles refunds and chargebacks as merchant of record. **It documents no statement, anywhere, that a refund or a chargeback changes a licence key's status by itself.** No event names a chargeback; `license_key_updated` names no trigger; `disabled`'s only documented cause is a human act in a dashboard, on one of two disagreeing pages.

**One documented propagation exists, and it is the shape Row 237 wants, in the one place Access forbids.**

> "If your product is a subscription, the “License length” field is not shown in the product form because the length of the license is instead tied to the subscription’s lifecycle. When a subscription becomes expired, the related license key’s status will automatically become expired."
> — tutorial, address above

**The service is documented to drive a licence status from a billing event — for subscriptions only.** Access forbids a subscription (Row 301). So the one documented mechanism that would keep Row 237 without anything of the Creator's own is the one the spec has ruled out.

---

## What is not stated, gathered, with what breaks

Each entry is a documented silence, not a documented negative.

**1. Whether a refund changes anything the licence check reads by itself.** *Breaks:* Row 237 entire — "A licence the check reports as no longer valid — refunded, charged back, or withdrawn — locks Smoozy at once". Row 237 is the only row in Access waiting on Facts 9 and 10 **joined**, and this is the join. If a refund leaves the key `active`, a refunded copy of Smoozy keeps working forever unless something acts. **What that something is turns on the tutorial-versus-reference contradiction:** if the tutorial is right, it is a manual dashboard act per refund; if the API reference is right, the whole route is documented and automatable — `order_refunded` → `GET /v1/license-keys?filter[order_id]=<id>` → `PATCH … "disabled": true`. **Either automated route needs the same Creator-side component the release route in Clause 2 already needs, so that component is paid for once rather than twice.** The silence itself is unchanged and real: nothing states that a refund changes a key's status on its own.

**2. Whether a chargeback is named anywhere as a licence-side signal.** *Breaks:* the same row, and the spec names "charged back" explicitly. No webhook event, no API field and no licence-side signal of a chargeback exists in the documentation set. **What softens this:** a chargeback is documented to produce a full refund "in most cases", and `order_refunded` fires on a full or partial refund — so in most cases it is learnable as a refund. What is missing is any page joining the two by name, and the "in most cases" hedge, which leaves the remainder unaccounted for.

**3. Whether the API can disable a key — two of their pages disagree.** *Breaks:* any automated route from a seller event to a locked app. If the tutorial is right, every revocation is a human act in a dashboard; if the update endpoint is right, it is one authenticated `PATCH`. The two answers differ by whether revocation is a mechanism or a chore, and nothing here settles which.

**4. What the `valid` boolean returns for a `disabled` or an `expired` key.** *Breaks:* Row 227's first outcome and Row 237's immediacy. A check that returns `"valid": true` beside `"status": "disabled"` would report a revoked licence as good, and Access locks on what it is told. **Not a total silence:** the vendor asserts "inactive - The license key is valid", and directs the implementer to read `status` "to determine if a customer should have access to your application". What is missing is the `valid` half for the two statuses that matter. **Corrected 2026-08-22: this sentence used to close *"— and the vendor's own example shows `"valid": true` and `"status": "active"` against a key four years past its `expires_at`"*.** That example is a frozen illustration and says nothing about server behaviour; Clause 3 above carries the reading. The gap itself is unchanged.

**5. What an unknown or mistyped key returns.** *Breaks:* Row 227's requirement that *this key is invalid* and *this licence is already on two computers* never be confused. Only the second has a documented error string, and it appears once, in an example.

**6. The `error` strings, as an enumerated contract.** *Breaks:* Row 227's "Smoozy says what it was told and adds nothing" and Row 232's release offer, which has to fire on the second refusal and not the first. Matching an example string is what the answer would rest on, and an example is not a contract.

**7. Any uniqueness or matching on `instance_name`.** *Breaks:* Row 231's promise that a third computer is refused *by name*, and Row 223's bar. The service counts activation events. Two activations from one computer take two slots; a name is a label the service never reasons about.

**8. Any record keyed on a computer that has not purchased.** *Breaks:* Rows 223 and 224 entire — "One trial per computer" and "The trial is recorded away from the user's computer at the first connection there is, and that record is what stops the same computer having a second one". Nothing in this documentation can hold that record. On this clause the fork is not narrowed; it is unserved.

**9. Any listing of a key's instances without an API key.** *Breaks:* Row 300 ("the computers a licence is on, and the way to release one" in Settings) and Row 233's release-from-a-third-computer, which is the lost-laptop case Row 233 says the promise exists to serve. The vendor forbids the API key in client-side code, so the documented route requires a component that is not the user's copy of Smoozy.

**10. Any service-side "now" in an unauthenticated response.** *Breaks:* nothing on its own, but it removes the last route by which the trial clock could have been defended at the service without a component of the Creator's own. Fact 13 established the clock cannot be defended on the user's computer.

**11. Availability, uptime, and what happens to keys if the service stops.** *Breaks:* Row 301 — "A licence is **bought once and is the user's forever**" — whose answer the shape says "is what a paid copy does if the service it checks against stops answering for good". **Searched across all 232 pages for `uptime`, `SLA`, `service level`, `status.lemonsqueezy`, `sunset`, and for any link to a status host: nothing.** No uptime commitment is stated anywhere in the set, and no page states anything about key validity outliving the vendor. **A status page does exist, at `status.lemonsqueezy.com`. Corrected 2026-08-22: this sentence read *"No uptime commitment and no status page exist"*, which turned an absence in the corpus into an absence in the world.** Read 2026-08-20, it is the vendor's own — title *"Lemon Squeezy Status"*, on their domain — and publishes rolling 60-day uptime per component (`lemonsqueezy.com`, App, API, Docs, Custom Domains, Checkout) with an incident history whose most recent entry is *"Jul 30, 2026 — Custom Domains has recovered."* **Neither the documentation site nor the marketing site links to it**, which is why the search above could not reach it; it is reachable only by knowing the hostname. **The uptime-commitment half stands**: measured uptime is a record and not a promise, and nothing on that page is a service-level commitment. Row 236 and Fact 14 (reachability from a company network or a blocking country) are likewise unaddressed anywhere in the set.

**And one thing the vendor carries outside this corpus, added 2026-08-22.** `https://www.lemonsqueezy.com/blog/2026-update`, read 2026-08-20, dated January 28, 2026 in its own field and signed *"JR Farr — Co-Founder, Lemon Squeezy (Stripe)"*: *"That's what we're doing with Stripe Managed Payments: Stripe's merchant of record solution."*, *"Very soon we will announce public access for Stripe Managed Payments allowing users to sign up without needing an invite. Our goal is to provide Lemon Squeezy users an easy way to migrate to Stripe Managed Payments."*, and, in the same voice, *"slower support responses and less frequent product updates."* **No shutdown is announced on that page and no end date is given**, and the service was answering on the day it was read. This reading's brief confined it to `docs.lemonsqueezy.com`, so the post is outside the corpus and is recorded here rather than inside a clause above; it bears on how much weight the licence path puts on this vendor's support, and on nothing this document rules.

**Deprecation is not on this list. The vendor documents a versioning and deprecation policy**, and this reading missed it by not opening the page:

> "The Lemon Squeezy API includes the major version number as a prefix for all endpoints (e.g. /v1). While we reserve the right to make changes to the API as we see fit, we endeavour not to make any changes to the API that will break backwards compatibility."
> "In the rare case that we do need to break backwards compatibility, we will release a new major version of the API and deprecate the old version. This will give any integrations time to migrate to the new version."
> — *API Reference*, `https://docs.lemonsqueezy.com/api` — not opened by this reading's first pass; surfaced by the citation check and confirmed on the live page, 2026-08-16

The same page enumerates what it counts as backwards-compatible: adding resources, adding optional request parameters, adding response properties, changing property order, and "Adding new webhook event types." **Label: documented.** It is a stated endeavour rather than a guarantee, and it says nothing about the help-centre behaviour the licence pages describe — only about the API's shape.

**12. Whether the vendor's documentation is current.** *Breaks:* the confidence in every entry above. The help centre carries no dates at all; the API changelog is dated and moving.

---

## Bearing on the clauses without answering them

Recorded because the brief asks for them, and labelled as not answering anything.

**The License API is documented to require no authentication.** Its Requests section names three requirements and an API key is not among them — "All API requests must be made over HTTPS to https://api.lemonsqueezy.com", "All requests must include the Accept: application/json header", "POST requests should be sent with the Content-Type: application/x-www-form-urlencoded header" (*License API*, address above). None of the three endpoint pages' `curl` examples carries an `Authorization` header, where every main-API example does. **Consequence for [Security and licence](Security%20and%20licence.md):** anyone holding the key string can read `activation_usage`, activate, and — given an `instance_id` — deactivate. The key string is the whole credential.

**Two different rate limits, and Row 300 sits on the other one.** "API calls are rate limited to 60 requests per minute." — *License API*, address above, and that figure is the **License API's**. The main API states a different figure — "There is a limit of 300 API calls per minute", with `X-Ratelimit-Limit` and `X-Ratelimit-Remaining` headers on successful responses (*API Reference*, `https://docs.lemonsqueezy.com/api` — not opened by this reading's first pass; surfaced by the citation check and confirmed on the live page, 2026-08-16). **Row 234's check runs on the 60; any Settings screen listing a licence's computers runs on the main API and inherits the 300.**

**`activation_usage` is documented in prose and examples only.** "you are able to check how many activations have occured from license_key.activation_usage" (tutorial, address above; the misspelling is the vendor's). It is never a named field on any endpoint reference page, and *The License Key Object* documents `instances_count` instead. **It rests on the same kind of evidence as the one `error` string** — an example and a sentence of tutorial prose, not a reference contract.

**A per-activation identifier exists and is a UUID.** `instance.id` / `identifier`, documented above. It is the only stable per-activation handle the service offers, it is issued by the service, and the tutorial makes keeping it the client's job.

**The vendor's own recommended integration uses the purchase email, twice.** "Once submitted, you will need to verify the active user somehow. We recommend checking against their email address" and "You can also verify that the customer inputting the license key in your app is the same customer who purchased it by checking against meta.customer_email in the response. If the email address matches, you can give access to the user" (tutorial, address above). **Under Fact 9's standing constraint this route is closed** — it puts an email field inside Smoozy. Recorded because it is the vendor's own recommendation and an implementer reading their tutorial will meet it first.

**The `meta` object returns customer data to any unauthenticated caller holding the key.** Documented fields include `customer_name` and `customer_email` (*Activate a License Key*, address above). Access states there is no email inside Smoozy; a documented response nonetheless delivers one to the app on every check. Bears on [Security and licence](Security%20and%20licence.md)'s logging-redaction scope.

**The vendor recommends hard-coding store and product identifiers into the client.** "You should verify that the store_id, product_id and/or variant_id from this response match the IDs of your Lemon Squeezy product. If you don't do this, someone using a license key from another Lemon Squeezy product could use it to get access to your product. We recommend hard-coding the store_id, product_id and/or variant_id into your client and using them to validate that the key belongs to your product" (tutorial, address above). Without it, **any Lemon Squeezy key from any seller activates Smoozy**.

**Webhook delivery is best-effort with a bounded retry.** "Return a 200 response to show that the webhook was successfully captured. If the response status is anything else, the webhook will be retried up to three more times using an exponential backoff strategy (e.g. 5 secs, 25 secs, 125 secs) at which point the request will be considered failed and will no longer be retried" (*Webhook Requests*, `https://docs.lemonsqueezy.com/help/webhooks/webhook-requests`, read 2026-08-16). Requests carry `X-Event-Name` and `X-Signature` headers, and "If a webhook is configured to send a request to an HTTPS URL, the SSL certificate of the URL will be verified before sending the request." **Anything built on `order_refunded` inherits a delivery that gives up after four attempts inside about three minutes.** This is the webhook case [The shape](The%20shape.md) leaves deliberately open pending Fact 63.

**Checkout custom data reaches licence events.** "If you have added custom data to your checkout, you will find a custom_data object inside the meta field for all Order, Subscription and License key related events" (*Webhook Requests*, address above). It is set at checkout, on the seller's site, per purchase — it is not a per-computer store and cannot hold a trial record.

**A key's expiry is an attribute, and expiry is evaluated by the service.** `expires_at`, documented above, writable by `PATCH` with an API key, and `status: expired` is the service's own verdict on it. Bears on Row 301 and on any arrangement that wants a service-held clock.

**Subscription keys expire with the subscription.** "the license key will remain active as long as the subscription is active and will only expire when the subscription expires" (*License Keys and Subscriptions*, `https://docs.lemonsqueezy.com/help/licensing/license-keys-subscriptions`, read 2026-08-16). Access forbids a subscription (Row 301), so this path is not open.

---

## Every address read, 2026-08-16

**Thirty-five addresses were opened by this reading.** All 35 were re-fetched by the citation check of 2026-08-16 and all 35 returned HTTP 200 serving the page named here; no address is dead and none serves a different page.

**One address quoted above was not opened by this reading:** `https://docs.lemonsqueezy.com/api`, the API Reference root, whose deprecation policy, main-API rate limit and 429 statement are carried from the citation check rather than read here. It is linked from the navigation of every page below, and missing it is what produced the one struck silence in this document.

`https://docs.lemonsqueezy.com/api/license-api` · `/api/license-api/activate-license-key` · `/api/license-api/validate-license-key` · `/api/license-api/deactivate-license-key` · `/api/license-keys/the-license-key-object` · `/api/license-keys/update-license-key` · `/api/license-keys/list-all-license-keys` · `/api/license-key-instances/the-license-key-instance-object` · `/api/license-key-instances/list-all-license-key-instances` · `/api/getting-started/requests` · `/api/getting-started/responses` · `/api/getting-started/changelog` · `/api/webhooks/the-webhook-object` · `/api/orders/issue-refund` · `/api/orders/the-order-object` · `/guides/tutorials/license-keys` · `/help/licensing` · `/help/licensing/generating-license-keys` · `/help/licensing/license-keys-subscriptions` · `/help/webhooks` · `/help/webhooks/event-types` · `/help/webhooks/webhook-requests` · `/help/webhooks/signing-requests` · `/help/payments/refunds-chargebacks` · `/help/payments/merchant-of-record` · `/help/orders/refund-order` · `/help/orders/resend-receipt` · `/help/online-store/my-orders` · `/help/online-store/customer-portal` · `/help/checkout/payment-methods` · `/help/checkout/customizing-receipt-emails` · `/help/checkout/passing-custom-data` · `/help/products/free-trials` · `/help/products/lead-magnets` · `/help/products/adding-products`

---

## What the citation check changed, 2026-08-16

Recorded so a later reader can tell a corrected claim from an original one. **84 claims ruled: 76 supported, 6 narrowed, 2 struck, 0 not found.**

**Struck.** The `valid` example's expiry dates — the page shows expiry a year *after* creation, and no example anywhere shows the reverse; the claim is rewritten to the stronger true version above. And deprecation as a documented silence — the vendor documents a versioning and deprecation policy on a page this reading did not open.

**Narrowed.** The changelog's dated-entry count; the `status`-to-`valid` relationship, which is asserted for `inactive` and directed at by the tutorial; Clause 4, which gains a documented 429 and 5xx as a fourth outcome; the *Refund Order* page's "complete body"; gap 2's chargeback signal; and gap 1's fallback, which is a manual dashboard act only if the tutorial wins the live contradiction.

**Added.** A third internal page defect on *Validate a License Key*; the `filter[order_id]` route that completes an automatable refund-to-revocation chain; the main API's 300-per-minute limit; `activation_usage`'s reliance on tutorial prose; and the one documented billing-event-to-licence-status propagation, which exists for subscriptions only.

**Unmoved and confirmed harder than this reading could prove.** Four negatives were re-searched across all 232 pages and nothing was found that this reading missed: no enumerated `error` strings, no documentation of an invalid key, no uniqueness on `instance_name`, and no record against a computer that has bought nothing. The unauthenticated-credential finding stands verbatim. The retrieval warning stands and is reinforced.

**Still not ruled here: the build-or-buy fork.**

---

# Later addition — Fact 87, read 2026-08-17

**This section was written after the document above and is not covered by the citation check of 2026-08-16.** Everything above is a reading of **2026-08-16**; everything below is a reading of **2026-08-17** and carries that date on every quote. Nothing above is restated or revised here. One claim above and two in Licence service are contradicted by what is below; the contradiction is written here and the old text is left standing.

**Fact 87 — whether the seller's refunded-order event distinguishes a full refund from a partial one, and what the order record carries about the amount refunded.**

**Verdict: closed outright, on the vendor's own reference documentation.** The **event** does not distinguish — its one-line description covers both cases and is the same line already recorded above. **The Order object it delivers does distinguish, in three separate documented fields**, one of which is defined in exactly those terms. The amount refunded is carried in three more.

**Retrieval, and what worked.** The agent's own page-fetching tool returned **HTTP 403 Forbidden** on `https://docs.lemonsqueezy.com/api/orders/the-order-object`, reproducing the warning above. **Direct anonymous HTTPS retrieval returned HTTP 200 on the identical address**, and the same route was extended to the whole set: **the site's own navigation was crawled to closure and yielded 266 internal addresses, of which 232 returned HTTP 200, 32 returned HTTP 308 section-index redirects, and 2 returned 404** (`/api/affiliates`, and a mail-obfuscation path). **The 232 figure matches the citation check's enumeration of 2026-08-16 exactly.** Nothing was run, no account exists, no request was made to `api.lemonsqueezy.com`, and every retrieval was anonymous.

---

## The event does not distinguish; the object it carries does

**Documented.** The event's description is one line and covers both cases:

> "order_refunded | Order object | Occurs when a full or partial refund is made on an order."
> — *Event Types*, `https://docs.lemonsqueezy.com/help/webhooks/event-types`, read 2026-08-17

**Documented — the payload is the Order object, stated as a general rule rather than for this event alone:**

> "The webhook body will be a JSON:API resource object that relates to the event (e.g. “order” events will be sent an order object, “subscription” events will be sent a subscription object, etc)."
> — *Webhook Requests*, `https://docs.lemonsqueezy.com/help/webhooks/webhook-requests`, read 2026-08-17

So the discriminator, if there is one, is a field of the Order object and not a property of the event.

## The Order object's refunded attributes, enumerated

**Documented. *The Order Object* enumerates all of them on its own reference page.** Three tell full from partial; three carry the amount; one gives the denominator.

**The boolean, and it is defined in exactly the terms Fact 87 asks about:**

> "refunded — Has the value true if the order has been fully refunded."
> — *The Order Object*, `https://docs.lemonsqueezy.com/api/orders/the-order-object`, read 2026-08-17

**The timestamp, carrying the same restriction in its own wording:**

> "refunded_at — If the order has been fully refunded, this will be an ISO 8601 formatted date-time string indicating when the order was refunded."
> — same page

**The status, whose value list contains a distinct name for each case:**

> "status — The status of the order. One of:"
> "pending"
> "failed"
> "paid"
> "refunded"
> "partial_refund"
> "fraudulent"
> — same page

**The amount, in three forms, plus the order total to measure it against:**

> "refunded_amount — A positive integer in cents representing the refunded amount of the order in the order currency."
> "refunded_amount_usd — A positive integer in cents representing the refunded amount of the order in USD."
> "refunded_amount_formatted — A human-readable string representing the refunded amount of the order in the order currency (e.g. $9.99)."
> "total — A positive integer in cents representing the total cost of the order in the order currency."
> — same page

**Documented, and dated by the vendor itself.** The API changelog dates the three amount fields and states what the boolean was narrowed to mean:

> "August 7th, 2024"
> "Added refunded_amount, refunded_amount_usd and refunded_amount_formatted parameters to Order objects and Subscription invoice objects"
> "The refunded property of Order objects now only"
> "The refunded property of Subscription invoice objects now only reflects whether the invoice has been fully refunded"
> — *API Changelog*, `https://docs.lemonsqueezy.com/api/getting-started/changelog`, read 2026-08-17

**The Order-object sentence is truncated mid-clause on the vendor's page and that is not a transcription error here** — it ends at "now only" in the page's own HTML, with no further text node in the list item. The parallel sentence for Subscription invoice objects is complete, and *The Order Object*'s own definition of `refunded` completes it to the same meaning. **Recorded as an internal page defect, in the manner of the three already recorded above.**

**The changelog's most recent entry is still dated July 29th, 2026**, unchanged from the reading of 2026-08-16.

## The vendor's own worked partial refund, and where it disagrees with its own status list

**Documented — a partial refund is an amount, and omitting the amount means the whole:**

> "Issues a partial or full refund for an order."
> "amount Optional — Optional. The amount that you want to refund for an order. If not included full refund will be issued."
> "Returns an Order object with updated refunded attributes."
> — *Issue a Refund*, `https://docs.lemonsqueezy.com/api/orders/issue-refund`, read 2026-08-17

**The page's own worked example is a partial refund**, requesting `"amount": 100` against an order whose `"total": 1199`, and its returned Order object reads:

> `"total": 1199,`
> `"refunded_amount": 100,`
> `"status": "paid",`
> `"status_formatted": "Paid",`
> `"refunded": false,`
> `"refunded_at": null,`
> `"refunded_amount_formatted": "$1.00",`
> — same page, Returns example

**Two things follow, and one of them is a disagreement.**

**Label: documented.** After a partial refund the boolean reads `false`, the timestamp reads `null`, and `refunded_amount` is non-zero and smaller than `total`. That is the vendor's own worked demonstration of the distinction its `refunded` definition asserts.

**Label: disagreement — the vendor's status list against the vendor's only worked partial refund.** *The Order Object* enumerates `partial_refund` as an order status; the one place in the whole set where a partial refund is actually carried out returns `"status": "paid"`. **The two cannot both be right about what `status` reads after a partial refund, and no page reconciles them.** Neither is picked here. What survives the disagreement untouched is the `refunded` boolean, the `refunded_at` timestamp and the `refunded_amount`-against-`total` comparison, all three of which the same example confirms rather than contradicts.

**And `partial_refund` is never defined for an order.** *The Order Object* lists the six status values with no gloss on any of them. **The only place in the set where those two names are defined is the analogous field on the subscription-invoice object:**

> "refunded - The invoice was paid but has since been fully refunded."
> "partial_refund - The invoice was paid but has since been partially refunded."
> "refunded — A boolean value indicating whether the invoice has been fully refunded."
> "refunded_at — If the invoice has been fully refunded, this will be an ISO 8601 formatted date-time string indicating when the invoice was refunded. Otherwise, it will be null."
> — *The Subscription Invoice Object*, `https://docs.lemonsqueezy.com/api/subscription-invoices/the-subscription-invoice-object`, read 2026-08-17

Access forbids a subscription (Row 301), so that object is not this service's; it is quoted only because it is where the vendor states what its own two status names mean.

## The examples lag the field list

**Documented, and it bears on how much an example is worth here.** The three amount fields were added in August 2024 and **do not appear in any Order-object example body in the set except the refund endpoint's own.** *The Order Object*'s example, *Retrieve an order*, *List all orders* and *Webhook Requests* all show `"refunded": false, "refunded_at": null` and no `refunded_amount` of any kind; the *Example Payloads* page's `order_created` body shows `"refunded": null`, a third value the field's own definition does not admit. Addresses: `/api/orders/the-order-object`, `/api/orders/retrieve-order`, `/api/orders/list-all-orders`, `/help/webhooks/webhook-requests`, `/help/webhooks/example-payloads`, all read 2026-08-17.

## What is still not stated

**Not stated — which fields the `order_refunded` body actually carries.** The set documents that an order event is sent "an order object" and documents the Order object's attributes, but **no page shows an `order_refunded` payload.** *Example Payloads* carries four examples — `order_created`, `subscription_created`, `subscription_cancelled`, `subscription_paused` — and no refund event among them (`/help/webhooks/example-payloads`, read 2026-08-17). Given the previous section, the gap that matters is whether the delivered body includes the 2024 amount fields. **Searched across all 232 pages: `refunded_amount` occurs on six pages, none of them a webhook page.**

**Not stated — how many times the event fires.** No page states whether a second partial refund on an already-partly-refunded order fires `order_refunded` again, nor whether a partial refund that later becomes whole fires it twice.

**Not stated — anything joining a refund to a licence.** **Searched at sentence level across all 232 pages for any sentence containing both a refund word and `licence`/`license`: zero matches.** Nothing states what a partial refund does to a key, and nothing states what a full one does either. **This confirms the silence recorded above rather than narrowing it**, and it bounds Fact 87: the distinction below is available in the order record, and the vendor says nothing about what either case means for entitlement.

**One route to observing this exists and was not taken.** The vendor documents that `order_refunded` is one of two order events that can be sent by hand from a test-mode store — "For more manual testing, you can simulate webhook events for any test mode order or subscription" (*Simulate Webhook Events*, `https://docs.lemonsqueezy.com/help/webhooks/simulate-webhook-events`, read 2026-08-17). It needs an account, which every agent on this stage is barred from creating, so **every claim above remains a documented assertion and none is an observation.**

## How widely the negatives were searched

All 232 pages were re-enumerated and re-fetched on **2026-08-17** by the direct-HTTPS route, converted to text, and searched whole. Counts: `partial` (any case) on **7** pages; `partial_refund` on **2**; `refunded_amount` on **6**; `refunded_at` on **12**; `order_refunded` on **5**. The sentence-level refund-and-licence search returned **0**.

## What this contradicts in what is already written

**Recorded here and not corrected there, as the discipline requires.**

**One. Licence service Section 9 states that the refunded attributes are unenumerated.** Its *Full against partial* paragraph reads that the vendor's refund endpoint "documents only *"Returns an Order object with updated refunded attributes"* and no page read enumerates them", and Section 15's item 5 repeats it. **The Order Object page enumerates all six of them, and that page is in this document's own address list of 2026-08-16 as having been read.** The quoted sentence about the refund endpoint is true of that endpoint's page; the conclusion drawn from it is not true of the set, and the field that decides the case — `refunded`, defined as fully refunded — is documented.

**Two. Nothing above is contradicted on the event.** The line "Occurs when a full or partial refund is made on an order" is quoted above from the same page and reads identically on 2026-08-17.

**Three. A second disagreement is added to the three internal page defects already recorded**, and it is between two pages rather than inside one: `partial_refund` as an enumerated order status against the same vendor's only worked partial refund returning `"status": "paid"`.

## Bearing, without deciding anything

**Fact 87 asked two things and both are answered.** The event does not distinguish. **The order record distinguishes in `refunded` — "Has the value true if the order has been fully refunded" — corroborated by `refunded_at`, and quantified by `refunded_amount` against `total`.** A partial refund is therefore documented to be recognisable without reading the contested `status` field at all.

**What this does not settle**, and it is not settled here: whether the delivered webhook body carries the amount fields, and what either kind of refund means for a key, on which the vendor is silent across the whole set. **No decision, plan or recommendation is made here**, in keeping with this document's opening.
