//! **The receiving-filesystem collision judgement, its taken half and the
//! limits of its name half** — `Desktop app and delivery` §7.
//!
//! **What is here.** C10's exclusive create of the final name, which is both the
//! answer to *is this name already taken* and the reservation publication
//! renames over; C12, nothing published and nothing overwritten where the name
//! is present; C13, the reservation's window declared, and what the next run does
//! with a reservation a record still names; C14, a deliberate replacement taking
//! no reservation; C18, *taken* said without a stored name where none was
//! supplied; C21, the count of what arrived read by path and never taken from
//! the count of what was fetched; C22, every ordinary publication a rename onto
//! an existing entry. And two limits of the name half: C8's visible hard errors,
//! and C9's thirty-one combining marks.
//!
//! **What is not here, said so nobody reads this file as the whole of §7.** C3
//! to C7's name check, C11's reasoning (which is this file's shape rather than a
//! function), C15 to C17's reading of the stored name, C19's outcome as one
//! value handed to `Download`, and C20's re-judging of a composed name. **None of
//! those has code in this crate**, and step 10a piece 12 was given the nine
//! above and no others. So [`WhatTheExclusiveCreateSaid::TheNameIsTaken`] carries
//! no stored name on any system today — which is C18's own reading, and it
//! costs wording and not the promise.
//!
//! **Where it sits, and why here rather than in a screen.** §7 is the platform
//! file adapter's and `Download` never takes it a second time (*"the user-facing
//! response to that adapter's collision judgement, not a second collision
//! judgement"*, `The shape`'s own sentence). The Transfers screen draws what this
//! decides; it decides nothing.
//!
//! **Markers used below that belong to another document, each with the document
//! it belongs to.** `Server protocol and transfers` S15a, S18, S28. `Job files
//! and outputs` N14.
//!
//! **A marker this crate's own plan also defines is written out with its
//! document at every site instead**, because the header list cannot tell the
//! two apart.

use std::io;
use std::path::{Path, PathBuf};

use smoozy_server_protocol::transfers::{Direction, Ending, TransferRecord};

use super::files::{self, Failure, Recheck, Where};

// ═══════════════════════════════════════════════════════════════════════════
// C8 and C9 — two limits of the name half
// ═══════════════════════════════════════════════════════════════════════════

/// **C8: everything else measured on the Mac is a visible hard error, and
/// visible errors are safe.** The three the plan names.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AVisibleHardError {
    /// `ENAMETOOLONG`.
    TheNameIsTooLong,
    /// `EILSEQ` — macOS refuses invalid UTF-8 outright (C3's own ground).
    TheNameCannotBeRepresented,
    /// `EEXIST` — which is C10's own answer when it comes from the exclusive
    /// create.
    TheNameIsTaken,
}

/// **C8, read off a returned error.**
///
/// **Only the Mac's two name errors are named, and only on the Mac.** C8's own
/// words: *"Nothing is claimed about what Windows returns: no page read states it
/// and nothing was measured"*, and the measurement is the Mac's, so Linux is not
/// read across to either. **A name that is already present is recognised on all
/// three** by the standard library's own `AlreadyExists` kind, because C10 and
/// C18 both say the exclusive create detects the collision *"exactly as on the
/// other two"* — that is the one reading every system gives back.
pub fn the_visible_hard_error_in(error: &io::Error) -> Option<AVisibleHardError> {
    if error.kind() == io::ErrorKind::AlreadyExists {
        return Some(AVisibleHardError::TheNameIsTaken);
    }
    #[cfg(target_os = "macos")]
    {
        match error.raw_os_error() {
            Some(code) if code == libc::ENAMETOOLONG => {
                return Some(AVisibleHardError::TheNameIsTooLong)
            }
            Some(code) if code == libc::EILSEQ => {
                return Some(AVisibleHardError::TheNameCannotBeRepresented)
            }
            _ => {}
        }
    }
    None
}

/// **C9: a base character carries at most thirty-one combining marks before
/// `EILSEQ`.** Measured, undocumented, on one volume format.
pub const THE_MOST_COMBINING_MARKS_ONE_CHARACTER_WAS_MEASURED_TO_TAKE: usize = 31;

/// **C9, carried as a limit of the check rather than as a rule about the
/// system.** True where some base character in `name` carries more combining
/// marks than the measurement took.
///
/// **What it counts, said rather than glossed.** The five Unicode blocks that
/// hold nothing but combining marks — the Combining Diacritical Marks and its
/// Extended and Supplement blocks, the marks for symbols, and the half marks.
/// **A combining mark outside those blocks is not counted**, so this can say
/// *too many* and can never say *enough*; where it stays silent, C8's `EILSEQ`
/// is the visible error that still refuses the name.
///
/// **Nothing calls it yet, and that is C3's absence and not this limit's.** The
/// name check it belongs inside is C3 to C7, which no code in this crate keeps.
pub fn a_character_carries_more_combining_marks_than_measured(name: &str) -> bool {
    let mut run = 0usize;
    for character in name.chars() {
        if is_in_a_combining_block(character) {
            run += 1;
            if run > THE_MOST_COMBINING_MARKS_ONE_CHARACTER_WAS_MEASURED_TO_TAKE {
                return true;
            }
        } else {
            run = 0;
        }
    }
    false
}

fn is_in_a_combining_block(character: char) -> bool {
    matches!(
        character as u32,
        0x0300..=0x036F | 0x1AB0..=0x1AFF | 0x1DC0..=0x1DFF | 0x20D0..=0x20FF | 0xFE20..=0xFE2F
    )
}

// ═══════════════════════════════════════════════════════════════════════════
// C10, C12, C18 — the taken check is an exclusive create
// ═══════════════════════════════════════════════════════════════════════════

/// **Smoozy's own zero-byte file standing at the final name** — C10's
/// reservation, which publication renames over.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AReservation {
    pub final_name: PathBuf,
    /// `Desktop app and delivery` F15a's pair for the object the create made,
    /// where the system answers. **It is what lets a reservation be taken back
    /// without taking back somebody else's file** that replaced it in the window.
    identity: Option<files::FileIdentity>,
}

/// **What the exclusive create answered.** C19's shape for the taken half.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WhatTheExclusiveCreateSaid {
    /// **C10: the name was absent and is now Smoozy's reservation.**
    TheNameIsReserved(AReservation),
    /// **C12: the name is present, and nothing is published and nothing
    /// overwritten.**
    ///
    /// **C18: where the system supplied no stored name, the outcome says *taken*
    /// without saying which name took it.** C15 to C17's readers are not in this
    /// crate, so on all three systems this is `None` today; **nothing guesses one
    /// from the name asked for**, which is the reading C11 forbids.
    TheNameIsTaken {
        the_stored_name_that_took_it: Option<PathBuf>,
    },
    /// The create was refused for another reason, classified by
    /// `Desktop app and delivery` F4.
    ItWasRefused(Failure),
}

/// **C10: the question *is this name already taken* is put to the filesystem
/// as an exclusive create of the final name — create if absent, fail if
/// present — and the created entry is the reservation publication renames
/// over.**
///
/// **C11 is why nothing is compared here.** The filesystem's own answer to
/// *create this name if absent* already embodies its own folding rules, the
/// ones nobody documents included, so a one-character name meeting a
/// two-character one is caught by the create and would be missed by any
/// comparison this file could make.
pub fn reserve_the_final_name(final_name: &Path) -> WhatTheExclusiveCreateSaid {
    match std::fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(final_name)
    {
        Ok(_) => WhatTheExclusiveCreateSaid::TheNameIsReserved(AReservation {
            final_name: final_name.to_path_buf(),
            identity: files::file_identity(final_name),
        }),
        Err(error)
            if the_visible_hard_error_in(&error) == Some(AVisibleHardError::TheNameIsTaken) =>
        {
            WhatTheExclusiveCreateSaid::TheNameIsTaken {
                the_stored_name_that_took_it: None,
            }
        }
        Err(error) => WhatTheExclusiveCreateSaid::ItWasRefused(Failure {
            refusal: files::classify(&error, Where::Open),
            reported: error.to_string(),
        }),
    }
}

/// **Take a reservation back, and only Smoozy's own.**
///
/// **Removed only while it is still zero bytes and still the object the create
/// made**, so a file the user saved into the window (C13's *a file the user
/// could meet*) is never removed in its place. Where the identity cannot be
/// read, the zero-byte test stands alone and nothing is claimed beyond it.
fn take_the_reservation_back(reservation: &AReservation) -> bool {
    let Ok(found) = std::fs::metadata(&reservation.final_name) else {
        return false;
    };
    if found.len() != 0 {
        return false;
    }
    if let Some(made) = reservation.identity {
        if files::file_identity(&reservation.final_name) != Some(made) {
            return false;
        }
    }
    files::remove_file(&reservation.final_name).is_ok()
}

// ═══════════════════════════════════════════════════════════════════════════
// C14 and C22 — the two publications
// ═══════════════════════════════════════════════════════════════════════════

/// Why a create-intended publication did not land.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WhyNothingWasPublished {
    /// **C12.** The name was present, and it is untouched.
    TheNameIsTaken {
        the_stored_name_that_took_it: Option<PathBuf>,
    },
    /// The reservation could not be made, or the rename over it failed.
    ItWasRefused(Failure),
}

/// **C22: a create-intended publication is the reservation, then the replacing
/// rename onto it.**
///
/// *"Because the reservation stands at the final name, every ordinary
/// publication is now a rename onto an existing entry"* — so the rename is
/// `Desktop app and delivery` F15's plain replacing one in every case, and
/// **Windows's documented absence of a crash guarantee is load-bearing for every
/// download** (§22 item 4), which is said here rather than softened.
///
/// **Where the rename fails the reservation is taken back at once**, and only
/// if it is still Smoozy's (see [`take_the_reservation_back`]). **Where the
/// process dies between the two, C13 is what holds**: the transfer record is
/// already in *publication pending* naming both paths — `Server protocol and
/// transfers` S15a requires it before the first step runs — and
/// [`what_the_next_run_does_with`] says what the next run does.
pub fn publish_create_intended(
    staged: &Path,
    final_name: &Path,
) -> Result<(), WhyNothingWasPublished> {
    let reservation = match reserve_the_final_name(final_name) {
        WhatTheExclusiveCreateSaid::TheNameIsReserved(reservation) => reservation,
        WhatTheExclusiveCreateSaid::TheNameIsTaken {
            the_stored_name_that_took_it,
        } => {
            return Err(WhyNothingWasPublished::TheNameIsTaken {
                the_stored_name_that_took_it,
            })
        }
        WhatTheExclusiveCreateSaid::ItWasRefused(failure) => {
            return Err(WhyNothingWasPublished::ItWasRefused(failure))
        }
    };
    files::publish(staged, &reservation.final_name).map_err(|failure| {
        take_the_reservation_back(&reservation);
        WhyNothingWasPublished::ItWasRefused(failure)
    })
}

/// **C14: a deliberate replacement is not a silent one.** Where the user has
/// answered `Download`'s question with *replace*, **the replacing rename runs
/// and no reservation is taken.** *"Row 303 forbids silence, not replacement."*
pub fn publish_replace_intended(staged: &Path, final_name: &Path) -> Result<(), Failure> {
    files::publish(staged, final_name)
}

// ═══════════════════════════════════════════════════════════════════════════
// C13 — the reservation's window, and the next run
// ═══════════════════════════════════════════════════════════════════════════

/// **C13: the reservation's window is declared rather than discovered.**
///
/// Between the exclusive create and the rename a **zero-byte** file stands at
/// the final name. It is not part-arrived bytes, and it is a file the user could
/// meet. **The Creator ruled it acceptable on 2026-08-18** and §22 item 30
/// carries what it costs.
pub const THE_RESERVATIONS_WINDOW_IS_DECLARED_AND_RULED_ACCEPTABLE: bool = true;

/// **What the next run does with a reservation a record still names** — C13's
/// *"the next run either completes the publication or removes the
/// reservation"*.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhatTheNextRunDoes {
    /// The staged object is whole and a zero-byte file stands at the final
    /// name: **the publication is completed**, by the rename over it (C22).
    CompleteThePublication,
    /// The staged object is gone and a zero-byte file stands at the final name:
    /// **the reservation is removed.**
    RemoveTheReservation,
    /// **Something that is not Smoozy's reservation stands at the final name** —
    /// bytes the record does not describe, or a file that cannot be read. It is
    /// never removed, because *"a file the user could meet"* may be the user's.
    LeaveWhatStandsThere,
    /// Nothing stands at the final name, or what stands there is the published
    /// object itself. **This file has nothing to do**; whether the publication
    /// happened is `Server protocol and transfers` S15's recovery, which reads the
    /// digest and the identity this file does not.
    NothingIsOwedHere,
    /// **The record's ending already says the work is over, and what stands at
    /// the final name is not Smoozy's reservation** — the user's own file, most
    /// often, which is why the publication failed. **It is left exactly as it
    /// stands, and the record owes nothing** (`Server protocol and transfers`
    /// S15's paragraph of 2026-09-29).
    TheWorkIsOverAndWhatStandsThereIsNotSmoozys,
}

/// **C13, decided from two readings taken now and the record's own pre-state.**
///
/// **A pre-state of zero bytes is never read as a reservation**: an empty file
/// is a real download, and a zero-byte file at its final name with the staging
/// gone is exactly what a finished publication of it looks like. So nothing is
/// removed there.
pub fn what_the_next_run_does_with(
    the_staged_bytes_recorded: u64,
    the_staged_object_now: &Recheck,
    the_final_name_now: &Recheck,
) -> WhatTheNextRunDoes {
    // A reading that names a length it expected is read by the length it
    // found, which is the only half of it that is about the world now.
    let as_found = |reading: &Recheck| match reading {
        Recheck::ShortOrDifferent { bytes, .. } => Recheck::Present { bytes: *bytes },
        other => other.clone(),
    };
    let the_final_name_now = &as_found(the_final_name_now);
    let the_staged_object_now = &as_found(the_staged_object_now);
    match the_final_name_now {
        Recheck::Absent => WhatTheNextRunDoes::NothingIsOwedHere,
        Recheck::CouldNotLook { .. } | Recheck::ShortOrDifferent { .. } => {
            WhatTheNextRunDoes::LeaveWhatStandsThere
        }
        Recheck::Present { bytes } if *bytes == 0 && the_staged_bytes_recorded > 0 => {
            match the_staged_object_now {
                Recheck::Present { bytes } if *bytes == the_staged_bytes_recorded => {
                    WhatTheNextRunDoes::CompleteThePublication
                }
                Recheck::Absent => WhatTheNextRunDoes::RemoveTheReservation,
                // A staged object of another size is not the whole one the
                // pre-state recorded, and one that cannot be read is not known
                // to be gone. Neither completes and neither removes.
                _ => WhatTheNextRunDoes::LeaveWhatStandsThere,
            }
        }
        Recheck::Present { bytes } if *bytes == the_staged_bytes_recorded => {
            WhatTheNextRunDoes::NothingIsOwedHere
        }
        Recheck::Present { .. } => WhatTheNextRunDoes::LeaveWhatStandsThere,
    }
}

/// **C13 for a record whose ending already says the work is over** — *failed*,
/// *stalled* or *stopped* (`Server protocol and transfers` S26). Added
/// 2026-09-29 under `Server protocol and transfers` S15's paragraph of that day,
/// the build runner's ruling.
///
/// **Nothing is completed.** The ending already said the work was over, and a
/// start that turned it into *published* would contradict the record it reads —
/// so the staged object's state does not enter, and only the final name is
/// read:
///
/// - **nothing there**: nothing is owed;
/// - **Smoozy's zero-byte reservation** (the transfer staged bytes, and a
///   zero-byte file stands at the name): it is removed, on C13's own reading;
/// - **anything else** — the user's own file, most often, which is why the
///   publication failed: **left exactly as it stands**, and the record owes
///   nothing;
/// - **a name that cannot be read**: left, and the record stays pending, since
///   what stands there is not known.
pub fn what_the_next_run_does_with_a_record_already_ended(
    the_staged_bytes_recorded: u64,
    the_final_name_now: &Recheck,
) -> WhatTheNextRunDoes {
    match the_final_name_now {
        Recheck::Absent => WhatTheNextRunDoes::NothingIsOwedHere,
        Recheck::CouldNotLook { .. } => WhatTheNextRunDoes::LeaveWhatStandsThere,
        Recheck::Present { bytes } | Recheck::ShortOrDifferent { bytes, .. }
            if *bytes == 0 && the_staged_bytes_recorded > 0 =>
        {
            WhatTheNextRunDoes::RemoveTheReservation
        }
        Recheck::Present { .. } | Recheck::ShortOrDifferent { .. } => {
            WhatTheNextRunDoes::TheWorkIsOverAndWhatStandsThereIsNotSmoozys
        }
    }
}

/// Whether a record's ending says the work is over — `Server protocol and
/// transfers` S26's *failed*, *stalled* and *stopped*, the three no retry
/// follows.
fn the_work_is_over(record: &TransferRecord) -> bool {
    matches!(
        record.ending,
        Some(Ending::Failed) | Some(Ending::Stalled) | Some(Ending::Stopped(_))
    )
}

/// **The records whose reservation the next run owes an answer.** A download,
/// in *publication pending*, create-intended, and not published.
///
/// **Called at application start, beside `Server protocol and transfers` S28's
/// start sweep, before any transfer is admitted** (`Download` R6) —
/// `crate::start`'s `what_the_last_run_left_is_settled`. *(Until 2026-09-28 this
/// said nothing called it at start, which was true: it and the sweep were each
/// exercised in tests alone, from the day they were written.)*
pub fn the_reservations_this_run_owes(records: &[TransferRecord]) -> Vec<&TransferRecord> {
    use smoozy_server_protocol::transfers::publication::Intent;
    records
        .iter()
        .filter(|record| record.direction == Direction::Down)
        .filter(|record| record.ending != Some(Ending::Published))
        .filter(|record| {
            record
                .publication_pending
                .as_ref()
                .is_some_and(|pending| pending.intent == Intent::CreateIntended)
        })
        .collect()
}

/// **C13 performed for one record**: both readings taken now, by path (F1), and
/// the one act that follows. Returns what it decided.
pub fn settle_the_reservation_of(record: &TransferRecord) -> WhatTheNextRunDoes {
    let Some(pending) = record.publication_pending.as_ref() else {
        return WhatTheNextRunDoes::NothingIsOwedHere;
    };
    let staged = PathBuf::from(&pending.staging_path);
    let final_name = PathBuf::from(&pending.final_path);
    // **A record already ended is settled without completing anything**
    // (`Server protocol and transfers` S15's paragraph of 2026-09-29).
    let decided = if the_work_is_over(record) {
        what_the_next_run_does_with_a_record_already_ended(
            pending.staged_byte_length,
            &files::outside_program_wrote(&final_name),
        )
    } else {
        what_the_next_run_does_with(
            pending.staged_byte_length,
            &files::outside_program_wrote(&staged),
            &files::outside_program_wrote(&final_name),
        )
    };
    match decided {
        WhatTheNextRunDoes::CompleteThePublication => {
            if files::publish(&staged, &final_name).is_err() {
                return WhatTheNextRunDoes::LeaveWhatStandsThere;
            }
        }
        WhatTheNextRunDoes::RemoveTheReservation => {
            if files::remove_file(&final_name).is_err() {
                return WhatTheNextRunDoes::LeaveWhatStandsThere;
            }
        }
        WhatTheNextRunDoes::LeaveWhatStandsThere
        | WhatTheNextRunDoes::NothingIsOwedHere
        | WhatTheNextRunDoes::TheWorkIsOverAndWhatStandsThereIsNotSmoozys => {}
    }
    decided
}

/// **The record once C13 has settled it** — what the next run writes down
/// after [`settle_the_reservation_of`] has acted, or `None` where nothing was
/// settled and the record stands as it is.
///
/// **Why the record has to move, and it is the half that matters more than
/// the window is wide.** `Download`'s own rule admits no transfer of a server
/// while any of its records holds a publication pending (`Download` R6's
/// ordering), so an act on the disk that left the record saying *pending*
/// would lift nothing: the server's downloads would stay refused, for good,
/// with nothing said. So the pre-state goes wherever the act has left nothing
/// it describes:
///
/// - **the publication completed**: the record is *published*, and nothing is
///   pending;
/// - **the reservation removed**: nothing is pending, and **the ending is left
///   as it was** — the staged object was already gone, so there is nothing to
///   carry on and nothing here to claim about why;
/// - **nothing owed here**: nothing is pending, and the record is *published*
///   only where `Server protocol and transfers` S15's own recovery judgement
///   proves the object at the final name is the one staged — never on a length
///   alone, since *"only an exact match with the staged-object evidence durably
///   recorded before rename proves that the expected object crossed"*;
/// - **the work already over and what stands there not Smoozy's**: nothing is
///   pending, the ending is left as it was, and the file is untouched
///   (`Server protocol and transfers` S15's paragraph of 2026-09-29);
/// - **what stands there left alone** — something that is not Smoozy's on a
///   record whose work is not over, a reading that could not be taken, or an
///   act that did not go through: the record is **not** moved, and the next
///   start asks again.
///
/// *(Until 2026-09-29 a file of the user's own at the final name kept every
/// record pending, and its server's downloads refused, for good. The runner's
/// ruling of that day, written into `Server protocol and transfers` S15, is
/// what the third bullet keeps.)*
pub fn the_record_once_settled(
    record: &TransferRecord,
    decided: WhatTheNextRunDoes,
    the_final_object_now: &smoozy_server_protocol::transfers::publication::WhatTheFinalObjectReadsAs,
) -> Option<TransferRecord> {
    use smoozy_server_protocol::transfers::publication::{judge_the_recovery, RecoveryVerdict};

    let pending = record.publication_pending.as_ref()?;
    let mut settled = record.clone();
    match decided {
        WhatTheNextRunDoes::CompleteThePublication => settled.ending = Some(Ending::Published),
        WhatTheNextRunDoes::RemoveTheReservation
        | WhatTheNextRunDoes::TheWorkIsOverAndWhatStandsThereIsNotSmoozys => {}
        WhatTheNextRunDoes::NothingIsOwedHere => {
            if judge_the_recovery(pending, false, the_final_object_now)
                == RecoveryVerdict::ThePublicationHappened
            {
                settled.ending = Some(Ending::Published);
            }
        }
        WhatTheNextRunDoes::LeaveWhatStandsThere => return None,
    }
    settled.publication_pending = None;
    Some(settled)
}

// ═══════════════════════════════════════════════════════════════════════════
// C21 — the count of what arrived
// ═══════════════════════════════════════════════════════════════════════════

/// **C21: the count of what arrived is never the count of what was fetched**,
/// and F1's re-check by path is what says which a caller has.
///
/// *"Seventeen names written on the rental came back as fourteen files on the
/// Mac"*, with every fetch reporting success (`Job files and outputs` N14).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct TheCountOfWhatArrived {
    /// A file stands at the final name now.
    pub arrived: usize,
    /// Nothing stands there.
    pub not_there: usize,
    /// The path could not be read, which is neither of the two above.
    pub could_not_look: usize,
}

/// **C21, taken by path, one reading per final name, now.** Two final names
/// the receiving filesystem folds into one are two names and **one file**, so
/// they are counted as two arrivals only where the system says both are there
/// — which on a folding volume it will, for the one file; the count this
/// returns is therefore a count of names that answer, and a caller comparing it
/// to the fetch count is told only that no fewer answered.
pub fn the_count_of_what_arrived(final_names: &[PathBuf]) -> TheCountOfWhatArrived {
    let mut count = TheCountOfWhatArrived::default();
    let mut identities_seen = Vec::new();
    for name in final_names {
        match files::outside_program_wrote(name) {
            Recheck::Present { .. } | Recheck::ShortOrDifferent { .. } => {
                // **Two names answering with one file identity are one
                // file**, which is the seventeen-to-fourteen loss arriving at
                // this count. Where no identity is supplied, the name counts on
                // its own and nothing is claimed beyond it.
                match files::file_identity(name) {
                    Some(identity) if identities_seen.contains(&identity) => count.not_there += 1,
                    Some(identity) => {
                        identities_seen.push(identity);
                        count.arrived += 1;
                    }
                    None => count.arrived += 1,
                }
            }
            Recheck::Absent => count.not_there += 1,
            Recheck::CouldNotLook { .. } => count.could_not_look += 1,
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    fn a_scratch(name: &str) -> PathBuf {
        let path = std::env::temp_dir().join(format!(
            "smoozy-collision-{name}-{}",
            std::process::id()
        ));
        let _ = std::fs::remove_dir_all(&path);
        std::fs::create_dir_all(&path).expect("a temporary directory");
        path
    }

    /// **C10: the exclusive create reserves an absent name**, and the entry it
    /// makes is zero bytes.
    #[test]
    fn an_absent_name_is_reserved_at_zero_bytes() {
        let scratch = a_scratch("reserve");
        let final_name = scratch.join("frame_0001.exr");
        match reserve_the_final_name(&final_name) {
            WhatTheExclusiveCreateSaid::TheNameIsReserved(reservation) => {
                assert_eq!(reservation.final_name, final_name);
            }
            other => panic!("expected a reservation, got {other:?}"),
        }
        assert_eq!(std::fs::metadata(&final_name).expect("it stands").len(), 0);
        let _ = std::fs::remove_dir_all(&scratch);
    }

    /// **C12 and C18: a present name is taken, it is untouched, and no stored
    /// name is guessed.**
    #[test]
    fn a_present_name_is_taken_untouched_and_no_stored_name_is_guessed() {
        let scratch = a_scratch("taken");
        let final_name = scratch.join("frame_0001.exr");
        std::fs::write(&final_name, b"the user's own compositing copy").expect("written");
        assert_eq!(
            reserve_the_final_name(&final_name),
            WhatTheExclusiveCreateSaid::TheNameIsTaken {
                the_stored_name_that_took_it: None
            }
        );
        assert_eq!(
            std::fs::read(&final_name).expect("still there"),
            b"the user's own compositing copy"
        );
        let _ = std::fs::remove_dir_all(&scratch);
    }

    /// **C22: a create-intended publication lands where nothing stands**, and
    /// the staged object is gone because the rename took it.
    #[test]
    fn a_create_intended_publication_lands_over_its_own_reservation() {
        let scratch = a_scratch("c22");
        let staged = scratch.join(".smoozy-staging-one");
        let final_name = scratch.join("frame_0002.exr");
        std::fs::write(&staged, b"the whole frame").expect("written");
        publish_create_intended(&staged, &final_name).expect("it lands");
        assert_eq!(std::fs::read(&final_name).expect("there"), b"the whole frame");
        assert!(!staged.exists());
        let _ = std::fs::remove_dir_all(&scratch);
    }

    /// **C12, through the publication: nothing published, nothing overwritten,
    /// and the staged object is left for its own cleanup rather than lost.**
    #[test]
    fn a_create_intended_publication_never_writes_over_a_present_name() {
        let scratch = a_scratch("c12");
        let staged = scratch.join(".smoozy-staging-two");
        let final_name = scratch.join("frame_0003.exr");
        std::fs::write(&staged, b"new").expect("written");
        std::fs::write(&final_name, b"old").expect("written");
        assert!(matches!(
            publish_create_intended(&staged, &final_name),
            Err(WhyNothingWasPublished::TheNameIsTaken { .. })
        ));
        assert_eq!(std::fs::read(&final_name).expect("there"), b"old");
        assert!(staged.exists(), "the staged object is the transfer's own cleanup, not this file's");
        let _ = std::fs::remove_dir_all(&scratch);
    }

    /// **The rename over the reservation fails, so the reservation is taken
    /// back** — and only because it is still Smoozy's zero-byte object.
    #[test]
    fn a_failed_rename_takes_its_own_reservation_back() {
        let scratch = a_scratch("back");
        let final_name = scratch.join("frame_0004.exr");
        let not_there = scratch.join("no-such-staged-object");
        assert!(publish_create_intended(&not_there, &final_name).is_err());
        assert!(!final_name.exists(), "the reservation was left behind");
        let _ = std::fs::remove_dir_all(&scratch);
    }

    /// **A file the user put into the window is never taken back in place of
    /// the reservation.**
    #[test]
    fn a_reservation_the_user_wrote_into_is_not_taken_back() {
        let scratch = a_scratch("window");
        let final_name = scratch.join("frame_0005.exr");
        let WhatTheExclusiveCreateSaid::TheNameIsReserved(reservation) =
            reserve_the_final_name(&final_name)
        else {
            panic!("expected a reservation");
        };
        std::fs::write(&final_name, b"saved out of Blender in the window").expect("written");
        assert!(!take_the_reservation_back(&reservation));
        assert!(final_name.exists());
        let _ = std::fs::remove_dir_all(&scratch);
    }

    /// **C14: a replacement replaces and takes no reservation.**
    #[test]
    fn a_replacement_replaces() {
        let scratch = a_scratch("c14");
        let staged = scratch.join(".smoozy-staging-three");
        let final_name = scratch.join("frame_0006.exr");
        std::fs::write(&staged, b"new").expect("written");
        std::fs::write(&final_name, b"old").expect("written");
        publish_replace_intended(&staged, &final_name).expect("it replaces");
        assert_eq!(std::fs::read(&final_name).expect("there"), b"new");
        let _ = std::fs::remove_dir_all(&scratch);
    }

    /// **C13's four answers, from the readings alone.**
    #[test]
    fn the_next_run_completes_removes_or_leaves() {
        let present = |bytes| Recheck::Present { bytes };
        assert_eq!(
            what_the_next_run_does_with(1024, &present(1024), &present(0)),
            WhatTheNextRunDoes::CompleteThePublication
        );
        assert_eq!(
            what_the_next_run_does_with(1024, &Recheck::Absent, &present(0)),
            WhatTheNextRunDoes::RemoveTheReservation
        );
        assert_eq!(
            what_the_next_run_does_with(1024, &Recheck::Absent, &present(1024)),
            WhatTheNextRunDoes::NothingIsOwedHere
        );
        assert_eq!(
            what_the_next_run_does_with(1024, &Recheck::Absent, &present(77)),
            WhatTheNextRunDoes::LeaveWhatStandsThere,
            "bytes the record does not describe may be the user's"
        );
        assert_eq!(
            what_the_next_run_does_with(0, &Recheck::Absent, &present(0)),
            WhatTheNextRunDoes::NothingIsOwedHere,
            "an empty download is a real file and never read as a reservation"
        );
        assert_eq!(
            what_the_next_run_does_with(1024, &present(10), &present(0)),
            WhatTheNextRunDoes::LeaveWhatStandsThere
        );
        assert_eq!(
            what_the_next_run_does_with(1024, &Recheck::Absent, &Recheck::Absent),
            WhatTheNextRunDoes::NothingIsOwedHere
        );
    }

    /// **C13 performed, over a real record and a real folder**: a crash between
    /// the reservation and the rename, then the next run.
    #[test]
    fn the_next_run_completes_a_publication_a_crash_left_reserved() {
        use smoozy_server_protocol::transfers::integrity::Digest;
        use smoozy_server_protocol::transfers::publication::{Intent, PublicationPreState};
        use smoozy_server_protocol::transfers::StableFileIdentity;
        use smoozy_state::identities::{Identity, IdentityKind};

        let scratch = a_scratch("c13");
        let issue = |kind| Identity::issue(kind).expect("random bytes");
        let final_name = scratch.join("frame_0007.exr");
        let mut record = TransferRecord::arranged(
            issue(IdentityKind::Transfer),
            Direction::Down,
            issue(IdentityKind::LocalServer),
            None,
            "/machine/frame_0007.exr".to_string(),
            final_name.to_string_lossy().into_owned(),
            &scratch.join(".smoozy-staging").to_string_lossy(),
            12,
            issue(IdentityKind::Run),
        );
        let staged = PathBuf::from(record.staging_path.as_text());
        std::fs::create_dir_all(staged.parent().expect("a parent")).expect("the staging folder");
        std::fs::write(&staged, b"twelve bytes").expect("staged");
        record.publication_pending = Some(PublicationPreState {
            intent: Intent::CreateIntended,
            staging_path: record.staging_path.as_text(),
            final_path: record.final_path.clone(),
            staged_identity: StableFileIdentity::TheSupplierCouldNotAnswer,
            staged_byte_length: 12,
            staged_digest: Digest::of(b"twelve bytes"),
        });
        // The crash: the reservation is made and the rename never ran.
        assert!(matches!(
            reserve_the_final_name(&final_name),
            WhatTheExclusiveCreateSaid::TheNameIsReserved(_)
        ));

        let records = [record];
        let owed = the_reservations_this_run_owes(&records);
        assert_eq!(owed.len(), 1);
        assert_eq!(
            settle_the_reservation_of(owed[0]),
            WhatTheNextRunDoes::CompleteThePublication
        );
        assert_eq!(std::fs::read(&final_name).expect("there"), b"twelve bytes");
        assert!(!staged.exists());
        let _ = std::fs::remove_dir_all(&scratch);
    }

    /// **The record once C13 has settled it**, for the two outcomes the start's
    /// own test does not reach: a record is *published* only on `Server
    /// protocol and transfers` S15's proof, and a record whose final name holds
    /// something C13 leaves alone is not moved at all.
    #[test]
    fn a_settled_record_is_published_only_on_proof_and_one_left_alone_is_not_moved() {
        use smoozy_server_protocol::transfers::integrity::Digest;
        use smoozy_server_protocol::transfers::publication::{
            Intent, PublicationPreState, WhatTheFinalObjectReadsAs,
        };
        use smoozy_server_protocol::transfers::StableFileIdentity;
        use smoozy_state::identities::{Identity, IdentityKind};

        let issue = |kind| Identity::issue(kind).expect("random bytes");
        let mut record = TransferRecord::arranged(
            issue(IdentityKind::Transfer),
            Direction::Down,
            issue(IdentityKind::LocalServer),
            None,
            "/machine/frame.exr".to_string(),
            "/here/frame.exr".to_string(),
            "/here/.smoozy-staging",
            12,
            issue(IdentityKind::Run),
        );
        record.ending = Some(Ending::Interrupted);
        record.publication_pending = Some(PublicationPreState {
            intent: Intent::CreateIntended,
            staging_path: record.staging_path.as_text(),
            final_path: record.final_path.clone(),
            staged_identity: StableFileIdentity::The("1:7".into()),
            staged_byte_length: 12,
            staged_digest: Digest::of(b"twelve bytes"),
        });
        let the_staged_object_itself = WhatTheFinalObjectReadsAs {
            exists: true,
            byte_length: Some(12),
            identity: Some("1:7".into()),
            at_rest_digest: None,
        };
        let another_object = WhatTheFinalObjectReadsAs {
            identity: Some("1:9".into()),
            ..the_staged_object_itself.clone()
        };

        // The rename landed and the record never heard: proved, so published.
        let settled = the_record_once_settled(
            &record,
            WhatTheNextRunDoes::NothingIsOwedHere,
            &the_staged_object_itself,
        )
        .expect("settled");
        assert_eq!(settled.ending, Some(Ending::Published));
        assert_eq!(settled.publication_pending, None);

        // The same length and another object: nothing pending, nothing claimed.
        let settled =
            the_record_once_settled(&record, WhatTheNextRunDoes::NothingIsOwedHere, &another_object)
                .expect("settled");
        assert_eq!(settled.ending, Some(Ending::Interrupted));
        assert_eq!(settled.publication_pending, None);

        // Something C13 leaves alone: the record stands as it was.
        assert_eq!(
            the_record_once_settled(
                &record,
                WhatTheNextRunDoes::LeaveWhatStandsThere,
                &another_object
            ),
            None
        );
    }

    /// **C13 for a record already ended, from the final name alone**: nothing
    /// is completed whatever is staged, Smoozy's reservation is removed, the
    /// user's file is left and owes nothing, and an unreadable name leaves the
    /// record pending.
    #[test]
    fn a_record_already_ended_completes_nothing_and_leaves_what_is_not_smoozys() {
        let present = |bytes| Recheck::Present { bytes };
        assert_eq!(
            what_the_next_run_does_with_a_record_already_ended(1024, &Recheck::Absent),
            WhatTheNextRunDoes::NothingIsOwedHere
        );
        assert_eq!(
            what_the_next_run_does_with_a_record_already_ended(1024, &present(0)),
            WhatTheNextRunDoes::RemoveTheReservation
        );
        for theirs in [present(77), present(1024)] {
            assert_eq!(
                what_the_next_run_does_with_a_record_already_ended(1024, &theirs),
                WhatTheNextRunDoes::TheWorkIsOverAndWhatStandsThereIsNotSmoozys
            );
        }
        assert_eq!(
            what_the_next_run_does_with_a_record_already_ended(0, &present(0)),
            WhatTheNextRunDoes::TheWorkIsOverAndWhatStandsThereIsNotSmoozys,
            "an empty download's zero-byte file is never read as a reservation"
        );
        assert_eq!(
            what_the_next_run_does_with_a_record_already_ended(
                1024,
                &Recheck::CouldNotLook {
                    reported: "refused".into()
                }
            ),
            WhatTheNextRunDoes::LeaveWhatStandsThere
        );
    }

    /// **C21: two names answering with one file are one arrival.**
    #[test]
    fn two_names_one_file_count_as_one_arrival() {
        let scratch = a_scratch("c21");
        let one = scratch.join("Frame.exr");
        std::fs::write(&one, b"x").expect("written");
        let hard_link = scratch.join("frame-linked.exr");
        std::fs::hard_link(&one, &hard_link).expect("a second name for one file");
        let count = the_count_of_what_arrived(&[one, hard_link, scratch.join("absent.exr")]);
        if files::file_identity(&scratch.join("Frame.exr")).is_some() {
            assert_eq!(count.arrived, 1);
            assert_eq!(count.not_there, 2);
        } else {
            assert_eq!(count.arrived, 2);
            assert_eq!(count.not_there, 1);
        }
        let _ = std::fs::remove_dir_all(&scratch);
    }

    /// **C8: a present name is recognised on every system.**
    #[test]
    fn an_already_present_error_is_the_taken_hard_error() {
        let error = io::Error::from(io::ErrorKind::AlreadyExists);
        assert_eq!(
            the_visible_hard_error_in(&error),
            Some(AVisibleHardError::TheNameIsTaken)
        );
        assert_eq!(
            the_visible_hard_error_in(&io::Error::from(io::ErrorKind::PermissionDenied)),
            None
        );
    }

    /// **C8, the Mac's two measured name errors, from a real create.**
    #[cfg(target_os = "macos")]
    #[test]
    fn the_macs_name_errors_are_visible() {
        let scratch = a_scratch("c8");
        let too_long = scratch.join("a".repeat(400));
        let error = std::fs::File::create(&too_long).expect_err("refused");
        assert_eq!(
            the_visible_hard_error_in(&error),
            Some(AVisibleHardError::TheNameIsTooLong)
        );
        let _ = std::fs::remove_dir_all(&scratch);
    }

    /// **C9's limit, counted.**
    #[test]
    fn thirty_two_combining_marks_are_more_than_measured() {
        let thirty_one = format!("e{}", "\u{0301}".repeat(31));
        let thirty_two = format!("e{}", "\u{0301}".repeat(32));
        assert!(!a_character_carries_more_combining_marks_than_measured(&thirty_one));
        assert!(a_character_carries_more_combining_marks_than_measured(&thirty_two));
        assert!(!a_character_carries_more_combining_marks_than_measured("frame_0001.exr"));
    }
}
