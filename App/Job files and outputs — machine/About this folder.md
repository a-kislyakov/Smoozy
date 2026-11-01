# Job files and outputs — machine

**This module is in two folders and this is the half the rented machine's own program may take.** The other is `App/Job files and outputs/`, which holds every act on the file-transfer subsystem. One plan governs both: `Preparation/Architecture/Job files and outputs.md`.

Here are the layout and the names under it, the submission manifest and the attempt record, the recorded reading, the map, the inventory, the main output's walk, the redirect, the reckoning, the root choice, the holds and the units, and the three paths a job's publication uses. Nothing here opens a connection, reads a disk or runs a process.

**Why the module is in two folders.** The program a rented machine runs must never carry the SSH client or its C cryptography, and the render worker, the queue and the reckoning all read what is here. **The acts could not come with it**: the seam over the file-transfer subsystem is implemented next door for that crate's own connection type, and the language allows that only in a crate that defines the seam or the type. Split at step 10c piece 3 of `Preparation/Build/Plan.md`; `App/Job files and outputs/` re-exports everything here at the paths it always had. The account is at `Audits/Build/Job files and outputs/Decisions record.md`.
