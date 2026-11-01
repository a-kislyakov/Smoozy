//! Declarations — partition units. `Keys` §11.
//!
//! Per `State, identity and recovery` §9.2, **declared here, decided by the
//! three ordered tests and never at run time.** Each action smaller than its
//! promise writes §9.4's outcome record.
//!
//! **Every outcome record here enumerates its units by role and by nothing
//! else, and carries no path, no fingerprint, no name and no comment.** That
//! is what keeps R2 and W10 true across a durable, user-held record that
//! outlives the row: it says which units were owed and how each ended, and it
//! holds **no value by which Keys could recognise the key again**. W6's
//! confirmation still names the three files by full path at the moment of the
//! act — that is a sentence on screen at that instant, not a durable subject,
//! and the two are not the same thing.

/// The roles a unit is enumerated by. **A role and never a value.**
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UnitRole {
    /// Delete's first file unit. Atomic by the platform's unlink — no
    /// half-deleted file exists to mistake (§9.2 test 2).
    ThePrivateHalf,
    ThePublicHalf,
    TheCertificate,
    /// `Security and licence` §5.1's three writes, **one ordered unit whose
    /// internal order is §5.1's own** (test 1 — the promise is about the whole
    /// forgetting).
    TheErasure,
    /// Generation's G9 directory-ensure, where the location does not exist.
    TheDirectory,
    /// **The tool run plus G4's verification as one unit** (test 2 — a
    /// generated-but-unverified pair could be mistaken for a finished key, so
    /// the unit's outcome is the verification's, and a failed unit removes the
    /// files the act wrote).
    TheKeyPair,
    /// The row creation.
    TheRow,
    /// **An independent unit — merely later, not unsafe**; its failure is
    /// reported and stops nothing.
    TheStorePut,
}

impl UnitRole {
    /// **The role's own name, and it is a role rather than a value.** This is
    /// what goes into `State, identity and recovery` §9.4's record, which
    /// names units in the owning module's own words and never looks at the
    /// text — so the whole of §11's *no path, no fingerprint, no name and no
    /// comment* is kept here, by there being nothing else this function could
    /// return.
    ///
    /// **These are not the words the user reads.** Presenting an outcome
    /// record is `Desktop app and delivery`'s, and Rule 9 binds it there;
    /// these are the plan's own words for its own units.
    pub fn as_role(self) -> &'static str {
        match self {
            UnitRole::ThePrivateHalf => "the private half",
            UnitRole::ThePublicHalf => "the public half",
            UnitRole::TheCertificate => "the certificate",
            UnitRole::TheErasure => "the erasure",
            UnitRole::TheDirectory => "the directory",
            UnitRole::TheKeyPair => "the key pair",
            UnitRole::TheRow => "the row",
            UnitRole::TheStorePut => "the store put",
        }
    }
}

/// Every multi-part action this document declares.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    /// **Delete a managed key.** Each named file is its own unit (test 1 — the
    /// spec's promise names the three files severally), ordered
    /// private-half-first, with the companions and the erasure **not
    /// attempted** where the private half's unit fails (test 3).
    DeleteAManagedKey {
        there_is_a_public_half: bool,
        there_is_a_certificate: bool,
    },
    /// **Remove from Smoozy**, and **clear a remembered passphrase**: one
    /// partition unit each — the §5.1 ordered write set. A crash inside leaves
    /// the mark, and the sweep finishes the erase; the row removal is never
    /// blocked by the store.
    RemoveFromSmoozy,
    ClearARememberedPassphrase,
    /// **Generation**, in order.
    Generation { the_directory_is_needed: bool },
    /// **Point at a key.** One partition unit: the row creation, after
    /// `Desktop app and delivery` K13's
    /// judgement of the picker's path and the U4/F4 readings, **which write
    /// nothing**. Atomic by the record's single write; a refusal leaves no
    /// trace.
    PointAtAKey,
    /// **Point an unusable key at its new location.** One partition unit: the
    /// row's recorded path. The fingerprint is unchanged, so nothing else is
    /// owed.
    PointAtANewLocation,
    /// **Point an unusable key at a replacement.** The erasure and the row's
    /// four fields, **two writes taken as one ordered unit** (F9).
    PointAtAReplacement,
    /// **A passphrase that stopped working, and the ask that follows it.** The
    /// erasure is one ordered unit; the store put of the newly entered
    /// passphrase is an **independent** unit taken on the new attempt's load
    /// succeeding. **The box and what it takes are no part of either unit**,
    /// being run-held and durable nowhere.
    APassphraseThatStoppedWorking,
}

impl Action {
    /// **The act's own name in §9.4's record**, in this document's words.
    pub fn as_action(self) -> &'static str {
        match self {
            Action::DeleteAManagedKey { .. } => "delete a managed key",
            Action::RemoveFromSmoozy => "remove from Smoozy",
            Action::ClearARememberedPassphrase => "clear a remembered passphrase",
            Action::Generation { .. } => "generate a key",
            Action::PointAtAKey => "point at a key",
            Action::PointAtANewLocation => "point at a new location",
            Action::PointAtAReplacement => "point at a replacement",
            Action::APassphraseThatStoppedWorking => "a passphrase that stopped working",
        }
    }

    /// The units by role, in the order they run — what §9.4's enumeration is
    /// written from.
    pub fn roles(self) -> Vec<&'static str> {
        self.units().into_iter().map(UnitRole::as_role).collect()
    }

    /// The units, in the order they run.
    pub fn units(self) -> Vec<UnitRole> {
        match self {
            Action::DeleteAManagedKey {
                there_is_a_public_half,
                there_is_a_certificate,
            } => {
                let mut units = vec![UnitRole::ThePrivateHalf];
                if there_is_a_public_half {
                    units.push(UnitRole::ThePublicHalf);
                }
                if there_is_a_certificate {
                    units.push(UnitRole::TheCertificate);
                }
                units.push(UnitRole::TheErasure);
                units
            }
            Action::RemoveFromSmoozy | Action::ClearARememberedPassphrase => {
                vec![UnitRole::TheErasure]
            }
            Action::Generation {
                the_directory_is_needed,
            } => {
                let mut units = Vec::new();
                if the_directory_is_needed {
                    units.push(UnitRole::TheDirectory);
                }
                units.extend([UnitRole::TheKeyPair, UnitRole::TheRow, UnitRole::TheStorePut]);
                units
            }
            Action::PointAtAKey => vec![UnitRole::TheRow],
            Action::PointAtANewLocation => vec![UnitRole::TheRow],
            // One ordered unit holding two writes: the erasure, then the row.
            Action::PointAtAReplacement => vec![UnitRole::TheErasure, UnitRole::TheRow],
            Action::APassphraseThatStoppedWorking => {
                vec![UnitRole::TheErasure, UnitRole::TheStorePut]
            }
        }
    }

    /// **Whether `State, identity and recovery` §9.4's outcome record is
    /// owed**, which that document decides by one question: are the action's
    /// units smaller than the action?
    ///
    /// **Three of this document's acts have such units** — delete a managed
    /// key, generation, and a passphrase that stopped working. *(This read
    /// **two** in the plan until 2026-08-29: generation declares four ordered
    /// units, so its record was owed all along.)*
    ///
    /// **A replacement's two writes are one ordered unit**, so the action is
    /// atomic entire and owes no record — which is why it is not a fourth here.
    pub fn owes_an_outcome_record(self) -> bool {
        matches!(
            self,
            Action::DeleteAManagedKey { .. }
                | Action::Generation { .. }
                | Action::APassphraseThatStoppedWorking
        )
    }
}

/// **A crash between the file units and the erasure leaves the row, and
/// startup completes nothing.**
///
/// After the last unlink and before §5.1's first write, the store item stands
/// **unmarked** — so the sweep, which acts on marks and nothing else, cannot
/// reach it — while the private half is already destroyed. **The row
/// persists**, honestly showing the file missing (U1 set 1, U4 clause 1
/// failing), and the passphrase stays until the user removes the row, which
/// runs §5.1 in full. **Nothing lies and nothing is used**: clause 1 fails, so
/// no attempt ever supplies that passphrase.
///
/// **Startup does not complete the erasure from the outcome record, and that
/// is the decision rather than an omission** — the sweep is the one thing
/// `Security and licence` §5.1 gives for finishing an owed erase, it runs on
/// marks, and a second driver reading outcome records would be a rule about
/// erasing that this document does not own.
pub const STARTUP_COMPLETES_NO_ERASURE_FROM_AN_OUTCOME_RECORD: bool = true;

/// **A crash between the tool run and the row creation leaves a verified pair
/// with no row**, which the next listing discovers as U1 set 2 — visible, not
/// silent; the passphrase is then not held and the first connect asks, the
/// spec's own fallback.
pub const AN_UNROWED_PAIR_IS_DISCOVERED_RATHER_THAN_LOST: bool = true;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_delete_names_only_the_files_that_are_there_and_the_private_half_first() {
        assert_eq!(
            Action::DeleteAManagedKey {
                there_is_a_public_half: true,
                there_is_a_certificate: true
            }
            .units(),
            vec![
                UnitRole::ThePrivateHalf,
                UnitRole::ThePublicHalf,
                UnitRole::TheCertificate,
                UnitRole::TheErasure
            ]
        );
        assert_eq!(
            Action::DeleteAManagedKey {
                there_is_a_public_half: false,
                there_is_a_certificate: false
            }
            .units(),
            vec![UnitRole::ThePrivateHalf, UnitRole::TheErasure]
        );
    }

    #[test]
    fn the_two_removals_are_one_unit_each() {
        assert_eq!(Action::RemoveFromSmoozy.units(), vec![UnitRole::TheErasure]);
        assert_eq!(
            Action::ClearARememberedPassphrase.units(),
            vec![UnitRole::TheErasure]
        );
    }

    #[test]
    fn generation_runs_the_pair_before_the_row_and_the_row_before_the_put() {
        assert_eq!(
            Action::Generation {
                the_directory_is_needed: true
            }
            .units(),
            vec![
                UnitRole::TheDirectory,
                UnitRole::TheKeyPair,
                UnitRole::TheRow,
                UnitRole::TheStorePut
            ]
        );
        assert_eq!(
            Action::Generation {
                the_directory_is_needed: false
            }
            .units()[0],
            UnitRole::TheKeyPair
        );
    }

    #[test]
    fn a_replacement_erases_before_it_writes_the_row() {
        assert_eq!(
            Action::PointAtAReplacement.units(),
            vec![UnitRole::TheErasure, UnitRole::TheRow]
        );
    }

    #[test]
    fn a_role_is_a_role_and_every_one_is_distinct() {
        let every = [
            UnitRole::ThePrivateHalf,
            UnitRole::ThePublicHalf,
            UnitRole::TheCertificate,
            UnitRole::TheErasure,
            UnitRole::TheDirectory,
            UnitRole::TheKeyPair,
            UnitRole::TheRow,
            UnitRole::TheStorePut,
        ];
        let names: std::collections::HashSet<&str> =
            every.iter().map(|role| role.as_role()).collect();
        assert_eq!(names.len(), every.len());
        // §11: no path, no fingerprint, no name, no comment. A role name
        // carries no separator a path would need and no colon a fingerprint's
        // printed form carries.
        assert!(names
            .iter()
            .all(|name| !name.contains('/') && !name.contains('\\') && !name.contains(':')));
    }

    #[test]
    fn exactly_three_acts_owe_an_outcome_record() {
        let every = [
            Action::DeleteAManagedKey {
                there_is_a_public_half: true,
                there_is_a_certificate: true,
            },
            Action::RemoveFromSmoozy,
            Action::ClearARememberedPassphrase,
            Action::Generation {
                the_directory_is_needed: false,
            },
            Action::PointAtAKey,
            Action::PointAtANewLocation,
            Action::PointAtAReplacement,
            Action::APassphraseThatStoppedWorking,
        ];
        let owing: Vec<&Action> = every
            .iter()
            .filter(|action| action.owes_an_outcome_record())
            .collect();
        assert_eq!(owing.len(), 3);
        assert!(!Action::PointAtAReplacement.owes_an_outcome_record());
        assert!(!Action::RemoveFromSmoozy.owes_an_outcome_record());
    }
}
