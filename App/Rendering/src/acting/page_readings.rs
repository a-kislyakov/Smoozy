//! **§5 W1 and §6.3 to §6.4 performed on the desk — the check standing read
//! out of the local record, the app-wide reporting item read at composition,
//! and A7's three-way reading of a machine taken over the command channel.**
//!
//! # The trait, and why it is synchronous
//!
//! [`Readings`] is the split `App/Keys/` drew at its own `usable::Readings`
//! and every crate since has kept: **the judgements above stay testable at
//! the desk with the record's answers arriving as values.** Three of its
//! five are reads of something Smoozy already holds — the check standing
//! (`Dependency Upload & Remap` V10 to V14's record), the app-wide reporting
//! item (one user-held item of the local record's settings), and the figure
//! the entry kept (`Queue & History` N6).
//!
//! **Two of the five do reach the machine, and they were added on
//! 2026-09-05**: A4's walk of the job output folder and A4's read of the
//! machine's own latest attempt record. **A4 requires both by name** — *the
//! figure is read from the machine when the action is taken, never
//! remembered from when the job ended* — and until they existed nothing in
//! this crate did any of it, so A9's could-not-check exception was the whole
//! of the behaviour. They are on this trait rather than on a second seam
//! because this trait is what the page's judgements already stand on, and a
//! second seam over the same page would be a second thing to keep true.
//!
//! # The reading of a machine, and the one thing it is not
//!
//! **A7's three-way distinction is taken over the command channel, and the
//! separator is the stat's own exit status.** Observed with controls either
//! side on the rented Linux box, on the job output folder itself: *"The stat
//! of this folder separates them by exit status — 0 against 1 — and the
//! directory test does not: `test -d` exits 0 for both."* A plain listing
//! exits 2 either way and is separated only by the English message, **which
//! nothing here reads.**
//!
//! **`test`'s own specification is why the file test could not have carried
//! it in any case** — The Open Group Base Specifications Issue 8, IEEE Std
//! 1003.1-2024, `test — evaluate expression`, read 2026-08-23: *"0 —
//! expression evaluated to true. 1 — expression evaluated to false or
//! expression was missing. >1 — An error occurred."* **No third value for *I
//! was not allowed to look*.**
//!
//! **A7a: what the stat does not separate is a path whose own stat is
//! refused**, which is what an unreadable parent produces, and there the walk
//! of the parent components is the separator. A macOS run of 2026-08-21 put
//! a stat and a file test on a path *under* an unreadable parent and both
//! exited 1 in either case — **a case not posed on the rented box**, and
//! §9 item 15's bound stands over it like every other machine claim here.
//!
//! # No path is ever written into a command
//!
//! `Server protocol and transfers` §1.1 makes a command string a
//! compile-time constant, so a machine-origin path rides that document's own
//! planted-link carriage and **the command names the token**. That is the
//! arrangement `Server Setup` already uses for its version ask,
//! `Render Settings` for its four-value read and `Dependency Upload & Remap`
//! for its own — **this module composes no command of its own and invents no
//! second carriage.**
//!
//! # What is never mined
//!
//! **No wording of the machine's is ever read for a cause** (`Job files and
//! outputs` C26). Every answer below is an exit status, and where a status
//! is absent the reading is A8's *did not complete* rather than a guess.
//!
//! **Markers used in this file that belong to another document, each with
//! the document it belongs to.** `Dependency Upload & Remap` V10, V11, V12,
//! V13, V14. `Queue & History` N6. `Job files and outputs` C4, C7, C9, C18,
//! C18a, C19, C26. `Server
//! protocol and transfers` §1.1.

use smoozy_server_protocol::carriage::{self, a_command_naming, CarriageSite};
use smoozy_server_protocol::subsystem::client::Subsystem;
use smoozy_server_protocol::transport::CommandString;
use smoozy_servers::readings::CommandChannel;
use smoozy_state::identities::Identity;
use tokio::io::{AsyncRead, AsyncWrite};

use crate::again::{
    the_could_not_check_case, the_carry_ons_figure_over, TheCarryOnsFigure,
    TheMachineCouldNotBeChecked, TheWalk, WhatTheCarryOnRead, WhatTheReadingGave,
};
use smoozy_dependencies::standing::WhatTheRecordSays;
use smoozy_job_files::inventory::WhatTheInventoryIs;
use smoozy_job_files::reckoning::WhatTheWalkOfTheFolderIs;

/// The readings this document's judgements stand on, behind one trait the
/// acting half implements.
///
/// **Every one is a read of something Smoozy already holds, and none reaches
/// a machine from here.**
pub trait Readings {
    /// **W1: the check standing for the three parts of its key** — the root
    /// file's machine path, the machine under its installation identity, and
    /// the submission's Blender version. A standing under a superseded
    /// installation identity is disqualified with everything machine-keyed,
    /// so a recycled machine reads as never checked (`Dependency Upload &
    /// Remap` V10).
    fn the_check_standing(
        &self,
        the_root_files_machine_path: &str,
        the_machines_installation: &Identity,
        the_submissions_blender_version: &str,
    ) -> WhatTheRecordSays;

    /// **S5a: the app-wide reporting item, one user-held item of the local
    /// record's settings, read once at composition and never afterwards.**
    /// `None` where the record holds none, which the composition treats as
    /// the field it must never leave absent — see
    /// [`crate::submission::the_reporting_field`].
    fn the_app_wide_reporting_item(
        &self,
    ) -> Option<smoozy_render_settings::reporting::ReportingChoice>;

    /// **A9: the figure `Queue & History` N6 kept for the entry**, read at
    /// the draw and never durable here.
    fn the_figure_the_entry_kept(&self, job: &Identity) -> Option<i64>;

    /// **A4: the walk of the job output folder, taken when the action is
    /// taken.** `Job files and outputs` C7's one walk of that folder and
    /// everything under it, which C4 of that document grants the desktop in
    /// as many words — *"the desktop may perform the same reckoning while it
    /// is connected, and what it produces is a reading it shows and never
    /// acts on"*.
    ///
    /// **A walk that did not complete is
    /// [`WhatTheWalkOfTheFolderIs::ItDidNotComplete`] and carries no
    /// entries**, so nothing downstream can read one as a folder found empty
    /// (A8; `Job files and outputs` C9).
    fn walk_the_job_output_folder(
        &self,
        job: &Identity,
        the_job_output_folder: &str,
    ) -> WhatTheWalkOfTheFolderIs;

    /// **A4: the machine's own latest attempt record that carries a frame
    /// list, and the inventory for the map that record holds in force.**
    ///
    /// **Where the desktop gets the map is stated by A4 rather than left to
    /// be inferred**: it is that record, found by `Job files and outputs`
    /// C18a's rule — the same record P1 step 6 reads the list from. `None`
    /// where no record on the machine carries a list, or where the read did
    /// not complete, which A4 sends to A9's could-not-check case rather than
    /// to a figure composed against a guessed inventory.
    fn the_map_in_force_on_the_machine(&self, job: &Identity) -> Option<TheMapInForceRead>;
}

/// **What A4's read of the machine's own attempt record hands back.**
///
/// **The inventory is `Job files and outputs`' own composition and never
/// this crate's** — it is the inventory for the map that record carries, and
/// nothing here composes a second one.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TheMapInForceRead {
    /// That record's frame list, in the order it was written (`Job files and
    /// outputs` C18, C19).
    pub the_frame_list_asked_for: Vec<i64>,
    /// The inventory for the map in force on that record (`Job files and
    /// outputs` C7).
    pub the_inventory: WhatTheInventoryIs,
}

/// **A4 and A9, performed on the desk: the figure the carry-on page opens
/// on.**
///
/// **Nothing in this crate took either reading until 2026-09-05**, so the
/// only figure the page could open on was the one `Queue & History` kept —
/// under a sentence saying the machine could not be checked. A user whose
/// server went down mid-render, reconnected, and opened an entry whose
/// machine was there and whose folder was readable was told Smoozy could not
/// check it, and the range opened at a figure the entry had remembered from
/// the job's own ending. **A4 gives the exact harm that was written
/// against**: a machine that went down can leave a frame empty that Blender
/// reported saved seconds earlier, so a remembered figure can start the
/// range after a hole nobody will ever fill.
///
/// **A9's route stays and is no longer the whole of the behaviour.**
pub fn the_carry_ons_figure<R>(
    readings: &R,
    job: &Identity,
    the_job_output_folder: &str,
) -> TheCarryOnsFigure
where
    R: Readings + ?Sized,
{
    let walk = readings.walk_the_job_output_folder(job, the_job_output_folder);
    let Some(map) = readings.the_map_in_force_on_the_machine(job) else {
        return TheCarryOnsFigure::TheMachineCouldNotBeChecked {
            why: TheMachineCouldNotBeChecked::NoAttemptRecordOnTheMachineCarriesAFrameList,
            case: the_could_not_check_case(readings.the_figure_the_entry_kept(job)),
        };
    };
    the_carry_ons_figure_over(
        WhatTheCarryOnRead {
            the_walk: &walk,
            the_job_output_folder,
            the_inventory_for_the_map_in_force: &map.the_inventory,
            the_frame_list_asked_for: &map.the_frame_list_asked_for,
        },
        readings.the_figure_the_entry_kept(job),
    )
}

// ---------------------------------------------------------------------------
// A7, A7a and A10 — the reading of a machine over the command channel
// ---------------------------------------------------------------------------

/// **The carriage the reading names.** The path is machine-origin, so it
/// rides a link and the command names the token.
pub const THE_PATH_SITE: CarriageSite = CarriageSite {
    link_path_on_the_subsystem: ".smoozy-carriage/rendering-path",
    named_in_a_command_as: "~/.smoozy-carriage/rendering-path",
};

/// **A7's stat of the path itself.**
///
/// `-L` follows the link, because what the reading is about is the path the
/// token names and not the token. **Nothing is printed and nothing is
/// parsed**: what the reading takes is the exit status, which is the whole of
/// what separates the two answers.
pub const STAT_THE_PATH: CommandString =
    a_command_naming("stat -L -c '' ~/.smoozy-carriage/rendering-path");

/// **A7's listing half**, which is what separates *there* from *there and
/// this login may not read it* once the stat has exited 0.
///
/// It is the file-transfer route's, not a command's: `File Browser` P1 tells
/// a refusal apart from an absence by distinct protocol statuses in one
/// request, and that is the reading taken here rather than a second one.
pub const THE_LISTING_IS_THE_FILE_TRANSFER_ROUTES_AND_NOT_A_COMMANDS: bool = true;

/// **A7a's walk of the parent components.**
///
/// Taken only where the stat itself exited 1, which is *gone, or refused
/// higher up*. **A parent that does not answer *true* is a refusal higher up;
/// every parent answering as a directory, with the stat still at exit 1, is
/// the path itself being absent** — both by exit status alone, in the shape
/// `File Browser` P3 walks.
///
/// **What the walk cannot separate, stated rather than smoothed** — A7a's own
/// residue: the 2026-08-21 run never put this test on the unreadable parent
/// itself, so a path whose *immediate* parent is the unreadable one has no
/// parent below it to answer, falls to the absence, and is reported gone. The
/// cost is Row 294's route offered where the file may be there.
pub const WALK_THE_PARENT: CommandString =
    a_command_naming("test -d ~/.smoozy-carriage/rendering-path");

/// The reading, performed.
///
/// **A reading that did not answer is A7's *the machine did not answer* and
/// never an absence** — the connection or supervisor condition, raised by the
/// request not being answered and never inferred.
pub async fn read_one_path<C, S>(
    channel: &mut C,
    subsystem: &mut Subsystem<S>,
    path: &[u8],
    parents_nearest_first: &[Vec<u8>],
    the_listing_was_refused: bool,
) -> WhatTheReadingGave
where
    C: CommandChannel,
    S: AsyncRead + AsyncWrite + Unpin + Send,
{
    let Some(exit_status) = the_stat_of(channel, subsystem, path).await else {
        return WhatTheReadingGave::TheRequestWasNotAnswered;
    };
    if exit_status == 0 {
        return WhatTheReadingGave::TheStat {
            exit_status,
            the_listing_was_refused,
            the_walk: None,
        };
    }
    // **A7a: the walk only where the stat was itself refused**, which is the
    // cheaper reading A7b's correction established — one stat for the case
    // A7 and A10 actually read, the walk only here.
    //
    // **Both answers were the wrong way round until 2026-09-05**, and both
    // against this project's own recorded evidence: a parent answering *false*
    // was read as an absence and a clean walk was filled in as a refusal, so
    // the observed refusal case was reported gone and the observed gone case
    // was reported refused. A7a gained its third answer in the same round and
    // this is that clause.
    let mut answers: Vec<Option<i32>> = Vec::with_capacity(parents_nearest_first.len());
    for parent in parents_nearest_first {
        let answer = the_directory_test_of(channel, subsystem, parent).await;
        // **Asking further is only ever a cost**: an answer that is not a
        // reached directory has already settled the walk, so the remaining
        // parents are not asked. The reading itself is
        // [`the_walks_answer`]'s and is not repeated here.
        let nothing_further_to_ask = !matches!(answer, Some(0));
        answers.push(answer);
        if nothing_further_to_ask {
            break;
        }
    }
    WhatTheReadingGave::TheStat {
        exit_status,
        the_listing_was_refused,
        the_walk: the_walks_answer(&answers, parents_nearest_first.len()),
    }
}

/// **A7a's reading of what the walk found, over the answers it collected.**
///
/// The answers are the parents' own directory tests, nearest first, and there
/// may be fewer of them than there are parents because the walk stops as soon
/// as one settles it. `parents_in_all` is how many there were.
///
/// **The two answers were the wrong way round until 2026-09-05**, and both
/// against this project's own recorded evidence. A parent answering something
/// other than *true* was read as *this parent is not a directory* — an
/// absence — where the 2026-08-21 macOS run records the same test exiting 1 on
/// a path merely **under** an unreadable parent; and a walk that found every
/// parent a directory was filled in as a refusal, where A7's own evidence says
/// a stat at exit 1 over a clean walk is the path being gone. So the observed
/// refusal was reported gone and the observed gone was reported refused, and
/// the second of those is the one with no way forward: `what_a_gone_blend_leads_to`
/// answers nothing on a refusal, so Row 294's file browser never opens and a
/// user who moved their own `.blend` is told Smoozy may not look at it.
pub fn the_walks_answer(
    answers_nearest_first: &[Option<i32>],
    parents_in_all: usize,
) -> Option<TheWalk> {
    for answer in answers_nearest_first {
        match answer {
            // A parent the test reached and called a directory: nothing above
            // it stopped the test, so the walk goes on past it.
            Some(0) => continue,
            // **Evidence about what sits above this parent, and never about
            // the parent**: a refusal higher up.
            Some(_) => return Some(TheWalk::AParentDidNotAnswerSoSomethingAboveItRefuses),
            // The request was not answered at all. Nothing is concluded from
            // it and the walk ends with no value, which A7a's own *gone, or
            // refused higher up* is the honest reading of.
            None => return None,
        }
    }
    // **Every parent answered as a directory and the stat still exited 1:
    // the path itself is absent.** A walk over no parent at all settles
    // nothing and is not this case.
    (parents_in_all > 0 && answers_nearest_first.len() == parents_in_all)
        .then_some(TheWalk::EveryParentAnsweredAndThePathItselfIsAbsent)
}

async fn the_stat_of<C, S>(
    channel: &mut C,
    subsystem: &mut Subsystem<S>,
    path: &[u8],
) -> Option<i32>
where
    C: CommandChannel,
    S: AsyncRead + AsyncWrite + Unpin + Send,
{
    the_status_of(channel, subsystem, path, STAT_THE_PATH).await
}

async fn the_directory_test_of<C, S>(
    channel: &mut C,
    subsystem: &mut Subsystem<S>,
    path: &[u8],
) -> Option<i32>
where
    C: CommandChannel,
    S: AsyncRead + AsyncWrite + Unpin + Send,
{
    the_status_of(channel, subsystem, path, WALK_THE_PARENT).await
}

/// One command's exit status, with the path riding the carriage.
///
/// **The link is planted, the command names the token, and the link is
/// removed afterwards** — the same three acts every other reading in this
/// workspace takes, and the reason none of them writes a path into a command.
async fn the_status_of<C, S>(
    channel: &mut C,
    subsystem: &mut Subsystem<S>,
    path: &[u8],
    command: CommandString,
) -> Option<i32>
where
    C: CommandChannel,
    S: AsyncRead + AsyncWrite + Unpin + Send,
{
    if carriage::plant(subsystem, THE_PATH_SITE, path)
        .await
        .is_err()
    {
        return None;
    }
    let ran = channel.run(command).await;
    let _ = subsystem
        .remove(THE_PATH_SITE.link_path_on_the_subsystem.as_bytes())
        .await;
    // **A status that is absent is a channel that closed without one**, which
    // is a real case rather than a zero to be assumed — and here it is the
    // request not being answered.
    ran.ok()?.status.map(|status| status as i32)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::again::{separate, ThreeWay};
    use smoozy_job_files::inventory::{AnEntry, TheInventory, WhichRouteProducedTheName};
    use smoozy_job_files::reckoning::{AWalkEntry, WhatKindOfEntry};
    use smoozy_render_settings::reporting::ReportingChoice;

    const THE_FOLDER: &str = "/ephemeral/smoozy/jobs/j1";

    /// A desk whose answers the test chooses, so A4's arithmetic is read off
    /// rather than argued about.
    struct ADeskThat {
        walk: WhatTheWalkOfTheFolderIs,
        map: Option<TheMapInForceRead>,
        kept: Option<i64>,
    }

    impl Readings for ADeskThat {
        fn the_check_standing(&self, _: &str, _: &Identity, _: &str) -> WhatTheRecordSays {
            WhatTheRecordSays::NobodyHasEverChecked
        }
        fn the_app_wide_reporting_item(&self) -> Option<ReportingChoice> {
            None
        }
        fn the_figure_the_entry_kept(&self, _: &Identity) -> Option<i64> {
            self.kept
        }
        fn walk_the_job_output_folder(&self, _: &Identity, _: &str) -> WhatTheWalkOfTheFolderIs {
            self.walk.clone()
        }
        fn the_map_in_force_on_the_machine(&self, _: &Identity) -> Option<TheMapInForceRead> {
            self.map.clone()
        }
    }

    fn an_inventory(frames: std::ops::RangeInclusive<i64>) -> WhatTheInventoryIs {
        WhatTheInventoryIs::It(TheInventory {
            entries: frames
                .map(|frame| AnEntry {
                    predicted_path: format!("{THE_FOLDER}/f_{frame:04}.png").into_bytes(),
                    frame: Some(frame),
                    the_route_that_produced_the_name:
                        WhichRouteProducedTheName::ItWasBuiltFromTheVersionsRecipe,
                    the_destinations_aiming_at_it: vec![0],
                    may_grow: false,
                })
                .collect(),
            added_write_place_folders: Vec::new(),
            collision_record: Vec::new(),
            a_destination_is_video_shaped: false,
            why_it_owes_nothing: None,
        })
    }

    fn a_file(frame: i64, size: u64) -> AWalkEntry {
        AWalkEntry {
            path_under_the_job_output_folder: format!("f_{frame:04}.png").into_bytes(),
            kind: WhatKindOfEntry::AFile,
            size,
        }
    }

    /// **A4, on the ground A4 itself cites: a provider-console reboot left
    /// eleven whole frames and six further files at zero bytes**, so anything
    /// counting files would have been six frames wrong.
    ///
    /// **The user's moment this is the whole of.** Their server goes down
    /// mid-render. They reconnect, open the entry and ask to carry on. The
    /// machine is there and its folder is readable. **Until 2026-09-05 this
    /// crate walked nothing and read no attempt record**, so the page could
    /// only ever open on the figure the entry had kept from the job's own
    /// ending, under a sentence saying the machine could not be checked — and
    /// a remembered figure can start the range after a hole nobody will ever
    /// fill.
    #[test]
    fn the_figure_is_reckoned_off_the_machines_own_walk_and_never_the_remembered_one() {
        let job = Identity::issue(smoozy_state::identities::IdentityKind::Job).unwrap();
        let mut found: Vec<AWalkEntry> = (1..=11).map(|frame| a_file(frame, 4096)).collect();
        // **`Job files and outputs` C13: the six placeholders Blender writes
        // ahead of itself, at zero bytes, are frames never written.**
        found.extend((12..=17).map(|frame| a_file(frame, 0)));
        let desk = ADeskThat {
            walk: WhatTheWalkOfTheFolderIs::ItCompleted(found),
            map: Some(TheMapInForceRead {
                the_frame_list_asked_for: (1..=17).collect(),
                the_inventory: an_inventory(1..=17),
            }),
            // **The entry remembered seventeen**, which is what the folder
            // said and what the truth was not.
            kept: Some(17),
        };
        assert_eq!(
            the_carry_ons_figure(&desk, &job, THE_FOLDER),
            TheCarryOnsFigure::ReadFromTheMachine(Some(11)),
            "eleven whole frames and six at zero bytes: the boundary is eleven"
        );
    }

    /// **`Job files and outputs` C19 and C18: the order is the record's and
    /// never the numbers'** — the highest-numbered frame present need not be
    /// the last one finished, so the walk backwards is through the list
    /// Smoozy asked for.
    #[test]
    fn the_boundary_is_the_last_frame_asked_for_and_not_the_highest_on_disk() {
        let job = Identity::issue(smoozy_state::identities::IdentityKind::Job).unwrap();
        let desk = ADeskThat {
            walk: WhatTheWalkOfTheFolderIs::ItCompleted(vec![a_file(40, 4096), a_file(22, 4096)]),
            map: Some(TheMapInForceRead {
                // The typed list's own order: 40 was asked for before 22.
                the_frame_list_asked_for: vec![40, 22, 23],
                the_inventory: an_inventory(22..=40),
            }),
            kept: None,
        };
        assert_eq!(
            the_carry_ons_figure(&desk, &job, THE_FOLDER),
            TheCarryOnsFigure::ReadFromTheMachine(Some(22))
        );
    }

    /// **A8 and A9: a reading that did not complete is not a reading, and
    /// the could-not-check case is what takes it** — with the kept figure,
    /// saying plainly that the machine could not be checked and never
    /// presenting it as checked.
    ///
    /// **The route stays and stops being the only one**, which is the whole
    /// of the repair.
    #[test]
    fn a_reading_that_did_not_complete_falls_to_the_could_not_check_case_and_says_so() {
        let job = Identity::issue(smoozy_state::identities::IdentityKind::Job).unwrap();
        let map = || {
            Some(TheMapInForceRead {
                the_frame_list_asked_for: vec![1, 2, 3],
                the_inventory: an_inventory(1..=3),
            })
        };
        let complete = || WhatTheWalkOfTheFolderIs::ItCompleted(vec![a_file(1, 4096)]);

        // The walk did not complete.
        let refused = the_carry_ons_figure(
            &ADeskThat {
                walk: WhatTheWalkOfTheFolderIs::ItDidNotComplete,
                map: map(),
                kept: Some(9),
            },
            &job,
            THE_FOLDER,
        );
        let TheCarryOnsFigure::TheMachineCouldNotBeChecked { why, case } = refused else {
            panic!("{refused:?}");
        };
        assert_eq!(
            why,
            TheMachineCouldNotBeChecked::TheWalkOfTheJobOutputFolderDidNotComplete
        );
        assert_eq!(case.the_kept_figure, Some(9));
        assert!(case.says_it_could_not_check_the_machine);
        assert!(!case.presented_as_checked);

        // No attempt record on the machine carries a frame list.
        let none = the_carry_ons_figure(
            &ADeskThat {
                walk: complete(),
                map: None,
                kept: Some(9),
            },
            &job,
            THE_FOLDER,
        );
        assert!(matches!(
            none,
            TheCarryOnsFigure::TheMachineCouldNotBeChecked {
                why: TheMachineCouldNotBeChecked::NoAttemptRecordOnTheMachineCarriesAFrameList,
                ..
            }
        ));

        // **The names could not be predicted: no figure is composed against a
        // guessed inventory**, and never *the job finished no frame*.
        let guessed = the_carry_ons_figure(
            &ADeskThat {
                walk: complete(),
                map: Some(TheMapInForceRead {
                    the_frame_list_asked_for: vec![1, 2, 3],
                    the_inventory: WhatTheInventoryIs::TheFrameSetIsNotPredictable(
                        smoozy_job_files::inventory::WhyTheFrameSetIsNotPredictable::TheRegularOutputPathsNameWasNotAnswered,
                    ),
                }),
                kept: Some(9),
            },
            &job,
            THE_FOLDER,
        );
        assert!(matches!(
            guessed,
            TheCarryOnsFigure::TheMachineCouldNotBeChecked {
                why: TheMachineCouldNotBeChecked::TheInventoryForTheMapInForceCouldNotBeComposed,
                ..
            }
        ));

        // **A completed walk that found nothing is a reading**: it says the
        // job finished no frame, which A6 tells apart from not being able to
        // check at all.
        let empty = the_carry_ons_figure(
            &ADeskThat {
                walk: WhatTheWalkOfTheFolderIs::ItCompleted(Vec::new()),
                map: map(),
                kept: Some(9),
            },
            &job,
            THE_FOLDER,
        );
        assert_eq!(empty, TheCarryOnsFigure::ReadFromTheMachine(None));
    }

    /// **The two commands name the token and carry no path**, which is
    /// `Server protocol and transfers` §1.1 kept by the type: a
    /// [`CommandString`] is a compile-time constant and there is no route
    /// from a path to one.
    #[test]
    fn no_command_here_carries_a_path_and_both_name_the_carriages_token() {
        for command in [STAT_THE_PATH, WALK_THE_PARENT] {
            let text = format!("{command:?}");
            assert!(text.contains(THE_PATH_SITE.named_in_a_command_as), "{text}");
        }
        const _: () = assert!(THE_LISTING_IS_THE_FILE_TRANSFER_ROUTES_AND_NOT_A_COMMANDS);
    }

    /// **The stat separates the two answers by exit status and the directory
    /// test does not** — which is why the stat is what is run on the path
    /// itself and the directory test only on a parent.
    #[test]
    fn the_stat_is_what_reads_the_path_itself_and_the_directory_test_only_a_parent() {
        assert!(format!("{STAT_THE_PATH:?}").contains("stat"));
        assert!(format!("{WALK_THE_PARENT:?}").contains("test -d"));
    }

    /// **A7a's three answers, each read the way this project's own evidence
    /// records it** — and the user's moment behind the second.
    ///
    /// A user finishes a job, the machine is still there, and they move the
    /// `.blend` into another folder themselves. The stat exits 1 and every
    /// parent is an ordinary readable directory. **Read as an absence, Row
    /// 294 opens the file browser on that machine and the user points at the
    /// file. Read as a refusal — which is what this said until 2026-09-05 —
    /// Smoozy tells them it may not look at a file they moved with their own
    /// hands, and nothing offers a way on.**
    #[test]
    fn the_walk_reads_a_clean_chain_as_the_path_being_gone_and_an_unanswered_parent_as_a_refusal() {
        use crate::again::what_a_gone_blend_leads_to;

        // Every parent an ordinary readable directory: the path itself.
        let clean = the_walks_answer(&[Some(0), Some(0), Some(0)], 3);
        assert_eq!(
            clean,
            Some(TheWalk::EveryParentAnsweredAndThePathItselfIsAbsent)
        );
        let moved = WhatTheReadingGave::TheStat {
            exit_status: 1,
            the_listing_was_refused: false,
            the_walk: clean,
        };
        assert_eq!(separate(&moved), ThreeWay::TheFolderIsGone);
        assert_eq!(
            what_a_gone_blend_leads_to(&moved),
            Some(crate::again::WhereTheBrowserOpens::TheMachineTheJobRanOn),
            "Row 294's browser is what a gone `.blend` leads to"
        );

        // The nearest parent does not answer *true*: something above it
        // refuses, whatever exit status it gave.
        for exit in [1, 2] {
            assert_eq!(
                the_walks_answer(&[Some(exit)], 4),
                Some(TheWalk::AParentDidNotAnswerSoSomethingAboveItRefuses)
            );
        }
        assert_eq!(
            the_walks_answer(&[Some(0), Some(1)], 4),
            Some(TheWalk::AParentDidNotAnswerSoSomethingAboveItRefuses)
        );

        // A request that was not answered concludes nothing, and a walk with
        // no parent to take settles nothing either.
        assert_eq!(the_walks_answer(&[Some(0), None], 4), None);
        assert_eq!(the_walks_answer(&[], 0), None);
        // A walk cut short with every answer a directory is not the clean
        // walk: parents are left unasked.
        assert_eq!(the_walks_answer(&[Some(0)], 4), None);
    }

    /// **The three-way distinction over the readings this module produces**,
    /// each said as itself.
    #[test]
    fn each_reading_this_module_can_produce_separates_into_one_of_the_three() {
        assert_eq!(
            separate(&WhatTheReadingGave::TheRequestWasNotAnswered),
            ThreeWay::TheMachineDidNotAnswer
        );
        assert_eq!(
            separate(&WhatTheReadingGave::TheStat {
                exit_status: 0,
                the_listing_was_refused: false,
                the_walk: None
            }),
            ThreeWay::ItIsThere
        );
        assert_eq!(
            separate(&WhatTheReadingGave::TheStat {
                exit_status: 0,
                the_listing_was_refused: true,
                the_walk: None
            }),
            ThreeWay::ThisLoginMayNotReadIt
        );
        assert_eq!(
            separate(&WhatTheReadingGave::TheStat {
                exit_status: 1,
                the_listing_was_refused: false,
                the_walk: Some(TheWalk::EveryParentAnsweredAndThePathItselfIsAbsent)
            }),
            ThreeWay::TheFolderIsGone
        );
        assert_eq!(
            separate(&WhatTheReadingGave::TheStat {
                exit_status: 1,
                the_listing_was_refused: false,
                the_walk: Some(TheWalk::AParentDidNotAnswerSoSomethingAboveItRefuses)
            }),
            ThreeWay::ThisLoginMayNotReadIt
        );
        // **A stat that exited 1 with no walk taken says *gone, or refused
        // higher up* and names neither** — which is the honest answer and
        // never an absence.
        assert_eq!(
            separate(&WhatTheReadingGave::TheStat {
                exit_status: 1,
                the_listing_was_refused: false,
                the_walk: None
            }),
            ThreeWay::GoneOrRefusedHigherUpAndTheWalkNamesWhich
        );
    }
}
