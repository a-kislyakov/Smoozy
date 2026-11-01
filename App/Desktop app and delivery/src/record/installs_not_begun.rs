//! **The installs still waiting behind a chain of Blender installs, in the
//! local record** — the table record version 43 made, step 10c's owed piece 4
//! (2026-10-16).
//!
//! **The rule is `Server Setup` Y9a's and the store is this document's**, the
//! split every row module here draws. Several Blenders picked at once install
//! one after another, and the desktop begins each when the last one's run
//! ends (`Server Setup` Row 526). **What is kept is the versions picked and
//! not yet begun, in the list's order**: written at the press, written again
//! each time the next one begins, and emptied when the chain ends while the
//! desktop is there to see it. So a list still standing when Smoozy opens is
//! one an earlier run left when it closed first, and the Set up card names it.
//!
//! **From record version 54 the list is of acts, not only installs**
//! (`Server Setup` Y9a item 10, the nineteenth notes, note 2, 2026-10-27):
//! anything pressed on the Set up card while a setup runs waits its turn, and
//! each row says which act it is and what it names. The installs-only reading
//! and writing below stay for the callers that keep only installs.
//!
//! **A field of the server row** (`Servers` D3's row of 2026-10-16), kept in a
//! table of its own as the row's other lists are, and taken by the row's
//! removal ([`crate::record::server_rows::remove_walking_the_enumeration`]).
//! **It is the user's own ask and nothing read from a machine**, so `Server
//! Setup` B12's rule against remembering what a machine holds does not reach
//! it.

use super::RecordFailure;

/// The one table, named once.
pub const TABLE: &str = "server_install_not_begun";

/// **Which act is kept** — `Server Setup` Y9a item 10, the card acts that may
/// wait their turn. Added 2026-10-27, record version 54.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TheActsKind {
    /// A Blender from the list, named by the version as the list asked for it.
    /// **Every row before version 54 is one.**
    AnInstallFromTheList,
    /// The user's own build (`Server Setup` B7), named by the file picked.
    TheUsersBuild,
    /// A removal (`Server Setup` B15), named by the build's answered version,
    /// or its path where the table read none.
    ARemoval,
    /// The ready action — *Set up again*. Names nothing.
    TheReadyAction,
    /// The Telegram placement (`Server Setup` G2). Names nothing.
    TheTelegramPlacement,
}

impl TheActsKind {
    /// **The one written form**, in the `act` column.
    pub fn written(self) -> &'static str {
        match self {
            TheActsKind::AnInstallFromTheList => "install",
            TheActsKind::TheUsersBuild => "build",
            TheActsKind::ARemoval => "remove",
            TheActsKind::TheReadyAction => "setup",
            TheActsKind::TheTelegramPlacement => "telegram",
        }
    }

    /// Read back; nothing for a form this binary never writes.
    pub fn read(written: &str) -> Option<Self> {
        [
            TheActsKind::AnInstallFromTheList,
            TheActsKind::TheUsersBuild,
            TheActsKind::ARemoval,
            TheActsKind::TheReadyAction,
            TheActsKind::TheTelegramPlacement,
        ]
        .into_iter()
        .find(|kind| kind.written() == written)
    }
}

/// **One act kept**: which it is and what it names.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AnActKept {
    pub kind: TheActsKind,
    pub named: String,
}

impl AnActKept {
    /// A Blender install from the list, by the version as the list asked for it.
    pub fn an_install(version: &str) -> Self {
        AnActKept { kind: TheActsKind::AnInstallFromTheList, named: version.to_string() }
    }
}

/// **The acts still waiting on that server**, in the order they were asked
/// for. Empty where none are. **A row in a form this binary never writes is
/// left out** rather than guessed at.
pub fn the_acts_waiting_on(
    connection: &rusqlite::Connection,
    server: &str,
) -> Result<Vec<AnActKept>, RecordFailure> {
    let mut asking = connection
        .prepare("SELECT act, version FROM server_install_not_begun WHERE server = ?1 ORDER BY position")?;
    let rows = asking.query_map([server], |row| Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?)))?;
    let mut out = Vec::new();
    for row in rows {
        let (act, named) = row?;
        if let Some(kind) = TheActsKind::read(&act) {
            out.push(AnActKept { kind, named });
        }
    }
    Ok(out)
}

/// **The acts still waiting on that server are kept**, replacing whatever
/// stood — an empty list empties it. **One write, all of it or nothing**: a
/// write that is refused leaves what stood before.
pub fn the_acts_waiting_are_kept(
    connection: &rusqlite::Connection,
    server: &str,
    waiting: &[AnActKept],
) -> Result<(), RecordFailure> {
    connection.execute_batch("SAVEPOINT the_installs_not_begun")?;
    let attempt = (|| -> Result<(), RecordFailure> {
        connection.execute("DELETE FROM server_install_not_begun WHERE server = ?1", [server])?;
        for (position, act) in waiting.iter().enumerate() {
            connection.execute(
                "INSERT INTO server_install_not_begun (server, position, version, act) VALUES (?1, ?2, ?3, ?4)",
                rusqlite::params![server, position as i64, act.named, act.kind.written()],
            )?;
        }
        Ok(())
    })();
    match attempt {
        Ok(()) => {
            connection.execute_batch("RELEASE the_installs_not_begun")?;
            Ok(())
        }
        Err(failure) => {
            let _ = connection
                .execute_batch("ROLLBACK TO the_installs_not_begun; RELEASE the_installs_not_begun");
            Err(failure)
        }
    }
}

/// **The versions of the Blender installs from the list still waiting on that
/// server**, in the order the list showed them — the installs-only reading of
/// [`the_acts_waiting_on`]. Empty where none are.
pub fn the_ones_waiting_on(
    connection: &rusqlite::Connection,
    server: &str,
) -> Result<Vec<String>, RecordFailure> {
    Ok(the_acts_waiting_on(connection, server)?
        .into_iter()
        .filter(|act| act.kind == TheActsKind::AnInstallFromTheList)
        .map(|act| act.named)
        .collect())
}

/// **The versions still waiting on that server are kept, each a Blender install
/// from the list**, replacing whatever stood — the installs-only writing of
/// [`the_acts_waiting_are_kept`].
pub fn the_ones_waiting_are_kept(
    connection: &rusqlite::Connection,
    server: &str,
    waiting: &[String],
) -> Result<(), RecordFailure> {
    let acts = waiting.iter().map(|version| AnActKept::an_install(version)).collect::<Vec<_>>();
    the_acts_waiting_are_kept(connection, server, &acts)
}

/// **The server row is removed, and its list with it** — called from inside
/// the removal walk's own transaction.
pub fn forget(connection: &rusqlite::Connection, server: &str) -> Result<(), RecordFailure> {
    connection.execute("DELETE FROM server_install_not_begun WHERE server = ?1", [server])?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::record::LocalRecord;

    fn scratch(name: &str) -> std::path::PathBuf {
        let mut path = std::env::temp_dir();
        path.push(format!("smoozy-installs-not-begun-{name}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&path);
        std::fs::create_dir_all(&path).unwrap();
        path.join("local-record.sqlite3")
    }

    /// **Kept in the list's order, replaced whole, emptied by an empty list,
    /// and one server's list never reaches another's.**
    #[test]
    fn the_list_is_kept_in_order_replaced_whole_and_emptied() {
        let path = scratch("kept");
        let record = LocalRecord::open_at(&path).expect("a fresh record");
        let c = record.connection();
        let versions = |all: &[&str]| all.iter().map(|one| one.to_string()).collect::<Vec<_>>();
        assert!(the_ones_waiting_on(c, "a").unwrap().is_empty());
        the_ones_waiting_are_kept(c, "a", &versions(&["5.2.2", "4.5.14"])).unwrap();
        the_ones_waiting_are_kept(c, "b", &versions(&["4.2.9"])).unwrap();
        assert_eq!(the_ones_waiting_on(c, "a").unwrap(), versions(&["5.2.2", "4.5.14"]));
        the_ones_waiting_are_kept(c, "a", &versions(&["4.5.14"])).unwrap();
        assert_eq!(the_ones_waiting_on(c, "a").unwrap(), versions(&["4.5.14"]));
        the_ones_waiting_are_kept(c, "a", &[]).unwrap();
        assert!(the_ones_waiting_on(c, "a").unwrap().is_empty());
        assert_eq!(the_ones_waiting_on(c, "b").unwrap(), versions(&["4.2.9"]));
        forget(c, "b").unwrap();
        assert!(the_ones_waiting_on(c, "b").unwrap().is_empty());
        drop(record);
        let _ = std::fs::remove_dir_all(path.parent().unwrap());
    }

    /// **`Server Setup` Y9a item 10: acts of every kind are kept in order and
    /// read back as they were written**, and the installs-only reading names
    /// only the installs among them. The nineteenth notes, note 2.
    #[test]
    fn acts_of_every_kind_are_kept_in_order_and_read_back() {
        let path = scratch("acts");
        let record = LocalRecord::open_at(&path).expect("a fresh record");
        let c = record.connection();
        let acts = vec![
            AnActKept { kind: TheActsKind::ARemoval, named: "5.2.2".to_string() },
            AnActKept::an_install("4.5.15"),
            AnActKept { kind: TheActsKind::TheUsersBuild, named: "blender-4.2-linux.tar.xz".to_string() },
            AnActKept { kind: TheActsKind::TheReadyAction, named: String::new() },
            AnActKept { kind: TheActsKind::TheTelegramPlacement, named: String::new() },
        ];
        the_acts_waiting_are_kept(c, "a", &acts).unwrap();
        assert_eq!(the_acts_waiting_on(c, "a").unwrap(), acts);
        assert_eq!(the_ones_waiting_on(c, "a").unwrap(), ["4.5.15"]);
        for kind in acts.iter().map(|act| act.kind) {
            assert_eq!(TheActsKind::read(kind.written()), Some(kind));
        }
        assert_eq!(TheActsKind::read("something else"), None);
        the_acts_waiting_are_kept(c, "a", &[]).unwrap();
        assert!(the_acts_waiting_on(c, "a").unwrap().is_empty());
        drop(record);
        let _ = std::fs::remove_dir_all(path.parent().unwrap());
    }
}
