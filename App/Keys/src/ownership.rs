//! Ownership, the file set, and the *Used by* line — `Keys` §9.1 to §9.4, §9.8.
//!
//! **W2: the derivation is computed when the controls are drawn and when the
//! act runs, never stored** — the same shape `Servers` P13's mark takes, so
//! there is nothing to go stale and no recorded kind to disagree with the
//! world. The Delete-against-Remove control a row shows is this derivation's
//! output at that draw, and the act re-derives at its own moment.

use std::path::{Path, PathBuf};

use crate::fingerprint::Fingerprint;
use crate::locations;
use crate::rows::KeyRow;

/// **W1: a key is managed when the canonical form of its recorded path has the
/// canonical form of the usual key location as its immediate parent
/// directory; otherwise it is borrowed.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Ownership {
    /// Delete destroys files. **W3: location decides, not the route in** — a
    /// file the user pointed at that canonically lies in the usual key
    /// location is managed, the spec's rule being *whatever sits in the
    /// operating system's usual key location*, *regardless of who created
    /// them*.
    Managed,
    /// Remove from Smoozy touches no file, ever. **W4: a symbolic link inside
    /// the location pointing out of it is borrowed** — canonical resolution
    /// lands the file outside, so delete never follows a link in the managed
    /// place out to a file that only looks managed.
    Borrowed,
}

/// The one path reading this derivation needs, and it is the adapter's.
///
/// **W1a: what the comparison does about a case-only difference is asked of
/// the filesystem and never modelled here.** Both readings a builder could
/// take on their own are wrong in one direction: a byte-for-byte comparison
/// makes a key genuinely sitting in the location read **borrowed**, against the
/// spec's *regardless of who created them*; a case-folding comparison makes a
/// borrowed key on a case-sensitive volume read **managed**, and delete
/// destroys a file Smoozy promised only to borrow — Row 251's unrecoverable
/// branch.
///
/// **So neither is chosen. Both sides are read back as the receiving
/// filesystem stores them**, through the reading `Desktop app and delivery`
/// F10a's third bullet already owns, and the comparison is made over those two
/// readings. It is a property asked of the volume rather than a rule about a
/// platform.
///
/// **Labelled, because the ground is missing rather than thin**: whether any of
/// the three platforms' canonical resolution answers a case-only difference at
/// all is established by nothing this project has read, and nothing here
/// guesses in either direction.
pub trait CanonicalPaths {
    /// Symbolic links followed, `.` and `..` collapsed, and the result read
    /// back as the filesystem stores it. `None` where the path does not
    /// resolve.
    fn canonical(&self, path: &Path) -> Option<PathBuf>;
}

/// W1, W3 and W4 in one derivation.
///
/// **Where the recorded path does not resolve, the parent directory is
/// canonicalised instead**, and the answer stays borrowed only where the parent
/// will not resolve either.
///
/// **Why, and it overturns a recorded build decision.** Canonical resolution
/// needs the file to be there, so a **managed** key whose files have gone away
/// resolved to nothing and derived as borrowed: the row drew *Remove* where
/// W9's own words are *"A delete of a key whose files are already missing
/// removes nothing, says the files were already gone, and still runs the
/// erasure unit — the row, the passphrase and the marks all go, which is how a
/// dead managed row is cleared"*. **That branch was unreachable**, and §12's
/// diary line recorded *a referenced key removed* for a key Smoozy managed. The
/// earlier decision reasoned that the plan did not name the case; it does, in
/// as many words, and that is the new evidence which reopened it.
///
/// **The caution the earlier decision was protecting is kept whole.** Comparing
/// the **parent** cannot destroy a borrowed file, because there is no file to
/// destroy — this branch only fires where the recorded path resolves to
/// nothing. Row 251's unrecoverable branch is not reached in either direction.
///
/// **It needs no reading nobody has taken**: the location itself is a directory
/// and is already canonicalised through this same adapter call on every
/// derivation, so this uses a call the code already makes and already relies
/// on.
pub fn ownership(
    recorded_path: &Path,
    user_home: &Path,
    paths: &dyn CanonicalPaths,
) -> Ownership {
    let location = locations::usual_key_location(user_home);
    let Some(location) = paths.canonical(&location) else {
        // No location resolves, so nothing can sit in it.
        return Ownership::Borrowed;
    };

    // *Immediate parent*: a file in a subdirectory of the location is not
    // *sitting in* it.
    if let Some(key) = paths.canonical(recorded_path) {
        return match key.parent() {
            Some(parent) if parent == location => Ownership::Managed,
            _ => Ownership::Borrowed,
        };
    }

    // **The file is gone.** The row still names where it sat, and W9 wants that
    // row cleared by a delete rather than refused one. **The parent is asked
    // the same question the file would have been**, and where the parent will
    // not resolve either the answer stays borrowed — the safe direction, and
    // the one the earlier decision took for every case.
    match recorded_path.parent().and_then(|at| paths.canonical(at)) {
        Some(parent) if parent == location => Ownership::Managed,
        _ => Ownership::Borrowed,
    }
}

/// **W6: delete's file set is exactly the members of `{path, path.pub,
/// path-cert.pub}` that exist at the moment of the act, and the confirmation
/// names exactly those, by full path.**
///
/// **Nothing wider** — no backups, no similarly-named neighbours, no directory
/// sweep — so *too wide* cannot happen by construction; and **nothing
/// narrower** — the private half is in the set always — so *too narrow* is
/// only reachable by a unit failing, which is then said, not silent.
///
/// `path` is the row's recorded path and therefore the real file (L3): a set
/// composed over a link's own path would unlink the link, leave the private
/// half standing, and the next draw would discover it as a new key — **the
/// user was told the key was deleted and it comes back.** The link itself is
/// not one of the three names and nothing here removes it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FileSet {
    pub private_half: PathBuf,
    /// Present only where the file exists at the moment of the act.
    pub public_half: Option<PathBuf>,
    pub certificate: Option<PathBuf>,
}

impl FileSet {
    /// The set, composed from the row's recorded path and the existence of the
    /// two companions at this moment.
    pub fn at_this_moment(
        recorded_path: &Path,
        public_half_exists: bool,
        certificate_exists: bool,
    ) -> FileSet {
        let beside = locations::companions(recorded_path);
        FileSet {
            private_half: recorded_path.to_path_buf(),
            public_half: public_half_exists.then_some(beside.public_half),
            certificate: certificate_exists.then_some(beside.certificate),
        }
    }

    /// What the confirmation names, by full path, in the order the units run —
    /// **the private half first** (W8).
    pub fn named_by_the_confirmation(&self) -> Vec<&Path> {
        let mut named: Vec<&Path> = vec![&self.private_half];
        named.extend(self.public_half.as_deref());
        named.extend(self.certificate.as_deref());
        named
    }
}

/// **W11 and W11a: the servers relying on a key, matched on the fingerprint and
/// nothing else** (`Servers` P16; the row holds the fingerprint for exactly
/// this).
///
/// **W12: where the server rows cannot be read at that moment, the warning
/// says so and never reads as good news.** The two states are distinct and
/// never conflated: *no saved server uses this key* is said only on a
/// **successful** read returning no match; a failed read produces *Smoozy could
/// not check its saved servers just now*, named as what could not be done,
/// **with the user still free to proceed**.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UsedBy {
    /// The read answered and these saved servers hold this key's fingerprint,
    /// **named by their labels, never counted**.
    TheseServers(Vec<String>),
    /// The read answered and no saved server holds it.
    NoSavedServer,
    /// W12's third state. **It arises at an act's own moment and never on the
    /// standing list**, where the key rows and the server rows come out of one
    /// read: a draw that has key rows to show has the server rows in the same
    /// hand.
    CouldNotCheck,
}

/// One saved server as this module reads it — the two fields W11 needs and
/// nothing else. **This document writes nothing into a server row, ever.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SavedServer {
    /// What the warning names it by. `Servers` D3's display-only label.
    pub label: String,
    /// The fingerprint that row holds beside its key reference.
    pub fingerprint: Fingerprint,
}

/// The match, over one read of the server rows.
///
/// **W11a: one read per draw, not one per row.** The server rows are read from
/// the local record once and matched against every key row in that one pass,
/// and **the match value is the row's recorded fingerprint**, never a
/// fingerprint read from the file at the draw. So an unusable key names its
/// servers exactly as a usable one does — which is the case that matters,
/// since a key whose file is gone is the one most likely to be taken away.
pub fn used_by(row: &KeyRow, servers: Option<&[SavedServer]>) -> UsedBy {
    let Some(servers) = servers else {
        return UsedBy::CouldNotCheck;
    };
    let Some(held) = &row.fingerprint else {
        // A row that has never held a fingerprint matches nothing, and the read
        // answered — so this is the *no saved server* state and not the third
        // one.
        return UsedBy::NoSavedServer;
    };
    let matching: Vec<String> = servers
        .iter()
        .filter(|server| server.fingerprint == held.value)
        .map(|server| server.label.clone())
        .collect();
    if matching.is_empty() {
        UsedBy::NoSavedServer
    } else {
        UsedBy::TheseServers(matching)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fingerprint::{ReadFrom, Reading};
    use smoozy_state::identities::{Identity, IdentityKind};
    use std::collections::HashMap;

    struct Volume(HashMap<PathBuf, PathBuf>);

    impl CanonicalPaths for Volume {
        fn canonical(&self, path: &Path) -> Option<PathBuf> {
            self.0.get(path).cloned()
        }
    }

    fn home() -> PathBuf {
        PathBuf::from("/Users/someone")
    }

    fn volume(pairs: &[(&str, &str)]) -> Volume {
        let mut resolved = HashMap::new();
        resolved.insert(
            locations::usual_key_location(&home()),
            PathBuf::from("/Users/someone/.ssh"),
        );
        for (from, to) in pairs {
            resolved.insert(PathBuf::from(from), PathBuf::from(to));
        }
        Volume(resolved)
    }

    fn sha(body: &str) -> Fingerprint {
        Fingerprint::from_the_printed_form(&format!("SHA256:{body}")).expect("a form")
    }

    fn a_row(fingerprint: Option<&str>) -> KeyRow {
        KeyRow::entering(
            Identity::issue(IdentityKind::LocalKey).expect("randomness"),
            PathBuf::from("/Users/someone/.ssh/id_ed25519"),
            fingerprint.map(|body| Reading {
                value: sha(body),
                read_from: ReadFrom::PrivateFile,
            }),
            None,
            false,
        )
        .expect("a row")
    }

    #[test]
    fn a_key_sitting_in_the_location_is_managed_however_it_got_there() {
        let volume = volume(&[(
            "/Users/someone/.ssh/id_ed25519",
            "/Users/someone/.ssh/id_ed25519",
        )]);
        assert_eq!(
            ownership(
                Path::new("/Users/someone/.ssh/id_ed25519"),
                &home(),
                &volume
            ),
            Ownership::Managed
        );
    }

    #[test]
    fn a_link_in_the_location_pointing_out_of_it_is_borrowed() {
        let volume = volume(&[("/Users/someone/.ssh/work", "/Volumes/stick/work_key")]);
        assert_eq!(
            ownership(Path::new("/Users/someone/.ssh/work"), &home(), &volume),
            Ownership::Borrowed
        );
    }

    #[test]
    fn a_file_in_a_subdirectory_of_the_location_is_not_sitting_in_it() {
        let volume = volume(&[(
            "/Users/someone/.ssh/old/id_rsa",
            "/Users/someone/.ssh/old/id_rsa",
        )]);
        assert_eq!(
            ownership(Path::new("/Users/someone/.ssh/old/id_rsa"), &home(), &volume),
            Ownership::Borrowed
        );
    }

    /// **W9's dead managed row, which had no reachable branch at all.** A
    /// managed key whose files have gone away derived as borrowed, so the row
    /// drew *Remove* and the delete refused — where W9 says such a delete
    /// removes nothing, says the files were already gone, and still runs the
    /// erasure unit, *"which is how a dead managed row is cleared"*.
    #[test]
    fn a_managed_key_whose_file_has_gone_is_still_managed() {
        // The file does not resolve; its parent is the usual key location.
        let volume = volume(&[]);
        assert_eq!(
            ownership(Path::new("/Users/someone/.ssh/gone"), &home(), &volume),
            Ownership::Managed
        );
    }

    /// **And the caution the old answer was protecting is kept.** A borrowed
    /// key whose file has gone stays borrowed, because its parent is not the
    /// location — so nothing that was only ever borrowed becomes deletable by
    /// disappearing.
    #[test]
    fn a_borrowed_key_whose_file_has_gone_stays_borrowed() {
        let volume = volume(&[("/Volumes/stick", "/Volumes/stick")]);
        assert_eq!(
            ownership(Path::new("/Volumes/stick/work_key"), &home(), &volume),
            Ownership::Borrowed
        );
    }

    /// **Where the parent will not resolve either, the answer stays borrowed** —
    /// the direction that destroys nothing, and the one the whole derivation
    /// took before this branch existed.
    #[test]
    fn a_path_whose_parent_does_not_resolve_takes_the_direction_that_destroys_nothing() {
        let volume = volume(&[]);
        assert_eq!(
            ownership(Path::new("/Volumes/unplugged/keys/id_rsa"), &home(), &volume),
            Ownership::Borrowed
        );
    }

    /// A file in a subdirectory of the location is not *sitting in* it, and
    /// that stays true when the file has gone.
    #[test]
    fn a_missing_file_in_a_subdirectory_of_the_location_is_still_not_sitting_in_it() {
        let volume = volume(&[("/Users/someone/.ssh/old", "/Users/someone/.ssh/old")]);
        assert_eq!(
            ownership(Path::new("/Users/someone/.ssh/old/id_rsa"), &home(), &volume),
            Ownership::Borrowed
        );
    }

    #[test]
    fn the_file_set_holds_the_private_half_always_and_nothing_that_is_not_there() {
        let set = FileSet::at_this_moment(Path::new("/k/id_rsa"), true, false);
        assert_eq!(set.private_half, PathBuf::from("/k/id_rsa"));
        assert_eq!(set.public_half, Some(PathBuf::from("/k/id_rsa.pub")));
        assert_eq!(set.certificate, None);
        assert_eq!(
            set.named_by_the_confirmation(),
            vec![Path::new("/k/id_rsa"), Path::new("/k/id_rsa.pub")]
        );
    }

    #[test]
    fn the_confirmation_names_the_private_half_first() {
        let set = FileSet::at_this_moment(Path::new("/k/id_rsa"), true, true);
        assert_eq!(set.named_by_the_confirmation()[0], Path::new("/k/id_rsa"));
        assert_eq!(set.named_by_the_confirmation().len(), 3);
    }

    #[test]
    fn the_servers_are_matched_on_the_fingerprint_and_named_by_their_labels() {
        let servers = vec![
            SavedServer {
                label: "render one".into(),
                fingerprint: sha("one"),
            },
            SavedServer {
                label: "render two".into(),
                fingerprint: sha("one"),
            },
            SavedServer {
                label: "something else".into(),
                fingerprint: sha("two"),
            },
        ];
        assert_eq!(
            used_by(&a_row(Some("one")), Some(&servers)),
            UsedBy::TheseServers(vec!["render one".into(), "render two".into()])
        );
    }

    #[test]
    fn a_failed_read_never_reads_as_no_saved_server_uses_this_key() {
        assert_eq!(used_by(&a_row(Some("one")), None), UsedBy::CouldNotCheck);
        assert_eq!(
            used_by(&a_row(Some("one")), Some(&[])),
            UsedBy::NoSavedServer
        );
    }

    #[test]
    fn an_unusable_row_names_its_servers_from_the_record_exactly_as_a_usable_one_does() {
        // Nothing here reads a file: the match value is the row's own recorded
        // fingerprint, so a key whose file is gone still names its servers.
        let servers = vec![SavedServer {
            label: "render one".into(),
            fingerprint: sha("one"),
        }];
        assert_eq!(
            used_by(&a_row(Some("one")), Some(&servers)),
            UsedBy::TheseServers(vec!["render one".into()])
        );
    }
}
