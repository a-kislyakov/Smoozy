//! Desired state and observed state — `State, identity and recovery` §3.
//!
//! D1: **every controllable thing has exactly one durable desired state,
//! written by exactly one writer, and separately zero or more observed
//! readings. They are never one field.** D2: **no desired state is ever
//! inferred from an observed reading** — a supervisor that finds a worker gone
//! concludes nothing about what was wanted. That is the whole of what stops
//! the observed Batch A failure, where a restart policy honoured a stop and
//! then reversed it two seconds later.
//!
//! What is here is D5, which is the one rule of the section a type can keep.

use std::time::{Duration, SystemTime};

/// Which clock a reading was taken on — `State, identity and recovery` D5.
///
/// Two constraints force the distinction. **The machine's clock is its own**:
/// observed on a rental, *"The machine is on UTC … It has no idea what time
/// zone anyone using it is in"*, synchronised and within about a hundredth of
/// a second of a real time source on two checks half an hour apart, with drift
/// over longer than half an hour untested. **The user's computer's clock is
/// not defended at all**: Fact 13 came back negative on the half that matters,
/// and `Security and licence` §12.3 rules that the only defence is a record
/// held away from the computer.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Clock {
    /// The user's own computer. Used **for presentation and for nothing a
    /// promise rests on** — with the one exception the Creator ruled on
    /// 2026-08-22, a job's fetched derivatives' age (`crate::lifetimes`).
    ThisComputer,
    /// One machine's own clock, named by the server row it was read from. **A
    /// duration about work on a machine is computed from that machine's own
    /// readings**, which is where a job's times and the idle stretch come
    /// from.
    AMachine(MachineClock),
}

/// Names one machine's clock, so two machines' readings cannot be subtracted
/// from one another either. The value is the local server identifier's text.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MachineClock(pub String);

/// An observed value with the moment it was taken and the clock it was taken
/// on — D5, *"and readings taken on different clocks are never subtracted from
/// one another."*
///
/// The subtraction rule is the reason this is a type. [`Reading::since`]
/// refuses a reading from another clock, so the arithmetic D5 forbids does not
/// compile into a wrong number — it comes back as [`None`] and the caller has
/// to say what it does instead.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Reading<T> {
    value: T,
    taken_at: SystemTime,
    clock: Clock,
}

impl<T> Reading<T> {
    pub fn taken(value: T, at: SystemTime, on: Clock) -> Self {
        Reading {
            value,
            taken_at: at,
            clock: on,
        }
    }

    pub fn value(&self) -> &T {
        &self.value
    }

    pub fn taken_at(&self) -> SystemTime {
        self.taken_at
    }

    pub fn clock(&self) -> &Clock {
        &self.clock
    }

    /// How long after `earlier` this reading was taken.
    ///
    /// **`None` where the two were taken on different clocks**, which is D5's
    /// prohibition kept by the type rather than by a rule somebody remembers.
    /// `None` also where this reading is the earlier of the two, which is a
    /// clock that moved rather than a duration.
    pub fn since<U>(&self, earlier: &Reading<U>) -> Option<Duration> {
        if self.clock != earlier.clock {
            return None;
        }
        self.taken_at.duration_since(earlier.taken_at).ok()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn machine(name: &str) -> Clock {
        Clock::AMachine(MachineClock(name.to_string()))
    }

    #[test]
    fn two_readings_on_one_machines_clock_subtract() {
        let start = SystemTime::UNIX_EPOCH + Duration::from_secs(1_000);
        let first = Reading::taken("started", start, machine("server-7"));
        let second = Reading::taken(
            "finished",
            start + Duration::from_secs(90),
            machine("server-7"),
        );
        assert_eq!(second.since(&first), Some(Duration::from_secs(90)));
    }

    #[test]
    fn a_machines_reading_and_this_computers_never_subtract() {
        let start = SystemTime::UNIX_EPOCH + Duration::from_secs(1_000);
        let on_the_machine = Reading::taken("started", start, machine("server-7"));
        let here = Reading::taken(
            "noticed",
            start + Duration::from_secs(90),
            Clock::ThisComputer,
        );
        assert_eq!(here.since(&on_the_machine), None);
    }

    #[test]
    fn two_machines_readings_never_subtract_either() {
        let start = SystemTime::UNIX_EPOCH + Duration::from_secs(1_000);
        let one = Reading::taken("started", start, machine("server-7"));
        let other = Reading::taken(
            "finished",
            start + Duration::from_secs(90),
            machine("server-8"),
        );
        assert_eq!(other.since(&one), None);
    }

    #[test]
    fn a_clock_that_moved_backwards_gives_no_duration() {
        let later = SystemTime::UNIX_EPOCH + Duration::from_secs(2_000);
        let first = Reading::taken("started", later, Clock::ThisComputer);
        let second = Reading::taken(
            "finished",
            later - Duration::from_secs(90),
            Clock::ThisComputer,
        );
        assert_eq!(second.since(&first), None);
    }
}
