//! The event journal and acknowledgement — §4.
//!
//! # What this is, and why it is here rather than beside the supervisor
//!
//! **§1.2 names the event journal as one of the machine's durable stores and
//! gives its content to this document** — *"every event, sequenced and durable
//! (§4)"*, *content owned by* **this document**. So what an event is, what a
//! sequence number promises, how a cursor works, what retention keeps and what
//! the cap is are all settled here.
//!
//! **What is not here is the store.** §1.3 rule 1 makes the supervisor the only
//! writer of every durable store in §1.2, and the supervisor is
//! `Rented-server runtime`'s program. That crate's `acting` half is the append
//! and the read, and **the bytes it lays down are a Build decision recorded
//! there** — no clause anywhere fixes a written form for this store any more
//! than it does for the operation records beside it.
//!
//! **Nothing in this file opens a file**, which is what makes every rule of §4
//! testable at the desk. It is the same split
//! [`crate::machine_operations`] already draws for §5.1's two records.
//!
//! # The seven rules, and which of them a type can keep
//!
//! - **E1.** Every event is journalled durably, with an event identity and a
//!   per-machine sequence number that only increases, **before it is offered to
//!   any reader**. *"An event that has not reached the journal has not happened
//!   as far as any reader is concerned."* The identity's kind is
//!   [`crate::identities::IdentityKind::Event`] and [`AnEvent::journalled`]
//!   refuses any other, so an event cannot be journalled under a job's or an
//!   operation's identity by a caller that reached for the wrong one.
//! - **E2.** Each reader holds its own cursor and the cursor is durable on the
//!   reader's side. **The journal holds no per-reader state beyond the
//!   registration that keeps its retention floor honest** — which is
//!   [`ARegisteredReader`] and the sequence it has passed, and nothing else.
//! - **E3.** A cursor advances after the work the event asked for is done,
//!   never on receipt. **That is why reading is separated from acknowledging**:
//!   nothing here advances a cursor as a side effect of a read, and a reader
//!   that dies between the two reads the same events again.
//! - **E4.** Delivery is therefore at least once, and **every reader applies an
//!   event idempotently by its event identity**. The identity is carried for
//!   that and for nothing else; this file does no deduplicating of its own,
//!   because the work an event asks for is the reader's.
//! - **E5.** A reader detects a gap by itself and nothing writes a marker for
//!   it: **the journal reports its own oldest retained sequence number with
//!   every read**, and a reader whose cursor is below that number knows it
//!   missed events. [`WhatAReadOffers::it_missed_events`] is that comparison,
//!   made once here rather than by each reader.
//! - **E6.** The journal keeps an event until every registered cursor has
//!   passed it; beyond that the cap applies and the oldest acknowledged entries
//!   are dropped first. [`what_retention_keeps`] is that rule.
//! - **E6a.** The cap is [`THE_ENTRY_CAP`] entries per machine at a
//!   [`THE_PER_ENTRY_BUDGET`]-byte budget, **fixed rather than read**.
//! - **E7.** An event carries the moment it happened, on the machine's clock,
//!   **and nothing about whether it has been presented**. There is no
//!   presentation field here, per event or otherwise, and a single flag could
//!   not serve two independent readers anyway.
//!
//! # The one thing this file deliberately does not decide
//!
//! **What kinds of event there are.** Every one of them is named by the clause
//! that journals it — `Rented-server runtime` Z9's refusal, `Rented-server
//! runtime` U26's holds, `Rented-server runtime` W11's repeated role-worker
//! failure, §6 step 4's dispositions — so the kind is
//! carried here as the owning document's own word and is never parsed. That is
//! the same shape [`crate::machine_operations`] uses for an operation's kind
//! and its recorded execution group.

use crate::identities::{Identity, IdentityKind};
use crate::observed::Reading;

/// A per-machine sequence number — E1.
///
/// **It only increases and the numbers are contiguous per machine** (E5), which
/// is what lets a reader detect a gap with no extra record.
pub type SequenceNumber = u64;

/// **E6a's cap: 32,768 entries per machine, and it is fixed rather than read.**
///
/// With [`THE_PER_ENTRY_BUDGET`] it multiplies to **16 MiB**, which is the
/// journal's share of the 64 MiB control-state reserve `Rented-server runtime`
/// M4a subtracts from every stream capture. **No free-space reading raises it
/// and none lowers it**: §11.4 forbids clearing anything on a machine on the
/// strength of a free-space reading alone, and this is the one store on that
/// machine that drops what it holds.
///
/// **A builder that fixes a different per-entry encoding derives a different
/// entry count from the same 16 MiB — it does not choose a count freely.**
///
/// *(These two figures stood in `Rented-server runtime`'s own retention module
/// until 2026-09-06, with that file saying in its own words that they are this
/// document's and that they move here the day the machine half of this store is
/// built. This is that day.)*
pub const THE_ENTRY_CAP: u64 = 32_768;

/// **E6a's per-entry budget: 512 bytes, and no entry exceeds it.**
///
/// **An entry's event identity, sequence number, moment and kind are never
/// abbreviated to fit**; a kind whose fields would exceed the budget **names the
/// record that holds the detail rather than copying it in**. So an entry that
/// does not fit is refused by the store rather than trimmed, and the caller that
/// composed it is the one that has to name a record instead.
pub const THE_PER_ENTRY_BUDGET: u64 = 512;

/// **E6a's own arithmetic**: what the cap and the budget multiply to.
pub const fn the_journals_bytes() -> u64 {
    THE_ENTRY_CAP * THE_PER_ENTRY_BUDGET
}

/// What a caller hands the journal — everything about an event except the two
/// things the journal itself issues.
///
/// **The identity and the sequence number are the journal's** (E1 and §2.2's
/// register, which makes the supervisor the issuer of an event identity *"when
/// the event is journalled, before it is offered to any reader"*), so a caller
/// cannot supply either and cannot choose where its event lands in the order.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WhatHappened {
    /// **The owning document's own word for what happened.** Carried and never
    /// parsed here, exactly as an operation's kind is.
    pub kind: String,
    /// The durable subject the event is about, where it has one — an operation,
    /// a job, a hold. **A reader finds its own work by this**, and it is absent
    /// for an event about the machine rather than about a subject.
    pub about: Option<Identity>,
    /// **The owning document's own words, or the name of the record that holds
    /// the detail** — E6a's rule for a kind whose fields would not fit.
    pub says: String,
    /// **E7: the moment it happened, on the machine's clock.** D5 binds: a
    /// reading carries the clock it was taken on, and readings taken on
    /// different clocks are never subtracted from one another.
    pub happened_at: Reading<()>,
}

/// One journalled event — E1.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AnEvent {
    event: Identity,
    sequence: SequenceNumber,
    what: WhatHappened,
}

/// An event identity of the wrong kind — E1 and §2.2's register.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ThatIsNotAnEventIdentity;

impl AnEvent {
    /// **Journalled: the identity the supervisor issued, the sequence number
    /// the journal assigned, and what happened.**
    ///
    /// **An identity of any other kind is refused rather than carried.** §2.2
    /// gives the event identity its own row and §2.1 rule 1 says an identity is
    /// issued once and never reused; an operation identity written into this
    /// field would make a reader's idempotency check under E4 compare an event
    /// against something that is not one.
    pub fn journalled(
        event: Identity,
        sequence: SequenceNumber,
        what: WhatHappened,
    ) -> Result<AnEvent, ThatIsNotAnEventIdentity> {
        if event.kind() != IdentityKind::Event {
            return Err(ThatIsNotAnEventIdentity);
        }
        Ok(AnEvent {
            event,
            sequence,
            what,
        })
    }

    /// **E4's key: the identity a reader applies this event idempotently by.**
    pub fn event(&self) -> &Identity {
        &self.event
    }

    /// **E1's per-machine sequence number**, which is also E3's cursor unit.
    pub fn sequence(&self) -> SequenceNumber {
        self.sequence
    }

    pub fn what(&self) -> &WhatHappened {
        &self.what
    }
}

/// **E2's registration, and it is the whole of the per-reader state the journal
/// holds.**
///
/// *"A reader never advances another reader's cursor, and the journal holds no
/// per-reader state beyond the registration that keeps its retention floor
/// honest."* **There are exactly two readers** and `Rented-server runtime` U27
/// says so in as many words — *"There are exactly two readers of that journal
/// and the Telegram worker is one of them"*.
///
/// **The set is closed rather than a name a caller invents**, because a third
/// registration would raise the retention floor for a reader §4 does not have,
/// and the journal would then keep entries against a cursor nothing advances.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ARegisteredReader {
    /// **The desktop.** E2: its cursor lives in the local record against the
    /// server row **and the installation identity it was taken under** — Y13
    /// makes that cursor one of the machine-derived values, so a new
    /// installation is read from the beginning of its own journal and never
    /// from a cursor taken under a dead one.
    TheDesktop,
    /// **The Telegram worker.** E2: its cursor lives in the machine's own state
    /// area, and `Rented-server runtime` T2 says it advances that cursor after
    /// the work is done, which is E3.
    TheTelegramWorker,
}

impl ARegisteredReader {
    /// Both of them, in one place, so a reader added to §4 fails the match
    /// rather than being missed by a list.
    pub const BOTH: [ARegisteredReader; 2] = [
        ARegisteredReader::TheDesktop,
        ARegisteredReader::TheTelegramWorker,
    ];

    /// The word the registration is written down under.
    pub fn as_text(self) -> &'static str {
        match self {
            ARegisteredReader::TheDesktop => "the desktop",
            ARegisteredReader::TheTelegramWorker => "the Telegram worker",
        }
    }

    /// Read one back. **A word that is neither is nothing**, and never a third
    /// reader made up to fill the field.
    pub fn read_back(text: &str) -> Option<ARegisteredReader> {
        ARegisteredReader::BOTH
            .into_iter()
            .find(|reader| reader.as_text() == text)
    }
}

/// What one read offers a reader — E5.
///
/// **The oldest retained sequence number is reported with every read**, so a
/// reader detects a gap by itself and nothing writes a marker for it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WhatAReadOffers {
    /// The events from the reader's own cursor onward, in sequence order.
    pub events: Vec<AnEvent>,
    /// **E5's number.** Nothing where the journal holds no entry at all.
    pub oldest_retained: Option<SequenceNumber>,
    /// One past the newest sequence the journal holds — **the cursor a reader
    /// that has done all this work would next ask from** (E3).
    pub next_sequence: SequenceNumber,
    /// The cursor this read was taken from, kept so the comparison below is the
    /// read's own rather than a caller's memory of what it asked for.
    pub from_the_cursor: SequenceNumber,
}

impl WhatAReadOffers {
    /// **E5: whether this reader missed events.**
    ///
    /// *"A reader whose cursor is below that number knows it missed events, says
    /// so, and does not present the events it did receive as a complete
    /// account."* **The comparison is made here once** rather than by each
    /// reader, because a reader that forgot to make it would present a partial
    /// account as a whole one, which is the thing E5 exists to prevent.
    pub fn it_missed_events(&self) -> bool {
        match self.oldest_retained {
            None => false,
            Some(oldest) => self.from_the_cursor < oldest,
        }
    }

    /// The cursor a reader has reached **once it has done the work every event
    /// here asked for** — E3, and never before.
    pub fn the_cursor_after_this_work_is_done(&self) -> SequenceNumber {
        match self.events.last() {
            Some(last) => last.sequence() + 1,
            None => self.from_the_cursor.max(self.next_sequence),
        }
    }
}

/// What retention keeps — E6.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WhatRetentionKeeps {
    /// The oldest sequence number the journal may still hold after this rule is
    /// applied.
    pub the_first_sequence_kept: SequenceNumber,
    /// **E6's own named case: the cap forced dropping an entry no cursor had
    /// passed.** *"Where the cap would force dropping an entry no cursor has
    /// passed, it is dropped and the oldest-retained number moves, which is
    /// exactly the case E5 makes visible."*
    pub the_cap_dropped_past_a_cursor: bool,
}

/// **E6, whole.**
///
/// **Nothing but the cap drops an entry.** `Rented-server runtime` R1's own row
/// for this store says its rule is another document's entirely, and its *never
/// ended by* column is empty for exactly that reason: no age, no sweep, no
/// schedule and no free-space reading reaches it. So an acknowledged entry is
/// **droppable** and is not dropped until the cap bites.
///
/// **The oldest acknowledged entries are dropped first, and that is kept by
/// construction rather than by a sort.** A cursor passes entries in order, so
/// everything below the lowest registered cursor is acknowledged and everything
/// at or above it is not; dropping from the oldest end therefore takes
/// acknowledged entries first and reaches an unacknowledged one only when the
/// cap forces it.
pub fn what_retention_keeps(
    oldest_retained: SequenceNumber,
    next_sequence: SequenceNumber,
    every_registered_cursor: &[SequenceNumber],
    cap: u64,
) -> WhatRetentionKeeps {
    let held = next_sequence.saturating_sub(oldest_retained);
    if held <= cap {
        return WhatRetentionKeeps {
            the_first_sequence_kept: oldest_retained,
            the_cap_dropped_past_a_cursor: false,
        };
    }
    let the_first_sequence_kept = next_sequence - cap;
    WhatRetentionKeeps {
        the_first_sequence_kept,
        the_cap_dropped_past_a_cursor: every_registered_cursor
            .iter()
            .any(|cursor| *cursor < the_first_sequence_kept),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::observed::{Clock, MachineClock};
    use std::time::{Duration, SystemTime};

    fn an_identity(kind: IdentityKind) -> Identity {
        Identity::issue(kind).expect("the desk has randomness")
    }

    fn a_moment() -> Reading<()> {
        Reading::taken(
            (),
            SystemTime::UNIX_EPOCH + Duration::from_secs(1_777_000_000),
            Clock::AMachine(MachineClock("the machine".into())),
        )
    }

    fn what_happened(says: &str) -> WhatHappened {
        WhatHappened {
            kind: "a refusal".into(),
            about: None,
            says: says.into(),
            happened_at: a_moment(),
        }
    }

    fn an_event(sequence: SequenceNumber) -> AnEvent {
        AnEvent::journalled(
            an_identity(IdentityKind::Event),
            sequence,
            what_happened("a caller of another login"),
        )
        .expect("an event identity")
    }

    /// **E1: the identity is an event's own kind and no other.**
    #[test]
    fn an_identity_of_any_other_kind_is_refused_rather_than_carried() {
        for wrong in [
            IdentityKind::Operation,
            IdentityKind::Job,
            IdentityKind::Run,
            IdentityKind::Installation,
        ] {
            assert_eq!(
                AnEvent::journalled(an_identity(wrong), 0, what_happened("x")),
                Err(ThatIsNotAnEventIdentity),
                "{wrong:?} is not an event identity"
            );
        }
        assert!(
            AnEvent::journalled(an_identity(IdentityKind::Event), 0, what_happened("x")).is_ok()
        );
    }

    /// **E5: a reader whose cursor is below the oldest retained number knows it
    /// missed events**, and one whose cursor is at or above it did not.
    #[test]
    fn a_cursor_below_the_oldest_retained_number_is_a_gap_and_one_at_it_is_not() {
        let offered = |from: SequenceNumber, oldest: Option<SequenceNumber>| WhatAReadOffers {
            events: Vec::new(),
            oldest_retained: oldest,
            next_sequence: 100,
            from_the_cursor: from,
        };
        assert!(offered(3, Some(40)).it_missed_events());
        assert!(!offered(40, Some(40)).it_missed_events());
        assert!(!offered(41, Some(40)).it_missed_events());
        // An empty journal has nothing to have missed.
        assert!(!offered(0, None).it_missed_events());
    }

    /// **E3: the cursor a reader reaches is one past the last event it did the
    /// work for**, and a read that offered nothing leaves the cursor where the
    /// journal now is rather than moving it backwards.
    #[test]
    fn the_cursor_after_the_work_is_one_past_the_last_event_done() {
        let read = WhatAReadOffers {
            events: vec![an_event(7), an_event(8), an_event(9)],
            oldest_retained: Some(7),
            next_sequence: 10,
            from_the_cursor: 7,
        };
        assert_eq!(read.the_cursor_after_this_work_is_done(), 10);

        let nothing_new = WhatAReadOffers {
            events: Vec::new(),
            oldest_retained: Some(7),
            next_sequence: 10,
            from_the_cursor: 10,
        };
        assert_eq!(nothing_new.the_cursor_after_this_work_is_done(), 10);
    }

    /// **E5 again, from the other side: a reader that fell behind a drop is
    /// carried forward to the oldest entry that is still there** rather than
    /// being left below it for ever.
    #[test]
    fn a_reader_that_missed_events_is_carried_forward_by_what_it_did_receive() {
        let read = WhatAReadOffers {
            events: vec![an_event(40), an_event(41)],
            oldest_retained: Some(40),
            next_sequence: 42,
            from_the_cursor: 3,
        };
        assert!(read.it_missed_events());
        assert_eq!(read.the_cursor_after_this_work_is_done(), 42);
    }

    /// **E6: nothing but the cap drops an entry**, so a journal holding fewer
    /// than the cap keeps every one of them however far past them every cursor
    /// has gone.
    #[test]
    fn an_acknowledged_entry_under_the_cap_is_kept_rather_than_dropped() {
        let kept = what_retention_keeps(0, 100, &[100, 100], THE_ENTRY_CAP);
        assert_eq!(kept.the_first_sequence_kept, 0);
        assert!(!kept.the_cap_dropped_past_a_cursor);
    }

    /// **E6: beyond the cap the oldest go, and they are the acknowledged
    /// ones.**
    #[test]
    fn the_cap_drops_from_the_oldest_end_and_says_it_crossed_no_cursor() {
        let next = THE_ENTRY_CAP + 10;
        let kept = what_retention_keeps(0, next, &[next], THE_ENTRY_CAP);
        assert_eq!(kept.the_first_sequence_kept, 10);
        assert!(
            !kept.the_cap_dropped_past_a_cursor,
            "every cursor had passed the ten that went"
        );
    }

    /// **E6's own named case, and it is the one E5 makes visible**: the cap
    /// forced dropping an entry no cursor had passed.
    #[test]
    fn a_drop_past_a_cursor_is_taken_and_is_reported() {
        let next = THE_ENTRY_CAP + 10;
        let kept = what_retention_keeps(0, next, &[3], THE_ENTRY_CAP);
        assert_eq!(kept.the_first_sequence_kept, 10);
        assert!(
            kept.the_cap_dropped_past_a_cursor,
            "the entry is dropped and the oldest-retained number moves"
        );
    }

    /// **A journal with no registered reader at all is bounded by the cap and by
    /// nothing else**, which is the state a machine is in before either reader
    /// has ever registered.
    #[test]
    fn a_journal_with_no_registered_reader_is_bounded_by_the_cap_alone() {
        let next = THE_ENTRY_CAP + 5;
        let kept = what_retention_keeps(0, next, &[], THE_ENTRY_CAP);
        assert_eq!(kept.the_first_sequence_kept, 5);
        assert!(!kept.the_cap_dropped_past_a_cursor);
    }

    /// **E6a's arithmetic, stated rather than remembered.**
    #[test]
    fn the_cap_and_the_budget_multiply_to_sixteen_binary_megabytes() {
        assert_eq!(the_journals_bytes(), 16 * 1024 * 1024);
        assert_eq!(THE_ENTRY_CAP, 32_768);
        assert_eq!(THE_PER_ENTRY_BUDGET, 512);
    }

    /// **E2: there are exactly two readers, and a word that is neither reads
    /// back as nothing.**
    #[test]
    fn the_two_readers_read_back_and_a_third_word_does_not() {
        for reader in ARegisteredReader::BOTH {
            assert_eq!(ARegisteredReader::read_back(reader.as_text()), Some(reader));
        }
        assert_eq!(ARegisteredReader::read_back("somebody else"), None);
        assert_eq!(ARegisteredReader::read_back(""), None);
        assert_ne!(
            ARegisteredReader::TheDesktop.as_text(),
            ARegisteredReader::TheTelegramWorker.as_text()
        );
    }

    /// **E7: an event carries the moment and the clock, and nothing about
    /// whether it has been presented.**
    ///
    /// The absence is the clause. There is no field to assert on, so what is
    /// asserted is that the two readers' events are the same value — an event
    /// one of them presented cannot read as presented to the other, because
    /// there is nowhere for that to be written.
    #[test]
    fn an_event_carries_its_moment_and_its_clock_and_nothing_about_presentation() {
        let event = an_event(4);
        let moment = a_moment();
        assert_eq!(event.what().happened_at.clock(), moment.clock());
        assert_eq!(event.what().happened_at.taken_at(), moment.taken_at());
        let read_by_one = event.clone();
        let read_by_the_other = event.clone();
        assert_eq!(read_by_one, read_by_the_other);
    }
}
