//! **§7 E1 performed, and the sequence that carries every other act — one
//! arrangement, run.**
//!
//! The walk turn, the two vouching mechanisms applied to what it read, the
//! name check at the moment a file first becomes fetchable, the units owed to
//! the queue, the ending walk when the ending arrives, and the durable write
//! at each moment something became true.
//!
//! # This module decides nothing; it orders
//!
//! Every judgement below is asked of the rules half —
//! [`crate::vouching`], [`crate::narrowing`], [`crate::ending`],
//! [`crate::arrangement`] — and every act is asked of the acting half's own
//! modules. **What is here is the order**, and the order is the part the plan
//! states and the part a builder would otherwise invent.
//!
//! # E1's four steps, in E1's order
//!
//! 1. **The ending arrives** — one of the four, live or at catch-up. **The
//!    module never finds out on its own**, so [`TheArrangementsRun`] is told
//!    and holds it; nothing here reads a machine for it.
//! 2. **The ending walk runs** — one final complete walk of the job output
//!    folder, and [`crate::ending::the_ending_walk`]'s verdict applied.
//! 3. **The fetch queue drains** to terminal states, under the ordinary
//!    pipeline, surviving drops and destination trouble on the way.
//! 4. **The arrangement is done when every unit is terminal.**
//!
//! **A discarded ending walk is not step 2**, and this is the one place that
//! matters most: a walk that did not complete is not a reading (W5), so
//! applying its verdict would put every frame of the job into *never coming*
//! on a refused listing. **So the ending walk is retried at the ordinary
//! cadence until one completes**, and the arrangement is not done in the
//! meantime.
//!
//! # Why the window closes where it does
//!
//! **W3: the window closes at the completion of the ending walk**, because the
//! arrangement's own bookkeeping for that attempt is finished — **not because
//! the folder can never change again.** And **done is not for ever**: a new
//! render-start re-arms the arrangement, re-opening its *never coming* entries
//! for the frames the new attempt's list names, and this sequence runs again
//! to a second done.
//!
//! # Every durable write is at the moment the thing became true
//!
//! R1's write takes the whole record, so *"every later change — the desired
//! state, the narrowing, the destination at a switch-on, an entry's account,
//! an entry's four-value reading — goes down through the same call, at the
//! moment it becomes true."* **This module writes after each step that changed
//! the record and never once at the end of a turn**, because a crash between
//! two steps must lose at most a judgement the next walk re-derives (R5) and
//! never an act Smoozy took.
//!
//! **Markers used in this file that belong to another document, each with the
//! document it belongs to.** `Queue & History` C1. `Server protocol and
//! transfers` S8, S13, S26. `Desktop app and delivery` C3, C10, F1, F9.

use std::time::Duration;

use smoozy_queue_history::ending::Ending;
use smoozy_server_protocol::subsystem::SubsystemFailure;
use smoozy_state::identities::Identity;
use smoozy_state::outcomes::Refused;

use crate::arrangement::{AReckoningEntry, AnArrangement, KeepsTheArrangement, WhatSmoozyDid};
use crate::ending::{every_unit_is_terminal, ANarrowedEntry, TheEndingWalksVerdict};
use crate::fetching::{judge_the_landing_path, TheReceivingSystemsRules};
use crate::vouching::{
    a_judged_finished_frame_is_fetchable, what_the_catch_up_vouches_for,
    what_the_live_walk_vouches_for, AnExpectedFile, HowTheSignalArrives, TheCatchUpsSignal,
    TheOrder,
};
use crate::watch::{AWalkEntry, TheWindow, WhyTheWalkIsNotAReading};

use super::fetching::{AUnitOwed, TheFetchQueue};
use super::walking::{ReadsOneDirectoryOnTheMachine, TheWalkSeries, WhatTheTurnCameTo};

/// **One arrangement's own run-held state.**
///
/// **R11: the arrangement's *activity* is run-held and dies with the process;
/// the record does not.** Everything in here is that activity — the walk
/// series, the ending in hand, whether the ending walk has completed — and
/// **none of it is durable.** A fresh run re-walks, re-derives what is owed,
/// and reads the reckoning for what is already taken.
#[derive(Debug, Default)]
pub struct TheArrangementsRun {
    series: TheWalkSeries,
    /// `Download` J6: a saved prefix from this connection's matching attempt.
    the_reported_saved_frame: Option<i64>,
    /// **E1 step 1: the ending, once it has arrived.** `Queue & History`'s,
    /// carried here and never decided here.
    the_ending: Option<Ending>,
    /// **E1 step 2: whether one *complete* ending walk has run.** A discarded
    /// one does not set it.
    the_ending_walk_has_completed: bool,
    /// **The entries the complete ending walk listed as plain files, kept
    /// beside the series and not in it** — for their sizes. Added 2026-09-30,
    /// step 10c's eleventh-pass fix piece 2. A symbolic link's listed size is
    /// the link's own, so a link is not in here and is sized as before.
    ///
    /// **W5 keeps the series for the change detection**, so the ending walk is
    /// taken outside it — and its sizes used to go with it, so every unit a
    /// fresh connection's ending walk made owed had its size read again, one
    /// round trip each inside the server's point, for a number the walk had
    /// just read. **Kept for exactly as long as the walk counts as completed**:
    /// set with [`Self::the_ending_walk_has_completed`] and cleared with it at
    /// a re-arm. Run-held like the rest (R11), and never a judgement.
    the_ending_walks_plain_files: Vec<AWalkEntry>,
    /// **J6: the frames seen active since this connection's baseline**, which
    /// is mechanism two's cumulative set and is dropped with the connection.
    active_since_the_baseline: Vec<i64>,
    /// **J6: whether mechanism two has already fired.** It is one-time by
    /// design — *"this rule is right exactly once and wrong as a standing
    /// frontier"*.
    the_catch_up_has_judged: bool,
    /// **What this run has already put in the fetch queue**, so a later walk
    /// does not offer the same file a second time.
    ///
    /// **It is run-held on purpose and it is not the reckoning.** The
    /// reckoning answers *what have I taken* (R5) and an entry standing at
    /// *owed* is not taken — so a fresh run must re-arrange it, which is
    /// exactly R5's *"a crash loses at most a judgement that the next walk
    /// re-derives"*. **A durable *this is already queued* field would make a
    /// crash lose the fetch for good**, because nothing would ever offer it
    /// again.
    already_offered: Vec<Vec<u8>>,
}

/// What one turn of an arrangement came to.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WhatTheTurnDid {
    /// **W3: the window is shut**, so nothing ran.
    TheWindowIsShut,
    /// A complete walk, with the paths that became fetchable at it.
    ItWalked {
        became_fetchable: Vec<Vec<u8>>,
        pause_before_the_next: Duration,
    },
    /// **W5: the walk was discarded and nothing changed.** The next one runs
    /// at the cadence; nothing gives up.
    ///
    /// **The cause is carried, and W5 is why**: *"a refusal reading the job's
    /// own folder is reported as the truth — this document could not read the
    /// job's folder, with the separated cause where the route separates it."*
    /// The layer below already carries both and dropping them here left the
    /// arrangement with nothing to say while its walks were discarded at the
    /// cadence for ever. **Smoozy going quiet is always Smoozy's fault.**
    TheWalkWasDiscarded {
        why: WhyTheWalkIsNotAReading,
        /// **The protocol's own status, carried as itself** — the separated
        /// cause, never a sentence composed here.
        ///
        /// **Absent on the one discard the machine said nothing about** — an
        /// entry whose kind the reply did not carry, where the listing arrived
        /// and one of its entries did not say what it was.
        what_the_machine_said: Option<SubsystemFailure>,
        pause_before_the_next: Duration,
    },
    /// **E1 step 2: the ending walk completed and its verdict was applied.**
    TheEndingWalkRan(Box<TheEndingWalksVerdict>),
    /// **E1 step 4: every unit is terminal.**
    ItIsDone,
}

/// **What one unit's fetch came to, as the caller of the transfer hands it
/// back.**
///
/// **Every field is somebody else's answer, carried.** The ending is
/// `Server protocol and transfers` S26's five; the taken name is
/// `Desktop app and delivery` C10's exclusive create as P6 reads it; the write
/// failure is that document's F1 to F9 classes. **Nothing here is produced
/// from a byte, a status, an error number or a symbol.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WhatTheUnitsFetchCameTo {
    /// **No transfer was ever arranged: the path could not be carried into the
    /// transfer record's own text.**
    ///
    /// The enumeration holds paths as bytes and the record holds them as text,
    /// and **nothing lossily converts** — which is `File Browser`'s own refusal
    /// in the other direction, word for word.
    TheNameCouldNotBeCarried(super::fetching::TheNameCannotBeCarried),
    /// The transfer ran under its own identity and ended one of S26's five.
    ItEnded {
        /// **R4's account half: the transfer identity the fetch was arranged
        /// under**, written whatever the transfer ended as.
        transfer: smoozy_state::identities::Identity,
        ending: smoozy_server_protocol::transfers::Ending,
        /// **P6: what the exclusive create found the final name taken by**,
        /// where the ending was a publication refusal. **Absent where the stop
        /// was not one.**
        taken_by: Option<crate::fetching::WhatTheTakenNameIs>,
        /// **D1's class where the local write failed**, which is what raises a
        /// destination condition. **Absent where none did.**
        the_write_failed: Option<crate::destination::WhatTheAdapterSaid>,
    },
}

/// **What followed from one unit's fetch ending.**
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct WhatFollowedFromTheFetch {
    /// The state the entry now stands at, **or nothing where the unit went
    /// back to *owed*** and nothing terminal was written.
    pub the_entry_now_stands_at: Option<WhatSmoozyDid>,
    /// **D4's condition, where a write failure raised one.** The queue's hold
    /// is already taken; the probe's cadence is the caller's and is handed the
    /// same key.
    pub a_destination_condition_was_raised: Option<crate::destination::TheConditionKey>,
    /// **The path that could not be carried**, so the telling can say which of
    /// the record's two paths it was and name it.
    pub the_name_that_could_not_be_carried: Option<super::fetching::TheNameCannotBeCarried>,
    /// **§12's act, composed at the moment the arrangement became done** and
    /// at no other moment.
    pub the_diary_act: Option<crate::diary::AnAct>,
}

/// **P6 applied, and what a stop with no publication refusal behind it
/// leaves.**
///
/// **A publication refusal is classified by P6 and by nothing else.** The
/// three classifications it can reach at fetch time are *left alone*, *cannot
/// land*, and the newer-version telling that keeps the entry at *fetched*; its
/// fourth arm cannot arise here, because R6's ordering requirement has every
/// publication-pending record completed or its reservation removed before this
/// document admits any transfer, and the entry is left as it stands for it.
///
/// **A stop that was not a publication refusal is written *left alone*.** The
/// transfer ended *stopped*, which is an ending Smoozy wrote itself, so
/// nothing re-arranges it — and *left alone* is the one terminal state that
/// says exactly that: the file was passed over, it stays on the machine, the
/// arrangement will not fetch over the name again, and P6's amber line names
/// it. **Leaving it at *owed* instead would make E1 step 4's *every unit is
/// terminal* unreachable**, which is the shape this whole step exists to
/// close.
fn the_state_a_stop_leaves(taken_by: Option<crate::fetching::WhatTheTakenNameIs>) -> WhatSmoozyDid {
    let Some(taken_by) = taken_by else {
        return WhatSmoozyDid::LeftAlone;
    };
    match crate::fetching::what_the_refusal_means(&taken_by) {
        crate::fetching::WhatTheRefusalMeans::LeftAlone => WhatSmoozyDid::LeftAlone,
        crate::fetching::WhatTheRefusalMeans::CannotLand => WhatSmoozyDid::CannotLand,
        crate::fetching::WhatTheRefusalMeans::NothingIsWrittenOverAndItIsToldAsANewerVersion => {
            WhatSmoozyDid::Fetched
        }
        // **Cannot arise at fetch time**, and the entry is left as it stands
        // rather than given a state invented for a case R6 forbids.
        crate::fetching::WhatTheRefusalMeans::CannotAriseAtFetchTime => WhatSmoozyDid::LeftAlone,
    }
}

/// **A first-frame check and its range job, by identity** (`Rendering` S13),
/// as the local record pairs them. Added 2026-10-31, the twenty-first notes,
/// note 6.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TheFirstFramePair<'a> {
    pub the_check: &'a Identity,
    pub the_range: &'a Identity,
}

impl TheFirstFramePair<'_> {
    /// **P10's dated note: the pair lands under the range job's name.**
    pub fn the_job_whose_folder_it_lands_in<'b>(
        pair: Option<TheFirstFramePair<'b>>,
        job: &'b Identity,
    ) -> &'b Identity {
        match pair {
            Some(pair) => pair.the_range,
            None => job,
        }
    }

    /// **Whether the range job's ending walk leaves this file to the check**:
    /// the arrangement is the range job's, no entry of its inventory names the
    /// file, and its name starts with the check's mark (`Job files and
    /// outputs` P18a). The check's own arrangement brought it home; fetched
    /// again here, its landing name would be taken and the range job would say
    /// it left the file behind.
    pub fn the_range_leaves_it_to_the_check(
        pair: Option<TheFirstFramePair<'_>>,
        job: &Identity,
        named_by_the_inventory: bool,
        path: &[u8],
    ) -> bool {
        let Some(pair) = pair else {
            return false;
        };
        let name = path.rsplit(|byte| *byte == b'/').next().unwrap_or(path);
        pair.the_range == job
            && !named_by_the_inventory
            && name.starts_with(smoozy_job_files::reading_script::THE_CHECKS_MARK.as_bytes())
    }

    /// **Whether the check's ending walk leaves this file to the range job** —
    /// the mirror of [`Self::the_range_leaves_it_to_the_check`]: the
    /// arrangement is the check's, no entry of its inventory names the file,
    /// and its name does **not** start with the check's mark. **Every file the
    /// check writes is marked** (`Job files and outputs` P18a), so such a file
    /// is the range job's. `Download` E1 step 2's dated note of 2026-10-31,
    /// the twenty-sixth notes, note 4: a check whose ending walk ran again
    /// after a restart took the range job's frames as its own.
    pub fn the_check_leaves_it_to_the_range(
        pair: Option<TheFirstFramePair<'_>>,
        job: &Identity,
        named_by_the_inventory: bool,
        path: &[u8],
    ) -> bool {
        let Some(pair) = pair else {
            return false;
        };
        let name = path.rsplit(|byte| *byte == b'/').next().unwrap_or(path);
        pair.the_check == job
            && !named_by_the_inventory
            && !name.starts_with(smoozy_job_files::reading_script::THE_CHECKS_MARK.as_bytes())
    }
}

/// What the caller supplies about the job at this moment, so nothing in this
/// module reads a machine for it.
pub struct WhatIsInHand<'a> {
    /// The job output folder, as the manifest names it.
    pub the_job_output_folder: &'a [u8],
    /// **The narrowed inventory entries, keyed as this document keys them** —
    /// [`super::record_directory::what_the_narrowing_leaves`]'s kept half,
    /// which is N6's whole.
    pub narrowed_entries: &'a [ANarrowedEntry],
    /// **N6's second half, from the same function** —
    /// [`super::record_directory::WhatTheNarrowingLeaves::it_may_grow`], which
    /// is [`crate::narrowing::the_whole`]'s video statement over that same
    /// cut.
    ///
    /// **It travels beside the entries because the whole is both figures**:
    /// a video destination's entry counts one and the whole is stated as
    /// possibly growing, and a reading that carried only the count would state
    /// half of N6.
    pub the_whole_may_grow: bool,
    /// **The inventory entries the arrangement's narrowing excludes** — the
    /// same function's other half.
    ///
    /// **They are here because the ending walk cannot do its job without
    /// them**: a walk file matching one of these is *skipped by narrowing* and
    /// must not come home, while a walk file matching no inventory entry at
    /// all is J8's added-place or unmatched file and comes home under every
    /// arrangement. Nothing else tells those two apart.
    ///
    /// **At a switch-on this is cut with the *new* narrowing**, because that
    /// is the choice the record is then brought into line with.
    pub the_entries_the_narrowing_excludes: &'a [ANarrowedEntry],
    /// **J2's order for the attempt in hand.** A value, never a range.
    pub order: &'a TheOrder,
    /// **The job's label, for P10's landing folder name.**
    ///
    /// **Absent is a value**: P10 falls back to the bare job identity, which
    /// is safe in every system's unit, and the folder is still the job's own.
    /// The full label stays authoritative on the queue entry
    /// (`Queue & History` Y8) and **nothing parses a folder name back into
    /// anything.**
    pub the_jobs_label: Option<&'a str>,
    /// **The first-frame pair this job is one of, where it is one** — the
    /// check and its range job write into one folder on the machine (`Job
    /// files and outputs` P18a), so **both land under the range job's landing
    /// name** (P10's dated note), and the range job's ending walk leaves the
    /// check's marked files to the check, which brought them home already.
    /// **Absent is a value**: the job's own landing name, every file of the
    /// walk as before. Added 2026-10-31, the twenty-first notes, note 6.
    pub the_first_frame_pair: Option<TheFirstFramePair<'a>>,
    /// **The predicted paths the pair's other half holds *fetched* into this
    /// arrangement's destination** — one folder, so the same landing path.
    /// E1 step 2's dated note of 2026-10-31: such a file is taken and nothing
    /// is fetched. **Empty is a value**: no pair, or nothing fetched by the
    /// other half. The twenty-sixth notes, note 4.
    pub the_pairs_other_half_fetched: &'a [Vec<u8>],
    /// **W3's three conditions**, asked of the caller because two of them —
    /// the desired state and the standing connection — are not this module's
    /// to know.
    pub window: TheWindow,
    /// **N5a: `Dependency Upload & Remap` M22f's placed-copy list**, out of
    /// the job's own local record, so a bake the user uploaded is not
    /// downloaded back.
    ///
    /// **Absent is a value**: where the placement record cannot be read
    /// nothing is skipped and the folder comes home whole, which is the safe
    /// direction the clause states.
    pub the_placed_copies: Option<&'a smoozy_dependencies::remap::PlacedCopyList>,
    /// **`Server protocol and transfers` S13's four values for each placed
    /// path as they read now**, which is what tells the copy Smoozy placed
    /// from one the render wrote over.
    ///
    /// **There is no producer for that reading anywhere in this workspace**
    /// and none is invented here, so this is empty today: no placed copy is
    /// judged, none is skipped, and each one is named on the verdict as a path
    /// that could not be judged.
    pub the_four_values_of_the_placed_copies:
        &'a [(Vec<u8>, smoozy_server_protocol::transfers::SourceReading)],
    /// How many arrangements are walking this server, which is W2's aggregate
    /// bound's own input.
    pub arrangements_walking_this_server: usize,
}

impl TheArrangementsRun {
    pub fn fresh() -> TheArrangementsRun {
        TheArrangementsRun::default()
    }

    /// The caller checked this connection, job and attempt (`Download` J6).
    /// Absence supplies no evidence; neither a count nor a range stands in.
    pub fn the_saved_prefix_was_reported(&mut self, frame: Option<i64>) {
        self.the_reported_saved_frame = frame;
    }

    /// **E1 step 1: the ending arrives — one of the four, live or at
    /// catch-up.** The module never finds out on its own.
    pub fn the_ending_arrived(&mut self, ending: Ending) {
        self.the_ending = Some(ending);
    }

    /// **E1 step 2: whether one complete ending walk has run**, which is the
    /// fact [`crate::ending::the_reading`] needs before it may say *ended*.
    ///
    /// **It is run-held like the rest of the activity** (R11), so a fresh run
    /// that has not walked yet does not say a job is finished — which is the
    /// morning E2a was written for.
    pub fn the_ending_walk_has_completed(&self) -> bool {
        self.the_ending_walk_has_completed
    }

    /// **R5's dated note of 2026-10-31: a fresh run told that the ending walk
    /// is written down as completed for this arrangement's start**, by a caller
    /// that holds the record. **Taken only where every unit is terminal**:
    /// where one is still owed, only a walk re-offers it, so the run walks once
    /// more as before. The twenty-sixth notes, note 4.
    pub fn the_ending_walk_was_written_down(&mut self, arrangement: &AnArrangement) {
        if every_unit_is_terminal(arrangement) {
            self.the_ending_walk_has_completed = true;
        }
    }

    /// **The last complete walk**, so a caller arranging a fetch can take the
    /// file's size from the reading that made it fetchable.
    ///
    /// **It is a reading and stays one** (R5, W5): it is period-held, it is
    /// never written down, and nothing here derives a judgement from it. A
    /// caller reads a size off it because `Server protocol and transfers` S9's
    /// record carries the expected total and **a fetch arranged with a size of
    /// nothing would draw a bar against nothing** — a second reading of the
    /// same folder to get the same number would be a second answer to a
    /// question the walk already answered.
    ///
    /// **A discarded walk never becomes one**, so what this hands back is
    /// always a complete reading or nothing.
    pub fn the_last_complete_walk(&self) -> &[AWalkEntry] {
        self.series.the_baseline()
    }

    /// **What a walk read of a unit's key**, whose size is what a fetch is
    /// arranged against. Added 2026-09-30, step 10c's eleventh-pass fix
    /// piece 2.
    ///
    /// **The complete ending walk's entry first**, being the latest reading of
    /// a finished job's folder, and only where it listed a plain file; **then
    /// the last complete walk's**, which was the only answer before. **Nothing
    /// where neither walk read the key.** It hands back the entry and reads no
    /// size itself: what the caller does with the size is the caller's, and
    /// §4's closure keeps every size-reading line in this crate to the one
    /// rule.
    pub fn what_a_walk_read_of(&self, key: &[u8]) -> Option<&AWalkEntry> {
        self.the_ending_walks_plain_files
            .iter()
            .find(|entry| entry.path == key)
            .or_else(|| {
                self.series
                    .the_baseline()
                    .iter()
                    .find(|entry| entry.path == key)
            })
    }

    /// **J6: on every (re)connection the baseline starts again**, and
    /// mechanism two may judge once more on the new connection.
    pub fn a_connection_began(&mut self) {
        self.series.a_connection_began();
        self.the_reported_saved_frame = None;
        self.active_since_the_baseline.clear();
        self.the_catch_up_has_judged = false;
    }

    /// **W3: a new render-start re-arms a done arrangement.**
    ///
    /// It opens a fresh window and the walks resume; the reckoning's *never
    /// coming* entries re-open to *owed* for the frames the new attempt's list
    /// names; **nothing else re-opens**, which is what keeps a re-run from
    /// arriving over the user's copies.
    ///
    /// **The write is here rather than at the end of the turn**, because a
    /// crash between the re-opening and the next walk must not lose the
    /// re-opening.
    pub fn a_render_start_re_armed_it<K: KeepsTheArrangement + ?Sized>(
        &mut self,
        keeper: &mut K,
        arrangement: &mut AnArrangement,
        the_attempts_frames: &[i64],
    ) -> Result<(), Refused> {
        arrangement.a_render_start_re_arms_it(the_attempts_frames);
        self.the_ending = None;
        self.the_reported_saved_frame = None;
        self.the_ending_walk_has_completed = false;
        self.the_ending_walks_plain_files.clear();
        // **The re-armed attempt is a fresh window**, so what the previous
        // attempt offered says nothing about this one: a frame re-opened to
        // *owed* has to be offerable again.
        self.already_offered.clear();
        keeper.write_it_down(arrangement)
    }

    /// **E1 step 3, first half: one granted unit composed as one transfer
    /// under `Server protocol and transfers` §7.**
    ///
    /// **The record is the contract's and this composes it rather than a
    /// second one of its own** — the transfer identity, the server row and its
    /// installation, the path on the machine, the landing path the unit has
    /// carried since it was owed, the staging directory inside the user's own
    /// destination folder, the size and the run.
    ///
    /// **The transfer identity is handed in.** A fresh fetch is a new transfer
    /// with a new identity (`State, identity and recovery` §9.5), and issuing
    /// one is that document's act and not this module's.
    ///
    /// **The path on the machine is the job output folder plus the unit's own
    /// key**, which is the reckoning's key — the predicted path relative to
    /// that folder — so the two spellings are joined in the one place that
    /// knows both.
    ///
    /// **A path that is not valid text refuses here and is never mangled.**
    /// The refusal is handed straight to [`TheArrangementsRun::a_fetch_ended`],
    /// which now has a state for it.
    #[allow(
        clippy::too_many_arguments,
        reason = "every one is a value the contract's own record takes, and \
                  `arrange_one_fetch` carries the same allow for the same \
                  reason: gathering them would be a second shape for the one \
                  `Server protocol and transfers` already fixes"
    )]
    pub fn arrange_the_fetch_for(
        &self,
        arrangement: &AnArrangement,
        unit: &AUnitOwed,
        the_job_output_folder: &[u8],
        transfer: smoozy_state::identities::Identity,
        installation: Option<smoozy_state::identities::Identity>,
        size: u64,
        written_by_run: smoozy_state::identities::Identity,
    ) -> Result<
        smoozy_server_protocol::transfers::TransferRecord,
        super::fetching::TheNameCannotBeCarried,
    > {
        let mut on_the_machine = the_job_output_folder.to_vec();
        if !on_the_machine.ends_with(b"/") {
            on_the_machine.push(b'/');
        }
        on_the_machine.extend_from_slice(&unit.key);
        super::fetching::arrange_one_fetch(
            transfer,
            arrangement.server_row.clone(),
            installation,
            &on_the_machine,
            &unit.the_landing_path,
            &unit.the_destination_normalised,
            size,
            written_by_run,
        )
    }

    /// **E1 step 3: one unit's fetch has ended, and everything that follows
    /// from it.**
    ///
    /// **This is the return journey, and without it the pipeline is one-way.**
    /// The walk vouches, the name check judges, the unit joins the queue and
    /// the queue offers it — and nothing took the answer back. As the crate
    /// stood before this, no arrangement could ever be done, no destination
    /// condition could ever be raised, no diary line was ever written, and
    /// P6's classification of a taken name was never applied: auto-download
    /// queued every judged frame and fetched none.
    ///
    /// **Every judgement here is the rules half's, asked and not re-taken.**
    /// [`super::fetching::what_the_fetch_came_to`] reads S26's ending,
    /// [`crate::fetching::what_the_refusal_means`] classifies a publication
    /// refusal, and [`crate::destination::the_condition_key`] composes D4's
    /// key. **Nothing here reads a status, a byte count or a digest.**
    ///
    /// **The slot goes back whatever the unit ended as** — it is a slot in
    /// this direction's ceiling — and **P3 decides whether the unit itself
    /// comes back**: for an arrangement a server-side or transfer failure
    /// returns it to *owed* and it is re-arranged at a later walk cycle, as a
    /// new transfer with a new identity.
    ///
    /// **Two seams stay seams and neither blocks this**: the adapter's own
    /// judgements and the durable store are recorded absences
    /// ([`crate::arriving_later`]), and what is written here is written
    /// against the values they will fill.
    #[allow(
        clippy::too_many_arguments,
        reason = "this is the sequencer's own step and every argument is a \
                  different one of the things E1 step 3 acts on — the store, \
                  the queue, the probe's cadence, the record, what is in hand, \
                  the unit and what its fetch came to. Gathering them would \
                  make one shape whose only reader is this function"
    )]
    pub fn a_fetch_ended<K>(
        &mut self,
        keeper: &mut K,
        queue: &mut TheFetchQueue,
        probing: &mut super::probing::TheProbesCadence,
        arrangement: &mut AnArrangement,
        in_hand: &WhatIsInHand<'_>,
        unit: AUnitOwed,
        came_to: WhatTheUnitsFetchCameTo,
    ) -> Result<WhatFollowedFromTheFetch, Refused>
    where
        K: KeepsTheArrangement + ?Sized,
    {
        let key = unit.key.clone();
        let mut followed = WhatFollowedFromTheFetch::default();

        match came_to {
            // **The refusal itself is right and is not softened**: a path that
            // is not valid text is never mangled into the transfer record. What
            // it lacked was a state to land in — such a unit sat at *owed* for
            // ever and the arrangement could never be done. **It is *cannot
            // land***: the unit is terminal, it leaves the whole, it is
            // reported separately, and nothing later makes the name carryable.
            WhatTheUnitsFetchCameTo::TheNameCouldNotBeCarried(which) => {
                queue.a_unit_ended();
                self.write_the_entry(arrangement, &key, in_hand, WhatSmoozyDid::CannotLand);
                followed.the_name_that_could_not_be_carried = Some(which);
                followed.the_entry_now_stands_at = Some(WhatSmoozyDid::CannotLand);
            }
            WhatTheUnitsFetchCameTo::ItEnded {
                transfer,
                ending,
                taken_by,
                the_write_failed,
            } => {
                // **D4: one condition, keyed on the refusal's kind and the
                // destination folder's own normalised path**, raised from the
                // adapter's own class and never from a byte or an error
                // number. **The hold is the queue's and reaches this
                // arrangement's units alone** (D6), and **the probe's cadence
                // starts with it** (D7) — the arrangement then keeps only its
                // watch and its probe, and D8's next successful write is what
                // withdraws the condition and nothing else.
                if let Some(class) = the_write_failed {
                    let key = crate::destination::the_condition_key(
                        class,
                        arrangement.destination.clone(),
                    );
                    queue.a_condition_was_raised(key.clone());
                    probing.a_condition_was_raised();
                    followed.a_destination_condition_was_raised = Some(key);
                }
                // **P6's built note of 2026-10-31: a publication refused for
                // a taken name ends by P6, whatever the transfer record's
                // ending.** `Server protocol and transfers` §7 reads the
                // refused create as a local failure and ends it *failed*,
                // which alone is owed again (P3) — so the same file crossed
                // whole at every pass. The twenty-sixth notes, note 4.
                let state = match super::fetching::what_the_fetch_came_to(&ending) {
                    _ if taken_by.is_some() => {
                        queue.a_unit_ended();
                        Some(the_state_a_stop_leaves(taken_by))
                    }
                    super::fetching::WhatTheFetchCameTo::ItIsFetched => {
                        queue.a_unit_ended();
                        Some(WhatSmoozyDid::Fetched)
                    }
                    super::fetching::WhatTheFetchCameTo::ItIsOwedAgain => {
                        // **P3: the unit returns to *owed* and is re-arranged
                        // at a later walk cycle**, and the slot goes back with
                        // it. Nothing is retried inside its own action.
                        queue.it_failed_and_may_come_back(unit);
                        None
                    }
                    super::fetching::WhatTheFetchCameTo::ItStoppedAndTheCallerSaysWhatTheEntryBecomes => {
                        queue.a_unit_ended();
                        Some(the_state_a_stop_leaves(taken_by))
                    }
                };
                if let Some(state) = state {
                    self.write_the_entry(arrangement, &key, in_hand, state.clone());
                    followed.the_entry_now_stands_at = Some(state);
                }
                // **R4's account half carries the transfer identity**, and it
                // is written whatever the transfer ended as: the account is
                // Smoozy's own history of what it did.
                if let Some(entry) = arrangement
                    .reckoning
                    .iter_mut()
                    .find(|entry| entry.predicted_path == key)
                {
                    entry.account.transfer = Some(transfer);
                }
            }
        }

        keeper.write_it_down(arrangement)?;

        // **§12's act, composed at the done transition and at no other
        // moment.** E1 step 4's *done* is the arrangement deciding for itself
        // that nothing more is coming, by finishing its own bookkeeping.
        if self.the_ending_walk_has_completed && every_unit_is_terminal(arrangement) {
            followed.the_diary_act = Some(self.the_act_at_done(arrangement, in_hand));
        }
        Ok(followed)
    }

    /// **§12's second act, composed once at the done transition.**
    ///
    /// The reading is E1 step 4's own — *ended*, never *complete*, with
    /// arrivals against the whole — and the counts are the reckoning's.
    fn the_act_at_done(
        &self,
        arrangement: &AnArrangement,
        in_hand: &WhatIsInHand<'_>,
    ) -> crate::diary::AnAct {
        let keys: Vec<Vec<u8>> = in_hand
            .narrowed_entries
            .iter()
            .map(|entry| entry.predicted_path.clone())
            .collect();
        let count = |wanted: WhatSmoozyDid| {
            arrangement
                .reckoning
                .iter()
                .filter(|entry| entry.account.state == wanted)
                .count()
        };
        crate::diary::AnAct::AnAutoDownloadArrangementDone {
            reading: crate::ending::the_reading(
                arrangement,
                keys.len(),
                in_hand.the_whole_may_grow,
                &keys,
                in_hand.window.a_connection_to_the_server_stands,
                self.the_ending.is_some(),
                self.the_ending_walk_has_completed,
            ),
            folders: arrangement.the_folders_it_has_fetched_into(),
            left_alone: count(WhatSmoozyDid::LeftAlone),
            cannot_land: count(WhatSmoozyDid::CannotLand),
            skipped_by_narrowing: count(WhatSmoozyDid::SkippedByNarrowing),
            never_coming: count(WhatSmoozyDid::NeverComing),
        }
    }

    /// **One turn of one arrangement.**
    ///
    /// The order is E1's, read from the bottom up: **where the ending is in
    /// hand and the ending walk has completed, the only question left is
    /// whether every unit is terminal** (step 4); where the ending is in hand
    /// and it has not, this turn is the ending walk (step 2); otherwise it is
    /// an ordinary walk turn and the two mechanisms.
    pub async fn one_turn<R, N, K>(
        &mut self,
        reader: &mut R,
        rules: &N,
        keeper: &mut K,
        queue: &mut TheFetchQueue,
        arrangement: &mut AnArrangement,
        in_hand: WhatIsInHand<'_>,
    ) -> Result<WhatTheTurnDid, Refused>
    where
        R: ReadsOneDirectoryOnTheMachine,
        N: TheReceivingSystemsRules + ?Sized,
        K: KeepsTheArrangement + ?Sized,
    {
        // **E1 step 4**, asked first because it is the only state that ends
        // the sequence, and because a done arrangement takes no more readings.
        if self.the_ending_walk_has_completed && every_unit_is_terminal(arrangement) {
            return Ok(WhatTheTurnDid::ItIsDone);
        }

        // **E1 step 2. The window's own third condition still governs it** —
        // W3 puts the ending walk inside the window and closes the window at
        // its completion, so an ending that arrives while the connection is
        // down waits for a connection like everything else, which is E2a's
        // *waiting for a connection to that server*.
        if let Some(ending) = self.the_ending {
            if !self.the_ending_walk_has_completed {
                if !in_hand.window.walks_run() {
                    return Ok(WhatTheTurnDid::TheWindowIsShut);
                }
                let answer = super::walking::walk_the_folder_and_everything_under_it(
                    reader,
                    in_hand.the_job_output_folder,
                )
                .await;
                let entries = match &answer.walk {
                    // **A discarded walk is not the ending walk.** Applying its
                    // verdict would put every frame into *never coming* on a
                    // refused listing, so it is retried at the cadence —
                    // **with the cause carried, as W5 requires.**
                    crate::watch::AWalk::Discarded(why) => {
                        return Ok(WhatTheTurnDid::TheWalkWasDiscarded {
                            why: *why,
                            what_the_machine_said: answer.what_the_machine_said,
                            pause_before_the_next: crate::watch::the_pause_before_the_next_walk(
                                answer.it_took,
                                in_hand.arrangements_walking_this_server,
                            ),
                        })
                    }
                    crate::watch::AWalk::Complete(entries) => entries,
                };
                // **E1 step 2's fallback is *the entries active in the last
                // complete walk*, and it is the active set rather than the
                // walk.** The walk itself is every file in the job's output
                // folder, so handing it in would mark all 720 files of a job
                // that ended badly as files Smoozy cannot vouch for — which
                // is J9b's own objection from the other side, on the one
                // screen where the mark has to mean something.
                let last_active: Vec<Vec<u8>> = self
                    .series
                    .the_active_entries_of_the_last_complete_walk()
                    .iter()
                    .map(|entry| entry.path.clone())
                    .collect();
                let verdict = crate::ending::the_ending_walk(
                    in_hand.narrowed_entries,
                    in_hand.the_entries_the_narrowing_excludes,
                    entries,
                    ending,
                    in_hand.order,
                    &last_active,
                    // **N5a: the placement record is the caller's, out of the
                    // job's own local record** (`Dependency Upload & Remap`
                    // M22f), and the four-value readings with it.
                    &crate::narrowing::ThePlacedCopies {
                        list: in_hand.the_placed_copies,
                        the_job_output_folder: in_hand.the_job_output_folder,
                        the_four_values_now: in_hand.the_four_values_of_the_placed_copies,
                    },
                );
                self.apply_the_ending_walk(rules, keeper, queue, arrangement, &in_hand, &verdict)?;
                self.the_ending_walk_has_completed = true;
                // **Its plain files are kept beside the series, for their
                // sizes** — see [`Self::what_a_walk_read_of`] — **and only
                // plain files**:
                // a symbolic link's listed size is the link's own, and a fetch
                // arranged against it would publish the wrong number of bytes,
                // or none from a link that reaches nothing.
                let plain: std::collections::HashSet<&[u8]> =
                    answer.the_plain_files.iter().map(Vec::as_slice).collect();
                self.the_ending_walks_plain_files = entries
                    .iter()
                    .filter(|entry| !entry.is_a_folder && plain.contains(entry.path.as_slice()))
                    .cloned()
                    .collect();
                return Ok(WhatTheTurnDid::TheEndingWalkRan(Box::new(verdict)));
            }
        }

        // The ordinary turn.
        let turn = self
            .series
            .one_turn(
                reader,
                in_hand.the_job_output_folder,
                in_hand.window,
                in_hand.arrangements_walking_this_server,
            )
            .await;
        match turn {
            WhatTheTurnCameTo::TheWindowIsShutSoNoWalkRan => Ok(WhatTheTurnDid::TheWindowIsShut),
            // **The cause travels with the discard**, which is what gives the
            // arrangement something to say while nothing arrives.
            WhatTheTurnCameTo::ItWasDiscarded {
                why,
                what_the_machine_said,
                pause_before_the_next,
            } => Ok(WhatTheTurnDid::TheWalkWasDiscarded {
                why,
                what_the_machine_said,
                pause_before_the_next,
            }),
            WhatTheTurnCameTo::AReading {
                active,
                it_is_the_baseline,
                pause_before_the_next,
            } => {
                let became_fetchable = self.what_this_walk_vouched_for(
                    rules,
                    keeper,
                    queue,
                    arrangement,
                    &in_hand,
                    &active,
                    it_is_the_baseline,
                )?;
                Ok(WhatTheTurnDid::ItWalked {
                    became_fetchable,
                    pause_before_the_next,
                })
            }
        }
    }

    /// **§4's two mechanisms, applied to one complete walk.**
    ///
    /// **J6: the first complete walk of a connection is the baseline and
    /// judges nothing by itself.** After it, mechanism two fires **once** —
    /// the earliest frame in the attempt's order with activity since the
    /// baseline, everything strictly earlier judged finished — and mechanism
    /// one's live frontier carries the rest of the render.
    ///
    /// **The two are never mixed** (J3's own warning): mechanism two reads the
    /// since-baseline set and fires once; mechanism one reads the current walk
    /// alone and rises with the render.
    #[allow(
        clippy::too_many_arguments,
        reason = "this is the sequencer's own step and every argument is a \
                  different one of the four things E1 puts in front of it — \
                  the adapter, the store, the queue, the record, what is in \
                  hand and what this walk read. Gathering them would make one \
                  shape whose only reader is this function"
    )]
    fn what_this_walk_vouched_for<N, K>(
        &mut self,
        rules: &N,
        keeper: &mut K,
        queue: &mut TheFetchQueue,
        arrangement: &mut AnArrangement,
        in_hand: &WhatIsInHand<'_>,
        active: &[AWalkEntry],
        it_is_the_baseline: bool,
    ) -> Result<Vec<Vec<u8>>, Refused>
    where
        N: TheReceivingSystemsRules + ?Sized,
        K: KeepsTheArrangement + ?Sized,
    {
        // J6: baseline presence is never activity. J6's independent worker
        // report can nevertheless prove a prefix on this very first walk.
        let active_frames = if it_is_the_baseline {
            Vec::new()
        } else {
            the_frames(in_hand.narrowed_entries, active)
        };
        for frame in &active_frames {
            if !self.active_since_the_baseline.contains(frame) {
                self.active_since_the_baseline.push(*frame);
            }
        }

        let mut judged: Vec<i64> = what_the_live_walk_vouches_for(in_hand.order, &active_frames);
        if let (Some(saved), TheOrder::The(order)) = (self.the_reported_saved_frame, in_hand.order) {
            if let Some(at) = order.frames().iter().position(|frame| *frame == saved) {
                for frame in &order.frames()[..=at] {
                    if !judged.contains(frame) {
                        judged.push(*frame);
                    }
                }
            }
        }
        if !self.the_catch_up_has_judged && !self.active_since_the_baseline.is_empty() {
            let caught_up = what_the_catch_up_vouches_for(
                in_hand.order,
                &TheCatchUpsSignal::ActivitySinceTheBaseline(
                    self.active_since_the_baseline.clone(),
                ),
            )
            // **The entries in hand are what an *everything the attempt
            // expected* answer is enumerated against**, and this signal never
            // gives that answer — the enumeration is handed in all the same,
            // so the arm the ending uses and the arm this uses meet the same
            // caller.
            .the_frames(
                in_hand
                    .narrowed_entries
                    .iter()
                    .filter_map(|entry| entry.frame.as_ref()),
            );
            for frame in caught_up {
                if !judged.contains(&frame) {
                    judged.push(frame);
                }
            }
            self.the_catch_up_has_judged = true;
        }

        // **J3: a frame judged finished is fetchable where every one of its
        // expected files is present and none is zero bytes.**
        let mut became_fetchable = Vec::new();
        for frame in judged {
            let files: Vec<AnExpectedFile> = in_hand
                .narrowed_entries
                .iter()
                .filter(|entry| entry.frame == Some(frame))
                .map(|entry| {
                    let seen = self
                        .series
                        .the_baseline()
                        .iter()
                        .find(|seen| seen.path == entry.predicted_path);
                    AnExpectedFile {
                        predicted_path: entry.predicted_path.clone(),
                        present: seen.is_some(),
                        size: seen.map(|seen| seen.size).unwrap_or_default(),
                    }
                })
                .collect();
            if files.is_empty() || !a_judged_finished_frame_is_fetchable(&files) {
                continue;
            }
            for file in files {
                if self.owe_it(rules, queue, arrangement, in_hand, &file.predicted_path)? {
                    became_fetchable.push(file.predicted_path);
                }
            }
        }
        if !became_fetchable.is_empty() {
            keeper.write_it_down(arrangement)?;
        }
        Ok(became_fetchable)
    }

    /// **P4: the name check runs at the moment a file first becomes fetchable
    /// for an arrangement**, and a refused name never becomes a unit.
    ///
    /// **R8 and R9 gate it first**: the fetch decision is the reckoning's, so
    /// a path whose account is already terminal is not owed and the check is
    /// not even asked.
    fn owe_it<N>(
        &mut self,
        rules: &N,
        queue: &mut TheFetchQueue,
        arrangement: &mut AnArrangement,
        in_hand: &WhatIsInHand<'_>,
        predicted_path: &[u8],
    ) -> Result<bool, Refused>
    where
        N: TheReceivingSystemsRules + ?Sized,
    {
        if !arrangement.a_fetch_is_owed_for(predicted_path) {
            return Ok(false);
        }
        // **E1 step 2's dated note of 2026-10-31: a file the pair's other half
        // already fetched into this destination is taken, and nothing is
        // fetched** — one folder, so the same landing path, and a fetch would
        // meet its own pair's file at the exclusive create. The twenty-sixth
        // notes, note 4.
        if in_hand
            .the_pairs_other_half_fetched
            .iter()
            .any(|fetched| fetched == predicted_path)
        {
            self.write_the_entry(arrangement, predicted_path, in_hand, WhatSmoozyDid::Fetched);
            return Ok(false);
        }
        // **One walk's judgement, not several.** The live frontier is
        // recomputed at every walk and rises, so the same frame is judged
        // finished again at the next walk; without this the queue would take
        // one file many times over.
        if self.already_offered.iter().any(|at| at == predicted_path) {
            return Ok(false);
        }
        // **P4 judges the components this document composes and no others.**
        // The landing path is the destination plus the predicted path
        // (P11), and the destination is a folder of the user's own that
        // somebody else already wrote — judging its components would refuse a
        // fetch over a name Smoozy did not choose and cannot change.
        let components: Vec<&[u8]> = predicted_path.split(|byte| *byte == b'/').collect();
        if judge_the_landing_path(rules, &components).is_err() {
            // **P8 and P9: the file is marked *cannot land*, the unit is
            // terminal, it leaves the whole and it is reported separately** —
            // an arrangement never waits on a file the name check has already
            // refused. **Nothing is repaired**: a render output's name is
            // never touched.
            self.write_the_entry(
                arrangement,
                predicted_path,
                in_hand,
                WhatSmoozyDid::CannotLand,
            );
            return Ok(false);
        }
        // **R4's entry, written at *owed* the moment the fetch is arranged.**
        // The reckoning holds one entry per file the arrangement has acted on,
        // and arranging a fetch is an act — **so E1 step 4's *every unit is
        // terminal* has a unit to be false about while the fetch is in
        // flight.** Without it a job whose last frames were fetchable would
        // read *done* the instant the ending walk ran, over transfers that had
        // not started.
        self.write_the_entry(arrangement, predicted_path, in_hand, WhatSmoozyDid::Owed);
        self.already_offered.push(predicted_path.to_vec());
        // **P2: within an arrangement the order is the attempt's order**, so
        // *earliest judged-finished first* is a place in that order and never
        // a frame's number and never the inventory's own order. **J2 settles
        // it in terms** — a list written `50, 10, 20` is that order — and
        // [`crate::fetching::the_fetch_order`] is the same answer read the
        // same way.
        //
        // **A file with no frame keeps the tail**, which is where a video
        // destination's entry and an added-place file belong: they become
        // fetchable at the ending walk and the attempt's order says nothing
        // about them.
        let at = in_hand
            .narrowed_entries
            .iter()
            .find(|entry| entry.predicted_path == predicted_path)
            .and_then(|entry| entry.frame)
            .and_then(|frame| match in_hand.order {
                TheOrder::The(order) => order.position_of(frame),
                TheOrder::ItCouldNotBeRead => None,
            })
            .unwrap_or(usize::MAX);
        queue.owe(AUnitOwed {
            key: predicted_path.to_vec(),
            in_order_at: at,
            arranged_by: crate::classes::WhatArrangedIt::AnAutoDownloadArrangement,
            // **D6: the unit carries the destination its own arrangement was
            // aimed at**, so a condition on another arrangement's folder never
            // holds it. It is the destination in force at this act, which is
            // the same field R1's fifth bullet writes on the entry.
            the_destination_normalised: arrangement.destination.clone(),
            // **P10, P11 and M9: the job's own folder sits between the
            // destination and the file.** The destination is the folder that
            // *contains* the job's folder, and the composition is
            // [`crate::fetching::the_landing_path_inside_the_jobs_folder`]'s —
            // the same one the manual whole-job route comes through, so the
            // two cannot drift.
            the_landing_path: crate::fetching::the_landing_path_inside_the_jobs_folder(
                rules,
                &arrangement.destination,
                TheFirstFramePair::the_job_whose_folder_it_lands_in(
                    in_hand.the_first_frame_pair,
                    &arrangement.job,
                ),
                in_hand.the_jobs_label,
                predicted_path,
            ),
        });
        Ok(true)
    }

    /// **E1 step 2 applied: what the ending walk made fetchable, what became
    /// *never coming*, and what carries the mark.**
    fn apply_the_ending_walk<N, K>(
        &mut self,
        rules: &N,
        keeper: &mut K,
        queue: &mut TheFetchQueue,
        arrangement: &mut AnArrangement,
        in_hand: &WhatIsInHand<'_>,
        verdict: &TheEndingWalksVerdict,
    ) -> Result<(), Refused>
    where
        N: TheReceivingSystemsRules + ?Sized,
        K: KeepsTheArrangement + ?Sized,
    {
        // **What is in hand is the turn's own, and the attempt's order among
        // it.** This step used to compose a stand-in whose order was *it could
        // not be read*, which threw away the one order P2 fetches in at the
        // one moment most of a job's units are owed.
        for path in &verdict.fetchable {
            // **`Job files and outputs` P18a: the range job leaves the check's
            // marked files to the check.**
            let named = in_hand
                .narrowed_entries
                .iter()
                .any(|entry| entry.predicted_path == *path);
            if TheFirstFramePair::the_range_leaves_it_to_the_check(
                in_hand.the_first_frame_pair,
                &arrangement.job,
                named,
                path,
            ) || TheFirstFramePair::the_check_leaves_it_to_the_range(
                in_hand.the_first_frame_pair,
                &arrangement.job,
                named,
                path,
            ) {
                continue;
            }
            self.owe_it(rules, queue, arrangement, in_hand, path)?;
        }
        let ended_short = self
            .the_ending
            .is_some_and(|ending| ending != Ending::Finished);
        let not_found: std::collections::HashSet<&[u8]> =
            verdict.not_found.iter().map(Vec::as_slice).collect();
        for path in &verdict.never_coming {
            // **N7: *never coming* stays in the whole and is the shortfall.**
            // The short count is the one saying the spec promises for what
            // nobody could vouch for.
            //
            // **It is written only over an entry the record does not name, or
            // one standing at *owed*** — the same gate `owe_it` asks on the
            // other side. **R4 makes *fetched* final and R9 says nothing
            // re-opens it**, so writing *never coming* over one would tell the
            // user a file never came home that is sitting in their Downloads
            // folder. The path this closes: attempt 1 brings frames 1 to 100
            // home, the user re-runs with a shorter list, `Job files and
            // outputs` I22 replaces what was left on the machine, and the
            // ending walk then finds every earlier frame gone. The other half
            // is the same file read at zero bytes after a machine reset.
            if !arrangement.a_fetch_is_owed_for(path) {
                continue;
            }
            // **N6's last sentence and N7 as narrowed: once the job has ended
            // short of a clean finish, an entry the ending walk did not find
            // at all is out of the whole**, since the frames a stopped or
            // failed render never reached are not a hole — so it is not
            // entered, and the reckoning's own length is what the walk found.
            // **A zero-byte entry was found, and stays in as the shortfall.**
            // **An entry the record already names at *owed*** was seen by an
            // earlier walk and is gone now; it is written *never coming* as
            // before, since leaving it *owed* would keep the arrangement from
            // ever being done. Added 2026-10-19, out of the live pass on Vast
            // for the thirteenth and fourteenth notes: Transfers read *ended
            // 100 / 450* for a job stopped at 52 frames.
            if ended_short
                && not_found.contains(path.as_slice())
                && arrangement.entry_for(path).is_none()
            {
                continue;
            }
            self.write_the_entry(arrangement, path, in_hand, WhatSmoozyDid::NeverComing);
        }
        self.write_what_the_narrowing_skipped(arrangement, in_hand);
        keeper.write_it_down(arrangement)
    }

    /// **N3, N7 and R4: the narrowing applied to the record.**
    ///
    /// **Every entry the user's choice excludes is written at *skipped by
    /// narrowing***, which is what makes it leave the whole and what lets it
    /// re-open to *owed* when the narrowing is widened. **An entry that was
    /// never written can do neither**, so leaving them unentered would make
    /// the spec's *"changing their mind about the narrowing costs the user
    /// nothing"* unkeepable.
    ///
    /// **It writes only over an entry the record does not name or one standing
    /// at *owed*** — the same gate every other write here asks. A file the
    /// arrangement already brought home stays *fetched*; the promise is that
    /// what has already come home stays.
    fn write_what_the_narrowing_skipped(
        &mut self,
        arrangement: &mut AnArrangement,
        in_hand: &WhatIsInHand<'_>,
    ) {
        for entry in in_hand.the_entries_the_narrowing_excludes {
            if !arrangement.a_fetch_is_owed_for(&entry.predicted_path) {
                continue;
            }
            self.write_the_entry(
                arrangement,
                &entry.predicted_path,
                in_hand,
                WhatSmoozyDid::SkippedByNarrowing,
            );
        }
    }

    /// **R3, R11 and N3: the arrangement switched on again, with the new
    /// narrowing applied to the record in the same act.**
    ///
    /// [`crate::arrangement::AnArrangement::switched_on_again`] resets the
    /// destination and the narrowing and re-opens every *skipped by narrowing*
    /// entry to *owed*, which is R11's *reset to everything*. **On its own
    /// that hands the user back the very files their new, narrower choice
    /// excludes** — they switch auto-download off, switch it on again asking
    /// for PNG alone, and every EXR they had just refused is owed again. So
    /// the new narrowing is run over the record immediately after the reset
    /// and puts them back at *skipped by narrowing*.
    ///
    /// **The excluded entries handed in must be cut with the new narrowing**,
    /// which is the choice the record is being brought into line with.
    pub fn switched_on_again<K: KeepsTheArrangement + ?Sized>(
        &mut self,
        keeper: &mut K,
        arrangement: &mut AnArrangement,
        destination: Vec<u8>,
        narrowing: crate::arrangement::TheNarrowing,
        in_hand: &WhatIsInHand<'_>,
    ) -> Result<Option<crate::arrangement::TheDestinationMoved>, Refused> {
        let telling = arrangement.switched_on_again(destination, narrowing);
        self.write_what_the_narrowing_skipped(arrangement, in_hand);
        // **A switch-on is a fresh window**, so what an earlier one offered
        // says nothing about this one.
        self.already_offered.clear();
        keeper.write_it_down(arrangement)?;
        Ok(telling)
    }

    /// One reckoning entry written, **with the destination in force when its
    /// own act was taken** (R1's fifth bullet).
    ///
    /// **A state change merges into the entry that stands rather than
    /// replacing it.** R4 splits the entry in two on purpose — the account of
    /// Smoozy's own act, and the server-file identity the fetch was arranged
    /// against — and a wholesale replacement threw the second half away, along
    /// with the transfer identity the account itself carries, every time a
    /// state moved. **A fresh entry is written only where the record names
    /// none.**
    fn write_the_entry(
        &mut self,
        arrangement: &mut AnArrangement,
        predicted_path: &[u8],
        in_hand: &WhatIsInHand<'_>,
        state: WhatSmoozyDid,
    ) {
        let frame = in_hand
            .narrowed_entries
            .iter()
            .find(|entry| entry.predicted_path == predicted_path)
            .and_then(|entry| entry.frame);
        match arrangement
            .reckoning
            .iter_mut()
            .find(|already| already.predicted_path == predicted_path)
        {
            Some(already) => {
                already.account.state = state;
                // **The destination in force is the one in force at *this*
                // act** (R1's fifth bullet), so it moves with the state and
                // the rest of the entry stands.
                already.account.destination_in_force = arrangement.destination.clone();
                // **A frame the entries name is filled in where the entry had
                // none**, and an entry that already carries one keeps it: the
                // frame is the inventory's fact about the file, not this act's.
                if already.frame.is_none() {
                    already.frame = frame;
                }
            }
            None => arrangement.reckoning.push(AReckoningEntry::acted_on(
                predicted_path.to_vec(),
                frame,
                state,
                arrangement.destination.clone(),
            )),
        }
    }
}

/// The frames the active walk entries belong to, read off the inventory's own
/// entries and never off a name.
fn the_frames(narrowed_entries: &[ANarrowedEntry], active: &[AWalkEntry]) -> Vec<i64> {
    let mut frames = Vec::new();
    for entry in active {
        let Some(frame) = narrowed_entries
            .iter()
            .find(|named| named.predicted_path == entry.path)
            .and_then(|named| named.frame)
        else {
            continue;
        };
        if !frames.contains(&frame) {
            frames.push(frame);
        }
    }
    frames
}

/// **J5 and J6's first bullet, held where the caller meets it: the last frame
/// of an attempt has no later frame, so its signal is the job's ending.**
///
/// **The ending is `Queue & History`'s and is carried here rather than decided
/// here**, live or handed over once per connection at its C1 catch-up; how the
/// live signal itself reaches a frame — the next frame's placeholder or its
/// first real write — is J4's and changes the latency and nothing else.
pub fn the_endings_signal(ending: Ending, _how: HowTheSignalArrives) -> TheCatchUpsSignal {
    TheCatchUpsSignal::TheJobsEnding(ending)
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use smoozy_server_protocol::subsystem::{StatusCode, SubsystemFailure};
    use smoozy_state::identities::{Identity, IdentityKind};

    use crate::arrangement::TheNarrowing;
    use crate::fetching::TheNameCheckSaid;

    use super::super::probing::TheProbesCadence;
    use super::super::walking::ADirectoryEntry;
    use super::*;

    #[derive(Default)]
    struct AMachine {
        folders: HashMap<Vec<u8>, Result<Vec<ADirectoryEntry>, SubsystemFailure>>,
    }

    impl AMachine {
        fn holding(&mut self, path: &[u8], entries: Vec<ADirectoryEntry>) {
            self.folders.insert(path.to_vec(), Ok(entries));
        }
        fn refusing(&mut self, path: &[u8], failure: SubsystemFailure) {
            self.folders.insert(path.to_vec(), Err(failure));
        }
    }

    impl ReadsOneDirectoryOnTheMachine for AMachine {
        async fn read_one_directory(
            &mut self,
            path: &[u8],
        ) -> Result<Vec<ADirectoryEntry>, SubsystemFailure> {
            self.folders
                .get(path)
                .cloned()
                .unwrap_or(Err(SubsystemFailure::TheServerRefused(
                    StatusCode::NoSuchFile,
                )))
        }
    }

    struct EverythingIsWritable;
    impl TheReceivingSystemsRules for EverythingIsWritable {
        fn check(&self, _component: &[u8]) -> TheNameCheckSaid {
            TheNameCheckSaid::TheNameMayBeWrittenAsGiven
        }
        fn substitute(&self, component: &[u8]) -> Vec<u8> {
            component.to_vec()
        }
        fn length_in_this_systems_own_unit(&self, component: &[u8]) -> usize {
            component.len()
        }
        fn the_longest_component(&self) -> usize {
            255
        }
    }

    #[derive(Default)]
    struct AList {
        writes: usize,
    }
    impl KeepsTheArrangement for AList {
        fn write_it_down(&mut self, _arrangement: &AnArrangement) -> Result<(), Refused> {
            self.writes += 1;
            Ok(())
        }
        fn the_arrangement_for(&self, _job: &Identity) -> Result<Option<AnArrangement>, Refused> {
            Ok(None)
        }
        fn it_ended(
            &mut self,
            _job: &Identity,
            _how: crate::arrangement::WhatEndsIt,
        ) -> Result<(), Refused> {
            Ok(())
        }
    }

    fn run<F: std::future::Future>(f: F) -> F::Output {
        let mut f = Box::pin(f);
        let waker = std::task::Waker::noop();
        let mut context = std::task::Context::from_waker(waker);
        loop {
            if let std::task::Poll::Ready(out) = f.as_mut().poll(&mut context) {
                return out;
            }
        }
    }

    fn an_identity(kind: IdentityKind) -> Identity {
        Identity::issue(kind).expect("the desk has randomness")
    }

    /// **The destination is the folder that *contains* the job's folder** —
    /// ruling 2 of this crate's round 1 — which is what `landing.rs`'s
    /// whole-job test has always used and what H5a's arithmetic needs.
    fn an_arrangement() -> AnArrangement {
        AnArrangement::switched_on(
            an_identity(IdentityKind::Job),
            an_identity(IdentityKind::LocalServer),
            b"/Users/a/Downloads".to_vec(),
            TheNarrowing::Everything,
        )
    }

    fn file(name: &str, size: u64, modified: i64) -> ADirectoryEntry {
        ADirectoryEntry {
            name: name.as_bytes().to_vec(),
            kind: super::super::walking::TheEntrysKind::NotAFolder,
            size,
            modification_time: modified,
        }
    }

    fn entries() -> Vec<ANarrowedEntry> {
        vec![
            ANarrowedEntry {
                predicted_path: b"0001.exr".to_vec(),
                frame: Some(1),
            },
            ANarrowedEntry {
                predicted_path: b"0002.exr".to_vec(),
                frame: Some(2),
            },
            ANarrowedEntry {
                predicted_path: b"0003.exr".to_vec(),
                frame: Some(3),
            },
        ]
    }

    fn an_order() -> TheOrder {
        TheOrder::The(
            crate::vouching::TheAttemptsOrder::as_the_attempt_record_carries_it(vec![1, 2, 3]),
        )
    }

    fn an_open_window() -> TheWindow {
        TheWindow {
            the_desired_state_is_on: true,
            a_connection_to_the_server_stands: true,
            the_attempts_window_is_open: true,
        }
    }

    fn in_hand<'a>(narrowed: &'a [ANarrowedEntry], order: &'a TheOrder) -> WhatIsInHand<'a> {
        WhatIsInHand {
            the_job_output_folder: b"/jobs/abc",
            narrowed_entries: narrowed,
            the_whole_may_grow: false,
            the_entries_the_narrowing_excludes: &[],
            order,
            the_jobs_label: Some("night"),
            the_first_frame_pair: None,
            the_pairs_other_half_fetched: &[],
            the_placed_copies: None,
            the_four_values_of_the_placed_copies: &[],
            window: an_open_window(),
            arrangements_walking_this_server: 1,
        }
    }

    #[test]
    fn seventeenth_a_saved_prefix_resumes_on_the_first_complete_walk() {
        let mut machine = AMachine::default();
        machine.holding(b"/jobs/abc", vec![file("0001.exr", 40, 1), file("0002.exr", 40, 2)]);
        let mut keeper = AList::default();
        let mut queue = TheFetchQueue::at_a_server();
        let mut arrangement = an_arrangement();
        let mut running = TheArrangementsRun::fresh();
        let narrowed = entries();
        let order = an_order();
        running.the_saved_prefix_was_reported(Some(2));
        let turn = run(running.one_turn(
            &mut machine, &EverythingIsWritable, &mut keeper, &mut queue,
            &mut arrangement, in_hand(&narrowed, &order),
        )).unwrap();
        match turn {
            WhatTheTurnDid::ItWalked { became_fetchable, pause_before_the_next } => {
                assert_eq!(became_fetchable, vec![b"0001.exr".to_vec(), b"0002.exr".to_vec()]);
                assert_eq!(pause_before_the_next, crate::watch::THE_PAUSE);
            }
            other => panic!("no complete first walk: {other:?}"),
        }
    }

    #[test]
    fn seventeenth_saved_prefix_keeps_the_attempts_order_and_requires_every_file() {
        for (saved, readable, second_size, expected) in [
            (Some(10), true, Some(40), vec![b"50.exr".to_vec(), b"10-a.exr".to_vec(), b"10-b.exr".to_vec()]),
            (Some(10), true, Some(0), vec![b"50.exr".to_vec()]),
            (Some(10), true, None, vec![b"50.exr".to_vec()]),
            (Some(99), true, Some(40), vec![]),
            (Some(10), false, Some(40), vec![]),
            (None, true, Some(40), vec![]),
        ] {
            let mut machine = AMachine::default();
            let mut files = vec![file("50.exr", 40, 1), file("10-a.exr", 40, 1), file("20.exr", 40, 1)];
            if let Some(size) = second_size { files.push(file("10-b.exr", size, 1)); }
            machine.holding(b"/jobs/abc", files);
            let narrowed: Vec<_> = [("50.exr", 50), ("10-a.exr", 10), ("10-b.exr", 10), ("20.exr", 20)].into_iter()
                .map(|(path, frame)| ANarrowedEntry { predicted_path: path.as_bytes().to_vec(), frame: Some(frame) }).collect();
            let order = if readable {
                TheOrder::The(crate::vouching::TheAttemptsOrder::as_the_attempt_record_carries_it(vec![50, 10, 20]))
            } else { TheOrder::ItCouldNotBeRead };
            let mut running = TheArrangementsRun::fresh();
            running.the_saved_prefix_was_reported(saved);
            let mut keeper = AList::default();
            let mut queue = TheFetchQueue::at_a_server();
            let mut arrangement = an_arrangement();
            let turn = run(running.one_turn(&mut machine, &EverythingIsWritable, &mut keeper, &mut queue,
                &mut arrangement, in_hand(&narrowed, &order))).unwrap();
            match turn {
                WhatTheTurnDid::ItWalked { became_fetchable, .. } => assert_eq!(became_fetchable, expected),
                other => panic!("{other:?}"),
            }
            assert!(running.active_since_the_baseline.is_empty());
        }
    }

    /// **J6: the first complete walk is the baseline and judges nothing.**

    /// **J6: the first complete walk is the baseline and judges nothing.**
    #[test]
    fn the_live_frontier_vouches_for_the_earlier_frame_and_never_for_the_one_in_flight() {
        let mut machine = AMachine::default();
        let mut keeper = AList::default();
        let mut queue = TheFetchQueue::at_a_server();
        let mut arrangement = an_arrangement();
        let mut running = TheArrangementsRun::fresh();
        let narrowed = entries();
        let order = an_order();

        machine.holding(b"/jobs/abc", vec![file("0001.exr", 40, 1)]);
        let first = run(running.one_turn(
            &mut machine,
            &EverythingIsWritable,
            &mut keeper,
            &mut queue,
            &mut arrangement,
            in_hand(&narrowed, &order),
        ))
        .expect("the store took it");
        assert_eq!(
            first,
            WhatTheTurnDid::ItWalked {
                became_fetchable: Vec::new(),
                pause_before_the_next: crate::watch::THE_PAUSE,
            }
        );

        machine.holding(
            b"/jobs/abc",
            vec![file("0001.exr", 40, 1), file("0002.exr", 12, 2)],
        );
        let second = run(running.one_turn(
            &mut machine,
            &EverythingIsWritable,
            &mut keeper,
            &mut queue,
            &mut arrangement,
            in_hand(&narrowed, &order),
        ))
        .expect("the store took it");
        match second {
            WhatTheTurnDid::ItWalked {
                became_fetchable, ..
            } => assert_eq!(became_fetchable, vec![b"0001.exr".to_vec()]),
            other => panic!("expected a walk, got {other:?}"),
        }
    }

    /// **W5: a discarded walk changes nothing and the next one runs at the
    /// cadence. Nothing gives up.**
    #[test]
    fn a_discarded_walk_owes_nothing_and_writes_nothing() {
        let mut machine = AMachine::default();
        machine.refusing(b"/jobs/abc", SubsystemFailure::TheChannelIsGone);
        let mut keeper = AList::default();
        let mut queue = TheFetchQueue::at_a_server();
        let mut arrangement = an_arrangement();
        let mut running = TheArrangementsRun::fresh();
        let narrowed = entries();
        let order = an_order();

        let turn = run(running.one_turn(
            &mut machine,
            &EverythingIsWritable,
            &mut keeper,
            &mut queue,
            &mut arrangement,
            in_hand(&narrowed, &order),
        ))
        .expect("the store took it");

        assert!(matches!(turn, WhatTheTurnDid::TheWalkWasDiscarded { .. }));
        assert_eq!(keeper.writes, 0);
        assert_eq!(queue.owed(), 0);
        assert!(arrangement.reckoning.is_empty());
    }

    /// **E1 step 2 is not applied on a discarded walk**, which is the one
    /// place a refused listing would put every frame of the job into *never
    /// coming*.
    #[test]
    fn a_discarded_ending_walk_never_marks_a_frame_never_coming() {
        let mut machine = AMachine::default();
        machine.refusing(
            b"/jobs/abc",
            SubsystemFailure::TheServerRefused(StatusCode::PermissionDenied),
        );
        let mut keeper = AList::default();
        let mut queue = TheFetchQueue::at_a_server();
        let mut arrangement = an_arrangement();
        let mut running = TheArrangementsRun::fresh();
        running.the_ending_arrived(Ending::Finished);
        let narrowed = entries();
        let order = an_order();

        let turn = run(running.one_turn(
            &mut machine,
            &EverythingIsWritable,
            &mut keeper,
            &mut queue,
            &mut arrangement,
            in_hand(&narrowed, &order),
        ))
        .expect("the store took it");

        assert!(matches!(turn, WhatTheTurnDid::TheWalkWasDiscarded { .. }));
        assert!(arrangement.reckoning.is_empty());
        assert_eq!(keeper.writes, 0);
    }

    /// **E1 step 2, on a complete ending walk: every narrowed entry present
    /// and not zero bytes is fetchable, and every one absent or zero-byte
    /// becomes *never coming*** — which stays in the whole and is the
    /// shortfall.
    #[test]
    fn the_ending_walk_makes_what_is_there_fetchable_and_the_rest_never_coming() {
        let mut machine = AMachine::default();
        machine.holding(
            b"/jobs/abc",
            vec![file("0001.exr", 40, 1), file("0002.exr", 0, 2)],
        );
        let mut keeper = AList::default();
        let mut queue = TheFetchQueue::at_a_server();
        let mut arrangement = an_arrangement();
        let mut running = TheArrangementsRun::fresh();
        running.the_ending_arrived(Ending::Finished);
        let narrowed = entries();
        let order = an_order();

        let turn = run(running.one_turn(
            &mut machine,
            &EverythingIsWritable,
            &mut keeper,
            &mut queue,
            &mut arrangement,
            in_hand(&narrowed, &order),
        ))
        .expect("the store took it");

        match turn {
            WhatTheTurnDid::TheEndingWalkRan(verdict) => {
                assert!(verdict.fetchable.contains(&b"0001.exr".to_vec()));
                assert!(verdict.never_coming.contains(&b"0002.exr".to_vec()));
                assert!(verdict.never_coming.contains(&b"0003.exr".to_vec()));
            }
            other => panic!("expected the ending walk, got {other:?}"),
        }
        assert_eq!(queue.owed(), 1);
        assert!(keeper.writes >= 1);
        // **A finished job's absent entry stays in the whole**, as N7's
        // shortfall: all three are in the reckoning.
        assert_eq!(arrangement.reckoning.len(), 3);
    }

    /// **`Download` P10's dated note and `Job files and outputs` P18a: a
    /// first-frame check lands under its range job's name, and the range
    /// job's ending walk leaves the check's marked files to the check.** The
    /// two share one folder on the machine, so the range's walk finds
    /// `check_0001.exr` matching no entry of its own; the check's arrangement
    /// brought it home already. The twenty-first notes, note 6.
    #[test]
    fn twenty_first_6_the_pair_lands_in_one_folder_and_the_range_leaves_the_checks_files() {
        let the_check = an_identity(IdentityKind::Job);
        let the_range = an_identity(IdentityKind::Job);
        let pair = TheFirstFramePair { the_check: &the_check, the_range: &the_range };

        // The check: its one marked frame lands under the range job's name.
        let mut machine = AMachine::default();
        machine.holding(b"/jobs/abc", vec![file("check_0001.exr", 40, 1)]);
        let mut keeper = AList::default();
        let mut queue = TheFetchQueue::at_a_server();
        let mut arrangement = an_arrangement();
        arrangement.job = the_check.clone();
        let mut running = TheArrangementsRun::fresh();
        running.the_ending_arrived(Ending::Finished);
        let narrowed = vec![ANarrowedEntry { predicted_path: b"check_0001.exr".to_vec(), frame: Some(1) }];
        let order = TheOrder::The(crate::vouching::TheAttemptsOrder::as_the_attempt_record_carries_it(vec![1]));
        let mut given = in_hand(&narrowed, &order);
        given.the_first_frame_pair = Some(pair);
        run(running.one_turn(&mut machine, &EverythingIsWritable, &mut keeper, &mut queue, &mut arrangement, given))
            .expect("the store took it");
        let unit = queue.offer(&[]).expect("a unit is granted");
        let mut expected = b"/Users/a/Downloads/".to_vec();
        expected.extend_from_slice(&crate::fetching::the_job_folders_landing_name(
            &EverythingIsWritable,
            &the_range,
            Some("night"),
        ));
        expected.extend_from_slice(b"/check_0001.exr");
        assert_eq!(unit.the_landing_path, expected);

        // The range: its three frames are owed, the check's file is not.
        let mut machine = AMachine::default();
        machine.holding(
            b"/jobs/abc",
            vec![
                file("check_0001.exr", 40, 1),
                file("0001.exr", 40, 2),
                file("0002.exr", 40, 3),
                file("0003.exr", 40, 4),
            ],
        );
        let mut keeper = AList::default();
        let mut queue = TheFetchQueue::at_a_server();
        let mut arrangement = an_arrangement();
        arrangement.job = the_range.clone();
        let mut running = TheArrangementsRun::fresh();
        running.the_ending_arrived(Ending::Finished);
        let narrowed = entries();
        let order = an_order();
        let mut given = in_hand(&narrowed, &order);
        given.the_first_frame_pair = Some(pair);
        run(running.one_turn(&mut machine, &EverythingIsWritable, &mut keeper, &mut queue, &mut arrangement, given))
            .expect("the store took it");
        assert_eq!(queue.owed(), 3, "the range's three frames, and not the check's");

        // With no pair, the same walk brings the unmatched file home as before.
        let mut keeper = AList::default();
        let mut queue = TheFetchQueue::at_a_server();
        let mut arrangement = an_arrangement();
        let mut running = TheArrangementsRun::fresh();
        running.the_ending_arrived(Ending::Finished);
        run(running.one_turn(&mut machine, &EverythingIsWritable, &mut keeper, &mut queue, &mut arrangement, in_hand(&narrowed, &order)))
            .expect("the store took it");
        assert_eq!(queue.owed(), 4);
    }

    /// **`Download` E1 step 2's dated note of 2026-10-31: the check's ending
    /// walk leaves the range job's files to the range job.** The pair shares
    /// one folder, so a check whose ending walk runs once the range has
    /// written finds the range's frames there, matching no entry of its own.
    /// **Every file the check writes carries the mark** (`Job files and
    /// outputs` P18a), so a file without it is the range's. Found live, the
    /// twenty-sixth notes, note 4: after a restart the check took all six of
    /// the range's frames.
    #[test]
    fn twenty_sixth_4_the_checks_ending_walk_leaves_the_ranges_frames_to_the_range() {
        let the_check = an_identity(IdentityKind::Job);
        let the_range = an_identity(IdentityKind::Job);
        let pair = TheFirstFramePair { the_check: &the_check, the_range: &the_range };
        let mut machine = AMachine::default();
        machine.holding(
            b"/jobs/abc",
            vec![file("check_x_0001.png", 40, 1), file("x_0001.png", 40, 2), file("x_0002.png", 40, 3)],
        );
        let mut keeper = AList::default();
        let mut queue = TheFetchQueue::at_a_server();
        let mut arrangement = an_arrangement();
        arrangement.job = the_check.clone();
        let mut running = TheArrangementsRun::fresh();
        running.the_ending_arrived(Ending::Finished);
        let narrowed = vec![ANarrowedEntry { predicted_path: b"check_x_0001.png".to_vec(), frame: Some(1) }];
        let order = TheOrder::The(crate::vouching::TheAttemptsOrder::as_the_attempt_record_carries_it(vec![1]));
        let mut given = in_hand(&narrowed, &order);
        given.the_first_frame_pair = Some(pair);
        run(running.one_turn(&mut machine, &EverythingIsWritable, &mut keeper, &mut queue, &mut arrangement, given))
            .expect("the store took it");
        assert_eq!(queue.owed(), 1, "the check owes its own file and none of the range's");
        let owed: Vec<&[u8]> = arrangement.reckoning.iter().map(|entry| entry.predicted_path.as_slice()).collect();
        assert_eq!(owed, vec![b"check_x_0001.png".as_slice()]);
    }

    /// **`Download` E1 step 2's dated note of 2026-10-31: a file the pair's
    /// other half already fetched is taken, and nothing is fetched.** The
    /// records a restart left before this fix hold the range's frames
    /// *fetched* under the check and *owed* under the range; the range's next
    /// ending walk writes them *fetched* rather than fetching them over a name
    /// that is already taken.
    #[test]
    fn twenty_sixth_4_a_file_the_pairs_other_half_fetched_is_taken_and_not_fetched_again() {
        let the_check = an_identity(IdentityKind::Job);
        let the_range = an_identity(IdentityKind::Job);
        let pair = TheFirstFramePair { the_check: &the_check, the_range: &the_range };
        let mut machine = AMachine::default();
        machine.holding(
            b"/jobs/abc",
            vec![file("0001.exr", 40, 1), file("0002.exr", 40, 2), file("0003.exr", 40, 3)],
        );
        let mut keeper = AList::default();
        let mut queue = TheFetchQueue::at_a_server();
        let mut arrangement = an_arrangement();
        arrangement.job = the_range.clone();
        let mut running = TheArrangementsRun::fresh();
        running.the_ending_arrived(Ending::Finished);
        let narrowed = entries();
        let order = an_order();
        let fetched_by_the_check = vec![b"0001.exr".to_vec(), b"0002.exr".to_vec()];
        let mut given = in_hand(&narrowed, &order);
        given.the_first_frame_pair = Some(pair);
        given.the_pairs_other_half_fetched = &fetched_by_the_check;
        run(running.one_turn(&mut machine, &EverythingIsWritable, &mut keeper, &mut queue, &mut arrangement, given))
            .expect("the store took it");
        assert_eq!(queue.owed(), 1, "only the frame nobody has fetched is owed");
        let stands = |path: &[u8]| arrangement.entry_for(path).map(|entry| entry.account.state.clone());
        assert_eq!(stands(b"0001.exr"), Some(WhatSmoozyDid::Fetched));
        assert_eq!(stands(b"0002.exr"), Some(WhatSmoozyDid::Fetched));
        assert_eq!(stands(b"0003.exr"), Some(WhatSmoozyDid::Owed));
    }

    /// **R5's dated note of 2026-10-31: a fresh run told that the ending walk
    /// is written down reads done and walks nothing — where every unit is
    /// terminal.** Where one is still owed, the run walks once more, since only
    /// a walk re-offers it. The twenty-sixth notes, note 4.
    #[test]
    fn twenty_sixth_4_a_written_down_ending_walk_is_not_walked_again_unless_a_unit_is_owed() {
        let mut machine = AMachine::default();
        machine.holding(b"/jobs/abc", vec![file("0001.exr", 40, 1), file("0002.exr", 40, 2), file("0003.exr", 40, 3)]);
        let narrowed = entries();
        let order = an_order();
        let mut arrangement = an_arrangement();
        for path in [b"0001.exr", b"0002.exr", b"0003.exr"] {
            arrangement.reckoning.push(AReckoningEntry::acted_on(path.to_vec(), None, WhatSmoozyDid::Fetched, arrangement.destination.clone()));
        }
        let mut running = TheArrangementsRun::fresh();
        running.the_ending_arrived(Ending::Finished);
        running.the_ending_walk_was_written_down(&arrangement);
        let came = run(running.one_turn(&mut machine, &EverythingIsWritable, &mut AList::default(), &mut TheFetchQueue::at_a_server(), &mut arrangement, in_hand(&narrowed, &order)))
            .expect("the store took it");
        assert_eq!(came, WhatTheTurnDid::ItIsDone);

        arrangement.reckoning[2].account.state = WhatSmoozyDid::Owed;
        let mut running = TheArrangementsRun::fresh();
        running.the_ending_arrived(Ending::Finished);
        running.the_ending_walk_was_written_down(&arrangement);
        let mut queue = TheFetchQueue::at_a_server();
        let came = run(running.one_turn(&mut machine, &EverythingIsWritable, &mut AList::default(), &mut queue, &mut arrangement, in_hand(&narrowed, &order)))
            .expect("the store took it");
        assert!(matches!(came, WhatTheTurnDid::TheEndingWalkRan(_)), "{came:?}");
        assert_eq!(queue.owed(), 1);
    }

    /// **`Download` P6's built note of 2026-10-31: a publication refused for a
    /// taken name ends by P6 whatever the transfer record's ending.** `Server
    /// protocol and transfers` §7 ends such a download *failed*, and *failed*
    /// alone is owed again (P3) — so before this the same file was fetched
    /// over a taken name at every pass, the whole file crossing each time.
    #[test]
    fn twenty_sixth_4_a_failed_fetch_over_a_taken_name_is_left_alone_and_not_owed_again() {
        let mut machine = AMachine::default();
        let mut keeper = AList::default();
        let mut queue = TheFetchQueue::at_a_server();
        let mut arrangement = an_arrangement();
        let mut running = TheArrangementsRun::fresh();
        let narrowed = entries();
        let order = an_order();
        let unit = a_fetch_in_flight(&mut machine, &mut keeper, &mut queue, &mut arrangement, &mut running, &narrowed, &order);
        let key = unit.key.clone();
        let followed = running
            .a_fetch_ended(
                &mut keeper,
                &mut queue,
                &mut TheProbesCadence::fresh(),
                &mut arrangement,
                &in_hand(&narrowed, &order),
                unit,
                WhatTheUnitsFetchCameTo::ItEnded {
                    transfer: an_identity(IdentityKind::Transfer),
                    ending: smoozy_server_protocol::transfers::Ending::Failed,
                    taken_by: Some(crate::fetching::WhatTheTakenNameIs::NothingTheRecordNames),
                    the_write_failed: None,
                },
            )
            .expect("the store took it");
        assert_eq!(followed.the_entry_now_stands_at, Some(WhatSmoozyDid::LeftAlone));
        assert_eq!(queue.owed(), 0, "the taken name was owed again");
        assert_eq!(arrangement.entry_for(&key).map(|entry| entry.account.state.clone()), Some(WhatSmoozyDid::LeftAlone));
    }

    /// **N6's last sentence and N7 as narrowed, 2026-10-19: once the job has
    /// ended short of a clean finish, the whole is what the ending's walk
    /// found.** A zero-byte entry was found and stays in as the shortfall; an
    /// entry the walk did not find at all is not entered, so the reckoning's
    /// own length is the whole. **An entry an earlier walk owed and the ending
    /// walk no longer finds** is written *never coming* as before, so the
    /// arrangement can still be done. Out of the live pass on Vast for the
    /// thirteenth and fourteenth notes: *ended 100 / 450* for a job stopped
    /// at 52 frames.
    #[test]
    fn a_short_ending_leaves_an_entry_the_walk_did_not_find_out_of_the_whole() {
        for ending in [Ending::Stopped, Ending::Failed, Ending::GivenUp] {
            let mut machine = AMachine::default();
            machine.holding(
                b"/jobs/abc",
                vec![file("0001.exr", 40, 1), file("0002.exr", 0, 2)],
            );
            let mut keeper = AList::default();
            let mut queue = TheFetchQueue::at_a_server();
            let mut arrangement = an_arrangement();
            let mut running = TheArrangementsRun::fresh();
            running.the_ending_arrived(ending);
            let narrowed = entries();
            let order = an_order();

            run(running.one_turn(
                &mut machine,
                &EverythingIsWritable,
                &mut keeper,
                &mut queue,
                &mut arrangement,
                in_hand(&narrowed, &order),
            ))
            .expect("the store took it");

            let state = |path: &[u8]| arrangement.entry_for(path).map(|entry| entry.account.state.clone());
            assert_eq!(state(b"0001.exr"), Some(WhatSmoozyDid::Owed), "{ending:?}");
            assert_eq!(
                state(b"0002.exr"),
                Some(WhatSmoozyDid::NeverComing),
                "{ending:?}: a zero-byte entry was found, and stays in the whole"
            );
            assert_eq!(state(b"0003.exr"), None, "{ending:?}: an entry not found is out of the whole");
            assert_eq!(arrangement.reckoning.len(), 2, "{ending:?}");
        }

        // **Owed by an earlier walk, and gone at the ending walk.**
        let mut machine = AMachine::default();
        machine.holding(b"/jobs/abc", vec![file("0001.exr", 40, 1)]);
        let mut keeper = AList::default();
        let mut queue = TheFetchQueue::at_a_server();
        let mut arrangement = an_arrangement();
        arrangement.reckoning.push(AReckoningEntry::acted_on(
            b"0003.exr".to_vec(),
            Some(3),
            WhatSmoozyDid::Owed,
            arrangement.destination.clone(),
        ));
        let mut running = TheArrangementsRun::fresh();
        running.the_ending_arrived(Ending::Stopped);
        let narrowed = entries();
        let order = an_order();
        run(running.one_turn(
            &mut machine,
            &EverythingIsWritable,
            &mut keeper,
            &mut queue,
            &mut arrangement,
            in_hand(&narrowed, &order),
        ))
        .expect("the store took it");
        assert_eq!(
            arrangement.entry_for(b"0003.exr").map(|entry| entry.account.state.clone()),
            Some(WhatSmoozyDid::NeverComing),
            "an entry an earlier walk owed is still settled"
        );
        assert_eq!(arrangement.entry_for(b"0002.exr"), None);
    }

    /// **E1 step 4: the arrangement is done when every unit is terminal**, and
    /// not before.
    #[test]
    fn it_is_done_only_when_every_unit_is_terminal() {
        let mut machine = AMachine::default();
        machine.holding(b"/jobs/abc", vec![file("0001.exr", 40, 1)]);
        let mut keeper = AList::default();
        let mut queue = TheFetchQueue::at_a_server();
        let mut arrangement = an_arrangement();
        let mut running = TheArrangementsRun::fresh();
        running.the_ending_arrived(Ending::Finished);
        let narrowed = entries();
        let order = an_order();

        run(running.one_turn(
            &mut machine,
            &EverythingIsWritable,
            &mut keeper,
            &mut queue,
            &mut arrangement,
            in_hand(&narrowed, &order),
        ))
        .expect("the store took it");

        // 0001 is fetchable and owed, so a unit is not terminal yet.
        let turn = run(running.one_turn(
            &mut machine,
            &EverythingIsWritable,
            &mut keeper,
            &mut queue,
            &mut arrangement,
            in_hand(&narrowed, &order),
        ))
        .expect("the store took it");
        assert_ne!(turn, WhatTheTurnDid::ItIsDone);

        // The fetch ends, and every unit is terminal.
        for entry in &mut arrangement.reckoning {
            if matches!(entry.account.state, WhatSmoozyDid::Owed) {
                entry.account.state = WhatSmoozyDid::Fetched;
            }
        }
        arrangement.reckoning.push(AReckoningEntry::acted_on(
            b"0001.exr".to_vec(),
            Some(1),
            WhatSmoozyDid::Fetched,
            arrangement.destination.clone(),
        ));
        let turn = run(running.one_turn(
            &mut machine,
            &EverythingIsWritable,
            &mut keeper,
            &mut queue,
            &mut arrangement,
            in_hand(&narrowed, &order),
        ))
        .expect("the store took it");
        assert_eq!(turn, WhatTheTurnDid::ItIsDone);
    }

    /// **W3: done is not for ever** — a new render-start re-arms the
    /// arrangement, its *never coming* entries re-open for the frames the new
    /// attempt names, **and nothing else re-opens.**
    #[test]
    fn a_render_start_re_arms_the_arrangement_and_re_opens_only_the_named_frames() {
        let mut keeper = AList::default();
        let mut arrangement = an_arrangement();
        let mut running = TheArrangementsRun::fresh();
        running.the_ending_arrived(Ending::Finished);

        arrangement.reckoning.push(AReckoningEntry::acted_on(
            b"0002.exr".to_vec(),
            Some(2),
            WhatSmoozyDid::NeverComing,
            arrangement.destination.clone(),
        ));
        arrangement.reckoning.push(AReckoningEntry::acted_on(
            b"0003.exr".to_vec(),
            Some(3),
            WhatSmoozyDid::NeverComing,
            arrangement.destination.clone(),
        ));
        arrangement.reckoning.push(AReckoningEntry::acted_on(
            b"0001.exr".to_vec(),
            Some(1),
            WhatSmoozyDid::Fetched,
            arrangement.destination.clone(),
        ));

        running
            .a_render_start_re_armed_it(&mut keeper, &mut arrangement, &[2])
            .expect("the store took it");

        let state = |path: &[u8]| {
            arrangement
                .entry_for(path)
                .expect("the entry stands")
                .account
                .state
                .clone()
        };
        assert_eq!(state(b"0002.exr"), WhatSmoozyDid::Owed);
        assert_eq!(state(b"0003.exr"), WhatSmoozyDid::NeverComing);
        assert_eq!(state(b"0001.exr"), WhatSmoozyDid::Fetched);
        assert_eq!(keeper.writes, 1);
    }

    /// **R9: a re-run never arrives over a file the user already has.** A
    /// walk that shows a *fetched* path active owes nothing for it.
    #[test]
    fn a_fetched_path_is_never_owed_again_however_the_walk_reads() {
        let mut machine = AMachine::default();
        let mut keeper = AList::default();
        let mut queue = TheFetchQueue::at_a_server();
        let mut arrangement = an_arrangement();
        arrangement.reckoning.push(AReckoningEntry::acted_on(
            b"0001.exr".to_vec(),
            Some(1),
            WhatSmoozyDid::Fetched,
            arrangement.destination.clone(),
        ));
        let mut running = TheArrangementsRun::fresh();
        let narrowed = entries();
        let order = an_order();

        machine.holding(b"/jobs/abc", vec![file("0001.exr", 40, 1)]);
        run(running.one_turn(
            &mut machine,
            &EverythingIsWritable,
            &mut keeper,
            &mut queue,
            &mut arrangement,
            in_hand(&narrowed, &order),
        ))
        .expect("the store took it");
        machine.holding(
            b"/jobs/abc",
            vec![file("0001.exr", 99, 9), file("0002.exr", 5, 9)],
        );
        run(running.one_turn(
            &mut machine,
            &EverythingIsWritable,
            &mut keeper,
            &mut queue,
            &mut arrangement,
            in_hand(&narrowed, &order),
        ))
        .expect("the store took it");

        assert_eq!(queue.owed(), 0);
    }

    /// **R4: arranging a fetch is an act, so the reckoning gets its entry at
    /// *owed* the moment the unit joins the queue** — which is what gives E1
    /// step 4 a unit to be false about while the fetch is in flight.
    ///
    /// **Without it a job whose last frames were fetchable reads *done* the
    /// instant the ending walk ran**, over transfers that had not started, and
    /// the user is told the outputs are home before a byte has moved.
    #[test]
    fn a_unit_that_became_fetchable_is_owed_in_the_reckoning_and_the_arrangement_is_not_done() {
        let mut machine = AMachine::default();
        machine.holding(b"/jobs/abc", vec![file("0001.exr", 40, 1)]);
        let mut keeper = AList::default();
        let mut queue = TheFetchQueue::at_a_server();
        let mut arrangement = an_arrangement();
        let mut running = TheArrangementsRun::fresh();
        running.the_ending_arrived(Ending::Finished);
        let narrowed = entries();
        let order = an_order();

        run(running.one_turn(
            &mut machine,
            &EverythingIsWritable,
            &mut keeper,
            &mut queue,
            &mut arrangement,
            in_hand(&narrowed, &order),
        ))
        .expect("the store took it");

        assert_eq!(
            arrangement
                .entry_for(b"0001.exr")
                .expect("the entry stands")
                .account
                .state,
            WhatSmoozyDid::Owed
        );
        assert!(!every_unit_is_terminal(&arrangement));
    }

    /// **The live frontier is recomputed at every walk and rises**, so the
    /// same frame is judged finished again at the next walk — and the queue
    /// must not take the same file twice.
    ///
    /// **The guard is run-held rather than durable**, because an entry
    /// standing at *owed* is not *taken* (R5): a fresh run has to re-arrange
    /// it, and a durable *already queued* field would lose the fetch for good
    /// on a crash.
    #[test]
    fn a_file_already_offered_is_not_offered_again_by_a_later_walk() {
        let mut machine = AMachine::default();
        let mut keeper = AList::default();
        let mut queue = TheFetchQueue::at_a_server();
        let mut arrangement = an_arrangement();
        let mut running = TheArrangementsRun::fresh();
        let narrowed = entries();
        let order = an_order();

        machine.holding(b"/jobs/abc", vec![file("0001.exr", 40, 1)]);
        run(running.one_turn(
            &mut machine,
            &EverythingIsWritable,
            &mut keeper,
            &mut queue,
            &mut arrangement,
            in_hand(&narrowed, &order),
        ))
        .expect("the store took it");

        // Two more walks, each showing a later frame active, so the live
        // frontier judges frame 1 finished at both of them.
        for at in 2..4u64 {
            machine.holding(
                b"/jobs/abc",
                vec![
                    file("0001.exr", 40, 1),
                    file("0002.exr", 10 + at, at as i64),
                ],
            );
            run(running.one_turn(
                &mut machine,
                &EverythingIsWritable,
                &mut keeper,
                &mut queue,
                &mut arrangement,
                in_hand(&narrowed, &order),
            ))
            .expect("the store took it");
        }

        assert_eq!(queue.owed(), 1);
    }

    /// **J2: where the order could not be read, mechanism one does not run**,
    /// and nothing is vouched for on a guessed order.
    #[test]
    fn no_frame_is_vouched_for_where_the_order_could_not_be_read() {
        let mut machine = AMachine::default();
        let mut keeper = AList::default();
        let mut queue = TheFetchQueue::at_a_server();
        let mut arrangement = an_arrangement();
        let mut running = TheArrangementsRun::fresh();
        let narrowed = entries();
        let order = TheOrder::ItCouldNotBeRead;

        machine.holding(b"/jobs/abc", vec![file("0001.exr", 40, 1)]);
        run(running.one_turn(
            &mut machine,
            &EverythingIsWritable,
            &mut keeper,
            &mut queue,
            &mut arrangement,
            in_hand(&narrowed, &order),
        ))
        .expect("the store took it");
        machine.holding(
            b"/jobs/abc",
            vec![file("0001.exr", 40, 1), file("0002.exr", 12, 2)],
        );
        run(running.one_turn(
            &mut machine,
            &EverythingIsWritable,
            &mut keeper,
            &mut queue,
            &mut arrangement,
            in_hand(&narrowed, &order),
        ))
        .expect("the store took it");

        assert_eq!(queue.owed(), 0);
    }

    /// One unit taken out of the queue, so a fetch has something to end.
    fn a_fetch_in_flight(
        machine: &mut AMachine,
        keeper: &mut AList,
        queue: &mut TheFetchQueue,
        arrangement: &mut AnArrangement,
        running: &mut TheArrangementsRun,
        narrowed: &[ANarrowedEntry],
        order: &TheOrder,
    ) -> AUnitOwed {
        machine.holding(b"/jobs/abc", vec![file("0001.exr", 40, 1)]);
        running.the_ending_arrived(Ending::Finished);
        run(running.one_turn(
            machine,
            &EverythingIsWritable,
            keeper,
            queue,
            arrangement,
            in_hand(narrowed, order),
        ))
        .expect("the store took it");
        queue.offer(&[]).expect("a unit is granted")
    }

    /// **E1 step 3's first half: the granted unit is composed as one transfer
    /// under §7, staging inside the user's own destination folder.**
    ///
    /// **The landing path is the one the unit has carried since it was owed**,
    /// so the path the name check judged is the path the fetch lands at, and
    /// the job's own folder is in it.
    #[test]
    fn a_granted_unit_composes_the_transfer_record_the_contract_drives() {
        let mut machine = AMachine::default();
        let mut keeper = AList::default();
        let mut queue = TheFetchQueue::at_a_server();
        let mut arrangement = an_arrangement();
        let mut running = TheArrangementsRun::fresh();
        let narrowed = entries();
        let order = an_order();
        let unit = a_fetch_in_flight(
            &mut machine,
            &mut keeper,
            &mut queue,
            &mut arrangement,
            &mut running,
            &narrowed,
            &order,
        );

        let record = running
            .arrange_the_fetch_for(
                &arrangement,
                &unit,
                b"/jobs/abc",
                an_identity(IdentityKind::Transfer),
                None,
                4096,
                an_identity(IdentityKind::Run),
            )
            .expect("both paths are text");

        assert_eq!(record.source_path, "/jobs/abc/0001.exr");
        assert_eq!(
            record.final_path.as_bytes(),
            unit.the_landing_path.as_slice()
        );
        assert_eq!(
            record.staging_path.directory(),
            "/Users/a/Downloads/.smoozy-staging"
        );
        assert!(!record.staging_path.as_text().contains("0001.exr"));
        assert_eq!(
            record.direction,
            smoozy_server_protocol::transfers::Direction::Down
        );
    }

    /// **A path that is not valid text refuses the arrangement rather than
    /// being mangled**, and the refusal names which of the record's two paths
    /// it was.
    #[test]
    fn a_unit_whose_machine_path_is_not_text_refuses_the_arrangement() {
        let arrangement = an_arrangement();
        let running = TheArrangementsRun::fresh();
        let unit = AUnitOwed {
            key: vec![0xff, 0xfe],
            in_order_at: 0,
            arranged_by: crate::classes::WhatArrangedIt::AnAutoDownloadArrangement,
            the_destination_normalised: b"/Users/a/Downloads".to_vec(),
            the_landing_path: b"/Users/a/Downloads/x".to_vec(),
        };
        let refused = running
            .arrange_the_fetch_for(
                &arrangement,
                &unit,
                b"/jobs/abc",
                an_identity(IdentityKind::Transfer),
                None,
                1,
                an_identity(IdentityKind::Run),
            )
            .expect_err("a name that is not text cannot be carried");
        assert_eq!(
            refused.which,
            super::super::fetching::WhichPath::OnTheMachine
        );
    }

    /// **E1 step 3: a published transfer is *fetched*, the entry carries its
    /// transfer identity, and the slot goes back.**
    ///
    /// **Nothing took the fetch's answer back before this.** No file was ever
    /// recorded as arrived, so no arrangement could ever be done and
    /// auto-download queued every judged frame and fetched none.
    #[test]
    fn a_published_fetch_is_recorded_fetched_with_its_transfer_and_gives_its_slot_back() {
        let mut machine = AMachine::default();
        let mut keeper = AList::default();
        let mut queue = TheFetchQueue::at_a_server();
        let mut arrangement = an_arrangement();
        let mut running = TheArrangementsRun::fresh();
        let narrowed = entries();
        let order = an_order();
        let unit = a_fetch_in_flight(
            &mut machine,
            &mut keeper,
            &mut queue,
            &mut arrangement,
            &mut running,
            &narrowed,
            &order,
        );
        assert_eq!(queue.in_flight(), 1);

        let transfer = an_identity(IdentityKind::Transfer);
        let followed = running
            .a_fetch_ended(
                &mut keeper,
                &mut queue,
                &mut TheProbesCadence::fresh(),
                &mut arrangement,
                &in_hand(&narrowed, &order),
                unit,
                WhatTheUnitsFetchCameTo::ItEnded {
                    transfer: transfer.clone(),
                    ending: smoozy_server_protocol::transfers::Ending::Published,
                    taken_by: None,
                    the_write_failed: None,
                },
            )
            .expect("the store took it");

        assert_eq!(
            followed.the_entry_now_stands_at,
            Some(WhatSmoozyDid::Fetched)
        );
        let entry = arrangement.entry_for(b"0001.exr").expect("it stands");
        assert_eq!(entry.account.state, WhatSmoozyDid::Fetched);
        assert_eq!(entry.account.transfer, Some(transfer));
        assert_eq!(queue.in_flight(), 0);
    }

    /// **P3: a unit that failed for a server-side or transfer reason returns
    /// to *owed* and is re-arranged at a later walk cycle**, as a new transfer
    /// with a new identity, and never retried inside its own action.
    #[test]
    fn a_failed_fetch_returns_to_owed_and_the_arrangement_is_not_done() {
        for ending in [
            smoozy_server_protocol::transfers::Ending::Failed,
            smoozy_server_protocol::transfers::Ending::Stalled,
            smoozy_server_protocol::transfers::Ending::Interrupted,
        ] {
            let mut machine = AMachine::default();
            let mut keeper = AList::default();
            let mut queue = TheFetchQueue::at_a_server();
            let mut arrangement = an_arrangement();
            let mut running = TheArrangementsRun::fresh();
            let narrowed = entries();
            let order = an_order();
            let unit = a_fetch_in_flight(
                &mut machine,
                &mut keeper,
                &mut queue,
                &mut arrangement,
                &mut running,
                &narrowed,
                &order,
            );

            let followed = running
                .a_fetch_ended(
                    &mut keeper,
                    &mut queue,
                    &mut TheProbesCadence::fresh(),
                    &mut arrangement,
                    &in_hand(&narrowed, &order),
                    unit,
                    WhatTheUnitsFetchCameTo::ItEnded {
                        transfer: an_identity(IdentityKind::Transfer),
                        ending,
                        taken_by: None,
                        the_write_failed: None,
                    },
                )
                .expect("the store took it");

            assert_eq!(followed.the_entry_now_stands_at, None);
            assert_eq!(queue.owed(), 1);
            assert_eq!(queue.in_flight(), 0);
            assert!(!every_unit_is_terminal(&arrangement));
            assert_eq!(followed.the_diary_act, None);
        }
    }

    /// **P6: where the exclusive create fails, the name is taken, and the
    /// reckoning says by what** — which is Row 212's second promise kept at
    /// the only moment it can be kept.
    #[test]
    fn a_publication_refusal_is_classified_by_p6_and_written_at_the_entry() {
        for (taken_by, expected) in [
            (
                crate::fetching::WhatTheTakenNameIs::NothingTheRecordNames,
                WhatSmoozyDid::LeftAlone,
            ),
            (
                crate::fetching::WhatTheTakenNameIs::ThisDocumentsOwnFetchUnderAnotherSpelling,
                WhatSmoozyDid::CannotLand,
            ),
            (
                crate::fetching::WhatTheTakenNameIs::AFetchedEntryAtTheByteEqualLandingPath,
                WhatSmoozyDid::Fetched,
            ),
        ] {
            let mut machine = AMachine::default();
            let mut keeper = AList::default();
            let mut queue = TheFetchQueue::at_a_server();
            let mut arrangement = an_arrangement();
            let mut running = TheArrangementsRun::fresh();
            let narrowed = entries();
            let order = an_order();
            let unit = a_fetch_in_flight(
                &mut machine,
                &mut keeper,
                &mut queue,
                &mut arrangement,
                &mut running,
                &narrowed,
                &order,
            );

            let followed = running
                .a_fetch_ended(
                    &mut keeper,
                    &mut queue,
                    &mut TheProbesCadence::fresh(),
                    &mut arrangement,
                    &in_hand(&narrowed, &order),
                    unit,
                    WhatTheUnitsFetchCameTo::ItEnded {
                        transfer: an_identity(IdentityKind::Transfer),
                        ending: smoozy_server_protocol::transfers::Ending::Stopped(
                            smoozy_server_protocol::transfers::StoppedReason::AbandonedByItsRequest,
                        ),
                        taken_by: Some(taken_by),
                        the_write_failed: None,
                    },
                )
                .expect("the store took it");
            assert_eq!(followed.the_entry_now_stands_at, Some(expected));
        }
    }

    /// **D4 and D6: a local write failure raises one condition, keyed on the
    /// refusal's kind and the arrangement's own destination folder — and the
    /// hold reaches this arrangement's units and no other's.**
    #[test]
    fn a_write_failure_raises_a_destination_condition_and_holds_this_arrangement_alone() {
        let mut machine = AMachine::default();
        let mut keeper = AList::default();
        let mut queue = TheFetchQueue::at_a_server();
        let mut arrangement = an_arrangement();
        let mut running = TheArrangementsRun::fresh();
        let narrowed = entries();
        let order = an_order();
        let unit = a_fetch_in_flight(
            &mut machine,
            &mut keeper,
            &mut queue,
            &mut arrangement,
            &mut running,
            &narrowed,
            &order,
        );

        let mut probing = TheProbesCadence::fresh();
        let followed = running
            .a_fetch_ended(
                &mut keeper,
                &mut queue,
                &mut probing,
                &mut arrangement,
                &in_hand(&narrowed, &order),
                unit,
                WhatTheUnitsFetchCameTo::ItEnded {
                    transfer: an_identity(IdentityKind::Transfer),
                    ending: smoozy_server_protocol::transfers::Ending::Failed,
                    taken_by: None,
                    the_write_failed: Some(crate::destination::WhatTheAdapterSaid::TheVolumeIsFull),
                },
            )
            .expect("the store took it");

        // **D7: the probe's cadence starts with the condition**, so the
        // arrangement keeps only its watch and its probe and D8 has something
        // to withdraw the condition with.
        assert!(probing.a_condition_stands());

        let key = followed
            .a_destination_condition_was_raised
            .expect("a condition was raised");
        assert_eq!(
            key,
            crate::destination::the_condition_key(
                crate::destination::WhatTheAdapterSaid::TheVolumeIsFull,
                b"/Users/a/Downloads".to_vec()
            )
        );
        // The unit came back to *owed* and is held rather than offered.
        assert_eq!(queue.owed(), 1);
        assert_eq!(queue.offer(&[]), None);
        // A unit of an arrangement aimed elsewhere is untouched.
        queue.owe(AUnitOwed {
            key: b"other".to_vec(),
            in_order_at: 0,
            arranged_by: crate::classes::WhatArrangedIt::AnAutoDownloadArrangement,
            the_destination_normalised: b"/Volumes/work".to_vec(),
            the_landing_path: b"/Volumes/work/other".to_vec(),
        });
        assert_eq!(
            queue.offer(&[]).map(|unit| unit.key),
            Some(b"other".to_vec())
        );
    }

    /// **A path that is not valid text refuses the arrangement, and the
    /// refusal now has a state to land in.**
    ///
    /// **The refusal itself is right and is not softened** — nothing lossily
    /// converts a name. What it lacked was a place to land: such a unit sat at
    /// *owed* for ever and the arrangement could never be done.
    #[test]
    fn a_name_that_could_not_be_carried_is_cannot_land_rather_than_owed_for_ever() {
        let mut machine = AMachine::default();
        let mut keeper = AList::default();
        let mut queue = TheFetchQueue::at_a_server();
        let mut arrangement = an_arrangement();
        let mut running = TheArrangementsRun::fresh();
        let narrowed = entries();
        let order = an_order();
        let unit = a_fetch_in_flight(
            &mut machine,
            &mut keeper,
            &mut queue,
            &mut arrangement,
            &mut running,
            &narrowed,
            &order,
        );

        let followed = running
            .a_fetch_ended(
                &mut keeper,
                &mut queue,
                &mut TheProbesCadence::fresh(),
                &mut arrangement,
                &in_hand(&narrowed, &order),
                unit,
                WhatTheUnitsFetchCameTo::TheNameCouldNotBeCarried(
                    super::super::fetching::TheNameCannotBeCarried {
                        which: super::super::fetching::WhichPath::OnTheMachine,
                        path: vec![b'/', 0xff],
                    },
                ),
            )
            .expect("the store took it");

        assert_eq!(
            followed.the_entry_now_stands_at,
            Some(WhatSmoozyDid::CannotLand)
        );
        assert!(followed.the_name_that_could_not_be_carried.is_some());
        assert_eq!(queue.in_flight(), 0);
    }

    /// **E1 step 4 and §12: the arrangement becomes done when every unit is
    /// terminal, and the diary act is composed at that moment and no other.**
    #[test]
    fn the_diary_act_is_composed_at_the_done_transition_and_never_before_it() {
        let mut machine = AMachine::default();
        machine.holding(
            b"/jobs/abc",
            vec![file("0001.exr", 40, 1), file("0002.exr", 40, 2)],
        );
        let mut keeper = AList::default();
        let mut queue = TheFetchQueue::at_a_server();
        let mut arrangement = an_arrangement();
        let mut running = TheArrangementsRun::fresh();
        running.the_ending_arrived(Ending::Finished);
        let narrowed = entries();
        let order = an_order();

        run(running.one_turn(
            &mut machine,
            &EverythingIsWritable,
            &mut keeper,
            &mut queue,
            &mut arrangement,
            in_hand(&narrowed, &order),
        ))
        .expect("the store took it");
        // Two units are owed and one is *never coming*.
        assert_eq!(queue.owed(), 2);

        let mut acts = Vec::new();
        while let Some(unit) = queue.offer(&[]) {
            let followed = running
                .a_fetch_ended(
                    &mut keeper,
                    &mut queue,
                    &mut TheProbesCadence::fresh(),
                    &mut arrangement,
                    &in_hand(&narrowed, &order),
                    unit,
                    WhatTheUnitsFetchCameTo::ItEnded {
                        transfer: an_identity(IdentityKind::Transfer),
                        ending: smoozy_server_protocol::transfers::Ending::Published,
                        taken_by: None,
                        the_write_failed: None,
                    },
                )
                .expect("the store took it");
            acts.push(followed.the_diary_act);
        }

        // **Once, at the last unit**, and not before it.
        assert_eq!(acts.len(), 2);
        assert!(acts[0].is_none());
        match acts[1].as_ref().expect("the act at done") {
            crate::diary::AnAct::AnAutoDownloadArrangementDone {
                reading,
                never_coming,
                folders,
                ..
            } => {
                assert!(reading.says_ended());
                assert!(reading.is_short());
                assert_eq!(*never_coming, 1);
                assert_eq!(folders, &vec![b"/Users/a/Downloads".to_vec()]);
            }
            other => panic!("{other:?}"),
        }
        // And the turn now says the arrangement is done.
        let turn = run(running.one_turn(
            &mut machine,
            &EverythingIsWritable,
            &mut keeper,
            &mut queue,
            &mut arrangement,
            in_hand(&narrowed, &order),
        ))
        .expect("the store took it");
        assert_eq!(turn, WhatTheTurnDid::ItIsDone);
    }

    /// **N7 and ruling 3: a file the user narrowed away is written at *skipped
    /// by narrowing*, not left unentered.**
    ///
    /// N7's third bullet has such an entry leave the whole, and R4 re-opens it
    /// to *owed* where the narrowing widens. **Neither can happen to an entry
    /// that was never written**, so recording it is what makes the promise
    /// that changing their mind costs the user nothing keepable at all.
    #[test]
    fn a_narrowed_away_file_is_recorded_as_skipped_and_never_fetched() {
        let narrowed = vec![ANarrowedEntry {
            predicted_path: b"f_0001.png".to_vec(),
            frame: Some(1),
        }];
        let excluded = vec![ANarrowedEntry {
            predicted_path: b"f_0001.exr".to_vec(),
            frame: Some(1),
        }];
        let mut machine = AMachine::default();
        machine.holding(
            b"/jobs/abc",
            vec![file("f_0001.png", 40, 1), file("f_0001.exr", 900, 1)],
        );
        let mut keeper = AList::default();
        let mut queue = TheFetchQueue::at_a_server();
        let mut arrangement = an_arrangement();
        let mut running = TheArrangementsRun::fresh();
        running.the_ending_arrived(Ending::Finished);
        let order = an_order();
        let in_hand = WhatIsInHand {
            the_job_output_folder: b"/jobs/abc",
            narrowed_entries: &narrowed,
            the_whole_may_grow: false,
            the_entries_the_narrowing_excludes: &excluded,
            order: &order,
            the_jobs_label: None,
            the_first_frame_pair: None,
            the_pairs_other_half_fetched: &[],
            the_placed_copies: None,
            the_four_values_of_the_placed_copies: &[],
            window: an_open_window(),
            arrangements_walking_this_server: 1,
        };

        run(running.one_turn(
            &mut machine,
            &EverythingIsWritable,
            &mut keeper,
            &mut queue,
            &mut arrangement,
            in_hand,
        ))
        .expect("the store took it");

        assert_eq!(
            arrangement
                .entry_for(b"f_0001.exr")
                .expect("the excluded entry is written")
                .account
                .state,
            WhatSmoozyDid::SkippedByNarrowing
        );
        // And no fetch was arranged for it.
        assert_eq!(queue.owed(), 1);
        let unit = queue.offer(&[]).expect("a unit is granted");
        assert_eq!(unit.key, b"f_0001.png".to_vec());
    }

    /// **R11 read with must-do 2: switching auto-download off and on with a
    /// *narrower* choice does not hand back the files the user just refused.**
    ///
    /// The reset re-opens every *skipped by narrowing* entry to *owed*, which
    /// is R11's *reset to everything*. **On its own that is the whole fault**:
    /// the user switches off, switches on asking for PNG alone, and every EXR
    /// they had just refused is owed again. The new narrowing is run over the
    /// record in the same act and puts them back.
    #[test]
    fn switching_off_and_on_with_a_narrower_choice_never_hands_back_the_refused_files() {
        let mut keeper = AList::default();
        let mut arrangement = an_arrangement();
        arrangement.reckoning.push(AReckoningEntry::acted_on(
            b"f_0001.exr".to_vec(),
            Some(1),
            WhatSmoozyDid::SkippedByNarrowing,
            arrangement.destination.clone(),
        ));
        arrangement.reckoning.push(AReckoningEntry::acted_on(
            b"f_0001.tif".to_vec(),
            Some(1),
            WhatSmoozyDid::SkippedByNarrowing,
            arrangement.destination.clone(),
        ));
        arrangement.switched_off();
        let mut running = TheArrangementsRun::fresh();

        // The new choice is PNG alone, so the EXR is still excluded and the
        // TIFF is the one the user has now asked back.
        let narrowed = vec![ANarrowedEntry {
            predicted_path: b"f_0001.tif".to_vec(),
            frame: Some(1),
        }];
        let excluded = vec![ANarrowedEntry {
            predicted_path: b"f_0001.exr".to_vec(),
            frame: Some(1),
        }];
        let order = an_order();
        let in_hand = WhatIsInHand {
            the_job_output_folder: b"/jobs/abc",
            narrowed_entries: &narrowed,
            the_whole_may_grow: false,
            the_entries_the_narrowing_excludes: &excluded,
            order: &order,
            the_jobs_label: None,
            the_first_frame_pair: None,
            the_pairs_other_half_fetched: &[],
            the_placed_copies: None,
            the_four_values_of_the_placed_copies: &[],
            window: an_open_window(),
            arrangements_walking_this_server: 1,
        };

        running
            .switched_on_again(
                &mut keeper,
                &mut arrangement,
                b"/Users/a/Downloads".to_vec(),
                TheNarrowing::TheseFormats(vec![b"TIFF".to_vec()]),
                &in_hand,
            )
            .expect("the store took it");

        let state = |path: &[u8]| {
            arrangement
                .entry_for(path)
                .expect("the entry stands")
                .account
                .state
                .clone()
        };
        assert_eq!(state(b"f_0001.exr"), WhatSmoozyDid::SkippedByNarrowing);
        assert_eq!(state(b"f_0001.tif"), WhatSmoozyDid::Owed);
    }

    /// **P10, P11 and M9: an arrangement's units land inside the job's own
    /// folder, and the destination is the folder that contains it.**
    ///
    /// The spec puts a whole job's outputs inside the job's own folder
    /// *"whichever way they were asked for"*, and *"two jobs' outputs no more
    /// mix on the user's disk than they do on the server's"*. **Nothing
    /// anywhere used to put a job's folder between the destination and the
    /// files on this route**, so two jobs downloading into one Downloads
    /// folder mixed their frames.
    #[test]
    fn an_arrangements_units_land_inside_the_jobs_own_folder_under_p10s_name() {
        let mut machine = AMachine::default();
        machine.holding(b"/jobs/abc", vec![file("0001.exr", 40, 1)]);
        let mut keeper = AList::default();
        let mut queue = TheFetchQueue::at_a_server();
        let mut arrangement = an_arrangement();
        let mut running = TheArrangementsRun::fresh();
        running.the_ending_arrived(Ending::Finished);
        let narrowed = entries();
        let order = an_order();

        run(running.one_turn(
            &mut machine,
            &EverythingIsWritable,
            &mut keeper,
            &mut queue,
            &mut arrangement,
            in_hand(&narrowed, &order),
        ))
        .expect("the store took it");

        let unit = queue.offer(&[]).expect("a unit is granted");
        let composed = crate::fetching::the_job_folders_landing_name(
            &EverythingIsWritable,
            &arrangement.job,
            Some("night"),
        );
        let mut expected = b"/Users/a/Downloads/".to_vec();
        expected.extend_from_slice(&composed);
        expected.extend_from_slice(b"/0001.exr");
        assert_eq!(unit.the_landing_path, expected);
        // **The destination stays the folder that contains it**, which is what
        // the hold and R1's fifth bullet both key on.
        assert_eq!(
            unit.the_destination_normalised,
            b"/Users/a/Downloads".to_vec()
        );
    }

    /// **R4 and R9: *never coming* is never written over a file the user
    /// already has.**
    ///
    /// Attempt 1 renders frames 1 to 100 and they come home. The user re-runs
    /// with a shorter list; `Job files and outputs` I22 replaces what was left
    /// on the machine rather than adding to it, so the earlier frames are gone
    /// from the folder. The ending walk then finds them absent — **and must
    /// not tell the user files never came home that are sitting in their
    /// Downloads folder.**
    #[test]
    fn a_file_already_fetched_is_never_re_recorded_as_never_coming() {
        let mut machine = AMachine::default();
        // The folder holds only the frame the second attempt rendered.
        machine.holding(b"/jobs/abc", vec![file("0003.exr", 40, 3)]);
        let mut keeper = AList::default();
        let mut queue = TheFetchQueue::at_a_server();
        let mut arrangement = an_arrangement();
        arrangement.reckoning.push(AReckoningEntry::acted_on(
            b"0001.exr".to_vec(),
            Some(1),
            WhatSmoozyDid::Fetched,
            arrangement.destination.clone(),
        ));
        arrangement.reckoning.push(AReckoningEntry::acted_on(
            b"0002.exr".to_vec(),
            Some(2),
            WhatSmoozyDid::LeftAlone,
            arrangement.destination.clone(),
        ));
        let mut running = TheArrangementsRun::fresh();
        running.the_ending_arrived(Ending::Finished);
        let narrowed = entries();
        let order = an_order();

        run(running.one_turn(
            &mut machine,
            &EverythingIsWritable,
            &mut keeper,
            &mut queue,
            &mut arrangement,
            in_hand(&narrowed, &order),
        ))
        .expect("the store took it");

        let state = |path: &[u8]| {
            arrangement
                .entry_for(path)
                .expect("the entry stands")
                .account
                .state
                .clone()
        };
        assert_eq!(state(b"0001.exr"), WhatSmoozyDid::Fetched);
        assert_eq!(state(b"0002.exr"), WhatSmoozyDid::LeftAlone);
        // The one the record does not name is the only one the walk may write.
        assert_eq!(state(b"0003.exr"), WhatSmoozyDid::Owed);
    }

    /// **R4's entry is two halves and a state change keeps both.**
    ///
    /// The transfer identity and the server-file identity are the entry's own
    /// history; a write that replaced the entry wholesale threw them away
    /// every time a state moved.
    #[test]
    fn a_state_change_merges_into_the_entry_and_keeps_its_transfer_and_its_reading() {
        let mut machine = AMachine::default();
        machine.holding(b"/jobs/abc", vec![file("0001.exr", 0, 1)]);
        let mut keeper = AList::default();
        let mut queue = TheFetchQueue::at_a_server();
        let mut arrangement = an_arrangement();
        let transfer = an_identity(IdentityKind::Transfer);
        arrangement.reckoning.push(AReckoningEntry::acted_on(
            b"0001.exr".to_vec(),
            Some(1),
            WhatSmoozyDid::Owed,
            arrangement.destination.clone(),
        ));
        arrangement.reckoning[0].account.transfer = Some(transfer.clone());
        let mut running = TheArrangementsRun::fresh();
        running.the_ending_arrived(Ending::Finished);
        let narrowed = entries();
        let order = an_order();

        run(running.one_turn(
            &mut machine,
            &EverythingIsWritable,
            &mut keeper,
            &mut queue,
            &mut arrangement,
            in_hand(&narrowed, &order),
        ))
        .expect("the store took it");

        let entry = arrangement.entry_for(b"0001.exr").expect("it stands");
        // A zero-byte file at the ending walk is *never coming*.
        assert_eq!(entry.account.state, WhatSmoozyDid::NeverComing);
        assert_eq!(entry.account.transfer, Some(transfer));
    }

    /// **P2 read against J2: the ordering value a unit joins the queue with is
    /// its frame's place in the attempt's order.**
    ///
    /// The attempt was written `50, 10, 20`, and the inventory holds its
    /// entries in a different order again. **Neither the frame's number nor
    /// the inventory's order decides**: frame 50 was rendered first, so it is
    /// fetched first.
    #[test]
    fn a_unit_joins_the_queue_at_its_frames_place_in_the_attempts_order() {
        let narrowed = vec![
            ANarrowedEntry {
                predicted_path: b"f_0010.exr".to_vec(),
                frame: Some(10),
            },
            ANarrowedEntry {
                predicted_path: b"f_0020.exr".to_vec(),
                frame: Some(20),
            },
            ANarrowedEntry {
                predicted_path: b"f_0050.exr".to_vec(),
                frame: Some(50),
            },
        ];
        let order = TheOrder::The(
            crate::vouching::TheAttemptsOrder::as_the_attempt_record_carries_it(vec![50, 10, 20]),
        );
        let mut machine = AMachine::default();
        machine.holding(
            b"/jobs/abc",
            vec![
                file("f_0010.exr", 40, 1),
                file("f_0020.exr", 40, 2),
                file("f_0050.exr", 40, 3),
            ],
        );
        let mut keeper = AList::default();
        let mut queue = TheFetchQueue::at_a_server();
        let mut arrangement = an_arrangement();
        let mut running = TheArrangementsRun::fresh();
        running.the_ending_arrived(Ending::Finished);

        run(running.one_turn(
            &mut machine,
            &EverythingIsWritable,
            &mut keeper,
            &mut queue,
            &mut arrangement,
            in_hand(&narrowed, &order),
        ))
        .expect("the store took it");

        assert_eq!(queue.owed(), 3);
        let mut taken = Vec::new();
        while let Some(unit) = queue.offer(&[]) {
            taken.push(unit.key);
            queue.a_unit_ended();
        }
        assert_eq!(
            taken,
            vec![
                b"f_0050.exr".to_vec(),
                b"f_0010.exr".to_vec(),
                b"f_0020.exr".to_vec()
            ]
        );
    }

    /// **W5: a discarded walk is reported as the truth, with the separated
    /// cause — on the ordinary walk and on the ending walk alike.**
    ///
    /// Without the cause the arrangement has nothing to give: the job folder
    /// becomes unreadable to the login, the walks are discarded at the cadence
    /// for ever, nothing arrives, and Smoozy goes quiet.
    #[test]
    fn a_discarded_walk_carries_why_and_what_the_machine_said_on_both_paths() {
        let mut machine = AMachine::default();
        machine.refusing(
            b"/jobs/abc",
            SubsystemFailure::TheServerRefused(StatusCode::PermissionDenied),
        );
        let mut keeper = AList::default();
        let mut queue = TheFetchQueue::at_a_server();
        let mut arrangement = an_arrangement();
        let mut running = TheArrangementsRun::fresh();
        let narrowed = entries();
        let order = an_order();

        // The ordinary walk.
        let turn = run(running.one_turn(
            &mut machine,
            &EverythingIsWritable,
            &mut keeper,
            &mut queue,
            &mut arrangement,
            in_hand(&narrowed, &order),
        ))
        .expect("the store took it");
        match turn {
            WhatTheTurnDid::TheWalkWasDiscarded {
                why,
                what_the_machine_said,
                ..
            } => {
                assert_eq!(why, WhyTheWalkIsNotAReading::ARefusal);
                assert_eq!(
                    what_the_machine_said,
                    Some(SubsystemFailure::TheServerRefused(
                        StatusCode::PermissionDenied
                    ))
                );
            }
            other => panic!("expected a discarded walk, got {other:?}"),
        }

        // And the ending walk, which is the path that never built the cause
        // at all.
        running.the_ending_arrived(Ending::Finished);
        machine.refusing(b"/jobs/abc", SubsystemFailure::TheChannelIsGone);
        let turn = run(running.one_turn(
            &mut machine,
            &EverythingIsWritable,
            &mut keeper,
            &mut queue,
            &mut arrangement,
            in_hand(&narrowed, &order),
        ))
        .expect("the store took it");
        match turn {
            WhatTheTurnDid::TheWalkWasDiscarded {
                why,
                what_the_machine_said,
                ..
            } => {
                assert_eq!(why, WhyTheWalkIsNotAReading::TheConnectionWent);
                assert_eq!(
                    what_the_machine_said,
                    Some(SubsystemFailure::TheChannelIsGone)
                );
            }
            other => panic!("expected a discarded ending walk, got {other:?}"),
        }
    }

    /// **E1 step 2: where the frame boundary cannot be established, the mark
    /// falls on the entries *active* in the last complete walk — never on the
    /// whole folder.**
    ///
    /// The attempt record could not be read and the job ended badly. Three
    /// files stand in the folder and only one of them moved at the last
    /// complete walk, so only that one carries the mark. **Handing in the
    /// whole walk would name every file the user just downloaded as one
    /// Smoozy cannot vouch for**, and a mark that falls on everything teaches
    /// the user it means nothing.
    #[test]
    fn a_badly_ended_job_with_no_readable_order_marks_only_what_was_active_at_the_last_walk() {
        let mut machine = AMachine::default();
        let mut keeper = AList::default();
        let mut queue = TheFetchQueue::at_a_server();
        let mut arrangement = an_arrangement();
        let mut running = TheArrangementsRun::fresh();
        let narrowed = entries();
        let order = TheOrder::ItCouldNotBeRead;

        // The baseline walk: everything reads as active, and it judges
        // nothing.
        machine.holding(
            b"/jobs/abc",
            vec![
                file("0001.exr", 40, 1),
                file("0002.exr", 40, 2),
                file("0003.exr", 10, 3),
            ],
        );
        run(running.one_turn(
            &mut machine,
            &EverythingIsWritable,
            &mut keeper,
            &mut queue,
            &mut arrangement,
            in_hand(&narrowed, &order),
        ))
        .expect("the store took it");

        // One more complete walk, at which only the third file moved.
        machine.holding(
            b"/jobs/abc",
            vec![
                file("0001.exr", 40, 1),
                file("0002.exr", 40, 2),
                file("0003.exr", 22, 4),
            ],
        );
        run(running.one_turn(
            &mut machine,
            &EverythingIsWritable,
            &mut keeper,
            &mut queue,
            &mut arrangement,
            in_hand(&narrowed, &order),
        ))
        .expect("the store took it");

        running.the_ending_arrived(Ending::Stopped);
        let turn = run(running.one_turn(
            &mut machine,
            &EverythingIsWritable,
            &mut keeper,
            &mut queue,
            &mut arrangement,
            in_hand(&narrowed, &order),
        ))
        .expect("the store took it");

        match turn {
            WhatTheTurnDid::TheEndingWalkRan(verdict) => assert_eq!(
                verdict.could_not_be_vouched_for,
                vec![b"0003.exr".to_vec()],
                "the mark fell on the whole folder rather than on what was active"
            ),
            other => panic!("expected the ending walk, got {other:?}"),
        }
    }

    /// **J6: the catch-up vouches for everything before the earliest frame
    /// *active since the baseline*, and the baseline's own entries are not in
    /// that set.**
    ///
    /// The user switches auto-download on forty frames into the render. The
    /// baseline walk shows forty finished frames, judges nothing, **and puts
    /// nothing into the since-baseline set** — on a fresh series every entry
    /// reads as active, so a set filled there would hold every frame the
    /// folder already has. Frame 41's first write is then the first activity
    /// since the baseline, and the catch-up vouches for 1 to 40.
    ///
    /// **The set is asserted as well as the outcome**, because mechanism one's
    /// live frontier happens to cover the same ground at that moment: a test
    /// that read only what became fetchable would pass over a mechanism two
    /// that fired on the whole folder and vouched for nothing.
    #[test]
    fn the_catch_up_vouches_for_everything_before_the_first_write_since_the_baseline() {
        let narrowed: Vec<ANarrowedEntry> = (1..=41)
            .map(|frame| ANarrowedEntry {
                predicted_path: format!("f_{frame:04}.exr").into_bytes(),
                frame: Some(frame),
            })
            .collect();
        let order = TheOrder::The(
            crate::vouching::TheAttemptsOrder::as_the_attempt_record_carries_it((1..=41).collect()),
        );

        let already_there: Vec<ADirectoryEntry> = (1..=40)
            .map(|frame| file(&format!("f_{frame:04}.exr"), 4096, frame))
            .collect();
        let mut machine = AMachine::default();
        machine.holding(b"/jobs/abc", already_there.clone());
        let mut keeper = AList::default();
        let mut queue = TheFetchQueue::at_a_server();
        let mut arrangement = an_arrangement();
        let mut running = TheArrangementsRun::fresh();

        run(running.one_turn(
            &mut machine,
            &EverythingIsWritable,
            &mut keeper,
            &mut queue,
            &mut arrangement,
            in_hand(&narrowed, &order),
        ))
        .expect("the store took it");
        // **The baseline judges nothing and remembers nothing.**
        assert!(running.active_since_the_baseline.is_empty());

        let mut with_41 = already_there;
        with_41.push(file("f_0041.exr", 12, 41));
        machine.holding(b"/jobs/abc", with_41);
        let turn = run(running.one_turn(
            &mut machine,
            &EverythingIsWritable,
            &mut keeper,
            &mut queue,
            &mut arrangement,
            in_hand(&narrowed, &order),
        ))
        .expect("the store took it");

        // Frame 41's write is the only activity since the baseline.
        assert_eq!(running.active_since_the_baseline, vec![41]);
        assert!(running.the_catch_up_has_judged);
        assert_eq!(
            what_the_catch_up_vouches_for(
                &order,
                &TheCatchUpsSignal::ActivitySinceTheBaseline(
                    running.active_since_the_baseline.clone()
                ),
            ),
            crate::vouching::WhatTheCatchUpVouchesFor::TheseFrames((1..=40).collect::<Vec<i64>>())
        );

        match turn {
            WhatTheTurnDid::ItWalked {
                became_fetchable, ..
            } => {
                assert_eq!(became_fetchable.len(), 40);
                assert!(became_fetchable.contains(&b"f_0001.exr".to_vec()));
                assert!(became_fetchable.contains(&b"f_0040.exr".to_vec()));
                assert!(!became_fetchable.contains(&b"f_0041.exr".to_vec()));
            }
            other => panic!("expected a walk, got {other:?}"),
        }
    }

    /// **P8 and P9: a name the receiving system refuses is *cannot land*, the
    /// unit is terminal and never joins the queue, and the arrangement never
    /// waits on it.**
    #[test]
    fn a_refused_name_becomes_cannot_land_and_never_becomes_a_unit() {
        struct NothingMayBeWritten;
        impl TheReceivingSystemsRules for NothingMayBeWritten {
            fn check(&self, _component: &[u8]) -> TheNameCheckSaid {
                TheNameCheckSaid::TooLongInThisSystemsOwnUnit
            }
            fn substitute(&self, component: &[u8]) -> Vec<u8> {
                component.to_vec()
            }
            fn length_in_this_systems_own_unit(&self, component: &[u8]) -> usize {
                component.len()
            }
            fn the_longest_component(&self) -> usize {
                255
            }
        }

        let mut machine = AMachine::default();
        machine.holding(b"/jobs/abc", vec![file("0001.exr", 40, 1)]);
        let mut keeper = AList::default();
        let mut queue = TheFetchQueue::at_a_server();
        let mut arrangement = an_arrangement();
        let mut running = TheArrangementsRun::fresh();
        running.the_ending_arrived(Ending::Finished);
        let narrowed = entries();
        let order = an_order();

        run(running.one_turn(
            &mut machine,
            &NothingMayBeWritten,
            &mut keeper,
            &mut queue,
            &mut arrangement,
            in_hand(&narrowed, &order),
        ))
        .expect("the store took it");

        assert_eq!(queue.owed(), 0);
        assert_eq!(
            arrangement
                .entry_for(b"0001.exr")
                .expect("the entry stands")
                .account
                .state,
            WhatSmoozyDid::CannotLand
        );
    }
}
