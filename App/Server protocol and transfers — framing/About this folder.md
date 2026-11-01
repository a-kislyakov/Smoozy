# Server protocol and transfers — framing

**This folder is not a module. It is half of one**, and the other half is `App/Server protocol and transfers/`. Both are governed by the same plan, `Preparation/Architecture/Server protocol and transfers.md`, and its row file sits beside that plan under `Rows/`.

**Why the module is in two folders.** What sits here is what both ends of the connection speak — a request's frame, the two versions that travel with it, and the shapes a result carries. What sits next door is everything that opens, holds and drives a connection. The program the rented machine runs needs the first and never the second: it is the far end of an SSH connection and never opens one. While the two were one crate, that program carried the desktop's SSH client and that client's C cryptography, which cost weight in the artifact the desktop uploads to every rented machine and made a Linux build impossible at the Creator's desk.

The split was made at step 8a of `Preparation/Build/Plan.md`; the module was built at step 4. What moved, why the line falls where it does, and what the dependency tree looks like either side are in `Audits/Build/Server protocol and transfers/Decisions record.md`.
