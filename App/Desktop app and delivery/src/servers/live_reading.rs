//! **P10d's live-reading sequence — the one bounded sequence this document's
//! own §3.3 starts.**
//!
//! **Why it exists at all, in the clause's own words.** `Queue & History`
//! promises a rendering entry that is live while the render runs, and T18
//! classifies as *now*, and posts notifications for, only an event read while
//! the connected period is live. **So something must read a connected machine
//! repeatedly inside a standing period** — and P9 and P10 here, `Queue &
//! History` A1 and P3, `Servers` W4 and `Rented-server runtime` U26, each read
//! alone, forbid every route such a reading could take. **The route is P10d,
//! and this file is it.**
//!
//! # Two things about it that are easy to get wrong
//!
//! **Its set of reads is closed and it is closed at three.** The read-forward
//! from the event cursor, every pass; the queue reading, only while a surface
//! showing that server's queue or its running entry is on screen; the running
//! entry's commentary read, only while that commentary is on screen.
//! **[`WhatJoinsNothing`] names the four things that do not join it** — the
//! hardware reading, a health reading, a free-space re-read, and any condition
//! record — and a fourth read is a broken clause rather than an addition.
//!
//! **It is a running sequence with delays inside it and never a standing
//! timer.** P9 and P10 forbid the timer, and P10d says so in its own last line.
//! A declared act starts it, it ends when the period ends, and nothing outside
//! the period can outlive it — which is P10a's first admitted shape, the same
//! one `Servers`' own repair runs in.
//!
//! # What starts it
//!
//! **P10d's own start is `Servers` Q1 step 7 completing for that server** —
//! desktop reconciliation through `State, identity and recovery` §7 step 5
//! done and the first queue reading taken. **`Servers` names step 7 as one of
//! the two steps its sequence does not build**
//! (`smoozy_servers::sequence::STEPS_NOT_BUILT`), and the hand-over runs from
//! whoever drives that sequence rather than from inside it.
//!
//! **Step 10c piece 7 built that hand-over**, in
//! `src/servers/the_machines_work.rs`: step 7 runs the moment a primary
//! connection that settled on the confirmed branch is lent, and this sequence
//! is started on the same thread when it completes. **That file gives
//! [`WhatAPassTakes`] its one end**: the read forward is step 3's own pass,
//! the queue reading step 5's own request, and the commentary read — which no
//! request carries — is never on screen and never taken.
//!
//! # The interval is this stage's to pick, and this is the pick
//!
//! P10d: *"The interval is Build's to pick and this document names no
//! number."* [`BETWEEN_ONE_PASS_AND_THE_NEXT`] is that pick and carries its
//! reasoning. **The pacing rule is not this stage's** — one pass at a time, and
//! the pause measured from the **end** of the previous pass and never from its
//! start, which is the clause's own and is the same rule `Servers` N3 states
//! for the repair.
//!
//! # How a clause marker is written in this crate, and it is a rule rather than
//! a habit
//!
//! A marker with no document beside it means a clause of this crate's own plan,
//! `Preparation/Architecture/Desktop app and delivery.md`. Every other
//! document's marker is written with that document — at the site, or in this
//! file's own header list below.
//!
//! **Markers used below that belong to another document, each with the document
//! it belongs to.** `Servers` N3, Q18, Q19. `Rented-server runtime` U26.
//!
//! **A marker this crate's own plan also defines is deliberately not listed
//! above, whichever document is meant, and is written out with its document at
//! every site instead** — the header list cannot tell the two apart, and a bare
//! one would resolve to a real and wrong clause.

use std::time::Duration;

use smoozy_servers::acting::Wait;

/// **P10d's closed set of reads, and it is closed at three.**
///
/// **A fourth read is not an addition to this list; it is a clause broken.** The
/// set was written out of a scenario replay, and the four things that were
/// considered and kept out are [`WhatJoinsNothing`]. Everything that takes a
/// pass dispatches on this enum, so a read that is not one of the three has
/// nowhere in the sequence to be taken from.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhatAPassReads {
    /// **The read-forward from the event cursor**, under `State, identity and
    /// recovery` §4 — **every pass, for the whole connected stretch, whatever
    /// is on screen**, because T18's *now* bucket and the fetches `Queue &
    /// History` P3 triggers serve a user who is looking elsewhere (T22).
    ///
    /// **Each event is applied idempotently and the cursor advances only after
    /// the application is durable**, which is §4's own rule and not a new one.
    TheReadForwardFromTheEventCursor,
    /// **`Queue & History` A1's one request, carrying `Queue & History` R5's
    /// live figures** —
    /// repeated **only while a surface showing that server's queue or its
    /// running entry is on screen**.
    TheQueueReading,
    /// **`Queue & History` E8's read of the sink's current bytes** — repeated
    /// **only while that commentary is on screen**.
    TheRunningEntrysCommentaryRead,
}

impl WhatAPassReads {
    /// **The whole set**, in the order P10d writes it.
    pub const ALL: [WhatAPassReads; 3] = [
        WhatAPassReads::TheReadForwardFromTheEventCursor,
        WhatAPassReads::TheQueueReading,
        WhatAPassReads::TheRunningEntrysCommentaryRead,
    ];

    /// **What has to be on screen for this read to be taken.**
    ///
    /// **P10d: what stops when the entry is off screen is the queue reading's
    /// and the commentary read's repetition. The read-forward continues, and a
    /// pass with nothing on screen is the read-forward alone.**
    pub const fn needs_on_screen(self) -> Option<WhatIsOnScreen> {
        match self {
            WhatAPassReads::TheReadForwardFromTheEventCursor => None,
            WhatAPassReads::TheQueueReading => Some(WhatIsOnScreen::TheServersQueueOrRunningEntry),
            WhatAPassReads::TheRunningEntrysCommentaryRead => {
                Some(WhatIsOnScreen::TheRunningEntrysCommentary)
            }
        }
    }
}

/// **The four things P10d names as not joining its set**, written out so that
/// an agent reaching for one meets the clause rather than an empty space.
///
/// **This is not a list of things that are unbuilt.** Each of them exists and
/// each is taken somewhere else on its own terms; what the clause forbids is
/// putting one on a repeating pass.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhatJoinsNothing {
    /// **`Servers` H15**: the hardware reading is taken on a connection's own
    /// sequence and not again.
    TheHardwareReading,
    /// **`Rented-server runtime` V2**: a health reading is its own thing and is
    /// not paced from here.
    AHealthReading,
    /// **§16.4 M15**: a free-space re-read. *As of the moment it was taken …
    /// never re-read to keep a message fresh.*
    AFreeSpaceReRead,
    /// **No condition record is this sequence's subject.** §8.4's rule is that
    /// nothing is polled to withdraw a record and no condition gets a watcher.
    ACondition,
}

impl WhatJoinsNothing {
    pub const ALL: [WhatJoinsNothing; 4] = [
        WhatJoinsNothing::TheHardwareReading,
        WhatJoinsNothing::AHealthReading,
        WhatJoinsNothing::AFreeSpaceReRead,
        WhatJoinsNothing::ACondition,
    ];
}

/// **What being on screen means for this sequence, and it is two things.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhatIsOnScreen {
    /// A surface showing that server's queue, or its running entry.
    TheServersQueueOrRunningEntry,
    /// That entry's commentary.
    TheRunningEntrysCommentary,
}

/// **What a pass takes, and the three are the whole of it.**
///
/// **Its one implementation reads a real machine**:
/// `crate::servers::the_machines_work::ThePassesOverThisConnection`, started
/// when `Servers` Q1 step 7 completes. **This is a seam and not a stand-in** —
/// the tests below answer it from a pass in hand, and nothing else does.
pub trait WhatAPassTakes {
    /// Whether the two on-screen reads are wanted this pass.
    fn is_on_screen(&mut self, server: &str, what: WhatIsOnScreen) -> bool;

    /// **The break declaration and the period ending are what end the
    /// sequence** — `Servers` K3 and `Servers` K5. P10d's own *nothing outside the
    /// period can outlive it*. **No pass is composed while the primary
    /// connection is not live.**
    fn the_period_stands_on_a_live_connection(&mut self, server: &str) -> bool;

    /// Take one of the three. **Nothing durable of this sequence's own is
    /// written**: what its reads write — the cursor's advance among them — is
    /// those mechanisms' own and unchanged.
    fn take(&mut self, server: &str, read: WhatAPassReads);
}

/// **The pause between one pass and the next — this stage's pick, which P10d
/// leaves to it by name.**
///
/// **Five seconds, and the reasoning rather than the number is the point.**
/// The promise it serves is a rendering entry that reads as live while a render
/// runs, so the figure has to be short enough that a moving bar moves and long
/// enough that a machine is never asked again before it has answered. **The
/// second half is carried by the pacing rather than by the figure** — one pass
/// at a time, measured from the end of the previous pass — so what the number
/// decides is only how stale the oldest thing on screen may be.
///
/// **It is no machine's measurement and claims to be none.** Nothing in this
/// project has observed what a queue reading costs, and the pacing rule is what
/// makes a slow machine safe rather than this figure.
pub const BETWEEN_ONE_PASS_AND_THE_NEXT: Duration = Duration::from_secs(5);

/// **What a pass reads, given what is on screen.**
///
/// **The read-forward is in every pass and the other two are conditional**,
/// which is P10d's own rule and the reason the set is expressed as a set rather
/// than as a function of the screen.
pub fn what_this_pass_reads<T: WhatAPassTakes>(takes: &mut T, server: &str) -> Vec<WhatAPassReads> {
    WhatAPassReads::ALL
        .into_iter()
        .filter(|read| match read.needs_on_screen() {
            None => true,
            Some(wanted) => takes.is_on_screen(server, wanted),
        })
        .collect()
}

/// **How the sequence ended.** Both are P10d's own closed list.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HowTheSequenceEnded {
    /// The break declaration (`Servers` K3, K5), there being no connection left
    /// to read on, **or** the period ending, which ends everything period-held.
    TheConnectionOrThePeriodWent,
}

/// **P10d's sequence, run for one server on one primary connection.**
///
/// **`Servers` Q18: a later connection's own step 7 starts a fresh one**, so
/// nothing here resumes and nothing here is held between connections.
///
/// **`Servers` Q19's re-take is a different sequence and is not in this set.**
/// That one runs before step 7 and settles the installation identity; this one
/// starts when step 7 has completed. Neither bounds the other.
pub async fn the_live_reading_sequence<T, W>(
    server: &str,
    takes: &mut T,
    wait: &mut W,
) -> HowTheSequenceEnded
where
    T: WhatAPassTakes,
    W: Wait,
{
    loop {
        if !takes.the_period_stands_on_a_live_connection(server) {
            return HowTheSequenceEnded::TheConnectionOrThePeriodWent;
        }
        // **One pass at a time.** Every read of this pass is taken before the
        // pause begins, so a slow machine is never stacked with concurrent
        // reads.
        for read in what_this_pass_reads(takes, server) {
            takes.take(server, read);
        }
        // **The pause is measured from the end of the previous pass and never
        // from its start** — P10d's own pacing rule, and `Servers` N3 is the
        // same rule stated for the repair.
        wait.until(BETWEEN_ONE_PASS_AND_THE_NEXT).await;
    }
}

#[cfg(test)]
mod tests {
    use std::time::SystemTime;

    use super::*;

    #[derive(Default)]
    struct APass {
        taken: Vec<(String, WhatAPassReads)>,
        queue_on_screen: bool,
        commentary_on_screen: bool,
        passes_left: usize,
    }

    impl WhatAPassTakes for APass {
        fn is_on_screen(&mut self, _server: &str, what: WhatIsOnScreen) -> bool {
            match what {
                WhatIsOnScreen::TheServersQueueOrRunningEntry => self.queue_on_screen,
                WhatIsOnScreen::TheRunningEntrysCommentary => self.commentary_on_screen,
            }
        }

        fn the_period_stands_on_a_live_connection(&mut self, _server: &str) -> bool {
            if self.passes_left == 0 {
                return false;
            }
            self.passes_left -= 1;
            true
        }

        fn take(&mut self, server: &str, read: WhatAPassReads) {
            self.taken.push((server.to_string(), read));
        }
    }

    struct TheWaiter {
        waited: Vec<Duration>,
    }

    impl Wait for TheWaiter {
        async fn until(&mut self, how_long: Duration) {
            self.waited.push(how_long);
        }
        fn now(&mut self) -> SystemTime {
            SystemTime::UNIX_EPOCH
        }
    }

    fn run(mut takes: APass) -> (APass, TheWaiter) {
        let mut waiter = TheWaiter { waited: Vec::new() };
        let ended = tauri::async_runtime::block_on(the_live_reading_sequence(
            "server-1",
            &mut takes,
            &mut waiter,
        ));
        assert_eq!(ended, HowTheSequenceEnded::TheConnectionOrThePeriodWent);
        (takes, waiter)
    }

    /// **The set is closed at three and a fourth read has nowhere to be taken
    /// from.**
    ///
    /// **This is the guard the clause asks for.** The set was written out of a
    /// scenario replay, and the four things kept out of it are named beside it
    /// rather than left to be rediscovered. **A variant added to
    /// [`WhatAPassReads`] fails here**; a read added anywhere else cannot be
    /// taken, because the sequence dispatches on this enum and nothing else.
    #[test]
    fn a_pass_reads_three_things_and_there_is_no_fourth() {
        let source = include_str!("live_reading.rs");
        let opens = source
            .find("pub enum WhatAPassReads {")
            .expect("WhatAPassReads is an enum");
        let body = &source[opens..];
        let set = &body[..body.find("\n}").expect("the enum closes")];
        let variants: Vec<&str> = set
            .lines()
            .map(str::trim)
            .filter(|line| !line.starts_with("///") && !line.starts_with("//"))
            .filter_map(|line| line.strip_suffix(','))
            .filter(|named| !named.is_empty() && !named.contains(' '))
            .collect();
        assert_eq!(
            variants,
            vec![
                "TheReadForwardFromTheEventCursor",
                "TheQueueReading",
                "TheRunningEntrysCommentaryRead",
            ],
            "**P10d's set of reads is closed**: the read-forward from the event \
             cursor, the queue reading and the running entry's commentary read. \
             **A fourth read has broken a clause that spent a scenario replay \
             being written**, and the four things that were kept out of it are \
             `WhatJoinsNothing`"
        );
        assert_eq!(WhatAPassReads::ALL.len(), 3);
    }

    /// **The four that do not join it are named, and none of them is in the
    /// set.**
    #[test]
    fn the_four_that_do_not_join_it_are_written_down_and_stay_out() {
        assert_eq!(WhatJoinsNothing::ALL.len(), 4);
        let set = format!("{:?}", WhatAPassReads::ALL);
        for kept_out in WhatJoinsNothing::ALL {
            assert!(
                !set.contains(&format!("{kept_out:?}")),
                "{kept_out:?} is in P10d's closed set. **It is named as not \
                 joining it**, and each of the four is taken somewhere else on \
                 its own terms"
            );
        }
    }

    /// **A pass with nothing on screen is the read-forward alone** — P10d's own
    /// sentence.
    #[test]
    fn a_pass_with_nothing_on_screen_is_the_read_forward_alone() {
        let (pass, _) = run(APass {
            passes_left: 1,
            ..APass::default()
        });
        assert_eq!(
            pass.taken
                .iter()
                .map(|(_, read)| *read)
                .collect::<Vec<WhatAPassReads>>(),
            vec![WhatAPassReads::TheReadForwardFromTheEventCursor]
        );
    }

    /// **The queue reading is repeated only while that server's queue is on
    /// screen, and the commentary read only while the commentary is.**
    #[test]
    fn the_two_conditional_reads_are_taken_only_while_their_surface_is_on_screen() {
        let (queue_only, _) = run(APass {
            passes_left: 1,
            queue_on_screen: true,
            ..APass::default()
        });
        assert_eq!(
            queue_only
                .taken
                .iter()
                .map(|(_, read)| *read)
                .collect::<Vec<WhatAPassReads>>(),
            vec![
                WhatAPassReads::TheReadForwardFromTheEventCursor,
                WhatAPassReads::TheQueueReading,
            ]
        );

        let (both, _) = run(APass {
            passes_left: 1,
            queue_on_screen: true,
            commentary_on_screen: true,
            ..APass::default()
        });
        assert_eq!(both.taken.len(), 3);

        // **The read-forward continues whatever is on screen**, which is what
        // makes T18's *now* bucket work for a user looking elsewhere.
        let (commentary_only, _) = run(APass {
            passes_left: 1,
            commentary_on_screen: true,
            ..APass::default()
        });
        assert_eq!(
            commentary_only
                .taken
                .iter()
                .map(|(_, read)| *read)
                .collect::<Vec<WhatAPassReads>>(),
            vec![
                WhatAPassReads::TheReadForwardFromTheEventCursor,
                WhatAPassReads::TheRunningEntrysCommentaryRead,
            ]
        );
    }

    /// **One pass at a time, and the pause falls after the pass rather than
    /// beside it** — so a slow machine is never stacked with concurrent reads.
    #[test]
    fn the_pause_falls_between_passes_and_never_inside_one() {
        let (pass, waiter) = run(APass {
            passes_left: 3,
            queue_on_screen: true,
            ..APass::default()
        });
        assert_eq!(pass.taken.len(), 6, "two reads a pass, three passes");
        assert_eq!(
            waiter.waited,
            vec![BETWEEN_ONE_PASS_AND_THE_NEXT; 3],
            "**P10d: one pass at a time, the pause between passes measured \
             from the end of the previous pass**"
        );
    }

    /// **What ends it: the break declaration, there being no connection left to
    /// read on, and the period ending.** No pass is composed after either.
    #[test]
    fn nothing_is_read_once_the_connection_or_the_period_has_gone() {
        let (pass, waiter) = run(APass::default());
        assert!(pass.taken.is_empty());
        assert!(waiter.waited.is_empty());
    }

    /// **P10a's single admitted repeating sequence of this section's own is
    /// single, and the check is over the whole crate rather than over this
    /// file.**
    ///
    /// **P10b: this section declares one sequence of the admitted kind and
    /// P10d is it.** `Download` W1 to W3 declares another and `Queue & History`
    /// W4 a third, and both are other documents' and are admitted on their own
    /// terms — **what this asserts is that this crate has not grown a second
    /// one of its own.**
    ///
    /// **The seam is what is counted, because it is the one thing a repeating
    /// pass cannot do without.** A sequence that paces itself takes
    /// `Servers`' waiting seam; a one-off pause after a failed call does not,
    /// and the crate has several of those, each bounded by the act it belongs
    /// to. **A second file taking the seam is a second sequence**, and it
    /// fails here rather than being noticed by somebody reading.
    #[test]
    fn this_document_starts_one_repeating_sequence_and_this_is_it() {
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
        let mut takes_the_seam = Vec::new();
        for (at, text) in every_rust_file_under(root) {
            let relative = beneath_the_crate(root, &at);
            let working = &text[..text.find("#[cfg(test)]").unwrap_or(text.len())];
            if working.contains("smoozy_servers::acting::Wait") {
                takes_the_seam.push(relative);
            }
        }
        takes_the_seam.sort();
        // **`src/servers/installation.rs` takes it for `Servers` Q19's re-take,
        // which is that document's sequence and not this one's** — `Servers`
        // Q19a hands it the connection a placement just committed on, and where
        // that re-take reads the fourth branch, Q19 governs, on its own two
        // figures (P10a's admitted form, as Q19 argues). It repeats nothing of
        // this document's. Added 2026-09-27, step 10c's eighth-pass fix piece 3.
        assert_eq!(
            takes_the_seam,
            vec![
                "src/servers/installation.rs".to_string(),
                "src/servers/live_reading.rs".to_string(),
            ],
            "**P10b: this section declares one sequence of the admitted kind              and P10d is it.** A second file that paces itself on a waiting              seam is a second repeating sequence of this document's own, and              P10 leaves this application one timer — the update check"
        );
    }

    /// A walked file's name as this test writes one: the part below the
    /// crate's own folder, with a forward slash between the parts on every
    /// system. The same function as `crate::the_store_standing`'s, and its head
    /// holds the whole account of why it is written this way.
    ///
    /// **In short: cutting the folder off the front as text is right at this
    /// desk and on Linux and wrong on Windows**, where the separator is a
    /// backslash, and a cut that misses leaves the whole path from the top of
    /// the disk — which loses every comparison against a name like
    /// `src/servers/live_reading.rs`. **This test had that fault from the day
    /// it was written and no Windows machine had ever run it**, so it would
    /// have failed the next hosted run rather than passing by luck.
    fn beneath_the_crate(root: &std::path::Path, at: &std::path::Path) -> String {
        at.strip_prefix(root)
            .expect("the walk hands back only files below the crate's own folder")
            .components()
            .map(|part| part.as_os_str().to_string_lossy().into_owned())
            .collect::<Vec<_>>()
            .join("/")
    }

    /// Every `.rs` file of this crate, as (path, text) — the same walk
    /// `crate::markers` and `crate::the_store_standing` make, and for the same
    /// reason: a rule that reads one level cannot fail on a file it never read.
    ///
    /// **It hands back the path itself rather than the path written out as
    /// text**, because the caller has to take the crate's folder off the front
    /// of it and that is a thing to do to a path.
    fn every_rust_file_under(at: &std::path::Path) -> Vec<(std::path::PathBuf, String)> {
        let mut out = Vec::new();
        for entry in std::fs::read_dir(at).expect("the folder is readable") {
            let path = entry.expect("the entry is readable").path();
            if path.is_dir() {
                if path.file_name().and_then(|s| s.to_str()) == Some("target") {
                    continue;
                }
                out.extend(every_rust_file_under(&path));
            } else if path.extension().and_then(|e| e.to_str()) == Some("rs") {
                let text = std::fs::read_to_string(&path).expect("the file is readable");
                out.push((path, text));
            }
        }
        out
    }

    /// **P10, P10a and P10b: this is a sequence with delays inside it and never
    /// a standing timer**, and P10d says so in its own last line.
    ///
    /// **What would make it a timer is a second figure or a schedule**, so the
    /// file holds exactly one figure — the interval P10d leaves to this stage —
    /// and nothing that fires with nothing under way.
    #[test]
    fn this_file_holds_one_figure_and_no_schedule() {
        let working: String = include_str!("live_reading.rs")
            .split("#[cfg(test)]")
            .next()
            .expect("the file has a working half")
            .lines()
            .filter(|line| !line.trim_start().starts_with("//"))
            .collect::<Vec<&str>>()
            .join("\n");
        assert_eq!(
            working.match_indices("Duration::from_").count(),
            1,
            "**P10d names no number and leaves the interval to this stage**, \
             and this file holds that one and no second figure"
        );
        for reached_for in ["thread::spawn", "thread::sleep", "Instant", "set_interval"] {
            assert!(
                !working.contains(reached_for),
                "this file names {reached_for}. **P10: there is exactly one \
                 timer on this side and it is the update check**, and this \
                 sequence is P10a's first admitted shape — a delay inside a \
                 sequence that is already running"
            );
        }
    }
}
