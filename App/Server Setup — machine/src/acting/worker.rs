//! **The setup worker: which step it may take next, what one commanded step
//! comes to, and what it reports.**
//!
//! **The worker performs and reports; the supervisor writes.** That is
//! `Rented-server runtime`'s own workers-report-supervisor-writes discipline
//! and S2a states it for this run in as many words: *"A
//! machine-side step's words are the worker's own capture … reported to the
//! supervisor, which writes."* **So nothing here opens a record.**
//!
//! **Every marker of another document in this file is written out at its
//! site.** This plan defines an S1, a W3 and a W4 of its own, so a header list
//! naming `Rented-server runtime`'s could not tell the two apart.

use std::path::{Path, PathBuf};

use smoozy_framing::requests as asked;

use crate::acting::fetching::{self, MakesTheRequestOnTheMachinesOwnLine, TheFetchDidNot};
use crate::acting::steps::{
    self, RunsOneCommandOnTheMachine, WhatOneCommandPrinted, WhatThePublicationDid,
    WhatTheVerificationClaims,
};
use crate::blender::{
    read_the_version_answer, what_a_version_answer_does, Version, WhatTheBelowFloorAnswerDoes,
    WhatTheUserAnswered, THE_TIME_LIMIT_EVERY_BLENDER_RUN_CARRIES,
};
use crate::driving::{who_performs, WhoPerformsTheStep};
use crate::ready_action::Step;
use crate::run::Boundary;

/// **Y7's machine-side list, as a step the worker is commanded.**
///
/// **This is Y7's own granularity and not [`Step`]'s**, and the two are
/// different on purpose. A [`Step`] is what a boundary hangs on; Y7's
/// machine-side list — *"fetch, unpack, `ldd` reads, publication,
/// read-backs"* — is what the worker is told to do. The tooling step's install
/// and the libraries step's install are elevated and the desktop's
/// ([`who_performs`]), so the worker's part of each is a step of its own here.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhichStep {
    /// R1 item 1's readings. **The worker does not perform it** —
    /// [`the_worker_performs`] says why.
    TheReadings,
    /// B4's fetch.
    TheFetch,
    /// R1 item 3's second half.
    TheUnpack,
    /// R1 item 4's first read.
    TheLibraryChainIsRead,
    /// R1 item 4's re-read.
    TheLibraryChainIsReadAgain,
    /// R1 item 7, whole.
    TheBuildIsPublished,
    /// **`Server Setup` B15's plain deletion of the install's folder.** Added
    /// 2026-10-10, the 38's piece 14c.
    TheBuildIsRemoved,
    /// R1 item 8's Telegram material. **The worker does not perform it** —
    /// [`the_worker_performs`] says why.
    TheTelegramMaterial,
    /// R1 item 9's final verification.
    TheFinalVerification,
}

impl WhichStep {
    /// **Every step the worker is commanded, in R1's order.**
    pub const IN_ORDER: [WhichStep; 9] = [
        WhichStep::TheReadings,
        WhichStep::TheFetch,
        WhichStep::TheUnpack,
        WhichStep::TheLibraryChainIsRead,
        WhichStep::TheLibraryChainIsReadAgain,
        WhichStep::TheBuildIsPublished,
        WhichStep::TheBuildIsRemoved,
        WhichStep::TheTelegramMaterial,
        WhichStep::TheFinalVerification,
    ];

    /// **The wire word, which is `smoozy-framing`'s and not a second
    /// spelling.**
    pub fn as_asked(self) -> &'static str {
        match self {
            WhichStep::TheReadings => asked::THE_READINGS_STEP,
            WhichStep::TheFetch => asked::THE_FETCH_STEP,
            WhichStep::TheUnpack => asked::THE_UNPACK_STEP,
            WhichStep::TheLibraryChainIsRead => asked::THE_LIBRARY_CHAIN_IS_READ_STEP,
            WhichStep::TheLibraryChainIsReadAgain => asked::THE_LIBRARY_CHAIN_IS_READ_AGAIN_STEP,
            WhichStep::TheBuildIsPublished => asked::THE_BUILD_IS_PUBLISHED_STEP,
            WhichStep::TheBuildIsRemoved => asked::THE_BUILD_IS_REMOVED_STEP,
            WhichStep::TheTelegramMaterial => asked::THE_TELEGRAM_MATERIAL_STEP,
            WhichStep::TheFinalVerification => asked::THE_FINAL_VERIFICATION_STEP,
        }
    }

    /// **Read back off a command**, and nothing else is read as a step.
    pub fn read_back(word: &str) -> Option<WhichStep> {
        WhichStep::IN_ORDER
            .into_iter()
            .find(|step| step.as_asked() == word)
    }

    /// **Which [`Step`] of the run this is the worker's part of**, which is what
    /// its boundary hangs on and what the re-run rule is measured in.
    pub fn its_step(self) -> Step {
        match self {
            WhichStep::TheReadings => Step::Readings,
            WhichStep::TheFetch => Step::Fetch,
            WhichStep::TheUnpack => Step::Tooling,
            WhichStep::TheLibraryChainIsRead | WhichStep::TheLibraryChainIsReadAgain => {
                Step::Libraries
            }
            WhichStep::TheBuildIsPublished => Step::BlenderPublication,
            WhichStep::TheBuildIsRemoved => Step::BlenderRemoval,
            WhichStep::TheTelegramMaterial => Step::TelegramMaterial,
            WhichStep::TheFinalVerification => Step::Done,
        }
    }

    /// **Y3: the boundary this step's completion lets the supervisor write**,
    /// where it completes an R1 step outright.
    ///
    /// **Three of the eight feed no boundary and each for its own reason.**
    ///
    /// - [`WhichStep::TheLibraryChainIsRead`] is the first half of R1 item 4;
    ///   the elevated install follows it and the re-read closes the step.
    /// - [`WhichStep::TheFinalVerification`] verifies and does not end the run.
    ///   Y4's *done* row guarantees *"the final verification passed; the state
    ///   (where set) is cleared and reconciliation ran"*, and the clear is
    ///   `Rented-server runtime` P3's
    ///   act through the ordinary command path — the desktop's. **So the
    ///   verification reports its finding and the clear writes *done*.**
    /// - The two the worker does not perform feed nothing here because the
    ///   desktop performs them and writes their boundaries, as it does for
    ///   every elevated step.
    pub fn the_boundary_it_feeds(self) -> Option<Boundary> {
        match self {
            WhichStep::TheFetch => Some(Boundary::Fetched),
            WhichStep::TheUnpack => Some(Boundary::Tooling),
            WhichStep::TheLibraryChainIsReadAgain => Some(Boundary::Libraries),
            WhichStep::TheBuildIsPublished => Some(Boundary::Blender),
            WhichStep::TheBuildIsRemoved => Some(Boundary::Removed),
            WhichStep::TheReadings
            | WhichStep::TheLibraryChainIsRead
            | WhichStep::TheTelegramMaterial
            | WhichStep::TheFinalVerification => None,
        }
    }
}

/// **Whether the setup worker performs this step at all, and the two it does
/// not are refused by name rather than answered emptily.**
///
/// **[`who_performs`] gives both of them to the setup worker and neither can be
/// performed there. That is a finding rather than a design**, and it is written
/// here so a reader meets it at the refusal:
///
/// - **The readings.** R1 item 1 is `Servers` H11's table — the PCI vendor scan
///   and the driver's own query — and both command strings are declared in
///   `smoozy-servers`, a crate the machine's program may never reach. **The
///   desktop takes them over the command channel and writes the boundary**, as
///   it does for an elevated step.
/// - **The Telegram material.** Its placement is built on the desktop, inside
///   the authenticated connection with every path a protocol field, and
///   `Server Setup` G1's desired state is `Rented-server runtime` W8's, written
///   through the ordinary command path. **`Server Setup`'s §7 entry on its own
///   acting-half list stands**, and a second placement here would be the
///   one-fact-in-two-places fault.
pub fn the_worker_performs(step: WhichStep) -> bool {
    !matches!(step, WhichStep::TheReadings | WhichStep::TheTelegramMaterial)
}

/// **Why the worker takes a step at all**, because [`who_performs`] alone does
/// not answer it for three of them.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhyTheWorkerTakesIt {
    /// **[`who_performs`] gives the whole R1 step to the setup worker.**
    TheJudgementGivesItToTheWorker,
    /// **Y7 names the act machine-side inside a step the command channel
    /// drives.** *"Machine-side steps (fetch, unpack, `ldd` reads,
    /// publication, read-backs) are performed by the setup worker"*, while the
    /// tooling step's tool install and the libraries step's package install are
    /// elevated — so [`who_performs`] gives the **step** to the command channel
    /// and Y7 gives **this act inside it** to the worker. Both are true and the
    /// granularities differ; this is where that is said.
    Y7NamesTheActMachineSideInsideAnElevatedStep,
    /// **Neither: the worker does not perform it** — [`the_worker_performs`]
    /// says why for each of the two.
    TheWorkerDoesNotPerformIt,
}

/// **[`who_performs`] and Y7's machine-side list, read together.**
pub fn why_the_worker_takes_it(step: WhichStep) -> WhyTheWorkerTakesIt {
    if !the_worker_performs(step) {
        return WhyTheWorkerTakesIt::TheWorkerDoesNotPerformIt;
    }
    match who_performs(step.its_step()) {
        WhoPerformsTheStep::TheSetupWorker => WhyTheWorkerTakesIt::TheJudgementGivesItToTheWorker,
        WhoPerformsTheStep::TheCommandChannelUnderElevation => {
            WhyTheWorkerTakesIt::Y7NamesTheActMachineSideInsideAnElevatedStep
        }
    }
}

/// **S4 item 3: the recovery run resumes at the recorded boundary and never
/// before it.** *"Steps at or before the marker are not run again; the step
/// after it runs afresh."*
///
/// **The comparison is in [`Step`]s and not in boundaries**, because two worker
/// steps share the libraries step and a marker of *libraries* must refuse both.
pub fn may_this_step_be_performed(the_marker: Option<Boundary>, step: WhichStep) -> bool {
    where_in_the_order(step.its_step()) > the_marker.map_or(-1, |marker| where_in_the_order(marker.its_step()))
}

/// **Where one step sits in R1's order**, which is the one comparison every
/// rule about *at or past a step* is made in.
fn where_in_the_order(step: Step) -> i32 {
    Step::IN_ORDER
        .iter()
        .position(|it| *it == step)
        .expect("every step is in the order") as i32
}

/// **Whether this run published a Blender**, read off the run's own two lists
/// rather than off anything a caller remembers.
///
/// **Two readings, and it takes both.** `Server Setup` Y4's composed list says
/// whether the run had a publication in it at all (R1 item 7 is composed only
/// where the run fetches a Blender), and `Server Setup` Y3's marker — the last
/// boundary reached — says whether the run got that far. **That is what tells a
/// run with no Blender in it — whose final check rightly reads nothing
/// (`Server Setup` R4) — from a run whose build has gone missing between two
/// steps.** Added 2026-09-23, out of step 10c's fifth pass on Vast.
///
/// **Until 2026-10-04 it read the marker alone**, as *at or past the
/// publication's place in R1's order*. *Telegram* (R1 item 8) comes after that
/// place, so every run that reached its Telegram step counted as one that had
/// published a Blender, and its final check ended it: `Server Setup` G2's
/// Telegram placement every time, and every ready action with Telegram on and
/// no Blender asked for (`Audits/Build/Step 10c/The setup's final check and
/// Install Telegram — what landed.md`). **The marker is still read**, because
/// an abandon (`Server Setup` S6) inherits the list of a run it takes over and
/// runs only this check: over a Blender install stopped before its
/// publication, the list holds one and nothing was published.
pub fn the_run_published_a_blender(the_composed_list: &[Boundary], the_marker: Option<Boundary>) -> bool {
    the_composed_list.contains(&Boundary::Blender)
        && the_marker.is_some_and(|marker| {
            where_in_the_order(marker.its_step()) >= where_in_the_order(Step::BlenderPublication)
        })
}

/// **What one commanded step needs from the desktop**, which is the version and
/// the executable and nothing else.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct TheStepsInputs {
    /// B4's fetch, R1 item 3's unpack and R1 item 7's publication —
    /// `smoozy-framing`'s `THE_STEPS_THAT_READ_THE_BLENDER_VERSION`.
    pub version: Option<Version>,
    /// **B4's second route: the build service's bare file name**, where the
    /// step asked for an unfinished build listed there — read off the same
    /// carried text as the version (`crate::blender::TheVersionAsked`). Added
    /// 2026-10-02, the Creator's notes 5 and 7 on 0.1.7.
    pub the_build_services_file: Option<String>,
    /// **The build a library step reads against**, where it is not this run's
    /// staged one.
    pub executable: Option<PathBuf>,
    /// **`Server Setup` B7: the archive the user uploaded, in place of a
    /// fetch** — its bare file name in this run's own staging area
    /// ([`where_the_users_archive_is_staged`]). Present, the fetch is refused,
    /// the unpack reads this archive, and the publication asks the build its
    /// version rather than requiring one. Added 2026-10-08, the 38's piece 14a.
    pub the_users_own_archive: Option<String>,
    /// **`Server Setup` B7a: the user's answer to the below-4.5 question**,
    /// carried on the publication command that follows a hold.
    pub the_users_answer: Option<WhatTheUserAnswered>,
    /// **B15's second route: the package the machine's own software manager
    /// holds the build under**, on the removal step where the build is that
    /// manager's. Present, the step deletes nothing and reads that the build
    /// is gone ([`the_build_is_removed`]). Added 2026-10-16, step 10c's owed
    /// piece 6.
    pub the_managers_package: Option<String>,
}

/// **Where a run's own material sits**, composed by whoever starts the worker
/// from the recorded root and never by this crate.
///
/// **`Job files and outputs` owns every machine path but the staging path**,
/// and this crate composes neither: it is handed them.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WhereTheRunWorks {
    /// The staging area this run's archive and unpacked material sit in.
    pub staging: PathBuf,
    /// `Server Setup` B9's Blender area, under the root.
    pub the_blender_area: PathBuf,
    /// **When the setup run started, as its record holds it**, which B8's
    /// folder name carries (2026-10-08). **Absent, the publication is refused**
    /// rather than named by any other time: a name the run's record cannot
    /// compose again is one a recovery could not find the build under.
    pub the_run_started: Option<std::time::SystemTime>,
}

/// **What one commanded step came to.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WhatTheStepCameTo {
    /// **Whether the step ran to its end.**
    pub it_ran_to_its_end: bool,
    /// **The boundary the supervisor may now write**, where this step fed one
    /// and ran to its end.
    pub the_boundary: Option<Boundary>,
    /// **Y4a's mark**, where there is a boundary: whether the step performed
    /// its act or found nothing to perform.
    pub it_performed: bool,
    /// **The machine's own words**, S2a's — verbatim, whole, the
    /// streams never merged and never mined for a cause. **Absent where nothing
    /// was printed and where the failure is Smoozy's own verification rather
    /// than a tool's refusal**, in which case the recorded fact of that read is
    /// the content.
    pub the_machines_own_words: Option<String>,
    /// **Why it did not run to its end**, in Smoozy's own words, where the
    /// failure is a verification of its own.
    pub why_not: Option<String>,
    /// **R1 item 4's whole not-found list**, from either library step.
    pub the_libraries_not_found: Vec<String>,
    /// **What the staged build answered**, from the publication step.
    pub the_version_answered: Option<Version>,
    /// **Where the build was published**, from the publication step.
    pub published_at: Option<PathBuf>,
    /// **This run's staged build**, once the unpack has found one — carried so
    /// the worker can hold it for the steps that follow.
    pub the_staged_executable: Option<PathBuf>,
    /// **`Server Setup` B7a: the publication is held for a person.** The
    /// user's build answered below 4.5 ([`Self::the_version_answered`]),
    /// nothing was published, and the build stays in staging. **Not a failure
    /// and not a boundary**: the run waits at its last boundary for the
    /// publication to be commanded again with the user's answer.
    pub held_for_a_person: bool,
    /// **`Server Setup` B7a's recorded discard**, where the user declined: the
    /// staged build was abandoned and never published, in Smoozy's own words.
    pub the_discard: Option<String>,
    /// **`Server Setup` Y7a item 2: what every command the step ran printed**,
    /// both streams apart, in the order the commands ran — a step that ran to
    /// its end as much as one that did not. **Empty where the step ran no
    /// program** (the fetch, a removal, a refusal before anything ran) **and
    /// where every program printed nothing**, which is a reading and never a
    /// failure. Never S2a's composed text, which
    /// [`Self::the_machines_own_words`] still carries for a failed step. Added
    /// 2026-10-24, the Creator's ruling on the sixteenth notes, note 1.
    pub what_its_commands_printed: Vec<WhatOneCommandPrinted>,
}

impl WhatTheStepCameTo {
    fn did_not(why: String) -> WhatTheStepCameTo {
        WhatTheStepCameTo {
            it_ran_to_its_end: false,
            the_boundary: None,
            it_performed: false,
            the_machines_own_words: None,
            why_not: Some(why),
            the_libraries_not_found: Vec::new(),
            the_version_answered: None,
            published_at: None,
            the_staged_executable: None,
            held_for_a_person: false,
            the_discard: None,
            what_its_commands_printed: Vec::new(),
        }
    }

    fn ran(step: WhichStep, it_performed: bool) -> WhatTheStepCameTo {
        WhatTheStepCameTo {
            it_ran_to_its_end: true,
            the_boundary: step.the_boundary_it_feeds(),
            it_performed,
            the_machines_own_words: None,
            why_not: None,
            the_libraries_not_found: Vec::new(),
            the_version_answered: None,
            published_at: None,
            the_staged_executable: None,
            held_for_a_person: false,
            the_discard: None,
            what_its_commands_printed: Vec::new(),
        }
    }
}

/// **Y7: one step, performed.**
///
/// **It performs and reports; it writes no record.** The boundary it names is
/// the supervisor's to write in its own durable act, with Y4a's mark and Y3's
/// moment beside it.
///
/// **A step the worker does not perform, and a step at or before the marker,
/// are refused by name.** The second is S4 item 3 kept at the act rather than
/// left to a caller — a recovery that re-ran the fetch would re-fetch 380 MB on
/// a machine being paid for.
///
/// **Two places on the machine are handed in rather than remembered here**, and
/// they are different things: `the_staged_executable` is where the unpack left
/// this run's build, and `the_published_folder` is where the publication put it
/// afterwards. **The publication consumes the first to make the second**
/// (R1 item 7), so after it has run the staged path names nothing at all —
/// which is why the final verification reads the published one and never falls
/// back to the staged one. **Added 2026-09-23, out of step 10c's fifth pass on
/// Vast**, where every Blender install failed its last step because the
/// publication's own answer reached nobody.
///
/// **`the_composed_list` is `Server Setup` Y4's**, what the run consists of,
/// and the final check is its one reader ([`the_run_published_a_blender`]).
/// Added 2026-10-04.
pub fn perform_one_step(
    step: WhichStep,
    the_marker: Option<Boundary>,
    the_composed_list: &[Boundary],
    inputs: &TheStepsInputs,
    works: &WhereTheRunWorks,
    the_run: &str,
    the_staged_executable: Option<&Path>,
    the_published_folder: Option<&Path>,
    line: &dyn MakesTheRequestOnTheMachinesOwnLine,
    runner: &dyn RunsOneCommandOnTheMachine,
) -> WhatTheStepCameTo {
    perform_one_step_telling(
        step,
        the_marker,
        the_composed_list,
        inputs,
        works,
        the_run,
        the_staged_executable,
        the_published_folder,
        line,
        runner,
        &crate::acting::NobodyIsTold,
    )
}

/// **[`perform_one_step`], telling how far the fetch (Y7b) or the unpack (Y7c)
/// has got as it goes.** Every other step tells nothing. **How often it reaches
/// the run's state is the caller's** ([`crate::acting::NoMoreOftenThan`]), and
/// a step that fails ends exactly as it did before. Added 2026-10-18, the
/// Creator's fourteenth notes, piece U.
pub fn perform_one_step_telling(
    step: WhichStep,
    the_marker: Option<Boundary>,
    the_composed_list: &[Boundary],
    inputs: &TheStepsInputs,
    works: &WhereTheRunWorks,
    the_run: &str,
    the_staged_executable: Option<&Path>,
    the_published_folder: Option<&Path>,
    line: &dyn MakesTheRequestOnTheMachinesOwnLine,
    runner: &dyn RunsOneCommandOnTheMachine,
    told: &dyn crate::acting::TellsHowFarItHasGot,
) -> WhatTheStepCameTo {
    perform_one_step_handing_on(
        step,
        the_marker,
        the_composed_list,
        inputs,
        works,
        the_run,
        the_staged_executable,
        the_published_folder,
        line,
        runner,
        told,
        &steps::NothingIsHandedOn,
        crate::driving::THE_REFRESH_CADENCE_WHILE_WORDS_MOVE,
    )
}

/// **[`perform_one_step_telling`], with the step's words handed on as they
/// print** — `Server Setup` Y7a item 2, Y7b and Y7c, as changed on 2026-10-24
/// by the eighteenth notes' design pass. Added 2026-10-25, piece 2b.
///
/// **Three kinds of words go to `on`, each on the step's own streams and in
/// the order they were printed**, and they are the step's words like any
/// program's:
///
/// - **every command the step runs**, each stream's bytes as they arrive;
/// - **the worker's own progress lines** while the fetch (Y7b) or the unpack
///   (Y7c) runs, on the ordinary stream, no more often than `every`
///   ([`TheProgressLines`]);
/// - **where the step fails with a sentence of Smoozy's own** — S2a's composed
///   text, which no command printed — **that sentence, last, on the error
///   stream** ([`the_failing_steps_own_sentence`]), so *What the server said*
///   and *Server output* say the same thing. A step whose S2a text is a
///   command's printed words adds nothing: those words are already there.
///
/// **The step itself is not changed by any of it**: what it comes to is
/// exactly what it came to before, and nothing decides anything on the words.
#[allow(clippy::too_many_arguments)]
pub fn perform_one_step_handing_on(
    step: WhichStep,
    the_marker: Option<Boundary>,
    the_composed_list: &[Boundary],
    inputs: &TheStepsInputs,
    works: &WhereTheRunWorks,
    the_run: &str,
    the_staged_executable: Option<&Path>,
    the_published_folder: Option<&Path>,
    line: &dyn MakesTheRequestOnTheMachinesOwnLine,
    runner: &dyn RunsOneCommandOnTheMachine,
    told: &dyn crate::acting::TellsHowFarItHasGot,
    on: &dyn steps::HandsOnWhatIsPrinted,
    every: std::time::Duration,
) -> WhatTheStepCameTo {
    // **`Server Setup` Y7a item 2: every command the step runs leaves through
    // the one seam, and what each printed is kept there**, so the report
    // carries every command's two streams whatever the step's own reading of
    // them. Added 2026-10-24, the Creator's ruling on the sixteenth notes,
    // note 1. **From 2026-10-25 they are handed on as they print as well.**
    let kept = steps::EveryCommandsWordsAreKept::around_handing_on(runner, on);
    let mut came = the_step_is_performed(
        step,
        the_marker,
        the_composed_list,
        inputs,
        works,
        the_run,
        the_staged_executable,
        the_published_folder,
        line,
        &kept,
        told,
        on,
        every,
    );
    if let Some(sentence) = the_failing_steps_own_sentence(&came) {
        let starts_a_line = if kept.the_error_stream_ends_mid_line() { "\n" } else { "" };
        on.printed(
            steps::TheStepsStream::TheErrors,
            format!("{starts_a_line}{sentence}\n").as_bytes(),
        );
    }
    came.what_its_commands_printed = kept.what_they_printed();
    came
}

/// **`Server Setup` Y7a item 2: the sentence a failing step prints as its last
/// line on the error stream**, where it has one to print.
///
/// **It is S2a's composed text and nothing else**: the supervisor writes a
/// failed step's words as the machine's own words where any were captured,
/// and Smoozy's own sentence otherwise. So the sentence is printed exactly
/// where no command's words stand in for it — **a step that ran to its end,
/// one held for a person, and one whose S2a text is a command's printed words
/// print nothing here**, the last because those words are already on the
/// streams. Added 2026-10-25, the eighteenth notes, piece 2b (note 3's missing
/// sentence in *Server output*).
pub fn the_failing_steps_own_sentence(came: &WhatTheStepCameTo) -> Option<&str> {
    if came.it_ran_to_its_end || came.held_for_a_person || came.the_machines_own_words.is_some() {
        return None;
    }
    came.why_not.as_deref().filter(|why| !why.is_empty())
}

/// **Which act a step's progress lines are of.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TheActInProgress {
    /// `Server Setup` Y7b: the fetch, counting the bytes staged.
    TheDownload,
    /// `Server Setup` Y7c: the unpack, counting the archive's bytes fed.
    TheUnpack,
}

/// **`Server Setup` Y7b and Y7c: the worker's own progress lines, printed on
/// the step's ordinary stream the way a terminal downloader prints them.**
/// Added 2026-10-25, the eighteenth notes, piece 2b — note 5: *"I refuse to
/// believe that downloading Blender, for example, is completely silent in the
/// terminal output."*
///
/// **The words are `Design/Server Setup § Nothing is silent while a setup
/// runs`'s**: *blender-5.2.2-linux-x64.tar.xz: 214 of 352 MB, 18.4 MB/s*,
/// ending with *blender-5.2.2-linux-x64.tar.xz: 352 MB downloaded in 19 s*;
/// the amount and the speed alone where the download gave no size; and
/// *blender-5.2.2-linux-x64.tar.xz: 41% unpacked*. **They are the server's
/// words and never the desktop's figure** (Y7a item 3), which goes on being
/// told to `told` beside them exactly as before.
///
/// **No more often than `every`**, and the first line only once `every` has
/// passed since the first count, so its speed is a reading rather than a
/// guess. **Megabytes are a million bytes, rounded down**, the way the step's
/// note counts them, so a line never says more has arrived than has; the speed
/// is over the time since the last line, to one decimal. **Each line ends with
/// a line feed**, so each is a line of its own in the window and in *Server
/// output*.
pub struct TheProgressLines<'a> {
    the_archive: String,
    act: TheActInProgress,
    every: std::time::Duration,
    on: &'a dyn steps::HandsOnWhatIsPrinted,
    told: &'a dyn crate::acting::TellsHowFarItHasGot,
    started: std::time::Instant,
    /// When printing the last line finished, or the first count came, and
    /// the count then.
    the_last_line: std::cell::Cell<Option<(std::time::Instant, u64)>>,
    /// The newest count told, with its whole.
    the_newest: std::cell::Cell<Option<(u64, Option<u64>)>>,
    /// The share the last unpack line said.
    the_last_share: std::cell::Cell<Option<u128>>,
}

impl<'a> TheProgressLines<'a> {
    pub fn of(
        act: TheActInProgress,
        the_archive: &str,
        every: std::time::Duration,
        on: &'a dyn steps::HandsOnWhatIsPrinted,
        told: &'a dyn crate::acting::TellsHowFarItHasGot,
    ) -> TheProgressLines<'a> {
        TheProgressLines {
            the_archive: the_archive.to_string(),
            act,
            every,
            on,
            told,
            started: std::time::Instant::now(),
            the_last_line: std::cell::Cell::new(None),
            the_newest: std::cell::Cell::new(None),
            the_last_share: std::cell::Cell::new(None),
        }
    }

    fn a_line(&self, line: String) {
        self.on.printed(steps::TheStepsStream::TheOrdinary, format!("{line}\n").as_bytes());
    }

    /// **The unpack's line for a count**, where the whole is known and is not
    /// nothing: a share of an unknown size says nothing.
    fn the_unpacks_line(&self, bytes: u64, whole: Option<u64>) -> Option<(u128, String)> {
        let whole = whole.filter(|whole| *whole > 0)?;
        let share = (u128::from(bytes) * 100 / u128::from(whole)).min(100);
        Some((share, format!("{}: {share}% unpacked", self.the_archive)))
    }

    /// **Y7b's *"once more when the fetch ends"***: the whole that was
    /// staged, and how long the fetch took. Called only where the whole
    /// archive arrived.
    pub fn the_download_ended(&self) {
        let Some((bytes, _)) = self.the_newest.get() else {
            return;
        };
        self.a_line(format!(
            "{}: {} MB downloaded in {} s",
            self.the_archive,
            bytes / 1_000_000,
            self.started.elapsed().as_secs_f64().round() as u64
        ));
    }

    /// **The unpack's last count, said once more as it ends**, where the last
    /// line said a different share — the same way as the fetch's (Y7c: *"the
    /// same way as Y7b's"*). Called only where the unpack tool ended cleanly.
    pub fn the_unpack_ended(&self) {
        let Some((bytes, whole)) = self.the_newest.get() else {
            return;
        };
        if let Some((share, line)) = self.the_unpacks_line(bytes, whole) {
            if self.the_last_share.get() != Some(share) {
                self.the_last_share.set(Some(share));
                self.a_line(line);
            }
        }
    }
}

impl crate::acting::TellsHowFarItHasGot for TheProgressLines<'_> {
    fn so_far(&self, bytes: u64, whole: Option<u64>) {
        self.told.so_far(bytes, whole);
        self.the_newest.set(Some((bytes, whole)));
        let now = std::time::Instant::now();
        let Some((then, bytes_then)) = self.the_last_line.get() else {
            self.the_last_line.set(Some((now, bytes)));
            return;
        };
        let since = now.duration_since(then);
        if since < self.every || since.is_zero() {
            return;
        }
        match self.act {
            TheActInProgress::TheDownload => {
                let rate = bytes.saturating_sub(bytes_then) as f64 / 1_000_000.0 / since.as_secs_f64();
                let so_far = bytes / 1_000_000;
                self.a_line(match whole {
                    Some(whole) => format!(
                        "{}: {so_far} of {} MB, {rate:.1} MB/s",
                        self.the_archive,
                        whole / 1_000_000
                    ),
                    None => format!("{}: {so_far} MB, {rate:.1} MB/s", self.the_archive),
                });
            }
            TheActInProgress::TheUnpack => {
                if let Some((share, line)) = self.the_unpacks_line(bytes, whole) {
                    self.the_last_share.set(Some(share));
                    self.a_line(line);
                }
            }
        }
        // Printing can block. The next interval starts after this receiver
        // returns, so a slow print cannot spend the next line's waiting time.
        self.the_last_line.set(Some((std::time::Instant::now(), bytes)));
    }
}

/// **The archive a step's progress lines name**: the build service's own file
/// name where the step carried one, the user's own archive where the run
/// installs one, and otherwise the release's file name B4 derives from the
/// version.
fn the_archive_the_lines_name(inputs: &TheStepsInputs) -> Option<String> {
    if let Some(name) = &inputs.the_users_own_archive {
        return Some(name.clone());
    }
    if let Some(name) = &inputs.the_build_services_file {
        return Some(name.clone());
    }
    inputs.version.map(fetching::the_archives_file_name)
}

/// **[`perform_one_step_telling`]'s step, on the runner that keeps every
/// command's words.**
#[allow(clippy::too_many_arguments)]
fn the_step_is_performed(
    step: WhichStep,
    the_marker: Option<Boundary>,
    the_composed_list: &[Boundary],
    inputs: &TheStepsInputs,
    works: &WhereTheRunWorks,
    the_run: &str,
    the_staged_executable: Option<&Path>,
    the_published_folder: Option<&Path>,
    line: &dyn MakesTheRequestOnTheMachinesOwnLine,
    runner: &dyn RunsOneCommandOnTheMachine,
    told: &dyn crate::acting::TellsHowFarItHasGot,
    on: &dyn steps::HandsOnWhatIsPrinted,
    every: std::time::Duration,
) -> WhatTheStepCameTo {
    if !the_worker_performs(step) {
        return WhatTheStepCameTo::did_not(format!(
            "the setup worker does not perform {}",
            step.as_asked()
        ));
    }
    if !may_this_step_be_performed(the_marker, step) {
        return WhatTheStepCameTo::did_not(format!(
            "the run's marker is past {}, and S4 item 3 does not run a step again",
            step.as_asked()
        ));
    }
    match step {
        WhichStep::TheReadings | WhichStep::TheTelegramMaterial => {
            WhatTheStepCameTo::did_not("the setup worker does not perform that step".to_string())
        }
        // **`Server Setup` B7: a run installing the user's own archive fetches
        // nothing**, and a fetch commanded on it is refused by name rather
        // than reaching Blender's downloads. Added 2026-10-08.
        WhichStep::TheFetch if inputs.the_users_own_archive.is_some() => WhatTheStepCameTo::did_not(
            "this run installs the user's own archive, which is already on the machine, and \
             fetches nothing"
                .to_string(),
        ),
        WhichStep::TheFetch => {
            let Some(version) = inputs.version else {
                return WhatTheStepCameTo::did_not(
                    "the fetch was commanded with no version to derive its address from".to_string(),
                );
            };
            let staged_at = fetching::where_the_archive_is_staged(&works.staging, the_run, version);
            let asked = crate::blender::TheVersionAsked {
                version,
                the_build_services_file: inputs.the_build_services_file.clone(),
            };
            // **Y7b: the worker's own progress lines beside its count.**
            let named = the_archive_the_lines_name(inputs).unwrap_or_default();
            let lines = TheProgressLines::of(TheActInProgress::TheDownload, &named, every, on, told);
            let fetched = fetching::the_build_is_fetched_telling(line, &staged_at, &asked, &lines);
            // **Once more when the fetch ends**, where the whole archive
            // arrived — whether or not its digest then matched.
            if matches!(fetched, Ok(_) | Err(TheFetchDidNot::TheDigestDidNotMatch { .. })) {
                lines.the_download_ended();
            }
            match fetched {
                Ok(_) => WhatTheStepCameTo::ran(step, true),
                Err(why) => WhatTheStepCameTo::did_not(the_fetchs_words(&why)),
            }
        }
        WhichStep::TheUnpack => {
            // **`Server Setup` B7: the user's own archive, where the run
            // carries one**, in place of the archive a fetch staged.
            let staged_at = match (&inputs.the_users_own_archive, inputs.version) {
                (Some(name), _) => match where_the_users_archive_is_staged(&works.staging, the_run, name) {
                    Ok(at) => at,
                    Err(why) => return WhatTheStepCameTo::did_not(why),
                },
                (None, Some(version)) => {
                    fetching::where_the_archive_is_staged(&works.staging, the_run, version)
                }
                (None, None) => {
                    return WhatTheStepCameTo::did_not(
                        "the unpack was commanded with no version to name its archive".to_string(),
                    )
                }
            };
            let unpacked_into = where_the_archive_is_unpacked(&works.staging, the_run);
            // **Y7c: the worker's own progress lines beside its count.**
            let named = the_archive_the_lines_name(inputs).unwrap_or_default();
            let lines = TheProgressLines::of(TheActInProgress::TheUnpack, &named, every, on, told);
            let unpacked = steps::the_archive_is_unpacked(runner, &staged_at, &unpacked_into, &lines);
            if unpacked.as_ref().is_ok_and(|went| went.it_exited_cleanly()) {
                lines.the_unpack_ended();
            }
            match unpacked {
                Err(why) => WhatTheStepCameTo::did_not(why),
                Ok(went) if !went.it_exited_cleanly() => {
                    let mut came = WhatTheStepCameTo::did_not(format!(
                        "The unpack tool exited {}.",
                        went.exit
                            .map(|it| it.to_string())
                            .unwrap_or_else(|| "with no status".to_string())
                    ));
                    came.the_machines_own_words = the_words(&went.ordinary, &went.error);
                    came
                }
                Ok(went) => match steps::the_staged_executable(&unpacked_into) {
                    Err(why) => {
                        let mut came = WhatTheStepCameTo::did_not(why);
                        came.the_machines_own_words = the_words(&went.ordinary, &went.error);
                        came
                    }
                    Ok(executable) => {
                        let mut came = WhatTheStepCameTo::ran(step, true);
                        came.the_machines_own_words = the_words(&went.ordinary, &went.error);
                        came.the_staged_executable = Some(executable);
                        came
                    }
                },
            }
        }
        WhichStep::TheLibraryChainIsRead | WhichStep::TheLibraryChainIsReadAgain => {
            let Some(executable) = inputs
                .executable
                .as_deref()
                .or(the_staged_executable)
            else {
                return WhatTheStepCameTo::did_not(
                    "a library step was commanded with no build to read against, and this run \
                     staged none"
                        .to_string(),
                );
            };
            match steps::the_library_chain_reads_clean(runner, executable) {
                Err(why) => WhatTheStepCameTo::did_not(why),
                Ok((went, not_found, clean)) => {
                    // The re-read closes the step and must come back clean;
                    // the first read is a reading and reports whatever it found.
                    let closes_the_step = step == WhichStep::TheLibraryChainIsReadAgain;
                    let mut came = if closes_the_step && !clean {
                        WhatTheStepCameTo::did_not(format!(
                            "{} system libraries still missing.",
                            not_found.len()
                        ))
                    } else {
                        // Y4a: an empty list at the first read is the step
                        // finding nothing to perform.
                        WhatTheStepCameTo::ran(step, !clean || closes_the_step)
                    };
                    came.the_machines_own_words = the_words(&went.ordinary, &went.error);
                    came.the_libraries_not_found = not_found;
                    came
                }
            }
        }
        WhichStep::TheBuildIsPublished if inputs.the_users_own_archive.is_some() => {
            the_users_build_is_published(inputs, works, the_run, the_staged_executable, runner)
        }
        WhichStep::TheBuildIsPublished => {
            let Some(version) = inputs.version else {
                return WhatTheStepCameTo::did_not(
                    "the publication was commanded with no version to require".to_string(),
                );
            };
            let Some(executable) = inputs.executable.as_deref().or(the_staged_executable) else {
                return WhatTheStepCameTo::did_not(
                    "the publication was commanded and this run staged no build".to_string(),
                );
            };
            let Some(the_run_started) = works.the_run_started else {
                return WhatTheStepCameTo::did_not(
                    "the publication was commanded with no time the run started, which its \
                     folder is named by"
                        .to_string(),
                );
            };
            let at = works
                .the_blender_area
                .join(the_published_folders_name(version, the_run_started));
            what_the_publication_came_to(step, steps::the_build_is_published(runner, executable, version, &at))
        }
        // **`Server Setup` B15: the plain deletion of the install's folder**,
        // refused for any path outside the Blender area. The 38's piece 14c.
        // **B15's second route rides the same step** where the command names
        // the manager's package: the desktop performed the removal, elevated
        // (Y7), and the step reads that the build is gone. Step 10c's owed
        // piece 6.
        WhichStep::TheBuildIsRemoved => match inputs.the_managers_package.as_deref() {
            Some(package) => the_managers_removal_is_read(step, inputs.executable.as_deref(), package),
            None => the_build_is_removed(step, inputs.executable.as_deref(), &works.the_blender_area),
        },
        WhichStep::TheFinalVerification => {
            // **The build is read where the publication put it, and the staged
            // path is never a fallback here.** R1 item 7 renames the staged
            // folder out of staging, so by this step the staged path names
            // nothing — step 10c's fifth pass on Vast watched every install
            // fail here with *No such file or directory* over a Blender that
            // was installed and working.
            let published = the_published_folder.map(steps::the_published_executable);
            let the_published_build = published.as_deref();
            // **The library half's subject is whichever build this run
            // actually has**: the one it published, or — where the machine
            // already held one and nothing was published — the one the desktop
            // named (R1 item 4's two cases).
            let read_against = the_published_build.or(inputs.executable.as_deref());
            match steps::the_final_verification(runner, read_against, the_published_build) {
                Err(why) => WhatTheStepCameTo::did_not(why),
                Ok(found) => match found.what_it_claims() {
                    WhatTheVerificationClaims::ItPassed => WhatTheStepCameTo::ran(step, true),
                    // **What did not pass, in the words a person reads** —
                    // never the two readings written out the way this language
                    // renders a value for a programmer, which is what step
                    // 10c's fourth pass on Vast showed the user.
                    WhatTheVerificationClaims::ItDidNotPass => {
                        WhatTheStepCameTo::did_not(found.what_did_not_pass())
                    }
                    // **R4: an absent half claims nothing — and a run that
                    // published a Blender had a subject.** Where the run's
                    // list and its marker say it published one and nothing
                    // carried where, the step says so rather than passing a
                    // check of nothing. Step 10c's fifth pass on Vast watched
                    // the other answer: a recovery re-ran this step with both
                    // halves absent and the screen said *The server is ready
                    // to render*.
                    WhatTheVerificationClaims::NothingWasRead
                        if the_run_published_a_blender(the_composed_list, the_marker) =>
                    {
                        WhatTheStepCameTo::did_not(
                            "the final check was commanded over a Blender this run published, \
                             and nothing said where it was published, so no build was read"
                                .to_string(),
                        )
                    }
                    // **A run with no Blender in it rightly reads nothing**,
                    // and Y4a's mark is what records that: the step found
                    // nothing to check rather than checking and passing.
                    WhatTheVerificationClaims::NothingWasRead => {
                        WhatTheStepCameTo::ran(step, false)
                    }
                },
            }
        }
    }
}

/// **`Server Setup` B15's plain deletion, performed**: the folder directly
/// under `Server Setup` B9's Blender area that holds the executable goes,
/// whole. The 38's piece 14c.
///
/// **Only a folder of Smoozy's own is ever deleted.** An executable that is
/// not under the Blender area — the system package areas, the snap mount, a
/// folder unpacked by hand anywhere else — is refused by name: the software
/// manager's route is not this step, and nothing outside Smoozy's own area is
/// Smoozy's to delete. A path with a `..` in it is refused too, so nothing
/// outside the area is reached by walking out of it.
///
/// **A folder already gone is nothing to do**, which is what makes a re-run
/// safe (`Server Setup` S7): the boundary is reached and Y4a's mark says the
/// step found nothing to delete.
pub fn the_build_is_removed(
    step: WhichStep,
    executable: Option<&Path>,
    the_blender_area: &Path,
) -> WhatTheStepCameTo {
    let Some(executable) = executable else {
        return WhatTheStepCameTo::did_not(
            "the removal was commanded with no executable, so nothing was removed".to_string(),
        );
    };
    let outside = || {
        WhatTheStepCameTo::did_not(format!(
            "{} is outside Smoozy's Blender folder ({}). Not removed.",
            executable.display(),
            the_blender_area.display()
        ))
    };
    if executable
        .components()
        .any(|part| matches!(part, std::path::Component::ParentDir | std::path::Component::CurDir))
    {
        return outside();
    }
    let Ok(under) = executable.strip_prefix(the_blender_area) else {
        return outside();
    };
    let mut parts = under.components();
    let (Some(std::path::Component::Normal(folder)), Some(_)) = (parts.next(), parts.next()) else {
        return outside();
    };
    let the_install = the_blender_area.join(folder);
    match std::fs::symlink_metadata(&the_install) {
        Err(why) if why.kind() == std::io::ErrorKind::NotFound => WhatTheStepCameTo::ran(step, false),
        Err(why) => WhatTheStepCameTo::did_not(format!("{}: {why}", the_install.display())),
        Ok(_) => match std::fs::remove_dir_all(&the_install) {
            Ok(()) => WhatTheStepCameTo::ran(step, true),
            Err(why) if why.kind() == std::io::ErrorKind::NotFound => {
                WhatTheStepCameTo::ran(step, false)
            }
            Err(why) => WhatTheStepCameTo::did_not(format!("{}: {why}", the_install.display())),
        },
    }
}

/// **B15's second route, the machine's half: the manager's removal read, and
/// nothing deleted.** Added 2026-10-16, step 10c's owed piece 6.
///
/// **The removal itself is the desktop's**: the manager's removal is an
/// elevated act and E2's flow applies (B15), and Y7 puts every elevated act on
/// the SSH command channel, the password on the elevating command's standard
/// input (E3). **What is left here is the record**: the step refuses a package
/// or a path outside the fixed pairing (§13 item 21,
/// [`crate::blender::a_managers_removal_may_be_recorded`]), then reads that
/// the executable is gone. **A build still there is the step failing**, in
/// words — the manager's exit is never the evidence, as R7a says of the
/// hand-over. A build gone is the boundary, and the supervisor's marks ride it
/// as they ride the plain deletion (`Queue & History` L21).
pub fn the_managers_removal_is_read(
    step: WhichStep,
    executable: Option<&Path>,
    package: &str,
) -> WhatTheStepCameTo {
    let Some(executable) = executable else {
        return WhatTheStepCameTo::did_not(
            "the removal was commanded with no executable, so nothing was recorded".to_string(),
        );
    };
    let Some(path) = executable.to_str() else {
        return WhatTheStepCameTo::did_not(format!(
            "{} is not managed by the package manager.",
            executable.display()
        ));
    };
    if let Err(why) = crate::blender::a_managers_removal_may_be_recorded(package, path) {
        return WhatTheStepCameTo::did_not(why);
    }
    the_build_is_read_gone(step, executable, package)
}

/// **Whether the build is gone, read off the disk**: gone is the step ending
/// performed, still there is the step failing in words, and any other answer is
/// the machine's own.
fn the_build_is_read_gone(step: WhichStep, at: &Path, package: &str) -> WhatTheStepCameTo {
    match std::fs::symlink_metadata(at) {
        Err(why) if why.kind() == std::io::ErrorKind::NotFound => WhatTheStepCameTo::ran(step, true),
        Err(why) => WhatTheStepCameTo::did_not(format!("{}: {why}", at.display())),
        Ok(_) => WhatTheStepCameTo::did_not(format!(
            "The package manager was asked to remove {package}, but {} is still there.",
            at.display()
        )),
    }
}

/// **`Server Setup` B7: where the user's own archive sits** — in this run's
/// own staging area, under the run's name like every other piece of its
/// material, so the sweep's and a recovery's reading of the area holds for it
/// too. **The runner's ruling on piece 14 (2026-10-08)**: the run is opened
/// first and the upload lands here.
///
/// **A name that is not bare is refused** — nothing outside the staging area is
/// ever unpacked, and no path the desktop sends is joined on as it came.
pub fn where_the_users_archive_is_staged(
    staging_area: &Path,
    run: &str,
    name: &str,
) -> Result<PathBuf, String> {
    let bare = !name.is_empty()
        && name != "."
        && name != ".."
        && !name.contains('/')
        && !name.contains('\\')
        && !name.contains('\0');
    if !bare {
        return Err(format!(
            "{name:?} is not a plain file name."
        ));
    }
    Ok(staging_area.join(format!("{run}-{name}")))
}

/// **`Server Setup` B7 clause 3, B7a and B8 on the user's own build: asked its
/// version in staging, held below 4.5 for a person, and published under the
/// version it answered.**
///
/// **A build the user picked has no version until it is unpacked and asked**,
/// so this route asks first and names the folder by the answer
/// (`Server Setup` B8). The publication itself is the listing's route's, which
/// asks again before the rename — the order R1 item 7 makes the contract.
///
/// **B7a's three answers.** At or above 4.5 the build is published. Below it,
/// with no answer from the user, the step is held: nothing is renamed and the
/// build stays in staging at no final name. With *Install anyway* it is
/// published as at or above. **With a decline the staged material is
/// discarded** — never published, never repaired in place — and the discard
/// is reported in Smoozy's own words; the run then goes on to *done*.
fn the_users_build_is_published(
    inputs: &TheStepsInputs,
    works: &WhereTheRunWorks,
    the_run: &str,
    the_staged_executable: Option<&Path>,
    runner: &dyn RunsOneCommandOnTheMachine,
) -> WhatTheStepCameTo {
    let step = WhichStep::TheBuildIsPublished;
    if inputs.the_users_answer == Some(WhatTheUserAnswered::Decline) {
        return the_users_build_is_discarded(inputs, works, the_run);
    }
    let Some(executable) = inputs.executable.as_deref().or(the_staged_executable) else {
        return WhatTheStepCameTo::did_not(
            "the publication was commanded and this run staged no build".to_string(),
        );
    };
    let Some(the_run_started) = works.the_run_started else {
        return WhatTheStepCameTo::did_not(
            "the publication was commanded with no time the run started, which its folder is \
             named by"
                .to_string(),
        );
    };
    // `Server Setup` B1: the `Blender ` line, both streams, under the limit.
    let went = match runner.run(
        &executable.display().to_string(),
        &[steps::THE_VERSION_SWITCH],
        Some(THE_TIME_LIMIT_EVERY_BLENDER_RUN_CARRIES),
    ) {
        Ok(went) => went,
        Err(why) => return WhatTheStepCameTo::did_not(why),
    };
    let answered = if went.it_was_killed_at_its_limit {
        None
    } else {
        read_the_version_answer(&went.ordinary).or_else(|| read_the_version_answer(&went.error))
    };
    let Some(answered) = answered else {
        let mut came =
            WhatTheStepCameTo::did_not("The build did not report a version.".to_string());
        came.the_machines_own_words = the_words(&went.ordinary, &went.error);
        return came;
    };
    if what_a_version_answer_does(answered) == WhatTheBelowFloorAnswerDoes::TheRunHoldsForAHumanAnswer
        && inputs.the_users_answer != Some(WhatTheUserAnswered::InstallAnyway)
    {
        let mut came = WhatTheStepCameTo::did_not(format!(
            "The build is Blender {}.{}.{}, below 4.5. Waiting for your answer.",
            answered.major, answered.minor, answered.patch
        ));
        came.held_for_a_person = true;
        came.the_version_answered = Some(answered);
        return came;
    }
    let at = works
        .the_blender_area
        .join(the_published_folders_name(answered, the_run_started));
    what_the_publication_came_to(
        step,
        steps::the_build_is_published(runner, executable, answered, &at),
    )
}

/// **`Server Setup` B7a's decline, performed**: the staged archive and its
/// unpacked folder are removed, and the discard is reported. **The step runs to
/// its end and feeds no boundary** — the supervisor strikes the publication
/// from the run's composed list and records the discard in its place.
fn the_users_build_is_discarded(
    inputs: &TheStepsInputs,
    works: &WhereTheRunWorks,
    the_run: &str,
) -> WhatTheStepCameTo {
    let mut not_removed = Vec::new();
    if let Some(name) = &inputs.the_users_own_archive {
        if let Ok(archive) = where_the_users_archive_is_staged(&works.staging, the_run, name) {
            if archive.exists() {
                if let Err(why) = std::fs::remove_file(&archive) {
                    not_removed.push(format!("{}: {why}", archive.display()));
                }
            }
        }
    }
    let unpacked = where_the_archive_is_unpacked(&works.staging, the_run);
    if unpacked.exists() {
        if let Err(why) = std::fs::remove_dir_all(&unpacked) {
            not_removed.push(format!("{}: {why}", unpacked.display()));
        }
    }
    if !not_removed.is_empty() {
        return WhatTheStepCameTo::did_not(format!(
            "Declined, but the build could not be deleted: {}",
            not_removed.join("; ")
        ));
    }
    let mut came = WhatTheStepCameTo::ran(WhichStep::TheBuildIsPublished, true);
    came.the_boundary = None;
    came.the_discard = Some(
        "Declined: the build was deleted, not installed."
            .to_string(),
    );
    came
}

/// **What one publication came to, as the step reports it** — shared by the
/// listing's route and the user's own build's, so the two never say it
/// differently.
fn what_the_publication_came_to(
    step: WhichStep,
    published: Result<(steps::HowItWent, WhatThePublicationDid), String>,
) -> WhatTheStepCameTo {
    match published {
        Err(why) => WhatTheStepCameTo::did_not(why),
        Ok((went, did)) => {
            let mut came = match &did {
                WhatThePublicationDid::ItWasPublished { at, answered } => {
                    let mut came = WhatTheStepCameTo::ran(step, true);
                    came.published_at = Some(at.clone());
                    came.the_version_answered = Some(*answered);
                    came
                }
                WhatThePublicationDid::ItAnsweredWrongly { expected, answered } => {
                    let mut came = WhatTheStepCameTo::did_not(match answered {
                        Some(answered) => format!(
                            "The build reported {}.{}.{}; expected {}.{}.{}.",
                            answered.major,
                            answered.minor,
                            answered.patch,
                            expected.major,
                            expected.minor,
                            expected.patch
                        ),
                        None => "The build did not report a version.".to_string(),
                    });
                    came.the_version_answered = *answered;
                    came
                }
                WhatThePublicationDid::TheRenameWasRefused { why } => {
                    WhatTheStepCameTo::did_not(why.clone())
                }
            };
            came.the_machines_own_words = the_words(&went.ordinary, &went.error);
            came
        }
    }
}

/// **Where a run's archive is unpacked** — a folder in the staging area named
/// for the run the material was staged under, beside the archive
/// ([`fetching::where_the_archive_is_staged`]). Composed here once: the unpack,
/// a recovery's take-over and the sweep all read it.
pub fn where_the_archive_is_unpacked(staging_area: &Path, run: &str) -> PathBuf {
    staging_area.join(format!("{run}-unpacked"))
}

/// **What the sweep of one completed run's staging did, per path.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WhatTheSweepDid {
    /// It was there and it is gone.
    Removed(PathBuf),
    /// Nothing was at the path — already gone, or never fetched because the
    /// machine held the Blender already. Nothing is said about why.
    NothingWasThere(PathBuf),
    /// It was there and could not be removed, in the machine's own words.
    NotRemoved { at: PathBuf, why: String },
}

/// **`Server Setup` §11's cleanup and `Rented-server runtime` G20's bounded
/// shape, for a run that has ended *completed*.**
///
/// §11 declares what the setup kind owes: *"the run's staged material at the
/// locations its operation record names, swept …; the fetch staging is kept
/// while its record says a resume is possible."* `Rented-server runtime` G20
/// bounds it: *"the sweep is
/// bounded to the locations the phase record names … and it reaches nothing
/// else on the machine."* **The sweep was declared and never built**, and step
/// 10c's sixth pass on Vast found what that left: the 383 MB archive in the
/// staging area after a completed install, beside an empty unpacked folder —
/// 2 per cent of a 20 GB rental disk, kept for the life of the machine.
///
/// **Exactly two paths, both derived from the record and never found by
/// looking**: the archive, named by the run the material was staged under and
/// the version the record carries ([`fetching::where_the_archive_is_staged`]),
/// and the folder it was unpacked into ([`where_the_archive_is_unpacked`]). No
/// directory is listed and nothing else in the staging area is touched
/// (`Server protocol and transfers` S29: presence is never ownership).
///
/// **Only a completed run is swept**, because a completed run's record is the
/// one that says no resume is possible: its build is published and nothing
/// reads the staged material again. A run that failed or was stopped keeps its
/// material, because a recovery takes it over (`Server Setup` S4). **The bound
/// on what this can remove**: the two paths carry the run's own issued
/// identity, so nothing another run staged can be named by them; the record
/// carries no file identity to compare as well (`Server protocol and
/// transfers` S28's third check), which is the one part of that clause's shape
/// this does not have.
pub fn the_completed_runs_staging_is_swept(
    staging_area: &Path,
    staged_under: &str,
    version: Version,
) -> Vec<WhatTheSweepDid> {
    let archive = fetching::where_the_archive_is_staged(staging_area, staged_under, version);
    let unpacked = where_the_archive_is_unpacked(staging_area, staged_under);
    let mut did = Vec::new();
    for (at, it_is_a_folder) in [(archive, false), (unpacked, true)] {
        let removed = if it_is_a_folder {
            std::fs::remove_dir_all(&at)
        } else {
            std::fs::remove_file(&at)
        };
        did.push(match removed {
            Ok(()) => WhatTheSweepDid::Removed(at),
            Err(why) if why.kind() == std::io::ErrorKind::NotFound => {
                WhatTheSweepDid::NothingWasThere(at)
            }
            Err(why) => WhatTheSweepDid::NotRemoved {
                why: why.to_string(),
                at,
            },
        });
    }
    did
}

/// **B8: the folder's name is the answered version plus the setup run's start,
/// for a human reader, and nothing anywhere parses it back.**
///
/// *"What a machine holds is read by looking (`Server Setup` B10), never from
/// names."* **The one place it is composed** is [`crate::blender::the_folders_name`];
/// the publication and a recovery's take-over both reach it here, from the
/// run's recorded start, so both compose the same name (2026-10-08).
pub fn the_published_folders_name(version: Version, the_run_started: std::time::SystemTime) -> String {
    crate::blender::the_folders_name(version, the_run_started)
}

/// **S2a: verbatim, whole, the streams kept apart.**
///
/// **Never merged and never mined for a cause.** They are written one after the
/// other with a line naming which is which, so a reader of the record can still
/// tell them apart, and neither is trimmed.
fn the_words(ordinary: &str, error: &str) -> Option<String> {
    if ordinary.is_empty() && error.is_empty() {
        return None;
    }
    Some(format!(
        "what it printed:\n{ordinary}\nwhat it printed on its error stream:\n{error}"
    ))
}

/// **The fetch's own refusals, in Smoozy's words**, because each is a
/// verification of Smoozy's own rather than a tool's printing.
fn the_fetchs_words(why: &TheFetchDidNot) -> String {
    match why {
        TheFetchDidNot::TheChecksumWasNotPublished { why } => {
            format!("Could not read the published checksum: {why}")
        }
        TheFetchDidNot::TheArchiveWasNotThere { status } => format!(
            "Download failed: HTTP {status}."
        ),
        TheFetchDidNot::ItWasRefused { why } => why.clone(),
        TheFetchDidNot::TheDigestDidNotMatch {
            published,
            what_arrived,
            the_staged_material_was_abandoned,
        } => format!(
            "Checksum mismatch: got {what_arrived}, expected {published}. The download was {}.",
            if *the_staged_material_was_abandoned {
                "deleted"
            } else {
                "kept: it could not be deleted"
            }
        ),
    }
}

#[cfg(test)]
mod tests {
    /// **A completed run's staging is swept, and only its own two paths.**
    /// Step 10c's sixth pass on Vast found the 383 MB archive and an empty
    /// unpacked folder left after a completed install.
    #[test]
    fn a_completed_runs_archive_and_unpacked_folder_are_swept_and_nothing_else() {
        use super::{the_completed_runs_staging_is_swept, where_the_archive_is_unpacked, WhatTheSweepDid};
        let area = std::env::temp_dir().join(format!("smoozy-sweep-{}", std::process::id()));
        std::fs::create_dir_all(&area).expect("the area is made");
        let version = crate::blender::Version { major: 5, minor: 2, patch: 2 };
        let archive = crate::acting::fetching::where_the_archive_is_staged(&area, "run-a", version);
        std::fs::write(&archive, b"archive").expect("the archive is there");
        let unpacked = where_the_archive_is_unpacked(&area, "run-a");
        std::fs::create_dir_all(&unpacked).expect("the unpacked folder is there");
        // **Another run's material beside it**, which no path of run-a's names.
        let other = crate::acting::fetching::where_the_archive_is_staged(&area, "run-b", version);
        std::fs::write(&other, b"other").expect("another run's archive");

        let did = the_completed_runs_staging_is_swept(&area, "run-a", version);
        assert_eq!(
            did,
            vec![
                WhatTheSweepDid::Removed(archive.clone()),
                WhatTheSweepDid::Removed(unpacked.clone())
            ]
        );
        assert!(!archive.exists() && !unpacked.exists());
        assert!(other.exists(), "another run's material is not reached");
        // **A second sweep finds nothing and says so**, never an error.
        assert!(the_completed_runs_staging_is_swept(&area, "run-a", version)
            .iter()
            .all(|one| matches!(one, WhatTheSweepDid::NothingWasThere(_))));
        std::fs::remove_dir_all(&area).ok();
    }

    use super::*;
    use crate::acting::steps::testing::{
        it_went, AStandInForTheMachinesTools, AStandInThatAnswersOnlyForAPathThatIsThere,
    };

    struct NoLineAtAll;

    impl MakesTheRequestOnTheMachinesOwnLine for NoLineAtAll {
        fn fetch_into(
            &self,
            _address: &str,
            _from_byte: u64,
            _into: &mut dyn crate::acting::fetching::TheBytesLand,
        ) -> Result<crate::acting::fetching::WhatTheLineGave, String> {
            Err("this test opens no line".to_string())
        }
    }

    /// **A Blender install's composed list** (`Server Setup` Y6b's own kind:
    /// R1's readings, fetch, tooling, libraries, publication and *done*), which
    /// every step below that is not about the list is performed under.
    const A_BLENDER_INSTALLS_LIST: [Boundary; 6] = [
        Boundary::Started,
        Boundary::Fetched,
        Boundary::Tooling,
        Boundary::Libraries,
        Boundary::Blender,
        Boundary::Done,
    ];

    fn works() -> WhereTheRunWorks {
        WhereTheRunWorks {
            staging: PathBuf::from("/staging"),
            the_blender_area: PathBuf::from("/root/blender"),
            the_run_started: None,
        }
    }

    /// **`Server Setup` B4's second route reaches the fetch from the step's
    /// inputs**: a fetch commanded with the build service's bare name asks the
    /// build service, at the archive's own address with `.sha256` after it,
    /// and one commanded with the version alone asks Blender's release folders
    /// — the Creator's notes 5 and 7 on 0.1.7.
    #[test]
    fn the_fetch_asks_the_route_its_inputs_name() {
        struct TheAddressesAsked(std::cell::RefCell<Vec<String>>);
        impl MakesTheRequestOnTheMachinesOwnLine for TheAddressesAsked {
            fn fetch_into(
                &self,
                address: &str,
                _from_byte: u64,
                _into: &mut dyn crate::acting::fetching::TheBytesLand,
            ) -> Result<crate::acting::fetching::WhatTheLineGave, String> {
                self.0.borrow_mut().push(address.to_string());
                Err("this test answers nothing".to_string())
            }
        }
        let alpha = "blender-5.3.0-alpha+main.64d181010e0d-linux.x86_64-release.tar.xz";
        let asked = |the_build_services_file: Option<&str>| {
            let line = TheAddressesAsked(std::cell::RefCell::new(Vec::new()));
            let came = perform_one_step(
                WhichStep::TheFetch,
                Some(Boundary::Started),
                &A_BLENDER_INSTALLS_LIST,
                &TheStepsInputs {
                    version: Some(Version { major: 5, minor: 3, patch: 0 }),
                    the_build_services_file: the_build_services_file.map(str::to_string),
                    executable: None,
                    the_users_own_archive: None,
                    the_users_answer: None,
                    the_managers_package: None,
                },
                &works(),
                "run-a",
                None,
                None,
                &line,
                &AStandInForTheMachinesTools::holding(vec![]),
            );
            assert!(!came.it_ran_to_its_end);
            line.0.into_inner()
        };
        assert_eq!(
            asked(Some(alpha)),
            vec![format!("{}{alpha}.sha256", crate::blender::THE_BUILD_SERVICES_DOWNLOADS)]
        );
        assert_eq!(
            asked(None),
            vec!["https://download.blender.org/release/Blender5.3/blender-5.3.0.sha256".to_string()]
        );
    }

    /// **`Server Setup` B15: the removal deletes the install's folder under
    /// the Blender area, finds nothing to do where it is gone, and refuses any
    /// path outside the area.** The 38's piece 14c.
    #[test]
    fn the_removal_deletes_the_install_folder_and_nothing_outside_the_area() {
        let at = std::env::temp_dir().join(format!("smoozy-removal-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&at);
        let area = at.join("blender");
        let install = area.join("5.1.0-1790000000");
        std::fs::create_dir_all(install.join("lib")).unwrap();
        std::fs::write(install.join("blender"), b"not a real blender").unwrap();
        let beside = at.join("elsewhere");
        std::fs::create_dir_all(&beside).unwrap();
        std::fs::write(beside.join("blender"), b"someone else's").unwrap();
        let step = WhichStep::TheBuildIsRemoved;

        let came = the_build_is_removed(step, Some(&install.join("blender")), &area);
        assert!(came.it_ran_to_its_end && came.it_performed, "{came:?}");
        assert_eq!(came.the_boundary, Some(Boundary::Removed));
        assert!(!install.exists(), "the install's folder stands");
        assert!(area.exists(), "the Blender area itself went");

        let again = the_build_is_removed(step, Some(&install.join("blender")), &area);
        assert!(again.it_ran_to_its_end && !again.it_performed, "a re-run is not nothing to do");

        for outside in [
            beside.join("blender"),
            area.join("..").join("elsewhere").join("blender"),
            area.clone(),
            area.join("blender"),
            PathBuf::from("/snap/blender/current/blender"),
        ] {
            let refused = the_build_is_removed(step, Some(&outside), &area);
            assert!(!refused.it_ran_to_its_end, "{} was taken", outside.display());
        }
        assert!(beside.join("blender").exists(), "a file outside the area went");
        assert!(!the_build_is_removed(step, None, &area).it_ran_to_its_end);
        let _ = std::fs::remove_dir_all(&at);
    }

    /// **B15's second route, the machine's half (step 10c's owed piece 6):
    /// the manager's removal is read and nothing is deleted.** A package
    /// other than the fixed one, or a path that package does not hold, is
    /// refused; a build gone is the boundary; a build still there is the step
    /// failing, whatever the manager's exit said.
    #[test]
    fn the_managers_removal_is_read_for_the_fixed_package_alone_and_deletes_nothing() {
        let step = WhichStep::TheBuildIsRemoved;
        // **The snap's own launcher is not on this desk**, so its removal reads
        // as done and the step reaches its boundary.
        let gone = the_managers_removal_is_read(step, Some(Path::new("/snap/bin/blender")), "blender");
        assert!(gone.it_ran_to_its_end && gone.it_performed, "{gone:?}");
        assert_eq!(gone.the_boundary, Some(Boundary::Removed));
        // **Any other package, and any path the package does not hold, is
        // refused** before anything is read.
        for (path, package) in [
            ("/snap/bin/blender", "blender-bin"),
            ("/usr/bin/blender", "blender"),
            ("/snap/blender/../bin/blender", "blender"),
            ("/opt/blender/blender", "blender"),
        ] {
            let refused = the_managers_removal_is_read(step, Some(Path::new(path)), package);
            assert!(!refused.it_ran_to_its_end, "{path} under {package} was read as removed");
        }
        assert!(!the_managers_removal_is_read(step, None, "blender").it_ran_to_its_end);
        // **A build still there is the step failing**, and nothing is deleted.
        let at = std::env::temp_dir().join(format!("smoozy-managed-{}", std::process::id()));
        std::fs::write(&at, b"a build the manager left").unwrap();
        let left = the_build_is_read_gone(step, &at, "blender");
        assert!(!left.it_ran_to_its_end, "{left:?}");
        assert!(at.exists(), "the step deleted what it was only to read");
        std::fs::remove_file(&at).ok();
        assert!(the_build_is_read_gone(step, &at, "blender").it_ran_to_its_end);
    }

    /// **Every wire word is `smoozy-framing`'s, and every one of that file's
    /// words is a step here.** Neither side can gain a step alone.
    /// **`Server Setup` B7: the user's archive is only ever read from this
    /// run's own staging area** — a name that is not bare is refused, so no
    /// path the desktop sends is joined on as it came. Added 2026-10-08.
    #[test]
    fn the_users_archive_is_named_bare_inside_the_runs_staging_area() {
        let area = Path::new("/staging");
        assert_eq!(
            where_the_users_archive_is_staged(area, "run-a", "my.tar.xz"),
            Ok(PathBuf::from("/staging/run-a-my.tar.xz"))
        );
        for not_bare in ["", ".", "..", "../my.tar.xz", "/etc/passwd", "a/b.tar.xz"] {
            assert!(
                where_the_users_archive_is_staged(area, "run-a", not_bare).is_err(),
                "{not_bare:?} was taken"
            );
        }
    }

    #[test]
    fn the_steps_and_their_wire_words_are_one_list() {
        let mine: Vec<&str> = WhichStep::IN_ORDER.iter().map(|it| it.as_asked()).collect();
        assert_eq!(mine, asked::EVERY_SETUP_STEP_THE_WORKER_IS_COMMANDED.to_vec());
        for word in asked::EVERY_SETUP_STEP_THE_WORKER_IS_COMMANDED {
            assert!(WhichStep::read_back(word).is_some(), "{word} reads back");
        }
        assert_eq!(WhichStep::read_back("the unpacking"), None);
    }

    /// **Every step the worker takes is taken for one of two stated reasons,
    /// and the two it does not take are refused by name.**
    ///
    /// **Exactly three are Y7's machine-side act inside a step the command
    /// channel drives** — the unpack and the two `ldd` reads — which is the
    /// granularity difference [`WhyTheWorkerTakesIt`] exists to state rather
    /// than paper over.
    #[test]
    fn every_step_the_worker_takes_is_taken_for_a_stated_reason() {
        let inside: Vec<WhichStep> = WhichStep::IN_ORDER
            .into_iter()
            .filter(|step| {
                why_the_worker_takes_it(*step)
                    == WhyTheWorkerTakesIt::Y7NamesTheActMachineSideInsideAnElevatedStep
            })
            .collect();
        assert_eq!(
            inside,
            vec![
                WhichStep::TheUnpack,
                WhichStep::TheLibraryChainIsRead,
                WhichStep::TheLibraryChainIsReadAgain
            ]
        );
        // And both refused ones are given to the setup worker by the judgement,
        // which is why they are refused by name rather than simply absent.
        for step in [WhichStep::TheReadings, WhichStep::TheTelegramMaterial] {
            assert_eq!(
                who_performs(step.its_step()),
                WhoPerformsTheStep::TheSetupWorker
            );
            assert_eq!(
                why_the_worker_takes_it(step),
                WhyTheWorkerTakesIt::TheWorkerDoesNotPerformIt
            );
        }
    }

    /// **S4 item 3: steps at or before the marker are not run again, and the
    /// libraries step's two halves are both refused by a marker of
    /// *libraries*.**
    #[test]
    fn a_recovery_never_runs_a_step_at_or_before_the_marker() {
        assert!(may_this_step_be_performed(None, WhichStep::TheFetch));
        assert!(!may_this_step_be_performed(
            Some(Boundary::Fetched),
            WhichStep::TheFetch
        ));
        assert!(may_this_step_be_performed(
            Some(Boundary::Fetched),
            WhichStep::TheUnpack
        ));
        for half in [
            WhichStep::TheLibraryChainIsRead,
            WhichStep::TheLibraryChainIsReadAgain,
        ] {
            assert!(
                !may_this_step_be_performed(Some(Boundary::Libraries), half),
                "{half:?} is at the marker"
            );
        }
        assert!(may_this_step_be_performed(
            Some(Boundary::Libraries),
            WhichStep::TheBuildIsPublished
        ));
    }

    /// **And the act refuses it too**, rather than leaving the rule to
    /// whichever caller remembers it.
    #[test]
    fn the_act_itself_refuses_a_step_the_marker_is_past() {
        let runner = AStandInForTheMachinesTools::holding(vec![]);
        let came = perform_one_step(
            WhichStep::TheFetch,
            Some(Boundary::Fetched),
            &A_BLENDER_INSTALLS_LIST,
            &TheStepsInputs::default(),
            &works(),
            "a-run",
            None,
            None,
            &NoLineAtAll,
            &runner,
        );
        assert!(!came.it_ran_to_its_end);
        assert!(came.why_not.is_some_and(|why| why.contains("S4 item 3")));
        assert!(runner.asked.borrow().is_empty(), "nothing was run");
    }

    /// **The two the worker does not perform are refused by name and run
    /// nothing.**
    #[test]
    fn the_two_steps_the_worker_does_not_perform_are_refused_by_name() {
        for step in [WhichStep::TheReadings, WhichStep::TheTelegramMaterial] {
            let runner = AStandInForTheMachinesTools::holding(vec![]);
            let came = perform_one_step(
                step,
                None,
                &A_BLENDER_INSTALLS_LIST,
                &TheStepsInputs::default(),
                &works(),
                "a-run",
                None,
                None,
                &NoLineAtAll,
                &runner,
            );
            assert!(!came.it_ran_to_its_end);
            assert!(came
                .why_not
                .is_some_and(|why| why.contains("does not perform")));
            assert!(runner.asked.borrow().is_empty());
        }
    }

    /// **The re-read must come back clean and the first read need not**, which
    /// is R1 item 4's own shape: the whole list is read, the desktop installs,
    /// and the re-read is the check.
    #[test]
    fn the_re_read_must_come_back_clean_and_the_first_read_reports_what_it_found() {
        let a_list = "\tlibX11.so.6 => not found\n";
        let runner = AStandInForTheMachinesTools::holding(vec![("ldd", it_went(0, a_list, ""))]);
        let first = perform_one_step(
            WhichStep::TheLibraryChainIsRead,
            None,
            &A_BLENDER_INSTALLS_LIST,
            &TheStepsInputs {
                version: None,
                the_build_services_file: None,
                executable: Some(PathBuf::from("/a/blender")),
                the_users_own_archive: None,
                the_users_answer: None,
                the_managers_package: None,
            },
            &works(),
            "a-run",
            None,
            None,
            &NoLineAtAll,
            &runner,
        );
        assert!(first.it_ran_to_its_end);
        assert_eq!(first.the_libraries_not_found, vec!["libX11.so.6".to_string()]);
        assert_eq!(first.the_boundary, None, "the install follows it");

        let runner = AStandInForTheMachinesTools::holding(vec![("ldd", it_went(0, a_list, ""))]);
        let again = perform_one_step(
            WhichStep::TheLibraryChainIsReadAgain,
            None,
            &A_BLENDER_INSTALLS_LIST,
            &TheStepsInputs {
                version: None,
                the_build_services_file: None,
                executable: Some(PathBuf::from("/a/blender")),
                the_users_own_archive: None,
                the_users_answer: None,
                the_managers_package: None,
            },
            &works(),
            "a-run",
            None,
            None,
            &NoLineAtAll,
            &runner,
        );
        assert!(!again.it_ran_to_its_end, "the re-read must come back clean");

        let runner = AStandInForTheMachinesTools::holding(vec![("ldd", it_went(0, "", ""))]);
        let clean = perform_one_step(
            WhichStep::TheLibraryChainIsReadAgain,
            None,
            &A_BLENDER_INSTALLS_LIST,
            &TheStepsInputs {
                version: None,
                the_build_services_file: None,
                executable: Some(PathBuf::from("/a/blender")),
                the_users_own_archive: None,
                the_users_answer: None,
                the_managers_package: None,
            },
            &works(),
            "a-run",
            None,
            None,
            &NoLineAtAll,
            &runner,
        );
        assert!(clean.it_ran_to_its_end);
        assert_eq!(clean.the_boundary, Some(Boundary::Libraries));
    }

    /// **Every step that reads the version is on the one list the desktop sends
    /// it by, and no other step is.** Added 2026-09-22: the third pass on Vast
    /// found the unpack reading a version the desktop never sent it, because
    /// the wire's own word for the step said it carried nothing.
    ///
    /// **Measured, not declared**: each step is performed with every input it
    /// could want except the version, and the ones that refuse for want of it
    /// must be exactly `smoozy-framing`'s list.
    #[test]
    fn the_steps_that_read_the_version_are_the_list_the_desktop_sends_it_by() {
        let wanting_the_version: Vec<&str> = WhichStep::IN_ORDER
            .into_iter()
            .filter(|step| {
                let runner = AStandInForTheMachinesTools::holding(vec![]);
                let came = perform_one_step(
                    *step,
                    None,
                    &A_BLENDER_INSTALLS_LIST,
                    &TheStepsInputs {
                        version: None,
                        the_build_services_file: None,
                        executable: Some(PathBuf::from("/a/blender")),
                        the_users_own_archive: None,
                        the_users_answer: None,
                        the_managers_package: None,
                    },
                    &works(),
                    "a-run",
                    Some(Path::new("/a/blender")),
                    None,
                    &NoLineAtAll,
                    &runner,
                );
                came.why_not
                    .is_some_and(|why| why.contains("with no version"))
            })
            .map(WhichStep::as_asked)
            .collect();
        assert_eq!(
            wanting_the_version,
            asked::THE_STEPS_THAT_READ_THE_BLENDER_VERSION.to_vec()
        );
    }

    /// **The install driven through, on a disk, with a stand-in that answers
    /// only for a path that is really there — so a stale path fails.**
    ///
    /// **This is the test step 10c's fifth pass on Vast is owed.** Every
    /// Blender install on that machine failed its last step: the publication
    /// renamed the unpacked folder out of staging, the final check ran the
    /// build at the staging path it no longer had, and the machine answered
    /// *No such file or directory* over a Blender that was installed, complete
    /// and working. **The desk never saw it because the stand-in every earlier
    /// test drove answered any path at all**, matching on the program's name
    /// alone — so the staged path and the published path were the same thing to
    /// it, and a check at the wrong one passed.
    ///
    /// **The fetch is not in this drive-through** and nothing here pretends it
    /// is: B4's fetch opens a line rather than naming a path, and it has its
    /// own tests against a real server. What is driven is every step that names
    /// a place on the machine — the unpack, both `ldd` reads, the publication,
    /// and the final check — with the archive written where the fetch would
    /// have left it and the unpacked tree where `tar` would have.
    #[test]
    fn the_final_check_reads_the_build_where_the_publication_put_it() {
        let area = crate::acting::its_own_folder("the-final-check-reads-the-published-build");
        let works = WhereTheRunWorks {
            staging: area.at().join("staging"),
            the_blender_area: area.at().join("blender"),
            the_run_started: Some(std::time::UNIX_EPOCH + std::time::Duration::from_secs(1_791_300_732)),
        };
        let version = Version {
            major: 5,
            minor: 2,
            patch: 0,
        };
        let inputs = TheStepsInputs {
            version: Some(version),
            the_build_services_file: None,
            executable: None,
            the_users_own_archive: None,
            the_users_answer: None,
            the_managers_package: None,
        };
        let run = "a-run";
        // **What the fetch and `tar` would have left**, written here because
        // neither is driven: the archive at its staging path, and the tree the
        // archive unpacks into.
        let archive =
            crate::acting::fetching::where_the_archive_is_staged(&works.staging, run, version);
        std::fs::create_dir_all(works.staging.join(format!("{run}-unpacked")))
            .expect("the staging tree is made");
        std::fs::write(&archive, b"").expect("the archive is there");
        let unpacked_build = works
            .staging
            .join(format!("{run}-unpacked"))
            .join("blender-5.2.0-linux-x64");
        std::fs::create_dir_all(&unpacked_build).expect("the unpacked folder is made");
        let staged_executable = unpacked_build.join(steps::THE_EXECUTABLES_NAME);
        std::fs::write(&staged_executable, b"").expect("the build is there");

        let one_step = |step: WhichStep,
                        marker: Option<Boundary>,
                        inputs: &TheStepsInputs,
                        staged: Option<&Path>,
                        published: Option<&Path>,
                        answers: Vec<(&str, steps::HowItWent)>| {
            let runner = AStandInThatAnswersOnlyForAPathThatIsThere::holding(answers);
            perform_one_step(
                step,
                marker,
                &A_BLENDER_INSTALLS_LIST,
                inputs,
                &works,
                run,
                staged,
                published,
                &NoLineAtAll,
                &runner,
            )
        };

        // The unpack finds the build by looking, and answers where it is.
        let unpacked = one_step(
            WhichStep::TheUnpack,
            Some(Boundary::Fetched),
            &inputs,
            None,
            None,
            vec![("tar", it_went(0, "", ""))],
        );
        assert!(unpacked.it_ran_to_its_end, "{:?}", unpacked.why_not);
        assert_eq!(
            unpacked.the_staged_executable.as_deref(),
            Some(staged_executable.as_path())
        );

        // Both `ldd` reads run over the staged build, which is still there.
        for (step, marker) in [
            (WhichStep::TheLibraryChainIsRead, Some(Boundary::Tooling)),
            (
                WhichStep::TheLibraryChainIsReadAgain,
                Some(Boundary::Tooling),
            ),
        ] {
            let read = one_step(
                step,
                marker,
                &inputs,
                Some(&staged_executable),
                None,
                vec![("ldd", it_went(0, "", ""))],
            );
            assert!(read.it_ran_to_its_end, "{:?}", read.why_not);
        }

        // The publication answers where it published, and consumes the staged
        // material doing it.
        let published = one_step(
            WhichStep::TheBuildIsPublished,
            Some(Boundary::Libraries),
            &inputs,
            Some(&staged_executable),
            None,
            vec![(
                steps::THE_EXECUTABLES_NAME,
                it_went(0, "Blender 5.2.0\n", ""),
            )],
        );
        assert!(published.it_ran_to_its_end, "{:?}", published.why_not);
        let published_at = published.published_at.clone().expect("it says where");
        assert!(!staged_executable.exists(), "the staging path is gone");
        assert!(steps::the_published_executable(&published_at).is_file());

        // **The final check, given the publication's answer, reads the build
        // the machine actually holds and passes.**
        let checked = one_step(
            WhichStep::TheFinalVerification,
            Some(Boundary::Blender),
            &inputs,
            Some(&staged_executable),
            Some(&published_at),
            vec![
                ("ldd", it_went(0, "", "")),
                (
                    steps::THE_EXECUTABLES_NAME,
                    it_went(0, "Blender 5.2.0\n", ""),
                ),
            ],
        );
        assert!(checked.it_ran_to_its_end, "{:?}", checked.why_not);
        assert!(checked.it_performed, "it read something");

        // **And the stale path fails.** Whatever hands the final check the path
        // the publication took away — the run's own staged build, or a desktop
        // naming it — the machine answers that nothing is there and the step
        // says so.
        let stale = one_step(
            WhichStep::TheFinalVerification,
            Some(Boundary::Blender),
            &TheStepsInputs {
                version: Some(version),
                the_build_services_file: None,
                executable: Some(staged_executable.clone()),
                the_users_own_archive: None,
                the_users_answer: None,
                the_managers_package: None,
            },
            Some(&staged_executable),
            None,
            vec![
                ("ldd", it_went(0, "", "")),
                (
                    steps::THE_EXECUTABLES_NAME,
                    it_went(0, "Blender 5.2.0\n", ""),
                ),
            ],
        );
        assert!(!stale.it_ran_to_its_end);
        assert!(
            stale
                .why_not
                .as_deref()
                .is_some_and(|why| why.contains("No such file or directory")),
            "{:?}",
            stale.why_not
        );
    }

    /// **`Server Setup` Y7a item 2 — the Creator's ruling on the sixteenth
    /// notes, note 1: every step the worker performs reports what every one of
    /// its commands printed, both streams whole and apart, in the order the
    /// commands ran** — the unpack, both library reads with the loader's
    /// lookup beside `ldd`, the publication's version ask, and both halves of
    /// the final check. **A step whose commands printed nothing reports empty
    /// words and still ran to its end**; a step that failed reports its words
    /// too, and S2a's composed text is unchanged beside them. Until 2026-10-24
    /// the publication's version ask, the loader's lookups and the final check
    /// handed back nothing, and a container whose setup installed nothing
    /// showed nothing.
    #[test]
    fn y7a_every_command_a_worker_step_runs_reports_its_two_streams_whole_and_apart() {
        let area = crate::acting::its_own_folder("y7a-every-commands-words");
        let works = WhereTheRunWorks {
            staging: area.at().join("staging"),
            the_blender_area: area.at().join("blender"),
            the_run_started: Some(std::time::UNIX_EPOCH + std::time::Duration::from_secs(1_791_300_732)),
        };
        let version = Version { major: 5, minor: 2, patch: 0 };
        let inputs = TheStepsInputs {
            version: Some(version),
            ..Default::default()
        };
        let run = "a-run";
        let archive = crate::acting::fetching::where_the_archive_is_staged(&works.staging, run, version);
        let unpacked_build = works.staging.join(format!("{run}-unpacked")).join("blender-5.2.0-linux-x64");
        std::fs::create_dir_all(&unpacked_build).expect("the unpacked folder is made");
        std::fs::write(&archive, b"an archive").expect("the archive is there");
        let staged_executable = unpacked_build.join(steps::THE_EXECUTABLES_NAME);
        std::fs::write(&staged_executable, b"").expect("the build is there");
        let printed = |ordinary: &str, error: &str| WhatOneCommandPrinted {
            ordinary: ordinary.to_string(),
            error: error.to_string(),
        };
        let one_step = |step: WhichStep,
                        marker: Option<Boundary>,
                        staged: Option<&Path>,
                        published: Option<&Path>,
                        answers: Vec<(&str, steps::HowItWent)>| {
            perform_one_step(
                step,
                marker,
                &A_BLENDER_INSTALLS_LIST,
                &inputs,
                &works,
                run,
                staged,
                published,
                &NoLineAtAll,
                &AStandInForTheMachinesTools::holding(answers),
            )
        };

        let unpacked = one_step(
            WhichStep::TheUnpack,
            Some(Boundary::Fetched),
            None,
            None,
            vec![("tar", it_went(0, "blender-5.2.0-linux-x64/blender\n", "tar: a warning\n"))],
        );
        assert!(unpacked.it_ran_to_its_end, "{:?}", unpacked.why_not);
        assert_eq!(
            unpacked.what_its_commands_printed,
            vec![printed("blender-5.2.0-linux-x64/blender\n", "tar: a warning\n")]
        );

        // **`ldd` and the loader's lookup beside it, in that order.**
        let read = one_step(
            WhichStep::TheLibraryChainIsRead,
            Some(Boundary::Tooling),
            Some(&staged_executable),
            None,
            vec![
                ("ldd", it_went(0, "\tlibX11.so.6 => /lib/libX11.so.6\n", "")),
                ("sh", it_went(0, "", "a lookup's complaint\n")),
            ],
        );
        assert!(read.it_ran_to_its_end, "{:?}", read.why_not);
        assert_eq!(
            read.what_its_commands_printed,
            vec![
                printed("\tlibX11.so.6 => /lib/libX11.so.6\n", ""),
                printed("", "a lookup's complaint\n"),
            ]
        );

        // **Printing nothing is a reading, never a failure.**
        let quiet = one_step(
            WhichStep::TheLibraryChainIsReadAgain,
            Some(Boundary::Tooling),
            Some(&staged_executable),
            None,
            vec![("ldd", it_went(0, "", "")), ("sh", it_went(0, "", ""))],
        );
        assert!(quiet.it_ran_to_its_end, "{:?}", quiet.why_not);
        assert_eq!(quiet.what_its_commands_printed, vec![printed("", ""), printed("", "")]);
        assert!(quiet.the_machines_own_words.is_none(), "S2a's words appeared for a quiet step");

        let published = one_step(
            WhichStep::TheBuildIsPublished,
            Some(Boundary::Libraries),
            Some(&staged_executable),
            None,
            vec![(
                steps::THE_EXECUTABLES_NAME,
                it_went(0, "Blender 5.2.0\n", "Color management: a warning\n"),
            )],
        );
        assert!(published.it_ran_to_its_end, "{:?}", published.why_not);
        assert_eq!(
            published.what_its_commands_printed,
            vec![printed("Blender 5.2.0\n", "Color management: a warning\n")]
        );
        let published_at = published.published_at.clone().expect("it says where");

        // **The final check: its `ldd`, its lookup and its version ask.**
        let checked = one_step(
            WhichStep::TheFinalVerification,
            Some(Boundary::Blender),
            None,
            Some(&published_at),
            vec![
                ("ldd", it_went(0, "\tlibc.so.6 => /lib/libc.so.6\n", "")),
                ("sh", it_went(0, "", "")),
                (steps::THE_EXECUTABLES_NAME, it_went(0, "Blender 5.2.0\n", "")),
            ],
        );
        assert!(checked.it_ran_to_its_end, "{:?}", checked.why_not);
        assert_eq!(
            checked.what_its_commands_printed,
            vec![
                printed("\tlibc.so.6 => /lib/libc.so.6\n", ""),
                printed("", ""),
                printed("Blender 5.2.0\n", ""),
            ]
        );

        // **A step that failed reports its words as well, and S2a's text is
        // what it always was.**
        std::fs::write(&archive, b"an archive").expect("the archive is there again");
        let refused = one_step(
            WhichStep::TheUnpack,
            Some(Boundary::Fetched),
            None,
            None,
            vec![("tar", it_went(2, "", "tar: This does not look like a tar archive\n"))],
        );
        assert!(!refused.it_ran_to_its_end);
        assert_eq!(
            refused.what_its_commands_printed,
            vec![printed("", "tar: This does not look like a tar archive\n")]
        );
        assert_eq!(
            refused.the_machines_own_words.as_deref(),
            Some(
                "what it printed:\n\nwhat it printed on its error stream:\ntar: This does not look \
                 like a tar archive\n"
            )
        );

        // **A step that runs no program reports none** — the fetch's line is
        // Smoozy's own (Y7a item 3).
        let fetched = one_step(WhichStep::TheFetch, Some(Boundary::Started), None, None, vec![]);
        assert!(fetched.what_its_commands_printed.is_empty());
    }

    /// **R4: a recovery that re-runs the final check with nothing to read
    /// never reports a pass.**
    ///
    /// **Step 10c's fifth pass on Vast watched the other answer.** *Set up
    /// again* over the failed check inherited every boundary, re-ran the check
    /// with both halves absent, and the screen said ***The server is ready to
    /// render*** over a reading nobody had taken. **`Server Setup` R4 says an
    /// absent half claims nothing**, and the run's composed list with its
    /// marker is what tells a run that had a Blender to check from a run that
    /// never had one.
    ///
    /// **And a run with no Blender in it reaches *done* whatever its marker
    /// says.** Until 2026-10-04 the marker alone decided, so a run standing at
    /// *telegram* counted as one past a publication: `Server Setup` G2's
    /// Telegram placement, and a ready action with Telegram on and no Blender,
    /// ended here every time.
    #[test]
    fn a_final_check_with_nothing_to_read_never_reports_a_pass() {
        let empty = TheStepsInputs::default();
        let the_check_over = |the_list: &[Boundary], marker: Option<Boundary>| {
            let runner = AStandInThatAnswersOnlyForAPathThatIsThere::holding(vec![]);
            let came = perform_one_step(
                WhichStep::TheFinalVerification,
                marker,
                the_list,
                &empty,
                &works(),
                "a-run",
                None,
                None,
                &NoLineAtAll,
                &runner,
            );
            assert!(runner.asked.borrow().is_empty(), "nothing was read");
            came
        };
        let refused = |came: &WhatTheStepCameTo| {
            !came.it_ran_to_its_end
                && came
                    .why_not
                    .as_deref()
                    .is_some_and(|why| why.contains("nothing said where it was published"))
        };
        let found_nothing_to_check =
            |came: &WhatTheStepCameTo| came.it_ran_to_its_end && !came.it_performed;

        // **A run that published a Blender**: the check had a subject and
        // nothing carried it, so the step says so — at the publication's own
        // boundary, and past it at *telegram* where the run had both.
        let over_a_blender = the_check_over(&A_BLENDER_INSTALLS_LIST, Some(Boundary::Blender));
        assert!(refused(&over_a_blender), "{:?}", over_a_blender.why_not);
        let a_ready_action_with_both = [
            Boundary::Started,
            Boundary::Fetched,
            Boundary::Tooling,
            Boundary::Libraries,
            Boundary::Blender,
            Boundary::Telegram,
            Boundary::Done,
        ];
        let past_it = the_check_over(&a_ready_action_with_both, Some(Boundary::Telegram));
        assert!(refused(&past_it), "{:?}", past_it.why_not);

        // **A run with no Blender in it is the case R4 allows**: it ran, and
        // Y4a's mark records that it found nothing to check rather than that it
        // checked and passed.
        let no_blender = [Boundary::Started, Boundary::Done];
        let over_nothing = the_check_over(&no_blender, Some(Boundary::Started));
        assert!(found_nothing_to_check(&over_nothing), "{:?}", over_nothing.why_not);

        // **`Server Setup` G2's Telegram placement, and a ready action with
        // Telegram on and no Blender**: both are composed of the readings, the
        // Telegram material and *done*, and both stand at *telegram* here.
        let telegram_and_no_blender = [Boundary::Started, Boundary::Telegram, Boundary::Done];
        let after_telegram = the_check_over(&telegram_and_no_blender, Some(Boundary::Telegram));
        assert!(found_nothing_to_check(&after_telegram), "{:?}", after_telegram.why_not);

        // **An abandon over a Blender install stopped before its publication**
        // (`Server Setup` S6): the inherited list holds a publication and the
        // marker never reached it, so nothing was published to read.
        let abandoned = the_check_over(&A_BLENDER_INSTALLS_LIST, Some(Boundary::Fetched));
        assert!(found_nothing_to_check(&abandoned), "{:?}", abandoned.why_not);

        // And both readings are taken rather than guessed at.
        assert!(the_run_published_a_blender(&A_BLENDER_INSTALLS_LIST, Some(Boundary::Blender)));
        assert!(the_run_published_a_blender(&a_ready_action_with_both, Some(Boundary::Telegram)));
        assert!(!the_run_published_a_blender(&telegram_and_no_blender, Some(Boundary::Telegram)));
        assert!(!the_run_published_a_blender(&A_BLENDER_INSTALLS_LIST, Some(Boundary::Libraries)));
        assert!(!the_run_published_a_blender(&A_BLENDER_INSTALLS_LIST, Some(Boundary::Started)));
        assert!(!the_run_published_a_blender(&A_BLENDER_INSTALLS_LIST, None));
        assert!(!the_run_published_a_blender(&[], Some(Boundary::Telegram)));
    }

    /// **B8, 2026-10-08: a published folder is named by the version and the
    /// run's start and never by the run's identity; a second apart is two
    /// folders; a name already there is never published over; and a step
    /// handed no start publishes nothing.** The Creator's third notes on the
    /// test copy, note 4.
    #[test]
    fn a_published_folder_is_named_by_the_version_and_the_runs_start() {
        let area = crate::acting::its_own_folder("a-published-folder-is-named-by-its-start");
        let version = Version { major: 5, minor: 2, patch: 0 };
        let inputs = TheStepsInputs { version: Some(version), ..TheStepsInputs::default() };
        let at = |seconds: u64| std::time::UNIX_EPOCH + std::time::Duration::from_secs(seconds);
        // One run's staged build, where the unpack would have left it.
        let staged = |run: &str| {
            let folder = area.at().join("staging").join(format!("{run}-unpacked")).join("b");
            std::fs::create_dir_all(&folder).expect("the staged folder is made");
            let executable = folder.join(steps::THE_EXECUTABLES_NAME);
            std::fs::write(&executable, run).expect("the build is there");
            executable
        };
        let publish = |run: &str, started: Option<std::time::SystemTime>, executable: &Path| {
            let runner = AStandInThatAnswersOnlyForAPathThatIsThere::holding(vec![(
                steps::THE_EXECUTABLES_NAME,
                it_went(0, "Blender 5.2.0\n", ""),
            )]);
            perform_one_step(
                WhichStep::TheBuildIsPublished,
                Some(Boundary::Libraries),
                &A_BLENDER_INSTALLS_LIST,
                &inputs,
                &WhereTheRunWorks {
                    staging: area.at().join("staging"),
                    the_blender_area: area.at().join("blender"),
                    the_run_started: started,
                },
                run,
                Some(executable),
                None,
                &NoLineAtAll,
                &runner,
            )
        };

        let first = staged("a-run-7f3e");
        let came = publish("a-run-7f3e", Some(at(1_791_300_732)), &first);
        assert!(came.it_ran_to_its_end, "{:?}", came.why_not);
        let first_at = came.published_at.expect("it says where");
        assert_eq!(first_at, area.at().join("blender").join("blender-5.2.0-20261006-153212"));
        assert!(!first_at.display().to_string().contains("a-run"), "no run identity");

        // **A second later, the same version is a folder of its own.**
        let second = staged("b-run");
        let came = publish("b-run", Some(at(1_791_300_733)), &second);
        assert!(came.it_ran_to_its_end, "{:?}", came.why_not);
        assert_eq!(
            came.published_at.as_deref(),
            Some(area.at().join("blender").join("blender-5.2.0-20261006-153213").as_path())
        );

        // **In the same second, the name is already there and is never
        // published over**: the step is refused, the first build is untouched
        // and the second's material stays in staging.
        let third = staged("c-run");
        let came = publish("c-run", Some(at(1_791_300_732)), &third);
        assert!(!came.it_ran_to_its_end);
        assert!(came.why_not.as_deref().is_some_and(|why| why.contains("already holds a build. Not replaced.")));
        assert_eq!(
            std::fs::read_to_string(steps::the_published_executable(&first_at)).unwrap(),
            "a-run-7f3e"
        );
        assert!(third.is_file());

        // **Handed no start, nothing is published.**
        let came = publish("c-run", None, &third);
        assert!(!came.it_ran_to_its_end);
        assert!(came.why_not.as_deref().is_some_and(|why| why.contains("no time the run started")));
        assert!(third.is_file());
        assert_eq!(std::fs::read_dir(area.at().join("blender")).unwrap().count(), 2);
    }

    // ── Y7b and Y7c: how far the fetch and the unpack have got — the
    // Creator's fourteenth notes, piece U, 2026-10-18.

    /// **A count heard**, when it was heard, the bytes so far and the whole.
    #[derive(Default)]
    struct TheCountsHeard(std::cell::RefCell<Vec<(std::time::Instant, u64, Option<u64>)>>);

    impl crate::acting::TellsHowFarItHasGot for TheCountsHeard {
        fn so_far(&self, bytes: u64, whole: Option<u64>) {
            self.0.borrow_mut().push((std::time::Instant::now(), bytes, whole));
        }
    }

    /// **The archive both stand-ins below serve and unpack** — forty pieces of
    /// a thousand bytes, opening with the xz mark so the unpack names `-J`.
    fn a_slow_archive() -> Vec<u8> {
        let mut archive = vec![0xFD, b'7', b'z', b'X', b'Z', 0x00];
        archive.extend((0..40_000 - 6).map(|at| (at % 251) as u8));
        archive
    }

    const A_PIECE: usize = 1_000;
    const BETWEEN_TWO_PIECES: std::time::Duration = std::time::Duration::from_millis(5);
    const THE_CADENCE_HERE: std::time::Duration = std::time::Duration::from_millis(30);

    /// **A download that hands its archive over a piece at a time**, the way a
    /// line does, honouring a range and announcing the body's length. Only
    /// the release folders' two addresses are answered.
    struct ASlowLine {
        archive: Vec<u8>,
        asked_from: std::cell::RefCell<Vec<u64>>,
    }

    impl MakesTheRequestOnTheMachinesOwnLine for ASlowLine {
        fn fetch_into(
            &self,
            address: &str,
            from_byte: u64,
            into: &mut dyn crate::acting::fetching::TheBytesLand,
        ) -> Result<crate::acting::fetching::WhatTheLineGave, String> {
            use crate::acting::fetching::WhatTheLineGave;
            use sha2::{Digest, Sha256};
            if address.ends_with(".sha256") {
                let digest: String = Sha256::digest(&self.archive)
                    .iter()
                    .map(|byte| format!("{byte:02x}"))
                    .collect();
                into.the_answer_is(WhatTheLineGave::TheWholeFile)?;
                into.take(format!("{digest}  blender-5.2.2-linux-x64.tar.xz\n").as_bytes())?;
                return Ok(WhatTheLineGave::TheWholeFile);
            }
            self.asked_from.borrow_mut().push(from_byte);
            let gave = if from_byte > 0 {
                WhatTheLineGave::TheRangeAsked
            } else {
                WhatTheLineGave::TheWholeFile
            };
            into.the_answer_is(gave)?;
            let rest = &self.archive[from_byte as usize..];
            into.the_body_is(rest.len() as u64);
            for piece in rest.chunks(A_PIECE) {
                std::thread::sleep(BETWEEN_TWO_PIECES);
                into.take(piece)?;
            }
            Ok(gave)
        }
    }

    /// **An unpack tool that reads what it is fed a piece at a time** and then
    /// leaves a build where the archive would have put one. What it proves is
    /// the feeding and the counting, never what `tar` does.
    struct ASlowUnpackTool {
        asked: std::cell::RefCell<Vec<(String, Vec<String>)>>,
        read: std::cell::Cell<u64>,
    }

    impl RunsOneCommandOnTheMachine for ASlowUnpackTool {
        fn run(&self, program: &str, _: &[&str], _: Option<std::time::Duration>) -> Result<steps::HowItWent, String> {
            Err(format!("this stand-in is only fed, and {program} was run unfed"))
        }

        fn run_fed(&self, program: &str, arguments: &[&str], fed: &mut dyn std::io::Read) -> Result<steps::HowItWent, String> {
            self.asked.borrow_mut().push((
                program.to_string(),
                arguments.iter().map(|it| it.to_string()).collect(),
            ));
            let mut piece = vec![0u8; A_PIECE];
            loop {
                let read = fed.read(&mut piece).map_err(|why| why.to_string())?;
                if read == 0 {
                    break;
                }
                self.read.set(self.read.get() + read as u64);
                std::thread::sleep(BETWEEN_TWO_PIECES);
            }
            let into = arguments.last().expect("the folder it unpacks into");
            let build = Path::new(into).join("blender-5.2.2-linux-x64");
            std::fs::create_dir_all(&build).map_err(|why| why.to_string())?;
            std::fs::write(build.join(steps::THE_EXECUTABLES_NAME), b"").map_err(|why| why.to_string())?;
            Ok(it_went(0, "", ""))
        }
    }

    /// **What a run of counts must look like**: at least three, each larger
    /// than the last, each against the same whole, none past it, and never two
    /// closer together than the cadence.
    fn the_counts_rise_no_more_often_than_the_cadence(heard: &[(std::time::Instant, u64, Option<u64>)], whole: u64) {
        assert!(heard.len() >= 3, "too few counts to rise: {:?}", heard.iter().map(|it| it.1).collect::<Vec<_>>());
        for pair in heard.windows(2) {
            assert!(pair[1].1 > pair[0].1, "a count did not rise: {} then {}", pair[0].1, pair[1].1);
            assert!(
                pair[1].0.duration_since(pair[0].0) >= THE_CADENCE_HERE,
                "two counts came closer together than the cadence"
            );
        }
        for (_, bytes, said_whole) in heard {
            assert_eq!(*said_whole, Some(whole), "a count came without its whole");
            assert!(*bytes <= whole, "a count went past its whole");
        }
    }

    fn a_slow_install(staging: &Path) -> (WhereTheRunWorks, TheStepsInputs) {
        (
            WhereTheRunWorks {
                staging: staging.to_path_buf(),
                the_blender_area: staging.join("blender"),
                the_run_started: None,
            },
            TheStepsInputs {
                version: Some(Version { major: 5, minor: 2, patch: 2 }),
                ..Default::default()
            },
        )
    }

    /// **Y7b and Y7c: while a slow download and a slow unpack run, the counts
    /// told rise, against the whole, no more often than the cadence — and the
    /// unpack is fed the archive and counts what it fed, never what it
    /// wrote.** Piece U.
    #[test]
    fn piece_u_a_slow_download_and_unpack_tell_rising_counts_no_more_often_than_the_cadence() {
        let folder = crate::acting::its_own_folder("piece-u-slow");
        let (works, inputs) = a_slow_install(folder.at());
        let archive = a_slow_archive();
        let line = ASlowLine {
            archive: archive.clone(),
            asked_from: std::cell::RefCell::new(Vec::new()),
        };
        let nothing_runs = AStandInForTheMachinesTools::holding(vec![]);

        let heard = TheCountsHeard::default();
        let told = crate::acting::NoMoreOftenThan::new(THE_CADENCE_HERE, &heard);
        let fetched = perform_one_step_telling(
            WhichStep::TheFetch,
            Some(Boundary::Started),
            &A_BLENDER_INSTALLS_LIST,
            &inputs,
            &works,
            "run-a",
            None,
            None,
            &line,
            &nothing_runs,
            &told,
        );
        assert!(fetched.it_ran_to_its_end, "{:?}", fetched.why_not);
        the_counts_rise_no_more_often_than_the_cadence(&heard.0.borrow(), archive.len() as u64);

        let tool = ASlowUnpackTool {
            asked: std::cell::RefCell::new(Vec::new()),
            read: std::cell::Cell::new(0),
        };
        let heard = TheCountsHeard::default();
        let told = crate::acting::NoMoreOftenThan::new(THE_CADENCE_HERE, &heard);
        let unpacked = perform_one_step_telling(
            WhichStep::TheUnpack,
            Some(Boundary::Fetched),
            &A_BLENDER_INSTALLS_LIST,
            &inputs,
            &works,
            "run-a",
            None,
            None,
            &NoLineAtAll,
            &tool,
            &told,
        );
        assert!(unpacked.it_ran_to_its_end, "{:?}", unpacked.why_not);
        assert!(unpacked.the_staged_executable.is_some());
        // **Fed whole, on its standard input, told what wraps it.**
        assert_eq!(tool.read.get(), archive.len() as u64, "the tool was not fed the whole archive");
        let into = where_the_archive_is_unpacked(folder.at(), "run-a").display().to_string();
        assert_eq!(
            tool.asked.borrow().as_slice(),
            &[(
                steps::THE_UNPACK_TOOL.to_string(),
                ["-x", "-J", "-f", "-", "-C", into.as_str()].map(str::to_string).to_vec()
            )]
        );
        // **Counted against the staged archive's own size**, which is what it
        // reads, and nothing about what it wrote.
        the_counts_rise_no_more_often_than_the_cadence(&heard.0.borrow(), archive.len() as u64);
    }

    // ── Y7a item 2, Y7b and Y7c: the step's words handed on as they print —
    // the eighteenth notes, piece 2b, 2026-10-25.

    /// **Every piece handed on**, when it came, on which stream.
    #[derive(Default)]
    struct TheWordsHeard(std::sync::Mutex<Vec<(std::time::Instant, steps::TheStepsStream, String)>>);

    impl steps::HandsOnWhatIsPrinted for TheWordsHeard {
        fn printed(&self, stream: steps::TheStepsStream, bytes: &[u8]) {
            if bytes.is_empty() {
                return;
            }
            self.0
                .lock()
                .expect("the words heard")
                .push((std::time::Instant::now(), stream, String::from_utf8_lossy(bytes).into_owned()));
        }
    }

    impl TheWordsHeard {
        fn on(&self, stream: steps::TheStepsStream) -> Vec<(std::time::Instant, String)> {
            self.0
                .lock()
                .expect("the words heard")
                .iter()
                .filter(|(_, on, _)| *on == stream)
                .map(|(at, _, words)| (*at, words.clone()))
                .collect()
        }

        fn whole(&self, stream: steps::TheStepsStream) -> String {
            self.on(stream).into_iter().map(|(_, words)| words).collect()
        }
    }

    /// A slow progress-line receiver must not consume the next line's interval.
    /// `Server Setup` Y7b and Y7c bound when lines are printed, so the clock
    /// starts again after the receiver has accepted the previous line.
    #[test]
    fn a_slow_progress_print_does_not_spend_the_next_lines_cadence() {
        struct ASlowFirstPrint {
            heard: TheWordsHeard,
            every: std::time::Duration,
        }
        impl steps::HandsOnWhatIsPrinted for ASlowFirstPrint {
            fn printed(&self, stream: steps::TheStepsStream, bytes: &[u8]) {
                if self.heard.on(stream).is_empty() {
                    std::thread::sleep(self.every * 3);
                }
                self.heard.printed(stream, bytes);
            }
        }
        use crate::acting::TellsHowFarItHasGot;
        let every = std::time::Duration::from_millis(100);
        let sink = ASlowFirstPrint { heard: TheWordsHeard::default(), every };
        let lines = TheProgressLines::of(
            TheActInProgress::TheDownload,
            "blender.tar.xz",
            every,
            &sink,
            &crate::acting::NobodyIsTold,
        );
        lines.so_far(0, Some(4_000_000));
        std::thread::sleep(every);
        lines.so_far(1_000_000, Some(4_000_000));
        lines.so_far(2_000_000, Some(4_000_000));
        assert_eq!(sink.heard.on(steps::TheStepsStream::TheOrdinary).len(), 1,
            "printing the first line used up the next interval");
        std::thread::sleep(every);
        lines.so_far(3_000_000, Some(4_000_000));
        assert_eq!(sink.heard.on(steps::TheStepsStream::TheOrdinary).len(), 2);
    }

    /// **Y7b and Y7c: while a slow download and a slow unpack run, the worker
    /// prints its own progress lines on the ordinary stream as it goes — in
    /// Design's words, each a whole line, no more often than the cadence,
    /// while the step is still running — and once more as each ends.**
    /// Nothing goes on the error stream, and the desktop's count is told
    /// beside the lines exactly as before. Before piece 2b the fetch handed on
    /// nothing at all: note 5's *"completely silent"*.
    #[test]
    fn eighteenth_y7b_y7c_the_worker_prints_its_progress_lines_as_it_goes() {
        let folder = crate::acting::its_own_folder("eighteenth-lines");
        let (works, inputs) = a_slow_install(folder.at());
        let archive = a_slow_archive();
        let line = ASlowLine {
            archive: archive.clone(),
            asked_from: std::cell::RefCell::new(Vec::new()),
        };
        let heard = TheCountsHeard::default();
        let words = TheWordsHeard::default();
        let fetched = perform_one_step_handing_on(
            WhichStep::TheFetch,
            Some(Boundary::Started),
            &A_BLENDER_INSTALLS_LIST,
            &inputs,
            &works,
            "run-a",
            None,
            None,
            &line,
            &AStandInForTheMachinesTools::holding(vec![]),
            &heard,
            &words,
            THE_CADENCE_HERE,
        );
        let the_step_ended = std::time::Instant::now();
        assert!(fetched.it_ran_to_its_end, "{:?}", fetched.why_not);
        assert!(!heard.0.borrow().is_empty(), "the count was no longer told");
        let printed = words.on(steps::TheStepsStream::TheOrdinary);
        assert!(printed.len() >= 3, "too few progress lines: {printed:?}");
        let (last, during) = printed.split_last().expect("lines");
        for (at, said) in during {
            assert!(*at < the_step_ended);
            let rest = said
                .strip_prefix("blender-5.2.2-linux-x64.tar.xz: 0 of 0 MB, ")
                .unwrap_or_else(|| panic!("not Design's download line: {said:?}"));
            let rate = rest.strip_suffix(" MB/s\n").unwrap_or_else(|| panic!("not one whole line: {said:?}"));
            assert!(rate.split_once('.').is_some_and(|(_, tenths)| tenths.len() == 1), "{said:?}");
        }
        for pair in during.windows(2) {
            assert!(pair[1].0.duration_since(pair[0].0) >= THE_CADENCE_HERE, "two lines closer than the cadence");
        }
        assert!(
            last.1.starts_with("blender-5.2.2-linux-x64.tar.xz: 0 MB downloaded in ") && last.1.ends_with(" s\n"),
            "the fetch did not say once more that it ended: {:?}",
            last.1
        );
        assert_eq!(words.whole(steps::TheStepsStream::TheErrors), "");

        let tool = ASlowUnpackTool {
            asked: std::cell::RefCell::new(Vec::new()),
            read: std::cell::Cell::new(0),
        };
        let words = TheWordsHeard::default();
        let unpacked = perform_one_step_handing_on(
            WhichStep::TheUnpack,
            Some(Boundary::Fetched),
            &A_BLENDER_INSTALLS_LIST,
            &inputs,
            &works,
            "run-a",
            None,
            None,
            &NoLineAtAll,
            &tool,
            &crate::acting::NobodyIsTold,
            &words,
            THE_CADENCE_HERE,
        );
        assert!(unpacked.it_ran_to_its_end, "{:?}", unpacked.why_not);
        let printed: Vec<String> = words
            .on(steps::TheStepsStream::TheOrdinary)
            .into_iter()
            .map(|(_, said)| said)
            .collect();
        assert!(printed.len() >= 3, "too few unpack lines: {printed:?}");
        let shares: Vec<u32> = printed
            .iter()
            .map(|said| {
                said.strip_prefix("blender-5.2.2-linux-x64.tar.xz: ")
                    .and_then(|rest| rest.strip_suffix("% unpacked\n"))
                    .and_then(|share| share.parse().ok())
                    .unwrap_or_else(|| panic!("not Design's unpack line: {said:?}"))
            })
            .collect();
        assert!(shares.windows(2).all(|pair| pair[1] > pair[0]), "{shares:?}");
        assert_eq!(shares.last(), Some(&100), "the unpack did not say once more that it ended");
    }

    /// **Y7b: where the download gave no size, the line is the amount and the
    /// speed alone**, and a share of an unknown size says nothing.
    #[test]
    fn eighteenth_a_download_with_no_size_says_the_amount_and_the_speed_alone() {
        let words = TheWordsHeard::default();
        let lines = TheProgressLines::of(
            TheActInProgress::TheDownload,
            "blender-5.2.2-linux-x64.tar.xz",
            std::time::Duration::ZERO,
            &words,
            &crate::acting::NobodyIsTold,
        );
        use crate::acting::TellsHowFarItHasGot;
        lines.so_far(1_000_000, None);
        std::thread::sleep(std::time::Duration::from_millis(5));
        lines.so_far(214_000_000, None);
        let said = words.whole(steps::TheStepsStream::TheOrdinary);
        assert!(said.starts_with("blender-5.2.2-linux-x64.tar.xz: 214 MB, "), "{said:?}");
        assert!(said.ends_with(" MB/s\n") && !said.contains(" of "), "{said:?}");
        let words = TheWordsHeard::default();
        let lines = TheProgressLines::of(
            TheActInProgress::TheUnpack,
            "blender-5.2.2-linux-x64.tar.xz",
            std::time::Duration::ZERO,
            &words,
            &crate::acting::NobodyIsTold,
        );
        lines.so_far(1, None);
        lines.so_far(2, None);
        lines.the_unpack_ended();
        assert_eq!(words.whole(steps::TheStepsStream::TheOrdinary), "");
    }

    /// **Y7a item 2: a failing step Smoozy's program runs ends with its own
    /// sentence as that step's last line on the error stream** — S2a's
    /// composed text, so *What the server said* and *Server output* say the
    /// same thing (note 3: *"that same message is not present in the server
    /// output"*). **A step whose S2a text is a command's printed words adds
    /// nothing**, and neither does a step that ran to its end. Before piece 2b
    /// a refused fetch handed on nothing at all.
    #[test]
    fn eighteenth_a_failing_steps_own_sentence_is_its_last_line_on_the_error_stream() {
        struct NothingAtTheArchivesAddress;
        impl MakesTheRequestOnTheMachinesOwnLine for NothingAtTheArchivesAddress {
            fn fetch_into(
                &self,
                address: &str,
                _from_byte: u64,
                into: &mut dyn crate::acting::fetching::TheBytesLand,
            ) -> Result<crate::acting::fetching::WhatTheLineGave, String> {
                use crate::acting::fetching::WhatTheLineGave;
                if address.ends_with(".sha256") {
                    into.the_answer_is(WhatTheLineGave::TheWholeFile)?;
                    into.take(format!("{}  blender-5.2.2-linux-x64.tar.xz\n", "0".repeat(64)).as_bytes())?;
                    return Ok(WhatTheLineGave::TheWholeFile);
                }
                into.the_answer_is(WhatTheLineGave::Neither { status: 404 })?;
                Ok(WhatTheLineGave::Neither { status: 404 })
            }
        }
        let folder = crate::acting::its_own_folder("eighteenth-sentence");
        let (works, inputs) = a_slow_install(folder.at());
        let performed = |step, marker, line: &dyn MakesTheRequestOnTheMachinesOwnLine, runner: &dyn RunsOneCommandOnTheMachine| {
            let words = TheWordsHeard::default();
            let came = perform_one_step_handing_on(
                step,
                marker,
                &A_BLENDER_INSTALLS_LIST,
                &inputs,
                &works,
                "run-a",
                None,
                None,
                line,
                runner,
                &crate::acting::NobodyIsTold,
                &words,
                THE_CADENCE_HERE,
            );
            (
                came,
                words.whole(steps::TheStepsStream::TheOrdinary),
                words.whole(steps::TheStepsStream::TheErrors),
            )
        };

        // **The fetch, refused by the far end**: no command ran, and the
        // program's own sentence is the step's one line.
        let (came, ordinary, errors) = performed(
            WhichStep::TheFetch,
            Some(Boundary::Started),
            &NothingAtTheArchivesAddress,
            &AStandInForTheMachinesTools::holding(vec![]),
        );
        assert!(!came.it_ran_to_its_end);
        assert_eq!(errors, "Download failed: HTTP 404.\n");
        assert_eq!(Some(errors.trim_end()), came.why_not.as_deref(), "Server output and What the server said differ");
        assert_eq!(ordinary, "");

        // **The unpack, where the tool printed why**: its words are S2a's, are
        // already on the stream, and nothing is added.
        let staged = crate::acting::fetching::where_the_archive_is_staged(
            folder.at(),
            "run-a",
            Version { major: 5, minor: 2, patch: 2 },
        );
        std::fs::write(&staged, a_slow_archive()).expect("an archive is staged");
        let (came, _, errors) = performed(
            WhichStep::TheUnpack,
            Some(Boundary::Fetched),
            &NoLineAtAll,
            &AStandInForTheMachinesTools::holding(vec![("tar", it_went(2, "", "xz: (stdin): File format not recognized"))]),
        );
        assert!(!came.it_ran_to_its_end && came.the_machines_own_words.is_some());
        assert_eq!(errors, "xz: (stdin): File format not recognized");

        // **The unpack, where the tool printed nothing**: Smoozy's sentence
        // is S2a's, and it is printed.
        let (came, _, errors) = performed(
            WhichStep::TheUnpack,
            Some(Boundary::Fetched),
            &NoLineAtAll,
            &AStandInForTheMachinesTools::holding(vec![("tar", it_went(2, "", ""))]),
        );
        assert!(came.the_machines_own_words.is_none());
        assert_eq!(errors, "The unpack tool exited 2.\n");

        // **A refusal before anything ran** is the same kind of sentence.
        let (came, _, errors) = performed(
            WhichStep::TheTelegramMaterial,
            None,
            &NoLineAtAll,
            &AStandInForTheMachinesTools::holding(vec![]),
        );
        assert_eq!(Some(errors.trim_end()), came.why_not.as_deref());

        // **Which endings print the sentence**, read off the step's own
        // record: Smoozy's own sentence with no command's words, and nothing
        // else.
        let smoozys_own = WhatTheStepCameTo::did_not("the final check did not pass".to_string());
        assert_eq!(the_failing_steps_own_sentence(&smoozys_own), Some("the final check did not pass"));
        let mut a_commands = WhatTheStepCameTo::did_not("the unpack tool exited 2".to_string());
        a_commands.the_machines_own_words = Some("what it printed:\n".to_string());
        assert_eq!(the_failing_steps_own_sentence(&a_commands), None);

        // **A sentence after words that did not end their line starts a line
        // of its own.**
        let kept_words = AStandInForTheMachinesTools::holding(vec![("ldd", it_went(1, "", "ldd: no newline"))]);
        let kept = steps::EveryCommandsWordsAreKept::around(&kept_words);
        assert!(!kept.the_error_stream_ends_mid_line());
        kept.run("ldd", &[], None).expect("the stand-in answers");
        assert!(kept.the_error_stream_ends_mid_line());
        let ran = WhatTheStepCameTo::ran(WhichStep::TheFetch, true);
        assert_eq!(the_failing_steps_own_sentence(&ran), None);
        let mut held = WhatTheStepCameTo::did_not("held".to_string());
        held.held_for_a_person = true;
        assert_eq!(the_failing_steps_own_sentence(&held), None);
    }

    /// **Y7b: a resumed download counts from what was already staged**, and its
    /// whole is the archive's, not the rest's. Piece U.
    #[test]
    fn piece_u_a_resumed_download_counts_what_was_staged() {
        let folder = crate::acting::its_own_folder("piece-u-resume");
        let (works, inputs) = a_slow_install(folder.at());
        let archive = a_slow_archive();
        let staged = crate::acting::fetching::where_the_archive_is_staged(
            folder.at(),
            "run-a",
            Version { major: 5, minor: 2, patch: 2 },
        );
        std::fs::create_dir_all(folder.at()).expect("the staging area is made");
        std::fs::write(&staged, &archive[..15_000]).expect("an earlier fetch's prefix is staged");
        let line = ASlowLine {
            archive: archive.clone(),
            asked_from: std::cell::RefCell::new(Vec::new()),
        };
        let heard = TheCountsHeard::default();
        let came = perform_one_step_telling(
            WhichStep::TheFetch,
            Some(Boundary::Started),
            &A_BLENDER_INSTALLS_LIST,
            &inputs,
            &works,
            "run-a",
            None,
            None,
            &line,
            &AStandInForTheMachinesTools::holding(vec![]),
            &heard,
        );
        assert!(came.it_ran_to_its_end, "{:?}", came.why_not);
        assert_eq!(line.asked_from.borrow().as_slice(), &[15_000]);
        let heard = heard.0.borrow();
        let (_, first, whole) = heard.first().expect("a count was told");
        assert_eq!(*first, 15_000 + A_PIECE as u64, "the first count did not start from what was staged");
        assert_eq!(*whole, Some(archive.len() as u64), "the whole was the rest's, not the archive's");
        assert_eq!(heard.last().map(|it| it.1), Some(archive.len() as u64));
    }
}
