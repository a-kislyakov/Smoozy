//! The record's own version, and the tables version 1 creates — `Desktop app
//! and delivery` L5 and L1.
//!
//! **L5: the record carries its own schema version, in the record.** Nothing
//! infers a version from which fields are present, because that is reading a
//! store's shape as state.
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
//! it belongs to.** `Security and licence` B30, E3a, S32. `Keys` N1, U7a, U9,
//! U9a. `Servers` B1, D3, D4, D6, H9a, P1a. `Server protocol and transfers`
//! S12, S28. `File Browser` N2, U1, U4, U8, U9a, U12.
//!
//! **A marker this crate's own plan also defines is deliberately not listed
//! above, whichever document is meant, and is written out with its document
//! at every site instead** — the header list cannot tell the two apart, and a
//! bare one would resolve to a real and wrong clause.
//!
//! **`Keys` N1 is written out at its site too**, because it stands inside a
//! semicolon list this crate's own N-clauses could be read into. **`Keys` and
//! `File Browser` each define a U9a**, and the two are different clauses — the
//! erase mark written with its reference, and the moment an upload unit's
//! outcome was written — so every one of those is written with its document at
//! its site and a bare one would resolve to whichever the reader guessed.

use super::job_records::EVERY_SET_BESIDE_THE_ROW;
use super::RecordFailure;

/// The schema this binary knows. **L9: a record carrying a higher number is not
/// opened at all**, and the fields this binary recognises are not read.
///
/// **It is the highest number [`super::migrations::ALL`] carries and never a
/// number above it.** A binary that claimed a version no migration produces
/// would open a record it had not finished making; one that claimed a version
/// below a migration it carries would run that migration and then refuse the
/// record it had just written.
pub const BINARY_KNOWS: u32 = 57;

/// The one-row table the version lives in.
///
/// **A table rather than the engine's own header value**, which is a Build
/// decision with one reason: L5's *in the record* is satisfied by either, and a
/// table is a thing a person can look at with any tool, where the header value
/// needs the engine to be asked for it. Nothing rests on the difference.
pub const VERSION_TABLE: &str = "smoozy_schema_version";

/// Read the version. **An absent table is version 0** — a record that has just
/// been created and has had no migration run against it — which is not a
/// fault and is the ordinary state at a first launch.
pub fn version_in(connection: &rusqlite::Connection) -> Result<u32, RecordFailure> {
    let exists: bool = connection.query_row(
        "SELECT count(*) > 0 FROM sqlite_schema WHERE type = 'table' AND name = ?1",
        [VERSION_TABLE],
        |row| row.get(0),
    )?;
    if !exists {
        return Ok(0);
    }
    let version: u32 =
        connection.query_row(&format!("SELECT version FROM {VERSION_TABLE}"), [], |row| {
            row.get(0)
        })?;
    Ok(version)
}

/// Write the version. Called only by a migration, inside that migration's own
/// transaction.
pub fn set_version(connection: &rusqlite::Connection, version: u32) -> Result<(), RecordFailure> {
    connection.execute(&format!("DELETE FROM {VERSION_TABLE}"), [])?;
    connection.execute(
        &format!("INSERT INTO {VERSION_TABLE} (version) VALUES (?1)"),
        [version],
    )?;
    Ok(())
}

/// Version 1 — the record as step 2 leaves it.
///
/// **L1 binds hard here: no row is added to `State, identity and recovery`
/// §1.1's inventory, and no table is created for a row of it whose owning
/// module has not been built.** That inventory names server rows, key rows,
/// history records, job records, transfer arrangements and a dozen more; every
/// one of them arrives with its own module at steps 3 to 11, as a migration
/// of its own. **Creating them empty now would be this document deciding their
/// shape**, which is precisely what L1 forbids.
///
/// **What is here is what step 2's own members hold**, and each names its
/// clause:
///
/// - **the store's standing** — `Security and licence` §3.3 S32's three
///   readings, which is what makes the nowhere-to-remember notice
///   once-per-installation rather than once-per-run.
/// - **the run-mark** — `State, identity and recovery` I2's pair, at that
///   document's §1.1, a state
///   and the run identity that wrote it. **The one durable value on this side
///   whose reading at launch takes a branch.**
/// - **history tombstones** — §10.1, kept *"for the life of the local record"*
///   and cleared by nothing. The table is here because **L12** is a rule about
///   migrations and a rule with nothing to protect cannot be kept or tested.
/// - **suppressed warnings** — §15.2's warning identity, here for the same
///   reason: **L11** forbids a migration removing a suppression row or
///   rewriting an identity, and Row 290's failure is invisible until a user
///   presses the control and one warning does not come back.
pub const VERSION_1: &[&str] = &[
    "CREATE TABLE smoozy_schema_version (version INTEGER NOT NULL)",
    // `Security and licence` §3.3 S32. One row, one value, for the whole
    // installation.
    "CREATE TABLE store_standing (
        only_row INTEGER PRIMARY KEY CHECK (only_row = 1),
        standing TEXT NOT NULL
     )",
    // `State, identity and recovery` I2, at that document's §1.1. One row,
    // holding the pair.
    "CREATE TABLE run_mark (
        only_row INTEGER PRIMARY KEY CHECK (only_row = 1),
        state TEXT NOT NULL,
        written_by TEXT NOT NULL
     )",
    // §10.1. One row per deleted history entry, for the life of the record.
    "CREATE TABLE history_tombstone (
        job TEXT PRIMARY KEY NOT NULL,
        deleted_at INTEGER NOT NULL,
        machine_side_copy TEXT NOT NULL
     )",
    // §15.2. One row per suppressed warning identity.
    //
    // **The moment here is dropped again at [`VERSION_15`], which G4 forbids
    // by name and G9 makes hazardous.** This statement is left as it was
    // because a migration is forward only, which is the same reason
    // [`VERSION_6`] gives for its own untouched line.
    "CREATE TABLE suppressed_warning (
        warning_identity TEXT PRIMARY KEY NOT NULL,
        suppressed_at INTEGER NOT NULL
     )",
];

/// Version 2 — `Keys`' own row, added at step 3 of `Preparation/Build/Plan.md`.
///
/// **L1: a module's row arrives with that module and never before it.** This is
/// the first of the eleven §1.1 rows that were deliberately absent from version
/// 1, and it arrives as its own migration because that is what L1 asks for.
///
/// **The fields are `Keys` R1's, and no more.** The local key identifier; the
/// recorded path, canonical and resolved at entry (`Keys` L3); the fingerprint
/// with **which half it was read from** (U7a — a fallback value may never be
/// compared for a swap, and the source is what says so); the name as last
/// read; the was-default-at-entry bit (`Keys` N1); and, where a passphrase was
/// stored, the secret-store subject reference **held with its erase mark**
/// (§5.1).
///
/// **The mark and the reference are one row's and are written together**
/// (`Keys` U9a): a reference standing with its mark cleared would make `Keys`
/// U9 answer *yes*
/// for an item that is gone, and an automatic attempt would be made where
/// `Servers` P3 withholds one. They are two columns of one row for that reason,
/// and never two tables.
///
/// **L10 holds here as everywhere: no secret value is written.** The subject
/// names an item in the operating system's store; the value stays there.
pub const VERSION_2: &[&str] = &[
    // `Keys` R1. One row per key, user-held (§11.1): it ends when the user
    // removes it and never otherwise.
    "CREATE TABLE key_row (
        identifier TEXT PRIMARY KEY NOT NULL,
        path TEXT NOT NULL,
        fingerprint TEXT,
        fingerprint_read_from TEXT,
        name TEXT,
        was_default_at_entry INTEGER NOT NULL,
        store_subject TEXT,
        owing_an_erase INTEGER NOT NULL
     )",
];

/// Version 3 — `State, identity and recovery` §9.4's outcome record.
///
/// **It arrives with the first module that owes one**, which is `Keys` at step
/// 3 — three of its acts have partition units smaller than themselves. **The
/// record itself is not `Keys`'**: it is a shared durable thing §9.4 owns and
/// every multi-part action in the app writes one, so it is built once, here,
/// rather than shaped in passing by whichever module reached it first.
///
/// **Two tables and not one, because the enumeration is a list.** `State,
/// identity and recovery` P5 item 1 makes the enumeration durable **before the
/// first unit is attempted** and item 2 writes each outcome **at the moment it
/// becomes true**, so the units are rows that are inserted once and updated
/// once each.
///
/// **The action's own outcome is in neither table**, which is `State, identity
/// and recovery` P5 item 3 kept by there being nowhere to write it: it is
/// derived from the unit outcomes at every reading. Two fields would drift.
///
/// **`outcome` is null until the unit's ending lands, and that null is
/// load-bearing** — it is the difference between a unit whose ending was lost
/// to an interruption and one that was deliberately never tried, which is
/// written as *not attempted* naming the unit that stopped it (`State,
/// identity and recovery` P4, at that document's §9.3).
///
/// **The unit is named by role, in the owning module's own words, and this
/// table never looks at the text.** `Keys` §11 forbids a path, a fingerprint,
/// a name or a comment in its own units; that is that module's rule kept at
/// that module's end.
pub const VERSION_3: &[&str] = &[
    // `State, identity and recovery` P5, at that document's §9.4. One row per
    // multi-part action,
    // user-held (§11.1): it ends when the user removes it and never otherwise.
    "CREATE TABLE outcome_record (
        identity TEXT PRIMARY KEY NOT NULL,
        action TEXT NOT NULL
     )",
    // The enumeration. `position` is the order the units run in, which is
    // §9.3's own order and is what *the rest were not attempted* is taken over.
    "CREATE TABLE outcome_record_unit (
        record TEXT NOT NULL,
        position INTEGER NOT NULL,
        role TEXT NOT NULL,
        outcome TEXT,
        stopped_by TEXT,
        PRIMARY KEY (record, position)
     )",
];

/// Version 4 — `Security and licence` §5.1 E3a's owed-erase register.
///
/// **The mark moves out of the key row, because a column of a row cannot
/// survive that row's removal** and `Security and licence` E3 asks for exactly
/// that: a removal goes ahead on a store failure and *"the subject stays
/// marked as owing an erase"*. Two tests demonstrated both directions of the
/// fault before this migration was written, and they are in this module.
///
/// **It is keyed by kind and subject and by nothing else.** It is not a row of
/// any module's — every stored kind's owed erases live here, and the three that
/// arrive at later steps need no table of their own.
///
/// **There is still one home for *is an erase owed*.** U9's availability answer
/// is the row's reference taken **against this register**, which is what
/// `Keys` U9a asks for and what a second column beside the reference could not
/// keep once the row was gone.
pub const VERSION_4: &[&str] = &[
    "CREATE TABLE owed_erase (
        kind TEXT NOT NULL,
        subject TEXT NOT NULL,
        PRIMARY KEY (kind, subject)
     )",
    // **Whatever version 2 and 3 had already marked comes with it.** A record
    // carrying an owed erase at the moment of this migration would otherwise
    // lose the debt to the migration itself, which is the fault this table
    // exists to end.
    "INSERT OR IGNORE INTO owed_erase (kind, subject)
     SELECT 'key passphrase', store_subject FROM key_row
     WHERE owing_an_erase = 1 AND store_subject IS NOT NULL",
    "ALTER TABLE key_row DROP COLUMN owing_an_erase",
];

/// Version 5 — `Servers`' own row, added at step 4 of
/// `Preparation/Build/Plan.md`.
///
/// **L1 again: a module's row arrives with that module and never before it.**
/// This is the second of the eleven §1.1 rows that were deliberately absent
/// from version 1, and it arrives as its own migration for the same reason
/// `Keys`' did.
///
/// **The fields are `Servers` D3's, in full, and no more.** The local server
/// identifier; the three connection-identity fields; the key reference as
/// P1a's pair — the identifier **and** the fingerprint beside it, because the
/// identifier alone is not the reference; the label and the colour; the
/// remembered setup bit; the remembered host key; the remembered hardware
/// reading; the last confirmed installation identity; the event cursor; and
/// the machine-password subject reference.
///
/// **Three tables and not one, because two of D3's fields are lists.** The
/// remembered setup is *which options were ticked* and the hardware reading's
/// cards are however many the machine has, so each is rows rather than a
/// packed value. **D6 makes the whole of D3 and D4 one enumeration and the
/// only route by which anything about a server is reached**, and a value
/// packed into a column would be reachable only by the code that wrote it.
///
/// **The hardware reading is spread across columns rather than stored whole.**
/// `Servers` H2 and `Servers` H2a make it two moments on two clocks, H9a makes
/// the
/// machine memory three cases of which two carry a figure, and `Servers` H11
/// makes the cards four states of which one is a list — so a single opaque
/// value would hide every one of those distinctions from anything but the code
/// that packed it. **The moments are nanoseconds since the epoch**, which
/// round-trips a reading exactly; seconds would quietly move `Servers` H2a's
/// *last seen* mark.
///
/// **L10 holds: no secret value is written.** The machine password's value is
/// in the operating system's store and this names it. **The remembered host
/// key is not a secret** — it is the machine's own public identity, and
/// `Security and licence` §7 puts it here by name: *"The remembered identity
/// lives in the local record against the saved server, and nowhere else."*
pub const VERSION_5: &[&str] = &[
    // `Servers` D3. One row per server, user-held (§11.1): *"the user removes
    // it, and never otherwise"*.
    "CREATE TABLE server_row (
        identifier TEXT PRIMARY KEY NOT NULL,
        host TEXT NOT NULL,
        port INTEGER NOT NULL,
        username TEXT NOT NULL,
        key_identifier TEXT NOT NULL,
        key_fingerprint TEXT NOT NULL,
        label TEXT,
        colour TEXT,
        remembered_setup INTEGER NOT NULL,
        remembered_host_key BLOB,
        hardware_cards TEXT,
        hardware_cards_unreadable TEXT,
        hardware_memory TEXT,
        hardware_memory_bytes INTEGER,
        hardware_free_space_bytes INTEGER,
        hardware_free_space_path TEXT,
        hardware_machine_moment INTEGER,
        hardware_desktop_moment INTEGER,
        hardware_read_under TEXT,
        last_confirmed_installation_identity TEXT,
        event_cursor INTEGER,
        machine_password_subject TEXT
     )",
    // **B1: the bit records which options were ticked.** A bit with no options
    // under it is a bit that says a setup is remembered and cannot say which,
    // so the row's own `remembered_setup` column says whether one stands and
    // these rows say what it holds.
    "CREATE TABLE server_remembered_setup_option (
        server TEXT NOT NULL,
        position INTEGER NOT NULL,
        option TEXT NOT NULL,
        PRIMARY KEY (server, position)
     )",
    // **`Servers` H5's count rule, kept by the store as well as by the
    // reading**: the count is the number of rows and is never a figure written
    // down beside them.
    "CREATE TABLE server_hardware_card (
        server TEXT NOT NULL,
        position INTEGER NOT NULL,
        unique_identifier TEXT NOT NULL,
        bus_location TEXT NOT NULL,
        make TEXT NOT NULL,
        total_memory_bytes INTEGER,
        PRIMARY KEY (server, position)
     )",
];

/// The tables L11, L12 and L12a protect, named once so all three rules read
/// from one list.
///
/// **L12a joined them on 2026-09-18 and the table it names is the job record.**
/// The list held two names because it was written when there was no third
/// table to hold: the job record arrived at version 23, and **the user's whole
/// render history lives in it**. As the list stood, a migration that emptied
/// that table would have taken the lot with nothing failing — no control
/// pressed, nothing refused, and a person opening History to find their work
/// gone.
///
/// **L12's own reasoning reaches it unchanged.** A tombstone is protected
/// because §10.1 keeps it *"for the life of the local record"* and a migration
/// is part of that life; the job record is kept on exactly the same terms. It
/// is cleared by the user's own delete, by a resolution, and by the acts that
/// cancel or drop a job that never rendered, and by nothing else — **and a
/// migration is not one of those acts and must not become one by accident.**
/// *(This listed a server's removal until 2026-09-18, when the Creator's
/// ruling C1 had a removal mark the records history never held rather than
/// take them — version 24.)*
///
/// **The job record is six tables and every one of them is named**, the row
/// and the five sets beside it (step 10b round 1 of *jobs, history and
/// downloads*, must-do 4; the fifth, `Queue & History` E9's kept words, since
/// record version 34). L12a says *"a migration never empties the local job
/// record"*, and a migration that emptied the disturbances table would lose
/// `Queue & History` N21's marks with nothing failing — the failure L12a was
/// written against. **The sets are taken from
/// [`super::job_records::EVERY_SET_BESIDE_THE_ROW`] rather than spelled a
/// second time**, so a set added there is protected here without anybody
/// editing this list.
pub const TABLES_A_MIGRATION_NEVER_EMPTIES: [&str; 3 + EVERY_SET_BESIDE_THE_ROW.len()] = {
    let mut all = [""; 3 + EVERY_SET_BESIDE_THE_ROW.len()];
    all[0] = "history_tombstone";
    all[1] = "suppressed_warning";
    all[2] = "job_record";
    let mut at = 0;
    while at < EVERY_SET_BESIDE_THE_ROW.len() {
        all[3 + at] = EVERY_SET_BESIDE_THE_ROW[at];
        at += 1;
    }
    all
};

/// **Version 6: the per-transfer records** — `Server protocol and transfers`
/// S9, whose durable home is `State, identity and recovery` §1.1's per-transfer
/// records nested under their transfer arrangement or multi-part action
/// outcome.
///
/// **`Server protocol and transfers` S8 is what makes this a table rather than
/// something written at the end**: the record is durable before the first
/// byte, so an upload cut off by the app closing keeps its leftover and the
/// user is told it is there.
pub const VERSION_6: &[&str] = &[
    "CREATE TABLE transfer_record (
        transfer TEXT PRIMARY KEY NOT NULL,
        attempt INTEGER NOT NULL,
        direction TEXT NOT NULL,
        server TEXT NOT NULL,
        installation TEXT,
        source_path TEXT NOT NULL,
        final_path TEXT NOT NULL,
        staging_directory TEXT NOT NULL,
        total_size_expected INTEGER NOT NULL,
        expected_digest TEXT,
        written_by_run TEXT NOT NULL,
        staging_identity TEXT,
        source_size INTEGER,
        source_inode TEXT,
        source_modification_time INTEGER,
        source_change_time INTEGER,
        publication_intent TEXT,
        publication_staging_path TEXT,
        publication_final_path TEXT,
        publication_staged_identity TEXT,
        publication_byte_length INTEGER,
        publication_digest TEXT,
        ending TEXT,
        stopped_reason TEXT,
        cleanup_is_owed INTEGER NOT NULL
     )",
    // **`Server protocol and transfers` S4's superseded attempt numbers,
    // retained so the sweep can derive their paths.** Rows rather than a packed
    // field, on `Servers` D6's rule. **The identity each number carries is
    // [`VERSION_8`]'s column**; a number alone is not enough for `Server
    // protocol and transfers` S28 to remove anything, and version 6 is left as
    // it was because a migration is forward only.
    "CREATE TABLE transfer_superseded_attempt (
        transfer TEXT NOT NULL,
        attempt INTEGER NOT NULL,
        PRIMARY KEY (transfer, attempt)
     )",
    // **A checkpoint is a byte offset into the staged file with the running
    // digest at it**, and the two are one row because `Server protocol and
    // transfers` S12 step 3 compares them together.
    "CREATE TABLE transfer_checkpoint (
        transfer TEXT NOT NULL,
        offset_into_the_staged_file INTEGER NOT NULL,
        digest TEXT NOT NULL,
        PRIMARY KEY (transfer, offset_into_the_staged_file)
     )",
];

/// **Version 7: the machine's own root, against the server row.**
///
/// `Job files and outputs` **L1**: *"the root is chosen once per installation
/// and never re-chosen. Its path is recorded in the machine's state area at
/// hand-over, beside the installation identity, and **it is recorded against
/// the server row on the desktop side as a machine-derived value carrying that
/// identity**."*
///
/// **Two columns, because the identity travels with the path** (§2.3), and a
/// path with no identity beside it says nothing about which installation chose
/// it. **The row's shape is `Servers`' and its D3 table was repaired to name
/// this field**; this is the store performing it, which is `Security and
/// licence` §1.4 B30's split.
///
/// **An `ALTER`, not a rebuild**, so nothing the record already holds is
/// dropped — L11 and L12 forbid a migration removing what the record holds.
pub const VERSION_7: &[&str] = &[
    "ALTER TABLE server_row ADD COLUMN machine_root_path TEXT",
    "ALTER TABLE server_row ADD COLUMN machine_package_area_path TEXT",
    "ALTER TABLE server_row ADD COLUMN machine_root_read_under TEXT",
];

/// **Version 8: the staging object's stable file identity, beside each
/// superseded attempt number** — `Server protocol and transfers` **S9**.
///
/// That clause records the identity *"beside each attempt number the record
/// retains — the live attempt's and each superseded attempt's, once that
/// attempt's staging object was created — or, per attempt, the recorded fact
/// that its supplier could not answer"*. **The live attempt's was already a
/// column of `transfer_record`; version 6 kept the superseded attempts as bare
/// numbers**, so the sweep derived every superseded path with no identity, and
/// `Server protocol and transfers` **S28**'s only removing branch wants the
/// path, the synthesised name **and** the identity to match. A superseded path
/// could therefore never be removed — **a part-file left inside the user's own
/// destination folder, for good, on every download that starts over**.
///
/// **An `ALTER`, not a rebuild**, so nothing the record already holds is
/// dropped — `Desktop app and delivery` L11 and L12 forbid a migration removing
/// what the record holds. **The column is nullable and every row a version 7
/// record already holds reads back as null**, which is the honest answer for an
/// attempt whose identity was never written down: S28 leaves those paths alone,
/// which is the same outcome those records had before this migration.
pub const VERSION_8: &[&str] =
    &["ALTER TABLE transfer_superseded_attempt ADD COLUMN staging_identity TEXT"];

/// **Version 9: the fields `File Browser` U1 adds to the outcome record**,
/// added at step 5 of `Preparation/Build/Plan.md`.
///
/// **L1 again: a module's row arrives with that module.** `File Browser` U1
/// says the upload request's durable half **is one `State, identity and
/// recovery` §9.4 outcome record** — not a record of its own — and that
/// document's half is already here at [`VERSION_3`], holding the enumeration
/// and the outcomes and nothing of U1's. **The fields `File Browser` U1 adds
/// had no table**,
/// so that module's own keeper stood over a test list and no upload request
/// could survive an application run.
///
/// **Three tables beside the shared two, and not columns on them.** The shared
/// `outcome_record` and `outcome_record_unit` are every module's — `Keys`'
/// three acts write them, `Download` M1's request will — and widening them with
/// one module's fields would make the shared table the union of every module's
/// as the modules land. So `File Browser` U1's fields sit in tables of that
/// module's own,
/// **keyed to §9.4's own enumeration by `(record, position)`**, which is that
/// section's own ordering field. There is **one enumeration**, joined rather
/// than copied: the unit outcomes stay where `State, identity and recovery` P5
/// item 2 puts them and are not
/// written twice.
///
/// **Every field is `File Browser` U1's own and no other is here.** The server row and the
/// installation identity; the destination folder; per entry the source path on
/// the user's own computer **as stored on disk**, the landing path, the size as
/// enumerated, the disposition after the collision answer, the transfer
/// identity once a transfer is arranged and `File Browser` U9a's moment;
/// `File Browser` U4's collision
/// answer; the affected jobs of the telling; and the run that opened it, which
/// is what `File Browser` U8 separates a fresh sitting on.
///
/// **The two paths are `BLOB` and never `TEXT`.** `File Browser` U1 says the
/// source path is
/// held *as stored on disk* and `File Browser` N2 carries names as bytes with
/// nothing rewriting, normalising or re-encoding one; a text column would put a
/// decode between the disk and the record, and `File Browser` X5's comparison
/// is byte for
/// byte.
///
/// **The request's own ending has no column, and that is `State, identity and
/// recovery` P5 item 3 kept by there being nowhere to write it.** `File
/// Browser` U1: *"derived from the unit outcomes and
/// the run, never a second stored field."*
///
/// **The disposition is one column and one companion, because two of the four
/// carry a value.** Keep-both carries `File Browser` X9's generated landing
/// path, computed
/// once when the answer is applied; handed-over carries the operation identity
/// of the act `Dependency Upload & Remap` opens (`File Browser` U12). The other
/// two carry
/// nothing, and a null disposition is a request whose answer has not been
/// applied yet — which `File Browser` U4 makes a real state rather than a
/// missing value.
///
/// **The telling is rows and its emptiness is its absence.** `File Browser` U1:
/// the affected
/// jobs and the act intended are fields of **this** record and never of a
/// second one, and *"a request that disturbed nothing carries neither field"* —
/// so no row is the whole of *no telling*, and no flag stands beside it. **The
/// act intended is the upload request itself**, which is why these rows hold
/// the jobs alone.
///
/// **L10 holds: no secret value is written.** L4 holds too — no field of this
/// record is ever placed in a filename; the landing path is a value the machine
/// side already carries, not a name this store composes.
pub const VERSION_9: &[&str] = &[
    // `File Browser` U1's record-level fields. One row per upload request,
    // beside the §9.4 outcome record of the same identity. **User-held**
    // (`State, identity and recovery` §11.1): it goes when the user removes
    // it, never by age.
    "CREATE TABLE upload_request (
        record TEXT PRIMARY KEY NOT NULL,
        server_row TEXT NOT NULL,
        installation TEXT,
        destination BLOB NOT NULL,
        collision_answer TEXT,
        opened_by_run TEXT NOT NULL
     )",
    // `File Browser` U1's enumeration entry. **`position` is §9.4's own
    // ordering** and is what
    // joins this to `outcome_record_unit`, so the unit's outcome and the moment
    // it was written are one unit rather than two.
    "CREATE TABLE upload_request_entry (
        record TEXT NOT NULL,
        position INTEGER NOT NULL,
        source_path BLOB NOT NULL,
        landing_path BLOB NOT NULL,
        size_as_enumerated INTEGER NOT NULL,
        disposition TEXT,
        keep_both_landing_path BLOB,
        handed_to_act TEXT,
        transfer TEXT,
        outcome_written_at INTEGER,
        PRIMARY KEY (record, position)
     )",
    // `File Browser` U1's telling. Rows rather than a packed field, on
    // `Servers` D6's rule,
    // and none at all for a request that disturbed nothing.
    "CREATE TABLE upload_request_telling_job (
        record TEXT NOT NULL,
        position INTEGER NOT NULL,
        job TEXT NOT NULL,
        PRIMARY KEY (record, position)
     )",
];

/// **Version 10: `Dependency Upload & Remap`'s five record kinds and its three
/// per-job fields**, added at step 7 of `Preparation/Build/Plan.md`.
///
/// **L1 again: a module's row arrives with that module.** `State, identity and
/// recovery` §1.1's inventory names this module's kinds and says the per-job
/// resolved set is a field of the job's own record; every table below is one of
/// those and nothing else is here.
///
/// **`Dependency Upload & Remap` K1 closes the list at five** — the
/// located-answer record (its K3), the placement record (K7), the fixing record
/// (K8), the check standing (V10) and the shift record (K6) — *"a builder that
/// needs a second state store has found a fault in this section rather than a
/// case it does not cover."* **Three more tables sit beside them and none is a
/// sixth kind**: that document's K9 per-job resolved set, its
/// `Dependency Upload & Remap` M22f placed-copy
/// list, and its F1 searched-folder list, each of which that document places as
/// a field or a list rather than as a kind. **They are tables of this module's
/// own, keyed by the job identity**, which is the shape [`VERSION_9`] already
/// took for `File Browser` U1 — widening another module's table with one
/// module's fields is what makes a shared table the union of every module's.
///
/// **Every path is `BLOB` and never `TEXT`, and that is load-bearing rather than
/// tidy.** `Dependency Upload & Remap` K3 keys a located answer on *"the
/// recorded location's exact bytes, capitals included"*, its F9 compares
/// enumerated bytes, its F12 records the stored location as the system returned
/// it, and `Dependency Upload & Remap` G3a says outright that a recorded
/// location *"is bytes and is not
/// required to be text"* — a real disk holds names that do not decode. A text
/// column would put a decode between the disk and the record, and a substituted
/// byte keys the wrong record, mirrors a wrong spelling and matches no pair at
/// the remap.
///
/// **A machine-keyed row carries the installation identity beside the server
/// row** (its K7), because *"a recycled machine answers on the same address with
/// everything gone, and the installation identity is the only thing that
/// tells"* (`State, identity and recovery` §2.3). **The rows are not deleted
/// when an identity is superseded**: §11.1 disqualifies them rather than ending
/// them, which is a question asked at every read and never a sweep.
///
/// **Where a key would otherwise be null it is the empty blob, and the reason is
/// SQLite's own.** A located answer is keyed by a holding file that is either a
/// path on the user's own computer (K3) or a machine location under an
/// installation identity (K5), never both — and this engine allows nulls inside
/// a primary key, so a nullable key column would let the same answer be written
/// twice with nothing refusing it. `holding_kind` says which of the two the row
/// is, and the columns the other case does not use hold the empty value.
///
/// **Nothing here has an age, a cap or an elapsed-time lifetime** (K10), and
/// **L10 holds: no secret value is written.**
pub const VERSION_10: &[&str] = &[
    // **`Dependency Upload & Remap` K3, K4 and K5 — the located answer.**
    // User-held where the holding file is the user's own (K10), and the whole
    // of what the user decided by hand; K2 keeps it off every rented machine.
    // **A `resolution` of null is a *not found* row** — that document's F7 makes
    // a *not found* row the **absence** of a located answer rather than a
    // statement about the disk, so the columns are null rather than carrying a
    // word for it.
    "CREATE TABLE dependency_located_answer (
        holding_kind TEXT NOT NULL,
        holding_source_path BLOB NOT NULL,
        holding_machine_location BLOB NOT NULL,
        holding_server_row TEXT NOT NULL,
        holding_installation TEXT NOT NULL,
        reference BLOB NOT NULL,
        resolved_to BLOB,
        resolved_to_is_a_folder INTEGER,
        how TEXT,
        how_searched_folder BLOB,
        how_arbitrary_pick BLOB,
        how_shift INTEGER,
        set_aside INTEGER NOT NULL,
        rejected_guess BLOB,
        PRIMARY KEY (
            holding_kind,
            holding_source_path,
            holding_machine_location,
            holding_server_row,
            holding_installation,
            reference
        )
     )",
    // **`Dependency Upload & Remap` F5's candidates the arbitrary pick was taken
    // among.** Rows rather than a packed field, on `Servers` D6's rule. **A row
    // here is only ever written where the pick was arbitrary**: F5 records *"the
    // pick taken, the candidates it was taken among, and that it was a coin toss
    // rather than a judgment"*, and no candidates at all is a pick that was not
    // one.
    "CREATE TABLE dependency_located_answer_candidate (
        holding_kind TEXT NOT NULL,
        holding_source_path BLOB NOT NULL,
        holding_machine_location BLOB NOT NULL,
        holding_server_row TEXT NOT NULL,
        holding_installation TEXT NOT NULL,
        reference BLOB NOT NULL,
        position INTEGER NOT NULL,
        candidate BLOB NOT NULL,
        PRIMARY KEY (
            holding_kind,
            holding_source_path,
            holding_machine_location,
            holding_server_row,
            holding_installation,
            reference,
            position
        )
     )",
    // **`Dependency Upload & Remap` K6 — the shift record**: the old root, the
    // new root, and the moment. User-held. **The key is a record key and not an
    // identity** in `State, identity and recovery` §2.2's register — K6 says so
    // in as many words, and nothing derives an identity from it.
    "CREATE TABLE dependency_shift (
        shift INTEGER PRIMARY KEY NOT NULL,
        old_root BLOB NOT NULL,
        new_root BLOB NOT NULL,
        written_at INTEGER NOT NULL
     )",
    // **`Dependency Upload & Remap` K7 — the placement record**, one per
    // (machine, mirrored location). The content identity is the transfer's own
    // pass-through digest beside the size (its Q1, `Server protocol and
    // transfers` S22); the four values are `Server protocol and transfers` S13's
    // shape and `Job files
    // and outputs` R10's one shape.
    "CREATE TABLE dependency_placement (
        server_row TEXT NOT NULL,
        installation TEXT NOT NULL,
        mirrored_location BLOB NOT NULL,
        source_location BLOB NOT NULL,
        content_size INTEGER NOT NULL,
        content_digest TEXT NOT NULL,
        published_size INTEGER NOT NULL,
        published_inode TEXT NOT NULL,
        published_modification_time INTEGER NOT NULL,
        published_change_time INTEGER NOT NULL,
        confirmation_outcome TEXT,
        confirmation_at INTEGER,
        marked_gone INTEGER NOT NULL,
        PRIMARY KEY (server_row, installation, mirrored_location)
     )",
    // **`Dependency Upload & Remap` K8 — the fixing record**, one per (machine,
    // fixed copy), holding the machine location of the copy it was derived from
    // **and that copy's content identity**, which is what
    // `Dependency Upload & Remap` M16b
    // compares to
    // learn that the copy was published over.
    "CREATE TABLE dependency_fixing (
        server_row TEXT NOT NULL,
        installation TEXT NOT NULL,
        fixed_copy BLOB NOT NULL,
        derived_from BLOB NOT NULL,
        derived_from_size INTEGER NOT NULL,
        derived_from_digest TEXT NOT NULL,
        PRIMARY KEY (server_row, installation, fixed_copy)
     )",
    // **What the fixing points at** — K8's list of (reference bytes → machine
    // location) pairs written into the copy, which its M16 second trigger
    // compares against the current located answers. Rows rather than a packed
    // field.
    "CREATE TABLE dependency_fixing_points_at (
        server_row TEXT NOT NULL,
        installation TEXT NOT NULL,
        fixed_copy BLOB NOT NULL,
        position INTEGER NOT NULL,
        reference BLOB NOT NULL,
        machine_location BLOB NOT NULL,
        PRIMARY KEY (server_row, installation, fixed_copy, position)
     )",
    // **`Dependency Upload & Remap` V10 — the check standing**, one per (root
    // file's machine path, machine under its installation identity). **The two
    // marks are columns and not an absence**: `Dependency Upload & Remap` V14
    // makes *nobody has ever
    // checked this file on this machine* and *a check was run and the file has
    // been replaced since* two different record states rather than two absences,
    // which is no row against a row carrying its marks.
    //
    // **`every_row_confirmed_or_set_aside` is a column because it cannot be
    // worked out from the six counts.** `Dependency Upload & Remap` V11's stop
    // rule is *a complete check
    // whose every row is either confirmed on the machine or set aside*, and an
    // `AlreadyOnTheMachine` or `SentBeforeNotConfirmed` row is not confirmed
    // and takes no count — so four zero counts do not mean every row was
    // confirmed or set aside. A keeper rebuilding the standing from this table
    // without it would have to guess, and the plausible guess stops
    // `Rendering`'s warning on a file that still has unconfirmed rows. **Added
    // 2026-09-05, out of round 1 of the code audit of `Dependency Upload &
    // Remap`.**
    //
    // **It was added by editing this version in place, on a ground that was
    // false when it was written.** The note said, in these words: *"VERSION_10
    // was edited in place rather than given a migration, 10 being the newest
    // version with nothing shipped and no record outside a builder's own
    // scratch copy able to have passed it — the same ground the renumber of
    // 2026-09-04 stood on."* **Kept rather than deleted**, because it is the
    // first instance of the fault that later cost §16's whole failure shape a
    // day of silence — see [`VERSION_14`]'s own note.
    //
    // **Version 10 had been reachable since the day before the edit**, so a
    // record opened in that window would carry this table without the column
    // and every read and write of it would answer *no such column*. **The
    // Creator's record was opened on 2026-09-13 and it carries the column**, so
    // no record anybody can point to was hurt — **and that is luck rather than
    // the reasoning being right.** [`version_16`] repairs it by shape, so a
    // record from that window is repaired and every other record crosses
    // untouched.
    "CREATE TABLE dependency_check_standing (
        server_row TEXT NOT NULL,
        installation TEXT NOT NULL,
        root BLOB NOT NULL,
        read_with_build_path TEXT NOT NULL,
        read_with_version TEXT NOT NULL,
        completion TEXT NOT NULL,
        completed_at INTEGER,
        count_not_found INTEGER NOT NULL,
        count_could_not_be_sent INTEGER NOT NULL,
        count_could_not_be_fixed INTEGER NOT NULL,
        count_found_but_not_sent INTEGER NOT NULL,
        count_set_aside INTEGER NOT NULL,
        count_confirmed INTEGER NOT NULL,
        root_size INTEGER NOT NULL,
        root_inode TEXT NOT NULL,
        root_modification_time INTEGER NOT NULL,
        root_change_time INTEGER NOT NULL,
        every_row_confirmed_or_set_aside INTEGER NOT NULL,
        replaced_since_check INTEGER NOT NULL,
        disturbed INTEGER NOT NULL,
        PRIMARY KEY (server_row, installation, root)
     )",
    // **`Dependency Upload & Remap` K9 — the job's resolved set**, composed at
    // submission and **immutable for the life of the job's record**. One row per
    // machine location, plus the material's own location, which K9 puts in the
    // set so that its V2 points-at reading, its V6 live-job rule and its V3
    // delete warning reach it with no new mechanism.
    "CREATE TABLE dependency_job_resolved_set (
        job TEXT NOT NULL,
        server_row TEXT NOT NULL,
        installation TEXT NOT NULL,
        remap_material BLOB NOT NULL,
        blend BLOB NOT NULL,
        PRIMARY KEY (job)
     )",
    "CREATE TABLE dependency_job_resolved_location (
        job TEXT NOT NULL,
        position INTEGER NOT NULL,
        machine_location BLOB NOT NULL,
        PRIMARY KEY (job, position)
     )",
    // **the placed-copy list of `Dependency Upload & Remap` M22f** — a field of
    // the job's own record rather than a sixth kind. Per copy: the exact
    // job-folder path written, and **the four-value identity of that copy read
    // the moment it was made** — which is what `Download` N5a skips on, and what
    // separates *untouched* from *written over* without a digest.
    "CREATE TABLE dependency_job_placed_copy (
        job TEXT NOT NULL,
        position INTEGER NOT NULL,
        job_folder_path BLOB NOT NULL,
        made_size INTEGER NOT NULL,
        made_inode TEXT NOT NULL,
        made_modification_time INTEGER NOT NULL,
        made_change_time INTEGER NOT NULL,
        PRIMARY KEY (job, position)
     )",
    // **`Dependency Upload & Remap` F1 — the searched-folder list**: one
    // app-wide ordered list, user-held, **never per server**. It describes the
    // user's own computer, so a new rented machine inherits it with nothing
    // re-added. **The position is the user's order and F4 decides on it**, so it
    // is the key rather than a sort hint. **`Dependency Upload & Remap` F1a: the
    // list carries no mark of its
    // own** — a folder's *could not be read* verdict is one search's and is
    // run-held, so there is no column for it here.
    "CREATE TABLE dependency_searched_folder (
        position INTEGER PRIMARY KEY NOT NULL,
        folder BLOB NOT NULL
     )",
];

/// **`State, identity and recovery` J6's server-package trust record** — one
/// of the three monotonic trust records, in its own namespace, sharing no
/// field with either of the others.
///
/// **One row, or none.** `Server protocol and transfers` B9 asks the desktop to
/// remember the greatest accepted key-set generation and release sequence and
/// to refuse a rollback of either; **`State, identity and recovery` J7 makes
/// the retained statement the thing those numbers stand on**, so the statement
/// and its root signature are columns of the same row rather than a second
/// table. A record whose statement will not verify is discarded whole, and one
/// row is what makes that one delete.
///
/// **The statement is kept as the bytes that were signed.** Verifying it again
/// means verifying exactly what was verified the first time, so a parsed and
/// re-encoded form would be a second encoding to disagree about.
///
/// The desktop updater has its separate trust record under
/// `Desktop app and delivery` E15 and E19. The retained sequences are
/// independent (`State, identity and recovery` J10).
pub const VERSION_11: &[&str] = &["CREATE TABLE server_package_trust (
        only_row INTEGER PRIMARY KEY NOT NULL CHECK (only_row = 1),
        greatest_accepted_release_sequence INTEGER NOT NULL,
        greatest_accepted_key_set_generation INTEGER NOT NULL,
        the_statement BLOB NOT NULL,
        its_root_signature BLOB NOT NULL
     )"];

/// Version 12 — the historical licence state and ordered slot list.
/// Shipped migration retained unchanged; a later migration drops these tables.
pub const VERSION_12: &[&str] = &[
    "CREATE TABLE licence_record (
        only_row INTEGER PRIMARY KEY NOT NULL CHECK (only_row = 1),
        state TEXT NOT NULL,
        lock_cause TEXT,
        lock_reason TEXT,
        trial_local_start INTEGER,
        trial_recorded_at_the_service INTEGER,
        trial_service_computed_ending INTEGER,
        last_successful_check_service_time INTEGER,
        high_water_mark INTEGER,
        grant_expiry_kind TEXT NOT NULL,
        grant_expiry INTEGER,
        reserve_remaining INTEGER,
        licence_key_subject TEXT,
        pending_first_entry INTEGER,
        accepted_statement_sequence INTEGER,
        accepted_statement BLOB,
        last_resort_identity BLOB
     )",
    // Historical slot order: taken-at moment followed by slot identifier.
    "CREATE TABLE licence_slot (
        position INTEGER PRIMARY KEY NOT NULL,
        slot_identifier TEXT NOT NULL,
        computer_name TEXT NOT NULL,
        taken_at INTEGER NOT NULL,
        last_checked INTEGER NOT NULL,
        whose TEXT NOT NULL
     )",
];

/// Version 13 — **the remembered pane widths**, `Desktop app and delivery`
/// §3.3 P11 and `State, identity and recovery` §1.1 I3. Added at step 10a
/// piece B, which is the piece that builds the seams that write them.
///
/// **L1: a row arrives with the thing that needs it.** There was no settings
/// table and no widths row in this record before this migration, because until
/// this piece nothing in Smoozy drew a pane, let alone a seam to drag.
///
/// **One row per seam and nothing else in the row.** I3 rule 1 closes the set
/// of seams — the rail's width is one value for every screen and a list's width
/// is one value shared by the queue and the history — so the key is a seam's
/// own name and never a screen's. **The closed set is
/// [`crate::surface::panes::THE_SEAMS`] and is enforced there rather than by a
/// constraint here**: a `CHECK` naming the three would be the same closed set
/// living in two places, which is the fault this stage keeps meeting, and it
/// would make a seam Design later draws a migration rather than a line.
///
/// **No moment is stored beside the width**, and that is deliberate rather than
/// an omission. `State, identity and recovery` I3 rule 5 makes a pane width
/// **user-held** — *"No age and no cap"* — so a timestamp here would be the
/// shape of a sweep nobody may write, sitting in the table waiting for
/// somebody. The only question this row ever answers is *how wide did the user
/// leave that seam*, and the width answers it.
///
/// **A row is removed only by the user's own reset** (I3 rule 4), which clears
/// the value rather than writing the drawn default into it. **L11 does not
/// reach this table** — that rule protects the suppression rows, whose loss is
/// invisible; a lost pane width costs a drag.
pub const VERSION_13: &[&str] = &["CREATE TABLE pane_width (
        seam TEXT PRIMARY KEY NOT NULL,
        width INTEGER NOT NULL
     )"];

/// Version 14 — **failure records**, `State, identity and recovery` §8 and
/// `Desktop app and delivery` §16.2 M4. Added at step 10a piece C, which is
/// the piece that presents them.
///
/// **L1: a row arrives with the thing that needs it.** `State, identity and
/// recovery` J1, at that document's §1.1, has named *failure records* in this
/// record's inventory
/// since step 2 and there was no table, because until this piece nothing in
/// Smoozy put a failure in front of anybody.
///
/// **Whether this should be a table at all was the one thing piece C's brief
/// told it to settle before writing code, and the whole of the reasoning is at
/// the head of [`super::failure_records`].** The short of it: §8.2's title is
/// about the **current value** being derived, not about the record;
/// `State, identity and recovery` J1,
/// §11.1's subject-held class, §8.2 F7's *removed at the next write to its
/// subject*, `State, identity and recovery` F21 and `State, identity and
/// recovery` F25's clearing all require the record to be
/// durable.
///
/// **Two tables because F5's value set is a set.** *"A condition record holds
/// the condition identity and the set of observed values it was raised
/// over"* — the connection condition's set has two members, because an outage
/// reads as *disconnected* and as *reconnecting* by turns while it lasts.
///
/// **The condition key is a column of the row it finds and never a table of its
/// own** (§8.3 F11), so *this key already has an identity* has one home. The
/// unique index is what makes F11's *no second record is raised* a property of
/// the store rather than of the caller's care.
///
/// **L11 does not reach these tables and that is deliberate.** That rule
/// protects the suppression rows, whose loss is invisible until a user presses
/// a control and one warning does not come back; a failure record has three
/// stated endings (`State, identity and recovery` F25) and two of them are
/// removals, so a table that
/// could never lose a row would be a table nothing could clear.
pub const VERSION_14: &[&str] = &[
    // `State, identity and recovery` §8.1, §8.2, §8.7. One row per failure
    // record, **subject-held** (§11.1) with the two extra endings §8.8 names.
    //
    // **No current value, no prose, no moment.** Each of the three is forbidden
    // by a named clause and each is checked by a test in
    // `super::failure_records`.
    "CREATE TABLE failure_record (
        kind TEXT NOT NULL CHECK (kind IN ('condition', 'ending')),
        durable_subject TEXT NOT NULL,
        durable_subject_kind TEXT NOT NULL,
        identity_or_mark TEXT NOT NULL,
        condition_key TEXT,
        the_diary_holds_the_sentence INTEGER NOT NULL,
        PRIMARY KEY (durable_subject, durable_subject_kind, identity_or_mark)
     )",
    // §8.3 F11's lookup. **A unique index rather than a second table**, and it
    // is what refuses a second record against one key.
    "CREATE UNIQUE INDEX failure_record_by_condition_key
        ON failure_record (condition_key)",
    // §8.2 F5's set.
    //
    // **`durable_subject_kind` is part of this key for the same reason it is
    // part of the row's own**, and it was missing until 2026-09-12. §8.7 F21's
    // list holds two subjects that carry the same identity's rendering — a
    // job's own record and its history entry — so a set keyed on the name
    // alone puts one subject's values under another's. Found at round 1 of the
    // code audit of what is drawn inside the window (F-1).
    //
    // **The column was added by editing this version in place, and the ground
    // given for that was false when it was written.** The note said, in these
    // words: *"VERSION_14 was edited in place rather than given a migration, 14
    // being a version nothing shipped has ever passed and no record outside a
    // builder's own scratch copy able to hold — the same ground VERSION_10's
    // own in-place edit stood on."* **It is kept here rather than deleted,
    // because the mistake is worth more than the fix.**
    //
    // **Why it was false.** The Creator's own record was made on 2026-09-03 by
    // step 2's own run, it had already passed through 14, and it held the
    // three-column table. So on the one computer this project runs on, every
    // read of a standing failure answered *no such column*, nothing was drawn,
    // nothing was logged and nothing said so — the whole of §16's failure shape
    // was invisible for a day, and it was found by accident.
    //
    // **The rule that replaces the reasoning: a shipped migration is never
    // edited again, whatever the argument about who could be holding one.**
    // The argument here was careful and specific and it was still wrong,
    // because **nobody can enumerate the records in the world, and the desk's
    // own record is one of them.** The check that would have caught it costs
    // one command: open the Creator's record and read the table.
    //
    // **The repair is [`version_16`]**, which brings a three-column table to
    // four by looking at the table rather than by reasoning about who has one.
    // **This statement is left exactly as it stands**, because a migration is
    // forward only and a fresh record is entitled to the shape this version
    // now makes.
    "CREATE TABLE failure_record_raised_over (
        durable_subject TEXT NOT NULL,
        durable_subject_kind TEXT NOT NULL,
        identity_or_mark TEXT NOT NULL,
        value TEXT NOT NULL,
        PRIMARY KEY (durable_subject, durable_subject_kind, identity_or_mark, value)
     )",
];

/// Version 15 — **the suppression row becomes one column**, `Desktop app and
/// delivery` §15.2 G4 and `State, identity and recovery` §10.3. Added at step
/// 10a piece F, which is the piece that writes the table and builds the control
/// that empties it.
///
/// **What the table as built got wrong, and both halves were found by reading
/// the clause rather than the code.** [`VERSION_1`] made it with two columns —
/// the warning identity and a `suppressed_at` moment. **G4 forbids the moment
/// by name**: *"No wording, no timestamp, no count, no per-subject qualifier —
/// a row carrying anything more would be a suppression two rows could disagree
/// about."* And it is not a harmless extra column. **G9 says suppressions are
/// user-held, with *no age, no cap, no sweep***, so a stored moment is the
/// exact shape of the sweep that clause forbids, sitting in the table waiting
/// for somebody to write it.
///
/// **The row is one column and there is no module column.** G4 and
/// `State, identity and recovery` §10.3 both say the row is *the module and the
/// warning identity*, and that is kept in the identity's own spelling: every
/// declared identity is prefixed with its own module's name — `File Browser`
/// M3's is `file-browser.delete` and `Rendering` W4a's is
/// `rendering.missing-assets` — and
/// [`crate::presentation::declarations::TheDeclarations::every_declared_warning_names_its_module`]
/// is where that binding is checked. **A module column would be the same fact
/// in two places**, with nothing reading it: G7's restore is one delete over
/// the whole table and walks no modules, and a suppression check is asked by
/// the owning module with its own identity already in hand.
///
/// **L11 and G6 bind this migration and it meets them by shape rather than by
/// care.** A column is dropped; no row is read, rewritten or removed, so a
/// table holding a hundred suppressions comes out of it holding the same
/// hundred with one column fewer. [`TABLES_A_MIGRATION_NEVER_EMPTIES`] already
/// names this table and [`super::migrations::nothing_protected_was_lost`]
/// counts it before and after.
///
/// **[`VERSION_1`] is left as it was, which is this file's own habit** —
/// [`VERSION_4`] drops a column [`VERSION_2`] made, and version 6's own note
/// says a version is left alone *"because a migration is forward only"*. A
/// fresh record therefore makes the column and drops it again in the same
/// start, which costs one statement and keeps one history for every record
/// whatever version it came from.
pub const VERSION_15: &[&str] = &[
    // §15.2 G4: the identity and nothing else. **G9's *no age, no cap, no
    // sweep* is what makes the moment worth removing rather than tolerating.**
    "ALTER TABLE suppressed_warning DROP COLUMN suppressed_at",
];

/// **Whether a table in this record has a column, asked of the engine's own
/// account of the table.**
///
/// **This is the whole of *by shape rather than by care*.** A migration that
/// asks this decides from the record in front of it; a migration that reasons
/// about which records could exist decides from an argument, and an argument
/// about that is what [`VERSION_10`] and [`VERSION_14`] were both edited in
/// place on.
///
/// **A table that is not there answers `false` for every column**, which is the
/// honest answer and is why the caller checks the table it is about to touch
/// rather than assuming a version implies one.
pub fn the_table_has_the_column(
    connection: &rusqlite::Connection,
    table: &str,
    column: &str,
) -> Result<bool, RecordFailure> {
    Ok(connection.query_row(
        "SELECT count(*) > 0 FROM pragma_table_info(?1) WHERE name = ?2",
        [table, column],
        |row| row.get(0),
    )?)
}

/// **The statements that bring `failure_record_raised_over` from three columns
/// to four**, run only where the table is the three-column one.
///
/// **The old table is renamed out of the way and the new one is created under
/// the real name**, rather than the new one being created under a working name
/// and renamed into place. The engine keeps the text of a `CREATE TABLE` and
/// rewrites only the name a rename touches, so the second order would leave the
/// repaired record carrying a quoted name and a different statement from the
/// one [`VERSION_14`] makes. **This order leaves the two byte for byte the
/// same**, which is what [`super::tests`]'s crossing test asserts.
///
/// **The kind each carried-over row gets is its own record's**, read out of
/// `failure_record` — which has carried `durable_subject_kind` since the day
/// version 14 was written and never lost it. That is a recovery rather than a
/// guess: §8.2 F5's set belongs to a record, and the record knows its kind.
///
/// **A value row whose record is gone keeps its place under the empty kind.**
/// It was already unreachable — every read enters through `failure_record` —
/// and deleting it would be this migration removing something the record held,
/// which is the one thing [`TABLES_A_MIGRATION_NEVER_EMPTIES`]'s rule is about
/// even where that rule does not name this table.
const THE_VALUE_SET_GAINS_THE_SUBJECT_KIND: &[&str] = &[
    "ALTER TABLE failure_record_raised_over RENAME TO failure_record_raised_over_before_16",
    // Byte for byte [`VERSION_14`]'s own statement.
    "CREATE TABLE failure_record_raised_over (
        durable_subject TEXT NOT NULL,
        durable_subject_kind TEXT NOT NULL,
        identity_or_mark TEXT NOT NULL,
        value TEXT NOT NULL,
        PRIMARY KEY (durable_subject, durable_subject_kind, identity_or_mark, value)
     )",
    "INSERT OR IGNORE INTO failure_record_raised_over
        (durable_subject, durable_subject_kind, identity_or_mark, value)
     SELECT DISTINCT was.durable_subject,
            COALESCE(record.durable_subject_kind, ''),
            was.identity_or_mark,
            was.value
     FROM failure_record_raised_over_before_16 AS was
     LEFT JOIN failure_record AS record
            ON record.durable_subject = was.durable_subject
           AND record.identity_or_mark = was.identity_or_mark",
    "DROP TABLE failure_record_raised_over_before_16",
];

/// Version 16 — **the two tables an in-place edit left in two shapes, brought
/// to one.** Added 2026-09-13.
///
/// **This is a repair and not a feature.** No row of `State, identity and
/// recovery` §1.1's inventory arrives with it and no table is created that a
/// fresh record does not already have. What it does is make every record agree
/// with the shape its own version claims.
///
/// **What it repairs, and both were found the same way — by opening the record
/// on the Creator's own desk and reading the table.**
///
/// - **`failure_record_raised_over` is three columns on any record that crossed
///   version 14 before 2026-09-12** ([`VERSION_14`]'s note). The code reads a
///   fourth, so every read of a standing failure answered *no such column* and
///   **no failure of any kind reached a screen** on that computer, silently.
/// - **`dependency_check_standing` is missing `every_row_confirmed_or_set_aside`
///   on any record that crossed version 10 on 2026-09-04**
///   ([`VERSION_10`]'s note). The desk's record is not one of those, and no
///   record anybody can point to is — **which is exactly the kind of statement
///   this migration exists because somebody trusted.**
///
/// **It decides from the table and never from the version.** A record that
/// already has both columns gets no statement at all and crosses untouched;
/// version 16 is written into it and nothing else happens. That is the rule
/// [`VERSION_15`] set — *by shape rather than by care* — applied to a shape the
/// migration has to look at rather than one it can assume.
///
/// **Rows are carried, not lost.** The value set is rebuilt with an
/// `INSERT ... SELECT` over the record's own rows, and the added column takes a
/// default rather than a rewrite. **[`TABLES_A_MIGRATION_NEVER_EMPTIES`] is
/// untouched**: neither table is on that list, and neither of the two tables
/// that are is read, written or dropped here.
///
/// **`every_row_confirmed_or_set_aside` defaults to 0 and that is the safe
/// direction rather than a neutral one.** [`VERSION_10`]'s own note says the
/// plausible guess — that every row was confirmed — *"stops `Rendering`'s
/// warning on a file that still has unconfirmed rows"*. 0 leaves the warning
/// standing, which is what a record that never held the fact has earned.
///
/// **The repaired column carries a `DEFAULT 0` its fresh counterpart does not**,
/// and that difference is permanent and is stated here rather than tidied away:
/// the engine cannot add a `NOT NULL` column to a table that may hold rows
/// without one, and rebuilding twenty columns to hide a default would be a
/// larger act than the fault.
pub fn version_16(connection: &rusqlite::Connection) -> Result<Vec<String>, RecordFailure> {
    let mut statements: Vec<String> = Vec::new();

    if !the_table_has_the_column(
        connection,
        "failure_record_raised_over",
        "durable_subject_kind",
    )? {
        statements.extend(
            THE_VALUE_SET_GAINS_THE_SUBJECT_KIND
                .iter()
                .map(|statement| statement.to_string()),
        );
    }

    if !the_table_has_the_column(
        connection,
        "dependency_check_standing",
        "every_row_confirmed_or_set_aside",
    )? {
        statements.push(
            "ALTER TABLE dependency_check_standing
                ADD COLUMN every_row_confirmed_or_set_aside INTEGER NOT NULL DEFAULT 0"
                .to_string(),
        );
    }

    Ok(statements)
}

/// Version 17 — **the settings the user holds**: the download folder they
/// chose, the two transfer speed caps, and the four notification switches.
/// Added 2026-09-13, out of `Audits/Build/Step 10a/Where the settings are kept
/// — a reading.md`.
///
/// **`State, identity and recovery` J1, at that document's §1.1, has named
/// *settings* in this record's inventory since step 2, and §11.1 makes them
/// user-held.** No row is added to that inventory here (L1); this is the row
/// arriving with the screen that needs it saved, which is step 10a piece 11's
/// Settings screen.
///
/// **Versions 1 to 16 are left byte for byte as they are.** This is a new
/// migration and nothing else, under the ruling of 2026-09-13 in
/// `Audits/Build/Desktop app and delivery/Decisions record.md`: a shipped
/// migration is never edited again. The Creator's own record was at 16 when
/// this was written, and it was read before this version was.
///
/// **Two tables, and the reason there are two.**
///
/// - **`setting` is one row** holding the folder and the two caps. A missing
///   row and a null column both mean *the user has set nothing*, which is the
///   same shape `State, identity and recovery` I3 rule 4 gives a pane width:
///   an absent value is the default, never a stored copy of it.
///   - **The folder is only ever the one the user chose** (`Download` H2).
///     **The system's own proposal is never written here**, because §8 W3 says
///     nothing caches it: the Linux tool resets a missing downloads folder at
///     next login and a stored copy would outlive that. So null means *use the
///     proposal, read fresh*. Bytes, like every other path in this record.
///   - **Each cap is in bytes per second** (R2, and `Server protocol and
///     transfers` R11), and null is no cap, which is the default (R8). A cap
///     of nought would stop every transfer and is refused by the table as well
///     as by the screen.
/// - **`notification_switch` is one row per kind the user has turned**, keyed
///   on the kind's stored name. **The four kinds are not columns**, because a
///   closed set written once in `crate::notifications::kinds::Kind::ALL` and
///   again as four column names is the fact-in-two-places fault this stage
///   keeps meeting; [`super::settings`] holds the one spelling and a test ties
///   it to `Kind::ALL`. A kind with no row is at its default.
///
/// **L11 does not reach these tables**, which is deliberate: that rule protects
/// suppression rows, whose loss is invisible. **And no statement here carries a
/// value** (L10) — both are shapes.
pub const VERSION_17: &[&str] = &[
    // `Download` H2, `Desktop app and delivery` R2, R3 and R8.
    "CREATE TABLE setting (
        the_one_row INTEGER PRIMARY KEY NOT NULL CHECK (the_one_row = 1),
        download_folder BLOB,
        upload_cap_bytes_per_second INTEGER CHECK (upload_cap_bytes_per_second > 0),
        download_cap_bytes_per_second INTEGER CHECK (download_cap_bytes_per_second > 0)
     )",
    // `Desktop app and delivery` T8 and T17a's third bullet.
    "CREATE TABLE notification_switch (
        kind TEXT PRIMARY KEY NOT NULL,
        attempted INTEGER NOT NULL CHECK (attempted IN (0, 1))
     )",
];

/// Version 18 — **the bot Telegram was set up with, and the conversation proved
/// for it.** Added 2026-09-13 by step 10 piece 4.
///
/// **`Telegram` P1 records the proved conversation *"in the local record against
/// the bot"*, and `Telegram` S3a names the two fields beside it** — the bot's
/// identity and its username, both from the answered check. The keeper is
/// [`super::telegram_bot`].
///
/// **Versions 1 to 17 are left byte for byte as they are**, under the same
/// ruling [`VERSION_17`] names. It was tried on a copy of the Creator's own
/// record, taken outside the repository, which was at version 16.
///
/// **One row**, the same shape [`VERSION_17`]'s `setting` takes: one
/// installation has one bot, and a new bot is a new proof that replaces it
/// (`Telegram` P4). **No token is here** — `Security and licence` §9 keeps every
/// secret out of the record, and no statement here carries a value (L10). **L11
/// does not reach this table.**
pub const VERSION_18: &[&str] = &[
    // `Telegram` P1 and `Telegram` S3a.
    "CREATE TABLE telegram_bot (
        the_one_row INTEGER PRIMARY KEY NOT NULL CHECK (the_one_row = 1),
        bot_id INTEGER NOT NULL,
        username TEXT NOT NULL,
        proved_conversation INTEGER NOT NULL
     )",
];

/// Version 19 — **`Telegram`'s told-record**, step 10 piece 5, 2026-09-13.
///
/// **`State, identity and recovery` §1.1 names the desktop's told-record per
/// server** in its inventory, and `Telegram` L1 says what an entry holds: per
/// machine and per told thing, the value last acknowledged, the moment, and the
/// installation identity it was told under. **Every value is Smoozy's own act
/// of telling and never a machine's reading** (`State, identity and recovery`
/// D4). **No token is here**, only a generation (`Security and licence` §9).
///
/// **The current generation is one row beside it**, because `Telegram` L4's
/// *marks every machine as holding the old generation* needs the generation the
/// store now holds to be durable, and version 18 is written.
///
/// **The trigger is what makes a server's removal reach its told entries.**
/// `Servers` D6 makes that module's enumeration the only route to anything
/// about a server, and the enumeration has no told-record kind; a told entry
/// left behind by a removal would be a durable thing about a server outside it,
/// which `Servers` D7 calls a fault. **The removal walk is another file's and is
/// not edited here**, so the record removes the entries itself in the same
/// statement that removes the row. The enumeration's want of a fifth kind is
/// recorded for the runner.
pub const VERSION_19: &[&str] = &[
    // `Telegram` L4.
    "CREATE TABLE telegram_token_generation (
        the_one_row INTEGER PRIMARY KEY NOT NULL CHECK (the_one_row = 1),
        generation INTEGER NOT NULL CHECK (generation >= 1)
     )",
    // `Telegram` L1.
    "CREATE TABLE told_entry (
        server TEXT NOT NULL,
        thing TEXT NOT NULL,
        value TEXT NOT NULL,
        acknowledged_at INTEGER NOT NULL,
        installation TEXT NOT NULL,
        PRIMARY KEY (server, thing)
     )",
    "CREATE TRIGGER a_told_entry_goes_with_its_server AFTER DELETE ON server_row
     BEGIN
        DELETE FROM told_entry WHERE server = old.identifier;
     END",
];

/// Version 20 — **the Telegram switch and what a job reports by default**, step
/// 10 piece 8, 2026-09-14.
///
/// **Both are settings the plans already name and version 17 gave no home.**
/// `Telegram` V1a: the value a job's reporting choice starts from *"is one item
/// of the local record's settings"*, user-held. `Design/Settings § Telegram`
/// draws a switch, and `Security and licence` §5.2 erases the token when it is
/// turned off, so whether it is on cannot be read from the bot row (which stays
/// for `Telegram` P4) or from the keychain (a call on every opening of the
/// screen). **One row, and no row is the default**: never switched on, and no
/// default chosen.
///
/// **The reporting choice is stored as the page's own identifier and the table
/// does not list the three**, because the closed set is written once, in
/// `crate::surface::render_settings::REPORTING_CHOICES`, and a test holds the
/// keeper to it.
pub const VERSION_20: &[&str] = &[
    // `Telegram` V1a, and `Design/Settings § Telegram`.
    "CREATE TABLE telegram_setting (
        the_one_row INTEGER PRIMARY KEY NOT NULL CHECK (the_one_row = 1),
        switched_on INTEGER NOT NULL DEFAULT 0 CHECK (switched_on IN (0, 1)),
        reporting_default TEXT
     )",
];

/// Version 21 — **version 19's trigger removed, because a server's removal now
/// reaches its told entries by its own walk**, 2026-09-14.
///
/// Version 19 carried `a_told_entry_goes_with_its_server` because `Servers`'
/// enumeration had no told-record kind and the removal walk was not that piece's
/// to edit. **The enumeration now names one**
/// (`smoozy_servers::store::KeyedByTheServer::ToldRecord`, on `Telegram` L1 and
/// `Servers` D7), and the walk in `super::server_rows` reaches `told_entry`
/// through it.
///
/// **The trigger goes rather than staying as a second guard.** `Servers` D6 makes
/// the enumeration *"the only route by which anything about a server is
/// reached"*, met *"by making the enumeration the structure rather than a list a
/// builder keeps in step by hand"*. A trigger is a second route kept in step by
/// hand, and while it stood no test could tell whether the walk reached the
/// entries at all. The walk is the only thing that deletes a server row.
/// **Version 19 is not edited**; this version undoes its one statement.
pub const VERSION_21: &[&str] = &["DROP TRIGGER IF EXISTS a_told_entry_goes_with_its_server"];

/// Version 22 — **that the bot token was stored, and for which bot**, 2026-09-14,
/// the round *setting it up, and what the desktop keeps*, fix-here item 2.
///
/// **`Security and licence` §3.0 rule 6: *"That an item is remembered is
/// answered from the local record's own subject."*** Until this version the
/// Telegram section drew a token as held whenever the switch was on, so a
/// replacement or a new bot whose save failed was still drawn as held, and
/// material for a server could pair the old bot's token with the new
/// conversation. **One row, written only when a save succeeds**, cleared when
/// the token's item is erased or a different bot's proof is recorded. **No
/// token is here**, only the bot's identity (`Security and licence` §9); no
/// statement carries a value (L10). **Versions 1 to 21 are not edited.**
pub const VERSION_22: &[&str] = &[
    // `Security and licence` §3.0 rule 6, and `Telegram` K1.
    "CREATE TABLE telegram_token_stored (
        the_one_row INTEGER PRIMARY KEY NOT NULL CHECK (the_one_row = 1),
        bot_id INTEGER NOT NULL
     )",
];

/// Version 23 — **the local job record, and it is the first table this store
/// has ever had for one**, 2026-09-18, step 10b piece 6.
///
/// **Nothing anywhere in Smoozy wrote a job record before this version**, which
/// is why the Queue and History screens were built and permanently empty. The
/// record is `Queue & History` N10 to `Queue & History` N22's
/// `LocalJobRecord` — Smoozy's own
/// note of Smoozy's own submission — and it is the record history *is*
/// (`Queue & History` C10, `Queue & History` C11).
///
/// **The machine's per-job note carries its half into columns of this same row
/// rather than into a kind of its own**, on the build runner's ruling of
/// 2026-09-17: `State, identity and recovery` J1's inventory already puts
/// `Dependency Upload & Remap`'s per-job resolved set *"as a field of the job's
/// own record rather than a sixth kind"*, `State, identity and recovery` J5
/// leaves no second state store to put one in, and `Queue & History` N2 is the
/// argument that one fact must not have two disagreeing homes. **The
/// `note_` columns are that half.**
///
/// **L1 holds: no row is added to `State, identity and recovery` §1.1's
/// inventory here.** That inventory already names *"job records — provisional
/// and confirmed"* and *"history records and their tombstones"*; what this
/// version does is give the named row a table at last. The ruling's own seam is
/// recorded at §22 item 14 rather than written into the inventory.
///
/// **The four tables beside the row are the repeated fields**, each a set the
/// record carries more than one of: `Queue & History` N21's disturbances,
/// appended and never replaced; `Queue & History` N9a's per-attempt statement;
/// `Queue & History` N1's re-run marks; and `Queue & History` N14's view-layer
/// pins. **Rows rather than a packed field**, which is the shape
/// `dependency_located_answer_candidate` already uses and the reason `Servers`
/// D6 gives for it.
///
/// **No secret value is written** (L10), and **no field of the record is placed
/// in a filename** (L4) — every one of them is a column.
///
/// **Versions 1 to 22 are not edited.**
pub const VERSION_23: &[&str] = &[
    // **The record itself.** `Queue & History` N10 to `Queue & History` N22,
    // with the note's
    // carried half in the `note_` columns.
    "CREATE TABLE job_record (
        job TEXT PRIMARY KEY NOT NULL,
        server TEXT NOT NULL,
        job_group TEXT,
        standing TEXT NOT NULL CHECK (standing IN ('provisional', 'confirmed')),
        render_start_learned INTEGER NOT NULL CHECK (render_start_learned IN (0, 1)),
        server_label TEXT,
        server_hue INTEGER,
        submitted_at INTEGER,
        label TEXT,
        blend_path BLOB,
        scene_name TEXT,
        submitted_with_no_scene_named INTEGER NOT NULL CHECK (submitted_with_no_scene_named IN (0, 1)),
        resolution_percentage_typed INTEGER,
        picture_width INTEGER,
        picture_height INTEGER,
        main_output TEXT,
        blender_build_path TEXT NOT NULL,
        blender_version TEXT NOT NULL,
        blender_origin TEXT NOT NULL,
        device_ask TEXT NOT NULL,
        device_choice TEXT,
        dependency_check TEXT,
        carries_on_from_job TEXT,
        carries_on_from_label TEXT,
        note_started INTEGER,
        note_started_machine TEXT,
        note_ended INTEGER,
        note_ended_machine TEXT,
        note_ending_outcome TEXT,
        note_ending_exit_status INTEGER,
        note_ending_capture_attempt INTEGER,
        note_attempts INTEGER NOT NULL DEFAULT 0,
        note_given_up_where_it_was INTEGER,
        note_last_frame_figure INTEGER,
        file_values_stood INTEGER NOT NULL CHECK (file_values_stood IN (0, 1)),
        file_values_frame_start INTEGER,
        file_values_frame_end INTEGER,
        file_values_frame_step INTEGER,
        file_values_sample_count INTEGER,
        file_values_resolution_percentage INTEGER,
        file_values_engine TEXT,
        file_values_device TEXT,
        file_values_use_simplify INTEGER,
        file_values_subdivision_render INTEGER,
        file_values_child_particles_render REAL,
        file_values_texture_limit_render TEXT
     )",
    // **`Queue & History` N21 — per disturbance, appended and never replaced**:
    // one job's files may be touched more than once and by more than one act.
    "CREATE TABLE job_record_disturbance (
        job TEXT NOT NULL,
        position INTEGER NOT NULL,
        member TEXT NOT NULL,
        act_intended TEXT NOT NULL,
        reading TEXT NOT NULL,
        PRIMARY KEY (job, position)
     )",
    // **`Queue & History` N9a — per attempt, and never per job.**
    "CREATE TABLE job_record_file_changed (
        job TEXT NOT NULL,
        attempt INTEGER NOT NULL,
        a_frame_an_earlier_attempt_wrote_was_kept INTEGER NOT NULL
            CHECK (a_frame_an_earlier_attempt_wrote_was_kept IN (0, 1)),
        PRIMARY KEY (job, attempt)
     )",
    // **`Queue & History` N1's re-run marks**, carried over by the fill-in:
    // the attempt, and the frame the re-run proceeded from where its list had
    // one. **Absent is not a zero.**
    "CREATE TABLE job_record_re_run_mark (
        job TEXT NOT NULL,
        attempt INTEGER NOT NULL,
        proceeded_from INTEGER,
        PRIMARY KEY (job, attempt)
     )",
    // **`Queue & History` N14's view-layer pins** — the file's own value for
    // each pinned switch the payload carries an override for, in the order the
    // companion composed them.
    "CREATE TABLE job_record_file_value_pin (
        job TEXT NOT NULL,
        position INTEGER NOT NULL,
        view_layer TEXT NOT NULL,
        switch TEXT NOT NULL,
        switch_value INTEGER,
        PRIMARY KEY (job, position)
     )",
];

/// Version 24 — **the mark that a job's start was never heard**, 2026-09-18,
/// step 10b round 1 of *jobs, history and downloads*, item C1.
///
/// *Creator-stated, 2026-09-18:* a job Smoozy submitted and never learned had
/// started, whose server is then destroyed or removed, is kept and shown in
/// History, marked that Smoozy never heard whether it started. **So a record
/// whose render start was never learned is no longer removed with its server**:
/// `Queue & History` C9's second way in marks it here instead, at the removal
/// (`Servers` D17) or at `State, identity and recovery` §7 V1 step 1's absent
/// or different branch, and History draws it from the mark.
///
/// **It is a column of its own and never `render_start_learned` set to one.**
/// That column says the start was learned, and writing it for a start nobody
/// heard of would put a false fact in the record the entry is drawn from. Every
/// record written before this version reads nought, which is exactly what it
/// was: no record carried the mark, because the removal took every one that
/// would have.
///
/// **Versions 1 to 23 are not edited**, and the column's default is what makes
/// this an addition rather than a rewrite — L11, L12 and L12a are untouched.
pub const VERSION_24: &[&str] = &[
    "ALTER TABLE job_record ADD COLUMN start_never_heard INTEGER NOT NULL DEFAULT 0
        CHECK (start_never_heard IN (0, 1))",
];

/// Version 25 — **the kept setup, written beside the job record**, step 10c
/// piece 6, on the runner's R4 for that step (corrected to 25 on 2026-09-19,
/// version 24 being step 10b's).
///
/// `Queue & History` N10 and `Queue & History` N17 put the kept setup on the local job record —
/// *"what submit again opens the page from"* — and `Rendering` A1 and
/// `Render Settings` Y15 say what it is: **the payload the job was submitted
/// with**. Version 23 held four of its facts and no column for the rest, and
/// the runner ruled under step 10b piece 6a that the column lands with its
/// writer, which is the submission. **Records written before the writer have
/// no kept setup and never will**, so the column and the writer arrive
/// together; a record from before this version reads nothing, which is what
/// it holds.
///
/// **One column holding the payload as `Render Settings` carries it**, the
/// same form the manifest's settings-payload field carries it in on the
/// machine (`Job files and outputs` M1), and read back through that crate's
/// own reader. **The payload's type belongs to `Render Settings` and this
/// store does not take it apart**, so a column per entry would be a second
/// statement of a shape that crate owns.
///
/// **Versions 1 to 24 are not edited**, and the column is nullable, so this
/// is an addition and not a rewrite — L11, L12 and L12a are untouched.
pub const VERSION_25: &[&str] = &["ALTER TABLE job_record ADD COLUMN kept_setup TEXT"];

/// Version 26 — **`Download`'s two durable stores**, step 10c piece 10, on the
/// runner's R4 for that step as it was corrected on 2026-09-19 (piece 6 took
/// 25, so these are 26).
///
/// **Two stores and no third.** `Download` R1's auto-download arrangement,
/// held against the local job record, and `Download` M1's manual download
/// request. `State, identity and recovery` §1.1's inventory already names
/// *transfer arrangements* here, and §11.1 lists them **user-held** — so
/// neither store has an age, a cap or a sweep, and both end when the user's own
/// act ends them.
///
/// **The arrangement is keyed by the job and the request by its record
/// identity**, because that is what each of them is one of: `Download` R1 makes
/// an arrangement one record per job, and `Download` M1 makes a request one
/// `State, identity and recovery` §9.4 outcome record. **So the request's units
/// are written into the shared `outcome_record` and `outcome_record_unit`
/// tables**, joined by `(record, position)`, exactly as `File Browser` U1's
/// upload request already is in [`VERSION_9`] — one enumeration rather than two
/// copies of one.
///
/// **The arrangement has no §9.4 half and is not given one.** `Download` R1
/// does not call it a multi-part action: it is a standing intention whose
/// entries re-open (`Download` R4's two re-opening states), and a §9.4 unit
/// outcome is immutable once written (`State, identity and recovery` §8.6). A
/// shared record would therefore refuse the one write the arrangement's own
/// clause requires.
///
/// **Every path is `BLOB` and never `TEXT`.** `Job files and outputs` N15 makes
/// a machine's names bytes, `File Browser` L1a says nothing is rewritten in
/// flight, and `Download` R4 keys the reckoning on the predicted path's own
/// bytes. A text column would put a decode between the machine and the record,
/// and a substituted byte keys the wrong entry.
///
/// **The four-value reading is written with its moment, its clock and the
/// installation identity it was read under** (`Download` R4's second half,
/// `State, identity and recovery` D5 and §2.3), so it can be disqualified at a
/// read rather than deleted by a sweep. **The account half beside it carries no
/// installation**, because that half is Smoozy's own history and is never
/// disqualified by anything the machine does.
///
/// **Versions 1 to 25 are not edited.** Five tables are created and nothing
/// existing is touched — L11, L12 and L12a are untouched.
///
/// **L4 holds: no field of either store is ever placed in a filename. L10
/// holds: no secret value is written.**
pub const VERSION_26: &[&str] = &[
    // **`Download` R1's record-level fields.** One row per job, and the
    // narrowing's two shapes are told apart by `narrowing`: *everything* is the
    // declared value and is no filter rather than a frozen list (`Download` N3),
    // so a row reading *everything* has no format rows at all and a row reading
    // *these formats* has its own.
    "CREATE TABLE download_arrangement (
        job TEXT PRIMARY KEY NOT NULL,
        server_row TEXT NOT NULL,
        destination BLOB NOT NULL,
        narrowing TEXT NOT NULL CHECK (narrowing IN ('everything', 'these formats')),
        desired_state TEXT NOT NULL CHECK (desired_state IN ('on', 'off'))
     )",
    // **`Download` R1's narrowing, as rows rather than a packed field**, on
    // `Servers` D6's rule. A format name is the reading's own bytes
    // (`Download` N1) and is never decoded here.
    "CREATE TABLE download_arrangement_format (
        job TEXT NOT NULL,
        position INTEGER NOT NULL,
        format BLOB NOT NULL,
        PRIMARY KEY (job, position)
     )",
    // **`Download` R4's per-file reckoning: one entry per file the arrangement
    // has acted on, keyed by the predicted path relative to the job output
    // folder — the inventory's own key, and nothing else.** **The attempt is
    // not in the key**, which is that clause in the table's own shape: there is
    // no second row for a second attempt to land in.
    //
    // **The two halves have two standings and the columns keep them apart.**
    // `account_state`, `transfer` and `destination_in_force` are Smoozy's own
    // history; the four values and their moment, clock and installation are the
    // machine-derived half, disqualified at a read under `State, identity and
    // recovery` §2.3 while the account half stands.
    "CREATE TABLE download_arrangement_entry (
        job TEXT NOT NULL,
        predicted_path BLOB NOT NULL,
        frame INTEGER,
        account_state TEXT NOT NULL,
        transfer TEXT,
        destination_in_force BLOB NOT NULL,
        source_size INTEGER,
        source_inode TEXT,
        source_modification_time INTEGER,
        source_change_time INTEGER,
        read_at INTEGER,
        read_on_machine TEXT,
        read_under TEXT,
        PRIMARY KEY (job, predicted_path)
     )",
    // **`Download` M1's record-level fields.** One row per manual request,
    // beside the §9.4 outcome record of the same identity, which is where its
    // unit outcomes live. **`opened_by_run` is M13's run marker**: closing
    // Smoozy ends the request, and a fresh sitting reads the marker rather than
    // resuming anything.
    "CREATE TABLE download_request (
        record TEXT PRIMARY KEY NOT NULL,
        server_row TEXT NOT NULL,
        installation TEXT,
        destination BLOB NOT NULL,
        collision_answer TEXT,
        opened_by_run TEXT NOT NULL
     )",
    // **`Download` M1's enumeration entry.** `position` is §9.4's own ordering
    // and is what joins this to `outcome_record_unit`.
    //
    // **`refusal` is `Download` P8's per-file name refusal** and stands beside
    // the disposition rather than inside it, because an entry may carry the
    // refusal before any answer has been applied.
    "CREATE TABLE download_request_entry (
        record TEXT NOT NULL,
        position INTEGER NOT NULL,
        source_path BLOB NOT NULL,
        landing_path BLOB NOT NULL,
        size_as_enumerated INTEGER NOT NULL,
        refusal TEXT,
        vouching TEXT NOT NULL,
        disposition TEXT,
        keep_both_landing_path BLOB,
        disposition_refusal TEXT,
        transfer TEXT,
        PRIMARY KEY (record, position)
     )",
    // **`Download` M4's recorded refusals, per subtree.** They are counted in
    // neither of M16's totals and are reported as what could not be read, so
    // they are rows of their own and never entries with an outcome.
    //
    // **`what_the_server_said` carries the protocol's own answer as itself**,
    // for the one cause that is none of `File Browser` P1's three:
    // `Server protocol and transfers` §1.3 says a status code is a number the
    // draft defines rather than text nobody specified, so the cause survives
    // the write separated instead of being flattened into the three.
    "CREATE TABLE download_request_refusal (
        record TEXT NOT NULL,
        position INTEGER NOT NULL,
        at_path BLOB NOT NULL,
        what_smoozy_met TEXT NOT NULL,
        what_the_server_said INTEGER,
        PRIMARY KEY (record, position)
     )",
];

/// Version 27 — **the reason the machine recorded for a job's ending**, out of
/// step 10c's sixth pass, fault 3.
///
/// **What it is for.** `Rendering` K14 records the reason for every *failed*
/// ending verbatim, on the machine, and `Render Settings`' own account of a
/// name the reading cannot anchor is that *"what the user meets is an ordinary
/// failed job entry with the reason recorded"*. The sixth pass met two such
/// endings — a card make with no technology to name, and a write place with no
/// setting name — and **History said only that the render did not finish**,
/// because the reason never left the machine. **This column is the desktop's
/// half**: where a connection carries the reason in, it is kept here beside
/// the record, survives the machine the way the rest of the entry does
/// (`Queue & History` C11), and the entry puts it behind *What the server
/// said*.
///
/// **One nullable column**, because the reason is one string recorded whole and
/// shown whole (`Desktop app and delivery` M9). **Absent is the ordinary
/// state** — a job that has not ended, one that finished, and every record
/// written before a machine carried a reason.
///
/// **Versions 1 to 26 are not edited**, and nothing existing is touched — L11,
/// L12 and L12a are untouched.
pub const VERSION_27: &[&str] =
    &["ALTER TABLE job_record ADD COLUMN the_reason_the_machine_recorded TEXT"];

/// Version 28 — **the device line the machine observed for a job's ending**,
/// out of step 10c's fix piece 2.
///
/// **What it is for.** History read *Ran on — not known* on every job of the
/// seventh passes, on all four providers, though each machine's render log
/// named the card: the render worker observed the line at its own loop
/// (`Queue & History` E13) and nothing carried it off the machine.
/// the paragraph after `Queue & History` N3 now has the note's ending carry
/// it, and **this column is the desktop's half**: the line is kept beside the
/// record, verbatim, beside version 27's reason, and survives the machine the
/// way the rest of the entry does (`Queue & History` C11).
///
/// **Kept on whatever ending it was carried on, and read on none but a
/// finished one**: `Queue & History` E12 makes a line from a run that did not
/// finish no evidence, and that judgement is made where the entry is drawn,
/// never by leaving the line out here.
///
/// **One nullable column**, because the line is one string recorded whole.
/// **Absent is the ordinary state** — a job that has not ended, an ending on
/// which no line passed, and every record written before a machine carried
/// one.
///
/// **Versions 1 to 27 are not edited**, and nothing existing is touched — L11,
/// L12 and L12a are untouched.
pub const VERSION_28: &[&str] =
    &["ALTER TABLE job_record ADD COLUMN the_device_line_as_observed TEXT"];

/// Version 29 — **what the check read each reference as**, out of step 10c's
/// dependency wave piece 7b.
///
/// **What it is for.** `Dependency Upload & Remap` M4 puts a kind and a
/// setting in every remap pair and its M10 composes the pairs from the record
/// at submission. Its K4 held neither until 2026-09-29, so the pairs could be
/// composed only in the run that did the check, and after a restart a checked
/// and sent project was submitted with none. **K4 now holds every kind and
/// setting the latest reading of the holding file carried the reference
/// under**, and this table is where.
///
/// **Rows rather than a packed field**, on `Servers` D6's rule and in the shape
/// [`VERSION_10`]'s `dependency_located_answer_candidate` already has: keyed
/// by the answer's own key (`Dependency Upload & Remap` K3) and a position, in
/// the reading's order. **Replaced whole with the answer it belongs to**, like
/// the candidates. The kind is the reading's own spelling
/// (`smoozy_dependencies::check::Kind`'s), and the setting is a property path,
/// text.
///
/// **No row for an answer is the ordinary state for every answer written
/// before this version**: such an answer gives no pair until the file is
/// checked again (the K4 sentence says so). Nothing shipped ever wrote a
/// located answer — the keeper arrived after 0.1.6 — so no user's record
/// holds one.
///
/// **Versions 1 to 28 are not edited**, and nothing existing is touched — L11,
/// L12 and L12a are untouched.
pub const VERSION_29: &[&str] = &["CREATE TABLE dependency_located_answer_read_as (
        holding_kind TEXT NOT NULL,
        holding_source_path BLOB NOT NULL,
        holding_machine_location BLOB NOT NULL,
        holding_server_row TEXT NOT NULL,
        holding_installation TEXT NOT NULL,
        reference BLOB NOT NULL,
        position INTEGER NOT NULL,
        kind TEXT NOT NULL,
        setting TEXT NOT NULL,
        PRIMARY KEY (
            holding_kind,
            holding_source_path,
            holding_machine_location,
            holding_server_row,
            holding_installation,
            reference,
            position
        )
     )"];

/// Version 30 — **the *Show hidden files* choice**, out of Row 520.
///
/// **What it is for.** `File Browser` L20 makes the choice *"one setting in the
/// local record — `State, identity and recovery` J1's *settings*, named at its
/// `State, identity and recovery` I7"*: app-wide and never keyed by server row, so **a column on the one-row
/// `setting` table** [`VERSION_17`] made, beside the download folder and the
/// two speed caps. No new store.
///
/// **Absent is the ordinary state and means off** (`File Browser` L20): the
/// tick writes `1` and the untick clears the column back to nothing rather than
/// writing a copy of the default — `State, identity and recovery` I3 rule 4's
/// shape. **The table refuses any other value**, so a nought or a two can never
/// arrive through some other door and be read as a choice the user made.
///
/// **Every record written before this version reads as off**, which is the
/// plan's own default and the drawing's unticked box.
///
/// **Versions 1 to 29 are not edited**, and nothing existing is touched — L11,
/// L12 and L12a are untouched.
pub const VERSION_30: &[&str] =
    &["ALTER TABLE setting ADD COLUMN hidden_files_shown INTEGER CHECK (hidden_files_shown = 1)"];

/// Version 31 — **`File Browser` §11's record for a delete, a move and a
/// copy**, out of step 10c's Files' owed acts piece 4.
///
/// **What it is for.** `File Browser` §11 (its paragraph of 2026-10-02) gives
/// those three acts *"one record of the same kind"* as an upload's: per unit,
/// at its position, the source, the destination and — for a copy — the
/// staging path, keyed by the act's operation identity and user-held. **The
/// plan names no table**, and the shape is [`VERSION_9`]'s for `File Browser`
/// U1: the shared `outcome_record` and `outcome_record_unit` hold the act and
/// each unit's outcome, and the module's own fields sit in tables of its own
/// keyed to that enumeration by `(record, position)`. One enumeration, joined
/// rather than copied.
///
/// **The telling is rows and its emptiness is its absence** (`File Browser`
/// W8), as [`VERSION_9`]'s is.
///
/// **Every path is `BLOB`**, for the reason [`VERSION_9`] gives: a name on the
/// machine is bytes and `File Browser` X5's comparison is byte for byte.
///
/// **Versions 1 to 30 are not edited**, and nothing existing is touched.
pub const VERSION_31: &[&str] = &[
    "CREATE TABLE file_act (
        record TEXT PRIMARY KEY NOT NULL,
        act TEXT NOT NULL CHECK (act IN ('delete', 'move', 'copy')),
        server_row TEXT NOT NULL,
        installation TEXT,
        opened_by_run TEXT NOT NULL
     )",
    "CREATE TABLE file_act_unit (
        record TEXT NOT NULL,
        position INTEGER NOT NULL,
        source BLOB NOT NULL,
        destination BLOB,
        staging BLOB,
        PRIMARY KEY (record, position)
     )",
    "CREATE TABLE file_act_telling_job (
        record TEXT NOT NULL,
        position INTEGER NOT NULL,
        job TEXT NOT NULL,
        PRIMARY KEY (record, position)
     )",
];

/// Version 32 — **how many files one delete took from a job's folder**, out of
/// the build runner's ruling (d) of 2026-10-03, written into `Design/Queue &
/// History § The marks`.
///
/// **What it is for.** A delete inside a job's folder leaves one mark per
/// delete act: the file it names where one file went, and the folder with the
/// count where more did — never one mark per file, and never the job's whole
/// folder for a single file. The member column already names the file or the
/// folder; the count had no home, so it is **a column on `Queue & History`
/// N21's own table** rather than a second store.
///
/// **Absent is every mark but that one**, and every mark written before this
/// version reads as it always did. **The table refuses a count of one or
/// less**: one file is named, never counted.
///
/// **Versions 1 to 31 are not edited**, and nothing existing is touched.
pub const VERSION_32: &[&str] = &[
    "ALTER TABLE job_record_disturbance ADD COLUMN how_many INTEGER CHECK (how_many IS NULL OR how_many > 1)",
];

/// Version 33 — **`Queue & History` W5's idle-spoken mark**, out of the 38's
/// piece 1 (2026-10-03).
///
/// **What it is for.** `Queue & History` W5: *"The mark is one small record in the local record,
/// subject-held on the server row, carrying the installation identity and the
/// event identity of the recorded event that opened the stretch."* It is looked
/// up before the idle notification is posted and written by that act alone, so
/// a stretch is told about once across restarts. **`Queue & History`'s opening
/// puts it in the local record** and `State, identity and recovery` §1.1 names
/// it; no plan names the table, and the shape is this one (hole 1 of the 38's
/// map, ruled as the map leans: one table).
///
/// **The opening event is absent for `Queue & History` W2's floor** — a machine that never had
/// work, whose stretch is keyed on the installation alone — and absent is a
/// null, never an empty string standing for one. **The key is the server, the
/// installation and the opening event together**, and a null opening event is
/// compared as null by the reader rather than made unique by the table.
///
/// **Subject-held on the server row**: `server` is the local server identifier,
/// and the row's removal walk takes these with it (`Servers` D4's list, widened
/// at Build for this record as it was for the told-record).
///
/// **Versions 1 to 32 are not edited**, and nothing existing is touched.
pub const VERSION_33: &[&str] = &["CREATE TABLE idle_spoken_mark (
        server TEXT NOT NULL,
        installation TEXT NOT NULL,
        opening_event TEXT
     )"];

/// Version 34 — **`Queue & History` E9's kept words**, out of the Creator's
/// notes on 0.1.8, piece 6d (2026-10-03).
///
/// **What it is for.** E9: *"on a connection that learns a job's ending, the
/// desktop fetches both captured streams and the per-stream collection
/// outcomes into the job's history record, whole or not at all, kept apart,
/// unedited"* — every attempt that carries a capture record, never the ending
/// attempt alone. Until this version the record had nowhere to keep them, and
/// History's *Blender output* answered *not built*.
///
/// **A table of its own, one row per stream per attempt** (`Rented-server
/// runtime` M19: a re-run's words are the new attempt's own, never merged),
/// **beside the job record** and named in
/// [`super::job_records::EVERY_SET_BESIDE_THE_ROW`], so a deletion takes the
/// words with the entry and a migration may never empty it (L12a). The bytes
/// are kept as they arrived, in one column, and nothing reads them.
///
/// - `outcome` is `Rented-server runtime` M13's, in that crate's own words;
///   **absent is `Desktop app and delivery` M10a's outcome not yet written**,
///   never one of the four.
/// - `retained`, `discarded` and `why_the_rest_went` are M5's, absent where
///   the machine reported none.
///
/// **`words_owed` on the record is the debt the connection that learns the
/// ending takes on**: set in the same act the fill-in writes the ending, and
/// cleared in the same act the words land, or when the machine says it holds
/// no such job. Every record written before this version reads nought —
/// its ending's reconciliation has passed, and nothing is fetched for it.
///
/// **Versions 1 to 33 are not edited**, and nothing existing is touched.
pub const VERSION_34: &[&str] = &[
    "ALTER TABLE job_record ADD COLUMN words_owed INTEGER NOT NULL DEFAULT 0
        CHECK (words_owed IN (0, 1))",
    "CREATE TABLE job_record_words (
        job TEXT NOT NULL,
        attempt INTEGER NOT NULL,
        stream TEXT NOT NULL CHECK (stream IN ('ordinary', 'complaints')),
        outcome TEXT,
        the_words BLOB NOT NULL,
        retained INTEGER,
        discarded INTEGER,
        why_the_rest_went TEXT,
        PRIMARY KEY (job, attempt, stream)
     )",
];

/// Version 35 — **what the job record directory tells of a job's frames**, out
/// of the 38's piece 10 (2026-10-03).
///
/// **What it is for.** Design, *History*: *"The frame count is the last frame
/// Smoozy confirmed, not the range the job was submitted with. A job that
/// failed at 61 reads 60/240."* The note carries the figure (`Queue & History`
/// N6) and nothing on the record carried the frames the job asked for, so every
/// History row drew a dash. And a job whose scene writes more than one view
/// (`Job files and outputs` I9a) shows no frame count at all, which the record
/// had no field to say.
///
/// - `frames_asked` is the asked order as runs (`Queue & History`'s
///   `TheFramesAsked`), written in this crate's own words by
///   `super::job_records`; absent where no read has landed.
/// - `frames_cannot_be_counted` is nought on every record written before this
///   version, which is the truthful default: nothing said they could not be.
///
/// **Two columns on the job record's own row**, filled by `Queue & History`
/// N9's fill-in alone (`LocalJobRecord::the_read_fills_what_it_lacks`).
/// **Versions 1 to 34 are not edited**, and nothing existing is touched.
pub const VERSION_35: &[&str] = &[
    "ALTER TABLE job_record ADD COLUMN frames_asked TEXT",
    "ALTER TABLE job_record ADD COLUMN frames_cannot_be_counted INTEGER NOT NULL DEFAULT 0
        CHECK (frames_cannot_be_counted IN (0, 1))",
];

/// Version 36 — the historical pending-key refusal marker (2026-10-03).
/// Shipped migration retained unchanged; its table is dropped by upgrade cleanup.
pub const VERSION_36: &[&str] = &[
    "ALTER TABLE licence_record ADD COLUMN pending_answered_not_valid INTEGER NOT NULL DEFAULT 0
        CHECK (pending_answered_not_valid IN (0, 1))",
];

/// Version 37 — **`Queue & History` P4's fetched derivatives, held on the job
/// record**, out of the 38's piece 3, desktop half (2026-10-03).
///
/// **What it is for.** `Queue & History` P3 and P4: each derivative's
/// production is journalled, the desktop fetches what it learns of — one fetch
/// per derivative — and *"holds it subject-held on the job record"*; R13 of
/// `State, identity and recovery` records **one moment beside the copy**, when
/// it was published on this computer, by this computer's clock, which is the
/// age ending's whole input (that document's R16 and R19).
///
/// **One row per frame of a job**, beside the job record and named in
/// [`super::job_records::EVERY_SET_BESIDE_THE_ROW`], so a removal takes it with
/// the entry and a migration may never empty it:
///
/// - `operation` is the machine's production record the event named, and
///   `view_layer` P10a's name as the bytes it is, absent where none is carried;
/// - `owed` is the fetch the event put on the job — **written before the cursor
///   moves past the event** (`State, identity and recovery` E3), so a fetch a
///   closed app never made is still owed at the next connection;
/// - `copy` and `published_on_this_computer_at` are the held copy and its one
///   moment, in milliseconds, absent until the first fetch publishes. **The
///   copy's name is issued fresh for each fetch** and no field of the record
///   is placed in it (`Desktop app and delivery` L4).
///
/// **Versions 1 to 36 are not edited**, and nothing existing is touched.
pub const VERSION_37: &[&str] = &["CREATE TABLE job_record_preview (
        job TEXT NOT NULL,
        frame INTEGER NOT NULL,
        operation TEXT NOT NULL,
        view_layer BLOB,
        owed INTEGER NOT NULL CHECK (owed IN (0, 1)),
        copy TEXT,
        published_on_this_computer_at INTEGER,
        PRIMARY KEY (job, frame)
     )"];

/// Version 38 — **`Queue & History` C2's catch-up notice record**, out of the
/// 38's piece 15 (2026-10-04).
///
/// **What it is for.** `Queue & History` C2: *"Applying step 3 writes the notice's content into
/// the local record as part of the application, before the cursor advances
/// past the events it names"* — so the account survives a restart and stands
/// until the user's own *Dismiss*. **Until this version it was held for the run
/// alone**: an account not read before Smoozy quit was lost, and *Dismiss* had
/// nothing to act on.
///
/// **Two tables, because `Queue & History` C1a's record can stand with no
/// line in it**: `Queue & History` C4's gap
/// is said on its own where the journal's cap passed the cursor and nothing
/// else arrived.
///
/// - `catch_up_notice` is the one standing record per machine, keyed by the
///   local server identifier, with `Queue & History` C4's statement.
/// - `catch_up_notice_entry` is one line of it: the job and the event's own
///   sequence number, which together recognise a line already held (`Queue & History` C2's
///   *"composes a notice only of what is new"*), and `place`, the order the
///   lines were composed in. `kind` says which of `Queue & History` C1's list it is; the columns
///   beside it are the ones that kind carries and are absent otherwise —
///   `ending` for an ending, `attempt` and `frame` for a re-run (`frame` absent
///   until that attempt's own list is read, never a zero), `why` and `name`
///   for a held job dropped (`Queue & History` H11: **the name is kept here
///   because the drop took every other record that held it**), and `name`
///   and `version` for a waiting job dropped for a removed Blender (`Queue & History` L22's
///   label and answered version).
///
/// **Subject-held on the server row**: the row's removal walk takes both with
/// it (`Servers` D4's list, widened at Build for this record as it was for the
/// told-record and the idle-spoken mark). **Versions 1 to 37 are not edited**,
/// and nothing existing is touched.
pub const VERSION_38: &[&str] = &[
    "CREATE TABLE catch_up_notice (
        server TEXT PRIMARY KEY,
        the_account_is_incomplete INTEGER NOT NULL
            CHECK (the_account_is_incomplete IN (0, 1))
     )",
    "CREATE TABLE catch_up_notice_entry (
        server TEXT NOT NULL,
        job TEXT NOT NULL,
        sequence INTEGER NOT NULL,
        place INTEGER NOT NULL,
        kind TEXT NOT NULL CHECK (kind IN
            ('ended', 're-run', 'held job dropped', 'dropped for a removed blender')),
        ending TEXT,
        attempt INTEGER,
        frame INTEGER,
        why TEXT,
        name TEXT,
        version TEXT,
        PRIMARY KEY (server, job, sequence)
     )",
];

/// Version 39 — **where a job's output folder sits, and the server's chosen
/// fallback folder**, out of the Creator's second notes on the test copy,
/// piece 4 (2026-10-04).
///
/// **What it is for.** `Job files and outputs` §6.6: the job's folder now
/// sits where the file's own outputs point, and is chosen at render start
/// where the desktop wrote no reading into the manifest (`Job files and
/// outputs` P21) — so the
/// desktop can no longer compose it from the server's layout, and holds what
/// the machine recorded instead (`Queue & History` Y8, `Queue & History` N9).
/// And `Job files and outputs` P23: the user
/// may name, per server, the folder a job falls back to.
///
/// - `job_record.job_output_folder` is `Queue & History`'s
///   `TheJobOutputFolderTheRecordHolds`, written as JSON: *chosen at render
///   start* at submission, and where it sits once `Queue & History` N9
///   learns it. **Absent on
///   every record before this version**, which reads as the job area every job
///   had then — what such a job's manifest names.
/// - `server_row.chosen_fallback_folder` is `Servers` D3's new row, the path
///   as typed with surrounding white space removed; **absent means Smoozy's
///   own job area**.
///
/// **Versions 1 to 38 are not edited**, and nothing existing is touched.
pub const VERSION_39: &[&str] = &[
    "ALTER TABLE job_record ADD COLUMN job_output_folder TEXT",
    "ALTER TABLE server_row ADD COLUMN chosen_fallback_folder TEXT",
];

/// Version 40 — **`Queue & History` P4's key becomes (job, frame, View
/// Layer)**, out of the Creator's seventh notes, note 4 (2026-10-10).
///
/// **What it is for.** A frame rendered from a multi-layer file now has one
/// copy per View Layer (`Queue & History` P1), and version 37's key — one row
/// per (job, frame) — would let one layer's copy replace another's. So:
///
/// - the key is the job, the frame and `view_layer`, **the layer absent where
///   the machine recorded none** — a frame's only copy. A key column that may
///   be absent does not keep itself unique, so two indexes keep it: one for a
///   named layer, one allowing a single unnamed copy per frame;
/// - `main_copy` says whether the row is the frame's main copy (P10), the one
///   the live fetch and the notification read;
/// - `place` is its place in the frame's set (P4), which orders a frame's
///   layers and names its file on the machine.
///
/// **Every row held before this version is carried over whole**, as the
/// frame's main copy at place nought with its layer as it was — each was then
/// its frame's only copy. The table is one a migration never empties, and the
/// rows are copied before the old table goes.
///
/// **Versions 1 to 39 are not edited.**
pub const VERSION_40: &[&str] = &[
    "ALTER TABLE job_record_preview RENAME TO job_record_preview_before_40",
    "CREATE TABLE job_record_preview (
        job TEXT NOT NULL,
        frame INTEGER NOT NULL,
        operation TEXT NOT NULL,
        view_layer BLOB,
        owed INTEGER NOT NULL CHECK (owed IN (0, 1)),
        copy TEXT,
        published_on_this_computer_at INTEGER,
        main_copy INTEGER NOT NULL DEFAULT 1 CHECK (main_copy IN (0, 1)),
        place INTEGER NOT NULL DEFAULT 0
     )",
    "INSERT INTO job_record_preview
        (job, frame, operation, view_layer, owed, copy, published_on_this_computer_at, main_copy, place)
     SELECT job, frame, operation, view_layer, owed, copy, published_on_this_computer_at, 1, 0
     FROM job_record_preview_before_40",
    "DROP TABLE job_record_preview_before_40",
    "CREATE UNIQUE INDEX job_record_preview_one_per_named_layer
        ON job_record_preview (job, frame, view_layer) WHERE view_layer IS NOT NULL",
    "CREATE UNIQUE INDEX job_record_preview_one_unnamed_copy
        ON job_record_preview (job, frame) WHERE view_layer IS NULL",
];

/// Version 41 — **the driver's two values and the missed OptiX bar**, the
/// fourth piece of the OptiX and driver work (2026-10-11).
///
/// **What it is for.** `Servers` H11b: the hardware reading carries the
/// driver's version and the CUDA version the driver reports, with the machine
/// kind `Rented-server runtime` K1 placed it in, and they are *shown greyed as
/// last seen* when not connected — so they are held with the rest of the
/// reading. And the statement is said on a machine Smoozy manages only where a
/// swap missed the bar (`Server Setup` R1 item 5), which a later connection
/// cannot read back from the machine — so the miss is held on the row.
///
/// - `hardware_driver_version`, `hardware_driver_cuda_version` and
///   `hardware_driver_placed` are three more columns of the reading the row
///   already holds, **absent on every record before this version** and
///   wherever the driver did not answer: a reading with no driver part, which
///   is what `Servers` H11b says such a reading is.
/// - `driver_missed_the_optix_bar` is `Servers` D3's row of 2026-10-11,
///   **nought on every record before this version** — no miss recorded.
///
/// **Optional columns added, as version 39 added its two; nothing existing is
/// touched or carried. Versions 1 to 40 are not edited.**
pub const VERSION_41: &[&str] = &[
    "ALTER TABLE server_row ADD COLUMN hardware_driver_version TEXT",
    "ALTER TABLE server_row ADD COLUMN hardware_driver_cuda_version TEXT",
    "ALTER TABLE server_row ADD COLUMN hardware_driver_placed TEXT",
    "ALTER TABLE server_row ADD COLUMN driver_missed_the_optix_bar INTEGER NOT NULL DEFAULT 0
        CHECK (driver_missed_the_optix_bar IN (0, 1))",
];

/// Version 42 — **`Queue & History` N23's mark**, held-job piece A
/// (2026-10-12).
///
/// **What it is for.** `Server Setup` R1c releases a job held for the driver
/// however its swap's run ended, and where the driver did not reach the bar the
/// job then fails as `Rendering` §9 item 3 says. The mark keeps that failure
/// from being unexplained, and it is written before the release is sent.
///
/// - `driver_mark_blender` is the job's Blender version as the record already
///   holds it, copied when the mark is written. **Its presence is the mark**:
///   absent on every record before this version and on every job never
///   released that way.
/// - `driver_mark_driver_version` is the driver's version as the driver wrote
///   it, **absent where the driver gave none** — it no longer answered.
///
/// **Optional columns added, as versions 39 and 41 added theirs; nothing
/// existing is touched or carried, so no stored record is migrated. Versions 1
/// to 41 are not edited.**
pub const VERSION_42: &[&str] = &[
    "ALTER TABLE job_record ADD COLUMN driver_mark_blender TEXT",
    "ALTER TABLE job_record ADD COLUMN driver_mark_driver_version TEXT",
];

/// Version 43 — **the installs still waiting behind a chain of Blender
/// installs**, kept against the server row — `Server Setup` Y9a and `Servers`
/// D3's row of 2026-10-16, step 10c's owed piece 4.
///
/// **What it is for.** Several Blenders picked at once install one after
/// another, and the desktop begins each when the last one's run ends. A
/// desktop that closes before the last is begun leaves the rest never begun,
/// and until this version nothing said so once Smoozy opened again. So the
/// versions picked and not yet begun are kept here, in the list's order, and
/// the Set up card names them on reopening.
///
/// - `server` is the local server identifier, and the rows go with the
///   server row's removal (`Servers` D17's walk, which reaches the row's own
///   lists).
/// - `position` is the version's place in the order the list showed them.
/// - `version` is the version as the list asked for it, named on the card the
///   way the list named it.
///
/// **A new table, empty on every record before this version** — nothing was
/// waiting that anybody kept. **Versions 1 to 42 are not edited.**
pub const VERSION_43: &[&str] = &["CREATE TABLE server_install_not_begun (
        server TEXT NOT NULL,
        position INTEGER NOT NULL,
        version TEXT NOT NULL,
        PRIMARY KEY (server, position)
     )"];

/// Version 44 — **where a preview's main picture came from**, the Creator's
/// tenth notes, note 2 (2026-10-17).
///
/// **What it is for.** A frame's main copy that is no one View Layer's — the
/// regular output path's file, fed by every layer, or a File Output node fed
/// by several — is named by where it came from and listed first
/// (`Design/Queue & History` line 201; `Job files and outputs` B31a's last
/// bullet; `Queue & History` P3). The machine's event carries that source,
/// and the desktop keeps it beside the copy so the page can name it after the
/// event has passed.
///
/// - `source` is the source in its one written form
///   (`smoozy_job_files::main_output::WhereThePictureCameFrom`): the output
///   path, or a File Output node with the node's name as its bytes. **Absent
///   on every row before this version** and on every copy that names a View
///   Layer or is its frame's only one — such a row reads as it did.
///
/// **One optional column added, as versions 39, 41 and 42 added theirs;
/// nothing existing is touched or carried, so no stored row is rewritten.
/// Versions 1 to 43 are not edited.**
pub const VERSION_44: &[&str] = &["ALTER TABLE job_record_preview ADD COLUMN source TEXT"];

/// Version 45 — exact submission material, kept before sending, and its cleanup debt.
pub const VERSION_45: &[&str] = &["CREATE TABLE submission_material (
    job TEXT PRIMARY KEY, server TEXT NOT NULL, installation TEXT NOT NULL,
    material_folder TEXT NOT NULL, material BLOB NOT NULL,
    remap_path TEXT NOT NULL, remap BLOB NOT NULL, output_folder TEXT,
    debt INTEGER NOT NULL DEFAULT 0,
    material_done INTEGER NOT NULL DEFAULT 0, remap_done INTEGER NOT NULL DEFAULT 0,
    folder_done INTEGER NOT NULL DEFAULT 0
)"];

/// Fifteenth notes, note 2 again: the key's item is independent of the proof
/// and saved marker. Existing saves retain their legacy subject; new saves
/// reserve fresh subjects before touching the store. No secret is recorded.
pub const VERSION_46: &[&str] = &[
    "CREATE TABLE telegram_token_item (
        the_one_row INTEGER PRIMARY KEY CHECK (the_one_row = 1),
        bot_id INTEGER NOT NULL, subject TEXT NOT NULL
    )",
    "INSERT INTO telegram_token_item (the_one_row, bot_id, subject)
     SELECT the_one_row, bot_id, 'telegram-bot' FROM telegram_bot",
];

/// `Queue & History` P5a: optional descriptive metadata on each existing job.
/// Older jobs stay unknown. The job's own lifetime and deletion cover it.
pub const VERSION_47: &[&str] = &[
    "ALTER TABLE job_record ADD COLUMN first_frame_purpose TEXT",
    "ALTER TABLE job_record ADD COLUMN first_frame_pair TEXT",
    "ALTER TABLE job_record ADD COLUMN first_frame_number INTEGER",
    "ALTER TABLE job_record ADD COLUMN first_frame_range TEXT",
];

/// `Server protocol and transfers` S0a / `State, identity and recovery` I8: parent starts, one shared counter, and the
/// dependency-send parent previously held only for the run. Legacy starts
/// remain NULL. No migration-time clock or server clock fills them.
pub const VERSION_48: &[&str] = &[
    "CREATE TABLE transfer_start_order (only_row INTEGER PRIMARY KEY CHECK (only_row = 1), last_order INTEGER NOT NULL)",
    "ALTER TABLE upload_request ADD COLUMN started_at_utc INTEGER",
    "ALTER TABLE upload_request ADD COLUMN start_order INTEGER",
    "ALTER TABLE download_request ADD COLUMN started_at_utc INTEGER",
    "ALTER TABLE download_request ADD COLUMN start_order INTEGER",
    "ALTER TABLE download_request ADD COLUMN counting_name TEXT",
    "ALTER TABLE download_arrangement ADD COLUMN started_at_utc INTEGER",
    "ALTER TABLE download_arrangement ADD COLUMN start_order INTEGER",
    "CREATE TABLE dependency_upload_request (
        request TEXT PRIMARY KEY NOT NULL, server TEXT NOT NULL, blend TEXT NOT NULL,
        started_at_utc INTEGER, start_order INTEGER)",
    "CREATE TABLE dependency_upload_file (
        request TEXT NOT NULL REFERENCES dependency_upload_request(request) ON DELETE CASCADE,
        position INTEGER NOT NULL, landing BLOB NOT NULL, size INTEGER NOT NULL,
        transfer TEXT, ending TEXT, PRIMARY KEY (request, position))",
];

/// Version 49 — **the update route's three rows**: `State, identity and
/// recovery` J6's desktop-update trust record, `State, identity and recovery`
/// I4's two values and `State, identity and recovery` I9's update-in-probe
/// record. Built at step 11's update-reading piece; the rows' meaning is
/// `crate::record::update_trust`'s head.
///
/// - `update_trust` — one row or none. **Its own table, beside the
///   server-package trust record of version 11 and sharing no field with it**
///   (`State, identity and recovery` J10). The statement is the bytes that were
///   signed, so verifying it again verifies what was verified the first time.
/// - `update_check` — one row, two values, each nullable and each written on
///   its own: the moment the last check completed, on this computer's clock as
///   the column's name says, and the declined version.
/// - `update_in_probe` — one row or none: Z21a's release, previous artifact and
///   record copy. **The two paths are `BLOB`**, as every path in this record is.
///
/// **Three new tables, empty on every record before this version**; nothing
/// existing is touched, so no stored row is rewritten and L11, L12 and L12a are
/// untouched. **Versions 1 to 48 are not edited. L10 holds: no secret value is
/// written.**
pub const VERSION_49: &[&str] = &[
    "CREATE TABLE update_trust (
        only_row INTEGER PRIMARY KEY NOT NULL CHECK (only_row = 1),
        greatest_accepted_release_sequence INTEGER NOT NULL,
        greatest_accepted_trust_sequence INTEGER NOT NULL,
        the_statement BLOB NOT NULL,
        its_recovery_signature BLOB NOT NULL
     )",
    "CREATE TABLE update_check (
        only_row INTEGER PRIMARY KEY NOT NULL CHECK (only_row = 1),
        last_completed_on_this_computer_at INTEGER,
        declined_version TEXT
     )",
    "CREATE TABLE update_in_probe (
        only_row INTEGER PRIMARY KEY NOT NULL CHECK (only_row = 1),
        release_sequence INTEGER NOT NULL,
        version TEXT NOT NULL,
        previous_artifact BLOB NOT NULL,
        record_copy BLOB NOT NULL
     )",
];

/// Version 50 — **`Rented-server runtime` V12's free-space reading, carried
/// with a job's ending** (`Desktop app and delivery` M15a), step 12's SC4
/// findings (2026-10-25).
///
/// **What it is for.** The reading is taken on the machine against the job's
/// own output folder, in the act that first tried to write the render's
/// recorded outcome, and reaches this side on the note with the ending. The
/// failure on History names the full disk from it (M14) — after the machine is
/// gone too, so it is kept with the rest of the note's carried half.
///
/// - `note_free_space_available` is `Server protocol and transfers` F3's
///   available figure in bytes. **Its presence is the reading**: absent on
///   every record before this version and wherever the ending carried none,
///   which is never a reading of nought.
/// - `note_free_space_folder` is the exact folder it was taken against
///   (`Server protocol and transfers` F1), as a path's bytes.
/// - `note_free_space_taken_at` and `note_free_space_machine` are its moment
///   and the machine whose clock it was taken on, as the note's other two
///   moments are kept (`State, identity and recovery` D5).
///
/// **Optional columns added, as versions 39, 41 and 42 added theirs; nothing
/// existing is touched or carried, so no stored record is migrated. Versions 1
/// to 49 are not edited.**
pub const VERSION_50: &[&str] = &[
    "ALTER TABLE job_record ADD COLUMN note_free_space_available INTEGER",
    "ALTER TABLE job_record ADD COLUMN note_free_space_folder BLOB",
    "ALTER TABLE job_record ADD COLUMN note_free_space_taken_at INTEGER",
    "ALTER TABLE job_record ADD COLUMN note_free_space_machine TEXT",
];

/// Version 51 — **the frame text the user typed, kept with the job's record**
/// (`Queue & History` P5b, `Render Settings` F1, `State, identity and recovery`
/// I10), the eighteenth notes, piece 7 (2026-10-25).
///
/// - `typed_frames` is the range or list the user typed and Render Settings
///   accepted, exactly as accepted (only spaces at its ends trimmed), written
///   at submission in the record's own transaction. **It is for the screen
///   alone**: never parsed again, counted, or sent to a machine — the kept
///   setup's parsed form stays the only source for every number. **Absent on
///   every record before this version**, on a job whose range was the file's
///   own, on a first-frame check, and on a job taken over from a server; each
///   of those draws the numbers form.
///
/// **One optional column added, as version 50 added its four; nothing existing
/// is touched or carried, so no stored record is migrated. Versions 1 to 50 are
/// not edited.** It shares the record's lifetime: deleting the record deletes it
/// (`Queue & History` X1).
pub const VERSION_51: &[&str] = &["ALTER TABLE job_record ADD COLUMN typed_frames TEXT"];

/// Version 52: the existing update-in-probe record distinguishes a Linux
/// rollback still owed from an ordinary launch completing a successful probe.
/// Phase 1 owes the reinstall; phase 2 confirms it and owes the remaining
/// record and cleanup acts, so interrupted cleanup needs no deleted cache.
/// `State, identity and recovery` I9; no new record kind. Older rows default
/// to the successful-probe route. No earlier migration is changed.
pub const VERSION_52: &[&str] = &[
    "ALTER TABLE update_in_probe ADD COLUMN linux_rollback_required INTEGER NOT NULL DEFAULT 0 CHECK (linux_rollback_required IN (0, 1, 2))",
];

/// Version 54 — **the acts still waiting on a server, not only the installs**
/// — `Server Setup` Y9a item 10, the nineteenth notes, note 2 (2026-10-27).
///
/// **What it is for.** Anything pressed on the Set up card while a setup runs
/// on that server now waits its turn — an install from the list, the user's
/// own build, a removal, *Set up again*, the Telegram placement — and the ones
/// a closed Smoozy never began are named on reopening, as the installs alone
/// were from version 43. So each row says which act it is.
///
/// - `act` is the act's kind in its one written form
///   (`crate::record::installs_not_begun::TheActsKind`). **Every row before
///   this version reads `install`**, a Blender install from the list, which is
///   the only act version 43 kept.
/// - `version` keeps its name and now holds what the act names: the version as
///   the list asked for it, the build's file name, the removed build's answered
///   version or its path, and nothing for the ready action and the Telegram
///   placement.
///
/// **One column added with a default; nothing existing is rewritten and the
/// table keeps its name, so the server row's removal walk reaches it as
/// before. Versions 1 to 53 are not edited.**
pub const VERSION_54: &[&str] =
    &["ALTER TABLE server_install_not_begun ADD COLUMN act TEXT NOT NULL DEFAULT 'install'"];

/// Version 55 — **a job's reporting choice, as the server holds it** —
/// `Telegram` V1c, the twenty-first notes, note 7 (2026-10-30).
///
/// **What it is for.** A live job's Telegram choice can be changed from its
/// queue entry, and the entry shows what the server holds. The column keeps the
/// `Telegram` V1a value in the manifest's own form, as JSON text: written at
/// submission from the manifest's field, and replaced by the value the
/// server's acknowledgement of a change carries. **A record from before this
/// version holds nothing**, and the entry's control then reads no choice.
///
/// **One optional column added, as version 51 added its one; nothing existing
/// is touched or carried, so no stored record is migrated. Versions 1 to 54 are
/// not edited.** It shares the record's lifetime: deleting the record deletes it.
pub const VERSION_55: &[&str] = &["ALTER TABLE job_record ADD COLUMN reporting_choice TEXT"];

/// Version 56 — **the app-wide reporting item holds any of the five, and the
/// phone's choice it last took** — `Telegram` V1a as amended and `Telegram`
/// V1d, the Telegram discussion of 2026-10-31.
///
/// **What it is for.** A choice made from the phone with `/reporting` becomes
/// the default for new jobs, and it can be *a picture every few minutes* with
/// its minutes, so the default carries minutes beside its identifier. **The
/// desktop takes a phone's choice once** (`Telegram` V1d: *"one whose identity
/// it already holds changes nothing"*), and only where its Telegram `date` is
/// later than the one it took last, so the choice's operation identity and its
/// moment are kept beside the default.
///
/// - `reporting_default_minutes` — the default's minutes, whole from 1 to 60;
///   nothing where none were typed or taken, which reads as 5.
/// - `phones_choice_operation` and `phones_choice_moment` — the operation
///   identity, as text, and Telegram's own `date` in whole seconds since 1970,
///   of the last phone's choice taken. Both or neither.
///
/// **Three optional columns added to the one-row table version 20 made;
/// nothing existing is touched or carried. Versions 1 to 55 are not edited.**
pub const VERSION_56: &[&str] = &[
    "ALTER TABLE telegram_setting ADD COLUMN reporting_default_minutes INTEGER CHECK (reporting_default_minutes BETWEEN 1 AND 60)",
    "ALTER TABLE telegram_setting ADD COLUMN phones_choice_operation TEXT",
    "ALTER TABLE telegram_setting ADD COLUMN phones_choice_moment INTEGER",
];

/// Version 57 — **that an auto-download's ending walk completed, written
/// down** — `Download` R5's dated note of 2026-10-31, the twenty-sixth notes,
/// note 4.
///
/// **What it is for.** Until this version the completion lived only in the
/// run, so every ended job's folder was walked to its end once more on the
/// first connection after Smoozy opened again, and Transfers drew every ended
/// auto-download as *downloading* until then. The column holds **the start
/// order of the arrangement the walk completed for**, `0` for an arrangement
/// whose start was never recorded; it counts only while it equals the
/// arrangement's own start, so a switch-on's new start asks for a new walk.
///
/// **One optional column added; nothing existing is touched or carried, so
/// every arrangement before this version reads as not yet walked, which is how
/// it read before. Versions 1 to 56 are not edited.** It shares the
/// arrangement's lifetime: removing the arrangement removes it.
pub const VERSION_57: &[&str] =
    &["ALTER TABLE download_arrangement ADD COLUMN ending_walk_completed_for INTEGER"];
