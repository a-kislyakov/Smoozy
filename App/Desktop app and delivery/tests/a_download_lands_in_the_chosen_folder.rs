//! **Step 10b piece 7's own proof: a real file comes down, into the folder the
//! user chose, at a rate the cap holds — and two classes take turns for the one
//! slot a low cap leaves.**
//!
//! **What the far end is.** OpenSSH's own `sftp-server`, the same program a
//! rental's own file-transfer server is and the one every observation behind
//! `Server protocol and transfers` §7 was taken against. It is spoken to over a
//! socket pair rather than over an SSH connection, and that difference is
//! stated rather than glossed: **what this file proves is the byte loop, the
//! budget, the admission and the publication**, none of which the transport
//! touches. `App/Server protocol and transfers/tests/` already drives the same
//! loop over a real SSH connection on the loopback address, and standing a
//! second SSH server up here would want a package this crate does not carry.
//!
//! **Nothing of the Creator's is touched.** Every path below is inside a
//! throwaway folder made for the test and deleted with it, and the download
//! folder is set to one of those — so **W4's *nothing is written into the
//! folder unasked* is never put to the test on the user's own Downloads
//! folder**, because that folder is never asked for here.
//!
//! **Where `sftp-server` is not on this system the tests say so and stop.**
//! They do not quietly pass: a printed line names the system and the miss.
//! **The socket pair is a Unix one**, so on Windows the whole file says that
//! and stops; nothing in the piece is per-system in shape, and what a Windows
//! run would add is the compile.
//!
//! **How a clause marker is written in this crate, and it is a rule rather than
//! a habit.** A marker with no document beside it means a clause of this
//! crate's own plan, `Preparation/Architecture/Desktop app and delivery.md`.
//! **Every marker of another document is written out with that document at its
//! own site and this file keeps no header list** — a list cannot tell a marker
//! both plans define from one only the other does, and `Download` collides with
//! this plan on almost every letter of its own series.

/// **On a system with no Unix socket pair the whole file says so and stops**,
/// which is the shape every integration test in this workspace uses for ground
/// it cannot stand on.
#[cfg(not(unix))]
#[test]
fn the_far_end_here_wants_a_unix_socket_pair() {
    println!(
        "SKIPPED: the far end here is OpenSSH's own sftp-server on a Unix socket pair, which \
         this system ({}) has none of. The download contract is unproved here.",
        std::env::consts::OS
    );
}

#[cfg(unix)]
mod on_a_desk_with_a_socket_pair {
    use std::path::{Path, PathBuf};
    use std::time::{Duration, Instant};

    use smoozy_desktop::downloading::{
        one_file_comes_down, where_a_download_lands, AFileToBringDown, TheRunsTransfers,
        WhereItLands, WhereTheCallerSaysItLands,
    };
    use smoozy_desktop::record::{settings, LocalRecord};
    use smoozy_desktop::scheduling::{
        BytesPerSecond, TheAdmissibleCount, TheCap, THE_TRAILING_WINDOW,
    };
    use smoozy_download::classes::{the_class, WhatArrangedIt};
    use smoozy_server_protocol::subsystem::client::Subsystem;
    use smoozy_server_protocol::transfers::{Direction, Ending};
    use smoozy_state::identities::{Identity, IdentityKind};

    // --------------------------------------- where OpenSSH's own server is

    /// The places each system's own OpenSSH puts its file-transfer server.
    ///
    /// **Read from the disk rather than assumed**: the test asks whether the
    /// file is there and says so when it is not.
    const WHERE_SFTP_SERVER_LIVES: &[&str] = &[
        "/usr/libexec/sftp-server",
        "/usr/lib/openssh/sftp-server",
        "/usr/libexec/openssh/sftp-server",
        "/usr/lib/ssh/sftp-server",
    ];

    fn the_machines_own_file_transfer_server() -> Option<PathBuf> {
        WHERE_SFTP_SERVER_LIVES
            .iter()
            .map(PathBuf::from)
            .find(|path| path.exists())
    }

    /// Says the miss out loud rather than passing quietly.
    macro_rules! the_server_or_say_so {
        () => {
            match the_machines_own_file_transfer_server() {
                Some(path) => path,
                None => {
                    println!(
                        "SKIPPED: no OpenSSH sftp-server on this system ({}). \
                         The download contract is unproved here.",
                        std::env::consts::OS
                    );
                    return;
                }
            }
        };
    }

    // --------------------------------------------------- the throwaway folder

    /// A folder of this test's own, deleted when it goes.
    struct Scratch {
        at: PathBuf,
    }

    impl Scratch {
        fn new(named: &str) -> Scratch {
            let at = std::env::temp_dir()
                .join(format!("smoozy-download-{named}-{}", std::process::id()));
            let _ = std::fs::remove_dir_all(&at);
            std::fs::create_dir_all(&at).expect("the throwaway folder is made");
            Scratch { at }
        }

        fn folder(&self, named: &str) -> PathBuf {
            let at = self.at.join(named);
            std::fs::create_dir_all(&at).expect("the folder is made");
            at
        }

        fn a_file_of(&self, named: &str, size: usize) -> (PathBuf, Vec<u8>) {
            let bytes: Vec<u8> = (0..size).map(|at| (at % 251) as u8).collect();
            let at = self.folder("machine").join(named);
            std::fs::write(&at, &bytes).expect("the file is written");
            (at, bytes)
        }
    }

    impl Drop for Scratch {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.at);
        }
    }

    // ------------------------------------------- the far end, over a socket pair

    /// **`sftp-server` on one end of a socket pair, and Smoozy's own subsystem
    /// client on the other.**
    ///
    /// The program speaks its protocol on its own standard input and output,
    /// which is exactly how an SSH subsystem session carries it; the pair
    /// stands in for the channel.
    async fn the_machines_server(
        program: &Path,
    ) -> (Subsystem<tokio::net::UnixStream>, std::process::Child) {
        use std::os::unix::io::{AsRawFd, FromRawFd};

        let (ours, theirs) = std::os::unix::net::UnixStream::pair().expect("a socket pair");
        ours.set_nonblocking(true).expect("the end we drive");

        let reading = unsafe { std::process::Stdio::from_raw_fd(copy_of(theirs.as_raw_fd())) };
        let writing = unsafe { std::process::Stdio::from_raw_fd(copy_of(theirs.as_raw_fd())) };
        let child = std::process::Command::new(program)
            .stdin(reading)
            .stdout(writing)
            .stderr(std::process::Stdio::null())
            .spawn()
            .expect("the machine's own file-transfer server starts");
        drop(theirs);

        let ours = tokio::net::UnixStream::from_std(ours).expect("the end we drive");
        let subsystem = Subsystem::negotiate(ours)
            .await
            .expect("the file-transfer server negotiates");
        (subsystem, child)
    }

    fn copy_of(descriptor: std::os::unix::io::RawFd) -> std::os::unix::io::RawFd {
        let copy = unsafe { libc::dup(descriptor) };
        assert!(copy >= 0, "the descriptor could not be copied");
        copy
    }

    // ------------------------------------------------------------- the record

    /// A throwaway record with the download folder the user chose already in
    /// it.
    fn a_record_pointing_at(scratch: &Scratch, chosen: &Path) -> rusqlite::Connection {
        let at = scratch.at.join("record/smoozy.sqlite3");
        std::fs::create_dir_all(at.parent().expect("a folder")).expect("made");
        let record = LocalRecord::open_at(&at).expect("the record opens");
        settings::the_download_folder_was_chosen(record.connection(), &the_bytes_of(chosen))
            .expect("the folder saves");
        drop(record);
        rusqlite::Connection::open(&at).expect("the record opens again")
    }

    fn the_bytes_of(path: &Path) -> Vec<u8> {
        use std::os::unix::ffi::OsStrExt;
        path.as_os_str().as_bytes().to_vec()
    }

    fn asked_for(
        on_the_machine: &Path,
        lands_under: &str,
        size: u64,
        by: WhatArrangedIt,
    ) -> AFileToBringDown {
        let issue = |kind| Identity::issue(kind).expect("random bytes");
        AFileToBringDown {
            on_the_machine: the_bytes_of(on_the_machine),
            // **`Download` H2's standing setting**, which is the route a
            // caller with no record behind it takes; step 10c piece 10 gave
            // the arrangement and the request their own, because each fixes a
            // destination of its own that may not drift to what the setting
            // reads later.
            lands: WhereTheCallerSaysItLands::UnderTheStandingFolder(
                lands_under.as_bytes().to_vec(),
            ),
            size,
            arranged_by: by,
            server_row: issue(IdentityKind::LocalServer),
            installation: Some(issue(IdentityKind::Installation)),
            run: issue(IdentityKind::Run),
            // **No identity is handed in here**, so one is issued at the
            // fetch — `State, identity and recovery` §9.5's fresh transfer.
            transfer: None,
        }
    }

    // ---------------------------------------------------------------- the proofs

    /// **`Download` H2: a real file comes down into the folder the user chose,
    /// and nowhere else.**
    #[tokio::test]
    async fn a_real_file_lands_in_the_folder_the_user_chose() {
        let _deadline = TheTestsDeadline::started();
        let server = the_server_or_say_so!();
        let scratch = Scratch::new("chosen");
        let chosen = scratch.folder("the folder the user chose");
        let mut connection = a_record_pointing_at(&scratch, &chosen);

        // **The chosen folder is what the record answers with**, and it is the
        // user's own rather than the system's proposal.
        assert_eq!(
            where_a_download_lands(&connection),
            WhereItLands::TheFolderTheUserChose(chosen.clone())
        );

        let (on_the_machine, bytes) = scratch.a_file_of("frame_0001.exr", 2_097_152);
        let (mut subsystem, mut child) = the_machines_server(&server).await;
        let run = TheRunsTransfers::for_this_run();

        let came_to = one_file_comes_down(
            &mut subsystem,
            &run,
            &mut connection,
            &asked_for(
                &on_the_machine,
                "frame_0001.exr",
                bytes.len() as u64,
                WhatArrangedIt::AManualRequest,
            ),
            &[the_class(WhatArrangedIt::AManualRequest)],
        )
        .await
        .expect("nothing refused the download");
        let _ = child.kill();

        assert_eq!(
            came_to.ending,
            Ending::Published,
            "the download did not publish"
        );
        assert_eq!(came_to.lands_at, chosen.join("frame_0001.exr"));
        assert_eq!(
            std::fs::read(&came_to.lands_at).expect("the file is on this computer"),
            bytes,
            "what landed is not what was on the machine"
        );
        // **`Server protocol and transfers` S26: a published transfer leaves no
        // staged material behind.**
        let left_behind: Vec<_> = std::fs::read_dir(chosen.join(".smoozy-staging"))
            .map(|entries| {
                entries
                    .filter_map(Result::ok)
                    .map(|entry| entry.path())
                    .collect()
            })
            .unwrap_or_default();
        assert!(
            left_behind.is_empty(),
            "staged material was left behind: {left_behind:?}"
        );
    }

    /// **A cap actually holding a rate, rather than a cap being read.**
    ///
    /// **The arithmetic is the mechanism's own and not a guess.** The budget is
    /// a trailing window (`Server protocol and transfers` R6b): no window of
    /// [`THE_TRAILING_WINDOW`] ever holds more than the window's worth of the
    /// cap, so the first window's worth is available at once and everything
    /// after it is paced. **A file of `size` bytes under a cap of `cap` bytes a
    /// second therefore takes at least `size / cap` less one window**, however
    /// quick or slow the machine is — a busy machine can only make it slower.
    ///
    /// **Both halves still count**, because a cap that is never reached proves
    /// nothing about a cap: the pacing has to be the cap's doing and not the
    /// machine's. **Until 2026-09-26 that was asked of the machine** — the
    /// uncapped run had to finish inside two and a half seconds or the test
    /// failed — and a busy hosted runner took 2.64 s in run `36240566832`, so
    /// the test failed over how busy the runner was and said nothing about the
    /// cap. **Now it is asked of the cap instead.** The uncapped run is taken
    /// first, as before, and the cap is set at a million bytes a second or at
    /// a sixth of the rate that run reached, whichever is lower. **So the cap
    /// is always well below what this machine has just shown it moves**, and
    /// the least time the cap allows is always several times what the same
    /// file took uncapped: at a quick machine it is five seconds against a
    /// fraction of one, and at a slow one it is six times the uncapped run
    /// less one window, which is never less than 3.75 times it.
    ///
    /// **What a busy machine can still do, and it is only on the other side.**
    /// It cannot fail this test while the cap holds, because the one assertion
    /// is a least time and load only adds time. With the cap taken away, the
    /// test fails unless the second run happened to be well over three times
    /// slower than the first, one straight after the other. **The one bound
    /// left is the test's own deadline**: the capped run lasts about six times
    /// the uncapped one, so a machine that takes more than about forty seconds
    /// to move eight megabytes over a socket pair is stopped by the deadline
    /// rather than judged.
    #[tokio::test]
    async fn a_cap_holds_the_rate_a_download_runs_at() {
        /// The cap on a machine quick enough not to need a lower one.
        const THE_CAP_ON_A_QUICK_MACHINE: u64 = 1_000_000;
        /// How far below the uncapped run's own rate the cap is set.
        const HOW_FAR_BELOW_THE_UNCAPPED_RATE: f64 = 6.0;

        let _deadline = TheTestsDeadline::started();
        let server = the_server_or_say_so!();
        let scratch = Scratch::new("cap");
        let chosen = scratch.folder("downloads");
        let mut connection = a_record_pointing_at(&scratch, &chosen);
        let size = 8_000_000usize;
        let (on_the_machine, bytes) = scratch.a_file_of("big.exr", size);

        // ---- Uncapped, which is R8's ordinary state.
        let (mut subsystem, mut child) = the_machines_server(&server).await;
        let run = TheRunsTransfers::for_this_run();
        let began = Instant::now();
        let came_to = one_file_comes_down(
            &mut subsystem,
            &run,
            &mut connection,
            &asked_for(
                &on_the_machine,
                "uncapped.exr",
                bytes.len() as u64,
                WhatArrangedIt::AManualRequest,
            ),
            &[the_class(WhatArrangedIt::AManualRequest)],
        )
        .await
        .expect("nothing refused the download");
        let uncapped = began.elapsed();
        let _ = child.kill();
        assert_eq!(came_to.ending, Ending::Published);

        // ---- The cap, well below what this machine has just shown it moves.
        let the_uncapped_rate = size as f64 / uncapped.as_secs_f64().max(f64::MIN_POSITIVE);
        let cap = (the_uncapped_rate / HOW_FAR_BELOW_THE_UNCAPPED_RATE)
            .min(THE_CAP_ON_A_QUICK_MACHINE as f64)
            .max(1.0) as u64;
        let the_least_the_cap_allows =
            Duration::from_secs_f64(size as f64 / cap as f64).saturating_sub(THE_TRAILING_WINDOW);

        // ---- The same file with that cap set.
        let (mut subsystem, mut child) = the_machines_server(&server).await;
        let run = TheRunsTransfers::for_this_run();
        run.the_caps_saved_are(TheCap::NoCapIsSet, TheCap::Of(BytesPerSecond(cap)));
        let began = Instant::now();
        let came_to = one_file_comes_down(
            &mut subsystem,
            &run,
            &mut connection,
            &asked_for(
                &on_the_machine,
                "capped.exr",
                bytes.len() as u64,
                WhatArrangedIt::AManualRequest,
            ),
            &[the_class(WhatArrangedIt::AManualRequest)],
        )
        .await
        .expect("nothing refused the download");
        let capped = began.elapsed();
        let _ = child.kill();

        assert_eq!(came_to.ending, Ending::Published);
        assert_eq!(
            std::fs::read(chosen.join("capped.exr")).expect("it landed"),
            bytes
        );
        // **A tenth off the least time, as this test has always had.** The
        // arithmetic above needs none, since the clock starts here before the
        // first draw; it is kept so that this repair changes one thing only.
        assert!(
            capped >= the_least_the_cap_allows.mul_f64(0.9),
            "the cap did not hold the rate: {size} bytes came down in {capped:?} against a cap \
             of {cap} bytes a second, which allows no less than {the_least_the_cap_allows:?}; \
             the same file uncapped took {uncapped:?}"
        );
    }

    /// **R17 at an admissible count of one, with a second class waiting, driven
    /// through real downloads.**
    ///
    /// **This is the clause a first caller makes true or false.** At a low cap
    /// the admissible count is one; an auto-download arrangement that keeps
    /// trying and never switches itself off could hold that one slot for ever,
    /// and R13 makes every refusal behind it silent — so the user's own manual
    /// download would show *underway* for ever and nothing would be reported.
    ///
    /// **The test fails if the first caller holds the slot**: the second
    /// download asks with both classes waiting, and a held slot answers it with
    /// nothing at all.
    #[tokio::test]
    async fn at_a_count_of_one_a_second_class_waiting_gets_the_next_slot() {
        let _deadline = TheTestsDeadline::started();
        let server = the_server_or_say_so!();
        let scratch = Scratch::new("turns");
        let chosen = scratch.folder("downloads");
        let mut connection = a_record_pointing_at(&scratch, &chosen);
        let (on_the_machine, bytes) = scratch.a_file_of("frame.exr", 32_768);

        let run = TheRunsTransfers::for_this_run();
        // **The floor is what makes the count one**, which is R7's quotient at a
        // cap of exactly one floor.
        run.the_caps_saved_are(
            TheCap::NoCapIsSet,
            TheCap::Of(smoozy_desktop::scheduling::THE_FLOOR_PER_DIRECTION),
        );
        assert_eq!(
            run.the_admissible_count(Direction::Down),
            TheAdmissibleCount::Of(1),
            "this is only R17's case where the admissible count is one"
        );

        let auto = the_class(WhatArrangedIt::AnAutoDownloadArrangement);
        let manual = the_class(WhatArrangedIt::AManualRequest);
        // Both classes hold work waiting at every decision, which is the shape
        // Row 219 describes: it keeps trying and never switches itself off.
        let waiting = [auto, manual];

        let mut admitted_as = Vec::new();
        for (at, by) in [
            WhatArrangedIt::AnAutoDownloadArrangement,
            WhatArrangedIt::AManualRequest,
            WhatArrangedIt::AnAutoDownloadArrangement,
        ]
        .into_iter()
        .enumerate()
        {
            let (mut subsystem, mut child) = the_machines_server(&server).await;
            let came_to = one_file_comes_down(
                &mut subsystem,
                &run,
                &mut connection,
                &asked_for(
                    &on_the_machine,
                    &format!("frame_{at}.exr"),
                    bytes.len() as u64,
                    by,
                ),
                &waiting,
            )
            .await
            .expect(
                "the slot did not come to the class whose turn it was, which is the first \
                 caller holding the one slot R17's turn-taking is about",
            );
            let _ = child.kill();
            assert_eq!(came_to.ending, Ending::Published);
            admitted_as.push(came_to.admitted_as);
        }

        assert_eq!(
            admitted_as,
            vec![auto, manual, auto],
            "at a count of one the guarantee is turn-taking between the classes with work \
             waiting, and the highest-ranked class took every slot instead"
        );
        // Three files, three landings — the class that waited is not a class
        // whose work was quietly dropped.
        for at in 0..3 {
            assert_eq!(
                std::fs::read(chosen.join(format!("frame_{at}.exr"))).expect("it landed"),
                bytes
            );
        }
    }

    /// **Must-do 1 of step 10b round 1 of *jobs, history and downloads*: a
    /// grant handed back unspent is not that class's turn.**
    ///
    /// At a count of one with both classes waiting, the first offer is the
    /// manual file — the class the point would **not** grant. The point grants
    /// the auto class and the file is handed back. The auto file is offered
    /// next, and **it must come down**.
    ///
    /// **This failed before the fix**: the hand-back had already been recorded
    /// as the auto class's turn, so the point granted the manual class to the
    /// auto file, which was handed back in its turn — and at a count of one
    /// every offer is then refused and nothing ever moves.
    #[tokio::test]
    async fn a_grant_handed_back_unspent_does_not_use_up_that_class_s_turn() {
        let _deadline = TheTestsDeadline::started();
        use smoozy_desktop::downloading::WhyNothingCameDown;

        let server = the_server_or_say_so!();
        let scratch = Scratch::new("handed-back");
        let chosen = scratch.folder("downloads");
        let mut connection = a_record_pointing_at(&scratch, &chosen);
        let (on_the_machine, bytes) = scratch.a_file_of("frame.exr", 32_768);

        let run = TheRunsTransfers::for_this_run();
        run.the_caps_saved_are(
            TheCap::NoCapIsSet,
            TheCap::Of(smoozy_desktop::scheduling::THE_FLOOR_PER_DIRECTION),
        );
        assert_eq!(run.the_admissible_count(Direction::Down), TheAdmissibleCount::Of(1));

        let auto = the_class(WhatArrangedIt::AnAutoDownloadArrangement);
        let manual = the_class(WhatArrangedIt::AManualRequest);
        let waiting = [auto, manual];

        let (mut subsystem, mut child) = the_machines_server(&server).await;
        let handed_back = one_file_comes_down(
            &mut subsystem,
            &run,
            &mut connection,
            &asked_for(&on_the_machine, "manual.exr", bytes.len() as u64, WhatArrangedIt::AManualRequest),
            &waiting,
        )
        .await;
        let _ = child.kill();
        assert!(
            matches!(handed_back, Err(WhyNothingCameDown::TheSlotWentToAnotherClass(class)) if class == auto),
            "the point was meant to grant the auto class first: {handed_back:?}"
        );

        let (mut subsystem, mut child) = the_machines_server(&server).await;
        let came_to = one_file_comes_down(
            &mut subsystem,
            &run,
            &mut connection,
            &asked_for(
                &on_the_machine,
                "auto.exr",
                bytes.len() as u64,
                WhatArrangedIt::AnAutoDownloadArrangement,
            ),
            &waiting,
        )
        .await
        .expect(
            "the file of the class that was granted was refused, so the handed-back grant \
             used up that class's turn",
        );
        let _ = child.kill();
        assert_eq!(came_to.ending, Ending::Published);
        assert_eq!(came_to.admitted_as, auto);
        assert_eq!(std::fs::read(chosen.join("auto.exr")).expect("it landed"), bytes);
    }

    // ═══════════════════════════════════════════════════════════════════════
    // Step 10c piece 10 — the two routes, over the same real server
    // ═══════════════════════════════════════════════════════════════════════
    //
    // **What these two prove and what they do not.** The walk, the enumeration,
    // the judgement, the name check, the fetch, the publication and the two
    // durable stores all run here against OpenSSH's own `sftp-server`. **What
    // they do not run is `Server protocol and transfers` T5's per-server
    // point**, which lends the reading session and the transfer's seat: this
    // file speaks to one subsystem directly, so the seat is proved by
    // `src/servers/sessions.rs`'s own tests and by the structural guard in
    // `src/downloading.rs` that no field anywhere holds one. **The sequence
    // below is the one `one_turn_of_an_arrangement` and `the_download_runs`
    // perform**, with the point taken out.

    use smoozy_desktop::downloading::{
        the_arrangement_is_made_at_submission, the_request_goes_without_a_question,
        the_folder_a_file_lands_in, ADownloadReadyToRun, TheArrangementsKeeper,
        TheLandingFolders, TheReceivingSystemsRulesHere,
        TheRecordADownloadWrites, TheRequestsKeeper, WhatTheSubmissionChose,
    };
    use smoozy_download::acting::enumerating::{
        enumerate, make_the_landing_folders, AJobsOwnFolder, ReadsTheLocalJobRecords,
    };
    use smoozy_download::acting::fetching::TheFetchQueue;
    use smoozy_download::acting::running::{TheArrangementsRun, WhatIsInHand, WhatTheTurnDid};
    use smoozy_download::acting::wiring::TheReadingRoute;
    use smoozy_download::arrangement::{KeepsTheArrangement, TheNarrowing, WhatSmoozyDid};
    use smoozy_download::ending::ANarrowedEntry;
    use smoozy_download::request::{
        what_the_pre_flight_has_to_say, AManualRequest, KeepsTheRequest,
    };
    use smoozy_download::vouching::{
        AVouchingMark, TheAttemptsOrder, TheOrder, WhatTheLocalRecordSays,
    };
    use smoozy_download::watch::TheWindow;

    /// The record, as a door — the shape the running application's own is.
    struct ARecordDoor {
        connection: std::sync::Mutex<rusqlite::Connection>,
    }

    impl TheRecordADownloadWrites for ARecordDoor {
        fn with_the_record(&self, act: &mut dyn FnMut(&rusqlite::Connection)) -> bool {
            let held = self
                .connection
                .lock()
                .unwrap_or_else(|poisoned| poisoned.into_inner());
            act(&held);
            true
        }
    }

    /// **`Download` J9's reading, off the local job records alone.** Nothing
    /// here reads the machine, which is that clause's own rule.
    struct NoJobIsKnown;

    impl ReadsTheLocalJobRecords for NoJobIsKnown {
        fn what_the_local_record_says(&self, _source_path: &[u8]) -> WhatTheLocalRecordSays {
            WhatTheLocalRecordSays {
                the_file_is_inside_the_jobs_output_folder: false,
                the_recorded_ending: None,
                the_completeness_statement: None,
            }
        }
        fn the_files_frame(&self, _source_path: &[u8]) -> Option<i64> {
            None
        }
        fn the_job_whose_output_folder_this_is(&self, _picked: &[u8]) -> Option<AJobsOwnFolder> {
            None
        }
    }

    /// **The whole arrangement route, over a real file-transfer server: made at
    /// submission, durable, walked, judged, fetched, and read back after a
    /// quit.**
    ///
    /// This is step 8's milestone in the small: a job's frames come back with
    /// nobody asking for them, from an arrangement the submission made.
    #[tokio::test]
    async fn an_arrangement_made_at_submission_walks_a_real_folder_and_brings_its_frames_home() {
        let _deadline = TheTestsDeadline::started();
        let server = the_server_or_say_so!();
        let scratch = Scratch::new("arrangement");
        let chosen = scratch.folder("chosen");
        let outputs = scratch.folder("machine");

        // Three frames on the machine, written in the attempt's own order.
        let mut bytes = Vec::new();
        for frame in 1..=3u32 {
            let (_, written) = scratch.a_file_of(&format!("frame_{frame:04}.png"), 4_096 * frame as usize);
            bytes.push(written);
        }

        let door = ARecordDoor {
            connection: std::sync::Mutex::new(a_record_pointing_at(&scratch, &chosen)),
        };
        let job = Identity::issue(IdentityKind::Job).expect("random bytes");
        let server_row = Identity::issue(IdentityKind::LocalServer).expect("random bytes");
        let run = Identity::issue(IdentityKind::Run).expect("random bytes");

        // **`Download` R1: the arrangement is made at submission**, with the
        // destination fixed at that moment.
        let mut arrangement = the_arrangement_is_made_at_submission(
            &mut TheArrangementsKeeper { record: &door },
            true,
            WhatTheSubmissionChose {
                job: job.clone(),
                server_row: server_row.clone(),
                destination: the_bytes_of(&chosen),
                narrowing: TheNarrowing::Everything,
            },
        )
        .expect("the arrangement is made");

        let (mut subsystem, mut child) = the_machines_server(&server).await;
        let entries: Vec<ANarrowedEntry> = (1..=3)
            .map(|frame| ANarrowedEntry {
                predicted_path: format!("frame_{frame:04}.png").into_bytes(),
                frame: Some(frame),
            })
            .collect();
        let order = TheOrder::The(TheAttemptsOrder::as_the_attempt_record_carries_it(vec![1, 2, 3]));
        let folder = the_bytes_of(&outputs);
        let rules = TheReceivingSystemsRulesHere;
        let mut running = TheArrangementsRun::fresh();
        let mut queue = TheFetchQueue::at_a_server();
        running.a_connection_began();
        // **The job ended**, which is what `Download` E1 step 1 hands in; the
        // module never finds out on its own.
        running.the_ending_arrived(smoozy_queue_history::ending::Ending::Finished);

        let in_hand = || WhatIsInHand {
            the_job_output_folder: &folder,
            narrowed_entries: &entries,
            the_whole_may_grow: false,
            the_entries_the_narrowing_excludes: &[],
            order: &order,
            the_jobs_label: None,
            the_first_frame_pair: None,
            the_pairs_other_half_fetched: &[],
            window: TheWindow {
                the_desired_state_is_on: true,
                a_connection_to_the_server_stands: true,
                the_attempts_window_is_open: true,
            },
            the_placed_copies: None,
            the_four_values_of_the_placed_copies: &[],
            arrangements_walking_this_server: 1,
        };

        // **E1 step 2: the ending walk, over the real server.**
        let came = {
            let mut reader = TheReadingRoute::on(&mut subsystem);
            running
                .one_turn(
                    &mut reader,
                    &rules,
                    &mut TheArrangementsKeeper { record: &door },
                    &mut queue,
                    &mut arrangement,
                    in_hand(),
                )
                .await
                .expect("the turn ran")
        };
        assert!(
            matches!(came, WhatTheTurnDid::TheEndingWalkRan(_)),
            "the ending walk did not run over the real server: {came:?}"
        );
        assert_eq!(queue.owed(), 3, "three frames became fetchable");

        // **E1 step 3: the queue drains, each file through `one_file_comes_down`.**
        let run_of_transfers = TheRunsTransfers::for_this_run();
        let mut connection = rusqlite::Connection::open(
            scratch.at.join("record/smoozy.sqlite3"),
        )
        .expect("the record opens for the transfer");
        let mut fetched = 0;
        let mut landed_at: Vec<PathBuf> = Vec::new();
        while let Some(unit) = queue.offer(&[]) {
            let mut on_the_machine = folder.clone();
            on_the_machine.push(b'/');
            on_the_machine.extend_from_slice(&unit.key);
            // **P11's structure, made before the publication's rename** — the
            // same call the driver makes, and the one an arrangement had no
            // maker for until this piece.
            the_folder_a_file_lands_in(&unit.the_landing_path)
                .expect("the folder the frame lands in is made");
            // **The size the fetch is arranged against**, read on the same
            // session the walk rides — which is what the driver does for a file
            // the ending walk sized and the series did not.
            let size = subsystem
                .stat(&on_the_machine)
                .await
                .expect("the file's own attributes")
                .size
                .expect("the server carried a size");
            let came_to = one_file_comes_down(
                &mut subsystem,
                &run_of_transfers,
                &mut connection,
                &AFileToBringDown {
                    on_the_machine,
                    lands: WhereTheCallerSaysItLands::TheFolderTheCallerHolds {
                        destination: unit.the_destination_normalised.clone(),
                        landing: unit.the_landing_path.clone(),
                    },
                    size,
                    arranged_by: WhatArrangedIt::AnAutoDownloadArrangement,
                    server_row: server_row.clone(),
                    installation: None,
                    run: run.clone(),
                    transfer: None,
                },
                &[the_class(WhatArrangedIt::AnAutoDownloadArrangement)],
            )
            .await
            .expect("the fetch ran");
            assert_eq!(came_to.ending, Ending::Published);
            landed_at.push(came_to.lands_at.clone());
            queue.a_unit_ended();
            // **R4's account half, written at the moment it became true.**
            let destination = arrangement.destination.clone();
            arrangement
                .reckoning
                .push(smoozy_download::arrangement::AReckoningEntry::acted_on(
                    unit.key.clone(),
                    None,
                    WhatSmoozyDid::Fetched,
                    destination,
                ));
            TheArrangementsKeeper { record: &door }
                .write_it_down(&arrangement)
                .expect("the reckoning is written down");
            fetched += 1;
        }
        let _ = child.kill();
        assert_eq!(fetched, 3);

        // **The frames are on the user's own computer, byte for byte, inside
        // the job's own folder** — `Download` P10 and M9: a job's outputs come
        // home inside the job's own folder, under Smoozy's own composed name,
        // so two jobs' outputs no more mix on the user's disk than on the
        // server's.
        landed_at.sort();
        bytes.sort_by_key(|written| written.len());
        assert_eq!(landed_at.len(), 3);
        for landed in &landed_at {
            assert!(
                landed.starts_with(&chosen),
                "a frame landed outside the folder the arrangement was given: {landed:?}"
            );
            assert_ne!(
                landed.parent(),
                Some(chosen.as_path()),
                "the job's own folder is not between the destination and the frame"
            );
        }
        for (at, written) in bytes.iter().enumerate() {
            let landed = &landed_at[at];
            assert_eq!(
                &std::fs::read(landed).expect("the frame landed"),
                written,
                "frame {} did not land whole",
                at + 1
            );
        }

        // **R11: the record survives the app closing and reopening.**
        drop(connection);
        let read = TheArrangementsKeeper { record: &door }
            .the_arrangement_for(&job)
            .expect("it reads")
            .expect("the arrangement is in the record");
        assert_eq!(read.reckoning.len(), 3);
        assert!(read
            .reckoning
            .iter()
            .all(|entry| entry.account.state == WhatSmoozyDid::Fetched));
    }

    /// **The manual route, over the same real server, with no arrangement
    /// anywhere in it** — `Download` M1, M2, M4, M6 and M16.
    ///
    /// This is step 10b's scenario's own route: its situation says outright
    /// *"auto-download was never switched on."*
    #[tokio::test]
    async fn a_manual_request_enumerates_a_real_folder_and_fetches_it_with_no_question() {
        let _deadline = TheTestsDeadline::started();
        let server = the_server_or_say_so!();
        let scratch = Scratch::new("manual");
        let chosen = scratch.folder("chosen");
        let outputs = scratch.folder("machine");
        let (_, one) = scratch.a_file_of("frame_0001.png", 2_048);
        let (_, two) = scratch.a_file_of("frame_0002.png", 3_072);

        let door = ARecordDoor {
            connection: std::sync::Mutex::new(a_record_pointing_at(&scratch, &chosen)),
        };
        let server_row = Identity::issue(IdentityKind::LocalServer).expect("random bytes");
        let run = Identity::issue(IdentityKind::Run).expect("random bytes");
        let (mut subsystem, mut child) = the_machines_server(&server).await;

        // **M2: the enumeration is a remote walk on the subsystem route.**
        let rules = TheReceivingSystemsRulesHere;
        let enumeration = {
            let mut reader = TheReadingRoute::on(&mut subsystem);
            enumerate(
                &mut reader,
                &rules,
                &NoJobIsKnown,
                &[the_bytes_of(&outputs)],
                &the_bytes_of(&chosen),
            )
            .await
        };
        assert_eq!(enumeration.entries.len(), 2, "the walk found both frames");
        assert!(enumeration.refusals.is_empty());

        // **M1: the request is durable before the question and before a byte
        // moves.**
        let mut request = AManualRequest::confirmed(
            Identity::issue(IdentityKind::Operation).expect("random bytes"),
            server_row.clone(),
            None,
            the_bytes_of(&chosen),
            enumeration.entries.clone(),
            run.clone(),
        );
        request.subtrees_that_could_not_be_read = enumeration.refusals.clone();
        TheRequestsKeeper { record: &door }
            .write_it_down(&request)
            .expect("the request is written down");

        // **M6, narrowed: nothing collides, nothing cannot be vouched for and
        // nothing refused, so no question is asked and no surface is composed.**
        let pre_flight = what_the_pre_flight_has_to_say(&request, &[]);
        assert!(
            !pre_flight.the_request_stops_and_asks(),
            "a clean request met a question with nothing in it"
        );
        let mut ready = ADownloadReadyToRun {
            request,
            colliding: Vec::new(),
            the_names_already_standing: Vec::new(),
            the_pre_flight: pre_flight,
            landing_folders: Vec::new(),
        };
        the_request_goes_without_a_question(&mut ready, &door).expect("the dispositions are written");
        // **M9: a downloaded folder arrives as a folder**, so the landing
        // folders are made before any byte moves — which is what the driver
        // itself does through the platform file adapter.
        for (folder, made) in make_the_landing_folders(&mut TheLandingFolders, &enumeration) {
            made.unwrap_or_else(|why| panic!("the landing folder {folder:?} was not made: {why:?}"));
        }

        // **The fetches, each through the one file's route.**
        let run_of_transfers = TheRunsTransfers::for_this_run();
        let mut connection = rusqlite::Connection::open(
            scratch.at.join("record/smoozy.sqlite3"),
        )
        .expect("the record opens for the transfer");
        for at in 0..ready.request.entries.len() {
            let entry = ready.request.entries[at].clone();
            let came_to = one_file_comes_down(
                &mut subsystem,
                &run_of_transfers,
                &mut connection,
                &AFileToBringDown {
                    on_the_machine: entry.source_path.clone(),
                    lands: WhereTheCallerSaysItLands::TheFolderTheCallerHolds {
                        destination: ready.request.destination.clone(),
                        landing: entry.landing_path.clone(),
                    },
                    size: entry.size_as_enumerated,
                    arranged_by: WhatArrangedIt::AManualRequest,
                    server_row: server_row.clone(),
                    installation: None,
                    run: run.clone(),
                    transfer: None,
                },
                &[the_class(WhatArrangedIt::AManualRequest)],
            )
            .await
            .expect("the fetch ran");
            assert_eq!(came_to.ending, Ending::Published);
            assert_eq!(came_to.admitted_as, the_class(WhatArrangedIt::AManualRequest));
            ready.request.entries[at]
                .its_outcome_became(smoozy_state::outcomes::UnitOutcome::Done)
                .expect("the unit's outcome is written once");
            TheRequestsKeeper { record: &door }
                .write_it_down(&ready.request)
                .expect("the outcome is written down");
        }
        let _ = child.kill();

        // **The folder arrived as a folder** (M9, P11): the structure the files
        // had on the server is the structure they have when they land.
        let landed = chosen.join("machine");
        assert_eq!(std::fs::read(landed.join("frame_0001.png")).expect("one"), one);
        assert_eq!(std::fs::read(landed.join("frame_0002.png")).expect("two"), two);

        // **M13: the record stands until the user removes it**, and its unit
        // outcomes came back.
        let read = TheRequestsKeeper { record: &door }
            .every_request_for(&server_row)
            .expect("it reads");
        assert_eq!(read.len(), 1);
        assert!(read[0]
            .entries
            .iter()
            .all(|entry| entry.outcome == Some(smoozy_state::outcomes::UnitOutcome::Done)));
    }

    /// **`Download` J9's reading for a browser request**, off the local job
    /// records alone: a file inside a job's output folder whose ending was
    /// *failed* is marked, and one outside every job's folder is not.
    struct OneJobEndedBadly {
        output_folder: Vec<u8>,
    }

    impl ReadsTheLocalJobRecords for OneJobEndedBadly {
        fn what_the_local_record_says(&self, source_path: &[u8]) -> WhatTheLocalRecordSays {
            WhatTheLocalRecordSays {
                the_file_is_inside_the_jobs_output_folder: source_path
                    .starts_with(&self.output_folder),
                the_recorded_ending: Some(smoozy_queue_history::ending::Ending::Failed),
                the_completeness_statement: None,
            }
        }
        fn the_files_frame(&self, _source_path: &[u8]) -> Option<i64> {
            None
        }
        fn the_job_whose_output_folder_this_is(&self, _picked: &[u8]) -> Option<AJobsOwnFolder> {
            None
        }
    }

    /// **Files' own Download, over a real file-transfer server: a file and a
    /// folder picked together come home with the structure they had.**
    ///
    /// **This is `Download` M2's own case** — *"files, folders, any mix, in any
    /// number … descending from what the user picked"* — and it is the one
    /// thing the browser's way in adds over the whole-job way in, which always
    /// picks exactly one folder. **P11's mirroring is what is being watched**:
    /// a picked file lands beside the destination, a picked folder arrives as a
    /// folder with what was under it under it.
    ///
    /// **`Download` J9's mark is watched at the same time**, because a browser
    /// request is the case `Download` J9 was written for: one request reaching
    /// files of a job and files of no job at all.
    #[tokio::test]
    async fn a_browser_request_of_a_file_and_a_folder_lands_with_its_structure() {
        let _deadline = TheTestsDeadline::started();
        let server = the_server_or_say_so!();
        let scratch = Scratch::new("browser");
        let chosen = scratch.folder("chosen");
        let outputs = scratch.folder("machine");
        let under = scratch.folder("machine/shot");
        let loose: Vec<u8> = (0..2_048).map(|at| (at % 251) as u8).collect();
        let deep: Vec<u8> = (0..5_120).map(|at| (at % 241) as u8).collect();
        std::fs::write(outputs.join("notes.txt"), &loose).expect("the loose file");
        std::fs::write(under.join("frame_0001.png"), &deep).expect("the file under the folder");

        let door = ARecordDoor {
            connection: std::sync::Mutex::new(a_record_pointing_at(&scratch, &chosen)),
        };
        let server_row = Identity::issue(IdentityKind::LocalServer).expect("random bytes");
        let run = Identity::issue(IdentityKind::Run).expect("random bytes");
        let (mut subsystem, mut child) = the_machines_server(&server).await;

        // **What the user picked: one file and one folder, in one request.**
        // The paths are the machine's as the listing gave them, and never a
        // display name (`Download` M1, `File Browser` N2).
        let picked = vec![
            the_bytes_of(&outputs.join("notes.txt")),
            the_bytes_of(&under),
        ];
        let rules = TheReceivingSystemsRulesHere;
        let records = OneJobEndedBadly {
            output_folder: the_bytes_of(&under),
        };
        let enumeration = {
            let mut reader = TheReadingRoute::on(&mut subsystem);
            enumerate(
                &mut reader,
                &rules,
                &records,
                &picked,
                &the_bytes_of(&chosen),
            )
            .await
        };
        assert_eq!(
            enumeration.entries.len(),
            2,
            "the file and the folder's one file"
        );
        assert!(enumeration.refusals.is_empty());

        // **`Download` J9: the file inside the job's output folder is marked
        // and the loose one is not**, because outside a job's folder Smoozy has
        // no standing to judge.
        let marked: Vec<_> = enumeration
            .entries
            .iter()
            .filter(|entry| entry.vouching == AVouchingMark::CannotBeVouchedFor)
            .map(|entry| entry.source_path.clone())
            .collect();
        assert_eq!(marked, vec![the_bytes_of(&under.join("frame_0001.png"))]);

        let mut request = AManualRequest::confirmed(
            Identity::issue(IdentityKind::Operation).expect("random bytes"),
            server_row.clone(),
            None,
            the_bytes_of(&chosen),
            enumeration.entries.clone(),
            run.clone(),
        );
        request.subtrees_that_could_not_be_read = enumeration.refusals.clone();
        TheRequestsKeeper { record: &door }
            .write_it_down(&request)
            .expect("the request is written down before a byte moves");

        // **M6: the question is asked because there is something to say** —
        // nothing collides, but one file cannot be vouched for.
        let pre_flight = what_the_pre_flight_has_to_say(&request, &[]);
        assert!(pre_flight.the_request_stops_and_asks());
        assert_eq!(pre_flight.cannot_vouch_for, 1);
        assert_eq!(pre_flight.the_headline(), 0);

        let mut ready = ADownloadReadyToRun {
            request,
            colliding: Vec::new(),
            the_names_already_standing: Vec::new(),
            the_pre_flight: pre_flight,
            landing_folders: Vec::new(),
        };
        the_request_goes_without_a_question(&mut ready, &door)
            .expect("the dispositions are written");
        for (folder, made) in make_the_landing_folders(&mut TheLandingFolders, &enumeration) {
            made.unwrap_or_else(|why| {
                panic!("the landing folder {folder:?} was not made: {why:?}")
            });
        }

        let run_of_transfers = TheRunsTransfers::for_this_run();
        let mut connection = rusqlite::Connection::open(scratch.at.join("record/smoozy.sqlite3"))
            .expect("the record opens for the transfer");
        for at in 0..ready.request.entries.len() {
            let entry = ready.request.entries[at].clone();
            let came_to = one_file_comes_down(
                &mut subsystem,
                &run_of_transfers,
                &mut connection,
                &AFileToBringDown {
                    on_the_machine: entry.source_path.clone(),
                    lands: WhereTheCallerSaysItLands::TheFolderTheCallerHolds {
                        destination: ready.request.destination.clone(),
                        landing: entry.landing_path.clone(),
                    },
                    size: entry.size_as_enumerated,
                    arranged_by: WhatArrangedIt::AManualRequest,
                    server_row: server_row.clone(),
                    installation: None,
                    run: run.clone(),
                    transfer: None,
                },
                &[the_class(WhatArrangedIt::AManualRequest)],
            )
            .await
            .expect("the fetch ran");
            assert_eq!(came_to.ending, Ending::Published);
        }
        let _ = child.kill();

        // **P11: the structure the files had on the server is the structure
        // they have when they land.** The picked file sits beside the
        // destination; the picked folder arrived as a folder.
        assert_eq!(
            std::fs::read(chosen.join("notes.txt")).expect("the loose file"),
            loose
        );
        assert_eq!(
            std::fs::read(chosen.join("shot/frame_0001.png")).expect("the file under the folder"),
            deep
        );
    }

    // ------------------------------------------------------ this test's own deadline

    /// **What every test in this file waits on**, named in the message a missed
    /// deadline prints.
    const WHAT_THE_TESTS_HERE_WAIT_ON: &str = "OpenSSH's own sftp-server on a socket pair";

    /// **No test in this file can wait for ever.** Added 2026-09-21, after hosted
    /// runs `35595657437` and `35601023619` sat three and a half hours on two tests
    /// that were waiting for an answer that could never come, and told nobody.
    ///
    /// **A test's own bound and nothing of the plan's**: five minutes, where the
    /// slowest test in the workspace waits a minute and a half on purpose and the
    /// rest take seconds. It is started as a test's first line and ends when the
    /// test does, pass or panic, because dropping it closes the channel it watches.
    /// **If it is still standing at the bound, a wait inside the test never
    /// ended**: it says which test and what it was waiting on, ends every program
    /// this binary started, and ends the binary, so the run fails instead of
    /// hanging.
    struct TheTestsDeadline {
        _held: std::sync::mpsc::Sender<()>,
    }

    impl TheTestsDeadline {
        const THE_BOUND: std::time::Duration = std::time::Duration::from_secs(300);

        fn started() -> TheTestsDeadline {
            let (held, watched) = std::sync::mpsc::channel::<()>();
            let test = std::thread::current()
                .name()
                .unwrap_or("a test in this file")
                .to_string();
            std::thread::spawn(move || {
                if watched.recv_timeout(Self::THE_BOUND)
                    == Err(std::sync::mpsc::RecvTimeoutError::Timeout)
                {
                    // **Straight to the complaints stream, not through `eprintln!`**: the
                    // test harness captures that in a test's threads and never prints what
                    // it captured when the binary is ended from inside, which is what
                    // happens next.
                    let said = format!(
                        "DEADLINE MISSED: {test} was still waiting after {:?}, on {}. Every \
                         program this test binary started is ended now, and so is the binary, so \
                         that this fails rather than hangs.\n",
                        Self::THE_BOUND,
                        WHAT_THE_TESTS_HERE_WAIT_ON
                    );
                    let _ = std::io::Write::write_all(&mut std::io::stderr(), said.as_bytes());
                    if cfg!(unix) {
                        let _ = std::process::Command::new("pkill")
                            .args(["-KILL", "-P", &std::process::id().to_string()])
                            .status();
                    }
                    std::process::exit(101);
                }
            });
            TheTestsDeadline { _held: held }
        }
    }
}
