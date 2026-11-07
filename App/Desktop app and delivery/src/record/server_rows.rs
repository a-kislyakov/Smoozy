//! `Servers`' rows in the local record — `Desktop app and delivery` §5 and
//! `Servers` §6.
//!
//! **The row's shape is `Servers`' and the store is this document's**, which
//! is the split `Security and licence` §1.4 B30 draws and the one `Keys`' rows
//! already use: that module owns the rule, this one owns the mechanism.
//! **Nothing here decides what a server row holds.**
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
//! it belongs to.** `Servers` D1, D2, D3, D4, D5, D6, D9, D13, D16, D17,
//! D18, D21, D22, P1a. `Security and licence` B30, E3a. `Job files and outputs`
//! L4a.
//!
//! **A marker this crate's own plan also defines is deliberately not listed
//! above, whichever document is meant, and is written out with its document
//! at every site instead** — the header list cannot tell the two apart, and a
//! bare one would resolve to a real and wrong clause.
//!
//! **D1 holds: everything durable Smoozy holds about a server is here**, and it
//! is either a field of the server row or a record keyed by the row's local
//! server identifier. There is no second store, no per-server file, and
//! **nothing written into the user's own SSH configuration**.
//!
//! **D2: the one thing not here is a secret's value.** What the row holds is
//! the **subject reference**, which carries no value and, per `Security and
//! licence` §3.0 rule 1, carries nothing about the user.
//!
//! **D6 is the shape of this file.** The table in D3 and the list in D4 are one
//! enumeration and **the only route by which anything about a server is
//! reached**, so the removal below walks that enumeration rather than a list
//! kept in step by hand — which is Row 41's real requirement.

use rusqlite::OptionalExtension;
use smoozy_security_and_licence::erasure::{EraseMarks, RecordFailure as EraseFailure};
use smoozy_security_and_licence::held::{HeldSecret, StoredKind, Subject};
use smoozy_servers::hardware::{
    Card, CardMake, Cards, DriverVersion, FreeSpace, HardwareReading, MachineMemory,
    TheDriversOwnAnswer, WhereK1PlacesIt, WhyUnreadable,
};
use smoozy_servers::identity::{Host, KeyReference, Port, Username};
use smoozy_servers::store::{
    KeyedByTheServer, RecordedMachineRoot, RememberedHostKey, RememberedSetup, ServerRow,
};
use smoozy_state::identities::{Identity, IdentityKind};

use super::{moment, nanoseconds, RecordFailure};

/// The columns the row's own table carries, named once so the read and the two
/// writes cannot drift apart.
const EVERY_COLUMN: &str = "identifier, host, port, username, key_identifier, key_fingerprint,
     label, colour, remembered_setup, remembered_host_key,
     hardware_cards, hardware_cards_unreadable, hardware_memory, hardware_memory_bytes,
     hardware_free_space_bytes, hardware_free_space_path,
     hardware_machine_moment, hardware_desktop_moment, hardware_read_under,
     machine_root_path, machine_package_area_path, machine_root_read_under,
     last_confirmed_installation_identity, event_cursor, machine_password_subject,
     chosen_fallback_folder,
     hardware_driver_version, hardware_driver_cuda_version, hardware_driver_placed,
     driver_missed_the_optix_bar, hardware_no_optix_library";

/// Every server row, in the order they were added.
///
/// **D22: nothing durable records that a server was connected**, so nothing
/// read back here says anything about a connection — which is what makes *every
/// server starts disconnected* true by construction rather than by a clearing
/// step that could fail.
pub fn all(connection: &rusqlite::Connection) -> Result<Vec<ServerRow>, RecordFailure> {
    let mut statement =
        connection.prepare(&format!("SELECT {EVERY_COLUMN} FROM server_row ORDER BY rowid"))?;
    let identifiers = statement.query_map([], |row| row.get::<_, String>(0))?;
    let mut wanted = Vec::new();
    for identifier in identifiers {
        wanted.push(identifier?);
    }

    let mut rows = Vec::new();
    for identifier in wanted {
        if let Some(row) = one(connection, &identifier)? {
            rows.push(row);
        }
    }
    Ok(rows)
}

/// One row by its identifier, with the two lists under it.
pub fn one(
    connection: &rusqlite::Connection,
    identifier: &str,
) -> Result<Option<ServerRow>, RecordFailure> {
    let flat = connection
        .query_row(
            &format!("SELECT {EVERY_COLUMN} FROM server_row WHERE identifier = ?1"),
            [identifier],
            |row| {
                Ok(Flat {
                    identifier: row.get(0)?,
                    host: row.get(1)?,
                    port: row.get(2)?,
                    username: row.get(3)?,
                    key_identifier: row.get(4)?,
                    key_fingerprint: row.get(5)?,
                    label: row.get(6)?,
                    colour: row.get(7)?,
                    remembered_setup: row.get::<_, i64>(8)? != 0,
                    remembered_host_key: row.get(9)?,
                    hardware_cards: row.get(10)?,
                    hardware_cards_unreadable: row.get(11)?,
                    hardware_memory: row.get(12)?,
                    hardware_memory_bytes: row.get(13)?,
                    hardware_free_space_bytes: row.get(14)?,
                    hardware_free_space_path: row.get(15)?,
                    hardware_machine_moment: row.get(16)?,
                    hardware_desktop_moment: row.get(17)?,
                    hardware_read_under: row.get(18)?,
                    machine_root_path: row.get(19)?,
                    machine_package_area_path: row.get(20)?,
                    machine_root_read_under: row.get(21)?,
                    last_confirmed_installation_identity: row.get(22)?,
                    event_cursor: row.get(23)?,
                    machine_password_subject: row.get(24)?,
                    chosen_fallback_folder: row.get(25)?,
                    hardware_driver_version: row.get(26)?,
                    hardware_driver_cuda_version: row.get(27)?,
                    hardware_driver_placed: row.get(28)?,
                    driver_missed_the_optix_bar: row.get::<_, i64>(29)? != 0,
                    hardware_no_optix_library: row.get::<_, i64>(30)? != 0,
                })
            },
        )
        .optional()?;
    let Some(flat) = flat else {
        return Ok(None);
    };

    // **An identifier that will not read back is not composed from anything
    // else instead** (`State, identity and recovery` §2.1), and `Servers` C6
    // forbids deriving one from a host, a port or a username. The row is skipped rather
    // than given an invented identity, which is the same refusal the issuer
    // makes and the same one `Keys`' rows make.
    if Identity::read_back(IdentityKind::LocalServer, &flat.identifier).is_none() {
        return Ok(None);
    }
    // **`Servers` C5: all four connection-identity fields must be filled in.**
    // A row whose host, port or username will not read back as one is a row
    // that can never reach anything, and it is skipped for the same reason.
    let (Some(host), Some(port), Some(username)) = (
        Host::what_the_user_typed(&flat.host),
        u16::try_from(flat.port).ok().and_then(Port::what_the_user_typed),
        Username::what_the_user_typed(&flat.username),
    ) else {
        return Ok(None);
    };

    let remembered_hardware_reading = the_reading(connection, &flat)?;
    let remembered_setup = if flat.remembered_setup {
        Some(RememberedSetup {
            options: setup_options(connection, &flat.identifier)?,
        })
    } else {
        None
    };

    Ok(Some(ServerRow {
        local_server_identifier: flat.identifier.clone(),
        host,
        port,
        username,
        // **P1a: the reference is the pair**, and both halves come back
        // together — an identifier-only row would answer yes for a server set
        // up with a key that has since left.
        key: KeyReference {
            local_key_identifier: flat.key_identifier,
            fingerprint: flat.key_fingerprint,
        },
        label: flat.label,
        colour: flat.colour,
        remembered_setup,
        remembered_host_key: flat.remembered_host_key.map(RememberedHostKey::approved),
        remembered_hardware_reading,
        // **`Job files and outputs` L1's root, with the installation identity
        // it was recorded under.** Both halves come back together or neither
        // does: a path with no identity beside it is a path that says nothing
        // about which installation chose it, and §2.3 forbids holding one.
        recorded_machine_root: match (flat.machine_root_path, flat.machine_root_read_under) {
            (Some(path), Some(read_under)) => Some(RecordedMachineRoot {
                path,
                // **Absent where L4a's second choice did not fire**, which is
                // `Server Setup` R7b's *nothing second is chosen, recorded or
                // handed over* rather than a value that went missing.
                the_package_areas_own_location: flat.machine_package_area_path,
                read_under,
            }),
            _ => None,
        },
        last_confirmed_installation_identity: flat.last_confirmed_installation_identity,
        event_cursor: flat.event_cursor.map(|cursor: i64| cursor as u64),
        machine_password_subject: flat
            .machine_password_subject
            .map(Subject::issued_by_the_local_record),
        // **`Servers` D3's row, record version 39**: absent means Smoozy's own
        // job area (`Job files and outputs` P23).
        chosen_fallback_folder: flat.chosen_fallback_folder,
        // **`Servers` D3's row of 2026-10-11, record version 41**: nought on
        // every row before it, which is *no miss recorded*.
        the_driver_missed_the_optix_bar: flat.driver_missed_the_optix_bar,
    }))
}

/// The row as the table holds it, before the lists under it are read.
struct Flat {
    identifier: String,
    host: String,
    port: i64,
    username: String,
    key_identifier: String,
    key_fingerprint: String,
    label: Option<String>,
    colour: Option<String>,
    remembered_setup: bool,
    remembered_host_key: Option<Vec<u8>>,
    hardware_cards: Option<String>,
    hardware_cards_unreadable: Option<String>,
    hardware_memory: Option<String>,
    hardware_memory_bytes: Option<i64>,
    hardware_free_space_bytes: Option<i64>,
    hardware_free_space_path: Option<String>,
    hardware_machine_moment: Option<i64>,
    hardware_desktop_moment: Option<i64>,
    hardware_read_under: Option<String>,
    machine_root_path: Option<String>,
    machine_package_area_path: Option<String>,
    machine_root_read_under: Option<String>,
    last_confirmed_installation_identity: Option<String>,
    event_cursor: Option<i64>,
    machine_password_subject: Option<String>,
    chosen_fallback_folder: Option<String>,
    hardware_driver_version: Option<String>,
    hardware_driver_cuda_version: Option<String>,
    hardware_driver_placed: Option<String>,
    driver_missed_the_optix_bar: bool,
    hardware_no_optix_library: bool,
}

/// The row creation — the single durable write of an add.
pub fn add(connection: &rusqlite::Connection, row: &ServerRow) -> Result<(), RecordFailure> {
    connection.execute(
        &format!(
            "INSERT INTO server_row ({EVERY_COLUMN}) VALUES
             (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18,
              ?19, ?20, ?21, ?22, ?23, ?24, ?25, ?26, ?27, ?28, ?29, ?30, ?31)"
        ),
        rusqlite::params_from_iter(the_values(row).iter()),
    )?;
    write_the_lists(connection, row)
}

/// Writes a row that already exists.
///
/// **D16: the whole of an edit's write is one durable write**, made before the
/// put-back is attempted — so this is one call and the caller does not compose
/// an edit out of several. **The identifier is never written**, which is what
/// makes a row's identity something no act here can change.
pub fn write(connection: &rusqlite::Connection, row: &ServerRow) -> Result<(), RecordFailure> {
    connection.execute("BEGIN IMMEDIATE", [])?;
    let attempt = (|| -> Result<(), RecordFailure> {
        connection.execute(
            "UPDATE server_row SET
                host = ?2, port = ?3, username = ?4, key_identifier = ?5, key_fingerprint = ?6,
                label = ?7, colour = ?8, remembered_setup = ?9, remembered_host_key = ?10,
                hardware_cards = ?11, hardware_cards_unreadable = ?12,
                hardware_memory = ?13, hardware_memory_bytes = ?14,
                hardware_free_space_bytes = ?15, hardware_free_space_path = ?16,
                hardware_machine_moment = ?17, hardware_desktop_moment = ?18,
                hardware_read_under = ?19,
                machine_root_path = ?20, machine_package_area_path = ?21,
                machine_root_read_under = ?22,
                last_confirmed_installation_identity = ?23, event_cursor = ?24,
                machine_password_subject = ?25,
                chosen_fallback_folder = ?26,
                hardware_driver_version = ?27, hardware_driver_cuda_version = ?28,
                hardware_driver_placed = ?29,
                driver_missed_the_optix_bar = ?30,
                hardware_no_optix_library = ?31
             WHERE identifier = ?1",
            rusqlite::params_from_iter(the_values(row).iter()),
        )?;
        // The two lists are rewritten whole rather than reconciled row by row:
        // `Servers` H4's *a reading replaces it whole* is the rule for the
        // cards, and the ticked options are one instruction rather than a set
        // to be merged.
        forget_the_lists(connection, &row.local_server_identifier)?;
        write_the_lists(connection, row)
    })();
    match attempt {
        Ok(()) => {
            connection.execute("COMMIT", [])?;
            Ok(())
        }
        Err(failure) => {
            let _ = connection.execute("ROLLBACK", []);
            Err(failure)
        }
    }
}

/// Every field of D3, in the one order the read and both writes share.
fn the_values(row: &ServerRow) -> Vec<Box<dyn rusqlite::ToSql>> {
    let reading = row.remembered_hardware_reading.as_ref();
    let (cards, unreadable) = match reading.map(|reading| &reading.cards) {
        Some(Cards::TheCards(_)) => (Some(THE_CARDS), None),
        Some(Cards::PresentAndUnreadable(why)) => (
            Some(PRESENT_AND_UNREADABLE),
            Some(match why {
                WhyUnreadable::TheDriverIsAbsent => "the driver is absent",
                WhyUnreadable::TheDriverAnsweredWithNone => "the driver answered with none",
            }),
        ),
        Some(Cards::NoCard) => (Some(NO_CARD), None),
        Some(Cards::CouldNotBeEstablished) => (Some(COULD_NOT_BE_ESTABLISHED), None),
        None => (None, None),
    };
    let (memory, memory_bytes) = match reading.map(|reading| reading.machine_memory) {
        Some(MachineMemory::TheAllocation(bytes)) => (Some(THE_ALLOCATION), Some(bytes as i64)),
        Some(MachineMemory::TheWholeMachine(bytes)) => (Some(THE_WHOLE_MACHINE), Some(bytes as i64)),
        Some(MachineMemory::CouldNotBeEstablished) => (Some(COULD_NOT_BE_ESTABLISHED), None),
        None => (None, None),
    };
    let free_space = reading.and_then(|reading| reading.free_space.as_ref());
    // **`Servers` H11b's part of the reading**, written with it and replaced
    // whole with it (`Servers` H4): a reading with no driver part writes none.
    let the_driver = reading.and_then(|reading| reading.the_driver.as_ref());

    vec![
        Box::new(row.local_server_identifier.clone()),
        Box::new(row.host.as_str().to_string()),
        Box::new(i64::from(row.port.number())),
        Box::new(row.username.as_str().to_string()),
        Box::new(row.key.local_key_identifier.clone()),
        Box::new(row.key.fingerprint.clone()),
        Box::new(row.label.clone()),
        Box::new(row.colour.clone()),
        Box::new(i64::from(row.remembered_setup.is_some())),
        // **The one place the remembered host key's value is written**, which
        // is `Security and licence` §7's own destination for it. D9 bars it from
        // every screen, every log and every other module, and the type's own
        // accessor is named for this call site.
        Box::new(
            row.remembered_host_key
                .as_ref()
                .map(|key| key.as_the_local_record_writes_it().to_vec()),
        ),
        Box::new(cards.map(str::to_string)),
        Box::new(unreadable.map(str::to_string)),
        Box::new(memory.map(str::to_string)),
        Box::new(memory_bytes),
        Box::new(free_space.map(|space| space.available_bytes as i64)),
        Box::new(free_space.map(|space| space.the_path_it_was_taken_against.clone())),
        // **`Servers` H2a's two marks.** [`nanoseconds`] answers a failure
        // rather than zero since 2026-09-07 (`Access` round 1, F-9), and
        // **this module's behaviour is deliberately unchanged by that** —
        // ruling 6 of that round: the repair belongs to the licence record and
        // every other caller was read and left where it stood. Both moments
        // here are taken from the desktop's and the machine's own clocks at the
        // reading, so neither is a value the wire can put past 2262; the zero
        // is what this row module already wrote and it is now written on
        // purpose rather than by the helper's default.
        Box::new(reading.map(|reading| {
            nanoseconds(reading.the_machines_own_moment.taken_at()).unwrap_or(0)
        })),
        Box::new(reading.map(|reading| {
            nanoseconds(reading.the_desktops_own_moment.taken_at()).unwrap_or(0)
        })),
        Box::new(reading.and_then(|reading| reading.read_under.clone())),
        Box::new(
            row.recorded_machine_root
                .as_ref()
                .map(|root| root.path.clone()),
        ),
        Box::new(
            row.recorded_machine_root
                .as_ref()
                .and_then(|root| root.the_package_areas_own_location.clone()),
        ),
        Box::new(
            row.recorded_machine_root
                .as_ref()
                .map(|root| root.read_under.clone()),
        ),
        Box::new(row.last_confirmed_installation_identity.clone()),
        Box::new(row.event_cursor.map(|cursor| cursor as i64)),
        Box::new(
            row.machine_password_subject
                .as_ref()
                .map(|subject| subject.as_str().to_string()),
        ),
        Box::new(row.chosen_fallback_folder.clone()),
        Box::new(the_driver.map(|driver| driver.version.as_the_driver_wrote_it().to_string())),
        Box::new(the_driver.and_then(|driver| driver.cuda_version.clone())),
        Box::new(the_driver.map(|driver| placed_as_text(driver.where_k1_placed_the_machine))),
        Box::new(i64::from(row.the_driver_missed_the_optix_bar)),
        // **`Servers` H11b's OptiX-library bullet, record version 58**: part of
        // the reading, so a row with no reading writes nought.
        Box::new(i64::from(reading.is_some_and(|reading| reading.the_optix_library_is_missing))),
    ]
}

/// **`Servers` D3's row of 2026-10-11, written alone** — the one value a setup
/// run's ending leaves on the row (`Server Setup` R1 item 5's miss, or that the
/// driver met the bar).
///
/// **One column and nothing else**, so the ending of a run never writes back a
/// row it read before the run began: the hardware reading a connection took
/// meanwhile, or an edit, stands as it was.
pub fn the_driver_miss_is_written(
    connection: &rusqlite::Connection,
    server: &str,
    missed: bool,
) -> Result<(), RecordFailure> {
    connection.execute(
        "UPDATE server_row SET driver_missed_the_optix_bar = ?2 WHERE identifier = ?1",
        rusqlite::params![server, i64::from(missed)],
    )?;
    Ok(())
}

const A_CONTAINER: &str = "a container";
const ITS_OWN_KERNEL: &str = "its own kernel";
const NOWHERE_IT_PLACES: &str = "nowhere it places";

/// **`Rented-server runtime` K1's three rows, as the record writes them.**
fn placed_as_text(placed: WhereK1PlacesIt) -> &'static str {
    match placed {
        WhereK1PlacesIt::AContainer => A_CONTAINER,
        WhereK1PlacesIt::ItsOwnKernel => ITS_OWN_KERNEL,
        WhereK1PlacesIt::NowhereItPlaces => NOWHERE_IT_PLACES,
    }
}

/// **`Servers` H11b's part of a held reading, read back.**
///
/// **All or nothing**: a version that does not read back as a version, or a
/// kind that is not one of `Rented-server runtime` K1's three, is no driver
/// part — never a guess at one, and never a reason the rest of the reading
/// fails, as `Servers` H11b has it of the live reading. **The kind is what the
/// statement turns on**, so a driver part that cannot say where the machine was
/// placed is not handed over with a default.
fn the_driver_of(flat: &Flat) -> Option<TheDriversOwnAnswer> {
    let version = DriverVersion::parse(flat.hardware_driver_version.as_deref()?)?;
    let where_k1_placed_the_machine = match flat.hardware_driver_placed.as_deref()? {
        A_CONTAINER => WhereK1PlacesIt::AContainer,
        ITS_OWN_KERNEL => WhereK1PlacesIt::ItsOwnKernel,
        NOWHERE_IT_PLACES => WhereK1PlacesIt::NowhereItPlaces,
        _ => return None,
    };
    Some(TheDriversOwnAnswer {
        version,
        cuda_version: flat.hardware_driver_cuda_version.clone(),
        where_k1_placed_the_machine,
    })
}

const THE_CARDS: &str = "the cards";
const PRESENT_AND_UNREADABLE: &str = "present and unreadable";
const NO_CARD: &str = "no card";
const COULD_NOT_BE_ESTABLISHED: &str = "could not be established";
const THE_ALLOCATION: &str = "the allocation";
const THE_WHOLE_MACHINE: &str = "the whole machine";

/// The reading, rebuilt from the row's columns and the cards under it.
///
/// **The two moments are what say a reading is held at all**, because
/// `Servers` H2 and `Servers` H2a make both of them part of every reading: a
/// reading
/// with no moment is not a reading with a gap in it, it is no reading.
fn the_reading(
    connection: &rusqlite::Connection,
    flat: &Flat,
) -> Result<Option<HardwareReading>, RecordFailure> {
    let (Some(machine_moment), Some(desktop_moment), Some(cards)) = (
        flat.hardware_machine_moment,
        flat.hardware_desktop_moment,
        flat.hardware_cards.as_deref(),
    ) else {
        return Ok(None);
    };
    let cards = match cards {
        THE_CARDS => Cards::TheCards(cards_of(connection, &flat.identifier)?),
        PRESENT_AND_UNREADABLE => Cards::PresentAndUnreadable(
            match flat.hardware_cards_unreadable.as_deref() {
                Some("the driver is absent") => WhyUnreadable::TheDriverIsAbsent,
                Some("the driver answered with none") => WhyUnreadable::TheDriverAnsweredWithNone,
                // **A row that says the cards are unreadable and cannot say
                // which of the two it was is not that row.** `Servers` H11
                // names the two separately and a reader must not be handed one
                // of them by default, so the whole reading is absent rather
                // than half true.
                _ => return Ok(None),
            },
        ),
        NO_CARD => Cards::NoCard,
        COULD_NOT_BE_ESTABLISHED => Cards::CouldNotBeEstablished,
        _ => return Ok(None),
    };
    let machine_memory = match (flat.hardware_memory.as_deref(), flat.hardware_memory_bytes) {
        (Some(THE_ALLOCATION), Some(bytes)) => MachineMemory::TheAllocation(bytes as u64),
        (Some(THE_WHOLE_MACHINE), Some(bytes)) => MachineMemory::TheWholeMachine(bytes as u64),
        (Some(COULD_NOT_BE_ESTABLISHED), _) => MachineMemory::CouldNotBeEstablished,
        // **`Servers` H9's own safe direction**: a figure whose kind will not
        // read back is never handed over as the rental's allocation, because
        // the `/proc` figure on a slice is the host's and reading it as the
        // rental's is the eightfold overstatement `Servers` H9 exists to
        // prevent.
        _ => MachineMemory::CouldNotBeEstablished,
    };
    let free_space = match (
        flat.hardware_free_space_bytes,
        flat.hardware_free_space_path.clone(),
    ) {
        (Some(bytes), Some(path)) => Some(FreeSpace {
            available_bytes: bytes as u64,
            the_path_it_was_taken_against: path,
        }),
        // **`Servers` H10: a figure with no path is not a free-space
        // reading**, because the reading is a choice of mount and the path is
        // what says which.
        _ => None,
    };
    Ok(Some(HardwareReading::taken(
        cards,
        machine_memory,
        free_space,
        moment(machine_moment),
        &flat.identifier,
        moment(desktop_moment),
        flat.hardware_read_under.clone(),
    )
    .with_the_driver(the_driver_of(flat))
    .with_the_optix_library_missing(flat.hardware_no_optix_library)))
}

fn setup_options(
    connection: &rusqlite::Connection,
    server: &str,
) -> Result<Vec<String>, RecordFailure> {
    let mut statement = connection.prepare(
        "SELECT option FROM server_remembered_setup_option WHERE server = ?1 ORDER BY position",
    )?;
    let rows = statement.query_map([server], |row| row.get::<_, String>(0))?;
    let mut options = Vec::new();
    for option in rows {
        options.push(option?);
    }
    Ok(options)
}

fn cards_of(connection: &rusqlite::Connection, server: &str) -> Result<Vec<Card>, RecordFailure> {
    let mut statement = connection.prepare(
        "SELECT unique_identifier, bus_location, make, total_memory_bytes
         FROM server_hardware_card WHERE server = ?1 ORDER BY position",
    )?;
    let rows = statement.query_map([server], |row| {
        Ok(Card {
            unique_identifier: row.get(0)?,
            bus_location: row.get(1)?,
            make: CardMake::from_the_drivers_own_query(&row.get::<_, String>(2)?),
            // **`Servers` H8's stated absence**: a card with no per-card
            // memory figure is a complete card and never a failed reading.
            total_memory_bytes: row.get::<_, Option<i64>>(3)?.map(|bytes| bytes as u64),
        })
    })?;
    let mut cards = Vec::new();
    for card in rows {
        cards.push(card?);
    }
    Ok(cards)
}

fn write_the_lists(
    connection: &rusqlite::Connection,
    row: &ServerRow,
) -> Result<(), RecordFailure> {
    if let Some(setup) = &row.remembered_setup {
        for (position, option) in setup.options.iter().enumerate() {
            connection.execute(
                "INSERT INTO server_remembered_setup_option (server, position, option)
                 VALUES (?1, ?2, ?3)",
                rusqlite::params![row.local_server_identifier, position as i64, option],
            )?;
        }
    }
    if let Some(Cards::TheCards(cards)) = row
        .remembered_hardware_reading
        .as_ref()
        .map(|reading| &reading.cards)
    {
        for (position, card) in cards.iter().enumerate() {
            connection.execute(
                "INSERT INTO server_hardware_card
                    (server, position, unique_identifier, bus_location, make, total_memory_bytes)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
                rusqlite::params![
                    row.local_server_identifier,
                    position as i64,
                    card.unique_identifier,
                    card.bus_location,
                    // **The driver's own name for the card, which is the half
                    // that cannot be derived** — `Servers` H7. The make's key
                    // is established by the route the reading took, and the
                    // one route a reading takes is NVIDIA's own tool, so
                    // [`CardMake::from_the_drivers_own_query`] gives it back on
                    // the read. Writing the key here instead, as this did until
                    // step 10c's sixth pass split the two, kept *NVIDIA* and
                    // lost which card it was. **A second maker's route, when
                    // one is written, needs its key kept beside the name.**
                    card.make.the_drivers_name_for_the_card(),
                    card.total_memory_bytes.map(|bytes| bytes as i64),
                ],
            )?;
        }
    }
    Ok(())
}

fn forget_the_lists(
    connection: &rusqlite::Connection,
    server: &str,
) -> Result<(), RecordFailure> {
    connection.execute(
        "DELETE FROM server_remembered_setup_option WHERE server = ?1",
        [server],
    )?;
    connection.execute("DELETE FROM server_hardware_card WHERE server = ?1", [server])?;
    Ok(())
}

/// **D17: a removal walks the enumeration of D6 and removes everything in it.**
///
/// **The walk is the enumeration and not a list kept in step by hand**, which is
/// what D6 asks for: `Servers`' own `enumeration` names D3's row and five
/// keyed-record kinds — D4's four, and the told-record `Telegram` L1 puts
/// against the row — and this iterates that value rather than restating it.
/// A kind added there is a kind this walk reaches without anybody editing it —
/// and a kind whose module has not been built yet has no table, which is L1 and
/// is answered here rather than guessed at.
///
/// **D21: history is untouched.** A confirmed job record is keyed by the job
/// identity and is outside the enumeration by D5's own rule, so nothing in this
/// walk can reach it — which is what leaves the entry for the animation the user
/// just rendered standing after the row that submitted it is gone.
///
/// **One kind is marked rather than removed, and it is the job records history
/// never held** — `Queue & History` C9's second way in, item C1: they become
/// history entries marked *start never heard*. See [`the_delete_for`].
///
/// **What this does not do is erase the machine password's value.** That is
/// D18's, it goes through the secret adapter in `Security and licence` §5.1's
/// order, and it is the caller's step before this one — see [`ServerRowMarks`].
pub fn remove_walking_the_enumeration(
    connection: &rusqlite::Connection,
    of: &str,
) -> Result<Vec<KeyedByTheServer>, RecordFailure> {
    connection.execute("BEGIN IMMEDIATE", [])?;
    let attempt = (|| -> Result<Vec<KeyedByTheServer>, RecordFailure> {
        let mut not_yet_built = Vec::new();
        for keyed in KeyedByTheServer::ALL {
            match table_for(keyed) {
                Some(table) if table_exists(connection, table)? => {
                    what_goes_first(connection, keyed, of)?;
                    connection.execute(&the_delete_for(keyed, table), [of])?;
                }
                // **L1: a module's rows arrive with that module.** The kind is
                // reported rather than skipped silently, so a caller that lands
                // one of these modules and forgets its table finds out here.
                _ => not_yet_built.push(keyed),
            }
        }
        forget_the_lists(connection, of)?;
        // **The installs still waiting on the server are a field of its row**
        // (`Servers` D3's row of 2026-10-16, `Server Setup` Y9a), so they go
        // with it. **Not in [`forget_the_lists`]**, which every write of the
        // row runs: the row is written for many reasons, and none of them
        // empties what the user picked.
        super::installs_not_begun::forget(connection, of)?;
        connection.execute("DELETE FROM server_row WHERE identifier = ?1", [of])?;
        Ok(not_yet_built)
    })();
    match attempt {
        Ok(not_yet_built) => {
            connection.execute("COMMIT", [])?;
            Ok(not_yet_built)
        }
        Err(failure) => {
            let _ = connection.execute("ROLLBACK", []);
            Err(failure)
        }
    }
}

/// **The rows another table keeps under a keyed record, taken before it** — so
/// nothing is left pointing at a record that is gone.
///
/// **A transfer record's superseded attempts and checkpoints are keyed by the
/// transfer and not by the server**, so they are taken by the transfers the
/// server's records name; **a failure record's value set** is keyed by the
/// record's own subject. Both inside the walk's one transaction.
fn what_goes_first(
    connection: &rusqlite::Connection,
    keyed: KeyedByTheServer,
    of: &str,
) -> Result<(), RecordFailure> {
    match keyed {
        KeyedByTheServer::PerTransferRecords => {
            for under in ["transfer_superseded_attempt", "transfer_checkpoint"] {
                if table_exists(connection, under)? {
                    connection.execute(
                        &format!(
                            "DELETE FROM {under} WHERE transfer IN
                             (SELECT transfer FROM transfer_record WHERE server = ?1)"
                        ),
                        [of],
                    )?;
                }
            }
        }
        // **A notice's lines go with its record** (`Queue & History` C2a).
        // The 38's piece 15.
        KeyedByTheServer::CatchUpNotices => {
            if table_exists(connection, super::catch_up_notices::ENTRY_TABLE)? {
                connection.execute(
                    &format!(
                        "DELETE FROM {} WHERE server = ?1",
                        super::catch_up_notices::ENTRY_TABLE
                    ),
                    [of],
                )?;
            }
        }
        // **A job record's sets go nowhere**: since item C1 the walk takes
        // no job record at all (see [`the_delete_for`]), so the disturbance
        // marks and the rest stay with the records they belong to.
        KeyedByTheServer::FailureRecords => {
            if table_exists(connection, "failure_record_raised_over")? {
                connection.execute(
                    &format!(
                        "DELETE FROM failure_record_raised_over
                         WHERE durable_subject = ?1 AND durable_subject_kind = '{THE_SERVER_ROWS_KIND}'"
                    ),
                    [of],
                )?;
            }
        }
        _ => {}
    }
    Ok(())
}

/// **The one delete a keyed kind takes.** Most are keyed by the server in a
/// `server` column; **a failure record is keyed by its durable subject and that
/// subject's kind** (`State, identity and recovery` §8.7), which for a server
/// row is the row's own identity under [`THE_SERVER_ROWS_KIND`].
fn the_delete_for(keyed: KeyedByTheServer, table: &str) -> String {
    match keyed {
        KeyedByTheServer::FailureRecords => format!(
            "DELETE FROM {table} WHERE durable_subject = ?1 AND durable_subject_kind = '{THE_SERVER_ROWS_KIND}'"
        ),
        // **The one delete in this walk with a second condition on it, and it
        // is what keeps D21 true while `State, identity and recovery` V1 step
        // 2, at that document's §7, is also kept.**
        //
        // **Those two sentences disagree and the disagreement is the runner's,
        // not this file's** — `Servers` §16 item 26 carries it, filed
        // 2026-09-02 and not settled. `Servers` D4, `Servers` D7, `Servers` D17
        // and `Servers` D21 all rest on a
        // confirmed job record being a history record, keyed by the job
        // identity and left exactly where it is; that step was widened on
        // 2026-08-29 to take *"its provisional and confirmed records"*, on the
        // ground that **a job Smoozy never learned had started rendering leaves
        // no trace**.
        //
        // **Both are true of exactly the same rows under one test**, which is
        // the ground the widening itself states and which `Queue & History` C9
        // already gives a name: a record whose render start has been learned is
        // a history entry, and one whose has not is not. **So the walk took
        // every job record of this server that history never held, and left
        // every one it did** — the runner's ruling of 2026-09-18, which closed
        // item 26 (`Audits/Build/Step 10b/Ruling — the two things piece 6
        // handed up.md`, part 1).
        //
        // **Since item C1 it takes none of them and marks them instead.**
        // *Creator-stated, 2026-09-18:* a job Smoozy submitted and never learned
        // had started, whose server is then destroyed or removed, is kept and
        // shown in History, marked that Smoozy never heard whether it started.
        // `Queue & History` C9's second way in and `State, identity and
        // recovery` V1 step 2, at that document's §7, now say so; **the
        // statement is `job_records`' own**, so this walk and §7 step 1's branch
        // cannot mark different records. D21 still holds: a record history
        // already held is untouched.
        KeyedByTheServer::ProvisionalJobRecords => {
            let _ = table;
            crate::record::job_records::THE_START_WAS_NEVER_HEARD.to_string()
        }
        _ => format!("DELETE FROM {table} WHERE server = ?1"),
    }
}

/// **A server row's kind, as `super::failure_records` writes it** — the same
/// string that file's `DurableSubject::kind_as_text` answers, written here for
/// the delete, and a test below holds the two together.
const THE_SERVER_ROWS_KIND: &str = "server row";

/// Which table each keyed kind lives in, or nothing where its owning module has
/// not been built.
///
/// **None of them is this document's content** — what is this document's is
/// that each is reachable from the row and goes with it.
fn table_for(keyed: KeyedByTheServer) -> Option<&'static str> {
    match keyed {
        // §10.3's one suppression table exists from version 1 — but it is keyed
        // by the **warning identity** rather than by a server, so the per-server
        // rows §10.3 describes arrive with the module that raises them.
        KeyedByTheServer::SuppressedWarningRows => None,
        // **§8.7's failure records and `Server protocol and transfers` S9's
        // transfer records have their tables** — versions 14 and 6 — **and the
        // walk takes them** (`Servers` D17, `State, identity and recovery`
        // F25). Until step 10a round 1 (fix-here item 11) both were mapped to
        // nothing under a note that they *"arrive with their own modules"*,
        // which had stopped being true, so a removal left both behind.
        //
        // **The failure records are deleted here rather than through
        // `FailureRecords::the_subject_was_removed`**, and the reason is the
        // transaction: that call opens its own `BEGIN IMMEDIATE`, which cannot
        // run inside the walk's, and the walk is one atomic unit (`Servers`
        // §14). The two deletes are the same two that call makes.
        KeyedByTheServer::FailureRecords => Some("failure_record"),
        KeyedByTheServer::PerTransferRecords => Some("transfer_record"),
        // **`State, identity and recovery` V1 step 2's job records, in the
        // table version 23 made** — and the one kind this walk marks rather
        // than deletes, for a reason written at [`the_delete_for`].
        KeyedByTheServer::ProvisionalJobRecords => Some("job_record"),
        // **`Telegram` L1's told-record**, in the table version 19 made, keyed
        // by the local server identifier in its `server` column. **This walk is
        // the only thing that takes the entries**: version 21 dropped the
        // trigger version 19 carried for want of this kind.
        KeyedByTheServer::ToldRecord => Some("told_entry"),
        // **`Queue & History` W5's idle-spoken marks**, in the table version
        // 33 made, keyed by the local server identifier in its `server` column
        // — subject-held on the row, so they go with it. The 38's piece 1.
        KeyedByTheServer::IdleSpokenMarks => Some(super::idle_marks::TABLE),
        // **`Queue & History` C2's notice record**, in the tables version 38
        // made — subject-held on the row, so it goes with it, its lines first
        // ([`what_goes_first`]). The 38's piece 15.
        KeyedByTheServer::CatchUpNotices => Some(super::catch_up_notices::TABLE),
    }
}

fn table_exists(connection: &rusqlite::Connection, table: &str) -> Result<bool, RecordFailure> {
    Ok(connection.query_row(
        "SELECT count(*) > 0 FROM sqlite_schema WHERE type = 'table' AND name = ?1",
        [table],
        |row| row.get(0),
    )?)
}

/// `Security and licence` §5.1's three writes over server rows.
///
/// **The machine password is one of the four stored kinds and the register is
/// shared**, so this is the same three writes `Keys`' rows make, over the field
/// D3 names. **Write 1 goes into the register and never into the row** (E3a),
/// which is what lets a debt outlive the row that carried the reference — the
/// case D18 calls *a remembered secret against a machine the user believes
/// Smoozy has forgotten*.
pub struct ServerRowMarks<'a>(pub &'a rusqlite::Connection);

impl EraseMarks for ServerRowMarks<'_> {
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
    /// mark stands.**
    fn remove_reference(&mut self, item: &HeldSecret) -> Result<(), EraseFailure> {
        self.the_rows_reference(item)
    }

    /// **Write 3: the reference and the mark together, never the mark alone.**
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

    /// **Every subject currently owed an erase**, read from the register rather
    /// than from any row.
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

impl ServerRowMarks<'_> {
    /// **A row that is already gone is the state the caller asked for**, so no
    /// rows changed is not a failure: the mark's own landing is the register's
    /// insert, which answers for itself.
    fn the_rows_reference(&self, item: &HeldSecret) -> Result<(), EraseFailure> {
        self.0
            .execute(
                "UPDATE server_row SET machine_password_subject = NULL
                 WHERE machine_password_subject = ?1",
                [item.subject.as_str()],
            )
            .map(|_| ())
            .map_err(|_| EraseFailure)
    }
}

/// The retained stored kinds, in the same words the register already holds them in.
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::record::LocalRecord;
    use smoozy_servers::store::enumeration;
    use std::time::{Duration, SystemTime};

    fn a_record(name: &str) -> LocalRecord {
        let path = crate::scratch_folders::a_folder(format!("smoozy-server-rows-{name}-{}", std::process::id()));
        LocalRecord::open_at(&path.join("local-record.sqlite3")).expect("a record")
    }

    fn a_row() -> ServerRow {
        ServerRow::saved(
            Identity::issue(IdentityKind::LocalServer)
                .expect("randomness")
                .as_text(),
            Host::what_the_user_typed("5.180.183.77").expect("a host"),
            Port::what_the_user_typed(33510).expect("a port"),
            Username::what_the_user_typed("root").expect("a username"),
            KeyReference {
                local_key_identifier: "key-1".into(),
                fingerprint: "SHA256:aaa".into(),
            },
        )
    }

    /// **The clock a reading is taken on is named by the server row it was read
    /// from** (`State, identity and recovery` D5), so the helper is handed the
    /// row's own identifier rather than a name of its own — two machines'
    /// readings can then never be subtracted from one another.
    fn a_reading(server: &str) -> HardwareReading {
        HardwareReading::taken(
            Cards::TheCards(vec![
                Card {
                    unique_identifier: "GPU-aaa".into(),
                    bus_location: "0000:01:00.0".into(),
                    make: CardMake::from_the_drivers_own_query("NVIDIA GeForce RTX 4090"),
                    total_memory_bytes: Some(25_757_220_864),
                },
                Card {
                    unique_identifier: "GPU-bbb".into(),
                    bus_location: "0000:02:00.0".into(),
                    make: CardMake::from_the_drivers_own_query("NVIDIA GeForce RTX 4090"),
                    // `Servers` H8's stated absence.
                    total_memory_bytes: None,
                },
            ]),
            MachineMemory::TheAllocation(16_051_527_680),
            None,
            SystemTime::UNIX_EPOCH + Duration::from_nanos(1_000_000_123),
            server,
            SystemTime::UNIX_EPOCH + Duration::from_nanos(2_000_000_456),
            Some("inst-1".into()),
        )
    }

    #[test]
    fn the_machine_root_comes_back_with_the_identity_it_was_recorded_under() {
        // `Job files and outputs` **L1**, and the reason both halves travel
        // together is §2.3: a machine-derived value carries the installation
        // identity it was read under, or it says nothing about which
        // installation chose it.
        let record = a_record("the-machine-root");
        let mut row = a_row();
        row.recorded_machine_root = Some(RecordedMachineRoot {
            path: "/ephemeral/smoozy".into(),
            // L4a's second arrangement, recorded in the same durable act as the
            // root's — `Server Setup` R7b.
            the_package_areas_own_location: Some("/state/smoozy".into()),
            read_under: "inst-1".into(),
        });
        add(record.connection(), &row).expect("added");
        assert_eq!(all(record.connection()).expect("read"), vec![row]);
    }

    #[test]
    fn a_root_recorded_with_no_identity_beside_it_is_not_read_back_at_all() {
        // Half a value is not a value. A path with no identity beside it would
        // be a machine-derived value with nothing saying which installation it
        // belongs to, which §2.3 forbids holding.
        let record = a_record("half-a-root");
        let row = a_row();
        add(record.connection(), &row).expect("added");
        record
            .connection()
            .execute(
                "UPDATE server_row SET machine_root_path = '/ephemeral/smoozy'",
                [],
            )
            .expect("written by hand");
        let back = all(record.connection()).expect("read");
        assert!(back[0].recorded_machine_root.is_none());
    }

    #[test]
    fn a_saved_row_comes_back_field_for_field() {
        let record = a_record("round-trip");
        let row = a_row();
        add(record.connection(), &row).expect("added");
        assert_eq!(all(record.connection()).expect("read"), vec![row]);
    }

    #[test]
    fn a_saved_row_records_nothing_about_a_connection() {
        // D22, and it is why there is no clearing step to fail at start-up.
        let record = a_record("nothing-about-a-connection");
        add(record.connection(), &a_row()).expect("added");
        let back = all(record.connection()).expect("read");
        assert!(back[0].last_confirmed_installation_identity.is_none());
        assert!(back[0].remembered_hardware_reading.is_none());
        assert!(back[0].event_cursor.is_none());
    }

    #[test]
    fn every_field_of_d3_survives_a_write_and_a_read() {
        let record = a_record("every-field");
        let mut row = a_row();
        add(record.connection(), &row).expect("added");

        row.label = Some("hetzner".into());
        row.colour = Some("green".into());
        row.remembered_setup = Some(RememberedSetup {
            options: vec!["blender".into(), "telegram".into()],
        });
        row.remembered_host_key = Some(RememberedHostKey::approved(b"ssh-ed25519 AAAA".to_vec()));
        row.remembered_hardware_reading = Some(a_reading(&row.local_server_identifier));
        row.last_confirmed_installation_identity = Some("inst-1".into());
        row.event_cursor = Some(41);
        row.machine_password_subject =
            Some(Subject::issued_by_the_local_record(&row.local_server_identifier));
        write(record.connection(), &row).expect("written");

        let back = all(record.connection()).expect("read");
        assert_eq!(back, vec![row]);
    }

    fn a_driver(version: &str, cuda: Option<&str>, placed: WhereK1PlacesIt) -> TheDriversOwnAnswer {
        TheDriversOwnAnswer {
            version: DriverVersion::parse(version).expect("a driver version"),
            cuda_version: cuda.map(str::to_string),
            where_k1_placed_the_machine: placed,
        }
    }

    /// **`Servers` H11b: the driver's version, the CUDA version and the kind
    /// are part of the held reading**, so they come back with it — and with
    /// the desktop's own moment beside them, which is what the card's *last
    /// seen* is drawn from when no connection stands. The fourth piece of the
    /// OptiX and driver work; until then the reading came back with no driver
    /// part at all.
    #[test]
    fn the_drivers_values_come_back_with_the_reading_they_were_read_in() {
        let record = a_record("the-driver");
        let mut row = a_row();
        add(record.connection(), &row).expect("added");
        for driver in [
            a_driver("550.163.01", Some("12.4"), WhereK1PlacesIt::AContainer),
            a_driver("575.57.08", None, WhereK1PlacesIt::ItsOwnKernel),
            a_driver("570.211.01", Some("12.8"), WhereK1PlacesIt::NowhereItPlaces),
        ] {
            row.remembered_hardware_reading = Some(
                a_reading(&row.local_server_identifier).with_the_driver(Some(driver.clone())),
            );
            write(record.connection(), &row).expect("written");
            let back = one(record.connection(), &row.local_server_identifier)
                .expect("read")
                .expect("the row")
                .remembered_hardware_reading
                .expect("the reading");
            assert_eq!(back.the_driver, Some(driver));
            assert_eq!(
                back.the_desktops_own_moment.taken_at(),
                SystemTime::UNIX_EPOCH + Duration::from_nanos(2_000_000_456),
                "the last seen moment travels with the driver's values"
            );
        }
        // **`Servers` H4: a reading replaces the held one whole**, so a later
        // reading whose driver did not answer leaves no driver behind.
        row.remembered_hardware_reading = Some(a_reading(&row.local_server_identifier));
        write(record.connection(), &row).expect("written");
        let back = one(record.connection(), &row.local_server_identifier)
            .expect("read")
            .expect("the row");
        assert_eq!(back.remembered_hardware_reading.expect("the reading").the_driver, None);
    }

    /// **`Servers` H11b's OptiX-library bullet: the missing library is part of
    /// the held reading**, so it comes back with it and goes with it — a later
    /// reading whose loader found the library leaves no mark behind (H4).
    /// Added 2026-11-02, record version 58.
    #[test]
    fn a_missing_optix_library_comes_back_with_the_reading_and_goes_with_it() {
        let record = a_record("the-optix-library");
        let mut row = a_row();
        let with_the_driver = |row: &ServerRow| {
            a_reading(&row.local_server_identifier).with_the_driver(Some(a_driver(
                "580.178.04",
                Some("13.0"),
                WhereK1PlacesIt::AContainer,
            )))
        };
        row.remembered_hardware_reading = Some(with_the_driver(&row).with_the_optix_library_missing(true));
        add(record.connection(), &row).expect("added");
        let missing = |record: &LocalRecord| {
            one(record.connection(), &row.local_server_identifier)
                .expect("read")
                .expect("the row")
                .remembered_hardware_reading
                .expect("the reading")
                .the_optix_library_is_missing
        };
        assert!(missing(&record), "the add carries it");
        let mut again = row.clone();
        again.remembered_hardware_reading = Some(with_the_driver(&row));
        write(record.connection(), &again).expect("written");
        assert!(!missing(&record), "a reading that found the library replaces it");
        again.remembered_hardware_reading = Some(with_the_driver(&row).with_the_optix_library_missing(true));
        write(record.connection(), &again).expect("written");
        assert!(missing(&record), "a whole-row write carries it");
    }

    /// **A driver part that cannot say where the machine was placed is no
    /// driver part**, and the rest of the reading still stands.
    #[test]
    fn a_driver_part_whose_kind_will_not_read_back_is_left_out_and_the_reading_stands() {
        let record = a_record("the-driver-kind");
        let mut row = a_row();
        row.remembered_hardware_reading = Some(
            a_reading(&row.local_server_identifier).with_the_driver(Some(a_driver(
                "550.163.01",
                Some("12.4"),
                WhereK1PlacesIt::AContainer,
            ))),
        );
        add(record.connection(), &row).expect("added");
        record
            .connection()
            .execute("UPDATE server_row SET hardware_driver_placed = 'a guess'", [])
            .expect("changed");
        let back = one(record.connection(), &row.local_server_identifier)
            .expect("read")
            .expect("the row")
            .remembered_hardware_reading
            .expect("the reading stands");
        assert_eq!(back.the_driver, None);
        assert!(matches!(back.cards, Cards::TheCards(_)));
    }

    /// **`Servers` D3's row of 2026-10-11: the missed bar is kept on the row**,
    /// written alone at a run's ending, carried by every whole-row write, and
    /// replaced by the next run's own answer.
    #[test]
    fn the_missed_bar_is_kept_on_the_row_until_a_run_replaces_it() {
        let record = a_record("the-missed-bar");
        let row = a_row();
        add(record.connection(), &row).expect("added");
        let server = row.local_server_identifier.clone();
        let missed = |record: &LocalRecord| {
            one(record.connection(), &server)
                .expect("read")
                .expect("the row")
                .the_driver_missed_the_optix_bar
        };
        assert!(!missed(&record), "a new row records no miss");
        the_driver_miss_is_written(record.connection(), &row.local_server_identifier, true)
            .expect("written");
        assert!(missed(&record));
        // **A whole-row write carries it**: an edit or a connection's reading
        // does not take it away.
        let mut back = one(record.connection(), &row.local_server_identifier)
            .expect("read")
            .expect("the row");
        back.label = Some("renamed".into());
        write(record.connection(), &back).expect("written");
        assert!(missed(&record));
        the_driver_miss_is_written(record.connection(), &row.local_server_identifier, false)
            .expect("written");
        assert!(!missed(&record));
    }

    #[test]
    fn the_two_moments_of_a_reading_come_back_to_the_nanosecond() {
        // `Servers` H2a's *last seen* mark is drawn from the desktop's own
        // moment, and a store that rounded it would move the mark on every
        // read-back.
        let record = a_record("two-moments");
        let mut row = a_row();
        row.remembered_hardware_reading = Some(a_reading(&row.local_server_identifier));
        add(record.connection(), &row).expect("added");
        let back = all(record.connection()).expect("read");
        let held = back[0]
            .remembered_hardware_reading
            .as_ref()
            .expect("a reading");
        assert_eq!(
            held.the_machines_own_moment.taken_at(),
            SystemTime::UNIX_EPOCH + Duration::from_nanos(1_000_000_123)
        );
        assert_eq!(
            held.the_desktops_own_moment.taken_at(),
            SystemTime::UNIX_EPOCH + Duration::from_nanos(2_000_000_456)
        );
        // D5's forbidden subtraction still answers nothing, across the store.
        assert_eq!(held.how_long_between_the_two_moments(), None);
    }

    #[test]
    fn the_card_count_is_the_number_of_rows_and_never_a_figure_beside_them() {
        // `Servers` H5's rule kept by the store as well as by the reading.
        let record = a_record("card-count");
        let mut row = a_row();
        row.remembered_hardware_reading = Some(a_reading(&row.local_server_identifier));
        add(record.connection(), &row).expect("added");
        let back = all(record.connection()).expect("read");
        let Some(Cards::TheCards(cards)) = back[0]
            .remembered_hardware_reading
            .as_ref()
            .map(|reading| reading.cards.clone())
        else {
            panic!("the cards")
        };
        assert_eq!(cards.len(), 2);
        assert_eq!(cards[1].total_memory_bytes, None);
    }

    #[test]
    fn the_four_card_states_are_four_and_never_collapse() {
        // `Servers` H11's table survives the store: *empty* is the one row
        // that concludes no card, and a scan with no route must not come back
        // as one.
        let record = a_record("four-card-states");
        for (position, cards) in [
            Cards::PresentAndUnreadable(WhyUnreadable::TheDriverIsAbsent),
            Cards::PresentAndUnreadable(WhyUnreadable::TheDriverAnsweredWithNone),
            Cards::NoCard,
            Cards::CouldNotBeEstablished,
        ]
        .into_iter()
        .enumerate()
        {
            let mut row = a_row();
            row.local_server_identifier = Identity::issue(IdentityKind::LocalServer)
                .expect("randomness")
                .as_text();
            let mut reading = a_reading(&row.local_server_identifier);
            reading.cards = cards.clone();
            row.remembered_hardware_reading = Some(reading);
            add(record.connection(), &row).expect("added");
            let back = one(record.connection(), &row.local_server_identifier)
                .expect("read")
                .expect("the row");
            assert_eq!(
                back.remembered_hardware_reading
                    .expect("a reading")
                    .cards,
                cards,
                "state {position}"
            );
        }
    }

    #[test]
    fn an_edit_is_one_durable_write_and_the_lists_go_with_it() {
        // D16, and `Servers` H4's *a reading replaces it whole*: the previous
        // reading's cards do not survive beside the new one.
        let record = a_record("one-durable-write");
        let mut row = a_row();
        row.remembered_hardware_reading = Some(a_reading(&row.local_server_identifier));
        row.remembered_setup = Some(RememberedSetup {
            options: vec!["blender".into()],
        });
        add(record.connection(), &row).expect("added");

        // D13: a host edit drops the hardware, the cursor and the confirmed
        // identity, and starts the host key over.
        row.remembered_hardware_reading = None;
        row.remembered_setup = None;
        write(record.connection(), &row).expect("written");

        let cards: i64 = record
            .connection()
            .query_row("SELECT count(*) FROM server_hardware_card", [], |row| {
                row.get(0)
            })
            .expect("counted");
        let options: i64 = record
            .connection()
            .query_row(
                "SELECT count(*) FROM server_remembered_setup_option",
                [],
                |row| row.get(0),
            )
            .expect("counted");
        assert_eq!((cards, options), (0, 0));
    }

    #[test]
    fn a_removal_walks_the_enumeration_and_leaves_nothing_of_the_row() {
        // D17 and D6: the walk is the enumeration, and what it reaches is what
        // `Servers` names rather than what this file remembers to delete.
        let record = a_record("the-walk");
        let mut row = a_row();
        row.remembered_hardware_reading = Some(a_reading(&row.local_server_identifier));
        row.remembered_setup = Some(RememberedSetup {
            options: vec!["blender".into()],
        });
        add(record.connection(), &row).expect("added");
        assert_eq!(enumeration(&row).keyed_records.len(), 7);
        // **`Queue & History` W5's idle-spoken mark is subject-held on the
        // row** (the 38's piece 1), so the walk takes it.
        record
            .connection()
            .execute(
                "INSERT INTO idle_spoken_mark (server, installation, opening_event) VALUES (?1, 'i', NULL)",
                [row.local_server_identifier.as_str()],
            )
            .expect("a mark");
        // **And the installs still waiting on it** (`Server Setup` Y9a, owed
        // piece 4 of step 10c), a field of the row.
        crate::record::installs_not_begun::the_ones_waiting_are_kept(
            record.connection(),
            &row.local_server_identifier,
            &["4.5.14".to_string()],
        )
        .expect("a list");
        // **So is `Queue & History` C2's notice record, and its lines** (the
        // 38's piece 15).
        record
            .connection()
            .execute(
                "INSERT INTO catch_up_notice (server, the_account_is_incomplete) VALUES (?1, 0)",
                [row.local_server_identifier.as_str()],
            )
            .expect("a notice");
        record
            .connection()
            .execute(
                "INSERT INTO catch_up_notice_entry (server, job, sequence, place, kind, ending)
                 VALUES (?1, 'j', 4, 0, 'ended', 'finished')",
                [row.local_server_identifier.as_str()],
            )
            .expect("a line");

        let not_yet_built =
            remove_walking_the_enumeration(record.connection(), &row.local_server_identifier)
                .expect("removed");
        // **One of D4's five has no table** — the suppression rows keyed by a
        // server — so the walk reports that one rather than silently reaching
        // none. The told-record, the failure records and the transfer records
        // have tables and are reached (step 10a round 1, fix-here item 11);
        // **the job records got theirs at version 23**, step 10b piece 6.
        assert_eq!(not_yet_built, vec![KeyedByTheServer::SuppressedWarningRows]);

        assert!(all(record.connection()).expect("read").is_empty());
        for table in [
            "server_hardware_card",
            "server_remembered_setup_option",
            "server_install_not_begun",
            "server_row",
            "idle_spoken_mark",
            "catch_up_notice",
            "catch_up_notice_entry",
        ] {
            let left: i64 = record
                .connection()
                .query_row(&format!("SELECT count(*) FROM {table}"), [], |row| {
                    row.get(0)
                })
                .expect("counted");
            assert_eq!(left, 0, "{table}");
        }
    }

    #[test]
    fn a_removal_leaves_no_told_entry_behind_by_the_walk_alone() {
        // D17 and D6, with `Telegram` L1: the told-record is a kind the walk
        // reaches. **Nothing else takes the entries** — version 21 dropped
        // version 19's trigger — so a bare delete of a row leaves its entries
        // standing, and the walk is what clears them.
        let record = a_record("told-by-the-walk");
        let c = record.connection();
        let triggers: i64 = c
            .query_row("SELECT count(*) FROM sqlite_schema WHERE type = 'trigger'", [], |row| {
                row.get(0)
            })
            .expect("counted");
        assert_eq!(triggers, 0);

        let removed = a_row();
        let deleted_bare = a_row();
        for row in [&removed, &deleted_bare] {
            add(c, row).expect("added");
            for thing in ["label", "token_generation"] {
                c.execute(
                    "INSERT INTO told_entry (server, thing, value, acknowledged_at, installation)
                     VALUES (?1, ?2, 'told', 0, 'installation-1')",
                    [row.local_server_identifier.as_str(), thing],
                )
                .expect("a told entry");
            }
        }
        let told = |server: &str| -> i64 {
            c.query_row("SELECT count(*) FROM told_entry WHERE server = ?1", [server], |row| {
                row.get(0)
            })
            .expect("counted")
        };

        remove_walking_the_enumeration(c, &removed.local_server_identifier).expect("removed");
        assert_eq!(told(&removed.local_server_identifier), 0);
        assert_eq!(told(&deleted_bare.local_server_identifier), 2);

        c.execute(
            "DELETE FROM server_row WHERE identifier = ?1",
            [deleted_bare.local_server_identifier.as_str()],
        )
        .expect("deleted bare");
        assert_eq!(told(&deleted_bare.local_server_identifier), 2);
    }

    /// **Step 10a round 1, fix-here item 11: a removal takes the server's
    /// failure records, their value sets, its transfer records and the rows
    /// under them**, and nothing of another server's. It fails on the walk
    /// that mapped both kinds to nothing.
    #[test]
    fn a_removal_takes_the_failure_and_transfer_records_with_it() {
        let record = a_record("failures-and-transfers");
        let c = record.connection();
        let removed = a_row();
        let kept = a_row();
        let kept = ServerRow {
            local_server_identifier: Identity::issue(IdentityKind::LocalServer)
                .expect("randomness")
                .as_text(),
            ..kept
        };
        for row in [&removed, &kept] {
            add(c, row).expect("added");
            let server = Identity::read_back(IdentityKind::LocalServer, &row.local_server_identifier)
                .expect("an identity");
            let subject = crate::record::failure_records::DurableSubject::ServerRow(server);
            assert_eq!(subject.kind_as_text(), THE_SERVER_ROWS_KIND);
            let mut records = crate::record::failure_records::FailureRecords::over(c);
            let key = crate::record::failure_records::ConditionKey::over(
                &subject,
                "the server connection",
                None,
            );
            records
                .a_condition_was_raised(&key, &subject, &["disconnected"], true)
                .expect("raised");
            let transfer = format!("transfer-{}", row.local_server_identifier);
            c.execute(
                "INSERT INTO transfer_record (transfer, attempt, direction, server, source_path,
                    final_path, staging_directory, total_size_expected, written_by_run,
                    cleanup_is_owed)
                 VALUES (?1, 1, 'up', ?2, '/a', '/b', '/c', 1, 'run', 0)",
                [transfer.as_str(), row.local_server_identifier.as_str()],
            )
            .expect("a transfer record");
            c.execute(
                "INSERT INTO transfer_checkpoint (transfer, offset_into_the_staged_file, digest)
                 VALUES (?1, 1, 'd')",
                [transfer.as_str()],
            )
            .expect("a checkpoint");
        }
        let count = |sql: &str, server: &str| -> i64 {
            c.query_row(sql, [server], |row| row.get(0)).expect("counted")
        };
        let failures = "SELECT count(*) FROM failure_record WHERE durable_subject = ?1";
        let values = "SELECT count(*) FROM failure_record_raised_over WHERE durable_subject = ?1";
        let transfers = "SELECT count(*) FROM transfer_record WHERE server = ?1";
        let checkpoints = "SELECT count(*) FROM transfer_checkpoint WHERE transfer = 'transfer-' || ?1";

        remove_walking_the_enumeration(c, &removed.local_server_identifier).expect("removed");
        for sql in [failures, values, transfers, checkpoints] {
            assert_eq!(count(sql, &removed.local_server_identifier), 0, "{sql}");
            assert_eq!(count(sql, &kept.local_server_identifier), 1, "{sql}");
        }
    }

    /// **A removal keeps every job record, and the ones history never held
    /// become history entries marked *start never heard*** — item C1,
    /// *Creator-stated, 2026-09-18*, carried into `Queue & History` C9 and
    /// `State, identity and recovery` V1 step 2, at that document's §7.
    ///
    /// **This test failed before the change**: the walk deleted the record
    /// whose start was never learned, with its disturbance mark, and History
    /// held one entry where it now holds two.
    #[test]
    fn a_removal_keeps_the_job_records_and_marks_the_ones_history_never_held() {
        use crate::record::job_records::JobRecords;

        let record = a_record("jobs-at-a-removal");
        let c = record.connection();
        let row = a_row();
        add(c, &row).expect("added");
        let server = Identity::read_back(IdentityKind::LocalServer, &row.local_server_identifier)
            .expect("an identity");

        let records = JobRecords::over(c);
        let never_started = Identity::issue(IdentityKind::Job).expect("randomness");
        let rendered = Identity::issue(IdentityKind::Job).expect("randomness");
        for (job, learned) in [(&never_started, false), (&rendered, true)] {
            c.execute(
                "INSERT INTO job_record (job, server, standing, render_start_learned,
                    submitted_with_no_scene_named, blender_build_path, blender_version,
                    blender_origin, device_ask, note_attempts, file_values_stood)
                 VALUES (?1, ?2, 'confirmed', ?3, 0, '/b', '4.5.12', 'resolved for them',
                         'the file''s own, unread', 0, 0)",
                rusqlite::params![job.as_text(), row.local_server_identifier, learned as i64],
            )
            .expect("a job record");
            c.execute(
                "INSERT INTO job_record_disturbance (job, position, member, act_intended, reading)
                 VALUES (?1, 0, '/a/tex.png', 'replace', 'disturbed')",
                [job.as_text()],
            )
            .expect("a disturbance");
        }

        remove_walking_the_enumeration(c, &row.local_server_identifier).expect("removed");

        assert!(records.read(&never_started).expect("read").is_some());
        assert!(records.read(&rendered).expect("read").is_some());
        let marks: i64 = c
            .query_row("SELECT count(*) FROM job_record_disturbance", [], |row| {
                row.get(0)
            })
            .expect("counted");
        assert_eq!(marks, 2, "both entries keep their marks");
        // **Both are drawable, and each says which way it came in.**
        let held = records.in_history().expect("read");
        assert_eq!(held.entries.len(), 2);
        for entry in &held.entries {
            assert_eq!(
                entry.start_never_heard,
                entry.record.job == never_started,
                "only the record whose start was never learned carries the mark"
            );
        }
        let _ = server;
    }

    #[test]
    fn a_removal_leaves_a_history_tombstone_exactly_where_it_is() {
        // D21 and D5: history is keyed by the job identity, is outside the
        // enumeration, and a removal reaches none of it.
        let record = a_record("history-untouched");
        let row = a_row();
        add(record.connection(), &row).expect("added");
        record
            .connection()
            .execute(
                "INSERT INTO history_tombstone (job, deleted_at, machine_side_copy)
                 VALUES ('job-1', 0, 'kept')",
                [],
            )
            .expect("a tombstone");
        remove_walking_the_enumeration(record.connection(), &row.local_server_identifier)
            .expect("removed");
        let left: i64 = record
            .connection()
            .query_row("SELECT count(*) FROM history_tombstone", [], |row| row.get(0))
            .expect("counted");
        assert_eq!(left, 1);
    }

    #[test]
    fn the_debt_outlives_the_row_that_carried_the_reference() {
        // E3a from the server row's side: a refused erase leaves the mark with
        // the row gone, and the sweep still has its subject.
        let record = a_record("the-debt");
        let mut row = a_row();
        row.machine_password_subject =
            Some(Subject::issued_by_the_local_record(&row.local_server_identifier));
        add(record.connection(), &row).expect("added");

        let held = HeldSecret::new(
            StoredKind::MachinePassword,
            Subject::issued_by_the_local_record(&row.local_server_identifier),
        );
        let mut marks = ServerRowMarks(record.connection());
        marks.mark_owing_an_erase(&held).expect("marked");
        marks.remove_reference(&held).expect("reference dropped");
        remove_walking_the_enumeration(record.connection(), &row.local_server_identifier)
            .expect("removed");

        assert_eq!(
            ServerRowMarks(record.connection())
                .marks()
                .expect("read")
                .len(),
            1
        );
    }

    #[test]
    fn a_row_whose_identifier_will_not_read_back_is_skipped_rather_than_invented() {
        // `State, identity and recovery` §2.1, and `Servers` C6: nothing is
        // derived from a host, a port or a username to stand in for it.
        let record = a_record("bad-identifier");
        let row = a_row();
        add(record.connection(), &row).expect("added");
        record
            .connection()
            .execute(
                "UPDATE server_row SET identifier = 'not an identity'",
                [],
            )
            .expect("rewritten");
        assert!(all(record.connection()).expect("read").is_empty());
    }

    #[test]
    fn the_remembered_host_key_never_renders_its_value() {
        // D9's *never logged*, kept by the type rather than by care at each
        // site — the same shape the transport's own credential uses.
        let key = RememberedHostKey::approved(b"ssh-ed25519 AAAA".to_vec());
        let rendered = format!("{key:?}");
        assert!(!rendered.contains("AAAA"));
        assert!(!rendered.contains("115"));
    }

    #[test]
    fn a_reading_with_no_free_space_figure_is_read_back_as_one() {
        // The figure is absent until `Server protocol and transfers` F2's route
        // exists, and **nothing is substituted for it** — not a whole-machine
        // figure and not one derived from a filesystem list.
        let record = a_record("no-free-space");
        let mut row = a_row();
        row.remembered_hardware_reading = Some(a_reading(&row.local_server_identifier));
        add(record.connection(), &row).expect("added");
        let back = all(record.connection()).expect("read");
        assert!(back[0]
            .remembered_hardware_reading
            .as_ref()
            .expect("a reading")
            .free_space
            .is_none());
    }
}
