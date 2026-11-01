//! **The update route's three rows in the local record** — §5, with `State,
//! identity and recovery` J6's first row, `State, identity and recovery` I4's
//! two values and `State, identity and recovery` I9's update-in-probe record.
//! All three arrive in record version 49, together, because the update route
//! is the one thing that needs them.
//!
//! **The rules are not here.** What the trust record is worth is E19 and
//! `State, identity and recovery` J7's, performed in
//! `crate::updating::trust`; when a check counts as completed is Z23a and
//! Z28's; what a launch does with a probe record is Z21a's. **What is here is
//! the store**, the split every row module in this folder draws. A2 holds: no
//! module has a durable store of its own.
//!
//! - **The desktop-update trust record** — one row or none: the greatest
//!   accepted release sequence, the greatest accepted trust sequence, and the
//!   statement accepted at it with its recovery signature. **It shares no field
//!   with the server-package trust record beside it** (`State, identity and
//!   recovery` J10), and it is its own table.
//! - **The last completed check's moment and the declined version** — one row,
//!   two values, each written on its own. The moment is this computer's clock,
//!   and the column says so in its name, because `State, identity and recovery`
//!   I4 carries the value *"with the clock it was taken on"*. **A moment and
//!   never an age** — nothing here subtracts.
//! - **The update-in-probe record** — one row or none, Z21a's three things: the
//!   release being published, the previous artifact kept, and L8's record copy.
//!   The two paths are kept as their bytes, as every path in this record is.
//!   **Its lifetime is `State, identity and recovery` R8a's**: it is cleared by
//!   the probe's success or by a completed rollback and by nothing else, and
//!   nothing here clears it on its own.
//!
//! **L10: no secret value is in any of them.** The statement and its signature
//! are public by construction.
//!
//! **How a clause marker is written in this crate, and it is a rule rather than
//! a habit.** A marker with no document beside it means a clause of this
//! crate's own plan, `Preparation/Architecture/Desktop app and delivery.md`.
//! Every other document's marker is written with that document at the site.

use rusqlite::OptionalExtension;

use super::{moment, nanoseconds, RecordFailure};
use crate::updating::trust::TheUpdateTrustRecord;

// --------------------------------------------- the desktop-update trust record

/// **Read the one row, or nothing.** A signature that is not sixty-four bytes
/// reads as no record — the damaged record `State, identity and recovery` J8
/// says must have a route out.
pub fn read_the_trust_record(
    connection: &rusqlite::Connection,
) -> Result<Option<TheUpdateTrustRecord>, RecordFailure> {
    let row = connection
        .query_row(
            "SELECT greatest_accepted_release_sequence, greatest_accepted_trust_sequence,
                    the_statement, its_recovery_signature
             FROM update_trust WHERE only_row = 1",
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
    let Some((release_sequence, trust_sequence, statement, signature)) = row else {
        return Ok(None);
    };
    let Ok(its_recovery_signature) = <[u8; 64]>::try_from(signature.as_slice()) else {
        return Ok(None);
    };
    // **Unsigned 64 bits kept in the engine's signed column**: written as the
    // same sixty-four bits and read back as them, so every value round-trips.
    Ok(Some(TheUpdateTrustRecord {
        greatest_accepted_release_sequence: release_sequence as u64,
        greatest_accepted_trust_sequence: trust_sequence as u64,
        the_statement_accepted_at_that_trust_sequence: statement,
        its_recovery_signature,
    }))
}

/// **Write the row, replacing whatever stood.** The caller writes what it has
/// just accepted, never a raised number on its own.
pub fn write_the_trust_record(
    connection: &rusqlite::Connection,
    record: &TheUpdateTrustRecord,
) -> Result<(), RecordFailure> {
    connection.execute(
        "INSERT INTO update_trust
             (only_row, greatest_accepted_release_sequence, greatest_accepted_trust_sequence,
              the_statement, its_recovery_signature)
         VALUES (1, ?1, ?2, ?3, ?4)
         ON CONFLICT (only_row) DO UPDATE SET
             greatest_accepted_release_sequence = excluded.greatest_accepted_release_sequence,
             greatest_accepted_trust_sequence = excluded.greatest_accepted_trust_sequence,
             the_statement = excluded.the_statement,
             its_recovery_signature = excluded.its_recovery_signature",
        rusqlite::params![
            record.greatest_accepted_release_sequence as i64,
            record.greatest_accepted_trust_sequence as i64,
            &record.the_statement_accepted_at_that_trust_sequence,
            &record.its_recovery_signature[..],
        ],
    )?;
    Ok(())
}

/// **`State, identity and recovery` J7's discard**: the statement and both
/// numbers go together, and the record is gone rather than lowered.
pub fn discard_the_trust_record(connection: &rusqlite::Connection) -> Result<(), RecordFailure> {
    connection.execute("DELETE FROM update_trust", [])?;
    Ok(())
}

// ------------------------------------------ the check's two values

/// **The moment the last update check completed, on this computer's clock**,
/// or none where no check has.
pub fn the_last_completed_check(
    connection: &rusqlite::Connection,
) -> Result<Option<std::time::SystemTime>, RecordFailure> {
    let at: Option<Option<i64>> = connection
        .query_row(
            "SELECT last_completed_on_this_computer_at FROM update_check WHERE only_row = 1",
            [],
            |row| row.get(0),
        )
        .optional()?;
    Ok(at.flatten().map(moment))
}

/// **A check completed, at this moment on this computer's clock.** Written
/// only for a check that completed (Z23a, Z28); the declined version beside it
/// is not touched.
pub fn a_check_completed(
    connection: &rusqlite::Connection,
    at: std::time::SystemTime,
) -> Result<(), RecordFailure> {
    let Some(at) = nanoseconds(at) else {
        return Err(RecordFailure::AMomentTheRecordCannotHold {
            column: "update_check.last_completed_on_this_computer_at",
            reported: format!("{at:?} is not a moment this record can spell"),
        });
    };
    connection.execute(
        "INSERT INTO update_check (only_row, last_completed_on_this_computer_at) VALUES (1, ?1)
         ON CONFLICT (only_row) DO UPDATE SET
             last_completed_on_this_computer_at = excluded.last_completed_on_this_computer_at",
        [at],
    )?;
    Ok(())
}

/// **The version the user last declined at the Linux install box**, or none.
pub fn the_declined_version(
    connection: &rusqlite::Connection,
) -> Result<Option<String>, RecordFailure> {
    let version: Option<Option<String>> = connection
        .query_row(
            "SELECT declined_version FROM update_check WHERE only_row = 1",
            [],
            |row| row.get(0),
        )
        .optional()?;
    Ok(version.flatten())
}

/// **The user declined this version.** One value overwritten, never a list
/// (`State, identity and recovery` I4); the moment beside it is not touched.
pub fn this_version_was_declined(
    connection: &rusqlite::Connection,
    version: &str,
) -> Result<(), RecordFailure> {
    connection.execute(
        "INSERT INTO update_check (only_row, declined_version) VALUES (1, ?1)
         ON CONFLICT (only_row) DO UPDATE SET declined_version = excluded.declined_version",
        [version],
    )?;
    Ok(())
}

// --------------------------------------------- the update-in-probe record

/// **Z21a's update-in-probe record, every field it names.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TheUpdateInProbe {
    /// The release being published: its release sequence and its version.
    pub release_sequence: u64,
    pub version: String,
    /// Where the previous artifact is kept — the named bundle backup, the
    /// cached package, or the kept folder — as the path's own bytes.
    pub the_previous_artifact: Vec<u8>,
    /// Where L8's record copy is, as the path's own bytes.
    pub the_record_copy: Vec<u8>,
}

/// The record, or none.
pub fn the_update_in_probe(
    connection: &rusqlite::Connection,
) -> Result<Option<TheUpdateInProbe>, RecordFailure> {
    Ok(connection
        .query_row(
            "SELECT release_sequence, version, previous_artifact, record_copy
             FROM update_in_probe WHERE only_row = 1",
            [],
            |row| {
                Ok(TheUpdateInProbe {
                    release_sequence: row.get::<_, i64>(0)? as u64,
                    version: row.get(1)?,
                    the_previous_artifact: row.get(2)?,
                    the_record_copy: row.get(3)?,
                })
            },
        )
        .optional()?)
}

/// **Written immediately before publication** (Z21a), replacing any that
/// stood.
pub fn an_update_goes_into_probe(
    connection: &rusqlite::Connection,
    probe: &TheUpdateInProbe,
) -> Result<(), RecordFailure> {
    connection.execute(
        "INSERT INTO update_in_probe (only_row, release_sequence, version, previous_artifact, record_copy)
         VALUES (1, ?1, ?2, ?3, ?4)
         ON CONFLICT (only_row) DO UPDATE SET
             release_sequence = excluded.release_sequence,
             version = excluded.version,
             previous_artifact = excluded.previous_artifact,
             record_copy = excluded.record_copy",
        rusqlite::params![
            probe.release_sequence as i64,
            probe.version,
            probe.the_previous_artifact,
            probe.the_record_copy,
        ],
    )?;
    Ok(())
}

/// **Cleared**, by the success token or by a completed rollback's last act
/// (Z21a), and by nothing else.
pub fn the_probe_is_cleared(connection: &rusqlite::Connection) -> Result<(), RecordFailure> {
    connection.execute("DELETE FROM update_in_probe", [])?;
    Ok(())
}

/// The Linux watcher failed: an ordinary launch must finish the rollback,
/// even if it can start the new release. Part of the existing probe record.
pub fn a_linux_rollback_is_required(
    connection: &rusqlite::Connection,
) -> Result<(), RecordFailure> {
    connection.execute(
        "UPDATE update_in_probe SET linux_rollback_required = 1 WHERE only_row = 1",
        [],
    )?;
    Ok(())
}

/// Read before ordinary startup, including before migrating a newer record.
pub fn a_linux_rollback_is_owed(connection: &rusqlite::Connection) -> Result<bool, RecordFailure> {
    use rusqlite::OptionalExtension;
    Ok(connection
        .query_row(
            "SELECT linux_rollback_required FROM update_in_probe WHERE only_row = 1",
            [],
            |row| row.get(0),
        )
        .optional()?
        .unwrap_or(false))
}

/// Package reinstall succeeded; only record restore and cleanup remain.
pub fn the_linux_reinstall_finished(connection: &rusqlite::Connection) -> Result<(), RecordFailure> {
    connection.execute("UPDATE update_in_probe SET linux_rollback_required = 2 WHERE only_row = 1", [])?;
    Ok(())
}

pub fn the_linux_rollback_phase(connection: &rusqlite::Connection) -> Result<u8, RecordFailure> {
    Ok(connection.query_row("SELECT linux_rollback_required FROM update_in_probe WHERE only_row = 1", [], |row| row.get(0)).optional()?.unwrap_or(0))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn an_open_record() -> rusqlite::Connection {
        let connection = rusqlite::Connection::open_in_memory().unwrap();
        for statement in super::super::schema::VERSION_49 {
            connection.execute(statement, []).unwrap();
        }
        connection
    }

    fn a_trust_record() -> TheUpdateTrustRecord {
        TheUpdateTrustRecord {
            greatest_accepted_release_sequence: u64::MAX,
            greatest_accepted_trust_sequence: 7,
            the_statement_accepted_at_that_trust_sequence: b"the statement's own bytes".to_vec(),
            its_recovery_signature: [5; 64],
        }
    }

    #[test]
    fn the_trust_record_reads_back_whole_is_replaced_whole_and_discards_whole() {
        let connection = an_open_record();
        assert_eq!(read_the_trust_record(&connection).unwrap(), None);
        write_the_trust_record(&connection, &a_trust_record()).unwrap();
        assert_eq!(
            read_the_trust_record(&connection).unwrap(),
            Some(a_trust_record())
        );
        let later = TheUpdateTrustRecord {
            greatest_accepted_trust_sequence: 8,
            ..a_trust_record()
        };
        write_the_trust_record(&connection, &later).unwrap();
        assert_eq!(read_the_trust_record(&connection).unwrap(), Some(later));
        discard_the_trust_record(&connection).unwrap();
        assert_eq!(read_the_trust_record(&connection).unwrap(), None);
    }

    #[test]
    fn a_signature_that_is_not_a_signature_reads_as_no_record() {
        let connection = an_open_record();
        write_the_trust_record(&connection, &a_trust_record()).unwrap();
        connection
            .execute("UPDATE update_trust SET its_recovery_signature = x'00'", [])
            .unwrap();
        assert_eq!(read_the_trust_record(&connection).unwrap(), None);
    }

    #[test]
    fn the_checks_two_values_are_kept_apart_and_each_is_one_value() {
        let connection = an_open_record();
        assert_eq!(the_last_completed_check(&connection).unwrap(), None);
        assert_eq!(the_declined_version(&connection).unwrap(), None);
        let at = std::time::UNIX_EPOCH + std::time::Duration::from_nanos(1_761_300_000_123_456_789);
        a_check_completed(&connection, at).unwrap();
        assert_eq!(the_last_completed_check(&connection).unwrap(), Some(at));
        assert_eq!(the_declined_version(&connection).unwrap(), None);
        this_version_was_declined(&connection, "0.2.1").unwrap();
        this_version_was_declined(&connection, "0.2.2").unwrap();
        assert_eq!(
            the_declined_version(&connection).unwrap().as_deref(),
            Some("0.2.2")
        );
        assert_eq!(
            the_last_completed_check(&connection).unwrap(),
            Some(at),
            "a decline moved the moment"
        );
        let later = at + std::time::Duration::from_secs(60);
        a_check_completed(&connection, later).unwrap();
        assert_eq!(the_last_completed_check(&connection).unwrap(), Some(later));
        assert_eq!(
            the_declined_version(&connection).unwrap().as_deref(),
            Some("0.2.2"),
            "a check moved the mark"
        );
        // A moment the record cannot spell is refused, never written as zero.
        let before_the_epoch = std::time::UNIX_EPOCH - std::time::Duration::from_secs(1);
        assert!(matches!(
            a_check_completed(&connection, before_the_epoch),
            Err(RecordFailure::AMomentTheRecordCannotHold { .. })
        ));
        assert_eq!(the_last_completed_check(&connection).unwrap(), Some(later));
    }

    #[test]
    fn the_probe_record_is_written_whole_and_cleared_only_by_its_own_act() {
        let connection = an_open_record();
        assert_eq!(the_update_in_probe(&connection).unwrap(), None);
        let probe = TheUpdateInProbe {
            release_sequence: 1_900,
            version: "0.2.0".into(),
            the_previous_artifact: b"/Applications/Smoozy \xff backup.app".to_vec(),
            the_record_copy: b"/Users/a/Library/Smoozy/local-record.sqlite3.before-migration"
                .to_vec(),
        };
        an_update_goes_into_probe(&connection, &probe).unwrap();
        assert_eq!(
            the_update_in_probe(&connection).unwrap(),
            Some(probe.clone())
        );
        // Neither of the check's values reaches it.
        a_check_completed(&connection, std::time::SystemTime::now()).unwrap();
        this_version_was_declined(&connection, "0.2.0").unwrap();
        assert_eq!(the_update_in_probe(&connection).unwrap(), Some(probe));
        the_probe_is_cleared(&connection).unwrap();
        assert_eq!(the_update_in_probe(&connection).unwrap(), None);
    }
}
