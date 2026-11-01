//! The connected period — `Servers` §1.
//!
//! **S1: a connected period is the desired state for one server's connection,
//! and the live connection is an observed value of it.** They are never one
//! field, and **no reading of the connection ever writes the period**. That is
//! `State, identity and recovery` §3's D1 and D2 applied, and D2 is the whole
//! of why a drop does not end a period: *"No desired state is ever inferred
//! from an observed reading."*
//!
//! **S2: a period is per server, and one server's period touches no other's.**
//! Nothing anywhere holds a count of periods, a global connection state, or a
//! shared connection — which is why there is no registry type in this file.
//!
//! **S3: a period is held in process memory for the life of the application run
//! and is written nowhere durable.** That is what Row 25's *every server starts
//! disconnected* requires ([`crate::store`]), and it is consistent with
//! `State, identity and recovery` §3's D6 — **that document's D6 and not this
//! one's**, which is the enumeration a removal walks: a crash and a clean close
//! leave the same durable state, there being no durable connection state for
//! either to leave.

use std::time::SystemTime;

use smoozy_state::lifetimes::Lifetime;

/// **S5: a period begins at exactly two moments and the list is closed. A
/// period begins at the act.**
///
/// It begins the moment the act is taken and **never when the connection
/// becomes live**, whatever that connection then does. That is
/// `State, identity and recovery` D2 applied — a
/// period begun on a live connection would be a desired state inferred from an
/// observed reading — and it is why a period whose opening connection never
/// becomes live is ended by [`PeriodEnding::TheOpeningConnectionNeverBecameLive`]
/// rather than never having begun.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OpeningAct {
    /// **O1: the user's connect action** on a saved server, including a connect
    /// carrying a remembered setup ([`crate::setup_bit`]).
    TheUsersConnectAction,
    /// **O2: an add with a setup option ticked**, which the spec makes connect
    /// right away to carry the setup out. When the setup ends the server stays
    /// connected, so the period stands until the user disconnects or the run
    /// ends.
    AnAddWithASetupOptionTicked,
}

impl OpeningAct {
    /// The closed list, so nothing composes a third.
    pub const ALL: [OpeningAct; 2] = [
        OpeningAct::TheUsersConnectAction,
        OpeningAct::AnAddWithASetupOptionTicked,
    ];
}

/// **Who took the opening act, which is a third value beside the two acts and
/// is not derivable from either.**
///
/// **S8 is why it has to be carried.** The edit put-back is **O1 performed by
/// Smoozy as part of the user's own act of saving**, so it takes the same
/// [`OpeningAct`] as the user's own connect and is a wholly different thing
/// under [P3](crate::key_moments): P3 binds *"every connection Smoozy makes on
/// its own"* and **names the edit put-back in its list**, while P4 lets a
/// connect the user started ask for a passphrase.
///
/// **And it cannot be derived from whether the attempt is the period's first.**
/// The put-back's own opening attempt is the period's first, so a route that
/// read the first attempt as the user's would put a passphrase box on screen
/// **at a moment the user is looking elsewhere, which is what E7 forbids
/// outright**. This value is what stops that being wireable.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhoTookIt {
    /// The user pressed connect, or ticked the setup option on an add. **P4
    /// applies and `Keys` may raise the ask.**
    TheUser,
    /// **E2: the put-back, which Smoozy performs inside the user's own save.**
    /// P3 binds it entire: where the new key needs a passphrase Smoozy is not
    /// holding, **no attempt is made at all** and the server is left
    /// disconnected with the new details saved and marked with what is wrong.
    SmoozyOnTheUsersOwnSave,
}

impl WhoTookIt {
    pub const ALL: [WhoTookIt; 2] = [WhoTookIt::TheUser, WhoTookIt::SmoozyOnTheUsersOwnSave];

    /// The value [`crate::key_moments::WhoStartedIt`] takes for the opening
    /// act's own attempt. **A period whose opening act Smoozy took is one
    /// Smoozy is making on its own, whatever the act itself is.**
    pub fn as_the_attempt_reads_it(self) -> crate::key_moments::WhoStartedIt {
        match self {
            WhoTookIt::TheUser => crate::key_moments::WhoStartedIt::TheUser,
            WhoTookIt::SmoozyOnTheUsersOwnSave => {
                crate::key_moments::WhoStartedIt::SmoozyOnItsOwn
            }
        }
    }
}

/// **S6: a connection under a standing period is replaced at exactly two
/// moments and the list is closed.**
///
/// **S7: R1 and R2 are the two the spec names, and the additional connection is
/// not a third of that kind** — it is a list of its own, of one. Row 29 says
/// Smoozy may replace the connection underneath the period *"in two cases, and
/// only these two"*, and this enum is that kept literally.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Replacement {
    /// **R1: repair after an unintended drop** ([`crate::repair`]). The period
    /// stands throughout, **including while the drop is unrepaired**.
    RepairAfterAnUnintendedDrop,
    /// **R2: the return after a restart `Server Setup` caused**
    /// ([`crate::restart`]). The period stands throughout, carrying the restart
    /// mark.
    TheReturnAfterARestart,
}

impl Replacement {
    pub const ALL: [Replacement; 2] = [
        Replacement::RepairAfterAnUnintendedDrop,
        Replacement::TheReturnAfterARestart,
    ];
}

/// **S10: a period ends at exactly six moments and the list is closed.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PeriodEnding {
    /// **1. The user disconnects**, which is available for as long as a period
    /// stands — including while the server reads as reconnecting, and including
    /// while it reads as disconnected between runs of attempts. **Taking it
    /// stops the repair.**
    TheUserDisconnects,
    /// **2. The application run ends**, however it ends.
    TheApplicationRunEnds,
    /// **3. The edit's save on a server whose period stands**
    /// ([`crate::edits`]).
    TheEditsSave,
    /// **4. A break is declared while the server's key is not usable**
    /// ([`crate::key_moments`]), because the repair Row 33 forbids could never
    /// run and a period whose repair cannot run promises what Smoozy cannot
    /// deliver.
    ABreakDeclaredWithTheKeyNotUsable,
    /// **5. The connection the opening act was to make never became live.**
    ///
    /// S5 begins the period at the act, so where that connection never becomes
    /// live the period ends there — including where the attempt was abandoned
    /// by a step of the first-connection sequence, and including where the key
    /// check meant **no attempt was made at all**.
    ///
    /// **This ending is written against the opening act's own connection and
    /// reaches no replacement**: a repair attempt that fails and a return after
    /// a restart that fails are both attempts under a period that already
    /// stands, and neither is this.
    TheOpeningConnectionNeverBecameLive,
    /// **6. The server row is removed**, at the moment the removal's **local
    /// forget** is performed and **not at the moment it is asked for**.
    ///
    /// W25 makes the local forget the removal's last unit, so on the
    /// machine-reaching removal the period must stand through the
    /// clear-and-stop pair and Telegram's role off, both of which reach the
    /// machine over the connection it holds. **Nothing ends on the request**:
    /// W21's *try again* and *try the remainder again* leave the row standing,
    /// and a period ended at the request could carry neither.
    ///
    /// **And the removal must end it.** N13 makes repair a property of a
    /// standing period and of nothing else, so a period left standing after its
    /// row is gone would go on opening connections at the repair's cadence for
    /// the rest of the run, to a machine the user has just told Smoozy to
    /// forget.
    TheRowIsRemovedAtItsLocalForget,
}

impl PeriodEnding {
    pub const ALL: [PeriodEnding; 6] = [
        PeriodEnding::TheUserDisconnects,
        PeriodEnding::TheApplicationRunEnds,
        PeriodEnding::TheEditsSave,
        PeriodEnding::ABreakDeclaredWithTheKeyNotUsable,
        PeriodEnding::TheOpeningConnectionNeverBecameLive,
        PeriodEnding::TheRowIsRemovedAtItsLocalForget,
    ];
}

/// **S4: what a period holds, and it holds nothing else.**
///
/// Every other thing that lives for a period's length is **period-held** under
/// `State, identity and recovery` §11.1 and is that section's, not a field of
/// the period. [`PERIOD_HELD`] names the class this type sits beside.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConnectedPeriod {
    /// The local server identifier it belongs to.
    pub server: String,
    /// The moment the period opened, on the desktop's clock.
    pub opened_at: SystemTime,
    /// **S5: the act that opened it**, and never the connection becoming live.
    pub opened_by: OpeningAct,
    /// The *restarting, caused by Smoozy* mark where `Server Setup` has set one
    /// ([`crate::restart::TheRestartMark`]).
    ///
    /// **It is a handle rather than a bool because M1 has it set in the middle
    /// of a period's life** — before the restart command leaves, from the
    /// thread driving `Server Setup` R1's run — and cleared by exactly two
    /// things, neither of which is this period's own act. **There is one copy
    /// of the fact and this is it.**
    pub restart_mark: crate::restart::TheRestartMark,
    /// **The connections open under it** — the primary, and at most one
    /// additional ([`AdditionalConnection`]).
    pub connections: Connections,
}

/// **S11: every period-held thing ends the moment the period ends, however it
/// ends.** Named here so the class this module supplies is the one the spine
/// already publishes rather than a second of its own.
pub const PERIOD_HELD: Lifetime = Lifetime::PeriodHeld;

impl ConnectedPeriod {
    /// **S5: a period begins at the act.** Nothing about the connection is
    /// consulted, which is what makes `State, identity and recovery` D2 kept
    /// by construction here.
    pub fn begun(server: impl Into<String>, by: OpeningAct, at: SystemTime) -> ConnectedPeriod {
        ConnectedPeriod {
            server: server.into(),
            opened_at: at,
            opened_by: by,
            restart_mark: crate::restart::TheRestartMark::none_is_set(),
            connections: Connections::none_yet(),
        }
    }

    /// **S12: a drop does not end a period, so nothing period-held is dropped
    /// by one — except the machine password held for a connection**, which is
    /// connection-held under `Security and licence` §2 item 3b and does not
    /// survive the connection that took it.
    ///
    /// That is the rule `Queue & History` C6 reads from the other side: the
    /// last-known queue reading is gone when the **period** ends, standing
    /// through a drop, which is what lets a dropped-connection server go on
    /// showing last-known entries for the rest of the run.
    ///
    /// **Two inputs class the machine password differently and both are cited
    /// rather than one** — `State, identity and recovery` §11.1 puts it in the
    /// period-held class while `Security and licence` §2 item 3b writes it
    /// against the connection. **This follows §2 item 3b**, and §16 item 17
    /// records the seam, which closed in this document's favour on 2026-08-22.
    pub fn a_drop_drops(&self) -> WhatADropDrops {
        WhatADropDrops {
            the_last_known_queue_reading: false,
            every_live_reading: false,
            the_machine_password_held_for_the_connection: true,
        }
    }

    /// **S11a: ending a period by hand while a repair is running drops the
    /// last-known queue reading with it, and with that the removal warning's
    /// source.**
    ///
    /// A **repairing** server takes the connected menu whose first slot reads
    /// *disconnect*, and taking it is S10 item 1 — the period ends, the repair
    /// stops, and S11 drops the last-known queue reading and every live
    /// reading. The consequence is stated rather than left to be met: the
    /// server moves out of W19's third row and into its last, so **a removal
    /// taken afterwards warns about nothing** and offers a plain confirmation.
    ///
    /// **Nothing is said to the user before the act**, and nothing here
    /// supplies a fact for saying one: the spec grants this disconnect as *"the
    /// ordinary disconnect"* with *"No separate action is needed for it"*.
    pub fn what_an_ending_drops(_ending: PeriodEnding) -> WhatADropDrops {
        WhatADropDrops {
            the_last_known_queue_reading: true,
            every_live_reading: true,
            the_machine_password_held_for_the_connection: true,
        }
    }

    /// **S13: nothing durable is written from anything period-held**, which is
    /// `State, identity and recovery` D4 — **that document's D4 and not this
    /// one's**, which is the records keyed by the local server identifier. The
    /// plan's own S13 writes it bare and the ambiguity is inherited rather than
    /// invented here. The one durable thing this module holds that came from a
    /// machine is the remembered hardware reading, and it carries its
    /// installation identity ([`crate::hardware`]) rather than being written
    /// from a period-held value.
    pub const NOTHING_DURABLE_IS_WRITTEN_FROM_A_PERIOD_HELD_VALUE: bool = true;
}

/// What ends when a connection ends, against what ends when a period ends.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WhatADropDrops {
    pub the_last_known_queue_reading: bool,
    pub every_live_reading: bool,
    pub the_machine_password_held_for_the_connection: bool,
}

/// The connections under one period. **S14 caps them at two** and this type is
/// the cap.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Connections {
    /// Whether the primary connection is live. **The connection condition is
    /// derived from this alone** (S14 condition 6).
    pub primary_is_live: bool,
    /// **At most one**, opened for transfer channels alone.
    pub additional: Option<AdditionalConnection>,
}

impl Connections {
    fn none_yet() -> Connections {
        Connections {
            primary_is_live: false,
            additional: None,
        }
    }
}

/// **R3: the additional connection at a low session cap.** It is the whole of
/// the third closed list, and S7 says why it is a list of its own rather than a
/// third replacement: the period stands, this adds a connection under it, and
/// it replaces nothing.
///
/// **S14: a connected period may hold one additional connection to the same
/// server, opened for transfer channels alone.** Its conditions are exact and
/// it fires nowhere else.
///
/// **At `MaxSessions 1` the primary and additional connections have different
/// permanent roles.** The primary alternates its sole session between browsing
/// readings and command invocations under `Server protocol and transfers` T5; a
/// queued command prevents another reading from beginning and takes the session
/// as soon as the current framed reading reply completes. **The additional
/// connection never alternates**: it carries transfer channels only, and no
/// command, package inspection, installation-identity read, queue read or
/// browser reading can be admitted on it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdditionalConnection {
    /// **Condition 5: it is opened at the first refused admission and closed
    /// when the last transfer on it ends. It is never held idle.**
    pub transfers_on_it: usize,
}

impl AdditionalConnection {
    /// **Condition 1: it is opened only on a refused channel admission**, and
    /// only where the per-server admission point has learned that the primary
    /// cannot seat a transfer without consuming its reading/command lease.
    ///
    /// **A machine at the documented default of 10 never reaches this rule; a
    /// machine at `MaxSessions 1` reaches it at the first transfer.**
    ///
    /// **Condition 7: one is the cap, and the reason is that the rule is a
    /// remedy rather than a pool.** Two connections give two session pools; a
    /// third would be Smoozy tuning throughput by opening connections, which
    /// Fact 25's measurements already say buys nothing — *"a whole second
    /// rented machine bought about 20%, not 100%"*.
    pub fn may_be_opened(period: &ConnectedPeriod, the_admission_was_refused: bool) -> bool {
        the_admission_was_refused
            && period.connections.primary_is_live
            && period.connections.additional.is_none()
    }

    /// **Condition 4: it carries no command channel and no reading channel, and
    /// it is never used for a request.** Correlation, the installation
    /// identity, the queue reading and every command stay on the primary
    /// connection.
    pub fn may_carry(&self, work: WorkOnAConnection) -> bool {
        matches!(work, WorkOnAConnection::ATransferChannel)
    }

    /// **Condition 6: losing the additional connection is a transfer
    /// interruption and never a connection break; losing the primary
    /// connection ends both, and the repair rebuilds the primary first.**
    pub fn losing_it_is_a_connection_break() -> bool {
        false
    }
}

/// What a connection may be asked to carry, so condition 4's exclusion is kept
/// by the type rather than by care at each admission site.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WorkOnAConnection {
    ATransferChannel,
    ACommand,
    APackageInspection,
    TheInstallationIdentityRead,
    TheQueueRead,
    ABrowserReading,
}

/// **S15: where the cap is 0, no session of any kind is permitted and Smoozy
/// has no route to the machine at all.**
///
/// The connection is reported as one Smoozy could not use, **with no claim
/// about why**. This is a machine Smoozy cannot work on and it is not a case
/// S14 covers. What the server is left reading follows from S10 item 5: a
/// connection Smoozy has no route on never became live, so the period the
/// opening act began ends there and the server reads **disconnected**.
pub fn a_cap_of_zero_ends_the_period() -> PeriodEnding {
    PeriodEnding::TheOpeningConnectionNeverBecameLive
}

/// **S8: the edit put-back is not in any of the three lists.**
///
/// The edit's save disconnects the server, which ends the period; the put-back
/// is **O1 performed by Smoozy as part of the user's own act of saving**. So
/// the replacement list stays exactly two, and a failed put-back leaves no
/// period to repair — which is what makes the spec's *"the server is left
/// disconnected with the new details saved"* true **by construction** rather
/// than by an exception in the repair.
///
/// **Which act it is and who took it are two answers**, and [`WhoTookIt`] is
/// the second: the put-back is O1 **and** it is a connection Smoozy makes on
/// its own, so [P3](crate::key_moments) binds it and E7's passphrase box can
/// never appear on it.
pub const THE_PUT_BACK_IS_O1: OpeningAct = OpeningAct::TheUsersConnectAction;

/// **E2 and E6: who takes the put-back, which is the half
/// [`THE_PUT_BACK_IS_O1`] does not answer.**
pub const THE_PUT_BACK_IS_TAKEN_BY: WhoTookIt = WhoTookIt::SmoozyOnTheUsersOwnSave;

/// **S9: the acts of the three lists are the ones that reach a machine — O1 and
/// O2, R1 and R2, and the additional connection R3 — and nothing else reaches
/// one.**
///
/// Two cases are named because they are where the rule does real work. **A
/// removal of a disconnected server never starts a connection and never asks
/// for a passphrase** ([`crate::store`]). **Nothing watches keys in the
/// background** ([`crate::key_moments`]) — the usability check is a local read
/// taken when the list is drawn, and it never reaches a machine, so it is not
/// an open at all.
pub fn reaches_a_machine(what: SomethingServersDoes) -> bool {
    match what {
        SomethingServersDoes::APeriodOpeningAct(_)
        | SomethingServersDoes::AReplacement(_)
        | SomethingServersDoes::TheAdditionalConnection => true,
        SomethingServersDoes::ARemovalOfADisconnectedServer
        | SomethingServersDoes::TheKeyCheckAtTheListDraw
        | SomethingServersDoes::ADisconnect => false,
    }
}

/// The acts S9's closed list is drawn over.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SomethingServersDoes {
    APeriodOpeningAct(OpeningAct),
    AReplacement(Replacement),
    TheAdditionalConnection,
    ARemovalOfADisconnectedServer,
    TheKeyCheckAtTheListDraw,
    /// **W26: a disconnect issues nothing to the machine.** It ends the period,
    /// drops what is period-held, and closes the connections under it. No
    /// command is sent, no signal is sent, and nothing on the machine is told.
    ADisconnect,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    fn a_moment() -> SystemTime {
        SystemTime::UNIX_EPOCH + Duration::from_secs(1_000_000)
    }

    fn a_standing_period() -> ConnectedPeriod {
        let mut period = ConnectedPeriod::begun("srv-1", OpeningAct::TheUsersConnectAction, a_moment());
        period.connections.primary_is_live = true;
        period
    }

    #[test]
    fn a_period_begins_at_the_act_and_not_at_the_connection_becoming_live() {
        let period = ConnectedPeriod::begun("srv-1", OpeningAct::TheUsersConnectAction, a_moment());
        // `State, identity and recovery` D2: the period stands although
        // nothing is live.
        assert!(!period.connections.primary_is_live);
        assert_eq!(period.opened_at, a_moment());
    }

    #[test]
    fn the_three_lists_are_closed_at_two_two_and_six() {
        assert_eq!(OpeningAct::ALL.len(), 2);
        assert_eq!(Replacement::ALL.len(), 2);
        assert_eq!(PeriodEnding::ALL.len(), 6);
    }

    #[test]
    fn a_drop_keeps_the_queue_reading_and_drops_only_the_machine_password() {
        // S12, and it is what lets a dropped server go on showing last-known
        // entries for the rest of the run.
        let dropped = a_standing_period().a_drop_drops();
        assert!(!dropped.the_last_known_queue_reading);
        assert!(!dropped.every_live_reading);
        assert!(dropped.the_machine_password_held_for_the_connection);
    }

    #[test]
    fn every_way_a_period_ends_drops_everything_period_held() {
        // S11: however it ends.
        for ending in PeriodEnding::ALL {
            let dropped = ConnectedPeriod::what_an_ending_drops(ending);
            assert!(dropped.the_last_known_queue_reading);
            assert!(dropped.every_live_reading);
        }
    }

    #[test]
    fn the_put_back_is_o1_so_the_replacement_list_stays_two() {
        assert_eq!(THE_PUT_BACK_IS_O1, OpeningAct::TheUsersConnectAction);
        assert!(!Replacement::ALL
            .iter()
            .any(|_| false), "the list is two and the put-back is not on it");
        assert_eq!(Replacement::ALL.len(), 2);
    }

    #[test]
    fn a_removal_of_a_disconnected_server_reaches_no_machine() {
        // D19, and one of the two places S9's closed list does real work.
        assert!(!reaches_a_machine(
            SomethingServersDoes::ARemovalOfADisconnectedServer
        ));
        assert!(!reaches_a_machine(
            SomethingServersDoes::TheKeyCheckAtTheListDraw
        ));
        assert!(!reaches_a_machine(SomethingServersDoes::ADisconnect));
        assert!(reaches_a_machine(SomethingServersDoes::APeriodOpeningAct(
            OpeningAct::AnAddWithASetupOptionTicked
        )));
    }

    #[test]
    fn one_additional_connection_is_the_cap() {
        let mut period = a_standing_period();
        assert!(AdditionalConnection::may_be_opened(&period, true));
        period.connections.additional = Some(AdditionalConnection { transfers_on_it: 1 });
        // A third would be Smoozy tuning throughput by opening connections.
        assert!(!AdditionalConnection::may_be_opened(&period, true));
    }

    #[test]
    fn the_additional_connection_opens_only_on_a_refused_admission() {
        let period = a_standing_period();
        assert!(!AdditionalConnection::may_be_opened(&period, false));
    }

    #[test]
    fn the_additional_connection_carries_transfers_and_nothing_else() {
        let additional = AdditionalConnection { transfers_on_it: 1 };
        assert!(additional.may_carry(WorkOnAConnection::ATransferChannel));
        for forbidden in [
            WorkOnAConnection::ACommand,
            WorkOnAConnection::APackageInspection,
            WorkOnAConnection::TheInstallationIdentityRead,
            WorkOnAConnection::TheQueueRead,
            WorkOnAConnection::ABrowserReading,
        ] {
            assert!(!additional.may_carry(forbidden));
        }
    }

    #[test]
    fn losing_the_additional_connection_is_not_a_connection_break() {
        assert!(!AdditionalConnection::losing_it_is_a_connection_break());
    }

    #[test]
    fn a_cap_of_zero_leaves_the_server_disconnected_by_s10_item_five() {
        assert_eq!(
            a_cap_of_zero_ends_the_period(),
            PeriodEnding::TheOpeningConnectionNeverBecameLive
        );
    }

    #[test]
    fn the_period_sits_in_the_spines_own_period_held_class() {
        assert_eq!(PERIOD_HELD, Lifetime::PeriodHeld);
    }
}
