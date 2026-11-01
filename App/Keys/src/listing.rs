//! What in a key location is listed as a key, and how a list gets its answers
//! — `Keys` §4.1 and §4.5 U10a.

use std::path::{Path, PathBuf};

use crate::fingerprint::Reading;
use crate::ownership::UsedBy;
use crate::rows::{KeyRow, Name};
use crate::usable::Verdict;

/// **U1: the list is the union of two sets and nothing else.**
///
/// 1. **Every key row in the local record** — shown whatever the current state
///    of its file, which is what keeps *broken managed keys are visible rather
///    than silently absent* true: a row whose file is missing, unreadable or
///    swapped shows as unusable with the reason, **from the record**, needing
///    nothing from the file.
/// 2. **Every regular file in the usual key location that passes the container
///    test and has no row** — a discovered key, given a row at first sight.
///
/// **U3: a file that was never known and does not pass the container test is
/// not shown.** The spec's *everything found there, usable or not* is a promise
/// about keys — broken **managed keys** stay visible — not an obligation to
/// render `known_hosts` as a broken key. A once-known key that decays past
/// recognition stays visible from its row, so nothing a user was ever shown
/// goes silently absent.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Candidate {
    /// The canonical path of the file found in the location.
    pub path: PathBuf,
    /// U2's in-place header read said this is a private key container.
    pub passes_the_container_test: bool,
    /// What the discovery pass's fingerprint read answered, where it answered.
    pub fingerprint: Option<Reading>,
    /// The comment field, where the same reading surfaced one.
    pub name: Option<Name>,
}

/// A file the discovery pass admits, and the row it is about to be given
/// (local key identifier, path, fingerprint where readable, name where
/// readable).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Discovered {
    pub path: PathBuf,
    pub fingerprint: Option<Reading>,
    pub name: Option<Name>,
}

/// U1 set 2, with U1a applied.
///
/// **U1a: discovery refuses a second row for a fingerprint a current row
/// already holds.** F4 and F5 both live inside the **pointing** refusal and
/// discovery does not run them — so without this a private key copied to a
/// second filename in the usual key location, or hard-linked there, would give
/// two files that both pass the container test, both lack a row, and both take
/// one, each with its own identifier and the same fingerprint.
///
/// **The existing row survives and keeps its recorded path**, because a row
/// holds one path (R1) and saved servers may already hold its identifier;
/// moving a live row onto a file that appeared beside it would carry those
/// servers onto a path their rows never named.
///
/// **What the rule cannot reach, said plainly rather than covered over: where
/// the discovered file's fingerprint cannot be read, the comparison cannot run
/// and the case is not caught.** That is `Keys` §13 item 6's class, which since
/// U7b is **any protected key with no readable `.pub` beside it** and not the
/// legacy-PEM rarity this sentence named until 2026-09-03; there the second
/// file takes its own row.
pub fn discoveries(found_in_the_location: &[Candidate], rows: &[KeyRow]) -> Vec<Discovered> {
    found_in_the_location
        .iter()
        .filter(|candidate| candidate.passes_the_container_test)
        .filter(|candidate| !rows.iter().any(|row| row.path == candidate.path))
        .filter(|candidate| match &candidate.fingerprint {
            Some(read) => !rows.iter().any(|row| {
                row.fingerprint
                    .as_ref()
                    .is_some_and(|held| held.value == read.value)
            }),
            None => true,
        })
        .map(|candidate| Discovered {
            path: candidate.path.clone(),
            fingerprint: candidate.fingerprint.clone(),
            name: candidate.name.clone(),
        })
        .collect()
}

/// One row as a list draws it — `Keys` U10a.
///
/// **A list draws first and each key's verdict fills in behind it; no draw
/// waits on a reading.** The answer is forced rather than chosen: every clause
/// of U4 is a filesystem call or a tool invocation, and `Desktop app and
/// delivery` P4 and `Desktop app and delivery` P5 put every one of them off
/// the thread that draws.
///
/// **What a set 1 row carries before its verdict arrives is everything the
/// record holds** — the name as last read, the fingerprint, the recorded path
/// and the *Used by* line — all of them R1's and none of them needing the
/// file. **What is absent is the usable-or-unusable verdict and U6's reason,
/// and nothing else.** How the row draws that absence is Design's; **the bound
/// is that no verdict may be drawn that has not been read**, which is why the
/// verdict is an absence here rather than a value with a third meaning.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AtDraw {
    pub row: KeyRow,
    /// W11a's line, taken once for the whole list at this draw.
    pub used_by: UsedBy,
    /// `None` until this key's own reading answers. **A row is never shown as
    /// usable while it is still being asked, and never as unusable either.**
    pub verdict: Option<Verdict>,
}

impl AtDraw {
    /// A set 1 row at the instant the list is drawn: everything the record
    /// holds, and no verdict.
    pub fn from_the_record(row: KeyRow, used_by: UsedBy) -> AtDraw {
        AtDraw {
            row,
            used_by,
            verdict: None,
        }
    }

    /// **Nothing is cached and U10 is unchanged**: leaving the screen and
    /// coming back is a new ask, and the verdict is never durable (R1's
    /// *derived, never stored*), so carrying it here is not state under
    /// `Desktop app and delivery` P7.
    pub fn the_reading_answered(&mut self, verdict: Verdict) {
        self.verdict = Some(verdict);
    }
}

/// **A row that arrives after W11a's server-row pass takes its *Used by* line
/// from the result that pass already held for this draw.**
///
/// W11a's reasoning is that the key rows and the server rows come out of one
/// read of the local record, which is true of set 1 and **not** of a row that
/// did not exist when the pass ran — and a discovered row drawn with **no**
/// *Used by* line would say what W11a states such a line means, *the read
/// succeeded and no saved server holds this key's fingerprint*, in front of a
/// **Delete** control, for a key a saved server relies on.
///
/// **The server-row result is held for the life of that draw and the late row
/// is matched against it**, so W11a's *one read per draw* is unchanged and no
/// second read is taken.
pub fn a_late_row_takes_the_line_this_draw_already_held(
    row: KeyRow,
    the_pass_this_draw_held: &dyn Fn(&KeyRow) -> UsedBy,
) -> AtDraw {
    let used_by = the_pass_this_draw_held(&row);
    AtDraw::from_the_record(row, used_by)
}

/// **U1 set 1 is every key row**, whether its recorded path lies in the usual
/// key location or was pointed at anywhere else. Named as a function so the
/// union is one thing a reader can find rather than an assumption at each
/// caller.
pub fn from_the_record(rows: &[KeyRow]) -> &[KeyRow] {
    rows
}

/// Whether a path found in the location already has a row — the one thing
/// discovery asks of the record before the fingerprint comparison.
pub fn already_has_a_row(path: &Path, rows: &[KeyRow]) -> bool {
    rows.iter().any(|row| row.path == path)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fingerprint::{Fingerprint, ReadFrom};
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

    fn a_row(path: &str, fingerprint: Option<&str>) -> KeyRow {
        KeyRow::entering(
            Identity::issue(IdentityKind::LocalKey).expect("randomness"),
            PathBuf::from(path),
            fingerprint.map(private),
            None,
            false,
        )
        .expect("a row")
    }

    fn a_candidate(path: &str, container: bool, fingerprint: Option<&str>) -> Candidate {
        Candidate {
            path: PathBuf::from(path),
            passes_the_container_test: container,
            fingerprint: fingerprint.map(private),
            name: None,
        }
    }

    #[test]
    fn nothing_that_fails_the_container_test_is_ever_discovered() {
        let found = vec![
            a_candidate("/k/known_hosts", false, None),
            a_candidate("/k/id_rsa.pub", false, None),
            a_candidate("/k/config", false, None),
        ];
        assert!(discoveries(&found, &[]).is_empty());
    }

    #[test]
    fn a_file_that_already_has_a_row_is_not_discovered_twice() {
        let rows = vec![a_row("/k/id_rsa", Some("one"))];
        let found = vec![a_candidate("/k/id_rsa", true, Some("one"))];
        assert!(discoveries(&found, &rows).is_empty());
    }

    #[test]
    fn a_copy_under_a_second_name_is_refused_a_row_and_the_first_row_keeps_its_path() {
        let rows = vec![a_row("/k/id_rsa", Some("one"))];
        let found = vec![a_candidate("/k/id_rsa_copy", true, Some("one"))];
        assert!(discoveries(&found, &rows).is_empty());
        assert_eq!(rows[0].path, PathBuf::from("/k/id_rsa"));
    }

    #[test]
    fn a_genuinely_new_key_beside_a_known_one_takes_a_row() {
        let rows = vec![a_row("/k/id_rsa", Some("one"))];
        let found = vec![a_candidate("/k/id_ed25519", true, Some("two"))];
        assert_eq!(
            discoveries(&found, &rows),
            vec![Discovered {
                path: PathBuf::from("/k/id_ed25519"),
                fingerprint: Some(private("two")),
                name: None,
            }]
        );
    }

    #[test]
    fn where_the_fingerprint_cannot_be_read_the_comparison_cannot_run_and_the_file_takes_a_row() {
        let rows = vec![a_row("/k/legacy", None)];
        let found = vec![a_candidate("/k/legacy_copy", true, None)];
        assert_eq!(discoveries(&found, &rows).len(), 1);
    }

    #[test]
    fn a_row_draws_from_the_record_with_no_verdict_and_takes_one_when_it_arrives() {
        let mut drawn = AtDraw::from_the_record(a_row("/k/id_rsa", Some("one")), UsedBy::NoSavedServer);
        assert_eq!(drawn.verdict, None);
        drawn.the_reading_answered(Verdict::Usable);
        assert_eq!(drawn.verdict, Some(Verdict::Usable));
    }

    #[test]
    fn a_late_row_never_draws_an_empty_used_by_line_it_did_not_earn() {
        let held = |_: &KeyRow| UsedBy::TheseServers(vec!["render one".into()]);
        let drawn =
            a_late_row_takes_the_line_this_draw_already_held(a_row("/k/found", Some("one")), &held);
        assert_eq!(
            drawn.used_by,
            UsedBy::TheseServers(vec!["render one".into()])
        );
        assert_eq!(drawn.verdict, None);
    }

    #[test]
    fn every_key_row_is_in_the_list_whatever_its_file_is_doing() {
        let rows = vec![a_row("/k/id_rsa", Some("one")), a_row("/elsewhere/borrowed", None)];
        assert_eq!(from_the_record(&rows).len(), 2);
        assert!(already_has_a_row(Path::new("/k/id_rsa"), &rows));
        assert!(!already_has_a_row(Path::new("/k/other"), &rows));
    }
}
