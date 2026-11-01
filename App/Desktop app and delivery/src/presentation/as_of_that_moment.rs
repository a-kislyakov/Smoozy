//! **Every reading presented anywhere is presented as of the moment it was
//! taken, on the clock it was taken on** — §16.1 M3a.
//!
//! **This file is here rather than in the drawing that first needed one**, and
//! that placement is M3a's own: *"Placement is fixed here rather than at 16.5 …
//! because the rule binds every subsection below and not only the one Row 291
//! answers."* Pieces D, E and F each draw readings, and each would otherwise
//! carry its own answer.
//!
//! ## What it requires, and what it costs
//!
//! **Requires:** every reading this section hands to a surface carries its own
//! moment and the clock that moment is on, and the surface presents it **as of
//! that moment**. A reading is never re-drawn as current because it is still on
//! screen, and it is never aged into a newer figure.
//!
//! **Does not require, because M1 and P9 already forbid it:** nothing takes a
//! fresh reading to keep a figure current, nothing polls to discover that
//! readings have stopped arriving, and no watcher is added anywhere. **M3a
//! fixes what a presented reading claims, not how often it is refreshed**,
//! which is why it costs no reading of its own.
//!
//! **It composes nothing.** Where two readings sit beside each other with
//! different moments, both moments stand. M31's refusal of a third reading
//! composed from two is the same rule and is not widened here.
//!
//! ## Why this is a type
//!
//! The failure M3a was written for is quiet: *"a live connection whose worker
//! has died declares no break, so `Servers` raises nothing and `Queue &
//! History` C7's not-updating mark is never keyed, and a figure sits on
//! screen
//! beside an honest state with nothing saying it is frozen."* **Nothing about
//! that is visible from inside the drawing that shows the figure.** So the
//! moment travels with the value and there is no constructor that does not take
//! one — the same reason `State, identity and recovery`'s own
//! [`smoozy_state::observed::Reading`] is a type, which this wraps rather than
//! replaces.
//!
//! **The words, the format of the moment, and whether it is drawn at all are
//! Design's** (§24). What is fixed here is that a moment exists for every
//! reading and that nothing may present the reading without it.
//!
//! **Markers used in this file that belong to another document, each with the
//! document it belongs to.** `State, identity and recovery` D5.
//!
//! **`Queue & History` C7 is deliberately not listed above and is written out
//! with its document at every site instead** — this plan defines a C7 of its own,
//! so a bare one would resolve to a real and wrong clause.

use std::time::SystemTime;

use smoozy_state::observed::{Clock, Reading};

use crate::typography::Verbatim;

/// **One reading on its way to a surface, with its moment and its clock.**
///
/// The value is a [`Verbatim`] because a reading is a value and not prose — a
/// byte count, a free-space figure, a state word, a version. X7 and X9: the
/// verbatim route performs no substitution of any kind, and a reading that went
/// through the typography point would be a value Smoozy quietly changed.
///
/// **Where a reading is worded — *Reconnecting…*, *Idle* — the wording is prose
/// and travels beside this rather than inside it**, on its own route, exactly as
/// a name does. That is the two-strings-two-routes discipline the folder's head
/// states, and it is why this type holds a value and not a sentence.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct AsOfThatMoment {
    /// The reading itself, by the verbatim route.
    pub value: Verbatim,
    /// **The moment it was taken**, in seconds since the epoch, as a number the
    /// page formats. Never an elapsed time: `State, identity and recovery` D5
    /// forbids arithmetic across two clocks, and an interval computed here
    /// would have already done it.
    pub taken_at: Option<u64>,
    /// **Which clock that moment is on** — `this computer`, or one machine,
    /// named by the server row it was read from. The page draws it; nothing
    /// subtracts across two of them.
    pub on_the_clock: String,
}

impl AsOfThatMoment {
    /// Wrap a reading for a surface. **There is no other constructor**, and
    /// that is the whole enforcement: a value reaching a surface without a
    /// moment has to be built some other way, and there is no other way in this
    /// folder.
    pub fn from(reading: &Reading<String>) -> Self {
        AsOfThatMoment {
            value: Verbatim::new(reading.value().clone()),
            taken_at: reading
                .taken_at()
                .duration_since(SystemTime::UNIX_EPOCH)
                .ok()
                .map(|since| since.as_secs()),
            on_the_clock: the_clocks_name(reading.clock()),
        }
    }
}

/// The clock's written form. **A name and never a comparison** — two of these
/// being different is what `State, identity and recovery` D5 forbids
/// subtracting across, and the page draws the name rather than reasoning about
/// it.
pub fn the_clocks_name(clock: &Clock) -> String {
    match clock {
        Clock::ThisComputer => "this computer".to_string(),
        Clock::AMachine(machine) => machine.0.clone(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use smoozy_state::observed::MachineClock;
    use std::time::Duration;

    fn at(seconds: u64) -> SystemTime {
        SystemTime::UNIX_EPOCH + Duration::from_secs(seconds)
    }

    /// **M3a: the moment and the clock travel with the value.**
    #[test]
    fn a_presented_reading_carries_its_own_moment_and_clock() {
        let reading = Reading::taken(
            "482 GB".to_string(),
            at(1_700_000_000),
            Clock::AMachine(MachineClock("server-7".into())),
        );
        let presented = AsOfThatMoment::from(&reading);
        assert_eq!(presented.value.as_str(), "482 GB");
        assert_eq!(presented.taken_at, Some(1_700_000_000));
        assert_eq!(presented.on_the_clock, "server-7");
    }

    /// **Two readings beside each other keep two moments.** M3a: *"Where two
    /// readings sit beside each other with different moments, both moments
    /// stand"*, and nothing here composes a third.
    #[test]
    fn two_readings_keep_their_own_moments() {
        let here = Reading::taken("connected".to_string(), at(100), Clock::ThisComputer);
        let there = Reading::taken(
            "rendering".to_string(),
            at(60),
            Clock::AMachine(MachineClock("server-7".into())),
        );
        let one = AsOfThatMoment::from(&here);
        let other = AsOfThatMoment::from(&there);
        assert_ne!(one.taken_at, other.taken_at);
        assert_ne!(one.on_the_clock, other.on_the_clock);
    }

    /// **The value takes the verbatim route** — X7 and X9. A reading is a value
    /// and Smoozy does not alter one, so no substitution reaches it.
    #[test]
    fn a_reading_is_never_touched_by_the_typography_point() {
        // A string carrying both of the point's two triggers: a space before an
        // em dash, and a last inter-word space.
        let reading = Reading::taken(
            "9.84 GB — of 24 GB".to_string(),
            at(1),
            Clock::ThisComputer,
        );
        assert_eq!(
            AsOfThatMoment::from(&reading).value.as_str(),
            "9.84 GB — of 24 GB"
        );
    }
}
