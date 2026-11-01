//! What Keys remembers between runs — `Keys` §10.
//!
//! **R2: this module keeps no durable store of its own beyond the key row.**
//! No cache of verdicts, no fingerprint index outside the rows, no file of any
//! kind in the key location beyond the key pairs the user asked for. The one
//! durable thing beside the row is owed by another document's rule rather than
//! kept by this one — `State, identity and recovery` §9.4's outcome record,
//! which [`crate::units`] declares and which **carries no value of the
//! key's**.

use smoozy_security_and_licence::held::{HeldSecret, StoredKind, Subject};
use smoozy_state::identities::{Identity, IdentityKind};

use crate::fingerprint::Reading;

/// **R1: one key row per key, in the local record, user-held** (`State,
/// identity and recovery` §1.1, §11.1).
///
/// **Derived, never stored** and so absent from this type: the
/// managed-or-borrowed kind ([`crate::ownership`]), the usable verdict
/// ([`crate::usable`]), and `Servers` P13's needing-a-new-key mark, which is
/// that document's from its side.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KeyRow {
    /// The **local key identifier**, issued by the local record when a key is
    /// added (`State, identity and recovery` §2.2). **A replacement keeps
    /// it** (F10), which is what keeps the row and the servers' reference to
    /// it.
    pub identifier: Identity,
    /// **L3: the canonical path of the file that actually backs the key**,
    /// resolved at the moment the key entered. Every later clause composes
    /// from this one path — the companion names, delete's file set and its
    /// confirmation, the copy read, and the unlinks with their re-check by
    /// path.
    pub path: std::path::PathBuf,
    /// The fingerprint and where it was read from. **A row that has never held
    /// one is `Keys` §13 item 6's case — a protected key with no readable
    /// `.pub` beside it**, which is what U7b widened that class to from the
    /// legacy-PEM rarity this sentence named until 2026-09-03. U4 judges such
    /// a row on clause 4a alone.
    pub fingerprint: Option<Reading>,
    /// **The name as last read** — at entry, and refreshed from the `.pub` at a
    /// check where one is readable, since an encrypted private file does not
    /// yield its comment (Fact 35).
    pub name: Option<Name>,
    /// **N1's recorded bit: this key's name equalled the default when it
    /// entered.** Recorded at entry and never recomputed at the question.
    ///
    /// **N3: there is no absent form and none is invented.** A row that
    /// entered before the rule existed arises in no planned state, so the
    /// field is a plain bit rather than one with a third *unknown* value that
    /// every reader would then have to guess at.
    pub was_default_at_entry: bool,
    /// **The secret-store subject reference, held with its erase mark where one
    /// is owed** (`Security and licence` §5.1). This is the whole of what U9's
    /// availability answer reads.
    pub passphrase: Option<PassphraseReference>,
}

impl KeyRow {
    /// A row as it is written at entry — generated, discovered, or pointed at.
    ///
    /// **The identifier's kind is named, so a comparison against a server
    /// identity or a run identity cannot compile** (`State, identity and
    /// recovery` §2.1 rule 2's opacity kept by the type).
    pub fn entering(
        identifier: Identity,
        path: std::path::PathBuf,
        fingerprint: Option<Reading>,
        name: Option<Name>,
        was_default_at_entry: bool,
    ) -> Option<KeyRow> {
        if identifier.kind() != IdentityKind::LocalKey {
            return None;
        }
        Some(KeyRow {
            identifier,
            path,
            fingerprint,
            name,
            was_default_at_entry,
            passphrase: None,
        })
    }

    /// **S1's keying: one store item per key — kind *key passphrase*, subject
    /// the secret-store subject issued by the local record against the local
    /// key identifier.**
    ///
    /// The passphrase therefore belongs to the key and not to where its file
    /// sits, by construction: **the path appears nowhere in the keying**, so a
    /// file going missing costs nothing and a file coming back somewhere else
    /// finds the passphrase still held.
    pub fn store_item(&self) -> Option<HeldSecret> {
        self.passphrase.as_ref().map(|reference| {
            HeldSecret::new(StoredKind::KeyPassphrase, reference.subject.clone())
        })
    }
}

/// **S2 and U9's marker.** S2 is what says the marker *is* this reference:
/// there is no second record of whether a passphrase is held.
///
/// **U9a's reading is what makes the marker trustworthy, and where the mark
/// lives was corrected on 2026-09-03.** The mark is not a column of this row —
/// `Security and licence` E3a holds it in a durable register of its own, keyed
/// by kind and subject, which outlives any row. U9's answer is this reference
/// taken against that register, and it still never reaches the store.
///
/// §5.1's three writes are mark the subject, erase the item, and remove the
/// row's reference **and the mark together, never the mark alone**. A sweep
/// that erased the item and cleared only the mark would leave a reference with
/// no mark behind it, U9 would answer *yes* for an item that is gone, and an
/// automatic attempt would be made where `Servers` P3 withholds one — which is
/// why the two live in one value here rather than in two fields that could
/// part company.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PassphraseReference {
    pub subject: Subject,
    /// Write 1 of §5.1 has landed and the erase is owed. **The availability
    /// answer is no from that instant** (F10's second bullet), so no attempt
    /// taken between the two writes is told a passphrase is available.
    pub owing_an_erase: bool,
}

/// **R1a: the name is the key's comment**, and the one reading that surfaces it
/// is the comment field of `ssh-keygen -l -f`'s output line — the same
/// invocation U7 already makes for the fingerprint, read from the `.pub` where
/// the private file does not yield it. At a generation the value is instead
/// the comment the act itself passed to `-C`.
///
/// **A row with no name is ordinary, not broken.** It is told apart from every
/// other row by its fingerprint (F3), the passphrase box names it by that
/// fingerprint (S15), and it shares a name with nobody (N1a).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Name(String);

/// What the tool prints in the comment field when there is none. **It is a
/// diagnostic of the tool's, not a key's name**, and nothing composes a name
/// out of it — U6's rule about reasons applied to the one other field a tool's
/// line could reach.
const THE_TOOLS_WORDS_FOR_NO_COMMENT: &str = "no comment";

impl Name {
    /// Reads the comment field of the tool's line into a name.
    ///
    /// **A comment that is not recoverable is recorded as absent**, and so is
    /// an empty one: `G1` passes what the user typed and *"the user can type
    /// anything"*, including nothing, so a row entered that way holds no name
    /// rather than a name that is the empty string.
    pub fn from_the_comment_field(field: &str) -> Option<Name> {
        let field = field.trim();
        if field.is_empty() || field == THE_TOOLS_WORDS_FOR_NO_COMMENT {
            return None;
        }
        Some(Name(field.to_string()))
    }

    /// The comment the make-key act passed to `-C`, which is what the user
    /// typed. **The same absence rule applies**, so one rule covers both ways
    /// of having none.
    pub fn what_the_user_typed(typed: &str) -> Option<Name> {
        let typed = typed.trim();
        if typed.is_empty() {
            return None;
        }
        Some(Name(typed.to_string()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fingerprint::{Fingerprint, ReadFrom};
    use std::path::PathBuf;

    fn an_identifier(kind: IdentityKind) -> Identity {
        Identity::issue(kind).expect("randomness at the desk")
    }

    fn a_reading() -> Reading {
        Reading {
            value: Fingerprint::from_the_printed_form("SHA256:abc").expect("a form"),
            read_from: ReadFrom::PrivateFile,
        }
    }

    #[test]
    fn a_row_takes_only_a_local_key_identifier() {
        assert!(KeyRow::entering(
            an_identifier(IdentityKind::LocalServer),
            PathBuf::from("/k"),
            None,
            None,
            false
        )
        .is_none());
        assert!(KeyRow::entering(
            an_identifier(IdentityKind::LocalKey),
            PathBuf::from("/k"),
            Some(a_reading()),
            None,
            false
        )
        .is_some());
    }

    #[test]
    fn a_row_holds_no_store_item_until_a_passphrase_was_stored() {
        let row = KeyRow::entering(
            an_identifier(IdentityKind::LocalKey),
            PathBuf::from("/k"),
            Some(a_reading()),
            None,
            false,
        )
        .expect("a row");
        assert_eq!(row.store_item(), None);
    }

    #[test]
    fn the_store_item_is_keyed_by_the_subject_and_never_by_the_path() {
        let mut row = KeyRow::entering(
            an_identifier(IdentityKind::LocalKey),
            PathBuf::from("/k"),
            Some(a_reading()),
            None,
            false,
        )
        .expect("a row");
        row.passphrase = Some(PassphraseReference {
            subject: Subject::issued_by_the_local_record("s-1"),
            owing_an_erase: false,
        });
        let item = row.store_item().expect("an item");
        assert_eq!(item.kind, StoredKind::KeyPassphrase);
        assert_eq!(item.subject.as_str(), "s-1");

        // The same item, whatever the file does.
        row.path = PathBuf::from("/somewhere/else");
        assert_eq!(row.store_item().expect("an item"), item);
    }

    #[test]
    fn the_tools_own_words_for_an_absent_comment_are_never_a_name() {
        assert_eq!(Name::from_the_comment_field("no comment"), None);
        assert_eq!(Name::from_the_comment_field("   "), None);
        assert_eq!(
            Name::from_the_comment_field("someone@their-mac")
                .expect("a name")
                .as_str(),
            "someone@their-mac"
        );
    }

    #[test]
    fn a_comment_the_user_left_empty_is_an_absent_name_and_not_an_empty_one() {
        assert_eq!(Name::what_the_user_typed(""), None);
        assert_eq!(Name::what_the_user_typed("  \t "), None);
    }
}
