//! Which Blender — `Render Settings` §7: the list and the pick (B1–B6). The
//! mismatch warning's material (B7–B9) is gone with the Creator's note 12 on
//! 0.1.7 — see [`NO_BREAK_NOTES_SHIP`].
//!
//! **Constraint:** Row 155. `Server Setup` §12's supply, taken whole; the
//! version match rule — nearest above, exact wins, nearest below where
//! nothing above exists — Creator-stated (2026-07-29), a decision about what
//! to prefer. Row 156, answered at its own altitude: the warning's content is
//! Design's and its wording Build's; what is this document's is where the
//! version-pair material lives and what the page says for a pair Smoozy has
//! no notes for.
//!
//! **Markers used in this file that belong to another document, each with the
//! document it belongs to.** `Server Setup` B11, B12, B13. `Rendering` L14.
//! `Dependency Upload & Remap` G2. `File Browser` N7a. `Desktop app and
//! delivery` Z25.
//!
//! **`Rendering` W2 is not listed above and is written out at the site**,
//! because this plan defines a W2 of its own and a bare one could mean
//! either.

use smoozy_server_setup::blender::{Version, THE_FLOOR};
use smoozy_server_setup::search::{the_listing, ACandidate, TheSearchResult};

use crate::file_half::BlenderIdentity;

/// **The list B1 hands over, re-exported so that a caller of this crate can
/// name it without naming `Server Setup`.** Added 2026-09-13 at step 10a piece
/// 8, which gives this crate its first caller: the page holds the list this
/// clause's [`resolve`] takes, and B1 is emphatic that **the list is `Server
/// Setup`'s search result taken whole** — so the caller must hold that type and
/// not a copy of it. **It is a re-export and not a second shape**, and it adds
/// no dependency anywhere: this crate already names that one.
pub use smoozy_server_setup::search::TheSearchResult as TheListThisPageAsksFor;

/// **B2: a release line** — the two components the match is made on, so that
/// a file's own build is not preferred where a newer build of the same line
/// stands. The saved-with version `File Browser` reads without opening the
/// file is two components, and this is what it becomes here.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct ReleaseLine {
    pub major: u32,
    pub minor: u32,
}

impl ReleaseLine {
    pub fn of(version: Version) -> ReleaseLine {
        ReleaseLine {
            major: version.major,
            minor: version.minor,
        }
    }
}

/// **B1: the list is `Server Setup`'s fresh search result — asked for when
/// this page wants it, period-held, never durable, shown with its moment and
/// with B11's honesty marks**: where refusals occurred the page says the
/// machine could not be searched completely, a renamed binary is invisible
/// and the answer says the search is by name, and a candidate that did not
/// answer is named as one. Nothing here re-runs, caches durably, or
/// supplements the search. **B12 makes this page's ask one of that search's
/// three moments.**
///
/// **One thing the built list does not carry, recorded rather than
/// invented**: B1 names *whether the answer was long-term-support or
/// unstable* per build, and `Server Setup`'s built entry carries the path
/// and the answered version alone. The page shows what the list carries.
pub const THE_LIST_IS_SERVER_SETUPS_SEARCH_RESULT_TAKEN_WHOLE: bool = true;

/// **B3 and B13: a version number does not identify a build, which is why the
/// pick carries the path.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PickedBuild {
    pub path: String,
    pub answered: Version,
}

impl PickedBuild {
    pub fn as_identity(&self) -> BlenderIdentity {
        BlenderIdentity {
            path: self.path.clone(),
            version: self.answered,
        }
    }
}

/// How the automatic match landed — **nearest below is said as such.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HowItMatched {
    TheExactLine,
    TheNearestLineAbove,
    TheNearestLineBelow,
}

/// **B2's outcome, and B5's two honest cases produced by the same rule —
/// neither is an error.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AutomaticMatch {
    Picked {
        build: PickedBuild,
        how: HowItMatched,
    },
    /// **B5's first case**: a machine holding nothing but builds below the
    /// floor — the automatic match has nothing to land on, no version is
    /// picked, the page says why, and the user may pick one themselves.
    NothingButBuildsBelowTheFloor,
    /// **B5's second case**: the list is empty — pre-load has nothing to run
    /// with, submitting is not offered, with the reason named and the user
    /// pointed at `Server Setup`.
    TheListIsEmpty,
    /// **The file's saved-with version is blank** — `File Browser` shows no
    /// version and never a guess — so B2's rule has nothing to match on. The
    /// resolution answers with the case rather than a guess, exactly as B2a
    /// has it for the two cases above; the user's own pick is the remedy.
    NoSavedWithVersionToMatchOn,
}

/// **B2: the automatic match is computed over answered versions parsed as
/// components, never as strings** (Fact 54's sort trap, closed by `Server
/// Setup` B2's discipline). An exact line match wins over everything, with
/// the newest build of that line taken — **the rule's *exact* is read as the
/// release line and not as the build, deliberately**; failing that, the
/// nearest line above; where nothing above exists, the nearest below, said as
/// such. **Builds below the whole-app floor of 4.5 appear in the list, can be
/// picked, and are never the automatic pick.**
///
/// **B3: two builds answering one version are one version to the rule**, and
/// which is picked is deterministic within one list — one stated ordering,
/// the newest patch and then the lowest path — so two asks over one list give
/// one answer.
pub fn the_automatic_match(
    saved_with: Option<ReleaseLine>,
    list: &TheSearchResult,
) -> AutomaticMatch {
    let listed: Vec<ACandidate> = the_listing(&list.entries);
    if listed.is_empty() {
        return AutomaticMatch::TheListIsEmpty;
    }
    let eligible: Vec<(String, Version)> = listed
        .into_iter()
        .filter_map(|entry| entry.answered.map(|answered| (entry.path, answered)))
        .filter(|(_, answered)| *answered >= THE_FLOOR)
        .collect();
    if eligible.is_empty() {
        return AutomaticMatch::NothingButBuildsBelowTheFloor;
    }
    let Some(saved_with) = saved_with else {
        return AutomaticMatch::NoSavedWithVersionToMatchOn;
    };

    let newest_of = |line: ReleaseLine| -> PickedBuild {
        let mut of_the_line: Vec<&(String, Version)> = eligible
            .iter()
            .filter(|(_, answered)| ReleaseLine::of(*answered) == line)
            .collect();
        // B3's one stated ordering: the newest version first, then the
        // lowest path.
        of_the_line.sort_by(|left, right| right.1.cmp(&left.1).then_with(|| left.0.cmp(&right.0)));
        let (path, answered) = of_the_line[0];
        PickedBuild {
            path: path.clone(),
            answered: *answered,
        }
    };

    let lines: Vec<ReleaseLine> = eligible
        .iter()
        .map(|(_, answered)| ReleaseLine::of(*answered))
        .collect();
    if lines.contains(&saved_with) {
        return AutomaticMatch::Picked {
            build: newest_of(saved_with),
            how: HowItMatched::TheExactLine,
        };
    }
    if let Some(above) = lines.iter().filter(|line| **line > saved_with).min() {
        return AutomaticMatch::Picked {
            build: newest_of(*above),
            how: HowItMatched::TheNearestLineAbove,
        };
    }
    let below = lines
        .iter()
        .filter(|line| **line < saved_with)
        .max()
        .expect("every eligible line is above or below a line it is not equal to");
    AutomaticMatch::Picked {
        build: newest_of(*below),
        how: HowItMatched::TheNearestLineBelow,
    }
}

/// **B2a: *the version selected in `Render Settings` for this file* is a
/// resolution rather than a page state.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Resolution {
    /// The store's version pick override where one stands (J3).
    TheUsersOverride(BlenderIdentity),
    /// Else B2's automatic match computed over a fresh B1 list.
    Automatic {
        build: PickedBuild,
        how: HowItMatched,
    },
    /// **The resolution answers with the case rather than a guess; no check
    /// runs**, and what the user is then told is `Dependency Upload & Remap`
    /// G2's.
    NoVersionResolves(WhyNone),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhyNone {
    NothingButBuildsBelowTheFloor,
    TheListIsEmpty,
    NoSavedWithVersionToMatchOn,
    /// **While the page has no machine, the version control has no list and
    /// no resolution, and nothing is claimed for it** (Y15's no-machine
    /// paragraph): B1's list is a machine's own fresh search result and there
    /// is no machine to ask it of.
    NoMachineInHand,
}

/// **B2a, performed.** The same store, the same rule, the same list ask
/// serve every consumer — a dependency check started from `File Browser`
/// N7a's toolbar with this page never opened resolves it exactly as a
/// submission resolves it at B4.
pub fn resolve(
    the_stores_override: Option<&BlenderIdentity>,
    saved_with: Option<ReleaseLine>,
    list: Option<&TheSearchResult>,
) -> Resolution {
    if let Some(pick) = the_stores_override {
        return Resolution::TheUsersOverride(pick.clone());
    }
    let Some(list) = list else {
        return Resolution::NoVersionResolves(WhyNone::NoMachineInHand);
    };
    match the_automatic_match(saved_with, list) {
        AutomaticMatch::Picked { build, how } => Resolution::Automatic { build, how },
        AutomaticMatch::NothingButBuildsBelowTheFloor => {
            Resolution::NoVersionResolves(WhyNone::NothingButBuildsBelowTheFloor)
        }
        AutomaticMatch::TheListIsEmpty => Resolution::NoVersionResolves(WhyNone::TheListIsEmpty),
        AutomaticMatch::NoSavedWithVersionToMatchOn => {
            Resolution::NoVersionResolves(WhyNone::NoSavedWithVersionToMatchOn)
        }
    }
}

impl Resolution {
    /// **B4: the payload carries the picked build's path and answered
    /// version, always** — the pick is resolved at submission into that
    /// concrete pair: no manifest ever says *automatic*. `None` where nothing
    /// resolves, and a job cannot be submitted without a Blender to render
    /// it (B5).
    pub fn the_payloads_pick(&self) -> Option<BlenderIdentity> {
        match self {
            Resolution::TheUsersOverride(pick) => Some(pick.clone()),
            Resolution::Automatic { build, .. } => Some(build.as_identity()),
            Resolution::NoVersionResolves(_) => None,
        }
    }

    /// **B5: submitting is offered only with a Blender to render by.**
    pub fn submitting_is_offered(&self) -> bool {
        self.the_payloads_pick().is_some()
    }

    /// **B5a: where no version resolves, the read is not offered either, and
    /// the user's own pick is the one act that offers it.** Pre-load is a
    /// consumer of B2a's resolution exactly as G2's check is: a read has no
    /// version to run with here, and a build chosen by Smoozy for the reading
    /// alone would break `Job files and outputs` R2 outright — the build that
    /// read the file would not be the build that renders it. The withholding
    /// lasts exactly until the pick.
    pub fn the_read_is_offered(&self) -> bool {
        self.the_payloads_pick().is_some()
    }
}

/// **B6: a pick made against a stale list costs a failed attempt, never a
/// wrong render.** The list is period-held and the machine can change under
/// it; what stands guard is `Rendering` L14 — the render worker confirms the
/// binary at the manifest's path answers with the manifest's version, and
/// fails the attempt before the render instruction where it does not. **And
/// one route moves the pick between a check and the submission by the
/// machinery** (B2a's last paragraph): a check run while a history-opened
/// page is still keyless resolves under the automatic match, the one write at
/// the pick puts the override into the store, and the check's standing then
/// meets `Rendering` W2's third case honestly.
pub const A_STALE_PICK_COSTS_A_FAILED_ATTEMPT_NEVER_A_WRONG_RENDER: bool = true;

/// **B7 and B9 are struck, and B8 composes nothing here** — 2026-10-02, the
/// Creator's note 12 on 0.1.7: *"Just a small indication (not a button) that
/// something may not work as expected is enough."* The mismatch line names the
/// two versions and says something may not work as expected; it opens nothing,
/// so no break notes ship. *(Until that day this file held the shipped notes,
/// keyed per version line from* Settings 10*, and the material the panel was
/// composed from. The two silent breaks they named are no longer named
/// anywhere in Smoozy, and that is the Creator's choice.)* The line itself is
/// composed where the page is — `src/surface/render_settings.rs` in the
/// desktop, `the_mismatch`.
pub const NO_BREAK_NOTES_SHIP: bool = true;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::file_half::testing::{a_blender, a_version};
    use smoozy_server_setup::search::WhatTheAnswerSays;
    use std::time::SystemTime;

    fn a_list(builds: &[(&str, Option<Version>)]) -> TheSearchResult {
        let entries: Vec<ACandidate> = builds
            .iter()
            .map(|(path, answered)| ACandidate {
                path: path.to_string(),
                answered: *answered,
            })
            .collect();
        TheSearchResult {
            answer: WhatTheAnswerSays {
                the_search_is_by_name: true,
                folders_refused: 0,
                did_not_answer: Vec::new(),
            },
            entries,
            at: SystemTime::UNIX_EPOCH,
        }
    }

    fn line(major: u32, minor: u32) -> ReleaseLine {
        ReleaseLine { major, minor }
    }

    #[test]
    fn an_exact_line_wins_and_the_newest_build_of_that_line_is_taken() {
        let list = a_list(&[
            ("/opt/a/blender", Some(a_version(4, 5, 9))),
            ("/opt/b/blender", Some(a_version(4, 5, 12))),
            ("/opt/c/blender", Some(a_version(5, 2, 1))),
        ]);
        let matched = the_automatic_match(Some(line(4, 5)), &list);
        assert_eq!(
            matched,
            AutomaticMatch::Picked {
                build: PickedBuild {
                    path: "/opt/b/blender".to_string(),
                    answered: a_version(4, 5, 12)
                },
                how: HowItMatched::TheExactLine
            }
        );
    }

    #[test]
    fn versions_are_matched_as_components_and_never_as_strings() {
        // Fact 54's sort trap: 4.5.9 against 4.5.23.
        let list = a_list(&[
            ("/opt/a/blender", Some(a_version(4, 5, 9))),
            ("/opt/b/blender", Some(a_version(4, 5, 23))),
        ]);
        let AutomaticMatch::Picked { build, .. } = the_automatic_match(Some(line(4, 5)), &list)
        else {
            panic!("a pick");
        };
        assert_eq!(build.answered, a_version(4, 5, 23));
    }

    #[test]
    fn failing_an_exact_line_the_nearest_above_wins_and_the_nearest_below_is_said_as_such() {
        let list = a_list(&[
            ("/opt/a/blender", Some(a_version(5, 0, 1))),
            ("/opt/b/blender", Some(a_version(5, 2, 1))),
        ]);
        let AutomaticMatch::Picked { build, how } = the_automatic_match(Some(line(4, 5)), &list)
        else {
            panic!("a pick");
        };
        assert_eq!(build.answered, a_version(5, 0, 1));
        assert_eq!(how, HowItMatched::TheNearestLineAbove);

        let AutomaticMatch::Picked { build, how } = the_automatic_match(Some(line(5, 1)), &list)
        else {
            panic!("a pick");
        };
        assert_eq!(build.answered, a_version(5, 2, 1));
        assert_eq!(how, HowItMatched::TheNearestLineAbove);

        let AutomaticMatch::Picked { build, how } = the_automatic_match(Some(line(6, 0)), &list)
        else {
            panic!("a pick");
        };
        assert_eq!(build.answered, a_version(5, 2, 1));
        assert_eq!(how, HowItMatched::TheNearestLineBelow);
    }

    #[test]
    fn two_builds_of_one_version_are_picked_deterministically_by_one_stated_ordering() {
        let list = a_list(&[
            ("/opt/z/blender", Some(a_version(4, 5, 12))),
            ("/opt/a/blender", Some(a_version(4, 5, 12))),
        ]);
        let AutomaticMatch::Picked { build, .. } = the_automatic_match(Some(line(4, 5)), &list)
        else {
            panic!("a pick");
        };
        assert_eq!(build.path, "/opt/a/blender");
        assert_eq!(
            the_automatic_match(Some(line(4, 5)), &list),
            the_automatic_match(Some(line(4, 5)), &list)
        );
    }

    #[test]
    fn builds_below_the_floor_are_never_the_automatic_pick_and_the_two_honest_cases_are_told_apart()
    {
        let below = a_list(&[("/opt/old/blender", Some(a_version(4, 2, 0)))]);
        assert_eq!(
            the_automatic_match(Some(line(4, 2)), &below),
            AutomaticMatch::NothingButBuildsBelowTheFloor
        );
        let empty = a_list(&[]);
        assert_eq!(
            the_automatic_match(Some(line(4, 5)), &empty),
            AutomaticMatch::TheListIsEmpty
        );
        // A candidate that did not answer is not listed (B11), so a list of
        // such candidates is empty to the rule.
        let silent = a_list(&[("/opt/mute/blender", None)]);
        assert_eq!(
            the_automatic_match(Some(line(4, 5)), &silent),
            AutomaticMatch::TheListIsEmpty
        );
        // A blank saved-with version has nothing to match on.
        let fine = a_list(&[("/opt/a/blender", Some(a_version(4, 5, 12)))]);
        assert_eq!(
            the_automatic_match(None, &fine),
            AutomaticMatch::NoSavedWithVersionToMatchOn
        );
    }

    #[test]
    fn the_resolution_is_the_stores_override_else_the_automatic_match_else_the_case() {
        let list = a_list(&[("/opt/a/blender", Some(a_version(4, 5, 12)))]);
        assert_eq!(
            resolve(Some(&a_blender()), Some(line(4, 5)), Some(&list)),
            Resolution::TheUsersOverride(a_blender())
        );
        // The override stands even for a below-floor build, which can be
        // picked (B2) — and stands with no list at all.
        assert_eq!(
            resolve(Some(&a_blender()), None, None),
            Resolution::TheUsersOverride(a_blender())
        );
        assert!(matches!(
            resolve(None, Some(line(4, 5)), Some(&list)),
            Resolution::Automatic { .. }
        ));
        assert_eq!(
            resolve(None, Some(line(4, 5)), None),
            Resolution::NoVersionResolves(WhyNone::NoMachineInHand)
        );
        let below = a_list(&[("/opt/old/blender", Some(a_version(4, 2, 0)))]);
        assert_eq!(
            resolve(None, Some(line(4, 2)), Some(&below)),
            Resolution::NoVersionResolves(WhyNone::NothingButBuildsBelowTheFloor)
        );
    }

    #[test]
    fn neither_the_read_nor_submitting_is_offered_while_no_version_resolves_and_a_pick_offers_both()
    {
        let none = Resolution::NoVersionResolves(WhyNone::NothingButBuildsBelowTheFloor);
        assert!(!none.the_read_is_offered());
        assert!(!none.submitting_is_offered());
        assert_eq!(none.the_payloads_pick(), None);
        let picked = Resolution::TheUsersOverride(a_blender());
        assert!(picked.the_read_is_offered());
        assert!(picked.submitting_is_offered());
        assert_eq!(picked.the_payloads_pick(), Some(a_blender()));
    }
}
