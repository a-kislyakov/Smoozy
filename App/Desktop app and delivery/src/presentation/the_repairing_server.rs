//! **A repair that says when it next tries** — §16.7 M28 to M31a, and the
//! drawing at `Preparation/Design/Mockup/Smoozy.html`'s server list.
//!
//! **Read the folder's head first.** This file draws inside the shape stated
//! there and adds nothing to it.
//!
//! ## Three things and never four
//!
//! **M28: presentation is handed three things about a repairing server and
//! never four.** [`TheThreeReadings`] holds exactly those three and
//! [`tests::the_readings_are_three_and_never_four`] is what says so — it counts
//! the struct's own fields out of the source, so a fourth added later fails
//! there rather than passing quietly.
//!
//! 1. **The connection condition's value**, which is `Servers` K6's closed set
//!    of three and is supplied to whatever draws a server's state.
//! 2. **The moment the next attempt is due** — `Servers` N10a — **run-held,
//!    absent while an attempt is in flight**, and absent for good where the
//!    repair has stopped permanently. **Absent is a value rather than a gap.**
//! 3. **Where the server was working when the break was declared, the
//!    last-known work reading with its own moment**, which `Servers` keeps alive
//!    through a drop and whose marking — the stamp and the not-updating
//!    statement — `Queue & History` C7 governs.
//!
//! **All three go through [`super::as_of_that_moment::AsOfThatMoment`]** —
//! M3a, and piece C put that file in this folder for exactly this reason.
//! `AsOfThatMoment::from` is the only constructor and it takes the moment.
//!
//! ## The word the column says is not the condition's value
//!
//! **M29**, in its own words: *"The word the work column says is not the
//! connection's observed value, and conflating them is what made this look
//! unanswerable … **What the column reads is whether a repair is under way at
//! all**, which is true from the declared break until the period ends, runs and
//! standing pauses alike."*
//!
//! So [`the_word_for_the_repair`] **takes whether a repair is under way and
//! nothing else**. There is no parameter through which the condition's value
//! could reach it, which is the conflation made unavailable rather than
//! discouraged.
//!
//! **M29a: the spec was changed on the Creator's ruling of 2026-08-22 so a
//! server reads as reconnecting for the whole repair.** It used to promise a
//! two-state flicker. **Do not shorten it back**: with attempts fifteen seconds
//! apart a label that alternates cannot be read. The cost, stated by the
//! Creator and accepted with the ruling, is that the user can no longer tell at
//! a glance whether an attempt is in flight at this second. **What is not lost
//! is the saying** — a failed run still raises its record and its
//! could-not-restore notification, because those fire on the condition's value
//! and never on the column's word.
//!
//! ## The countdown is a drawing, and this file draws nothing
//!
//! **M30: the countdown is a drawing and is bounded by P10b.** *"It ticks
//! against M28's moment, takes no reading, makes no request, posts nothing on
//! P6's queue, and ceases to exist when the thing it draws is off screen. Where
//! the moment is absent the line has no countdown and the word stands alone."*
//!
//! **So what is here is the moment and nothing else.** The tick is on the page,
//! in `web/the-underway-shape.js`, and it is the one place P10b's rule about a
//! drawing that ticks is applied rather than quoted. `src/scheduling.rs`
//! already carries P10b as a prohibition on itself, and
//! `src/presentation/mod.rs`'s own `nothing_here_watches_or_ticks` would fail on a timer in
//! this folder — **which is the test working as intended and not an obstacle**.
//!
//! **P10c's hazard rides on it and is named rather than repaired.** A moment on
//! this computer's clock says nothing about a computer that was suspended
//! between the moment being written and the drawing reading it, so a countdown
//! can wake reading long past due. The repair is driven by attempts ending
//! rather than by the clock, so a late attempt is still an attempt.
//!
//! ## Two of the column's four contents can be true at once
//!
//! **M31: a server that was rendering when the connection went owes the column
//! *Rendering* with its job's name **and** *Reconnecting…* at the same moment.
//! Both are supplied, each with its own moment; which occupies the one slot
//! Design drew is Design's.** [`what_the_column_is_owed`] answers with a **list**
//! for that reason.
//!
//! **What is not Design's to invent is a third reading composed from the two**,
//! because the last-known work reading is stale by construction and saying so is
//! Row 323's requirement. **That is kept the way piece C kept M7a**: there is
//! nowhere to put a third. No function in this file takes two contents and
//! answers with one, and the list is the answer rather than an input to one.
//!
//! **Markers used in this file that belong to another document, each with the
//! document it belongs to.** `Servers` N10a, N2, N6, N8b.
//!
//! **Every other document's marker below is written out with that document at
//! the site**, and that is deliberate rather than lazy: this plan defines a
//! C-series, a K-series, an S-series and a W-series of its own, and `Servers`,
//! `Queue & History` and `State, identity and recovery` define clauses of the
//! same short names. A header list cannot tell two apart, so a bare one would
//! resolve to a real and wrong clause.

use crate::presentation::as_of_that_moment::AsOfThatMoment;
use crate::typography::Verbatim;

/// **M28's three, and never four.**
///
/// **Each is a reading and each carries its own moment and clock** — M3a. Two
/// of the three are optional and **an absence is a value rather than a gap**:
/// `Servers` N10a says so of the moment in its own words, and M28 says the work
/// reading exists only *where the server was working when the break was
/// declared*.
///
/// **Nothing here is held.** This struct is composed for the length of one
/// drawing and dropped with it, which is why the condition's value can travel
/// in it without M6's second copy existing anywhere: there is no field on any
/// longer-lived thing in this folder that a value could be kept in, and
/// the folder's own sweep at `src/presentation/mod.rs` is what says so.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct TheThreeReadings {
    /// **One.** `Servers` K6's closed set of three, as supplied. **This is
    /// carried and never drawn as the work column's word** — M29, and the
    /// column's four words are none of K6's three.
    pub the_connection_condition: AsOfThatMoment,
    /// **Two.** `Servers` N10a's moment the next attempt is due. **Run-held,
    /// absent while an attempt is in flight, and absent for good under
    /// `Servers` N8b's permanent refusal.** It covers both pauses and does not
    /// distinguish them — `Servers` N2's fifteen seconds inside a run and
    /// `Servers` N6's sixty between runs are both *when the next attempt
    /// falls*, and whoever draws it never learns which.
    pub the_moment_the_next_attempt_is_due: Option<AsOfThatMoment>,
    /// **Three.** The last-known work reading with its own moment, **where the
    /// server was working when the break was declared**. Its marking — the
    /// stamp and the not-updating statement — is `Queue & History` C7's, and it
    /// is stale by construction, which is what makes saying so a requirement
    /// rather than a courtesy.
    pub the_last_known_work_reading: Option<AsOfThatMoment>,
}

/// **The work column's four contents, and the vocabulary is Design's own.**
///
/// M29: *"The work column's vocabulary is Design's own — *Rendering* with a
/// job's name, *Idle*, *Reconnecting…*, or nothing — and none of those four is
/// one of `Servers` K6's three."*
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub enum WhatTheColumnSays {
    /// *Rendering*, **with the job's name beside it and never joined into it**
    /// — X7, and the folder head's rule 4. The name is a value on the verbatim
    /// route.
    Rendering(Verbatim),
    Idle,
    /// **The whole repair reads as this**, runs and standing pauses alike —
    /// M29a's ruling of 2026-08-22.
    Reconnecting,
    /// *"or nothing"*. A server with nothing to say about work says nothing.
    Nothing,
}

impl WhatTheColumnSays {
    /// The written form, which is the word Design drew. **Prose, because it is
    /// text for a person** — the caller puts it through the typography point on
    /// the way to the screen (X1), and the job's name beside it does not go
    /// with it (X7).
    pub fn the_word(&self) -> &'static str {
        match self {
            WhatTheColumnSays::Rendering(_) => "Rendering",
            WhatTheColumnSays::Idle => "Idle",
            WhatTheColumnSays::Reconnecting => "Reconnecting…",
            WhatTheColumnSays::Nothing => "",
        }
    }
}

/// **M29, performed: what the column reads is whether a repair is under way at
/// all.**
///
/// **It takes one thing and there is no parameter for the condition's value.**
/// That is the whole of how M29's conflation is kept out — *"conflating them is
/// what made this look unanswerable"* — and it is why the column says
/// *Reconnecting…* for the whole repair while `Servers` K6's value goes on
/// moving between *reconnecting* and *disconnected* underneath, where it is
/// supply and nothing draws it.
///
/// **M29a: do not shorten this back to the two-state flicker.** The spec was
/// changed on the Creator's ruling of 2026-08-22 and this is the answer that
/// ruling produced.
pub fn the_word_for_the_repair(a_repair_is_under_way: bool) -> Option<WhatTheColumnSays> {
    if a_repair_is_under_way {
        Some(WhatTheColumnSays::Reconnecting)
    } else {
        None
    }
}

/// **M31: both are supplied and neither is chosen here.**
///
/// *"A server that was rendering when the connection went owes the column
/// *Rendering* with its job's name **and** *Reconnecting…* at the same moment.
/// Both are supplied, each with its own moment; which occupies the one slot
/// Design drew is Design's."*
///
/// **The answer is a list, and that is the point.** A function answering with
/// one of the two would be this document choosing, and one answering with
/// something built out of both would be the third reading M31 forbids — *"the
/// last-known work reading is stale by construction and saying so is Row 323's
/// requirement"*. **There is nowhere here to put a third**, which is how piece
/// C kept M7a and is the property to keep rather than a rule to add.
pub fn what_the_column_is_owed(
    readings: &TheThreeReadings,
    a_repair_is_under_way: bool,
) -> Vec<WhatTheColumnSays> {
    let mut owed = Vec::new();
    if let Some(work) = &readings.the_last_known_work_reading {
        owed.push(WhatTheColumnSays::Rendering(work.value.clone()));
    }
    if let Some(word) = the_word_for_the_repair(a_repair_is_under_way) {
        owed.push(word);
    }
    if owed.is_empty() {
        owed.push(WhatTheColumnSays::Nothing);
    }
    owed
}

/// **One repairing server, as the page receives it.**
///
/// **Composed here and never on the page.** The countdown the page draws ticks
/// against [`TheThreeReadings::the_moment_the_next_attempt_is_due`] and against
/// nothing else — M30 and P10b.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct TheRepairingServerDrawn {
    /// The server row's identity, by the verbatim route.
    pub server_row: Verbatim,
    /// M28's three, unchanged.
    pub readings: TheThreeReadings,
    /// **M31: both contents where both are true**, each on its own, in the
    /// order Design's own drawing has them.
    pub the_column_is_owed: Vec<WhatTheColumnSays>,
}

/// **The one way a drawing composes a repairing server.**
///
/// **Nothing is taken here and nothing is composed from two readings.** The
/// three arrive already taken, each with its own moment (M1, M3a), and what is
/// added is the word Design's vocabulary supplies for whether a repair is under
/// way (M29).
pub fn a_repairing_server(
    server_row: &str,
    readings: TheThreeReadings,
    a_repair_is_under_way: bool,
) -> TheRepairingServerDrawn {
    let the_column_is_owed = what_the_column_is_owed(&readings, a_repair_is_under_way);
    TheRepairingServerDrawn {
        server_row: Verbatim::new(server_row.to_string()),
        readings,
        the_column_is_owed,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use smoozy_state::observed::{Clock, MachineClock, Reading};
    use std::time::{Duration, SystemTime};

    /// This file's working half — everything before the tests.
    ///
    /// **The sweeps below read it rather than the whole file.** The words they
    /// forbid are written out here in order to forbid them, and a check that
    /// read its own list would fail on itself.
    fn the_working_half() -> &'static str {
        include_str!("the_repairing_server.rs")
            .split("#[cfg(test)]")
            .next()
            .expect("the file has a working half")
    }

    fn here(value: &str, at: u64) -> AsOfThatMoment {
        AsOfThatMoment::from(&Reading::taken(
            value.to_string(),
            SystemTime::UNIX_EPOCH + Duration::from_secs(at),
            Clock::ThisComputer,
        ))
    }

    fn there(value: &str, at: u64, machine: &str) -> AsOfThatMoment {
        AsOfThatMoment::from(&Reading::taken(
            value.to_string(),
            SystemTime::UNIX_EPOCH + Duration::from_secs(at),
            Clock::AMachine(MachineClock(machine.to_string())),
        ))
    }

    fn a_repairing_one() -> TheThreeReadings {
        TheThreeReadings {
            the_connection_condition: here("reconnecting", 1_700_000_100),
            the_moment_the_next_attempt_is_due: Some(here("1700000112", 1_700_000_100)),
            the_last_known_work_reading: Some(there("Product spins — 02", 1_700_000_040, "Hetzner box")),
        }
    }

    /// **M28: three things and never four.**
    ///
    /// The fields are counted out of the source rather than out of a list kept
    /// here, so a fourth added later fails here rather than passing quietly.
    #[test]
    fn the_readings_are_three_and_never_four() {
        let source = the_working_half();
        let opens = source
            .find("pub struct TheThreeReadings {")
            .expect("the three are a struct");
        let body = &source[opens..opens + source[opens..].find("\n}").expect("it closes")];
        let fields = body
            .lines()
            .filter(|line| line.trim_start().starts_with("pub the_"))
            .count();
        assert_eq!(
            fields, 3,
            "**M28: presentation is handed three things about a repairing server \
             and never four** — the connection condition's value, the moment the \
             next attempt is due, and, where the server was working when the \
             break was declared, the last-known work reading with its own moment"
        );
    }

    /// **M3a and M28: all three go through `AsOfThatMoment`, whose only
    /// constructor takes the moment.**
    #[test]
    fn every_one_of_the_three_carries_its_own_moment_and_clock() {
        let readings = a_repairing_one();
        assert_eq!(readings.the_connection_condition.taken_at, Some(1_700_000_100));
        assert_eq!(readings.the_connection_condition.on_the_clock, "this computer");
        let work = readings
            .the_last_known_work_reading
            .as_ref()
            .expect("the server was working");
        assert_eq!(work.taken_at, Some(1_700_000_040));
        // **M3a: where two readings sit beside each other with different
        // moments, both moments stand**, and the clocks are not subtracted
        // across.
        assert_eq!(work.on_the_clock, "Hetzner box");
        assert_ne!(
            work.on_the_clock,
            readings.the_connection_condition.on_the_clock
        );
    }

    /// **M29: the column's word is whether a repair is under way, and never
    /// `Servers` K6's value.**
    ///
    /// The column says *Reconnecting…* for the whole repair while K6's value
    /// moves between its two underneath.
    #[test]
    fn the_column_says_reconnecting_for_the_whole_repair() {
        for value in ["reconnecting", "disconnected"] {
            let readings = TheThreeReadings {
                the_connection_condition: here(value, 1_700_000_100),
                the_moment_the_next_attempt_is_due: Some(here("1700000112", 1_700_000_100)),
                the_last_known_work_reading: None,
            };
            let drawn = a_repairing_server("server-7", readings, true);
            assert_eq!(
                drawn.the_column_is_owed,
                vec![WhatTheColumnSays::Reconnecting],
                "**M29a: the spec was changed on the Creator's ruling of \
                 2026-08-22 so a server reads as reconnecting for the whole \
                 repair.** With attempts fifteen seconds apart a label that \
                 alternates cannot be read"
            );
        }
    }

    /// **M29, from the other side: the word's own function has no route to the
    /// condition's value.**
    #[test]
    fn nothing_derives_the_columns_word_from_the_conditions_value() {
        let source = the_working_half();
        let opens = source
            .find("pub fn the_word_for_the_repair(")
            .expect("the word has a function");
        let signature = &source[opens..opens + source[opens..].find(" {").expect("it opens")];
        assert!(
            !signature.contains("the_connection_condition")
                && !signature.contains("AsOfThatMoment")
                && !signature.contains("TheThreeReadings"),
            "`the_word_for_the_repair` can reach the connection condition. \
             **M29: the word the work column says is not the connection's \
             observed value, and conflating them is what made this look \
             unanswerable**"
        );
    }

    /// **M31: two of the column's four contents can be true at once and both
    /// are supplied.**
    #[test]
    fn a_server_that_was_rendering_owes_the_column_both() {
        let drawn = a_repairing_server("server-7", a_repairing_one(), true);
        assert_eq!(
            drawn.the_column_is_owed,
            vec![
                WhatTheColumnSays::Rendering(Verbatim::new("Product spins — 02")),
                WhatTheColumnSays::Reconnecting,
            ]
        );
    }

    /// **M31: nothing composes a third reading out of the two.**
    ///
    /// **Kept the way piece C kept M7a — by there being nowhere to put one.**
    /// Each content is exactly what it would be were it the only one owed, and
    /// no function in this file takes two and answers with one.
    #[test]
    fn nothing_composes_a_third_reading_from_the_two() {
        let both = a_repairing_server("server-7", a_repairing_one(), true).the_column_is_owed;
        let mut alone = a_repairing_one();
        alone.the_last_known_work_reading = None;
        let repairing_only = a_repairing_server("server-7", alone, true).the_column_is_owed;
        assert_eq!(both[1], repairing_only[0]);

        let source = the_working_half();
        for composing in ["fn the_one_reading", "fn combined", "fn merge"] {
            assert!(
                !source.contains(composing),
                "this file composes a reading out of two. **M31: what is not \
                 Design's to invent is a third reading composed from the two**, \
                 because the last-known work reading is stale by construction"
            );
        }
    }

    /// **M30: where the moment is absent the line has no countdown and the word
    /// stands alone**, which is the honest reading of *an attempt is running
    /// right now*. `Servers` N10a: absent is a value rather than a gap.
    #[test]
    fn an_attempt_in_flight_publishes_no_moment_and_the_word_stands_alone() {
        let readings = TheThreeReadings {
            the_connection_condition: here("reconnecting", 1_700_000_100),
            the_moment_the_next_attempt_is_due: None,
            the_last_known_work_reading: None,
        };
        let drawn = a_repairing_server("server-7", readings, true);
        assert!(drawn.readings.the_moment_the_next_attempt_is_due.is_none());
        assert_eq!(
            drawn.the_column_is_owed,
            vec![WhatTheColumnSays::Reconnecting]
        );
    }

    /// **Design's four words, and none of them is `Servers` K6's three.**
    #[test]
    fn the_columns_vocabulary_is_designs_own() {
        let words = [
            WhatTheColumnSays::Rendering(Verbatim::new("Kitchen — final")).the_word(),
            WhatTheColumnSays::Idle.the_word(),
            WhatTheColumnSays::Reconnecting.the_word(),
            WhatTheColumnSays::Nothing.the_word(),
        ];
        assert_eq!(words, ["Rendering", "Idle", "Reconnecting…", ""]);
        for value in ["connected", "reconnecting", "disconnected"] {
            assert!(
                !words.contains(&value),
                "the column draws `Servers` K6's value {value}. **M29: none of \
                 the column's four is one of K6's three**"
            );
        }
    }

    /// **X7: the job's name travels beside the word and is never joined into
    /// it.** A name joined into a sentence before the typography point is a
    /// name with a non-breaking space substituted inside it, and nothing
    /// anywhere reports it.
    #[test]
    fn the_jobs_name_is_never_joined_into_the_word() {
        let content = WhatTheColumnSays::Rendering(Verbatim::new("Work — renders — final one"));
        assert_eq!(content.the_word(), "Rendering");
        let WhatTheColumnSays::Rendering(name) = &content else {
            panic!("it is the rendering content");
        };
        assert_eq!(name.as_str(), "Work — renders — final one");
    }
}
