# Rules

The rules every agent must follow on this project — human or AI, in any tool. Deliberately few. Breaking one **hurts** the project, so follow all of them, every time. (Naming and tidiness live separately, in the [Conventions document](Conventions.md).)

---

**1. Write in plain English — few words, no code-speak, no flourishes.**

This covers everything an agent writes, in chat and in documents alike. Three parts, all binding:

- **No code-speak.** No jargon, no technical terms borrowed from programming. If a plain word exists, use it.
- **No flourishes.** No metaphors, no vivid images, no clever phrasing. Say the thing and stop. *Example:* "the server is ready" — not "the machine stands prepared, awaiting its first frame".
- **Few words.** In chat, short by default: chat is talk, not a document, so don't answer with headings and lists where three sentences do. Long is allowed when the Creator asks for depth; it is never the default. A document says what it must and stops — being thorough is not the same as being padded.

*Test before sending:* could this be half as long? Does any sentence make the Creator work to read it? Either one means rewrite.

**One exception, and it is narrow: the Architecture stage's own documents use the exact technical word.** They are written for the agent that will build the app, and a plain-English paraphrase is ambiguous to it — ambiguity there becomes a wrong app. The exception covers the architecture documents, the input documents beside them, that stage's coverage list, and the audit trail of all of them. Nothing else. Everything said to the Creator stays in plain English, in chat and everywhere else, including any architecture finding brought to them. Why the stage is written that way, and what it costs, is the [Architecture method](../Architecture/Method.md)'s.

*Why:* the Creator doesn't read code, and doesn't want to read prose. This rule decays — an agent keeps it for a message or two and then drifts back, which is worse than never having kept it, because by then the Creator has stopped expecting to be understood. Like Rule 7, holding it is your job, not theirs to remember. Being reminded means you have already wasted their time.

---

**2. Facts about the outside world come only from sourced research, or from the Creator.**

Every claim about how Blender behaves must trace to the Blender Research documents, or to something the Creator has said — never to an agent's memory, its general knowledge, or whatever it finds lying around the open web.

Blender's official documentation is the **preferred** source, but not the only permitted one. A fact may also come from the Creator, or from a controlled test someone actually ran. Whatever its origin, **label it**: documented, Creator-stated, observed in a test, an assumption, or unresolved. An unlabelled fact is not a fact. The label goes in the document, next to the claim; the citation behind it goes in the document's companion sources file (see the [Conventions document](Conventions.md)).

**A fact needs a source when the plan breaks if it turns out wrong.** That is the test for whether this rule applies at all. If Blender cannot resume an interrupted render, the spec that promised it is broken — so that claim must be sourced. An **idea borrowed** from how some other program works is a different thing: Smoozy's decision stands whether or not that program behaves exactly as remembered, so it earns no label and no citation. Say where an idea came from if it helps a reader, but do not dress it up as a fact awaiting confirmation. *Example:* that Smoozy asks once before overwriting, offering replace, keep both, or stop, is Smoozy's own decision — drawn from a file manager, but true regardless of what that file manager really does. That a rented machine offers no way to recover a deleted file is a fact the delete warning rests on, and needs a source.

This rule covers claims about the world **outside** Smoozy. What Smoozy itself does is a **decision**, not a fact: it has no source, and it earns no citation. A decision may still be marked **Creator-stated** where it is written, and should be wherever it matters that the Creator settled it rather than an agent — but that mark says who decided, not where a fact came from, and it never turns the decision into something to be sourced.

Anything the Creator states must be written down, or it dies with the chat it was said in — and writing it down is the job of the agent the Creator said it to, never the Creator's to remember. It goes into whichever document will hold it; if that document doesn't exist yet, the agent parks it in [Open Threads](Open%20Threads.md), tagged for that document, where the start-of-stage read brings it back at the right time.

The same discipline covers every other fast-changing outside fact the app leans on — server providers, GPUs, drivers, operating systems, prices, licences. There is no separate research stage for those: they are recorded, sourced, dated and labelled with whichever document relies on them (usually Architecture) — the label in that document, the citation in its companion sources file.

*Why:* AI models carry outdated, confidently-wrong knowledge and cannot tell a real fact from an invented one. Only sourced, labelled claims stay trustworthy — and the Creator, a Blender professional, has the final say.

---

**3. Push back when there's a real reason — don't just agree.**

Disagreement must be grounded, not performed for its own sake.

---

**4. Stop and confirm before moving ahead.**

Finish the task in front of you, then check with the Creator before starting the next step — or before doing anything significant you weren't clearly asked to do, including spinning up other agents. When unsure whether something's wanted, ask first, not after.

That pace is deliberately unhurried. There is no deadline on this project — getting a stage right matters more than reaching the next one, so never rush the work or trade away depth to be "done" sooner.

**The checking half is waived for the whole Architecture stage.** The Creator does not read that stage, so stopping to confirm would put them back in a loop it exists to keep them out of: agents carry on without asking and commit as they go. **And every sitting ends committed and pushed**, whether or not it closed a round — the same automatic act [Auditing](Auditing.md) step 7 already requires at a round close, extended to writing a document, running a gate, a sweep, or any other stretch of work. Creator-stated, 2026-08-20, after a sitting's whole round sat unpushed on one computer while a second agent elsewhere read the repository and correctly reported the work missing. **Work that exists on one machine is work nothing else the Creator owns can see**, and judging when to push is the agent's, never the Creator's. **The unhurried half is not waived** — nothing there is licence to rush. Four moments still reach the Creator and the [Architecture method](../Architecture/Method.md) sets them out: the three an agent stops dead for — the architecture cannot keep a promise a spec makes, a fact the promise cannot stand without turns out to be unanswerable, or the answer costs the user something no spec says they get — plus the runner's one batched request for a rented server. An ordinary architecture round opens when its counted readiness test passes and closes when its bounded close test passes; neither is a product ruling.

*Why:* the Creator sets the direction and pace; a quick check-in keeps the decisions theirs and avoids work that has to be undone.

---

**5. Sign your work.**

Every *meaningful* change an agent makes should show which agent made it, so authorship is always traceable. (What counts as meaningful, the exact format, and how code is signed through its commits instead, all live in the [Conventions document](Conventions.md).)

*Why:* many different models are used across many tools; knowing who produced what is essential for chasing down mistakes.

---

**6. Never describe the present moment in a document.**

Avoid anything that assumes when it's being read — "the stage happening now," "this doesn't exist yet." Word things so they stay true whenever they're read.

The one exception: a document or section explicitly marked as a **live tracker** — like the checklists in `Start Here`, or `Open Threads` — may describe changing state, because keeping it current is its entire job. Everything else stays timeless.

*Why:* documents are read months apart; anything tied to "now" turns false the moment that moment passes.

---

**7. Nothing gets built that isn't written down first.**

When the Creator proposes something new mid-flight — *"let's add…", "wouldn't it be cool if…", "quick change…"* — do not build it. Stop and ask one question: **is this fixing a hole, or adding a want?**

A **hole** means the plan is already broken, and was broken before anyone noticed. Fix the earlier document, record why, then carry the fix down through everything that depended on it.

A **want** means the plan is fine and a new idea has arrived. The default is to park it in [Open Threads](Open%20Threads.md) and carry on — most wants should die there, and that is the system working. If the Creator wants it now, it enters at the **highest stage it touches** — Specs if it changes what the user gets, Design if it changes only how the app looks or feels, Architecture if the user never sees it — and flows down from there. Even when that's a single sentence in a spec.

Never build first and document after.

If a change lands in a document that has already been audited, the change is not finished until a line has been added to that Subject's decisions record saying what was added and that no auditor has seen it. The agent writes that line, never the Creator.

Agents get ideas too. The same rule binds them: raise it, park it, never act on it.

*Why:* the Creator will forget this rule — they will be deep in a build with a good idea and no memory of the process. **You must not forget it. Stopping them is your job, not their job to remember.** Features welded straight into the app leave the documents describing a product that no longer exists — and that gap between plan and reality is what rots a project.

---

**8. Judge Smoozy by its promises, not by the user's mistakes.**

What Smoozy promises, and where its job stops, is written under *Where Smoozy's job stops* in the [Whole app spec](../Specs/Whole%20app.md). Read it before you call anything a fault. This binds every agent, not only auditors: it decides what goes into a spec, what a design has to cover, and what counts as a problem worth fixing.

Three things follow.

A problem the user brought on themselves is not a fault in the plan. Nobody owes them a rescue, and writing one in is not an improvement.

Smoozy telling the truth about what happened is never past the line. That promise holds no matter whose fault the trouble was, so a place where Smoozy goes quiet, or says something worked when it did not, is always a fault.

Something helpful past the line is a **want**, and Rule 7 already says what to do with a want. Raise it, say plainly that it is past the line, and park it. Never file it as a fault, and never build it.

*Why:* the user is a professional who knows their craft, and an app that checks their work for them is worse to use than one that lets them be wrong. Without this written down, every audit round invents safety nets nobody asked for, and the Creator has to turn them down one at a time.

---

**9. Anything the user reads is written in standard software words.**

This binds every word on a screen — labels, buttons, statuses, messages, tooltips — and every word in a Design document describing one. Label: **Creator-stated** (2026-08-11).

**Use the word the rest of the industry uses.** Downloaded, uploaded, cancelled, failed, retry, rendering, saved, deleted, connected, **CPU**, **GPU**. A 3D artist has used software for twenty years and already knows every one of them. Nothing is gained by inventing a fresh way to say *downloaded*.

**The specs say *processor* and *cards*; a screen says CPU and GPU.** That gap is the ordinary case rather than a mistake — a spec is written to be reasoned about, an interface to be read at speed — and it is named here because it is the pair that keeps slipping through.

**Never a metaphor, an idiom, or a phrase you thought sounded warm.** *Frames have come home*, *the first frame is back*, *work Smoozy is not watching* — every one of those makes the reader stop and work out what actually happened. **The test is whether a plain word would say it exactly**: if *downloaded* would have done, then *came home* is not a nicer way of saying it, it is a worse one.

**Vague is the real failure, not ugly.** *The first frame is back* does not say whether it rendered, downloaded, or arrived on a phone. A label that could mean three things is broken however pleasant it reads.

**Short, too. Added 2026-09-29, Creator-stated.** Every sentence on a screen is the shortest one that gets the point across. No flourish, no second sentence where one does, no word that carries nothing. *Download* beats *come down*, and *Download failed.* beats a sentence explaining that a download could not be completed.

**Rule 1 is not this rule.** Rule 1 governs how agents write to the Creator and to each other, and it bans code-speak. This one governs what the user reads, and it *requires* the standard term — which is often the technical one. Where they seem to collide, this one wins on a screen and Rule 1 wins everywhere else.

*Why:* a model reaching for a warm, human-sounding phrase is exactly what makes an interface read as written by a machine, and it costs the user a second of decoding every time. This rule decays the same way Rule 1 does: it holds for a screen or two and then drifts back into poetry, and the Creator has to catch it again. Catching it is your job, not theirs.

---

**10. From here on, Design conforms to Architecture — never the other way round.**

Creator-stated, 2026-08-23, in their own words: *"I don't want to reopen anything from either specs or architecture, so we need to write down a hard rule that from now on design has to conform to architecture — not the other way around."*

This reverses the order the stages were built in, deliberately, and only from this date. Design came before Architecture so that the technical structure would be built to serve the intended experience. **Specs and Architecture are now closed to Design.** Where a design and an architecture answer disagree, **the design changes, or the thing is dropped from the design.** There is no third ending: it does not go up as a hole, it does not become a want that enters at Specs, and it is not a reason to reopen an answered row.

**What this costs, stated plainly so nobody rediscovers it as a surprise.** The three modules that never got a design document — `Access`, `Rendering` and `Telegram` — had their architecture written with no design to serve. So wherever those answers imply something about what the user sees, that implication **is** the design now, whether or not anyone chose it.

**It also closes a pile that was already open.** [Open Threads](Open%20Threads.md) parks several drafted design sentences that the architecture bounds rather than supplies. Under this rule they resolve to the architecture's version. They need no ruling and must not be brought back as ones.

**What still stops.** This rule binds Design against the earlier stages and nothing else. Rule 7 is untouched: a genuinely new thing still enters at the highest stage it touches, and nothing here is licence to build something nobody wrote down.

*Why:* the earlier stages were finished at real cost, and every route from Design back up into them reopens rows that were answered, audited and counted. A project that can always go back up never closes.

---
