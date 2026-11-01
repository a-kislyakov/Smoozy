//! **§3 W1 and W4 performed — the walk itself.**
//!
//! One directory reading per folder over the file-transfer subsystem's reading
//! route, descending through the whole tree, every entry coming back as its
//! path relative to the job output folder. **The cadence, the window, the walk
//! entry's shape and the change detection are [`crate::watch`]'s and are not
//! taken again here**; what is here is the reading and the sequencing of it.
//!
//! # Why the reading route is this crate's own trait and not `File Browser`'s
//!
//! `File Browser` L1 already has a reading channel and this walk rides the
//! same one — `Server protocol and transfers` T5 gives a connection **one**
//! long-lived reading channel and W2's aggregate bound is arithmetic about
//! that channel. **What is not shared is the type of a path.** That crate's
//! [`ReadsADirectory`](smoozy_file_browser::listing::ReadsADirectory) takes a
//! path as text, because a browser shows text; this walk's every key is the
//! inventory's own **byte** path relative to the job output folder
//! ([`AWalkEntry::path`]), and the subsystem's own operations take bytes. So a
//! text-taking trait would put a bytes-to-text conversion between the machine
//! and the inventory's key, on exactly the names `Job files and outputs` N15
//! and `File Browser` L1a keep as bytes for.
//!
//! **The serialisation W2 depends on is kept the same way it is kept there**:
//! one of these holds one `&mut` to one subsystem, so two readings cannot be
//! in flight on it at once, whichever trait asked.
//!
//! # A walk that did not complete is discarded whole
//!
//! `Job files and outputs` C9's rule, applied to this walk (W1's fourth
//! reason). **One failed reading anywhere in the tree discards the whole
//! walk** — not the branch — because a walk is one reading and a tree missing
//! a branch is a folder with fewer files in it, which is the one shape that
//! would put every frame of a job into *never coming*.
//!
//! **The separated cause rides beside the discard rather than inside it.**
//! [`crate::watch::WhyTheWalkIsNotAReading`] carries no status; W5 asks for
//! the truth *"with the separated cause where the route separates it"*, so
//! [`TheWalksAnswer`] carries the protocol's own failure as itself. **Nothing
//! here parses a message and nothing composes a cause**: the status is
//! `Server protocol and transfers` §1.3's own value, carried.
//!
//! **Three of its four values are read off a failure and the fourth is not.**
//! An entry whose kind the reply did not carry discards the walk with **no
//! status at all**, because nothing failed: the listing arrived and one of its
//! entries did not say what it was.
//!
//! # W6 is kept by there being nothing here that writes
//!
//! **Nothing is planted, renamed, opened or changed on the machine to make
//! watching easier.** This module calls one operation on the subsystem — the
//! directory reading — and no other.
//!
//! **Markers used in this file that belong to another document, each with the
//! document it belongs to.** `Job files and outputs` C7, C9, N15. `File
//! Browser` L1, L1a. `Server protocol and transfers` §1.3, T5.

use std::time::Duration;

use smoozy_server_protocol::subsystem::{StatusCode, SubsystemFailure};

use crate::watch::{
    the_pause_before_the_next_walk, AWalk, AWalkEntry, TheWindow, WhyTheWalkIsNotAReading,
};

/// One entry of one directory reading, as the route hands it back.
///
/// **The four fields W4 names**, and nothing else is read: the name as the
/// bytes the subsystem returned, whether it is a folder, its size and its
/// modification time.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ADirectoryEntry {
    pub name: Vec<u8>,
    pub kind: TheEntrysKind,
    pub size: u64,
    pub modification_time: i64,
}

/// **W1 treats an entry's *kind* as a field the route supplies, and on this
/// route it may not be there.**
///
/// **The absence is a value rather than a false.** On the file-transfer
/// subsystem the kind is read out of the listing's permissions field, which
/// revision 3 makes optional and which a server fills only where it set the
/// permissions flag in its reply; the attributes structure carries no separate
/// kind field at all. **An absent value read as *this is not a folder* is a
/// silent wrong answer**, and it is the one wrong answer nothing else here
/// could catch: a job's `beauty/`, `mist/` and `depth/` sub-folders would come
/// back as files, the walk would never descend, and it would still report a
/// **complete** walk because nothing failed.
///
/// **Whether every server this app renders on carries the kind is an open
/// question** parked with the protocol crate's owner, and it is not answered
/// here or guessed at.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TheEntrysKind {
    AFolder,
    /// **Not a folder, and the listing said it is a plain file.** Added
    /// 2026-09-30, step 10c's eleventh-pass fix piece 2, so a caller can tell
    /// an entry whose size is the file's own from one whose size is not.
    APlainFile,
    /// **Not a folder, and not known to be a plain file** — a symbolic link,
    /// whose listed size is the link's own and not what it reaches, or
    /// anything else that is not a folder.
    NotAFolder,
    /// **The reply did not carry it.**
    ItWasNotCarried,
}

impl TheEntrysKind {
    /// Whether the entry is a folder, **or nothing where the reply did not
    /// say.**
    pub fn is_a_folder(self) -> Option<bool> {
        match self {
            TheEntrysKind::AFolder => Some(true),
            TheEntrysKind::APlainFile | TheEntrysKind::NotAFolder => Some(false),
            TheEntrysKind::ItWasNotCarried => None,
        }
    }
}

/// **The one reading this module takes, and the far side is a machine.**
///
/// One directory, batched at the protocol's own reply size by the route
/// itself, answering only once the server has reported the directory's end —
/// which is what makes a cut listing a failure here rather than a short
/// reading (W1's fourth reason). [`super::wiring`] is the real end.
pub trait ReadsOneDirectoryOnTheMachine {
    fn read_one_directory(
        &mut self,
        path: &[u8],
    ) -> impl std::future::Future<Output = Result<Vec<ADirectoryEntry>, SubsystemFailure>> + Send;
}

/// **W5's discard, with the separated cause beside it.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TheWalksAnswer {
    pub walk: AWalk,
    /// **The protocol's own status, carried as itself** where the walk was
    /// discarded on one. It is what the truth-telling of W5 names the cause
    /// from.
    ///
    /// **It is absent on a complete walk, and absent on the one discard the
    /// machine said nothing about** — an entry whose kind the reply did not
    /// carry. Nothing failed there: the listing arrived and one of its entries
    /// did not say what it was.
    pub what_the_machine_said: Option<SubsystemFailure>,
    /// How long the reading took, **measured from its start to its end** —
    /// which is what W2's floor and its from-the-end rule both take.
    pub it_took: Duration,
    /// **The entries of a complete walk the listing said are plain files**, by
    /// path. Added 2026-09-30, step 10c's eleventh-pass fix piece 2: a size
    /// the walk read is the file's own only for these, so only these may
    /// stand in for a size read of the file itself. Empty on a discard.
    pub the_plain_files: Vec<Vec<u8>>,
}

/// **W5's three reasons that are read off the protocol's own status, and off
/// nothing else.**
///
/// **The fourth reason is not one of these** — an entry whose kind the reply
/// did not carry is a discard with no failure behind it, decided at the walk
/// itself.
///
/// - **The channel is gone** is the connection going. The connection condition
///   `Servers` supplies covers the dead-machine case, and the window closes
///   with it.
/// - **A lost or absent connection status, or an answer this revision does not
///   define at that point**, is a listing cut mid-flight.
/// - **Everything else the server refused is a refusal**, which is
///   [`WhyTheWalkIsNotAReading::ARefusal`]'s own words — *this document could
///   not read the job's folder* — with the separated cause carried beside it.
///   **An absent folder lands here**, because none of the other values means
///   it and inventing one would be rewriting the rules half; what separates it
///   from a permission refusal is the status, which is carried.
fn why_it_is_not_a_reading(failure: &SubsystemFailure) -> WhyTheWalkIsNotAReading {
    match failure {
        SubsystemFailure::TheChannelIsGone => WhyTheWalkIsNotAReading::TheConnectionWent,
        SubsystemFailure::TheAnswerCouldNotBeRead
        | SubsystemFailure::TheServerRefused(StatusCode::ConnectionLost)
        | SubsystemFailure::TheServerRefused(StatusCode::NoConnection) => {
            WhyTheWalkIsNotAReading::ItWasCut
        }
        _ => WhyTheWalkIsNotAReading::ARefusal,
    }
}

/// **W4: one walk of the folder and everything under it.**
///
/// `Job files and outputs` C7's shape, **because the redirect deepens paths by
/// construction and a single directory's entries would miss the inventory's
/// predicted paths.**
///
/// The descent is depth-first over an explicit list of folders still to read —
/// the list is taken from its end — so the depth of the tree costs a list
/// entry and never a stack frame, and **the entries come back in the order the
/// folders were met**, which is the order the change detection and the ending
/// walk both read them in. **Which of the two orders the descent takes decides
/// nothing here**: every reading is keyed by path and no clause of this
/// document reads a walk's order.
///
/// **Folders are entries too.** [`AWalkEntry::is_a_folder`] carries it, and
/// [`crate::ending::the_ending_walk`] skips them when it decides what has no
/// frame; nothing here drops one, because a folder an added write place
/// created is a thing the arrangement's own reading has to be able to see —
/// **nothing but Smoozy's own staging folder**
/// ([`is_smoozys_own_staging_folder`]), which no render made.
///
/// **A symbolic link is not descended into**, and that is a decision rather
/// than an oversight: the route reads the link's own entry, the walk keeps no
/// visited set — `Job files and outputs` C7 gives it none — and a link back up
/// a tree would walk for ever. What it costs is a folder reached only through
/// a link inside a job output folder, which `Job files and outputs` L7 says
/// nothing but a render puts there; **what it never costs is a wrong file.**
pub async fn walk_the_folder_and_everything_under_it<R>(
    reader: &mut R,
    the_job_output_folder: &[u8],
) -> TheWalksAnswer
where
    R: ReadsOneDirectoryOnTheMachine,
{
    let began = std::time::Instant::now();
    let mut entries: Vec<AWalkEntry> = Vec::new();
    let mut the_plain_files: Vec<Vec<u8>> = Vec::new();
    // The folders still to read, each as its own path relative to the job
    // output folder. The root is the empty path.
    let mut still_to_read: Vec<Vec<u8>> = vec![Vec::new()];

    while let Some(relative) = still_to_read.pop() {
        let absolute = under(the_job_output_folder, &relative);
        match reader.read_one_directory(&absolute).await {
            Err(failure) => {
                return TheWalksAnswer {
                    walk: AWalk::Discarded(why_it_is_not_a_reading(&failure)),
                    what_the_machine_said: Some(failure),
                    it_took: began.elapsed(),
                    the_plain_files: Vec::new(),
                }
            }
            Ok(read) => {
                for entry in read {
                    if entry.name == b"." || entry.name == b".." {
                        continue;
                    }
                    // **Smoozy's own staging folder is not a job's output**
                    // and is neither listed nor descended into — the fixes
                    // before 0.1.8, fault 3. An upload leaves one in every
                    // folder it writes into (`File Browser` X12's reserved
                    // name), the copy of a baked cache carried one into the
                    // job's folder, and the job's download brought it home.
                    if is_smoozys_own_staging_folder(&entry.name) {
                        continue;
                    }
                    let path = under(&relative, &entry.name);
                    // **An entry whose kind the reply did not carry makes this
                    // walk a reading that cannot be used**, so it is discarded
                    // whole and retried at the cadence. Reading the absence as
                    // *not a folder* would stop the descent silently and still
                    // report a complete walk.
                    let Some(is_a_folder) = entry.kind.is_a_folder() else {
                        return TheWalksAnswer {
                            walk: AWalk::Discarded(
                                WhyTheWalkIsNotAReading::AnEntrysKindWasNotCarried,
                            ),
                            what_the_machine_said: None,
                            it_took: began.elapsed(),
                            the_plain_files: Vec::new(),
                        };
                    };
                    if is_a_folder {
                        still_to_read.push(path.clone());
                    }
                    if entry.kind == TheEntrysKind::APlainFile {
                        the_plain_files.push(path.clone());
                    }
                    entries.push(AWalkEntry {
                        path,
                        is_a_folder,
                        size: entry.size,
                        modification_time: entry.modification_time,
                    });
                }
            }
        }
    }

    TheWalksAnswer {
        walk: AWalk::Complete(entries),
        what_the_machine_said: None,
        it_took: began.elapsed(),
        the_plain_files,
    }
}

/// **Whether a name is `File Browser` X12's reserved staging folder** — the one
/// name every walk and every enumeration of this crate steps over. Added
/// 2026-10-02, the fixes before 0.1.8, fault 3: **Smoozy's own working files
/// are never brought home as anything's output**, and what sits in that folder
/// is a transfer's staged bytes or nothing at all.
pub fn is_smoozys_own_staging_folder(name: &[u8]) -> bool {
    name == smoozy_file_browser::partial::THE_RESERVED_STAGING_FOLDER.as_bytes()
}

/// One path under another, **as bytes throughout** and with no separator
/// doubled. An empty parent gives the child alone, which is what makes the
/// walk's own root the empty path.
fn under(parent: &[u8], child: &[u8]) -> Vec<u8> {
    if parent.is_empty() {
        return child.to_vec();
    }
    if child.is_empty() {
        return parent.to_vec();
    }
    let mut out = parent.to_vec();
    if !out.ends_with(b"/") {
        out.push(b'/');
    }
    out.extend_from_slice(child.strip_prefix(b"/".as_ref()).unwrap_or(child));
    out
}

/// **W1's own sentence held by the type: the pause sits inside a running
/// sequence and is not a standing timer.**
///
/// *"It is not a standing timer: its pause sits inside a running sequence
/// bounded by W3's window"*, which is what `Desktop app and delivery` P10b
/// admits. **So there is no clock, no scheduler and no registration
/// anywhere in this module.** [`TheWalkSeries`] holds the state one series
/// needs between two walks and answers, at each turn, how long to wait and
/// whether to walk at all; **the waiting itself is the caller's**, inside the
/// sequence it is already running.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct TheWalkSeries {
    /// The last **complete** walk, which stays the baseline until another
    /// completes (W5). A discarded walk never becomes one.
    the_previous_complete_walk: Vec<AWalkEntry>,
    /// **The entries that were *active* in that last complete walk**, kept
    /// beside it because E1 step 2's fallback asks for them by name — *"the
    /// entries active in the last complete walk"* — and the walk itself is a
    /// different and much larger set.
    ///
    /// **The two must not be confused.** The whole of the last complete walk
    /// is every file in the job's output folder; marking all of them *could
    /// not be vouched for* would tell a user whose job ended badly that all
    /// 720 files may be incomplete, which is the teaching J9b objects to from
    /// the other side.
    the_previous_complete_walks_active_entries: Vec<AWalkEntry>,
    /// Whether a complete walk has ever run in this series — which is what
    /// J6's baseline turns on, and it is not the same question as the walk
    /// being empty.
    a_complete_walk_has_run: bool,
    /// **W2's floor**: how long the previous walk took, measured from its end.
    the_previous_walk_took: Duration,
}

/// What one turn of the series comes to.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WhatTheTurnCameTo {
    /// **W3: the window is shut**, so no walk ran and nothing changed. The
    /// series keeps its baseline for when the window opens again.
    TheWindowIsShutSoNoWalkRan,
    /// A complete walk, with what is active against the previous complete one
    /// (W5) and how long to pause before the next turn (W2).
    AReading {
        active: Vec<AWalkEntry>,
        /// **Whether this walk was the series' first complete one**, which is
        /// J6's baseline and judges nothing by itself.
        it_is_the_baseline: bool,
        pause_before_the_next: Duration,
    },
    /// **W5: the walk is discarded whole, nothing changes, and the next one
    /// runs at the cadence. Nothing gives up.**
    ItWasDiscarded {
        why: WhyTheWalkIsNotAReading,
        /// **Absent on the one discard the machine said nothing about** — an
        /// entry whose kind the reply did not carry.
        what_the_machine_said: Option<SubsystemFailure>,
        pause_before_the_next: Duration,
    },
}

impl TheWalkSeries {
    /// A series that has taken no walk yet.
    pub fn fresh() -> TheWalkSeries {
        TheWalkSeries::default()
    }

    /// **The most recent complete walk**, which the change detection compares
    /// the next one against.
    ///
    /// **After a turn that completed, that is the walk this turn just read**,
    /// not the one before it — so a caller wanting *what the previous walk
    /// held* cannot take it from here, and a caller wanting *what this walk
    /// showed* can. **J6's baseline is a different question** and
    /// [`WhatTheTurnCameTo::AReading::it_is_the_baseline`] answers it.
    pub fn the_baseline(&self) -> &[AWalkEntry] {
        &self.the_previous_complete_walk
    }

    /// **The entries that were active in the last complete walk**, which is
    /// E1 step 2's own fallback for the *could not be vouched for* mark where
    /// the frame boundary cannot be established.
    ///
    /// **It is not [`TheWalkSeries::the_baseline`]**, which is every file the
    /// folder held at that walk.
    pub fn the_active_entries_of_the_last_complete_walk(&self) -> &[AWalkEntry] {
        &self.the_previous_complete_walks_active_entries
    }

    /// **One turn: the window is asked, a walk runs where it is open, and the
    /// pause before the next turn comes back with the reading.**
    ///
    /// **The window is asked first and every time**, because W3's three
    /// conditions include a standing connection and a desired state that the
    /// user can change between two turns.
    ///
    /// **The pause is measured from the end of the walk** — the duration this
    /// function measured, never a moment it began — which is the load-bearing
    /// half of W2 and the reason a slow walk cannot pile requests onto a
    /// struggling connection.
    pub async fn one_turn<R>(
        &mut self,
        reader: &mut R,
        the_job_output_folder: &[u8],
        window: TheWindow,
        arrangements_walking_this_server: usize,
    ) -> WhatTheTurnCameTo
    where
        R: ReadsOneDirectoryOnTheMachine,
    {
        if !window.walks_run() {
            return WhatTheTurnCameTo::TheWindowIsShutSoNoWalkRan;
        }
        let answer = walk_the_folder_and_everything_under_it(reader, the_job_output_folder).await;
        let pause_before_the_next = the_pause_before_the_next_walk(
            answer.it_took.max(self.the_previous_walk_took),
            arrangements_walking_this_server,
        );
        match answer.walk {
            AWalk::Discarded(why) => WhatTheTurnCameTo::ItWasDiscarded {
                why,
                what_the_machine_said: answer.what_the_machine_said,
                pause_before_the_next,
            },
            AWalk::Complete(entries) => {
                let active: Vec<AWalkEntry> =
                    crate::watch::what_is_active(&self.the_previous_complete_walk, &entries)
                        .into_iter()
                        .cloned()
                        .collect();
                let it_is_the_baseline = !self.a_complete_walk_has_run;
                self.the_previous_complete_walk = entries;
                self.the_previous_complete_walks_active_entries = active.clone();
                self.a_complete_walk_has_run = true;
                self.the_previous_walk_took = answer.it_took;
                WhatTheTurnCameTo::AReading {
                    active,
                    it_is_the_baseline,
                    pause_before_the_next,
                }
            }
        }
    }

    /// **J6: on every (re)connection the first complete walk is the baseline
    /// again.**
    ///
    /// A connection's baseline is the connection's, so a replacement or a
    /// period-opening connection starts the since-baseline set over. **The
    /// previous complete walk is dropped with it**, because change detection
    /// across a gap of unknown length is not the two-complete-walks comparison
    /// W5 declares.
    pub fn a_connection_began(&mut self) {
        *self = TheWalkSeries::fresh();
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;

    /// A machine whose folders are written by hand.
    #[derive(Default)]
    struct AFolderTree {
        folders: HashMap<Vec<u8>, Result<Vec<ADirectoryEntry>, SubsystemFailure>>,
        readings: usize,
    }

    impl AFolderTree {
        fn holding(mut self, path: &[u8], entries: Vec<ADirectoryEntry>) -> AFolderTree {
            self.folders.insert(path.to_vec(), Ok(entries));
            self
        }
        fn refusing(mut self, path: &[u8], failure: SubsystemFailure) -> AFolderTree {
            self.folders.insert(path.to_vec(), Err(failure));
            self
        }
    }

    impl ReadsOneDirectoryOnTheMachine for AFolderTree {
        async fn read_one_directory(
            &mut self,
            path: &[u8],
        ) -> Result<Vec<ADirectoryEntry>, SubsystemFailure> {
            self.readings += 1;
            self.folders
                .get(path)
                .cloned()
                .unwrap_or(Err(SubsystemFailure::TheServerRefused(
                    StatusCode::NoSuchFile,
                )))
        }
    }

    fn file(name: &str, size: u64) -> ADirectoryEntry {
        ADirectoryEntry {
            name: name.as_bytes().to_vec(),
            kind: TheEntrysKind::NotAFolder,
            size,
            modification_time: 10,
        }
    }

    fn folder(name: &str) -> ADirectoryEntry {
        ADirectoryEntry {
            name: name.as_bytes().to_vec(),
            kind: TheEntrysKind::AFolder,
            size: 0,
            modification_time: 10,
        }
    }

    fn of_unknown_kind(name: &str) -> ADirectoryEntry {
        ADirectoryEntry {
            name: name.as_bytes().to_vec(),
            kind: TheEntrysKind::ItWasNotCarried,
            size: 0,
            modification_time: 10,
        }
    }

    /// **W1: an entry whose kind the reply did not carry makes the walk a
    /// reading that cannot be used.**
    ///
    /// **The wrong answer this closes is a silent one.** Read as *not a
    /// folder*, a job's `beauty/`, `mist/` and `depth/` sub-folders come back
    /// as files: the walk never descends, never reads them, and **still
    /// reports a complete walk, because nothing failed** — so the whole-walk
    /// discard cannot catch it. Every file under a sub-folder is then missing
    /// from the ending walk and becomes *never coming*, and a job whose frames
    /// are all safely on the machine reads **ended, 0 of 720**. Each such
    /// folder is also pushed onto the fetchable list and a file transfer is
    /// arranged for a directory.
    ///
    /// **Discarding costs one cadence and fails towards fetching late**, which
    /// is the direction this document claims everywhere else.
    #[test]
    fn an_entry_whose_kind_was_not_carried_discards_the_walk_rather_than_reading_it_as_a_file() {
        let mut machine = AFolderTree::default().holding(
            b"/jobs/abc",
            vec![file("0001.exr", 40), of_unknown_kind("beauty")],
        );

        let answer = run(walk_the_folder_and_everything_under_it(
            &mut machine,
            b"/jobs/abc",
        ));

        assert!(answer.walk.changes_nothing());
        assert_eq!(
            answer.walk,
            AWalk::Discarded(WhyTheWalkIsNotAReading::AnEntrysKindWasNotCarried)
        );
        // **The machine said nothing wrong**, so no status is invented for it.
        assert_eq!(answer.what_the_machine_said, None);
        assert!(answer.walk.entries().is_none());
    }

    /// **A walk of entries that all carry their kind is a reading as before**,
    /// so the discard reaches only the case it is for.
    #[test]
    fn a_walk_whose_entries_all_carry_their_kind_is_still_a_reading() {
        let mut machine = AFolderTree::default()
            .holding(b"/jobs/abc", vec![file("0001.exr", 40), folder("beauty")])
            .holding(b"/jobs/abc/beauty", vec![file("0001.exr", 90)]);

        let answer = run(walk_the_folder_and_everything_under_it(
            &mut machine,
            b"/jobs/abc",
        ));
        let entries = answer.walk.entries().expect("it is a reading");
        assert!(entries.iter().any(|entry| entry.path == b"beauty/0001.exr"));
    }

    fn run<F: std::future::Future>(f: F) -> F::Output {
        // The tests drive one future to completion with no waiting in it, so
        // no runtime is wanted — the same shape the sibling crates' own
        // reading tests use.
        let mut f = Box::pin(f);
        let waker = std::task::Waker::noop();
        let mut context = std::task::Context::from_waker(waker);
        loop {
            if let std::task::Poll::Ready(out) = f.as_mut().poll(&mut context) {
                return out;
            }
        }
    }

    fn an_open_window() -> TheWindow {
        TheWindow {
            the_desired_state_is_on: true,
            a_connection_to_the_server_stands: true,
            the_attempts_window_is_open: true,
        }
    }

    /// **W4: the walk descends, and every entry's path is relative to the job
    /// output folder.** A single directory's entries would miss the
    /// inventory's predicted paths, which is what the redirect makes deep.
    #[test]
    fn the_walk_descends_and_every_path_is_relative_to_the_job_output_folder() {
        let mut machine = AFolderTree::default()
            .holding(b"/jobs/abc", vec![folder("layer"), file("frame.png", 12)])
            .holding(b"/jobs/abc/layer", vec![file("0001.exr", 40)]);

        let answer = run(walk_the_folder_and_everything_under_it(
            &mut machine,
            b"/jobs/abc",
        ));

        let mut paths: Vec<Vec<u8>> = answer
            .walk
            .entries()
            .expect("the walk completed")
            .iter()
            .map(|entry| entry.path.clone())
            .collect();
        paths.sort();
        assert_eq!(
            paths,
            vec![
                b"frame.png".to_vec(),
                b"layer".to_vec(),
                b"layer/0001.exr".to_vec(),
            ]
        );
        assert!(answer.what_the_machine_said.is_none());
    }

    /// **`Job files and outputs` C9's rule: a walk that did not complete is
    /// not a reading and is discarded whole** — not branch by branch. A tree
    /// missing a branch is a folder with fewer files in it, which is the one
    /// reading that would put every frame of a job into *never coming*.
    #[test]
    fn one_refused_folder_anywhere_discards_the_whole_walk_and_not_the_branch() {
        let mut machine = AFolderTree::default()
            .holding(b"/jobs/abc", vec![folder("layer"), file("frame.png", 12)])
            .refusing(
                b"/jobs/abc/layer",
                SubsystemFailure::TheServerRefused(StatusCode::PermissionDenied),
            );

        let answer = run(walk_the_folder_and_everything_under_it(
            &mut machine,
            b"/jobs/abc",
        ));

        assert!(answer.walk.changes_nothing());
        assert_eq!(answer.walk.entries(), None);
        assert_eq!(
            answer.walk,
            AWalk::Discarded(WhyTheWalkIsNotAReading::ARefusal)
        );
    }

    /// **W5 asks for the truth with the separated cause where the route
    /// separates it**, and the three-valued discard carries no status — so the
    /// status rides beside it, as the protocol's own value.
    #[test]
    fn the_separated_cause_rides_beside_the_discard_as_the_protocols_own_status() {
        for (failure, expected) in [
            (
                SubsystemFailure::TheChannelIsGone,
                WhyTheWalkIsNotAReading::TheConnectionWent,
            ),
            (
                SubsystemFailure::TheAnswerCouldNotBeRead,
                WhyTheWalkIsNotAReading::ItWasCut,
            ),
            (
                SubsystemFailure::TheServerRefused(StatusCode::PermissionDenied),
                WhyTheWalkIsNotAReading::ARefusal,
            ),
            (
                SubsystemFailure::TheServerRefused(StatusCode::NoSuchFile),
                WhyTheWalkIsNotAReading::ARefusal,
            ),
        ] {
            let mut machine = AFolderTree::default().refusing(b"/jobs/abc", failure.clone());
            let answer = run(walk_the_folder_and_everything_under_it(
                &mut machine,
                b"/jobs/abc",
            ));
            assert_eq!(answer.walk, AWalk::Discarded(expected));
            assert_eq!(answer.what_the_machine_said, Some(failure));
        }
    }

    /// **W3 is asked at every turn and not once**, because the desired state
    /// and the connection can both change between two walks.
    #[test]
    fn a_shut_window_takes_no_reading_at_all() {
        let mut machine = AFolderTree::default().holding(b"/jobs/abc", vec![file("a.png", 1)]);
        let mut series = TheWalkSeries::fresh();
        let shut = TheWindow {
            the_attempts_window_is_open: false,
            ..an_open_window()
        };

        let turn = run(series.one_turn(&mut machine, b"/jobs/abc", shut, 1));

        assert_eq!(turn, WhatTheTurnCameTo::TheWindowIsShutSoNoWalkRan);
        assert_eq!(machine.readings, 0);
        assert!(series.the_baseline().is_empty());
    }

    /// **J6: the first complete walk of a connection is the baseline and
    /// judges nothing by itself**, and a later walk's active set is the
    /// comparison of two complete walks (W5).
    #[test]
    fn the_first_complete_walk_is_the_baseline_and_the_next_one_carries_the_change() {
        let mut machine = AFolderTree::default().holding(b"/jobs/abc", vec![file("a.png", 5)]);
        let mut series = TheWalkSeries::fresh();

        let first = run(series.one_turn(&mut machine, b"/jobs/abc", an_open_window(), 1));
        match first {
            WhatTheTurnCameTo::AReading {
                it_is_the_baseline,
                ref active,
                ..
            } => {
                assert!(it_is_the_baseline);
                // Everything is new against an empty baseline, which is what
                // makes the first walk a reading and not a judgement.
                assert_eq!(active.len(), 1);
            }
            other => panic!("expected a reading, got {other:?}"),
        }

        machine =
            AFolderTree::default().holding(b"/jobs/abc", vec![file("a.png", 5), file("b.png", 7)]);
        let second = run(series.one_turn(&mut machine, b"/jobs/abc", an_open_window(), 1));
        match second {
            WhatTheTurnCameTo::AReading {
                it_is_the_baseline,
                active,
                ..
            } => {
                assert!(!it_is_the_baseline);
                assert_eq!(
                    active.iter().map(|e| e.path.clone()).collect::<Vec<_>>(),
                    vec![b"b.png".to_vec()]
                );
            }
            other => panic!("expected a reading, got {other:?}"),
        }
    }

    /// **W5: a discarded walk changes nothing** — the previous complete walk
    /// stays the baseline, and the next walk's change detection is still the
    /// comparison of two *complete* walks.
    #[test]
    fn a_discarded_walk_leaves_the_baseline_exactly_where_it_was() {
        let mut good = AFolderTree::default().holding(b"/jobs/abc", vec![file("a.png", 5)]);
        let mut series = TheWalkSeries::fresh();
        run(series.one_turn(&mut good, b"/jobs/abc", an_open_window(), 1));
        let baseline = series.the_baseline().to_vec();

        let mut bad =
            AFolderTree::default().refusing(b"/jobs/abc", SubsystemFailure::TheChannelIsGone);
        let turn = run(series.one_turn(&mut bad, b"/jobs/abc", an_open_window(), 1));

        assert!(matches!(turn, WhatTheTurnCameTo::ItWasDiscarded { .. }));
        assert_eq!(series.the_baseline(), baseline.as_slice());
    }

    /// **W2, from the end and never the start**, and the floor is the longer
    /// of the ten seconds and the walk's own duration — so the pause a turn
    /// hands back is never below the constant.
    #[test]
    fn the_pause_a_turn_hands_back_is_never_below_the_declared_constant() {
        let mut machine = AFolderTree::default().holding(b"/jobs/abc", vec![file("a.png", 5)]);
        let mut series = TheWalkSeries::fresh();
        let turn = run(series.one_turn(&mut machine, b"/jobs/abc", an_open_window(), 1));
        match turn {
            WhatTheTurnCameTo::AReading {
                pause_before_the_next,
                ..
            } => assert!(pause_before_the_next >= crate::watch::THE_PAUSE),
            other => panic!("expected a reading, got {other:?}"),
        }
    }

    /// **A connection's baseline is the connection's**, so a fresh connection
    /// starts the series over rather than comparing across a gap of unknown
    /// length.
    #[test]
    fn a_fresh_connection_starts_the_series_over() {
        let mut machine = AFolderTree::default().holding(b"/jobs/abc", vec![file("a.png", 5)]);
        let mut series = TheWalkSeries::fresh();
        run(series.one_turn(&mut machine, b"/jobs/abc", an_open_window(), 1));
        assert!(!series.the_baseline().is_empty());

        series.a_connection_began();

        assert!(series.the_baseline().is_empty());
        let turn = run(series.one_turn(&mut machine, b"/jobs/abc", an_open_window(), 1));
        assert!(matches!(
            turn,
            WhatTheTurnCameTo::AReading {
                it_is_the_baseline: true,
                ..
            }
        ));
    }

    /// **W6, kept by the type**: the only operation this module can make is
    /// the directory reading, so nothing can be planted, renamed, opened or
    /// changed to make watching easier.
    #[test]
    fn the_reading_trait_carries_one_operation_and_it_writes_nothing() {
        let source = include_str!("walking.rs");
        // **The needles are joined here rather than written out**, because a
        // source-reading test that spells what it forbids finds itself.
        for writing in [
            ["make", "_directory"].concat(),
            ["posix", "_rename"].concat(),
            ["send_a", "_write"].concat(),
            ["remove_", "directory"].concat(),
        ] {
            assert!(
                !source.contains(&writing),
                "the walk names a writing operation: {writing}"
            );
        }
    }
}
