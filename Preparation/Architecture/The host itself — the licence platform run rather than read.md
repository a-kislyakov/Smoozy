# The host itself — the licence platform run rather than read

*What a throwaway Worker and D1 database, deployed to the Creator's own Cloudflare account on 2026-08-17, did when the four clauses `Licence service` rests on were put to it, together with the cold-start timing Fact 86 asks for and four smaller questions posed while the account was open.*

An **input** to the stage, like [The rented machine](The%20rented%20machine.md), [Batch A](Batch%20A%20—%20the%20shape%20throwaway.md) and [Six machines](Six%20machines%20—%20the%20container,%20the%20webhook%20and%20the%20ready%20action.md): nothing here is a decision or a plan. Rule 1's plain-English rule is suspended here, per the stage's exception.

**This is the first time anything in this project has observed the licence platform rather than read about it.** Licence service (`Preparation/Architecture/Licence service.md`, withdrawn) §15 item 3 records the position it replaces: *"Every claim about this host is documented assertion with zero observation."* Everything below is observation, on one account, on one day. Where an observation contradicts [Where the licence service runs](Where%20the%20licence%20service%20runs%20—%20the%20hosts%20read%20against%20what%20it%20must%20do.md) or Licence service, it says so and does not edit either.

**Every entry carries the line saying what a negative result would have looked like.** A test that cannot fail is not a test, so five of the eight questions below were run with a deliberate control that produced the negative — a second shape of the same transaction, shape B, that grants thirty slots out of thirty where two are allowed, the same three writes outside a batch that survive the failure, a damaged value read back before it is restored, a marker written to the console so that the search proves itself, and a payload altered by one character that the signature must reject.

---

## Five limits, before anything else

**One — this is one account on one day, and it is the Creator's own.** A different account, a different plan or a different month may differ in every particular. Read it as *this is what that account did*, never as *this is what Cloudflare does*.

**Two — the account is on the Workers Free plan, observed rather than read.** The plan itself could not be read: the token supplied refuses `/accounts/{id}/subscriptions` with `{"code":10000,"message":"Authentication error"}`. What was observed is the processor ceiling — see [question 8](#8-what-a-refusal-actually-looks-like) — and it matches the documented Free figure of ten milliseconds and rules out the Paid plan's thirty-second default. **Every clause of Licence service §1 that rests on the Paid plan is therefore unanswerable on this account and is recorded as such**, not as a failure: the thirty-day restore window, the five-minute processor ceiling, and the absence of the Free plan's daily D1 refusal.

**Three — the primary was in Prague and the caller in [redacted].** The database reports `served_by_region: "EEUR"`, `served_by_colo: "PRG"`; every request came from one caller, whose location is [redacted]. Every latency figure below carries that pairing and no other.

**Four — nothing pre-existing in the account was written to, read from, renamed, moved or deleted.** The proof is the two inventories below. Everything created carries the prefix `zz-fact85-`, and every one of those was deleted at the end.

**Five — nothing was bought, no plan was upgraded, no payment method was added, and no account-level setting was changed.** The only settings touched were on resources created for this work. **Nothing cost anything**, and the two limits reached in [question 8](#8-what-a-refusal-actually-looks-like) are refusals rather than charges.

---

## The account, before anything was created

*Personal account entries and caller-location values are withheld for publication. Smoozy entries and observed conclusions are retained.*

Read on **2026-08-17 05:24:19Z**, with an account-scoped token. The account is a `standard` account in the Creator's own name. The token is account-owned rather than user-owned, so `/user/tokens/verify` returns `{"success":false,...,"message":"Invalid API Token"}` while every account endpoint answers — a shape worth knowing before it is read as a bad credential.

| What | What was there |
|---|---|
| Workers scripts | **`smoozy-trial-api`** (created 2026-03-24, modified 2026-04-17), `smoozy-website` (created 2026-03-16) |
| D1 databases | **none** |
| KV namespaces | **`smoozy-trials`** |
| Durable Object namespaces | none |
| R2 buckets | `smoozy-website-opennext-cache` |
| Queues, Pages projects, Hyperdrive, Vectorize, Workflows, Tunnels, AI gateways, Turnstile widgets, mTLS certificates, Secrets Store stores | all empty |
| Workers custom domains | `smoozy.app` and `www.smoozy.app` → `smoozy-website` |
| Workers subdomain | `[redacted]` |
| Workers account settings | `{"default_usage_model":"standard","green_compute":false}` |
| Zones | the token cannot list zones; `/zones` returns an empty result while `workers/domains` names two zones, so **zone-level reading was outside this token and nothing zone-level was attempted** |
| Account API tokens | one, name `[redacted]`, active |
| Refused to this token | `subscriptions`, `images`, `stream`, `pipelines`, `dispatch namespaces` |

**`smoozy-trial-api` and `smoozy-trials` are the trial ticker the Creator said not to touch.** They were listed and never opened. No request was made to that Worker, no key was read from that namespace, and nothing was written to either. **The account held no D1 database at all before this work**, which is what makes the database created below unambiguously this work's own.

## What was created, and what it was for

Seven Workers and one D1 database, every name beginning `zz-fact85-`.

- **`zz-fact85-db`** — a D1 database created 2026-08-17 05:27:52Z, `running_in_region: "EEUR"`, `read_replication: {"mode":"disabled"}` with nothing asked for and nothing switched on. Five tables: `licence`, `slot`, `trial` — the three Licence service §3 names, with its own columns — plus `batchtest` and `restoretest`.
- **`zz-fact85-worker`** — one Worker bound to that database, holding an ECDSA P-256 private key in the platform's secret binding and the matching public key in a plain variable, exposing one route per question below.
- **`zz-fact85-cold-1` … `zz-fact85-cold-6`** — six identical copies of that Worker, deployed once and then left alone, so that six independent cold starts could be timed without waiting between them.

---

## 1. One primary

**The platform's own answer is yes, and it says so on every query.** A Worker running in [redacted], asked for the metadata of one read and one write:

```
{"read":{"by":"v3-prod","region":"EEUR","colo":"PRG","primary":true},
 "write":{"colo":"PRG","primary":true}}
```

`served_by_primary` came back `true` on every read and every write, from a colo that is not the caller's — so a read genuinely left [redacted] and went to the primary in Prague rather than being served locally. The same three fields appear on the command-line route: `"served_by":"v3-prod","served_by_region":"EEUR","served_by_colo":"PRG","served_by_primary":true`. And an error thrown by a query names the runtime class that produced it:

```
at D1DatabaseSessionAlwaysPrimary._sendOrThrow (cloudflare-intern…
```

**That confirms the routing claim and does not confirm what `Licence service` rests on it for.** The two are different, and this is the finding the runner most needs.

### The two-slot ceiling, three shapes, thirty and fifty simultaneous claims

One licence key, seeded `good`, its slot table emptied before each run. Thirty or fifty distinct computer identities claiming a slot at once, dispatched from one process over a connection pool warmed twice beforehand so that no request paid for a handshake.

**How concurrent it really was.** Client-side dispatch of the whole burst spanned **0.08 ms to 3.0 ms**. The service's own timestamp at handler entry spanned **9 to 20 ms** across every request in a burst, and in one forty-request burst **every one of the forty reported the same millisecond**. That is the honest figure: these are simultaneous within a few milliseconds at the caller and within about twenty at the service, not simultaneous to the microsecond.

| Shape | What it does | Slots granted where two are allowed |
|---|---|---|
| **A** — separate statements, no batch | read the licence, read this computer's slot, count the slots, then insert | **19 of 30**, then **45 of 50**, then **40 of 50** |
| **B** — read in one batch, decide in the Worker, write in a second batch | count in a batch; if under two, insert in a batch | **30 of 30** |
| **C** — the guard in SQL, one statement | `INSERT INTO slot … SELECT ? … WHERE (SELECT COUNT(*) FROM slot WHERE key_digest = ?) < 2 AND NOT EXISTS (…) AND EXISTS (…)` inside a batch | **exactly 2**, at 30 once and at 50 three times, every time |

Shape C's outcome, at fifty, three runs running: `{"200 entitled":2,"200 already_on_two_or_refused":48}`, and the table afterwards holds two rows each time.

**What a negative would have looked like:** a third slot row for one key digest under shape C. It did not happen in four runs. **What did happen is that shapes A and B produced exactly that negative**, which is what makes shape C's result a test rather than a formality.

**This is where Licence service §7 has to change.** §7 says *"Steps 3 to 5 are one statement sequence in one transaction on the primary"*, and step 4's decision — *if fewer than two slot rows exist for this key digest, insert one* — depends on step 3's result. **A D1 batch returns all its results only after the whole batch has committed**, so a decision taken from a read inside a batch cannot govern a write inside the same batch. Written literally, §7 is shape B, and shape B granted thirty slots out of thirty against a two-slot key. **The ceiling holds only when the guard is in SQL**, as shape C — one statement whose `WHERE` clause counts the rows it is about to add to.

### The trial record, forty simultaneous first claims for one computer

Forty requests, one computer identity, forty different trial claim values, dispatched together. Both shapes were tried.

- **Naive read-then-insert:** `{"500 D1_ERROR: UNIQUE constraint failed: trial.computer_identity: SQLITE_CONSTRAINT (extended: SQLITE_CONSTRAINT_PRIMARYKEY)":39,"200 entitled/recorded":1}`. **One row in the table.**
- **`INSERT OR IGNORE` plus a read-back in one batch:** `{"200 entitled":1,"200 not_valid":39}`, with exactly one request reporting that it wrote. **One row in the table.**

Both times the stored row was the first claim to arrive and the other thirty-nine were told the trial was already used.

**What a negative would have looked like:** two trial rows for one computer identity, or a second row silently replacing the first. Neither happened under either shape. **The primary key is what holds it, not the routing** — and it holds even when the Worker's own logic is wrong, which is the opposite of the slot case.

**One consequence for Licence service §10.** The naive shape answers thirty-nine of forty callers with HTTP 500 and a database error string. §10's rule — *the service never answers a refusal it does not mean* — is kept, because an unsigned 500 is *could not be reached* by §5's own table. But **the shape that produces it is the one a builder writes first**, and it turns a normal race into an error for every caller but one.

---

## 2. A batch is atomic

Three statements, the second of them a deliberate `UNIQUE` violation, the first and third valid.

**The batch:**

```
{"threw":true,
 "message":"D1_ERROR: UNIQUE constraint failed: batchtest.note: SQLITE_CONSTRAINT (extended: SQLITE_CONSTRAINT_UNIQUE)",
 "cause":"UNIQUE constraint failed: batchtest.note: SQLITE_CONSTRAINT (extended: SQLITE_CONSTRAINT_UNIQUE)"}
```

and the table afterwards: `[]`. **The statement before the failure did not survive it.**

**A second failure of a different kind** — the middle statement naming a table that does not exist:

```
{"threw":true,"message":"D1_ERROR: no such table: no_such_table: SQLITE_ERROR", …}
```

table afterwards: `[]`.

**The control, and it is what makes this a test.** The same three writes issued as three separate statements rather than as a batch:

```
{"out":[{"id":21,"ok":true},
        {"id":22,"ok":false,"message":"D1_ERROR: UNIQUE constraint failed: …"},
        {"id":23,"ok":true}]}
```

table afterwards: `[{"id":21,…},{"id":23,…}]`. **Two rows survive.**

**What a negative would have looked like:** the row inserted before the failure still present after the batch threw — exactly what the control shows. **Held**, and the vendor's own words — *"it aborts or rolls back the entire sequence"* — are what was observed.

---

## 3. The always-on point-in-time restore restores

**Held.** The sequence, run twice:

1. Write `zz-fact85-ORIGINAL-VALUE` into a row.
2. Take a bookmark: `00000000-000000fa-000050ca-68d20fcab74733f0d843eba0b1b912bd`.
3. Overwrite the row with `zz-fact85-DAMAGED-VALUE`, and confirm the damage by reading it back.
4. `wrangler d1 time-travel restore zz-fact85-db --bookmark=…`
5. Read the row.

Step 5, both times: `{"payload":"zz-fact85-ORIGINAL-VALUE"}`.

**What a negative would have looked like:** the damaged value still there after the restore, or the row gone. Neither. Step 3's read-back is the check that the damage was real, so the restore is not being credited with undoing nothing.

**How long it took.** The restore command returned in **3.50 s** and **3.56 s** on the two runs. Nothing was enabled to make it available; the database was created with no options.

**What a caller saw while it ran.** A second process polled the service every 150 ms across the whole restore. The outcome sequence:

| Answer | Count | From | To |
|---|---|---|---|
| `200 {"payload":"zz-fact85-DAMAGED-VALUE"}` | 24 | 05:49:26.478Z | 05:49:31.130Z |
| `500 D1_ERROR: {"D1_RESTORE_BOOKMARK_RESET_DO":true,"message":"Resetting D1 DB to restore database to bookmark: …"}` | **1** | 05:49:31.315Z | 05:49:31.315Z |
| `200 {"payload":"zz-fact85-ORIGINAL-VALUE"}` | 70 | 05:49:31.621Z | 05:49:46.343Z |

**Exactly one poll in flight was refused, and the unavailable window was under half a second** — 05:49:31.130Z to 05:49:31.621Z — inside a restore whose command took three and a half seconds. The vendor's *"Queries in flight will be cancelled, and an error returned to the client"* is what was observed, and the error is catchable inside the Worker rather than a platform-level failure, so Licence service §10's *could not be reached* can be produced from it deliberately.

**How far back it will go could not be established, and that is a real result rather than a failure.** The database was minutes old, so no restore point older than the database exists to reach for. What was observed is the shape of the refusal: a bookmark asked for at a moment before the database existed returns

```
{"success":false,"result":null,
 "errors":[{"code":7500,"message":"internal error; reference = e_xSb5mO_a886d0b3df13406fb76537f81d808210"}]}
```

— **an opaque internal error, not a stated boundary.** The same code came back for one day, six days, eight days, twenty days and twenty-nine days back, so **nothing distinguishes *outside your plan's window* from *before this database existed* from *something went wrong*.** The only boundary that announces itself is a client-side one in `wrangler`, which refuses a timestamp older than thirty days before contacting the platform at all: `Invalid timestamp '2026-07-08T05:31:04Z'. Please provide a timestamp within the last 30 days` — **and that check does not know which plan the account is on**, so it is not evidence of a thirty-day window here.

**Unanswerable on this account:** whether the window is seven days or thirty. It needs a database older than seven days.

---

## 4. Request logging when it is turned off

**This is the clause that disqualified another candidate, and it produced the largest gap between what is documented and what happens.**

Four markers per pass, each a distinct random string, sent to two routes: `/marker-quiet`, which writes nothing to the console, and `/marker-logged`, which writes one console line carrying the request body. Each request carried one marker in the JSON body and another in a custom header `x-zz-marker`. Afterwards the account's own stored log was queried through the Workers Observability API, filtered to this Worker, over a window a few seconds wide.

### Pass one — `invocation_logs = true`, the default

Three events in the window, triggers `POST /marker-logged` ×2 and `POST /marker-quiet` ×1.

| Marker | Where it was put | Occurrences in the log |
|---|---|---|
| `ZZQUIETBODY-…` | body of the quiet request | **0** |
| `ZZQUIETHEADER-…` | header of the quiet request | **1** |
| `ZZLOGGEDBODY-…` | body of the logged request | **2** |
| `ZZLOGGEDHEADER-…` | header of the logged request | **1** |

**The positive control is the point.** The console-logged body marker was found, so the search would have found the quiet body marker had it been there. **It was not.**

**What the invocation log does hold**, from one event verbatim, with identifying values redacted for publication:

```
"headers": {
  "accept": "*/*", "accept-encoding": "gzip, br",
  "cf-connecting-ip": "[the caller's public IP address, redacted]",
  "cf-ipcountry": "[redacted]", "cf-ray": "a2c64f17ad50b01d",
  "content-length": "107", "content-type": "application/json",
  "host": "[redacted].workers.dev",
  "user-agent": "curl/8.7.1",
  "x-forwarded-proto": "https", "x-real-ip": "[the same IP, redacted]",
  "x-zz-marker": "ZZMARKERLOGGEDONE-46d2f76b6074"
},
"method": "POST",
"url": "https://[redacted].workers.dev/marker-logged"
```

and beside it, in the same event: `"colo":"[redacted]"`, `"country":"[redacted]"`, `"city"`, `"region"`, **`"postalCode"`**, **`"longitude"`**, **`"latitude"`**, `"timezone":"[redacted]"`, `"asn"`, `"asOrganization"` naming the caller's internet provider, `"tlsClientRandom"`, `"tlsClientCiphersSha1"`, `"tlsClientExtensionsSha1"`, `"tlsClientHelloLength"`, plus `"cpuTimeMs"`, `"wallTimeMs"` and `"outcome"`. **Every one of those five bolded fields carried a real value identifying where the caller was; the values are redacted here and the field names are the finding.**

**So: every request header in full, the caller's IP twice, their city, postcode, latitude and longitude, their network operator, and a TLS fingerprint. No request body, and no response body.**

### Pass two — `invocation_logs = false`, the documented setting

Same four markers, fresh values, same query.

**One event in the window**, trigger `POST /marker-logged`, and it is the console line:

```
{"source":{"message":"zz-fact85 console line carrying {\"licence_key\":\"ZZOFFLOGGEDBODY-01102df644cb\"}"},
 "$workers":{"scriptName":"zz-fact85-worker",
   "event":{"request":{"method":"POST","url":"https://…/marker-logged","path":"/marker-logged"}}}}
```

- quiet body marker: **0**
- quiet header marker: **0**
- logged body marker: 2 — it is in the line the code itself wrote
- logged header marker: **0**

**The quiet request produced no event at all.** The setting does what the vendor says: `"Invocation logs can be disabled in wrangler by adding the invocation_logs = false configuration."` **Held.** And what remains of a console line is stripped of headers, IP and geolocation, keeping only the method, URL and path.

### Pass three — `observability.enabled = false`

**Zero stored events in the window, of any kind, including the console line.**

### And the thing that is not covered by any of it

`wrangler tail` was attached to the Worker during pass two and pass three and a marker request sent while it watched. With `invocation_logs = false`:

```
"event": { "request": {
  "url": "https://[redacted].workers.dev/marker-quiet",
  "method": "POST",
  "headers": { …, "cf-connecting-ip": "[redacted]", …, "x-zz-marker": "ZZTAILHEADER-2123cac03759" },
  "cf": { "colo":"[redacted]", "city":"[redacted]", "postalCode":"[redacted]", "latitude":"[redacted]", … } } }
```

- header marker in the tail: **1**
- body marker in the tail: **0**

With `observability.enabled = false` as well: header marker in the tail **2**, console marker **1**, body marker **0**, and stored events **0**.

**So the documented setting turns off the stored log and does not turn off the live stream.** Anyone holding a token that can tail this Worker sees every request header and the caller's IP in real time, whatever the observability setting says. **What is never seen either way is the request body.**

### What this changes for `Licence service`

**Three things, and the first two are corrections to what the hosts reading records.**

1. [Where the licence service runs](Where%20the%20licence%20service%20runs%20—%20the%20hosts%20read%20against%20what%20it%20must%20do.md) R9 records *"no page read says whether request bodies are captured, or what … includes beyond the method and URL named in the message"*, and that `header` never appears describing what is captured. **Observed: request bodies are not captured; the complete header set is, along with the caller's IP and their location to a postcode.**
2. Licence service §1 reason 3 reads Cloudflare's disclosure as *"states what it captures by default and states how to stop it"*. The stopping is real for the stored log and does not extend to `wrangler tail`, which no page read mentions in this connection.
3. **§5's rule that the key and the computer identity travel in the body and never in the address or a query string is what keeps them out of the log**, and it is now the load-bearing part rather than a tidiness rule. A key in a query string lands in `"url"`, which is captured in all three passes and in the tail. **The same rule must extend to headers**, which §5 does not currently say: a custom header is captured in full.

---

## 5. The cryptographic primitives, run rather than documented

**Every one of the five ran, in the deployed Worker, with the results below.** Licence service §15 item 11 records them as documented with zero observation; this is the observation.

**An asymmetric signature, produced by the service and verified off the platform.** The service's ECDSA P-256 private key was stored in the platform's secret binding as base64 PKCS8, imported with `crypto.subtle.importKey("pkcs8", …, false, ["sign"])`, and used to sign a payload of the shape §5 describes. The matching SPKI public key was shipped in a plain variable, standing in for the one §5 ships in the application.

- Verified **inside** the Worker against that public key: `true`.
- Verified **outside** the platform, in Node on the Creator's own machine, against the same public key: `{"desktop_side_verify_good":true,"desktop_side_verify_tampered":false}`.

**That closes the half the hosts reading says no Cloudflare page bears on.** The signature is 64 bytes; the public key is 91 bytes as SPKI.

**A freshly generated keypair, both asymmetric algorithms:**

| Algorithm | Signature | Public key | Correct payload | Payload altered by one character |
|---|---|---|---|---|
| `ECDSA` P-256 with `SHA-256` | 64 bytes | 91 bytes SPKI | `true` | `false` |
| `Ed25519` | 64 bytes | 32 bytes raw | `true` | `false` |

**`HMAC` — the algorithm the vendor's own worked example uses — was not used anywhere above**, because §5 ships a public key and a symmetric key cannot be shipped.

**A salted digest.** `HMAC-SHA-256` over the licence key with a fixed service-held salt, which is §3's arrangement:

```
"hmac_sha256":                 "UwyTip9lLaEOedyEKHkXGgRhPFlZLuEu6DLV/1gAkEM="
"hmac_sha256_same_key_again":  "UwyTip9lLaEOedyEKHkXGgRhPFlZLuEu6DLV/1gAkEM="
"hmac_sha256_other_key":       "18/+l9EgT/g9fW3uCzXUzOrtiop3lxNiGYGfkuXtpQs="
"plain_sha256":                "B8CZfSmr01ucA7ifoEsybAQQyFVeCvvd25uus0//1v8="
"pbkdf2_100k":                 "MNrVDOWiNi+EMTneEXgmfjRpYKr5tLxpFWeXeVF6dfo="
```

Deterministic for one key, different for another, and different again from the unsalted digest of the same key. `PBKDF2` at 100,000 iterations with `SHA-256` also ran.

**Random values, and a ceiling nobody had.** `getRandomValues` filled 32 bytes twice with different results; `randomUUID()` returned `6da9fd5a-6ca0-41ac-bb20-412651acdb69`. The hosts reading records *"No size ceiling on `getRandomValues` is stated on any page read … Label: not stated — a silence, not a guarantee."* **The silence is now closed by observation:**

```
65536 bytes: "ok"
65537 bytes: "getRandomValues() only accepts buffers of size <= 64K but provided 65537 bytes."
```

**Constant-time comparison.** `typeof crypto.subtle.timingSafeEqual` is `"function"`; equal buffers `true`, unequal buffers `false`; and unequal lengths throw, with the vendor's documented edge in its own words:

```
"Input buffers must have the same byte length."
```

**What a negative would have looked like:** a verify returning `true` for an altered payload, a salted digest that differed between two runs on the same key, two identical draws from `getRandomValues`, or `timingSafeEqual` missing from the runtime. The altered-payload verify returning `false` in four places is the check that the verification is real.

---

## 6. What one licence check costs in processor time

**Measured against the free plan's ten-millisecond ceiling, using the platform's own per-invocation `cpuTimeMs` from the invocation log.** Licence service §1 records that *"no page read gives a processor cost for any cryptographic operation, so the free plan can be shown neither to fit nor not to fit."* It fits, with room to spare.

The route measured is the whole of §5: the salted digest of the presented key, one D1 batch of three statements — read the licence, read the slot list, update this computer's last-checked — and an ECDSA signature over the response payload.

| Route | What it does | `cpuTimeMs`, per invocation | `wallTimeMs` |
|---|---|---|---|
| `/ping` | nothing at all | `0,0,0,0,0,0,0,0` | `0` × 8 |
| `/check` | **one whole licence check** | `0,1,0,0,0,0,1,0,0,2` | `40,40,40,39,41,39,39,39,45,44` |
| `/check-nodb` | digest and signature, no database | `0` × 10 | `0` × 10 |
| 100 × digest+signature | | `4,6,4,4,4,4` | `5,6,4,4,4,5` |
| 400 × digest+signature | | `14,15,14,20,20` | `14,15,14,20,21` |
| 1000 × digest+signature | | `41,34,36,41` | `41,35,36,41` |
| 1000 × salted digest only | | `11,10,9,10,9,10` | `12,11,9,10,10,10` |

**So, dividing out:**

- one salted `HMAC-SHA-256` digest: **about 0.010 ms**
- one `HMAC-SHA-256` digest plus one ECDSA P-256 signature: **about 0.037 ms**, so the signature alone is about **0.027 ms**
- **one complete licence check: 0 to 2 ms, median 0**, which is to say below the invocation log's own whole-millisecond resolution

**The whole-millisecond figure was then sharpened to microseconds** on the platform's own analytics, which reports processor time in microseconds and by quantile. Two clean runs, one route each, filtered to this Worker over a thirty-second window:

| Route | Requests | P25 | P50 | P75 | P99 |
|---|---|---|---|---|---|
| `/ping` — an empty JSON response and nothing else | 80 | 209 µs | **218 µs** | 231 µs | 311 µs |
| `/check` — **one whole licence check** | 77 | 742 µs | **867 µs** | 942 µs | 1640 µs |

**One complete licence check costs 0.87 ms of processor time at the median and 1.64 ms at the ninety-ninth percentile, against the free plan's ten-millisecond ceiling.** The platform's own floor for answering anything at all is 0.22 ms of that, so the check itself adds about 0.65 ms — of which the cryptography is 0.037 ms and the rest is parsing the request and marshalling the database call.

**Roughly two hundred and fifty checks' worth of the cryptography, or eleven whole checks, fit inside the free plan's ten-millisecond ceiling.** The vendor's *"Heavier workloads that handle authentication … typically use 10-20 ms"* does not describe this workload. **`Licence service` §1's third reason for the paid plan therefore falls away as an argument; its first two do not.**

*(The analytics rows echo `scriptName` as `"__unknown__"` at this granularity; the filter was on `scriptName: "zz-fact85-worker"` and the request counts match what was sent, 77 and 80 against 80 and 80.)*

**`wallTimeMs` of 39 to 45 ms against `cpuTimeMs` of 0 is the D1 round trip from [redacted] to Prague**, and it is time the platform does not bill against the processor ceiling.

**One thing the measurement showed that the reading did not.** Invocations measured at **14, 15 and 20 ms** of processor time returned HTTP 200 on the free plan. The documented *"built-in flexibility to allow for cases where your Worker infrequently runs over the configured limit"* is real and was observed; the ten-millisecond figure is not a hard wall.

---

## 7. The clock

**The documented behaviour was watched happening.** One request, taking eight readings of `Date.now()` and `performance.now()`:

| Where | `Date.now()` | `performance.now()` |
|---|---|---|
| handler entry | 1786945590888 | 1786945590888 |
| after pure computation | 1786945590888 | 1786945590888 |
| after more pure computation | 1786945590888 | 1786945590888 |
| **after a D1 read** | **1786945591116** | **1786945591116** |
| after pure computation | 1786945591116 | 1786945591116 |
| **after a second D1 read** | **1786945591134** | 1786945591134 |
| **after `scheduler.wait(1000)`** | **1786945592134** | 1786945592134 |
| after a third D1 read | 1786945592154 | 1786945592154 |

with `"timeOrigin": 0`.

**Three things follow.** The clock is frozen while code runs and advances only across I/O, exactly as documented. `performance.now()` is not a relative timer here — it returns the same epoch value as `Date.now()`, which is what `timeOrigin` being zero means. And a wait of one second advanced it by exactly 1000.

**Whether the service can state the time accurately enough.** Twelve `/ping` calls, each timed at both ends, comparing the service's stated time against the midpoint of the request:

```
offsets, ms: 192.5, 167.5, 168, 167.5, 205.5, 169, 169, 128, 169.5, 170.5, 170.5, 173.5
min 128   max 205.5   mean 170.9
```

and, immediately afterwards, this machine's own clock against a public time source:

```
+0.169537 +/- 0.068964 time.apple.com 17.253.38.35
```

**The machine was 169.5 ms behind true time and the service read 170.9 ms ahead of the machine, so the service's clock agrees with a public time source to within roughly a tenth of a second.** **Corrected 2026-08-23 at the stage-close gate; this read *to within about one and a half milliseconds*, and the readings above are untouched.** The `+/-` term printed by `sntp` is that query's own error bound rather than a spread the 170.9 ms offset can be taken through: the same instrument re-run on the same machine on 2026-08-20 — five consecutive queries, nothing set or slewed — returned error terms of 33 to 59 ms while the offset barely moved, so a quantity known only to within tens of milliseconds cannot be subtracted from another and the difference stated to 1.5 ms. The 170.9 ms is also a one-way apparent offset over a [redacted]-to-Prague round trip and carries a path asymmetry of the same size. **The conclusion is unchanged.** The boundaries it feeds — Licence service §6's seven-day trial ending and §12's grant expiry — are measured in days. **There is no accuracy problem here of any size.**

**What a negative would have looked like:** the clock advancing across a pure computation, or a stated time minutes or hours away from a public source. Neither. **The offsets clustering at 169 ms with two outliers at 128 and 205.5 is the round trip's own asymmetry, not the clock's.**

---

## 8. What a refusal actually looks like

**Two documented limits were reached and neither cost anything.**

### The processor ceiling

A route doing nothing but arithmetic, sized by an iteration count rather than by a clock, since the clock is frozen while it runs:

```
burn ms=5      http=200
burn ms=50     http=200
burn ms=200    http=503   error code: 1102
burn ms=400    http=503   error code: 1102
burn ms=800    http=503   error code: 1102
burn ms=1600   http=503   error code: 1102
```

**The refusal is `HTTP/2 503` with the three-word body `error code: 1102`**, served with `server: cloudflare` and a `cf-ray` header. **It never reaches the Worker's own code**, so the Worker cannot dress it as anything else: whatever the service would have answered, the caller gets 503 and that string. **Corrected 2026-08-23: this said *six-word*. The observed string is unchanged and is three words; Licence service §10 called it three-word and was right.**

**That is exactly the shape Licence service §10 says must not be confused with a refusal about this key**, and the desktop side of the rule holds without effort — an `error code: 1102` body carries no signature and echoes no per-request value, so §5's table puts it in *could not be reached* by construction.

**It is not perfectly reproducible at one input.** One 400 ms run earlier in the session returned 200 with `"iters":160000000`, before the same input began failing consistently. That is the documented flexibility, and it means **a service sitting near the ceiling refuses some callers and not others**.

### The D1 value-size ceiling, which is not where it is documented

Writing a single text value of increasing size into one column:

```
1,000,000 bytes  {"ok":true,"stored":1000000}
1,999,000 bytes  {"ok":true,"stored":1999000}
2,000,001 bytes  {"ok":true,"stored":2000001}
3,000,000 bytes  {"ok":false,"message":"D1_ERROR: string or blob too big: SQLITE_TOOBIG"}
```

Bisected: **largest value stored 2,199,218 bytes; smallest value refused 2,200,194 bytes.**

**The documented figure is *"Maximum string, BLOB or table row size | 2,000,000 bytes (2 MB)"*, and the platform accepted 2,199,218.** The documented ceiling is conservative rather than wrong, and **nothing about the licence record comes anywhere near either figure** — the largest thing §3 stores is a bounded 255-byte computer name.

Unlike the processor refusal, this one **arrives inside the Worker as a catchable error**, so a service can answer it deliberately.

### What was not reached, on purpose

The free plan's daily D1 read and write ceilings, and its daily request ceiling, were not approached: reaching them would have taken tens of thousands of writes and would have refused every subsequent request from the account for the rest of the day — including anything the pre-existing trial ticker does. **Not reaching them is a deliberate choice, not a gap in the method.**

---

## Fact 86 — how long a caller waits for the first request to an idle service

**Answered. The worst first call to a genuinely idle service was 0.567 s end to end, including a durable write and an asymmetric signature.** **Corrected 2026-08-21: the seller's five seconds is the gap before its second delivery attempt, not a deadline the receiver must answer inside** — the vendor's sentence governs what happens *"If the response status is anything else"*. **No response deadline is published**, on any of its three webhook pages, searched 2026-08-20 with a working control. The measured figures are unchanged and are small against any plausible value; `Licence service` §15 item 4 carries the reopened question.

**How the idleness was made real.** Six identical Workers were deployed and then not called at all. Each is a separate script with its own hostname, so six independent first-calls could be taken without waiting between them. The route called is the whole of Licence service §5 — a salted digest, a D1 batch that **updates a row**, and an ECDSA signature over the response — and the row updated was seeded beforehand from the command line, so each call is a genuine durable write rather than an update that changes nothing.

**Round one, after 36 minutes idle**, and every column is seconds:

| Worker | DNS | TLS done | first byte | total | HTTP |
|---|---|---|---|---|---|
| cold-1 | 0.027 | 0.061 | 0.566 | **0.567** | 200 |
| cold-2 | 0.024 | 0.055 | 0.193 | **0.194** | 200 |
| cold-3 | 0.024 | 0.055 | 0.118 | **0.121** | 200 |
| cold-4 | 0.023 | 0.051 | 0.113 | **0.114** | 200 |
| cold-5 | 0.021 | 0.050 | 0.110 | **0.111** | 200 |
| cold-6 | 0.017 | 0.138 | 0.200 | **0.202** | 200 |

**Round two, after a further 18 minutes idle:**

| Worker | DNS | TLS done | first byte | total | HTTP |
|---|---|---|---|---|---|
| cold-1 | 0.029 | 0.061 | 0.535 | **0.536** | 200 |
| cold-2 | 0.031 | 0.069 | 0.132 | **0.135** | 200 |
| cold-3 | 0.018 | 0.047 | 0.108 | **0.108** | 200 |
| cold-4 | 0.026 | 0.052 | 0.110 | **0.110** | 200 |
| cold-5 | 0.104 | 0.136 | 0.199 | **0.201** | 200 |
| cold-6 | 0.019 | 0.053 | 0.113 | **0.114** | 200 |

**Twelve first calls to an idle service: minimum 0.108 s, median 0.128 s, maximum 0.567 s.** Every one returned HTTP 200 carrying `"outcome":"entitled"`, the per-request value echoed back, and an 88-character signature — so **a durable write and a signed response fit, twelve times out of twelve.**

**Two contrasts, so the cold figure means something.**

- **Warm**, the same six called again straight away, three rounds of six: **0.091 s to 0.185 s**, median about 0.097 s.
- **First call after a fresh deploy** — a version that has never run anywhere — three Workers: **0.092 s, 0.098 s, 0.138 s**. **No measurable penalty at all**, which matches what the deploy itself reports: `Worker Startup Time: 4 ms`.

**What the spread is actually made of, said plainly rather than hidden in an average.** The slowest call in each round was **the first hostname called in that round**, both times, and it was slow at first byte rather than at DNS or TLS. **So part of the 0.5 s is the caller's own first-connection cost on a home line and part is the service; this measurement cannot separate them**, and the honest reading is that a first call is *somewhere under six-tenths of a second*, not that a cold Worker costs half a second. The nine calls that were not first-in-round all landed between 0.108 s and 0.202 s.

**What a negative would have looked like:** a first call taking longer than about five seconds, or answering unsigned, or answering before the write had committed. **None of the twelve did.** The worst observed is **about one ninth of the gap before the seller's second delivery attempt**, and the median is about one fortieth of it. **Corrected 2026-08-21: the seller's five seconds is the gap before its second delivery attempt, not a deadline the receiver must answer inside** — the vendor's sentence governs what happens *"If the response status is anything else"*. **No response deadline is published**, on any of its three webhook pages, searched 2026-08-20 with a working control. The measured figures are unchanged and are small against any plausible value; `Licence service` §15 item 4 carries the reopened question.

**What this does not establish.** Nothing here was a real webhook: no seller signature was verified, no outbound call to a seller API was made, and the payload was this project's own. Licence service §9's revocation route adds a signature verification and possibly one outbound `fetch` to that figure, and **an outbound call to a third party is not bounded by anything measured here**. What is settled is the platform's own share of it.

**So Row 237's immediacy is the ordinary path rather than the exception**, on this evidence, and the daily reconciliation remains what it already was — the answer to a webhook that never arrives rather than to one that arrives late.

---

## What could not be posed on this account

Four things, each a real result rather than a failure.

1. **Anything that needs the Workers Paid plan.** The account is on the free plan (question 8), and the brief forbids upgrading. So Licence service §1's five-minute processor ceiling, its thirty-day restore window, and its escape from the free plan's daily D1 refusal are all **unobserved**, and the same document's argument for taking the paid plan is untouched by this work.
2. **How far back a restore will actually go.** It needs a database older than the plan's window; this one was minutes old.
3. **Anything zone-level, including a custom domain.** The token cannot list zones, and Licence service §12 turns on the address never changing. **Nothing about a custom domain was observed**, and the two zones the account holds were not touched.
4. **Anything about the seller.** No account, no key, no product, no webhook. [Facts](Facts.md) Fact 87 was answered by reading and this work adds nothing to it.

---

## Where this disagrees with what was read

**Seven places, gathered here because the runner needs them in one list. Neither input document is edited by this one.**

| # | What the reading records | What was observed |
|---|---|---|
| 1 | Licence service §7 — *"Steps 3 to 5 are one statement sequence in one transaction on the primary"*, which is what makes the two-slot ceiling hold | **Not implementable as written.** A batch returns its results only after committing, so a decision taken from a read inside a batch cannot govern a write inside it. Written that way it is shape B, and under shape B thirty simultaneous claims took thirty slots. The ceiling holds only with the guard in SQL. [Question 1](#1-one-primary) |
| 2 | Licence service §1 — one authoritative copy with serialized writes is the reason the ceiling is safe | **One primary is confirmed and is not sufficient.** Every read and every write reports `served_by_primary: true`, and that alone stops nothing: three shapes of the same transaction, where two are allowed, gave as many as 45 of 50 under shape A, 30 of 30 under shape B, and exactly 2 under shape C. [Question 1](#1-one-primary) |
| 3 | [Where the licence service runs](Where%20the%20licence%20service%20runs%20—%20the%20hosts%20read%20against%20what%20it%20must%20do.md) R9 — *"no page read says whether request bodies are captured"*, and `header` never appears describing what is captured | **Bodies are not captured. The complete header set is, along with the caller's IP address, city, postcode, latitude, longitude, network operator and TLS fingerprint.** [Question 4](#4-request-logging-when-it-is-turned-off) |
| 4 | Licence service §1 — the vendor *"states what it captures by default and states how to stop it"* | **The documented setting stops the stored log and does not stop `wrangler tail`,** which streams every header and the caller's IP with invocation logs off and with observability off entirely. [Question 4](#4-request-logging-when-it-is-turned-off) |
| 5 | Licence service §1 — *"no page read gives a processor cost for any cryptographic operation, so the free plan can be shown neither to fit nor not to fit"* | **It fits.** One whole licence check is 0.87 ms at the median and 1.64 ms at the ninety-ninth percentile against a ten-millisecond ceiling; the cryptography alone is 0.037 ms. That reason for the paid plan falls away; §1's other two do not. [Question 6](#6-what-one-licence-check-costs-in-processor-time) |
| 6 | [Where the licence service runs](Where%20the%20licence%20service%20runs%20—%20the%20hosts%20read%20against%20what%20it%20must%20do.md) Q3 — *"No size ceiling on `getRandomValues` is stated on any page read … a silence, not a guarantee"* | **The ceiling is 64 KiB**, with the runtime's own message. [Question 5](#5-the-cryptographic-primitives-run-rather-than-documented) |
| 7 | The vendor's *"Maximum string, BLOB or table row size \| 2,000,000 bytes (2 MB)"* | **2,199,218 bytes stored; 2,200,194 refused.** The documented figure is conservative rather than wrong. [Question 8](#8-what-a-refusal-actually-looks-like) |

**And two places where what was read held exactly.** The batch is atomic, in the vendor's own words and under a control that proves the control could have failed ([question 2](#2-a-batch-is-atomic)); and the clock behaves precisely as the *Performance and timers* page describes, down to `timeOrigin` being zero ([question 7](#7-the-clock)).

---

## The account, after everything was deleted

Every Worker and database created for this work was deleted, matched against the `zz-fact85-` prefix before anything was sent, with the script refusing any name that did not match:

```
deleted zz-fact85-cold-1 -> {"success":true,"errors":[]}
deleted zz-fact85-cold-2 -> {"success":true,"errors":[]}
deleted zz-fact85-cold-3 -> {"success":true,"errors":[]}
deleted zz-fact85-cold-4 -> {"success":true,"errors":[]}
deleted zz-fact85-cold-5 -> {"success":true,"errors":[]}
deleted zz-fact85-cold-6 -> {"success":true,"errors":[]}
deleted zz-fact85-worker -> {"success":true,"errors":[]}
deleted zz-fact85-db     -> {"success":true,"errors":[]}
```

The same inventory was then re-run, at **2026-08-17 06:37:22Z**, against the one taken at **05:24:19Z** before anything was created. **The two are identical line for line**, across all twenty-seven things the token can see — including the modification timestamps of all three pre-existing Workers, which are unchanged at 2026-05-27, **2026-04-17** and 2026-04-09.

**So `smoozy-trial-api` and `smoozy-trials` are exactly as they were found.** Neither was called, read, written, renamed or redeployed at any point, and the account holds no D1 database now, as it held none then.
