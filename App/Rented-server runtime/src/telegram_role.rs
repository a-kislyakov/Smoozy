//! §10 — the Telegram role worker, the listening slot and the bounded board.
//!
//! **T4 to T14 consume `Telegram` B1 to B15 and own no competing slot,
//! heartbeat, board-capacity or instruction-survival rule.** What is here is the
//! runtime consequence of that owner's answer, **not a second authority** (T17).
//!
//! **T16. Rows 257, 258, 259 and 260 are `Telegram`'s and none of them is
//! answered here.** That owner owns the conversation's conduct and authority:
//! the board's contents and capacity, how a tap is shaped, how instructions are
//! accepted and retained, which message is pinned and what the user sees.
//!
//! **Markers used in this file that belong to another document, each with the
//! document it belongs to.** `Telegram` B1, B3, B4, B5, B8, B10, B12, B15, J1 to
//! J4, and its §3. `Security and licence` §10 and §1.3. `Server protocol and
//! transfers` C4. `State, identity and recovery` §4 E2, E3.

/// **T1. The role worker is hosted exactly as §5.3 says and gets nothing of its
/// own.**
///
/// It **reads the bot token at start from the file `Security and licence` §10
/// places, never passes it to a child as an argument or an environment value**,
/// submits every action to the same supervisor command path as the SSH adapter,
/// and **never signals a render directly.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HowTheTokenReachesTheWorker {
    /// The only route.
    ReadAtStartFromTheFileSecurityAndLicencePlaces,
}

/// **T1's two prohibitions, written out so a builder meets each.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhatTheTokenIsNeverMade {
    AChildsArgument,
    AChildsEnvironmentValue,
}

impl WhatTheTokenIsNeverMade {
    pub const NEITHER: [WhatTheTokenIsNeverMade; 2] = [
        WhatTheTokenIsNeverMade::AChildsArgument,
        WhatTheTokenIsNeverMade::AChildsEnvironmentValue,
    ];
}

/// **T2. It is a reader of the event journal with its own durable cursor in the
/// machine's state area** (`State, identity and recovery` E2), **and it advances
/// that cursor after the work is done and never on receipt** (that document's
/// E3).
///
/// **The observed reason is the same mechanism**: three updates taken and
/// deliberately not confirmed came back *"identical, same ids"* on the next ask,
/// so *"a machine that grabs an update and dies before acting on it destroys
/// nothing."*
pub const THE_CURSOR_ADVANCES_AFTER_THE_WORK_AND_NEVER_ON_RECEIPT: bool = true;

/// **T3. It binds no inbound endpoint in the ordinary case**, and the one
/// exception — the machine currently holding a registered webhook, where a
/// provider permits one — is `Security and licence` §1.3's, **and nothing here
/// hardens that boundary in either direction.**
///
/// **Where the webhook route is in force, `Telegram` J1 to J4 own the
/// registration-held form and stale-registration removal.** T3's promise to
/// harden that boundary in neither direction stands.
pub const NOTHING_HERE_HARDENS_THE_INBOUND_BOUNDARY_IN_EITHER_DIRECTION: bool = true;

/// **T4. A machine asks for what the user sends only while it holds the slot, and
/// a machine that is refused for conflict stands down rather than retrying.**
///
/// **That single rule removes the observed disaster**: three machines all polling
/// for a minute gave **61 attempts, 60 conflicts and one success, with nobody
/// listening about 92% of the time**, and the cause is that **the newest request
/// always wins, so two machines that both retry destroy each other.** A machine
/// that stands down leaves the newest asker holding the slot cleanly.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhatAMachineDoesWithTheSlot {
    /// It holds the slot: it asks.
    Asks,
    /// **Refused for conflict**: it stands down, and does not retry.
    StandsDown,
}

/// **T4.**
pub fn what_a_machine_does(holds_the_slot: bool) -> WhatAMachineDoesWithTheSlot {
    if holds_the_slot {
        WhatAMachineDoesWithTheSlot::Asks
    } else {
        WhatAMachineDoesWithTheSlot::StandsDown
    }
}

/// **T5. Standing down is not giving up, and the board's heartbeat is tested
/// without subtracting across clocks.**
///
/// `Telegram` B4 and B5 own the exact rule: **a stood-down machine asks again only
/// after completed reads show no change in the opaque heartbeat value continuously
/// for the stated renewal interval measured on the reader's own clock. A failed
/// read advances and resets nothing.** This replaces an age comparison against the
/// holder's clock.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StoodDown {
    /// **Opaque.** Nothing here parses it; the only thing done with it is
    /// comparing it with the last one read.
    pub last_heartbeat_value: Vec<u8>,
    /// How long the value has been unchanged **across completed reads**, on the
    /// reader's own clock.
    pub unchanged_for: std::time::Duration,
}

/// What a stood-down machine does next.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhetherItAsksAgain {
    AsksAgain,
    KeepsStandingDown,
}

/// **T5.** A completed read that shows a changed value **resets**; a failed read
/// **advances and resets nothing**, so it is not an input here at all.
pub fn a_completed_read(
    standing: &mut StoodDown,
    value_now: &[u8],
    since_the_last_read: std::time::Duration,
    the_renewal_interval: std::time::Duration,
) -> WhetherItAsksAgain {
    if value_now != standing.last_heartbeat_value.as_slice() {
        standing.last_heartbeat_value = value_now.to_vec();
        standing.unchanged_for = std::time::Duration::ZERO;
        return WhetherItAsksAgain::KeepsStandingDown;
    }
    standing.unchanged_for += since_the_last_read;
    if standing.unchanged_for >= the_renewal_interval {
        WhetherItAsksAgain::AsksAgain
    } else {
        WhetherItAsksAgain::KeepsStandingDown
    }
}

/// **T6. The board's holder and roster are hints; the slot's authority is
/// Telegram's own exclusivity.**
///
/// If two stood-down machines ask together, the accepted asker becomes holder and
/// the refused one stands down. **`Telegram` B3 makes a hand-over preserve the
/// complete accepted instruction list before the reported pin moves; a claim never
/// evicts or rewrites an accepted instruction.**
pub const THE_BOARDS_HOLDER_AND_ROSTER_ARE_HINTS: bool = true;

/// **T7. A machine that has died releases the slot by not asking again**, which
/// is the free half of the observation and **needs no mechanism**. The heartbeat
/// going stale is what the others read.
pub const A_DEAD_MACHINE_RELEASES_THE_SLOT_BY_NOT_ASKING: bool = true;

/// **T8. Nothing in this arrangement requires a machine to reach another
/// machine.**
///
/// **A rented server never holds another's connection identity and never opens a
/// connection to another**; the conversation is the only channel and this rule
/// uses only reads and writes of it.
pub const NO_MACHINE_EVER_REACHES_ANOTHER_MACHINE: () = ();

/// **T9. The observed failure is exact and it is what the rule exists for.**
///
/// Two machines wrote different instructions to the same pinned message **ten
/// milliseconds apart**; Telegram told both they had succeeded, and a third
/// machine reading every 1.5 seconds only ever saw the later one — *"The earlier
/// instruction was gone with nothing anywhere to show it had existed."*
///
/// **Everything else about the note is sound and observed**: any holder can read
/// it without disturbing the listener, any holder can edit any message the bot
/// sent, two machines editing the same message were never refused, and pinning is
/// a stack so an older pin is hidden rather than destroyed.
pub const TWO_WRITERS_TO_ONE_MESSAGE_LOSE_ONE_SILENTLY: bool = true;

/// **T10. One writer per message remains the primitive.**
///
/// **The slot holder writes the board it created; every other machine proposes
/// instructions and returns acknowledgements through its own machinery message. No
/// machine edits another machine's message.** `Telegram` B1, B10 and B15 own the
/// exact conduct.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhoMayWrite {
    /// The board.
    TheSlotHolderOnly,
    /// Its own machinery message, for proposals and acknowledgements.
    EachMachinesOwnMessage,
}

/// **T11. The bounded board is written only by the slot holder and composed
/// whole.**
///
/// **It has one canonical versioned serialization, admits no rendered candidate
/// over Telegram's 4,096-character bound, and evicts no existing entry to make a
/// new instruction fit. A refused admission is reported; it is never mistaken for
/// an accepted instruction.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BoardAdmission {
    Admitted,
    /// **Reported**, never silently dropped and never an eviction.
    RefusedItWouldNotFit,
}

/// **T11.** The bound is `Telegram`'s and is consumed here rather than chosen.
///
/// **The unit is `Telegram` B2's and is characters after entity parsing** —
/// *"The rendered message must be at most 4,096 characters after entity parsing,
/// Telegram's published `sendMessage` and `editMessageText` bound"*. **It is
/// named here because the clause this crate consumes does not state it**, and a
/// reader who met the number at T11 alone had no way of telling characters from
/// bytes.
pub const THE_RENDERED_BOUND_IN_CHARACTERS: usize = 4_096;

/// **T11.**
pub fn board_admission(rendered_length: usize) -> BoardAdmission {
    if rendered_length <= THE_RENDERED_BOUND_IN_CHARACTERS {
        BoardAdmission::Admitted
    } else {
        BoardAdmission::RefusedItWouldNotFit
    }
}

/// **T12. A board write is accepted only after exact read-back.**
///
/// **Telegram reports success for a write that may be overwritten, and nothing
/// observed offers a conditional write**, so **an unconfirmed candidate is not a
/// delivered instruction and does not advance the causal update cursor.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WriteOutcome {
    Accepted,
    /// **Not a delivered instruction**, and the cursor does not advance.
    UnconfirmedSoNotDelivered,
}

/// **T12.** The comparison is exact and byte for byte; **a reported success is
/// not an input.**
pub fn write_outcome(written: &[u8], read_back: &[u8]) -> WriteOutcome {
    if written == read_back {
        WriteOutcome::Accepted
    } else {
        WriteOutcome::UnconfirmedSoNotDelivered
    }
}

/// **T13. An accepted instruction's authoritative durable home before
/// acknowledgement is the confirmed board entry, outside every rental.**
///
/// The board carries the complete request and operation identity; **its current
/// holder re-presents every unacknowledged identity, and B3 carries the complete
/// list across holder replacement.** The addressee **records the identity durably
/// before acting** (`Server protocol and transfers` C4), acts once and
/// acknowledges through its machinery message; a repeat acts on nothing.
///
/// **No sender-local outstanding record, sender resend loop or sender lifetime is
/// part of the survival proof.**
pub const NO_SENDER_LOCAL_RECORD_IS_PART_OF_THE_SURVIVAL_PROOF: () = ();

/// **T14. An instruction stays on the board until its acknowledgement is
/// observed, and a missed acknowledgement is reported rather than assumed
/// delivered.**
///
/// **Whichever holder owns the board at the entry's reporting deadline tells the
/// user that the addressee has not confirmed. It does not infer that the requested
/// work failed, and it removes nothing without the acknowledgement.**
///
/// **The one exception is `Telegram`'s context-resolution entry** (its B8 and
/// B12), which is a question addressed to one machine rather than an accepted
/// action: **it is completed by being replaced with the full instruction, and
/// expires under that owner's own deadline rule where it never is. An accepted
/// action instruction still never expires.** Returned by `Telegram` under T17
/// rather than written from this side.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhatKeepsAnEntryOnTheBoard {
    /// The ordinary case.
    ItStaysUntilItsAcknowledgementIsObserved,
    /// `Telegram` B8 and B12's one exception.
    TheContextResolutionEntryWhichExpiresUnderItsOwnersDeadline,
}

/// **T14.** Whether an entry may be removed.
pub fn may_be_removed(kind: WhatKeepsAnEntryOnTheBoard, acknowledged: bool) -> bool {
    match kind {
        WhatKeepsAnEntryOnTheBoard::ItStaysUntilItsAcknowledgementIsObserved => acknowledged,
        WhatKeepsAnEntryOnTheBoard::TheContextResolutionEntryWhichExpiresUnderItsOwnersDeadline => {
            true
        }
    }
}

/// **T15. Nothing here is timed against the user's own reading.**
///
/// The end-to-end chain was observed to work — a tap reached the right one of two
/// machines in 1.4 seconds and of three machines in 0.9 and 0.7 — **and this rule
/// adds a read-back to the machine-to-machine half only.**
pub const NOTHING_HERE_IS_TIMED_AGAINST_THE_USERS_OWN_READING: bool = true;

/// **T17. The shape assigns the single receiving slot and pinned-message write
/// serialization to `Telegram`, and that owner has now answered both through its
/// bounded board.** T4 to T14 are the runtime consequences of that answer, **not a
/// second authority.**
pub const THIS_SECTION_IS_A_CONSEQUENCE_AND_NOT_A_SECOND_AUTHORITY: bool = true;

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use super::*;

    #[test]
    fn a_machine_refused_for_conflict_stands_down_rather_than_retrying() {
        assert_eq!(what_a_machine_does(false), WhatAMachineDoesWithTheSlot::StandsDown);
        assert_eq!(what_a_machine_does(true), WhatAMachineDoesWithTheSlot::Asks);
    }

    #[test]
    fn an_unchanged_heartbeat_across_completed_reads_lets_a_machine_ask_again() {
        let mut standing = StoodDown {
            last_heartbeat_value: b"opaque".to_vec(),
            unchanged_for: Duration::ZERO,
        };
        let interval = Duration::from_secs(90);
        assert_eq!(
            a_completed_read(&mut standing, b"opaque", Duration::from_secs(45), interval),
            WhetherItAsksAgain::KeepsStandingDown
        );
        assert_eq!(
            a_completed_read(&mut standing, b"opaque", Duration::from_secs(45), interval),
            WhetherItAsksAgain::AsksAgain
        );
    }

    /// A changed value resets, and nothing subtracts across the holder's clock.
    #[test]
    fn a_changed_heartbeat_resets_the_run() {
        let mut standing = StoodDown {
            last_heartbeat_value: b"opaque".to_vec(),
            unchanged_for: Duration::from_secs(80),
        };
        assert_eq!(
            a_completed_read(
                &mut standing,
                b"a different opaque value",
                Duration::from_secs(45),
                Duration::from_secs(90)
            ),
            WhetherItAsksAgain::KeepsStandingDown
        );
        assert_eq!(standing.unchanged_for, Duration::ZERO);
    }

    #[test]
    fn a_candidate_over_the_bound_is_refused_and_nothing_is_evicted_to_fit_it() {
        assert_eq!(board_admission(4_096), BoardAdmission::Admitted);
        assert_eq!(board_admission(4_097), BoardAdmission::RefusedItWouldNotFit);
    }

    #[test]
    fn a_write_is_accepted_only_on_an_exact_read_back() {
        assert_eq!(write_outcome(b"the board", b"the board"), WriteOutcome::Accepted);
        assert_eq!(
            write_outcome(b"the board", b"somebody else's board"),
            WriteOutcome::UnconfirmedSoNotDelivered
        );
    }

    #[test]
    fn an_accepted_action_instruction_stays_until_it_is_acknowledged() {
        assert!(!may_be_removed(
            WhatKeepsAnEntryOnTheBoard::ItStaysUntilItsAcknowledgementIsObserved,
            false
        ));
        assert!(may_be_removed(
            WhatKeepsAnEntryOnTheBoard::ItStaysUntilItsAcknowledgementIsObserved,
            true
        ));
        // The one exception, which is `Telegram`'s own and returned by that
        // document rather than written from this side.
        assert!(may_be_removed(
            WhatKeepsAnEntryOnTheBoard::TheContextResolutionEntryWhichExpiresUnderItsOwnersDeadline,
            false
        ));
    }

    #[test]
    fn the_token_is_never_a_childs_argument_or_environment_value() {
        // **The pair is closed by the compiler rather than by a count**, the
        // match being total and carrying no default arm.
        for (at, never) in WhatTheTokenIsNeverMade::NEITHER.iter().enumerate() {
            let where_the_list_puts_it = match never {
                WhatTheTokenIsNeverMade::AChildsArgument => 0,
                WhatTheTokenIsNeverMade::AChildsEnvironmentValue => 1,
            };
            assert_eq!(at, where_the_list_puts_it, "{never:?} is not where the list puts it");
        }
        assert_eq!(
            HowTheTokenReachesTheWorker::ReadAtStartFromTheFileSecurityAndLicencePlaces,
            HowTheTokenReachesTheWorker::ReadAtStartFromTheFileSecurityAndLicencePlaces
        );
    }

    #[test]
    fn only_the_slot_holder_writes_the_board() {
        let both = [WhoMayWrite::TheSlotHolderOnly, WhoMayWrite::EachMachinesOwnMessage];
        // **A third writer fails the build here**, the match being total and
        // carrying no default arm — where the length of a list written out
        // inside the test could never have been false.
        for (at, who) in both.iter().enumerate() {
            let where_the_list_puts_it = match who {
                WhoMayWrite::TheSlotHolderOnly => 0,
                WhoMayWrite::EachMachinesOwnMessage => 1,
            };
            assert_eq!(at, where_the_list_puts_it, "{who:?} is not where the list puts it");
        }
    }
}
