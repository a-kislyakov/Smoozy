//! The upload request in the local record — `Desktop app and delivery` §5, and
//! `File Browser` §3's durable half.
//!
//! **The shape is that document's and the store is this one's**, which is the
//! split `outcome_records`, `key_rows`, `server_rows` and `transfer_records`
//! already stand on. Nothing here decides what an upload request holds.
//!
//! **A2: no module has a durable store of its own**, and `State, identity and
//! recovery` §1.1 says the same from the other side — so `File Browser`'s
//! request cannot keep one, and its `KeepsTheRequest` seam is answered here.
//! §21's supply line already names this record for that module's upload
//! request.
//!
//! **`File Browser` U1 says the request's durable half *is* one `State,
//! identity and recovery` §9.4 outcome record** — not a record beside one. So
//! this module writes both halves of one record: the shared `outcome_record`
//! and `outcome_record_unit` that every module's multi-part action writes, and
//! the three tables of version 9 that hold the fields U1 adds. **They are
//! joined by `(record, position)`, which is §9.4's own ordering**, so there is
//! **one enumeration** rather than two copies of one.
//!
//! **Why the units are written by position and not through
//! [`super::outcome_records::OutcomeRecords`].** That keeper's `a_unit_ended`
//! finds a unit **by role**, which is right for `Keys`, whose three units carry
//! three different roles. **`File Browser` §11 gives every unit of an upload
//! request the same role** — *one file's transfer*, the Partition unit column
//! word for word — and says outright that what tells two units apart is §9.4's
//! own ordering and nothing else. **A write by role would therefore reach an
//! arbitrary one of three hundred textures.** The immutability that keeper
//! enforces is kept here in the same way it keeps it: the update is conditional
//! on the outcome still being absent, in one statement.
//!
//! **How a clause marker is written in this crate, and it is a rule rather than
//! a habit.** A marker with no document beside it means a clause of this
//! crate's own plan, `Preparation/Architecture/Desktop app and delivery.md`.
//! Every other document's marker is written with that document — at the site,
//! or in this file's own header list below. **Where a marker is a clause of
//! this crate's own plan *and* of the document meant, the header list is not
//! enough and the document is written out at the site.**
//!
//! **Markers used below that belong to another document, each with the document
//! it belongs to.** `File Browser` U1, U4, U8, U9a, U9b, U11, U12, N2,
//! §11. `State, identity and recovery` §9.4, §11.1, §2.1.
//!
//! **A marker this crate's own plan also defines is deliberately not listed
//! above, whichever document is meant, and is written out with its document
//! at every site instead** — the header list cannot tell the two apart, and a
//! bare one would resolve to a real and wrong clause.
//!
//! **This crate's own plan defines an F19, a P5, an X5, an X9, an L1, an L4, an
//! N2 and a U1 of its own**, so none of those is listed above.
//!
//! **L4 holds: no field of this record is ever placed in a filename.** **L10
//! holds: no secret value is written.**

use rusqlite::OptionalExtension;
use smoozy_file_browser::already_exists::CollisionAnswer;
use smoozy_file_browser::request::{
    self, Disposition, EnumerationEntry, KeepsTheRequest, TheTelling, UploadRequest,
};
use smoozy_state::identities::{Identity, IdentityKind};
use smoozy_state::outcomes::{self, Refused};

use super::outcome_records::{as_text, from_text, the_store};
use super::{moment, nanoseconds};

/// The keeper, over one open local record.
pub struct UploadRequests<'a> {
    connection: &'a rusqlite::Connection,
}

impl<'a> UploadRequests<'a> {
    pub fn over(connection: &'a rusqlite::Connection) -> Self {
        UploadRequests { connection }
    }
}

/// **`File Browser` U1's four dispositions, written down.** Three words and no
/// more per value, and **a null column is a request whose answer has not been
/// applied yet**, which that document's U4 makes a real state rather than a
/// missing value.
fn disposition_as_text(disposition: &Disposition) -> (&'static str, Option<Vec<u8>>, Option<String>) {
    match disposition {
        Disposition::ToSend => ("to send", None, None),
        Disposition::SkipExisting => ("skip existing", None, None),
        // **`File Browser` X9's generated landing path**, computed once when
        // the answer is applied so a resume re-derives nothing.
        Disposition::RenamedUnderKeepBoth(landing) => ("keep both", Some(landing.clone()), None),
        // **`File Browser` U12's fourth**, naming the operation identity of the
        // act `Dependency Upload & Remap` opens.
        Disposition::HandedOver { the_act } => ("handed over", None, Some(the_act.as_text())),
    }
}

/// Read one back. **A row whose disposition text is none of the four, or whose
/// companion value will not read back, is read as no disposition at all**
/// rather than guessed at — the same refusal `server_rows` makes for an
/// identifier that will not read back, and it is the safe direction: `File
/// Browser` U4 already makes *no disposition yet* a real state.
fn disposition_from_text(
    disposition: Option<String>,
    keep_both_landing_path: Option<Vec<u8>>,
    handed_to_act: Option<String>,
) -> Option<Disposition> {
    match disposition.as_deref() {
        Some("to send") => Some(Disposition::ToSend),
        Some("skip existing") => Some(Disposition::SkipExisting),
        Some("keep both") => keep_both_landing_path.map(Disposition::RenamedUnderKeepBoth),
        Some("handed over") => handed_to_act
            .and_then(|text| Identity::read_back(IdentityKind::Operation, &text))
            .map(|the_act| Disposition::HandedOver { the_act }),
        _ => None,
    }
}

/// **`File Browser` U4's answer — the one field of the request that cannot be
/// rebuilt from the machine or from the transfer records**, which is why its
/// durability is what makes *the answers the user already gave still stand*
/// true by construction.
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

impl KeepsTheRequest for UploadRequests<'_> {
    /// **`File Browser` U1's write, and `Server protocol and transfers` S8's
    /// ordering applies to it**: the enumeration is durable before the first
    /// byte moves, and every later change to the request goes down through this
    /// same call at the moment it becomes true.
    ///
    /// **Both halves of the one record go down in one transaction**, so an
    /// interruption inside this call leaves either the whole enumeration or
    /// none of it — which is `State, identity and recovery` P5 item 1's, at
    /// that document's §9.4,
    /// *so an interruption cannot lose the fact that a unit was owed*, and it
    /// is why the §9.4 half is written from here rather than by a second call
    /// that would open a transaction of its own.
    ///
    /// **An outcome already written is never overwritten** (`State, identity
    /// and recovery` F19, at that document's §8.6). The update carries
    /// `outcome IS NULL`, so a
    /// second write of a unit that has ended changes nothing and the value that
    /// stands is the first one written. **It is not reported as a refusal
    /// here**: this call writes the whole request rather than one unit, so a
    /// request holding an ending the store already has is the ordinary case at
    /// every later write, not a caller doing something wrong.
    fn write_it_down(&mut self, request: &UploadRequest) -> Result<(), Refused> {
        let record = request.record.as_text();
        let role = request::the_role_every_unit_carries();
        self.in_one_transaction(|| {
            self.the_requests_own_row_is_written(request)?;
            for (position, entry) in request.entries.iter().enumerate() {
                self.one_entry_is_written(&record, role, position, entry)?;
            }

            // **The telling is rows and its emptiness is its absence** (`File
            // Browser` U1: *a request that disturbed nothing carries neither
            // field*). No row is the whole of *no telling* and no flag stands
            // beside it, which is the shape version 9 was written in.
            self.connection
                .prepare_cached("DELETE FROM upload_request_telling_job WHERE record = ?1")?
                .execute([&record])?;
            if let Some(telling) = &request.telling {
                let mut insert = self.connection.prepare_cached(
                    "INSERT INTO upload_request_telling_job (record, position, job)
                     VALUES (?1, ?2, ?3)",
                )?;
                for (position, job) in telling.the_jobs.iter().enumerate() {
                    insert.execute(rusqlite::params![record, position as i64, job.as_text()])?;
                }
            }
            Ok(())
        })
    }

    /// Every request record held for a server row — **what a fresh sitting
    /// reads** (`File Browser` U8), and what `File Browser` U9b's reason line
    /// is composed over.
    ///
    /// **A record whose identities will not read back is skipped rather than
    /// given invented ones**, which is `State, identity and recovery` §2.1 and
    /// the same refusal `server_rows` and `key_rows` make. **So is a record
    /// whose enumeration and whose §9.4 units do not line up**: half a record
    /// read back would say a unit was never owed, which is the one thing §9.4
    /// item 1 exists to prevent.
    ///
    /// **The order is the local record's own write order, oldest first, and it
    /// is supplied by `rowid`.** `File Browser` N6 says the answer to *where
    /// did this file come from* is the most recently published entry, *"recency
    /// by the local record's own write order and never by comparing clocks"*,
    /// and its `where_it_came_from` keeps the last match it sees — so what this
    /// statement orders by **is** that clause's recency. **It is not `record`**:
    /// a record is `State, identity and recovery` §2.1's identity, sixteen
    /// random bytes written as hexadecimal, and that document's rule 2 says
    /// outright *"Nothing parses one, sorts by one, or infers anything from two
    /// being adjacent."* **`rowid` is this store's own established answer for
    /// write order** — `server_row`, `key_row`, `owed_erase` and
    /// `transfer_record` all read it the same way — and [`Self::write_it_down`]
    /// keeps the row rather than replacing it so a later write cannot move a
    /// request down the order.
    fn every_request_for(&self, server_row: &Identity) -> Result<Vec<UploadRequest>, Refused> {
        let mut statement = self
            .connection
            .prepare(
                "SELECT record, installation, destination, collision_answer, opened_by_run
                 FROM upload_request WHERE server_row = ?1 ORDER BY rowid",
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
            requests.push(UploadRequest {
                record: identity,
                server_row: server_row.clone(),
                installation: installation
                    .and_then(|text| Identity::read_back(IdentityKind::Installation, &text)),
                destination,
                entries,
                collision_answer: answer_from_text(collision_answer),
                telling: self.telling_of(&record)?,
                opened_by_run,
            });
        }
        Ok(requests)
    }

    /// **`State, identity and recovery` §11.1: user-held.** It ends when the
    /// user removes it and never otherwise — no age, no cap, no sweep.
    ///
    /// **Both halves of the one record go in one transaction**, the §9.4 half
    /// included: a §9.4 record whose `File Browser` U1 fields had gone, or the
    /// other way about, would be half a record either way. **The shared
    /// keeper's own removal is not called from here** because it opens a
    /// transaction of its own and the removal has to be one.
    ///
    /// **What it costs is named where the user is asked and not here** —
    /// removing a request record removes the provenance of its files with it
    /// (`File Browser` N8, taken at §21).
    fn the_user_removed_it(&mut self, record: &Identity) -> Result<(), Refused> {
        let record = record.as_text();
        self.connection
            .execute("BEGIN IMMEDIATE", [])
            .map_err(the_store)?;
        let removed = (|| -> Result<(), rusqlite::Error> {
            for statement in [
                "DELETE FROM upload_request_telling_job WHERE record = ?1",
                "DELETE FROM upload_request_entry WHERE record = ?1",
                "DELETE FROM upload_request WHERE record = ?1",
                "DELETE FROM outcome_record_unit WHERE record = ?1",
                "DELETE FROM outcome_record WHERE identity = ?1",
            ] {
                self.connection.execute(statement, [&record])?;
            }
            Ok(())
        })();
        match removed {
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

impl UploadRequests<'_> {
    /// **`File Browser` W8's telling for an upload: the request with its
    /// telling, and each affected job's mark, in one write** — before the act's
    /// first effect on the machine. Step 10c's Files' owed acts piece 2.
    ///
    /// *"For an upload that record is U1's, whose field list names them"*, so
    /// the jobs go down as the request's own telling rows; and *"the same write
    /// lands each affected job's disturbed-file mark on that job's local
    /// record"* (`Queue & History` N21), reading *may have been disturbed*.
    /// **Both or neither**: a telling standing without its marks would be a
    /// job told nothing, and a mark without its telling a mark no act owns.
    ///
    /// **A mark onto a job whose record is gone writes nothing and is not a
    /// refusal** — there is no entry to carry it.
    pub fn the_telling_is_written(
        &mut self,
        request: &UploadRequest,
        marks: &[(Identity, smoozy_queue_history::record::ADisturbance)],
    ) -> Result<(), Refused> {
        let record = request.record.as_text();
        let role = request::the_role_every_unit_carries();
        let connection = self.connection;
        self.in_one_transaction(|| {
            self.the_requests_own_row_is_written(request)?;
            for (position, entry) in request.entries.iter().enumerate() {
                self.one_entry_is_written(&record, role, position, entry)?;
            }
            connection
                .prepare_cached("DELETE FROM upload_request_telling_job WHERE record = ?1")?
                .execute([&record])?;
            if let Some(telling) = &request.telling {
                let mut insert = connection.prepare_cached(
                    "INSERT INTO upload_request_telling_job (record, position, job)
                     VALUES (?1, ?2, ?3)",
                )?;
                for (position, job) in telling.the_jobs.iter().enumerate() {
                    insert.execute(rusqlite::params![record, position as i64, job.as_text()])?;
                }
            }
            for (job, mark) in marks {
                super::job_records::a_mark_is_appended_to(connection, job, mark)?;
            }
            Ok(())
        })
    }

    /// **One unit's entry moved, and only it goes down** — the request's own
    /// row and the one entry at `at`, in one transaction. Added 2026-09-28,
    /// step 10c's transfer wave piece 1.
    ///
    /// **The case [`KeepsTheRequest::write_it_down`] was first written for** is
    /// a request of a few files, where writing every entry again at every
    /// change cost nothing. **The case it now also meets** is a 10,000-file
    /// folder: `src/uploading.rs` wrote the request twice per file — when the
    /// file's transfer was arranged and when it ended — so each file wrote
    /// 10,000 entries of about three statements each, twice, and the folder
    /// cost the square of its size. **Only the entry that moved has changed at
    /// either moment**, so only it is written, and the whole write stays for
    /// the moments the whole request changes (`File Browser` U1's first write,
    /// `File Browser` U4's answer).
    ///
    /// **`State, identity and recovery` F19 is kept the same way the whole
    /// write keeps it**: the unit's outcome is set only where none is written
    /// yet, in one statement, so an outcome that stands is never overwritten.
    ///
    /// **`File Browser`'s `KeepsTheRequest` does not change**: it is the
    /// module's seam, and the door in this crate reaches the store directly for
    /// this one write, as `src/uploading.rs` already did for every other.
    ///
    /// **A savepoint rather than `BEGIN`**, so the write is a transaction of
    /// its own where nothing else holds one and nests where a caller does —
    /// `src/uploading.rs` puts it in the same save as the file's transfer
    /// record.
    pub fn one_entry_moved(&mut self, request: &UploadRequest, at: usize) -> Result<(), Refused> {
        let Some(entry) = request.entries.get(at) else {
            return Err(Refused::TheStore(format!(
                "the request has no entry at {at}, so nothing was written"
            )));
        };
        self.this_entry_moved(request, at, entry)
    }

    /// **[`Self::one_entry_moved`], with the entry handed in** rather than
    /// read off the request — for a caller that writes an entry's next state
    /// in the same save as the act that made it true, before its own copy of
    /// the request has moved.
    pub fn this_entry_moved(
        &mut self,
        request: &UploadRequest,
        at: usize,
        entry: &EnumerationEntry,
    ) -> Result<(), Refused> {
        if at >= request.entries.len() {
            return Err(Refused::TheStore(format!(
                "the request has no entry at {at}, so nothing was written"
            )));
        }
        let record = request.record.as_text();
        let role = request::the_role_every_unit_carries();
        self.connection
            .execute_batch("SAVEPOINT one_upload_entry_moved")
            .map_err(the_store)?;
        let written = self
            .the_requests_own_row_is_written(request)
            .and_then(|()| self.one_entry_is_written(&record, role, at, entry))
            .and_then(|()| self.connection.execute_batch("RELEASE one_upload_entry_moved"));
        if let Err(error) = written {
            let _ = self.connection.execute_batch(
                "ROLLBACK TO one_upload_entry_moved; RELEASE one_upload_entry_moved",
            );
            return Err(the_store(error));
        }
        Ok(())
    }

    /// **Every request held for a server row, counted by the store rather than
    /// read back** — what the Transfers screen composes a row from. Added
    /// 2026-09-28, step 10c's transfer wave piece 1.
    ///
    /// **`File Browser` U11's counts, over the units a transfer is arranged
    /// for**: how many, how many are done, their enumerated sizes and the done
    /// ones' sizes, and whether every one of them has an outcome. **The files
    /// that failed are named**, because the screen draws a row for each one
    /// whose write the machine refused (`File Browser` P1 and `File Browser`
    /// P6a).
    ///
    /// **The case the screen's reading was first written for** is a request of
    /// a few files, where [`KeepsTheRequest::every_request_for`] reading every
    /// entry of every request back on every ask cost nothing. **The case it now
    /// also meets** is a 10,000-file request asked about at every file's end
    /// while the record's lock is held — the whole enumeration composed back
    /// into memory, to be counted and thrown away.
    ///
    /// **The same records are skipped as [`KeepsTheRequest::every_request_for`]
    /// skips**, so the two readings cannot disagree about which requests there
    /// are: a record whose identities will not read back, and one whose
    /// enumeration and §9.4 units do not line up. **What a disposition or an
    /// outcome reads as is this file's own writing of it**
    /// ([`disposition_as_text`], and `outcome_records::as_text` for the three
    /// outcomes), and a test below holds the counts to what the whole reading
    /// composes.
    pub fn every_request_counted_for(
        &self,
        server_row: &Identity,
    ) -> Result<Vec<AnUploadRequestCounted>, Refused> {
        let mut statement = self
            .connection
            .prepare_cached(
                "SELECT record, destination, opened_by_run, collision_answer FROM upload_request
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
            // key and joined to its unit by the unit's key, so a reading costs
            // the request's length and nothing of any other request's.
            let numbers: [i64; 10] = self
                .connection
                .prepare_cached(&format!(
                    "SELECT count(*), count(u.position),
                            coalesce(sum({SENT}), 0),
                            coalesce(sum({SENT} AND u.outcome = 'done'), 0),
                            coalesce(sum(CASE WHEN {SENT} THEN e.size_as_enumerated END), 0),
                            coalesce(sum(CASE WHEN {SENT} AND u.outcome = 'done'
                                              THEN e.size_as_enumerated END), 0),
                            coalesce(sum({SENT} AND (u.outcome IS NULL OR u.outcome NOT IN
                                         ('done', 'failed', 'not attempted'))), 0),
                            coalesce(sum(e.disposition IS NULL), 0),
                            coalesce(sum(e.size_as_enumerated), 0),
                            count(*)
                     FROM upload_request_entry AS e
                     LEFT JOIN outcome_record_unit AS u
                       ON u.record = e.record AND u.position = e.position
                     WHERE e.record = ?1"
                ))
                .map_err(the_store)?
                .query_row([&record], |row| {
                    let mut numbers = [0i64; 10];
                    for (at, number) in numbers.iter_mut().enumerate() {
                        *number = row.get(at)?;
                    }
                    Ok(numbers)
                })
                .map_err(the_store)?;
            let [owed, joined, whole, done, bytes, bytes_done, open, undecided, bytes_enumerated, enumerated] =
                numbers;
            if owed != joined {
                continue;
            }
            // **The download's rule, kept for an upload** (`Download` M1 and M6
            // there, `File Browser` U1 and U4 here — step 10c's eleventh-pass
            // fix piece 1 and Vast-pass fix piece 1, in
            // `crate::record::download_records`). **Nothing decided**: no entry
            // holds a disposition and no collision answer is written — the
            // request as `File Browser` U1 writes it before the walk, and as it
            // stays where the walk never ran. **Stopped at the question**: the
            // user answered *Stop* (U4). **Either way nothing was sent**, and
            // the whole is what the request asked for, so its row reads what
            // did not go up rather than *0 files · done*. The Hyperstack check
            // before 0.1.7 met the first: a file dragged in over a connection
            // that was gone read *0 files into /home/ubuntu · done* at once.
            // Step 10c's check-before-0.1.7 fix piece 1.
            let nothing_is_decided = enumerated > 0 && undecided == enumerated && !answered;
            let (whole, bytes) = if nothing_is_decided || stopped {
                (enumerated, bytes_enumerated)
            } else {
                (whole, bytes)
            };
            let failed_landings: Vec<(Vec<u8>, u64)> = self
                .connection
                .prepare_cached(&format!(
                    "SELECT e.landing_path, e.size_as_enumerated FROM upload_request_entry AS e
                     JOIN outcome_record_unit AS u
                       ON u.record = e.record AND u.position = e.position
                     WHERE e.record = ?1 AND u.outcome = 'failed' AND {SENT}
                     ORDER BY e.position"
                ))
                .map_err(the_store)?
                .query_map([&record], |row| {
                    Ok((row.get::<_, Vec<u8>>(0)?, row.get::<_, i64>(1)? as u64))
                })
                .map_err(the_store)?
                .collect::<Result<_, _>>()
                .map_err(the_store)?;
            let changed_landings: Vec<Vec<u8>> = self
                .connection
                .prepare_cached(&format!(
                    "SELECT e.landing_path FROM upload_request_entry AS e
                     JOIN outcome_record_unit AS u
                       ON u.record = e.record AND u.position = e.position
                     JOIN transfer_record AS t ON t.transfer = e.transfer
                     WHERE e.record = ?1 AND u.outcome = 'failed' AND {SENT}
                       AND t.ending = 'stopped' AND t.stopped_reason = 'the source changed'
                     ORDER BY e.position"
                ))
                .map_err(the_store)?
                .query_map([&record], |row| row.get::<_, Vec<u8>>(0))
                .map_err(the_store)?
                .collect::<Result<_, _>>()
                .map_err(the_store)?;
            counted.push(AnUploadRequestCounted {
                record: identity,
                destination,
                changed_landings,
                files_in_the_whole: whole as usize,
                files_done: done as usize,
                bytes_in_the_whole: bytes as u64,
                bytes_done: bytes_done as u64,
                every_unit_has_ended: !nothing_is_decided && open == 0,
                failed_landings,
                nothing_is_decided,
                stopped_at_the_question: stopped,
                opened_by_run,
            });
        }
        Ok(counted)
    }

    /// **The request's own row, and the §9.4 record it is.**
    fn the_requests_own_row_is_written(&self, request: &UploadRequest) -> rusqlite::Result<()> {
        let record = request.record.as_text();
        let start = super::transfer_records::start_for_parent(self.connection, "upload_request", "record", &record, false)?;
        self.connection
            .prepare_cached(
                // **An upsert that keeps the row, and it is load-bearing.**
                // `File Browser` N6 answers *where did this file come from* by
                // **the local record's own write order**, and
                // [`KeepsTheRequest::every_request_for`] supplies that order
                // off `rowid`. A replace deletes the row and inserts a new one,
                // so a request written again later — every write after the
                // first one is — would move to the end of that order and change
                // the answer. **The conflict updates the row in place
                // instead**, so a request's place in the order is the moment it
                // was first written down. The two entry tables are keyed and
                // read by `position`, which is `State, identity and recovery`
                // §9.4's own ordering, so neither depends on this.
                "INSERT INTO upload_request
                   (record, server_row, installation, destination, collision_answer, opened_by_run, started_at_utc, start_order)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)
                 ON CONFLICT(record) DO UPDATE SET
                   server_row = excluded.server_row,
                   installation = excluded.installation,
                   destination = excluded.destination,
                   collision_answer = excluded.collision_answer,
                   opened_by_run = excluded.opened_by_run",
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

        // **The §9.4 half of the same record.** `File Browser` U1: the
        // request's durable half *is* one outcome record, so this is not a
        // second record beside it.
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
        // **The role is `File Browser` §11's own words and the same for every
        // unit**; the position is what tells two apart.
        self.connection
            .prepare_cached(
                "INSERT OR IGNORE INTO outcome_record_unit
                   (record, position, role, outcome, stopped_by)
                 VALUES (?1, ?2, ?3, NULL, NULL)",
            )?
            .execute(rusqlite::params![record, position, role])?;
        if let Some(outcome) = &entry.outcome {
            let (outcome, stopped_by) = as_text(outcome);
            self.connection
                .prepare_cached(
                    "UPDATE outcome_record_unit SET outcome = ?1, stopped_by = ?2
                     WHERE record = ?3 AND position = ?4 AND outcome IS NULL",
                )?
                .execute(rusqlite::params![outcome, stopped_by, record, position])?;
        }

        let (disposition, keep_both, handed_to) = match &entry.disposition {
            Some(disposition) => {
                let (text, keep_both, handed_to) = disposition_as_text(disposition);
                (Some(text), keep_both, handed_to)
            }
            None => (None, None, None),
        };
        self.connection
            .prepare_cached(
                "INSERT OR REPLACE INTO upload_request_entry
                   (record, position, source_path, landing_path, size_as_enumerated,
                    disposition, keep_both_landing_path, handed_to_act, transfer,
                    outcome_written_at)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
            )?
            .execute(rusqlite::params![
                record,
                position,
                // **The two paths are bytes** — `File Browser` U1 holds the
                // source path as stored on disk, `File Browser` N2 rewrites
                // nothing, and `File Browser` X5's comparison is byte for byte.
                entry.source_path,
                entry.landing_path,
                entry.size_as_enumerated as i64,
                disposition,
                keep_both,
                handed_to,
                entry.transfer.as_ref().map(|identity| identity.as_text()),
                // **`File Browser` U9a's moment**, the only moment any upload
                // record holds. [`nanoseconds`] answers a failure rather than
                // zero since 2026-09-07 (`Access` round 1, F-9), and **this
                // module's behaviour is deliberately unchanged by that** —
                // ruling 6 of that round: the repair belongs to the licence
                // record and every other caller was read and left where it
                // stood. U9a's moment is drawn from the desktop's own clock at
                // the durable write and is not a value any wire can put past
                // 2262; the zero is what this row module already wrote and it
                // is now written on purpose rather than by the helper's
                // default.
                entry
                    .outcome_written_at
                    .map(|at| nanoseconds(at).unwrap_or(0)),
            ])?;
        Ok(())
    }

    /// **Both halves of the one record go down in one transaction**, so an
    /// interruption inside a write leaves either all of it or none of it —
    /// `State, identity and recovery` P5 item 1, at that document's §9.4.
    fn in_one_transaction(
        &self,
        write: impl FnOnce() -> rusqlite::Result<()>,
    ) -> Result<(), Refused> {
        self.connection
            .execute("BEGIN IMMEDIATE", [])
            .map_err(the_store)?;
        match write() {
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

    /// **The enumeration, in §9.4's own order, read as one thing.**
    ///
    /// The join is on `(record, position)`, which is §9.4's ordering field: the
    /// unit's outcome comes back beside the entry it belongs to rather than
    /// from a second list somebody has to line up. `None` where the two halves
    /// disagree about how many units there are.
    fn entries_of(&self, record: &str) -> Result<Option<Vec<EnumerationEntry>>, Refused> {
        let owed: i64 = self
            .connection
            .query_row(
                "SELECT count(*) FROM upload_request_entry WHERE record = ?1",
                [record],
                |row| row.get(0),
            )
            .map_err(the_store)?;

        let mut statement = self
            .connection
            .prepare(
                "SELECT entry.source_path, entry.landing_path, entry.size_as_enumerated,
                        entry.disposition, entry.keep_both_landing_path, entry.handed_to_act,
                        entry.transfer, entry.outcome_written_at,
                        unit.outcome, unit.stopped_by
                 FROM upload_request_entry AS entry
                 JOIN outcome_record_unit AS unit
                   ON unit.record = entry.record AND unit.position = entry.position
                 WHERE entry.record = ?1
                 ORDER BY entry.position",
            )
            .map_err(the_store)?;
        let rows = statement
            .query_map([record], |row| {
                Ok(EnumerationEntry {
                    source_path: row.get(0)?,
                    landing_path: row.get(1)?,
                    size_as_enumerated: row.get::<_, i64>(2)? as u64,
                    disposition: disposition_from_text(row.get(3)?, row.get(4)?, row.get(5)?),
                    transfer: row
                        .get::<_, Option<String>>(6)?
                        .and_then(|text| Identity::read_back(IdentityKind::Transfer, &text)),
                    outcome_written_at: row.get::<_, Option<i64>>(7)?.map(moment),
                    outcome: from_text(row.get(8)?, row.get(9)?),
                })
            })
            .map_err(the_store)?;

        let mut entries = Vec::new();
        for row in rows {
            entries.push(row.map_err(the_store)?);
        }
        if entries.len() as i64 != owed {
            return Ok(None);
        }
        Ok(Some(entries))
    }

    /// **`File Browser` U1's telling.** Rows, in the order they were written,
    /// and **no rows at all is no telling** — which is what that clause's *a
    /// request that disturbed nothing carries neither field* asks for. A
    /// telling naming no job is not a state §8's live-job warning produces, so
    /// nothing is lost by the two being one absence.
    fn telling_of(&self, record: &str) -> Result<Option<TheTelling>, Refused> {
        let mut statement = self
            .connection
            .prepare(
                "SELECT job FROM upload_request_telling_job
                 WHERE record = ?1 ORDER BY position",
            )
            .map_err(the_store)?;
        let rows = statement
            .query_map([record], |row| row.get::<_, String>(0))
            .map_err(the_store)?;
        let mut the_jobs = Vec::new();
        for row in rows {
            let text = row.map_err(the_store)?;
            // **A job identity that will not read back is not invented**, and a
            // telling that lost one is not a true telling — so the whole
            // telling is refused rather than shortened.
            let Some(job) = Identity::read_back(IdentityKind::Job, &text) else {
                return Ok(None);
            };
            the_jobs.push(job);
        }
        Ok(if the_jobs.is_empty() {
            None
        } else {
            Some(TheTelling { the_jobs })
        })
    }
}

/// **One request record, by its own identity.** Nothing in `KeepsTheRequest`
/// asks for it; it is here because a test that closes and re-opens the record
/// has to be able to ask for exactly the one it wrote, and because §21's
/// presentation of a leftover names a record rather than a server row.
pub fn read(
    connection: &rusqlite::Connection,
    record: &Identity,
) -> Result<Option<UploadRequest>, Refused> {
    let server_row: Option<String> = connection
        .query_row(
            "SELECT server_row FROM upload_request WHERE record = ?1",
            [record.as_text()],
            |row| row.get(0),
        )
        .optional()
        .map_err(the_store)?;
    let Some(server_row) = server_row else {
        return Ok(None);
    };
    let Some(server_row) = Identity::read_back(IdentityKind::LocalServer, &server_row) else {
        return Ok(None);
    };
    let keeper = UploadRequests::over(connection);
    Ok(keeper
        .every_request_for(&server_row)?
        .into_iter()
        .find(|held| &held.record == record))
}

/// A build send has ended and its temporary source is about to go. Its
/// unfinished transfers cannot be resumed; only this request's units change.
pub(crate) fn a_build_send_has_ended(
    connection: &rusqlite::Connection,
    record: &Identity,
) -> Result<(), Refused> {
    use smoozy_server_protocol::transfers::Ending;
    let Some(mut request) = read(connection, record)? else {
        return Err(Refused::TheStore("the build upload's record is missing".to_string()));
    };
    for entry in &mut request.entries {
        if !entry.disposition.as_ref().is_some_and(Disposition::a_transfer_is_arranged) {
            continue;
        }
        if let Some(transfer) = &entry.transfer {
            let mut part = super::transfer_records::read(connection, transfer)
                .map_err(|why| Refused::TheStore(format!("{why:?}")))?
                .ok_or_else(|| Refused::TheStore("the build upload's transfer is missing".to_string()))?;
            if matches!(part.ending, None | Some(Ending::Interrupted | Ending::Stalled)) {
                part.ending = Some(Ending::Failed);
                part.cleanup_is_owed = true;
                super::transfer_records::put(connection, &part)
                    .map_err(|why| Refused::TheStore(format!("{why:?}")))?;
            }
        }
        if entry.outcome.is_none() {
            entry.its_outcome_became(outcomes::UnitOutcome::Failed, std::time::SystemTime::now())?;
        }
    }
    UploadRequests::over(connection).write_it_down(&request)
}

/// **Whether an entry's disposition arranges a transfer, as the store reads
/// it** — `File Browser`'s `a_transfer_is_arranged` (*to send*, or *keep both*
/// with its landing path), in [`disposition_as_text`]'s own words and with
/// [`disposition_from_text`]'s rule that *keep both* without its landing path
/// is no disposition at all.
const SENT: &str = "(e.disposition = 'to send'
     OR (e.disposition = 'keep both' AND e.keep_both_landing_path IS NOT NULL))";

/// **One upload request, counted by the store** — what
/// [`UploadRequests::every_request_counted_for`] answers, and what the
/// Transfers screen composes a row from.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AnUploadRequestCounted {
    pub record: Identity,
    pub destination: Vec<u8>,
    /// The units a transfer is arranged for (`File Browser` U11).
    pub files_in_the_whole: usize,
    /// Of those, the ones *done*.
    pub files_done: usize,
    /// Their enumerated sizes, summed.
    pub bytes_in_the_whole: u64,
    /// The done ones' enumerated sizes, summed.
    pub bytes_done: u64,
    /// **Every unit a transfer is arranged for has an outcome.**
    pub every_unit_has_ended: bool,
    /// **The landing paths of those that failed, each with its enumerated
    /// size**, in the enumeration's order.
    pub failed_landings: Vec<(Vec<u8>, u64)>,
    /// **Of those, the ones `File Browser` K8 or K9 stopped because the file
    /// changed while it was going up** — the transfer record's own ending,
    /// *stopped: the source changed* — which `Design/Transfers § A file that
    /// changed while it was going up` draws with *Upload it again*. The 38's
    /// piece 9, item 17.
    pub changed_landings: Vec<Vec<u8>>,
    /// **No entry holds a disposition and no collision answer is written** —
    /// `File Browser` U1's request before the walk, or one whose walk never
    /// ran. Its whole is then its enumeration, and it has not ended by its
    /// units. Step 10c's check-before-0.1.7 fix piece 1, on the download's
    /// rule (`AManualRequestCounted::nothing_is_decided`).
    pub nothing_is_decided: bool,
    /// **The user answered `File Browser` U4's question *Stop*.** Nothing was
    /// sent, and the whole is what the request asked for.
    pub stopped_at_the_question: bool,
    /// **The run that opened the request** (`File Browser` U1's own field).
    pub opened_by_run: Identity,
}

/// The record identity kind an upload request carries, named once so a test and
/// a caller cannot disagree: it is `State, identity and recovery` §9.4's
/// operation identity, which [`outcomes::KIND`] fixes for every outcome record.
pub const THE_RECORD_IDENTITY_KIND: IdentityKind = outcomes::KIND;

#[cfg(test)]
mod tests {
    use super::*;
    use smoozy_state::outcomes::UnitOutcome;
    use std::time::{Duration, SystemTime};

    fn an_identity(kind: IdentityKind) -> Identity {
        Identity::issue(kind).expect("this machine supplies random bytes")
    }

    /// Every version, the way a record on a user's computer is made — the
    /// migration list runs them all, so a test standing on version 9 alone
    /// would be writing against a shape no record has.
    fn a_record() -> rusqlite::Connection {
        let connection = rusqlite::Connection::open_in_memory().unwrap();
        the_tables(&connection);
        connection
    }

    fn the_tables(connection: &rusqlite::Connection) {
        for migration in super::super::migrations::ALL {
            super::super::migrations::run_one_for_tests(connection, migration)
                .expect("the tables");
        }
    }

    /// A request with all four dispositions in it, two units ended and one not,
    /// paths that are **not valid text**, and a telling.
    fn a_request(server_row: &Identity, run: &Identity) -> UploadRequest {
        let mut entries = vec![
            EnumerationEntry::enumerated(vec![0xff, b'/', b'a'], b"/machine/a".to_vec(), 11),
            EnumerationEntry::enumerated(b"/home/b".to_vec(), vec![b'/', 0xfe, b'b'], 22),
            EnumerationEntry::enumerated(b"/home/c".to_vec(), b"/machine/c".to_vec(), 33),
            EnumerationEntry::enumerated(b"/home/d".to_vec(), b"/machine/d".to_vec(), 44),
        ];
        entries[0].disposition = Some(Disposition::ToSend);
        entries[0].transfer = Some(an_identity(IdentityKind::Transfer));
        entries[0]
            .its_outcome_became(
                UnitOutcome::Done,
                SystemTime::UNIX_EPOCH + Duration::from_nanos(1_700_000_000_123_456_789),
            )
            .unwrap();
        entries[1].disposition = Some(Disposition::RenamedUnderKeepBoth(
            b"/machine/b (2)".to_vec(),
        ));
        entries[1].transfer = Some(an_identity(IdentityKind::Transfer));
        entries[2].disposition = Some(Disposition::SkipExisting);
        entries[3].disposition = Some(Disposition::HandedOver {
            the_act: an_identity(IdentityKind::Operation),
        });
        entries[3]
            .its_outcome_became(
                UnitOutcome::NotAttempted {
                    stopped_by: "handed to Dependency Upload & Remap".to_string(),
                },
                SystemTime::UNIX_EPOCH + Duration::from_nanos(1_700_000_000_987_654_321),
            )
            .unwrap();

        let mut request = UploadRequest::confirmed(
            request::open_an_upload_request().expect("the answer handed in says open"),
            an_identity(THE_RECORD_IDENTITY_KIND),
            server_row.clone(),
            Some(an_identity(IdentityKind::Installation)),
            b"/machine/destination".to_vec(),
            entries,
            run.clone(),
        );
        request.collision_answer = Some(CollisionAnswer::KeepBoth);
        request.telling = Some(TheTelling {
            the_jobs: vec![an_identity(IdentityKind::Job), an_identity(IdentityKind::Job)],
        });
        request
    }

    #[test]
    fn an_ended_build_send_fails_its_unfinished_transfers_and_keeps_other_work() {
        use smoozy_server_protocol::transfers::{Direction, Ending, TransferRecord};
        let connection = a_record();
        let server = an_identity(IdentityKind::LocalServer);
        let run = an_identity(IdentityKind::Run);
        let mut request = a_request(&server, &run);
        let mut not_started = EnumerationEntry::enumerated(b"/temporary/build".to_vec(), b"/staging/build".to_vec(), 10);
        not_started.disposition = Some(Disposition::ToSend);
        request.entries.push(not_started);
        let transfer = |id: Identity| TransferRecord::arranged(id, Direction::Up, server.clone(), request.installation.clone(),
            "/temporary/build".to_string(), "/staging/build".to_string(), "/staging/.smoozy-staging", 10, run.clone());
        let mut published = transfer(request.entries[0].transfer.clone().unwrap());
        published.ending = Some(Ending::Published);
        let mut unfinished = transfer(request.entries[1].transfer.clone().unwrap());
        unfinished.ending = Some(Ending::Interrupted);
        let mut unrelated = transfer(an_identity(IdentityKind::Transfer));
        unrelated.ending = Some(Ending::Interrupted);
        for part in [&published, &unfinished, &unrelated] {
            super::super::transfer_records::put(&connection, part).unwrap();
        }
        UploadRequests::over(&connection).write_it_down(&request).unwrap();
        a_build_send_has_ended(&connection, &request.record).unwrap();
        let read_part = |part: &TransferRecord| super::super::transfer_records::read(&connection, &part.transfer).unwrap().unwrap();
        assert_eq!(read_part(&unfinished).ending, Some(Ending::Failed), "a deleted temporary source was left resumable");
        assert!(read_part(&unfinished).cleanup_is_owed);
        assert_eq!(read_part(&published), published);
        assert_eq!(read_part(&unrelated), unrelated);
        let read = read(&connection, &request.record).unwrap().unwrap();
        assert_eq!(read.entries[0].outcome, Some(UnitOutcome::Done));
        assert_eq!(read.entries[1].outcome, Some(UnitOutcome::Failed));
        assert_eq!(read.entries[2].outcome, None, "a skipped file changed");
        assert_eq!(read.entries[4].outcome, Some(UnitOutcome::Failed), "a file that never started was left waiting");
        a_build_send_has_ended(&connection, &request.record).unwrap();
        assert_eq!(self::read(&connection, &request.record).unwrap().unwrap(), read, "a second close changed a finished outcome");
    }

    /// **The test that would have caught the fault this store exists to end.**
    ///
    /// `File Browser` §3 promises an upload request survives a drop, the app
    /// closing and a fresh sitting; with no store behind `KeepsTheRequest` it
    /// survived none of them. **A fresh sitting is exactly this**: the record
    /// on disk, closed with the run that wrote it, opened again by a second
    /// run, and the request read back whole with its units in §9.4's order.
    #[test]
    fn a_request_survives_the_record_being_closed_and_opened_again() {
        let directory = std::env::temp_dir().join(format!(
            "smoozy-upload-request-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        std::fs::create_dir_all(&directory).unwrap();
        let path = directory.join("record.sqlite");

        let server_row = an_identity(IdentityKind::LocalServer);
        let first_run = an_identity(IdentityKind::Run);
        let written = {
            let connection = rusqlite::Connection::open(&path).unwrap();
            the_tables(&connection);
            let request = a_request(&server_row, &first_run);
            let mut keeper = UploadRequests::over(&connection);
            keeper.write_it_down(&request).unwrap();
            request
        };
        // The application run ends here. `File Browser` U7: the activity dies
        // with the process and only the record is left.

        let connection = rusqlite::Connection::open(&path).unwrap();
        let keeper = UploadRequests::over(&connection);
        let read = keeper.every_request_for(&server_row).unwrap();
        assert_eq!(read, vec![written.clone()], "the request came back whole");

        let read = &read[0];
        // §9.4's order, and every unit under §11's one role.
        assert_eq!(
            read.entries
                .iter()
                .map(|entry| entry.landing_path.clone())
                .collect::<Vec<_>>(),
            written
                .entries
                .iter()
                .map(|entry| entry.landing_path.clone())
                .collect::<Vec<_>>()
        );
        assert_eq!(read.the_units_by_role().len(), 4);
        assert!(read
            .the_units_by_role()
            .iter()
            .all(|role| *role == request::the_role_every_unit_carries()));
        // **`File Browser` U9a's moment comes back to the nanosecond**, which
        // is what U9b's reason line orders two earlier requests on.
        assert_eq!(
            read.entries[0].outcome_written_at,
            Some(SystemTime::UNIX_EPOCH + Duration::from_nanos(1_700_000_000_123_456_789))
        );
        // **`File Browser` U8's separation**: the record carries which run
        // wrote it, so a second run reads it as an earlier run's.
        let second_run = an_identity(IdentityKind::Run);
        assert!(read.ended_by_the_run_ending(&second_run));
        assert!(!read.ended_by_the_run_ending(&first_run));

        drop(connection);
        std::fs::remove_dir_all(&directory).unwrap();
    }

    /// **The join is by position and never by role**, which is what `File
    /// Browser` §11 asks for: two files of the same name in different folders
    /// carry the same role and are told apart by where they sit.
    #[test]
    fn two_units_of_one_name_come_back_at_their_own_positions() {
        let connection = a_record();
        let server_row = an_identity(IdentityKind::LocalServer);
        let mut entries = vec![
            EnumerationEntry::enumerated(b"/one/scene.blend".to_vec(), b"/m/one/scene.blend".to_vec(), 1),
            EnumerationEntry::enumerated(b"/two/scene.blend".to_vec(), b"/m/two/scene.blend".to_vec(), 2),
        ];
        entries[0].disposition = Some(Disposition::ToSend);
        entries[1].disposition = Some(Disposition::ToSend);
        entries[1]
            .its_outcome_became(UnitOutcome::Failed, SystemTime::UNIX_EPOCH)
            .unwrap();
        let request = UploadRequest::confirmed(
            request::open_an_upload_request().unwrap(),
            an_identity(THE_RECORD_IDENTITY_KIND),
            server_row.clone(),
            None,
            b"/m".to_vec(),
            entries,
            an_identity(IdentityKind::Run),
        );

        let mut keeper = UploadRequests::over(&connection);
        keeper.write_it_down(&request).unwrap();
        let read = keeper.every_request_for(&server_row).unwrap();
        assert_eq!(read[0].entries[0].outcome, None);
        assert_eq!(read[0].entries[1].outcome, Some(UnitOutcome::Failed));
    }

    /// **`State, identity and recovery` F19, at that document's §8.6: a unit
    /// outcome is immutable**,
    /// and this store keeps it whatever a caller hands down.
    #[test]
    fn an_outcome_the_store_already_holds_is_never_written_over() {
        let connection = a_record();
        let server_row = an_identity(IdentityKind::LocalServer);
        let mut entries = vec![EnumerationEntry::enumerated(b"/a".to_vec(), b"/m/a".to_vec(), 1)];
        entries[0].disposition = Some(Disposition::ToSend);
        entries[0]
            .its_outcome_became(UnitOutcome::Done, SystemTime::UNIX_EPOCH)
            .unwrap();
        let mut request = UploadRequest::confirmed(
            request::open_an_upload_request().unwrap(),
            an_identity(THE_RECORD_IDENTITY_KIND),
            server_row.clone(),
            None,
            b"/m".to_vec(),
            entries,
            an_identity(IdentityKind::Run),
        );
        let mut keeper = UploadRequests::over(&connection);
        keeper.write_it_down(&request).unwrap();

        // A caller that has lost the first ending and writes a second one.
        request.entries[0].outcome = Some(UnitOutcome::Failed);
        keeper.write_it_down(&request).unwrap();

        let read = keeper.every_request_for(&server_row).unwrap();
        assert_eq!(
            read[0].entries[0].outcome,
            Some(UnitOutcome::Done),
            "the value that stands is the first one written \
             (`State, identity and recovery` F19, at that document's §8.6)"
        );
    }

    /// **`File Browser` U1: a request that disturbed nothing carries neither
    /// field**, and the answer that has not been given yet is a real state
    /// rather than a missing value (U4).
    #[test]
    fn a_request_that_disturbed_nothing_and_was_never_answered_reads_back_as_both_absences() {
        let connection = a_record();
        let server_row = an_identity(IdentityKind::LocalServer);
        let request = UploadRequest::confirmed(
            request::open_an_upload_request().unwrap(),
            an_identity(THE_RECORD_IDENTITY_KIND),
            server_row.clone(),
            None,
            b"/m".to_vec(),
            vec![EnumerationEntry::enumerated(
                b"/a".to_vec(),
                b"/m/a".to_vec(),
                1,
            )],
            an_identity(IdentityKind::Run),
        );
        let mut keeper = UploadRequests::over(&connection);
        keeper.write_it_down(&request).unwrap();

        let read = keeper.every_request_for(&server_row).unwrap();
        assert_eq!(read[0].telling, None);
        assert_eq!(read[0].collision_answer, None);
        assert_eq!(read[0].entries[0].disposition, None);
        assert_eq!(read[0].installation, None);
        let rows: i64 = connection
            .query_row("SELECT count(*) FROM upload_request_telling_job", [], |row| {
                row.get(0)
            })
            .unwrap();
        assert_eq!(rows, 0, "no row is the whole of no telling");
    }

    /// **`State, identity and recovery` §11.1: user-held**, and both halves of
    /// the one record go together.
    #[test]
    fn the_user_removing_it_takes_the_ninth_versions_rows_and_the_shared_ones_with_it() {
        let connection = a_record();
        let server_row = an_identity(IdentityKind::LocalServer);
        let request = a_request(&server_row, &an_identity(IdentityKind::Run));
        let record = request.record.clone();
        let mut keeper = UploadRequests::over(&connection);
        keeper.write_it_down(&request).unwrap();
        assert!(read(&connection, &record).unwrap().is_some());

        keeper.the_user_removed_it(&record).unwrap();
        assert!(read(&connection, &record).unwrap().is_none());
        for table in [
            "upload_request",
            "upload_request_entry",
            "upload_request_telling_job",
            "outcome_record",
            "outcome_record_unit",
        ] {
            let left: i64 = connection
                .query_row(&format!("SELECT count(*) FROM {table}"), [], |row| row.get(0))
                .unwrap();
            assert_eq!(left, 0, "{table} still holds a row");
        }
    }

    /// **A record belonging to another server row is not this server row's**,
    /// which is what `File Browser` U8's fresh sitting and U9b's candidates
    /// both read under.
    #[test]
    fn a_request_under_another_server_row_is_not_returned() {
        let connection = a_record();
        let mine = an_identity(IdentityKind::LocalServer);
        let theirs = an_identity(IdentityKind::LocalServer);
        let run = an_identity(IdentityKind::Run);
        let mut keeper = UploadRequests::over(&connection);
        keeper.write_it_down(&a_request(&mine, &run)).unwrap();
        keeper.write_it_down(&a_request(&theirs, &run)).unwrap();

        assert_eq!(keeper.every_request_for(&mine).unwrap().len(), 1);
        assert_eq!(keeper.every_request_for(&theirs).unwrap().len(), 1);
    }

    /// One request, one unit, published at a landing path from a source path.
    fn a_published_request(
        server_row: &Identity,
        record: Identity,
        source_path: &[u8],
        landing_path: &[u8],
        installation: &Identity,
    ) -> UploadRequest {
        let mut entries = vec![EnumerationEntry::enumerated(
            source_path.to_vec(),
            landing_path.to_vec(),
            1,
        )];
        entries[0].disposition = Some(Disposition::ToSend);
        entries[0].transfer = Some(an_identity(IdentityKind::Transfer));
        entries[0]
            .its_outcome_became(UnitOutcome::Done, SystemTime::UNIX_EPOCH)
            .unwrap();
        UploadRequest::confirmed(
            request::open_an_upload_request().unwrap(),
            record,
            server_row.clone(),
            Some(installation.clone()),
            b"/m".to_vec(),
            entries,
            an_identity(IdentityKind::Run),
        )
    }

    /// Two record identities where the **second** one issued sorts **before**
    /// the first as text. Issued until the pair has that shape, which takes a
    /// couple of tries and never long: the values are random.
    fn two_records_whose_identities_sort_the_wrong_way() -> (Identity, Identity) {
        loop {
            let first = an_identity(THE_RECORD_IDENTITY_KIND);
            let second = an_identity(THE_RECORD_IDENTITY_KIND);
            if second.as_text() < first.as_text() {
                return (first, second);
            }
        }
    }

    /// **`File Browser` N6's recency is the local record's own write order, and
    /// this store is what supplies it.**
    ///
    /// The statement used to end `ORDER BY record`, and a record is `State,
    /// identity and recovery` §2.1's identity — sixteen random bytes — whose
    /// rule 2 forbids sorting by one in as many words. So the answer to *where
    /// did this file come from* was whichever matching request's identity
    /// happened to sort last.
    #[test]
    fn the_later_write_is_the_answer_even_where_its_identity_sorts_first() {
        let connection = a_record();
        let server_row = an_identity(IdentityKind::LocalServer);
        let installation = an_identity(IdentityKind::Installation);
        let (first, second) = two_records_whose_identities_sort_the_wrong_way();

        let mut keeper = UploadRequests::over(&connection);
        keeper
            .write_it_down(&a_published_request(
                &server_row,
                first,
                b"/home/older.png",
                b"/m/landing.png",
                &installation,
            ))
            .unwrap();
        keeper
            .write_it_down(&a_published_request(
                &server_row,
                second,
                b"/home/newer.png",
                b"/m/landing.png",
                &installation,
            ))
            .unwrap();

        let requests = keeper.every_request_for(&server_row).unwrap();
        assert_eq!(
            smoozy_file_browser::uploading::where_it_came_from(
                &requests,
                &server_row,
                b"/m/landing.png",
                Some(&installation),
            ),
            smoozy_file_browser::uploading::WhereItCameFrom::TheSourcePath(
                b"/home/newer.png".to_vec()
            ),
            "the most recently written request is the answer, whatever its identity sorts as"
        );
    }

    /// **A later write of an earlier request leaves the order alone.**
    ///
    /// This is the half the row identifier depends on: a replace would delete
    /// the row and insert a new one, so the first request would move to the end
    /// of the write order and answer for a file it did not send.
    #[test]
    fn writing_a_request_again_does_not_move_it_down_the_write_order() {
        let connection = a_record();
        let server_row = an_identity(IdentityKind::LocalServer);
        let installation = an_identity(IdentityKind::Installation);
        let (first, second) = two_records_whose_identities_sort_the_wrong_way();

        let mut keeper = UploadRequests::over(&connection);
        let mut older = a_published_request(
            &server_row,
            first,
            b"/home/older.png",
            b"/m/landing.png",
            &installation,
        );
        keeper.write_it_down(&older).unwrap();
        keeper
            .write_it_down(&a_published_request(
                &server_row,
                second,
                b"/home/newer.png",
                b"/m/landing.png",
                &installation,
            ))
            .unwrap();

        // The older request is written again — the ordinary case, since every
        // change to a request goes down through this same call.
        older.collision_answer = Some(CollisionAnswer::Replace);
        keeper.write_it_down(&older).unwrap();

        let requests = keeper.every_request_for(&server_row).unwrap();
        assert_eq!(
            requests
                .iter()
                .map(|request| request.entries[0].source_path.clone())
                .collect::<Vec<_>>(),
            vec![b"/home/older.png".to_vec(), b"/home/newer.png".to_vec()],
            "the order is where each request was first written down"
        );
        assert_eq!(
            smoozy_file_browser::uploading::where_it_came_from(
                &requests,
                &server_row,
                b"/m/landing.png",
                Some(&installation),
            ),
            smoozy_file_browser::uploading::WhereItCameFrom::TheSourcePath(
                b"/home/newer.png".to_vec()
            )
        );
        // And the write did land: the row was updated in place, not skipped.
        assert_eq!(requests[0].collision_answer, Some(CollisionAnswer::Replace));
    }

    /// **A thousand-entry request whose second save writes one entry** —
    /// `src/uploading.rs`'s save at a file's arrangement and at its ending.
    ///
    /// **The case this bites on is the whole write it replaced**: that wrote
    /// all thousand entries, three statements each, at every file.
    #[test]
    fn a_thousand_entry_request_s_second_save_writes_one_entry() {
        let connection = a_record();
        let server_row = an_identity(IdentityKind::LocalServer);
        let run = an_identity(IdentityKind::Run);
        let entries: Vec<EnumerationEntry> = (0..1_000)
            .map(|at| {
                let mut entry = EnumerationEntry::enumerated(
                    format!("/home/f/{at:04}.png").into_bytes(),
                    format!("/machine/f/{at:04}.png").into_bytes(),
                    1_000 + at,
                );
                entry.disposition = Some(Disposition::ToSend);
                entry
            })
            .collect();
        let mut request = UploadRequest::confirmed(
            request::open_an_upload_request().expect("open"),
            an_identity(THE_RECORD_IDENTITY_KIND),
            server_row.clone(),
            None,
            b"/machine/f".to_vec(),
            entries,
            run,
        );
        let mut keeper = UploadRequests::over(&connection);
        keeper.write_it_down(&request).expect("the first write is the whole request");

        // **The file's transfer is arranged**, then **it ends**.
        request.entries[417].transfer = Some(an_identity(IdentityKind::Transfer));
        let before = connection.total_changes();
        keeper.one_entry_moved(&request, 417).expect("written");
        let arranged = connection.total_changes() - before;
        request.entries[417]
            .its_outcome_became(UnitOutcome::Done, SystemTime::now())
            .unwrap();
        let before = connection.total_changes();
        keeper.one_entry_moved(&request, 417).expect("written");
        let ended = connection.total_changes() - before;
        assert!(
            arranged <= 3 && ended <= 4,
            "a save of one moved entry changed {arranged} and {ended} rows"
        );

        let back = keeper.every_request_for(&server_row).unwrap();
        assert_eq!(back.len(), 1);
        assert_eq!(back[0].entries, request.entries);

        // **An outcome that stands is never overwritten** (`State, identity
        // and recovery` F19), by the one-entry write as by the whole one.
        let mut later = request.clone();
        later.entries[417].outcome = Some(UnitOutcome::Failed);
        keeper.one_entry_moved(&later, 417).expect("written");
        assert_eq!(
            keeper.every_request_for(&server_row).unwrap()[0].entries[417].outcome,
            Some(UnitOutcome::Done)
        );
        assert!(keeper.one_entry_moved(&request, 1_000).is_err());
    }

    /// **The counts the store answers are the counts the whole reading
    /// composes**, over every disposition and every outcome, and a record the
    /// whole reading skips is skipped here too.
    #[test]
    fn the_counts_the_store_answers_are_what_the_whole_reading_composes() {
        let connection = a_record();
        let server_row = an_identity(IdentityKind::LocalServer);
        let run = an_identity(IdentityKind::Run);
        let mut keeper = UploadRequests::over(&connection);
        let request = a_request(&server_row, &run);
        keeper.write_it_down(&request).unwrap();
        let mut failed = a_request(&server_row, &run);
        failed.entries[1]
            .its_outcome_became(UnitOutcome::Failed, SystemTime::now())
            .unwrap();
        keeper.write_it_down(&failed).unwrap();
        // **Half a record**: a unit with no entry, which the whole reading
        // refuses to read back.
        let half = a_request(&server_row, &run);
        keeper.write_it_down(&half).unwrap();
        connection
            .execute(
                "DELETE FROM outcome_record_unit WHERE record = ?1 AND position = 2",
                [half.record.as_text()],
            )
            .unwrap();

        let whole = keeper.every_request_for(&server_row).unwrap();
        let counted = keeper.every_request_counted_for(&server_row).unwrap();
        assert_eq!(whole.len(), 2, "the half record is not read back whole");
        assert_eq!(counted.len(), whole.len());
        for (one, counted) in whole.iter().zip(&counted) {
            let sending: Vec<&EnumerationEntry> = one
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
            assert_eq!(counted.record, one.record);
            assert_eq!(counted.destination, one.destination);
            assert_eq!(counted.files_in_the_whole, sending.len());
            assert_eq!(counted.files_done, sending.iter().filter(done).count());
            assert_eq!(
                counted.bytes_in_the_whole,
                sending.iter().map(|entry| entry.size_as_enumerated).sum::<u64>()
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
                counted.failed_landings,
                sending
                    .iter()
                    .filter(|entry| entry.outcome == Some(UnitOutcome::Failed))
                    .map(|entry| (entry.landing_path.clone(), entry.size_as_enumerated))
                    .collect::<Vec<_>>()
            );
        }
        assert_eq!(counted[1].failed_landings, vec![(vec![b'/', 0xfe, b'b'], 22)]);
        assert!(counted.iter().all(|one| !one.nothing_is_decided && !one.stopped_at_the_question));
    }

    /// **A failed file whose transfer stopped because it changed while it went
    /// up is counted among the changed ones** (`File Browser` K8, the record's
    /// own *stopped: the source changed*), and a failure of any other kind is
    /// not — the 38's piece 9, item 17, which draws the first with *Upload it
    /// again*.
    #[test]
    fn a_file_that_changed_while_it_went_up_is_counted_as_changed() {
        use smoozy_server_protocol::transfers::{Direction, Ending, StoppedReason, TransferRecord};
        let connection = a_record();
        let server_row = an_identity(IdentityKind::LocalServer);
        let run = an_identity(IdentityKind::Run);
        let mut keeper = UploadRequests::over(&connection);
        let mut request = a_request(&server_row, &run);
        request.entries[1]
            .its_outcome_became(UnitOutcome::Failed, SystemTime::now())
            .unwrap();
        keeper.write_it_down(&request).unwrap();
        let a_record_ending = |transfer: &Identity, ending: Ending| {
            let mut record = TransferRecord::arranged(
                transfer.clone(),
                Direction::Up,
                server_row.clone(),
                None,
                "/home/b".to_string(),
                "/machine/b (2)".to_string(),
                "/machine/.smoozy-staging",
                22,
                run.clone(),
            );
            record.ending = Some(ending);
            crate::record::transfer_records::put(&connection, &record).unwrap();
        };
        let transfer = request.entries[1].transfer.clone().expect("a transfer");
        a_record_ending(&transfer, Ending::Failed);
        let counted = keeper.every_request_counted_for(&server_row).unwrap();
        assert_eq!(counted[0].failed_landings.len(), 1);
        assert!(counted[0].changed_landings.is_empty(), "a failure that was not a change");

        a_record_ending(&transfer, Ending::Stopped(StoppedReason::TheSourceChangedUnderIt));
        let counted = keeper.every_request_counted_for(&server_row).unwrap();
        assert_eq!(counted[0].changed_landings, vec![vec![b'/', 0xfe, b'b']]);
    }

    /// **A request nothing has decided, and one stopped at `File Browser` U4's
    /// question, are counted over what they asked for and the first has not
    /// ended by its units** — the download's rule
    /// (`crate::record::download_records`), kept for an upload. Before it,
    /// both counted *0 of 0, ended*, which the screen read as *done*: the
    /// Hyperstack check before 0.1.7 saw *0 files into /home/ubuntu · done*
    /// for a file whose walk was lent nothing. Step 10c's check-before-0.1.7
    /// fix piece 1.
    #[test]
    fn a_request_that_sent_nothing_is_counted_over_what_it_asked_for() {
        let connection = a_record();
        let server_row = an_identity(IdentityKind::LocalServer);
        let run = an_identity(IdentityKind::Run);
        let mut keeper = UploadRequests::over(&connection);
        let asked = |keeper: &mut UploadRequests<'_>| {
            let request = UploadRequest::confirmed(
                request::open_an_upload_request().expect("open"),
                an_identity(THE_RECORD_IDENTITY_KIND),
                server_row.clone(),
                None,
                b"/home/ubuntu".to_vec(),
                vec![
                    EnumerationEntry::enumerated(b"/a".to_vec(), b"/home/ubuntu/a".to_vec(), 1_000),
                    EnumerationEntry::enumerated(b"/b".to_vec(), b"/home/ubuntu/b".to_vec(), 24),
                ],
                run.clone(),
            );
            keeper.write_it_down(&request).unwrap();
            request
        };
        let undecided = asked(&mut keeper);
        let mut stopped = asked(&mut keeper);
        stopped.collision_answer = Some(CollisionAnswer::Stop);
        keeper.write_it_down(&stopped).unwrap();

        let counted = keeper.every_request_counted_for(&server_row).unwrap();
        assert_eq!(counted.len(), 2);
        for one in &counted {
            assert_eq!((one.files_in_the_whole, one.files_done), (2, 0));
            assert_eq!((one.bytes_in_the_whole, one.bytes_done), (1_024, 0));
            assert_eq!(one.opened_by_run, run);
        }
        assert_eq!(counted[0].record, undecided.record);
        assert!(counted[0].nothing_is_decided);
        assert!(!counted[0].every_unit_has_ended, "a request nothing has decided read ended");
        assert!(!counted[0].stopped_at_the_question);
        assert_eq!(counted[1].record, stopped.record);
        assert!(counted[1].stopped_at_the_question);
        assert!(!counted[1].nothing_is_decided);
    }
}
