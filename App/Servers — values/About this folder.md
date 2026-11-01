# Servers — values

**This module is in two folders and this is the half the rented machine's own program may take.** The other is `App/Servers/`, which holds every act. One plan governs both: `Preparation/Architecture/Servers.md`.

Here are the hardware reading's values and the judgements over them, and the keepalive figures. Nothing here opens a connection, reads a disk or runs a process.

**Why the module is in two folders.** The program a rented machine runs must never carry the SSH client or its C cryptography. The render worker needs a machine's cards and the setup worker needs the keepalive interval, and while both sat in `App/Servers/` taking them meant taking that client too. Split at step 10c piece 3 of `Preparation/Build/Plan.md`; `App/Servers/` re-exports both at the paths they always had. The account is at `Audits/Build/Servers/Decisions record.md`.
