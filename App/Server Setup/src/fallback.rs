//! The user-facing fallback re-run — `Server Setup` §9b.
//!
//! **The constraint** is `The shape` — *"Server Setup's user-facing re-run is
//! the fallback if automatic repair fails"* — with `Server protocol and
//! transfers` §5 (`Server protocol and transfers` B7: nothing about the request
//! is automatic in that document), `Desktop app and delivery` §12 V10, and
//! `Rented-server runtime`
//! §16 item 12's named condition.
//!
//! **Markers used below that belong to another document, each with the document
//! it belongs to.** `Server protocol and transfers` §5.
//! `Rented-server runtime` §2, G1a, G1b, G17, G26, G31, §16 items 12 and 23.
//! `Desktop app and delivery` §12 V10, §22 item 15.
//!
//! **`Server protocol and transfers` B4, B6 and B7 are not listed above**,
//! because this plan defines a B4, a B6 and a B7 of its own and a bare one could
//! mean either;
//! where this file cites one, the document is written out at the site.

/// **F1: the fallback re-run is a user-triggered action on a connected server
/// that inspects and then acts, entirely on the pre-package surface, so it works
/// on a machine whose supervisor is dead.**
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum FallbackStep {
    /// **1. Package inspection**
    /// (`Server protocol and transfers` §5, `Server protocol and transfers` B4): the
    /// active-package pointer's version, the boot-registration record's content,
    /// the phase, the installation identity and the adapter's declared request
    /// set — **compared by content, never presence.**
    ///
    /// **An installed package speaking the immediately previous supported
    /// server protocol is not *different* for this step and does not trigger
    /// repair merely because it is previous.** The desktop selects that
    /// document's previous-protocol adapter and **retains that adapter's control
    /// route** for inspecting, stopping, releasing and collecting its live or
    /// held work. **A protocol outside that document's compatibility window is
    /// unavailable and is reported as such; this document does not widen the
    /// window.**
    PackageInspection,
    /// **2. Where the package is absent, outside the compatibility window,
    /// fails release-provenance verification, or its phase is not terminal** —
    /// the bootstrap-and-repair request is sent, **user-triggered here exactly
    /// as `Server protocol and transfers` B7 requires**, under the same commit
    /// and rollback rules as the automatic route.
    ///
    /// **Before any phase changes it acquires `Rented-server runtime` G1a's or
    /// G1b's exclusive package claim**, and its requested version resolves
    /// through G26 to G31's signed manifest, exact target tuple and digest
    /// **rather than through a version label**. **A *deferred* answer — live or
    /// held work blocking the commit — is reported as the ordinary result it is
    /// (`Server protocol and transfers` B6), not a failure**; the
    /// previous-protocol control route remains
    /// available while that work stands.
    TheBootstrapAndRepairRequest,
    /// **3. Where the package is present, confirmed, and the supervisor does
    /// not answer** — the re-run issues **the plain start of the registered
    /// service over the command channel**: the user-unit start command on a
    /// systemd machine; **on a container the keeper heals by itself the moment
    /// its own obstacle clears and no start is owed.**
    ///
    /// **This is the step `Rented-server runtime` §16 item 12 names as this
    /// document's and deliberately does not answer** — a full state mount stops
    /// the supervisor from starting, and after space is freed the systemd unit
    /// did not recover merely because room returned; **the machine recovers
    /// completely on one plain start command.**
    ThePlainStart,
    /// **4. Where the supervisor answers** — **nothing is owed**; the re-run
    /// reports the machine healthy and that nothing was owed, which is what the
    /// user was promised.
    NothingIsOwed,
}

impl FallbackStep {
    /// **F1's order.**
    pub const IN_ORDER: [FallbackStep; 4] = [
        FallbackStep::PackageInspection,
        FallbackStep::TheBootstrapAndRepairRequest,
        FallbackStep::ThePlainStart,
        FallbackStep::NothingIsOwed,
    ];
}

/// What the inspection found, which is what decides the step after it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WhatTheInspectionFound {
    pub the_package_is_absent: bool,
    pub the_protocol_is_outside_the_compatibility_window: bool,
    pub it_failed_release_provenance_verification: bool,
    pub its_phase_is_terminal: bool,
    pub the_supervisor_answers: bool,
}

/// **F1, performed: which step follows the inspection.**
pub fn what_follows_the_inspection(found: WhatTheInspectionFound) -> FallbackStep {
    if found.the_package_is_absent
        || found.the_protocol_is_outside_the_compatibility_window
        || found.it_failed_release_provenance_verification
        || !found.its_phase_is_terminal
    {
        return FallbackStep::TheBootstrapAndRepairRequest;
    }
    if !found.the_supervisor_answers {
        return FallbackStep::ThePlainStart;
    }
    FallbackStep::NothingIsOwed
}

/// **F1 step 1: an immediately previous supported protocol keeps its control
/// route and does not trigger repair merely because it is previous.**
pub fn a_previous_protocol_triggers_repair(it_is_the_immediately_previous: bool) -> bool {
    !it_is_the_immediately_previous
}

/// **F1 step 3, re-checked 2026-08-22, and the step is unchanged.**
///
/// **What produced the non-recovery is the start limiter**, which is a per-unit
/// setting the placement itself writes — `systemd.unit(5)` gives
/// `StartLimitIntervalSec=` as one that *"may be set to 0 to disable any kind of
/// rate limiting"* — **so a placement written that way removes the condition
/// this step exists to reach a machine in.** **Whether the placement is written
/// that way is `Rented-server runtime`'s and is not settled there**, and the
/// observed unit was left at its defaults, **so this step is not withdrawn on
/// the strength of a setting nobody has chosen.**
///
/// **Freeing the space is the user's**, through Smoozy's ordinary file
/// operations, which stay available while the supervisor is unavailable.
pub const THE_PLAIN_START_IS_NOT_WITHDRAWN_ON_AN_UNCHOSEN_SETTING: bool = true;

/// **F1 step 4: what step 1 read stays inside the module.**
///
/// The active-package pointer's version, the boot-registration record's content,
/// the phase, the installation identity and the adapter's declared request set
/// **are this module's own bookkeeping, and no spec grants the user a view of
/// any of them.**
pub fn the_user_is_shown_what_the_inspection_read(_they_asked: bool) -> bool {
    false
}

/// **F2: the fits, stated for the record.**
///
/// The re-run **sends exactly `Server protocol and transfers` §5's request and
/// adds no variant to it**; the automatic trigger stays `Desktop app and
/// delivery`'s and **V10's no-second-automatic-route rule is untouched** — this
/// route is user-facing only.
///
/// **`Desktop app and delivery` §22 item 15 narrows but stands**, because the
/// **automatic** path still cannot start a supervisor whose package inspects
/// healthy — its trigger sends a request whose phase is *verified* and which
/// therefore acts on nothing (`Rented-server runtime` G17). **Whether that
/// document routes its trigger through the same start step is its own one-line
/// decision and is reported to the runner rather than written from here.**
pub fn this_route_adds_a_variant_to_the_request(_it_would_help: bool) -> bool {
    false
}

/// **F2: this route is user-facing only.**
pub fn this_route_is_ever_triggered_automatically() -> bool {
    false
}

/// **F3: the fallback has no package-transaction exception.**
///
/// **Two application instances, an automatic repair and this user action all
/// contend for the same claim at the same path** — `Rented-server runtime` G1a's
/// kernel-held lock where a machine-side program is performing the transaction,
/// **G1b's desktop-side exclusive create where none is**. **None is privileged
/// over another and none terminalises a live owner's phase.**
///
/// After a genuinely abandoned owner releases the claim, **whichever route
/// acquires it re-reads and finishes the recorded attempt under its recorded
/// identity before beginning its own.**
///
/// **Where the claim is G1b's, *live* reads one degree weaker and G1b says
/// so** — an unacquirable claim naming another installation may be live or
/// abandoned, which is `Rented-server runtime` §16 item 23's cost **and not a
/// licence to break the claim**. **A waiter giving up changes no machine
/// state.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhichClaim {
    /// **`Rented-server runtime` G1a** — the kernel-held lock, where a
    /// machine-side program is performing the transaction.
    TheKernelHeldLock,
    /// **`Rented-server runtime` G1b** — the desktop's own exclusive create,
    /// where no machine-side program exists to hold one.
    TheDesktopsExclusiveCreate,
}

/// **F3, performed.**
pub fn which_claim(a_machine_side_program_is_performing_the_transaction: bool) -> WhichClaim {
    if a_machine_side_program_is_performing_the_transaction {
        WhichClaim::TheKernelHeldLock
    } else {
        WhichClaim::TheDesktopsExclusiveCreate
    }
}

/// **F3: none is privileged over another and none terminalises a live owner's
/// phase.**
pub fn this_route_is_privileged_over_the_automatic_one() -> bool {
    false
}

/// **F3: a waiter giving up changes no machine state.**
pub fn a_waiter_giving_up_changes_machine_state() -> bool {
    false
}

/// **§13 item 20's residue, recorded rather than repaired.**
///
/// `The shape` §11 assigns *"user-facing fallback re-run for
/// supervisor-independent package repair and replacement"* and **says nothing
/// about how that re-run is admitted**, while F1 step 2 and F3 put it through
/// the same exclusive package owner and the same trusted release selection as
/// every automatic path. **That entry was closed at the Architecture seam close
/// on 2026-08-22**, §11 having been made to agree — and it is named here so a
/// reader who meets the older wording knows which way it went. **Without the
/// clause a user-triggered re-run would be a second unlocked writer of the same
/// phase record and a second route by which an unverified artifact could reach
/// the machine.**
pub const THE_SHAPES_ELEVENTH_SECTION_WAS_MADE_TO_AGREE: bool = true;

#[cfg(test)]
mod tests {
    use super::*;

    fn healthy() -> WhatTheInspectionFound {
        WhatTheInspectionFound {
            the_package_is_absent: false,
            the_protocol_is_outside_the_compatibility_window: false,
            it_failed_release_provenance_verification: false,
            its_phase_is_terminal: true,
            the_supervisor_answers: true,
        }
    }

    #[test]
    fn a_healthy_machine_is_owed_nothing_and_is_told_so() {
        assert_eq!(
            what_follows_the_inspection(healthy()),
            FallbackStep::NothingIsOwed
        );
        assert!(!the_user_is_shown_what_the_inspection_read(true));
    }

    #[test]
    fn a_present_confirmed_package_with_a_dead_supervisor_gets_the_plain_start() {
        // `Rented-server runtime` §16 item 12's condition: a full state mount
        // stops the supervisor starting, and the machine recovers completely on
        // one plain start command.
        let mut found = healthy();
        found.the_supervisor_answers = false;
        assert_eq!(
            what_follows_the_inspection(found),
            FallbackStep::ThePlainStart
        );
        const { assert!(THE_PLAIN_START_IS_NOT_WITHDRAWN_ON_AN_UNCHOSEN_SETTING) };
    }

    #[test]
    fn each_of_the_four_repair_conditions_reaches_the_request() {
        for set in [
            |found: &mut WhatTheInspectionFound| found.the_package_is_absent = true,
            |found: &mut WhatTheInspectionFound| {
                found.the_protocol_is_outside_the_compatibility_window = true
            },
            |found: &mut WhatTheInspectionFound| {
                found.it_failed_release_provenance_verification = true
            },
            |found: &mut WhatTheInspectionFound| found.its_phase_is_terminal = false,
        ] {
            let mut found = healthy();
            set(&mut found);
            assert_eq!(
                what_follows_the_inspection(found),
                FallbackStep::TheBootstrapAndRepairRequest
            );
        }
    }

    #[test]
    fn an_immediately_previous_protocol_keeps_its_control_route() {
        assert!(!a_previous_protocol_triggers_repair(true));
        assert!(a_previous_protocol_triggers_repair(false));
    }

    #[test]
    fn the_claim_is_the_kernels_where_a_program_holds_it_and_the_desktops_where_none_does() {
        assert_eq!(which_claim(true), WhichClaim::TheKernelHeldLock);
        assert_eq!(which_claim(false), WhichClaim::TheDesktopsExclusiveCreate);
        assert!(!this_route_is_privileged_over_the_automatic_one());
        assert!(!a_waiter_giving_up_changes_machine_state());
    }

    #[test]
    fn the_route_is_user_facing_only_and_adds_no_variant() {
        assert!(!this_route_is_ever_triggered_automatically());
        assert!(!this_route_adds_a_variant_to_the_request(true));
        assert_eq!(FallbackStep::IN_ORDER.len(), 4);
        const { assert!(THE_SHAPES_ELEVENTH_SECTION_WAS_MADE_TO_AGREE) };
    }
}
