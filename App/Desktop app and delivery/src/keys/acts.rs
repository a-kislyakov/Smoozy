//! The acts that touch this computer — `Keys` §5.3, §5.5, §9.5, §9.6 and §9.7.
//!
//! **Making a key is next door** at [`super::making`], because it is the one
//! act with a tool run in it. What is here is everything else the user presses:
//! pointing at a key, mending an unusable one in its two forms, copying the
//! public half, deleting a managed key, and removing one from Smoozy.
//!
//! **`Keys` decides and this file acts.** Every judgement is a call into
//! `smoozy_keys`; what is here is the readings, the file adapter, the local
//! record and the store.
//!
//! **Every local write and deletion goes through the platform file adapter**
//! (`Keys` §12, `Desktop app and delivery` F10a). **The readings do not**,
//! and that is the plan's own split: F10a is a rule about writes, and the
//! readings are `Keys` §4.3's,
//! taken by [`super::readings`].
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
//! it belongs to.** `Keys` U4, U6, U7, U7b, U9, U10, W7a, W8, W9, W10.
//! `Reporting a problem` J2a.
//!
//! **A marker this crate's own plan also defines is deliberately not listed
//! above, whichever document is meant, and is written out with its document
//! at every site instead** — the header list cannot tell the two apart, and a
//! bare one would resolve to a real and wrong clause.
//!
//! *(`Keys` W3 was on this list until 2026-09-04, when the one site citing it
//! turned out to mean `Keys` W6 and was corrected.)* **That the document is
//! written out includes the F11 inside a test's own assertion message**,
//! written out there on 2026-09-04 so the message a failing test prints names
//! the document as well as the marker.

use std::path::{Path, PathBuf};
use std::time::Duration;

use smoozy_keys::diary::{AKeyPointedAt, AKeyRemoved, Act};
use smoozy_keys::fingerprint::{Fingerprint, PublicKeyBlob, ReadFrom};
use smoozy_keys::locations;
use smoozy_keys::naming;
use smoozy_keys::ownership::FileSet;
use smoozy_keys::pointing::{self, Form, Offered, Picked};
use smoozy_keys::removal::{self, Copy, Deleted, Unlinks};
use smoozy_keys::rows::KeyRow;
use smoozy_keys::usable::{self, Checked, Readings, ToolAnswer};
use smoozy_security_and_licence::erasure::Erasure;
use smoozy_security_and_licence::secret_store::SecretStore;
use smoozy_state::identities::{Identity, IdentityKind};

use super::readings::OnThisComputer;
use crate::platform::files;
use crate::record::key_rows;

/// The file adapter, as `Keys`' delete asks for it.
///
/// **`Desktop app and delivery` F1's discipline: gone is the outcome**,
/// established by re-reading the path after the unlink and never by the call's
/// return value. Written out in full because this file's header names `Keys`,
/// and a bare `F1` here would read as that document's.
pub struct TheFileAdapter;

impl Unlinks for TheFileAdapter {
    fn unlink_and_recheck(&self, path: &Path) -> Result<bool, String> {
        let was_there = !matches!(
            files::outside_program_wrote(path),
            files::Recheck::Absent
        );
        if let Err(failure) = files::remove_file(path) {
            // **What the system reported, in its own words** — never the
            // adapter's own class written out the way this language renders a
            // value for a programmer, which is what the rest of this match
            // already does with `CouldNotLook`.
            return Err(failure.reported);
        }
        match files::outside_program_wrote(path) {
            files::Recheck::Absent => Ok(was_there),
            // Anything still at the path is the file left behind, whatever its
            // length reads as.
            files::Recheck::Present { .. } | files::Recheck::ShortOrDifferent { .. } => {
                Err("the file is still there".to_string())
            }
            files::Recheck::CouldNotLook { reported } => Err(reported),
        }
    }
}

/// **`Desktop app and delivery` K13: whatever the picker returns is a path
/// from the user, judged like any other name** — so a picked file is read by U10's one function, over a row
/// standing for it.
///
/// **The identity is issued before the judgement and is simply never used where
/// the point is refused.** §2.1 rule 1 forbids reusing one, not discarding one:
/// a refused point leaves an identity nothing ever named.
pub fn read_a_picked_file(
    path: &Path,
    temporary_area: &Path,
) -> Option<(Identity, Picked, Checked)> {
    let identifier = Identity::issue(IdentityKind::LocalKey).ok()?;
    // The recorded path is canonical and resolved at entry (`Keys` L3), which
    // is what ownership is derived from later. **The adapter owns what
    // canonical means on each system**, which is where Windows'
    // extended-length prefix is dealt with.
    let path = files::canonical(path).unwrap_or_else(|| path.to_path_buf());
    let standing_for = KeyRow::entering(identifier.clone(), path.clone(), None, None, false)?;
    let checked = usable::check(
        &standing_for,
        &OnThisComputer::over(temporary_area.to_path_buf()),
    );
    let picked = Picked {
        path,
        verdict: checked.verdict.clone(),
        why_it_would_not_read: checked.why_it_would_not_read,
        fingerprint: checked.fingerprint_now.clone(),
        blob: the_blob_beside(&standing_for.path),
        name: checked.name_now.clone(),
    };
    Some((identifier, picked, checked))
}

/// **`Keys` F5's second reading**, taken only where a fingerprint already
/// matched.
///
/// The blob is the public half's own bytes as the `.pub` beside the key carries
/// them. **Where there is no `.pub` there is no blob and the comparison cannot
/// run** — the refusal then stands on the fingerprint alone, which is §13 item
/// 1's stated residual.
pub fn the_blob_beside(private_half: &Path) -> Option<PublicKeyBlob> {
    let beside = locations::companions(private_half).public_half;
    let contents = std::fs::read(&beside).ok()?;
    // The `.pub` is `<type> <base64 blob> <comment>`; the blob is the middle
    // field, and it is compared as the bytes it is written as.
    let middle = contents
        .split(|byte| *byte == b' ')
        .nth(1)
        .map(|blob| blob.to_vec())?;
    Some(PublicKeyBlob(middle))
}

/// **`Keys` F9: a first point is one partition unit — the row creation**
/// (§11), so it writes no outcome record. Atomic by the record's single write;
/// a refusal leaves no trace.
///
/// **The record is lent for the read and the write and is never held across the
/// `ssh-keygen` reading of the picked file** — step 10a, the screens round's
/// loose ends, L4.
pub fn point_at_a_key(
    picked_path: &Path,
    the_default_name_now: &str,
    record_lender: &dyn super::LendsTheRecord,
    temporary_area: &Path,
) -> Result<KeyRow, pointing::Refused> {
    // **The tool run first, with nothing lent.**
    let Some((identifier, picked, checked)) = read_a_picked_file(picked_path, temporary_area) else {
        return Err(pointing::Refused::TheClauseThatFailed {
            clause: smoozy_keys::usable::Unusable::Unreadable,
            situation: None,
        });
    };
    let mut rows = Vec::new();
    record_lender.lend(&mut |connection| {
        rows = key_rows::all(connection).unwrap_or_default();
    });

    pointing::judge(&picked, &Offered::AFirstPoint, &rows, &|row| {
        the_blob_beside(&row.path)
    })?;

    let name = checked.name_now.clone();
    let row = KeyRow::entering(
        identifier,
        picked.path.clone(),
        checked.fingerprint_now,
        name.clone(),
        naming::was_default_at_entry(name.as_ref(), the_default_name_now),
    )
    .ok_or(pointing::Refused::NotThisKey)?;
    let mut written = false;
    record_lender.lend(&mut |connection| {
        written = key_rows::add(connection, &row).is_ok();
    });
    if !written {
        return Err(pointing::Refused::NotThisKey);
    }

    // **§12 act two, first form: a key pointed at.** Written **at the row
    // write**, which is this act's single partition unit (§11). **A refused
    // point writes nothing** — `Keys` F4's refusal and U6's failed-clause
    // statement are sentences, not completed acts, and every one of them
    // returns above.
    record_lender.write_a_diary_line(
        &Act::AKeyPointedAt(AKeyPointedAt {
            form: Form::AKeyKeptElsewhere,
            the_path_the_row_took: row.path.clone(),
            name: row.name.clone(),
            // **The erasure field is the replacement's alone.** A first point
            // erases nothing, so there is nothing to state.
            the_erasure: None,
        }),
    );
    Ok(row)
}

/// One row write, against a record lent for that write alone.
fn the_row_is_written(
    record_lender: &dyn super::LendsTheRecord,
    row: &KeyRow,
) -> Result<(), pointing::Refused> {
    let mut written = false;
    record_lender.lend(&mut |connection| {
        written = key_rows::write(connection, row).is_ok();
    });
    written.then_some(()).ok_or(pointing::Refused::NotThisKey)
}

/// What mending a row came to.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Mended {
    /// **`Keys` F9: one write — the row's recorded path.** The fingerprint
    /// does not change, so the keying, the server rows' match value and the
    /// *Used by* line are untouched.
    ItsNewLocation,
    /// **`Keys` F9 and `Keys` F10: the erasure and the row's four fields, two writes
    /// taken as one ordered unit.**
    AReplacement { erasure: Erasure },
}

/// **The two mending acts, §5.3 and §5.5.** Which control was pressed is named
/// by the caller rather than inferred, and `Keys` F6 then settles which act a
/// pointed file may actually take.
///
/// **Neither owes an outcome record**: a new location is one unit, and a
/// replacement's two writes are one ordered unit (§11).
///
/// **The row is named by its own identity and by nothing else, and there is one
/// identifier rather than two.** This took a position in `key_rows::all` — a
/// list ordered by insertion — while [`Offered`] carried a **second** position
/// into the same list for `judge` to exclude. Two indices for one row, which
/// could disagree with each other, and neither of which is the key. The
/// neighbouring acts here already take the row itself, so this was inconsistent
/// inside its own file. **A caller naming a row that is not there is refused**,
/// where an index answered from whatever happened to sit at that position.
///
/// **The record is lent for each read and each write and is never held across
/// the `ssh-keygen` reading of the picked file** — step 10a, the screens
/// round's loose ends, L4.
#[allow(clippy::too_many_arguments)]
pub fn point_a_row_at(
    picked_path: &Path,
    offered: &Offered,
    the_default_name_now: &str,
    record_lender: &dyn super::LendsTheRecord,
    store: &dyn SecretStore,
    deadline: Duration,
    temporary_area: &Path,
) -> Result<Mended, pointing::Refused> {
    let Some(mending) = offered.the_row_being_mended() else {
        // A first point is not this act's, and running it here would be this
        // function writing a row it was not asked to write.
        return Err(pointing::Refused::NotThisKey);
    };
    // **The tool run first, with nothing lent.**
    let Some((_identifier, picked, checked)) = read_a_picked_file(picked_path, temporary_area) else {
        return Err(pointing::Refused::TheClauseThatFailed {
            clause: smoozy_keys::usable::Unusable::Unreadable,
            situation: None,
        });
    };
    let mut rows = Vec::new();
    record_lender.lend(&mut |connection| {
        rows = key_rows::all(connection).unwrap_or_default();
    });
    let Some(at) = rows.iter().position(|row| row.identifier == *mending) else {
        return Err(pointing::Refused::NotThisKey);
    };
    let mut marks = super::TheMarksLent(record_lender);

    let form = pointing::judge(&picked, offered, &rows, &|row| the_blob_beside(&row.path))?;

    match form {
        Form::AnUnusableKeysNewLocation => {
            pointing::take_the_new_location(&mut rows[at], picked.path.clone());
            the_row_is_written(record_lender, &rows[at])?;
            // **§12 act two, second form**, at the row write — this act's own
            // single partition unit. The erasure field is absent: a new
            // location erases nothing.
            record_lender.write_a_diary_line(&Act::AKeyPointedAt(AKeyPointedAt {
                form,
                the_path_the_row_took: rows[at].path.clone(),
                name: rows[at].name.clone(),
                the_erasure: None,
            }));
            Ok(Mended::ItsNewLocation)
        }
        Form::AReplacement => {
            // **The erasure runs first**, because §5.1's own first write is
            // what closes the window: marking the subject as owing an erase
            // turns U9's availability answer to *no* at that instant.
            let the_row_carried_a_reference = rows[at].passphrase.is_some();
            let erasure = removal::remove_from_smoozy(&rows[at], &mut marks, store, deadline);
            pointing::take_the_replacement(
                &mut rows[at],
                picked.path.clone(),
                checked.fingerprint_now,
                checked.name_now,
                the_default_name_now,
            );
            the_row_is_written(record_lender, &rows[at])?;
            // **§12 act two, third form**, at the second write of this act's
            // ordered unit. **For the replacement alone, and only where the
            // row carried a secret-store subject reference**, the line says
            // whether the erasure was confirmed or left as §5.1's recorded
            // debt — the one thing about this act nothing afterwards would
            // ever show (`Keys` F10). Where the row carried none the field is
            // absent.
            record_lender.write_a_diary_line(&Act::AKeyPointedAt(AKeyPointedAt {
                form,
                the_path_the_row_took: rows[at].path.clone(),
                name: rows[at].name.clone(),
                the_erasure: super::how_the_erasure_ended(the_row_carried_a_reference, &erasure),
            }));
            Ok(Mended::AReplacement { erasure })
        }
        // A first point cannot come out of a mending offer, and if it did it
        // would be this function writing a row it was not asked to write.
        Form::AKeyKeptElsewhere => Err(pointing::Refused::NotThisKey),
    }
}

/// **`Keys` W7: copy reads `path.pub` and only that**, and W7a compares the
/// two fingerprints before the contents reach the clipboard.
///
/// **Nothing new is read for the comparison**: the private half's own
/// fingerprint is U4 clause 4a's reading and the public half's is U7's.
pub fn copy_the_public_half(row: &KeyRow, temporary_area: &Path) -> Copy {
    let readings = OnThisComputer::over(temporary_area.to_path_buf());
    let beside = locations::companions(&row.path).public_half;
    let contents = std::fs::read_to_string(&beside).ok();
    // **U7b is what makes this comparison mean anything.** The private half's
    // own value is derived from the private key; the public half's is read off
    // the `.pub`. Until 2026-09-03 both were `-l -f <path>`, which answers from
    // the `.pub` either way — so W7a was comparing that file against itself.
    removal::copy(
        contents,
        the_value_of(readings.the_private_halfs_own_fingerprint_line(&row.path), ReadFrom::PrivateFile)
            .as_ref(),
        the_value_of(
            readings.the_public_halfs_fingerprint_line(&beside),
            ReadFrom::PublicHalfBeside,
        )
        .as_ref(),
    )
}

fn the_value_of(answer: ToolAnswer, from: ReadFrom) -> Option<Fingerprint> {
    let ToolAnswer::Printed(line) = answer else {
        return None;
    };
    usable::parse_the_tools_line(&line, from).map(|said| said.reading.value)
}

/// **W8: delete a managed key**, performed.
///
/// **`Keys` W2 re-derives ownership at the act's own moment** rather than
/// trusting the control that called it, and the file set is taken at this
/// moment too, so the confirmation named what is really there.
///
/// **The record is lent for each read and each write and is never held across
/// the file work** — step 10a, the screens round's loose ends, L4.
pub fn delete_a_managed_key(
    row: &KeyRow,
    user_home: &Path,
    record_lender: &dyn super::LendsTheRecord,
    store: &dyn SecretStore,
    deadline: Duration,
) -> Result<Deleted, removal::NotRun> {
    let mut records = super::TheOutcomeRecordsLent(record_lender);
    let mut marks = super::TheMarksLent(record_lender);
    // **`Keys` W1's derivation takes the home directory**, and composes the
    // usual key location itself — handing it the location instead would apply
    // `.ssh` twice and call every managed key borrowed.
    let ownership = smoozy_keys::ownership::ownership(&row.path, user_home, &Canonical);
    let set = the_file_set_now(&row.path);

    let deleted = removal::delete(
        row,
        ownership,
        &set,
        &TheFileAdapter,
        &mut records,
        &mut marks,
        store,
        deadline,
    )?;
    if deleted.the_row_is_removed {
        record_lender.lend(&mut |connection| {
            let _ = key_rows::remove(connection, &row.identifier);
        });
        // **§12 act three, first form: a managed key deleted.** Written **at
        // the local forget** — the erasure unit's removal of the row, which is
        // the last unit of both forms — so the line records the act completed
        // and never the act asked for. **A delete whose private half's unit
        // failed writes no line at all** (W9): `the_row_is_removed` is false
        // there, the row stays, the key stays listed, and nothing claims it is
        // gone.
        record_lender.write_a_diary_line(
            &Act::AKeyRemoved(AKeyRemoved {
                // **Taken from `Keys` W2's derivation at the act's own
                // moment**, which is the one the act itself re-derived above.
                a_managed_key_was_deleted: ownership == smoozy_keys::ownership::Ownership::Managed,
                file_units: the_file_units(&deleted),
                the_path_the_row_held: row.path.clone(),
                name: row.name.clone(),
                the_erasure: deleted.erasure.as_ref().and_then(|erasure| {
                    super::how_the_erasure_ended(row.passphrase.is_some(), erasure)
                }),
            }),
        );
    }
    Ok(deleted)
}

/// **`Keys` W6's file units by role and how each ended**, for §12 act three's
/// line.
///
/// **These are the very outcomes this act wrote into `State, identity and
/// recovery` §9.4's record a moment ago**, held in hand rather than read back
/// through a second query of the record the act just wrote — which is what §12
/// means by *never composed afresh*: nothing here re-reads the filesystem, so a
/// companion left behind by a failed unit is what the line names, exactly as
/// the record holds it.
fn the_file_units(deleted: &Deleted) -> Vec<(smoozy_keys::units::UnitRole, removal::FileUnit)> {
    use smoozy_keys::units::UnitRole;
    let mut units = vec![(UnitRole::ThePrivateHalf, deleted.private_half.clone())];
    if let Some(ended) = &deleted.public_half {
        units.push((UnitRole::ThePublicHalf, ended.clone()));
    }
    if let Some(ended) = &deleted.certificate {
        units.push((UnitRole::TheCertificate, ended.clone()));
    }
    units
}

/// **W10: remove from Smoozy is the erasure unit alone, and no file is
/// touched, ever.** It is one unit and writes no outcome record.
pub fn remove_from_smoozy(
    row: &KeyRow,
    record_lender: &dyn super::LendsTheRecord,
    store: &dyn SecretStore,
    deadline: Duration,
) -> Erasure {
    let the_row_carried_a_reference = row.passphrase.is_some();
    let mut marks = super::TheMarksLent(record_lender);
    let erasure = removal::remove_from_smoozy(row, &mut marks, store, deadline);
    // **A store failure never blocks the removal** (§5.1): the row goes, the
    // mark stays, and the sweep retries.
    record_lender.lend(&mut |connection| {
        let _ = key_rows::remove(connection, &row.identifier);
    });
    // **§12 act three, second form: a referenced key removed.** Written at the
    // local forget, which for this form is the whole act. **No file unit is
    // named**, W10 touching no file at all.
    record_lender.write_a_diary_line(
        &Act::AKeyRemoved(AKeyRemoved {
            a_managed_key_was_deleted: false,
            file_units: Vec::new(),
            the_path_the_row_held: row.path.clone(),
            name: row.name.clone(),
            the_erasure: super::how_the_erasure_ended(the_row_carried_a_reference, &erasure),
        }),
    );
    erasure
}

/// **`Keys` S3 item 4: the user clearing a remembered passphrase, performed.**
///
/// **The row stays.** That is the whole of the difference from
/// [`remove_from_smoozy`], which runs the same §5.1 erasure and then drops the
/// row — so it was never a stand-in for this act, however alike the two look
/// from the store's end. No file is touched, and nothing else on the row moves.
///
/// **It is one partition unit and writes no outcome record** (§11), and **no
/// diary line** — §12 declines it on J2a's second test, the spec naming its
/// completion as *"the next connection asks for it fresh"*, which is a step
/// inside a later act rather than something the user is told about.
pub fn clear_a_remembered_passphrase(
    row: &KeyRow,
    marks: &mut dyn smoozy_security_and_licence::erasure::EraseMarks,
    store: &dyn SecretStore,
    deadline: Duration,
) -> Erasure {
    removal::clear_a_remembered_passphrase(row, marks, store, deadline)
}

/// **`Keys` W6: delete's file set is exactly the members of
/// `{path, path.pub, path-cert.pub}` that exist at the moment of the act, and
/// the confirmation names exactly those, by full path.** Nothing wider and
/// nothing narrower — the private half is in the set always.
///
/// **The marker was `Keys` W3 until 2026-09-04 and it was wrong.**
/// `Keys` W3 is that
/// document's ownership rule — *location decides, not the route in* — and this
/// function derives no ownership; it takes a path already fixed and reports
/// which of the three files are there. **The two companion names it composes
/// are `Keys` W5's rule, kept in that crate at `smoozy_keys::locations::companions`**,
/// so nothing of `Keys` W5 is kept here.
pub fn the_file_set_now(private_half: &Path) -> FileSet {
    let beside = locations::companions(private_half);
    FileSet::at_this_moment(
        private_half,
        beside.public_half.exists(),
        beside.certificate.exists(),
    )
}

/// The canonical-path reading ownership is derived from (§9.1).
pub struct Canonical;

impl smoozy_keys::ownership::CanonicalPaths for Canonical {
    fn canonical(&self, path: &Path) -> Option<PathBuf> {
        files::canonical(path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use smoozy_keys::types::KeyType;
    use smoozy_security_and_licence::plaintext::Plaintext;
    use smoozy_security_and_licence::secret_store::{GetFailure, ProbeOutcome, StoreFailure};

    fn scratch(name: &str) -> PathBuf {
        let mut path = std::env::temp_dir();
        path.push(format!("smoozy-acts-{name}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&path);
        std::fs::create_dir_all(&path).expect("a scratch directory");
        path
    }

    /// **`Keys` L3: the recorded path is canonical**, so a test comparing
    /// paths compares that form and never the one it handed the tool.
    ///
    /// **The scratch area itself is left exactly as the operating system gave
    /// it.** On this Mac the canonical form differs by a symbolic link, and on
    /// Windows it carries the extended-length prefix — which `ssh-keygen` is
    /// not this project's to assume anything about. **What Smoozy canonicalises
    /// is the path it records; what it hands the tool is the path it was
    /// given.**
    fn as_recorded(path: &Path) -> PathBuf {
        // **The adapter's own answer and never the raw system call**, because
        // what *canonical* means differs per system and the adapter is where
        // that is settled — on Windows it is the system call with the
        // extended-length prefix taken off a plain drive path.
        files::canonical(path).unwrap_or_else(|| path.to_path_buf())
    }

    fn there_is_a_generator() -> bool {
        !matches!(super::super::readings::run(&["-h".to_string()]), ToolAnswer::NoGenerator)
    }

    fn a_record() -> rusqlite::Connection {
        let connection = rusqlite::Connection::open_in_memory().expect("a record");
        for migration in crate::record::migrations::ALL {
            crate::record::migrations::run_one_for_tests(&connection, migration)
                .expect("a migration");
        }
        connection
    }

    struct NoStore;

    impl SecretStore for NoStore {
        fn put(
            &self,
            _item: &smoozy_security_and_licence::held::HeldSecret,
            _value: Plaintext,
            _deadline: Duration,
        ) -> Result<(), StoreFailure> {
            Ok(())
        }
        fn get(
            &self,
            _item: &smoozy_security_and_licence::held::HeldSecret,
            _deadline: Duration,
        ) -> Result<Plaintext, GetFailure> {
            Err(GetFailure::NotThere)
        }
        fn erase(
            &self,
            _item: &smoozy_security_and_licence::held::HeldSecret,
            _deadline: Duration,
        ) -> Result<(), StoreFailure> {
            Ok(())
        }
        fn probe(&self, _deadline: Duration) -> ProbeOutcome {
            ProbeOutcome::RoundTripWorked
        }
        fn enumerate(
            &self,
            _deadline: Duration,
        ) -> Result<Vec<smoozy_security_and_licence::held::HeldSecret>, StoreFailure> {
            Ok(Vec::new())
        }
    }

    /// Makes a real key pair with the real tool, outside any Smoozy location.
    fn a_real_key(at: &Path, name: &str) -> PathBuf {
        let path = at.join(name);
        let printed = super::super::readings::run(&[
            "-t".to_string(),
            KeyType::Ed25519.as_str().to_string(),
            "-C".to_string(),
            "someone@their-mac".to_string(),
            "-N".to_string(),
            String::new(),
            "-f".to_string(),
            path.display().to_string(),
        ]);
        assert!(matches!(printed, ToolAnswer::Printed(_)), "the tool made a key");
        path
    }

    #[test]
    fn a_key_kept_elsewhere_is_pointed_at_and_rowed_once() {
        if !there_is_a_generator() {
            return;
        }
        let area = scratch("point");
        let path = a_real_key(&area, "elsewhere");
        let connection = a_record();

        let row = point_at_a_key(&path, "someone@their-mac", &crate::keys::TheRecordItself::over(&connection), &area).expect("a point");
        assert_eq!(row.fingerprint.as_ref().unwrap().read_from, ReadFrom::PrivateFile);
        assert!(row.was_default_at_entry);
        assert_eq!(key_rows::all(&connection).unwrap().len(), 1);

        // **`Keys` F4: the same key a row already lists is refused, naming
        // it**, and nothing is written a second time.
        let again = point_at_a_key(&path, "someone@their-mac", &crate::keys::TheRecordItself::over(&connection), &area);
        assert!(matches!(
            again,
            Err(pointing::Refused::ThisKeyIsAlreadyListed { .. })
        ));
        assert_eq!(key_rows::all(&connection).unwrap().len(), 1);
        let _ = std::fs::remove_dir_all(&area);
    }

    #[test]
    fn an_unusable_row_takes_its_new_location_and_nothing_else_moves() {
        if !there_is_a_generator() {
            return;
        }
        let area = scratch("newlocation");
        let path = a_real_key(&area, "moved");
        let connection = a_record();
        let row = point_at_a_key(&path, "someone@their-mac", &crate::keys::TheRecordItself::over(&connection), &area).expect("a point");
        let fingerprint_before = row.fingerprint.clone();

        // The user moves the file. The row is now unusable at its old path.
        let moved = area.join("moved_elsewhere");
        std::fs::rename(&path, &moved).expect("moved");
        std::fs::rename(
            locations::companions(&path).public_half,
            locations::companions(&moved).public_half,
        )
        .expect("its public half too");

        let mended = point_a_row_at(
            &moved,
            &Offered::ANewLocationFor(row.identifier.clone()),
            "someone@their-mac",
            &crate::keys::TheRecordItself::over(&connection),
            &NoStore,
            Duration::from_secs(2),
            &area,
        )
        .expect("a mend");

        assert_eq!(mended, Mended::ItsNewLocation);
        let after = &key_rows::all(&connection).unwrap()[0];
        assert_eq!(after.path, as_recorded(&moved));
        assert_eq!(after.fingerprint, fingerprint_before, "one write, the path");
        assert_eq!(after.identifier, row.identifier);
        let _ = std::fs::remove_dir_all(&area);
    }

    #[test]
    fn a_different_key_offered_as_a_new_location_is_refused_as_not_this_key() {
        if !there_is_a_generator() {
            return;
        }
        let area = scratch("notthiskey");
        let one = a_real_key(&area, "one");
        let other = a_real_key(&area, "other");
        let connection = a_record();
        let one_row =
            point_at_a_key(&one, "someone@their-mac", &crate::keys::TheRecordItself::over(&connection), &area).expect("a point");

        let refused = point_a_row_at(
            &other,
            &Offered::ANewLocationFor(one_row.identifier.clone()),
            "someone@their-mac",
            &crate::keys::TheRecordItself::over(&connection),
            &NoStore,
            Duration::from_secs(2),
            &area,
        );
        assert_eq!(refused, Err(pointing::Refused::NotThisKey));
        assert_eq!(key_rows::all(&connection).unwrap()[0].path, as_recorded(&one));
        let _ = std::fs::remove_dir_all(&area);
    }

    #[test]
    fn a_replacement_writes_four_fields_and_keeps_the_identifier() {
        if !there_is_a_generator() {
            return;
        }
        let area = scratch("replacement");
        let one = a_real_key(&area, "gone");
        let other = a_real_key(&area, "instead");
        let connection = a_record();
        let before = point_at_a_key(&one, "someone@their-mac", &crate::keys::TheRecordItself::over(&connection), &area).expect("a point");

        let mended = point_a_row_at(
            &other,
            &Offered::AReplacementFor(before.identifier.clone()),
            "somebody@else",
            &crate::keys::TheRecordItself::over(&connection),
            &NoStore,
            Duration::from_secs(2),
            &area,
        )
        .expect("a mend");
        assert!(matches!(mended, Mended::AReplacement { .. }));

        let after = &key_rows::all(&connection).unwrap()[0];
        assert_eq!(
            after.identifier,
            before.identifier,
            "`Keys` F11: the identifier stands"
        );
        assert_eq!(after.path, as_recorded(&other));
        assert_ne!(after.fingerprint, before.fingerprint);
        assert!(!after.was_default_at_entry, "the default string has moved on");
        let _ = std::fs::remove_dir_all(&area);
    }

    /// **`Keys` W7 and W7a**, over the real files.
    #[test]
    fn copy_hands_over_the_public_half_and_refuses_a_mismatched_one() {
        if !there_is_a_generator() {
            return;
        }
        let area = scratch("copy");
        let mine = a_real_key(&area, "mine");
        let other = a_real_key(&area, "other");
        let connection = a_record();
        let row = point_at_a_key(&mine, "someone@their-mac", &crate::keys::TheRecordItself::over(&connection), &area).expect("a point");

        let copied = copy_the_public_half(&row, &area);
        let contents = std::fs::read_to_string(locations::companions(&mine).public_half).unwrap();
        assert_eq!(copied, Copy::ToTheClipboard(contents));

        // **W7a: the `.pub` beside it is a different key's.** Nothing is
        // deleted and nothing is rewritten — the control is unavailable.
        std::fs::copy(
            locations::companions(&other).public_half,
            locations::companions(&mine).public_half,
        )
        .expect("the wrong public half");
        assert_eq!(
            copy_the_public_half(&row, &area),
            Copy::Unavailable(removal::WhyNot::ItIsADifferentKeysPublicHalf)
        );
        assert!(mine.exists(), "the private half is untouched");
        let _ = std::fs::remove_dir_all(&area);
    }

    /// **`Keys` W2 and W8: a borrowed key is never deleted**, and the refusal
    /// is the act's own re-derivation rather than the caller's word.
    #[test]
    fn a_borrowed_key_is_refused_and_its_files_stand() {
        if !there_is_a_generator() {
            return;
        }
        let area = scratch("borrowed");
        let home = area.join("home");
        std::fs::create_dir_all(&home).unwrap();
        // The key sits outside the usual key location, so it is borrowed.
        let path = a_real_key(&area, "theirs");
        let connection = a_record();
        let row = point_at_a_key(&path, "someone@their-mac", &crate::keys::TheRecordItself::over(&connection), &area).expect("a point");

        let refused = delete_a_managed_key(
            &row,
            &home,
            &crate::keys::TheRecordItself::over(&connection),
            &NoStore,
            Duration::from_secs(2),
        );
        assert_eq!(refused, Err(removal::NotRun::TheKeyIsNotSmoozysToDelete));
        assert!(path.exists(), "a borrowed key's file is never destroyed");
        assert_eq!(key_rows::all(&connection).unwrap().len(), 1);
        let _ = std::fs::remove_dir_all(&area);
    }

    /// **W8 and W9, on real files**: the three units run in order, the private
    /// half first, and the record says so.
    #[test]
    fn a_managed_key_is_deleted_with_its_companions_and_its_row() {
        if !there_is_a_generator() {
            return;
        }
        let area = scratch("delete");
        let home = area.join("home");
        let location = locations::usual_key_location(&home);
        std::fs::create_dir_all(&location).unwrap();
        let path = a_real_key(&location, "smoozy_ed25519");
        let connection = a_record();
        let row = point_at_a_key(&path, "someone@their-mac", &crate::keys::TheRecordItself::over(&connection), &area).expect("a point");

        let deleted = delete_a_managed_key(
            &row,
            &home,
            &crate::keys::TheRecordItself::over(&connection),
            &NoStore,
            Duration::from_secs(2),
        )
        .expect("a delete");
        use smoozy_state::outcomes::Keeps;
        let records = crate::record::outcome_records::OutcomeRecords::over(&connection);

        assert!(deleted.the_key_is_gone());
        assert!(!row.path.exists());
        assert!(!locations::companions(&row.path).public_half.exists());
        assert!(key_rows::all(&connection).unwrap().is_empty());

        let record = records.read(&deleted_record(&connection)).unwrap().unwrap();
        assert_eq!(
            record.units.iter().map(|(role, _)| role.as_str()).collect::<Vec<_>>(),
            vec!["the private half", "the public half", "the erasure"]
        );
        assert_eq!(
            record.the_actions_outcome(),
            Some(smoozy_state::outcomes::ActionOutcome::Whole)
        );
        let _ = std::fs::remove_dir_all(&area);
    }

    /// The one outcome record the act wrote, found by asking the record rather
    /// than by the act handing its identity back — which it does not, the
    /// delete's own answer being what the user is shown.
    fn deleted_record(connection: &rusqlite::Connection) -> Identity {
        let text: String = connection
            .query_row("SELECT identity FROM outcome_record", [], |row| row.get(0))
            .expect("one record");
        Identity::read_back(IdentityKind::Operation, &text).expect("it reads back")
    }

    /// **F-10: an act writes a diary line, and it writes it through the
    /// declaration.**
    ///
    /// No caller in this module constructed an `Act` or handed one to the
    /// service at all, so every line §12 declared was a line nothing wrote —
    /// and `Reporting a problem`'s own reason for a declaration is that a
    /// module writing nowhere satisfies its own spec and leaves a hole the
    /// Creator cannot tell from a thing that did not happen.
    ///
    /// **And what the line does not carry is checked with it**: no fingerprint,
    /// no local key identifier, no part of any value.
    #[test]
    fn pointing_at_a_key_and_removing_it_each_write_one_line_and_no_fingerprint() {
        use smoozy_reporting::lines::LineKind;

        if !there_is_a_generator() {
            return;
        }
        let area = scratch("diary");
        let path = a_real_key(&area, "kept");
        let connection = a_record();

        let service = crate::diary_service::DiaryService::open_at(area.join("diary"))
            .expect("the diary area");
        let mut declarations = smoozy_reporting::lines::Declarations::new();
        declarations.module_declares(crate::keys::THE_MODULE, smoozy_keys::diary::DECLARED.to_vec());
        let run = Identity::issue(IdentityKind::Run).expect("randomness");
        let diary = crate::diary_service::ModulesDiary {
            service: &service,
            declarations: &declarations,
            run: run.clone(),
            module: crate::keys::THE_MODULE,
        };

        let row = point_at_a_key(&path, "someone@their-mac", &crate::keys::TheRecordItself::over(&connection).with_the_diary(&diary), &area)
            .expect("a point");
        remove_from_smoozy(
            &row,
            &crate::keys::TheRecordItself::over(&connection).with_the_diary(&diary),
            &NoStore,
            Duration::from_secs(2),
        );

        let written: String = std::fs::read_dir(service.area())
            .expect("the diary")
            .flatten()
            .map(|entry| std::fs::read_to_string(entry.path()).expect("a segment"))
            .collect();
        let lines: Vec<&str> = written.lines().collect();
        assert_eq!(lines.len(), 2, "one line per act: {lines:?}");
        assert!(lines[0].contains("key pointed at"));
        assert!(lines[0].contains("a key kept elsewhere"));
        assert!(lines[1].contains("key removed"));
        assert!(lines[1].contains("a referenced key removed"));

        // **What no line of this module's ever carries.**
        assert!(!written.contains("SHA256:"), "no fingerprint, ever");
        assert!(
            !written.contains(&row.identifier.as_text()),
            "no local key identifier"
        );
        // The run identity is the only identity a line carries.
        assert!(written.contains(&format!("{run:?}")));

        // **A module with no declaration writes nothing**, which is A7's own
        // rule and is what the service checks rather than each caller.
        let undeclared = crate::diary_service::ModulesDiary {
            service: &service,
            declarations: &smoozy_reporting::lines::Declarations::new(),
            run,
            module: crate::keys::THE_MODULE,
        };
        let refused = undeclared.write(
            &smoozy_keys::diary::Act::AKeyRemoved(smoozy_keys::diary::AKeyRemoved {
                a_managed_key_was_deleted: false,
                file_units: Vec::new(),
                the_path_the_row_held: path.clone(),
                name: None,
                the_erasure: None,
            })
            .line(
                Identity::issue(IdentityKind::Run).expect("randomness"),
                std::time::SystemTime::now(),
            ),
        );
        assert!(matches!(
            refused,
            crate::diary_service::Wrote::No { .. }
        ));
        let _ = LineKind::Act;
        let _ = std::fs::remove_dir_all(&area);
    }

    /// **W10: remove from Smoozy touches no file at all.**
    #[test]
    fn remove_from_smoozy_leaves_every_file_where_it_is() {
        if !there_is_a_generator() {
            return;
        }
        let area = scratch("remove");
        let path = a_real_key(&area, "kept");
        let connection = a_record();
        let row = point_at_a_key(&path, "someone@their-mac", &crate::keys::TheRecordItself::over(&connection), &area).expect("a point");

        let erasure = remove_from_smoozy(
            &row,
            &crate::keys::TheRecordItself::over(&connection),
            &NoStore,
            Duration::from_secs(2),
        );
        assert_eq!(erasure, Erasure::Done);
        assert!(path.exists());
        assert!(locations::companions(&path).public_half.exists());
        assert!(key_rows::all(&connection).unwrap().is_empty());
        let _ = std::fs::remove_dir_all(&area);
    }
}
