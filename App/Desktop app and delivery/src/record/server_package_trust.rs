//! The server-package trust record in the local record — `Desktop app and
//! delivery` §5 and `State, identity and recovery` J6.
//!
//! **The rule is `Server protocol and transfers` B9's and the store is this
//! document's**, which is the split every other row module here draws. Nothing
//! decides here what the record means; what is decided here is that it is one
//! row of the one local record, like everything else durable on this side.
//!
//! **A2 holds: no module has a durable store of its own.**
//!
//! **`State, identity and recovery` J7 is why [`read`] hands back the row
//! rather than a verdict.** Whether the stored numbers stand at all is a
//! question about the retained statement's signature under the release root,
//! and the root and the verification are `Server protocol and transfers`' —
//! `smoozy_server_protocol::release::trust::what_the_record_is_worth`. A store
//! that answered it here would be a second copy of a rule, and the copy that
//! goes stale is always the one further from the plan.
//!
//! **How a clause marker is written in this crate, and it is a rule rather than
//! a habit.** A marker with no document beside it means a clause of this crate's
//! own plan, `Preparation/Architecture/Desktop app and delivery.md`. Every other
//! document's marker is written with that document — at the site, or in this
//! file's own header list below.
//!
//! **Markers used below that belong to another document, each with the document
//! it belongs to.** `State, identity and recovery` J6, J7, J8, J10.
//! `Server protocol and transfers` B9.

use rusqlite::OptionalExtension;
use smoozy_server_protocol::release::trust::TheTrustRecord;

use super::RecordFailure;

/// **Read the one row, or nothing.**
///
/// **Nothing is returned where a column cannot be read back into the shape it
/// was written as** — a signature that is not sixty-four bytes is not a
/// signature, and a row like that is the damaged record `State, identity and
/// recovery` J8 says must have a route out rather than a refusal that lasts
/// for ever. It reads as no record, and the next genuine statement
/// re-establishes the record's floor on its own merits, above whatever
/// `Server protocol and transfers` B9's built-in pair already holds.
pub fn read(
    connection: &rusqlite::Connection,
) -> Result<Option<TheTrustRecord>, RecordFailure> {
    let row = connection
        .query_row(
            "SELECT greatest_accepted_release_sequence, greatest_accepted_key_set_generation,
                    the_statement, its_root_signature
             FROM server_package_trust WHERE only_row = 1",
            [],
            |row| {
                Ok((
                    row.get::<_, i64>(0)?,
                    row.get::<_, i64>(1)?,
                    row.get::<_, Vec<u8>>(2)?,
                    row.get::<_, Vec<u8>>(3)?,
                ))
            },
        )
        .optional()?;
    let Some((release_sequence, generation, statement, signature)) = row else {
        return Ok(None);
    };
    let Ok(its_root_signature) = <[u8; 64]>::try_from(signature.as_slice()) else {
        return Ok(None);
    };
    Ok(Some(TheTrustRecord {
        greatest_accepted_release_sequence: release_sequence as u64,
        greatest_accepted_key_set_generation: generation as u64,
        the_statement_accepted_at_that_generation: statement,
        its_root_signature,
    }))
}

/// **Write the row, replacing whatever stood.**
///
/// **The caller writes what it has just accepted and never a raised number on
/// its own**, because `State, identity and recovery` J6 keeps the two numbers
/// and the statement that carried them together: a number without its statement
/// is the state J7 discards.
pub fn write(
    connection: &rusqlite::Connection,
    record: &TheTrustRecord,
) -> Result<(), RecordFailure> {
    connection.execute(
        "INSERT INTO server_package_trust
             (only_row, greatest_accepted_release_sequence,
              greatest_accepted_key_set_generation, the_statement, its_root_signature)
         VALUES (1, ?1, ?2, ?3, ?4)
         ON CONFLICT (only_row) DO UPDATE SET
             greatest_accepted_release_sequence = excluded.greatest_accepted_release_sequence,
             greatest_accepted_key_set_generation = excluded.greatest_accepted_key_set_generation,
             the_statement = excluded.the_statement,
             its_root_signature = excluded.its_root_signature",
        rusqlite::params![
            record.greatest_accepted_release_sequence as i64,
            record.greatest_accepted_key_set_generation as i64,
            &record.the_statement_accepted_at_that_generation,
            &record.its_root_signature[..],
        ],
    )?;
    Ok(())
}

/// **Discard it — `State, identity and recovery` J7's own act.**
///
/// The statement and both numbers go together. **This is not a rollback**: the
/// record's floor is gone rather than lowered, and what stands in its place is
/// `Server protocol and transfers` B9's built-in pair, which this build carries
/// and which this store knows nothing about.
pub fn discard(connection: &rusqlite::Connection) -> Result<(), RecordFailure> {
    connection.execute("DELETE FROM server_package_trust", [])?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn a_record() -> TheTrustRecord {
        TheTrustRecord {
            greatest_accepted_release_sequence: 42,
            greatest_accepted_key_set_generation: 7,
            the_statement_accepted_at_that_generation: b"the statement's own bytes".to_vec(),
            its_root_signature: [3; 64],
        }
    }

    fn an_open_record() -> rusqlite::Connection {
        let connection = rusqlite::Connection::open_in_memory().unwrap();
        for statement in super::super::schema::VERSION_11 {
            connection.execute(statement, []).unwrap();
        }
        connection
    }

    #[test]
    fn a_record_that_was_never_written_reads_as_nothing() {
        assert_eq!(read(&an_open_record()).unwrap(), None);
    }

    #[test]
    fn what_was_written_reads_back_whole_and_a_second_write_replaces_it() {
        let connection = an_open_record();
        write(&connection, &a_record()).unwrap();
        assert_eq!(read(&connection).unwrap(), Some(a_record()));

        let later = TheTrustRecord {
            greatest_accepted_release_sequence: 43,
            ..a_record()
        };
        write(&connection, &later).unwrap();
        assert_eq!(read(&connection).unwrap(), Some(later));
        let how_many: i64 = connection
            .query_row("SELECT count(*) FROM server_package_trust", [], |row| {
                row.get(0)
            })
            .unwrap();
        assert_eq!(how_many, 1);
    }

    #[test]
    fn discarding_leaves_no_record_rather_than_a_lower_one() {
        let connection = an_open_record();
        write(&connection, &a_record()).unwrap();
        discard(&connection).unwrap();
        assert_eq!(read(&connection).unwrap(), None);
    }

    #[test]
    fn a_signature_that_is_not_a_signature_reads_as_no_record_rather_than_as_one() {
        let connection = an_open_record();
        write(&connection, &a_record()).unwrap();
        connection
            .execute(
                "UPDATE server_package_trust SET its_root_signature = ?1",
                rusqlite::params![&b"too short"[..]],
            )
            .unwrap();
        assert_eq!(read(&connection).unwrap(), None);
    }
}
