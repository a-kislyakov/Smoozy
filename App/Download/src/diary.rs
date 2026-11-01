//! This module's diary declaration — `Download` §12's supply to
//! `Reporting a problem`.
//!
//! The rules are `Reporting a problem` J2, `Reporting a problem` J2a,
//! `Reporting a problem` J3, `Reporting a problem` J4 and
//! `Reporting a problem` J4a, taken as they stand. **The list is fixed**, and an act absent from it is one this
//! document does not write rather than one nobody got to. Lines are written
//! through the diary service; **a module declares or writes nothing itself.**
//!
//! **Two acts, and no told lines.** `Reporting a problem` J2 gives the spec's
//! fifth act — *published a download* — to this document, and
//! `Reporting a problem` J2a fixes the grain: **a publication is one file's, so a line per publication would
//! be a line per frame**, which the third test names outright. **The act the
//! user asked for is the request or the arrangement, and there are two of
//! them.**
//!
//! **Nothing below is decided at run time, and no line's text is composed at
//! run time.**
//!
//! **Markers used below that belong to another document, each with the
//! document it belongs to.** `Reporting a problem` J2a, J4a, U7.
//! `Desktop app and delivery` A7.
//!
//! **A marker this plan also defines is deliberately not listed above,
//! whichever document is meant, and is written out with its document at every
//! site instead** — the header list cannot tell the two apart, and a bare one
//! would resolve to a real and wrong clause. `Reporting a problem` J2,
//! `Reporting a problem` J3, `Reporting a problem` J4,
//! `Reporting a problem` J6, `Reporting a problem` N3,
//! `Reporting a problem` R3 and `State, identity and recovery` D5 are the ones
//! that arise here.

use smoozy_reporting::lines::LineKind;
use smoozy_state::identities::Identity;

use crate::ending::TheArrangementsReading;
use crate::request::HowARequestEnded;

/// **The one kind this module declares**, and no others.
///
/// **`Reporting a problem` J4a: this document writes no told lines, and the
/// absence is declared so that it is a decision rather than a silence.** The
/// one machine ending it acts on is the job's own ending, which is
/// `Queue & History`'s and is carried here rather than learned here — a told
/// line for it here would be a second copy of that document's. Everything else
/// this document reads from a machine runs at the machine's rate and not the
/// user's: the walk's readings, the per-file four-value reads and the
/// newer-version comparison, and the match of an arriving file against the
/// inventory. **J4a's bound forbids a line per file, per frame or per walk,
/// and its third bullet's reason is exactly the cost here** — a job of several
/// hundred frames would spend the diary's whole cap — `Reporting a problem`
/// R3 — on one
/// arrangement.
pub const DECLARED: [LineKind; 1] = [LineKind::Act];

/// **The two acts.**
///
/// Each fixed line form carries the act's name, **the moment on the desktop
/// clock, stated as that clock** (`State, identity and recovery` D5), and the
/// declared values — and nothing free-composed at run time
/// (`Reporting a problem` J3).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AnAct {
    /// **A manual download request ended.**
    ///
    /// **It is Smoozy's own**, arranged and ended by this document rather than
    /// read back from a machine — J2a's first test. **The spec names its
    /// completion as what the user gets** — the files in the folder they were
    /// shown at the moment they asked, and the request's own reading, counted
    /// in files and drawn as a bar — rather than as a step inside something
    /// else — the second test. **And it happens once per request the user
    /// confirmed, whatever the enumeration held** — the third.
    AManualDownloadRequestEnded {
        /// **The destination the request landed in** — the standing folder or
        /// the per-request override — a path held under the acted-on rule
        /// `Reporting a problem` N3 states, as the folder the user picked.
        destination: Vec<u8>,
        /// The request's ending as it is derived from the unit outcomes.
        ending: HowARequestEnded,
        /// **Units fetched and the whole in files.**
        units_fetched: usize,
        the_whole_in_files: usize,
        /// **The counts of *left alone*, *cannot land*, and what the
        /// enumeration recorded as refused.**
        left_alone: usize,
        cannot_land: usize,
        refused_by_the_enumeration: usize,
    },
    /// **An auto-download arrangement done.**
    ///
    /// **Smoozy's own again**: the arrangement decides for itself that nothing
    /// more is coming, by finishing its own bookkeeping and never by a
    /// judgement about the render. **The spec names the completion in terms** —
    /// *"the user is told when those outputs are home"* — and where it says so
    /// is the job's entry, a row the user reads afterwards, **which is the
    /// second test met on the spec's own sentence rather than on a screen this
    /// document invents.** **And the done transition happens at most once per
    /// arrangement per attempt**: a new render-start re-arms the arrangement
    /// and it runs to a second done, which is a second act the user asked for
    /// and not a second line about the first.
    AnAutoDownloadArrangementDone {
        /// **The arrangement's own reading at done** — *ended*, never
        /// *complete*, with arrivals against the whole.
        reading: TheArrangementsReading,
        /// **The destination folder, and both folders where a switch-on reset
        /// has split the job.**
        folders: Vec<Vec<u8>>,
        /// **The counts of the units that left the whole and of the
        /// shortfall** — *left alone*, *cannot land*, *skipped by narrowing*
        /// and *never coming*.
        left_alone: usize,
        cannot_land: usize,
        skipped_by_narrowing: usize,
        never_coming: usize,
    },
}

impl AnAct {
    /// The act's own name, which is the fixed line form's first field.
    pub fn name(&self) -> &'static str {
        match self {
            AnAct::AManualDownloadRequestEnded { .. } => "a manual download request ended",
            AnAct::AnAutoDownloadArrangementDone { .. } => "an auto-download arrangement done",
        }
    }
}

/// **The identities a line carries.**
///
/// The run identity of the run that wrote it (`Reporting a problem` U7) is
/// added by the diary service; these are the ones this document supplies.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct TheIdentitiesItCarries {
    /// **The server row and the installation identity the request was made
    /// under**, for the first act; **the job identity and the server row the
    /// arrangement holds**, for the second.
    pub server_row: Option<Identity>,
    pub installation: Option<Identity>,
    /// **The job identity where the request brought a job home *as a whole
    /// job*, by H5a's test and no other.**
    ///
    /// **A request of files the user picked out carries no job identity, and
    /// that absence is the line's own reading rather than a gap.**
    pub job: Option<Identity>,
}

/// **What neither line carries, held as a value rather than left to a
/// builder's restraint.**
///
/// **The names of the files behind any of those counts** — `Reporting a
/// problem` N3 admits the subjects of the recorded act and bars an inventory,
/// and the per-file account stays where it already lives, in the reckoning and
/// the request record. **Nor the reckoning's four-value server-file identity
/// readings, the staging paths, the transfer identities or the transfer
/// classes**, each a mechanism's own vocabulary under the fixed-form rule
/// `Reporting a problem` J3 states. **And no byte figure for an arrangement,
/// which has none.**
pub const NEITHER_LINE_CARRIES_A_FILE_NAME_A_STAGING_PATH_OR_A_TRANSFER_CLASS: bool = true;

/// **What is written below the act, and the answer is nothing.**
///
/// **No line per fetch, per publication, per walk cycle, per probe, per unit
/// re-arranged after a failure, per file marked *could not be vouched for* at
/// the ending walk, or per file told as a newer version on the server** —
/// each fails `Reporting a problem` J2a's third test.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhatIsNotWritten {
    APerFetchLine,
    APerPublicationLine,
    APerWalkCycleLine,
    APerProbeLine,
    APerReArrangedUnitLine,
    APerCouldNotBeVouchedForFileLine,
    APerNewerVersionTellingLine,
}

impl WhatIsNotWritten {
    pub const ALL: [WhatIsNotWritten; 7] = [
        WhatIsNotWritten::APerFetchLine,
        WhatIsNotWritten::APerPublicationLine,
        WhatIsNotWritten::APerWalkCycleLine,
        WhatIsNotWritten::APerProbeLine,
        WhatIsNotWritten::APerReArrangedUnitLine,
        WhatIsNotWritten::APerCouldNotBeVouchedForFileLine,
        WhatIsNotWritten::APerNewerVersionTellingLine,
    ];
}

/// **Two things that look like acts and are not.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhatLooksLikeAnActAndIsNot {
    /// **The destination-unwritable condition reaches the diary as a
    /// *failure* line**, captured verbatim at its first presentation by
    /// `Desktop app and delivery` §16's plumbing (`Reporting a problem` J6)
    /// and said once by its key. **This document declares nothing for it,
    /// because a failure line is not an act line and nothing here composes
    /// one.**
    TheDestinationUnwritableCondition,
    /// **Switching an arrangement on or off and changing its narrowing** are
    /// the user's own controls writing one record field, which the spec names
    /// as a control's effect and never as a completion the user is told about
    /// afterwards — **the second test, failed.**
    SwitchingAnArrangementOnOrOffAndChangingItsNarrowing,
}

/// **The narrowing material's arrival is not a told line either**: it is a
/// reading with its moment and **it makes a control available rather than
/// being an ending or a reading the user could name.**
pub const THE_NARROWING_MATERIALS_ARRIVAL_IS_NOT_A_TOLD_LINE: bool = true;

#[cfg(test)]
mod tests {
    use super::*;
    use smoozy_reporting::lines::LineKind;

    #[test]
    fn this_module_declares_the_act_kind_and_no_told_kind() {
        assert_eq!(DECLARED, [LineKind::Act]);
        assert!(!DECLARED.contains(&LineKind::Told));
        assert!(!DECLARED.contains(&LineKind::Failure));
    }

    #[test]
    fn there_are_two_acts_and_each_has_its_own_fixed_name() {
        let request = AnAct::AManualDownloadRequestEnded {
            destination: b"/Users/a/Downloads".to_vec(),
            ending: HowARequestEnded::Completed,
            units_fetched: 3,
            the_whole_in_files: 4,
            left_alone: 1,
            cannot_land: 0,
            refused_by_the_enumeration: 0,
        };
        let arrangement = AnAct::AnAutoDownloadArrangementDone {
            reading: TheArrangementsReading::Ended {
                arrivals: 9,
                whole: 10,
                the_whole_may_grow: false,
                also_home_outside_the_whole: 0,
                folders: vec![b"/Users/a/Downloads".to_vec()],
            },
            folders: vec![b"/Users/a/Downloads".to_vec()],
            left_alone: 0,
            cannot_land: 0,
            skipped_by_narrowing: 0,
            never_coming: 1,
        };
        assert_ne!(request.name(), arrangement.name());
    }

    #[test]
    fn a_request_of_files_picked_out_carries_no_job_identity() {
        let picked_out = TheIdentitiesItCarries::default();
        assert!(picked_out.job.is_none());
    }

    #[test]
    fn nothing_below_the_act_is_written_and_the_list_is_the_seven_the_plan_names() {
        assert_eq!(WhatIsNotWritten::ALL.len(), 7);
    }
}
