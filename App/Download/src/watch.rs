//! The watch — `Download` §3, W1 to W6, Row 210.
//!
//! **Constraint:** Row 210 — the arrangement has to learn what the job wrote
//! without planting anything on the machine and without an event route that
//! cannot answer for a file that landed before it started listening.
//!
//! **What is here is the cadence, the window and the reading; the walking is
//! the acting half's.** W1's walk runs over the file-transfer subsystem's
//! reading route, which is a connection this crate does not open
//! ([`crate::arriving_later`]). Every judgement the walk feeds — what is
//! active, whether a walk is a reading at all, when a fresh one may begin — is
//! here and is driven by values written by hand.
//!
//! **Markers used below that belong to another document, each with the
//! document it belongs to.** `Job files and outputs` C7, C9, I22, L7, N15.
//! `Server protocol and transfers` T5.
//!
//! **A marker this plan also defines is deliberately not listed above,
//! whichever document is meant, and is written out with its document at every
//! site instead** — the header list cannot tell the two apart, and a bare one
//! would resolve to a real and wrong clause. `Servers` N3 and
//! `Desktop app and delivery` P10b are the two that arise here.

use std::time::Duration;

/// **W1: the watch is a periodic walk of the job output folder over the
/// file-transfer subsystem's reading route, compared against the inventory. No
/// event mechanism, no machine-side program and no worker kind exists for
/// it.**
///
/// **It is not a standing timer**: its pause sits inside a running sequence
/// bounded by [`TheWindow`], which is what `Desktop app and delivery` P10b
/// admits, and that clause's rule rather than any list of examples is what it
/// is judged against.
///
/// The five reasons, each observed:
///
/// - **One mechanism serves both halves of the promise.** A walk answers *what
///   is there now*, so the first walk of a connection is the catch-up reading
///   and every later walk is the live watch. An event route cannot answer for
///   a file that landed before it started listening, so it would need the walk
///   anyway.
/// - **The subsystem route takes the protocol's own fields** — kind, size,
///   modification time, name, batched at about 100 entries per reply, 52 round
///   trips for 5010 entries — **so no text is mined and a name holding a
///   newline cannot split a reading** (`Job files and outputs` N15).
/// - **It needs no program on the machine.** What programs an arbitrary rental
///   carries and what they emit is unestablished ground, and the one event
///   tool the evidence describes follows inodes — a watch on a file follows
///   the inode, so a save-by-rename kills it silently.
/// - **Its failures separate.** A refusal, an absence and a dead machine carry
///   distinct protocol statuses at readdir, and a walk cut mid-listing is
///   separable from a complete short one by status — so **a walk that did not
///   complete is not a reading and is discarded whole**
///   (`Job files and outputs` C9's rule, applied to this walk).
/// - **The cost is the connection's.** No measured figure is offered for what
///   the machine spends, because none exists for this route: this walk plants
///   no program to measure. **What the machine is spared is structural rather
///   than measured** — nothing is installed, nothing is run, and no worker
///   kind is declared or wanted.
pub const NO_EVENT_MECHANISM_NO_MACHINE_SIDE_PROGRAM_AND_NO_WORKER_KIND: bool = true;

/// **W6: nothing is planted, renamed, opened or changed on the machine to make
/// watching easier.**
///
/// The spec forbids it outright; the rename lever the evidence found is closed
/// off by that; and *downloading never changes anything on the machine* covers
/// the watch as much as the fetch.
pub const NOTHING_IS_PLANTED_RENAMED_OPENED_OR_CHANGED_TO_MAKE_WATCHING_EASIER: bool = true;

/// **W2's floor: a fresh walk begins after a pause of 10 seconds, or the
/// previous walk's own duration where that was longer.**
///
/// **It is a declared constant, not a tuning left open.**
///
/// **10 seconds is short against a frame**: the one frame cost on record is
/// about 16.8 seconds and real frames run longer, so per-frame arrival —
/// *outputs come home as they are written* — holds at this cadence. That
/// figure is one output format and one picture size (§15 item 16); a scene
/// whose frames finish faster has several frames complete between two walks
/// and its files come home a few frames behind rather than one, **which is
/// latency and not a judgement**.
pub const THE_PAUSE: Duration = Duration::from_secs(10);

/// **W2: the pause before the next walk, measured from the end of the previous
/// one.**
///
/// Three constraints fix the shape and the number, and all three are in this
/// one function.
///
/// **Measured from the end, never the start**, so walks never overlap and a
/// slow walk cannot pile requests onto a struggling connection — the same
/// load-bearing shape as `Servers` N3. That is the caller's discipline and
/// this signature's: what is handed in is how long the previous walk *took*,
/// not when it began.
///
/// **The previous-duration floor bounds the duty cycle at one half, per
/// arrangement**, on a folder large enough that the walk itself is slow.
///
/// **And the aggregate is bounded in this document's own cadence, which is
/// this document's to declare: all of a server's walks take at most one half
/// of the reading channel between them, each arrangement's pause stretching as
/// their number grows.** `Server protocol and transfers` T5 gives a connection
/// one long-lived reading channel to carry listings, attribute reads and free-
/// space readings, the user's own among them; two arrangements would exhaust a
/// per-series half and three would oversubscribe it. **The channel is never
/// held for more than half its time by this document whatever the number.**
///
/// **What order the walks and a user's own interactive listing take *inside*
/// the channel is not supplied anywhere and is not declared here**; it is that
/// document's and is carried at §15 item 22.
///
/// # The multiplier is 2n−1, and W2 now says so
///
/// **W2's illustration was corrected in the plan on 2026-09-05 to *twice their
/// number less one*** — two arrangements at three times the pause, three at
/// five — **and its bound is unchanged**, which the correction says in as many
/// words.
///
/// The bound is W2's declaration: *"all of a server's walks take at most one
/// half of that channel between them"*. The illustration had read *two
/// arrangements walk at twice the pause, three at three times*, which does not
/// deliver it: **at a pause of n times the floor the aggregate is n/(n+1)** —
/// two thirds at two arrangements, three quarters at three.
///
/// **At (2n−1) times the floor the aggregate is exactly one half at every n**,
/// and **it reduces to W2's own single-series arithmetic at n = 1**: one
/// arrangement pauses for the walk's own duration, which is the third bullet
/// word for word. So the multiplier is the plan's own one-series rule extended
/// to n series so that the aggregate it declares holds, rather than a number
/// chosen here.
///
/// **The bound is what the clause declares and the multiplier is its gloss**,
/// which is why the bound is what this function keeps. The code was built to
/// 2n−1 before the plan carried it; the disagreement is recorded as a finding
/// in `Audits/Build/Download/Decisions record.md`, a reader confirmed it, and
/// the plan was repaired. **Nothing here moved for the repair.**
pub fn the_pause_before_the_next_walk(
    the_previous_walk_took: Duration,
    arrangements_walking_this_server: usize,
) -> Duration {
    let floor = THE_PAUSE.max(the_previous_walk_took);
    let arrangements = arrangements_walking_this_server.max(1) as u32;
    floor * (2 * arrangements - 1)
}

/// **W2's aggregate bound, derived rather than asserted.**
///
/// The share of the reading channel all of a server's walks take between them,
/// as a fraction with the numerator and the denominator kept whole so nothing
/// rounds. **It is never above one half.**
pub fn the_share_of_the_channel_all_the_walks_take(
    the_walk_takes: Duration,
    arrangements_walking_this_server: usize,
) -> (u128, u128) {
    let arrangements = arrangements_walking_this_server.max(1);
    let pause = the_pause_before_the_next_walk(the_walk_takes, arrangements);
    let working = the_walk_takes.as_nanos() * arrangements as u128;
    let cycle = the_walk_takes.as_nanos() + pause.as_nanos();
    if cycle == 0 {
        return (0, 1);
    }
    (working, cycle)
}

/// **W3: walks run only while they can mean something.**
///
/// Three conditions, all of them, and the third is per attempt rather than per
/// job.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TheWindow {
    /// The arrangement's desired state is on.
    pub the_desired_state_is_on: bool,
    /// A connection to the server stands. **The condition is `Servers`' and is
    /// read, never derived here.**
    pub a_connection_to_the_server_stands: bool,
    /// **The job is between *a* render-start and the completion of the ending
    /// walk.** Before the first render-start there is nothing to watch.
    pub the_attempts_window_is_open: bool,
}

impl TheWindow {
    /// All three, and nothing else.
    pub fn walks_run(self) -> bool {
        self.the_desired_state_is_on
            && self.a_connection_to_the_server_stands
            && self.the_attempts_window_is_open
    }
}

/// **W3: why the window closes at the ending walk, written out because the
/// wrong reason is available and is wrong.**
///
/// It closes because the arrangement's own bookkeeping for that attempt is
/// finished — **not because the folder can never change again, which is a
/// thing this document does not know and `Job files and outputs` L7 does not
/// say**: that clause says only that nothing but what a render writes ever
/// sits inside a job output folder, which is a statement about what may be
/// there rather than about when, and that document's own I22 says the opposite
/// in terms — *"Running the job again replaces what was left rather than
/// adding to it."*
pub const THE_WINDOW_CLOSES_ON_THE_BOOKKEEPING_AND_NOT_ON_A_CLAIM_ABOUT_THE_FOLDER: bool = true;

/// **W3: the window is per attempt and not per job, and a new render-start for
/// the same job re-arms a done arrangement.**
///
/// It opens a fresh window and the walks resume; the reckoning's *never
/// coming* entries re-open to *owed* for the frames the new attempt's list
/// names; the whole is recomputed; and the arrangement runs to a second ending
/// and a second done. **Nothing else re-opens**, which is what keeps a re-run
/// from arriving over the user's copies.
pub fn a_render_start_re_opens_the_window(window: TheWindow) -> TheWindow {
    TheWindow {
        the_attempts_window_is_open: true,
        ..window
    }
}

/// **W4: one entry of one walk.**
///
/// The walk is **one walk of the folder and everything under it** —
/// `Job files and outputs` C7's shape, because the redirect deepens paths by
/// construction and a single directory's entries would miss the inventory's
/// predicted paths. Each entry comes back as its path relative to the job
/// output folder, its kind, its size and its modification time.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AWalkEntry {
    /// **Relative to the job output folder**, which is the inventory's own key.
    pub path: Vec<u8>,
    pub is_a_folder: bool,
    pub size: u64,
    pub modification_time: i64,
}

/// Why a walk is not a reading.
///
/// **All three carry distinct protocol statuses**, which is the fourth reason
/// W1 gives for the route.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhyTheWalkIsNotAReading {
    /// A refusal reading the job's own folder. **Reported as the truth** —
    /// this document could not read the job's folder, with the separated cause
    /// where the route separates it — **and the walks continue; nothing gives
    /// up.**
    ARefusal,
    /// A listing cut mid-flight. **Separable from a genuinely short complete
    /// one by status**, which is the whole reason a partial listing is not
    /// mistaken for a folder with fewer files in it.
    ItWasCut,
    /// The connection went. **The connection condition covers the
    /// dead-machine case** and the window closes with it.
    TheConnectionWent,
    /// **An entry's kind was not carried by the reply**, so what came back is
    /// not a reading of the folder.
    ///
    /// **This one carries no protocol status**, because nothing failed: the
    /// listing arrived and one of its entries did not say what it was. W1
    /// treats *kind* as a field the route supplies, and on this route it is
    /// one bit of a value a server may or may not send.
    ///
    /// **Reading an absent kind as *not a folder* is the one wrong answer
    /// nothing else here catches**: the walk would never descend into a job's
    /// sub-folders, would still report a complete walk because nothing failed,
    /// every file under them would become *never coming*, and a job whose
    /// frames are all safely on the machine would read **ended, 0 of 720**.
    /// Each such folder would also be pushed onto the fetchable list and a
    /// file transfer arranged for a directory.
    ///
    /// **Discarding costs one walk cadence per unanswered entry and fails
    /// towards fetching late**, which is the direction this document claims
    /// everywhere else. **It is what stands while the question is open and it
    /// is not a substitute for answering it.**
    AnEntrysKindWasNotCarried,
}

/// **W5: what a walk yields is a reading and only a reading.**
///
/// It is period-held, carries its moment, and is never written as state. **A
/// walk that did not complete is not a reading and is discarded whole.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AWalk {
    Complete(Vec<AWalkEntry>),
    Discarded(WhyTheWalkIsNotAReading),
}

impl AWalk {
    /// The entries where this walk is a reading at all.
    ///
    /// **A discarded walk yields nothing rather than an empty folder**, which
    /// is the one place reading a failure as an absence would put every frame
    /// of a job into *never coming*.
    pub fn entries(&self) -> Option<&[AWalkEntry]> {
        match self {
            AWalk::Complete(entries) => Some(entries),
            AWalk::Discarded(_) => None,
        }
    }

    /// **W5: a walk failure discards that walk; the next one runs at the
    /// cadence.** Nothing about a discarded walk changes any record.
    pub fn changes_nothing(&self) -> bool {
        matches!(self, AWalk::Discarded(_))
    }
}

/// **W5: change detection is the comparison of two complete walks.**
///
/// An entry is **active** where it is new, or where its size or its
/// modification time differs from the previous complete walk.
///
/// **Only two complete walks are ever compared.** A discarded walk is not a
/// reading, so it is never one of the two, and the previous *complete* walk
/// stays the baseline until another completes.
pub fn what_is_active<'a>(
    the_previous_complete_walk: &[AWalkEntry],
    now: &'a [AWalkEntry],
) -> Vec<&'a AWalkEntry> {
    now.iter()
        .filter(|entry| {
            match the_previous_complete_walk
                .iter()
                .find(|before| before.path == entry.path)
            {
                None => true,
                Some(before) => {
                    before.size != entry.size || before.modification_time != entry.modification_time
                }
            }
        })
        .collect()
}

/// **W1's own bound, held as a value: the walk reads the folder and never
/// anything about the render.**
///
/// It is what §15 item 5 rests on — nothing here cites a cost figure measured
/// on a machine-side event watcher, which is the route W1's third reason
/// rejects.
pub const NOTHING_HERE_RESTS_ON_A_MEASUREMENT_OF_WHAT_A_RENDER_COST: bool = true;

/// **E2b's figure: a minute of passes in which nothing ended.** So a single
/// pass's refused listing or a busy seat says nothing.
pub const THE_MINUTE_BEFORE_IT_IS_SAID: std::time::Duration = std::time::Duration::from_secs(60);

/// **E2b: what one pass makes of one arrangement** — units owed and none of them
/// ended in this pass, or the job has ended and its ending walk has not completed.
/// A unit that came back to *owed* did not move. A file in flight never reaches
/// this: the pass waits for it, and its ending moves.
pub fn it_stood_still_this_pass(owed_before: usize, owed_after: usize, ending_walk_is_owed: bool) -> bool {
    (owed_after > 0 && owed_after >= owed_before) || ending_walk_is_owed
}

/// **E2b's clocks, one connection's and run-held** (R11): a job that moved has
/// none, and a job that stood still keeps the moment it began to.
#[derive(Debug, Default)]
pub struct TheDownloadsStandingStill {
    since: std::collections::HashMap<String, std::time::Instant>,
}

impl TheDownloadsStandingStill {
    /// **What a pass saw of a job, at a moment.**
    pub fn a_pass_saw(&mut self, job: &str, it_stood_still: bool, at: std::time::Instant) {
        if it_stood_still {
            self.since.entry(job.to_string()).or_insert(at);
        } else {
            self.since.remove(job);
        }
    }

    /// **The same, now.**
    pub fn a_pass_saw_now(&mut self, job: &str, it_stood_still: bool) {
        self.a_pass_saw(job, it_stood_still, std::time::Instant::now());
    }

    /// **Whether the minute has passed for a job, at a moment.**
    pub fn it_is_said(&self, job: &str, at: std::time::Instant) -> bool {
        self.since
            .get(job)
            .is_some_and(|since| at.duration_since(*since) >= THE_MINUTE_BEFORE_IT_IS_SAID)
    }

    /// **The same, now.**
    pub fn it_is_said_now(&self, job: &str) -> bool {
        self.it_is_said(job, std::time::Instant::now())
    }
}

#[cfg(test)]
mod tests {
    /// **E2b.**
    #[test]
    fn a_download_says_so_only_after_a_minute_of_standing_still() {
        use std::time::{Duration, Instant};
        assert!(it_stood_still_this_pass(2, 2, false), "owed, and none ended");
        assert!(!it_stood_still_this_pass(2, 1, false), "a unit ended");
        assert!(!it_stood_still_this_pass(0, 0, false), "nothing is owed");
        assert!(it_stood_still_this_pass(0, 0, true), "the ending walk is owed");
        let mut clocks = TheDownloadsStandingStill::default();
        let at = Instant::now();
        clocks.a_pass_saw("j", true, at);
        assert!(!clocks.it_is_said("j", at + Duration::from_secs(59)));
        clocks.a_pass_saw("j", true, at + Duration::from_secs(59));
        assert!(clocks.it_is_said("j", at + Duration::from_secs(60)));
        clocks.a_pass_saw("j", false, at + Duration::from_secs(61));
        assert!(!clocks.it_is_said("j", at + Duration::from_secs(62)), "it moved");
        clocks.a_pass_saw("j", true, at + Duration::from_secs(63));
        assert!(!clocks.it_is_said("j", at + Duration::from_secs(64)), "a fresh minute");
    }

    use super::*;

    fn entry(path: &[u8], size: u64, modified: i64) -> AWalkEntry {
        AWalkEntry {
            path: path.to_vec(),
            is_a_folder: false,
            size,
            modification_time: modified,
        }
    }

    #[test]
    fn the_pause_is_ten_seconds_or_the_previous_walks_own_duration_where_that_is_longer() {
        assert_eq!(
            the_pause_before_the_next_walk(Duration::from_secs(2), 1),
            Duration::from_secs(10)
        );
        assert_eq!(
            the_pause_before_the_next_walk(Duration::from_secs(40), 1),
            Duration::from_secs(40)
        );
    }

    #[test]
    fn each_arrangements_pause_stretches_as_their_number_grows() {
        assert_eq!(
            the_pause_before_the_next_walk(Duration::from_secs(2), 2),
            Duration::from_secs(30)
        );
        assert_eq!(
            the_pause_before_the_next_walk(Duration::from_secs(2), 3),
            Duration::from_secs(50)
        );
        // A server with no arrangement walking it still has the floor, rather
        // than a pause of nothing.
        assert_eq!(
            the_pause_before_the_next_walk(Duration::from_secs(2), 0),
            THE_PAUSE
        );
    }

    #[test]
    fn all_of_a_servers_walks_never_take_more_than_half_the_channel_between_them() {
        // The bound holds at the hard case W2 names — a folder large enough
        // that the walk itself is slow — and at every number of arrangements.
        for arrangements in 1..=8usize {
            for walk in [1u64, 10, 60, 600] {
                let (working, cycle) = the_share_of_the_channel_all_the_walks_take(
                    Duration::from_secs(walk),
                    arrangements,
                );
                assert!(
                    working * 2 <= cycle,
                    "{arrangements} arrangements at a {walk}s walk took {working}/{cycle}"
                );
            }
        }
    }

    /// **This holds W2's repaired multiplier by driving the one it replaced.**
    /// Until 2026-09-05 W2 illustrated its bound with *two arrangements at
    /// twice the pause, three at three times*; at n times the floor the
    /// aggregate is n/(n+1), which is above one half from two arrangements on.
    /// Anything that put that figure back would break the bound the test above
    /// keeps.
    #[test]
    fn the_multiplier_w2_replaced_would_have_broken_the_bound_the_same_clause_declares() {
        let walk = Duration::from_secs(60);
        for arrangements in 2..=8u128 {
            let pause = walk.as_nanos() * arrangements;
            let working = walk.as_nanos() * arrangements;
            let cycle = walk.as_nanos() + pause;
            assert!(
                working * 2 > cycle,
                "{arrangements} arrangements at n times the floor stayed inside the half"
            );
        }
    }

    #[test]
    fn walks_run_only_where_all_three_conditions_hold() {
        let open = TheWindow {
            the_desired_state_is_on: true,
            a_connection_to_the_server_stands: true,
            the_attempts_window_is_open: true,
        };
        assert!(open.walks_run());
        assert!(!TheWindow {
            the_desired_state_is_on: false,
            ..open
        }
        .walks_run());
        assert!(!TheWindow {
            a_connection_to_the_server_stands: false,
            ..open
        }
        .walks_run());
        assert!(!TheWindow {
            the_attempts_window_is_open: false,
            ..open
        }
        .walks_run());
    }

    #[test]
    fn a_render_start_re_opens_a_closed_window_and_touches_neither_other_condition() {
        let done = TheWindow {
            the_desired_state_is_on: true,
            a_connection_to_the_server_stands: false,
            the_attempts_window_is_open: false,
        };
        let re_armed = a_render_start_re_opens_the_window(done);
        assert!(re_armed.the_attempts_window_is_open);
        // The connection is `Servers`' and a render-start says nothing about
        // it, so the window is open and the walks still do not run.
        assert!(!re_armed.a_connection_to_the_server_stands);
        assert!(!re_armed.walks_run());
    }

    #[test]
    fn a_discarded_walk_is_not_a_reading_and_is_never_an_empty_folder() {
        for why in [
            WhyTheWalkIsNotAReading::ARefusal,
            WhyTheWalkIsNotAReading::ItWasCut,
            WhyTheWalkIsNotAReading::TheConnectionWent,
        ] {
            let walk = AWalk::Discarded(why);
            assert!(walk.entries().is_none());
            assert!(walk.changes_nothing());
        }
        let complete = AWalk::Complete(Vec::new());
        assert_eq!(complete.entries(), Some(&[][..]));
        assert!(!complete.changes_nothing());
    }

    #[test]
    fn an_entry_is_active_where_it_is_new_or_its_size_or_its_time_moved() {
        let before = vec![
            entry(b"frame_0001.png", 1024, 100),
            entry(b"frame_0002.png", 512, 100),
            entry(b"frame_0003.png", 2048, 100),
        ];
        let now = vec![
            entry(b"frame_0001.png", 1024, 100),
            entry(b"frame_0002.png", 900, 100),
            entry(b"frame_0003.png", 2048, 300),
            entry(b"frame_0004.png", 0, 400),
        ];
        let active: Vec<&[u8]> = what_is_active(&before, &now)
            .into_iter()
            .map(|entry| entry.path.as_slice())
            .collect();
        assert_eq!(
            active,
            vec![
                &b"frame_0002.png"[..],
                &b"frame_0003.png"[..],
                &b"frame_0004.png"[..]
            ]
        );
    }

    #[test]
    fn the_walk_reads_the_whole_tree_and_keys_on_the_path_below_the_job_output_folder() {
        // C7's shape: the redirect deepens paths, so an entry two folders down
        // is compared by its whole relative path and not by its own name.
        let before = vec![entry(b"beauty/frame_0001.exr", 10, 1)];
        let now = vec![
            entry(b"beauty/frame_0001.exr", 10, 1),
            entry(b"mist/frame_0001.exr", 10, 1),
        ];
        let active = what_is_active(&before, &now);
        assert_eq!(active.len(), 1);
        assert_eq!(active[0].path, b"mist/frame_0001.exr");
    }
}
