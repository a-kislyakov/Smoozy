//! Where downloads land — `Download` §11, H1 to H11, Rows 204, 311, 328, 413
//! and 509.
//!
//! **Constraint:** Row 204 — where a download lands on three operating systems
//! — and Row 509's *show a job's downloaded files* control, whose three
//! remaining clauses H11 accounts for.
//!
//! **The folder is `Desktop app and delivery` §8's, consumed whole**: three
//! routes on three systems that are three mechanisms rather than three
//! spellings; the Windows default-path flag never passed; the Linux path read
//! and never assembled, with the home folder as the documented fallback and
//! nothing cached; **nothing written into the folder unasked**, which is
//! Apple's own instruction; a route that returns nothing proposing nothing;
//! and every name written into it judged like any other. **None of that is
//! taken a second time here.**
//!
//! **Markers used below that belong to another document, each with the
//! document it belongs to.** `Desktop app and delivery` C15, C19, K20, K21,
//! K22, K23, K24. `Queue & History` N17, N18, Y8. `Reporting a problem` V7.
//!
//! **A marker this plan also defines is deliberately not listed above,
//! whichever document is meant, and is written out with its document at every
//! site instead** — the header list cannot tell the two apart, and a bare one
//! would resolve to a real and wrong clause. `Desktop app and delivery`'s W
//! series is the one that arises here, and every use of it names that
//! document.

use crate::arrangement::AnArrangement;
use crate::request::{AManualRequest, Disposition};

/// **H2: what is this document's is when the user directs, and the answer is
/// that every write follows a request or an arrangement the user made, whose
/// destination was put in front of them at that moment.**
///
/// **That is the explicit direction `Desktop app and delivery` W4 requires:
/// nothing lands anywhere the user was not shown at the moment they asked for
/// the thing that lands.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TheDestinationForThisAct {
    /// **The standing setting** — the proposed folder, or the folder the user
    /// pointed at instead, set once and user-held. It is the default the
    /// request or the switch-on moment shows.
    TheStandingFolder(Vec<u8>),
    /// **A single download can be sent elsewhere without disturbing the
    /// standing setting.**
    APerRequestOverride(Vec<u8>),
    /// **H3: where the route returns nothing and no standing folder is set,
    /// the request or the switch-on asks the user to choose a folder before it
    /// proceeds.**
    ///
    /// **Composing a path instead is refused** — the documented Linux fallback
    /// is the home folder and not a downloads folder inside it, and a composed
    /// path is the one wrong answer `Desktop app and delivery` W3 names. The
    /// question is the standing folder-choice control the spec already grants,
    /// **met at a forced moment**.
    TheUserIsAskedToChooseAFolderBeforeItProceeds,
}

/// **H2 and H3, as one judgement.**
///
/// The proposal is `Desktop app and delivery` §8's route and is handed in;
/// **a route that returns nothing proposes nothing**
/// (`Desktop app and delivery` W5), and nothing here composes a path in its
/// place.
pub fn the_destination_for_this_act(
    the_per_request_override: Option<Vec<u8>>,
    the_standing_folder: Option<Vec<u8>>,
) -> TheDestinationForThisAct {
    if let Some(sent_elsewhere) = the_per_request_override {
        return TheDestinationForThisAct::APerRequestOverride(sent_elsewhere);
    }
    match the_standing_folder {
        Some(standing) => TheDestinationForThisAct::TheStandingFolder(standing),
        None => TheDestinationForThisAct::TheUserIsAskedToChooseAFolderBeforeItProceeds,
    }
}

/// **H1's last rule, held as a value: a route that returns nothing proposes
/// nothing, and no path is ever composed here.**
pub const NO_PATH_IS_EVER_COMPOSED_WHERE_THE_ROUTE_RETURNED_NOTHING: bool = true;

/// **H4: the two unresolved halves — whether the macOS route looks the
/// location up or composes it, and whether Windows localises the on-disk name
/// — are `Desktop app and delivery` W6's, bear on no clause here, and are not
/// re-carried.**
pub const THE_TWO_UNRESOLVED_HALVES_ARE_THE_ADAPTERS_AND_BEAR_ON_NO_CLAUSE_HERE: bool = true;

/// The path components of a path, empty ones dropped.
///
/// **The crate's one splitting of a path into components on the rules side**,
/// so the containment test here and [`crate::request`]'s top-level-thing
/// reading split a path the same way.
pub(crate) fn components(path: &[u8]) -> Vec<&[u8]> {
    path.split(|byte| *byte == b'/')
        .filter(|component| !component.is_empty())
        .collect()
}

/// **Whether one path lies inside a folder, component by component.**
///
/// **It is never a byte-prefix test**, because a byte prefix says a folder
/// `/Users/a/Downloads/.smoozy-staging-old` is inside
/// `/Users/a/Downloads/.smoozy-staging` — two folders of the user's that share
/// a spelling and nothing else. **This is the crate's one containment test on
/// the rules side** and [`crate::request`]'s two bounds ask it rather than
/// each composing a prefix of their own; the acting side's twin is
/// [`crate::acting::record_directory`]'s own relative-path reading, which
/// answers the same question and hands back the remainder.
pub(crate) fn is_inside(path: &[u8], folder: &[u8]) -> bool {
    let folder = components(folder);
    let path = components(path);
    path.len() > folder.len() && path[..folder.len()] == folder[..]
}

fn depth_below(path: &[u8], folder: &[u8]) -> Option<usize> {
    is_inside(path, folder).then(|| components(path).len() - components(folder).len())
}

/// **H5a: a manual request brought a job home *as a whole job* where the thing
/// it asked for was the job's own output folder, and that is read off two
/// fields the request record already holds.**
///
/// The test runs per enumerated entry, over the entries whose recorded source
/// path lies under the job's output folder and **whose unit outcome is a
/// publication** — an entry that never landed puts no folder anywhere.
///
/// - **The landing path relative to the request's destination is one component
///   longer than the source path relative to the job's output folder** → the
///   thing asked for was the job's own output folder, and that request's
///   destination is one of the job's landing places.
/// - **Any other count** → the thing asked for was a file, or a folder inside
///   the job's output folder, and that destination is not a landing place of
///   the job.
///
/// **The count, and never a name, because every name on the way can be
/// rewritten and the depth cannot.** P11 composes a landing path as the
/// destination plus the picked thing's own name plus what lay below it, **so
/// the picked root's depth below the job's output folder is the only thing
/// that separates the two cases**: a picked root at depth k lands a file
/// sitting d below the job's output folder at 1 + (d − k) components under the
/// destination against d in the source, **and the two are equal only at
/// k = 0**, which is the job's output folder itself.
///
/// **Matching the top component by name instead fails on all three things that
/// rewrite it** — P10 composes the job folder's landing name against the
/// *receiving* system rather than carrying the machine's, M8's Keep both puts
/// a counted suffix on the whole arriving top-level thing, and the receiving
/// filesystem may fold two spellings into one
/// (`Desktop app and delivery` C15/C19). **The component count survives all
/// three.**
pub fn brought_the_job_home_as_a_whole_job(
    request: &AManualRequest,
    the_jobs_output_folder: &[u8],
) -> bool {
    the_whole_jobs_landing_place(request, the_jobs_output_folder).is_some()
}

/// **H5's landing place for a request H5a counts: the job's own folder as it
/// landed** — the destination plus the first component of a counted entry's
/// landing path below it. Added 2026-10-03 with H5's paragraph of that date
/// (the Creator's note 15 on the test copy).
///
/// **M8's *Keep both* suffix is carried**, because applying that answer
/// rewrites the entry's landing path itself
/// ([`crate::request::apply_the_collision_answer`]). Nothing here composes a
/// name: the folder is read off what was recorded.
pub fn the_whole_jobs_landing_place(
    request: &AManualRequest,
    the_jobs_output_folder: &[u8],
) -> Option<Vec<u8>> {
    request.entries.iter().find_map(|entry| {
        if !it_published(entry) {
            return None;
        }
        let in_the_source = depth_below(&entry.source_path, the_jobs_output_folder)?;
        let under_the_destination = depth_below(&entry.landing_path, &request.destination)?;
        if under_the_destination != in_the_source + 1 {
            return None;
        }
        let top = crate::request::the_top_level_thing_of(&entry.landing_path, &request.destination)?;
        Some(crate::fetching::the_landing_path(&request.destination, &top))
    })
}

/// **What one manual request put directly under its destination**: the
/// distinct top-level things its published entries landed as, each as a whole
/// path, in the order first met. A whole job is one thing, the job's own
/// folder; a picked file or folder is itself.
///
/// **It is the reading *Files on your computer* on a finished download
/// selects from** (`Design/Transfers`, the paragraph of 2026-10-03): one thing
/// is shown selected, and several open the destination with none selected.
pub fn what_the_request_landed(request: &AManualRequest) -> Vec<Vec<u8>> {
    let mut out: Vec<Vec<u8>> = Vec::new();
    for entry in request.entries.iter().filter(|entry| it_published(entry)) {
        let Some(top) =
            crate::request::the_top_level_thing_of(&entry.landing_path, &request.destination)
        else {
            continue;
        };
        let place = crate::fetching::the_landing_path(&request.destination, &top);
        if !out.contains(&place) {
            out.push(place);
        }
    }
    out
}

/// Whether an entry's unit outcome is a publication — P5's, which is what puts
/// a folder anywhere at all.
fn it_published(entry: &crate::request::EnumerationEntry) -> bool {
    entry
        .outcome
        .as_ref()
        .is_some_and(|outcome| matches!(outcome, smoozy_state::outcomes::UnitOutcome::Done))
        && entry
            .disposition
            .as_ref()
            .is_some_and(Disposition::a_transfer_is_arranged)
}

/// **H5: a job's set of landing places is a derived reading over records
/// already kept, and no new field and no stored list is created.**
///
/// The set is the distinct folders drawn from two sources, **in the order each
/// was first written to**:
///
/// 1. **the arrangement's per-file reckoning** — every entry standing at
///    *fetched*, each already carrying **the destination in force when its own
///    act was taken**. That field was written for the reckoning's own matching
///    and it is exactly the field this reading needs;
/// 2. **the manual request outcome records that brought that job home as a
///    whole job**, which is H5a's test, applied to entries whose recorded
///    source path lies under **that job's own output folder**, whose path is
///    the manifest's recorded one (`Queue & History` Y8). **The tie is those
///    two recorded fields and nothing else, so no request record gains a job
///    field** — a manual download is not a job's act and is not made one here.
///
/// **H6: two acts extend the set and the spec names only one of them.** A
/// **destination reset** — the arrangement switched off and on, which asks
/// once more — is the spec's own case, and it is why the reckoning's per-entry
/// destination exists. **A hand-started whole-job download sent to a folder of
/// its own is the other**, and it splits a job's outputs with no arrangement
/// and no reset involved. **The promise's plain words cover both**, and the
/// arrangement's own two-folder progress reading reaches only the first, which
/// is why that reading is not this answer.
///
/// **A place is the job's own folder as it landed, not the destination that
/// holds it** — H5's paragraph of 2026-10-03, the Creator's note 15 on the
/// test copy. For an arrangement's entry it is the destination in force plus
/// P10's landing name, composed by
/// [`crate::fetching::the_job_folders_landing_name`] with the same `rules` and
/// `label` the fetch composed it with; for a manual request it is
/// [`the_whole_jobs_landing_place`]. The order is unchanged.
pub fn the_set_of_landing_places<R: crate::fetching::TheReceivingSystemsRules + ?Sized>(
    arrangement: Option<&AnArrangement>,
    rules: &R,
    the_jobs_label: Option<&str>,
    requests: &[AManualRequest],
    the_jobs_output_folder: &[u8],
) -> Vec<Vec<u8>> {
    let mut out: Vec<Vec<u8>> = Vec::new();
    let add = |folder: Vec<u8>, out: &mut Vec<Vec<u8>>| {
        if !out.contains(&folder) {
            out.push(folder);
        }
    };
    if let Some(arrangement) = arrangement {
        let the_landing_name =
            crate::fetching::the_job_folders_landing_name(rules, &arrangement.job, the_jobs_label);
        for destination in arrangement.the_folders_it_has_fetched_into() {
            add(
                crate::fetching::the_landing_path(&destination, &the_landing_name),
                &mut out,
            );
        }
    }
    for request in requests {
        if let Some(place) = the_whole_jobs_landing_place(request, the_jobs_output_folder) {
            add(place, &mut out);
        }
    }
    out
}

/// **H8: a place that no longer exists stays in the set and says so.**
///
/// The promise is *shows both places*, and this document already carries a
/// destination folder that can be deleted or sit on a drive that is not
/// plugged in. **The reading is taken when the control is used and never
/// cached**, it distinguishes nothing that §9.1 does not already distinguish,
/// and **an unreachable place is presented as a place Smoozy cannot open
/// rather than dropped from the set** — dropping it would make a job that
/// landed in two folders read as a job that landed in one.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct APlace {
    pub folder: Vec<u8>,
    /// Read when the control is used. **Never cached.**
    pub smoozy_can_open_it: bool,
}

/// **H8, over a set and a reading taken at the moment the control is used.**
pub fn the_places(set: &[Vec<u8>], can_open: impl Fn(&[u8]) -> bool) -> Vec<APlace> {
    set.iter()
        .map(|folder| APlace {
            folder: folder.clone(),
            smoozy_can_open_it: can_open(folder),
        })
        .collect()
}

/// **H7 and H7a: what the *show a job's downloaded files* control is handed.**
///
/// **It is handed the set, never a folder.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WhatTheControlIsHanded {
    /// **One place, and it opens. Where the set holds more than one, the
    /// control opens every place in it and picks none** — it does not silently
    /// open the most recent, which would be the app quietly choosing which
    /// half of the user's outputs is the real one. **What is left to Design is
    /// only how several openings are presented and announced**, and the bound
    /// is that no place is hidden.
    ThePlaces(Vec<APlace>),
    /// **The set is empty and no manual entry under the job's output folder
    /// published** → **nothing of the job came home, and the control says so**
    /// rather than opening anything. An empty set already carries that the
    /// arrangement fetched nothing, since H5's first source puts a place in
    /// the set for every *fetched* entry.
    NothingOfTheJobCameHome,
    /// **The set is empty and manual entries under the job's output folder did
    /// publish** → **what came home was picked out file by file, and the
    /// control says that instead.** Those destinations are **not** offered:
    /// each file went where the user sent it, and a folder holding three
    /// frames of a job is not the job's folder — offering it would be Smoozy
    /// naming a place the spec says is not one.
    WhatCameHomeWasPickedOutFileByFile,
}

/// **H7 and H7a, as one composition. The words are Design's, and the bound is
/// the same one from the other side: no place hidden where one exists, and
/// none named where none does.**
pub fn what_the_control_is_handed(
    set: &[Vec<u8>],
    requests: &[AManualRequest],
    the_jobs_output_folder: &[u8],
    can_open: impl Fn(&[u8]) -> bool,
) -> WhatTheControlIsHanded {
    if !set.is_empty() {
        // **That holds where files were also picked out one at a time beside
        // the whole job**, because the spec's exception is keyed on *only*.
        return WhatTheControlIsHanded::ThePlaces(the_places(set, can_open));
    }
    let anything_published_under_the_folder = requests.iter().any(|request| {
        request.entries.iter().any(|entry| {
            it_published(entry) && is_inside(&entry.source_path, the_jobs_output_folder)
        })
    });
    if anything_published_under_the_folder {
        WhatTheControlIsHanded::WhatCameHomeWasPickedOutFileByFile
    } else {
        WhatTheControlIsHanded::NothingOfTheJobCameHome
    }
}

/// **H7: the opening itself is a named service this document does not own, and
/// it is written.**
///
/// The act is the reveal service `Reporting a problem` V7 declares, against
/// Fact 132 — answered on all three systems, documentation alone off the Mac —
/// whose finding is that the chosen shell exposes none of it: its one
/// browser-opening call refuses the file scheme by name and returns nothing
/// either way, **so the act is platform code of `Desktop app and delivery`'s
/// own**. That code is written, at that document's `Desktop app and delivery`
/// K20 to `Desktop app and delivery` K24, and it is supplied to this document
/// by that document's §21.
///
/// **Each opening reports its own outcome to the caller** —
/// `Desktop app and delivery` K22's rule — which is what H8's *cannot open*
/// reading is; **and nothing in H5 to H8 waits on the service, because the set
/// and its order are composed from records already kept and are correct
/// whether or not anything opens.**
pub const THE_OPENING_IS_THE_ADAPTERS_SERVICE_AND_NOTHING_HERE_WAITS_ON_IT: bool = true;

/// **H9: nothing here is supplied to `Queue & History` as a second reading.**
///
/// That document is handed the toggle's effects, the narrowing control's
/// effect, the arrangement's progress reading — **and H5's set as a third
/// supply**, because the control that consumes it lives on that document's
/// entry while the rule behind it is this document's, which is what that spec
/// assigns.
pub const THE_SET_IS_A_THIRD_SUPPLY_TO_THE_ENTRY_AND_NOT_A_SECOND_READING: bool = true;

/// **H10: the spec disagreement this row found is not repaired here.**
///
/// `Preparation/Specs/Queue & History.md` states of a job's whole-job
/// downloads that *"Both land in that job's own folder, so both are the same
/// thing to open"*, and assigns the rule behind it to Download — and the
/// change of 2026-08-20 to `Preparation/Specs/Download.md` withdrew the
/// argument under it. **That is a spec correction and only the Creator rules
/// on one; the runner carries it.**
///
/// **Nothing here waits on the ruling**: the set is composed the same way
/// whichever wording the spec settles on, because the promise that a job can
/// have more than one landing place is already in the spec that moved.
///
/// **And the same paragraph's other exception is not a disagreement and is not
/// carried to anybody**: *"where the only thing that came home was picked out
/// file by file in the file browser, Smoozy says that instead"* is a scope
/// this document had over-reached rather than a sentence the spec has to move,
/// and H5a and H7a keep it from records already kept.
pub const THE_SPEC_DISAGREEMENT_IS_THE_RUNNERS_AND_NOTHING_HERE_WAITS_ON_IT: bool = true;

/// **H11: the three clauses of the control's spec sentence that H5 to H10 do
/// not reach, and where each is answered instead.**
///
/// **Nothing below is decided here, nothing below moves, and no clause above
/// is reopened.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhichClauseOfTheControlsSentence {
    /// *"It needs no connection and no machine, since the files are already
    /// there, so it still works after the machine is gone"* — **answered at
    /// `Queue & History` N18's third bullet**, which states it and names its
    /// ground: the control is not a reach-back, that document's N17 never
    /// withholds it, and **both records the set is composed from are
    /// user-held** in the local record, neither a copy of a machine state the
    /// machine still holds.
    ///
    /// **This section's half is that H5's two sources are those two records
    /// and no third**, so no clause of H5 to H8 opens a connection, reads a
    /// machine or resolves a server row.
    ItNeedsNoConnectionAndNoMachine,
    /// *"It is offered whatever the job's ending was, including the part of a
    /// job that failed, was stopped or was given up before it finished"* —
    /// **answered at `Queue & History` N18's third bullet**, over that
    /// document's own entry states, and cited rather than re-taken.
    ///
    /// **This section's half is that no clause of H5 to H8 reads a job's
    /// ending at all.** The one place an ending is read in this document is
    /// the *cannot be vouched for* mark, which marks files a by-hand request
    /// brings and **withholds no place from the set and no folder from the
    /// control.**
    ItIsOfferedWhateverTheJobsEndingWas,
    /// *"Where the only thing that came home was picked out file by file in
    /// the file browser, Smoozy says that instead"* — **answered here, and it
    /// is this section's own**: H5a's component-count test keeps such a
    /// request out of the set, and H7a's third case has the control say what
    /// came home was picked out file by file, with those destinations not
    /// offered.
    TheFileByFileEnding,
}

impl WhichClauseOfTheControlsSentence {
    /// Whether this document answers the clause itself, or cites the answer.
    pub fn is_answered_here(self) -> bool {
        matches!(self, WhichClauseOfTheControlsSentence::TheFileByFileEnding)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::arrangement::tests::an_arrangement;
    use crate::fetching::tests::ASystem;
    use crate::arrangement::{AReckoningEntry, TheNarrowing, WhatSmoozyDid};
    use crate::request::EnumerationEntry;
    use crate::vouching::AVouchingMark;
    use smoozy_state::identities::{Identity, IdentityKind};
    use smoozy_state::outcomes::UnitOutcome;

    fn an_identity(kind: IdentityKind) -> Identity {
        Identity::issue(kind).expect("the desk has randomness")
    }

    fn a_published_entry(source: &[u8], landing: &[u8]) -> EnumerationEntry {
        let mut entry = EnumerationEntry::enumerated(
            source.to_vec(),
            landing.to_vec(),
            10,
            AVouchingMark::NoClaimIsMade,
        );
        entry.disposition = Some(Disposition::ToFetch);
        entry
            .its_outcome_became(UnitOutcome::Done)
            .expect("first write");
        entry
    }

    fn a_request(destination: &[u8], entries: Vec<EnumerationEntry>) -> AManualRequest {
        AManualRequest::confirmed(
            an_identity(IdentityKind::Operation),
            an_identity(IdentityKind::LocalServer),
            None,
            destination.to_vec(),
            entries,
            an_identity(IdentityKind::Run),
        )
    }

    const JOB_FOLDER: &[u8] = b"/smoozy/jobs/abc/outputs";

    #[test]
    fn the_standing_folder_is_the_default_and_a_single_download_can_be_sent_elsewhere() {
        assert_eq!(
            the_destination_for_this_act(None, Some(b"/Users/a/Downloads".to_vec())),
            TheDestinationForThisAct::TheStandingFolder(b"/Users/a/Downloads".to_vec())
        );
        assert_eq!(
            the_destination_for_this_act(
                Some(b"/Volumes/work".to_vec()),
                Some(b"/Users/a/Downloads".to_vec())
            ),
            TheDestinationForThisAct::APerRequestOverride(b"/Volumes/work".to_vec())
        );
    }

    #[test]
    fn where_nothing_is_proposed_and_nothing_is_set_the_user_is_asked_and_no_path_is_composed() {
        assert_eq!(
            the_destination_for_this_act(None, None),
            TheDestinationForThisAct::TheUserIsAskedToChooseAFolderBeforeItProceeds
        );
    }

    #[test]
    fn a_whole_job_download_puts_its_destination_in_the_set_and_a_picked_file_does_not() {
        // The job's own output folder was asked for: the landing path is one
        // component longer than the source path below the job folder.
        let whole_job = a_request(
            b"/Users/a/Downloads",
            vec![a_published_entry(
                b"/smoozy/jobs/abc/outputs/beauty/f_0001.exr",
                b"/Users/a/Downloads/abc night/beauty/f_0001.exr",
            )],
        );
        assert!(brought_the_job_home_as_a_whole_job(&whole_job, JOB_FOLDER));

        // One file picked out: the landing path is at the same depth.
        let picked_out = a_request(
            b"/Users/a/Downloads",
            vec![a_published_entry(
                b"/smoozy/jobs/abc/outputs/beauty/f_0001.exr",
                b"/Users/a/Downloads/f_0001.exr",
            )],
        );
        assert!(!brought_the_job_home_as_a_whole_job(
            &picked_out,
            JOB_FOLDER
        ));

        // A folder inside the job's output folder: one component longer than
        // its own root, but two shorter than the job folder's depth.
        let inner_folder = a_request(
            b"/Users/a/Downloads",
            vec![a_published_entry(
                b"/smoozy/jobs/abc/outputs/beauty/f_0001.exr",
                b"/Users/a/Downloads/beauty/f_0001.exr",
            )],
        );
        assert!(!brought_the_job_home_as_a_whole_job(
            &inner_folder,
            JOB_FOLDER
        ));
    }

    #[test]
    fn the_test_is_the_component_count_and_survives_every_name_that_is_rewritten() {
        // Keep both's counted suffix, and a landing name composed against the
        // receiving system: both change the top component's name and neither
        // changes its depth.
        for renamed in [
            &b"/Users/a/Downloads/abc night (2)/beauty/f_0001.exr"[..],
            &b"/Users/a/Downloads/abc/beauty/f_0001.exr"[..],
            &b"/Users/a/Downloads/ABC NIGHT/beauty/f_0001.exr"[..],
        ] {
            let request = a_request(
                b"/Users/a/Downloads",
                vec![a_published_entry(
                    b"/smoozy/jobs/abc/outputs/beauty/f_0001.exr",
                    renamed,
                )],
            );
            assert!(brought_the_job_home_as_a_whole_job(&request, JOB_FOLDER));
        }
    }

    #[test]
    fn an_entry_that_never_landed_puts_no_folder_anywhere() {
        let mut never_landed = EnumerationEntry::enumerated(
            b"/smoozy/jobs/abc/outputs/beauty/f_0001.exr".to_vec(),
            b"/Users/a/Downloads/abc night/beauty/f_0001.exr".to_vec(),
            10,
            AVouchingMark::NoClaimIsMade,
        );
        never_landed.disposition = Some(Disposition::ToFetch);
        let request = a_request(b"/Users/a/Downloads", vec![never_landed]);
        assert!(!brought_the_job_home_as_a_whole_job(&request, JOB_FOLDER));
    }

    #[test]
    fn the_set_takes_the_arrangements_folders_first_in_the_order_each_was_written_to() {
        let mut arrangement = an_arrangement(b"/Users/a/Downloads");
        arrangement.reckoning.push(AReckoningEntry::acted_on(
            b"f_0001.png".to_vec(),
            Some(1),
            WhatSmoozyDid::Fetched,
            b"/Users/a/Downloads".to_vec(),
        ));
        arrangement.switched_off();
        arrangement.switched_on_again(b"/Volumes/work".to_vec(), TheNarrowing::Everything);
        arrangement.reckoning.push(AReckoningEntry::acted_on(
            b"f_0002.png".to_vec(),
            Some(2),
            WhatSmoozyDid::Fetched,
            b"/Volumes/work".to_vec(),
        ));
        let hand_started = a_request(
            b"/Users/a/Archive",
            vec![a_published_entry(
                b"/smoozy/jobs/abc/outputs/f_0003.png",
                b"/Users/a/Archive/abc night/f_0003.png",
            )],
        );
        // **Each place is the job's own folder in its destination** (H5's
        // paragraph of 2026-10-03): P10's landing name under each folder the
        // arrangement fetched into, and the hand-started request's own.
        let name = String::from_utf8(crate::fetching::the_job_folders_landing_name(
            &ASystem::plain(),
            &arrangement.job,
            Some("night"),
        ))
        .expect("text");
        assert_eq!(
            the_set_of_landing_places(
                Some(&arrangement),
                &ASystem::plain(),
                Some("night"),
                &[hand_started],
                JOB_FOLDER
            ),
            vec![
                format!("/Users/a/Downloads/{name}").into_bytes(),
                format!("/Volumes/work/{name}").into_bytes(),
                b"/Users/a/Archive/abc night".to_vec()
            ]
        );
    }

    #[test]
    fn a_hand_started_whole_job_download_extends_the_set_with_no_reset_involved() {
        // H6's second act: no arrangement at all, and the set still has a
        // place in it.
        let hand_started = a_request(
            b"/Users/a/Archive",
            vec![a_published_entry(
                b"/smoozy/jobs/abc/outputs/f_0003.png",
                b"/Users/a/Archive/abc night/f_0003.png",
            )],
        );
        assert_eq!(
            the_set_of_landing_places(None, &ASystem::plain(), None, &[hand_started], JOB_FOLDER),
            vec![b"/Users/a/Archive/abc night".to_vec()]
        );
    }

    /// **H5's landing place, of 2026-10-03: the job's own folder as it
    /// landed, never the destination that holds it** — the Creator's note 15
    /// on the test copy, *"so I have to guess which one it is"*.
    #[test]
    fn a_whole_job_download_into_a_folder_hands_the_jobs_own_folder_in_it() {
        let whole_job = a_request(
            b"/Users/a/x",
            vec![a_published_entry(
                b"/smoozy/jobs/abc/outputs/beauty/f_0001.exr",
                b"/Users/a/x/abc night/beauty/f_0001.exr",
            )],
        );
        assert_eq!(
            the_set_of_landing_places(None, &ASystem::plain(), None, &[whole_job], JOB_FOLDER),
            vec![b"/Users/a/x/abc night".to_vec()]
        );
    }

    /// **M8's *Keep both* suffix is carried into the place**, because the
    /// answer rewrites the entry's landing path and the place is read off it.
    #[test]
    fn a_whole_job_kept_beside_an_earlier_copy_hands_the_renamed_folder() {
        let mut entry = a_published_entry(
            b"/smoozy/jobs/abc/outputs/beauty/f_0001.exr",
            b"/Users/a/x/abc night (2)/beauty/f_0001.exr",
        );
        entry.disposition = Some(Disposition::RenamedUnderKeepBoth(entry.landing_path.clone()));
        let kept_both = a_request(b"/Users/a/x", vec![entry]);
        assert_eq!(
            the_whole_jobs_landing_place(&kept_both, JOB_FOLDER),
            Some(b"/Users/a/x/abc night (2)".to_vec())
        );
    }

    /// **What a request landed, which *Files on your computer* selects from**:
    /// a picked single file hands its own path, a whole job its folder, and
    /// several picked things each of them.
    #[test]
    fn a_picked_single_file_hands_its_own_path() {
        let one_file = a_request(
            b"/Users/a/x",
            vec![a_published_entry(
                b"/smoozy/jobs/abc/outputs/beauty/f_0001.exr",
                b"/Users/a/x/f_0001.exr",
            )],
        );
        assert_eq!(what_the_request_landed(&one_file), vec![b"/Users/a/x/f_0001.exr".to_vec()]);

        let whole_job = a_request(
            b"/Users/a/x/",
            vec![
                a_published_entry(
                    b"/smoozy/jobs/abc/outputs/beauty/f_0001.exr",
                    b"/Users/a/x/abc night/beauty/f_0001.exr",
                ),
                a_published_entry(
                    b"/smoozy/jobs/abc/outputs/beauty/f_0002.exr",
                    b"/Users/a/x/abc night/beauty/f_0002.exr",
                ),
            ],
        );
        assert_eq!(what_the_request_landed(&whole_job), vec![b"/Users/a/x/abc night".to_vec()]);

        let two_picked = a_request(
            b"/Users/a/x",
            vec![
                a_published_entry(b"/smoozy/a.png", b"/Users/a/x/a.png"),
                a_published_entry(b"/smoozy/b.png", b"/Users/a/x/b.png"),
            ],
        );
        assert_eq!(what_the_request_landed(&two_picked).len(), 2);
    }

    #[test]
    fn a_place_that_no_longer_exists_stays_in_the_set_and_says_it_cannot_be_opened() {
        let set = vec![b"/Users/a/Downloads".to_vec(), b"/Volumes/work".to_vec()];
        let places = the_places(&set, |folder| folder != b"/Volumes/work");
        assert_eq!(places.len(), 2);
        assert!(places[0].smoozy_can_open_it);
        assert!(!places[1].smoozy_can_open_it);
    }

    #[test]
    fn the_control_is_handed_every_place_and_picks_none() {
        let set = vec![b"/Users/a/Downloads".to_vec(), b"/Volumes/work".to_vec()];
        match what_the_control_is_handed(&set, &[], JOB_FOLDER, |_| true) {
            WhatTheControlIsHanded::ThePlaces(places) => assert_eq!(places.len(), 2),
            other => panic!("{other:?}"),
        }
    }

    #[test]
    fn an_empty_set_reads_one_of_two_ways_and_the_file_by_file_destinations_are_not_offered() {
        let picked_out = a_request(
            b"/Users/a/Downloads",
            vec![a_published_entry(
                b"/smoozy/jobs/abc/outputs/beauty/f_0001.exr",
                b"/Users/a/Downloads/f_0001.exr",
            )],
        );
        assert_eq!(
            what_the_control_is_handed(&[], std::slice::from_ref(&picked_out), JOB_FOLDER, |_| {
                true
            }),
            WhatTheControlIsHanded::WhatCameHomeWasPickedOutFileByFile
        );
        assert_eq!(
            what_the_control_is_handed(&[], &[], JOB_FOLDER, |_| true),
            WhatTheControlIsHanded::NothingOfTheJobCameHome
        );
    }

    #[test]
    fn files_picked_out_beside_a_whole_job_download_do_not_take_the_set_away() {
        // The spec's exception is keyed on *only*, so a non-empty set is
        // still H7's answer.
        let whole_job = a_request(
            b"/Users/a/Downloads",
            vec![a_published_entry(
                b"/smoozy/jobs/abc/outputs/f_0001.png",
                b"/Users/a/Downloads/abc night/f_0001.png",
            )],
        );
        let picked_out = a_request(
            b"/Users/a/Archive",
            vec![a_published_entry(
                b"/smoozy/jobs/abc/outputs/f_0002.png",
                b"/Users/a/Archive/f_0002.png",
            )],
        );
        let requests = vec![whole_job, picked_out];
        let set = the_set_of_landing_places(None, &ASystem::plain(), None, &requests, JOB_FOLDER);
        assert_eq!(set, vec![b"/Users/a/Downloads/abc night".to_vec()]);
        match what_the_control_is_handed(&set, &requests, JOB_FOLDER, |_| true) {
            WhatTheControlIsHanded::ThePlaces(places) => {
                assert_eq!(places.len(), 1);
                assert_eq!(places[0].folder, b"/Users/a/Downloads/abc night");
            }
            other => panic!("{other:?}"),
        }
    }

    #[test]
    fn one_of_the_three_clauses_is_answered_here_and_two_are_cited() {
        assert!(WhichClauseOfTheControlsSentence::TheFileByFileEnding.is_answered_here());
        assert!(
            !WhichClauseOfTheControlsSentence::ItNeedsNoConnectionAndNoMachine.is_answered_here()
        );
        assert!(
            !WhichClauseOfTheControlsSentence::ItIsOfferedWhateverTheJobsEndingWas
                .is_answered_here()
        );
    }
}
