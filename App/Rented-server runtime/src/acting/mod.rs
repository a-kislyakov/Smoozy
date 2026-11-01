//! The half that reaches the machine: the real end of every reading and every
//! act the rules above are written over.
//!
//! # What this is
//!
//! The rest of this crate is judgement — a rule, a table, a total function over
//! a reading somebody else took. **Nothing there opens a socket, starts a
//! process, sends a signal, renames a file or runs a command.** This module is
//! where all of that happens, and it is the only place in the crate that does.
//!
//! **It runs on the rented Linux machine.** The supervisor, its machine-local
//! endpoint, every worker and the machine half of the package transaction all
//! live there, which `Preparation/Architecture/The shape.md` settles. Nothing
//! here reads the user's computer and no path, figure or capability of the
//! desktop appears anywhere in it.
//!
//! # Why it is gated to a unix
//!
//! Every act below is a unix act — a filesystem-named socket, a process
//! identifier paired with a kernel-set start time, a signal, an advisory lock,
//! a rename. **Windows has none of them and the machine Smoozy rents is never
//! Windows**, so the module is absent there rather than answered wrongly.
//! **macOS is a unix and is the desk**, which is what lets the endpoint, the
//! commit's rename, the worker start, the wait, the termination and the two
//! sinks be run for real at the desk rather than only reasoned about.
//!
//! **What the desk cannot exercise is named where it sits**: the machine kind's
//! two readings, the boot registration, the linger grant, the card query and the
//! machine's own running time are all Linux's, and each carries a way to be
//! pointed at a root of the caller's choosing so the reading itself can be
//! tested without the machine that would answer it.
//!
//! # Where the outside facts come from
//!
//! Rule 2 binds here exactly as everywhere else: a fact about the machine — an
//! operating system, a shell, a tool, a standard — comes from that thing's own
//! publication, quoted, with the address and the date read, or from an
//! observation this project actually made, labelled as one. **Every such fact
//! below carries its citation at the point of use.** The publications read for
//! this module were read on **2026-09-04**; the observations are the frozen
//! evidence documents' and are named where they are used.
//!
//! **One fact wanted here could not be got and is left as a question**, at
//! [`registration`]: the `--user` option's own description in `systemctl(1)`.
//! What stands in its place is stated there.
//!
//! # What is not here, and why
//!
//! [`crate::arriving_later`] is the record. **Every one of this crate's
//! twenty-one traits has its real end here**, the last three having landed on
//! 2026-09-04 when `Server protocol and transfers` §2's request framing was
//! built: the health answer at [`package`], the worker's report at [`workers`],
//! and V4's observed value at [`readings`]. **All three were waiting on a frame
//! rather than on a transport**, and two of them never touch SSH at all.
//!
//! **What is still not here is a supervisor loop.** [`endpoint`] answers one
//! caller at a time and the request set is handed in, because which requests a
//! supervisor serves is each owning document's rather than this module's.

pub mod capture;
pub mod durable;
pub mod endpoint;
pub mod execution_group;
pub mod journal;
pub mod machine;
pub mod machine_kind;
pub mod operation_records;
pub mod package;
pub mod readings;
pub mod registration;
pub mod starting;
pub mod stores;
pub mod workers;
