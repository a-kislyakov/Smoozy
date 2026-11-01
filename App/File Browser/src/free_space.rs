//! The free-space figure — `File Browser` §1.4, L11 to L13, Rows 184 and 346.
//!
//! **Constraint:** Row 184 — *"The browser also shows how much free disk space
//! is left on the machine's disk."*
//!
//! **Markers used below that belong to another document, each with the
//! document it belongs to.** `Server protocol and transfers` F1, F2, F3, F10,
//! F16. `Servers` §15.
//!
//! **A marker this plan also defines is deliberately not listed above,
//! whichever document is meant, and is written out with its document at every
//! site instead** — the header list cannot tell the two apart, and a bare one
//! would resolve to a real and wrong clause.

use std::time::SystemTime;

use smoozy_server_protocol::free_space::{FreeSpaceReading, WhyThereIsNoReading};

/// **L11: the figure is a per-path reading against the folder being
/// browsed**, taken by `Server protocol and transfers` F2's two-route
/// negotiation and reported unavailable only where both routes are absent.
///
/// **Uploads land where the user is browsing, so the browsed folder's mount is
/// the mount the figure is about.** The available figure is the one read, never
/// the free figure (F3).
///
/// A reading against the browsed path counts memory-backed mounts and the
/// host's bind-mounted disk **only when the user is actually browsing inside
/// one**, which is then true of the place shown; **nothing here ever sums,
/// totals or picks a biggest** (F1), and **the figure is never widened into a
/// statement about the machine** (F16).
///
/// `Desktop app and delivery` M17: the disk-full message reaches no further
/// than this reading does.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BrowsingFigure {
    /// The folder the figure is about — **the one being browsed, and never a
    /// mount, a device or the machine**.
    pub folder: String,
    /// The reading, or the stated reason there is none. **Both routes absent is
    /// the only unavailable case** and it is that document's own value.
    pub reading: Result<FreeSpaceReading, WhyThereIsNoReading>,
    /// **L12: presented as of the moment it was taken, on the machine's
    /// clock.** The clock is the reading's own and is not restated here.
    pub taken_at_on_the_machines_clock: SystemTime,
    /// **L13: while the listing is stale the figure is stale with it and reads
    /// as such** (`Servers` §15; Design's *While the connection is down*
    /// carries the presentation).
    pub the_listing_it_rides_is_stale: bool,
}

/// **L12: when the figure is re-taken, and the list is closed.**
///
/// *"It is re-taken at each folder navigation and when an operation of this
/// document's own — a publication, a delete, a move's completion — lands inside
/// the browsed tree."*
///
/// **It is never re-read to keep a message fresh** (F10).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhenItIsReTaken {
    /// The user opened another folder.
    AFolderNavigation,
    /// A publication of this document's landed inside the browsed tree.
    APublicationInsideTheBrowsedTree,
    /// A delete of this document's landed inside the browsed tree.
    ADeleteInsideTheBrowsedTree,
    /// A move of this document's completed inside the browsed tree.
    AMovesCompletionInsideTheBrowsedTree,
}

impl WhenItIsReTaken {
    /// The closed list. **A moment not on it does not re-take the figure**,
    /// which is F10 kept by there being nowhere else to ask.
    pub const ALL: [WhenItIsReTaken; 4] = [
        WhenItIsReTaken::AFolderNavigation,
        WhenItIsReTaken::APublicationInsideTheBrowsedTree,
        WhenItIsReTaken::ADeleteInsideTheBrowsedTree,
        WhenItIsReTaken::AMovesCompletionInsideTheBrowsedTree,
    ];

    /// Whether an act of this document's that landed at `path` re-takes the
    /// figure for the folder being browsed. **Inside the browsed tree** is a
    /// containment test on path text, per X5's byte rule — the same comparison
    /// everything else here uses, and no resolution of any kind.
    pub fn an_act_at(path: &str, inside_the_browsed_tree: &str) -> bool {
        crate::already_exists::is_inside(path.as_bytes(), inside_the_browsed_tree.as_bytes())
    }
}

/// **L12's split with `Servers`, stated so nobody makes a second figure.**
///
/// *"That document's remembered figure serves the server list, this one serves
/// the browser, and they cannot disagree in kind because both are F1's
/// per-path reading."*
pub const THE_TWO_FIGURES_ARE_ONE_KIND_OF_READING_ON_TWO_PATHS: bool = true;

/// **L13: nothing is cleared, warned about or decided from the figure.**
///
/// Stated as a constant because it is a rule about this crate's callers — the
/// figure is shown, and it is not an input to any judgement here. The
/// will-it-fit prohibition's other half is
/// [`crate::listing::NO_WILL_IT_FIT_DECISION_IS_MADE_FROM_A_LISTED_SIZE`].
pub const NOTHING_IS_DECIDED_FROM_THE_FREE_SPACE_FIGURE: bool = true;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn an_act_inside_the_browsed_tree_re_takes_the_figure_and_one_outside_does_not() {
        assert!(WhenItIsReTaken::an_act_at("/jobs/scene.blend", "/jobs"));
        assert!(WhenItIsReTaken::an_act_at(
            "/jobs/nested/scene.blend",
            "/jobs"
        ));
        assert!(!WhenItIsReTaken::an_act_at("/other/scene.blend", "/jobs"));
        // A name that merely begins with the folder's text is not inside it.
        assert!(!WhenItIsReTaken::an_act_at(
            "/jobsibling/scene.blend",
            "/jobs"
        ));
    }

    #[test]
    fn the_re_taking_moments_are_a_closed_list_of_four() {
        assert_eq!(WhenItIsReTaken::ALL.len(), 4);
    }

    #[test]
    fn a_figure_carries_its_folder_its_moment_and_whether_it_is_stale() {
        let figure = BrowsingFigure {
            folder: "/jobs".to_string(),
            reading: Err(WhyThereIsNoReading::OnlyTheCarriageIsLeftAndItsProgramIsUnestablished),
            taken_at_on_the_machines_clock: SystemTime::UNIX_EPOCH,
            the_listing_it_rides_is_stale: true,
        };
        assert!(figure.reading.is_err());
        assert!(figure.the_listing_it_rides_is_stale);
    }
}
