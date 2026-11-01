//! The per-transfer records in the local record — `Desktop app and delivery`
//! §5 and `Server protocol and transfers` §7.3.
//!
//! **The record's shape is that document's and the store is this one's**, which
//! is the split `Keys`' rows and `Servers`' rows already use.
//!
//! **Markers used below that belong to another document, each with the document
//! it belongs to.** `Server protocol and transfers` S15, S28, S31;
//! `State, identity and recovery` §2.1; `File Browser` N6.
//!
//! **A marker this crate's own plan also defines is deliberately not listed
//! above, whichever document is meant, and is written out with its document
//! at every site instead** — the header list cannot tell the two apart, and a
//! bare one would resolve to a real and wrong clause.
//!
//! **This crate's own plan defines an S4, an S7, an S8, an S9, a C11 and a §2.1
//! of its own** — every store call running off the interface thread, the
//! askpass helper, the three plaintext hops, the post-update keychain dialog on
//! macOS, what makes a judgement independent of every folding rule, and the
//! figures section — so none of those six is listed above. `File Browser` N6
//! collides with nothing here and is written out all the same.
//!
//! **`Server protocol and transfers` S7: what a transfer has staged, and how
//! far, is recorded on the user's computer — for an upload as well as for a
//! download.** The reason is observed: **on a full disk nothing further can be
//! written**, every write
//! refused with `No space left on device`, *"So a marker written on the machine
//! is exactly what cannot be written in the case it is needed for"*. A record
//! on the machine would also be destroyed by a recycle, silently and on the
//! same address.
//!
//! **`Server protocol and transfers` S8: the record is durable before the
//! first byte is written.** Row 197's third clause is why — an upload cut off
//! by *"the app closing"* keeps its
//! leftover and the user is told it is there — and *a record written at the end
//! of a transfer is a record a closing process never writes.*
//!
//! **Its lifetime is `State, identity and recovery` §11.1's**, nested under the
//! transfer arrangement or multi-part action outcome that owns it. **Nothing
//! here decides that**, and nothing here sweeps.

use rusqlite::OptionalExtension;
use smoozy_server_protocol::transfers::integrity::Digest;
use smoozy_server_protocol::transfers::publication::{Intent, PublicationPreState};
use smoozy_server_protocol::transfers::{
    Checkpoint, Direction, Ending, SourceReading, StableFileIdentity, StagingPath, StoppedReason,
    SupersededAttempt, TransferRecord,
};
use smoozy_state::identities::{Identity, IdentityKind};

use super::RecordFailure;

/// The columns the record's own table carries, named once so the read and the
/// write cannot drift apart.
const EVERY_COLUMN: &str = "transfer, attempt, direction, server, installation,
     source_path, final_path, staging_directory, total_size_expected, expected_digest,
     written_by_run, staging_identity, source_size, source_inode,
     source_modification_time, source_change_time,
     publication_intent, publication_staging_path, publication_final_path,
     publication_staged_identity, publication_byte_length, publication_digest,
     ending, stopped_reason, cleanup_is_owed";

/// **`Server protocol and transfers` S9's record, put down in one save that
/// writes what changed.** *"It holds no copy of any byte and no observed value
/// the machine still holds."*
///
/// **The case this was first written for, and the case it now also meets.**
/// It was written for a record of a few marks, and it wrote the record whole
/// every time — the row, then every superseded attempt and every checkpoint
/// deleted and inserted again, **one statement at a time and each its own
/// save**. At the record's own settings (`State, identity and recovery` §1.3
/// rule 6: `synchronous = EXTRA`, a rollback journal, and the full flush on a
/// Mac) one save costs about 16 ms at the desk whether it holds one row or a
/// thousand. `Server protocol and transfers` S8 puts the record down at every
/// checkpoint, and a checkpoint is every 4 MiB, so **the *n*th checkpoint of a
/// transfer cost *n* + 3 saves**: a 4,000,000,000-byte file has 954, which is
/// about 458,000 saves and two hours at the desk — the tester measured
/// 7,206 s. **It now meets 954 marks**, and the save is one save whatever the
/// record holds, and the *n*th checkpoint writes one row.
///
/// **The moment the record and the world parted company** was the comment
/// that said *written whole*: it was true of the result and said nothing of
/// the cost, and the cost grew with the square of the file's size while every
/// test, all of them a few marks long, stayed fast.
///
/// **What is written, in one transaction**:
///
/// - **the record's own row**, as an upsert that keeps it (below), and only
///   where it differs from the stored row — so a save in which nothing moved
///   writes nothing and costs a reading, not a flush;
/// - **the superseded attempts** only where they differ from the stored ones —
///   a start-over adds one, and nothing else moves them;
/// - **the checkpoints above the highest one stored** where the stored ones
///   are the start of the record's list, which is every save the byte loop
///   makes between two start-overs; **and all of them again only where they
///   are not** — `Server protocol and transfers` S12 step 5's start-over empties
///   the list, and step 1's *nothing staged* clears it, so the stored marks are
///   replaced by what the record now says rather than kept beside it.
///
/// **The three settings are untouched**, and that is the point of doing it this
/// way round: every save is still durable exactly as rule 6 asks, and there are
/// simply far fewer of them. **The statements are prepared once per connection
/// and kept** (the engine's own statement cache), so a save costs the same work
/// at the thousandth checkpoint as at the first.
///
/// **A savepoint rather than `BEGIN`**, so the one save is a transaction of
/// its own when nothing else holds one and nests where a caller already does.
///
/// **An upsert that keeps the row, and it is the same repair the neighbouring
/// upload-request table took.** A replace deletes the row and inserts a new
/// one, so it hands the row a new row identifier — and [`all`] below reads this
/// table `ORDER BY rowid`. `Server protocol and transfers` S8 puts this record
/// down before the first byte **and again at every checkpoint**, so under a
/// replace a transfer's place in that order was not where it was first written
/// down but where it last checkpointed, and the busiest transfer was always
/// last.
///
/// **Nothing today reads this table for write order**, and that is said here
/// rather than left to be worked out: the reading `Server protocol and
/// transfers` S28's desktop-side sweep takes ([`the_downloads_the_start_settles`])
/// begins from every
/// record in a removable state and derives each one's staging paths — a
/// judgement per record, in no order — and `File Browser` N6's write-order
/// recency is that document's own enumeration entries and says in as many
/// words that it "is not read across" record kinds. **The pairing is made
/// consistent anyway**, because two tables a file apart answering *what does
/// `ORDER BY rowid` mean here* differently is a thing a later reader has to
/// discover, and the one that reads write order for real — `upload_request`,
/// under `File Browser` N6 — is the one they would take the answer from.
pub fn put(
    connection: &rusqlite::Connection,
    record: &TransferRecord,
) -> Result<(), RecordFailure> {
    connection.execute_batch("SAVEPOINT a_transfer_record_is_put")?;
    let written = what_changed_is_written(connection, record)
        .and_then(|()| Ok(connection.execute_batch("RELEASE a_transfer_record_is_put")?));
    if written.is_err() {
        // **Nothing of a save that failed half-way stays**, and the savepoint
        // is let go of either way so the connection is not left inside it.
        let _ = connection.execute_batch(
            "ROLLBACK TO a_transfer_record_is_put; RELEASE a_transfer_record_is_put",
        );
    }
    written
}

/// **The record's own row, in [`EVERY_COLUMN`]'s order, as the store writes
/// it** — so a stored row can be compared with it before anything is written.
fn the_row_as_written(record: &TransferRecord) -> Vec<rusqlite::types::Value> {
    use rusqlite::types::Value;
    let text = |held: Option<String>| held.map(Value::Text).unwrap_or(Value::Null);
    let number = |held: Option<i64>| held.map(Value::Integer).unwrap_or(Value::Null);
    vec![
        Value::Text(record.transfer.as_text()),
        Value::Integer(i64::from(record.attempt)),
        Value::Text(
            match record.direction {
                Direction::Up => "up",
                Direction::Down => "down",
            }
            .to_string(),
        ),
        Value::Text(record.server.as_text()),
        text(record.installation.as_ref().map(|identity| identity.as_text())),
        Value::Text(record.source_path.clone()),
        Value::Text(record.final_path.clone()),
        Value::Text(record.staging_path.directory().to_string()),
        Value::Integer(record.total_size_expected as i64),
        text(record.expected_digest.as_ref().map(|digest| digest.as_text())),
        Value::Text(record.written_by_run.as_text()),
        text(record.staging_identity.as_ref().map(written_identity)),
        number(record.source_reading.as_ref().map(|read| read.size as i64)),
        text(record.source_reading.as_ref().map(|read| read.inode.clone())),
        number(record.source_reading.as_ref().map(|read| read.modification_time)),
        number(record.source_reading.as_ref().map(|read| read.change_time)),
        text(record.publication_pending.as_ref().map(|pending| {
            match pending.intent {
                Intent::ReplaceIntended => "replace",
                Intent::CreateIntended => "create",
            }
            .to_string()
        })),
        text(
            record
                .publication_pending
                .as_ref()
                .map(|pending| pending.staging_path.clone()),
        ),
        text(
            record
                .publication_pending
                .as_ref()
                .map(|pending| pending.final_path.clone()),
        ),
        text(
            record
                .publication_pending
                .as_ref()
                .map(|pending| written_identity(&pending.staged_identity)),
        ),
        number(
            record
                .publication_pending
                .as_ref()
                .map(|pending| pending.staged_byte_length as i64),
        ),
        text(
            record
                .publication_pending
                .as_ref()
                .map(|pending| pending.staged_digest.as_text()),
        ),
        text(record.ending.as_ref().map(|ending| written_ending(ending).to_string())),
        text(
            record
                .ending
                .as_ref()
                .and_then(written_reason)
                .map(str::to_string),
        ),
        Value::Integer(i64::from(record.cleanup_is_owed)),
    ]
}

/// **The upsert, composed once.** The assignments are derived from
/// [`EVERY_COLUMN`] rather than written out, for that constant's own reason: a
/// column added to the list is written on a conflict without anybody having to
/// remember a second place. The conflict key is set to itself, which is what
/// it already equals.
fn the_upsert() -> &'static str {
    static COMPOSED: std::sync::OnceLock<String> = std::sync::OnceLock::new();
    COMPOSED.get_or_init(|| {
        let placeholders = (1..=25)
            .map(|at| format!("?{at}"))
            .collect::<Vec<_>>()
            .join(", ");
        let assignments = EVERY_COLUMN
            .split(',')
            .map(|column| column.trim())
            .map(|column| format!("{column} = excluded.{column}"))
            .collect::<Vec<_>>()
            .join(", ");
        format!(
            "INSERT INTO transfer_record ({EVERY_COLUMN}) VALUES ({placeholders})
             ON CONFLICT(transfer) DO UPDATE SET {assignments}"
        )
    })
}

fn what_changed_is_written(
    connection: &rusqlite::Connection,
    record: &TransferRecord,
) -> Result<(), RecordFailure> {
    let transfer = record.transfer.as_text();
    // **The record's own row, written only where it differs from the stored
    // one.** The loop puts a record down again where nothing in it moved — the
    // write after a removal that went through repeats the ending's own — and a
    // save that changes nothing is a flush of the disk for nothing. *(This
    // named a fresh transfer's empty list cleared before its first byte, under
    // `Server protocol and transfers` S12 step 1; the loop stopped writing that
    // one on 2026-09-28, step 10c's transfer wave piece 2.)* **A save in
    // which nothing at all changed writes nothing**, so it costs a reading.
    let the_row = the_row_as_written(record);
    let stored: Option<Vec<rusqlite::types::Value>> = connection
        .prepare_cached(&format!(
            "SELECT {EVERY_COLUMN} FROM transfer_record WHERE transfer = ?1"
        ))?
        .query_row([&transfer], |row| {
            (0..25)
                .map(|at| row.get::<_, rusqlite::types::Value>(at))
                .collect::<Result<Vec<_>, _>>()
        })
        .optional()?;
    if stored.as_ref() != Some(&the_row) {
        connection
            .prepare_cached(the_upsert())?
            .execute(rusqlite::params_from_iter(the_row.iter()))?;
    }

    // **`Server protocol and transfers` S4's superseded attempt numbers,
    // retained so the sweep can derive their paths, each with the stable file
    // identity `Server protocol and transfers` S9 records beside it.** They are
    // rows of their own rather than a packed field, which is the same rule
    // `Servers` D6 keeps for the hardware reading: a packed value is reachable
    // only by the code that packed it.
    //
    // **The identity is written with the number and never as a second table.**
    // `Server protocol and transfers` S28 removes a derived path only where the
    // path, the synthesised name and the identity all match, so a number stored
    // apart from its identity is a number the sweep cannot act on.
    //
    // **Written only where they differ from what is stored.** The list is a
    // handful long and moves only at a start-over, so it is compared whole and,
    // where it moved, written whole.
    let written: Vec<(u32, Option<String>)> = record
        .superseded_attempts
        .iter()
        .map(|superseded| {
            (
                superseded.attempt,
                superseded.staging_identity.as_ref().map(written_identity),
            )
        })
        .collect();
    let stored: Vec<(u32, Option<String>)> = connection
        .prepare_cached(
            "SELECT attempt, staging_identity FROM transfer_superseded_attempt
             WHERE transfer = ?1 ORDER BY attempt",
        )?
        .query_map([&transfer], |row| Ok((row.get(0)?, row.get(1)?)))?
        .collect::<Result<_, _>>()?;
    let mut in_order = written.clone();
    in_order.sort_by_key(|(attempt, _)| *attempt);
    if stored != in_order {
        connection
            .prepare_cached("DELETE FROM transfer_superseded_attempt WHERE transfer = ?1")?
            .execute([&transfer])?;
        let mut insert = connection.prepare_cached(
            "INSERT INTO transfer_superseded_attempt (transfer, attempt, staging_identity)
             VALUES (?1, ?2, ?3)",
        )?;
        for (attempt, identity) in &written {
            insert.execute(rusqlite::params![transfer, attempt, identity])?;
        }
    }

    // **The checkpointed high-water offsets, each with the running digest at
    // it.** A checkpoint is a byte offset into the staged file and its digest
    // covers that file's own bytes (`Server protocol and transfers` S31), so
    // the two travel together.
    //
    // **Only the marks the stored copy lacks.** The stored marks are read in
    // offset order and compared, one by one, with the start of the record's
    // own list; where every stored mark is the list's own at the same place,
    // the rest of the list is what is new and it alone is inserted. **Where
    // one is not** — a start-over emptied the list, or it was cleared because
    // nothing was staged — the stored marks are the old attempt's, and they
    // are replaced whole by what the record now says.
    let new: Vec<(i64, String)> = record
        .checkpoints
        .iter()
        .map(|checkpoint| {
            (
                checkpoint.offset as i64,
                checkpoint.digest_of_everything_up_to_it.as_text(),
            )
        })
        .collect();
    let mut the_stored_are_its_start = true;
    let mut how_many_are_stored = 0usize;
    {
        let mut reading = connection.prepare_cached(
            "SELECT offset_into_the_staged_file, digest FROM transfer_checkpoint
             WHERE transfer = ?1 ORDER BY offset_into_the_staged_file",
        )?;
        let mut rows = reading.query([&transfer])?;
        while let Some(row) = rows.next()? {
            let offset: i64 = row.get(0)?;
            let digest: String = row.get(1)?;
            match new.get(how_many_are_stored) {
                Some((at, said)) if *at == offset && *said == digest => {}
                _ => the_stored_are_its_start = false,
            }
            how_many_are_stored += 1;
            if !the_stored_are_its_start {
                break;
            }
        }
    }
    let from = if the_stored_are_its_start {
        how_many_are_stored
    } else {
        connection
            .prepare_cached("DELETE FROM transfer_checkpoint WHERE transfer = ?1")?
            .execute([&transfer])?;
        0
    };
    if from < new.len() {
        let mut insert = connection.prepare_cached(
            "INSERT INTO transfer_checkpoint (transfer, offset_into_the_staged_file, digest)
             VALUES (?1, ?2, ?3)",
        )?;
        for (offset, digest) in &new[from..] {
            insert.execute(rusqlite::params![transfer, offset, digest])?;
        }
    }
    Ok(())
}

/// One record, by its transfer identity.
pub fn read(
    connection: &rusqlite::Connection,
    transfer: &Identity,
) -> Result<Option<TransferRecord>, RecordFailure> {
    let row: Option<Vec<rusqlite::types::Value>> = connection
        .query_row(
            &format!("SELECT {EVERY_COLUMN} FROM transfer_record WHERE transfer = ?1"),
            [transfer.as_text()],
            |row| {
                let mut values = Vec::new();
                for at in 0..25 {
                    values.push(row.get::<_, rusqlite::types::Value>(at)?);
                }
                Ok(values)
            },
        )
        .optional()?;
    let Some(values) = row else { return Ok(None) };
    compose(connection, &values)
}

/// Every record. *(This was written for the desktop-side sweep of `Server
/// protocol and transfers` S28 at application start, which had no caller
/// until 2026-09-28; the sweep, once called, reads the narrower
/// [`the_downloads_the_start_settles`] instead.)*
///
/// **The order is the local record's own write order, oldest first, and
/// [`put`] is what supplies it** — it keeps the row on a rewrite rather than
/// replacing it, so a record's place here is the moment it was first written
/// down and not the moment it last checkpointed. **It is not `transfer`**: that
/// is `State, identity and recovery` §2.1's identity, whose rule 2 says *"Nothing
/// parses one, sorts by one, or infers anything from two being adjacent."*
///
/// **S28's sweep does not depend on the order** — it judges each record on its
/// own ending and its own recorded identity — so what this ordering buys is a
/// deterministic answer rather than a promise, and no clause anywhere rests on
/// it. It is stated so that a later reader does not take the row identifier to
/// mean something different here from what it means one table away.
pub fn all(connection: &rusqlite::Connection) -> Result<Vec<TransferRecord>, RecordFailure> {
    let mut statement = connection.prepare(&format!(
        "SELECT {EVERY_COLUMN} FROM transfer_record ORDER BY rowid"
    ))?;
    let rows = statement.query_map([], |row| {
        let mut values = Vec::new();
        for at in 0..25 {
            values.push(row.get::<_, rusqlite::types::Value>(at)?);
        }
        Ok(values)
    })?;
    let mut composed = Vec::new();
    for row in rows {
        if let Some(record) = compose(connection, &row?)? {
            composed.push(record);
        }
    }
    Ok(composed)
}

/// **One server's records that `Download`'s admission asks about — those not
/// yet ended and those holding a publication pending — read in one query and
/// without their marks.** Added 2026-09-28, step 10c's transfer wave piece 1.
///
/// **The question it answers is `Download`'s `transfers_may_be_admitted`**, and
/// that question reads one thing of each record: whether a publication is
/// pending. **The case the old reading was first written for** was a record
/// holding a few transfers, where reading [`all`] of them, each with every
/// checkpoint and superseded attempt, before every offer cost nothing. **The
/// case it now also meets** is a 5,000-frame job, where every offer read every
/// transfer record ever written, each with all its marks — 963 records by the
/// tenth pass's end, and more with every frame — to learn that one thing.
///
/// **So the records come back without their marks**: `checkpoints` and
/// `superseded_attempts` are empty here whatever the record holds. **A caller
/// that needs a record's marks reads it with [`read`]**, and this reading is
/// named for the one question it is fit for so that nobody takes an empty list
/// here for a record with nothing staged.
///
/// **A record that has ended with no publication pending cannot change the
/// answer**, so leaving it out changes nothing the question can see; one that
/// has not ended is kept because it may yet hold one.
pub fn the_unended_and_the_pending_without_their_marks(
    connection: &rusqlite::Connection,
    server: &Identity,
) -> Result<Vec<TransferRecord>, RecordFailure> {
    let mut statement = connection.prepare_cached(&format!(
        "SELECT {EVERY_COLUMN} FROM transfer_record
         WHERE server = ?1 AND (ending IS NULL OR publication_intent IS NOT NULL)
         ORDER BY rowid"
    ))?;
    let rows = statement.query_map([server.as_text()], |row| {
        let mut values = Vec::new();
        for at in 0..25 {
            values.push(row.get::<_, rusqlite::types::Value>(at)?);
        }
        Ok(values)
    })?;
    let mut composed = Vec::new();
    for row in rows {
        if let Some(record) = the_row_composed(&row?) {
            composed.push(record);
        }
    }
    Ok(composed)
}

/// **The upload parts that stand at one landing on one server** — records in
/// the upload direction for `final_path`, unended or *interrupted*, each read
/// whole with its marks. **What the dependency check's send asks before it
/// arranges a file afresh**, so a part an earlier send left is carried on
/// (`File Browser` K10 and K12: *"the one unchanged-source answer, for every
/// upload of a local source"*). The 38's piece 7.
///
/// **Which of them may be taken up is the caller's to judge** — the run that
/// wrote it, the source, the installation; this reading only narrows the
/// table to the landing, so a send pays one indexed reading per file rather
/// than reading every record ever written.
pub fn the_upload_parts_at(
    connection: &rusqlite::Connection,
    server: &Identity,
    final_path: &str,
) -> Result<Vec<TransferRecord>, RecordFailure> {
    let mut statement = connection.prepare_cached(
        "SELECT transfer FROM transfer_record
         WHERE server = ?1 AND final_path = ?2 AND direction = 'up'
           AND (ending IS NULL OR ending = 'interrupted')
         ORDER BY rowid",
    )?;
    let transfers = statement
        .query_map(rusqlite::params![server.as_text(), final_path], |row| {
            row.get::<_, String>(0)
        })?
        .collect::<Result<Vec<String>, _>>()?;
    let mut parts = Vec::new();
    for transfer in transfers {
        let Some(transfer) = Identity::read_back(IdentityKind::Transfer, &transfer) else {
            continue;
        };
        if let Some(part) = read(connection, &transfer)? {
            parts.push(part);
        }
    }
    Ok(parts)
}

/// **The downloads the start settles** — `Server protocol and transfers` S28's
/// desktop-side sweep and `Desktop app and delivery` C13's settling, at
/// application start. Added 2026-09-28, step 10c's transfer wave piece 2.
///
/// **It is [`all`], narrowed to the records either act can find anything in**,
/// each composed whole, marks and superseded attempts included:
///
/// - **downloads only** — a download's staging sits in the user's own folder,
///   which is the one direction a start on this computer can reach (S28's
///   second bullet); an upload's is on the machine and is the next
///   connection's;
/// - **every one that has not published**, whatever else it holds, since its
///   live attempt may be in a removable state;
/// - **every one holding a publication pending**, which is C13's;
/// - **and a published one only where it retains a superseded attempt**, whose
///   path S28 still derives.
///
/// **What is left out is a download that published with nothing superseded**,
/// which is nearly every frame an auto-download ever fetched, and from which
/// neither act can derive a single path. **The case [`all`] was written for**
/// was a record of a few transfers; **the case this meets** is every start
/// after a 5,000-frame job, where reading every record ever written, each with
/// its marks, would have been paid before the window opened, every time, and
/// more with every job.
pub fn the_downloads_the_start_settles(
    connection: &rusqlite::Connection,
) -> Result<Vec<TransferRecord>, RecordFailure> {
    let mut statement = connection.prepare(&format!(
        "SELECT {EVERY_COLUMN} FROM transfer_record
         WHERE direction = 'down'
           AND (ending IS NULL
                OR ending != 'published'
                OR publication_intent IS NOT NULL
                OR EXISTS (SELECT 1 FROM transfer_superseded_attempt AS superseded
                           WHERE superseded.transfer = transfer_record.transfer))
         ORDER BY rowid"
    ))?;
    let rows = statement.query_map([], |row| {
        let mut values = Vec::new();
        for at in 0..25 {
            values.push(row.get::<_, rusqlite::types::Value>(at)?);
        }
        Ok(values)
    })?;
    let mut composed = Vec::new();
    for row in rows {
        if let Some(record) = compose(connection, &row?)? {
            composed.push(record);
        }
    }
    Ok(composed)
}

fn compose(
    connection: &rusqlite::Connection,
    values: &[rusqlite::types::Value],
) -> Result<Option<TransferRecord>, RecordFailure> {
    let Some(mut record) = the_row_composed(values) else {
        return Ok(None);
    };
    let transfer = record.transfer.as_text();

    // **Each retained attempt number comes back with its own identity**
    // (`Server protocol and transfers` S9). **A null is not read as a supplier
    // that could not answer**: it is an attempt whose staging object was never
    // created, or a row a record written before this column existed holds, and
    // both are *nothing was recorded* rather than a recorded fact.
    {
        let mut statement = connection.prepare_cached(
            "SELECT attempt, staging_identity FROM transfer_superseded_attempt
             WHERE transfer = ?1 ORDER BY attempt",
        )?;
        let rows = statement.query_map([&transfer], |row| {
            Ok((row.get::<_, u32>(0)?, row.get::<_, Option<String>>(1)?))
        })?;
        for row in rows {
            let (attempt, written) = row?;
            record.superseded_attempts.push(SupersededAttempt {
                attempt,
                staging_identity: written.map(|text| read_identity(Some(text))),
            });
        }
    }

    {
        let mut statement = connection.prepare_cached(
            "SELECT offset_into_the_staged_file, digest FROM transfer_checkpoint
             WHERE transfer = ?1 ORDER BY offset_into_the_staged_file",
        )?;
        let rows = statement.query_map([&transfer], |row| {
            Ok((row.get::<_, i64>(0)?, row.get::<_, String>(1)?))
        })?;
        for row in rows {
            let (offset, digest) = row?;
            // **A digest that cannot be read back is a checkpoint that cannot
            // be verified against**, so it is left out rather than carried as a
            // value nothing can compare.
            if let Some(digest) = Digest::read_back(&digest) {
                record.checkpoints.push(Checkpoint {
                    offset: offset as u64,
                    digest_of_everything_up_to_it: digest,
                });
            }
        }
    }
    Ok(Some(record))
}

/// **The record's own row, with no marks.** [`compose`] adds them; the
/// admission's reading above does not.
fn the_row_composed(values: &[rusqlite::types::Value]) -> Option<TransferRecord> {
    let text = |at: usize| -> Option<String> {
        match &values[at] {
            rusqlite::types::Value::Text(text) => Some(text.clone()),
            _ => None,
        }
    };
    let number = |at: usize| -> Option<i64> {
        match &values[at] {
            rusqlite::types::Value::Integer(number) => Some(*number),
            _ => None,
        }
    };

    let transfer = text(0).and_then(|text| Identity::read_back(IdentityKind::Transfer, &text))?;
    let server = text(3).and_then(|text| Identity::read_back(IdentityKind::LocalServer, &text))?;
    let run = text(10).and_then(|text| Identity::read_back(IdentityKind::Run, &text))?;
    let attempt = number(1).unwrap_or(1) as u32;
    let staging_directory = text(7).unwrap_or_default();

    let source_reading = match (number(12), text(13), number(14), number(15)) {
        (Some(size), Some(inode), Some(modification_time), Some(change_time)) => {
            Some(SourceReading {
                size: size as u64,
                inode,
                modification_time,
                change_time,
            })
        }
        _ => None,
    };

    let publication_pending = match (text(16), text(17), text(18), number(20), text(21)) {
        (
            Some(intent),
            Some(staging_path),
            Some(final_path),
            Some(byte_length),
            Some(digest_text),
        ) => Digest::read_back(&digest_text).map(|staged_digest| PublicationPreState {
            intent: if intent == "create" {
                Intent::CreateIntended
            } else {
                Intent::ReplaceIntended
            },
            staging_path,
            final_path,
            staged_identity: read_identity(text(19)),
            staged_byte_length: byte_length as u64,
            staged_digest,
        }),
        _ => None,
    };

    Some(TransferRecord {
        staging_path: StagingPath::synthesise(&staging_directory, &transfer, attempt),
        transfer,
        attempt,
        superseded_attempts: Vec::new(),
        direction: if text(2).as_deref() == Some("down") {
            Direction::Down
        } else {
            Direction::Up
        },
        server,
        installation: text(4)
            .and_then(|text| Identity::read_back(IdentityKind::Installation, &text)),
        source_path: text(5).unwrap_or_default(),
        final_path: text(6).unwrap_or_default(),
        total_size_expected: number(8).unwrap_or(0) as u64,
        expected_digest: text(9).and_then(|text| Digest::read_back(&text)),
        checkpoints: Vec::new(),
        written_by_run: run,
        staging_identity: values
            .get(11)
            .filter(|value| !matches!(value, rusqlite::types::Value::Null))
            .map(|_| read_identity(text(11))),
        source_reading,
        publication_pending,
        ending: read_ending(text(22), text(23)),
        cleanup_is_owed: number(24).unwrap_or(0) != 0,
    })
}

/// **The identity is recorded as unavailable rather than left blank**, which is
/// what `Server protocol and transfers` S15's identity-unavailable branch is
/// taken from.
fn written_identity(identity: &StableFileIdentity) -> String {
    match identity {
        StableFileIdentity::The(value) => format!("the:{value}"),
        StableFileIdentity::TheSupplierCouldNotAnswer => "the supplier could not answer".to_string(),
    }
}

fn read_identity(written: Option<String>) -> StableFileIdentity {
    match written {
        Some(text) => match text.strip_prefix("the:") {
            Some(value) => StableFileIdentity::The(value.to_string()),
            None => StableFileIdentity::TheSupplierCouldNotAnswer,
        },
        None => StableFileIdentity::TheSupplierCouldNotAnswer,
    }
}

fn written_ending(ending: &Ending) -> &'static str {
    match ending {
        Ending::Published => "published",
        Ending::Stopped(_) => "stopped",
        Ending::Failed => "failed",
        Ending::Stalled => "stalled",
        Ending::Interrupted => "interrupted",
    }
}

fn written_reason(ending: &Ending) -> Option<&'static str> {
    match ending {
        Ending::Stopped(StoppedReason::StagingRemovedByUser) => Some("staging removed by user"),
        Ending::Stopped(StoppedReason::AbandonedByItsRequest) => Some("abandoned by its request"),
        Ending::Stopped(StoppedReason::TheSourceChangedUnderIt) => Some("the source changed"),
        _ => None,
    }
}

fn read_ending(written: Option<String>, reason: Option<String>) -> Option<Ending> {
    match written.as_deref()? {
        "published" => Some(Ending::Published),
        "failed" => Some(Ending::Failed),
        "stalled" => Some(Ending::Stalled),
        "interrupted" => Some(Ending::Interrupted),
        "stopped" => Some(Ending::Stopped(match reason.as_deref() {
            Some("staging removed by user") => StoppedReason::StagingRemovedByUser,
            Some("the source changed") => StoppedReason::TheSourceChangedUnderIt,
            _ => StoppedReason::AbandonedByItsRequest,
        })),
        // **A word this binary does not know is not an ending it may guess
        // at.** Reading it as *interrupted* would tell `Server protocol and
        // transfers` C11 step 4 to carry on work that is over. **That document
        // is written out because this crate's own plan defines a C11 too** —
        // what makes a judgement independent of every folding rule — and a bare
        // marker here resolves to it.
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::record::LocalRecord;

    fn a_record_on_disk(named: &str) -> (LocalRecord, std::path::PathBuf) {
        let folder = std::env::temp_dir().join(format!(
            "smoozy-transfer-records-{named}-{}",
            std::process::id()
        ));
        let _ = std::fs::remove_dir_all(&folder);
        std::fs::create_dir_all(&folder).expect("a folder");
        (
            LocalRecord::open_at(&folder.join("local-record.sqlite3")).expect("the record opens"),
            folder,
        )
    }

    fn an_upload_arranged() -> TransferRecord {
        let issue = |kind| Identity::issue(kind).expect("random bytes");
        TransferRecord::arranged(
            issue(IdentityKind::Transfer),
            Direction::Up,
            issue(IdentityKind::LocalServer),
            Some(issue(IdentityKind::Installation)),
            "/Users/a/large.blend".to_string(),
            "/home/ubuntu/jobs/large.blend".to_string(),
            "/home/ubuntu/jobs/.smoozy-staging",
            4_000_000_000,
            issue(IdentityKind::Run),
        )
    }

    fn a_mark_at(offset: u64, of: &str) -> Checkpoint {
        Checkpoint {
            offset,
            digest_of_everything_up_to_it: Digest::of(format!("{of} up to {offset}").as_bytes()),
        }
    }

    /// **How many rows one save wrote**, read off the engine's own count of
    /// rows changed on this connection.
    fn the_rows_one_save_writes(record: &LocalRecord, transfer: &TransferRecord) -> u64 {
        let before = record.connection().total_changes();
        put(record.connection(), transfer).expect("written");
        record.connection().total_changes() - before
    }

    /// **A save at the thousandth checkpoint writes what the first one did —
    /// the one new mark — and reads back whole.**
    ///
    /// **The case this bites on is `put` as it was**: it deleted every mark and
    /// inserted them all again, so the thousandth checkpoint wrote two
    /// thousand and one rows, each its own save.
    #[test]
    fn the_thousandth_checkpoint_writes_one_mark_and_the_record_reads_back_whole() {
        let (record, folder) = a_record_on_disk("thousand");
        let mut transfer = an_upload_arranged();
        put(record.connection(), &transfer).expect("the record before the first byte");
        for step in 1..=1_000u64 {
            transfer.checkpoints.push(a_mark_at(step * 4_194_304, "attempt 1"));
            let wrote = the_rows_one_save_writes(&record, &transfer);
            assert_eq!(
                wrote, 1,
                "checkpoint {step} wrote {wrote} rows: the new mark is one, and the record's own \
                 row did not move"
            );
        }
        let back = read(record.connection(), &transfer.transfer)
            .expect("it reads")
            .expect("it is there");
        assert_eq!(back, transfer);
        drop(record);
        let _ = std::fs::remove_dir_all(&folder);
    }

    /// **A start-over, then a resume, reads back exactly what was written** —
    /// `Server protocol and transfers` S12 step 5 empties the list, and the
    /// stored marks are the old attempt's, so they are replaced rather than
    /// kept beside the new ones; the new attempt's marks then go down one at a
    /// time again. **The list cleared because nothing was staged** (step 1)
    /// goes the same way.
    #[test]
    fn a_start_over_then_a_resume_reads_back_exactly_what_was_written() {
        let (record, folder) = a_record_on_disk("start-over");
        let mut transfer = an_upload_arranged();
        for step in 1..=5u64 {
            transfer.checkpoints.push(a_mark_at(step * 4_194_304, "attempt 1"));
            put(record.connection(), &transfer).expect("written");
        }
        transfer.staging_identity = Some(StableFileIdentity::The("volume-1:file-7".into()));
        put(record.connection(), &transfer).expect("written");

        // **The start-over**: the attempt moves on, the old one is retained
        // with its identity, and the list is empty.
        transfer.start_over();
        put(record.connection(), &transfer).expect("the start-over is written");
        let back = read(record.connection(), &transfer.transfer).unwrap().unwrap();
        assert_eq!(back, transfer, "the old attempt's marks were left beside the new ones");
        assert!(back.checkpoints.is_empty());
        assert_eq!(back.superseded_attempts.len(), 1);

        // **The resume**: the new attempt's marks, one save each, and one row
        // each.
        for step in 1..=3u64 {
            transfer.checkpoints.push(a_mark_at(step * 4_194_304, "attempt 2"));
            assert_eq!(the_rows_one_save_writes(&record, &transfer), 1);
        }

        // **A save in which nothing moved writes nothing.**
        assert_eq!(the_rows_one_save_writes(&record, &transfer), 0);
        assert_eq!(read(record.connection(), &transfer.transfer).unwrap().unwrap(), transfer);

        // **Step 1's clearing**: nothing staged, so the marks go and nothing
        // else moves.
        transfer.checkpoints.clear();
        put(record.connection(), &transfer).expect("the clearing is written");
        assert_eq!(read(record.connection(), &transfer.transfer).unwrap().unwrap(), transfer);

        // **A superseded attempt's identity learned later is written**, which
        // is the one other way that list moves.
        transfer.superseded_attempts[0].staging_identity =
            Some(StableFileIdentity::TheSupplierCouldNotAnswer);
        put(record.connection(), &transfer).expect("written");
        assert_eq!(read(record.connection(), &transfer.transfer).unwrap().unwrap(), transfer);
        drop(record);
        let _ = std::fs::remove_dir_all(&folder);
    }

    /// **A save that fails half-way leaves nothing of itself**: the mark that
    /// would have broken the table's key is refused, and the record reads back
    /// as it stood before that save — the savepoint's whole point.
    #[test]
    fn a_save_that_fails_leaves_the_record_as_it_stood() {
        let (record, folder) = a_record_on_disk("fails");
        let mut transfer = an_upload_arranged();
        transfer.checkpoints.push(a_mark_at(4_194_304, "attempt 1"));
        put(record.connection(), &transfer).expect("written");
        let before = read(record.connection(), &transfer.transfer).unwrap().unwrap();

        let mut broken = transfer.clone();
        broken.checkpoints.push(a_mark_at(8_388_608, "attempt 1"));
        broken.checkpoints.push(a_mark_at(8_388_608, "attempt 1"));
        broken.cleanup_is_owed = true;
        assert!(put(record.connection(), &broken).is_err());
        assert_eq!(read(record.connection(), &transfer.transfer).unwrap().unwrap(), before);
        assert!(
            record.connection().is_autocommit(),
            "the failed save left the connection inside a transaction"
        );
        drop(record);
        let _ = std::fs::remove_dir_all(&folder);
    }

    /// **The admission's reading holds this server's unended records and those
    /// with a publication pending, and nothing else** — and says it carries no
    /// marks rather than pretending to.
    #[test]
    fn the_admissions_reading_is_this_servers_unended_and_pending_records() {
        use smoozy_server_protocol::transfers::publication::PublicationPreState;
        let (record, folder) = a_record_on_disk("admission");
        let mut unended = an_upload_arranged();
        unended.checkpoints.push(a_mark_at(4_194_304, "attempt 1"));
        let server = unended.server.clone();
        let mut ended = an_upload_arranged();
        ended.server = server.clone();
        ended.ending = Some(Ending::Published);
        let mut pending = an_upload_arranged();
        pending.server = server.clone();
        pending.ending = Some(Ending::Interrupted);
        pending.publication_pending = Some(PublicationPreState {
            intent: Intent::CreateIntended,
            staging_path: "/home/ubuntu/jobs/.smoozy-staging/x".into(),
            final_path: "/home/ubuntu/jobs/large.blend".into(),
            staged_identity: StableFileIdentity::The("volume-1:file-9".into()),
            staged_byte_length: 4_000_000_000,
            staged_digest: Digest::of(b"the whole file"),
        });
        let elsewhere = an_upload_arranged();
        for one in [&unended, &ended, &pending, &elsewhere] {
            put(record.connection(), one).expect("written");
        }

        let read_back =
            the_unended_and_the_pending_without_their_marks(record.connection(), &server)
                .expect("it reads");
        let transfers: Vec<&Identity> = read_back.iter().map(|one| &one.transfer).collect();
        assert_eq!(transfers, vec![&unended.transfer, &pending.transfer]);
        assert!(read_back.iter().all(|one| one.checkpoints.is_empty()));
        assert_eq!(read_back[1].publication_pending, pending.publication_pending);
        drop(record);
        let _ = std::fs::remove_dir_all(&folder);
    }
}

/// `Server protocol and transfers` S0a: milliseconds on the desktop UTC wall clock and one desktop-wide
/// sequence. These are presentation values, never trust sequences.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct TransferStart {
    pub started_at_utc: i64,
    pub start_order: i64,
}

/// Called inside the parent's write transaction, before its first insert.
/// Existing legacy parents stay unknown. Only an off -> on arrangement
/// explicitly asks for a new pair. A failed parent write rolls back the pair.
pub(crate) fn start_for_parent(
    connection: &rusqlite::Connection, table: &str, key: &str, id: &str, renew: bool,
) -> rusqlite::Result<Option<TransferStart>> {
    let previous = connection.query_row(
        &format!("SELECT started_at_utc, start_order FROM {table} WHERE {key} = ?1"),
        [id], |row| Ok((row.get::<_, Option<i64>>(0)?, row.get::<_, Option<i64>>(1)?)),
    ).optional()?;
    if !renew {
        if let Some((at, order)) = previous {
            return Ok(at.zip(order).map(|(started_at_utc, start_order)| TransferStart { started_at_utc, start_order }));
        }
    }
    connection.execute("INSERT OR IGNORE INTO transfer_start_order (only_row, last_order) VALUES (1, 0)", [])?;
    connection.execute("UPDATE transfer_start_order SET last_order = last_order + 1 WHERE only_row = 1", [])?;
    let start_order = connection.query_row("SELECT last_order FROM transfer_start_order WHERE only_row = 1", [], |r| r.get(0))?;
    let started_at_utc = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH)
        .map_err(|_| rusqlite::Error::InvalidQuery)?.as_millis().try_into().map_err(|_| rusqlite::Error::InvalidQuery)?;
    Ok(Some(TransferStart { started_at_utc, start_order }))
}

pub(crate) fn parent_start(connection: &rusqlite::Connection, table: &str, key: &str, id: &str) -> rusqlite::Result<Option<TransferStart>> {
    connection.query_row(&format!("SELECT started_at_utc, start_order FROM {table} WHERE {key} = ?1"), [id], |r| {
        Ok(r.get::<_, Option<i64>>(0)?.zip(r.get::<_, Option<i64>>(1)?)
            .map(|(started_at_utc, start_order)| TransferStart { started_at_utc, start_order }))
    }).optional().map(Option::flatten)
}

#[cfg(test)]
mod presentation_tests {
    use super::*;
    use smoozy_download::arrangement::{AnArrangement, TheNarrowing, KeepsTheArrangement, DesiredState};
    use crate::record::download_records::DownloadArrangements;
    use crate::the_found_files_sent::{ASend, AFileSent, HowItEnded};

    #[test]
    fn seventeenth_start_allocations_roll_back_preserve_and_renew_only_on_switch_on() {
        let c = rusqlite::Connection::open_in_memory().unwrap();
        crate::record::migrations::bring_up_to_date(&c, std::path::Path::new("in-memory"), false).unwrap();
        let issue = |kind| Identity::issue(kind).unwrap();
        let mut send = ASend { request: issue(IdentityKind::Operation), server: issue(IdentityKind::LocalServer).as_text(), file: "/scene.blend".into(),
            files: vec![AFileSent { landing: b"/texture".to_vec(), size: u64::MAX, transfer: None, ended: None }] };
        assert!(crate::record::dependency_records::save_dependency_send(&c, &send).is_err());
        assert_eq!(c.query_row("SELECT count(*) FROM dependency_upload_request", [], |r| r.get::<_, i64>(0)).unwrap(), 0);
        assert_eq!(c.query_row("SELECT count(*) FROM transfer_start_order", [], |r| r.get::<_, i64>(0)).unwrap(), 0);
        send.files[0].size = 1;
        crate::record::dependency_records::save_dependency_send(&c, &send).unwrap();
        let start = parent_start(&c, "dependency_upload_request", "request", &send.request.as_text()).unwrap().unwrap();
        assert_eq!(start.start_order, 1);
        let older_snapshot = send.clone();
        send.files[0].ended = Some(HowItEnded::Published);
        crate::record::dependency_records::dependency_file_moved(&c, &send.request, 0, &send.files[0]).unwrap();
        crate::record::dependency_records::save_dependency_send(&c, &older_snapshot).unwrap();
        assert_eq!(crate::record::dependency_records::saved_dependency_sends(&c).unwrap()[0].files[0].ended, Some(HowItEnded::Published), "a late snapshot must never erase a file's ending");
        assert_eq!(parent_start(&c, "dependency_upload_request", "request", &send.request.as_text()).unwrap(), Some(start));
        let counted = crate::record::dependency_records::dependency_sends_counted(&c).unwrap();
        assert_eq!((counted[0].files, counted[0].published, counted[0].bytes, counted[0].published_bytes), (1, 1, 1, 1));
        assert!(counted[0].all_ended && !counted[0].failed);
        let mut auto = AnArrangement::switched_on(issue(IdentityKind::Job), issue(IdentityKind::LocalServer), b"/downloads".to_vec(), TheNarrowing::Everything);
        let mut store = DownloadArrangements::over(&c);
        store.write_it_down(&auto).unwrap();
        let first = parent_start(&c, "download_arrangement", "job", &auto.job.as_text()).unwrap().unwrap();
        auto.narrowing = TheNarrowing::TheseFormats(vec![b"PNG".to_vec()]);
        store.write_it_down(&auto).unwrap();
        auto.desired_state = DesiredState::Off;
        store.write_it_down(&auto).unwrap();
        assert_eq!(parent_start(&c, "download_arrangement", "job", &auto.job.as_text()).unwrap(), Some(first));
        auto.desired_state = DesiredState::On;
        store.write_it_down(&auto).unwrap();
        let next = parent_start(&c, "download_arrangement", "job", &auto.job.as_text()).unwrap().unwrap();
        assert_eq!(next.start_order, first.start_order + 1);
        store.write_it_down(&auto).unwrap();
        assert_eq!(parent_start(&c, "download_arrangement", "job", &auto.job.as_text()).unwrap(), Some(next));
        // Expiry/removal of parents cannot reset the desktop-wide order.
        c.execute("DELETE FROM dependency_upload_file", []).unwrap();
        c.execute("DELETE FROM dependency_upload_request", []).unwrap();
        c.execute("DELETE FROM download_arrangement", []).unwrap();
        store.write_it_down(&auto).unwrap();
        assert_eq!(parent_start(&c, "download_arrangement", "job", &auto.job.as_text()).unwrap().unwrap().start_order, next.start_order + 1);
    }
}
