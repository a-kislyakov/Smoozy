//! The idle half hour — `Queue & History` §12 W1–W3, W5, W5a, and W1a's
//! refusal.
//!
//! **Constraint:** Row 55. `Desktop app and delivery` T9 and T14 — the idle
//! kind is this document's reading and its half hour; `State, identity and
//! recovery` D5 — no arithmetic across clocks; the notification itself is
//! `Desktop app and delivery`'s.
//!
//! **What is here**: the idle judgement, the quiet moment's selection and
//! its floor, the overdue arithmetic on the machine's clock, the mark's
//! shape and its one use, and the heading that the mark does not bound.
//! **What is the acting half's**: W4's re-check — the desktop timer that
//! schedules it and the fresh reading it re-takes — and the telling.
//!
//! **Markers used in this file that belong to another document, each with the
//! document it belongs to.** `Rented-server runtime` U5, U6. `Desktop app
//! and delivery` T9, T14.

use std::time::Duration;

use smoozy_state::identities::{Identity, IdentityKind};
use smoozy_state::lifetimes::Lifetime;
use smoozy_state::observed::Reading;

use crate::line::{the_line, JobRecord};

/// **W1: a machine is idle when it is connected, no render operation is
/// running, and its line holds nothing that can start** — an empty line, or
/// only held jobs, which cannot start until a person answers
/// (`Rented-server runtime` U5). **Untracked work counts as work**: the line
/// is the machine's own records, tracked or not. **A transfer still running
/// delays nothing here**, and there is no transfer condition to add.
pub fn is_idle(
    connected: bool,
    a_render_operation_is_running: bool,
    records: &[JobRecord],
) -> bool {
    connected && !a_render_operation_is_running && the_line(records).is_empty()
}

/// **W1a: *idle* is a present condition throughout this document and no
/// figure says when a machine will become one.** The mockup's *idle in 12
/// min* on a server heading is refused rather than supplied, on B9's ground
/// plus three the smaller figure never had to meet: a job that has not
/// started has no operand at all, the line is not a fixed quantity, and the
/// harm is larger on the one heading that stands for the machine. **A type
/// with no values, so nothing composes one**; the drawn value goes back to
/// Design to be parked, and nothing is built for it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhenAMachineWillFallQuiet {}

/// The half hour, as the spec fixes it.
pub const THE_HALF_HOUR: Duration = Duration::from_secs(30 * 60);

/// One recorded event on the machine, as W2 reads it: its identity, its
/// moment on the machine's clock, and whether it left the line without
/// startable work.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ARecordedEvent {
    pub identity: Identity,
    pub at: Reading<()>,
    /// A recorded ending, a clear's outcome, a drop's outcome, a cancel's
    /// removal, an attempt that placed a hold and ended without rendering
    /// (`Rented-server runtime` U6) — whichever left the line without
    /// startable work. The list is examples; the rule is general.
    pub left_the_line_without_startable_work: bool,
}

/// **W2: the quiet moment is the machine's own, read fresh, never
/// remembered** — the most recent recorded event that left the line without
/// startable work — **and the stretch's key.** Where no such event exists
/// the floor is stated rather than left to a builder: the stretch is keyed
/// on the installation identity with a null opening-event identity, and the
/// quiet moment is the earliest event the machine's journal retains. The
/// key and the moment are split deliberately.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AQuietStretch {
    pub key: StretchKey,
    /// On the machine's clock.
    pub quiet_since: Reading<()>,
}

/// **W5's key**: the installation identity beside the opening event's
/// identity — stable across reboots and the journal's cap, which neither the
/// supervisor's start nor the earliest retained event would be.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StretchKey {
    pub installation: Identity,
    pub opening_event: Option<Identity>,
}

/// W2, performed over the machine's retained events, which are read on
/// request while connected and held durably nowhere.
pub fn the_quiet_stretch(
    installation: &Identity,
    retained: &[ARecordedEvent],
) -> Option<AQuietStretch> {
    debug_assert_eq!(installation.kind(), IdentityKind::Installation);
    let opening = retained
        .iter()
        .filter(|event| event.left_the_line_without_startable_work)
        .max_by_key(|event| event.at.taken_at());
    if let Some(event) = opening {
        return Some(AQuietStretch {
            key: StretchKey {
                installation: installation.clone(),
                opening_event: Some(event.identity.clone()),
            },
            quiet_since: event.at.clone(),
        });
    }
    let earliest = retained.iter().min_by_key(|event| event.at.taken_at())?;
    Some(AQuietStretch {
        key: StretchKey {
            installation: installation.clone(),
            opening_event: None,
        },
        quiet_since: earliest.at.clone(),
    })
}

/// **W3: the elapsed half hour is computed on the machine's clock alone.**
/// Overdue is machine-now minus quiet-moment, both taken from the machine in
/// one reading. `None` where the two are on different clocks — the type
/// refuses it — or the clock moved.
pub fn how_long_quiet(machine_now: &Reading<()>, stretch: &AQuietStretch) -> Option<Duration> {
    machine_now.since(&stretch.quiet_since)
}

/// **W3: a connection that finds a machine which went quiet an hour ago finds
/// it long overdue and says so at once**, rather than starting the wait over.
pub fn is_past_the_half_hour(quiet_for: Duration) -> bool {
    quiet_for >= THE_HALF_HOUR
}

/// **W4's arithmetic**: the remaining interval the desktop's timer schedules
/// the re-check at — machine-clock arithmetic, with the claim re-taken from
/// the machine at that moment. `None` where already past.
pub fn the_remaining_interval(quiet_for: Duration) -> Option<Duration> {
    (quiet_for < THE_HALF_HOUR).then(|| THE_HALF_HOUR - quiet_for)
}

/// **W5: the idle-spoken mark** — one small record in the local record,
/// subject-held on the server row, carrying the installation identity and
/// the event identity of the recorded event that opened the stretch. Smoozy's
/// own note of Smoozy's own act.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IdleSpokenMark {
    pub server: Identity,
    pub stretch: StretchKey,
}

impl IdleSpokenMark {
    pub const LIFETIME: Lifetime = Lifetime::SubjectHeld;
}

/// **W5: speaks once per idle stretch, across restarts, by the mark.** Before
/// speaking, the mark is looked up; present for this stretch means already
/// spoken. A machine that takes work and falls quiet again has a new opening
/// event, a new stretch, and speaks again.
pub fn may_speak(marks: &[IdleSpokenMark], server: &Identity, stretch: &AQuietStretch) -> bool {
    !marks
        .iter()
        .any(|mark| &mark.server == server && mark.stretch == stretch.key)
}

/// **W5a: what *once per stretch* bounds is the telling and nothing drawn.**
/// The heading is a composition over W1 to W3's readings, redrawn like any
/// other reading, and the mark has no place in it — a machine idle for six
/// hours is told about once and says *idle for six hours* on its heading
/// throughout. **This takes no mark**, which is the rule kept by the
/// signature.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TheHeading {
    Idle {
        quiet_for: Duration,
    },
    /// **Idle, with the length not known.** W1 answers *idle* from the line
    /// and the render operation alone; W3's length is a separate reading and
    /// is absent where the two moments are on different clocks or the clock
    /// moved. **Drawing that case as *not idle* states the opposite of the
    /// truth about a machine the user is being charged for**, so it is its own
    /// value.
    ///
    /// **The words are Design's, not this crate's** — every entry value's
    /// wording is, and this module composes values only. The wording is owed
    /// to `Preparation/Design/Queue & History.md`.
    IdleWithTheLengthNotKnown,
    NotIdle,
}

pub fn the_heading(idle: bool, quiet_for: Option<Duration>) -> TheHeading {
    match (idle, quiet_for) {
        (true, Some(quiet_for)) => TheHeading::Idle { quiet_for },
        (true, None) => TheHeading::IdleWithTheLengthNotKnown,
        (false, _) => TheHeading::NotIdle,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::line::testing::{a_line_of, an_operation};
    use crate::line::QueueState;
    use smoozy_state::observed::{Clock, MachineClock};
    use std::time::SystemTime;

    fn machine() -> Clock {
        Clock::AMachine(MachineClock("server-7".to_string()))
    }

    fn at(seconds: u64) -> Reading<()> {
        Reading::taken(
            (),
            SystemTime::UNIX_EPOCH + Duration::from_secs(seconds),
            machine(),
        )
    }

    fn an_installation() -> Identity {
        Identity::issue(IdentityKind::Installation).unwrap()
    }

    #[test]
    fn a_machine_is_idle_only_connected_with_no_render_and_nothing_startable() {
        let empty: Vec<JobRecord> = Vec::new();
        assert!(is_idle(true, false, &empty));
        assert!(!is_idle(false, false, &empty));
        assert!(!is_idle(true, true, &empty));
        // Only held jobs: nothing can start, so idle.
        let mut held = a_line_of(1);
        held[0].state = QueueState::Held {
            hold: an_operation(),
        };
        held[0].sequence_key = None;
        assert!(is_idle(true, false, &held));
        // A waiting job, tracked or not, is work.
        assert!(!is_idle(true, false, &a_line_of(1)));
    }

    #[test]
    fn the_quiet_moment_is_the_latest_event_that_left_the_line_without_startable_work() {
        let installation = an_installation();
        let ending = an_operation();
        let retained = vec![
            ARecordedEvent {
                identity: an_operation(),
                at: at(100),
                left_the_line_without_startable_work: true,
            },
            ARecordedEvent {
                identity: an_operation(),
                at: at(500),
                left_the_line_without_startable_work: false,
            },
            ARecordedEvent {
                identity: ending.clone(),
                at: at(400),
                left_the_line_without_startable_work: true,
            },
        ];
        let stretch = the_quiet_stretch(&installation, &retained).unwrap();
        assert_eq!(stretch.key.opening_event, Some(ending));
        assert_eq!(stretch.quiet_since, at(400));
    }

    #[test]
    fn where_no_such_event_exists_the_floor_keys_on_the_installation_and_dates_from_the_earliest_retained(
    ) {
        let installation = an_installation();
        let retained = vec![
            ARecordedEvent {
                identity: an_operation(),
                at: at(900),
                left_the_line_without_startable_work: false,
            },
            ARecordedEvent {
                identity: an_operation(),
                at: at(300),
                left_the_line_without_startable_work: false,
            },
        ];
        let stretch = the_quiet_stretch(&installation, &retained).unwrap();
        assert_eq!(stretch.key.opening_event, None);
        assert_eq!(stretch.key.installation, installation);
        assert_eq!(stretch.quiet_since, at(300));
        assert_eq!(the_quiet_stretch(&installation, &[]), None);
    }

    #[test]
    fn overdue_is_machine_clock_arithmetic_and_refuses_the_desktops_clock() {
        let installation = an_installation();
        let stretch = the_quiet_stretch(
            &installation,
            &[ARecordedEvent {
                identity: an_operation(),
                at: at(1_000),
                left_the_line_without_startable_work: true,
            }],
        )
        .unwrap();
        let quiet_for = how_long_quiet(&at(1_000 + 3_600), &stretch).unwrap();
        assert_eq!(quiet_for, Duration::from_secs(3_600));
        assert!(is_past_the_half_hour(quiet_for));
        assert_eq!(the_remaining_interval(quiet_for), None);
        let short = Duration::from_secs(600);
        assert_eq!(
            the_remaining_interval(short),
            Some(Duration::from_secs(1_200))
        );
        // The desktop's clock: refused by the type.
        let here = Reading::taken(
            (),
            SystemTime::UNIX_EPOCH + Duration::from_secs(9_000),
            Clock::ThisComputer,
        );
        assert_eq!(how_long_quiet(&here, &stretch), None);
    }

    #[test]
    fn the_mark_lets_a_stretch_be_spoken_once_and_a_new_stretch_speaks_again() {
        let installation = an_installation();
        let server = Identity::issue(IdentityKind::LocalServer).unwrap();
        let first = the_quiet_stretch(
            &installation,
            &[ARecordedEvent {
                identity: an_operation(),
                at: at(10),
                left_the_line_without_startable_work: true,
            }],
        )
        .unwrap();
        assert!(may_speak(&[], &server, &first));
        let marks = vec![IdleSpokenMark {
            server: server.clone(),
            stretch: first.key.clone(),
        }];
        assert!(!may_speak(&marks, &server, &first));
        let second = the_quiet_stretch(
            &installation,
            &[ARecordedEvent {
                identity: an_operation(),
                at: at(20),
                left_the_line_without_startable_work: true,
            }],
        )
        .unwrap();
        assert!(may_speak(&marks, &server, &second));
        assert_eq!(IdleSpokenMark::LIFETIME, Lifetime::SubjectHeld);
    }

    #[test]
    fn the_heading_says_idle_for_as_long_as_it_is_true_whatever_the_mark_says() {
        assert_eq!(
            the_heading(true, Some(Duration::from_secs(6 * 3_600))),
            TheHeading::Idle {
                quiet_for: Duration::from_secs(6 * 3_600)
            }
        );
        assert_eq!(
            the_heading(false, Some(Duration::from_secs(10))),
            TheHeading::NotIdle
        );
        // **An idle machine whose quiet stretch cannot be measured is idle**,
        // and the heading says so with the length absent. Drawn as *not idle*
        // it stated the opposite of the truth about a machine the user is
        // being charged for — and W3 answers `None` whenever the two moments
        // are on different clocks or the clock moved, which is an ordinary
        // reading rather than a rare one.
        assert_eq!(
            the_heading(true, None),
            TheHeading::IdleWithTheLengthNotKnown
        );
    }

    #[test]
    fn no_figure_for_when_a_machine_will_fall_quiet_can_be_composed() {
        assert_eq!(std::mem::size_of::<Option<WhenAMachineWillFallQuiet>>(), 0);
    }
}
