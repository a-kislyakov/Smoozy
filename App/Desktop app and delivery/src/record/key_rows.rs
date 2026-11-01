//! `Keys`' rows in the local record — `Desktop app and delivery` §5 and
//! `Keys` R1.
//!
//! **The row's shape is `Keys`' and the store is this document's**, which is
//! the same split `Security and licence` §1.4 B30 draws for the secret
//! adapter: that module owns the rule, this one owns the mechanism. Nothing
//! here decides what a key row holds.
//!
//! **How a clause marker is written in this crate, and it is a rule rather than
//! a habit.** A marker with no document beside it means a clause of this crate's
//! own plan, `Preparation/Architecture/Desktop app and delivery.md`. Every other
//! document's marker is written with that document — at the site, or in this
//! file's own header list below. **Where a marker is a clause of this crate's
//! own plan *and* of the document meant, the header list is not enough and the
//! document is written out at the site**, because a bare one there resolves to a
//! real and wrong clause.
//!
//! **Markers used below that belong to another document, each with the document
//! it belongs to.** `Keys` U1, U7a, U9, U9a, W8, W10. `Security and licence`
//! B30, E3a.
//!
//! **A marker this crate's own plan also defines is deliberately not listed
//! above, whichever document is meant, and is written out with its document
//! at every site instead** — the header list cannot tell the two apart, and a
//! bare one would resolve to a real and wrong clause.
//!
//! **A2 holds: no module has a durable store of its own.** Every key row lives
//! here, in the one local record, and `Keys` R2 forbids a second store
//! anywhere.

use rusqlite::OptionalExtension;
use smoozy_keys::fingerprint::{Fingerprint, ReadFrom, Reading};
use smoozy_keys::rows::{KeyRow, Name, PassphraseReference};
use smoozy_security_and_licence::erasure::{EraseMarks, RecordFailure as EraseFailure};
use smoozy_security_and_licence::held::{HeldSecret, StoredKind, Subject};
use smoozy_state::identities::{Identity, IdentityKind};

use super::RecordFailure;

/// **U1 set 1: every key row in the local record**, shown whatever the current
/// state of its file. The order is the order they were added, which is a
/// stable order and not a promise about how a list draws.
pub fn all(connection: &rusqlite::Connection) -> Result<Vec<KeyRow>, RecordFailure> {
    // **U9's availability answer is the row's reference taken against the
    // owed-erase register** (`Security and licence` §5.1 E3a): a subject that
    // appears there is owed an erase, so availability is no from the instant
    // write 1 lands. **One home for the fact, and one that outlives the row.**
    let mut statement = connection.prepare(
        "SELECT k.identifier, k.path, k.fingerprint, k.fingerprint_read_from, k.name,
                k.was_default_at_entry, k.store_subject,
                EXISTS (SELECT 1 FROM owed_erase o
                        WHERE o.kind = 'key passphrase' AND o.subject = k.store_subject)
         FROM key_row k ORDER BY k.rowid",
    )?;
    let rows = statement.query_map([], |row| {
        Ok((
            row.get::<_, String>(0)?,
            row.get::<_, String>(1)?,
            row.get::<_, Option<String>>(2)?,
            row.get::<_, Option<String>>(3)?,
            row.get::<_, Option<String>>(4)?,
            row.get::<_, i64>(5)?,
            row.get::<_, Option<String>>(6)?,
            row.get::<_, i64>(7)?,
        ))
    })?;

    let mut keys = Vec::new();
    for row in rows {
        let (identifier, path, fingerprint, read_from, name, was_default, subject, owing) = row?;
        // **An identifier that will not read back is not composed from
        // anything else instead** (`State, identity and recovery` §2.1): the
        // row is skipped rather than given an invented identity, which is the
        // same refusal the issuer makes.
        let Some(identifier) = Identity::read_back(IdentityKind::LocalKey, &identifier) else {
            continue;
        };
        keys.push(KeyRow {
            identifier,
            path: path.into(),
            fingerprint: reading(fingerprint, read_from),
            name: name.as_deref().and_then(Name::what_the_user_typed),
            was_default_at_entry: was_default != 0,
            passphrase: subject.map(|subject| PassphraseReference {
                subject: Subject::issued_by_the_local_record(subject),
                owing_an_erase: owing != 0,
            }),
        });
    }
    Ok(keys)
}

/// **The fingerprint and the half it was read from travel together** (U7a).
/// Either alone is not a reading: a value with no source could be compared for
/// a swap it is no evidence of.
fn reading(value: Option<String>, read_from: Option<String>) -> Option<Reading> {
    let value = Fingerprint::from_the_printed_form(&value?)?;
    let read_from = match read_from?.as_str() {
        "private" => ReadFrom::PrivateFile,
        "public" => ReadFrom::PublicHalfBeside,
        _ => return None,
    };
    Some(Reading { value, read_from })
}

fn read_from_text(read_from: ReadFrom) -> &'static str {
    match read_from {
        ReadFrom::PrivateFile => "private",
        ReadFrom::PublicHalfBeside => "public",
    }
}

/// The row creation — the single partition unit of *point at a key*, and
/// generation's third unit.
pub fn add(connection: &rusqlite::Connection, row: &KeyRow) -> Result<(), RecordFailure> {
    // **The owed mark is not a field of this row** (`Security and licence` §5.1
    // E3a): it is the register's, so that a debt outlives the row that carried
    // the reference. A row written here owes nothing by construction.
    connection.execute(
        "INSERT INTO key_row
            (identifier, path, fingerprint, fingerprint_read_from, name,
             was_default_at_entry, store_subject)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
        rusqlite::params![
            row.identifier.as_text(),
            row.path.to_string_lossy(),
            row.fingerprint.as_ref().map(|read| read.value.as_str()),
            row.fingerprint.as_ref().map(|read| read_from_text(read.read_from)),
            row.name.as_ref().map(|name| name.as_str()),
            i64::from(row.was_default_at_entry),
            row.passphrase.as_ref().map(|held| held.subject.as_str()),
        ],
    )?;
    Ok(())
}

/// Writes a row that already exists — the new location's one field, the
/// replacement's four, and the name `Keys` R1 refreshes at a check.
///
/// **The identifier is never written**, which is what keeps a replacement's
/// stranding of the server rows true (`Keys` F11) and what makes a row's
/// identity something no act here can change.
pub fn write(connection: &rusqlite::Connection, row: &KeyRow) -> Result<(), RecordFailure> {
    connection.execute(
        "UPDATE key_row SET
            path = ?2, fingerprint = ?3, fingerprint_read_from = ?4, name = ?5,
            was_default_at_entry = ?6, store_subject = ?7
         WHERE identifier = ?1",
        rusqlite::params![
            row.identifier.as_text(),
            row.path.to_string_lossy(),
            row.fingerprint.as_ref().map(|read| read.value.as_str()),
            row.fingerprint.as_ref().map(|read| read_from_text(read.read_from)),
            row.name.as_ref().map(|name| name.as_str()),
            i64::from(row.was_default_at_entry),
            row.passphrase.as_ref().map(|held| held.subject.as_str()),
        ],
    )?;
    Ok(())
}

/// Removes the row — the last write of both removal forms (W8, W10).
pub fn remove(connection: &rusqlite::Connection, of: &Identity) -> Result<(), RecordFailure> {
    connection.execute("DELETE FROM key_row WHERE identifier = ?1", [of.as_text()])?;
    Ok(())
}

/// `Security and licence` §5.1's three writes over key rows.
///
/// **The three are what the order needs and nothing more.** Write 1 sets the
/// mark; `Security and licence` E3's middle case drops the reference and
/// **leaves the mark standing** so the sweep retries; write 3 clears the
/// reference **and the mark together, never the mark alone** (U9a).
pub struct KeyRowMarks<'a>(pub &'a rusqlite::Connection);

impl EraseMarks for KeyRowMarks<'_> {
    /// **Write 1, into the register and never into a row.** A row may be
    /// removed by write 3 or by a delete standing over it, and `Security and
    /// licence` E3 requires the mark to survive both.
    ///
    /// **Every stored kind is accepted here**, because the register is
    /// `Security and licence`'s rather than any module's rows — the three kinds
    /// that arrive at later steps need no table of their own and no second
    /// adapter.
    fn mark_owing_an_erase(&mut self, item: &HeldSecret) -> Result<(), EraseFailure> {
        self.0
            .execute(
                "INSERT OR IGNORE INTO owed_erase (kind, subject) VALUES (?1, ?2)",
                rusqlite::params![as_text(item.kind), item.subject.as_str()],
            )
            .map(|_| ())
            .map_err(|_| EraseFailure)
    }

    /// **`Security and licence` E3's middle case: the reference goes and the
    /// mark stands.** The register entry is deliberately untouched — it is
    /// what the sweep retries from, and it is the whole reason the register
    /// outlives the row.
    fn remove_reference(&mut self, item: &HeldSecret) -> Result<(), EraseFailure> {
        self.the_rows_reference(item)
    }

    /// **Write 3: the reference and the mark together, never the mark alone**
    /// (U9a). Where the row is already gone the reference clearing touches
    /// nothing, which is the state a delete leaves and is not a failure.
    fn clear_reference_and_mark(&mut self, item: &HeldSecret) -> Result<(), EraseFailure> {
        self.the_rows_reference(item)?;
        self.0
            .execute(
                "DELETE FROM owed_erase WHERE kind = ?1 AND subject = ?2",
                rusqlite::params![as_text(item.kind), item.subject.as_str()],
            )
            .map(|_| ())
            .map_err(|_| EraseFailure)
    }

    /// **Every subject currently owed an erase**, which is the sweep's whole
    /// input — and **it is the register rather than any row**, so a debt
    /// survives the removal of the row that carried the reference (§5.1 E3a).
    ///
    /// **All retained stored kinds live here.** The three that arrive with their
    /// own modules at later steps need no table and no second adapter, and
    /// **the absence of a row is still never a reason to erase**: the sweep
    /// acts on what is written here and on nothing else.
    fn marks(&self) -> Result<Vec<HeldSecret>, EraseFailure> {
        let mut statement = self
            .0
            .prepare("SELECT kind, subject FROM owed_erase ORDER BY rowid")
            .map_err(|_| EraseFailure)?;
        let rows = statement
            .query_map([], |row| {
                Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
            })
            .map_err(|_| EraseFailure)?;
        let mut marked = Vec::new();
        for row in rows {
            let (kind, subject) = row.map_err(|_| EraseFailure)?;
            // **A kind that will not read back is skipped rather than given an
            // invented one**, the same refusal `all` makes for an identifier.
            let Some(kind) = from_text(&kind) else {
                continue;
            };
            marked.push(HeldSecret::new(
                kind,
                Subject::issued_by_the_local_record(subject),
            ));
        }
        Ok(marked)
    }
}

impl KeyRowMarks<'_> {
    /// Clearing the row's own reference.
    ///
    /// **A row that is already gone is the state the caller asked for**, so no
    /// rows changed is not a failure here — which it was while the mark lived
    /// on the row, where a missing row meant a mark that never landed. The
    /// mark's own landing is now the register's insert, which answers for
    /// itself.
    fn the_rows_reference(&self, item: &HeldSecret) -> Result<(), EraseFailure> {
        self.0
            .execute(
                "UPDATE key_row SET store_subject = NULL WHERE store_subject = ?1",
                [item.subject.as_str()],
            )
            .map(|_| ())
            .map_err(|_| EraseFailure)
    }
}

/// The retained stored kinds, written down. **A word and never a number**, so the
/// register is a thing a person can read with any tool.
fn as_text(kind: StoredKind) -> &'static str {
    match kind {
        StoredKind::KeyPassphrase => "key passphrase",
        StoredKind::MachinePassword => "machine password",
        StoredKind::BotToken => "bot token",
        StoredKind::StoreProbe => "store probe",
    }
}

fn from_text(kind: &str) -> Option<StoredKind> {
    match kind {
        "key passphrase" => Some(StoredKind::KeyPassphrase),
        "machine password" => Some(StoredKind::MachinePassword),
        "bot token" => Some(StoredKind::BotToken),
        "store probe" => Some(StoredKind::StoreProbe),
        _ => None,
    }
}

/// Whether a subject is already issued against a row — the check `State,
/// identity and recovery` §2.2's *first store of a kind against a subject*
/// stands on.
pub fn row_holding(
    connection: &rusqlite::Connection,
    subject: &Subject,
) -> Result<Option<Identity>, RecordFailure> {
    let identifier: Option<String> = connection
        .query_row(
            "SELECT identifier FROM key_row WHERE store_subject = ?1",
            [subject.as_str()],
            |row| row.get(0),
        )
        .optional()?;
    Ok(identifier.and_then(|text| Identity::read_back(IdentityKind::LocalKey, &text)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::record::LocalRecord;
    use std::path::PathBuf;

    /// A record of its own per test, in the system's temporary directory —
    /// never Smoozy's own areas, which the application sweeps.
    fn a_record(name: &str) -> LocalRecord {
        let path = crate::scratch_folders::a_folder(format!("smoozy-key-rows-{name}-{}", std::process::id()));
        LocalRecord::open_at(&path.join("local-record.sqlite3")).expect("a record")
    }

    fn a_row(path: &str, fingerprint: Option<&str>) -> KeyRow {
        KeyRow::entering(
            Identity::issue(IdentityKind::LocalKey).expect("randomness"),
            PathBuf::from(path),
            fingerprint.map(|body| Reading {
                value: Fingerprint::from_the_printed_form(&format!("SHA256:{body}"))
                    .expect("a form"),
                read_from: ReadFrom::PrivateFile,
            }),
            Name::what_the_user_typed("work"),
            true,
        )
        .expect("a row")
    }

    #[test]
    fn a_row_comes_back_field_for_field() {
        let record = a_record("round-trip");
        let row = a_row("/k/.ssh/smoozy_ed25519", Some("one"));
        add(record.connection(), &row).expect("added");
        let back = all(record.connection()).expect("read");
        assert_eq!(back, vec![row]);
    }

    #[test]
    fn the_half_a_fingerprint_was_read_from_survives_the_round_trip() {
        let record = a_record("read-from");
        let mut row = a_row("/k/legacy", Some("one"));
        row.fingerprint.as_mut().expect("a reading").read_from = ReadFrom::PublicHalfBeside;
        add(record.connection(), &row).expect("added");
        let back = all(record.connection()).expect("read");
        assert_eq!(
            back[0].fingerprint.as_ref().expect("a reading").read_from,
            ReadFrom::PublicHalfBeside
        );
        assert!(!back[0]
            .fingerprint
            .as_ref()
            .expect("a reading")
            .may_be_compared_for_a_swap());
    }

    #[test]
    fn a_row_that_never_held_a_fingerprint_comes_back_holding_none() {
        let record = a_record("no-fingerprint");
        add(record.connection(), &a_row("/k/legacy", None)).expect("added");
        assert_eq!(all(record.connection()).expect("read")[0].fingerprint, None);
    }

    #[test]
    fn the_identifier_is_never_written_by_an_update() {
        let record = a_record("identifier");
        let mut row = a_row("/k/id_rsa", Some("one"));
        add(record.connection(), &row).expect("added");
        let identifier = row.identifier.clone();
        row.path = PathBuf::from("/elsewhere/id_rsa");
        write(record.connection(), &row).expect("written");
        let back = all(record.connection()).expect("read");
        assert_eq!(back[0].identifier, identifier);
        assert_eq!(back[0].path, PathBuf::from("/elsewhere/id_rsa"));
    }

    #[test]
    fn the_mark_and_the_reference_go_together_and_never_the_mark_alone() {
        let record = a_record("mark");
        let mut row = a_row("/k/id_rsa", Some("one"));
        row.passphrase = Some(PassphraseReference {
            subject: Subject::issued_by_the_local_record("s-1"),
            owing_an_erase: false,
        });
        add(record.connection(), &row).expect("added");

        let item = row.store_item().expect("an item");
        let mut marks = KeyRowMarks(record.connection());
        marks.mark_owing_an_erase(&item).expect("marked");
        assert_eq!(marks.marks().expect("marks"), vec![item.clone()]);
        assert!(all(record.connection()).expect("read")[0]
            .passphrase
            .as_ref()
            .expect("a reference")
            .owing_an_erase);

        marks.clear_reference_and_mark(&item).expect("cleared");
        assert!(marks.marks().expect("marks").is_empty());
        assert_eq!(all(record.connection()).expect("read")[0].passphrase, None);
    }

    #[test]
    fn a_store_failure_drops_the_reference_and_leaves_the_mark_for_the_sweep() {
        let record = a_record("sweep");
        let mut row = a_row("/k/id_rsa", Some("one"));
        row.passphrase = Some(PassphraseReference {
            subject: Subject::issued_by_the_local_record("s-2"),
            owing_an_erase: false,
        });
        add(record.connection(), &row).expect("added");

        let item = row.store_item().expect("an item");
        let mut marks = KeyRowMarks(record.connection());
        marks.mark_owing_an_erase(&item).expect("marked");
        marks.remove_reference(&item).expect("dropped");

        let back = all(record.connection()).expect("read");
        assert_eq!(back[0].passphrase, None);
        // **The mark stands in the register, carrying its own subject**, so
        // the sweep has something to retry against — which is `Security and
        // licence` E3's *the subject stays marked* kept for real. While the
        // mark was a column beside the reference it was left naming nothing.
        assert_eq!(marks.marks().expect("marks"), vec![item]);
    }

    #[test]
    /// **Every stored kind is marked here, and that changed on 2026-09-03.**
    /// While the mark was a column of the key row, a kind that was not a key
    /// passphrase had no row to mark and was refused. **The register is
    /// `Security and licence`'s rather than any module's rows**, so the three
    /// kinds arriving at later steps need no table and no second adapter — and
    /// each comes back under its own kind rather than as a key passphrase.
    fn every_stored_kind_is_marked_and_comes_back_as_itself() {
        let record = a_record("other-kind");
        let mut marks = KeyRowMarks(record.connection());
        let a_token = HeldSecret::new(
            StoredKind::BotToken,
            Subject::issued_by_the_local_record("bot"),
        );
        marks.mark_owing_an_erase(&a_token).expect("marked");
        assert_eq!(marks.marks().expect("marks"), vec![a_token]);
    }

    #[test]
    /// **A debt against a subject no row holds is still a debt**, and that is
    /// the whole change: the register does not ask whether a row is there,
    /// because the case it exists for is a row that has gone.
    fn a_subject_no_row_holds_is_still_marked_because_the_row_may_have_gone() {
        let record = a_record("nobody");
        let mut marks = KeyRowMarks(record.connection());
        let nobodys = HeldSecret::new(
            StoredKind::KeyPassphrase,
            Subject::issued_by_the_local_record("nobody"),
        );
        marks.mark_owing_an_erase(&nobodys).expect("marked");
        assert_eq!(marks.marks().expect("marks"), vec![nobodys]);
    }

    #[test]
    fn a_removed_row_leaves_nothing_behind() {
        let record = a_record("removed");
        let row = a_row("/k/id_rsa", Some("one"));
        add(record.connection(), &row).expect("added");
        remove(record.connection(), &row.identifier).expect("removed");
        assert!(all(record.connection()).expect("read").is_empty());
    }

    #[test]
    fn a_subject_names_the_row_that_holds_it() {
        let record = a_record("subject");
        let mut row = a_row("/k/id_rsa", Some("one"));
        row.passphrase = Some(PassphraseReference {
            subject: Subject::issued_by_the_local_record("s-3"),
            owing_an_erase: false,
        });
        add(record.connection(), &row).expect("added");
        assert_eq!(
            row_holding(
                record.connection(),
                &Subject::issued_by_the_local_record("s-3")
            )
            .expect("read"),
            Some(row.identifier)
        );
    }

    /// A store that is never reached, for the one test where `Security and
    /// licence` E1's first write refuses before any store call.
    pub struct NoStore;

    impl smoozy_security_and_licence::secret_store::SecretStore for NoStore {
        fn put(
            &self,
            _item: &HeldSecret,
            _value: smoozy_security_and_licence::plaintext::Plaintext,
            _deadline: std::time::Duration,
        ) -> Result<(), smoozy_security_and_licence::secret_store::StoreFailure> {
            unreachable!("the mark refused first")
        }
        fn get(
            &self,
            _item: &HeldSecret,
            _deadline: std::time::Duration,
        ) -> Result<
            smoozy_security_and_licence::plaintext::Plaintext,
            smoozy_security_and_licence::secret_store::GetFailure,
        > {
            unreachable!("the mark refused first")
        }
        fn erase(
            &self,
            _item: &HeldSecret,
            _deadline: std::time::Duration,
        ) -> Result<(), smoozy_security_and_licence::secret_store::StoreFailure> {
            unreachable!("the mark refused first")
        }
        fn probe(
            &self,
            _deadline: std::time::Duration,
        ) -> smoozy_security_and_licence::secret_store::ProbeOutcome {
            unreachable!("the mark refused first")
        }
        fn enumerate(
            &self,
            _deadline: std::time::Duration,
        ) -> Result<Vec<HeldSecret>, smoozy_security_and_licence::secret_store::StoreFailure>
        {
            unreachable!("the mark refused first")
        }
    }
}

#[cfg(test)]
mod the_owed_erase {
    //! **A finding, kept as a failing-shaped test rather than a paragraph.**
    //!
    //! `Security and licence` E3, at that document's §5.1, says a store
    //! failure never blocks a
    //! removal: *"the local record's row is removed regardless, the subject
    //! stays marked as owing an erase, and the sweep retries it."* `Security
    //! and licence` E4 has the sweep act on those marks and on nothing else.
    //!
    //! **`Keys` U9a makes the mark and the reference two columns of one row**,
    //! for its own good reason — a reference standing with its mark cleared
    //! would make U9 answer *yes* for an item that is gone.
    //!
    //! **The two could not both hold while the mark was a column of the key
    //! row.** These tests were written to demonstrate that, in both
    //! directions; **§5.1 E3a moved the mark into a register of its own on
    //! 2026-09-03**, and they now hold it there.

    use super::*;
    use smoozy_security_and_licence::erasure::{self, Erasure};
    use smoozy_security_and_licence::plaintext::Plaintext;
    use smoozy_security_and_licence::secret_store::{
        GetFailure, ProbeOutcome, SecretStore, StoreFailure,
    };
    use smoozy_state::identities::{Identity, IdentityKind};
    use std::time::Duration;

    struct ARefusingStore;

    impl SecretStore for ARefusingStore {
        fn put(
            &self,
            _item: &HeldSecret,
            _value: Plaintext,
            _deadline: Duration,
        ) -> Result<(), StoreFailure> {
            Ok(())
        }
        fn get(&self, _item: &HeldSecret, _deadline: Duration) -> Result<Plaintext, GetFailure> {
            Err(GetFailure::NotThere)
        }
        fn erase(&self, _item: &HeldSecret, _deadline: Duration) -> Result<(), StoreFailure> {
            Err(StoreFailure::StoreUnavailable)
        }
        fn probe(&self, _deadline: Duration) -> ProbeOutcome {
            ProbeOutcome::RoundTripWorked
        }
        fn enumerate(&self, _deadline: Duration) -> Result<Vec<HeldSecret>, StoreFailure> {
            Ok(Vec::new())
        }
    }

    fn a_record_holding_a_key_with_a_passphrase() -> (rusqlite::Connection, KeyRow) {
        let connection = rusqlite::Connection::open_in_memory().expect("a record");
        for migration in super::super::migrations::ALL {
            super::super::migrations::run_one_for_tests(&connection, migration)
                .expect("a migration");
        }
        let mut row = KeyRow::entering(
            Identity::issue(IdentityKind::LocalKey).expect("randomness"),
            std::path::PathBuf::from("/k/.ssh/smoozy_ed25519"),
            None,
            None,
            false,
        )
        .expect("a row");
        row.passphrase = Some(PassphraseReference {
            subject: Subject::issued_by_the_local_record("s-1"),
            owing_an_erase: false,
        });
        add(&connection, &row).expect("the row");
        (connection, row)
    }

    /// **The finding, and now the repair.** The store refuses the erase, so
    /// §5.1's middle case runs. **Afterwards the sweep's input still holds the
    /// subject**, which is what `Security and licence` E3's *the subject stays
    /// marked* asks for and what a mark beside the reference could not keep.
    #[test]
    fn a_refused_erase_leaves_a_debt_the_sweep_can_still_see() {
        let (connection, row) = a_record_holding_a_key_with_a_passphrase();
        let item = row.store_item().expect("an item");
        let mut marks = KeyRowMarks(&connection);

        let erasure = erasure::erase(&mut marks, &ARefusingStore, &item, Duration::from_secs(2));
        assert!(matches!(
            erasure,
            Erasure::ReferenceGoneAndOwedToTheSweep(_)
        ));

        // The row's reference is gone, as `Security and licence` E3's middle
        // case has it…
        assert_eq!(all(&connection).unwrap()[0].passphrase, None);

        // …and the debt stands in the register, carrying its own subject, so
        // the sweep has something to retry against.
        assert_eq!(
            marks.marks().expect("the marks"),
            vec![item],
            "`Security and licence` E3's owed erase is the register's, and it \
             carries its own subject"
        );
    }

    /// **And the delete path keeps it**, which is the direction that had no
    /// answer at all while the mark was a column of the row delete removes.
    #[test]
    fn removing_the_row_leaves_the_debt_standing() {
        let (connection, row) = a_record_holding_a_key_with_a_passphrase();
        let item = row.store_item().expect("an item");
        let mut marks = KeyRowMarks(&connection);
        let _ = erasure::erase(&mut marks, &ARefusingStore, &item, Duration::from_secs(2));

        remove(&connection, &row.identifier).expect("the row goes");
        assert_eq!(
            marks.marks().expect("the marks"),
            vec![item],
            "`Security and licence` E3 has the row removed and the subject stay \
             marked, and the register is what lets both be true"
        );
    }
}

/// The shipped secret-store item's opaque subject, retained for erasure only.
pub const LEGACY_LICENCE_KEY_SUBJECT: &str = "licence-record";

/// A settled migration never creates this debt a second time. An unavailable
/// store leaves it for a later launch; other kinds are never handed over.
pub fn retry_the_legacy_licence_erases(
    connection: &rusqlite::Connection,
    store: &dyn smoozy_security_and_licence::secret_store::SecretStore,
) {
    let Ok(mut query) = connection.prepare("SELECT subject FROM owed_erase WHERE kind = 'licence key'") else { return; };
    let Ok(rows) = query.query_map([], |row| row.get::<_, String>(0)) else { return; };
    let Ok(subjects) = rows.collect::<Result<Vec<_>, _>>() else { return; };
    drop(query);
    for subject in subjects {
        let subject = Subject::issued_by_the_local_record(subject);
        if store.erase_legacy_licence_key(&subject, std::time::Duration::from_secs(10)).is_ok() {
            let _ = connection.execute(
                "DELETE FROM owed_erase WHERE kind = 'licence key' AND subject = ?1",
                [subject.as_str()],
            );
        }
    }
}

pub fn with_legacy_licence_erases_owed(connection: &rusqlite::Connection, retry: impl FnOnce()) {
    if connection.query_row("SELECT EXISTS(SELECT 1 FROM owed_erase WHERE kind = 'licence key')", [], |row| row.get::<_, bool>(0)).unwrap_or(false) {
        retry();
    }
}
