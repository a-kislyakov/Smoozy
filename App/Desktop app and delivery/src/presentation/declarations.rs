//! **What a module declares that this document's services need** — §4 A7.
//!
//! A7 names five things a module declares in its own document and does not
//! decide at run time. **Two of them are this section's and are built here**;
//! the other three have routes already and are named below so nobody builds a
//! second one.
//!
//! | A7's five | Where the route is |
//! |---|---|
//! | its actions' partition unit, and which of `State, identity and recovery` §9.2's three tests decided it | the owning module's own crate — `File Browser`'s `declarations::THE_PARTITION_UNITS` is the shape every other one follows — read here through [`ThePartitionUnit`] |
//! | each transfer's priority class | the owning module's own crate, ranked at `Server protocol and transfers` §6.2 |
//! | **each dismissible warning's identity** (§15.2) | **here**, [`Declared::dismissible_warnings`]. **Piece F writes to it** |
//! | **each condition it raises** (`State, identity and recovery` §8.3) | **here**, [`Declared::conditions`] |
//! | each act it writes to the diary | [`smoozy_reporting::lines::Declarations`], registered at start into [`crate::start::Started::declarations`] |
//!
//! ## Why the two that are here are here
//!
//! **A condition with no named supplier cannot be withdrawn and must not be
//! raised** — `State, identity and recovery` F12, in those words. §8.3's table
//! gives every condition kind four things: who supplies the observed value,
//! when it changes, what it holds over, and what withdraws it. **A raiser that
//! has not said all four has raised a record with no route out**, and §8.2 F6's
//! *withdrawn by that change alone* has nothing to fire on. So the declaration
//! is what [`crate::record::failure_records::FailureRecords::a_condition_was_raised`]
//! refuses without.
//!
//! **And a dismissible warning's identity is stable for the life of the
//! application** (§15 G5), which is why it is declared rather than composed:
//! G2's failure is invisible — *"nobody finds out until a user presses the
//! control and one warning does not come back"* — and a changed identity is
//! that same failure by a different door.
//!
//! ## The identity carries its own module, and that is where G4's *the module*
//! lives
//!
//! **§15 G4 and `State, identity and recovery` §10.3 both say a suppression row
//! is *the module and the warning identity*.** The row in the local record is
//! **one column** — the ruling is at
//! [`crate::record::schema::VERSION_15`] and the reasoning is at
//! [`crate::record::suppressed_warnings`] — so what makes those two sentences
//! literally true is the identity's own spelling: **every declared warning
//! identity is prefixed with its own module's name.** `File Browser` M3
//! declares `file-browser.delete`; `Rendering` W4a declares
//! `rendering.missing-assets`.
//!
//! **This route is the one place that binding can be checked**, because it is
//! the one place a module and its identities are held together, and
//! [`TheDeclarations::every_declared_warning_names_its_module`] is the check.
//! A module column in the table would be a second copy of the same fact, with
//! nothing reading it: G7's restore is one delete over the whole table and
//! walks no modules.
//!
//! ## What this is not
//!
//! **It is not a place a module's declaration lives.** The declaration lives in
//! the module's own document and, in code, in the module's own crate, exactly
//! as `File Browser`'s does. **This is the register the services read**, filled
//! at start from those crates, the same shape the diary declarations already
//! take. **A module with no declaration is a module that cannot be hosted, not
//! a default this document supplies** — so nothing here has a fallback and
//! every answer for an unregistered module is *no*.
//!
//! **Markers used in this file that belong to another document, each with the
//! document it belongs to.** `State, identity and recovery` §8.3 and §9.2.
//!
//! **`State, identity and recovery` F12 and `Server protocol and transfers` T8
//! are deliberately not listed above and are written out with their document at
//! every site instead** — this plan defines an F12 and a T8 of its own, so a bare
//! one would resolve to a real and wrong clause.

use std::collections::HashMap;

/// **The prefix a module's warning identities carry** — §15 G4's *the module*,
/// carried inside the identity rather than in a second column.
///
/// It is the module's own name, lower-cased, with every run of anything that is
/// not a letter or a digit becoming one hyphen. `File Browser` gives
/// `file-browser`; `Rendering` gives `rendering`; `Dependency Upload & Remap`
/// would give `dependency-upload-remap`.
///
/// **It composes nothing that is written anywhere.** An identity is declared in
/// the owning module's own document and is stable for the life of the
/// application (G5); this derives what that identity has to begin with so the
/// binding can be **checked**, and a builder that generated an identity from it
/// would have made the identity a function of a name somebody can rename, which
/// is G2's invisible failure by a third door.
pub fn the_prefix_a_modules_identities_carry(module: &str) -> String {
    let mut out = String::new();
    for character in module.chars() {
        if character.is_ascii_alphanumeric() {
            out.push(character.to_ascii_lowercase());
        } else if !out.ends_with('-') {
            out.push('-');
        }
    }
    out.trim_matches('-').to_string()
}

/// **Whether a warning identity names the module that declared it** — §15 G4.
///
/// The identity begins with the module's own prefix and a full stop, and has
/// something after it. A bare prefix is not an identity: it names a module and
/// no warning.
pub fn the_identity_names_its_module(module: &str, identity: &str) -> bool {
    let prefix = the_prefix_a_modules_identities_carry(module);
    if prefix.is_empty() {
        return false;
    }
    identity
        .strip_prefix(&prefix)
        .and_then(|rest| rest.strip_prefix('.'))
        .is_some_and(|rest| !rest.is_empty())
}

/// **One condition a module raises**, against `State, identity and recovery`
/// §8.3's shape. **All four columns, because `State, identity and recovery` F12
/// requires all four.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ACondition {
    /// The condition's kind, in §8.3's own words — *the server connection*,
    /// *disk full at a mount*. It is part of the condition key and it is what
    /// makes a disk-full and a permission refusal at one mount two conditions
    /// rather than one.
    pub kind: &'static str,
    /// **Who supplies the observed value.** `State, identity and recovery` F12:
    /// a condition with no named
    /// supplier cannot be withdrawn and must not be raised.
    pub supplied_by: &'static str,
    /// **What it holds over** — the values over which the condition is shown.
    /// The connection condition has two, because an outage reads as
    /// *disconnected* and as *reconnecting* by turns while it lasts.
    pub holds_over: &'static [&'static str],
    /// **What withdraws it.** Written out rather than derived as *anything
    /// else*, because the disk-full row takes **both** signals and not either:
    /// a non-zero free-space reading **and** a real write to that place
    /// succeeding.
    pub withdrawn_on: &'static str,
}

/// **One action's partition unit** — §9.2's declaration, read here rather than
/// re-decided. **M27 is what needs it**: where the partition unit is the whole
/// action there is no unit list to show, and *"which kind an action is comes
/// from its own document's declaration and is never decided at the moment of
/// drawing it."*
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ThePartitionUnit {
    /// The action, in the owning module's own words.
    pub action: &'static str,
    /// **Whether the unit is the whole action.** This is the one bit M27 turns
    /// on, and it is the module's answer rather than a count of rows in a
    /// record: an action whose enumeration happens to hold one unit today is
    /// not the same thing as an action that is atomic entire.
    pub the_unit_is_the_whole_action: bool,
}

/// One module's declaration.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Declared {
    /// §8.3, and `State, identity and recovery` F12.
    pub conditions: Vec<ACondition>,
    /// §15.2 G4 and G5. **The identity and nothing else** — no wording, no
    /// timestamp, no count, no per-subject qualifier.
    pub dismissible_warnings: Vec<&'static str>,
    /// §9.2, read for M27.
    pub partition_units: Vec<ThePartitionUnit>,
}

/// **The register the services read.**
///
/// **A module that declared nothing answers *no* to everything**, which is A7's
/// *a declaration missing is a module that cannot be hosted, not a default this
/// document supplies*. There is no default set and no *every*.
#[derive(Debug, Default)]
pub struct TheDeclarations {
    by_module: HashMap<&'static str, Declared>,
}

impl TheDeclarations {
    pub fn new() -> Self {
        TheDeclarations::default()
    }

    /// Register one module's declaration, at start, from that module's own
    /// crate.
    pub fn module_declares(&mut self, module: &'static str, declared: Declared) {
        self.by_module.insert(module, declared);
    }

    /// **Whether this module declared this condition kind.** What
    /// [`crate::record::failure_records`] refuses a raising without.
    pub fn the_condition_is_declared(&self, module: &str, kind: &str) -> bool {
        self.by_module
            .get(module)
            .is_some_and(|declared| declared.conditions.iter().any(|row| row.kind == kind))
    }

    /// The declaration itself, for a presentation that needs the withdrawal
    /// columns.
    pub fn the_condition(&self, module: &str, kind: &str) -> Option<&ACondition> {
        self.by_module
            .get(module)?
            .conditions
            .iter()
            .find(|row| row.kind == kind)
    }

    /// **Whether this warning identity was declared by this module** — §15 G5.
    pub fn the_warning_is_declared(&self, module: &str, identity: &str) -> bool {
        self.by_module
            .get(module)
            .is_some_and(|declared| declared.dismissible_warnings.contains(&identity))
    }

    /// **Every declared warning identity that does not name its own module** —
    /// §15 G4, checked at the one place the binding lives.
    ///
    /// **An empty answer is the whole of the rule kept.** G4 and
    /// `State, identity and recovery` §10.3 say a suppression row is *the
    /// module and the warning identity*, and the row is one column: the module
    /// is in the spelling. An identity that does not carry it makes those two
    /// sentences false and leaves nothing anywhere able to say which module a
    /// row belongs to.
    pub fn every_declared_warning_names_its_module(&self) -> Vec<(&'static str, &'static str)> {
        let mut wrong: Vec<(&'static str, &'static str)> = self
            .by_module
            .iter()
            .flat_map(|(module, declared)| {
                declared
                    .dismissible_warnings
                    .iter()
                    .filter(|identity| !the_identity_names_its_module(module, identity))
                    .map(move |identity| (*module, *identity))
            })
            .collect();
        wrong.sort_unstable();
        wrong
    }

    /// **M27: whether this action's partition unit is the whole action**, so
    /// there is no unit list to show. **`None` where the module declared
    /// nothing about the action**, which is a hosting fault the caller reports
    /// rather than a *no* it may act on.
    pub fn the_unit_is_the_whole_action(&self, module: &str, action: &str) -> Option<bool> {
        self.by_module
            .get(module)?
            .partition_units
            .iter()
            .find(|row| row.action == action)
            .map(|row| row.the_unit_is_the_whole_action)
    }

    /// Every warning identity registered, across every module. **Piece F's one
    /// control deletes over the table and walks no modules** (§15 G7), so this
    /// is for checking a row against a declaration and never for composing the
    /// delete.
    pub fn every_declared_warning(&self) -> Vec<&'static str> {
        let mut out: Vec<&'static str> = self
            .by_module
            .values()
            .flat_map(|declared| declared.dismissible_warnings.iter().copied())
            .collect();
        out.sort_unstable();
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const A_CONNECTION: ACondition = ACondition {
        kind: "the server connection",
        supplied_by: "Servers",
        holds_over: &["disconnected", "reconnecting"],
        withdrawn_on: "connected",
    };

    fn a_register() -> TheDeclarations {
        let mut register = TheDeclarations::new();
        register.module_declares(
            "Servers",
            Declared {
                conditions: vec![A_CONNECTION],
                dismissible_warnings: Vec::new(),
                partition_units: Vec::new(),
            },
        );
        register
    }

    /// **A7: a module with no declaration writes nothing, and it is a hosting
    /// fault rather than a default.** Nothing here has a fallback.
    #[test]
    fn a_module_that_declared_nothing_answers_no_to_everything() {
        let register = a_register();
        assert!(!register.the_condition_is_declared("File Browser", "the server connection"));
        assert!(!register.the_warning_is_declared("File Browser", "file-browser.delete"));
        assert_eq!(register.the_unit_is_the_whole_action("File Browser", "delete"), None);
    }

    /// **`State, identity and recovery` F12: all four columns.**
    #[test]
    fn a_declared_condition_names_its_supplier_and_both_withdrawal_columns() {
        let register = a_register();
        let declared = register
            .the_condition("Servers", "the server connection")
            .expect("it was declared");
        assert_eq!(declared.supplied_by, "Servers");
        assert_eq!(declared.holds_over.len(), 2);
        assert!(!declared.withdrawn_on.is_empty());
    }

    /// **M27 reads the declaration, not the record.** An action declared atomic
    /// entire says so whatever a record happens to hold.
    #[test]
    fn the_partition_unit_is_read_from_the_declaration() {
        let mut register = a_register();
        register.module_declares(
            "File Browser",
            Declared {
                conditions: Vec::new(),
                dismissible_warnings: vec!["file-browser.delete"],
                partition_units: vec![
                    ThePartitionUnit {
                        action: "rename",
                        the_unit_is_the_whole_action: true,
                    },
                    ThePartitionUnit {
                        action: "an upload request",
                        the_unit_is_the_whole_action: false,
                    },
                ],
            },
        );
        assert_eq!(
            register.the_unit_is_the_whole_action("File Browser", "rename"),
            Some(true)
        );
        assert_eq!(
            register.the_unit_is_the_whole_action("File Browser", "an upload request"),
            Some(false)
        );
        assert!(register.the_warning_is_declared("File Browser", "file-browser.delete"));
        assert_eq!(register.every_declared_warning(), vec!["file-browser.delete"]);
    }

    /// **§15 G4: the module is in the identity's own spelling.**
    ///
    /// **This is the test the ruling of step 10a piece F rests on.** The
    /// suppression row is one column, so *the module and the warning identity*
    /// is true of the string or it is not true at all. A module column would be
    /// the same fact in two places with no reader, which is the fault this
    /// Build stage keeps meeting.
    #[test]
    fn every_declared_warning_identity_names_its_own_module() {
        let mut register = a_register();
        register.module_declares(
            "File Browser",
            Declared {
                dismissible_warnings: vec!["file-browser.delete"],
                ..Declared::default()
            },
        );
        register.module_declares(
            "Rendering",
            Declared {
                dismissible_warnings: vec!["rendering.missing-assets"],
                ..Declared::default()
            },
        );
        assert_eq!(
            register.every_declared_warning_names_its_module(),
            Vec::new(),
            "a declared warning identity does not carry its own module"
        );
    }

    /// **And an identity that does not carry its module is caught rather than
    /// registered quietly.**
    ///
    /// The three shapes a builder would actually write: the module left off
    /// altogether, another module's prefix, and a prefix with no warning after
    /// it.
    #[test]
    fn an_identity_that_does_not_name_its_module_is_found() {
        let mut register = TheDeclarations::new();
        register.module_declares(
            "File Browser",
            Declared {
                dismissible_warnings: vec!["delete", "rendering.missing-assets", "file-browser."],
                ..Declared::default()
            },
        );
        assert_eq!(
            register.every_declared_warning_names_its_module(),
            vec![
                ("File Browser", "delete"),
                ("File Browser", "file-browser."),
                ("File Browser", "rendering.missing-assets"),
            ]
        );
    }

    /// **The prefix is the module's own name and nothing invented.**
    #[test]
    fn the_prefix_is_the_modules_own_name() {
        assert_eq!(
            the_prefix_a_modules_identities_carry("File Browser"),
            "file-browser"
        );
        assert_eq!(
            the_prefix_a_modules_identities_carry("Rendering"),
            "rendering"
        );
        assert_eq!(
            the_prefix_a_modules_identities_carry("Dependency Upload & Remap"),
            "dependency-upload-remap"
        );
        assert_eq!(the_prefix_a_modules_identities_carry(""), "");
        assert!(!the_identity_names_its_module("", "anything"));
    }
}
