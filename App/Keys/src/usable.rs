//! *Usable*, composed — `Keys` §4.2 to §4.5.
//!
//! **U10: the check is one function — U4's five clauses evaluated fresh
//! against the row at the moment of the call — and the standalone keys view,
//! the server list draw and the connection attempt all call that one
//! function.** Nothing watches keys in the background; nothing caches a
//! verdict. It is a local read that never reaches a machine (`Servers` P14)
//! and it runs once per key per ask.
//!
//! **Every clause is a local read, promptless, passphrase-free and
//! network-free.**

use std::path::Path;

use crate::container;
use crate::fingerprint::{Fingerprint, ReadFrom, Reading};
use crate::locations;
use crate::rows::{KeyRow, Name};
use crate::types::KeyType;

/// **U6: the unusable reasons are a closed set and each is the clause that
/// failed.**
///
/// **No diagnostic string of the tool's is ever parsed to compose these** —
/// the clause that failed is Smoozy's own reading, which is `Security and
/// licence` §13 rule 5 kept.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Unusable {
    /// Clause 1.
    FileMissing,
    /// Clause 2, **clause 4a**, or a read the tool refuses.
    Unreadable,
    /// Clause 3.
    NotASupportedType,
    /// **Clause 4b**, and only 4b: the swap case.
    NoLongerTheSameKey,
    /// Clause 5.
    FilePermissionsTooOpen,
}

/// What the one check came to.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Verdict {
    Usable,
    Unusable(Unusable),
    /// **U6a: where the tool a reading needs is absent, the key is *not
    /// checked*, and that is a state above U6's set rather than a sixth member
    /// of it.**
    ///
    /// A key whose clauses could not be evaluated **is not an unusable key** —
    /// nothing about it failed, and *unreadable* would be a statement about the
    /// key that is false; the keys are fine and the tool is absent. **Nothing
    /// is handed to a caller in place of a verdict that was never read.**
    ///
    /// The cause is said **once**, above the list, and never per row.
    NotChecked,
}

impl Verdict {
    /// **Only clause 4b raises `Security and licence` §5.2's swap event and
    /// with it the erasure.** 4a failing raises none, and the remembered
    /// passphrase is untouched — a file that cannot be read at this moment is
    /// the direction the spec has already chosen for a file that goes away,
    /// *"A key's file going missing does not cost the passphrase"*.
    pub fn raises_the_swap_event(&self) -> bool {
        matches!(self, Verdict::Unusable(Unusable::NoLongerTheSameKey))
    }
}

/// **Which situation produced an *unreadable* verdict, where the readings knew
/// one — carried beside the verdict and never inside it.**
///
/// **U6's closed set is not widened and no sixth reason is invented**: the
/// verdict stays *unreadable*. This is U6a's own precedent applied — a thing
/// carried beside the closed set rather than a new member of it — and it exists
/// because two different failures were reaching the user under one sentence.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhyItWouldNotRead {
    /// **The key has a passphrase and there is no `.pub` beside it**, so §13
    /// item 6's class is met: U7b's `-y -f` needs the key unlocked and is
    /// refused, and U7's fallback has no file to read. **The file itself reads
    /// perfectly** — what cannot be read is its fingerprint — so a refusal
    /// saying only *unreadable* would send the user hunting for damage in a
    /// healthy file, which `Preparation/Design/Keys.md` forbids in as many
    /// words: *"Two different refusals, and they must not read alike"*.
    ///
    /// **The words, ruled 2026-09-13 at step 10a piece 4** under the Creator's
    /// standing delegation of 2026-08-31, the Design stage being closed. They
    /// are composed at `src/surface/keys.rs` in the desktop crate, where every
    /// word this screen shows is composed, and the reasoning is in that piece's
    /// account and in both decisions records. **The marker is cleared here
    /// because it is closed and not because it was forgotten.**
    ItIsProtectedAndHasNoPublicHalfBeside,
    /// **The file was read and it is not a private key container** — U2's test
    /// failing, which is clause 2 rather than clause 4a.
    ///
    /// **`Preparation/Design/Keys.md` requires the split in as many words:**
    /// *"Not a usable key says what failed, from the architecture U6's own
    /// closed set: **the file could not be read**, or **it is not a private key
    /// Smoozy recognises**, or it is a key of a type Smoozy does not support, or
    /// its permissions let others read it"*, and the drawing gives the two their
    /// own states — `keys-noread` and `keys-bad`. **Both are U6's *unreadable***,
    /// so a screen holding the verdict alone could draw only one of the two, and
    /// the one it drew would send the user hunting for damage in a file that has
    /// none — the exact failure this type was added for.
    ///
    /// **U6's closed set is still not widened**: the verdict stays *unreadable*
    /// and this rides beside it, which is the same shape U6a takes for *not
    /// checked*. Added 2026-09-13 at step 10a piece 4, and it decides nothing —
    /// it carries a distinction the readings had already made and were throwing
    /// away one line later.
    ItIsNotAPrivateKeyContainer,
}

/// The whole of what one check produces: the verdict, and the readings the
/// caller has to do something with.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Checked {
    pub verdict: Verdict,
    /// **Which situation produced an *unreadable* verdict**, where the readings
    /// answered one. Absent for every other verdict and for an *unreadable* one
    /// the readings cannot explain.
    pub why_it_would_not_read: Option<WhyItWouldNotRead>,
    /// What the fingerprint reading answered at this moment, with its source.
    /// **U7a decides what the row keeps of it** — never a fallback value over
    /// a private-file one.
    pub fingerprint_now: Option<Reading>,
    /// The comment field as last read, where this check could read one. R1
    /// refreshes the row's name from it.
    pub name_now: Option<Name>,
    /// **U8's protection reading**, which S14's ask and the availability
    /// question both stand on. Absent where the container could not be read.
    pub protected: Option<bool>,
}

/// What the filesystem answered for the recorded path — U4 clause 1, and the
/// mode half of clause 5.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Presence {
    /// A regular file is present and readable.
    ARegularFileThatOpens,
    /// Nothing is there. Clause 1.
    NotThere,
    /// Something is there and it cannot be read, or it is not a regular file.
    /// Clause 1 fails on *readable*; the reason U6 gives is *unreadable*.
    ThereButNotReadable,
}

/// **U4 clause 5, and the asymmetry between the systems is deliberate.**
///
/// **On macOS and Linux the gate is applied**: the private file grants no
/// access to group or other. Observed: a private key at mode 0644 is refused
/// by the tool with `bad permissions` before the passphrase question is
/// reached. **Smoozy never widens permissions to mend this**; the key shows
/// unusable with the reason, and mending is the user's.
///
/// **On Windows the clause is not applied.** The gate's Windows form is an ACL
/// judgement, not mode bits, and that the client **refuses** a key whose
/// access control is loose is established by nothing read — *"inventing a
/// verdict would manufacture unusable ones"*. §13 item 8 carries what that
/// costs: such a key is caught at connect and reported as a failed connection,
/// which forgets nothing and lies about nothing.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PermissionGate {
    Passes,
    GrantsAccessToGroupOrOther,
    NotAppliedOnThisSystem,
}

/// What a run of `ssh-keygen -l -E sha256 -f <path>` came to.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ToolAnswer {
    /// The printed line, exit 0.
    Printed(String),
    /// The tool ran and refused. **No diagnostic text is read off it** (U6).
    Refused,
    /// **G6: this computer has no generator**, so no reading that runs the
    /// tool can be taken at all.
    NoGenerator,
}

/// The readings behind the clauses — `Keys` §4.3 U7.
///
/// **Three readings serve every clause**: the in-place container header read,
/// the fingerprint read, and the filesystem's own answers for presence,
/// readability and mode. The acting half implements this; the check is written
/// against it so that **the one function of U10 is genuinely one function**.
pub trait Readings {
    /// **G6: whether this computer has a generator at all**, which is one
    /// reading of the installed binary and not a per-key question.
    ///
    /// **It is asked before U4's first clause, and that is the whole point of
    /// it being here.** `Verdict::NotChecked` was reachable only from inside
    /// the fingerprint read, which sits behind clauses 1 and 2 — and those two
    /// need no tool, so on a computer with no `ssh-keygen` a row whose file was
    /// missing showed *file missing* beside rows marked *not checked*.
    /// `Preparation/Design/Keys.md` forbids that flatly: *"no row carries a
    /// message of its own either — not even one saying the file is not there,
    /// which needs no tool to see. The architecture U6a and G6 allow
    /// neither."*
    ///
    /// **It is on this trait rather than being handed in as the run's offered
    /// set**, which keeps `App/Keys/` with no route to a process — the split
    /// this crate is built on — and it does not fight `Desktop app and
    /// delivery` P5, the whole check already running off the drawing thread.
    fn there_is_a_generator(&self) -> bool;

    /// The filesystem's own answers for presence and readability.
    fn presence(&self, path: &Path) -> Presence;

    /// The mode half of clause 5, or the statement that this system does not
    /// apply it.
    fn permission_gate(&self, path: &Path) -> PermissionGate;

    /// The opening bytes of the file, read **in place and transiently** —
    /// `Security and licence` §6's own read. Nothing decrypts and nothing is
    /// retained.
    fn opening_bytes(&self, path: &Path) -> Option<Vec<u8>>;

    /// **U7b: the private half's own fingerprint line**, with the hash named
    /// at the call ([`crate::fingerprint::HASH`]).
    ///
    /// **It is not `ssh-keygen -l -f <private path>`.** Observed on the
    /// Creator's Mac, OpenSSH_10.3p1, 2026-09-03: that invocation answers from
    /// the `.pub` beside the key whenever one exists — it printed a second
    /// key's fingerprint and comment for a private file whose `.pub` had been
    /// replaced. **What reads the private half itself is deriving its public
    /// half from it and fingerprinting that**, which is uninfluenced by
    /// whatever sits beside the file.
    ///
    /// For an encrypted key with no passphrase available it ends at the tool's
    /// own refusal, promptlessly, which is the failure U7's fallback exists
    /// for.
    fn the_private_halfs_own_fingerprint_line(&self, path: &Path) -> ToolAnswer;

    /// **U7's fallback: the `.pub` beside the key, read directly.** This one
    /// is `ssh-keygen -l -f <public path>` and it is a public-half read in the
    /// plainest sense — the file it names is the public half.
    fn the_public_halfs_fingerprint_line(&self, public_half: &Path) -> ToolAnswer;
}

/// **U10's one function.** Evaluated fresh against the row at the moment of the
/// call, in U4's own order.
pub fn check(row: &KeyRow, readings: &dyn Readings) -> Checked {
    let nothing_read = |verdict| Checked {
        verdict,
        why_it_would_not_read: None,
        fingerprint_now: None,
        name_now: None,
        protected: None,
    };

    // **U6a and G6, asked before clause 1 and not after clause 2.** Clauses 1
    // and 2 need no tool, so a computer with no generator would otherwise give
    // *file missing* and *unreadable* to some rows and *not checked* to the
    // rest — a false per-key verdict beside an absent one, on a computer where
    // nothing about any key failed.
    if !readings.there_is_a_generator() {
        return nothing_read(Verdict::NotChecked);
    }

    // Clause 1: a regular file is present and readable at the recorded path.
    match readings.presence(&row.path) {
        Presence::NotThere => return nothing_read(Verdict::Unusable(Unusable::FileMissing)),
        Presence::ThereButNotReadable => {
            return nothing_read(Verdict::Unusable(Unusable::Unreadable))
        }
        Presence::ARegularFileThatOpens => {}
    }

    // Clause 2: its content passes the container test.
    let Some(container) = readings
        .opening_bytes(&row.path)
        .and_then(|bytes| container::read(&bytes))
    else {
        // **Clause 2 failing is carried beside the verdict** — the file was
        // opened and read and it is not a private key container, which is a
        // different thing from a file that would not read at all. Both are
        // *unreadable* under U6 and `Preparation/Design/Keys.md` gives them two
        // sentences that must not read alike.
        return Checked {
            verdict: Verdict::Unusable(Unusable::Unreadable),
            why_it_would_not_read: Some(WhyItWouldNotRead::ItIsNotAPrivateKeyContainer),
            fingerprint_now: None,
            name_now: None,
            protected: None,
        };
    };
    // U8's protection reading is this same read, on the same bytes. **It is an
    // absence rather than a guess where the body could not be made sense of**
    // (U2, F-12): a damaged `openssh-key-v1` file still has the armour line and
    // is still a key, and it fails clause 4a below rather than vanishing from
    // the list.
    let protected = container.protected;

    // Clauses 3 and 4 need a reading that runs the tool, and U7's fallback is
    // part of taking it. **G6's absence takes every one of them down at once**,
    // which is U6a and not a verdict.
    let read = match read_the_fingerprint(&row.path, readings) {
        FingerprintRead::NoGenerator => {
            return Checked {
                verdict: Verdict::NotChecked,
                why_it_would_not_read: None,
                fingerprint_now: None,
                name_now: None,
                protected,
            }
        }
        FingerprintRead::Refused {
            there_is_no_public_half,
        } => {
            // **U-3 and the honest ending: the verdict stays *unreadable* and
            // U6's closed set is unchanged; which situation produced it is
            // carried beside it.** Both halves of the answer are already in
            // hand at this moment — the container read has said *this is a key
            // and it is protected*, and the fallback has just found no `.pub`.
            let why = (protected == Some(true) && there_is_no_public_half)
                .then_some(WhyItWouldNotRead::ItIsProtectedAndHasNoPublicHalfBeside);
            return Checked {
                verdict: Verdict::Unusable(Unusable::Unreadable),
                why_it_would_not_read: why,
                fingerprint_now: None,
                name_now: None,
                protected,
            };
        }
        FingerprintRead::Answered(read) => read,
    };

    let taken = Checked {
        verdict: Verdict::Usable,
        why_it_would_not_read: None,
        fingerprint_now: Some(read.reading.clone()),
        name_now: read.name.clone(),
        protected,
    };
    let with = |verdict| Checked { verdict, ..taken.clone() };

    // Clause 3: its type is in the supported set.
    let Some(_supported) = read.key_type else {
        return with(Verdict::Unusable(Unusable::NotASupportedType));
    };

    // Clause 4b: and it matches the fingerprint the row holds.
    //
    // **A row that has never held a fingerprint is judged on 4a alone**, so 4b
    // is skipped for it and the row reads by 4a, which has already answered
    // above.
    //
    // **U7ab is what the two guards below keep, and it is a bound rather than
    // a mechanism.** A swap is only detectable where both values came from the
    // private half's own read, so **the check reaches keys with no passphrase
    // and reaches no protected key at all** — the fallback value is the `.pub`
    // beside the file, which says nothing about which private key sits there.
    // The bound is stated rather than mended: telling two protected keys apart
    // would need the passphrase, which nothing here has a route to or a grant
    // to ask for.
    if let Some(held) = &row.fingerprint {
        let both_are_the_private_halfs_own =
            held.may_be_compared_for_a_swap() && read.reading.may_be_compared_for_a_swap();
        let both_are_the_fallback = !held.may_be_compared_for_a_swap()
            && !read.reading.may_be_compared_for_a_swap();
        let they_disagree = held.value != read.reading.value;

        if both_are_the_private_halfs_own && they_disagree {
            return with(Verdict::Unusable(Unusable::NoLongerTheSameKey));
        }
        // **U4 clause 4's last paragraph, which had no branch at all.** Where
        // the row's held value came from U7's fallback and today's fallback
        // disagrees, the disagreement is not established to be a swap of the
        // key — it is equally a `.pub` that changed beside a private half that
        // did not. **So 4b is not evaluated on it, the row reads *unreadable*
        // by 4a, and no erase is raised.** Falling through to `Usable` left a
        // key whose only source of identity had changed under it reading as
        // fine.
        if both_are_the_fallback && they_disagree {
            return with(Verdict::Unusable(Unusable::Unreadable));
        }
    }

    // Clause 5: the permission gate.
    match readings.permission_gate(&row.path) {
        PermissionGate::GrantsAccessToGroupOrOther => {
            return with(Verdict::Unusable(Unusable::FilePermissionsTooOpen))
        }
        PermissionGate::Passes | PermissionGate::NotAppliedOnThisSystem => {}
    }

    taken
}

/// What one fingerprint reading, fallback included, came to.
///
/// **It is public because U7's reading is composite and every caller has to
/// take the whole of it.** A caller that took the private half's own line alone
/// would be taking a reading that is refused for every protected key — which is
/// what made a key generated with a passphrase fail its own verification and be
/// unlinked a moment after it was made.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FingerprintRead {
    Answered(WhatTheToolSaid),
    /// **Neither half answered.** §13 item 6's residual, and the two ways of
    /// arriving at it are told apart because the refusal the user meets must
    /// not read alike for both.
    Refused {
        /// **There is no `.pub` beside the key**, so the fallback had no file
        /// to read. With U8's protection reading this is what says the key is
        /// healthy and its fingerprint simply cannot be reached
        /// ([`WhyItWouldNotRead`]).
        there_is_no_public_half: bool,
    },
    NoGenerator,
}

/// The fields of the tool's printed line this module reads. **The comment may
/// hold spaces**, so it is taken as what lies between the fingerprint and the
/// bracketed type rather than by counting words.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WhatTheToolSaid {
    pub reading: Reading,
    pub name: Option<Name>,
    /// `None` where the bracketed type is not one of T1's three — clause 3's
    /// failure, and never a fourth type admitted because the tool printed a
    /// word.
    pub key_type: Option<KeyType>,
}

/// **U7: where the private file's fingerprint read fails without prompting and
/// a `.pub` sits beside it by W5's naming rule, the fingerprint is read from
/// the public half instead** — the two halves of one pair are identical
/// (Fact 76) — **which is the ordinary reading for every key that carries a
/// passphrase, and a rare exception for nothing**.
///
/// **Which keys each half answers for, since the two are not interchangeable.**
/// U7b's `-y -f` derives the public half **from the private key itself**, so it
/// needs the key unlocked: it answers for a key with **no passphrase** and is
/// refused, promptlessly, for **every protected key** — including every key
/// Smoozy itself makes with one. Taking the private half's line alone is
/// therefore not a narrower reading of U7 but a different one, and it is what
/// made a generation with a passphrase fail its own verification.
///
/// The residual where neither half reads is §13 item 6, which is a protected
/// key with no `.pub` beside it.
pub fn read_the_fingerprint(path: &Path, readings: &dyn Readings) -> FingerprintRead {
    match readings.the_private_halfs_own_fingerprint_line(path) {
        ToolAnswer::NoGenerator => return FingerprintRead::NoGenerator,
        ToolAnswer::Printed(line) => {
            if let Some(said) = parse_the_tools_line(&line, ReadFrom::PrivateFile) {
                return FingerprintRead::Answered(said);
            }
        }
        ToolAnswer::Refused => {}
    }

    let beside = locations::companions(path).public_half;
    // **Asked of the filesystem, never read off the tool's refusal**, which U6
    // forbids: *there is no file* and *the tool would not read the file* are
    // told apart by the one reading that can say which, and no diagnostic
    // string is parsed anywhere.
    // Taken only where the fallback has refused, so a reading that answers
    // costs nothing extra.
    let refused = || FingerprintRead::Refused {
        there_is_no_public_half: readings.presence(&beside) == Presence::NotThere,
    };
    match readings.the_public_halfs_fingerprint_line(&beside) {
        ToolAnswer::NoGenerator => FingerprintRead::NoGenerator,
        ToolAnswer::Printed(line) => parse_the_tools_line(&line, ReadFrom::PublicHalfBeside)
            .map(FingerprintRead::Answered)
            .unwrap_or_else(refused),
        ToolAnswer::Refused => refused(),
    }
}

/// Reads the tool's own line — `2048 SHA256:… someone@mac (RSA)`.
///
/// **The type is taken from the bracketed field at the end and the comment
/// from everything between**, which is what lets a comment with spaces,
/// apostrophes and non-Latin characters come back verbatim (observed at G1).
pub fn parse_the_tools_line(line: &str, read_from: ReadFrom) -> Option<WhatTheToolSaid> {
    let line = line.trim();
    let (_bits, rest) = line.split_once(char::is_whitespace)?;
    let rest = rest.trim();
    let (printed, rest) = rest.split_once(char::is_whitespace).unwrap_or((rest, ""));
    let value = Fingerprint::from_the_printed_form(printed)?;

    let rest = rest.trim();
    let (comment, bracketed) = match (rest.rfind('('), rest.ends_with(')')) {
        (Some(at), true) => (&rest[..at], &rest[at..]),
        _ => (rest, ""),
    };

    Some(WhatTheToolSaid {
        reading: Reading { value, read_from },
        name: Name::from_the_comment_field(comment),
        key_type: KeyType::from_the_tools_name(bracketed),
    })
}

/// **U9: *is a passphrase available for this attempt without asking* is
/// answered from the local record and never by reaching the store.**
///
/// **U5 is why this is a separate function and not a clause of the check.** The
/// passphrase is no part of *usable*: a protected key whose passphrase Smoozy
/// is not holding is usable, which is what keeps *existing keys protected with
/// a passphrase are supported the same as ones Smoozy made itself* true
/// without holding anything.
///
/// The answer is: the row's secret-store subject reference for the
/// key-passphrase kind exists **and is not marked as owing an erase**. Reading
/// the store itself could raise a system unlock prompt, which would break
/// `Servers` P1's *answerable without prompting*; the record read cannot.
pub fn a_passphrase_is_available_without_asking(row: &KeyRow) -> bool {
    row.passphrase
        .as_ref()
        .is_some_and(|reference| !reference.owing_an_erase)
}

#[cfg(test)]
mod tests {
    use super::*;
    use smoozy_state::identities::{Identity, IdentityKind};
    use std::collections::HashMap;
    use std::path::PathBuf;

    struct Bench {
        there_is_a_generator: bool,
        presence: Presence,
        /// The paths the filesystem answers *present* for, beyond the recorded
        /// one — which is what says whether a `.pub` sits beside the key.
        also_present: Vec<PathBuf>,
        gate: PermissionGate,
        bytes: Option<Vec<u8>>,
        lines: HashMap<PathBuf, ToolAnswer>,
    }

    impl Default for Bench {
        fn default() -> Self {
            Bench {
                there_is_a_generator: true,
                presence: Presence::ARegularFileThatOpens,
                also_present: Vec::new(),
                gate: PermissionGate::Passes,
                bytes: Some(b"-----BEGIN RSA PRIVATE KEY-----\nMIIE\n".to_vec()),
                lines: HashMap::new(),
            }
        }
    }

    impl Readings for Bench {
        fn there_is_a_generator(&self) -> bool {
            self.there_is_a_generator
        }
        fn presence(&self, path: &Path) -> Presence {
            if path == a_key() {
                return self.presence;
            }
            if self.also_present.iter().any(|there| there == path) {
                Presence::ARegularFileThatOpens
            } else {
                Presence::NotThere
            }
        }
        fn permission_gate(&self, _path: &Path) -> PermissionGate {
            self.gate
        }
        fn opening_bytes(&self, _path: &Path) -> Option<Vec<u8>> {
            self.bytes.clone()
        }
        fn the_private_halfs_own_fingerprint_line(&self, path: &Path) -> ToolAnswer {
            self.lines.get(path).cloned().unwrap_or(ToolAnswer::Refused)
        }
        fn the_public_halfs_fingerprint_line(&self, public_half: &Path) -> ToolAnswer {
            self.lines
                .get(public_half)
                .cloned()
                .unwrap_or(ToolAnswer::Refused)
        }
    }

    fn a_key() -> PathBuf {
        PathBuf::from("/home/someone/.ssh/id_rsa")
    }

    fn a_row(held: Option<Reading>) -> KeyRow {
        KeyRow::entering(
            Identity::issue(IdentityKind::LocalKey).expect("randomness"),
            a_key(),
            held,
            None,
            false,
        )
        .expect("a row")
    }

    fn sha(body: &str) -> Fingerprint {
        Fingerprint::from_the_printed_form(&format!("SHA256:{body}")).expect("a form")
    }

    fn private(body: &str) -> Reading {
        Reading {
            value: sha(body),
            read_from: ReadFrom::PrivateFile,
        }
    }

    fn line(body: &str, key_type: &str) -> ToolAnswer {
        ToolAnswer::Printed(format!("2048 SHA256:{body} someone@mac ({key_type})"))
    }

    #[test]
    fn the_tools_line_is_read_field_by_field_and_a_comment_may_hold_spaces() {
        let said = parse_the_tools_line(
            "256 SHA256:abcdef my key, made at home (ED25519)",
            ReadFrom::PrivateFile,
        )
        .expect("a line");
        assert_eq!(said.reading.value, sha("abcdef"));
        assert_eq!(said.name.expect("a name").as_str(), "my key, made at home");
        assert_eq!(said.key_type, Some(KeyType::Ed25519));
    }

    #[test]
    fn the_tools_words_for_an_absent_comment_leave_the_row_with_no_name() {
        let said = parse_the_tools_line("2048 SHA256:abc no comment (RSA)", ReadFrom::PrivateFile)
            .expect("a line");
        assert_eq!(said.name, None);
    }

    #[test]
    fn a_missing_file_is_clause_one_and_nothing_else() {
        let bench = Bench {
            presence: Presence::NotThere,
            ..Default::default()
        };
        let checked = check(&a_row(Some(private("one"))), &bench);
        assert_eq!(checked.verdict, Verdict::Unusable(Unusable::FileMissing));
        assert!(!checked.verdict.raises_the_swap_event());
    }

    #[test]
    fn a_file_that_fails_the_container_test_is_unreadable_and_not_a_type_verdict() {
        let bench = Bench {
            bytes: Some(b"ssh-ed25519 AAAA someone@mac\n".to_vec()),
            ..Default::default()
        };
        assert_eq!(
            check(&a_row(Some(private("one"))), &bench).verdict,
            Verdict::Unusable(Unusable::Unreadable)
        );
    }

    /// **The two ways of being *unreadable* are told apart, and the drawing
    /// gives them two states.**
    ///
    /// `Preparation/Design/Keys.md` requires it in as many words — *"the file
    /// could not be read, or it is not a private key Smoozy recognises"* — and
    /// the mockup draws `keys-noread` and `keys-bad`. **U6's closed set is
    /// unchanged**: both are *unreadable* and the situation rides beside the
    /// verdict, which is what U6a's own precedent already does for *not
    /// checked*. Added 2026-09-13 at step 10a piece 4, whose screen is the first
    /// caller either sentence has ever had.
    #[test]
    fn the_two_ways_of_being_unreadable_do_not_read_alike() {
        // The file was read and it is not a private key container.
        let read = Bench {
            bytes: Some(b"ssh-ed25519 AAAA someone@mac\n".to_vec()),
            ..Default::default()
        };
        let checked = check(&a_row(Some(private("one"))), &read);
        assert_eq!(checked.verdict, Verdict::Unusable(Unusable::Unreadable));
        assert_eq!(
            checked.why_it_would_not_read,
            Some(WhyItWouldNotRead::ItIsNotAPrivateKeyContainer)
        );

        // The file would not read at all, and the readings know nothing more.
        let would_not_read = Bench {
            presence: Presence::ThereButNotReadable,
            ..Default::default()
        };
        let checked = check(&a_row(Some(private("one"))), &would_not_read);
        assert_eq!(checked.verdict, Verdict::Unusable(Unusable::Unreadable));
        assert_eq!(checked.why_it_would_not_read, None);
    }

    #[test]
    fn a_changed_fingerprint_is_the_swap_case_and_it_is_the_only_one() {
        let mut bench = Bench::default();
        bench.lines.insert(a_key(), line("two", "RSA"));
        let checked = check(&a_row(Some(private("one"))), &bench);
        assert_eq!(
            checked.verdict,
            Verdict::Unusable(Unusable::NoLongerTheSameKey)
        );
        assert!(checked.verdict.raises_the_swap_event());
    }

    #[test]
    fn a_read_that_produced_no_fingerprint_is_unreadable_and_raises_no_erase() {
        let bench = Bench::default(); // every path refuses
        let checked = check(&a_row(Some(private("one"))), &bench);
        assert_eq!(checked.verdict, Verdict::Unusable(Unusable::Unreadable));
        assert!(!checked.verdict.raises_the_swap_event());
    }

    #[test]
    fn a_row_that_never_held_a_fingerprint_is_judged_on_clause_four_a_alone() {
        let mut bench = Bench::default();
        bench.lines.insert(a_key(), line("two", "RSA"));
        let checked = check(&a_row(None), &bench);
        assert_eq!(checked.verdict, Verdict::Usable);
    }

    #[test]
    fn a_fallback_value_is_never_evidence_of_a_swap() {
        let mut bench = Bench::default();
        // The private file refuses; the `.pub` beside it answers differently.
        bench.lines.insert(
            locations::companions(&a_key()).public_half,
            line("two", "RSA"),
        );
        let checked = check(&a_row(Some(private("one"))), &bench);
        assert_eq!(checked.verdict, Verdict::Usable);
        assert!(!checked.verdict.raises_the_swap_event());
        assert_eq!(
            checked.fingerprint_now.expect("a reading").read_from,
            ReadFrom::PublicHalfBeside
        );
    }

    #[test]
    fn an_unsupported_type_is_its_own_reason_and_not_unreadable() {
        let mut bench = Bench::default();
        bench.lines.insert(a_key(), line("one", "DSA"));
        assert_eq!(
            check(&a_row(Some(private("one"))), &bench).verdict,
            Verdict::Unusable(Unusable::NotASupportedType)
        );
    }

    #[test]
    fn a_key_open_to_group_or_other_is_refused_and_smoozy_mends_nothing() {
        let mut bench = Bench {
            gate: PermissionGate::GrantsAccessToGroupOrOther,
            ..Default::default()
        };
        bench.lines.insert(a_key(), line("one", "RSA"));
        assert_eq!(
            check(&a_row(Some(private("one"))), &bench).verdict,
            Verdict::Unusable(Unusable::FilePermissionsTooOpen)
        );
    }

    #[test]
    fn where_the_gate_is_not_applied_the_key_is_not_refused_for_it() {
        let mut bench = Bench {
            gate: PermissionGate::NotAppliedOnThisSystem,
            ..Default::default()
        };
        bench.lines.insert(a_key(), line("one", "RSA"));
        assert_eq!(
            check(&a_row(Some(private("one"))), &bench).verdict,
            Verdict::Usable
        );
    }

    #[test]
    fn with_no_generator_the_key_is_not_checked_rather_than_called_unusable() {
        let mut bench = Bench::default();
        bench.lines.insert(a_key(), ToolAnswer::NoGenerator);
        let checked = check(&a_row(Some(private("one"))), &bench);
        assert_eq!(checked.verdict, Verdict::NotChecked);
        assert!(!checked.verdict.raises_the_swap_event());
        // The protection reading needs no tool and still answered.
        assert_eq!(checked.protected, Some(false));
    }

    /// **U6a and G6, and the case that had no branch: a computer with no
    /// generator, and a key whose file is not there.** Clauses 1 and 2 need no
    /// tool, so this row used to answer *file missing* beside rows that
    /// answered *not checked* — a per-key verdict on a computer where nothing
    /// about any key failed, which `Preparation/Design/Keys.md` forbids in as
    /// many words.
    #[test]
    fn with_no_generator_even_a_missing_file_is_not_checked_rather_than_a_verdict() {
        let bench = Bench {
            there_is_a_generator: false,
            presence: Presence::NotThere,
            ..Default::default()
        };
        let checked = check(&a_row(Some(private("one"))), &bench);
        assert_eq!(checked.verdict, Verdict::NotChecked);
        assert!(!checked.verdict.raises_the_swap_event());
        // **Nothing is handed to a caller in place of a verdict that was never
        // read**, and no reading was taken at all.
        assert_eq!(checked.protected, None);
        assert_eq!(checked.why_it_would_not_read, None);

        // The same for a file that is there and does not pass the container
        // test, which is the other tool-free clause.
        let bench = Bench {
            there_is_a_generator: false,
            bytes: Some(b"ssh-ed25519 AAAA someone@mac\n".to_vec()),
            ..Default::default()
        };
        assert_eq!(
            check(&a_row(Some(private("one"))), &bench).verdict,
            Verdict::NotChecked
        );
    }

    /// **U4 clause 4's last paragraph, which had no branch.** The row's held
    /// value came from the fallback, today's fallback disagrees — so the only
    /// source of this key's identity has changed under it. **4b is not
    /// evaluated, the row reads *unreadable* by 4a, and no erase is raised.**
    /// It used to fall through to `Usable`.
    #[test]
    fn a_fallback_value_disagreeing_with_the_row_is_unreadable_and_raises_no_erase() {
        let mut bench = Bench::default();
        let beside = locations::companions(&a_key()).public_half;
        bench.also_present.push(beside.clone());
        // The private half refuses; the `.pub` answers, and differently from
        // the value the row took from that same `.pub` before.
        bench.lines.insert(beside, line("two", "RSA"));

        let held = Reading {
            value: sha("one"),
            read_from: ReadFrom::PublicHalfBeside,
        };
        let checked = check(&a_row(Some(held)), &bench);
        assert_eq!(checked.verdict, Verdict::Unusable(Unusable::Unreadable));
        assert!(!checked.verdict.raises_the_swap_event());
        // And it is not the protected-with-no-public-half situation: there is
        // a `.pub` and it answered.
        assert_eq!(checked.why_it_would_not_read, None);
    }

    /// The other direction of the same rule, kept: a fallback value that
    /// **agrees** is not a swap and not a failure either.
    #[test]
    fn a_fallback_value_agreeing_with_the_row_leaves_the_key_usable() {
        let mut bench = Bench::default();
        let beside = locations::companions(&a_key()).public_half;
        bench.also_present.push(beside.clone());
        bench.lines.insert(beside, line("one", "RSA"));

        let held = Reading {
            value: sha("one"),
            read_from: ReadFrom::PublicHalfBeside,
        };
        assert_eq!(check(&a_row(Some(held)), &bench).verdict, Verdict::Usable);
    }

    /// **U-3 and §13 item 6: a protected key with no `.pub` beside it.** The
    /// file reads perfectly; what cannot be read is its fingerprint. The
    /// verdict stays *unreadable* — U6's closed set is not widened — and which
    /// situation produced it is carried beside it, so the refusal does not read
    /// as a fault in a healthy file.
    #[test]
    fn a_protected_key_with_no_public_half_says_which_situation_produced_the_refusal() {
        let bench = Bench {
            // An encrypted PEM key: protected, and every tool run refuses.
            bytes: Some(
                b"-----BEGIN RSA PRIVATE KEY-----\nProc-Type: 4,ENCRYPTED\nMIIE\n".to_vec(),
            ),
            ..Default::default()
        };
        let checked = check(&a_row(None), &bench);
        assert_eq!(checked.verdict, Verdict::Unusable(Unusable::Unreadable));
        assert_eq!(checked.protected, Some(true));
        assert_eq!(
            checked.why_it_would_not_read,
            Some(WhyItWouldNotRead::ItIsProtectedAndHasNoPublicHalfBeside)
        );
    }

    /// **And the two refusals do not read alike.** A `.pub` that is there and
    /// will not read is a different thing from one that is not there, and only
    /// the second is the situation above.
    #[test]
    fn a_public_half_that_is_there_and_refuses_is_not_that_situation() {
        let mut bench = Bench {
            bytes: Some(
                b"-----BEGIN RSA PRIVATE KEY-----\nProc-Type: 4,ENCRYPTED\nMIIE\n".to_vec(),
            ),
            ..Default::default()
        };
        let beside = locations::companions(&a_key()).public_half;
        bench.also_present.push(beside);
        let checked = check(&a_row(None), &bench);
        assert_eq!(checked.verdict, Verdict::Unusable(Unusable::Unreadable));
        assert_eq!(checked.why_it_would_not_read, None);
    }

    /// **An unprotected key with no `.pub` is not that situation either** — its
    /// own private half reads, so nothing about it is refused.
    #[test]
    fn an_unprotected_key_needs_no_public_half_at_all() {
        let mut bench = Bench::default();
        bench.lines.insert(a_key(), line("one", "RSA"));
        let checked = check(&a_row(Some(private("one"))), &bench);
        assert_eq!(checked.verdict, Verdict::Usable);
        assert_eq!(checked.why_it_would_not_read, None);
    }

    /// **F-12: a damaged private key file in the usual place is a key, and it
    /// is listed as unusable with the reason.**
    ///
    /// U2 makes the armour line the discriminator and nothing else. Turning a
    /// body the reader could not make sense of into *this is not a key* kept
    /// such a file — a copy that failed part-way — out of the list entirely,
    /// where U1 and U3 want everything found there, usable or not. It fails
    /// clause 4a instead.
    #[test]
    fn a_damaged_container_is_listed_as_unusable_rather_than_not_being_a_key_at_all() {
        let bench = Bench {
            bytes: Some(b"-----BEGIN OPENSSH PRIVATE KEY-----\n!!!not base64!!!\n".to_vec()),
            ..Default::default()
        };
        let checked = check(&a_row(Some(private("one"))), &bench);
        assert_eq!(checked.verdict, Verdict::Unusable(Unusable::Unreadable));
        // **The protection answer is absent rather than guessed**, and the file
        // is still a key.
        assert_eq!(checked.protected, None);
        assert!(!checked.verdict.raises_the_swap_event());
    }

    #[test]
    fn availability_reads_the_row_and_a_mark_turns_it_to_no_at_that_instant() {
        use crate::rows::PassphraseReference;
        use smoozy_security_and_licence::held::Subject;

        let mut row = a_row(Some(private("one")));
        assert!(!a_passphrase_is_available_without_asking(&row));

        row.passphrase = Some(PassphraseReference {
            subject: Subject::issued_by_the_local_record("s-1"),
            owing_an_erase: false,
        });
        assert!(a_passphrase_is_available_without_asking(&row));

        row.passphrase.as_mut().expect("a reference").owing_an_erase = true;
        assert!(!a_passphrase_is_available_without_asking(&row));
    }
}
