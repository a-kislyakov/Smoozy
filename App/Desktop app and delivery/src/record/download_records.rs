//! `Download`'s two durable stores in the local record — `Desktop app and
//! delivery` §5, record version 26.
//!
//! **The shape is that document's and the store is this one's**, which is the
//! split `upload_requests`, `outcome_records`, `key_rows`, `server_rows` and
//! `transfer_records` already stand on. Nothing here decides what an
//! arrangement or a request holds.
//!
//! **A2: no module has a durable store of its own**, and `State, identity and
//! recovery` §1.1 says the same from the other side — its inventory already
//! names *transfer arrangements* in this record — so `Download`'s
//! `KeepsTheArrangement` and `KeepsTheRequest` seams are answered here. Until
//! this file neither had an implementation anywhere outside that crate's own
//! tests, which its `arriving_later` said in terms.
//!
//! # Two stores, and why one of them has a §9.4 half and the other does not
//!
//! **`Download` M1 says a manual download request *is* one `State, identity and
//! recovery` §9.4 outcome record** — not a record beside one — so this module
//! writes both halves of that one record: the shared `outcome_record` and
//! `outcome_record_unit` that every module's multi-part action writes, and the
//! three tables of version 26 that hold the fields M1 adds. They are joined by
//! `(record, position)`, which is §9.4's own ordering. **This is `File Browser`
//! U1's shape in the other direction, taken rather than re-invented**, down to
//! the units being written by position rather than by role: `Download` §12
//! gives every unit of a request the same role — *one file's transfer* — so a
//! write by role would reach an arbitrary one of five hundred frames.
//!
//! **`Download` R1's arrangement has no §9.4 half and is not given one.** It is
//! not a multi-part action: it is a standing intention whose entries **re-open**
//! (`Download` R4 names the two events that re-open one), and a §9.4 unit
//! outcome is immutable once written (`State, identity and recovery` §8.6). A
//! shared record would refuse the one write `Download` R4 requires.
//!
//! # Every write is the whole record, at the moment something became true
//!
//! `Download` R1: *"every later change — the desired state, the narrowing, the
//! destination at a switch-on, an entry's account, an entry's four-value
//! reading — goes down through the same call, at the moment it becomes true."*
//! **So [`DownloadArrangements::write_it_down`] takes the whole arrangement and
//! is called after each step that changed it**, never once at the end of a
//! turn. Both halves of each store go down in one transaction, so an
//! interruption inside one call leaves either the whole record or none of it.
//!
//! **An outcome already written is never overwritten** (`State, identity and
//! recovery` F19, at that document's §8.6): the unit update carries
//! `outcome IS NULL`, so a second write of a unit that has ended changes
//! nothing. It is not reported as a refusal here, because these calls write the
//! whole record rather than one unit.
//!
//! # What is never written here
//!
//! **No doneness judgement** (`Download` R5): a judgement is a derivation from
//! readings, recomputed per walk, and what is durable is what Smoozy did and the
//! readings its acts were keyed to. **No walk, no fetchability, no frontier and
//! no whole** is stored — a fresh run re-walks and re-derives them.
//!
//! **No age, no cap and no sweep** (`State, identity and recovery` §11.1, and
//! `Download` R3 and M1): both stores are user-held and end on the user's own
//! act.
//!
//! **L4 holds: no field of either store is ever placed in a filename. L10
//! holds: no secret value is written.**
//!
//! # How a clause marker is written in this file
//!
//! A marker with no document beside it means a clause of this crate's own plan,
//! `Preparation/Architecture/Desktop app and delivery.md`. **Every marker of
//! another document is written out with that document at its own site and this
//! file keeps no header list**, which is the shape `src/downloading.rs` and
//! `src/uploading.rs` settled: `Download` collides with this plan on almost
//! every letter of its own series, and a header list cannot tell the two apart.

use rusqlite::OptionalExtension;

use smoozy_download::arrangement::{
    AReckoningEntry, AnArrangement, DesiredState, KeepsTheArrangement, TheAccountOfSmoozysOwnAct,
    TheNarrowing, TheServerFileIdentity, WhatEndsIt, WhatSmoozyDid,
};
use smoozy_download::fetching::WhichRefusalItWas;
use smoozy_download::request::{
    self, AManualRequest, ARecordedRefusal, Disposition, EnumerationEntry, KeepsTheRequest,
};
use smoozy_download::vouching::AVouchingMark;
use smoozy_file_browser::already_exists::CollisionAnswer;
use smoozy_file_browser::permission::WhatSmoozyMet;
use smoozy_server_protocol::subsystem::{StatusCode, SubsystemFailure};
use smoozy_server_protocol::transfers::SourceReading;
use smoozy_state::identities::{Identity, IdentityKind};
use smoozy_state::observed::{Clock, MachineClock, Reading};
use smoozy_state::outcomes::Refused;

use super::outcome_records::{as_text, from_text, the_store};
use super::{moment, nanoseconds};

// ════════════════════════════════════════════════════════════════════════════
// The arrangement
// ════════════════════════════════════════════════════════════════════════════

/// The keeper for `Download` R1's arrangement, over one open local record.
pub struct DownloadArrangements<'a> {
    connection: &'a rusqlite::Connection,
}

impl<'a> DownloadArrangements<'a> {
    pub fn over(connection: &'a rusqlite::Connection) -> Self {
        DownloadArrangements { connection }
    }

    /// **`Download` R5's dated note of 2026-10-31: the ending walk completed,
    /// written down against the arrangement's start it completed for** —
    /// record version 57. A start never recorded is written as `0`, the same
    /// value [`Self::the_ending_walk_is_written_down`] compares it with. The
    /// twenty-sixth notes, note 4.
    pub fn the_ending_walk_completed(&self, job: &Identity) -> Result<(), Refused> {
        self.connection
            .execute(
                "UPDATE download_arrangement SET ending_walk_completed_for = coalesce(start_order, 0)
                 WHERE job = ?1",
                [job.as_text()],
            )
            .map(|_| ())
            .map_err(the_store)
    }

    /// **Whether the ending walk is written down as completed for this
    /// arrangement's current start** — nothing written, or written for an
    /// earlier start, is *no*. A row that is not there is *no*.
    pub fn the_ending_walk_is_written_down(&self, job: &Identity) -> Result<bool, Refused> {
        self.connection
            .query_row(
                "SELECT ending_walk_completed_for IS NOT NULL
                        AND ending_walk_completed_for = coalesce(start_order, 0)
                 FROM download_arrangement WHERE job = ?1",
                [job.as_text()],
                |row| row.get::<_, bool>(0),
            )
            .optional()
            .map(|held| held.unwrap_or(false))
            .map_err(the_store)
    }

    /// **The predicted paths an arrangement holds *fetched* into one
    /// destination** — the first-frame pair's other half, for `Download` E1
    /// step 2's dated note of 2026-10-31: one folder, so a file the other half
    /// fetched there is taken. The twenty-sixth notes, note 4.
    pub fn the_paths_fetched_into(&self, job: &Identity, destination: &[u8]) -> Result<Vec<Vec<u8>>, Refused> {
        let mut statement = self
            .connection
            .prepare_cached(
                "SELECT predicted_path FROM download_arrangement_entry
                 WHERE job = ?1 AND account_state = 'fetched' AND destination_in_force = ?2",
            )
            .map_err(the_store)?;
        let rows = statement
            .query_map(rusqlite::params![job.as_text(), destination], |row| row.get::<_, Vec<u8>>(0))
            .map_err(the_store)?;
        rows.collect::<Result<_, _>>().map_err(the_store)
    }

    /// **Every arrangement held against a server row** — what a fresh run
    /// reads before it resumes anything (`Download` R12).
    ///
    /// **A row whose identities will not read back is skipped rather than
    /// given invented ones**, which is `State, identity and recovery` §2.1 and
    /// the same refusal `server_rows` and `key_rows` make.
    pub fn every_arrangement_for(
        &self,
        server_row: &Identity,
    ) -> Result<Vec<AnArrangement>, Refused> {
        let mut statement = self
            .connection
            .prepare("SELECT job FROM download_arrangement WHERE server_row = ?1 ORDER BY rowid")
            .map_err(the_store)?;
        let jobs: Vec<String> = statement
            .query_map([server_row.as_text()], |row| row.get::<_, String>(0))
            .map_err(the_store)?
            .collect::<Result<_, _>>()
            .map_err(the_store)?;
        let mut arrangements = Vec::new();
        for job in jobs {
            let Some(job) = Identity::read_back(IdentityKind::Job, &job) else {
                continue;
            };
            if let Some(arrangement) = self.the_arrangement_for(&job)? {
                arrangements.push(arrangement);
            }
        }
        Ok(arrangements)
    }

    /// **`Download` R1's narrowing, read back.** Its two shapes are told apart
    /// by the column and never by whether the format rows happen to be empty:
    /// *everything* is **no filter rather than a frozen list** (`Download` N3),
    /// and a chosen list that is empty is a different statement.
    fn the_narrowing_of(&self, job: &str, said: &str) -> Result<TheNarrowing, Refused> {
        if said == "everything" {
            return Ok(TheNarrowing::Everything);
        }
        let mut statement = self
            .connection
            .prepare(
                "SELECT format FROM download_arrangement_format WHERE job = ?1 ORDER BY position",
            )
            .map_err(the_store)?;
        let formats: Vec<Vec<u8>> = statement
            .query_map([job], |row| row.get::<_, Vec<u8>>(0))
            .map_err(the_store)?
            .collect::<Result<_, _>>()
            .map_err(the_store)?;
        Ok(TheNarrowing::TheseFormats(formats))
    }

    /// **`Download` R4's reckoning, read back in the store's own key order.**
    ///
    /// **An entry whose account state is none of the six is skipped**, which is
    /// the refusal every other row module here makes for a value that will not
    /// read back: an invented state would be a claim about what Smoozy did.
    fn the_reckoning_of(&self, job: &str) -> Result<Vec<AReckoningEntry>, Refused> {
        let mut statement = self
            .connection
            .prepare(
                "SELECT predicted_path, frame, account_state, transfer, destination_in_force,
                        source_size, source_inode, source_modification_time, source_change_time,
                        read_at, read_on_machine, read_under
                 FROM download_arrangement_entry WHERE job = ?1 ORDER BY rowid",
            )
            .map_err(the_store)?;
        let rows = statement
            .query_map([job], |row| {
                Ok(ARowOfTheReckoning {
                    predicted_path: row.get(0)?,
                    frame: row.get(1)?,
                    account_state: row.get(2)?,
                    transfer: row.get(3)?,
                    destination_in_force: row.get(4)?,
                    source_size: row.get(5)?,
                    source_inode: row.get(6)?,
                    source_modification_time: row.get(7)?,
                    source_change_time: row.get(8)?,
                    read_at: row.get(9)?,
                    read_on_machine: row.get(10)?,
                    read_under: row.get(11)?,
                })
            })
            .map_err(the_store)?;

        let mut reckoning = Vec::new();
        for row in rows {
            let row = row.map_err(the_store)?;
            let Some(state) = state_from_text(&row.account_state) else {
                continue;
            };
            let the_server_file_identity = the_four_values_read_back(&row);
            reckoning.push(AReckoningEntry {
                predicted_path: row.predicted_path,
                frame: row.frame,
                account: TheAccountOfSmoozysOwnAct {
                    transfer: row
                        .transfer
                        .and_then(|text| Identity::read_back(IdentityKind::Transfer, &text)),
                    state,
                    destination_in_force: row.destination_in_force,
                },
                the_server_file_identity,
            });
        }
        Ok(reckoning)
    }
}

/// One row of the reckoning, before it is read back into a shape.
struct ARowOfTheReckoning {
    predicted_path: Vec<u8>,
    frame: Option<i64>,
    account_state: String,
    transfer: Option<String>,
    destination_in_force: Vec<u8>,
    source_size: Option<i64>,
    source_inode: Option<String>,
    source_modification_time: Option<i64>,
    source_change_time: Option<i64>,
    read_at: Option<i64>,
    read_on_machine: Option<String>,
    read_under: Option<String>,
}

/// **`Download` R4's second half, read back whole or not at all.**
///
/// `Server protocol and transfers` S13's four values are one reading, so a row
/// missing any of them carries **no** reading rather than a reading with a
/// value invented for the gap — a half-read identity would be compared against
/// the machine and would decide R7's newer-version telling on a value nobody
/// read.
fn the_four_values_read_back(row: &ARowOfTheReckoning) -> Option<TheServerFileIdentity> {
    let size = u64::try_from(row.source_size?).ok()?;
    let inode = row.source_inode.clone()?;
    let modification_time = row.source_modification_time?;
    let change_time = row.source_change_time?;
    let taken_at = moment(row.read_at?);
    // **The clock is read back as it was written** (`State, identity and
    // recovery` D5): a machine's reading names that machine, and a reading on
    // this computer names none. **Readings on different clocks are never
    // subtracted from one another**, which is only true if the clock survives
    // the write.
    let clock = match &row.read_on_machine {
        Some(machine) => Clock::AMachine(MachineClock(machine.clone())),
        None => Clock::ThisComputer,
    };
    Some(TheServerFileIdentity {
        reading: Reading::taken(
            SourceReading {
                size,
                inode,
                modification_time,
                change_time,
            },
            taken_at,
            clock,
        ),
        // **Absent is a state**, `Download` R4's own word: a reading taken
        // where no installation identity was in hand carries none and is
        // disqualified by the same rule that disqualifies a mismatch.
        read_under: row
            .read_under
            .as_deref()
            .and_then(|text| Identity::read_back(IdentityKind::Installation, text)),
    })
}

/// **`Download` R4's six account states, written down.** Three words and no
/// more per value.
fn state_as_text(state: &WhatSmoozyDid) -> &'static str {
    match state {
        WhatSmoozyDid::Owed => "owed",
        WhatSmoozyDid::Fetched => "fetched",
        WhatSmoozyDid::LeftAlone => "left alone",
        WhatSmoozyDid::CannotLand => "cannot land",
        WhatSmoozyDid::SkippedByNarrowing => "skipped by narrowing",
        WhatSmoozyDid::NeverComing => "never coming",
    }
}

fn state_from_text(said: &str) -> Option<WhatSmoozyDid> {
    match said {
        "owed" => Some(WhatSmoozyDid::Owed),
        "fetched" => Some(WhatSmoozyDid::Fetched),
        "left alone" => Some(WhatSmoozyDid::LeftAlone),
        "cannot land" => Some(WhatSmoozyDid::CannotLand),
        "skipped by narrowing" => Some(WhatSmoozyDid::SkippedByNarrowing),
        "never coming" => Some(WhatSmoozyDid::NeverComing),
        _ => None,
    }
}

impl KeepsTheArrangement for DownloadArrangements<'_> {
    /// **`Download` R1's write, whole, in one transaction.**
    ///
    /// **The row is kept rather than replaced.** A replace would delete the row
    /// and insert a new one, moving the arrangement to the end of the store's
    /// own write order at every later write — and that order is what
    /// [`DownloadArrangements::every_arrangement_for`] answers with.
    ///
    /// **The reckoning is written as it stands and the rows that are no longer
    /// in it go.** `Download` R4's key is the predicted path and nothing else,
    /// so the record's rows and the arrangement's entries are one set by
    /// construction; leaving a row behind for an entry the caller dropped would
    /// put a file in the account that Smoozy never acted on.
    ///
    /// **What is written is what moved, and it is still one save.** The case
    /// this was first written for is an arrangement of a few frames, where
    /// writing every entry again at every change cost nothing. **The case it
    /// now also meets is a 5,000-frame job**: `Download` calls this at every
    /// frame's end with the whole arrangement, so every frame wrote every
    /// entry and then looked each stored path up in a list of all of them.
    /// **So the stored reckoning is read once and compared**, only an entry
    /// that reads back differently is written, and the rows the arrangement no
    /// longer carries are found by key. What is durable afterwards is exactly
    /// what it was before: the whole arrangement, as it stands.
    fn write_it_down(&mut self, arrangement: &AnArrangement) -> Result<(), Refused> {
        let job = arrangement.job.as_text();
        let (narrowing, formats) = match &arrangement.narrowing {
            TheNarrowing::Everything => ("everything", Vec::new()),
            TheNarrowing::TheseFormats(formats) => ("these formats", formats.clone()),
        };
        let desired_state = match arrangement.desired_state {
            DesiredState::On => "on",
            DesiredState::Off => "off",
        };

        self.connection
            .execute("BEGIN IMMEDIATE", [])
            .map_err(the_store)?;
        let written = (|| -> Result<(), rusqlite::Error> {
            let was_off = self.connection.query_row(
                "SELECT desired_state = 'off' FROM download_arrangement WHERE job = ?1",
                [&job], |row| row.get::<_, bool>(0),
            ).optional()?.unwrap_or(false);
            let start = super::transfer_records::start_for_parent(
                self.connection, "download_arrangement", "job", &job, was_off && desired_state == "on",
            )?;
            self.connection
                .prepare_cached(
                    "INSERT INTO download_arrangement
                       (job, server_row, destination, narrowing, desired_state, started_at_utc, start_order)
                     VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)
                     ON CONFLICT(job) DO UPDATE SET
                       server_row = excluded.server_row,
                       destination = excluded.destination,
                       narrowing = excluded.narrowing,
                       desired_state = excluded.desired_state,
                       started_at_utc = excluded.started_at_utc,
                       start_order = excluded.start_order",
                )?
                .execute(rusqlite::params![
                    job,
                    arrangement.server_row.as_text(),
                    arrangement.destination,
                    narrowing,
                    desired_state,
                    start.map(|s| s.started_at_utc),
                    start.map(|s| s.start_order),
                ])?;

            // **The formats move only when the user narrows again**, so they
            // are compared whole and, where they moved, written whole.
            let stored_formats: Vec<Vec<u8>> = self
                .connection
                .prepare_cached(
                    "SELECT format FROM download_arrangement_format WHERE job = ?1
                     ORDER BY position",
                )?
                .query_map([&job], |row| row.get::<_, Vec<u8>>(0))?
                .collect::<Result<_, _>>()?;
            if stored_formats != formats {
                self.connection
                    .prepare_cached("DELETE FROM download_arrangement_format WHERE job = ?1")?
                    .execute([&job])?;
                let mut insert = self.connection.prepare_cached(
                    "INSERT INTO download_arrangement_format (job, position, format)
                     VALUES (?1, ?2, ?3)",
                )?;
                for (position, format) in formats.iter().enumerate() {
                    insert.execute(rusqlite::params![job, position as i64, format])?;
                }
            }

            // **Only the entries whose account moved are written.** The stored
            // reckoning is read once, keyed by `Download` R4's own key, and an
            // entry that reads back exactly as the arrangement holds it is left
            // as it stands.
            let mut stored: std::collections::HashMap<Vec<u8>, ARowAsWritten> = self
                .connection
                .prepare_cached(
                    "SELECT predicted_path, frame, account_state, transfer, destination_in_force,
                            source_size, source_inode, source_modification_time,
                            source_change_time, read_at, read_on_machine, read_under
                     FROM download_arrangement_entry WHERE job = ?1",
                )?
                .query_map([&job], |row| {
                    Ok((
                        row.get::<_, Vec<u8>>(0)?,
                        ARowAsWritten {
                            frame: row.get(1)?,
                            account_state: row.get(2)?,
                            transfer: row.get(3)?,
                            destination_in_force: row.get(4)?,
                            source_size: row.get(5)?,
                            source_inode: row.get(6)?,
                            source_modification_time: row.get(7)?,
                            source_change_time: row.get(8)?,
                            read_at: row.get(9)?,
                            read_on_machine: row.get(10)?,
                            read_under: row.get(11)?,
                        },
                    ))
                })?
                .collect::<Result<_, _>>()?;

            let mut upsert = self.connection.prepare_cached(
                "INSERT INTO download_arrangement_entry
                   (job, predicted_path, frame, account_state, transfer,
                    destination_in_force, source_size, source_inode,
                    source_modification_time, source_change_time,
                    read_at, read_on_machine, read_under)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13)
                 ON CONFLICT(job, predicted_path) DO UPDATE SET
                   frame = excluded.frame,
                   account_state = excluded.account_state,
                   transfer = excluded.transfer,
                   destination_in_force = excluded.destination_in_force,
                   source_size = excluded.source_size,
                   source_inode = excluded.source_inode,
                   source_modification_time = excluded.source_modification_time,
                   source_change_time = excluded.source_change_time,
                   read_at = excluded.read_at,
                   read_on_machine = excluded.read_on_machine,
                   read_under = excluded.read_under",
            )?;
            for entry in &arrangement.reckoning {
                let row = ARowAsWritten::of(entry);
                // **Taken out of the stored set as it is met**, so what is
                // left afterwards is exactly the rows the arrangement no
                // longer carries.
                if stored.remove(&entry.predicted_path).as_ref() == Some(&row) {
                    continue;
                }
                upsert.execute(rusqlite::params![
                    job,
                    entry.predicted_path,
                    row.frame,
                    row.account_state,
                    row.transfer,
                    row.destination_in_force,
                    row.source_size,
                    row.source_inode,
                    row.source_modification_time,
                    row.source_change_time,
                    row.read_at,
                    row.read_on_machine,
                    row.read_under,
                ])?;
            }
            drop(upsert);
            // **Rows for entries the arrangement no longer carries go** — what
            // is left of the stored set, compared by key in this same
            // transaction, so nothing can move between the read and the
            // removal. *(Until 2026-09-28 every stored path was looked for in
            // a list of every entry's key, which is the square of the frame
            // count: 25 million comparisons at every frame of a 5,000-frame
            // job.)*
            if !stored.is_empty() {
                let mut remove = self.connection.prepare_cached(
                    "DELETE FROM download_arrangement_entry
                     WHERE job = ?1 AND predicted_path = ?2",
                )?;
                for path in stored.keys() {
                    remove.execute(rusqlite::params![job, path])?;
                }
            }
            Ok(())
        })();
        self.finish(written)
    }

    /// **The arrangement held against a job's entry, where there is one.**
    ///
    /// **A record whose identities will not read back is no arrangement at
    /// all** rather than one with invented ones — `State, identity and
    /// recovery` §2.1.
    fn the_arrangement_for(&self, job: &Identity) -> Result<Option<AnArrangement>, Refused> {
        let text = job.as_text();
        let row: Option<(String, Vec<u8>, String, String)> = self
            .connection
            .query_row(
                "SELECT server_row, destination, narrowing, desired_state
                 FROM download_arrangement WHERE job = ?1",
                [&text],
                |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?)),
            )
            .optional()
            .map_err(the_store)?;
        let Some((server_row, destination, narrowing, desired_state)) = row else {
            return Ok(None);
        };
        let Some(server_row) = Identity::read_back(IdentityKind::LocalServer, &server_row) else {
            return Ok(None);
        };
        // **A desired state that is neither word is no arrangement**, not an
        // arrangement read as off: reading it as off would silently stop work
        // the user asked for, and reading it as on would start work they did
        // not.
        let desired_state = match desired_state.as_str() {
            "on" => DesiredState::On,
            "off" => DesiredState::Off,
            _ => return Ok(None),
        };
        Ok(Some(AnArrangement {
            job: job.clone(),
            server_row,
            destination,
            narrowing: self.the_narrowing_of(&text, &narrowing)?,
            desired_state,
            reckoning: self.the_reckoning_of(&text)?,
        }))
    }

    /// **`State, identity and recovery` §11.1: user-held**, and `Download` R3's
    /// three endings decide whether the record goes or stays.
    ///
    /// **A removed server ends the arrangement permanently and the record stays
    /// readable** — that clause's own sentence — so the desired state is
    /// written *off*, which is what stops any further fetch being arranged, and
    /// nothing is deleted. The record goes with the other two, whole, in one
    /// transaction.
    fn it_ended(&mut self, job: &Identity, how: WhatEndsIt) -> Result<(), Refused> {
        let job = job.as_text();
        self.connection
            .execute("BEGIN IMMEDIATE", [])
            .map_err(the_store)?;
        let ended = (|| -> Result<(), rusqlite::Error> {
            if how.the_record_goes_with_it() {
                for table in THE_ARRANGEMENTS_TABLES {
                    self.connection
                        .execute(&format!("DELETE FROM {table} WHERE job = ?1"), [&job])?;
                }
            } else {
                self.connection.execute(
                    "UPDATE download_arrangement SET desired_state = 'off' WHERE job = ?1",
                    [&job],
                )?;
            }
            Ok(())
        })();
        self.finish(ended)
    }
}

/// **One reckoning entry exactly as the store writes it**, so an entry and a
/// stored row can be compared for equality before anything is written.
#[derive(Debug, Clone, PartialEq, Eq)]
struct ARowAsWritten {
    frame: Option<i64>,
    account_state: String,
    transfer: Option<String>,
    destination_in_force: Vec<u8>,
    source_size: Option<i64>,
    source_inode: Option<String>,
    source_modification_time: Option<i64>,
    source_change_time: Option<i64>,
    read_at: Option<i64>,
    read_on_machine: Option<String>,
    read_under: Option<String>,
}

impl ARowAsWritten {
    fn of(entry: &AReckoningEntry) -> ARowAsWritten {
        let (size, inode, modified, changed, at, machine) = the_four_values_as_written(entry);
        ARowAsWritten {
            frame: entry.frame,
            account_state: state_as_text(&entry.account.state).to_string(),
            transfer: entry
                .account
                .transfer
                .as_ref()
                .map(|identity| identity.as_text()),
            destination_in_force: entry.account.destination_in_force.clone(),
            source_size: size,
            source_inode: inode,
            source_modification_time: modified,
            source_change_time: changed,
            read_at: at,
            read_on_machine: machine,
            read_under: entry
                .the_server_file_identity
                .as_ref()
                .and_then(|held| held.read_under.as_ref())
                .map(|identity| identity.as_text()),
        }
    }
}

/// **One arrangement, counted by the store** — what
/// [`DownloadArrangements::every_arrangement_counted_for`] answers, and what the
/// Transfers screen composes an auto-download's row from.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AnArrangementCounted {
    pub job: Identity,
    pub destination: Vec<u8>,
    pub narrowing: TheNarrowing,
    pub desired_state: DesiredState,
    /// **The reckoning's own length** — every entry whose account reads back.
    pub entries: usize,
    /// Of those, *fetched*.
    pub fetched: usize,
    /// Of those, *left alone*.
    pub left_alone: usize,
    /// Of those, still *owed*.
    pub owed: usize,
    /// **Of those, *skipped by narrowing*** — out of the whole, silently
    /// correct (`Download` N7's third bullet), so the Transfers row's whole
    /// leaves them out. The ending walk enters every entry the narrowing
    /// excludes, so without this a narrowed job's whole read as the asked
    /// range's. Added 2026-10-19 with `Download` N6's last sentence.
    pub skipped_by_narrowing: usize,
    pub cannot_land: usize,
}

impl DownloadArrangements<'_> {
    /// **Every arrangement held against a server row, counted by the store
    /// rather than read back whole** — what the Transfers screen composes an
    /// auto-download's row from. Added 2026-09-28, step 10c's transfer wave
    /// piece 1.
    ///
    /// **The case the screen's reading was first written for** is a job of a
    /// few frames, where [`Self::every_arrangement_for`] reading every entry of
    /// the reckoning back on every ask cost nothing. **The case it now also
    /// meets** is a 5,000-frame job asked about at every frame's end, while the
    /// record's lock is held.
    ///
    /// **The same arrangements are skipped as the whole reading skips** — a job
    /// whose identity will not read back, and a desired state that is neither
    /// word — and **the same entries are counted as it reads back**: those
    /// whose account state is one of `Download` R4's six ([`state_as_text`]).
    /// A test below holds the counts to what the whole reading composes.
    pub fn every_arrangement_counted_for(
        &self,
        server_row: &Identity,
    ) -> Result<Vec<AnArrangementCounted>, Refused> {
        let mut statement = self
            .connection
            .prepare_cached(
                "SELECT a.job, a.destination, a.narrowing, a.desired_state,
                        count(e.predicted_path),
                        coalesce(sum(e.account_state = 'fetched'), 0),
                        coalesce(sum(e.account_state = 'left alone'), 0),
                        coalesce(sum(e.account_state = 'owed'), 0),
                        coalesce(sum(e.account_state = 'skipped by narrowing'), 0),
                        coalesce(sum(e.account_state = 'cannot land'), 0)
                 FROM download_arrangement AS a
                 LEFT JOIN download_arrangement_entry AS e
                   ON e.job = a.job
                  AND e.account_state IN ('owed', 'fetched', 'left alone', 'cannot land',
                                          'skipped by narrowing', 'never coming')
                 WHERE a.server_row = ?1
                 GROUP BY a.job
                 ORDER BY a.rowid",
            )
            .map_err(the_store)?;
        let rows = statement
            .query_map([server_row.as_text()], |row| {
                Ok((
                    row.get::<_, String>(0)?,
                    row.get::<_, Vec<u8>>(1)?,
                    row.get::<_, String>(2)?,
                    row.get::<_, String>(3)?,
                    row.get::<_, i64>(4)?,
                    row.get::<_, i64>(5)?,
                    row.get::<_, i64>(6)?,
                    row.get::<_, i64>(7)?,
                    row.get::<_, i64>(8)?,
                    row.get::<_, i64>(9)?,
                ))
            })
            .map_err(the_store)?;
        let mut flat = Vec::new();
        for row in rows {
            flat.push(row.map_err(the_store)?);
        }
        drop(statement);

        let mut counted = Vec::new();
        for (job, destination, narrowing, desired_state, entries, fetched, left_alone, owed, skipped, cannot_land) in
            flat
        {
            let Some(identity) = Identity::read_back(IdentityKind::Job, &job) else {
                continue;
            };
            let desired_state = match desired_state.as_str() {
                "on" => DesiredState::On,
                "off" => DesiredState::Off,
                _ => continue,
            };
            counted.push(AnArrangementCounted {
                job: identity,
                destination,
                narrowing: self.the_narrowing_of(&job, &narrowing)?,
                desired_state,
                entries: entries as usize,
                fetched: fetched as usize,
                left_alone: left_alone as usize,
                owed: owed as usize,
                skipped_by_narrowing: skipped as usize,
                cannot_land: cannot_land as usize,
            });
        }
        Ok(counted)
    }
}

/// **`Download` R1's arrangement is these three tables, named once**, the
/// row-level ones before the arrangement's own. Its ending above removes them,
/// and so does `JobRecords::remove` in `src/record/job_records.rs`, **because
/// the arrangement rides the job's entry** (`Download` R3) and every removal
/// of the job's record takes it — in that removal's own transaction, which is
/// why the tables are shared rather than this store's ending called there.
pub const THE_ARRANGEMENTS_TABLES: [&str; 3] = [
    "download_arrangement_entry",
    "download_arrangement_format",
    "download_arrangement",
];

/// **`Download` R4's four values as they are written**, or six nulls where the
/// entry carries no reading.
#[allow(clippy::type_complexity)]
fn the_four_values_as_written(
    entry: &AReckoningEntry,
) -> (
    Option<i64>,
    Option<String>,
    Option<i64>,
    Option<i64>,
    Option<i64>,
    Option<String>,
) {
    let Some(held) = &entry.the_server_file_identity else {
        return (None, None, None, None, None, None);
    };
    let read = held.reading.value();
    let machine = match held.reading.clock() {
        Clock::AMachine(MachineClock(machine)) => Some(machine.clone()),
        Clock::ThisComputer => None,
    };
    (
        i64::try_from(read.size).ok(),
        Some(read.inode.clone()),
        Some(read.modification_time),
        Some(read.change_time),
        nanoseconds(held.reading.taken_at()),
        machine,
    )
}

impl DownloadArrangements<'_> {
    fn finish(&self, written: Result<(), rusqlite::Error>) -> Result<(), Refused> {
        match written {
            Ok(()) => {
                self.connection.execute("COMMIT", []).map_err(the_store)?;
                Ok(())
            }
            Err(error) => {
                let _ = self.connection.execute("ROLLBACK", []);
                Err(the_store(error))
            }
        }
    }
}

// ════════════════════════════════════════════════════════════════════════════
// The manual request
// ════════════════════════════════════════════════════════════════════════════

/// The keeper for `Download` M1's manual download request, over one open local
/// record.
pub struct DownloadRequests<'a> {
    connection: &'a rusqlite::Connection,
}

impl<'a> DownloadRequests<'a> {
    pub fn over(connection: &'a rusqlite::Connection) -> Self {
        DownloadRequests { connection }
    }
}

/// **`Download` M1's collision answer, written down** — the one field of the
/// request that cannot be rebuilt from the machine or from the transfer
/// records, which is why its durability is what makes *the answer the user
/// already gave still stands* true by construction.
fn answer_as_text(answer: &CollisionAnswer) -> &'static str {
    match answer {
        CollisionAnswer::Replace => "replace",
        CollisionAnswer::KeepBoth => "keep both",
        CollisionAnswer::SkipExisting => "skip existing",
        CollisionAnswer::Stop => "stop",
    }
}

fn answer_from_text(answer: Option<String>) -> Option<CollisionAnswer> {
    match answer.as_deref() {
        Some("replace") => Some(CollisionAnswer::Replace),
        Some("keep both") => Some(CollisionAnswer::KeepBoth),
        Some("skip existing") => Some(CollisionAnswer::SkipExisting),
        Some("stop") => Some(CollisionAnswer::Stop),
        _ => None,
    }
}

/// **`Download` P4's three name-check refusals, written down.**
fn refusal_as_text(refusal: &WhichRefusalItWas) -> &'static str {
    match refusal {
        WhichRefusalItWas::NotRepresentable => "not representable",
        WhichRefusalItWas::TooLongInThisSystemsOwnUnit => "too long",
        WhichRefusalItWas::SilentlyTransformed => "silently transformed",
    }
}

fn refusal_from_text(refusal: Option<&str>) -> Option<WhichRefusalItWas> {
    match refusal {
        Some("not representable") => Some(WhichRefusalItWas::NotRepresentable),
        Some("too long") => Some(WhichRefusalItWas::TooLongInThisSystemsOwnUnit),
        Some("silently transformed") => Some(WhichRefusalItWas::SilentlyTransformed),
        _ => None,
    }
}

/// **`Download` M1's five dispositions, written down**, with the companion
/// value each of two of them carries.
fn disposition_as_text(
    disposition: &Disposition,
) -> (&'static str, Option<Vec<u8>>, Option<&'static str>) {
    match disposition {
        Disposition::ToFetch => ("to fetch", None, None),
        Disposition::SkipExisting => ("skip existing", None, None),
        // **`Download` M8's composed name**, computed once when the answer is
        // applied so a resume re-derives nothing.
        Disposition::RenamedUnderKeepBoth(landing) => ("keep both", Some(landing.clone()), None),
        Disposition::Replace => ("replace", None, None),
        Disposition::CannotLand(which) => ("cannot land", None, Some(refusal_as_text(which))),
    }
}

/// Read one back. **A row whose disposition text is none of the five, or whose
/// companion value will not read back, is read as no disposition at all**
/// rather than guessed at — and that is the safe direction, because `Download`
/// M6 already makes *no disposition yet* a real state and a unit with no
/// disposition is never fetched.
fn disposition_from_text(
    disposition: Option<String>,
    keep_both_landing_path: Option<Vec<u8>>,
    disposition_refusal: Option<String>,
) -> Option<Disposition> {
    match disposition.as_deref() {
        Some("to fetch") => Some(Disposition::ToFetch),
        Some("skip existing") => Some(Disposition::SkipExisting),
        Some("keep both") => keep_both_landing_path.map(Disposition::RenamedUnderKeepBoth),
        Some("replace") => Some(Disposition::Replace),
        Some("cannot land") => {
            refusal_from_text(disposition_refusal.as_deref()).map(Disposition::CannotLand)
        }
        _ => None,
    }
}

/// **`Download` J9's two vouching marks, written down.**
///
/// **Unmarked is a value and never an absence** — `Download` J9's own words are that
/// *no claim is made*, which is not the same as a mark nobody wrote — so the
/// column is not null and a row that will not read back takes the mark that
/// claims nothing.
fn vouching_as_text(mark: &AVouchingMark) -> &'static str {
    match mark {
        AVouchingMark::CannotBeVouchedFor => "cannot be vouched for",
        AVouchingMark::NoClaimIsMade => "no claim is made",
    }
}

fn vouching_from_text(said: &str) -> AVouchingMark {
    match said {
        "cannot be vouched for" => AVouchingMark::CannotBeVouchedFor,
        _ => AVouchingMark::NoClaimIsMade,
    }
}

/// **`File Browser` P1's three, and the fourth that is not one of them**,
/// written down for `Download` M4's recorded refusals.
///
/// **The fourth carries the protocol's own answer as itself** — a status code
/// is a number the draft defines rather than text nobody specified
/// (`Server protocol and transfers` §1.3) — so the round trip keeps the cause
/// separated rather than flattening four answers into three.
fn what_smoozy_met_as_text(met: &WhatSmoozyMet) -> (&'static str, Option<i64>) {
    match met {
        WhatSmoozyMet::Refused => ("refused", None),
        WhatSmoozyMet::Gone => ("gone", None),
        WhatSmoozyMet::TheMachineDidNotAnswer => ("the machine did not answer", None),
        WhatSmoozyMet::SomethingElse(failure) => ("something else", Some(the_number_of(failure))),
    }
}

/// The five shapes of a subsystem failure as one number, and back.
///
/// **Nothing is guessed from a number the draft does not define**: a status
/// code above the five shapes is carried as itself, which is what
/// [`StatusCode::from_number`] already does.
fn the_number_of(failure: &SubsystemFailure) -> i64 {
    match failure {
        SubsystemFailure::TheChannelIsGone => -1,
        SubsystemFailure::TheAnswerCouldNotBeRead => -2,
        SubsystemFailure::ThatCapabilityWasNotAnnounced => -3,
        SubsystemFailure::TheServerOfferedAnotherRevision(revision) => -1000 - i64::from(*revision),
        SubsystemFailure::TheServerRefused(code) => i64::from(the_status_number(*code)),
    }
}

fn the_failure_of(number: i64) -> SubsystemFailure {
    match number {
        -1 => SubsystemFailure::TheChannelIsGone,
        -2 => SubsystemFailure::TheAnswerCouldNotBeRead,
        -3 => SubsystemFailure::ThatCapabilityWasNotAnnounced,
        below if below <= -1000 => SubsystemFailure::TheServerOfferedAnotherRevision(
            u32::try_from(-1000 - below).unwrap_or(0),
        ),
        code => SubsystemFailure::TheServerRefused(StatusCode::from_number(
            u32::try_from(code).unwrap_or(0),
        )),
    }
}

/// The draft's own numbers, which [`StatusCode::from_number`] reads the other
/// way. **It is written here rather than guessed**, and a value that document
/// does not define is carried as itself.
fn the_status_number(code: StatusCode) -> u32 {
    match code {
        StatusCode::Ok => 0,
        StatusCode::EndOfFile => 1,
        StatusCode::NoSuchFile => 2,
        StatusCode::PermissionDenied => 3,
        StatusCode::Failure => 4,
        StatusCode::BadMessage => 5,
        StatusCode::NoConnection => 6,
        StatusCode::ConnectionLost => 7,
        StatusCode::OperationUnsupported => 8,
        StatusCode::Other(number) => number,
    }
}

fn what_smoozy_met_from_text(said: &str, number: Option<i64>) -> Option<WhatSmoozyMet> {
    match said {
        "refused" => Some(WhatSmoozyMet::Refused),
        "gone" => Some(WhatSmoozyMet::Gone),
        "the machine did not answer" => Some(WhatSmoozyMet::TheMachineDidNotAnswer),
        "something else" => Some(WhatSmoozyMet::SomethingElse(the_failure_of(number?))),
        _ => None,
    }
}

impl KeepsTheRequest for DownloadRequests<'_> {
    /// **`Download` M1's write, and `Server protocol and transfers` S8's
    /// ordering applies to it**: the enumeration is durable before the first
    /// byte moves, and every later change goes down through this same call at
    /// the moment it becomes true.
    ///
    /// **Both halves of the one record go down in one transaction**, which is
    /// `State, identity and recovery` P5 item 1 at that document's §9.4 — so an
    /// interruption cannot lose the fact that a unit was owed.
    fn write_it_down(&mut self, request: &AManualRequest) -> Result<(), Refused> {
        let record = request.record.as_text();
        let role = request::the_role_every_unit_carries();

        self.connection
            .execute("BEGIN IMMEDIATE", [])
            .map_err(the_store)?;
        let written = (|| -> Result<(), rusqlite::Error> {
            self.the_requests_own_row_is_written(request)?;
            for (position, entry) in request.entries.iter().enumerate() {
                self.one_entry_is_written(&record, role, position, entry)?;
            }

            // **`Download` M4's refusals are rows and their emptiness is their
            // absence.** A request that met none carries none, and no flag
            // stands beside them.
            self.connection
                .prepare_cached("DELETE FROM download_request_refusal WHERE record = ?1")?
                .execute([&record])?;
            let mut insert = self.connection.prepare_cached(
                "INSERT INTO download_request_refusal
                   (record, position, at_path, what_smoozy_met, what_the_server_said)
                 VALUES (?1, ?2, ?3, ?4, ?5)",
            )?;
            for (position, refusal) in request.subtrees_that_could_not_be_read.iter().enumerate() {
                let (what, number) = what_smoozy_met_as_text(&refusal.what_smoozy_met);
                insert.execute(rusqlite::params![
                    record,
                    position as i64,
                    refusal.at,
                    what,
                    number
                ])?;
            }
            Ok(())
        })();
        self.finish(written)
    }

    /// **Every request record held for a server row** — what a fresh sitting
    /// reads (`Download` M13).
    ///
    /// **A record whose identities will not read back is skipped**, and so is
    /// one whose enumeration and whose §9.4 units do not line up: half a record
    /// read back would say a unit was never owed, which is the one thing
    /// `State, identity and recovery` §9.4 item 1 exists to prevent.
    ///
    /// **The order is the store's own write order, oldest first**, supplied by
    /// `rowid` and never by the record identity — that identity is sixteen
    /// random bytes and `State, identity and recovery` §2.1 rule 2 says
    /// outright that nothing sorts by one.
    fn every_request_for(&self, server_row: &Identity) -> Result<Vec<AManualRequest>, Refused> {
        let mut statement = self
            .connection
            .prepare(
                "SELECT record, installation, destination, collision_answer, opened_by_run
                 FROM download_request WHERE server_row = ?1 ORDER BY rowid",
            )
            .map_err(the_store)?;
        let rows = statement
            .query_map([server_row.as_text()], |row| {
                Ok((
                    row.get::<_, String>(0)?,
                    row.get::<_, Option<String>>(1)?,
                    row.get::<_, Vec<u8>>(2)?,
                    row.get::<_, Option<String>>(3)?,
                    row.get::<_, String>(4)?,
                ))
            })
            .map_err(the_store)?;
        let mut flat = Vec::new();
        for row in rows {
            flat.push(row.map_err(the_store)?);
        }
        drop(statement);

        let mut requests = Vec::new();
        for (record, installation, destination, collision_answer, opened_by_run) in flat {
            let Some(identity) = Identity::read_back(IdentityKind::Operation, &record) else {
                continue;
            };
            let Some(opened_by_run) = Identity::read_back(IdentityKind::Run, &opened_by_run) else {
                continue;
            };
            let Some(entries) = self.entries_of(&record)? else {
                continue;
            };
            requests.push(AManualRequest {
                record: identity,
                server_row: server_row.clone(),
                installation: installation
                    .and_then(|text| Identity::read_back(IdentityKind::Installation, &text)),
                destination,
                entries,
                collision_answer: answer_from_text(collision_answer),
                subtrees_that_could_not_be_read: self.refusals_of(&record)?,
                opened_by_run,
            });
        }
        Ok(requests)
    }

    /// **`State, identity and recovery` §11.1: user-held.** It ends when the
    /// user removes it and never otherwise — no age, no cap, no sweep.
    ///
    /// **Both halves of the one record go in one transaction**, the §9.4 half
    /// included: a §9.4 record whose `Download` M1 fields had gone, or the other
    /// way about, would be half a record either way.
    fn the_user_removed_it(&mut self, record: &Identity) -> Result<(), Refused> {
        let record = record.as_text();
        self.connection
            .execute("BEGIN IMMEDIATE", [])
            .map_err(the_store)?;
        let removed = (|| -> Result<(), rusqlite::Error> {
            for statement in [
                "DELETE FROM download_request_refusal WHERE record = ?1",
                "DELETE FROM download_request_entry WHERE record = ?1",
                "DELETE FROM download_request WHERE record = ?1",
                "DELETE FROM outcome_record_unit WHERE record = ?1",
                "DELETE FROM outcome_record WHERE identity = ?1",
            ] {
                self.connection.execute(statement, [&record])?;
            }
            Ok(())
        })();
        self.finish(removed)
    }
}

impl DownloadRequests<'_> {
    /// **One unit's entry moved, and only it goes down** — the request's own
    /// row and the one entry at `at`, in one transaction. Added 2026-09-28,
    /// step 10c's transfer wave piece 1, and `src/record/upload_requests.rs`'s
    /// own write of the same name in the other direction.
    ///
    /// **The case [`KeepsTheRequest::write_it_down`] was first written for** is
    /// a request of a few files. **The case it now also meets** is a *Download
    /// all* of a long job: `src/downloading.rs` wrote the request twice per
    /// file — when its transfer was arranged and when it ended — so a
    /// 10,000-file request wrote 10,000 entries twice per file. **Only the
    /// entry that moved has changed at either moment**, so only it is written.
    ///
    /// **`State, identity and recovery` F19 is kept as the whole write keeps
    /// it**, by the condition on the unit's update, and **`Download`'s
    /// `KeepsTheRequest` does not change**: the door in this crate reaches the
    /// store directly for this one write.
    pub fn one_entry_moved(&mut self, request: &AManualRequest, at: usize) -> Result<(), Refused> {
        let Some(entry) = request.entries.get(at) else {
            return Err(Refused::TheStore(format!(
                "the request has no entry at {at}, so nothing was written"
            )));
        };
        let record = request.record.as_text();
        let role = request::the_role_every_unit_carries();
        self.connection
            .execute("BEGIN IMMEDIATE", [])
            .map_err(the_store)?;
        let written = self
            .the_requests_own_row_is_written(request)
            .and_then(|()| self.one_entry_is_written(&record, role, at, entry));
        self.finish(written)
    }

    /// **Every manual request held for a server row, counted by the store
    /// rather than read back** — what the Transfers screen composes a row
    /// from. Added 2026-09-28, step 10c's transfer wave piece 1.
    ///
    /// **`Download` M16's two totals**, over the units a transfer is arranged
    /// for, and the units whose name check refused them, which the row reports
    /// separately.
    ///
    /// **A request nothing has decided yet is counted over its enumeration.**
    /// Changed 2026-09-30, step 10c's eleventh-pass fix piece 1 (`Download` M1,
    /// M6, M16). **The case this was first written for** was a request whose
    /// dispositions had been taken: its whole is the units a transfer is
    /// arranged for, and *every unit has ended* is those units' outcomes.
    /// **The case it now also meets** is a request `Download` M1 has written
    /// down and M6 has not yet answered — its question open, or the request
    /// still held between its record and its dispositions. No entry holds a
    /// disposition, so the arranged whole was nought, *every unit has ended*
    /// was true of no units at all, and the Transfers row read *done*. **They
    /// parted when the record began to be written before the question was
    /// asked** (M1: *"durable before the first byte moves"*), which is also
    /// before the first read of the destination — on the eleventh pass that
    /// read waited on the system's own Downloads permission, and the row read
    /// *done* for the whole wait. **Now such a request's whole is the entries a
    /// transfer could be arranged for** — every entry the name check did not
    /// refuse, which is what `the_request_goes_without_a_question` arranges —
    /// and it has not ended. ~~A request answered *Stop* at M6 holds its
    /// collision answer and is counted as it was.~~ **Changed 2026-10-01,
    /// step 10c's Vast-pass fix piece 1: a request answered *Stop* at M6 is
    /// counted over what it asked for too**, and is marked stopped, because
    /// counted as it was it read *0 files into … · done* on the Vast pass.
    /// **And a request holding no file but a recorded refusal (M4) is
    /// undecided the same way**, where it read *done* while its question was
    /// still open.
    ///
    /// **The same records are skipped as
    /// [`KeepsTheRequest::every_request_for`] skips** — an identity that will
    /// not read back, an enumeration with a gap in §9.4's ordering or a size
    /// below nought, and one whose units do not line up — so the two readings
    /// cannot disagree about which requests there are. A test below holds the
    /// counts to what the whole reading composes.
    pub fn every_request_counted_for(
        &self,
        server_row: &Identity,
    ) -> Result<Vec<AManualRequestCounted>, Refused> {
        let mut statement = self
            .connection
            .prepare_cached(
                "SELECT record, destination, opened_by_run, collision_answer
                 FROM download_request
                 WHERE server_row = ?1 ORDER BY rowid",
            )
            .map_err(the_store)?;
        let requests: Vec<(String, Vec<u8>, String, Option<String>)> = statement
            .query_map([server_row.as_text()], |row| {
                Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?))
            })
            .map_err(the_store)?
            .collect::<Result<_, _>>()
            .map_err(the_store)?;
        drop(statement);

        let mut counted = Vec::new();
        for (record, destination, run, answer) in requests {
            let answered = answer.is_some();
            let stopped = answer.as_deref() == Some(answer_as_text(&CollisionAnswer::Stop));
            let Some(identity) = Identity::read_back(IdentityKind::Operation, &record) else {
                continue;
            };
            let Some(opened_by_run) = Identity::read_back(IdentityKind::Run, &run) else {
                continue;
            };
            // **One pass over this request's own entries**, each met by its
            // key and joined to its unit by the unit's key.
            let numbers: [i64; 13] = self
                .connection
                .prepare_cached(&format!(
                    "SELECT count(*),
                            coalesce(min(e.position), 0),
                            coalesce(max(e.position), -1),
                            coalesce(sum(e.size_as_enumerated < 0), 0),
                            coalesce(sum({FETCHED}), 0),
                            coalesce(sum({FETCHED} AND u.outcome = 'done'), 0),
                            coalesce(sum(CASE WHEN {FETCHED} THEN e.size_as_enumerated END), 0),
                            coalesce(sum(CASE WHEN {FETCHED} AND u.outcome = 'done'
                                              THEN e.size_as_enumerated END), 0),
                            coalesce(sum({FETCHED} AND (u.outcome IS NULL OR u.outcome NOT IN
                                         ('done', 'failed', 'not attempted'))), 0),
                            coalesce(sum(e.refusal IN ('not representable', 'too long',
                                                       'silently transformed')), 0),
                            coalesce(sum(e.disposition IS NULL), 0),
                            coalesce(sum(e.refusal IS NULL), 0),
                            coalesce(sum(CASE WHEN e.refusal IS NULL THEN e.size_as_enumerated END), 0)
                     FROM download_request_entry AS e
                     LEFT JOIN outcome_record_unit AS u
                       ON u.record = e.record AND u.position = e.position
                     WHERE e.record = ?1"
                ))
                .map_err(the_store)?
                .query_row([&record], |row| {
                    let mut numbers = [0i64; 13];
                    for (at, number) in numbers.iter_mut().enumerate() {
                        *number = row.get(at)?;
                    }
                    Ok(numbers)
                })
                .map_err(the_store)?;
            let [entries, lowest, highest, below_nought, whole, done, bytes, bytes_done, open, cannot_land, undecided, enumerated, bytes_enumerated] =
                numbers;
            let units: i64 = self
                .connection
                .prepare_cached("SELECT count(*) FROM outcome_record_unit WHERE record = ?1")
                .map_err(the_store)?
                .query_row([&record], |row| row.get(0))
                .map_err(the_store)?;
            let the_ordering_has_no_gap = entries == 0 || (lowest == 0 && highest == entries - 1);
            if !the_ordering_has_no_gap || below_nought != 0 || units != entries {
                continue;
            }
            // **`Download` M4's recorded refusals**, counted.
            let could_not_be_read: i64 = self
                .connection
                .prepare_cached("SELECT count(*) FROM download_request_refusal WHERE record = ?1")
                .map_err(the_store)?
                .query_row([&record], |row| row.get(0))
                .map_err(the_store)?;
            // **Nothing decided**: no entry holds a disposition and no
            // collision answer is written (`Download` M1, M6) — over a request
            // that holds a file or a refusal.
            let nothing_is_decided =
                (entries > 0 || could_not_be_read > 0) && undecided == entries && !answered;
            // **A request stopped at M6's question is counted over what it
            // asked for**, so its row reads what did not come. Step 10c's
            // Vast-pass fix piece 1.
            let (whole, bytes) = if nothing_is_decided || stopped {
                (enumerated, bytes_enumerated)
            } else {
                (whole, bytes)
            };
            // **The units that failed, by their landing paths**, the way an
            // upload's request counts its own (`failed_landings` in
            // `crate::record::upload_requests`). Step 10c's thirteenth-pass fix
            // piece 3: without them a request whose every unit had an outcome
            // read *done*, whatever the outcomes were.
            let failed_landings: Vec<(Vec<u8>, u64)> = self
                .connection
                .prepare_cached(&format!(
                    "SELECT e.landing_path, e.size_as_enumerated FROM download_request_entry AS e
                     JOIN outcome_record_unit AS u
                       ON u.record = e.record AND u.position = e.position
                     WHERE e.record = ?1 AND u.outcome = 'failed' AND {FETCHED}
                     ORDER BY e.position"
                ))
                .map_err(the_store)?
                .query_map([&record], |row| {
                    Ok((row.get::<_, Vec<u8>>(0)?, row.get::<_, i64>(1)? as u64))
                })
                .map_err(the_store)?
                .collect::<Result<_, _>>()
                .map_err(the_store)?;
            counted.push(AManualRequestCounted {
                record: identity,
                destination,
                files_in_the_whole: whole as usize,
                files_done: done as usize,
                bytes_in_the_whole: bytes as u64,
                bytes_done: bytes_done as u64,
                every_unit_has_ended: !nothing_is_decided && open == 0,
                cannot_land: cannot_land as usize,
                nothing_is_decided,
                opened_by_run,
                failed_landings,
                stopped_at_the_question: stopped,
                could_not_be_read: could_not_be_read as usize,
            });
        }
        Ok(counted)
    }

    /// Save the counting phase with its start. Its empty enumeration is not
    /// a successfully completed download after an interruption.
    pub(crate) fn begin_counting(&self, request: &AManualRequest, name: &str) -> Result<(), Refused> {
        self.connection.execute("BEGIN IMMEDIATE", []).map_err(the_store)?;
        let written = (|| -> rusqlite::Result<()> {
            self.the_requests_own_row_is_written(request)?;
            self.connection.execute("UPDATE download_request SET counting_name = ?2 WHERE record = ?1", rusqlite::params![request.record.as_text(), name])?;
            Ok(())
        })();
        self.finish(written)
    }

    /// **The request's own row, and the §9.4 record it is.**
    fn the_requests_own_row_is_written(&self, request: &AManualRequest) -> rusqlite::Result<()> {
        let record = request.record.as_text();
        let start = super::transfer_records::start_for_parent(self.connection, "download_request", "record", &record, false)?;
        self.connection
            .prepare_cached(
                // **The row is kept rather than replaced**, for the reason
                // `upload_requests` gives on its own side: the store's write
                // order is what a listing answers with, and a replace would move
                // a request to the end of it at every later write.
                "INSERT INTO download_request
                   (record, server_row, installation, destination, collision_answer, opened_by_run, started_at_utc, start_order)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)
                 ON CONFLICT(record) DO UPDATE SET
                   server_row = excluded.server_row,
                   installation = excluded.installation,
                   destination = excluded.destination,
                   collision_answer = excluded.collision_answer,
                   opened_by_run = excluded.opened_by_run,
                   counting_name = NULL",
            )?
            .execute(rusqlite::params![
                record,
                request.server_row.as_text(),
                request
                    .installation
                    .as_ref()
                    .map(|identity| identity.as_text()),
                request.destination,
                request.collision_answer.as_ref().map(answer_as_text),
                request.opened_by_run.as_text(),
                start.map(|s| s.started_at_utc),
                start.map(|s| s.start_order),
            ])?;

        // **The §9.4 half of the same record**, because `Download` M1 says the
        // request's durable half *is* one outcome record.
        self.connection
            .prepare_cached(
                "INSERT OR IGNORE INTO outcome_record (identity, action) VALUES (?1, ?2)",
            )?
            .execute(rusqlite::params![record, request::THE_ACTION])?;
        Ok(())
    }

    /// **One entry of the enumeration and its §9.4 unit**, at its position.
    fn one_entry_is_written(
        &self,
        record: &str,
        role: &str,
        position: usize,
        entry: &EnumerationEntry,
    ) -> rusqlite::Result<()> {
        let position = position as i64;
        self.connection
            .prepare_cached(
                "INSERT OR IGNORE INTO outcome_record_unit
                   (record, position, role, outcome, stopped_by)
                 VALUES (?1, ?2, ?3, NULL, NULL)",
            )?
            .execute(rusqlite::params![record, position, role])?;
        if let Some(outcome) = &entry.outcome {
            let (outcome, stopped_by) = as_text(outcome);
            // **An outcome once written is immutable** (`State, identity and
            // recovery` §8.6), kept by the condition rather than by a rule
            // somebody has to remember.
            self.connection
                .prepare_cached(
                    "UPDATE outcome_record_unit SET outcome = ?1, stopped_by = ?2
                     WHERE record = ?3 AND position = ?4 AND outcome IS NULL",
                )?
                .execute(rusqlite::params![outcome, stopped_by, record, position])?;
        }

        let (disposition, keep_both, disposition_refusal) = match &entry.disposition {
            Some(disposition) => {
                let (text, keep_both, refusal) = disposition_as_text(disposition);
                (Some(text), keep_both, refusal)
            }
            None => (None, None, None),
        };
        self.connection
            .prepare_cached(
                "INSERT OR REPLACE INTO download_request_entry
                   (record, position, source_path, landing_path, size_as_enumerated,
                    refusal, vouching, disposition, keep_both_landing_path,
                    disposition_refusal, transfer)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)",
            )?
            .execute(rusqlite::params![
                record,
                position,
                // **Both paths are bytes** — `Job files and outputs` N15 makes a
                // machine's names bytes and `Download` P11 carries them as the
                // machine's.
                entry.source_path,
                entry.landing_path,
                entry.size_as_enumerated as i64,
                entry.refusal.as_ref().map(refusal_as_text),
                vouching_as_text(&entry.vouching),
                disposition,
                keep_both,
                disposition_refusal,
                entry.transfer.as_ref().map(|identity| identity.as_text()),
            ])?;
        Ok(())
    }

    fn finish(&self, written: Result<(), rusqlite::Error>) -> Result<(), Refused> {
        match written {
            Ok(()) => {
                self.connection.execute("COMMIT", []).map_err(the_store)?;
                Ok(())
            }
            Err(error) => {
                let _ = self.connection.execute("ROLLBACK", []);
                Err(the_store(error))
            }
        }
    }

    /// **The enumeration and its §9.4 units, read as one.**
    ///
    /// **`None` where the two do not line up**, which is the caller's signal to
    /// skip the whole record rather than to read half of one.
    fn entries_of(&self, record: &str) -> Result<Option<Vec<EnumerationEntry>>, Refused> {
        let mut statement = self
            .connection
            .prepare(
                "SELECT e.position, e.source_path, e.landing_path, e.size_as_enumerated,
                        e.refusal, e.vouching, e.disposition, e.keep_both_landing_path,
                        e.disposition_refusal, e.transfer, u.outcome, u.stopped_by
                 FROM download_request_entry e
                 LEFT JOIN outcome_record_unit u
                   ON u.record = e.record AND u.position = e.position
                 WHERE e.record = ?1 ORDER BY e.position",
            )
            .map_err(the_store)?;
        let rows = statement
            .query_map([record], |row| {
                Ok((
                    row.get::<_, i64>(0)?,
                    row.get::<_, Vec<u8>>(1)?,
                    row.get::<_, Vec<u8>>(2)?,
                    row.get::<_, i64>(3)?,
                    row.get::<_, Option<String>>(4)?,
                    row.get::<_, String>(5)?,
                    row.get::<_, Option<String>>(6)?,
                    row.get::<_, Option<Vec<u8>>>(7)?,
                    row.get::<_, Option<String>>(8)?,
                    row.get::<_, Option<String>>(9)?,
                    row.get::<_, Option<String>>(10)?,
                    row.get::<_, Option<String>>(11)?,
                ))
            })
            .map_err(the_store)?;

        let mut entries = Vec::new();
        for (at, row) in rows.enumerate() {
            let row = row.map_err(the_store)?;
            // **§9.4's own ordering is the key**, so a gap in it is a record
            // that cannot be read back rather than one to be renumbered.
            if row.0 != at as i64 {
                return Ok(None);
            }
            let Ok(size) = u64::try_from(row.3) else {
                return Ok(None);
            };
            entries.push(EnumerationEntry {
                source_path: row.1,
                landing_path: row.2,
                size_as_enumerated: size,
                refusal: refusal_from_text(row.4.as_deref()),
                vouching: vouching_from_text(&row.5),
                disposition: disposition_from_text(row.6, row.7, row.8),
                transfer: row
                    .9
                    .and_then(|text| Identity::read_back(IdentityKind::Transfer, &text)),
                outcome: from_text(row.10, row.11),
            });
        }
        drop(statement);

        // **The two halves have to be the same enumeration.** A §9.4 record
        // with more or fewer units than the request has entries is half a
        // record, and reading it would say a unit was never owed.
        let units: i64 = self
            .connection
            .query_row(
                "SELECT count(*) FROM outcome_record_unit WHERE record = ?1",
                [record],
                |row| row.get(0),
            )
            .map_err(the_store)?;
        if units != entries.len() as i64 {
            return Ok(None);
        }
        Ok(Some(entries))
    }

    /// **`Download` M4's recorded refusals, in the order they were written.**
    ///
    /// **A row whose cause will not read back is skipped rather than given
    /// one**, which keeps the promise `File Browser` P1 makes: the three are
    /// told apart, and nothing composes a fourth answer out of a value nobody
    /// wrote.
    fn refusals_of(&self, record: &str) -> Result<Vec<ARecordedRefusal>, Refused> {
        let mut statement = self
            .connection
            .prepare(
                "SELECT at_path, what_smoozy_met, what_the_server_said
                 FROM download_request_refusal WHERE record = ?1 ORDER BY position",
            )
            .map_err(the_store)?;
        let rows = statement
            .query_map([record], |row| {
                Ok((
                    row.get::<_, Vec<u8>>(0)?,
                    row.get::<_, String>(1)?,
                    row.get::<_, Option<i64>>(2)?,
                ))
            })
            .map_err(the_store)?;
        let mut refusals = Vec::new();
        for row in rows {
            let (at, said, number) = row.map_err(the_store)?;
            let Some(what_smoozy_met) = what_smoozy_met_from_text(&said, number) else {
                continue;
            };
            refusals.push(ARecordedRefusal {
                at,
                what_smoozy_met,
            });
        }
        Ok(refusals)
    }
}

/// **Whether an entry's disposition arranges a transfer, as the store reads
/// it** — `Download`'s `a_transfer_is_arranged` (*to fetch*, *replace*, or
/// *keep both* with its landing path), in [`disposition_as_text`]'s own words
/// and with [`disposition_from_text`]'s rule that *keep both* without its
/// landing path is no disposition at all.
const FETCHED: &str = "(e.disposition IN ('to fetch', 'replace')
     OR (e.disposition = 'keep both' AND e.keep_both_landing_path IS NOT NULL))";

/// **One manual request, counted by the store** — what
/// [`DownloadRequests::every_request_counted_for`] answers, and what the
/// Transfers screen composes a row from.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AManualRequestCounted {
    pub record: Identity,
    pub destination: Vec<u8>,
    /// The units a transfer is arranged for (`Download` M16).
    pub files_in_the_whole: usize,
    /// Of those, the ones *done*.
    pub files_done: usize,
    /// Their enumerated sizes, summed.
    pub bytes_in_the_whole: u64,
    /// The done ones' enumerated sizes, summed.
    pub bytes_done: u64,
    /// **Every unit a transfer is arranged for has an outcome.**
    pub every_unit_has_ended: bool,
    /// **Units whose name check refused them** (`Download` P4), reported
    /// separately.
    pub cannot_land: usize,
    /// **No entry holds a disposition and no collision answer is written** —
    /// the request `Download` M1 writes before M6's question is answered.
    /// Its whole is then its enumeration, and it has not ended. Step 10c's
    /// eleventh-pass fix piece 1.
    pub nothing_is_decided: bool,
    /// **`Download` M13's run marker**: the run that opened the request.
    pub opened_by_run: Identity,
    /// **The landing paths of the units that failed, each with its
    /// enumerated size**, in the enumeration's order — `Download` P3's
    /// recorded unit outcome, which `Design/Transfers § A file that failed`
    /// draws on its own row (E2-1). Step 10c's thirteenth-pass fix piece 3.
    pub failed_landings: Vec<(Vec<u8>, u64)>,
    /// **The user answered `Download` M6's question *Stop*.** Nothing was
    /// fetched, and the whole is what the request asked for. Step 10c's
    /// Vast-pass fix piece 1.
    pub stopped_at_the_question: bool,
    /// **`Download` M4's recorded refusals**: what the request met on the
    /// server and could not read. Step 10c's Vast-pass fix piece 1.
    pub could_not_be_read: usize,
}


#[cfg(test)]
mod tests {
    use super::*;
    use smoozy_state::outcomes::UnitOutcome;
    use std::time::{Duration, SystemTime};

    fn scratch(name: &str) -> std::path::PathBuf {
        let mut path = std::env::temp_dir();
        path.push(format!(
            "smoozy-download-records-{name}-{}",
            std::process::id()
        ));
        let _ = std::fs::remove_dir_all(&path);
        std::fs::create_dir_all(&path).unwrap();
        path.join("local-record.sqlite3")
    }

    fn an_identity(kind: IdentityKind) -> Identity {
        Identity::issue(kind).expect("the desk has randomness")
    }

    fn a_reading(machine: Option<&Identity>) -> Reading<SourceReading> {
        Reading::taken(
            SourceReading {
                size: 40_000_000,
                inode: "918273".to_string(),
                modification_time: 1_700_000_000,
                change_time: 1_700_000_001,
            },
            SystemTime::UNIX_EPOCH + Duration::from_nanos(1_700_000_000_000_000_000),
            match machine {
                Some(row) => Clock::AMachine(MachineClock(row.as_text())),
                None => Clock::ThisComputer,
            },
        )
    }

    // ────────────────────────────────────────────────────────────────────
    // The arrangement
    // ────────────────────────────────────────────────────────────────────

    /// **`Download` R11's second survival, proved rather than argued: the
    /// arrangement survives the app closing and reopening.**
    ///
    /// The record is opened, the arrangement written, the record **closed
    /// whole**, and a second record opened at the same path. `State, identity
    /// and recovery` D6 makes a crash and a clean close leave the same record,
    /// so this is the shape both take.
    #[test]
    fn an_arrangement_survives_the_app_closing_and_reopening() {
        let path = scratch("survives-a-quit");
        let job = an_identity(IdentityKind::Job);
        let server_row = an_identity(IdentityKind::LocalServer);
        let installation = an_identity(IdentityKind::Installation);
        let transfer = an_identity(IdentityKind::Transfer);

        let written = {
            let record = crate::record::LocalRecord::open_at(&path).expect("a record");
            let mut keeper = DownloadArrangements::over(record.connection());
            let mut arrangement = AnArrangement::switched_on(
                job.clone(),
                server_row.clone(),
                b"/Users/artist/Renders".to_vec(),
                TheNarrowing::TheseFormats(vec![b"PNG".to_vec(), b"OPEN_EXR".to_vec()]),
            );
            arrangement.reckoning.push(AReckoningEntry {
                predicted_path: b"frame_0001.png".to_vec(),
                frame: Some(1),
                account: TheAccountOfSmoozysOwnAct {
                    transfer: Some(transfer.clone()),
                    state: WhatSmoozyDid::Fetched,
                    destination_in_force: b"/Users/artist/Renders".to_vec(),
                },
                the_server_file_identity: Some(TheServerFileIdentity {
                    reading: a_reading(Some(&server_row)),
                    read_under: Some(installation.clone()),
                }),
            });
            arrangement.reckoning.push(AReckoningEntry::acted_on(
                b"frame_0002.png".to_vec(),
                Some(2),
                WhatSmoozyDid::NeverComing,
                b"/Users/artist/Renders".to_vec(),
            ));
            keeper.write_it_down(&arrangement).expect("it writes");
            arrangement
        };

        let record = crate::record::LocalRecord::open_at(&path).expect("it reopens");
        let keeper = DownloadArrangements::over(record.connection());
        let read = keeper
            .the_arrangement_for(&job)
            .expect("it reads")
            .expect("the arrangement is there after a quit");
        assert_eq!(read, written, "the arrangement did not survive a quit");

        // **And it is found from the server row too**, which is what a fresh
        // run reads before it resumes anything (`Download` R12).
        let every = keeper.every_arrangement_for(&server_row).expect("it reads");
        assert_eq!(every, vec![written]);

        drop(record);
        let _ = std::fs::remove_dir_all(path.parent().unwrap());
    }

    fn in_memory() -> rusqlite::Connection {
        let connection = rusqlite::Connection::open_in_memory().expect("a record");
        crate::record::migrations::bring_up_to_date(
            &connection,
            std::path::Path::new("in-memory"),
            false,
        )
        .expect("it migrates");
        connection
    }

    /// **`Download` R4: the key is the predicted path and the attempt is not in
    /// it**, kept by the store's own shape.
    ///
    /// A second attempt re-rendering one name meets the same row: there is no
    /// second key for it to land under, and the entry moves rather than
    /// doubling.
    #[test]
    fn a_second_write_moves_the_entry_rather_than_adding_a_second_one() {
        let connection = in_memory();
        let mut keeper = DownloadArrangements::over(&connection);
        let job = an_identity(IdentityKind::Job);
        let mut arrangement = AnArrangement::switched_on(
            job.clone(),
            an_identity(IdentityKind::LocalServer),
            b"/there".to_vec(),
            TheNarrowing::Everything,
        );
        arrangement.reckoning.push(AReckoningEntry::acted_on(
            b"frame_0001.png".to_vec(),
            Some(1),
            WhatSmoozyDid::NeverComing,
            b"/there".to_vec(),
        ));
        keeper.write_it_down(&arrangement).expect("it writes");

        arrangement.reckoning[0].account.state = WhatSmoozyDid::Owed;
        keeper.write_it_down(&arrangement).expect("it writes again");

        let read = keeper.the_arrangement_for(&job).unwrap().unwrap();
        assert_eq!(read.reckoning.len(), 1);
        assert_eq!(read.reckoning[0].account.state, WhatSmoozyDid::Owed);
    }

    /// **`Download` N3: *everything* is no filter rather than a frozen list**,
    /// and the two are told apart by the column rather than by whether the
    /// format rows happen to be empty.
    #[test]
    fn everything_and_an_empty_chosen_list_are_two_different_narrowings() {
        let connection = in_memory();
        let mut keeper = DownloadArrangements::over(&connection);
        for (job, narrowing) in [
            (an_identity(IdentityKind::Job), TheNarrowing::Everything),
            (
                an_identity(IdentityKind::Job),
                TheNarrowing::TheseFormats(Vec::new()),
            ),
        ] {
            let arrangement = AnArrangement::switched_on(
                job.clone(),
                an_identity(IdentityKind::LocalServer),
                b"/there".to_vec(),
                narrowing.clone(),
            );
            keeper.write_it_down(&arrangement).expect("it writes");
            let read = keeper.the_arrangement_for(&job).unwrap().unwrap();
            assert_eq!(read.narrowing, narrowing);
        }
    }

    /// **`Download` R4's second half survives whole or not at all**, with its
    /// moment, its clock and the installation it was read under — which is what
    /// `State, identity and recovery` §2.3 disqualifies it by.
    #[test]
    fn the_four_values_come_back_with_their_clock_and_their_installation() {
        let connection = in_memory();
        let mut keeper = DownloadArrangements::over(&connection);
        let job = an_identity(IdentityKind::Job);
        let server_row = an_identity(IdentityKind::LocalServer);
        let installation = an_identity(IdentityKind::Installation);
        let mut arrangement = AnArrangement::switched_on(
            job.clone(),
            server_row.clone(),
            b"/there".to_vec(),
            TheNarrowing::Everything,
        );
        let mut entry = AReckoningEntry::acted_on(
            b"a.png".to_vec(),
            Some(1),
            WhatSmoozyDid::Fetched,
            b"/there".to_vec(),
        );
        entry.the_server_file_identity = Some(TheServerFileIdentity {
            reading: a_reading(Some(&server_row)),
            read_under: Some(installation.clone()),
        });
        arrangement.reckoning.push(entry);
        keeper.write_it_down(&arrangement).expect("it writes");

        let read = keeper.the_arrangement_for(&job).unwrap().unwrap();
        let held = read.reckoning[0]
            .the_server_file_identity
            .as_ref()
            .expect("the reading came back");
        assert_eq!(held.reading.value().size, 40_000_000);
        assert_eq!(held.reading.value().inode, "918273");
        assert_eq!(
            *held.reading.clock(),
            Clock::AMachine(MachineClock(server_row.as_text())),
            "a machine's reading came back on this computer's clock"
        );
        assert_eq!(held.read_under.as_ref(), Some(&installation));
    }

    /// **A row missing part of the reading carries no reading at all.**
    ///
    /// `Server protocol and transfers` S13's four values are one reading, and
    /// half of one would decide `Download` R7's newer-version telling on a
    /// value nobody read.
    #[test]
    fn half_a_four_value_reading_comes_back_as_no_reading() {
        let connection = in_memory();
        let mut keeper = DownloadArrangements::over(&connection);
        let job = an_identity(IdentityKind::Job);
        let mut arrangement = AnArrangement::switched_on(
            job.clone(),
            an_identity(IdentityKind::LocalServer),
            b"/there".to_vec(),
            TheNarrowing::Everything,
        );
        let mut entry = AReckoningEntry::acted_on(
            b"a.png".to_vec(),
            Some(1),
            WhatSmoozyDid::Fetched,
            b"/there".to_vec(),
        );
        entry.the_server_file_identity = Some(TheServerFileIdentity {
            reading: a_reading(None),
            read_under: None,
        });
        arrangement.reckoning.push(entry);
        keeper.write_it_down(&arrangement).expect("it writes");
        connection
            .execute(
                "UPDATE download_arrangement_entry SET source_inode = NULL",
                [],
            )
            .unwrap();

        let read = keeper.the_arrangement_for(&job).unwrap().unwrap();
        assert!(read.reckoning[0].the_server_file_identity.is_none());
        // **The account half stands**, which is the whole reason `Download` R4
        // splits the entry in two.
        assert_eq!(read.reckoning[0].account.state, WhatSmoozyDid::Fetched);
    }

    /// **`Download` R3's three endings, each doing what that clause says.**
    ///
    /// A removed entry and a cancelled job take the record; **a removed server
    /// ends the arrangement permanently and the record stays readable** as part
    /// of the entry's account, switched off so no further fetch is arranged.
    #[test]
    fn a_removed_server_leaves_the_record_readable_and_arranges_nothing_more() {
        let connection = in_memory();
        let mut keeper = DownloadArrangements::over(&connection);
        for (how, still_there) in [
            (WhatEndsIt::TheEntryIsRemoved, false),
            (WhatEndsIt::TheJobWasCancelled, false),
            (WhatEndsIt::TheServerWasRemoved, true),
        ] {
            let job = an_identity(IdentityKind::Job);
            let mut arrangement = AnArrangement::switched_on(
                job.clone(),
                an_identity(IdentityKind::LocalServer),
                b"/there".to_vec(),
                TheNarrowing::TheseFormats(vec![b"PNG".to_vec()]),
            );
            arrangement.reckoning.push(AReckoningEntry::acted_on(
                b"a.png".to_vec(),
                Some(1),
                WhatSmoozyDid::Fetched,
                b"/there".to_vec(),
            ));
            keeper.write_it_down(&arrangement).expect("it writes");
            keeper.it_ended(&job, how).expect("it ends");

            let read = keeper.the_arrangement_for(&job).unwrap();
            assert_eq!(read.is_some(), still_there, "{how:?} left the wrong record");
            if let Some(read) = read {
                assert_eq!(read.desired_state, DesiredState::Off);
                assert_eq!(read.reckoning.len(), 1, "the account is still readable");
            }
        }
        // **Nothing of a removed arrangement is left behind in the companion
        // tables**, which would otherwise read back as an account of a job that
        // has no record.
        for table in ["download_arrangement_entry", "download_arrangement_format"] {
            let count: i64 = connection
                .query_row(&format!("SELECT count(*) FROM {table}"), [], |row| {
                    row.get(0)
                })
                .unwrap();
            assert_eq!(count, 1, "{table} kept rows a removal should have taken");
        }
    }

    // ────────────────────────────────────────────────────────────────────
    // The manual request
    // ────────────────────────────────────────────────────────────────────

    fn a_request(server_row: &Identity) -> AManualRequest {
        let mut request = AManualRequest::confirmed(
            an_identity(IdentityKind::Operation),
            server_row.clone(),
            Some(an_identity(IdentityKind::Installation)),
            b"/Users/artist/Renders".to_vec(),
            vec![
                EnumerationEntry::enumerated(
                    b"/smoozy/jobs/a/outputs/frame_0001.png".to_vec(),
                    b"/Users/artist/Renders/a/frame_0001.png".to_vec(),
                    1_024,
                    AVouchingMark::NoClaimIsMade,
                ),
                EnumerationEntry::enumerated(
                    b"/smoozy/jobs/a/outputs/frame_0002.png".to_vec(),
                    b"/Users/artist/Renders/a/frame_0002.png".to_vec(),
                    2_048,
                    AVouchingMark::CannotBeVouchedFor,
                ),
            ],
            an_identity(IdentityKind::Run),
        );
        request.collision_answer = Some(CollisionAnswer::KeepBoth);
        request.entries[0].disposition = Some(Disposition::RenamedUnderKeepBoth(
            b"/Users/artist/Renders/a/frame_0001 2.png".to_vec(),
        ));
        request.entries[1].disposition =
            Some(Disposition::CannotLand(WhichRefusalItWas::NotRepresentable));
        request.entries[1].refusal = Some(WhichRefusalItWas::NotRepresentable);
        request
            .subtrees_that_could_not_be_read
            .push(ARecordedRefusal {
                at: b"/smoozy/jobs/a/outputs/private".to_vec(),
                what_smoozy_met: WhatSmoozyMet::Refused,
            });
        request
    }

    /// **`Download` M1's whole record round-trips**, the collision answer and
    /// every entry's disposition, vouching mark and refusal with it.
    #[test]
    fn a_manual_request_round_trips_whole() {
        let connection = in_memory();
        let server_row = an_identity(IdentityKind::LocalServer);
        let mut keeper = DownloadRequests::over(&connection);
        let request = a_request(&server_row);
        keeper.write_it_down(&request).expect("it writes");

        let read = keeper.every_request_for(&server_row).expect("it reads");
        assert_eq!(read, vec![request.clone()]);

        // **The §9.4 half is the same record and not a second one beside it.**
        let action: String = connection
            .query_row(
                "SELECT action FROM outcome_record WHERE identity = ?1",
                [request.record.as_text()],
                |row| row.get(0),
            )
            .unwrap();
        assert_eq!(action, request::THE_ACTION);
        let roles: Vec<String> = connection
            .prepare("SELECT role FROM outcome_record_unit WHERE record = ?1 ORDER BY position")
            .unwrap()
            .query_map([request.record.as_text()], |row| row.get(0))
            .unwrap()
            .collect::<Result<_, _>>()
            .unwrap();
        assert_eq!(
            roles,
            vec![
                request::the_role_every_unit_carries().to_string(),
                request::the_role_every_unit_carries().to_string(),
            ],
            "§12 gives every unit of one request the same role"
        );
    }

    /// **`State, identity and recovery` §8.6: a unit outcome once written is
    /// immutable**, kept by the update's own condition rather than by a rule
    /// somebody has to remember.
    #[test]
    fn a_unit_outcome_already_written_is_never_overwritten() {
        let connection = in_memory();
        let server_row = an_identity(IdentityKind::LocalServer);
        let mut keeper = DownloadRequests::over(&connection);
        let mut request = a_request(&server_row);
        keeper.write_it_down(&request).expect("it writes");

        request.entries[0].outcome = Some(UnitOutcome::Done);
        keeper.write_it_down(&request).expect("it writes");
        request.entries[0].outcome = Some(UnitOutcome::Failed);
        keeper.write_it_down(&request).expect("it writes again");

        let read = keeper.every_request_for(&server_row).unwrap();
        assert_eq!(
            read[0].entries[0].outcome,
            Some(UnitOutcome::Done),
            "the second write moved an outcome that had already been written"
        );
    }

    /// **`Download` M4's fourth cause is not flattened into the three.**
    ///
    /// `File Browser` P1 separates *may not read it*, *gone* and *the machine
    /// did not answer*; a server that answered something else is carried as
    /// itself, protocol status and all, so the separation survives the write.
    #[test]
    fn a_refusal_that_is_none_of_the_three_comes_back_as_the_servers_own_answer() {
        let connection = in_memory();
        let server_row = an_identity(IdentityKind::LocalServer);
        let mut keeper = DownloadRequests::over(&connection);
        let mut request = a_request(&server_row);
        for met in [
            WhatSmoozyMet::Refused,
            WhatSmoozyMet::Gone,
            WhatSmoozyMet::TheMachineDidNotAnswer,
            WhatSmoozyMet::SomethingElse(SubsystemFailure::TheChannelIsGone),
            WhatSmoozyMet::SomethingElse(SubsystemFailure::TheAnswerCouldNotBeRead),
            WhatSmoozyMet::SomethingElse(SubsystemFailure::ThatCapabilityWasNotAnnounced),
            WhatSmoozyMet::SomethingElse(SubsystemFailure::TheServerOfferedAnotherRevision(6)),
            WhatSmoozyMet::SomethingElse(SubsystemFailure::TheServerRefused(
                StatusCode::OperationUnsupported,
            )),
            WhatSmoozyMet::SomethingElse(SubsystemFailure::TheServerRefused(StatusCode::Other(42))),
        ] {
            request.subtrees_that_could_not_be_read = vec![ARecordedRefusal {
                at: b"/a/branch".to_vec(),
                what_smoozy_met: met.clone(),
            }];
            keeper.write_it_down(&request).expect("it writes");
            let read = keeper.every_request_for(&server_row).unwrap();
            assert_eq!(
                read[0].subtrees_that_could_not_be_read, request.subtrees_that_could_not_be_read,
                "{met:?} did not survive the write"
            );
        }
    }

    /// **`State, identity and recovery` §11.1: the request is user-held**, and
    /// both halves of the one record go together.
    #[test]
    fn a_request_ends_when_the_user_removes_it_and_takes_both_halves_with_it() {
        let connection = in_memory();
        let server_row = an_identity(IdentityKind::LocalServer);
        let mut keeper = DownloadRequests::over(&connection);
        let request = a_request(&server_row);
        keeper.write_it_down(&request).expect("it writes");
        keeper
            .the_user_removed_it(&request.record)
            .expect("it removes");

        assert!(keeper.every_request_for(&server_row).unwrap().is_empty());
        for table in [
            "download_request",
            "download_request_entry",
            "download_request_refusal",
            "outcome_record",
            "outcome_record_unit",
        ] {
            let count: i64 = connection
                .query_row(&format!("SELECT count(*) FROM {table}"), [], |row| {
                    row.get(0)
                })
                .unwrap();
            assert_eq!(count, 0, "{table} kept half a record");
        }
    }

    /// **A record whose two halves do not line up is skipped rather than read
    /// as half of one** — `State, identity and recovery` §9.4 item 1, which
    /// exists so nothing can say a unit was never owed.
    #[test]
    fn a_request_whose_units_do_not_line_up_is_not_read_back() {
        let connection = in_memory();
        let server_row = an_identity(IdentityKind::LocalServer);
        let mut keeper = DownloadRequests::over(&connection);
        let request = a_request(&server_row);
        keeper.write_it_down(&request).expect("it writes");
        connection
            .execute(
                "DELETE FROM outcome_record_unit WHERE record = ?1 AND position = 1",
                [request.record.as_text()],
            )
            .unwrap();
        assert!(
            keeper.every_request_for(&server_row).unwrap().is_empty(),
            "half a record was read back"
        );
    }

    // ────────────────────────────────────────────────────────────────────
    // What a save costs, added 2026-09-28 (step 10c's transfer wave piece 1)
    // ────────────────────────────────────────────────────────────────────

    fn a_thousand_frame_arrangement() -> AnArrangement {
        let mut arrangement = AnArrangement::switched_on(
            an_identity(IdentityKind::Job),
            an_identity(IdentityKind::LocalServer),
            b"/there".to_vec(),
            TheNarrowing::Everything,
        );
        for frame in 1..=1_000 {
            arrangement.reckoning.push(AReckoningEntry::acted_on(
                format!("frame_{frame:04}.png").into_bytes(),
                Some(frame),
                WhatSmoozyDid::Owed,
                b"/there".to_vec(),
            ));
        }
        arrangement
    }

    /// **A thousand-frame arrangement whose second save writes the one entry
    /// that moved** — `Download` calls the write with the whole arrangement at
    /// every frame's end. **The case this bites on is the write it replaced**,
    /// which wrote all thousand entries at every frame.
    #[test]
    fn a_thousand_frame_arrangement_s_second_save_writes_one_entry() {
        let connection = in_memory();
        let mut keeper = DownloadArrangements::over(&connection);
        let mut arrangement = a_thousand_frame_arrangement();
        keeper.write_it_down(&arrangement).expect("the first write");

        arrangement.reckoning[417].account.state = WhatSmoozyDid::Fetched;
        arrangement.reckoning[417].account.transfer = Some(an_identity(IdentityKind::Transfer));
        arrangement.reckoning[417].the_server_file_identity = Some(TheServerFileIdentity {
            reading: a_reading(Some(&arrangement.server_row)),
            read_under: None,
        });
        let before = connection.total_changes();
        keeper
            .write_it_down(&arrangement)
            .expect("the second write");
        let wrote = connection.total_changes() - before;
        assert!(wrote <= 2, "a save of one moved entry changed {wrote} rows");
        assert_eq!(
            keeper
                .the_arrangement_for(&arrangement.job)
                .unwrap()
                .unwrap(),
            arrangement
        );

        // **An entry the arrangement no longer carries still goes**, and one
        // it gains is still written.
        arrangement.reckoning.remove(3);
        arrangement.reckoning.push(AReckoningEntry::acted_on(
            b"frame_1001.png".to_vec(),
            Some(1_001),
            WhatSmoozyDid::NeverComing,
            b"/there".to_vec(),
        ));
        keeper.write_it_down(&arrangement).expect("the third write");
        let mut read = keeper
            .the_arrangement_for(&arrangement.job)
            .unwrap()
            .unwrap();
        let mut held = arrangement.clone();
        read.reckoning
            .sort_by(|one, two| one.predicted_path.cmp(&two.predicted_path));
        held.reckoning
            .sort_by(|one, two| one.predicted_path.cmp(&two.predicted_path));
        assert_eq!(read, held);
    }

    /// **The arrangement's counts are what the whole reading composes.**
    #[test]
    fn an_arrangement_s_counts_are_what_the_whole_reading_composes() {
        let connection = in_memory();
        let mut keeper = DownloadArrangements::over(&connection);
        let mut arrangement = a_thousand_frame_arrangement();
        for (at, state) in [
            (1, WhatSmoozyDid::Fetched),
            (2, WhatSmoozyDid::Fetched),
            (3, WhatSmoozyDid::LeftAlone),
            (4, WhatSmoozyDid::NeverComing),
            (5, WhatSmoozyDid::CannotLand),
            (6, WhatSmoozyDid::SkippedByNarrowing),
        ] {
            arrangement.reckoning[at].account.state = state;
        }
        arrangement.narrowing = TheNarrowing::TheseFormats(vec![b"PNG".to_vec()]);
        keeper.write_it_down(&arrangement).unwrap();
        // **An entry whose state will not read back** is in neither reading.
        connection
            .execute(
                "UPDATE download_arrangement_entry SET account_state = 'misread'
                 WHERE predicted_path = ?1",
                [b"frame_0010.png".to_vec()],
            )
            .unwrap();

        let whole = keeper
            .every_arrangement_for(&arrangement.server_row)
            .unwrap();
        let counted = keeper
            .every_arrangement_counted_for(&arrangement.server_row)
            .unwrap();
        assert_eq!(whole.len(), 1);
        assert_eq!(counted.len(), 1);
        let (whole, counted) = (&whole[0], &counted[0]);
        let of = |state: WhatSmoozyDid| {
            whole
                .reckoning
                .iter()
                .filter(|entry| entry.account.state == state)
                .count()
        };
        assert_eq!(counted.job, whole.job);
        assert_eq!(counted.destination, whole.destination);
        assert_eq!(counted.narrowing, whole.narrowing);
        assert_eq!(counted.desired_state, whole.desired_state);
        assert_eq!(counted.entries, whole.reckoning.len());
        assert_eq!(counted.entries, 999);
        assert_eq!(counted.fetched, of(WhatSmoozyDid::Fetched));
        assert_eq!(counted.left_alone, of(WhatSmoozyDid::LeftAlone));
        assert_eq!(counted.owed, of(WhatSmoozyDid::Owed));
        assert_eq!(counted.skipped_by_narrowing, of(WhatSmoozyDid::SkippedByNarrowing));
    }

    /// **A thousand-entry manual request whose second save writes one entry**
    /// — `src/downloading.rs`'s save at a file's arrangement and at its
    /// ending. **The case this bites on is the whole write it replaced.**
    #[test]
    fn a_thousand_entry_manual_request_s_second_save_writes_one_entry() {
        let connection = in_memory();
        let server_row = an_identity(IdentityKind::LocalServer);
        let mut keeper = DownloadRequests::over(&connection);
        let mut request = AManualRequest::confirmed(
            an_identity(IdentityKind::Operation),
            server_row.clone(),
            None,
            b"/Users/artist/Renders".to_vec(),
            (0..1_000)
                .map(|at| {
                    let mut entry = EnumerationEntry::enumerated(
                        format!("/smoozy/out/frame_{at:04}.png").into_bytes(),
                        format!("/Users/artist/Renders/out/frame_{at:04}.png").into_bytes(),
                        1_000 + at,
                        AVouchingMark::NoClaimIsMade,
                    );
                    entry.disposition = Some(Disposition::ToFetch);
                    entry
                })
                .collect(),
            an_identity(IdentityKind::Run),
        );
        keeper.write_it_down(&request).expect("the first write");

        request.entries[417].transfer = Some(an_identity(IdentityKind::Transfer));
        let before = connection.total_changes();
        keeper.one_entry_moved(&request, 417).expect("written");
        let arranged = connection.total_changes() - before;
        request.entries[417]
            .its_outcome_became(UnitOutcome::Done)
            .unwrap();
        let before = connection.total_changes();
        keeper.one_entry_moved(&request, 417).expect("written");
        let ended = connection.total_changes() - before;
        assert!(
            arranged <= 3 && ended <= 4,
            "a save of one moved entry changed {arranged} and {ended} rows"
        );
        let back = keeper.every_request_for(&server_row).unwrap();
        assert_eq!(back, vec![request.clone()]);

        let mut later = request.clone();
        later.entries[417].outcome = Some(UnitOutcome::Failed);
        keeper.one_entry_moved(&later, 417).expect("written");
        assert_eq!(
            keeper.every_request_for(&server_row).unwrap()[0].entries[417].outcome,
            Some(UnitOutcome::Done),
            "an outcome that stands was overwritten"
        );
        assert!(keeper.one_entry_moved(&request, 1_000).is_err());
    }

    /// **A manual request's counts are what the whole reading composes**, and
    /// a record the whole reading skips is skipped here too.
    #[test]
    fn a_manual_request_s_counts_are_what_the_whole_reading_composes() {
        let connection = in_memory();
        let server_row = an_identity(IdentityKind::LocalServer);
        let mut keeper = DownloadRequests::over(&connection);
        let mut one = a_request(&server_row);
        one.entries[0]
            .its_outcome_became(UnitOutcome::Done)
            .unwrap();
        keeper.write_it_down(&one).unwrap();
        let mut two = a_request(&server_row);
        two.entries[0].disposition = Some(Disposition::ToFetch);
        keeper.write_it_down(&two).unwrap();
        let half = a_request(&server_row);
        keeper.write_it_down(&half).unwrap();
        connection
            .execute(
                "DELETE FROM outcome_record_unit WHERE record = ?1 AND position = 1",
                [half.record.as_text()],
            )
            .unwrap();

        let whole = keeper.every_request_for(&server_row).unwrap();
        let counted = keeper.every_request_counted_for(&server_row).unwrap();
        assert_eq!(whole.len(), 2, "the half record is not read back whole");
        assert_eq!(counted.len(), 2);
        for (whole, counted) in whole.iter().zip(&counted) {
            let sending: Vec<&EnumerationEntry> = whole
                .entries
                .iter()
                .filter(|entry| {
                    entry
                        .disposition
                        .as_ref()
                        .is_some_and(Disposition::a_transfer_is_arranged)
                })
                .collect();
            let done = |entry: &&&EnumerationEntry| entry.outcome == Some(UnitOutcome::Done);
            assert_eq!(counted.record, whole.record);
            assert_eq!(counted.destination, whole.destination);
            assert_eq!(counted.files_in_the_whole, sending.len());
            assert_eq!(counted.files_done, sending.iter().filter(done).count());
            assert_eq!(
                counted.bytes_in_the_whole,
                sending
                    .iter()
                    .map(|entry| entry.size_as_enumerated)
                    .sum::<u64>()
            );
            assert_eq!(
                counted.bytes_done,
                sending
                    .iter()
                    .filter(done)
                    .map(|entry| entry.size_as_enumerated)
                    .sum::<u64>()
            );
            assert_eq!(
                counted.every_unit_has_ended,
                sending.iter().all(|entry| entry.outcome.is_some())
            );
            assert_eq!(
                counted.cannot_land,
                whole
                    .entries
                    .iter()
                    .filter(|entry| entry.refusal.is_some())
                    .count()
            );
        }
        assert!(counted[0].every_unit_has_ended);
        assert!(!counted[1].every_unit_has_ended);
    }

    /// **A request written down and not yet decided is counted whole and has
    /// not ended** — step 10c's eleventh-pass fix piece 1, `Download` M1, M6
    /// and M16. This is the record `a_download_is_prepared` writes before M6's
    /// question is put, and it stays this way while the question is open and
    /// while anything holds the request between its record and its
    /// dispositions.
    ///
    /// **Before the fix it counted a whole of nought and *every unit has
    /// ended***, because only units with an arranged disposition were counted,
    /// and the Transfers row read *0 files … done* for the whole of the
    /// eleventh pass's wait on the system's Downloads box.
    #[test]
    fn a_request_with_no_dispositions_is_counted_whole_and_has_not_ended() {
        let connection = in_memory();
        let server_row = an_identity(IdentityKind::LocalServer);
        let mut keeper = DownloadRequests::over(&connection);
        let mut waiting = a_request(&server_row);
        waiting.collision_answer = None;
        for entry in &mut waiting.entries {
            entry.disposition = None;
        }
        keeper.write_it_down(&waiting).unwrap();

        let counted = keeper.every_request_counted_for(&server_row).unwrap();
        assert_eq!(counted.len(), 1);
        // **The whole is the enumerated entries a transfer could be arranged
        // for**: the one whose name the check refused is reported separately,
        // as it is once dispositions are taken.
        assert_eq!(counted[0].files_in_the_whole, 1, "the whole is not the enumeration");
        assert_eq!(counted[0].bytes_in_the_whole, 1_024);
        assert_eq!(counted[0].files_done, 0);
        assert_eq!(counted[0].cannot_land, 1);
        assert!(!counted[0].every_unit_has_ended, "a request nothing has decided reads ended");
        assert!(counted[0].nothing_is_decided);
        assert_eq!(counted[0].opened_by_run, waiting.opened_by_run);

        // **Answered *Stop* at M6, it has ended, counted over what it asked
        // for** (step 10c's Vast-pass fix piece 1): nothing arranged, so every
        // arranged unit — none — has ended, and the whole is the enumeration,
        // so the row reads what did not come.
        waiting.collision_answer = Some(CollisionAnswer::Stop);
        keeper.write_it_down(&waiting).unwrap();
        let stopped = &keeper.every_request_counted_for(&server_row).unwrap()[0];
        assert_eq!((stopped.files_in_the_whole, stopped.bytes_in_the_whole), (1, 1_024));
        assert!(stopped.every_unit_has_ended);
        assert!(stopped.stopped_at_the_question);
        assert!(!stopped.nothing_is_decided);

        // **A disposition taken without an answer** — M6's narrowing, no
        // question asked — is decided too.
        waiting.collision_answer = None;
        waiting.entries[0].disposition = Some(Disposition::ToFetch);
        waiting.entries[1].disposition = Some(Disposition::CannotLand(WhichRefusalItWas::NotRepresentable));
        keeper.write_it_down(&waiting).unwrap();
        let going = &keeper.every_request_counted_for(&server_row).unwrap()[0];
        assert!(!going.nothing_is_decided);
        assert_eq!(going.files_in_the_whole, 1);
        assert!(!going.every_unit_has_ended);
    }

    /// **A request holding no file but a recorded refusal (`Download` M4),
    /// its question still open, has not ended, and its refusal is counted** —
    /// step 10c's Vast-pass fix piece 1. Before it, such a request had ended
    /// the moment it was written, and its row read *done* while the question
    /// was up.
    #[test]
    fn a_request_with_only_a_refusal_and_its_question_open_has_not_ended() {
        let connection = in_memory();
        let server_row = an_identity(IdentityKind::LocalServer);
        let mut keeper = DownloadRequests::over(&connection);
        let mut waiting = a_request(&server_row);
        waiting.entries.clear();
        waiting.collision_answer = None;
        keeper.write_it_down(&waiting).unwrap();
        let counted = &keeper.every_request_counted_for(&server_row).unwrap()[0];
        assert_eq!(counted.could_not_be_read, 1);
        assert!(counted.nothing_is_decided);
        assert!(!counted.every_unit_has_ended);
        assert!(!counted.stopped_at_the_question);
    }

    /// **A unit that failed is counted as failed, with its landing path and
    /// size** — step 10c's thirteenth-pass fix piece 3. **Before the fix the
    /// count carried no failure at all**: a request whose every unit had an
    /// outcome was ended, whatever the outcomes were, and the Transfers row
    /// read *done* on the thirteenth pass for a *Replace* that wrote nothing.
    #[test]
    fn a_failed_unit_is_counted_with_its_landing_path() {
        let connection = in_memory();
        let server_row = an_identity(IdentityKind::LocalServer);
        let mut keeper = DownloadRequests::over(&connection);
        let mut request = a_request(&server_row);
        request.entries[0]
            .its_outcome_became(UnitOutcome::Failed)
            .unwrap();
        keeper.write_it_down(&request).unwrap();
        let counted = &keeper.every_request_counted_for(&server_row).unwrap()[0];
        assert!(counted.every_unit_has_ended);
        assert_eq!(counted.files_done, 0);
        assert_eq!(
            counted.failed_landings,
            vec![(b"/Users/artist/Renders/a/frame_0001.png".to_vec(), 1_024)]
        );

        let mut done = a_request(&server_row);
        done.entries[0].its_outcome_became(UnitOutcome::Done).unwrap();
        keeper.write_it_down(&done).unwrap();
        let counted = keeper.every_request_counted_for(&server_row).unwrap();
        assert!(counted[1].failed_landings.is_empty());
    }
}
