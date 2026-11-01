//! The local job record: what the read fills in, and every field Smoozy's
//! own record keeps — `Queue & History` §10 N10–N22, with E12a's ask, N4's
//! one duration and N9a's statement.
//!
//! **The machine's per-job note carries its half here as fields of this
//! record, and not as a record kind of its own.** Ruled by the build runner on
//! 2026-09-17, before any code, at `Audits/Build/Step 10b/Ruling — where the
//! machine's per-job note lands on the desktop.md`. The short of it: the
//! inventory at `State, identity and recovery` J1 has settled this same
//! question twice already and both times the same way — `Dependency Upload &
//! Remap`'s per-job resolved set is *"a field of the job's own record rather
//! than a sixth kind"*, and N11 and N12 put the server's label, its hue and
//! the root `.blend` path on this record as copy-at-submission fields —
//! while `State, identity and recovery` J5 leaves no second store to put one
//! in and N2 is the argument that one fact must not have two disagreeing
//! homes. [`WhatTheNoteCarried`] is that half and
//! [`LocalJobRecord::the_fill_in_from_the_note`] is the one thing that writes
//! it.
//!
//! **Constraint:** Rows 76, 83, 87 and the Design rows 319, 320, 379–381,
//! 433, 448–450, 494. `State, identity and recovery` §1.1: the local record
//! is the only durable store that holds state on the user's side, and this
//! record is a row of it. Its D4: no durable copy of an observed machine
//! state the machine still holds — so every field below is either Smoozy's
//! own note written at submission, or what §7 step 4's fill-in carried from
//! a record the machine wrote, and nothing is read back from a server row.
//!
//! **What is here**: the record's shape, the merge rule, and every draw
//! composed from it — each returning a value and never wording, which is
//! Design's. **What is the acting half's**: the store, §7's reconciliation
//! that fills it in, and the note (N1–N9) on the machine.
//!
//! **Markers used in this file that belong to another document, each with the
//! document it belongs to.** `Render Settings` Y1a, Y15. `Servers` W31.
//! `Rendering` S6, A1a. `Download` H5a, H7a. `File Browser` W8. `Server
//! Setup` B13. `Job files and outputs` M9.
//!
//! **`Rendering` L15, W1, W2, A1 and A9, `Download` H5 to H9, `File Browser`
//! W5 and `File Browser` W9, `Job files and outputs` R13 and `Render Settings` F5 are not
//! listed above and are written out at each site**, because this plan
//! defines a clause of each name and a bare one could mean either.

use std::time::{Duration, SystemTime};

use smoozy_render_settings::device::DeviceChoice;
use smoozy_render_settings::payload::{Companion, VersionPick};
use smoozy_state::identities::{Identity, IdentityKind};
use smoozy_state::observed::{Clock, Reading};

// **The note's own types, taken rather than copied.** N2's whole point is that
// one fact has one home; a second spelling of an ending or of a re-run mark
// here would be the two-homes shape this record exists to avoid.
use crate::acting::note::{AnAttemptMark, TheEndingOnTheNote, TheFreeSpaceReadAtTheEnding, TheNote};
use crate::ending::{the_ending, Ending};
use crate::readings::{LandingPlaces, Readings};

/// **N12: the server's label and badge hue, as they stood at submission.**
/// Copies, never read back from the server row — the row's colour ends at
/// the removal, and history records are keyed by the job identity. A rename
/// therefore does not reach history. **N13: absent on a record written
/// before the rule, drawn as absent and never as a default colour.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TheServerAsItStood {
    /// The local server identifier the job was submitted under — what
    /// `State, identity and recovery` §7 step 2 asks the machine about and
    /// what `Servers` W31's edit warning counts by. **Once the row is gone
    /// it dangles, and a dangling identifier reads as nothing.**
    pub local_server: Identity,
    pub label: Option<String>,
    pub hue: Option<Hue>,
}

/// The badge hue, as the server row carried it at submission. A value the
/// row's own store defines; held here as the number it was.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Hue(pub u16);

/// **N10a: each answer the record carries says whether it was the user's own
/// or resolved for them** — written at submission from which side supplied
/// the value, and no later reading changes it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AnswerOrigin {
    TheUsersOwnOverride,
    ResolvedForThem,
}

/// **N19: which Blender the entry keeps is a pair, not a version** — the
/// picked build's path and its answered version together (`Server Setup`
/// B13), with N10a's origin beside it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TheBlenderTheEntryKeeps {
    pub pick: VersionPick,
    pub origin: AnswerOrigin,
}

/// **E12a: what the job was asked to render on**, composed at submission
/// from the user's own answer or the file's own value as the reading in
/// force gave it — and, where no reading stood and no answer was given, the
/// file's own, unread. **No reading fills it and no reading overwrites it.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TheDeviceAsk {
    TheUsersOwn(DeviceChoice),
    TheFilesOwnAsTheReadingInForceGaveIt(DeviceChoice),
    /// Submitted with no reading standing and no answer: the file's own,
    /// whatever the file says when the job starts.
    TheFilesOwnUnread,
}

impl TheDeviceAsk {
    /// **The device choice the ask resolved to, where one is known at
    /// submission** — nothing for the file's own unread. `Server Setup` R1c
    /// reads it: a job whose device is not known at submission is not held.
    /// Added 2026-10-12, held-job piece A.
    pub fn the_choice(self) -> Option<DeviceChoice> {
        match self {
            TheDeviceAsk::TheUsersOwn(choice) | TheDeviceAsk::TheFilesOwnAsTheReadingInForceGaveIt(choice) => {
                Some(choice)
            }
            TheDeviceAsk::TheFilesOwnUnread => None,
        }
    }
}

/// **N15's first mark: the record state the missing-assets evaluation found
/// at submission** — `Rendering` W2's three, or that a complete standing
/// stood. One field wide enough for the state, never a boolean: a single
/// bit worded as *nothing checked this file* would assert the one thing
/// `Rendering` W2 forbids against exactly the job whose file was checked and
/// then replaced. A copy, never the standing itself, which is machine-keyed
/// and disqualified with a recycled machine.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DependencyCheckAtSubmission {
    NoStandingAtAll,
    AStandingCarryingTheReplacedSinceCheckOrDisturbedMark,
    AStandingKeyedToAnotherBlenderVersion,
    /// A complete standing satisfying `Rendering` W1 stood: no mark is drawn.
    ACompleteStandingStood,
}

/// **N15's second mark: it carries on from an earlier job** — the earlier
/// job's identity and its label as it stood, copied because the link can
/// outlive its target.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CarriesOnFrom {
    pub earlier_job: Identity,
    pub the_label_as_it_stood: String,
}

/// **N21: the disturbed-file mark, per disturbance, appended and never
/// replaced.** The disturbed member as the record spells it (`File Browser`
/// W5's matched path), the act intended, and the composed reading (`File
/// Browser` W9).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ADisturbance {
    pub the_member: String,
    pub the_act_intended: String,
    pub reading: DisturbanceReading,
    /// **How many files one delete took from the folder [`Self::the_member`]
    /// names**, where it took more than one — `Design/Queue & History § The
    /// marks`, the build runner's ruling (d) of 2026-10-03: a delete inside a
    /// job's folder leaves one mark per delete act, naming the file where one
    /// went and the count with the folder where more did. **Nothing** for
    /// every other mark, whose member is the one thing it names.
    pub how_many: Option<u32>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DisturbanceReading {
    Disturbed,
    /// The act's outcome was never learned.
    MayHaveBeenDisturbed,
}

/// **What only a read can know, landed by §7 step 4's fill-in from the
/// recorded reading (N9)** — and it is the machine's own record, `Job files
/// and outputs`', carried and never re-derived.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct WhatTheReadProduced {
    pub scene_name: Option<String>,
    pub picture_size: Option<(u32, u32)>,
    /// The main output, or the job's own folder where the walk declared
    /// none; what *show outputs* opens (N10, N18).
    pub main_output_or_the_jobs_folder: Option<String>,
    /// **N9a: the attempts whose record states that the file changed** (`Job
    /// files and outputs` R13; `Job files and outputs` M9), read and never
    /// computed, per attempt — **each with what became of the frames that were
    /// already finished.**
    pub attempts_where_the_file_changed: Vec<AnAttemptWhereTheFileChanged>,
    /// **What the read tells of the job's frames** — the asked order, and
    /// whether the frames can be counted at all (the 38's piece 10).
    pub the_frames: TheFramesTheReadFound,
    /// **Where the render worker chose the job's folder at render start**
    /// (`Job files and outputs` P21's second case), with its case and whether
    /// the server's chosen fallback folder was passed over (`Job files and
    /// outputs` P23) — **N9's one more field**, read from the job record
    /// directory beside the reading. `None` where the machine recorded no
    /// choice: the manifest named the folder, or the render has not chosen.
    pub the_job_output_folder: Option<smoozy_job_files::where_the_folder_sits::WhereTheJobOutputFolderSits>,
}

/// **Where a job's output folder is, as the local record holds it** — Y8's
/// *"the folder name shown is the manifest's recorded path verbatim"*, widened
/// 2026-10-04 with `Job files and outputs` §6.6. The Creator's second notes,
/// piece 4.
#[derive(Debug, Clone, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize)]
pub enum TheJobOutputFolderTheRecordHolds {
    /// **A record written before 2026-10-04**: the job's folder is in the job
    /// area, composed from the server row's layout by `Job files and outputs`
    /// N3 exactly as every job's was then — which is the manifest's recorded
    /// path for such a job, so nothing about an older job moves.
    #[default]
    AsEveryJobBeforeItHadIt,
    /// **Chosen at render start and not learned yet** (`Job files and
    /// outputs` P21). Y8: *"until it arrives the job has no folder and show
    /// outputs is withheld"*.
    ChosenAtRenderStart,
    /// **Where it sits**, learned by N9 from the record the render worker
    /// wrote, with `Job files and outputs` P19's case and `Job files and
    /// outputs` P23's passed-over beside it.
    It(smoozy_job_files::where_the_folder_sits::WhereTheJobOutputFolderSits),
}

impl TheJobOutputFolderTheRecordHolds {
    /// **The form the local record keeps it in** — one JSON text, the form
    /// the machine writes the same choice in.
    pub fn as_written(&self) -> String {
        serde_json::to_string(self).expect("where a folder is is made of values that always render")
    }

    /// **Read back**, or nothing where the text is not this shape.
    pub fn read_back(written: &str) -> Option<TheJobOutputFolderTheRecordHolds> {
        serde_json::from_str(written).ok()
    }
}

/// **What the job record directory tells of a job's frames, kept on the record
/// so History can say how far the job got after the machine is gone** — the
/// 38's piece 10, items 24 and 38. Record version 35.
///
/// - **The frames the job asked for, in its own asked order** (`Queue &
///   History` N6's *"the job's own asked order"*), as runs. **Read off the
///   first attempt record that stands** (`Job files and outputs` M8's frame
///   list *"in the order it was written"*), which on a job's first rendering
///   attempt is every frame it asked for: the job's folder is its own and
///   empty, so the already-there test skips nothing. The build runner's ruling
///   recorded in the piece's account: the manifest's range reaches the desktop
///   only for a typed range, and a reading taken before submitting carries no
///   settings half to read the file's own range from.
/// - **Whether the frames cannot be counted** — `Job files and outputs` I9a's
///   two-view case, where the inventory is not composed; Design: *"the entry
///   and its History row show no frame count"*.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct TheFramesTheReadFound {
    pub asked: Option<TheFramesAsked>,
    pub they_cannot_be_counted: bool,
}

/// **The asked order as runs** — each a first frame, a step and how many — so a
/// range of a million frames is one run and never a million numbers.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct TheFramesAsked {
    pub runs: Vec<ARunOfFrames>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ARunOfFrames {
    pub first: i64,
    pub step: i64,
    pub how_many: u64,
}

impl TheFramesAsked {
    /// **The order, folded into runs**, and nothing for an empty order.
    pub fn from_the_order(order: &[i64]) -> Option<TheFramesAsked> {
        let mut runs: Vec<ARunOfFrames> = Vec::new();
        for frame in order {
            if let Some(last) = runs.last_mut() {
                let next = last.first + last.step * last.how_many as i64;
                if last.how_many == 1 && *frame != last.first {
                    last.step = frame - last.first;
                    last.how_many = 2;
                    continue;
                }
                if *frame == next {
                    last.how_many += 1;
                    continue;
                }
            }
            runs.push(ARunOfFrames {
                first: *frame,
                step: 1,
                how_many: 1,
            });
        }
        (!runs.is_empty()).then_some(TheFramesAsked { runs })
    }

    pub fn how_many(&self) -> u64 {
        self.runs.iter().map(|run| run.how_many).sum()
    }

    /// **The asked order again, frame by frame** — the runs unfolded, for
    /// `Rendering` A4's carried-on range, which opens at a frame of this order
    /// and takes every frame after it. The 38's piece 2.
    pub fn the_order(&self) -> Vec<i64> {
        self.runs
            .iter()
            .flat_map(|run| (0..run.how_many).map(move |at| run.first + run.step * at as i64))
            .collect()
    }

    /// **How many frames come before `frame` in the asked order**, or nothing
    /// where the job never asked for it.
    pub fn how_many_before(&self, frame: i64) -> Option<u64> {
        let mut before = 0u64;
        for run in &self.runs {
            let offset = frame - run.first;
            if run.step != 0 && offset % run.step == 0 {
                let at = offset / run.step;
                if at >= 0 && (at as u64) < run.how_many {
                    return Some(before + at as u64);
                }
            } else if run.step == 0 && frame == run.first {
                return Some(before);
            }
            before += run.how_many;
        }
        None
    }
}

/// **N9a, per attempt: the file changed, and what became of the frames that
/// were already finished.**
///
/// **Both facts are read and neither is computed here.** The first is `Job
/// files and outputs` R13's statement, written into that attempt's record; the
/// second is `Job files and outputs` C12a, written beside it. **Corrected 2026-09-04,
/// out of a code round.** This carried the attempt's number alone, and the
/// mark drawn from it said flatly that frames finished under the earlier
/// arrangement **were rendered again rather than reused** — which holds only
/// where the arrangement moved. A file edited in a way that moves no
/// destination yields the same arrangement, the already-there test finds the
/// earlier attempt's frames at exactly the paths it predicts, and **they are
/// reused**. The entry was telling the user something that had not happened,
/// on the common case.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AnAttemptWhereTheFileChanged {
    pub attempt: u32,
    pub what_became_of_the_frames_already_finished: WhatBecameOfTheFramesAlreadyFinished,
}

/// The two outcomes, and there is no third.
///
/// **No count either way** — `Job files and outputs` C12a is a plain yes or
/// no, because a number would want a walk of the earlier attempt's map that no
/// clause anywhere takes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhatBecameOfTheFramesAlreadyFinished {
    /// The arrangement did not move, so the frames finished before the change
    /// were found where the check looks and kept. **Nothing was paid for
    /// twice**, and the job's folder holds frames rendered from two versions of
    /// the file.
    TheyWereReused,
    /// The arrangement moved, so no frame finished under the earlier one was
    /// found and all of them were rendered again. **Frames already paid for are
    /// paid for a second time**, and the job's folder holds two arrangements of
    /// subfolders.
    TheyWereRenderedAgain,
}

impl AnAttemptWhereTheFileChanged {
    /// **Read off the attempt record and derived from nothing else.**
    ///
    /// Returns nothing where that record does not state that the file changed,
    /// because the mark exists only for an attempt that found it changed.
    pub fn read_from(
        record: &smoozy_job_files::attempts::AttemptRecord,
    ) -> Option<AnAttemptWhereTheFileChanged> {
        if !record.the_file_changed_before_this_attempt {
            return None;
        }
        Some(AnAttemptWhereTheFileChanged {
            attempt: record.number,
            what_became_of_the_frames_already_finished: if record
                .a_frame_an_earlier_attempt_wrote_was_kept
            {
                WhatBecameOfTheFramesAlreadyFinished::TheyWereReused
            } else {
                WhatBecameOfTheFramesAlreadyFinished::TheyWereRenderedAgain
            },
        })
    }
}

/// **The note's carried half, as fields of this record.**
///
/// **N1 says what the note holds and where**: one durable record per job, in
/// the package area, written by the supervisor. **This is the part of it the
/// desktop keeps once `State, identity and recovery` V1 step 4, at that
/// document's §7, has carried it over**, so that an entry still says what
/// happened after the machine is gone — which is what the whole of C11 rests
/// on.
///
/// **The label is not here and that is not an omission.** N1 puts the label on
/// the note too, and the record already carries its own, written at
/// submission; N10's merge rule gives Smoozy's own the win, so a second copy
/// of it here would be a fact with two homes and no rule for which is read.
///
/// **Absent is the ordinary state and never a zero.** A job that has not ended
/// carries no ending (N1's null ending), a job whose ending step never ran
/// carries no figure (N7), and a video job carries none at all (N8).
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct WhatTheNoteCarried {
    /// **The moment the render started, on the machine's clock** (N1, N4).
    pub started: Option<Reading<()>>,
    /// **The moment it ended, on the same clock** (N1, N4).
    pub ended: Option<Reading<()>>,
    /// **N1's ending, with its outcome, the exit status verbatim and N3's
    /// pointer into the attempt's capture record.** Null until an outcome was
    /// recorded, never invented.
    pub ending: Option<TheEndingOnTheNote>,
    /// **The attempt count** (N1).
    pub attempts: u32,
    /// **The re-run marks, and from which frame each re-run proceeded** (N1).
    pub re_run_marks: Vec<AnAttemptMark>,
    /// **Given up, where it was** (N1).
    pub given_up_where_it_was: Option<i64>,
    /// **N6's last-frame figure**, computed on the machine by the ending's own
    /// step and never re-derived here.
    pub last_frame_figure: Option<i64>,
    /// **`Rented-server runtime` V12's free-space reading, carried with the
    /// ending** — against the job's own output folder, on the machine's own
    /// clock, as of the act that first tried to write the outcome. **Absent
    /// where the ending carried none**, which is never a reading of nought
    /// (`Desktop app and delivery` M15a), and on every record before the
    /// reading travelled. Added 2026-10-25, step 12's SC4 findings.
    pub the_free_space_at_the_ending: Option<TheFreeSpaceReadAtTheEnding>,
}

/// **The two standings `State, identity and recovery` V1 step 2, at that
/// document's §7, gives a record, and there is
/// no third.**
///
/// A record is written **provisional** at submission and before the submission
/// is sent; it becomes **confirmed** on a connection where the machine answers
/// that it holds the job. **A machine that cannot say leaves it provisional**
/// and it is asked again next time — see
/// [`WhatAnAbsenceDoes::TheRecordStaysProvisionalAndIsAskedAgainNextTime`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Standing {
    Provisional,
    Confirmed,
}

/// **What a connection's ask about one provisional record came back with** —
/// the three branches of `State, identity and recovery` V1 step 2, at that
/// document's §7, and there is
/// no fourth.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhatTheMachineSaidAboutTheJob {
    /// **Present.**
    ItHoldsTheJob,
    /// **Absent, and the machine's operation record shows the submission was
    /// refused or never taken.**
    ItsOperationRecordShowsTheSubmissionWasRefusedOrNeverTaken,
    /// **Absent, and the machine cannot say.** A connection that dropped, a
    /// machine that would not answer, a read that failed — every one of them
    /// is this branch.
    TheMachineCannotSay,
}

/// **What each of those three does to the record**, and the third one is the
/// whole reason this is a value rather than three lines somewhere.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhatAnAbsenceDoes {
    TheRecordBecomesConfirmed,
    /// *"The local record is removed, leaving no trace, which is what
    /// `Preparation/Specs/Queue & History.md` requires of a job that never
    /// rendered."*
    TheRecordIsRemovedLeavingNoTrace,
    /// **No absence, of any length, removes a provisional record.**
    ///
    /// **`Preparation/Specs/Whole app.md` forbids the determination that would
    /// license one**: *"Smoozy cannot tell a network problem from a machine
    /// that was switched off from a machine that no longer exists — all three
    /// look the same to it. So it never claims to know which it is"* — so *the
    /// machine will not come back* is not a state this step may reach.
    ///
    /// **An agent tidying this up will reach for a timeout**, and a timeout is
    /// that determination in arithmetic. There is nothing to measure one
    /// against here on purpose: this branch takes no duration, no count and no
    /// moment, so a timeout cannot be added without changing the shape.
    TheRecordStaysProvisionalAndIsAskedAgainNextTime,
}

/// **`State, identity and recovery` V1 step 2, at that document's §7,
/// performed for one record.**
///
/// **It takes the machine's answer and nothing else** — no clock, no attempt
/// count, no age of the record and no reading of how long the machine has been
/// quiet. That is what keeps the third branch honest.
pub fn what_step_2_does(said: WhatTheMachineSaidAboutTheJob) -> WhatAnAbsenceDoes {
    match said {
        WhatTheMachineSaidAboutTheJob::ItHoldsTheJob => WhatAnAbsenceDoes::TheRecordBecomesConfirmed,
        WhatTheMachineSaidAboutTheJob::ItsOperationRecordShowsTheSubmissionWasRefusedOrNeverTaken => {
            WhatAnAbsenceDoes::TheRecordIsRemovedLeavingNoTrace
        }
        WhatTheMachineSaidAboutTheJob::TheMachineCannotSay => {
            WhatAnAbsenceDoes::TheRecordStaysProvisionalAndIsAskedAgainNextTime
        }
    }
}

/// **The local job record.** Written at submission with the provisional
/// record (`State, identity and recovery` §7 step 2), filled in by step 4,
/// and Smoozy's own note thereafter — the record history *is* (C10, C11).
#[derive(Debug, Clone, PartialEq)]
pub struct LocalJobRecord {
    pub job: Identity,
    pub group: Option<Identity>,
    pub server: TheServerAsItStood,
    /// **N16: the submitted moment, on the desktop's clock** — the one figure
    /// on the entry taken from it, drawn and subtracted against nothing.
    /// Absent on a record written before the rule.
    pub submitted_at: Option<Reading<()>>,
    /// The label, whole, as typed (Y8).
    pub label: Option<String>,
    /// **N11: the root `.blend` file's machine path as the submission named
    /// it** — copied because the manifest dies with the machine.
    pub blend_path_on_the_machine: Option<String>,
    /// **N11: no scene name where none was named** — and N22's mark below
    /// says so even after the read fills the name in.
    pub scene_name: Option<String>,
    /// **N22: the mark that the submission named no scene.** One presence
    /// field, holding no scene name of its own; no read removes it.
    pub submitted_with_no_scene_named: bool,
    /// **N11: a percentage only where the user typed one; no pixel size at
    /// all** until the read supplies one.
    pub resolution_percentage_typed: Option<u32>,
    pub picture_size_from_the_read: Option<(u32, u32)>,
    pub main_output_from_the_read: Option<String>,
    pub blender: TheBlenderTheEntryKeeps,
    pub device_asked_for: TheDeviceAsk,
    /// **N14: the file's value for each setting the payload carries an
    /// override for** (`Render Settings` Y1a), **and for the frame range, the
    /// samples, the percentage and the engine whether or not it does** (N14 as
    /// corrected 2026-10-07). Absent where no reading stood at submission.
    pub the_files_values_beside_the_overrides: Option<Companion>,
    pub dependency_check: Option<DependencyCheckAtSubmission>,
    pub carries_on_from: Option<CarriesOnFrom>,
    pub disturbances: Vec<ADisturbance>,
    /// **N9a, per attempt**, and Smoozy's own record once the read has landed:
    /// it rides the history record and survives the machine.
    pub attempts_where_the_file_changed: Vec<AnAttemptWhereTheFileChanged>,
    /// **What the read tells of the job's frames** — filled by
    /// [`LocalJobRecord::the_read_fills_what_it_lacks`] and by nothing else.
    /// Record version 35, the 38's piece 10.
    pub the_frames: TheFramesTheReadFound,
    /// **The standing `State, identity and recovery` V1 step 2, at that
    /// document's §7, gives it** — provisional
    /// from the submission, confirmed when the machine says it holds the job.
    pub standing: Standing,
    /// **C9: a job is in history from the moment it starts rendering, and the
    /// moment is the machine's, learned late.** This is the learning itself,
    /// kept because C9 presents the record in history *from* it and the local
    /// record is the only durable store on this side that holds state
    /// (`State, identity and recovery` J5).
    ///
    /// **It is not the same fact as the standing.** Confirmation is the machine
    /// having the job; this is the render having started — which is why a
    /// confirmed record can still be in no list at all.
    pub the_render_start_has_been_learned: bool,
    /// **The note's carried half** — the ruling of 2026-09-17, named at this
    /// file's head. Filled by [`LocalJobRecord::the_fill_in_from_the_note`] and
    /// by nothing else.
    pub what_the_note_carried: WhatTheNoteCarried,
    /// **Where the job's output folder is** — record version 39, the Creator's
    /// second notes, piece 4. Written at submission (*chosen at render
    /// start*, every job the desktop submits today) and filled in once by
    /// [`LocalJobRecord::the_read_fills_what_it_lacks`] from N9's field.
    pub the_job_output_folder: TheJobOutputFolderTheRecordHolds,
    /// **N23: a job held for the driver and released after a swap that did not
    /// bring the driver up to the bar carries a mark saying so** — beside N21's
    /// field, and of N21's shape: a copy on Smoozy's own record, written once
    /// by the desktop before that job's release is sent (`Server Setup` R1c),
    /// never filled or overwritten by a reading (N10), and kept into history
    /// until the entry is deleted (X1). **Nothing on every other job**, and on
    /// every record written before record version 42. Added 2026-10-12,
    /// held-job piece A; **the mark it draws is the history surface's**, which
    /// is held-job piece B's.
    pub the_driver_was_below_the_bar: Option<TheDriverWasBelowTheBar>,
}

/// **N23's field: the driver's version and the job's Blender version as they
/// stood when the release was decided.** Wording is Design's.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TheDriverWasBelowTheBar {
    /// **The driver's version as the driver wrote it**, from the last reading
    /// the swap's run took. **Nothing where the driver gave none** — it no
    /// longer answered, which is one of the ways `Server Setup` R1c releases
    /// after a run that did not bring the driver up.
    pub the_drivers_version: Option<String>,
    /// **The job's Blender, N19's answered version**, copied from this record.
    pub the_jobs_blender: String,
}

impl LocalJobRecord {
    /// **Where this job's output folder is, as the local record holds it** —
    /// Y8: *"the folder name shown is the manifest's recorded path verbatim"*,
    /// and never a re-rendering of it. A record written before 2026-10-04 is
    /// in the job area, and that is `Job files and outputs` N3's own
    /// composition from the layout the server row records — the path such a
    /// job's manifest names. **Nothing where the folder is chosen at render
    /// start and has not arrived** (`Job files and outputs` P21): the job has
    /// no folder yet, and nothing reaches into one. The Creator's second
    /// notes, piece 4.
    pub fn the_job_output_folder(&self, layout: &smoozy_job_files::layout::TheLayout) -> Option<String> {
        match &self.the_job_output_folder {
            TheJobOutputFolderTheRecordHolds::AsEveryJobBeforeItHadIt => {
                smoozy_job_files::publication::AJobsPaths::composed(layout, &self.job, self.label.as_deref())
                    .ok()
                    .map(|paths| paths.job_output_folder)
            }
            TheJobOutputFolderTheRecordHolds::ChosenAtRenderStart => None,
            TheJobOutputFolderTheRecordHolds::It(sits) => Some(sits.path.clone()),
        }
    }

    /// **N10: the merge rule — Smoozy's own record wins where it carries a
    /// fact, and the reading fills what it lacks.** A job submitted again
    /// from a history entry carries the old job's scene name as Smoozy's own
    /// record, never overwritten by a later reading's; the picture size is
    /// not carried and fills in from the read. **Every mark, every copy and
    /// the ask are untouched**, which the field-by-field shape of this
    /// function is: it writes six fields and no seventh (four until record
    /// version 35, when the frames joined them — the 38's piece 10; five until
    /// record version 39, when the job's folder chosen at render start joined
    /// them — the Creator's second notes, piece 4).
    pub fn the_read_fills_what_it_lacks(&mut self, read: &WhatTheReadProduced) {
        // **The folder chosen at render start is learned once** (`Job files
        // and outputs` P21: chosen once and never again), and only onto a
        // record that is waiting for it — never over a folder Smoozy's own
        // record already holds.
        if self.the_job_output_folder == TheJobOutputFolderTheRecordHolds::ChosenAtRenderStart {
            if let Some(sits) = &read.the_job_output_folder {
                self.the_job_output_folder = TheJobOutputFolderTheRecordHolds::It(sits.clone());
            }
        }
        // **The asked order is written once**: it is the first attempt's, and
        // no later read changes what the job asked for. **That the frames
        // cannot be counted is never taken back** by a later read.
        if self.the_frames.asked.is_none() {
            self.the_frames.asked = read.the_frames.asked.clone();
        }
        if read.the_frames.they_cannot_be_counted {
            self.the_frames.they_cannot_be_counted = true;
        }
        if self.scene_name.is_none() {
            self.scene_name = read.scene_name.clone();
        }
        if read.picture_size.is_some() {
            self.picture_size_from_the_read = read.picture_size;
        }
        if read.main_output_or_the_jobs_folder.is_some() {
            self.main_output_from_the_read = read.main_output_or_the_jobs_folder.clone();
        }
        for attempt in &read.attempts_where_the_file_changed {
            if !self
                .attempts_where_the_file_changed
                .iter()
                .any(|held| held.attempt == attempt.attempt)
            {
                self.attempts_where_the_file_changed.push(*attempt);
            }
        }
    }

    /// **`State, identity and recovery` V1 step 4's fill-in, at that
    /// document's §7, performed:
    /// what the local record does not hold, from the machine's own note.**
    ///
    /// **It writes the carried half and nothing else.** Every other field of
    /// this record is Smoozy's own note of Smoozy's own act, and N10's merge
    /// rule gives Smoozy's own the win — so there is no branch here that can
    /// reach one, which is the shape rather than a rule to be remembered.
    ///
    /// **N16 is the one it would be easiest to overwrite and it is never
    /// touched.** The submitted moment is the one figure on the entry taken
    /// from the desktop's clock, the machine was never told when the
    /// submission happened, and the note holds two moments of its own on a
    /// different clock. A fill-in that put the note's started moment into it
    /// would put a machine's clock in a field D5 says is this computer's, and
    /// the entry would show a submitted moment that is not one.
    ///
    /// **The caller checks the tombstone, not this.** `State, identity and
    /// recovery` H4 refuses this fill for the job identities a history
    /// tombstone names, unconditionally; the refusal is at the store, because
    /// a record a tombstone names has already been removed and there is
    /// nothing here to refuse.
    ///
    /// **C9's learning rides the note's started moment**, which is written when
    /// the render operation's attempt starts (N2) — so a note carrying one is
    /// the machine saying the render started, and the record enters history.
    /// **The learning is never taken away**: a note that later carries no
    /// started moment does not un-learn it.
    pub fn the_fill_in_from_the_note(&mut self, note: &TheNote) {
        if note.started.is_some() {
            self.what_the_note_carried.started = note.started.clone();
            self.the_render_start_has_been_learned = true;
        }
        if note.ended.is_some() {
            self.what_the_note_carried.ended = note.ended.clone();
        }
        if note.ending.is_some() {
            self.what_the_note_carried.ending = note.ending.clone();
        }
        if note.attempts > self.what_the_note_carried.attempts {
            self.what_the_note_carried.attempts = note.attempts;
        }
        if !note.re_run_marks.is_empty() {
            self.what_the_note_carried
                .re_run_marks
                .clone_from(&note.re_run_marks);
        }
        if note.given_up_where_it_was.is_some() {
            self.what_the_note_carried.given_up_where_it_was = note.given_up_where_it_was;
        }
        if note.last_frame_figure.is_some() {
            self.what_the_note_carried.last_frame_figure = note.last_frame_figure;
        }
        // **`Rented-server runtime` V12's reading is immutable with the
        // ending it rides** (`State, identity and recovery` §8.6): taken once,
        // carried over once, and never replaced by a later note or cleared by
        // one carrying none.
        if self.what_the_note_carried.the_free_space_at_the_ending.is_none() {
            self.what_the_note_carried.the_free_space_at_the_ending = note
                .the_free_space_at_the_ending
                .as_ref()
                .and_then(|free| free.the_reading())
                .cloned();
        }
    }

    /// **C11: the ending Smoozy learns**, read off the carried half and
    /// decided by E2's own mapping rather than by a second one here.
    pub fn the_ending_learned(&self) -> Option<Ending> {
        self.what_the_note_carried
            .ending
            .as_ref()
            .map(|ending| the_ending(ending.outcome.as_a_recorded_outcome()))
    }

    /// **How far the job got, as History's row says it: the frames the server
    /// held when the job ended, which is N6's figure and every frame before it
    /// in the asked order, over every frame asked** — `Design/Queue & History`:
    /// *the frame count is the frames the server held when the job ended*, the
    /// count Telegram's ending message says. A job stopped with frames 1–52 on
    /// the server reads *52/150*. The 38's piece 10, item 24; the live pass on
    /// Vast, 2026-10-19, fault 5.
    ///
    /// - **The figure is the last frame, in the asked order, that counts as
    ///   already there** (N6), so the count runs *up to and including* it —
    ///   its place in the asked order plus one, and never the figure itself,
    ///   which over a range starting at 1001 would read *1061/240*. *(Until
    ///   2026-10-19 this counted the frames before the figure, so History said
    ///   51/150 where the server and Telegram said 52.)*
    /// - **`Rendering` A4's re-render of the boundary frame is *Carry on*'s and
    ///   not this count's**: *Carry on* still opens at the figure and renders
    ///   it again, read off the figure and the asked order, never off this
    ///   pair.
    /// - **A clean finish is every frame asked.** An ending short of one with
    ///   no figure finished no frame (N6), and says *0*.
    /// - **Nothing** where no ending is learned, where the record holds no
    ///   asked order, where the frames cannot be counted (`Job files and
    ///   outputs` I9a), or where the figure is a frame the job never asked
    ///   for — absent is drawn as absent (N13), never a made-up pair.
    pub fn how_far_it_got(&self) -> Option<(u64, u64)> {
        if self.the_frames.they_cannot_be_counted {
            return None;
        }
        let asked = self.the_frames.asked.as_ref()?;
        let all = asked.how_many();
        let figure = self
            .what_the_note_carried
            .last_frame_figure
            .or(self.what_the_note_carried.given_up_where_it_was);
        match self.the_ending_learned()? {
            Ending::Finished => Some((all, all)),
            Ending::Failed | Ending::Stopped | Ending::GivenUp => match figure {
                Some(figure) => Some((asked.how_many_before(figure)? + 1, all)),
                None => Some((0, all)),
            },
        }
    }

    /// **N16 and N4's four moments, composed from this record alone.** The
    /// submitted moment is the desktop's; the started and ended moments are the
    /// note's, on the machine's clock, and the one duration is theirs.
    pub fn the_four_moments(&self) -> TheFourMoments {
        TheFourMoments {
            submitted: self.submitted_at.clone(),
            started: self.what_the_note_carried.started.clone(),
            ended: self.what_the_note_carried.ended.clone(),
        }
    }

    /// **N11: what is known when nothing was read** — the fields written at
    /// submission, so the entry reads them with no machine and no read.
    pub fn what_is_known_when_nothing_was_read(&self) -> KnownAtSubmission<'_> {
        KnownAtSubmission {
            scene: match &self.scene_name {
                Some(name) => SceneAtSubmission::Named(name),
                None => SceneAtSubmission::WhicheverTheFileWasSavedShowing,
            },
            percentage_only_where_typed: self.resolution_percentage_typed,
            no_pixel_size_at_all: self.picture_size_from_the_read.is_none(),
            the_blend_path: self.blend_path_on_the_machine.as_deref(),
        }
    }

    /// **N22's two draws, composed at the draw from the two fields, stored
    /// never.** Where the record's scene-name field stands filled, the mark
    /// draws naming that scene; where no read has landed, it draws as N11's
    /// own sentence. Absent on a record without the mark.
    pub fn the_no_scene_mark(&self) -> NoSceneMark<'_> {
        if !self.submitted_with_no_scene_named {
            return NoSceneMark::NoMark;
        }
        NoSceneMark::SubmittedWithNoSceneNamed {
            the_scene_it_turned_out_to_be: self.scene_name.as_deref(),
        }
    }

    /// **N12, N13: the server on the entry is drawn from the copied label
    /// and hue alone.** This takes no row and no readings: there is nothing
    /// to read the row with, which is the guarantee.
    pub fn the_server_on_the_entry(&self) -> ServerOnTheEntry<'_> {
        ServerOnTheEntry {
            label: self.server.label.as_deref(),
            hue: self.server.hue,
        }
    }

    /// **N17: which of the entry's machine-reaching controls are offered,
    /// composed at the draw and cached never.** The entry's own copied server
    /// identifier is looked up against the server store at the moment the
    /// entry is drawn; a dangling identifier resolves to no row, and the
    /// reach-back controls are not offered, with the reason stated. **Two
    /// acts are not reach-backs**: *submit again* and deletion, always
    /// offered; **and a third joins them at N18**.
    pub fn which_controls_are_offered(&self, readings: &dyn Readings) -> ControlsOffered {
        let a_row_stands = readings.a_server_row_stands(&self.server.local_server);
        ControlsOffered {
            reach_backs: if a_row_stands {
                ReachBacks::Offered
            } else {
                ReachBacks::NotOfferedTheMachineIsGone
            },
            submit_again: true,
            delete: true,
            show_the_jobs_downloaded_files: true,
        }
    }

    /// **N18: the computer-side shortcut, handed a set of landing places and
    /// never a folder** — `Download` H5 to `Download` H8's set, consumed
    /// whole. One place and it opens; more than one and every place opens
    /// and none is picked (`Download` H7). **It is not a reach-back and N17 never withholds it.**
    /// Where the set is empty, `Download` H7a decides which of two readings
    /// the entry states.
    pub fn the_computer_side_shortcut(&self, readings: &dyn Readings) -> ComputerSideShortcut {
        let LandingPlaces {
            places,
            manual_entries_under_the_jobs_output_folder_published,
        } = readings.the_landing_places(&self.job);
        if places.is_empty() {
            return if manual_entries_under_the_jobs_output_folder_published {
                ComputerSideShortcut::WhatCameHomeWasPickedOutFileByFileAndTheDestinationsAreNotOffered
            } else {
                ComputerSideShortcut::NothingOfTheJobCameHome
            };
        }
        ComputerSideShortcut::OpensEveryPlaceAndPicksNone(
            places.into_iter().map(|place| place.path).collect(),
        )
    }

    /// **N20: the carry-on mark's two draws and the way it offers.** The
    /// way to the earlier entry is a lookup at the draw, on the copied
    /// identity, against the local record alone: a standing record means the
    /// way is offered; a tombstoned identity and an identity no record names
    /// read alike as not offered, and the copied label leaves the mark whole
    /// either way.
    pub fn the_carry_on_mark(
        &self,
        standing_of: impl Fn(&Identity) -> RecordStanding,
    ) -> CarryOnMark<'_> {
        match &self.carries_on_from {
            None => CarryOnMark::NoMark,
            Some(mark) => CarryOnMark::CarriesOnFrom {
                the_label_as_it_stood: &mark.the_label_as_it_stood,
                the_way_is_offered: matches!(
                    standing_of(&mark.earlier_job),
                    RecordStanding::ARecordStands
                ),
            },
        }
    }

    /// **N19's fourth bullet: what survives the machine is the answered
    /// version, and the path is kept without meaning.** On an entry whose
    /// machine is gone the path reads as nothing — nothing resolves it,
    /// nothing composes a list from it, and no installation is reconstructed
    /// out of an entry. What the entry shows of the pair is Design's.
    pub fn the_blender_on_the_entry(&self, the_machine_stands: bool) -> BlenderOnTheEntry<'_> {
        BlenderOnTheEntry {
            version: &self.blender.pick.version,
            path: the_machine_stands.then_some(self.blender.pick.build_path.as_str()),
            origin: self.blender.origin,
        }
    }

    /// **N15: the four marks and N22's fifth are one set, and each is shown
    /// only when true.** Composed, stored never. **N23's mark joins them**
    /// where its field stands (held-job piece B, 2026-10-12).
    pub fn the_marks(&self) -> Vec<AMark<'_>> {
        let mut marks = Vec::new();
        for disturbance in &self.disturbances {
            marks.push(AMark::AFileTheJobNeedsWasDisturbed(disturbance));
        }
        match self.dependency_check {
            Some(DependencyCheckAtSubmission::ACompleteStandingStood) | None => {}
            Some(state) => marks.push(AMark::TheDependencyCheckAtSubmission(state)),
        }
        if let Some(carries_on) = &self.carries_on_from {
            marks.push(AMark::ItCarriesOnFromAnEarlierJob(carries_on));
        }
        if self.submitted_with_no_scene_named {
            marks.push(AMark::ItWasSubmittedWithNoSceneNamed);
        }
        for attempt in &self.attempts_where_the_file_changed {
            marks.push(AMark::TheFileChangedBeforeAttempt(*attempt));
        }
        if let Some(below) = &self.the_driver_was_below_the_bar {
            marks.push(AMark::TheDriverWasBelowTheBar(below));
        }
        marks
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KnownAtSubmission<'a> {
    pub scene: SceneAtSubmission<'a>,
    pub percentage_only_where_typed: Option<u32>,
    pub no_pixel_size_at_all: bool,
    pub the_blend_path: Option<&'a str>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SceneAtSubmission<'a> {
    Named(&'a str),
    /// The spec's own sentence: submitted rendering whichever scene the file
    /// was saved showing (`Rendering` L15, S6).
    WhicheverTheFileWasSavedShowing,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NoSceneMark<'a> {
    NoMark,
    SubmittedWithNoSceneNamed {
        /// Named where the read has landed; N11's own sentence where not.
        the_scene_it_turned_out_to_be: Option<&'a str>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ServerOnTheEntry<'a> {
    pub label: Option<&'a str>,
    pub hue: Option<Hue>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ControlsOffered {
    pub reach_backs: ReachBacks,
    /// `Rendering` A1, A1a — opens the page from the kept setup with no
    /// machine in it.
    pub submit_again: bool,
    /// X1 — unconditional locally.
    pub delete: bool,
    /// N18 — composed from user-held records, never withheld.
    pub show_the_jobs_downloaded_files: bool,
}

/// The three reach-backs: *show outputs*, the whole-job download, and
/// *carry on from where this stopped*. Where a row stands each control's
/// own clauses answer the disconnected and could-not-check cases
/// (`Rendering` A9 among them); this fixes the no-row case alone.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReachBacks {
    Offered,
    NotOfferedTheMachineIsGone,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ComputerSideShortcut {
    OpensEveryPlaceAndPicksNone(Vec<String>),
    /// `Download` H7a's first reading, the spec's own sentence.
    NothingOfTheJobCameHome,
    /// `Download` H7a's second reading — H5a's component-count test keeps
    /// those destinations out of the set.
    WhatCameHomeWasPickedOutFileByFileAndTheDestinationsAreNotOffered,
}

/// What the local record says about an identity, for N20's lookup.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecordStanding {
    ARecordStands,
    Tombstoned,
    NoRecordNamesIt,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CarryOnMark<'a> {
    NoMark,
    CarriesOnFrom {
        the_label_as_it_stood: &'a str,
        the_way_is_offered: bool,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlenderOnTheEntry<'a> {
    pub version: &'a str,
    pub path: Option<&'a str>,
    pub origin: AnswerOrigin,
}

/// The marks, as values; the wording of each is Design's.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AMark<'a> {
    AFileTheJobNeedsWasDisturbed(&'a ADisturbance),
    TheDependencyCheckAtSubmission(DependencyCheckAtSubmission),
    ItCarriesOnFromAnEarlierJob(&'a CarriesOnFrom),
    ItWasSubmittedWithNoSceneNamed,
    /// N9a, per attempt: the file changed, and **what became of the frames
    /// that were already finished** — reused, or rendered again. **No count of
    /// frames is stated either way**, which is a bound rather than an omission.
    ///
    /// **Corrected 2026-09-04, out of a code round.** This mark carried the
    /// attempt's number alone and said, unconditionally, that the frames were
    /// rendered again rather than reused — a statement that is false whenever
    /// the arrangement did not move, which is the ordinary way a file changes.
    TheFileChangedBeforeAttempt(AnAttemptWhereTheFileChanged),
    /// **N23: the job was held for the driver and released after a swap that
    /// did not bring the driver up to the bar** — the driver's version and the
    /// job's Blender as they stood when the release was decided. Last among
    /// the marks, as Design lists it seventh. Held-job piece B, 2026-10-12.
    TheDriverWasBelowTheBar(&'a TheDriverWasBelowTheBar),
}

/// **N16 and N4: the four times are four moments; exactly one duration
/// exists on the entry and it is machine-clock-only.** The started and ended
/// moments are the note's, on the machine's clock; the submitted moment is
/// the desktop's and is never subtracted against anything.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TheFourMoments {
    pub submitted: Option<Reading<()>>,
    pub started: Option<Reading<()>>,
    pub ended: Option<Reading<()>>,
}

impl TheFourMoments {
    /// **N4: the duration an entry shows is computed from the note's two
    /// readings and never across clocks** — `State, identity and recovery`
    /// D5's type refuses a subtraction across two clocks, so a note whose
    /// two moments were somehow taken on different clocks yields no duration
    /// rather than a wrong one. **A gap between *submitted* and *started* is
    /// not computed**: there is no function here that takes the submitted
    /// moment into a subtraction.
    pub fn exactly_one_duration(&self) -> Option<Duration> {
        let started = self.started.as_ref()?;
        let ended = self.ended.as_ref()?;
        ended.since(started)
    }

    /// **N16: the submitted moment is on the desktop's clock**, drawn and
    /// nothing else. `None` where absent, which draws as absent.
    pub fn the_submitted_moment(&self) -> Option<SystemTime> {
        self.submitted
            .as_ref()
            .filter(|reading| *reading.clock() == Clock::ThisComputer)
            .map(Reading::taken_at)
    }
}

/// **N10a's use: `Render Settings` Y15 takes the version pick from a history
/// entry only where the entry records it as the user's own override**;
/// otherwise the automatic match runs afresh for the machine in hand.
pub fn the_version_a_re_submission_takes(kept: &TheBlenderTheEntryKeeps) -> Option<&VersionPick> {
    match kept.origin {
        AnswerOrigin::TheUsersOwnOverride => Some(&kept.pick),
        AnswerOrigin::ResolvedForThem => None,
    }
}

/// **N12's last bullet: a job submitted again from a history entry takes
/// the *current* server's label and hue**, because it is a new submission on
/// whichever server the user picked; and **N14: it composes a fresh
/// companion** from the reading in force at that submission.
pub fn a_re_submission_copies_the_current_server(
    the_current_row: (&Identity, Option<&str>, Option<Hue>),
) -> TheServerAsItStood {
    debug_assert_eq!(the_current_row.0.kind(), IdentityKind::LocalServer);
    TheServerAsItStood {
        local_server: the_current_row.0.clone(),
        label: the_current_row.1.map(str::to_string),
        hue: the_current_row.2,
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::line::testing::a_job;
    use crate::readings::{AJobAsTheMachineReadsIt, ALandingPlace};
    use smoozy_state::observed::MachineClock;

    pub(crate) fn a_server() -> Identity {
        Identity::issue(IdentityKind::LocalServer).unwrap()
    }

    pub(crate) fn a_record(server: &Identity) -> LocalJobRecord {
        LocalJobRecord {
            job: a_job(),
            group: None,
            server: TheServerAsItStood {
                local_server: server.clone(),
                label: Some("the big box".to_string()),
                hue: Some(Hue(210)),
            },
            submitted_at: Some(Reading::taken(
                (),
                SystemTime::UNIX_EPOCH,
                Clock::ThisComputer,
            )),
            label: Some("Kitchen".to_string()),
            blend_path_on_the_machine: Some("/ephemeral/smoozy/assets/kitchen.blend".to_string()),
            scene_name: None,
            submitted_with_no_scene_named: true,
            resolution_percentage_typed: None,
            picture_size_from_the_read: None,
            main_output_from_the_read: None,
            blender: TheBlenderTheEntryKeeps {
                pick: VersionPick {
                    build_path: "/ephemeral/smoozy/blender/blender-4.5.12/blender".to_string(),
                    version: "4.5.12".to_string(),
                },
                origin: AnswerOrigin::ResolvedForThem,
            },
            device_asked_for: TheDeviceAsk::TheUsersOwn(DeviceChoice::TheCards),
            the_files_values_beside_the_overrides: None,
            dependency_check: Some(DependencyCheckAtSubmission::NoStandingAtAll),
            carries_on_from: None,
            disturbances: Vec::new(),
            attempts_where_the_file_changed: Vec::new(),
            the_frames: TheFramesTheReadFound::default(),
            standing: Standing::Provisional,
            the_render_start_has_been_learned: false,
            what_the_note_carried: WhatTheNoteCarried::default(),
            the_job_output_folder: Default::default(),
            the_driver_was_below_the_bar: None,
        }
    }

    /// **`Job files and outputs` P21 on the local record**: a job submitted
    /// to have its folder chosen at render start has none until N9 brings the
    /// machine's choice, which is learned once and never over a folder the
    /// record already holds; a record from before holds the job area's
    /// composition, as its manifest does. The Creator's second notes, piece 4.
    #[test]
    fn a_folder_chosen_at_render_start_is_learned_once_and_an_older_record_keeps_the_job_area() {
        use smoozy_job_files::where_the_folder_sits::{WhereItIs, WhereTheJobOutputFolderSits};
        let layout = smoozy_job_files::layout::TheLayout::of(
            smoozy_job_files::layout::MachineRoot::chosen("/ephemeral/smoozy").unwrap(),
            smoozy_job_files::layout::WhereThePackageAreaSits::UnderTheRoot,
        );
        let server = Identity::issue(IdentityKind::LocalServer).unwrap();
        let older = a_record(&server);
        assert_eq!(
            older.the_job_output_folder(&layout),
            Some(smoozy_job_files::publication::AJobsPaths::composed(&layout, &older.job, Some("Kitchen")).unwrap().job_output_folder)
        );

        let mut record = LocalJobRecord {
            the_job_output_folder: TheJobOutputFolderTheRecordHolds::ChosenAtRenderStart,
            ..a_record(&server)
        };
        assert_eq!(record.the_job_output_folder(&layout), None, "no folder until it arrives");
        record.the_read_fills_what_it_lacks(&WhatTheReadProduced::default());
        assert_eq!(record.the_job_output_folder, TheJobOutputFolderTheRecordHolds::ChosenAtRenderStart);
        let chosen = WhereTheJobOutputFolderSits {
            path: "/home/ubuntu/Render/job-Kitchen".to_string(),
            where_it_is: WhereItIs::WhereTheFilePoints,
        };
        record.the_read_fills_what_it_lacks(&WhatTheReadProduced {
            the_job_output_folder: Some(chosen.clone()),
            ..WhatTheReadProduced::default()
        });
        assert_eq!(record.the_job_output_folder(&layout).as_deref(), Some("/home/ubuntu/Render/job-Kitchen"));
        record.the_read_fills_what_it_lacks(&WhatTheReadProduced {
            the_job_output_folder: Some(WhereTheJobOutputFolderSits { path: "/elsewhere".to_string(), ..chosen }),
            ..WhatTheReadProduced::default()
        });
        assert_eq!(record.the_job_output_folder(&layout).as_deref(), Some("/home/ubuntu/Render/job-Kitchen"), "never chosen again");

        let mut older = a_record(&server);
        older.the_read_fills_what_it_lacks(&WhatTheReadProduced {
            the_job_output_folder: Some(WhereTheJobOutputFolderSits { path: "/elsewhere".to_string(), where_it_is: WhereItIs::WhereTheFilePoints }),
            ..WhatTheReadProduced::default()
        });
        assert_eq!(older.the_job_output_folder, TheJobOutputFolderTheRecordHolds::AsEveryJobBeforeItHadIt);
    }

    /// A readings double: which rows stand, and what `Download` composes.
    pub(crate) struct ADesk {
        pub rows: Vec<Identity>,
        pub landing: LandingPlaces,
    }

    impl Readings for ADesk {
        fn the_queue_reading(
            &self,
            _server: &Identity,
        ) -> Option<Reading<Vec<AJobAsTheMachineReadsIt>>> {
            None
        }
        fn a_server_row_stands(&self, local_server: &Identity) -> bool {
            self.rows.contains(local_server)
        }
        fn the_landing_places(&self, _job: &Identity) -> LandingPlaces {
            self.landing.clone()
        }
        fn now_on_this_computer(&self) -> SystemTime {
            SystemTime::UNIX_EPOCH
        }
    }

    fn no_places() -> LandingPlaces {
        LandingPlaces {
            places: Vec::new(),
            manual_entries_under_the_jobs_output_folder_published: false,
        }
    }

    #[test]
    fn the_read_fills_what_the_record_lacks_and_never_what_smoozys_own_record_carries() {
        let server = a_server();
        let mut record = a_record(&server);
        let read = WhatTheReadProduced {
            scene_name: Some("Interior".to_string()),
            picture_size: Some((1920, 1080)),
            main_output_or_the_jobs_folder: Some(
                "/ephemeral/smoozy/jobs/x/frame_####.png".to_string(),
            ),
            attempts_where_the_file_changed: vec![an_attempt(
                2,
                WhatBecameOfTheFramesAlreadyFinished::TheyWereReused,
            )],
            ..Default::default()
        };
        record.the_read_fills_what_it_lacks(&read);
        assert_eq!(record.scene_name.as_deref(), Some("Interior"));
        assert_eq!(record.picture_size_from_the_read, Some((1920, 1080)));
        assert_eq!(
            record.attempts_where_the_file_changed,
            vec![an_attempt(
                2,
                WhatBecameOfTheFramesAlreadyFinished::TheyWereReused
            )]
        );
        // Smoozy's own record: the mark, the ask, the copies, untouched.
        assert!(record.submitted_with_no_scene_named);
        assert_eq!(
            record.device_asked_for,
            TheDeviceAsk::TheUsersOwn(DeviceChoice::TheCards)
        );
        assert_eq!(record.server.label.as_deref(), Some("the big box"));
        // A later read never overwrites a scene name the record carries.
        let later = WhatTheReadProduced {
            scene_name: Some("Exterior".to_string()),
            ..Default::default()
        };
        record.the_read_fills_what_it_lacks(&later);
        assert_eq!(record.scene_name.as_deref(), Some("Interior"));
        // **N9a is per attempt**, so a second fill-in of the same attempt
        // leaves one entry rather than drawing the mark twice on one entry.
        record.the_read_fills_what_it_lacks(&read);
        assert_eq!(record.attempts_where_the_file_changed.len(), 1);
        assert_eq!(
            record
                .the_marks()
                .iter()
                .filter(|mark| matches!(mark, AMark::TheFileChangedBeforeAttempt(_)))
                .count(),
            1
        );
    }

    /// **The 38's piece 10, items 24 and 38: how far the job got, from the
    /// asked order the read found and N6's figure.** A job stopped with frames
    /// 1001–1061 on the server, figure 1061 of 1001–1240, reads *61/240* — the
    /// frames the server held, the count Telegram says (the live pass on Vast,
    /// 2026-10-19, fault 5) — and never *60/240* or *1061/240*. A clean finish is every
    /// frame; a stop with no figure finished none; a two-view job and a job
    /// with no ending say nothing.
    #[test]
    fn the_asked_order_unfolds_back_into_the_frames_it_was_folded_from() {
        for order in [
            (1..=240).collect::<Vec<i64>>(),
            vec![1, 6, 11, 16, 40, 41, 42, 99],
            vec![7],
        ] {
            assert_eq!(TheFramesAsked::from_the_order(&order).unwrap().the_order(), order);
        }
    }

    /// **`Rented-server runtime` V12's reading is carried over from the note
    /// once and kept**, and a later note carrying none, or its stated absence,
    /// never clears it — the reading is immutable with the ending it rides.
    /// Before step 12's SC4 findings the fill-in carried no reading at all.
    #[test]
    fn the_fill_in_carries_the_free_space_at_the_ending_once_and_keeps_it() {
        use crate::acting::note::TheFreeSpaceAtTheEnding;
        let server = a_server();
        let mut record = a_record(&server);
        let mut note = TheNote::created_with_the_job_record("Shot 40");
        note.the_free_space_at_the_ending = Some(TheFreeSpaceAtTheEnding::NoneWasTaken {
            why: "the job's output folder is gone".to_string(),
        });
        record.the_fill_in_from_the_note(&note);
        assert_eq!(
            record.what_the_note_carried.the_free_space_at_the_ending, None,
            "no reading is not a reading of nought"
        );
        let reading = TheFreeSpaceReadAtTheEnding {
            available_bytes: 0,
            the_folder_it_was_taken_against: "/root/jobs/shot-40".to_string(),
            taken_at: Reading::taken(
                (),
                SystemTime::UNIX_EPOCH + Duration::from_secs(1_700_000_000),
                Clock::AMachine(MachineClock("server-1".to_string())),
            ),
        };
        note.the_free_space_at_the_ending = Some(TheFreeSpaceAtTheEnding::It(reading.clone()));
        record.the_fill_in_from_the_note(&note);
        assert_eq!(record.what_the_note_carried.the_free_space_at_the_ending, Some(reading.clone()));
        note.the_free_space_at_the_ending = None;
        record.the_fill_in_from_the_note(&note);
        assert_eq!(record.what_the_note_carried.the_free_space_at_the_ending, Some(reading));
    }

    #[test]
    fn how_far_it_got_counts_the_frames_up_to_and_including_the_figure_in_the_asked_order() {
        use crate::acting::note::{TheEndingOnTheNote, TheOutcomeAsWritten};
        let server = a_server();
        let mut record = a_record(&server);
        let order: Vec<i64> = (1001..=1240).collect();
        record.the_read_fills_what_it_lacks(&WhatTheReadProduced {
            the_frames: TheFramesTheReadFound {
                asked: TheFramesAsked::from_the_order(&order),
                they_cannot_be_counted: false,
            },
            ..Default::default()
        });
        assert_eq!(
            record.the_frames.asked.as_ref().map(|asked| asked.runs.len()),
            Some(1),
            "a range is one run"
        );
        assert_eq!(record.how_far_it_got(), None, "no ending learned");
        let ended = |record: &mut LocalJobRecord, outcome, figure| {
            record.what_the_note_carried.ending = Some(TheEndingOnTheNote {
                outcome,
                exit_status: None,
                the_attempt_whose_capture_record_holds_the_words: 1,
                the_reason_the_machine_recorded: None,
                the_device_line_as_observed: None,
            });
            record.what_the_note_carried.last_frame_figure = figure;
        };
        ended(&mut record, TheOutcomeAsWritten::LeftStopped, Some(1061));
        assert_eq!(
            record.how_far_it_got(),
            Some((61, 240)),
            "frames 1001–1061 are on the server: 61, as Telegram says"
        );
        ended(&mut record, TheOutcomeAsWritten::Failed, Some(1001));
        assert_eq!(record.how_far_it_got(), Some((1, 240)), "the first frame alone is one");
        ended(&mut record, TheOutcomeAsWritten::Failed, None);
        assert_eq!(record.how_far_it_got(), Some((0, 240)));
        ended(&mut record, TheOutcomeAsWritten::Finished, None);
        assert_eq!(record.how_far_it_got(), Some((240, 240)));
        ended(&mut record, TheOutcomeAsWritten::Failed, Some(7));
        assert_eq!(record.how_far_it_got(), None, "a figure the job never asked for");

        // **A typed order that is not one range** keeps its order.
        let asked = TheFramesAsked::from_the_order(&[50, 10, 20, 30, 7]).unwrap();
        assert_eq!(asked.how_many(), 5);
        assert_eq!(asked.how_many_before(50), Some(0));
        assert_eq!(asked.how_many_before(30), Some(3));
        assert_eq!(asked.how_many_before(7), Some(4));
        assert_eq!(asked.how_many_before(40), None);

        // **`Job files and outputs` I9a's two-view job: no count, whatever the
        // ending.**
        ended(&mut record, TheOutcomeAsWritten::LeftStopped, Some(1061));
        record.the_read_fills_what_it_lacks(&WhatTheReadProduced {
            the_frames: TheFramesTheReadFound {
                asked: None,
                they_cannot_be_counted: true,
            },
            ..Default::default()
        });
        assert_eq!(record.how_far_it_got(), None);
        assert!(record.the_frames.asked.is_some(), "a later read never takes the order away");
    }

    #[test]
    fn the_no_scene_mark_survives_the_read_and_names_the_scene_it_turned_out_to_be() {
        let server = a_server();
        let mut record = a_record(&server);
        assert_eq!(
            record.the_no_scene_mark(),
            NoSceneMark::SubmittedWithNoSceneNamed {
                the_scene_it_turned_out_to_be: None
            }
        );
        record.the_read_fills_what_it_lacks(&WhatTheReadProduced {
            scene_name: Some("Interior".to_string()),
            ..Default::default()
        });
        assert_eq!(
            record.the_no_scene_mark(),
            NoSceneMark::SubmittedWithNoSceneNamed {
                the_scene_it_turned_out_to_be: Some("Interior")
            }
        );
        let mut named = a_record(&server);
        named.submitted_with_no_scene_named = false;
        named.scene_name = Some("Kitchen".to_string());
        assert_eq!(named.the_no_scene_mark(), NoSceneMark::NoMark);
    }

    #[test]
    fn what_is_known_when_nothing_was_read_is_the_submissions_own_fields() {
        let server = a_server();
        let record = a_record(&server);
        let known = record.what_is_known_when_nothing_was_read();
        assert_eq!(
            known.scene,
            SceneAtSubmission::WhicheverTheFileWasSavedShowing
        );
        assert_eq!(known.percentage_only_where_typed, None);
        assert!(known.no_pixel_size_at_all);
        assert_eq!(
            known.the_blend_path,
            Some("/ephemeral/smoozy/assets/kitchen.blend")
        );
    }

    #[test]
    fn the_server_is_drawn_from_the_copies_and_an_old_record_draws_absent_never_a_default() {
        let server = a_server();
        let record = a_record(&server);
        assert_eq!(
            record.the_server_on_the_entry(),
            ServerOnTheEntry {
                label: Some("the big box"),
                hue: Some(Hue(210))
            }
        );
        let mut old = a_record(&server);
        old.server.label = None;
        old.server.hue = None;
        assert_eq!(
            old.the_server_on_the_entry(),
            ServerOnTheEntry {
                label: None,
                hue: None
            }
        );
    }

    #[test]
    fn the_reach_backs_are_withheld_where_no_row_stands_and_the_other_three_never_are() {
        let server = a_server();
        let record = a_record(&server);
        let standing = ADesk {
            rows: vec![server.clone()],
            landing: no_places(),
        };
        let offered = record.which_controls_are_offered(&standing);
        assert_eq!(offered.reach_backs, ReachBacks::Offered);
        let gone = ADesk {
            rows: vec![a_server()],
            landing: no_places(),
        };
        let withheld = record.which_controls_are_offered(&gone);
        assert_eq!(withheld.reach_backs, ReachBacks::NotOfferedTheMachineIsGone);
        assert!(
            withheld.submit_again && withheld.delete && withheld.show_the_jobs_downloaded_files
        );
    }

    #[test]
    fn the_computer_side_shortcut_opens_every_place_and_states_the_right_emptiness() {
        let server = a_server();
        let record = a_record(&server);
        let two = ADesk {
            rows: Vec::new(),
            landing: LandingPlaces {
                places: vec![
                    ALandingPlace {
                        path: "/Users/me/Downloads/Kitchen".to_string(),
                        still_exists: true,
                    },
                    ALandingPlace {
                        path: "/Volumes/Renders/Kitchen".to_string(),
                        still_exists: false,
                    },
                ],
                manual_entries_under_the_jobs_output_folder_published: false,
            },
        };
        assert_eq!(
            record.the_computer_side_shortcut(&two),
            ComputerSideShortcut::OpensEveryPlaceAndPicksNone(vec![
                "/Users/me/Downloads/Kitchen".to_string(),
                "/Volumes/Renders/Kitchen".to_string()
            ])
        );
        let nothing = ADesk {
            rows: Vec::new(),
            landing: no_places(),
        };
        assert_eq!(
            record.the_computer_side_shortcut(&nothing),
            ComputerSideShortcut::NothingOfTheJobCameHome
        );
        let picked = ADesk {
            rows: Vec::new(),
            landing: LandingPlaces {
                places: Vec::new(),
                manual_entries_under_the_jobs_output_folder_published: true,
            },
        };
        assert_eq!(
            record.the_computer_side_shortcut(&picked),
            ComputerSideShortcut::WhatCameHomeWasPickedOutFileByFileAndTheDestinationsAreNotOffered
        );
    }

    #[test]
    fn the_carry_on_mark_reads_whole_and_offers_the_way_only_to_a_standing_record() {
        let server = a_server();
        let mut record = a_record(&server);
        let earlier = a_job();
        record.carries_on_from = Some(CarriesOnFrom {
            earlier_job: earlier.clone(),
            the_label_as_it_stood: "Kitchen, first pass".to_string(),
        });
        assert_eq!(
            record.the_carry_on_mark(|_| RecordStanding::ARecordStands),
            CarryOnMark::CarriesOnFrom {
                the_label_as_it_stood: "Kitchen, first pass",
                the_way_is_offered: true
            }
        );
        for gone in [RecordStanding::Tombstoned, RecordStanding::NoRecordNamesIt] {
            assert_eq!(
                record.the_carry_on_mark(|_| gone),
                CarryOnMark::CarriesOnFrom {
                    the_label_as_it_stood: "Kitchen, first pass",
                    the_way_is_offered: false
                }
            );
        }
        let plain = a_record(&server);
        assert_eq!(
            plain.the_carry_on_mark(|_| RecordStanding::ARecordStands),
            CarryOnMark::NoMark
        );
    }

    #[test]
    fn the_version_keeps_meaning_and_the_path_reads_as_nothing_on_a_gone_machine() {
        let server = a_server();
        let record = a_record(&server);
        let shown = record.the_blender_on_the_entry(false);
        assert_eq!(shown.version, "4.5.12");
        assert_eq!(shown.path, None);
        assert!(record.the_blender_on_the_entry(true).path.is_some());
    }

    #[test]
    fn a_re_submission_takes_the_version_only_where_it_was_the_users_own_override() {
        let server = a_server();
        let record = a_record(&server);
        assert_eq!(the_version_a_re_submission_takes(&record.blender), None);
        let mut own = record.clone();
        own.blender.origin = AnswerOrigin::TheUsersOwnOverride;
        assert_eq!(
            the_version_a_re_submission_takes(&own.blender),
            Some(&own.blender.pick)
        );
    }

    #[test]
    fn a_re_submission_copies_the_current_servers_label_and_hue() {
        let current = a_server();
        let copied = a_re_submission_copies_the_current_server((
            &current,
            Some("the new box"),
            Some(Hue(30)),
        ));
        assert_eq!(copied.local_server, current);
        assert_eq!(copied.label.as_deref(), Some("the new box"));
        assert_eq!(copied.hue, Some(Hue(30)));
    }

    #[test]
    fn the_marks_are_shown_only_where_true_and_a_disturbance_is_appended_never_replaced() {
        let server = a_server();
        let mut record = a_record(&server);
        record.disturbances.push(ADisturbance {
            the_member: "/ephemeral/smoozy/assets/textures/wood.png".to_string(),
            the_act_intended: "replace".to_string(),
            reading: DisturbanceReading::Disturbed,
            how_many: None,
        });
        record.disturbances.push(ADisturbance {
            the_member: "/ephemeral/smoozy/assets/textures/wood.png".to_string(),
            the_act_intended: "delete".to_string(),
            reading: DisturbanceReading::MayHaveBeenDisturbed,
            how_many: None,
        });
        let marks = record.the_marks();
        assert_eq!(marks.len(), 4, "{marks:?}");
        assert!(matches!(marks[0], AMark::AFileTheJobNeedsWasDisturbed(_)));
        assert!(matches!(marks[1], AMark::AFileTheJobNeedsWasDisturbed(_)));
        assert!(matches!(
            marks[2],
            AMark::TheDependencyCheckAtSubmission(DependencyCheckAtSubmission::NoStandingAtAll)
        ));
        assert_eq!(marks[3], AMark::ItWasSubmittedWithNoSceneNamed);
        // A complete standing draws no mark.
        let mut plain = a_record(&server);
        plain.dependency_check = Some(DependencyCheckAtSubmission::ACompleteStandingStood);
        plain.submitted_with_no_scene_named = false;
        assert!(plain.the_marks().is_empty());
    }

    #[test]
    fn exactly_one_duration_exists_and_it_is_machine_clock_only() {
        let machine = Clock::AMachine(MachineClock("server-7".to_string()));
        let start = SystemTime::UNIX_EPOCH + Duration::from_secs(1_000);
        let moments = TheFourMoments {
            submitted: Some(Reading::taken(
                (),
                start - Duration::from_secs(600),
                Clock::ThisComputer,
            )),
            started: Some(Reading::taken((), start, machine.clone())),
            ended: Some(Reading::taken(
                (),
                start + Duration::from_secs(90),
                machine.clone(),
            )),
        };
        assert_eq!(
            moments.exactly_one_duration(),
            Some(Duration::from_secs(90))
        );
        assert_eq!(
            moments.the_submitted_moment(),
            Some(start - Duration::from_secs(600))
        );
        // Two moments that do not line up rather than a number that is wrong.
        let across_clocks = TheFourMoments {
            started: Some(Reading::taken((), start, Clock::ThisComputer)),
            ..moments.clone()
        };
        assert_eq!(across_clocks.exactly_one_duration(), None);
        // No ending learned: no duration, and the submitted moment still draws.
        let unended = TheFourMoments {
            ended: None,
            ..moments
        };
        assert_eq!(unended.exactly_one_duration(), None);
        assert!(unended.the_submitted_moment().is_some());
    }

    fn an_attempt(
        attempt: u32,
        what_became_of_the_frames_already_finished: WhatBecameOfTheFramesAlreadyFinished,
    ) -> AnAttemptWhereTheFileChanged {
        AnAttemptWhereTheFileChanged {
            attempt,
            what_became_of_the_frames_already_finished,
        }
    }

    fn an_attempt_record(
        number: u32,
        the_file_changed_before_this_attempt: bool,
        a_frame_an_earlier_attempt_wrote_was_kept: bool,
    ) -> smoozy_job_files::attempts::AttemptRecord {
        smoozy_job_files::attempts::AttemptRecord {
            number,
            frame_list_in_the_order_it_was_written: vec![1, 2, 3],
            map_in_force: smoozy_job_files::attempts::TheMapInForce::NoneWasComposedForThisAttempt,
            file_identity_at_this_attempt: smoozy_server_protocol::transfers::SourceReading {
                size: 1,
                inode: "1".to_string(),
                modification_time: 1,
                change_time: 1,
            },
            the_file_changed_before_this_attempt,
            a_frame_an_earlier_attempt_wrote_was_kept,
        }
    }

    /// **N9a: the entry says which of the two happened, and the two are
    /// different marks.**
    ///
    /// The mark used to be one shape carrying the attempt's number alone, and
    /// it said flatly that the frames were rendered again. **Where the
    /// arrangement did not move they are reused**, so a mark that cannot say
    /// so is a mark that tells the user something that did not happen.
    #[test]
    fn the_mark_says_which_of_the_two_happened_and_the_two_are_not_the_same_mark() {
        let server = a_server();
        let mut reused = a_record(&server);
        reused.attempts_where_the_file_changed = vec![an_attempt(
            2,
            WhatBecameOfTheFramesAlreadyFinished::TheyWereReused,
        )];
        let mut again = a_record(&server);
        again.attempts_where_the_file_changed = vec![an_attempt(
            2,
            WhatBecameOfTheFramesAlreadyFinished::TheyWereRenderedAgain,
        )];

        let the_mark = |record: &LocalJobRecord| {
            record
                .the_marks()
                .into_iter()
                .find_map(|mark| match mark {
                    AMark::TheFileChangedBeforeAttempt(attempt) => Some(attempt),
                    _ => None,
                })
                .expect("the file changed before attempt 2, so the mark is drawn")
        };

        assert_eq!(
            the_mark(&reused).what_became_of_the_frames_already_finished,
            WhatBecameOfTheFramesAlreadyFinished::TheyWereReused
        );
        assert_eq!(
            the_mark(&again).what_became_of_the_frames_already_finished,
            WhatBecameOfTheFramesAlreadyFinished::TheyWereRenderedAgain
        );
        assert_ne!(the_mark(&reused), the_mark(&again));
        // **Per attempt and not per job**, and the attempt is named.
        assert_eq!(the_mark(&reused).attempt, 2);
    }

    /// **No absence, of any length, removes a provisional record.**
    ///
    /// **This is the first thing a later agent will break**, because folding
    /// the third branch into the second looks like tidying. The scan is of the
    /// working half's own text, because the fault arrives as a line added
    /// later: a timeout is arithmetic against a clock, and there is no clock,
    /// no duration and no count anywhere in step 2 to do it with.
    #[test]
    fn no_absence_of_any_length_removes_a_provisional_record() {
        assert_eq!(
            what_step_2_does(WhatTheMachineSaidAboutTheJob::TheMachineCannotSay),
            WhatAnAbsenceDoes::TheRecordStaysProvisionalAndIsAskedAgainNextTime
        );
        // Asked again, however many times, and the answer never changes.
        for _ in 0..1_000 {
            assert_ne!(
                what_step_2_does(WhatTheMachineSaidAboutTheJob::TheMachineCannotSay),
                WhatAnAbsenceDoes::TheRecordIsRemovedLeavingNoTrace
            );
        }
        // The other two branches are the ones that may move a record, and they
        // are the machine's own answers rather than an elapsed anything.
        assert_eq!(
            what_step_2_does(WhatTheMachineSaidAboutTheJob::ItHoldsTheJob),
            WhatAnAbsenceDoes::TheRecordBecomesConfirmed
        );
        assert_eq!(
            what_step_2_does(
                WhatTheMachineSaidAboutTheJob::ItsOperationRecordShowsTheSubmissionWasRefusedOrNeverTaken
            ),
            WhatAnAbsenceDoes::TheRecordIsRemovedLeavingNoTrace
        );

        let source = include_str!("record.rs");
        let working_half = source
            .split("#[cfg(test)]")
            .next()
            .expect("the file has a working half");
        let step_2 = working_half
            .split("pub fn what_step_2_does")
            .nth(1)
            .expect("step 2 is in the working half");
        let body = step_2.split("\n}").next().expect("the function has a body");
        for forbidden in [
            "SystemTime",
            "Duration",
            "Instant",
            "elapsed",
            "since",
            "now",
            "deadline",
            "timeout",
            "attempts",
        ] {
            assert!(
                !body.contains(forbidden),
                "{forbidden} in step 2 is the determination \
                 `Preparation/Specs/Whole app.md` forbids"
            );
        }
    }

    /// **The fill-in writes the note's carried half and nothing else, and what
    /// N16 protects is never overwritten.**
    #[test]
    fn the_fill_in_carries_the_notes_half_and_never_overwrites_smoozys_own_record() {
        let server = a_server();
        let mut record = a_record(&server);
        let before = record.clone();
        let machine = Clock::AMachine(MachineClock(server.as_text()));
        let started = SystemTime::UNIX_EPOCH + Duration::from_secs(9_000);

        let mut note = TheNote::created_with_the_job_record("a label the note also holds");
        note.started = Some(Reading::taken((), started, machine.clone()));
        note.ended = Some(Reading::taken(
            (),
            started + Duration::from_secs(120),
            machine,
        ));
        note.ending = Some(TheEndingOnTheNote {
            outcome: crate::acting::note::TheOutcomeAsWritten::Finished,
            exit_status: Some(0),
            the_attempt_whose_capture_record_holds_the_words: 2,
            the_reason_the_machine_recorded: None,
            the_device_line_as_observed: None,
        });
        note.attempts = 2;
        note.re_run_marks = vec![AnAttemptMark::for_a_re_run(2, &[41, 42, 43])];
        note.last_frame_figure = Some(40);

        record.the_fill_in_from_the_note(&note);

        // The carried half landed.
        assert_eq!(record.the_ending_learned(), Some(Ending::Finished));
        assert_eq!(record.what_the_note_carried.attempts, 2);
        assert_eq!(record.what_the_note_carried.last_frame_figure, Some(40));
        assert_eq!(
            record.what_the_note_carried.re_run_marks[0].the_re_run_proceeded_from,
            Some(41)
        );
        assert_eq!(
            record.the_four_moments().exactly_one_duration(),
            Some(Duration::from_secs(120))
        );
        // **C9: the note's started moment is the learning.**
        assert!(record.the_render_start_has_been_learned);

        // **N16: the submitted moment is untouched, and it is still this
        // computer's.** A machine's clock reaching it would put a moment in a
        // field D5 says is the desktop's.
        assert_eq!(record.submitted_at, before.submitted_at);
        assert_eq!(
            record.the_four_moments().the_submitted_moment(),
            before.the_four_moments().the_submitted_moment()
        );
        assert!(record.the_four_moments().the_submitted_moment().is_some());

        // And every other field Smoozy's own record carries.
        assert_eq!(record.label, before.label);
        assert_eq!(record.server, before.server);
        assert_eq!(record.scene_name, before.scene_name);
        assert_eq!(
            record.submitted_with_no_scene_named,
            before.submitted_with_no_scene_named
        );
        assert_eq!(
            record.blend_path_on_the_machine,
            before.blend_path_on_the_machine
        );
        assert_eq!(record.blender, before.blender);
        assert_eq!(record.device_asked_for, before.device_asked_for);
        assert_eq!(record.dependency_check, before.dependency_check);
        assert_eq!(record.carries_on_from, before.carries_on_from);
        assert_eq!(record.disturbances, before.disturbances);
        assert_eq!(record.the_marks().len(), before.the_marks().len());

        // **A later note that has forgotten the start does not un-learn it**,
        // and a null ending never replaces one already carried (N1).
        let quiet = TheNote::created_with_the_job_record("a label the note also holds");
        record.the_fill_in_from_the_note(&quiet);
        assert!(record.the_render_start_has_been_learned);
        assert_eq!(record.the_ending_learned(), Some(Ending::Finished));
        assert_eq!(record.what_the_note_carried.attempts, 2);
    }

    /// **Both facts are read off the attempt record and neither is computed
    /// here** — `Job files and outputs` R13 for the first, and
    /// `Job files and outputs` C12a for the second — and an attempt whose
    /// record does not say the file changed draws no mark at all.
    #[test]
    fn the_two_facts_are_read_off_the_attempt_record_and_an_unchanged_attempt_draws_nothing() {
        assert_eq!(
            AnAttemptWhereTheFileChanged::read_from(&an_attempt_record(1, false, true)),
            None
        );
        assert_eq!(
            AnAttemptWhereTheFileChanged::read_from(&an_attempt_record(2, true, true)),
            Some(an_attempt(
                2,
                WhatBecameOfTheFramesAlreadyFinished::TheyWereReused
            ))
        );
        assert_eq!(
            AnAttemptWhereTheFileChanged::read_from(&an_attempt_record(3, true, false)),
            Some(an_attempt(
                3,
                WhatBecameOfTheFramesAlreadyFinished::TheyWereRenderedAgain
            ))
        );
    }
}
