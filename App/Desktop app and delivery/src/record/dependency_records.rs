//! The keeper of `Dependency Upload & Remap`'s record — the store behind that
//! module's `record::TheRecord`, over the tables version 10 of the local record
//! made. `Desktop app and delivery` §5; added 2026-09-29, step 10c's
//! dependency wave piece 4.
//!
//! **What was true before this file**: the eleven `dependency_…` tables stood
//! in every record and nothing read or wrote ten of them. A check's findings,
//! every decision the user made by hand, every placement and every check
//! standing lived in process memory or nowhere, and a reopening lost all of
//! it. `Dependency Upload & Remap` K1 says this record is the only place the
//! artist's file's remap lives and that nothing anywhere holds a copy.
//!
//! **The shapes are that module's and the store is this one's**, the split
//! `job_records` and `neighbouring_records` already stand on. Nothing here
//! decides what an answer means, what a standing stops or which job is live;
//! each is `App/Dependency Upload & Remap/`'s or `App/Queue & History/`'s.
//!
//! # What it keeps, kind by kind
//!
//! - **`Dependency Upload & Remap` K1's five kinds**: the located answer
//!   (`Dependency Upload & Remap` K3, `Dependency Upload & Remap` K4 and
//!   `Dependency Upload & Remap` K5), the shift record (`Dependency Upload & Remap` K6), the
//!   placement record (`Dependency Upload & Remap` K7), the fixing record
//!   (`Dependency Upload & Remap` K8) and the check standing
//!   (`Dependency Upload & Remap` V10).
//! - **`Dependency Upload & Remap` K9's resolved set** and the placed-copy list
//!   of `Dependency Upload & Remap` M22f, both fields of the job's own record
//!   and neither a sixth kind.
//! - **`Dependency Upload & Remap` F1's searched-folder list is not kept a
//!   second time.** Its keeper is `crate::surface::settings::the_list`, written
//!   at step 10a piece 11 and already keeping the user's order across a
//!   reopening; [`the_searched_folder_list`] is the same keeper under this
//!   file's name, so there is one door and one writer. **Moving it here is one
//!   cut in `src/surface/settings.rs`**, which is not this file's to make.
//!
//! # One save per reading, not one per answer
//!
//! **Every save costs about 16 ms on the Creator's Mac** at the record's real
//! settings (`State, identity and recovery` §1.3 rule 6: the full flush and the
//! folder flushed after it) — step 10c transfer wave piece 1's measurement. So
//! [`TheDependencyRecord::the_answers_are_written`] writes a whole reading's
//! findings in one save, whatever their number, and **every write here is one
//! save**, taken as a savepoint so it nests inside a caller's own.
//!
//! **`Dependency Upload & Remap` G16 still holds**: a reading's findings are
//! down before the next reading starts, and a decision the user makes is one
//! save of one answer at the moment they make it (`Dependency Upload & Remap`
//! K4). **What one save per
//! reading costs is the reading, never a sitting**: a line cut mid-reading
//! loses that reading, which `Dependency Upload & Remap` G18 already declares
//! atomic.
//!
//! # Reading it
//!
//! **Every reading answers a failure rather than an empty list** (L8a): a
//! record that would not read is said, never drawn as a record holding
//! nothing. **So `TheRecord` is answered by a snapshot**,
//! [`TheDependencyRecord::what_it_holds_for`], taken for one machine in one
//! call that can fail. Its questions cannot carry a failure, and a store
//! answering them straight off the disk would have to turn every failed read
//! into *nothing was placed here*.
//!
//! **A row that will not read back into the shape it was written as fails the
//! reading whole**, rather than being dropped. A dropped located answer would
//! read as a reference nobody ever decided about, and the user's decision would
//! go without a word.
//!
//! # Where the record and the world can part company, said at each site
//!
//! - **No machine is ever read here** (`Dependency Upload & Remap` V1). A
//!   placement record says what Smoozy published and never that the file is
//!   still there.
//! - **Rows under a superseded installation are kept and answer nothing**
//!   (`State, identity and recovery` §11.1): the keys carry the installation,
//!   and nothing sweeps them.
//! - **A job is live or waiting here where the local job record has learned no
//!   ending for it** — see [`TheDependencyRecord::the_live_or_waiting_jobs_on`].
//! - **A published transfer record is never answered** — see
//!   [`TheDependencyRecord::what_it_holds_for`].
//!
//! **How a clause marker is written in this file**: a marker with no document
//! beside it is this crate's own plan's. Every other document's is written with
//! it at the site, and this file keeps no header list, because this crate's
//! plan defines a K7, a K8, a V1, a V2, a V5 and a V10 of its own.

use std::collections::HashSet;
use std::time::SystemTime;

use rusqlite::{params, Connection, OptionalExtension, Row};

use smoozy_dependencies::bytes::{MachineLocation, RecordedLocation, StoredLocation};
use smoozy_dependencies::check::Kind;
use smoozy_dependencies::contents::ContentIdentity;
use smoozy_dependencies::record::{
    ArbitraryPick, Confirmation, ConfirmationOutcome, FixingRecord, HoldingFile, HowResolved,
    InMemoryRecord, LiveJob, LocatedAnswer, LocatedAnswerKey, MachineKey, PlacementRecord, ReadAs,
    Resolution, ResolvedSet, ResolvedTo, ShiftKey, ShiftRecord,
};
use smoozy_dependencies::remap::{PlacedCopy, PlacedCopyList};
use smoozy_dependencies::standing::{CheckStanding, Completion, Counts};
use smoozy_render_settings::file_half::{version_text, BlenderIdentity};
use smoozy_server_protocol::transfers::integrity::Digest;
use smoozy_server_protocol::transfers::SourceReading;
use smoozy_state::identities::{Identity, IdentityKind};
use smoozy_state::observed::{Clock, Reading};

use super::job_records::JobRecords;
use super::{moment, nanoseconds, RecordFailure};

/// **`Dependency Upload & Remap` F1's list, under this file's name.** The
/// keeper is `crate::surface::settings::the_list` and it is the only writer of
/// `dependency_searched_folder`; see the head of this file.
pub use crate::surface::settings::the_list as the_searched_folder_list;

/// **The keeper, over the record's own connection.**
pub struct TheDependencyRecord<'a> {
    connection: &'a Connection,
}

impl<'a> TheDependencyRecord<'a> {
    pub fn over(connection: &'a Connection) -> Self {
        TheDependencyRecord { connection }
    }
}

// ════════════════════════════════════════════════════════════════════════════
// The one save
// ════════════════════════════════════════════════════════════════════════════

/// **One save**: a savepoint, so it is its own transaction where the caller
/// holds none and nests where it does. **Nothing of a save that failed part
/// way stays**, and the savepoint is let go of either way so the connection is
/// never left inside it — `transfer_records::put`'s shape.
fn in_one_save<T>(
    connection: &Connection,
    name: &str,
    work: impl FnOnce() -> Result<T, RecordFailure>,
) -> Result<T, RecordFailure> {
    connection.execute_batch(&format!("SAVEPOINT {name}"))?;
    let done = work().and_then(|value| {
        connection.execute_batch(&format!("RELEASE {name}"))?;
        Ok(value)
    });
    if done.is_err() {
        let _ = connection.execute_batch(&format!("ROLLBACK TO {name}; RELEASE {name}"));
    }
    done
}

fn a_moment(column: &'static str, at: SystemTime) -> Result<i64, RecordFailure> {
    nanoseconds(at).ok_or_else(|| RecordFailure::AMomentTheRecordCannotHold {
        column,
        reported: format!("{at:?}"),
    })
}

fn not_as_written(what: &str) -> RecordFailure {
    RecordFailure::Engine {
        reported: format!(
            "a dependency record's {what} did not read back into the shape it was written as"
        ),
    }
}

fn an_identity(kind: IdentityKind, text: &str, what: &str) -> Result<Identity, RecordFailure> {
    Identity::read_back(kind, text).ok_or_else(|| not_as_written(what))
}

fn a_machine(server_row: &str, installation: &str) -> Result<MachineKey, RecordFailure> {
    Ok(MachineKey {
        server_row: an_identity(IdentityKind::LocalServer, server_row, "server row")?,
        installation: an_identity(IdentityKind::Installation, installation, "installation")?,
    })
}

fn a_digest(text: &str) -> Result<Digest, RecordFailure> {
    Digest::read_back(text).ok_or_else(|| not_as_written("digest"))
}

/// **A count the record holds as a signed number** — a row count, which is
/// never near the edge; a value that is refused rather than wrapped.
fn a_count(value: usize) -> Result<i64, RecordFailure> {
    i64::try_from(value).map_err(|_| not_as_written("count"))
}

fn a_size(value: u64) -> Result<i64, RecordFailure> {
    i64::try_from(value).map_err(|_| not_as_written("size"))
}

fn on_this_computer(nanoseconds: i64) -> Reading<()> {
    Reading::taken((), moment(nanoseconds), Clock::ThisComputer)
}

// ════════════════════════════════════════════════════════════════════════════
// The located answer — `Dependency Upload & Remap` K3, `Dependency Upload & Remap` K4 and `Dependency Upload & Remap` K5
// ════════════════════════════════════════════════════════════════════════════

/// **The holding file's five key columns.** `Dependency Upload & Remap` K3's
/// key has no machine in it and `Dependency Upload & Remap` K5's has nothing else, so the columns the other
/// case does not use hold the empty value — schema version 10's own note on
/// why a key column is never null.
struct TheHoldingColumns {
    kind: &'static str,
    source_path: Vec<u8>,
    machine_location: Vec<u8>,
    server_row: String,
    installation: String,
}

const ON_THE_USERS_COMPUTER: &str = "on-the-users-computer";
const ON_THE_MACHINE_WITH_NO_KNOWN_LOCAL_ORIGIN: &str = "on-the-machine-with-no-known-local-origin";

fn the_holding_columns(holding: &HoldingFile) -> TheHoldingColumns {
    match holding {
        HoldingFile::OnTheUsersComputer(source) => TheHoldingColumns {
            kind: ON_THE_USERS_COMPUTER,
            source_path: source.as_bytes().to_vec(),
            machine_location: Vec::new(),
            server_row: String::new(),
            installation: String::new(),
        },
        HoldingFile::OnTheMachineWithNoKnownLocalOrigin { machine, location } => {
            TheHoldingColumns {
                kind: ON_THE_MACHINE_WITH_NO_KNOWN_LOCAL_ORIGIN,
                source_path: Vec::new(),
                machine_location: location.as_bytes().to_vec(),
                server_row: machine.server_row.as_text(),
                installation: machine.installation.as_text(),
            }
        }
    }
}

fn the_holding_file_of(columns: &TheHoldingColumns) -> Result<HoldingFile, RecordFailure> {
    match columns.kind {
        ON_THE_USERS_COMPUTER => Ok(HoldingFile::OnTheUsersComputer(
            StoredLocation::as_the_system_returned_it(&columns.source_path),
        )),
        ON_THE_MACHINE_WITH_NO_KNOWN_LOCAL_ORIGIN => {
            Ok(HoldingFile::OnTheMachineWithNoKnownLocalOrigin {
                machine: a_machine(&columns.server_row, &columns.installation)?,
                location: MachineLocation::composed(columns.machine_location.clone()),
            })
        }
        _ => Err(not_as_written("holding file")),
    }
}

/// `Dependency Upload & Remap` K4's four ways, as the record spells them.
const THE_RECORDED_LOCATION_ITSELF: &str = "the-recorded-location-itself";
const THE_SEARCH: &str = "the-search";
const A_SHIFT: &str = "a-shift";
const THE_USERS_OWN_HAND: &str = "the-users-own-hand";

const THE_ANSWERS_COLUMNS: &str = "holding_kind, holding_source_path, holding_machine_location,
     holding_server_row, holding_installation, reference, resolved_to,
     resolved_to_is_a_folder, how, how_searched_folder, how_arbitrary_pick, how_shift,
     set_aside, rejected_guess";

/// One answer's row and its candidates, written inside a save the caller
/// holds. **The answer is replaced whole**: its candidates go first, so an
/// answer that stopped being an arbitrary pick leaves none behind.
fn one_answer_is_put(connection: &Connection, answer: &LocatedAnswer) -> Result<(), RecordFailure> {
    let holding = the_holding_columns(&answer.key.holding_file);
    let reference = answer.key.reference.as_bytes();
    connection
        .prepare_cached(
            "DELETE FROM dependency_located_answer_candidate
             WHERE holding_kind = ?1 AND holding_source_path = ?2
               AND holding_machine_location = ?3 AND holding_server_row = ?4
               AND holding_installation = ?5 AND reference = ?6",
        )?
        .execute(params![
            holding.kind,
            holding.source_path,
            holding.machine_location,
            holding.server_row,
            holding.installation,
            reference
        ])?;

    let (resolved_to, is_a_folder, how, searched_folder, arbitrary, shift) =
        match &answer.resolution {
            None => (None, None, None, None, None, None),
            Some(Resolution { to, how }) => {
                let is_a_folder = matches!(to, ResolvedTo::AFolderForASetRow(_));
                let (how, folder, arbitrary, shift) = match how {
                    HowResolved::TheRecordedLocationItself => {
                        (THE_RECORDED_LOCATION_ITSELF, None, None, None)
                    }
                    HowResolved::TheSearch {
                        from_folder,
                        arbitrary,
                    } => (
                        THE_SEARCH,
                        Some(from_folder.clone()),
                        arbitrary.clone(),
                        None,
                    ),
                    HowResolved::AShift(ShiftKey(key)) => {
                        let key = i64::try_from(*key).map_err(|_| not_as_written("shift key"))?;
                        (A_SHIFT, None, None, Some(key))
                    }
                    HowResolved::TheUsersOwnHand => (THE_USERS_OWN_HAND, None, None, None),
                };
                (
                    Some(to.location().as_bytes().to_vec()),
                    Some(is_a_folder),
                    Some(how),
                    folder,
                    arbitrary,
                    shift,
                )
            }
        };

    connection
        .prepare_cached(&format!(
            "INSERT OR REPLACE INTO dependency_located_answer ({THE_ANSWERS_COLUMNS})
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14)"
        ))?
        .execute(params![
            holding.kind,
            holding.source_path,
            holding.machine_location,
            holding.server_row,
            holding.installation,
            reference,
            resolved_to,
            is_a_folder,
            how,
            searched_folder,
            arbitrary
                .as_ref()
                .map(|pick| pick.taken.as_bytes().to_vec()),
            shift,
            answer.set_aside,
            answer
                .rejected_guess
                .as_ref()
                .map(|guess| guess.as_bytes().to_vec()),
        ])?;

    // **`Dependency Upload & Remap` K4's last limb: what the check read the
    // reference as**, replaced whole with the answer (record version 29).
    connection
        .prepare_cached(
            "DELETE FROM dependency_located_answer_read_as
             WHERE holding_kind = ?1 AND holding_source_path = ?2
               AND holding_machine_location = ?3 AND holding_server_row = ?4
               AND holding_installation = ?5 AND reference = ?6",
        )?
        .execute(params![
            holding.kind,
            holding.source_path,
            holding.machine_location,
            holding.server_row,
            holding.installation,
            reference
        ])?;
    if !answer.read_as.is_empty() {
        let mut insert = connection.prepare_cached(
            "INSERT INTO dependency_located_answer_read_as
                (holding_kind, holding_source_path, holding_machine_location,
                 holding_server_row, holding_installation, reference, position, kind, setting)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
        )?;
        for (position, read_as) in answer.read_as.iter().enumerate() {
            insert.execute(params![
                holding.kind,
                holding.source_path,
                holding.machine_location,
                holding.server_row,
                holding.installation,
                reference,
                a_count(position)?,
                a_kinds_text(read_as.kind)?,
                read_as.setting,
            ])?;
        }
    }

    // **`Dependency Upload & Remap` F5: the candidates are written only where
    // the pick was arbitrary** — schema version 10's own rule for the table.
    if let Some(pick) = arbitrary {
        let mut insert = connection.prepare_cached(
            "INSERT INTO dependency_located_answer_candidate
                (holding_kind, holding_source_path, holding_machine_location,
                 holding_server_row, holding_installation, reference, position, candidate)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
        )?;
        for (position, candidate) in pick.among.iter().enumerate() {
            insert.execute(params![
                holding.kind,
                holding.source_path,
                holding.machine_location,
                holding.server_row,
                holding.installation,
                reference,
                a_count(position)?,
                candidate.as_bytes(),
            ])?;
        }
    }
    Ok(())
}

/// One answer's row as it came off the store, before its candidates.
struct AnAnswersRow {
    holding: TheHoldingColumns,
    reference: Vec<u8>,
    resolved_to: Option<Vec<u8>>,
    is_a_folder: Option<bool>,
    how: Option<String>,
    searched_folder: Option<Vec<u8>>,
    arbitrary_pick: Option<Vec<u8>>,
    shift: Option<i64>,
    set_aside: bool,
    rejected_guess: Option<Vec<u8>>,
}

fn an_answers_row(row: &Row<'_>) -> rusqlite::Result<AnAnswersRow> {
    let kind: String = row.get(0)?;
    Ok(AnAnswersRow {
        holding: TheHoldingColumns {
            // The kind is compared, never kept as text of its own.
            kind: match kind.as_str() {
                ON_THE_USERS_COMPUTER => ON_THE_USERS_COMPUTER,
                ON_THE_MACHINE_WITH_NO_KNOWN_LOCAL_ORIGIN => {
                    ON_THE_MACHINE_WITH_NO_KNOWN_LOCAL_ORIGIN
                }
                _ => "",
            },
            source_path: row.get(1)?,
            machine_location: row.get(2)?,
            server_row: row.get(3)?,
            installation: row.get(4)?,
        },
        reference: row.get(5)?,
        resolved_to: row.get(6)?,
        is_a_folder: row.get(7)?,
        how: row.get(8)?,
        searched_folder: row.get(9)?,
        arbitrary_pick: row.get(10)?,
        shift: row.get(11)?,
        set_aside: row.get(12)?,
        rejected_guess: row.get(13)?,
    })
}

fn the_candidates_of(
    connection: &Connection,
    row: &AnAnswersRow,
) -> Result<Vec<StoredLocation>, RecordFailure> {
    let mut statement = connection.prepare_cached(
        "SELECT candidate FROM dependency_located_answer_candidate
         WHERE holding_kind = ?1 AND holding_source_path = ?2
           AND holding_machine_location = ?3 AND holding_server_row = ?4
           AND holding_installation = ?5 AND reference = ?6
         ORDER BY position",
    )?;
    let candidates = statement
        .query_map(
            params![
                row.holding.kind,
                row.holding.source_path,
                row.holding.machine_location,
                row.holding.server_row,
                row.holding.installation,
                row.reference
            ],
            |candidate| candidate.get::<_, Vec<u8>>(0),
        )?
        .collect::<rusqlite::Result<Vec<_>>>()?;
    Ok(candidates
        .iter()
        .map(|bytes| StoredLocation::as_the_system_returned_it(bytes))
        .collect())
}

/// **A kind as the record spells it: the variant's own name**, which is also
/// the spelling the check's reading carries it in (the kind's derived
/// serialisation renames nothing), so there is no second list of names to
/// keep. The test `a_kinds_spelling_here_is_the_readings_own` holds the two
/// together.
fn a_kinds_text(kind: Kind) -> Result<String, RecordFailure> {
    Ok(format!("{kind:?}"))
}

/// **A spelling that names no kind fails the reading whole** (see the head).
fn the_kind_spelled(text: String) -> Result<Kind, RecordFailure> {
    Kind::ALL
        .into_iter()
        .find(|kind| format!("{kind:?}") == text)
        .ok_or_else(|| not_as_written("kind"))
}

/// **`Dependency Upload & Remap` K4's last limb for one answer**, in the
/// reading's order. **A kind the record cannot read back fails the whole
/// reading** (see the head): dropped, the pair it gives would be missing
/// with nothing said.
fn what_it_was_read_as(
    connection: &Connection,
    row: &AnAnswersRow,
) -> Result<Vec<ReadAs>, RecordFailure> {
    let mut statement = connection.prepare_cached(
        "SELECT kind, setting FROM dependency_located_answer_read_as
         WHERE holding_kind = ?1 AND holding_source_path = ?2
           AND holding_machine_location = ?3 AND holding_server_row = ?4
           AND holding_installation = ?5 AND reference = ?6
         ORDER BY position",
    )?;
    let rows = statement
        .query_map(
            params![
                row.holding.kind,
                row.holding.source_path,
                row.holding.machine_location,
                row.holding.server_row,
                row.holding.installation,
                row.reference
            ],
            |read| Ok((read.get::<_, String>(0)?, read.get::<_, String>(1)?)),
        )?
        .collect::<rusqlite::Result<Vec<_>>>()?;
    rows.into_iter()
        .map(|(kind, setting)| {
            Ok(ReadAs {
                kind: the_kind_spelled(kind)?,
                setting,
            })
        })
        .collect()
}

fn the_answer_from(
    connection: &Connection,
    row: AnAnswersRow,
) -> Result<LocatedAnswer, RecordFailure> {
    let resolution = match (&row.resolved_to, row.is_a_folder, row.how.as_deref()) {
        (None, None, None) => None,
        (Some(to), Some(is_a_folder), Some(how)) => {
            let at = StoredLocation::as_the_system_returned_it(to);
            let to = if is_a_folder {
                ResolvedTo::AFolderForASetRow(at)
            } else {
                ResolvedTo::AWholeLocation(at)
            };
            let how = match how {
                THE_RECORDED_LOCATION_ITSELF => HowResolved::TheRecordedLocationItself,
                THE_SEARCH => HowResolved::TheSearch {
                    from_folder: row
                        .searched_folder
                        .clone()
                        .ok_or_else(|| not_as_written("searched folder"))?,
                    arbitrary: match &row.arbitrary_pick {
                        None => None,
                        Some(taken) => Some(ArbitraryPick {
                            taken: StoredLocation::as_the_system_returned_it(taken),
                            among: the_candidates_of(connection, &row)?,
                        }),
                    },
                },
                A_SHIFT => HowResolved::AShift(ShiftKey(
                    row.shift
                        .and_then(|key| u64::try_from(key).ok())
                        .ok_or_else(|| not_as_written("shift key"))?,
                )),
                THE_USERS_OWN_HAND => HowResolved::TheUsersOwnHand,
                _ => return Err(not_as_written("way it was resolved")),
            };
            Some(Resolution { to, how })
        }
        _ => return Err(not_as_written("resolution")),
    };
    Ok(LocatedAnswer {
        key: LocatedAnswerKey {
            holding_file: the_holding_file_of(&row.holding)?,
            reference: RecordedLocation::as_the_reading_carried_it(&row.reference),
        },
        resolution,
        set_aside: row.set_aside,
        rejected_guess: row
            .rejected_guess
            .as_deref()
            .map(StoredLocation::as_the_system_returned_it),
        read_as: what_it_was_read_as(connection, &row)?,
    })
}

impl TheDependencyRecord<'_> {
    /// **A reading's findings, or one decision, in one save** — however many
    /// answers there are (`Dependency Upload & Remap` K4 and `Dependency Upload & Remap` G16; see the head of
    /// this file for why one save).
    ///
    /// **Each answer is replaced whole under its key**, which is
    /// `Dependency Upload & Remap` K3's: the holding file and the reference's
    /// exact bytes, capitals included. **So the caller composes each answer
    /// from what the record already holds** — `Dependency Upload & Remap` G17: a fresh check
    /// starts from every located answer the record supplies, and nothing is
    /// re-decided. A set-aside mark or a rejected guess handed in absent is
    /// written absent.
    ///
    /// **An answer with no resolution is a *not found* row**
    /// (`Dependency Upload & Remap` F7) and is written as one: the reference
    /// was found and nothing was found for it.
    pub fn the_answers_are_written(&self, answers: &[LocatedAnswer]) -> Result<(), RecordFailure> {
        in_one_save(self.connection, "dependency_answers_are_written", || {
            for answer in answers {
                one_answer_is_put(self.connection, answer)?;
            }
            Ok(())
        })
    }

    /// **Every answer one holding file has**, in the order of the reference's
    /// bytes — `Dependency Upload & Remap` G11a's order for the panel's rows.
    pub fn the_answers_of(
        &self,
        holding: &HoldingFile,
    ) -> Result<Vec<LocatedAnswer>, RecordFailure> {
        let key = the_holding_columns(holding);
        let rows = self
            .connection
            .prepare_cached(&format!(
                "SELECT {THE_ANSWERS_COLUMNS} FROM dependency_located_answer
                 WHERE holding_kind = ?1 AND holding_source_path = ?2
                   AND holding_machine_location = ?3 AND holding_server_row = ?4
                   AND holding_installation = ?5
                 ORDER BY reference"
            ))?
            .query_map(
                params![
                    key.kind,
                    key.source_path,
                    key.machine_location,
                    key.server_row,
                    key.installation
                ],
                an_answers_row,
            )?
            .collect::<rusqlite::Result<Vec<_>>>()?;
        rows.into_iter()
            .map(|row| the_answer_from(self.connection, row))
            .collect()
    }

    /// **Every answer the record holds**, for [`Self::what_it_holds_for`].
    /// Located answers carry no machine (`Dependency Upload & Remap` K3), so a
    /// snapshot for one machine holds them all.
    fn every_answer(&self) -> Result<Vec<LocatedAnswer>, RecordFailure> {
        let rows = self
            .connection
            .prepare_cached(&format!(
                "SELECT {THE_ANSWERS_COLUMNS} FROM dependency_located_answer
                 ORDER BY holding_kind, holding_source_path, holding_machine_location,
                          holding_server_row, holding_installation, reference"
            ))?
            .query_map([], an_answers_row)?
            .collect::<rusqlite::Result<Vec<_>>>()?;
        rows.into_iter()
            .map(|row| the_answer_from(self.connection, row))
            .collect()
    }
}

// ════════════════════════════════════════════════════════════════════════════
// The shift record — `Dependency Upload & Remap` K6
// ════════════════════════════════════════════════════════════════════════════

impl TheDependencyRecord<'_> {
    /// **A shift taught by a hand-locate, written with the key the record
    /// gives it** — the next number after every shift ever written.
    ///
    /// **`Dependency Upload & Remap` K6: the key is a record key and not an
    /// identity**, and nothing derives anything from it. Shift records are
    /// never removed (`Dependency Upload & Remap` F20's undo returns the answers and keeps the record),
    /// so a key is never given twice.
    ///
    /// **The answers the shift resolves are a second write**,
    /// [`Self::the_answers_are_written`]. `Dependency Upload & Remap` F20 makes
    /// a shift stopped part way leave only answers naming it, so the two need
    /// not share a save.
    pub fn a_shift_is_taught(
        &self,
        old_root: Vec<u8>,
        new_root: Vec<u8>,
        at: SystemTime,
    ) -> Result<ShiftRecord, RecordFailure> {
        let written_at = a_moment("dependency_shift.written_at", at)?;
        in_one_save(self.connection, "dependency_shift_is_taught", || {
            let key: i64 = self.connection.query_row(
                "SELECT COALESCE(MAX(shift), 0) + 1 FROM dependency_shift",
                [],
                |row| row.get(0),
            )?;
            self.connection.execute(
                "INSERT INTO dependency_shift (shift, old_root, new_root, written_at)
                 VALUES (?1, ?2, ?3, ?4)",
                params![key, old_root, new_root, written_at],
            )?;
            Ok(ShiftRecord {
                key: ShiftKey(u64::try_from(key).map_err(|_| not_as_written("shift key"))?),
                old_root: old_root.clone(),
                new_root: new_root.clone(),
                at: on_this_computer(written_at),
            })
        })
    }

    pub fn the_shift(&self, key: ShiftKey) -> Result<Option<ShiftRecord>, RecordFailure> {
        let Ok(stored) = i64::try_from(key.0) else {
            return Ok(None);
        };
        let row = self
            .connection
            .query_row(
                "SELECT shift, old_root, new_root, written_at FROM dependency_shift
                 WHERE shift = ?1",
                [stored],
                a_shift_row,
            )
            .optional()?;
        row.map(the_shift_from).transpose()
    }

    fn every_shift(&self) -> Result<Vec<ShiftRecord>, RecordFailure> {
        let rows = self
            .connection
            .prepare_cached(
                "SELECT shift, old_root, new_root, written_at FROM dependency_shift
                 ORDER BY shift",
            )?
            .query_map([], a_shift_row)?
            .collect::<rusqlite::Result<Vec<_>>>()?;
        rows.into_iter().map(the_shift_from).collect()
    }
}

type AShiftRow = (i64, Vec<u8>, Vec<u8>, i64);

fn a_shift_row(row: &Row<'_>) -> rusqlite::Result<AShiftRow> {
    Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?))
}

fn the_shift_from((key, old_root, new_root, at): AShiftRow) -> Result<ShiftRecord, RecordFailure> {
    Ok(ShiftRecord {
        key: ShiftKey(u64::try_from(key).map_err(|_| not_as_written("shift key"))?),
        old_root,
        new_root,
        at: on_this_computer(at),
    })
}

// ════════════════════════════════════════════════════════════════════════════
// The placement record — `Dependency Upload & Remap` K7
// ════════════════════════════════════════════════════════════════════════════

const CONFIRMED: &str = "confirmed";
const GONE: &str = "gone";
const NOT_CONFIRMED: &str = "not-confirmed";

impl TheDependencyRecord<'_> {
    /// **A placement record, written at its publication or rewritten at a
    /// confirmation** — one per (machine, mirrored location), the key
    /// `Dependency Upload & Remap` K7 gives it, so a second write under the
    /// same key is the same record moved on (`Dependency Upload & Remap` Q7).
    pub fn a_placement_is_written(&self, placement: &PlacementRecord) -> Result<(), RecordFailure> {
        let (outcome, at) = match &placement.confirmation {
            None => (None, None),
            Some(Confirmation { outcome, at }) => (
                Some(match outcome {
                    ConfirmationOutcome::Confirmed => CONFIRMED,
                    ConfirmationOutcome::Gone => GONE,
                    ConfirmationOutcome::NotConfirmed => NOT_CONFIRMED,
                }),
                Some(a_moment(
                    "dependency_placement.confirmation_at",
                    at.taken_at(),
                )?),
            ),
        };
        let content_size = a_size(placement.content.size)?;
        let published_size = a_size(placement.four_values_at_publication.size)?;
        in_one_save(self.connection, "dependency_placement_is_written", || {
            self.connection
                .prepare_cached(
                    "INSERT OR REPLACE INTO dependency_placement
                        (server_row, installation, mirrored_location, source_location,
                         content_size, content_digest, published_size, published_inode,
                         published_modification_time, published_change_time,
                         confirmation_outcome, confirmation_at, marked_gone)
                     VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13)",
                )?
                .execute(params![
                    placement.machine.server_row.as_text(),
                    placement.machine.installation.as_text(),
                    placement.mirrored_location.as_bytes(),
                    placement.source_location.as_bytes(),
                    content_size,
                    placement.content.digest.as_text(),
                    published_size,
                    placement.four_values_at_publication.inode,
                    placement.four_values_at_publication.modification_time,
                    placement.four_values_at_publication.change_time,
                    outcome,
                    at,
                    placement.marked_gone,
                ])?;
            Ok(())
        })
    }

    /// **Every placement record under one machine key** — the server row and
    /// the installation together, so a record left under a superseded
    /// installation is not among them (`Dependency Upload & Remap` K7).
    pub fn the_placements_on(
        &self,
        machine: &MachineKey,
    ) -> Result<Vec<PlacementRecord>, RecordFailure> {
        let mut statement = self.connection.prepare_cached(
            "SELECT mirrored_location, source_location, content_size, content_digest,
                    published_size, published_inode, published_modification_time,
                    published_change_time, confirmation_outcome, confirmation_at, marked_gone
             FROM dependency_placement
             WHERE server_row = ?1 AND installation = ?2
             ORDER BY mirrored_location",
        )?;
        type ARow = (
            Vec<u8>,
            Vec<u8>,
            i64,
            String,
            i64,
            String,
            i64,
            i64,
            Option<String>,
            Option<i64>,
            bool,
        );
        let rows = statement
            .query_map(
                params![machine.server_row.as_text(), machine.installation.as_text()],
                |row| -> rusqlite::Result<ARow> {
                    Ok((
                        row.get(0)?,
                        row.get(1)?,
                        row.get(2)?,
                        row.get(3)?,
                        row.get(4)?,
                        row.get(5)?,
                        row.get(6)?,
                        row.get(7)?,
                        row.get(8)?,
                        row.get(9)?,
                        row.get(10)?,
                    ))
                },
            )?
            .collect::<rusqlite::Result<Vec<_>>>()?;
        rows.into_iter()
            .map(
                |(
                    mirror,
                    source,
                    size,
                    digest,
                    published,
                    inode,
                    modified,
                    changed,
                    outcome,
                    at,
                    gone,
                )| {
                    let confirmation = match (outcome.as_deref(), at) {
                        (None, None) => None,
                        (Some(outcome), Some(at)) => Some(Confirmation {
                            outcome: match outcome {
                                CONFIRMED => ConfirmationOutcome::Confirmed,
                                GONE => ConfirmationOutcome::Gone,
                                NOT_CONFIRMED => ConfirmationOutcome::NotConfirmed,
                                _ => return Err(not_as_written("confirmation")),
                            },
                            at: on_this_computer(at),
                        }),
                        _ => return Err(not_as_written("confirmation")),
                    };
                    Ok(PlacementRecord {
                        machine: machine.clone(),
                        mirrored_location: MachineLocation::composed(mirror),
                        source_location: StoredLocation::as_the_system_returned_it(&source),
                        content: ContentIdentity {
                            size: u64::try_from(size).map_err(|_| not_as_written("size"))?,
                            digest: a_digest(&digest)?,
                        },
                        four_values_at_publication: SourceReading {
                            size: u64::try_from(published).map_err(|_| not_as_written("size"))?,
                            inode,
                            modification_time: modified,
                            change_time: changed,
                        },
                        confirmation,
                        marked_gone: gone,
                    })
                },
            )
            .collect()
    }
}

// ════════════════════════════════════════════════════════════════════════════
// The fixing record — `Dependency Upload & Remap` K8
// ════════════════════════════════════════════════════════════════════════════

impl TheDependencyRecord<'_> {
    /// **A fixing record and what its fixing points at, in one save.** One per
    /// (machine, fixed copy), `Dependency Upload & Remap` K8's key; a second
    /// write is the fixed copy re-produced, and its pairs replace the old ones
    /// whole, in their order.
    pub fn a_fixing_is_written(&self, fixing: &FixingRecord) -> Result<(), RecordFailure> {
        let server_row = fixing.machine.server_row.as_text();
        let installation = fixing.machine.installation.as_text();
        let size = a_size(fixing.derived_from_content.size)?;
        in_one_save(self.connection, "dependency_fixing_is_written", || {
            self.connection
                .prepare_cached(
                    "INSERT OR REPLACE INTO dependency_fixing
                        (server_row, installation, fixed_copy, derived_from,
                         derived_from_size, derived_from_digest)
                     VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
                )?
                .execute(params![
                    server_row,
                    installation,
                    fixing.fixed_copy.as_bytes(),
                    fixing.derived_from.as_bytes(),
                    size,
                    fixing.derived_from_content.digest.as_text(),
                ])?;
            self.connection
                .prepare_cached(
                    "DELETE FROM dependency_fixing_points_at
                     WHERE server_row = ?1 AND installation = ?2 AND fixed_copy = ?3",
                )?
                .execute(params![
                    server_row,
                    installation,
                    fixing.fixed_copy.as_bytes()
                ])?;
            let mut insert = self.connection.prepare_cached(
                "INSERT INTO dependency_fixing_points_at
                    (server_row, installation, fixed_copy, position, reference, machine_location)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            )?;
            for (position, (reference, at)) in fixing.points_at.iter().enumerate() {
                insert.execute(params![
                    server_row,
                    installation,
                    fixing.fixed_copy.as_bytes(),
                    a_count(position)?,
                    reference.as_bytes(),
                    at.as_bytes(),
                ])?;
            }
            Ok(())
        })
    }

    pub fn the_fixings_on(&self, machine: &MachineKey) -> Result<Vec<FixingRecord>, RecordFailure> {
        let server_row = machine.server_row.as_text();
        let installation = machine.installation.as_text();
        let rows = self
            .connection
            .prepare_cached(
                "SELECT fixed_copy, derived_from, derived_from_size, derived_from_digest
                 FROM dependency_fixing
                 WHERE server_row = ?1 AND installation = ?2
                 ORDER BY fixed_copy",
            )?
            .query_map(params![server_row, installation], |row| {
                Ok((
                    row.get::<_, Vec<u8>>(0)?,
                    row.get::<_, Vec<u8>>(1)?,
                    row.get::<_, i64>(2)?,
                    row.get::<_, String>(3)?,
                ))
            })?
            .collect::<rusqlite::Result<Vec<_>>>()?;
        let mut pairs = self.connection.prepare_cached(
            "SELECT reference, machine_location FROM dependency_fixing_points_at
             WHERE server_row = ?1 AND installation = ?2 AND fixed_copy = ?3
             ORDER BY position",
        )?;
        let mut out = Vec::with_capacity(rows.len());
        for (fixed_copy, derived_from, size, digest) in rows {
            let points_at = pairs
                .query_map(params![server_row, installation, fixed_copy], |row| {
                    Ok((row.get::<_, Vec<u8>>(0)?, row.get::<_, Vec<u8>>(1)?))
                })?
                .collect::<rusqlite::Result<Vec<_>>>()?
                .into_iter()
                .map(|(reference, at)| {
                    (
                        RecordedLocation::as_the_reading_carried_it(&reference),
                        MachineLocation::composed(at),
                    )
                })
                .collect();
            out.push(FixingRecord {
                machine: machine.clone(),
                derived_from: MachineLocation::composed(derived_from),
                derived_from_content: ContentIdentity {
                    size: u64::try_from(size).map_err(|_| not_as_written("size"))?,
                    digest: a_digest(&digest)?,
                },
                fixed_copy: MachineLocation::composed(fixed_copy),
                points_at,
            });
        }
        Ok(out)
    }
}

// ════════════════════════════════════════════════════════════════════════════
// The check standing — `Dependency Upload & Remap` V10 to V14
// ════════════════════════════════════════════════════════════════════════════

const COMPLETED: &str = "completed";
const STOPPED_BY_THE_USER: &str = "stopped-by-the-user";
const THE_CONNECTION_DROPPED: &str = "the-connection-dropped";

const THE_STANDINGS_COLUMNS: &str = "root, read_with_build_path, read_with_version, completion,
     completed_at, count_not_found, count_could_not_be_sent, count_could_not_be_fixed,
     count_found_but_not_sent, count_set_aside, count_confirmed, root_size, root_inode,
     root_modification_time, root_change_time, every_row_confirmed_or_set_aside,
     replaced_since_check, disturbed";

impl TheDependencyRecord<'_> {
    /// **A check standing, written at the sitting's end or rewritten with a
    /// mark** — one per (root file's machine path, machine under its
    /// installation), `Dependency Upload & Remap` V10's key.
    ///
    /// **The two marks are columns, never an absence** (`Dependency Upload & Remap`
    /// V14): *nobody has
    /// ever checked this file on this machine* is no row, and *checked and
    /// replaced since* is a row carrying its mark. **A mark is written by
    /// writing the standing again** with it set, which is how
    /// `neighbouring_records::TheNeighbours` hands a marked standing back.
    pub fn a_standing_is_written(&self, standing: &CheckStanding) -> Result<(), RecordFailure> {
        let completed_at = a_moment(
            "dependency_check_standing.completed_at",
            standing.at.taken_at(),
        )?;
        let counts = &standing.counts;
        let numbers = [
            a_count(counts.not_found)?,
            a_count(counts.could_not_be_sent)?,
            a_count(counts.could_not_be_fixed)?,
            a_count(counts.found_but_not_sent)?,
            a_count(counts.set_aside)?,
            a_count(counts.confirmed)?,
        ];
        let root_size = a_size(standing.root_four_values.size)?;
        in_one_save(self.connection, "dependency_standing_is_written", || {
            self.connection
                .prepare_cached(&format!(
                    "INSERT OR REPLACE INTO dependency_check_standing
                        (server_row, installation, {THE_STANDINGS_COLUMNS})
                     VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14,
                             ?15, ?16, ?17, ?18, ?19, ?20)"
                ))?
                .execute(params![
                    standing.machine.server_row.as_text(),
                    standing.machine.installation.as_text(),
                    standing.root.as_bytes(),
                    standing.read_with.path,
                    version_text(standing.read_with.version),
                    match standing.completion {
                        Completion::Completed => COMPLETED,
                        Completion::StoppedByTheUser => STOPPED_BY_THE_USER,
                        Completion::TheConnectionDropped => THE_CONNECTION_DROPPED,
                    },
                    completed_at,
                    numbers[0],
                    numbers[1],
                    numbers[2],
                    numbers[3],
                    numbers[4],
                    numbers[5],
                    root_size,
                    standing.root_four_values.inode,
                    standing.root_four_values.modification_time,
                    standing.root_four_values.change_time,
                    standing.every_row_confirmed_or_set_aside,
                    standing.replaced_since_check,
                    standing.disturbed,
                ])?;
            Ok(())
        })
    }

    /// **Every standing under one machine key**, for
    /// `neighbouring_records::TheNeighbours`, which marks them in place.
    pub fn the_standings_on(
        &self,
        machine: &MachineKey,
    ) -> Result<Vec<CheckStanding>, RecordFailure> {
        let rows = self
            .connection
            .prepare_cached(&format!(
                "SELECT {THE_STANDINGS_COLUMNS} FROM dependency_check_standing
                 WHERE server_row = ?1 AND installation = ?2
                 ORDER BY root"
            ))?
            .query_map(
                params![machine.server_row.as_text(), machine.installation.as_text()],
                a_standings_row,
            )?
            .collect::<rusqlite::Result<Vec<_>>>()?;
        rows.into_iter()
            .map(|row| the_standing_from(machine, row))
            .collect()
    }

    /// **The standing of one root on one machine**, or none — which is
    /// `Dependency Upload & Remap` V14's *nobody has ever checked this file on
    /// this machine*.
    pub fn the_standing(
        &self,
        root: &MachineLocation,
        machine: &MachineKey,
    ) -> Result<Option<CheckStanding>, RecordFailure> {
        let row = self
            .connection
            .query_row(
                &format!(
                    "SELECT {THE_STANDINGS_COLUMNS} FROM dependency_check_standing
                     WHERE server_row = ?1 AND installation = ?2 AND root = ?3"
                ),
                params![
                    machine.server_row.as_text(),
                    machine.installation.as_text(),
                    root.as_bytes()
                ],
                a_standings_row,
            )
            .optional()?;
        row.map(|row| the_standing_from(machine, row)).transpose()
    }
}

/// One standing's row: the root, the Blender, the completion and its moment,
/// the six counts, the four values and the three flags.
struct AStandingsRow {
    root: Vec<u8>,
    build_path: String,
    version: String,
    completion: String,
    completed_at: Option<i64>,
    counts: [i64; 6],
    four_values: (i64, String, i64, i64),
    flags: [bool; 3],
}

fn a_standings_row(row: &Row<'_>) -> rusqlite::Result<AStandingsRow> {
    Ok(AStandingsRow {
        root: row.get(0)?,
        build_path: row.get(1)?,
        version: row.get(2)?,
        completion: row.get(3)?,
        completed_at: row.get(4)?,
        counts: [
            row.get(5)?,
            row.get(6)?,
            row.get(7)?,
            row.get(8)?,
            row.get(9)?,
            row.get(10)?,
        ],
        four_values: (row.get(11)?, row.get(12)?, row.get(13)?, row.get(14)?),
        flags: [row.get(15)?, row.get(16)?, row.get(17)?],
    })
}

fn the_standing_from(
    machine: &MachineKey,
    row: AStandingsRow,
) -> Result<CheckStanding, RecordFailure> {
    let count = |at: usize| usize::try_from(row.counts[at]).map_err(|_| not_as_written("count"));
    let (size, inode, modified, changed) = row.four_values;
    Ok(CheckStanding {
        root: MachineLocation::composed(row.root),
        machine: machine.clone(),
        read_with: BlenderIdentity {
            path: row.build_path,
            version: smoozy_server_setup::blender::Version::parse(&row.version)
                .ok_or_else(|| not_as_written("Blender version"))?,
        },
        // **Every standing this file writes carries its moment**; a row with
        // none was not written here, and is not read as one that was.
        at: on_this_computer(row.completed_at.ok_or_else(|| not_as_written("moment"))?),
        completion: match row.completion.as_str() {
            COMPLETED => Completion::Completed,
            STOPPED_BY_THE_USER => Completion::StoppedByTheUser,
            THE_CONNECTION_DROPPED => Completion::TheConnectionDropped,
            _ => return Err(not_as_written("completion")),
        },
        counts: Counts {
            not_found: count(0)?,
            could_not_be_sent: count(1)?,
            could_not_be_fixed: count(2)?,
            found_but_not_sent: count(3)?,
            set_aside: count(4)?,
            confirmed: count(5)?,
        },
        every_row_confirmed_or_set_aside: row.flags[0],
        root_four_values: SourceReading {
            size: u64::try_from(size).map_err(|_| not_as_written("size"))?,
            inode,
            modification_time: modified,
            change_time: changed,
        },
        replaced_since_check: row.flags[1],
        disturbed: row.flags[2],
    })
}

// ════════════════════════════════════════════════════════════════════════════
// The job's two fields — `Dependency Upload & Remap` K9 and `Dependency Upload & Remap` M22f
// ════════════════════════════════════════════════════════════════════════════

impl TheDependencyRecord<'_> {
    /// **A job's resolved set, written once** — `Dependency Upload & Remap`
    /// K9: composed at submission, **immutable for the life of the job's
    /// record**. A second write under the same job is refused by the store and
    /// the first stands.
    pub fn a_jobs_resolved_set_is_written(
        &self,
        job: &Identity,
        machine: &MachineKey,
        blend: &MachineLocation,
        set: &ResolvedSet,
    ) -> Result<(), RecordFailure> {
        in_one_save(
            self.connection,
            "dependency_resolved_set_is_written",
            || {
                self.connection.execute(
                    "INSERT INTO dependency_job_resolved_set
                    (job, server_row, installation, remap_material, blend)
                 VALUES (?1, ?2, ?3, ?4, ?5)",
                    params![
                        job.as_text(),
                        machine.server_row.as_text(),
                        machine.installation.as_text(),
                        set.remap_material.as_bytes(),
                        blend.as_bytes(),
                    ],
                )?;
                let mut insert = self.connection.prepare_cached(
                    "INSERT INTO dependency_job_resolved_location (job, position, machine_location)
                 VALUES (?1, ?2, ?3)",
                )?;
                for (position, at) in set.locations.iter().enumerate() {
                    insert.execute(params![job.as_text(), a_count(position)?, at.as_bytes()])?;
                }
                Ok(())
            },
        )
    }

    /// **`Dependency Upload & Remap` V5's whole answer for every live or
    /// waiting job on one machine**: its `.blend` and its resolved set.
    ///
    /// **Live or waiting is read from the local job record alone** —
    /// `Dependency Upload & Remap` V1 answers from the record with no read of
    /// the machine — **as a job for which the record has learned no ending**
    /// (`JobRecords::with_no_learned_ending_for`, `Queue & History`'s own
    /// reading of its record). **This is where the record and the world can
    /// part company, and it parts on the wide side**: a job that has ended on
    /// the machine while nobody was connected still reads live here until its
    /// ending is learned, so a warning names it. The warnings warn and never
    /// block (`Dependency Upload & Remap` V3), so naming one job too many costs
    /// a line on a warning; missing one would let a live job's file be touched
    /// unwarned. **A job with no job record at all is not live**: nothing
    /// tracks it any more.
    pub fn the_live_or_waiting_jobs_on(
        &self,
        machine: &MachineKey,
    ) -> Result<Vec<LiveJob>, RecordFailure> {
        let not_ended: HashSet<String> = JobRecords::over(self.connection)
            .with_no_learned_ending_for(&machine.server_row)?
            .into_iter()
            .map(|record| record.job.as_text())
            .collect();
        let sets = self
            .connection
            .prepare_cached(
                "SELECT job, remap_material, blend FROM dependency_job_resolved_set
                 WHERE server_row = ?1 AND installation = ?2
                 ORDER BY job",
            )?
            .query_map(
                params![machine.server_row.as_text(), machine.installation.as_text()],
                |row| {
                    Ok((
                        row.get::<_, String>(0)?,
                        row.get::<_, Vec<u8>>(1)?,
                        row.get::<_, Vec<u8>>(2)?,
                    ))
                },
            )?
            .collect::<rusqlite::Result<Vec<_>>>()?;
        let mut locations = self.connection.prepare_cached(
            "SELECT machine_location FROM dependency_job_resolved_location
             WHERE job = ?1 ORDER BY position",
        )?;
        let mut out = Vec::new();
        for (job, material, blend) in sets {
            if !not_ended.contains(&job) {
                continue;
            }
            let named = locations
                .query_map([&job], |row| row.get::<_, Vec<u8>>(0))?
                .collect::<rusqlite::Result<Vec<_>>>()?;
            out.push(LiveJob {
                job: an_identity(IdentityKind::Job, &job, "job")?,
                machine: machine.clone(),
                blend: MachineLocation::composed(blend),
                resolved_set: ResolvedSet {
                    locations: named.into_iter().map(MachineLocation::composed).collect(),
                    remap_material: MachineLocation::composed(material),
                },
            });
        }
        Ok(out)
    }

    /// **`Dependency Upload & Remap` M22f's placed-copy list, written whole** —
    /// each copy's job-folder path and its four values the moment it was made.
    pub fn the_placed_copies_are_written(
        &self,
        job: &Identity,
        copies: &PlacedCopyList,
    ) -> Result<(), RecordFailure> {
        in_one_save(
            self.connection,
            "dependency_placed_copies_are_written",
            || {
                self.connection.execute(
                    "DELETE FROM dependency_job_placed_copy WHERE job = ?1",
                    [job.as_text()],
                )?;
                let mut insert = self.connection.prepare_cached(
                    "INSERT INTO dependency_job_placed_copy
                    (job, position, job_folder_path, made_size, made_inode,
                     made_modification_time, made_change_time)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
                )?;
                for (position, copy) in copies.copies.iter().enumerate() {
                    insert.execute(params![
                        job.as_text(),
                        a_count(position)?,
                        copy.job_folder_path.as_bytes(),
                        a_size(copy.four_values_at_placement.size)?,
                        copy.four_values_at_placement.inode,
                        copy.four_values_at_placement.modification_time,
                        copy.four_values_at_placement.change_time,
                    ])?;
                }
                Ok(())
            },
        )
    }

    pub fn the_placed_copies_of(&self, job: &Identity) -> Result<PlacedCopyList, RecordFailure> {
        let rows = self
            .connection
            .prepare_cached(
                "SELECT job_folder_path, made_size, made_inode, made_modification_time,
                        made_change_time
                 FROM dependency_job_placed_copy WHERE job = ?1 ORDER BY position",
            )?
            .query_map([job.as_text()], |row| {
                Ok((
                    row.get::<_, Vec<u8>>(0)?,
                    row.get::<_, i64>(1)?,
                    row.get::<_, String>(2)?,
                    row.get::<_, i64>(3)?,
                    row.get::<_, i64>(4)?,
                ))
            })?
            .collect::<rusqlite::Result<Vec<_>>>()?;
        Ok(PlacedCopyList {
            copies: rows
                .into_iter()
                .map(|(path, size, inode, modified, changed)| {
                    Ok(PlacedCopy {
                        job_folder_path: MachineLocation::composed(path),
                        four_values_at_placement: SourceReading {
                            size: u64::try_from(size).map_err(|_| not_as_written("size"))?,
                            inode,
                            modification_time: modified,
                            change_time: changed,
                        },
                    })
                })
                .collect::<Result<_, RecordFailure>>()?,
        })
    }
}

// ════════════════════════════════════════════════════════════════════════════
// `TheRecord`, answered for one machine
// ════════════════════════════════════════════════════════════════════════════

impl TheDependencyRecord<'_> {
    /// **What the record holds for one machine, read in one call that can
    /// fail, as `Dependency Upload & Remap`'s own `TheRecord`** — `Dependency Upload & Remap` V1:
    /// every question answered from the local record alone.
    ///
    /// **The value is that crate's `InMemoryRecord`**, whose own words make it
    /// a test double and never a second store. This is neither of those: it is
    /// read from the one store for the questions in hand and let go of, the
    /// footing `neighbouring_records::TheNeighbours` already gives its
    /// standings. Its shape is taken rather than a copy of it written here.
    ///
    /// **Everything machine-keyed is under this key alone**; located answers
    /// and shift records carry no machine (`Dependency Upload & Remap` K3 and `Dependency Upload & Remap` K6)
    /// and are all held.
    ///
    /// **Published transfer records are never answered, and that is the plan's
    /// own seam rather than an omission.** `Dependency Upload & Remap` Q5's
    /// fast path reads *the four recorded at publication* from a published
    /// transfer record, and `Server protocol and transfers` S9 says that record
    /// *"holds no copy of any byte and no observed value the machine still
    /// holds"* — the published copy's four values are not in it. **What holds
    /// is `Dependency Upload & Remap` Q5's own**: where the fast path cannot fire, the read-through runs,
    /// and `Dependency Upload & Remap` Q9's lookup missing a hand-upload costs one redundant copy, *"never
    /// wrongness"*.
    pub fn what_it_holds_for(&self, machine: &MachineKey) -> Result<InMemoryRecord, RecordFailure> {
        Ok(InMemoryRecord {
            placements: self.the_placements_on(machine)?,
            published_transfers: Vec::new(),
            fixings: self.the_fixings_on(machine)?,
            located_answers: self.every_answer()?,
            shifts: self.every_shift()?,
            jobs: self.the_live_or_waiting_jobs_on(machine)?,
            standings: self.the_standings_on(machine)?,
        })
    }
}

// ════════════════════════════════════════════════════════════════════════════
// `File Browser` N6's answers, as the check and the walk take them
// ════════════════════════════════════════════════════════════════════════════

/// **`File Browser` N6's answers for one machine, read once from the upload
/// records** — *where did this file come from*, for any file on the machine.
/// Step 10c's dependency wave piece 7b.
///
/// **Two readers take it, and they must take it the same way.**
/// `Dependency Upload & Remap` K3 keys a root's answers under `File Browser` N6's source path
/// (else `Dependency Upload & Remap` K5's machine key), and `Dependency Upload & Remap` V13a's walk now takes `File Browser` N6's answer first
/// (`smoozy_dependencies::standing::the_holding_file_the_walk_takes`). **A
/// check keyed one way and a walk keyed another is a *Replace* that marks
/// nothing**, which is what every root uploaded through Files met until this
/// piece. So the band, the composition at submission and every caller that
/// writes a mark read `File Browser` N6 here. *(The check's own start, in
/// `crate::surface::the_dependency_check`, keeps the same rule inline.)*
///
/// **Upload records that will not read answer nothing**, which is
/// `Dependency Upload & Remap` K5's case at the check's start too: nothing is
/// guessed.
pub struct WhatFilesSays {
    requests: Vec<smoozy_file_browser::request::UploadRequest>,
    machine: MachineKey,
}

impl WhatFilesSays {
    pub fn read(connection: &Connection, machine: &MachineKey) -> WhatFilesSays {
        use smoozy_file_browser::request::KeepsTheRequest as _;
        WhatFilesSays {
            requests: super::upload_requests::UploadRequests::over(connection)
                .every_request_for(&machine.server_row)
                .unwrap_or_default(),
            machine: machine.clone(),
        }
    }

    /// **`File Browser` N6's answer for `file`**: the source path the most recent upload
    /// through Files published it from, under the machine's current
    /// installation.
    pub fn where_it_came_from(&self, file: &MachineLocation) -> Option<StoredLocation> {
        use smoozy_file_browser::uploading::{where_it_came_from, WhereItCameFrom};
        match where_it_came_from(
            &self.requests,
            &self.machine.server_row,
            file.as_bytes(),
            Some(&self.machine.installation),
        ) {
            WhereItCameFrom::TheSourcePath(source) => {
                Some(StoredLocation::as_the_system_returned_it(&source))
            }
            _ => None,
        }
    }

    /// **`Dependency Upload & Remap` K3's holding file for a check of `file`**:
    /// `File Browser` N6's source path, else `Dependency Upload & Remap` K5's machine key.
    pub fn the_holding_file_a_check_keys(&self, file: &MachineLocation) -> HoldingFile {
        match self.where_it_came_from(file) {
            Some(source) => HoldingFile::OnTheUsersComputer(source),
            None => HoldingFile::OnTheMachineWithNoKnownLocalOrigin {
                machine: self.machine.clone(),
                location: file.clone(),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::{Path, PathBuf};
    use std::time::Duration;

    use smoozy_dependencies::record::TheRecord;
    use smoozy_dependencies::search::SearchedFolderList;

    use crate::record::LocalRecord;

    fn a_folder(name: &str) -> PathBuf {
        let mut folder = std::env::temp_dir();
        folder.push(format!(
            "smoozy-dependency-records-{name}-{}",
            std::process::id()
        ));
        let _ = std::fs::remove_dir_all(&folder);
        std::fs::create_dir_all(&folder).expect("a folder for the record");
        folder
    }

    fn the_record_at(folder: &Path) -> LocalRecord {
        LocalRecord::open_at(&folder.join("local-record.sqlite3")).expect("the record opens")
    }

    /// **How many saves have reached the record's file**: the engine's own
    /// file change counter, four bytes from offset 24 of the file's header,
    /// which in the rollback journal this record uses moves once for every
    /// transaction that changed the file.
    fn the_saves_so_far(folder: &Path) -> u32 {
        let bytes = std::fs::read(folder.join("local-record.sqlite3")).expect("the file reads");
        u32::from_be_bytes([bytes[24], bytes[25], bytes[26], bytes[27]])
    }

    fn a_server_row() -> Identity {
        Identity::issue(IdentityKind::LocalServer).expect("randomness")
    }

    fn an_installation() -> Identity {
        Identity::issue(IdentityKind::Installation).expect("randomness")
    }

    fn at(bytes: &[u8]) -> MachineLocation {
        MachineLocation::composed(bytes.to_vec())
    }

    fn stored(bytes: &[u8]) -> StoredLocation {
        StoredLocation::as_the_system_returned_it(bytes)
    }

    fn recorded(bytes: &[u8]) -> RecordedLocation {
        RecordedLocation::as_the_reading_carried_it(bytes)
    }

    /// A moment the record holds exactly, to the nanosecond.
    fn a_moment_at(seconds: u64) -> SystemTime {
        SystemTime::UNIX_EPOCH + Duration::new(seconds, 123_456_789)
    }

    fn four_values(size: u64, inode: &str) -> SourceReading {
        SourceReading {
            size,
            inode: inode.to_string(),
            modification_time: 1_700_000_000_123,
            change_time: 1_700_000_000_456,
        }
    }

    fn contents(bytes: &[u8]) -> ContentIdentity {
        ContentIdentity {
            size: bytes.len() as u64,
            digest: Digest::of(bytes),
        }
    }

    fn the_users_file() -> HoldingFile {
        HoldingFile::OnTheUsersComputer(stored(b"/Users/artist/proj/scene.blend"))
    }

    fn an_answer(
        holding: &HoldingFile,
        reference: &[u8],
        resolution: Option<Resolution>,
    ) -> LocatedAnswer {
        LocatedAnswer {
            key: LocatedAnswerKey {
                holding_file: holding.clone(),
                reference: recorded(reference),
            },
            resolution,
            set_aside: false,
            rejected_guess: None,
            read_as: Vec::new(),
        }
    }

    /// **A reading of `count` references, as the check would hand it over**:
    /// every third not found, the rest found at their recorded location.
    fn a_reading_of(holding: &HoldingFile, count: usize) -> Vec<LocatedAnswer> {
        (0..count)
            .map(|n| {
                an_answer(
                    holding,
                    format!("//tex/t{n:02}.png").as_bytes(),
                    (n % 3 != 0).then(|| Resolution {
                        to: ResolvedTo::AWholeLocation(stored(
                            format!("/Users/artist/proj/tex/t{n:02}.png").as_bytes(),
                        )),
                        how: HowResolved::TheRecordedLocationItself,
                    }),
                )
            })
            .collect()
    }

    /// **Every question `Dependency Upload & Remap` K1 answers — its five kinds
    /// and `Dependency Upload & Remap` K9's field — written, the record closed, opened again and read
    /// back field for field**, through the keeper's own readings and through
    /// that module's `TheRecord`.
    ///
    /// **What it bites on**: any kind not written, any field written and not
    /// read back — the four ways an answer was resolved, a folder for a set
    /// row, an arbitrary pick's candidates in their order, the set-aside mark,
    /// a rejected guess, `Dependency Upload & Remap` K5's machine-keyed holding
    /// file, a reference whose bytes are not text, a confirmation and its
    /// moment, a fixing's pairs in order, every figure of a standing and both
    /// marks, the resolved set and the placed copies.
    #[test]
    fn every_question_k1_answers_is_written_and_read_back_after_a_reopening() {
        let folder = a_folder("every-kind");
        let machine = MachineKey::of(a_server_row(), an_installation());
        let users = the_users_file();
        let on_the_machine = HoldingFile::OnTheMachineWithNoKnownLocalOrigin {
            machine: machine.clone(),
            location: at(b"/ephemeral/smoozy/assets/files/x/a.blend"),
        };

        let record = the_record_at(&folder);
        let keeper = TheDependencyRecord::over(record.connection());
        let shift = keeper
            .a_shift_is_taught(
                b"/work".to_vec(),
                b"/Archive".to_vec(),
                a_moment_at(1_700_000_100),
            )
            .expect("the shift is written");
        let answers = vec![
            an_answer(
                &users,
                b"//tex/METallic.png",
                Some(Resolution {
                    to: ResolvedTo::AWholeLocation(stored(b"/Users/artist/proj/tex/METallic.png")),
                    how: HowResolved::TheRecordedLocationItself,
                }),
            ),
            an_answer(&users, b"//tex/metallic.png", None),
            an_answer(
                &users,
                b"//seq/frame_####.png",
                Some(Resolution {
                    to: ResolvedTo::AFolderForASetRow(stored(b"/Volumes/Work/seq")),
                    how: HowResolved::TheSearch {
                        from_folder: b"/Volumes/Work".to_vec(),
                        arbitrary: Some(ArbitraryPick {
                            taken: stored(b"/Volumes/Work/seq"),
                            among: vec![stored(b"/Volumes/Work/seq"), stored(b"/Volumes/Old/seq")],
                        }),
                    },
                }),
            ),
            an_answer(
                &users,
                b"/work/proj/wood.png",
                Some(Resolution {
                    to: ResolvedTo::AWholeLocation(stored(b"/Archive/proj/wood.png")),
                    how: HowResolved::AShift(shift.key),
                }),
            ),
            LocatedAnswer {
                set_aside: true,
                rejected_guess: Some(stored(b"/Users/artist/Rock.png")),
                ..an_answer(
                    &users,
                    b"//rock.png",
                    Some(Resolution {
                        to: ResolvedTo::AWholeLocation(stored(b"/Users/artist/rock.png")),
                        how: HowResolved::TheUsersOwnHand,
                    }),
                )
            },
            an_answer(&users, &[b'/', 0xff, 0xfe, b'.', b'p', b'n', b'g'], None),
            an_answer(
                &on_the_machine,
                b"//hdri.exr",
                Some(Resolution {
                    to: ResolvedTo::AWholeLocation(stored(b"/Users/artist/hdri.exr")),
                    how: HowResolved::TheSearch {
                        from_folder: b"/Users/artist".to_vec(),
                        arbitrary: None,
                    },
                }),
            ),
        ];
        keeper
            .the_answers_are_written(&answers)
            .expect("the answers are written");

        let placement = PlacementRecord::at_publication(
            machine.clone(),
            smoozy_dependencies::record::Published {
                at: at(b"/root/assets/files/Users/artist/proj/tex/METallic.png"),
                source: stored(b"/Users/artist/proj/tex/METallic.png"),
                content: contents(b"red"),
                four_values: four_values(3, "77"),
            },
        )
        .confirmed(
            ConfirmationOutcome::NotConfirmed,
            a_moment_at(1_700_000_200),
        );
        keeper
            .a_placement_is_written(&placement)
            .expect("the placement is written");

        let fixing = FixingRecord {
            machine: machine.clone(),
            derived_from: at(b"/root/assets/files/Users/artist/lib.blend"),
            derived_from_content: contents(b"lib"),
            fixed_copy: at(b"/root/assets/fixed/abc/lib.blend"),
            points_at: vec![
                (recorded(b"//b.png"), at(b"/root/assets/files/b.png")),
                (recorded(b"//a.png"), at(b"/root/assets/files/a.png")),
            ],
        };
        keeper
            .a_fixing_is_written(&fixing)
            .expect("the fixing is written");

        let standing = CheckStanding {
            root: at(b"/root/assets/files/Users/artist/proj/scene.blend"),
            machine: machine.clone(),
            read_with: BlenderIdentity {
                path: "/ephemeral/smoozy/blender/blender-5.2.2/blender".to_string(),
                version: smoozy_server_setup::blender::Version {
                    major: 5,
                    minor: 2,
                    patch: 2,
                },
            },
            at: on_this_computer(nanoseconds(a_moment_at(1_700_000_300)).unwrap()),
            completion: Completion::TheConnectionDropped,
            counts: Counts {
                not_found: 1,
                could_not_be_sent: 2,
                could_not_be_fixed: 3,
                found_but_not_sent: 4,
                set_aside: 5,
                confirmed: 6,
            },
            every_row_confirmed_or_set_aside: false,
            root_four_values: four_values(9_000, "12"),
            replaced_since_check: true,
            disturbed: false,
        };
        keeper
            .a_standing_is_written(&standing)
            .expect("the standing is written");

        let job = Identity::issue(IdentityKind::Job).expect("randomness");
        let set = ResolvedSet {
            locations: vec![
                at(b"/root/assets/files/b.png"),
                at(b"/root/assets/fixed/abc/lib.blend"),
            ],
            remap_material: at(b"/root/assets/remaps/abc"),
        };
        let blend = at(b"/root/assets/files/Users/artist/proj/scene.blend");
        keeper
            .a_jobs_resolved_set_is_written(&job, &machine, &blend, &set)
            .expect("the resolved set is written");
        let jobs = JobRecords::over(record.connection());
        jobs.write_the_provisional_record(&a_job_record(&job, &machine.server_row))
            .expect("the job record is written");
        let copies = PlacedCopyList {
            copies: vec![PlacedCopy {
                job_folder_path: at(b"/root/jobs/abc/bake/cache.vdb"),
                four_values_at_placement: four_values(44, "900"),
            }],
        };
        keeper
            .the_placed_copies_are_written(&job, &copies)
            .expect("the placed copies are written");
        drop(record);

        // **The reopening.**
        let record = the_record_at(&folder);
        let keeper = TheDependencyRecord::over(record.connection());

        let mut users_answers: Vec<LocatedAnswer> = answers
            .iter()
            .filter(|one| one.key.holding_file == users)
            .cloned()
            .collect();
        users_answers.sort_by(|a, b| a.key.reference.as_bytes().cmp(b.key.reference.as_bytes()));
        assert_eq!(
            keeper.the_answers_of(&users).expect("it reads"),
            users_answers
        );
        assert_eq!(
            keeper.the_answers_of(&on_the_machine).expect("it reads"),
            vec![answers[6].clone()]
        );
        assert_eq!(
            keeper.the_shift(shift.key).expect("it reads"),
            Some(shift.clone())
        );
        assert_eq!(
            keeper.the_placements_on(&machine).expect("it reads"),
            vec![placement.clone()]
        );
        assert_eq!(
            keeper.the_fixings_on(&machine).expect("it reads"),
            vec![fixing.clone()]
        );
        assert_eq!(
            keeper
                .the_standing(&standing.root, &machine)
                .expect("it reads"),
            Some(standing.clone())
        );
        assert_eq!(keeper.the_placed_copies_of(&job).expect("it reads"), copies);

        let held = keeper
            .what_it_holds_for(&machine)
            .expect("the record reads");
        assert_eq!(held.placements_on(&machine), vec![placement]);
        assert_eq!(held.fixings_on(&machine), vec![fixing]);
        assert_eq!(held.located_answers_of(&users), users_answers);
        assert_eq!(
            held.located_answer(&answers[4].key),
            Some(answers[4].clone())
        );
        assert_eq!(held.shift(shift.key), Some(shift));
        assert_eq!(
            held.check_standing(&standing.root, &machine),
            Some(standing)
        );
        assert_eq!(
            held.live_or_waiting_jobs_on(&machine),
            vec![LiveJob {
                job,
                machine: machine.clone(),
                blend,
                resolved_set: set,
            }]
        );
        assert!(held.published_transfers_on(&machine).is_empty());

        // **A recycled machine answers nothing**: same server row, a new
        // installation (`Dependency Upload & Remap` K7), and the rows kept.
        let recycled = MachineKey::of(machine.server_row.clone(), an_installation());
        let fresh = keeper
            .what_it_holds_for(&recycled)
            .expect("the record reads");
        assert!(fresh.placements_on(&recycled).is_empty());
        assert!(fresh.fixings_on(&recycled).is_empty());
        assert!(fresh.live_or_waiting_jobs_on(&recycled).is_empty());
        assert_eq!(fresh.located_answers_of(&users).len(), 6, "answers travel");
        assert_eq!(
            keeper.the_placements_on(&machine).expect("it reads").len(),
            1
        );

        drop(record);
        let _ = std::fs::remove_dir_all(folder);
    }

    fn a_job_record(
        job: &Identity,
        server: &Identity,
    ) -> smoozy_queue_history::record::LocalJobRecord {
        use smoozy_queue_history::record::{
            AnswerOrigin, LocalJobRecord, Standing, TheBlenderTheEntryKeeps, TheDeviceAsk,
            TheServerAsItStood, WhatTheNoteCarried,
        };
        LocalJobRecord {
            job: job.clone(),
            group: None,
            server: TheServerAsItStood {
                local_server: server.clone(),
                label: None,
                hue: None,
            },
            submitted_at: None,
            label: None,
            blend_path_on_the_machine: None,
            scene_name: None,
            submitted_with_no_scene_named: true,
            resolution_percentage_typed: None,
            picture_size_from_the_read: None,
            main_output_from_the_read: None,
            blender: TheBlenderTheEntryKeeps {
                pick: smoozy_render_settings::payload::VersionPick {
                    build_path: "/ephemeral/smoozy/blender/blender-5.2.2/blender".to_string(),
                    version: "5.2.2".to_string(),
                },
                origin: AnswerOrigin::ResolvedForThem,
            },
            device_asked_for: TheDeviceAsk::TheFilesOwnUnread,
            the_files_values_beside_the_overrides: None,
            dependency_check: None,
            carries_on_from: None,
            disturbances: Vec::new(),
            attempts_where_the_file_changed: Vec::new(),
            the_frames: smoozy_queue_history::record::TheFramesTheReadFound::default(),
            standing: Standing::Provisional,
            the_render_start_has_been_learned: false,
            what_the_note_carried: WhatTheNoteCarried::default(),
            the_job_output_folder: Default::default(),
            the_driver_was_below_the_bar: None,
        }
    }

    /// **A job whose ending the record has learned is not live**, and one with
    /// no job record at all is not either; a job with no ending learned is.
    #[test]
    fn a_job_is_live_until_the_record_learns_its_ending() {
        use smoozy_queue_history::acting::note::{
            TheEndingOnTheNote, TheNote, TheOutcomeAsWritten,
        };

        let folder = a_folder("live");
        let record = the_record_at(&folder);
        let keeper = TheDependencyRecord::over(record.connection());
        let jobs = JobRecords::over(record.connection());
        let machine = MachineKey::of(a_server_row(), an_installation());
        let set = ResolvedSet {
            locations: vec![at(b"/root/assets/files/b.png")],
            remap_material: at(b"/root/assets/remaps/one"),
        };
        let [running, ended, untracked] =
            [(); 3].map(|()| Identity::issue(IdentityKind::Job).expect("randomness"));
        for job in [&running, &ended, &untracked] {
            keeper
                .a_jobs_resolved_set_is_written(job, &machine, &at(b"/root/a.blend"), &set)
                .expect("written");
        }
        for job in [&running, &ended] {
            jobs.write_the_provisional_record(&a_job_record(job, &machine.server_row))
                .expect("written");
        }
        let mut note = TheNote::created_with_the_job_record("Kitchen");
        note.ending = Some(TheEndingOnTheNote {
            outcome: TheOutcomeAsWritten::GivenUp,
            exit_status: Some(1),
            the_attempt_whose_capture_record_holds_the_words: 1,
            the_reason_the_machine_recorded: None,
            the_device_line_as_observed: None,
        });
        jobs.the_fill_in(&ended, &note)
            .expect("the note is carried over");

        let live: Vec<Identity> = keeper
            .the_live_or_waiting_jobs_on(&machine)
            .expect("it reads")
            .into_iter()
            .map(|job| job.job)
            .collect();
        assert_eq!(live, vec![running]);

        drop(record);
        let _ = std::fs::remove_dir_all(folder);
    }

    /// **A 75-reference reading goes down in one save**, and all 75 are there
    /// after a reopening. The count is the record file's own change counter.
    ///
    /// **What it bites on**: the answers written one save each — 75 flushes of
    /// the disk, about 16 ms each on the Creator's Mac, which is 1.2 seconds a
    /// reading before the next can start.
    #[test]
    fn a_seventy_five_reference_reading_is_written_in_one_save() {
        let folder = a_folder("one-save");
        let holding = the_users_file();
        let reading = a_reading_of(&holding, 75);

        let record = the_record_at(&folder);
        let keeper = TheDependencyRecord::over(record.connection());
        let before = the_saves_so_far(&folder);
        let started = std::time::Instant::now();
        keeper
            .the_answers_are_written(&reading)
            .expect("the reading is written");
        let took = started.elapsed();
        let saves = the_saves_so_far(&folder) - before;
        println!("75 answers: {saves} save(s) in {took:?}");
        assert_eq!(saves, 1, "a reading's findings took {saves} saves");
        drop(record);

        let record = the_record_at(&folder);
        let back = TheDependencyRecord::over(record.connection())
            .the_answers_of(&holding)
            .expect("it reads");
        assert_eq!(back.len(), 75);
        let mut sorted = reading;
        sorted.sort_by(|a, b| a.key.reference.as_bytes().cmp(b.key.reference.as_bytes()));
        assert_eq!(back, sorted);

        drop(record);
        let _ = std::fs::remove_dir_all(folder);
    }

    /// **A save that fails part way leaves nothing of itself, and what stood
    /// before it still stands.** The engine is made to refuse the reading's
    /// last answer.
    ///
    /// **What it bites on**: the answers written outside one save (the first
    /// 74 would stay), and a failed save that did not roll back.
    #[test]
    fn a_reading_whose_save_fails_leaves_nothing_of_itself() {
        let folder = a_folder("fails");
        let holding = the_users_file();
        let record = the_record_at(&folder);
        let keeper = TheDependencyRecord::over(record.connection());
        let earlier = vec![LocatedAnswer {
            set_aside: true,
            ..an_answer(&holding, b"//kept.png", None)
        }];
        keeper.the_answers_are_written(&earlier).expect("written");

        record
            .connection()
            .execute_batch(
                "CREATE TEMP TRIGGER the_last_answer_is_refused
                 BEFORE INSERT ON main.dependency_located_answer
                 WHEN NEW.reference = CAST('//tex/t74.png' AS BLOB)
                 BEGIN SELECT RAISE(ABORT, 'refused'); END",
            )
            .expect("the trigger is made");
        assert!(keeper
            .the_answers_are_written(&a_reading_of(&holding, 75))
            .is_err());
        assert_eq!(keeper.the_answers_of(&holding).expect("it reads"), earlier);
        assert!(
            record.connection().is_autocommit(),
            "the save was let go of"
        );

        drop(record);
        let _ = std::fs::remove_dir_all(folder);
    }

    /// **A job's resolved set is written once** (`Dependency Upload & Remap`
    /// K9): a second write is refused and the first stands whole.
    #[test]
    fn a_jobs_resolved_set_is_written_once() {
        let folder = a_folder("once");
        let record = the_record_at(&folder);
        let keeper = TheDependencyRecord::over(record.connection());
        let jobs = JobRecords::over(record.connection());
        let machine = MachineKey::of(a_server_row(), an_installation());
        let job = Identity::issue(IdentityKind::Job).expect("randomness");
        jobs.write_the_provisional_record(&a_job_record(&job, &machine.server_row))
            .expect("written");
        let first = ResolvedSet {
            locations: vec![at(b"/root/assets/files/a.png")],
            remap_material: at(b"/root/assets/remaps/first"),
        };
        let second = ResolvedSet {
            locations: vec![
                at(b"/root/assets/files/b.png"),
                at(b"/root/assets/files/c.png"),
            ],
            remap_material: at(b"/root/assets/remaps/second"),
        };
        keeper
            .a_jobs_resolved_set_is_written(&job, &machine, &at(b"/root/a.blend"), &first)
            .expect("the first is written");
        assert!(keeper
            .a_jobs_resolved_set_is_written(&job, &machine, &at(b"/root/b.blend"), &second)
            .is_err());
        let live = keeper
            .the_live_or_waiting_jobs_on(&machine)
            .expect("it reads");
        assert_eq!(live.len(), 1);
        assert_eq!(live[0].resolved_set, first);
        assert_eq!(live[0].blend, at(b"/root/a.blend"));

        drop(record);
        let _ = std::fs::remove_dir_all(folder);
    }

    /// **A mark written onto a standing reads back as that standing marked**
    /// (`Dependency Upload & Remap` V13 and `Dependency Upload & Remap` V13a): the standing is written again,
    /// never a second row beside it.
    #[test]
    fn a_marked_standing_is_the_same_standing_marked() {
        let folder = a_folder("marks");
        let record = the_record_at(&folder);
        let keeper = TheDependencyRecord::over(record.connection());
        let machine = MachineKey::of(a_server_row(), an_installation());
        let standing = CheckStanding::at_the_sittings_end(
            at(b"/root/assets/files/scene.blend"),
            machine.clone(),
            BlenderIdentity {
                path: "/b/blender".to_string(),
                version: smoozy_server_setup::blender::Version {
                    major: 4,
                    minor: 5,
                    patch: 12,
                },
            },
            a_moment_at(1_700_000_400),
            Completion::Completed,
            &[],
            four_values(10, "1"),
        );
        keeper.a_standing_is_written(&standing).expect("written");
        let mut held = keeper.the_standings_on(&machine).expect("it reads");
        assert_eq!(held, vec![standing.clone()]);
        held[0] = held[0].clone().disturbed();
        keeper
            .a_standing_is_written(&held[0])
            .expect("the mark is written");
        assert_eq!(
            keeper.the_standings_on(&machine).expect("it reads"),
            vec![standing.disturbed()]
        );

        drop(record);
        let _ = std::fs::remove_dir_all(folder);
    }

    /// **`Dependency Upload & Remap` F1: the searched-folder list keeps the
    /// user's order across a reopening** — an order that is neither
    /// alphabetical nor the order the folders were first added in.
    #[test]
    fn the_search_lists_order_is_kept_across_a_reopening() {
        let folder = a_folder("search-list");
        let list = SearchedFolderList {
            folders: vec![b"/c".to_vec(), b"/a".to_vec(), b"/b".to_vec()],
        };
        {
            let record = the_record_at(&folder);
            the_searched_folder_list::write(
                record.connection(),
                &SearchedFolderList {
                    folders: vec![b"/a".to_vec(), b"/b".to_vec(), b"/c".to_vec()],
                },
            )
            .expect("written");
            the_searched_folder_list::write(record.connection(), &list).expect("moved");
        }
        let record = the_record_at(&folder);
        assert_eq!(
            the_searched_folder_list::read(record.connection()).expect("it reads"),
            list
        );
        drop(record);
        let _ = std::fs::remove_dir_all(folder);
    }

    /// **`Dependency Upload & Remap` K4's last limb, record version 29: what
    /// the check read each reference as is kept across a reopening, in the
    /// reading's order, and replaced whole with its answer** — so the remap's
    /// pairs can be composed from the record in a run that never checked.
    ///
    /// **What it bites on**: the kinds not written, not read back, read back
    /// out of order, or left behind when an answer is written again with
    /// fewer.
    #[test]
    fn what_the_check_read_each_reference_as_is_kept_and_replaced_whole() {
        let folder = a_folder("read-as");
        let holding = the_users_file();
        let resolved = Some(Resolution {
            to: ResolvedTo::AWholeLocation(stored(b"/Users/artist/proj/clip.mov")),
            how: HowResolved::TheRecordedLocationItself,
        });
        let twice = LocatedAnswer {
            read_as: vec![
                ReadAs {
                    kind: Kind::MovieClip,
                    setting: "filepath".to_string(),
                },
                ReadAs {
                    kind: Kind::Image,
                    setting: "filepath".to_string(),
                },
            ],
            ..an_answer(&holding, b"//clip.mov", resolved.clone())
        };
        let node = LocatedAnswer {
            read_as: vec![ReadAs {
                kind: Kind::ImportNodePath,
                setting: "inputs[\"Path\"].default_value".to_string(),
            }],
            ..an_answer(&holding, b"//points.csv", None)
        };
        {
            let record = the_record_at(&folder);
            TheDependencyRecord::over(record.connection())
                .the_answers_are_written(&[twice.clone(), node.clone()])
                .expect("written");
        }
        let record = the_record_at(&folder);
        let keeper = TheDependencyRecord::over(record.connection());
        assert_eq!(
            keeper.the_answers_of(&holding).expect("it reads"),
            vec![twice.clone(), node.clone()]
        );

        // **A later reading carried the clip as a picture only.**
        let once = LocatedAnswer {
            read_as: vec![ReadAs {
                kind: Kind::Image,
                setting: "filepath".to_string(),
            }],
            ..twice
        };
        keeper
            .the_answers_are_written(std::slice::from_ref(&once))
            .expect("written again");
        assert_eq!(
            keeper.the_answers_of(&holding).expect("it reads"),
            vec![once, node]
        );
        drop(record);
        let _ = std::fs::remove_dir_all(folder);
    }

    /// **The record spells a kind as the check's reading spells it**, for
    /// every kind — so a kind written by one reading reads back as the same
    /// kind, and a rename on either side is caught here.
    #[test]
    fn a_kinds_spelling_here_is_the_readings_own() {
        for kind in Kind::ALL {
            let here = a_kinds_text(kind).expect("spelled");
            assert_eq!(
                serde_json::to_value(kind).expect("the reading's spelling"),
                serde_json::Value::String(here.clone())
            );
            assert_eq!(the_kind_spelled(here).expect("read back"), kind);
        }
    }
}

/// `Server protocol and transfers` S0a: the dependency request's parent and its file outcomes, saved together.
/// Previously this parent existed only in `the_found_files_sent`'s run list.
/// File checkpoints never allocate a parent start.
pub(crate) fn save_dependency_send(connection: &Connection, send: &crate::the_found_files_sent::ASend) -> Result<(), RecordFailure> {
    in_one_save(connection, "dependency_send", || {
        let request = send.request.as_text();
        let start = super::transfer_records::start_for_parent(connection, "dependency_upload_request", "request", &request, false)?;
        connection.execute(
            "INSERT INTO dependency_upload_request (request, server, blend, started_at_utc, start_order)
             VALUES (?1, ?2, ?3, ?4, ?5) ON CONFLICT(request) DO NOTHING",
            params![request, send.server, send.file, start.map(|s| s.started_at_utc), start.map(|s| s.start_order)],
        )?;
        for (position, file) in send.files.iter().enumerate() {
            use crate::the_found_files_sent::HowItEnded;
            let ending = file.ended.as_ref().map(|e| match e {
                HowItEnded::Published => "published", HowItEnded::Refused => "refused",
                HowItEnded::Failed => "failed", HowItEnded::DidNotFinish => "did not finish",
            });
            connection.execute(
                "INSERT INTO dependency_upload_file (request, position, landing, size, transfer, ending)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6) ON CONFLICT(request, position) DO UPDATE SET
                 transfer = coalesce(dependency_upload_file.transfer, excluded.transfer),
                 ending = coalesce(dependency_upload_file.ending, excluded.ending)",
                params![request, position as i64, file.landing, a_size(file.size)?, file.transfer.as_ref().map(Identity::as_text), ending],
            )?;
        }
        Ok(())
    })
}

#[cfg(test)]
pub(crate) fn saved_dependency_sends(connection: &Connection) -> Result<Vec<crate::the_found_files_sent::ASend>, RecordFailure> {
    use crate::the_found_files_sent::{ASend, AFileSent, HowItEnded};
    let mut query = connection.prepare_cached("SELECT request, server, blend FROM dependency_upload_request ORDER BY request")?;
    let rows = query.query_map([], |r| Ok((r.get::<_, String>(0)?, r.get::<_, String>(1)?, r.get::<_, String>(2)?)))?;
    let mut sends = Vec::new();
    for row in rows {
        let (request, server, file) = row?;
        let Some(identity) = Identity::read_back(IdentityKind::Operation, &request) else { continue };
        let mut files = connection.prepare_cached("SELECT landing, size, transfer, ending FROM dependency_upload_file WHERE request = ?1 ORDER BY position")?;
        let files = files.query_map([request], |r| {
            let ending: Option<String> = r.get(3)?;
            Ok(AFileSent {
                landing: r.get(0)?, size: r.get::<_, i64>(1)? as u64,
                transfer: r.get::<_, Option<String>>(2)?.and_then(|s| Identity::read_back(IdentityKind::Transfer, &s)),
                ended: match ending.as_deref() {
                    Some("published") => Some(HowItEnded::Published), Some("refused") => Some(HowItEnded::Refused),
                    Some("failed") => Some(HowItEnded::Failed), Some("did not finish") => Some(HowItEnded::DidNotFinish), _ => None,
                },
            })
        })?.collect::<Result<Vec<_>, _>>()?;
        sends.push(ASend { request: identity, server, file, files });
    }
    Ok(sends)
}

/// A file's change writes that file only; a large send must not rewrite its
/// entire enumeration at every publication. Neither parent stamp changes.
pub(crate) fn dependency_file_moved(connection: &Connection, request: &Identity, position: usize,
    file: &crate::the_found_files_sent::AFileSent) -> Result<(), RecordFailure> {
    use crate::the_found_files_sent::HowItEnded;
    let ending = file.ended.as_ref().map(|e| match e {
        HowItEnded::Published => "published", HowItEnded::Refused => "refused",
        HowItEnded::Failed => "failed", HowItEnded::DidNotFinish => "did not finish",
    });
    connection.execute("UPDATE dependency_upload_file SET
        transfer = coalesce(transfer, ?3), ending = coalesce(ending, ?4)
        WHERE request = ?1 AND position = ?2",
        params![request.as_text(), position as i64, file.transfer.as_ref().map(Identity::as_text), ending])?;
    Ok(())
}

pub(crate) struct DependencySendCounted {
    pub request: Identity,
    pub server: String,
    pub blend: String,
    pub files: usize,
    pub published: usize,
    pub bytes: u64,
    pub published_bytes: u64,
    pub all_ended: bool,
    pub failed: bool,
}

/// The screen needs totals, not every saved file on every byte-progress event.
pub(crate) fn dependency_sends_counted(connection: &Connection) -> Result<Vec<DependencySendCounted>, RecordFailure> {
    let mut query = connection.prepare_cached(
        "SELECT r.request, r.server, r.blend, count(f.position),
         coalesce(sum(f.ending = 'published'), 0), coalesce(sum(f.size), 0),
         coalesce(sum(CASE WHEN f.ending = 'published' THEN f.size ELSE 0 END), 0),
         coalesce(sum(f.ending IN ('published', 'refused', 'failed')), 0),
         coalesce(sum(f.ending IN ('refused', 'failed')), 0)
         FROM dependency_upload_request AS r LEFT JOIN dependency_upload_file AS f ON f.request = r.request
         GROUP BY r.request ORDER BY r.request")?;
    let rows = query.query_map([], |r| Ok((r.get::<_, String>(0)?, r.get::<_, String>(1)?, r.get::<_, String>(2)?,
        r.get::<_, i64>(3)?, r.get::<_, i64>(4)?, r.get::<_, i64>(5)?, r.get::<_, i64>(6)?, r.get::<_, i64>(7)?, r.get::<_, i64>(8)?)))?;
    let mut out = Vec::new();
    for row in rows {
        let (request, server, blend, files, published, bytes, published_bytes, ended, failed) = row?;
        let Some(request) = Identity::read_back(IdentityKind::Operation, &request) else { continue };
        out.push(DependencySendCounted { request, server, blend, files: files as usize, published: published as usize, bytes: bytes as u64, published_bytes: published_bytes as u64, all_ended: ended == files, failed: failed > 0 });
    }
    Ok(out)
}
