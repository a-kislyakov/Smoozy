//! The listing cache and the stale listing — `File Browser` §1.5, L14 to L18,
//! Rows 198 and 346.
//!
//! **Constraint:** Row 198 — the listing stays on screen, says plainly it is
//! not live, everything needing the machine is unavailable rather than failing
//! on a click, and the listing is read again when the connection comes back.
//!
//! **Markers used below that belong to another document, each with the
//! document it belongs to.** `Servers` S8, S10, S11, S12, W21, W25, D17, P11,
//! §15, 12.2. `Server protocol and transfers` C14, C18. `State, identity and
//! recovery` D4, §11.1.

use smoozy_servers::condition::ObservedValue;
use smoozy_servers::period::PeriodEnding;
use smoozy_servers::transfers::ConnectionLabel;

use crate::blend_version::TagsThisPeriod;
use crate::free_space::BrowsingFigure;
use crate::listing::CompleteListing;

/// **L14: the cache is a stored tree, per server row, in process memory — and
/// it is deliberately not durable.**
///
/// *"One entry per folder Smoozy has listed in this connected period, holding
/// that folder's last complete listing and the moment it was taken; the
/// `.blend` version tags and the free-space figure ride the same entries."*
///
/// **Opening a folder Smoozy has not already seen is unavailable while the line
/// is down precisely because the ones it has seen are in this tree** — a stored
/// tree rather than a stored screen.
///
/// **It does not survive Smoozy closing, and must not**: `State, identity and
/// recovery` §1.1's one durable store holds no copy of an observed machine
/// state the machine still holds (D4), and a durable listing would be a thing
/// that can be lost and a thing that can be wrong for good. **A fresh sitting
/// reads afresh.** There is no write route out of this type, which is how that
/// is kept.
///
/// **Its lifetime has no class name and wants none.** §11.1 names the listing
/// tree among the objects outside its five classes, on the tombstone treatment
/// it already establishes, **with the lifetime kept at L16** — which is
/// [`ListingTree::a_period_ended`] below. It is not **period-held**: it stands
/// at one of `Servers` S10's six endings.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct ListingTree {
    /// The server row this tree is keyed to. **L16: an entry left standing
    /// against a removed row would be a remembered observed machine state for a
    /// machine the user has just told Smoozy to forget** — D4, and `Servers`
    /// D17's enumeration, which the removal walks entire.
    server_row: String,
    folders: Vec<FolderEntry>,
    tags: TagsThisPeriod,
}

/// One folder's entry in the tree.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FolderEntry {
    pub path: String,
    /// **That folder's last complete listing**, never a fragment
    /// ([`crate::listing::A_CUT_PASS_NEVER_REPLACES_THE_LAST_COMPLETE_ONE`]).
    pub listing: CompleteListing,
    /// The free-space figure rides the same entry (L14).
    pub free_space: Option<BrowsingFigure>,
}

impl ListingTree {
    pub fn for_server_row(server_row: impl Into<String>) -> ListingTree {
        ListingTree {
            server_row: server_row.into(),
            folders: Vec::new(),
            tags: TagsThisPeriod::new(),
        }
    }

    pub fn server_row(&self) -> &str {
        &self.server_row
    }

    pub fn tags(&self) -> &TagsThisPeriod {
        &self.tags
    }

    pub fn tags_mut(&mut self) -> &mut TagsThisPeriod {
        &mut self.tags
    }

    /// A pass was accepted: it replaces that folder's entry.
    ///
    /// **Only an accepted pass reaches here** — the type
    /// [`crate::listing::DirectoryRead`] has no route to its entries until it
    /// is accepted, so a cut pass cannot replace a complete one.
    pub fn a_folder_was_listed(&mut self, path: impl Into<String>, listing: CompleteListing) {
        let path = path.into();
        self.folders.retain(|entry| entry.path != path);
        self.folders.push(FolderEntry {
            path,
            listing,
            free_space: None,
        });
    }

    pub fn folder(&self, path: &str) -> Option<&FolderEntry> {
        self.folders.iter().find(|entry| entry.path == path)
    }

    pub fn folders_held(&self) -> usize {
        self.folders.len()
    }

    /// **L16: what becomes of the tree, said per ending.**
    ///
    /// `Servers` S10 closes the list at six, and `Servers` §15's sentence is
    /// the rule underneath all six — *a period ending drops every live
    /// reading*, **so a listing goes stale rather than empty** — and the tree
    /// is what is left when the live reading is dropped, never a live reading
    /// itself.
    pub fn a_period_ended(&mut self, ending: PeriodEnding) {
        if what_an_ending_does(ending) == WhatBecomesOfTheTree::ItGoes {
            self.folders.clear();
            self.tags.the_period_ended();
        }
    }
}

/// L16's two outcomes, and there is no third.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhatBecomesOfTheTree {
    /// **Thrown away by the user rather than taken from them.**
    ItGoes,
    /// **It stands, with its moment, exactly as L15's does.**
    ItStands,
}

/// **L16, per ending.**
///
/// - **The user disconnects**, **the application run ends**, and **the edit's
///   save on a server whose period stands**: the tree goes. All three are the
///   user's own act about that server — the run ending takes process memory
///   with it in any case — and there the listing is thrown away by the user
///   rather than taken from them. `Servers` S8 is why the third is named as
///   the save: *"The edit put-back is not in any of the three lists"*, so it is
///   period-opening and never a replacement, and L17's two labels never see it.
/// - **The server row is removed**: the tree goes, and the fate is the above
///   rather than a distinct one. What this ending adds is the **moment** and
///   the **key**, and only those — the moment is `Servers` S10 item 6's, the
///   removal's local forget rather than the request. **A removal that stops
///   part-way and leaves the row standing** — `Servers` W21's *try again* and
///   *try the remainder again* — **ends no period, so nothing here fires and
///   the tree stands under L15 like any other standing period's.**
/// - **The opening act's connection never becomes live**: there is nothing to
///   decide, because no period ever held a listing to show. The tree goes with
///   the period that never held one, which is the same thing as there being
///   nothing to keep.
/// - **A break is declared while the server's key is not usable** (`Servers`
///   P11): **the tree stands, with its moment, exactly as L15's does.** This is
///   the one ending that arrives underneath a user who is looking at a listing,
///   and nothing about that listing's truth changed when the period ended;
///   emptying it here would be the *"throwing away what the user was looking
///   at"* Row 198 refuses. **There is no repair to wait for** — P11 forbids the
///   attempt — so nothing is shown as pending and the route back is the user's
///   own connect.
pub fn what_an_ending_does(ending: PeriodEnding) -> WhatBecomesOfTheTree {
    match ending {
        PeriodEnding::TheUserDisconnects
        | PeriodEnding::TheApplicationRunEnds
        | PeriodEnding::TheEditsSave
        | PeriodEnding::TheRowIsRemovedAtItsLocalForget
        | PeriodEnding::TheOpeningConnectionNeverBecameLive => WhatBecomesOfTheTree::ItGoes,
        PeriodEnding::ABreakDeclaredWithTheKeyNotUsable => WhatBecomesOfTheTree::ItStands,
    }
}

/// **L15: a drop does not empty it, because a drop does not end the period**
/// (`Servers` S12).
///
/// While the connection's observed value is *reconnecting* or *disconnected
/// under a standing period*, the browser composes from the cache: the listing
/// shown with its moment, stale said plainly, **every machine-needing act
/// unavailable rather than failing** — opening an unseen folder, uploading,
/// downloading, deleting, renaming, moving, copying, making a folder. **Folders
/// already in the tree stay openable from the tree.**
///
/// **The connection's observed value comes from `Servers`' three values and is
/// never derived here — no ping, no probe request** (`Server protocol and
/// transfers` C18).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HowTheBrowserComposes {
    /// The line is up: the listing is live and every act is available.
    Live,
    /// **L15 and L18.** The listing is shown with its moment, stale said
    /// plainly, and every machine-needing act is unavailable rather than
    /// failing on a click.
    FromTheCacheAndStale,
}

/// The composition, read off `Servers`' own observed value and nothing else.
pub fn how_the_browser_composes(observed: ObservedValue) -> HowTheBrowserComposes {
    match observed {
        ObservedValue::Connected => HowTheBrowserComposes::Live,
        ObservedValue::Reconnecting | ObservedValue::Disconnected => {
            HowTheBrowserComposes::FromTheCacheAndStale
        }
    }
}

/// Every act of this document's that needs the machine — **L15's own list**,
/// closed here so that *unavailable rather than failing on a click* is decided
/// once rather than at each control.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AnActThatNeedsTheMachine {
    OpeningAFolderSmoozyHasNotSeen,
    Uploading,
    Downloading,
    Deleting,
    Renaming,
    Moving,
    Copying,
    MakingAFolder,
}

impl AnActThatNeedsTheMachine {
    pub const ALL: [AnActThatNeedsTheMachine; 8] = [
        AnActThatNeedsTheMachine::OpeningAFolderSmoozyHasNotSeen,
        AnActThatNeedsTheMachine::Uploading,
        AnActThatNeedsTheMachine::Downloading,
        AnActThatNeedsTheMachine::Deleting,
        AnActThatNeedsTheMachine::Renaming,
        AnActThatNeedsTheMachine::Moving,
        AnActThatNeedsTheMachine::Copying,
        AnActThatNeedsTheMachine::MakingAFolder,
    ];
}

/// What a control does while the line is down.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Availability {
    Available,
    /// **Unavailable rather than failing on a click** — Row 198's own words.
    Unavailable,
}

/// **L15, per act.** *"Folders already in the tree stay openable from the
/// tree"*, which is the one act that is available while the line is down and
/// only for a folder the tree holds.
pub fn availability(
    act: AnActThatNeedsTheMachine,
    composing: HowTheBrowserComposes,
    the_folder_is_already_in_the_tree: bool,
) -> Availability {
    match composing {
        HowTheBrowserComposes::Live => Availability::Available,
        HowTheBrowserComposes::FromTheCacheAndStale => match act {
            AnActThatNeedsTheMachine::OpeningAFolderSmoozyHasNotSeen
                if the_folder_is_already_in_the_tree =>
            {
                Availability::Available
            }
            _ => Availability::Unavailable,
        },
    }
}

/// **L17: when `Servers` hands over a repaired connection, the label decides.**
///
/// A **replacement** re-reads the folder the browser is standing in — the
/// spec's own instruction, *"the listing is read again and the controls
/// return"* — and the cache's other entries stand until re-opened, each still
/// carrying its own moment. A **period-opening** connection starts a fresh
/// tree.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhatTheLabelDoes {
    /// Re-read the folder the browser is standing in; the other entries stand.
    ReReadTheStandingFolder,
    /// A fresh tree.
    StartAFreshTree,
}

pub fn what_the_label_does(label: ConnectionLabel) -> WhatTheLabelDoes {
    match label {
        ConnectionLabel::Replacement => WhatTheLabelDoes::ReReadTheStandingFolder,
        ConnectionLabel::PeriodOpening => WhatTheLabelDoes::StartAFreshTree,
    }
}

/// **L17's gate: before anything is trusted, the installation-identity gate has
/// run** (`Server protocol and transfers` C14).
///
/// On *present and different* or *absent*, **the cached tree is disqualified
/// with everything else machine-derived** — it describes a machine that is gone
/// — and the browser reads the fresh machine from nothing.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TheIdentityGate {
    /// The same installation as the tree was built under.
    PresentAndTheSame,
    /// A different installation. **The machine is gone.**
    PresentAndDifferent,
    /// No installation identity could be read at all.
    Absent,
}

impl TheIdentityGate {
    pub fn the_tree_is_disqualified(self) -> bool {
        !matches!(self, TheIdentityGate::PresentAndTheSame)
    }
}

/// **L18: a destroyed machine and a restarted one look alike at first, and the
/// browser does not decide which it is.**
///
/// `Servers` owns the repair and the telling apart; **what the browser does is
/// the same in both cases and in the *not yet known* stretch**: the stale
/// listing stands with its moment, the controls stay unavailable, and they
/// return only on a live connection. **On a machine that never answers again
/// the listing is stale for good and the controls never return**, which is this
/// promise as written.
pub const THE_BROWSER_NEVER_DECIDES_WHETHER_A_MACHINE_IS_GONE: bool = true;

/// **L14's last sentence, and it is a real state rather than a case nobody
/// meets**: a server this run never held a period for shows no listing,
/// because no entry was ever made.
pub fn a_server_with_no_tree_shows_no_listing(tree: Option<&ListingTree>) -> bool {
    tree.is_none_or(|tree| tree.folders_held() == 0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::listing::{ApparentSize, EntryName, Kind, ListedEntry};
    use std::time::SystemTime;

    fn a_listing(at: u64) -> CompleteListing {
        CompleteListing::of(
            vec![ListedEntry {
                name: EntryName::from_the_machine(b"scene.blend".to_vec()),
                size: ApparentSize::of(1),
                last_changed: None,
                kind: Kind::File,
            }],
            SystemTime::UNIX_EPOCH + std::time::Duration::from_secs(at),
            false,
        )
    }

    #[test]
    fn a_break_while_the_key_is_not_usable_leaves_the_listing_standing() {
        // The one ending that arrives underneath a user who is looking at a
        // listing. Emptying it here is what Row 198 refuses.
        let mut tree = ListingTree::for_server_row("row-1");
        tree.a_folder_was_listed("/jobs", a_listing(10));
        tree.a_period_ended(PeriodEnding::ABreakDeclaredWithTheKeyNotUsable);
        assert_eq!(tree.folders_held(), 1);
        assert!(tree.folder("/jobs").is_some());
    }

    #[test]
    fn the_other_five_endings_take_the_tree() {
        for ending in [
            PeriodEnding::TheUserDisconnects,
            PeriodEnding::TheApplicationRunEnds,
            PeriodEnding::TheEditsSave,
            PeriodEnding::TheRowIsRemovedAtItsLocalForget,
            PeriodEnding::TheOpeningConnectionNeverBecameLive,
        ] {
            let mut tree = ListingTree::for_server_row("row-1");
            tree.a_folder_was_listed("/jobs", a_listing(10));
            tree.a_period_ended(ending);
            assert_eq!(tree.folders_held(), 0, "{ending:?} left the tree standing");
        }
    }

    #[test]
    fn every_ending_servers_declares_is_answered_here() {
        // `Servers` S10 closes the list at six. A seventh arriving there must
        // fail here rather than fall into a default.
        assert_eq!(PeriodEnding::ALL.len(), 6);
        for ending in PeriodEnding::ALL {
            let _ = what_an_ending_does(ending);
        }
    }

    #[test]
    fn the_tags_go_with_the_tree_and_stand_with_it() {
        use crate::blend_version::{Tag, TagKey, WhyItIsBlank};

        let mut tree = ListingTree::for_server_row("row-1");
        tree.tags_mut().remember(
            TagKey {
                path: "/jobs/a.blend".to_string(),
                size: ApparentSize::of(1),
                last_changed: None,
            },
            Tag::Blank(WhyItIsBlank::NoRecognisedOpeningShape),
        );
        tree.a_period_ended(PeriodEnding::ABreakDeclaredWithTheKeyNotUsable);
        assert_eq!(tree.tags().how_many_are_held(), 1);
        tree.a_period_ended(PeriodEnding::TheUserDisconnects);
        assert_eq!(tree.tags().how_many_are_held(), 0);
    }

    #[test]
    fn a_drop_does_not_empty_the_tree_and_every_machine_act_goes_unavailable() {
        // A drop does not end the period, so nothing here fires at all; what
        // changes is the composition.
        assert_eq!(
            how_the_browser_composes(ObservedValue::Reconnecting),
            HowTheBrowserComposes::FromTheCacheAndStale
        );
        assert_eq!(
            how_the_browser_composes(ObservedValue::Disconnected),
            HowTheBrowserComposes::FromTheCacheAndStale
        );
        assert_eq!(
            how_the_browser_composes(ObservedValue::Connected),
            HowTheBrowserComposes::Live
        );

        for act in AnActThatNeedsTheMachine::ALL {
            let seen_before = matches!(
                act,
                AnActThatNeedsTheMachine::OpeningAFolderSmoozyHasNotSeen
            );
            assert_eq!(
                availability(act, HowTheBrowserComposes::FromTheCacheAndStale, false),
                Availability::Unavailable,
                "{act:?} was available with the line down"
            );
            // And the one exception: a folder already in the tree opens from it.
            if seen_before {
                assert_eq!(
                    availability(act, HowTheBrowserComposes::FromTheCacheAndStale, true),
                    Availability::Available
                );
            }
        }
    }

    #[test]
    fn the_two_labels_decide_and_nothing_else_does() {
        assert_eq!(
            what_the_label_does(ConnectionLabel::Replacement),
            WhatTheLabelDoes::ReReadTheStandingFolder
        );
        assert_eq!(
            what_the_label_does(ConnectionLabel::PeriodOpening),
            WhatTheLabelDoes::StartAFreshTree
        );
    }

    #[test]
    fn a_recycled_machine_disqualifies_the_tree() {
        assert!(!TheIdentityGate::PresentAndTheSame.the_tree_is_disqualified());
        assert!(TheIdentityGate::PresentAndDifferent.the_tree_is_disqualified());
        assert!(TheIdentityGate::Absent.the_tree_is_disqualified());
    }

    #[test]
    fn a_server_this_run_never_held_a_period_for_shows_no_listing() {
        assert!(a_server_with_no_tree_shows_no_listing(None));
        let tree = ListingTree::for_server_row("row-1");
        assert!(a_server_with_no_tree_shows_no_listing(Some(&tree)));
    }

    #[test]
    fn a_second_pass_over_one_folder_replaces_it_rather_than_sitting_beside_it() {
        let mut tree = ListingTree::for_server_row("row-1");
        tree.a_folder_was_listed("/jobs", a_listing(10));
        tree.a_folder_was_listed("/jobs", a_listing(20));
        assert_eq!(tree.folders_held(), 1);
        assert_eq!(
            tree.folder("/jobs").unwrap().listing.taken_at,
            SystemTime::UNIX_EPOCH + std::time::Duration::from_secs(20)
        );
    }
}
