//! **The four kinds, and what composes each** — `Desktop app and delivery`
//! §13.3, T9 to T14 and T27.
//!
//! **Four kinds and no fifth** (T9), and every one of them is composed from
//! records this desktop already holds. **No store is added anywhere in this
//! file**: T11's *have all of this group's jobs reached a recorded ending* is
//! answered from the job records themselves, because every member job's local
//! record carries the group identity `State, identity and recovery` §2.2
//! issued when the all-scenes submission was composed.
//!
//! **The words are here too, and they are prose** — §17 X5: notification text
//! is drawn by the operating system and by no engine, so a rule about how the
//! application's own view lays out cannot reach it and **one transform at the
//! point strings are produced is what covers it**. Every piece of prose below is
//! built through [`crate::typography::Prose`], and the name of the thing a
//! notification is about never is: a job's or a server's own name is a value and
//! travels by the verbatim route (X7, X9).
//!
//! ## The words are Design's, and this file composes them
//!
//! **Repaired 2026-09-12, step 10a round 1 must-do F-3.** `Preparation/Design/
//! Whole app.md`'s *Notifications* section settles every one of the four in
//! full, labelled **settled at Design (2026-08-23)**, and **every kind is two
//! lines**: the first names the thing and what happened, the second carries the
//! figures a user wants before deciding whether to go and look. Its own
//! examples:
//!
//! - *Kitchen — final finished* over *240 frames in 6 hours 2 minutes · RunPod
//!   4090*
//! - *Product spins finished* over *4 scenes, 480 frames in 3 hours 14 minutes ·
//!   Vast 5060*, **and the scene count is what tells a batch from a single job**
//! - *Kitchen — final v3 failed* over *at frame 61 of 240 · RunPod 4090*
//! - *Kitchen — clay abandoned* over *after two interruptions, at frame 132 of
//!   240 · Old 4080*
//! - *Old 4080 disconnected* over *Could not reconnect.*
//! - *Vast 5060 is idle* over *No jobs for 30 minutes.*
//!
//! **What this file composed until that date was the opposite shape**: a generic
//! first line naming nothing — *Render finished*, *Batch finished* — with the
//! bare name second and not one of Design's figures anywhere, because
//! [`WhatToPost`] held one piece of prose and a name and had no third place to
//! put anything. **And the idle kind read *Machine idle for 30 minutes***, where
//! Design's *Words* is absolute and Creator-stated: *"The rented computer is a
//! **server**. The user's own is **your computer**. Never **machine**, whichever
//! of the two is meant."*
//!
//! **So a post carries two pieces of prose and a name.** Each piece of prose
//! goes through the point on its own; the name keeps its own route and is never
//! touched by it. **Joining them afterwards is not the point parsing anything**
//! — X2 binds the point, not the caller, and X10 already requires a block of
//! more than one sentence to hand each sentence over separately.
//!
//! **Where a figure is not known, this file says less rather than inventing
//! one.** Until the 38's piece 1 (2026-10-03) the second line of the three
//! ending kinds was empty, the ending carrying no figure at all. **It now
//! carries what the job's own local record holds once it is filled in**: the
//! duration (the note's two machine-clock moments), the frame a failed job was
//! on (`Queue & History` N6), and for a batch the count of its members. **The
//! frames a job asked for are on no record on this side** — they are the
//! file's own wherever the user typed no range — so *240 frames* and *of 240*
//! are left out, and each site says so. The server's label is a value and
//! travels beside the prose, on [`crate::notifications::APost::on`].
//!
//! **Markers used in this file that belong to another document, each with the
//! document it belongs to.** `State, identity and recovery` D5, F28.

use smoozy_queue_history::ending::Ending;
use smoozy_queue_history::idle::THE_HALF_HOUR;
use smoozy_state::identities::Identity;
use smoozy_state::observed::Reading;

use crate::presentation::the_presentation_key::ThePresentationKey;
use crate::typography::Prose;

/// **The four kinds are the spec's and no fifth is added** — T9.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Kind {
    /// A job finishing.
    AJobFinished,
    /// A job failing or being given up. **One kind, not two** — T9's own
    /// wording — and the title tells them apart because `Queue & History` E2
    /// keeps them apart on the entry.
    AJobFailedOrWasGivenUp,
    /// A connection Smoozy opened and could not restore.
    AConnectionCouldNotBeRestored,
    /// A machine that has had nothing to do for half an hour, which
    /// `Queue & History` owns and states in full.
    AMachineHasBeenIdle,
}

impl Kind {
    /// **T9: four and no fifth.** The line a test counts.
    pub const ALL: [Kind; 4] = [
        Kind::AJobFinished,
        Kind::AJobFailedOrWasGivenUp,
        Kind::AConnectionCouldNotBeRestored,
        Kind::AMachineHasBeenIdle,
    ];
}

/// **One thing to post**, composed and not yet handed to any system.
///
/// It carries the key T24 replaces on, the kind, and **the two pieces of prose
/// the user reads**. **What it does not carry is the name of the thing it is
/// about**: that is a value rather than prose (X7), it comes from the record the
/// caller is holding, and it is added at [`crate::notifications::APost`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WhatToPost {
    pub key: ThePresentationKey,
    pub kind: Kind,
    /// **The rest of the first line, after the name.** Design's first line is
    /// the thing and what happened — *Kitchen — final finished* — so the name is
    /// the caller's and this is what happened to it.
    pub said: Prose,
    /// **The second line.** Design's own: the figures a user wants before
    /// deciding whether to go and look, or, where Design gives words rather than
    /// figures, those words. **Empty where the figures are owed**, and the site
    /// that leaves it empty says which record will fill it.
    pub body: Prose,
}

/// **A job's recorded ending, as this desktop learned it.**
///
/// The ending is `Queue & History` E2's, mapped there from a recorded outcome
/// and from nothing else — no stream, no exit number, no timing.
///
/// **The three figures beside it are the job's own local record's, read once
/// the fill-in has carried the note's half over** — the 38's piece 1,
/// 2026-10-03. Each is absent where the record holds none, and **an absent
/// figure leaves its part of the second line out**; it is never written as
/// nought.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AJobsRecordedEnding {
    pub job: Identity,
    /// **The group identity the job's own local record carries**, where the
    /// job came from an all-scenes submission — T11. Absent for a job
    /// submitted by itself.
    pub group: Option<Identity>,
    pub ending: Ending,
    /// **When the render started and ended, on the machine's clock** — the
    /// note's two readings (`Queue & History` N4). Subtracted only by
    /// `State, identity and recovery` D5's own type, which refuses two clocks.
    pub started: Option<Reading<()>>,
    pub ended: Option<Reading<()>>,
    /// **`Queue & History` N6's figure: the frame the job was on** where it
    /// ended short of a clean finish, or `Queue & History` N1's *given up, where it was* behind
    /// it — the same figure History's entry draws, and never one reckoned by
    /// adding one to another.
    pub at_frame: Option<i64>,
}

/// **One member of a group, as its own job record stands.**
///
/// `ending` is absent where that member has not reached a recorded ending yet,
/// which is the only question T11 asks of the other members. **Its two
/// moments are for the batch's own duration** — from the first member's start
/// to the last member's end, on the one machine's clock.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AGroupMember {
    pub job: Identity,
    pub ending: Option<Ending>,
    pub started: Option<Reading<()>>,
    pub ended: Option<Reading<()>>,
}

/// **T10 to T12 and T27, performed: what one job's ending posts.**
///
/// The list is at most two entries long and each of the two has a reason:
/// **a failing job speaks for itself, group or not** (T12), and **the last
/// member of a group carries the group's own finishing notification** (T11,
/// T27). A job that fails as the last of its group therefore posts both, which
/// is the spec's own split — *"what stops a batch's one message hiding a
/// failure inside it"* — meeting the group's completion in the same moment.
///
/// `the_groups_members` is every job record this desktop holds carrying the
/// same group identity, **including the one that just ended**, with each
/// member's own recorded ending where it has one. Where the ending has no
/// group the argument is ignored.
pub fn what_this_ending_posts(
    ended: &AJobsRecordedEnding,
    the_groups_members: &[AGroupMember],
) -> Vec<WhatToPost> {
    let mut posts = Vec::new();

    // **T12: a job that fails still speaks for itself**, group or not, at the
    // moment its failure is learned.
    match ended.ending {
        Ending::Failed | Ending::GivenUp => posts.push(WhatToPost {
            key: ThePresentationKey::AJobsEnding(ended.job.clone()),
            kind: Kind::AJobFailedOrWasGivenUp,
            said: what_an_ending_says(ended.ending),
            // **Design's second line for these two is figures and nothing
            // else** — *at frame 61 of 240 · RunPod 4090*, and *after two
            // interruptions, at frame 132 of 240 · Old 4080*. **What the
            // record holds is the frame**, and that part is said. **The frame
            // total is the frames the job asked for**, which is the file's own
            // wherever the user typed no range and is on no record on this
            // side; **the interruption count is not said either**, the note's
            // attempt count and re-run marks not being the same figure as
            // Design's *interruptions* by any clause. Both parts are left out
            // rather than reckoned. The server is [`crate::notifications::APost::on`].
            body: match ended.at_frame {
                Some(frame) => Prose::new(format!("at frame {frame}")),
                None => Prose::new(""),
            },
        }),
        // **T10: a job the user stopped is deliberately not among the four**,
        // so a job whose recorded outcome is *left stopped* posts nothing.
        Ending::Stopped => {}
        Ending::Finished => {
            // **T11: no member posts its own finishing notification.** A job
            // submitted by itself has no group and posts its own.
            if ended.group.is_none() {
                posts.push(WhatToPost {
                    key: ThePresentationKey::AJobsEnding(ended.job.clone()),
                    kind: Kind::AJobFinished,
                    said: what_an_ending_says(Ending::Finished),
                    // **Design's second line here is *240 frames in 6 hours 2
                    // minutes · RunPod 4090*** — two figures, *"because those
                    // are the two a user wants before deciding whether to go and
                    // look"*, and the server label. **The duration is the
                    // record's** and is said. **The frame count is the frames
                    // the job asked for**, which is on no record on this side,
                    // so that part and the word joining it to the duration are
                    // left out. The server is [`crate::notifications::APost::on`].
                    body: Prose::new(
                        how_long_it_rendered(ended.started.as_ref(), ended.ended.as_ref())
                            .unwrap_or_default(),
                    ),
                });
            }
        }
    }

    if let Some(group) = &ended.group {
        if the_group_finishes_now(the_groups_members) {
            posts.push(WhatToPost {
                key: ThePresentationKey::AGroupsFinishing(group.clone()),
                kind: Kind::AJobFinished,
                // **A batch is not a fifth kind and does not say a different
                // word.** Design: *Product spins finished* — the batch's own
                // name and the same *finished* — and **the scene count is what
                // tells a batch from a single job**, which is the second line's
                // work and not the first's.
                said: what_an_ending_says(Ending::Finished),
                // **Design's second line here is *4 scenes, 480 frames in 3
                // hours 14 minutes · Vast 5060***, and the scene count is the
                // part that is doing the telling. **It is a count of this
                // group's members**, which `State, identity and recovery`
                // §2.2's group identity already gathers. **The duration is the
                // batch's own: from the first member's start to the last
                // member's end**, on the one machine's clock — a reading of
                // the records, never a sum of members. The frames are the
                // frames each job asked for and are on no record here, so that
                // part is left out. The server is
                // [`crate::notifications::APost::on`].
                body: Prose::new(the_batchs_figures(the_groups_members)),
            });
        }
    }

    posts
}

/// **How long a render took, in Design's own words** — *6 hours 2 minutes*,
/// *47 minutes*, *less than a minute*.
///
/// **The two moments are the note's, on the machine's clock** (`Queue &
/// History` N4), and the subtraction is `State, identity and recovery` D5's
/// own type's: two clocks, or an end before its start, give nothing, and then
/// nothing is said. **A part of nought is left out** — *6 hours*, not *6 hours
/// 0 minutes* — and a render shorter than a minute is never *0 minutes*.
pub fn how_long_it_rendered(
    started: Option<&Reading<()>>,
    ended: Option<&Reading<()>>,
) -> Option<String> {
    let took = ended?.since(started?)?;
    let minutes = took.as_secs() / 60;
    let (hours, minutes) = (minutes / 60, minutes % 60);
    let a = |figure: u64, one: &str, more: &str| {
        crate::typography::a_figure_with_its_unit(figure, if figure == 1 { one } else { more })
    };
    Some(match (hours, minutes) {
        (0, 0) => "less than a minute".to_string(),
        (0, minutes) => a(minutes, "minute", "minutes"),
        (hours, 0) => a(hours, "hour", "hours"),
        (hours, minutes) => format!("{} {}", a(hours, "hour", "hours"), a(minutes, "minute", "minutes")),
    })
}

/// **A batch's own figures: how many scenes, and how long from the first
/// start to the last end** — Design's *4 scenes, 480 frames in 3 hours 14
/// minutes*, less the frames.
fn the_batchs_figures(members: &[AGroupMember]) -> String {
    let scenes = crate::typography::a_figure_with_its_unit(
        members.len(),
        if members.len() == 1 { "scene" } else { "scenes" },
    );
    let first_start = members
        .iter()
        .filter_map(|member| member.started.as_ref())
        .min_by_key(|at| at.taken_at());
    let last_end = members
        .iter()
        .filter_map(|member| member.ended.as_ref())
        .max_by_key(|at| at.taken_at());
    match how_long_it_rendered(first_start, last_end) {
        Some(took) => format!("{scenes} in {took}"),
        None => scenes,
    }
}

/// **T11 and T27's test, and the two halves are separate promises.**
///
/// **Every member has reached a recorded ending** — which is answered from the
/// job records this desktop already holds, with no group record, no count kept
/// beside them and nothing read from a machine — **and at least one member
/// finished.**
///
/// **A left-stopped member satisfies the last-member test** (T27): T10 removes
/// that member's *own* post and says nothing about the group's completion, and
/// the spec's *"when the whole batch is done"* is not served by a batch that
/// is done and silent.
///
/// **A group all of whose members failed or were given up posts no finishing
/// notification at all** (T27), its members' own T12 posts being the whole of
/// what is said — because otherwise an all-scenes submission whose five
/// members all fail would post five failure notifications and then tell the
/// user, over T15's picture, that the batch finished, for a batch in which
/// nothing finished.
pub fn the_group_finishes_now(members: &[AGroupMember]) -> bool {
    if members.is_empty() {
        return false;
    }
    let all_have_ended = members.iter().all(|member| member.ending.is_some());
    let at_least_one_finished = members
        .iter()
        .any(|member| member.ending == Some(Ending::Finished));
    all_have_ended && at_least_one_finished
}

/// **T13: the connection kind is raised from the condition record and not from
/// a reading.**
///
/// `Servers` supplies the observed value under `State, identity and recovery`
/// §8.5's constraint — a break reported **without the caller having to use the
/// connection first** — and Fact 57 is why that constraint exists: a caller's
/// own line pulled went undetected for 21 minutes 45 seconds with keepalives
/// off. **The notification is posted when the condition is raised and not when
/// a request happens to fail.**
pub fn what_a_raised_connection_condition_posts(condition: &Identity) -> WhatToPost {
    WhatToPost {
        key: ThePresentationKey::ACondition(condition.clone()),
        kind: Kind::AConnectionCouldNotBeRestored,
        // **Design's own two lines** — *Old 4080 disconnected* over *Could not
        // reconnect.* The server's name is the caller's and comes by the
        // verbatim route.
        said: Prose::new("disconnected"),
        // **Design gives this kind words rather than figures**, so the whole of
        // it lands now. Its own note says why nothing more is said: *"No cause
        // named, because three quite different things look identical from
        // here."*
        body: Prose::new("Could not reconnect."),
    }
}

/// **T14: the idle kind is `Queue & History`'s reading and its half hour, and
/// nothing here re-derives either.**
///
/// The half hour in the title is read from that document's own constant rather
/// than written again — a number in two places is the fault this project keeps
/// meeting, and a title saying thirty minutes beside a judgement made at some
/// other length would be Smoozy wrong on screen about its own reading.
pub fn what_an_idle_machine_posts(server: &Identity) -> WhatToPost {
    WhatToPost {
        key: ThePresentationKey::AMachineIdle(server.clone()),
        kind: Kind::AMachineHasBeenIdle,
        // **Design's own two lines** — *Vast 5060 is idle* over *No jobs for
        // 30 minutes.* The server's name is the caller's and comes by the
        // verbatim route.
        //
        // **This kind read *Machine idle for 30 minutes* until 2026-09-12**, and
        // Design's *Words* forbids that one word outright and Creator-stated:
        // *"The rented computer is a **server**. The user's own is **your
        // computer**. Never **machine**, whichever of the two is meant"* — the
        // reason being that a word meaning either end of a transfer is the last
        // word an interface should use.
        said: Prose::new("is idle"),
        // **Design gives this kind words rather than figures**, with the half
        // hour inside them, and its own note says why the length is said at all:
        // *"This one is about money, so it says how long rather than only that
        // it happened."* The half hour is read from `Queue & History`'s own
        // constant rather than written again — a number in two places is the
        // fault this project keeps meeting, and a line saying thirty minutes
        // beside a judgement made at some other length would be Smoozy wrong on
        // screen about its own reading.
        body: Prose::new(format!(
            "No jobs for {}.",
            crate::typography::a_figure_with_its_unit(THE_HALF_HOUR.as_secs() / 60, "minutes")
        )),
    }
}

/// **What happened to a job, as the first line's second half** — `Queue &
/// History` E2's four words, and Design's own first lines: *Kitchen — final
/// finished*, *Kitchen — final v3 failed*, *Kitchen — clay abandoned*. The
/// ending's stored name stays *given up*; only the word shown changed
/// (2026-10-28, the final words sweep).
///
/// Rule 9: the standard software words a 3D artist has read for twenty years.
/// **Stopped has no words because it has no notification** (T10), and the
/// function says so by refusing to make one rather than by returning something
/// nobody may use.
fn what_an_ending_says(ending: Ending) -> Prose {
    match ending {
        Ending::Finished => Prose::new("finished"),
        Ending::Failed => Prose::new("failed"),
        Ending::GivenUp => Prose::new("abandoned"),
        Ending::Stopped => {
            debug_assert!(false, "T10: a job the user stopped posts nothing");
            Prose::new("stopped")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use smoozy_state::identities::IdentityKind;

    fn a_job() -> Identity {
        Identity::issue(IdentityKind::Job).expect("the desk has randomness")
    }

    fn a_group() -> Identity {
        Identity::issue(IdentityKind::Group).expect("the desk has randomness")
    }

    fn ended(job: &Identity, group: Option<&Identity>, ending: Ending) -> AJobsRecordedEnding {
        AJobsRecordedEnding {
            job: job.clone(),
            group: group.cloned(),
            ending,
            started: None,
            ended: None,
            at_frame: None,
        }
    }

    fn member(job: &Identity, ending: Option<Ending>) -> AGroupMember {
        AGroupMember {
            job: job.clone(),
            ending,
            started: None,
            ended: None,
        }
    }

    /// A moment on one machine's clock, so two of them subtract.
    fn on_the_machine(seconds: u64) -> Reading<()> {
        Reading::taken(
            (),
            std::time::SystemTime::UNIX_EPOCH + std::time::Duration::from_secs(seconds),
            smoozy_state::observed::Clock::AMachine(smoozy_state::observed::MachineClock(
                "the machine".to_string(),
            )),
        )
    }

    /// **T9: four kinds and no fifth.**
    #[test]
    fn there_are_four_kinds() {
        assert_eq!(Kind::ALL.len(), 4);
        let mut seen = Kind::ALL.to_vec();
        seen.dedup();
        assert_eq!(seen.len(), 4);
    }

    /// **T10: a job the user stopped posts nothing.**
    ///
    /// This is the line that fails if somebody adds the fifth kind by giving
    /// *stopped* a message, which reads like completeness and is exactly what
    /// T9 refuses.
    #[test]
    fn a_job_the_user_stopped_posts_nothing() {
        let job = a_job();
        assert!(what_this_ending_posts(&ended(&job, None, Ending::Stopped), &[]).is_empty());
    }

    /// A job submitted by itself posts its own ending, all three ways it can
    /// have one that speaks.
    #[test]
    fn a_job_on_its_own_posts_its_own_ending() {
        for (ending, kind) in [
            (Ending::Finished, Kind::AJobFinished),
            (Ending::Failed, Kind::AJobFailedOrWasGivenUp),
            (Ending::GivenUp, Kind::AJobFailedOrWasGivenUp),
        ] {
            let job = a_job();
            let posts = what_this_ending_posts(&ended(&job, None, ending), &[]);
            assert_eq!(posts.len(), 1, "{ending:?} posts once");
            assert_eq!(posts[0].kind, kind);
            assert_eq!(posts[0].key, ThePresentationKey::AJobsEnding(job));
        }
    }

    /// **T11: a batch speaks once, and no member posts its own finishing
    /// notification.**
    ///
    /// Three members, two already finished; the third finishing is the last,
    /// and what comes out is **one** post, keyed on the group.
    #[test]
    fn a_batch_speaks_once_and_only_when_the_last_member_ends() {
        let group = a_group();
        let (one, two, three) = (a_job(), a_job(), a_job());

        // The first member finishing posts nothing at all: it is neither its
        // own finishing notification nor the group's, the group not being done.
        let posts = what_this_ending_posts(
            &ended(&one, Some(&group), Ending::Finished),
            &[
                member(&one, Some(Ending::Finished)),
                member(&two, None),
                member(&three, None),
            ],
        );
        assert!(posts.is_empty());

        // The last one carries the whole batch's message, once.
        let posts = what_this_ending_posts(
            &ended(&three, Some(&group), Ending::Finished),
            &[
                member(&one, Some(Ending::Finished)),
                member(&two, Some(Ending::Finished)),
                member(&three, Some(Ending::Finished)),
            ],
        );
        assert_eq!(posts.len(), 1);
        assert_eq!(posts[0].key, ThePresentationKey::AGroupsFinishing(group));
        assert_eq!(posts[0].kind, Kind::AJobFinished);
        // **Design's own first line for a batch is the batch's name and
        // *finished***, the same word a single job takes — *Product spins
        // finished*. The word is not what tells the two apart; the scene count
        // on the second line is, and it lands with the record that holds it.
        assert_eq!(posts[0].said.as_str(), "finished");
    }

    /// **T27's first half: a group all of whose members failed posts no
    /// finishing notification at all.**
    ///
    /// The harm this prevents is named in T27 itself: five failure
    /// notifications and then, over the batch's own picture, a message saying
    /// the batch finished — for a batch in which nothing finished.
    #[test]
    fn a_group_in_which_nothing_finished_posts_no_finishing_notification() {
        let group = a_group();
        let (one, two) = (a_job(), a_job());
        let posts = what_this_ending_posts(
            &ended(&two, Some(&group), Ending::Failed),
            &[
                member(&one, Some(Ending::GivenUp)),
                member(&two, Some(Ending::Failed)),
            ],
        );
        assert_eq!(posts.len(), 1);
        assert_eq!(posts[0].kind, Kind::AJobFailedOrWasGivenUp);
        assert_eq!(posts[0].key, ThePresentationKey::AJobsEnding(two));
    }

    /// **T27's second half: a left-stopped member still counts towards the
    /// last.**
    ///
    /// T10 removes that member's *own* post and says nothing about the group's
    /// completion. Without this the batch would be done and silent.
    #[test]
    fn a_left_stopped_member_still_counts_towards_the_last() {
        let group = a_group();
        let (one, two) = (a_job(), a_job());
        let posts = what_this_ending_posts(
            &ended(&two, Some(&group), Ending::Stopped),
            &[
                member(&one, Some(Ending::Finished)),
                member(&two, Some(Ending::Stopped)),
            ],
        );
        assert_eq!(posts.len(), 1);
        assert_eq!(posts[0].key, ThePresentationKey::AGroupsFinishing(group));
    }

    /// **T12 and T27 meeting in one moment**: the last member of a group fails,
    /// and an earlier one had finished. Two posts, and both are owed.
    #[test]
    fn a_failing_last_member_speaks_for_itself_and_carries_the_batch() {
        let group = a_group();
        let (one, two) = (a_job(), a_job());
        let posts = what_this_ending_posts(
            &ended(&two, Some(&group), Ending::Failed),
            &[
                member(&one, Some(Ending::Finished)),
                member(&two, Some(Ending::Failed)),
            ],
        );
        assert_eq!(posts.len(), 2);
        assert_eq!(posts[0].kind, Kind::AJobFailedOrWasGivenUp);
        assert_eq!(posts[1].key, ThePresentationKey::AGroupsFinishing(group));
    }

    /// **T11: the answer comes from the job records and from no group store.**
    ///
    /// The judgement takes members and nothing else — no count, no group
    /// record, nothing read from a machine — so a member the desktop has not
    /// heard about keeps the batch open rather than closing it early.
    #[test]
    fn a_member_that_has_not_ended_keeps_the_batch_open() {
        let (one, two) = (a_job(), a_job());
        assert!(!the_group_finishes_now(&[
            member(&one, Some(Ending::Finished)),
            member(&two, None),
        ]));
        assert!(!the_group_finishes_now(&[]));
    }

    /// **T14: the half hour is `Queue & History`'s and is not written again
    /// here — and the word Design bans is not used.**
    ///
    /// The half-hour half is the line that fails if somebody types thirty into
    /// the words, which would leave them and the judgement free to disagree.
    ///
    /// **The other half is Design's *Words*, and it is absolute and
    /// Creator-stated**: *"The rented computer is a **server**. The user's own
    /// is **your computer**. Never **machine**, whichever of the two is
    /// meant."* This kind read *Machine idle for 30 minutes* until 2026-09-12
    /// (F-3), and the reason the word is banned is that a word meaning either
    /// end of a transfer is the last word an interface should use.
    #[test]
    fn the_idle_kind_reads_the_half_hour_from_the_document_that_owns_it() {
        let server =
            Identity::issue(IdentityKind::LocalServer).expect("the desk has randomness");
        let post = what_an_idle_machine_posts(&server);
        assert_eq!(post.kind, Kind::AMachineHasBeenIdle);
        // Design's own two lines, with the name the caller's.
        assert_eq!(post.said.as_str(), "is\u{00A0}idle");
        assert_eq!(
            post.body.as_str(),
            "No jobs for 30\u{00A0}minutes."
        );
        assert_eq!(THE_HALF_HOUR.as_secs(), 30 * 60);

        for post in every_kind_composed() {
            for said in [post.said.as_str(), post.body.as_str()] {
                let said = said.to_lowercase();
                assert!(
                    !said.split(|c: char| !c.is_alphabetic()).any(|word| word == "machine"),
                    "a notification says `machine`. **Design's *Words* is \
                     absolute and Creator-stated: the rented computer is a \
                     server, the user's own is your computer, never machine, \
                     whichever of the two is meant**"
                );
            }
        }
    }

    /// **T13: the connection kind is keyed on the condition record's own
    /// identity**, which is `State, identity and recovery` F28's — so a
    /// hundred refusals of one condition are one post, whatever their words.
    #[test]
    fn the_connection_kind_is_keyed_on_the_condition_identity() {
        let condition =
            Identity::issue(IdentityKind::Condition).expect("the desk has randomness");
        let post = what_a_raised_connection_condition_posts(&condition);
        assert_eq!(post.key, ThePresentationKey::ACondition(condition.clone()));
        assert_eq!(
            what_a_raised_connection_condition_posts(&condition).key,
            post.key
        );
    }

    /// **Every kind this file composes, for the checks that are about all of
    /// them.**
    fn every_kind_composed() -> Vec<WhatToPost> {
        let job = a_job();
        let group = a_group();
        let server =
            Identity::issue(IdentityKind::LocalServer).expect("the desk has randomness");
        let condition =
            Identity::issue(IdentityKind::Condition).expect("the desk has randomness");

        let mut all = vec![
            what_a_raised_connection_condition_posts(&condition),
            what_an_idle_machine_posts(&server),
        ];
        for ending in [Ending::Finished, Ending::Failed, Ending::GivenUp] {
            all.extend(what_this_ending_posts(&ended(&job, None, ending), &[]));
        }
        // The batch, which is the one composed only when a group completes.
        all.extend(what_this_ending_posts(
            &ended(&job, Some(&group), Ending::Finished),
            &[member(&job, Some(Ending::Finished))],
        ));
        all
    }

    /// **X5: every piece of prose has been through the typography point.**
    ///
    /// Notification text is drawn by the operating system and by no engine, so
    /// nothing about the application's own view can reach it — X5's own reason
    /// for the point covering more than the page.
    ///
    /// **The check is on the type rather than on the characters.** A one-word
    /// piece of prose — *finished* — has no inter-word space for the point to
    /// bind, and it has been through the point all the same; a check for a
    /// non-breaking space would refuse exactly the strings Design asks for. The
    /// multi-word ones are shown binding so the point is watched working as well
    /// as typed.
    #[test]
    fn every_piece_of_prose_has_been_through_the_typography_point() {
        for post in every_kind_composed() {
            let _: &Prose = &post.said;
            let _: &Prose = &post.body;
        }

        let server =
            Identity::issue(IdentityKind::LocalServer).expect("the desk has randomness");
        let condition =
            Identity::issue(IdentityKind::Condition).expect("the desk has randomness");
        assert!(what_an_idle_machine_posts(&server)
            .said
            .as_str()
            .contains('\u{00A0}'));
        assert!(what_a_raised_connection_condition_posts(&condition)
            .body
            .as_str()
            .contains('\u{00A0}'));
    }

    /// **Every kind says Design's own words** — F-3, and this is the check that
    /// would have caught what was there.
    ///
    /// **What was there.** A generic first line naming nothing — *Render
    /// finished*, *Render failed*, *Batch finished*, *Connection could not be
    /// restored* — with the bare name second, and **not one of Design's figures
    /// carried at all**, because the shape a notification was built from held
    /// one piece of prose and one name and had no third place to put anything.
    /// **The batch line said *Batch finished* over nothing**, so it could not be
    /// told from a single job, which is the one thing Design says that line
    /// exists to do.
    #[test]
    fn every_kind_says_what_design_settled_it_says() {
        let job = a_job();
        let server =
            Identity::issue(IdentityKind::LocalServer).expect("the desk has randomness");
        let condition =
            Identity::issue(IdentityKind::Condition).expect("the desk has randomness");

        // *Kitchen — final finished*, *Kitchen — final v3 failed*, *Kitchen —
        // clay abandoned*: the name, then what happened to it.
        for (ending, said) in [
            (Ending::Finished, "finished"),
            (Ending::Failed, "failed"),
            (Ending::GivenUp, "abandoned"),
        ] {
            let posts = what_this_ending_posts(&ended(&job, None, ending), &[]);
            assert_eq!(posts[0].said.as_str(), said, "{ending:?}");
        }

        // *Old 4080 disconnected* over *Could not reconnect.*
        let connection = what_a_raised_connection_condition_posts(&condition);
        assert_eq!(connection.said.as_str(), "disconnected");
        assert_eq!(
            connection.body.as_str(),
            "Could not\u{00A0}reconnect."
        );

        // *Vast 5060 is idle* over *No jobs for 30 minutes.*
        let idle = what_an_idle_machine_posts(&server);
        assert_eq!(idle.said.as_str(), "is\u{00A0}idle");

        // **No kind opens with a word that names nothing.** The first line is
        // the thing and what happened, and the thing is the caller's name.
        for post in every_kind_composed() {
            for generic in ["Render ", "Batch ", "Connection ", "Machine "] {
                assert!(
                    !post.said.as_str().starts_with(generic),
                    "a notification's first line opens with `{generic}` and names \
                     nothing. **Design's first line names the thing and what \
                     happened** — *Kitchen — final finished*, not *Render \
                     finished*"
                );
            }
        }
    }

    /// **The second line carries the figures the job's record holds, and an
    /// absent one is left out rather than written as nought** — the 38's piece
    /// 1, over Design's *240 frames in 6 hours 2 minutes*, *at frame 61 of
    /// 240* and *4 scenes, 480 frames in 3 hours 14 minutes*, less the frames
    /// a job asked for, which no record on this side holds.
    ///
    /// **This is the line that fails if the second line goes back to empty**,
    /// which is what every ending posted until this piece, and if a missing
    /// figure is drawn as a nought.
    #[test]
    fn the_second_line_says_the_records_figures_and_leaves_an_absent_one_out() {
        let job = a_job();
        let finished = AJobsRecordedEnding {
            started: Some(on_the_machine(1_000)),
            ended: Some(on_the_machine(1_000 + 6 * 3_600 + 2 * 60 + 30)),
            ..ended(&job, None, Ending::Finished)
        };
        let posts = what_this_ending_posts(&finished, &[]);
        assert_eq!(posts[0].body.as_str(), "6\u{00A0}hours 2\u{00A0}minutes");

        // **No figure, no part, and no nought.**
        let posts = what_this_ending_posts(&ended(&job, None, Ending::Finished), &[]);
        assert_eq!(posts[0].body.as_str(), "");
        assert_eq!(
            how_long_it_rendered(Some(&on_the_machine(0)), Some(&on_the_machine(59))).as_deref(),
            Some("less than a minute")
        );
        assert_eq!(
            how_long_it_rendered(Some(&on_the_machine(0)), Some(&on_the_machine(3_600))).as_deref(),
            Some("1\u{00A0}hour")
        );
        // **Two clocks, or an end before its start, give nothing.**
        let here = Reading::taken((), std::time::SystemTime::UNIX_EPOCH, smoozy_state::observed::Clock::ThisComputer);
        assert_eq!(how_long_it_rendered(Some(&here), Some(&on_the_machine(60))), None);
        assert_eq!(how_long_it_rendered(Some(&on_the_machine(60)), Some(&on_the_machine(0))), None);

        // *at frame 61*, and nothing where the record holds no frame.
        let failed = AJobsRecordedEnding {
            at_frame: Some(61),
            ..ended(&job, None, Ending::Failed)
        };
        assert_eq!(what_this_ending_posts(&failed, &[])[0].body.as_str(), "at frame\u{00A0}61");
        assert_eq!(
            what_this_ending_posts(&ended(&job, None, Ending::GivenUp), &[])[0].body.as_str(),
            ""
        );

        // *4 scenes in 3 hours 14 minutes*: the count of members, and the
        // first start to the last end.
        let group = a_group();
        let jobs: Vec<Identity> = (0..4).map(|_| a_job()).collect();
        let members: Vec<AGroupMember> = jobs
            .iter()
            .enumerate()
            .map(|(at, job)| AGroupMember {
                started: Some(on_the_machine(10_000 + at as u64 * 2_000)),
                ended: Some(on_the_machine(10_000 + at as u64 * 2_000 + 1_000)),
                ..member(job, Some(Ending::Finished))
            })
            .collect();
        let last = members[3].ended.clone().expect("ended").taken_at();
        assert_eq!(
            last.duration_since(std::time::SystemTime::UNIX_EPOCH).unwrap().as_secs(),
            10_000 + 6_000 + 1_000
        );
        let posts = what_this_ending_posts(&ended(&jobs[3], Some(&group), Ending::Finished), &members);
        assert_eq!(posts.len(), 1);
        assert_eq!(
            posts[0].body.as_str(),
            "4\u{00A0}scenes in 1\u{00A0}hour 56\u{00A0}minutes"
        );
        // Without the moments, the count alone.
        let bare: Vec<AGroupMember> = jobs.iter().map(|job| member(job, Some(Ending::Finished))).collect();
        assert_eq!(
            what_this_ending_posts(&ended(&jobs[3], Some(&group), Ending::Finished), &bare)[0]
                .body
                .as_str(),
            "4\u{00A0}scenes"
        );
    }
}
