//! **The update check** — E10a, E11, Z4, Z22, Z23, Z23a and Z26 to Z30, with
//! P10's one standing timer. Built at step 11's check piece. **It reads the
//! update file and says what it found; it fetches no artifact, stages
//! nothing, holds nothing and installs nothing** — that is step 11's
//! replacement pieces, which take [`WhatTheCheckFound::ANewerVersion`] from
//! here. **On a Mac they run on this same sequence from step 11's Mac update
//! piece** ([`TheUpdatesAct`], `crate::updating::the_mac`): after a check that
//! found a newer version, and at the user's *Install now* (Z32). **The Linux
//! and Windows routes run on it the same way** (step 11's Linux and Windows update pieces:
//! `crate::updating::the_linux_package`,
//! `crate::updating::the_windows_side_by_side`). While they run, a press starts
//! no check (Z27's second bullet).
//!
//! **What starts a check, and nothing else does (Z26's list of three).** The
//! application's start; the one repeating check while the application runs
//! (Z23); and the user's *Check for updates…* (Z26). All three are one act,
//! [`one_check`], reached by three callers — so E11, Z22, E15 and E16 bind
//! each of them identically by construction. **Z4: nothing checks while the
//! application is not running**, there being no daemon: the sequence is a
//! thread of the running application and ends with it.
//!
//! **The interval, Build's to pick (Z23, R14): three hours** —
//! [`THE_INTERVAL`] carries the reasons.
//!
//! **One sequence, and it is the one standing timer this side has (P10).**
//! [`TheUpdateCheck::the_sequence_runs`] is one thread: it runs the start's
//! check at once, then waits for the next due moment or for the user's press,
//! whichever comes first. **The user's press is not a second timer** (Z26's
//! second bullet): it wakes the same thread, starts no standing sequence and is
//! not a schedule of its own. A refused admission is asked again after
//! `Server protocol and transfers` R6's accounting period, which is a delay
//! inside a running check and bounded by it (P10a, P10b).
//!
//! **Z27: a press joins a check already under way and never starts a second.**
//! And **a completed check resets the interval, whichever route asked for it**:
//! the next automatic check falls one interval after the end of the later of
//! the last completed check and the last automatic one — so a press that
//! completed is not followed by an automatic check moments later, and an
//! automatic check that could not be reached waits a whole interval before
//! the next rather than trying again at once.
//!
//! **Z23a and Z28: the moment is written when a check completes, and only
//! then** — it reached the update file and got an answer, a newer version found
//! or none. A check that could not be reached writes nothing and leaves the
//! moment what it was. **Reached and refused is not an answer**: a file that
//! does not verify is an update that did not happen (E15), so it is *could not
//! be reached* here and writes nothing either. The moment is this computer's
//! clock, `State, identity and recovery` I4's, and About presents it as a
//! moment and never an age.
//!
//! **Z22 and R19: admitted under the update fetch's class at the one desktop
//! point, and every byte drawn from the download budget.** The request is held
//! until it is admitted, for as long as that takes (Z30: no deadline is put on
//! the wait, and waiting is inside *Checking…*).
//!
//! **E11: the request says which version is running and which system, and
//! nothing else** — [`WhatTheRequestSays`], two fields, and the real wire
//! writes them as the one thing it says about who is asking.
//!
//! **E10a: one address, and a test copy never looks.** The file is read from
//! the releases repository's fixed latest address and from nowhere else
//! ([`WHERE_THE_UPDATE_FILE_IS_READ_FROM`]). **A copy that never checks** is a
//! test copy, or any copy started on a throwaway folder
//! ([`this_copy_never_checks`]): no sequence is started and the press reaches
//! no network.
//!
//! **Which version is running** (E15's note of 2026-10-24, written by this
//! piece): a build names its own release by its package version, the one value
//! the release stamps into every build. **A completed check has found a newer
//! version when the verified manifest's version is above it**, compared part by
//! part as whole numbers ([`a_version_above`]). The release sequence keeps its
//! job — the high-water and the floor, `crate::updating::verifying`'s — and is
//! never compared with the running build, which carries none.
//!
//! **E10b: a build that carries no recovery key never checks either.**
//! No update file could verify against it, so every check it made would end
//! as *could not be reached* and About would show a failure that did not
//! happen — the released 0.2.0's control said exactly that. **Such a build is
//! held where a test copy is** ([`why_a_copy_never_checks`]): no sequence, the
//! press reaches no network, and About reads
//! [`TheReading::ThisBuildCannotUpdate`] from the start, known from the build
//! itself. The rule is the carried set's own
//! ([`TheCarriedSet::carries_a_recovery_key`]), the same one the release reads
//! to decide whether to write the update file. The tests stand a signed file
//! at the seam, against a carried set they name.
//!
//! **No network in tests.** The fetch is the seam [`ReadsTheUpdateFile`]; the
//! real one is [`OverHttps`] and nothing under test reaches it.
//!
//! **Markers used below that belong to another document, each with the
//! document it belongs to.** `State, identity and recovery` J7.
//! `Server protocol and transfers` B8b, B8d.
//!
//! **How a clause marker is written in this crate, and it is a rule rather than
//! a habit.** A marker with no document beside it means a clause of this
//! crate's own plan, `Preparation/Architecture/Desktop app and delivery.md`.
//! Every other document's marker is written with that document at the site,
//! or in the list above.

use std::io::Read;
use std::sync::{Arc, Condvar, Mutex, MutexGuard};
use std::time::{Duration, Instant, SystemTime};

use smoozy_server_protocol::transfers::moving::{DrawsFromTheSharedBudget, THE_ACCOUNTING_PERIOD};
use smoozy_server_protocol::transfers::Direction;

use super::manifest::{
    THE_DEBIAN_13_ROW, THE_MAC_ROW, THE_UBUNTU_24_04_ROW, THE_UBUNTU_26_04_ROW, THE_WINDOWS_ROW,
};
use super::trust::{TheCarriedSet, TheUpdateTrustRecord};
use super::verifying::verify_an_update_file_against;
use crate::downloading::TheRunsTransfers;
use crate::events::TheOneQueue;
use crate::record::update_trust;
use crate::scheduling::{TheChannelsDraw, THE_UPDATE_FETCHS_CLASS};

// ════════════════════════════════════════════════════════════════════════════
// The numbers, the address, and what the request says
// ════════════════════════════════════════════════════════════════════════════

/// **The one repeating interval — three hours. Build's decision (Z23 names no
/// number, R14's discipline).**
///
/// **Why not longer.** Z23's whole reason for a repeating check is the user
/// who keeps Smoozy open across a long render, and a render runs for hours to
/// days; three hours puts that user on a new release inside the same working
/// day. **And Z31's hold lifts only at an automatic trigger** — a held update
/// goes in at the first start or repeating check that reads nothing running —
/// so the interval is also how long a held update can sit after the last job
/// ends. Six or twelve hours would leave it waiting most of a day.
///
/// **Why not shorter.** Each check reads one small file over the user's own
/// line, admitted below every user-asked transfer (R19), and a release is
/// published a few times a month at most. Eight reads a day costs nothing;
/// one an hour would be twenty-four reads for the same answer, and a check
/// every few minutes would be the poll P9 forbids in spirit.
///
/// **It is measured from the end of a check, never its start** (Z27, and P10d's
/// pacing rule), on the monotonic clock. **P10c is named and not defended**: a
/// computer asleep across the due moment checks when it wakes, later than the
/// wall clock would say.
pub const THE_INTERVAL: Duration = Duration::from_secs(3 * 60 * 60);

/// **E10a: where the update file is read from, and nowhere else** — the
/// releases repository `Server protocol and transfers` B8b's fixed latest
/// address reads, with the update file's own name,
/// [`super::THE_UPDATE_FILES_NAME`] (a test holds the two together). **Not
/// `Server protocol and transfers` B8d's own-release address**: the update
/// check is the one read that has to see a newer release.
///
/// The repository is the one `crate::setting_up` reads each build's own
/// release from; nothing about what it serves is trusted (E15, E19).
pub const WHERE_THE_UPDATE_FILE_IS_READ_FROM: &str =
    "https://github.com/a-kislyakov/Smoozy-releases/releases/latest/download/desktop-update";

/// **The most the update file may be.** A file of a manifest, its signatures
/// and the trust statements a release keeps is a few kilobytes; a body larger
/// than this is not one, and reading it would be reading whatever a network put
/// there.
pub const THE_MOST_THE_UPDATE_FILE_MAY_BE: u64 = 1024 * 1024;

/// **This build's package version** — the one value the release stamps into
/// every build (E15's note of 2026-10-24), and the version E11's request says.
pub const THIS_BUILDS_VERSION: &str = env!("CARGO_PKG_VERSION");

/// **E11: what an update request says, and the two fields are all of it.**
///
/// *"The request says which version is running and which system it is running
/// on, because a manifest cannot be answered without them, and it says nothing
/// else."* No licence key, no computer identity or name, no account, no
/// server, no path, no count — there is no field here that could hold one.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WhatTheRequestSays {
    /// The running version — [`THIS_BUILDS_VERSION`].
    pub version: String,
    /// The system: this copy's row of E4b where it is one, and otherwise the
    /// operating system and processor the build was made for.
    pub system: String,
}

impl WhatTheRequestSays {
    /// What this copy's requests say.
    pub fn for_this_copy() -> WhatTheRequestSays {
        WhatTheRequestSays {
            version: THIS_BUILDS_VERSION.to_string(),
            system: this_copys_system(),
        }
    }

    /// **The one thing the wire says about who is asking**, written from the
    /// two fields and from nothing else — `Smoozy/0.1.8 (macos-aarch64)`.
    pub fn as_who_is_asking(&self) -> String {
        format!("Smoozy/{} ({})", self.version, self.system)
    }
}

// ════════════════════════════════════════════════════════════════════════════
// Which row this copy runs on
// ════════════════════════════════════════════════════════════════════════════

/// **E4b's row this copy runs on, or none.**
///
/// macOS on Apple silicon and Windows on x86_64 are one row each. **The three
/// Linux rows share one program**, so the row is read from the system's own
/// description exactly as each row's package reads it before it installs —
/// `/etc/os-release`, then `/usr/lib/os-release`, its `ID` and `VERSION_ID`
/// (step 11's release-matrix account). A system that is none of the five has no
/// row, and a check on it completes and finds nothing for it.
pub fn this_copys_row() -> Option<&'static str> {
    let description = if cfg!(target_os = "linux") {
        std::fs::read_to_string("/etc/os-release")
            .or_else(|_| std::fs::read_to_string("/usr/lib/os-release"))
            .ok()
    } else {
        None
    };
    the_row_on(
        std::env::consts::OS,
        std::env::consts::ARCH,
        description.as_deref(),
    )
}

/// The same reading over values handed in, so every row can be driven at the
/// desk.
pub fn the_row_on(system: &str, processor: &str, description: Option<&str>) -> Option<&'static str> {
    match (system, processor) {
        ("macos", "aarch64") => Some(THE_MAC_ROW),
        ("windows", "x86_64") => Some(THE_WINDOWS_ROW),
        ("linux", "x86_64") => {
            let description = description?;
            let field = |name: &str| {
                description.lines().find_map(|line| {
                    let value = line.trim().strip_prefix(name)?.strip_prefix('=')?;
                    Some(value.trim().trim_matches('"').trim_matches('\'').to_string())
                })
            };
            match (field("ID")?.as_str(), field("VERSION_ID")?.as_str()) {
                ("ubuntu", "24.04") => Some(THE_UBUNTU_24_04_ROW),
                ("ubuntu", "26.04") => Some(THE_UBUNTU_26_04_ROW),
                ("debian", "13") => Some(THE_DEBIAN_13_ROW),
                _ => None,
            }
        }
        _ => None,
    }
}

/// **The system E11's request names**: the row where there is one, and
/// otherwise the system and processor alone — never anything read about the
/// computer beyond them.
pub fn this_copys_system() -> String {
    match this_copys_row() {
        Some(row) => row.to_string(),
        None => format!("{}-{}", std::env::consts::OS, std::env::consts::ARCH),
    }
}

// ════════════════════════════════════════════════════════════════════════════
// Newer or not
// ════════════════════════════════════════════════════════════════════════════

/// **Whether an offered version is above the running one** — E15's note of
/// 2026-10-24: part by part, as whole numbers, a missing part read as nought.
/// **A version either side that does not read as whole numbers is never taken
/// as newer**, because the safe side of a wrong reading is a check that found
/// nothing rather than one that offers an older build.
pub fn a_version_above(offered: &str, running: &str) -> bool {
    fn parts(version: &str) -> Option<Vec<u64>> {
        version
            .split('.')
            .map(|part| {
                if part.is_empty() || !part.bytes().all(|b| b.is_ascii_digit()) {
                    None
                } else {
                    part.parse::<u64>().ok()
                }
            })
            .collect()
    }
    let (Some(offered), Some(running)) = (parts(offered), parts(running)) else {
        return false;
    };
    let length = offered.len().max(running.len());
    for at in 0..length {
        let one = offered.get(at).copied().unwrap_or(0);
        let other = running.get(at).copied().unwrap_or(0);
        if one != other {
            return one > other;
        }
    }
    false
}

// ════════════════════════════════════════════════════════════════════════════
// What one check comes to
// ════════════════════════════════════════════════════════════════════════════

/// **What a completed check found** — Z23a's *"a newer version found or
/// none"*.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WhatTheCheckFound {
    /// The release offered is this build's own, an older one, or one with
    /// nothing for this copy's row.
    NothingNewer,
    /// **A release newer than the running one, with an artifact for this row.**
    /// What happens to it is step 11's replacement pieces'.
    ANewerVersion { version: String },
}

/// **How one check ended.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TheCheckEnded {
    /// It reached the update file and the file verified (Z23a's *completes*).
    Completed(WhatTheCheckFound),
    /// **Nothing came back, or what came back did not verify** — Z23a's *a
    /// check that could not be reached*, and E15's *an update that did not
    /// happen*. Nothing is written.
    CouldNotBeReached,
}

impl TheCheckEnded {
    pub fn completed(&self) -> bool {
        matches!(self, TheCheckEnded::Completed(_))
    }
}

/// **The seam the update file is read through.** The real one is
/// [`OverHttps`]; the tests stand a file here, so no test reaches a network.
///
/// It is handed the address, **what the request says and nothing else** (E11),
/// the most it may read, and the admitted channel's draw — every byte it reads
/// is drawn from the download budget first (Z22).
pub trait ReadsTheUpdateFile: Send + Sync {
    fn read(
        &self,
        from: &str,
        says: &WhatTheRequestSays,
        the_most: u64,
        draw: &mut dyn DrawsFromTheSharedBudget,
    ) -> Result<Vec<u8>, String>;
}

/// **Z22 and R19: the update fetch admitted at the one desktop point, under
/// its own class.** Nothing, where the point refuses this time (R13: a refusal
/// is never a failure).
pub trait AdmitsTheUpdateFetch: Send + Sync {
    fn admit_the_update_fetch(&self) -> Option<TheChannelsDraw>;
}

impl AdmitsTheUpdateFetch for TheRunsTransfers {
    fn admit_the_update_fetch(&self) -> Option<TheChannelsDraw> {
        self.admit_to_carry(Direction::Down, &[THE_UPDATE_FETCHS_CLASS], THE_UPDATE_FETCHS_CLASS)
            .ok()
    }
}

/// **One check, whole** — the act all three triggers reach (Z26's third
/// bullet).
///
/// 1. **Admitted** under the update fetch's class, asked again after each
///    refusal for as long as it takes (Z22, Z30).
/// 2. **Read**, from the one address, saying the version and the system and
///    nothing else (E10a, E11), every byte drawn from the budget.
/// 3. **Verified** against what this build carries and the kept record (E15,
///    E19, `State, identity and recovery` J7) — nothing is believed before it.
/// 4. **Newer or not**: the entry for this row, at a version above the running
///    one.
///
/// **Nothing is written here.** The kept trust record moves only at
/// publication (E15), which is not a check's; the moment is the caller's, once
/// this has said the check completed.
pub fn one_check(
    file: &dyn ReadsTheUpdateFile,
    admission: &dyn AdmitsTheUpdateFetch,
    carried: &TheCarriedSet,
    remembered: Option<&TheUpdateTrustRecord>,
    row: Option<&str>,
    says: &WhatTheRequestSays,
) -> TheCheckEnded {
    one_check_and_what_it_verified(file, admission, carried, remembered, row, says).0
}

/// **The same one act, handing on what it verified** where a newer version
/// was found — the manifest's entry for this row and the trust record to keep,
/// which the replacement takes (step 11's Mac update piece). Nothing else is
/// different, and nothing is written here either.
pub fn one_check_and_what_it_verified(
    file: &dyn ReadsTheUpdateFile,
    admission: &dyn AdmitsTheUpdateFetch,
    carried: &TheCarriedSet,
    remembered: Option<&TheUpdateTrustRecord>,
    row: Option<&str>,
    says: &WhatTheRequestSays,
) -> (TheCheckEnded, Option<super::verifying::AVerifiedUpdate>) {
    let mut draw = loop {
        if let Some(draw) = admission.admit_the_update_fetch() {
            break draw;
        }
        // A delay inside a running check (P10a), one accounting period.
        std::thread::sleep(THE_ACCOUNTING_PERIOD);
    };
    let read = file.read(
        WHERE_THE_UPDATE_FILE_IS_READ_FROM,
        says,
        THE_MOST_THE_UPDATE_FILE_MAY_BE,
        &mut draw,
    );
    // **R5: the share returns at once**, before anything is verified.
    let _ = draw.the_channel_ended(Instant::now());
    let Ok(bytes) = read else {
        return (TheCheckEnded::CouldNotBeReached, None);
    };
    let Ok(verified) =
        verify_an_update_file_against(carried, remembered, &bytes, row.unwrap_or(""))
    else {
        return (TheCheckEnded::CouldNotBeReached, None);
    };
    match verified.the_entry_for_this_row {
        Some(_) if a_version_above(&verified.manifest.version, &says.version) => (
            TheCheckEnded::Completed(WhatTheCheckFound::ANewerVersion {
                version: verified.manifest.version.clone(),
            }),
            Some(verified),
        ),
        _ => (TheCheckEnded::Completed(WhatTheCheckFound::NothingNewer), None),
    }
}

// ════════════════════════════════════════════════════════════════════════════
// The real wire
// ════════════════════════════════════════════════════════════════════════════

/// **How many times the address may hand the read on.** The latest address
/// answers with the release's own, and that with where the bytes sit; every
/// hop is held to HTTPS, and nothing about the bytes rests on where they came
/// from (E15).
const HOW_MANY_TIMES_THE_ADDRESS_MAY_HAND_THE_READ_ON: u32 = 5;

/// **The bound on each stage of the read**: connecting, the answer's headers,
/// and the body. A file of a few kilobytes needs none of them; they exist so a
/// line that goes silent ends the check as *could not be reached* rather than
/// holding it, and the user's *Checking…*, for ever.
const THE_BOUND_ON_EACH_STAGE: Duration = Duration::from_secs(60);

/// How much is drawn and read at a time.
const HOW_MUCH_AT_A_TIME: usize = 16 * 1024;

/// **The real read, over the workspace's one HTTPS client** — the same client
/// and the same cryptography every other read of Smoozy's uses
/// (`smoozy_server_protocol::release::fetching`),
/// configured here because what it says about who is asking is E11's and not
/// theirs.
pub struct OverHttps;

impl ReadsTheUpdateFile for OverHttps {
    fn read(
        &self,
        from: &str,
        says: &WhatTheRequestSays,
        the_most: u64,
        draw: &mut dyn DrawsFromTheSharedBudget,
    ) -> Result<Vec<u8>, String> {
        if !from.starts_with("https://") {
            return Err(format!("{from} is not an HTTPS address"));
        }
        let agent = ureq::Agent::config_builder()
            .tls_config(
                ureq::tls::TlsConfig::builder()
                    .provider(ureq::tls::TlsProvider::Rustls)
                    .unversioned_rustls_crypto_provider(Arc::new(
                        rustls::crypto::ring::default_provider(),
                    ))
                    .build(),
            )
            .https_only(true)
            .max_redirects(HOW_MANY_TIMES_THE_ADDRESS_MAY_HAND_THE_READ_ON)
            .http_status_as_error(false)
            // **E11, and the whole of it**: the version and the system.
            .user_agent(says.as_who_is_asking())
            // Nothing the client would add on its own.
            .accept(ureq::config::AutoHeaderValue::None)
            .accept_encoding(ureq::config::AutoHeaderValue::None)
            .timeout_connect(Some(THE_BOUND_ON_EACH_STAGE))
            .timeout_recv_response(Some(THE_BOUND_ON_EACH_STAGE))
            .timeout_recv_body(Some(THE_BOUND_ON_EACH_STAGE))
            .build()
            .new_agent();
        let mut answer = agent.get(from).call().map_err(|failed| failed.to_string())?;
        let status = answer.status().as_u16();
        if status != 200 {
            return Err(format!("{from} answered {status}"));
        }
        if answer.body().content_length().is_some_and(|coming| coming > the_most) {
            return Err(format!(
                "{from} offered more than {}",
                crate::typography::a_figure_with_its_unit(the_most, "bytes")
            ));
        }
        let mut reading = answer.body_mut().as_reader();
        read_drawing_from_the_budget(&mut reading, the_most, draw)
    }
}

/// **A body read with every byte drawn first** (Z22): a draw of nothing waits
/// one accounting period and does not spin, and what was drawn and not read is
/// given back at once.
pub(crate) fn read_drawing_from_the_budget(
    reading: &mut dyn Read,
    the_most: u64,
    draw: &mut dyn DrawsFromTheSharedBudget,
) -> Result<Vec<u8>, String> {
    let mut held = Vec::new();
    let mut chunk = vec![0u8; HOW_MUCH_AT_A_TIME];
    loop {
        let allowed = draw.draw(HOW_MUCH_AT_A_TIME);
        if allowed == 0 {
            std::thread::sleep(THE_ACCOUNTING_PERIOD);
            continue;
        }
        let read = match reading.read(&mut chunk[..allowed]) {
            Ok(read) => read,
            Err(failed) if failed.kind() == std::io::ErrorKind::Interrupted => {
                draw.give_back(allowed);
                continue;
            }
            Err(failed) => {
                draw.give_back(allowed);
                return Err(failed.to_string());
            }
        };
        if read < allowed {
            draw.give_back(allowed - read);
        }
        if read == 0 {
            return Ok(held);
        }
        if held.len() as u64 + read as u64 > the_most {
            return Err(format!(
                "the answer ran past {}",
                crate::typography::a_figure_with_its_unit(the_most, "bytes")
            ));
        }
        held.extend_from_slice(&chunk[..read]);
    }
}

// ════════════════════════════════════════════════════════════════════════════
// The run's check: the one sequence, the press, and what About reads
// ════════════════════════════════════════════════════════════════════════════

/// **Whether this copy never checks** — E10a's *"a throwaway test copy never
/// checks for an update"*. A test copy is built with `SMOOZY_TEST_COPY` set
/// (`App/Build a test copy.sh`) and always
/// starts on a throwaway folder; **any copy started on a throwaway folder is
/// held to it too** (Build's decision), since no such copy is one that should
/// replace itself with a release. A release build has neither.
pub fn this_copy_never_checks() -> bool {
    option_env!("SMOOZY_TEST_COPY").is_some() || crate::platform::locations::this_copy_is_a_throwaway_one()
}

/// **Why a copy never checks** — E10a's test copy, or E10b's build that
/// carries no recovery key.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhyItNeverChecks {
    /// A throwaway test copy (E10a).
    ATestCopy,
    /// **A build that carries no recovery key** (E10b): no update file could
    /// verify against it, so it asks for none.
    ItCarriesNoUpdateKey,
}

/// **Whether, and why, a copy never checks.** A test copy is named first, so
/// its own words stand; otherwise a build that carries no recovery key never
/// checks. Nothing here reaches a network.
pub fn why_a_copy_never_checks(a_test_copy: bool, carried: &TheCarriedSet) -> Option<WhyItNeverChecks> {
    if a_test_copy {
        Some(WhyItNeverChecks::ATestCopy)
    } else if !carried.carries_a_recovery_key() {
        Some(WhyItNeverChecks::ItCarriesNoUpdateKey)
    } else {
        None
    }
}

/// **What About reads about the check** — Design's five readings, the held
/// form of the third, and a copy that never checks.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TheReading {
    /// No check has begun in this run. About reads the stored moment alone.
    NoneYet,
    /// *Checking…* — the request, and the wait to be admitted (Z30).
    Checking,
    /// The last check in this run completed and found nothing newer.
    UpToDate,
    /// **The last check in this run found a newer version.** `held` is Z31's:
    /// whether publication would wait for running jobs and transfers.
    ANewerVersion { version: String, held: bool },
    /// The last check in this run could not be reached.
    CouldNotBeReached,
    /// A newer update was found, but could not be installed.
    CouldNotInstall,
    /// **This copy never checks** (E10a).
    ThisCopyNeverChecks,
    /// **This build carries no recovery key, so it never checks** (E10b):
    /// About says automatic updates aren't available in this version, and
    /// never that the update service could not be reached.
    ThisBuildCannotUpdate,
}

/// **What the user's press did** (Z26, Z27).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThePress {
    /// Nothing was under way, so the press starts the one act.
    StartsACheck,
    /// **A check was already under way, and the press joined it** (Z27): its
    /// completion is the user's answer.
    JoinsTheCheckUnderWay,
    /// This copy never checks, and nothing was started.
    ThisCopyNeverChecks,
    /// **An update is already past its check** — fetching, verifying, or in
    /// its probe — so no check is started at all (Z27's second bullet), and
    /// About goes on reading what the update under way left.
    TheUpdateIsUnderWay,
}

/// **What the sequence asks of the replacement** — step 11's Mac update
/// piece. Both run on the sequence itself, one at a time, never beside a check.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TheUpdatesAct {
    /// A completed check found a newer version: fetch, stage, verify, then
    /// publish unless publication is held (Z31).
    ANewerVersionWasFound,
    /// **Z32: the user's *Install now*** — the held publication, now.
    TheHeldUpdateGoesIn,
    /// **Linux: the update box was answered *Install*** (Z10, Z10a).
    TheLinuxInstallWasAccepted,
    /// **Linux: the update box was dismissed** (Z10b): the mark is written.
    TheLinuxOfferWasDeclined,
    /// **Linux: About's *Install…***, the box for the version already found,
    /// with no check of its own (Z29's ground, Z26's three triggers).
    TheLinuxBoxWasAsked,
}

/// The sequence's own state, behind one lock.
#[derive(Debug)]
struct TheChecksState {
    the_start_check_was_taken: bool,
    a_check_is_running: bool,
    the_user_asked: bool,
    the_last_completed_ended: Option<Instant>,
    the_last_automatic_ended: Option<Instant>,
    the_reading: TheReading,
    /// **Raised at every change of the reading**, so the page can tell the
    /// newer of two About readings whichever arrives first.
    the_readings_number: u64,
    the_run_ended: bool,
    /// **Z27's second bullet**: the replacement is running on the sequence.
    an_update_is_under_way: bool,
    /// **Z32's press**, waiting for the sequence to take it.
    the_user_installs_anyway: bool,
    /// **The Linux box on screen**, by the version it offers (Z10b).
    linux_offer: Option<String>,
    /// Its answer, waiting for the sequence: `true` is *Install*.
    linux_answer: Option<bool>,
    /// About's *Install…*, waiting for the sequence.
    the_user_opens_the_linux_box: bool,
    /// A press joined the check under way (Z27), so its completion is the
    /// user's answer and Z29 reads no mark.
    the_user_joined: bool,
    the_completed_check_was_user_asked: bool,
}

/// What the sequence does next.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum WhatComesNext {
    ACheck { automatic: bool },
    TheHeldUpdateGoesIn,
    TheLinuxAnswer(bool),
    TheLinuxBoxIsAsked,
    NothingUntil(Instant),
    TheRunHasEnded,
}

/// **Z23, Z26 and Z27 as one decision.** The start's check first; then the
/// user's press; then the repeating check, one interval after the end of the
/// later of the last completed check and the last automatic one.
fn what_comes_next(state: &TheChecksState, now: Instant, interval: Duration) -> WhatComesNext {
    if state.the_run_ended {
        return WhatComesNext::TheRunHasEnded;
    }
    if !state.the_start_check_was_taken {
        return WhatComesNext::ACheck { automatic: true };
    }
    if let Some(install) = state.linux_answer {
        return WhatComesNext::TheLinuxAnswer(install);
    }
    if state.the_user_opens_the_linux_box {
        return WhatComesNext::TheLinuxBoxIsAsked;
    }
    if state.the_user_installs_anyway {
        return WhatComesNext::TheHeldUpdateGoesIn;
    }
    if state.the_user_asked {
        return WhatComesNext::ACheck { automatic: false };
    }
    let from = match (state.the_last_completed_ended, state.the_last_automatic_ended) {
        (Some(one), Some(other)) => Some(one.max(other)),
        (one, other) => one.or(other),
    };
    let due = from.map(|from| from + interval).unwrap_or(now);
    if now >= due {
        WhatComesNext::ACheck { automatic: true }
    } else {
        WhatComesNext::NothingUntil(due)
    }
}

/// **The reading a check's ending leaves.**
fn the_reading_after(ended: &TheCheckEnded, held: &dyn Fn() -> bool) -> TheReading {
    match ended {
        TheCheckEnded::Completed(WhatTheCheckFound::NothingNewer) => TheReading::UpToDate,
        TheCheckEnded::Completed(WhatTheCheckFound::ANewerVersion { version }) => {
            TheReading::ANewerVersion {
                version: version.clone(),
                held: held(),
            }
        }
        TheCheckEnded::CouldNotBeReached => TheReading::CouldNotBeReached,
    }
}

/// **The run's one update check.** Made once at the application's start and
/// managed by the shell, so the menu's press and the sequence reach the same
/// one.
#[derive(Debug)]
pub struct TheUpdateCheck {
    held: Mutex<TheChecksState>,
    woken: Condvar,
    this_copy_never_checks: bool,
    the_interval: Duration,
}

impl TheUpdateCheck {
    /// One check, never started for a copy that never checks.
    pub fn new(this_copy_never_checks: bool, the_interval: Duration) -> Arc<TheUpdateCheck> {
        TheUpdateCheck::never_checking_where(
            this_copy_never_checks.then_some(WhyItNeverChecks::ATestCopy),
            the_interval,
        )
    }

    /// **One check, never started where there is a reason it never checks** —
    /// and About's first reading is that reason's.
    pub fn never_checking_where(
        why: Option<WhyItNeverChecks>,
        the_interval: Duration,
    ) -> Arc<TheUpdateCheck> {
        let this_copy_never_checks = why.is_some();
        Arc::new(TheUpdateCheck {
            held: Mutex::new(TheChecksState {
                the_start_check_was_taken: false,
                a_check_is_running: false,
                the_user_asked: false,
                the_last_completed_ended: None,
                the_last_automatic_ended: None,
                the_reading: match why {
                    Some(WhyItNeverChecks::ATestCopy) => TheReading::ThisCopyNeverChecks,
                    Some(WhyItNeverChecks::ItCarriesNoUpdateKey) => {
                        TheReading::ThisBuildCannotUpdate
                    }
                    None => TheReading::NoneYet,
                },
                the_readings_number: 0,
                the_run_ended: false,
                an_update_is_under_way: false,
                the_user_installs_anyway: false,
                linux_offer: None,
                linux_answer: None,
                the_user_opens_the_linux_box: false,
                the_user_joined: false,
                the_completed_check_was_user_asked: false,
            }),
            woken: Condvar::new(),
            this_copy_never_checks,
            the_interval,
        })
    }

    /// **This copy's**: [`this_copy_never_checks`], what this build carries
    /// (E10b), and [`THE_INTERVAL`].
    pub fn for_this_copy() -> Arc<TheUpdateCheck> {
        TheUpdateCheck::never_checking_where(
            why_a_copy_never_checks(this_copy_never_checks(), &TheCarriedSet::this_build()),
            THE_INTERVAL,
        )
    }

    fn state(&self) -> MutexGuard<'_, TheChecksState> {
        self.held.lock().unwrap_or_else(|poisoned| poisoned.into_inner())
    }

    /// Whether this copy never checks.
    pub fn never_checks(&self) -> bool {
        self.this_copy_never_checks
    }

    /// **What About reads now**, with the number of that reading.
    pub fn the_reading(&self) -> (TheReading, u64) {
        let state = self.state();
        (state.the_reading.clone(), state.the_readings_number)
    }

    /// **The user pressed *Check for updates…*** — Z26, Z27 and Z30. **The
    /// reading is *Checking…* from the press**, which is a local act and waits
    /// on nothing; the sequence then runs the check, or the press joins the one
    /// under way.
    pub fn the_user_asks(&self) -> ThePress {
        if self.this_copy_never_checks {
            return ThePress::ThisCopyNeverChecks;
        }
        let mut state = self.state();
        if state.an_update_is_under_way {
            return ThePress::TheUpdateIsUnderWay;
        }
        if state.a_check_is_running || state.the_user_asked {
            state.the_user_joined = true;
            return ThePress::JoinsTheCheckUnderWay;
        }
        state.the_user_asked = true;
        state.the_reading = TheReading::Checking;
        state.the_readings_number += 1;
        self.woken.notify_all();
        ThePress::StartsACheck
    }

    /// **The run is ending**: the sequence stops at its next look, and a check
    /// under way finishes first.
    pub fn the_run_ends(&self) {
        self.state().the_run_ended = true;
        self.woken.notify_all();
    }

    /// **Z32: the user pressed *Install now*.** It reaches the sequence, which
    /// hands it to the replacement; **only the held form of the newer-version
    /// reading carries the button**, so a press while About reads anything
    /// else does nothing. Whether it was taken.
    pub fn the_user_installs_anyway(&self) -> bool {
        let mut state = self.state();
        let held = matches!(state.the_reading, TheReading::ANewerVersion { held: true, .. });
        if !held || state.an_update_is_under_way || self.this_copy_never_checks {
            return false;
        }
        state.the_user_installs_anyway = true;
        self.woken.notify_all();
        true
    }

    /// The Linux route also honours a user's press that joined an automatic
    /// check, so that completion ignores the declined mark on both paths.
    pub fn the_completed_check_was_user_asked(&self) -> bool {
        self.state().the_completed_check_was_user_asked
    }

    /// One box at a time. It stays on its offered version until answered.
    pub fn offer_linux_version(&self, version: &str) -> bool {
        let mut state = self.state();
        if state.linux_offer.is_some() || self.this_copy_never_checks {
            return false;
        }
        state.linux_offer = Some(version.into());
        true
    }

    /// **About's *Install…* on Linux**: taken only from the unheld form of
    /// the newer-version reading, with no box already open and nothing under
    /// way. Whether it was taken.
    pub fn the_user_opens_the_linux_box(&self) -> bool {
        let mut state = self.state();
        let found = matches!(state.the_reading, TheReading::ANewerVersion { held: false, .. });
        if !found
            || state.an_update_is_under_way
            || state.linux_offer.is_some()
            || self.this_copy_never_checks
        {
            return false;
        }
        state.the_user_opens_the_linux_box = true;
        self.woken.notify_all();
        true
    }

    pub fn has_linux_offer(&self) -> bool {
        self.state().linux_offer.is_some()
    }

    /// A stale page cannot install or dismiss a different offered version.
    pub fn answer_linux_offer(&self, version: &str, install: bool) -> bool {
        let mut state = self.state();
        if state.linux_offer.as_deref() != Some(version) || state.linux_answer.is_some() {
            return false;
        }
        state.linux_answer = Some(install);
        self.woken.notify_all();
        true
    }

    /// An orderly quit dismisses an open Linux box too. A process killed
    /// without a dismissal writes no user's choice.
    pub fn dismiss_linux_on_quit(&self, record: &rusqlite::Connection) -> Result<(), String> {
        let mut state = self.state();
        if let Some(version) = state.linux_offer.as_deref() {
            super::the_linux_package::dismissed(record, version)?;
            state.linux_offer = None;
        }
        Ok(())
    }

    /// **The one sequence** — P10's one standing timer. It runs until the run
    /// ends, and **runs nothing at all in a copy that never checks**.
    ///
    /// `one_check` is the act ([`one_check`] over the real wire, in the
    /// running application). `told` hears each reading as it changes — the
    /// start of a check with no ending, and its end with one — and is where the
    /// moment is written and About is told. `held` is Z31's reading, asked when
    /// a newer version is found.
    pub fn the_sequence_runs(
        &self,
        one_check: &mut dyn FnMut() -> TheCheckEnded,
        told: &mut dyn FnMut(&TheReading, u64, Option<&TheCheckEnded>),
        held: &dyn Fn() -> bool,
    ) {
        self.the_sequence_runs_on(one_check, told, held, &mut |_| None)
    }

    /// **The same sequence, with the replacement on it** — `goes_on` is asked
    /// after a check that found a newer version and at Z32's press, and hands
    /// back the reading About is to show where it changes one.
    pub fn the_sequence_runs_on(
        &self,
        one_check: &mut dyn FnMut() -> TheCheckEnded,
        told: &mut dyn FnMut(&TheReading, u64, Option<&TheCheckEnded>),
        held: &dyn Fn() -> bool,
        goes_on: &mut dyn FnMut(TheUpdatesAct) -> Option<TheReading>,
    ) {
        if self.this_copy_never_checks {
            return;
        }
        loop {
            let (automatic, number) = {
                let mut state = self.state();
                loop {
                    let now = Instant::now();
                    match what_comes_next(&state, now, self.the_interval) {
                        WhatComesNext::TheRunHasEnded => return,
                        WhatComesNext::TheLinuxAnswer(install) => {
                            state.linux_answer = None;
                            state.linux_offer = None;
                            state.an_update_is_under_way = true;
                            drop(state);
                            let act = if install {
                                TheUpdatesAct::TheLinuxInstallWasAccepted
                            } else {
                                TheUpdatesAct::TheLinuxOfferWasDeclined
                            };
                            let reading = goes_on(act);
                            self.the_update_has_gone_on(reading, told);
                            state = self.state();
                        }
                        WhatComesNext::TheLinuxBoxIsAsked => {
                            state.the_user_opens_the_linux_box = false;
                            state.an_update_is_under_way = true;
                            drop(state);
                            let reading = goes_on(TheUpdatesAct::TheLinuxBoxWasAsked);
                            self.the_update_has_gone_on(reading, told);
                            state = self.state();
                        }
                        WhatComesNext::TheHeldUpdateGoesIn => {
                            state.the_user_installs_anyway = false;
                            state.an_update_is_under_way = true;
                            drop(state);
                            let reading = goes_on(TheUpdatesAct::TheHeldUpdateGoesIn);
                            self.the_update_has_gone_on(reading, told);
                            state = self.state();
                        }
                        WhatComesNext::ACheck { automatic } => {
                            state.the_start_check_was_taken = true;
                            state.the_user_joined |= state.the_user_asked;
                            state.the_user_asked = false;
                            state.a_check_is_running = true;
                            state.the_reading = TheReading::Checking;
                            state.the_readings_number += 1;
                            break (automatic, state.the_readings_number);
                        }
                        WhatComesNext::NothingUntil(due) => {
                            let wait = due.saturating_duration_since(now);
                            state = self
                                .woken
                                .wait_timeout(state, wait)
                                .unwrap_or_else(|poisoned| poisoned.into_inner())
                                .0;
                        }
                    }
                }
            };
            told(&TheReading::Checking, number, None);
            let ended = one_check();
            let reading = the_reading_after(&ended, held);
            let number = {
                let mut state = self.state();
                let now = Instant::now();
                state.a_check_is_running = false;
                state.the_completed_check_was_user_asked = !automatic || state.the_user_joined;
                state.the_user_joined = false;
                if ended.completed() {
                    state.the_last_completed_ended = Some(now);
                }
                if automatic {
                    state.the_last_automatic_ended = Some(now);
                }
                state.the_reading = reading.clone();
                state.the_readings_number += 1;
                if matches!(reading, TheReading::ANewerVersion { .. }) {
                    state.an_update_is_under_way = true;
                }
                state.the_readings_number
            };
            told(&reading, number, Some(&ended));
            if matches!(reading, TheReading::ANewerVersion { .. }) {
                let reading = goes_on(TheUpdatesAct::ANewerVersionWasFound);
                self.the_update_has_gone_on(reading, told);
            }
        }
    }

    /// **The replacement has returned**: a press starts checks again, and
    /// About is told the reading it left, where it left one.
    fn the_update_has_gone_on(
        &self,
        reading: Option<TheReading>,
        told: &mut dyn FnMut(&TheReading, u64, Option<&TheCheckEnded>),
    ) {
        let number = {
            let mut state = self.state();
            state.an_update_is_under_way = false;
            let Some(reading) = reading.clone() else {
                return;
            };
            state.the_reading = reading;
            state.the_readings_number += 1;
            state.the_readings_number
        };
        if let Some(reading) = reading {
            told(&reading, number, None);
        }
    }
}

/// **Z23a and Z28, then About told** — P7's order: the moment is written
/// first, and only for a check that completed; About's reading is composed
/// from what was written, or from what stands where nothing was.
///
/// A write that fails posts About's reading against the moment that stands,
/// so About never names a moment that was not written.
pub fn the_moment_is_kept_and_about_is_told(
    record: Option<&rusqlite::Connection>,
    queue: &TheOneQueue,
    reading: &TheReading,
    number: u64,
    completed_at: Option<SystemTime>,
) {
    let about = |moment: &Option<SystemTime>| {
        crate::surface::menus::abouts_reading(reading, number, *moment)
    };
    let Some(record) = record else {
        let _ = queue.read_then_post(|| Ok::<_, ()>(None), about);
        return;
    };
    if let Some(at) = completed_at {
        if queue
            .write_then_post(|| update_trust::a_check_completed(record, at).map(|()| Some(at)), about)
            .is_ok()
        {
            return;
        }
    }
    if queue
        .read_then_post(|| update_trust::the_last_completed_check(record), about)
        .is_err()
    {
        let _ = queue.read_then_post(|| Ok::<_, ()>(None), about);
    }
}

/// **The sequence started, in the running application** — `src/main.rs`'s one
/// call. **A copy that never checks starts nothing.**
///
/// The sequence reaches the record, the queue and the one admission point
/// through the shell's own managed state, and holds no lock while a check is
/// out.
pub fn the_sequence_starts<R: tauri::Runtime>(handle: tauri::AppHandle<R>, check: Arc<TheUpdateCheck>) {
    if check.never_checks() {
        return;
    }
    let _ = std::thread::Builder::new()
        .name("smoozy update check".into())
        .spawn(move || {
            let checking = handle.clone();
            let holding = handle.clone();
            let going_on = handle.clone();
            let telling = handle;
            // **What the last check verified, and the Mac route**, kept on this
            // sequence alone (step 11's Mac update piece).
            let verified = std::cell::RefCell::new(None);
            let mut route = super::the_mac::TheMacRoute::new();
            let mut linux = super::the_linux_package::LinuxRoute::default();
            // **And the Windows route** (step 11's Windows update piece), side by side.
            let mut windows = super::the_windows_side_by_side::TheWindowsRoute::new();
            check.the_sequence_runs_on(
                &mut || {
                    let (ended, what) = a_real_check(&checking);
                    *verified.borrow_mut() = what;
                    ended
                },
                &mut |reading, number, ended| {
                    let completed_at = ended
                        .filter(|ended| ended.completed())
                        .map(|_| SystemTime::now());
                    about_is_told_here(&telling, reading, number, completed_at);
                },
                &|| super::the_mac::publication_is_held_here(&holding),
                &mut |act| match act {
                    TheUpdatesAct::ANewerVersionWasFound => {
                        if cfg!(target_os = "linux") {
                            linux.found(&going_on, verified.borrow().as_ref(), &check, false, false)
                        } else if cfg!(target_os = "windows") {
                            windows.a_newer_version_was_found(&going_on, verified.borrow().as_ref())
                        } else {
                            route.a_newer_version_was_found(&going_on, verified.borrow().as_ref())
                        }
                    }
                    TheUpdatesAct::TheHeldUpdateGoesIn => {
                        if cfg!(target_os = "linux") {
                            linux.held(&going_on, &check)
                        } else if cfg!(target_os = "windows") {
                            windows.the_held_update_goes_in(&going_on)
                        } else {
                            route.the_held_update_goes_in(&going_on)
                        }
                    }
                    TheUpdatesAct::TheLinuxInstallWasAccepted => {
                        linux.answered(&going_on, &check, true)
                    }
                    TheUpdatesAct::TheLinuxOfferWasDeclined => {
                        linux.answered(&going_on, &check, false)
                    }
                    TheUpdatesAct::TheLinuxBoxWasAsked => linux.asked(&going_on, &check),
                },
            );
        });
}

/// One real check, over the shell's own managed state.
fn a_real_check<R: tauri::Runtime>(
    handle: &tauri::AppHandle<R>,
) -> (TheCheckEnded, Option<super::verifying::AVerifiedUpdate>) {
    use tauri::Manager;
    let carried = TheCarriedSet::this_build();
    let remembered = handle
        .try_state::<Mutex<crate::start::Started>>()
        .and_then(|started| {
            let started = started.lock().ok()?;
            let record = started.record.as_ref()?;
            super::verifying::the_kept_record_as_it_stands(record.connection(), &carried)
                .ok()
                .flatten()
        });
    let Some(transfers) = handle.try_state::<Arc<TheRunsTransfers>>() else {
        return (TheCheckEnded::CouldNotBeReached, None);
    };
    let transfers: &TheRunsTransfers = transfers.inner().as_ref();
    one_check_and_what_it_verified(
        &OverHttps,
        transfers,
        &carried,
        remembered.as_ref(),
        this_copys_row(),
        &WhatTheRequestSays::for_this_copy(),
    )
}

/// About told, through the shell's own managed state.
fn about_is_told_here<R: tauri::Runtime>(
    handle: &tauri::AppHandle<R>,
    reading: &TheReading,
    number: u64,
    completed_at: Option<SystemTime>,
) {
    use tauri::Manager;
    let Some(queue) = handle.try_state::<Arc<TheOneQueue>>() else {
        return;
    };
    match handle.try_state::<Mutex<crate::start::Started>>() {
        Some(started) => {
            let started = started.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
            the_moment_is_kept_and_about_is_told(
                started.record.as_ref().map(|record| record.connection()),
                &queue,
                reading,
                number,
                completed_at,
            );
        }
        None => the_moment_is_kept_and_about_is_told(None, &queue, reading, number, completed_at),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::events::{Delivers, Delivery, Said};
    use crate::scheduling::{BytesPerSecond, TheCap};
    use crate::updating::manifest::{AnEntry, UpdateManifest, THE_MANIFEST_FORMAT};
    use crate::updating::trust::tests::{
        a_build_with_both_recovery_keys, a_statement, an_online_key, online_1, recovery_a, signed,
    };
    use crate::updating::trust::WhichStatement;
    use crate::updating::verifying::{ASignedStatement, TheUpdateFile};
    use crate::updating::THE_MANIFEST_SIGNING_CONTEXT;
    use smoozy_server_protocol::release::a_signature_over;
    use std::sync::mpsc;

    // ── A file stood at the seam ──────────────────────────────────────────

    /// **What the seam was asked, kept whole** — the address, what the request
    /// said, and how many bytes it drew from the budget.
    #[derive(Default)]
    struct WhatWasAsked {
        asked: Mutex<Vec<(String, WhatTheRequestSays)>>,
        drawn: Mutex<usize>,
    }

    /// **A file at the seam**: the bytes it serves, or none (could not be
    /// reached), and a gate a test may hold it at.
    struct AFileAtTheSeam {
        serves: Option<Vec<u8>>,
        asked: Arc<WhatWasAsked>,
        hold: Option<Arc<(Mutex<bool>, Condvar)>>,
        reached: Option<mpsc::Sender<()>>,
    }

    impl AFileAtTheSeam {
        fn serving(serves: Option<Vec<u8>>) -> (AFileAtTheSeam, Arc<WhatWasAsked>) {
            let asked = Arc::new(WhatWasAsked::default());
            (
                AFileAtTheSeam {
                    serves,
                    asked: asked.clone(),
                    hold: None,
                    reached: None,
                },
                asked,
            )
        }
    }

    impl ReadsTheUpdateFile for AFileAtTheSeam {
        fn read(
            &self,
            from: &str,
            says: &WhatTheRequestSays,
            the_most: u64,
            draw: &mut dyn DrawsFromTheSharedBudget,
        ) -> Result<Vec<u8>, String> {
            self.asked
                .asked
                .lock()
                .unwrap()
                .push((from.to_string(), says.clone()));
            if let Some(reached) = &self.reached {
                let _ = reached.send(());
            }
            if let Some(hold) = &self.hold {
                let (open, turned) = &**hold;
                let mut open_now = open.lock().unwrap();
                let deadline = Instant::now() + Duration::from_secs(20);
                while !*open_now {
                    let left = deadline.saturating_duration_since(Instant::now());
                    assert!(!left.is_zero(), "the test never let the read go");
                    open_now = turned.wait_timeout(open_now, left).unwrap().0;
                }
            }
            let Some(bytes) = self.serves.clone() else {
                return Err("nothing answered".to_string());
            };
            assert!(bytes.len() as u64 <= the_most);
            let mut reading: &[u8] = &bytes;
            let read = read_drawing_from_the_budget(&mut reading, the_most, draw)?;
            *self.asked.drawn.lock().unwrap() += read.len();
            Ok(read)
        }
    }

    /// The whole update file, signed under the test keys: one statement by
    /// recovery key A allowing online key 1, and a manifest at `version` with
    /// an entry for the Mac row.
    fn an_update_file(version: &str, release_sequence: u64) -> (TheCarriedSet, Vec<u8>) {
        let carried = a_build_with_both_recovery_keys();
        let statement = a_statement(
            1,
            WhichStatement::Transition,
            vec![an_online_key("online-1", &online_1())],
        );
        let (statement_bytes, statement_signature) = signed(&recovery_a(), &statement);
        let manifest = UpdateManifest {
            manifest_format: THE_MANIFEST_FORMAT.to_string(),
            release_sequence,
            version: version.to_string(),
            signing_keys: vec!["online-1".to_string()],
            entries: vec![AnEntry {
                row: THE_MAC_ROW.to_string(),
                file_name: "Smoozy.zip".to_string(),
                byte_length: 10,
                digest: [7; 32],
            }],
        };
        let manifest_bytes = manifest.canonical_bytes();
        let signature =
            a_signature_over(&online_1().secret, THE_MANIFEST_SIGNING_CONTEXT, &manifest_bytes);
        let file = TheUpdateFile {
            statements: vec![ASignedStatement {
                bytes: statement_bytes,
                its_recovery_signature: statement_signature,
            }],
            manifest: manifest_bytes,
            manifest_signatures: vec![signature],
        };
        (carried, file.canonical_bytes())
    }

    fn the_mac_asks() -> WhatTheRequestSays {
        WhatTheRequestSays {
            version: "0.1.8".to_string(),
            system: THE_MAC_ROW.to_string(),
        }
    }

    // ── E11: what is sent ─────────────────────────────────────────────────

    /// **E11: the request says the running version and the system, and
    /// nothing else**, and it goes to E10a's one address.
    ///
    /// Three halves. The seam is handed exactly the address and the two
    /// fields. The type holds two fields, so nothing else can be put on it.
    /// And the real wire writes the two as the one thing it says about who is
    /// asking, switches off what the client would add on its own, and sets no
    /// header of its own.
    #[test]
    fn the_request_says_the_version_and_the_system_and_nothing_else() {
        let transfers = TheRunsTransfers::for_this_run();
        let (file, asked) = AFileAtTheSeam::serving(None);
        let says = WhatTheRequestSays::for_this_copy();
        let _ = one_check(
            &file,
            &transfers,
            &TheCarriedSet::this_build(),
            None,
            this_copys_row(),
            &says,
        );
        let asked = asked.asked.lock().unwrap().clone();
        assert_eq!(
            asked,
            vec![(WHERE_THE_UPDATE_FILE_IS_READ_FROM.to_string(), says.clone())]
        );
        assert_eq!(says.version, env!("CARGO_PKG_VERSION"));
        if cfg!(all(target_os = "macos", target_arch = "aarch64")) {
            assert_eq!(says.system, "macos-aarch64");
            assert_eq!(
                says.as_who_is_asking(),
                format!("Smoozy/{} (macos-aarch64)", env!("CARGO_PKG_VERSION"))
            );
        }

        let source = include_str!("checking.rs");
        let (working, _) = source.split_once("#[cfg(test)]").expect("the tests are last");
        let the_struct = working
            .split_once("pub struct WhatTheRequestSays {")
            .and_then(|(_, rest)| rest.split_once('}'))
            .map(|(fields, _)| fields)
            .expect("the request's type is here");
        let fields: Vec<&str> = the_struct
            .lines()
            .map(str::trim)
            .filter(|line| line.starts_with("pub "))
            .collect();
        assert_eq!(fields, vec!["pub version: String,", "pub system: String,"]);
        let the_wire = working
            .split_once("impl ReadsTheUpdateFile for OverHttps {")
            .and_then(|(_, rest)| rest.split_once("fn read_drawing_from_the_budget"))
            .map(|(wire, _)| wire)
            .expect("the real wire is here");
        assert!(the_wire.contains(".user_agent(says.as_who_is_asking())"));
        assert!(the_wire.contains(".accept(ureq::config::AutoHeaderValue::None)"));
        assert!(the_wire.contains(".accept_encoding(ureq::config::AutoHeaderValue::None)"));
        assert!(!the_wire.contains(".header("), "the wire sets a header of its own");
        assert!(!the_wire.contains(".query("), "the wire adds to the address");
        assert!(the_wire.contains("agent.get(from)"));
    }

    /// **E10a: the address is the releases repository's fixed latest one, with
    /// the update file's own name** — never the server package's release
    /// file, and never a build's own-release address.
    #[test]
    fn the_file_is_read_from_the_latest_address_under_its_own_name() {
        assert!(WHERE_THE_UPDATE_FILE_IS_READ_FROM.starts_with(
            "https://github.com/a-kislyakov/Smoozy-releases/releases/latest/download/"
        ));
        assert!(WHERE_THE_UPDATE_FILE_IS_READ_FROM.ends_with(&format!("/{}", super::super::THE_UPDATE_FILES_NAME)));
        assert!(!WHERE_THE_UPDATE_FILE_IS_READ_FROM.ends_with("/release"));
        assert!(!WHERE_THE_UPDATE_FILE_IS_READ_FROM.contains(env!("CARGO_PKG_VERSION")));
    }

    // ── Z22, R19: admitted under the class, drawn from the budget ─────────

    /// **Z22 and R19: the check waits to be admitted under the update fetch's
    /// class, and draws every byte it reads from the download budget.**
    ///
    /// The download cap is set to one floor, so the point admits one channel;
    /// a manual download holds it. The check is refused, asks again, and is
    /// admitted the moment the download's channel ends — and the bytes it
    /// read were drawn.
    #[test]
    fn the_check_is_admitted_under_its_class_and_draws_from_the_budget() {
        let transfers = Arc::new(TheRunsTransfers::for_this_run());
        transfers.a_cap_was_saved(
            Direction::Down,
            TheCap::Of(BytesPerSecond(crate::scheduling::THE_FLOOR_PER_DIRECTION.0)),
        );
        let manual = smoozy_file_browser::pool::THE_DECLARED_ORDER[1].name;
        let holding = transfers
            .admit_to_carry(Direction::Down, &[manual], manual)
            .expect("the download takes the one channel");
        assert!(
            transfers.admit_the_update_fetch().is_none(),
            "the update fetch was admitted beside a channel the budget cannot afford"
        );

        let (version, sequence) = ("0.1.9", 5);
        let (carried, bytes) = an_update_file(version, sequence);
        let (file, asked) = AFileAtTheSeam::serving(Some(bytes.clone()));
        let (done, ended) = mpsc::channel();
        let checking = transfers.clone();
        let running = std::thread::spawn(move || {
            let ended_as = one_check(
                &file,
                checking.as_ref(),
                &carried,
                None,
                Some(THE_MAC_ROW),
                &the_mac_asks(),
            );
            let _ = done.send(ended_as);
        });
        assert!(
            ended.recv_timeout(Duration::from_millis(600)).is_err(),
            "the check ran while the one channel was held"
        );
        assert!(asked.asked.lock().unwrap().is_empty(), "the file was read before admission");
        drop(holding);
        let ended_as = ended
            .recv_timeout(Duration::from_secs(20))
            .expect("the check was admitted once the channel ended");
        running.join().unwrap();
        assert_eq!(
            ended_as,
            TheCheckEnded::Completed(WhatTheCheckFound::ANewerVersion {
                version: version.to_string()
            })
        );
        assert_eq!(*asked.drawn.lock().unwrap(), bytes.len(), "the bytes were not drawn");
        assert_eq!(
            transfers.grants_made(Direction::Down),
            2,
            "one grant to the download and one to the update fetch"
        );

        // And the class: the source says which class it asks for.
        let source = include_str!("checking.rs");
        assert!(source.contains(
            "self.admit_to_carry(Direction::Down, &[THE_UPDATE_FETCHS_CLASS], THE_UPDATE_FETCHS_CLASS)"
        ));
    }

    // ── Newer or not ──────────────────────────────────────────────────────

    /// **E15's note of 2026-10-24: newer is a higher package version**, part by
    /// part as whole numbers; the same or lower is nothing newer, and a version
    /// that does not read is never newer.
    #[test]
    fn a_newer_version_is_a_higher_package_version() {
        assert!(a_version_above("0.1.9", "0.1.8"));
        assert!(a_version_above("0.2.0", "0.1.8"));
        assert!(a_version_above("0.1.10", "0.1.9"), "compared as numbers, not letters");
        assert!(a_version_above("1.0", "0.9.9"));
        assert!(!a_version_above("0.1.8", "0.1.8"));
        assert!(!a_version_above("0.1.7", "0.1.8"));
        assert!(!a_version_above("0.1.8.0", "0.1.8"));
        assert!(!a_version_above("0.2.0-beta", "0.1.8"));
        assert!(!a_version_above("0.2.0", "0.1.8-dev"));
        assert!(!a_version_above("", "0.1.8"));
    }

    /// **What a verified file comes to.** A higher version with an entry for
    /// this row is newer; this build's own version is nothing newer; a release
    /// with nothing for this row is nothing newer; a file that does not verify
    /// is *could not be reached*, and so is nothing at all.
    #[test]
    fn a_verified_file_is_newer_or_nothing_and_a_refused_one_was_not_reached() {
        let transfers = TheRunsTransfers::for_this_run();
        let run = |version: &str, row: Option<&str>, carried_by_this_build: bool| {
            let (carried, bytes) = an_update_file(version, 9);
            let (file, _) = AFileAtTheSeam::serving(Some(bytes));
            let carried = if carried_by_this_build { TheCarriedSet::this_build() } else { carried };
            one_check(&file, &transfers, &carried, None, row, &the_mac_asks())
        };
        assert_eq!(
            run("0.1.9", Some(THE_MAC_ROW), false),
            TheCheckEnded::Completed(WhatTheCheckFound::ANewerVersion { version: "0.1.9".into() })
        );
        assert_eq!(
            run("0.1.8", Some(THE_MAC_ROW), false),
            TheCheckEnded::Completed(WhatTheCheckFound::NothingNewer)
        );
        assert_eq!(
            run("0.1.9", Some(THE_WINDOWS_ROW), false),
            TheCheckEnded::Completed(WhatTheCheckFound::NothingNewer)
        );
        assert_eq!(
            run("0.1.9", None, false),
            TheCheckEnded::Completed(WhatTheCheckFound::NothingNewer)
        );
        // **This build carries none of the test file's keys** — it carries the
        // Creator's — so the same well-signed file is refused, and a refusal
        // is not an answer.
        assert_eq!(run("0.1.9", Some(THE_MAC_ROW), true), TheCheckEnded::CouldNotBeReached);
        // Damaged bytes, and nothing at all.
        let (file, _) = AFileAtTheSeam::serving(Some(b"not an update file".to_vec()));
        assert_eq!(
            one_check(&file, &transfers, &TheCarriedSet::this_build(), None, Some(THE_MAC_ROW), &the_mac_asks()),
            TheCheckEnded::CouldNotBeReached
        );
        let (file, _) = AFileAtTheSeam::serving(None);
        assert_eq!(
            one_check(&file, &transfers, &TheCarriedSet::this_build(), None, Some(THE_MAC_ROW), &the_mac_asks()),
            TheCheckEnded::CouldNotBeReached
        );
    }

    /// **E4b's rows, read the way each Linux package reads its own.**
    #[test]
    fn the_row_is_read_as_the_packages_read_it() {
        assert_eq!(the_row_on("macos", "aarch64", None), Some(THE_MAC_ROW));
        assert_eq!(the_row_on("macos", "x86_64", None), None);
        assert_eq!(the_row_on("windows", "x86_64", None), Some(THE_WINDOWS_ROW));
        let ubuntu = "NAME=\"Ubuntu\"\nVERSION_ID=\"24.04\"\nID=ubuntu\nID_LIKE=debian\n";
        assert_eq!(the_row_on("linux", "x86_64", Some(ubuntu)), Some(THE_UBUNTU_24_04_ROW));
        let ubuntu_26 = "ID=ubuntu\nVERSION_ID=\"26.04\"\n";
        assert_eq!(the_row_on("linux", "x86_64", Some(ubuntu_26)), Some(THE_UBUNTU_26_04_ROW));
        let debian = "PRETTY_NAME=\"Debian GNU/Linux 13 (trixie)\"\nVERSION_ID=\"13\"\nID=debian\n";
        assert_eq!(the_row_on("linux", "x86_64", Some(debian)), Some(THE_DEBIAN_13_ROW));
        assert_eq!(the_row_on("linux", "x86_64", Some("ID=ubuntu\nVERSION_ID=\"22.04\"\n")), None);
        assert_eq!(the_row_on("linux", "x86_64", Some("ID_LIKE=debian\nVERSION_ID=\"13\"\n")), None);
        assert_eq!(the_row_on("linux", "x86_64", None), None);
        assert_eq!(the_row_on("linux", "aarch64", Some(debian)), None);
    }

    // ── The sequence: the three triggers, the coalescing ─────────────────

    /// A sequence driven at the desk: every check it runs is reported on a
    /// channel, and each check waits until the test lets it end.
    struct ADrivenSequence {
        check: Arc<TheUpdateCheck>,
        started: mpsc::Receiver<bool>,
        let_end: mpsc::Sender<TheCheckEnded>,
        told: Arc<Mutex<Vec<(TheReading, u64, bool)>>>,
        running: Option<std::thread::JoinHandle<()>>,
    }

    impl ADrivenSequence {
        fn of(never: bool, interval: Duration) -> ADrivenSequence {
            let check = TheUpdateCheck::new(never, interval);
            let (started_tx, started) = mpsc::channel();
            let (let_end, ends) = mpsc::channel::<TheCheckEnded>();
            let told = Arc::new(Mutex::new(Vec::new()));
            let running = {
                let check = check.clone();
                let told = told.clone();
                std::thread::spawn(move || {
                    check.the_sequence_runs(
                        &mut || {
                            let _ = started_tx.send(true);
                            ends.recv_timeout(Duration::from_secs(20))
                                .unwrap_or(TheCheckEnded::CouldNotBeReached)
                        },
                        &mut |reading, number, ended| {
                            told.lock().unwrap().push((reading.clone(), number, ended.is_some()));
                        },
                        &|| false,
                    );
                })
            };
            ADrivenSequence {
                check,
                started,
                let_end,
                told,
                running: Some(running),
            }
        }

        fn a_check_starts_within(&self, within: Duration) -> bool {
            self.started.recv_timeout(within).is_ok()
        }

        fn the_check_ends_as(&self, ended: TheCheckEnded) {
            self.let_end.send(ended).unwrap();
        }

        /// Wait, to a fixed deadline, for the sequence to have told `count`
        /// readings.
        fn told_at_least(&self, count: usize) -> Vec<(TheReading, u64, bool)> {
            let deadline = Instant::now() + Duration::from_secs(20);
            loop {
                let told = self.told.lock().unwrap().clone();
                if told.len() >= count || Instant::now() >= deadline {
                    return told;
                }
                std::thread::sleep(Duration::from_millis(5));
            }
        }
    }

    impl Drop for ADrivenSequence {
        fn drop(&mut self) {
            self.check.the_run_ends();
            let _ = self.let_end.send(TheCheckEnded::CouldNotBeReached);
            if let Some(running) = self.running.take() {
                let _ = running.join();
            }
        }
    }

    /// **Z23: a check at the application's start, at once.**
    #[test]
    fn a_check_runs_at_the_start() {
        let driven = ADrivenSequence::of(false, Duration::from_secs(3600));
        assert!(driven.a_check_starts_within(Duration::from_secs(5)), "no check at the start");
        assert_eq!(driven.check.the_reading().0, TheReading::Checking);
        driven.the_check_ends_as(TheCheckEnded::Completed(WhatTheCheckFound::NothingNewer));
        let told = driven.told_at_least(2);
        assert_eq!(told[0], (TheReading::Checking, 1, false));
        assert_eq!(told[1], (TheReading::UpToDate, 2, true));
        assert!(
            !driven.a_check_starts_within(Duration::from_millis(300)),
            "a second check ran with nothing asking for one"
        );
    }

    /// **Z23: the one repeating check, one interval after the last one's end**
    /// — and an automatic check that could not be reached waits a whole
    /// interval before the next.
    #[test]
    fn a_check_repeats_at_the_interval() {
        let interval = Duration::from_millis(400);
        let driven = ADrivenSequence::of(false, interval);
        assert!(driven.a_check_starts_within(Duration::from_secs(5)));
        driven.the_check_ends_as(TheCheckEnded::CouldNotBeReached);
        let ended_at = Instant::now();
        assert!(
            !driven.a_check_starts_within(Duration::from_millis(250)),
            "the next check came before the interval had passed"
        );
        assert!(driven.a_check_starts_within(Duration::from_secs(5)), "no repeating check");
        assert!(ended_at.elapsed() >= Duration::from_millis(350));
        driven.the_check_ends_as(TheCheckEnded::Completed(WhatTheCheckFound::NothingNewer));
        assert!(driven.a_check_starts_within(Duration::from_secs(5)), "the check stopped repeating");
    }

    /// **Z26: the user's press starts the one act**, and its reading is
    /// *Checking…* from the press (Z30).
    #[test]
    fn the_users_press_starts_a_check() {
        let driven = ADrivenSequence::of(false, Duration::from_secs(3600));
        assert!(driven.a_check_starts_within(Duration::from_secs(5)));
        driven.the_check_ends_as(TheCheckEnded::CouldNotBeReached);
        driven.told_at_least(2);
        assert_eq!(driven.check.the_reading().0, TheReading::CouldNotBeReached);

        assert_eq!(driven.check.the_user_asks(), ThePress::StartsACheck);
        assert_eq!(driven.check.the_reading().0, TheReading::Checking, "the press drew nothing");
        assert!(driven.a_check_starts_within(Duration::from_secs(5)), "the press started no check");
        driven.the_check_ends_as(TheCheckEnded::Completed(WhatTheCheckFound::NothingNewer));
        let told = driven.told_at_least(4);
        assert_eq!(told[3].0, TheReading::UpToDate);
    }

    /// **Z27: a press while a check is under way joins it and never starts a
    /// second**, whichever route started the first; and a completed check
    /// resets the interval, whichever route asked for it.
    #[test]
    fn a_press_joins_the_check_under_way() {
        let interval = Duration::from_millis(600);
        let driven = ADrivenSequence::of(false, interval);
        assert!(driven.a_check_starts_within(Duration::from_secs(5)));
        // The start's check is out: two presses join it.
        assert_eq!(driven.check.the_user_asks(), ThePress::JoinsTheCheckUnderWay);
        assert_eq!(driven.check.the_user_asks(), ThePress::JoinsTheCheckUnderWay);
        driven.the_check_ends_as(TheCheckEnded::Completed(WhatTheCheckFound::NothingNewer));
        assert!(
            !driven.a_check_starts_within(Duration::from_millis(300)),
            "a press under way started a second check"
        );
        // A press now starts one, and a second press before it begins joins.
        assert_eq!(driven.check.the_user_asks(), ThePress::StartsACheck);
        assert_eq!(driven.check.the_user_asks(), ThePress::JoinsTheCheckUnderWay);
        assert!(driven.a_check_starts_within(Duration::from_secs(5)));
        driven.the_check_ends_as(TheCheckEnded::Completed(WhatTheCheckFound::NothingNewer));
        let pressed_at = Instant::now();
        // The press's completed check moved the interval: the automatic one
        // falls an interval after it, not after the start's.
        assert!(driven.a_check_starts_within(Duration::from_secs(5)));
        assert!(
            pressed_at.elapsed() >= Duration::from_millis(500),
            "an automatic check followed a completed press at once"
        );
    }

    /// **Z27's second bullet and Z32, on the sequence.** A check finds a
    /// newer version and the replacement runs on the sequence: a press while
    /// it runs starts no check at all. The replacement leaves the held form;
    /// *Install now* is taken only from it, once, and reaches the replacement
    /// as the held update going in.
    #[test]
    fn the_update_runs_on_the_sequence_and_install_now_goes_past_the_hold_once() {
        let check = TheUpdateCheck::new(false, Duration::from_secs(3600));
        let (asked, acts) = mpsc::channel::<TheUpdatesAct>();
        let (answer, answers) = mpsc::channel::<Option<TheReading>>();
        let told = Arc::new(Mutex::new(Vec::new()));
        let running = {
            let check = check.clone();
            let told = told.clone();
            std::thread::spawn(move || {
                let mut checks = 0;
                check.the_sequence_runs_on(
                    &mut || {
                        checks += 1;
                        if checks == 1 {
                            TheCheckEnded::Completed(WhatTheCheckFound::ANewerVersion { version: "0.2.0".into() })
                        } else {
                            TheCheckEnded::CouldNotBeReached
                        }
                    },
                    &mut |reading, _, _| told.lock().unwrap().push(reading.clone()),
                    &|| true,
                    &mut |act| {
                        let _ = asked.send(act);
                        answers.recv_timeout(Duration::from_secs(20)).ok().flatten()
                    },
                );
            })
        };
        assert_eq!(
            acts.recv_timeout(Duration::from_secs(5)),
            Ok(TheUpdatesAct::ANewerVersionWasFound),
            "the replacement was not asked after a newer version was found"
        );
        assert_eq!(check.the_user_asks(), ThePress::TheUpdateIsUnderWay, "a press started a check");
        assert!(!check.the_user_installs_anyway(), "Install now was taken while the update ran");
        let held = TheReading::ANewerVersion { version: "0.2.0".into(), held: true };
        answer.send(Some(held.clone())).unwrap();
        let deadline = Instant::now() + Duration::from_secs(5);
        while told.lock().unwrap().len() < 3 {
            assert!(Instant::now() < deadline, "the held form was not left");
            std::thread::sleep(Duration::from_millis(5));
        }
        assert!(check.the_user_installs_anyway(), "Install now was refused on the held form");
        assert_eq!(acts.recv_timeout(Duration::from_secs(5)), Ok(TheUpdatesAct::TheHeldUpdateGoesIn));
        answer.send(Some(TheReading::CouldNotInstall)).unwrap();
        let deadline = Instant::now() + Duration::from_secs(5);
        while told.lock().unwrap().len() < 4 {
            assert!(Instant::now() < deadline, "the update that did not happen was not told");
            std::thread::sleep(Duration::from_millis(5));
        }
        assert!(!check.the_user_installs_anyway(), "Install now was taken off the held form");
        assert!(
            acts.recv_timeout(Duration::from_millis(300)).is_err(),
            "the replacement was asked again with nothing asking"
        );
        let told = told.lock().unwrap().clone();
        assert_eq!(
            told,
            vec![
                TheReading::Checking,
                TheReading::ANewerVersion { version: "0.2.0".into(), held: true },
                held,
                TheReading::CouldNotInstall,
            ]
        );
        check.the_run_ends();
        running.join().unwrap();
    }

    /// **E10b: a build checks exactly when it carries an update key.** A
    /// build with none — the released 0.2.0's case, where the control said
    /// *Could not reach the update service.* about a file no copy of it could
    /// ever believe — never checks, and its reading is known from the build
    /// before anything is asked. **From 2026-11-01 this build carries the
    /// Creator's two master keys** (`crate::updating::trust`), so this copy
    /// checks again, and its reading is never *this build cannot update*.
    #[test]
    fn a_build_checks_exactly_when_it_carries_an_update_key() {
        assert!(TheCarriedSet::this_build().carries_a_recovery_key());
        let check = TheUpdateCheck::for_this_copy();
        assert_ne!(check.the_reading().0, TheReading::ThisBuildCannotUpdate);
        if !this_copy_never_checks() {
            assert!(!check.never_checks(), "a build carrying update keys never checks");
        }

        // **The rule, from the carried set alone.** A test copy keeps its own
        // reason; a build with keys checks; a build with none does not.
        let a_build_with_no_key = TheCarriedSet {
            recovery_keys: Vec::new(),
            statement: None,
            greatest_release_sequence_published: 0,
        };
        assert_eq!(why_a_copy_never_checks(false, &TheCarriedSet::this_build()), None);
        assert_eq!(why_a_copy_never_checks(false, &a_build_with_both_recovery_keys()), None);
        assert_eq!(
            why_a_copy_never_checks(false, &a_build_with_no_key),
            Some(WhyItNeverChecks::ItCarriesNoUpdateKey)
        );
        assert_eq!(
            why_a_copy_never_checks(true, &TheCarriedSet::this_build()),
            Some(WhyItNeverChecks::ATestCopy)
        );
        assert_eq!(
            why_a_copy_never_checks(true, &a_build_with_no_key),
            Some(WhyItNeverChecks::ATestCopy)
        );

        // **No check at the start, none at the interval, the press reaches
        // nothing, and About reads the build's own reason throughout.**
        let check = TheUpdateCheck::never_checking_where(
            Some(WhyItNeverChecks::ItCarriesNoUpdateKey),
            Duration::from_millis(20),
        );
        assert_eq!(check.the_reading().0, TheReading::ThisBuildCannotUpdate);
        assert_eq!(check.the_user_asks(), ThePress::ThisCopyNeverChecks);
        assert_eq!(check.the_reading().0, TheReading::ThisBuildCannotUpdate);
        assert!(!check.the_user_installs_anyway());
        let mut told = 0;
        check.the_sequence_runs(
            &mut || panic!("a build with no update key made a check"),
            &mut |_, _, _| told += 1,
            &|| false,
        );
        assert_eq!(told, 0);
        assert_eq!(check.the_reading().0, TheReading::ThisBuildCannotUpdate);
    }

    /// **E10a: a test copy never checks** — no check at the start, none at
    /// the interval, and the press starts nothing.
    #[test]
    fn a_copy_that_never_checks_runs_nothing() {
        let driven = ADrivenSequence::of(true, Duration::from_millis(50));
        assert!(!driven.a_check_starts_within(Duration::from_millis(400)), "a test copy checked");
        assert_eq!(driven.check.the_user_asks(), ThePress::ThisCopyNeverChecks);
        assert!(!driven.a_check_starts_within(Duration::from_millis(200)), "the press checked");
        assert_eq!(driven.check.the_reading().0, TheReading::ThisCopyNeverChecks);
        assert!(driven.told.lock().unwrap().is_empty());

        // **And the running application starts no sequence for one**, and
        // decides it from the test copy's build mark and the throwaway folder.
        let source = include_str!("checking.rs");
        let (working, _) = source.split_once("#[cfg(test)]").expect("the tests are last");
        assert!(working.contains(
            "option_env!(\"SMOOZY_TEST_COPY\").is_some() || crate::platform::locations::this_copy_is_a_throwaway_one()"
        ));
        let starts = working
            .split_once("pub fn the_sequence_starts")
            .and_then(|(_, rest)| rest.split_once("std::thread::Builder"))
            .map(|(head, _)| head)
            .expect("the start is here");
        assert!(starts.contains("if check.never_checks() {\n        return;"));
        let main = std::fs::read_to_string(
            std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src/main.rs"),
        )
        .unwrap();
        assert!(main.contains("checking::TheUpdateCheck::for_this_copy()"));
        assert!(main.contains("checking::the_sequence_starts("));
    }

    // ── Z23a, Z28: the moment ─────────────────────────────────────────────

    struct Held(Arc<Mutex<Vec<Delivery>>>);
    impl Delivers for Held {
        fn deliver(&self, delivery: &Delivery) {
            self.0.lock().unwrap().push(delivery.clone());
        }
    }

    fn a_record_at_49() -> rusqlite::Connection {
        let connection = rusqlite::Connection::open_in_memory().unwrap();
        for statement in crate::record::schema::VERSION_49 {
            connection.execute(statement, []).unwrap();
        }
        connection
    }

    /// **Z23a and Z28: the moment is written when a check completes, and only
    /// then**; a check that could not be reached leaves it what it was; and
    /// About is told against what was written.
    #[test]
    fn the_moment_is_written_only_when_a_check_completes() {
        let record = a_record_at_49();
        let seen = Arc::new(Mutex::new(Vec::new()));
        let queue = TheOneQueue::new();
        queue.the_page_is_ready(Box::new(Held(seen.clone())));
        let first = std::time::UNIX_EPOCH + Duration::from_secs(1_761_300_000);

        // Could not be reached, with nothing ever written: nothing written.
        the_moment_is_kept_and_about_is_told(Some(&record), &queue, &TheReading::CouldNotBeReached, 3, None);
        assert_eq!(update_trust::the_last_completed_check(&record).unwrap(), None);

        // Completed: written, at the moment handed in.
        the_moment_is_kept_and_about_is_told(Some(&record), &queue, &TheReading::UpToDate, 4, Some(first));
        assert_eq!(update_trust::the_last_completed_check(&record).unwrap(), Some(first));

        // Could not be reached afterwards: the moment stands.
        the_moment_is_kept_and_about_is_told(Some(&record), &queue, &TheReading::CouldNotBeReached, 5, None);
        assert_eq!(update_trust::the_last_completed_check(&record).unwrap(), Some(first));

        // A newer version found completes too, and moves it.
        let later = first + Duration::from_secs(3 * 3600);
        the_moment_is_kept_and_about_is_told(
            Some(&record),
            &queue,
            &TheReading::ANewerVersion { version: "0.1.9".into(), held: false },
            6,
            Some(later),
        );
        assert_eq!(update_trust::the_last_completed_check(&record).unwrap(), Some(later));

        // About told with no ending: it reads the moment that stands.
        the_moment_is_kept_and_about_is_told(Some(&record), &queue, &TheReading::UpToDate, 7, None);
        assert_eq!(update_trust::the_last_completed_check(&record).unwrap(), Some(later));

        let seen = seen.lock().unwrap();
        assert_eq!(seen.len(), 5, "About was not told at every ending");
        let moment = |at: usize| seen[at].said.get("lastChecked").cloned();
        let seconds = |at: &str| Some(Said::Value(crate::typography::Verbatim::new(at)));
        // The first reading is the only one that names the moment; the
        // failure and the newer version do not (Design's drawing).
        assert_eq!(moment(0), None);
        assert_eq!(moment(1), seconds("1761300000"));
        assert_eq!(moment(2), None);
        assert_eq!(moment(3), None);
        assert_eq!(moment(4), seconds("1761310800"));
        assert!(seen.iter().all(|one| one.kind == crate::surface::menus::THE_ABOUTS_READING));
    }

    /// **The sequence's own route to the moment**: only an ending that
    /// completed carries one, so the start of a check and an ending that could
    /// not be reached write nothing.
    #[test]
    fn the_sequence_hands_a_moment_only_to_a_completed_check() {
        let source = include_str!("checking.rs");
        let (working, _) = source.split_once("#[cfg(test)]").expect("the tests are last");
        assert!(working.contains(
            ".filter(|ended| ended.completed())\n                        .map(|_| SystemTime::now());"
        ));
        assert!(TheCheckEnded::Completed(WhatTheCheckFound::NothingNewer).completed());
        assert!(!TheCheckEnded::CouldNotBeReached.completed());
    }

    /// **P10: the sequence is the one standing timer, and a press is not a
    /// second.** One thread is started, by one function, and nothing else in
    /// this file starts one.
    #[test]
    fn the_sequence_is_the_one_timer_and_a_press_starts_no_thread() {
        let source = include_str!("checking.rs");
        let (working, _) = source.split_once("#[cfg(test)]").expect("the tests are last");
        assert_eq!(working.matches("std::thread::Builder::new()").count(), 1);
        assert_eq!(working.matches("std::thread::spawn").count(), 0);
        let press = working
            .split_once("pub fn the_user_asks(&self) -> ThePress {")
            .and_then(|(_, rest)| rest.split_once("pub fn the_run_ends"))
            .map(|(press, _)| press)
            .expect("the press is here");
        assert!(!press.contains("thread"), "the press starts a thread of its own");
        assert!(press.contains("self.woken.notify_all();"));
    }
}
