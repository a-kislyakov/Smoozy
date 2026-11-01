//! The seams the acting half stands on.
//!
//! **Five things this module does not own and reaches through a trait**: what
//! opens a connection, what waits, what supplies a credential, what reads the
//! machine's installation identity, and where the values this document supplies
//! are published. **Everything else in the acting half is behaviour**, and every
//! bit of it is exercised at the desk against these.
//!
//! **That is the same split steps 2, 3 and 4 have all used** — `Keys` put its
//! readings behind one trait, [`crate::readings`] put the command channel
//! behind another — and the reason is unchanged: a judgement that can only be
//! reached through a socket is a judgement nobody tests.
//!
//! **The real implementations are [`crate::wiring`]**, which is the only place
//! in this module that opens a socket or waits on a clock.
//!
//! **Markers used below that belong to another document, each with the
//! document it belongs to.** `Server protocol and transfers` C16.
//! `Desktop app and delivery` T13.

use std::future::Future;
use std::time::{Duration, SystemTime};

use smoozy_server_protocol::transport::{
    Credential, DecideHostTrust, Ending, Endpoint, OpenFailure,
};
use smoozy_state::identities::InstallationCheck;

use crate::condition::ObservedValue;
use crate::identity::KeyReference;
use crate::key_moments::{TheTwoAnswers, WhoStartedIt};
use crate::readings::CommandChannel;
use crate::repair::{NextAttemptDue, PermanentRefusal, WhatTheRecordSays};
use crate::store::RememberedHostKey;

/// **What opens a connection — K1's transport, reached as a seam.**
///
/// **K1a is kept on the far side of it**: the real implementation drives the SSH
/// transport inside Smoozy's own process and runs no `ssh`, `sftp` or `scp` at
/// any moment. Nothing here hands a client an option, because there is no
/// client.
pub trait OpenConnections {
    type Connection: CommandChannel + crate::readings::TakesAFreeSpaceReading + Send;

    /// One attempt. **N2's connect deadline is carried here rather than left to
    /// the transport's own patience** — it is a waiter's deadline under `Server
    /// protocol and transfers` C16 and **changes no durable state when it
    /// expires**.
    fn open(
        &mut self,
        endpoint: Endpoint,
        credential: Credential,
        decide: Box<dyn DecideHostTrust>,
        deadline: Duration,
    ) -> impl Future<Output = Result<Self::Connection, OpenFailure>> + Send;

    /// **K10: no request is made to learn whether the connection is alive.**
    /// This reads a value the transport reported on its own.
    fn ended(&mut self, connection: &Self::Connection) -> Option<Ending>;

    /// **§8.5's requirement: a break is reported without the caller having to
    /// use the connection first.** This waits; it sends nothing.
    fn wait_until_it_ends(
        &mut self,
        connection: &mut Self::Connection,
    ) -> impl Future<Output = Ending> + Send;

    /// **K5: a break declared is a break, and the connection under it is torn
    /// down rather than kept for later.** Fact 57's ten-minute silent gap left a
    /// connection unrecoverable even after the path was restored, so repair
    /// always builds a new one.
    fn tear_down(&mut self, connection: Self::Connection) -> impl Future<Output = Ending> + Send;
}

/// **What waits, and what the moment is.**
///
/// Every pause in this module is one of the repair's own two figures, so a test
/// drives the whole of a run and the whole of a standing repair without waiting
/// for any of them.
pub trait Wait {
    fn until(&mut self, how_long: Duration) -> impl Future<Output = ()> + Send;

    /// The desktop's own clock. **N2a's run span and N10a's reading are both
    /// measured on it**, and nothing here is measured on a machine's.
    fn now(&mut self) -> SystemTime;
}

/// **P1's two questions and P4a's ask, both of which are `Keys`'.**
///
/// **This document neither holds a passphrase nor asks for one**: what crosses
/// at the ask is three display-only fields of the row beside P1a's pair, and
/// what comes back is a credential for **one attempt**. No module receives a
/// plaintext.
pub trait SupplyTheCredential {
    /// **P1 and P2: asked at the moment of the attempt and never from a cached
    /// value.** Row 46's mark is computed at each list draw and a key can go
    /// away between a draw and a connect.
    fn the_two_answers(&mut self, key: &KeyReference) -> TheTwoAnswers;

    /// **P4: a connect the user started may ask; a connection Smoozy makes on
    /// its own may not.** Where nothing can be supplied without asking and
    /// nobody may be asked, this answers nothing and P3 means no attempt is
    /// made.
    fn for_one_attempt(
        &mut self,
        key: &KeyReference,
        who: WhoStartedIt,
        naming: TheServerBeingConnectedTo<'_>,
    ) -> Option<Credential>;

    /// **N8b: whether a *no* to the second question is permanent.**
    ///
    /// It is permanent where `Security and licence` §3.3's store standing reads
    /// **never answered** and the row's key is protected — **a property of the
    /// installation and not of the moment**, so no run of attempts can ever
    /// change it. **Neither half is derivable from [`TheTwoAnswers`]**, which is
    /// why this is asked rather than composed: a repair that guessed at it would
    /// either retry for ever against an answer that cannot change, or stop for
    /// good on one that can.
    fn the_refusal_is_permanent(&mut self, key: &KeyReference) -> PermanentRefusal;
}

/// **P4a: what this document hands `Keys` at the ask, and nothing else
/// crosses.** Three display-only fields, so the box can name the server being
/// connected to. **No secret crosses at all.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TheServerBeingConnectedTo<'a> {
    pub label: Option<&'a str>,
    pub username: &'a str,
    pub host: &'a str,
}

/// **Step 5's read, and it is not this document's.**
///
/// §2.3's check has two halves — the value read from Smoozy's own state area,
/// and the supervisor answering to confirm it — and **both belong to modules
/// built at step 6** of `Preparation/Build/Plan.md`. What `Servers` owns is
/// where the read sits in the sequence and what each of the four branches does,
/// which is [`crate::sequence`] and [`crate::hardware`].
pub trait TheInstallationIdentityRead<C> {
    fn read(&mut self, channel: &mut C) -> impl Future<Output = TheIdentityRead> + Send;
}

/// What step 5 came back with.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TheIdentityRead {
    /// §2.3's four outcomes, **and they are the complete set**.
    pub check: InstallationCheck,
    /// **H17: the installation identity present on the machine when the
    /// reading was taken, or the explicit absence of one.** §2.3 requires the
    /// carrying; **the explicit absence is what makes the rule work for the
    /// ordinary case of a machine Smoozy has not prepared.**
    pub identity_on_the_machine: Option<String>,
}

/// **K7: `Servers` supplies the observed value and the transitions, and
/// presents neither.** This is where they go.
///
/// **Nothing here draws anything**, and what each value is called on a screen is
/// Design's. `Desktop app and delivery` T13 presents the condition record and
/// raises the could-not-restore notification **from that record and not from a
/// request failing**, which is kept by the value moving on the transport's own
/// report rather than on any request's outcome.
pub trait WhatServersPublishes {
    /// K6's three, as they move.
    fn the_observed_value(&mut self, server: &str, value: ObservedValue);

    /// **N10a: *the moment the next attempt is due*, or absent** — and absent is
    /// a value rather than a gap.
    fn the_next_attempt_due(&mut self, server: &str, reading: NextAttemptDue);

    /// **N5: raised at the run's failure and never at the break declaration.**
    fn raise_the_connection_condition(&mut self, server: &str, says: WhatTheRecordSays);

    /// **N10: withdrawn by the observed value reaching *connected***, which is
    /// §8.2's derivation and needs no act of this document's.
    fn withdraw_the_connection_condition(&mut self, server: &str);

    /// **Q6's third path.** The user decides; Q17 bounds how often it is raised
    /// under a standing repair.
    fn the_changed_identity_warning(&mut self, server: &str);

    /// **D8's one durable write of this document's** — the scanned key recorded
    /// against the server on Q6's first path, and the replacement on an accept.
    /// **D9 bars every other destination**: it is never shown, never logged, and
    /// never handed to another module.
    fn record_the_scanned_host_key(&mut self, server: &str, key: RememberedHostKey);
}

/// The five seams as one value, so the sequence and the repair take one
/// argument rather than five and nothing can be wired up half way.
pub struct TheActingHalf<O, K, I, P, W> {
    pub opener: O,
    pub credentials: K,
    pub installation: I,
    pub publishes: P,
    pub wait: W,
}
