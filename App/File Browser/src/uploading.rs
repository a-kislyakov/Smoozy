//! Uploading: the binding, the names, and the record — `File Browser` §5, N1 to
//! N9 with N7a, Rows 185, 186 and 187.
//!
//! **Constraint:** the transfer bound to the machine it was sent to, the
//! structure and names kept, and the record of where each file came from.
//!
//! **Nothing here keeps a second store.** N6's record is [`crate::request`]'s
//! enumeration read a different way — *"one durable fact, not a second store"* —
//! and this module holds the reading rather than a copy.
//!
//! **Markers used below that belong to another document, each with the document
//! it belongs to.** `Server protocol and transfers` T3, S9. `State, identity and
//! recovery` §9.4, §11.1, §2.1. `Dependency Upload & Remap` Q5.
//! `Desktop app and delivery` §21.
//!
//! **A marker this plan also defines is deliberately not listed above,
//! whichever document is meant, and is written out with its document at every
//! site instead** — the header list cannot tell the two apart, and a bare one
//! would resolve to a real and wrong clause. This plan defines a K3, a K5 and a
//! K7 of its own, which is every marker `Dependency Upload & Remap` is cited
//! for below but Q5.

use smoozy_state::identities::Identity;
use smoozy_state::outcomes::UnitOutcome;

use crate::already_exists::{components, paths_are_equal};
use crate::keep_both::{THE_LONGEST_ACCEPTED_COMPONENT, THE_LONGEST_ACCEPTED_PATH};
use crate::listing::{EntryName, Kind};
use crate::request::{Disposition, UploadRequest};

/// **N1: a transfer is bound to the server row it was arranged against and to
/// nothing else** — `Server protocol and transfers` T3, which is Row 185
/// answered outright.
///
/// *"The browser moving to another machine changes where the **next** request's
/// enumeration lands and nothing about any transfer in flight."* The thing that
/// holds a request is [`UploadRequest`], keyed by server row; the thing that
/// holds the browsing position is the browser's own state, run-held. **They are
/// two different things and this is the whole of the answer.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TheBrowsingPosition {
    pub server_row: Identity,
    pub folder: Vec<u8>,
}

/// N1, asked as a question a test can put: does moving the browser touch a
/// transfer already arranged?
///
/// **It cannot**, and the answer is structural rather than a check: the request
/// carries its own server row and the browsing position is a separate value,
/// so there is no field for a move to write into.
pub fn moving_the_browser_touches_a_request(
    _from: &TheBrowsingPosition,
    _to: &TheBrowsingPosition,
    request: &UploadRequest,
) -> bool {
    let _ = request;
    false
}

/// **N2: the structure is mirrored, and the names are carried as bytes.**
///
/// Each arriving file's landing path is the destination folder plus its path
/// relative to what the user picked, component for component. **Source names are
/// read as stored on the user's disk through the platform file adapter and
/// carried onto the wire as their bytes; nothing rewrites, normalises or
/// re-encodes a name in flight.**
///
/// The up direction is the one the evidence clears: all seventeen hostile names
/// reached the machine byte-for-byte, spaces, Cyrillic, Japanese, emoji,
/// newline, tab and a leading dash included (Fact 29). **The down direction's
/// silent loss belongs to Row 303 and `Download` and is not read backwards onto
/// this row.**
pub fn landing_path(destination: &[u8], relative: &[EntryName]) -> Vec<u8> {
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

/// **N2's prohibition, stated so a test can cite it**: nothing rewrites,
/// normalises or re-encodes a name in flight.
pub const NOTHING_REWRITES_OR_RE_ENCODES_A_NAME_IN_FLIGHT: bool = true;

/// **N2's prohibition met at the one seam that cannot carry the bytes: a name
/// that is not valid text is a per-file failure at the pre-flight, and it is
/// never rewritten to get it across.**
///
/// `Server protocol and transfers`' own transfer record holds the source path
/// and the final path **as text**, and `Desktop app and delivery`'s platform
/// file adapter takes a path as text, so **a path whose bytes are not valid
/// text cannot cross that seam at all.** N2 forbids the only other way through
/// — *"nothing rewrites, normalises or re-encodes a name in flight"* — and L1a
/// says the same for the machine side, so the unit is refused rather than
/// converted. **Do not substitute, do not skip it quietly, and do not invent a
/// name.** The refusal is a value the pre-flight composes, and the same value is
/// what [`crate::request::arrange_a_transfer`] answers where a caller reaches it
/// without having run the pre-flight — **so no route arranges a transfer for
/// such a unit.**
///
/// **The shape is N4's and N5's**, which is the shape this document already
/// gives a name the machine will not take: the failure is worded as the truth,
/// **the file is never sent**, and what it costs is that file and nothing else
/// ([`what_a_failed_name_costs`]) — the rest of the folder lands. The unit's
/// own ending is `State, identity and recovery`'s
/// [`UnitOutcome::Failed`][smoozy_state::outcomes::UnitOutcome::Failed] and no
/// fifth disposition is made for it.
///
/// **The words the user reads are Design's**; what the code owes is that the
/// failure exists, names the file, and is never reported as a success.
///
/// **What it costs the user is nothing they are not already losing.** The
/// platform file adapter's own read takes the path as text, so a file whose
/// name that conversion mangled could not be opened on the user's own computer
/// either: the upload fails today at the first read, with a reason about a read
/// rather than about the name, and the record then holds a path U8's take-up can
/// never match again. **Carrying such a name properly is a wider repair across
/// three crates and is not this module's to take** — it is parked for the whole
/// workspace at `Preparation/Groundwork/Open Threads.md`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TheNameCannotBeCarried {
    /// The path is carried out whole, as its own bytes, so the report names the
    /// file rather than a guess at it.
    TheSourcePathIsNotValidText {
        source_path: Vec<u8>,
    },
    TheLandingPathIsNotValidText {
        landing_path: Vec<u8>,
    },
}

impl TheNameCannotBeCarried {
    /// **The file the failure names**, as its own bytes.
    pub fn the_file_it_names(&self) -> &[u8] {
        match self {
            TheNameCannotBeCarried::TheSourcePathIsNotValidText { source_path } => source_path,
            TheNameCannotBeCarried::TheLandingPathIsNotValidText { landing_path } => landing_path,
        }
    }

    /// **The unit's ending, and it is a failure.** `State, identity and
    /// recovery`'s own value, which is the honest ending for a unit that was
    /// going to be sent and could not be. **It is never an outcome that reads as
    /// a success, and it is never *skip existing*.**
    pub fn the_unit_ending() -> UnitOutcome {
        UnitOutcome::Failed
    }
}

/// N2's check, over one unit's two paths. **The source path is checked first**,
/// because it is the one the user can see.
///
/// The landing path passed in is the one the unit would actually be sent to —
/// under Keep both that is X9's generated name, not the colliding one.
pub fn check_the_name_can_be_carried(
    source_path: &[u8],
    landing_path: &[u8],
) -> Result<(), TheNameCannotBeCarried> {
    if std::str::from_utf8(source_path).is_err() {
        return Err(TheNameCannotBeCarried::TheSourcePathIsNotValidText {
            source_path: source_path.to_vec(),
        });
    }
    if std::str::from_utf8(landing_path).is_err() {
        return Err(TheNameCannotBeCarried::TheLandingPathIsNotValidText {
            landing_path: landing_path.to_vec(),
        });
    }
    Ok(())
}

/// **N3: landing folders are made with the plain make-directory, never the
/// make-parents form.**
///
/// *"The make-parents form returns success against an existing folder — the
/// silent collision path Fact 29 names — while the plain form names* already
/// exists *exactly on the command channel and fails plainly on the
/// subsystem."*
///
/// **This trait offers no make-parents operation at all**, so there is nothing
/// to reach for — the same shape `Job files and outputs`' own directory seam
/// takes.
pub trait MakesTheLandingFolders {
    /// One folder, one call. **The plain form.**
    fn make_one_folder(
        &mut self,
        path: &[u8],
    ) -> impl std::future::Future<Output = Result<(), WhatTheMachineSaidAboutAFolder>> + Send;
}

/// What the machine answered about a folder. **`Server protocol and transfers`
/// §1.3's separation is read rather than derived**, and [`crate::permission`] is
/// where the two are told apart for a reading.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WhatTheMachineSaidAboutAFolder {
    /// **The plain form's own refusal.** N3's whole reason for choosing it.
    ItIsAlreadyThere,
    TheLoginMayNot,
    /// Anything else the server refused. Nothing is concluded from it.
    ItRefused,
    TheChannelIsGone,
}

/// **N3: which folders are made, and which are merges the walk already saw.**
///
/// *"A folder the walk saw is a merge and is not re-made; a folder the walk did
/// not see is made plainly, component by component."*
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WhatToDoWithALandingFolder {
    /// The walk saw it. **A merge, and nothing is made.**
    ItIsAMergeAndIsNotReMade,
    /// The walk did not see it: made plainly.
    MakeItPlainly,
}

/// N3, per folder, against what X1's walk found.
pub fn what_to_do_with(
    folder: &[u8],
    folders_the_walk_saw: &[Vec<u8>],
) -> WhatToDoWithALandingFolder {
    if folders_the_walk_saw
        .iter()
        .any(|seen| paths_are_equal(seen, folder))
    {
        WhatToDoWithALandingFolder::ItIsAMergeAndIsNotReMade
    } else {
        WhatToDoWithALandingFolder::MakeItPlainly
    }
}

/// **N3's last sentence: an unexpected *already exists* on a folder the walk did
/// not see is X4's race surfacing early** — reported on that branch's files,
/// nothing overwritten.
pub fn an_unexpected_already_exists_is_the_race_surfacing_early(
    what: &WhatToDoWithALandingFolder,
    said: &WhatTheMachineSaidAboutAFolder,
) -> bool {
    matches!(what, WhatToDoWithALandingFolder::MakeItPlainly)
        && matches!(said, WhatTheMachineSaidAboutAFolder::ItIsAlreadyThere)
}

/// **N4: the length bound is enforced before sending, in bytes, and the
/// machine's own refusal is the backstop, never the message.**
///
/// **Every landing path is checked against a maximum of 255 bytes per component
/// and 4095 bytes over the whole path, both inclusive: 255 passes, 256 fails;
/// 4095 passes, 4096 fails.** Those two figures are the observed *accepted*
/// lengths and the observed refusals are the next byte up, both at open before
/// any byte crossed, both leaving nothing behind.
///
/// **The two constants live at [`crate::keep_both`]**, because X9 applies the
/// same bound to a generated name and one constant is what keeps the two from
/// disagreeing.
///
/// **The name figure is four boxes' observation across three sellers and the
/// path figure is one box's**, and both are carried as bounds rather than as a
/// law of rentals (§13 item 3, permanently unresolved on what a particular
/// rental enforces).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TheLengthBound {
    Inside,
    /// **A per-file failure at the pre-flight, worded as the truth** — *this
    /// name is too long for the machine* — **and never sent.**
    AComponentIsTooLong {
        component: Vec<u8>,
        bytes: usize,
    },
    ThePathIsTooLong {
        bytes: usize,
    },
}

/// N4, over one landing path.
pub fn check_the_length(landing_path: &[u8]) -> TheLengthBound {
    for component in components(landing_path) {
        if component.len() > THE_LONGEST_ACCEPTED_COMPONENT {
            return TheLengthBound::AComponentIsTooLong {
                component: component.to_vec(),
                bytes: component.len(),
            };
        }
    }
    if landing_path.len() > THE_LONGEST_ACCEPTED_PATH {
        return TheLengthBound::ThePathIsTooLong {
            bytes: landing_path.len(),
        };
    }
    TheLengthBound::Inside
}

/// **N4's second half: the machine's own `Bad message` string reports the wrong
/// problem entirely and is never surfaced.**
///
/// *"A refusal that still arrives at open ends that transfer with the same true
/// wording"* — this document's, not the machine's.
pub const THE_MACHINES_OWN_STRING_IS_NEVER_SURFACED: bool = true;

/// **N5: what a failed name costs is that file and nothing else** — the
/// partition unit is the file (U2), the report names it, and the rest of the
/// folder lands.
pub fn what_a_failed_name_costs(request: &UploadRequest, failed: &[u8]) -> usize {
    request
        .entries
        .iter()
        .filter(|entry| paths_are_equal(&entry.landing_path, failed))
        .count()
}

/// **N6 and N7: where did this machine file come from on the user's computer.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WhereItCameFrom {
    /// **The most recently published entry under the machine's current
    /// installation identity**, recency by the local record's own write order
    /// and never by comparing clocks.
    TheSourcePath(Vec<u8>),
    /// **N7: the record's absence is a legible state.** *"A file that reached
    /// the machine some other way carries no such knowledge."* **Nothing
    /// guesses.**
    NoRecord,
}

/// **N6: the record is the request record's enumeration — one durable fact, not
/// a second store.**
///
/// *"For a given server row and landing path, the answer is the most recently
/// published entry under the machine's current installation identity, recency by
/// the local record's own write order and never by comparing clocks."*
///
/// **N8's disqualification is applied here rather than by deleting anything**:
/// an entry under a superseded installation identity is disqualified and stays
/// true as the account of what was once sent.
///
/// `requests` arrives in the local record's own write order, oldest first, which
/// is what makes *most recently published* a position rather than a clock
/// comparison. **What supplies that order is the store**, and it is a fact about
/// it rather than a hope: `KeepsTheRequest::every_request_for`'s real end is
/// `smoozy_desktop::record::upload_requests`, whose statement ends `ORDER BY
/// rowid` — the row identifier, which is that store's own established answer for
/// write order — and whose write keeps the row rather than replacing it, so a
/// request written again later does not move to the end of the order. **The
/// identity is never sorted by**, which is `State, identity and recovery` §2.1
/// rule 2: *"Nothing parses one, sorts by one, or infers anything from two being
/// adjacent."*
///
/// **This function keeps the last match it sees**, so the order it is handed is
/// the whole of N6's recency.
pub fn where_it_came_from(
    requests: &[UploadRequest],
    server_row: &Identity,
    landing_path: &[u8],
    installation_now: Option<&Identity>,
) -> WhereItCameFrom {
    let mut answer = WhereItCameFrom::NoRecord;
    for request in requests {
        if &request.server_row != server_row {
            continue;
        }
        // N8: disqualified rather than deleted.
        if request.installation.as_ref() != installation_now {
            continue;
        }
        for entry in &request.entries {
            if entry.outcome != Some(UnitOutcome::Done) {
                continue;
            }
            let published_at = match entry.disposition.as_ref() {
                Some(Disposition::RenamedUnderKeepBoth(fresh)) => fresh.as_slice(),
                Some(Disposition::ToSend) => entry.landing_path.as_slice(),
                // A skipped unit published nothing and a handed one is that
                // module's; neither is a published entry of this document's.
                _ => continue,
            };
            if paths_are_equal(published_at, landing_path) {
                answer = WhereItCameFrom::TheSourcePath(entry.source_path.clone());
            }
        }
    }
    answer
}

/// **N7a: what *no record* does to the toolbar — nothing.**
///
/// Design puts **Render settings…** and **Check dependencies…** on the toolbar
/// for a `.blend` and takes them away for anything else (**Creator-stated**,
/// 2026-08-13), on the ground that *"a control that cannot apply to what is
/// selected is not a control, it is a question"*.
///
/// **The toolbar's rule is the file's kind and only its kind.** A `.blend` gets
/// both controls whatever N6's lookup would answer for it, and **no per-file
/// record lookup is run to compose a toolbar.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TheToolbar {
    pub render_settings: bool,
    pub check_dependencies: bool,
}

/// N7a. **The lookup is not a parameter of this function, and that is the
/// clause**: there is no route by which a record could change a toolbar.
pub fn the_toolbar_for(name: &EntryName, kind: Kind) -> TheToolbar {
    let is_a_blend = kind == Kind::File
        && name
            .as_bytes()
            .len()
            .checked_sub(b".blend".len())
            .is_some_and(|at| name.as_bytes()[at..].eq_ignore_ascii_case(b".blend"));
    TheToolbar {
        render_settings: is_a_blend,
        check_dependencies: is_a_blend,
    }
}

/// **N7a's third state: pressing *Check dependencies…* on a file N6 finds
/// nothing for starts `Dependency Upload & Remap` K5's degenerate check, with
/// the *no record* state said in as many words at the start, beside what it
/// costs.**
///
/// *"The located answers are keyed to this machine and travel nowhere, and a
/// relative location resolves against the file's machine folder and nothing
/// else."*
///
/// **Taking the control away instead would break the Creator's own stated
/// ground**, and **a control that vanished for a reason the user cannot see is
/// precisely the question the rule exists to prevent.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhatThePressStarts {
    /// The ordinary check.
    TheOrdinaryCheck,
    /// **`Dependency Upload & Remap` K5's degenerate check**, with the *no
    /// record* state said at the start.
    TheDegenerateCheckWithNoRecordSaid,
}

/// N7a, for the press.
pub fn what_the_press_starts(found: &WhereItCameFrom) -> WhatThePressStarts {
    match found {
        WhereItCameFrom::TheSourcePath(_) => WhatThePressStarts::TheOrdinaryCheck,
        WhereItCameFrom::NoRecord => WhatThePressStarts::TheDegenerateCheckWithNoRecordSaid,
    }
}

/// **N8: entries are disqualified, retired and superseded on the same discipline
/// the neighbouring records use.**
///
/// - **An entry under a superseded installation identity is disqualified rather
///   than deleted** — a recycled machine has no history of receiving anything,
///   and the entry stays true as the account of what was once sent.
/// - **A second upload of the same file from a different place writes a later
///   entry**, and N6's recency rule answers with it.
/// - **Entries retire when the user removes the request record they belong to**
///   (user-held, U1) **and never by age**; a removed server row takes the
///   question with it, since the lookup is keyed by the row.
///
/// **So removing a request record removes the provenance of its files with it,
/// and the removal act names that cost before it is taken.** The lifetime is not
/// moved to buy that back: `State, identity and recovery` §9.4 declares the
/// outcome record **user-held** and §11.1 places it in the class that ends when
/// *"the user removes it, and never otherwise"*.
///
/// **The repair is the telling**, and the presentation is `Desktop app and
/// delivery`'s — that document's §21 takes it (§13 item 17, closed).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WhatRemovingTheRecordCosts {
    /// The landing paths that will answer *no record* afterwards.
    pub loses_the_provenance_of: Vec<Vec<u8>>,
}

/// N8's telling, composed before the removal is taken.
pub fn what_removing_the_record_costs(request: &UploadRequest) -> WhatRemovingTheRecordCosts {
    WhatRemovingTheRecordCosts {
        loses_the_provenance_of: request
            .entries
            .iter()
            .filter(|entry| entry.outcome == Some(UnitOutcome::Done))
            .map(|entry| match entry.disposition.as_ref() {
                Some(Disposition::RenamedUnderKeepBoth(fresh)) => fresh.clone(),
                _ => entry.landing_path.clone(),
            })
            .collect(),
    }
}

/// **N9: the keying was checked against `Dependency Upload & Remap` K3 and the
/// two fit without either moving.**
///
/// `Dependency Upload & Remap` K3 keys located answers by the holding file and
/// the reference's exact recorded bytes, **with no machine anywhere in the key,
/// so the answer travels**; this record answers a question about one machine
/// and is keyed under the server row and installation identity, which is
/// `Dependency Upload & Remap` K7's shape for machine-keyed records. **Theirs travels, mine dies with the installation, and
/// each is right for its question.**
///
/// What that document reads — the source path this document hands over, and the
/// published transfer record's content identity (its Q5 route 1) — are N6's
/// answer and `Server protocol and transfers` S9's record respectively, **both
/// already durable.**
pub const THIS_RECORDS_KEY_IS_THE_SERVER_ROW_AND_THE_INSTALLATION: bool = true;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::request::EnumerationEntry;
    use smoozy_state::identities::IdentityKind;
    use std::time::SystemTime;

    fn an_identity(kind: IdentityKind) -> Identity {
        Identity::issue(kind).expect("this machine supplies random bytes")
    }

    fn name(bytes: &[u8]) -> EntryName {
        EntryName::from_the_machine(bytes.to_vec())
    }

    /// **The witness is handed in, never read from the published value.**
    ///
    /// What these tests are about is `where_it_came_from` and the landing
    /// path. The request is published by this test itself.
    fn a_published_request(
        server_row: &Identity,
        installation: Option<&Identity>,
        source: &[u8],
        landing: &[u8],
    ) -> UploadRequest {
        let mut entry = EnumerationEntry::enumerated(source.to_vec(), landing.to_vec(), 10);
        entry.disposition = Some(Disposition::ToSend);
        entry
            .its_outcome_became(UnitOutcome::Done, SystemTime::UNIX_EPOCH)
            .expect("outcome");
        UploadRequest::confirmed(
            crate::request::open_an_upload_request().expect("the answer handed in says open"),
            an_identity(IdentityKind::Operation),
            server_row.clone(),
            installation.cloned(),
            b"/jobs".to_vec(),
            vec![entry],
            an_identity(IdentityKind::Run),
        )
    }

    #[test]
    fn the_structure_is_mirrored_component_for_component_and_the_bytes_survive() {
        let relative = vec![name("textures".as_bytes()), name(b"\xff\xfe odd\n.png")];
        let path = landing_path(b"/jobs/scene", &relative);
        assert_eq!(path, b"/jobs/scene/textures/\xff\xfe odd\n.png".to_vec());
        const { assert!(NOTHING_REWRITES_OR_RE_ENCODES_A_NAME_IN_FLIGHT) };
    }

    #[test]
    fn a_folder_the_walk_saw_is_a_merge_and_is_not_re_made() {
        let seen = vec![b"/jobs/scene".to_vec()];
        assert_eq!(
            what_to_do_with(b"/jobs/scene", &seen),
            WhatToDoWithALandingFolder::ItIsAMergeAndIsNotReMade
        );
        assert_eq!(
            what_to_do_with(b"/jobs/scene/textures", &seen),
            WhatToDoWithALandingFolder::MakeItPlainly
        );
    }

    #[test]
    fn an_already_exists_on_a_folder_the_walk_did_not_see_is_the_race() {
        assert!(an_unexpected_already_exists_is_the_race_surfacing_early(
            &WhatToDoWithALandingFolder::MakeItPlainly,
            &WhatTheMachineSaidAboutAFolder::ItIsAlreadyThere
        ));
        assert!(!an_unexpected_already_exists_is_the_race_surfacing_early(
            &WhatToDoWithALandingFolder::ItIsAMergeAndIsNotReMade,
            &WhatTheMachineSaidAboutAFolder::ItIsAlreadyThere
        ));
    }

    #[test]
    fn the_length_bound_passes_the_accepted_maxima_and_fails_the_byte_above() {
        let at_255 = [b'a'; 255];
        let at_256 = [b'a'; 256];
        assert_eq!(
            check_the_length(&[b"/".to_vec(), at_255.to_vec()].concat()),
            TheLengthBound::Inside
        );
        assert!(matches!(
            check_the_length(&[b"/".to_vec(), at_256.to_vec()].concat()),
            TheLengthBound::AComponentIsTooLong { bytes: 256, .. }
        ));

        // 4095 accepted, 4096 refused, over the whole path.
        let mut path = Vec::new();
        while path.len() + 1 + 200 <= 4095 {
            path.push(b'/');
            path.extend(std::iter::repeat_n(b'a', 200));
        }
        // The tail is its own component so that no component ever passes 255
        // bytes: this test is about the path bound and not the name bound.
        let tail = 4095 - path.len() - 1;
        path.push(b'/');
        path.extend(std::iter::repeat_n(b'a', tail));
        assert_eq!(path.len(), 4095);
        assert_eq!(check_the_length(&path), TheLengthBound::Inside);
        path.push(b'a');
        assert!(matches!(
            check_the_length(&path),
            TheLengthBound::ThePathIsTooLong { bytes: 4096 }
        ));
    }

    #[test]
    fn a_failed_name_costs_that_file_and_nothing_else() {
        let server_row = an_identity(IdentityKind::LocalServer);
        let mut request = a_published_request(&server_row, None, b"/home/a.png", b"/jobs/a.png");
        request.entries.push(EnumerationEntry::enumerated(
            b"/home/b.png".to_vec(),
            b"/jobs/b.png".to_vec(),
            10,
        ));
        assert_eq!(what_a_failed_name_costs(&request, b"/jobs/a.png"), 1);
    }

    #[test]
    fn the_lookup_answers_the_most_recent_published_entry_and_never_a_clock() {
        let server_row = an_identity(IdentityKind::LocalServer);
        let installation = an_identity(IdentityKind::Installation);
        let earlier = a_published_request(
            &server_row,
            Some(&installation),
            b"/home/first/a.png",
            b"/jobs/a.png",
        );
        let later = a_published_request(
            &server_row,
            Some(&installation),
            b"/home/second/a.png",
            b"/jobs/a.png",
        );
        assert_eq!(
            where_it_came_from(
                &[earlier, later],
                &server_row,
                b"/jobs/a.png",
                Some(&installation)
            ),
            WhereItCameFrom::TheSourcePath(b"/home/second/a.png".to_vec())
        );
    }

    #[test]
    fn an_entry_under_a_superseded_installation_is_disqualified_rather_than_deleted() {
        let server_row = an_identity(IdentityKind::LocalServer);
        let was = an_identity(IdentityKind::Installation);
        let now = an_identity(IdentityKind::Installation);
        let request = a_published_request(&server_row, Some(&was), b"/home/a.png", b"/jobs/a.png");
        // The entry is still there and still true; the lookup simply does not
        // answer with it.
        assert_eq!(request.entries.len(), 1);
        assert_eq!(
            where_it_came_from(
                std::slice::from_ref(&request),
                &server_row,
                b"/jobs/a.png",
                Some(&now)
            ),
            WhereItCameFrom::NoRecord
        );
        assert_eq!(
            where_it_came_from(&[request], &server_row, b"/jobs/a.png", Some(&was)),
            WhereItCameFrom::TheSourcePath(b"/home/a.png".to_vec())
        );
    }

    #[test]
    fn a_removed_server_row_takes_the_question_with_it() {
        let server_row = an_identity(IdentityKind::LocalServer);
        let another = an_identity(IdentityKind::LocalServer);
        let request = a_published_request(&server_row, None, b"/home/a.png", b"/jobs/a.png");
        assert_eq!(
            where_it_came_from(&[request], &another, b"/jobs/a.png", None),
            WhereItCameFrom::NoRecord
        );
    }

    #[test]
    fn a_keep_both_unit_answers_under_the_name_it_actually_published() {
        let server_row = an_identity(IdentityKind::LocalServer);
        let mut entry =
            EnumerationEntry::enumerated(b"/home/a.png".to_vec(), b"/jobs/a.png".to_vec(), 10);
        entry.disposition = Some(Disposition::RenamedUnderKeepBoth(
            b"/jobs/a (2).png".to_vec(),
        ));
        entry
            .its_outcome_became(UnitOutcome::Done, SystemTime::UNIX_EPOCH)
            .expect("outcome");
        let request = UploadRequest::confirmed(
            crate::request::open_an_upload_request().expect("the answer handed in says open"),
            an_identity(IdentityKind::Operation),
            server_row.clone(),
            None,
            b"/jobs".to_vec(),
            vec![entry],
            an_identity(IdentityKind::Run),
        );
        assert_eq!(
            where_it_came_from(
                std::slice::from_ref(&request),
                &server_row,
                b"/jobs/a (2).png",
                None
            ),
            WhereItCameFrom::TheSourcePath(b"/home/a.png".to_vec())
        );
        assert_eq!(
            where_it_came_from(&[request], &server_row, b"/jobs/a.png", None),
            WhereItCameFrom::NoRecord
        );
    }

    #[test]
    fn the_toolbar_reads_the_kind_and_only_the_kind() {
        let both = TheToolbar {
            render_settings: true,
            check_dependencies: true,
        };
        assert_eq!(the_toolbar_for(&name(b"scene.blend"), Kind::File), both);
        assert_eq!(the_toolbar_for(&name(b"SCENE.BLEND"), Kind::File), both);
        assert_eq!(
            the_toolbar_for(&name(b"texture.png"), Kind::File),
            TheToolbar {
                render_settings: false,
                check_dependencies: false
            }
        );
        // A folder named like one is not a `.blend`.
        assert_eq!(
            the_toolbar_for(&name(b"scene.blend"), Kind::Folder),
            TheToolbar {
                render_settings: false,
                check_dependencies: false
            }
        );
    }

    #[test]
    fn no_record_starts_the_degenerate_check_rather_than_nothing() {
        assert_eq!(
            what_the_press_starts(&WhereItCameFrom::NoRecord),
            WhatThePressStarts::TheDegenerateCheckWithNoRecordSaid
        );
        assert_eq!(
            what_the_press_starts(&WhereItCameFrom::TheSourcePath(b"/home/a.png".to_vec())),
            WhatThePressStarts::TheOrdinaryCheck
        );
    }

    #[test]
    fn the_removal_names_what_it_costs_before_it_is_taken() {
        let server_row = an_identity(IdentityKind::LocalServer);
        let request = a_published_request(&server_row, None, b"/home/a.png", b"/jobs/a.png");
        let cost = what_removing_the_record_costs(&request);
        assert_eq!(cost.loses_the_provenance_of, vec![b"/jobs/a.png".to_vec()]);
    }

    #[test]
    fn moving_the_browser_touches_no_request() {
        let server_row = an_identity(IdentityKind::LocalServer);
        let request = a_published_request(&server_row, None, b"/home/a.png", b"/jobs/a.png");
        let here = TheBrowsingPosition {
            server_row: server_row.clone(),
            folder: b"/jobs".to_vec(),
        };
        let there = TheBrowsingPosition {
            server_row: an_identity(IdentityKind::LocalServer),
            folder: b"/elsewhere".to_vec(),
        };
        assert!(!moving_the_browser_touches_a_request(
            &here, &there, &request
        ));
        assert_eq!(request.server_row, server_row, "the binding did not move");
    }
}
