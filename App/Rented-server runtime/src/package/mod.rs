//! §2 — the package transaction.
//!
//! **The request's carriage, framing and idempotency key are `Server protocol
//! and transfers` §5's and are not restated.** What is here is the machine side:
//! the trusted release and the exact artifact ([`release`]), the durable phase
//! record and the one owner of the transaction ([`phase`]), the single deciding
//! write with its health exchange and rollback ([`commit`]), and the retry rule
//! with what blocks a commit ([`retry`]).
//!
//! **The pair problem Fact 78 names is removed rather than solved.** No single
//! operation updates two independent files atomically, so an arrangement whose
//! commit is two writes has to recover deterministically to one matching pair.
//! **This arrangement has one write.**

pub mod commit;
pub mod phase;
pub mod release;
pub mod retry;
