//! The platform file adapter — `Desktop app and delivery` §6.
//!
//! **F10: every local write in the application goes through it, and there is
//! no second route to the filesystem.** F10a widens *write* to cover deletion,
//! to own the ground an outside program writes on, and to own reading a name
//! back as the filesystem stores it.
//!
//! Every rule below was forced by something somebody watched a real machine
//! do, and each is written where it is kept rather than summarised here. The
//! one to read first is [`Written`], which is F1.

use std::fs::{File, OpenOptions};
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::time::SystemTime;

/// How a refusal is classified — **F4: by what the system returned, and never
/// by an amount.**
///
/// The macOS values are observed; the Windows and Linux ones are the same
/// symbols read from those systems' own headers, which is F7's rule — the
/// adapter maps **by symbol per system and never by number**, because
/// `EDQUOT` is 69 on macOS and 122 on Linux.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Refusal {
    /// `ENOSPC` at open or at write. **No amount is consulted to reach it**
    /// (F5).
    FullVolume,
    /// `EACCES` at open. **A permission refusal cannot arise at write**,
    /// because refusal happens at open.
    PermissionRefused,
    /// **F3: two causes are not claimed apart where the system does not
    /// separate them.** A removed directory and a detached volume both give
    /// `ENOENT` where the mount point went with the volume, so the adapter
    /// reports *the destination is no longer there* and names neither.
    DestinationGone,
    /// **F4a, the fourth class**, for the `EIO` a detached volume gives once at
    /// write while `fsync` and `close` then return 0. **It names no cause**:
    /// a detached volume is the only cause observed for it and nothing read
    /// establishes that it is the only cause that returns it.
    WriteFailedAtTheDevice,
    /// A refusal the system reported in terms this adapter does not map.
    /// **It is never folded into one of the four above**, on F4's own rule
    /// that a refusal is classified by what the system returned; claiming a
    /// class here would claim a cause the system did not give.
    Unclassified,
}

/// What a call failed with: the class, and the machine's own words kept beside
/// it for the diary and for a failure record.
///
/// **The words are never a screen's text as they stand** — §16.3 owns how a
/// machine's own words reach the user, and this type only carries them.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Failure {
    pub refusal: Refusal,
    pub reported: String,
}

impl Failure {
    fn from_error(error: &io::Error, at: Where) -> Self {
        Failure {
            refusal: classify(error, at),
            reported: error.to_string(),
        }
    }
}

/// Which call in the sequence a refusal came back from. F4 classifies two of
/// them differently, so the adapter cannot ask the question without it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Where {
    Open,
    Write,
}

/// **F4 and F7.** Classification is by symbol, per system, and a value this
/// adapter does not map stays unmapped.
#[cfg(unix)]
pub fn classify(error: &io::Error, at: Where) -> Refusal {
    match error.raw_os_error() {
        Some(code) if code == libc::ENOSPC => Refusal::FullVolume,
        Some(code) if code == libc::EACCES && at == Where::Open => Refusal::PermissionRefused,
        Some(code) if code == libc::ENOENT => Refusal::DestinationGone,
        Some(code) if code == libc::EIO && at == Where::Write => Refusal::WriteFailedAtTheDevice,
        _ => Refusal::Unclassified,
    }
}

/// **F4 and F7 on Windows, and nothing here is observed** — no Windows
/// computer has ever run it (§22 item 2), and §22 item 9 records that the
/// detached-volume value is unestablished on this system. **So there is no
/// Windows mapping for F4a's fourth class**, and a device failure arrives
/// unclassified rather than being given macOS's value, which is F7's rule
/// holding in the direction that costs nothing.
#[cfg(windows)]
pub fn classify(error: &io::Error, at: Where) -> Refusal {
    use windows_sys::Win32::Foundation::{
        ERROR_ACCESS_DENIED, ERROR_DISK_FULL, ERROR_FILE_NOT_FOUND, ERROR_HANDLE_DISK_FULL,
        ERROR_PATH_NOT_FOUND,
    };
    let Some(code) = error.raw_os_error() else {
        return Refusal::Unclassified;
    };
    let code = code as u32;
    if code == ERROR_DISK_FULL || code == ERROR_HANDLE_DISK_FULL {
        Refusal::FullVolume
    } else if code == ERROR_ACCESS_DENIED && at == Where::Open {
        Refusal::PermissionRefused
    } else if code == ERROR_FILE_NOT_FOUND || code == ERROR_PATH_NOT_FOUND {
        Refusal::DestinationGone
    } else {
        Refusal::Unclassified
    }
}

/// The outcome of one write sequence — **F1 and F2 together, which is the rule
/// the whole adapter is built on.**
///
/// **F1: no return value is an outcome.** The destination is re-checked by
/// path after closing and that re-check is what is reported. Observed on the
/// Mac: a directory deleted underneath an open descriptor reports success at
/// every call while the bytes go to an unlinked inode that vanishes at close.
///
/// **F2: the outcome is the first error any call in the sequence returned, and
/// a later success never clears it.** Observed: a detached volume returns
/// `EIO` once, at the write, and `fsync` and `close` then return 0.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Written {
    /// F2's first error, if any call in the sequence gave one.
    pub first_error: Option<Failure>,
    /// F1's re-check: what was at the path after the sequence closed.
    pub recheck: Recheck,
}

/// What the re-check found. **F12: this is never remembered as a fact about a
/// file** — what is written durably is the write's own outcome record, which
/// is a record Smoozy wrote about what Smoozy did.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Recheck {
    /// The path holds a file of exactly the length written.
    Present { bytes: u64 },
    /// The path holds a file of a different length. **F9's case**: where a
    /// volume filled during the write, a truncated file remained on every run.
    ShortOrDifferent { bytes: u64, expected: u64 },
    /// Nothing is at the path. **F9: no local writer reads this as evidence
    /// that nothing was written**, because a volume that filled during a write
    /// left a file behind and one already full left none.
    Absent,
    /// The re-check itself could not be taken.
    CouldNotLook { reported: String },
}

impl Written {
    /// Whether the sequence is to be reported as having written the file.
    ///
    /// **Both halves must hold.** F2's first error must be absent *and* F1's
    /// re-check must find the whole file. Either alone is one of the two
    /// observed ways to report a frame written that does not exist.
    pub fn wrote_the_file(&self) -> bool {
        self.first_error.is_none() && matches!(self.recheck, Recheck::Present { .. })
    }
}

/// Write a whole file and report F1's re-check.
///
/// The sequence is run to the end even after a refusal, because F2's first
/// error is what is reported and a sequence abandoned at the first error
/// leaves a descriptor open and the destination unchecked.
pub fn write_whole_file(path: &Path, bytes: &[u8], durable: Durability) -> Written {
    let mut first_error: Option<Failure> = None;
    let mut note = |failure: Failure| {
        if first_error.is_none() {
            first_error = Some(failure);
        }
    };

    match OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(path)
    {
        Ok(mut file) => {
            if let Err(error) = file.write_all(bytes) {
                note(Failure::from_error(&error, Where::Write));
            }
            if durable == Durability::ToTheDevice {
                if let Err(error) = flush_to_the_device(&file) {
                    note(Failure::from_error(&error, Where::Write));
                }
            }
            drop(file);
        }
        Err(error) => note(Failure::from_error(&error, Where::Open)),
    }

    Written {
        first_error,
        recheck: recheck(path, bytes.len() as u64),
    }
}

/// F1's re-check, by path and after closing.
fn recheck(path: &Path, expected: u64) -> Recheck {
    match std::fs::metadata(path) {
        Ok(found) if found.len() == expected => Recheck::Present { bytes: found.len() },
        Ok(found) => Recheck::ShortOrDifferent {
            bytes: found.len(),
            expected,
        },
        Err(error) if error.kind() == io::ErrorKind::NotFound => Recheck::Absent,
        Err(error) => Recheck::CouldNotLook {
            reported: error.to_string(),
        },
    }
}

/// Whether a write has to reach the storage device before it is called done —
/// `State, identity and recovery` §1.3 rule 6.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Durability {
    /// The record and every ordering rule that says *written durably before*.
    ToTheDevice,
    /// Material whose loss costs a re-fetch and nothing else.
    Ordinary,
}

/// §1.3 rule 6's flush, and **the call differs by system and the difference is
/// not cosmetic.**
///
/// **macOS: `F_FULLFSYNC`.** That document quotes Apple's own manual — *"while
/// fsync() will flush all data from the host to the drive … the drive itself
/// may not physically write the data to the platters for quite some time"*,
/// *"This is not a theoretical edge case."*, and *"Mac OS X provides the
/// F_FULLFSYNC fcntl."* **Linux: `fsync`.** **Windows:
/// `FlushFileBuffers`**, which is the call `Reporting a problem` J8 already
/// names for the same guarantee in this project.
#[cfg(target_os = "macos")]
pub fn flush_to_the_device(file: &File) -> io::Result<()> {
    use std::os::unix::io::AsRawFd;
    // `F_FULLFSYNC` takes no argument; the manual's own form is
    // `fcntl(fd, F_FULLFSYNC)`.
    if unsafe { libc::fcntl(file.as_raw_fd(), libc::F_FULLFSYNC) } == -1 {
        return Err(io::Error::last_os_error());
    }
    Ok(())
}

#[cfg(all(unix, not(target_os = "macos")))]
pub fn flush_to_the_device(file: &File) -> io::Result<()> {
    file.sync_all()
}

#[cfg(windows)]
pub fn flush_to_the_device(file: &File) -> io::Result<()> {
    // `File::sync_all` is `FlushFileBuffers` on this system.
    file.sync_all()
}

/// §1.3 rule 6's last clause, **which is not optional and is the one that is
/// ordinarily missed**: the directory holding a published name is flushed too.
///
/// On Windows there is no directory handle to flush in the same sense, and no
/// page read names a counterpart; the rule's own words leave the call to
/// whichever the system documents, and there is none. **So this is a stated
/// silence on that system rather than a call quietly skipped** (§22 item 4
/// already carries the Windows publication question).
pub fn flush_the_holding_directory(path: &Path) -> io::Result<()> {
    #[cfg(unix)]
    {
        let Some(parent) = path.parent() else {
            return Ok(());
        };
        let directory = File::open(parent)?;
        directory.sync_all()
    }
    #[cfg(not(unix))]
    {
        let _ = path;
        Ok(())
    }
}

/// **F15: publication is the platform's atomic rename of the staged file onto
/// the final path, and staging is on the same filesystem as the final name.**
///
/// **F16, per system.** macOS: `rename(2)`, on Apple's own crash guarantee —
/// *"The rename() system call guarantees that an instance of new will always
/// exist, even if the system should crash in the middle of the operation."*
/// Linux: `rename(2)`, on that system's own manual — *"If newpath already
/// exists, it will be atomically replaced."* **Windows: the documentation
/// route is answered negatively** — Microsoft documents `MoveFileExW` and
/// `ReplaceFileW` and **no crash guarantee for either** — and F19 is what
/// holds: the unit is written as atomic and this adapter says so, rather than
/// the boundary being softened to whatever a platform happens to give.
///
/// The two requirements that do not depend on the platform are kept here:
/// **staging on the same filesystem**, checked before the rename, and **no
/// route to the final name but this step**.
pub fn publish(staged: &Path, final_name: &Path) -> Result<(), Failure> {
    if !same_filesystem(staged, final_name) {
        return Err(Failure {
            refusal: Refusal::Unclassified,
            reported: "the staged file is not on the same filesystem as its final name".into(),
        });
    }

    std::fs::rename(staged, final_name)
        .map_err(|error| Failure::from_error(&error, Where::Write))?;

    // §1.3 rule 6: the rename follows the flush, and the directory holding the
    // published name is flushed after it.
    if let Err(error) = flush_the_holding_directory(final_name) {
        return Err(Failure::from_error(&error, Where::Write));
    }
    Ok(())
}

/// F15's *same filesystem* requirement, answered by the identity call rather
/// than by comparing text.
fn same_filesystem(one: &Path, other: &Path) -> bool {
    let holder = |path: &Path| {
        path.parent()
            .map(PathBuf::from)
            .unwrap_or_else(|| path.into())
    };
    match (volume_of(&holder(one)), volume_of(&holder(other))) {
        (Some(a), Some(b)) => a == b,
        // **Where the volume cannot be established, the requirement is not
        // claimed met.** F15's own discipline: a value nobody stands behind,
        // passed off as an identity, is worse than no answer.
        _ => false,
    }
}

#[cfg(unix)]
fn volume_of(path: &Path) -> Option<u64> {
    use std::os::unix::fs::MetadataExt;
    std::fs::metadata(path).ok().map(|found| found.dev())
}

#[cfg(windows)]
fn volume_of(path: &Path) -> Option<u64> {
    file_identity(path).map(|identity| identity.volume)
}

/// **F15a: the staged and final objects' stable file identity, per system.**
///
/// macOS and Linux: the `st_dev`/`st_ino` pair, which is the standard's own
/// rule — *"The st_ino and st_dev fields taken together uniquely identify the
/// file within the system."* **The pair and never the inode alone**, because
/// inode numbers *"are guaranteed to be unique only within a filesystem"*.
///
/// **Nothing reads an identity match as proof against reuse.** Neither
/// publication promises the pair is never reused after a delete, so a match is
/// compared beside the byte length and digest by its consumers and never
/// alone, and F21b keeps it out of every durable key.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FileIdentity {
    pub volume: u64,
    pub file: u128,
}

#[cfg(unix)]
pub fn file_identity(path: &Path) -> Option<FileIdentity> {
    use std::os::unix::fs::MetadataExt;
    let found = std::fs::metadata(path).ok()?;
    Some(FileIdentity {
        volume: found.dev(),
        file: found.ino() as u128,
    })
}

/// **F15a on Windows: the 128-bit file identifier with the volume serial
/// number, `GetFileInformationByHandleEx` with `FileIdInfo`.** Microsoft's own
/// words: *"The file identifier and the volume serial number uniquely identify
/// a file on a single computer."*
///
/// **Documentation alone, on a system with no computer** (§22 item 2). Where
/// the call refuses, the adapter reports the identity as unavailable rather
/// than substituting anything, which is F15a's own branch.
#[cfg(windows)]
pub fn file_identity(path: &Path) -> Option<FileIdentity> {
    use std::os::windows::fs::OpenOptionsExt;
    use std::os::windows::io::AsRawHandle;
    use windows_sys::Win32::Storage::FileSystem::{
        FileIdInfo, GetFileInformationByHandleEx, FILE_ID_INFO,
    };

    let file = OpenOptions::new()
        .read(true)
        .custom_flags(windows_sys::Win32::Storage::FileSystem::FILE_FLAG_BACKUP_SEMANTICS)
        .open(path)
        .ok()?;

    let mut info: FILE_ID_INFO = unsafe { std::mem::zeroed() };
    let ok = unsafe {
        GetFileInformationByHandleEx(
            file.as_raw_handle() as _,
            FileIdInfo,
            &mut info as *mut _ as *mut core::ffi::c_void,
            std::mem::size_of::<FILE_ID_INFO>() as u32,
        )
    };
    if ok == 0 {
        return None;
    }
    Some(FileIdentity {
        volume: info.VolumeSerialNumber,
        file: u128::from_le_bytes(info.FileId.Identifier),
    })
}

/// **F21: whether two paths name one folder is answered by comparing identity
/// numbers the filesystem assigns, live, at the moment the answer is needed —
/// never by comparing text.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SameFolder {
    Yes,
    No,
    /// **F21a and F21d.** A folder that does not exist yet is identified by no
    /// system; a network volume is one no vendor publishes identity across.
    /// **The interface says so rather than pretending** the answer is certain.
    CannotDetermine,
}

/// F21, with its three qualifications.
///
/// **Symbolic links are resolved before the call** on all three systems, on the
/// trap Apple's own file-identity value disagreeing with `stat` through a link
/// showed and Windows documents for its own call.
///
/// **F21b: the result is never stored as a durable key.** Both POSIX and
/// Microsoft state these values are reused over time, so the comparison is
/// taken live and discarded — which is what returning an answer rather than an
/// identity from this function makes true by construction.
pub fn folders_are_the_same(one: &Path, other: &Path) -> SameFolder {
    let (Ok(one), Ok(other)) = (std::fs::canonicalize(one), std::fs::canonicalize(other)) else {
        // F21a: a path that does not exist is identified by nothing, and the
        // fallback is the resolved canonical text with the answer marked
        // uncertain rather than presented as certain.
        return SameFolder::CannotDetermine;
    };

    if !on_a_local_volume(&one) || !on_a_local_volume(&other) {
        return SameFolder::CannotDetermine;
    }

    match (file_identity(&one), file_identity(&other)) {
        (Some(a), Some(b)) if a == b => SameFolder::Yes,
        (Some(_), Some(_)) => SameFolder::No,
        _ => SameFolder::CannotDetermine,
    }
}

/// **F21d's test, and it is answered on one system of the three.**
///
/// macOS publishes it: `statfs(2)`'s `MNT_LOCAL` — *"File system is stored
/// locally"* — so this system takes F21d's branch for real. **Windows and
/// Linux establish no such test anywhere read** (§22 item 56, whose own words
/// are that *nothing establishes how the adapter tells a network volume from a
/// local one in the first place*), so on those two this answers *local* and
/// F21d's branch is reached only through the identity call refusing. **That is
/// the silence recorded rather than a detection invented**, and the decisions
/// record carries what it costs.
#[cfg(target_os = "macos")]
fn on_a_local_volume(path: &Path) -> bool {
    use std::ffi::CString;
    use std::os::unix::ffi::OsStrExt;

    let Ok(name) = CString::new(path.as_os_str().as_bytes()) else {
        return false;
    };
    let mut buffer: libc::statfs = unsafe { std::mem::zeroed() };
    if unsafe { libc::statfs(name.as_ptr(), &mut buffer) } != 0 {
        return false;
    }
    buffer.f_flags & libc::MNT_LOCAL as u32 != 0
}

#[cfg(not(target_os = "macos"))]
fn on_a_local_volume(_path: &Path) -> bool {
    true
}

/// **F10a's first bullet: a local deletion is a local write for F10's rule.**
/// Removing a file, removing a directory and clearing a staged area all come
/// here, and there is no second route for removal either.
pub fn remove_file(path: &Path) -> Result<(), Failure> {
    match std::fs::remove_file(path) {
        Ok(()) => Ok(()),
        // A file that is already absent is the state the caller asked for.
        Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(()),
        Err(error) => Err(Failure::from_error(&error, Where::Write)),
    }
}

/// L3a's sweep and `Security and licence` §7's, in one place: **everything
/// found in the area is removed before any module uses it.**
///
/// It returns what it could not remove rather than stopping at the first, so
/// one stubborn file does not leave the rest of the area standing.
pub fn clear_directory(path: &Path) -> Vec<Failure> {
    let mut trouble = Vec::new();
    let entries = match std::fs::read_dir(path) {
        Ok(entries) => entries,
        // An area that does not exist yet is an area with nothing in it.
        Err(error) if error.kind() == io::ErrorKind::NotFound => return trouble,
        Err(error) => return vec![Failure::from_error(&error, Where::Open)],
    };
    for entry in entries.flatten() {
        let path = entry.path();
        let removed = if path.is_dir() {
            std::fs::remove_dir_all(&path)
        } else {
            std::fs::remove_file(&path)
        };
        if let Err(error) = removed {
            trouble.push(Failure::from_error(&error, Where::Write));
        }
    }
    trouble
}

/// Make a directory Smoozy owns, and every level above it that is also
/// Smoozy's. **A local write, so it is here** (F10).
pub fn ensure_directory(path: &Path) -> Result<(), Failure> {
    std::fs::create_dir_all(path).map_err(|error| Failure::from_error(&error, Where::Open))
}

/// The same, with the mode the caller's clause names — `Keys` G9's 0700 for
/// the key location.
///
/// **The mode is set on the directory this call creates and never on one that
/// was already there**, because a directory the user already has is theirs and
/// `Security and licence` §6 has Smoozy change no mode it did not set.
///
/// **On Windows the mode is not applied, and the silence is deliberate**: a
/// Unix permission triple has no Windows form, the Windows form of the same
/// judgement is an access-control list, and what that list would have to say is
/// established by nothing this project has read (`Keys` §13 item 8,
/// permanently unresolved). The directory is created; nothing pretends to have
/// protected it.
pub fn ensure_directory_at_mode(path: &Path, mode: u32) -> Result<(), Failure> {
    if path.is_dir() {
        return Ok(());
    }
    ensure_directory(path)?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(path, std::fs::Permissions::from_mode(mode))
            .map_err(|error| Failure::from_error(&error, Where::Write))?;
    }
    #[cfg(not(unix))]
    let _ = mode;
    Ok(())
}

/// The canonical form of a path — symbolic links followed, `.` and `..`
/// collapsed — as `Keys` L3 records it and as `Keys` W1's ownership derivation
/// compares it.
///
/// **On Windows the extended-length prefix is taken off a plain drive path,
/// and that is a Build decision with a reason.** The system's own
/// canonicalisation answers `\\?\C:\…`, which is a form `Keys`' whole
/// reading half then hands to `ssh-keygen` — a program this project has read
/// nothing about the path handling of. **A recorded path that a tool will not
/// open is a key Smoozy lists and can never check**, so the prefix comes off
/// where the rest is an ordinary drive path and stays on where it is anything
/// else (a UNC share, a device path), because there the prefix is the path.
///
/// **Nothing about the resolution changes** — the links are still followed and
/// the result is still the filesystem's own — and on the other two systems
/// this is the system call and nothing more.
pub fn canonical(path: &Path) -> Option<PathBuf> {
    let resolved = std::fs::canonicalize(path).ok()?;
    #[cfg(windows)]
    {
        let text = resolved.to_string_lossy();
        if let Some(rest) = text.strip_prefix(r"\\?\") {
            // A plain drive path — `C:\…` — and never a UNC form, which the
            // prefix would leave as `UNC\server\share`.
            let mut letters = rest.chars();
            let looks_like_a_drive = matches!(
                (letters.next(), letters.next(), letters.next()),
                (Some(letter), Some(':'), Some('\\')) if letter.is_ascii_alphabetic()
            );
            if looks_like_a_drive {
                return Some(PathBuf::from(rest));
            }
        }
    }
    Some(resolved)
}

/// **F10a's second bullet: a file an outside program writes.**
///
/// The destination is composed and checked through the adapter before the run,
/// and the outcome is established by re-reading that path through the adapter
/// after it. **The adapter does not claim to have performed the write; it owns
/// the name and the verdict**, which is F1's discipline applied to somebody
/// else's write.
pub fn outside_program_wrote(path: &Path) -> Recheck {
    match std::fs::metadata(path) {
        Ok(found) => Recheck::Present { bytes: found.len() },
        Err(error) if error.kind() == io::ErrorKind::NotFound => Recheck::Absent,
        Err(error) => Recheck::CouldNotLook {
            reported: error.to_string(),
        },
    }
}

/// A free-space reading, **supplied only where a module asks to show one**
/// (F5).
///
/// **F5: no local decision is taken from a free-space reading, so there is no
/// full-volume test in this adapter at all.** A genuinely full volume reported
/// 1,068 KB free rather than zero, and that observation is the reason no
/// threshold exists rather than a rule to derive one from.
///
/// It is taken **against the destination path's own volume and never against
/// the computer**, and it carries the moment it was taken so nobody can
/// present it as current — the same shape `State, identity and recovery` D5
/// requires of every reading with a clock behind it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FreeSpace {
    pub bytes: u64,
    pub taken_at: SystemTime,
}

// ── The update's own writes: Z24 and F20 ────────────────────────────────────
//
// **Z24: the updater's direct writes are this adapter's, like every other
// write in the application**, and the updater is not a carve-out from F10.
// Added at step 11's Mac update piece. Three operations, and each keeps F1's
// re-check by path and F2's first-error rule:
//
// - **[`the_bundle_is_replaced`]** — F20's macOS variant in full: the
//   supported replacement call's own operation, with a named backup of the
//   complete previous bundle (Z5, Z7).
// - **[`the_archive_is_unpacked`]** — the Mac download is a ZIP made by the
//   system's own `ditto` (`App/Release/src/the_mac_bundle.rs`), and it is
//   unpacked by the same tool. **An outside program writes the bundle**, so this
//   is F10a's second bullet: the adapter composes the place, and re-reads it
//   after the run, and does not claim to have written it.
// - **[`a_folder_is_removed_whole`]** — F10a's first bullet: removing a
//   directory is a local write.

/// **A folder and everything in it, removed** — F10a's first bullet. A folder
/// already gone is the state the caller asked for. **F1: the path is read
/// again afterwards**, and a folder still there is a failure whatever the call
/// returned.
pub fn a_folder_is_removed_whole(path: &Path) -> Result<(), Failure> {
    match std::fs::remove_dir_all(path) {
        Ok(()) => {}
        Err(error) if error.kind() == io::ErrorKind::NotFound => {}
        Err(error) => return Err(Failure::from_error(&error, Where::Write)),
    }
    match outside_program_wrote(path) {
        Recheck::Absent => Ok(()),
        Recheck::CouldNotLook { reported } => Err(Failure {
            refusal: Refusal::Unclassified,
            reported,
        }),
        _ => Err(Failure {
            refusal: Refusal::Unclassified,
            reported: format!("{} is still there after it was removed", path.display()),
        }),
    }
}

/// **The Mac download unpacked by the system's own tool, into a folder of its
/// own** — `ditto -x -k`, whose own manual says `-x` *"Extract the archives
/// given as source arguments"* and `-k` *"Create or extract from a PKZip
/// archive instead of the default CPIO"*, and that it *"will preserve resource
/// forks and HFS meta-data information when copying unless instructed
/// otherwise"* (macOS 26.6's `ditto(1)`, read 2026-10-25). That last is why it
/// is `ditto` and nothing else: a bundle's signature lives partly in that
/// metadata.
///
/// **The place is this adapter's, the writing is the tool's, and the verdict
/// is this adapter's again** (F10a): `into` must not exist before the run,
/// the tool's own ending is read, and the folder is read back by path after.
pub fn the_archive_is_unpacked(archive: &Path, into: &Path) -> Result<(), Failure> {
    if !matches!(outside_program_wrote(into), Recheck::Absent) {
        return Err(Failure {
            refusal: Refusal::Unclassified,
            reported: format!("{} is already there", into.display()),
        });
    }
    if !cfg!(target_os = "macos") {
        return Err(Failure {
            refusal: Refusal::Unclassified,
            reported: "a Mac download is unpacked on a Mac alone".into(),
        });
    }
    let ran = std::process::Command::new("/usr/bin/ditto")
        .arg("-x")
        .arg("-k")
        .arg(archive)
        .arg(into)
        .stdin(std::process::Stdio::null())
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::piped())
        .output()
        .map_err(|error| Failure::from_error(&error, Where::Open))?;
    if !ran.status.success() {
        return Err(Failure {
            refusal: Refusal::Unclassified,
            reported: format!(
                "ditto ended {}: {}",
                ran.status,
                String::from_utf8_lossy(&ran.stderr).trim()
            ),
        });
    }
    match outside_program_wrote(into) {
        Recheck::Present { .. } if into.is_dir() => Ok(()),
        Recheck::CouldNotLook { reported } => Err(Failure {
            refusal: Refusal::Unclassified,
            reported,
        }),
        _ => Err(Failure {
            refusal: Refusal::Unclassified,
            reported: format!("{} is not a folder after the archive was unpacked", into.display()),
        }),
    }
}

/// **Z5 and Z7: the complete bundle replaced at its installed path, with a
/// named backup of the complete previous bundle** — F20's macOS variant, which
/// is the supported replacement call's own operation. **Nothing inside either
/// bundle is written, deleted or renamed** (Z5, on TN2206's sealing rule): the
/// unit is the whole folder.
///
/// The call is `NSFileManager`'s `replaceItemAtURL:withItemAtURL:backupItemName:options:resultingItemURL:error:`.
/// Apple's own words for it, read 2026-10-25 (the sources file of this piece's
/// account): it *"Replaces the contents of the item at the specified URL in a
/// manner that ensures no data loss occurs"*; the backup name, *"If provided,
/// the name used to create a backup of the original item. The backup is placed
/// in the same directory as the original item … If there is already an item
/// with the same name as the backup item, that item will be removed. The
/// backup item will be removed in the event of success unless the
/// [withoutDeletingBackupItem] option is provided"*, which is why that option
/// is passed — Z7's backup has to outlive the replacement; and *"This method
/// works only when the originalItemURL and newItemURL parameters are located on
/// the same volume"*, which is E18. **The page speaks of *the item* throughout
/// and never says the item may be a folder** — `Desktop app and delivery`
/// §22 item 43, which this call does not close.
///
/// **F1: both places are read again by path afterwards**, and the outcome is
/// that reading: the installed path holds a folder, and the backup, where one
/// was named, is beside it. `backup` is a name, not a path, as the call takes
/// it; with none, the item replaced is gone afterwards, which is the rollback's
/// use of this same call (Z21).
pub fn the_bundle_is_replaced(installed: &Path, with: &Path, backup: Option<&str>) -> Result<(), Failure> {
    if !same_filesystem(with, installed) {
        return Err(Failure {
            refusal: Refusal::Unclassified,
            reported: "the staged bundle is not on the same filesystem as the installed one".into(),
        });
    }
    the_supported_replacement(installed, with, backup)?;
    if let Err(error) = flush_the_holding_directory(installed) {
        return Err(Failure::from_error(&error, Where::Write));
    }
    let is_a_folder = |path: &Path| path.is_dir();
    if !is_a_folder(installed) {
        return Err(Failure {
            refusal: Refusal::Unclassified,
            reported: format!("{} is not a folder after the replacement", installed.display()),
        });
    }
    if let Some(name) = backup {
        let kept = installed.with_file_name(name);
        if !is_a_folder(&kept) {
            return Err(Failure {
                refusal: Refusal::Unclassified,
                reported: format!("the backup {} is not there after the replacement", kept.display()),
            });
        }
    }
    Ok(())
}

#[cfg(target_os = "macos")]
fn the_supported_replacement(installed: &Path, with: &Path, backup: Option<&str>) -> Result<(), Failure> {
    use objc2_foundation::{NSFileManager, NSFileManagerItemReplacementOptions, NSString, NSURL};

    let text = |path: &Path| {
        path.to_str().map(str::to_owned).ok_or_else(|| Failure {
            refusal: Refusal::Unclassified,
            reported: format!("{} is not a path this call can name", path.display()),
        })
    };
    let original = NSURL::fileURLWithPath(&NSString::from_str(&text(installed)?));
    let new_item = NSURL::fileURLWithPath(&NSString::from_str(&text(with)?));
    let backup = backup.map(NSString::from_str);
    let options = if backup.is_some() {
        NSFileManagerItemReplacementOptions::WithoutDeletingBackupItem
    } else {
        NSFileManagerItemReplacementOptions::empty()
    };
    let manager = NSFileManager::defaultManager();
    manager
        .replaceItemAtURL_withItemAtURL_backupItemName_options_resultingItemURL_error(
            &original,
            &new_item,
            backup.as_deref(),
            options,
            None,
        )
        .map_err(|error| Failure {
            refusal: Refusal::Unclassified,
            reported: error.localizedDescription().to_string(),
        })
}

#[cfg(not(target_os = "macos"))]
fn the_supported_replacement(_installed: &Path, _with: &Path, _backup: Option<&str>) -> Result<(), Failure> {
    Err(Failure {
        refusal: Refusal::Unclassified,
        reported: "a bundle is replaced on a Mac alone".into(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn scratch(name: &str) -> PathBuf {
        let mut path = std::env::temp_dir();
        path.push(format!("smoozy-adapter-test-{name}-{}", std::process::id()));
        let _ = std::fs::create_dir_all(&path);
        path
    }

    /// F1 and F2 together: both halves must hold before a write is reported as
    /// having written the file.
    #[test]
    fn a_write_is_reported_on_the_recheck_and_not_on_the_return_value() {
        let clean = Written {
            first_error: None,
            recheck: Recheck::Present { bytes: 3 },
        };
        assert!(clean.wrote_the_file());

        // F2: an error early in the sequence is not cleared by a later success,
        // even where the re-check finds the whole file.
        let errored = Written {
            first_error: Some(Failure {
                refusal: Refusal::WriteFailedAtTheDevice,
                reported: "input/output error".into(),
            }),
            recheck: Recheck::Present { bytes: 3 },
        };
        assert!(!errored.wrote_the_file());

        // F1: no error anywhere and the file is not there. This is the observed
        // case the whole rule exists for.
        let vanished = Written {
            first_error: None,
            recheck: Recheck::Absent,
        };
        assert!(!vanished.wrote_the_file());

        // F9: a truncated file is not a written file either.
        let short = Written {
            first_error: None,
            recheck: Recheck::ShortOrDifferent {
                bytes: 1,
                expected: 3,
            },
        };
        assert!(!short.wrote_the_file());
    }

    /// F4 and F7: classification is by symbol, and a permission refusal is only
    /// a permission refusal at open, because refusal happens there.
    #[cfg(unix)]
    #[test]
    fn refusals_are_classified_by_symbol_and_by_where() {
        let full = io::Error::from_raw_os_error(libc::ENOSPC);
        assert_eq!(classify(&full, Where::Open), Refusal::FullVolume);
        assert_eq!(classify(&full, Where::Write), Refusal::FullVolume);

        let refused = io::Error::from_raw_os_error(libc::EACCES);
        assert_eq!(classify(&refused, Where::Open), Refusal::PermissionRefused);
        assert_eq!(classify(&refused, Where::Write), Refusal::Unclassified);

        // F4a: the fourth class, and only at write.
        let device = io::Error::from_raw_os_error(libc::EIO);
        assert_eq!(
            classify(&device, Where::Write),
            Refusal::WriteFailedAtTheDevice
        );

        // F3: one class for two causes the system does not separate.
        let gone = io::Error::from_raw_os_error(libc::ENOENT);
        assert_eq!(classify(&gone, Where::Open), Refusal::DestinationGone);
    }

    /// F7's own example, which is why the mapping is by symbol: the quota value
    /// differs between the two systems that have it.
    #[cfg(unix)]
    #[test]
    fn an_unmapped_value_is_never_given_a_class() {
        let quota = io::Error::from_raw_os_error(libc::EDQUOT);
        assert_eq!(classify(&quota, Where::Write), Refusal::Unclassified);
    }

    /// A real write, its re-check, and its publication — the one end-to-end
    /// path every local writer takes.
    #[test]
    fn a_written_file_publishes_by_rename() {
        let area = scratch("publish");
        let staged = area.join("staged");
        let final_name = area.join("published");

        let written = write_whole_file(&staged, b"twelve bytes", Durability::ToTheDevice);
        assert!(written.wrote_the_file(), "{written:?}");

        publish(&staged, &final_name).expect("same filesystem, so this must publish");
        assert_eq!(std::fs::read(&final_name).unwrap(), b"twelve bytes");
        // F15's second platform-independent requirement: nothing is left at the
        // staging name.
        assert!(!staged.exists());

        let _ = std::fs::remove_dir_all(&area);
    }

    /// F15: staging is on the same filesystem as the final name, and a
    /// publication that is not gets refused rather than falling back to a copy.
    #[test]
    fn publication_refuses_a_staged_file_on_another_filesystem() {
        let area = scratch("cross");
        let staged = area.join("staged");
        write_whole_file(&staged, b"x", Durability::Ordinary);
        // A destination whose holder does not exist has no volume to compare,
        // and F15 does not claim the requirement met where it cannot be shown.
        let elsewhere = area.join("no-such-folder").join("published");
        assert!(publish(&staged, &elsewhere).is_err());
        let _ = std::fs::remove_dir_all(&area);
    }

    /// F21: sameness is the identity numbers and never the text. A folder
    /// reached by two different spellings of one path is one folder.
    #[test]
    fn one_folder_reached_two_ways_is_one_folder() {
        let area = scratch("identity");
        let sideways = area.join("inner").join("..");
        std::fs::create_dir_all(area.join("inner")).unwrap();
        assert_eq!(folders_are_the_same(&area, &sideways), SameFolder::Yes);

        let other = area.join("inner");
        assert_eq!(folders_are_the_same(&area, &other), SameFolder::No);

        let _ = std::fs::remove_dir_all(&area);
    }

    /// F21a: a folder that does not exist is identified by no system, and the
    /// adapter says so rather than falling back to text and calling it certain.
    #[test]
    fn a_folder_that_does_not_exist_cannot_be_compared() {
        let area = scratch("absent");
        let missing = area.join("never-made");
        assert_eq!(
            folders_are_the_same(&missing, &missing),
            SameFolder::CannotDetermine
        );
        let _ = std::fs::remove_dir_all(&area);
    }

    /// F10a: clearing an area is a local write and goes through the adapter,
    /// and an area that does not exist is an area with nothing in it.
    #[test]
    fn clearing_an_area_removes_everything_in_it() {
        let area = scratch("sweep");
        std::fs::write(area.join("one"), b"a").unwrap();
        std::fs::create_dir_all(area.join("two")).unwrap();
        std::fs::write(area.join("two").join("three"), b"b").unwrap();

        assert!(clear_directory(&area).is_empty());
        assert_eq!(std::fs::read_dir(&area).unwrap().count(), 0);

        let _ = std::fs::remove_dir_all(&area);
        assert!(clear_directory(&area).is_empty());
    }

    /// F10a: removing a file that is already gone is the state the caller
    /// asked for, and is not a refusal to classify.
    #[test]
    fn removing_an_absent_file_is_not_a_failure() {
        let area = scratch("remove");
        assert!(remove_file(&area.join("never-made")).is_ok());
        let _ = std::fs::remove_dir_all(&area);
    }

    /// F5: there is no full-volume test in this adapter. A reading exists only
    /// as something to show, and it carries the moment it was taken.
    #[test]
    fn a_free_space_reading_carries_its_moment() {
        let reading = FreeSpace {
            bytes: 1_068 * 1024,
            taken_at: SystemTime::now(),
        };
        // The observed value that is the reason no threshold exists: a
        // genuinely full volume reported this rather than zero.
        assert_ne!(reading.bytes, 0);
        let _ = reading.taken_at;
    }

    /// A folder shaped like a bundle, with one file whose words say which.
    fn a_bundle_at(path: &Path, says: &str) {
        std::fs::create_dir_all(path.join("Contents/MacOS")).unwrap();
        std::fs::write(path.join("Contents/MacOS/which"), says).unwrap();
    }

    fn which(path: &Path) -> String {
        std::fs::read_to_string(path.join("Contents/MacOS/which")).unwrap_or_default()
    }

    /// **Z5, Z7 and Z24: the complete bundle is replaced at its path and the
    /// complete previous one is kept under its name beside it; and the same
    /// call with no name puts the backup back and leaves nothing of the failed
    /// one** — the rollback's use of it (Z21). Run on throwaway folders, never on
    /// an installed copy.
    ///
    /// **What this does and does not establish, said so it is not read as
    /// more** (§22 item 43): it shows Apple's call carrying a folder and its
    /// named backup on this Mac. It is neither Apple's page saying so nor F18's
    /// shape, which kills the process at the boundary again and again.
    #[cfg(target_os = "macos")]
    #[test]
    fn a_bundle_is_replaced_whole_with_its_named_backup_and_put_back() {
        let area = scratch("bundle-replace");
        let _ = std::fs::remove_dir_all(&area);
        std::fs::create_dir_all(&area).unwrap();
        let installed = area.join("Smoozy.app");
        let staged = area.join(".Smoozy update").join("Smoozy.app");
        a_bundle_at(&installed, "old");
        a_bundle_at(&staged, "new");

        the_bundle_is_replaced(&installed, &staged, Some(".Smoozy previous.app")).expect("replaced");
        assert_eq!(which(&installed), "new");
        let backup = area.join(".Smoozy previous.app");
        assert_eq!(which(&backup), "old", "the named backup is the complete previous bundle");
        assert!(!staged.exists(), "the staged bundle moved into place");

        // The rollback: the backup goes back, and the failed one is gone.
        the_bundle_is_replaced(&installed, &backup, None).expect("put back");
        assert_eq!(which(&installed), "old");
        assert!(!backup.exists());
        assert_eq!(
            std::fs::read_dir(&area).unwrap().count(),
            2,
            "only the installed bundle and the emptied staging folder are left"
        );

        // A staged bundle that is not there is refused, and the installed one
        // is untouched.
        assert!(the_bundle_is_replaced(&installed, &area.join("absent.app"), Some(".b.app")).is_err());
        assert_eq!(which(&installed), "old");
        let _ = std::fs::remove_dir_all(&area);
    }

    /// **The Mac download is unpacked by the system's own tool into a folder
    /// that was not there, and a folder already there is refused**; and a
    /// folder is removed whole, absent or not.
    #[cfg(target_os = "macos")]
    #[test]
    fn a_download_is_unpacked_by_ditto_and_a_folder_is_removed_whole() {
        let area = scratch("unpack");
        let _ = std::fs::remove_dir_all(&area);
        std::fs::create_dir_all(&area).unwrap();
        let bundle = area.join("Smoozy.app");
        a_bundle_at(&bundle, "zipped");
        let zip = area.join("Smoozy-macos.zip");
        let made = std::process::Command::new("/usr/bin/ditto")
            .args(["-c", "-k", "--keepParent"])
            .arg(&bundle)
            .arg(&zip)
            .status()
            .unwrap();
        assert!(made.success());

        let into = area.join("unpacked");
        the_archive_is_unpacked(&zip, &into).expect("unpacked");
        assert_eq!(which(&into.join("Smoozy.app")), "zipped");
        assert!(the_archive_is_unpacked(&zip, &into).is_err(), "unpacked over a folder already there");
        let not_a_zip = area.join("not.zip");
        std::fs::write(&not_a_zip, b"not an archive").unwrap();
        assert!(the_archive_is_unpacked(&not_a_zip, &area.join("other")).is_err());

        a_folder_is_removed_whole(&into).expect("removed");
        assert!(!into.exists());
        a_folder_is_removed_whole(&into).expect("an absent folder is what was asked for");
        let _ = std::fs::remove_dir_all(&area);
    }
}
