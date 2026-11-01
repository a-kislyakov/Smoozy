# Server protocol and transfers

This module's code. Its plan is `Preparation/Architecture/Server protocol and transfers.md`, and its row file sits beside it under `Rows/`.

It is built at step 4 of `Preparation/Build/Plan.md`.

**This folder is half of the module and `App/Server protocol and transfers — framing/` is the other half.** This half is every act of connecting: the SSH transport and the command channel, the file-transfer subsystem, the planted-link carriage, the free-space reading's two routes, and the whole staging, integrity, resume and publication contract. That half is what both ends speak, and it opens nothing. **Everything that moved there is re-exported from here at the path it has always been reached at**, so nothing outside the module changed with the split. Why the line falls where it does is in that folder's own note and in this module's decisions record; the split was made at step 8a.
