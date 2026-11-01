//! §4 — the machine-local endpoint: its existence and lifecycle, and the access
//! boundary implemented.
//!
//! **Taken as given from `Security and licence` §8**: the endpoint is a
//! filesystem object in the machine's own namespace, it **binds no address**, and
//! it is therefore outside the *no inbound listener* question entirely.
//!
//! **Markers used in this file that belong to another document, each with the
//! document it belongs to.** `Security and licence` §8 and its requirements 1, 2
//! and 3. `Server protocol and transfers` — its authentication boundary and its
//! entitlement rule. `State, identity and recovery` §1.2, §1.3 rule 5, §4.
//!
//! **A marker this plan also defines is never listed here and is written out
//! with its document at the site**, because a header list cannot tell the two
//! apart and a bare one there would resolve to a real and wrong clause.

/// **Z1. The endpoint is created by the supervisor at every start and is never a
/// store.**
///
/// `State, identity and recovery` §1.2 lists it as volatile for exactly that
/// reason. **Nothing about the machine's state is read from it, and nothing
/// survives across a supervisor run in it.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Endpoint {
    /// **Z2. Its path is fixed for the installation and does not vary with the
    /// package version**, and it is recorded in the package area at placement.
    ///
    /// **The stability across package versions is what puts both supervisors on
    /// one endpoint, which is what S3's interlock needs to act on at all**: two
    /// supervisors of two package versions binding two different objects would
    /// both succeed, and the machine would have two writers of the machine-local
    /// queue, the operation records, the event journal and the hold records at
    /// exactly the moment a runtime upgrade is in flight.
    pub recorded_path: String,
}

/// **Z5. The endpoint is not a credential and its path is not a secret**
/// (`Security and licence` §8), **so nothing here randomises it, hides it, or
/// treats knowledge of it as authority.**
///
/// The value is `()` because the clause is an absence: there is no randomising
/// function here and no secrecy anywhere on the path.
pub const THE_PATH_IS_NOT_A_SECRET: () = ();

/// **Z3. The containing directory is created by the placement, owned by the login
/// the supervisor runs as, and permits that login alone** (`Security and licence`
/// §8 requirement 1). **The endpoint object carries the same ownership and the
/// same permission.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OwnedByTheSupervisorsLoginAlone;

/// What a bind attempt gave, and **the refusal carries no information about
/// whether anything holds the endpoint.**
///
/// The same address-in-use result is given for a live holder **and for a stale
/// filesystem object left by a crashed run** — the specified local address is
/// already in use *"or the filesystem socket object already exists"* — so **the
/// bind's result cannot serve as the interlock** and Z12 always was it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BindAttempt {
    Bound,
    RefusedTheAddressIsInUse,
}

/// **Z12's probe result, and it is never a reading of presence.**
///
/// **A stale object left by a crashed run and a live endpoint held by a running
/// supervisor refuse a bind alike, and Z4 makes the stale case the ordinary one
/// the next start must clear** — so a start that treated every refusal as a live
/// holder would never start a supervisor after a crash, and one that treated
/// every refusal as stale would remove a live supervisor's endpoint and break
/// S3.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConnectProbe {
    /// **A connection accepted means S3 applies and this start ends without
    /// touching a store.**
    Accepted,
    /// **A connection refused means nothing holds it and Z2's second branch
    /// applies.**
    Refused,
}

/// What a start does after a refused bind.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhatTheStartDoes {
    /// The bind succeeded first time.
    ItIsBound,
    /// **Z2's second branch**: the supervisor removes the object at its own
    /// recorded path and binds again.
    RemoveTheObjectAtItsOwnRecordedPathAndBindAgain,
    /// **S3**: this start ends, without touching a store and without signalling
    /// anything.
    ThisStartEndsAnotherSupervisorHoldsIt,
}

/// **Z2 and Z12 together: the supervisor binds first and removes second, and
/// what separates the two refusals is an attempt to reach the endpoint as a
/// caller.**
///
/// **The interlock is the connect probe's own result and never a reading of
/// whether an object was there**, which is what keeps this inside `State,
/// identity and recovery` §1.3 rule 5.
///
/// **The exact mechanism by which a connection is refused is Build's and is
/// tentative in the plan**, and nothing above rests on which system error it
/// arrives as — which is why [`ConnectProbe`] carries two members and no error
/// value.
pub fn what_the_start_does(bind: BindAttempt, probe: Option<ConnectProbe>) -> WhatTheStartDoes {
    match bind {
        BindAttempt::Bound => WhatTheStartDoes::ItIsBound,
        BindAttempt::RefusedTheAddressIsInUse => match probe {
            Some(ConnectProbe::Accepted) => WhatTheStartDoes::ThisStartEndsAnotherSupervisorHoldsIt,
            // **A connection refused means nothing holds it**, so this is Z2's
            // second branch: the object at the recorded path is what a crashed
            // run left, and Z4 makes that the ordinary case rather than a corner
            // one.
            Some(ConnectProbe::Refused) => {
                WhatTheStartDoes::RemoveTheObjectAtItsOwnRecordedPathAndBindAgain
            }
            // A probe that could not be taken at all is not a third answer, and
            // it is not read as *stale*: removing a live supervisor's endpoint
            // is the failure S1 says makes every rule here a hope, so the safe
            // reading is that something holds it.
            None => WhatTheStartDoes::ThisStartEndsAnotherSupervisorHoldsIt,
        },
    }
}

/// **Z4. It ends when the supervisor's run ends, and a run that ended without
/// removing it leaves an object Z2's second attempt removes.**
///
/// **No sweep and no schedule is involved, and no reading of presence**: what Z2
/// acts on is a refused bind, not an object it found.
pub const NOTHING_SWEEPS_THE_ENDPOINT_OBJECT: () = ();

/// **Z2b. The remove-and-rebind is racy between two starts, and nothing else in
/// this document names it.**
///
/// Each step is an ordinary operation: removing a **live** holder's path succeeds
/// and leaves the holder listening; a second socket then binds that same path
/// while the first is still listening; new callers reach the second while the
/// first is alive and unreachable. **Every step was run** — a caller reached the
/// holder before the removal, the removal returned no error, the holder's own
/// name still read the same path afterwards while a caller on that path failed
/// as though nothing were there, a second socket bound the same path, and three
/// fresh callers were accepted by the **second** and by neither the first.
///
/// **So two starts running Z12 at once both bind, both are refused, both probe,
/// both are refused, both conclude stale, the first removes and binds and is
/// live, and the second then removes the first's live object and binds its own**
/// — two supervisors, the condition S1 says makes every rule in this document a
/// rule rather than a hope.
///
/// **What bounds it is the restarters rather than anything here** — one unit on a
/// systemd machine, one keeper on a container, and G25 ends before it starts —
/// **and this document establishes no bound.** §16 item 19.
///
/// **Two standard arrangements would remove it and both carry a cost this
/// document does not pay today**: an advisory lock, and the abstract namespace,
/// which has no object to go stale but is Linux-only and has no owner and no
/// permission, which would take away `Security and licence` §8 requirement 1.
/// **Neither is chosen here**, and this constant is the record of that rather
/// than a mechanism.
pub const THE_REMOVE_AND_REBIND_RACE_IS_UNBOUNDED_HERE: bool = true;

/// **Z6. The supervisor establishes the caller's local identity from the accepted
/// connection itself, by peer credentials read from the kernel, and refuses any
/// caller whose identity is not the login it runs as.**
///
/// Fact 84 answers the mechanism on both machine kinds and both halves: a server
/// was told the caller's process, user and group identifiers for a caller running
/// as an ordinary login, **unchanged under a container's reduced capability set
/// and identically on a virtual machine**, with the endpoint deliberately left
/// world-writable **so what was observed was credentials rather than file
/// permissions.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PeerCredentials {
    /// **Z10. The recorded identifier the kernel reports is a stale handle the
    /// instant the caller finishes** — observed: by the time the server looked up
    /// the identifier it had just been told, the caller had exited and its entry
    /// was gone. **So nothing here reads anything further about a caller by its
    /// identifier**, and the identity established at Z6 is the whole of what is
    /// used.
    pub process: u32,
    pub user: u32,
    pub group: u32,
}

/// What the boundary does with a caller.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Admission {
    Admitted,
    /// **Z9. The refusal happens before the request is framed.** A refused caller
    /// **receives nothing that describes the protocol**, and the refusal is
    /// journalled as an event rather than answered as a request result
    /// (`State, identity and recovery` §4).
    RefusedBeforeTheRequestIsFramed,
}

/// **Z6.** The whole of the check.
///
/// **Z7. The endpoint cannot be lied to, and that is why the check is the second
/// layer rather than a courtesy.** An unprivileged caller attaching a claim of
/// the superuser's identity was **refused outright by the kernel**, as was
/// claiming another process's identifier, and a datagram attaching nothing still
/// arrived carrying the caller's true identity.
pub fn admit(credentials: PeerCredentials, the_login_the_supervisor_runs_as: u32) -> Admission {
    if credentials.user == the_login_the_supervisor_runs_as {
        Admission::Admitted
    } else {
        Admission::RefusedBeforeTheRequestIsFramed
    }
}

/// **Z8. No request field names its caller, and any such field is ignored rather
/// than checked** (`Server protocol and transfers` A2, `Security and licence` §8
/// requirement 2).
///
/// **Ignoring and checking are the same thing for security and only one of them
/// is safe to write down.** [`admit`] takes the kernel's credentials and nothing
/// from the request, and that shape is the clause.
pub const NO_REQUEST_FIELD_NAMES_ITS_CALLER: () = ();

/// **Z11. This boundary separates one login from another on a machine and does
/// not separate Smoozy from the machine's owner.**
///
/// `Security and licence` §8 states the limit — *"Root ignores file
/// permissions"*, and **on some rentals the login Smoozy holds is root** — and
/// nothing here is built on top of it as though it were more.
pub const THIS_BOUNDARY_DOES_NOT_SEPARATE_SMOOZY_FROM_THE_MACHINES_OWNER: bool = true;

/// Binding, probing, taking the next caller and reading its peer credentials,
/// behind one trait — **each is an act and the rules above are what they are
/// for.**
pub trait DrivesTheEndpoint {
    /// **The accepted connection, as one value carried from the accept through
    /// the credential check, the read and the answer.**
    ///
    /// **It is handed back rather than parked in the endpoint**, because Z6's
    /// boundary is a decision about *this* connection: a second accept landing
    /// in a shared slot between one caller's decision and its read would have
    /// the first request answered on a connection nobody checked.
    type AcceptedCaller;

    /// **Z2.** Attempt the bind at the recorded path.
    fn bind_at(&self, path: &str) -> Result<BindAttempt, String>;
    /// **Z12.** Connect to the endpoint at that path **as an ordinary caller**.
    /// A probe that cannot be taken at all returns an error, which
    /// [`what_the_start_does`] reads in the safe direction.
    fn probe_as_a_caller(&self, path: &str) -> Result<ConnectProbe, String>;
    /// **Z2's second branch.** Remove the object at the supervisor's **own**
    /// recorded path. The path is not a parameter, because a remove aimed
    /// anywhere else is not this clause.
    fn remove_its_own_object(&self) -> Result<(), String>;
    /// **Take the next caller, inside the caller's own deadline.**
    ///
    /// **The bound is an argument and never a number chosen here**, which is
    /// K17: a caller waits on its own deadline and never on a timer that assumed
    /// a number. A wait with no bound would park a supervisor nobody calls for
    /// ever.
    fn take_the_next_caller(
        &self,
        within: std::time::Duration,
    ) -> Result<Self::AcceptedCaller, String>;
    /// **Z6.** The credentials of that accepted connection, read from the kernel.
    fn peer_credentials(&self, caller: &Self::AcceptedCaller) -> Result<PeerCredentials, String>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_successful_bind_needs_no_probe() {
        assert_eq!(
            what_the_start_does(BindAttempt::Bound, None),
            WhatTheStartDoes::ItIsBound
        );
    }

    #[test]
    fn a_refused_bind_with_an_accepted_probe_ends_this_start() {
        assert_eq!(
            what_the_start_does(
                BindAttempt::RefusedTheAddressIsInUse,
                Some(ConnectProbe::Accepted)
            ),
            WhatTheStartDoes::ThisStartEndsAnotherSupervisorHoldsIt
        );
    }

    #[test]
    fn a_refused_bind_with_a_refused_probe_clears_the_stale_object() {
        assert_eq!(
            what_the_start_does(
                BindAttempt::RefusedTheAddressIsInUse,
                Some(ConnectProbe::Refused)
            ),
            WhatTheStartDoes::RemoveTheObjectAtItsOwnRecordedPathAndBindAgain
        );
    }

    /// A probe that could not be taken is never read as *stale*: removing a live
    /// supervisor's endpoint is the failure S1 says makes every rule a hope.
    #[test]
    fn a_probe_that_could_not_be_taken_ends_this_start_rather_than_removing_anything() {
        assert_eq!(
            what_the_start_does(BindAttempt::RefusedTheAddressIsInUse, None),
            WhatTheStartDoes::ThisStartEndsAnotherSupervisorHoldsIt
        );
    }

    #[test]
    fn a_caller_of_another_login_is_refused_before_the_request_is_framed() {
        let another_login = PeerCredentials {
            process: 1628,
            user: 1000,
            group: 1000,
        };
        assert_eq!(admit(another_login, 1001), Admission::RefusedBeforeTheRequestIsFramed);
        assert_eq!(admit(another_login, 1000), Admission::Admitted);
    }

    /// The superuser is not exempt from Z6: the check is *is it the login the
    /// supervisor runs as*, not *is it privileged*. On a rental where Smoozy's
    /// own login is root, that login is admitted because it is the same login —
    /// which is Z11's stated limit rather than a hole here.
    #[test]
    fn the_superuser_is_admitted_only_where_it_is_the_login_the_supervisor_runs_as() {
        let root = PeerCredentials {
            process: 1,
            user: 0,
            group: 0,
        };
        assert_eq!(admit(root, 1001), Admission::RefusedBeforeTheRequestIsFramed);
        assert_eq!(admit(root, 0), Admission::Admitted);
    }
}
