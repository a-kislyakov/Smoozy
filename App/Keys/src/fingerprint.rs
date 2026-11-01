//! The fingerprint — `Keys` §5.
//!
//! **F3: the fingerprint is the key's identity everywhere in the
//! application** — the row's display line, the saved server's match value
//! (`Servers` P16), the delete warning's join, the second-reference refusal,
//! and the passphrase box's naming of a key whose row has no name. A key's own
//! fingerprint is shown deliberately; a **host** key's never is (`Security and
//! licence` §5.4 — a different thing, not touched here).

/// **F8: the hash is pinned to SHA-256 explicitly, wherever a fingerprint is
/// taken or compared.**
///
/// The printed form is selectable — `FingerprintHash` takes *"md5 and sha256
/// (the default)"* — and the two forms are not equally safe. **In MD5 two
/// distinct keys demonstrably can share one**: RFC 6151, *"The attacks
/// presented in [KLIM2006] can find MD5 collision in about one minute on a
/// standard notebook PC"*. **Relying on the default is not enough, because the
/// default is a setting somebody can change**, so the hash is named at every
/// call — and this is the constant every call names it with.
pub const HASH: &str = "sha256";

/// The prefix the tool prints a SHA-256 fingerprint under. **It is the
/// discriminator this type refuses an MD5 value on**, and not a decoration.
const PRINTED_PREFIX: &str = "SHA256:";

/// One key's fingerprint, in the printed form `ssh-keygen -l` gives it:
/// `SHA256:` plus unpadded base64 (Fact 76, as `Security and licence` §6
/// carries it).
///
/// **An MD5 fingerprint is never treated as an identity** (F8), and the only
/// constructor refuses one — so a value of this type cannot be an MD5 one,
/// whatever a tool was persuaded to print.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Fingerprint(String);

impl Fingerprint {
    /// Takes the fingerprint field out of the tool's own printed line —
    /// `2048 SHA256:… comment (RSA)`.
    ///
    /// **Anything that is not a SHA-256 fingerprint is refused rather than
    /// carried**, which is F8's rule kept by the type rather than by care at
    /// each of the places a fingerprint is compared.
    pub fn from_the_printed_form(printed: &str) -> Option<Fingerprint> {
        let printed = printed.trim();
        let body = printed.strip_prefix(PRINTED_PREFIX)?;
        if body.is_empty() || body.contains(char::is_whitespace) {
            return None;
        }
        Some(Fingerprint(printed.to_string()))
    }

    /// The printed form, which is what a row's display line shows and what a
    /// server row holds beside its key reference (`Servers` D3).
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// **U7a: where a fingerprint came from, because the two sources do not carry
/// the same weight.**
///
/// Fact 76 establishes that the two halves of **one pair** print the same
/// fingerprint. It does not establish that the file at `<path>.pub` is **this**
/// key's public half, and in the only case where the fallback fires nothing
/// can establish it, since that case is defined by the private file's own
/// fingerprint being unreadable.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReadFrom {
    /// The private file's own `-l -f` reading. **This is identity**, and once
    /// one has answered for a row the fallback is not consulted again for it.
    PrivateFile,
    /// The `.pub` beside it, under W5's naming rule. **The user's own file
    /// state** — a dotfile sync, a backup restore, their own hand can put a
    /// stranger's public half there — so it is a value and never evidence of a
    /// swap.
    PublicHalfBeside,
}

/// A fingerprint with the source it was read from, so that U4 clause 4b can
/// refuse to evaluate on a fallback value rather than having to remember to.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Reading {
    pub value: Fingerprint,
    pub read_from: ReadFrom,
}

impl Reading {
    /// **U7a's first rule: a fingerprint read from the public half is recorded
    /// and used only where the private file's own read has never succeeded for
    /// this row.** Given what the row already holds and what has just been
    /// read, this answers what the row keeps.
    ///
    /// So a `.pub` appearing or changing beside a key whose private half reads
    /// is not a source of identity at all.
    pub fn what_the_row_keeps(held: Option<&Reading>, fresh: Reading) -> Reading {
        match held {
            Some(held) if held.read_from == ReadFrom::PrivateFile => {
                if fresh.read_from == ReadFrom::PrivateFile {
                    fresh
                } else {
                    held.clone()
                }
            }
            _ => fresh,
        }
    }

    /// **U7a's second rule, and the two are one rule reached from two
    /// directions**: a disagreement whose only source is the fallback value
    /// raises no erase, so 4b is not evaluated on it.
    pub fn may_be_compared_for_a_swap(&self) -> bool {
        self.read_from == ReadFrom::PrivateFile
    }
}

/// The wire-format public key blob — RFC 4253 §6.6's bytes, which is what the
/// fingerprint is taken over.
///
/// **F5: where both blobs are readable, the refusal compares the blobs beside
/// the fingerprints.** What the comparison guards is **the identity arriving in
/// the wrong form** — an MD5 fingerprint, whose collisions are documented as
/// cheap — rather than a SHA-256 collision, which nobody has produced.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PublicKeyBlob(pub Vec<u8>);

/// What comparing a pointed-at key against a listed row came to — F4 and F5.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SecondReference {
    /// The fingerprints differ. Not this key; the point may go on to whatever
    /// act it was.
    ADifferentKey,
    /// **F4: the same key Smoozy already lists.** Refused, with a plain
    /// message naming the existing entry, and one entry kept.
    TheKeyThisRowAlreadyHolds,
    /// **F5: two different blobs under one fingerprint.** Refused **as two
    /// different keys that Smoozy cannot tell apart**, said in those words,
    /// rather than folded into one entry.
    TwoKeysSmoozyCannotTellApart,
}

/// F4 and F5 together, over one listed row.
///
/// **The comparison is the fingerprint, over every current row, at the moment
/// of the point.** The blob comparison runs **only where both blobs are
/// readable** — both public halves present, or either key unprotected — and
/// where one is not, *"the refusal stands on the fingerprint alone"* and the
/// residual is §13 item 1's.
pub fn compare_against_a_listed_row(
    pointed_at: &Fingerprint,
    pointed_at_blob: Option<&PublicKeyBlob>,
    row_holds: &Fingerprint,
    row_blob: Option<&PublicKeyBlob>,
) -> SecondReference {
    if pointed_at != row_holds {
        return SecondReference::ADifferentKey;
    }
    match (pointed_at_blob, row_blob) {
        (Some(one), Some(other)) if one != other => SecondReference::TwoKeysSmoozyCannotTellApart,
        _ => SecondReference::TheKeyThisRowAlreadyHolds,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sha(body: &str) -> Fingerprint {
        Fingerprint::from_the_printed_form(&format!("SHA256:{body}")).expect("a SHA-256 form")
    }

    #[test]
    fn an_md5_fingerprint_is_never_a_fingerprint_at_all() {
        assert_eq!(
            Fingerprint::from_the_printed_form("MD5:d4:1d:8c:d9:8f:00:b2:04:e9:80"),
            None
        );
        assert_eq!(Fingerprint::from_the_printed_form("SHA256:"), None);
        assert_eq!(Fingerprint::from_the_printed_form("nothing like one"), None);
    }

    #[test]
    fn the_hash_is_named_and_is_not_left_to_a_default() {
        assert_eq!(HASH, "sha256");
    }

    #[test]
    fn a_private_file_reading_is_never_replaced_by_the_public_half_fallback() {
        let private = Reading {
            value: sha("one"),
            read_from: ReadFrom::PrivateFile,
        };
        let fallback = Reading {
            value: sha("two"),
            read_from: ReadFrom::PublicHalfBeside,
        };
        assert_eq!(
            Reading::what_the_row_keeps(Some(&private), fallback),
            private
        );
    }

    #[test]
    fn a_fallback_value_is_replaced_the_moment_the_private_file_answers() {
        let fallback = Reading {
            value: sha("two"),
            read_from: ReadFrom::PublicHalfBeside,
        };
        let private = Reading {
            value: sha("one"),
            read_from: ReadFrom::PrivateFile,
        };
        assert_eq!(
            Reading::what_the_row_keeps(Some(&fallback), private.clone()),
            private
        );
    }

    #[test]
    fn only_a_private_file_reading_may_be_compared_for_a_swap() {
        assert!(Reading {
            value: sha("one"),
            read_from: ReadFrom::PrivateFile
        }
        .may_be_compared_for_a_swap());
        assert!(!Reading {
            value: sha("one"),
            read_from: ReadFrom::PublicHalfBeside
        }
        .may_be_compared_for_a_swap());
    }

    #[test]
    fn a_matching_fingerprint_with_matching_blobs_is_the_key_already_listed() {
        let blob = PublicKeyBlob(vec![1, 2, 3]);
        assert_eq!(
            compare_against_a_listed_row(&sha("one"), Some(&blob), &sha("one"), Some(&blob)),
            SecondReference::TheKeyThisRowAlreadyHolds
        );
    }

    #[test]
    fn two_blobs_under_one_fingerprint_are_two_keys_and_are_never_folded_into_one() {
        assert_eq!(
            compare_against_a_listed_row(
                &sha("one"),
                Some(&PublicKeyBlob(vec![1])),
                &sha("one"),
                Some(&PublicKeyBlob(vec![2]))
            ),
            SecondReference::TwoKeysSmoozyCannotTellApart
        );
    }

    #[test]
    fn where_a_blob_cannot_be_read_the_refusal_stands_on_the_fingerprint_alone() {
        assert_eq!(
            compare_against_a_listed_row(&sha("one"), None, &sha("one"), Some(&PublicKeyBlob(vec![2]))),
            SecondReference::TheKeyThisRowAlreadyHolds
        );
    }

    #[test]
    fn a_different_fingerprint_is_a_different_key() {
        assert_eq!(
            compare_against_a_listed_row(&sha("one"), None, &sha("two"), None),
            SecondReference::ADifferentKey
        );
    }
}
