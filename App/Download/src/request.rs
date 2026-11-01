//! The manual request — `Download` §8, M1 to M17 with M11a and M13a, Rows 202,
//! 203, 205, 208, 209, 299, 411, 414, 415 and 417.
//!
//! **Constraint:** Row 202 — *do what can be done, and say what can't* — with
//! the question asked once, before a byte moves, and Row 209's carry-on across
//! a drop.
//!
//! **The durable half is one `State, identity and recovery` §9.4 outcome
//! record and this crate holds no store of its own.** The shape is here, the
//! store is `Desktop app and delivery` §5's — the same split `File Browser`
//! U1 already draws in the other direction, taken rather than re-invented.
//! [`KeepsTheRequest`] is the seam and [`crate::arriving_later`] names the real
//! end.
//!
//! **Markers used below that belong to another document, each with the
//! document it belongs to.** `Server protocol and transfers` S9, S12, S13,
//! S15, S24, S26, S27, S28, C11, C16, C17, C19, C23, G4, G4a, G4b, G6, G7.
//! `State, identity and recovery` §8.6, §9.4, §11.1. `File Browser` K8,
//! O7, U1, U4, U5, U8, U11, X5, X6, X7, X9, X10, X12.
//! `Desktop app and delivery` C3, C10, C11, C13, C14, C20, F10, F10a.
//! `Servers` S10.
//!
//! **A marker this plan also defines is deliberately not listed above,
//! whichever document is meant, and is written out with its document at every
//! site instead** — the header list cannot tell the two apart, and a bare one
//! would resolve to a real and wrong clause. `File Browser` N2 and
//! `Job files and outputs` N3 are the two that arise here.

use std::time::SystemTime;

use smoozy_file_browser::already_exists::CollisionAnswer;
use smoozy_server_protocol::transfers::{Ending, Report, StartOverCause, TransferRecord};
use smoozy_state::identities::Identity;
use smoozy_state::outcomes::{Refused, UnitOutcome};

use crate::fetching::WhichRefusalItWas;
use crate::vouching::AVouchingMark;

/// **M1's enumeration entry — one per file, and every field is M1's own.**
///
/// The entry is what makes the request *one thing* while the unit stays the
/// file, and **nothing about it is re-derived from the machine** once the
/// enumeration is written.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EnumerationEntry {
    /// **The source path on the machine as listed** — names carried as the
    /// machine's bytes, `File Browser` N2's discipline in the other direction.
    pub source_path: Vec<u8>,
    /// Its landing path on the user's own computer.
    pub landing_path: Vec<u8>,
    /// **Its size as enumerated.** The bar's byte total is drawn from these
    /// and from `Server protocol and transfers` G4's per-transfer counts, and
    /// **it is never re-read to keep a total fresh.**
    pub size_as_enumerated: u64,
    /// **Its refusal, where the name check refused the landing path.** A
    /// refused entry is *cannot land*, out of both totals and reported
    /// separately.
    pub refusal: Option<WhichRefusalItWas>,
    /// **Its vouching mark**, from the local job records alone.
    pub vouching: AVouchingMark,
    /// **Its disposition after the collision answer.** Absent until the answer
    /// is applied, which happens once.
    pub disposition: Option<Disposition>,
    /// **Its transfer identity once a transfer is arranged.** Absent for a
    /// unit no transfer was arranged for.
    pub transfer: Option<Identity>,
    /// **Its unit outcome as it becomes true**, and `State, identity and
    /// recovery` §8.6 makes it immutable once written.
    pub outcome: Option<UnitOutcome>,
}

impl EnumerationEntry {
    /// An entry as the walk's enumeration gives it, before any answer.
    pub fn enumerated(
        source_path: Vec<u8>,
        landing_path: Vec<u8>,
        size_as_enumerated: u64,
        vouching: AVouchingMark,
    ) -> EnumerationEntry {
        EnumerationEntry {
            source_path,
            landing_path,
            size_as_enumerated,
            refusal: None,
            vouching,
            disposition: None,
            transfer: None,
            outcome: None,
        }
    }

    /// **`State, identity and recovery` §8.6 kept here as well as at the
    /// store**: a unit outcome is immutable, so a second write is refused
    /// rather than taken.
    pub fn its_outcome_became(&mut self, outcome: UnitOutcome) -> Result<(), Refused> {
        if self.outcome.is_some() {
            return Err(Refused::ThatUnitHasAlreadyEnded);
        }
        self.outcome = Some(outcome);
        Ok(())
    }
}

/// **The dispositions the collision answer gives an entry, applied once to the
/// enumeration.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Disposition {
    ToFetch,
    /// The user answered **Skip existing** and this entry collided.
    SkipExisting,
    /// **M8's composed name**, computed once when the answer is applied so a
    /// resume re-derives nothing. The value is the fresh landing path.
    RenamedUnderKeepBoth(Vec<u8>),
    /// The user answered **Replace** and this entry collided. **The one
    /// deliberate replacement in this document**, published with the replacing
    /// rename and taking no reservation.
    Replace,
    /// **The name check refused the landing path**, so no transfer is ever
    /// arranged for it.
    CannotLand(WhichRefusalItWas),
}

impl Disposition {
    /// **M16: which units are in the whole's two totals.** The count is unit
    /// outcomes over the units a transfer is arranged for, and the bar is
    /// bytes over those same units. **Skipped, refused, left-alone and
    /// cannot-land units are in neither total and reported separately.**
    pub fn a_transfer_is_arranged(&self) -> bool {
        matches!(
            self,
            Disposition::ToFetch | Disposition::RenamedUnderKeepBoth(_) | Disposition::Replace
        )
    }
}

/// **M1: a manual download request is one multi-part action with one
/// `State, identity and recovery` §9.4 outcome record in the local record,
/// user-held, written when the user confirms the request and before any
/// transfer is arranged.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AManualRequest {
    /// The §9.4 record's own operation identity.
    pub record: Identity,
    pub server_row: Identity,
    pub installation: Option<Identity>,
    /// **The destination** — the standing folder, or the per-request override
    /// the spec grants.
    pub destination: Vec<u8>,
    /// **The enumeration: one entry per file, complete before the collision
    /// question is asked and durable before the first byte moves.**
    pub entries: Vec<EnumerationEntry>,
    /// **The collision answer, written durably at the moment it is given and
    /// never re-asked for the life of the request.**
    pub collision_answer: Option<CollisionAnswer>,
    /// **M4's recorded refusals**, per subtree, which are counted in neither
    /// total and reported as what could not be read.
    pub subtrees_that_could_not_be_read: Vec<ARecordedRefusal>,
    /// **M13's run marker**, carried on the request as it is on a transfer
    /// record: which application run opened it.
    pub opened_by_run: Identity,
}

impl AManualRequest {
    /// **M1's write, as a value.** Nothing here writes it down.
    pub fn confirmed(
        record: Identity,
        server_row: Identity,
        installation: Option<Identity>,
        destination: Vec<u8>,
        entries: Vec<EnumerationEntry>,
        opened_by_run: Identity,
    ) -> AManualRequest {
        AManualRequest {
            record,
            server_row,
            installation,
            destination,
            entries,
            collision_answer: None,
            subtrees_that_could_not_be_read: Vec::new(),
            opened_by_run,
        }
    }

    /// **M1: the request's own ending, derived from the unit outcomes and the
    /// run, never a second stored field.**
    ///
    /// `None` while the request is still alive.
    pub fn how_it_ended(
        &self,
        this_run: &Identity,
        the_user_stopped_it: bool,
    ) -> Option<HowARequestEnded> {
        if self.ended_by_the_run_ending(this_run) {
            // M13: the activity ended with the application run.
            return Some(HowARequestEnded::EndedByTheRunEnding);
        }
        if the_user_stopped_it {
            return Some(HowARequestEnded::StoppedByTheUser);
        }
        // **An enumeration with no units and no refusals is complete the
        // moment the request is confirmed.** M1 derives the request's ending
        // from the unit outcomes, and for no units that is *nothing failed*.
        // **A picked folder with no files in it is the case**: without this it
        // could never reach *completed* at all, so the row sat open for ever
        // over a request that had nothing left to do. **The folder still
        // arrives as a folder** (M9), which is the adapter's act at the
        // enumeration.
        if self.entries.is_empty() {
            return self
                .subtrees_that_could_not_be_read
                .is_empty()
                .then_some(HowARequestEnded::Completed);
        }
        if self.entries.iter().any(|entry| entry.disposition.is_none()) {
            return None;
        }
        let every_unit_ended = self.entries.iter().all(|entry| {
            entry.outcome.is_some()
                || !entry
                    .disposition
                    .as_ref()
                    .is_some_and(Disposition::a_transfer_is_arranged)
        });
        every_unit_ended.then_some(HowARequestEnded::Completed)
    }

    /// **M13: closing Smoozy ends the request**, read off the run marker
    /// alone.
    pub fn ended_by_the_run_ending(&self, this_run: &Identity) -> bool {
        &self.opened_by_run != this_run
    }

    /// **M16: the whole, counted in files and drawn in bytes, both totals
    /// existing before the first transfer starts.**
    pub fn the_whole(&self) -> TheRequestsWhole {
        let in_the_totals: Vec<&EnumerationEntry> = self
            .entries
            .iter()
            .filter(|entry| {
                entry
                    .disposition
                    .as_ref()
                    .is_some_and(Disposition::a_transfer_is_arranged)
            })
            .collect();
        TheRequestsWhole {
            files: in_the_totals.len(),
            bytes: in_the_totals
                .iter()
                .map(|entry| entry.size_as_enumerated)
                .sum(),
        }
    }
}

/// **M1: how a request ended.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HowARequestEnded {
    Completed,
    StoppedByTheUser,
    /// **M13: closing Smoozy ends the request.**
    EndedByTheRunEnding,
}

/// **M16 and M17: the request's whole, in files and in bytes.**
///
/// **M17: while a deep folder is still being enumerated there is no whole, and
/// the reading says so rather than showing a number that will move.** The
/// enumeration is a reading and reports underway; the totals appear when it
/// completes, **which is before the collision question and therefore always
/// before the first byte** — so no state exists in which transfers run under a
/// whole still being discovered.
///
/// **The byte total is drawn from the enumeration's listed sizes, which are
/// apparent sizes**; the subsystem route carries no allocated-block count, so
/// a sparse file's bar is drawn against its claimed size, **and that limit is
/// declared rather than papered over.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TheRequestsWhole {
    pub files: usize,
    pub bytes: u64,
}

/// **M17: there is no whole while the enumeration is still running.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TheWholesReading {
    TheEnumerationIsUnderway,
    The(TheRequestsWhole),
}

/// **M2 and M3: a refusal the enumeration met, named at the component it
/// arises at.**
///
/// **All three failures — *may not read it*, *gone*, *the machine did not
/// answer* — arrive as distinct protocol statuses in one request**
/// (`File Browser` P1). **This clause used to say the stat case gave one
/// byte-identical answer and to put that in front of the user**; it was
/// reading the transfer client program's rendering rather than the protocol's
/// reply.
///
/// **What the component walk adds is *where*: `File Browser` P3 is adopted as
/// it stands**, components opened in turn from the deepest listable ancestor
/// and the refusal named at the component it arises at, with `File Browser`
/// `File Browser` P4's residue message where the walk cannot reach that far. **Adopted, not
/// adapted**: it is the same problem on the same limit, and a second wording
/// would drift.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ARecordedRefusal {
    /// The path of the branch that ended here.
    pub at: Vec<u8>,
    /// Which of `File Browser` P1's three it was, as that document's own
    /// value.
    pub what_smoozy_met: smoozy_file_browser::permission::WhatSmoozyMet,
}

/// **M4: a folder that refuses readdir ends its branch with a recorded
/// refusal, and the enumeration is still complete.**
///
/// **The declaration: the enumeration's unit is the whole question, and it is
/// complete when every branch has ended in a listing or a recorded per-entry
/// refusal** — so the totals are known and honest, the refused subtrees are
/// counted in neither total and reported as what could not be read, **and no
/// count is wrong in the quiet direction.**
pub fn the_enumeration_is_complete(
    every_branch_ended_in_a_listing_or_a_recorded_refusal: bool,
) -> bool {
    every_branch_ended_in_a_listing_or_a_recorded_refusal
}

/// **M5: there is no elevated download, and the reason is stated so nobody
/// reaches for one.**
///
/// Elevation runs on the command channel and never inside the subsystem
/// (`File Browser` P6); the bytes of a fetch move on a subsystem channel
/// authenticated as the login; and **copying a refused file to somewhere
/// readable would change the machine, which downloading never does.** The
/// refusal is reported as it stands; the elevated *listing* remains
/// `File Browser`'s where the user goes looking.
pub const THERE_IS_NO_ELEVATED_DOWNLOAD: bool = true;

/// **M6: the three things a pre-flight can have to put in front of the
/// user.**
///
/// **The pre-flight is put in front of the user exactly where it has something
/// to put there**, and **a request whose complete enumeration finds none of
/// the three starts its transfers without a question, and no empty surface is
/// composed for it.**
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct WhatThePreFlightHasToSay {
    /// A collision to answer.
    pub collisions: usize,
    /// A cannot-vouch mark to name for the go-ahead.
    pub cannot_vouch_for: usize,
    /// A refusal to say.
    pub refusals: usize,
}

impl WhatThePreFlightHasToSay {
    /// **M6, narrowed 2026-08-29.** This clause read unconditionally until a
    /// scenario replay played a clean whole-job download through it and met a
    /// confirmation with nothing in it; **the spec conditions the stop in
    /// terms** — *"If any of the files it is bringing down already exist where
    /// they would go, it stops and asks"* — and the condition is carried here.
    pub fn the_request_stops_and_asks(&self) -> bool {
        self.collisions > 0 || self.cannot_vouch_for > 0 || self.refusals > 0
    }

    /// **M6: the headline is how many of the request already exist.**
    pub fn the_headline(&self) -> usize {
        self.collisions
    }
}

/// **M6, composed over the enumeration and the recorded refusals.**
///
/// **The cannot-vouch files and the refusals are named beside the same
/// question, so the user answers once knowing all of it.**
pub fn what_the_pre_flight_has_to_say(
    request: &AManualRequest,
    the_colliding_landing_paths: &[Vec<u8>],
) -> WhatThePreFlightHasToSay {
    WhatThePreFlightHasToSay {
        collisions: request
            .entries
            .iter()
            .filter(|entry| the_colliding_landing_paths.contains(&entry.landing_path))
            .count(),
        cannot_vouch_for: request
            .entries
            .iter()
            .filter(|entry| entry.vouching == AVouchingMark::CannotBeVouchedFor)
            .count(),
        refusals: request.subtrees_that_could_not_be_read.len()
            + request
                .entries
                .iter()
                .filter(|entry| entry.refusal.is_some())
                .count(),
    }
}

/// **M6: the already-exists comparison is by name and place only, file by
/// file, against the destination's listings read through the platform file
/// adapter with names as stored on disk — byte comparison, no folding
/// modelled.**
///
/// **The staging directory and every reservation-record path are subtracted
/// first.** The staging half mirrors `File Browser` X10 — *"a staging path is
/// never a landing place and never compared as one"* — **and not X12**, which
/// makes no subtraction and says the opposite about visibility: the staging
/// directory is deliberately shown, because hiding it would make the listing
/// lie. **The reservation half has no upload counterpart to mirror at all**:
/// `File Browser` X6 records that the exclusive-create reservation route was
/// weighed and declined on that side.
pub fn what_the_comparison_reads(
    the_destinations_listing: &[Vec<u8>],
    the_staging_directory: &[u8],
    the_reservation_record_paths: &[Vec<u8>],
) -> Vec<Vec<u8>> {
    the_destinations_listing
        .iter()
        .filter(|path| !it_is_the_staging_directory_or_inside_it(path, the_staging_directory))
        .filter(|path| {
            !the_reservation_record_paths
                .iter()
                .any(|reserved| reserved == *path)
        })
        .cloned()
        .collect()
}

/// **M6's staging subtraction: the reserved staging directory itself, and
/// everything inside it.**
///
/// **The test is component-wise and never a byte prefix.** A byte prefix says
/// a folder of the user's called `.smoozy-staging-old` is inside
/// `.smoozy-staging`, so an ordinary folder of theirs would be subtracted from
/// the comparison and its collisions never reported. **The directory itself is
/// subtracted as well as its contents**, because `File Browser` X10's rule is
/// that a staging path is never a landing place and never compared as one, and
/// the directory is not a landing place either.
fn it_is_the_staging_directory_or_inside_it(path: &[u8], the_staging_directory: &[u8]) -> bool {
    path == the_staging_directory || crate::landing::is_inside(path, the_staging_directory)
}

/// **M6: the four answers are the spec's — Replace, Keep both, Skip existing,
/// Stop — given once and applied to the enumeration once, dispositions per
/// file.**
///
/// **M8: Keep both renames the whole arriving top-level thing and nothing
/// inside it merges** — the spec's rule. The composed name is the original
/// with the first non-colliding counted suffix, judged like any other name
/// (`Desktop app and delivery` C3, C20), bounded per component in the
/// receiving system's own unit with the stem trimmed at a boundary of that
/// unit; **publication of everything under it stays create-intended, so a
/// composed name that still lands on something is refused rather than written
/// over.** Replace touches just the colliding files; **a folder onto a
/// same-named folder merges file by file.**
/// **P8 rides in front of every answer: a name the check refused is *cannot
/// land* whatever the user answered.** No transfer is ever arranged for it, so
/// a caller applying the answer plainly cannot fetch a refused name.
pub fn the_disposition(
    answer: CollisionAnswer,
    the_entrys_refusal: Option<WhichRefusalItWas>,
    it_collides: bool,
    the_kept_both_name: Option<Vec<u8>>,
) -> Option<Disposition> {
    if let Some(refusal) = the_entrys_refusal {
        return Some(Disposition::CannotLand(refusal));
    }
    if !it_collides {
        return Some(Disposition::ToFetch);
    }
    match answer {
        CollisionAnswer::Replace => Some(Disposition::Replace),
        CollisionAnswer::SkipExisting => Some(Disposition::SkipExisting),
        CollisionAnswer::KeepBoth => the_kept_both_name.map(Disposition::RenamedUnderKeepBoth),
        // **Stop is the request not proceeding at all**, so no entry takes a
        // disposition from it.
        CollisionAnswer::Stop => None,
    }
}

/// **M6's other reading, and the crate's own seam for it: what is standing in
/// one folder of the user's own.**
///
/// **The far side is `Desktop app and delivery`'s platform file adapter** —
/// the same wall every other local act in this crate meets — and **no
/// implementation of this trait exists outside this crate's own tests**
/// ([`crate::arriving_later`]).
pub trait ReadsTheDestinationsListing {
    /// **The whole paths standing in one folder, with names as stored on
    /// disk** — byte comparison, no folding modelled, which is M6's own rule.
    /// A folder that is not there answers nothing, which is the ordinary case
    /// for a fresh destination.
    fn what_is_standing_in(&mut self, the_folder: &[u8]) -> Vec<Vec<u8>>;
}

/// **M8's composed name: the original with the first non-colliding counted
/// suffix, judged like any other name.**
///
/// **The counting starts at two** — the thing already standing is the first —
/// and the first suffix that does not collide is the answer. **The name is
/// bounded per component in the receiving system's own unit
/// (`Desktop app and delivery` C5) with the stem trimmed at a boundary of that
/// unit**, so the suffix is never what is cut; and **the composed name is
/// judged again like any other** (`Desktop app and delivery` C20), which is
/// what stops a composed name being one the destination will not take.
///
/// **Nothing is returned where no composed name is acceptable**, and that is a
/// value rather than a fallback: the caller has no fresh name to land at, and
/// inventing one would be Smoozy writing a name it has not judged.
///
/// **The suffix goes before a file's extension** — `scene.blend` becomes
/// `scene (2).blend` — **and at the end of a folder's name**, by `File
/// Browser`'s own `split` (`File Browser` X9), so one rule names a kept copy in
/// both directions. Step 10c's thirteenth-pass fix piece 3. *(This appended the
/// suffix to the whole name, so a download's copy landed as `p13-cube.blend
/// (2)`, which is no longer a `.blend` — while an upload's landed as `p13-cube
/// (2).blend`.)*
pub fn the_kept_both_name<R: crate::fetching::TheReceivingSystemsRules + ?Sized>(
    rules: &R,
    the_original_name: &[u8],
    is_a_folder: bool,
    the_names_already_standing: &[Vec<u8>],
) -> Option<Vec<u8>> {
    let smoozy_file_browser::keep_both::Split { stem, rest } = smoozy_file_browser::keep_both::split(
        &smoozy_file_browser::listing::EntryName::from_the_machine(the_original_name.to_vec()),
        is_a_folder,
    );
    // **The suffix bounds the stem and never the other way round.** A stem
    // trimmed to fit is still the user's name shortened; a suffix trimmed to
    // fit is a name that collides again. **What follows the stem is kept
    // whole too**, for the same reason: an extension trimmed is a file of
    // another kind.
    for counted in 2..=1000u32 {
        let suffix = format!(" ({counted})").into_bytes();
        let room = rules
            .the_longest_component()
            .checked_sub(rules.length_in_this_systems_own_unit(&suffix))?
            .checked_sub(rules.length_in_this_systems_own_unit(&rest))?;
        let stem = trim_to(rules, &stem, room);
        if stem.is_empty() {
            return None;
        }
        let mut composed = stem;
        composed.extend_from_slice(&suffix);
        composed.extend_from_slice(&rest);
        if the_names_already_standing.contains(&composed) {
            continue;
        }
        // C20: the composed name is judged again like any other.
        if rules.check(&composed).is_acceptable_as_given() {
            return Some(composed);
        }
        return None;
    }
    None
}

/// Trim a name to the room left, **in the receiving system's own unit**, and
/// at a character boundary where the bytes decode — never mid-character.
fn trim_to<R: crate::fetching::TheReceivingSystemsRules + ?Sized>(
    rules: &R,
    name: &[u8],
    room: usize,
) -> Vec<u8> {
    if rules.length_in_this_systems_own_unit(name) <= room {
        return name.to_vec();
    }
    let mut at = name.len();
    while at > 0 {
        at -= 1;
        if let Ok(text) = std::str::from_utf8(name) {
            if !text.is_char_boundary(at) {
                continue;
            }
        }
        if rules.length_in_this_systems_own_unit(&name[..at]) <= room {
            return name[..at].to_vec();
        }
    }
    Vec::new()
}

/// **M6 and M8: one collision answer applied to a whole enumeration, once.**
///
/// **The four answers are given once and applied to the enumeration once,
/// dispositions per file**, and this is that application. Nothing re-asks and
/// nothing re-derives a collision.
///
/// **Keep both renames the whole arriving top-level thing and nothing inside
/// it merges**, which is the spec's own rule — so the unit here is the
/// component directly under the destination, not the file. A top-level thing
/// with any colliding file beneath it is renamed whole, and every landing path
/// under it moves with it; one with none is untouched. **Replace touches just
/// the colliding files**, and a folder onto a same-named folder merges file by
/// file, which falls out of the landing paths being unchanged.
///
/// **A refused name is *cannot land* whatever the answer was** (P8), and it is
/// counted in neither total.
///
/// **Stop is the request not proceeding at all**, so no entry takes a
/// disposition and the enumeration is left as it stands.
pub fn apply_the_collision_answer<R: crate::fetching::TheReceivingSystemsRules + ?Sized>(
    rules: &R,
    request: &mut AManualRequest,
    answer: CollisionAnswer,
    the_colliding_landing_paths: &[Vec<u8>],
    the_names_already_standing: &[Vec<u8>],
) {
    request.collision_answer = Some(answer);
    if matches!(answer, CollisionAnswer::Stop) {
        return;
    }
    // **M8's unit: the top-level thing arriving under the destination.**
    let mut renamed: Vec<(Vec<u8>, Vec<u8>)> = Vec::new();
    if matches!(answer, CollisionAnswer::KeepBoth) {
        for top in the_arriving_top_level_things(request) {
            let collides = request.entries.iter().any(|entry| {
                entry.refusal.is_none()
                    && the_colliding_landing_paths.contains(&entry.landing_path)
                    && the_top_level_thing_of(&entry.landing_path, &request.destination)
                        .is_some_and(|its| its == top)
            });
            if !collides {
                continue;
            }
            // **A file arrives at the top level where one of its landing paths
            // is the top-level thing itself**; anything else under it makes
            // it a folder.
            let is_a_folder = !request.entries.iter().any(|entry| {
                let destination = crate::landing::components(&request.destination);
                let path = crate::landing::components(&entry.landing_path);
                path.len() == destination.len() + 1
                    && path[..destination.len()] == destination[..]
                    && path[destination.len()] == top.as_slice()
            });
            let Some(fresh) =
                the_kept_both_name(rules, &top, is_a_folder, the_names_already_standing)
            else {
                continue;
            };
            renamed.push((top, fresh));
        }
    }

    for at in 0..request.entries.len() {
        let refusal = request.entries[at].refusal;
        let it_collides = the_colliding_landing_paths.contains(&request.entries[at].landing_path);
        let fresh_landing =
            the_top_level_thing_of(&request.entries[at].landing_path, &request.destination)
                .and_then(|top| {
                    renamed.iter().find(|(original, _)| *original == top).map(
                        |(original, fresh)| {
                            the_landing_path_under_a_fresh_top(
                                &request.entries[at].landing_path,
                                &request.destination,
                                original,
                                fresh,
                            )
                        },
                    )
                });
        // **Keep both's disposition carries the fresh landing path**, and it
        // is only a Keep both where this top-level thing was actually renamed.
        let it_collides = it_collides || fresh_landing.is_some();
        request.entries[at].disposition =
            the_disposition(answer, refusal, it_collides, fresh_landing.clone());
        if let Some(fresh) = fresh_landing {
            request.entries[at].landing_path = fresh;
        }
    }
}

/// The distinct top-level things a request's landing paths put under the
/// destination, in the order they were first met.
fn the_arriving_top_level_things(request: &AManualRequest) -> Vec<Vec<u8>> {
    let mut out: Vec<Vec<u8>> = Vec::new();
    for entry in &request.entries {
        let Some(top) = the_top_level_thing_of(&entry.landing_path, &request.destination) else {
            continue;
        };
        if !out.contains(&top) {
            out.push(top);
        }
    }
    out
}

/// The one component a landing path puts directly under the destination, **by
/// components and never by a byte prefix.**
pub(crate) fn the_top_level_thing_of(landing_path: &[u8], the_destination: &[u8]) -> Option<Vec<u8>> {
    let destination = crate::landing::components(the_destination);
    let path = crate::landing::components(landing_path);
    (path.len() > destination.len() && path[..destination.len()] == destination[..])
        .then(|| path[destination.len()].to_vec())
}

/// One landing path with its top-level thing renamed, **component by
/// component**, so nothing inside it moves.
fn the_landing_path_under_a_fresh_top(
    landing_path: &[u8],
    the_destination: &[u8],
    the_original_top: &[u8],
    the_fresh_top: &[u8],
) -> Vec<u8> {
    let destination = crate::landing::components(the_destination);
    let path = crate::landing::components(landing_path);
    let mut out = the_destination.to_vec();
    if !out.ends_with(b"/") {
        out.push(b'/');
    }
    out.extend_from_slice(the_fresh_top);
    for component in path.iter().skip(destination.len() + 1) {
        out.push(b'/');
        out.extend_from_slice(component);
    }
    debug_assert_eq!(path[destination.len()], the_original_top);
    out
}

/// **M7: a folding near-miss the byte comparison cannot see is caught at
/// publication, not modelled at the pre-flight.**
///
/// The exclusive create refuses it and the truth is said per file — the same
/// shape `File Browser` X5/X7 declared on the other side, and `Desktop app and
/// delivery` C11's own reasoning: **the filesystem is asked, nothing is
/// modelled.**
pub const A_FOLDING_NEAR_MISS_IS_CAUGHT_AT_PUBLICATION_AND_NEVER_MODELLED: bool = true;

/// **M9: a downloaded folder arrives as a folder**, landing folders created
/// through the adapter and a folder that exists merging per the spec — **and a
/// whole-job download the user starts by hand lands on that job's own folder
/// under the composed landing name, colliding only with what an earlier
/// download of the same job left there.**
pub const A_DOWNLOADED_FOLDER_ARRIVES_AS_A_FOLDER: bool = true;

/// **M10: Row 208's answer is `Server protocol and transfers` §7 entire, and
/// the two constraints that pull against each other are both its clauses.**
///
/// **Every part-arrived byte sits at the staging path and never at the final
/// name** — so *Skip existing*, comparing by name and place only, can never
/// count a partial as present — **and the staged material of an interrupted
/// transfer is kept with its checkpoints** — so Row 209's carry-on has
/// something to carry on from.
///
/// **The one thing standing at the final name is the zero-byte reservation,
/// which is an entry and never arrived bytes**, subtracted from every
/// enumeration and completed or removed on the next run by
/// `Desktop app and delivery` C13.
pub const A_PARTIAL_NEVER_SITS_AT_THE_FINAL_NAME_AND_IS_NEVER_COUNTED_AS_PRESENT: bool = true;

/// **M11 and M11a: how a row composes while the line is down, and the two
/// waiting states inside it.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HowTheRowComposes {
    /// **Waiting**, with the transfer's own bytes still staged: this is the
    /// resume of `Server protocol and transfers` S12 step 4 — **what already
    /// crossed stays crossed, and the retained figure is the truthful one.**
    WaitingAndTheFigureIsTruthful,
    /// **Waiting, and the transfer started over**: the bytes are gone and the
    /// figure is zero or climbing. **The row reads `Server protocol and
    /// transfers` G4a's mark and never the figure's direction** — a row that
    /// inferred *this is a pause* from a held figure would state the opposite
    /// of the truth on exactly the two cases S12 step 5 and S13 exist for.
    WaitingAndItStartedOver(StartOverCause),
    /// **Inside the resume state, a staged prefix being verified carries
    /// `Server protocol and transfers` G4b's second mark and the row reads
    /// that one too.** The transfer is neither waiting on the connection nor
    /// stalled, and **the figure will not move until the first byte above the
    /// checkpoint is written** — about twelve minutes on a nearly complete
    /// 4 GiB prefix at that document's measured read-back rate, which is why a
    /// mark rather than the figure's stillness is what the row reads here too.
    VerifyingAStagedPrefixOf(u64),
    /// **Not waiting.** A record reading *stopped* never composes as waiting,
    /// which excludes by construction the two endings Smoozy writes itself:
    /// the user's Stop, and a left-alone publication refusal.
    NotWaiting,
}

/// **M11: a manual download interrupted by a drop carries on where `Servers`
/// repairs the connection, without the user asking again.**
///
/// The two connection labels decide and are read, never re-derived. **While
/// the line is down the interrupted files compose as waiting** — record
/// *interrupted*, request open, period standing.
///
/// **A changed-source difference is neither of those, because it is not an
/// ending at all.** `Server protocol and transfers` S13 says so in terms:
/// *"That is a start-over and not an ending"* — so the record stays
/// *interrupted* while the start-over runs, **and the file still composes as
/// waiting**, which is the case Row 209 exists for.
///
/// **An integrity failure is not written *stopped* here either**:
/// `Server protocol and transfers` S24 gives a digest mismatch its own
/// vocabulary, which is none of S26's three, and that gap in that document's
/// closed set is a seam recorded at §15 item 20 rather than a row this
/// document fills by assignment.
///
/// **That is `File Browser` K8/U5's composition, answered the same way in this
/// direction, word for word where it matters: a transfer Smoozy stops itself
/// records *stopped* and never *interrupted*, so it never composes as waiting
/// and is never re-established at the next repair.**
///
/// **Nothing new is recorded on this side.** The marks are the transfer's
/// report and are period-held like the count; no local record gains a field.
pub fn how_the_row_composes(
    the_records_ending: Option<&Ending>,
    the_request_is_open: bool,
    report: &Report,
) -> HowTheRowComposes {
    if !the_request_is_open {
        return HowTheRowComposes::NotWaiting;
    }
    match the_records_ending {
        // **Only *interrupted* composes as waiting** —
        // `Server protocol and transfers` S26b's own rule, consumed. **M11's
        // waiting state is *record interrupted, request open, period
        // standing*, and all three halves are asked**: a record with **no
        // ending at all** is a transfer that is moving bytes right now, and
        // describing it as waiting for the connection states the opposite of
        // what is happening.
        Some(ending) if ending.may_the_work_be_carried_on() => {
            if let Some(prefix) = report.verifying_a_prefix_of {
                return HowTheRowComposes::VerifyingAStagedPrefixOf(prefix);
            }
            match report.started_over {
                Some(cause) => HowTheRowComposes::WaitingAndItStartedOver(cause),
                None => HowTheRowComposes::WaitingAndTheFigureIsTruthful,
            }
        }
        _ => HowTheRowComposes::NotWaiting,
    }
}

/// **M12: the user's Stop acts on the whole request, and a single transfer can
/// also be stopped.**
///
/// The whole-request Stop is `Server protocol and transfers` S26's deliberate
/// stop: the record written *stopped* before the channel is closed
/// (`Server protocol and transfers` C23), **finished files stay, unarranged
/// files are never arranged, the one in flight is stopped**, staged material
/// removed at once or owed under S27.
///
/// **A single transfer can also be stopped, and that is the same act on one
/// unit**: the record written *stopped*, the staged material removed at once
/// or owed, **the rest of the request untouched**.
///
/// **Both controls exist and nothing in this mechanism moved** — Design ruled
/// on 2026-08-13 that no per-file control existed and the Creator reversed it
/// on 2026-08-20, the spec having granted it since 2026-07-22. **Where each
/// sits is Design's.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhatAStopReaches {
    TheWholeRequest,
    OneUnit,
}

/// **M12, as one statement of what a stop leaves.**
pub fn what_a_stop_leaves(reaches: WhatAStopReaches) -> WhatAStopLeaves {
    WhatAStopLeaves {
        finished_files_stay: true,
        unarranged_files_are_never_arranged: matches!(reaches, WhatAStopReaches::TheWholeRequest),
        the_rest_of_the_request_is_untouched: matches!(reaches, WhatAStopReaches::OneUnit),
    }
}

/// What a stop leaves behind, whichever of M12's two acts it was.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WhatAStopLeaves {
    pub finished_files_stay: bool,
    pub unarranged_files_are_never_arranged: bool,
    pub the_rest_of_the_request_is_untouched: bool,
}

/// **M13: closing Smoozy ends the request, and what a fresh sitting finds is
/// bounded on both sides** — the leftover discipline, answered the same way as
/// `File Browser` U8 **because the two directions must not disagree about a
/// fresh sitting**.
///
/// - Channels die with the process; every in-flight record reads *interrupted*
///   (`State, identity and recovery` D6 makes a crash and a clean
///   close identical); **nothing restarts itself** — reopening Smoozy does not
///   quietly resume a manual download, and the only automatic resumption
///   anywhere is `Server protocol and transfers` C11 step 4, which excludes
///   earlier-run records by the run marker S9 carries.
/// - **The user is told the leftover is there and can remove it** — records
///   with unfinished units and *interrupted* transfer records under the
///   current installation identity present in the transfer window as
///   leftovers, with a remove act; **removing a leftover removes its staged
///   material and closes the record.**
pub fn it_presents_as_a_leftover(
    request: &AManualRequest,
    this_run: &Identity,
    the_current_installation: &Option<Identity>,
) -> bool {
    request.ended_by_the_run_ending(this_run)
        && &request.installation == the_current_installation
        && request.entries.iter().any(|entry| {
            entry.outcome.is_none()
                && entry
                    .disposition
                    .as_ref()
                    .is_some_and(Disposition::a_transfer_is_arranged)
        })
}

/// **M13's third bullet: whether a fresh, user-started download may take up an
/// interrupted record's staged material.**
///
/// **Where a new request's enumeration matches an interrupted record's machine
/// path and landing path, that unit takes up the interrupted transfer** — its
/// identity, its record, its staging path — and proceeds through
/// `Server protocol and transfers` S12 entire plus S13's four-value
/// comparison; **where either gate says no, the unit starts fresh and the old
/// staging is abandoned to S28's desktop-side sweep.**
///
/// **The taken-up unit is governed by the *new* request's disposition and
/// collision answer throughout** — *skip existing* does not take it up at all,
/// *replace* publishes replace-intended, an ordinary send publishes
/// create-intended, **and Keep both can never match because its landing path
/// is fresh.**
pub fn may_take_up_the_staged_material(
    record: &TransferRecord,
    the_new_entrys_source_path: &[u8],
    the_new_entrys_landing_path: &[u8],
    disposition: &Disposition,
) -> bool {
    if matches!(
        disposition,
        Disposition::SkipExisting | Disposition::CannotLand(_)
    ) {
        return false;
    }
    record.source_path.as_bytes() == the_new_entrys_source_path
        && record.final_path.as_bytes() == the_new_entrys_landing_path
}

/// **M14: an auto-download arrangement meets the same leftover question, and
/// the answer is one answer for both directions.**
///
/// Its earlier-run interrupted records are never resumed automatically either;
/// **the resuming arrangement's fresh fetch of the same file takes them up
/// through the same two gates.**
pub const ONE_ANSWER_SERVES_BOTH_DIRECTIONS: bool = true;

/// **M13a: local staging whose transfer record is gone has one bounded,
/// explicit route.**
///
/// After `Server protocol and transfers` S28's record-driven sweep at
/// application start, `Download` asks the platform file adapter to enumerate
/// the reserved staging directory inside each destination folder **still named
/// by an auto-download arrangement or a manual request record**. An entry for
/// which no transfer record supplies the exact staging path is presented as
/// **unowned download staging**, naming the destination path and offering
/// **Remove**.
///
/// **Enumeration is never ownership evidence and removes nothing.** Remove is
/// the user's explicit act on that named entry: the adapter re-reads the exact
/// path, **verifies that the entry itself is still inside the reserved staging
/// directory**, and removes that entry **without following it outside the
/// directory**; a refusal is reported as an ordinary failed removal. **The act
/// creates no transfer record, writes no transfer outcome and infers no
/// publication or failure from the orphan's contents.**
///
/// **The reach is stated rather than inflated**: where no surviving
/// arrangement or request record names the destination folder, Smoozy has no
/// bounded place to look, **presents nothing, and does not claim the orphan is
/// absent.** It remains an ordinary visible entry in the user's own folder
/// until the user removes it with their file manager.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WhatALocalStagingEntryIs {
    /// A transfer record supplies this exact staging path.
    ATransferRecordNamesIt,
    /// **Unowned download staging**, with the destination path to name.
    UnownedDownloadStaging { destination: Vec<u8> },
}

/// **M13a's classification, over the records this crate already holds.**
pub fn what_a_local_staging_entry_is(
    the_staging_path: &[u8],
    destination: &[u8],
    the_transfer_records: &[TransferRecord],
) -> WhatALocalStagingEntryIs {
    let named = the_transfer_records.iter().any(|record| {
        record
            .every_staging_path_it_has_named()
            .iter()
            .any(|path| path.as_bytes() == the_staging_path)
    });
    if named {
        WhatALocalStagingEntryIs::ATransferRecordNamesIt
    } else {
        WhatALocalStagingEntryIs::UnownedDownloadStaging {
            destination: destination.to_vec(),
        }
    }
}

/// **M13a's bound: which folders are looked in at all.**
///
/// Only the reserved staging directory inside each destination folder **still
/// named by a surviving arrangement or request record**. **Nothing searches
/// anywhere else and nothing claims an orphan elsewhere is absent.**
pub fn the_folders_that_may_be_looked_in(
    the_destinations_arrangements_still_name: &[Vec<u8>],
    the_destinations_requests_still_name: &[Vec<u8>],
) -> Vec<Vec<u8>> {
    let mut out: Vec<Vec<u8>> = Vec::new();
    for destination in the_destinations_arrangements_still_name
        .iter()
        .chain(the_destinations_requests_still_name)
    {
        let staging = crate::fetching::the_staging_directory_inside(destination);
        if !out.contains(&staging) {
            out.push(staging);
        }
    }
    out
}

/// **M13a: the removal never follows an entry outside the reserved staging
/// directory.**
///
/// **The test is component-wise and never a byte prefix**, and this is a
/// deletion bound rather than a reading: a byte prefix says
/// `/Users/a/Downloads/.smoozy-staging-old/keep_this` is inside
/// `/Users/a/Downloads/.smoozy-staging`, so the adapter would be asked to
/// remove something standing in an ordinary folder of the user's own.
///
/// **The directory itself is not inside itself**, which is the boundary M13a
/// wants: the act is on an entry the reading named, and the reserved directory
/// is not one of them.
pub fn the_removal_may_run(the_entry: &[u8], the_reserved_staging_directory: &[u8]) -> bool {
    crate::landing::is_inside(the_entry, the_reserved_staging_directory)
}

/// **M15: J9 and J10 are the answer** — the reading, the marks, the saying at
/// the pre-flight before any transfer, and the honest silence where Smoozy has
/// no standing to judge.
///
/// **What the standing itself is — a frame caught part-written, a write that
/// ran out of room — stays `What a job writes`' and is not retaken.**
pub const THE_VOUCHING_READING_IS_SECTION_FOURS_AND_IS_NOT_RETAKEN_HERE: bool = true;

/// **What a manual download request is called in `State, identity and
/// recovery` §9.4's outcome record, and it is §12's own word.**
///
/// M1 makes the request *one multi-part action with one §9.4 outcome record*,
/// and §9.4 names an action by the owning module's own words. **They are
/// §12's**, read off [`crate::declarations::THE_PARTITION_UNITS`] rather than
/// spelled a second time here — a name in two places is the one fault this
/// stage keeps paying for.
pub const THE_ACTION: &str = "a manual download request";

/// **What a unit of a manual download request is called — §12's own words, and
/// it is a role rather than a value.**
///
/// §12 declares the partition unit as *one file's transfer*, and **the role is
/// those words, the same for every unit of one request**; §9.4's own ordering
/// is what tells two units apart. **So a store writes its units by position and
/// never by role** — a write by role would reach an arbitrary one of a
/// job's frames.
pub fn the_role_every_unit_carries() -> &'static str {
    crate::declarations::the_partition_unit_of(THE_ACTION)
        .expect("§12 declares a manual download request, and `declarations` holds every row")
        .unit
}

/// **The durable half — the shape is this document's and the store is
/// `Desktop app and delivery` §5's.**
///
/// **The real end is `smoozy_desktop::record::download_records`**, written at
/// step 10c piece 10; before it no implementation existed outside this crate's
/// own tests ([`crate::arriving_later`]).
pub trait KeepsTheRequest {
    /// **M1's write**, and `Server protocol and transfers` S8's ordering
    /// applies to it: the enumeration is durable before the first byte moves.
    fn write_it_down(&mut self, request: &AManualRequest) -> Result<(), Refused>;

    /// Every request record held for a server row. **Read at a fresh
    /// sitting.**
    fn every_request_for(&self, server_row: &Identity) -> Result<Vec<AManualRequest>, Refused>;

    /// **`State, identity and recovery` §11.1: user-held.** It ends when the
    /// user removes it and never otherwise — no age, no cap, no sweep.
    fn the_user_removed_it(&mut self, record: &Identity) -> Result<(), Refused>;
}

/// **M1: the moment a unit outcome was written, where the store keeps one.**
///
/// It is the desktop's own clock and is used for presentation and for nothing
/// a promise rests on.
pub type WhenAnOutcomeWasWritten = SystemTime;

#[cfg(test)]
mod tests {
    use super::*;
    use smoozy_server_protocol::transfers::{Direction, StoppedReason};
    use smoozy_state::identities::IdentityKind;

    fn an_identity(kind: IdentityKind) -> Identity {
        Identity::issue(kind).expect("the desk has randomness")
    }

    fn an_entry(source: &[u8], landing: &[u8], size: u64) -> EnumerationEntry {
        EnumerationEntry::enumerated(
            source.to_vec(),
            landing.to_vec(),
            size,
            AVouchingMark::NoClaimIsMade,
        )
    }

    fn a_request(entries: Vec<EnumerationEntry>, run: Identity) -> AManualRequest {
        AManualRequest::confirmed(
            an_identity(IdentityKind::Operation),
            an_identity(IdentityKind::LocalServer),
            Some(an_identity(IdentityKind::Installation)),
            b"/Users/a/Downloads".to_vec(),
            entries,
            run,
        )
    }

    /// **M1 and ruling 6: a download of an empty folder finishes.**
    ///
    /// The request's ending is derived from the unit outcomes, and for no
    /// units that is *nothing failed*. **Without this it could never reach
    /// *completed* at all** — the two earlier returns fire, so such a request
    /// could still end by the run ending or by the user stopping it, but the
    /// row otherwise sat open for ever over a request with nothing left to do.
    #[test]
    fn a_download_of_an_empty_folder_is_complete_the_moment_it_is_confirmed() {
        let this_run = an_identity(IdentityKind::Run);
        let request = a_request(Vec::new(), this_run.clone());
        assert_eq!(
            request.how_it_ended(&this_run, false),
            Some(HowARequestEnded::Completed)
        );
        // The whole is nothing in both units, which is honest rather than
        // absent.
        assert_eq!(request.the_whole().files, 0);
        assert_eq!(request.the_whole().bytes, 0);
    }

    /// **An enumeration that found nothing because every branch refused is not
    /// complete**: it has something to say and nothing was read.
    #[test]
    fn an_empty_enumeration_with_a_recorded_refusal_does_not_complete_on_its_own() {
        let this_run = an_identity(IdentityKind::Run);
        let mut request = a_request(Vec::new(), this_run.clone());
        request
            .subtrees_that_could_not_be_read
            .push(ARecordedRefusal {
                at: b"/jobs/abc".to_vec(),
                what_smoozy_met: smoozy_file_browser::permission::WhatSmoozyMet::Refused,
            });
        assert_eq!(request.how_it_ended(&this_run, false), None);
        // The user's own Stop still ends it, as it ends any request.
        assert_eq!(
            request.how_it_ended(&this_run, true),
            Some(HowARequestEnded::StoppedByTheUser)
        );
    }

    fn a_report(
        staged: u64,
        started_over: Option<StartOverCause>,
        verifying: Option<u64>,
    ) -> Report {
        Report {
            bytes_staged: staged,
            bytes_expected: 4096,
            started_over,
            verifying_a_prefix_of: verifying,
        }
    }

    #[test]
    fn a_clean_request_with_nothing_to_say_starts_without_a_question() {
        let request = a_request(
            vec![an_entry(b"/j/out/a.png", b"/Users/a/Downloads/a.png", 10)],
            an_identity(IdentityKind::Run),
        );
        let nothing = what_the_pre_flight_has_to_say(&request, &[]);
        assert!(!nothing.the_request_stops_and_asks());
        assert_eq!(nothing, WhatThePreFlightHasToSay::default());
    }

    #[test]
    fn each_of_the_three_reasons_alone_puts_the_pre_flight_up() {
        let run = an_identity(IdentityKind::Run);
        // A collision.
        let request = a_request(
            vec![an_entry(b"/j/out/a.png", b"/Users/a/Downloads/a.png", 10)],
            run.clone(),
        );
        let said =
            what_the_pre_flight_has_to_say(&request, &[b"/Users/a/Downloads/a.png".to_vec()]);
        assert!(said.the_request_stops_and_asks());
        assert_eq!(said.the_headline(), 1);

        // A cannot-vouch mark alone.
        let mut marked = an_entry(b"/j/out/a.png", b"/Users/a/Downloads/a.png", 10);
        marked.vouching = AVouchingMark::CannotBeVouchedFor;
        let request = a_request(vec![marked], run.clone());
        let said = what_the_pre_flight_has_to_say(&request, &[]);
        assert!(said.the_request_stops_and_asks());
        assert_eq!(said.the_headline(), 0);

        // A refusal alone.
        let mut request = a_request(
            vec![an_entry(b"/j/out/a.png", b"/Users/a/Downloads/a.png", 10)],
            run,
        );
        request
            .subtrees_that_could_not_be_read
            .push(ARecordedRefusal {
                at: b"/j/out/private".to_vec(),
                what_smoozy_met: smoozy_file_browser::permission::WhatSmoozyMet::Refused,
            });
        assert!(what_the_pre_flight_has_to_say(&request, &[]).the_request_stops_and_asks());
    }

    #[test]
    fn the_comparison_subtracts_the_staging_directory_and_every_reservation_path_first() {
        let listing = vec![
            b"/Users/a/Downloads/a.png".to_vec(),
            b"/Users/a/Downloads/.smoozy-staging/xyz".to_vec(),
            b"/Users/a/Downloads/b.png".to_vec(),
        ];
        let read = what_the_comparison_reads(
            &listing,
            b"/Users/a/Downloads/.smoozy-staging",
            &[b"/Users/a/Downloads/b.png".to_vec()],
        );
        assert_eq!(read, vec![b"/Users/a/Downloads/a.png".to_vec()]);
    }

    /// **The staging subtraction is component-wise too**, so a folder of the
    /// user's whose name merely begins with the reserved one keeps its place
    /// in the comparison — and its collisions are still reported.
    ///
    /// The directory itself stays subtracted, which is `File Browser` X10's
    /// rule that a staging path is never compared as a landing place.
    #[test]
    fn the_subtraction_keeps_a_folder_that_merely_begins_with_the_reserved_name() {
        let listing = vec![
            b"/Users/a/Downloads/.smoozy-staging".to_vec(),
            b"/Users/a/Downloads/.smoozy-staging/xyz".to_vec(),
            b"/Users/a/Downloads/.smoozy-staging-old/keep_this".to_vec(),
        ];
        let read = what_the_comparison_reads(&listing, b"/Users/a/Downloads/.smoozy-staging", &[]);
        assert_eq!(
            read,
            vec![b"/Users/a/Downloads/.smoozy-staging-old/keep_this".to_vec()]
        );
    }

    #[test]
    fn the_four_answers_apply_once_and_stop_gives_no_entry_a_disposition() {
        assert_eq!(
            the_disposition(CollisionAnswer::Replace, None, true, None),
            Some(Disposition::Replace)
        );
        assert_eq!(
            the_disposition(CollisionAnswer::SkipExisting, None, true, None),
            Some(Disposition::SkipExisting)
        );
        assert_eq!(
            the_disposition(
                CollisionAnswer::KeepBoth,
                None,
                true,
                Some(b"/Users/a/Downloads/a (2).png".to_vec())
            ),
            Some(Disposition::RenamedUnderKeepBoth(
                b"/Users/a/Downloads/a (2).png".to_vec()
            ))
        );
        assert_eq!(
            the_disposition(CollisionAnswer::Stop, None, true, None),
            None
        );
        // A file that collides with nothing is fetched whatever the answer was.
        for answer in [
            CollisionAnswer::Replace,
            CollisionAnswer::SkipExisting,
            CollisionAnswer::KeepBoth,
            CollisionAnswer::Stop,
        ] {
            assert_eq!(
                the_disposition(
                    answer,
                    false.then_some(WhichRefusalItWas::NotRepresentable),
                    false,
                    None
                ),
                Some(Disposition::ToFetch)
            );
            // **P8 rides in front of every answer**: a name the check refused
            // is *cannot land* whatever the user answered, so no caller
            // applying the answer plainly can fetch a refused name.
            assert_eq!(
                the_disposition(
                    answer,
                    Some(WhichRefusalItWas::TooLongInThisSystemsOwnUnit),
                    true,
                    None
                ),
                Some(Disposition::CannotLand(
                    WhichRefusalItWas::TooLongInThisSystemsOwnUnit
                ))
            );
        }
    }

    /// **M8: the composed name is the original with the first non-colliding
    /// counted suffix, judged like any other name.**
    #[test]
    fn keep_both_composes_the_first_counted_suffix_that_does_not_collide() {
        let rules = crate::fetching::tests::ASystem::plain();
        assert_eq!(
            the_kept_both_name(&rules, b"abc night", true, &[]),
            Some(b"abc night (2)".to_vec())
        );
        // The first one that is free is the answer, and the counting starts at
        // two because the thing already standing is the first.
        assert_eq!(
            the_kept_both_name(
                &rules,
                b"abc night",
                true,
                &[b"abc night (2)".to_vec(), b"abc night (3)".to_vec()]
            ),
            Some(b"abc night (4)".to_vec())
        );
    }

    /// **Keep both names a file's copy before its extension and a folder's at
    /// the end of its name** — `File Browser` X9's one rule, by that crate's
    /// own `split`, so an upload and a download name a kept copy alike. Step
    /// 10c's thirteenth-pass fix piece 3: the thirteenth pass's copy landed as
    /// `p13-cube.blend (2)`, which is no longer a `.blend`.
    #[test]
    fn keep_both_names_a_file_before_its_extension_and_a_folder_at_its_end() {
        let rules = crate::fetching::tests::ASystem::plain();
        assert_eq!(
            the_kept_both_name(&rules, b"a.blend", false, &[]),
            Some(b"a (2).blend".to_vec())
        );
        assert_eq!(
            the_kept_both_name(&rules, b"a.blend", false, &[b"a (2).blend".to_vec()]),
            Some(b"a (3).blend".to_vec())
        );
        assert_eq!(the_kept_both_name(&rules, b"f", true, &[]), Some(b"f (2)".to_vec()));
        assert_eq!(
            the_kept_both_name(&rules, b"shot.v2", true, &[]),
            Some(b"shot.v2 (2)".to_vec()),
            "a folder has no extension to keep"
        );
        // **The extension is kept whole and the stem is what is trimmed.**
        let short = crate::fetching::tests::ASystem {
            the_longest: 16,
            refuses: vec![b'/', 0],
            transforms_a_trailing: Vec::new(),
        };
        let composed = the_kept_both_name(&short, b"a long name.blend", false, &[])
            .expect("a name fits at sixteen");
        assert_eq!(composed, b"a long (2).blend".to_vec());
    }

    /// **The stem is trimmed at a boundary of the receiving system's own unit,
    /// and the suffix is never what is cut** — a trimmed suffix would collide
    /// again.
    #[test]
    fn the_stem_is_trimmed_to_fit_and_the_counted_suffix_is_kept_whole() {
        let rules = crate::fetching::tests::ASystem {
            the_longest: 12,
            refuses: vec![b'/', 0],
            transforms_a_trailing: Vec::new(),
        };
        let composed = the_kept_both_name(&rules, b"a very long name indeed", true, &[])
            .expect("a name fits at twelve");
        assert_eq!(composed.len(), 12);
        assert!(composed.ends_with(b" (2)"));
    }

    /// **Nothing is returned where no composed name is acceptable**, which is
    /// a value rather than a fallback: Smoozy never lands at a name it has not
    /// judged.
    #[test]
    fn no_name_is_composed_where_the_system_refuses_the_composed_one() {
        struct RefusesEverySuffix;
        impl crate::fetching::TheReceivingSystemsRules for RefusesEverySuffix {
            fn check(&self, component: &[u8]) -> crate::fetching::TheNameCheckSaid {
                if component.ends_with(b")") {
                    crate::fetching::TheNameCheckSaid::NotRepresentable
                } else {
                    crate::fetching::TheNameCheckSaid::TheNameMayBeWrittenAsGiven
                }
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
        assert_eq!(the_kept_both_name(&RefusesEverySuffix, b"a", false, &[]), None);
    }

    /// **M8: Keep both renames the whole arriving top-level thing and nothing
    /// inside it merges.**
    ///
    /// One file of the arriving job folder collides, so the folder is renamed
    /// whole and every landing path under it moves with it. **The unit is the
    /// top-level thing and never the file**, which is the spec's own rule.
    #[test]
    fn keep_both_renames_the_whole_arriving_top_level_thing_and_moves_everything_under_it() {
        let run = an_identity(IdentityKind::Run);
        let mut request = a_request(
            vec![
                an_entry(
                    b"/j/abc night/a.png",
                    b"/Users/a/Downloads/abc night/a.png",
                    10,
                ),
                an_entry(
                    b"/j/abc night/beauty/0001.exr",
                    b"/Users/a/Downloads/abc night/beauty/0001.exr",
                    20,
                ),
                an_entry(b"/j/loose.png", b"/Users/a/Downloads/loose.png", 30),
            ],
            run,
        );

        apply_the_collision_answer(
            &crate::fetching::tests::ASystem::plain(),
            &mut request,
            CollisionAnswer::KeepBoth,
            // Only the one file collides; the whole folder is renamed for it.
            &[b"/Users/a/Downloads/abc night/a.png".to_vec()],
            &[b"abc night".to_vec()],
        );

        assert_eq!(
            request.entries[0].landing_path,
            b"/Users/a/Downloads/abc night (2)/a.png".to_vec()
        );
        assert_eq!(
            request.entries[1].landing_path,
            b"/Users/a/Downloads/abc night (2)/beauty/0001.exr".to_vec()
        );
        // The file that arrived beside the folder is untouched and is fetched.
        assert_eq!(
            request.entries[2].landing_path,
            b"/Users/a/Downloads/loose.png".to_vec()
        );
        assert_eq!(request.entries[2].disposition, Some(Disposition::ToFetch));
        assert!(matches!(
            request.entries[1].disposition,
            Some(Disposition::RenamedUnderKeepBoth(_))
        ));
        assert_eq!(request.collision_answer, Some(CollisionAnswer::KeepBoth));
    }

    /// **Replace touches just the colliding files**, and every other answer is
    /// applied per file the same way.
    #[test]
    fn replace_and_skip_touch_only_the_colliding_files_and_a_refused_name_cannot_land() {
        let run = an_identity(IdentityKind::Run);
        let mut refused = an_entry(b"/j/bad", b"/Users/a/Downloads/bad", 1);
        refused.refusal = Some(WhichRefusalItWas::NotRepresentable);
        let entries = vec![
            an_entry(b"/j/a.png", b"/Users/a/Downloads/a.png", 10),
            an_entry(b"/j/b.png", b"/Users/a/Downloads/b.png", 20),
            refused,
        ];
        let colliding = vec![b"/Users/a/Downloads/a.png".to_vec()];

        let mut request = a_request(entries.clone(), run.clone());
        apply_the_collision_answer(
            &crate::fetching::tests::ASystem::plain(),
            &mut request,
            CollisionAnswer::Replace,
            &colliding,
            &[],
        );
        assert_eq!(request.entries[0].disposition, Some(Disposition::Replace));
        assert_eq!(request.entries[1].disposition, Some(Disposition::ToFetch));
        // **P8: a refused name is *cannot land* whatever the answer was.**
        assert_eq!(
            request.entries[2].disposition,
            Some(Disposition::CannotLand(WhichRefusalItWas::NotRepresentable))
        );

        let mut request = a_request(entries.clone(), run.clone());
        apply_the_collision_answer(
            &crate::fetching::tests::ASystem::plain(),
            &mut request,
            CollisionAnswer::SkipExisting,
            &colliding,
            &[],
        );
        assert_eq!(
            request.entries[0].disposition,
            Some(Disposition::SkipExisting)
        );
        assert_eq!(request.entries[1].disposition, Some(Disposition::ToFetch));

        // **Stop is the request not proceeding at all**, so no entry takes a
        // disposition.
        let mut request = a_request(entries, run);
        apply_the_collision_answer(
            &crate::fetching::tests::ASystem::plain(),
            &mut request,
            CollisionAnswer::Stop,
            &colliding,
            &[],
        );
        assert!(request
            .entries
            .iter()
            .all(|entry| entry.disposition.is_none()));
        assert_eq!(request.collision_answer, Some(CollisionAnswer::Stop));
    }

    /// **M6: the destination's listing is read through a seam of this crate's
    /// own, in the same shape as its other adapter seams.**
    ///
    /// The comparison itself is [`what_the_comparison_reads`]'s and is not
    /// taken a second time; what the seam adds is the one route to the
    /// reading.
    #[test]
    fn the_destinations_listing_is_read_through_the_crates_own_seam() {
        struct AFolder(Vec<Vec<u8>>);
        impl ReadsTheDestinationsListing for AFolder {
            fn what_is_standing_in(&mut self, _the_folder: &[u8]) -> Vec<Vec<u8>> {
                self.0.clone()
            }
        }
        let mut folder = AFolder(vec![
            b"/Users/a/Downloads/a.png".to_vec(),
            b"/Users/a/Downloads/.smoozy-staging/xyz".to_vec(),
        ]);
        let standing = folder.what_is_standing_in(b"/Users/a/Downloads");
        assert_eq!(
            what_the_comparison_reads(&standing, b"/Users/a/Downloads/.smoozy-staging", &[]),
            vec![b"/Users/a/Downloads/a.png".to_vec()]
        );
    }

    #[test]
    fn the_whole_counts_only_the_units_a_transfer_is_arranged_for() {
        let run = an_identity(IdentityKind::Run);
        let mut entries = vec![
            an_entry(b"/j/out/a.png", b"/Users/a/Downloads/a.png", 100),
            an_entry(b"/j/out/b.png", b"/Users/a/Downloads/b.png", 200),
            an_entry(b"/j/out/c.png", b"/Users/a/Downloads/c.png", 400),
        ];
        entries[0].disposition = Some(Disposition::ToFetch);
        entries[1].disposition = Some(Disposition::SkipExisting);
        entries[2].disposition = Some(Disposition::CannotLand(WhichRefusalItWas::NotRepresentable));
        let request = a_request(entries, run);
        assert_eq!(
            request.the_whole(),
            TheRequestsWhole {
                files: 1,
                bytes: 100
            }
        );
    }

    #[test]
    fn a_request_ends_when_every_arranged_unit_has_ended_and_not_before() {
        let run = an_identity(IdentityKind::Run);
        let mut entries = vec![
            an_entry(b"/j/out/a.png", b"/Users/a/Downloads/a.png", 100),
            an_entry(b"/j/out/b.png", b"/Users/a/Downloads/b.png", 200),
        ];
        entries[0].disposition = Some(Disposition::ToFetch);
        entries[1].disposition = Some(Disposition::SkipExisting);
        let mut request = a_request(entries, run.clone());
        assert_eq!(request.how_it_ended(&run, false), None);
        request.entries[0]
            .its_outcome_became(UnitOutcome::Done)
            .expect("first write");
        assert_eq!(
            request.how_it_ended(&run, false),
            Some(HowARequestEnded::Completed)
        );
        // §8.6: a unit outcome is immutable.
        assert!(request.entries[0]
            .its_outcome_became(UnitOutcome::Failed)
            .is_err());
    }

    #[test]
    fn closing_smoozy_ends_the_request_and_the_leftover_is_told() {
        let opened_by = an_identity(IdentityKind::Run);
        let this_run = an_identity(IdentityKind::Run);
        let mut entries = vec![an_entry(b"/j/out/a.png", b"/Users/a/Downloads/a.png", 100)];
        entries[0].disposition = Some(Disposition::ToFetch);
        let request = a_request(entries, opened_by);
        assert_eq!(
            request.how_it_ended(&this_run, false),
            Some(HowARequestEnded::EndedByTheRunEnding)
        );
        assert!(it_presents_as_a_leftover(
            &request,
            &this_run,
            &request.installation.clone()
        ));
        // A different installation's leftover is not this installation's.
        assert!(!it_presents_as_a_leftover(
            &request,
            &this_run,
            &Some(an_identity(IdentityKind::Installation))
        ));
    }

    #[test]
    fn a_row_reads_the_mark_and_never_the_figures_direction() {
        // Both waiting states carry the same open request and the same
        // interrupted record; only the mark separates them.
        let holding = a_report(3_000_000, None, None);
        assert_eq!(
            how_the_row_composes(Some(&Ending::Interrupted), true, &holding),
            HowTheRowComposes::WaitingAndTheFigureIsTruthful
        );
        let started_over = a_report(
            0,
            Some(StartOverCause::TheSourceChangedUnderADownload),
            None,
        );
        assert_eq!(
            how_the_row_composes(Some(&Ending::Interrupted), true, &started_over),
            HowTheRowComposes::WaitingAndItStartedOver(
                StartOverCause::TheSourceChangedUnderADownload
            )
        );
        // A held figure with no mark is the resume, whatever the figure does.
        let held_at_zero = a_report(0, None, None);
        assert_eq!(
            how_the_row_composes(Some(&Ending::Interrupted), true, &held_at_zero),
            HowTheRowComposes::WaitingAndTheFigureIsTruthful
        );
    }

    #[test]
    fn a_verifying_prefix_reads_as_verifying_and_not_as_a_pause_or_a_stall() {
        let verifying = a_report(4_000_000_000, None, Some(4_000_000_000));
        assert_eq!(
            how_the_row_composes(Some(&Ending::Interrupted), true, &verifying),
            HowTheRowComposes::VerifyingAStagedPrefixOf(4_000_000_000)
        );
        // The bounds ride with the mark and are that document's own.
        assert!(!verifying.may_be_read_as_stalled());
        assert!(!verifying.a_completion_estimate_may_be_composed_over_it());
    }

    #[test]
    fn a_transfer_smoozy_stopped_itself_never_composes_as_waiting() {
        let report = a_report(1000, None, None);
        for stopped in [
            StoppedReason::StagingRemovedByUser,
            StoppedReason::AbandonedByItsRequest,
            StoppedReason::TheSourceChangedUnderIt,
        ] {
            assert_eq!(
                how_the_row_composes(Some(&Ending::Stopped(stopped)), true, &report),
                HowTheRowComposes::NotWaiting
            );
        }
        // Neither does a failed or a stalled one: on both the connection is up
        // and the work is over.
        assert_eq!(
            how_the_row_composes(Some(&Ending::Failed), true, &report),
            HowTheRowComposes::NotWaiting
        );
        assert_eq!(
            how_the_row_composes(Some(&Ending::Stalled), true, &report),
            HowTheRowComposes::NotWaiting
        );
    }

    /// **M11: a record with no ending at all is a transfer that is moving, and
    /// it never composes as waiting.**
    ///
    /// The waiting state is *record interrupted, request open, period
    /// standing* — three halves, and a record with no ending fails the first.
    /// Composing it as waiting would tell the user a transfer whose bytes are
    /// crossing at that moment is waiting for a connection.
    #[test]
    fn a_record_with_no_ending_is_a_transfer_that_is_moving_and_never_composes_as_waiting() {
        let moving = a_report(3_000_000, None, None);
        assert_eq!(
            how_the_row_composes(None, true, &moving),
            HowTheRowComposes::NotWaiting
        );
        // Not even where the report carries a mark: the marks are read inside
        // the resume state and this record is not in one.
        let verifying = a_report(3_900_000, None, Some(3_900_000));
        assert_eq!(
            how_the_row_composes(None, true, &verifying),
            HowTheRowComposes::NotWaiting
        );
    }

    #[test]
    fn a_whole_request_stop_never_arranges_more_and_a_single_stop_leaves_the_rest() {
        let whole = what_a_stop_leaves(WhatAStopReaches::TheWholeRequest);
        assert!(whole.finished_files_stay);
        assert!(whole.unarranged_files_are_never_arranged);
        assert!(!whole.the_rest_of_the_request_is_untouched);
        let one = what_a_stop_leaves(WhatAStopReaches::OneUnit);
        assert!(one.finished_files_stay);
        assert!(!one.unarranged_files_are_never_arranged);
        assert!(one.the_rest_of_the_request_is_untouched);
    }

    fn a_record(source: &str, landing: &str, run: Identity) -> TransferRecord {
        TransferRecord::arranged(
            an_identity(IdentityKind::Transfer),
            Direction::Down,
            an_identity(IdentityKind::LocalServer),
            None,
            source.to_string(),
            landing.to_string(),
            "/Users/a/Downloads/.smoozy-staging",
            4096,
            run,
        )
    }

    #[test]
    fn a_take_up_needs_both_gates_and_skip_existing_never_takes_one_up() {
        let record = a_record(
            "/j/out/a.png",
            "/Users/a/Downloads/a.png",
            an_identity(IdentityKind::Run),
        );
        assert!(may_take_up_the_staged_material(
            &record,
            b"/j/out/a.png",
            b"/Users/a/Downloads/a.png",
            &Disposition::ToFetch
        ));
        // Keep both's landing path is fresh, so it can never match.
        assert!(!may_take_up_the_staged_material(
            &record,
            b"/j/out/a.png",
            b"/Users/a/Downloads/a (2).png",
            &Disposition::RenamedUnderKeepBoth(b"/Users/a/Downloads/a (2).png".to_vec())
        ));
        assert!(!may_take_up_the_staged_material(
            &record,
            b"/j/out/a.png",
            b"/Users/a/Downloads/a.png",
            &Disposition::SkipExisting
        ));
        // A different machine path with the same landing path is not the same
        // transfer.
        assert!(!may_take_up_the_staged_material(
            &record,
            b"/j/out/b.png",
            b"/Users/a/Downloads/a.png",
            &Disposition::ToFetch
        ));
    }

    #[test]
    fn staging_no_transfer_record_names_reads_as_unowned_and_removes_nothing_by_itself() {
        let record = a_record(
            "/j/out/a.png",
            "/Users/a/Downloads/a.png",
            an_identity(IdentityKind::Run),
        );
        let mine = record.staging_path.as_bytes();
        assert_eq!(
            what_a_local_staging_entry_is(
                &mine,
                b"/Users/a/Downloads",
                std::slice::from_ref(&record)
            ),
            WhatALocalStagingEntryIs::ATransferRecordNamesIt
        );
        assert_eq!(
            what_a_local_staging_entry_is(
                b"/Users/a/Downloads/.smoozy-staging/leftover",
                b"/Users/a/Downloads",
                std::slice::from_ref(&record)
            ),
            WhatALocalStagingEntryIs::UnownedDownloadStaging {
                destination: b"/Users/a/Downloads".to_vec()
            }
        );
    }

    #[test]
    fn only_the_folders_a_surviving_record_still_names_are_looked_in() {
        let looked_in = the_folders_that_may_be_looked_in(
            &[b"/Users/a/Downloads".to_vec()],
            &[b"/Users/a/Downloads".to_vec(), b"/Volumes/work".to_vec()],
        );
        assert_eq!(
            looked_in,
            vec![
                b"/Users/a/Downloads/.smoozy-staging".to_vec(),
                b"/Volumes/work/.smoozy-staging".to_vec()
            ]
        );
        assert!(the_folders_that_may_be_looked_in(&[], &[]).is_empty());
    }

    #[test]
    fn the_removal_never_follows_an_entry_outside_the_reserved_directory() {
        let reserved = b"/Users/a/Downloads/.smoozy-staging";
        assert!(the_removal_may_run(
            b"/Users/a/Downloads/.smoozy-staging/xyz",
            reserved
        ));
        assert!(!the_removal_may_run(b"/Users/a/Downloads/a.png", reserved));
        // The directory itself is not an entry inside it.
        assert!(!the_removal_may_run(reserved, reserved));
    }

    /// **The bound is component-wise, and this is the case a byte prefix gets
    /// wrong.**
    ///
    /// `.smoozy-staging-old` is an ordinary folder of the user's whose name
    /// happens to begin with the reserved one. A prefix test says the thing
    /// inside it is inside the reserved directory, and the adapter is then
    /// asked to delete a file of theirs.
    #[test]
    fn a_folder_whose_name_merely_begins_with_the_reserved_one_is_never_removed_from() {
        let reserved = b"/Users/a/Downloads/.smoozy-staging";
        assert!(!the_removal_may_run(
            b"/Users/a/Downloads/.smoozy-staging-old/keep_this",
            reserved
        ));
        assert!(!the_removal_may_run(
            b"/Users/a/Downloads/.smoozy-stagingx",
            reserved
        ));
        // And a deeper entry genuinely inside it still may be removed.
        assert!(the_removal_may_run(
            b"/Users/a/Downloads/.smoozy-staging/a/b",
            reserved
        ));
    }
}
