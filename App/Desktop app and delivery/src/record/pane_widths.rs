//! The remembered pane widths in the local record — `Desktop app and delivery`
//! §3.3 P11 and `State, identity and recovery` §1.1 I3.
//!
//! **The rule is P11's and I3's; the store is this document's**, which is the
//! split every other row module here draws. **A2 holds: no module has a durable
//! store of its own.**
//!
//! **Four rules land on this file and each is a line of code rather than a
//! habit.**
//!
//! 1. **One value per seam, and the set is closed** (`State, identity and
//!    recovery` I3 rule 1). The set is [`crate::surface::panes::THE_SEAMS`] and
//!    the key is a seam's own name; the primary key is what makes *one value*
//!    true in the store as well as in the caller.
//! 2. **The user's own drag is the only writer** (P11). [`a_drag_ended`] is the
//!    one function here that writes a width, and its name is what it is so that
//!    a second caller reads wrongly. **The narrowing steps write nothing** —
//!    each step's re-declared width is a drawing-time computation over the
//!    stored value and the window's width, which is what makes *set aside, not
//!    destroyed* true by construction.
//! 3. **The reset clears the value rather than writing a copy of the drawn
//!    default into it** (I3 rule 4), so *default* has one representation and a
//!    default Design later changes reaches every user who never dragged.
//! 4. **An absent value is the ordinary state and never a fault** (P11, I3
//!    rule 4). [`read_all`] answers with the seams it holds and says nothing
//!    about the ones it does not; L8a is untouched — an absent setting is an
//!    ordinary state, a read that did not answer is not.
//!
//! **How a clause marker is written in this crate, and it is a rule rather than
//! a habit.** A marker with no document beside it means a clause of this crate's
//! own plan, `Preparation/Architecture/Desktop app and delivery.md`. Every other
//! document's marker is written with that document — at the site, or in this
//! file's own header list below.
//!
//! **`State, identity and recovery`'s own markers are written out with that
//! document at every site below and are deliberately not listed here.** This
//! plan defines clauses of the same short names, and a header list cannot tell
//! the two apart — a bare one would resolve to a real and wrong clause.
//! **`Desktop app and delivery` L1 and P11 are this crate's own and are written
//! bare.** So is L8a.

use super::RecordFailure;

/// The one table. Named here so the guard test below and the schema read from
/// one place.
pub const TABLE: &str = "pane_width";

/// **Every width the record holds, seam by seam.**
///
/// **A seam the record does not name is not in the answer**, and that is
/// `State, identity and recovery` I3 rule 4's ordinary state rather than a gap: it means this user has not
/// dragged that seam and the width is the one the drawing declares.
pub fn read_all(connection: &rusqlite::Connection) -> Result<Vec<(String, u32)>, RecordFailure> {
    let mut asking = connection.prepare("SELECT seam, width FROM pane_width ORDER BY seam")?;
    let rows = asking.query_map([], |row| {
        Ok((row.get::<_, String>(0)?, row.get::<_, i64>(1)?))
    })?;
    let mut out = Vec::new();
    for row in rows {
        let (seam, width) = row?;
        // A width that will not fit a pixel count is not a width. It reads as
        // no value, which is the drawn default — the same answer
        // `State, identity and recovery` I3 rule 4 gives for a seam nobody has
        // dragged.
        if let Ok(width) = u32::try_from(width) {
            out.push((seam, width));
        }
    }
    Ok(out)
}

/// One seam's width, or nothing.
pub fn read(connection: &rusqlite::Connection, seam: &str) -> Result<Option<u32>, RecordFailure> {
    Ok(read_all(connection)?
        .into_iter()
        .find(|(named, _)| named == seam)
        .map(|(_, width)| width))
}

/// **The drag ended, and that is the moment the fact becomes true** — P11 and
/// `State, identity and recovery` I3 rule 3. That rule is
/// `State, identity and recovery` D6 applied: the release is the fact, so a
/// crash costs no width the user had already set. **Nothing is written on exit
/// and nothing is written while the pointer is moving**; the cost is one flush
/// per completed drag, which is the user's own rate.
///
/// **The one writer.** Nothing else in this crate writes this table, and a test
/// in this file says so rather than leaving it to a habit.
pub fn a_drag_ended(
    connection: &rusqlite::Connection,
    seam: &str,
    width: u32,
) -> Result<(), RecordFailure> {
    connection.execute(
        "INSERT INTO pane_width (seam, width) VALUES (?1, ?2)
         ON CONFLICT(seam) DO UPDATE SET width = excluded.width",
        rusqlite::params![seam, i64::from(width)],
    )?;
    Ok(())
}

/// **The seam was put back where it started** — Design's *"double-clicking a
/// seam puts it back where it started… it is the only reset there is."*
///
/// **It clears the value and never writes the drawn default into it**
/// (`State, identity and recovery` I3 rule 4). Writing the default would give
/// *default* two representations, and a default Design later changed would then
/// reach every user except the ones who had once pressed reset.
pub fn the_seam_was_reset(
    connection: &rusqlite::Connection,
    seam: &str,
) -> Result<(), RecordFailure> {
    connection.execute("DELETE FROM pane_width WHERE seam = ?1", [seam])?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::record::LocalRecord;

    /// A record of its own, in a folder of its own, the way every other test
    /// in this module makes one.
    fn a_record(name: &str) -> (std::path::PathBuf, LocalRecord) {
        let mut folder = std::env::temp_dir();
        folder.push(format!(
            "smoozy-pane-width-test-{name}-{}",
            std::process::id()
        ));
        let _ = std::fs::remove_dir_all(&folder);
        std::fs::create_dir_all(&folder).expect("a folder to put the record in");
        let record =
            LocalRecord::open_at(&folder.join("local-record.sqlite3")).expect("the record opens");
        (folder, record)
    }

    /// Take the folder away again.
    fn done(folder: std::path::PathBuf) {
        let _ = std::fs::remove_dir_all(folder);
    }

    /// **A drag writes the width, and reading it back gives the same number.**
    #[test]
    fn a_drag_is_remembered() {
        let (folder, record) = a_record("remembered");
        a_drag_ended(record.connection(), "rail", 240).expect("the width goes down");
        assert_eq!(
            read(record.connection(), "rail").expect("the read answers"),
            Some(240)
        );
        drop(record);
        done(folder);
    }

    /// **One stored value per key, never one per drawing** — P11.
    ///
    /// A second drag on the same seam replaces the first rather than adding a
    /// row beside it, which is what *one value* means in the store.
    #[test]
    fn a_seam_holds_one_width_and_the_second_drag_replaces_the_first() {
        let (folder, record) = a_record("one-value");
        a_drag_ended(record.connection(), "rail", 240).expect("the first goes down");
        a_drag_ended(record.connection(), "rail", 300).expect("the second goes down");
        assert_eq!(
            read_all(record.connection()).expect("the read answers"),
            vec![("rail".to_string(), 300)]
        );
        drop(record);
        done(folder);
    }

    /// **A record holding no width is the drawn default and not a failure** —
    /// P11, and `State, identity and recovery` I3 rule 4.
    ///
    /// A fresh installation has dragged nothing, so the read has to answer
    /// *nothing* rather than fail; L8a's line is that an absent setting is an
    /// ordinary state and a read that did not answer is not.
    #[test]
    fn a_seam_nobody_has_dragged_reads_as_nothing_and_not_as_trouble() {
        let (folder, record) = a_record("absent");
        assert_eq!(
            read(record.connection(), "rail").expect("the read answers"),
            None
        );
        assert!(read_all(record.connection())
            .expect("the read answers")
            .is_empty());
        drop(record);
        done(folder);
    }

    /// **The reset clears the value rather than writing a default into it** —
    /// `State, identity and recovery` I3 rule 4.
    ///
    /// **The assertion that matters is the second one**: after a reset the seam
    /// is absent, not present holding the drawn number. A reset that stored the
    /// default would look identical on screen today and would freeze that
    /// user's width at whatever the default was on the day they pressed it.
    #[test]
    fn the_reset_clears_the_width_rather_than_storing_the_default() {
        let (folder, record) = a_record("reset");
        a_drag_ended(record.connection(), "rail", 240).expect("the width goes down");
        the_seam_was_reset(record.connection(), "rail").expect("the reset goes down");
        assert_eq!(
            read(record.connection(), "rail").expect("the read answers"),
            None,
            "the reset left a value behind. **`State, identity and recovery` \
             I3 rule 4: the reset clears the \
             value**, so *default* has one representation instead of two"
        );
        drop(record);
        done(folder);
    }

    /// **Seams do not read each other's widths.**
    #[test]
    fn each_seam_holds_its_own() {
        let (folder, record) = a_record("its-own");
        a_drag_ended(record.connection(), "rail", 240).expect("the rail goes down");
        a_drag_ended(record.connection(), "list", 420).expect("the list goes down");
        the_seam_was_reset(record.connection(), "rail").expect("the reset goes down");
        assert_eq!(
            read_all(record.connection()).expect("the read answers"),
            vec![("list".to_string(), 420)]
        );
        drop(record);
        done(folder);
    }

    /// **The user's own drag is the only writer** — P11, held over the crate
    /// rather than over this file.
    ///
    /// **This is the check with teeth and it is why the table's name is a
    /// constant.** P11's own sentence is that the narrowing steps write
    /// nothing: *"a width the narrowing wrote back would be the failure the
    /// design's own sentence warns against, and it cannot occur because the
    /// steps have no write."* A later piece that reached for this table from a
    /// layout path would break that with nothing on screen showing it — the
    /// width would simply stop coming back when the window widened again.
    #[test]
    fn nothing_else_in_this_crate_writes_a_pane_width() {
        let mut writers = Vec::new();
        walk(
            &std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src"),
            &mut writers,
        );
        assert_eq!(
            writers,
            vec!["pane_widths.rs".to_string()],
            "something besides the drag writes a pane width. **P11: a dragged \
             pane width is durable state, written at the moment the drag ends, \
             and the user's own drag is its only writer.** **The narrowing \
             steps write nothing** — each step's re-declared width is a \
             drawing-time computation, which is what makes *set aside, not \
             destroyed* true by construction"
        );
    }

    /// Every `.rs` file under `src` whose working half writes the table.
    fn walk(folder: &std::path::Path, found: &mut Vec<String>) {
        let Ok(entries) = std::fs::read_dir(folder) else {
            return;
        };
        let mut paths: Vec<_> = entries
            .map(|entry| entry.expect("the entry is readable").path())
            .collect();
        paths.sort();
        for path in paths {
            if path.is_dir() {
                walk(&path, found);
                continue;
            }
            if path.extension().and_then(|e| e.to_str()) != Some("rs") {
                continue;
            }
            let text = std::fs::read_to_string(&path).expect("the file is readable");
            let working = text.split("#[cfg(test)]").next().unwrap_or_default();
            let writes = [
                "INSERT INTO pane_width",
                "UPDATE pane_width",
                "DELETE FROM pane_width",
            ]
            .iter()
            .any(|statement| working.contains(statement));
            if writes {
                found.push(
                    path.file_name()
                        .and_then(|s| s.to_str())
                        .expect("the file has a name")
                        .to_string(),
                );
            }
        }
    }

    /// The table's name is the one the schema makes.
    #[test]
    fn the_table_is_the_one_the_schema_makes() {
        assert!(
            crate::record::schema::VERSION_13
                .iter()
                .any(|statement| statement.contains(&format!("CREATE TABLE {TABLE}"))),
            "the migration no longer makes the table this file writes"
        );
    }
}
