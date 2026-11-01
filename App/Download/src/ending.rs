//! What ends an arrangement — `Download` §7, E1 to E5 with E2a and E3a, Rows
//! 214, 215 and 220.
//!
//! **Constraint:** E1's own opening — the arrangement **outlives the job's
//! ending** on `Queue & History`'s signal, and then decides for itself that
//! nothing more is coming **by finishing its own bookkeeping, never by a
//! judgement about the render.**
//!
//! # E1 step 2 carried a wording N4 had corrected, and the plan now says what
//! this code does
//!
//! **E1 step 2 reads *added-place and unmatched files are fetchable under
//! every arrangement, narrowed or not*, corrected in the plan on 2026-09-05.**
//! It had read *fetchable where the arrangement is unnarrowed*, and it was the
//! last place in the plan still carrying the reading N4 and J8 both corrected
//! on 2026-08-21 answering Row 314. N4's own words: *"such a file is *outside
//! the narrowing*, which means it is not filtered out by it, and not *outside
//! the arrangement*, which is what this rule used to say"*. N4b names the two
//! specs the older reading contradicted outright, each promising
//! unconditionally that a bake and a cache come home — and the user cannot ask
//! one back, an added write place having no per-file entries and so no format
//! for the cut to select on (`Job files and outputs` I14).
//!
//! **[`the_ending_walk`] takes no narrowing at all for a file with no narrowed
//! entry, and a test drives a narrowed arrangement to prove it.** The code was
//! built to the corrected reading before the plan carried it, on N4 and J8;
//! the disagreement is recorded as a finding in
//! `Audits/Build/Download/Decisions record.md`, a reader confirmed it, and the
//! plan was repaired. **Nothing here moved for the repair.**
//!
//! **Markers used below that belong to another document, each with the
//! document it belongs to.** `Queue & History` C1. `Desktop app and delivery`
//! T9, T10, P10d. `Rented-server runtime` V13. `Job files and outputs` I14,
//! I20.
//!
//! **A marker this plan also defines is deliberately not listed above,
//! whichever document is meant, and is written out with its document at every
//! site instead** — the header list cannot tell the two apart, and a bare one
//! would resolve to a real and wrong clause. `Queue & History` E2 and
//! `Servers` W40 are the two that arise here.

use smoozy_queue_history::ending::Ending;

use crate::arrangement::{AnArrangement, DesiredState, WhatSmoozyDid};
use crate::narrowing::{ThePlacedCopies, WhatN5aSays};
use crate::vouching::{a_zero_byte_file_is_never_a_finished_frame, TheOrder};
use crate::watch::AWalkEntry;

/// **E1 step 1: the ending arrives — one of the four, live or at catch-up.
/// The module never finds out on its own.**
///
/// The endings are `Queue & History` E2's and are carried here rather than
/// decided here; the catch-up is handed over once per connection
/// (`Queue & History` C1).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HowTheEndingArrived {
    Live,
    AtTheCatchUp,
}

/// **E3: the ending's kind changes nothing about what is fetched, only how
/// much there is — and one thing about what is said.**
///
/// That one thing is the *could not be vouched for* mark of E1 step 2 on the
/// frame that was in flight when the job ended badly.
pub fn the_ending_marks_the_frame_that_was_in_flight(ending: Ending) -> bool {
    !matches!(ending, Ending::Finished)
}

/// One narrowed inventory entry, as the ending walk needs it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ANarrowedEntry {
    /// The predicted path relative to the job output folder.
    pub predicted_path: Vec<u8>,
    /// **Absent for an entry with no frame** — a video destination's is the
    /// standing case (`Job files and outputs` I20).
    pub frame: Option<i64>,
}

/// **E1 step 2's verdict over one final complete walk.**
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct TheEndingWalksVerdict {
    /// Everything the ending makes fetchable, in the walk's own order for the
    /// files with no frame and in the entries' order for the rest.
    pub fetchable: Vec<Vec<u8>>,
    /// **Every narrowed entry absent or zero-byte becomes *never coming*.**
    pub never_coming: Vec<Vec<u8>>,
    /// **Of [`Self::never_coming`], the entries the walk did not find at
    /// all** — the zero-byte ones are the rest. **N6's last sentence and N7's
    /// narrowing turn on the difference**: once a job has ended short of a
    /// clean finish, the whole is what the ending's walk found, so an entry
    /// found at zero bytes stays in it as the shortfall and one not found at
    /// all is out. Added 2026-10-19, out of the live pass on Vast for the
    /// thirteenth and fourteenth notes (*ended 100 / 450* for a job stopped at
    /// 52 frames).
    pub not_found: Vec<Vec<u8>>,
    /// **Fetched, could not be vouched for** — the entries of the frame that
    /// was in flight when the job ended badly.
    pub could_not_be_vouched_for: Vec<Vec<u8>>,
    /// **N7's third bullet: the files the user's narrowing cut out** — out of
    /// the whole, silently correct, and re-openable where the narrowing
    /// widens.
    ///
    /// **They are recorded rather than left unentered**, because an entry that
    /// was never written cannot leave the whole and cannot re-open.
    pub skipped_by_narrowing: Vec<Vec<u8>>,
    /// **N5a: the copies Smoozy itself placed in the job folder before the
    /// render, skipped on the way home.**
    ///
    /// *"Without this rule the user would download the bake they had just
    /// uploaded, which on a simulation cache is not a rounding error."*
    pub skipped_as_the_copy_smoozy_placed: Vec<Vec<u8>>,
    /// **N5a's paths that could not be judged, which come home.**
    ///
    /// A path the placement record names but whose four values could not be
    /// read is brought home — the safe direction, costing at worst a
    /// re-download of something the user already has. **It is named here so
    /// the arrangement's reading can say so rather than letting it pass
    /// silently.** With `Server protocol and transfers` S13's reading having
    /// no producer in this workspace, this is every placed path today.
    pub placed_paths_that_could_not_be_judged: Vec<Vec<u8>>,
}

/// **E1 step 2: the ending walk — one final complete walk of the job output
/// folder — and everything it decides.**
///
/// - **Every narrowed entry present and not zero bytes is fetchable.**
/// - **Added-place and unmatched files are fetchable**, and a video
///   destination's entry is fetchable here and nowhere earlier. See this
///   module's own header for why no narrowing gate stands in front of them.
/// - **Every narrowed entry absent or zero-byte becomes *never coming*.**
/// - **And where the ending is *stopped*, *failed* or *given up*, the entries
///   of the highest frame in the attempt's order that shows any file — or,
///   where the frame boundary cannot be established, the entries active in the
///   last complete walk — are marked *fetched, could not be vouched for* and
///   named on the arrangement's existing telling surface.** They are fetched
///   exactly as they are now; the machine is the user's.
///
/// **The reason for the mark** is that the one file most likely to be torn in
/// the whole job is the frame that was being written when the job ended badly,
/// and it is the one file nothing else here would say anything about: the
/// count is not short, because the file arrived; the categories that leave the
/// whole do not include it; and there is no Blender saved line for a frame
/// whose save never completed.
///
/// **Wording is Design's.**
#[allow(
    clippy::too_many_arguments,
    reason = "E1 step 2 decides five things over five different readings — \
              the two halves of the narrowing cut, the walk, the ending, the \
              attempt's order, the last complete walk's active set and the \
              placement record. Gathering them would make one shape whose \
              only reader is this function"
)]
pub fn the_ending_walk(
    narrowed_entries: &[ANarrowedEntry],
    the_entries_the_narrowing_excludes: &[ANarrowedEntry],
    the_walk: &[AWalkEntry],
    ending: Ending,
    order: &TheOrder,
    the_last_complete_walks_active_entries: &[Vec<u8>],
    the_placed_copies: &ThePlacedCopies<'_>,
) -> TheEndingWalksVerdict {
    let mut verdict = TheEndingWalksVerdict::default();
    let read = |path: &[u8]| the_walk.iter().find(|entry| entry.path == path);

    for entry in narrowed_entries {
        match read(&entry.predicted_path) {
            Some(seen) if !a_zero_byte_file_is_never_a_finished_frame(seen.size) => {
                verdict.fetchable.push(entry.predicted_path.clone());
            }
            Some(_) => verdict.never_coming.push(entry.predicted_path.clone()),
            None => {
                verdict.never_coming.push(entry.predicted_path.clone());
                verdict.not_found.push(entry.predicted_path.clone());
            }
        }
    }

    // **N7's third bullet: what the user narrowed away is recorded as skipped
    // rather than left unentered.** It leaves the whole, and it re-opens to
    // *owed* where the narrowing widens — neither of which can happen to an
    // entry the record does not name.
    for entry in the_entries_the_narrowing_excludes {
        verdict
            .skipped_by_narrowing
            .push(entry.predicted_path.clone());
    }

    // The files with no frame: everything the walk shows that **no inventory
    // entry** names. They are fetched here and never live, and no narrowing
    // gate stands in front of them.
    //
    // **The excluded entries have to be told from these two.** A walk file
    // that matches an entry the user narrowed away is *skipped by narrowing*
    // and must not come home; a walk file that matches no inventory entry at
    // all is J8's added-place or unmatched file and comes home **under every
    // arrangement, narrowed or not** (N4). Without the excluded set in hand
    // this loop cannot tell them apart and fetches both, which defeats the cut
    // the user made — *"an artist who only wants a quick look should not have
    // to drag heavy files across an entire animation."*
    for seen in the_walk {
        if seen.is_a_folder {
            continue;
        }
        let named = narrowed_entries
            .iter()
            .chain(the_entries_the_narrowing_excludes)
            .any(|entry| entry.predicted_path == seen.path);
        if named {
            continue;
        }
        // **N5a: a bake or cache Smoozy itself placed in the job folder before
        // the render is skipped on the way home.** The test is Smoozy's own
        // placement record and never the file — not by name and not by looking
        // like a bake — and a copy whose values have moved is one the render
        // wrote to and comes home like everything else.
        match the_placed_copies.what_it_says_about(&seen.path) {
            WhatN5aSays::ItIsTheCopySmoozyPlacedAndIsSkipped => {
                verdict
                    .skipped_as_the_copy_smoozy_placed
                    .push(seen.path.clone());
                continue;
            }
            WhatN5aSays::ItIsAPlacedPathThatCouldNotBeJudged => {
                verdict
                    .placed_paths_that_could_not_be_judged
                    .push(seen.path.clone());
            }
            WhatN5aSays::ItIsNotAPlacedCopy => {}
        }
        verdict.fetchable.push(seen.path.clone());
    }

    if the_ending_marks_the_frame_that_was_in_flight(ending) {
        verdict.could_not_be_vouched_for = the_frame_in_flights_entries(
            narrowed_entries,
            the_walk,
            order,
            the_last_complete_walks_active_entries,
        );
    }
    verdict
}

/// **E1 step 2's mark, and its stated fallback.**
///
/// The entries of the **highest frame in the attempt's order that shows any
/// file** — or, **where the frame boundary cannot be established**, the
/// entries active in the last complete walk.
fn the_frame_in_flights_entries(
    narrowed_entries: &[ANarrowedEntry],
    the_walk: &[AWalkEntry],
    order: &TheOrder,
    the_last_complete_walks_active_entries: &[Vec<u8>],
) -> Vec<Vec<u8>> {
    let TheOrder::The(order) = order else {
        return the_last_complete_walks_active_entries.to_vec();
    };
    let shows_any_file = |frame: i64| {
        narrowed_entries.iter().any(|entry| {
            entry.frame == Some(frame)
                && the_walk
                    .iter()
                    .any(|seen| seen.path == entry.predicted_path)
        })
    };
    let Some(frame) = order
        .frames()
        .iter()
        .rev()
        .find(|frame| shows_any_file(**frame))
        .copied()
    else {
        return the_last_complete_walks_active_entries.to_vec();
    };
    narrowed_entries
        .iter()
        .filter(|entry| entry.frame == Some(frame))
        .filter(|entry| {
            the_walk
                .iter()
                .any(|seen| seen.path == entry.predicted_path)
        })
        .map(|entry| entry.predicted_path.clone())
        .collect()
}

/// **E1 step 4: the arrangement is done when every unit is terminal.**
///
/// The fetch queue drains to terminal states — fetched, left alone, cannot
/// land — under the ordinary pipeline, surviving drops and destination trouble
/// on the way.
pub fn every_unit_is_terminal(arrangement: &AnArrangement) -> bool {
    arrangement
        .reckoning
        .iter()
        .all(|entry| entry.account.state.is_terminal())
}

/// **The arrangement's reading, which is what the job's entry shows.**
///
/// **E2a added the third of these** and it is written out because *"neither of
/// E2's two readings covers that"* — a user who switched auto-download on at
/// submission and closed Smoozy for the night meets, in the morning, a job
/// that ended and files that never came, **with nothing between *ended and
/// still coming home* and *done* to say why**. **Smoozy going quiet is
/// Smoozy's fault whoever caused it.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TheArrangementsReading {
    /// The ordinary case before the job's ending: the arrangement is active
    /// and its progress is moving — arrivals against the whole.
    ArrivalsAgainstTheWhole {
        arrivals: usize,
        whole: usize,
        the_whole_may_grow: bool,
        also_home_outside_the_whole: usize,
    },
    /// **E2: in the interval between the ending and done, the reading says the
    /// job has ended and files are still coming home** — the arrangement still
    /// active, its progress moving. The entry's own ending is
    /// `Queue & History`'s to show; the bar's wording for a badly-ended job is
    /// Design's marked question and is not settled here.
    TheJobHasEndedAndFilesAreStillComingHome {
        arrivals: usize,
        whole: usize,
        the_whole_may_grow: bool,
        also_home_outside_the_whole: usize,
    },
    /// **E2a's third reading: waiting for a connection to that server**, where
    /// the desired state is on, units are still owed, and no walk can run
    /// because no connection to that server stands.
    ///
    /// **It is composed from what is already on the user's own computer** —
    /// the arrangement record and the connection condition `Servers` supplies
    /// — so **it needs no reading of a machine** and nothing here joins
    /// `Desktop app and delivery` P10d's closed set.
    ///
    /// **It ends by itself and nothing watches for it**: the next connection
    /// to that server resumes the walks and the reading returns to one of the
    /// others. Nothing withdraws it, nothing schedules it, nothing reaches
    /// out.
    WaitingForAConnectionToThatServer,
    /// **E1 step 4: done.** The reading says **ended**, with arrivals against
    /// the whole — short where short, and **never *complete***, which is the
    /// spec's own wording for a job that ends badly and the correct wording
    /// for one that ended well short of a lost write too.
    ///
    /// **Where a switch-on has reset the destination part-way through, the
    /// reading names both folders**: what came home before the reset is at the
    /// folder it went to and stays there, and what came after is at the new
    /// one.
    Ended {
        arrivals: usize,
        whole: usize,
        the_whole_may_grow: bool,
        also_home_outside_the_whole: usize,
        folders: Vec<Vec<u8>>,
    },
}

impl TheArrangementsReading {
    /// **E1 step 4: never *complete*.** A reading that says *ended* says so
    /// whether or not the arrivals reached the whole.
    pub fn says_ended(&self) -> bool {
        matches!(self, TheArrangementsReading::Ended { .. })
    }

    /// Whether the arrivals fell short of the whole — the shortfall the spec
    /// promises for what nobody could vouch for.
    pub fn is_short(&self) -> bool {
        match self {
            TheArrangementsReading::ArrivalsAgainstTheWhole {
                arrivals, whole, ..
            }
            | TheArrangementsReading::TheJobHasEndedAndFilesAreStillComingHome {
                arrivals,
                whole,
                ..
            }
            | TheArrangementsReading::Ended {
                arrivals, whole, ..
            } => arrivals < whole,
            TheArrangementsReading::WaitingForAConnectionToThatServer => false,
        }
    }

    /// **N4d's separate figure: what came home that the whole does not
    /// count.**
    ///
    /// Added-place and unmatched files are *"counted separately and reported
    /// separately, never folded into the narrowed count"*, and
    /// `Job files and outputs` I14 puts them in no count at all.
    pub fn also_home_outside_the_whole(&self) -> usize {
        match self {
            TheArrangementsReading::ArrivalsAgainstTheWhole {
                also_home_outside_the_whole,
                ..
            }
            | TheArrangementsReading::TheJobHasEndedAndFilesAreStillComingHome {
                also_home_outside_the_whole,
                ..
            }
            | TheArrangementsReading::Ended {
                also_home_outside_the_whole,
                ..
            } => *also_home_outside_the_whole,
            TheArrangementsReading::WaitingForAConnectionToThatServer => 0,
        }
    }

    /// **N6's second half, stated: whether the whole may still grow.**
    ///
    /// A video destination's entry counts one and the whole is stated as
    /// possibly growing (`Job files and outputs` I19, I20); further split
    /// files arriving under the folder are fetched at the ending and counted
    /// as the growth the statement allowed. **The waiting reading states no
    /// whole at all**, so it states no growth either.
    pub fn the_whole_may_grow(&self) -> bool {
        match self {
            TheArrangementsReading::ArrivalsAgainstTheWhole {
                the_whole_may_grow, ..
            }
            | TheArrangementsReading::TheJobHasEndedAndFilesAreStillComingHome {
                the_whole_may_grow,
                ..
            }
            | TheArrangementsReading::Ended {
                the_whole_may_grow, ..
            } => *the_whole_may_grow,
            TheArrangementsReading::WaitingForAConnectionToThatServer => false,
        }
    }
}

/// **E1 step 4, E2 and E2a, as one composition over the record and the
/// connection condition.**
///
/// **It reads no machine.** Every value it takes is on the user's own
/// computer: the arrangement record, the whole, whether a connection to that
/// server stands, and the run's own two facts.
///
/// # *Ended* needs the ending walk to have run, and this is why
///
/// **E1 step 4's *done* is the end of a sequence whose second step is the
/// ending walk**, so a reading that said *ended* without it would say the job
/// is finished before Smoozy has looked. **[`every_unit_is_terminal`] is true
/// of a record with no entries in it at all**, which is exactly the state of
/// an arrangement that has never had a connection.
///
/// **The failure path this closes.** Auto-download on at submission, Smoozy
/// closed for the night, the job ends. In the morning the user opens Smoozy
/// and has not connected to that server: the record is empty, the ending has
/// been carried in, and the reading said **Ended — 0 of 720 files** on a
/// design that draws that as a finished bar. E2a's own words for what that
/// costs are *"a user reading the job as finished, concluding the frames are
/// home, and destroying the machine they are still on"*. **The milder half
/// needs no closed app**: an ending arriving live while 620 files are still on
/// the machine read *ended* before the walk had found them.
///
/// **And with the ending in hand and the walk not yet run, an empty record
/// falls through to the connection test**, which is the reading E2a was
/// written for.
///
/// # The arrivals are counted over the entries the whole counts, and no others
///
/// **N6 fixes the whole as the narrowed inventory entries plus the video
/// statement**, and N4d puts added-place files *"counted separately and
/// reported separately, never folded into the narrowed count"* —
/// `Job files and outputs` I14 puts them in no count at all. **So a count of
/// every entry standing at *fetched* is a count of a different set from the
/// whole it is read against**: a job of 10 frames whose file wrote a 300-file
/// simulation cache into an added write place read **310 of 10**.
///
/// **The keys the whole counts are handed in**, because they are the
/// narrowing's own answer and nothing here re-derives one. Whatever came home
/// outside them is the second figure, which is N4d's.
///
/// **N6's whole is two things and both are handed in.** The count, and the
/// video statement — *"a video destination's entry counts one and the whole is
/// stated as possibly growing"*. [`crate::narrowing::the_whole`] composes the
/// statement over the narrowed entries and
/// [`crate::acting::record_directory::what_the_narrowing_leaves`] carries it,
/// so a reading of a video job says its whole may still grow rather than
/// stating a number that will have to be broken.
pub fn the_reading(
    arrangement: &AnArrangement,
    whole: usize,
    the_whole_may_grow: bool,
    the_keys_the_whole_counts: &[Vec<u8>],
    a_connection_to_the_server_stands: bool,
    the_jobs_ending_has_arrived: bool,
    the_ending_walk_has_completed: bool,
) -> TheArrangementsReading {
    let home = arrangement
        .reckoning
        .iter()
        .filter(|entry| entry.account.state == WhatSmoozyDid::Fetched);
    let arrivals = home
        .clone()
        .filter(|entry| the_keys_the_whole_counts.contains(&entry.predicted_path))
        .count();
    let also_home_outside_the_whole = home.count() - arrivals;
    if the_ending_walk_has_completed
        && every_unit_is_terminal(arrangement)
        && the_jobs_ending_has_arrived
    {
        return TheArrangementsReading::Ended {
            arrivals,
            whole,
            the_whole_may_grow,
            also_home_outside_the_whole,
            folders: arrangement.the_folders_it_has_fetched_into(),
        };
    }
    if arrangement.desired_state == DesiredState::On && !a_connection_to_the_server_stands {
        return TheArrangementsReading::WaitingForAConnectionToThatServer;
    }
    if the_jobs_ending_has_arrived {
        return TheArrangementsReading::TheJobHasEndedAndFilesAreStillComingHome {
            arrivals,
            whole,
            the_whole_may_grow,
            also_home_outside_the_whole,
        };
    }
    TheArrangementsReading::ArrivalsAgainstTheWhole {
        arrivals,
        whole,
        the_whole_may_grow,
        also_home_outside_the_whole,
    }
}

/// **E2a's own bounds, held as values.**
///
/// **It is not a notification and none is owed.** `Desktop app and delivery`
/// T9's four kinds are closed, and the outputs being home is already ruled out
/// of them; something that has **not** happened is further out still.
///
/// **What it does not claim.** It says nothing about what is on the machine
/// beyond what the arrangement's own record holds, and **it is not a warning
/// against destroying the rental** — no clause anywhere lets Smoozy speak for
/// a rental it did not take (`Servers` W40, `Rented-server runtime` V13).
/// **What it prevents is the one real loss in that morning**: a user reading
/// the job as *finished*, concluding the frames are home, and destroying the
/// machine they are still on.
pub const THE_WAITING_READING_IS_NOT_A_NOTIFICATION_AND_NONE_IS_OWED: bool = true;

/// **E1 step 4: done is not for ever.**
///
/// A new render-start for the same job re-arms the arrangement — the *never
/// coming* entries re-open to *owed* for the frames the new attempt's list
/// names, the whole is recomputed, the walks resume, and the sequence runs
/// again to a second done. **Nothing else re-opens, so a re-run never arrives
/// over a file the user already has.**
pub const DONE_IS_NOT_FOR_EVER: bool = true;

/// **E3: a job that ends badly still gives up what it made.**
///
/// Steps 2 and 3 run identically for *failed*, *stopped* and *given up*.
/// Where the server itself went down, what it made may be less than what it
/// reported making, the emptied frames land as zero-byte *never coming*
/// entries, and the short count says so. **And the user is told when those
/// outputs are home, the same as for a finished job: the moment is the done
/// transition of E1 step 4 and the surface is the job's own entry** — *"where
/// it says so is the job's entry, not the moment"* — **and it does not change
/// with the ending's kind.**
pub const THE_MOMENT_IS_THE_DONE_TRANSITION_AND_THE_SURFACE_IS_THE_JOBS_ENTRY: bool = true;

/// **E3a: no notification is posted for the outputs being home and none is
/// owed.**
///
/// `Desktop app and delivery` T9 fixes the notification kinds as the spec's
/// four and closed — a job finishing, a job failing or being given up, a
/// connection Smoozy opened and could not restore, a machine idle for half an
/// hour — **and none of them is *the outputs are home***; its T10 keeps a job
/// the user stopped off even the first. **The render finishing and the files
/// being home are two different moments**, which is the spec's own sentence,
/// and only the first has a kind.
///
/// **And the done transition needs no reading of a machine**: every unit's
/// terminal state is a fact about the arrangement record on the user's own
/// computer, so nothing here asks to join `Desktop app and delivery` P10d's
/// closed set of live readings.
pub const NO_NOTIFICATION_IS_POSTED_FOR_THE_OUTPUTS_BEING_HOME: bool = true;

/// **E4: two things end an arrangement without an ending ever arriving, and
/// one thing merely strands it, truthfully.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhatEndsItWithNoEnding {
    /// **A removed server ends it permanently.**
    TheServerWasRemoved,
    /// **A cancelled job takes the arrangement with it.**
    TheJobWasCancelled,
    /// **A machine whose installation is gone — recycled, reimaged — has no
    /// job record left to end**; the arrangement stands idle, fetches nothing,
    /// and ends by the user switching it off, removing the entry, or removing
    /// the server. **Nothing pretends an ending that never happened.**
    TheInstallationIsGoneAndItIsStranded,
}

impl WhatEndsItWithNoEnding {
    /// Whether the arrangement is ended by it at all, or merely stranded.
    pub fn it_ends_the_arrangement(self) -> bool {
        !matches!(
            self,
            WhatEndsItWithNoEnding::TheInstallationIsGoneAndItIsStranded
        )
    }
}

/// **E5: Row 214's damage is bounded by this section.**
///
/// A format the version could not deliver leaves narrowed entries that never
/// arrive — *never coming* at the ending walk — **so the arrangement still
/// ends, short and saying so, rather than waiting forever.**
pub const ROW_214S_DAMAGE_IS_BOUNDED_BY_THE_ENDING_WALK: bool = true;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::arrangement::tests::an_arrangement;
    use crate::arrangement::{AReckoningEntry, TheNarrowing};
    use crate::vouching::tests::an_order;

    fn seen(path: &[u8], size: u64) -> AWalkEntry {
        AWalkEntry {
            path: path.to_vec(),
            is_a_folder: false,
            size,
            modification_time: 1,
        }
    }

    /// **Nothing was placed**, which is the ordinary arrangement and the one
    /// N5a does not reach at all.
    fn nothing_was_placed() -> ThePlacedCopies<'static> {
        ThePlacedCopies {
            list: None,
            the_job_output_folder: b"/jobs/abc",
            the_four_values_now: &[],
        }
    }

    fn entry(path: &[u8], frame: Option<i64>) -> ANarrowedEntry {
        ANarrowedEntry {
            predicted_path: path.to_vec(),
            frame,
        }
    }

    #[test]
    fn a_narrowed_entry_absent_or_at_zero_bytes_becomes_never_coming() {
        let entries = vec![
            entry(b"f_0001.png", Some(1)),
            entry(b"f_0002.png", Some(2)),
            entry(b"f_0003.png", Some(3)),
        ];
        let walk = vec![seen(b"f_0001.png", 1024), seen(b"f_0002.png", 0)];
        let verdict = the_ending_walk(
            &entries,
            &[],
            &walk,
            Ending::Finished,
            &an_order(&[1, 2, 3]),
            &[],
            &nothing_was_placed(),
        );
        assert_eq!(verdict.fetchable, vec![b"f_0001.png".to_vec()]);
        assert_eq!(
            verdict.never_coming,
            vec![b"f_0002.png".to_vec(), b"f_0003.png".to_vec()]
        );
        // **Of those, the one the walk did not find at all** — the zero-byte
        // one was found (`Download` N6's last sentence, 2026-10-19).
        assert_eq!(verdict.not_found, vec![b"f_0003.png".to_vec()]);
        // A finished job marks nothing.
        assert!(verdict.could_not_be_vouched_for.is_empty());
    }

    /// **The clause this module's header is about.** A narrowed arrangement
    /// still brings home what the render wrote into an added write place, and
    /// this drives it rather than asserting it.
    ///
    /// **A file matching *no inventory entry at all* is the one this covers.**
    /// N4: it carries no format, so the cut cannot speak about it — it is
    /// outside the narrowing, meaning not filtered out by it.
    #[test]
    fn a_file_with_no_inventory_entry_comes_home_under_a_narrowed_arrangement_too() {
        let narrowed = vec![entry(b"f_0001.png", Some(1))];
        let walk = vec![
            seen(b"f_0001.png", 1024),
            seen(b"cache/fluid_0001.bphys", 4096),
        ];
        let verdict = the_ending_walk(
            &narrowed,
            &[],
            &walk,
            Ending::Finished,
            &an_order(&[1]),
            &[],
            &nothing_was_placed(),
        );
        assert_eq!(
            verdict.fetchable,
            vec![b"f_0001.png".to_vec(), b"cache/fluid_0001.bphys".to_vec()]
        );
        // The narrowing itself is a field of the arrangement and nothing in
        // this walk reads it, which is exactly the corrected reading.
        let _ = TheNarrowing::TheseFormats(vec![b"PNG".to_vec()]);
    }

    /// **The other half of the same walk, and it must come out the opposite
    /// way: a file the user narrowed away does not come home.**
    ///
    /// The EXR **has** a format and the user's choice did not pick it, so it
    /// is *skipped by narrowing* — out of the whole, silently correct. The
    /// cache file beside it has no format and comes home regardless. **The
    /// unmatched loop cannot tell those two apart without the excluded set**,
    /// and fetching both defeats the cut the user made: *"an artist who only
    /// wants a quick look should not have to drag heavy files across an entire
    /// animation."*
    #[test]
    fn a_file_the_user_narrowed_away_is_skipped_and_never_comes_home() {
        let narrowed = vec![entry(b"f_0001.png", Some(1))];
        let excluded = vec![entry(b"f_0001.exr", Some(1))];
        let walk = vec![
            seen(b"f_0001.png", 1024),
            seen(b"cache/fluid_0001.bphys", 4096),
            seen(b"f_0001.exr", 2048),
        ];
        let verdict = the_ending_walk(
            &narrowed,
            &excluded,
            &walk,
            Ending::Finished,
            &an_order(&[1]),
            &[],
            &nothing_was_placed(),
        );
        assert_eq!(
            verdict.fetchable,
            vec![b"f_0001.png".to_vec(), b"cache/fluid_0001.bphys".to_vec()]
        );
        assert!(!verdict.fetchable.contains(&b"f_0001.exr".to_vec()));
        assert_eq!(verdict.skipped_by_narrowing, vec![b"f_0001.exr".to_vec()]);
        // And it is not the shortfall either: it left the whole.
        assert!(!verdict.never_coming.contains(&b"f_0001.exr".to_vec()));
    }

    /// **N5a: a bake the user uploaded is not downloaded back.**
    ///
    /// *"Without this rule the user would download the bake they had just
    /// uploaded, which on a simulation cache is not a rounding error."* The
    /// test is Smoozy's own placement record and the four values it read when
    /// it made the copy — never the file, never its name, never whether it
    /// looks like a bake.
    #[test]
    fn a_copy_smoozy_placed_before_the_render_is_skipped_on_the_way_home() {
        use smoozy_dependencies::bytes::MachineLocation;
        use smoozy_dependencies::remap::{PlacedCopy, PlacedCopyList};
        use smoozy_server_protocol::transfers::SourceReading;

        let as_placed = SourceReading {
            size: 6_000_000_000,
            inode: "1".into(),
            modification_time: 10,
            change_time: 10,
        };
        let list = PlacedCopyList {
            copies: vec![PlacedCopy {
                job_folder_path: MachineLocation::composed(b"/jobs/abc/cache/fluid.bphys".to_vec()),
                four_values_at_placement: as_placed.clone(),
            }],
        };
        let placed = ThePlacedCopies {
            list: Some(&list),
            the_job_output_folder: b"/jobs/abc",
            the_four_values_now: &[(b"/jobs/abc/cache/fluid.bphys".to_vec(), as_placed)],
        };

        let narrowed = vec![entry(b"f_0001.png", Some(1))];
        let walk = vec![
            seen(b"f_0001.png", 1024),
            seen(b"cache/fluid.bphys", 6_000_000_000),
            seen(b"cache/result.bphys", 9_000),
        ];
        let verdict = the_ending_walk(
            &narrowed,
            &[],
            &walk,
            Ending::Finished,
            &an_order(&[1]),
            &[],
            &placed,
        );

        assert_eq!(
            verdict.skipped_as_the_copy_smoozy_placed,
            vec![b"cache/fluid.bphys".to_vec()]
        );
        // **Anything the render wrote beside it is not a path Smoozy placed
        // and comes home like everything else.**
        assert_eq!(
            verdict.fetchable,
            vec![b"f_0001.png".to_vec(), b"cache/result.bphys".to_vec()]
        );
        assert!(verdict.placed_paths_that_could_not_be_judged.is_empty());
    }

    /// **A copy the render wrote over is not that copy any more and comes
    /// home**, which is N5a's own rule and its safe failure direction.
    #[test]
    fn a_placed_copy_whose_values_have_moved_comes_home_like_everything_else() {
        use smoozy_dependencies::bytes::MachineLocation;
        use smoozy_dependencies::remap::{PlacedCopy, PlacedCopyList};
        use smoozy_server_protocol::transfers::SourceReading;

        let list = PlacedCopyList {
            copies: vec![PlacedCopy {
                job_folder_path: MachineLocation::composed(b"/jobs/abc/cache/fluid.bphys".to_vec()),
                four_values_at_placement: SourceReading {
                    size: 10,
                    inode: "1".into(),
                    modification_time: 10,
                    change_time: 10,
                },
            }],
        };
        let placed = ThePlacedCopies {
            list: Some(&list),
            the_job_output_folder: b"/jobs/abc",
            the_four_values_now: &[(
                b"/jobs/abc/cache/fluid.bphys".to_vec(),
                SourceReading {
                    size: 900,
                    inode: "1".into(),
                    modification_time: 99,
                    change_time: 99,
                },
            )],
        };

        let walk = vec![seen(b"cache/fluid.bphys", 900)];
        let verdict = the_ending_walk(
            &[],
            &[],
            &walk,
            Ending::Finished,
            &an_order(&[1]),
            &[],
            &placed,
        );
        assert_eq!(verdict.fetchable, vec![b"cache/fluid.bphys".to_vec()]);
        assert!(verdict.skipped_as_the_copy_smoozy_placed.is_empty());
    }

    /// **Where the four values cannot be read, nothing is skipped — and the
    /// walk says which paths it could not judge rather than letting it pass
    /// silently.**
    ///
    /// **That is today's state and it is a recorded absence**, not a gap:
    /// `Server protocol and transfers` S13's reading has no producer anywhere
    /// in this workspace. The direction the plan states is the safe one — the
    /// copy comes home, costing at worst a re-download of something the user
    /// already has.
    #[test]
    fn where_the_four_values_cannot_be_read_nothing_is_skipped_and_the_walk_says_so() {
        use smoozy_dependencies::bytes::MachineLocation;
        use smoozy_dependencies::remap::{PlacedCopy, PlacedCopyList};
        use smoozy_server_protocol::transfers::SourceReading;

        let list = PlacedCopyList {
            copies: vec![PlacedCopy {
                job_folder_path: MachineLocation::composed(b"/jobs/abc/cache/fluid.bphys".to_vec()),
                four_values_at_placement: SourceReading {
                    size: 10,
                    inode: "1".into(),
                    modification_time: 10,
                    change_time: 10,
                },
            }],
        };
        let placed = ThePlacedCopies {
            list: Some(&list),
            the_job_output_folder: b"/jobs/abc",
            // No reading is in hand for it, which is every placed path today.
            the_four_values_now: &[],
        };

        let walk = vec![seen(b"cache/fluid.bphys", 10)];
        let verdict = the_ending_walk(
            &[],
            &[],
            &walk,
            Ending::Finished,
            &an_order(&[1]),
            &[],
            &placed,
        );
        assert_eq!(verdict.fetchable, vec![b"cache/fluid.bphys".to_vec()]);
        assert!(verdict.skipped_as_the_copy_smoozy_placed.is_empty());
        assert_eq!(
            verdict.placed_paths_that_could_not_be_judged,
            vec![b"cache/fluid.bphys".to_vec()]
        );
    }

    #[test]
    fn a_badly_ended_job_marks_the_entries_of_the_highest_frame_the_order_shows() {
        let entries = vec![
            entry(b"f_0001.png", Some(1)),
            entry(b"f_0002.png", Some(2)),
            entry(b"f_0002.exr", Some(2)),
            entry(b"f_0003.png", Some(3)),
        ];
        let walk = vec![
            seen(b"f_0001.png", 1024),
            seen(b"f_0002.png", 900),
            seen(b"f_0002.exr", 900),
        ];
        for ending in [Ending::Stopped, Ending::Failed, Ending::GivenUp] {
            let verdict = the_ending_walk(
                &entries,
                &[],
                &walk,
                ending,
                &an_order(&[1, 2, 3]),
                &[],
                &nothing_was_placed(),
            );
            assert_eq!(
                verdict.could_not_be_vouched_for,
                vec![b"f_0002.png".to_vec(), b"f_0002.exr".to_vec()]
            );
            // The mark says nothing about what is fetched: those files are
            // fetched exactly as they are.
            assert!(verdict.fetchable.contains(&b"f_0002.png".to_vec()));
        }
    }

    #[test]
    fn where_the_frame_boundary_cannot_be_established_the_last_walks_active_entries_are_marked() {
        let entries = vec![entry(b"f_0001.png", Some(1))];
        let walk = vec![seen(b"f_0001.png", 1024)];
        let active = vec![b"f_0001.png".to_vec()];
        let verdict = the_ending_walk(
            &entries,
            &[],
            &walk,
            Ending::Stopped,
            &TheOrder::ItCouldNotBeRead,
            &active,
            &nothing_was_placed(),
        );
        assert_eq!(verdict.could_not_be_vouched_for, active);
    }

    /// The keys N6's whole counts, for an arrangement built by
    /// [`with_states`] — every entry it writes is a narrowed one.
    fn the_keys(states: &[WhatSmoozyDid]) -> Vec<Vec<u8>> {
        (0..states.len())
            .map(|at| format!("f_{at:04}.png").into_bytes())
            .collect()
    }

    fn with_states(states: &[WhatSmoozyDid]) -> AnArrangement {
        let mut arrangement = an_arrangement(b"/Users/a/Downloads");
        let destination = arrangement.destination.clone();
        for (at, state) in states.iter().enumerate() {
            arrangement.reckoning.push(AReckoningEntry::acted_on(
                format!("f_{at:04}.png").into_bytes(),
                Some(at as i64),
                state.clone(),
                destination.clone(),
            ));
        }
        arrangement
    }

    #[test]
    fn the_reading_says_ended_and_never_complete_and_is_short_where_it_is_short() {
        let states = [WhatSmoozyDid::Fetched, WhatSmoozyDid::NeverComing];
        let arrangement = with_states(&states);
        let reading = the_reading(&arrangement, 2, false, &the_keys(&states), true, true, true);
        assert!(reading.says_ended());
        assert!(reading.is_short());
        match reading {
            TheArrangementsReading::Ended {
                arrivals,
                whole,
                the_whole_may_grow,
                also_home_outside_the_whole,
                folders,
            } => {
                assert_eq!((arrivals, whole), (1, 2));
                assert!(!the_whole_may_grow);
                assert_eq!(also_home_outside_the_whole, 0);
                assert_eq!(folders, vec![b"/Users/a/Downloads".to_vec()]);
            }
            other => panic!("{other:?}"),
        }
    }

    #[test]
    fn an_arrangement_with_units_owed_and_no_connection_reads_as_waiting() {
        let states = [WhatSmoozyDid::Fetched, WhatSmoozyDid::Owed];
        let arrangement = with_states(&states);
        assert_eq!(
            the_reading(
                &arrangement,
                2,
                false,
                &the_keys(&states),
                false,
                true,
                false
            ),
            TheArrangementsReading::WaitingForAConnectionToThatServer
        );
        // With a connection standing the same arrangement is E2's reading.
        assert_eq!(
            the_reading(
                &arrangement,
                2,
                false,
                &the_keys(&states),
                true,
                true,
                false
            ),
            TheArrangementsReading::TheJobHasEndedAndFilesAreStillComingHome {
                arrivals: 1,
                whole: 2,
                the_whole_may_grow: false,
                also_home_outside_the_whole: 0
            }
        );
    }

    #[test]
    fn an_arrangement_switched_off_never_reads_as_waiting() {
        let states = [WhatSmoozyDid::Owed];
        let mut arrangement = with_states(&states);
        arrangement.switched_off();
        assert_eq!(
            the_reading(
                &arrangement,
                1,
                false,
                &the_keys(&states),
                false,
                true,
                false
            ),
            TheArrangementsReading::TheJobHasEndedAndFilesAreStillComingHome {
                arrivals: 0,
                whole: 1,
                the_whole_may_grow: false,
                also_home_outside_the_whole: 0
            }
        );
    }

    #[test]
    fn before_the_ending_the_reading_is_arrivals_against_the_whole() {
        let states = [WhatSmoozyDid::Fetched, WhatSmoozyDid::Owed];
        let arrangement = with_states(&states);
        assert_eq!(
            the_reading(
                &arrangement,
                5,
                false,
                &the_keys(&states),
                true,
                false,
                false
            ),
            TheArrangementsReading::ArrivalsAgainstTheWhole {
                arrivals: 1,
                whole: 5,
                the_whole_may_grow: false,
                also_home_outside_the_whole: 0
            }
        );
    }

    /// **N6, N4d and I14: what came home is counted over the entries the whole
    /// counts, and the rest is a second figure.**
    ///
    /// A job of ten frames whose file writes a simulation cache into an added
    /// write place brings that cache home too — N4 says it must, under every
    /// arrangement — **but those files are in no count**. Counting them with
    /// the frames read *310 of 10*.
    #[test]
    fn the_count_of_what_came_home_counts_only_files_the_whole_counts() {
        let mut arrangement = an_arrangement(b"/Users/a/Downloads");
        let destination = arrangement.destination.clone();
        let mut keys: Vec<Vec<u8>> = Vec::new();
        for frame in 0..10 {
            let key = format!("f_{frame:04}.png").into_bytes();
            keys.push(key.clone());
            arrangement.reckoning.push(AReckoningEntry::acted_on(
                key,
                Some(frame),
                WhatSmoozyDid::Fetched,
                destination.clone(),
            ));
        }
        // The cache the render wrote into an added write place: home, in no
        // count, and with no frame.
        for at in 0..300 {
            arrangement.reckoning.push(AReckoningEntry::acted_on(
                format!("cache/fluid_{at:04}.bphys").into_bytes(),
                None,
                WhatSmoozyDid::Fetched,
                destination.clone(),
            ));
        }

        match the_reading(&arrangement, 10, false, &keys, true, true, true) {
            TheArrangementsReading::Ended {
                arrivals,
                whole,
                also_home_outside_the_whole,
                ..
            } => {
                assert_eq!((arrivals, whole), (10, 10));
                assert_eq!(also_home_outside_the_whole, 300);
            }
            other => panic!("{other:?}"),
        }
    }

    /// **E2a: the morning after.**
    ///
    /// Auto-download was on at submission, Smoozy was closed for the night and
    /// the job ended. The user opens Smoozy and has not connected to that
    /// server: **the record is empty, the ending is in hand, and the ending
    /// walk has not run.** *Every unit is terminal* is true of a record with
    /// nothing in it, so without the walk's own fact the reading said **Ended
    /// — 0 of 720**, which a finished bar draws as a job whose outputs are all
    /// home. **What that costs is a user destroying the machine the frames are
    /// still on.**
    #[test]
    fn an_empty_record_with_the_ending_in_hand_and_no_walk_reads_as_waiting_and_never_as_ended() {
        let arrangement = with_states(&[]);
        assert!(arrangement.reckoning.is_empty());
        assert!(every_unit_is_terminal(&arrangement));

        assert_eq!(
            the_reading(&arrangement, 720, false, &[], false, true, false),
            TheArrangementsReading::WaitingForAConnectionToThatServer
        );
        // And with a connection standing but the walk still to run, it is E2's
        // reading rather than *ended*.
        assert_eq!(
            the_reading(&arrangement, 720, false, &[], true, true, false),
            TheArrangementsReading::TheJobHasEndedAndFilesAreStillComingHome {
                arrivals: 0,
                whole: 720,
                the_whole_may_grow: false,
                also_home_outside_the_whole: 0
            }
        );
    }

    /// **The milder half needs no closed app**: an ending arriving live while
    /// files are still on the machine must not read *ended* before the walk
    /// has found them.
    #[test]
    fn a_live_ending_never_reads_as_ended_before_the_ending_walk_has_completed() {
        let states = [WhatSmoozyDid::Fetched, WhatSmoozyDid::Fetched];
        let arrangement = with_states(&states);
        let keys = the_keys(&states);
        assert!(every_unit_is_terminal(&arrangement));
        assert!(!the_reading(&arrangement, 720, false, &keys, true, true, false).says_ended());
        assert!(the_reading(&arrangement, 720, false, &keys, true, true, true).says_ended());
    }

    #[test]
    fn done_needs_both_the_ending_and_every_unit_terminal() {
        let states = [WhatSmoozyDid::Fetched, WhatSmoozyDid::LeftAlone];
        let all_terminal = with_states(&states);
        let keys = the_keys(&states);
        assert!(every_unit_is_terminal(&all_terminal));
        // Terminal but no ending: the arrangement is not done.
        assert!(!the_reading(&all_terminal, 2, false, &keys, true, false, true).says_ended());
        assert!(the_reading(&all_terminal, 2, false, &keys, true, true, true).says_ended());
    }

    #[test]
    fn the_reading_names_both_folders_where_a_reset_split_the_job() {
        let mut arrangement = with_states(&[WhatSmoozyDid::Fetched]);
        arrangement.switched_off();
        arrangement.switched_on_again(b"/Volumes/work".to_vec(), TheNarrowing::Everything);
        let destination = arrangement.destination.clone();
        arrangement.reckoning.push(AReckoningEntry::acted_on(
            b"f_0009.png".to_vec(),
            Some(9),
            WhatSmoozyDid::Fetched,
            destination,
        ));
        match the_reading(
            &arrangement,
            2,
            false,
            &[b"f_0000.png".to_vec(), b"f_0009.png".to_vec()],
            true,
            true,
            true,
        ) {
            TheArrangementsReading::Ended { folders, .. } => assert_eq!(
                folders,
                vec![b"/Users/a/Downloads".to_vec(), b"/Volumes/work".to_vec()]
            ),
            other => panic!("{other:?}"),
        }
    }

    #[test]
    fn only_the_recycled_installation_strands_the_arrangement_rather_than_ending_it() {
        assert!(WhatEndsItWithNoEnding::TheServerWasRemoved.it_ends_the_arrangement());
        assert!(WhatEndsItWithNoEnding::TheJobWasCancelled.it_ends_the_arrangement());
        assert!(
            !WhatEndsItWithNoEnding::TheInstallationIsGoneAndItIsStranded.it_ends_the_arrangement()
        );
    }

    #[test]
    fn a_finished_ending_marks_nothing_and_the_other_three_mark_the_frame_in_flight() {
        assert!(!the_ending_marks_the_frame_that_was_in_flight(
            Ending::Finished
        ));
        for ended_short in [Ending::Failed, Ending::Stopped, Ending::GivenUp] {
            assert!(the_ending_marks_the_frame_that_was_in_flight(ended_short));
        }
    }
}
