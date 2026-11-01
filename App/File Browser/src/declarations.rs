//! Declarations — `File Browser` §11.
//!
//! **Per `State, identity and recovery` §9.2 — each action's partition unit,
//! and the test that decided it. Nothing below is decided at run time.**
//!
//! **Markers used below that belong to another document, each with the document
//! it belongs to.** `State, identity and recovery` §9.2, §9.3, §9.4, §11.1.
//! `Desktop app and delivery` §15, G4, G5, A7. `Server protocol and transfers`
//! T8.
//!
//! **A marker this plan also defines is deliberately not listed above,
//! whichever document is meant, and is written out with its document at every
//! site instead** — the header list cannot tell the two apart, and a bare one
//! would resolve to a real and wrong clause. This plan defines a W1 of its own
//! — the runtime marker cited below is therefore written with its document.

/// One row of §11's table.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Declared {
    /// The action, in this document's own words.
    pub action: &'static str,
    /// **The partition unit** — what one unit of it is.
    pub partition_unit: &'static str,
    /// **The test that decided it**, from `State, identity and recovery` §9.2's
    /// three. `None` for the elevated retry, which takes the declaration of the
    /// act it retries.
    pub test: Option<u8>,
    /// The kind, in §11's own words.
    pub kind: &'static str,
    /// The clause of this document that the row is composed from.
    pub clause: &'static str,
}

/// **§11's table, entire.**
///
/// **It is a declaration and not a computation**: nothing here is decided at
/// run time, and a caller reads a row rather than judging an act.
pub const THE_PARTITION_UNITS: [Declared; 10] = [
    Declared {
        action: "an upload request",
        partition_unit: "one file's transfer",
        test: Some(1),
        kind: "partial across files, atomic within one — staging and a single publication; the request's outcome is the enumeration's list",
        clause: "U1-U2",
    },
    Declared {
        action: "the already-exists walk",
        partition_unit: "the whole question",
        test: Some(1),
        kind: "a reading; complete or not asked — a partial count would be wrong in the quiet direction",
        clause: "X2",
    },
    Declared {
        action: "delete",
        partition_unit: "one entry",
        test: Some(1),
        kind: "partial across entries, bottom-up within a folder",
        clause: "M2",
    },
    Declared {
        action: "rename",
        partition_unit: "the rename",
        test: Some(1),
        kind: "atomic entire — one base rename, refused or done",
        clause: "M4",
    },
    Declared {
        action: "move, same filesystem",
        partition_unit: "one top-level item",
        test: Some(1),
        kind: "partial across items, atomic within one",
        clause: "M5",
    },
    Declared {
        action: "move, across filesystems",
        partition_unit: "one file's (copy, delete) pair",
        test: Some(3),
        kind: "one ordered unit per file: the delete is not attempted where the copy did not publish, naming it",
        clause: "M6",
    },
    Declared {
        action: "copy",
        partition_unit: "one file",
        test: Some(1),
        kind: "partial across files, atomic within one — staged and published by one rename",
        clause: "M7",
    },
    Declared {
        action: "new folder",
        partition_unit: "the folder",
        test: Some(1),
        kind: "atomic entire — the plain form, refused or done",
        clause: "M9",
    },
    Declared {
        action: "a disturb pushed past the live-job warning",
        partition_unit: "the (telling, act) pair",
        test: Some(3),
        kind: "one ordered unit: the act is not attempted where the telling could not be written, naming it",
        clause: "W8",
    },
    Declared {
        action: "an elevated retry",
        partition_unit: "the unit of the act retried",
        test: None,
        kind: "the same declaration as the act it retries; elevation changes the route, never the unit. The acts it covers are P6's own — an elevated listing, and the machine's own remove, move, copy or make-directory — and no transfer is retried elevated, so no upload request is in this row",
        clause: "P6, P6a",
    },
];

/// **§11's naming: what a unit of one of these actions is called in `State,
/// identity and recovery` §9.4's outcome record.**
///
/// *"The role is the Partition unit column, word for word, and it is the same
/// words for every unit of one action … What tells two units of one action apart
/// is §9.4's own ordering and nothing else."*
///
/// **So this is a lookup and never a composition.** Nothing here builds a name:
/// the row's own words are the role, which is what keeps *no path, no machine
/// location and no file name* true by there being nothing else it could return.
///
/// `None` where the action is none of §11's. **Three of the rows write no
/// outcome record at all** — the already-exists walk, which is one reading, and
/// a rename and a new folder, both **atomic entire** — because their units are
/// no smaller than the action and §9.4 asks for no record. This answers for them
/// all the same, because what it reports is the declaration rather than whether
/// a record exists.
pub fn the_role_a_unit_of_carries(action: &str) -> Option<&'static str> {
    THE_PARTITION_UNITS
        .iter()
        .find(|row| row.action == action)
        .map(|row| row.partition_unit)
}

/// **The first of §11's three declarations that are not partition units: the
/// upload class.**
///
/// *"The **upload class** is this document's one transfer-priority class (U10),
/// ranked at O4."* `Server protocol and transfers` T8 requires the arranging
/// module to declare it **per action**: every transfer an upload request
/// arranges carries it.
///
/// **Its rank and share are §6.2's and live at [`crate::pool`]**, not restated
/// here — this is the declaration and not the order.
pub const THE_TRANSFER_PRIORITY_CLASS: &str = "upload";

/// **L29's class: picture view**, declared against O4's list beside the upload
/// class — *"a class this clause declares against O4's list: picture view,
/// ranked third — below manual download, above dependency-upload — because a
/// user is at the screen waiting for it and the cap bounds it. N in O5 counts
/// it."* Added 2026-10-31, the Creator's seventeenth notes, note 11.
///
/// **It is a reading and not a transfer**: no transfer record, no Transfers
/// row, nothing lands (L29's *memory only*). It is a class because it takes a
/// channel through the pool, and a channel taken there is admitted by class.
/// **Its rank is placed at [`crate::pool::the_declared_order_with_the_picture_view`]**,
/// by O4's own route for a class declared against the list, so O4's five stand
/// as the plan lists them.
pub const THE_PICTURE_VIEW_CLASS: &str = "picture-view";

/// **The second: the suppression row, and there is one.**
///
/// *"(`File Browser`, `file-browser.delete`) — the delete warning, and nothing
/// else here is dismissible"* (M3, `Desktop app and delivery` §15 G4–G5). **The
/// identity is stable for the life of the application.**
///
/// **The two disturb warnings of §8 are not dismissible and no suppression row
/// exists for them** — Creator-stated. **§8 is built**, at [`crate::warnings`],
/// and it adds no row: [`crate::warnings::no_suppression_row_exists_for_the_live_job_warning`]
/// asks this array that question rather than asserting it, so a row added for
/// §8 fails there.
pub const THE_SUPPRESSION_ROWS: [&str; 1] = ["file-browser.delete"];

/// **The third: no worker kind is declared or wanted.**
///
/// *"Every machine-side act above is a request on the pre-package surface, a
/// transfer channel, or the machine's own program on the command channel, and
/// `Rented-server runtime` W1's closed set is untouched — checked against the
/// copy and the elevated retry, which are command invocations, not workers."*
pub const NO_WORKER_KIND_IS_DECLARED_OR_WANTED: bool = true;

/// Uploads remain available without consulting paid access.
pub fn sending_files_up_is_closed() -> bool {
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_table_is_the_plans_ten_rows_and_every_one_names_its_clause() {
        assert_eq!(THE_PARTITION_UNITS.len(), 10);
        for row in THE_PARTITION_UNITS {
            assert!(!row.action.is_empty());
            assert!(!row.partition_unit.is_empty());
            assert!(!row.kind.is_empty());
            assert!(!row.clause.is_empty(), "{} names no clause", row.action);
        }
    }

    #[test]
    fn the_role_a_unit_carries_is_its_rows_partition_unit_and_never_a_value() {
        for row in THE_PARTITION_UNITS {
            assert_eq!(
                the_role_a_unit_of_carries(row.action),
                Some(row.partition_unit),
                "{}'s role is its own row's words and nothing composed",
                row.action
            );
            // **A role and never a value.** §11: no path, no machine location
            // and no file name — the entry at the unit's position already holds
            // them, and a path here would be one fact with two homes.
            assert!(
                !row.partition_unit.contains('/') && !row.partition_unit.contains('.'),
                "{} names something that looks like a path",
                row.action
            );
        }
        assert_eq!(the_role_a_unit_of_carries("an act nobody declared"), None);
    }

    #[test]
    fn the_two_ordered_pairs_are_the_two_the_plan_declares_and_no_others() {
        let ordered: Vec<&str> = THE_PARTITION_UNITS
            .iter()
            .filter(|row| row.test == Some(3))
            .map(|row| row.action)
            .collect();
        assert_eq!(
            ordered,
            vec![
                "move, across filesystems",
                "a disturb pushed past the live-job warning"
            ]
        );
    }

    #[test]
    fn the_elevated_retry_takes_the_declaration_of_the_act_it_retries() {
        let row = THE_PARTITION_UNITS
            .iter()
            .find(|row| row.action == "an elevated retry")
            .expect("the row is in the table");
        assert_eq!(row.test, None);
        assert!(row.kind.contains("no transfer is retried elevated"));
    }

    #[test]
    fn there_is_exactly_one_suppression_row_and_it_is_the_delete_warning() {
        assert_eq!(THE_SUPPRESSION_ROWS, ["file-browser.delete"]);
        const { assert!(NO_WORKER_KIND_IS_DECLARED_OR_WANTED) };
        assert_eq!(THE_TRANSFER_PRIORITY_CLASS, "upload");
        assert_eq!(THE_PICTURE_VIEW_CLASS, "picture-view");
    }

    #[test]
    fn uploads_are_open_without_any_policy() {
        assert!(!sending_files_up_is_closed());
    }
}
