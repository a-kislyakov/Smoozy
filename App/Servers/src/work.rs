//! The machine's work — `Servers` §11.1 to §11.7.
//!
//! **The constraint** includes `Rented-server runtime` X6 and `The shape`:
//! **nothing is ever found by executable name, command-line pattern, or process
//! search.**
//!
//! **W1: the last-known queue reading is period-held and there is no durable
//! cache of it anywhere.** `Queue & History` C6 states it and names this
//! document's period as the bound. **W2: so Row 28's *the work is not
//! remembered* is that rule kept from this side** — shown while connected; held
//! as last-known for the rest of the run after a drop, **because a drop does not
//! end the period**; absent on a fresh launch, **because a launch has no
//! period**.
//!
//! **W3: a last-known entry carries the moment it was taken and says it is not
//! updating**, which is `Queue & History` C7 and is Design's to word. **That
//! document's C7 and not this one's**, which is about three providers read on
//! one date.
//!
//! **Markers used below that belong to another document, each with the
//! document it belongs to.** `Queue & History` A1, A2, A6, L13, L16,
//! L17. `Rented-server runtime` X6.
//!
//! **`Queue & History` A5 is deliberately not listed above and is written out
//! with its document at the site**, because this plan defines an A5 of its own
//! and a bare one could mean either.

use crate::condition::WhatIsTrueOfTheServer;

/// **W4: the read is `Queue & History` A1's queue reading and this document
/// does not compose a second one.**
///
/// One request, on demand, **after the installation identity confirms**,
/// period-held, stamped with its moment and clock, returning the machine's whole
/// working state. **It is never taken before the installation check confirms**,
/// and on the absent or different branches **nothing on the machine is read as
/// Smoozy's at all**.
///
/// **W5: it runs on *any* connection, not only the first**, which is what the
/// promise says, so it sits at step 7 of the sequence **on every connection that
/// reaches it**.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LastKnownQueueReading {
    /// **W6: tracked is told from untracked by the desktop's local record and
    /// never by anything on the machine.** `Queue & History` A2: the reading
    /// reaches untracked and declined work because the store does not
    /// distinguish them, and *"identities it names are tracked; identities it
    /// does not are untracked."* **That is Row 43's *tell tracked from
    /// untracked* answered and cited.**
    pub identities_on_the_machine: Vec<String>,
    pub anything_queued_or_running: bool,
}

impl LastKnownQueueReading {
    /// W6, performed against the desktop's own record.
    pub fn untracked<'a>(&'a self, tracked_here: &'a [String]) -> Vec<&'a str> {
        self.identities_on_the_machine
            .iter()
            .filter(|identity| !tracked_here.contains(identity))
            .map(String::as_str)
            .collect()
    }
}

/// **W7: the offer is `Queue & History` A5's and this document presents it.**
/// One offer per
/// machine, covering everything at once, composed from the reading, **less any
/// identity a history tombstone names**.
///
/// **Declining writes nothing durable, so the next connection that finds the
/// work offers again** — which is the spec's *declining is not final*, delivered
/// **by the absence of a record rather than by a schedule**.
///
/// **W8: it never resumes silently.** The offer is made and **nothing enters the
/// local record until the user accepts**, which is A6's atomic write. This is
/// also **the only route by which work abandoned by a *leave it be* removal ever
/// comes back**, and it comes back only on a connection to a re-added server
/// that succeeds.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdoptionOffer {
    pub covers: Vec<String>,
}

impl AdoptionOffer {
    /// **W9: the offer is not filtered by what the current login can do about
    /// the work, and no check of that kind is made in advance.** Two reasons,
    /// and the second is what settles it:
    ///
    /// - **`Queue & History` A5 composes one offer per machine covering
    ///   everything at once.**
    ///   Withholding part of it would leave the machine's state **partly
    ///   invisible**, which is worse than an offer with a stated limit — the
    ///   user would have work running on a machine they are paying for and **no
    ///   route by which Smoozy would ever mention it**.
    /// - **Smoozy cannot establish in advance that it could not stop a job.**
    ///   Fact 60's refusal is **discovered by trying**, the existence probe by
    ///   signal is refused too, and the shape forbids finding anything by
    ///   process search. **A pre-check would be a claim Smoozy has no evidence
    ///   for.**
    ///
    /// **W10: what the offer does not promise is that an adopted job can be
    /// stopped.** Where an adopted job's stop is refused, that is the stop's own
    /// outcome, under `Rendering` K10's bound — **the stop was sent and the
    /// render had not ended when Smoozy stopped waiting**, never *the render
    /// could not be stopped*.
    pub fn composed(
        reading: &LastKnownQueueReading,
        tombstoned: &[String],
    ) -> AdoptionOffer {
        AdoptionOffer {
            covers: reading
                .identities_on_the_machine
                .iter()
                .filter(|identity| !tombstoned.contains(identity))
                .cloned()
                .collect(),
        }
    }

    /// **W7: declining writes nothing durable.**
    pub fn declining_writes_anything_durable() -> bool {
        false
    }
}

/// **W14: after a username edit the work is neither seen nor stoppable through
/// any route this architecture has.**
///
/// **W12: Fact 60 is answered and closed against a real render: seen yes,
/// stopped no.** From a second ordinary login **everything is visible** — both
/// processes, owner, command line, thread count, start time — and **nothing can
/// be stopped**: polite and forcible stops, a pattern kill of the render, and a
/// pattern kill of every process of that login were all refused, and **all 44
/// threads survived**.
///
/// **W13: but Smoozy's own reach is narrower than *seen yes*.** Two things close
/// it: **the supervisor's endpoint refuses any login but the one the
/// installation was placed under** — peer credentials that *"cannot be lied to
/// about who is calling"* — and **the route Fact 60 saw the work by is forbidden
/// here**: nothing is ever found by executable name, command-line pattern, or
/// process search, **and a process listing is exactly that**.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AfterAUsernameEdit {
    /// **W15 item 1: no queue reading is taken, so no adoption offer is
    /// composed.** **Row 43 is *not* a route back to work abandoned by a
    /// username edit.** The spec's own sentence — *"If a later connection finds
    /// that work again, it is picked up the same way as any untracked work"* —
    /// is conditional, **and under the new login no later connection finds it.**
    pub a_queue_reading_is_taken: bool,
    pub an_adoption_offer_is_composed: bool,
    /// **W16: Smoozy is not quiet about it.** The supervisor-unavailable
    /// condition is raised and presented against the server, and **the server
    /// connection is *not* marked down**. So the user is told plainly that
    /// Smoozy could not read the machine, **and nothing claims success over
    /// anything lost.**
    pub the_supervisor_unavailable_condition_is_raised: bool,
    pub the_connection_is_marked_down: bool,
}

impl AfterAUsernameEdit {
    pub fn what_follows() -> AfterAUsernameEdit {
        AfterAUsernameEdit {
            a_queue_reading_is_taken: false,
            an_adoption_offer_is_composed: false,
            the_supervisor_unavailable_condition_is_raised: true,
            the_connection_is_marked_down: false,
        }
    }

    /// **W15 item 2: the route back is the username, and it is the user's.**
    /// Editing the username back to the one the installation was placed under
    /// **restores every route at once, because nothing was destroyed**: the work
    /// kept running, the queue and the journal are on the machine, and the next
    /// connection reads them.
    ///
    /// **W15 item 3: the jobs are abandoned in the fullest sense the spec
    /// allows.** They keep running there and Smoozy stops tracking them.
    pub fn the_route_back() -> &'static str {
        "editing the username back to the one the installation was placed under"
    }
}

/// **W19: the removal warning is composed from the period-held last-known record
/// and from nothing else.**
///
/// **The table is keyed on two facts and never on the two-value shorthand.**
/// As it stood the first column read *connected* and *disconnected*, and **a
/// period standing with a run of attempts in flight was in neither**, while **a
/// period standing whose last-known reading holds no work had no row at all**.
/// The two facts are **whether a connected period stands** and **whether the
/// primary connection is live**.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhatTheLastKnownRecordHolds {
    QueuedOrRunningWork,
    NothingQueuedAndNothingRunning,
    /// A connection that has not reached step 7 has taken no queue reading —
    /// **which includes a connection sitting on §2.3's fourth branch and a
    /// connection that dropped before step 7 ever ran** (row five), and every
    /// case where the period is gone (row six).
    NoRecordAtAll,
}

/// What the removal warning says and what it offers.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RemovalWarning {
    pub warns: Warns,
    pub offers: Offers,
    /// **W20: the switch-off is one row in the one suppression table**,
    /// carrying this document and this warning's identity, **so it silences this
    /// warning on every server at once** until warnings are restored in
    /// Settings. **The connected warning has no such row and no path can write
    /// one**, which makes *cannot be switched off* **a structural fact rather
    /// than a rule to remember**. The difference is the spec's own: the
    /// disconnected warning may be turned off **because it can be wrong**, and
    /// the connected one may not **because it is not**.
    pub may_be_switched_off: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Warns {
    /// From the reading — what is on the machine now.
    FromTheReading,
    /// **From what Smoozy last knew, saying so.**
    FromWhatSmoozyLastKnew,
    /// **None at all. Smoozy does not reach out to find out.**
    NotAtAll,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Offers {
    BothChoices,
    /// ***leave it be* only.** *Stop and clear* would have to reach a machine
    /// over a connection that is down.
    LeaveItBeOnly,
    /// **A plain confirmation and nothing else** — nothing to stop and nothing
    /// to clear, and **nothing is reached for to find out.**
    APlainConfirmation,
}

/// W19's table, performed.
///
/// **Row three covers the whole repair and not only part of it** — a run of
/// attempts in flight and both pauses alike, which is **every stretch in which a
/// period stands over a connection that is not live**. So a *reconnecting*
/// server whose last-known reading holds work is offered ***leave it be* only**.
///
/// **Row six is where a period-held record cannot outlive its period**: **a
/// server the user disconnected by hand this run** — the disconnect ended the
/// period and dropped the reading with it (S11a) — **alongside every case after
/// the app was closed and reopened**. §16 item 18 records the narrowing.
pub fn removal_warning(
    of: WhatIsTrueOfTheServer,
    holds: WhatTheLastKnownRecordHolds,
) -> RemovalWarning {
    if !of.a_period_stands {
        // Row six: no record at all, by S11.
        return RemovalWarning {
            warns: Warns::NotAtAll,
            offers: Offers::APlainConfirmation,
            may_be_switched_off: false,
        };
    }
    match (of.the_primary_connection_is_live, holds) {
        // Row five: reachable while a period stands.
        (_, WhatTheLastKnownRecordHolds::NoRecordAtAll) => RemovalWarning {
            warns: Warns::NotAtAll,
            offers: Offers::APlainConfirmation,
            may_be_switched_off: false,
        },
        (true, WhatTheLastKnownRecordHolds::QueuedOrRunningWork) => RemovalWarning {
            warns: Warns::FromTheReading,
            offers: Offers::BothChoices,
            // **This warning cannot be switched off.**
            may_be_switched_off: false,
        },
        (true, WhatTheLastKnownRecordHolds::NothingQueuedAndNothingRunning) => RemovalWarning {
            warns: Warns::NotAtAll,
            offers: Offers::APlainConfirmation,
            may_be_switched_off: false,
        },
        (false, WhatTheLastKnownRecordHolds::QueuedOrRunningWork) => RemovalWarning {
            warns: Warns::FromWhatSmoozyLastKnew,
            offers: Offers::LeaveItBeOnly,
            may_be_switched_off: true,
        },
        (false, WhatTheLastKnownRecordHolds::NothingQueuedAndNothingRunning) => RemovalWarning {
            warns: Warns::NotAtAll,
            offers: Offers::APlainConfirmation,
            may_be_switched_off: false,
        },
    }
}

/// **W18: the mechanism is `Queue & History` L13 to L17's and this document
/// neither chooses the order nor watches it.**
///
/// Stop-and-clear is **one request, two ordered halves, applied on the
/// machine**, the order held by the supervisor rather than watched by the
/// desktop; **both commands are recorded durably before anything acts, so a
/// connection dropping mid-way changes nothing**. **The waiting jobs are cleared
/// first and the running render is stopped second. Where the clear's write is
/// refused the stop is not attempted, and says so.**
pub const THE_CLEAR_IS_FIRST_AND_THE_STOP_IS_SECOND: bool = true;

/// **W21: the three distinguishable results are L17's, verbatim, and this
/// document presents them.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StopAndClearResult {
    /// Says that **nothing on the machine was changed**; asks whether to **try
    /// again** or to **remove the server and leave the work running**.
    CouldNotBeginAtAll,
    /// Says what it did; asks nothing — **the removal proceeds**.
    ClearedAndStopped,
    /// Says what worked and what did not — the waiting jobs went, and **the stop
    /// was sent and the render had not ended when Smoozy stopped waiting**; asks
    /// whether to **try the remainder again** or to **remove the server anyway
    /// and leave the rest alone**.
    ///
    /// **W23: the middle result is the one the machine bills for, and the
    /// wording is bounded so it cannot lie.** `Rendering` K10 is exact: Smoozy
    /// cannot tell a stop that failed from a render slow to honour it, so
    /// **nothing may claim the render could not be stopped**. **A render that
    /// honours the interrupt after the deadline records its own outcome**, and
    /// the expiry changed nothing.
    ClearedAndTheStopFailedOrExpired,
}

impl StopAndClearResult {
    pub const ALL: [StopAndClearResult; 3] = [
        StopAndClearResult::CouldNotBeginAtAll,
        StopAndClearResult::ClearedAndStopped,
        StopAndClearResult::ClearedAndTheStopFailedOrExpired,
    ];

    /// **W23's bound**, kept as a function so no wording composed anywhere can
    /// pass it.
    pub fn may_claim_the_render_could_not_be_stopped(self) -> bool {
        false
    }

    /// **W22: Smoozy does not treat a machine it could not reach as a machine it
    /// cleared, and no path can produce that sentence.** The server row is
    /// removed **only after the user has answered, and only where they answered
    /// *remove***.
    pub fn the_row_is_removed_without_asking(self) -> bool {
        matches!(self, StopAndClearResult::ClearedAndStopped)
    }
}

/// **W25: the machine-reaching removal's units, declared. Three, in order.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RemovalUnit {
    /// **The clear-and-stop ordered pair**, which `Queue & History` L16 already
    /// makes one unit.
    TheClearAndStopPair,
    /// **Telegram's role off**, an independent unit reported like any other.
    ///
    /// **W24: Telegram's piece comes off on exactly one removal and this
    /// document decides which.** *Stop and clear first* is **the one removal
    /// that reaches the machine**, so on it — and on no other — the role's
    /// durable enabled-or-disabled desired state is set to **disabled** and the
    /// token file is taken off. **On *leave it be*, and on every removal of a
    /// disconnected server, nothing reaches the machine and the role stays
    /// exactly as it was.**
    TelegramsRoleOff,
    /// **The local forget**, which is **atomic entire and runs last and only on
    /// the user's word**. **The period ends with that third unit and not
    /// before**, because the first two reach the machine over the connection it
    /// holds.
    TheLocalForget,
}

impl RemovalUnit {
    pub const IN_ORDER: [RemovalUnit; 3] = [
        RemovalUnit::TheClearAndStopPair,
        RemovalUnit::TelegramsRoleOff,
        RemovalUnit::TheLocalForget,
    ];
}

/// **W19a: what the two ways out may be said to cost, before the user chooses.**
///
/// Design offers them as **statements rather than as buttons**, so the screen
/// states a consequence **before** the act, **and the statement is only true if
/// the machine behaves that way**. **This document supplies what is true and does
/// not word it.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhatMayBeSaidBeforeTheAct {
    /// ***Leave it running*** — the render carries on and Smoozy stops tracking
    /// it. **Nothing about it is in doubt.**
    TheRenderCarriesOnAndSmoozyStopsTracking,
    /// ***Stop and clear first*, the order** — the waiting jobs are cleared
    /// first and the running render is stopped second, one request in two
    /// ordered halves.
    TheClearIsFirstAndTheStopIsSecond,
    /// **The frames already written stay.** `Rendering` K7: *"what a stop leaves
    /// on disk is the frames, which are the user's."*
    TheFramesAlreadyWrittenStay,
    /// **The frame being rendered *may not arrive*, and never that it *is
    /// lost*.**
    ///
    /// The termination is `SIGINT`, and it lands in one of two places.
    /// **Mid-render: that frame's file never appears.** **During that frame's own
    /// write: the write finishes and the frame is whole** — *"An **interrupt**
    /// has never left a short image — not in one run, by anybody, in any
    /// version, on either machine, across hundreds of runs."*
    ///
    /// **So the outcome is one of exactly two and nobody can know which before
    /// the act.** **The flat form is false in the one case anybody has
    /// measured.**
    TheFrameBeingRenderedMayNotArrive,
    /// **The half of the warning that is unconditional is the one worth keeping,
    /// and the evidence is stronger for it than for anything else here: nothing
    /// arrives damaged.** An interrupt has never left a short file at a frame's
    /// final name, and the frames already there are untouched. **A polite
    /// terminate and an outright kill each *do* leave a short file**, which is
    /// exactly why the termination is an interrupt and not either of those.
    NothingArrivesDamaged,
}

impl WhatMayBeSaidBeforeTheAct {
    /// **The flat form is refused**, because it is false in the one case anybody
    /// has measured.
    pub fn the_frame_being_rendered_is_lost_may_be_said() -> bool {
        false
    }

    /// **And nobody is left guessing afterwards.** The completeness statement
    /// composed at the ending says which frames are there, **so *may not arrive*
    /// is resolved into a fact the user reads** rather than into a doubt they
    /// carry.
    pub const THE_COMPLETENESS_STATEMENT_RESOLVES_IT: bool = true;
}

/// **W26 to W29: disconnecting does not stop the machine's work.**
///
/// **W26: a disconnect issues nothing to the machine.** It ends the period,
/// drops what is period-held, and closes the connections under it. **No command
/// is sent, no signal is sent, and nothing on the machine is told.**
///
/// **W27: the work survives because of how it was launched and not because of
/// anything the connection does.** Observed — *"A render started detached
/// survives the connection dropping"*, its parent the system itself, still
/// writing frames after the connection was closed; *"A render **not** detached
/// dies when the connection drops"*, ended at once with the frame in flight lost
/// entirely. **The launch is `Rendering`'s and the machine-side arrangement is
/// `Rented-server runtime`'s**, and this document neither chooses nor performs
/// either.
///
/// **W28: *resumes following* means re-attaching to something already running and
/// never restarting it.** On the next connection, §7's steps read forward from
/// the durable event cursor and take a fresh queue reading. **Nothing is
/// re-submitted, nothing is re-launched, and nothing about the machine's line is
/// written from the desktop.**
///
/// **W29: the jobs waiting behind the render stay queued and run in their
/// turn**, because the admission point that starts the next one is the
/// supervisor's and **needs no connection**.
pub fn a_disconnect_sends_anything_to_the_machine() -> bool {
    false
}

/// **W30: the edit warning is not the removal warning and does not read the
/// removal warning's source.**
///
/// **The two warnings answer different questions.** The removal warning states
/// what is on the machine **now**, because *stop and clear first* acts on it.
/// The edit warning states what the edit **puts out of Smoozy's reach**, which
/// is **a fact about Smoozy's own tracking rather than about the machine's
/// line**.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EditWarningSource {
    /// A period stands and a queue reading is held, after reconciliation has
    /// run. **Names everything the reading shows on that machine, tracked and
    /// untracked alike.**
    TheReading,
    /// **No reading is held** — a period ended by a disconnect, an application
    /// closed and reopened, or a launch that has not connected. **The durable
    /// local record alone**, naming **the jobs this desktop tracks on that
    /// server for which it has learned no ending**.
    ///
    /// **W32: the durable set exists, and it is not a cache of the machine's
    /// state, which is the whole reason it may be read here.** The history record
    /// is *"Smoozy's own note, not a copy"*, and an entry whose machine never
    /// came back *"keeps the job with no ending Smoozy ever learned, shown as
    /// such, for good."* **`State, identity and recovery` D4 is untouched** —
    /// **that document's D4 and not this one's**, which is the records keyed by
    /// the local server identifier, and the plan's own W32 writes it bare: no
    /// queue reading becomes durable, no live figure is shown, and nothing is
    /// presented as current.
    TheDurableLocalRecordAlone,
    /// **W35: where both sources are empty there is no warning, and that is a
    /// true statement rather than the removal warning's ignorance.** Smoozy
    /// tracks nothing on that server and learned an ending for everything it ever
    /// tracked there, **so the edit abandons nothing of Smoozy's**.
    ///
    /// **Untracked work is reached by the first source only**: where a reading is
    /// held it is named like anything else, and where none is held **nothing can
    /// know of it without reaching out to the machine, which no edit ever
    /// does**.
    BothAreEmpty,
}

/// **W31: the edit warning has two sources, taken in this order, and the second
/// is durable.**
pub fn edit_warning_source(
    a_reading_is_held: bool,
    jobs_with_no_learned_ending: usize,
) -> EditWarningSource {
    if a_reading_is_held {
        return EditWarningSource::TheReading;
    }
    if jobs_with_no_learned_ending > 0 {
        return EditWarningSource::TheDurableLocalRecordAlone;
    }
    EditWarningSource::BothAreEmpty
}

/// **W33: so W19's no-record rows do not reproduce here, and that is the whole
/// of Row 317's answer.**
///
/// The ordinary case the ruling of 2026-08-19 was made about — **the user opens
/// Smoozy, sees a disconnected server, and corrects a typo in the login name** —
/// has no period, no reading, and under the removal warning's rule **no warning
/// at all**. It has a durable local record, because the jobs it is about are
/// jobs this desktop submitted. **The warning appears.**
pub fn the_ordinary_case_gets_a_warning() -> bool {
    matches!(
        edit_warning_source(false, 3),
        EditWarningSource::TheDurableLocalRecordAlone
    )
}

/// **W34: the warning built from the durable set says what it knows and never
/// more.**
///
/// It names **jobs Smoozy has not seen end**, **never jobs queued or running**:
/// the durable record carries no line position, and a job with no learned ending
/// may have ended hours ago. **It is over-inclusive by construction and the
/// wording must carry that.** The words are Design's; **the bound on them is that
/// nothing may state the work is still running.**
///
/// This is the spec's own posture on the duplicate flag, cited rather than
/// re-argued — **flag more rather than less**, because a flag raised wrongly only
/// asks the user to read it, and a missed one costs exactly what the ruling was
/// made to prevent.
pub fn the_durable_warning_may_say_the_work_is_still_running() -> bool {
    false
}

/// **W36: the warning cannot be switched off, and no path can write a
/// suppression row for it.**
///
/// The one suppression table carries a module and a warning identity, and the
/// spec grants a switch-off to the **disconnected removal warning alone**, on the
/// reason that it can be wrong. **Giving the edit warning one would be a control
/// no spec asks for**, so it has no row and nothing writes one — the same
/// structural argument W20 makes for the connected removal warning.
pub fn the_edit_warning_may_be_switched_off() -> bool {
    false
}

/// **W37: which edits carry it is the spec's closed list and this document adds
/// none.** **Host or port, and username.** **A key edit and a rename carry no
/// warning**, and neither reaches or moves any work.
pub fn the_edit_carries_a_warning(changed: crate::edits::WhatChanged) -> bool {
    matches!(
        changed,
        crate::edits::WhatChanged::HostOrPort | crate::edits::WhatChanged::Username
    )
}

/// **W38: the warning is composed and answered before anything is written and
/// before the edit's disconnect.**
///
/// §5.1's disconnect and D16's single durable write both happen **only on *go
/// ahead***, so a warning composed from a reading is composed **while the period
/// holding that reading still stands**. **Declining changes nothing at all** — no
/// field, no period, no connection, and nothing durable.
pub const COMPOSED_BEFORE_THE_DISCONNECT_AND_BEFORE_THE_WRITE: bool = true;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::edits::WhatChanged;

    fn connected() -> WhatIsTrueOfTheServer {
        WhatIsTrueOfTheServer {
            a_period_stands: true,
            the_primary_connection_is_live: true,
            a_run_of_attempts_is_in_flight: false,
            the_restart_mark_stands: false,
        }
    }

    fn repairing_mid_run() -> WhatIsTrueOfTheServer {
        WhatIsTrueOfTheServer {
            a_period_stands: true,
            the_primary_connection_is_live: false,
            a_run_of_attempts_is_in_flight: true,
            the_restart_mark_stands: false,
        }
    }

    fn between_runs() -> WhatIsTrueOfTheServer {
        WhatIsTrueOfTheServer {
            a_run_of_attempts_is_in_flight: false,
            ..repairing_mid_run()
        }
    }

    fn no_period() -> WhatIsTrueOfTheServer {
        WhatIsTrueOfTheServer {
            a_period_stands: false,
            the_primary_connection_is_live: false,
            a_run_of_attempts_is_in_flight: false,
            the_restart_mark_stands: false,
        }
    }

    #[test]
    fn a_connected_server_with_work_warns_from_the_reading_and_cannot_be_silenced() {
        let warning = removal_warning(
            connected(),
            WhatTheLastKnownRecordHolds::QueuedOrRunningWork,
        );
        assert_eq!(warning.warns, Warns::FromTheReading);
        assert_eq!(warning.offers, Offers::BothChoices);
        assert!(!warning.may_be_switched_off);
    }

    #[test]
    fn a_repairing_server_with_work_is_offered_leave_it_be_only() {
        // Row three covers the whole repair: a run in flight and both pauses.
        for state in [repairing_mid_run(), between_runs()] {
            let warning =
                removal_warning(state, WhatTheLastKnownRecordHolds::QueuedOrRunningWork);
            assert_eq!(warning.warns, Warns::FromWhatSmoozyLastKnew);
            assert_eq!(warning.offers, Offers::LeaveItBeOnly);
            // The disconnected warning may be turned off because it can be
            // wrong.
            assert!(warning.may_be_switched_off);
        }
    }

    #[test]
    fn a_period_standing_with_no_work_had_no_row_at_all_before_the_re_keying() {
        // The row that was missing: a period stands, the connection is live,
        // and the record holds nothing.
        let warning = removal_warning(
            connected(),
            WhatTheLastKnownRecordHolds::NothingQueuedAndNothingRunning,
        );
        assert_eq!(warning.warns, Warns::NotAtAll);
        assert_eq!(warning.offers, Offers::APlainConfirmation);
    }

    #[test]
    fn a_connection_that_never_reached_step_seven_warns_about_nothing() {
        // Row five, reachable while a period stands.
        let warning = removal_warning(connected(), WhatTheLastKnownRecordHolds::NoRecordAtAll);
        assert_eq!(warning.warns, Warns::NotAtAll);
    }

    #[test]
    fn a_server_disconnected_by_hand_this_run_warns_about_nothing() {
        // Row six, and S11a is the consequence stated rather than left to be
        // met: the disconnect dropped the reading with the period.
        let warning = removal_warning(no_period(), WhatTheLastKnownRecordHolds::NoRecordAtAll);
        assert_eq!(warning.warns, Warns::NotAtAll);
        assert_eq!(warning.offers, Offers::APlainConfirmation);
        assert!(!warning.may_be_switched_off);
    }

    #[test]
    fn only_the_disconnected_warning_with_work_may_ever_be_switched_off() {
        let switchable = [
            (connected(), WhatTheLastKnownRecordHolds::QueuedOrRunningWork),
            (connected(), WhatTheLastKnownRecordHolds::NothingQueuedAndNothingRunning),
            (connected(), WhatTheLastKnownRecordHolds::NoRecordAtAll),
            (between_runs(), WhatTheLastKnownRecordHolds::QueuedOrRunningWork),
            (between_runs(), WhatTheLastKnownRecordHolds::NothingQueuedAndNothingRunning),
            (no_period(), WhatTheLastKnownRecordHolds::NoRecordAtAll),
        ]
        .into_iter()
        .filter(|(of, holds)| removal_warning(*of, *holds).may_be_switched_off)
        .count();
        assert_eq!(switchable, 1);
    }

    #[test]
    fn tracked_is_told_from_untracked_by_the_desktops_own_record() {
        let reading = LastKnownQueueReading {
            identities_on_the_machine: vec!["job-1".into(), "job-2".into(), "job-3".into()],
            anything_queued_or_running: true,
        };
        let tracked_here = vec!["job-1".to_string()];
        assert_eq!(reading.untracked(&tracked_here), vec!["job-2", "job-3"]);
    }

    #[test]
    fn the_offer_covers_everything_less_a_tombstoned_identity() {
        let reading = LastKnownQueueReading {
            identities_on_the_machine: vec!["job-1".into(), "job-2".into()],
            anything_queued_or_running: true,
        };
        let offer = AdoptionOffer::composed(&reading, &["job-2".to_string()]);
        assert_eq!(offer.covers, vec!["job-1"]);
        assert!(!AdoptionOffer::declining_writes_anything_durable());
    }

    #[test]
    fn the_offer_is_not_filtered_by_what_the_login_can_stop() {
        // W9: a pre-check would be a claim Smoozy has no evidence for.
        let reading = LastKnownQueueReading {
            identities_on_the_machine: vec!["job-1".into(), "job-2".into()],
            anything_queued_or_running: true,
        };
        assert_eq!(AdoptionOffer::composed(&reading, &[]).covers.len(), 2);
    }

    #[test]
    fn a_username_edit_composes_no_offer_and_is_not_quiet_about_it() {
        let after = AfterAUsernameEdit::what_follows();
        assert!(!after.a_queue_reading_is_taken);
        assert!(!after.an_adoption_offer_is_composed);
        assert!(after.the_supervisor_unavailable_condition_is_raised);
        // W16: the server connection is not marked down.
        assert!(!after.the_connection_is_marked_down);
    }

    #[test]
    fn the_three_stop_and_clear_results_never_claim_a_render_could_not_be_stopped() {
        assert_eq!(StopAndClearResult::ALL.len(), 3);
        for result in StopAndClearResult::ALL {
            assert!(!result.may_claim_the_render_could_not_be_stopped());
        }
    }

    #[test]
    fn the_removals_units_are_three_with_the_local_forget_last() {
        assert_eq!(RemovalUnit::IN_ORDER.len(), 3);
        assert_eq!(RemovalUnit::IN_ORDER[2], RemovalUnit::TheLocalForget);
    }

    #[test]
    fn the_frame_being_rendered_may_not_arrive_and_is_never_said_to_be_lost() {
        assert!(!WhatMayBeSaidBeforeTheAct::the_frame_being_rendered_is_lost_may_be_said());
    }

    #[test]
    fn a_disconnect_tells_the_machine_nothing() {
        assert!(!a_disconnect_sends_anything_to_the_machine());
    }

    #[test]
    fn the_edit_warning_falls_back_to_the_durable_record_where_no_reading_is_held() {
        assert_eq!(edit_warning_source(true, 0), EditWarningSource::TheReading);
        assert_eq!(
            edit_warning_source(false, 2),
            EditWarningSource::TheDurableLocalRecordAlone
        );
        assert_eq!(edit_warning_source(false, 0), EditWarningSource::BothAreEmpty);
    }

    #[test]
    fn the_ordinary_typo_case_gets_a_warning_where_the_removal_warning_would_not() {
        assert!(the_ordinary_case_gets_a_warning());
        // The removal warning's own rule on the same server says nothing.
        let warning = removal_warning(no_period(), WhatTheLastKnownRecordHolds::NoRecordAtAll);
        assert_eq!(warning.warns, Warns::NotAtAll);
    }

    #[test]
    fn the_durable_warning_never_states_the_work_is_still_running() {
        assert!(!the_durable_warning_may_say_the_work_is_still_running());
        assert!(!the_edit_warning_may_be_switched_off());
    }

    #[test]
    fn only_host_or_port_and_username_carry_the_edit_warning() {
        assert!(the_edit_carries_a_warning(WhatChanged::HostOrPort));
        assert!(the_edit_carries_a_warning(WhatChanged::Username));
        assert!(!the_edit_carries_a_warning(WhatChanged::TheKey));
        assert!(!the_edit_carries_a_warning(WhatChanged::TheLabelOrTheColour));
    }
}
