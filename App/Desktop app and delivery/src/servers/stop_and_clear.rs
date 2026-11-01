//! **The desktop's half of stop and clear: one sender for the pair** — step
//! 10c's S2, `Queue & History` L13 to `Queue & History` L17 on the desktop, over the machine's
//! half S1 built.
//!
//! **What it is, in the order the clauses give it.**
//!
//! 1. **Two fresh operation identities, one ordered pair** — `Queue & History`
//!    L13's `ThePair`, which has no field for a target.
//! 2. **`State, identity and recovery` §9.4's outcome record, written before
//!    anything is sent** — `Queue & History` L17: *"holds the enumeration
//!    durably before the first unit is attempted, so an interruption loses the
//!    saying and never the fact"*. **Where it will not write, nothing is sent**:
//!    a pair sent with no record behind it is exactly the saying-without-the-fact
//!    `Queue & History` L17 forbids.
//! 3. **The clear, carrying its stop's identity, then the stop, carrying its
//!    clear's** — each half names the other, so whichever the machine meets
//!    first records both before anything acts (S1's account, *the three things
//!    established*, item 1). A line that drops between the two changes nothing.
//! 4. **The stop is waited on by its own identity, with its own deadline** —
//!    `Queue & History` L12 and `Rendering` K8: the identity is asked again,
//!    and `Server protocol and transfers` C8's *still in hand* is an answer
//!    rather than a silence. **The deadline is the waiter's own, `Rendering`
//!    K9's 120 seconds**, and its expiry is the waiter giving up, never the
//!    stop failing: **no outcome is written for it** (`Server protocol and
//!    transfers` C17, `Servers` W23).
//! 5. **Each half's outcome is written the moment it is learned**, and
//!    `Queue & History` L17's three results are read off what was learned —
//!    with *could not begin* split into `Servers` W21's two drawn endings by
//!    what the machine said: nothing changed there, or it refused the clear's
//!    write so the stop was never attempted (`Queue & History` L16).
//!
//! **An older program's refusal is *could not begin at all*** — the stop and
//! clear plan pass's SC-2, now `Servers` W21's last paragraph. Every release
//! before the one whose program answers the clear refuses it by name, and
//! nothing on the machine changed; **it is never told as a machine Smoozy could
//! not reach, because Smoozy reached it.**
//!
//! **A lost answer is asked again by the same identity and never re-sent under a
//! fresh one** (`Server protocol and transfers` C9). The machine records before
//! it acts (C4), so the same request arriving twice acts once.
//!
//! # What this file does not do
//!
//! - **It presents nothing.** What the three results are said as is `Servers`
//!   W21's and Design's, at `src/surface/servers.rs`.
//! - **It forgets no server.** `Servers` W25's third unit is the removal's,
//!   and runs only on *cleared and stopped* or on the user's word.
//! - **It sends no single stop and no cancel.** Those are the queue's own acts
//!   (S3).
//!
//! # How a clause marker is written in this file
//!
//! **This file keeps no header list** and writes every other document's name
//! at its own site. **A bare marker below means a clause of this crate's own
//! plan, `Preparation/Architecture/Desktop app and delivery.md`.**

use std::time::{Duration, Instant};

use smoozy_queue_history::stop_and_clear::{
    the_stops_unit_where_the_clear_was_refused, ThePair, THE_ACTION, THE_CLEAR, THE_STOP,
};
use smoozy_server_protocol::asking_the_supervisor::WhatCameBack;
use smoozy_server_protocol::framing::{Fields, Request, THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS};
use smoozy_server_protocol::requests::{
    ITS_OWN_ENDING, IT_HAS_A_RECORDED_OUTCOME, IT_IS_STILL_IN_HAND, IT_WAS_CLEARED,
    IT_WAS_LEFT_STOPPED, NOTHING_WAS_RENDERING, THERE_IS_NO_RECORD_OF_IT, THE_CLEARS_OUTCOME_FIELD,
    THE_CLEAR_REQUEST, THE_GROUP_FIELD, THE_PAIRED_CLEAR_FIELD, THE_PAIRED_STOP_FIELD,
    THE_RENDER_IS_NOT_A_MEMBER, THE_RENDER_THE_DESKTOP_LAST_SAW_FIELD,
    THE_STOPS_OUTCOME_FIELD, THE_STOP_PAIRED_WITH_A_CLEAR_REQUEST, THE_STOP_WAS_NOT_ATTEMPTED,
    THE_TELEGRAM_ROLE_IS_DISABLED_REQUEST, WHAT_BECAME_OF_IT_FIELD, WHAT_THE_ROLE_WORKER_DID_FIELD,
};
use smoozy_state::identities::{Identity, IdentityKind};
use smoozy_state::outcomes::{Keeps, UnitOutcome, Writing};

use crate::servers::the_machines_work::AsksTheSupervisor;

/// **`Servers` W25's second unit, named as the removal's §9.4 record
/// enumerates it**: Telegram's role off, *"an independent unit reported like
/// any other"*. Only the removal enumerates it (`Servers` W24: *"on it — and on
/// no other"*).
pub const TELEGRAMS_ROLE_OFF: &str = "Telegram's role off";

/// **How long the pair is waited on, and how often its identities are asked
/// again.**
///
/// **The deadline is `Rendering` K9's: the waiter's own, 120 seconds**, an
/// assumption bounded from above by the two sets of measurements that clause
/// names. **It belongs to the waiter** (`Server protocol and transfers` C16):
/// it is process memory and nothing durable is written when it passes.
///
/// **The pause is this file's own**: the stop is observed to take 1 to 3
/// seconds on a virtual machine and 1.9 to 14 on a container (`Queue &
/// History` L12, Fact 2), so asking once a second learns it within a second
/// of its landing and asks a few dozen times at most. **It is a delay inside
/// a sequence already running** — the one shape P10b admits — and nothing
/// here repeats once the act has ended.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TheWait {
    pub deadline: Duration,
    pub pause: Duration,
}

impl TheWait {
    /// `Rendering` K9's deadline, and a one-second pause.
    pub const THE_STOPS_OWN: TheWait = TheWait {
        deadline: Duration::from_secs(120),
        pause: Duration::from_secs(1),
    };

    /// **What is left of this wait, begun at `began`** — nothing once its
    /// deadline has passed. **Every ask is given what is left and no more**,
    /// and none is begun once nothing is (`Server protocol and transfers` C16).
    /// Added 2026-10-19, the dead-line sweep's two-minute-wait repair: until then the
    /// deadline was looked at only after an ask returned, so an ask begun just
    /// before it on a line gone silent held the user's wait a whole break
    /// window past it, and the pair's second ask could be begun after it.
    pub fn what_is_left(&self, began: Instant) -> Option<Duration> {
        self.deadline
            .checked_sub(began.elapsed())
            .filter(|left| !left.is_zero())
    }

    /// **The pause between two asks, never reaching past the deadline.**
    pub fn pauses(&self, began: Instant) {
        if let Some(left) = self.what_is_left(began) {
            std::thread::sleep(self.pause.min(left));
        }
    }

    /// **The rest of this wait, as a wait of its own** — for a second act the
    /// same press makes after the first, so the user's one wait keeps its
    /// one deadline.
    pub fn the_rest_of(&self, began: Instant) -> TheWait {
        TheWait {
            deadline: self.what_is_left(began).unwrap_or(Duration::ZERO),
            pause: self.pause,
        }
    }
}

/// **What the pair carries besides the two identities** — `Queue & History`
/// L19's group, and the render the desktop's period-held reading last showed.
///
/// **The render is never a target** (`Queue & History` L13): the machine reads
/// it only so that *"where the resolved operation is not the one the desktop's
/// reading last showed, the outcome names both"*.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct WhatThePairCarries {
    pub group: Option<Identity>,
    pub the_render_the_desktop_last_saw: Option<Identity>,
}

/// **What the pair came to** — `Queue & History` L17's three results, with
/// *could not begin at all* split the way `Servers` W21's drawing splits it,
/// and one more that is not a result at all.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhatThePairCameTo {
    /// **Cleared and stopped.** Both halves done.
    ClearedAndStopped,
    /// **Could not begin at all, and nothing on the machine changed**: the
    /// program refused the request by name (SC-2), refused it before recording
    /// it, the request never reached the machine, or the record would not
    /// write and nothing was sent.
    NothingWasChanged,
    /// **Could not begin at all, because the machine refused the clear's
    /// write** — `Queue & History` L16: the stop was not attempted, and says
    /// so. **A clear still in hand when the wait ended is said the same way**:
    /// it is a clear the machine could not finish (S1's *still in hand*, a
    /// clear refused part-way), and until it is finished the stop is not
    /// attempted either.
    TheClearWasRefused,
    /// **Cleared, and the stop failed or its waiter gave up** — `Rendering`
    /// K10: *"the stop was sent, and the render had not ended when Smoozy
    /// stopped waiting"*.
    TheRenderHadNotStopped,
    /// **Nothing that answers the clear came back, and the wait ended.** The
    /// request may have reached the machine, in which case the machine carries
    /// the pair out with nobody connected (`Queue & History` L13). **So the
    /// clear's and the stop's outcomes are left unwritten** — the fact is not
    /// invented — and what is said is the caller's (see
    /// `src/surface/servers.rs`).
    NothingCameBack,
}

/// **What the act left behind**: what it came to, and the §9.4 record it wrote
/// — absent only where the record would not write and nothing was sent.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ThePairWent {
    pub came_to: WhatThePairCameTo,
    pub the_record: Option<Identity>,
}

/// **A machine nobody can ask** — no live connection, or no package area
/// recorded to ask it at. **Every ask is *never asked***, which is the truth:
/// nothing reached the machine.
pub struct NoMachineToAsk;

impl AsksTheSupervisor for NoMachineToAsk {
    fn ask(&mut self, _request: &Request) -> WhatCameBack {
        WhatCameBack::ItWasNeverAsked
    }
}

/// **What one ask of one half heard**, read off `Server protocol and
/// transfers` C8's closed set of three, plus the two ways nothing was heard.
#[derive(Debug, Clone, PartialEq, Eq)]
enum Heard {
    /// **The program does not answer this request, or does not speak this
    /// protocol version** — an older program (SC-2). Nothing on the machine
    /// changed.
    ARefusal,
    /// C8's *no record of that identity*: *"the work did not happen"*.
    NoRecord,
    /// C8's *still in hand*: recorded, no outcome yet.
    StillInHand,
    /// C8's *a recorded outcome*, with the half's own word.
    AnOutcome(String),
    /// **The command ran and nothing correlated came back** — the request may
    /// have arrived.
    NothingCorrelated,
    /// **The request never reached the machine.**
    NeverAsked,
}

fn what_was_heard(came_back: WhatCameBack, the_halfs_own_field: &str) -> Heard {
    match came_back {
        WhatCameBack::ItAnswered(answered) => {
            if answered.the_request_name_it_says_it_does_not_answer().is_some()
                || answered.the_protocol_version_it_says_it_does_not_speak().is_some()
            {
                return Heard::ARefusal;
            }
            match answered.fields.text(WHAT_BECAME_OF_IT_FIELD) {
                Some(IT_HAS_A_RECORDED_OUTCOME) => Heard::AnOutcome(
                    answered
                        .fields
                        .text(the_halfs_own_field)
                        .unwrap_or_default()
                        .to_string(),
                ),
                Some(IT_IS_STILL_IN_HAND) => Heard::StillInHand,
                Some(THERE_IS_NO_RECORD_OF_IT) => Heard::NoRecord,
                // **An answer that says none of the three is not one of them**,
                // and nothing is guessed from it.
                _ => Heard::NothingCorrelated,
            }
        }
        WhatCameBack::NothingCorrelatedCameBack => Heard::NothingCorrelated,
        WhatCameBack::ItWasNeverAsked => Heard::NeverAsked,
    }
}

/// **The stop's recorded outcome, as the §9.4 unit.**
///
/// - ***Left stopped***, ***its own ending*** and ***nothing was rendering***
///   are **done**: the render is not running, which is what the stop was for.
///   *Its own ending* is `State, identity and recovery` §5.3 step 7 — the
///   render ended before the interrupt was issued — and the line was already
///   empty by then, so the machine is emptied either way.
/// - ***The render is not a member of the group*** is **done** too: `Queue &
///   History` L19's group clear stops only a member, and *no member rendering*
///   is the stop's whole work done. *(Only a group's clear meets it, and that
///   is S3's; nothing on this screen sends a group.)*
/// - ***Not attempted, because the clear was not done*** is `Queue & History`
///   L16's *not attempted*, naming the clear.
/// - **Anything else is failed** — *the stop failed*, and any word this build
///   does not know.
fn the_stops_unit(word: &str) -> UnitOutcome {
    match word {
        IT_WAS_LEFT_STOPPED | ITS_OWN_ENDING | NOTHING_WAS_RENDERING | THE_RENDER_IS_NOT_A_MEMBER => {
            UnitOutcome::Done
        }
        THE_STOP_WAS_NOT_ATTEMPTED => the_stops_unit_where_the_clear_was_refused(),
        _ => UnitOutcome::Failed,
    }
}

/// **One outcome written.** A write the store refuses is not a reason to stop
/// saying what happened: the saying is what the user is owed now, and the
/// record keeps what it could.
fn ended(writing: &mut Writing<'_>, unit: &str, outcome: UnitOutcome) {
    let _ = writing.a_unit_ended(unit, outcome);
}

/// One half, framed: its own identity, its partner's under the partner's
/// field, and what the pair carries (S1's account, *for the pieces after this
/// one*).
fn one_half(
    name: &str,
    itself: &Identity,
    the_partners_field: &str,
    partner: &Identity,
    carries: &WhatThePairCarries,
) -> Request {
    let mut fields = Fields::none().and(the_partners_field, partner.as_text());
    if let Some(group) = &carries.group {
        fields = fields.and(THE_GROUP_FIELD, group.as_text());
    }
    if let Some(render) = &carries.the_render_the_desktop_last_saw {
        fields = fields.and(THE_RENDER_THE_DESKTOP_LAST_SAW_FIELD, render.as_text());
    }
    Request::speaking(THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS, itself.clone(), name).carrying(fields)
}

/// **`Queue & History` L13 to `Queue & History` L17: the pair, sent, waited on and recorded.**
///
/// `the_units_after` are units the caller's own act enumerates after the
/// pair's two — the removal's `Servers` W25 second unit — so one record holds
/// the whole act's enumeration before its first unit is attempted (`Servers`
/// W22). **This writes none of their outcomes**; the caller does, through the
/// record it is handed back.
pub fn the_pair_is_sent(
    keeper: &mut dyn Keeps,
    machine: &mut dyn AsksTheSupervisor,
    carries: &WhatThePairCarries,
    the_units_after: &[&str],
    wait: TheWait,
) -> ThePairWent {
    let nothing = |the_record| ThePairWent {
        came_to: WhatThePairCameTo::NothingWasChanged,
        the_record,
    };
    let (Ok(clear), Ok(stop)) = (
        Identity::issue(IdentityKind::Operation),
        Identity::issue(IdentityKind::Operation),
    ) else {
        return nothing(None);
    };
    let pair = ThePair::composed(clear, stop, carries.group.clone());

    // **2. The record, before anything is sent** (`Queue & History` L17).
    let mut units = vec![THE_CLEAR, THE_STOP];
    units.extend_from_slice(the_units_after);
    let Ok(mut writing) = Writing::open(keeper, THE_ACTION, &units) else {
        return nothing(None);
    };
    let the_record = Some(writing.identity().clone());

    let the_clear = one_half(THE_CLEAR_REQUEST, &pair.clear, THE_PAIRED_STOP_FIELD, &pair.stop, carries);
    let the_stop = one_half(
        THE_STOP_PAIRED_WITH_A_CLEAR_REQUEST,
        &pair.stop,
        THE_PAIRED_CLEAR_FIELD,
        &pair.clear,
        carries,
    );

    // **Each outcome is written the moment it is learned** ([`ended`]).
    let mut the_clears: Option<UnitOutcome> = None;
    let mut the_stops: Option<UnitOutcome> = None;
    // **Whether the machine may hold the pair**: something ran on it and
    // nothing said it did not.
    let mut it_may_have_arrived = false;
    // **Whether the machine says the clear is recorded and not finished** —
    // S1's *still in hand*, a clear refused part-way.
    let mut the_clear_is_in_hand = false;
    let began = Instant::now();

    loop {
        // **3a. The clear**, asked until its outcome is known — **with what
        // is left of the wait and no more**, the dead-line sweep's two-minute-wait repair.
        if the_clears.is_none() {
            let Some(left) = wait.what_is_left(began) else {
                return the_wait_is_over(&the_clears, the_clear_is_in_hand, the_record);
            };
            match what_was_heard(machine.ask_within(&the_clear, left), THE_CLEARS_OUTCOME_FIELD) {
                Heard::ARefusal | Heard::NoRecord => {
                    // **Nothing on the machine changed**, and the stop is not
                    // sent: it is unsafe without the clear (`State, identity
                    // and recovery` §9.3), and a program that refuses one half
                    // refuses the other.
                    ended(&mut writing, THE_CLEAR, UnitOutcome::Failed);
                    ended(&mut writing, THE_STOP, the_stops_unit_where_the_clear_was_refused());
                    return nothing(the_record);
                }
                Heard::NeverAsked if !it_may_have_arrived => {
                    ended(&mut writing, THE_CLEAR, UnitOutcome::Failed);
                    ended(&mut writing, THE_STOP, the_stops_unit_where_the_clear_was_refused());
                    return nothing(the_record);
                }
                Heard::AnOutcome(word) if word == IT_WAS_CLEARED => {
                    it_may_have_arrived = true;
                    the_clear_is_in_hand = false;
                    the_clears = Some(UnitOutcome::Done);
                    ended(&mut writing, THE_CLEAR, UnitOutcome::Done);
                }
                Heard::AnOutcome(_) => {
                    // **`Queue & History` L16**: the clear's write was refused,
                    // so the stop was not attempted, and says so.
                    ended(&mut writing, THE_CLEAR, UnitOutcome::Failed);
                    ended(&mut writing, THE_STOP, the_stops_unit_where_the_clear_was_refused());
                    return ThePairWent {
                        came_to: WhatThePairCameTo::TheClearWasRefused,
                        the_record,
                    };
                }
                Heard::StillInHand => {
                    it_may_have_arrived = true;
                    the_clear_is_in_hand = true;
                }
                Heard::NothingCorrelated => it_may_have_arrived = true,
                Heard::NeverAsked => {}
            }
        }

        // **3b. The stop** — sent once the clear may have arrived, and then
        // asked again by its own identity until its outcome is known
        // (`Queue & History` L12, `Rendering` K8). **It is sent even where the
        // clear's own answer was lost**: it names the clear, so a machine the
        // clear never reached records both on its arrival and carries the pair
        // out whole.
        //
        // **No stop is begun once the wait is over** (the dead-line sweep's
        // two-minute-wait repair): until then a clear that came back at the deadline was
        // followed by a stop sent after it. **Nothing is lost by not sending
        // it**: the clear names its stop, so a machine the clear reached
        // records both and carries the pair out whole (S1).
        if the_stops.is_none() && it_may_have_arrived {
            let Some(left) = wait.what_is_left(began) else {
                return the_wait_is_over(&the_clears, the_clear_is_in_hand, the_record);
            };
            match what_was_heard(machine.ask_within(&the_stop, left), THE_STOPS_OUTCOME_FIELD) {
                Heard::AnOutcome(word) => {
                    let outcome = the_stops_unit(&word);
                    ended(&mut writing, THE_STOP, outcome.clone());
                    the_stops = Some(outcome);
                }
                Heard::NoRecord if the_clears == Some(UnitOutcome::Done) => {
                    // The clear was done and the machine holds no stop — the
                    // work did not happen (C8).
                    ended(&mut writing, THE_STOP, UnitOutcome::Failed);
                    the_stops = Some(UnitOutcome::Failed);
                }
                _ => {}
            }
        }

        match (&the_clears, &the_stops) {
            (Some(UnitOutcome::Done), Some(UnitOutcome::Done)) => {
                return ThePairWent {
                    came_to: WhatThePairCameTo::ClearedAndStopped,
                    the_record,
                };
            }
            (Some(UnitOutcome::Done), Some(_)) => {
                return ThePairWent {
                    came_to: WhatThePairCameTo::TheRenderHadNotStopped,
                    the_record,
                };
            }
            // **The stop's own answer says the clear was not done** (`Queue &
            // History` L16), learned before the clear's own answer was.
            (None, Some(UnitOutcome::NotAttempted { .. })) => {
                ended(&mut writing, THE_CLEAR, UnitOutcome::Failed);
                return ThePairWent {
                    came_to: WhatThePairCameTo::TheClearWasRefused,
                    the_record,
                };
            }
            _ => {}
        }

        // **4. The waiter's own deadline.** Its expiry writes nothing.
        if wait.what_is_left(began).is_none() {
            return the_wait_is_over(&the_clears, the_clear_is_in_hand, the_record);
        }
        wait.pauses(began);
    }
}

/// **What the pair came to when its waiter's deadline passed** — step 4.
/// Nothing is written for it (`Server protocol and transfers` C17).
fn the_wait_is_over(
    the_clears: &Option<UnitOutcome>,
    the_clear_is_in_hand: bool,
    the_record: Option<Identity>,
) -> ThePairWent {
    let came_to = match the_clears {
        Some(UnitOutcome::Done) => WhatThePairCameTo::TheRenderHadNotStopped,
        _ if the_clear_is_in_hand => WhatThePairCameTo::TheClearWasRefused,
        _ => WhatThePairCameTo::NothingCameBack,
    };
    ThePairWent { came_to, the_record }
}

/// **`Servers` W24: Telegram's role off** — `Rented-server runtime` W8's
/// other act, the role's durable desired state written to *disabled* through
/// the ordinary command path, and the one thing the supervisor does after it
/// is stop the role worker (`Rented-server runtime` W10).
///
/// **Done where the supervisor says what it did with the role worker**, the
/// same reading `src/setting_up.rs` takes of the *enabled* twin: an answer that
/// says nothing is not a state that was written. **Failed otherwise**,
/// including a program that does not answer it.
///
/// **The token file is not taken off.** `Servers` W24 names it beside the
/// desired state, and no request anywhere asks the machine to remove it; the
/// desired state is what stops the bot, and the file is recorded as owed.
pub fn the_telegram_role_is_switched_off(machine: &mut dyn AsksTheSupervisor) -> UnitOutcome {
    let Ok(command) = Identity::issue(IdentityKind::Operation) else {
        return UnitOutcome::Failed;
    };
    let request = Request::speaking(
        THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS,
        command,
        THE_TELEGRAM_ROLE_IS_DISABLED_REQUEST,
    );
    match machine.ask(&request) {
        WhatCameBack::ItAnswered(answered)
            if answered.the_request_name_it_says_it_does_not_answer().is_none()
                && answered.fields.text(WHAT_THE_ROLE_WORKER_DID_FIELD).is_some() =>
        {
            UnitOutcome::Done
        }
        _ => UnitOutcome::Failed,
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use smoozy_server_protocol::framing::Answered;
    use smoozy_server_protocol::requests::{
        IT_WAS_NOT_CLEARED, THE_MACHINES_OWN_WORDS_FIELD, THE_STOP_FAILED,
    };
    use smoozy_state::outcomes::{Recorded, Refused};
    use std::cell::RefCell;
    use std::rc::Rc;

    /// **A keeper that holds its records in a list** and writes every act it
    /// is asked into the one log the stand-in machine writes to, so the order
    /// of the two is read off one list.
    struct AKeeper {
        records: Vec<Recorded>,
        log: Rc<RefCell<Vec<String>>>,
        refuses: bool,
    }

    impl AKeeper {
        fn over(log: &Rc<RefCell<Vec<String>>>) -> Self {
            AKeeper { records: Vec::new(), log: log.clone(), refuses: false }
        }
        fn the_one(&self) -> &Recorded {
            assert_eq!(self.records.len(), 1, "one record for one act");
            &self.records[0]
        }
        fn outcome_of(&self, unit: &str) -> Option<UnitOutcome> {
            self.the_one()
                .units
                .iter()
                .find(|(owed, _)| owed == unit)
                .and_then(|(_, outcome)| outcome.clone())
        }
    }

    impl Keeps for AKeeper {
        fn enumerate(&mut self, action: &str, units: &[&str]) -> Result<Identity, Refused> {
            if self.refuses {
                return Err(Refused::TheStore("the disk is full".into()));
            }
            self.log.borrow_mut().push(format!("the record: {}", units.join(", ")));
            let identity = smoozy_state::outcomes::issue().map_err(|_| Refused::TheStore("no randomness".into()))?;
            self.records.push(Recorded {
                identity: identity.clone(),
                action: action.to_string(),
                units: units.iter().map(|unit| (unit.to_string(), None)).collect(),
            });
            Ok(identity)
        }
        fn a_unit_ended(&mut self, record: &Identity, unit: &str, outcome: UnitOutcome) -> Result<(), Refused> {
            self.log.borrow_mut().push(format!("{unit}: {outcome:?}"));
            let held = self.records.iter_mut().find(|held| held.identity == *record).ok_or(Refused::ThatUnitWasNotOwed)?;
            let entry = held.units.iter_mut().find(|(owed, _)| owed == unit).ok_or(Refused::ThatUnitWasNotOwed)?;
            if entry.1.is_some() {
                return Err(Refused::ThatUnitHasAlreadyEnded);
            }
            entry.1 = Some(outcome);
            Ok(())
        }
        fn read(&self, record: &Identity) -> Result<Option<Recorded>, Refused> {
            Ok(self.records.iter().find(|held| held.identity == *record).cloned())
        }
        fn the_user_removed_it(&mut self, record: &Identity) -> Result<(), Refused> {
            self.records.retain(|held| held.identity != *record);
            Ok(())
        }
    }

    /// **A stand-in supervisor answering by identity**, the way S1's program
    /// does: the first half to arrive records both, a repeat of an identity is
    /// answered from the record, and the stop's outcome lands after a number
    /// of asks.
    pub(crate) struct ASupervisor {
        log: Rc<RefCell<Vec<String>>>,
        /// What the clear comes to: a word, or how the machine meets it.
        the_clear: Script,
        /// What the stop comes to, and after how many asks of its identity.
        the_stop: Script,
        stop_lands_after: usize,
        /// Every request, as asked.
        pub(crate) asked: Vec<Request>,
        stop_asks: usize,
    }

    #[derive(Clone)]
    pub(crate) enum Script {
        Word(&'static str),
        /// The program does not know the request — 0.1.0 to 0.1.6.
        RefusedByName,
        /// Recorded and never finished (a clear refused part-way).
        InHand,
        /// The command ran and nothing correlated came back, every time.
        Silence,
        /// The command ran and nothing came back the first time only.
        LostOnce,
    }

    impl ASupervisor {
        pub(crate) fn new(log: &Rc<RefCell<Vec<String>>>, the_clear: Script, the_stop: Script, after: usize) -> Self {
            ASupervisor {
                log: log.clone(),
                the_clear,
                the_stop,
                stop_lands_after: after,
                asked: Vec::new(),
                stop_asks: 0,
            }
        }
        pub(crate) fn how_many(&self, name: &str) -> usize {
            self.asked.iter().filter(|asked| asked.name == name).count()
        }
    }

    fn answered(request: &Request, fields: Fields) -> WhatCameBack {
        WhatCameBack::ItAnswered(Answered::to(request, THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS, "test").carrying(fields))
    }

    impl AsksTheSupervisor for ASupervisor {
        fn ask(&mut self, request: &Request) -> WhatCameBack {
            self.log.borrow_mut().push(format!("asked: {}", request.name));
            // **`Server protocol and transfers` C9, held by the stand-in**: a
            // half asked again carries the identity it was first asked under.
            if let Some(first) = self.asked.iter().find(|asked| asked.name == request.name) {
                assert_eq!(first.identity, request.identity, "a fresh identity was sent for a half already asked");
            }
            self.asked.push(request.clone());
            if request.name == THE_TELEGRAM_ROLE_IS_DISABLED_REQUEST {
                return answered(request, Fields::none().and(WHAT_THE_ROLE_WORKER_DID_FIELD, "signalled"));
            }
            let is_the_clear = request.name == THE_CLEAR_REQUEST;
            let script = if is_the_clear { self.the_clear.clone() } else { self.the_stop.clone() };
            match script {
                Script::RefusedByName => {
                    return WhatCameBack::ItAnswered(
                        Answered::to(request, THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS, "0.1.6")
                            .saying_it_does_not_answer_that_request(&request.name),
                    )
                }
                Script::Silence => return WhatCameBack::NothingCorrelatedCameBack,
                // It arrived; the answer was lost.
                Script::LostOnce if self.how_many(&request.name) == 1 => {
                    return WhatCameBack::NothingCorrelatedCameBack
                }
                _ => {}
            }
            // **Each half names the other** (S1).
            if is_the_clear {
                assert!(request.fields.text(THE_PAIRED_STOP_FIELD).is_some(), "the clear names no stop");
            } else {
                assert!(request.fields.text(THE_PAIRED_CLEAR_FIELD).is_some(), "the stop names no clear");
            }
            let clears_word = match &self.the_clear {
                Script::Word(word) => Some(*word),
                Script::LostOnce => Some(IT_WAS_CLEARED),
                _ => None,
            };
            if is_the_clear {
                return match clears_word {
                    Some(word) => answered(
                        request,
                        Fields::none()
                            .and(WHAT_BECAME_OF_IT_FIELD, IT_HAS_A_RECORDED_OUTCOME)
                            .and(THE_CLEARS_OUTCOME_FIELD, word),
                    ),
                    None => answered(request, Fields::none().and(WHAT_BECAME_OF_IT_FIELD, IT_IS_STILL_IN_HAND)),
                };
            }
            self.stop_asks += 1;
            if clears_word == Some(IT_WAS_NOT_CLEARED) {
                return answered(
                    request,
                    Fields::none()
                        .and(WHAT_BECAME_OF_IT_FIELD, IT_HAS_A_RECORDED_OUTCOME)
                        .and(THE_STOPS_OUTCOME_FIELD, THE_STOP_WAS_NOT_ATTEMPTED),
                );
            }
            match &self.the_stop {
                Script::Word(word) if self.stop_asks > self.stop_lands_after => answered(
                    request,
                    Fields::none()
                        .and(WHAT_BECAME_OF_IT_FIELD, IT_HAS_A_RECORDED_OUTCOME)
                        .and(THE_STOPS_OUTCOME_FIELD, *word),
                ),
                _ => answered(
                    request,
                    Fields::none()
                        .and(WHAT_BECAME_OF_IT_FIELD, IT_IS_STILL_IN_HAND)
                        .and(THE_MACHINES_OWN_WORDS_FIELD, "the stop is in hand"),
                ),
            }
        }
    }

    pub(crate) const QUICK: TheWait = TheWait { deadline: Duration::from_millis(200), pause: Duration::from_millis(1) };

    /// **A waiter shortened for the desk**: one second, asked again a fifth
    /// of a second apart — so the fifth ask begins just before the deadline.
    /// The dead-line sweep's two-minute-wait repair.
    pub(crate) const SHORTENED: TheWait =
        TheWait { deadline: Duration::from_millis(1_000), pause: Duration::from_millis(200) };

    /// **How far past its deadline a waiter may answer and still be on time**:
    /// the moment the last ask takes to be given up, on a loaded desk.
    pub(crate) const ON_TIME: Duration = Duration::from_millis(600);

    /// **A synchronous waiter run on a thread of its own and watched for
    /// `bound`** — so a waiter that never returns fails its test rather than
    /// hanging it. **The thread is left behind where it never returns**; it
    /// ends with the stand-in point it waits on.
    pub(crate) async fn watched_for<T: Send + 'static>(
        bound: Duration,
        wait: impl FnOnce() -> T + Send + 'static,
    ) -> Option<(T, Duration)> {
        let began = Instant::now();
        let (tell, told) = std::sync::mpsc::channel();
        std::thread::spawn(move || {
            let _ = tell.send(wait());
        });
        loop {
            if let Ok(came) = told.try_recv() {
                return Some((came, began.elapsed()));
            }
            if began.elapsed() >= bound {
                return None;
            }
            tokio::time::sleep(Duration::from_millis(10)).await;
        }
    }

    /// **The machine's supervisor, asked through the application's own point**
    /// over the point's stand-in — the route every queue act takes
    /// (`crate::surface::servers::TheRemovalsOwnEnds::the_machine`). Its
    /// supervisor answers every ask with nothing C8 names, which a waiter
    /// reads as *still in hand*.
    pub(crate) fn the_machine_over(
        point: &crate::servers::sessions::tests::APointLending,
    ) -> crate::servers::the_machines_work::OverThisServersPoint<crate::servers::sessions::tests::AConnectionToIt> {
        crate::servers::the_machines_work::OverThisServersPoint {
            sessions: point.sessions.clone(),
            package_area: "/nowhere/smoozy-package-area".to_string(),
        }
    }

    /// **`Queue & History` L13 to `Queue & History` L17 under the dead-line sweep's two-minute-wait repair:
    /// the pair's wait ends at its own deadline, and no half is asked once it
    /// has passed.** A shortened waiter; the clear's ask begun just before the
    /// deadline meets a silent line.
    ///
    /// **The case it bites on.** Until the repair the deadline was looked at
    /// only after the stop's ask returned, and each ask was made whole: the
    /// clear's ask on the silent line waited for as long as the line said
    /// nothing — a whole break window on a rental, and for good here — and the
    /// stop was then sent after the deadline had passed.
    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn the_pairs_wait_ends_at_its_own_deadline_when_an_ask_meets_a_silent_line() {
        use crate::servers::sessions::tests::{a_point_lending, additional_connections_to, AMachine};
        use std::sync::atomic::Ordering;
        let machine = AMachine::with_a_limit_of(10);
        let point = a_point_lending(&machine, additional_connections_to(&machine, false)).await;
        // **Clear, stop, a pause of 0.3 s; three times; then the clear's
        // fourth ask**, the seventh ask, begun at about 0.9 s — just before
        // the deadline — meets the silent line. The stop after it would be
        // begun past the deadline.
        machine.the_supervisor_is_silent_from.store(7, Ordering::SeqCst);
        let mut over = the_machine_over(&point);
        let wait = TheWait { deadline: Duration::from_millis(1_000), pause: Duration::from_millis(300) };
        let began = Instant::now();
        let came = watched_for(Duration::from_secs(10), move || {
            let log = Rc::new(RefCell::new(Vec::new()));
            let mut keeper = AKeeper::over(&log);
            let went = the_pair_is_sent(&mut keeper, &mut over, &WhatThePairCarries::default(), &[], wait);
            (went.came_to, keeper.outcome_of(THE_CLEAR), keeper.outcome_of(THE_STOP))
        })
        .await;
        let ((came_to, clear, stop), took) = came.expect("the pair's wait went on past its deadline");
        assert!(took < wait.deadline + ON_TIME, "the pair's wait took {took:?}");
        assert_eq!(came_to, WhatThePairCameTo::NothingCameBack);
        assert_eq!((clear, stop), (None, None), "the waiter's expiry wrote an outcome");
        let asked_at = machine.supervisor_asked_at.lock().unwrap().clone();
        assert!(
            asked_at.iter().all(|at| at.duration_since(began) < wait.deadline),
            "an ask was begun after the deadline: {:?}",
            asked_at.iter().map(|at| at.duration_since(began)).collect::<Vec<_>>()
        );
        assert_eq!(asked_at.len(), 7, "the silent ask was never met");
        assert!(point.sessions.a_connection_is_live());
        point.ends().await;
    }

    fn sent(keeper: &mut AKeeper, machine: &mut ASupervisor) -> ThePairWent {
        the_pair_is_sent(keeper, machine, &WhatThePairCarries::default(), &[], QUICK)
    }

    /// **`Queue & History` L17 and `Servers` W22: the record is written before
    /// the first request, and an interruption loses the saying and never the
    /// fact.** The machine here takes the clear and the line then goes silent
    /// for the whole wait: the clear's outcome is written, and the stop's is
    /// left unwritten — the waiter gave up, which writes nothing (`Server
    /// protocol and transfers` C17).
    #[test]
    fn the_record_is_written_before_the_first_request_and_a_silence_writes_no_outcome() {
        let log = Rc::new(RefCell::new(Vec::new()));
        let mut keeper = AKeeper::over(&log);
        let mut machine = ASupervisor::new(&log, Script::Word(IT_WAS_CLEARED), Script::InHand, usize::MAX);
        let went = sent(&mut keeper, &mut machine);
        assert_eq!(went.came_to, WhatThePairCameTo::TheRenderHadNotStopped);
        let log = log.borrow();
        assert_eq!(log[0], "the record: the clear, the stop", "{log:?}");
        assert_eq!(log[1], format!("asked: {THE_CLEAR_REQUEST}"), "{log:?}");
        assert_eq!(keeper.outcome_of(THE_CLEAR), Some(UnitOutcome::Done));
        assert_eq!(keeper.outcome_of(THE_STOP), None, "the waiter's expiry wrote an outcome");
        assert_eq!(went.the_record.as_ref(), Some(&keeper.the_one().identity));

        // **And where nothing that answers the clear ever comes back**, neither
        // half is written: the machine may hold the pair.
        let log = Rc::new(RefCell::new(Vec::new()));
        let mut keeper = AKeeper::over(&log);
        let mut machine = ASupervisor::new(&log, Script::Silence, Script::Silence, 0);
        assert_eq!(sent(&mut keeper, &mut machine).came_to, WhatThePairCameTo::NothingCameBack);
        assert_eq!(keeper.outcome_of(THE_CLEAR), None);
        assert_eq!(keeper.outcome_of(THE_STOP), None);
        // **The stop went too**: it names the clear, so a machine the clear's
        // answer was lost from carries the whole pair out on its arrival.
        assert!(machine.how_many(THE_STOP_PAIRED_WITH_A_CLEAR_REQUEST) >= 1, "the stop was never sent");
    }

    /// **Where the record will not write, nothing is sent.**
    #[test]
    fn a_record_that_will_not_write_sends_nothing() {
        let log = Rc::new(RefCell::new(Vec::new()));
        let mut keeper = AKeeper::over(&log);
        keeper.refuses = true;
        let mut machine = ASupervisor::new(&log, Script::Word(IT_WAS_CLEARED), Script::Word(IT_WAS_LEFT_STOPPED), 0);
        let went = sent(&mut keeper, &mut machine);
        assert_eq!(went, ThePairWent { came_to: WhatThePairCameTo::NothingWasChanged, the_record: None });
        assert!(machine.asked.is_empty(), "a pair was sent with no record behind it");
    }

    /// **Cleared and stopped**: the clear carries its stop and the stop its
    /// clear, the stop is asked again by its own identity while it is in
    /// hand, and both outcomes are written as they land.
    #[test]
    fn cleared_and_stopped_is_learned_by_asking_the_stops_own_identity() {
        let log = Rc::new(RefCell::new(Vec::new()));
        let mut keeper = AKeeper::over(&log);
        let mut machine = ASupervisor::new(&log, Script::Word(IT_WAS_CLEARED), Script::Word(IT_WAS_LEFT_STOPPED), 3);
        let went = sent(&mut keeper, &mut machine);
        assert_eq!(went.came_to, WhatThePairCameTo::ClearedAndStopped);
        assert_eq!(keeper.outcome_of(THE_CLEAR), Some(UnitOutcome::Done));
        assert_eq!(keeper.outcome_of(THE_STOP), Some(UnitOutcome::Done));
        assert_eq!(machine.how_many(THE_CLEAR_REQUEST), 1);
        assert_eq!(machine.how_many(THE_STOP_PAIRED_WITH_A_CLEAR_REQUEST), 4, "the stop was not asked again while in hand");
        let stops: Vec<&Identity> = machine
            .asked
            .iter()
            .filter(|asked| asked.name == THE_STOP_PAIRED_WITH_A_CLEAR_REQUEST)
            .map(|asked| &asked.identity)
            .collect();
        assert!(stops.windows(2).all(|two| two[0] == two[1]), "the stop was asked under a fresh identity");
        let clear = &machine.asked[0];
        assert_eq!(clear.fields.text(THE_PAIRED_STOP_FIELD), Some(stops[0].as_text().as_str()));
        assert_eq!(
            machine.asked[1].fields.text(THE_PAIRED_CLEAR_FIELD),
            Some(clear.identity.as_text().as_str())
        );
        assert!(clear.fields.text(THE_GROUP_FIELD).is_none(), "a removal's clear is the whole machine's");

        // **A render that ended on its own before the interrupt** is the stop
        // done as well: the line is empty and nothing renders.
        let log = Rc::new(RefCell::new(Vec::new()));
        let mut keeper = AKeeper::over(&log);
        let mut machine = ASupervisor::new(&log, Script::Word(IT_WAS_CLEARED), Script::Word(ITS_OWN_ENDING), 0);
        assert_eq!(sent(&mut keeper, &mut machine).came_to, WhatThePairCameTo::ClearedAndStopped);
    }

    /// **`Servers` W21's endings, each from what the machine said.**
    #[test]
    fn each_ending_is_read_off_what_the_machine_said() {
        let run = |clear: Script, stop: Script| {
            let log = Rc::new(RefCell::new(Vec::new()));
            let mut keeper = AKeeper::over(&log);
            let mut machine = ASupervisor::new(&log, clear, stop, 0);
            let went = sent(&mut keeper, &mut machine);
            (went.came_to, keeper.outcome_of(THE_CLEAR), keeper.outcome_of(THE_STOP), machine)
        };
        // **An older program refuses by name: nothing changed, and the stop is
        // not sent** (SC-2).
        let (came_to, clear, stop, machine) = run(Script::RefusedByName, Script::RefusedByName);
        assert_eq!(came_to, WhatThePairCameTo::NothingWasChanged);
        assert_eq!((clear, stop), (Some(UnitOutcome::Failed), Some(the_stops_unit_where_the_clear_was_refused())));
        assert_eq!(machine.how_many(THE_STOP_PAIRED_WITH_A_CLEAR_REQUEST), 0);

        // **`Queue & History` L16: the clear's write refused, the stop not
        // attempted, naming the clear.**
        let (came_to, clear, stop, _) = run(Script::Word(IT_WAS_NOT_CLEARED), Script::Word(IT_WAS_LEFT_STOPPED));
        assert_eq!(came_to, WhatThePairCameTo::TheClearWasRefused);
        assert_eq!((clear, stop), (Some(UnitOutcome::Failed), Some(the_stops_unit_where_the_clear_was_refused())));

        // **A clear refused part-way stays in hand**, and is said the same way,
        // with nothing written.
        let (came_to, clear, stop, _) = run(Script::InHand, Script::InHand);
        assert_eq!(came_to, WhatThePairCameTo::TheClearWasRefused);
        assert_eq!((clear, stop), (None, None));

        // **The stop failed**: cleared, and the stop's own outcome written.
        let (came_to, clear, stop, _) = run(Script::Word(IT_WAS_CLEARED), Script::Word(THE_STOP_FAILED));
        assert_eq!(came_to, WhatThePairCameTo::TheRenderHadNotStopped);
        assert_eq!((clear, stop), (Some(UnitOutcome::Done), Some(UnitOutcome::Failed)));
    }

    /// **`Server protocol and transfers` C9: a lost answer is asked again by
    /// the same identity**, and never re-sent under a fresh one — the stand-in
    /// fails the test where a fresh identity arrives for a half it holds.
    #[test]
    fn a_lost_answer_is_asked_again_by_the_same_identity() {
        let log = Rc::new(RefCell::new(Vec::new()));
        let mut keeper = AKeeper::over(&log);
        let mut machine = ASupervisor::new(&log, Script::LostOnce, Script::Word(IT_WAS_LEFT_STOPPED), 0);
        let went = sent(&mut keeper, &mut machine);
        assert_eq!(went.came_to, WhatThePairCameTo::ClearedAndStopped);
        let clears: Vec<&Identity> = machine
            .asked
            .iter()
            .filter(|asked| asked.name == THE_CLEAR_REQUEST)
            .map(|asked| &asked.identity)
            .collect();
        assert!(clears.len() >= 2, "the lost answer was not asked again");
        assert!(clears.windows(2).all(|two| two[0] == two[1]));
    }

    /// **A machine nobody can ask changes nothing, and says so.**
    #[test]
    fn a_machine_nobody_can_ask_is_nothing_changed() {
        let log = Rc::new(RefCell::new(Vec::new()));
        let mut keeper = AKeeper::over(&log);
        let went = the_pair_is_sent(&mut keeper, &mut NoMachineToAsk, &WhatThePairCarries::default(), &[TELEGRAMS_ROLE_OFF], QUICK);
        assert_eq!(went.came_to, WhatThePairCameTo::NothingWasChanged);
        assert_eq!(keeper.outcome_of(THE_CLEAR), Some(UnitOutcome::Failed));
        assert_eq!(keeper.outcome_of(TELEGRAMS_ROLE_OFF), None, "the caller's own unit was written here");
        assert_eq!(
            keeper.the_one().units.iter().map(|(unit, _)| unit.as_str()).collect::<Vec<_>>(),
            vec![THE_CLEAR, THE_STOP, TELEGRAMS_ROLE_OFF]
        );
    }

    /// **`Servers` W24's request, and what counts as done.**
    #[test]
    fn telegrams_role_off_is_done_only_where_the_supervisor_says_what_it_did() {
        let log = Rc::new(RefCell::new(Vec::new()));
        let mut machine = ASupervisor::new(&log, Script::RefusedByName, Script::RefusedByName, 0);
        assert_eq!(the_telegram_role_is_switched_off(&mut machine), UnitOutcome::Done);
        assert_eq!(machine.how_many(THE_TELEGRAM_ROLE_IS_DISABLED_REQUEST), 1);
        assert_eq!(the_telegram_role_is_switched_off(&mut NoMachineToAsk), UnitOutcome::Failed);
    }
}
