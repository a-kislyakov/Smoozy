//! The upload request — `File Browser` §3, U1 to U15, Rows 190, 197, 486 and
//! 511, with Row 428 at §3.7.
//!
//! **Constraint:** Row 190 — *"The unit is the file, the request is still one
//! thing"* — with the collision answer that *"still stand[s]"* across a drop and
//! a resume, the waiting state, the per-file breakdown under a whole, and
//! *"Closing Smoozy stops uploads and ends the request with them."*
//!
//! **The durable half is one `State, identity and recovery` §9.4 outcome record
//! and this crate holds no store of its own.** The shape is here, the store is
//! `Desktop app and delivery` §5's, which is the split that document already
//! draws for the outcome record it keeps for every module. **The table for the
//! fields U1 adds is that document's local record at version 9** — three tables
//! beside the shared two, keyed to §9.4's own enumeration by position — **and
//! its reader and writer are built there too**, so a request survives a run.
//! See [`KeepsTheRequest`].
//!
//! **Markers used below that belong to another document, each with the document
//! it belongs to.** `Server protocol and transfers` S4, S8, S9, S10a, S12, S13,
//! S15a, S26, S27, S28, C11, C23a, G4, G4a, G6, T3, T8. `State, identity and
//! recovery` §8.6, §9.2, §9.4, §11.1, D5, D6, F19. `Dependency Upload & Remap`
//! A9, A15, A16, A17, A18, Q8. `Servers` S10.
//! `Desktop app and delivery` §5, A2.
//!
//! **A marker this plan also defines is deliberately not listed above,
//! whichever document is meant, and is written out with its document at every
//! site instead** — the header list cannot tell the two apart, and a bare one
//! would resolve to a real and wrong clause.

use std::time::SystemTime;

use smoozy_server_protocol::transfers::publication::Intent;
use smoozy_server_protocol::transfers::{Ending, Report, StartOverCause, TransferRecord};
use smoozy_state::identities::Identity;
use smoozy_state::outcomes::{Refused, UnitOutcome};

use crate::already_exists::{paths_are_equal, AComparison, CollisionAnswer, Comparison};
use crate::diary::{HowARequestEnded, TheFourCounts};
use crate::uploading::TheNameCannotBeCarried;

/// **U1's enumeration entry — one per file, and every field is U1's own.**
///
/// The entry is what makes the request *one thing* while the unit stays the
/// file: the whole of what a resume, a fresh sitting, a total and a report read
/// is here, and nothing is re-derived from the machine.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EnumerationEntry {
    /// **The file's source path on the user's own computer, as stored on
    /// disk.** N2's rule for the reading, and `Dependency Upload & Remap` K3's
    /// holding-file key at the hand-over (U15).
    pub source_path: Vec<u8>,
    /// Its landing path on the machine.
    pub landing_path: Vec<u8>,
    /// Its size as enumerated. **Never re-read to keep a total fresh** — U11's
    /// totals are composed from this and from G4's per-transfer counts.
    pub size_as_enumerated: u64,
    /// **Its disposition after the collision answer.** Absent until U4's answer
    /// is applied, which happens once (U4).
    pub disposition: Option<Disposition>,
    /// **Its transfer identity once a transfer is arranged.** Absent for a unit
    /// no transfer was arranged for — a skipped one, and a handed one (U14).
    pub transfer: Option<Identity>,
    /// **Its unit outcome as it becomes true**, and `State, identity and
    /// recovery` §8.6 makes it immutable once written.
    pub outcome: Option<UnitOutcome>,
    /// **U9a: the moment that outcome was written, and the only moment any
    /// upload record holds.**
    ///
    /// Taken from the desktop's clock at the durable write `State, identity and
    /// recovery` §9.4 item 2 already requires — an existing write at an existing
    /// moment, no second occasion and no sweep. **It is a desktop-clock value
    /// used for presentation and for nothing a promise rests on** (D5), and it
    /// is never subtracted from a machine reading.
    pub outcome_written_at: Option<SystemTime>,
}

impl EnumerationEntry {
    /// An entry as the walk's enumeration gives it, before any answer.
    pub fn enumerated(
        source_path: Vec<u8>,
        landing_path: Vec<u8>,
        size_as_enumerated: u64,
    ) -> EnumerationEntry {
        EnumerationEntry {
            source_path,
            landing_path,
            size_as_enumerated,
            disposition: None,
            transfer: None,
            outcome: None,
            outcome_written_at: None,
        }
    }

    /// **§8.6 F19 kept here as well as at the store**: a unit outcome is
    /// immutable, so a second write is refused rather than taken.
    ///
    /// The moment rides with the outcome and is written in the same act (U9a).
    pub fn its_outcome_became(
        &mut self,
        outcome: UnitOutcome,
        at: SystemTime,
    ) -> Result<(), Refused> {
        if self.outcome.is_some() {
            return Err(Refused::ThatUnitHasAlreadyEnded);
        }
        self.outcome = Some(outcome);
        self.outcome_written_at = Some(at);
        Ok(())
    }
}

/// **U1's three dispositions, and U12's fourth.**
///
/// *"Accepting gives each of them a **fourth disposition — handed over** —
/// beside U1's three."*
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Disposition {
    ToSend,
    SkipExisting,
    /// **X9's generated name**, computed once when the answer is applied so a
    /// resume re-derives nothing. The value is the fresh landing path.
    RenamedUnderKeepBoth(Vec<u8>),
    /// **U12's fourth.** The act `Dependency Upload & Remap` opens for the unit
    /// is named by its operation identity, which its A16 returns as the act
    /// opens.
    HandedOver {
        the_act: Identity,
    },
}

impl Disposition {
    /// **U11 and U14: which units are in the whole's two totals.** The count is
    /// unit outcomes over the units whose disposition is to send, and the bar is
    /// bytes over those same units.
    pub fn is_in_the_totals(&self) -> bool {
        self.a_transfer_is_arranged()
    }

    /// Whether this request arranges a transfer for the unit at all.
    ///
    /// **The two questions have the same answer and that is not a coincidence**:
    /// U11's denominator is the bytes this request will move, so a unit it
    /// arranges nothing for belongs in neither total.
    pub fn a_transfer_is_arranged(&self) -> bool {
        matches!(
            self,
            Disposition::ToSend | Disposition::RenamedUnderKeepBoth(_)
        )
    }
}

/// **U1's telling, where an upload was pushed past the live-job warning.**
///
/// *"They are fields of **this** record and never of a second one: §9.4 gives
/// one outcome record per action, the disturbing upload is one action, and a
/// second record beside this one would be two outcome records for one action,
/// which that section forbids in its own words. A request that disturbed nothing
/// carries neither field."*
///
/// **The act intended is the upload request itself**, which is why this holds
/// the jobs alone: W8's record for an upload *is* U1's, so naming the act again
/// inside it would name the record from inside the record.
///
/// **What fills it is §8's live-job warning, which is built at
/// [`crate::warnings`]** — W8's telling is
/// [`crate::warnings::TheTellingThatStandsFirst`], composed by
/// [`crate::warnings::the_telling_for`]. *(Corrected 2026-09-07: this said §8
/// was step 7's and pointed at [`crate::arriving_later`]. Step 7 landed it, and
/// that page has held one clause of §1 since 2026-09-06, which is not this
/// one.)* **What still does not happen is the joining**: the only value this
/// field is ever given is `None`, because nothing composes an upload request
/// out of a telling that stood first — the one constructor sets it to `None`. A
/// request that disturbed nothing carries `None` too — **an absence written as
/// one, never a field left out.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TheTelling {
    /// The affected jobs by identity (W8).
    pub the_jobs: Vec<Identity>,
}

/// **U1: an upload request is one multi-part action, and its durable half is one
/// `State, identity and recovery` §9.4 outcome record in the local record.**
///
/// Written when the user confirms the upload, **before any transfer is
/// arranged**.
///
/// **The record is user-held** (`State, identity and recovery` §11.1): it is the
/// account of the upload and it goes when the user removes it, never by age.
/// **It is also Row 187's record** — [`crate::uploading`] reads these same
/// entries rather than keeping a second copy.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UploadRequest {
    /// The §9.4 record's own operation identity.
    pub record: Identity,
    /// The server row the request was arranged under (T3's binding, N1).
    pub server_row: Identity,
    /// The installation identity it was arranged under.
    pub installation: Option<Identity>,
    /// **The destination folder — where the user was browsing.**
    pub destination: Vec<u8>,
    /// **The enumeration: one entry per file, complete before the already-exists
    /// question is asked and durable before the first byte moves** (`Server
    /// protocol and transfers` S8's ordering, `State, identity and recovery`
    /// §9.4's rule).
    pub entries: Vec<EnumerationEntry>,
    /// **U4's answer**, absent until the user gives it.
    pub collision_answer: Option<CollisionAnswer>,
    /// **U1's telling**, absent for a request that disturbed nothing.
    pub telling: Option<TheTelling>,
    /// **C11 step 4's marker, carried on the request as it is on a transfer
    /// record**: which application run opened it. U8 turns on it — *"the record
    /// carries which run wrote it, so the automatic case and the taken-up case
    /// are separable."*
    pub opened_by_run: Identity,
}

/// The upload boundary was opened.
#[derive(Debug)]
pub struct SendingFilesUpWasOpen(());

/// Open the upload without consulting paid access.
pub fn open_an_upload_request() -> Result<SendingFilesUpWasOpen, std::convert::Infallible> {
    Ok(SendingFilesUpWasOpen(()))
}

impl UploadRequest {
    /// **U1's write**, as a value. Nothing here writes it down; the ordering is
    /// [`KeepsTheRequest`]'s caller's, exactly as `Server protocol and
    /// transfers` S8's is.
    ///
    /// Takes the witness returned by [`open_an_upload_request`].
    pub fn confirmed(
        _sending_files_up_was_open: SendingFilesUpWasOpen,
        record: Identity,
        server_row: Identity,
        installation: Option<Identity>,
        destination: Vec<u8>,
        entries: Vec<EnumerationEntry>,
        opened_by_run: Identity,
    ) -> UploadRequest {
        UploadRequest {
            record,
            server_row,
            installation,
            destination,
            entries,
            collision_answer: None,
            telling: None,
            opened_by_run,
        }
    }

    /// The entry landing at a path, under X5's byte-for-byte rule.
    pub fn entry_landing_at(&self, landing_path: &[u8]) -> Option<&EnumerationEntry> {
        self.entries
            .iter()
            .find(|entry| paths_are_equal(&entry.landing_path, landing_path))
    }

    /// **U1's third field group: the request's own ending — completed, stopped
    /// by the user, or ended by the run ending — derived from the unit outcomes
    /// and the run, never a second stored field.**
    ///
    /// `None` while the request is still alive: every unit has not ended and the
    /// run that opened it is the run asking.
    pub fn how_it_ended(
        &self,
        this_run: &Identity,
        the_user_stopped_it: bool,
    ) -> Option<HowARequestEnded> {
        if self.ended_by_the_run_ending(this_run) {
            // U7: the activity ended with the application run. **The third
            // ending has no line and no moment** — D6 writes nothing on exit.
            return None;
        }
        if the_user_stopped_it {
            return Some(HowARequestEnded::StoppedByTheUser);
        }
        if !self.every_unit_ended() {
            return None;
        }
        Some(HowARequestEnded::Completed)
    }

    /// **Whether every unit of the request has ended**, read off the unit
    /// outcomes alone — U1's *derived from the unit outcomes*. Nothing here
    /// asks which run is reading.
    ///
    /// **The answer has to have been applied**: a request whose entries carry
    /// no disposition has not been asked yet, and reading that as ended would
    /// call an unstarted upload finished. **Every unit this request arranges a
    /// transfer for has to have an outcome**; a skipped unit and a handed one
    /// arrange none, so neither holds the request open.
    pub fn every_unit_ended(&self) -> bool {
        if self.entries.is_empty() {
            return false;
        }
        if self.entries.iter().any(|entry| entry.disposition.is_none()) {
            return false;
        }
        self.entries.iter().all(|entry| {
            entry.outcome.is_some()
                || !entry
                    .disposition
                    .as_ref()
                    .is_some_and(Disposition::a_transfer_is_arranged)
        })
    }

    /// **U1's third ending, read off the run marker alone.**
    ///
    /// A record whose run is not this run is a record no activity survives for
    /// (U7), and that is exactly *ended by the run ending*. It is separate from
    /// [`Self::how_it_ended`] because that answer is the two endings a line is
    /// written for, and this one writes none.
    pub fn ended_by_the_run_ending(&self, this_run: &Identity) -> bool {
        &self.opened_by_run != this_run
    }

    /// **§9.4's enumeration of units, in the order they run**, which is the
    /// order of U1's own entries.
    ///
    /// **Every unit carries the same role**, so this is the list a keeper writes
    /// and never a list of names: what tells two units apart is the position,
    /// which is §9.4's own ordering and which U1's entry at that position
    /// carries the source path, the landing path and the disposition for.
    pub fn the_units_by_role(&self) -> Vec<&'static str> {
        let role = the_role_every_unit_carries();
        self.entries.iter().map(|_| role).collect()
    }
}

/// This request's own action, in §11's words — the row the role is read at.
pub const THE_ACTION: &str = "an upload request";

/// **What a unit of an upload request is called in `State, identity and
/// recovery` §9.4's outcome record — §11's own words, and it is a role rather
/// than a value.**
///
/// §9.4 names each unit **by role, in the owning module's own words**, and §11
/// declares the partition unit of an upload request as *one file's transfer*.
/// **The role is those words**, the same for every unit of one request; §9.4's
/// own ordering is what tells two units apart.
///
/// **It is read off §11's own table and never spelled again here**, so the words
/// have one home ([`crate::declarations::THE_PARTITION_UNITS`]).
///
/// **It carries no path, no machine location and no file name.** That is this
/// document's own reason rather than `Keys`' secrecy one: U1's enumeration entry
/// at the same position already holds the source path and the landing path, and
/// a path in the role would give one fact two homes in a table every module
/// shares.
///
/// **These are not the words the user reads.** Presenting an outcome record is
/// `Desktop app and delivery`'s and Rule 9 binds it there.
pub fn the_role_every_unit_carries() -> &'static str {
    crate::declarations::the_role_a_unit_of_carries(THE_ACTION)
        .expect("§11 declares an upload request, and a test in `declarations` holds every row")
}

/// **The durable half — the shape is this document's and the store is `Desktop
/// app and delivery` §5's.**
///
/// **The §9.4 half of this record is `State, identity and recovery` §9.4's and
/// is not re-decided here**: the enumeration is durable before the first unit is
/// attempted, each outcome is durable at the moment it becomes true, and an
/// outcome once written is immutable. The fields U1 adds — the destination, the
/// per-entry source and landing paths, the disposition, the transfer identity,
/// U9a's moment, U4's answer and the telling — are this document's own fields on
/// that same record.
///
/// **The table for them is `Desktop app and delivery`'s local record at version
/// 9** — three tables of this module's own beside the shared `outcome_record`
/// and `outcome_record_unit`, which hold §9.4's half and nothing of U1's. They
/// are **keyed to §9.4's own enumeration by position**, so there is one
/// enumeration joined rather than two copied, and the unit outcomes stay where
/// §9.4 puts them.
///
/// **What every unit of this request is called in that record is
/// [`the_role_every_unit_carries`]**, which is §11's own words. §9.4 names a
/// unit by role in the owning module's own words, and until §11 said what the
/// role was no keeper could write a row at all.
///
/// **Nothing is stubbed for it and no second store is made here.** The seam is
/// this trait and the store's real end is that crate's, because
/// `Desktop app and delivery` A2 puts every durable store in the one local
/// record and `State, identity and recovery` §1.1 says the same from the other
/// side. **The real end is built** — `smoozy_desktop::record::upload_requests`,
/// over the five tables, joined by position — **so a request survives the
/// application run, which is what §3 promises across a drop, the app closing and
/// a fresh sitting.** *(This said the reader and the writer were still owed;
/// they landed 2026-09-04.)* **A test keeper over a list stays beside it** and is
/// what makes §3 testable in this crate, which holds no engine.
pub trait KeepsTheRequest {
    /// **U1's write, and `Server protocol and transfers` S8's ordering applies
    /// to it**: the enumeration is durable before the first byte moves. Every
    /// later change to the request — U4's answer, a transfer identity, a unit
    /// outcome — goes down through the same call, at the moment it becomes true.
    fn write_it_down(&mut self, request: &UploadRequest) -> Result<(), Refused>;

    /// Every request record held for a server row. **Read at a fresh sitting**
    /// (U8) and to compose U9b's reason line.
    fn every_request_for(&self, server_row: &Identity) -> Result<Vec<UploadRequest>, Refused>;

    /// **`State, identity and recovery` §11.1: user-held.** It ends when the
    /// user removes it and never otherwise — no age, no cap, no sweep.
    ///
    /// **N8's cost is named at the removal and not here**: removing a request
    /// record removes the provenance of its files with it.
    fn the_user_removed_it(&mut self, record: &Identity) -> Result<(), Refused>;
}

/// **U2: the partition unit is one file's transfer** — partial across files,
/// atomic within one.
///
/// *"Three hundred textures are three hundred transfers, each with its own
/// transfer identity, each admitted separately, each with its own `Server
/// protocol and transfers` S9 record; the request is the enumeration over
/// them."*
pub const THE_PARTITION_UNIT_IS_ONE_FILES_TRANSFER: bool = true;

/// **U3: every transfer the request arranges is `Server protocol and transfers`
/// §7's, unmodified** — bound to the server row (T3), staged under S4's path,
/// recorded under S9 before its first byte, ended under S26.
///
/// **This document adds the layer above** — which transfers exist, in what order
/// they are offered to admission ([`crate::pool`]), and what the whole means
/// ([`TheWhole`]). **No second transfer, staging or resume shape is invented
/// here**, which is why this function returns that document's own record type
/// rather than one of ours.
///
/// `Ok(None)` where the unit's disposition arranges no transfer — a skipped unit
/// and a handed one.
///
/// **`Err` where the unit's name cannot be carried across the transfer seam**,
/// which is N2's rule met at the one place that cannot hold the bytes. That
/// document's transfer record holds the source path and the final path **as
/// text**; this enumeration holds them as bytes, correctly, and **nothing here
/// converts one into the other by replacing the bytes it cannot read.** A
/// lossy conversion would leave the record holding a different path from the one
/// the already-exists walk compared and from the one the enumeration holds,
/// which is exactly the rewriting N2 forbids — *"nothing rewrites, normalises or
/// re-encodes a name in flight"* — and which L1a forbids on the machine side and
/// X9's byte-trim argument rests on.
///
/// **The refusal is [`crate::uploading::check_the_name_can_be_carried`]'s own
/// value**, so the pre-flight and this function answer the same per-file
/// failure, and **no route arranges a transfer for such a unit.**
pub fn arrange_a_transfer(
    request: &UploadRequest,
    entry: &EnumerationEntry,
    transfer: Identity,
    staging_directory: &str,
    written_by_run: Identity,
) -> Result<Option<TransferRecord>, TheNameCannotBeCarried> {
    let Some(disposition) = entry.disposition.as_ref() else {
        return Ok(None);
    };
    if !disposition.a_transfer_is_arranged() {
        return Ok(None);
    }
    // **A unit that has already ended is never sent.** `State, identity and
    // recovery` §8.6 F19 makes a unit outcome immutable, so a unit with one
    // written has ended — the Keep-both name that could not be composed is the
    // case this exists for, and it is what makes that failure's *never sent*
    // structural rather than a caller's to remember.
    if entry.outcome.is_some() {
        return Ok(None);
    }
    let landing = match disposition {
        Disposition::RenamedUnderKeepBoth(fresh) => fresh.clone(),
        _ => entry.landing_path.clone(),
    };
    let source_path = String::from_utf8(entry.source_path.clone()).map_err(|not_text| {
        TheNameCannotBeCarried::TheSourcePathIsNotValidText {
            source_path: not_text.into_bytes(),
        }
    })?;
    let final_path = String::from_utf8(landing).map_err(|not_text| {
        TheNameCannotBeCarried::TheLandingPathIsNotValidText {
            landing_path: not_text.into_bytes(),
        }
    })?;
    Ok(Some(TransferRecord::arranged(
        transfer,
        smoozy_server_protocol::transfers::Direction::Up,
        request.server_row.clone(),
        request.installation.clone(),
        source_path,
        final_path,
        staging_directory,
        entry.size_as_enumerated,
        written_by_run,
    )))
}

/// **U4: the answer is written durably into the request record at the moment the
/// user gives it, before any transfer whose arrangement depends on it is
/// arranged.**
///
/// *"It is given once for the whole request and is never asked again for the
/// life of the request: a drop, a repair an hour later, and every resume read
/// the recorded answer. Every other field of the request can be rebuilt from the
/// machine or the transfer records; **this one cannot**, and its durability is
/// what makes* the answers the user already gave about collisions still stand
/// *true by construction."*
///
/// **The answer's effect is applied to the enumeration once** — dispositions per
/// file, Keep-both renames computed — **so a resume re-derives nothing.**
///
/// The Keep-both names are supplied by the caller in the enumeration's own
/// order, because composing one is [`crate::keep_both`]'s and wants the
/// destination listing this function does not hold.
///
/// **The answers are keyed to the entry by landing path and never paired by
/// position.** The walk's list is one comparison per **arriving thing** and X3's
/// taxonomy has folders in it, while U1's enumeration is **one entry per file**
/// — two different populations. **An entry no comparison names is refused**,
/// rather than left with no disposition: a request holding an entry with no
/// disposition never reads completed, so it would write no diary line and never
/// end.
///
/// **The moment is U9a's**, taken from the desktop's clock by the caller and
/// used only where a unit ends here.
pub fn apply_the_collision_answer(
    request: &mut UploadRequest,
    comparisons: &[AComparison],
    answer: CollisionAnswer,
    keep_both_names: &mut dyn Iterator<Item = Vec<u8>>,
    keeper: &mut dyn KeepsTheRequest,
    at: SystemTime,
) -> Result<WhatTheAnswerDid, Refused> {
    if request.collision_answer.is_some() {
        // Asked twice is a caller fault, not a state: U4 says the answer is
        // given once for the life of the request.
        return Err(Refused::ThatUnitHasAlreadyEnded);
    }
    let did = if answer == CollisionAnswer::Stop {
        // **Stop stops the request.** No disposition is written, no transfer is
        // arranged, and the record stands as the account of what the user
        // asked for and then called off — U1's *stopped by the user* ending.
        request.collision_answer = Some(answer);
        WhatTheAnswerDid::TheRequestStops
    } else {
        // **Every entry is matched before anything is written**, so a request
        // the answer cannot be applied to whole is left exactly as it was.
        let mut matched = Vec::with_capacity(request.entries.len());
        for entry in &request.entries {
            let found = comparisons
                .iter()
                .find(|comparison| paths_are_equal(&comparison.landing_path, &entry.landing_path));
            // **Refused rather than truncated.** A caller whose walk and whose
            // enumeration disagree has enumerated one population and asked
            // about another, which is what this refusal says.
            let Some(comparison) = found else {
                return Err(Refused::ThatUnitWasNotOwed);
            };
            matched.push(comparison.what);
        }

        request.collision_answer = Some(answer);
        let mut these_files_failed = Vec::new();
        for (entry, comparison) in request.entries.iter_mut().zip(matched) {
            let made = what_the_answer_makes_of_it(comparison, answer, keep_both_names);
            // **U13: a handed entry takes no disposition from the answer.** Its
            // name is still taken from the list, so every entry after it keeps
            // the name composed for it (X9's names run in the enumeration's
            // order). Added 2026-10-04, the 38's piece 7, when the hand-over
            // was first wired: until then nothing handed a unit over.
            if matches!(entry.disposition, Some(Disposition::HandedOver { .. })) {
                continue;
            }
            entry.disposition = Some(made.disposition);
            if made.the_name_could_not_be_composed {
                // **The unit ends here, as a failure naming the file**, at the
                // moment the answer is applied.
                entry.its_outcome_became(UnitOutcome::Failed, at)?;
                these_files_failed.push(entry.landing_path.clone());
            }
        }
        WhatTheAnswerDid::TheDispositionsAreWritten { these_files_failed }
    };
    // **Durable before any transfer whose arrangement depends on it**, which is
    // the whole of U4.
    keeper.write_it_down(request)?;
    Ok(did)
}

/// What applying U4's answer did to the request.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WhatTheAnswerDid {
    /// Every entry took its disposition and the request goes on.
    TheDispositionsAreWritten {
        /// **The landing paths of the units that ended here as failures** —
        /// Keep-both answers for which no name inside the length bounds could be
        /// composed. **The failure report names them** (N5), and they are in
        /// neither of U11's separate counts.
        these_files_failed: Vec<Vec<u8>>,
    },
    /// **Stop.** Nothing is sent, nothing is arranged, and the request's ending
    /// is *stopped by the user*.
    TheRequestStops,
}

/// What one entry's disposition is, and whether the unit ended while composing
/// it.
struct WhatTheAnswerMakesOfIt {
    disposition: Disposition,
    /// **Set where a Keep-both answer had no composable name for this file.**
    the_name_could_not_be_composed: bool,
}

fn what_the_answer_makes_of_it(
    comparison: Comparison,
    answer: CollisionAnswer,
    keep_both_names: &mut dyn Iterator<Item = Vec<u8>>,
) -> WhatTheAnswerMakesOfIt {
    let plainly = |disposition| WhatTheAnswerMakesOfIt {
        disposition,
        the_name_could_not_be_composed: false,
    };
    if !comparison.is_a_collision() {
        return plainly(Disposition::ToSend);
    }
    match answer {
        CollisionAnswer::Replace => plainly(Disposition::ToSend),
        CollisionAnswer::SkipExisting => plainly(Disposition::SkipExisting),
        CollisionAnswer::KeepBoth => match keep_both_names.next() {
            Some(fresh) => plainly(Disposition::RenamedUnderKeepBoth(fresh)),
            // **A generated name that could not be composed is never a silent
            // send under the colliding name, and it is never recorded as *skip
            // existing* either.** [`crate::keep_both::compose`] answers nothing
            // where no name inside X9's length bounds can be made; writing
            // *skip existing* there told the user one file was skipped because
            // it was already there, when nothing was there and the name would
            // not fit, and it carried that untruth into U11's separate count of
            // what Skip existing did and into B2's diary line.
            //
            // **The honest ending is `State, identity and recovery`'s
            // [`UnitOutcome::Failed`]** — a unit that was going to be sent and
            // could not be — written at the moment the answer is applied. **No
            // fifth disposition is added**: U1 declares three and U12 adds the
            // fourth, and a fifth would be a change to the plan. The disposition
            // stands as *to send*, which is the shape N4's over-long name
            // already takes, and [`arrange_a_transfer`] arranges nothing for a
            // unit that has already ended, so **it is never sent.**
            None => WhatTheAnswerMakesOfIt {
                disposition: Disposition::ToSend,
                the_name_could_not_be_composed: true,
            },
        },
        // Never reached: Stop is answered above, before any disposition is
        // written. It is matched rather than left to a catch-all so that a
        // fifth answer could not fall in here silently.
        CollisionAnswer::Stop => plainly(Disposition::SkipExisting),
    }
}

/// **The publication intent each unit carries**, taken from the walk and the
/// collision answer — X7's parameter, and it is `Server protocol and transfers`
/// S15a's own declared type rather than one of ours.
///
/// **The answer read is the one the record holds** (U4), never one passed in
/// afresh, so a resume re-derives nothing.
pub fn intent_for(
    request: &UploadRequest,
    entry: &EnumerationEntry,
    comparison: Comparison,
) -> Option<Intent> {
    match entry.disposition.as_ref()? {
        // A Keep-both name is a fresh name, so what is published is
        // create-intended whatever the walk found under the original.
        Disposition::RenamedUnderKeepBoth(_) => Some(Intent::CreateIntended),
        Disposition::ToSend => {
            crate::already_exists::publication_intent(comparison, request.collision_answer?)
        }
        Disposition::SkipExisting | Disposition::HandedOver { .. } => None,
    }
}

/// **U5: *waiting* is a composed presentation, not a fourth transfer ending.**
///
/// A file reads as waiting for the connection where its transfer record says
/// *interrupted* (`Server protocol and transfers` S26), the request is still
/// open, and the server's period still stands.
///
/// **A record reading *stopped* never composes as waiting**, and that excludes
/// by construction the two endings Smoozy writes itself: X7's create-intended
/// refusal and K8's and K9's changed-source stop both record *stopped*, so a
/// file Smoozy stopped is shown stopped with its reason rather than amber as
/// waiting for a line that is up. **Files whose transfers were never arranged
/// yet read as waiting their turn in the pool.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HowAUnitReads {
    /// Waiting for the connection — the amber reading.
    WaitingForTheConnection,
    /// Waiting its turn in the pool. **A different waiting and a different
    /// reading**: nothing is wrong with the line.
    WaitingItsTurn,
    Moving,
    Done,
    /// **Stopped, with its reason** — never amber.
    Stopped,
    Failed,
    Stalled,
    /// The unit this request arranges no transfer for.
    NotThisRequests,
}

/// U5, composed.
pub fn how_a_unit_reads(
    entry: &EnumerationEntry,
    ending: Option<&Ending>,
    the_request_is_still_open: bool,
    the_period_still_stands: bool,
) -> HowAUnitReads {
    match entry.disposition.as_ref() {
        None => HowAUnitReads::WaitingItsTurn,
        Some(disposition) if !disposition.a_transfer_is_arranged() => {
            HowAUnitReads::NotThisRequests
        }
        Some(_) => match ending {
            None if entry.transfer.is_none() => HowAUnitReads::WaitingItsTurn,
            None => HowAUnitReads::Moving,
            Some(Ending::Published) => HowAUnitReads::Done,
            Some(Ending::Stopped(_)) => HowAUnitReads::Stopped,
            Some(Ending::Failed) => HowAUnitReads::Failed,
            Some(Ending::Stalled) => HowAUnitReads::Stalled,
            Some(Ending::Interrupted) => {
                if the_request_is_still_open && the_period_still_stands {
                    HowAUnitReads::WaitingForTheConnection
                } else {
                    // Outside that stretch it is a leftover rather than a wait —
                    // U8's, and what U8 presents is composed from the records.
                    HowAUnitReads::Stopped
                }
            }
        },
    }
}

/// **U6: a waiting upload is stoppable exactly as a moving one.**
///
/// The stop is `Server protocol and transfers` S26's deliberate stop: the record
/// is written *stopped* before anything else, the staged material's removal
/// happens on the connection where one is up and is otherwise the owed cleanup
/// of S27.
///
/// **Stopping the request stops the whole request** — finished files stay,
/// unarranged files are never arranged, the one in flight is stopped. The stop
/// control sits on the request, which is Design's ruling.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhatAStopDoesToAUnit {
    /// Already published. **Finished files stay.**
    ItStays,
    /// A transfer was arranged and has not ended: it is stopped.
    StopTheTransfer,
    /// No transfer was ever arranged: **it is never arranged.**
    NeverArrangeIt,
    /// Nothing to do — the unit already ended some other way.
    NothingToDo,
}

/// U6, per unit.
pub fn what_a_stop_does(entry: &EnumerationEntry, ending: Option<&Ending>) -> WhatAStopDoesToAUnit {
    match (entry.transfer.as_ref(), ending) {
        (_, Some(Ending::Published)) => WhatAStopDoesToAUnit::ItStays,
        (_, Some(_)) => WhatAStopDoesToAUnit::NothingToDo,
        (Some(_), None) => WhatAStopDoesToAUnit::StopTheTransfer,
        (None, None) => WhatAStopDoesToAUnit::NeverArrangeIt,
    }
}

/// **U7: the request's activity is run-held; its record is not.**
///
/// *"Arrangement, admission, the byte loops and the waiting composition live in
/// process memory and end with the application run. **Closing Smoozy ends the
/// request**: channels die with the process, every in-flight transfer's record
/// reads* interrupted *(`Server protocol and transfers` S26's third ending — D6
/// makes a crash and a clean close leave the same records), and no activity
/// survives to arrange anything."*
pub const THE_ACTIVITY_IS_RUN_HELD_AND_THE_RECORD_IS_NOT: bool = true;

/// **U8: what a fresh sitting finds, and it is bounded on both sides.**
///
/// On the next connection to that server — after the installation-identity gate
/// — `Server protocol and transfers` C11 step 4 refuses to resume records
/// written by an earlier run; **that is taken, not softened.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ALeftover {
    pub request: Identity,
    pub server_row: Identity,
    pub transfer: Identity,
    /// The landing path the unit was for. **The line names the file and never a
    /// byte figure.**
    pub landing_path: Vec<u8>,
}

/// **U8's first bullet: the user is told the leftover is there and can remove
/// it** — Row 197's third clause, granted by the spec for exactly this case.
///
/// *"The telling is composed from the durable records: request records with
/// unfinished units and transfer records reading* interrupted *under the current
/// installation identity are presented in the transfer window as leftovers on
/// that machine, with a remove act."*
///
/// **The offline leftover line carries no byte total** (Row 511). `Server
/// protocol and transfers` S9's expected total is the size of the finished files
/// rather than what is staged, and its checkpointed high-water offset is an
/// accounted boundary which S10a observes can sit either ahead of or behind the
/// staged object's real size. **The real size belongs to the machine's
/// filesystem and cannot be read while that machine is disconnected; neither
/// durable candidate is relabelled as disk use.**
pub fn the_leftovers(
    requests: &[UploadRequest],
    transfers: &[TransferRecord],
    this_run: &Identity,
    installation_now: Option<&Identity>,
) -> Vec<ALeftover> {
    let mut out = Vec::new();
    for request in requests {
        if !request.ended_by_the_run_ending(this_run) {
            continue;
        }
        for entry in &request.entries {
            let Some(transfer) = entry.transfer.as_ref() else {
                continue;
            };
            if entry.outcome.is_some() {
                continue;
            }
            let Some(record) = transfers.iter().find(|record| &record.transfer == transfer) else {
                continue;
            };
            if record.ending != Some(Ending::Interrupted) {
                continue;
            }
            if record.installation.as_ref() != installation_now {
                continue;
            }
            out.push(ALeftover {
                request: request.record.clone(),
                server_row: request.server_row.clone(),
                transfer: transfer.clone(),
                landing_path: entry.landing_path.clone(),
            });
        }
    }
    out
}

/// **Row 511, stated as a value so nothing composes one by accident.**
///
/// *"The existence, server and remove act remain available from the records, and
/// Design is handed back a line with no figure."*
pub const THE_OFFLINE_LEFTOVER_LINE_CARRIES_NO_BYTE_TOTAL: bool = true;

/// **U8's second bullet: what a fresh, user-started upload does with staged
/// material.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WhatTheNewRequestDoes {
    /// **It takes up the interrupted transfer — its transfer identity, its
    /// record and its staging path** — and proceeds through `Server protocol and
    /// transfers` S12 entire plus Row 201's unchanged-source gate
    /// ([`crate::continuation`]).
    TakeUpTheInterruptedTransfer {
        transfer: Identity,
        /// **U8's third bullet.** The publication intent comes from the *new*
        /// request's collision answer, applied to the new enumeration once like
        /// every other unit's.
        publishing: Intent,
    },
    /// **Skip existing does not take it up at all** — the unit is skipped and
    /// the staging is abandoned to `Server protocol and transfers` S28's sweep.
    SkipItAndAbandonTheStaging,
    /// A fresh transfer under a new transfer identity; the old staging is
    /// abandoned to S28's sweep.
    StartFreshAndAbandonTheOldStaging,
    /// Nothing matched: an ordinary fresh unit.
    AnOrdinaryFreshUnit,
}

/// U8's second and third bullets together.
///
/// **The match is on source path *and* landing path**, which is why Keep both
/// can never match: the landing path it composes is a fresh name.
///
/// **A taken-up unit is governed by the new request throughout, and the old
/// request's collision answer never governs it.** *"A taken-up transfer is
/// continued rather than arranged, so X7's* each transfer the request arranges
/// *would leave its publication ungoverned and a builder could reach for the
/// only durable answer bound to that record — the old request's — and publish
/// replace-intended over a file the user has just answered **Skip existing**
/// for."*
pub fn what_the_new_request_does(
    entry: &EnumerationEntry,
    comparison: Comparison,
    interrupted: &[TransferRecord],
) -> WhatTheNewRequestDoes {
    let Some(disposition) = entry.disposition.as_ref() else {
        return WhatTheNewRequestDoes::AnOrdinaryFreshUnit;
    };
    // Keep both composes a fresh landing path, so the match below cannot fire
    // for it — it is spelled out rather than left to fall through, because the
    // clause says so in its own words.
    if matches!(disposition, Disposition::RenamedUnderKeepBoth(_)) {
        return WhatTheNewRequestDoes::AnOrdinaryFreshUnit;
    }
    let matched = interrupted.iter().find(|record| {
        record.ending == Some(Ending::Interrupted)
            && paths_are_equal(record.source_path.as_bytes(), &entry.source_path)
            && paths_are_equal(record.final_path.as_bytes(), &entry.landing_path)
    });
    let Some(matched) = matched else {
        return WhatTheNewRequestDoes::AnOrdinaryFreshUnit;
    };
    match disposition {
        Disposition::SkipExisting => WhatTheNewRequestDoes::SkipItAndAbandonTheStaging,
        Disposition::HandedOver { .. } => WhatTheNewRequestDoes::SkipItAndAbandonTheStaging,
        Disposition::ToSend => {
            let publishing = if comparison.is_a_collision() {
                Intent::ReplaceIntended
            } else {
                Intent::CreateIntended
            };
            WhatTheNewRequestDoes::TakeUpTheInterruptedTransfer {
                transfer: matched.transfer.clone(),
                publishing,
            }
        }
        Disposition::RenamedUnderKeepBoth(_) => WhatTheNewRequestDoes::AnOrdinaryFreshUnit,
    }
}

/// **U8's fourth bullet: nothing starts itself.**
///
/// *"No upload starts itself again on a later connection is kept by construction
/// rather than by care: the only mechanism that resumes transfers unasked is
/// `Server protocol and transfers` C11 step 4, which excludes earlier-run records
/// by the run marker S9 carries, and the only mechanism that arranges new
/// transfers is a request the user opened in this run."*
pub const NOTHING_STARTS_ITSELF_ON_A_LATER_CONNECTION: bool = true;

/// **U9: the already-exists check in that fresh sitting is what puts Skip
/// existing forward.**
///
/// The finished files sit at their final names and the walk of X1 counts them;
/// the partial sits at a staging name and is never counted (X10). *"Smoozy says
/// what is already there and puts Skip existing forward is therefore the
/// ordinary collision question doing its ordinary work, with Design told which
/// answer to lead with."*
/// **`None` where nothing is put forward.** The spec grants the saying only
/// *"where Smoozy holds the earlier request **and can say why**"*, and the four
/// answers otherwise stand as they are; this function answers U9's question and
/// no wider one.
///
/// **It takes the reason line itself and never a flag, so the two halves of that
/// sentence cannot come apart.** A plain *there is an earlier unfinished
/// request* is a different condition from [`the_reason_line`]'s — which wants an
/// entry whose unit ended *done* at a landing path the walk found occupied — and
/// nothing joined them, so a caller reading the parameter's own name would put
/// the answer forward with no reason beside it, which is the case the spec's
/// *and* rules out. **U9b is explicit about the negative**: *"Where no candidate
/// entry exists, nothing is supplied."*
pub fn the_answer_to_lead_with(the_reason_line: Option<&TheReasonLine>) -> Option<CollisionAnswer> {
    the_reason_line.map(|_| CollisionAnswer::SkipExisting)
}

/// **U9b: the earlier request the reason line names, and the value that line
/// dates, are composed at the new request's pre-flight and are one record and
/// one value.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TheReasonLine {
    /// The record the line names.
    pub the_earlier_request: Identity,
    /// **The moment the line dates: the last file that upload landed at these
    /// paths** — the nearest recorded moment to the application run ending, the
    /// run ending itself having none (U9a).
    pub the_last_file_it_landed: SystemTime,
}

/// U9b, composed. **Reads the local record and never the machine, and reads no
/// transfer record.**
///
/// *"A leftover swept under `Server protocol and transfers` S28, or removed by
/// the user under U8's first bullet, moves neither the record named nor the
/// value — the finished files the line is about are at their final names and
/// their entries are immutable."*
///
/// **Where no candidate entry exists, nothing is supplied.** The negative case
/// in the box is Design's ruling.
pub fn the_reason_line(
    requests: &[UploadRequest],
    this_run: &Identity,
    server_row: &Identity,
    installation_now: Option<&Identity>,
    occupied_landing_paths: &[Vec<u8>],
) -> Option<TheReasonLine> {
    let mut best: Option<TheReasonLine> = None;
    for request in requests {
        if &request.server_row != server_row {
            continue;
        }
        if request.installation.as_ref() != installation_now {
            continue;
        }
        // **Whose ending derives as *ended by the run ending*** — U1's third
        // ending, which by construction is a record no longer alive and so an
        // earlier run's. A completed request and one the user stopped are both
        // excluded, and the line is about neither.
        //
        // **An earlier run's request whose every unit ended is completed, not
        // ended by the run ending**: U1 derives the ending from the unit
        // outcomes and the run, and the run says nothing about a request that
        // had already finished in it. **The case this was first written for**
        // was a request the run's end caught mid-send. **The case it now also
        // meets** is one that finished, then Smoozy was reopened: the pass
        // before 0.1.9 on Vast read *This upload was interrupted* over files
        // an upload had finished sending.
        if !request.ended_by_the_run_ending(this_run) || request.every_unit_ended() {
            continue;
        }
        for entry in &request.entries {
            if entry.outcome != Some(UnitOutcome::Done) {
                continue;
            }
            let occupied = occupied_landing_paths
                .iter()
                .any(|path| paths_are_equal(path, &entry.landing_path));
            if !occupied {
                continue;
            }
            let Some(at) = entry.outcome_written_at else {
                continue;
            };
            let better = match &best {
                None => true,
                Some(held) => at > held.the_last_file_it_landed,
            };
            if better {
                best = Some(TheReasonLine {
                    the_earlier_request: request.record.clone(),
                    the_last_file_it_landed: at,
                });
            }
        }
    }
    best
}

/// **U10: this document's uploads are one declared priority class — the upload
/// class** — declared per `Server protocol and transfers` T8 by the arranging
/// module, per action: every transfer an upload request arranges carries it.
///
/// Its rank and share are [`crate::pool`]'s; the class name itself is
/// [`crate::declarations::THE_TRANSFER_PRIORITY_CLASS`] and is not spelled twice.
pub fn the_priority_class() -> &'static str {
    crate::declarations::THE_TRANSFER_PRIORITY_CLASS
}

/// **U11: the whole is counted in files, and the bar is drawn in bytes; both
/// totals exist before the first transfer starts.**
///
/// `Server protocol and transfers` G6 leaves to Row 190 what the whole is
/// counted in and what it reads while a deep folder is still being listed.
///
/// **Skipped units are in neither total and are reported separately as what Skip
/// existing did**, because the spec's own sentence for that answer is *"send up
/// only the files that are not already there"* — counting them into *m* would
/// draw a denominator holding bytes that will never cross, and counting them done
/// before a byte moves would show progress for work nobody asked for.
///
/// **Handed units are in neither total either** (U14) **and for the same
/// reason**: their bytes cross under `Dependency Upload & Remap`'s transfers and
/// its declared priority class (its A9), not this request's upload class.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TheWhole {
    /// *n of m files* — unit outcomes over the units whose disposition is to
    /// send.
    pub files_done: usize,
    pub files_to_send: usize,
    /// Bytes staged against bytes expected over those same units, summed from
    /// the per-transfer counts of G4 against the enumerated sizes.
    pub bytes_staged: u64,
    pub bytes_expected: u64,
    /// **Reported separately**, never inside the totals.
    pub skipped_existing: usize,
    /// **Reported separately**, never inside the totals (U14).
    pub handed_over: usize,
    /// **The statement that accounts for a bar that fell.** One statement over
    /// the request however many units carry G4a's mark at once, standing for as
    /// long as any of them does.
    pub a_file_started_over: bool,
}

impl TheWhole {
    /// The four counts B2's diary line names, read off the same composition
    /// rather than counted a second way.
    pub fn the_four_counts(&self) -> TheFourCounts {
        TheFourCounts {
            to_send: self.files_to_send,
            done: self.files_done,
            skipped_existing: self.skipped_existing,
            handed_over: self.handed_over,
        }
    }
}

/// U11, composed over the enumeration and the live reports.
///
/// **Because the enumeration is complete before the collision question and
/// durable before the first byte (U1), there is no state in which the whole is
/// still being discovered while transfers run** — the walk happens first, and a
/// request that has started moving knows its totals.
///
/// **The whole's report reads G4a's marks and never the bar's direction**: a
/// summed byte total that has just begun and one that has fallen because a unit
/// began again are the same figure. **The file count does not move on a
/// start-over**, no unit outcome having been written, so the bar is the only
/// total that falls and the statement is what accounts for it.
pub fn the_whole(request: &UploadRequest, reports: &[(Identity, Report)]) -> TheWhole {
    let mut whole = TheWhole {
        files_done: 0,
        files_to_send: 0,
        bytes_staged: 0,
        bytes_expected: 0,
        skipped_existing: 0,
        handed_over: 0,
        a_file_started_over: false,
    };
    for entry in &request.entries {
        match entry.disposition.as_ref() {
            Some(Disposition::SkipExisting) => whole.skipped_existing += 1,
            Some(Disposition::HandedOver { .. }) => whole.handed_over += 1,
            Some(disposition) if disposition.is_in_the_totals() => {
                whole.files_to_send += 1;
                whole.bytes_expected += entry.size_as_enumerated;
                if entry.outcome == Some(UnitOutcome::Done) {
                    whole.files_done += 1;
                }
                if let Some(transfer) = entry.transfer.as_ref() {
                    if let Some((_, report)) =
                        reports.iter().find(|(identity, _)| identity == transfer)
                    {
                        whole.bytes_staged += report.bytes_staged;
                        if report.started_over.is_some() {
                            whole.a_file_started_over = true;
                        }
                    }
                }
            }
            _ => {}
        }
    }
    whole
}

/// **G4a's two causes, carried rather than re-invented.** Stated here so a
/// reader of U11's start-over statement can see that no mark of this document's
/// own is made: the marks are read where they arrive.
pub fn a_start_over_cause_is_read_not_composed(cause: StartOverCause) -> StartOverCause {
    cause
}

/// **U12: the offer's unit is the enumeration entry, and taking it up removes
/// nothing from the request.**
///
/// **The enumeration is never shortened.** `State, identity and recovery` §9.4
/// item 1 makes it durable before the first unit is attempted *"so an
/// interruption cannot lose the fact that a unit was owed"*, and deleting entries
/// at the collision question is exactly that loss.
///
/// **The disposition and the unit outcome are written when that module's act is
/// opened**, so a hand-over that does not open leaves the entries as the
/// collision answer left them.
///
/// **U13: the collision answer governs the units the request keeps and never a
/// handed one.** A handed entry takes no disposition from it, this document
/// arranges no transfer for it, and what does move it moves under that module's
/// own rules and its own replace-or-stop bound (its Q8).
pub fn hand_over(
    request: &mut UploadRequest,
    landing_path: &[u8],
    the_act: Identity,
    at: SystemTime,
    keeper: &mut dyn KeepsTheRequest,
) -> Result<(), Refused> {
    let Some(entry) = request
        .entries
        .iter_mut()
        .find(|entry| paths_are_equal(&entry.landing_path, landing_path))
    else {
        return Err(Refused::ThatUnitWasNotOwed);
    };
    entry.disposition = Some(Disposition::HandedOver {
        the_act: the_act.clone(),
    });
    entry.its_outcome_became(
        UnitOutcome::NotAttempted {
            // **U12's own words**: the outcome names the operation identity of
            // the act that module opens for them. Nothing else is composed into
            // this field and no prose is written here.
            stopped_by: the_act.as_text(),
        },
        at,
    )?;
    keeper.write_it_down(request)
}

/// **U14: a request every one of whose units was handed over ends completed
/// having arranged no transfer**, which is a true reading of what it did and not
/// an empty one.
pub fn ends_completed_having_arranged_nothing(request: &UploadRequest) -> bool {
    !request.entries.is_empty()
        && request
            .entries
            .iter()
            .all(|entry| matches!(entry.disposition, Some(Disposition::HandedOver { .. })))
}

/// **U15: what crosses at the hand-over, per unit, and nothing else.**
///
/// **Nothing of this request's own state crosses** — not the collision answer,
/// not the destination folder, not the totals.
///
/// **The entry point that takes them is `Dependency Upload & Remap`'s own,
/// declared at its 5.6 A15–A18.** A15 takes these four fields and reads nothing
/// of this request's own state; A16 opens one act per unit and returns that
/// act's operation identity, which is the value [`hand_over`] writes; A17 is the
/// send, the reading and the fixing that module already describes; A18 keeps the
/// occupied landing path as what the offer was about rather than as the act's
/// destination.
///
/// **The class is W1a's, and W1a is built at [`crate::warnings`]** — its
/// three-way class is `crate::warnings::WhatTheLandingPathIsFor`. *(Corrected
/// 2026-09-07: this said W1a is step 7's and pointed at
/// [`crate::arriving_later`]; step 7 landed it.)* This type still carries the
/// class rather than composing one: the field's type is the caller's, and this
/// document neither reads it nor interprets it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WhatCrosses<Class> {
    /// The file's source path on the user's own computer, as stored on disk —
    /// U1's entry field, which is `Dependency Upload & Remap` K3's holding-file
    /// key and `Dependency Upload & Remap` K7's source location.
    pub source_path: Vec<u8>,
    pub server_row: Identity,
    pub installation: Option<Identity>,
    /// The landing path the walk found occupied.
    pub occupied_landing_path: Vec<u8>,
    /// **W1a's class for it**, carried and never read here.
    pub the_class_w1a_read: Class,
}

/// U15, composed for one entry.
pub fn what_crosses<Class>(
    request: &UploadRequest,
    entry: &EnumerationEntry,
    the_class_w1a_read: Class,
) -> WhatCrosses<Class> {
    WhatCrosses {
        source_path: entry.source_path.clone(),
        server_row: request.server_row.clone(),
        installation: request.installation.clone(),
        occupied_landing_path: entry.landing_path.clone(),
        the_class_w1a_read,
    }
}

#[cfg(test)]
pub mod in_a_list {
    //! A keeper that holds its requests in a list — **the whole of U1's
    //! ordering with none of a store**, so §3 is testable at the desk exactly
    //! as `State, identity and recovery`'s own `a_list` keeper makes §9.4
    //! testable.

    use super::*;

    #[derive(Default)]
    pub struct AList {
        pub written: Vec<UploadRequest>,
        pub writes: usize,
    }

    impl KeepsTheRequest for AList {
        fn write_it_down(&mut self, request: &UploadRequest) -> Result<(), Refused> {
            self.writes += 1;
            match self
                .written
                .iter_mut()
                .find(|held| held.record == request.record)
            {
                Some(held) => *held = request.clone(),
                None => self.written.push(request.clone()),
            }
            Ok(())
        }

        fn every_request_for(&self, server_row: &Identity) -> Result<Vec<UploadRequest>, Refused> {
            Ok(self
                .written
                .iter()
                .filter(|request| &request.server_row == server_row)
                .cloned()
                .collect())
        }

        fn the_user_removed_it(&mut self, record: &Identity) -> Result<(), Refused> {
            self.written.retain(|request| &request.record != record);
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use smoozy_server_protocol::transfers::Direction;
    use smoozy_state::identities::IdentityKind;
    use std::time::Duration;

    fn an_identity(kind: IdentityKind) -> Identity {
        Identity::issue(kind).expect("this machine supplies random bytes")
    }

    /// A request opened through the ordinary upload boundary.
    fn a_request(entries: Vec<EnumerationEntry>) -> UploadRequest {
        UploadRequest::confirmed(
            open_an_upload_request().expect("the answer handed in says open"),
            an_identity(IdentityKind::Operation),
            an_identity(IdentityKind::LocalServer),
            Some(an_identity(IdentityKind::Installation)),
            b"/jobs".to_vec(),
            entries,
            an_identity(IdentityKind::Run),
        )
    }

    fn an_entry(source: &[u8], landing: &[u8], size: u64) -> EnumerationEntry {
        EnumerationEntry::enumerated(source.to_vec(), landing.to_vec(), size)
    }

    /// U9a's moment, fixed so a test reads the same twice.
    fn a_moment() -> SystemTime {
        SystemTime::UNIX_EPOCH + Duration::from_secs(1_700_000_000)
    }

    /// The walk's answers for a file-only enumeration, **keyed by landing path
    /// the way `already_exists::walk` keys them**. A test that wants a folder
    /// among the arriving things builds the list itself.
    fn answers_for(request: &UploadRequest, comparisons: &[Comparison]) -> Vec<AComparison> {
        request
            .entries
            .iter()
            .zip(comparisons)
            .map(|(entry, what)| AComparison {
                landing_path: entry.landing_path.clone(),
                what: *what,
            })
            .collect()
    }

    /// **§9.4 names a unit by role, and §11 now says what this document's role
    /// is.** Until it did, a keeper had nothing to write into the enumeration's
    /// name and the store could not be built at all.
    #[test]
    fn every_unit_is_named_by_the_one_role_and_the_position_tells_them_apart() {
        let request = a_request(vec![
            an_entry(b"/home/a.png", b"/jobs/a.png", 10),
            an_entry(b"/home/b.png", b"/jobs/b.png", 20),
            an_entry(b"/home/deep/a.png", b"/jobs/deep/a.png", 30),
        ]);
        // One role per unit, in the enumeration's own order.
        assert_eq!(request.the_units_by_role().len(), request.entries.len());
        // **The same words for every unit.** Two files of the same name in two
        // folders are two units and their roles are identical — what tells them
        // apart is the position, which U1's entry at that position carries the
        // paths for.
        assert!(request
            .the_units_by_role()
            .iter()
            .all(|role| *role == the_role_every_unit_carries()));
        // **The words are §11's own row and are not spelled twice.**
        assert_eq!(
            the_role_every_unit_carries(),
            crate::declarations::the_role_a_unit_of_carries(THE_ACTION).expect("§11 declares it")
        );
        // **A role and never a value**: no path reaches the shared table.
        let role = the_role_every_unit_carries();
        for entry in &request.entries {
            assert_ne!(role.as_bytes(), entry.source_path.as_slice());
            assert_ne!(role.as_bytes(), entry.landing_path.as_slice());
        }
        assert!(!role.contains('/'));
    }

    #[test]
    fn u4s_answer_is_written_once_and_never_asked_again() {
        let mut request = a_request(vec![an_entry(b"/home/a.png", b"/jobs/a.png", 10)]);
        let mut keeper = in_a_list::AList::default();
        let comparisons = answers_for(&request, &[Comparison::CollidesWithAFile]);
        apply_the_collision_answer(
            &mut request,
            &comparisons,
            CollisionAnswer::Replace,
            &mut std::iter::empty(),
            &mut keeper,
            a_moment(),
        )
        .expect("the answer is written");
        assert_eq!(request.collision_answer, Some(CollisionAnswer::Replace));
        // A second answer is refused: U4 gives it once for the life of the
        // request.
        let comparisons = answers_for(&request, &[Comparison::CollidesWithAFile]);
        let again = apply_the_collision_answer(
            &mut request,
            &comparisons,
            CollisionAnswer::SkipExisting,
            &mut std::iter::empty(),
            &mut keeper,
            a_moment(),
        );
        assert_eq!(again, Err(Refused::ThatUnitHasAlreadyEnded));
        assert_eq!(request.collision_answer, Some(CollisionAnswer::Replace));
    }

    #[test]
    fn the_answer_is_durable_before_a_transfer_that_depends_on_it_is_arranged() {
        let mut request = a_request(vec![an_entry(b"/home/a.png", b"/jobs/a.png", 10)]);
        let mut keeper = in_a_list::AList::default();
        let comparisons = answers_for(&request, &[Comparison::CollidesWithAFile]);
        apply_the_collision_answer(
            &mut request,
            &comparisons,
            CollisionAnswer::Replace,
            &mut std::iter::empty(),
            &mut keeper,
            a_moment(),
        )
        .expect("written");
        // The write landed before anything could be arranged: the keeper holds
        // the answer already.
        assert_eq!(keeper.writes, 1);
        assert_eq!(
            keeper.written[0].collision_answer,
            Some(CollisionAnswer::Replace)
        );
    }

    #[test]
    fn keep_both_takes_the_generated_name_and_replace_does_not() {
        let mut request = a_request(vec![
            an_entry(b"/home/a.png", b"/jobs/a.png", 10),
            an_entry(b"/home/b.png", b"/jobs/b.png", 20),
        ]);
        let mut keeper = in_a_list::AList::default();
        let mut names = vec![b"/jobs/a (2).png".to_vec()].into_iter();
        let comparisons = answers_for(
            &request,
            &[Comparison::CollidesWithAFile, Comparison::LandsFresh],
        );
        apply_the_collision_answer(
            &mut request,
            &comparisons,
            CollisionAnswer::KeepBoth,
            &mut names,
            &mut keeper,
            a_moment(),
        )
        .expect("written");
        assert_eq!(
            request.entries[0].disposition,
            Some(Disposition::RenamedUnderKeepBoth(
                b"/jobs/a (2).png".to_vec()
            ))
        );
        assert_eq!(request.entries[1].disposition, Some(Disposition::ToSend));
    }

    /// **U13: the answer governs the units the request keeps and never a
    /// handed one** — and the keep-both names still run in the enumeration's
    /// order, so the entry after a handed one keeps the name composed for it.
    /// The 38's piece 7, when the hand-over was first wired.
    #[test]
    fn the_answer_leaves_a_handed_unit_handed_and_every_other_name_in_its_place() {
        let mut request = a_request(vec![
            an_entry(b"/home/a.png", b"/jobs/a.png", 10),
            an_entry(b"/home/b.png", b"/jobs/b.png", 20),
        ]);
        let mut keeper = in_a_list::AList::default();
        let act = an_identity(IdentityKind::Operation);
        hand_over(&mut request, b"/jobs/a.png", act.clone(), a_moment(), &mut keeper)
            .expect("the hand-over is written");
        let comparisons = answers_for(
            &request,
            &[Comparison::CollidesWithAFile, Comparison::CollidesWithAFile],
        );
        let mut names = vec![b"/jobs/a (2).png".to_vec(), b"/jobs/b (2).png".to_vec()].into_iter();
        apply_the_collision_answer(
            &mut request,
            &comparisons,
            CollisionAnswer::KeepBoth,
            &mut names,
            &mut keeper,
            a_moment(),
        )
        .expect("written");
        assert_eq!(
            request.entries[0].disposition,
            Some(Disposition::HandedOver { the_act: act })
        );
        assert_eq!(
            request.entries[1].disposition,
            Some(Disposition::RenamedUnderKeepBoth(b"/jobs/b (2).png".to_vec()))
        );
    }

    #[test]
    fn a_skipped_unit_arranges_no_transfer_and_a_sent_one_does() {
        let mut request = a_request(vec![
            an_entry(b"/home/a.png", b"/jobs/a.png", 10),
            an_entry(b"/home/b.png", b"/jobs/b.png", 20),
        ]);
        let mut keeper = in_a_list::AList::default();
        let comparisons = answers_for(
            &request,
            &[Comparison::CollidesWithAFile, Comparison::LandsFresh],
        );
        apply_the_collision_answer(
            &mut request,
            &comparisons,
            CollisionAnswer::SkipExisting,
            &mut std::iter::empty(),
            &mut keeper,
            a_moment(),
        )
        .expect("written");
        let run = an_identity(IdentityKind::Run);
        assert!(arrange_a_transfer(
            &request,
            &request.entries[0],
            an_identity(IdentityKind::Transfer),
            "/jobs/.smoozy-staging",
            run.clone(),
        )
        .expect("both paths are valid text")
        .is_none());
        let arranged = arrange_a_transfer(
            &request,
            &request.entries[1],
            an_identity(IdentityKind::Transfer),
            "/jobs/.smoozy-staging",
            run,
        )
        .expect("both paths are valid text")
        .expect("a unit to send is arranged");
        assert_eq!(arranged.direction, Direction::Up);
        assert_eq!(arranged.final_path, "/jobs/b.png");
        assert_eq!(arranged.total_size_expected, 20);
        assert_eq!(arranged.server, request.server_row);
    }

    /// **N2 at the transfer seam: a name that is not valid text is refused per
    /// file and never rewritten to get it across.**
    #[test]
    fn a_source_path_that_is_not_valid_text_fails_that_file_and_arranges_no_transfer() {
        // A single byte no decoder can read. The enumeration holds it as it is
        // on disk, which is what N2 requires of the reading.
        let bad = b"/home/\xffscene.blend".to_vec();
        let mut request = a_request(vec![
            an_entry(&bad, b"/jobs/\xffscene.blend", 10),
            an_entry(b"/home/b.png", b"/jobs/b.png", 20),
        ]);
        let mut keeper = in_a_list::AList::default();
        let comparisons = answers_for(&request, &[Comparison::LandsFresh, Comparison::LandsFresh]);
        apply_the_collision_answer(
            &mut request,
            &comparisons,
            CollisionAnswer::Replace,
            &mut std::iter::empty(),
            &mut keeper,
            a_moment(),
        )
        .expect("written");

        // The pre-flight refuses it, names the file by its own bytes, and the
        // unit's ending is a failure rather than anything that reads as a
        // success.
        let refused = crate::uploading::check_the_name_can_be_carried(
            &request.entries[0].source_path,
            &request.entries[0].landing_path,
        )
        .expect_err("this name cannot be sent");
        assert_eq!(refused.the_file_it_names(), bad.as_slice());
        assert_eq!(
            TheNameCannotBeCarried::the_unit_ending(),
            UnitOutcome::Failed
        );

        // And no route arranges a transfer for it.
        assert_eq!(
            arrange_a_transfer(
                &request,
                &request.entries[0],
                an_identity(IdentityKind::Transfer),
                "/jobs/.smoozy-staging",
                an_identity(IdentityKind::Run),
            ),
            Err(TheNameCannotBeCarried::TheSourcePathIsNotValidText { source_path: bad })
        );

        // **N5: what it costs is that file and nothing else.** The rest of the
        // folder is arranged exactly as before.
        let rest = arrange_a_transfer(
            &request,
            &request.entries[1],
            an_identity(IdentityKind::Transfer),
            "/jobs/.smoozy-staging",
            an_identity(IdentityKind::Run),
        )
        .expect("this one's paths are valid text")
        .expect("a unit to send is arranged");
        assert_eq!(rest.final_path, "/jobs/b.png");
    }

    /// **The record's paths are the enumeration's bytes, for every unit that is
    /// arranged** — the property the two lossy conversions broke.
    #[test]
    fn an_arranged_transfers_paths_are_the_enumerations_own_bytes() {
        // Names that are valid text and are not plain: spaces, Cyrillic,
        // Japanese and an emoji all reached the machine byte-for-byte.
        let entries = vec![
            an_entry(
                "/home/съёмка 1.blend".as_bytes(),
                "/jobs/съёмка 1.blend".as_bytes(),
                10,
            ),
            an_entry(
                "/home/レンダー 🎬.png".as_bytes(),
                "/jobs/レンダー 🎬.png".as_bytes(),
                20,
            ),
            an_entry(
                b"/home/-leading-dash\tand\ttabs",
                b"/jobs/-leading-dash\tand\ttabs",
                30,
            ),
            // And one that is not valid text at all. **It is in this list on
            // purpose**: a conversion that replaced the byte it cannot read
            // would arrange a transfer whose paths are not the enumeration's,
            // and this loop is where that shows.
            an_entry(b"/home/\xfe\xfe.png", b"/jobs/\xfe\xfe.png", 40),
        ];
        let mut request = a_request(entries.clone());
        let mut keeper = in_a_list::AList::default();
        let comparisons = answers_for(&request, &vec![Comparison::LandsFresh; entries.len()]);
        apply_the_collision_answer(
            &mut request,
            &comparisons,
            CollisionAnswer::Replace,
            &mut std::iter::empty(),
            &mut keeper,
            a_moment(),
        )
        .expect("written");

        let mut arranged_units = 0;
        for (entry, enumerated) in request.entries.iter().zip(&entries) {
            match arrange_a_transfer(
                &request,
                entry,
                an_identity(IdentityKind::Transfer),
                "/jobs/.smoozy-staging",
                an_identity(IdentityKind::Run),
            ) {
                Ok(Some(arranged)) => {
                    assert_eq!(arranged.source_path.as_bytes(), enumerated.source_path);
                    assert_eq!(arranged.final_path.as_bytes(), enumerated.landing_path);
                    arranged_units += 1;
                }
                Ok(None) => panic!("every unit here is one to send"),
                Err(refused) => {
                    // The only unit that may be refused is the one whose name
                    // cannot be carried, and it is arranged nowhere.
                    assert!(std::str::from_utf8(&enumerated.source_path).is_err());
                    assert_eq!(refused.the_file_it_names(), enumerated.source_path);
                }
            }
        }
        assert_eq!(arranged_units, entries.len() - 1);
    }

    /// **A Keep-both name that cannot be composed is a failure naming the file,
    /// and never *skip existing*.**
    ///
    /// [`crate::keep_both::compose`] answers nothing where no name inside X9's
    /// length bounds can be made. Writing *skip existing* there told the user
    /// one file was skipped because it was already there — nothing was there
    /// and the name would not fit — and carried the wrong figure into U11's
    /// separate count and B2's diary line.
    #[test]
    fn a_keep_both_name_that_cannot_be_composed_fails_that_file_and_is_never_a_skip() {
        let mut request = a_request(vec![
            an_entry(b"/home/a.png", b"/jobs/a.png", 10),
            an_entry(b"/home/b.png", b"/jobs/b.png", 20),
            an_entry(b"/home/c.png", b"/jobs/c.png", 30),
        ]);
        let mut keeper = in_a_list::AList::default();
        // Two files collide; only one name could be composed.
        let mut names = vec![b"/jobs/a (2).png".to_vec()].into_iter();
        let comparisons = answers_for(
            &request,
            &[
                Comparison::CollidesWithAFile,
                Comparison::CollidesWithAFile,
                Comparison::LandsFresh,
            ],
        );
        let did = apply_the_collision_answer(
            &mut request,
            &comparisons,
            CollisionAnswer::KeepBoth,
            &mut names,
            &mut keeper,
            a_moment(),
        )
        .expect("written");

        // **The failure names the file**, at the moment the answer was applied.
        assert_eq!(
            did,
            WhatTheAnswerDid::TheDispositionsAreWritten {
                these_files_failed: vec![b"/jobs/b.png".to_vec()]
            }
        );
        assert_eq!(request.entries[1].outcome, Some(UnitOutcome::Failed));
        assert_eq!(request.entries[1].outcome_written_at, Some(a_moment()));
        assert_ne!(
            request.entries[1].disposition,
            Some(Disposition::SkipExisting),
            "nothing was there and the name would not fit"
        );

        // **And it is never sent**, under the colliding name or any other.
        assert!(arrange_a_transfer(
            &request,
            &request.entries[1],
            an_identity(IdentityKind::Transfer),
            "/jobs/.smoozy-staging",
            an_identity(IdentityKind::Run),
        )
        .expect("its paths are valid text")
        .is_none());

        // **U11's separate count of what Skip existing did is untouched.**
        let whole = the_whole(&request, &[]);
        assert_eq!(whole.skipped_existing, 0);
        assert_eq!(whole.the_four_counts().skipped_existing, 0);

        // **And the rest of the folder lands.**
        assert_eq!(
            request.entries[0].disposition,
            Some(Disposition::RenamedUnderKeepBoth(
                b"/jobs/a (2).png".to_vec()
            ))
        );
        assert_eq!(request.entries[2].disposition, Some(Disposition::ToSend));
        assert!(arrange_a_transfer(
            &request,
            &request.entries[2],
            an_identity(IdentityKind::Transfer),
            "/jobs/.smoozy-staging",
            an_identity(IdentityKind::Run),
        )
        .expect("valid text")
        .is_some());
    }

    /// **A folder among the arriving things must not shift the files' answers.**
    ///
    /// `already_exists::walk` returns one comparison per **arriving thing** and
    /// X3's taxonomy has folders in it, while U1's enumeration is one entry per
    /// file. Paired by position, every entry after the first folder took another
    /// file's answer — a wrong publication intent arriving quietly.
    #[test]
    fn a_folder_among_the_arriving_things_shifts_nobodys_answer() {
        let mut request = a_request(vec![
            an_entry(b"/home/tree/a.png", b"/jobs/tree/a.png", 10),
            an_entry(b"/home/tree/b.png", b"/jobs/tree/b.png", 20),
        ]);
        let mut keeper = in_a_list::AList::default();
        // The walk's list: the folder first — it merges onto a same-named
        // folder — then the two files, of which only the second collides.
        let comparisons = vec![
            AComparison {
                landing_path: b"/jobs/tree".to_vec(),
                what: Comparison::Merges,
            },
            AComparison {
                landing_path: b"/jobs/tree/a.png".to_vec(),
                what: Comparison::LandsFresh,
            },
            AComparison {
                landing_path: b"/jobs/tree/b.png".to_vec(),
                what: Comparison::CollidesWithAFile,
            },
        ];
        apply_the_collision_answer(
            &mut request,
            &comparisons,
            CollisionAnswer::SkipExisting,
            &mut std::iter::empty(),
            &mut keeper,
            a_moment(),
        )
        .expect("written");

        assert_eq!(request.entries[0].disposition, Some(Disposition::ToSend));
        assert_eq!(
            request.entries[1].disposition,
            Some(Disposition::SkipExisting),
            "the second file took its own answer and not the folder's"
        );
    }

    /// **An entry no comparison names is refused rather than truncated**, and
    /// the request is left exactly as it was.
    #[test]
    fn an_entry_the_walk_says_nothing_about_is_refused_and_writes_nothing() {
        let mut request = a_request(vec![
            an_entry(b"/home/a.png", b"/jobs/a.png", 10),
            an_entry(b"/home/b.png", b"/jobs/b.png", 20),
        ]);
        let mut keeper = in_a_list::AList::default();
        let comparisons = vec![AComparison {
            landing_path: b"/jobs/a.png".to_vec(),
            what: Comparison::LandsFresh,
        }];
        assert_eq!(
            apply_the_collision_answer(
                &mut request,
                &comparisons,
                CollisionAnswer::Replace,
                &mut std::iter::empty(),
                &mut keeper,
                a_moment(),
            ),
            Err(Refused::ThatUnitWasNotOwed)
        );
        assert_eq!(request.collision_answer, None);
        assert!(request
            .entries
            .iter()
            .all(|entry| entry.disposition.is_none()));
        assert_eq!(keeper.writes, 0);
    }

    #[test]
    fn a_keep_both_transfer_is_arranged_at_the_fresh_name() {
        let mut request = a_request(vec![an_entry(b"/home/a.png", b"/jobs/a.png", 10)]);
        let mut keeper = in_a_list::AList::default();
        let mut names = vec![b"/jobs/a (2).png".to_vec()].into_iter();
        let comparisons = answers_for(&request, &[Comparison::CollidesWithAFile]);
        apply_the_collision_answer(
            &mut request,
            &comparisons,
            CollisionAnswer::KeepBoth,
            &mut names,
            &mut keeper,
            a_moment(),
        )
        .expect("written");
        let arranged = arrange_a_transfer(
            &request,
            &request.entries[0],
            an_identity(IdentityKind::Transfer),
            "/jobs/.smoozy-staging",
            an_identity(IdentityKind::Run),
        )
        .expect("both paths are valid text")
        .expect("arranged");
        assert_eq!(arranged.final_path, "/jobs/a (2).png");
    }

    #[test]
    fn u5_never_composes_a_stopped_record_as_waiting() {
        let mut entry = an_entry(b"/home/a.png", b"/jobs/a.png", 10);
        entry.disposition = Some(Disposition::ToSend);
        entry.transfer = Some(an_identity(IdentityKind::Transfer));
        assert_eq!(
            how_a_unit_reads(&entry, Some(&Ending::Interrupted), true, true),
            HowAUnitReads::WaitingForTheConnection
        );
        for ending in [
            Ending::Stopped(
                smoozy_server_protocol::transfers::StoppedReason::TheSourceChangedUnderIt,
            ),
            Ending::Stopped(smoozy_server_protocol::transfers::StoppedReason::StagingRemovedByUser),
            Ending::Failed,
            Ending::Stalled,
        ] {
            assert_ne!(
                how_a_unit_reads(&entry, Some(&ending), true, true),
                HowAUnitReads::WaitingForTheConnection,
                "{ending:?} must never read as waiting for the connection"
            );
        }
    }

    #[test]
    fn a_unit_with_no_transfer_yet_reads_as_waiting_its_turn() {
        let mut entry = an_entry(b"/home/a.png", b"/jobs/a.png", 10);
        entry.disposition = Some(Disposition::ToSend);
        assert_eq!(
            how_a_unit_reads(&entry, None, true, true),
            HowAUnitReads::WaitingItsTurn
        );
    }

    #[test]
    fn a_stop_keeps_the_finished_stops_the_moving_and_never_arranges_the_rest() {
        let mut done = an_entry(b"/home/a.png", b"/jobs/a.png", 10);
        done.transfer = Some(an_identity(IdentityKind::Transfer));
        assert_eq!(
            what_a_stop_does(&done, Some(&Ending::Published)),
            WhatAStopDoesToAUnit::ItStays
        );
        let mut moving = an_entry(b"/home/b.png", b"/jobs/b.png", 10);
        moving.transfer = Some(an_identity(IdentityKind::Transfer));
        assert_eq!(
            what_a_stop_does(&moving, None),
            WhatAStopDoesToAUnit::StopTheTransfer
        );
        let waiting = an_entry(b"/home/c.png", b"/jobs/c.png", 10);
        assert_eq!(
            what_a_stop_does(&waiting, None),
            WhatAStopDoesToAUnit::NeverArrangeIt
        );
    }

    #[test]
    fn a_unit_outcome_is_immutable_and_carries_its_moment() {
        let mut entry = an_entry(b"/home/a.png", b"/jobs/a.png", 10);
        let at = SystemTime::UNIX_EPOCH + Duration::from_secs(1_000);
        entry
            .its_outcome_became(UnitOutcome::Done, at)
            .expect("the first outcome lands");
        assert_eq!(entry.outcome_written_at, Some(at));
        assert_eq!(
            entry.its_outcome_became(UnitOutcome::Failed, at + Duration::from_secs(1)),
            Err(Refused::ThatUnitHasAlreadyEnded)
        );
        assert_eq!(entry.outcome, Some(UnitOutcome::Done));
    }

    #[test]
    fn u9as_moment_is_the_only_moment_an_upload_record_holds() {
        // The request holds no moment of its own, and no moment of the run
        // ending: D6 writes nothing on exit, so there is no write to hang one
        // on. The check is structural — the entry's field is the only one.
        let request = a_request(vec![an_entry(b"/home/a.png", b"/jobs/a.png", 10)]);
        let held = format!("{request:?}");
        assert_eq!(held.matches("outcome_written_at").count(), 1);
    }

    fn an_interrupted_record(source: &str, final_path: &str) -> TransferRecord {
        let mut record = TransferRecord::arranged(
            an_identity(IdentityKind::Transfer),
            Direction::Up,
            an_identity(IdentityKind::LocalServer),
            Some(an_identity(IdentityKind::Installation)),
            source.to_string(),
            final_path.to_string(),
            "/jobs/.smoozy-staging",
            100,
            an_identity(IdentityKind::Run),
        );
        record.ending = Some(Ending::Interrupted);
        record
    }

    #[test]
    fn a_matching_unit_takes_up_the_interrupted_transfer_under_the_new_answer() {
        let record = an_interrupted_record("/home/a.png", "/jobs/a.png");
        let mut entry = an_entry(b"/home/a.png", b"/jobs/a.png", 100);
        entry.disposition = Some(Disposition::ToSend);
        assert_eq!(
            what_the_new_request_does(
                &entry,
                Comparison::CollidesWithAFile,
                std::slice::from_ref(&record)
            ),
            WhatTheNewRequestDoes::TakeUpTheInterruptedTransfer {
                transfer: record.transfer.clone(),
                publishing: Intent::ReplaceIntended,
            }
        );
        // The same material under a fresh landing name — the walk found nothing
        // — publishes create-intended.
        assert_eq!(
            what_the_new_request_does(
                &entry,
                Comparison::LandsFresh,
                std::slice::from_ref(&record)
            ),
            WhatTheNewRequestDoes::TakeUpTheInterruptedTransfer {
                transfer: record.transfer,
                publishing: Intent::CreateIntended,
            }
        );
    }

    #[test]
    fn skip_existing_never_takes_the_staging_up_and_keep_both_can_never_match() {
        let record = an_interrupted_record("/home/a.png", "/jobs/a.png");
        let mut skipped = an_entry(b"/home/a.png", b"/jobs/a.png", 100);
        skipped.disposition = Some(Disposition::SkipExisting);
        assert_eq!(
            what_the_new_request_does(
                &skipped,
                Comparison::CollidesWithAFile,
                std::slice::from_ref(&record)
            ),
            WhatTheNewRequestDoes::SkipItAndAbandonTheStaging
        );
        let mut kept = an_entry(b"/home/a.png", b"/jobs/a.png", 100);
        kept.disposition = Some(Disposition::RenamedUnderKeepBoth(
            b"/jobs/a (2).png".to_vec(),
        ));
        assert_eq!(
            what_the_new_request_does(&kept, Comparison::CollidesWithAFile, &[record]),
            WhatTheNewRequestDoes::AnOrdinaryFreshUnit
        );
    }

    #[test]
    fn a_leftover_needs_an_earlier_run_an_interrupted_record_and_this_installation() {
        let this_run = an_identity(IdentityKind::Run);
        let installation = an_identity(IdentityKind::Installation);
        let mut record = an_interrupted_record("/home/a.png", "/jobs/a.png");
        record.installation = Some(installation.clone());
        let mut entry = an_entry(b"/home/a.png", b"/jobs/a.png", 100);
        entry.disposition = Some(Disposition::ToSend);
        entry.transfer = Some(record.transfer.clone());
        let mut request = a_request(vec![entry]);
        request.installation = Some(installation.clone());

        // The run that opened it is not this run, so it is a leftover.
        let found = the_leftovers(
            std::slice::from_ref(&request),
            std::slice::from_ref(&record),
            &this_run,
            Some(&installation),
        );
        assert_eq!(found.len(), 1);
        assert_eq!(found[0].transfer, record.transfer);

        // The same request read by the run that opened it is not a leftover.
        let opened_by = request.opened_by_run.clone();
        assert!(the_leftovers(
            std::slice::from_ref(&request),
            std::slice::from_ref(&record),
            &opened_by,
            Some(&installation)
        )
        .is_empty());

        // A superseded installation disqualifies it.
        assert!(the_leftovers(
            std::slice::from_ref(&request),
            std::slice::from_ref(&record),
            &this_run,
            Some(&an_identity(IdentityKind::Installation))
        )
        .is_empty());
    }

    #[test]
    fn the_leftover_line_carries_no_byte_figure() {
        // Structural: the type has no byte field to fill in, so nothing can
        // relabel a durable candidate as disk use.
        let leftover = ALeftover {
            request: an_identity(IdentityKind::Operation),
            server_row: an_identity(IdentityKind::LocalServer),
            transfer: an_identity(IdentityKind::Transfer),
            landing_path: b"/jobs/a.png".to_vec(),
        };
        let shown = format!("{leftover:?}");
        assert!(!shown.contains("bytes"));
        const { assert!(THE_OFFLINE_LEFTOVER_LINE_CARRIES_NO_BYTE_TOTAL) };
    }

    #[test]
    fn the_reason_line_names_the_later_of_two_earlier_requests() {
        let this_run = an_identity(IdentityKind::Run);
        let server_row = an_identity(IdentityKind::LocalServer);
        let installation = an_identity(IdentityKind::Installation);
        let make = |landing: &[u8], at: SystemTime| {
            let mut entry = an_entry(b"/home/a.png", landing, 10);
            entry.disposition = Some(Disposition::ToSend);
            entry
                .its_outcome_became(UnitOutcome::Done, at)
                .expect("first outcome");
            // **A second unit the run's end caught mid-send**, so the request
            // is an interrupted one and not a completed one.
            let mut cut = an_entry(b"/home/cut.png", b"/jobs/cut.png", 10);
            cut.disposition = Some(Disposition::ToSend);
            let mut request = a_request(vec![entry, cut]);
            request.server_row = server_row.clone();
            request.installation = Some(installation.clone());
            request
        };
        let earlier = make(
            b"/jobs/a.png",
            SystemTime::UNIX_EPOCH + Duration::from_secs(100),
        );
        let later = make(
            b"/jobs/b.png",
            SystemTime::UNIX_EPOCH + Duration::from_secs(500),
        );
        let line = the_reason_line(
            &[earlier, later.clone()],
            &this_run,
            &server_row,
            Some(&installation),
            &[b"/jobs/a.png".to_vec(), b"/jobs/b.png".to_vec()],
        )
        .expect("a candidate exists");
        assert_eq!(line.the_earlier_request, later.record);
        assert_eq!(
            line.the_last_file_it_landed,
            SystemTime::UNIX_EPOCH + Duration::from_secs(500)
        );
    }

    #[test]
    fn a_landing_path_no_candidate_claims_takes_nothing_from_the_ones_that_do() {
        let this_run = an_identity(IdentityKind::Run);
        let server_row = an_identity(IdentityKind::LocalServer);
        let installation = an_identity(IdentityKind::Installation);
        let mut entry = an_entry(b"/home/a.png", b"/jobs/a.png", 10);
        entry.disposition = Some(Disposition::ToSend);
        entry
            .its_outcome_became(
                UnitOutcome::Done,
                SystemTime::UNIX_EPOCH + Duration::from_secs(42),
            )
            .expect("outcome");
        let mut cut = an_entry(b"/home/cut.png", b"/jobs/cut.png", 10);
        cut.disposition = Some(Disposition::ToSend);
        let mut request = a_request(vec![entry, cut]);
        request.server_row = server_row.clone();
        request.installation = Some(installation.clone());
        // The second occupied path was placed by the user's own hand and no
        // entry claims it.
        let line = the_reason_line(
            &[request],
            &this_run,
            &server_row,
            Some(&installation),
            &[b"/jobs/a.png".to_vec(), b"/jobs/by-hand.png".to_vec()],
        )
        .expect("the one candidate still answers");
        assert_eq!(
            line.the_last_file_it_landed,
            SystemTime::UNIX_EPOCH + Duration::from_secs(42)
        );
    }

    #[test]
    fn a_completed_request_and_a_stopped_one_are_never_the_reason_line() {
        let server_row = an_identity(IdentityKind::LocalServer);
        let installation = an_identity(IdentityKind::Installation);
        let mut entry = an_entry(b"/home/a.png", b"/jobs/a.png", 10);
        entry.disposition = Some(Disposition::ToSend);
        entry
            .its_outcome_became(UnitOutcome::Done, SystemTime::UNIX_EPOCH)
            .expect("outcome");
        let mut request = a_request(vec![entry]);
        request.server_row = server_row.clone();
        request.installation = Some(installation.clone());
        // Read by the run that opened it, the ending does not derive as *ended
        // by the run ending*, so it is no candidate.
        let its_own_run = request.opened_by_run.clone();
        assert!(the_reason_line(
            std::slice::from_ref(&request),
            &its_own_run,
            &server_row,
            Some(&installation),
            &[b"/jobs/a.png".to_vec()],
        )
        .is_none());
        // **Read by a later run it is still completed**, every unit having
        // ended in the run that opened it — U1 derives *completed* from the
        // unit outcomes before the run is asked. The pass before 0.1.9 on
        // Vast, new fault 2: an upload that finished, then a reopen, then the
        // same files again, read *This upload was interrupted*.
        assert!(the_reason_line(
            &[request],
            &an_identity(IdentityKind::Run),
            &server_row,
            Some(&installation),
            &[b"/jobs/a.png".to_vec()],
        )
        .is_none());
    }

    #[test]
    fn the_whole_counts_files_draws_bytes_and_reports_the_other_two_separately() {
        let mut to_send = an_entry(b"/home/a.png", b"/jobs/a.png", 100);
        to_send.disposition = Some(Disposition::ToSend);
        to_send.transfer = Some(an_identity(IdentityKind::Transfer));
        let mut done = an_entry(b"/home/b.png", b"/jobs/b.png", 200);
        done.disposition = Some(Disposition::ToSend);
        done.transfer = Some(an_identity(IdentityKind::Transfer));
        done.its_outcome_became(UnitOutcome::Done, SystemTime::UNIX_EPOCH)
            .expect("outcome");
        let mut skipped = an_entry(b"/home/c.png", b"/jobs/c.png", 4_000);
        skipped.disposition = Some(Disposition::SkipExisting);
        let mut handed = an_entry(b"/home/d.png", b"/jobs/d.png", 8_000);
        handed.disposition = Some(Disposition::HandedOver {
            the_act: an_identity(IdentityKind::Operation),
        });

        let transfer = to_send.transfer.clone().expect("a transfer");
        let request = a_request(vec![to_send, done, skipped, handed]);
        let whole = the_whole(
            &request,
            &[(
                transfer,
                Report {
                    bytes_staged: 40,
                    bytes_expected: 100,
                    started_over: None,
                    verifying_a_prefix_of: None,
                },
            )],
        );
        assert_eq!(whole.files_to_send, 2);
        assert_eq!(whole.files_done, 1);
        assert_eq!(whole.bytes_expected, 300);
        assert_eq!(whole.bytes_staged, 40);
        assert_eq!(whole.skipped_existing, 1);
        assert_eq!(whole.handed_over, 1);
        assert_eq!(whole.the_four_counts().to_send, 2);
    }

    #[test]
    fn the_whole_states_a_start_over_and_never_infers_it_from_the_bar() {
        let mut entry = an_entry(b"/home/a.png", b"/jobs/a.png", 100);
        entry.disposition = Some(Disposition::ToSend);
        entry.transfer = Some(an_identity(IdentityKind::Transfer));
        let transfer = entry.transfer.clone().expect("a transfer");
        let request = a_request(vec![entry]);
        let begun = Report {
            bytes_staged: 0,
            bytes_expected: 100,
            started_over: None,
            verifying_a_prefix_of: None,
        };
        let begun_again = Report {
            started_over: Some(StartOverCause::TheStagedPrefixDidNotVerify),
            ..begun.clone()
        };
        // The same figure, two different reports: only the mark tells them
        // apart, which is why nothing reads the bar's direction.
        assert_eq!(begun.bytes_staged, begun_again.bytes_staged);
        assert!(!the_whole(&request, &[(transfer.clone(), begun)]).a_file_started_over);
        let after = the_whole(&request, &[(transfer, begun_again)]);
        assert!(after.a_file_started_over);
        assert_eq!(after.files_done, 0, "the file count does not move");
    }

    #[test]
    fn a_handed_unit_keeps_its_entry_takes_no_transfer_and_names_the_act() {
        let mut entry = an_entry(b"/home/a.png", b"/jobs/a.png", 10);
        entry.disposition = Some(Disposition::ToSend);
        let mut request = a_request(vec![entry]);
        let mut keeper = in_a_list::AList::default();
        let act = an_identity(IdentityKind::Operation);
        hand_over(
            &mut request,
            b"/jobs/a.png",
            act.clone(),
            SystemTime::UNIX_EPOCH,
            &mut keeper,
        )
        .expect("the hand-over is written");
        assert_eq!(
            request.entries.len(),
            1,
            "the enumeration is never shortened"
        );
        assert_eq!(
            request.entries[0].disposition,
            Some(Disposition::HandedOver {
                the_act: act.clone()
            })
        );
        assert_eq!(
            request.entries[0].outcome,
            Some(UnitOutcome::NotAttempted {
                stopped_by: act.as_text()
            })
        );
        assert!(request.entries[0].transfer.is_none());
        assert!(ends_completed_having_arranged_nothing(&request));
    }

    #[test]
    fn what_crosses_is_four_things_and_carries_the_class_without_reading_it() {
        let mut entry = an_entry(b"/home/a.png", b"/jobs/a.png", 10);
        entry.disposition = Some(Disposition::ToSend);
        let request = a_request(vec![entry.clone()]);
        // The class's type is the caller's — W1a is step 7's — and this stands
        // in for it without this crate composing one.
        let crossing = what_crosses(&request, &entry, "placed");
        assert_eq!(crossing.source_path, b"/home/a.png".to_vec());
        assert_eq!(crossing.occupied_landing_path, b"/jobs/a.png".to_vec());
        assert_eq!(crossing.server_row, request.server_row);
        assert_eq!(crossing.installation, request.installation);
        assert_eq!(crossing.the_class_w1a_read, "placed");
        // Nothing of the request's own state crosses.
        let shown = format!("{crossing:?}");
        assert!(!shown.contains("destination"));
        assert!(!shown.contains("collision"));
    }

    #[test]
    fn the_priority_class_is_the_one_declared_and_not_a_second_spelling() {
        assert_eq!(
            the_priority_class(),
            crate::declarations::THE_TRANSFER_PRIORITY_CLASS
        );
    }

    /// **U9's condition is the spec's whole sentence, not half of it.**
    ///
    /// *Skip existing* is put forward only where Smoozy holds the earlier
    /// request **and can say why**, so the function takes the reason line itself
    /// — a flag would let a caller put the answer forward with no reason beside
    /// it, which is the case the *and* rules out.
    #[test]
    fn the_answer_led_with_is_skip_existing_only_where_there_is_a_reason_to_say() {
        let line = TheReasonLine {
            the_earlier_request: an_identity(IdentityKind::Operation),
            the_last_file_it_landed: a_moment(),
        };
        assert_eq!(
            the_answer_to_lead_with(Some(&line)),
            Some(CollisionAnswer::SkipExisting),
            "U9's own sentence"
        );
        // **U9b's negative**: where no candidate entry exists, nothing is
        // supplied — and there is no way to ask for one without the reason.
        assert_eq!(the_answer_to_lead_with(None), None);
    }

    #[test]
    fn stop_stops_the_request_and_writes_no_disposition() {
        let mut request = a_request(vec![
            an_entry(b"/home/a.png", b"/jobs/a.png", 10),
            an_entry(b"/home/b.png", b"/jobs/b.png", 20),
        ]);
        let mut keeper = in_a_list::AList::default();
        let comparisons = answers_for(
            &request,
            &[Comparison::CollidesWithAFile, Comparison::LandsFresh],
        );
        let did = apply_the_collision_answer(
            &mut request,
            &comparisons,
            CollisionAnswer::Stop,
            &mut std::iter::empty(),
            &mut keeper,
            a_moment(),
        )
        .expect("the answer is written");
        assert_eq!(did, WhatTheAnswerDid::TheRequestStops);
        assert!(request
            .entries
            .iter()
            .all(|entry| entry.disposition.is_none()));
        assert_eq!(request.collision_answer, Some(CollisionAnswer::Stop));
        // The answer is durable all the same: it is the one field that cannot
        // be rebuilt.
        assert_eq!(
            keeper.written[0].collision_answer,
            Some(CollisionAnswer::Stop)
        );
        let run = request.opened_by_run.clone();
        assert_eq!(
            request.how_it_ended(&run, true),
            Some(HowARequestEnded::StoppedByTheUser)
        );
    }

    #[test]
    fn a_request_ended_by_the_run_ending_has_no_line_to_write() {
        let mut request = a_request(vec![an_entry(b"/home/a.png", b"/jobs/a.png", 10)]);
        request.entries[0].disposition = Some(Disposition::ToSend);
        let another_run = an_identity(IdentityKind::Run);
        assert!(request.ended_by_the_run_ending(&another_run));
        assert_eq!(request.how_it_ended(&another_run, false), None);
        assert_eq!(
            request.how_it_ended(&another_run, true),
            None,
            "the run ending wins: there is no write to hang a line on"
        );
    }

    #[test]
    fn a_request_completes_only_when_every_unit_it_arranged_has_ended() {
        let mut request = a_request(vec![
            an_entry(b"/home/a.png", b"/jobs/a.png", 10),
            an_entry(b"/home/b.png", b"/jobs/b.png", 20),
        ]);
        let run = request.opened_by_run.clone();
        assert_eq!(request.how_it_ended(&run, false), None, "unanswered");
        request.entries[0].disposition = Some(Disposition::ToSend);
        request.entries[1].disposition = Some(Disposition::SkipExisting);
        assert_eq!(request.how_it_ended(&run, false), None, "one unit is owed");
        request.entries[0]
            .its_outcome_became(UnitOutcome::Done, SystemTime::UNIX_EPOCH)
            .expect("outcome");
        assert_eq!(
            request.how_it_ended(&run, false),
            Some(HowARequestEnded::Completed)
        );
    }

    #[test]
    fn a_start_over_cause_is_read_where_it_arrives() {
        assert_eq!(
            a_start_over_cause_is_read_not_composed(StartOverCause::TheStagedPrefixDidNotVerify),
            StartOverCause::TheStagedPrefixDidNotVerify
        );
    }

    #[test]
    fn the_user_removing_the_record_removes_it_and_nothing_else() {
        let request = a_request(vec![an_entry(b"/home/a.png", b"/jobs/a.png", 10)]);
        let other = a_request(vec![an_entry(b"/home/b.png", b"/jobs/b.png", 10)]);
        let mut keeper = in_a_list::AList::default();
        keeper.write_it_down(&request).expect("written");
        keeper.write_it_down(&other).expect("written");
        keeper
            .the_user_removed_it(&request.record)
            .expect("removed");
        assert_eq!(keeper.written.len(), 1);
        assert_eq!(keeper.written[0].record, other.record);
    }

    #[test]
    fn intent_is_the_other_documents_own_type_and_a_skipped_unit_has_none() {
        let mut request = a_request(vec![an_entry(b"/home/a.png", b"/jobs/a.png", 10)]);
        request.collision_answer = Some(CollisionAnswer::Replace);
        let mut entry = an_entry(b"/home/a.png", b"/jobs/a.png", 10);
        entry.disposition = Some(Disposition::ToSend);
        assert_eq!(
            intent_for(&request, &entry, Comparison::CollidesWithAFile),
            Some(Intent::ReplaceIntended)
        );
        assert_eq!(
            intent_for(&request, &entry, Comparison::LandsFresh),
            Some(Intent::CreateIntended)
        );
        entry.disposition = Some(Disposition::RenamedUnderKeepBoth(
            b"/jobs/a (2).png".to_vec(),
        ));
        assert_eq!(
            intent_for(&request, &entry, Comparison::CollidesWithAFile),
            Some(Intent::CreateIntended)
        );
        entry.disposition = Some(Disposition::SkipExisting);
        assert_eq!(
            intent_for(&request, &entry, Comparison::CollidesWithAFile),
            None
        );
    }

    #[test]
    fn an_upload_request_opens_without_any_policy() {
        assert!(open_an_upload_request().is_ok());
    }
}
