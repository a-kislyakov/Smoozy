# Scenarios

This folder holds the **scenarios** — one person doing one piece of real work with the app, written out step by step so it can be played through the project's documents.

What a scenario is and how one is audited is set out in **`../Groundwork/Auditing.md`**.

## Why there are so few

The folder held eighteen on 2026-08-08 and now holds eight. The Creator cut it that day, and the reasoning is worth keeping, because the folder will grow again and the same mistakes are easy to repeat. Label: **Creator-stated** (2026-08-08).

**A scenario round is expensive and the project has to ship.** Every one costs five auditors, a consolidator, an implementer and a set of rulings. Preparing for everything is not a goal; the point is to find the faults nothing else finds.

**Most of what was cut was the same story wearing different clothes.** Five of them were a transfer stopping partway and the user wanting to carry on — the connection dropping, the app closing, a disk filling, at one end or the other. Three were the user going away during a render and coming back. Different causes, one question, and the same few paragraphs answering all of them. Where several stories put the same question to the same documents, one of them is a scenario and the rest are noise.

**That whole shape mostly yields the weakest kind of finding.** A single shock, then the user looks at the screen, produces *nothing says what happens* — which a plain document audit already catches, more cheaply and more thoroughly. What earns a scenario round is the other kind: every document is written, they all agree, and what they agree on is bad for the artist. Prefer a story where the user is *doing* something over one where something is *done to them*.

**One story was retired outright as unplayable**, and four more were found describing an app this project is not building. All five had the user click to start a render and Smoozy then upload the file. See `Audits/Specs/The connection drops while uploading/What happened.md`, which cost five auditors to establish.

**So a scenario is checked against the specs as it is written, and that is where the check belongs.** Every step has to be something Smoozy actually does, and every step has to be worded so that two auditors would play it the same way — an answer left unstated is an answer each of them invents. Use the project's own words for what the user does: a job waiting its turn is **cancelled** and a job that is rendering is **stopped**, and they leave different things behind. Do not write in what the machine or Blender does about it, only what the user does and what happens to them — a step asserting an outcome hands the auditors the answer and, where the project has not established that outcome, hands them a wrong one. The **runner does not do this check**: by the time a round opens it is too late and it is the wrong person. Label: **Creator-stated** (2026-08-08).

**They were written badly in the first place**, and that is the root of all of it: eighteen were produced by one model in one sitting. `../Groundwork/Auditing.md` says writing a scenario is the Creator's job with an agent helping, because it rests on knowing what real work looks like. Written the other way round, they come out as generic app playbooks.

## What is here now

Five are unaudited and waiting to be run:

- **The user adds a texture and renders again** — the loop the whole app is built around, failing quietly at full price.
- **The user renders the same project on a new machine** — rent, use, destroy, rent again a week later.
- **The provider reboots the machine** — the one interruption story where the render truly dies.
- **The server disk fills up during render** — the machine failing in the middle of a job.
- **The user stops a running job** — the only one where the user changes their mind.

Three have been played at the Specs stage already: **The connection drops while rendering**, **The machine is destroyed**, and **The user queues multiple jobs overnight** — the last of which cannot be replayed at a later stage, for the reason recorded in its audit folder.

The transfer behaviour the cut stories were reaching for is better checked as a **thread seam** — one round, every document that says what an interrupted transfer does, read against each other. It is cheaper and it is the right tool for that shape.
