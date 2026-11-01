//! Already exists, and the rule collisions are judged under — `File Browser`
//! §4.1 and §4.2, X1 to X8, Rows 191 and 192.
//!
//! **Constraint:** Rows 191, 192 and 197 — the whole-tree check before a byte
//! moves, the count as the headline, collisions judged file by file by name and
//! place, and a part-arrived file never counted as already being there. **Filed
//! as three rows; answered as one thing**, with the partial at
//! [`crate::partial`] and Keep both at [`crate::keep_both`].
//!
//! **Markers used below that belong to another document, each with the document
//! it belongs to.** `Server protocol and transfers` S16, S17, S15a, G4.
//! `Desktop app and delivery` C18. `Dependency Upload & Remap` Q8.

use std::time::SystemTime;

use smoozy_server_protocol::transfers::publication::Intent;

use crate::listing::{CompleteListing, EntryName};
use crate::permission::WhatSmoozyMet;

/// **X5: the comparison is byte-for-byte equality of names, per path
/// component, against the destination listings.**
///
/// That is the receiving filesystem's own rule on every rental this project
/// has observed — **four observation documents record it and the last of them
/// covers three machines across three sellers**: `Case.txt` and `case.txt` two
/// files, `JOBFILE.png`, `JobFile.png` and `jobfile.png` three files, all
/// seventeen hostile spellings kept distinct, and *"case-sensitive on all
/// three"* across the three-seller batch.
///
/// **On such a filesystem byte-equality *is* the filesystem's answer, with no
/// folding rule modelled and no normalisation applied.** Two spellings of one
/// accented name are two names here, exactly as the machine holds them.
pub fn names_are_equal(one: &EntryName, other: &EntryName) -> bool {
    one.as_bytes() == other.as_bytes()
}

/// **X5's rule applied to a whole path**: whole and exact, per component — no
/// folding, no normalisation, no canonicalising, no resolution of any kind.
pub fn paths_are_equal(one: &[u8], other: &[u8]) -> bool {
    components(one) == components(other)
}

/// A path's components, in order, empty ones dropped. **Bytes throughout.**
pub fn components(path: &[u8]) -> Vec<&[u8]> {
    path.split(|byte| *byte == b'/')
        .filter(|part| !part.is_empty())
        .collect()
}

/// Whether `path` equals or sits beneath `folder`, **compared per component
/// under X5's rule**.
///
/// A name that merely begins with the folder's text is not inside it, which is
/// what comparing components rather than prefixes buys.
pub fn is_inside(path: &[u8], folder: &[u8]) -> bool {
    let path = components(path);
    let folder = components(folder);
    path.len() >= folder.len() && path[..folder.len()] == folder[..]
}

/// One thing arriving, as the local enumeration of X1 gives it.
///
/// **The relative path is component for component**, and the names are read
/// **as stored on the user's disk** through the platform file adapter — the
/// machine side takes names as bytes for the same reason and under L1a, *"so
/// the two sides now hold the same rule"*.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Arriving {
    /// The path relative to what the user picked.
    pub relative: Vec<EntryName>,
    pub is_a_folder: bool,
}

impl Arriving {
    /// The landing path: the destination folder plus the relative path,
    /// component for component.
    pub fn landing_path(&self, destination: &[u8]) -> Vec<u8> {
        let mut path = Vec::new();
        for component in components(destination) {
            path.push(b'/');
            path.extend_from_slice(component);
        }
        for component in &self.relative {
            path.push(b'/');
            path.extend_from_slice(component.as_bytes());
        }
        path
    }

    /// The folder this thing lands in, relative to the destination.
    fn receiving_folder(&self) -> &[EntryName] {
        &self.relative[..self.relative.len().saturating_sub(1)]
    }

    fn name(&self) -> Option<&EntryName> {
        self.relative.last()
    }
}

/// **X3: what the comparison yields, per arriving thing — the spec's own
/// taxonomy, and there is no sixth.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Comparison {
    /// Nothing of that name is there.
    LandsFresh,
    /// **Collides with a file.**
    CollidesWithAFile,
    /// **Lands on a same-named folder, which cannot merge and is a
    /// collision.**
    LandsOnASameNamedFolder,
    /// **A folder on a file**, which cannot merge either.
    AFolderOnAFile,
    /// **Merges, for a folder onto a same-named folder.**
    Merges,
}

impl Comparison {
    /// **The headline is the count of these across the whole request** — every
    /// outcome that is neither fresh nor a merge.
    pub fn is_a_collision(self) -> bool {
        matches!(
            self,
            Comparison::CollidesWithAFile
                | Comparison::LandsOnASameNamedFolder
                | Comparison::AFolderOnAFile
        )
    }
}

/// What one receiving folder's reading answered.
///
/// **An accepted pass or nothing**: `Server protocol and transfers` §1.1 item
/// 2's best-effort continuation is refused here by L3a, and *"the
/// already-exists question starts no transfer until every required directory
/// has an accepted pass."*
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReceivingFolder {
    /// One open-to-end pass.
    Listing(CompleteListing),
    /// **X2: a receiving folder that does not exist ends its branch of the
    /// walk** — nothing beneath it can collide, and its whole subtree lands
    /// fresh.
    NotThere,
    /// **A reading that fails for permission is a refusal under §2.1 and the
    /// upload does not start until the user has an answer.**
    CouldNotBeRead(WhatSmoozyMet),
}

/// **X1's machine-side reading: one directory reading per receiving folder.**
///
/// Fact 7 makes that cheap — batched attributes, 52 round trips for 5010
/// entries — and it is the shape that separates failures (P1), which a stat per
/// landing place never could. **A stat per landing place is never issued**:
/// three hundred landing places are answered from the listings of the handful
/// of folders that hold them.
///
/// **It is asynchronous because the reading is the machine's**, and its real
/// end — [`crate::wiring::TheReadingChannel`] — is the file-transfer
/// subsystem's own directory reading on L1's one long-lived reading channel.
pub trait ReadsAReceivingFolder {
    fn read(&mut self, path: &[u8]) -> impl std::future::Future<Output = ReceivingFolder> + Send;
}

/// **X1's prohibition, stated as a constant so a test and the declaration table
/// can cite it.**
pub const A_STAT_PER_LANDING_PLACE_IS_NEVER_ISSUED: bool = true;

/// **One comparison, and the landing path it is about.**
///
/// **The path is not decoration.** This list is one comparison per **arriving
/// thing**, and X3's taxonomy has folders in it — *"merges, for a folder onto a
/// same-named folder"* and *"a folder on a file"* — while U1's enumeration is
/// **one entry per file**. **Two different populations**, so nothing may pair
/// them by position: a folder among the arriving things would give every entry
/// after it another file's answer, which is a wrong publication intent arriving
/// quietly. **The landing path is the key**, and it is the same key
/// [`crate::request::hand_over`] and
/// [`crate::request::UploadRequest::entry_landing_at`] already use.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AComparison {
    /// The landing path this comparison is about — the destination folder plus
    /// the arriving thing's relative path.
    pub landing_path: Vec<u8>,
    pub what: Comparison,
}

/// What the walk answered.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TheAnswer {
    /// One comparison per arriving thing, **each keyed by the landing path it
    /// is about**, in the enumeration's own order.
    pub comparisons: Vec<AComparison>,
    /// **X3's headline: the count of colliding things across the whole
    /// request.** The colliding names are listed beneath it (Design's box) and
    /// **the one answer covers them all.**
    pub colliding: usize,
    /// **How many of the arriving things are files**, which is what the
    /// question's *of these N files* counts (`Design/File Browser § When the
    /// files are already there`). The comparisons hold the folders too. After
    /// 0.1.8, the checks' leftovers, item 5: a folder of 4 files read *4 of
    /// these 6 files*.
    pub arriving_files: usize,
    /// **X4: the answer is as of the walk's moment, and no reservation is taken
    /// to freeze it.** The machine is the user's; something else may write
    /// between the walk and the landing. **What stands between that race and a
    /// silent overwrite is not the walk but the publication rule of X7.**
    pub as_of: SystemTime,
    /// How many directory readings it cost — one per receiving folder that
    /// exists, and never one per landing place.
    pub readings_taken: usize,
}

/// Why the question stopped without an answer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TheQuestionStopped {
    /// **X2.** *"A check that silently skipped an unreadable folder would
    /// report a count that is wrong in the quiet direction."* The upload does
    /// not start until the user has an answer.
    AFolderCouldNotBeRead { folder: Vec<u8>, met: WhatSmoozyMet },
}

/// **X1: both sides are walked before a byte moves, and the walk cannot be lazy
/// because the count is the headline.**
///
/// The local side is the enumeration passed in — every file inside every folder
/// the user pointed at. The machine side is one directory reading per receiving
/// folder: **the destination folder itself, and each folder of the arriving
/// tree's shape that already exists beneath it.**
///
/// **The reader is a type parameter rather than `dyn`**, because the reading is
/// the machine's and is therefore asynchronous, and an asynchronous trait
/// method has no object-safe form. That is the whole of what wiring the
/// machine side cost this function.
pub async fn walk<R: ReadsAReceivingFolder>(
    destination: &[u8],
    arriving: &[Arriving],
    reader: &mut R,
    at: SystemTime,
) -> Result<TheAnswer, TheQuestionStopped> {
    // Every receiving folder, relative to the destination, shortest first — so
    // a parent is always read before the folders beneath it and X2 can end a
    // branch before anything under it is asked for.
    let mut wanted: Vec<Vec<EntryName>> = Vec::new();
    for thing in arriving {
        let folder = thing.receiving_folder().to_vec();
        if !wanted.contains(&folder) {
            wanted.push(folder);
        }
    }
    wanted.sort_by_key(Vec::len);

    let mut read: Vec<(Vec<EntryName>, Option<CompleteListing>)> = Vec::new();
    let mut readings_taken = 0;
    for folder in wanted {
        // X2: a branch whose ancestor is not there ends here — nothing beneath
        // an absent folder can collide, and nothing is asked for it.
        let an_ancestor_is_absent = read.iter().any(|(held, listing)| {
            listing.is_none() && held.len() < folder.len() && folder[..held.len()] == held[..]
        });
        if an_ancestor_is_absent {
            read.push((folder, None));
            continue;
        }

        let path = path_of(destination, &folder);
        readings_taken += 1;
        match reader.read(&path).await {
            ReceivingFolder::Listing(listing) => read.push((folder, Some(listing))),
            ReceivingFolder::NotThere => read.push((folder, None)),
            ReceivingFolder::CouldNotBeRead(met) => {
                return Err(TheQuestionStopped::AFolderCouldNotBeRead { folder: path, met })
            }
        }
    }

    let mut comparisons = Vec::with_capacity(arriving.len());
    for thing in arriving {
        let folder = thing.receiving_folder();
        let listing = read
            .iter()
            .find(|(held, _)| held.as_slice() == folder)
            .and_then(|(_, listing)| listing.as_ref());
        comparisons.push(AComparison {
            landing_path: thing.landing_path(destination),
            what: compare(thing, listing),
        });
    }

    let colliding = comparisons
        .iter()
        .filter(|comparison| comparison.what.is_a_collision())
        .count();

    Ok(TheAnswer {
        comparisons,
        colliding,
        arriving_files: arriving.iter().filter(|thing| !thing.is_a_folder).count(),
        as_of: at,
        readings_taken,
    })
}

fn path_of(destination: &[u8], relative: &[EntryName]) -> Vec<u8> {
    let mut path = Vec::new();
    for component in components(destination) {
        path.push(b'/');
        path.extend_from_slice(component);
    }
    for component in relative {
        path.push(b'/');
        path.extend_from_slice(component.as_bytes());
    }
    path
}

fn compare(thing: &Arriving, listing: Option<&CompleteListing>) -> Comparison {
    let Some(name) = thing.name() else {
        return Comparison::LandsFresh;
    };
    // X2: the receiving folder does not exist, so its whole subtree lands fresh.
    let Some(listing) = listing else {
        return Comparison::LandsFresh;
    };
    match listing.holds(name) {
        None => Comparison::LandsFresh,
        Some(there) => match (thing.is_a_folder, there.kind.is_a_folder()) {
            (true, true) => Comparison::Merges,
            (true, false) => Comparison::AFolderOnAFile,
            (false, true) => Comparison::LandsOnASameNamedFolder,
            (false, false) => Comparison::CollidesWithAFile,
        },
    }
}

/// **X6: Row 303's *asking rather than modelling* was weighed, and its
/// mechanism was declined for the pre-flight while its principle is kept at
/// publication.**
///
/// The exclusive-create route answers *is this name taken* perfectly — **but
/// asked for hundreds of names before anything is sent it would strew
/// zero-byte reservations across a machine the user browses, files a crash
/// leaves behind and no spec grants**; that is Method's third escalation
/// arriving on the server side, and it was checked and declined (§14). **What
/// is kept instead is X7, where the filesystem itself still gets the last word
/// at the only moment a wrong answer could destroy anything.**
pub const NO_RESERVATION_IS_TAKEN_IN_THE_PRE_FLIGHT: bool = true;

/// **U4's four answers.** The unit is defined here because X7 is composed from
/// it; **its durable write into the request record is §3.2's**, which is built
/// at [`crate::request::apply_the_collision_answer`].
///
/// *"It is given once for the whole request and is never asked again for the
/// life of the request: a drop, a repair an hour later, and every resume read
/// the recorded answer."*
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CollisionAnswer {
    Replace,
    KeepBoth,
    SkipExisting,
    Stop,
}

/// **X7: publication is the moment the filesystem answers, and the answer the
/// user gave decides which rename is used.**
///
/// - **create-intended** — the walk found the landing name absent, or Keep both
///   composed a fresh name. Publication uses the **base rename**, which the
///   protocol itself refuses over an existing target (`Server protocol and
///   transfers` S16, observed: status 4, `Failure`). Where the destination
///   filesystem folds two spellings this document compared as distinct — or
///   something landed in the race window — **the rename refuses rather than
///   replaces**: the transfer ends **stopped** with the truth said, *the
///   machine already holds something under this name*, nothing overwritten.
///   **On every observed rental this branch never fires; on a folding one it is
///   what keeps Replace-by-accident impossible.**
/// - **replace-intended** — the file collided and the user answered Replace;
///   for a transfer `Dependency Upload & Remap` arranged, its Q8's replace sets
///   the same intent from its side. Publication uses the POSIX-rename extension
///   route S16 establishes, **replacing deliberately. Row 303 forbids silence,
///   not replacement.**
///
/// **A create-intended publication never takes S17's remove-then-rename route**,
/// and the parameter is what carries that: applying S17 to every publication
/// would remove the folded twin, or the file that landed in X4's admitted race
/// window, and rename over it — **the silent Replace-by-accident this clause
/// exists to make impossible, arriving through the clause meant to prevent
/// it.** That scoping is `Server protocol and transfers` S17's own opening
/// sentence and is taken rather than restated;
/// [`smoozy_server_protocol::transfers::publication::which_operation`] is the
/// one place it is decided.
///
/// **This is supplied to `Server protocol and transfers` as a per-transfer
/// parameter of its own publication step** — its S15a declares the parameter
/// itself: every publication carries a caller-declared intent, two intents, no
/// third, no default. **That is the shape a builder consumes**, which is why
/// this function returns that document's own type rather than one of ours.
///
/// **`None` where the unit is not published by this request at all** — a
/// skipped unit, and a unit the user answered Stop for.
pub fn publication_intent(comparison: Comparison, answer: CollisionAnswer) -> Option<Intent> {
    if !comparison.is_a_collision() {
        // The walk found the landing name absent, or the arriving folder merges
        // into a same-named one and only the files beneath it are published.
        return Some(Intent::CreateIntended);
    }
    match answer {
        // Keep both composes a fresh name, so what is published is create-
        // intended and the generated name is checked and backstopped like any
        // other (X9).
        CollisionAnswer::KeepBoth => Some(Intent::CreateIntended),
        CollisionAnswer::Replace => Some(Intent::ReplaceIntended),
        CollisionAnswer::SkipExisting | CollisionAnswer::Stop => None,
    }
}

/// **X8: what the refusal cannot say is which stored name is in the way**,
/// because no folding rule is modelled and none is documented — **the same
/// honest limit `Desktop app and delivery` C18 accepts for Windows.**
///
/// *"The failure row names the landing name and that the machine holds
/// something under it; the user's remedy is the browser itself, where the
/// folder's listing shows what is there."*
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TheMachineHoldsSomethingUnderThisName {
    /// The landing name as Smoozy composed it. **Never a claim about which
    /// stored name is in the way.**
    pub the_landing_name: Vec<u8>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::listing::{ApparentSize, Kind, ListedEntry};

    fn name(bytes: &[u8]) -> EntryName {
        EntryName::from_the_machine(bytes.to_vec())
    }

    fn listing(entries: &[(&[u8], Kind)]) -> CompleteListing {
        CompleteListing::of(
            entries
                .iter()
                .map(|(bytes, kind)| ListedEntry {
                    name: name(bytes),
                    size: ApparentSize::of(1),
                    last_changed: None,
                    kind: *kind,
                })
                .collect(),
            SystemTime::UNIX_EPOCH,
            false,
        )
    }

    struct Machine {
        folders: Vec<(Vec<u8>, ReceivingFolder)>,
        reads: Vec<Vec<u8>>,
    }

    impl ReadsAReceivingFolder for Machine {
        async fn read(&mut self, path: &[u8]) -> ReceivingFolder {
            self.reads.push(path.to_vec());
            self.folders
                .iter()
                .find(|(held, _)| held == path)
                .map(|(_, answer)| answer.clone())
                .unwrap_or(ReceivingFolder::NotThere)
        }
    }

    fn a_file(relative: &[&[u8]]) -> Arriving {
        Arriving {
            relative: relative.iter().map(|bytes| name(bytes)).collect(),
            is_a_folder: false,
        }
    }

    fn a_folder(relative: &[&[u8]]) -> Arriving {
        Arriving {
            relative: relative.iter().map(|bytes| name(bytes)).collect(),
            is_a_folder: true,
        }
    }

    #[test]
    fn capitals_are_told_apart_and_nothing_is_normalised() {
        assert!(names_are_equal(&name(b"Case.txt"), &name(b"Case.txt")));
        assert!(!names_are_equal(&name(b"Case.txt"), &name(b"case.txt")));
        // Two spellings of one accented name are two names here — the one
        // code point, and the letter with a combining mark after it.
        assert!(!names_are_equal(
            &name("\u{e9}.png".as_bytes()),
            &name("e\u{301}.png".as_bytes())
        ));
    }

    #[test]
    fn a_path_is_compared_per_component_and_never_as_a_prefix() {
        assert!(paths_are_equal(b"/jobs/a", b"/jobs/a"));
        assert!(paths_are_equal(b"/jobs//a", b"/jobs/a"));
        assert!(!paths_are_equal(b"/jobs/a", b"/jobs/A"));
        assert!(is_inside(b"/jobs/a/b", b"/jobs"));
        assert!(is_inside(b"/jobs", b"/jobs"));
        assert!(!is_inside(b"/jobsibling/a", b"/jobs"));
        assert!(!is_inside(b"/jobs", b"/jobs/a"));
    }

    #[tokio::test]
    async fn the_taxonomy_is_the_specs_own_five() {
        let mut machine = Machine {
            folders: vec![(
                b"/dest".to_vec(),
                ReceivingFolder::Listing(listing(&[
                    (b"there.txt", Kind::File),
                    (b"assets", Kind::Folder),
                ])),
            )],
            reads: Vec::new(),
        };
        let arriving = vec![
            a_file(&[b"fresh.txt"]),
            a_file(&[b"there.txt"]),
            a_file(&[b"assets"]),
            a_folder(&[b"there.txt"]),
            a_folder(&[b"assets"]),
        ];
        let answer = walk(b"/dest", &arriving, &mut machine, SystemTime::UNIX_EPOCH)
            .await
            .unwrap();
        assert_eq!(
            answer
                .comparisons
                .iter()
                .map(|comparison| comparison.what)
                .collect::<Vec<_>>(),
            vec![
                Comparison::LandsFresh,
                Comparison::CollidesWithAFile,
                Comparison::LandsOnASameNamedFolder,
                Comparison::AFolderOnAFile,
                Comparison::Merges,
            ]
        );
        // **And each one says which landing path it is about**, so nothing
        // downstream pairs this list to the enumeration by position.
        assert_eq!(
            answer
                .comparisons
                .iter()
                .map(|comparison| comparison.landing_path.clone())
                .collect::<Vec<_>>(),
            vec![
                b"/dest/fresh.txt".to_vec(),
                b"/dest/there.txt".to_vec(),
                b"/dest/assets".to_vec(),
                b"/dest/there.txt".to_vec(),
                b"/dest/assets".to_vec(),
            ]
        );
        assert_eq!(answer.colliding, 3);
    }

    /// **Test 5 of the *Show hidden files* plan text: an upload landing on an
    /// existing hidden file raises the already-exists question and counts
    /// it** — L21: *"an already-exists count includes a hidden collision"*,
    /// because X1's walk reads the machine whole and never takes the choice.
    #[tokio::test]
    async fn a_hidden_file_already_there_is_a_collision_and_is_counted() {
        let mut machine = Machine {
            folders: vec![(
                b"/dest".to_vec(),
                ReceivingFolder::Listing(listing(&[(b".env", Kind::File), (b".cache", Kind::Folder)])),
            )],
            reads: Vec::new(),
        };
        let arriving = vec![a_file(&[b".env"]), a_folder(&[b".cache"]), a_file(&[b"scene.blend"])];
        let answer = walk(b"/dest", &arriving, &mut machine, SystemTime::UNIX_EPOCH)
            .await
            .unwrap();
        assert_eq!(
            answer.comparisons.iter().map(|comparison| comparison.what).collect::<Vec<_>>(),
            vec![Comparison::CollidesWithAFile, Comparison::Merges, Comparison::LandsFresh]
        );
        // **A folder landing on a same-named folder merges and is not a
        // collision** (the taxonomy above), so the count is the hidden file.
        assert_eq!(answer.colliding, 1, "a hidden name already there was not counted");
        assert_eq!(answer.arriving_files, 2, "a folder is not one of the files");
    }

    #[tokio::test]
    async fn three_hundred_landing_places_cost_one_reading_of_the_folder_that_holds_them() {
        let mut machine = Machine {
            folders: vec![(
                b"/dest".to_vec(),
                ReceivingFolder::Listing(listing(&[(b"file-7", Kind::File)])),
            )],
            reads: Vec::new(),
        };
        let arriving: Vec<Arriving> = (0..300)
            .map(|n| {
                let name = format!("file-{n}");
                a_file(&[name.as_bytes()])
            })
            .collect();
        let answer = walk(b"/dest", &arriving, &mut machine, SystemTime::UNIX_EPOCH)
            .await
            .unwrap();
        assert_eq!(answer.readings_taken, 1);
        assert_eq!(machine.reads.len(), 1);
        assert_eq!(answer.colliding, 1);
        const { assert!(A_STAT_PER_LANDING_PLACE_IS_NEVER_ISSUED) };
    }

    #[tokio::test]
    async fn a_receiving_folder_that_is_not_there_ends_its_branch_and_nothing_beneath_it_is_asked_for(
    ) {
        let mut machine = Machine {
            folders: vec![
                (b"/dest".to_vec(), ReceivingFolder::Listing(listing(&[]))),
                (b"/dest/tree".to_vec(), ReceivingFolder::NotThere),
            ],
            reads: Vec::new(),
        };
        let arriving = vec![
            a_folder(&[b"tree"]),
            a_file(&[b"tree", b"a.png"]),
            a_file(&[b"tree", b"deep", b"b.png"]),
        ];
        let answer = walk(b"/dest", &arriving, &mut machine, SystemTime::UNIX_EPOCH)
            .await
            .unwrap();
        assert_eq!(
            answer
                .comparisons
                .iter()
                .map(|comparison| comparison.what)
                .collect::<Vec<_>>(),
            vec![
                Comparison::LandsFresh,
                Comparison::LandsFresh,
                Comparison::LandsFresh
            ]
        );
        // The destination and the absent folder — and never `/dest/tree/deep`,
        // whose branch ended above it.
        assert_eq!(machine.reads.len(), 2);
        assert!(!machine.reads.iter().any(|path| path == b"/dest/tree/deep"));
    }

    #[tokio::test]
    async fn an_unreadable_folder_stops_the_question_rather_than_being_skipped() {
        // The count wrong in the quiet direction is the fault X2 exists
        // against.
        let mut machine = Machine {
            folders: vec![
                (
                    b"/dest".to_vec(),
                    ReceivingFolder::Listing(listing(&[(b"tree", Kind::Folder)])),
                ),
                (
                    b"/dest/tree".to_vec(),
                    ReceivingFolder::CouldNotBeRead(WhatSmoozyMet::Refused),
                ),
            ],
            reads: Vec::new(),
        };
        let arriving = vec![a_folder(&[b"tree"]), a_file(&[b"tree", b"a.png"])];
        let stopped = walk(b"/dest", &arriving, &mut machine, SystemTime::UNIX_EPOCH)
            .await
            .unwrap_err();
        assert_eq!(
            stopped,
            TheQuestionStopped::AFolderCouldNotBeRead {
                folder: b"/dest/tree".to_vec(),
                met: WhatSmoozyMet::Refused
            }
        );
    }

    #[tokio::test]
    async fn the_walk_carries_its_moment_and_takes_no_reservation() {
        let mut machine = Machine {
            folders: vec![(b"/dest".to_vec(), ReceivingFolder::Listing(listing(&[])))],
            reads: Vec::new(),
        };
        let at = SystemTime::UNIX_EPOCH + std::time::Duration::from_secs(99);
        let answer = walk(b"/dest", &[a_file(&[b"a.txt"])], &mut machine, at)
            .await
            .unwrap();
        assert_eq!(answer.as_of, at);
        const { assert!(NO_RESERVATION_IS_TAKEN_IN_THE_PRE_FLIGHT) };
    }

    #[test]
    fn a_fresh_landing_is_create_intended_whatever_the_answer_was() {
        for answer in [
            CollisionAnswer::Replace,
            CollisionAnswer::KeepBoth,
            CollisionAnswer::SkipExisting,
            CollisionAnswer::Stop,
        ] {
            assert_eq!(
                publication_intent(Comparison::LandsFresh, answer),
                Some(Intent::CreateIntended)
            );
        }
    }

    #[test]
    fn replace_replaces_deliberately_and_keep_both_creates() {
        assert_eq!(
            publication_intent(Comparison::CollidesWithAFile, CollisionAnswer::Replace),
            Some(Intent::ReplaceIntended)
        );
        assert_eq!(
            publication_intent(Comparison::CollidesWithAFile, CollisionAnswer::KeepBoth),
            Some(Intent::CreateIntended)
        );
        assert_eq!(
            publication_intent(Comparison::CollidesWithAFile, CollisionAnswer::SkipExisting),
            None
        );
        assert_eq!(
            publication_intent(Comparison::CollidesWithAFile, CollisionAnswer::Stop),
            None
        );
    }

    #[test]
    fn a_create_intended_publication_never_takes_the_remove_then_rename_route() {
        use smoozy_server_protocol::transfers::publication::{which_operation, Operation};
        // With and without the extension, on the machines where it is absent —
        // which is the case the hazard is about.
        for announced in [true, false] {
            assert_eq!(
                which_operation(Intent::CreateIntended, announced),
                Operation::TheBaseRename
            );
        }
        assert_eq!(
            which_operation(Intent::ReplaceIntended, false),
            Operation::RemoveTheFinalNameThenTheBaseRename
        );
    }

    #[test]
    fn a_landing_path_is_the_destination_plus_the_relative_path_component_for_component() {
        let thing = a_file(&[b"tree", b"a.png"]);
        assert_eq!(thing.landing_path(b"/dest"), b"/dest/tree/a.png".to_vec());
        assert_eq!(thing.landing_path(b"/dest/"), b"/dest/tree/a.png".to_vec());
    }
}
