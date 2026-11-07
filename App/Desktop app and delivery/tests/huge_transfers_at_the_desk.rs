//! **The kept harness: huge files and huge folders, moved at the desk through
//! the application's real routes, with the real record at its real settings
//! and every hold of the record's lock timed.** Written 2026-09-28 by step
//! 10c's transfer wave, piece 1 — *the record's cost*.
//!
//! **Why it is in the tree.** The tester who first measured a 4 GB upload at
//! two hours wrote a harness like this in a scratch folder, and a restart
//! cleared it; the one before it was lost the same way. So this one is a test
//! file, **ignored by default** because a run takes minutes to hours and wants
//! gigabytes of disk, and run on purpose:
//!
//! ```text
//! cargo test -p smoozy-desktop --release --test huge_transfers_at_the_desk \
//!     -- --ignored --nocapture --test-threads=1
//! ```
//!
//! **What it is sized by, read from the environment**, each with the size the
//! wave's figures were taken at:
//!
//! - `SMOOZY_HARNESS_BYTES` — the one large file, up then down
//!   (4,000,000,000 for the figures; 400,000,000 when unset).
//! - `SMOOZY_HARNESS_FILES` — the small files of the folder upload
//!   (10,002 for the figures and when unset).
//! - `SMOOZY_HARNESS_FRAMES` — the 64 KB frames of the auto-download
//!   (5,000 for the figures and when unset).
//! - `SMOOZY_HARNESS_SCRATCH` — where the throwaway folders go (cargo's own
//!   folder for integration tests when unset). **A 4 GB run wants about 8 GB
//!   there at its peak**: the source and its landed copy, then the landed copy
//!   and the one downloaded from it.
//! - `SMOOZY_HARNESS_LIVE_TRANSFER_TIMES=1` — the large sitting B reproduction
//!   uses the application's 15-second keepalive and 20-second opening deadline.
//! - `SMOOZY_HARNESS_DEADLINE_SECONDS` — how long one test may run before it
//!   is ended as hung (3,600 when unset).
//! - `SMOOZY_HARNESS_CAP` — a cap in bytes a second put on both directions of
//!   the large file (none when unset), so the speed beside the cap is read
//!   against a transfer that lasts longer than a minute (step 10c's transfer
//!   wave piece 5).
//!
//! **What the far end is.** The shape of `tests/through_the_servers_own_point.rs`:
//! a stand-in machine whose every session is **OpenSSH's own `sftp-server`**
//! over a Unix socket pair, behind the **real per-server point**
//! (`ThisServersSessions`) with a session limit of ten. **What it is not is an
//! SSH connection**, so what is measured is everything above the transport:
//! the point, the byte loop, the request, the arrangement and the record.
//! **One section is the exception**, *one real connection, paced*: OpenSSH's
//! own `sshd` behind a line with a round trip on it, so what two channels
//! share on one connection is measured too (step 10c's owed piece 3).
//! **What no desk line here can carry is TCP's own acknowledgement clock** —
//! this computer's loopback acknowledges at once — so a hold that waits on an
//! acknowledgement shows on a real line and never here.
//!
//! **The record is the application's own `LocalRecord`, opened with its own
//! three settings** (`State, identity and recovery` §1.3 rule 6), behind one
//! lock as the running application's is. **Every hold of that lock is timed**,
//! and about once a second the harness takes the reading the Transfers screen
//! takes — `crate::surface::transfers::the_screen_the_record_answers`, through
//! the same lock — and times how long it waited and took.
//!
//! **What each test asserts is the wave's figure, scaled to the size it was
//! given**, so a run that misses one fails and says by how much. The figures
//! are `Audits/Build/Step 10c/The transfer wave before 0.1.7 — what each piece
//! is, which files it owns, and in what order.md` section 1 item 5.
//!
//! **Nothing of the Creator's is touched.** Every path is inside a throwaway
//! folder made for the test and deleted with it.
//!
//! **How a clause marker is written in this crate, and it is a rule rather than
//! a habit.** A marker with no document beside it means a clause of this
//! crate's own plan, `Preparation/Architecture/Desktop app and delivery.md`.
//! **Every marker of another document is written out with that document at its
//! own site and this file keeps no header list.**

#[cfg(not(unix))]
#[test]
fn the_far_end_here_wants_a_unix_socket_pair() {
    println!(
        "SKIPPED: the far end here is OpenSSH's own sftp-server on a Unix socket pair, which \
         this system ({}) has none of. The harness does not run here.",
        std::env::consts::OS
    );
}

#[cfg(unix)]
mod on_a_desk_with_a_socket_pair {
    use std::collections::HashMap;
    use std::io::{Read, Write};
    use std::path::{Path, PathBuf};
    use std::pin::Pin;
    use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
    use std::sync::{Arc, Mutex};
    use std::task::{Context, Poll};
    use std::time::{Duration, Instant};

    use tokio::io::{AsyncRead, AsyncWrite, ReadBuf};

    use smoozy_desktop::downloading::{
        a_download_is_prepared, one_turn_of_an_arrangement, the_arrangement_is_made_at_submission,
        the_download_runs, the_owed_fetches_come_down, the_request_goes_without_a_question,
        AnArrangementOwingFetches, TheArrangementsKeeper, TheRecordADownloadWrites,
        TheRunsTransfers, WhatIsBeingDownloaded, WhatTheSubmissionChose,
    };
    use smoozy_desktop::record::{server_rows, LocalRecord};
    use smoozy_desktop::servers::sessions::{
        lend_until_it_ends, AHeldConnection, NoSession, TheAdditionalConnection,
        ThisServersSessions,
    };
    use smoozy_desktop::scheduling::{BytesPerSecond, TheCap, THE_TRAILING_WINDOW};
    use smoozy_server_protocol::transfers::Direction;
    use smoozy_desktop::surface::files::{a_folder_is_read, WhatTheReadingFound, Words};
    use smoozy_desktop::surface::transfers::{
        the_screen_the_record_answers, ARequestDrawn, TheTransfersScreen,
    };
    use smoozy_desktop::uploading::{
        an_upload_is_prepared, the_answer_is_applied, the_upload_runs, TheRecordAnUploadWrites,
        WhatIsBeingUploaded, WhatTheAnswerLeft, WhenAFileIsPublished,
    };
    use smoozy_download::acting::enumerating::{AJobsOwnFolder, ReadsTheLocalJobRecords};
    use smoozy_download::acting::fetching::TheFetchQueue;
    use smoozy_download::acting::probing::TheProbesCadence;
    use smoozy_download::acting::running::{TheArrangementsRun, WhatIsInHand};
    use smoozy_download::arrangement::TheNarrowing;
    use smoozy_download::ending::ANarrowedEntry;
    use smoozy_download::vouching::{TheAttemptsOrder, TheOrder, WhatTheLocalRecordSays};
    use smoozy_download::watch::TheWindow;
    use smoozy_server_protocol::transport::{CommandResult, CommandString, Ending};
    use smoozy_servers::identity::{Host, KeyReference, Port, Username};
    use smoozy_servers::store::ServerRow;
    use smoozy_state::identities::{Identity, IdentityKind};
    use smoozy_state::outcomes::UnitOutcome;

    /// **One of the wave's figures, held or missed** — a miss is kept rather
    /// than failing at once, so a run says every figure before it fails.
    macro_rules! a_figure {
        ($missed:ident, $held:expr, $($said:tt)+) => {
            if !$held {
                $missed.push(format!($($said)+));
            }
        };
    }

    // --------------------------------------------------------- the sizing

    fn a_number_from(named: &str, otherwise: u64) -> u64 {
        std::env::var(named)
            .ok()
            .and_then(|said| said.replace(['_', ','], "").parse().ok())
            .unwrap_or(otherwise)
    }

    const FOUR_GB: f64 = 4_000_000_000.0;

    // --------------------------------------- where OpenSSH's own server is

    const WHERE_SFTP_SERVER_LIVES: &[&str] = &[
        "/usr/libexec/sftp-server",
        "/usr/lib/openssh/sftp-server",
        "/usr/libexec/openssh/sftp-server",
        "/usr/lib/ssh/sftp-server",
    ];

    macro_rules! the_server_or_say_so {
        () => {
            match WHERE_SFTP_SERVER_LIVES
                .iter()
                .map(PathBuf::from)
                .find(|path| path.exists())
            {
                Some(path) => path,
                None => {
                    println!(
                        "SKIPPED: no OpenSSH sftp-server on this system ({}). \
                         The harness does not run here.",
                        std::env::consts::OS
                    );
                    return;
                }
            }
        };
    }

    // --------------------------------------------------- the throwaway folder

    struct Scratch {
        at: PathBuf,
    }

    impl Scratch {
        fn new(named: &str) -> Scratch {
            let under = std::env::var_os("SMOOZY_HARNESS_SCRATCH")
                .map(PathBuf::from)
                .unwrap_or_else(|| PathBuf::from(env!("CARGO_TARGET_TMPDIR")));
            let at = under.join(format!("smzy-huge-{named}-{}", std::process::id()));
            let _ = std::fs::remove_dir_all(&at);
            std::fs::create_dir_all(&at).expect("the throwaway folder is made");
            let at = std::fs::canonicalize(&at).expect("the folder is there");
            Scratch { at }
        }

        fn folder(&self, named: &str) -> PathBuf {
            let at = self.at.join(named);
            std::fs::create_dir_all(&at).expect("the folder is made");
            at
        }
    }

    impl Drop for Scratch {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.at);
        }
    }

    fn the_bytes_of(path: &Path) -> Vec<u8> {
        use std::os::unix::ffi::OsStrExt;
        path.as_os_str().as_bytes().to_vec()
    }

    /// **Every eight bytes hold their own position in the file**, so a byte
    /// landing in the wrong place shows when the copy is compared.
    fn the_pattern_at(from: u64, into: &mut [u8]) {
        for (at, byte) in into.iter_mut().enumerate() {
            let position = from + at as u64;
            let word = (position / 8) * 8;
            *byte = word.to_le_bytes()[(position % 8) as usize];
        }
    }

    fn a_large_file_at(path: &Path, size: u64) {
        let mut file = std::fs::File::create(path).expect("the large file is made");
        let mut chunk = vec![0u8; 8 << 20];
        let mut at = 0u64;
        while at < size {
            let take = chunk.len().min((size - at) as usize);
            the_pattern_at(at, &mut chunk[..take]);
            file.write_all(&chunk[..take]).expect("the large file is written");
            at += take as u64;
        }
    }

    /// **What each of three parts weighs where only one is large** — step 10c's
    /// eleventh-pass fix piece 4. **A request's files now move several at
    /// once**, so a test that means *one file in flight* when it cuts the line
    /// or presses *Stop* makes the other two small: they land first, and the
    /// large one, `part_1.blend`, then moves alone.
    const A_SMALL_PART: u64 = 2_000_000;

    fn three_parts_one_large(folder: &Path, large: u64) {
        for at in 0..3 {
            let size = if at == 1 { large } else { A_SMALL_PART };
            a_large_file_at(&folder.join(format!("part_{at}.blend")), size);
        }
    }

    fn it_holds_the_pattern(path: &Path, size: u64) -> bool {
        let Ok(mut file) = std::fs::File::open(path) else {
            return false;
        };
        if file.metadata().map(|held| held.len()).unwrap_or(0) != size {
            return false;
        }
        let mut read = vec![0u8; 8 << 20];
        let mut wanted = vec![0u8; 8 << 20];
        let mut at = 0u64;
        while at < size {
            let take = read.len().min((size - at) as usize);
            if file.read_exact(&mut read[..take]).is_err() {
                return false;
            }
            the_pattern_at(at, &mut wanted[..take]);
            if read[..take] != wanted[..take] {
                return false;
            }
            at += take as u64;
        }
        true
    }

    // ----------------------------------------------- a machine, standing in

    /// **A machine with a session limit, whose every session is the machine's
    /// own `sftp-server`** — `tests/through_the_servers_own_point.rs`'s, less
    /// the supervisor this harness never asks.
    #[derive(Default)]
    struct AMachine {
        program: PathBuf,
        home: PathBuf,
        limit: usize,
        /// **Every `sftp-server` this machine started**, so a test can cut the
        /// line under a transfer that is moving. Added 2026-09-29, step 10c's
        /// transfer wave piece 5.
        children: Mutex<Vec<u32>>,
        /// **Every session asked for is held, unopened, while this stands** —
        /// so a test can cut the line again at the moment a transfer asks for
        /// its channel after a repair, which is a line that comes back and goes
        /// again. Added 2026-09-29, step 10c's transfer wave piece 6.
        holds_opens: AtomicBool,
        /// **A session was asked for while opens were held.**
        an_open_is_held: AtomicBool,
        /// **Every session asked for finds the connection gone while this
        /// stands** — what a real transport answers a channel asked for at
        /// the moment its line goes, a moment before it reports the ending
        /// itself. Added 2026-10-02, out of the hosted run that was red on
        /// Linux at `d7a25677`.
        opens_find_the_line_gone: AtomicBool,
        /// **A session was asked for while opens found the line gone.**
        an_open_found_the_line_gone: AtomicBool,
        /// **Every attribute read asked of this machine by path** — a `STAT`
        /// or an `LSTAT`, counted off the requests as they are sent. Added
        /// 2026-09-30, step 10c's eleventh-pass fix piece 2, so a test can say
        /// a fetch made no size read of its own. *(An `FSTAT` was counted too
        /// until 2026-10-08. Since the Creator's second notes, piece 10, every
        /// download sends one on the handle it opened, for the server file's
        /// time (`Server protocol and transfers` S15c). It is sent without
        /// waiting and rides with the reads, so it costs no round trip, and
        /// this count no longer includes it.)*
        attribute_reads: AtomicUsize,
        /// **Every staging folder asked for** — a `MKDIR` whose path ends in
        /// the reserved folder's name. Added 2026-09-30, step 10c's
        /// eleventh-pass fix piece 4, so a test can say a request asks for a
        /// folder once and not once per file.
        staging_folders_asked: AtomicUsize,
        /// **A round trip on every request**: every byte the desk sends
        /// reaches the machine this long later, and replies come back at
        /// once. Requests are delayed, not queued, so pipelined requests stay
        /// pipelined — `Transfer wave piece 13 — the pace at the desk.md`'s
        /// stand-in. Nought, the default, is no delay at all. Added
        /// 2026-10-01, step 10c's twelfth-pass fix piece 3.
        round_trip: Duration,
        /// **Each session's answers come back no faster than this**, in bytes
        /// a second — a line every session has to itself, so one large file
        /// is slow while small ones beside it are not. Nought, the default, is
        /// no pacing at all; it paces only where `round_trip` puts a line
        /// under the session. Added 2026-10-16, step 10c's owed piece 3.
        each_sessions_answers_a_second: u64,
        /// **Every request sent, by its type**, counted as it is sent. Added
        /// 2026-10-01, step 10c's twelfth-pass fix piece 3, so a count can be
        /// read as round trips.
        requests_by_type: Mutex<HashMap<u8, usize>>,
        /// **The machine answers `Server protocol and transfers` S13's
        /// four-value reading** — the one command
        /// `smoozy_render_settings::acting::four_values` sends, answered as
        /// GNU `stat -L -c '%s %i %Y %Z'` answers it, from the file the planted
        /// link names. **Unset, every command is refused**, as before: the
        /// reading cannot be taken. Added 2026-10-04, the 38's piece 9 (item
        /// 18).
        answers_the_four_values: AtomicBool,
        /// **Round trips, counted off the wire**: a request that goes out on a
        /// session with nothing outstanding on it. Requests sent while others
        /// wait for their answers ride with them and are not counted again.
        /// Added 2026-10-16, step 10c's owed piece 1, so a file's wait can be
        /// read as round trips rather than guessed.
        round_trips: AtomicUsize,
        /// **Every session this machine opened** — a seat. On a real line a
        /// seat costs at least the opening of its channel, which this stand-in
        /// does not delay. Added 2026-10-16, owed piece 1.
        seats: AtomicUsize,
        /// **Every attribute read of a path inside a staging folder** — `Server
        /// protocol and transfers` S12 step 1's reading of the staged part.
        /// Added 2026-10-16, owed piece 1.
        staging_reads: AtomicUsize,
        /// **Every removal of a path inside a staging folder**. Added
        /// 2026-10-16, owed piece 1.
        staging_removes: AtomicUsize,
        /// **Every extended request, by the extension it names.** Added
        /// 2026-10-16, owed piece 1.
        extended_by_name: Mutex<HashMap<String, usize>>,
    }

    impl AMachine {
        /// **What the machine has been asked, as a sum to subtract from** —
        /// round trips, seats, staging-folder asks, staged-part reads and
        /// removals, and the requests by type.
        fn the_count_now(&self) -> TheWireCount {
            TheWireCount {
                round_trips: self.round_trips.load(Ordering::SeqCst),
                seats: self.seats.load(Ordering::SeqCst),
                staging_folders: self.staging_folders_asked.load(Ordering::SeqCst),
                staging_reads: self.staging_reads.load(Ordering::SeqCst),
                staging_removes: self.staging_removes.load(Ordering::SeqCst),
                by_type: self.requests_by_type.lock().expect("the counts").clone(),
                extended: self.extended_by_name.lock().expect("the counts").clone(),
            }
        }

        /// **The line is cut**: every session the machine has open ends at
        /// once, as it does when the machine goes away under a transfer. What
        /// the point does about it is the caller's — a test ending the period
        /// also stops the lending.
        fn every_session_ends(&self) {
            for child in self.children.lock().expect("the children").drain(..) {
                unsafe {
                    libc::kill(child as libc::pid_t, libc::SIGKILL);
                }
            }
        }
    }

    /// **A count of what the machine was asked**, taken before and after an
    /// act so the act's own share can be read. Added 2026-10-16, owed piece 1.
    #[derive(Clone, Default)]
    struct TheWireCount {
        round_trips: usize,
        seats: usize,
        staging_folders: usize,
        staging_reads: usize,
        staging_removes: usize,
        by_type: HashMap<u8, usize>,
        extended: HashMap<String, usize>,
    }

    impl TheWireCount {
        fn since(&self, before: &TheWireCount) -> TheWireCount {
            TheWireCount {
                round_trips: self.round_trips - before.round_trips,
                seats: self.seats - before.seats,
                staging_folders: self.staging_folders - before.staging_folders,
                staging_reads: self.staging_reads - before.staging_reads,
                staging_removes: self.staging_removes - before.staging_removes,
                by_type: the_requests_since(&before.by_type, &self.by_type)
                    .into_iter()
                    .collect(),
                extended: self
                    .extended
                    .iter()
                    .map(|(name, count)| {
                        (
                            name.clone(),
                            count - before.extended.get(name).copied().unwrap_or(0),
                        )
                    })
                    .filter(|(_, count)| *count > 0)
                    .collect(),
            }
        }

        /// **Per file, in words**: round trips, seats, the three staging
        /// asks, and every request type by its name.
        fn per_file(&self, files: usize) -> String {
            let each = |count: usize| count as f64 / files.max(1) as f64;
            let mut by_type: Vec<(u8, usize)> =
                self.by_type.iter().map(|(k, c)| (*k, *c)).collect();
            by_type.sort();
            let named: Vec<String> = by_type
                .iter()
                .map(|(kind, count)| format!("{} {:.2}", the_request_named(*kind), each(*count)))
                .collect();
            let mut extended: Vec<String> = self
                .extended
                .iter()
                .map(|(name, count)| format!("{name} {:.2}", each(*count)))
                .collect();
            extended.sort();
            format!(
                "{:.2} round trips a file ({} in all), {:.2} seats, staging folder asked {:.2}, staged \
                 part read {:.2}, removed {:.2}; requests a file: {}; extended by name: {}",
                each(self.round_trips),
                self.round_trips,
                each(self.seats),
                each(self.staging_folders),
                each(self.staging_reads),
                each(self.staging_removes),
                named.join(", "),
                extended.join(", ")
            )
        }
    }

    /// The file-transfer protocol's request types, by name, for the counts.
    fn the_request_named(kind: u8) -> String {
        match kind {
            1 => "INIT".into(),
            3 => "OPEN".into(),
            4 => "CLOSE".into(),
            5 => "READ".into(),
            6 => "WRITE".into(),
            7 => "LSTAT".into(),
            8 => "FSTAT".into(),
            9 => "SETSTAT".into(),
            10 => "FSETSTAT".into(),
            11 => "OPENDIR".into(),
            12 => "READDIR".into(),
            13 => "REMOVE".into(),
            14 => "MKDIR".into(),
            15 => "RMDIR".into(),
            16 => "REALPATH".into(),
            17 => "STAT".into(),
            18 => "RENAME".into(),
            19 => "READLINK".into(),
            20 => "SYMLINK".into(),
            200 => "EXTENDED".into(),
            other => format!("type {other}"),
        }
    }

    struct AConnectionToIt {
        machine: Arc<AMachine>,
        open: Arc<AtomicUsize>,
    }

    struct ASession {
        stream: tokio::net::UnixStream,
        child: std::process::Child,
        open: Arc<AtomicUsize>,
        /// The machine this session is on, for its request count.
        machine: Arc<AMachine>,
        /// **What was sent and not yet read as a whole packet**, since one
        /// request can cross in more than one write.
        sent: Vec<u8>,
        /// **What came back and not yet read as a whole packet.** Added
        /// 2026-10-16, owed piece 1.
        received: Vec<u8>,
        /// **Requests sent on this session and not yet answered** — a request
        /// sent with none outstanding is a round trip. Added 2026-10-16, owed
        /// piece 1.
        outstanding: usize,
    }

    impl ASession {
        /// **The requests in what was sent, counted** — each a four-byte
        /// length, then its type and the rest.
        fn count_what_was_sent(&mut self, bytes: &[u8]) {
            const STAT: u8 = 17;
            const LSTAT: u8 = 7;
            const MKDIR: u8 = 14;
            const REMOVE: u8 = 13;
            const EXTENDED: u8 = 200;
            self.sent.extend_from_slice(bytes);
            let mut at = 0usize;
            while self.sent.len() >= at + 5 {
                let length =
                    u32::from_be_bytes(self.sent[at..at + 4].try_into().expect("four")) as usize;
                if length == 0 || self.sent.len() < at + 4 + length {
                    break;
                }
                let kind = self.sent[at + 4];
                *self
                    .machine
                    .requests_by_type
                    .lock()
                    .expect("the counts")
                    .entry(kind)
                    .or_default() += 1;
                // **A round trip is a request that waits on nothing already
                // sent** (owed piece 1).
                if self.outstanding == 0 {
                    self.machine.round_trips.fetch_add(1, Ordering::SeqCst);
                }
                self.outstanding += 1;
                if matches!(kind, STAT | LSTAT) {
                    self.machine.attribute_reads.fetch_add(1, Ordering::SeqCst);
                }
                // **An extended request names its extension first**, after its
                // type and id.
                if kind == EXTENDED && length >= 9 {
                    let name_length =
                        u32::from_be_bytes(self.sent[at + 9..at + 13].try_into().expect("four"))
                            as usize;
                    let name = &self.sent[at + 13..(at + 13 + name_length).min(at + 4 + length)];
                    *self
                        .machine
                        .extended_by_name
                        .lock()
                        .expect("the counts")
                        .entry(String::from_utf8_lossy(name).into_owned())
                        .or_default() += 1;
                }
                // **The path these four carry first**: their type, their id,
                // then the path's length and bytes.
                if matches!(kind, STAT | LSTAT | MKDIR | REMOVE) && length >= 9 {
                    let path_length =
                        u32::from_be_bytes(self.sent[at + 9..at + 13].try_into().expect("four"))
                            as usize;
                    let path = &self.sent[at + 13..(at + 13 + path_length).min(at + 4 + length)];
                    let inside_a_staging_folder =
                        path.windows(16).any(|part| part == b".smoozy-staging/");
                    if kind == MKDIR && path.ends_with(b".smoozy-staging") {
                        self.machine
                            .staging_folders_asked
                            .fetch_add(1, Ordering::SeqCst);
                    }
                    if matches!(kind, STAT | LSTAT) && inside_a_staging_folder {
                        self.machine.staging_reads.fetch_add(1, Ordering::SeqCst);
                    }
                    if kind == REMOVE && inside_a_staging_folder {
                        self.machine.staging_removes.fetch_add(1, Ordering::SeqCst);
                    }
                }
                at += 4 + length;
            }
            self.sent.drain(..at);
        }

        /// **The answers in what came back, counted** — each one answers a
        /// request outstanding on this session. Added 2026-10-16, owed piece 1.
        fn count_what_came_back(&mut self, bytes: &[u8]) {
            self.received.extend_from_slice(bytes);
            let mut at = 0usize;
            while self.received.len() >= at + 4 {
                let length = u32::from_be_bytes(self.received[at..at + 4].try_into().expect("four"))
                    as usize;
                if self.received.len() < at + 4 + length {
                    break;
                }
                self.outstanding = self.outstanding.saturating_sub(1);
                at += 4 + length;
            }
            self.received.drain(..at);
        }
    }

    impl Drop for ASession {
        fn drop(&mut self) {
            let _ = self.child.kill();
            let _ = self.child.wait();
            self.open.fetch_sub(1, Ordering::SeqCst);
        }
    }

    impl AsyncRead for ASession {
        fn poll_read(
            mut self: Pin<&mut Self>,
            cx: &mut Context<'_>,
            buf: &mut ReadBuf<'_>,
        ) -> Poll<std::io::Result<()>> {
            let before = buf.filled().len();
            let read = Pin::new(&mut self.stream).poll_read(cx, buf);
            if let Poll::Ready(Ok(())) = read {
                let came = buf.filled()[before..].to_vec();
                self.count_what_came_back(&came);
            }
            read
        }
    }

    impl AsyncWrite for ASession {
        fn poll_write(
            mut self: Pin<&mut Self>,
            cx: &mut Context<'_>,
            buf: &[u8],
        ) -> Poll<std::io::Result<usize>> {
            let wrote = Pin::new(&mut self.stream).poll_write(cx, buf);
            if let Poll::Ready(Ok(written)) = wrote {
                self.count_what_was_sent(&buf[..written]);
            }
            wrote
        }

        fn poll_flush(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<std::io::Result<()>> {
            Pin::new(&mut self.stream).poll_flush(cx)
        }

        fn poll_shutdown(
            mut self: Pin<&mut Self>,
            cx: &mut Context<'_>,
        ) -> Poll<std::io::Result<()>> {
            Pin::new(&mut self.stream).poll_shutdown(cx)
        }
    }

    /// **The desk's end of a session whose every byte reaches the machine a
    /// round trip later** (`AMachine::round_trip`). The machine's end,
    /// `to_the_machine`, is handed in; what the desk drives is handed back.
    /// Everything ends when either end closes.
    fn a_delay_line_to(
        to_the_machine: std::os::unix::net::UnixStream,
        round_trip: Duration,
        answers_a_second: u64,
    ) -> std::os::unix::net::UnixStream {
        use tokio::io::{AsyncReadExt, AsyncWriteExt};
        let (the_desks, the_lines) = std::os::unix::net::UnixStream::pair().expect("a socket pair");
        the_desks.set_nonblocking(true).expect("the end we drive");
        the_lines.set_nonblocking(true).expect("the line's end");
        let the_lines = tokio::net::UnixStream::from_std(the_lines).expect("the line's end");
        let the_machines = tokio::net::UnixStream::from_std(to_the_machine).expect("the machine's end");
        tokio::spawn(async move {
            let (mut from_the_desk, mut to_the_desk) = the_lines.into_split();
            let (mut from_the_machine, mut to_the_machine) = the_machines.into_split();
            let (tell, mut told) = tokio::sync::mpsc::unbounded_channel::<(tokio::time::Instant, Vec<u8>)>();
            let sent = async move {
                let mut read = vec![0u8; 256 * 1024];
                loop {
                    match from_the_desk.read(&mut read).await {
                        Ok(0) | Err(_) => break,
                        Ok(got) => {
                            let due = tokio::time::Instant::now() + round_trip;
                            if tell.send((due, read[..got].to_vec())).is_err() {
                                break;
                            }
                        }
                    }
                }
            };
            let delivered = async move {
                while let Some((due, bytes)) = told.recv().await {
                    tokio::time::sleep_until(due).await;
                    if to_the_machine.write_all(&bytes).await.is_err() {
                        break;
                    }
                }
            };
            let answered = async move {
                if answers_a_second == 0 {
                    let _ = tokio::io::copy(&mut from_the_machine, &mut to_the_desk).await;
                    return;
                }
                // **Paced**: each read of the machine's answers is held back
                // until the session's own line would have carried it.
                let began = tokio::time::Instant::now();
                let mut carried = 0u64;
                let mut read = vec![0u8; 16 * 1024];
                loop {
                    match from_the_machine.read(&mut read).await {
                        Ok(0) | Err(_) => break,
                        Ok(got) => {
                            carried += got as u64;
                            let due = began
                                + Duration::from_secs_f64(carried as f64 / answers_a_second as f64);
                            tokio::time::sleep_until(due).await;
                            if to_the_desk.write_all(&read[..got]).await.is_err() {
                                break;
                            }
                        }
                    }
                }
            };
            tokio::select! {
                () = async { tokio::join!(sent, delivered); } => {}
                () = answered => {}
            }
        });
        the_desks
    }

    fn copy_of(descriptor: std::os::unix::io::RawFd) -> std::os::unix::io::RawFd {
        let copy = unsafe { libc::dup(descriptor) };
        assert!(copy >= 0, "the descriptor could not be copied");
        copy
    }

    impl AHeldConnection for AConnectionToIt {
        type Channel = ASession;

        async fn open_a_session(&mut self) -> Result<ASession, NoSession> {
            use std::os::unix::io::{AsRawFd, FromRawFd};
            // **Held while the test holds opens**, and never for longer than
            // half a minute. A lending that is ended while it waits here drops
            // the wait with it, which is the line going under a transfer that
            // is asking for its channel.
            if self.machine.holds_opens.load(Ordering::SeqCst) {
                self.machine.an_open_is_held.store(true, Ordering::SeqCst);
                let began = Instant::now();
                while self.machine.holds_opens.load(Ordering::SeqCst)
                    && began.elapsed() < Duration::from_secs(30)
                {
                    tokio::time::sleep(Duration::from_millis(10)).await;
                }
            }
            if self.machine.opens_find_the_line_gone.load(Ordering::SeqCst) {
                self.machine
                    .an_open_found_the_line_gone
                    .store(true, Ordering::SeqCst);
                return Err(NoSession::TheConnectionIsGone);
            }
            if self.open.load(Ordering::SeqCst) >= self.machine.limit {
                return Err(NoSession::Refused);
            }
            let (ours, theirs) = std::os::unix::net::UnixStream::pair().expect("a socket pair");
            ours.set_nonblocking(true).expect("the end we drive");
            let reading = unsafe { std::process::Stdio::from_raw_fd(copy_of(theirs.as_raw_fd())) };
            let writing = unsafe { std::process::Stdio::from_raw_fd(copy_of(theirs.as_raw_fd())) };
            let child = std::process::Command::new(&self.machine.program)
                .current_dir(&self.machine.home)
                .stdin(reading)
                .stdout(writing)
                .stderr(std::process::Stdio::null())
                .spawn()
                .expect("the machine's own file-transfer server starts");
            self.machine
                .children
                .lock()
                .expect("the children")
                .push(child.id());
            drop(theirs);
            self.open.fetch_add(1, Ordering::SeqCst);
            let ours = if self.machine.round_trip.is_zero() {
                ours
            } else {
                a_delay_line_to(
                    ours,
                    self.machine.round_trip,
                    self.machine.each_sessions_answers_a_second,
                )
            };
            self.machine.seats.fetch_add(1, Ordering::SeqCst);
            Ok(ASession {
                stream: tokio::net::UnixStream::from_std(ours).expect("the end we drive"),
                child,
                open: Arc::clone(&self.open),
                machine: Arc::clone(&self.machine),
                sent: Vec::new(),
                received: Vec::new(),
                outstanding: 0,
            })
        }

        async fn a_command(
            &mut self,
            command: CommandString,
            _frame: Option<Vec<u8>>,
        ) -> Result<CommandResult, NoSession> {
            // **`Server protocol and transfers` S13's reading, where this
            // machine answers it**: the path after
            // `~/` is under the machine's home, and the link there is followed
            // (`-L`), as the planted carriage needs.
            const THE_READING: &str = "stat -L -c '%s %i %Y %Z' ~/";
            if self.machine.answers_the_four_values.load(Ordering::SeqCst) {
                if let Some(at) = command.as_str().strip_prefix(THE_READING) {
                    use std::os::unix::fs::MetadataExt;
                    if let Ok(found) = std::fs::metadata(self.machine.home.join(at)) {
                        return Ok(CommandResult {
                            output: format!("{} {} {} {}\n", found.len(), found.ino(), found.mtime(), found.ctime())
                                .into_bytes(),
                            complaint: Vec::new(),
                            status: Some(0),
                        });
                    }
                }
            }
            Ok(CommandResult {
                output: Vec::new(),
                complaint: b"no such command here".to_vec(),
                status: Some(127),
            })
        }

        async fn until_it_ends(&mut self) -> Ending {
            std::future::pending().await
        }

        async fn torn_down(self) -> Ending {
            Ending::TornDownBySmoozy
        }
    }

    /// **The point, lending the stand-in's primary connection**, with `Servers`
    /// S14's additional connection made on asking.
    struct APointLending {
        sessions: ThisServersSessions<AConnectionToIt>,
        lending: tokio::task::JoinHandle<Ending>,
    }

    impl APointLending {
        async fn over(machine: &Arc<AMachine>) -> APointLending {
            let sessions = ThisServersSessions::<AConnectionToIt>::for_one_server();
            let lending = APointLending::lending_on(machine, &sessions);
            while !sessions.a_connection_is_live() {
                tokio::task::yield_now().await;
            }
            APointLending { sessions, lending }
        }

        /// **`Servers`' repair, as the point meets it**: a new primary
        /// connection lent through **the same point**, which is the period's
        /// and survives a replacement. Added 2026-09-29, step 10c's transfer
        /// wave piece 6. The caller waits for it to lend where it needs to.
        fn the_line_comes_back(&mut self, machine: &Arc<AMachine>) {
            self.lending = APointLending::lending_on(machine, &self.sessions);
        }

        /// **The line is cut and the connection with it** — the lending ends,
        /// so the point lends nothing, and every session the machine had open
        /// ends. **The period is not ended by this**: a repair can follow.
        fn the_line_is_cut(&self, machine: &AMachine) {
            self.lending.abort();
            machine.every_session_ends();
        }

        fn lending_on(
            machine: &Arc<AMachine>,
            sessions: &ThisServersSessions<AConnectionToIt>,
        ) -> tokio::task::JoinHandle<Ending> {
            let more = Arc::clone(machine);
            let additional: TheAdditionalConnection<AConnectionToIt> = Arc::new(move || {
                let more = Arc::clone(&more);
                Box::pin(async move {
                    Some(AConnectionToIt {
                        machine: more,
                        open: Arc::new(AtomicUsize::new(0)),
                    })
                })
            });
            let mut primary = AConnectionToIt {
                machine: Arc::clone(machine),
                open: Arc::new(AtomicUsize::new(0)),
            };
            tokio::spawn({
                let sessions = sessions.clone();
                async move { lend_until_it_ends(&sessions, &mut primary, &additional).await }
            })
        }
    }

    impl Drop for APointLending {
        fn drop(&mut self) {
            self.lending.abort();
        }
    }

    // ------------------------------------------------------------- the record

    /// **One hold of the record's lock**: when it began, how long the caller
    /// waited for it, and how long it was held.
    #[derive(Clone, Copy)]
    struct AHold {
        at: Duration,
        waited: Duration,
        held: Duration,
        /// **Rows the act changed**, by the engine's own count — nought for a
        /// reading, and what a save wrote otherwise.
        changed: u64,
    }

    /// **The application's own record, behind one lock, every hold timed.**
    /// It is both doors — an upload's and a download's — because the running
    /// application's one record is both.
    struct TheTimedRecord {
        record: Mutex<LocalRecord>,
        began: Instant,
        holds: Mutex<Vec<AHold>>,
        /// When the page would have been told something moved.
        moved: Mutex<Vec<Duration>>,
        /// **When the page would have been told bytes moved inside a file**,
        /// kept apart from `moved` so the thousands below still count files
        /// (step 10c's transfer wave piece 5).
        moving: Mutex<Vec<Duration>>,
    }

    impl TheTimedRecord {
        fn at(path: &Path) -> Arc<TheTimedRecord> {
            Arc::new(TheTimedRecord {
                record: Mutex::new(LocalRecord::open_at(path).expect("the record opens")),
                began: Instant::now(),
                holds: Mutex::new(Vec::new()),
                moved: Mutex::new(Vec::new()),
                moving: Mutex::new(Vec::new()),
            })
        }

        fn held(&self, act: &mut dyn FnMut(&rusqlite::Connection)) -> bool {
            let asked = Instant::now();
            let Ok(record) = self.record.lock() else {
                return false;
            };
            let got = Instant::now();
            let before = record.connection().total_changes();
            act(record.connection());
            let held = got.elapsed();
            let changed = record.connection().total_changes() - before;
            drop(record);
            self.holds.lock().expect("the holds").push(AHold {
                at: got - self.began,
                waited: got - asked,
                held,
                changed,
            });
            true
        }

        fn the_holds_since(&self, from: Duration) -> Vec<AHold> {
            self.holds
                .lock()
                .expect("the holds")
                .iter()
                .filter(|hold| hold.at >= from)
                .copied()
                .collect()
        }

        fn now(&self) -> Duration {
            self.began.elapsed()
        }

        /// **How many times the page would have been told something moved.**
        fn told(&self) -> usize {
            self.moved.lock().expect("the moments").len()
                + self.moving.lock().expect("the moments").len()
        }
    }

    impl TheRecordAnUploadWrites for TheTimedRecord {
        fn with_the_record(&self, act: &mut dyn FnMut(&rusqlite::Connection)) -> bool {
            self.held(act)
        }
        fn a_transfer_moved(&self) {
            self.moved.lock().expect("the moments").push(self.now());
        }
        fn a_transfer_is_moving(&self) {
            self.moving.lock().expect("the moments").push(self.now());
        }
    }

    impl TheRecordADownloadWrites for TheTimedRecord {
        fn with_the_record(&self, act: &mut dyn FnMut(&rusqlite::Connection)) -> bool {
            self.held(act)
        }
        fn a_transfer_moved(&self) {
            self.moved.lock().expect("the moments").push(self.now());
        }
        fn a_transfer_is_moving(&self) {
            self.moving.lock().expect("the moments").push(self.now());
        }
    }

    /// **A server row in the record for the stand-in**, so the Transfers
    /// reading finds the requests the harness makes — the screen reads the
    /// requests of the rows it knows and of no other.
    fn a_server_row_for(record: &TheTimedRecord) -> Identity {
        let row = Identity::issue(IdentityKind::LocalServer).expect("random bytes");
        let written = ServerRow {
            local_server_identifier: row.as_text(),
            host: Host::what_the_user_typed("198.51.100.10").expect("a host"),
            port: Port::STANDARD_SSH,
            username: Username::what_the_user_typed("root").expect("a username"),
            key: KeyReference {
                local_key_identifier: "key-1".into(),
                fingerprint: "SHA256:aaa".into(),
            },
            label: None,
            colour: None,
            remembered_setup: None,
            remembered_host_key: None,
            remembered_hardware_reading: None,
            recorded_machine_root: None,
            last_confirmed_installation_identity: None,
            event_cursor: None,
            machine_password_subject: None,
            chosen_fallback_folder: None,
            the_driver_missed_the_optix_bar: false,
        };
        let record = record.record.lock().expect("the record");
        server_rows::add(record.connection(), &written).expect("the row is added");
        row
    }

    // ------------------------------------------------- the Transfers reading

    /// **The reading the Transfers screen takes, through the same lock, as
    /// the page takes it**: again whenever it is told a transfer moved, one
    /// ask at a time (`Desktop app and delivery` P7 and P8 — the page asks
    /// again at every file's end, and never while an ask is still out), and
    /// at least once a second besides, as a person looking at it would. Each
    /// is timed from the moment it asked for the lock to the moment the screen
    /// was composed.
    ///
    /// **Each reading also keeps what the request's row showed** — when it
    /// was taken, the bar's value and the count — so a run can say whether
    /// the row moved while the file did (step 10c's transfer wave piece 5).
    struct TheTransfersReadings {
        stop: Arc<AtomicBool>,
        taking: Option<std::thread::JoinHandle<Vec<AReading>>>,
    }

    /// How long a reading took, how much of that was composing, whether it
    /// showed the request, when it was taken, and the row's bar and count.
    type AReading = (Duration, Duration, bool, Duration, Option<u64>, String);

    /// **The row as the readings saw it**, in the shape [`the_row_moved`]
    /// reads.
    fn the_rows_seen(readings: &[AReading]) -> Vec<(Duration, Option<u64>, String)> {
        readings
            .iter()
            .filter(|reading| reading.2)
            .map(|(_, _, _, at, bar, count)| (*at, *bar, count.clone()))
            .collect()
    }

    impl TheTransfersReadings {
        fn taken_over(
            record: &Arc<TheTimedRecord>,
            the_run: &Arc<TheRunsTransfers>,
            looking_for: String,
        ) -> TheTransfersReadings {
            let stop = Arc::new(AtomicBool::new(false));
            let record = Arc::clone(record);
            let the_run = Arc::clone(the_run);
            let stopping = Arc::clone(&stop);
            let taking = std::thread::spawn(move || {
                let mut readings = Vec::new();
                let began = Instant::now();
                let mut told = record.told();
                let mut last = Instant::now();
                while !stopping.load(Ordering::SeqCst) {
                    std::thread::sleep(Duration::from_millis(10));
                    let now_told = record.told();
                    if now_told == told && last.elapsed() < Duration::from_secs(1) {
                        continue;
                    }
                    told = now_told;
                    last = Instant::now();
                    let asked = Instant::now();
                    let (screen, composed_in) = {
                        let held = record.record.lock().expect("the record");
                        let got = Instant::now();
                        (the_screen_answered(&held, &the_run), got.elapsed())
                    };
                    let took = asked.elapsed();
                    let row = the_row(&screen, &looking_for);
                    readings.push((
                        took,
                        composed_in,
                        row.is_some(),
                        began.elapsed(),
                        row.and_then(|row| row.bar.as_ref().map(|bar| bar.value)),
                        row.map(|row| as_read(&row.count)).unwrap_or_default(),
                    ));
                }
                readings
            });
            TheTransfersReadings {
                stop,
                taking: Some(taking),
            }
        }

        fn ended(mut self) -> Vec<AReading> {
            self.stop.store(true, Ordering::SeqCst);
            self.taking
                .take()
                .map(|taking| taking.join().expect("the readings"))
                .unwrap_or_default()
        }
    }

    impl Drop for TheTransfersReadings {
        fn drop(&mut self) {
            self.stop.store(true, Ordering::SeqCst);
        }
    }

    // --------------------------------------------------------- the figures

    fn the_median(of: &mut [Duration]) -> Duration {
        if of.is_empty() {
            return Duration::ZERO;
        }
        of.sort_unstable();
        of[of.len() / 2]
    }

    /// **What a run of the record's holds came to**, printed and returned.
    struct TheHoldsCameTo {
        longest: Duration,
        the_first_steps: Duration,
        the_last_steps: Duration,
        the_first_saves: Duration,
        the_last_saves: Duration,
    }

    /// **The step is the time between two holds, the save is a hold**, each
    /// taken as the median over the first and the last tenth of the run so a
    /// stray one does not decide it.
    fn the_holds_came_to(said: &str, holds: &[AHold]) -> TheHoldsCameTo {
        let longest = holds.iter().map(|hold| hold.held).max().unwrap_or_default();
        let held: Duration = holds.iter().map(|hold| hold.held).sum();
        let tenth = (holds.len() / 10).max(1);
        let gaps: Vec<Duration> = holds
            .windows(2)
            .map(|two| two[1].at.saturating_sub(two[0].at))
            .collect();
        let the_first_steps = the_median(&mut gaps.iter().take(tenth).copied().collect::<Vec<_>>());
        let the_last_steps = the_median(
            &mut gaps
                .iter()
                .rev()
                .skip(3)
                .take(tenth)
                .copied()
                .collect::<Vec<_>>(),
        );
        let the_first_saves =
            the_median(&mut holds.iter().take(tenth).map(|hold| hold.held).collect::<Vec<_>>());
        let the_last_saves = the_median(
            &mut holds
                .iter()
                .rev()
                .skip(3)
                .take(tenth)
                .map(|hold| hold.held)
                .collect::<Vec<_>>(),
        );
        let waited = holds.iter().map(|hold| hold.waited).max().unwrap_or_default();
        // **The longest few, with when each fell**, so a figure that misses
        // says whether it was one stray flush or a cost that grows.
        let mut longest_few: Vec<AHold> = holds.to_vec();
        longest_few.sort_by(|one, two| two.held.cmp(&one.held));
        longest_few.truncate(5);
        let over = |bound: u64| {
            holds
                .iter()
                .filter(|hold| hold.held > Duration::from_millis(bound))
                .count()
        };
        println!(
            "{said}: holds over 50 ms: {}, over 100 ms: {}; the longest five, each at its moment in \
             the run: {}.",
            over(50),
            over(100),
            longest_few
                .iter()
                .map(|hold| format!(
                    "{:.0} ms at {:.1} s writing {} rows",
                    hold.held.as_secs_f64() * 1e3,
                    hold.at.as_secs_f64(),
                    hold.changed
                ))
                .collect::<Vec<_>>()
                .join(", ")
        );
        println!(
            "{said}: {} holds of the record's lock, {:.1} s held in all; the longest {:.3} s; \
             the longest wait for it {:.3} s. A step (hold to hold) at the start {:.1} ms, at \
             the end {:.1} ms; a save at the start {:.1} ms, at the end {:.1} ms.",
            holds.len(),
            held.as_secs_f64(),
            longest.as_secs_f64(),
            waited.as_secs_f64(),
            the_first_steps.as_secs_f64() * 1e3,
            the_last_steps.as_secs_f64() * 1e3,
            the_first_saves.as_secs_f64() * 1e3,
            the_last_saves.as_secs_f64() * 1e3,
        );
        TheHoldsCameTo {
            longest,
            the_first_steps,
            the_last_steps,
            the_first_saves,
            the_last_saves,
        }
    }

    /// **The slowest reading, from asking for the lock to the screen composed**
    /// — and, apart, the slowest composition once the lock was had, so a slow
    /// answer says whether it waited or worked.
    fn the_readings_came_to(said: &str, readings: &[AReading]) -> Duration {
        let slowest = readings.iter().map(|reading| reading.0).max().unwrap_or_default();
        let slowest_work = readings.iter().map(|reading| reading.1).max().unwrap_or_default();
        let over = readings
            .iter()
            .filter(|reading| reading.0 > Duration::from_millis(100))
            .count();
        let shown = readings.iter().filter(|reading| reading.2).count();
        println!(
            "{said}: {} Transfers readings taken while it ran, the slowest {:.3} s ({:.3} s of \
             it composing once the lock was had), {over} over 0.1 s; {} of them showed the \
             request.",
            readings.len(),
            slowest.as_secs_f64(),
            slowest_work.as_secs_f64(),
            shown
        );
        slowest
    }

    /// **The last thousand against the first thousand**, from the moments the
    /// page would have been told a file moved — and **how much of each the
    /// record's lock was held for**, so a slowing says whether it is the
    /// record's or somewhere else's.
    fn the_thousands(said: &str, moments: &[Duration], holds: &[AHold]) -> (Duration, Duration) {
        let per = 1_000.min(moments.len() / 2).max(1);
        if moments.len() < 2 * per || moments.len() < 2 {
            return (Duration::ZERO, Duration::ZERO);
        }
        let held_between = |from: Duration, to: Duration| -> Duration {
            holds
                .iter()
                .filter(|hold| hold.at >= from && hold.at < to)
                .map(|hold| hold.held)
                .sum()
        };
        let (first_from, first_to) = (moments[0], moments[per - 1]);
        let (last_from, last_to) = (moments[moments.len() - per], moments[moments.len() - 1]);
        let first = first_to.saturating_sub(first_from);
        let last = last_to.saturating_sub(last_from);
        println!(
            "{said}: the first {per} took {:.1} s ({:.1} s of it holding the record), the last \
             {per} took {:.1} s ({:.1} s of it holding the record).",
            first.as_secs_f64(),
            held_between(first_from, first_to).as_secs_f64(),
            last.as_secs_f64(),
            held_between(last_from, last_to).as_secs_f64(),
        );
        (first, last)
    }

    // ---------------------------------------------------- the local job records

    /// **`Download` J9's reading, off the local job records alone**, and there
    /// is no job here.
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

    struct NobodyIsTold;

    impl WhenAFileIsPublished for NobodyIsTold {
        fn a_file_was_published(&self, _landing_path: &[u8], outcome: &UnitOutcome) {
            assert_eq!(outcome, &UnitOutcome::Done);
        }
    }

    fn a_run() -> Identity {
        Identity::issue(IdentityKind::Run).expect("random bytes")
    }

    // ------------------------------------------------ the speed beside the cap

    /// **The speed beside the cap, and the bytes the row says moved**, taken
    /// together every quarter of a second: when, the reading the screen
    /// draws (`Desktop app and delivery` R15, over `Server protocol and
    /// transfers` R12a's trailing three seconds), and the bytes staged of the
    /// one file in flight as its byte loop last reported them (`Server protocol
    /// and transfers` G4). Step 10c's transfer wave piece 5, the map's third
    /// desk check.
    struct TheSpeedWatched {
        stop: Arc<AtomicBool>,
        taking: Option<std::thread::JoinHandle<Vec<(Duration, Option<u64>, u64)>>>,
    }

    impl TheSpeedWatched {
        fn over(the_run: &Arc<TheRunsTransfers>, direction: Direction, request: String) -> TheSpeedWatched {
            let stop = Arc::new(AtomicBool::new(false));
            let (the_run, stopping) = (Arc::clone(the_run), Arc::clone(&stop));
            let taking = std::thread::spawn(move || {
                let began = Instant::now();
                let mut seen = Vec::new();
                while !stopping.load(Ordering::SeqCst) {
                    let reading = the_run.the_reading(direction).map(|reading| reading.value().0);
                    let staged = the_run
                        .what_is_under_way()
                        .get(&request)
                        .and_then(|under_way| under_way.moving.first().map(|file| file.report.bytes_staged))
                        .unwrap_or(0);
                    seen.push((began.elapsed(), reading, staged));
                    std::thread::sleep(Duration::from_millis(250));
                }
                seen
            });
            TheSpeedWatched {
                stop,
                taking: Some(taking),
            }
        }

        fn ended(mut self) -> Vec<(Duration, Option<u64>, u64)> {
            self.stop.store(true, Ordering::SeqCst);
            self.taking
                .take()
                .map(|taking| taking.join().expect("the speeds"))
                .unwrap_or_default()
        }
    }

    impl Drop for TheSpeedWatched {
        fn drop(&mut self) {
            self.stop.store(true, Ordering::SeqCst);
        }
    }

    /// **Whether the speed beside the cap agreed with the bytes moved**, over
    /// every minute of the steady stretch (the whole stretch where it is
    /// shorter than a minute): the bytes moved across the minute, divided by
    /// it, have to fall between the smallest and the largest reading taken
    /// inside it — the spread of the three-second window itself. The first
    /// three seconds are left out, since the window is still filling there.
    fn the_speed_agreed(said: &str, seen: &[(Duration, Option<u64>, u64)]) -> Vec<String> {
        let mut missed = Vec::new();
        let steady: Vec<(Duration, u64, u64)> = seen
            .iter()
            .filter_map(|(at, reading, staged)| reading.filter(|_| *staged > 0).map(|reading| (*at, reading, *staged)))
            .collect();
        let Some(first) = steady.first().map(|one| one.0) else {
            missed.push(format!("{said}: no speed was read while the file moved"));
            return missed;
        };
        let steady: Vec<(Duration, u64, u64)> = steady
            .into_iter()
            .filter(|one| one.0 >= first + Duration::from_secs(3))
            .collect();
        if steady.len() < 8 {
            println!("{said}: too short a steady stretch to read the speed against ({} readings).", steady.len());
            return missed;
        }
        let stretch = steady[steady.len() - 1].0 - steady[0].0;
        let window = stretch.min(Duration::from_secs(60));
        let mut windows = 0;
        let mut disagreed = 0;
        let mut worst = 1.0f64;
        for (at, end) in steady.iter().enumerate() {
            let Some(start) = steady[..at].iter().rev().find(|one| end.0 - one.0 >= window) else {
                continue;
            };
            let span = (end.0 - start.0).as_secs_f64();
            let moved = (end.2 - start.2) as f64 / span;
            let inside: Vec<u64> = steady
                .iter()
                .filter(|one| one.0 > start.0 && one.0 <= end.0)
                .map(|one| one.1)
                .collect();
            let (low, high) = (
                *inside.iter().min().unwrap_or(&0) as f64,
                *inside.iter().max().unwrap_or(&0) as f64,
            );
            let mean = inside.iter().sum::<u64>() as f64 / inside.len().max(1) as f64;
            windows += 1;
            if moved < low || moved > high {
                disagreed += 1;
            }
            worst = if (mean / moved - 1.0).abs() > (worst - 1.0).abs() { mean / moved } else { worst };
            if windows == 1 {
                println!(
                    "{said}: over {:.0} s from {:.1} s: {:.2} MB/s moved; the readings {:.2} to \
                     {:.2} MB/s, {:.2} MB/s on average.",
                    span,
                    start.0.as_secs_f64(),
                    moved / 1e6,
                    low / 1e6,
                    high / 1e6,
                    mean / 1e6
                );
            }
        }
        println!(
            "{said}: the speed read against the bytes moved over {windows} windows of {:.0} s: \
             {disagreed} outside the readings' own spread; the average reading at worst {:.3}× \
             the bytes moved.",
            window.as_secs_f64(),
            worst
        );
        a_figure!(
            missed,
            disagreed == 0,
            "{said}: the speed beside the cap and the bytes moved disagreed in {disagreed} of \
             {windows} windows"
        );
        missed
    }

    // ---------------------------------------------------------------- the runs

    /// **One large file up through Files, then down again by hand**, each
    /// timed, with every 4 MB step's cost read off the record's holds.
    ///
    /// **The figures**: about a minute and a half each way for 4 GB, and over
    /// three minutes means the cost is not all found; every step at the end
    /// costs what one at the start did; no hold of the lock over 0.1 s; a
    /// Transfers reading taken during the run answers within 0.1 s.
    ///
    /// **And, since step 10c's transfer wave piece 5**: the row moves at least
    /// once a second while the file does and ends at the whole, and the speed
    /// beside the cap agrees with the bytes moved over a minute (or over the
    /// whole transfer, where it is shorter) to within the spread of the
    /// readings themselves.
    #[tokio::test(flavor = "multi_thread", worker_threads = 4)]
    #[ignore = "the kept harness: minutes to hours, and gigabytes of disk; run on purpose"]
    async fn one_large_file_goes_up_and_comes_down() {
        let _deadline = TheTestsDeadline::started();
        let program = the_server_or_say_so!();

        let size = a_number_from("SMOOZY_HARNESS_BYTES", 400_000_000);
        let the_bound = Duration::from_secs_f64((180.0 * size as f64 / FOUR_GB).max(20.0));

        let scratch = Scratch::new("large");
        let here = scratch.folder("here");
        let machine_side = scratch.folder("machine");
        let chosen = scratch.folder("chosen");
        let source = here.join("large.blend");
        a_large_file_at(&source, size);

        let record = TheTimedRecord::at(&scratch.folder("record").join("smoozy.sqlite3"));
        let server_row = a_server_row_for(&record);
        let machine = Arc::new(AMachine {
            program,
            home: scratch.at.clone(),
            limit: 10,
            children: Mutex::new(Vec::new()),
            ..AMachine::default()
        });
        let point = APointLending::over(&machine).await;
        let the_run = Arc::new(TheRunsTransfers::for_this_run());
        let capped = a_number_from("SMOOZY_HARNESS_CAP", 0);
        if capped > 0 {
            let cap = TheCap::Of(BytesPerSecond(capped));
            the_run.the_caps_saved_are(cap, cap);
        }
        let the_bound = if capped > 0 {
            the_bound.max(Duration::from_secs_f64(size as f64 / capped as f64 * 1.5))
        } else {
            the_bound
        };

        // ---- up
        let asked = WhatIsBeingUploaded {
            server_row: server_row.clone(),
            installation: Some(Identity::issue(IdentityKind::Installation).expect("random")),
            destination: the_bytes_of(&machine_side),
            chosen: vec![source.clone()],
            run: a_run(),
        };
        let as_a_door: Arc<dyn TheRecordAnUploadWrites> = record.clone();
        let mut ready = an_upload_is_prepared(&asked, &point.sessions, as_a_door.as_ref())
            .await
            .expect("the upload is prepared");
        assert_eq!(
            the_answer_is_applied(&mut ready, None, &point.sessions, as_a_door.as_ref()).await,
            WhatTheAnswerLeft::ItGoes
        );
        let speeds = TheSpeedWatched::over(&the_run, Direction::Up, ready.request.record.as_text());
        let readings = TheTransfersReadings::taken_over(&record, &the_run, ready.request.record.as_text());
        let from = record.now();
        let began = Instant::now();
        let came_to = the_upload_runs(
            ready,
            &point.sessions,
            &the_run,
            Arc::clone(&as_a_door),
            &NobodyIsTold,
        )
        .await;
        let up = began.elapsed();
        let up_readings = readings.ended();
        let up_speeds = speeds.ended();
        assert_eq!(came_to.uploaded, 1, "{came_to:?}");
        let landed = machine_side.join("large.blend");
        println!(
            "UP: {size} bytes in {:.1} s ({:.1} MB/s).",
            up.as_secs_f64(),
            size as f64 / up.as_secs_f64() / 1e6
        );
        let up_holds = the_holds_came_to("UP", &record.the_holds_since(from));
        let up_slowest = the_readings_came_to("UP", &up_readings);
        // **Step 10c's transfer wave piece 5: the row moves at least once a
        // second while the file does, and ends at the whole.**
        // **Held to once a second only where no cap is set**: under a cap the
        // bytes themselves go in bursts (see [`the_row_moved`]).
        let still = (capped == 0).then(|| Duration::from_secs(1));
        let mut the_rows_missed = the_row_moved("UP", &the_rows_seen(&up_readings), size, still);
        assert!(it_holds_the_pattern(&landed, size), "the upload landed whole");
        // **The source is let go of before the download**, so the run never
        // holds more than two copies of the file at once.
        let _ = std::fs::remove_file(&source);

        // ---- down
        let asked = WhatIsBeingDownloaded {
            server_row: server_row.clone(),
            installation: None,
            destination: the_bytes_of(&chosen),
            picked: vec![the_bytes_of(&landed)],
            run: a_run(),
        };
        let mut ready = a_download_is_prepared(
            asked,
            &point.sessions,
            record.as_ref() as &dyn TheRecordADownloadWrites,
            &NoJobIsKnown,
        )
        .await
        .expect("the download is prepared");
        the_request_goes_without_a_question(&mut ready, record.as_ref())
            .expect("the dispositions are written");
        let speeds = TheSpeedWatched::over(&the_run, Direction::Down, ready.request.record.as_text());
        let readings = TheTransfersReadings::taken_over(&record, &the_run, ready.request.record.as_text());
        let from = record.now();
        let began = Instant::now();
        let came_to = the_download_runs(&mut ready, &point.sessions, &the_run, record.as_ref()).await;
        let down = began.elapsed();
        let down_readings = readings.ended();
        let down_speeds = speeds.ended();
        assert_eq!(came_to.fetched, 1, "{came_to:?}");
        println!(
            "DOWN: {size} bytes in {:.1} s ({:.1} MB/s).",
            down.as_secs_f64(),
            size as f64 / down.as_secs_f64() / 1e6
        );
        let down_holds = the_holds_came_to("DOWN", &record.the_holds_since(from));
        let down_slowest = the_readings_came_to("DOWN", &down_readings);
        the_rows_missed.extend(the_row_moved(
            "DOWN",
            &the_rows_seen(&down_readings),
            size,
            still,
        ));
        assert!(
            it_holds_the_pattern(&chosen.join("large.blend"), size),
            "the download landed whole"
        );

        // ---- the figures, every one said before any miss fails the run
        let mut missed = the_rows_missed;
        missed.extend(the_speed_agreed("UP", &up_speeds));
        missed.extend(the_speed_agreed("DOWN", &down_speeds));
        for (way, took, holds, slowest) in [
            ("up", up, &up_holds, up_slowest),
            ("down", down, &down_holds, down_slowest),
        ] {
            a_figure!(
                missed,
                took <= the_bound,
                "{way}: {:.1} s, over the {:.1} s this size is owed",
                took.as_secs_f64(),
                the_bound.as_secs_f64()
            );
            a_figure!(
                missed,
                holds.longest <= Duration::from_millis(100),
                "{way}: a hold of the record's lock of {:.3} s",
                holds.longest.as_secs_f64()
            );
            // **The last step costs what the first did**: within half again,
            // and never more than 20 ms apart, which is noise at this size.
            for (first, last, what) in [
                (holds.the_first_steps, holds.the_last_steps, "step"),
                (holds.the_first_saves, holds.the_last_saves, "save"),
            ] {
                a_figure!(
                missed,
                    last <= first.mul_f64(1.5) + Duration::from_millis(20),
                    "{way}: a {what} at the end cost {:.1} ms against {:.1} ms at the start",
                    last.as_secs_f64() * 1e3,
                    first.as_secs_f64() * 1e3
                );
            }
            a_figure!(
                missed,
                slowest <= Duration::from_millis(100),
                "{way}: a Transfers reading took {:.3} s",
                slowest.as_secs_f64()
            );
        }
        the_figures_held("ONE LARGE FILE", missed);
    }

    /// **What one small file costs the record: its transfer record's own saves
    /// and nothing more, each writing a handful of rows.** Not ignored — it
    /// takes seconds, and it is the desk's own guard on the shape the figures
    /// below stand on.
    ///
    /// **A file's transfer record is put down six times** by `Server protocol
    /// and transfers` §7's loop — before the first byte (`Server protocol and
    /// transfers` S8), again as `Server protocol and transfers` S12 step 1
    /// clears a list that holds nothing yet, its staging identity, its one
    /// checkpoint, the publication's pre-state (`Server protocol and
    /// transfers` S15) and its ending (`Server protocol and transfers` C23).
    /// **The second changes nothing, so it writes nothing**, and **the request
    /// entry rides two of the others** (see `TheTransferRecords` in
    /// `src/uploading.rs`): five saves. **What it bites on**: the request
    /// written whole at a file's two moments (every entry, every time), the
    /// entry written in two saves of its own, and a save that moved nothing
    /// flushing the disk all the same.
    #[tokio::test(flavor = "multi_thread", worker_threads = 4)]
    async fn a_small_file_costs_its_transfer_records_saves_and_no_more() {
        let _deadline = TheTestsDeadline::started();
        let program = the_server_or_say_so!();

        let count = 40usize;

        let scratch = Scratch::new("small");
        let here = scratch.folder("here/few");
        for at in 0..count {
            let mut bytes = vec![0u8; 1_024 + at * 97];
            the_pattern_at(at as u64 * 8, &mut bytes);
            std::fs::write(here.join(format!("file_{at:02}.bin")), &bytes).expect("written");
        }
        let machine_side = scratch.folder("machine");
        let record = TheTimedRecord::at(&scratch.folder("record").join("smoozy.sqlite3"));
        let server_row = a_server_row_for(&record);
        let machine = Arc::new(AMachine {
            program,
            home: scratch.at.clone(),
            limit: 10,
            children: Mutex::new(Vec::new()),
            ..AMachine::default()
        });
        let point = APointLending::over(&machine).await;
        let the_run = Arc::new(TheRunsTransfers::for_this_run());
        let asked = WhatIsBeingUploaded {
            server_row,
            installation: Some(Identity::issue(IdentityKind::Installation).expect("random")),
            destination: the_bytes_of(&machine_side),
            chosen: vec![here.clone()],
            run: a_run(),
        };
        let as_a_door: Arc<dyn TheRecordAnUploadWrites> = record.clone();
        let mut ready = an_upload_is_prepared(&asked, &point.sessions, as_a_door.as_ref())
            .await
            .expect("the upload is prepared");
        assert_eq!(
            the_answer_is_applied(&mut ready, None, &point.sessions, as_a_door.as_ref()).await,
            WhatTheAnswerLeft::ItGoes
        );
        let request = ready.request.record.clone();
        let from = record.now();
        let came_to = the_upload_runs(
            ready,
            &point.sessions,
            &the_run,
            Arc::clone(&as_a_door),
            &NobodyIsTold,
        )
        .await;
        assert_eq!(came_to.uploaded, count, "{came_to:?}");

        let saves: Vec<AHold> = record
            .the_holds_since(from)
            .into_iter()
            .filter(|hold| hold.changed > 0)
            .collect();
        let most = saves.iter().map(|hold| hold.changed).max().unwrap_or(0);
        assert!(
            saves.len() <= 5 * count,
            "{} saves for {count} small files: more than five each",
            saves.len()
        );
        assert!(most <= 6, "one save wrote {most} rows");

        // **And every unit is done, with its moment** (`File Browser` U1 and
        // `File Browser` U9a), which is what riding the transfer record's
        // saves must not lose.
        let mut kept = Vec::new();
        let door: &dyn TheRecordAnUploadWrites = record.as_ref();
        door.with_the_record(&mut |connection| {
            use smoozy_file_browser::request::KeepsTheRequest as _;
            kept = smoozy_desktop::record::upload_requests::UploadRequests::over(connection)
                .every_request_for(&asked.server_row)
                .expect("the requests read back");
        });
        let kept = kept
            .into_iter()
            .find(|one| one.record == request)
            .expect("the request is in the record");
        assert!(kept.entries.iter().all(|entry| entry.outcome == Some(UnitOutcome::Done)
            && entry.outcome_written_at.is_some()
            && entry.transfer.is_some()));
    }

    // ------------------------------------------- what the row says is moving

    /// **The Transfers screen as the page asks for it now**, through the
    /// record's own lock. Added 2026-09-29, step 10c's transfer wave piece 5.
    fn the_screen_now(record: &TheTimedRecord, the_run: &TheRunsTransfers) -> TheTransfersScreen {
        let held = record.record.lock().expect("the record");
        the_screen_answered(&held, the_run)
    }

    /// **The screen the command composes**, from the record held and the
    /// run's own transfers.
    fn the_screen_answered(record: &LocalRecord, the_run: &TheRunsTransfers) -> TheTransfersScreen {
        the_screen_the_record_answers(Some(record), the_run)
    }

    fn the_row<'s>(screen: &'s TheTransfersScreen, request: &str) -> Option<&'s ARequestDrawn> {
        screen.requests.iter().find(|row| row.id.as_str() == request)
    }

    /// **Every word and value of a composed line, in order**, as the page
    /// would lay them out.
    fn as_read(said: &Words) -> String {
        said.pieces
            .iter()
            .filter_map(|piece| {
                piece
                    .said
                    .as_ref()
                    .map(|words| words.as_str().to_string())
                    .or_else(|| piece.value.as_ref().map(|value| value.as_str().to_string()))
            })
            .collect::<Vec<_>>()
            .join(" ")
            // **The typography point makes the last space of a piece of prose
            // a non-breaking one**, so words are compared with it taken out.
            .replace('\u{a0}', " ")
    }

    /// **What a person looking at one request's row saw**, one look every
    /// tenth of a second: when, the bar's value where a bar was drawn, and the
    /// count.
    struct TheRowWatched {
        stop: Arc<AtomicBool>,
        looking: Option<std::thread::JoinHandle<Vec<(Duration, Option<u64>, String)>>>,
    }

    impl TheRowWatched {
        fn over(
            record: &Arc<TheTimedRecord>,
            the_run: &Arc<TheRunsTransfers>,
            request: String,
        ) -> TheRowWatched {
            let stop = Arc::new(AtomicBool::new(false));
            let (record, the_run, stopping) =
                (Arc::clone(record), Arc::clone(the_run), Arc::clone(&stop));
            let looking = std::thread::spawn(move || {
                let began = Instant::now();
                let mut seen = Vec::new();
                while !stopping.load(Ordering::SeqCst) {
                    let screen = the_screen_now(&record, &the_run);
                    if let Some(row) = the_row(&screen, &request) {
                        seen.push((
                            began.elapsed(),
                            row.bar.as_ref().map(|bar| bar.value),
                            as_read(&row.count),
                        ));
                    }
                    std::thread::sleep(Duration::from_millis(100));
                }
                seen
            });
            TheRowWatched {
                stop,
                looking: Some(looking),
            }
        }

        fn ended(mut self) -> Vec<(Duration, Option<u64>, String)> {
            self.stop.store(true, Ordering::SeqCst);
            self.looking
                .take()
                .map(|looking| looking.join().expect("the looks"))
                .unwrap_or_default()
        }
    }

    impl Drop for TheRowWatched {
        fn drop(&mut self) {
            self.stop.store(true, Ordering::SeqCst);
        }
    }

    /// **The row moved while the file moved**: it was seen inside the file at
    /// least three times, never went back, and once the file was across it
    /// read *done* — and, where `still` is given, it never stood still for
    /// longer than that while the file moved.
    ///
    /// **`still` is not held under a cap**, and that is the enforcement's and
    /// not the row's: `src/scheduling.rs` lets a channel draw whatever the
    /// trailing three-second window has room for, so at a cap the bytes go in
    /// a burst of up to three seconds' worth and then truly stand still until
    /// that burst leaves the window. The row shows exactly that.
    fn the_row_moved(
        said: &str,
        seen: &[(Duration, Option<u64>, String)],
        whole: u64,
        still: Option<Duration>,
    ) -> Vec<String> {
        let mut missed = Vec::new();
        let inside: Vec<(Duration, u64)> = seen
            .iter()
            .filter_map(|(at, bar, _)| bar.filter(|value| *value > 0 && *value < whole).map(|value| (*at, value)))
            .collect();
        let mut changes: Vec<(Duration, u64)> = Vec::new();
        for (at, value) in &inside {
            if changes.last().map(|(_, last)| last) != Some(value) {
                changes.push((*at, *value));
            }
        }
        let longest_still = changes
            .windows(2)
            .map(|two| two[1].0.saturating_sub(two[0].0))
            .max()
            .unwrap_or_default();
        let went_back = changes.windows(2).any(|two| two[1].1 < two[0].1);
        let last = seen.last().map(|(_, bar, count)| (*bar, count.clone()));
        println!(
            "{said}: {} looks at the row, {} of them inside the file, {} different figures; the \
             longest the figure stood still while the file moved {:.2} s; the last look {:?}.",
            seen.len(),
            inside.len(),
            changes.len(),
            longest_still.as_secs_f64(),
            last
        );
        a_figure!(
            missed,
            changes.len() >= 3,
            "{said}: the row was seen at {} figures inside the file, not moving with it",
            changes.len()
        );
        if let Some(still) = still {
            a_figure!(
                missed,
                longest_still <= still,
                "{said}: the row stood still for {:.2} s while the file moved",
                longest_still.as_secs_f64()
            );
        }
        a_figure!(missed, !went_back, "{said}: the row's figure went back");
        a_figure!(
            missed,
            matches!(&last, Some((None, count)) if count == "done"),
            "{said}: the row's last look was {last:?}, not done"
        );
        missed
    }

    /// **A file's row moves while its bytes move, up and then down** — the
    /// fault as a person met it: the upload bar read *0 of 1* for the whole of
    /// a 4 GB upload, because the byte loop's reports went to nobody and the
    /// row counted finished files only. Added 2026-09-29, step 10c's transfer
    /// wave piece 5 (`Server protocol and transfers` G4, `File Browser` U11,
    /// `Download` M16).
    ///
    /// **Held to a cap so it takes seconds rather than a blink**: 48 MB at
    /// 8 MB/s, of which the cap's first three-second window lets 24 MB go at
    /// once and the rest takes about three seconds each way, looked at every
    /// tenth of a second.
    #[tokio::test(flavor = "multi_thread", worker_threads = 4)]
    async fn a_files_row_moves_while_its_bytes_move_up_and_down() {
        let _deadline = TheTestsDeadline::started();
        let program = the_server_or_say_so!();

        let size = 48_000_000u64;

        let scratch = Scratch::new("moving");
        let here = scratch.folder("here");
        let machine_side = scratch.folder("machine");
        let chosen = scratch.folder("chosen");
        let source = here.join("moving.blend");
        a_large_file_at(&source, size);
        let record = TheTimedRecord::at(&scratch.folder("record").join("smoozy.sqlite3"));
        let server_row = a_server_row_for(&record);
        let machine = Arc::new(AMachine {
            program,
            home: scratch.at.clone(),
            limit: 10,
            children: Mutex::new(Vec::new()),
            ..AMachine::default()
        });
        let point = APointLending::over(&machine).await;
        let the_run = Arc::new(TheRunsTransfers::for_this_run());
        let cap = TheCap::Of(BytesPerSecond(8_000_000));
        the_run.the_caps_saved_are(cap, cap);

        // ---- up
        let asked = WhatIsBeingUploaded {
            server_row: server_row.clone(),
            installation: Some(Identity::issue(IdentityKind::Installation).expect("random")),
            destination: the_bytes_of(&machine_side),
            chosen: vec![source.clone()],
            run: a_run(),
        };
        let as_a_door: Arc<dyn TheRecordAnUploadWrites> = record.clone();
        let mut ready = an_upload_is_prepared(&asked, &point.sessions, as_a_door.as_ref())
            .await
            .expect("the upload is prepared");
        assert_eq!(
            the_answer_is_applied(&mut ready, None, &point.sessions, as_a_door.as_ref()).await,
            WhatTheAnswerLeft::ItGoes
        );
        let watched = TheRowWatched::over(&record, &the_run, ready.request.record.as_text());
        let came_to = the_upload_runs(
            ready,
            &point.sessions,
            &the_run,
            Arc::clone(&as_a_door),
            &NobodyIsTold,
        )
        .await;
        std::thread::sleep(Duration::from_millis(250));
        let up = watched.ended();
        assert_eq!(came_to.uploaded, 1, "{came_to:?}");
        let landed = machine_side.join("moving.blend");
        let mut missed = the_row_moved("UP", &up, size, None);

        // ---- down
        let asked = WhatIsBeingDownloaded {
            server_row,
            installation: None,
            destination: the_bytes_of(&chosen),
            picked: vec![the_bytes_of(&landed)],
            run: a_run(),
        };
        let mut ready = a_download_is_prepared(
            asked,
            &point.sessions,
            record.as_ref() as &dyn TheRecordADownloadWrites,
            &NoJobIsKnown,
        )
        .await
        .expect("the download is prepared");
        the_request_goes_without_a_question(&mut ready, record.as_ref())
            .expect("the dispositions are written");
        let watched = TheRowWatched::over(&record, &the_run, ready.request.record.as_text());
        let came_to = the_download_runs(&mut ready, &point.sessions, &the_run, record.as_ref()).await;
        std::thread::sleep(Duration::from_millis(250));
        let down = watched.ended();
        assert_eq!(came_to.fetched, 1, "{came_to:?}");
        assert!(it_holds_the_pattern(&chosen.join("moving.blend"), size));
        missed.extend(the_row_moved("DOWN", &down, size, None));
        the_figures_held("A FILE'S ROW", missed);
    }

    /// **A request whose line was cut reads as ended, never as under way** —
    /// the fault as a person met it: a request cut off by the line going
    /// read *running* for ever, with *Stop* on it and the rail counting it,
    /// while nothing in Smoozy was moving it or ever would again. Added
    /// 2026-09-29, step 10c's transfer wave piece 5 (`File Browser` U7 with its
    /// paragraph of 2026-09-28, `Download` M13's).
    ///
    /// **Three files of 24 MB, held to 4 MB/s** — the cap's first window lets
    /// half the first go at once, so the second starts about three seconds
    /// in; the line is cut once the first is up and the second is moving, and
    /// the period ends with it.
    ///
    /// **The period's end is told to the run's register, as the running
    /// application's period does at its end** (`src/servers/a_period.rs`), so
    /// this is the cut the user makes with *Disconnect* — step 10c's transfer
    /// wave piece 6, whose desk check this also is: under `File Browser` K3 a
    /// period's end is right not to carry on, the request ends and reads ended,
    /// and **the cut file's record stays *interrupted*** (`File Browser` U7's
    /// paragraph of 2026-09-28). Until piece 6 nothing needed telling, because
    /// nothing waited for a repair.
    #[tokio::test(flavor = "multi_thread", worker_threads = 4)]
    async fn a_request_whose_line_was_cut_reads_ended_and_not_running() {
        let _deadline = TheTestsDeadline::started();
        let program = the_server_or_say_so!();

        let size = 24_000_000u64;

        let scratch = Scratch::new("cut");
        let here = scratch.folder("here/three");
        three_parts_one_large(&here, size);
        let machine_side = scratch.folder("machine");
        let record = TheTimedRecord::at(&scratch.folder("record").join("smoozy.sqlite3"));
        let server_row = a_server_row_for(&record);
        let the_server = server_row.as_text();
        let machine = Arc::new(AMachine {
            program,
            home: scratch.at.clone(),
            limit: 10,
            children: Mutex::new(Vec::new()),
            ..AMachine::default()
        });
        let point = APointLending::over(&machine).await;
        let the_run = Arc::new(TheRunsTransfers::for_this_run());
        let cap = TheCap::Of(BytesPerSecond(4_000_000));
        the_run.the_caps_saved_are(cap, cap);

        let asked = WhatIsBeingUploaded {
            server_row,
            installation: Some(Identity::issue(IdentityKind::Installation).expect("random")),
            destination: the_bytes_of(&machine_side),
            chosen: vec![here.clone()],
            run: a_run(),
        };
        let as_a_door: Arc<dyn TheRecordAnUploadWrites> = record.clone();
        let mut ready = an_upload_is_prepared(&asked, &point.sessions, as_a_door.as_ref())
            .await
            .expect("the upload is prepared");
        assert_eq!(
            the_answer_is_applied(&mut ready, None, &point.sessions, as_a_door.as_ref()).await,
            WhatTheAnswerLeft::ItGoes
        );
        let request = ready.request.record.as_text();
        let cutting = async {
            let began = Instant::now();
            while record.moved.lock().expect("the moments").len() < 2
                && began.elapsed() < Duration::from_secs(60)
            {
                tokio::time::sleep(Duration::from_millis(20)).await;
            }
            // **The two small files are across and the large one moves
            // alone**, a few hundred milliseconds in (step 10c's eleventh-pass
            // fix piece 4: the files of a request move several at once).
            tokio::time::sleep(Duration::from_millis(400)).await;
            machine.every_session_ends();
            point.lending.abort();
            the_run.a_servers_period_ended(&the_server);
        };
        let (came_to, ()) = tokio::join!(
            the_upload_runs(
                ready,
                &point.sessions,
                &the_run,
                Arc::clone(&as_a_door),
                &NobodyIsTold,
            ),
            cutting
        );
        assert_eq!(came_to.uploaded, 2, "{came_to:?}");
        assert_eq!(came_to.did_not_finish, 1, "{came_to:?}");

        // ---- what a person looking at Transfers now reads
        let screen = the_screen_now(&record, &the_run);
        let row = the_row(&screen, &request).expect("the request is on the screen");
        let control = row.control.as_ref().map(|control| control.act.as_str().to_string());
        println!(
            "CUT: the row reads «{}» · «{}» · «{}», control {control:?}, bar {:?}, the rail's \
             figure {:?}.",
            as_read(&row.name),
            as_read(&row.count),
            as_read(&row.where_it_goes),
            row.bar,
            screen.the_rails_figure
        );
        let mut missed = Vec::new();
        a_figure!(
            missed,
            control.as_deref() != Some("stop"),
            "CUT: the row offers Stop on a request nothing is moving"
        );
        a_figure!(
            missed,
            screen.the_rails_figure.is_none(),
            "CUT: the rail counts {:?} request running",
            screen.the_rails_figure
        );
        a_figure!(missed, row.bar.is_none(), "CUT: the row draws a bar as if under way");
        a_figure!(
            missed,
            as_read(&row.name).contains("did not finish"),
            "CUT: the row does not say the request did not finish"
        );
        a_figure!(
            missed,
            as_read(&row.count) == "2 / 3",
            "CUT: the row counts «{}», not 2 of 3",
            as_read(&row.count)
        );
        // **The cut file's record stays *interrupted*** — nothing of the
        // request is arranged again (`File Browser` U7).
        let endings: Vec<Option<smoozy_server_protocol::transfers::Ending>> = {
            let held = record.record.lock().expect("the record");
            smoozy_desktop::record::transfer_records::all(held.connection())
                .expect("the transfer records read")
                .into_iter()
                .map(|one| one.ending)
                .collect()
        };
        println!("CUT: the transfer records' endings {endings:?}.");
        a_figure!(
            missed,
            endings
                .iter()
                .filter(|ending| **ending == Some(smoozy_server_protocol::transfers::Ending::Interrupted))
                .count()
                == 1,
            "CUT: the cut file's record does not read interrupted: {endings:?}"
        );
        the_figures_held("A CUT REQUEST", missed);
    }

    // ------------------------------- carrying on across a repair (piece 6)

    /// **One look at the run's register for a file in flight**: when, its
    /// bytes staged, its two marks, and whether it waits for the connection.
    #[derive(Debug, Clone, Copy)]
    struct ALook {
        at: Instant,
        staged: u64,
        verifying: Option<u64>,
        started_over: bool,
        /// **Why it started over**, where it did (`Server protocol and
        /// transfers` G4a). The 38's piece 9.
        cause: Option<smoozy_server_protocol::transfers::StartOverCause>,
        waits: bool,
    }

    /// **The file's own reports, looked at every two milliseconds** from the
    /// run's register — what its row is drawn from.
    struct TheFileWatched {
        stop: Arc<AtomicBool>,
        looking: Option<std::thread::JoinHandle<Vec<ALook>>>,
    }

    impl TheFileWatched {
        fn over(the_run: &Arc<TheRunsTransfers>, request: String) -> TheFileWatched {
            let stop = Arc::new(AtomicBool::new(false));
            let (the_run, stopping) = (Arc::clone(the_run), Arc::clone(&stop));
            let looking = std::thread::spawn(move || {
                let mut seen = Vec::new();
                while !stopping.load(Ordering::SeqCst) {
                    if let Some(file) = the_run
                        .what_is_under_way()
                        .get(&request)
                        .and_then(|under_way| under_way.moving.first().cloned())
                    {
                        seen.push(ALook {
                            at: Instant::now(),
                            staged: file.report.bytes_staged,
                            verifying: file.report.verifying_a_prefix_of,
                            started_over: file.report.started_over.is_some(),
                            cause: file.report.started_over,
                            waits: file.waits_for_the_connection,
                        });
                    }
                    std::thread::sleep(Duration::from_millis(2));
                }
                seen
            });
            TheFileWatched {
                stop,
                looking: Some(looking),
            }
        }

        fn ended(mut self) -> Vec<ALook> {
            self.stop.store(true, Ordering::SeqCst);
            self.looking
                .take()
                .map(|looking| looking.join().expect("the looks"))
                .unwrap_or_default()
        }
    }

    impl Drop for TheFileWatched {
        fn drop(&mut self) {
            self.stop.store(true, Ordering::SeqCst);
        }
    }

    /// **A bounded wait on something the test is certain to see**, with what
    /// it was waiting for said where it is not seen.
    async fn until(said: &str, within: Duration, mut seen: impl FnMut() -> bool) {
        let began = Instant::now();
        while !seen() {
            assert!(began.elapsed() < within, "{said}: not seen within {within:?}");
            tokio::time::sleep(Duration::from_millis(2)).await;
        }
    }

    /// **What the cut and the repairs came to**, for the figures.
    struct WhatTheCutsCameTo {
        /// Bytes the file had staged when the line was cut.
        at_the_cut: u64,
        /// **The row a person read while the line was down**: whether the
        /// request still read as under way, its control, and the file row's
        /// state and tail.
        while_down: (bool, Option<String>, String, Option<String>),
        /// When the line came back for good.
        back_for_good: Instant,
    }

    /// **How the second cut meets the file asking for its channel.**
    #[derive(Clone, Copy)]
    enum TheSecondCut {
        /// **The asking is held and the lending ends under it** — the
        /// point's own ending drops the ask unanswered.
        WhileItsChannelIsAskedFor,
        /// **The asking is answered *the connection is gone*, and the
        /// transport reports the ending a moment later**, as a real line
        /// does: the channel's opening fails at once, and the keepalive or
        /// the line's watch says so after. Added 2026-10-02, out of the
        /// hosted run that was red on Linux at `d7a25677`.
        ItsChannelFindsTheLineGone,
    }

    /// **How long the transport's report trails the channel's refusal** in
    /// [`TheSecondCut::ItsChannelFindsTheLineGone`] — well inside
    /// `THE_BREAK_IS_LOOKED_FOR_FOR`'s two seconds, and long enough that a
    /// single glance at the point always comes first.
    const THE_REPORT_TRAILS_BY: Duration = Duration::from_millis(200);

    /// **The line is cut under a file at `half`, comes back, goes again the
    /// moment the file asks for its channel, and comes back for good** — a
    /// repair, twice, and never a *Disconnect*: the period stands throughout.
    #[allow(clippy::too_many_arguments)]
    async fn the_line_is_cut_twice_and_repaired(
        said: &str,
        machine: &Arc<AMachine>,
        point: &mut APointLending,
        the_run: &Arc<TheRunsTransfers>,
        record: &Arc<TheTimedRecord>,
        request: &str,
        half: u64,
        second: TheSecondCut,
    ) -> WhatTheCutsCameTo {
        let a_file_of = |the_run: &TheRunsTransfers| {
            the_run
                .what_is_under_way()
                .get(request)
                .and_then(|under_way| under_way.moving.first().cloned())
        };
        let waits = |the_run: &TheRunsTransfers| {
            the_run
                .what_is_under_way()
                .get(request)
                .is_some_and(|under_way| under_way.waits_for_the_connection)
        };
        until(&format!("{said}: half the file staged"), Duration::from_secs(300), || {
            a_file_of(the_run).is_some_and(|file| file.report.bytes_staged >= half)
        })
        .await;
        let at_the_cut = a_file_of(the_run).map_or(0, |file| file.report.bytes_staged);
        point.the_line_is_cut(machine);
        until(&format!("{said}: the request waits for the connection"), Duration::from_secs(10), || {
            waits(the_run)
        })
        .await;
        // **What a person looking at Transfers reads now.**
        let while_down = {
            let screen = the_screen_now(record, the_run);
            let row = the_row(&screen, request).expect("the request is on the screen");
            (
                !as_read(&row.name).contains("did not finish"),
                row.control.as_ref().map(|control| control.act.as_str().to_string()),
                row.files.first().map(|file| file.state.to_string()).unwrap_or_default(),
                row.files
                    .first()
                    .and_then(|file| file.tail.as_ref().map(|tail| tail.as_str().to_string())),
            )
        };
        // **The line comes back, and goes again as the file asks for its
        // channel.**
        match second {
            TheSecondCut::WhileItsChannelIsAskedFor => {
                machine.an_open_is_held.store(false, Ordering::SeqCst);
                machine.holds_opens.store(true, Ordering::SeqCst);
                point.the_line_comes_back(machine);
                until(&format!("{said}: the file asks for its channel again"), Duration::from_secs(10), || {
                    machine.an_open_is_held.load(Ordering::SeqCst)
                })
                .await;
                point.the_line_is_cut(machine);
                machine.holds_opens.store(false, Ordering::SeqCst);
            }
            TheSecondCut::ItsChannelFindsTheLineGone => {
                machine.an_open_found_the_line_gone.store(false, Ordering::SeqCst);
                machine.opens_find_the_line_gone.store(true, Ordering::SeqCst);
                point.the_line_comes_back(machine);
                until(&format!("{said}: the file's channel finds the line gone"), Duration::from_secs(10), || {
                    machine.an_open_found_the_line_gone.load(Ordering::SeqCst)
                })
                .await;
                tokio::time::sleep(THE_REPORT_TRAILS_BY).await;
                point.the_line_is_cut(machine);
                machine.opens_find_the_line_gone.store(false, Ordering::SeqCst);
            }
        }
        until(&format!("{said}: the request waits again"), Duration::from_secs(10), || {
            waits(the_run)
        })
        .await;
        tokio::time::sleep(Duration::from_millis(500)).await;
        let back_for_good = Instant::now();
        point.the_line_comes_back(machine);
        WhatTheCutsCameTo {
            at_the_cut,
            while_down,
            back_for_good,
        }
    }

    /// **The figures one direction's carry-on is held to.**
    fn the_carry_on_held(
        said: &str,
        cuts: &WhatTheCutsCameTo,
        looks: &[ALook],
        it_starts_over: bool,
    ) -> Vec<String> {
        let mut missed = Vec::new();
        let (under_way, control, state, tail) = &cuts.while_down;
        let after: Vec<&ALook> = looks.iter().filter(|look| look.at > cuts.back_for_good).collect();
        let resumed_from = after.iter().map(|look| look.staged).min().unwrap_or(0);
        let verified = after.iter().find_map(|look| look.verifying);
        let started_over = after.iter().any(|look| look.started_over);
        let waited = looks.iter().any(|look| look.waits);
        println!(
            "{said}: cut at {} bytes; while the line was down the row read under way {under_way}, \
             control {control:?}, the file «{state}» «{tail:?}», the file marked waiting {waited}; \
             after the repair the lowest figure {resumed_from}, verifying {verified:?}, started \
             over {started_over}.",
            cuts.at_the_cut
        );
        a_figure!(missed, *under_way, "{said}: the request read ended while the line was down");
        a_figure!(
            missed,
            control.as_deref() == Some("stop"),
            "{said}: a request waiting for its connection offers {control:?}, not Stop"
        );
        a_figure!(missed, waited, "{said}: the file was never marked waiting for the connection");
        if it_starts_over {
            // **`Download` M11a and `Server protocol and transfers` G4a**: a
            // download's resume starts over, and says so.
            a_figure!(missed, started_over, "{said}: the start-over carried no mark");
        } else {
            a_figure!(
                missed,
                // The tail names the direction (step 13's words sweep): it said
                // *sent* both ways, and this check never told them apart.
                state == "waitpart" && matches!(tail.as_deref(), Some("uploaded" | "downloaded")),
                "{said}: the cut file read «{state}» «{tail:?}», not partial and waiting"
            );
            let checkpoint = smoozy_server_protocol::transfers::moving::THE_CHECKPOINT_INTERVAL;
            a_figure!(
                missed,
                resumed_from > 0 && resumed_from + 2 * checkpoint >= cuts.at_the_cut,
                "{said}: after the repair the file stood at {resumed_from} bytes, where the cut \
                 was at {} — not carried on from its last checkpoint",
                cuts.at_the_cut
            );
            a_figure!(missed, !started_over, "{said}: the upload started over");
        }
        missed
    }

    /// **What is left in a staging folder.**
    fn what_is_left_staged(folder: &Path) -> Vec<String> {
        std::fs::read_dir(folder.join(".smoozy-staging"))
            .map(|entries| {
                entries
                    .filter_map(Result::ok)
                    .map(|entry| entry.file_name().to_string_lossy().into_owned())
                    .collect()
            })
            .unwrap_or_default()
    }

    /// **A file the line cuts carries on across a repair, up and then down** —
    /// step 10c's transfer wave piece 6, the map's first desk check (`File
    /// Browser` K1, K2, K4 and K10; `Download` M11; `Server protocol and
    /// transfers` C11 step 4 and `Server protocol and transfers` S12).
    ///
    /// **The fault as a person met it**: the line dropped for a minute under
    /// an upload and came back inside the period, and the upload had ended —
    /// *did not finish* — and so had every file after it, though nothing had
    /// stopped it and the line was healthy again. **The cut here is a repair
    /// and not a *Disconnect***: the point goes dead and comes back through the
    /// same point, inside one period. It goes twice — once under the file, and
    /// again the moment the file asks for its channel after the first repair.
    async fn a_file_the_line_cuts_carries_on(
        named: &str,
        size: u64,
        cap: Option<u64>,
        second: TheSecondCut,
        the_machine: TheFourValuesOnTheMachine,
    ) {
        let program = the_server_or_say_so!();


        let scratch = Scratch::new(named);
        let here = scratch.folder("here");
        let machine_side = scratch.folder("machine");
        let chosen = scratch.folder("chosen");
        let source = here.join("carried.blend");
        a_large_file_at(&source, size);
        let record = TheTimedRecord::at(&scratch.folder("record").join("smoozy.sqlite3"));
        let server_row = a_server_row_for(&record);
        let machine = Arc::new(AMachine {
            program,
            home: scratch.at.clone(),
            limit: 10,
            answers_the_four_values: AtomicBool::new(the_machine != TheFourValuesOnTheMachine::CannotBeRead),
            ..AMachine::default()
        });
        let mut point = APointLending::over(&machine).await;
        let sessions = point.sessions.clone();
        let the_run = Arc::new(TheRunsTransfers::for_this_run());
        if let Some(cap) = cap {
            let cap = TheCap::Of(BytesPerSecond(cap));
            the_run.the_caps_saved_are(cap, cap);
        }
        let within = Duration::from_secs(if size > 100_000_000 { 1_800 } else { 180 });

        // ---- up
        let asked = WhatIsBeingUploaded {
            server_row: server_row.clone(),
            installation: Some(Identity::issue(IdentityKind::Installation).expect("random")),
            destination: the_bytes_of(&machine_side),
            chosen: vec![source.clone()],
            run: a_run(),
        };
        let as_a_door: Arc<dyn TheRecordAnUploadWrites> = record.clone();
        let mut ready = an_upload_is_prepared(&asked, &sessions, as_a_door.as_ref())
            .await
            .expect("the upload is prepared");
        assert_eq!(
            the_answer_is_applied(&mut ready, None, &sessions, as_a_door.as_ref()).await,
            WhatTheAnswerLeft::ItGoes
        );
        let request = ready.request.record.as_text();
        let watched = TheFileWatched::over(&the_run, request.clone());
        let (came_to, cuts) = tokio::join!(
            tokio::time::timeout(
                within,
                the_upload_runs(ready, &sessions, &the_run, Arc::clone(&as_a_door), &NobodyIsTold),
            ),
            the_line_is_cut_twice_and_repaired(
                "UP",
                &machine,
                &mut point,
                &the_run,
                &record,
                &request,
                size / 2,
                second,
            ),
        );
        let looks = watched.ended();
        let came_to = came_to.expect("the upload came to an end inside its bound");
        let landed = machine_side.join("carried.blend");
        let mut missed = the_carry_on_held("UP", &cuts, &looks, false);
        a_figure!(missed, came_to.uploaded == 1, "UP: the upload came to {came_to:?}");
        a_figure!(
            missed,
            it_holds_the_pattern(&landed, size),
            "UP: the file on the machine is not the file that was sent"
        );
        let left = what_is_left_staged(&machine_side);
        a_figure!(missed, left.is_empty(), "UP: left under .smoozy-staging: {left:?}");

        // ---- down
        let asked = WhatIsBeingDownloaded {
            server_row,
            installation: None,
            destination: the_bytes_of(&chosen),
            picked: vec![the_bytes_of(&landed)],
            run: a_run(),
        };
        let mut ready = a_download_is_prepared(
            asked,
            &sessions,
            record.as_ref() as &dyn TheRecordADownloadWrites,
            &NoJobIsKnown,
        )
        .await
        .expect("the download is prepared");
        the_request_goes_without_a_question(&mut ready, record.as_ref())
            .expect("the dispositions are written");
        let request = ready.request.record.as_text();
        let watched = TheFileWatched::over(&the_run, request.clone());
        // **Where the file on the machine changes while the line is down**,
        // it is touched the moment the request waits for the connection: its
        // modification and change times move, its bytes do not.
        let the_file_is_touched = async {
            if the_machine != TheFourValuesOnTheMachine::TheFileChangesWhileTheLineIsDown {
                return;
            }
            until("DOWN: the request waits for the connection", Duration::from_secs(300), || {
                the_run.what_is_under_way().get(&request).is_some_and(|under_way| under_way.waits_for_the_connection)
            })
            .await;
            std::fs::File::options()
                .write(true)
                .open(&landed)
                .and_then(|file| file.set_modified(std::time::SystemTime::now() + Duration::from_secs(5)))
                .expect("the file on the machine is touched");
        };
        let (came_to, cuts, ()) = tokio::join!(
            tokio::time::timeout(
                within,
                the_download_runs(&mut ready, &sessions, &the_run, record.as_ref()),
            ),
            the_line_is_cut_twice_and_repaired(
                "DOWN",
                &machine,
                &mut point,
                &the_run,
                &record,
                &request,
                size / 2,
                second,
            ),
            the_file_is_touched,
        );
        let looks = watched.ended();
        let came_to = came_to.expect("the download came to an end inside its bound");
        // **`Server protocol and transfers` S13 for a download, the 38's piece
        // 9 (item 18)**: the four values read when it was arranged and again
        // before it carries on. **Where they agree, it carries on from its last
        // checkpoint**; where any differs it starts over and says the file
        // changed on the server; where none could be read it starts over, as
        // it did before, and is never read as changed.
        let it_starts_over = the_machine != TheFourValuesOnTheMachine::AreReadAndAgree;
        missed.extend(the_carry_on_held("DOWN", &cuts, &looks, it_starts_over));
        let mut causes: Vec<_> = looks.iter().filter_map(|look| look.cause).collect();
        causes.dedup();
        println!("DOWN: the start-over causes seen {causes:?}.");
        let changed = smoozy_server_protocol::transfers::StartOverCause::TheSourceChangedUnderADownload;
        match the_machine {
            TheFourValuesOnTheMachine::TheFileChangesWhileTheLineIsDown => a_figure!(
                missed,
                causes.contains(&changed),
                "DOWN: the file changed on the machine and the start-over did not say so: {causes:?}"
            ),
            _ => a_figure!(
                missed,
                !causes.contains(&changed),
                "DOWN: the file did not change, and the start-over said it had: {causes:?}"
            ),
        }
        a_figure!(missed, came_to.fetched == 1, "DOWN: the download came to {came_to:?}");
        a_figure!(
            missed,
            it_holds_the_pattern(&chosen.join("carried.blend"), size),
            "DOWN: the file on this computer is not the file on the machine"
        );
        // **The attempt the carry-on superseded is gone once the fresh one has
        // published** — step 10c's transfer wave piece 7, item 4: removed by
        // its record and its recorded identity (`Server protocol and
        // transfers` S12 step 5, `Server protocol and transfers` S28), not left for the next start's sweep.
        let left = what_is_left_staged(&chosen);
        println!("DOWN: left under .smoozy-staging: {left:?}");
        a_figure!(missed, left.is_empty(), "DOWN: the superseded part-file is left staged: {left:?}");
        the_figures_held("CARRIED ON ACROSS A REPAIR", missed);
    }

    /// **A channel that fails on a line that stays up is not carried on** —
    /// `Server protocol and transfers` C11 step 4 carries a transfer on only
    /// across a **replaced** connection, and none is coming. Step 10c's
    /// transfer wave piece 6. **The file's sessions end under it and the
    /// point goes on lending**: the file ends *interrupted*, owed, within
    /// moments, and nothing is tried again on a line that is up — which is
    /// what keeps an ending the line did not cause from being retried for
    /// ever.
    #[tokio::test(flavor = "multi_thread", worker_threads = 4)]
    async fn a_channel_that_fails_on_a_line_that_stays_up_is_not_carried_on() {
        let _deadline = TheTestsDeadline::started();
        let program = the_server_or_say_so!();

        let size = 24_000_000u64;
        let scratch = Scratch::new("stays-up");
        let here = scratch.folder("here");
        let machine_side = scratch.folder("machine");
        let source = here.join("cut.blend");
        a_large_file_at(&source, size);
        let record = TheTimedRecord::at(&scratch.folder("record").join("smoozy.sqlite3"));
        let server_row = a_server_row_for(&record);
        let machine = Arc::new(AMachine {
            program,
            home: scratch.at.clone(),
            limit: 10,
            ..AMachine::default()
        });
        let point = APointLending::over(&machine).await;
        let the_run = Arc::new(TheRunsTransfers::for_this_run());
        let cap = TheCap::Of(BytesPerSecond(4_000_000));
        the_run.the_caps_saved_are(cap, cap);
        let asked = WhatIsBeingUploaded {
            server_row,
            installation: Some(Identity::issue(IdentityKind::Installation).expect("random")),
            destination: the_bytes_of(&machine_side),
            chosen: vec![source],
            run: a_run(),
        };
        let as_a_door: Arc<dyn TheRecordAnUploadWrites> = record.clone();
        let mut ready = an_upload_is_prepared(&asked, &point.sessions, as_a_door.as_ref())
            .await
            .expect("the upload is prepared");
        assert_eq!(
            the_answer_is_applied(&mut ready, None, &point.sessions, as_a_door.as_ref()).await,
            WhatTheAnswerLeft::ItGoes
        );
        let request = ready.request.record.as_text();
        let cutting = async {
            until("STAYS UP: a third of the file staged", Duration::from_secs(60), || {
                the_run
                    .what_is_under_way()
                    .get(&request)
                    .and_then(|under_way| under_way.moving.first().map(|file| file.report.bytes_staged))
                    .is_some_and(|staged| staged >= size / 3)
            })
            .await;
            machine.every_session_ends();
        };
        let began = Instant::now();
        let (came_to, ()) = tokio::join!(
            tokio::time::timeout(
                Duration::from_secs(60),
                the_upload_runs(ready, &point.sessions, &the_run, Arc::clone(&as_a_door), &NobodyIsTold),
            ),
            cutting
        );
        let came_to = came_to.expect("the upload came to an end inside its bound");
        println!(
            "STAYS UP: the upload came to {came_to:?} after {:.2} s, the point lending {}.",
            began.elapsed().as_secs_f64(),
            point.sessions.a_connection_is_live()
        );
        assert_eq!(came_to.uploaded, 0, "a file whose channel failed on a live line was carried on");
        assert_eq!(came_to.did_not_finish, 1, "{came_to:?}");
    }

    /// **The quick twin, run at every desk run**: 48 MB each way, held to
    /// 8 MB/s so the cut lands inside the file.
    #[tokio::test(flavor = "multi_thread", worker_threads = 4)]
    async fn a_file_the_line_cuts_carries_on_across_a_repair_up_and_down() {
        let _deadline = TheTestsDeadline::started();
        a_file_the_line_cuts_carries_on(
            "repaired",
            48_000_000,
            Some(8_000_000),
            TheSecondCut::WhileItsChannelIsAskedFor,
            TheFourValuesOnTheMachine::CannotBeRead,
        )
        .await;
    }

    /// **A file whose channel finds the line gone waits for the repair, up
    /// and down** — `File Browser` K1 and K2, `File Browser` U5, `Download`
    /// M11, `Server protocol and transfers` C11 step 4. Added 2026-10-02, out
    /// of the hosted run that was red on Linux at `d7a25677`.
    ///
    /// **The fault**: a file asking for its channel at the moment the line
    /// went was answered *the connection is gone* by the point while the point
    /// was still lending, glanced once at the point, saw it lending, and ended
    /// *did not finish* — though the line came back inside the period. **On
    /// Linux the same glance lost a race to the lending's own ending**, which
    /// drops a waiting ask before it stops saying it lends. Here the
    /// transport's report trails the refusal by [`THE_REPORT_TRAILS_BY`], as
    /// on a real line, so the glance loses every time.
    #[tokio::test(flavor = "multi_thread", worker_threads = 4)]
    async fn a_file_whose_channel_finds_the_line_gone_waits_for_the_repair_up_and_down() {
        let _deadline = TheTestsDeadline::started();
        a_file_the_line_cuts_carries_on(
            "refused-as-gone",
            48_000_000,
            Some(8_000_000),
            TheSecondCut::ItsChannelFindsTheLineGone,
            TheFourValuesOnTheMachine::CannotBeRead,
        )
        .await;
    }

    /// **Which of `Server protocol and transfers` S13's three answers the
    /// machine gives a download that carries on** — the 38's piece 9, item 18.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum TheFourValuesOnTheMachine {
        /// The reading cannot be taken (every command is refused).
        CannotBeRead,
        /// It is taken, and the file did not change.
        AreReadAndAgree,
        /// It is taken, and the file was touched while the line was down.
        TheFileChangesWhileTheLineIsDown,
    }

    /// **A download the line cuts carries on from its last checkpoint where the
    /// file on the machine has not changed** — `Download` M11, `Server
    /// protocol and transfers` S12 and `Server protocol and transfers` S13.
    /// The 38's piece 9, item 18: until
    /// then nothing read the four values, and every cut download started over
    /// (*"DOWN: cut at 24000000 bytes … after the repair the lowest figure 0
    /// … started over true"*, the desk on 2026-10-04 before the fix).
    #[tokio::test(flavor = "multi_thread", worker_threads = 4)]
    async fn a_download_the_line_cuts_carries_on_where_the_file_on_the_machine_has_not_changed() {
        let _deadline = TheTestsDeadline::started();
        a_file_the_line_cuts_carries_on(
            "four-values-agree",
            48_000_000,
            Some(8_000_000),
            TheSecondCut::WhileItsChannelIsAskedFor,
            TheFourValuesOnTheMachine::AreReadAndAgree,
        )
        .await;
    }

    /// **A download the line cuts starts over where the file on the machine
    /// changed while the line was down, and says why** — `Server protocol and
    /// transfers` S13: *"any difference starts the transfer over rather than
    /// resuming it"*. The 38's piece 9, item 18.
    #[tokio::test(flavor = "multi_thread", worker_threads = 4)]
    async fn a_download_the_line_cuts_starts_over_where_the_file_on_the_machine_changed() {
        let _deadline = TheTestsDeadline::started();
        a_file_the_line_cuts_carries_on(
            "four-values-differ",
            48_000_000,
            Some(8_000_000),
            TheSecondCut::WhileItsChannelIsAskedFor,
            TheFourValuesOnTheMachine::TheFileChangesWhileTheLineIsDown,
        )
        .await;
    }

    /// **The map's own size: a 1 GB file cut at half**, up and then down, with
    /// no cap. Run on purpose, in release.
    #[tokio::test(flavor = "multi_thread", worker_threads = 4)]
    #[ignore = "the kept harness: a gigabyte each way; run on purpose"]
    async fn a_large_file_the_line_cuts_carries_on_across_a_repair() {
        let _deadline = TheTestsDeadline::started();
        let size = a_number_from("SMOOZY_HARNESS_BYTES", 1_000_000_000);
        a_file_the_line_cuts_carries_on(
            "repaired-large",
            size,
            None,
            TheSecondCut::WhileItsChannelIsAskedFor,
            TheFourValuesOnTheMachine::AreReadAndAgree,
        )
        .await;
    }

    /// **Many small files up through Files**, in nested folders, the last
    /// thousand timed against the first thousand.
    ///
    /// **The figures**: the last thousand no slower than the first thousand,
    /// and under 20 minutes for 10,002.
    #[tokio::test(flavor = "multi_thread", worker_threads = 4)]
    #[ignore = "the kept harness: minutes to hours; run on purpose"]
    async fn many_small_files_go_up() {
        let _deadline = TheTestsDeadline::started();
        let program = the_server_or_say_so!();

        let count = a_number_from("SMOOZY_HARNESS_FILES", 10_002) as usize;
        let the_bound = Duration::from_secs_f64((1_200.0 * count as f64 / 10_002.0).max(20.0));

        let scratch = Scratch::new("many");
        let here = scratch.folder("here/many");
        for at in 0..count {
            let folder = here.join(format!("f{:02}/g{:02}", at % 10, (at / 10) % 10));
            std::fs::create_dir_all(&folder).expect("the folder is made");
            let size = 1_024 + (at * 7_919) % 7_169;
            let mut bytes = vec![0u8; size];
            the_pattern_at(at as u64 * 8, &mut bytes);
            std::fs::write(folder.join(format!("file_{at:05}.bin")), &bytes).expect("written");
        }
        let machine_side = scratch.folder("machine");

        let record = TheTimedRecord::at(&scratch.folder("record").join("smoozy.sqlite3"));
        let server_row = a_server_row_for(&record);
        let machine = Arc::new(AMachine {
            program,
            home: scratch.at.clone(),
            limit: 10,
            children: Mutex::new(Vec::new()),
            ..AMachine::default()
        });
        let point = APointLending::over(&machine).await;
        let the_run = Arc::new(TheRunsTransfers::for_this_run());

        let asked = WhatIsBeingUploaded {
            server_row,
            installation: Some(Identity::issue(IdentityKind::Installation).expect("random")),
            destination: the_bytes_of(&machine_side),
            chosen: vec![here.clone()],
            run: a_run(),
        };
        let as_a_door: Arc<dyn TheRecordAnUploadWrites> = record.clone();
        let began = Instant::now();
        let mut ready = an_upload_is_prepared(&asked, &point.sessions, as_a_door.as_ref())
            .await
            .expect("the upload is prepared");
        assert_eq!(
            the_answer_is_applied(&mut ready, None, &point.sessions, as_a_door.as_ref()).await,
            WhatTheAnswerLeft::ItGoes
        );
        println!("MANY UP: ready in {:.2} s.", began.elapsed().as_secs_f64());
        let readings = TheTransfersReadings::taken_over(&record, &the_run, ready.request.record.as_text());
        let from = record.now();
        let began = Instant::now();
        let came_to = the_upload_runs(
            ready,
            &point.sessions,
            &the_run,
            Arc::clone(&as_a_door),
            &NobodyIsTold,
        )
        .await;
        let took = began.elapsed();
        let slowest = the_readings_came_to("MANY UP", &readings.ended());
        assert_eq!(came_to.uploaded, count, "{came_to:?}");
        println!(
            "MANY UP: {count} files in {:.1} s ({:.1} files a second).",
            took.as_secs_f64(),
            count as f64 / took.as_secs_f64()
        );
        let every_hold = record.the_holds_since(from);
        let holds = the_holds_came_to("MANY UP", &every_hold);
        let moments: Vec<Duration> = record
            .moved
            .lock()
            .expect("the moments")
            .iter()
            .filter(|at| **at >= from)
            .copied()
            .collect();
        let (first, last) = the_thousands("MANY UP", &moments, &every_hold);
        for at in [0usize, count / 2, count - 1] {
            let path = machine_side.join(format!(
                "many/f{:02}/g{:02}/file_{at:05}.bin",
                at % 10,
                (at / 10) % 10
            ));
            let size = 1_024 + (at * 7_919) % 7_169;
            let mut bytes = vec![0u8; size];
            the_pattern_at(at as u64 * 8, &mut bytes);
            assert_eq!(std::fs::read(&path).expect("landed"), bytes, "{path:?}");
        }

        let mut missed = Vec::new();
        a_figure!(
            missed,
            took <= the_bound,
            "{:.1} s, over the {:.1} s this count is owed",
            took.as_secs_f64(),
            the_bound.as_secs_f64()
        );
        a_figure!(
            missed,
            last <= first.mul_f64(1.25) + Duration::from_millis(500),
            "the last thousand took {:.1} s against {:.1} s for the first",
            last.as_secs_f64(),
            first.as_secs_f64()
        );
        // **The lock and the reading are said above and not held to a
        // figure here**: the wave's map sets 0.1 s for both on the large
        // file only, and at this many saves a single slow flush of the disk
        // itself — one save writing one row — is what the longest hold is.
        let _ = (holds.longest, slowest);
        the_figures_held("MANY UP", missed);
    }

    /// **A Transfers reading of one finished 10,002-file upload request, timed
    /// on its own** — the reading the page takes at every file's end, with no
    /// transfer beside it, so what is timed is the composition alone. Sized by
    /// `SMOOZY_HARNESS_FILES`.
    ///
    /// **The figure**: within 0.1 s.
    #[test]
    #[ignore = "the kept harness: run on purpose"]
    fn a_transfers_reading_of_a_large_request() {
        use smoozy_file_browser::request::{
            Disposition, EnumerationEntry, KeepsTheRequest as _, UploadRequest,
        };
        let count = a_number_from("SMOOZY_HARNESS_FILES", 10_002) as usize;
        let scratch = Scratch::new("reading");
        let record = TheTimedRecord::at(&scratch.folder("record").join("smoozy.sqlite3"));
        let server_row = a_server_row_for(&record);
        let entries: Vec<EnumerationEntry> = (0..count)
            .map(|at| {
                let mut entry = EnumerationEntry::enumerated(
                    format!("/Users/a/many/f{:02}/file_{at:05}.bin", at % 10).into_bytes(),
                    format!("/home/ubuntu/many/f{:02}/file_{at:05}.bin", at % 10).into_bytes(),
                    1_024 + at as u64,
                );
                entry.disposition = Some(Disposition::ToSend);
                entry.transfer = Some(Identity::issue(IdentityKind::Transfer).expect("random"));
                entry
                    .its_outcome_became(
                        if at % 1_000 == 7 {
                            UnitOutcome::Failed
                        } else {
                            UnitOutcome::Done
                        },
                        std::time::SystemTime::now(),
                    )
                    .expect("once");
                entry
            })
            .collect();
        let request = UploadRequest::confirmed(
            smoozy_file_browser::request::open_an_upload_request().expect("open"),
            Identity::issue(smoozy_desktop::record::upload_requests::THE_RECORD_IDENTITY_KIND)
                .expect("random"),
            server_row,
            None,
            b"/home/ubuntu".to_vec(),
            entries,
            a_run(),
        );
        {
            let held = record.record.lock().expect("the record");
            smoozy_desktop::record::upload_requests::UploadRequests::over(held.connection())
                .write_it_down(&request)
                .expect("written");
        }
        let mut took = Vec::new();
        for _ in 0..20 {
            let held = record.record.lock().expect("the record");
            let began = Instant::now();
            let screen = the_screen_answered(&held, &TheRunsTransfers::for_this_run());
            took.push(began.elapsed());
            assert_eq!(screen.requests.len(), 1);
        }
        let slowest = took.iter().max().copied().unwrap_or_default();
        println!(
            "READING: a {count}-file request composed in {:.1} ms at the median and {:.1} ms at \
             the slowest of 20.",
            the_median(&mut took.clone()).as_secs_f64() * 1e3,
            slowest.as_secs_f64() * 1e3
        );
        assert!(
            slowest <= Duration::from_millis(100),
            "a Transfers reading of a {count}-file request took {:.3} s",
            slowest.as_secs_f64()
        );
    }

    /// **A job's frames come down by auto-download**, the arrangement made at
    /// submission, the ending walk run over the real folder, and the owed
    /// fetches drained pass after pass as a period drives them.
    ///
    /// **The figures**: the rate steady to the end, and under 15 minutes for
    /// 5,000 frames of 64 KB.
    #[tokio::test(flavor = "multi_thread", worker_threads = 4)]
    #[ignore = "the kept harness: minutes to hours; run on purpose"]
    async fn many_frames_come_down_by_auto_download() {
        let _deadline = TheTestsDeadline::started();
        let program = the_server_or_say_so!();
        let frames = a_number_from("SMOOZY_HARNESS_FRAMES", 5_000) as u32;
        let the_bound = Duration::from_secs_f64((900.0 * frames as f64 / 5_000.0).max(20.0));

        let scratch = Scratch::new("frames");
        let outputs = scratch.folder("machine/job/output");
        let chosen = scratch.folder("chosen");
        let mut frame_bytes = vec![0u8; 64 * 1024];
        for frame in 1..=frames {
            the_pattern_at(frame as u64 * 65_536, &mut frame_bytes);
            std::fs::write(outputs.join(format!("frame_{frame:05}.png")), &frame_bytes)
                .expect("the frame is written");
        }

        let record = TheTimedRecord::at(&scratch.folder("record").join("smoozy.sqlite3"));
        let server_row = a_server_row_for(&record);
        let machine = Arc::new(AMachine {
            program,
            home: scratch.at.clone(),
            limit: 10,
            children: Mutex::new(Vec::new()),
            ..AMachine::default()
        });
        let point = APointLending::over(&machine).await;
        let the_run = Arc::new(TheRunsTransfers::for_this_run());
        let run = a_run();
        let job = Identity::issue(IdentityKind::Job).expect("random bytes");

        let door: &dyn TheRecordADownloadWrites = record.as_ref();
        let mut arrangement = the_arrangement_is_made_at_submission(
            &mut TheArrangementsKeeper { record: door },
            true,
            WhatTheSubmissionChose {
                job: job.clone(),
                server_row: server_row.clone(),
                destination: the_bytes_of(&chosen),
                narrowing: TheNarrowing::Everything,
            },
        )
        .expect("the arrangement is made");

        let entries: Vec<ANarrowedEntry> = (1..=frames)
            .map(|frame| ANarrowedEntry {
                predicted_path: format!("frame_{frame:05}.png").into_bytes(),
                frame: Some(frame as i64),
            })
            .collect();
        let order = TheOrder::The(TheAttemptsOrder::as_the_attempt_record_carries_it(
            (1..=frames as i64).collect(),
        ));
        let folder = the_bytes_of(&outputs);
        let mut running = TheArrangementsRun::fresh();
        let mut queue = TheFetchQueue::at_a_server();
        let mut probing = TheProbesCadence::fresh();
        running.a_connection_began();
        running.the_ending_arrived(smoozy_queue_history::ending::Ending::Finished);
        let window = TheWindow {
            the_desired_state_is_on: true,
            a_connection_to_the_server_stands: true,
            the_attempts_window_is_open: true,
        };
        let in_hand = || WhatIsInHand {
            the_job_output_folder: &folder,
            narrowed_entries: &entries,
            the_whole_may_grow: false,
            the_entries_the_narrowing_excludes: &[],
            order: &order,
            the_jobs_label: None,
            the_first_frame_pair: None,
            the_pairs_other_half_fetched: &[],
            window,
            the_placed_copies: None,
            the_four_values_of_the_placed_copies: &[],
            arrangements_walking_this_server: 1,
        };

        let began = Instant::now();
        one_turn_of_an_arrangement(
            &mut running,
            &point.sessions,
            door,
            &mut queue,
            &mut arrangement,
            in_hand(),
        )
        .await
        .expect("the ending walk ran");
        assert_eq!(queue.owed(), frames as usize, "every frame became fetchable");
        println!("FRAMES: the ending walk took {:.2} s.", began.elapsed().as_secs_f64());

        let readings = TheTransfersReadings::taken_over(&record, &the_run, job.as_text());
        let from = record.now();
        let began = Instant::now();
        let mut fetched = 0;
        while fetched < frames as usize && began.elapsed() < the_bound * 4 {
            let mut owing = [AnArrangementOwingFetches {
                running: &mut running,
                queue: &mut queue,
                probing: &mut probing,
                arrangement: &mut arrangement,
                in_hand: in_hand(),
            }];
            let came = the_owed_fetches_come_down(
                &mut owing,
                &point.sessions,
                &the_run,
                door,
                &run,
                Duration::from_secs(3_600),
            )
            .await;
            if came.fetched == 0 {
                break;
            }
            fetched += came.fetched;
        }
        let took = began.elapsed();
        let slowest = the_readings_came_to("FRAMES", &readings.ended());
        assert_eq!(fetched, frames as usize, "every frame came down");
        println!(
            "FRAMES: {frames} frames in {:.1} s ({:.2} frames a second).",
            took.as_secs_f64(),
            frames as f64 / took.as_secs_f64()
        );
        let every_hold = record.the_holds_since(from);
        let holds = the_holds_came_to("FRAMES", &every_hold);
        let moments: Vec<Duration> = record
            .moved
            .lock()
            .expect("the moments")
            .iter()
            .filter(|at| **at >= from)
            .copied()
            .collect();
        let (first, last) = the_thousands("FRAMES", &moments, &every_hold);

        let mut missed = Vec::new();
        a_figure!(
            missed,
            took <= the_bound,
            "{:.1} s, over the {:.1} s this count is owed",
            took.as_secs_f64(),
            the_bound.as_secs_f64()
        );
        a_figure!(
            missed,
            last <= first.mul_f64(1.25) + Duration::from_millis(500),
            "the last thousand took {:.1} s against {:.1} s for the first",
            last.as_secs_f64(),
            first.as_secs_f64()
        );
        // **The lock and the reading are said above and not held to a
        // figure here**: the wave's map sets 0.1 s for both on the large
        // file only, and at this many saves a single slow flush of the disk
        // itself — one save writing one row — is what the longest hold is.
        let _ = (holds.longest, slowest);
        the_figures_held("FRAMES", missed);
    }

    // ---------------------------- a folder download's count, timed (piece 3)

    /// **What one count came to**: the walk alone, the whole preparation
    /// (walk, record, destination), and the requests sent while it ran, by
    /// type.
    struct WhatTheCountCameTo {
        the_walk: Duration,
        the_whole: Duration,
        sent: Vec<(u8, usize)>,
        entries: usize,
    }

    fn the_requests_sent(machine: &AMachine) -> HashMap<u8, usize> {
        machine.requests_by_type.lock().expect("the counts").clone()
    }

    fn the_requests_since(before: &HashMap<u8, usize>, after: &HashMap<u8, usize>) -> Vec<(u8, usize)> {
        let mut since: Vec<(u8, usize)> = after
            .iter()
            .map(|(kind, count)| (*kind, count - before.get(kind).copied().unwrap_or(0)))
            .filter(|(_, count)| *count > 0)
            .collect();
        since.sort();
        since
    }

    /// **One count of the folder, walked alone and then prepared whole** —
    /// the walk through the route a manual download's own walk rides
    /// (`TheWalksReadingRoute`, `Download` M2), the whole through
    /// `a_download_is_prepared` as the application calls it.
    async fn the_folder_is_counted(
        point: &APointLending,
        machine: &AMachine,
        record: &Arc<TheTimedRecord>,
        server_row: &Identity,
        picked: &Path,
        chosen: &Path,
    ) -> WhatTheCountCameTo {
        use smoozy_desktop::downloading::{TheReceivingSystemsRulesHere, TheWalksReadingRoute};
        let before = the_requests_sent(machine);
        let began = Instant::now();
        let walked = smoozy_download::acting::enumerating::enumerate(
            &mut TheWalksReadingRoute::on(&point.sessions),
            &TheReceivingSystemsRulesHere,
            &NoJobIsKnown,
            &[the_bytes_of(picked)],
            &the_bytes_of(chosen),
        )
        .await;
        let the_walk = began.elapsed();
        // **The application's own route**, the count shown on Transfers from
        // the press (step 10c's twelfth-pass fix piece 3).
        let the_run = TheRunsTransfers::for_this_run();
        let began = Instant::now();
        let preparing = smoozy_desktop::downloading::a_download_is_counted_and_prepared(
            WhatIsBeingDownloaded {
                server_row: server_row.clone(),
                installation: None,
                destination: the_bytes_of(chosen),
                picked: vec![the_bytes_of(picked)],
                run: a_run(),
            },
            smoozy_desktop::downloading::TheCountIsShown {
                the_run: &the_run,
                request: Identity::issue(IdentityKind::Operation).expect("random bytes"),
                named: "p12-many".into(),
            },
            &point.sessions,
            record.as_ref() as &dyn TheRecordADownloadWrites,
            &NoJobIsKnown,
        );
        tokio::pin!(preparing);
        // **The row is there from the press**: the count is held before the
        // first folder is read.
        let mut the_row_at = None;
        let ready = loop {
            tokio::select! {
                biased;
                ready = &mut preparing => break ready,
                () = tokio::task::yield_now(), if the_row_at.is_none() => {
                    if !the_run.what_is_being_counted().is_empty() {
                        the_row_at = Some(began.elapsed());
                    }
                }
            }
        }
        .expect("the download is prepared");
        let the_whole = began.elapsed();
        println!(
            "COUNT: the counting row stood {:.3} s after the press.",
            the_row_at.map_or(f64::NAN, |at| at.as_secs_f64())
        );
        assert_eq!(ready.request.entries.len(), walked.entries.len());
        WhatTheCountCameTo {
            the_walk,
            the_whole,
            sent: the_requests_since(&before, &the_requests_sent(machine)),
            entries: walked.entries.len(),
        }
    }

    fn the_count_said(said: &str, came: &WhatTheCountCameTo) {
        println!(
            "{said}: {} files walked in {:.2} s; prepared whole in {:.2} s; requests sent (type: count) {:?}.",
            came.entries,
            came.the_walk.as_secs_f64(),
            came.the_whole.as_secs_f64(),
            came.sent
        );
    }

    /// **A folder download's count, timed alone and beside an auto-download
    /// walking and draining on the same server** — step 10c's twelfth-pass
    /// fix piece 3, before anything is cut. The twelfth pass's 1,000-file,
    /// 45-folder download showed nothing for three minutes before its first
    /// file, beside a render's walk and drain; this says where the time goes.
    ///
    /// **The figures are read, not held**: this test asserts only that every
    /// count found every file. `SMOOZY_HARNESS_ROUND_TRIP_MS` (150 when
    /// unset), `SMOOZY_HARNESS_COUNT_FILES` (1,000), `SMOOZY_HARNESS_COUNT_FRAMES`
    /// (240, of 256 KB, for the drain beside).
    #[tokio::test(flavor = "multi_thread", worker_threads = 4)]
    #[ignore = "the kept harness: minutes; run on purpose"]
    async fn a_folders_count_is_timed_alone_and_beside_a_drain() {
        let _deadline = TheTestsDeadline::started();
        let program = the_server_or_say_so!();
        let round_trip = Duration::from_millis(a_number_from("SMOOZY_HARNESS_ROUND_TRIP_MS", 150));
        let files = a_number_from("SMOOZY_HARNESS_COUNT_FILES", 1_000) as usize;
        let frames = a_number_from("SMOOZY_HARNESS_COUNT_FRAMES", 240) as u32;
        const FOLDERS: usize = 45;

        let scratch = Scratch::new("count");
        let many = scratch.folder("machine/p12-many");
        for at in 0..files {
            let folder = many.join(format!("folder-{:02}", at % FOLDERS));
            std::fs::create_dir_all(&folder).expect("the folder is made");
            std::fs::write(folder.join(format!("file_{at:04}.bin")), vec![7u8; 1_024 * (1 + at % 8)])
                .expect("the file is written");
        }
        let outputs = scratch.folder("machine/job/output");
        let mut frame_bytes = vec![0u8; 256 * 1024];
        for frame in 1..=frames {
            the_pattern_at(frame as u64 * 262_144, &mut frame_bytes);
            std::fs::write(outputs.join(format!("frame_{frame:05}.png")), &frame_bytes)
                .expect("the frame is written");
        }

        let record = TheTimedRecord::at(&scratch.folder("record").join("smoozy.sqlite3"));
        let server_row = a_server_row_for(&record);
        let machine = Arc::new(AMachine {
            program,
            home: scratch.at.clone(),
            limit: 10,
            round_trip,
            ..AMachine::default()
        });
        let point = APointLending::over(&machine).await;
        println!(
            "COUNT: {files} files in {FOLDERS} folders, a round trip of {:.2} s.",
            round_trip.as_secs_f64()
        );

        // ---- alone, three times
        for at in 0..3 {
            let came = the_folder_is_counted(
                &point,
                &machine,
                &record,
                &server_row,
                &many,
                &scratch.folder(&format!("alone-{at}")),
            )
            .await;
            assert_eq!(came.entries, files, "every file was counted");
            the_count_said("COUNT ALONE", &came);
        }

        // ---- beside an auto-download walking and draining
        let the_run = Arc::new(TheRunsTransfers::for_this_run());
        let run = a_run();
        let door: &dyn TheRecordADownloadWrites = record.as_ref();
        let mut arrangement = the_arrangement_is_made_at_submission(
            &mut TheArrangementsKeeper { record: door },
            true,
            WhatTheSubmissionChose {
                job: Identity::issue(IdentityKind::Job).expect("random bytes"),
                server_row: server_row.clone(),
                destination: the_bytes_of(&scratch.folder("frames-home")),
                narrowing: TheNarrowing::Everything,
            },
        )
        .expect("the arrangement is made");
        let entries: Vec<ANarrowedEntry> = (1..=frames)
            .map(|frame| ANarrowedEntry {
                predicted_path: format!("frame_{frame:05}.png").into_bytes(),
                frame: Some(frame as i64),
            })
            .collect();
        let order = TheOrder::The(TheAttemptsOrder::as_the_attempt_record_carries_it(
            (1..=frames as i64).collect(),
        ));
        let folder = the_bytes_of(&outputs);
        let mut running = TheArrangementsRun::fresh();
        let mut queue = TheFetchQueue::at_a_server();
        let mut probing = TheProbesCadence::fresh();
        running.a_connection_began();
        running.the_ending_arrived(smoozy_queue_history::ending::Ending::Finished);
        let window = TheWindow {
            the_desired_state_is_on: true,
            a_connection_to_the_server_stands: true,
            the_attempts_window_is_open: true,
        };
        let in_hand = || WhatIsInHand {
            the_job_output_folder: &folder,
            narrowed_entries: &entries,
            the_whole_may_grow: false,
            the_entries_the_narrowing_excludes: &[],
            order: &order,
            the_jobs_label: None,
            the_first_frame_pair: None,
            the_pairs_other_half_fetched: &[],
            window,
            the_placed_copies: None,
            the_four_values_of_the_placed_copies: &[],
            arrangements_walking_this_server: 1,
        };
        one_turn_of_an_arrangement(&mut running, &point.sessions, door, &mut queue, &mut arrangement, in_hand())
            .await
            .expect("the ending walk ran");
        assert_eq!(queue.owed(), frames as usize, "every frame became fetchable");

        let counted = AtomicBool::new(false);
        let fetched = AtomicUsize::new(0);
        let drain = async {
            let began = Instant::now();
            while !counted.load(Ordering::SeqCst) && fetched.load(Ordering::SeqCst) < frames as usize {
                let mut owing = [AnArrangementOwingFetches {
                    running: &mut running,
                    queue: &mut queue,
                    probing: &mut probing,
                    arrangement: &mut arrangement,
                    in_hand: in_hand(),
                }];
                let came = the_owed_fetches_come_down(
                    &mut owing,
                    &point.sessions,
                    &the_run,
                    door,
                    &run,
                    Duration::from_secs(5),
                )
                .await;
                fetched.fetch_add(came.fetched, Ordering::SeqCst);
                if came.fetched == 0 && began.elapsed() > Duration::from_secs(600) {
                    break;
                }
            }
        };
        // **A render's walk of its output folder at `Download` W2's pause**,
        // beside the drain, on the same reading session.
        let walks = async {
            use smoozy_desktop::downloading::TheWalksReadingRoute;
            use smoozy_download::acting::walking::ReadsOneDirectoryOnTheMachine as _;
            let mut walked = 0usize;
            while !counted.load(Ordering::SeqCst) {
                let _ = TheWalksReadingRoute::on(&point.sessions).read_one_directory(&folder).await;
                walked += 1;
                let began = Instant::now();
                while !counted.load(Ordering::SeqCst) && began.elapsed() < smoozy_download::watch::THE_PAUSE {
                    tokio::time::sleep(Duration::from_millis(50)).await;
                }
            }
            walked
        };
        let count = async {
            // **The drain at its pace first**, then the count.
            tokio::time::sleep(Duration::from_secs(3)).await;
            let at_the_start = fetched.load(Ordering::SeqCst);
            let came = the_folder_is_counted(
                &point,
                &machine,
                &record,
                &server_row,
                &many,
                &scratch.folder("beside"),
            )
            .await;
            let during = fetched.load(Ordering::SeqCst) - at_the_start;
            counted.store(true, Ordering::SeqCst);
            (came, during)
        };
        let ((), walked, (came, during)) = tokio::join!(drain, walks, count);
        assert_eq!(came.entries, files, "every file was counted");
        the_count_said("COUNT BESIDE A DRAIN", &came);
        println!(
            "COUNT BESIDE A DRAIN: {during} frames came down while it ran ({} in all), and {walked} walks of the \
             frames folder ran.",
            fetched.load(Ordering::SeqCst)
        );
    }

    // ------------------------ the leftover told, removed and taken up (piece 7)

    /// **A server row whose last confirmed installation is the one the
    /// requests are arranged under** — `File Browser` U8 and `Download` M13
    /// tell a leftover *"under the current installation identity"*, which the
    /// desk reads off the row. Added 2026-09-29, step 10c's transfer wave
    /// piece 7.
    fn a_server_row_installed(record: &TheTimedRecord) -> (Identity, Identity) {
        let row = a_server_row_for(record);
        let installation = Identity::issue(IdentityKind::Installation).expect("random bytes");
        let held = record.record.lock().expect("the record");
        let mut written = server_rows::all(held.connection())
            .expect("the rows read")
            .into_iter()
            .find(|one| one.local_server_identifier == row.as_text())
            .expect("the row is there");
        written.last_confirmed_installation_identity = Some(installation.as_text());
        server_rows::write(held.connection(), &written).expect("the row is written");
        (row, installation)
    }

    /// **Frames the ending walk sized come down with no size read of their
    /// own** — step 10c's eleventh-pass fix piece 2, run at every desk run.
    ///
    /// **The case this bites on**: a job that had already ended when the
    /// connection began, so every frame was made owed by the ending walk. The
    /// ending walk is taken outside the series (`Download` W5), its sizes went
    /// with it, and every frame's fetch asked the machine for the size the
    /// walk had just read — one round trip each inside the server's point, one
    /// of the nine a frame cost on the eleventh pass. **The count is the
    /// stand-in's own**: the attribute reads it was asked for while the frames
    /// came down, which must be none.
    #[tokio::test(flavor = "multi_thread", worker_threads = 4)]
    async fn frames_the_ending_walk_sized_come_down_with_no_size_read() {
        let _deadline = TheTestsDeadline::started();
        let program = the_server_or_say_so!();
        let frames = 5u32;
        let scratch = Scratch::new("sized");
        let outputs = scratch.folder("machine/job/output");
        let chosen = scratch.folder("chosen");
        let mut frame_bytes = vec![0u8; 300_000];
        for frame in 1..=frames {
            the_pattern_at(frame as u64 * 300_000, &mut frame_bytes);
            std::fs::write(outputs.join(format!("frame_{frame:05}.png")), &frame_bytes)
                .expect("the frame is written");
        }
        let record = TheTimedRecord::at(&scratch.folder("record").join("smoozy.sqlite3"));
        let server_row = a_server_row_for(&record);
        let machine = Arc::new(AMachine {
            program,
            home: scratch.at.clone(),
            limit: 10,
            ..AMachine::default()
        });
        let point = APointLending::over(&machine).await;
        let the_run = Arc::new(TheRunsTransfers::for_this_run());
        let run = a_run();
        let door: &dyn TheRecordADownloadWrites = record.as_ref();
        let mut arrangement = the_arrangement_is_made_at_submission(
            &mut TheArrangementsKeeper { record: door },
            true,
            WhatTheSubmissionChose {
                job: Identity::issue(IdentityKind::Job).expect("random bytes"),
                server_row,
                destination: the_bytes_of(&chosen),
                narrowing: TheNarrowing::Everything,
            },
        )
        .expect("the arrangement is made");
        let entries: Vec<ANarrowedEntry> = (1..=frames)
            .map(|frame| ANarrowedEntry {
                predicted_path: format!("frame_{frame:05}.png").into_bytes(),
                frame: Some(frame as i64),
            })
            .collect();
        let order = TheOrder::The(TheAttemptsOrder::as_the_attempt_record_carries_it(
            (1..=frames as i64).collect(),
        ));
        let folder = the_bytes_of(&outputs);
        let mut running = TheArrangementsRun::fresh();
        let mut queue = TheFetchQueue::at_a_server();
        let mut probing = TheProbesCadence::fresh();
        // **A fresh connection to a job that has already ended**: the first
        // turn is the ending walk, and no ordinary walk has read the folder.
        running.a_connection_began();
        running.the_ending_arrived(smoozy_queue_history::ending::Ending::Finished);
        let window = TheWindow {
            the_desired_state_is_on: true,
            a_connection_to_the_server_stands: true,
            the_attempts_window_is_open: true,
        };
        let in_hand = || WhatIsInHand {
            the_job_output_folder: &folder,
            narrowed_entries: &entries,
            the_whole_may_grow: false,
            the_entries_the_narrowing_excludes: &[],
            order: &order,
            the_jobs_label: None,
            the_first_frame_pair: None,
            the_pairs_other_half_fetched: &[],
            window,
            the_placed_copies: None,
            the_four_values_of_the_placed_copies: &[],
            arrangements_walking_this_server: 1,
        };
        one_turn_of_an_arrangement(
            &mut running,
            &point.sessions,
            door,
            &mut queue,
            &mut arrangement,
            in_hand(),
        )
        .await
        .expect("the ending walk ran");
        assert_eq!(
            queue.owed(),
            frames as usize,
            "every frame became fetchable"
        );
        assert!(
            running.the_last_complete_walk().is_empty(),
            "an ordinary walk ran"
        );

        let before = machine.attribute_reads.load(Ordering::SeqCst);
        let mut owing = [AnArrangementOwingFetches {
            running: &mut running,
            queue: &mut queue,
            probing: &mut probing,
            arrangement: &mut arrangement,
            in_hand: in_hand(),
        }];
        let came = the_owed_fetches_come_down(
            &mut owing,
            &point.sessions,
            &the_run,
            door,
            &run,
            Duration::from_secs(60),
        )
        .await;
        let asked = machine.attribute_reads.load(Ordering::SeqCst) - before;
        assert_eq!(
            came.fetched, frames as usize,
            "every frame came down: {came:?}"
        );
        for frame in 1..=frames {
            let landed = std::fs::read_dir(&chosen)
                .expect("the destination")
                .filter_map(Result::ok)
                .map(|folder| folder.path().join(format!("frame_{frame:05}.png")))
                .find(|path| path.exists())
                .expect("the frame landed");
            assert!(it_holds_the_frame(&landed, frame as u64 * 300_000, 300_000));
        }
        assert_eq!(
            asked, 0,
            "{asked} attribute reads were asked while {frames} frames the ending walk sized came down"
        );
    }

    /// **Each of the drain's two channels takes the next frame when its own
    /// ends** — step 10c's owed piece 3, run at every desk run.
    ///
    /// **The case this bites on**: the drain took two frames, moved both, and
    /// waited for both before taking the next two, so a channel whose frame
    /// had landed stood idle until the other's did. Over one connection the
    /// second of every pair is seated a seat later than the first (`Server
    /// protocol and transfers` T5), so the first stood idle for that seat on
    /// every pair — one channel waiting on the other. **Here the first frame
    /// is large and the rest small**, on a line where each session's answers
    /// are paced: waited for in pairs, only the large frame's partner lands
    /// while it moves; taken in turn, the other channel carries the small
    /// frames one after another beside it.
    #[tokio::test(flavor = "multi_thread", worker_threads = 4)]
    async fn a_drains_channel_takes_the_next_frame_while_the_other_still_moves_a_large_one() {
        let _deadline = TheTestsDeadline::started();
        let program = the_server_or_say_so!();
        const LARGE: usize = 4_000_000;
        const SMALL: usize = 50_000;
        const SMALL_FRAMES: u32 = 6;
        let frames = 1 + SMALL_FRAMES;
        let size_of = |frame: u32| if frame == 1 { LARGE } else { SMALL };
        let scratch = Scratch::new("in-turn");
        let outputs = scratch.folder("machine/job/output");
        let chosen = scratch.folder("chosen");
        for frame in 1..=frames {
            let mut bytes = vec![0u8; size_of(frame)];
            the_pattern_at(frame as u64 * 1_000_000, &mut bytes);
            std::fs::write(outputs.join(format!("frame_{frame:05}.png")), &bytes)
                .expect("the frame is written");
        }
        let record = TheTimedRecord::at(&scratch.folder("record").join("smoozy.sqlite3"));
        let server_row = a_server_row_for(&record);
        // **The large frame takes about two seconds on its own session's
        // line**; a small one about a tenth of that, plus its round trips.
        let machine = Arc::new(AMachine {
            program,
            home: scratch.at.clone(),
            limit: 10,
            round_trip: Duration::from_millis(20),
            each_sessions_answers_a_second: 2_000_000,
            ..AMachine::default()
        });
        let point = APointLending::over(&machine).await;
        let the_run = Arc::new(TheRunsTransfers::for_this_run());
        let run = a_run();
        let door: &dyn TheRecordADownloadWrites = record.as_ref();
        let mut arrangement = the_arrangement_is_made_at_submission(
            &mut TheArrangementsKeeper { record: door },
            true,
            WhatTheSubmissionChose {
                job: Identity::issue(IdentityKind::Job).expect("random bytes"),
                server_row,
                destination: the_bytes_of(&chosen),
                narrowing: TheNarrowing::Everything,
            },
        )
        .expect("the arrangement is made");
        let entries: Vec<ANarrowedEntry> = (1..=frames)
            .map(|frame| ANarrowedEntry {
                predicted_path: format!("frame_{frame:05}.png").into_bytes(),
                frame: Some(frame as i64),
            })
            .collect();
        let order = TheOrder::The(TheAttemptsOrder::as_the_attempt_record_carries_it(
            (1..=frames as i64).collect(),
        ));
        let folder = the_bytes_of(&outputs);
        let mut running = TheArrangementsRun::fresh();
        let mut queue = TheFetchQueue::at_a_server();
        let mut probing = TheProbesCadence::fresh();
        running.a_connection_began();
        running.the_ending_arrived(smoozy_queue_history::ending::Ending::Finished);
        let window = TheWindow {
            the_desired_state_is_on: true,
            a_connection_to_the_server_stands: true,
            the_attempts_window_is_open: true,
        };
        let in_hand = || WhatIsInHand {
            the_job_output_folder: &folder,
            narrowed_entries: &entries,
            the_whole_may_grow: false,
            the_entries_the_narrowing_excludes: &[],
            order: &order,
            the_jobs_label: None,
            the_first_frame_pair: None,
            the_pairs_other_half_fetched: &[],
            window,
            the_placed_copies: None,
            the_four_values_of_the_placed_copies: &[],
            arrangements_walking_this_server: 1,
        };
        one_turn_of_an_arrangement(
            &mut running,
            &point.sessions,
            door,
            &mut queue,
            &mut arrangement,
            in_hand(),
        )
        .await
        .expect("the ending walk ran");
        assert_eq!(
            queue.owed(),
            frames as usize,
            "every frame became fetchable"
        );

        // **When each frame first stands at its own name**, read off the
        // destination every few milliseconds while the drain runs.
        let landed: Arc<Mutex<HashMap<u32, Instant>>> = Arc::new(Mutex::new(HashMap::new()));
        let watching = Arc::new(AtomicBool::new(true));
        let watcher = tokio::spawn({
            let landed = Arc::clone(&landed);
            let watching = Arc::clone(&watching);
            let chosen = chosen.clone();
            async move {
                while watching.load(Ordering::SeqCst) {
                    for folder in std::fs::read_dir(&chosen).into_iter().flatten().flatten() {
                        for frame in 1..=frames {
                            if folder.path().join(format!("frame_{frame:05}.png")).exists() {
                                landed
                                    .lock()
                                    .expect("the landings")
                                    .entry(frame)
                                    .or_insert_with(Instant::now);
                            }
                        }
                    }
                    tokio::time::sleep(Duration::from_millis(5)).await;
                }
            }
        });
        let mut owing = [AnArrangementOwingFetches {
            running: &mut running,
            queue: &mut queue,
            probing: &mut probing,
            arrangement: &mut arrangement,
            in_hand: in_hand(),
        }];
        let came = the_owed_fetches_come_down(
            &mut owing,
            &point.sessions,
            &the_run,
            door,
            &run,
            Duration::from_secs(60),
        )
        .await;
        tokio::time::sleep(Duration::from_millis(30)).await;
        watching.store(false, Ordering::SeqCst);
        let _ = watcher.await;
        assert_eq!(
            came.fetched, frames as usize,
            "every frame came down: {came:?}"
        );
        for frame in 1..=frames {
            let landed_at = std::fs::read_dir(&chosen)
                .expect("the destination")
                .filter_map(Result::ok)
                .map(|folder| folder.path().join(format!("frame_{frame:05}.png")))
                .find(|path| path.exists())
                .expect("the frame landed");
            assert!(it_holds_the_frame(
                &landed_at,
                frame as u64 * 1_000_000,
                size_of(frame)
            ));
        }
        let landed = landed.lock().expect("the landings").clone();
        let the_large_one = *landed.get(&1).expect("the large frame was seen landing");
        let small_ones_before = (2..=frames)
            .filter(|frame| landed.get(frame).is_some_and(|at| *at < the_large_one))
            .count();
        println!(
            "IN TURN: {small_ones_before} of {SMALL_FRAMES} small frames landed while the large one \
             moved"
        );
        assert!(
            small_ones_before >= 3,
            "only {small_ones_before} of {SMALL_FRAMES} small frames landed while the large one moved: \
             a channel waited for the other's frame before taking the next"
        );
    }

    /// **A frame landed whole**: its bytes are the pattern from `from`.
    fn it_holds_the_frame(path: &Path, from: u64, size: usize) -> bool {
        let Ok(read) = std::fs::read(path) else {
            return false;
        };
        let mut wanted = vec![0u8; size];
        the_pattern_at(from, &mut wanted);
        read == wanted
    }

    /// **Every transfer record's ending, oldest first.**
    fn the_endings(record: &TheTimedRecord) -> Vec<Option<smoozy_server_protocol::transfers::Ending>> {
        let held = record.record.lock().expect("the record");
        smoozy_desktop::record::transfer_records::all(held.connection())
            .expect("the transfer records read")
            .into_iter()
            .map(|one| one.ending)
            .collect()
    }

    /// **The one transfer record that reads *interrupted*** — the file the cut
    /// fell on, whichever it was: a download walks the machine's folder in the
    /// order the machine lists it.
    fn the_interrupted_one(record: &TheTimedRecord) -> smoozy_server_protocol::transfers::TransferRecord {
        let held = record.record.lock().expect("the record");
        let mut found: Vec<_> = smoozy_desktop::record::transfer_records::all(held.connection())
            .expect("the transfer records read")
            .into_iter()
            .filter(|one| one.ending == Some(smoozy_server_protocol::transfers::Ending::Interrupted))
            .collect();
        assert_eq!(found.len(), 1, "one file was cut");
        found.remove(0)
    }

    /// **The line is cut once the first file is across and the next moving,
    /// and the period ends with it** — the cut a *Disconnect* makes (`File
    /// Browser` U7's paragraph of 2026-09-28, `Download` M13's), as
    /// [`a_request_whose_line_was_cut_reads_ended_and_not_running`] makes it.
    ///
    /// **It waits for the file in flight to pass two checkpoints**, so its
    /// record holds a checkpoint a take-up can carry on from — a part cut
    /// before its first checkpoint is started over (`Server protocol and
    /// transfers` S12 step 2), which is not the case the desk checks are about.
    async fn cut_by_a_disconnect(
        record: &TheTimedRecord,
        machine: &AMachine,
        point: &APointLending,
        the_run: &TheRunsTransfers,
        the_server: &str,
        request: &str,
    ) {
        let _ = record;
        let two_checkpoints = 2 * smoozy_server_protocol::transfers::moving::THE_CHECKPOINT_INTERVAL + 1_000_000;
        // **The two small parts are across and the large one moves alone**
        // ([`three_parts_one_large`]; step 10c's eleventh-pass fix piece 4).
        until("the file in flight passes two checkpoints", Duration::from_secs(60), || {
            the_run.what_is_under_way().get(request).is_some_and(|under_way| {
                under_way.moving.len() == 1
                    && under_way
                        .moving
                        .iter()
                        .any(|file| file.report.bytes_staged >= two_checkpoints && file.report.bytes_staged < file.report.bytes_expected)
            }) && record.moved.lock().expect("the moments").len() >= 2
        })
        .await;
        machine.every_session_ends();
        point.lending.abort();
        the_run.a_servers_period_ended(the_server);
    }

    /// **An upload a *Disconnect* cut is told as a leftover, and asking again
    /// for the cut file takes its staged part up** — step 10c's transfer wave
    /// piece 7, the map's first two desk checks, upload half (`File Browser`
    /// U8's first and second bullets; `Design/Transfers § What a past sitting
    /// left behind`).
    ///
    /// **The fault as a person met it**: after a *Disconnect* the request read
    /// *— did not finish* and nothing more — no word that part-sent files were
    /// still on the server and no way to remove them — and asking again for
    /// the same file sent it again from its first byte.
    #[tokio::test(flavor = "multi_thread", worker_threads = 4)]
    async fn an_upload_a_disconnect_cut_is_a_leftover_and_asking_again_takes_it_up() {
        let _deadline = TheTestsDeadline::started();
        let program = the_server_or_say_so!();

        let size = 24_000_000u64;

        let scratch = Scratch::new("leftover-up");
        let here = scratch.folder("here/three");
        three_parts_one_large(&here, size);
        let machine_side = scratch.folder("machine");
        let record = TheTimedRecord::at(&scratch.folder("record").join("smoozy.sqlite3"));
        let (server_row, installation) = a_server_row_installed(&record);
        let the_server = server_row.as_text();
        let machine = Arc::new(AMachine {
            program,
            home: scratch.at.clone(),
            limit: 10,
            ..AMachine::default()
        });
        let point = APointLending::over(&machine).await;
        let the_run = Arc::new(TheRunsTransfers::for_this_run());
        let cap = TheCap::Of(BytesPerSecond(4_000_000));
        the_run.the_caps_saved_are(cap, cap);
        let as_a_door: Arc<dyn TheRecordAnUploadWrites> = record.clone();

        let asked = WhatIsBeingUploaded {
            server_row: server_row.clone(),
            installation: Some(installation.clone()),
            destination: the_bytes_of(&machine_side),
            chosen: vec![here.clone()],
            run: a_run(),
        };
        let mut ready = an_upload_is_prepared(&asked, &point.sessions, as_a_door.as_ref())
            .await
            .expect("the upload is prepared");
        assert_eq!(
            the_answer_is_applied(&mut ready, None, &point.sessions, as_a_door.as_ref()).await,
            WhatTheAnswerLeft::ItGoes
        );
        let request = ready.request.record.as_text();
        let (came_to, ()) = tokio::join!(
            the_upload_runs(ready, &point.sessions, &the_run, Arc::clone(&as_a_door), &NobodyIsTold),
            cut_by_a_disconnect(&record, &machine, &point, &the_run, &the_server, &request)
        );
        assert_eq!(came_to.uploaded, 2, "{came_to:?}");
        println!("LEFTOVER UP: the endings after the cut {:?}.", the_endings(&record));

        // ---- what a person looking at Transfers reads now
        let mut missed = Vec::new();
        // **With *Show hidden files* off** — a fresh record's own state, and
        // test 8 of that checkbox's plan text: the staging folder is left out
        // of the browser's list, and the leftover is still told here and can
        // still be removed (`File Browser` L21, `File Browser` X12 as amended 2026-10-02).
        let the_box = {
            let held = record.record.lock().expect("the record");
            smoozy_desktop::record::settings::hidden_files_are_shown(held.connection()).ok()
        };
        a_figure!(missed, the_box == Some(false), "LEFTOVER UP: Show hidden files reads {the_box:?}, not off");
        a_figure!(
            missed,
            machine_side.join("three").join(".smoozy-staging").is_dir(),
            "LEFTOVER UP: the part is not in the hidden staging folder"
        );
        let screen = the_screen_now(&record, &the_run);
        let row = the_row(&screen, &request).expect("the request is on the screen");
        let control = row.control.as_ref().map(|control| control.act.as_str().to_string());
        println!(
            "LEFTOVER UP: the row reads «{}» · «{}» · «{}», control {control:?}.",
            as_read(&row.name),
            as_read(&row.count),
            as_read(&row.where_it_goes)
        );
        a_figure!(missed, control.as_deref() == Some("remove"), "LEFTOVER UP: the row offers {control:?}, not Remove");
        a_figure!(
            missed,
            as_read(&row.where_it_goes).contains("partial files still use space on the server"),
            "LEFTOVER UP: the row does not say partial files are still on the server"
        );
        a_figure!(
            missed,
            !as_read(&row.where_it_goes).contains("stopped when Smoozy closed"),
            "LEFTOVER UP: the row says Smoozy closed, where a Disconnect ended it"
        );
        a_figure!(missed, row.bar.is_none(), "LEFTOVER UP: the leftover draws a bar");

        // ---- Connect again, and ask for the cut file again
        let point = APointLending::over(&machine).await;
        let cut_file = here.join("part_1.blend");
        let landing_folder = machine_side.join("three");
        let asked = WhatIsBeingUploaded {
            server_row: server_row.clone(),
            installation: Some(installation.clone()),
            destination: the_bytes_of(&landing_folder),
            chosen: vec![cut_file.clone()],
            run: a_run(),
        };
        let mut ready = an_upload_is_prepared(&asked, &point.sessions, as_a_door.as_ref())
            .await
            .expect("the second upload is prepared");
        assert_eq!(
            the_answer_is_applied(&mut ready, None, &point.sessions, as_a_door.as_ref()).await,
            WhatTheAnswerLeft::ItGoes
        );
        let again = ready.request.record.as_text();
        let watched = TheFileWatched::over(&the_run, again.clone());
        let came_to = the_upload_runs(ready, &point.sessions, &the_run, Arc::clone(&as_a_door), &NobodyIsTold).await;
        let looks = watched.ended();
        let verified = looks.iter().find_map(|look| look.verifying);
        let lowest = looks.iter().map(|look| look.staged).filter(|staged| *staged > 0).min();
        println!(
            "LEFTOVER UP, ASKED AGAIN: came to {came_to:?}; the part verified {verified:?}; the \
             lowest figure above nought {lowest:?}; left staged {:?}.",
            what_is_left_staged(&landing_folder)
        );
        a_figure!(missed, came_to.uploaded == 1, "LEFTOVER UP, ASKED AGAIN: came to {came_to:?}");
        a_figure!(
            missed,
            verified.is_some(),
            "LEFTOVER UP, ASKED AGAIN: the part already sent was never verified — it was not taken up"
        );
        a_figure!(
            missed,
            lowest.is_some_and(|lowest| lowest >= smoozy_server_protocol::transfers::moving::THE_CHECKPOINT_INTERVAL),
            "LEFTOVER UP, ASKED AGAIN: the file was sent from {lowest:?}, not from the part already there"
        );
        a_figure!(
            missed,
            it_holds_the_pattern(&landing_folder.join("part_1.blend"), size),
            "LEFTOVER UP, ASKED AGAIN: the file on the machine is not the file that was sent"
        );
        a_figure!(
            missed,
            what_is_left_staged(&landing_folder).is_empty(),
            "LEFTOVER UP, ASKED AGAIN: something is left staged"
        );
        let screen = the_screen_now(&record, &the_run);
        let row = the_row(&screen, &request).expect("the first request is still on the screen");
        a_figure!(
            missed,
            row.control.as_ref().map(|control| control.act.as_str()) != Some("remove"),
            "LEFTOVER UP, ASKED AGAIN: the part was taken up and the first request still offers Remove"
        );
        the_figures_held("THE LEFTOVER, UP", missed);
    }

    /// **A file changed between a cut upload and asking for it again is sent
    /// from its start, and its row says so** — `File Browser` K10 clause 2
    /// (`Dependency Upload & Remap` A10 adopted) and K12, *the one
    /// unchanged-source answer, for every upload of a local source*;
    /// `Specs/File Browser`: *"Smoozy carries on only where it can tell the
    /// file is unchanged, and sends that file again from the beginning where it
    /// cannot."* The two transfer faults after the 38's pieces 7 and 9, fault 1.
    ///
    /// **The fault as it stood**: Files took the part up whatever the file now
    /// held. The loop's own resume checks the bytes on the server against the
    /// record, never the file on this computer, so it carried on over the old
    /// first part, sent the rest, and `File Browser` K9's gate then refused it:
    /// the user's own request for the file as it is now ended *It changed
    /// while it was uploading — nothing was written*, after the whole file had
    /// crossed.
    #[tokio::test(flavor = "multi_thread", worker_threads = 4)]
    async fn an_upload_asked_for_again_after_the_file_changed_starts_over_and_says_so() {
        let _deadline = TheTestsDeadline::started();
        let program = the_server_or_say_so!();

        let size = 24_000_000u64;

        let scratch = Scratch::new("leftover-up-changed");
        let here = scratch.folder("here/three");
        three_parts_one_large(&here, size);
        let machine_side = scratch.folder("machine");
        let record = TheTimedRecord::at(&scratch.folder("record").join("smoozy.sqlite3"));
        let (server_row, installation) = a_server_row_installed(&record);
        let the_server = server_row.as_text();
        let machine = Arc::new(AMachine {
            program,
            home: scratch.at.clone(),
            limit: 10,
            ..AMachine::default()
        });
        let point = APointLending::over(&machine).await;
        let the_run = Arc::new(TheRunsTransfers::for_this_run());
        let cap = TheCap::Of(BytesPerSecond(4_000_000));
        the_run.the_caps_saved_are(cap, cap);
        let as_a_door: Arc<dyn TheRecordAnUploadWrites> = record.clone();

        let asked = WhatIsBeingUploaded {
            server_row: server_row.clone(),
            installation: Some(installation.clone()),
            destination: the_bytes_of(&machine_side),
            chosen: vec![here.clone()],
            run: a_run(),
        };
        let mut ready = an_upload_is_prepared(&asked, &point.sessions, as_a_door.as_ref())
            .await
            .expect("the upload is prepared");
        assert_eq!(
            the_answer_is_applied(&mut ready, None, &point.sessions, as_a_door.as_ref()).await,
            WhatTheAnswerLeft::ItGoes
        );
        let request = ready.request.record.as_text();
        let (came_to, ()) = tokio::join!(
            the_upload_runs(ready, &point.sessions, &the_run, Arc::clone(&as_a_door), &NobodyIsTold),
            cut_by_a_disconnect(&record, &machine, &point, &the_run, &the_server, &request)
        );
        assert_eq!(came_to.uploaded, 2, "{came_to:?}");

        // ---- the file is saved again while the line is down: its first
        // megabyte changes and its size does not
        let cut_file = here.join("part_1.blend");
        {
            use std::io::{Seek, SeekFrom};
            let mut file = std::fs::OpenOptions::new()
                .write(true)
                .open(&cut_file)
                .expect("the cut file opens for writing");
            file.seek(SeekFrom::Start(0)).expect("the start of the file");
            file.write_all(&vec![0xAB; 1_000_000]).expect("the first part is rewritten");
        }
        let the_file_now = std::fs::read(&cut_file).expect("the changed file reads");

        // ---- Connect again, and ask for the cut file again
        let point = APointLending::over(&machine).await;
        let landing_folder = machine_side.join("three");
        let asked = WhatIsBeingUploaded {
            server_row: server_row.clone(),
            installation: Some(installation.clone()),
            destination: the_bytes_of(&landing_folder),
            chosen: vec![cut_file.clone()],
            run: a_run(),
        };
        let mut ready = an_upload_is_prepared(&asked, &point.sessions, as_a_door.as_ref())
            .await
            .expect("the second upload is prepared");
        assert_eq!(
            the_answer_is_applied(&mut ready, None, &point.sessions, as_a_door.as_ref()).await,
            WhatTheAnswerLeft::ItGoes
        );
        let again = ready.request.record.as_text();
        let watched = TheFileWatched::over(&the_run, again.clone());
        let came_to = the_upload_runs(ready, &point.sessions, &the_run, Arc::clone(&as_a_door), &NobodyIsTold).await;
        let looks = watched.ended();
        let verified = looks.iter().find_map(|look| look.verifying);
        let cause = looks.iter().find_map(|look| look.cause);
        let highest = looks.iter().map(|look| look.staged).max();
        let landed = std::fs::read(landing_folder.join("part_1.blend")).ok();
        println!(
            "CHANGED, ASKED AGAIN: came to {came_to:?}; the part verified {verified:?}; started \
             over {cause:?}; the highest figure {highest:?}; the file on the machine is the file \
             now {}; left staged {:?}.",
            landed.as_deref() == Some(the_file_now.as_slice()),
            what_is_left_staged(&landing_folder)
        );
        let mut missed = Vec::new();
        a_figure!(missed, came_to.uploaded == 1, "CHANGED, ASKED AGAIN: came to {came_to:?}, not uploaded");
        a_figure!(
            missed,
            verified.is_none(),
            "CHANGED, ASKED AGAIN: the part sent before the change was taken up and carried on from"
        );
        a_figure!(
            missed,
            cause == Some(smoozy_server_protocol::transfers::StartOverCause::TheStagedPrefixDidNotVerify),
            "CHANGED, ASKED AGAIN: the row never said it was uploading the file from the start ({cause:?})"
        );
        a_figure!(
            missed,
            landed.as_deref() == Some(the_file_now.as_slice()),
            "CHANGED, ASKED AGAIN: the file on the machine is not the file as it is now"
        );
        the_figures_held("THE LEFTOVER, UP, CHANGED", missed);
    }

    /// **A manual download a *Disconnect* cut is told as a leftover in the
    /// folder it was coming to** — step 10c's transfer wave piece 7, the map's
    /// first desk check, download half (`Download` M13's second bullet;
    /// `Design/Transfers § What a past sitting left behind`: *a download's is
    /// on the user's own computer, inside the folder the files were coming
    /// to*).
    #[tokio::test(flavor = "multi_thread", worker_threads = 4)]
    async fn a_download_a_disconnect_cut_is_a_leftover_and_remove_leaves_nothing() {
        let _deadline = TheTestsDeadline::started();
        let program = the_server_or_say_so!();

        let size = 24_000_000u64;

        let scratch = Scratch::new("leftover-down");
        let on_the_machine = scratch.folder("machine/three");
        three_parts_one_large(&on_the_machine, size);
        let chosen = scratch.folder("chosen");
        let record = TheTimedRecord::at(&scratch.folder("record").join("smoozy.sqlite3"));
        let (server_row, installation) = a_server_row_installed(&record);
        let the_server = server_row.as_text();
        let machine = Arc::new(AMachine {
            program,
            home: scratch.at.clone(),
            limit: 10,
            ..AMachine::default()
        });
        let point = APointLending::over(&machine).await;
        let the_run = Arc::new(TheRunsTransfers::for_this_run());
        let cap = TheCap::Of(BytesPerSecond(4_000_000));
        the_run.the_caps_saved_are(cap, cap);

        let asked = WhatIsBeingDownloaded {
            server_row: server_row.clone(),
            installation: Some(installation.clone()),
            destination: the_bytes_of(&chosen),
            picked: vec![the_bytes_of(&on_the_machine)],
            run: a_run(),
        };
        let mut ready = a_download_is_prepared(
            asked,
            &point.sessions,
            record.as_ref() as &dyn TheRecordADownloadWrites,
            &NoJobIsKnown,
        )
        .await
        .expect("the download is prepared");
        the_request_goes_without_a_question(&mut ready, record.as_ref())
            .expect("the dispositions are written");
        let request = ready.request.record.as_text();
        let (came_to, ()) = tokio::join!(
            the_download_runs(&mut ready, &point.sessions, &the_run, record.as_ref()),
            cut_by_a_disconnect(&record, &machine, &point, &the_run, &the_server, &request)
        );
        assert_eq!(came_to.fetched, 2, "{came_to:?}");
        println!(
            "LEFTOVER DOWN: the endings after the cut {:?}; staged {:?}.",
            the_endings(&record),
            what_is_left_staged(&chosen)
        );

        let mut missed = Vec::new();
        let screen = the_screen_now(&record, &the_run);
        let row = the_row(&screen, &request).expect("the request is on the screen");
        let control = row.control.as_ref().map(|control| control.act.as_str().to_string());
        println!(
            "LEFTOVER DOWN: the row reads «{}» · «{}» · «{}», control {control:?}.",
            as_read(&row.name),
            as_read(&row.count),
            as_read(&row.where_it_goes)
        );
        a_figure!(missed, control.as_deref() == Some("remove"), "LEFTOVER DOWN: the row offers {control:?}, not Remove");
        a_figure!(
            missed,
            as_read(&row.where_it_goes).contains("partial files are still in"),
            "LEFTOVER DOWN: the row does not say partial files are still in the folder"
        );
        a_figure!(
            missed,
            !what_is_left_staged(&chosen).is_empty(),
            "LEFTOVER DOWN: nothing was staged to remove"
        );

        // ---- Remove
        let removed = smoozy_desktop::downloading::a_leftover_is_removed(
            record.as_ref(),
            &the_run,
            &Identity::read_back(IdentityKind::Operation, &request).expect("the request"),
            &server_row,
            Direction::Down,
        );
        let cut = the_endings(&record);
        println!(
            "LEFTOVER DOWN, REMOVED: {removed:?}; the endings {cut:?}; staged {:?}.",
            what_is_left_staged(&chosen)
        );
        a_figure!(missed, removed.removed == 1, "LEFTOVER DOWN, REMOVED: came to {removed:?}");
        a_figure!(
            missed,
            what_is_left_staged(&chosen).is_empty(),
            "LEFTOVER DOWN, REMOVED: something is still staged"
        );
        a_figure!(
            missed,
            cut.contains(&Some(smoozy_server_protocol::transfers::Ending::Stopped(
                smoozy_server_protocol::transfers::StoppedReason::StagingRemovedByUser
            ))),
            "LEFTOVER DOWN, REMOVED: no record reads stopped, staging removed by user: {cut:?}"
        );
        let screen = the_screen_now(&record, &the_run);
        let row = the_row(&screen, &request).expect("the request is still on the screen");
        a_figure!(
            missed,
            row.control.as_ref().map(|control| control.act.as_str()) == Some("files-on-your-computer")
                && !as_read(&row.where_it_goes).contains("partial files"),
            "LEFTOVER DOWN, REMOVED: the row still reads as a leftover: «{}», {:?}",
            as_read(&row.where_it_goes),
            row.control.as_ref().map(|control| control.act.as_str())
        );
        the_figures_held("THE LEFTOVER, DOWN", missed);
    }

    /// **Skip existing on a file whose part a *Disconnect* left abandons the
    /// part, and says so in the record first** — `Server protocol and
    /// transfers` S28a, `Download` M13's third bullet: *skip existing does not
    /// take it up at all*, and the module that abandons an *interrupted*
    /// transfer's staging writes it *stopped, abandoned by its request* before
    /// anything else, or no sweep ever reaches it. Step 10c's transfer wave
    /// piece 7, the map's third desk check, download half.
    #[tokio::test(flavor = "multi_thread", worker_threads = 4)]
    async fn skip_existing_abandons_a_downloads_leftover_to_the_sweep() {
        use smoozy_file_browser::already_exists::CollisionAnswer;
        use smoozy_server_protocol::transfers::{Ending, StoppedReason};
        let _deadline = TheTestsDeadline::started();
        let program = the_server_or_say_so!();

        let size = 24_000_000u64;

        let scratch = Scratch::new("skip-down");
        let on_the_machine = scratch.folder("machine/three");
        three_parts_one_large(&on_the_machine, size);
        let chosen = scratch.folder("chosen");
        let record = TheTimedRecord::at(&scratch.folder("record").join("smoozy.sqlite3"));
        let (server_row, installation) = a_server_row_installed(&record);
        let the_server = server_row.as_text();
        let machine = Arc::new(AMachine {
            program,
            home: scratch.at.clone(),
            limit: 10,
            ..AMachine::default()
        });
        let point = APointLending::over(&machine).await;
        let the_run = Arc::new(TheRunsTransfers::for_this_run());
        let cap = TheCap::Of(BytesPerSecond(4_000_000));
        the_run.the_caps_saved_are(cap, cap);

        let asked = |picked: PathBuf, into: &Path| WhatIsBeingDownloaded {
            server_row: server_row.clone(),
            installation: Some(installation.clone()),
            destination: the_bytes_of(into),
            picked: vec![the_bytes_of(&picked)],
            run: a_run(),
        };
        let mut ready = a_download_is_prepared(
            asked(on_the_machine.clone(), &chosen),
            &point.sessions,
            record.as_ref() as &dyn TheRecordADownloadWrites,
            &NoJobIsKnown,
        )
        .await
        .expect("the download is prepared");
        the_request_goes_without_a_question(&mut ready, record.as_ref())
            .expect("the dispositions are written");
        let request = ready.request.record.as_text();
        let (_, ()) = tokio::join!(
            the_download_runs(&mut ready, &point.sessions, &the_run, record.as_ref()),
            cut_by_a_disconnect(&record, &machine, &point, &the_run, &the_server, &request)
        );
        let landed_in = chosen.join("three");
        // **A download stages in the request's own destination**, which is
        // `chosen` here (`Server protocol and transfers` S1, `Download` M13).
        let staged_before = what_is_left_staged(&chosen);
        assert_eq!(staged_before.len(), 1, "one part was left: {staged_before:?}");

        // **The user already has the file** — it stands at its landing name —
        // and asks for it again, answering *Skip existing*.
        let cut = PathBuf::from(the_interrupted_one(&record).source_path);
        let named = cut.file_name().expect("a name").to_owned();
        std::fs::copy(&cut, landed_in.join(&named)).expect("the file is put where it lands");
        let point = APointLending::over(&machine).await;
        let mut ready = a_download_is_prepared(
            asked(cut.clone(), &landed_in),
            &point.sessions,
            record.as_ref() as &dyn TheRecordADownloadWrites,
            &NoJobIsKnown,
        )
        .await
        .expect("the second download is prepared");
        assert_eq!(ready.colliding.len(), 1, "the file stands at its landing name");
        smoozy_desktop::downloading::the_answer_is_applied(
            &mut ready,
            record.as_ref(),
            CollisionAnswer::SkipExisting,
        )
        .expect("the answer is written");
        let came_to = the_download_runs(&mut ready, &point.sessions, &the_run, record.as_ref()).await;
        let endings = the_endings(&record);
        println!("SKIP DOWN: came to {came_to:?}; the endings {endings:?}.");
        let mut missed = Vec::new();
        a_figure!(missed, came_to.skipped == 1, "SKIP DOWN: came to {came_to:?}");
        a_figure!(
            missed,
            !endings.contains(&Some(Ending::Interrupted)),
            "SKIP DOWN: the part's record still reads interrupted, so no sweep will ever reach it"
        );
        a_figure!(
            missed,
            endings.contains(&Some(Ending::Stopped(StoppedReason::AbandonedByItsRequest))),
            "SKIP DOWN: no record reads stopped, abandoned by its request"
        );
        the_figures_held("SKIP EXISTING, DOWN", missed);
    }

    /// **Asking again for a file a *Disconnect* cut in a download takes its
    /// part up** — `Download` M13's third bullet: the unit continues the
    /// interrupted transfer, its identity, record and staging path. **A
    /// download's resume starts over** (`Server protocol and transfers` S13's
    /// four values are read by nothing), so what the take-up buys is that the
    /// part is the transfer's own superseded attempt and goes once the file
    /// lands, and the earlier request is no longer a leftover. Step 10c's
    /// transfer wave piece 7, the map's second desk check, download half.
    #[tokio::test(flavor = "multi_thread", worker_threads = 4)]
    async fn a_downloads_leftover_is_taken_up_by_asking_again() {
        let _deadline = TheTestsDeadline::started();
        let program = the_server_or_say_so!();

        let size = 24_000_000u64;
        let scratch = Scratch::new("takeup-down");
        let on_the_machine = scratch.folder("machine/three");
        three_parts_one_large(&on_the_machine, size);
        let chosen = scratch.folder("chosen");
        let record = TheTimedRecord::at(&scratch.folder("record").join("smoozy.sqlite3"));
        let (server_row, installation) = a_server_row_installed(&record);
        let the_server = server_row.as_text();
        let machine = Arc::new(AMachine {
            program,
            home: scratch.at.clone(),
            limit: 10,
            ..AMachine::default()
        });
        let point = APointLending::over(&machine).await;
        let the_run = Arc::new(TheRunsTransfers::for_this_run());
        let cap = TheCap::Of(BytesPerSecond(4_000_000));
        the_run.the_caps_saved_are(cap, cap);
        let asked = |picked: PathBuf, into: &Path| WhatIsBeingDownloaded {
            server_row: server_row.clone(),
            installation: Some(installation.clone()),
            destination: the_bytes_of(into),
            picked: vec![the_bytes_of(&picked)],
            run: a_run(),
        };
        let mut ready = a_download_is_prepared(
            asked(on_the_machine.clone(), &chosen),
            &point.sessions,
            record.as_ref() as &dyn TheRecordADownloadWrites,
            &NoJobIsKnown,
        )
        .await
        .expect("the download is prepared");
        the_request_goes_without_a_question(&mut ready, record.as_ref())
            .expect("the dispositions are written");
        let first = ready.request.record.as_text();
        let (_, ()) = tokio::join!(
            the_download_runs(&mut ready, &point.sessions, &the_run, record.as_ref()),
            cut_by_a_disconnect(&record, &machine, &point, &the_run, &the_server, &first)
        );
        let cut = the_interrupted_one(&record);
        assert_eq!(what_is_left_staged(&chosen).len(), 1, "one part was left");
        // **A person looking at Transfers now reads a leftover** — and the
        // screen has read what the request left, which the take-up then moves.
        let screen = the_screen_now(&record, &the_run);
        assert_eq!(
            the_row(&screen, &first)
                .and_then(|row| row.control.as_ref())
                .map(|control| control.act.as_str()),
            Some("remove"),
            "the cut request reads as a leftover"
        );

        // ---- Connect again, and ask for the cut file again, landing where it
        // was going
        let point = APointLending::over(&machine).await;
        let mut ready = a_download_is_prepared(
            asked(PathBuf::from(&cut.source_path), &chosen.join("three")),
            &point.sessions,
            record.as_ref() as &dyn TheRecordADownloadWrites,
            &NoJobIsKnown,
        )
        .await
        .expect("the second download is prepared");
        the_request_goes_without_a_question(&mut ready, record.as_ref())
            .expect("the dispositions are written");
        let came_to = the_download_runs(&mut ready, &point.sessions, &the_run, record.as_ref()).await;
        let taken = ready.request.entries[0].transfer.as_ref().map(Identity::as_text);
        let mut missed = Vec::new();
        println!(
            "TAKE-UP DOWN: came to {came_to:?}; the unit's transfer {taken:?}, the cut one {}; staged \
             {:?} and {:?}.",
            cut.transfer.as_text(),
            what_is_left_staged(&chosen),
            what_is_left_staged(&chosen.join("three"))
        );
        a_figure!(missed, came_to.fetched == 1, "TAKE-UP DOWN: came to {came_to:?}");
        a_figure!(
            missed,
            taken.as_deref() == Some(cut.transfer.as_text().as_str()),
            "TAKE-UP DOWN: the unit did not take up the cut transfer"
        );
        a_figure!(
            missed,
            it_holds_the_pattern(&PathBuf::from(&cut.final_path), size),
            "TAKE-UP DOWN: the file on this computer is not the file on the machine"
        );
        a_figure!(
            missed,
            what_is_left_staged(&chosen).is_empty() && what_is_left_staged(&chosen.join("three")).is_empty(),
            "TAKE-UP DOWN: something is left staged"
        );
        let screen = the_screen_now(&record, &the_run);
        let row = the_row(&screen, &first).expect("the first request is on the screen");
        a_figure!(
            missed,
            row.control.as_ref().map(|control| control.act.as_str()) != Some("remove"),
            "TAKE-UP DOWN: the first request still offers Remove"
        );
        the_figures_held("TAKE-UP, DOWN", missed);
    }

    // ------------- the download question's answers (thirteenth-pass piece 3)

    /// **One file on the machine, and a file of the user's own already standing
    /// at the name it lands at** — the thirteenth pass's *Replace* and *Keep
    /// both*, through the application's real route: the real point, the
    /// machine's own `sftp-server`, the real record and the platform file
    /// adapter writing into a real folder. Added 2026-10-01, step 10c's
    /// thirteenth-pass fix piece 3.
    struct AFileAlreadyThere {
        scratch: Scratch,
        chosen: PathBuf,
        on_the_machine: PathBuf,
        machine: Arc<AMachine>,
        record: Arc<TheTimedRecord>,
        server_row: Identity,
        installation: Identity,
        the_run: Arc<TheRunsTransfers>,
    }

    /// The bytes of the file on the machine, and of the user's own copy —
    /// different, so a file that was replaced reads as the machine's.
    const THE_MACHINES_BYTES: u8 = 0x4d;
    const THE_USERS_OWN_BYTES: u8 = 0x55;

    fn a_file_already_there(named: &str, program: PathBuf, file: &str) -> AFileAlreadyThere {
        let scratch = Scratch::new(named);
        let on_the_machine = scratch.folder("machine").join(file);
        std::fs::write(&on_the_machine, vec![THE_MACHINES_BYTES; 65_536]).expect("the machine's file");
        let chosen = scratch.folder("chosen");
        std::fs::write(chosen.join(file), vec![THE_USERS_OWN_BYTES; 1_000]).expect("the user's own file");
        let record = TheTimedRecord::at(&scratch.folder("record").join("smoozy.sqlite3"));
        let (server_row, installation) = a_server_row_installed(&record);
        let machine = Arc::new(AMachine {
            program,
            home: scratch.at.clone(),
            limit: 10,
            ..AMachine::default()
        });
        AFileAlreadyThere {
            scratch,
            chosen,
            on_the_machine,
            machine,
            record,
            server_row,
            installation,
            the_run: Arc::new(TheRunsTransfers::for_this_run()),
        }
    }

    impl AFileAlreadyThere {
        /// **The file asked for again**, answered as `answer` says — or, with
        /// none, sent without the question, the way a request with nothing
        /// standing at its names goes (`Download` M6). `before_it_runs` is
        /// done between the question and the first byte.
        async fn asked_for(
            &self,
            answer: Option<smoozy_file_browser::already_exists::CollisionAnswer>,
            before_it_runs: impl FnOnce(),
        ) -> (String, String) {
            let point = APointLending::over(&self.machine).await;
            let mut ready = a_download_is_prepared(
                WhatIsBeingDownloaded {
                    server_row: self.server_row.clone(),
                    installation: Some(self.installation.clone()),
                    destination: the_bytes_of(&self.chosen),
                    picked: vec![the_bytes_of(&self.on_the_machine)],
                    run: a_run(),
                },
                &point.sessions,
                self.record.as_ref() as &dyn TheRecordADownloadWrites,
                &NoJobIsKnown,
            )
            .await
            .expect("the download is prepared");
            match answer {
                Some(answer) => {
                    assert_eq!(ready.colliding.len(), 1, "the user's own file stands at the name");
                    smoozy_desktop::downloading::the_answer_is_applied(&mut ready, self.record.as_ref(), answer)
                        .expect("the answer is written");
                }
                None => the_request_goes_without_a_question(&mut ready, self.record.as_ref())
                    .expect("the dispositions are written"),
            }
            before_it_runs();
            let came_to =
                the_download_runs(&mut ready, &point.sessions, &self.the_run, self.record.as_ref()).await;
            (ready.request.record.as_text(), format!("{came_to:?}"))
        }

        /// **The row a person reads on Transfers**, one line: the name, the
        /// count, and each file row's state, sentence and controls.
        fn the_row_reads(&self, request: &str) -> String {
            let screen = the_screen_now(&self.record, &self.the_run);
            let Some(row) = the_row(&screen, request) else {
                return "no row".to_string();
            };
            let files: Vec<String> = row
                .files
                .iter()
                .map(|file| {
                    let failure = file.failure.as_ref().map(|failure| {
                        format!(
                            "{} [{}]",
                            failure.sentence.as_str().replace('\u{a0}', " "),
                            failure
                                .controls
                                .iter()
                                .map(|control| control.act.as_str().to_string())
                                .collect::<Vec<_>>()
                                .join(", ")
                        )
                    });
                    format!("{} {} {}", file.path.as_str(), file.state, failure.unwrap_or_default())
                })
                .collect();
            format!("{} · {} | files: {files:?}", as_read(&row.name), as_read(&row.count))
        }
    }

    fn the_inode_of(path: &Path) -> Option<u64> {
        use std::os::unix::fs::MetadataExt;
        std::fs::metadata(path).ok().map(|read| read.ino())
    }

    fn it_is_all(path: &Path, byte: u8, size: usize) -> bool {
        std::fs::read(path).is_ok_and(|read| read.len() == size && read.iter().all(|one| *one == byte))
    }

    /// **The download question's *Replace* writes over the file already
    /// there** — `Download` P5 and `Desktop app and delivery` C14: the one
    /// deliberate replacement is a manual request's user-answered *Replace*,
    /// which publishes with the replacing rename. **On the thirteenth pass it
    /// wrote nothing** (same inode, same time), the record said *failed* and
    /// Transfers read *1 file into /Users/artist/Downloads · done*: the
    /// fetch was create-intended whatever the answer, so the publication met
    /// the old file and refused.
    #[tokio::test(flavor = "multi_thread", worker_threads = 4)]
    async fn replace_writes_over_the_file_already_there() {
        use smoozy_file_browser::already_exists::CollisionAnswer;
        let _deadline = TheTestsDeadline::started();
        let program = the_server_or_say_so!();

        let there = a_file_already_there("replace-down", program, "p13-cube.blend");
        let landing = there.chosen.join("p13-cube.blend");
        let before = the_inode_of(&landing);

        let (request, came_to) = there.asked_for(Some(CollisionAnswer::Replace), || {}).await;
        let reads = there.the_row_reads(&request);
        let after = the_inode_of(&landing);
        println!("REPLACE: came to {came_to}; inode {before:?} then {after:?}; the row reads «{reads}».");
        let mut missed = Vec::new();
        a_figure!(missed, came_to.contains("fetched: 1"), "REPLACE: came to {came_to}");
        a_figure!(
            missed,
            it_is_all(&landing, THE_MACHINES_BYTES, 65_536),
            "REPLACE: the file at the name is not the machine's — nothing was written over it"
        );
        a_figure!(missed, after.is_some() && after != before, "REPLACE: the file is at the same inode");
        a_figure!(
            missed,
            what_is_left_staged(&there.chosen).is_empty(),
            "REPLACE: something is left staged: {:?}",
            what_is_left_staged(&there.chosen)
        );
        drop(there.scratch);
        the_figures_held("REPLACE", missed);
    }

    /// **A download whose file could not be put in place on this computer
    /// does not read *done*** — Rule 8, `Design/Transfers § A file that
    /// failed` with E2-1. The thirteenth pass's second fault, found under the
    /// first: `a_manual_request_known` read *Finished* once every unit had an
    /// outcome, whatever the outcome was, so **every** failed manual download
    /// said *done*. Here the publication is refused the honest way: a file
    /// appears at the name between the question and the first byte, and the
    /// create-intended publication will not write over it.
    #[tokio::test(flavor = "multi_thread", worker_threads = 4)]
    async fn a_file_that_could_not_be_put_in_place_does_not_read_done() {
        let _deadline = TheTestsDeadline::started();
        let program = the_server_or_say_so!();

        let there = a_file_already_there("refused-down", program, "p13-cube.blend");
        let landing = there.chosen.join("p13-cube.blend");
        // Nothing stands at the name when the question would be asked …
        std::fs::remove_file(&landing).expect("the user's own file goes");
        // … and something does by the time the file is to be put in place.
        let (request, came_to) = there
            .asked_for(None, || {
                std::fs::write(&landing, vec![THE_USERS_OWN_BYTES; 1_000]).expect("a file appears");
            })
            .await;
        let reads = there.the_row_reads(&request);
        println!("REFUSED: came to {came_to}; the row reads «{reads}».");
        let mut missed = Vec::new();
        a_figure!(missed, came_to.contains("failed: [["), "REFUSED: came to {came_to}");
        a_figure!(
            missed,
            it_is_all(&landing, THE_USERS_OWN_BYTES, 1_000),
            "REFUSED: the file that appeared was written over"
        );
        a_figure!(missed, !reads.contains("· done"), "REFUSED: the row reads done: «{reads}»");
        a_figure!(missed, reads.contains("— did not finish"), "REFUSED: «{reads}»");
        a_figure!(missed, reads.contains("0 / 1"), "REFUSED: no count: «{reads}»");
        a_figure!(
            missed,
            reads.contains("failed Download failed. [try-again, details, clear-a-failure]"),
            "REFUSED: the failed file's row is not drawn as E2-1 has it, with its Try again: «{reads}»"
        );
        // **What *Details* answers: this computer's own words**, kept by the
        // run for the one file.
        let details = smoozy_desktop::downloading::what_this_computer_said_for(
            &request,
            &the_bytes_of(&landing),
        );
        println!("REFUSED: Details reads {details:?}.");
        a_figure!(
            missed,
            details.as_deref() == Some("File already exists. Not overwritten."),
            "REFUSED: Details reads {details:?}"
        );
        drop(there.scratch);
        the_figures_held("REFUSED", missed);
    }

    /// **Keep both names the copy before its extension, and a second Keep
    /// both counts on** — `File Browser` X9's one rule in both directions
    /// (`File Browser/src/keep_both.rs`, `split`), and `Download` M8's *first
    /// non-colliding counted suffix*. On the thirteenth pass the copy landed
    /// as *p13-cube.blend (2)*, which is no longer a `.blend`.
    #[tokio::test(flavor = "multi_thread", worker_threads = 4)]
    async fn keep_both_names_the_copy_before_its_extension_and_counts_on() {
        use smoozy_file_browser::already_exists::CollisionAnswer;
        let _deadline = TheTestsDeadline::started();
        let program = the_server_or_say_so!();

        let there = a_file_already_there("keep-both-down", program, "p13-cube.blend");

        let (first, came_first) = there.asked_for(Some(CollisionAnswer::KeepBoth), || {}).await;
        let (second, came_second) = there.asked_for(Some(CollisionAnswer::KeepBoth), || {}).await;
        let mut standing: Vec<String> = std::fs::read_dir(&there.chosen)
            .expect("the folder reads")
            .filter_map(|one| one.ok())
            .map(|one| one.file_name().to_string_lossy().into_owned())
            // **The reserved staging folder is not a landing place** (`Server
            // protocol and transfers` S1); what is staged inside it is read
            // below.
            .filter(|one| one != ".smoozy-staging")
            .collect();
        standing.sort();
        println!(
            "KEEP BOTH: came to {came_first} then {came_second}; standing {standing:?}; the rows read \
             «{}» and «{}».",
            there.the_row_reads(&first),
            there.the_row_reads(&second)
        );
        let mut missed = Vec::new();
        a_figure!(
            missed,
            standing == ["p13-cube (2).blend", "p13-cube (3).blend", "p13-cube.blend"],
            "KEEP BOTH: the folder holds {standing:?}"
        );
        a_figure!(
            missed,
            it_is_all(&there.chosen.join("p13-cube.blend"), THE_USERS_OWN_BYTES, 1_000),
            "KEEP BOTH: the user's own file was touched"
        );
        a_figure!(missed, what_is_left_staged(&there.chosen).is_empty(), "KEEP BOTH: something is left staged");
        a_figure!(missed, !came_second.contains("failed: [["), "KEEP BOTH: the second came to {came_second}");
        drop(there.scratch);
        the_figures_held("KEEP BOTH", missed);
    }

    // ------------- the Vast pass's two download faults (Vast-pass fix piece 1)

    /// **A download asked of the stand-in machine, through the application's
    /// real route**, answered as `answer` says — or, with none, sent without
    /// the question (`Download` M6). **A *Stop* runs nothing**, as the
    /// application's own answer runs nothing for it. Added 2026-10-01, step
    /// 10c's Vast-pass fix piece 1.
    #[allow(clippy::too_many_arguments, reason = "one download, as the application asks for it")]
    async fn a_download_answered<J: ReadsTheLocalJobRecords>(
        machine: &Arc<AMachine>,
        record: &Arc<TheTimedRecord>,
        server_row: &Identity,
        installation: &Identity,
        the_run: &Arc<TheRunsTransfers>,
        destination: &Path,
        picked: Vec<u8>,
        records: &J,
        answer: Option<smoozy_file_browser::already_exists::CollisionAnswer>,
    ) -> (String, String) {
        use smoozy_file_browser::already_exists::CollisionAnswer;
        let point = APointLending::over(machine).await;
        let mut ready = a_download_is_prepared(
            WhatIsBeingDownloaded {
                server_row: server_row.clone(),
                installation: Some(installation.clone()),
                destination: the_bytes_of(destination),
                picked: vec![picked],
                run: a_run(),
            },
            &point.sessions,
            record.as_ref() as &dyn TheRecordADownloadWrites,
            records,
        )
        .await
        .expect("the download is prepared");
        match answer {
            Some(answer) => smoozy_desktop::downloading::the_answer_is_applied(&mut ready, record.as_ref(), answer)
                .expect("the answer is written"),
            None => the_request_goes_without_a_question(&mut ready, record.as_ref())
                .expect("the dispositions are written"),
        }
        if answer == Some(CollisionAnswer::Stop) {
            return (ready.request.record.as_text(), "stopped".to_string());
        }
        let came_to = the_download_runs(&mut ready, &point.sessions, the_run, record.as_ref()).await;
        (ready.request.record.as_text(), format!("{came_to:?}"))
    }

    /// **A file at the top of the server is one file** — the Vast pass's fault
    /// 1. **On Vast the top of the server is where everything goes**: the login
    /// is `root`, Files opens at *Root*, and *Uploads go to /*.
    /// *Download* on `/pv-4gb.bin` put up *0 files are already there · 1 thing
    /// on the server could not be read.*, and the record held the file as
    /// *gone*: a picked file is found by asking its own parent for it, and the
    /// parent of `/x` was taken as an empty path, which the machine reads as
    /// *no such file*.
    ///
    /// **The top of this computer's own disk stands in for the top of the
    /// server**, because the machine here is this computer's own `sftp-server`
    /// and reads this computer's paths. Only a listing is read and nothing is
    /// fetched. Added 2026-10-01, step 10c's Vast-pass fix piece 1.
    #[tokio::test(flavor = "multi_thread", worker_threads = 4)]
    async fn a_file_at_the_top_of_the_server_is_one_file() {
        let _deadline = TheTestsDeadline::started();
        let program = the_server_or_say_so!();
        let at_the_top = std::fs::read_dir("/").ok().and_then(|entries| {
            entries
                .filter_map(Result::ok)
                .find(|entry| entry.file_type().is_ok_and(|kind| kind.is_file()))
                .map(|entry| entry.path())
        });
        let Some(at_the_top) = at_the_top else {
            println!("SKIPPED: this computer has no plain file at the top of its disk to stand in.");
            return;
        };

        let scratch = Scratch::new("top-of-the-server");
        let chosen = scratch.folder("chosen");
        let record = TheTimedRecord::at(&scratch.folder("record").join("smoozy.sqlite3"));
        let (server_row, installation) = a_server_row_installed(&record);
        let machine = Arc::new(AMachine {
            program,
            home: scratch.at.clone(),
            limit: 10,
            ..AMachine::default()
        });
        let point = APointLending::over(&machine).await;
        let ready = a_download_is_prepared(
            WhatIsBeingDownloaded {
                server_row,
                installation: Some(installation),
                destination: the_bytes_of(&chosen),
                picked: vec![the_bytes_of(&at_the_top)],
                run: a_run(),
            },
            &point.sessions,
            record.as_ref() as &dyn TheRecordADownloadWrites,
            &NoJobIsKnown,
        )
        .await;
        let mut missed = Vec::new();
        match &ready {
            Ok(ready) => {
                println!(
                    "TOP: {} came to {} entries {:?}, could not be read {:?}, asks {:?}.",
                    at_the_top.display(),
                    ready.request.entries.len(),
                    ready
                        .request
                        .entries
                        .iter()
                        .map(|entry| String::from_utf8_lossy(&entry.landing_path).into_owned())
                        .collect::<Vec<_>>(),
                    ready.request.subtrees_that_could_not_be_read,
                    ready.the_pre_flight,
                );
                a_figure!(
                    missed,
                    ready.request.entries.len() == 1
                        && ready.request.entries[0].source_path == the_bytes_of(&at_the_top),
                    "TOP: the file is not one entry"
                );
                a_figure!(
                    missed,
                    ready.request.entries.first().map(|entry| entry.landing_path.clone())
                        == Some(the_bytes_of(&chosen.join(at_the_top.file_name().expect("a name")))),
                    "TOP: the file does not land in the chosen folder"
                );
                a_figure!(
                    missed,
                    ready.request.subtrees_that_could_not_be_read.is_empty(),
                    "TOP: the file was recorded as something that could not be read"
                );
                a_figure!(
                    missed,
                    !ready.the_pre_flight.the_request_stops_and_asks(),
                    "TOP: the request asks {:?}",
                    ready.the_pre_flight
                );
            }
            Err(why) => missed.push(format!("TOP: the download did not start: {why:?}")),
        }
        drop(point);
        drop(scratch);
        the_figures_held("TOP", missed);
    }

    /// **A download that met only something it could not read starts nothing
    /// and asks nothing** — the rest of the Vast pass's fault 1. The pass's
    /// request held no file and one thing it could not read; it put up the
    /// collision question with *0 files are already there* and its four
    /// answers, and after *Stop* its row read *0 files into … · done*. **With
    /// nothing to download there is no question to ask**: it is said as a
    /// download that did not start (`Download` M4, M6; Rule 8). Added
    /// 2026-10-01, step 10c's Vast-pass fix piece 1.
    #[tokio::test(flavor = "multi_thread", worker_threads = 4)]
    async fn a_download_of_only_something_it_could_not_read_does_not_start() {
        let _deadline = TheTestsDeadline::started();
        let program = the_server_or_say_so!();

        let scratch = Scratch::new("only-unreadable");
        let chosen = scratch.folder("chosen");
        let gone = scratch.folder("machine").join("gone.bin");
        let record = TheTimedRecord::at(&scratch.folder("record").join("smoozy.sqlite3"));
        let (server_row, installation) = a_server_row_installed(&record);
        let machine = Arc::new(AMachine {
            program,
            home: scratch.at.clone(),
            limit: 10,
            ..AMachine::default()
        });
        let point = APointLending::over(&machine).await;
        let ready = a_download_is_prepared(
            WhatIsBeingDownloaded {
                server_row: server_row.clone(),
                installation: Some(installation),
                destination: the_bytes_of(&chosen),
                picked: vec![the_bytes_of(&gone)],
                run: a_run(),
            },
            &point.sessions,
            record.as_ref() as &dyn TheRecordADownloadWrites,
            &NoJobIsKnown,
        )
        .await;
        let the_run = TheRunsTransfers::for_this_run();
        let screen = the_screen_now(&record, &the_run);
        let rows: Vec<String> = screen
            .requests
            .iter()
            .map(|row| format!("{} · {}", as_read(&row.name), as_read(&row.count)))
            .collect();
        println!(
            "ONLY UNREADABLE: came to {:?}; the rows read {rows:?}.",
            ready.as_ref().map(|ready| ready.the_pre_flight)
        );
        let mut missed = Vec::new();
        a_figure!(
            missed,
            matches!(ready, Err(smoozy_desktop::downloading::WhyTheDownloadDidNotStart::NothingPickedCouldBeRead)),
            "ONLY UNREADABLE: it came to {:?}",
            ready.as_ref().map(|ready| ready.the_pre_flight)
        );
        a_figure!(missed, !rows.iter().any(|row| row.contains("done")), "ONLY UNREADABLE: a row reads done: {rows:?}");
        drop(point);
        drop(scratch);
        the_figures_held("ONLY UNREADABLE", missed);
    }

    /// **An empty folder picked alone downloads as an empty folder** —
    /// `Download` M9, *a downloaded folder arrives as a folder*, and the
    /// spec's own sentence of it. After the Vast pass's fix piece 1 it did not
    /// start and said *Smoozy could not read what you chose on the server*,
    /// which is false: the folder was read, and holds nothing. Added
    /// 2026-10-01, step 10c's Vast-pass fix piece 1b.
    #[tokio::test(flavor = "multi_thread", worker_threads = 4)]
    async fn an_empty_folder_picked_alone_downloads_as_an_empty_folder() {
        let _deadline = TheTestsDeadline::started();
        let program = the_server_or_say_so!();

        let there = a_file_already_there("empty-folder", program, "unused.bin");
        std::fs::remove_file(there.chosen.join("unused.bin")).expect("the user's file goes");
        let empty = there.scratch.folder("machine/empty");
        let point = APointLending::over(&there.machine).await;
        let ready = a_download_is_prepared(
            WhatIsBeingDownloaded {
                server_row: there.server_row.clone(),
                installation: Some(there.installation.clone()),
                destination: the_bytes_of(&there.chosen),
                picked: vec![the_bytes_of(&empty)],
                run: a_run(),
            },
            &point.sessions,
            there.record.as_ref() as &dyn TheRecordADownloadWrites,
            &NoJobIsKnown,
        )
        .await;
        let mut missed = Vec::new();
        match ready {
            Err(why) => missed.push(format!("EMPTY FOLDER: the download did not start: {why:?}")),
            Ok(mut ready) => {
                a_figure!(
                    missed,
                    !ready.the_pre_flight.the_request_stops_and_asks(),
                    "EMPTY FOLDER: the request asks {:?}",
                    ready.the_pre_flight
                );
                the_request_goes_without_a_question(&mut ready, there.record.as_ref())
                    .expect("the dispositions are written");
                let came_to =
                    the_download_runs(&mut ready, &point.sessions, &there.the_run, there.record.as_ref()).await;
                let request = ready.request.record.as_text();
                let reads = there.the_row_reads(&request);
                let landed = there.chosen.join("empty");
                let inside: Vec<String> = std::fs::read_dir(&landed)
                    .map(|entries| {
                        entries
                            .filter_map(Result::ok)
                            .map(|one| one.file_name().to_string_lossy().into_owned())
                            .collect()
                    })
                    .unwrap_or_else(|_| vec!["(no folder)".to_string()]);
                println!("EMPTY FOLDER: came to {came_to:?}; it holds {inside:?}; the row reads «{reads}».");
                a_figure!(missed, landed.is_dir(), "EMPTY FOLDER: no folder landed at {}", landed.display());
                a_figure!(missed, inside.is_empty(), "EMPTY FOLDER: the folder holds {inside:?}");
                a_figure!(
                    missed,
                    !reads.contains("could not") && !reads.contains("did not"),
                    "EMPTY FOLDER: «{reads}»"
                );
            }
        }
        drop(point);
        drop(there.scratch);
        the_figures_held("EMPTY FOLDER", missed);
    }

    /// **A download stopped at the question does not read *done*** — the Vast
    /// pass's fault 1, after its *Stop*. Nothing was downloaded, and *done*
    /// says it worked (Rule 8, `Design/Transfers § A download meeting files
    /// already there`). Added 2026-10-01, step 10c's Vast-pass fix piece 1.
    #[tokio::test(flavor = "multi_thread", worker_threads = 4)]
    async fn a_download_stopped_at_the_question_does_not_read_done() {
        use smoozy_file_browser::already_exists::CollisionAnswer;
        let _deadline = TheTestsDeadline::started();
        let program = the_server_or_say_so!();

        let there = a_file_already_there("stopped-down", program, "p13-cube.blend");
        let (request, _) = a_download_answered(
            &there.machine,
            &there.record,
            &there.server_row,
            &there.installation,
            &there.the_run,
            &there.chosen,
            the_bytes_of(&there.on_the_machine),
            &NoJobIsKnown,
            Some(CollisionAnswer::Stop),
        )
        .await;
        let reads = there.the_row_reads(&request);
        println!("STOPPED: the row reads «{reads}».");
        let mut missed = Vec::new();
        a_figure!(missed, !reads.contains("· done"), "STOPPED: the row reads done: «{reads}»");
        a_figure!(missed, reads.contains("— did not finish"), "STOPPED: «{reads}»");
        a_figure!(missed, reads.contains("0 / 1"), "STOPPED: «{reads}»");
        a_figure!(
            missed,
            it_is_all(&there.chosen.join("p13-cube.blend"), THE_USERS_OWN_BYTES, 1_000),
            "STOPPED: the user's own file was touched"
        );
        drop(there.scratch);
        the_figures_held("STOPPED", missed);
    }

    /// **A job's own folder, as a whole-job download finds it on the local
    /// record** (`Download` M9, P10) — the route *Download all* sends.
    struct AJobsOutputFolder {
        the_output_folder: Vec<u8>,
        job: Identity,
    }

    impl ReadsTheLocalJobRecords for AJobsOutputFolder {
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
        fn the_job_whose_output_folder_this_is(&self, picked: &[u8]) -> Option<AJobsOwnFolder> {
            (picked == self.the_output_folder).then(|| AJobsOwnFolder {
                job: self.job.clone(),
                label: Some("pv long2".to_string()),
            })
        }
    }

    /// **Keep both on a job's second *Download all* lands the job's second
    /// folder** — the Vast pass's fault 2. All 240 frames of *pv long2* ended
    /// *Download failed.*: the record named landing paths under `… pv long2
    /// (2)/frames/`, and no `(2)` folder was ever made. **The landing folders
    /// were made before the question, from the names before it**, and *Keep
    /// both* renamed the job's folder after them (`Download` M8, M9). *Replace*
    /// lands in the folders already there, which is why it worked. Added
    /// 2026-10-01, step 10c's Vast-pass fix piece 1.
    #[tokio::test(flavor = "multi_thread", worker_threads = 4)]
    async fn keep_both_on_a_whole_jobs_download_lands_the_jobs_second_folder() {
        use smoozy_file_browser::already_exists::CollisionAnswer;
        let _deadline = TheTestsDeadline::started();
        let program = the_server_or_say_so!();

        let there = a_file_already_there("keep-both-job", program, "unused.bin");
        std::fs::remove_file(there.chosen.join("unused.bin")).expect("the user's file goes");
        let output = there.scratch.folder("machine/jobs/abc");
        let frames = there.scratch.folder("machine/jobs/abc/frames");
        for frame in ["0001.png", "0002.png"] {
            std::fs::write(frames.join(frame), vec![THE_MACHINES_BYTES; 4_096]).expect("a frame");
        }
        let records = AJobsOutputFolder {
            the_output_folder: the_bytes_of(&output),
            job: Identity::issue(IdentityKind::Job).expect("random bytes"),
        };
        let asked = |answer| {
            a_download_answered(
                &there.machine,
                &there.record,
                &there.server_row,
                &there.installation,
                &there.the_run,
                &there.chosen,
                the_bytes_of(&output),
                &records,
                answer,
            )
        };
        let (_, came_first) = asked(None).await;
        let the_jobs_folder: Vec<String> = std::fs::read_dir(&there.chosen)
            .expect("the folder reads")
            .filter_map(Result::ok)
            .map(|one| one.file_name().to_string_lossy().into_owned())
            .filter(|one| one != ".smoozy-staging")
            .collect();
        let (second, came_second) = asked(Some(CollisionAnswer::KeepBoth)).await;
        let mut standing: Vec<String> = std::fs::read_dir(&there.chosen)
            .expect("the folder reads")
            .filter_map(Result::ok)
            .map(|one| one.file_name().to_string_lossy().into_owned())
            .filter(|one| one != ".smoozy-staging")
            .collect();
        standing.sort();
        let reads = there.the_row_reads(&second);
        println!(
            "KEEP BOTH, A JOB: came to {came_first} then {came_second}; standing {standing:?}; the \
             second row reads «{reads}»."
        );
        let mut missed = Vec::new();
        a_figure!(missed, the_jobs_folder.len() == 1, "KEEP BOTH, A JOB: the first came to {the_jobs_folder:?}");
        let first = the_jobs_folder.first().cloned().unwrap_or_default();
        let kept = format!("{first} (2)");
        a_figure!(
            missed,
            standing == [first.clone(), kept.clone()],
            "KEEP BOTH, A JOB: the folder holds {standing:?}"
        );
        for frame in ["0001.png", "0002.png"] {
            a_figure!(
                missed,
                it_is_all(&there.chosen.join(&kept).join("frames").join(frame), THE_MACHINES_BYTES, 4_096),
                "KEEP BOTH, A JOB: {kept}/frames/{frame} is not the machine's"
            );
        }
        a_figure!(missed, !came_second.contains("failed: [["), "KEEP BOTH, A JOB: the second came to {came_second}");
        a_figure!(missed, reads.contains("· done"), "KEEP BOTH, A JOB: «{reads}»");
        drop(there.scratch);
        the_figures_held("KEEP BOTH, A JOB", missed);
    }

    /// **Skip existing on an uploaded file whose part a *Disconnect* left
    /// abandons the part in the record first** — `Server protocol and
    /// transfers` S28a, `File Browser` U8's third bullet — **and the next
    /// connection's sweep leaves it**, since on the machine no stable identity
    /// can be matched (`Server protocol and transfers` §12 item 15). Step 10c's
    /// transfer wave piece 7, the map's third desk check, upload half.
    #[tokio::test(flavor = "multi_thread", worker_threads = 4)]
    async fn skip_existing_abandons_an_uploads_leftover_and_the_sweep_leaves_it() {
        use smoozy_desktop::servers::the_machines_work::the_staging_left_on_this_machine_is_swept;
        use smoozy_file_browser::already_exists::CollisionAnswer;
        use smoozy_server_protocol::transfers::{Ending, StoppedReason};
        let _deadline = TheTestsDeadline::started();
        let program = the_server_or_say_so!();

        let size = 24_000_000u64;
        let scratch = Scratch::new("skip-up");
        let here = scratch.folder("here/three");
        three_parts_one_large(&here, size);
        let machine_side = scratch.folder("machine");
        let record = TheTimedRecord::at(&scratch.folder("record").join("smoozy.sqlite3"));
        let (server_row, installation) = a_server_row_installed(&record);
        let the_server = server_row.as_text();
        let machine = Arc::new(AMachine {
            program,
            home: scratch.at.clone(),
            limit: 10,
            ..AMachine::default()
        });
        let point = APointLending::over(&machine).await;
        let the_run = Arc::new(TheRunsTransfers::for_this_run());
        let cap = TheCap::Of(BytesPerSecond(4_000_000));
        the_run.the_caps_saved_are(cap, cap);
        let as_a_door: Arc<dyn TheRecordAnUploadWrites> = record.clone();
        let asked = |destination: &Path, chosen: PathBuf| WhatIsBeingUploaded {
            server_row: server_row.clone(),
            installation: Some(installation.clone()),
            destination: the_bytes_of(destination),
            chosen: vec![chosen],
            run: a_run(),
        };
        let mut ready = an_upload_is_prepared(&asked(&machine_side, here.clone()), &point.sessions, as_a_door.as_ref())
            .await
            .expect("the upload is prepared");
        assert_eq!(
            the_answer_is_applied(&mut ready, None, &point.sessions, as_a_door.as_ref()).await,
            WhatTheAnswerLeft::ItGoes
        );
        let request = ready.request.record.as_text();
        let (_, ()) = tokio::join!(
            the_upload_runs(ready, &point.sessions, &the_run, Arc::clone(&as_a_door), &NobodyIsTold),
            cut_by_a_disconnect(&record, &machine, &point, &the_run, &the_server, &request)
        );
        let landing_folder = machine_side.join("three");
        assert_eq!(what_is_left_staged(&landing_folder).len(), 1, "one part was left");

        // **The file already stands at its name on the machine**, and the user
        // asks for it again, answering *Skip existing*.
        std::fs::copy(here.join("part_1.blend"), landing_folder.join("part_1.blend")).expect("put there");
        let point = APointLending::over(&machine).await;
        let mut ready = an_upload_is_prepared(
            &asked(&landing_folder, here.join("part_1.blend")),
            &point.sessions,
            as_a_door.as_ref(),
        )
        .await
        .expect("the second upload is prepared");
        assert!(ready.something_collides());
        assert_eq!(
            the_answer_is_applied(
                &mut ready,
                Some(CollisionAnswer::SkipExisting),
                &point.sessions,
                as_a_door.as_ref()
            )
            .await,
            WhatTheAnswerLeft::ItGoes
        );
        let came_to = the_upload_runs(ready, &point.sessions, &the_run, Arc::clone(&as_a_door), &NobodyIsTold).await;
        let endings = the_endings(&record);
        let door = |act: &mut dyn FnMut(&rusqlite::Connection)| record.held(act);
        let swept = the_staging_left_on_this_machine_is_swept(&point.sessions, &door, &server_row).await;
        println!(
            "SKIP UP: came to {came_to:?}; the endings {endings:?}; the sweep {swept:?}; staged {:?}.",
            what_is_left_staged(&landing_folder)
        );
        let mut missed = Vec::new();
        a_figure!(missed, came_to.skipped == 1, "SKIP UP: came to {came_to:?}");
        a_figure!(
            missed,
            endings.contains(&Some(Ending::Stopped(StoppedReason::AbandonedByItsRequest)))
                && !endings.contains(&Some(Ending::Interrupted)),
            "SKIP UP: the part's record reads {endings:?}, not stopped, abandoned by its request"
        );
        a_figure!(missed, swept.left == 1, "SKIP UP: the sweep came to {swept:?}");
        the_figures_held("SKIP EXISTING, UP", missed);
    }

    /// **An upload's leftover, removed: the record says so first, and the part
    /// stays on the server with its cleanup owed** — step 10c's transfer wave
    /// piece 7, the map's first and third desk checks, upload half, as the
    /// plans let them be met.
    ///
    /// **Why the part stays.** `File Browser` U8 removes *"only the exact
    /// staging object whose synthesized name and stable file identity match
    /// that record (or records it as an owed cleanup)"*, and `Server protocol
    /// and transfers` S28's sweep the same; on the machine the supplier cannot
    /// answer a stable identity (`Server protocol and transfers` §12 item 15,
    /// permanently unresolved). So *Remove leaves nothing staged* and *the next
    /// connection's sweep removes it* cannot be met for an upload, and what is
    /// met instead is held here: **the record goes *stopped* first, the debt is
    /// written, the row goes on saying the part-sent files are on the server,
    /// the sweep leaves the part — and once it is gone from the machine, the
    /// sweep clears the debt and the row stops saying so.**
    #[tokio::test(flavor = "multi_thread", worker_threads = 4)]
    async fn an_uploads_leftover_removed_owes_its_cleanup_and_the_sweep_clears_it_once_gone() {
        use smoozy_desktop::servers::the_machines_work::the_staging_left_on_this_machine_is_swept;
        use smoozy_server_protocol::transfers::{Ending, StoppedReason};
        let _deadline = TheTestsDeadline::started();
        let program = the_server_or_say_so!();

        let size = 24_000_000u64;

        let scratch = Scratch::new("remove-up");
        let here = scratch.folder("here/three");
        three_parts_one_large(&here, size);
        let machine_side = scratch.folder("machine");
        let record = TheTimedRecord::at(&scratch.folder("record").join("smoozy.sqlite3"));
        let (server_row, installation) = a_server_row_installed(&record);
        let the_server = server_row.as_text();
        let machine = Arc::new(AMachine {
            program,
            home: scratch.at.clone(),
            limit: 10,
            ..AMachine::default()
        });
        let point = APointLending::over(&machine).await;
        let the_run = Arc::new(TheRunsTransfers::for_this_run());
        let cap = TheCap::Of(BytesPerSecond(4_000_000));
        the_run.the_caps_saved_are(cap, cap);
        let as_a_door: Arc<dyn TheRecordAnUploadWrites> = record.clone();
        let asked = WhatIsBeingUploaded {
            server_row: server_row.clone(),
            installation: Some(installation.clone()),
            destination: the_bytes_of(&machine_side),
            chosen: vec![here.clone()],
            run: a_run(),
        };
        let mut ready = an_upload_is_prepared(&asked, &point.sessions, as_a_door.as_ref())
            .await
            .expect("the upload is prepared");
        assert_eq!(
            the_answer_is_applied(&mut ready, None, &point.sessions, as_a_door.as_ref()).await,
            WhatTheAnswerLeft::ItGoes
        );
        let request = ready.request.record.as_text();
        let (_, ()) = tokio::join!(
            the_upload_runs(ready, &point.sessions, &the_run, Arc::clone(&as_a_door), &NobodyIsTold),
            cut_by_a_disconnect(&record, &machine, &point, &the_run, &the_server, &request)
        );
        let landing_folder = machine_side.join("three");
        assert_eq!(what_is_left_staged(&landing_folder).len(), 1, "one part was left");

        // ---- Remove
        let removed = smoozy_desktop::downloading::a_leftover_is_removed(
            record.as_ref(),
            &the_run,
            &Identity::read_back(IdentityKind::Operation, &request).expect("the request"),
            &server_row,
            Direction::Up,
        );
        let the_part = {
            let held = record.record.lock().expect("the record");
            smoozy_desktop::record::transfer_records::all(held.connection())
                .expect("the records read")
                .into_iter()
                .find(|one| one.ending != Some(Ending::Published))
                .expect("the cut file's record")
        };
        let mut missed = Vec::new();
        println!(
            "REMOVE UP: {removed:?}; the part's record {:?}, owed {}; staged {:?}.",
            the_part.ending,
            the_part.cleanup_is_owed,
            what_is_left_staged(&landing_folder)
        );
        a_figure!(missed, removed.owed == 1 && removed.removed == 0, "REMOVE UP: came to {removed:?}");
        a_figure!(
            missed,
            the_part.ending == Some(Ending::Stopped(StoppedReason::StagingRemovedByUser))
                && the_part.cleanup_is_owed,
            "REMOVE UP: the record reads {:?}, owed {}, not stopped with its cleanup owed",
            the_part.ending,
            the_part.cleanup_is_owed
        );
        let screen = the_screen_now(&record, &the_run);
        let row = the_row(&screen, &request).expect("the request is on the screen");
        println!(
            "REMOVE UP: the row reads «{}», control {:?}.",
            as_read(&row.where_it_goes),
            row.control.as_ref().map(|control| control.act.as_str())
        );
        // **The 38's piece 9, item 33: a part whose cleanup is owed keeps its
        // *Remove***, which now reaches the machine (`File Browser` U8 as
        // amended 2026-10-03). Until then the row offered nothing.
        a_figure!(
            missed,
            as_read(&row.where_it_goes).contains("partial files still use space on the server")
                && row.control.as_ref().map(|control| control.act.as_str()) == Some("remove"),
            "REMOVE UP: the row does not say the part is still on the server, or offers no Remove"
        );

        // ---- Connect again: the sweep leaves what it cannot claim
        let point = APointLending::over(&machine).await;
        let door = |act: &mut dyn FnMut(&rusqlite::Connection)| record.held(act);
        let swept = the_staging_left_on_this_machine_is_swept(&point.sessions, &door, &server_row).await;
        println!("SWEEP UP: {swept:?}; staged {:?}.", what_is_left_staged(&landing_folder));
        a_figure!(missed, swept.left == 1 && swept.debts_cleared == 0, "SWEEP UP: came to {swept:?}");
        a_figure!(
            missed,
            what_is_left_staged(&landing_folder).len() == 1,
            "SWEEP UP: the sweep removed a part it cannot claim"
        );

        // ---- The part goes by the user's own hand; the next sweep clears the debt
        for part in what_is_left_staged(&landing_folder) {
            std::fs::remove_file(landing_folder.join(".smoozy-staging").join(part)).expect("removed by hand");
        }
        let swept = the_staging_left_on_this_machine_is_swept(&point.sessions, &door, &server_row).await;
        the_run.every_leftover_is_read_again();
        let screen = the_screen_now(&record, &the_run);
        let row = the_row(&screen, &request).expect("the request is on the screen");
        println!("SWEEP UP, ONCE GONE: {swept:?}; the row reads «{}».", as_read(&row.where_it_goes));
        a_figure!(missed, swept.gone == 1 && swept.debts_cleared == 1, "SWEEP UP, ONCE GONE: came to {swept:?}");
        a_figure!(
            missed,
            !as_read(&row.where_it_goes).contains("partial files"),
            "SWEEP UP, ONCE GONE: the row still says partial files are on the server"
        );
        the_figures_held("REMOVE AND SWEEP, UP", missed);
    }

    /// **An upload's leftover, removed with the server connected: the part on
    /// the machine goes, the record is stopped first and its debt cleared, and
    /// a staging path outside Smoozy's own staging folder is refused** —
    /// `File Browser` U8 as amended 2026-10-03 (the 38's piece 9, item 33,
    /// hole 12). Until then *Remove* recorded the removal and left the part on
    /// the machine (*"REMOVE UP: TheLeftoverRemoved { removed: 0, owed: 1 …
    /// staged [\"…-1.smoozy-part\"]"*, the desk on 2026-10-04 before the fix).
    #[tokio::test(flavor = "multi_thread", worker_threads = 4)]
    async fn an_uploads_leftover_removed_with_the_server_connected_leaves_nothing_on_the_machine() {
        use smoozy_server_protocol::transfers::{Ending, StoppedReason, TransferRecord};
        let _deadline = TheTestsDeadline::started();
        let program = the_server_or_say_so!();

        let size = 24_000_000u64;

        let scratch = Scratch::new("remove-up-connected");
        let here = scratch.folder("here/three");
        three_parts_one_large(&here, size);
        let machine_side = scratch.folder("machine");
        let record = TheTimedRecord::at(&scratch.folder("record").join("smoozy.sqlite3"));
        let (server_row, installation) = a_server_row_installed(&record);
        let the_server = server_row.as_text();
        let machine = Arc::new(AMachine {
            program,
            home: scratch.at.clone(),
            limit: 10,
            ..AMachine::default()
        });
        let point = APointLending::over(&machine).await;
        let the_run = Arc::new(TheRunsTransfers::for_this_run());
        let cap = TheCap::Of(BytesPerSecond(4_000_000));
        the_run.the_caps_saved_are(cap, cap);
        let as_a_door: Arc<dyn TheRecordAnUploadWrites> = record.clone();
        let asked = WhatIsBeingUploaded {
            server_row: server_row.clone(),
            installation: Some(installation.clone()),
            destination: the_bytes_of(&machine_side),
            chosen: vec![here.clone()],
            run: a_run(),
        };
        let mut ready = an_upload_is_prepared(&asked, &point.sessions, as_a_door.as_ref())
            .await
            .expect("the upload is prepared");
        assert_eq!(
            the_answer_is_applied(&mut ready, None, &point.sessions, as_a_door.as_ref()).await,
            WhatTheAnswerLeft::ItGoes
        );
        let request = ready.request.record.as_text();
        let (_, ()) = tokio::join!(
            the_upload_runs(ready, &point.sessions, &the_run, Arc::clone(&as_a_door), &NobodyIsTold),
            cut_by_a_disconnect(&record, &machine, &point, &the_run, &the_server, &request)
        );
        let landing_folder = machine_side.join("three");
        assert_eq!(what_is_left_staged(&landing_folder).len(), 1, "one part was left");

        // ---- Connect again, and Remove
        let point = APointLending::over(&machine).await;
        let the_records = |record: &TheTimedRecord| -> Vec<TransferRecord> {
            let held = record.record.lock().expect("the record");
            smoozy_desktop::record::transfer_records::all(held.connection())
                .expect("the records read")
                .into_iter()
                .filter(|one| one.ending == Some(Ending::Interrupted))
                .collect()
        };
        let the_leftover = the_records(&record);
        assert_eq!(the_leftover.len(), 1, "the cut file's record reads interrupted");
        let removed = smoozy_desktop::downloading::an_upload_leftover_is_removed_on_the_machine(
            record.as_ref(),
            &the_run,
            &point.sessions,
            the_leftover,
        )
        .await;
        let the_part = {
            let held = record.record.lock().expect("the record");
            smoozy_desktop::record::transfer_records::all(held.connection())
                .expect("the records read")
                .into_iter()
                .find(|one| one.ending != Some(Ending::Published))
                .expect("the cut file's record")
        };
        let mut missed = Vec::new();
        println!(
            "REMOVE UP, CONNECTED: {removed:?}; the part's record {:?}, owed {}; staged {:?}.",
            the_part.ending,
            the_part.cleanup_is_owed,
            what_is_left_staged(&landing_folder)
        );
        a_figure!(missed, removed.removed == 1 && removed.owed == 0 && removed.refused == 0, "REMOVE UP, CONNECTED: came to {removed:?}");
        a_figure!(
            missed,
            what_is_left_staged(&landing_folder).is_empty(),
            "REMOVE UP, CONNECTED: the part is still on the machine"
        );
        a_figure!(
            missed,
            the_part.ending == Some(Ending::Stopped(StoppedReason::StagingRemovedByUser)) && !the_part.cleanup_is_owed,
            "REMOVE UP, CONNECTED: the record reads {:?}, owed {}, not stopped with its debt cleared",
            the_part.ending,
            the_part.cleanup_is_owed
        );
        let screen = the_screen_now(&record, &the_run);
        let row = the_row(&screen, &request).expect("the request is on the screen");
        println!("REMOVE UP, CONNECTED: the row reads «{}».", as_read(&row.where_it_goes));
        a_figure!(
            missed,
            !as_read(&row.where_it_goes).contains("partial files") && row.control.is_none(),
            "REMOVE UP, CONNECTED: the row still says partial files are on the server, or offers a control"
        );

        // ---- A record naming a path outside the staging folder is refused
        let elsewhere = machine_side.join("not-staging");
        std::fs::create_dir_all(&elsewhere).expect("a folder of the user's own");
        let mut outside = TransferRecord::arranged(
            Identity::issue(IdentityKind::Transfer).expect("random"),
            Direction::Up,
            server_row.clone(),
            Some(installation.clone()),
            "/here/x.blend".to_string(),
            elsewhere.join("x.blend").to_string_lossy().into_owned(),
            &elsewhere.to_string_lossy(),
            10,
            a_run(),
        );
        outside.ending = Some(Ending::Interrupted);
        let the_users_file = std::path::PathBuf::from(outside.staging_path.as_text());
        std::fs::write(&the_users_file, b"the user's").expect("a file of the user's own");
        let refused = smoozy_desktop::downloading::an_upload_leftover_is_removed_on_the_machine(
            record.as_ref(),
            &the_run,
            &point.sessions,
            vec![outside],
        )
        .await;
        println!("REMOVE UP, OUTSIDE: {refused:?}.");
        a_figure!(missed, refused.refused == 1 && refused.removed == 0, "REMOVE UP, OUTSIDE: came to {refused:?}");
        a_figure!(missed, the_users_file.exists(), "REMOVE UP, OUTSIDE: a file outside the staging folder was removed");
        the_figures_held("REMOVE WITH THE SERVER CONNECTED, UP", missed);
    }

    /// **Stop on a request: what landed stays, what waits is never sent, and
    /// the file in flight is stopped and its part removed** — `Server protocol
    /// and transfers` C22, `File Browser` U6 and `Download` M12, up and then
    /// down. Step 10c's transfer wave piece 7, item 2.
    ///
    /// **The fault as a person met it**: *Stop* answered *"Smoozy cannot do
    /// this yet, because nothing in Smoozy moves a transfer yet"* on a
    /// transfer that was moving, and the files went on going.
    #[tokio::test(flavor = "multi_thread", worker_threads = 4)]
    async fn stop_on_a_request_keeps_what_landed_and_stops_what_is_in_flight_up_and_down() {
        let _deadline = TheTestsDeadline::started();
        stop_is_pressed("stop", false).await;
    }

    /// **Stop on one file: that file is stopped and its part removed, and the
    /// rest of the request goes on** — `Download` M12's single-transfer stop,
    /// `Design/Transfers § Stopping one file`, up and then down.
    #[tokio::test(flavor = "multi_thread", worker_threads = 4)]
    async fn stop_on_one_file_stops_that_file_and_the_rest_go_up_and_down() {
        let _deadline = TheTestsDeadline::started();
        stop_is_pressed("stop-one", true).await;
    }

    async fn stop_is_pressed(named: &str, one_file: bool) {
        use smoozy_server_protocol::transfers::{Ending, StoppedReason};
        let program = the_server_or_say_so!();

        let size = 24_000_000u64;
        let scratch = Scratch::new(named);
        let here = scratch.folder("here/three");
        three_parts_one_large(&here, size);
        let machine_side = scratch.folder("machine");
        let chosen = scratch.folder("chosen");
        let record = TheTimedRecord::at(&scratch.folder("record").join("smoozy.sqlite3"));
        let (server_row, installation) = a_server_row_installed(&record);
        let machine = Arc::new(AMachine {
            program,
            home: scratch.at.clone(),
            limit: 10,
            ..AMachine::default()
        });
        let point = APointLending::over(&machine).await;
        let the_run = Arc::new(TheRunsTransfers::for_this_run());
        let cap = TheCap::Of(BytesPerSecond(4_000_000));
        the_run.the_caps_saved_are(cap, cap);
        let as_a_door: Arc<dyn TheRecordAnUploadWrites> = record.clone();
        let said = if one_file { "STOP ONE FILE" } else { "STOP" };

        // **Press Stop once the large file moves alone**, on the request or on
        // that one file by the landing path its row names. **Step 10c's
        // eleventh-pass fix piece 4**: a request's files move several at once,
        // so the two small parts land first ([`three_parts_one_large`]). That
        // a whole request's Stop never arranges a file still waiting is held
        // by `stop_on_a_request_never_arranges_the_files_still_waiting`.
        let press = |request: String| {
            let the_run = Arc::clone(&the_run);
            let record = Arc::clone(&record);
            async move {
                let mut pressed_on = None;
                // **The two small files of this half are across** once the
                // page has been told two more units ended than before it began.
                let told_before = record.moved.lock().expect("the moments").len();
                until(
                    "the large file moves alone",
                    Duration::from_secs(60),
                    || {
                        let under_way = the_run.what_is_under_way().get(&request).cloned();
                        let alone = under_way
                            .as_ref()
                            .is_some_and(|under_way| under_way.moving.len() == 1);
                        let moving =
                            under_way.and_then(|under_way| under_way.moving.first().cloned());
                        let second = record.moved.lock().expect("the moments").len()
                            >= told_before + 2
                            && alone
                            && moving
                                .as_ref()
                                .is_some_and(|file| file.report.bytes_staged > 1_000_000);
                        if second {
                            pressed_on = moving.map(|file| file.path);
                        }
                        second
                    },
                )
                .await;
                let file = pressed_on.expect("a file in flight");
                assert!(the_run.the_user_stops(&request, one_file.then_some(file.as_slice())));
                file
            }
        };
        let mut missed = Vec::new();

        // ---- up
        let asked = WhatIsBeingUploaded {
            server_row: server_row.clone(),
            installation: Some(installation.clone()),
            destination: the_bytes_of(&machine_side),
            chosen: vec![here.clone()],
            run: a_run(),
        };
        let mut ready = an_upload_is_prepared(&asked, &point.sessions, as_a_door.as_ref())
            .await
            .expect("the upload is prepared");
        assert_eq!(
            the_answer_is_applied(&mut ready, None, &point.sessions, as_a_door.as_ref()).await,
            WhatTheAnswerLeft::ItGoes
        );
        let request = ready.request.record.as_text();
        let (came_to, stopped) = tokio::join!(
            the_upload_runs(ready, &point.sessions, &the_run, Arc::clone(&as_a_door), &NobodyIsTold),
            press(request.clone())
        );
        let endings = the_endings(&record);
        let staged = what_is_left_staged(&machine_side.join("three"));
        println!(
            "{said} UP: came to {came_to:?}; stopped on {}; the endings {endings:?}; staged {staged:?}.",
            String::from_utf8_lossy(&stopped)
        );
        let (uploaded, not_finished) = (2, 1);
        a_figure!(
            missed,
            came_to.uploaded == uploaded
                && came_to.did_not_finish == not_finished
                && came_to.failed.is_empty(),
            "{said} UP: came to {came_to:?}"
        );
        let the_stop = Some(Ending::Stopped(StoppedReason::StagingRemovedByUser));
        a_figure!(
            missed,
            endings.iter().filter(|ending| **ending == the_stop).count() == 1
                && endings.iter().filter(|ending| **ending == Some(Ending::Published)).count() == uploaded,
            "{said} UP: the endings are {endings:?}"
        );
        a_figure!(missed, staged.is_empty(), "{said} UP: the stopped file's part is still staged: {staged:?}");
        a_figure!(
            missed,
            !machine_side.join("three").join(PathBuf::from(String::from_utf8_lossy(&stopped).into_owned()).file_name().expect("a name")).exists(),
            "{said} UP: the stopped file reached its name"
        );
        let screen = the_screen_now(&record, &the_run);
        let row = the_row(&screen, &request).expect("the request is on the screen");
        a_figure!(
            missed,
            row.control.as_ref().map(|control| control.act.as_str()) != Some("stop")
                && row.control.as_ref().map(|control| control.act.as_str()) != Some("remove")
                && as_read(&row.name).contains("did not finish"),
            "{said} UP: the row reads «{}» with {:?}",
            as_read(&row.name),
            row.control.as_ref().map(|control| control.act.as_str())
        );

        // ---- down, of what landed and of the rest put there for it
        for at in 0..3 {
            let landed = machine_side.join(format!("three/part_{at}.blend"));
            if !landed.exists() {
                a_large_file_at(&landed, size);
            }
        }
        let before = the_endings(&record).len();
        let asked = WhatIsBeingDownloaded {
            server_row,
            installation: Some(installation),
            destination: the_bytes_of(&chosen),
            picked: vec![the_bytes_of(&machine_side.join("three"))],
            run: a_run(),
        };
        let mut ready = a_download_is_prepared(
            asked,
            &point.sessions,
            record.as_ref() as &dyn TheRecordADownloadWrites,
            &NoJobIsKnown,
        )
        .await
        .expect("the download is prepared");
        the_request_goes_without_a_question(&mut ready, record.as_ref())
            .expect("the dispositions are written");
        let request = ready.request.record.as_text();
        let (came_to, _) = tokio::join!(
            the_download_runs(&mut ready, &point.sessions, &the_run, record.as_ref()),
            press(request.clone())
        );
        let endings: Vec<_> = the_endings(&record).into_iter().skip(before).collect();
        let staged = what_is_left_staged(&chosen);
        println!("{said} DOWN: came to {came_to:?}; the endings {endings:?}; staged {staged:?}.");
        let fetched = 2;
        a_figure!(
            missed,
            came_to.fetched == fetched
                && came_to.did_not_finish == 3 - fetched
                && came_to.failed.is_empty(),
            "{said} DOWN: came to {came_to:?}"
        );
        a_figure!(
            missed,
            endings.iter().filter(|ending| **ending == the_stop).count() == 1,
            "{said} DOWN: the endings are {endings:?}"
        );
        a_figure!(missed, staged.is_empty(), "{said} DOWN: the stopped file's part is still staged: {staged:?}");
        the_figures_held(said, missed);
    }

    /// **Stop on a request that waits for its connection** — `File Browser`
    /// U6: *a waiting upload is stoppable exactly as a moving one*, and the
    /// staged part's removal *is otherwise the owed cleanup of* `Server protocol and
    /// transfers` S27. The line is
    /// cut as a repair (the period stands), the request waits, and Stop ends
    /// it: up, the record *stopped* with its cleanup owed, no channel being up;
    /// down, the part on this computer removed at once. Step 10c's transfer
    /// wave piece 7.
    #[tokio::test(flavor = "multi_thread", worker_threads = 4)]
    async fn stop_on_a_request_waiting_for_its_connection_ends_it_up_and_down() {
        use smoozy_server_protocol::transfers::{Ending, StoppedReason};
        let _deadline = TheTestsDeadline::started();
        let program = the_server_or_say_so!();

        let size = 24_000_000u64;
        let scratch = Scratch::new("stop-waiting");
        let here = scratch.folder("here");
        let source = here.join("waiting.blend");
        a_large_file_at(&source, size);
        let machine_side = scratch.folder("machine");
        let chosen = scratch.folder("chosen");
        let record = TheTimedRecord::at(&scratch.folder("record").join("smoozy.sqlite3"));
        let (server_row, installation) = a_server_row_installed(&record);
        let machine = Arc::new(AMachine {
            program,
            home: scratch.at.clone(),
            limit: 10,
            ..AMachine::default()
        });
        let point = APointLending::over(&machine).await;
        let sessions = point.sessions.clone();
        let the_run = Arc::new(TheRunsTransfers::for_this_run());
        let cap = TheCap::Of(BytesPerSecond(4_000_000));
        the_run.the_caps_saved_are(cap, cap);
        let as_a_door: Arc<dyn TheRecordAnUploadWrites> = record.clone();
        let mut missed = Vec::new();
        let the_stop = Some(Ending::Stopped(StoppedReason::StagingRemovedByUser));

        // **Cut as a repair once the file is under way, wait for the request to
        // wait, and press Stop.**
        let cut_then_stop = |request: String, point: &APointLending| {
            let (the_run, machine) = (Arc::clone(&the_run), Arc::clone(&machine));
            let lending = point.lending.abort_handle();
            async move {
                until("the file passes a checkpoint", Duration::from_secs(60), || {
                    the_run.what_is_under_way().get(&request).is_some_and(|under_way| {
                        under_way.moving.iter().any(|file| {
                            file.report.bytes_staged > smoozy_server_protocol::transfers::moving::THE_CHECKPOINT_INTERVAL
                        })
                    })
                })
                .await;
                lending.abort();
                machine.every_session_ends();
                until("the request waits for its connection", Duration::from_secs(10), || {
                    the_run
                        .what_is_under_way()
                        .get(&request)
                        .is_some_and(|under_way| under_way.waits_for_the_connection)
                })
                .await;
                assert!(the_run.the_user_stops(&request, None));
            }
        };

        // ---- up
        let asked = WhatIsBeingUploaded {
            server_row: server_row.clone(),
            installation: Some(installation.clone()),
            destination: the_bytes_of(&machine_side),
            chosen: vec![source.clone()],
            run: a_run(),
        };
        let mut ready = an_upload_is_prepared(&asked, &sessions, as_a_door.as_ref())
            .await
            .expect("the upload is prepared");
        assert_eq!(
            the_answer_is_applied(&mut ready, None, &sessions, as_a_door.as_ref()).await,
            WhatTheAnswerLeft::ItGoes
        );
        let request = ready.request.record.as_text();
        let (came_to, ()) = tokio::join!(
            tokio::time::timeout(
                Duration::from_secs(60),
                the_upload_runs(ready, &sessions, &the_run, Arc::clone(&as_a_door), &NobodyIsTold)
            ),
            cut_then_stop(request.clone(), &point)
        );
        let came_to = came_to.expect("the upload ended once stopped");
        let part = {
            let held = record.record.lock().expect("the record");
            smoozy_desktop::record::transfer_records::all(held.connection())
                .expect("the records read")
                .into_iter()
                .last()
                .expect("the file's record")
        };
        println!(
            "STOP WAITING UP: came to {came_to:?}; the record {:?}, owed {}.",
            part.ending, part.cleanup_is_owed
        );
        a_figure!(missed, came_to.uploaded == 0 && came_to.did_not_finish == 1, "STOP WAITING UP: came to {came_to:?}");
        a_figure!(
            missed,
            part.ending == the_stop && part.cleanup_is_owed,
            "STOP WAITING UP: the record reads {:?}, owed {} — not stopped with its removal owed",
            part.ending,
            part.cleanup_is_owed
        );

        // ---- down
        let mut point = point;
        point.the_line_comes_back(&machine);
        until("the line is back", Duration::from_secs(10), || sessions.a_connection_is_live()).await;
        let on_the_machine = machine_side.join("waiting.blend");
        a_large_file_at(&on_the_machine, size);
        let asked = WhatIsBeingDownloaded {
            server_row,
            installation: Some(installation),
            destination: the_bytes_of(&chosen),
            picked: vec![the_bytes_of(&on_the_machine)],
            run: a_run(),
        };
        let mut ready = a_download_is_prepared(
            asked,
            &sessions,
            record.as_ref() as &dyn TheRecordADownloadWrites,
            &NoJobIsKnown,
        )
        .await
        .expect("the download is prepared");
        the_request_goes_without_a_question(&mut ready, record.as_ref())
            .expect("the dispositions are written");
        let request = ready.request.record.as_text();
        let (came_to, ()) = tokio::join!(
            tokio::time::timeout(
                Duration::from_secs(60),
                the_download_runs(&mut ready, &sessions, &the_run, record.as_ref())
            ),
            cut_then_stop(request.clone(), &point)
        );
        let came_to = came_to.expect("the download ended once stopped");
        let part = {
            let held = record.record.lock().expect("the record");
            smoozy_desktop::record::transfer_records::all(held.connection())
                .expect("the records read")
                .into_iter()
                .last()
                .expect("the file's record")
        };
        let staged = what_is_left_staged(&chosen);
        println!(
            "STOP WAITING DOWN: came to {came_to:?}; the record {:?}, owed {}; staged {staged:?}.",
            part.ending, part.cleanup_is_owed
        );
        a_figure!(missed, came_to.fetched == 0 && came_to.did_not_finish == 1, "STOP WAITING DOWN: came to {came_to:?}");
        a_figure!(
            missed,
            part.ending == the_stop && !part.cleanup_is_owed && staged.is_empty(),
            "STOP WAITING DOWN: the record reads {:?}, owed {}, staged {staged:?}",
            part.ending,
            part.cleanup_is_owed
        );
        the_figures_held("STOP WHILE WAITING", missed);
    }

    // ------------------- a line that goes silent (twelfth-pass fix piece 1)

    impl AMachine {
        /// **The line goes silent**: every session the machine has open stops
        /// reading what is sent to it and stops answering, **and nothing is
        /// closed**. Added 2026-10-01, step 10c's twelfth-pass fix piece 1.
        ///
        /// **Why this and not [`AMachine::every_session_ends`].** That cut
        /// ends the machine's end of every session, so every wait on this side
        /// sees its socket close at once — an announced ending. **The twelfth
        /// pass's cut was the provider's firewall dropping every packet**:
        /// nothing was refused and nothing closed, the machine stopped
        /// acknowledging, and an upload's write waited for room that never
        /// came. Here each session's `sftp-server` is stopped where it stands,
        /// with its socket open, so what it has not read stays unread and what
        /// is sent to it next waits for room — the same wait.
        ///
        /// **Only this binary's own live children are stopped**: the list
        /// keeps every session this machine ever started, and a number an
        /// ended one had may since belong to something else.
        fn every_session_goes_silent(&self) {
            let ours = std::process::id().to_string();
            let live: Vec<u32> = std::process::Command::new("pgrep")
                .args(["-P", &ours])
                .output()
                .map(|out| {
                    String::from_utf8_lossy(&out.stdout)
                        .lines()
                        .filter_map(|line| line.trim().parse().ok())
                        .collect()
                })
                .unwrap_or_default();
            for child in self
                .children
                .lock()
                .expect("the children")
                .iter()
                .filter(|child| live.contains(child))
            {
                unsafe {
                    libc::kill(*child as libc::pid_t, libc::SIGSTOP);
                }
            }
        }
    }

    impl APointLending {
        /// **`Servers` K3 declares the break**: the lending ends, as it does
        /// when the transport reports its keepalive went unanswered — **and
        /// nothing on the machine is closed or ended by it**. Added 2026-10-01,
        /// step 10c's twelfth-pass fix piece 1.
        fn the_break_is_declared(&self) {
            self.lending.abort();
        }
    }

    /// **How long the line stays silent before the break is declared.** On a
    /// real line `Servers` K3 takes 45 to 60 seconds; here it is long enough
    /// for the write to be waiting for room and for a whole trailing window
    /// of the speed (`Server protocol and transfers` R12a's three seconds) to
    /// pass with nothing moving. **Six seconds, and the figures are read over
    /// its last two**: at the stand-in's pace the loop still met the cap's
    /// own refusals for up to about three seconds into the silence, and
    /// reported beside them, before its write began to wait for room.
    const THE_SILENCE: Duration = Duration::from_secs(6);
    /// The last stretch of the silence the page's tellings are counted over.
    const THE_LAST_OF_THE_SILENCE: Duration = Duration::from_secs(2);

    /// **What a silent cut and its break came to.**
    struct WhatTheSilenceCameTo {
        cuts: WhatTheCutsCameTo,
        /// **How many times the page was told the file moved in the
        /// silence's last two seconds** — `Server protocol and transfers`
        /// G4's time bound, kept while a wait stands.
        told_in_the_silence: usize,
        /// **The speed the screen would draw at the end of the silence.**
        the_speed_at_the_break: Option<u64>,
        /// From the break's declaration to the request reading *waiting for
        /// the connection*.
        break_to_waiting: Duration,
    }

    /// **The line goes silent under a file at `half`, the break is declared
    /// after [`THE_SILENCE`], and the line comes back** — a repair inside
    /// one period, never a *Disconnect*.
    #[allow(clippy::too_many_arguments)]
    async fn the_line_goes_silent_and_then_breaks(
        said: &str,
        machine: &Arc<AMachine>,
        point: &mut APointLending,
        the_run: &Arc<TheRunsTransfers>,
        record: &Arc<TheTimedRecord>,
        request: &str,
        direction: Direction,
        half: u64,
    ) -> WhatTheSilenceCameTo {
        let a_file_of = |the_run: &TheRunsTransfers| {
            the_run
                .what_is_under_way()
                .get(request)
                .and_then(|under_way| under_way.moving.first().cloned())
        };
        let waits = |the_run: &TheRunsTransfers| {
            the_run
                .what_is_under_way()
                .get(request)
                .is_some_and(|under_way| under_way.waits_for_the_connection)
        };
        until(&format!("{said}: half the file staged"), Duration::from_secs(300), || {
            a_file_of(the_run).is_some_and(|file| file.report.bytes_staged >= half)
        })
        .await;
        let at_the_cut = a_file_of(the_run).map_or(0, |file| file.report.bytes_staged);
        let silent_from = record.now();
        machine.every_session_goes_silent();
        tokio::time::sleep(THE_SILENCE).await;
        let the_last_window = silent_from + THE_SILENCE - THE_LAST_OF_THE_SILENCE;
        let told_in_the_silence = record
            .moving
            .lock()
            .expect("the moments")
            .iter()
            .filter(|at| **at >= the_last_window)
            .count();
        let the_speed_at_the_break = the_run.the_reading(direction).map(|reading| reading.value().0);
        let staged_at_the_break = a_file_of(the_run).map_or(0, |file| file.report.bytes_staged);
        let under_way_in_the_silence = the_run.what_is_under_way().contains_key(request) && !waits(the_run);
        println!(
            "{said}: silent at {at_the_cut} bytes; after {THE_SILENCE:?} the row read {staged_at_the_break} \
             bytes, under way {under_way_in_the_silence}, told {told_in_the_silence} times in the last \
             {THE_LAST_OF_THE_SILENCE:?}, the speed {the_speed_at_the_break:?}."
        );
        let broke = Instant::now();
        point.the_break_is_declared();
        until(
            &format!("{said}: the request waits for the connection after the break"),
            Duration::from_secs(10),
            || waits(the_run),
        )
        .await;
        let break_to_waiting = broke.elapsed();
        let while_down = {
            let screen = the_screen_now(record, the_run);
            let row = the_row(&screen, request).expect("the request is on the screen");
            (
                !as_read(&row.name).contains("did not finish"),
                row.control.as_ref().map(|control| control.act.as_str().to_string()),
                row.files.first().map(|file| file.state.to_string()).unwrap_or_default(),
                row.files
                    .first()
                    .and_then(|file| file.tail.as_ref().map(|tail| tail.as_str().to_string())),
            )
        };
        tokio::time::sleep(Duration::from_millis(500)).await;
        let back_for_good = Instant::now();
        point.the_line_comes_back(machine);
        WhatTheSilenceCameTo {
            cuts: WhatTheCutsCameTo {
                at_the_cut,
                while_down,
                back_for_good,
            },
            told_in_the_silence,
            the_speed_at_the_break,
            break_to_waiting,
        }
    }

    /// **The figures a silent cut is held to, beside the carry-on's own.**
    fn the_silence_held(said: &str, silence: &WhatTheSilenceCameTo, cap: u64) -> Vec<String> {
        let mut missed = Vec::new();
        println!(
            "{said}: the break was declared and the request read waiting {:.2} s later.",
            silence.break_to_waiting.as_secs_f64()
        );
        // **`Server protocol and transfers` G4**: the count reaches the page at
        // least once per accounting period while a wait stands, so the speed
        // on screen is recomposed and falls to nothing. The page is told at
        // most every quarter of a second; two seconds hold eight.
        a_figure!(
            missed,
            silence.told_in_the_silence >= 4,
            "{said}: the page was told {} times in the silence's last two seconds — the speed \
             on screen stood at its last figure",
            silence.told_in_the_silence
        );
        a_figure!(
            missed,
            silence.the_speed_at_the_break.is_none_or(|speed| speed < cap / 100),
            "{said}: the speed read {:?} after three seconds with nothing moving",
            silence.the_speed_at_the_break
        );
        // **`Servers` K3's declaration ends the file at once** — the channel
        // learns its connection has ended, and the write waiting for room is
        // answered *the channel is gone*.
        a_figure!(
            missed,
            silence.break_to_waiting <= Duration::from_secs(5),
            "{said}: {:.1} s from the break to waiting for the connection",
            silence.break_to_waiting.as_secs_f64()
        );
        missed
    }

    /// **A file the line cuts silently ends at the break and carries on
    /// across the repair, up and then down** — step 10c's twelfth-pass fix
    /// piece 1:
    /// `Server protocol and transfers` C11 step 4,
    /// `Server protocol and transfers` C18,
    /// `Server protocol and transfers` C19,
    /// `Server protocol and transfers` G4,
    /// `Server protocol and transfers` S12,
    /// `Servers` K3, `File Browser` K1–K4 and `Download` M11.
    ///
    /// **The fault as a person met it**, on the twelfth pass: a one-minute
    /// cut at the provider's firewall under a 1 GB upload froze the row at
    /// *916.8 MB* for good, with the last speed still showing and no
    /// *Reconnecting…*. The byte loop was waiting for room to write, which
    /// nothing ended: the channel is woken only when the machine opens its
    /// window again, never when the connection dies. **The run never
    /// returned**, so the request stayed under way, the take-up could not
    /// see the part, and the part was never offered for removal.
    ///
    /// **The quick twin, run at every desk run**: 48 MB each way, held to
    /// 8 MB/s so the cut lands inside the file.
    #[tokio::test(flavor = "multi_thread", worker_threads = 4)]
    async fn a_file_the_line_cuts_silently_carries_on_across_a_repair_up_and_down() {
        let _deadline = TheTestsDeadline::started();
        let program = the_server_or_say_so!();

        let size = 48_000_000u64;
        let cap = 8_000_000u64;

        let scratch = Scratch::new("silent");
        let here = scratch.folder("here");
        let machine_side = scratch.folder("machine");
        let chosen = scratch.folder("chosen");
        let source = here.join("silent.blend");
        a_large_file_at(&source, size);
        let record = TheTimedRecord::at(&scratch.folder("record").join("smoozy.sqlite3"));
        let server_row = a_server_row_for(&record);
        let machine = Arc::new(AMachine {
            program,
            home: scratch.at.clone(),
            limit: 10,
            ..AMachine::default()
        });
        let mut point = APointLending::over(&machine).await;
        let sessions = point.sessions.clone();
        let the_run = Arc::new(TheRunsTransfers::for_this_run());
        let capped = TheCap::Of(BytesPerSecond(cap));
        the_run.the_caps_saved_are(capped, capped);
        let within = Duration::from_secs(180);

        // ---- up
        let asked = WhatIsBeingUploaded {
            server_row: server_row.clone(),
            installation: Some(Identity::issue(IdentityKind::Installation).expect("random")),
            destination: the_bytes_of(&machine_side),
            chosen: vec![source.clone()],
            run: a_run(),
        };
        let as_a_door: Arc<dyn TheRecordAnUploadWrites> = record.clone();
        let mut ready = an_upload_is_prepared(&asked, &sessions, as_a_door.as_ref())
            .await
            .expect("the upload is prepared");
        assert_eq!(
            the_answer_is_applied(&mut ready, None, &sessions, as_a_door.as_ref()).await,
            WhatTheAnswerLeft::ItGoes
        );
        let request = ready.request.record.as_text();
        let watched = TheFileWatched::over(&the_run, request.clone());
        let (came_to, silence) = tokio::join!(
            tokio::time::timeout(
                within,
                the_upload_runs(ready, &sessions, &the_run, Arc::clone(&as_a_door), &NobodyIsTold),
            ),
            the_line_goes_silent_and_then_breaks(
                "SILENT UP", &machine, &mut point, &the_run, &record, &request, Direction::Up, size / 2
            ),
        );
        let looks = watched.ended();
        let came_to = came_to.expect("the upload came to an end inside its bound");
        let landed = machine_side.join("silent.blend");
        let mut missed = the_silence_held("SILENT UP", &silence, cap);
        missed.extend(the_carry_on_held("SILENT UP", &silence.cuts, &looks, false));
        a_figure!(missed, came_to.uploaded == 1, "SILENT UP: the upload came to {came_to:?}");
        a_figure!(
            missed,
            it_holds_the_pattern(&landed, size),
            "SILENT UP: the file on the machine is not the file that was sent"
        );
        let left = what_is_left_staged(&machine_side);
        a_figure!(missed, left.is_empty(), "SILENT UP: left under .smoozy-staging: {left:?}");

        // ---- down
        let asked = WhatIsBeingDownloaded {
            server_row,
            installation: None,
            destination: the_bytes_of(&chosen),
            picked: vec![the_bytes_of(&landed)],
            run: a_run(),
        };
        let mut ready = a_download_is_prepared(
            asked,
            &sessions,
            record.as_ref() as &dyn TheRecordADownloadWrites,
            &NoJobIsKnown,
        )
        .await
        .expect("the download is prepared");
        the_request_goes_without_a_question(&mut ready, record.as_ref())
            .expect("the dispositions are written");
        let request = ready.request.record.as_text();
        let watched = TheFileWatched::over(&the_run, request.clone());
        let (came_to, silence) = tokio::join!(
            tokio::time::timeout(
                within,
                the_download_runs(&mut ready, &sessions, &the_run, record.as_ref()),
            ),
            the_line_goes_silent_and_then_breaks(
                "SILENT DOWN", &machine, &mut point, &the_run, &record, &request, Direction::Down, size / 2
            ),
        );
        let looks = watched.ended();
        let came_to = came_to.expect("the download came to an end inside its bound");
        missed.extend(the_silence_held("SILENT DOWN", &silence, cap));
        // **`Download` M11a**: a download's carry-on starts over, and says so.
        missed.extend(the_carry_on_held("SILENT DOWN", &silence.cuts, &looks, true));
        a_figure!(missed, came_to.fetched == 1, "SILENT DOWN: the download came to {came_to:?}");
        a_figure!(
            missed,
            it_holds_the_pattern(&chosen.join("silent.blend"), size),
            "SILENT DOWN: the file on this computer is not the file on the machine"
        );
        machine.every_session_ends();
        the_figures_held("A SILENT CUT, CARRIED ON", missed);
    }

    /// **A *Disconnect* while the line is silent leaves the upload a leftover
    /// with *Remove*, and asking again takes its part up and verifies it** —
    /// step 10c's twelfth-pass fix piece 1: `File Browser` U7,
    /// `File Browser` U8, `Server protocol and transfers` S12 and
    /// `Server protocol and transfers` S26.
    ///
    /// **The fault as a person met it**, on the twelfth pass: after the silent
    /// cut, a *Disconnect* left a row reading *did not finish · 0 / 1* **with
    /// no *part-sent … Remove***, though 914 MB sat staged on the server — the
    /// run had never returned, so the transfer's record had no ending — and
    /// taking it up sent the file again from nought.
    #[tokio::test(flavor = "multi_thread", worker_threads = 4)]
    async fn an_upload_a_disconnect_cuts_on_a_silent_line_is_a_leftover_and_asking_again_takes_it_up() {
        let _deadline = TheTestsDeadline::started();
        let program = the_server_or_say_so!();

        let size = 24_000_000u64;

        let scratch = Scratch::new("silent-leftover");
        let here = scratch.folder("here");
        let source = here.join("part.blend");
        a_large_file_at(&source, size);
        let machine_side = scratch.folder("machine");
        let record = TheTimedRecord::at(&scratch.folder("record").join("smoozy.sqlite3"));
        let (server_row, installation) = a_server_row_installed(&record);
        let the_server = server_row.as_text();
        let machine = Arc::new(AMachine {
            program,
            home: scratch.at.clone(),
            limit: 10,
            ..AMachine::default()
        });
        let point = APointLending::over(&machine).await;
        let the_run = Arc::new(TheRunsTransfers::for_this_run());
        let cap = TheCap::Of(BytesPerSecond(4_000_000));
        the_run.the_caps_saved_are(cap, cap);
        let as_a_door: Arc<dyn TheRecordAnUploadWrites> = record.clone();

        let asked = WhatIsBeingUploaded {
            server_row: server_row.clone(),
            installation: Some(installation.clone()),
            destination: the_bytes_of(&machine_side),
            chosen: vec![source.clone()],
            run: a_run(),
        };
        let mut ready = an_upload_is_prepared(&asked, &point.sessions, as_a_door.as_ref())
            .await
            .expect("the upload is prepared");
        assert_eq!(
            the_answer_is_applied(&mut ready, None, &point.sessions, as_a_door.as_ref()).await,
            WhatTheAnswerLeft::ItGoes
        );
        let request = ready.request.record.as_text();
        // **Two checkpoints across, then the line goes silent, and the user
        // presses *Disconnect* while it is** — the period ends and the
        // lending with it, and nothing on the machine closes.
        let cutting = async {
            let two_checkpoints =
                2 * smoozy_server_protocol::transfers::moving::THE_CHECKPOINT_INTERVAL + 1_000_000;
            until("SILENT LEFTOVER: two checkpoints across", Duration::from_secs(60), || {
                the_run.what_is_under_way().get(&request).is_some_and(|under_way| {
                    under_way
                        .moving
                        .iter()
                        .any(|file| file.report.bytes_staged >= two_checkpoints)
                })
            })
            .await;
            machine.every_session_goes_silent();
            tokio::time::sleep(Duration::from_secs(1)).await;
            point.the_break_is_declared();
            the_run.a_servers_period_ended(&the_server);
        };
        let (came_to, ()) = tokio::join!(
            tokio::time::timeout(
                Duration::from_secs(60),
                the_upload_runs(ready, &point.sessions, &the_run, Arc::clone(&as_a_door), &NobodyIsTold),
            ),
            cutting
        );
        let came_to = came_to.expect("the upload returned after the Disconnect");
        machine.every_session_ends();
        println!(
            "SILENT LEFTOVER: came to {came_to:?}; the endings {:?}; under way {}.",
            the_endings(&record),
            the_run.what_is_under_way().contains_key(&request)
        );

        let mut missed = Vec::new();
        a_figure!(
            missed,
            !the_run.what_is_under_way().contains_key(&request),
            "SILENT LEFTOVER: the request is still under way after the Disconnect"
        );
        let screen = the_screen_now(&record, &the_run);
        let row = the_row(&screen, &request).expect("the request is on the screen");
        let control = row.control.as_ref().map(|control| control.act.as_str().to_string());
        println!(
            "SILENT LEFTOVER: the row reads «{}» · «{}» · «{}», control {control:?}.",
            as_read(&row.name),
            as_read(&row.count),
            as_read(&row.where_it_goes)
        );
        a_figure!(missed, control.as_deref() == Some("remove"), "SILENT LEFTOVER: the row offers {control:?}, not Remove");
        a_figure!(
            missed,
            as_read(&row.where_it_goes).contains("partial files still use space on the server"),
            "SILENT LEFTOVER: the row does not say partial files are still on the server"
        );

        // ---- Connect again, and ask for the same file again
        let point = APointLending::over(&machine).await;
        let asked = WhatIsBeingUploaded {
            server_row: server_row.clone(),
            installation: Some(installation.clone()),
            destination: the_bytes_of(&machine_side),
            chosen: vec![source.clone()],
            run: a_run(),
        };
        let mut ready = an_upload_is_prepared(&asked, &point.sessions, as_a_door.as_ref())
            .await
            .expect("the second upload is prepared");
        assert_eq!(
            the_answer_is_applied(&mut ready, None, &point.sessions, as_a_door.as_ref()).await,
            WhatTheAnswerLeft::ItGoes
        );
        let again = ready.request.record.as_text();
        let watched = TheFileWatched::over(&the_run, again.clone());
        let came_to = tokio::time::timeout(
            Duration::from_secs(60),
            the_upload_runs(ready, &point.sessions, &the_run, Arc::clone(&as_a_door), &NobodyIsTold),
        )
        .await
        .expect("the second upload came to an end");
        let looks = watched.ended();
        let verified = looks.iter().find_map(|look| look.verifying);
        let lowest = looks.iter().map(|look| look.staged).filter(|staged| *staged > 0).min();
        println!(
            "SILENT LEFTOVER, ASKED AGAIN: came to {came_to:?}; the part verified {verified:?}; the \
             lowest figure above nought {lowest:?}; left staged {:?}.",
            what_is_left_staged(&machine_side)
        );
        a_figure!(missed, came_to.uploaded == 1, "SILENT LEFTOVER, ASKED AGAIN: came to {came_to:?}");
        a_figure!(
            missed,
            verified.is_some(),
            "SILENT LEFTOVER, ASKED AGAIN: the part already sent was never verified — it was not taken up"
        );
        a_figure!(
            missed,
            lowest.is_some_and(|lowest| lowest >= smoozy_server_protocol::transfers::moving::THE_CHECKPOINT_INTERVAL),
            "SILENT LEFTOVER, ASKED AGAIN: the file was sent from {lowest:?}, not from the part already there"
        );
        a_figure!(
            missed,
            it_holds_the_pattern(&machine_side.join("part.blend"), size),
            "SILENT LEFTOVER, ASKED AGAIN: the file on the machine is not the file that was sent"
        );
        a_figure!(
            missed,
            what_is_left_staged(&machine_side).is_empty(),
            "SILENT LEFTOVER, ASKED AGAIN: something is left staged"
        );
        machine.every_session_ends();
        the_figures_held("A DISCONNECT ON A SILENT LINE", missed);
    }

    // ----------- an upload lent nothing (check-before-0.1.7 fix piece 1)

    /// **An upload whose walk is lent nothing never reads *done*** — step
    /// 10c's check-before-0.1.7 fix piece 1, run at every desk run.
    ///
    /// **The case it bites on.** On the Hyperstack check, the same file
    /// dragged in again over a connection that was gone read ***UPLOAD 0 files
    /// into /home/ubuntu · done*** at once: `File Browser` U1 writes the
    /// request down before the walk, the walk was lent nothing, and the row
    /// counted the units with a disposition — none — and called none of them
    /// unfinished. **Here the line is cut and the lending with it**, the upload
    /// is asked for anyway, and the row must read what was asked for and did
    /// not go: *1 file into … — Not started · 0 / 1* (`Design/Transfers`,
    /// the build runner’s ruling of 2026-11-04). Its walk never began.
    #[tokio::test(flavor = "multi_thread", worker_threads = 4)]
    async fn an_upload_whose_walk_is_lent_nothing_does_not_read_done() {
        use smoozy_desktop::uploading::WhyTheUploadDidNotStart;
        let _deadline = TheTestsDeadline::started();
        let program = the_server_or_say_so!();

        let scratch = Scratch::new("lent-nothing");
        let here = scratch.folder("here");
        let file = here.join("c17-1gb.bin");
        a_large_file_at(&file, 1_000_000);
        let machine_side = scratch.folder("machine");
        let record = TheTimedRecord::at(&scratch.folder("record").join("smoozy.sqlite3"));
        let (server_row, installation) = a_server_row_installed(&record);
        let machine = Arc::new(AMachine {
            program,
            home: scratch.at.clone(),
            limit: 10,
            ..AMachine::default()
        });
        let point = APointLending::over(&machine).await;
        let the_run = Arc::new(TheRunsTransfers::for_this_run());
        point.the_line_is_cut(&machine);
        while point.sessions.a_connection_is_live() {
            tokio::task::yield_now().await;
        }

        let as_a_door: Arc<dyn TheRecordAnUploadWrites> = record.clone();
        let asked = WhatIsBeingUploaded {
            server_row: server_row.clone(),
            installation: Some(installation.clone()),
            destination: the_bytes_of(&machine_side),
            chosen: vec![file.clone()],
            run: a_run(),
        };
        let prepared = an_upload_is_prepared(&asked, &point.sessions, as_a_door.as_ref()).await;
        assert!(
            matches!(prepared, Err(WhyTheUploadDidNotStart::TheServerLentNothing(_))),
            "the walk was lent something over a cut line"
        );

        let screen = the_screen_now(&record, &the_run);
        let rows: Vec<&ARequestDrawn> = screen.requests.iter().filter(|row| row.tag_class == "up").collect();
        assert_eq!(rows.len(), 1, "the request written down is on the screen");
        let (name, count) = (as_read(&rows[0].name), as_read(&rows[0].count));
        println!("LENT NOTHING: the row reads {name:?} · {count:?}");
        assert!(name.ends_with("— Not started"), "the row reads {name:?} · {count:?}");
        assert_ne!(count, "done", "the row reads {name:?} · {count:?}");
        assert!(count.contains("0") && count.contains("1"), "the row reads {name:?} · {count:?}");
    }

    /// **An upload waiting on its question reads under way, and one answered
    /// *Stop* reads *— did not finish*** — never *done* either way. Step 10c's
    /// check-before-0.1.7 fix piece 1, run at every desk run: both have no
    /// unit with a disposition, which the row used to count as nothing left to
    /// do.
    #[tokio::test(flavor = "multi_thread", worker_threads = 4)]
    async fn an_upload_at_its_question_reads_under_way_and_stopped_there_does_not_read_done() {
        use smoozy_file_browser::already_exists::CollisionAnswer;
        let _deadline = TheTestsDeadline::started();
        let program = the_server_or_say_so!();

        let scratch = Scratch::new("at-the-question");
        let here = scratch.folder("here");
        let file = here.join("c17-1gb.bin");
        a_large_file_at(&file, 1_000_000);
        let machine_side = scratch.folder("machine");
        std::fs::copy(&file, machine_side.join("c17-1gb.bin")).expect("already there");
        let record = TheTimedRecord::at(&scratch.folder("record").join("smoozy.sqlite3"));
        let (server_row, installation) = a_server_row_installed(&record);
        let machine = Arc::new(AMachine {
            program,
            home: scratch.at.clone(),
            limit: 10,
            ..AMachine::default()
        });
        let point = APointLending::over(&machine).await;
        let the_run = Arc::new(TheRunsTransfers::for_this_run());
        let as_a_door: Arc<dyn TheRecordAnUploadWrites> = record.clone();
        let asked = WhatIsBeingUploaded {
            server_row: server_row.clone(),
            installation: Some(installation.clone()),
            destination: the_bytes_of(&machine_side),
            chosen: vec![file.clone()],
            run: a_run(),
        };
        let mut ready = an_upload_is_prepared(&asked, &point.sessions, as_a_door.as_ref())
            .await
            .expect("the upload is prepared");
        assert!(ready.something_collides(), "the file is already there");
        let request = ready.request.record.as_text();
        let read = |screen: &TheTransfersScreen| {
            let row = the_row(screen, &request).expect("the request is on the screen");
            (as_read(&row.name), as_read(&row.count))
        };

        let (name, count) = read(&the_screen_now(&record, &the_run));
        println!("AT THE QUESTION: the row reads {name:?} · {count:?}");
        assert_ne!(count, "done", "a request at its question read done");
        assert!(!name.ends_with("— did not finish"), "a request at its question read ended");

        assert_eq!(
            the_answer_is_applied(&mut ready, Some(CollisionAnswer::Stop), &point.sessions, as_a_door.as_ref())
                .await,
            WhatTheAnswerLeft::TheUserStoppedIt
        );
        let (name, count) = read(&the_screen_now(&record, &the_run));
        println!("STOPPED THERE: the row reads {name:?} · {count:?}");
        assert!(name.ends_with("— did not finish"), "the row reads {name:?} · {count:?}");
        assert_eq!(count, "0 / 1", "the row reads {name:?} · {count:?}");
    }

    // --------------------------------------------------------- the figures held

    fn the_figures_held(said: &str, missed: Vec<String>) {
        assert!(missed.is_empty(), "{said}: figures missed: {missed:#?}");
    }

    // ------------------------------- many files at once (eleventh-pass piece 4)

    /// **The most files of one request the run's register held moving at
    /// once**, looked at every millisecond — what its Transfers row draws.
    struct TheMostAtOnce {
        stop: Arc<AtomicBool>,
        looking: Option<std::thread::JoinHandle<usize>>,
    }

    impl TheMostAtOnce {
        fn over(the_run: &Arc<TheRunsTransfers>, request: String) -> TheMostAtOnce {
            let stop = Arc::new(AtomicBool::new(false));
            let (the_run, stopping) = (Arc::clone(the_run), Arc::clone(&stop));
            let looking = std::thread::spawn(move || {
                let mut most = 0;
                while !stopping.load(Ordering::SeqCst) {
                    if let Some(under_way) = the_run.what_is_under_way().get(&request) {
                        most = most.max(under_way.moving.len());
                    }
                    std::thread::sleep(Duration::from_millis(1));
                }
                most
            });
            TheMostAtOnce {
                stop,
                looking: Some(looking),
            }
        }

        fn ended(mut self) -> usize {
            self.stop.store(true, Ordering::SeqCst);
            self.looking
                .take()
                .map(|looking| looking.join().expect("the looks"))
                .unwrap_or_default()
        }
    }

    impl Drop for TheMostAtOnce {
        fn drop(&mut self) {
            self.stop.store(true, Ordering::SeqCst);
        }
    }

    /// **One request's files fill `File Browser` O1's four channels up and
    /// `Download` Q1's two down, and every file lands whole** — step 10c's
    /// eleventh-pass fix piece 4, run at every desk run. **And the request
    /// asks for a staging folder once per folder, not once per file** (`Server
    /// protocol and transfers` S1): the loop's own ask before each file stays,
    /// so the count is the files plus at most one per channel per folder.
    ///
    /// **The fault as a person met it**: a thousand small files went up one at
    /// a time — 46 minutes at a round trip of 0.2 s — and came down one at a
    /// time, while four channels up and two down stood unused.
    #[tokio::test(flavor = "multi_thread", worker_threads = 4)]
    async fn a_requests_files_fill_the_four_channels_up_and_the_two_down() {
        let _deadline = TheTestsDeadline::started();
        let program = the_server_or_say_so!();

        let (files, folders) = (24usize, 2usize);
        let size_of = |at: usize| 65_536 + 1_009 * at as u64;
        let scratch = Scratch::new("many-at-once");
        let here = scratch.folder("here");
        for at in 0..files {
            let folder = here.join(format!("f{}", at % folders));
            std::fs::create_dir_all(&folder).expect("the folder");
            a_large_file_at(&folder.join(format!("file_{at:02}.bin")), size_of(at));
        }
        let machine_side = scratch.folder("machine");
        let chosen = scratch.folder("chosen");
        let record = TheTimedRecord::at(&scratch.folder("record").join("smoozy.sqlite3"));
        let server_row = a_server_row_for(&record);
        let machine = Arc::new(AMachine {
            program,
            home: scratch.at.clone(),
            limit: 10,
            ..AMachine::default()
        });
        let point = APointLending::over(&machine).await;
        let the_run = Arc::new(TheRunsTransfers::for_this_run());
        let as_a_door: Arc<dyn TheRecordAnUploadWrites> = record.clone();

        // ---- up
        let asked = WhatIsBeingUploaded {
            server_row: server_row.clone(),
            installation: Some(Identity::issue(IdentityKind::Installation).expect("random")),
            destination: the_bytes_of(&machine_side),
            chosen: vec![here.clone()],
            run: a_run(),
        };
        let mut ready = an_upload_is_prepared(&asked, &point.sessions, as_a_door.as_ref())
            .await
            .expect("the upload is prepared");
        assert_eq!(
            the_answer_is_applied(&mut ready, None, &point.sessions, as_a_door.as_ref()).await,
            WhatTheAnswerLeft::ItGoes
        );
        let watched = TheMostAtOnce::over(&the_run, ready.request.record.as_text());
        let asked_before = machine.staging_folders_asked.load(Ordering::SeqCst);
        let came_to = the_upload_runs(
            ready,
            &point.sessions,
            &the_run,
            Arc::clone(&as_a_door),
            &NobodyIsTold,
        )
        .await;
        let up_at_once = watched.ended();
        let staging_asked = machine.staging_folders_asked.load(Ordering::SeqCst) - asked_before;

        // ---- down, by hand
        let asked = WhatIsBeingDownloaded {
            server_row,
            installation: None,
            destination: the_bytes_of(&chosen),
            picked: vec![the_bytes_of(&machine_side.join("here"))],
            run: a_run(),
        };
        let mut ready = a_download_is_prepared(
            asked,
            &point.sessions,
            record.as_ref() as &dyn TheRecordADownloadWrites,
            &NoJobIsKnown,
        )
        .await
        .expect("the download is prepared");
        the_request_goes_without_a_question(&mut ready, record.as_ref())
            .expect("the dispositions are written");
        let watched = TheMostAtOnce::over(&the_run, ready.request.record.as_text());
        let came_down =
            the_download_runs(&mut ready, &point.sessions, &the_run, record.as_ref()).await;
        let down_at_once = watched.ended();
        println!(
            "MANY AT ONCE: up {came_to:?}, {up_at_once} at once, the staging folder asked for \
             {staging_asked} times for {files} files in {folders} folders; down {came_down:?}, \
             {down_at_once} at once."
        );

        let mut missed = Vec::new();
        a_figure!(
            missed,
            came_to.uploaded == files && came_to.did_not_finish == 0,
            "UP: came to {came_to:?}"
        );
        a_figure!(
            missed,
            (2..=4).contains(&up_at_once),
            "UP: {up_at_once} files of the request moved at once, not two to four"
        );
        a_figure!(
            missed,
            staging_asked <= files + 4 * folders,
            "UP: the staging folder was asked for {staging_asked} times for {files} files — once per \
             file by the request as well as by the loop"
        );
        a_figure!(
            missed,
            came_down.fetched == files && came_down.did_not_finish == 0,
            "DOWN: came to {came_down:?}"
        );
        a_figure!(
            missed,
            down_at_once == 2,
            "DOWN: {down_at_once} files of the request moved at once, not Download Q1's two"
        );
        for at in 0..files {
            let name = format!("f{}/file_{at:02}.bin", at % folders);
            a_figure!(
                missed,
                it_holds_the_pattern(&machine_side.join("here").join(&name), size_of(at)),
                "UP: {name} did not land whole"
            );
            a_figure!(
                missed,
                it_holds_the_pattern(&chosen.join("here").join(&name), size_of(at)),
                "DOWN: {name} did not land whole"
            );
        }
        the_figures_held("MANY AT ONCE", missed);
    }

    // ------------------------------------ many small files, counted (owed piece 1)

    /// **Small files in folders a request is to make**: `files` of 1–8 KB
    /// spread over `folders`, under `here`. Added 2026-10-16, step 10c's owed
    /// piece 1.
    fn small_files_in_folders(here: &Path, files: usize, folders: usize, named: &str) {
        for at in 0..files {
            let folder = here.join(format!("folder-{:02}", at % folders));
            std::fs::create_dir_all(&folder).expect("the folder is made");
            let mut bytes = vec![0u8; 1_024 * (1 + at % 8)];
            the_pattern_at(at as u64 * 8, &mut bytes);
            std::fs::write(folder.join(format!("{named}_{at:04}.bin")), &bytes).expect("written");
        }
    }

    fn a_small_file_landed(folder: &Path, at: usize, folders: usize, named: &str) -> bool {
        let mut bytes = vec![0u8; 1_024 * (1 + at % 8)];
        the_pattern_at(at as u64 * 8, &mut bytes);
        std::fs::read(folder.join(format!("folder-{:02}/{named}_{at:04}.bin", at % folders)))
            .is_ok_and(|landed| landed == bytes)
    }

    /// **A fresh file takes no staging-folder ask, staged-part read or removal
    /// of its own; a file whose staging folder its request did not make takes
    /// all three, as before** — `Server protocol and transfers` S12a
    /// (the build runner's ruling of 2026-10-16), step 10c's owed piece 1. Run at every desk
    /// run.
    ///
    /// **The fault as a person met it**: on the thirteenth pass 1,000 small
    /// files went up at 0.94 s a file, and before every one of them the loop
    /// asked for the staging folder its own request had just made, read the
    /// staged part a fresh transfer cannot have, and removed it.
    ///
    /// **What is held**: the first request's folders are all new, so each
    /// staging folder is asked for once, by the request, and no file reads or
    /// removes a staged part. The second request lands beside staging folders
    /// that already stand, so every file asks all three before its staging
    /// object, as it always did.
    #[tokio::test(flavor = "multi_thread", worker_threads = 4)]
    async fn a_fresh_file_takes_no_staging_folder_ask_read_or_removal_of_its_own() {
        let _deadline = TheTestsDeadline::started();
        let program = the_server_or_say_so!();

        let (files, folders) = (12usize, 3usize);
        let scratch = Scratch::new("fresh-files");
        let first = scratch.folder("first/many");
        small_files_in_folders(&first, files, folders, "first");
        let second = scratch.folder("second/many");
        small_files_in_folders(&second, files, folders, "second");
        let machine_side = scratch.folder("machine");
        let record = TheTimedRecord::at(&scratch.folder("record").join("smoozy.sqlite3"));
        let server_row = a_server_row_for(&record);
        let machine = Arc::new(AMachine {
            program,
            home: scratch.at.clone(),
            limit: 10,
            ..AMachine::default()
        });
        let point = APointLending::over(&machine).await;
        let the_run = Arc::new(TheRunsTransfers::for_this_run());
        let as_a_door: Arc<dyn TheRecordAnUploadWrites> = record.clone();

        let mut counted = Vec::new();
        for chosen in [&first, &second] {
            let asked = WhatIsBeingUploaded {
                server_row: server_row.clone(),
                installation: Some(Identity::issue(IdentityKind::Installation).expect("random")),
                destination: the_bytes_of(&machine_side),
                chosen: vec![chosen.clone()],
                run: a_run(),
            };
            let mut ready = an_upload_is_prepared(&asked, &point.sessions, as_a_door.as_ref())
                .await
                .expect("the upload is prepared");
            assert_eq!(
                the_answer_is_applied(&mut ready, None, &point.sessions, as_a_door.as_ref()).await,
                WhatTheAnswerLeft::ItGoes
            );
            let before = machine.the_count_now();
            let came_to = the_upload_runs(
                ready,
                &point.sessions,
                &the_run,
                Arc::clone(&as_a_door),
                &NobodyIsTold,
            )
            .await;
            let count = machine.the_count_now().since(&before);
            println!(
                "FRESH FILES: came to {came_to:?}; {}.",
                count.per_file(files)
            );
            counted.push((came_to, count));
        }

        let mut missed = Vec::new();
        let (first_came_to, fresh) = &counted[0];
        let (second_came_to, standing) = &counted[1];
        for (said, came_to) in [("FIRST", first_came_to), ("SECOND", second_came_to)] {
            a_figure!(
                missed,
                came_to.uploaded == files
                    && came_to.did_not_finish == 0
                    && came_to.failed.is_empty(),
                "{said}: came to {came_to:?}"
            );
        }
        a_figure!(
            missed,
            fresh.staging_folders == folders,
            "FIRST: the staging folder was asked for {} times for {files} files in {folders} new \
             folders — the loop asked again for each file",
            fresh.staging_folders
        );
        a_figure!(
            missed,
            fresh.staging_reads == 0,
            "FIRST: {} staged parts were read for {files} fresh files",
            fresh.staging_reads
        );
        a_figure!(
            missed,
            fresh.staging_removes == 0,
            "FIRST: {} staged parts were removed for {files} fresh files",
            fresh.staging_removes
        );
        // **Where the request did not make the staging folder, nothing is
        // skipped**: each file asks for it, reads its staged part and removes
        // it before its staging object, exactly as before the paragraph.
        a_figure!(
            missed,
            standing.staging_reads == files && standing.staging_removes == files,
            "SECOND: beside staging folders already standing, {} reads and {} removals for {files} \
             files, not one of each per file",
            standing.staging_reads,
            standing.staging_removes
        );
        a_figure!(
            missed,
            standing.staging_folders >= files,
            "SECOND: the staging folder was asked for {} times for {files} files, not once per file \
             by the loop",
            standing.staging_folders
        );
        for at in 0..files {
            a_figure!(
                missed,
                a_small_file_landed(&machine_side.join("many"), at, folders, "first"),
                "FIRST: file {at} did not land whole"
            );
            a_figure!(
                missed,
                a_small_file_landed(&machine_side.join("many"), at, folders, "second"),
                "SECOND: file {at} did not land whole"
            );
        }
        the_figures_held("FRESH FILES", missed);
    }

    /// **A thousand small files up, then down by hand, at a round trip, each
    /// file's wait counted** — step 10c's owed piece 1, the count before the
    /// cut and after it. The shape of the thirteenth pass's: 1,000 files of
    /// 1–8 KB in 45 folders, one request each way.
    ///
    /// **The figures are read, not held**: this test asserts only that every
    /// file landed whole both ways. `SMOOZY_HARNESS_ROUND_TRIP_MS` (200 when
    /// unset), `SMOOZY_HARNESS_SMALL_FILES` (1,000).
    #[tokio::test(flavor = "multi_thread", worker_threads = 4)]
    #[ignore = "the kept harness: minutes; run on purpose"]
    async fn a_thousand_small_files_go_up_and_come_down_at_a_round_trip() {
        let _deadline = TheTestsDeadline::started();
        let program = the_server_or_say_so!();

        let round_trip = Duration::from_millis(a_number_from("SMOOZY_HARNESS_ROUND_TRIP_MS", 200));
        let files = a_number_from("SMOOZY_HARNESS_SMALL_FILES", 1_000) as usize;
        const FOLDERS: usize = 45;

        let scratch = Scratch::new("small-paced");
        let here = scratch.folder("here/p13-many");
        small_files_in_folders(&here, files, FOLDERS, "file");
        let machine_side = scratch.folder("machine");
        let chosen = scratch.folder("chosen");
        let record = TheTimedRecord::at(&scratch.folder("record").join("smoozy.sqlite3"));
        let server_row = a_server_row_for(&record);
        let machine = Arc::new(AMachine {
            program,
            home: scratch.at.clone(),
            limit: 10,
            round_trip,
            ..AMachine::default()
        });
        let point = APointLending::over(&machine).await;
        let the_run = Arc::new(TheRunsTransfers::for_this_run());
        let as_a_door: Arc<dyn TheRecordAnUploadWrites> = record.clone();
        println!(
            "SMALL: {files} files of 1–8 KB in {FOLDERS} folders, a round trip of {:.2} s.",
            round_trip.as_secs_f64()
        );

        // ---- up
        let asked = WhatIsBeingUploaded {
            server_row: server_row.clone(),
            installation: Some(Identity::issue(IdentityKind::Installation).expect("random")),
            destination: the_bytes_of(&machine_side),
            chosen: vec![here.clone()],
            run: a_run(),
        };
        let began = Instant::now();
        let mut ready = an_upload_is_prepared(&asked, &point.sessions, as_a_door.as_ref())
            .await
            .expect("the upload is prepared");
        assert_eq!(
            the_answer_is_applied(&mut ready, None, &point.sessions, as_a_door.as_ref()).await,
            WhatTheAnswerLeft::ItGoes
        );
        let preparing_up = began.elapsed();
        let before = machine.the_count_now();
        let began = Instant::now();
        let came_to = the_upload_runs(
            ready,
            &point.sessions,
            &the_run,
            Arc::clone(&as_a_door),
            &NobodyIsTold,
        )
        .await;
        let up_took = began.elapsed();
        let up = machine.the_count_now().since(&before);
        println!(
            "SMALL UP: {:.1} s ({:.3} s a file, {:.2} files a second), and {:.1} s preparing; came to \
             {came_to:?}.",
            up_took.as_secs_f64(),
            up_took.as_secs_f64() / files as f64,
            files as f64 / up_took.as_secs_f64(),
            preparing_up.as_secs_f64()
        );
        println!("SMALL UP: {}.", up.per_file(files));

        // ---- down, by hand
        let asked = WhatIsBeingDownloaded {
            server_row,
            installation: None,
            destination: the_bytes_of(&chosen),
            picked: vec![the_bytes_of(&machine_side.join("p13-many"))],
            run: a_run(),
        };
        let began = Instant::now();
        let mut ready = a_download_is_prepared(
            asked,
            &point.sessions,
            record.as_ref() as &dyn TheRecordADownloadWrites,
            &NoJobIsKnown,
        )
        .await
        .expect("the download is prepared");
        the_request_goes_without_a_question(&mut ready, record.as_ref())
            .expect("the dispositions are written");
        let preparing_down = began.elapsed();
        let before = machine.the_count_now();
        let began = Instant::now();
        let came_down =
            the_download_runs(&mut ready, &point.sessions, &the_run, record.as_ref()).await;
        let down_took = began.elapsed();
        let down = machine.the_count_now().since(&before);
        println!(
            "SMALL DOWN: {:.1} s ({:.3} s a file, {:.2} files a second), and {:.1} s preparing; came \
             to {came_down:?}.",
            down_took.as_secs_f64(),
            down_took.as_secs_f64() / files as f64,
            files as f64 / down_took.as_secs_f64(),
            preparing_down.as_secs_f64()
        );
        println!("SMALL DOWN: {}.", down.per_file(files));

        assert_eq!(came_to.uploaded, files, "{came_to:?}");
        assert_eq!(came_down.fetched, files, "{came_down:?}");
        for at in 0..files {
            assert!(
                a_small_file_landed(&machine_side.join("p13-many"), at, FOLDERS, "file"),
                "up: file {at} did not land whole"
            );
            assert!(
                a_small_file_landed(&chosen.join("p13-many"), at, FOLDERS, "file"),
                "down: file {at} did not land whole"
            );
        }
    }

    /// **A cap still holds across four channels, and the speed beside it agrees
    /// with the bytes moved** — step 10c's eleventh-pass fix piece 4, the map's
    /// fourth desk check (`Desktop app and delivery` R7 and R15, `Server
    /// protocol and transfers` R12a). The bytes moved are the files landed and
    /// the bytes staged of every file in flight, each file once.
    #[tokio::test(flavor = "multi_thread", worker_threads = 4)]
    async fn a_cap_holds_across_four_channels_and_the_speed_beside_it_agrees() {
        let _deadline = TheTestsDeadline::started();
        let program = the_server_or_say_so!();

        let (files, size, capped) = (10usize, 1_000_000u64, 1_000_000u64);
        let scratch = Scratch::new("cap-at-once");
        let here = scratch.folder("here/capped");
        for at in 0..files {
            a_large_file_at(&here.join(format!("file_{at:02}.bin")), size);
        }
        let machine_side = scratch.folder("machine");
        let record = TheTimedRecord::at(&scratch.folder("record").join("smoozy.sqlite3"));
        let server_row = a_server_row_for(&record);
        let machine = Arc::new(AMachine {
            program,
            home: scratch.at.clone(),
            limit: 10,
            ..AMachine::default()
        });
        let point = APointLending::over(&machine).await;
        let the_run = Arc::new(TheRunsTransfers::for_this_run());
        the_run.a_cap_was_saved(Direction::Up, TheCap::Of(BytesPerSecond(capped)));
        let as_a_door: Arc<dyn TheRecordAnUploadWrites> = record.clone();
        let asked = WhatIsBeingUploaded {
            server_row,
            installation: Some(Identity::issue(IdentityKind::Installation).expect("random")),
            destination: the_bytes_of(&machine_side),
            chosen: vec![here.clone()],
            run: a_run(),
        };
        let mut ready = an_upload_is_prepared(&asked, &point.sessions, as_a_door.as_ref())
            .await
            .expect("the upload is prepared");
        assert_eq!(
            the_answer_is_applied(&mut ready, None, &point.sessions, as_a_door.as_ref()).await,
            WhatTheAnswerLeft::ItGoes
        );
        let request = ready.request.record.as_text();
        let most = TheMostAtOnce::over(&the_run, request.clone());
        // **The speed and the bytes, every quarter of a second.**
        let stop = Arc::new(AtomicBool::new(false));
        let taking = {
            let (the_run, stopping, landed_in) = (
                Arc::clone(&the_run),
                Arc::clone(&stop),
                machine_side.join("capped"),
            );
            std::thread::spawn(move || {
                let began = Instant::now();
                let mut seen = Vec::new();
                while !stopping.load(Ordering::SeqCst) {
                    let reading = the_run
                        .the_reading(Direction::Up)
                        .map(|reading| reading.value().0);
                    let mut moved = 0u64;
                    // **A file in flight whose name already stands has
                    // landed**, and is counted once, as landed.
                    if let Some(under_way) = the_run.what_is_under_way().get(&request) {
                        for file in &under_way.moving {
                            let path =
                                PathBuf::from(String::from_utf8_lossy(&file.path).into_owned());
                            if !path.exists() {
                                moved += file.report.bytes_staged;
                            }
                        }
                    }
                    for at in 0..files {
                        let landed = landed_in.join(format!("file_{at:02}.bin"));
                        if landed.exists() {
                            moved += std::fs::metadata(&landed)
                                .map(|held| held.len())
                                .unwrap_or(0);
                        }
                    }
                    seen.push((began.elapsed(), reading, moved));
                    std::thread::sleep(Duration::from_millis(250));
                }
                seen
            })
        };
        let began = Instant::now();
        let came_to = the_upload_runs(
            ready,
            &point.sessions,
            &the_run,
            Arc::clone(&as_a_door),
            &NobodyIsTold,
        )
        .await;
        let took = began.elapsed();
        stop.store(true, Ordering::SeqCst);
        let seen = taking.join().expect("the speeds");
        let at_once = most.ended();
        let owed = Duration::from_secs_f64((files as u64 * size) as f64 / capped as f64);
        println!(
            "CAP AT ONCE: {came_to:?} in {:.1} s against the cap's {:.1} s; {at_once} at once.",
            took.as_secs_f64(),
            owed.as_secs_f64()
        );
        // **The speed beside the cap against the bytes moved, window for
        // window**, over the steady stretch — from three seconds after the
        // first reading, the window being full there, to the last reading
        // taken while bytes moved. Each reading is set beside the bytes moved
        // over the trailing window that ends with it
        // (`smoozy_desktop::scheduling::THE_TRAILING_WINDOW`), and the two
        // averages are compared. **Held to within 15 %**: `the_speed_agreed`'s
        // test of the readings' own spread is too fine for it — it missed by
        // 1 % of a reading at the cap once in four runs at the desk.
        //
        // **Window for window, and not the stretch's ends, since the
        // hosted run of 2026-09-30.** The cap lets a whole window's worth
        // through at once and then nothing until it leaves the window
        // (`Server protocol and transfers` R6b's trailing window), so the
        // bytes land as a staircase — about three megabytes every three
        // seconds here. Taken from the stretch's two ends alone, a stretch of
        // six seconds held one step or two depending on where the steps fell:
        // 0.89 MB/s moved at this desk and 0.52 on the hosted Linux runner,
        // with the reading at the cap on both.
        let window = THE_TRAILING_WINDOW;
        let steady: Vec<(Duration, u64, u64)> = seen
            .iter()
            .filter_map(|(at, reading, moved)| reading.map(|reading| (*at, reading, *moved)))
            .collect();
        let first = steady.first().map(|one| one.0).unwrap_or_default();
        let paired: Vec<(f64, f64)> = steady
            .into_iter()
            .filter(|one| one.0 >= first + window && one.2 < files as u64 * size)
            .filter_map(|(at, reading, moved)| {
                // The sample nearest one window before this one.
                let (then, moved_then) = seen
                    .iter()
                    .filter(|earlier| earlier.0 < at)
                    .min_by_key(|earlier| {
                        (earlier.0.as_millis() as i128
                            - (at.as_millis() as i128 - window.as_millis() as i128))
                            .abs()
                    })
                    .map(|earlier| (earlier.0, earlier.2))?;
                let span = (at - then).as_secs_f64();
                (span > 0.0).then(|| {
                    (
                        reading as f64,
                        moved.saturating_sub(moved_then) as f64 / span,
                    )
                })
            })
            .collect();
        let mut missed = Vec::new();
        if paired.is_empty() {
            missed.push("CAP AT ONCE: no steady stretch was read".to_string());
        } else {
            let read = paired.iter().map(|one| one.0).sum::<f64>() / paired.len() as f64;
            let moved = paired.iter().map(|one| one.1).sum::<f64>() / paired.len() as f64;
            println!(
                "CAP AT ONCE: over {} readings, {:.2} MB/s moved and {:.2} MB/s read on average, window for window.",
                paired.len(),
                moved / 1e6,
                read / 1e6
            );
            a_figure!(
                missed,
                (read / moved - 1.0).abs() <= 0.15,
                "CAP AT ONCE: the speed beside the cap read {:.2} MB/s where {:.2} MB/s moved",
                read / 1e6,
                moved / 1e6
            );
        }
        a_figure!(
            missed,
            came_to.uploaded == files,
            "CAP AT ONCE: came to {came_to:?}"
        );
        a_figure!(
            missed,
            at_once >= 2,
            "CAP AT ONCE: only {at_once} file moved at once"
        );
        a_figure!(
            missed,
            took >= owed.mul_f64(0.85),
            "CAP AT ONCE: {:.1} s is faster than the cap allows ({:.1} s)",
            took.as_secs_f64(),
            owed.as_secs_f64()
        );
        a_figure!(
            missed,
            took <= owed.mul_f64(1.5) + Duration::from_secs(3),
            "CAP AT ONCE: {:.1} s, where the cap allows {:.1} s",
            took.as_secs_f64(),
            owed.as_secs_f64()
        );
        the_figures_held("CAP AT ONCE", missed);
    }

    /// **A manual download's second file waits for the one slot a low cap
    /// admits, and is not failed** — `Desktop app and delivery` R13 and R17,
    /// step 10c's eleventh-pass fix piece 4. At a cap below two floors the
    /// desktop point admits one channel; with `Download` Q1's two files of one
    /// request at once, the second is refused while the first moves, and a
    /// refusal is a reason to wait, never to end the file *did not finish*.
    #[tokio::test(flavor = "multi_thread", worker_threads = 4)]
    async fn a_manual_downloads_second_file_waits_for_the_one_slot_and_is_not_failed() {
        let _deadline = TheTestsDeadline::started();
        let program = the_server_or_say_so!();

        let (files, size) = (3usize, 8_000u64);
        let scratch = Scratch::new("one-slot-down");
        let on_the_machine = scratch.folder("machine/slow");
        for at in 0..files {
            a_large_file_at(&on_the_machine.join(format!("file_{at}.bin")), size);
        }
        let chosen = scratch.folder("chosen");
        let record = TheTimedRecord::at(&scratch.folder("record").join("smoozy.sqlite3"));
        let server_row = a_server_row_for(&record);
        let machine = Arc::new(AMachine {
            program,
            home: scratch.at.clone(),
            limit: 10,
            ..AMachine::default()
        });
        let point = APointLending::over(&machine).await;
        let the_run = Arc::new(TheRunsTransfers::for_this_run());
        the_run.a_cap_was_saved(Direction::Down, TheCap::Of(BytesPerSecond(20_000)));
        assert_eq!(
            the_run.the_admissible_count(Direction::Down),
            smoozy_desktop::scheduling::TheAdmissibleCount::Of(1),
            "the cap admits one channel"
        );
        let asked = WhatIsBeingDownloaded {
            server_row,
            installation: None,
            destination: the_bytes_of(&chosen),
            picked: vec![the_bytes_of(&on_the_machine)],
            run: a_run(),
        };
        let mut ready = a_download_is_prepared(
            asked,
            &point.sessions,
            record.as_ref() as &dyn TheRecordADownloadWrites,
            &NoJobIsKnown,
        )
        .await
        .expect("the download is prepared");
        the_request_goes_without_a_question(&mut ready, record.as_ref())
            .expect("the dispositions are written");
        let most = TheMostAtOnce::over(&the_run, ready.request.record.as_text());
        let came_to =
            the_download_runs(&mut ready, &point.sessions, &the_run, record.as_ref()).await;
        let at_once = most.ended();
        println!("ONE SLOT DOWN: {came_to:?}; {at_once} at once.");
        let mut missed = Vec::new();
        a_figure!(
            missed,
            came_to.fetched == files && came_to.did_not_finish == 0,
            "ONE SLOT DOWN: came to {came_to:?}"
        );
        a_figure!(
            missed,
            at_once == 1,
            "ONE SLOT DOWN: {at_once} moved at once where one is admitted"
        );
        for at in 0..files {
            a_figure!(
                missed,
                it_holds_the_pattern(&chosen.join("slow").join(format!("file_{at}.bin")), size),
                "ONE SLOT DOWN: file_{at}.bin did not land whole"
            );
        }
        the_figures_held("ONE SLOT DOWN", missed);
    }

    /// **A whole request's Stop never arranges a file still waiting for its
    /// turn** — `File Browser` U6, step 10c's eleventh-pass fix piece 4. At a
    /// cap that admits one channel, one file of the request moves and the
    /// channels holding the next three wait at the desktop point for the slot;
    /// Stop is pressed, and the files that then get the slot are not arranged:
    /// the one transfer record is the stopped file's, and nothing landed.
    #[tokio::test(flavor = "multi_thread", worker_threads = 4)]
    async fn stop_on_a_request_never_arranges_the_files_still_waiting() {
        let _deadline = TheTestsDeadline::started();
        let program = the_server_or_say_so!();

        let (files, size) = (6usize, 200_000u64);
        let scratch = Scratch::new("stop-waiting");
        let here = scratch.folder("here/six");
        for at in 0..files {
            a_large_file_at(&here.join(format!("file_{at}.bin")), size);
        }
        let machine_side = scratch.folder("machine");
        let record = TheTimedRecord::at(&scratch.folder("record").join("smoozy.sqlite3"));
        let (server_row, installation) = a_server_row_installed(&record);
        let machine = Arc::new(AMachine {
            program,
            home: scratch.at.clone(),
            limit: 10,
            ..AMachine::default()
        });
        let point = APointLending::over(&machine).await;
        let the_run = Arc::new(TheRunsTransfers::for_this_run());
        the_run.a_cap_was_saved(Direction::Up, TheCap::Of(BytesPerSecond(20_000)));
        let as_a_door: Arc<dyn TheRecordAnUploadWrites> = record.clone();
        let asked = WhatIsBeingUploaded {
            server_row,
            installation: Some(installation),
            destination: the_bytes_of(&machine_side),
            chosen: vec![here.clone()],
            run: a_run(),
        };
        let mut ready = an_upload_is_prepared(&asked, &point.sessions, as_a_door.as_ref())
            .await
            .expect("the upload is prepared");
        assert_eq!(
            the_answer_is_applied(&mut ready, None, &point.sessions, as_a_door.as_ref()).await,
            WhatTheAnswerLeft::ItGoes
        );
        let request = ready.request.record.as_text();
        let press = async {
            until("the first file moves", Duration::from_secs(60), || {
                the_run
                    .what_is_under_way()
                    .get(&request)
                    .is_some_and(|under_way| {
                        under_way
                            .moving
                            .iter()
                            .any(|file| file.report.bytes_staged > 0)
                    })
            })
            .await;
            assert!(the_run.the_user_stops(&request, None));
        };
        let (came_to, ()) = tokio::join!(
            the_upload_runs(
                ready,
                &point.sessions,
                &the_run,
                Arc::clone(&as_a_door),
                &NobodyIsTold
            ),
            press
        );
        let endings = the_endings(&record);
        println!(
            "STOP WAITING: came to {came_to:?}; the endings {endings:?}; staged {:?}.",
            what_is_left_staged(&machine_side.join("six"))
        );
        let mut missed = Vec::new();
        // **The file in flight is stopped, or reached its name in the moment of
        // the press** (`File Browser` U6: finished files stay); either way the
        // five behind it did not finish.
        a_figure!(
            missed,
            came_to.uploaded <= 1
                && came_to.uploaded + came_to.did_not_finish == files
                && came_to.failed.is_empty(),
            "STOP WAITING: came to {came_to:?}"
        );
        a_figure!(
            missed,
            endings.len() == 1,
            "STOP WAITING: {} transfers were arranged where one was moving: {endings:?}",
            endings.len()
        );
        a_figure!(
            missed,
            what_is_left_staged(&machine_side.join("six")).is_empty(),
            "STOP WAITING: something is left staged"
        );
        the_figures_held("STOP WAITING", missed);
    }

    // ------------------------------------------ one real connection, paced

    /// **Where OpenSSH's own server is**, for the paced real connection.
    const WHERE_SSHD_LIVES: &[&str] = &["/usr/sbin/sshd", "/usr/local/sbin/sshd"];

    /// **OpenSSH's own `sshd`, one connection at a time on a socket it is
    /// handed**, its keys made by OpenSSH's own `ssh-keygen` — the far end of
    /// `tests/a_line_cut_silently_under_a_real_connection.rs`. Added
    /// 2026-10-16, step 10c's owed piece 3.
    ///
    /// **Why this harness has a real connection at all.** Everything else in
    /// this file stands the machine in as one socket pair per session, so two
    /// sessions never share anything below the point. **On a rental they
    /// share one SSH connection and one TCP connection**, and the thirteenth
    /// pass measured two of Smoozy's channels draining a finished job at the
    /// pace of one `sftp` connection, where two `sftp` connections moved
    /// nearly twice as much. **What two channels share is only here.**
    struct TheRealFarEnd {
        sshd: PathBuf,
        config: PathBuf,
        client_key: PathBuf,
        /// **The keys and the configuration live outside the build folder**,
        /// whose path has a space in it that `sshd_config` would split.
        folder: PathBuf,
        /// **The throwaway home its sessions and commands start in**, where
        /// it was made with one (see [`TheRealFarEnd::at_one_session`]).
        home: Option<PathBuf>,
    }

    impl Drop for TheRealFarEnd {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.folder);
        }
    }

    impl TheRealFarEnd {
        /// **Nothing where this system has no `sshd` that runs for this user.**
        fn made(sftp: &Path) -> Option<TheRealFarEnd> {
            TheRealFarEnd::made_as("paced", sftp, false)
        }

        /// **A machine that permits one session per connection** — `Servers`
        /// S14's case, so every transfer rides the additional connection —
        /// **its sessions and commands in a throwaway home** (`sftp-server
        /// -d`, `SetEnv`), so nothing a carriage plants lands in the home of
        /// the person running the test. **Its `stat` answers in GNU's
        /// fields** where this system's speaks BSD's, as a rental's does.
        /// Added 2026-10-19, the dead-line sweep's extra-connection repair.
        fn at_one_session(sftp: &Path) -> Option<TheRealFarEnd> {
            TheRealFarEnd::made_as("one-session", sftp, true)
        }

        fn made_as(named: &str, sftp: &Path, at_one_session: bool) -> Option<TheRealFarEnd> {
            let sshd = WHERE_SSHD_LIVES
                .iter()
                .map(PathBuf::from)
                .find(|path| path.exists())?;
            let folder =
                std::env::temp_dir().join(format!("smzy-{named}-sshd-{}", std::process::id()));
            let _ = std::fs::remove_dir_all(&folder);
            std::fs::create_dir_all(&folder).ok()?;
            let home = at_one_session.then(|| folder.join("home"));
            let far_end = TheRealFarEnd {
                sshd,
                config: folder.join("sshd_config"),
                client_key: folder.join("client"),
                folder: folder.clone(),
                home: home.clone(),
            };
            let mut more = String::new();
            let mut subsystem = sftp.display().to_string();
            if let Some(home) = &home {
                use std::os::unix::fs::PermissionsExt;
                let bin = home.join("bin");
                std::fs::create_dir_all(&bin).ok()?;
                let stat = bin.join("stat");
                std::fs::write(
                    &stat,
                    "#!/bin/sh\n# GNU's `stat -L -c '%s %i %Y %Z' FILE`, in this system's own.\n\
                     if /usr/bin/stat --version >/dev/null 2>&1; then exec /usr/bin/stat \"$@\"; fi\n\
                     exec /usr/bin/stat -L -f '%z %i %m %c' \"$4\"\n",
                )
                .ok()?;
                std::fs::set_permissions(&stat, std::fs::Permissions::from_mode(0o755)).ok()?;
                subsystem = format!("{subsystem} -d {}", home.display());
                more = format!(
                    "MaxSessions 1\nSetEnv HOME={} PATH={}:/usr/bin:/bin\n",
                    home.display(),
                    bin.display()
                );
            }
            for key in ["host", "client"] {
                let made = std::process::Command::new("ssh-keygen")
                    .args(["-q", "-t", "ed25519", "-N", "", "-C", key, "-f"])
                    .arg(folder.join(key))
                    .status()
                    .ok()?;
                if !made.success() {
                    return None;
                }
            }
            std::fs::copy(folder.join("client.pub"), folder.join("authorized_keys")).ok()?;
            std::fs::write(
                &far_end.config,
                format!(
                    "HostKey {}\nAuthorizedKeysFile {}\nPidFile none\nUsePAM no\nStrictModes no\n\
                     PasswordAuthentication no\nKbdInteractiveAuthentication no\n\
                     Subsystem sftp {}\n{more}",
                    folder.join("host").display(),
                    folder.join("authorized_keys").display(),
                    subsystem
                ),
            )
            .ok()?;
            let checked = std::process::Command::new(&far_end.sshd)
                .args(["-t", "-f"])
                .arg(&far_end.config)
                .output()
                .ok()?;
            if !checked.status.success() {
                println!(
                    "sshd will not run here: {}",
                    String::from_utf8_lossy(&checked.stderr).trim()
                );
                return None;
            }
            Some(far_end)
        }

        /// One connection's server: `sshd -i` on a loopback TCP pair (macOS's
        /// own `sshd` refuses a socket with no address).
        fn one_connection(&self) -> (std::process::Child, std::net::TcpStream) {
            use std::os::unix::io::{AsRawFd, FromRawFd};
            let listener = std::net::TcpListener::bind(("127.0.0.1", 0)).expect("a listener");
            let ours = std::net::TcpStream::connect(listener.local_addr().expect("an address"))
                .expect("the pair connects");
            let (theirs, _) = listener.accept().expect("the pair is accepted");
            let reading = unsafe { std::process::Stdio::from_raw_fd(copy_of(theirs.as_raw_fd())) };
            let writing = unsafe { std::process::Stdio::from_raw_fd(copy_of(theirs.as_raw_fd())) };
            use std::os::unix::process::CommandExt;
            let child = std::process::Command::new(&self.sshd)
                .process_group(0)
                .args(["-i", "-e", "-f"])
                .arg(&self.config)
                .stdin(reading)
                .stdout(writing)
                .stderr(std::process::Stdio::null())
                .spawn()
                .expect("OpenSSH's own server starts");
            drop(theirs);
            (child, ours)
        }
    }

    /// **What the desk sent down the line, as the line received it**: when
    /// each read off the desk's socket arrived, and how many bytes it held.
    #[derive(Default)]
    struct WhatTheDeskSent {
        arrivals: Mutex<Vec<(Instant, usize)>>,
    }

    impl WhatTheDeskSent {
        /// **Flights**: arrivals grouped where they come less than two
        /// milliseconds apart — one burst of the desk's sends.
        fn flights_between(&self, from: Instant, to: Instant) -> usize {
            let arrivals = self.arrivals.lock().expect("the arrivals");
            let mut flights = 0usize;
            let mut last: Option<Instant> = None;
            for (at, _) in arrivals.iter().filter(|(at, _)| *at >= from && *at <= to) {
                if last
                    .is_none_or(|was| at.saturating_duration_since(was) >= Duration::from_millis(2))
                {
                    flights += 1;
                }
                last = Some(*at);
            }
            flights
        }
    }

    /// **A line with a round trip on it, carrying one real SSH connection**:
    /// every byte the desk sends reaches the machine `round_trip` later, and
    /// the machine's bytes come back at once — the kept harness's own delay
    /// line, put under the transport instead of under each session.
    struct ThePacedLine {
        address: std::net::SocketAddr,
        sent: Arc<WhatTheDeskSent>,
        servers: Arc<Mutex<Vec<std::process::Child>>>,
    }

    impl Drop for ThePacedLine {
        fn drop(&mut self) {
            if let Ok(mut servers) = self.servers.lock() {
                for server in servers.iter_mut() {
                    // The fixture's own process group includes its sessions.
                    unsafe { libc::kill(-(server.id() as i32), libc::SIGCONT); }
                    unsafe { libc::kill(-(server.id() as i32), libc::SIGKILL); }
                    let _ = server.kill();
                    let _ = server.wait();
                }
            }
        }
    }

    impl ThePacedLine {
        async fn to(far_end: &Arc<TheRealFarEnd>, round_trip: Duration) -> ThePacedLine {
            let listener = tokio::net::TcpListener::bind(("127.0.0.1", 0))
                .await
                .expect("the loopback address takes a listener");
            let address = listener.local_addr().expect("an address");
            let sent = Arc::new(WhatTheDeskSent::default());
            let servers = Arc::new(Mutex::new(Vec::new()));
            let far_end = Arc::clone(far_end);
            tokio::spawn({
                let sent = Arc::clone(&sent);
                let servers = Arc::clone(&servers);
                async move {
                    while let Ok((desk, _)) = listener.accept().await {
                        let (server, machine) = far_end.one_connection();
                        if let Ok(mut held) = servers.lock() {
                            held.push(server);
                        }
                        machine.set_nonblocking(true).expect("the machine's end");
                        let machine =
                            tokio::net::TcpStream::from_std(machine).expect("the machine's end");
                        // **The line's own two ends send at once**, so what
                        // the line adds is the round trip and nothing else.
                        let _ = machine.set_nodelay(true);
                        let _ = desk.set_nodelay(true);
                        tokio::spawn(the_line_carries(
                            desk,
                            machine,
                            round_trip,
                            Arc::clone(&sent),
                        ));
                    }
                }
            });
            ThePacedLine {
                address,
                sent,
                servers,
            }
        }
    }

    async fn the_line_carries(
        desk: tokio::net::TcpStream,
        machine: tokio::net::TcpStream,
        round_trip: Duration,
        sent: Arc<WhatTheDeskSent>,
    ) {
        use tokio::io::{AsyncReadExt, AsyncWriteExt};
        let (mut from_the_desk, mut to_the_desk) = desk.into_split();
        let (mut from_the_machine, mut to_the_machine) = machine.into_split();
        let (tell, mut told) =
            tokio::sync::mpsc::unbounded_channel::<(tokio::time::Instant, Vec<u8>)>();
        let towards = async move {
            let mut read = vec![0u8; 256 * 1024];
            loop {
                match from_the_desk.read(&mut read).await {
                    Ok(0) | Err(_) => break,
                    Ok(got) => {
                        if let Ok(mut arrivals) = sent.arrivals.lock() {
                            arrivals.push((Instant::now(), got));
                        }
                        let due = tokio::time::Instant::now() + round_trip;
                        if tell.send((due, read[..got].to_vec())).is_err() {
                            break;
                        }
                    }
                }
            }
        };
        let delivered = async move {
            while let Some((due, bytes)) = told.recv().await {
                tokio::time::sleep_until(due).await;
                if to_the_machine.write_all(&bytes).await.is_err() {
                    break;
                }
            }
            let _ = to_the_machine.shutdown().await;
        };
        let back = async move {
            let _ = tokio::io::copy(&mut from_the_machine, &mut to_the_desk).await;
            let _ = to_the_desk.shutdown().await;
        };
        tokio::join!(
            async {
                tokio::join!(towards, delivered);
            },
            back
        );
    }

    struct TheMachineIsTrusted;
    impl smoozy_server_protocol::transport::DecideHostTrust for TheMachineIsTrusted {
        fn decide(&mut self, _offered: &[u8]) -> smoozy_server_protocol::transport::HostTrust {
            smoozy_server_protocol::transport::HostTrust::Proceed
        }
    }

    /// **What one drain over the real connection came to.**
    #[derive(Debug)]
    struct WhatTheDrainCameTo {
        frames: usize,
        fetched: usize,
        seconds: f64,
        /// The desk's flights while the drain ran.
        flights: usize,
    }

    impl WhatTheDrainCameTo {
        fn frames_a_second(&self) -> f64 {
            self.fetched as f64 / self.seconds.max(f64::EPSILON)
        }
    }

    /// **A finished job's frames, drained over one real SSH connection with a
    /// round trip on its line** — the thirteenth pass's shape: a fresh
    /// connection to a job that has already ended, its ending walk, then
    /// `Download` E1 step 3 through the application's own drain and point.
    /// Nothing where this system has no `sshd` to run.
    async fn a_finished_jobs_frames_drained_over_a_real_connection(
        named: &str,
        frames: u32,
        each: usize,
        round_trip: Duration,
    ) -> Option<WhatTheDrainCameTo> {
        let sftp = WHERE_SFTP_SERVER_LIVES
            .iter()
            .map(PathBuf::from)
            .find(|path| path.exists())?;
        let Some(far_end) = TheRealFarEnd::made(&sftp) else {
            println!(
                "SKIPPED: no OpenSSH sshd and ssh-keygen this harness can run on this system ({}).",
                std::env::consts::OS
            );
            return None;
        };
        let far_end = Arc::new(far_end);
        let scratch = Scratch::new(named);
        let outputs = scratch.folder("machine/job/output");
        let chosen = scratch.folder("chosen");
        let mut frame_bytes = vec![0u8; each];
        for frame in 1..=frames {
            the_pattern_at(frame as u64 * each as u64, &mut frame_bytes);
            std::fs::write(outputs.join(format!("frame_{frame:05}.png")), &frame_bytes)
                .expect("the frame is written");
        }
        let line = ThePacedLine::to(&far_end, round_trip).await;
        let user = std::env::var("USER")
            .or_else(|_| std::env::var("LOGNAME"))
            .expect("the user running the test is named");
        let mut connection = smoozy_server_protocol::transport::ssh::open(
            &smoozy_server_protocol::transport::Endpoint {
                host: "127.0.0.1".to_string(),
                port: line.address.port(),
                username: user,
            },
            smoozy_server_protocol::transport::Credential {
                private_key_file: far_end.client_key.clone(),
                passphrase: None,
            },
            // **`Servers` K3's own figures.**
            smoozy_server_protocol::transport::Figures {
                interval: Duration::from_secs(15),
                unanswered_before_the_break: 3,
            },
            Box::new(TheMachineIsTrusted),
        )
        .await
        .expect("the connection opens through the paced line");
        let sessions = ThisServersSessions::<smoozy_server_protocol::transport::ssh::Connection>::for_one_server();
        let lending = tokio::spawn({
            let sessions = sessions.clone();
            async move {
                let none: TheAdditionalConnection<
                    smoozy_server_protocol::transport::ssh::Connection,
                > = Arc::new(|| Box::pin(async { None }));
                lend_until_it_ends(&sessions, &mut connection, &none).await
            }
        });
        while !sessions.a_connection_is_live() {
            tokio::task::yield_now().await;
        }

        let record = TheTimedRecord::at(&scratch.folder("record").join("smoozy.sqlite3"));
        let server_row = a_server_row_for(&record);
        let the_run = Arc::new(TheRunsTransfers::for_this_run());
        let run = a_run();
        let door: &dyn TheRecordADownloadWrites = record.as_ref();
        let mut arrangement = the_arrangement_is_made_at_submission(
            &mut TheArrangementsKeeper { record: door },
            true,
            WhatTheSubmissionChose {
                job: Identity::issue(IdentityKind::Job).expect("random bytes"),
                server_row,
                destination: the_bytes_of(&chosen),
                narrowing: TheNarrowing::Everything,
            },
        )
        .expect("the arrangement is made");
        let entries: Vec<ANarrowedEntry> = (1..=frames)
            .map(|frame| ANarrowedEntry {
                predicted_path: format!("frame_{frame:05}.png").into_bytes(),
                frame: Some(frame as i64),
            })
            .collect();
        let order = TheOrder::The(TheAttemptsOrder::as_the_attempt_record_carries_it(
            (1..=frames as i64).collect(),
        ));
        let folder = the_bytes_of(&outputs);
        let mut running = TheArrangementsRun::fresh();
        let mut queue = TheFetchQueue::at_a_server();
        let mut probing = TheProbesCadence::fresh();
        running.a_connection_began();
        running.the_ending_arrived(smoozy_queue_history::ending::Ending::Finished);
        let window = TheWindow {
            the_desired_state_is_on: true,
            a_connection_to_the_server_stands: true,
            the_attempts_window_is_open: true,
        };
        let in_hand = || WhatIsInHand {
            the_job_output_folder: &folder,
            narrowed_entries: &entries,
            the_whole_may_grow: false,
            the_entries_the_narrowing_excludes: &[],
            order: &order,
            the_jobs_label: None,
            the_first_frame_pair: None,
            the_pairs_other_half_fetched: &[],
            window,
            the_placed_copies: None,
            the_four_values_of_the_placed_copies: &[],
            arrangements_walking_this_server: 1,
        };
        one_turn_of_an_arrangement(
            &mut running,
            &sessions,
            door,
            &mut queue,
            &mut arrangement,
            in_hand(),
        )
        .await
        .expect("the ending walk ran");
        assert_eq!(
            queue.owed(),
            frames as usize,
            "every frame became fetchable"
        );

        let began = Instant::now();
        let mut owing = [AnArrangementOwingFetches {
            running: &mut running,
            queue: &mut queue,
            probing: &mut probing,
            arrangement: &mut arrangement,
            in_hand: in_hand(),
        }];
        let came = the_owed_fetches_come_down(
            &mut owing,
            &sessions,
            &the_run,
            door,
            &run,
            Duration::from_secs(3_600),
        )
        .await;
        let ended = Instant::now();
        let seconds = ended.duration_since(began).as_secs_f64();
        for frame in 1..=frames {
            let landed = std::fs::read_dir(&chosen)
                .expect("the destination")
                .filter_map(Result::ok)
                .map(|folder| folder.path().join(format!("frame_{frame:05}.png")))
                .find(|path| path.exists())
                .expect("the frame landed");
            assert!(
                it_holds_the_frame(&landed, frame as u64 * each as u64, each),
                "frame {frame} landed whole"
            );
        }
        lending.abort();
        Some(WhatTheDrainCameTo {
            frames: frames as usize,
            fetched: came.fetched,
            seconds,
            flights: line.sent.flights_between(began, ended),
        })
    }

    /// **The thirteenth pass's drain, at the desk, over one real connection
    /// with a round trip on its line** — step 10c's owed piece 3. The bar is
    /// the eleventh-pass map's: **1.85 frames a second at 0.09 s**, the
    /// `sftp` tool's own one-connection figure on that rental.
    ///
    /// Sized by `SMOOZY_HARNESS_PACED_FRAMES` (60), `SMOOZY_HARNESS_FRAME_BYTES`
    /// (700,000, the thirteenth pass's frames) and
    /// `SMOOZY_HARNESS_ROUND_TRIP_MS` (90).
    #[tokio::test(flavor = "multi_thread", worker_threads = 4)]
    #[ignore = "the kept harness: a minute or more; run on purpose"]
    async fn a_finished_jobs_frames_come_down_over_one_real_connection_at_a_round_trip() {
        let _deadline = TheTestsDeadline::started();
        let frames = a_number_from("SMOOZY_HARNESS_PACED_FRAMES", 60) as u32;
        let each = a_number_from("SMOOZY_HARNESS_FRAME_BYTES", 700_000) as usize;
        let round_trip = Duration::from_millis(a_number_from("SMOOZY_HARNESS_ROUND_TRIP_MS", 90));
        let Some(came) = a_finished_jobs_frames_drained_over_a_real_connection(
            "paced", frames, each, round_trip,
        )
        .await
        else {
            return;
        };
        println!(
            "PACED DRAIN: {} of {} frames of {each} bytes in {:.1} s at a {:.3} s round trip: {:.2} \
             frames a second ({:.3} s a frame); the desk sent in {} flights, {:.1} a second, {:.2} a \
             round trip",
            came.fetched,
            came.frames,
            came.seconds,
            round_trip.as_secs_f64(),
            came.frames_a_second(),
            came.seconds / came.fetched.max(1) as f64,
            came.flights,
            came.flights as f64 / came.seconds,
            came.flights as f64 / (came.seconds / round_trip.as_secs_f64()),
        );
        assert_eq!(came.fetched, came.frames, "every frame came down");
        // **The bar is read at its own round trip only**; at any other the
        // figure is printed and not judged.
        if round_trip == Duration::from_millis(90) {
            assert!(
                came.frames_a_second() >= 1.85,
                "PACED DRAIN: {:.2} frames a second at 0.09 s, under the bar of 1.85",
                came.frames_a_second()
            );
        }
    }

    // ------- the additional connection, dying alone (the dead-line sweep's extra-connection repair)

    /// **A line under each connection of its own**, between the desk and
    /// OpenSSH's own `sshd` — so one connection can go silent while another
    /// carries on. Added 2026-10-19, the dead-line sweep's extra-connection repair.
    ///
    /// **Silent, a connection's line carries nothing and closes nothing**, the
    /// way a provider's firewall or a lost route does: what the desk sends
    /// stays unread and nothing comes back. **The route itself can be down
    /// too**: a connection made while it is down is silent from its first byte,
    /// so opening one goes unanswered.
    struct ALineUnderEachConnection {
        address: std::net::SocketAddr,
        /// Whether a connection made now is carried.
        the_route: tokio::sync::watch::Sender<bool>,
        /// **Each connection's two halves** — towards the machine, and back —
        /// in the order the connections were made.
        each: Arc<Mutex<Vec<(tokio::sync::watch::Sender<bool>, tokio::sync::watch::Sender<bool>)>>>,
        /// **How fast a connection made now carries the desk's bytes**, in
        /// bytes a second; nought is as fast as it can.
        pace: Arc<std::sync::atomic::AtomicU64>,
        servers: Arc<Mutex<Vec<std::process::Child>>>,
    }

    impl Drop for ALineUnderEachConnection {
        fn drop(&mut self) {
            if let Ok(mut servers) = self.servers.lock() {
                for server in servers.iter_mut() {
                    // The fixture's own process group includes its sessions.
                    unsafe { libc::kill(-(server.id() as i32), libc::SIGCONT); }
                    unsafe { libc::kill(-(server.id() as i32), libc::SIGKILL); }
                    let _ = server.kill();
                    let _ = server.wait();
                }
            }
        }
    }

    impl ALineUnderEachConnection {
        async fn to(far_end: &Arc<TheRealFarEnd>) -> ALineUnderEachConnection {
            let listener = tokio::net::TcpListener::bind(("127.0.0.1", 0))
                .await
                .expect("the loopback address takes a listener");
            let address = listener.local_addr().expect("an address");
            let (the_route, route) = tokio::sync::watch::channel(true);
            let each = Arc::new(Mutex::new(Vec::new()));
            let pace = Arc::new(std::sync::atomic::AtomicU64::new(0));
            let servers = Arc::new(Mutex::new(Vec::new()));
            tokio::spawn({
                let far_end = Arc::clone(far_end);
                let each = Arc::clone(&each);
                let pace = Arc::clone(&pace);
                let servers = Arc::clone(&servers);
                async move {
                    while let Ok((desk, _)) = listener.accept().await {
                        let (server, machine) = far_end.one_connection();
                        if let Ok(mut held) = servers.lock() {
                            held.push(server);
                        }
                        machine.set_nonblocking(true).expect("the machine's end");
                        let machine =
                            tokio::net::TcpStream::from_std(machine).expect("the machine's end");
                        let carried = *route.borrow();
                        let (towards, _) = tokio::sync::watch::channel(carried);
                        let (back, _) = tokio::sync::watch::channel(carried);
                        let (desk_reads, desk_writes) = desk.into_split();
                        let (machine_reads, machine_writes) = machine.into_split();
                        let paced = pace.load(Ordering::SeqCst);
                        tokio::spawn(carried_while_it_is_up(desk_reads, machine_writes, towards.subscribe(), paced));
                        tokio::spawn(carried_while_it_is_up(machine_reads, desk_writes, back.subscribe(), 0));
                        if let Ok(mut held) = each.lock() {
                            held.push((towards, back));
                        }
                    }
                }
            });
            ALineUnderEachConnection {
                address,
                the_route,
                each,
                pace,
                servers,
            }
        }

        fn connections_made(&self) -> usize {
            self.each.lock().expect("the connections").len()
        }

        /// **One connection's line goes silent**: the desk's half first and
        /// the machine's a moment later, as on a real line, where what was
        /// already on its way back still arrives — so the desk is caught
        /// sending into a line that takes nothing.
        async fn goes_silent(&self, which: usize) {
            self.each.lock().expect("the connections")[which].0.send_replace(false);
            tokio::time::sleep(Duration::from_millis(300)).await;
            self.each.lock().expect("the connections")[which].1.send_replace(false);
        }

        /// **The route to the machine for a connection made from now on.**
        fn the_route_is_up(&self, up: bool) {
            self.the_route.send_replace(up);
        }

        /// **How fast a connection made from now on carries the desk's
        /// bytes**; nought is as fast as it can.
        fn carries_at(&self, bytes_a_second: u64) {
            self.pace.store(bytes_a_second, Ordering::SeqCst);
        }
    }

    async fn until_it_is(up: &mut tokio::sync::watch::Receiver<bool>, wanted: bool) {
        loop {
            if *up.borrow_and_update() == wanted {
                return;
            }
            if up.changed().await.is_err() {
                std::future::pending::<()>().await;
            }
        }
    }

    /// **One direction of one connection, carried while its half is up** —
    /// at `pace` bytes a second where it is not nought. A read is given up the
    /// moment the half goes silent, and a socket keeps what it has not handed
    /// over, so nothing is lost and nothing is read while it is silent.
    async fn carried_while_it_is_up<R, W>(
        mut from: R,
        mut to: W,
        mut up: tokio::sync::watch::Receiver<bool>,
        pace: u64,
    ) where
        R: tokio::io::AsyncRead + Unpin,
        W: tokio::io::AsyncWrite + Unpin,
    {
        use tokio::io::{AsyncReadExt, AsyncWriteExt};
        let mut buffer = vec![0u8; 64 * 1024];
        loop {
            until_it_is(&mut up, true).await;
            let read = tokio::select! {
                read = from.read(&mut buffer) => read,
                () = until_it_is(&mut up, false) => continue,
            };
            let got = match read {
                Ok(0) | Err(_) => break,
                Ok(got) => got,
            };
            until_it_is(&mut up, true).await;
            if to.write_all(&buffer[..got]).await.is_err() {
                break;
            }
            if pace > 0 {
                tokio::time::sleep(Duration::from_secs_f64(got as f64 / pace as f64)).await;
            }
        }
        let _ = to.shutdown().await;
    }

    /// **The desk's figures**, not `Servers` K3's 15 seconds and 3, which would
    /// make each silence a minute: **the declaration window at these is 3 to 4
    /// seconds** (`Figures::the_declaration_window`).
    const K3_AT_THE_DESK: smoozy_server_protocol::transport::Figures =
        smoozy_server_protocol::transport::Figures {
            interval: Duration::from_secs(1),
            unanswered_before_the_break: 3,
        };

    fn the_keepalive_figures_for_this_test() -> smoozy_server_protocol::transport::Figures {
        if std::env::var_os("SMOOZY_HARNESS_LIVE_TRANSFER_TIMES").is_some() {
            smoozy_server_protocol::transport::Figures {
                interval: Duration::from_secs(15),
                unanswered_before_the_break: 3,
            }
        } else {
            K3_AT_THE_DESK
        }
    }

    /// **One SSH connection through the line**, or nothing where it does not
    /// open within three seconds — `Servers`' own opening bound, shortened.
    async fn a_connection_through(
        port: u16,
        key: PathBuf,
    ) -> Option<smoozy_server_protocol::transport::ssh::Connection> {
        let user = std::env::var("USER")
            .or_else(|_| std::env::var("LOGNAME"))
            .expect("the user running the test is named");
        tokio::time::timeout(
            if std::env::var_os("SMOOZY_HARNESS_LIVE_TRANSFER_TIMES").is_some() {
                Duration::from_secs(20)
            } else {
                Duration::from_secs(3)
            },
            smoozy_server_protocol::transport::ssh::open(
                &smoozy_server_protocol::transport::Endpoint {
                    host: "127.0.0.1".to_string(),
                    port,
                    username: user,
                },
                smoozy_server_protocol::transport::Credential {
                    private_key_file: key,
                    passphrase: None,
                },
                the_keepalive_figures_for_this_test(),
                Box::new(TheMachineIsTrusted),
            ),
        )
        .await
        .ok()?
        .ok()
    }

    /// **When each additional connection's transport reported it ended, and
    /// how.**
    type TheDeclarations = Arc<Mutex<Vec<(Instant, Ending)>>>;

    /// **`Servers` S14's additional connection, opened through the line** —
    /// `Servers` Q16's three steps, standing in — **and its transport's own
    /// report of its ending kept**, so a test can say when the break was
    /// declared.
    fn the_additional_through(
        line: &ALineUnderEachConnection,
        far_end: &TheRealFarEnd,
        declared: &TheDeclarations,
    ) -> TheAdditionalConnection<smoozy_server_protocol::transport::ssh::Connection> {
        let (port, key, declared) = (line.address.port(), far_end.client_key.clone(), Arc::clone(declared));
        Arc::new(move || {
            let (key, declared) = (key.clone(), Arc::clone(&declared));
            Box::pin(async move {
                let opened = a_connection_through(port, key).await?;
                let its_ending = opened.its_ending();
                tokio::spawn(async move {
                    let ending = its_ending.await;
                    if let Ok(mut held) = declared.lock() {
                        held.push((Instant::now(), ending));
                    }
                });
                Some(opened)
            })
        })
    }

    /// **The first keepalive declaration after `since`.**
    fn declared_after(declared: &TheDeclarations, since: Instant) -> Option<Instant> {
        declared
            .lock()
            .expect("the declarations")
            .iter()
            .find(|(at, ending)| *at > since && matches!(ending, Ending::TheKeepaliveWentUnanswered))
            .map(|(at, _)| *at)
    }

    /// **The pieces of the desk every lone cut is watched against.**
    struct TheDesk<'a> {
        line: &'a ALineUnderEachConnection,
        sessions: &'a ThisServersSessions<smoozy_server_protocol::transport::ssh::Connection>,
        lending: &'a tokio::task::JoinHandle<Ending>,
        declared: &'a TheDeclarations,
        /// The machine's home, which the primary reads while the additional
        /// connection is down.
        home: &'a Path,
    }

    /// **What a lone cut came to.**
    struct WhatTheLoneCutCameTo {
        /// From the silence to its transport's keepalive declaration.
        silent_to_declared: Duration,
        /// **From the declaration to the transfer's wait ending** — to the
        /// request reading *waiting for the connection*, or a material write
        /// answering.
        declared_to_ended: Duration,
        /// Whether anything said the transfer had ended before the
        /// declaration.
        ended_before_it: bool,
        /// **What the primary did meanwhile**: still lending, and a folder
        /// read on it while the additional connection was down.
        the_primary_stood: bool,
        the_primary_read: bool,
    }

    /// **`Servers` S14's additional connection goes silent alone, and its
    /// transport declares it gone; the route for a fresh one stays down past
    /// one attempt at it, then comes back.** `ended` says when the transfer's
    /// wait has ended.
    async fn the_additional_dies_alone(
        said: &str,
        desk: &TheDesk<'_>,
        mut ended: impl FnMut() -> bool,
        freeze_and_ask_for_another_seat: bool,
    ) -> (WhatTheLoneCutCameTo, Instant) {
        let which = desk.line.connections_made() - 1;
        assert!(which >= 1, "{said}: the transfer rides no additional connection");
        desk.line.the_route_is_up(false);
        let silent = Instant::now();
        if freeze_and_ask_for_another_seat {
            let servers = desk.line.servers.lock().expect("the fixture's servers");
            let id = servers[which].id() as i32;
            assert_eq!(unsafe { libc::kill(-id, libc::SIGSTOP) }, 0, "the fixture's extra SSH group could not be frozen");
        } else {
            desk.line.goes_silent(which).await;
        }
        // A second transfer asks while the extra connection is silent, before
        // its keepalive declares it gone. Its session opening gets no reply.
        // The declaration must release this wait too (`Servers` S14 condition 6),
        // so it cannot keep the download or the primary's readings waiting.
        let another_seat = freeze_and_ask_for_another_seat.then(|| {
            let sessions = desk.sessions.clone();
            tokio::spawn(async move { sessions.a_transfer_channel().await })
        });
        let mut ended_before_it = false;
        let declared_at = loop {
            if let Some(at) = declared_after(desk.declared, silent) {
                break at;
            }
            ended_before_it |= ended();
            assert!(
                silent.elapsed() < Duration::from_secs(90),
                "{said}: the additional connection's break was never declared"
            );
            tokio::time::sleep(Duration::from_millis(2)).await;
        };
        until(&format!("{said}: the transfer's wait ends"), Duration::from_secs(10), &mut ended).await;
        let declared_to_ended = declared_at.elapsed();
        // **The primary, while the additional connection is down.**
        let read = tokio::time::timeout(
            Duration::from_secs(8),
            a_folder_is_read(desk.sessions, desk.home.to_string_lossy().as_bytes()),
        ).await;
        let the_primary_read = matches!(read, Ok(WhatTheReadingFound::TheFolder { .. }));
        assert!(the_primary_read, "{said}: a session opening on the silent extra connection held the healthy primary");
        if let Some(another_seat) = another_seat {
            let refused = tokio::time::timeout(Duration::from_secs(30), another_seat)
                .await.expect("the pending extra session opening never ended")
                .expect("the pending seat task panicked");
            assert!(refused.is_err(), "a new seat was offered while its route was down");
        }
        // **The route stays down past one attempt at a fresh connection.**
        tokio::time::sleep(Duration::from_secs(7)).await;
        let the_primary_stood = desk.sessions.a_connection_is_live() && !desk.lending.is_finished();
        let back = Instant::now();
        desk.line.the_route_is_up(true);
        if freeze_and_ask_for_another_seat {
            let servers = desk.line.servers.lock().expect("the fixture's servers");
            let id = servers[which].id() as i32;
            assert_eq!(unsafe { libc::kill(-id, libc::SIGCONT) }, 0, "the fixture's extra SSH group could not be resumed");
        }
        (
            WhatTheLoneCutCameTo {
                silent_to_declared: declared_at.saturating_duration_since(silent),
                declared_to_ended,
                ended_before_it,
                the_primary_stood,
                the_primary_read,
            },
            back,
        )
    }

    /// **The figures a lone cut is held to.**
    fn the_lone_cut_held(said: &str, cut: &WhatTheLoneCutCameTo) -> Vec<String> {
        let mut missed = Vec::new();
        let (opens, closes) = the_keepalive_figures_for_this_test().the_declaration_window();
        println!(
            "{said}: declared {:.2} s after the silence (window {:.0} to {:.0} s); the wait ended \
             {:.2} s after the declaration; the primary stood {}, read {}.",
            cut.silent_to_declared.as_secs_f64(),
            opens.as_secs_f64(),
            closes.as_secs_f64(),
            cut.declared_to_ended.as_secs_f64(),
            cut.the_primary_stood,
            cut.the_primary_read
        );
        // **When the transport declares is `Servers` K3's and not this
        // test's**; that it declares within seconds is what this test needs.
        // An upload's cut is declared up to about three seconds past the
        // window at this desk (the account of this repair says more).
        a_figure!(
            missed,
            cut.silent_to_declared <= closes + Duration::from_secs(6),
            "{said}: the additional connection's break was declared {:.1} s after the silence",
            cut.silent_to_declared.as_secs_f64()
        );
        a_figure!(missed, !cut.ended_before_it, "{said}: the transfer's wait ended before the declaration");
        // **`Server protocol and transfers` C18: the declaration ends the
        // wait** — the transfer's channel learns its connection has ended.
        a_figure!(
            missed,
            cut.declared_to_ended <= Duration::from_millis(1_500),
            "{said}: the transfer's wait ended {:.1} s after its connection was declared gone",
            cut.declared_to_ended.as_secs_f64()
        );
        a_figure!(missed, cut.the_primary_stood, "{said}: the primary connection did not stand");
        a_figure!(missed, cut.the_primary_read, "{said}: the primary read no folder while the additional was down");
        missed
    }

    /// **An upload, a download and a large submission-material write, each on
    /// `Servers` S14's additional connection, end at that connection's own
    /// declaration when it alone goes silent — the primary answering
    /// throughout — and each carries on once the route is back** — the
    /// dead-line sweep's extra-connection repair, 2026-10-19 (`Audits/Build/Step 10c/A dead
    /// line under every server act — the sweep.md`, finding 1, and its desk
    /// proof): `Servers` S14 condition 6, `Server protocol and transfers` C11
    /// step 4, `Server protocol and transfers` C18, `Server protocol and
    /// transfers` S12, `File Browser` K1–K4, `File Browser` K10 and `Download`
    /// M11.
    ///
    /// **The machine is OpenSSH's own `sshd` at one session per connection**,
    /// so the primary holds the reading session and every transfer rides the
    /// additional connection; **each connection has a line of its own**.
    ///
    /// **The case it bites on.** Until the repair the additional connection's
    /// ending was never heard while the primary stood: the point let a
    /// connection go only when its own lending ended or the connection fell
    /// idle, so a write waiting for room on the dead additional connection
    /// waited for good — the upload's, the download's and the material's
    /// alike. **And a transfer the additional connection's death did end was
    /// not carried on**: `crate::uploading::carried_across_repairs` asked only
    /// whether the primary had gone, found it standing, and ended the file.
    #[tokio::test(flavor = "multi_thread", worker_threads = 4)]
    async fn transfers_on_the_additional_connection_end_at_its_own_declaration_and_carry_on() {
        use smoozy_job_files::acting::TheMachinesDirectories;
        let _deadline = TheTestsDeadline::started();
        let sftp = the_server_or_say_so!();

        let Some(far_end) = TheRealFarEnd::at_one_session(&sftp) else {
            println!(
                "SKIPPED: no OpenSSH sshd and ssh-keygen this harness can run on this system ({}).",
                std::env::consts::OS
            );
            return;
        };
        let far_end = Arc::new(far_end);
        let home = far_end.home.clone().expect("a far end at one session has a home");
        let line = ALineUnderEachConnection::to(&far_end).await;
        let mut primary = a_connection_through(line.address.port(), far_end.client_key.clone())
            .await
            .expect("the primary connection opens through the line");
        let sessions = ThisServersSessions::<smoozy_server_protocol::transport::ssh::Connection>::for_one_server();
        let declared: TheDeclarations = Arc::new(Mutex::new(Vec::new()));
        let additional = the_additional_through(&line, &far_end, &declared);
        let lending = tokio::spawn({
            let sessions = sessions.clone();
            async move { lend_until_it_ends(&sessions, &mut primary, &additional).await }
        });
        while !sessions.a_connection_is_live() {
            tokio::task::yield_now().await;
        }
        // **The reading session is opened first**, as the server's page
        // opens it, so the point learns the machine's one session and seats
        // every transfer on the additional connection.
        let first = a_folder_is_read(&sessions, home.to_string_lossy().as_bytes()).await;
        assert!(matches!(first, WhatTheReadingFound::TheFolder { .. }), "the first reading reads: {first:?}");
        let desk = TheDesk {
            line: &line,
            sessions: &sessions,
            lending: &lending,
            declared: &declared,
            home: &home,
        };

        let size = 48_000_000u64;
        let scratch = Scratch::new("additional-alone");
        let here = scratch.folder("here");
        let machine_side = scratch.folder("machine");
        let chosen = scratch.folder("chosen");
        let source = here.join("alone.blend");
        a_large_file_at(&source, size);
        let record = TheTimedRecord::at(&scratch.folder("record").join("smoozy.sqlite3"));
        let server_row = a_server_row_for(&record);
        let the_run = Arc::new(TheRunsTransfers::for_this_run());
        let capped = TheCap::Of(BytesPerSecond(8_000_000));
        the_run.the_caps_saved_are(capped, capped);
        let within = Duration::from_secs(180);
        // **A transfer's wait has ended** once its request reads *waiting for
        // the connection* — or is no longer under way at all.
        let waits = |the_run: &TheRunsTransfers, request: &str| {
            the_run
                .what_is_under_way()
                .get(request)
                .is_none_or(|under_way| under_way.waits_for_the_connection)
        };
        let staged = |the_run: &TheRunsTransfers, request: &str| {
            the_run
                .what_is_under_way()
                .get(request)
                .and_then(|under_way| under_way.moving.first().map(|file| file.report.bytes_staged))
                .unwrap_or(0)
        };
        let mut missed = Vec::new();

        // ---- up
        let asked = WhatIsBeingUploaded {
            server_row: server_row.clone(),
            installation: Some(Identity::issue(IdentityKind::Installation).expect("random")),
            destination: the_bytes_of(&machine_side),
            chosen: vec![source.clone()],
            run: a_run(),
        };
        let as_a_door: Arc<dyn TheRecordAnUploadWrites> = record.clone();
        let mut ready = an_upload_is_prepared(&asked, &sessions, as_a_door.as_ref())
            .await
            .expect("the upload is prepared");
        assert_eq!(
            the_answer_is_applied(&mut ready, None, &sessions, as_a_door.as_ref()).await,
            WhatTheAnswerLeft::ItGoes
        );
        let request = ready.request.record.as_text();
        let watched = TheFileWatched::over(&the_run, request.clone());
        let cutting = async {
            until("ALONE UP: half the file staged", Duration::from_secs(120), || {
                staged(&the_run, &request) >= size / 2
            })
            .await;
            let at_the_cut = staged(&the_run, &request);
            let (cut, back_for_good) =
                the_additional_dies_alone("ALONE UP", &desk, || waits(&the_run, &request), false).await;
            let while_down = the_row_while_down(&record, &the_run, &request);
            (cut, WhatTheCutsCameTo { at_the_cut, while_down, back_for_good })
        };
        let (came_to, (cut, cuts)) = tokio::join!(
            tokio::time::timeout(
                within,
                the_upload_runs(ready, &sessions, &the_run, Arc::clone(&as_a_door), &NobodyIsTold),
            ),
            cutting
        );
        let looks = watched.ended();
        let came_to = came_to.expect("ALONE UP: the upload came to an end inside its bound");
        let landed = machine_side.join("alone.blend");
        missed.extend(the_lone_cut_held("ALONE UP", &cut));
        missed.extend(the_carry_on_held("ALONE UP", &cuts, &looks, false));
        a_figure!(missed, came_to.uploaded == 1, "ALONE UP: the upload came to {came_to:?}");
        a_figure!(
            missed,
            it_holds_the_pattern(&landed, size),
            "ALONE UP: the file on the machine is not the file that was sent"
        );
        let left = what_is_left_staged(&machine_side);
        a_figure!(missed, left.is_empty(), "ALONE UP: left under .smoozy-staging: {left:?}");

        // ---- down
        let asked = WhatIsBeingDownloaded {
            server_row,
            installation: None,
            destination: the_bytes_of(&chosen),
            picked: vec![the_bytes_of(&landed)],
            run: a_run(),
        };
        let mut ready = a_download_is_prepared(
            asked,
            &sessions,
            record.as_ref() as &dyn TheRecordADownloadWrites,
            &NoJobIsKnown,
        )
        .await
        .expect("the download is prepared");
        the_request_goes_without_a_question(&mut ready, record.as_ref())
            .expect("the dispositions are written");
        let request = ready.request.record.as_text();
        let watched = TheFileWatched::over(&the_run, request.clone());
        let cutting = async {
            until("ALONE DOWN: half the file staged", Duration::from_secs(120), || {
                staged(&the_run, &request) >= size / 2
            })
            .await;
            let at_the_cut = staged(&the_run, &request);
            let (cut, back_for_good) =
                the_additional_dies_alone("ALONE DOWN", &desk, || waits(&the_run, &request), true).await;
            let while_down = the_row_while_down(&record, &the_run, &request);
            (cut, WhatTheCutsCameTo { at_the_cut, while_down, back_for_good })
        };
        let (came_to, (cut, cuts)) = tokio::join!(
            tokio::time::timeout(
                within,
                the_download_runs(&mut ready, &sessions, &the_run, record.as_ref()),
            ),
            cutting
        );
        let looks = watched.ended();
        let came_to = came_to.expect("ALONE DOWN: the download came to an end inside its bound");
        missed.extend(the_lone_cut_held("ALONE DOWN", &cut));
        // **`Server protocol and transfers` S13's four values are read through
        // the primary and agree**, so the download carries on from its last
        // checkpoint rather than starting over.
        missed.extend(the_carry_on_held("ALONE DOWN", &cuts, &looks, false));
        a_figure!(missed, came_to.fetched == 1, "ALONE DOWN: the download came to {came_to:?}");
        a_figure!(
            missed,
            it_holds_the_pattern(&chosen.join("alone.blend"), size),
            "ALONE DOWN: the file on this computer is not the file on the machine"
        );
        let left = what_is_left_staged(&chosen);
        a_figure!(missed, left.is_empty(), "ALONE DOWN: left under .smoozy-staging: {left:?}");

        // ---- a large submission-material write
        //
        // **The material's own write** — `Job files and outputs` M8's
        // durable whole-file write, the one `crate::submitting`'s material
        // placement makes on its seat — **is not timed by the byte loop**, so
        // until the repair nothing ended it on a dead additional connection.
        // A material write cut is not resumed: the submission ends *Not
        // submitted.* and is sent again, so what carries on is the next write,
        // whole, on a fresh additional connection.
        //
        // **Four megabytes, on a line carrying a megabyte a second**, so the
        // write is caught part-way with its room used up. **Not larger**: that
        // write hands every piece over before it reads one answer, and above
        // about six megabytes at this desk the connection under it stalls on
        // a healthy line (the account of this repair says more).
        let material: Vec<u8> = {
            let mut bytes = vec![0u8; a_number_from("SMOOZY_HARNESS_MATERIAL_BYTES", 4_000_000) as usize];
            the_pattern_at(0, &mut bytes);
            bytes
        };
        line.carries_at(1_000_000);
        let channel = sessions.a_transfer_channel().await.expect("a seat for the material");
        assert!(channel.is_on_the_additional_connection(), "the material's seat is on the primary");
        let mut subsystem = smoozy_server_protocol::subsystem::client::Subsystem::negotiate(channel)
            .await
            .expect("the material's session negotiates");
        let first = machine_side.join("material-cut.bin");
        let writing = tokio::spawn({
            let (path, bytes) = (first.to_string_lossy().into_owned(), material.clone());
            async move {
                let wrote = subsystem.write_a_whole_file_durably(&path, &bytes).await;
                (wrote.is_ok(), Instant::now())
            }
        });
        until("MATERIAL: the write is under way", Duration::from_secs(30), || {
            std::fs::metadata(&first).map(|held| held.len()).unwrap_or(0) >= 1 << 20
        })
        .await;
        line.carries_at(0);
        let (cut, back_for_good) =
            the_additional_dies_alone("MATERIAL", &desk, || writing.is_finished(), false).await;
        missed.extend(the_lone_cut_held("MATERIAL", &cut));
        let (wrote, _) = writing.await.expect("the material's write did not panic");
        a_figure!(missed, !wrote, "MATERIAL: the cut write said it was written whole");
        let channel = tokio::time::timeout(Duration::from_secs(60), sessions.a_transfer_channel())
            .await
            .expect("MATERIAL: the next seat waited for good")
            .expect("MATERIAL: the next seat was refused");
        a_figure!(
            missed,
            channel.is_on_the_additional_connection() && !channel.the_connection_it_sits_on().has_ended(),
            "MATERIAL: the next seat is not on a fresh additional connection"
        );
        let mut subsystem = smoozy_server_protocol::subsystem::client::Subsystem::negotiate(channel)
            .await
            .expect("the material's next session negotiates");
        let again = machine_side.join("material.bin");
        let wrote = subsystem
            .write_a_whole_file_durably(&again.to_string_lossy(), &material)
            .await;
        a_figure!(missed, wrote.is_ok(), "MATERIAL: the next write came to {wrote:?}");
        a_figure!(
            missed,
            it_holds_the_pattern(&again, material.len() as u64),
            "MATERIAL: the next write did not land whole"
        );
        println!(
            "MATERIAL: the next write landed {:.1} s after the route came back.",
            back_for_good.elapsed().as_secs_f64()
        );
        drop(subsystem);
        a_figure!(
            missed,
            sessions.a_connection_is_live() && !lending.is_finished(),
            "the primary connection did not stand to the end"
        );
        lending.abort();
        the_figures_held("THE ADDITIONAL CONNECTION, DYING ALONE", missed);
    }

    /// **What a person looking at Transfers reads for the request now**:
    /// whether it reads under way, its control, and its file row's state and
    /// tail.
    fn the_row_while_down(
        record: &TheTimedRecord,
        the_run: &TheRunsTransfers,
        request: &str,
    ) -> (bool, Option<String>, String, Option<String>) {
        let screen = the_screen_now(record, the_run);
        let row = the_row(&screen, request).expect("the request is on the screen");
        (
            !as_read(&row.name).contains("did not finish"),
            row.control.as_ref().map(|control| control.act.as_str().to_string()),
            row.files.first().map(|file| file.state.to_string()).unwrap_or_default(),
            row.files
                .first()
                .and_then(|file| file.tail.as_ref().map(|tail| tail.as_str().to_string())),
        )
    }

    // ------------------------------------------------------------- the deadline

    const WHAT_THE_TESTS_HERE_WAIT_ON: &str =
        "OpenSSH's own sftp-server over a socket pair, and the record's lock";

    /// **A run that hangs is ended rather than left**, after
    /// `SMOOZY_HARNESS_DEADLINE_SECONDS` (an hour when unset): it says which
    /// test and ends every program this binary started, and the binary.
    ///
    /// **And the harness's tests run one at a time**, whatever the test
    /// runner's own threads — added 2026-09-29, step 10c's transfer wave piece
    /// 7. Each holds its own caps and times its own figures, and nine of them
    /// run at once had one of piece 6's ten-second bounds missed on a busy
    /// desk (`a_file_the_line_cuts_carries_on_across_a_repair_up_and_down`,
    /// which passed alone three times out of three). **The deadline starts once
    /// the test has its turn.**
    struct TheTestsDeadline {
        _held: std::sync::mpsc::Sender<()>,
        _its_turn: std::sync::MutexGuard<'static, ()>,
    }

    static ONE_AT_A_TIME: Mutex<()> = Mutex::new(());

    impl TheTestsDeadline {
        fn started() -> TheTestsDeadline {
            let its_turn = ONE_AT_A_TIME
                .lock()
                .unwrap_or_else(|poisoned| poisoned.into_inner());
            let bound = Duration::from_secs(a_number_from("SMOOZY_HARNESS_DEADLINE_SECONDS", 3_600));
            let (held, watched) = std::sync::mpsc::channel::<()>();
            let test = std::thread::current()
                .name()
                .unwrap_or("a test in this file")
                .to_string();
            std::thread::spawn(move || {
                if watched.recv_timeout(bound) == Err(std::sync::mpsc::RecvTimeoutError::Timeout) {
                    let said = format!(
                        "DEADLINE MISSED: {test} was still running after {bound:?}, on {}. Every \
                         program this test binary started is ended now, and so is the binary.\n",
                        WHAT_THE_TESTS_HERE_WAIT_ON
                    );
                    let _ = std::io::Write::write_all(&mut std::io::stderr(), said.as_bytes());
                    let _ = std::process::Command::new("pkill")
                        .args(["-KILL", "-P", &std::process::id().to_string()])
                        .status();
                    std::process::exit(101);
                }
            });
            TheTestsDeadline {
                _held: held,
                _its_turn: its_turn,
            }
        }
    }
    /// Sitting B's download, close to its finish, with only its extra SSH
    /// process group frozen. The primary stays alive and a second transfer
    /// asks for a channel during the freeze. Large and slow only when asked.
    #[tokio::test(flavor = "multi_thread", worker_threads = 4)]
    #[ignore = "the live-size reproduction: 512 MiB and several minutes; run on purpose"]
    async fn sitting_b_a_large_download_carries_on_after_only_its_extra_server_is_frozen() {
        let _deadline = TheTestsDeadline::started();
        let sftp = the_server_or_say_so!();

        let far_end = Arc::new(TheRealFarEnd::at_one_session(&sftp).expect("the desk has SSH"));
        let home = far_end.home.clone().expect("the throwaway home");
        let changing_limit = std::env::var_os("SMOOZY_HARNESS_TRANSFER_LIMIT_CHANGE").is_some();
        let config = std::fs::read_to_string(&far_end.config).expect("the fixture's configuration");
        if changing_limit {
            std::fs::write(&far_end.config, config.replace("MaxSessions 1", "MaxSessions 2"))
                .expect("the primary's two-session configuration");
        }
        let line = ALineUnderEachConnection::to(&far_end).await;
        let mut primary = a_connection_through(line.address.port(), far_end.client_key.clone())
            .await.expect("the primary opens");
        let sessions = ThisServersSessions::<smoozy_server_protocol::transport::ssh::Connection>::for_one_server();
        let declared: TheDeclarations = Arc::new(Mutex::new(Vec::new()));
        let additional = the_additional_through(&line, &far_end, &declared);
        let lending = tokio::spawn({
            let sessions = sessions.clone();
            async move { lend_until_it_ends(&sessions, &mut primary, &additional).await }
        });
        until("the point lends", Duration::from_secs(5), || sessions.a_connection_is_live()).await;
        assert!(matches!(a_folder_is_read(&sessions, home.to_string_lossy().as_bytes()).await,
            WhatTheReadingFound::TheFolder { .. }));
        // The listener's new limit applies to new connections. Keep another
        // transfer's session on the older primary while the extra opens at one.
        let primary_seat = if changing_limit {
            let seat = sessions.a_transfer_channel().await.expect("the older primary's spare session");
            assert!(!seat.is_on_the_additional_connection());
            std::fs::write(&far_end.config, &config).expect("the extra's one-session configuration");
            Some(seat)
        } else {
            None
        };
        let desk = TheDesk { line: &line, sessions: &sessions, lending: &lending, declared: &declared, home: &home };
        let scratch = Scratch::new("sitting-b-download");
        let machine_side = scratch.folder("machine");
        let chosen = scratch.folder("chosen");
        let source = machine_side.join("sitting-b.bin");
        let size = a_number_from("SMOOZY_HARNESS_SITTING_B_BYTES", 536_870_912);
        let cut_at = if size == 536_870_912 { 477_000_000 } else { size / 2 };
        a_large_file_at(&source, size);
        let record = TheTimedRecord::at(&scratch.folder("record").join("smoozy.sqlite3"));
        let server_row = a_server_row_for(&record);
        let the_run = Arc::new(TheRunsTransfers::for_this_run());
        let capped = TheCap::Of(BytesPerSecond(4_000_000));
        the_run.the_caps_saved_are(capped, capped);
        let mut ready = a_download_is_prepared(WhatIsBeingDownloaded {
            server_row, installation: None, destination: the_bytes_of(&chosen),
            picked: vec![the_bytes_of(&source)], run: a_run(),
        }, &sessions, record.as_ref(), &NoJobIsKnown).await.expect("the download is prepared");
        the_request_goes_without_a_question(&mut ready, record.as_ref()).expect("the answer is written");
        let request = ready.request.record.as_text();
        let watched = TheFileWatched::over(&the_run, request.clone());
        let cutting = async {
            until("SITTING B: the live pass's cut point", Duration::from_secs(200), || {
                the_run.what_is_under_way().get(&request)
                    .and_then(|row| row.moving.first().map(|file| file.report.bytes_staged))
                    .is_some_and(|bytes| bytes >= cut_at)
            }).await;
            let at_the_cut = the_run.what_is_under_way()[&request].moving[0].report.bytes_staged;
            let (cut, back_for_good) = the_additional_dies_alone("SITTING B", &desk, || {
                the_run.what_is_under_way().get(&request)
                    .is_none_or(|row| row.waits_for_the_connection)
            }, true).await;
            let while_down = the_row_while_down(&record, &the_run, &request);
            (cut, WhatTheCutsCameTo { at_the_cut, while_down, back_for_good })
        };
        let (came, (cut, cuts)) = tokio::join!(
            tokio::time::timeout(Duration::from_secs(360),
                the_download_runs(&mut ready, &sessions, &the_run, record.as_ref())), cutting);
        let looks = watched.ended();
        let came = came.expect("the live-size download never finished");
        let mut missed = the_lone_cut_held("SITTING B", &cut);
        missed.extend(the_carry_on_held("SITTING B", &cuts, &looks, false));
        a_figure!(missed, came.fetched == 1, "SITTING B: {came:?}");
        a_figure!(missed, it_holds_the_pattern(&chosen.join("sitting-b.bin"), size), "SITTING B: the contents differ");
        a_figure!(missed, !the_run.what_is_under_way().contains_key(&request), "SITTING B: an ended download is still drawn as moving");
        drop(primary_seat);
        lending.abort();
        let _ = lending.await;
        the_figures_held("SITTING B", missed);
    }

}
