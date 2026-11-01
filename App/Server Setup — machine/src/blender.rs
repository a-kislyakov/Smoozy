//! The Blender lifecycle — `Server Setup` §5.1, §5.2 and §5.3.
//!
//! **The constraint** is Row 92 and the Creator's rulings of 2026-08-12: the
//! list is built from the shop-front, the fetch stays on Blender's own
//! downloads, and the quick path installs the latest finished release —
//! whichever line publishes it, without preferring an older long-term-support
//! release (Creator-stated 2026-10-27). **Revised by the Creator on
//! 2026-10-02 for unfinished builds alone** (notes 5 and 7 on 0.1.7): alpha,
//! beta and release-candidate builds are listed and fetched from Blender's own
//! build service. The fetch still comes only from Blender itself.
//!
//! **No Blender fact is settled here and none is looked up here.** Everything
//! below traces to the clause of the plan that carries it, with the plan's own
//! label.
//!
//! **Markers used below that belong to another document, each with the document
//! it belongs to.** `Server protocol and transfers` §7.1, S9, S12, S22,
//! S24, R17. `Rented-server runtime` G19. `Job files and outputs` N1,
//! L6, 1.2. `File Browser` L8. `Rendering` L14. `Queue & History` N10.
//!
//! **`Server protocol and transfers` S4 and S5, and `Rented-server runtime` §13's
//! R1, are not listed above and are written out at each site**, because this plan
//! defines an S4, an S5 and an R1 of its own and a bare one could mean either.

use std::time::Duration;

/// **B1: the reading discipline every Blender run in this document sits
/// inside** (Row 94).
///
/// **A build is asked its version by running it with its version switch; the
/// answer is the line beginning `Blender `, never the first line** — the apt
/// build printed two lines of colour-management noise first (Fact 45).
pub const THE_LINE_THE_VERSION_IS_READ_FROM: &str = "Blender ";

/// **B1: what that one line carries, and both further marks are taken from it
/// and from nothing else.**
///
/// **No folder name, path or channel is parsed for either** — what a machine
/// holds is read by asking the build, and the shop-front's own track and risk
/// marks describe the list rather than a build on a machine.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WhatTheBuildAnswered {
    pub version: Version,
    /// Whether the build is a long-term-support release. Label: **observed**.
    pub long_term_support: bool,
    /// Whether it is a finished release or an unstable build. Label:
    /// **documented**, and **untested by this project**, every build tested
    /// having been a finished release.
    pub finished_release: bool,
}

/// **B1, performed: the version answer read off a captured run.**
///
/// **A run that answered nothing answers nothing here** — never a guess, and
/// never the first line.
pub fn read_the_version_answer(output: &str) -> Option<Version> {
    output
        .lines()
        .find_map(|line| line.strip_prefix(THE_LINE_THE_VERSION_IS_READ_FROM))
        .and_then(|rest| Version::parse(rest.split_whitespace().next()?))
}

/// **B1: both streams are captured, always.**
///
/// The refusal to open a window goes to the error stream in 4.5 and the
/// ordinary output in the other three, and some of Blender's own messages reach
/// the error stream in ordinary runs anyway — **so an empty error stream is
/// never read as a property of having no window.**
pub fn an_empty_error_stream_says_anything_about_the_window(_it_was_empty: bool) -> bool {
    false
}

/// **B1: the time limit every Blender run this module makes carries.**
///
/// *"Anything running 4.5 unattended needs a time limit and something outside
/// Blender ready to kill it; the other three versions do not"* — **applied to
/// all four versions because the module cannot know a candidate's version
/// before asking.**
///
/// **This used to be a `bool` asserting the promise, with a test asserting the
/// constant, while no run on the path carried a limit at all** — so a reader of
/// the crate was told the promise was kept. **The bound landed 2026-09-04, out
/// of this module's round-1 code audit**, and the one run the module makes is
/// `acts::ask_one_build_its_version` in `smoozy-server-setup`, which carries it.
///
/// **Sixty seconds, and the value is decided rather than measured**, exactly as
/// the command-gap window and the frontend lock's bounded wait are, and it says
/// so here as they do. A version ask is one process start and one printed line;
/// the whole-disk search itself was observed at 5.3 seconds on a real disk; **a
/// build that has not printed its `Blender ` line inside a minute is not going to
/// print one.** **A run that hits the limit answers nothing**, which B11 already
/// names as a candidate that did not answer.
///
/// **What it costs if it is wrong**: too short and a build that would have
/// answered lands on the did-not-answer list where the user can see it; too long
/// and the user waits longer at a settings page on a machine billed by the hour.
/// **Neither damages a machine.**
pub const THE_TIME_LIMIT_EVERY_BLENDER_RUN_CARRIES: Duration = Duration::from_secs(60);

/// **B1: what that kill reaches, stated here because two rules elsewhere forbid
/// every signal and neither is meant to reach this one.**
///
/// **The target is the invoked Blender process and nothing above it** — never
/// the setup worker, never a shell around it, never anything found by name,
/// pattern or process search. [`crate::driving`] Y8's rule that no signal is
/// ever sent to the worker or its children, and the declarations — `declarations` in `smoozy-server-setup` —
/// *none* for which member receives a termination, are both about ending the
/// **operation**, and neither reaches a child the run itself put a limit on.
/// **The package system mid-transaction is still never signalled**, and this
/// narrows those rules' reach rather than weakening them.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhatTheTimeLimitKills {
    TheInvokedBlenderProcessAndNothingAboveIt,
}

/// **B1's second half is unkept, and it is recorded rather than claimed.**
///
/// **What is built** is the desktop's own bound on the wait
/// ([`THE_TIME_LIMIT_EVERY_BLENDER_RUN_CARRIES`]), which ends the hang: the ask
/// stops waiting, the candidate answers nothing, and B11's did-not-answer list
/// is where it lands.
///
/// **What is not built** is the kill itself. It must reach *"the invoked Blender
/// process and nothing above it"*, which is on the machine and not at the desk,
/// and **nothing in these plans establishes a route for it**: a command naming
/// that one process would carry a machine-origin identifier into a command
/// string, which `Server protocol and transfers` 1.1 forbids in either phase and
/// none of its carriages covers — the fifth, added 2026-09-23, carries a name a
/// detection tool on the machine answers, and no tool answers which process a
/// desk-side limit meant — and writing such a command from memory is
/// what Rule 2 forbids outright.
///
/// **What breaks while it is unkept**: a run that hit the desktop's limit leaves
/// **a Blender still running on a machine the user pays for**, until the machine
/// is restarted or the process ends on its own. **Nothing hangs and nothing is
/// claimed** — what the user is told is that the candidate did not answer.
pub const B1S_KILL_HALF_IS_UNKEPT_AND_WHAT_BREAKS_IS_A_BLENDER_LEFT_RUNNING: bool = true;

/// **B1: a run killed at its limit answers nothing and is recorded as answering
/// nothing** — named as one that did not answer where it was a search candidate
/// (`search` in `smoozy-server-setup` B11).
pub fn a_killed_run_answers(_it_was_killed: bool) -> Option<Version> {
    None
}

/// **B1: no build's engine list is got by the command that prints one, on an
/// unattended machine.**
///
/// Two topics record that command both working and failing with no display
/// server, both stand, and *"Until it is settled, do not get a build's engine
/// list that way on an unattended machine."* **Nothing in this document asks
/// for an engine list.**
pub fn an_engine_list_is_asked_for_anywhere_here() -> bool {
    false
}

/// A version, parsed into its components. **Never a string comparison** — a
/// plain alphabetical sort picks 4.2.9 as newest where 4.2.23 exists (Fact 54).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Version {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
}

impl Version {
    pub fn parse(text: &str) -> Option<Version> {
        let mut parts = text.trim().split('.');
        let major = parts.next()?.parse().ok()?;
        let minor = parts.next()?.parse().ok()?;
        // A two-component version is a real shape — the settings folder is two
        // numbers, not three — so an absent patch is zero rather than a
        // refusal.
        let patch = match parts.next() {
            Some(text) => text.parse().ok()?,
            None => 0,
        };
        if parts.next().is_some() {
            return None;
        }
        Some(Version {
            major,
            minor,
            patch,
        })
    }

    /// **The version-numbered settings folder is two numbers, not three**
    /// ([`crate::addons`] A6): patch releases share it.
    pub fn settings_folder(self) -> String {
        format!("{}.{}", self.major, self.minor)
    }
}

/// **The whole-app floor, which [`the_offered_list`] is floored at.**
pub const THE_FLOOR: Version = Version {
    major: 4,
    minor: 5,
    patch: 0,
};

/// **B2: one entry of the shop-front's own machine-readable listing.**
///
/// Observed, 2026-08-19: the Snap Store's info endpoint for the Blender
/// Foundation's own package answers **one entry per channel**, each carrying the
/// track, the risk, the version, the release moment and the byte size — **the
/// long-term-support lines marked in the track's own name, the unstable kinds
/// marked by risk.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PublishedChannel {
    /// `latest`, `4.5lts`, `5.0`, …
    pub track: String,
    pub risk: Risk,
    pub version: Version,
    /// **The release moment, as the shop-front's own text and never as a moment
    /// Smoozy has worked out.**
    ///
    /// **This was a clock reading until 2026-09-07 and nothing ever set one**,
    /// because nothing fetched the listing. When the fetch was built, turning
    /// the shop-front's text into a moment meant writing a date reading from
    /// memory for a value **nothing in this module compares, sorts or subtracts**
    /// — the sort is on version components (Fact 54) and the floor is a version
    /// comparison. **And the moment is the shop-front's clock**: readings taken
    /// on different clocks are never subtracted here, so a value that cannot be
    /// subtracted by accident is the safer one to hold.
    ///
    /// **What it costs**: anything that ever wants to compare release moments
    /// has to read this text first, and that reading does not exist yet. Nothing
    /// wants one today.
    pub released_at: String,
    pub size_bytes: u64,
    /// **B2 and B4: the build service's own file name, for an unfinished build
    /// listed from Blender's build service** — the one thing about the entry
    /// the install step carries beside the version, so the machine composes the
    /// address from its own constant base and this bare name. **Absent for a
    /// shop-front channel**, whose file is derived from the version on
    /// Blender's release folders. Added 2026-10-02, the Creator's notes 5 and 7
    /// on 0.1.7.
    pub the_build_services_file: Option<String>,
}

/// **B2: Blender's own build service's machine-readable listing** — read
/// 2026-10-02, HTTP 200, 131 entries, one per file, each carrying `version`,
/// `risk_id`, `platform`, `architecture`, `file_name`, `file_mtime` and more.
/// Label: **observed** (`Preparation/Architecture/Sources/Server
/// Setup.sources.md`, *The build service read again*).
pub const THE_BUILD_SERVICES_LISTING: &str = "https://builder.blender.org/download/daily/?format=json&v=1";

/// **B4: the build service's own base for every file it lists**, a constant of
/// this build that never arrives from anywhere — the base the listing gives
/// every entry's `url`, observed 2026-10-02.
pub const THE_BUILD_SERVICES_DOWNLOADS: &str = "https://cdn.builder.blender.org/download/daily/";

/// **B2 and B4: the one archive form fetched from the build service** — the
/// Linux x86_64 release `.tar.xz`. An entry offered ends with it, and a name the
/// machine is handed is refused unless it does.
pub const THE_BUILD_SERVICES_ARCHIVE_ENDING: &str = "-linux.x86_64-release.tar.xz";

/// **B4: a build-service name the machine will compose an address from** — a
/// bare name ending [`THE_BUILD_SERVICES_ARCHIVE_ENDING`], with no `/`, no
/// space and no `..`, so a constant base and this name can only ever be a file
/// directly under that base. **Anything else is refused before any request.**
pub fn a_bare_build_service_name(name: &str) -> bool {
    name.len() > THE_BUILD_SERVICES_ARCHIVE_ENDING.len()
        && name.ends_with(THE_BUILD_SERVICES_ARCHIVE_ENDING)
        && !name.contains('/')
        && !name.contains('\\')
        && !name.contains("..")
        && !name.chars().any(char::is_whitespace)
}

/// **The version as the install step asks for it**: the version, and — for a
/// build listed from Blender's build service — that file's bare name after one
/// space. **One value carried unchanged** from the desktop's command into the
/// run's own record and down a chain of recoveries (`Server Setup` S4 item 2,
/// Y6b), so a taken-over install fetches the same file the first one asked for.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TheVersionAsked {
    pub version: Version,
    pub the_build_services_file: Option<String>,
}

impl TheVersionAsked {
    /// **Read back from the text the step carries.** A second word that is not
    /// a bare build-service name is not read as one: the text answers nothing
    /// and the step says it was commanded with no version it can use.
    pub fn read(text: &str) -> Option<TheVersionAsked> {
        let mut words = text.split_whitespace();
        let version = Version::parse(words.next()?)?;
        let the_build_services_file = match words.next() {
            None => None,
            Some(name) if a_bare_build_service_name(name) => Some(name.to_string()),
            Some(_) => return None,
        };
        if words.next().is_some() {
            return None;
        }
        Some(TheVersionAsked { version, the_build_services_file })
    }

    /// **The text the step carries**, which [`TheVersionAsked::read`] reads
    /// back.
    pub fn as_carried(&self) -> String {
        let version = format!("{}.{}.{}", self.version.major, self.version.minor, self.version.patch);
        match &self.the_build_services_file {
            Some(name) => format!("{version} {name}"),
            None => version,
        }
    }
}

/// **B2: the shop-front's own package-information endpoint, and the header it
/// is requested with.**
///
/// `https://api.snapcraft.io/v2/snaps/info/blender`, requested with
/// `Snap-Device-Series: 16`. Read **2026-08-19**, HTTP 200. Label:
/// **observed** — `Preparation/Architecture/Sources/Server Setup.sources.md`,
/// shape 2, which B2, B3, B5 and B6a all rest on.
pub const THE_STORES_ENDPOINT: &str = "https://api.snapcraft.io/v2/snaps/info/blender";

/// **B2's request header, from the same reading.**
pub const THE_STORES_REQUIRED_HEADER: (&str, &str) = ("Snap-Device-Series", "16");

/// **B2: the risk marks the two listings carry.** The shop-front's are
/// `stable`, `candidate`, `beta` and `edge`; the build service's `risk_id` is
/// `alpha`, `beta`, `candidate` or `stable`. **Only the shop-front's `stable`
/// and the build service's unfinished kinds are offered**, so `Edge` is read
/// and never offered (2026-10-02, the Creator's notes 5 and 7 on 0.1.7).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Risk {
    Stable,
    Candidate,
    Beta,
    Alpha,
    Edge,
}

impl Risk {
    /// **The unstable kinds, included in the offered list and marked as what
    /// they are**, which is what the spec's *Smoozy shows what Blender
    /// publishes and the user picks* asks for — except where B2's narrowing in
    /// [`the_offered_list`] leaves one out.
    pub fn is_unstable(self) -> bool {
        !matches!(self, Risk::Stable)
    }
}

/// **B2: the offered list is the shop-front's channels at or above the
/// whole-app floor, with their risk marks, sorted by parsed version and never
/// by string — keeping each series' newest finished patch and narrowing
/// unfinished builds only where that series has caught up with them.**
///
/// **B2's narrowing** (`Preparation/Architecture/Server Setup.md` B2; the spec's
/// *From a list Smoozy offers*, Creator-stated 2026-09-27 and widened
/// 2026-09-28): **a channel whose risk is not `stable` is not offered where a
/// `stable` channel in the same answer carries the same major and minor version
/// at the same or a newer version**, compared as parsed components. **Finished
/// patches are narrowed too** (the twentieth notes, note 1, 2026-10-28): a
/// `stable` entry goes only where another `stable` entry of the same major and
/// minor version is strictly newer. Equal finished entries stay for their
/// channel marks. **Only the
/// listing's own fields are read** — the risk and the parsed version — so
/// nothing is read off a track's name or a version's spelling. **What it cannot
/// reach, as B2 states it:** an unfinished build of a series no `stable`
/// channel in this answer carries is still offered, and an entry that could not
/// be read is not in `published` at all, so an unreadable `stable` entry
/// narrows nothing.
pub fn the_offered_list(published: &[PublishedChannel]) -> Vec<PublishedChannel> {
    let a_finished_release_of_its_series_supersedes = |entry: &PublishedChannel| {
        published.iter().any(|finished| {
            finished.risk == Risk::Stable
                && finished.version.major == entry.version.major
                && finished.version.minor == entry.version.minor
                && (finished.version > entry.version
                    || (entry.risk.is_unstable() && finished.version == entry.version))
        })
    };
    let mut offered: Vec<PublishedChannel> = published
        .iter()
        .filter(|channel| channel.version >= THE_FLOOR)
        .filter(|channel| !a_finished_release_of_its_series_supersedes(channel))
        .cloned()
        .collect();
    // **The sort is on version components** (Fact 54).
    offered.sort_by_key(|entry| std::cmp::Reverse(entry.version));
    offered
}

/// **B2: finished releases come from the shop-front and unfinished builds from
/// Blender's own build service** — the Creator's revision of 2026-10-02 of
/// their 2026-08-12 ruling, for unfinished builds alone. *(Until that day this
/// said the list's whole source was the shop-front.)*
pub const UNFINISHED_BUILDS_COME_FROM_BLENDERS_BUILD_SERVICE: bool = true;

/// The shop-front's default track. **Its stable channel need not be newest**:
/// `Server Setup` B3, observed 2026-10-27, carried 5.2.2 while `5.2lts` carried
/// stable 5.2.3.
pub const THE_DEFAULT_TRACK: &str = "latest";

/// **B3: the highest finished version, whichever track publishes it.**
/// Compared as parsed version components; unfinished channels never win.
pub fn the_latest_finished_release(published: &[PublishedChannel]) -> Option<&PublishedChannel> {
    published
        .iter()
        .filter(|channel| channel.risk == Risk::Stable)
        .max_by_key(|channel| channel.version)
}

/// **B4: the fetch derives the folder from the version and stays on Blender's
/// own downloads.**
///
/// Observed 2026-08-19 for 5.2.0, 5.1.2, 5.0.1, 4.5.0 through 4.5.12, 4.3.1 and
/// 4.2.20 through 4.2.23.
pub fn the_archives_path(version: Version) -> String {
    format!(
        "Blender{}.{}/blender-{}.{}.{}-linux-x64.tar.xz",
        version.major, version.minor, version.major, version.minor, version.patch
    )
}

/// **B4: the checksum file beside it in the same folder.**
pub fn the_checksums_path(version: Version) -> String {
    format!(
        "Blender{}.{}/blender-{}.{}.{}.sha256",
        version.major, version.minor, version.major, version.minor, version.patch
    )
}

/// **B4: the fetch runs on the machine, over the machine's own line.**
///
/// Fact 54: **37 MB/s on the machine's own line**, against **1.2 to 1.3 MB/s
/// up** from the user's connection (Fact 25, one home line on one evening) and
/// **0.62 MB/s up** on the later reading of that line (`Server protocol and
/// transfers` R17, 2026-08-22). **At the later reading the machine's line is
/// sixty times the user's rather than thirty.**
pub const THE_FETCH_RUNS_ON_THE_MACHINE: bool = true;

/// **B4 and S7's fetch bullet: this fetch is not a `Server protocol and
/// transfers` §7 transfer, and the mechanism is not borrowed.**
///
/// The bytes never pass through Smoozy's own byte loop, and that loop is where
/// §7's checkpoint digests come from (S22 — *"there is no second loop on the
/// machine for this digest to be computed in"*), with the checkpoint record
/// itself kept per transfer on the desktop side (S9, S12). **Neither exists
/// here, so §7's prefix verification is not available and is not claimed.**
///
/// **What binds is §7.1's four contract clauses, cited rather than restated.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WhatTheFetchHonours {
    /// The archive is staged and **never at a final name**.
    pub staged_and_never_at_a_final_name: bool,
    /// **The whole is verified against the published checksum before anything
    /// is unpacked.**
    pub the_whole_is_verified_before_anything_is_unpacked: bool,
    /// **A resume is by byte range into the same staging path** — the downloads
    /// honour byte ranges at `http_code=206` — **with the published checksum as
    /// the only verification**, a whole-file checksum being unable to verify a
    /// prefix.
    pub a_resume_is_by_byte_range_into_the_same_staging_path: bool,
    /// **On a mismatch the staged material is abandoned and the fetch starts
    /// over into fresh staging, never repaired in place** (S24).
    pub a_mismatch_abandons_rather_than_repairs: bool,
}

/// **B4's contract, performed. Either way nothing half-fetched is ever
/// installed.**
pub const WHAT_THE_FETCH_HONOURS: WhatTheFetchHonours = WhatTheFetchHonours {
    staged_and_never_at_a_final_name: true,
    the_whole_is_verified_before_anything_is_unpacked: true,
    a_resume_is_by_byte_range_into_the_same_staging_path: true,
    a_mismatch_abandons_rather_than_repairs: true,
};

/// **B5: Fact 52 is posed by reading, 2026-08-19, and the disagreement is real
/// but confined to the unstable kinds on the day read.**
///
/// Every stable channel checked had its exact file at the derived address; **the
/// candidate and edge channels named versions with no file and no folder** —
/// 5.2.1 answered 404 and 5.3.0 has no release folder at all. **So the list can
/// name a release Smoozy then cannot fetch**, exactly as the spec allows for.
/// Label: **observed**.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhenTheDerivedAddressHasNoFile {
    /// **Smoozy says the build could not be fetched and names it, and never
    /// quietly substitutes a different version.**
    SayItAndNameTheVersion,
}

/// **B5, as the question a builder would otherwise answer wrongly.**
pub fn a_different_version_may_be_substituted(_the_named_one_is_missing: bool) -> bool {
    false
}

/// **B6: blender.org's own build service publishes a machine-readable listing,
/// and daily builds sit on it** (Fact 53, posed by reading 2026-08-19, read
/// again 2026-10-02).
///
/// **From 2026-10-02 B2 lists unfinished builds from it and B4 fetches them
/// from it**, on the Creator's revision of that day.
///
/// **One trap carried**: the listing held **one entry two years stale** beside
/// the current ones, so whatever reads it reads the file moment and keeps only
/// the newest entry per version and kind — `smoozy-server-setup`'s listing
/// reader does.
pub const THE_BUILDERS_DAILY_LISTING_IS_READ_FOR_THE_RECORD: bool = true;

/// **B6a: what the offered list matches against on the machine, and what the
/// mark may claim.**
///
/// **A published entry is a channel, not a build**, and a machine's build is a
/// path with an answered version — **so the match is a version comparison and
/// can never be a build comparison**, which `search` in `smoozy-server-setup` B13 states
/// outright: *"A version number does not identify a build."*
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TheMark {
    /// The entry's current version is present on the machine.
    ThisVersionIsPresent,
    /// **The machine holds a build of the same release line at a different
    /// version**, and the entry is marked with **the version that is present** —
    /// which is what the mockup's *4.5.9 installed* beside a 4.5.10 entry draws.
    AnotherVersionOfThisLineIsPresent(Version),
    /// Neither.
    Unmarked,
}

/// **B6a, performed over the machine's answered versions.**
pub fn the_mark_for(entry: &PublishedChannel, answered_on_the_machine: &[Version]) -> TheMark {
    if answered_on_the_machine.contains(&entry.version) {
        return TheMark::ThisVersionIsPresent;
    }
    match answered_on_the_machine.iter().find(|version| {
        version.major == entry.version.major && version.minor == entry.version.minor
    }) {
        Some(version) => TheMark::AnotherVersionOfThisLineIsPresent(*version),
        None => TheMark::Unmarked,
    }
}

/// **B6a: nothing acts on the mark and that is what makes the imprecision
/// safe.**
///
/// Installing proceeds whatever the mark says, `search` in `smoozy-server-setup` B8 never
/// publishes over an existing folder, and the removal warning is composed from
/// recorded **paths** rather than from versions. **The mark is a reading for the
/// user's eye and no decision anywhere reads it.**
pub fn anything_decides_on_the_mark(_mark: TheMark) -> bool {
    false
}

/// **B6a, struck 2026-10-02: the list's unfinished builds come from the build
/// service's daily listing** (B2), on the Creator's revision of that day. The
/// mark is the same version comparison for them. *(Until that day this said the
/// offered list holds no daily build.)*
pub const THE_OFFERED_LIST_HOLDS_NO_DAILY_BUILD: bool = false;

/// **B7: Row 93 is `Server protocol and transfers` §7 and is cited, not
/// re-solved. Three clauses.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhatABuildTheUserPointsAtGoesThrough {
    /// **1. The transfer** is the staging-and-publication contract in both
    /// directions: part-arrived bytes never sit at the final name, integrity
    /// and resume are measured against staged material, **publication is the
    /// last step**. That closes the row's own observation — a killed copy
    /// leaving a piece under the final name with no marker — and the
    /// one-space-three-tools breakage, because **the staging path is synthesised
    /// from the transfer identity and no tool ever names it**
    /// (`Server protocol and transfers` S4, `Server protocol and transfers` S5).
    TheStagingAndPublicationContract,
    /// **2. A build that arrived short cannot look whole**: the digest is
    /// computed in Smoozy's own byte loop as the bytes pass and compared before
    /// publication (S22, S24). **Only then is the archive unpacked.**
    TheDigestComputedInSmoozysOwnByteLoop,
    /// **3. The unpack uses the one tool Blender's documentation names and
    /// nothing else.** The failure is silent in both directions — the wrong
    /// tool reported success and the result would not start — **so no exit
    /// status is read as the unpack having worked**; the build is then asked its
    /// version under B1, which is owed anyway and is what catches a truncated or
    /// wrongly-unpacked archive.
    TheOneToolAndTheVersionAskThatCatchesIt,
}

impl WhatABuildTheUserPointsAtGoesThrough {
    pub const THE_THREE: [WhatABuildTheUserPointsAtGoesThrough; 3] = [
        WhatABuildTheUserPointsAtGoesThrough::TheStagingAndPublicationContract,
        WhatABuildTheUserPointsAtGoesThrough::TheDigestComputedInSmoozysOwnByteLoop,
        WhatABuildTheUserPointsAtGoesThrough::TheOneToolAndTheVersionAskThatCatchesIt,
    ];
}

/// **B7 clause 3: no exit status is read as the unpack having worked.**
pub fn an_unpacks_exit_status_says_it_worked(_it_was_zero: bool) -> bool {
    false
}

/// **B7a: the below-floor answer holds the publication for a human, and a
/// decline ends the run completed with nothing published.**
///
/// **The warning can arise on this route only.** [`the_offered_list`] is floored
/// at 4.5, and a build the ready action fetched must answer the expected version
/// or fail the step — **so no ending of `statements` in `smoozy-server-setup` W1's is ever owed
/// to the warning.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhatTheUserAnswered {
    InstallAnyway,
    Decline,
}

/// **B7a, performed: what a below-floor answer does to the run.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhatTheBelowFloorAnswerDoes {
    /// The version clears the floor and nothing is asked.
    NothingIsAskedAndThePublicationProceeds,
    /// **The hold is [`crate::driving`] Y7's driving shape and no new
    /// mechanism.** The rename is not issued until the answer clears the floor
    /// or the user says install anyway; **the refresh is not a step command and
    /// continues at its cadence while the question stands**, so the command-gap
    /// window does not end a run held on a human answer. **The build sits in
    /// staging at no final name throughout**, which is what makes the decline
    /// possible at all.
    TheRunHoldsForAHumanAnswer,
}

/// **B7a, performed.**
pub fn what_a_version_answer_does(answered: Version) -> WhatTheBelowFloorAnswerDoes {
    if answered >= THE_FLOOR {
        WhatTheBelowFloorAnswerDoes::NothingIsAskedAndThePublicationProceeds
    } else {
        WhatTheBelowFloorAnswerDoes::TheRunHoldsForAHumanAnswer
    }
}

/// **B7a: *decline* is a step command, and the declined run completes rather
/// than stopping.**
///
/// The desktop commands the staged material's discard; the worker abandons it in
/// S24's shape — never published, never repaired in place — and B8's area never
/// holds it. **The decline revises what the run contains**, the one act that
/// changes [`crate::run`] Y4's composed list after run start: **the publication
/// step is struck and the discard recorded in its place with the reason**, so
/// Y3's boundary meaning holds against the revised list and a reading that meets
/// the record later names what happened rather than meeting a list that quietly
/// shrank.
///
/// **The run then proceeds to *done*** — the final verification over what
/// remains, the machine-level state cleared at *done* as ever, reconciliation
/// run. **[`crate::recovery`] S6's clearing set stays closed at two; this is its
/// first clear, reached by the run finishing.** Work already at boundaries stays
/// — **a tool installed is installed.**
pub fn a_decline_revises_the_composed_list(steps: &[crate::ready_action::Step]) -> Vec<crate::ready_action::Step> {
    steps
        .iter()
        .copied()
        .filter(|step| *step != crate::ready_action::Step::BlenderPublication)
        .collect()
}

/// **B7a: not Row 113's stop.**
///
/// A stop leaves a resume owed, and a resume of this run would re-ask and
/// re-offer; **a declined build must not be publishable by any later
/// resumption**, and the completed run with the discard recorded is what forbids
/// it.
pub fn a_declined_build_may_be_published_by_a_later_resumption(_declined: bool) -> bool {
    false
}

/// **B7a: a user who never answers leaves what a lost connection leaves.**
///
/// The refresh stops when the sitting does, the window elapses, and the worker
/// ends the run at its boundary with the kind's own *ended at boundary N, not
/// completed*; **staged material is kept and swept under
/// the cleanup declaration in `declarations`, in `smoozy-server-setup`.** **A resumed run cannot slip
/// past the question**: the version ask is owed before any rename, so the
/// recovery re-runs the step after the marker, asks afresh, and the warning
/// fires again on the fresh answer.
pub const A_RESUMED_RUN_CANNOT_SLIP_PAST_THE_QUESTION: bool = true;

/// **B7b: what *Builds 10* adds to the unpacking route, and the two clauses it
/// does not need to add.**
///
/// The item that routed this asked for one clause naming the `xz` pre-condition
/// as a machine requirement rather than an unpack outcome, and one naming the
/// library gap as separate from a successful unpack. **Both already exist and
/// neither is rewritten**: [`crate::ready_action`] R1 item 3 is the tooling step
/// and item 4 is the libraries step.
///
/// **And the 7-zip half is now settled version-wide**, which B7 clause 3
/// anticipated and does not need widening for: two passes over each archive
/// reported success and exited cleanly in all four versions; **the runnable mark
/// is gone and every symbolic link has become a plain file** — the tar copies
/// hold 71, 75, 77 and 81 links and the 7-zip copies hold none — and **putting
/// the mark back by hand is not enough**, the program then failing on one of its
/// own libraries. **Clause 3's rule that no exit status is read as the unpack
/// having worked is confirmed by this**, and B1's version ask is the check that
/// catches it.
pub const THE_SEVEN_ZIP_HALF_IS_SETTLED_VERSION_WIDE: bool = true;

/// **B8: Smoozy's own installs go into one Blender area on the same mount as
/// the root, one folder per install, and publication is the last step.**
///
/// The build is fetched and unpacked in staging and **renamed whole into its
/// folder — never over an existing folder, never over one a job is using** (the
/// copy-over-a-running-folder kill is the observed reason; **deleting is
/// harmless, copying over is not**).
pub fn the_publication_may_go_over_an_existing_folder(_it_exists: bool) -> bool {
    false
}

/// **B8: the folder's name is the answered version plus the install's date and
/// time, for a human reader; nothing anywhere parses it back** —
/// `blender-5.1.1-20261006-153212`.
///
/// **The date and time are the setup run's start**, as its record holds it, so a
/// recovery that takes the run over composes the same name and finds the build
/// where it was published (B8, 2026-10-08). **They are read in UTC, to the whole
/// second**: the record keeps whole seconds and a machine's own time zone is
/// nothing Smoozy reads. Until 2026-10-08 the name began with the run's
/// identity, a random string the Creator asked to be rid of (third notes on the
/// test copy, note 4); folders already on machines keep that name and still
/// work, because what a machine holds is read by looking (`search` in
/// `smoozy-server-setup` B10), **never from names**.
///
/// **Two installs of one version starting in the same second compose one name**,
/// and the second's publication is refused, because B8 never publishes over an
/// existing folder ([`the_publication_may_go_over_an_existing_folder`]).
pub fn the_folders_name(answered: Version, the_run_started: std::time::SystemTime) -> String {
    let seconds = the_run_started
        .duration_since(std::time::UNIX_EPOCH)
        .map(|since| since.as_secs())
        .unwrap_or(0);
    let (year, month, day) = the_civil_date(seconds / 86_400);
    let in_the_day = seconds % 86_400;
    format!(
        "blender-{}.{}.{}-{year:04}{month:02}{day:02}-{:02}{:02}{:02}",
        answered.major,
        answered.minor,
        answered.patch,
        in_the_day / 3600,
        in_the_day % 3600 / 60,
        in_the_day % 60
    )
}

/// The calendar date of a count of days since 1970-01-01, in the proleptic
/// Gregorian calendar (the well-known days-to-civil conversion).
fn the_civil_date(days: u64) -> (u64, u64, u64) {
    let z = days + 719_468;
    let era = z / 146_097;
    let day_of_era = z % 146_097;
    let year_of_era =
        (day_of_era - day_of_era / 1460 + day_of_era / 36_524 - day_of_era / 146_096) / 365;
    let day_of_year = day_of_era - (365 * year_of_era + year_of_era / 4 - year_of_era / 100);
    let shifted_month = (5 * day_of_year + 2) / 153;
    let day = day_of_year - (153 * shifted_month + 2) / 5 + 1;
    let month = if shifted_month < 10 { shifted_month + 3 } else { shifted_month - 9 };
    let year = year_of_era + era * 400 + u64::from(month <= 2);
    (year, month, day)
}

/// **B8, as the question a builder would otherwise answer wrongly.**
pub fn a_folder_name_is_parsed_back_for_a_version(_name: &str) -> Option<Version> {
    None
}

/// **B8a: which of three ways a build got onto the machine is readable, and it
/// is read from a record rather than from a name.**
///
/// **The origin is not a property of the build and never can be.** B1 asks the
/// build and the build answers a version and two marks; **it does not answer
/// where it came from.** B8 forbids parsing the folder name back, and
/// `search` in `smoozy-server-setup` B12 and Row 104 forbid remembering anything across
/// connections. **So the reading has to come from a record of the act that
/// placed it, and one already exists.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HowTheBuildGotHere {
    /// **Smoozy placed it from the published listing** (B2 to B4) — no tag.
    ThePublishedListing,
    /// **Smoozy placed it from a build the user pointed at** (B7) — the
    /// design's *your build*.
    ABuildTheUserPointedAt,
    /// **No install record names this path, which is a build Smoozy did not
    /// place at all** — an apt or snap build the search finds (Fact 45), or one
    /// installed before this rule. **The third is real and common and the design
    /// has no tag for it**; what it is called, and whether it is called
    /// anything, is Design's.
    SmoozyDidNotPlaceIt,
}

/// One install operation record, as B8a reads it.
///
/// **A Blender install is an operation, its operation record is durable, and its
/// subject is the published build.** `Rented-server runtime` §13 R1 already
/// retains an operation record *"the same moment as their subject's machine-side
/// record"* — **so naming the subject is the whole of this clause**: the record
/// stands for as long as the build stands and ends when the build is removed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AnInstallRecord {
    /// **The join is by executable path**, which is `search` in `smoozy-server-setup` B13's own
    /// identifier and the only one this document trusts.
    pub executable_path: String,
    /// **Which of B5.1's two routes placed the build — and nothing else about
    /// it.** No name, no channel, no provenance beyond the route.
    pub route: HowTheBuildGotHere,
}

/// **B8a, performed: the origin of a build the search found.**
///
/// **Nothing is parsed and nothing is remembered on the desktop.**
pub fn how_it_got_here(executable_path: &str, records: &[AnInstallRecord]) -> HowTheBuildGotHere {
    records
        .iter()
        .find(|record| record.executable_path == executable_path)
        .map(|record| record.route)
        .unwrap_or(HowTheBuildGotHere::SmoozyDidNotPlaceIt)
}

/// **B8a: a build removed and re-installed gets a fresh record**, because B8
/// never publishes over an existing folder and the removal ends the old subject.
pub const A_REINSTALL_GETS_A_FRESH_RECORD: bool = true;

/// **B4's and R1 item 4's timings, carried so a duration shown to the user is
/// not budgeted as a transfer.**
pub const THE_UNPACK_ON_FOUR_CORES: (Duration, Duration) =
    (Duration::from_secs(62), Duration::from_secs(67));

// ─────────────── B15's second route: a build the machine's own manager placed

/// **B15's manager route, and the one package it may name: the Snap Store's
/// `blender`.** Added 2026-10-16, step 10c's owed piece 6.
///
/// **The name is fixed at build time**, because `Server protocol and transfers`
/// 1.1 forbids echoing a machine-read name into a command string, and §13 item
/// 21 says a managed build under any other name has no admissible removal
/// carriage. **It is the name B2 reads the store by** ([`THE_STORES_ENDPOINT`]),
/// and the Blender research quotes the manual's own install line for it,
/// *snap install blender --classic* (Findings 5.1, *Builds*). Label:
/// **documented**, and **observed** on the rented machine of 2026-08-06, where
/// `snap list blender` named it.
///
/// **The distribution's own package is not here, and that part is stopped
/// rather than guessed.** §13 item 21 names it, and the research read
/// `apt-cache policy blender` on a rented machine — but **no plan names the
/// removal command for it**, and **nothing tells its `/usr/bin/blender` from a
/// link someone made at that same path**: the research's own rented machine
/// held exactly such a link, to a hand-unpacked folder, with no package
/// installed at all (Findings 5.2, *Builds*, hand test of 2026-08-06). A row at
/// that path keeps the answer that the act is not built.
pub const THE_SNAPS_PACKAGE: &str = "blender";

/// **Where the Snap Store's `blender` was seen on a machine: its launcher on
/// the command path.** Label: **observed** — the six-machine batch's Fact 45
/// (`/snap/bin/blender` resolves to the snap launcher, which B10 carries), and
/// the research's hand test of 2026-08-06, where `which -a blender` printed
/// this path once `snap install blender --classic` had run.
pub const THE_SNAPS_LAUNCHER: &str = "/snap/bin/blender";

/// **The snap's own folder, the read-only mount B15 cites.** Label:
/// **observed** — the same hand test read `bpy.app.binary_path` as
/// `/snap/blender/7599/blender` for the snap build. **That the revision is one
/// folder of any name is an assumption**: one revision was seen. **What breaks
/// if it is wrong** is the safe direction — a snap build laid out otherwise is
/// not recognised, and its row keeps the answer that the act is not built.
pub const THE_SNAPS_OWN_FOLDER: &str = "/snap/blender/";

/// **B15: the package a build's executable belongs to, where it is a manager's
/// this module may remove**, and nothing everywhere else.
///
/// **Two paths, both the snap's**: [`THE_SNAPS_LAUNCHER`] exactly, or
/// `blender` one folder under [`THE_SNAPS_OWN_FOLDER`]. **Nothing is read
/// off the machine for this**: B10's search hands over a path, and this is
/// Smoozy's own comparison of that text against two observed shapes. A path
/// with `.` or `..` in it is no package's.
pub fn the_managed_package_at(executable: &str) -> Option<&'static str> {
    if executable == THE_SNAPS_LAUNCHER {
        return Some(THE_SNAPS_PACKAGE);
    }
    let revision = executable
        .strip_prefix(THE_SNAPS_OWN_FOLDER)?
        .strip_suffix("/blender")?;
    let one_plain_folder =
        !revision.is_empty() && revision != "." && revision != ".." && !revision.contains('/');
    one_plain_folder.then_some(THE_SNAPS_PACKAGE)
}

/// **§13 item 21 and B15, held on the machine too: a manager's removal is
/// recorded only for the fixed package, and only for a path that package
/// holds.**
///
/// The desktop never asks for another — but the step is the machine's record of
/// a removal and the marks ride it (`Queue & History` L21), so a name or a path
/// outside the pairing is refused here and nothing is marked.
pub fn a_managers_removal_may_be_recorded(package: &str, executable: &str) -> Result<(), String> {
    if package != THE_SNAPS_PACKAGE {
        return Err(format!("Removing the package {package:?} is not supported."));
    }
    match the_managed_package_at(executable) {
        Some(holds) if holds == package => Ok(()),
        _ => Err(format!(
            "{executable} is not part of the package {package:?}. Removal not recorded."
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// **B15's second route: the two observed snap shapes are the snap's
    /// `blender`, and nothing else is any package's** (step 10c's owed piece
    /// 6). The distribution's `/usr/bin/blender` is deliberately none: a link
    /// at that path to a hand-unpacked folder was seen on a rented machine.
    #[test]
    fn only_the_two_observed_snap_shapes_are_a_managed_package() {
        assert_eq!(the_managed_package_at("/snap/bin/blender"), Some("blender"));
        assert_eq!(the_managed_package_at("/snap/blender/7599/blender"), Some("blender"));
        assert_eq!(the_managed_package_at("/snap/blender/current/blender"), Some("blender"));
        for not_one in [
            "/usr/bin/blender",
            "/bin/blender",
            "/snap/bin/blender-launcher",
            "/snap/blender/blender",
            "/snap/blender/7599/5.2/blender",
            "/snap/blender/../bin/blender",
            "/snap/blender/./blender",
            "/snap/other/7/blender",
            "/opt/blender/blender",
        ] {
            assert_eq!(the_managed_package_at(not_one), None, "{not_one}");
        }
        assert!(a_managers_removal_may_be_recorded("blender", "/snap/bin/blender").is_ok());
        assert!(a_managers_removal_may_be_recorded("blender-bin", "/snap/bin/blender").is_err());
        assert!(a_managers_removal_may_be_recorded("blender", "/usr/bin/blender").is_err());
    }

    fn a_channel(track: &str, risk: Risk, version: &str) -> PublishedChannel {
        PublishedChannel {
            track: track.to_string(),
            risk,
            version: Version::parse(version).expect("a version"),
            released_at: "2026-08-25T13:56:59.398880+00:00".to_string(),
            size_bytes: 377_902_364,
            the_build_services_file: None,
        }
    }

    #[test]
    fn the_version_is_read_off_the_blender_line_and_never_off_the_first_one() {
        // Fact 45: the apt build printed two lines of colour-management noise
        // first.
        let output = "Color management: using fallback mode\nColor management: sRGB\nBlender 4.5.9\n";
        assert_eq!(read_the_version_answer(output), Version::parse("4.5.9"));
        // And a run that answered nothing answers nothing.
        assert_eq!(read_the_version_answer("no such file\n"), None);
        assert_eq!(a_killed_run_answers(true), None);
    }

    #[test]
    fn the_sort_is_on_version_components_and_never_on_the_string() {
        // Fact 54: a plain alphabetical sort picks 4.2.9 as newest where 4.2.23
        // exists.
        assert!(Version::parse("4.2.23").unwrap() > Version::parse("4.2.9").unwrap());
        let published = [
            a_channel("4.5lts", Risk::Stable, "4.5.9"),
            a_channel("latest", Risk::Stable, "5.2.0"),
            a_channel("4.5lts", Risk::Stable, "4.5.23"),
        ];
        let offered = the_offered_list(&published);
        assert_eq!(offered[0].version, Version::parse("5.2.0").unwrap());
        assert_eq!(offered[1].version, Version::parse("4.5.23").unwrap());
    }

    #[test]
    fn the_offered_list_is_floored_at_the_whole_apps_floor() {
        let published = [
            a_channel("4.2", Risk::Stable, "4.2.23"),
            a_channel("latest", Risk::Stable, "5.2.0"),
        ];
        let offered = the_offered_list(&published);
        assert_eq!(offered.len(), 1);
        assert!(offered.iter().all(|entry| entry.version >= THE_FLOOR));
    }

    #[test]
    fn only_the_newest_finished_patch_of_a_line_is_offered() {
        let published = [
            a_channel("latest", Risk::Stable, "5.2.2"),
            a_channel("5.2lts", Risk::Stable, "5.2.3"),
        ];
        assert_eq!(the_offered_list(&published), vec![published[1].clone()]);
        let reversed = [published[1].clone(), published[0].clone()];
        assert_eq!(the_offered_list(&reversed), vec![published[1].clone()]);
    }

    #[test]
    fn each_line_keeps_its_newest_finished_patch_and_its_equal_channels() {
        let published = [
            a_channel("4.5lts", Risk::Stable, "4.5.9"),
            a_channel("latest", Risk::Stable, "5.2.3"),
            a_channel("4.5lts", Risk::Stable, "4.5.23"),
            a_channel("5.1", Risk::Stable, "5.1.2"),
            a_channel("5.2lts", Risk::Stable, "5.2.10"),
            a_channel("5.1", Risk::Stable, "5.1.0"),
            a_channel("latest", Risk::Stable, "5.2.10"),
        ];
        assert_eq!(
            the_offered_list(&published),
            vec![published[4].clone(), published[6].clone(), published[3].clone(), published[2].clone()]
        );
    }

    #[test]
    fn narrowing_finished_patches_leaves_unfinished_lines_as_before() {
        let published = [
            a_channel("latest", Risk::Stable, "5.2.2"),
            a_channel("5.2lts", Risk::Stable, "5.2.3"),
            a_channel("", Risk::Alpha, "5.3.0"),
            a_channel("", Risk::Beta, "5.3.1"),
            a_channel("", Risk::Candidate, "5.2.4"),
            a_channel("", Risk::Candidate, "5.2.3"),
        ];
        assert_eq!(
            the_offered_list(&published),
            vec![published[3].clone(), published[2].clone(), published[4].clone(), published[1].clone()]
        );
    }

    #[test]
    fn narrowing_finished_patches_does_not_change_the_quick_paths_choice() {
        let published = [
            a_channel("latest", Risk::Stable, "5.2.2"),
            a_channel("5.2lts", Risk::Stable, "5.2.3"),
            a_channel("4.5lts", Risk::Stable, "4.5.23"),
            a_channel("", Risk::Beta, "5.3.0"),
        ];
        let chosen = the_latest_finished_release(&published).unwrap().clone();
        assert_eq!(chosen, published[1]);
        let offered = the_offered_list(&published);
        assert_eq!(the_latest_finished_release(&offered), Some(&chosen));
        assert_eq!(the_latest_finished_release(&published), Some(&chosen));
    }

    #[test]
    fn the_unstable_kinds_are_offered_and_marked_rather_than_hidden() {
        let published = [
            a_channel("", Risk::Alpha, "5.3.0"),
            a_channel("latest", Risk::Stable, "5.2.0"),
        ];
        assert_eq!(the_offered_list(&published).len(), 2);
        assert!(Risk::Alpha.is_unstable());
        assert!(Risk::Edge.is_unstable());
        assert!(Risk::Candidate.is_unstable());
        assert!(!Risk::Stable.is_unstable());
    }

    /// **B4: the version as asked carries a build-service file only as a bare
    /// name of the one archive form, and reads back to what was carried** —
    /// the Creator's notes 5 and 7 on 0.1.7. **A name holding a `/`, a space or
    /// `..`, or of any other form, is refused**: the machine is never handed an
    /// address.
    #[test]
    fn the_version_asked_carries_a_bare_build_service_name_and_nothing_else() {
        let alpha = "blender-5.3.0-alpha+main.64d181010e0d-linux.x86_64-release.tar.xz";
        let asked = TheVersionAsked::read(&format!("5.3.0 {alpha}")).expect("it reads");
        assert_eq!(asked.version, Version::parse("5.3.0").unwrap());
        assert_eq!(asked.the_build_services_file.as_deref(), Some(alpha));
        assert_eq!(asked.as_carried(), format!("5.3.0 {alpha}"));
        assert_eq!(TheVersionAsked::read("5.2.2").map(|it| it.as_carried()).as_deref(), Some("5.2.2"));
        for refused in [
            "https://example.com/blender-5.3.0-alpha-linux.x86_64-release.tar.xz",
            "../blender-5.3.0-alpha-linux.x86_64-release.tar.xz",
            "daily/blender-5.3.0-alpha-linux.x86_64-release.tar.xz",
            "blender-5.3.0-alpha+main.64d181010e0d-windows.amd64-release.zip",
            "-linux.x86_64-release.tar.xz",
        ] {
            assert!(!a_bare_build_service_name(refused), "{refused}");
            assert_eq!(TheVersionAsked::read(&format!("5.3.0 {refused}")), None, "{refused}");
        }
        assert!(a_bare_build_service_name(alpha));
        assert_eq!(TheVersionAsked::read(&format!("5.3.0 {alpha} more")), None);
        assert_eq!(TheVersionAsked::read("not a version"), None);
    }

    /// **B2's narrowing, at its edges: an unfinished build goes where a finished
    /// release of its series is at the same or a newer version, and stays where
    /// it is newer than every finished release of its series** — the drawing's
    /// own *5.2.4 Beta* and *5.2.3 Release candidate* beside a finished 5.2.2.
    /// Finished releases are narrowed separately to their line's newest patch.
    #[test]
    fn an_unfinished_build_goes_only_where_its_series_is_released_at_or_past_it() {
        let published = [
            a_channel("latest", Risk::Stable, "5.2.2"),
            a_channel("latest", Risk::Beta, "5.2.4"),
            a_channel("latest", Risk::Candidate, "5.2.3"),
            a_channel("5.2lts", Risk::Candidate, "5.2.2"),
            a_channel("latest", Risk::Alpha, "5.2.1"),
            a_channel("5.1", Risk::Stable, "5.1.2"),
            a_channel("5.1", Risk::Stable, "5.1.0"),
        ];
        let offered: Vec<(Risk, Version)> = the_offered_list(&published)
            .iter()
            .map(|entry| (entry.risk, entry.version))
            .collect();
        let v = |text: &str| Version::parse(text).unwrap();
        assert_eq!(
            offered,
            vec![
                (Risk::Beta, v("5.2.4")),
                (Risk::Candidate, v("5.2.3")),
                (Risk::Stable, v("5.2.2")),
                (Risk::Stable, v("5.1.2")),
            ]
        );
    }

    #[test]
    fn the_latest_finished_release_is_the_default_tracks_stable_channel() {
        // B3, observed 2026-08-19: `latest/stable` named 5.2.0. A long-term
        // support line that happens to share the version is still the latest
        // finished release.
        let published = [
            a_channel("4.5lts", Risk::Stable, "4.5.23"),
            a_channel("latest", Risk::Candidate, "5.3.0"),
            a_channel("latest", Risk::Stable, "5.2.0"),
        ];
        assert_eq!(
            the_latest_finished_release(&published).map(|it| it.version),
            Version::parse("5.2.0")
        );
    }

    /// B3, the nineteenth notes, note 1: the store's default track can lag.
    #[test]
    fn the_latest_finished_release_can_be_on_a_long_term_support_track() {
        let published = [
            a_channel("latest", Risk::Stable, "5.2.2"),
            a_channel("5.2lts", Risk::Stable, "5.2.3"),
            a_channel("latest", Risk::Candidate, "5.2.3"),
            a_channel("latest", Risk::Beta, "5.3.0"),
            a_channel("latest", Risk::Edge, "5.4.0"),
            a_channel("4.5lts", Risk::Stable, "4.5.23"),
        ];
        assert_eq!(
            the_latest_finished_release(&published).map(|it| it.version),
            Version::parse("5.2.3")
        );
    }

    #[test]
    fn latest_finished_release_compares_numbers_and_needs_a_stable_channel() {
        let published = [
            a_channel("5.2lts", Risk::Stable, "5.2.10"),
            a_channel("latest", Risk::Stable, "5.2.9"),
            a_channel("latest", Risk::Beta, "5.3.0"),
        ];
        assert_eq!(
            the_latest_finished_release(&published).map(|it| it.version),
            Version::parse("5.2.10")
        );
        assert!(the_latest_finished_release(&published[2..]).is_none());
        assert!(the_latest_finished_release(&[]).is_none());
    }

    #[test]
    fn the_archive_and_the_checksum_sit_in_the_same_derived_folder() {
        let version = Version::parse("5.2.0").unwrap();
        assert_eq!(
            the_archives_path(version),
            "Blender5.2/blender-5.2.0-linux-x64.tar.xz"
        );
        assert_eq!(the_checksums_path(version), "Blender5.2/blender-5.2.0.sha256");
    }

    #[test]
    fn a_named_release_with_no_file_is_said_and_never_substituted() {
        // B5, Fact 52: the candidate and edge channels named versions with no
        // file and no folder.
        assert!(!a_different_version_may_be_substituted(true));
        assert_eq!(
            WhenTheDerivedAddressHasNoFile::SayItAndNameTheVersion,
            WhenTheDerivedAddressHasNoFile::SayItAndNameTheVersion
        );
    }

    #[test]
    fn the_mark_claims_a_version_and_nothing_more() {
        // B6a: the match is a version comparison and can never be a build
        // comparison.
        let entry = a_channel("4.5lts", Risk::Stable, "4.5.10");
        assert_eq!(
            the_mark_for(&entry, &[Version::parse("4.5.10").unwrap()]),
            TheMark::ThisVersionIsPresent
        );
        assert_eq!(
            the_mark_for(&entry, &[Version::parse("4.5.9").unwrap()]),
            TheMark::AnotherVersionOfThisLineIsPresent(Version::parse("4.5.9").unwrap())
        );
        assert_eq!(
            the_mark_for(&entry, &[Version::parse("5.2.0").unwrap()]),
            TheMark::Unmarked
        );
        // And no decision anywhere reads it.
        assert!(!anything_decides_on_the_mark(TheMark::ThisVersionIsPresent));
    }

    #[test]
    fn a_below_floor_answer_holds_the_run_for_a_human_and_the_decline_strikes_the_publication() {
        use crate::ready_action::Step;
        assert_eq!(
            what_a_version_answer_does(Version::parse("4.2.23").unwrap()),
            WhatTheBelowFloorAnswerDoes::TheRunHoldsForAHumanAnswer
        );
        assert_eq!(
            what_a_version_answer_does(Version::parse("4.5.0").unwrap()),
            WhatTheBelowFloorAnswerDoes::NothingIsAskedAndThePublicationProceeds
        );
        let revised = a_decline_revises_the_composed_list(&[
            Step::Readings,
            Step::Fetch,
            Step::Tooling,
            Step::BlenderPublication,
            Step::Done,
        ]);
        assert!(!revised.contains(&Step::BlenderPublication));
        // Work already at boundaries stays — a tool installed is installed.
        assert!(revised.contains(&Step::Tooling));
        assert!(revised.contains(&Step::Done));
        assert!(!a_declined_build_may_be_published_by_a_later_resumption(true));
    }

    #[test]
    fn no_exit_status_is_read_as_the_unpack_having_worked() {
        // B7 clause 3, confirmed by B7b's 7-zip passes: two passes reported
        // success and exited cleanly in all four versions, and the result would
        // not start.
        assert!(!an_unpacks_exit_status_says_it_worked(true));
        assert_eq!(WhatABuildTheUserPointsAtGoesThrough::THE_THREE.len(), 3);
        const { assert!(THE_SEVEN_ZIP_HALF_IS_SETTLED_VERSION_WIDE) };
    }

    #[test]
    fn a_folder_name_is_never_parsed_back_and_never_published_over() {
        assert!(!the_publication_may_go_over_an_existing_folder(true));
        let at = |seconds: u64| std::time::UNIX_EPOCH + std::time::Duration::from_secs(seconds);
        let name = the_folders_name(Version::parse("5.2.0").unwrap(), at(1_791_300_732));
        assert_eq!(name, "blender-5.2.0-20261006-153212");
        assert_eq!(a_folder_name_is_parsed_back_for_a_version(&name), None);
    }

    /// **B8, 2026-10-08: the name is the version and the run's start, read
    /// right on every calendar edge** — a leap day's last second, the first
    /// second of a year, and the clock's own beginning.
    #[test]
    fn a_folders_name_reads_the_runs_start_as_a_date_and_time() {
        let at = |seconds: u64| std::time::UNIX_EPOCH + std::time::Duration::from_secs(seconds);
        let v = Version::parse("5.1.1").unwrap();
        assert_eq!(the_folders_name(v, at(1_709_251_199)), "blender-5.1.1-20240229-235959");
        assert_eq!(the_folders_name(v, at(1_709_251_200)), "blender-5.1.1-20240301-000000");
        assert_eq!(the_folders_name(v, at(1_767_225_600)), "blender-5.1.1-20260101-000000");
        assert_eq!(the_folders_name(v, at(0)), "blender-5.1.1-19700101-000000");
        // **A second apart is two names**, and a part of a second is not.
        assert_ne!(the_folders_name(v, at(1_791_300_732)), the_folders_name(v, at(1_791_300_733)));
        assert_eq!(
            the_folders_name(v, at(1_791_300_732)),
            the_folders_name(v, at(1_791_300_732) + std::time::Duration::from_millis(999))
        );
    }

    #[test]
    fn a_build_no_install_record_names_is_one_smoozy_did_not_place() {
        // B8a's third case, which is real and common.
        let records = [AnInstallRecord {
            executable_path: "/smoozy/blender/i-7-blender-5.2.0/blender".to_string(),
            route: HowTheBuildGotHere::ABuildTheUserPointedAt,
        }];
        assert_eq!(
            how_it_got_here("/smoozy/blender/i-7-blender-5.2.0/blender", &records),
            HowTheBuildGotHere::ABuildTheUserPointedAt
        );
        assert_eq!(
            how_it_got_here("/snap/bin/blender", &records),
            HowTheBuildGotHere::SmoozyDidNotPlaceIt
        );
        const { assert!(A_REINSTALL_GETS_A_FRESH_RECORD) };
    }

    #[test]
    fn the_settings_folder_is_two_numbers_so_patch_releases_share_it() {
        assert_eq!(
            Version::parse("4.5.9").unwrap().settings_folder(),
            Version::parse("4.5.23").unwrap().settings_folder()
        );
    }

    #[test]
    fn the_fetch_honours_the_four_contract_clauses_and_claims_no_prefix_verification() {
        const { assert!(WHAT_THE_FETCH_HONOURS.staged_and_never_at_a_final_name) };
        const { assert!(WHAT_THE_FETCH_HONOURS.the_whole_is_verified_before_anything_is_unpacked) };
        const { assert!(WHAT_THE_FETCH_HONOURS.a_resume_is_by_byte_range_into_the_same_staging_path) };
        const { assert!(WHAT_THE_FETCH_HONOURS.a_mismatch_abandons_rather_than_repairs) };
        const { assert!(THE_FETCH_RUNS_ON_THE_MACHINE) };
    }

    #[test]
    fn the_reading_discipline_refuses_the_three_things_it_forbids() {
        assert!(!an_empty_error_stream_says_anything_about_the_window(true));
        assert!(!an_engine_list_is_asked_for_anywhere_here());
        // **The limit is the one the one run on the path actually carries**,
        // rather than a constant asserting a promise nothing kept.
        assert_eq!(THE_TIME_LIMIT_EVERY_BLENDER_RUN_CARRIES, Duration::from_secs(60));
        assert_eq!(
            WhatTheTimeLimitKills::TheInvokedBlenderProcessAndNothingAboveIt,
            WhatTheTimeLimitKills::TheInvokedBlenderProcessAndNothingAboveIt
        );
        // And B1's kill half is recorded unkept rather than claimed.
        const { assert!(B1S_KILL_HALF_IS_UNKEPT_AND_WHAT_BREAKS_IS_A_BLENDER_LEFT_RUNNING) };
        const { assert!(UNFINISHED_BUILDS_COME_FROM_BLENDERS_BUILD_SERVICE) };
        assert_eq!(THE_STORES_REQUIRED_HEADER, ("Snap-Device-Series", "16"));
        assert!(THE_STORES_ENDPOINT.ends_with("/info/blender"));
        const { assert!(THE_BUILDERS_DAILY_LISTING_IS_READ_FOR_THE_RECORD) };
        const { assert!(!THE_OFFERED_LIST_HOLDS_NO_DAILY_BUILD) };
        assert!(THE_BUILD_SERVICES_LISTING.starts_with("https://builder.blender.org/"));
        assert!(THE_BUILD_SERVICES_DOWNLOADS.starts_with("https://cdn.builder.blender.org/"));
        const { assert!(A_RESUMED_RUN_CANNOT_SLIP_PAST_THE_QUESTION) };
        assert_eq!(THE_UNPACK_ON_FOUR_CORES.0, Duration::from_secs(62));
    }
}
