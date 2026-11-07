//! Where Smoozy's own places are, per system — `Desktop app and delivery` L3,
//! L3a, L3b and F11.
//!
//! **F11: this is the only place a location this document names is resolved to
//! a path**, and no path is synthesised from an identity anywhere else. The
//! closed set below is F11's own enumeration, kept by a type rather than by
//! prose: a module that needs a location this list does not hold has found a
//! gap in that section and never a licence to compose one.
//!
//! **L3 and L3a: every reading is the system's own call.** The per-user
//! application-data location is never assembled from the login account name;
//! the user home directory is never assembled from the login account name and
//! never from an environment value.
//!
//! **L3b names the call**, per system, for both, each read from that system's
//! own publication. The citations are in the plan's sources file; what is
//! carried here is the call and the reason it and not a neighbour.
//!
//! **And one thing here is no clause's: the throwaway copy.** A development
//! build can be pointed at a disposable folder from its own command line, so
//! that a copy run at the desk has a record, a home folder and a set of secrets
//! nobody minds losing. It is a development convenience and not something the
//! user gets, so no clause asks for it; what binds it is the clauses it must
//! not break, which are F11, L3, L3a and L3b above. See [`throwaway`].

use std::path::PathBuf;

/// The one folder name Smoozy uses inside every system's per-user
/// application-data location.
///
/// **A Build decision and a small one.** One name on all three systems rather
/// than a company level on Windows and a bundle identifier on macOS: it is one
/// thing for a user to find, one thing to name in a bug report, and one thing
/// for `Security and licence` §5's erasure to point at. L4's rule is not in
/// play — this is Smoozy's own name and not a field of the record.
pub const FOLDER: &str = "Smoozy";

/// Every location the adapter resolves — `Desktop app and delivery` F11's own
/// enumeration, less the downloads folder.
///
/// **The downloads folder is deliberately absent and it is not an oversight.**
/// [Section 8](../../../../Preparation/Architecture/Desktop%20app%20and%20delivery.md)
/// W1 gives it three separate mechanisms rather than three spellings of one,
/// and its only consumer is `Download`. *(Corrected 2026-09-07: this said that
/// module is built at step 11. It was built at step 8.)* **What it consumes is
/// the proposal as a value rather than this enumeration**: that crate's
/// `landing::the_destination_for_this_act` takes §8's route already resolved
/// and composes no path of its own, and it asks nothing of this adapter. A
/// variant that does not exist refuses a caller exactly as F11's missing half
/// says it should, and no caller asks for one.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Location {
    /// L3. The root every place below sits in.
    PerUserApplicationData,
    /// Section 5. The local record's own file.
    LocalRecord,
    /// L2a. `Reporting a problem` §1's diary area, beside the record and never
    /// inside it.
    DiaryArea,
    /// `Security and licence` §7. One file per connection attempt, swept at
    /// application start.
    ThrowawayKnownHostsArea,
    /// L3a. Smoozy's own temporary area — never the operating system's shared
    /// temporary directory, never a place any module scans for keys, swept at
    /// application start.
    TemporaryArea,
    /// `State, identity and recovery` §1.1. The small viewable copies fetched
    /// from a machine.
    FetchedDerivativeArea,
    /// `Server protocol and transfers`. Download material before publication.
    StagedTransferArea,
    /// P2. The file whose lock is the one running copy's hold. **Empty, and
    /// read by nobody**: the lock on it is the whole of what it is for, and
    /// the file's presence says nothing. Added at step 11's R1.
    TheRunningCopysHold,
    /// P2. How a second launch reaches the running copy. On macOS and Linux it
    /// is a named pipe the running copy reads; on Windows it is a file naming
    /// the running copy's process, which the second launch reads. **Neither is
    /// state**: each holder makes it afresh, and only a launch that could not
    /// take the hold ever reads it. Added at step 11's R1.
    TheRunningCopysDoor,
    /// **Z8's installed path: the complete bundle this program is running
    /// from**, read from the system's own report of the running program and
    /// never composed from a name. **macOS only**: elsewhere, and on a Mac where
    /// the program is not inside a bundle, it is not named. Added at step 11's
    /// Mac update piece, with F11's dated note.
    TheInstalledBundle,
    /// **E18's staging place: a folder beside the installed bundle**, so the
    /// staged bundle is on the same filesystem as the one it replaces. Hidden by
    /// its leading dot. Composed from [`Location::TheInstalledBundle`] and from
    /// nothing else.
    TheUpdatesStagingFolder,
    /// **Z7's named backup: where the complete previous bundle is kept** beside
    /// the installed one through Z21's probe. Hidden by its leading dot, and
    /// composed from [`Location::TheInstalledBundle`] alone.
    ThePreviousBundleKept,
    /// L3a. The user's home directory. **A reading and not a place Smoozy
    /// writes** — Microsoft's own instruction for the same folder is that
    /// applications *"should not create files or folders at this level"*, and
    /// `Keys` is the one consumer.
    UserHome,
}

/// Why a location has no path — [W5](#)'s rule, which L3b takes rather than
/// writing a second one: **the adapter says so and composes nothing.**
///
/// It does not fall back to a path built out of a name, and it does not create
/// a folder the system did not name.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NotNamed {
    /// Which location could not be answered.
    pub location: Location,
    /// What the system said, in the system's own terms, for the diary and for
    /// a failure record. **Never shown to the user as it stands** — `Desktop
    /// app and delivery` §16.3 owns how a machine's own words reach a screen.
    pub reported: String,
}

impl NotNamed {
    fn new(location: Location, reported: impl Into<String>) -> Self {
        NotNamed {
            location,
            reported: reported.into(),
        }
    }
}

/// Resolve one location to a path.
///
/// F11: the only route. Every caller in the application arrives here, and the
/// two readings the system answers — the application-data root and the home
/// directory — are taken fresh rather than remembered, because neither is
/// state and `State, identity and recovery` §1.3 rule 5 forbids reading a
/// place as one.
pub fn resolve(location: Location) -> Result<PathBuf, NotNamed> {
    if location == Location::UserHome {
        return the_user_home();
    }
    if matches!(
        location,
        Location::TheInstalledBundle | Location::TheUpdatesStagingFolder | Location::ThePreviousBundleKept
    ) {
        let installed = the_running_bundle()
            .ok_or_else(|| NotNamed::new(location, "this program is not running from a bundle"))?;
        return Ok(beside_the_installed_bundle(location, &installed));
    }
    Ok(the_area(location, the_application_data_root()?))
}

/// **The bundle the running program sits in**, read from the system's own
/// report of the running program (`std::env::current_exe`): its folder must be
/// `MacOS`, inside `Contents`, inside a folder whose name ends `.app`. Anything
/// else names nothing. **macOS only.**
fn the_running_bundle() -> Option<PathBuf> {
    if !cfg!(target_os = "macos") {
        return None;
    }
    let program = std::env::current_exe().ok()?;
    the_bundle_a_program_sits_in(&program)
}

/// The same reading over a path handed in, so it can be held at the desk.
pub fn the_bundle_a_program_sits_in(program: &std::path::Path) -> Option<PathBuf> {
    let macos = program.parent()?;
    let contents = macos.parent()?;
    let bundle = contents.parent()?;
    let named = |path: &std::path::Path, name: &str| path.file_name().is_some_and(|it| it == name);
    let is_a_bundle = bundle
        .file_name()
        .and_then(|it| it.to_str())
        .is_some_and(|it| it.ends_with(".app") && it.len() > 4);
    (named(macos, "MacOS") && named(contents, "Contents") && is_a_bundle).then(|| bundle.to_path_buf())
}

/// **The two places beside the installed bundle**, composed in one function so
/// a test can ask it of any bundle — the same shape as [`the_area`].
pub fn beside_the_installed_bundle(location: Location, installed: &std::path::Path) -> PathBuf {
    let stem = installed
        .file_stem()
        .map(|it| it.to_string_lossy().into_owned())
        .unwrap_or_else(|| FOLDER.to_string());
    match location {
        Location::TheUpdatesStagingFolder => installed.with_file_name(format!(".{stem} update")),
        Location::ThePreviousBundleKept => installed.with_file_name(format!(".{stem} previous.app")),
        _ => installed.to_path_buf(),
    }
}

/// Where each area sits under the application-data root — **the composition
/// F11 names, in one function so a test can ask it of any root.**
///
/// **The two replacements the throwaway setting makes are the root and the home
/// directory, and neither is here**: this says where an area sits under
/// whichever root the copy has, which is the same for a real copy and a
/// throwaway one.
fn the_area(location: Location, root: PathBuf) -> PathBuf {
    match location {
        // **The home directory is a reading and is never composed from this
        // root** (L3a). It is answered before this function is reached.
        Location::UserHome | Location::PerUserApplicationData => root,
        Location::LocalRecord => root.join("local-record.sqlite3"),
        Location::DiaryArea => root.join("diary"),
        Location::ThrowawayKnownHostsArea => root.join("known-hosts"),
        Location::TemporaryArea => root.join("temporary"),
        Location::FetchedDerivativeArea => root.join("previews"),
        Location::StagedTransferArea => root.join("staging"),
        Location::TheRunningCopysHold => root.join("running-copy.lock"),
        Location::TheRunningCopysDoor => root.join("running-copy.door"),
        // **Never under this root**: [`resolve`] answers these three from the
        // running bundle before this function is reached, and nothing else
        // calls it with them.
        Location::TheInstalledBundle
        | Location::TheUpdatesStagingFolder
        | Location::ThePreviousBundleKept => root,
    }
}

/// The root every area sits under — **the first of the throwaway setting's two
/// replacements, and it is inside F11's one funnel.**
fn the_application_data_root() -> Result<PathBuf, NotNamed> {
    #[cfg(debug_assertions)]
    if let Some(folder) = the_throwaway_folder() {
        return Ok(throwaway::the_application_data_root_under(folder));
    }
    per_user_application_data()
}

/// The user's home directory — **the second replacement, in the same funnel.**
fn the_user_home() -> Result<PathBuf, NotNamed> {
    #[cfg(debug_assertions)]
    if let Some(folder) = the_throwaway_folder() {
        return Ok(throwaway::the_user_home_under(folder));
    }
    user_home()
}

/// **L3a's temporary area, or nothing where it cannot be named.**
///
/// *"Never the operating system's shared temporary directory"* — L3a. **So
/// there is no fallback**: a caller handed nothing makes no attempt that needs
/// the area and says so the way its own act already says it could not go on.
/// *(Added 2026-09-18, step 10b's first audit round, item 16: two callers fell
/// back to the shared temporary directory and opened the passphrase channel
/// there, and a third fell back to an empty path.)*
pub fn smoozys_own_temporary_area() -> Option<PathBuf> {
    resolve(Location::TemporaryArea).ok()
}

/// The Linux package cache survives restart until the probe record clears.
/// `Desktop app and delivery` Z11 and its Linux build detail; resolved only here.
pub fn the_linux_update_cache() -> Result<PathBuf, NotNamed> {
    resolve(Location::PerUserApplicationData).map(|root| root.join("linux-update"))
}

/// **Z15's new per-user folders on Windows, one per installed version**, under
/// F11's per-user application data, in `windows-update`: each version Smoozy
/// installs for itself is a folder of its own there, named by its version.
/// Resolved only here (`Desktop app and delivery` Z15 and its Windows build detail;
/// step 11's Windows update piece).
pub fn the_windows_builds_folder() -> Result<PathBuf, NotNamed> {
    resolve(Location::PerUserApplicationData).map(|root| root.join("windows-update"))
}

/// The two areas L3a and `Security and licence` §7 sweep at application start.
///
/// **The sweep is the area's rather than the writing module's**, which is L3a's
/// own reason: *discarded at once* does not cover a process that exits first,
/// so an interrupted run leaves key material with no owner until some later
/// run picks it up. Both are listed here so neither can be forgotten by the
/// module that writes into it.
pub const SWEPT_AT_START: [Location; 2] =
    [Location::TemporaryArea, Location::ThrowawayKnownHostsArea];

// ---------------------------------------------------------------------------
// The throwaway copy — a development build and nothing else
// ---------------------------------------------------------------------------

/// **One way to start a copy of Smoozy whose record, home folder and secrets
/// are all disposable.** Without it nobody can open the application on the
/// Creator's own computer to look at a screen, because the record's folder
/// comes from the system and the home folder — with `~/.ssh` in it — comes from
/// the password database.
///
/// **It is a launch argument and never an environment value.** L3a closes an
/// environment value by name for the home directory reading, so a setting that
/// redirected either location out of the environment would break the rule it
/// stands inside. The application's own command line is not a reading of
/// anything; it is what this run was asked to do.
///
/// **Both replacements sit inside F11's one funnel** — [`resolve`] and nowhere
/// else. `resolve` composes every area from one root and the override replaces
/// that root; `user_home` answers the password database and the override
/// answers the given folder instead. **Nothing anywhere else changes and F11
/// stays true.**
///
/// # What a throwaway copy does not redirect, and what to remove after one
///
/// **The setting redirects three things and no others: the record's root, the
/// home folder, and the secrets** — the last by the development mark every item
/// a throwaway copy writes carries in its subject, so its account names begin
/// `development.` (`crate::platform::secrets`). **Everything else a copy
/// touches goes where it always goes**, and deleting the throwaway folder does
/// not remove it:
///
/// - **The window's own storage and caches.** The system web view keeps them
///   outside anything Smoozy resolves — on this Mac under `~/Library/WebKit/`
///   and `~/Library/Caches/`, in folders named after whatever bundle
///   identifier the copy ran under (`com.smoozy.desktop` for the application's
///   own bundle, or the identifier an agent gave its own wrapper). Observed at
///   this desk: step 10b's first audit round found six such pairs left behind
///   by agents' copies.
/// - **The Downloads folder.** A download a copy makes lands where `Download`
///   puts every download, which is not under the throwaway folder.
/// - **Notifications.** A copy posts to the operating system like any other,
///   and what it posted stays in the system's list until removed there.
/// - **Development items in the secret store.** They are swept only when a
///   throwaway copy starts in a folder that holds no record yet
///   (`crate::start`), and never when a copy closes or is killed. *(Corrected
///   2026-10-17: this said they were swept at every throwaway start and again
///   at close, which the build for the Creator's eleventh notes, note 2,
///   stopped that day, so a test copy keeps its secrets while it keeps its
///   record.)* So a copy's items stay after it, until a throwaway copy next
///   starts in a fresh folder.
///
/// **So an agent that ran a throwaway copy removes, before it reports:** the
/// throwaway folder; any secret-store item whose account name begins
/// `development.`; the two folders under `~/Library/WebKit/` and
/// `~/Library/Caches/` named after the identifier its copy ran under, and only
/// those it made; anything the copy downloaded; and any notification the copy
/// posted. **Widening the setting to redirect these is a want and is parked**
/// (step 10b's first audit round, item 12); writing them down is what closes
/// the clean-up.
///
/// **`#[cfg(debug_assertions)]` is the gate, and it is the whole gate.** The
/// release route is `tauri build`, which builds the release profile, where
/// Cargo's own default leaves debug assertions off; `tauri dev` and
/// `cargo test` build profiles where they are on. **So the gate is exactly the
/// line between a build somebody made to look at and a build that goes out.** A
/// Cargo feature would be a worse gate, because `--all-features` switches one
/// on and agents and tooling reach for it; nothing turns debug assertions on in
/// a release build but an environment variable nothing in this project sets.
/// **`#[cfg(test)]` would not do either**, because the point is a running
/// application with a window rather than a test.
#[cfg(debug_assertions)]
pub mod throwaway {
    use std::ffi::OsString;
    use std::path::{Path, PathBuf};

    /// The argument, in the application's own command line. Either
    /// `--throwaway-copy=<folder>` or `--throwaway-copy <folder>`.
    pub const THE_ARGUMENT: &str = "--throwaway-copy";

    /// What the copy's application-data root is, under the folder it was
    /// pointed at. **It still ends in [`super::FOLDER`]**, so everything L3
    /// says about the shape of that root holds for a throwaway copy too.
    pub fn the_application_data_root_under(folder: &Path) -> PathBuf {
        folder.join("application-data").join(super::FOLDER)
    }

    /// What the copy's home directory is, under the same folder. **Smoozy
    /// writes nothing at this level itself** — `Keys` is the one consumer and
    /// it writes under `.ssh` — and putting it inside the throwaway folder is
    /// what keeps a desk copy away from the Creator's real `~/.ssh`.
    pub fn the_user_home_under(folder: &Path) -> PathBuf {
        folder.join("home")
    }

    /// **Smoozy's own temporary area in a copy pointed at `folder`**, composed
    /// by F11's own function — so a test can put the passphrase channel exactly
    /// where the running copy puts it. *(Added 2026-09-18, step 10b's audit
    /// round's loose ends, L1: the test that passed used a short scratch folder
    /// the running copy never uses, and the copy failed.)*
    pub fn the_temporary_area_under(folder: &Path) -> PathBuf {
        super::the_area(
            super::Location::TemporaryArea,
            the_application_data_root_under(folder),
        )
    }

    /// The folder a launch line points at — **read from the arguments rather
    /// than from the process, so a test can stand where the application
    /// stands.**
    ///
    /// **A relative folder is made absolute against the working directory
    /// rather than refused.** A refusal that said nothing would leave the copy
    /// running against the Creator's real record, which is the one direction
    /// this whole setting exists to close.
    pub fn the_folder_named_on(arguments: impl IntoIterator<Item = OsString>) -> Option<PathBuf> {
        let mut arguments = arguments.into_iter();
        let with_a_value = format!("{THE_ARGUMENT}=");
        while let Some(argument) = arguments.next() {
            let text = argument.to_string_lossy().into_owned();
            let given = if let Some(value) = text.strip_prefix(&with_a_value) {
                PathBuf::from(value)
            } else if text == THE_ARGUMENT {
                PathBuf::from(arguments.next()?)
            } else {
                continue;
            };
            if given.as_os_str().is_empty() {
                return None;
            }
            return Some(made_absolute(given));
        }
        None
    }

    /// **An argument that begins `--throwaway` and names no folder — the one
    /// the copy refuses to start on**, handed back so the refusal can say which.
    ///
    /// *(Added 2026-09-18, step 10b's first audit round, item 10.)* `--throwaway-copy=`
    /// with nothing after it, a bare `--throwaway-copy` as the last word, and a
    /// misspelling such as `--throwaway_copy` all used to answer *no folder*,
    /// and a copy asked for no folder opens the Creator's real record, home
    /// folder and keychain items. **An empty shell variable did it by
    /// accident.** So any argument that begins `--throwaway` must be one of the
    /// two spellings with a folder, or the copy does not start.
    pub fn an_argument_that_names_no_folder(
        arguments: impl IntoIterator<Item = OsString>,
    ) -> Option<String> {
        let mut arguments = arguments.into_iter();
        let with_a_value = format!("{THE_ARGUMENT}=");
        while let Some(argument) = arguments.next() {
            let text = argument.to_string_lossy().into_owned();
            if !text.starts_with("--throwaway") {
                continue;
            }
            let names_a_folder = if let Some(value) = text.strip_prefix(&with_a_value) {
                !value.is_empty()
            } else if text == THE_ARGUMENT {
                arguments.next().is_some_and(|next| !next.is_empty())
            } else {
                false
            };
            if !names_a_folder {
                return Some(text);
            }
        }
        None
    }

    /// **The line printed where the copy refuses to start.** It is the
    /// terminal's and no screen's, and it names the argument.
    pub fn the_refusal(argument: &str) -> String {
        format!(
            "Smoozy did not start: `{argument}` names no folder. A throwaway copy \
             is started with `{THE_ARGUMENT}=<folder>`, and without a folder this \
             copy would open the real record."
        )
    }

    fn made_absolute(given: PathBuf) -> PathBuf {
        if given.is_absolute() {
            return given;
        }
        match std::env::current_dir() {
            Ok(here) => here.join(given),
            Err(_) => given,
        }
    }

    /// Where this copy was pointed. **Read once**, at the first ask, from
    /// [`this_launch_line`].
    pub fn the_folder_this_copy_was_pointed_at() -> Option<&'static Path> {
        static THE_FOLDER: std::sync::OnceLock<Option<PathBuf>> = std::sync::OnceLock::new();
        THE_FOLDER
            .get_or_init(|| the_folder_named_on(this_launch_line()))
            .as_deref()
    }

    // -----------------------------------------------------------------------
    // A test copy's own launch words, built in
    // -----------------------------------------------------------------------

    /// **The folder a test copy was built for** — `App/Build a test copy.sh`
    /// sets `SMOOZY_TEST_COPY_FOLDER` when it builds *Smoozy Test*. Absent in
    /// every other build.
    ///
    /// *(Added 2026-11-01, the Mac watch's third finding.)* Until then the
    /// bundle's starting file was a launcher script that handed the program
    /// these words, and **macOS could not match the program that asked for
    /// notifications to the bundle**, so a test copy could never show one. The
    /// program is now the bundle's starting file, as in a release, and the
    /// words it used to be handed are built into it instead — so it starts on
    /// its own folder however it is opened, even straight from a shell, which
    /// the launcher never covered.
    pub const THE_FOLDER_A_TEST_COPY_WAS_BUILT_FOR: Option<&str> =
        option_env!("SMOOZY_TEST_COPY_FOLDER");

    /// **The desk program a test copy was built with, relative to the
    /// program's own folder** — `SMOOZY_TEST_COPY_DESK_PROGRAM`, which the
    /// script sets to the server program in the bundle's `Resources`.
    pub const THE_DESK_PROGRAM_A_TEST_COPY_WAS_BUILT_WITH: Option<&str> =
        option_env!("SMOOZY_TEST_COPY_DESK_PROGRAM");

    /// **The launch line this copy honours: its own command line, then the
    /// words it was built with where the line carries none of its own.**
    ///
    /// **A line that already says anything about a throwaway folder or a desk
    /// program is left exactly as given**, so an agent's own
    /// `--throwaway-copy=<folder>` wins and a misspelt one is still refused.
    pub fn this_launch_line() -> Vec<OsString> {
        the_launch_line_with(
            std::env::args_os().skip(1),
            THE_FOLDER_A_TEST_COPY_WAS_BUILT_FOR,
            THE_DESK_PROGRAM_A_TEST_COPY_WAS_BUILT_WITH,
            std::env::current_exe().ok().as_deref().and_then(Path::parent),
        )
    }

    /// [`this_launch_line`], with what it reads handed in — so a test can
    /// stand where a test copy stands.
    pub fn the_launch_line_with(
        given: impl IntoIterator<Item = OsString>,
        built_for: Option<&str>,
        desk_program_beside: Option<&str>,
        the_programs_folder: Option<&Path>,
    ) -> Vec<OsString> {
        let mut line: Vec<OsString> = given.into_iter().collect();
        let Some(folder) = built_for.filter(|folder| !folder.is_empty()) else {
            return line;
        };
        let says = |start: &str| {
            line.iter()
                .any(|word| word.to_string_lossy().starts_with(start))
        };
        if says("--throwaway") || says("--desk") {
            return line;
        }
        let mut pointed = OsString::from(format!("{THE_ARGUMENT}="));
        pointed.push(folder);
        line.push(pointed);
        if let (Some(beside), Some(here)) = (
            desk_program_beside.filter(|beside| !beside.is_empty()),
            the_programs_folder,
        ) {
            let mut given = OsString::from(format!("{THE_DESK_PROGRAM_ARGUMENT}="));
            given.push(here.join(beside));
            line.push(given);
        }
        line
    }

    // -----------------------------------------------------------------------
    // The desk program — `Server protocol and transfers` B10a
    // -----------------------------------------------------------------------

    /// **The second argument, and it is honoured only beside the first.**
    /// Either `--desk-program=<file>` or `--desk-program <file>`, naming a
    /// server program built on this computer, which the ready action then
    /// places instead of fetching the current release (`Server protocol and
    /// transfers` B10a; `crate::the_desk_program`).
    ///
    /// **It lives inside this module for the reason the throwaway argument
    /// does**: its text, and every route that reads it, is absent from a build
    /// made without debug assertions, and the workflow reads each release
    /// binary for this text to prove so.
    pub const THE_DESK_PROGRAM_ARGUMENT: &str = "--desk-program";

    /// The file a launch line names, whether or not the line also carries a
    /// throwaway folder. **A relative file is made absolute** the way a
    /// relative folder is.
    pub fn the_desk_program_named_on(
        arguments: impl IntoIterator<Item = OsString>,
    ) -> Option<PathBuf> {
        let mut arguments = arguments.into_iter();
        let with_a_value = format!("{THE_DESK_PROGRAM_ARGUMENT}=");
        while let Some(argument) = arguments.next() {
            let text = argument.to_string_lossy().into_owned();
            let given = if let Some(value) = text.strip_prefix(&with_a_value) {
                PathBuf::from(value)
            } else if text == THE_DESK_PROGRAM_ARGUMENT {
                PathBuf::from(arguments.next()?)
            } else {
                continue;
            };
            if given.as_os_str().is_empty() {
                return None;
            }
            return Some(made_absolute(given));
        }
        None
    }

    /// **Why this launch line must not start, where the desk program argument
    /// is on it and cannot be honoured.** Three cases, and each refuses the
    /// start rather than letting the copy run on as though the argument were
    /// not there:
    ///
    /// - **no throwaway folder beside it.** `Server protocol and transfers`
    ///   B10a permits the route only in a copy pointed at a throwaway folder, so
    ///   a copy holding the Creator's real record never places a desk program;
    /// - **it names no file**, or is a misspelling of the argument — the rule
    ///   the throwaway argument keeps, for the same reason;
    /// - **the file it names is not there**, so the mistake is met at the
    ///   terminal rather than half-way through a setup on a rented machine.
    pub fn the_desk_programs_refusal(
        arguments: impl IntoIterator<Item = OsString>,
    ) -> Option<String> {
        let arguments: Vec<OsString> = arguments.into_iter().collect();
        let with_a_value = format!("{THE_DESK_PROGRAM_ARGUMENT}=");
        let mut present = false;
        let mut words = arguments.iter();
        while let Some(argument) = words.next() {
            let text = argument.to_string_lossy().into_owned();
            if !text.starts_with("--desk") {
                continue;
            }
            present = true;
            let names_a_file = if let Some(value) = text.strip_prefix(&with_a_value) {
                !value.is_empty()
            } else if text == THE_DESK_PROGRAM_ARGUMENT {
                words.next().is_some_and(|next| !next.is_empty())
            } else {
                false
            };
            if !names_a_file {
                return Some(format!(
                    "Smoozy did not start: `{text}` names no file. A desk program is \
                     given with `{THE_DESK_PROGRAM_ARGUMENT}=<file>`."
                ));
            }
        }
        if !present {
            return None;
        }
        if the_folder_named_on(arguments.iter().cloned()).is_none() {
            return Some(format!(
                "Smoozy did not start: `{THE_DESK_PROGRAM_ARGUMENT}` is honoured only in \
                 a throwaway copy, and this launch line names no `{THE_ARGUMENT}=<folder>`."
            ));
        }
        let file = the_desk_program_named_on(arguments.iter().cloned())?;
        if !file.is_file() {
            return Some(format!(
                "Smoozy did not start: the desk program `{}` is not a file on this computer.",
                file.display()
            ));
        }
        None
    }

    /// **The desk program this copy was given, and only where this copy is a
    /// throwaway one.** Read once, like the folder. The start refuses a line
    /// carrying the argument without a folder, and this asks again rather than
    /// trusting that it did.
    pub fn the_desk_program_this_copy_was_given() -> Option<&'static Path> {
        static THE_FILE: std::sync::OnceLock<Option<PathBuf>> = std::sync::OnceLock::new();
        the_folder_this_copy_was_pointed_at()?;
        THE_FILE
            .get_or_init(|| the_desk_program_named_on(this_launch_line()))
            .as_deref()
    }
}

/// **Where this copy was pointed, and in a release build the answer is always
/// nowhere.**
pub fn the_throwaway_folder() -> Option<&'static std::path::Path> {
    #[cfg(debug_assertions)]
    {
        throwaway::the_folder_this_copy_was_pointed_at()
    }
    #[cfg(not(debug_assertions))]
    {
        None
    }
}

/// Whether this copy's record, home folder and secrets are all disposable.
///
/// **The one thing every other part of the application asks**, so that no
/// second reading of the command line exists anywhere.
pub fn this_copy_is_a_throwaway_one() -> bool {
    the_throwaway_folder().is_some()
}

/// **What a launch line points *this build* at — the question a release build
/// must answer *nowhere* to**, and the one the test below asks.
///
/// It exists unconditionally on purpose: a test cannot ask a function that is
/// not compiled, so the gate is a wall that is always there rather than a door
/// that is sometimes missing.
pub fn the_throwaway_folder_a_launch_line_points_at(
    arguments: impl IntoIterator<Item = std::ffi::OsString>,
) -> Option<PathBuf> {
    #[cfg(debug_assertions)]
    {
        throwaway::the_folder_named_on(arguments)
    }
    #[cfg(not(debug_assertions))]
    {
        let _ = arguments.into_iter();
        None
    }
}

/// **Which desk program a launch line has *this build* place, and in a release
/// build the answer is always none** — `Server protocol and transfers` B10a's
/// *no release build honours it*, asked in both builds by the test below.
///
/// In a development build the answer is the named file **only where the same
/// line also points the copy at a throwaway folder**, which is that clause's other
/// half. It exists unconditionally for the reason
/// [`the_throwaway_folder_a_launch_line_points_at`] does.
pub fn the_desk_program_a_launch_line_names(
    arguments: impl IntoIterator<Item = std::ffi::OsString>,
) -> Option<PathBuf> {
    #[cfg(debug_assertions)]
    {
        let arguments: Vec<std::ffi::OsString> = arguments.into_iter().collect();
        throwaway::the_folder_named_on(arguments.iter().cloned())?;
        throwaway::the_desk_program_named_on(arguments)
    }
    #[cfg(not(debug_assertions))]
    {
        let _ = arguments.into_iter();
        None
    }
}

// ---------------------------------------------------------------------------
// macOS
// ---------------------------------------------------------------------------

/// L3b, macOS. `FileManager.url(for: .applicationSupportDirectory, in:
/// .userDomainMask, appropriateFor: nil, create:)`.
///
/// This is [W1](#)'s own mechanism with one constant changed, which is why no
/// second route is introduced for it. Apple's words for the constant are
/// *"Application support files (`Library/Application Support`)"* and for the
/// domain *"The user's home directory—the place to install user's personal
/// items (`~`)"*.
///
/// **The folder is asked for with creation on**, because the method is
/// documented to *"locate and optionally create"* and a location that does not
/// exist yet is the ordinary state on a fresh installation. Creating Smoozy's
/// own folder under it is [`ensure_directory`](super::files::ensure_directory)'s
/// and goes through the adapter like every other local write (F10a).
#[cfg(target_os = "macos")]
fn per_user_application_data() -> Result<PathBuf, NotNamed> {
    use objc2_foundation::{NSFileManager, NSSearchPathDirectory, NSSearchPathDomainMask};

    let manager = NSFileManager::defaultManager();
    let url = manager
        .URLForDirectory_inDomain_appropriateForURL_create_error(
            NSSearchPathDirectory::ApplicationSupportDirectory,
            NSSearchPathDomainMask::UserDomainMask,
            None,
            true,
        )
        .map_err(|error| {
            NotNamed::new(
                Location::PerUserApplicationData,
                error.localizedDescription().to_string(),
            )
        })?;

    let path = url.path().ok_or_else(|| {
        NotNamed::new(
            Location::PerUserApplicationData,
            "the location the system named is not a file path",
        )
    })?;

    Ok(PathBuf::from(path.to_string()).join(FOLDER))
}

/// L3b, macOS. The password-database entry for the **effective** user id, field
/// `pw_dir`.
///
/// **The route [I3](#) already takes on this system for the login account name,
/// one field over.** macOS's own manual page says where the entry comes from —
/// *"These functions obtain information from `opendirectoryd`(8), including
/// records in `/etc/master.passwd`"* — and names the field *"home directory"*.
///
/// **Not `NSHomeDirectory()`**, which Apple's own page says answers two
/// different questions behind one call — *"the application's sandbox
/// directory, or the current user's home directory if the application isn't in
/// a sandbox"* — and which says nothing about whether it consults `$HOME`.
/// L3a wants the directory `ssh` and `ssh-keygen` themselves resolve, which
/// Fact 33 observed on this system to be the real account home. **And never
/// `NSHomeDirectoryForUser`**, which takes an account name and is L3a's first
/// closed way in another spelling.
#[cfg(unix)]
fn user_home() -> Result<PathBuf, NotNamed> {
    password_database_home()
}

/// L3b, Linux. The same password-database entry, on the Linux man-pages
/// project's own `getpwnam(3)` — *"the record in the password database"*, whose
/// `pw_dir` is the *"home directory"*. **Documentation alone**, on a system
/// this project has no computer for, exactly as I3's Linux bullet already
/// records for the login account name.
///
/// **The environment is not consulted, on either system.** L3a closes that by
/// name, and the `$HOME` the Linux application-data location below uses is the
/// specification's own stated default rather than a home-directory reading —
/// the same split [W3](#) already draws for the downloads folder.
#[cfg(unix)]
fn password_database_home() -> Result<PathBuf, NotNamed> {
    use std::ffi::{CStr, OsString};
    use std::os::unix::ffi::OsStringExt;

    // `getpwuid_r` rather than `getpwuid`, because the plain form hands back a
    // pointer into storage the next caller may overwrite. The buffer is grown
    // rather than guessed: the system tells us what it wants, and a system
    // that will not say gets a starting size that is enlarged on `ERANGE`.
    let mut size = match unsafe { libc::sysconf(libc::_SC_GETPW_R_SIZE_MAX) } {
        n if n > 0 => n as usize,
        _ => 1024,
    };

    loop {
        let mut entry: libc::passwd = unsafe { std::mem::zeroed() };
        let mut found: *mut libc::passwd = std::ptr::null_mut();
        let mut buffer = vec![0i8; size];

        let status = unsafe {
            libc::getpwuid_r(
                libc::geteuid(),
                &mut entry,
                buffer.as_mut_ptr(),
                size,
                &mut found,
            )
        };

        if status == libc::ERANGE && size < 1 << 20 {
            size *= 2;
            continue;
        }
        if status != 0 {
            return Err(NotNamed::new(
                Location::UserHome,
                format!("the password database could not be read (error {status})"),
            ));
        }
        if found.is_null() {
            return Err(NotNamed::new(
                Location::UserHome,
                "there is no password-database entry for this account",
            ));
        }
        if entry.pw_dir.is_null() {
            return Err(NotNamed::new(
                Location::UserHome,
                "the password-database entry names no home directory",
            ));
        }

        let bytes = unsafe { CStr::from_ptr(entry.pw_dir) }.to_bytes().to_vec();
        if bytes.is_empty() {
            return Err(NotNamed::new(
                Location::UserHome,
                "the password-database entry names an empty home directory",
            ));
        }
        return Ok(PathBuf::from(OsString::from_vec(bytes)));
    }
}

// ---------------------------------------------------------------------------
// Linux
// ---------------------------------------------------------------------------

/// L3b, Linux. `XDG_DATA_HOME`, and where that is unset or empty the
/// specification's own default of `$HOME/.local/share`.
///
/// **Read and never assembled**, which is [W3](#)'s discipline applied to a
/// location that does have a specification. The specification's absolute-path
/// rule is kept as written — *"All paths set in these environment variables
/// must be absolute. If an implementation encounters a relative path in any of
/// these variables it should consider the path invalid and ignore it."* — so a
/// relative value is ignored and the default is taken instead.
///
/// **The `$HOME` here is the specification's own default and is not a
/// home-directory reading**, and it is not read across to one: L3a's home
/// directory on this system is the password-database entry above.
#[cfg(target_os = "linux")]
fn per_user_application_data() -> Result<PathBuf, NotNamed> {
    use std::path::Path;

    if let Some(value) = std::env::var_os("XDG_DATA_HOME") {
        let path = PathBuf::from(value);
        if !path.as_os_str().is_empty() && path.is_absolute() {
            return Ok(path.join(FOLDER));
        }
    }

    match std::env::var_os("HOME") {
        Some(home) if Path::new(&home).is_absolute() => Ok(PathBuf::from(home)
            .join(".local")
            .join("share")
            .join(FOLDER)),
        _ => Err(NotNamed::new(
            Location::PerUserApplicationData,
            "XDG_DATA_HOME is unset and HOME names no absolute path",
        )),
    }
}

// ---------------------------------------------------------------------------
// Windows
// ---------------------------------------------------------------------------

/// L3b, Windows. `SHGetKnownFolderPath(FOLDERID_LocalAppData, 0, NULL, &p)`,
/// then `CoTaskMemFree`.
///
/// **The local location and not the roaming one, on Microsoft's own words
/// rather than on a preference**: `FOLDERID_LocalAppData` is *"The file system
/// directory that serves as a data repository for local (nonroaming)
/// applications"*, and `FOLDERID_RoamingAppData` carries no such statement. A
/// roamed copy of this record would be one installation's state arriving on
/// another computer.
///
/// **[W2](#)'s refusal holds here and for the same reason**: the flags argument
/// is `0` and `KF_FLAG_DEFAULT_PATH` is never passed, so a relocated folder is
/// followed rather than defeated. **Documentation alone** — no Windows
/// computer has ever run any of this (Section 22 item 2).
#[cfg(target_os = "windows")]
fn per_user_application_data() -> Result<PathBuf, NotNamed> {
    known_folder(
        Location::PerUserApplicationData,
        &windows_sys::Win32::UI::Shell::FOLDERID_LocalAppData,
    )
    .map(|path| path.join(FOLDER))
}

/// L3b, Windows. `SHGetKnownFolderPath(FOLDERID_Profile, 0, NULL, &p)`.
///
/// Microsoft's own entry gives its default path as `%USERPROFILE%`, and the
/// older equivalent's description carries the instruction this adapter already
/// obeys — *"Applications should not create files or folders at this level"* —
/// which is why the reading is a reading and Smoozy writes nothing there.
///
/// **Whether this call agrees with what Windows' own `ssh` resolves is `Keys`
/// L1a's question and no page settles it.** It wants a Windows computer and is
/// permanently unresolved (`Desktop app and delivery` §22 items 2 and 59).
/// **No space is left for the answer**: the call is named, it is used, and the
/// silence is recorded rather than coded around.
#[cfg(target_os = "windows")]
fn user_home() -> Result<PathBuf, NotNamed> {
    known_folder(
        Location::UserHome,
        &windows_sys::Win32::UI::Shell::FOLDERID_Profile,
    )
}

/// The one Windows route, shared by both readings above.
///
/// Microsoft's own rules, kept as written: the flags argument *"can be 0"*; a
/// null token *"requests the known folder for the current user"*; the caller
/// frees the result *"whether SHGetKnownFolderPath succeeds or not"*; and the
/// two named failures are `E_FAIL` and `E_INVALIDARG`.
#[cfg(target_os = "windows")]
fn known_folder(
    location: Location,
    id: *const windows_sys::core::GUID,
) -> Result<PathBuf, NotNamed> {
    use std::ffi::OsString;
    use std::os::windows::ffi::OsStringExt;
    use windows_sys::Win32::System::Com::CoTaskMemFree;
    use windows_sys::Win32::UI::Shell::SHGetKnownFolderPath;

    let mut wide: windows_sys::core::PWSTR = std::ptr::null_mut();
    let status = unsafe { SHGetKnownFolderPath(id, 0, std::ptr::null_mut(), &mut wide) };

    // Freed on both paths, which is what the page asks for.
    let path = if wide.is_null() {
        None
    } else {
        let mut length = 0usize;
        while unsafe { *wide.add(length) } != 0 {
            length += 1;
        }
        let slice = unsafe { std::slice::from_raw_parts(wide, length) };
        let owned = OsString::from_wide(slice);
        unsafe { CoTaskMemFree(wide as *const core::ffi::c_void) };
        Some(PathBuf::from(owned))
    };

    if status < 0 {
        return Err(NotNamed::new(
            location,
            format!("the system named no such folder (0x{:08X})", status as u32),
        ));
    }

    match path {
        Some(path) if !path.as_os_str().is_empty() => Ok(path),
        _ => Err(NotNamed::new(location, "the system returned an empty path")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// F11's closure, kept by the type. Every variant resolves through the one
    /// function and none of them composes a path anywhere else.
    #[test]
    fn every_area_sits_under_the_application_data_root() {
        let Ok(root) = resolve(Location::PerUserApplicationData) else {
            // A machine that names no such location is a case W5 covers, and
            // the rest of this test has nothing to say about it.
            return;
        };
        for location in [
            Location::LocalRecord,
            Location::DiaryArea,
            Location::ThrowawayKnownHostsArea,
            Location::TemporaryArea,
            Location::FetchedDerivativeArea,
            Location::StagedTransferArea,
            Location::TheRunningCopysHold,
            Location::TheRunningCopysDoor,
        ] {
            let path = resolve(location).expect("the root resolved, so these must");
            assert!(path.starts_with(&root), "{location:?} sat outside the root");
            assert_ne!(path, root, "{location:?} is the root itself");
        }
    }

    /// L2a and L3a: the diary and the temporary area sit **beside** the local
    /// record and never inside it.
    #[test]
    fn nothing_sits_inside_the_local_record() {
        let Ok(record) = resolve(Location::LocalRecord) else {
            return;
        };
        for location in [Location::DiaryArea, Location::TemporaryArea] {
            let path = resolve(location).expect("the root resolved, so these must");
            assert!(
                !path.starts_with(&record),
                "{location:?} sat inside the record"
            );
        }
    }

    /// L3a: Smoozy's own temporary area is **never** the operating system's
    /// shared temporary directory.
    #[test]
    fn the_temporary_area_is_not_the_systems_own() {
        let Ok(ours) = resolve(Location::TemporaryArea) else {
            return;
        };
        assert!(!ours.starts_with(std::env::temp_dir()));
        assert_eq!(smoozys_own_temporary_area(), Some(ours));
    }

    /// **L3a, as a sweep of this crate's own text: nothing falls back to the
    /// shared temporary directory where Smoozy's own area cannot be named** —
    /// step 10b's first audit round, item 16. The helper answers nothing
    /// rather than a fallback, and no working file composes one.
    #[test]
    fn nothing_falls_back_to_the_shared_temporary_directory() {
        for (name, text) in [
            ("src/servers/a_period.rs", include_str!("../servers/a_period.rs")),
            ("src/surface/keys.rs", include_str!("../surface/keys.rs")),
            ("src/surface/servers.rs", include_str!("../surface/servers.rs")),
        ] {
            let working = text.split("#[cfg(test)]").next().unwrap_or_default();
            for line in working.lines().filter(|line| !line.trim_start().starts_with("//")) {
                assert!(
                    !(line.contains("TemporaryArea") && line.contains("unwrap_or")),
                    "{name} falls back where Smoozy's own temporary area cannot be named: {line}"
                );
                assert!(
                    !line.contains("temp_dir()"),
                    "{name} reaches the shared temporary directory outside a test: {line}"
                );
            }
        }
    }

    /// L3a's sweep list is both areas and neither is forgotten.
    #[test]
    fn both_swept_areas_are_listed() {
        assert!(SWEPT_AT_START.contains(&Location::TemporaryArea));
        assert!(SWEPT_AT_START.contains(&Location::ThrowawayKnownHostsArea));
    }

    /// L3 and L3a: no reading is assembled from the login account name. The
    /// home directory may legitimately *contain* it — that is the system's own
    /// answer — but the application-data root is never built by joining it.
    #[test]
    fn the_application_data_root_is_not_built_from_an_account_name() {
        let Ok(root) = resolve(Location::PerUserApplicationData) else {
            return;
        };
        assert_eq!(
            root.file_name().and_then(|name| name.to_str()),
            Some(FOLDER)
        );
    }

    /// L3a: the home directory is a reading, and an absolute one where the
    /// system answers at all.
    #[test]
    fn the_home_directory_is_absolute_where_it_is_named() {
        match resolve(Location::UserHome) {
            Ok(home) => assert!(home.is_absolute(), "the system named a relative home"),
            Err(not_named) => assert_eq!(not_named.location, Location::UserHome),
        }
    }

    /// **The gate, and it is the one test this piece owes.**
    ///
    /// Run with debug assertions on — `cargo test` — the launch argument names
    /// a folder. Run with them off — `cargo test --release` — **the same
    /// argument names nothing**, because the whole override is not compiled.
    /// A build that goes out cannot be pointed anywhere by anybody.
    #[test]
    fn a_release_build_does_not_honour_the_throwaway_argument() {
        use std::ffi::OsString;

        // An absolute folder in this system's own spelling, so the argument is
        // taken exactly as given on all three.
        let folder = an_absolute_folder();
        let mut argument = OsString::from("--throwaway-copy=");
        argument.push(&folder);
        let line = vec![OsString::from("smoozy"), argument];
        let answered = the_throwaway_folder_a_launch_line_points_at(line);

        if cfg!(debug_assertions) {
            assert_eq!(
                answered,
                Some(folder),
                "a development build did not honour the argument, so nobody \
                 can open the application without opening the Creator's own \
                 record"
            );
        } else {
            assert_eq!(
                answered,
                None,
                "the throwaway override is reachable with debug assertions \
                 off. A build that goes out can be pointed at another record, \
                 another home folder and another set of secrets from its own \
                 command line"
            );
            assert!(
                !this_copy_is_a_throwaway_one(),
                "a release build called itself a throwaway copy"
            );
        }
    }

    /// **A test copy built with its own folder starts on it with nothing on
    /// its command line** — the Mac watch's third finding, 2026-11-01. The
    /// program is the bundle's starting file, so nothing hands it words; the
    /// words it was built with stand in, and the desk program is the one beside
    /// it. **A line that says anything of its own is left as it is.**
    ///
    /// **Every folder here is absolute in this system's own spelling**, as
    /// [`an_absolute_folder`] says why. Until 2026-11-07 they were written as
    /// Mac paths, and on Windows `/Users/...` has no drive, so it is not
    /// absolute there and was made absolute against the working folder — hosted
    /// run `42299470433` read `D:/Users/...`. The code was right; the test
    /// asked a Mac question on Windows.
    #[cfg(debug_assertions)]
    #[test]
    fn a_test_copy_starts_on_the_folder_it_was_built_for() {
        use std::ffi::OsString;
        let base = an_absolute_folder();
        let folder_path = base.join("Smoozy Test");
        let folder_text = folder_path.to_string_lossy().into_owned();
        let folder = folder_text.as_str();
        let here_path = base.join("Smoozy Test.app").join("Contents").join("MacOS");
        let here = here_path.as_path();
        let elsewhere = base.join("elsewhere");
        let its_own_word = format!("--throwaway-copy={}", elsewhere.display());
        let built = |given: &[&str]| {
            throwaway::the_launch_line_with(
                given.iter().map(OsString::from),
                Some(folder),
                Some("../Resources/smoozy-server"),
                Some(here),
            )
        };

        let line = built(&[]);
        assert_eq!(
            throwaway::the_folder_named_on(line.clone()),
            Some(PathBuf::from(folder)),
            "a test copy opened by a double-click would have opened the real record"
        );
        assert_eq!(
            throwaway::the_desk_program_named_on(line.clone()),
            Some(here.join("../Resources/smoozy-server"))
        );
        assert_eq!(throwaway::an_argument_that_names_no_folder(line), None);

        // **An agent's own folder wins**, and a misspelt one is still refused.
        let own = built(&[its_own_word.as_str()]);
        assert_eq!(throwaway::the_folder_named_on(own.clone()), Some(elsewhere.clone()));
        assert_eq!(throwaway::the_desk_program_named_on(own), None);
        let misspelt = built(&["--throwaway_copy=/tmp/elsewhere"]);
        assert!(throwaway::an_argument_that_names_no_folder(misspelt).is_some());

        // **A build made for no folder adds nothing**, which is every build but
        // a test copy's.
        let plain = throwaway::the_launch_line_with(
            Vec::<OsString>::new(),
            None,
            Some("../Resources/smoozy-server"),
            Some(here),
        );
        assert!(plain.is_empty());
        assert_eq!(throwaway::the_launch_line_with(Vec::<OsString>::new(), Some(""), None, Some(here)), Vec::<OsString>::new());
    }

    /// **`Server protocol and transfers` B10a's gate, asked in both builds.**
    ///
    /// Run with debug assertions on, a line carrying both arguments names the
    /// desk program, and a line carrying the desk program alone names none. Run
    /// with them off — `cargo test -p smoozy-desktop --release --lib` — **the
    /// same line names nothing**, because nothing that reads the argument is
    /// compiled. A build that goes out cannot place a program built at a desk.
    #[test]
    fn a_release_build_does_not_honour_the_desk_program_argument() {
        use std::ffi::OsString;

        let folder = an_absolute_folder();
        let program = folder.join("smoozy-server");
        let mut pointed = OsString::from("--throwaway-copy=");
        pointed.push(&folder);
        let mut given = OsString::from("--desk-program=");
        given.push(&program);
        let both = vec![OsString::from("smoozy"), pointed, given.clone()];
        let alone = vec![OsString::from("smoozy"), given];

        if cfg!(debug_assertions) {
            assert_eq!(
                the_desk_program_a_launch_line_names(both),
                Some(program),
                "a development build pointed at a throwaway folder did not \
                 honour the desk program, so nothing machine-side can be watched \
                 before a release"
            );
            assert_eq!(
                the_desk_program_a_launch_line_names(alone),
                None,
                "a copy holding the real record would place a desk program"
            );
        } else {
            assert_eq!(
                the_desk_program_a_launch_line_names(both),
                None,
                "the desk program is honoured with debug assertions off. A build \
                 that goes out can place a program nobody signed"
            );
            assert_eq!(the_desk_program_a_launch_line_names(alone), None);
        }
    }

    /// Both spellings of the desk program argument, and every line the start
    /// refuses.
    #[cfg(debug_assertions)]
    #[test]
    fn the_desk_program_is_refused_without_a_throwaway_folder_or_a_file() {
        use std::ffi::OsString;

        let scratch = std::env::temp_dir().join(format!(
            "smoozy-desk-program-test-{}",
            std::process::id()
        ));
        std::fs::create_dir_all(&scratch).expect("a scratch folder");
        let program = scratch.join("smoozy-server");
        std::fs::write(&program, b"bytes").expect("a stand-in program");
        let folder = scratch.join("copy");
        let folder_text = folder.to_string_lossy().into_owned();
        let program_text = program.to_string_lossy().into_owned();
        let line = |words: &[&str]| words.iter().map(OsString::from).collect::<Vec<_>>();
        let refused = |words: &[&str]| throwaway::the_desk_programs_refusal(line(words));

        // Both spellings are read, and a line holding both arguments starts.
        let with_value = format!("--desk-program={program_text}");
        let pointed = format!("--throwaway-copy={folder_text}");
        assert_eq!(
            throwaway::the_desk_program_named_on(line(&[&with_value])),
            Some(program.clone())
        );
        assert_eq!(
            throwaway::the_desk_program_named_on(line(&["--desk-program", &program_text])),
            Some(program.clone())
        );
        assert_eq!(refused(&[&pointed, &with_value]), None);
        assert_eq!(refused(&[&pointed, "--desk-program", &program_text]), None);
        assert_eq!(refused(&[&pointed]), None, "a plain throwaway copy is refused nothing");
        assert_eq!(refused(&[]), None, "an ordinary launch is refused nothing");

        // Without a throwaway folder the start is refused.
        let alone = refused(&[&with_value]).expect("a real-record copy was allowed a desk program");
        assert!(alone.contains("--throwaway-copy"));

        // A value naming nothing, a misspelling, and a file that is not there.
        assert!(refused(&[&pointed, "--desk-program="]).is_some());
        assert!(refused(&[&pointed, "--desk-program"]).is_some());
        assert!(refused(&[&pointed, &format!("--desk_program={program_text}")]).is_some());
        let missing = format!("--desk-program={}", scratch.join("not-there").display());
        assert!(refused(&[&pointed, &missing]).is_some());

        let _ = std::fs::remove_file(&program);
        let _ = std::fs::remove_dir(&scratch);
    }

    /// An absolute folder in this system's own spelling. **What is absolute is
    /// a question about a system and not about a string**, which is the same
    /// lesson the specification-value test below already carries from the
    /// Windows runner.
    fn an_absolute_folder() -> PathBuf {
        std::env::temp_dir().join("a-throwaway-copy")
    }

    /// Both spellings of the argument, and the two values that name nothing.
    #[cfg(debug_assertions)]
    #[test]
    fn the_argument_names_the_folder_in_either_spelling() {
        use std::ffi::OsString;

        let folder = an_absolute_folder();
        let text = folder.to_string_lossy().into_owned();
        let named = |arguments: &[&str]| {
            throwaway::the_folder_named_on(arguments.iter().map(OsString::from))
        };

        assert_eq!(
            named(&[&format!("--throwaway-copy={text}")]),
            Some(folder.clone())
        );
        assert_eq!(named(&["--throwaway-copy", &text]), Some(folder.clone()));
        // Anything else on the line is passed over rather than read.
        assert_eq!(
            named(&["--other", "value", &format!("--throwaway-copy={text}")]),
            Some(folder)
        );
        assert_eq!(named(&["--not-this-one=somewhere"]), None);

        // **An argument that names no folder refuses the start** rather than
        // opening the real record — step 10b's first audit round, item 10.
        let refused = |arguments: &[&str]| {
            throwaway::an_argument_that_names_no_folder(arguments.iter().map(OsString::from))
        };
        assert_eq!(refused(&["--throwaway-copy="]), Some("--throwaway-copy=".into()));
        assert_eq!(refused(&["--throwaway-copy"]), Some("--throwaway-copy".into()));
        assert_eq!(refused(&["--throwaway-copy", ""]), Some("--throwaway-copy".into()));
        assert_eq!(
            refused(&[&format!("--throwaway_copy={text}")]),
            Some(format!("--throwaway_copy={text}")),
            "a misspelling names no folder this copy honours"
        );
        assert_eq!(refused(&["--throwawaycopy", &text]), Some("--throwawaycopy".into()));
        // Both spellings with a folder, and a line with none of it, start.
        assert_eq!(refused(&[&format!("--throwaway-copy={text}")]), None);
        assert_eq!(refused(&["--throwaway-copy", &text]), None);
        assert_eq!(refused(&["--other", "value"]), None);
        assert!(throwaway::the_refusal("--throwaway-copy=").contains("--throwaway-copy="));

        // A relative folder is made absolute rather than refused quietly.
        let relative = named(&["--throwaway-copy=a-throwaway-copy"]).expect("a folder was named");
        assert!(relative.is_absolute());
        assert!(relative.ends_with("a-throwaway-copy"));
    }

    /// **Every area of a throwaway copy sits inside the folder it was pointed
    /// at, and so does its home directory** — which is what makes the whole of
    /// it disposable by deleting one folder.
    #[cfg(debug_assertions)]
    #[test]
    fn everything_a_throwaway_copy_holds_sits_in_the_one_folder() {
        let folder = an_absolute_folder();
        let root = throwaway::the_application_data_root_under(&folder);
        let home = throwaway::the_user_home_under(&folder);

        assert!(root.starts_with(&folder));
        assert!(home.starts_with(&folder));
        // L3's shape is kept: the root still ends in Smoozy's own folder name.
        assert_eq!(root.file_name().and_then(|name| name.to_str()), Some(FOLDER));
        // The home directory is not inside the record's root, and `~/.ssh`
        // under it is not the Creator's.
        assert!(!home.starts_with(&root));
        assert!(!root.starts_with(&home));

        for location in [
            Location::LocalRecord,
            Location::DiaryArea,
            Location::ThrowawayKnownHostsArea,
            Location::TemporaryArea,
            Location::FetchedDerivativeArea,
            Location::StagedTransferArea,
            Location::TheRunningCopysHold,
            Location::TheRunningCopysDoor,
        ] {
            let path = the_area(location, root.clone());
            assert!(
                path.starts_with(&folder),
                "{location:?} sat outside the throwaway folder"
            );
        }
    }

    /// The Linux route's own rule: a relative value in the specification's
    /// variable is ignored rather than joined — *"If an implementation
    /// encounters a relative path in any of these variables it should consider
    /// the path invalid and ignore it."*
    ///
    /// **What is absolute is a question about a system and not about a
    /// string**, which the Windows runner said plainly: this test asserted
    /// that `/absolute/share` is absolute on every system, and on Windows it
    /// is not — a path is absolute there only with a volume. So the relative
    /// half is checked everywhere and the absolute half is checked in that
    /// system's own spelling.
    #[test]
    fn a_relative_specification_value_is_ignored() {
        assert!(!PathBuf::from("relative/share").is_absolute());
        assert!(!PathBuf::from("").is_absolute());

        #[cfg(unix)]
        assert!(PathBuf::from("/absolute/share").is_absolute());
        #[cfg(windows)]
        assert!(PathBuf::from(r"C:\absolute\share").is_absolute());
    }

    /// **Z8's installed path is read from where the program runs, and the two
    /// places beside it are composed from it alone** (F11, E18): a program
    /// inside `Something.app/Contents/MacOS/` names that bundle, and a program
    /// anywhere else names none.
    #[test]
    fn the_installed_bundle_is_the_one_the_program_runs_from() {
        let program = PathBuf::from("/Applications/Smoozy.app/Contents/MacOS/smoozy-desktop");
        let installed = the_bundle_a_program_sits_in(&program).expect("a bundle");
        assert_eq!(installed, PathBuf::from("/Applications/Smoozy.app"));
        assert_eq!(
            beside_the_installed_bundle(Location::TheUpdatesStagingFolder, &installed),
            PathBuf::from("/Applications/.Smoozy update")
        );
        assert_eq!(
            beside_the_installed_bundle(Location::ThePreviousBundleKept, &installed),
            PathBuf::from("/Applications/.Smoozy previous.app")
        );
        assert_eq!(beside_the_installed_bundle(Location::TheInstalledBundle, &installed), installed);
        for elsewhere in [
            "/usr/local/bin/smoozy-desktop",
            "/Applications/Smoozy/Contents/MacOS/smoozy-desktop",
            "/Applications/Smoozy.app/Contents/Resources/smoozy-desktop",
            "/Applications/.app/Contents/MacOS/smoozy-desktop",
        ] {
            assert_eq!(the_bundle_a_program_sits_in(&PathBuf::from(elsewhere)), None, "{elsewhere}");
        }
        // A test program is not inside a bundle, so nothing is named here.
        assert!(resolve(Location::TheInstalledBundle).is_err());
    }
}
