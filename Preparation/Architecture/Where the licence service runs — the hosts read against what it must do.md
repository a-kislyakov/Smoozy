# Where the licence service runs — the hosts read against what it must do

What four candidate hosts' own documentation stated on **2026-08-16** about the things the licence component the Creator ruled for on 2026-08-16 (Fact 9) has to be able to do. An **input** to the stage, like [Lemon Squeezy](Lemon%20Squeezy%20—%20the%20licence%20service%20read%20against%20what%20Access%20needs.md) and [The computer itself](The%20computer%20itself%20—%20its%20name,%20its%20identity,%20and%20its%20clock.md): **nothing here is a decision, a plan or a recommendation. No host is chosen here.** `Licence service` chooses, later, against what is written below.

Every claim carries a label: **documented**, **not stated**, or **disagreement**. Every documented claim carries the exact words, the address and the date read. A claim that could not be quoted is not here.

Rule 1's plain-English rule is suspended, per the stage's exception.

---

## What a citation check changed, 2026-08-16

A fresh agent re-fetched the forty-four addresses this reading first rested on, and twenty-seven further pages of the same four documentation sets, and ruled 172 claims: 153 stood, 14 were narrowed, 5 were struck. **This said 171 claims, of which 4 were struck** — corrected 2026-08-31. The check's own count table, heading and signature all say four, and this document inherited that; the check's ruling 10, *"Fly.io carries no date of any kind"*, is struck in its body and appears in neither the table nor the four-item list, because the narrower sentence that survives it was counted as supported. That is how 153 + 14 + 4 reached 171. The struck vendor-wide claim is already gone from limit three, which carries the narrower form. Everything it struck or narrowed has been fetched again and confirmed against the vendor's own page before being written in here. **Four of its findings change what this document says, and three of them make a candidate look better rather than worse.**

- **Hetzner's full current price list is public, anonymous and complete** — served as JSON at an address the site's own script names. The observations that led to the earlier conclusion all reproduce; the conclusion does not. **Hetzner's price is the freshest figure in this document, not the one it could not read.** Limit four, and the Hetzner price section.
- **Cloudflare documents what consistency an unreplicated D1 read offers**, on a page linked from every D1 page's sidebar: every read and every write goes to one primary instance unless the Creator's own code asks a session for a replica. **That is R4 answered outright rather than a silence.** Cloudflare R4.
- **Fly.io publishes resume and cold-start figures** — a few hundred milliseconds, and about two seconds. **Corrected 2026-08-21: the seller's five seconds is the gap before its second delivery attempt, not a deadline the receiver must answer inside** — the vendor's sentence governs what happens *"If the response status is anything else"*, so a status has already been produced. **The vendor publishes no response deadline**: three of its webhook pages were searched on 2026-08-20 for `timeout`, `respond within` and five neighbouring terms, with zero matches and a working control. `Licence service` §15 item 4 carries the reopened question. **This is the one place the wrong ruler could bite**: about two seconds was waved through against five, and against an unpublished timeout it is not bounded. Fly.io R8, and silence 6.
- **Fly.io documents that a resumed Machine's clock can lag a few seconds until NTP syncs**, on a service every one of whose successful replies must carry its own statement of the time. Fly.io R7.

Six further additions the check surfaced are in place below: Supabase's point-in-time recovery price and its retirement behaviour, Fly.io's removable TLS termination and its free trial, Cloudflare's Durable Objects free-plan limits, and one price Hetzner's prose documentation does carry.

---

## Five limits, before anything else

**One. Nothing here was run. Every claim is a documented assertion, never an observation.** No account exists at any of the four vendors, nothing was signed up for, no email address was supplied, no card was entered, not one cent was spent, and not one authenticated request was made. No service was deployed, no database was created, no endpoint was called. The distinction [The computer itself](The%20computer%20itself%20—%20its%20name,%20its%20identity,%20and%20its%20clock.md) draws between documented and observed applies with full force: **every behaviour below is what a vendor says its platform does.** In particular, **no latency, no reachability and no availability figure below was measured** — where one appears it is a vendor's own published number.

**Two. Pricing is the fastest-rotting thing in this document, and a figure read in August says nothing about September.** Every price below carries the address and the date it was read. Two of the four vendors moved a price inside the last eight months by their own record: Fly.io's pricing page carries *"Starting January 1st 2026, we're introducing charges for volume snapshot storage"*, followed by *"You'll see the first charges on the invoice issued at the start of February 2026"*, and Cloudflare's Workers pricing page is stamped *"Last updated Jul 7, 2026"*. **The freshest price figures here are Hetzner's, and they rot fastest of all**: they come from that vendor's own live price feed, a machine-readable document whose only version is an HTTP `last-modified` header, stamped the day before the date above (limit four). **Nothing downstream may treat a figure here as current.**

**Three. Only two of the four vendors date their documentation, and one of those two dates the build rather than the page.**

- **Cloudflare** stamps every documentation page with its own revision date — *"Last updated Apr 21, 2026"*, *"Last updated Jul 7, 2026"*, *"Last updated Aug 14, 2026"* — and the dates differ page to page, so each is the page's own.
- **Hetzner** stamps every documentation page with *"Last change on YYYY-MM-DD • Created on YYYY-MM-DD • ID: …"*, and those dates differ page to page too. They are also **old**: the billing page whose worked example is quoted below reads *"Last change on 2020-07-07"*, and the two pages stating the Primary IPv4 price both read *"Last change on 2022-06-28"*. Neither `www.hetzner.com` page carries a stamp at all.
- **Supabase** shows no date on any page. Its pages carry an `article:modified_time` value in the HTML metadata, but **four different pages carried timestamps within one tenth of a second of each other** (`2026-08-14T13:08:21.512Z`, `.515Z`, `.606Z`, `.611Z`), so **that value dates the site build and not the page's content.** Its pricing page carries no such value at all.
- **Fly.io** dates a minority of its pages, and none of the pages the readings below rest on. The pricing page and the volumes page carry no visible date and no `datePublished`, `dateModified`, `article:modified_time` or `lastmod` metadata. Other pages of the same set do carry one, in the source front matter the site serves inside the HTML: `date: 2025-10-27` (*Fly.io Free Trial*), `date: 2025-08-15` (*Machine suspend and resume*), `date: 2025-10-23` (*Cost Management*), `date: 2022-12-08` (*Machine sizing*), all read 2026-08-16. **So the vendor is not uniformly undated — but every page this document leans on is.**

So for every Fly.io page cited below, and for Supabase without qualification, exactly as for Lemon Squeezy, **nothing distinguishes a sentence written last month from one written three years ago.**

**Four. One vendor's server prices are on none of its pages, and are published separately as data at an address that vendor's own script names.** Hetzner's prose documentation carries no server prices; each page says to look at `hetzner.com/cloud` — *"For information about the prices, see the respective info box at hetzner.com/cloud"* (*Backups/Snapshots*, address below). That page **renders its price fields with JavaScript and they are empty on every route tried**: direct HTTPS retrieval returns `ho-price-container` elements carrying the class `price` with an empty slot; a rendered read of `https://www.hetzner.com/cloud/` and of `https://www.hetzner.com/cloud/cost-optimized/` returns the literal page text *"starting from"* followed by *"max/mo."* with nothing between. The guessed address `/_resources/app/data/prices.json` returns the site's own 404 page.

**The figures themselves are public, anonymous and complete.** The site's own script names where they live:

> `oz=` `${oE}/_resources/app/data/app/live_data_prices.json`
> — `https://www.hetzner.com/_resources/themes/static/dist/vendors.js`, read 2026-08-16, where `oE` is `"https://www.hetzner.com"` two characters earlier

> `https://www.hetzner.com/_resources/app/data/app/live_data_prices.json`
> HTTP 200 · `content-type: application/json` · 263,299 bytes · `last-modified: Sat, 15 Aug 2026 22:17:50 GMT` · read 2026-08-16

Retrieved with no account, no cookie, no consent banner and no sign-in, exactly like every other address here. The site serves **two** files named `vendors.js` — `/_resources/themes/hetzner/dist/vendors.js` and `/_resources/themes/static/dist/vendors.js` — and the address is written into the second. A companion document, `live_data_vat.json`, is served the same way. The product keys that index the feed are on the vendor's own pages, as `product-key` attributes beside each server's name.

**So Hetzner's price is the freshest figure in this document rather than the one that could not be read.** What it costs instead is stated where the figures are: a machine-readable feed is not a page of prose, it is versioned only by an HTTP header, and it rots faster than anything else here.

**And one narrowing of this limit's own first sentence.** Hetzner's prose documentation does carry *some* prices, including on a page this reading already cited: *"We do not charge for our Primary IPs of type IPv6. We charge an additional cost of € 0.50/month (excl. VAT) for our Primary IPs of type IPv4"* (*Cloud Servers*, `https://docs.hetzner.com/cloud/servers/overview/`, last change on 2022-06-28, read 2026-08-16). **What holds is narrower than *the documentation carries no prices*: it carries no server prices.**

**One thing that reproduces and is not a price failure at all.** Every per-server row on `https://www.hetzner.com/cloud/cost-optimized/` reads *"not available"*, and that string is a **static** label in the served HTML — `product-highlight-label-NotAvailable`, beside `CX23`, `CAX11` and the rest — as is *"Currently not available"* under the *"Shared Resources"* heading on `/cloud/`. Those are the vendor saying a server cannot be bought. They belong to availability, and are read there.

**Five. Every address was retrieved by direct HTTPS and every one returned HTTP 200 serving the page or document named.** Eighteen at `developers.cloudflare.com`, fourteen at `fly.io`, nine at `supabase.com`, eight at `docs.hetzner.com` and five at `www.hetzner.com` — **fifty-four in all**, of which two are not documentation pages but the price feed and the script naming it. One page was **also** read rendered in a browser, which is not direct HTTPS retrieval: `https://www.hetzner.com/cloud/`, for the reason limit four gives, with no cookie banner accepted and nothing signed into. Nothing here rests on a forum, a blog, a comparison site, a search-result summary, an SDK repository, or any recollection of how any of these platforms work. Where a vendor documents something nowhere, this document says how widely it was looked for.

---

## What the host has to provide, and where each requirement comes from

Ten requirements. Each is derived from a row, a fact, or a scope clause already written down, and each names it. **Nothing here is invented, and nothing is imported from outside this project.**

**R1 — an HTTPS endpoint the desktop application can reach from wherever the user is, at an address Smoozy ships.**
*From:* Row 236 (*"an outage at the other end, or a company network in the way, would otherwise lock out somebody who paid"*), Row 234 (*"A licence is checked from time to time"*), Fact 14. And [Security and licence](Security%20and%20licence.md) §12.4's last row, which requires *"the check is made over a channel authenticated to the service, and a check that could not authenticate the service is *could not be reached* rather than *succeeded*"* — so the endpoint must carry a TLS certificate for a name the check can pin its expectations to.
*Note:* [The shape](The%20shape.md) already settles that Row 239's prohibition on an always-on Smoozy component does **not** cover this service — *"The Access ruling against an always-on Smoozy service therefore does not cover this licence service, which holds that record and nothing else."*

**R2 — the address must be one the Creator controls, so it can move hosts without changing what installed copies call.**
*From:* Row 301 (*"bought once and is the user's forever"*) joined with [The shape](The%20shape.md)'s Licence service scope, which puts *"migration, and retirement"* on the built branch. A desktop application ships the address it calls; if that address is a vendor's own subdomain, leaving the vendor breaks every copy already installed. So a **custom domain** is a requirement, not a preference.

**R3 — a durable store holding the trial record and the two computer slots against a key, and nothing else.**
*From:* Row 279 (*"whether a trial has been used, and which two computers a licence is on, is recorded away from the user's computer"*), Row 230, Row 224, Fact 9. Bounded by [The shape](The%20shape.md): *"It stores no SSH private key, SSH passphrase, server connection identity, Telegram token, job, history entry, file path, render output, or queue state."* The store must survive a process restart, a redeployment, and the host moving the service between machines.

**R4 — one authoritative copy of that record, with writes that can be made idempotent and serialized.**
*From:* Row 224 (*"the record is written on a connection that may be the user's first ever and may be days late, so writing it has to be idempotent — a computer whose record already exists must not be recorded twice"*) and Rows 230 and 231 (a third computer is refused). Two computers can ask for the second slot at the same moment. So the store must offer a transaction or a single-writer primary; a store that answers reads from regional copies that can disagree cannot keep Row 224's idempotency or Row 230's ceiling.

**R5 — the store must survive years, unattended, while nothing is happening.**
*From:* Row 301, whose answer [the rows file](Rows/Access.md) states as *"what a paid copy does if the service it checks against stops answering for good"*. A licence is bought once; the record behind it must still be there when the user opens Smoozy after a quiet year. **This is the requirement a free plan's inactivity clause is read against**, and it is the axis the four candidates were chosen to span.

**R6 — backup and restoration of that store.**
*From:* [The shape](The%20shape.md), Licence service scope on the built branch: *"the service process, deployment boundary, durable store, backup and restoration, availability and failure behaviour, migration, and retirement."* Losing the store refuses every paying user (Row 230) and gives every computer a fresh trial (Row 224). The reading below asks two things of each host: what it takes by itself without being asked, and how far back a restore reaches.

**R7 — the service's own clock, in every successful check.**
*From:* [Security and licence](Security%20and%20licence.md) §12.3 rule 1 — *"Every successful check carries the service's own statement of the current time, and that value — never the local clock alone — is what the count since the last successful check is measured against once a check has ever succeeded"* — which rests on Fact 13's negative and Fact 11's negative. Rows 222, 224, 235. **This is the least demanding requirement of the ten**: any host whose service can read a wall clock it, and not the user, controls satisfies it. It is listed because it is load-bearing and because [Lemon Squeezy](Lemon%20Squeezy%20—%20the%20licence%20service%20read%20against%20what%20Access%20needs.md) is the reason a Creator-run component exists at all.

**R8 — a seller seam that binds to no seller.**
*From:* Fact 10, Row 237 (*"refunded, charged back, or withdrawn — locks Smoozy at once"*), and the standing constraint `Preparation/Groundwork/Open Threads.md` carries — the seller is expected to change, so **nothing may bind to one seller**. Three concrete host capabilities follow: an **inbound HTTPS endpoint** that can take a seller's webhook and write durably before answering it; an **outbound HTTPS call** to a seller's API; and a **place to hold the seller's credential that never reaches the client**, which is the credential Lemon Squeezy's own *Requests* page says *"Do not share your API keys in publicly accessible areas such as GitHub, client-side code, and so forth."* The Lemon Squeezy reading also fixes the window the first of those has to fit inside: that vendor retries a failed webhook *"up to three more times using an exponential backoff strategy (e.g. 5 secs, 25 secs, 125 secs)"*.

**R9 — nothing of the user's may be written to a log, including by the host itself.**
*From:* [Security and licence](Security%20and%20licence.md) §13 rule 4 — *"No licence key, no computer identity, and no part of either is written to any log, including a failed-check record"* — and §13's opening rule that *"Redaction here is never emitted, not filtered before display."* A host that records request URLs, headers or bodies by default is writing exactly what §12.1 and §1.2 say crosses that boundary: the licence key, the local computer identity, and the computer name. So each host is read for **what it logs without being asked, whether that can be turned off, and how long it keeps it.**

**R10 — migration and retirement, at a cost that does not reach the user.**
*From:* [The shape](The%20shape.md)'s Licence service scope (*"migration, and retirement"*), and [Method](Method.md)'s third escalation, which stops an agent where an answer would cost the user *"a place to look, something to type, a thing that can be lost"*. Concretely: the record can be got out in a form another host can read, and stopping does not require the user to do anything. **Hosting cost falls on the Creator**, which [The shape](The%20shape.md) states outright — *"Hosting cost falls on the Creator rather than the user, so this does not return to Specs"* — so price is evidence for the fork rather than a promise to anybody.

---

## Why these four

Chosen to span what happens to a durable store over years, and what a retirement or a migration costs — which is the axis R5 and R10 sit on. One line each.

- **Cloudflare Workers with D1** — named by the Creator as a guess, and the extreme where there is no machine, no operating system and no disk of the Creator's own: the store is the vendor's own product and leaving means exporting it.
- **Fly.io** — one step back: the Creator's own process in a container, writing to a block volume they can copy off, so the store is an ordinary file in a standard format and migration is a file move rather than a translation.
- **Supabase** — the managed middle: somebody else runs the database, but it is Postgres and comes out through `pg_dump`, so the store is portable while the platform is not. Included because it is the only one of the four that documents deleting or pausing an idle project, which is R5's question.
- **Hetzner Cloud** — the far end: a plain rented Linux machine where every one of the ten requirements is the Creator's own to build and keep, retirement is stopping the invoice, and migration is copying a disk.

---

## Cloudflare — Workers with D1

Read as: a Worker serving the endpoint, D1 holding the record. Where Durable Objects would serve R4 better than D1, that is noted; the two are alternatives inside one vendor, not two candidates.

### R1 — the endpoint

**Documented.** A Worker is reached over HTTPS on a route or a custom domain and is the origin for it:

> "Custom Domains allow you to connect your Worker to a domain or subdomain, without having to make changes to your DNS settings or perform any certificate management. After you set up a Custom Domain for your Worker, Cloudflare will create DNS records and issue necessary certificates on your behalf."
> — *Custom Domains*, `https://developers.cloudflare.com/workers/configuration/routing/custom-domains/`, last updated Aug 14, 2026, read 2026-08-16

> "Creating a Custom Domain will also generate an Advanced Certificate on your target zone for your target hostname."
> — same page

**The reachability half is where this candidate has a documented exclusion, and it is the only one of the four that has one.** Cloudflare's own China Network page states both the problem and the price of solving it:

> "Delivering content quickly and securely to users in Mainland China requires infrastructure within China itself. Traffic routed through servers outside the country faces significant latency and reliability issues due to China's network boundaries."
> "The Cloudflare China Network is available as a separate subscription for customers on an Enterprise plan."
> "You must have a valid ICP (Internet Content Provider) filing or license for each apex domain you wish to onboard to Cloudflare."
> — *Cloudflare China Network*, `https://developers.cloudflare.com/china-network/`, last updated Apr 30, 2026, read 2026-08-16

**Label: documented, and it bears on Row 236 without settling it.** Cloudflare documents that traffic from Mainland China served from outside the country faces *"significant latency and reliability issues"*, and that the remedy is an Enterprise subscription plus a filing per apex domain. It does **not** say the endpoint is unreachable, and nothing on any page read says a company network blocks it. Row 236's *"a company network in the way"* is unaddressed here as it is everywhere else — see the last section.

### R2 — an address the Creator controls

**Documented, with one condition that has a migration cost of its own.**

> "To add a Custom Domain, you must have: An active Cloudflare zone . A Worker to invoke."
> "Custom Domains are routes to a domain or subdomain (such as example.com or shop.example.com ) within a Cloudflare zone where the Worker is the origin."
> — *Custom Domains*, address above

> "You cannot create a Custom Domain on a hostname with an existing CNAME DNS record or on a zone you do not own."
> — same page

**So the domain's own zone has to live at Cloudflare.** That is a heavier condition than the two other managed candidates impose — Supabase documents *"Custom domains can only be powered by CNAME records"* and Fly.io documents CNAME, A/AAAA and ACME challenge routes — and it is what R10 pays for: moving the service off Cloudflare also means moving the zone, or repointing it while it is still hosted there.

### R3 — the durable store

**Documented.** D1 is a SQL database bound to a Worker; storage limits are documented per plan:

> "Maximum database size | 10 GB (Workers Paid) / 500 MB (Free)"
> "Maximum storage per account | 1 TB (Workers Paid) 2 / 5 GB (Free)"
> "Maximum number of rows per table | Unlimited (excluding per-database storage limits)"
> — *Limits*, `https://developers.cloudflare.com/d1/platform/limits/`, last updated Apr 21, 2026, read 2026-08-16

**A trial record and two slots per key are small.** The vendor's own arithmetic is that *"An empty database consumes approximately 12 KB of storage"* and *"A row that is 1 KB and a row that is 100 KB both count as one row"* (*Pricing*, address below). Nothing on any page read gives a per-row byte figure for a populated table, so **no estimate of how many users fit in 500 MB is derivable from the documentation**, and none is made here.

### R4 — one authoritative copy, and serialized writes

**Documented, and this is the clause D1 answers well.** There is one primary:

> "By default, D1 will automatically create your primary database instance in a location close to where you issued the request to create a database."
> "Workers may still access the database constrained to a jurisdiction from anywhere in the world. The jurisdiction constraint only controls where the database itself runs and persists data."
> — *Data location*, `https://developers.cloudflare.com/d1/configuration/data-location/`, last updated Apr 21, 2026, read 2026-08-16

and a documented transaction:

> "Our implementation guarantees that each statement in the list will execute and commit, sequentially, non-concurrently."
> "Batched statements are SQL transactions . If a statement in the sequence fails, then an error is returned for that specific statement, and it aborts or rolls back the entire sequence."
> — *D1 Database*, `https://developers.cloudflare.com/d1/worker-api/d1-database/`, last updated Jun 22, 2026, read 2026-08-16

**Read replication is opt-in and labelled Beta** in the product's own navigation (*Global read replication Beta*, listed on every D1 page's sidebar), and the pricing page states *"Read replication does not charge extra for read replicas"* — so a replicated read path exists and is not the default. **What the page behind that sidebar entry documents is R4's requirement answered outright:**

> "When using D1 without read replication, D1 routes all queries (both read and write) to a specific database instance in one location in the world, known as the primary database instance."
> "To use read replication, you must use the D1 Sessions API, otherwise all queries will continue to be executed only by the primary database."
> "All write queries are still forwarded to the primary database instance."
> "Sessions API ensures sequential consistency for all queries in a session."
> — *Global read replication*, `https://developers.cloudflare.com/d1/best-practices/read-replication/`, last updated Aug 10, 2026, read 2026-08-16

**Read that against R4 and it is stronger than the requirement asks.** Without the Sessions API every read and every write goes to one instance, so Row 224's idempotency and Row 230's ceiling have a single authority to rest on; and enabling replication cannot silently remove it, because a read only leaves the primary when the Creator's own code asks a session for it. The same page names the hazard that would apply to a service that did use replication — *"at any given time, a read replica may be arbitrarily out of date"* — and calls the gap *"replica lag"*.

**Durable Objects are the vendor's own answer to serialization, and they are on the free plan:**

> "Each Durable Object has a globally-unique name , which allows you to send requests to a specific object from anywhere in the world. Thus, a Durable Object can be used to coordinate between multiple clients who need to work together."
> "Each Durable Object has some durable storage attached. Since this storage lives together with the object, it is strongly consistent yet fast to access."
> "Available on Free and Paid plans"
> — *Durable Objects*, `https://developers.cloudflare.com/durable-objects/`, last updated Jul 15, 2026, read 2026-08-16

**On the free plan the product is restricted, and the restriction is on a page of its own:**

> "Durable Objects are available both on Workers Free and Workers Paid plans."
> "Workers Free plan: Only Durable Objects with SQLite storage backend are available."
> "If you exceed any one of the free tier limits, further operations of that type will fail with an error."
> "Requests | 100,000 / day" · "Duration3 | 13,000 GB-s / day"
> — *Pricing*, `https://developers.cloudflare.com/durable-objects/platform/pricing/`, last updated Aug 10, 2026, read 2026-08-16

So on the free plan this route is the SQLite-backed object only, at 100,000 requests and 13,000 GB-seconds a day, and past either the operation fails rather than degrading — the same refusal shape D1's own free ceiling produces, below.

### R5 — surviving years unattended

**Not stated, and searched for.** No page read states that a D1 database, a Worker or a Cloudflare account is paused, suspended, archived or deleted for inactivity. The nearest thing to a commitment on the free plan is a promise about the plan rather than about a database:

> "Will D1 always have a Free plan? Yes, the Workers Free plan will always include the ability to prototype and experiment with D1 for free."
> — *Pricing*, `https://developers.cloudflare.com/d1/platform/pricing/`, last updated Apr 21, 2026, read 2026-08-16

**Read the two halves of that sentence separately.** *"Always"* is stated of the plan's existence. What the plan is *for* is stated as *"to prototype and experiment"*, which is a description of a use rather than a limit, and no page read attaches any consequence to using it for something else. **Nothing states that data on the free plan is retained indefinitely, and nothing states that it is not.** Searched across the D1 pricing, limits, Time Travel, export, data-location and overview pages, the Workers pricing and limits pages, and the Workers logs page for `inactive`, `inactivity`, `dormant`, `suspend`, `archive`, `reclaim` and `delete your database`: nothing.

**What is documented is what happens at the free limits, and it is a refusal rather than a deletion:**

> "When your account hits the daily read and/or write limits, you will not be able to run queries against D1. D1 API will return errors to your client indicating that your daily limits have been exceeded. Once you have reached your included storage limit, you will need to delete unused databases or clean up stale data before you can insert new data, create or alter tables or create indexes and triggers."
> — *Pricing*, address above

**That matters to Row 227 and Row 234 rather than to Row 301.** A daily limit reached on the free plan makes the check answer an error, which is a refusal shape Row 227 must not confuse with *this key is invalid*; the count of checks against the free ceiling is `Licence service`'s to work out and is not worked out here.

### R6 — backup and restoration

**Documented, and it is the strongest of the four on this clause.**

> "Time Travel is D1's approach to backups and point-in-time-recovery, and allows you to restore a database to any minute within the last 30 days."
> "You do not need to enable Time Travel. It is always on."
> "Database history and restoring a database incur no additional costs."
> "Time Travel automatically creates bookmarks on your behalf. You do not need to manually trigger or remember to initiate a backup."
> — *Time Travel and backups*, `https://developers.cloudflare.com/d1/reference/time-travel/`, last updated Apr 21, 2026, read 2026-08-16

The window is plan-dependent and the boundary is hard:

> "Time Travel duration (point-in-time recovery) | 30 days (Workers Paid) / 7 days (Free)"
> — *Limits*, address above

> "Bookmarks older than 30 days are invalid and cannot be used as a restore point."
> — *Time Travel*, address above

**Two documented properties of a restore that a downstream answer has to carry:**

> "Restoring a database to a specific point-in-time is a destructive operation, and overwrites the database in place."
> "Queries in flight will be cancelled, and an error returned to the client."
> — *Time Travel*, address above

So a restore refuses checks while it runs, which is Row 227's third outcome (*the check could not be reached*) arriving from the Creator's own maintenance.

### R7 — the service's own clock

**Not stated as a feature, and it does not need to be.** No page read documents a clock API, and none needed to: a Worker is a program the Creator runs, and the requirement is only that the time it reads is not the user's. Searched the Workers limits, pricing and known-issues pages for `Date`, `clock` and `time source`: **the only clock statement found is about a limit rather than a source** — *"Max of 5 minutes of CPU time per invocation (default: 30 seconds)"*, in the **Standard** (paid) row of the pricing table (*Pricing*, `https://developers.cloudflare.com/workers/platform/pricing/`, last updated Jul 7, 2026, read 2026-08-16). **Label: not stated, and nothing rests on it.** The same holds for Supabase and for Hetzner and is not repeated below. **It does not hold for Fly.io**, which documents a clock behaviour of its own — read under that vendor's R7.

### R8 — the seller seam

**Documented in all three parts.**

Inbound is the same HTTPS endpoint as R1. Outbound is the platform's own `fetch`, documented by the vendor as the way a Worker reaches an external dependency — *"If you do have external dependencies, you can create a Request object with the target URI, and use fetch() to reach out"* (*Custom Domains*, address above) — and billed as free: *"Cloudflare does not bill for subrequests you make from your Worker"* (*Pricing*, address above).

The credential:

> "Secrets are a type of binding that allow you to attach encrypted text values to your Worker. Secrets are used for storing sensitive information like API keys and auth tokens."
> "Secrets are environment variables . The difference is secret values are not visible within Wrangler or Cloudflare dashboard after you define them."
> — *Secrets*, `https://developers.cloudflare.com/workers/configuration/secrets/`, last updated Jul 3, 2026, read 2026-08-16

Scheduled work, which a seller seam may want for reconciliation:

> "Cron Triggers allow users to map a cron expression to a Worker using a scheduled() handler that enables Workers to be executed on a schedule."
> — *Cron Triggers*, `https://developers.cloudflare.com/workers/configuration/cron-triggers/`, last updated Jun 20, 2026, read 2026-08-16

### R9 — logging

**Documented, and it is on by default.** This is the clause that most needs acting on if this candidate is chosen.

> "All newly created Workers will come with the observability setting enabled by default."
> "Each Workers invocation returns a single invocation log that contains details such as the Request, Response, and related metadata."
> "In the Workers Logs UI, logs are presented with a localized timestamp and a message. The message is dependent on the invocation handler. For example, Fetch requests will have a message describing the request method and the request URL"
> — *Workers Logs*, `https://developers.cloudflare.com/workers/observability/logs/workers-logs/`, last updated Aug 11, 2026, read 2026-08-16

**A licence check whose key or computer identity travels in the URL would land in that log by default.** It can be turned off, and the vendor documents exactly how:

> "Invocation logs can be disabled in wrangler by adding the invocation_logs = false configuration."
> — same page

Retention is bounded either way:

> "Maximum log retention period | 7 Days"
> "Workers Free | 200,000 per day | 3 Days" · "Workers Paid | 20 million included per month +$0.60 per additional million | 7 Days"
> — same page

**What is not stated:** no page read says whether request bodies are captured, or what *"details such as the Request, Response, and related metadata"* includes beyond the method and URL named in the message. Searched the Workers Logs page in full: `body`, `payload` and `redact` do not appear at all, and `header` appears only in the page's own navigation and inside a code sample setting a response's `content-type` — never describing what is captured.

### R10 — migration and retirement

**Documented, with one stated limit.**

> "you can export a D1 database to a .sql file using wrangler d1 export and then execute (import) with d1 execute --file ."
> "npx wrangler d1 export <database_name> --remote --output=./database.sql"
> — *Import and export data*, `https://developers.cloudflare.com/d1/best-practices/import-export-data/`, last updated Apr 21, 2026, read 2026-08-16

> "A running export will block other database requests."
> — same page, *Known limitations*

The output is SQL text rather than a SQLite file, and importing into D1 from elsewhere needs hand editing — *"You cannot import a raw SQLite database ( .sqlite3 files) directly"*, and the documented conversion asks the reader to *"Remove BEGIN TRANSACTION and COMMIT; from the file"* and to remove D1's own reserved `_cf_KV` table. **So the store comes out in a portable format and goes in with a documented amount of hand work.**

### Price, read 2026-08-16

**Free plan.** *"By default, users have access to the Workers Free plan."* On that plan the Workers pricing table gives **100,000 requests per day**, **no charge for duration**, and **10 milliseconds of CPU time per invocation** — one row's three cells under the column headers *Requests*, *Duration* and *CPU time*, rather than three sentences printed together (*Pricing*, `https://developers.cloudflare.com/workers/platform/pricing/`, last updated Jul 7, 2026). D1: *"Rows read | 5 million / day"*, *"Rows written | 100,000 / day"*, *"Storage (per GB stored) | 5 GB (total)"* (*D1 Pricing*, address above).

**Paid plan.** *"The Workers Paid plan includes Workers, Pages Functions, Workers KV, Hyperdrive, and Durable Objects usage for a minimum charge of $5 USD per month for an account."* Beyond it: *"10 million included per month +$0.30 per additional million"* requests, *"30 million CPU milliseconds included per month +$0.02 per additional million CPU milliseconds"*. D1 on that plan: *"First 25 billion / month included + $0.001 / million rows"* read, *"First 50 million / month included + $1.00 / million rows"* written, *"First 5 GB included + $0.75 / GB-mo"* storage. *"There are no additional charges for data transfer (egress) or throughput (bandwidth)."*

**A custom domain needs a zone at Cloudflare**, and no page read states the price of holding a zone there. Searched the custom-domains page and both pricing pages: not stated.

---

## Fly.io

Read as: the Creator's own program in a container, on one or more Machines, writing to a Fly Volume.

### R1 — the endpoint

**Documented.**

> "When you create a Fly App, it is automatically given a fly.dev subdomain, based on the app's name. This is great for testing, but when you want to go to full production you'll want your application to appear on your own domain and have HTTPS set up for you as it is with your .fly.dev domain."
> — *Custom domains and TLS certificates*, `https://fly.io/docs/networking/custom-domain/`, read 2026-08-16

> "Each application receives a shared IPv4 address and unlimited Anycast IPv6 addresses for global load balancing."
> "Dedicated IPv4 addresses are $2/mo."
> — *Fly.io Resource Pricing*, `https://fly.io/docs/about/pricing/`, read 2026-08-16

### R2 — an address the Creator controls

**Documented, and the DNS stays wherever the Creator keeps it.**

> "After you have configured your DNS, Fly.io automatically validates your domain ownership and issues certificates."
> "**TLS-ALPN challenge**: Validates through a TLS handshake with Fly Proxy. This is the preferred method and works automatically for direct connections."
> "In addition to the automatic Let's Encrypt certificates that Fly.io manages for you, you can upload your own TLS certificate and private key."
> — *Custom domains*, address above

> "Single hostname certificates: $0.10/mo" · "Every organization's first 10 single hostname certificates are free."
> — *Pricing*, address above

**No zone has to move**, which is the clearest difference from Cloudflare on R2 and R10 both.

### R3 and R5 — the durable store, and whether it survives

**Documented, and this vendor's own documentation is the sharpest warning of the four.**

> "Fly Volumes are local persistent storage for Machines."
> — *Pricing*, address above

> "**Create and store backups**: If you only have a single copy of your data on a single volume, and that drive fails, then the data is lost. Fly.io takes daily snapshots and can retain them from 1 to 60 days (5 days by default), but the snapshots shouldn't be your primary backup method."
> — *Fly Volumes overview*, `https://fly.io/docs/volumes/overview/`, read 2026-08-16

> "**Warning: Always provision at least two volumes per app.** Running an app with a single Machine and volume leaves you at risk for downtime and data loss. Volumes don't have built-in replication between them, so your app or database needs to take care of replicating data between volumes."
> — same page

> "We try to keep individual Fly Machines up for as long as possible, but hardware failures happen. That's why we recommend running at least two Machines per app to increase availability. You can run two or more Machines in one region, or better yet, one or more Machines in multiple regions. If you only have one Machine and volume, then you'll have downtime if there's a host or network failure, and whenever you deploy your app."
> — same page

> "**Create redundancy in your primary region**: If your app needs a volume to function, and the NVMe drive hosting your volume fails, then that instance of your app goes down. There's no way around that."
> — same page

**Verdict on R3 and R5: documented, and documented to be the Creator's problem.** The vendor states plainly that a single volume is a single copy, that snapshots are not a backup plan, that replication between volumes is *"your app or database"*'s job, and that a single Machine means downtime *"whenever you deploy your app"* — which for a licence service is Row 227's *could not be reached* arriving on every deployment.

**Nothing states that an idle app or an idle volume is deleted or reclaimed.** Searched the volumes, snapshots, pricing, billing, deploy, autostop and support pages for `inactive`, `inactivity`, `dormant`, `reclaim`, `expire` and `unused`: the only match is *"unattached"*, which is a volume without a Machine and is documented as **still billed** — *"You'll be charged for volumes that you create, whether they are attached to a Machine or not, including when an attached Machine is stopped"* (*Pricing*, address above). **So on Fly.io a store survives by being paid for, and stops surviving when it is not** — and every organization must have a card on file: *"All organizations (except for Linked Organizations) require a credit card on file"* (*Pricing*, address above).

### R4 — one authoritative copy, and serialized writes

**Not stated by the vendor, because it is not the vendor's to state.** The store is a file on a volume and the serialization is whatever the Creator's own program does. What the vendor does document is the constraint that shapes the answer: volumes have no replication between them (quoted above), so **two Machines with two volumes are two records unless the Creator's program reconciles them**, and one Machine with one volume is one record with the downtime the vendor names. **Label: not stated, and it lands on `Licence service` rather than on the host.**

### R6 — backup and restoration

**Documented.**

> "A snapshot is a point-in-time copy of a volume's data. We automatically take daily snapshots of all Fly Volumes. We store snapshots for 5 days by default, but you can set volume snapshot retention from 1 to 60 days."
> "If the data stored on your volume updates frequently, then you should have other methods to back up or replicate your data in addition to daily snapshots. If you only have a single copy of your data on a single volume, and the host fails, then any data stored between the time when the snapshot was taken and the time when the failure occurred will be lost."
> — *Manage volume snapshots*, `https://fly.io/docs/volumes/snapshots/`, read 2026-08-16

Restoring is documented as creating a new volume from a snapshot — *"Restore the data from a volume by creating a new volume from a snapshot"* — and snapshots can be turned off: *"Automatic daily snapshots are enabled by default. To disable automatic snapshots, use the --scheduled-snapshots=false flag."*

**The granularity is a day, against Cloudflare's a minute.** That is the sharpest single difference between the two on R6, and the vendor states the consequence itself in the sentence quoted above.

### R7 — the service's own clock

**Not stated as a feature — and this is the one vendor of the four that documents a behaviour of that clock.**

> "On resume, the clock can lag a few seconds until NTP syncs."
> — *Machine suspend and resume*, `https://fly.io/docs/reference/suspend-resume/`, `date: 2025-08-15`, read 2026-08-16

**Read against [Security and licence](Security%20and%20licence.md) §12.3 rule 1**, which makes *"the service's own statement of the current time"* the value every count is measured against: the moment that clock can be wrong is the moment a Machine wakes, and on the autostop/suspend arrangement in R8 below that is the moment a check arrives. The same page's own instruction is *"Always design for both resume and cold start paths."*

**Label: documented.** It is not a bar to R7 and it settles nothing here; it is a fact about this candidate that a downstream answer has to hold.

### R8 — the seller seam

**Documented in part, and by implication for the rest.** The credential:

> "You can specify secrets for your Fly App using the fly secrets command. Secrets allow sensitive values, such as credentials, to be passed securely to your Fly App. The secret is encrypted and stored in a vault."
> "When you set a secret through flyctl , it sends the secret value through our API, which writes to the vault for your specific Fly App. The API servers can only encrypt; they cannot decrypt secret values. Secret values are never logged."
> — *Secrets and Fly Apps*, `https://fly.io/docs/apps/secrets/`, read 2026-08-16

with the vendor's own warning attached:

> "**Warning:** flyctl and our API servers are designed to prevent user secrets from being extracted. However, secrets are available to your application code as environment variables. People with deploy access **can** deploy code that reads secret values and prints them to logs, or writes them to unencrypted data stores."
> — same page

**Outbound is not documented as a feature and is documented as a charge**, which is the nearest thing to a statement that it exists: *"We bill for data leaving your app destined for the public internet"*, at *"$0.02 per GB"* for North America and Europe (*Pricing*, address above). **Label: not stated directly; inferred from a billing statement, and recorded as such.**

**One documented interaction with R8's webhook window.** Fly Proxy can stop Machines when nothing is happening:

> "Use autostop/autostart to automatically start and stop or suspend existing Machines based on incoming requests. Fly Machines are fast to start and stop, and you don't pay for their CPU and RAM when they're in a stopped or suspended state."
> "For apps that don't explicitly specify any autostop/autostart settings in fly.toml , Fly Proxy will automatically start stopped Machines when needed, but won't automatically stop or suspend them."
> — *Autostop/autostart Machines*, `https://fly.io/docs/launch/autostop-autostart/`, read 2026-08-16

**The vendor publishes how long that start takes, on the page its own navigation puts beside autostop:**

> "**Typical performance:**
> - Resume from suspend: a few hundred ms
> - Cold start: ~2+ seconds for common apps"
> "That can make startup take just **hundreds of milliseconds** instead of multiple seconds."
> — *Machine suspend and resume*, address above

> "The proxy will automatically suspend machines during low traffic, checking for idle periods every few minutes, and resume them when requests arrive."
> — same page

**R8's window has no published far edge.** **Corrected 2026-08-21: the seller's five seconds is the gap before its second delivery attempt, not a deadline the receiver must answer inside** — the vendor's sentence governs what happens *"If the response status is anything else"*, so a status has already been produced. **The vendor publishes no response deadline**: three of its webhook pages were searched on 2026-08-20 for `timeout`, `respond within` and five neighbouring terms, with zero matches and a working control. `Licence service` §15 item 4 carries the reopened question. **This paragraph used to say both figures fall inside it, with the far edge at Lemon Squeezy's *"5 secs"*** — corrected 2026-08-21, and the ~2 s cold start is the reading that was waved through on it. The slower of the two is the one that applies after a deployment: the same page states *"Snapshots aren't guaranteed to persist"* and that *"Cold starts may happen if: You deploy a new version of your app … The machine is migrated to a different host … The snapshot file is lost or corrupted … We perform system maintenance or updates."* Suspend is also conditional — a Machine of *"≤ 2 GB"* memory, with *"No swap configured"* and *"No schedule configured"*.

**Label: documented, and it is a bound rather than a silence.**

### R9 — logging

**Documented, and this is the quietest of the four by default.**

> "Fly.io app logs are the stdout from the processes run in your apps: whatever an app outputs becomes a log."
> "We have an app log search feature in Grafana that retains logs for 7 days."
> — *Logging overview*, `https://fly.io/docs/monitoring/logging-overview/`, read 2026-08-16

**Nothing on that page states that Fly Proxy records request URLs, headers or bodies.** Searched the logging-overview, secrets, custom-domain, autostop, pricing and billing pages for `access log`, `request log`, `URL` and `header`, and the proxy, request-header, TLS-termination and log-export pages besides: nothing. **Label: not stated.** It is a silence rather than a documented guarantee.

**Two things the vendor does document, and the second changes what this candidate is worth against §13 rule 4:**

> "The Fly.io platform logs errors to customer log streams. Each log line contains an error code and a field containing that code."
> — *Error codes and troubleshooting*, `https://fly.io/docs/monitoring/error-codes/`, undated, read 2026-08-16

> "If you want to terminate TLS yourself, then you only need to remove the handlers from your services in fly.toml or in your Machine config and we'll forward TCP directly to your app."
> — *TLS termination by Fly Proxy*, `https://fly.io/docs/security/tls-termination/`, undated, read 2026-08-16

**So the app's own log stream also carries the platform's error lines, and the proxy can be taken out of the plaintext path entirely.** What the proxy keeps stays unstated, but on this candidate that silence is answerable by configuration rather than only by hope — at the price of the Creator's own program doing the TLS termination the vendor otherwise does for it.

### R10 — migration and retirement

**Documented for retirement, not stated for export.** Retirement: *"To delete your Fly.io account, go to Account > Settings > Delete Account in your dashboard"*, with a clean-up list including *"Delete apps (or remove yourself from the associated organizations)"* (*Billing*, `https://fly.io/docs/about/billing/`, read 2026-08-16). **Export is not a Fly.io operation at all** — the store is the Creator's own file on a volume they mount, so getting it out is their own program's job and the vendor documents nothing about it. That is R10's best case and the reason this candidate is in the set.

### Price, read 2026-08-16

All from *Fly.io Resource Pricing*, `https://fly.io/docs/about/pricing/`, undated:

- Compute: `shared-cpu-1x`, 1 shared CPU, 256MB — *"$0.00000078"* per second, *"$0.0028"* per hour, *"$2.02"* per month. 512MB — *"$3.32"* per month. 1GB — *"$5.92"* per month.
- Volumes: *"$0.15/GB per month of provisioned capacity"*, billed *"whether they are attached to a Machine or not"*.
- Snapshots: *"$0.08/GB per month"*, *"First 10GB free each month"*.
- Stopped Machines: *"For stopped Machines we charge only for the root file system (rootfs) needed for each Machine. Each 1GB of rootfs for a Machine stopped for 30 days is $0.15."* The same charge, in different words, is on the **billing** page rather than this one — *"Stopped and suspended Machines are billed based on their root file system (rootfs) usage per second (the time they spend in the `stopped` or `suspended` state) by $0.15 per GB per month"* (*Billing for Fly.io*, `https://fly.io/docs/about/billing/`, read 2026-08-16).
- Certificates: first ten single-hostname certificates free, then *"$0.10/mo"*.
- Egress: *"$0.02 per GB"* (North America, Europe).
- **No free plan is documented on that page**, and *"All organizations (except for Linked Organizations) require a credit card on file."* What the vendor documents instead is a trial, on the page the pricing page's own *Related reading* links: *"A free trial on Fly.io includes 2 hours of machine runtime or 7 days of access, whichever comes first"*, and *"If you don't add a payment method by the end of your 7-day trial, or if you use up the included resources, your apps will stop running"* (*Fly.io Free Trial*, `https://fly.io/docs/about/free-trial/`, `date: 2025-10-27`, read 2026-08-16). **A trial that ends is no answer to R5**, and it makes R3 and R5's conclusion — that on this vendor a store survives by being paid for — documented rather than inferred.

---

## Supabase

Read as: an Edge Function serving the endpoint, the project's Postgres holding the record.

### R1 and R2 — the endpoint and the address

**Documented, and the custom domain is a paid add-on.**

> "Custom domains allow you to present a branded experience to your users. These are available as a paid add-on for projects on a paid plan ."
> "Custom domains help you keep your APIs portable for the long term. By using a custom domain you can migrate from one Supabase project to another, or make it easier to version APIs in the future."
> "Custom domains can only be powered by CNAME records."
> "You can only attach a single custom domain to any given Supabase project."
> — *Custom Domains*, `https://supabase.com/docs/guides/platform/custom-domains`, read 2026-08-16 (undated; site build stamp 2026-08-14)

> "Custom Domains | Not included in | $10 per domain per month per project add on"
> — *Pricing*, `https://supabase.com/pricing`, read 2026-08-16 (undated)

**So R2 costs $10 a month on top of the plan**, and the free plan cannot have it at all — which makes the free plan unusable for R2 whatever it does for R5.

**One documented restriction that touches R1 directly:**

> "Serving of HTML content is only supported with custom domains (Otherwise GET requests that return text/html will be rewritten to text/plain )."
> — *Limits*, `https://supabase.com/docs/guides/functions/limits`, read 2026-08-16

That does not bind a JSON licence check, and is recorded because it is the one documented rewrite of a response on this platform.

### R3 and R4 — the durable store, and serialized writes

**Documented, and it is Postgres.** The store is the project's own database; a transaction and a unique constraint are properties of Postgres rather than of this vendor, and **no page read states them**, so nothing is claimed here that the vendor did not write. What the vendor does document is the size ceiling and the compute the store runs on:

> "500 MB database size · Shared CPU • 500 MB RAM"
> — *Pricing*, Free plan, address above

> "Paid plans include $10/mo in compute credits, enough to cover one Micro instance."
> — same page

### R5 — surviving years unattended

**Documented, and this is the finding that most changes what a free-plan answer can say.**

> "Free projects are paused after 1 week of inactivity. Limit of 2 active projects."
> — *Pricing*, address above

> "Supabase pauses Free Plan projects that show low activity over a 7-day period to save server resources."
> "A Free plan project is considered inactive if it does not receive sufficient user database activity over the past week. Projects with too few user queries during that window are the clearest candidates for pausing. While you may be actively using the project, it's possible that usage is not enough to exclude it from automatic pausing. Typically a few user requests to the database each day over the previous week is enough to keep the project from being paused."
> "Projects under a paid plan cannot be paused and are not subject to automatic pausing for inactivity."
> — *Project Pausing*, `https://supabase.com/docs/guides/platform/free-project-pausing`, read 2026-08-16

**And the restore window is bounded, with the vendor's own reason:**

> "Once the project is paused, there is a 1-year window to restore the project on the platform from within Supabase Studio. The time limit exists because backups are only retained for a limited period, and platform changes may not be backward compatible with older backups. Unlike active projects, static backups can't be updated to accommodate such changes."
> — same page

**Verdict on R5: the free plan is documented to fail it, and the paid plan is documented to pass it.** Row 301 promises a licence is the user's forever; a free Supabase project holding the trial-and-slots record is documented to pause after a week of low activity, and to be restorable for one year and no longer. **The paid plan's exemption is stated flatly** — *"Projects under a paid plan cannot be paused"* — and the pricing page's own comparison table says it a third time, in one line: *"Pausing | After 1 week of inactivity | Never | Never | Never"* (Free, Pro, Team, Enterprise in that order). So on this candidate R5 is bought rather than assumed. **Nothing on any page read states what happens to a paused project's data after the one-year window.** Searched the pausing, backups, pricing and going-to-production pages for `after 1 year`, `deleted`, `permanently` and `purge`: not stated.

### R6 — backup and restoration

**Documented, and the free plan is documented to be on its own.**

> "Pro Plan projects can access the last 7 days of daily backups. Team Plan projects can access the last 14 days of daily backups, while Enterprise Plan projects can access up to 30 days of daily backups. If you need more frequent backups, consider enabling Point-in-Time Recovery . **We recommend that free tier plan projects regularly export their data using the Supabase CLI db dump command and maintain off-site backups.**"
> — *Database Backups*, `https://supabase.com/docs/guides/platform/backups`, read 2026-08-16

> "Point-in-Time Recovery (PITR) allows you to back up a project at shorter intervals, giving you the option to restore to any chosen point with up to seconds of granularity."
> "Pro, Team and Enterprise Plan projects can enable PITR as an add-on."
> "Projects that want to use PITR must also use at least a Small compute add-on to ensure smooth functioning."
> — same page

**And it is priced, on both pages, at four times the plan it sits on:**

> "Point in time recovery | Not included in | $100 per month per 7 days retention | $100 per month per 7 days retention | $100 per month per 7 days retention, >28 days retention available"
> — *Pricing*, address above (Free, Pro, Team, Enterprise in that order)

> "Recovery Retention Period in Days | Hourly Price USD | Monthly Price USD — 7 | $ 0.137 | ~$ 100 · 14 | $ 0.274 | ~$ 200 · 28 | $ 0.55 | ~$ 400"
> — *Database Backups*, address above

**Set that beside Cloudflare's Time Travel**, which the same requirement reads as *"always on"*, restoring *"to any minute within the last 30 days"*, with *"no additional costs"* and seven days of it on the free plan. **Seven days of point-in-time recovery is $100 a month here and $0 there**, at a second's granularity against a minute's.

> "The project is inaccessible during this process, so plan for downtime beforehand. Downtime depends on the size of the database—the larger it is, the longer the downtime will be."
> — same page, on restoring

**So on the free plan, R6 is a chore the Creator must remember** — which is exactly the shape [Method](Method.md)'s third escalation forbids putting in front of the *user*, and does not forbid putting in front of the Creator. It is recorded here as a cost to the Creator, not as a promise broken.

### R8 — the seller seam

**Documented in all three parts, with one restriction.**

Secrets: *"Maximum number of secrets per project: 100"*, *"Maximum secret size: 48 KiB ( 24,576 characters)"*, *"Names must NOT start with the prefix SUPABASE_ (this prefix is reserved)"* (*Limits*, address above).

Outbound: **not stated as a feature, and stated only as a restriction** — *"Outgoing connections to ports 25 and 587 are not allowed"* (*Limits*, address above), which implies other ports are and says nothing more. Beside that, the Edge Functions navigation on that same page lists *"Handling Stripe Webhooks"* among its examples; **that is a link title in the documentation's own navigation, not a page read here**, and it is recorded only as evidence that a seller webhook is an intended use of the product.

Runtime, which bounds what a webhook handler may do before answering:

> "Maximum Memory: 256MB"
> "Maximum Duration (Wall clock limit) ... Free plan: 150s · Paid plans: 400s"
> "Maximum CPU Time: 2s (Amount of actual time spent on the CPU per request - does not include async I/O.)"
> "Request idle timeout: 150s (If an Edge Function doesn't send a response before the timeout, 504 Gateway Timeout will be returned)"
> — *Limits*, address above

### R9 — logging

**Documented, and this is the worst of the four against §13 rule 4.**

> "Logs are provided for each function invocation, locally and in hosted environments."
> "**Invocations**: Request/Response data including headers, body, status codes, and execution duration. Filter by date, time, or status code."
> — *Logging*, `https://supabase.com/docs/guides/functions/logging`, read 2026-08-16

**The platform records the request body of every function invocation — the vendor states the *every* itself, three lines above the sentence that says what is kept — and a licence check's body is the licence key and the computer identity.** Retention is plan-dependent:

> "Log retention (API & Database) | 1 day | 7 days | 28 days | 90 days"
> — *Pricing*, address above (Free, Pro, Team, Enterprise in that order)

**Nothing read states that invocation capture can be turned off.** Searched the Edge Functions logging page, the limits page and the log-drains page for `disable`, `opt out`, `turn off` and `redact`: **zero matches**. **Label: not stated, and the silence is the finding** — on this candidate a downstream answer cannot satisfy §13 rule 4 by configuration, and would have to satisfy it by never putting the key or the identity anywhere the platform captures, which the quoted sentence says is the request body itself.

### R10 — migration and retirement

**Migration is the best-stated of the four.** The vendor names portability as the point of a custom domain — *"you can migrate from one Supabase project to another"* — and names the export tool by name for both backup and migration: *"regularly export their data using the Supabase CLI db dump command"*, and, after disabling point-in-time recovery, *"you need to take a manual legacy logical backup using the Supabase CLI or pg_dump"* (*Database Backups*, address above). **A `pg_dump` output is a portable Postgres dump**, which is the widest-supported of the three export formats in this document.

**Retirement is documented too, on that same page, and it is final:**

> "When you delete a project, we permanently remove all associated data, including any backups stored in S3. This action is irreversible, so consider it carefully before proceeding."
> — same page

That is the vendor's only statement anywhere read about permanent loss. It answers deleting a project; it does not answer what becomes of a *paused* project's data at the end of the one-year window, which stays unstated above.

### Price, read 2026-08-16

All from *Pricing*, `https://supabase.com/pricing`, undated:

- Free: *"$ 0 / month"*, *"Unlimited API requests"*, *"500 MB database size"*, *"5 GB egress"*, *"Community support"*, *"Free projects are paused after 1 week of inactivity. Limit of 2 active projects."*
- Pro: *"From $ 25 / month"* — the page's own word, and its worked example totals $25 for one Micro project — *"First project included. Additional projects from $10/mo."*, and *"Paid plans include $10/mo in compute credits, enough to cover one Micro instance."*
- Edge Functions invocations: *"500,000 included"* (Free), *"2 Million included then $2 per 1 Million"* (Pro and Team).
- Custom domain: *"$10 per domain per month per project add on"*.
- Point-in-time recovery: *"$100 per month per 7 days retention"*, listed among the add-ons as *"Starts from $100/month"*.
- **Uptime SLAs: "Not included in" Free, Pro and Team; "Included in" Enterprise.**

**So a Supabase arrangement that satisfies R2 and R5 is documented at $25 a month plus $10 a month**, and point-in-time recovery on top of that is a further $100 a month for seven days of retention, requiring at least a Small compute add-on.

---

## Hetzner Cloud

Read as: a plain rented Linux server on which the Creator runs everything. Every one of the ten requirements above is theirs to build; the vendor documents the machine, the disk, the backup and the certificate, and nothing else.

### R1 and R2 — the endpoint and the address

**Not stated for a plain server, and documented only through a Load Balancer.** No page read documents TLS termination, a certificate or a domain for a cloud server on its own — the certificate documentation attaches to one resource only:

> "Uploaded certificate — Upload an existing certificate that you already created (e.g. with Let's Encrypt). You will need to renew the certificate yourself."
> "Managed certificate — Have Hetzner create a new Let's Encrypt certificate for you. Hetzner will automatically renew the certificate. For this option, Hetzner DNS is required."
> "You can add certificates to the following resources: Load Balancers"
> "We do not charge for managed certificates."
> — *Certificates*, `https://docs.hetzner.com/networking/certificates/overview/`, last change on 2026-02-10, read 2026-08-16

**So R1 and R2 are either the Creator's own work on the machine, or a Load Balancer at EUR 7.49 a month.** The documentation page carries no price — *"For information about available plans and prices, see hetzner.com/cloud/load-balancer"* (*Load Balancers*, `https://docs.hetzner.com/networking/load-balancers/overview/`, last change on 2021-12-23, read 2026-08-16) — and that address returns HTTP 200 with its smallest plan, `LB11`, keyed `CLOUD_25`, which the vendor's live price feed gives as **EUR 7.49 / USD 8.49 a month, EUR 0.0120 an hour** (`https://www.hetzner.com/cloud/load-balancer/`, read 2026-08-16; feed address and stamp under limit four).

**The managed-certificate route carries the same shape of lock-in Cloudflare's custom domain does**, from the other direction: *"For this option, Hetzner DNS is required."*

### R3, R4 and R5 — the store, serialized writes, and surviving

**Not stated, because the store is a file on a disk the Creator owns.** What the vendor documents is the disk under it, and the documentation splits sharply between a server's own disk and an attached volume.

> "Our Volumes are based on the networked block storage model and every block of data is stored on three different physical servers at our Hetzner data centers."
> — *Volumes overview*, `https://docs.hetzner.com/cloud/volumes/overview/`, last change on 2023-08-01, read 2026-08-16

That is the strongest documented durability statement of the four, and it comes with the sharpest documented exclusion:

> "Please note that we do not provide Backups or Snapshots for Volumes. Backups and Snapshots for servers do not include any Volumes which are attached to your server."
> — same page

**So on this candidate the well-replicated place to put the record is the one place the vendor documents no backup for**, and the place the vendor backs up is the server's own disk. That contradiction between two of the vendor's own features is the thing a downstream answer has to arrange around; **it is not a disagreement between two pages**, since both statements are on the same page and are consistent with each other.

**R5 has a documented answer here and it is the plainest of the four:** the machine exists until it is deleted, and is billed until then.

> "Yes, you pay for a server that has completed the creation process for as long as it exists, regardless of whether it is turned on or not. We will bill you for your servers until you delete them, independent of their state."
> — *Billing FAQ*, `https://docs.hetzner.com/cloud/billing/faq/`, last change on 2020-07-07, read 2026-08-16

**Nothing on any page read states that a machine, a disk or an account is reclaimed for inactivity.** Searched the billing FAQ, the servers overview, the volumes overview, the backups overview and the enabling-backups page for `inactive`, `dormant`, `reclaim`, `suspend` and `expire`: nothing.

### R6 — backup and restoration

**Documented, at a coarser granularity than any of the other three.**

> "Backups are copies of a server's disk that are created automatically on a daily basis. Each server has a total of seven Backup slots. You can therefore have up to seven Backups at the same time. Once this limit is reached, the oldest Backup will automatically be deleted as soon as a new Backup is created."
> "A Snapshot is a copy of a server's disk that is created manually. Snapshots can be used to capture the current state of the server's disk, and they are saved until you decide to delete them. Snapshots are never deleted or updated automatically."
> "Note: Backups and Snapshots do not include any Volumes which are attached to your server."
> — *Backups/Snapshots*, `https://docs.hetzner.com/cloud/servers/backups-snapshots/overview/`, last change on 2022-08-23, read 2026-08-16

> "For every server, there are seven slots for Backups. If all slots are full, the oldest Backup will be deleted."
> — *Enabling Backups*, `https://docs.hetzner.com/cloud/servers/getting-started/enabling-backups/`, last change on 2021-12-13, read 2026-08-16

**Daily, seven deep, whole-disk, and only if the record is not on a volume.** Restoring is documented as rebuilding the server or creating a duplicate from the image, with the vendor's own warning: *"You can rebuild the server using a Backup/Snapshot, but you will lose any data that is not saved on the Backup/Snapshot."*

### R8 and R9 — the seller seam, and logging

**Not stated, and correctly so.** Outbound calls, secret storage and logging are all the Creator's own on a machine they run; the vendor documents none of them and is not expected to. **The consequence for R9 is the inverse of Supabase's**: nothing captures the request body unless the Creator's own program does, and nothing is retained unless they retain it — but nothing is provided either, so every rule in [Security and licence](Security%20and%20licence.md) §13 has to be built rather than configured.

**The one documented network fact that bears on R8:** *"We only bill for outgoing traffic. Incoming and internal traffic is free"*, and over-usage is billed *"in blocks of 100MB"* (*Billing FAQ*, address above).

### R10 — migration and retirement

**Documented for retirement and not stated for export, and both are the best case.** Retirement is deleting the server, which the billing FAQ states stops the charge — *"If you delete your cloud server before the end of the billing month, we will only bill you for the hourly rate"*. Export is not the vendor's operation at all: the store is a file the Creator can copy anywhere. **This candidate has the lowest documented migration cost of the four and the highest documented amount of work everywhere else.**

### Availability, and what makes this candidate different on it

**Documented, and it is the only one of the four with a published uptime figure reachable without an enterprise contract:**

> "Our performance promises for professional requirements: An SLA with 99.9% guaranteed uptime and clearly defined credits in the event of a malfunction offers you predictable operational reliability."
> — *Cloud*, `https://www.hetzner.com/cloud/`, undated, read 2026-08-16

**Read narrowly.** The sentence says an SLA exists and names a figure; **the SLA document itself was not read and is not quoted here**, so nothing below rests on what it covers or excludes. Compare Supabase, whose own pricing table lists *"Uptime SLAs"* as *"Not included in"* the Free, Pro and Team plans, and **Cloudflare and Fly.io, whose documentation pages read for this document carry no uptime figure and no SLA statement at all.** Searched all thirty-two Cloudflare and Fly.io addresses listed at the end of this document for `SLA`, `uptime` and `guarantee`: **zero occurrences of `SLA` and zero of `uptime` on either vendor**, and every occurrence of `guarantee` is about something else entirely — D1 location hints (*"Providing a location hint does not guarantee that D1 runs in your preferred location"*), D1 batch ordering (*"Our implementation guarantees that each statement in the list will execute and commit, sequentially, non-concurrently"*), D1 session bookmarks (*"pass a bookmark parameter to guarantee that the session starts with a database version that is at least as up-to-date as the provided bookmark"*), Fly.io volume placement (*"this is not guaranteed when creating Fly Volumes through other methods"*), and Fly.io machine snapshots (*"Snapshots aren't guaranteed to persist"*).

Where the machines are:

> "As one of Europe's leading internet service providers, Hetzner is represented in many parts of the world. Our cloud servers are available to you in Germany, Finland, Singapore, and the USA."
> — *Cloud*, address above

**One documented availability fact, and it is availability rather than price:** the same page carries a static badge *"Currently not available"* under its *"Shared Resources"* heading, and every server row on `https://www.hetzner.com/cloud/cost-optimized/` carries a static *"not available"* label in the served HTML — `product-highlight-label-NotAvailable`, beside `CX23`, `CAX11` and the rest — on the date read. **That is the vendor saying those servers cannot be bought**, and it stands beside a price feed that quotes every one of them.

### Price, read 2026-08-16

**Read from the vendor's own live price feed**, whose address, provenance and stamp are under limit four: `https://www.hetzner.com/_resources/app/data/app/live_data_prices.json`, `last-modified: Sat, 15 Aug 2026 22:17:50 GMT`, retrieved anonymously on 2026-08-16. The keys that index it are on the vendor's own pages — each server row on `https://www.hetzner.com/cloud/cost-optimized/` carries a `product-key` beside the server's name.

Germany (Nuremberg, Falkenstein) and Finland (Helsinki), excluding VAT:

| Server | vCPU | RAM | SSD | EUR/mo | USD/mo | EUR/h |
|---|---|---|---|---|---|---|
| CX23 | 2 Intel/AMD | 4 GB | 40 GB | 5.49 | 6.49 | 0.0088 |
| CX33 | 4 Intel/AMD | 8 GB | 80 GB | 8.49 | 9.99 | 0.0136 |
| CX43 | 8 Intel/AMD | 16 GB | 160 GB | 15.99 | 18.49 | 0.0256 |
| CX53 | 16 Intel/AMD | 32 GB | 320 GB | 29.49 | 34.99 | 0.0473 |
| CAX11 | 2 Ampere | 4 GB | 40 GB | 5.99 | 6.99 | 0.0096 |
| CAX21 | 4 Ampere | 8 GB | 80 GB | 10.49 | 12.49 | 0.0168 |
| CAX31 | 8 Ampere | 16 GB | 160 GB | 20.99 | 24.99 | 0.0336 |
| CAX41 | 16 Ampere | 32 GB | 320 GB | 40.99 | 48.49 | 0.0657 |

Three things about that table a downstream reader has to carry.

- **The figures above are the server alone.** Each row's own price cell asks the feed for two keys added together — CX23's is `CLOUD_132+CLOUD_21` — so what that page would render for CX23 is **EUR 5.99**, the server plus one Primary IPv4.
- **Primary IPv4** (`CLOUD_21`) is **EUR 0.50 / USD 0.60 a month**, EUR 0.0008 an hour, and it is the one figure here the vendor also states in prose, twice: *"We charge an additional cost of € 0.50/month (excl. VAT) for our Primary IPs of type IPv4"* (*Cloud Servers*, `https://docs.hetzner.com/cloud/servers/overview/`, last change on 2022-06-28, read 2026-08-16), and *"We do not charge for our Primary IPv6s. We charge € 0.50 / $ 0.60 per month (excl. VAT) for our Primary IPv4s"*, with a table reading *"IPv4 € 0.0008 / h $ 0.0010 / h € 0.50 / mo $ 0.60 / mo"* and *"IPv6 free free"* (*Primary IPs*, `https://docs.hetzner.com/cloud/servers/primary-ips/overview/`, last change on 2022-06-28, read 2026-08-16). **The feed and the prose agree exactly**, which is the only cross-check available between the two.
- **Floating IPs** (`CLOUD_3`), labelled *"per IPv6 per month"* on the vendor's own add-on card, are **EUR 1.00 / USD 1.50 a month**. **Load Balancer** `LB11` (`CLOUD_25`) is **EUR 7.49 / USD 8.49 a month**, EUR 0.0120 an hour.

**One figure in the feed cannot safely be quoted at all**, and it is the one R6 would want. The Snapshots add-on card is labelled *"per GB per month"* but asks the feed for the hourly field, and the feed gives `CLOUD_24` an hourly price only — EUR 0.0143 / USD 0.0199. **Label: disagreement**, between the vendor's own card and the vendor's own feed. The vendor's prose states the rule without a number (quoted below), so **nothing downstream should cite a snapshot price from this vendor.**

**Backups are priced as a proportion of the server rather than as a figure**, and the vendor states that rule in prose:

> "We will bill you for Backups with a monthly flat price, which is 20% of the price of the server you activate them for. So if you activate Backups for a server that costs € 3.29 ($ 4,49) per month, we will bill you for an additional € 3.29 * 0.2 = € 0.658/month ($ 4,49 * 0,2 = $ 0,898/month). For this price, you will get a total of 7 Backup slots."
> — *Billing FAQ*, address above, last change on 2020-07-07

**The € 3.29 in it is an illustration on a page last changed in 2020, and no server in the live feed costs it** — the rule is what survives, not the number. The vendor's own add-on card states the same rule the same way, as an operation on the live price rather than as a figure: *"Backups — mo. price related to instance, starting at:"*.

> "We will bill you for Snapshots per gigabyte per month. If a Snapshot only exists for a fraction of a month, then we will only bill you for this fraction. We will only bill you for the space the compressed Snapshot uses in our storage system."
> — same page

Also documented on that page, and relevant to any cost estimate: *"We always round up the hourly usage of a server. If you create a server just for a few minutes, we will still bill you for one whole hour"*, and *"Servers have both a monthly price cap and a price per hour. Your server's bill will never exceed its monthly price cap."*

---

## What none of them documents

Each entry is a silence, with how widely it was looked for and what it breaks. Most run across all four vendors; the three that do not say in their own heading how many they cover.

**1. Whether the check is reachable from a company network.** *Breaks:* Row 236 entire, and Fact 14, which the register already records as *"It may not be fully answerable at all"*. Searched every address listed below for `firewall`, `corporate`, `blocked`, `blocking`, `censor`, `proxy` and `restricted network`. **Those terms occur widely, and never in that sense**: `firewall` is Hetzner's own firewall product, `proxy` is Fly.io's own edge proxy, and Fly.io's `blocked` matches are about an ACME challenge blocked by the reader's own configuration. **Not one vendor says anything about a network between the user and the endpoint.** Cloudflare's China Network page is the only page in the whole set that discusses a country, and it discusses Cloudflare's own presence rather than anybody blocking it. **This silence is not a defect in any vendor** — no host can document what somebody else's firewall does — and it is recorded because Row 236 is a row that has to close somehow, and no host reading will close it.

**2. Any uptime or availability commitment reachable on a small plan, at three of the four.** *Breaks:* Row 234's condition that *"the check being reachable means the user never meets the three days"*, and Row 301's forever clause, whose answer is *"what a paid copy does if the service it checks against stops answering for good"*. Cloudflare and Fly.io document no SLA or uptime figure anywhere in the pages read; Supabase's own table lists *"Uptime SLAs"* as *"Not included in"* everything below Enterprise; Hetzner names *"99.9% guaranteed uptime"* on a marketing page whose SLA document was not read. **So on three of four the answer to *what if the host is down* is the three-and-seven-day count and nothing else.**

**3. What a licence key does if the Creator stops paying, or stops.** *Breaks:* Row 301. Every host documents what happens to the *service* — Supabase pauses a free project and keeps it restorable for a year; Fly.io bills a volume until it is deleted; Hetzner bills a server until it is deleted; Cloudflare refuses queries past a free daily limit. **None documents anything about the thing the service was for.** That is not a hosting question at all: it is the same question the Lemon Squeezy reading recorded as its silence 11, and it is `Licence service`'s to answer on the desktop side, because no host can answer it.

**4. Any per-row or per-record size figure, at any of the four.** *Breaks:* nothing outright, but it means **no estimate of how many users fit inside a free plan's storage is derivable from any vendor's documentation**, and none is made anywhere above. Searched all four for `bytes per row`, `record size` and `row size`. **The only matches are Cloudflare's, and every one of them says the opposite of what an estimate needs**: *"Row size or the number of columns in a row does not impact how rows are counted. A row that is 1 KB and a row that is 100 KB both count as one row"*, and *"An empty table consumes at least a few kilobytes, based on the number of columns (table width) in the table. An empty database consumes approximately 12 KB of storage"* (*D1 Pricing*, address above, in the page's own order) — a floor for an empty table and nothing for a full one. Supabase states *"500 MB database size"*; Fly.io and Hetzner state a price per gigabyte and say nothing about what goes in it.

**5. What the platform's own edge keeps, at two of the four.** *Breaks:* [Security and licence](Security%20and%20licence.md) §13 rule 4's completeness. Cloudflare documents its invocation log and documents how to disable it; **Supabase documents capturing request bodies and documents no way to stop it**; Fly.io says nothing about what its proxy records **but documents removing the TLS handlers so that the proxy never holds the plaintext**, which makes its entry configurable rather than unknown; Hetzner terminates nothing unless a Load Balancer is used and documents nothing about what one logs. **So one vendor answers it, one answers it badly, one leaves it unstated with a way round it, and one leaves it unstated with nothing.**

**6. How long a stopped or idle instance takes to answer its first request, at two of the four.** *Breaks:* nothing on its own; it bounds R8's webhook window, where Lemon Squeezy's documented retry is *"5 secs, 25 secs, 125 secs"* and then failure. Searched all four for `cold start`, `startup time`, `boot time` and `wake up`. **Corrected 2026-08-21: that search did not work, over a file this document itself lists as retrieved.** `https://developers.cloudflare.com/workers/llms-full.txt` — the whole Workers documentation set in one file, among the thirty-two addresses — was fetched again on 2026-08-20 (HTTP 200, 4,491,987 bytes) and `cold start` occurs in it **six** times. **Cloudflare documents the property plainly**, on *How Workers works*, `https://developers.cloudflare.com/workers/reference/how-workers-works/`, last updated Apr 23, 2026, read 2026-08-20: *"Isolates are also designed to start very quickly. Instead of creating a virtual machine for each function, an isolate is created within an existing environment. **This model eliminates the cold starts of the virtual machine model.**"* and *"Any given isolate can start around a hundred times faster than a Node process on a container or virtual machine."* That page has existed since at least 2024-08-13, so it was readable when this was written. **This is the clearest instance in these documents of a silence that was never a silence**, and the shape is worth keeping: a whole-documentation file was retrieved and then searched for the wrong words. **What remains true is Supabase's half** — what exists there is a ceiling on the program, not a figure for the wait. **And one thing not to carry**: a claim that Cloudflare begins startup during the TLS handshake is in none of its documentation — `tls handshake` returns four matches in that file, all about mTLS, cipher suites and client-hello fields, and `no cold start`, `without cold start` and `zero millisecond` return none. Cloudflare documents a *"Worker startup time"* limit — *"A Worker must parse and execute its global scope (top-level code outside of handlers) within 1 second"* (*Limits*, `https://developers.cloudflare.com/workers/platform/limits/`, read 2026-08-16) — which is a deployment-time validation of the Creator's own code, not a statement of how long a caller waits. Supabase documents a *"Request idle timeout: 150s"*, which is a ceiling on the handler. Hetzner's machine is always running and always billed, which is one place this silence does not bite — and the vendor says so: *"we allocate full resources to servers, regardless of their power state. This enables rapid startup and boot times for you."*

**Fly.io is the other place it does not bite, and it is the only vendor here that answers the question with numbers:** *"Resume from suspend: a few hundred ms"* and *"Cold start: ~2+ seconds for common apps"* (*Machine suspend and resume*, address above). **Corrected 2026-08-21: this said both are inside Lemon Squeezy's first retry at five seconds and that R8's window was therefore bounded.** **Corrected 2026-08-21: the seller's five seconds is the gap before its second delivery attempt, not a deadline the receiver must answer inside** — the vendor's sentence governs what happens *"If the response status is anything else"*, so a status has already been produced. **The vendor publishes no response deadline**: three of its webhook pages were searched on 2026-08-20 for `timeout`, `respond within` and five neighbouring terms, with zero matches and a working control. `Licence service` §15 item 4 carries the reopened question. **So this entry's window is not bounded on that candidate after all** — the two published figures stand as figures, and what they were being measured against does not exist.

**7. Whether an account, as opposed to a resource, is closed for inactivity.** *Breaks:* R5, and through it Row 301. Supabase is the only one of the four that documents anything at all about inactivity, and it documents it for a *project* on a *free* plan — twice, the second time in one line: *"Upgrade to Pro to guarantee that we won't pause your project for inactivity"* (*Production Checklist*, `https://supabase.com/docs/guides/deployment/going-into-prod`, read 2026-08-16). **Cloudflare, Fly.io and Hetzner document nothing about an idle account in any page read** — searched each for `inactive`, `inactivity`, `dormant`, `reclaim`, `suspend`, `unused` and `close your account`; the only matches at those three are Cloudflare's instruction to *"delete unused databases"* when a storage limit is reached, its Durable Object hibernation billing, a note about unused certificates, and Fly.io's advice about an app shutting *itself* down — **not one of them is a vendor reclaiming anything.** Fly.io and Hetzner both document that a resource is billed until deleted, which answers the question sideways and only while a card keeps working; **none of the three states what happens when a card stops working**, which is the ordinary way a Creator-run service dies quietly.

**8. Whether any of them is currently current.** *Breaks:* the confidence in every claim above. Cloudflare and Hetzner date their pages, and Hetzner's run from 2020 to 2026. Supabase's timestamps date its build, not its pages. Fly.io dates a minority of its pages and none of the ones cited here. **So the reading is worth exactly its own date on all four** — with one inversion worth naming: **the single freshest source anywhere here is Hetzner's live price feed**, stamped the day before the reading, at the vendor whose prose pages are the oldest in the set.

---

## Bearing on the requirements without answering them

Recorded because they will matter to `Licence service` and are not answers to any requirement.

**Cloudflare's D1 free tier refuses rather than degrades.** *"When your account hits the daily read and/or write limits, you will not be able to run queries against D1. D1 API will return errors to your client"* — a refusal whose shape Row 227 has to place among its three outcomes, and which is caused by the Creator's own usage rather than by anything the user did.

**A Cloudflare point-in-time restore is destructive and cancels in-flight queries.** *"Restoring a database to a specific point-in-time is a destructive operation, and overwrites the database in place"*, and *"Queries in flight will be cancelled, and an error returned to the client."* Any restore is therefore a window in which Row 227's *could not be reached* is the honest answer.

**A Supabase restore takes the project offline for a length that grows with the database.** *"The project is inaccessible during this process, so plan for downtime beforehand. Downtime depends on the size of the database—the larger it is, the longer the downtime will be."*

**A Fly.io Machine's clock can be wrong for the first seconds after it wakes.** *"On resume, the clock can lag a few seconds until NTP syncs."* Every successful check carries the service's own statement of the time ([Security and licence](Security%20and%20licence.md) §12.3 rule 1), and on an autostop or suspend arrangement the wake and the check are the same moment. Whatever Fly arrangement is chosen has to hold that; the vendor states it and does not solve it.

**Fly.io's own advice is two Machines and two volumes, and the licence record is a single authoritative thing.** Those pull against each other: *"Volumes don't have built-in replication between them, so your app or database needs to take care of replicating data between volumes"* against Row 224's requirement that a computer already recorded must not be recorded twice. Whatever Fly arrangement is chosen has to reconcile them, and the vendor states plainly that the reconciling is the Creator's.

**Every one of the four documents its secret store as write-only from the operator's side, and every one that says anything about it says the running code can leak it.** Cloudflare: *"secret values are not visible within Wrangler or Cloudflare dashboard after you define them."* Fly.io: *"The API servers can only encrypt; they cannot decrypt secret values. Secret values are never logged"*, and then *"People with deploy access can deploy code that reads secret values and prints them to logs."* **The seller credential Row 237's revocation route needs is therefore as safe as the service's own code and no safer**, on every candidate.

**Three of the four give the endpoint a vendor-owned default name.** Cloudflare's `workers.dev` route and Fly.io's *"fly.dev subdomain, based on the app's name"* are both quoted from those vendors' pages; Supabase's project URL of the form `abcdefghijklmnopqrst.supabase.co` is **not quoted from any page read here** and is the one shape in this document with no citation behind it. **R2 exists precisely so that none of those is what Smoozy ships.**

**Cloudflare and Hetzner both make their own DNS a condition of the easy certificate path** — *"an active Cloudflare zone"* and *"For this option, Hetzner DNS is required"* — while Fly.io and Supabase do not. That is a migration cost that shows up years later rather than at set-up.

---

## Every address read, 2026-08-16

**Fifty-four addresses. All returned HTTP 200 serving the page or document named.** Retrieved by direct HTTPS. No page here was reached through a search engine, a cache, or any third party. **Ten of them are marked †**: those were read when this document was corrected against its citation check, on the same date, and every quotation drawn from them was confirmed on the vendor's own page before it was written in.

**Cloudflare (18)** — `https://developers.cloudflare.com/d1/` · `/d1/platform/pricing/` · `/d1/platform/limits/` · `/d1/reference/time-travel/` · `/d1/best-practices/import-export-data/` · `/d1/best-practices/read-replication/` † · `/d1/configuration/data-location/` · `/d1/worker-api/d1-database/` · `/workers/platform/pricing/` · `/workers/platform/limits/` · `/workers/platform/known-issues/` · `/workers/configuration/routing/custom-domains/` · `/workers/configuration/secrets/` · `/workers/configuration/cron-triggers/` · `/workers/observability/logs/workers-logs/` · `/durable-objects/` · `/durable-objects/platform/pricing/` † · `https://developers.cloudflare.com/china-network/`
*(One address returned HTTP 404 and is recorded so nobody re-tries it: `https://developers.cloudflare.com/workers/platform/free/`. It was a guessed address; the vendor's own link behind *"Read more about the Free plan limits"* on the Workers pricing page points at `/workers/platform/limits/#account-plan-limits`, which is already in this list. Nothing above rests on the 404.)*

**Fly.io (14)** — `https://fly.io/docs/volumes/overview/` · `/docs/volumes/snapshots/` · `/docs/about/pricing/` · `/docs/about/billing/` · `/docs/about/support/` · `/docs/about/free-trial/` † · `/docs/apps/secrets/` · `/docs/networking/custom-domain/` · `/docs/security/tls-termination/` † · `/docs/monitoring/logging-overview/` · `/docs/monitoring/error-codes/` † · `/docs/reference/suspend-resume/` † · `/docs/launch/autostop-autostart/` · `/docs/launch/deploy/`

**Supabase (9)** — `https://supabase.com/pricing` · `https://supabase.com/docs/guides/platform/backups` · `/docs/guides/platform/free-project-pausing` · `/docs/guides/platform/custom-domains` · `/docs/guides/functions/limits` · `/docs/guides/functions/secrets` · `/docs/guides/functions/logging` · `/docs/guides/monitoring-and-debugging/log-drains` · `/docs/guides/deployment/going-into-prod`
*(Two addresses returned HTTP 404 before the correct one was found and are recorded so the search width is visible: `/docs/guides/platform/project-pausing` and `/docs/guides/platform/pause-and-restore`.)*

**Hetzner (13)** — `https://docs.hetzner.com/cloud/servers/overview/` · `/cloud/servers/getting-started/enabling-backups/` · `/cloud/servers/backups-snapshots/overview/` · `/cloud/servers/primary-ips/overview/` † · `/cloud/volumes/overview/` · `/cloud/billing/faq/` · `https://docs.hetzner.com/networking/load-balancers/overview/` · `https://docs.hetzner.com/networking/certificates/overview/` · `https://www.hetzner.com/cloud/` · `https://www.hetzner.com/cloud/cost-optimized/` · `https://www.hetzner.com/cloud/load-balancer/` †
**Two of the thirteen are not documentation pages** and are marked as such wherever they are used: the live price feed `https://www.hetzner.com/_resources/app/data/app/live_data_prices.json` † and the script naming it, `https://www.hetzner.com/_resources/themes/static/dist/vendors.js` †.
*(One address returned HTTP 404 and is recorded so nobody re-tries it: `https://docs.hetzner.com/cloud/certificates/overview/`; the certificates page lives under `networking/`, not `cloud/`. Two more, tried while looking for the prices described in limit four, returned the site's own 404 page and still do: `/_resources/app/data/prices.json` and `/_resources/app/data/price/prices.json` — the real address is one folder deeper.)*

---

**Still not settled here: which host.** That is `Licence service`'s, against the above.

---

# Later addition, read 2026-08-17 — what Cloudflare documents about signing, digesting, random values, constant-time comparison, and where a key lives

**This section was added after everything above, and nothing above was changed to make room for it.** It is **not** covered by the citation check of 2026-08-16: that check ruled on the claims that existed then, and every claim below was read a day later, on **2026-08-17**, at addresses the check never saw. Nothing above is restated, revised, corrected or withdrawn here.

**Why it exists.** The before-audit gate on `Licence service` found that the reading above — fifty-four addresses, four vendors — states nowhere that any of these platforms offers cryptographic primitives, while that document's chosen shape rests on three: **a signature over the check response that the desktop application verifies** (its §5, and three of its §13 defences), **a salted digest of the licence key** so that the service stores a digest and never a key (its §3), and **a per-request value the caller supplies and the response carries back**. `Licence service` recorded the hole as unresolved. This section closes it **for the one host that document chose — Cloudflare Workers with D1 — and for no other.** Fly.io, Supabase and Hetzner were not read for this, and nothing below says anything about them.

**Five questions were asked and all five have documented answers.** They are: producing and verifying a signature inside a Worker and which algorithms are named; producing a digest and which are named; generating random values; constant-time comparison; and where a signing key itself would live.

**The five limits at the top of this document hold here without exception.** Nothing was run. No account exists, nothing was signed up for, no email address was supplied, no card was entered, no authenticated request was made, nothing was deployed. Every retrieval was anonymous direct HTTPS and every one returned HTTP 200. **Every claim below is a documented assertion, never an observation.** Two of the addresses are not documentation pages but the vendor's own machine-readable documentation index documents, and are marked as such where they are used — the same treatment limit four gives Hetzner's price feed.

*One note on the date, for exactness.* These retrievals carry HTTP `date` headers late on **2026-08-16 UTC**, which is **2026-08-17** on the Creator's clock. The date used throughout this section is the local one, matching the brief this reading was done under.

---

## Q1 — producing and verifying a signature inside a Worker

**Documented, on a page of the runtime's own reference, and the algorithms are named in a table.**

> "The Web Crypto API provides a set of low-level functions for common cryptographic tasks. The Workers runtime implements the full surface of this API, but with some differences in the supported algorithms compared to those implemented in most browsers."
> "The Web Crypto API is implemented through the SubtleCrypto interface, accessible via the global crypto.subtle binding."
> — *Web Crypto*, `https://developers.cloudflare.com/workers/runtime-apis/web-crypto/`, last updated Apr 23, 2026, read 2026-08-17

The two operations themselves:

> "sign(algorithm, key, data) : Promise<ArrayBuffer>"
> "Returns a Promise that fulfills with the signature corresponding to the text, algorithm, and key given as parameters."
> "verify(algorithm, key, signature, data) : Promise<boolean>"
> "Returns a Promise that fulfills with a Boolean value indicating if the signature given as a parameter matches the text, algorithm, and key that are also given as parameters."
> — same page, each heading followed by its own description

**And the algorithms are not left to inference.** The page carries a table it introduces this way:

> "Workers implements all operations of the WebCrypto standard , as shown in the following table."
> "A checkmark (✓) indicates that this feature is believed to be fully supported according to the spec."
> — same page

Its first column is `Algorithm` and its second is `sign()` over `verify()`. **Six rows carry a checkmark in that column**, and these are the row labels exactly as the vendor's own table spells them — with spaces, not the hyphens the same page's prose uses:

> `RSASSA PKCS1 v1.5` · `RSA PSS` · `ECDSA` · `Ed25519` · `NODE ED25519` · `HMAC`
> — same page, the `sign()`/`verify()` column

`Ed25519` carries a footnote reading *"Algorithms as specified in the Secure Curves API"*; `NODE ED25519` carries a second one, quoted under *documented limits* below. Every one of the six also carries a checkmark under `generateKey()`, `exportKey()` and `importKey()`.

**The vendor also publishes a worked example of exactly this shape of thing**, whose own summary line is:

> "Verify a signed request using the HMAC and SHA-256 algorithms or return a 403."
> "You can both verify and generate signed requests from within a Worker using the Web Crypto APIs ."
> — *Sign requests*, `https://developers.cloudflare.com/workers/examples/signing-requests/`, last updated Apr 23, 2026, read 2026-08-17; the first is the page's own subtitle, directly under its title

Its code imports a Worker secret as a key and signs with it — `crypto.subtle.importKey("raw", secretKeyData, { name: "HMAC", hash: "SHA-256" }, false, ["sign", "verify"])`, then `crypto.subtle.sign("HMAC", key, encoder.encode(dataToAuthenticate))` — and its own comment on the verifying half is worth carrying:

> "Use crypto.subtle.verify() to guard against timing attacks. Since HMACs use symmetric keys, you could implement this by calling crypto.subtle.sign() and then doing a string comparison -- this is insecure, as string comparisons bail out on the first mismatch, which leaks information to potential attackers."
> — same page, in the sample's own comments

**A second, independent route is documented**, and it is not the Web Crypto API:

> "The node:crypto ↗ module provides cryptographic functionality that includes a set of wrappers for OpenSSL's hash, HMAC, cipher, decipher, sign, and verify functions."
> "All node:crypto APIs are fully supported in Workers with the following exceptions:"
> "For compatibility dates of 2026-08-04 or later, Workers enables both nodejs_compat and nodejs_compat_v2 by default."
> "The WebCrypto API is also available within Cloudflare Workers. This does not require the nodejs_compat compatibility flag."
> — *crypto*, `https://developers.cloudflare.com/workers/runtime-apis/nodejs/crypto/`, last updated Aug 12, 2026, read 2026-08-17

**One half of `Licence service`'s §5 is not this vendor's to document and is not answered here.** Cloudflare documents producing a signature inside a Worker. **Whether the desktop application can verify one is a question about the desktop side, and no Cloudflare page bears on it at all.** Which of the six named algorithms is chosen — and in particular whether it is a symmetric one, where the verifier must hold the same key that signs, or an asymmetric one, where it need not — is `Licence service`'s and is not ruled on here.

---

## Q2 — producing a digest

**Documented, and five algorithms are named.**

> "digest(algorithm, data) : Promise<ArrayBuffer>"
> "Returns a Promise that fulfills with a digest generated from the algorithm and text given as parameters."
> — *Web Crypto*, address above, the heading and its own description

The same table's `digest()` column carries a checkmark on **five** rows, again as that table spells them:

> `SHA 1` · `SHA 256` · `SHA 384` · `SHA 512` · `MD5`
> — same page, the `digest()` column

The page's own opening example uses `SHA-256` in prose — *"A simple example of calculating a digest (also known as a hash) is:"*, followed by `crypto.subtle.digest({ name: 'SHA-256' }, myText)`. A streaming variant is documented beside it:

> "crypto.DigestStream(algorithm) DigestStream"
> "A non-standard extension to the crypto API that supports generating a hash digest from streaming data. The DigestStream itself is a WritableStream that does not retain the data written into it. Instead, it generates a hash digest automatically when the flow of data has ended."
> — same page, the heading and its own description

**On the *salted* half specifically, which is what §3 actually needs.** Two key-derivation algorithms are named, both carrying a checkmark in the table's `deriveBits()`/`deriveKey()` column and in `importKey()`:

> `HKDF` · `PBKDF2`
> — same page, the `deriveBits()`/`deriveKey()` column

**No purpose-built password-hashing function is documented anywhere read, and one is documented as absent by name** — see *documented limits* below, where `argon2` appears in the vendor's own exclusion list for `node:crypto`. **So a salted digest on this platform is documented as buildable from `digest()`, `HMAC`, `HKDF` or `PBKDF2`, and from nothing stronger.**

**One thing the store side does not offer, searched for and absent.** No page in the vendor's own complete D1 documentation text names a SQL-side hash function: searched that document (`https://developers.cloudflare.com/d1/llms-full.txt`, 563,535 bytes, retrieved 2026-08-17, an index document rather than a page) for `sha256`, `randomblob` and `hex(` — **zero occurrences of each**. **Label: not stated.** The digest is the Worker's to compute, not the database's, and the record D1 holds is whatever the Worker writes into it. D1 does document a ceiling wide enough to hold one many times over — *"Maximum string, BLOB or table row size | 2,000,000 bytes (2 MB)"* (*Limits*, `https://developers.cloudflare.com/d1/platform/limits/`, last updated Apr 21, 2026, an address this document already lists, re-read 2026-08-17).

---

## Q3 — generating random values

**Documented, two calls, both named as cryptographic.**

> "Fills the passed ArrayBufferView with cryptographically sound random values and returns the buffer."
> "Must be an Int8Array | Uint8Array | Uint8ClampedArray | Int16Array | Uint16Array | Int32Array | Uint32Array | BigInt64Array | BigUint64Array."
> "Generates a new random (version 4) UUID as defined in RFC 4122 ↗."
> — *Web Crypto*, address above, the descriptions under the headings `crypto.getRandomValues(...) : ArrayBufferView` and `crypto.randomUUID() : string`

The vendor states the same thing again, as advice, on its own best-practices page, together with what not to use:

> "The Workers runtime provides the Web Crypto API for cryptographic operations. Use crypto.randomUUID() for unique identifiers and crypto.getRandomValues() for random bytes. Never use Math.random() for anything security-sensitive. It is not cryptographically secure."
> "Node.js node:crypto is also fully supported when nodejs_compat is enabled, so you can use whichever API you or your libraries prefer."
> — *Workers Best Practices*, `https://developers.cloudflare.com/workers/best-practices/workers-best-practices/`, last updated Jun 3, 2026, read 2026-08-17

**No size ceiling on `getRandomValues` is stated on any page read.** Searched the Web Crypto page, the Workers limits page and the vendor's complete Workers documentation text for `65536`, `quota` beside `random`, and any byte figure attached to `getRandomValues`: **nothing**. **Label: not stated** — a silence, not a guarantee.

**One clarification this reading owes `Licence service`.** The *per-request value the caller supplies and the response carries back* is generated on the **desktop side**, not by the service; no Cloudflare page bears on where that value comes from. What Q3 answers on this host is the other use — **generating a salt**, which §3 needs and which the two calls above are documented to cover.

---

## Q4 — constant-time comparison

**Documented, named as a first-class function, and given a page of its own.** This was the question most likely to come back a silence, and it does not.

> "timingSafeEqual(a, b) : bool"
> "Compare two buffers in a way that is resistant to timing attacks. This is a non-standard extension to the Web Crypto API."
> — *Web Crypto*, address above, the heading and its own description; the two parameters are each given as `ArrayBuffer | TypedArray`

The dedicated page states the guarantee and, in the same breath, its edges:

> "The crypto.subtle.timingSafeEqual function compares two values using a constant-time algorithm. The time taken is independent of the contents of the values."
> "When strings are compared using the equality operator ( == or === ), the comparison will end at the first mismatched character. By using timingSafeEqual , an attacker would not be able to use timing to find where at which point in the two strings there is a difference."
> "The timingSafeEqual function takes two ArrayBuffer or TypedArray values to compare. These buffers must be of equal length, otherwise an exception is thrown."
> "Note that this function is not constant time with respect to the length of the parameters and also does not guarantee constant time for the surrounding code."
> "Handling of secrets should be taken with care to not introduce timing side channels."
> "Do not return early when the input and secret have different lengths. An early return leaks the length of the secret through response timing. Instead, always perform a constant-time comparison as shown in the examples below — when lengths differ, compare the user input against itself and negate the result so the check still fails but takes the same amount of time."
> "In order to compare two strings, you must use the TextEncoder API."
> — *Using timingSafeEqual*, `https://developers.cloudflare.com/workers/examples/protect-against-timing-attacks/`, last updated Apr 23, 2026, read 2026-08-17

and the best-practices page names the exact class of value this service compares:

> "When comparing secret values (API keys, tokens, HMAC signatures), use crypto.subtle.timingSafeEqual() to prevent timing side-channel attacks. Do not short-circuit on length mismatch. Encode both values to a fixed-size hash first."
> — *Workers Best Practices*, address above

**Two further documented behaviours of this runtime turned up while searching for the above, and they are recorded because they are load-bearing rather than because they were asked for.**

> "To this end, Workers is designed to make it impossible for code to measure its own execution time locally. For example, the value returned by Date.now() is locked in place while code is executing. No other timers are provided. Moreover, Cloudflare provides no access to concurrency (for example, multi-threading), as it could allow attackers to construct ad hoc timers."
> — *Security model*, `https://developers.cloudflare.com/workers/reference/security-model/`, last updated Aug 11, 2026, read 2026-08-17

> "When Workers are deployed to Cloudflare, as a security measure to mitigate against Spectre attacks, APIs that return timers, including performance.now() ↗ and Date.now() ↗, only advance or increment after I/O occurs."
> "In the Workers runtime, the timeOrigin property returns 0."
> "In local development, however, timers will increment regardless of whether I/O happens or not."
> — *Performance and timers*, `https://developers.cloudflare.com/workers/runtime-apis/performance/`, last updated Apr 23, 2026, read 2026-08-17

**Read those narrowly.** Both describe code **inside** a Worker being unable to time **itself**. **Neither says anything about an attacker timing the HTTP response from outside**, which is the threat `timingSafeEqual` exists for, and no page read says anything about that. So they neither strengthen nor weaken the answer above; they are recorded because the second one bears on something else entirely.

**That second quotation bears on R7 without revising it, and this section does not revise it.** R7 above reads *not stated*, on the ground that a Worker is a program the Creator runs and any wall clock it reads will do. What is now documented is a **property of the clock it reads**: `Date.now()` only advances after I/O, and returns a value locked in place while code executes. It is not a bar — a check that reads D1 has done I/O — but it is a documented behaviour of the value §12.3 rule 1 makes every count depend on, and a downstream answer has to hold it. **This is the same shape as the Fly.io clock finding already recorded above, at a different vendor, and reached by a different route.**

---

## Q5 — where a signing key itself would live

**Asked plainly, the answer is plainly no. The platform documents holding a key *as a string*, not *as a key*** — with a single exception that cannot sign anything.

**The secret binding, which this document already covers, is documented as text, and the vendor says so twice more on the same page:**

> "To your Worker, there is no difference between an environment variable and a secret. The secret's value is passed through as defined."
> "Secrets can be accessed from Workers as you would any other environment variables."
> — *Secrets*, `https://developers.cloudflare.com/workers/configuration/secrets/`, last updated Jul 3, 2026, an address this document already lists, re-read 2026-08-17

**The vendor's own documented way of putting a private key into a Worker is to pipe PEM text into that same string store:**

> "The put command can also receive piped input. For example:"
> `echo "-----BEGIN PRIVATE KEY-----\nM...==\n-----END PRIVATE KEY-----\n" | wrangler secret put PRIVATE_KEY`
> — *Workers commands*, `https://developers.cloudflare.com/workers/wrangler/commands/workers/`, last updated Aug 13, 2026, read 2026-08-17

**And the signing example says the same thing in its own words**, in the sample whose whole subject is holding a signing key:

> "You will need some secret data to use as a symmetric key. This should be attached to your Worker as an encrypted secret."
> "Import your secret as a CryptoKey for both 'sign' and 'verify' operations"
> — *Sign requests*, address above, in the sample's own comments

**So a key becomes a key only inside one request**, by `importKey`, from a string the environment hands over. The runtime does document the flag that keeps it from being read back out once imported — *"importKey(format, keyData, algorithm, extractable, keyUsages) : Promise<CryptoKey>"*, whose fourth parameter the page gives as a boolean (*Web Crypto*, address above), and the vendor's own signing sample passes `false` — but **that governs the imported `CryptoKey`, not the material it was imported from**, which is a string in an environment variable for as long as the Worker exists.

**The account-level alternative is the same shape, and adds a beta and an exclusion:**

> "Cloudflare Secrets Store is a secure, centralized location in which account-level secrets are stored and managed. The secrets are securely encrypted and stored across all Cloudflare data centers ↗."
> "Available in open beta"
> "Secrets Store is unavailable in the Cloudflare China Network, operated by Cloudflare's partner JD Cloud."
> — *Cloudflare Secrets Store*, `https://developers.cloudflare.com/secrets-store/`, last updated Aug 14, 2026, read 2026-08-17

> "Once the secret is saved, the secret value will no longer be available for viewing."
> "Bindings are located on the env object. To access the secret you first need an asynchronous call."
> — *Workers integration*, `https://developers.cloudflare.com/secrets-store/integrations/workers/`, last updated May 5, 2026, read 2026-08-17

with the sample call `const APIkey = await env.<BINDING_VARIABLE>.get()` — **a value fetched, not a key operated.**

**The one place a key is genuinely held as a key, and why it does not help.**

> "First, upload a certificate and its private key to your account using the wrangler mtls-certificate command:"
> "Adding an mTLS certificate binding includes a variable in the Worker's environment on which the fetch() method is available. This fetch() method uses the standard Fetch API and has the exact same signature as the global fetch, but always presents the client certificate when establishing the TLS connection."
> "Currently, mTLS for Workers cannot be used for requests made to a service that is a proxied zone on Cloudflare. If your Worker presents a client certificate to a service proxied by Cloudflare, Cloudflare will return a 520 error."
> — *mTLS*, `https://developers.cloudflare.com/workers/runtime-apis/bindings/mtls/`, last updated Apr 23, 2026, read 2026-08-17

**That is a private key uploaded to the account and used by the platform on the Worker's behalf.** No page read documents signing arbitrary data with it, and the only operation documented on the binding is `fetch()`. **So it is a key held as a key, usable only for presenting a client certificate on an outbound connection — which is R8's seller call, never §5's signature.**

**Nothing else exists, and this was searched for widely.** Searched the vendor's own complete Workers documentation text (`https://developers.cloudflare.com/workers/llms-full.txt`, 4,489,089 bytes, retrieved 2026-08-17, an index document rather than a page) for `HSM`, `key management`, `Keyless` and `non-extractable`: **`key management`, `Keyless` and `non-extractable` return zero occurrences each, and every `HSM` match is the letters inside the word `lengthsMatch` in the timing-attack sample.** **Label: not stated.** **No key-management product, no hardware-backed key, and no *platform-held* key object is documented anywhere in the Workers documentation set.** **Narrowed 2026-08-22.** The last clause read *no key object that outlives one request*, which is too wide and is contradicted on a page inside this reading's own address list: *"Do not store request-scoped state in global scope — Workers reuse isolates across requests. A variable set during one request is still present during the next."* (*Workers Best Practices*, `https://developers.cloudflare.com/workers/best-practices/workers-best-practices/`, read 2026-08-20). So a `CryptoKey` in a module-level variable **does** outlive a request — on an isolate the platform may discard at any time and does not undertake to keep. **What the search actually establishes is unchanged**: nothing the platform holds on the service's behalf. `Licence service` §2 carries this quotation with the same narrowing.

---

## Documented limits on all of the above

Eight, in the order they would bite.

**One — the free plan's CPU ceiling is ten milliseconds, and the vendor names this exact class of work as the expensive one.**

> "CPU time | 10 ms | 5 min" (Workers Free, Workers Paid)
> "CPU time per HTTP request | 10 ms | 5 min (default: 30 seconds)"
> "Most Workers consume very little CPU time. The average Worker uses approximately 2.2 ms per request. Heavier workloads that handle authentication, server-side rendering, or parse large payloads typically use 10-20 ms."
> "Each isolate has some built-in flexibility to allow for cases where your Worker infrequently runs over the configured limit. If your Worker starts hitting the limit consistently, its execution will be terminated according to the limit configured."
> "When a Worker exceeds its CPU time limit, Cloudflare returns Error 1102 to the client with the message Worker exceeded resource limits."
> — *Limits*, `https://developers.cloudflare.com/workers/platform/limits/`, last updated Jul 28, 2026, an address this document already lists, re-read 2026-08-17

**And no page read gives a CPU cost for any Web Crypto operation.** Searched the Web Crypto page, the limits page and the complete Workers documentation text for a millisecond or microsecond figure attached to `sign`, `digest`, `PBKDF2` or `timingSafeEqual`: **nothing**. **So whether a signature plus a salted digest fits inside ten milliseconds is not derivable from this vendor's documentation, and no estimate is made here.** Error 1102 is a **refusal shape** caused by the Creator's own choices rather than by anything the user did — the same shape D1's daily free ceiling produces, which this document already records against Row 227.

**Two — a secret is size-capped, by a limit stated of environment variables.** *"Environment variable size | 5 KB"*, and *"Environment variables | 64/Worker (Free) | 128/Worker (Paid)"* (*Limits*, address above). **That cap is stated of environment variables, not of secrets**; joining it to secrets rests on the vendor's own *"Secrets are environment variables"* (*Secrets*, address above), and the join is this reading's, not a sentence the vendor wrote. A PEM signing key is far inside 5 KB either way. Bulk upload is separately capped: *"You can upload up to 100 secrets per bulk request for a single version"* (*Secrets*, address above).

**Three — RSA key parameters are restricted, by a flag that has been the default since 2023-08-01.**

> "Strict crypto error checking · Default as of 2023-08-01 · Flag to enable strict_crypto_checks · Flag to disable no_strict_crypto_checks"
> "Perform additional error checking in the Web Crypto API to conform with the specification and reject possibly unsafe key parameters:"
> "For RSA key generation, key sizes are required to be multiples of 128 bits as boringssl may otherwise truncate the key."
> "The size of imported RSA keys must be at least 256 bits and at most 16384 bits, as with newly generated keys."
> "The public exponent for imported RSA keys is restricted to the commonly used values [3, 17, 37, 65537]."
> — *Compatibility flags*, `https://developers.cloudflare.com/workers/configuration/compatibility-flags/`, last updated Aug 12, 2026, read 2026-08-17

**Four — one of the five named digest algorithms is documented as unfit for this use, by the vendor.**

> "MD5 is not part of the WebCrypto standard but is supported in Cloudflare Workers for interacting with legacy systems that require MD5. MD5 is considered a weak algorithm. Do not rely upon MD5 for security."
> — *Web Crypto*, address above, footnote 3

**Five — one of the six named signing algorithms is documented as possibly changing under the service's feet.**

> "Legacy non-standard EdDSA is supported for the Ed25519 curve in addition to the Secure Curves version. Since this algorithm is non-standard, note the following while using it:"
> "Use NODE-ED25519 as the algorithm and namedCurve parameters."
> "Unlike NodeJS, Cloudflare will not support raw import of private keys."
> "The algorithm implementation may change over time. While Cloudflare cannot guarantee it at this time, Cloudflare will strive to maintain backward compatibility and compatibility with NodeJS's behavior."
> — *Web Crypto*, address above, footnote 2

**That is the nearest thing to a documented deprecation anywhere in this reading.** The plain `Ed25519` row carries no such footnote; the two are separate rows in the vendor's own table.

**Six — the second route's exclusions are named.**

> "The functions generateKeyPair ↗ and generateKeyPairSync ↗ do not support DSA or DH key pairs."
> "argon2 and argon2Sync are not supported."
> "ed448 and x448 curves are not supported."
> "It is not possible to manually enable or disable FIPS mode ↗."
> — *crypto*, address above

**Seven — the constant-time function has two stated non-guarantees and one hard precondition**, all three quoted under Q4: equal lengths or an exception, not constant time with respect to length, and no guarantee for the surrounding code.

**Eight — the account-level secret store is in open beta and is excluded from the China Network**, both quoted under Q5. **The second of those compounds a restriction this document already records under Cloudflare's R1**, from a different page and for a different product.

**One thing that is *not* a limit, and was checked.** No page read attaches Web Crypto, `node:crypto`, `timingSafeEqual`, `getRandomValues` or the secret binding to any **plan**. Searched the Web Crypto, Node.js crypto, timing-attack, secrets, best-practices and limits pages: **the only plan-dependent figures found anywhere near this subject are request counts and CPU time.** **Label: not stated** — the primitives are documented without a plan attached, which is a silence rather than a promise that the free plan carries them.

---

## What this changes, and what it argues against

**Recorded here because the gate asked for evidence against as well as for. None of it is a ruling, and no host is chosen or unchosen here.**

**The gap is closed for this host, and the answers are stronger than the question assumed.** All three primitives `Licence service` rests on are documented by the vendor itself, on reference pages, with named algorithms, and two of the three have worked examples the vendor publishes for exactly this purpose. Constant-time comparison — the one most platforms leave to a library — is a named function with a page of its own.

**Three things argue against, and they are stated plainly.**

**The ten-millisecond free-plan CPU ceiling is now the sharpest unquantified risk on this candidate.** The vendor names *"workloads that handle authentication"* as the ones that run to *"10-20 ms"*, and the licence check is precisely that; a salted digest and a signature both land inside that budget; and the vendor publishes **no cost figure for any cryptographic operation**. So the free plan cannot be shown to fit, and cannot be shown not to. Every earlier free-plan reading in this document was about **storage and request counts**; this is a different axis and it was not visible before.

**The signing key lives as a string in an environment variable, and this vendor documents nothing better.** That is the same exposure the *bearing on the requirements* section above already records across all four candidates — a secret is as safe as the code that can read it. What this reading adds is that on Cloudflare there is **no alternative at all**: no key-management product, no hardware-backed key, no key object surviving a request, searched across the whole Workers documentation set. **A signature over the check response is therefore as strong as the deploy path**, and `Licence service`'s §13 defences that rest on it inherit exactly that ceiling.

**`Date.now()` on this platform is documented to advance only after I/O.** §12.3 rule 1 makes the service's own statement of the time the value every count is measured against. R7 above records the clock as *not stated* for this vendor and this section does not revise that; what is added is that the vendor **does** document a behaviour of it. It is not a bar and it is not a defect, but it is a fact a downstream answer must hold, and it was not in this document before.

---

## Addresses added 2026-08-17

**Later additions to the list above, not covered by the citation check of 2026-08-16.** **Eighteen documentation addresses and two index documents** — thirteen pages new to this document, one further new page that turned out to be empty, and four already listed above and re-read — all at `developers.cloudflare.com`, all retrieved by direct anonymous HTTPS, all returning HTTP 200. **No forum, no blog, no comparison site, no search-result summary, no SDK repository, and no standards body's own page** — where a standard is named above it is the vendor naming it, quoted from the vendor's page.

**New pages (11)** — `https://developers.cloudflare.com/workers/runtime-apis/web-crypto/` · `/workers/runtime-apis/nodejs/crypto/` · `/workers/runtime-apis/nodejs/` · `/workers/runtime-apis/bindings/mtls/` · `/workers/runtime-apis/performance/` · `/workers/runtime-apis/web-standards/` · `/workers/reference/security-model/` · `/workers/examples/signing-requests/` · `/workers/examples/protect-against-timing-attacks/` · `/workers/best-practices/workers-best-practices/` · `/workers/wrangler/commands/workers/`

**New pages, Secrets Store (2)** — `https://developers.cloudflare.com/secrets-store/` · `/secrets-store/integrations/workers/`

**Already in the list above, re-read on 2026-08-17** — `/workers/configuration/secrets/` · `/workers/configuration/compatibility-flags/` · `/workers/platform/limits/` · `/d1/platform/limits/`. **Nothing they were quoted for above was re-ruled**; they were opened again for the questions in this section only.

**Two index documents, which are not documentation pages** and are marked as such wherever used, exactly as limit four marks Hetzner's price feed — `https://developers.cloudflare.com/workers/llms-full.txt` (`content-type: text/markdown`, 4,489,089 bytes) and `https://developers.cloudflare.com/d1/llms-full.txt` (`content-type: text/markdown`, 563,535 bytes), both retrieved 2026-08-17. **Both are the vendor's own machine-readable copy of its own documentation**, offered by a banner at the top of every page read, naming the index for that page's own product — *"Fetch the complete documentation index at: https://developers.cloudflare.com/workers/llms.txt"* on the Workers pages, and the equivalent address on the Secrets Store and D1 ones. They are used **only to state how widely a silence was searched**, never as the source of a quotation: **every quotation above was confirmed on the vendor's own page.** Neither carries a `last-modified` header, so like Hetzner's feed they are dated only by the moment they were fetched.

**One page returned HTTP 200 and is recorded because it is effectively empty:** `https://developers.cloudflare.com/workers/runtime-apis/bindings/secrets/` serves a heading, a breadcrumb and a *"Last updated Apr 23, 2026"* stamp, and no body text. **Nothing above rests on it**; the secret binding is quoted from `/workers/configuration/secrets/` instead.
