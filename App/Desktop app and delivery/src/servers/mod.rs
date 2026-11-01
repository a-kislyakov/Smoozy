//! The desktop application's real ends of `Servers`' seams.
//!
//! **`Servers` reaches five things through a trait** and owns none of them:
//! what opens a connection, what waits, what supplies a credential, what reads
//! the machine's installation identity, and where the values that module
//! supplies are published. **The first two are that crate's own** and are
//! built at its `wiring`. **The credential is `Keys`'.** **What is here is the
//! rest**, and both of them are built.
//!
//! **Nothing here decides anything `Servers` decides.** Each file is a
//! translation: it asks the machine, it reads the local record, and it hands
//! back the value that module's own judgement takes.
//!
//! **Two of the five have their real ends here now.** [`installation`] is the
//! machine's installation identity read. [`published`] is where the values that
//! module supplies go — the observed value, the moment the next attempt is due,
//! the connection condition raised and withdrawn, the changed-identity warning,
//! and the one durable write of the scanned host key.
//!
//! **Two more files here are not seams of that module's at all.**
//! [`a_period`] is where a connected period runs and what the run holds about
//! one — the application's side of `Servers` §1, §3 and §4 rather than a
//! judgement of theirs. [`live_reading`] is this crate's own plan's P10d, the
//! one bounded repeating read a standing period carries; it is declared in
//! `Desktop app and delivery` §3.3 and belongs to no other module.
//!
//! **And [`sessions`] is the per-server point** that `Server protocol and
//! transfers` T5 puts every session on a connection through — the reading
//! session, one command at a time, and the transfer channels, with `Servers`
//! S14's additional connection behind them — and the one general way to ask
//! the machine's supervisor something over it. A period's connection is lent
//! from there and from nowhere else.
//!
//! **And [`the_machines_work`] is `Servers` Q1 step 7**, which that module's
//! own sequence leaves unbuilt and hands to whoever drives it: `State, identity
//! and recovery` V1 steps 2 to 5 on the desktop, the queue reading and the
//! adoption offer, and then P10d's sequence started with its three reads given
//! their ends.
//!
//! **And [`stop_and_clear`] is the desktop's half of `Queue & History` L13 to
//! `Queue & History` L17** — the one sender of the stop-and-clear pair, its §9.4 record and its
//! wait — which the removal's *Stop and clear first* and `Desktop app and
//! delivery` V15's *Stop jobs and update* both go through. Step 10c's S2.
//!
//! **And [`the_queues_own_acts`] is the queue's own *Stop*, *Cancel* and a
//! group's *Cancel all*** — `Server protocol and transfers` C21's single stop,
//! `Queue & History` L1's cancel, and `Queue & History` L19's group form sent
//! through [`stop_and_clear`]'s one sender. Step 10c's S3.
//!
//! **And [`the_endings_posted`] and [`the_idle_recheck`] are what a connected
//! period tells the operating system** — a job's ending read while connected
//! (`Desktop app and delivery` T18), and `Queue & History` W4's re-check of a
//! server with nothing to do, `Desktop app and delivery` P10e's one instance.
//! The 38's piece 1.
//!
//! **And [`the_words_kept`] is `Queue & History` E9's fetch** — both captured
//! streams of every attempt, taken into the history record by the connection
//! that learns a job's ending. The Creator's notes on 0.1.8, piece 6d.
//!
//! **And [`the_driver_swap`] is `Server Setup` R1b on the desktop** — what the
//! comparison of the driver against the newest Blender's OptiX bar concluded,
//! held for the connected period; the live set and this desktop's transfers
//! with each machine; and the two moments an owed swap starts at. Added
//! 2026-10-12.
//!
//! **How a clause marker is written in this crate, and it is a rule rather than
//! a habit.** A marker with no document beside it means a clause of this
//! crate's own plan, `Preparation/Architecture/Desktop app and delivery.md`.
//! Every other document's marker is written with that document — at the site,
//! or in that file's own header list. **Where a marker is a clause of this
//! crate's own plan *and* of the document meant, the header list is not enough
//! and the document is written out at the site**, because a bare one there
//! resolves to a real and wrong clause.

pub mod a_period;
pub mod installation;
pub mod live_reading;
pub mod published;
pub mod sessions;
pub mod stop_and_clear;
pub mod the_driver_swap;
pub mod the_endings_posted;
pub mod the_idle_recheck;
pub mod the_machines_work;
pub mod the_queues_own_acts;
pub mod the_words_kept;
