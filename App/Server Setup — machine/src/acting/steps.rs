//! **The rest of Y7's machine-side list, performed: the unpack, the two `ldd`
//! reads, the publication, and the final verification.**
//!
//! **Every one of them is a process run on the machine**, so they sit behind
//! one seam — [`RunsOneCommandOnTheMachine`] — whose real end is the program a
//! rented machine runs. **At the desk the stand-ins are named as stand-ins**:
//! this desk is a Mac with no `ldd` and no Linux Blender, and a test that
//! pretended otherwise would prove nothing about either.
//!
//! **Nothing here is an SSH command string**, so `Server protocol and
//! transfers` §1.1's admissibility rule is not what binds. Each is an
//! invocation the worker makes itself with its own arguments, which is the same
//! ground `Server Setup`'s own §6 entry stands on: *"each is an invocation of
//! the build with its own arguments, which are values of user and machine
//! origin and so ride the worker rather than a command string."*
//!
//! **Markers used in this file that belong to another document, each with the
//! document it belongs to.** `Server protocol and transfers` §7.1.

use std::path::{Path, PathBuf};
use std::time::Duration;

use crate::blender::{read_the_version_answer, Version, THE_TIME_LIMIT_EVERY_BLENDER_RUN_CARRIES};

/// **How one run of one program went.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HowItWent {
    /// **The exit status**, absent where none was reaped — which is what a run
    /// killed at its time limit leaves.
    pub exit: Option<i32>,
    /// **The ordinary stream, whole.**
    pub ordinary: String,
    /// **The error stream, whole, and kept apart from the other**
    /// (`Security and licence` §13 rule 5: never merged, never mined for a
    /// cause).
    pub error: String,
    /// **Whether the run was killed at its time limit** — `Server Setup` B1's
    /// *"something outside Blender ready to kill it"*.
    pub it_was_killed_at_its_limit: bool,
}

impl HowItWent {
    /// **Whether the program said it succeeded.** **Not whether the act
    /// worked** — `Server Setup` B7 clause 3 is explicit that no exit status is
    /// read as an unpack having worked, and the version ask is what catches a
    /// wrongly-unpacked archive.
    pub fn it_exited_cleanly(&self) -> bool {
        self.exit == Some(0)
    }
}

/// **The one thing this crate cannot do itself: run a program on the
/// machine.**
///
/// **Its real end is the machine's program.** The time limit is honoured by the
/// implementation, and what the kill reaches is `Server Setup` B1's rule:
/// **the invoked process and nothing above it** — never the setup worker, never
/// a shell around it, never anything found by name or pattern.
pub trait RunsOneCommandOnTheMachine {
    fn run(
        &self,
        program: &str,
        arguments: &[&str],
        a_time_limit: Option<Duration>,
    ) -> Result<HowItWent, String>;

    /// **The same, with `fed` handed to the program's standard input, whole,
    /// and the input then closed** — Y7c: the worker feeds the staged archive
    /// to the unpacker itself, so what it counts is what the unpacker has
    /// read. **No time limit**: the one program fed is the unpack, which
    /// carries none. Added 2026-10-18, the Creator's fourteenth notes,
    /// piece U.
    ///
    /// **A program that ends before it has read the whole** is answered with
    /// what it printed and how it exited, which is its own account of why;
    /// **a feed that would not read** is said in Smoozy's own words.
    fn run_fed(
        &self,
        program: &str,
        arguments: &[&str],
        fed: &mut dyn std::io::Read,
    ) -> Result<HowItWent, String>;

    /// **[`Self::run`], with what the program prints handed on as it prints**
    /// — `Server Setup` Y7a item 2: *"the capture streams: the worker passes
    /// each stream's bytes to the supervisor as they arrive, apart, in the
    /// order the commands ran, the part of a line not yet ended included."*
    /// The answer is the same whole one. Added 2026-10-25, the eighteenth
    /// notes, piece 2b.
    ///
    /// **This default hands each stream on whole once the program has
    /// ended**, which is all a stand-in can do; the machine's own tools
    /// override it and hand each piece on the moment it is read.
    fn run_handing_on(
        &self,
        program: &str,
        arguments: &[&str],
        a_time_limit: Option<Duration>,
        on: &dyn HandsOnWhatIsPrinted,
    ) -> Result<HowItWent, String> {
        let went = self.run(program, arguments, a_time_limit)?;
        on.printed(TheStepsStream::TheOrdinary, went.ordinary.as_bytes());
        on.printed(TheStepsStream::TheErrors, went.error.as_bytes());
        Ok(went)
    }

    /// **[`Self::run_fed`], with what the program prints handed on as it
    /// prints**, the same way as [`Self::run_handing_on`]. Added 2026-10-25.
    fn run_fed_handing_on(
        &self,
        program: &str,
        arguments: &[&str],
        fed: &mut dyn std::io::Read,
        on: &dyn HandsOnWhatIsPrinted,
    ) -> Result<HowItWent, String> {
        let went = self.run_fed(program, arguments, fed)?;
        on.printed(TheStepsStream::TheOrdinary, went.ordinary.as_bytes());
        on.printed(TheStepsStream::TheErrors, went.error.as_bytes());
        Ok(went)
    }
}

// ------------------------------------------- what a step prints, as it prints (Y7a item 2)

/// **Which of a step's two streams some words came on.** Added 2026-10-25,
/// the eighteenth notes, piece 2b.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TheStepsStream {
    /// The ordinary stream — a program's standard output, and the worker's
    /// own progress lines (`Server Setup` Y7b, Y7c).
    TheOrdinary,
    /// The error stream — a program's complaint output, and a failing step's
    /// own sentence (`Server Setup` S2a, Y7a item 2).
    TheErrors,
}

/// **Where a step's words go the moment they are printed** — `Server Setup`
/// Y7a item 2. **Its one real end is the machine's program**, which passes
/// them to the supervisor; it is a seam so the order and the words can be
/// proved at the desk. Added 2026-10-25, the eighteenth notes, piece 2b.
///
/// **It is shared across threads**, because a program's two streams are read
/// on threads of their own, and **it is never asked to decide anything**: it
/// is given bytes, in the order they were printed on their stream, and keeps
/// them apart by stream. Nothing here reads a cause off them (`Security and
/// licence` §13 rule 5).
pub trait HandsOnWhatIsPrinted: Sync {
    fn printed(&self, stream: TheStepsStream, bytes: &[u8]);
}

/// **Nobody is handed anything**: every caller that keeps no running words.
pub struct NothingIsHandedOn;

impl HandsOnWhatIsPrinted for NothingIsHandedOn {
    fn printed(&self, _stream: TheStepsStream, _bytes: &[u8]) {}
}

// ------------------------------------------- what every command printed (Y7a)

/// **What one command a worker step ran printed**, both streams whole,
/// unedited and apart — `Server Setup` Y7a item 2. Added 2026-10-24, the
/// Creator's ruling on the sixteenth notes, note 1.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct WhatOneCommandPrinted {
    /// The ordinary stream, whole.
    pub ordinary: String,
    /// The error stream, whole, and never merged with the other.
    pub error: String,
}

/// **Every command a step runs, run through the one seam, and what each
/// printed kept in the order they ran** — `Server Setup` Y7a item 2: *"The
/// worker reports every command's two streams with the step's report, apart,
/// in the order the commands ran."*
///
/// **Why one wrapper rather than each step handing its runs back.** Every
/// program a worker step starts leaves through [`RunsOneCommandOnTheMachine`]
/// — the unpack, `ldd`, the loader lookups beside it (R1 item 4's second
/// reading), a build asked its version, and both halves of the final check.
/// **Keeping the words at the seam keeps them for every one**, including a
/// command a later step adds, and leaves each step's own reading of its answer
/// exactly as it was. Until 2026-10-24 only the unpack, the library reads and
/// the publication attached words, and only as S2a's one composed text; the
/// loader lookups and the final check's two runs handed back none.
///
/// **A program that could not be started printed nothing** and adds nothing;
/// its refusal is the step's own words, as before.
pub struct EveryCommandsWordsAreKept<'a> {
    runs: &'a dyn RunsOneCommandOnTheMachine,
    kept: std::cell::RefCell<Vec<WhatOneCommandPrinted>>,
    /// **Where each command's words go as it prints them** (`Server Setup`
    /// Y7a item 2, from 2026-10-25). Nowhere for [`Self::around`].
    on: &'a dyn HandsOnWhatIsPrinted,
}

impl<'a> EveryCommandsWordsAreKept<'a> {
    pub fn around(runs: &'a dyn RunsOneCommandOnTheMachine) -> Self {
        EveryCommandsWordsAreKept::around_handing_on(runs, &NothingIsHandedOn)
    }

    /// **The same, with every command's words also handed on as they print**
    /// — `Server Setup` Y7a item 2. Added 2026-10-25, the eighteenth notes,
    /// piece 2b.
    pub fn around_handing_on(runs: &'a dyn RunsOneCommandOnTheMachine, on: &'a dyn HandsOnWhatIsPrinted) -> Self {
        EveryCommandsWordsAreKept {
            runs,
            kept: std::cell::RefCell::new(Vec::new()),
            on,
        }
    }

    /// **Whether the last command's error stream ended part-way through a
    /// line** — so a sentence printed after it starts a line of its own.
    pub fn the_error_stream_ends_mid_line(&self) -> bool {
        self.kept
            .borrow()
            .iter()
            .rev()
            .find(|one| !one.error.is_empty())
            .is_some_and(|one| !one.error.ends_with('\n'))
    }

    /// **What every command printed, in the order they ran.**
    pub fn what_they_printed(self) -> Vec<WhatOneCommandPrinted> {
        self.kept.into_inner()
    }

    fn kept(&self, went: Result<HowItWent, String>) -> Result<HowItWent, String> {
        if let Ok(went) = &went {
            self.kept.borrow_mut().push(WhatOneCommandPrinted {
                ordinary: went.ordinary.clone(),
                error: went.error.clone(),
            });
        }
        went
    }
}

impl RunsOneCommandOnTheMachine for EveryCommandsWordsAreKept<'_> {
    fn run(
        &self,
        program: &str,
        arguments: &[&str],
        a_time_limit: Option<Duration>,
    ) -> Result<HowItWent, String> {
        self.kept(self.runs.run_handing_on(program, arguments, a_time_limit, self.on))
    }

    fn run_fed(
        &self,
        program: &str,
        arguments: &[&str],
        fed: &mut dyn std::io::Read,
    ) -> Result<HowItWent, String> {
        self.kept(self.runs.run_fed_handing_on(program, arguments, fed, self.on))
    }
}

// ---------------------------------------------------------------- the unpack

/// **The one unpack tool Blender's documentation names** (Row 115), and
/// nothing else. **Every version's manual names TAR as required and 7-zip as
/// unsupported**, and two passes of 7-zip over each archive reported success
/// while losing every symbolic link and the runnable mark (`Server Setup` B7b,
/// *Builds 10*).
pub const THE_UNPACK_TOOL: &str = "tar";

/// **`Server Setup` R1 item 3's second half: the archive unpacked into staging
/// by that tool and nothing else.**
///
/// **The tool is installed where absent by the elevated half of the same
/// step**, which is the desktop's. On a machine without it the failure is loud
/// and total in all four versions — exit code 2 with nothing landing on disk
/// (`Server Setup` B7b, observed) — **so there is no half-unpacked state for
/// the re-run rule to meet.**
///
/// **A clean exit is not read as the unpack having worked** (`Server Setup` B7
/// clause 3): the wrong tool reported success and the result would not start.
/// What catches that is [`the_build_is_published`]'s version ask, which is owed
/// anyway.
///
/// **Y7c: the worker feeds the archive to the tool itself and counts the bytes
/// fed**, beside the archive's own size, so the step can say how far it has
/// got. The count is of the archive read and never of files written, so it
/// needs no fact about what the archive holds. Changed 2026-10-18, the
/// Creator's fourteenth notes, piece U: until then the tool read the archive
/// by its path and nobody could say how far it had got.
///
/// **What the tool is told to undo is read off the archive's first bytes**
/// ([`the_unpack_tools_option_for`]) and named to it, rather than left to the
/// tool to find on its standard input. **That GNU tar, the machines' own,
/// unpacks a fed archive with the option named has not been watched at this
/// desk**, whose tar is bsdtar; a pass on a rented machine owes it. Where the
/// first bytes are none of the marks, no option is given and the tool reads
/// the archive as it stands, saying in its own words if it cannot.
pub fn the_archive_is_unpacked(
    runner: &dyn RunsOneCommandOnTheMachine,
    archive: &Path,
    into: &Path,
    told: &dyn crate::acting::TellsHowFarItHasGot,
) -> Result<HowItWent, String> {
    use std::io::Read;
    std::fs::create_dir_all(into).map_err(|why| format!("{}: {why}", into.display()))?;
    let the_archives_size = std::fs::metadata(archive)
        .map_err(|why| format!("{}: {why}", archive.display()))?
        .len();
    let mut file = std::fs::File::open(archive).map_err(|why| format!("{}: {why}", archive.display()))?;
    let mut first = [0u8; THE_LONGEST_MARK];
    let mut looked = 0;
    while looked < first.len() {
        match file.read(&mut first[looked..]) {
            Ok(0) => break,
            Ok(read) => looked += read,
            Err(why) => return Err(format!("{}: {why}", archive.display())),
        }
    }
    let mut arguments = vec!["-x"];
    if let Some(option) = the_unpack_tools_option_for(&first[..looked]) {
        arguments.push(option);
    }
    let into = into.display().to_string();
    arguments.extend(["-f", "-", "-C", &into]);
    let mut fed = TheArchiveAsItIsFed {
        looked_at: std::io::Cursor::new(first[..looked].to_vec()),
        rest: file,
        fed: 0,
        whole: the_archives_size,
        told,
    };
    runner.run_fed(THE_UNPACK_TOOL, &arguments, &mut fed)
}

/// **The longest of the marks [`the_unpack_tools_option_for`] reads.**
const THE_LONGEST_MARK: usize = 6;

/// **What the unpack tool is told the archive is wrapped in, from the
/// archive's first bytes** — each wrapping's own published mark — in the
/// tool's own option for each. **Nothing where the first bytes are none of
/// them**: a plain archive needs no option, and anything else is the tool's to
/// refuse.
///
/// Blender's own archives are the first of these; the others are here because
/// the user's own build (B7) may come in any wrapping the tool undoes, and
/// until 2026-10-18 the tool found the wrapping for itself.
pub fn the_unpack_tools_option_for(first: &[u8]) -> Option<&'static str> {
    const THE_MARKS: [(&[u8], &str); 6] = [
        (&[0xFD, b'7', b'z', b'X', b'Z', 0x00], "-J"),
        (&[0x1F, 0x8B], "-z"),
        (b"BZh", "-j"),
        (&[0x28, 0xB5, 0x2F, 0xFD], "--zstd"),
        (b"LZIP", "--lzip"),
        (&[0x5D, 0x00, 0x00], "--lzma"),
    ];
    THE_MARKS
        .iter()
        .find(|(mark, _)| first.starts_with(mark))
        .map(|(_, option)| *option)
}

/// **The staged archive, as the unpack tool reads it** — the first bytes
/// already looked at, then the rest of the file, and every piece counted as it
/// goes (Y7c).
struct TheArchiveAsItIsFed<'a> {
    looked_at: std::io::Cursor<Vec<u8>>,
    rest: std::fs::File,
    fed: u64,
    whole: u64,
    told: &'a dyn crate::acting::TellsHowFarItHasGot,
}

impl std::io::Read for TheArchiveAsItIsFed<'_> {
    fn read(&mut self, into: &mut [u8]) -> std::io::Result<usize> {
        let read = match self.looked_at.read(into)? {
            0 => self.rest.read(into)?,
            read => read,
        };
        if read > 0 {
            self.fed += read as u64;
            self.told.so_far(self.fed, Some(self.whole));
        }
        Ok(read)
    }
}

/// **What the executable inside a Blender's own folder is called.**
///
/// **One name, one place.** [`the_staged_executable`] looks for it under the
/// staging folder and [`the_published_executable`] names it under the published
/// folder, and a second spelling of it would be the one-fact-in-two-places
/// fault this stage keeps paying for. Added 2026-09-23.
pub const THE_EXECUTABLES_NAME: &str = "blender";

/// **Where the unpacked build's own executable sits, found rather than
/// composed.**
///
/// **The archive unpacks into one folder of its own naming** and this project
/// has no reading of what that folder is called in every version, so the
/// executable is found by walking the staging folder for a file named
/// `blender` **at the depth the archive puts it** and never by composing a name
/// from the version. A staging folder holding none, or more than one, is said
/// so rather than guessed at.
pub fn the_staged_executable(staging: &Path) -> Result<PathBuf, String> {
    let mut found = Vec::new();
    let entries = std::fs::read_dir(staging).map_err(|why| format!("{}: {why}", staging.display()))?;
    for entry in entries {
        let at = entry
            .map_err(|why| format!("{}: {why}", staging.display()))?
            .path();
        if !at.is_dir() {
            continue;
        }
        let candidate = at.join(THE_EXECUTABLES_NAME);
        if candidate.is_file() {
            found.push(candidate);
        }
    }
    match found.len() {
        1 => Ok(found.remove(0)),
        0 => Err(format!(
            "{}: no blender executable in the archive",
            staging.display()
        )),
        how_many => Err(format!(
            "{}: {how_many} blender executables in the archive",
            staging.display()
        )),
    }
}

// ------------------------------------------------------------- the libraries

/// **The tool `Server Setup` R1 item 4 names, and the only reading this step
/// takes.**
pub const THE_LIBRARY_CHAIN_TOOL: &str = "ldd";

/// **What the mark on a line that did not resolve is**, which is what the whole
/// list is read by. `Server Setup` R1 item 4's own entry quotes the machine:
/// *"libX11.so.6: cannot open shared object file"*, and `ldd`'s own line for
/// one is the library, an arrow, and this.
pub const WHAT_AN_UNRESOLVED_LINE_SAYS: &str = "not found";

/// **`Server Setup` R1 item 4's read: `ldd` over the executable the machine
/// will render with, and the whole not-found list at once.**
///
/// **Never one library at a time, never from the launch error, never left to
/// the driver's dependencies** (Fact 47, `Server Setup` R5). The list is a
/// property of the image — one image needed ten packages, another could not
/// satisfy nine distinct libraries across ten lines — **so nothing here is a
/// list to hard-code**, and reading the whole of it at once is what makes the
/// difference between images cost nothing.
pub fn the_library_chain_is_read(
    runner: &dyn RunsOneCommandOnTheMachine,
    executable: &Path,
) -> Result<(HowItWent, Vec<String>), String> {
    let went = runner.run(
        THE_LIBRARY_CHAIN_TOOL,
        &[&executable.display().to_string()],
        None,
    )?;
    let mut not_found = the_not_found_list(&went.ordinary);
    for opened in crate::ready_action::WHAT_BLENDER_OPENS_BY_NAME {
        if !not_found.iter().any(|named| named == opened.library)
            && the_loader_does_not_find(runner, &opened)
        {
            not_found.push(opened.library.to_string());
        }
    }
    Ok((went, not_found))
}

/// **The program the shared lookup line is handed to** — the POSIX shell,
/// which runs it as one line (`crate::the_optix_library`'s head).
pub const THE_SHELL: &str = "sh";

/// **R1 item 4's second reading, added 2026-10-11: a library Blender opens by
/// name while it runs, asked of the machine's loader**, because `ldd` lists
/// only what the executable links and never names it.
///
/// **Only *not found* adds it.** A lookup that gives no answer — a shell that
/// could not be started, or a status `grep` gives for an error — says nothing
/// about the library, and nothing is installed on nothing known; the step is
/// then exactly what it was before this reading existed.
fn the_loader_does_not_find(
    runner: &dyn RunsOneCommandOnTheMachine,
    opened: &crate::ready_action::OpenedByName,
) -> bool {
    use crate::the_optix_library::{what_the_status_says_about, WhatTheLoaderFound};
    match runner.run(THE_SHELL, &["-c", opened.the_lookup], None) {
        Ok(went) => {
            what_the_status_says_about(opened.library, went.exit) == WhatTheLoaderFound::NoLibrary
        }
        Err(_) => false,
    }
}

/// **The whole not-found list, out of the tool's own printing.**
///
/// **A library is named once however many lines mention it**, because the
/// packages are installed by name and a repeated name would be a repeated
/// install rather than a second library.
pub fn the_not_found_list(printed: &str) -> Vec<String> {
    let mut found: Vec<String> = Vec::new();
    for line in printed.lines() {
        if !line.contains(WHAT_AN_UNRESOLVED_LINE_SAYS) {
            continue;
        }
        let Some(named) = line.split_whitespace().next() else {
            continue;
        };
        if !found.iter().any(|already| already == named) {
            found.push(named.to_string());
        }
    }
    found
}

/// **`Server Setup` R1 item 4's re-read, which must come back clean.**
///
/// **Clean means the list is empty**, and nothing else is read as clean — not
/// the exit status, which `ldd` sets for its own reasons, and not a shorter
/// list than before.
pub fn the_library_chain_reads_clean(
    runner: &dyn RunsOneCommandOnTheMachine,
    executable: &Path,
) -> Result<(HowItWent, Vec<String>, bool), String> {
    let (went, not_found) = the_library_chain_is_read(runner, executable)?;
    let clean = not_found.is_empty();
    Ok((went, not_found, clean))
}

// ----------------------------------------------------------- the publication

/// **The switch a build is asked its version with** — `Server Setup` B1: *"a
/// build is asked its version by running it with its version switch"*.
pub const THE_VERSION_SWITCH: &str = "--version";

/// **What the publication step did.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WhatThePublicationDid {
    /// **The build answered the expected version in staging and was then
    /// renamed to its final folder** — `Server Setup` R1 item 7's order, which
    /// is what makes the step re-runnable.
    ItWasPublished { at: PathBuf, answered: Version },
    /// **The build answered a different version, or none, and nothing was
    /// renamed.** The staged material stays where it is, at no final name.
    ItAnsweredWrongly {
        expected: Version,
        answered: Option<Version>,
    },
    /// **The version answered but the rename was refused**, its own words
    /// carried. `Server Setup` B8 never publishes over an existing folder, and
    /// a folder already there is this case rather than a silent replacement.
    TheRenameWasRefused { why: String },
}

/// **`Server Setup` R1 item 7, whole: the staged build asked its version in
/// staging under `Server Setup` B1's discipline, required to answer the
/// expected one, and only then renamed to its final folder.**
///
/// **The order is the contract and not a preference.** *"The rename consumes
/// the staged material, so a version ask placed after it would leave a re-run
/// nothing to re-validate by content"*, and `Server Setup` B8 forbids a rename
/// over an existing folder. `Server protocol and transfers` §7.1 clause 3 is
/// the same rule from the other side: publication is the last step, and it is
/// one step.
///
/// **The version is read from the line beginning `Blender `, never the first
/// line** (`Server Setup` B1; the apt build printed two lines of colour-
/// management noise first), and **both streams are captured** — the refusal to
/// open a window goes to the error stream in one version and the ordinary
/// stream in the other three.
///
/// **The run carries `Server Setup` B1's time limit**, applied to all four
/// versions because the step cannot know a candidate's version before asking.
pub fn the_build_is_published(
    runner: &dyn RunsOneCommandOnTheMachine,
    staged_executable: &Path,
    expected: Version,
    the_folder_it_is_published_at: &Path,
) -> Result<(HowItWent, WhatThePublicationDid), String> {
    let went = runner.run(
        &staged_executable.display().to_string(),
        &[THE_VERSION_SWITCH],
        Some(THE_TIME_LIMIT_EVERY_BLENDER_RUN_CARRIES),
    )?;
    // B1: a run killed at its limit answers nothing and is recorded as
    // answering nothing.
    let answered = if went.it_was_killed_at_its_limit {
        None
    } else {
        read_the_version_answer(&went.ordinary).or_else(|| read_the_version_answer(&went.error))
    };
    if answered != Some(expected) {
        return Ok((went, WhatThePublicationDid::ItAnsweredWrongly { expected, answered }));
    }
    // The staged build's own folder is what is renamed, not the executable.
    let Some(staged_folder) = staged_executable.parent() else {
        return Ok((
            went,
            WhatThePublicationDid::TheRenameWasRefused {
                why: "the staged executable has no folder of its own".to_string(),
            },
        ));
    };
    if the_folder_it_is_published_at.exists() {
        return Ok((
            went,
            WhatThePublicationDid::TheRenameWasRefused {
                why: format!(
                    "{} already holds a build. Not replaced.",
                    the_folder_it_is_published_at.display()
                ),
            },
        ));
    }
    if let Some(area) = the_folder_it_is_published_at.parent() {
        if let Err(why) = std::fs::create_dir_all(area) {
            return Ok((
                went,
                WhatThePublicationDid::TheRenameWasRefused {
                    why: format!("{}: {why}", area.display()),
                },
            ));
        }
    }
    match std::fs::rename(staged_folder, the_folder_it_is_published_at) {
        Ok(()) => Ok((
            went,
            WhatThePublicationDid::ItWasPublished {
                at: the_folder_it_is_published_at.to_path_buf(),
                answered: expected,
            },
        )),
        Err(why) => Ok((
            went,
            WhatThePublicationDid::TheRenameWasRefused {
                why: format!(
                    "{} to {}: {why}",
                    staged_folder.display(),
                    the_folder_it_is_published_at.display()
                ),
            },
        )),
    }
}

/// **Where the published build's own executable sits.**
///
/// **`Server Setup` R1 item 7 renames the staged build's own folder to the
/// published folder**, so the executable that sat at `<staged folder>/blender`
/// sits at `<published folder>/blender` and nowhere else. **Nothing is walked
/// and nothing is parsed**: the publication moved a folder whose contents it
/// did not touch, so the name is the one [`the_staged_executable`] found by
/// looking.
///
/// **Added 2026-09-23, out of step 10c's fifth pass on Vast**, where the final
/// check ran the build at the staging path the publication had just taken away
/// and every Blender install failed its last step with *No such file or
/// directory* after everything had worked.
pub fn the_published_executable(the_published_folder: &Path) -> PathBuf {
    the_published_folder.join(THE_EXECUTABLES_NAME)
}

// ----------------------------------------------------- the final verification

/// **What `Server Setup` R1 item 9's final verification found.**
///
/// **Each part is present only where the run contained the step it verifies**,
/// which is `Server Setup` R4's whole rule: *"where there was no Blender to
/// read one against, the library half was not read and *ready* claims nothing
/// about it."*
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WhatTheFinalVerificationFound {
    /// **`ldd` clean**, where a build was there to read one against.
    pub the_library_chain_is_clean: Option<bool>,
    /// **The published build answers**, where this run published one.
    pub the_published_build_answers: Option<bool>,
}

/// **What the final verification is entitled to claim, and it is never more
/// than it read.**
///
/// **Three answers rather than two, because an absent half is neither a pass
/// nor a failure.** `Server Setup` R4: *"where there was no Blender to read one
/// against, the library half was not read and *ready* claims nothing about
/// it."* **Added 2026-09-23, out of step 10c's fifth pass on Vast**, where a
/// recovery re-ran the final check with both halves absent, a two-answer
/// verification called that a pass, and the screen said *The server is ready to
/// render* over a reading nobody had taken.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhatTheVerificationClaims {
    /// **At least one half had a subject and every half that had one passed.**
    ItPassed,
    /// **A half that had a subject did not pass**, and
    /// [`WhatTheFinalVerificationFound::what_did_not_pass`] says which.
    ItDidNotPass,
    /// **No half had a subject: nothing was read, so nothing is claimed.**
    /// What the caller may do with that is the caller's, and it depends on
    /// whether the run contained a step whose subject this was.
    NothingWasRead,
}

impl WhatTheFinalVerificationFound {
    /// **What this reading entitles anybody to claim**, which is `Server Setup`
    /// R4's rule as a value.
    ///
    /// **A part that did not run is not a pass and not a failure**; it is
    /// absent. **This used to answer a plain yes or no with an absent half
    /// counted as a pass**, which made a check of nothing report a pass — step
    /// 10c's fifth pass on Vast watched it happen on a real machine.
    pub fn what_it_claims(&self) -> WhatTheVerificationClaims {
        if self.the_library_chain_is_clean == Some(false)
            || self.the_published_build_answers == Some(false)
        {
            return WhatTheVerificationClaims::ItDidNotPass;
        }
        match (
            self.the_library_chain_is_clean,
            self.the_published_build_answers,
        ) {
            (None, None) => WhatTheVerificationClaims::NothingWasRead,
            _ => WhatTheVerificationClaims::ItPassed,
        }
    }

    /// **Which half did not pass, in words a person reads.**
    ///
    /// **`Server Setup` S2a carries these words to the desktop and the desktop
    /// shows them whole behind *What the server said***, so they are read by a
    /// person and never by a program. Added 2026-09-22, after step 10c's fourth
    /// pass on Vast put *"the library chain read clean Some(false), the
    /// published build answered Some(false)"* in front of the user: the two
    /// readings had been written out as this language renders a value for a
    /// programmer.
    ///
    /// **A half that did not run says nothing**, which is `Server Setup` R4's
    /// own rule: it is absent rather than failed, and a sentence naming it
    /// would claim a reading nobody took.
    pub fn what_did_not_pass(&self) -> String {
        let mut halves: Vec<&str> = Vec::new();
        if self.the_library_chain_is_clean == Some(false) {
            halves.push("system libraries still missing");
        }
        if self.the_published_build_answers == Some(false) {
            halves.push("the installed Blender did not report its version");
        }
        match halves.len() {
            0 => "Final check failed.".to_string(),
            _ => format!("Final check failed: {}.", halves.join("; ")),
        }
    }
}

/// **`Server Setup` R1 item 9's final verification, over what the run actually
/// contained.**
///
/// **The driver's own answer is not read here**, because the driver step is
/// elevated and the desktop that ran it holds that reading (`Server Setup`
/// R1 item 5's check-first order); the worker verifies the two halves that are
/// its own.
///
/// **What its `ldd` and version runs printed reaches the step's report** by
/// the runner it is handed — [`EveryCommandsWordsAreKept`] around the
/// machine's own, from 2026-10-24 (`Server Setup` Y7a item 2). The reading
/// below still keeps only what it judges by.
pub fn the_final_verification(
    runner: &dyn RunsOneCommandOnTheMachine,
    the_executable_to_read_against: Option<&Path>,
    the_published_build: Option<&Path>,
) -> Result<WhatTheFinalVerificationFound, String> {
    let the_library_chain_is_clean = match the_executable_to_read_against {
        None => None,
        Some(executable) => {
            let (_, _, clean) = the_library_chain_reads_clean(runner, executable)?;
            Some(clean)
        }
    };
    let the_published_build_answers = match the_published_build {
        None => None,
        Some(executable) => {
            let went = runner.run(
                &executable.display().to_string(),
                &[THE_VERSION_SWITCH],
                Some(THE_TIME_LIMIT_EVERY_BLENDER_RUN_CARRIES),
            )?;
            Some(
                !went.it_was_killed_at_its_limit
                    && (read_the_version_answer(&went.ordinary).is_some()
                        || read_the_version_answer(&went.error).is_some()),
            )
        }
    };
    Ok(WhatTheFinalVerificationFound {
        the_library_chain_is_clean,
        the_published_build_answers,
    })
}

#[cfg(test)]
pub mod testing {
    //! **A stand-in for a Linux machine's own tools, named as one.**
    //!
    //! **Nothing it answers was observed.** It replays what a test writes into
    //! it, so what the tests below prove is the worker's order, its records and
    //! its resumption — never what `tar`, `ldd` or a Linux Blender actually do.

    use super::*;
    use std::cell::RefCell;

    /// One canned answer, matched on the program's name.
    pub struct AStandInForTheMachinesTools {
        pub answers: RefCell<Vec<(String, HowItWent)>>,
        pub asked: RefCell<Vec<(String, Vec<String>)>>,
    }

    impl AStandInForTheMachinesTools {
        pub fn holding(answers: Vec<(&str, HowItWent)>) -> Self {
            AStandInForTheMachinesTools {
                answers: RefCell::new(
                    answers
                        .into_iter()
                        .map(|(program, went)| (program.to_string(), went))
                        .collect(),
                ),
                asked: RefCell::new(Vec::new()),
            }
        }
    }

    pub fn it_went(exit: i32, ordinary: &str, error: &str) -> HowItWent {
        HowItWent {
            exit: Some(exit),
            ordinary: ordinary.to_string(),
            error: error.to_string(),
            it_was_killed_at_its_limit: false,
        }
    }

    impl RunsOneCommandOnTheMachine for AStandInForTheMachinesTools {
        fn run(
            &self,
            program: &str,
            arguments: &[&str],
            _a_time_limit: Option<Duration>,
        ) -> Result<HowItWent, String> {
            self.asked.borrow_mut().push((
                program.to_string(),
                arguments.iter().map(|it| it.to_string()).collect(),
            ));
            let mut answers = self.answers.borrow_mut();
            let at = answers
                .iter()
                .position(|(named, _)| program.ends_with(named.as_str()))
                .ok_or_else(|| format!("the stand-in has no answer for {program}"))?;
            Ok(answers.remove(at).1)
        }

        /// **The feed is read whole, as a tool would read it, and then the
        /// canned answer is given** — so what a test proves is the counting
        /// and the order, never what the tool does with the bytes.
        fn run_fed(
            &self,
            program: &str,
            arguments: &[&str],
            fed: &mut dyn std::io::Read,
        ) -> Result<HowItWent, String> {
            std::io::copy(fed, &mut std::io::sink()).map_err(|why| format!("{program}: {why}"))?;
            self.run(program, arguments, None)
        }
    }

    /// **What a Linux machine says about a path that is not there**, which is
    /// the sentence step 10c's fifth pass on Vast read off every failed Blender
    /// install: *"…/blender: No such file or directory (os error 2)"*.
    pub const WHAT_A_MACHINE_SAYS_ABOUT_A_PATH_THAT_IS_NOT_THERE: &str =
        "No such file or directory (os error 2)";

    /// **A stand-in that answers only for a path that is really there.**
    ///
    /// **[`AStandInForTheMachinesTools`] answers any path at all**, matching on
    /// the program's name alone — so a step handed a path that had been taken
    /// away got an answer as though the build were still at it. **That is why
    /// the desk never saw the fifth pass's first fault**: the final check ran
    /// the build at the staging path the publication had just renamed out of
    /// staging, and every test passed.
    ///
    /// **This one looks.** A program named by a path that is not a file on this
    /// disk, and a tool asked about such a path, are refused in the machine's
    /// own words — so a stale path fails the test rather than passing it. Added
    /// 2026-09-23.
    pub struct AStandInThatAnswersOnlyForAPathThatIsThere {
        pub answers: RefCell<Vec<(String, HowItWent)>>,
        pub asked: RefCell<Vec<(String, Vec<String>)>>,
    }

    impl AStandInThatAnswersOnlyForAPathThatIsThere {
        pub fn holding(answers: Vec<(&str, HowItWent)>) -> Self {
            AStandInThatAnswersOnlyForAPathThatIsThere {
                answers: RefCell::new(
                    answers
                        .into_iter()
                        .map(|(program, went)| (program.to_string(), went))
                        .collect(),
                ),
                asked: RefCell::new(Vec::new()),
            }
        }

        /// **A path this stand-in was asked about**, which is a program named by
        /// a path or an argument that names one.
        ///
        /// **Any separator this system writes a path with, and not only `/`.**
        /// Until 2026-09-26 this asked for `/` alone, and on Windows every path
        /// the test composes is written with `\` — so this stand-in never looked
        /// there, answered the stale path as though the build were still at it,
        /// and hosted run `36240566832` failed on `!stale.it_ran_to_its_end`.
        /// `std::path::is_separator` is `/` on Linux and macOS, and both `\`
        /// and `/` on Windows, which is what the standard library itself splits
        /// a path on.
        fn is_a_path(word: &str) -> bool {
            word.chars().any(std::path::is_separator)
        }
    }

    impl RunsOneCommandOnTheMachine for AStandInThatAnswersOnlyForAPathThatIsThere {
        fn run(
            &self,
            program: &str,
            arguments: &[&str],
            _a_time_limit: Option<Duration>,
        ) -> Result<HowItWent, String> {
            self.asked.borrow_mut().push((
                program.to_string(),
                arguments.iter().map(|it| it.to_string()).collect(),
            ));
            // **The program itself**, where it is named by a path: a build that
            // has been moved cannot be run where it used to be.
            if Self::is_a_path(program) && !Path::new(program).is_file() {
                return Err(format!(
                    "{program}: {WHAT_A_MACHINE_SAYS_ABOUT_A_PATH_THAT_IS_NOT_THERE}"
                ));
            }
            // **And what a tool was asked about** — `ldd` over a build that is
            // no longer at the path it is named by.
            for argument in arguments {
                if Self::is_a_path(argument) && !Path::new(argument).exists() {
                    return Err(format!(
                        "{argument}: {WHAT_A_MACHINE_SAYS_ABOUT_A_PATH_THAT_IS_NOT_THERE}"
                    ));
                }
            }
            let mut answers = self.answers.borrow_mut();
            let at = answers
                .iter()
                .position(|(named, _)| program.ends_with(named.as_str()))
                .ok_or_else(|| format!("the stand-in has no answer for {program}"))?;
            Ok(answers.remove(at).1)
        }

        /// The feed read whole, then the answer, as next door.
        fn run_fed(
            &self,
            program: &str,
            arguments: &[&str],
            fed: &mut dyn std::io::Read,
        ) -> Result<HowItWent, String> {
            std::io::copy(fed, &mut std::io::sink()).map_err(|why| format!("{program}: {why}"))?;
            self.run(program, arguments, None)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::testing::*;
    use super::*;

    fn a_version() -> Version {
        Version {
            major: 5,
            minor: 2,
            patch: 0,
        }
    }

    fn an_area(named: &str) -> crate::acting::ItsOwnFolder {
        crate::acting::its_own_folder(named)
    }

    /// **Y7c: the tool is told the archive's wrapping, read off its first
    /// bytes** — Blender's own `.tar.xz` gives `-J`, a gzip `-z`, and a plain
    /// archive or anything unknown nothing at all, left to the tool. Piece U.
    #[test]
    fn piece_u_the_unpack_tool_is_told_the_wrapping_its_first_bytes_name() {
        assert_eq!(the_unpack_tools_option_for(&[0xFD, b'7', b'z', b'X', b'Z', 0x00, 0x00]), Some("-J"));
        assert_eq!(the_unpack_tools_option_for(&[0x1F, 0x8B, 0x08]), Some("-z"));
        assert_eq!(the_unpack_tools_option_for(b"BZh91AY"), Some("-j"));
        assert_eq!(the_unpack_tools_option_for(b"blender-5.2.2-linux-x64/"), None);
        assert_eq!(the_unpack_tools_option_for(&[0xFD, b'7']), None, "a mark cut short is no mark");
        assert_eq!(the_unpack_tools_option_for(&[]), None);
    }

    /// **The whole not-found list at once, each library named once.**
    #[test]
    fn the_whole_not_found_list_is_read_and_a_library_is_named_once() {
        let printed = "\
\tlinux-vdso.so.1 (0x00007fff)
\tlibX11.so.6 => not found
\tlibXi.so.6 => not found
\tlibX11.so.6 => not found
\tlibc.so.6 => /lib/x86_64-linux-gnu/libc.so.6 (0x00007f00)
";
        assert_eq!(
            the_not_found_list(printed),
            vec!["libX11.so.6".to_string(), "libXi.so.6".to_string()]
        );
        assert!(the_not_found_list("\tlibc.so.6 => /lib/libc.so.6").is_empty());
    }

    /// **Clean means the list is empty and nothing else** — not the exit
    /// status, and not a shorter list than before.
    #[test]
    fn clean_is_an_empty_list_and_never_an_exit_status() {
        let runner = AStandInForTheMachinesTools::holding(vec![(
            "ldd",
            it_went(0, "\tlibX11.so.6 => not found\n", ""),
        )]);
        let (_, not_found, clean) =
            the_library_chain_reads_clean(&runner, Path::new("/a/blender")).expect("it ran");
        assert_eq!(not_found, vec!["libX11.so.6".to_string()]);
        assert!(!clean, "a clean exit over a not-found line is not clean");
    }

    // ------------------------------------- what Blender opens by name (E4)

    /// **What a clean `ldd` over 5.2.2 prints** — every linked library
    /// resolved, and `libEGL.so.1` nowhere in it, because Blender does not
    /// link it (`Server Setup` R1 item 4, the entry of 2026-10-11).
    const A_CLEAN_CHAIN: &str = "\
\tlinux-vdso.so.1 (0x00007fff)
\tlibX11.so.6 => /lib/x86_64-linux-gnu/libX11.so.6 (0x00007f00)
\tlibc.so.6 => /lib/x86_64-linux-gnu/libc.so.6 (0x00007f01)
";

    /// The bare container's nine, as `ldd` prints them.
    fn what_a_bare_container_printed() -> String {
        crate::ready_action::WHAT_A_BARE_CONTAINER_REPORTED
            .iter()
            .map(|library| format!("\t{library} => not found\n"))
            .collect()
    }

    /// **A clean `ldd`, and the loader does not find `libEGL.so.1`: the
    /// library joins the list, the step is not clean, and the entry is
    /// installed.** Today's step read this machine as clean and kept the gap
    /// (the Creator's eighth notes, note 5).
    #[test]
    fn a_clean_chain_with_the_loader_not_finding_egl_installs_the_entry() {
        let runner = AStandInForTheMachinesTools::holding(vec![
            ("ldd", it_went(0, A_CLEAN_CHAIN, "")),
            ("sh", it_went(1, "", "")),
        ]);
        let (_, not_found, clean) =
            the_library_chain_reads_clean(&runner, Path::new("/a/blender")).expect("it ran");
        assert_eq!(not_found, vec!["libEGL.so.1".to_string()]);
        assert!(!clean, "a library the loader cannot find is not clean");
        let crate::ready_action::WhatTheLibrariesStepInstalls::ThesePackages(packages) =
            crate::ready_action::what_to_install_for(&not_found)
        else {
            panic!("the entry is keyed on libEGL.so.1");
        };
        assert_eq!(packages, crate::ready_action::THE_MAPPING[0].packages.to_vec());
        assert!(packages.contains(&"libegl1"));
        // **The loader was asked by the one shared lookup, by the name alone.**
        let asked = runner.asked.borrow();
        let (program, arguments) = &asked[1];
        assert_eq!(program, "sh");
        assert_eq!(arguments[0], "-c");
        assert_eq!(
            arguments[1],
            crate::the_optix_library::THE_LOADERS_OWN_LOOKUP
                .replace(crate::the_optix_library::THE_LIBRARY, "libEGL.so.1")
        );
    }

    /// **A clean `ldd`, and the loader finds it: nothing to install.**
    #[test]
    fn a_clean_chain_with_the_loader_finding_egl_installs_nothing() {
        let runner = AStandInForTheMachinesTools::holding(vec![
            ("ldd", it_went(0, A_CLEAN_CHAIN, "")),
            ("sh", it_went(0, "", "")),
        ]);
        let (_, not_found, clean) =
            the_library_chain_reads_clean(&runner, Path::new("/a/blender")).expect("it ran");
        assert!(not_found.is_empty(), "{not_found:?}");
        assert!(clean);
        assert_eq!(
            crate::ready_action::what_to_install_for(&not_found),
            crate::ready_action::WhatTheLibrariesStepInstalls::NothingWasMissing
        );
        assert_eq!(runner.asked.borrow().len(), 2, "the loader was asked");
    }

    /// **A lookup that gives no answer adds nothing**: a status `grep` gives
    /// for an error, or a shell that could not be started. Nothing is known
    /// about the library, so nothing is installed for it.
    #[test]
    fn a_lookup_with_no_answer_adds_nothing() {
        let errored = AStandInForTheMachinesTools::holding(vec![
            ("ldd", it_went(0, A_CLEAN_CHAIN, "")),
            ("sh", it_went(2, "", "")),
        ]);
        let (_, not_found, clean) =
            the_library_chain_reads_clean(&errored, Path::new("/a/blender")).expect("it ran");
        assert!(not_found.is_empty() && clean, "{not_found:?}");

        let not_started =
            AStandInForTheMachinesTools::holding(vec![("ldd", it_went(0, A_CLEAN_CHAIN, ""))]);
        let (_, not_found, clean) =
            the_library_chain_reads_clean(&not_started, Path::new("/a/blender")).expect("it ran");
        assert!(not_found.is_empty() && clean, "{not_found:?}");
    }

    /// **The bare container's case is unchanged**: whatever the loader says,
    /// the nine are read and one entry's packages are selected.
    #[test]
    fn the_bare_containers_case_selects_the_same_one_entry_whatever_the_loader_says() {
        for (status, egl_joins) in [(0, false), (1, true)] {
            let runner = AStandInForTheMachinesTools::holding(vec![
                ("ldd", it_went(0, &what_a_bare_container_printed(), "")),
                ("sh", it_went(status, "", "")),
            ]);
            let (_, not_found, clean) =
                the_library_chain_reads_clean(&runner, Path::new("/a/blender")).expect("it ran");
            assert!(!clean);
            let mut expected: Vec<String> = crate::ready_action::WHAT_A_BARE_CONTAINER_REPORTED
                .iter()
                .map(|library| library.to_string())
                .collect();
            if egl_joins {
                expected.push("libEGL.so.1".to_string());
            }
            assert_eq!(not_found, expected, "status {status}");
            assert_eq!(
                crate::ready_action::what_to_install_for(&not_found),
                crate::ready_action::WhatTheLibrariesStepInstalls::ThesePackages(
                    crate::ready_action::THE_MAPPING[0].packages.to_vec()
                ),
                "status {status}"
            );
        }
    }

    /// **Where `ldd` itself names `libEGL.so.1`, the loader is not asked and
    /// the name is not doubled.**
    #[test]
    fn a_library_ldd_already_names_is_not_looked_up_again() {
        let runner = AStandInForTheMachinesTools::holding(vec![
            ("ldd", it_went(0, "\tlibEGL.so.1 => not found\n", "")),
            ("sh", it_went(1, "", "")),
        ]);
        let (_, not_found, _) =
            the_library_chain_reads_clean(&runner, Path::new("/a/blender")).expect("it ran");
        assert_eq!(not_found, vec!["libEGL.so.1".to_string()]);
        assert_eq!(runner.asked.borrow().len(), 1, "only ldd was run");
    }

    /// **`Server Setup` R1 item 7's order: the version is asked in staging, and
    /// only on that answer is anything renamed.**
    #[test]
    fn a_build_that_answers_wrongly_is_not_published() {
        let staging = an_area("a-staging-area");
        let unpacked = staging.at().join("blender-5.2.0-linux-x64");
        std::fs::create_dir_all(&unpacked).expect("the folder is made");
        std::fs::write(unpacked.join("blender"), b"").expect("the build is there");
        let area = an_area("a-blender-area");
        let at = area.at().join("smoozy-5.2.0");

        let runner = AStandInForTheMachinesTools::holding(vec![(
            "blender",
            it_went(0, "Color management: using fallback\nBlender 5.1.2\n", ""),
        )]);
        let (_, did) = the_build_is_published(&runner, &unpacked.join("blender"), a_version(), &at)
            .expect("it ran");
        assert_eq!(
            did,
            WhatThePublicationDid::ItAnsweredWrongly {
                expected: a_version(),
                answered: Some(Version {
                    major: 5,
                    minor: 1,
                    patch: 2
                }),
            }
        );
        assert!(!at.exists(), "nothing is renamed on a wrong answer");
        assert!(unpacked.exists(), "the staged material stays in staging");
    }

    /// **And on the expected answer the staged folder is renamed whole, once**
    /// — and never over an existing folder (`Server Setup` B8).
    #[test]
    fn a_build_that_answers_as_expected_is_published_and_never_over_an_existing_folder() {
        let staging = an_area("a-second-staging-area");
        let unpacked = staging.at().join("blender-5.2.0-linux-x64");
        std::fs::create_dir_all(&unpacked).expect("the folder is made");
        std::fs::write(unpacked.join("blender"), b"").expect("the build is there");
        let area = an_area("a-second-blender-area");
        let at = area.at().join("smoozy-5.2.0");

        let runner = AStandInForTheMachinesTools::holding(vec![(
            "blender",
            // B1: the line beginning `Blender `, never the first line.
            it_went(0, "Color management: using fallback\nBlender 5.2.0\n", ""),
        )]);
        let (_, did) = the_build_is_published(&runner, &unpacked.join("blender"), a_version(), &at)
            .expect("it ran");
        assert_eq!(
            did,
            WhatThePublicationDid::ItWasPublished {
                at: at.clone(),
                answered: a_version()
            }
        );
        assert!(at.join("blender").is_file());
        assert!(!unpacked.exists(), "the rename consumes the staged material");

        // A second run over a folder that is now there is refused rather than
        // replacing it.
        let again = staging.at().join("blender-5.2.0-linux-x64");
        std::fs::create_dir_all(&again).expect("the folder is made");
        std::fs::write(again.join("blender"), b"").expect("the build is there");
        let runner = AStandInForTheMachinesTools::holding(vec![(
            "blender",
            it_went(0, "Blender 5.2.0\n", ""),
        )]);
        let (_, did) =
            the_build_is_published(&runner, &again.join("blender"), a_version(), &at).expect("it ran");
        assert!(matches!(did, WhatThePublicationDid::TheRenameWasRefused { .. }));
    }

    /// **`Server Setup` R4: a part of the final verification that did not run
    /// is absent, and absence is neither a pass nor a failure.**
    ///
    /// **An empty reading never reports a pass. Rewritten 2026-09-23**: this
    /// test used to end `assert!(found.it_passed(), "nothing ran and nothing
    /// failed")`, which is the sentence that let step 10c's fifth pass on Vast
    /// put *The server is ready to render* on the screen over a check of
    /// nothing.
    #[test]
    fn the_final_verification_claims_nothing_about_a_half_that_did_not_run() {
        let runner = AStandInForTheMachinesTools::holding(vec![]);
        let found = the_final_verification(&runner, None, None).expect("it ran");
        assert_eq!(found.the_library_chain_is_clean, None);
        assert_eq!(found.the_published_build_answers, None);
        assert_eq!(
            found.what_it_claims(),
            WhatTheVerificationClaims::NothingWasRead,
            "nothing was read, so nothing is claimed"
        );
        assert_ne!(found.what_it_claims(), WhatTheVerificationClaims::ItPassed);
        assert!(runner.asked.borrow().is_empty(), "nothing was run");
    }

    /// **The three answers, one reading each**, so nothing can quietly become a
    /// pass again.
    #[test]
    fn what_the_verification_claims_is_cut_to_what_it_read() {
        let a_reading = |library, published| WhatTheFinalVerificationFound {
            the_library_chain_is_clean: library,
            the_published_build_answers: published,
        };
        assert_eq!(
            a_reading(Some(true), Some(true)).what_it_claims(),
            WhatTheVerificationClaims::ItPassed
        );
        // **One half with a subject is enough to claim a pass about that half**
        // — R4's own case, a run over a Blender the machine already held.
        assert_eq!(
            a_reading(Some(true), None).what_it_claims(),
            WhatTheVerificationClaims::ItPassed
        );
        assert_eq!(
            a_reading(None, Some(true)).what_it_claims(),
            WhatTheVerificationClaims::ItPassed
        );
        for did_not in [
            a_reading(Some(false), Some(true)),
            a_reading(Some(true), Some(false)),
            a_reading(Some(false), None),
            a_reading(None, Some(false)),
        ] {
            assert_eq!(did_not.what_it_claims(), WhatTheVerificationClaims::ItDidNotPass);
        }
        assert_eq!(
            a_reading(None, None).what_it_claims(),
            WhatTheVerificationClaims::NothingWasRead
        );
    }

    /// **The publication moves the staged build's own folder, so the executable
    /// is where the folder now is** — and the staging path it used to sit at is
    /// gone.
    #[test]
    fn the_published_executable_is_inside_the_folder_the_publication_made() {
        let staging = an_area("the-publication-moves-the-executable-with-it");
        let unpacked = staging.at().join("a-run-unpacked").join("blender-5.2.0-linux-x64");
        std::fs::create_dir_all(&unpacked).expect("the staging folder is made");
        std::fs::write(unpacked.join(THE_EXECUTABLES_NAME), b"").expect("the build is there");
        let at = staging.at().join("a-run-blender-5.2.0");
        let runner = AStandInForTheMachinesTools::holding(vec![(
            THE_EXECUTABLES_NAME,
            it_went(0, "Blender 5.2.0\n", ""),
        )]);
        let staged = unpacked.join(THE_EXECUTABLES_NAME);
        let (_, did) =
            the_build_is_published(&runner, &staged, a_version(), &at).expect("it ran");
        assert!(matches!(did, WhatThePublicationDid::ItWasPublished { .. }));
        assert!(!staged.exists(), "the staging path is gone");
        assert_eq!(the_published_executable(&at), at.join(THE_EXECUTABLES_NAME));
        assert!(the_published_executable(&at).is_file());
    }

    /// **What the final verification found is said in words a person reads**
    /// — step 10c's fourth pass on Vast, fault 3: *Abandon setup* put *"the
    /// library chain read clean Some(false), the published build answered
    /// Some(false)"* in front of the user. **`Server Setup` S2a carries these
    /// words to the desktop and the desktop shows them whole**, so they are
    /// read by a person and never by a program.
    #[test]
    fn what_the_final_verification_found_is_never_a_debugging_rendering() {
        let both = WhatTheFinalVerificationFound {
            the_library_chain_is_clean: Some(false),
            the_published_build_answers: Some(false),
        };
        let said = both.what_did_not_pass();
        for debugging in ["Some(", "None", "{", "::", "true", "false"] {
            assert!(!said.contains(debugging), "{said}");
        }
        assert!(said.contains("system libraries still missing"), "{said}");
        assert!(said.contains("did not report its version"), "{said}");

        // **A half that did not run says nothing** — `Server Setup` R4: it is
        // absent rather than failed.
        let one = WhatTheFinalVerificationFound {
            the_library_chain_is_clean: Some(false),
            the_published_build_answers: None,
        };
        let said = one.what_did_not_pass();
        assert!(said.contains("system libraries still missing"), "{said}");
        assert!(!said.contains("did not report its version"), "{said}");
    }

    /// **The staged executable is found and never composed from the version**,
    /// and a staging folder holding none says so.
    #[test]
    fn the_staged_executable_is_found_and_an_absence_is_said() {
        let staging = an_area("a-third-staging-area");
        assert!(the_staged_executable(staging.at()).is_err());
        let unpacked = staging.at().join("blender-5.2.0-linux-x64");
        std::fs::create_dir_all(&unpacked).expect("the folder is made");
        std::fs::write(unpacked.join("blender"), b"").expect("the build is there");
        assert_eq!(
            the_staged_executable(staging.at()).expect("it is found"),
            unpacked.join("blender")
        );
    }
}
