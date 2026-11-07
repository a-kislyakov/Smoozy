//! The arrangement and its record — `Download` §2, R1 to R12, Rows 212, 217
//! and 394.
//!
//! **Constraint:** the spec's own words — *"each time it is answering what is
//! here that I have not taken yet, never shall I replace this."*
//!
//! **The record is one durable record in the local record and this crate holds
//! no store of its own.** The shape is here, the store is `Desktop app and
//! delivery` §5's, which is the split every member of this stage already
//! draws; [`KeepsTheArrangement`] is the seam and [`crate::arriving_later`]
//! names the real end.
//!
//! **R5 governs this whole file: the reckoning answers *what have I taken*,
//! and only the walk answers *what is there*.** No doneness judgement is ever
//! written here — a judgement is a derivation from readings, recomputed per
//! walk, and what is durable is what Smoozy did and the readings its acts were
//! keyed to.
//!
//! **Markers used below that belong to another document, each with the
//! document it belongs to.** `Server protocol and transfers` S9, S13, S15,
//! S26, S27, S28, C11, C13. `Desktop app and delivery` C10, C15, C19.
//! `Queue & History` Y8.
//!
//! **A marker this plan also defines is deliberately not listed above,
//! whichever document is meant, and is written out with its document at every
//! site instead** — the header list cannot tell the two apart, and a bare one
//! would resolve to a real and wrong clause. `Servers`' and `Rendering`'s
//! clause series collide with this plan's throughout, so both are always
//! written out.

use smoozy_server_protocol::transfers::{SourceReading, TransferRecord};
use smoozy_state::identities::{Identity, InstallationCheck, MachineDerivedValue};
use smoozy_state::observed::Reading;
use smoozy_state::outcomes::Refused;

/// **R1's narrowing field: a set of chosen formats, or the declared value
/// *everything*, which is no filter rather than a frozen list.**
///
/// The distinction is the whole of why a format list arriving later
/// invalidates nothing ([`crate::narrowing`]).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TheNarrowing {
    /// **No filter.** An arrangement made with no list covers everything, and
    /// this is a declared value rather than a list of every format there
    /// happened to be.
    Everything,
    /// The formats the user chose, as the recorded reading records them.
    TheseFormats(Vec<Vec<u8>>),
}

impl TheNarrowing {
    /// Whether this narrowing filters at all.
    pub fn is_no_filter(&self) -> bool {
        matches!(self, TheNarrowing::Everything)
    }
}

/// **R1's on/off state, which is the arrangement's durable desired state under
/// `State, identity and recovery` D1.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DesiredState {
    On,
    Off,
}

/// **R4's account half: what Smoozy itself did for one file.**
///
/// **This half is Smoozy's own history, durable, and never disqualified by
/// anything the machine does** — which is the whole reason the entry is split
/// in two.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TheAccountOfSmoozysOwnAct {
    /// That a fetch was arranged, by its transfer identity. **Absent where
    /// none was** — a unit skipped by the narrowing, or one the name check
    /// refused before any byte moved.
    pub transfer: Option<Identity>,
    pub state: WhatSmoozyDid,
    /// **R1's fifth bullet: the destination in force when this act was
    /// taken**, so a *fetched* entry's landing path stays resolvable after a
    /// switch-on has reset the destination. That is what
    /// [`WhatStandsAtADestinationName`]'s first kind needs to match anything
    /// at all once a job's outputs are split between two folders.
    pub destination_in_force: Vec<u8>,
}

/// **R4's five terminal states, with the one non-terminal state written
/// out beside them rather than left implicit.**
///
/// The plan states which re-open, by what event, and which are final,
/// *"because *terminal* alone leaves a builder to guess in either direction"*.
/// [`WhatSmoozyDid::what_re_opens_it`] is that statement, held by the type.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WhatSmoozyDid {
    /// **Not one of R4's five.** It is what a fresh entry carries before its
    /// act and what a re-opened entry becomes, and it is written out so that
    /// nothing has to read the absence of a terminal state as one.
    Owed,
    /// **Final.** R9, and nothing re-opens it.
    Fetched,
    /// **Final, and it is said rather than left to be inferred.** P6's amber
    /// line names those files to the user and the natural next act is to clear
    /// them; a user who does that is owed the sentence saying nothing more
    /// will happen, because the arrangement does not fetch over a name it has
    /// already passed over.
    LeftAlone,
    /// **Final.** P9 says so: the name check has already refused the name and
    /// nothing later makes it legal.
    CannotLand,
    /// **Re-opened to *owed*** where the narrowing is widened, or reset to
    /// everything by a switch-off-and-on, with the whole recomputed. Without
    /// it the arrangement would stand permanently short by exactly the files
    /// the user has just asked to have back, and silently.
    SkippedByNarrowing,
    /// **Re-opened to *owed*** for the frames a new attempt's list names, on
    /// the render-start that re-arms the arrangement. A frame absent at one
    /// attempt's ending walk is the frame the next attempt renders.
    NeverComing,
}

/// The events R4 names as re-opening an entry, and no others.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhatHappened {
    /// The user widened the narrowing, or reset it to everything by switching
    /// the arrangement off and on again.
    TheNarrowingWidened,
    /// A new attempt's render-start, whose ordered frame list names this
    /// entry's frame.
    ARenderStartNamingThisEntrysFrame,
}

impl WhatSmoozyDid {
    /// **Which states are R4's five terminal ones.**
    pub fn is_terminal(&self) -> bool {
        !matches!(self, WhatSmoozyDid::Owed)
    }

    /// **R4's statement of which terminal states re-open and by what**, and
    /// the answer for every other pairing is that nothing moves.
    ///
    /// **Two of the five are final in both directions** — *fetched* and
    /// *cannot land* — and *left alone* is final too, which is why P6's amber
    /// line has to say so.
    pub fn after(&self, happened: WhatHappened) -> WhatSmoozyDid {
        match (self, happened) {
            (WhatSmoozyDid::SkippedByNarrowing, WhatHappened::TheNarrowingWidened) => {
                WhatSmoozyDid::Owed
            }
            (WhatSmoozyDid::NeverComing, WhatHappened::ARenderStartNamingThisEntrysFrame) => {
                WhatSmoozyDid::Owed
            }
            (already, _) => already.clone(),
        }
    }

    /// **R9: a file whose account says *fetched* is never fetched again**,
    /// whatever has become of the local copy.
    ///
    /// *What has already come home stays* is a promise about Smoozy not taking
    /// files back, not a promise to re-supply them; the manual whole-job
    /// download is the recovery route.
    pub fn a_fetch_may_be_arranged_for_it(&self) -> bool {
        matches!(self, WhatSmoozyDid::Owed)
    }
}

/// **R4's second half: the server-file identity the fetch was arranged
/// against.**
///
/// `Server protocol and transfers` S13's four values, read when the fetch was
/// arranged, carried with its moment and the installation identity it was read
/// under. **This half is a machine-derived value**: on an installation-identity
/// change it is disqualified under `State, identity and recovery` §2.3 while
/// the account half stands.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TheServerFileIdentity {
    /// The four values with their moment and the clock they were taken on
    /// (`State, identity and recovery` D5).
    pub reading: Reading<SourceReading>,
    /// The installation identity it was read under. **Absent is a state**: a
    /// reading taken where no installation identity was in hand carries none
    /// and is disqualified by the same rule that disqualifies a mismatch.
    pub read_under: Option<Identity>,
}

impl TheServerFileIdentity {
    /// **`State, identity and recovery` §2.3, consumed as that document
    /// states it** — this file makes no second rule about what a check means.
    pub fn what_becomes_of_it(&self, check: InstallationCheck) -> MachineDerivedValue {
        check.what_becomes_of_machine_derived_values()
    }

    /// Whether the four values may be compared at all.
    ///
    /// **A disqualified reading is never read, never shown and never used as
    /// authority**, so R7's newer-version comparison does not run over one.
    pub fn may_be_compared(&self, check: InstallationCheck) -> bool {
        matches!(self.what_becomes_of_it(check), MachineDerivedValue::Current)
    }
}

/// **R4: one entry per file the arrangement has acted on, keyed by the file's
/// predicted path relative to the job output folder — the inventory's own key,
/// and nothing else.**
///
/// **The attempt is not in the key**, and the fetch decision consults the
/// reckoning by predicted path alone: a re-rendered file arriving at a name
/// the reckoning already holds is told under [`what_a_re_read_says`] and is
/// never fetched over the user's copy. **An attempt number could not be
/// carried honestly in any case** — whether two attempts touched one name is
/// knowable only once the second one does, and nothing retro-labels an entry
/// written when there was one attempt.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AReckoningEntry {
    /// **R4's key.** The predicted path relative to the job output folder.
    pub predicted_path: Vec<u8>,
    /// **R4's frame**, carried from the inventory entry the key names, because
    /// the re-opening event of [`WhatHappened::ARenderStartNamingThisEntrysFrame`]
    /// is per frame. **Absent for an entry with no frame** — a video
    /// destination's, an added write place's file, an unmatched file
    /// ([`crate::vouching`]).
    pub frame: Option<i64>,
    pub account: TheAccountOfSmoozysOwnAct,
    /// **Absent where no fetch was ever arranged**, which is the ordinary case
    /// for a *skipped by narrowing* or a *never coming* entry.
    pub the_server_file_identity: Option<TheServerFileIdentity>,
}

impl AReckoningEntry {
    /// An entry as an act writes it, with the destination in force at that
    /// moment.
    pub fn acted_on(
        predicted_path: Vec<u8>,
        frame: Option<i64>,
        state: WhatSmoozyDid,
        destination_in_force: Vec<u8>,
    ) -> AReckoningEntry {
        AReckoningEntry {
            predicted_path,
            frame,
            account: TheAccountOfSmoozysOwnAct {
                transfer: None,
                state,
                destination_in_force,
            },
            the_server_file_identity: None,
        }
    }

    /// **R4's re-opening, applied to one entry.**
    ///
    /// The frame gate is part of the rule rather than the caller's: a
    /// render-start re-opens *never coming* **for the frames the new attempt's
    /// list names** and for no others.
    pub fn after(&mut self, happened: WhatHappened, the_attempts_frames: &[i64]) {
        if let WhatHappened::ARenderStartNamingThisEntrysFrame = happened {
            let named = self
                .frame
                .is_some_and(|frame| the_attempts_frames.contains(&frame));
            if !named {
                return;
            }
        }
        self.account.state = self.account.state.after(happened);
    }
}

/// **R1: an auto-download arrangement is one durable record in the local
/// record, held against the local job record.**
///
/// `State, identity and recovery` §1.1's inventory already names *transfer
/// arrangements*, and its §11.1 lists them **user-held**.
///
/// **R2: the arrangement is an intention and its own acts, never a copy of the
/// machine's queue.** Nothing in it is an observed machine value except the
/// per-fetch identity readings of [`TheServerFileIdentity`], which carry their
/// moment, their clock and the installation identity they were read under.
/// `Queue & History`'s rule that Smoozy keeps no lasting copy of a machine's
/// state is kept by construction.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AnArrangement {
    /// **R1: the job identity and the server row the job belongs to.**
    pub job: Identity,
    pub server_row: Identity,
    /// **R1: the destination** — the folder chosen or accepted when the
    /// arrangement was made, **fixed for the life of the job** (nothing can be
    /// asked of a user who is not there), and moved only by the user switching
    /// the arrangement off and on again, which resets it and asks once more.
    pub destination: Vec<u8>,
    pub narrowing: TheNarrowing,
    /// **R1: the durable desired state under
    /// `State, identity and recovery` D1.**
    pub desired_state: DesiredState,
    /// **R4's per-file reckoning.**
    pub reckoning: Vec<AReckoningEntry>,
}

impl AnArrangement {
    /// **R1's write, as a value.** Nothing here writes it down; that is
    /// [`KeepsTheArrangement`]'s caller's.
    ///
    /// It is created when auto-download is switched on — at submission
    /// (`Rendering`'s moment) or afterwards from the queue entry
    /// (`Queue & History`'s toggle).
    pub fn switched_on(
        job: Identity,
        server_row: Identity,
        destination: Vec<u8>,
        narrowing: TheNarrowing,
    ) -> AnArrangement {
        AnArrangement {
            job,
            server_row,
            destination,
            narrowing,
            desired_state: DesiredState::On,
            reckoning: Vec::new(),
        }
    }

    /// The entry the reckoning holds for a predicted path. **R4: by predicted
    /// path alone, the attempt ignored.**
    pub fn entry_for(&self, predicted_path: &[u8]) -> Option<&AReckoningEntry> {
        self.reckoning
            .iter()
            .find(|entry| entry.predicted_path == predicted_path)
    }

    /// **R8: the fetch decision is the reckoning's — *what have I taken* — and
    /// never a comparison against the destination.**
    ///
    /// A path the reckoning does not name at all is owed; a path it names is
    /// owed only where its own account says so. **The destination is consulted
    /// exactly once per file, by the exclusive create at publication**
    /// (`Desktop app and delivery` C10), which is the adapter's act and not a
    /// comparison.
    pub fn a_fetch_is_owed_for(&self, predicted_path: &[u8]) -> bool {
        match self.entry_for(predicted_path) {
            None => true,
            Some(entry) => entry.account.state.a_fetch_may_be_arranged_for_it(),
        }
    }

    /// **R3: switching the arrangement off ends the arranging of new fetches
    /// from that moment, removes nothing, and lets every transfer already
    /// arranged run to its ending.**
    ///
    /// The spec's own rule, Creator-stated 2026-07-22, because only a set
    /// number run at once, so that is a few files rather than many, and
    /// cutting them off would throw away work already half done. **It is the
    /// arranging that stops and not the fetching.**
    pub fn switched_off(&mut self) {
        self.desired_state = DesiredState::Off;
    }

    /// **R3 and R11: switching it on again resets destination and narrowing —
    /// two fields rewritten, Creator-stated — and keeps the per-file
    /// reckoning**, which is what makes *switching it on again catches up*
    /// answer *what is here that I have not taken yet* correctly.
    ///
    /// **Two things follow from the reset and both are stated rather than left
    /// to a builder.** The narrowing reset to everything re-opens the entries
    /// it had put out as *skipped by narrowing*, to *owed*, with the whole
    /// recomputed. And where the destination chosen at the switch-on differs
    /// from the one the arrangement previously held, and the reckoning already
    /// carries entries whose account says *fetched*, **the switch-on says so
    /// before it proceeds**: what is already downloaded stays where it is, and
    /// only what has not yet arrived comes to the new folder. That telling is
    /// the spec's own, Creator-stated 2026-08-20. **Wording and placement are
    /// Design's.**
    pub fn switched_on_again(
        &mut self,
        destination: Vec<u8>,
        narrowing: TheNarrowing,
    ) -> Option<TheDestinationMoved> {
        let telling = self.what_the_switch_on_has_to_say_first(&destination);
        let widened = narrowing.is_no_filter() || narrowing != self.narrowing;
        self.destination = destination;
        self.narrowing = narrowing;
        self.desired_state = DesiredState::On;
        if widened {
            for entry in &mut self.reckoning {
                entry.after(WhatHappened::TheNarrowingWidened, &[]);
            }
        }
        telling
    }

    /// **R11's telling, composed before the switch-on proceeds.**
    ///
    /// It fires on both halves together and on neither alone: a destination
    /// that has moved with nothing yet fetched costs the user nothing to say,
    /// and a reckoning full of *fetched* entries at an unchanged destination
    /// has split nothing.
    pub fn what_the_switch_on_has_to_say_first(
        &self,
        the_new_destination: &[u8],
    ) -> Option<TheDestinationMoved> {
        if the_new_destination == self.destination {
            return None;
        }
        if !self
            .reckoning
            .iter()
            .any(|entry| entry.account.state == WhatSmoozyDid::Fetched)
        {
            return None;
        }
        Some(TheDestinationMoved {
            what_is_already_downloaded_stays_at: self.destination.clone(),
            what_has_not_arrived_comes_to: the_new_destination.to_vec(),
        })
    }

    /// **R11's fourth survival, applied: a re-run writing into the same
    /// folder.**
    ///
    /// The reckoning is consulted by predicted path and ignores the attempt,
    /// so [`what_a_re_read_says`] handles every file an attempt re-writes and
    /// nothing is fetched over the user's copy; and the new render-start
    /// re-arms the arrangement, re-opening its *never coming* entries **for
    /// the frames the new attempt's list names**.
    pub fn a_render_start_re_arms_it(&mut self, the_attempts_frames: &[i64]) {
        for entry in &mut self.reckoning {
            entry.after(
                WhatHappened::ARenderStartNamingThisEntrysFrame,
                the_attempts_frames,
            );
        }
    }

    /// **H5's first source**, and R1's fifth bullet is exactly the field it
    /// needs: every entry standing at *fetched*, each carrying the destination
    /// in force when its own act was taken, **in the order each was first
    /// written to**.
    pub fn the_folders_it_has_fetched_into(&self) -> Vec<Vec<u8>> {
        let mut out: Vec<Vec<u8>> = Vec::new();
        for entry in &self.reckoning {
            if entry.account.state != WhatSmoozyDid::Fetched {
                continue;
            }
            let folder = &entry.account.destination_in_force;
            if !out.iter().any(|already| already == folder) {
                out.push(folder.clone());
            }
        }
        out
    }
}

/// **R11's telling when a switch-on has moved the destination part-way
/// through.** The words are Design's; these are the two places.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TheDestinationMoved {
    pub what_is_already_downloaded_stays_at: Vec<u8>,
    pub what_has_not_arrived_comes_to: Vec<u8>,
}

/// **R6: three kinds of file can stand at one destination name, and the
/// reckoning names all three.**
///
/// This is the same shape as `File Browser` Row 197's record on the upload
/// side: records name Smoozy's own material, and the residual class is the
/// world's.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WhatStandsAtADestinationName {
    /// **A file this arrangement fetched** — a reckoning entry whose account
    /// says *fetched*, matched by byte-equal landing path, or through the
    /// stored-name outcome of `Desktop app and delivery` C15/C19 where the
    /// receiving filesystem folds two spellings this document holds as
    /// distinct.
    AFileThisArrangementFetched,
    /// **A file it did not put there** — any entry at the destination named by
    /// no reckoning entry and no reservation record. It is left alone and
    /// said.
    AFileItDidNotPutThere,
    /// **Smoozy's own zero-byte reservation** — a final path named by a
    /// transfer record standing in *publication pending* (`Server protocol and
    /// transfers` S15), created by the exclusive create of `Desktop app and
    /// delivery` C10 and completed or removed on Smoozy's next run by that
    /// document's C13.
    ///
    /// **It is never announced under the second kind's promise** — announcing
    /// it there would be Smoozy telling the user it left a file behind that
    /// Smoozy itself put there.
    SmoozysOwnZeroByteReservation,
}

/// **R6, as one function over the two records and the path.**
///
/// **Every destination enumeration in this document subtracts the paths the
/// reservation records name before anything is classified**, which is why the
/// reservation is tested first here.
///
/// The `folded_onto` argument is `Desktop app and delivery` C15/C19's
/// stored-name outcome, handed in: where the receiving filesystem folds two
/// spellings this document holds as distinct, the adapter says which
/// reckoning entry's landing path the taken name is. **Nothing here models a
/// folding rule** — that is C11's own reasoning and P6's, and it is the
/// filesystem that is asked.
pub fn what_stands_at(
    landing_path: &[u8],
    the_fetched_landing_paths: &[Vec<u8>],
    the_publication_pending_final_paths: &[Vec<u8>],
    folded_onto: Option<&[u8]>,
) -> WhatStandsAtADestinationName {
    if the_publication_pending_final_paths
        .iter()
        .any(|reserved| reserved == landing_path)
    {
        return WhatStandsAtADestinationName::SmoozysOwnZeroByteReservation;
    }
    let matched = the_fetched_landing_paths
        .iter()
        .any(|fetched| fetched == landing_path)
        || folded_onto.is_some_and(|onto| {
            the_fetched_landing_paths
                .iter()
                .any(|fetched| fetched == onto)
        });
    if matched {
        WhatStandsAtADestinationName::AFileThisArrangementFetched
    } else {
        WhatStandsAtADestinationName::AFileItDidNotPutThere
    }
}

/// **R6's ordering requirement, which is this document's own and is stated
/// here because no cited clause states it.**
///
/// `Desktop app and delivery` C13 has the next run complete or remove the
/// reservation and carries no ordering of any kind relative to admitting
/// transfers; `Server protocol and transfers` S28's second bullet orders its
/// own desktop-side sweep at application start, which is the right shape on
/// different material and a different act. **So it is declared here: no
/// transfer of this document is admitted until every publication-pending
/// record has been completed or its reservation removed.**
///
/// That is what lets P6's third bullet say the reservation cannot be met at
/// fetch time, and it runs beside S28's start sweep rather than inside it.
///
/// **R6a: only a record in the download direction can hold what R6 orders
/// against.** An upload's mark names a file on the machine, and an *interrupted*
/// upload keeps it for its carry-on. Read of every direction, three uploads cut
/// at publication refused every auto-download of a real server for the rest of
/// the run, with nothing said.
pub fn transfers_may_be_admitted(records: &[TransferRecord]) -> bool {
    records
        .iter()
        .filter(|record| record.direction == smoozy_server_protocol::transfers::Direction::Down)
        .all(|record| record.publication_pending.is_none())
}

/// **R7: a file the arrangement fetched and the machine has since re-written
/// is a fourth telling, and it is told rather than acted on.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhatARereadSays {
    /// The four values are as they were recorded. Nothing is said and nothing
    /// is done.
    Unchanged,
    /// **A newer version on the server, not fetched over the local copy.**
    /// The local copy is **not** replaced: *local files are never silently
    /// overwritten* covers a file the user may already be compositing with,
    /// whoever put it there, **and the arrangement never asks the replace
    /// question at all**. The newer version stays on the server and is
    /// fetchable by hand, where the manual question is asked.
    ANewerVersionOnTheServer,
}

/// **R7, as one comparison.**
///
/// Where a walk shows change at a path whose account says *fetched* — a
/// re-run's attempt re-rendering the last frame its reckoning counted as
/// already there is the ordinary case — the four values are re-read and
/// compared with the recorded ones.
///
/// **Presentation rides the same per-request telling surface as *left
/// alone*; wording is Design's.**
pub fn what_a_re_read_says(recorded: &SourceReading, now: &SourceReading) -> WhatARereadSays {
    if recorded.is_the_same_file_as(now) {
        WhatARereadSays::Unchanged
    } else {
        WhatARereadSays::ANewerVersionOnTheServer
    }
}

/// **R7's own bound, held as a value: the telling fetches nothing.**
///
/// It is the one exception to R9's *never fetched again*, and it is an
/// exception that fetches nothing either.
pub const THE_NEWER_VERSION_TELLING_FETCHES_NOTHING_AND_REPLACES_NOTHING: bool = true;

/// **R10: whether the arrangement recognises files the user downloaded by
/// hand from the same job is a product decision and is not settled here** —
/// the spec's own parenthesis.
///
/// Such files carry no reckoning entry and land in [`WhatStandsAtADestinationName::AFileItDidNotPutThere`],
/// warned about as the spec describes. **Nothing in this crate reads the
/// manual request records to widen the first kind**, and a test holds that.
pub const WHETHER_HAND_DOWNLOADED_FILES_ARE_RECOGNISED_IS_NOT_SETTLED_HERE: bool = true;

/// **R3: the arrangement's lifetime is user-held, riding the job's entry.**
///
/// It is removed when the user removes the job's entry, and never by age or by
/// a sweep.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhatEndsIt {
    /// The user removes the job's entry.
    TheEntryIsRemoved,
    /// **A cancelled job's provisional record removal takes the arrangement
    /// with it**, which is the spec's *a cancelled job takes its arrangement
    /// with it*.
    TheJobWasCancelled,
    /// **A removed server ends the arrangement permanently** — the spec's own
    /// sentence — meaning no further fetch is ever arranged; the record itself
    /// stays readable as part of the entry's account until the entry goes.
    TheServerWasRemoved,
}

impl WhatEndsIt {
    /// Whether the record itself goes with the ending, or stays readable.
    pub fn the_record_goes_with_it(self) -> bool {
        !matches!(self, WhatEndsIt::TheServerWasRemoved)
    }
}

/// **R11: what the record survives, and it survives all five by being one
/// durable record in the one local record.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhatItSurvives {
    /// **A drop and a repair** — nothing about the record is
    /// connection-held; the arrangement's in-flight transfers follow
    /// `Server protocol and transfers` C11 step 4 on a replacement.
    ADropAndARepair,
    /// **The app closing and reopening** —
    /// `State, identity and recovery` D6 makes a crash and a clean close
    /// leave the same record; the
    /// arrangement's *activity* is run-held and dies with the process, the
    /// record does not.
    TheAppClosingAndReopening,
    /// **The arrangement switched off and on again.**
    SwitchedOffAndOnAgain,
    /// **A re-run writing into the same folder.**
    ARerunWritingIntoTheSameFolder,
    /// **The reservation record** — `Server protocol and transfers` S9/S15
    /// records live in the same local record and survive the same close, so
    /// the removal owed to Smoozy's *next* run has a record to read. That
    /// record is S9's and is not duplicated here.
    TheReservationRecord,
}

impl WhatItSurvives {
    pub const ALL: [WhatItSurvives; 5] = [
        WhatItSurvives::ADropAndARepair,
        WhatItSurvives::TheAppClosingAndReopening,
        WhatItSurvives::SwitchedOffAndOnAgain,
        WhatItSurvives::ARerunWritingIntoTheSameFolder,
        WhatItSurvives::TheReservationRecord,
    ];
}

/// Which of `Servers`' two connection labels a connection carries.
///
/// **They are read, never re-derived** — that document decides them and this
/// one consumes the answer.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhichConnection {
    /// A repaired connection. `Server protocol and transfers` C11 step 4
    /// re-establishes every transfer whose record says *interrupted within the
    /// current application run*.
    AReplacement,
    /// A connect after a disconnect. **Nothing resumes on it.**
    PeriodOpening,
}

/// **R12: the declaration `Server protocol and transfers` C13 leaves to this
/// document.**
///
/// That C13: *"`File Browser` and `Download` own which of their arrangements
/// survive a run."* **The declaration: an auto-download arrangement whose
/// desired state is on survives every application run and resumes on any
/// connection to its server — replacement or period-opening.**
///
/// The spec states it outright — *"auto-download resumes on the next
/// connection — because it is an arrangement attached to a job, not a request
/// made at a moment"*, Creator-stated — and the spec has already answered the
/// explicit-connect objection: the arrangement is work the user already asked
/// for, and the connection it resumes on is one the user opened.
///
/// **What resumes is the arrangement — its walks and its fresh fetches.** Its
/// individual transfers follow the ordinary rules unchanged, which is
/// [`a_transfer_of_an_earlier_run_resumes_automatically`].
pub fn the_arrangement_resumes_on(desired_state: DesiredState, _which: WhichConnection) -> bool {
    matches!(desired_state, DesiredState::On)
}

/// **R12's second half, and it is the opposite answer to the first.**
///
/// An interrupted transfer of the current run resumes on a replacement under
/// `Server protocol and transfers` C11 step 4; **a record written by an
/// earlier run is never resumed automatically**, and is taken up only as M13
/// and M14 declare.
pub fn a_transfer_of_an_earlier_run_resumes_automatically(
    record: &TransferRecord,
    this_run: &Identity,
    which: WhichConnection,
) -> bool {
    matches!(which, WhichConnection::AReplacement) && &record.written_by_run == this_run
}

/// **R12's last sentence: a manual request has no such declaration and ends
/// with the run.**
pub const A_MANUAL_REQUEST_HAS_NO_SURVIVING_ARRANGEMENT: bool = true;

/// **The durable half — the shape is this document's and the store is
/// `Desktop app and delivery` §5's.**
///
/// **Nothing is stubbed for it and no second store is made here.** The seam is
/// this trait; the real end is that crate's, because `State, identity and
/// recovery` §1.1 puts every durable store in the one local record and the
/// arrangement is already in its inventory by name.
///
/// **The real end is `smoozy_desktop::record::download_records`**, written at
/// step 10c piece 10 at record version 26; before it no implementation existed
/// outside this crate's own tests ([`crate::arriving_later`]).
pub trait KeepsTheArrangement {
    /// **R1's write.** Every later change — the desired state, the narrowing,
    /// the destination at a switch-on, an entry's account, an entry's
    /// four-value reading — goes down through the same call, at the moment it
    /// becomes true.
    fn write_it_down(&mut self, arrangement: &AnArrangement) -> Result<(), Refused>;

    /// The arrangement held against a job's entry, where there is one.
    fn the_arrangement_for(&self, job: &Identity) -> Result<Option<AnArrangement>, Refused>;

    /// **`State, identity and recovery` §11.1: user-held.** R3's three
    /// endings are the caller's; this removes the record where the ending
    /// takes it.
    fn it_ended(&mut self, job: &Identity, how: WhatEndsIt) -> Result<(), Refused>;
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use smoozy_state::identities::IdentityKind;
    use smoozy_state::observed::Clock;
    use std::time::{Duration, SystemTime};

    pub(crate) fn an_identity(kind: IdentityKind) -> Identity {
        Identity::issue(kind).expect("the desk has randomness")
    }

    pub(crate) fn four_values(
        size: u64,
        inode: &str,
        modified: i64,
        changed: i64,
    ) -> SourceReading {
        SourceReading {
            size,
            inode: inode.to_string(),
            modification_time: modified,
            change_time: changed,
        }
    }

    pub(crate) fn an_arrangement(destination: &[u8]) -> AnArrangement {
        AnArrangement::switched_on(
            an_identity(IdentityKind::Job),
            an_identity(IdentityKind::LocalServer),
            destination.to_vec(),
            TheNarrowing::Everything,
        )
    }

    fn fetched(arrangement: &mut AnArrangement, path: &[u8], frame: Option<i64>) {
        let destination = arrangement.destination.clone();
        arrangement.reckoning.push(AReckoningEntry::acted_on(
            path.to_vec(),
            frame,
            WhatSmoozyDid::Fetched,
            destination,
        ));
    }

    #[test]
    fn the_fetch_decision_reads_the_reckoning_and_never_a_destination_listing() {
        let mut arrangement = an_arrangement(b"/Users/a/Downloads");
        fetched(&mut arrangement, b"frame_0001.png", Some(1));
        // R8 and R9: a path the reckoning has taken is not owed, and a path it
        // does not name at all is.
        assert!(!arrangement.a_fetch_is_owed_for(b"frame_0001.png"));
        assert!(arrangement.a_fetch_is_owed_for(b"frame_0002.png"));
    }

    #[test]
    fn the_key_is_the_predicted_path_and_the_attempt_is_not_in_it() {
        let mut arrangement = an_arrangement(b"/Users/a/Downloads");
        fetched(&mut arrangement, b"frame_0001.png", Some(1));
        // A second attempt re-rendering the same name meets the same entry:
        // there is no second key for it to land under.
        assert_eq!(arrangement.reckoning.len(), 1);
        assert!(arrangement.entry_for(b"frame_0001.png").is_some());
    }

    #[test]
    fn only_two_of_the_five_terminal_states_re_open_and_each_by_its_own_event() {
        use WhatHappened::*;
        use WhatSmoozyDid::*;
        for final_state in [Fetched, LeftAlone, CannotLand] {
            for happened in [TheNarrowingWidened, ARenderStartNamingThisEntrysFrame] {
                assert_eq!(final_state.after(happened), final_state);
            }
        }
        assert_eq!(SkippedByNarrowing.after(TheNarrowingWidened), Owed);
        assert_eq!(
            SkippedByNarrowing.after(ARenderStartNamingThisEntrysFrame),
            SkippedByNarrowing
        );
        assert_eq!(NeverComing.after(ARenderStartNamingThisEntrysFrame), Owed);
        assert_eq!(NeverComing.after(TheNarrowingWidened), NeverComing);
    }

    #[test]
    fn a_render_start_re_opens_never_coming_only_for_the_frames_the_attempts_list_names() {
        let mut arrangement = an_arrangement(b"/Users/a/Downloads");
        let destination = arrangement.destination.clone();
        for frame in [1, 2, 3] {
            arrangement.reckoning.push(AReckoningEntry::acted_on(
                format!("frame_{frame:04}.png").into_bytes(),
                Some(frame),
                WhatSmoozyDid::NeverComing,
                destination.clone(),
            ));
        }
        arrangement.a_render_start_re_arms_it(&[2, 3]);
        assert_eq!(
            arrangement
                .entry_for(b"frame_0001.png")
                .unwrap()
                .account
                .state,
            WhatSmoozyDid::NeverComing
        );
        assert_eq!(
            arrangement
                .entry_for(b"frame_0002.png")
                .unwrap()
                .account
                .state,
            WhatSmoozyDid::Owed
        );
        assert_eq!(
            arrangement
                .entry_for(b"frame_0003.png")
                .unwrap()
                .account
                .state,
            WhatSmoozyDid::Owed
        );
    }

    #[test]
    fn a_re_run_never_arrives_over_a_file_the_user_already_has() {
        // R9 read against R11's fourth survival: the render-start re-arms
        // *never coming* and leaves *fetched* exactly where it was.
        let mut arrangement = an_arrangement(b"/Users/a/Downloads");
        fetched(&mut arrangement, b"frame_0001.png", Some(1));
        arrangement.a_render_start_re_arms_it(&[1, 2, 3]);
        assert_eq!(
            arrangement
                .entry_for(b"frame_0001.png")
                .unwrap()
                .account
                .state,
            WhatSmoozyDid::Fetched
        );
        assert!(!arrangement.a_fetch_is_owed_for(b"frame_0001.png"));
    }

    #[test]
    fn a_switch_on_that_moves_the_destination_says_so_before_it_proceeds() {
        let mut arrangement = an_arrangement(b"/Users/a/Downloads");
        fetched(&mut arrangement, b"frame_0001.png", Some(1));
        arrangement.switched_off();
        let said = arrangement
            .switched_on_again(b"/Volumes/work".to_vec(), TheNarrowing::Everything)
            .expect("something is already fetched and the folder moved");
        assert_eq!(
            said.what_is_already_downloaded_stays_at,
            b"/Users/a/Downloads"
        );
        assert_eq!(said.what_has_not_arrived_comes_to, b"/Volumes/work");
        // R1's fifth bullet: the fetched entry keeps the folder its own act
        // was taken in, which is what makes both places nameable afterwards.
        assert_eq!(
            arrangement.the_folders_it_has_fetched_into(),
            vec![b"/Users/a/Downloads".to_vec()]
        );
    }

    #[test]
    fn a_switch_on_with_nothing_fetched_or_with_the_same_folder_says_nothing() {
        let mut arrangement = an_arrangement(b"/Users/a/Downloads");
        assert!(arrangement
            .switched_on_again(b"/Volumes/work".to_vec(), TheNarrowing::Everything)
            .is_none());
        fetched(&mut arrangement, b"frame_0001.png", Some(1));
        assert!(arrangement
            .switched_on_again(b"/Volumes/work".to_vec(), TheNarrowing::Everything)
            .is_none());
    }

    #[test]
    fn a_switch_on_again_resets_the_narrowing_and_re_opens_what_it_had_put_out() {
        let mut arrangement = AnArrangement::switched_on(
            an_identity(IdentityKind::Job),
            an_identity(IdentityKind::LocalServer),
            b"/Users/a/Downloads".to_vec(),
            TheNarrowing::TheseFormats(vec![b"PNG".to_vec()]),
        );
        arrangement.reckoning.push(AReckoningEntry::acted_on(
            b"frame_0001.exr".to_vec(),
            Some(1),
            WhatSmoozyDid::SkippedByNarrowing,
            b"/Users/a/Downloads".to_vec(),
        ));
        arrangement.switched_off();
        arrangement.switched_on_again(b"/Users/a/Downloads".to_vec(), TheNarrowing::Everything);
        assert_eq!(
            arrangement
                .entry_for(b"frame_0001.exr")
                .unwrap()
                .account
                .state,
            WhatSmoozyDid::Owed
        );
        assert!(arrangement.narrowing.is_no_filter());
    }

    #[test]
    fn switching_off_removes_nothing_and_leaves_the_reckoning_where_it_was() {
        let mut arrangement = an_arrangement(b"/Users/a/Downloads");
        fetched(&mut arrangement, b"frame_0001.png", Some(1));
        arrangement.switched_off();
        assert_eq!(arrangement.desired_state, DesiredState::Off);
        assert_eq!(arrangement.reckoning.len(), 1);
    }

    #[test]
    fn a_reservation_is_never_announced_as_a_file_smoozy_did_not_put_there() {
        let reserved = vec![b"/Users/a/Downloads/frame_0009.png".to_vec()];
        assert_eq!(
            what_stands_at(b"/Users/a/Downloads/frame_0009.png", &[], &reserved, None),
            WhatStandsAtADestinationName::SmoozysOwnZeroByteReservation
        );
    }

    #[test]
    fn a_folded_spelling_reaches_the_fetched_entry_it_folds_onto() {
        let fetched_paths = vec![b"/Users/a/Downloads/Frame.png".to_vec()];
        assert_eq!(
            what_stands_at(
                b"/Users/a/Downloads/frame.png",
                &fetched_paths,
                &[],
                Some(b"/Users/a/Downloads/Frame.png")
            ),
            WhatStandsAtADestinationName::AFileThisArrangementFetched
        );
        // Without the adapter's stored-name outcome the same path is the
        // world's, which is the honest answer where nothing folded.
        assert_eq!(
            what_stands_at(b"/Users/a/Downloads/frame.png", &fetched_paths, &[], None),
            WhatStandsAtADestinationName::AFileItDidNotPutThere
        );
    }

    #[test]
    fn no_transfer_is_admitted_while_a_publication_pending_record_stands() {
        let run = an_identity(IdentityKind::Run);
        let mut record = TransferRecord::arranged(
            an_identity(IdentityKind::Transfer),
            smoozy_server_protocol::transfers::Direction::Down,
            an_identity(IdentityKind::LocalServer),
            None,
            "/smoozy/jobs/j/outputs/frame_0001.png".to_string(),
            "/Users/a/Downloads/frame_0001.png".to_string(),
            "/Users/a/Downloads/.smoozy-staging",
            1024,
            run,
        );
        assert!(transfers_may_be_admitted(std::slice::from_ref(&record)));
        record.publication_pending = Some(
            smoozy_server_protocol::transfers::publication::PublicationPreState {
                intent: smoozy_server_protocol::transfers::publication::Intent::CreateIntended,
                staging_path: record.staging_path.as_text(),
                final_path: "/Users/a/Downloads/frame_0001.png".to_string(),
                staged_identity:
                    smoozy_server_protocol::transfers::StableFileIdentity::TheSupplierCouldNotAnswer,
                staged_byte_length: 1024,
                staged_digest: smoozy_server_protocol::transfers::integrity::Digest::of(b"x"),
            },
        );
        assert!(!transfers_may_be_admitted(std::slice::from_ref(&record)));
    }

    /// **R6a.** What a real pass's record held: an upload cut at publication,
    /// *interrupted*, its mark kept for the carry-on, and every auto-download
    /// of that server refused after it.
    #[test]
    fn an_uploads_publication_never_holds_a_download_back() {
        let mut record = TransferRecord::arranged(
            an_identity(IdentityKind::Transfer),
            smoozy_server_protocol::transfers::Direction::Up,
            an_identity(IdentityKind::LocalServer),
            None,
            "/Users/a/shot/assets/fx/tendril.blend".to_string(),
            "/smoozy/assets/files/Users/a/shot/assets/fx/tendril.blend".to_string(),
            "/smoozy/assets/files/Users/a/shot/assets/fx/.smoozy-staging",
            1024,
            an_identity(IdentityKind::Run),
        );
        record.ending = Some(smoozy_server_protocol::transfers::Ending::Interrupted);
        record.publication_pending = Some(
            smoozy_server_protocol::transfers::publication::PublicationPreState {
                intent: smoozy_server_protocol::transfers::publication::Intent::CreateIntended,
                staging_path: record.staging_path.as_text(),
                final_path: "/smoozy/assets/files/Users/a/shot/assets/fx/tendril.blend".to_string(),
                staged_identity:
                    smoozy_server_protocol::transfers::StableFileIdentity::TheSupplierCouldNotAnswer,
                staged_byte_length: 1024,
                staged_digest: smoozy_server_protocol::transfers::integrity::Digest::of(b"x"),
            },
        );
        assert!(transfers_may_be_admitted(std::slice::from_ref(&record)));
    }

    #[test]
    fn the_newer_version_telling_fires_on_any_of_the_four_values_moving() {
        let recorded = four_values(1024, "12", 100, 100);
        assert_eq!(
            what_a_re_read_says(&recorded, &four_values(1024, "12", 100, 100)),
            WhatARereadSays::Unchanged
        );
        for moved in [
            four_values(2048, "12", 100, 100),
            four_values(1024, "13", 100, 100),
            four_values(1024, "12", 200, 100),
            four_values(1024, "12", 100, 200),
        ] {
            assert_eq!(
                what_a_re_read_says(&recorded, &moved),
                WhatARereadSays::ANewerVersionOnTheServer
            );
        }
    }

    #[test]
    fn the_machine_derived_half_is_disqualified_where_the_account_half_stands() {
        let identity = TheServerFileIdentity {
            reading: Reading::taken(
                four_values(1024, "12", 100, 100),
                SystemTime::UNIX_EPOCH + Duration::from_secs(1_700_000_000),
                Clock::ThisComputer,
            ),
            read_under: Some(an_identity(IdentityKind::Installation)),
        };
        assert!(identity.may_be_compared(InstallationCheck::PresentAndEqualConfirmed));
        assert!(!identity.may_be_compared(InstallationCheck::PresentAndDifferent));
        assert!(!identity.may_be_compared(InstallationCheck::Absent));
        // Unconfirmed is neither: the value stands as it is and is not
        // compared, which is that document's own third answer.
        assert!(!identity.may_be_compared(InstallationCheck::PresentAndTheSupervisorDidNotAnswer));
    }

    #[test]
    fn the_arrangement_resumes_on_either_label_and_an_earlier_runs_transfer_never_does() {
        assert!(the_arrangement_resumes_on(
            DesiredState::On,
            WhichConnection::PeriodOpening
        ));
        assert!(the_arrangement_resumes_on(
            DesiredState::On,
            WhichConnection::AReplacement
        ));
        assert!(!the_arrangement_resumes_on(
            DesiredState::Off,
            WhichConnection::AReplacement
        ));

        let this_run = an_identity(IdentityKind::Run);
        let earlier = an_identity(IdentityKind::Run);
        let record = TransferRecord::arranged(
            an_identity(IdentityKind::Transfer),
            smoozy_server_protocol::transfers::Direction::Down,
            an_identity(IdentityKind::LocalServer),
            None,
            "/smoozy/jobs/j/outputs/frame_0001.png".to_string(),
            "/Users/a/Downloads/frame_0001.png".to_string(),
            "/Users/a/Downloads/.smoozy-staging",
            1024,
            earlier,
        );
        assert!(!a_transfer_of_an_earlier_run_resumes_automatically(
            &record,
            &this_run,
            WhichConnection::AReplacement
        ));
        let mine = TransferRecord {
            written_by_run: this_run.clone(),
            ..record
        };
        assert!(a_transfer_of_an_earlier_run_resumes_automatically(
            &mine,
            &this_run,
            WhichConnection::AReplacement
        ));
        // A period-opening connection resumes nothing, whoever wrote it.
        assert!(!a_transfer_of_an_earlier_run_resumes_automatically(
            &mine,
            &this_run,
            WhichConnection::PeriodOpening
        ));
    }

    #[test]
    fn a_removed_server_leaves_the_record_readable_and_the_other_two_take_it() {
        assert!(!WhatEndsIt::TheServerWasRemoved.the_record_goes_with_it());
        assert!(WhatEndsIt::TheEntryIsRemoved.the_record_goes_with_it());
        assert!(WhatEndsIt::TheJobWasCancelled.the_record_goes_with_it());
    }
}
