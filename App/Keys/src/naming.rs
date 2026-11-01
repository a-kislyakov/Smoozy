//! The filename scheme, the pre-filled name, and the later reading of *is this
//! name the default one* — `Keys` §6.4, §6.6 and §7.
//!
//! **Markers used in this file that belong to another document, each with the
//! document it belongs to.** `Desktop app and delivery` I2, I3, I5.

use std::path::{Path, PathBuf};

use crate::fingerprint::Fingerprint;
use crate::locations;
use crate::rows::{KeyRow, Name};
use crate::types::KeyType;

/// **G7: the file is the name the user typed, made safe as a file name**, in
/// the usual key location: *hyperstack* gives `hyperstack` (and, beside it,
/// `hyperstack.pub`). The spec, Creator-stated 2026-10-08, sixth notes, note 6.
///
/// Any character a file name cannot hold on a supported system — `/ \ : * ?
/// " < > |` and the control characters — becomes `_`, and spaces and dots are
/// trimmed from both ends. **An empty result, or a name Windows reserves for a
/// device** (`CON`, `NUL`, `COM1` and the like), **falls back to
/// `smoozy_<type>`** — `smoozy_ed25519`, `smoozy_ecdsa`, `smoozy_rsa`.
///
/// The counter of G8 stays positional, not an identifier (`State, identity and
/// recovery` §2.1 rule 7 kept): the first is the bare name, the second
/// `<name>_2`. *(Until 2026-10-08 the name was always `smoozy_<type>`.)*
pub fn stem(typed: &str, kind: KeyType, counter: u32) -> String {
    let base = the_typed_name_made_safe(typed)
        .unwrap_or_else(|| format!("smoozy_{}", kind.as_str()));
    match counter {
        1 => base,
        n => format!("{base}_{n}"),
    }
}

/// G7's making-safe, or nothing where it leaves no usable name.
fn the_typed_name_made_safe(typed: &str) -> Option<String> {
    let replaced: String = typed
        .trim()
        .chars()
        .map(|c| match c {
            '/' | '\\' | ':' | '*' | '?' | '"' | '<' | '>' | '|' => '_',
            c if c.is_control() => '_',
            c => c,
        })
        .collect();
    let trimmed = replaced.trim_matches(|c| c == ' ' || c == '.');
    if trimmed.is_empty() || windows_reserves(trimmed) {
        return None;
    }
    Some(trimmed.to_string())
}

/// **A name Windows reserves for a device**, in any case — and Windows reads
/// the part before the first dot, so `con.pub` is reserved too, which is why
/// the whole stem is refused rather than only an exact match.
fn windows_reserves(name: &str) -> bool {
    let before_the_dot = name.split('.').next().unwrap_or(name).trim_end_matches(' ');
    let upper = before_the_dot.to_ascii_uppercase();
    matches!(upper.as_str(), "CON" | "PRN" | "AUX" | "NUL")
        || ((upper.starts_with("COM") || upper.starts_with("LPT"))
            && upper.len() == 4
            && matches!(upper.as_bytes()[3], b'1'..=b'9'))
}

/// **G8: a collision is the ordinary case and the outcome taken is the
/// whole-app rule's second: append a non-colliding name.**
///
/// A second key with the same name is `<name>_2`, then `_3`, **the first
/// integer whose three names — `<name>`, `<name>.pub`, `<name>-cert.pub` — are
/// all free in the location**, checked immediately before the invocation.
/// Appending rather than prompting puts nothing new in front of the user.
///
/// **The window between the check and the tool's write is not defended by the
/// check, and the backstop is the tool's own refusal**: observed,
/// `ssh-keygen` handed a `-f` naming an existing file with no terminal input
/// answers *"already exists. Overwrite (y/n)?"*, exits 1, and **changes
/// nothing**. A generation failing that way re-runs this composition once with
/// the next free name.
pub fn compose_a_free_name(
    key_location: &Path,
    typed: &str,
    kind: KeyType,
    taken: &dyn Fn(&Path) -> bool,
) -> PathBuf {
    let mut counter = 1;
    loop {
        let candidate = key_location.join(stem(typed, kind, counter));
        let beside = locations::companions(&candidate);
        let free = !taken(&candidate) && !taken(&beside.public_half) && !taken(&beside.certificate);
        if free {
            return candidate;
        }
        counter += 1;
    }
}

/// **G11: the pre-fill is `<login account name>@<computer name>`**, both read
/// through `Desktop app and delivery`'s application-identity adapter (I2, I3)
/// at the moment the dialog opens, and nowhere else.
///
/// Adopting OpenSSH's documented `user@host` default as the pre-fill is
/// Smoozy's own choice. **The name is a comment and never an identity;
/// nothing is decided by it** (`Desktop app and delivery` I4 and I5 kept),
/// and the user can type anything
/// over it.
pub fn default_name(login_account_name: &str, computer_name: &str) -> String {
    format!("{login_account_name}@{computer_name}")
}

/// **N1: the reading taken is recorded-at-entry, not computed-at-question.**
///
/// When a key enters Smoozy — generated, discovered, or pointed at — Keys
/// computes G11's default string **at that moment** and records one bit on the
/// row: *this key's name equalled the default when it entered*.
///
/// **N2: why this reading and not the other.** The spec's quiet case is *two
/// ordinary keys made on one machine*, and the recorded bit is precisely *this
/// name was this machine's default when the key appeared here*, so it survives
/// the computer being renamed. **Computed-at-question inverts the spec's
/// ruling on every renamed computer** — every older default-named pair would
/// read as a clash for ever, because the old default never becomes current
/// again.
pub fn was_default_at_entry(name: Option<&Name>, default_now: &str) -> bool {
    name.is_some_and(|name| name.as_str() == default_now)
}

/// Whether the spec's duplicate-name warning fires between a key entering and
/// one already listed.
///
/// The spec warns where a new or newly referenced key shares a name with a
/// listed key under a different fingerprint, **unless the shared name is the
/// default one** — which N1 reads off the recorded bit on **both** rows: the
/// warning is suppressed when both carry it, and fires otherwise.
///
/// **N1a: two rows carrying no name do not share a name**, so the warning does
/// not fire between them. R1a records an unrecoverable or empty comment as
/// **absent** rather than as a value, and absence is not a value two rows can
/// hold in common.
pub fn the_duplicate_name_warning_fires(
    entering_name: Option<&Name>,
    entering_was_default: bool,
    entering_fingerprint: Option<&Fingerprint>,
    listed: &KeyRow,
) -> bool {
    let (Some(entering_name), Some(listed_name)) = (entering_name, listed.name.as_ref()) else {
        return false;
    };
    if entering_name != listed_name {
        return false;
    }
    // Under one fingerprint it is one key, and F4's refusal has already dealt
    // with it — the warning is about *two different keys under one name*.
    let same_key = match (entering_fingerprint, listed.fingerprint.as_ref()) {
        (Some(entering), Some(listed)) => *entering == listed.value,
        _ => false,
    };
    if same_key {
        return false;
    }
    !(entering_was_default && listed.was_default_at_entry)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fingerprint::{ReadFrom, Reading};
    use smoozy_state::identities::{Identity, IdentityKind};
    use std::collections::HashSet;

    fn sha(body: &str) -> Fingerprint {
        Fingerprint::from_the_printed_form(&format!("SHA256:{body}")).expect("a form")
    }

    fn a_row(name: Option<&str>, was_default: bool, fingerprint: &str) -> KeyRow {
        KeyRow::entering(
            Identity::issue(IdentityKind::LocalKey).expect("randomness"),
            PathBuf::from("/Users/someone/.ssh/id_ed25519"),
            Some(Reading {
                value: sha(fingerprint),
                read_from: ReadFrom::PrivateFile,
            }),
            name.and_then(Name::what_the_user_typed),
            was_default,
        )
        .expect("a row")
    }

    #[test]
    fn the_file_is_the_name_the_user_typed() {
        assert_eq!(stem("hyperstack", KeyType::Ed25519, 1), "hyperstack");
        assert_eq!(stem("hyperstack", KeyType::Ed25519, 2), "hyperstack_2");
        assert_eq!(stem("someone@their-mac", KeyType::Rsa, 1), "someone@their-mac");
        assert_eq!(stem("my key", KeyType::Rsa, 1), "my key");
    }

    #[test]
    fn a_character_no_file_name_can_hold_becomes_an_underscore() {
        assert_eq!(stem("a/b:c", KeyType::Ed25519, 1), "a_b_c");
        assert_eq!(stem("a\\b*c?d\"e<f>g|h", KeyType::Ed25519, 1), "a_b_c_d_e_f_g_h");
        assert_eq!(stem("tab\there", KeyType::Ed25519, 1), "tab_here");
    }

    #[test]
    fn spaces_and_dots_are_trimmed_from_both_ends() {
        assert_eq!(stem("  .work. ", KeyType::Ed25519, 1), "work");
        assert_eq!(stem("my.key", KeyType::Ed25519, 1), "my.key");
    }

    #[test]
    fn an_empty_result_falls_back_to_the_type() {
        assert_eq!(stem("", KeyType::Ed25519, 1), "smoozy_ed25519");
        assert_eq!(stem("  ", KeyType::Rsa, 2), "smoozy_rsa_2");
        assert_eq!(stem("...", KeyType::Ecdsa, 3), "smoozy_ecdsa_3");
    }

    #[test]
    fn a_name_windows_reserves_falls_back_to_the_type() {
        for reserved in ["CON", "con", "Nul", "PRN", "AUX", "COM1", "lpt9", "con.key"] {
            assert_eq!(stem(reserved, KeyType::Ed25519, 1), "smoozy_ed25519", "{reserved}");
        }
        // Near misses are ordinary names.
        assert_eq!(stem("CONSOLE", KeyType::Ed25519, 1), "CONSOLE");
        assert_eq!(stem("COM0", KeyType::Ed25519, 1), "COM0");
        assert_eq!(stem("COM10", KeyType::Ed25519, 1), "COM10");
    }

    #[test]
    fn the_composition_steps_past_a_name_whose_certificate_alone_is_taken() {
        let location = Path::new("/Users/someone/.ssh");
        let taken: HashSet<PathBuf> = ["/Users/someone/.ssh/hyperstack-cert.pub"]
            .iter()
            .map(PathBuf::from)
            .collect();
        let composed = compose_a_free_name(location, "hyperstack", KeyType::Ed25519, &|path| {
            taken.contains(path)
        });
        assert_eq!(composed, PathBuf::from("/Users/someone/.ssh/hyperstack_2"));
    }

    #[test]
    fn the_composition_steps_past_a_name_whose_public_half_alone_is_taken() {
        let location = Path::new("/k");
        let taken: HashSet<PathBuf> = ["/k/hyperstack.pub", "/k/hyperstack_2"]
            .iter()
            .map(PathBuf::from)
            .collect();
        let composed =
            compose_a_free_name(location, "hyperstack", KeyType::Ed25519, &|path| taken.contains(path));
        assert_eq!(composed, PathBuf::from("/k/hyperstack_3"));
    }

    #[test]
    fn nothing_free_is_stepped_past() {
        let composed =
            compose_a_free_name(Path::new("/k"), "", KeyType::Rsa, &|_| false);
        assert_eq!(composed, PathBuf::from("/k/smoozy_rsa"));
    }

    #[test]
    fn the_pre_fill_is_the_two_adapter_readings_and_nothing_else() {
        assert_eq!(default_name("someone", "their-mac"), "someone@their-mac");
    }

    #[test]
    fn the_bit_is_taken_against_the_default_at_the_moment_of_entry() {
        let name = Name::what_the_user_typed("someone@their-mac");
        assert!(was_default_at_entry(name.as_ref(), "someone@their-mac"));
        assert!(!was_default_at_entry(name.as_ref(), "someone@renamed-mac"));
        assert!(!was_default_at_entry(None, "someone@their-mac"));
    }

    #[test]
    fn two_ordinary_default_named_keys_draw_no_warning_even_after_a_rename() {
        let listed = a_row(Some("someone@their-mac"), true, "one");
        let entering = Name::what_the_user_typed("someone@their-mac");
        assert!(!the_duplicate_name_warning_fires(
            entering.as_ref(),
            true,
            Some(&sha("two")),
            &listed
        ));
    }

    #[test]
    fn two_different_keys_sharing_a_name_that_is_not_the_default_draw_one() {
        let listed = a_row(Some("work"), false, "one");
        let entering = Name::what_the_user_typed("work");
        assert!(the_duplicate_name_warning_fires(
            entering.as_ref(),
            false,
            Some(&sha("two")),
            &listed
        ));
    }

    #[test]
    fn two_rows_with_no_name_do_not_share_one() {
        let listed = a_row(None, false, "one");
        assert!(!the_duplicate_name_warning_fires(
            None,
            false,
            Some(&sha("two")),
            &listed
        ));
    }

    #[test]
    fn one_key_under_one_fingerprint_is_not_two_keys_under_one_name() {
        let listed = a_row(Some("work"), false, "one");
        let entering = Name::what_the_user_typed("work");
        assert!(!the_duplicate_name_warning_fires(
            entering.as_ref(),
            false,
            Some(&sha("one")),
            &listed
        ));
    }
}
