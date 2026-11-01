//! The settings the user holds, in the local record — the download folder they
//! chose, the two transfer speed caps, and the four notification switches.
//!
//! **Where each is kept is the plans'; the store is this document's**, which is
//! the split every other keeper in this folder draws.
//!
//! - **The folder**: `Download` H2 — *"the proposed folder, or the folder the
//!   user pointed at instead, set once and user-held"*. **Only the folder the
//!   user chose is written.** The proposal is §8's and W3 says nothing caches
//!   it, so *nothing held* means *use the proposal, read fresh*.
//! - **The caps**: R3 — *"The setting that sets it is durable and is
//!   user-held"*; R1, one per direction and never one for both; R2, bytes per
//!   second; R8, no cap set is the ordinary state.
//! - **The switches**: T8 — *"the user's own record of what they want"* — and
//!   T17a's third bullet, which says what a switch governs: **whether Smoozy
//!   attempts the post, never whether one arrives.** The check itself is
//!   [`crate::notifications::TheNotificationService::post`]'s.
//! - **The *Show hidden files* choice**: `File Browser` L20 and `State,
//!   identity and recovery` I7 — one value for the app, absent meaning off,
//!   the untick clearing it. Added 2026-10-02 at version 30; it is the File
//!   browser's and not this screen's, so [`TheSettings`] does not carry it.
//! - **All seven are *settings* in `State, identity and recovery` J1 and are
//!   user-held under that document's §11.1**: nothing ages them and nothing
//!   sweeps them; only the user's own act changes one.
//!
//! **What a switch nobody has turned is set to is not in any plan**, and it is
//! ruled here under the standing delegation and written in `Audits/Build/Desktop
//! app and delivery/Decisions record.md` under 2026-09-13: **on**. The spec's
//! own sentence is *"Each kind can be switched off in Settings"*, which makes
//! off the act and on the ground it is taken from; and the drawing's `set` state
//! is a user's screen with one kind already switched off rather than a first
//! launch. [`A_KIND_NOBODY_HAS_TURNED_IS_ATTEMPTED`] is the one place it lives.
//!
//! **An absent value is the default and never a stored copy of it** — the shape
//! `State, identity and recovery` I3 rule 4 gives a pane width. **L8a is
//! untouched**: an absent setting is an ordinary state, and a read that did not
//! answer is a failure, which this file returns rather than turning into a
//! default.
//!
//! **Markers used below that belong to another document**: none are left bare.
//! `Download`'s, `State, identity and recovery`'s and `Server protocol and
//! transfers`' are written with their document at every site, because this
//! crate's own plan defines clauses of the same short names.

use smoozy_server_protocol::transfers::Direction;

use super::RecordFailure;
use crate::notifications::kinds::Kind;
use crate::scheduling::{BytesPerSecond, TheCap};

/// **What a kind nobody has turned is set to**, ruled 2026-09-13. See the head
/// of this file.
pub const A_KIND_NOBODY_HAS_TURNED_IS_ATTEMPTED: bool = true;

/// **The four switches, as the record holds them.** One answer per kind in
/// [`Kind::ALL`]'s order, with a kind nobody has turned already at its default.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TheSwitches {
    attempted: [bool; 4],
}

impl TheSwitches {
    /// Every kind at its default — a record in which nobody has turned one.
    pub fn nobody_has_turned_one() -> TheSwitches {
        TheSwitches {
            attempted: [A_KIND_NOBODY_HAS_TURNED_IS_ATTEMPTED; 4],
        }
    }

    /// **Whether Smoozy attempts a post of this kind** — T17a's third bullet.
    pub fn attempts(&self, kind: Kind) -> bool {
        self.attempted[the_place_of(kind)]
    }

    /// The same, with one kind changed. For a caller that has just written the
    /// change and holds a copy.
    pub fn with(mut self, kind: Kind, attempted: bool) -> TheSwitches {
        self.attempted[the_place_of(kind)] = attempted;
        self
    }
}

fn the_place_of(kind: Kind) -> usize {
    Kind::ALL
        .iter()
        .position(|one| *one == kind)
        .expect("every kind is in Kind::ALL")
}

/// **The one spelling a kind is stored under.** `Kind::ALL` is the closed set
/// and this is its name in the record; a test holds the two together.
pub fn the_stored_name_of(kind: Kind) -> &'static str {
    match kind {
        Kind::AJobFinished => "a-job-finishes",
        Kind::AJobFailedOrWasGivenUp => "a-job-fails",
        Kind::AConnectionCouldNotBeRestored => "a-connection-could-not-be-restored",
        Kind::AMachineHasBeenIdle => "a-server-has-had-nothing-to-do",
    }
}

/// The kind a stored name is, or nothing.
pub fn the_kind_stored_as(name: &str) -> Option<Kind> {
    Kind::ALL
        .into_iter()
        .find(|kind| the_stored_name_of(*kind) == name)
}

/// **Everything the user holds on this screen**, as the record answers it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TheSettings {
    /// **The folder the user chose, and nothing where they chose none.** Never
    /// the system's proposal (§8 W3).
    pub the_chosen_download_folder: Option<Vec<u8>>,
    /// `Desktop app and delivery` R1: one per direction.
    pub the_upload_cap: TheCap,
    pub the_download_cap: TheCap,
    pub the_switches: TheSwitches,
}

/// **Read all seven.** A row that is not there is the default for every value
/// in it; a read that does not answer is a failure and is returned as one.
pub fn read(connection: &rusqlite::Connection) -> Result<TheSettings, RecordFailure> {
    let row: Option<(Option<Vec<u8>>, Option<i64>, Option<i64>)> = {
        let mut asking = connection.prepare(
            "SELECT download_folder, upload_cap_bytes_per_second, download_cap_bytes_per_second
             FROM setting WHERE the_one_row = 1",
        )?;
        let mut rows = asking.query([])?;
        match rows.next()? {
            Some(row) => Some((row.get(0)?, row.get(1)?, row.get(2)?)),
            None => None,
        }
    };
    let (folder, up, down) = row.unwrap_or((None, None, None));

    let mut switches = TheSwitches::nobody_has_turned_one();
    let mut asking = connection.prepare("SELECT kind, attempted FROM notification_switch")?;
    let rows = asking.query_map([], |row| Ok((row.get::<_, String>(0)?, row.get::<_, i64>(1)?)))?;
    for row in rows {
        let (name, attempted) = row?;
        // **A row naming no kind this binary knows is left alone rather than
        // read as a failure**: T9 closes the set at four, so a fifth name can
        // only be a later binary's, and it governs nothing this one posts.
        if let Some(kind) = the_kind_stored_as(&name) {
            switches = switches.with(kind, attempted != 0);
        }
    }

    Ok(TheSettings {
        the_chosen_download_folder: folder,
        the_upload_cap: a_cap_from(up, "upload_cap_bytes_per_second")?,
        the_download_cap: a_cap_from(down, "download_cap_bytes_per_second")?,
        the_switches: switches,
    })
}

/// **A stored cap, read back.** The table refuses nought and below, so a value
/// that is not a positive byte count is not a cap this record could have
/// written — and it is a read that did not answer rather than *no cap* (L8a),
/// because reading it as no cap would lift a limit the user set.
fn a_cap_from(stored: Option<i64>, column: &'static str) -> Result<TheCap, RecordFailure> {
    match stored {
        None => Ok(TheCap::NoCapIsSet),
        Some(bytes) => match u64::try_from(bytes) {
            Ok(bytes) if bytes > 0 => Ok(TheCap::Of(BytesPerSecond(bytes))),
            _ => Err(RecordFailure::Engine {
                reported: format!("{column} holds {bytes}, which is not a cap"),
            }),
        },
    }
}

/// **The user chose a folder** — `Download` H2's *"the folder the user pointed
/// at instead"*. It replaces the one before it; a single download sent
/// elsewhere never comes through here (`Download` M1).
pub fn the_download_folder_was_chosen(
    connection: &rusqlite::Connection,
    folder: &[u8],
) -> Result<(), RecordFailure> {
    connection.execute(
        "INSERT INTO setting (the_one_row, download_folder) VALUES (1, ?1)
         ON CONFLICT(the_one_row) DO UPDATE SET download_folder = excluded.download_folder",
        [folder],
    )?;
    Ok(())
}

/// **The user set a cap, or cleared one** — R3's durable setting. **No cap is
/// written as nothing** rather than as a large number (R8).
pub fn a_cap_was_set(
    connection: &rusqlite::Connection,
    direction: Direction,
    cap: TheCap,
) -> Result<(), RecordFailure> {
    let bytes: Option<i64> = match cap {
        TheCap::NoCapIsSet => None,
        TheCap::Of(BytesPerSecond(bytes)) => Some(i64::try_from(bytes).map_err(|_| {
            RecordFailure::Engine {
                reported: format!(
                    "{} per second is more than the record holds",
                    crate::typography::a_figure_with_its_unit(bytes, "bytes")
                ),
            }
        })?),
    };
    let statement = match direction {
        Direction::Up => {
            "INSERT INTO setting (the_one_row, upload_cap_bytes_per_second) VALUES (1, ?1)
             ON CONFLICT(the_one_row) DO UPDATE
             SET upload_cap_bytes_per_second = excluded.upload_cap_bytes_per_second"
        }
        Direction::Down => {
            "INSERT INTO setting (the_one_row, download_cap_bytes_per_second) VALUES (1, ?1)
             ON CONFLICT(the_one_row) DO UPDATE
             SET download_cap_bytes_per_second = excluded.download_cap_bytes_per_second"
        }
    };
    connection.execute(statement, [bytes])?;
    Ok(())
}

/// **The user turned a switch** — T8. The row is written whichever way it was
/// turned, because turning a kind back on is the user's choice too and not a
/// reset to whatever the default happens to be.
pub fn a_switch_was_turned(
    connection: &rusqlite::Connection,
    kind: Kind,
    attempted: bool,
) -> Result<(), RecordFailure> {
    connection.execute(
        "INSERT INTO notification_switch (kind, attempted) VALUES (?1, ?2)
         ON CONFLICT(kind) DO UPDATE SET attempted = excluded.attempted",
        rusqlite::params![the_stored_name_of(kind), i64::from(attempted)],
    )?;
    Ok(())
}

/// **Whether hidden files are shown** — `File Browser` L20, version 30.
///
/// **One value for the whole app, never keyed by server row** (`State,
/// identity and recovery` I7). **No row, or a row with nothing in the column,
/// is off** — the plan's default and the drawing's unticked box. **A read that
/// does not answer is a failure and is returned as one** (`Desktop app and
/// delivery` L8a, which `File Browser` L20 binds here), never a silent *off*.
pub fn hidden_files_are_shown(connection: &rusqlite::Connection) -> Result<bool, RecordFailure> {
    let mut asking = connection.prepare("SELECT hidden_files_shown FROM setting WHERE the_one_row = 1")?;
    let mut rows = asking.query([])?;
    let stored: Option<i64> = match rows.next()? {
        Some(row) => row.get(0)?,
        None => None,
    };
    match stored {
        None => Ok(false),
        Some(1) => Ok(true),
        // **The table refuses anything else**, so a value that is not the one
        // this record writes is a read that did not answer rather than *off*.
        Some(other) => Err(RecordFailure::Engine {
            reported: format!("hidden_files_shown holds {other}, which is not a choice"),
        }),
    }
}

/// **The user ticked or unticked *Show hidden files*** — `File Browser` L20.
/// **The tick writes it on and the untick clears it** rather than writing a
/// copy of the default (`State, identity and recovery` I3 rule 4). Only the
/// user's own act on the checkbox comes through here, durably at that act
/// (`State, identity and recovery` D6).
pub fn hidden_files_were_switched(connection: &rusqlite::Connection, shown: bool) -> Result<(), RecordFailure> {
    let stored: Option<i64> = shown.then_some(1);
    connection.execute(
        "INSERT INTO setting (the_one_row, hidden_files_shown) VALUES (1, ?1)
         ON CONFLICT(the_one_row) DO UPDATE SET hidden_files_shown = excluded.hidden_files_shown",
        [stored],
    )?;
    Ok(())
}

/// **The Telegram switch and what a job reports by default** — version 20.
///
/// `Telegram` V1a makes the reporting default *"one item of the local record's
/// settings"*, user-held, and `Design/Settings § Telegram` draws the switch.
/// **No row is never switched on, with no default chosen.** The idle message's
/// switch is not here: it is the fourth notification switch above, which
/// `Telegram` V2 carries to machines.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct TheTelegramSettings {
    pub switched_on: bool,
    /// **The page's own identifier for one of the five** (three until
    /// 2026-10-31), or nothing where the user never chose. Which five is
    /// `crate::surface::render_settings`'.
    pub the_reporting_default: Option<String>,
    /// **The default's minutes, for *a picture every few minutes*** — version
    /// 56, `Telegram` V1a as amended. Nothing where none were typed or taken,
    /// which reads as 5. Kept whatever the default, so picking another row and
    /// coming back finds them.
    pub the_default_minutes: Option<u8>,
    /// **The phone's choice the default last took** (`Telegram` V1d), or
    /// nothing where none was ever taken.
    pub the_phones_choice_taken: Option<ThePhonesChoiceTaken>,
}

/// **Which phone's choice was taken, and Telegram's moment for it** — version
/// 56. `Telegram` V1d: the desktop takes *"the phone's choice with the latest
/// `date` across every reading"*, once per operation identity.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ThePhonesChoiceTaken {
    /// The operation identity, as the reading carried it.
    pub operation: String,
    /// Telegram's own `date`, whole seconds since 1970.
    pub moment: i64,
}

/// **Read them all.** A missing row is the default; a read that does not answer is
/// a failure (L8a).
pub fn the_telegram_settings(connection: &rusqlite::Connection) -> Result<TheTelegramSettings, RecordFailure> {
    let mut asking = connection.prepare(
        "SELECT switched_on, reporting_default, reporting_default_minutes, phones_choice_operation, phones_choice_moment
         FROM telegram_setting WHERE the_one_row = 1",
    )?;
    let mut rows = asking.query([])?;
    match rows.next()? {
        None => Ok(TheTelegramSettings::default()),
        Some(row) => Ok(TheTelegramSettings {
            switched_on: row.get::<_, i64>(0)? != 0,
            the_reporting_default: row.get(1)?,
            the_default_minutes: row
                .get::<_, Option<i64>>(2)?
                .and_then(|minutes| u8::try_from(minutes).ok()),
            the_phones_choice_taken: match (row.get::<_, Option<String>>(3)?, row.get::<_, Option<i64>>(4)?) {
                (Some(operation), Some(moment)) => Some(ThePhonesChoiceTaken { operation, moment }),
                _ => None,
            },
        }),
    }
}

/// **The switch, as it now stands** — written only once the act it stands for
/// is done: a bot kept, or the token erased.
pub fn telegram_was_switched(connection: &rusqlite::Connection, on: bool) -> Result<(), RecordFailure> {
    connection.execute(
        "INSERT INTO telegram_setting (the_one_row, switched_on) VALUES (1, ?1)
         ON CONFLICT(the_one_row) DO UPDATE SET switched_on = excluded.switched_on",
        [i64::from(on)],
    )?;
    Ok(())
}

/// **The user chose what a job reports by default.** The caller has already
/// held the value to the closed set. **The phone's choice taken is left as it
/// is**, so the same phone's choice read again changes nothing and this click
/// stands until a new one (`Telegram` V1d).
pub fn the_reporting_default_was_set(connection: &rusqlite::Connection, choice: &str) -> Result<(), RecordFailure> {
    connection.execute(
        "INSERT INTO telegram_setting (the_one_row, reporting_default) VALUES (1, ?1)
         ON CONFLICT(the_one_row) DO UPDATE SET reporting_default = excluded.reporting_default",
        [choice],
    )?;
    Ok(())
}

/// **The user typed the default's minutes, and so chose that row** — version
/// 56. The caller has already held the minutes to 1 to 60 and the identifier
/// to the closed set; the two are written in one statement.
pub fn the_default_minutes_were_set(
    connection: &rusqlite::Connection,
    the_minutes_choice: &str,
    minutes: u8,
) -> Result<(), RecordFailure> {
    connection.execute(
        "INSERT INTO telegram_setting (the_one_row, reporting_default, reporting_default_minutes) VALUES (1, ?1, ?2)
         ON CONFLICT(the_one_row) DO UPDATE SET reporting_default = excluded.reporting_default,
             reporting_default_minutes = excluded.reporting_default_minutes",
        rusqlite::params![the_minutes_choice, i64::from(minutes)],
    )?;
    Ok(())
}

/// **A phone's choice read from a server, taken as the default where it is
/// newer than the one last taken** — `Telegram` V1d. Taken where none was
/// taken before, or where its operation is another and its Telegram `date` is
/// later; otherwise nothing changes, so a choice already taken never undoes a
/// later click at the desk, and a server that heard an older choice never
/// replaces a newer one. **One statement**, so the default, its minutes and
/// the choice taken never disagree. `minutes` is written only for the
/// minutes row and otherwise left as it stood. Says whether it was taken.
pub fn the_phones_choice_is_taken_where_newer(
    connection: &rusqlite::Connection,
    choice: &str,
    minutes: Option<u8>,
    operation: &str,
    moment: i64,
) -> Result<bool, RecordFailure> {
    let changed = connection.execute(
        "INSERT INTO telegram_setting (the_one_row, reporting_default, reporting_default_minutes,
             phones_choice_operation, phones_choice_moment)
         VALUES (1, ?1, ?2, ?3, ?4)
         ON CONFLICT(the_one_row) DO UPDATE SET
             reporting_default = excluded.reporting_default,
             reporting_default_minutes = coalesce(excluded.reporting_default_minutes, reporting_default_minutes),
             phones_choice_operation = excluded.phones_choice_operation,
             phones_choice_moment = excluded.phones_choice_moment
         WHERE phones_choice_operation IS NULL
            OR phones_choice_moment IS NULL
            OR (phones_choice_operation <> excluded.phones_choice_operation
                AND phones_choice_moment < excluded.phones_choice_moment)",
        rusqlite::params![choice, minutes.map(i64::from), operation, moment],
    )?;
    Ok(changed > 0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::record::LocalRecord;

    /// **Version 20: never switched on and no default, then each kept without
    /// moving the other or the seven settings above.**
    #[test]
    fn the_telegram_switch_and_the_reporting_default_are_kept_apart() {
        let (folder, record) = a_record("telegram");
        let c = record.connection();
        assert_eq!(the_telegram_settings(c).unwrap(), TheTelegramSettings::default());
        the_reporting_default_was_set(c, "pictures-only").unwrap();
        assert!(!the_telegram_settings(c).unwrap().switched_on, "choosing a default switched Telegram on");
        telegram_was_switched(c, true).unwrap();
        a_switch_was_turned(c, Kind::AMachineHasBeenIdle, false).unwrap();
        let held = the_telegram_settings(c).unwrap();
        assert!(held.switched_on);
        assert_eq!(held.the_reporting_default.as_deref(), Some("pictures-only"));
        telegram_was_switched(c, false).unwrap();
        let held = the_telegram_settings(c).unwrap();
        assert!(!held.switched_on);
        assert_eq!(held.the_reporting_default.as_deref(), Some("pictures-only"), "switching off forgot the default");
        assert!(!read(c).unwrap().the_switches.attempts(Kind::AMachineHasBeenIdle));
        drop(record);
        done(folder);
    }

    /// **Version 56, `Telegram` V1d: a phone's choice is taken once, and only
    /// where it is newer**; a later click at the desk stands until a new one;
    /// the minutes typed are kept with the row they belong to.
    #[test]
    fn a_phones_choice_is_taken_once_and_only_where_newer_and_a_later_click_stands() {
        let (folder, record) = a_record("telegram-phone");
        let c = record.connection();
        assert_eq!(the_telegram_settings(c).unwrap().the_default_minutes, None);
        the_default_minutes_were_set(c, "a-picture-every-few-minutes", 12).unwrap();
        let held = the_telegram_settings(c).unwrap();
        assert_eq!(held.the_reporting_default.as_deref(), Some("a-picture-every-few-minutes"));
        assert_eq!(held.the_default_minutes, Some(12));
        assert_eq!(held.the_phones_choice_taken, None);

        // The first phone's choice is taken; its minutes leave the typed ones be
        // where it carries none.
        assert!(the_phones_choice_is_taken_where_newer(c, "only-what-needs-you", None, "op-1", 1_000).unwrap());
        let held = the_telegram_settings(c).unwrap();
        assert_eq!(held.the_reporting_default.as_deref(), Some("only-what-needs-you"));
        assert_eq!(held.the_default_minutes, Some(12));
        assert_eq!(
            held.the_phones_choice_taken,
            Some(ThePhonesChoiceTaken { operation: "op-1".into(), moment: 1_000 })
        );

        // A click at the desk afterwards; the same choice read again from
        // another server changes nothing.
        the_reporting_default_was_set(c, "everything").unwrap();
        assert!(!the_phones_choice_is_taken_where_newer(c, "only-what-needs-you", None, "op-1", 1_000).unwrap());
        assert_eq!(the_telegram_settings(c).unwrap().the_reporting_default.as_deref(), Some("everything"));

        // An older choice another server still holds is not taken either.
        assert!(!the_phones_choice_is_taken_where_newer(c, "nothing", None, "op-0", 900).unwrap());
        assert_eq!(the_telegram_settings(c).unwrap().the_reporting_default.as_deref(), Some("everything"));

        // A new, later one is, with its minutes.
        assert!(the_phones_choice_is_taken_where_newer(c, "a-picture-every-few-minutes", Some(10), "op-2", 1_060).unwrap());
        let held = the_telegram_settings(c).unwrap();
        assert_eq!(held.the_reporting_default.as_deref(), Some("a-picture-every-few-minutes"));
        assert_eq!(held.the_default_minutes, Some(10));
        assert_eq!(held.the_phones_choice_taken.map(|taken| taken.operation), Some("op-2".to_string()));
        assert!(!held.switched_on, "taking a phone's choice switched Telegram on");
        drop(record);
        done(folder);
    }

    /// **L8a: a table that will not read is a failure, never *switched off*.**
    #[test]
    fn a_telegram_setting_that_does_not_read_is_not_switched_off() {
        let (folder, record) = a_record("telegram-unread");
        record.connection().execute_batch("DROP TABLE telegram_setting").unwrap();
        assert!(the_telegram_settings(record.connection()).is_err());
        drop(record);
        done(folder);
    }

    /// **`File Browser` L20: absent is off, the tick writes it, the untick
    /// clears it, and neither moves another setting.**
    #[test]
    fn the_hidden_files_choice_is_off_until_ticked_and_the_untick_clears_it() {
        let (folder, record) = a_record("hidden-files");
        let c = record.connection();
        assert!(!hidden_files_are_shown(c).unwrap(), "a fresh record shows hidden files");
        the_download_folder_was_chosen(c, b"/Volumes/Work/Renders").unwrap();
        assert!(!hidden_files_are_shown(c).unwrap(), "a settings row with nothing in the column is not off");
        hidden_files_were_switched(c, true).unwrap();
        assert!(hidden_files_are_shown(c).unwrap());
        assert_eq!(read(c).unwrap().the_chosen_download_folder.as_deref(), Some(&b"/Volumes/Work/Renders"[..]));
        hidden_files_were_switched(c, false).unwrap();
        assert!(!hidden_files_are_shown(c).unwrap());
        let stored: Option<i64> = c
            .query_row("SELECT hidden_files_shown FROM setting", [], |row| row.get(0))
            .unwrap();
        assert_eq!(stored, None, "the untick wrote a copy of the default rather than clearing it");
        assert!(read(c).unwrap().the_chosen_download_folder.is_some(), "the untick moved the folder");
        drop(record);
        done(folder);
    }

    /// **The choice survives the record being closed and opened again** — the
    /// app restarting — **and is one value, whatever server is being browsed**:
    /// the column is on the one-row table and nothing keys it.
    #[test]
    fn the_hidden_files_choice_survives_a_restart() {
        let (folder, record) = a_record("hidden-files-restart");
        hidden_files_were_switched(record.connection(), true).unwrap();
        drop(record);
        let again = LocalRecord::open_at(&folder.join("local-record.sqlite3")).expect("the record opens again");
        assert!(hidden_files_are_shown(again.connection()).unwrap());
        let rows: i64 = again
            .connection()
            .query_row("SELECT count(*) FROM setting", [], |row| row.get(0))
            .unwrap();
        assert_eq!(rows, 1, "the choice is not one value for the app");
        drop(again);
        done(folder);
    }

    /// **L8a: a column that will not read is a failure, never *off*; and the
    /// table refuses a value that is not the choice.**
    #[test]
    fn a_hidden_files_choice_that_does_not_read_is_not_off() {
        let (folder, record) = a_record("hidden-files-unread");
        let c = record.connection();
        assert!(c
            .execute("INSERT INTO setting (the_one_row, hidden_files_shown) VALUES (1, 0)", [])
            .is_err());
        assert!(c
            .execute("INSERT INTO setting (the_one_row, hidden_files_shown) VALUES (1, 2)", [])
            .is_err());
        c.execute_batch("DROP TABLE setting").unwrap();
        assert!(hidden_files_are_shown(c).is_err());
        drop(record);
        done(folder);
    }

    fn a_record(name: &str) -> (std::path::PathBuf, LocalRecord) {
        let mut folder = std::env::temp_dir();
        folder.push(format!(
            "smoozy-settings-store-test-{name}-{}",
            std::process::id()
        ));
        let _ = std::fs::remove_dir_all(&folder);
        std::fs::create_dir_all(&folder).expect("a folder to put the record in");
        let record =
            LocalRecord::open_at(&folder.join("local-record.sqlite3")).expect("the record opens");
        (folder, record)
    }

    fn done(folder: std::path::PathBuf) {
        let _ = std::fs::remove_dir_all(folder);
    }

    /// **A record in which the user has set nothing reads as every default**,
    /// and not as trouble.
    #[test]
    fn a_record_with_nothing_set_reads_as_the_defaults() {
        let (folder, record) = a_record("nothing-set");
        let settings = read(record.connection()).expect("the read answers");
        assert_eq!(settings.the_chosen_download_folder, None);
        assert_eq!(settings.the_upload_cap, TheCap::NoCapIsSet);
        assert_eq!(settings.the_download_cap, TheCap::NoCapIsSet);
        for kind in Kind::ALL {
            assert_eq!(
                settings.the_switches.attempts(kind),
                A_KIND_NOBODY_HAS_TURNED_IS_ATTEMPTED
            );
        }
        drop(record);
        done(folder);
    }

    /// **Each of the seven is written and read back, and none disturbs
    /// another.**
    #[test]
    fn each_setting_is_kept_and_none_disturbs_another() {
        let (folder, record) = a_record("each");
        let c = record.connection();
        a_cap_was_set(c, Direction::Up, TheCap::Of(BytesPerSecond(12_500_000))).unwrap();
        the_download_folder_was_chosen(c, b"/Volumes/Work/Renders").unwrap();
        a_switch_was_turned(c, Kind::AMachineHasBeenIdle, false).unwrap();
        a_cap_was_set(c, Direction::Down, TheCap::Of(BytesPerSecond(40_000_000))).unwrap();

        let settings = read(c).unwrap();
        assert_eq!(
            settings.the_chosen_download_folder.as_deref(),
            Some(&b"/Volumes/Work/Renders"[..])
        );
        assert_eq!(settings.the_upload_cap, TheCap::Of(BytesPerSecond(12_500_000)));
        assert_eq!(settings.the_download_cap, TheCap::Of(BytesPerSecond(40_000_000)));
        assert!(!settings.the_switches.attempts(Kind::AMachineHasBeenIdle));
        assert!(settings.the_switches.attempts(Kind::AJobFinished));

        // **R8: clearing a cap stores nothing**, and the other direction's cap
        // and the folder are untouched by it.
        a_cap_was_set(c, Direction::Up, TheCap::NoCapIsSet).unwrap();
        let settings = read(c).unwrap();
        assert_eq!(settings.the_upload_cap, TheCap::NoCapIsSet);
        assert_eq!(settings.the_download_cap, TheCap::Of(BytesPerSecond(40_000_000)));
        assert!(settings.the_chosen_download_folder.is_some());
        let stored: Option<i64> = c
            .query_row("SELECT upload_cap_bytes_per_second FROM setting", [], |row| row.get(0))
            .unwrap();
        assert_eq!(stored, None, "no cap was written as a number");

        drop(record);
        done(folder);
    }

    /// **A kind turned back on is written as on**, not removed — so it is the
    /// user's choice rather than whatever the default later becomes.
    #[test]
    fn a_kind_turned_back_on_is_the_users_choice_and_not_a_reset() {
        let (folder, record) = a_record("back-on");
        let c = record.connection();
        a_switch_was_turned(c, Kind::AJobFailedOrWasGivenUp, false).unwrap();
        a_switch_was_turned(c, Kind::AJobFailedOrWasGivenUp, true).unwrap();
        let rows: i64 = c
            .query_row("SELECT count(*) FROM notification_switch", [], |row| row.get(0))
            .unwrap();
        assert_eq!(rows, 1);
        assert!(read(c).unwrap().the_switches.attempts(Kind::AJobFailedOrWasGivenUp));
        drop(record);
        done(folder);
    }

    /// **L8a: a table that will not read is a failure, never the defaults.**
    #[test]
    fn a_read_that_does_not_answer_is_not_turned_into_the_defaults() {
        let (folder, record) = a_record("unread");
        record
            .connection()
            .execute_batch("DROP TABLE notification_switch")
            .unwrap();
        assert!(read(record.connection()).is_err());
        drop(record);
        done(folder);
    }

    /// **The table refuses a cap of nought**, so a zero can never stop every
    /// transfer by arriving through some other door.
    #[test]
    fn the_table_refuses_a_cap_of_nought() {
        let (folder, record) = a_record("nought");
        assert!(record
            .connection()
            .execute(
                "INSERT INTO setting (the_one_row, upload_cap_bytes_per_second) VALUES (1, 0)",
                []
            )
            .is_err());
        drop(record);
        done(folder);
    }

    /// **The four stored names are `Kind::ALL`'s, one each, and they are the
    /// names the page hands back** — one closed set, spelled once.
    #[test]
    fn every_kind_has_one_stored_name_and_the_page_uses_it() {
        let page = std::fs::read_to_string(
            std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("web/settings.js"),
        )
        .expect("the page is readable");
        let mut seen = std::collections::BTreeSet::new();
        for kind in Kind::ALL {
            let name = the_stored_name_of(kind);
            assert!(seen.insert(name), "{name} is used for two kinds");
            assert_eq!(the_kind_stored_as(name), Some(kind));
            assert!(page.contains(&format!("\"{name}\"")), "the page never names {name}");
        }
        assert_eq!(the_kind_stored_as("a-fifth-kind"), None);
    }

    /// **W3: the system's proposal is never written into the record.** Only
    /// [`the_download_folder_was_chosen`] writes the folder, and nothing in the
    /// crate's working code reaches the proposal and this table together.
    #[test]
    fn nothing_but_the_users_own_choice_writes_the_folder() {
        let mut writers = Vec::new();
        walk(
            &std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src"),
            &mut writers,
        );
        assert_eq!(writers, vec!["settings.rs".to_string()]);
        let surface = include_str!("../surface/settings.rs")
            .split("#[cfg(test)]")
            .next()
            .unwrap()
            .to_string();
        let chosen_at = surface
            .find("the_download_folder_was_chosen(")
            .expect("the screen writes the user's choice");
        let the_writing_function = &surface[surface[..chosen_at].rfind("fn ").unwrap()..chosen_at];
        assert!(
            !the_writing_function.contains("the_folder_the_system_proposes"),
            "the function that writes the folder reads the proposal. **W3: nothing caches it**"
        );
    }

    fn walk(folder: &std::path::Path, found: &mut Vec<String>) {
        let Ok(entries) = std::fs::read_dir(folder) else {
            return;
        };
        let mut paths: Vec<_> = entries.map(|entry| entry.unwrap().path()).collect();
        paths.sort();
        for path in paths {
            if path.is_dir() {
                walk(&path, found);
                continue;
            }
            if path.extension().and_then(|e| e.to_str()) != Some("rs") {
                continue;
            }
            let text = std::fs::read_to_string(&path).unwrap();
            let working = text.split("#[cfg(test)]").next().unwrap_or_default();
            if working.contains("SET download_folder") || working.contains("INTO setting (the_one_row, download_folder") {
                found.push(path.file_name().unwrap().to_str().unwrap().to_string());
            }
        }
    }
}
