//! **Staleness — `Telegram` B5, B6 and B13.** When a machine that is not
//! holding may ask, decided without subtracting across clocks.
//!
//! **A read that did not complete counts for nothing either way.** It neither
//! advances the interval nor resets it, and it is never *no readable board*.
//! And a machine that has not completed a read for a full interval does not ask
//! at all. Without both, the machine on the worst connection would take the
//! slot at every recovery (B5's second paragraph).

use std::time::Duration;

/// **`Telegram` B13 — every enabled role worker reads the reported pin every
/// two seconds**, holding or not.
pub const THE_READ_CADENCE: Duration = Duration::from_secs(2);

/// **`Telegram` B2 and B13 — the stated renewal interval's default.**
pub const THE_DEFAULT_RENEWAL_INTERVAL: Duration = Duration::from_secs(60);

/// **`Telegram` B4 — how often the holder edits its heartbeat, for a stated
/// interval.** B5 lets a reader ask once it has seen no change for the whole
/// stated interval between its own readings, so a holder that renewed exactly
/// on the interval would look stale to a reader whose readings straddle it.
/// **This build reads *on the renewal interval* as *at least once within every
/// stated interval*** and renews ten seconds early — five reads — or at half the
/// interval where that is shorter. Recorded at
/// `Audits/Build/Telegram/Decisions record.md`.
pub fn how_often_the_holder_renews(stated: Duration) -> Duration {
    stated - Duration::from_secs(10).min(stated / 2)
}

/// **What one completed read of the reported pin saw**, for B5.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WhatACompletedReadSaw {
    /// A readable board, with its opaque heartbeat value. Compared for equality
    /// only (`State, identity and recovery` D5).
    AHeartbeat(String),
    /// No pin, or a pin that is not a readable board — a machinery message in
    /// flight included. **Never a fresh heartbeat.**
    NoReadableBoard,
}

/// **`Telegram` B5 — one stood-down machine's watch on the holder**, measured
/// between its own reading moments on its own clock.
#[derive(Debug, Clone, Default)]
pub struct TheStalenessWatch {
    last_heartbeat: Option<String>,
    /// The first completed reading since the heartbeat last changed.
    unchanged_since: Option<Duration>,
    last_completed: Option<Duration>,
}

impl TheStalenessWatch {
    pub fn new() -> TheStalenessWatch {
        TheStalenessWatch::default()
    }

    /// A read completed at `at`, on this machine's clock.
    pub fn a_read_completed(&mut self, at: Duration, saw: WhatACompletedReadSaw) {
        if let WhatACompletedReadSaw::AHeartbeat(value) = saw {
            if self.last_heartbeat.as_deref() != Some(value.as_str()) {
                self.last_heartbeat = Some(value);
                self.unchanged_since = Some(at);
            }
        }
        if self.unchanged_since.is_none() {
            // B6: counted from this machine's own first reading.
            self.unchanged_since = Some(at);
        }
        self.last_completed = Some(at);
    }

    /// **A read that did not complete: nothing advances and nothing resets.**
    /// Written as a method so every caller says so rather than forgetting.
    pub fn a_read_did_not_complete(&mut self) {}

    /// **`Telegram` B5 and B6 — whether this machine may ask now.** Only where
    /// its completed readings have shown no fresh heartbeat across a whole
    /// stated interval, and it has completed a read within the last interval.
    pub fn may_ask(&self, now: Duration, renewal: Duration) -> bool {
        let (Some(since), Some(last)) = (self.unchanged_since, self.last_completed) else {
            return false;
        };
        let in_contact = now.saturating_sub(last) < renewal;
        in_contact && last.saturating_sub(since) >= renewal
    }

    /// **`Rented-server runtime` T4 — after a refused or lost claim, standing down starts
    /// the count again** from the next completed reading.
    pub fn stand_down(&mut self) {
        self.unchanged_since = None;
    }
}

/// **`Telegram` B14 — whether this machine has seen a holder alive**, kept
/// apart from [`TheStalenessWatch`]'s count. Added 2026-10-18, the Creator's
/// fourteenth notes, note 2, with the two independent checks of that day
/// folded in: a machine announces itself, and pins any utterance again, only
/// to a holder it has seen alive.
///
/// **Seen alive** means two completed readings of **the same board**, naming
/// **another machine** as holder, showed different heartbeat values, and the
/// later of them was within the last stated renewal interval on this machine's
/// own clock. So:
/// - the first reading of a board sets the starting value and is no change;
/// - a reading of *nothing pinned*, of a machinery message, or of a board this
///   machine holds is no change, and is never handed in here;
/// - a different board is no change either: it starts its own comparison, so
///   a hand-over's new board must renew once before it counts;
/// - only the heartbeat is compared, never the text, because the polite stop's
///   last edit changes the roster and leaves the heartbeat alone.
///
/// **The proof belongs to one board and expires**: it holds for the board it
/// was seen on, for one stated interval after the change. **Nothing but a new
/// change revives it** — [`TheStalenessWatch::stand_down`], which a refused or
/// lost claim calls, does not touch it. A reading of nothing pinned or of a
/// machinery message between two readings of the same board does not break
/// the comparison: a heartbeat seen to differ between two readings of one board
/// was edited between them, which is what alive means.
#[derive(Debug, Clone, Default)]
pub struct TheProofOfLife {
    /// The last completed reading of another machine's board: its identity and
    /// heartbeat.
    last: Option<(i64, String)>,
    /// The board whose heartbeat was last seen to change, and when.
    changed: Option<(i64, Duration)>,
}

impl TheProofOfLife {
    pub fn new() -> TheProofOfLife {
        TheProofOfLife::default()
    }

    /// **A completed read at `at` showed `board`, naming another machine as
    /// its holder, with `heartbeat`.** A board naming this machine is never
    /// handed in (B14: it is never proof of life).
    pub fn another_holders_board_was_read(&mut self, at: Duration, board: i64, heartbeat: &str) {
        if let Some((seen, before)) = &self.last {
            if *seen == board && before != heartbeat {
                self.changed = Some((board, at));
            }
        }
        self.last = Some((board, heartbeat.to_string()));
    }

    /// **Whether the holder of `board` has been seen alive** at `now`: its
    /// heartbeat seen to change within the last `renewal`.
    pub fn the_holder_is_alive(&self, board: i64, now: Duration, renewal: Duration) -> bool {
        self.changed
            .is_some_and(|(seen, at)| seen == board && now.saturating_sub(at) < renewal)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const RENEWAL: Duration = Duration::from_secs(60);
    fn s(n: u64) -> Duration {
        Duration::from_secs(n)
    }
    fn beat(v: &str) -> WhatACompletedReadSaw {
        WhatACompletedReadSaw::AHeartbeat(v.into())
    }

    #[test]
    fn an_unchanged_heartbeat_for_a_whole_interval_lets_a_machine_ask() {
        let mut watch = TheStalenessWatch::new();
        for t in (0..=58).step_by(2) {
            watch.a_read_completed(s(t), beat("h1"));
        }
        assert!(!watch.may_ask(s(58), RENEWAL));
        watch.a_read_completed(s(60), beat("h1"));
        assert!(watch.may_ask(s(60), RENEWAL));
    }

    #[test]
    fn a_fresh_heartbeat_starts_the_count_again() {
        let mut watch = TheStalenessWatch::new();
        watch.a_read_completed(s(0), beat("h1"));
        watch.a_read_completed(s(50), beat("h2"));
        watch.a_read_completed(s(100), beat("h2"));
        assert!(!watch.may_ask(s(100), RENEWAL));
        watch.a_read_completed(s(110), beat("h2"));
        assert!(watch.may_ask(s(110), RENEWAL));
    }

    /// **B5: a machinery message on top is no readable board, never a fresh
    /// heartbeat, and the same heartbeat seen after it is no change.**
    #[test]
    fn a_pin_that_is_not_a_board_is_no_fresh_heartbeat() {
        let mut watch = TheStalenessWatch::new();
        watch.a_read_completed(s(0), beat("h1"));
        watch.a_read_completed(s(30), WhatACompletedReadSaw::NoReadableBoard);
        watch.a_read_completed(s(60), beat("h1"));
        assert!(watch.may_ask(s(60), RENEWAL));
    }

    /// **The brief's first thing: a read that did not complete counts for
    /// nothing either way**, and a machine out of contact for an interval does
    /// not ask until it has completed a read.
    #[test]
    fn a_read_that_did_not_complete_counts_for_nothing_either_way() {
        let mut watch = TheStalenessWatch::new();
        watch.a_read_completed(s(0), beat("h1"));
        for _ in 0..40 {
            watch.a_read_did_not_complete();
        }
        // Minutes of failed reads: never "no readable board", and out of contact.
        assert!(!watch.may_ask(s(120), RENEWAL));
        // One completed read brings it back into contact; the interval between
        // its two completed readings is what counts.
        watch.a_read_completed(s(121), beat("h1"));
        assert!(watch.may_ask(s(121), RENEWAL));

        // And failed reads never reset an interval under way.
        let mut watch = TheStalenessWatch::new();
        watch.a_read_completed(s(0), beat("h1"));
        watch.a_read_did_not_complete();
        watch.a_read_completed(s(40), beat("h1"));
        watch.a_read_completed(s(60), beat("h1"));
        assert!(watch.may_ask(s(60), RENEWAL));
    }

    #[test]
    fn nothing_read_yet_means_nothing_to_ask_on() {
        assert!(!TheStalenessWatch::new().may_ask(s(1000), RENEWAL));
    }

    #[test]
    fn standing_down_starts_the_count_again() {
        let mut watch = TheStalenessWatch::new();
        watch.a_read_completed(s(0), WhatACompletedReadSaw::NoReadableBoard);
        watch.a_read_completed(s(60), WhatACompletedReadSaw::NoReadableBoard);
        assert!(watch.may_ask(s(60), RENEWAL));
        watch.stand_down();
        watch.a_read_completed(s(62), WhatACompletedReadSaw::NoReadableBoard);
        assert!(!watch.may_ask(s(62), RENEWAL));
    }

    /// **B14, the checks' change 1: the first reading of a board is no
    /// change**, and two readings of the same board whose heartbeats differ
    /// are proof, for that board, for one interval after the change.
    #[test]
    fn a_change_between_two_readings_of_one_board_is_proof_and_expires() {
        let mut alive = TheProofOfLife::new();
        alive.another_holders_board_was_read(s(0), 7, "h1");
        assert!(!alive.the_holder_is_alive(7, s(0), RENEWAL), "a first reading is no change");
        alive.another_holders_board_was_read(s(2), 7, "h1");
        assert!(!alive.the_holder_is_alive(7, s(2), RENEWAL), "an unchanged heartbeat is no proof");
        alive.another_holders_board_was_read(s(50), 7, "h2");
        assert!(alive.the_holder_is_alive(7, s(50), RENEWAL));
        assert!(alive.the_holder_is_alive(7, s(108), RENEWAL));
        assert!(!alive.the_holder_is_alive(7, s(110), RENEWAL), "the proof expires a full interval after the change");
        assert!(!alive.the_holder_is_alive(8, s(60), RENEWAL), "proof belongs to the board it was seen on");
    }

    /// **B14, the checks' changes 1 and D7: a replacement board, another
    /// holder's board, a failed read and *nothing pinned* never count as a
    /// change**, and a lost claim never revives a proof that has expired.
    #[test]
    fn a_replacement_board_a_failed_read_and_nothing_pinned_are_no_change() {
        // A different board, its heartbeat different too: no change, and the
        // new board starts its own comparison.
        let mut alive = TheProofOfLife::new();
        alive.another_holders_board_was_read(s(0), 7, "h1");
        alive.another_holders_board_was_read(s(2), 9, "h9");
        assert!(!alive.the_holder_is_alive(9, s(2), RENEWAL), "a hand-over's new board is no change");
        assert!(!alive.the_holder_is_alive(7, s(2), RENEWAL));
        alive.another_holders_board_was_read(s(4), 7, "h2");
        assert!(!alive.the_holder_is_alive(7, s(4), RENEWAL), "back to the old board: its comparison started again");
        alive.another_holders_board_was_read(s(52), 9, "h10");
        assert!(!alive.the_holder_is_alive(9, s(52), RENEWAL), "the new board compared with the old one's last reading");
        alive.another_holders_board_was_read(s(54), 9, "h11");
        assert!(alive.the_holder_is_alive(9, s(54), RENEWAL), "and once it renews, it counts");

        // Nothing pinned, a machinery message and failed reads are never handed
        // in, so they change nothing either way: a dead board stays dead
        // however many come between its readings, and a lost claim (B5's
        // `stand_down`) has no say here.
        let mut alive = TheProofOfLife::new();
        let mut watch = TheStalenessWatch::new();
        for t in (0..=120).step_by(2) {
            if t % 6 == 0 {
                watch.a_read_completed(s(t), WhatACompletedReadSaw::NoReadableBoard);
            } else if t % 10 == 0 {
                watch.a_read_did_not_complete();
            } else {
                alive.another_holders_board_was_read(s(t), 7, "dead");
                watch.a_read_completed(s(t), beat("dead"));
            }
            watch.stand_down();
            assert!(!alive.the_holder_is_alive(7, s(t), RENEWAL), "a dead board's holder is never alive, at {t}");
        }

        // A proof seen, then expired: a lost claim does not revive it.
        let mut alive = TheProofOfLife::new();
        alive.another_holders_board_was_read(s(0), 7, "h1");
        alive.another_holders_board_was_read(s(2), 7, "h2");
        let mut watch = TheStalenessWatch::new();
        watch.a_read_completed(s(2), beat("h2"));
        watch.a_read_completed(s(62), beat("h2"));
        watch.stand_down();
        alive.another_holders_board_was_read(s(64), 7, "h2");
        assert!(!alive.the_holder_is_alive(7, s(64), RENEWAL), "a lost claim never revives the proof");
    }

    #[test]
    fn the_holder_renews_inside_the_stated_interval() {
        assert_eq!(how_often_the_holder_renews(s(60)), s(50));
        assert_eq!(how_often_the_holder_renews(s(12)), s(6));
    }
}
