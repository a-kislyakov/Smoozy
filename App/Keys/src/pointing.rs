//! Pointing at a key, mending one, and the replacement act — `Keys` §5.3 and
//! §5.5.
//!
//! **`Desktop app and delivery` K11 and `Desktop app and delivery` K13 come
//! first and are that document's**: the system
//! file picker is reached through the adapter alone, and whatever a picker
//! returns is a path from the user, judged like any other name. What arrives
//! here is a path that has already been through that.

use std::path::{Path, PathBuf};

use smoozy_state::identities::Identity;

use crate::fingerprint::{self, Fingerprint, PublicKeyBlob, Reading, SecondReference};
use crate::naming;
use crate::ownership::UsedBy;
use crate::rows::{KeyRow, Name};
use crate::usable::{Unusable, Verdict, WhyItWouldNotRead};

/// **F6 and F9: which act a pointed-at file may take, and the fingerprint
/// decides which happened.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Form {
    /// A key kept elsewhere entering Smoozy for the first time.
    AKeyKeptElsewhere,
    /// **New location:** the pointed-at file's fingerprint equals the row's —
    /// then the row's path is updated and nothing else moves. **It costs
    /// nothing, and the row states nothing ahead of it.**
    AnUnusableKeysNewLocation,
    /// **Replacement:** a different fingerprint, taken deliberately through
    /// the replacement offer.
    AReplacement,
}

/// Why a point was refused. **Nothing is added to the failed clause** (Row
/// 245's second quote).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Refused {
    /// U4's clause that failed, said plainly, **with which situation produced
    /// it where the readings knew one**.
    ///
    /// **U6's closed set is not widened and no sixth reason is invented** — the
    /// clause is still one of the five. The situation is carried beside it, as
    /// U6a carries *not checked* beside the set, because two different refusals
    /// were reaching the user under one sentence: a file that could not be
    /// read, and a perfectly good key whose fingerprint has nowhere to be read
    /// from. `Preparation/Design/Keys.md`: *"Two different refusals, and they
    /// must not read alike… a single message would leave the user hunting for
    /// a fault in a file that has none."*
    TheClauseThatFailed {
        clause: Unusable,
        situation: Option<WhyItWouldNotRead>,
    },
    /// **F4: the same key another row already lists**, with a plain message
    /// naming that entry.
    ThisKeyIsAlreadyListed { as_row: PathBuf },
    /// **F5: two different blobs under one fingerprint**, refused as two
    /// different keys that Smoozy cannot tell apart, said in those words.
    TwoKeysSmoozyCannotTellApart { the_other_row: PathBuf },
    /// **F6: a non-matching file offered as a new location is refused as not
    /// this key.** The replacement offer is the deliberate route for a
    /// different key, and this refusal is what makes it deliberate.
    NotThisKey,
    /// **U6a: the readings a point stands on could not be taken at all**,
    /// because this computer has no `ssh-keygen` (G6). It is **not** a failed
    /// clause and no member of U6's set is composed for it: nothing about the
    /// file failed. The statement is 6.3's, said once where the readings were
    /// needed rather than per file.
    TheSystemsSshToolsAreNotInstalled,
}

/// What the user picked, as this module has read it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Picked {
    pub path: PathBuf,
    /// U10's check over the picked file, taken at this moment.
    pub verdict: Verdict,
    /// **Which situation produced an *unreadable* verdict**, carried through so
    /// the refusal can say it. Absent for every other verdict.
    pub why_it_would_not_read: Option<WhyItWouldNotRead>,
    pub fingerprint: Option<Reading>,
    pub blob: Option<PublicKeyBlob>,
    pub name: Option<Name>,
}

/// Which offer the user took. **F6 settles which offer a pointed-at file may
/// take**, and the two mending offers are different acts with different
/// writes, so the caller names which control was pressed rather than this
/// function inferring it.
///
/// **The row a mending offer names is named by its own identity and never by a
/// position in a list.** A position is not the key: it depends on what order
/// the caller happened to be holding, two of them could disagree with each
/// other, and neither can be checked. The identity is the row's own, issued by
/// the local record and changed by nothing — including by a replacement, which
/// keeps it (F10, F11). It is also the one identifier here, where there were
/// two: the act took a position and the offer carried a second one into the
/// same list.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Offered {
    /// A key kept elsewhere, from the standalone screen or a server dialog.
    AFirstPoint,
    /// The unusable row's *point at its new location* control.
    ANewLocationFor(Identity),
    /// The unusable row's *point at a replacement* control.
    AReplacementFor(Identity),
}

impl Offered {
    /// The row being mended, where one is being mended.
    pub fn the_row_being_mended(&self) -> Option<&Identity> {
        match self {
            Offered::AFirstPoint => None,
            Offered::ANewLocationFor(row) | Offered::AReplacementFor(row) => Some(row),
        }
    }
}

/// **F9: the picked file is judged before either act runs.**
///
/// U4's five clauses first, a failure refused with the failed clause said;
/// then F4 over **every other current row**. A file whose fingerprint equals
/// this row's own is the new location and never a replacement; a file whose
/// fingerprint equals a different row's is F4's refusal — **which is what
/// keeps F7 true across a replacement exactly as across a first point.**
///
/// `rows` is every current row and `offered` names which row is being mended,
/// **by that row's own identity**. `blob_of` is F5's second reading, taken only
/// where a fingerprint already matched: **where a blob is unreadable without a
/// passphrase the comparison cannot run and the refusal stands on the
/// fingerprint alone** (§13 item 1 carries the residual).
pub fn judge(
    picked: &Picked,
    offered: &Offered,
    rows: &[KeyRow],
    blob_of: &dyn Fn(&KeyRow) -> Option<PublicKeyBlob>,
) -> Result<Form, Refused> {
    match &picked.verdict {
        Verdict::Unusable(reason) => {
            return Err(Refused::TheClauseThatFailed {
                clause: *reason,
                situation: picked.why_it_would_not_read,
            })
        }
        // **U6a: nothing is handed to a caller in place of a verdict that was
        // never read.** A computer with no generator cannot judge a picked
        // file either, and calling it unreadable would be a statement about
        // the key that is false.
        Verdict::NotChecked => return Err(Refused::TheSystemsSshToolsAreNotInstalled),
        Verdict::Usable => {}
    }

    let mending = offered.the_row_being_mended();

    for row in rows.iter() {
        if mending == Some(&row.identifier) {
            continue;
        }
        let (Some(picked_fingerprint), Some(held)) = (&picked.fingerprint, &row.fingerprint) else {
            continue;
        };
        match fingerprint::compare_against_a_listed_row(
            &picked_fingerprint.value,
            picked.blob.as_ref(),
            &held.value,
            blob_of(row).as_ref(),
        ) {
            SecondReference::ADifferentKey => {}
            SecondReference::TheKeyThisRowAlreadyHolds => {
                return Err(Refused::ThisKeyIsAlreadyListed {
                    as_row: row.path.clone(),
                })
            }
            SecondReference::TwoKeysSmoozyCannotTellApart => {
                return Err(Refused::TwoKeysSmoozyCannotTellApart {
                    the_other_row: row.path.clone(),
                })
            }
        }
    }

    match mending {
        None => Ok(Form::AKeyKeptElsewhere),
        Some(identifier) => {
            // **The row is found by its own identity.** A caller naming a row
            // that is not in this list is naming nothing, which is refused
            // rather than answered from whatever sat at that position.
            let Some(row) = rows.iter().find(|row| row.identifier == *identifier) else {
                return Err(Refused::NotThisKey);
            };
            let same_key = match (&picked.fingerprint, &row.fingerprint) {
                (Some(picked), Some(held)) => picked.value == held.value,
                _ => false,
            };
            match (offered, same_key) {
                (Offered::ANewLocationFor(_), true) => Ok(Form::AnUnusableKeysNewLocation),
                (Offered::ANewLocationFor(_), false) => Err(Refused::NotThisKey),
                // **A file whose fingerprint equals this row's own is the new
                // location and never a replacement**, whichever control was
                // pressed: the act that follows writes one field rather than
                // four, and nothing is erased.
                (Offered::AReplacementFor(_), true) => Ok(Form::AnUnusableKeysNewLocation),
                (Offered::AReplacementFor(_), false) => Ok(Form::AReplacement),
                (Offered::AFirstPoint, _) => Ok(Form::AKeyKeptElsewhere),
            }
        }
    }
}

/// **F9: the new location's act is one write — the row's recorded path.**
///
/// The fingerprint does not change, so S1's keying, the server rows' match
/// value (`Servers` P16) and the *Used by* line are untouched.
pub fn take_the_new_location(row: &mut KeyRow, path: PathBuf) {
    row.path = path;
}

/// **F9 and F10: what the replacement act writes, and it is four fields, not
/// two.**
///
/// A replacement is a **different key** with its own comment, so leaving R1's
/// *name as last read* and *was-default-at-entry* bit standing would have the
/// row permanently displaying the departed key's name and would run N1's
/// duplicate-name reading against a name and a bit belonging to a key that is
/// gone.
///
/// **The erasure runs first and is the caller's** — `Security and licence`
/// §5.1's three writes, raised deliberately as §5.2's swap event (F10) —
/// because §5.1's own first write is what closes the window: marking the
/// subject as owing an erase turns U9's availability answer to **no** at that
/// instant. This function is the second write of that ordered unit, and it
/// **drops the reference**, so a passphrase later entered for the replaced key
/// is a first store and takes a new subject.
///
/// **Nothing else on the row is erased**: the **local key identifier** is
/// untouched, which is what keeps the row and its servers' reference to it —
/// and what makes F11's stranding happen at all.
pub fn take_the_replacement(
    row: &mut KeyRow,
    path: PathBuf,
    fingerprint: Option<Reading>,
    name: Option<Name>,
    the_default_string_now: &str,
) {
    row.was_default_at_entry = naming::was_default_at_entry(name.as_ref(), the_default_string_now);
    row.path = path;
    row.fingerprint = fingerprint;
    row.name = name;
    row.passphrase = None;
}

/// **F11: the replacement leaves the server rows unmatched, and this document
/// re-points none of them.**
///
/// A server row holds a **pair** — a local key identifier and the fingerprint
/// beside it (`Servers` D3) — and the recognition `Servers` P16 hands to this
/// document is taken **over both**. Where the two disagree the answer to
/// `Servers` P1's first question is no, and `Servers` P13's derivation is
/// *needing a new key* at the next draw.
///
/// **A replacement keeps the identifier and changes the fingerprint, so every
/// server that held the old key is unmatched at the next draw**; the identifier
/// alone would have carried those servers silently onto a key their rows never
/// named.
pub fn the_server_row_still_names_this_key(
    row: &KeyRow,
    server_holds_this_identifier: &smoozy_state::identities::Identity,
    server_holds_this_fingerprint: &Fingerprint,
) -> bool {
    row.identifier == *server_holds_this_identifier
        && row
            .fingerprint
            .as_ref()
            .is_some_and(|held| held.value == *server_holds_this_fingerprint)
}

/// **F12: the cost said on the row before the control is pressed, and both
/// halves are values the row already carries.**
///
/// **Both are the local record's own**, so the statement stands on the row
/// before any file is chosen: nothing is read from the file being pointed at,
/// nothing reaches the store — `Security and licence` §3.0 rule 6 is untouched,
/// since no line here is composed from a stored value — and nothing reaches a
/// machine.
///
/// **The statement is not a confirmation and adds no act.** Its wording and
/// placement are Design's.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WhatAReplacementCosts {
    /// **W11a's standing *Used by* line at that draw**, matched on the row's
    /// recorded fingerprint — the old one, which is exactly the set the act
    /// strands. **No second read is taken here and no second list is
    /// composed.**
    pub the_servers_it_strands: UsedBy,
    /// **U9's marker** — stated **only where the row carries a secret-store
    /// subject reference**. **Where none is carried there is nothing to
    /// lose**, and stating that a remembered passphrase goes would name a cost
    /// this act is not taking.
    pub the_remembered_passphrase_goes: bool,
}

impl WhatAReplacementCosts {
    pub fn of(row: &KeyRow, used_by: UsedBy) -> WhatAReplacementCosts {
        WhatAReplacementCosts {
            the_servers_it_strands: used_by,
            the_remembered_passphrase_goes: row.passphrase.is_some(),
        }
    }
}

/// **U12: a key the caller's dialog was holding may have been replaced here,
/// and that does not blank the field — it leaves the held pair's fingerprint
/// stale.**
///
/// What crossed back at the pick is the pair, and a replacement keeps the
/// identifier and changes the fingerprint. Left alone, a server written from
/// that pair is unmatched at its first draw and `Servers` P13's derivation is
/// *needing a new key* — **a server created already needing one**, which §12
/// states is impossible.
///
/// **The rule is the one U12 already uses for a removed key, applied to the
/// same moment: the pair is re-taken from the key row when the dialog is shown
/// again**, before anything of the dialog's is written. A gone row empties the
/// field; a live row hands its **current** fingerprint back beside the
/// identifier it already held. **No key row is changed by it.**
pub fn re_take_the_pair(
    held_identifier: &smoozy_state::identities::Identity,
    rows: &[KeyRow],
) -> Option<(smoozy_state::identities::Identity, Option<Fingerprint>)> {
    rows.iter()
        .find(|row| row.identifier == *held_identifier)
        .map(|row| {
            (
                row.identifier.clone(),
                row.fingerprint.as_ref().map(|held| held.value.clone()),
            )
        })
}

/// **U11: the screen opened from a server dialog is U1's list with no unusable
/// key offered, plus a display-only context the caller hands over; no rule of
/// this screen turns on that context.**
///
/// For the **Edit Server** dialog it is the saved row's label. For the **Add
/// Server** dialog it is the dialog's own fields as they stand — the optional
/// name, the username and the host — **any or all of which may be empty**.
/// **Nothing is read from the local record for an unsaved server, because
/// there is no row**, and no name is composed for one from anything else.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct DialogContext {
    pub label: Option<String>,
    pub username: Option<String>,
    pub host: Option<String>,
}

/// The rows a server dialog's pick may be taken from: **no unusable key
/// offered**, which is what keeps §12's *a server can never be created already
/// needing a new key* true.
///
/// **A row whose verdict has not arrived is not offered either** — U10a's bound
/// is that no verdict may be drawn that has not been read, and offering a row
/// for a pick is standing on a verdict.
pub fn offerable_to_a_server_dialog<'a>(
    drawn: &'a [(KeyRow, Option<Verdict>)],
) -> Vec<&'a KeyRow> {
    drawn
        .iter()
        .filter(|(_, verdict)| matches!(verdict, Some(Verdict::Usable)))
        .map(|(row, _)| row)
        .collect()
}

/// What a pick hands back — **the pair, or nothing at a cancel. Keys hands
/// back no fields, no context and no record of having been open.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WhatReturns {
    APick {
        identifier: smoozy_state::identities::Identity,
        fingerprint: Option<Fingerprint>,
    },
    Nothing,
}

impl WhatReturns {
    pub fn picked(row: &KeyRow) -> WhatReturns {
        WhatReturns::APick {
            identifier: row.identifier.clone(),
            fingerprint: row.fingerprint.as_ref().map(|held| held.value.clone()),
        }
    }
}

/// The path a picker returned, as this module records it — **L3's canonical
/// path of the file that actually backs the key**, resolved at the moment the
/// key entered.
pub fn recorded_path(picked: &Path, paths: &dyn crate::ownership::CanonicalPaths) -> Option<PathBuf> {
    paths.canonical(picked)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fingerprint::ReadFrom;
    use crate::rows::PassphraseReference;
    use smoozy_security_and_licence::held::Subject;
    use smoozy_state::identities::{Identity, IdentityKind};

    fn sha(body: &str) -> Fingerprint {
        Fingerprint::from_the_printed_form(&format!("SHA256:{body}")).expect("a form")
    }

    fn private(body: &str) -> Reading {
        Reading {
            value: sha(body),
            read_from: ReadFrom::PrivateFile,
        }
    }

    fn a_row(path: &str, fingerprint: &str, name: Option<&str>) -> KeyRow {
        KeyRow::entering(
            Identity::issue(IdentityKind::LocalKey).expect("randomness"),
            PathBuf::from(path),
            Some(private(fingerprint)),
            name.and_then(Name::what_the_user_typed),
            false,
        )
        .expect("a row")
    }

    fn no_blob(_: &KeyRow) -> Option<PublicKeyBlob> {
        None
    }

    fn picked(path: &str, fingerprint: &str) -> Picked {
        Picked {
            path: PathBuf::from(path),
            verdict: Verdict::Usable,
            why_it_would_not_read: None,
            fingerprint: Some(private(fingerprint)),
            blob: None,
            name: None,
        }
    }

    #[test]
    fn a_failed_clause_is_said_plainly_and_nothing_is_added() {
        let mut picked = picked("/k/broken", "one");
        picked.verdict = Verdict::Unusable(Unusable::FilePermissionsTooOpen);
        assert_eq!(
            judge(&picked, &Offered::AFirstPoint, &[], &no_blob),
            Err(Refused::TheClauseThatFailed {
                clause: Unusable::FilePermissionsTooOpen,
                situation: None
            })
        );
    }

    /// **U-3 and F-8: the two refusals must not read alike.** A protected key
    /// with no `.pub` beside it is *unreadable* — U6's set is unchanged — and
    /// the refusal carries which situation produced it, so the user is not sent
    /// hunting for damage in a file that has none.
    #[test]
    fn a_protected_key_with_no_public_half_is_refused_saying_which_situation_it_is() {
        let mut picked = picked("/k/protected", "one");
        picked.verdict = Verdict::Unusable(Unusable::Unreadable);
        picked.why_it_would_not_read =
            Some(WhyItWouldNotRead::ItIsProtectedAndHasNoPublicHalfBeside);
        assert_eq!(
            judge(&picked, &Offered::AFirstPoint, &[], &no_blob),
            Err(Refused::TheClauseThatFailed {
                clause: Unusable::Unreadable,
                situation: Some(WhyItWouldNotRead::ItIsProtectedAndHasNoPublicHalfBeside)
            })
        );
    }

    /// **F-11: a mending offer names a row that is not in the list.** It names
    /// nothing, and is refused rather than answered from whatever sat at that
    /// position — which is what a pair of list positions could not do.
    #[test]
    fn a_mending_offer_naming_a_row_that_is_not_there_is_refused() {
        let rows = vec![a_row("/k/id_rsa", "one", None)];
        let somebody_else = Identity::issue(IdentityKind::LocalKey).expect("randomness");
        assert_eq!(
            judge(
                &picked("/elsewhere/other", "two"),
                &Offered::ANewLocationFor(somebody_else),
                &rows,
                &no_blob
            ),
            Err(Refused::NotThisKey)
        );
    }

    /// **And the row being mended is the one excluded from F4's sweep**, found
    /// by its identity wherever it sits in the list rather than by a position
    /// the caller had to get right twice.
    #[test]
    fn the_row_being_mended_is_excluded_wherever_it_sits_in_the_list() {
        let rows = vec![
            a_row("/k/first", "one", None),
            a_row("/k/second", "two", None),
        ];
        // The second row is mended with its own key: the new location, not
        // F4's *already listed* refusal against itself.
        assert_eq!(
            judge(
                &picked("/elsewhere/second", "two"),
                &Offered::ANewLocationFor(rows[1].identifier.clone()),
                &rows,
                &no_blob
            ),
            Ok(Form::AnUnusableKeysNewLocation)
        );
    }

    #[test]
    fn pointing_at_a_key_already_listed_is_refused_naming_the_existing_entry() {
        let rows = vec![a_row("/k/id_rsa", "one", None)];
        assert_eq!(
            judge(&picked("/elsewhere/copy", "one"), &Offered::AFirstPoint, &rows, &no_blob),
            Err(Refused::ThisKeyIsAlreadyListed {
                as_row: PathBuf::from("/k/id_rsa")
            })
        );
    }

    #[test]
    fn a_first_point_at_a_key_nobody_holds_is_a_key_kept_elsewhere() {
        let rows = vec![a_row("/k/id_rsa", "one", None)];
        assert_eq!(
            judge(&picked("/elsewhere/work", "two"), &Offered::AFirstPoint, &rows, &no_blob),
            Ok(Form::AKeyKeptElsewhere)
        );
    }

    #[test]
    fn a_matching_file_offered_as_a_new_location_is_the_new_location() {
        let rows = vec![a_row("/k/id_rsa", "one", None)];
        assert_eq!(
            judge(
                &picked("/elsewhere/id_rsa", "one"),
                &Offered::ANewLocationFor(rows[0].identifier.clone()),
                &rows,
                &no_blob
            ),
            Ok(Form::AnUnusableKeysNewLocation)
        );
    }

    #[test]
    fn a_non_matching_file_offered_as_a_new_location_is_refused_as_not_this_key() {
        let rows = vec![a_row("/k/id_rsa", "one", None)];
        assert_eq!(
            judge(
                &picked("/elsewhere/other", "two"),
                &Offered::ANewLocationFor(rows[0].identifier.clone()),
                &rows,
                &no_blob
            ),
            Err(Refused::NotThisKey)
        );
    }

    #[test]
    fn a_file_matching_this_rows_own_key_is_never_a_replacement() {
        let rows = vec![a_row("/k/id_rsa", "one", None)];
        assert_eq!(
            judge(
                &picked("/elsewhere/id_rsa", "one"),
                &Offered::AReplacementFor(rows[0].identifier.clone()),
                &rows,
                &no_blob
            ),
            Ok(Form::AnUnusableKeysNewLocation)
        );
    }

    #[test]
    fn a_replacement_whose_key_another_row_holds_is_refused_by_f4() {
        let rows = vec![
            a_row("/k/id_rsa", "one", None),
            a_row("/k/id_ed25519", "two", None),
        ];
        assert_eq!(
            judge(
                &picked("/elsewhere/copy", "two"),
                &Offered::AReplacementFor(rows[0].identifier.clone()),
                &rows,
                &no_blob
            ),
            Err(Refused::ThisKeyIsAlreadyListed {
                as_row: PathBuf::from("/k/id_ed25519")
            })
        );
    }

    #[test]
    fn the_new_location_writes_one_field_and_costs_nothing() {
        let mut row = a_row("/k/id_rsa", "one", Some("work"));
        row.passphrase = Some(PassphraseReference {
            subject: Subject::issued_by_the_local_record("s-1"),
            owing_an_erase: false,
        });
        let before = row.clone();
        take_the_new_location(&mut row, PathBuf::from("/elsewhere/id_rsa"));
        assert_eq!(row.path, PathBuf::from("/elsewhere/id_rsa"));
        assert_eq!(row.fingerprint, before.fingerprint);
        assert_eq!(row.name, before.name);
        assert_eq!(row.passphrase, before.passphrase);
        assert_eq!(row.identifier, before.identifier);
    }

    #[test]
    fn the_replacement_writes_four_fields_and_drops_the_reference() {
        let mut row = a_row("/k/id_rsa", "one", Some("someone@their-mac"));
        row.was_default_at_entry = true;
        row.passphrase = Some(PassphraseReference {
            subject: Subject::issued_by_the_local_record("s-1"),
            owing_an_erase: false,
        });
        let identifier = row.identifier.clone();

        take_the_replacement(
            &mut row,
            PathBuf::from("/k/id_ed25519"),
            Some(private("two")),
            Name::what_the_user_typed("work"),
            "someone@their-mac",
        );

        assert_eq!(row.path, PathBuf::from("/k/id_ed25519"));
        assert_eq!(row.fingerprint, Some(private("two")));
        assert_eq!(row.name.expect("a name").as_str(), "work");
        assert!(!row.was_default_at_entry);
        assert_eq!(row.passphrase, None);
        // The identifier is untouched, which is what strands the servers.
        assert_eq!(row.identifier, identifier);
    }

    #[test]
    fn a_replacement_strands_every_server_that_held_the_old_key() {
        let mut row = a_row("/k/id_rsa", "one", None);
        let identifier = row.identifier.clone();
        assert!(the_server_row_still_names_this_key(
            &row,
            &identifier,
            &sha("one")
        ));
        take_the_replacement(&mut row, PathBuf::from("/k/new"), Some(private("two")), None, "d");
        assert!(!the_server_row_still_names_this_key(
            &row,
            &identifier,
            &sha("one")
        ));
    }

    #[test]
    fn the_cost_names_a_passphrase_only_where_the_row_carries_a_reference() {
        let mut row = a_row("/k/id_rsa", "one", None);
        assert!(
            !WhatAReplacementCosts::of(&row, UsedBy::NoSavedServer).the_remembered_passphrase_goes
        );
        row.passphrase = Some(PassphraseReference {
            subject: Subject::issued_by_the_local_record("s-1"),
            owing_an_erase: false,
        });
        assert!(
            WhatAReplacementCosts::of(&row, UsedBy::NoSavedServer).the_remembered_passphrase_goes
        );
    }

    #[test]
    fn a_dialog_shown_again_re_takes_the_pair_and_never_writes_a_stale_one() {
        let mut row = a_row("/k/id_rsa", "one", None);
        let identifier = row.identifier.clone();
        take_the_replacement(&mut row, PathBuf::from("/k/new"), Some(private("two")), None, "d");
        let rows = vec![row];
        assert_eq!(
            re_take_the_pair(&identifier, &rows),
            Some((identifier.clone(), Some(sha("two"))))
        );
    }

    #[test]
    fn a_dialog_holding_a_key_that_was_removed_finds_nothing_and_the_field_is_empty() {
        let gone = Identity::issue(IdentityKind::LocalKey).expect("randomness");
        assert_eq!(re_take_the_pair(&gone, &[]), None);
    }

    #[test]
    fn a_server_dialog_is_offered_no_unusable_key_and_no_unread_verdict() {
        let drawn = vec![
            (a_row("/k/good", "one", None), Some(Verdict::Usable)),
            (
                a_row("/k/bad", "two", None),
                Some(Verdict::Unusable(Unusable::FileMissing)),
            ),
            (a_row("/k/waiting", "three", None), None),
        ];
        let offerable = offerable_to_a_server_dialog(&drawn);
        assert_eq!(offerable.len(), 1);
        assert_eq!(offerable[0].path, PathBuf::from("/k/good"));
    }

    #[test]
    fn a_cancel_hands_back_nothing_at_all() {
        assert_eq!(WhatReturns::Nothing, WhatReturns::Nothing);
        let row = a_row("/k/good", "one", None);
        assert_eq!(
            WhatReturns::picked(&row),
            WhatReturns::APick {
                identifier: row.identifier.clone(),
                fingerprint: Some(sha("one"))
            }
        );
    }
}
