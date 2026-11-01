# Rented-server runtime — program

**This module is in two folders and this is the program half.** The other is `App/Rented-server runtime/`, which holds the library. One plan governs both: `Preparation/Architecture/Rented-server runtime.md`, and its row file sits beside it under `Rows/`.

Here is `smoozy-server` — the one program a rented machine runs, and `The shape`'s three executable classes as three roles of it. The library beside it is what those roles run.

It was a binary inside the library's own package from step 8a and moved here at step 8b of `Preparation/Build/Plan.md`. **It moved so that it could reach the work it drives**: dependencies belong to a package rather than to a target, so a binary in the library could reach only what the library reaches, and every crate holding machine-side work depends on that library. A package nothing depends on can depend on all of them. The ruling is `Audits/Build/The machine program/Decisions record.md`.

**What must never come back into this package's tree is the SSH client and its C cryptography.** The machine is the far end of an SSH connection and never opens one.
