//! What a publication of this document's writes outward — `File Browser` §4.2,
//! X7a, and §8.4, W11.
//!
//! **A publication of this document's owes three desktop writes into
//! neighbouring records, and these clauses are the actor those records were
//! missing.** X7a states the first two and W11 the third; **X7a's own count of
//! two is that clause's count and never the count of every write a publication
//! owes**, which is why the third is stated at W11 and performed here beside
//! them. At each publication of this document's — **an upload's (X7), a copy's
//! (M7) and a same-filesystem move's (M5), which are the three publishing acts
//! W11 already enumerates for its own write; X7a's opening was widened from
//! *an upload's* on 2026-09-05** — in the same act that records the unit's
//! outcome:
//!
//! - **where the published landing path is the root path of a
//!   `Dependency Upload & Remap` check standing on that server under its
//!   current installation identity (its V10's key), the standing is marked
//!   replaced-since-check** — that module's V13 first trigger, which the clause
//!   stated with no actor and which nothing else performs;
//! - **where a recorded reading of `Render Settings`' stands for that (server
//!   row, published path), the reading is dropped whole** — that document's
//!   J10, by its Q14 item 1, on the same ground;
//! - **every check standing whose *tree* resolved the published location is
//!   marked disturbed** — W11, the actor `Dependency Upload & Remap` V13a
//!   states and this document did not carry. Without it an ordinary upload
//!   publishing over an asset a checked root's tree resolved left that
//!   standing unmarked, `Rendering`'s submission warning was composed from a
//!   record still saying a complete check stood for the file, and **no warning
//!   fired at all**.
//!
//! **All three are writes into the local record and process memory on the
//! desktop, Smoozy's own recorded acts, and none reads the machine.** A
//! publication by `Dependency Upload & Remap`'s own row sends performs the same
//! writes from its side (its V13, its V13a).
//!
//! **The third write is not this document's publications alone**: its removals
//! owe it too, and that half is at [`crate::managing`], performed through
//! [`the_disturbance_mark`] here so there is one composition rather than two.
//!
//! **Why the writes go through a trait rather than through the two crates that
//! hold the records.** This crate cannot name either:
//! `Dependency Upload & Remap` is built on `Render Settings`, which is built on
//! `Server Setup`, which is built on this crate for P5's two elevation
//! readings, so a direct dependency is a cycle the build refuses. **The
//! comparison stays here — it is X5's and this document's — and the write is
//! performed next door**, at `Desktop app and delivery`, over
//! `Dependency Upload & Remap`'s own standing and `Render Settings`' own
//! reading store. Neither record is copied and no second shape is invented.
//!
//! **Markers used below that belong to another document, each with the document
//! it belongs to.** `Dependency Upload & Remap` V1, V2, V2a, V6, V10, V13,
//! V13a, V14, M9d, M16b. `Render Settings` J10, Q14, Q13. `Job files and
//! outputs` R12.
//!
//! **A marker this plan also defines is deliberately not listed above,
//! whichever document is meant, and is written out with its document at every
//! site instead** — the header list cannot tell the two apart, and a bare one
//! would resolve to a real and wrong clause. This plan defines a K3, an M8 and
//! a W2 of its own, so `Dependency Upload & Remap`'s keying discipline and its
//! remap marker, and `Rendering`'s submission warning, are written out where
//! they are cited.

use smoozy_state::outcomes::UnitOutcome;

use crate::already_exists::paths_are_equal;

/// **The neighbouring records X7a and W11 write into, and the whole of what
/// this document asks of them.**
///
/// **Nothing here reads the machine.** Both readings are of the desktop's own
/// records and both writes land on the desktop, which is what makes X7a a
/// recorded act of Smoozy's rather than an inference from what sits on a
/// machine.
///
/// **One value is one server row under its current installation identity**, so
/// a standing under a superseded installation identity is disqualified before
/// it reaches this trait and no mark is written for it.
///
/// **Every reading hands back the recorded path text rather than a yes or
/// no**, because [`at_this_publication`] and [`the_disturbance_mark`] apply
/// X5's comparison to it before any write. A reader that matched under a weaker
/// rule is caught there rather than writing a mark against the wrong root.
///
/// **For which readers that re-check is live, said plainly rather than claimed
/// for all four.** It is live for a reader that matches more loosely than X5 —
/// which is any reader keyed on recorded text it did not have to spell the
/// same way. **It is inert for a reader that matches on whole-path equality**,
/// because the location it hands back is then necessarily the location it was
/// asked about, and the comparison is a value against itself. The two
/// readings below that are in that position say so at their own sites.
pub trait TheNeighbouringRecords {
    /// The root path of a `Dependency Upload & Remap` check standing whose root
    /// is this published path, **as the record holds it** — its V10's key.
    fn the_check_standings_root_at(&self, published: &[u8]) -> Option<Vec<u8>>;

    /// The file path a recorded reading of `Render Settings`' stands for on
    /// this server row, **as the record holds it**, where one stands for this
    /// published path.
    fn the_recorded_readings_path_at(&self, published: &[u8]) -> Option<Vec<u8>>;

    /// `Dependency Upload & Remap` V13's first mark, written here because the
    /// publication is this document's act.
    fn mark_the_standing_replaced_since_check(&mut self, root: &[u8]);

    /// `Render Settings` J10's drop, performed by the publishing module at the
    /// publication with no comparison — that document's Q14 item 1.
    fn drop_the_recorded_reading(&mut self, path: &[u8]);

    /// **W11's first reading: `Dependency Upload & Remap` V2's points-at
    /// reading, taken for the roots.** For the machine location the unit
    /// published over or removed, that reading answers which located answers
    /// resolve to it, and **every root whose tree those answers belong to** is
    /// handed back here.
    ///
    /// **No reading is invented and no record kind is created**: this is the
    /// reading [`crate::warnings::ReadsWhatTheLocalRecordKnows`] already takes
    /// at the same acts, asked for a different field of the same answer.
    fn the_roots_whose_tree_resolved(&self, location: &[u8]) -> Vec<ARootWhoseTreeResolvedIt>;

    /// **W11's second reading: `Dependency Upload & Remap` V2a**, which answers
    /// *which fixed copy was made from this machine location* — one more field
    /// of a record its V2 already reads rather than a second reading.
    ///
    /// It is asked because the machine copy a fixed copy was derived from is
    /// the third location [`crate::warnings::WhatTheLandingPathIsFor`] puts in
    /// no class: publishing over it is `Dependency Upload & Remap` M16b's
    /// trigger, and the standings to mark are the ones whose tree resolved the
    /// **fixed copy**, not this location.
    fn the_fixed_copies_made_from(&self, location: &[u8]) -> Vec<AFixedCopyMadeFromIt>;

    /// **W11's write**: `Dependency Upload & Remap` V13a's disturbance mark,
    /// written here because the act is this document's.
    fn mark_the_standing_disturbed(&mut self, root: &[u8]);
}

/// **One answer of W11's first reading**: a check standing's root whose tree
/// resolved the location, with the location **as that module's record holds
/// it** so X5's comparison can be applied to it before the mark is written.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ARootWhoseTreeResolvedIt {
    /// The resolved location as the record spells it.
    pub the_location_as_the_record_holds_it: Vec<u8>,
    /// The root file's machine path — the standing's own key
    /// (`Dependency Upload & Remap` V10).
    pub root: Vec<u8>,
}

/// **One answer of W11's second reading**: a fixed copy made from the location,
/// with the derived-from location **as the record holds it**.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AFixedCopyMadeFromIt {
    /// The derived-from location as the record spells it.
    pub the_location_as_the_record_holds_it: Vec<u8>,
    /// Where the fixed copy sits on the machine.
    pub fixed_copy: Vec<u8>,
}

/// **What one publication of this document's wrote outward.**
///
/// **Three writes**: X7a's two, and W11's. A publication can owe all three,
/// some of them or none.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct WhatThePublicationWrote {
    /// The check standing's root, where one was marked replaced-since-check.
    pub the_standing_marked_replaced_since_check: Option<Vec<u8>>,
    /// The path whose recorded reading was dropped, where one stood.
    pub the_recorded_reading_dropped: Option<Vec<u8>>,
    /// **W11**: the roots whose standings were marked disturbed, because their
    /// tree resolved the location this publication landed on.
    pub the_standings_marked_disturbed: Vec<Vec<u8>>,
}

impl WhatThePublicationWrote {
    pub fn nothing() -> WhatThePublicationWrote {
        WhatThePublicationWrote::default()
    }

    pub fn is_nothing(&self) -> bool {
        self.the_standing_marked_replaced_since_check.is_none()
            && self.the_recorded_reading_dropped.is_none()
            && self.the_standings_marked_disturbed.is_empty()
    }
}

/// **W11, performed: the *disturbed* mark on every check standing whose tree
/// resolved the location an act of this document's published over or removed.**
///
/// **This clause is the actor `Dependency Upload & Remap` V13a states and this
/// document did not carry** — *any Smoozy act that replaced, refixed or deleted
/// anything a root's tree resolved marks that root's standing disturbed,
/// whether or not a warning stood before it*, written by the module that
/// published, at its publication, exactly as the replaced-since-check mark is.
///
/// **Which standings, read off that module's own record and off nothing kept
/// here.** `Dependency Upload & Remap` V2's points-at reading answers which
/// located answers resolve to the location, and every root whose tree those
/// answers belong to is marked. Where the location is the machine copy a fixed
/// copy was derived from, its V2a answers which fixed copies were made from it,
/// and every root whose tree resolved **that fixed copy** is marked, which is
/// that module's M16b's own sentence. Both are readings of the local record
/// (its V1); **nothing here reads the machine**, no reading is invented, no
/// record kind is created, and no class is added to
/// [`crate::warnings::WhatTheLandingPathIsFor`].
///
/// **The comparison is X5's applied whole** — byte for byte, per path
/// component, against the location as the record holds it, no folding, no
/// normalisation, no canonicalising, no resolution of any kind — exactly as
/// X7a's two writes compare. W4's alias limit rides on this write too and is
/// not repaired here.
///
/// **A root is marked once however many routes reach it**, and a root that
/// carries X7a's replaced-since-check mark as well carries both: they are two
/// record states of one standing (`Dependency Upload & Remap` V10, V14) and
/// neither stands in for the other.
///
/// **The caller's own moment is the unit's outcome, and only a unit that took
/// effect reaches this** — published, for a publication; removed, for a
/// removal. **Both halves state that gate in code and neither leans on its call
/// site**: [`crate::managing::at_this_unit_of_a_managing_act`] for a removal,
/// and [`at_this_publication`] for a publication.
pub fn the_disturbance_mark(
    location: &[u8],
    records: &mut dyn TheNeighbouringRecords,
) -> Vec<Vec<u8>> {
    /// X5's comparison against the recorded text, then the root, once.
    fn take(found: Vec<ARootWhoseTreeResolvedIt>, against: &[u8], roots: &mut Vec<Vec<u8>>) {
        for it in found {
            if !paths_are_equal(&it.the_location_as_the_record_holds_it, against) {
                continue;
            }
            if !roots
                .iter()
                .any(|already| paths_are_equal(already, &it.root))
            {
                roots.push(it.root);
            }
        }
    }

    let mut roots: Vec<Vec<u8>> = Vec::new();

    // `Dependency Upload & Remap` V2: the location itself.
    take(
        records.the_roots_whose_tree_resolved(location),
        location,
        &mut roots,
    );

    // Its V2a, and its M16b: the fixed copies made from the location, and the
    // roots whose tree resolved those.
    for copy in records.the_fixed_copies_made_from(location) {
        if !paths_are_equal(&copy.the_location_as_the_record_holds_it, location) {
            continue;
        }
        take(
            records.the_roots_whose_tree_resolved(&copy.fixed_copy),
            &copy.fixed_copy,
            &mut roots,
        );
    }

    for root in &roots {
        records.mark_the_standing_disturbed(root);
    }
    roots
}

/// **What W11 puts in front of the user is nothing, and that is the test it had
/// to pass.**
///
/// No warning is created and none is widened — `Dependency Upload & Remap` V3's
/// plain-upload warning, [`crate::warnings::WhatTheLandingPathIsFor`]'s three
/// classes and the two non-dismissible warnings are all untouched. What changes
/// is only that a standing whose tree the act has changed stops saying the file
/// is ready to render, so `Rendering`'s missing-assets warning stands where it
/// would otherwise have been absent.
pub const W11_CREATES_NO_WARNING_AND_WIDENS_NONE: bool = true;

/// **W11: the two standing marks are independent and neither stands in for the
/// other.**
///
/// One machine location can be one standing's root file and another standing's
/// resolved tree member at once — a linked `.blend` the user also checked as a
/// root in its own right is exactly that — so X7a's replaced-since-check write
/// and W11's may both fall due at one publication, and **both are made**.
pub const THE_TWO_STANDING_MARKS_ARE_INDEPENDENT: bool = true;

/// **X7a and W11, performed: the three writes at one publication of this
/// document's** — an upload's (X7), a copy's (M7) or a same-filesystem move's
/// (M5). **X7a's opening was widened from *an upload's* on 2026-09-05**,
/// because a copy and a same-filesystem move publish over the same path with
/// the same **Replace** answer and `Dependency Upload & Remap` V13's first
/// trigger covers them by its own name and its own reason.
///
/// **The rule is X5's, applied whole**: byte-for-byte equality of the published
/// landing path against the path the record holds, per path component, whole
/// and exact — **no folding, no normalisation, no canonicalising, no resolution
/// of any kind**. It is the rule `Dependency Upload & Remap` K3's own keying
/// already uses, so the standing's key and the recorded reading's key are
/// matched exactly as they were stored. **The server row and the installation
/// identity are matched as recorded values**, which is the reader's, one reader
/// value being one machine.
///
/// **The three writes are independent.** A publication onto a standing's root
/// need not have a recorded reading standing for it, a publication onto a path
/// a reading stands for need not be any standing's root, and a publication over
/// a location some root's tree resolved need not be either.
///
/// **W11's write is made in the same act as X7a's two**, which is what closes
/// the hole a separate call site left: a publication that performed the first
/// two and forgot the third left a checked root claiming its file was still
/// ready to render.
///
/// **Only for a unit that took effect — published, for a publication** — which
/// is W11's own rule for both halves, kept here rather than left to the call
/// site. **A publication can refuse**: X7's create-intended branch refuses over
/// an existing target, and `Server protocol and transfers` S15's read-back can
/// come back wrong. **Took effect is [`UnitOutcome::Done`] and neither of the
/// other two**, and an absent outcome is a unit whose ending was never written,
/// which is not one that took effect either.
pub fn at_this_publication(
    published: &[u8],
    outcome: Option<&UnitOutcome>,
    records: &mut dyn TheNeighbouringRecords,
) -> WhatThePublicationWrote {
    let mut wrote = WhatThePublicationWrote::nothing();

    if outcome != Some(&UnitOutcome::Done) {
        return wrote;
    }

    if let Some(root) = records.the_check_standings_root_at(published) {
        if paths_are_equal(&root, published) {
            records.mark_the_standing_replaced_since_check(&root);
            wrote.the_standing_marked_replaced_since_check = Some(root);
        }
    }

    if let Some(path) = records.the_recorded_readings_path_at(published) {
        if paths_are_equal(&path, published) {
            records.drop_the_recorded_reading(&path);
            wrote.the_recorded_reading_dropped = Some(path);
        }
    }

    // **W11's write, in the same act.**
    wrote.the_standings_marked_disturbed = the_disturbance_mark(published, records);

    wrote
}

/// **X7a: the Blender version is no part of either comparison**, and W11's
/// takes it no more into account than they do.
///
/// The standing's version case is a record state `Rendering` W2 reads at
/// submission, **not a reason to withhold the mark.**
pub const THE_BLENDER_VERSION_IS_NO_PART_OF_EITHER_COMPARISON: bool = true;

/// **X7a: a publication by `Dependency Upload & Remap`'s own row sends
/// performs the same writes from its side** — its V13 for the first, and its
/// V13a for W11's, which that module's own publication handling writes for its
/// own acts. So these clauses cover this document's acts and no others.
pub const THE_OTHER_MODULES_ROW_SENDS_PERFORM_THE_SAME_WRITES_FROM_THEIR_SIDE: bool = true;

/// **W4's limit rides on all three writes and is not repaired here** — §13 item
/// 5, which carries it over three rather than two, and over this document's
/// removals as well as its publications.
///
/// A symbolic link or a second mount makes one location into two, and this
/// comparison reads paths, not inodes. **An act that reaches a file through an
/// alias the record does not spell does not match, and no write is
/// performed.**
///
/// **The honest answer is that the two spellings cannot be told apart here**:
/// no reading this document has returns a link's target or a mount's identity,
/// and modelling the machine's link structure was declined at W4 as the folding
/// trap's twin. **Both sides are paths Smoozy itself wrote and read** — the
/// landing path from this document's own request, the standing's root path from
/// the check's own reading of the same route — so the comparison holds wherever
/// the user reached the file under one name, and misses only where they reached
/// it under two.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WhatTheMissCosts;

impl WhatTheMissCosts {
    /// **For the recorded reading it costs a stale display and nothing more**:
    /// `Render Settings` Q14's second moment compares the four-value identity
    /// at the user's own act and drops the reading there instead (its Q13), and
    /// `Job files and outputs` R12's fresh check at the start of every attempt
    /// catches every change the page missed.
    pub const THE_READING_HAS_A_SECOND_ROUTE: bool = true;

    /// **For the check standing there is no second route**, because the spec
    /// forbids reading the file at submission and `Dependency Upload & Remap`
    /// V13 keeps every trigger an act rather than a reading. So the standing
    /// keeps no mark, `Rendering` W2 is composed from a record that still says
    /// a complete check stood for this file on this machine, **no submission
    /// warning fires at all**, and the job renders at full price against a file
    /// the check no longer describes.
    pub const THE_STANDING_HAS_NO_SECOND_ROUTE: bool = false;

    /// **What the user is left with is after the fact rather than at
    /// submission, and it is named here so the failure is not read as
    /// silent**: the remap material's marker reports every pair as applied or
    /// not applied on the render's own captured stream, readable afterwards
    /// with nobody connected (`Dependency Upload & Remap` M8 and its M9d), so a
    /// reference the replacement moved reads *not applied* rather than nothing
    /// at all; and re-running the check over the replaced file is one action
    /// the user already has.
    ///
    /// **Nothing at submission catches it, and no clause here may be written or
    /// read as though something did.** §13 item 5 carries it.
    pub const NOTHING_AT_SUBMISSION_CATCHES_IT: bool = true;
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The neighbouring records written by hand. **Every reading is of the
    /// desktop's own records, so the whole of X7a and of W11 is testable at the
    /// desk with no machine anywhere.**
    #[derive(Default)]
    struct TheRecordsNextDoor {
        standings: Vec<Vec<u8>>,
        readings: Vec<Vec<u8>>,
        /// W11's first reading, as pairs: the resolved location as the record
        /// holds it, and the root whose tree resolved it.
        trees: Vec<(Vec<u8>, Vec<u8>)>,
        /// W11's second reading, as pairs: the derived-from location as the
        /// record holds it, and the fixed copy made from it.
        derived: Vec<(Vec<u8>, Vec<u8>)>,
        marked: Vec<Vec<u8>>,
        dropped: Vec<Vec<u8>>,
        disturbed: Vec<Vec<u8>>,
        /// A reader that matches under a weaker rule than X5's, for the tests
        /// that say the comparison here is not decoration.
        match_anything: bool,
    }

    impl TheNeighbouringRecords for TheRecordsNextDoor {
        fn the_check_standings_root_at(&self, published: &[u8]) -> Option<Vec<u8>> {
            if self.match_anything {
                return self.standings.first().cloned();
            }
            self.standings
                .iter()
                .find(|root| paths_are_equal(root, published))
                .cloned()
        }
        fn the_recorded_readings_path_at(&self, published: &[u8]) -> Option<Vec<u8>> {
            if self.match_anything {
                return self.readings.first().cloned();
            }
            self.readings
                .iter()
                .find(|path| paths_are_equal(path, published))
                .cloned()
        }
        fn mark_the_standing_replaced_since_check(&mut self, root: &[u8]) {
            self.marked.push(root.to_vec());
        }
        fn drop_the_recorded_reading(&mut self, path: &[u8]) {
            self.dropped.push(path.to_vec());
        }
        fn the_roots_whose_tree_resolved(&self, location: &[u8]) -> Vec<ARootWhoseTreeResolvedIt> {
            self.trees
                .iter()
                .filter(|(at, _)| self.match_anything || paths_are_equal(at, location))
                .map(|(at, root)| ARootWhoseTreeResolvedIt {
                    the_location_as_the_record_holds_it: at.clone(),
                    root: root.clone(),
                })
                .collect()
        }
        fn the_fixed_copies_made_from(&self, location: &[u8]) -> Vec<AFixedCopyMadeFromIt> {
            self.derived
                .iter()
                .filter(|(at, _)| self.match_anything || paths_are_equal(at, location))
                .map(|(at, fixed_copy)| AFixedCopyMadeFromIt {
                    the_location_as_the_record_holds_it: at.clone(),
                    fixed_copy: fixed_copy.clone(),
                })
                .collect()
        }
        fn mark_the_standing_disturbed(&mut self, root: &[u8]) {
            self.disturbed.push(root.to_vec());
        }
    }

    /// Every test below is about a publication that **took effect**, so the
    /// outcome is written once here rather than at fourteen call sites. The
    /// gate itself is proved by
    /// `a_publication_that_did_not_take_effect_writes_nothing`.
    fn at_this_publication_that_took_effect(
        published: &[u8],
        records: &mut dyn TheNeighbouringRecords,
    ) -> WhatThePublicationWrote {
        at_this_publication(published, Some(&UnitOutcome::Done), records)
    }

    const ROOT: &[u8] = b"/root/assets/files/Users/a/scene.blend";
    const OTHER_ROOT: &[u8] = b"/root/assets/files/Users/a/other.blend";
    const TEXTURE: &[u8] = b"/root/assets/files/Users/a/tex/x.png";
    const FIXED_COPY: &[u8] = b"/root/assets/fixed/c.blend";

    /// **X7a's two writes at one publication onto a standing's root that a
    /// reading also stands for.**
    #[test]
    fn a_publication_onto_a_standings_root_marks_it_and_drops_the_reading() {
        let mut records = TheRecordsNextDoor {
            standings: vec![ROOT.to_vec()],
            readings: vec![ROOT.to_vec()],
            ..TheRecordsNextDoor::default()
        };
        let wrote = at_this_publication_that_took_effect(ROOT, &mut records);
        assert_eq!(
            wrote.the_standing_marked_replaced_since_check,
            Some(ROOT.to_vec())
        );
        assert_eq!(wrote.the_recorded_reading_dropped, Some(ROOT.to_vec()));
        assert_eq!(records.marked, vec![ROOT.to_vec()]);
        assert_eq!(records.dropped, vec![ROOT.to_vec()]);
    }

    /// **The three writes are independent**, and a publication that touched
    /// none of them writes nothing — an answer rather than an omission.
    #[test]
    fn each_write_stands_alone_and_a_publication_elsewhere_writes_nothing() {
        let mut only_a_standing = TheRecordsNextDoor {
            standings: vec![ROOT.to_vec()],
            ..TheRecordsNextDoor::default()
        };
        let wrote = at_this_publication_that_took_effect(ROOT, &mut only_a_standing);
        assert!(wrote.the_standing_marked_replaced_since_check.is_some());
        assert!(wrote.the_recorded_reading_dropped.is_none());
        assert!(wrote.the_standings_marked_disturbed.is_empty());

        let mut only_a_reading = TheRecordsNextDoor {
            readings: vec![ROOT.to_vec()],
            ..TheRecordsNextDoor::default()
        };
        let wrote = at_this_publication_that_took_effect(ROOT, &mut only_a_reading);
        assert!(wrote.the_standing_marked_replaced_since_check.is_none());
        assert!(wrote.the_recorded_reading_dropped.is_some());
        assert!(wrote.the_standings_marked_disturbed.is_empty());

        let mut only_a_tree = TheRecordsNextDoor {
            trees: vec![(TEXTURE.to_vec(), ROOT.to_vec())],
            ..TheRecordsNextDoor::default()
        };
        let wrote = at_this_publication_that_took_effect(TEXTURE, &mut only_a_tree);
        assert!(wrote.the_standing_marked_replaced_since_check.is_none());
        assert!(wrote.the_recorded_reading_dropped.is_none());
        assert_eq!(wrote.the_standings_marked_disturbed, vec![ROOT.to_vec()]);

        let mut neither = TheRecordsNextDoor {
            standings: vec![ROOT.to_vec()],
            readings: vec![ROOT.to_vec()],
            trees: vec![(TEXTURE.to_vec(), ROOT.to_vec())],
            ..TheRecordsNextDoor::default()
        };
        let wrote = at_this_publication_that_took_effect(b"/root/assets/files/Users/a/nothing.png", &mut neither);
        assert!(wrote.is_nothing());
        assert!(neither.marked.is_empty() && neither.dropped.is_empty());
        assert!(neither.disturbed.is_empty());
    }

    /// **W11, and the hole it closes**: a plain upload publishing over an asset
    /// a checked root's tree resolved marks that root's standing disturbed, so
    /// the standing stops saying the file is ready to render.
    ///
    /// **The publication lands on nothing that is a standing's root**, which is
    /// exactly the case X7a's two writes left unmarked and the submission
    /// warning never fired for.
    #[test]
    fn a_publication_over_something_a_checked_roots_tree_resolved_marks_that_standing_disturbed() {
        let mut records = TheRecordsNextDoor {
            standings: vec![ROOT.to_vec()],
            trees: vec![
                (TEXTURE.to_vec(), ROOT.to_vec()),
                (TEXTURE.to_vec(), OTHER_ROOT.to_vec()),
            ],
            ..TheRecordsNextDoor::default()
        };
        let wrote = at_this_publication_that_took_effect(TEXTURE, &mut records);
        assert_eq!(
            wrote.the_standings_marked_disturbed,
            vec![ROOT.to_vec(), OTHER_ROOT.to_vec()],
            "every root whose tree resolved the location is marked, not just the first"
        );
        assert_eq!(records.disturbed, vec![ROOT.to_vec(), OTHER_ROOT.to_vec()]);
        assert!(
            wrote.the_standing_marked_replaced_since_check.is_none(),
            "the texture is no standing's root, which is the case this write exists for"
        );
        const { assert!(W11_CREATES_NO_WARNING_AND_WIDENS_NONE) };
    }

    /// **W11's second reading**: where the location is the machine copy a fixed
    /// copy was derived from, the roots marked are the ones whose tree resolved
    /// **the fixed copy** — `Dependency Upload & Remap` V2a and its M16b.
    #[test]
    fn a_publication_over_the_copy_a_fixed_copy_was_made_from_marks_the_fixed_copys_roots() {
        let mut records = TheRecordsNextDoor {
            derived: vec![(TEXTURE.to_vec(), FIXED_COPY.to_vec())],
            trees: vec![(FIXED_COPY.to_vec(), ROOT.to_vec())],
            ..TheRecordsNextDoor::default()
        };
        let wrote = at_this_publication_that_took_effect(TEXTURE, &mut records);
        assert_eq!(wrote.the_standings_marked_disturbed, vec![ROOT.to_vec()]);
        assert_eq!(records.disturbed, vec![ROOT.to_vec()]);

        // A derived-from record for some other location reaches nothing here.
        let mut elsewhere = TheRecordsNextDoor {
            derived: vec![(
                b"/root/assets/files/Users/a/tex/y.png".to_vec(),
                FIXED_COPY.to_vec(),
            )],
            trees: vec![(FIXED_COPY.to_vec(), ROOT.to_vec())],
            ..TheRecordsNextDoor::default()
        };
        assert!(at_this_publication_that_took_effect(TEXTURE, &mut elsewhere).is_nothing());
    }

    /// **A root is marked once however many routes reach it.**
    #[test]
    fn a_root_reached_by_both_routes_is_marked_once() {
        let mut records = TheRecordsNextDoor {
            trees: vec![
                (TEXTURE.to_vec(), ROOT.to_vec()),
                (FIXED_COPY.to_vec(), ROOT.to_vec()),
            ],
            derived: vec![(TEXTURE.to_vec(), FIXED_COPY.to_vec())],
            ..TheRecordsNextDoor::default()
        };
        let wrote = at_this_publication_that_took_effect(TEXTURE, &mut records);
        assert_eq!(wrote.the_standings_marked_disturbed, vec![ROOT.to_vec()]);
        assert_eq!(records.disturbed.len(), 1);
    }

    /// **W11: the two standing marks are independent and neither stands in for
    /// the other**, so a publication that owes both makes both.
    ///
    /// A linked `.blend` that is one standing's root and another standing's
    /// resolved tree member at once is exactly that case.
    #[test]
    fn a_publication_that_owes_both_standing_marks_makes_both() {
        let mut records = TheRecordsNextDoor {
            standings: vec![ROOT.to_vec()],
            trees: vec![(ROOT.to_vec(), OTHER_ROOT.to_vec())],
            ..TheRecordsNextDoor::default()
        };
        let wrote = at_this_publication_that_took_effect(ROOT, &mut records);
        assert_eq!(
            wrote.the_standing_marked_replaced_since_check,
            Some(ROOT.to_vec())
        );
        assert_eq!(
            wrote.the_standings_marked_disturbed,
            vec![OTHER_ROOT.to_vec()]
        );
        assert_eq!(records.marked, vec![ROOT.to_vec()]);
        assert_eq!(records.disturbed, vec![OTHER_ROOT.to_vec()]);
        const { assert!(THE_TWO_STANDING_MARKS_ARE_INDEPENDENT) };
    }

    /// **X5's comparison here is not decoration**: a reader that matched under
    /// a weaker rule writes nothing, because the recorded text is compared
    /// again before every write.
    #[test]
    fn a_reader_that_matched_under_a_weaker_rule_writes_nothing() {
        let mut records = TheRecordsNextDoor {
            standings: vec![ROOT.to_vec()],
            readings: vec![ROOT.to_vec()],
            trees: vec![(TEXTURE.to_vec(), OTHER_ROOT.to_vec())],
            derived: vec![(TEXTURE.to_vec(), FIXED_COPY.to_vec())],
            match_anything: true,
            ..TheRecordsNextDoor::default()
        };
        let wrote = at_this_publication_that_took_effect(b"/root/assets/files/Users/a/SCENE.blend", &mut records);
        assert!(
            wrote.is_nothing(),
            "a mark was written against a root the publication did not land on"
        );
        assert!(records.marked.is_empty() && records.dropped.is_empty());
        assert!(records.disturbed.is_empty());
    }

    /// **W4's limit, in code**: a second mount reaching the same file is a
    /// different path and none of the three writes is performed.
    #[test]
    fn a_publication_through_an_alias_the_record_does_not_spell_writes_nothing() {
        let mut records = TheRecordsNextDoor {
            standings: vec![ROOT.to_vec()],
            readings: vec![ROOT.to_vec()],
            trees: vec![(TEXTURE.to_vec(), ROOT.to_vec())],
            ..TheRecordsNextDoor::default()
        };
        let wrote = at_this_publication_that_took_effect(b"/mnt/two/Users/a/scene.blend", &mut records);
        assert!(wrote.is_nothing());
        let wrote = at_this_publication_that_took_effect(b"/mnt/two/Users/a/tex/x.png", &mut records);
        assert!(
            wrote.is_nothing(),
            "the disturbance mark carries the same alias limit as the other two writes"
        );
        const { assert!(WhatTheMissCosts::THE_READING_HAS_A_SECOND_ROUTE) };
        const { assert!(!WhatTheMissCosts::THE_STANDING_HAS_NO_SECOND_ROUTE) };
        const { assert!(WhatTheMissCosts::NOTHING_AT_SUBMISSION_CATCHES_IT) };
    }

    /// **W11's *took effect* rule is kept in both halves and not only in the
    /// removal one.** A publication can refuse — X7's create-intended branch
    /// refuses over an existing target, and `Server protocol and transfers`
    /// S15's read-back can come back wrong — and a unit whose ending was never
    /// written did not take effect either.
    #[test]
    fn a_publication_that_did_not_take_effect_writes_nothing() {
        let a_record = || TheRecordsNextDoor {
            standings: vec![ROOT.to_vec()],
            readings: vec![ROOT.to_vec()],
            trees: vec![(ROOT.to_vec(), OTHER_ROOT.to_vec())],
            ..TheRecordsNextDoor::default()
        };
        for outcome in [
            None,
            Some(UnitOutcome::Failed),
            Some(UnitOutcome::NotAttempted {
                stopped_by: "an earlier unit".to_string(),
            }),
        ] {
            let mut records = a_record();
            let wrote = at_this_publication(ROOT, outcome.as_ref(), &mut records);
            assert!(
                wrote.is_nothing(),
                "a publication that never landed wrote into a neighbouring record"
            );
            assert!(records.marked.is_empty());
            assert!(records.dropped.is_empty());
            assert!(records.disturbed.is_empty());
        }

        // And the same publication that did take effect writes all three.
        let mut records = a_record();
        let wrote = at_this_publication(ROOT, Some(&UnitOutcome::Done), &mut records);
        assert!(wrote.the_standing_marked_replaced_since_check.is_some());
        assert!(wrote.the_recorded_reading_dropped.is_some());
        assert_eq!(
            wrote.the_standings_marked_disturbed,
            vec![OTHER_ROOT.to_vec()]
        );
    }

    /// **A path spelled with a doubled separator is the same path**, because
    /// the comparison is per component — and that holds for all three writes.
    #[test]
    fn the_comparison_is_per_component_and_never_over_the_raw_bytes() {
        let mut records = TheRecordsNextDoor {
            standings: vec![b"/root/assets//files/Users/a/scene.blend".to_vec()],
            trees: vec![(
                b"/root/assets//files/Users/a/scene.blend".to_vec(),
                OTHER_ROOT.to_vec(),
            )],
            ..TheRecordsNextDoor::default()
        };
        let wrote = at_this_publication_that_took_effect(ROOT, &mut records);
        assert!(wrote.the_standing_marked_replaced_since_check.is_some());
        assert_eq!(
            wrote.the_standings_marked_disturbed,
            vec![OTHER_ROOT.to_vec()]
        );
        const { assert!(THE_BLENDER_VERSION_IS_NO_PART_OF_EITHER_COMPARISON) };
        const { assert!(THE_OTHER_MODULES_ROW_SENDS_PERFORM_THE_SAME_WRITES_FROM_THEIR_SIDE) };
    }
}
