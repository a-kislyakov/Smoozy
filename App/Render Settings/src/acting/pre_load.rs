//! **Pre-load Settings, driven** — `Render Settings` §1.2 Q4, Q5, Q17; §1.3
//! Q8, Q9; §7.1 B5a.
//!
//! **Q4: Pre-load Settings is the user's own act, and the read it starts is
//! one file-inspection operation driven from the desktop through the
//! supervisor's command path.** It is admitted at the machine's one admission
//! point under §10's declarations, exactly as `Dependency Upload & Remap` G1
//! admits its check reading. **The render-start reading is not this
//! operation** — it runs inside the admitted render operation as a step of the
//! render worker (`Rendering` P1 step 4) and contends with nothing, so nothing
//! in this file is reached for it.
//!
//! # The invocation is a seam, and that is a clause rather than a shortcut
//!
//! **`Job files and outputs` R1 owns the invocation** — one invocation of the
//! Blender the manifest names, headless, one JSON document between sentinels —
//! and its §4 is recorded in that crate's own `arriving_later` as arriving at
//! this step. **So what this file drives is [`DrivesOneFileInspection`]**, and
//! admission at the machine's one admission point is `Rented-server runtime`
//! A4's on the far side of it. This is the same shape `Servers` uses for what
//! opens a connection.
//!
//! **Nothing here composes the reader or reads what it printed.** The frame
//! that comes back is judged by [`crate::endings::judge`] over Smoozy's own
//! sentinels, the exit status and the error map, and never over anything
//! Blender printed — `Rented-server runtime` §11.4's rule.
//!
//! # What is held here, and what each holds against
//!
//! - **Q5: one read per (server, file) at a time, and a second Pre-load
//!   asked for while one runs joins the running one** rather than starting a
//!   second beside it. [`ReadsInFlight`] is that, and the join gives the
//!   second asker the running read's own outcome.
//! - **Q17: the read carries the version pick and the four-value file identity
//!   it was started under**, because the read is in flight for 3.2 to 3.8
//!   seconds against a page that may move under it. The judgement on landing
//!   is `crate::file_half::FileHalf::a_reading_landed`'s and is not repeated
//!   here.
//! - **B5a: no read runs while no version resolves.** Pre-load is a consumer
//!   of B2a's resolution exactly as `Dependency Upload & Remap` G2's check is,
//!   and a build chosen by Smoozy for the reading alone would break `Job files
//!   and outputs` R2 outright — the build that read the file would not be the
//!   build that renders it.
//! - **Q5's other half: while the read runs the page is left exactly as it
//!   was.** Nothing in this file writes anything anywhere until a reading has
//!   been judged whole, which is why there is no provisional store and no
//!   blanking act to find.
//! - **Q9: a failed read changes nothing.** Every ending but a reading returns
//!   without touching the file's half, and the override store is unreachable
//!   from here by construction (J4).
//!
//! # W2's termination contract, as this file keeps it
//!
//! **Terminable at any moment, and nothing durable is half-written at any
//! point** (W1, W2 item 2): the reading is consumed only whole, so ending it
//! anywhere loses a reading and corrupts nothing. **A read dropped mid-flight
//! leaves no entry behind** — [`ReadsInFlight`]'s entry is cleared whether the
//! read finished or was dropped, and a joiner on a read that went away is told
//! Smoozy stopped it rather than being left waiting or told something about
//! the file.
//!
//! **Markers used in this file that belong to another document, each with the
//! document it belongs to.** `Job files and outputs` R1, R2. `Rented-server
//! runtime` A4. `Dependency Upload & Remap` G1, G2. `Rendering` P1.
//! `Desktop app and delivery` L14.

use std::collections::HashMap;
use std::future::Future;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Mutex;
use std::time::SystemTime;

use smoozy_server_protocol::transfers::SourceReading;
use smoozy_state::identities::Identity;
use tokio::sync::broadcast;

use crate::diary::ABlendRead;
use crate::endings::{judge, Ending, WhatTheInvocationLeft};
use crate::file_half::{BlenderIdentity, FileHalf, FileKey, Landed, StartedUnder};
use crate::reading::SettingsHalf;
use crate::version::Resolution;

/// **The one file-inspection operation, named for the far side to perform.**
///
/// **`Job files and outputs` R1 is the invocation and this is only the ask**:
/// the Blender at `with.path`, headless, against the `.blend` at
/// `the_blends_path` on `server`, admitted at the machine's one admission
/// point under §10 W1's declarations (`Rented-server runtime` A4).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ABlendToRead<'a> {
    pub server: &'a Identity,
    /// The `.blend`'s recorded path on that server.
    pub the_blends_path: &'a str,
    /// **Q11's pair**: the build's path and its answered version. The reading
    /// is taken by the version that will render (`Job files and outputs` R2).
    pub with: &'a BlenderIdentity,
}

/// **The seam: what drives one file-inspection operation.**
///
/// It returns Smoozy's own frame and nothing else — the channel's outcome, the
/// two sentinels, the error map's emptiness, the exit status, and whether
/// Smoozy itself ended the invocation. **A caller of this trait never returns a
/// diagnosis**, because telling the three endings apart is
/// [`crate::endings::judge`]'s and is held at the desk.
pub trait DrivesOneFileInspection {
    fn read_one_blend(
        &mut self,
        of: ABlendToRead<'_>,
    ) -> impl Future<Output = WhatTheInvocationLeft> + Send;
}

/// **What the page holds at the moment a reading lands** — Q17's two tests,
/// asked of the page rather than remembered here, because the read is in
/// flight against a page that may have moved.
pub trait ThePageTheReadingArrivesAt {
    /// **The page's current version pick** for that (server, file), which is
    /// B2a's resolution as it stands now. A reading whose pick is not this one
    /// is discarded.
    fn the_version_pick_now(&self, key: &FileKey) -> Option<BlenderIdentity>;

    /// **The four values now standing** for that (server, file).
    fn the_four_values_now_standing(&self, key: &FileKey) -> TheFourValuesNowStanding;
}

/// **Q17's second test's material.**
///
/// **The absent arm is not an oversight and is the safe direction.** Q14 item
/// 1 drops the file's half at a publication *with no comparison*, so after one
/// there are no four values standing for the path at all; and Q14 item 2's
/// read can be one the channel could not answer. In both, the page cannot say
/// whether the file the reading describes is the file that is there now —
/// **so the reading is discarded and nothing is written**, which is what
/// Q17's own act does for every other way the page moves under a read.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TheFourValuesNowStanding {
    These(SourceReading),
    /// Nothing stands for that (server, file).
    NoneStands,
}

/// **What a Pre-load did.**
#[derive(Debug, Clone, PartialEq)]
pub enum HowThePreLoadWent {
    /// **Q7's reading, with Q6 and Q17's outcome for it.**
    AReading {
        landed: Landed,
        /// **The diary's act line, owed exactly where the reading was
        /// stored.** A read that gave nothing writes no act line, because
        /// nothing was read, and neither does a reading Q17 discarded on
        /// landing. Writing it is the host's — lines go through the diary
        /// service `Desktop app and delivery` §5.3 L14 names — and composing
        /// it is `crate::diary`'s.
        the_diary_act: Option<ABlendRead>,
    },
    /// **Q8's endings, Q9's rule: nothing is stored, nothing is shown, and
    /// the previous recorded reading goes on standing.**
    NoReading(Ending),
    /// **B5a: no version resolves, so the read is not offered and none ran.**
    /// The withholding lasts exactly until the user's own pick.
    TheReadIsNotOffered,
}

impl HowThePreLoadWent {
    /// **The outcome as a joiner gets it: the running read's, with no act
    /// line.** §12's declaration is one line per Pre-load and names Q5's join
    /// in the same breath, so the one read that ran owes the one line and the
    /// joiner owes none.
    fn with_no_diary_act_of_its_own(self) -> HowThePreLoadWent {
        match self {
            HowThePreLoadWent::AReading { landed, .. } => HowThePreLoadWent::AReading {
                landed,
                the_diary_act: None,
            },
            went => went,
        }
    }

    /// Whether the file's half moved. **Q9: a failed read changes nothing.**
    pub fn the_files_half_was_replaced(&self) -> bool {
        matches!(
            self,
            HowThePreLoadWent::AReading {
                landed: Landed::Stored,
                ..
            }
        )
    }
}

/// **Where the reading is stored, reached as a seam so that two callers of
/// [`pre_load`] can run at once.** Q5's join is only meaningful where a second
/// asker can arrive while the first read is in flight, and one exclusive
/// borrow of the file's half for the whole of a read would make that
/// impossible.
///
/// **The judgement is not here.** Every implementation is expected to call
/// `crate::file_half::FileHalf::a_reading_landed`, which is Q6, Q17 and Q18 as
/// one act; the implementation below over a locked file half does exactly
/// that and nothing else.
pub trait HoldsTheFilesHalf {
    #[allow(clippy::too_many_arguments)]
    fn a_reading_landed(
        &self,
        key: FileKey,
        started_under: &StartedUnder,
        settings: SettingsHalf,
        the_pages_current_pick: Option<&BlenderIdentity>,
        the_four_values_now: &SourceReading,
        stored_at: SystemTime,
    ) -> Landed;
}

impl HoldsTheFilesHalf for Mutex<FileHalf> {
    fn a_reading_landed(
        &self,
        key: FileKey,
        started_under: &StartedUnder,
        settings: SettingsHalf,
        the_pages_current_pick: Option<&BlenderIdentity>,
        the_four_values_now: &SourceReading,
        stored_at: SystemTime,
    ) -> Landed {
        self.lock()
            .expect("the file's half is held by no panicking thread")
            .a_reading_landed(
                key,
                started_under,
                settings,
                the_pages_current_pick,
                the_four_values_now,
                stored_at,
            )
    }
}

/// **Q5: one read per (server, file) runs at a time.**
///
/// A second Pre-load asked for while one runs **joins the running one rather
/// than starting a second beside it**, and gets that read's own outcome.
#[derive(Debug, Default)]
pub struct ReadsInFlight {
    running: Mutex<HashMap<FileKey, ARunningRead>>,
    next_token: AtomicU64,
}

#[derive(Debug)]
struct ARunningRead {
    /// **Which read this entry is.** A read that was dropped clears its own
    /// entry, and the token is what stops a late clearance from taking a
    /// *later* read's entry with it — which would let a third asker start a
    /// second read beside one already running, the exact thing Q5 forbids.
    token: u64,
    telling: broadcast::Sender<HowThePreLoadWent>,
}

enum WhoRunsIt<'a> {
    ThisCaller(TheReadThisCallerRuns<'a>),
    JoinTheRunningOne(broadcast::Receiver<HowThePreLoadWent>),
}

/// The running read's own claim on its (server, file), cleared when the read
/// ends **and when it is dropped**, which is W2's terminable-at-any-moment.
struct TheReadThisCallerRuns<'a> {
    in_flight: &'a ReadsInFlight,
    key: FileKey,
    token: u64,
    telling: broadcast::Sender<HowThePreLoadWent>,
}

impl Drop for TheReadThisCallerRuns<'_> {
    /// **W2 item 2: terminable at any moment.** A read whose future is dropped
    /// clears its own claim here, so the next Pre-load of that file starts a
    /// fresh read rather than joining one nobody is running. **The token is
    /// what keeps a late clearance from taking a later read's claim with it.**
    fn drop(&mut self) {
        self.in_flight.it_ended(&self.key, self.token);
    }
}

impl ReadsInFlight {
    pub fn new() -> ReadsInFlight {
        ReadsInFlight::default()
    }

    /// How many reads are in flight. **Q5's rule, readable.**
    pub fn how_many_are_running(&self) -> usize {
        self.held().len()
    }

    fn held(&self) -> std::sync::MutexGuard<'_, HashMap<FileKey, ARunningRead>> {
        self.running
            .lock()
            .expect("the in-flight table is held by no panicking thread")
    }

    fn take_the_read_or_join(&self, key: &FileKey) -> WhoRunsIt<'_> {
        let mut running = self.held();
        if let Some(already) = running.get(key) {
            return WhoRunsIt::JoinTheRunningOne(already.telling.subscribe());
        }
        let token = self.next_token.fetch_add(1, Ordering::Relaxed);
        let (telling, _) = broadcast::channel(1);
        running.insert(
            key.clone(),
            ARunningRead {
                token,
                telling: telling.clone(),
            },
        );
        WhoRunsIt::ThisCaller(TheReadThisCallerRuns {
            in_flight: self,
            key: key.clone(),
            token,
            telling,
        })
    }

    fn it_ended(&self, key: &FileKey, token: u64) {
        let mut running = self.held();
        if running.get(key).map(|held| held.token) == Some(token) {
            running.remove(key);
        }
    }
}

/// **Q4, Q5, Q8, Q9 and Q17, performed: one Pre-load.**
///
/// The order matters and each step is a clause:
///
/// 1. **B5a**: no version resolves, no read runs.
/// 2. **Q5**: this caller runs the read, or joins the one already running.
/// 3. **Q17**: the read carries the version pick and the four values it was
///    started under.
/// 4. **Q7, Q8**: the frame that comes back is judged whole-or-nothing over
///    Smoozy's own sentinels.
/// 5. **Q6, Q17, Q18**: a reading is stored only where both tests still stand,
///    and the storing stamps the desktop's moment.
///
/// **The act line is composed and handed back rather than written here**, for
/// the reason `crate::diary` gives: the diary service is `Desktop app and
/// delivery`'s and this crate reaches it through its host.
///
/// **Each argument is a distinct thing the clauses name** — the in-flight
/// table (Q5), the seam (Q4), the page as it stands at the landing (Q17), the
/// file's half (Q6), the key, the resolution (B5a), the four values the read
/// starts under (Q17) and the storing moment (Q18) — so bundling them would
/// hide which clause each answers.
#[allow(clippy::too_many_arguments)]
pub async fn pre_load(
    in_flight: &ReadsInFlight,
    reads: &mut impl DrivesOneFileInspection,
    page: &impl ThePageTheReadingArrivesAt,
    files_half: &impl HoldsTheFilesHalf,
    key: FileKey,
    resolution: &Resolution,
    the_four_values_the_read_starts_under: SourceReading,
    stored_at: SystemTime,
) -> HowThePreLoadWent {
    // B5a. R2 binds the reading to the version that will render, and there is
    // no value for it here; a build chosen for the reading alone would mean
    // the build that read the file is not the build that renders it.
    let Some(version_pick) = resolution.the_payloads_pick() else {
        return HowThePreLoadWent::TheReadIsNotOffered;
    };

    let running = match in_flight.take_the_read_or_join(&key) {
        WhoRunsIt::JoinTheRunningOne(mut joined) => {
            // A read that went away without telling anybody is one Smoozy
            // ended — the read's own future was dropped, which is W2's
            // termination — and it is never diagnosed as one of Q8's three.
            //
            // **§12's diary declaration is one line per Pre-load, and it names
            // Q5's join in the same breath, precisely to bound the count.** A
            // joiner did not run a read, so it is owed no act line: handing it
            // the running read's own would write two `blend read` lines for
            // one read, at the same path, the same build and the same moment.
            // The joiner still gets the running read's outcome, which is all
            // Q5 promises.
            return joined
                .recv()
                .await
                .unwrap_or(HowThePreLoadWent::NoReading(Ending::SmoozyStoppedIt))
                .with_no_diary_act_of_its_own();
        }
        WhoRunsIt::ThisCaller(running) => running,
    };

    let started_under = StartedUnder {
        version_pick,
        file_identity: the_four_values_the_read_starts_under,
    };

    let left = reads
        .read_one_blend(ABlendToRead {
            server: &key.server,
            the_blends_path: &key.path,
            with: &started_under.version_pick,
        })
        .await;

    let went = match judge(left) {
        Ending::AReading(settings) => {
            let taken_by = started_under.version_pick.clone();
            let landed = match page.the_four_values_now_standing(&key) {
                TheFourValuesNowStanding::NoneStands => Landed::DiscardedTheFileMoved,
                TheFourValuesNowStanding::These(now) => files_half.a_reading_landed(
                    key.clone(),
                    &started_under,
                    settings,
                    page.the_version_pick_now(&key).as_ref(),
                    &now,
                    stored_at,
                ),
            };
            HowThePreLoadWent::AReading {
                the_diary_act: match landed {
                    Landed::Stored => Some(ABlendRead {
                        the_blends_recorded_path: key.path.clone(),
                        taken_by,
                    }),
                    _ => None,
                },
                landed,
            }
        }
        no_reading => HowThePreLoadWent::NoReading(no_reading),
    };

    // The claim is given up **before** the outcome goes out, so a caller
    // arriving after the telling starts a fresh read rather than subscribing
    // to a channel that has already spoken.
    in_flight.it_ended(&running.key, running.token);
    let _ = running.telling.send(went.clone());
    went
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::endings::{ChannelOutcome, ExitStatus};
    use crate::file_half::testing::{a_server, a_settings_half, four_values};
    use crate::version::{HowItMatched, PickedBuild};
    use smoozy_server_setup::blender::Version;
    use std::sync::atomic::AtomicUsize;

    fn a_build() -> BlenderIdentity {
        BlenderIdentity {
            path: "/opt/blender-5.2/blender".to_string(),
            version: Version {
                major: 5,
                minor: 2,
                patch: 0,
            },
        }
    }

    fn a_resolution() -> Resolution {
        Resolution::Automatic {
            build: PickedBuild {
                path: a_build().path,
                answered: a_build().version,
            },
            how: HowItMatched::TheExactLine,
        }
    }

    /// **One file on one server.** The identity is issued fresh, so it is made
    /// once per test and shared: two calls to `a_server()` are two different
    /// machines, and a (server, file) key made twice would be two keys.
    fn a_file_on(server: &Identity) -> FileKey {
        FileKey {
            server: server.clone(),
            path: "/ephemeral/smoozy/assets/kitchen.blend".to_string(),
        }
    }

    /// A driver that answers one frame and counts how often it was asked.
    struct ADriver {
        answers: WhatTheInvocationLeft,
        asked: AtomicUsize,
        yields_first: bool,
    }

    impl ADriver {
        fn answering(answers: WhatTheInvocationLeft) -> ADriver {
            ADriver {
                answers,
                asked: AtomicUsize::new(0),
                yields_first: false,
            }
        }

        fn asked(&self) -> usize {
            self.asked.load(Ordering::Relaxed)
        }
    }

    impl DrivesOneFileInspection for ADriver {
        async fn read_one_blend(&mut self, _of: ABlendToRead<'_>) -> WhatTheInvocationLeft {
            self.asked.fetch_add(1, Ordering::Relaxed);
            if self.yields_first {
                tokio::task::yield_now().await;
            }
            self.answers.clone()
        }
    }

    fn a_reading_came_back() -> WhatTheInvocationLeft {
        WhatTheInvocationLeft {
            channel: ChannelOutcome::TheInvocationAnswered,
            smoozy_ended_it: false,
            exit: ExitStatus::Succeeded,
            start_sentinel_seen: true,
            end_sentinel_seen: true,
            error_map_is_empty: Some(true),
            document: Some(a_settings_half()),
        }
    }

    struct APage {
        pick: Option<BlenderIdentity>,
        four_values: TheFourValuesNowStanding,
    }

    impl APage {
        fn unmoved() -> APage {
            APage {
                pick: Some(a_build()),
                four_values: TheFourValuesNowStanding::These(four_values(4096)),
            }
        }
    }

    impl ThePageTheReadingArrivesAt for APage {
        fn the_version_pick_now(&self, _key: &FileKey) -> Option<BlenderIdentity> {
            self.pick.clone()
        }

        fn the_four_values_now_standing(&self, _key: &FileKey) -> TheFourValuesNowStanding {
            self.four_values.clone()
        }
    }

    fn block_on<F: Future>(future: F) -> F::Output {
        tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .expect("a runtime")
            .block_on(future)
    }

    fn a_moment() -> SystemTime {
        SystemTime::UNIX_EPOCH + std::time::Duration::from_secs(1_756_800_000)
    }

    #[test]
    fn a_reading_that_lands_on_the_page_that_asked_for_it_is_stored_and_owes_an_act_line() {
        let key = a_file_on(&a_server());
        let in_flight = ReadsInFlight::new();
        let mut driver = ADriver::answering(a_reading_came_back());
        let half = Mutex::new(FileHalf::new());
        let went = block_on(pre_load(
            &in_flight,
            &mut driver,
            &APage::unmoved(),
            &half,
            key.clone(),
            &a_resolution(),
            four_values(4096),
            a_moment(),
        ));
        assert!(went.the_files_half_was_replaced());
        let HowThePreLoadWent::AReading { the_diary_act, .. } = went else {
            panic!("a reading");
        };
        assert_eq!(
            the_diary_act,
            Some(ABlendRead {
                the_blends_recorded_path: key.path.clone(),
                taken_by: a_build(),
            })
        );
        assert!(half.lock().expect("the half").standing(&key).is_some());
        // Q5: the claim is given up when the read ends.
        assert_eq!(in_flight.how_many_are_running(), 0);
    }

    #[test]
    fn no_read_runs_while_no_version_resolves_and_nothing_is_asked_of_the_machine() {
        // B5a. The fault this stops: a build chosen for the reading alone,
        // which would mean the build that read the file is not the build that
        // renders it (`Job files and outputs` R2).
        let key = a_file_on(&a_server());
        let in_flight = ReadsInFlight::new();
        let mut driver = ADriver::answering(a_reading_came_back());
        let half = Mutex::new(FileHalf::new());
        let went = block_on(pre_load(
            &in_flight,
            &mut driver,
            &APage::unmoved(),
            &half,
            key.clone(),
            &Resolution::NoVersionResolves(crate::version::WhyNone::TheListIsEmpty),
            four_values(4096),
            a_moment(),
        ));
        assert_eq!(went, HowThePreLoadWent::TheReadIsNotOffered);
        assert_eq!(driver.asked(), 0);
        assert!(half.lock().expect("the half").standing(&key).is_none());
    }

    #[test]
    fn a_reading_arriving_at_a_page_whose_version_moved_is_discarded_and_writes_no_act_line() {
        // Q17. Without the discard, the page would hold a reading taken by a
        // Blender other than the one the job will render with.
        let key = a_file_on(&a_server());
        let in_flight = ReadsInFlight::new();
        let mut driver = ADriver::answering(a_reading_came_back());
        let half = Mutex::new(FileHalf::new());
        let moved = APage {
            pick: Some(BlenderIdentity {
                path: "/opt/blender-4.5/blender".to_string(),
                version: Version {
                    major: 4,
                    minor: 5,
                    patch: 12,
                },
            }),
            four_values: TheFourValuesNowStanding::These(four_values(4096)),
        };
        let went = block_on(pre_load(
            &in_flight,
            &mut driver,
            &moved,
            &half,
            key.clone(),
            &a_resolution(),
            four_values(4096),
            a_moment(),
        ));
        assert_eq!(
            went,
            HowThePreLoadWent::AReading {
                landed: Landed::DiscardedTheVersionMoved,
                the_diary_act: None,
            }
        );
        assert!(half.lock().expect("the half").standing(&key).is_none());
    }

    #[test]
    fn a_reading_arriving_at_a_page_whose_file_moved_is_discarded() {
        let key = a_file_on(&a_server());
        let in_flight = ReadsInFlight::new();
        let mut driver = ADriver::answering(a_reading_came_back());
        let half = Mutex::new(FileHalf::new());
        let moved = APage {
            pick: Some(a_build()),
            four_values: TheFourValuesNowStanding::These(four_values(8192)),
        };
        let went = block_on(pre_load(
            &in_flight,
            &mut driver,
            &moved,
            &half,
            key.clone(),
            &a_resolution(),
            four_values(4096),
            a_moment(),
        ));
        assert_eq!(
            went,
            HowThePreLoadWent::AReading {
                landed: Landed::DiscardedTheFileMoved,
                the_diary_act: None,
            }
        );
    }

    #[test]
    fn a_reading_landing_where_no_four_values_stand_is_discarded_rather_than_stored() {
        // Q14 item 1's publication drops the file's half with no comparison, so
        // there is nothing to test the arriving reading against. Storing it
        // would put a reading of the replaced file on the page.
        let key = a_file_on(&a_server());
        let in_flight = ReadsInFlight::new();
        let mut driver = ADriver::answering(a_reading_came_back());
        let half = Mutex::new(FileHalf::new());
        let published_over = APage {
            pick: Some(a_build()),
            four_values: TheFourValuesNowStanding::NoneStands,
        };
        let went = block_on(pre_load(
            &in_flight,
            &mut driver,
            &published_over,
            &half,
            key.clone(),
            &a_resolution(),
            four_values(4096),
            a_moment(),
        ));
        assert_eq!(
            went,
            HowThePreLoadWent::AReading {
                landed: Landed::DiscardedTheFileMoved,
                the_diary_act: None,
            }
        );
        assert!(half.lock().expect("the half").standing(&key).is_none());
    }

    #[test]
    fn every_ending_that_is_not_a_reading_changes_nothing_and_owes_no_act_line() {
        // Q9, and the read Smoozy ended itself, which is none of Q8's three.
        let key = a_file_on(&a_server());
        let frames = [
            (
                WhatTheInvocationLeft {
                    channel: ChannelOutcome::TheMachineCouldNotBeReached,
                    exit: ExitStatus::Failed,
                    start_sentinel_seen: false,
                    end_sentinel_seen: false,
                    error_map_is_empty: None,
                    document: None,
                    ..a_reading_came_back()
                },
                Ending::TheMachineCouldNotBeReached,
            ),
            (
                WhatTheInvocationLeft {
                    exit: ExitStatus::Failed,
                    start_sentinel_seen: false,
                    end_sentinel_seen: false,
                    error_map_is_empty: None,
                    document: None,
                    ..a_reading_came_back()
                },
                Ending::TheFileCouldNotBeOpenedByThisVersion,
            ),
            (
                WhatTheInvocationLeft {
                    end_sentinel_seen: false,
                    document: None,
                    ..a_reading_came_back()
                },
                Ending::TheReadRanAndSomethingInItFailed,
            ),
            (
                WhatTheInvocationLeft {
                    smoozy_ended_it: true,
                    exit: ExitStatus::Failed,
                    start_sentinel_seen: false,
                    end_sentinel_seen: false,
                    error_map_is_empty: None,
                    document: None,
                    ..a_reading_came_back()
                },
                Ending::SmoozyStoppedIt,
            ),
        ];
        let resolution = a_resolution();
        for (frame, ending) in frames {
            let in_flight = ReadsInFlight::new();
            let mut driver = ADriver::answering(frame);
            let half = Mutex::new(FileHalf::new());
            let went = block_on(pre_load(
                &in_flight,
                &mut driver,
                &APage::unmoved(),
                &half,
                key.clone(),
                &resolution,
                four_values(4096),
                a_moment(),
            ));
            assert_eq!(went, HowThePreLoadWent::NoReading(ending));
            assert!(!went.the_files_half_was_replaced());
            assert!(half.lock().expect("the half").standing(&key).is_none());
            assert_eq!(in_flight.how_many_are_running(), 0);
        }
    }

    #[test]
    fn a_second_pre_load_asked_for_while_one_runs_joins_it_rather_than_starting_a_second() {
        // Q5. The fault this stops: two Blenders reading one file beside each
        // other on a machine billed by the hour, for one act of the user's.
        let key = a_file_on(&a_server());
        let in_flight = ReadsInFlight::new();
        let mut first = ADriver {
            yields_first: true,
            ..ADriver::answering(a_reading_came_back())
        };
        let mut second = ADriver::answering(a_reading_came_back());
        let half = Mutex::new(FileHalf::new());
        let page = APage::unmoved();
        let resolution = a_resolution();

        let (one, two) = block_on(async {
            tokio::join!(
                pre_load(
                    &in_flight,
                    &mut first,
                    &page,
                    &half,
                    key.clone(),
                    &resolution,
                    four_values(4096),
                    a_moment(),
                ),
                pre_load(
                    &in_flight,
                    &mut second,
                    &page,
                    &half,
                    key.clone(),
                    &resolution,
                    four_values(4096),
                    a_moment(),
                ),
            )
        });

        assert_eq!(second.asked(), 0, "the second Pre-load started a read");
        assert_eq!(first.asked(), 1);
        // Both askers get the running read's own outcome.
        assert!(one.the_files_half_was_replaced());
        assert!(two.the_files_half_was_replaced());
        assert_eq!(in_flight.how_many_are_running(), 0);

        // **§12: one line per Pre-load, and the declaration names Q5's join in
        // the same breath.** One read ran, so one act line is owed — the
        // runner's. A joiner handed the running read's act line would write a
        // second `blend read` for one read, at the same path, the same build
        // and the same moment.
        let the_act = |went: &HowThePreLoadWent| match went {
            HowThePreLoadWent::AReading { the_diary_act, .. } => the_diary_act.clone(),
            _ => None,
        };
        assert!(the_act(&one).is_some(), "the read that ran owes its line");
        assert_eq!(the_act(&two), None, "the joiner owes no line of its own");
    }

    #[test]
    fn a_read_on_another_file_runs_beside_it_rather_than_joining() {
        // Q5's rule is one read per (server, file), and §10 W1 declares no
        // machine-level exclusion of the kind's own kind — several readings of
        // different files may run at once.
        let key = a_file_on(&a_server());
        let in_flight = ReadsInFlight::new();
        let another = FileKey {
            server: key.server.clone(),
            path: "/ephemeral/smoozy/assets/other.blend".to_string(),
        };
        let mut first = ADriver {
            yields_first: true,
            ..ADriver::answering(a_reading_came_back())
        };
        let mut second = ADriver {
            yields_first: true,
            ..ADriver::answering(a_reading_came_back())
        };
        let half = Mutex::new(FileHalf::new());
        let page = APage::unmoved();
        let resolution = a_resolution();
        block_on(async {
            tokio::join!(
                pre_load(
                    &in_flight,
                    &mut first,
                    &page,
                    &half,
                    key.clone(),
                    &resolution,
                    four_values(4096),
                    a_moment(),
                ),
                pre_load(
                    &in_flight,
                    &mut second,
                    &page,
                    &half,
                    another,
                    &resolution,
                    four_values(4096),
                    a_moment(),
                ),
            )
        });
        assert_eq!(first.asked(), 1);
        assert_eq!(second.asked(), 1);
    }

    #[test]
    fn a_read_that_was_dropped_mid_flight_leaves_no_claim_behind() {
        // W2: terminable at any moment. Without the clearance the next Pre-load
        // of that file would join a read that is never coming back, and the
        // page would wait for ever on a read nobody is running.
        let key = a_file_on(&a_server());
        let in_flight = ReadsInFlight::new();
        let mut driver = ADriver {
            yields_first: true,
            ..ADriver::answering(a_reading_came_back())
        };
        let half = Mutex::new(FileHalf::new());
        let page = APage::unmoved();
        let resolution = a_resolution();
        block_on(async {
            let mut running = Box::pin(pre_load(
                &in_flight,
                &mut driver,
                &page,
                &half,
                key.clone(),
                &resolution,
                four_values(4096),
                a_moment(),
            ));
            // Poll it once so the read is genuinely in flight, then drop it.
            let polled = futures_lite_poll_once(&mut running).await;
            assert!(polled.is_none(), "the read should still be in flight");
            assert_eq!(in_flight.how_many_are_running(), 1);
            drop(running);
        });
        assert_eq!(in_flight.how_many_are_running(), 0);
        assert!(half.lock().expect("the half").standing(&key).is_none());
    }

    #[test]
    fn a_joiner_whose_read_went_away_is_told_it_was_stopped_rather_than_left_waiting() {
        // **W2 item 2 again, from the joiner's side.** A read is terminable at
        // any moment, and a second Pre-load waiting on one has no deadline of
        // its own — it is waiting on the same read. So the join must *end* when
        // the read goes away rather than wait for ever, and it must not be
        // diagnosed as one of Q8's three: nothing was learned about the file.
        let key = a_file_on(&a_server());
        let in_flight = ReadsInFlight::new();
        let mut first = ADriver {
            yields_first: true,
            ..ADriver::answering(a_reading_came_back())
        };
        let mut second = ADriver::answering(a_reading_came_back());
        let half = Mutex::new(FileHalf::new());
        let page = APage::unmoved();
        let resolution = a_resolution();
        let went = block_on(async {
            let mut running = Box::pin(pre_load(
                &in_flight,
                &mut first,
                &page,
                &half,
                key.clone(),
                &resolution,
                four_values(4096),
                a_moment(),
            ));
            assert!(futures_lite_poll_once(&mut running).await.is_none());
            let mut joining = Box::pin(pre_load(
                &in_flight,
                &mut second,
                &page,
                &half,
                key.clone(),
                &resolution,
                four_values(4096),
                a_moment(),
            ));
            assert!(futures_lite_poll_once(&mut joining).await.is_none());
            drop(running);
            joining.await
        });
        assert_eq!(went, HowThePreLoadWent::NoReading(Ending::SmoozyStoppedIt));
        assert_eq!(second.asked(), 0);
        assert_eq!(in_flight.how_many_are_running(), 0);
    }

    /// One poll of a future, without pulling in a crate for it.
    async fn futures_lite_poll_once<F: Future + Unpin>(future: &mut F) -> Option<F::Output> {
        std::future::poll_fn(|context| {
            std::task::Poll::Ready(match std::pin::Pin::new(&mut *future).poll(context) {
                std::task::Poll::Ready(value) => Some(value),
                std::task::Poll::Pending => None,
            })
        })
        .await
    }
}
