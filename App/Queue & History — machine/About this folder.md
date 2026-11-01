# Queue & History — machine

**This module is in two folders and this is the half the rented machine's own program may take.** The other is `App/Queue & History/`, which holds every desktop act, the local job record, the bars, the catch-up, the idle half hour, deletion and the diary. One plan governs both: `Preparation/Architecture/Queue & History.md`.

Here are the line and the job record, how an ending is decided and what the entry says about the devices, the readings the render worker's copy loop produces and the reader that produces them, the waiting predicate, the previews' prefix arithmetic and their queueing, the acts on the machine-side record, and the machine's own stores and acts: the machine-local queue, the per-job note, the command and act records, the held jobs, reconciliation, the machine readings and the words. Nothing here opens a connection. The stores it writes are the rented machine's own.

**Why the module is in two folders.** The program a rented machine runs must never carry the SSH client or its C cryptography, and `App/Queue & History/` reaches both. Split at step 10c piece 3 of `Preparation/Build/Plan.md`; `App/Queue & History/` re-exports everything here at the paths it always had. The account is at `Audits/Build/Queue & History/Decisions record.md`.
