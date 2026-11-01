//! The readings taken on the user's own computer — F8's enumeration, F12 and
//! F14a's stored-name reading, Q2's digest and A10's prefix digest.
//!
//! # Where this belongs, and why it is here
//!
//! **F13 asks that the exact-case reading live in `Desktop app and delivery`'s
//! platform file adapter**, as the sending-end twin of that document's C15,
//! *"one platform surface rather than two"*. **That request is not granted
//! yet**: the adapter's own crate depends on the feature modules and supplies
//! their adapters, so a feature module cannot reach into it, and the crate
//! that would host this is not this builder's to write in. **F13 says what
//! stands in the meantime, in as many words: *"Until it is granted, the macOS
//! call set above is the requirement's content."*** So the call set is here,
//! **in one file and behind [`crate::readings::Readings`]**, which is the half
//! of F13 that can be kept — there is exactly one route to the filesystem in
//! this crate and it is this file. **Nothing here manipulates a string to
//! decide a name**: every answer is a call, and the trap F13 names is the one
//! thing this file is written to avoid.
//!
//! **Where each system stands, and it is the plan's own list rather than this
//! file's.** Settled: **macOS**, on Fact 16's observed byte-for-byte storage,
//! with F12's call set. Settled by citation and unrun: a case-folding **ext4**
//! directory, and a Windows **NTFS or ReFS** volume, where exactness comes
//! from the normalized name and never from the listing (F14a). Permanently
//! unresolved: a Windows volume that is neither NTFS nor ReFS, a case-folding
//! Linux filesystem that is not ext4, HFS+, and network mounts (§12 items 3,
//! 11 and 21). **No space is left anywhere below for an answer to any of
//! them** — the Creator ruled on 2026-08-31 that no machine is coming.
//!
//! # The facts about each system, each from that system's own publication
//!
//! **macOS `fcntl(2)`, read on this Mac on 2026-09-04** (`man 2 fcntl`):
//! `F_GETPATH` *"Get the path of the file descriptor fildes. The argument must
//! be a buffer of size MAXPATHLEN or greater."* **That is one call returning a
//! whole location**, which is what F12 asks for — *"read from the system and
//! never rejoined from strings"*.
//!
//! **macOS `realpath(3)`, read the same day** (`man 3 realpath`): *"Defining
//! _DARWIN_C_SOURCE or _DARWIN_BETTER_REALPATH before including stdlib.h will
//! cause the provided implementation of realpath() to use F_GETPATH from
//! fcntl(2) to discover the path."* **So which route `realpath` takes on this
//! system depends on how the caller was compiled**, which is not a thing this
//! code can state about a library it did not build — so `F_GETPATH` is called
//! directly here rather than through `realpath`, and F12's third named call is
//! not relied on. On Linux `realpath` is the reading, where that qualification
//! does not arise.
//!
//! **macOS `statfs(2)`, read the same day** (`man 2 statfs`): the structure
//! carries *"char f_fstypename[MFSTYPENAMELEN]; /* fs type name */"*. That is
//! what separates APFS — the filesystem **Fact 16** observed — from every
//! other volume a Mac can mount.
//!
//! **The ext4 magic number is `libc`'s own source shipped on this disk**,
//! `libc-0.2.189/src/unix/linux_like/mod.rs`, read 2026-09-04:
//! `pub const EXT4_SUPER_MAGIC: c_long = 0x0000ef53;`. The behaviour it stands
//! for is the kernel's own documentation, quoted at F14a.
//!
//! **Windows `FileNormalizedNameInfo` is `[MS-FSCC]` 2.4.35's**, quoted at
//! F14a, with that specification's own product-behaviour note: the class *"is
//! implemented on ReFS and NTFS file systems. Other file systems return
//! STATUS_INVALID_DEVICE_REQUEST."*
//!
//! # Two Build decisions taken here, both recorded rather than assumed
//!
//! **A symbolic link is never descended into.** [`crate::search`]'s walk keeps
//! no visited set — it is a walk of the user's own folders and the plan gives
//! it none — so a link back up a tree would walk for ever. What it costs is a
//! project folder reached only through a link: its files are not searched.
//! **What it never costs is a wrong file**, and an unwalked folder is the same
//! outcome as a folder the user has not added.
//!
//! **A directory's own casing answer is taken once per directory** and never
//! per entry, which is where [`crate::search`]'s walk asks for it.
//!
//! **Markers used in this file that belong to another document, each with the
//! document it belongs to.** `Server protocol and transfers` S12, S22.
//! `Desktop app and delivery` C15, C16, F10a.

use std::fs;
use std::io::Read;

use smoozy_server_protocol::transfers::integrity::{Digest, Running};

use crate::bytes::StoredLocation;
use crate::contents::ContentIdentity;
use crate::readings::{EnumeratedEntry, EnumerationCasing, FolderListing, StoredNameAnswer};

/// How much is read at a time when a whole file is passed through the digest.
///
/// **Q4: the cost is processor rather than disk, and no rate is promised
/// anywhere.** The number is a Build choice with nothing resting on it; it is
/// one buffer rather than one allocation per file, because a `.blend` on this
/// project's own record is 669,966,622 bytes (Fact 15) and reading one into
/// memory to digest it would be a cost nothing asks for.
const THE_READ_BUFFER: usize = 1024 * 1024;

/// **The user's own computer, read.** One value, no state, and the only route
/// to the filesystem in this crate.
#[derive(Debug, Clone, Copy, Default)]
pub struct TheUsersOwnDisk;

impl TheUsersOwnDisk {
    /// **F8's enumeration of one folder.** The walk enumerates; **it never
    /// asks the disk whether two names are the same**, and this offers no
    /// lookup by name beside it.
    ///
    /// **F7: missing, moved or refused is one case and is not separated.**
    /// F1a says so in as many words — *"the verdict does not separate missing,
    /// moved and refused"* — so every failure to **open the folder** is one
    /// answer here.
    ///
    /// **F7's other half: a failure fails alone.** One entry that will not
    /// read is skipped and counted, and the rest of the listing stands —
    /// *could not be read* is answered only where the folder itself would not
    /// open. Until 2026-09-05 a single failing entry threw away every entry
    /// gathered so far, and where that folder was the first in the user's
    /// order its whole subtree went unwalked, so the folders below it supplied
    /// the answers and F4's order was quietly reversed.
    ///
    /// **A name crosses as the bytes the system returned**, never as text: a
    /// real disk holds names that do not decode, and a name put through a
    /// decode would key the wrong record and mirror a wrong spelling.
    pub fn enumerate_folder(&self, folder: &[u8]) -> FolderListing {
        let Some(path) = a_path(folder) else {
            return FolderListing::CouldNotBeRead;
        };
        let Ok(entries) = fs::read_dir(path) else {
            return FolderListing::CouldNotBeRead;
        };
        let mut out = Vec::new();
        let mut that_would_not_read = 0;
        for entry in entries {
            let Ok(entry) = entry else {
                that_would_not_read += 1;
                continue;
            };
            // **`file_type` here is the entry's own kind and does not follow a
            // link**, which is what keeps the walk finite: see the module's
            // own note.
            let is_a_folder = entry
                .file_type()
                .map(|kind| kind.is_dir() && !kind.is_symlink())
                .unwrap_or(false);
            out.push(EnumeratedEntry {
                name: name_bytes(&entry.file_name()),
                is_a_folder,
            });
        }
        FolderListing::Entries {
            entries: out,
            that_would_not_read,
        }
    }

    /// **What the casing an enumeration returns is worth on the filesystem
    /// holding this folder**, read from the system and never inferred from the
    /// names themselves.
    ///
    /// **Established for exactly two filesystems and claimed for no other**,
    /// which is F8's own narrowing: macOS's APFS (Fact 16, observed) and a
    /// case-folding ext4 directory (F14a, the kernel's own documentation).
    /// Everywhere else the answer is *not established* and exactness comes
    /// from the stored-name reading alone.
    pub fn enumeration_casing_of(&self, folder: &[u8]) -> EnumerationCasing {
        match the_filesystems_own_name(folder) {
            Some(name) if name == THE_ONE_MAC_FILESYSTEM_FACT_16_OBSERVED => {
                EnumerationCasing::IsTheStoredName
            }
            Some(name) if name == THE_ONE_LINUX_FILESYSTEM_F14A_CITES => {
                EnumerationCasing::IsTheStoredName
            }
            _ => EnumerationCasing::NotEstablished,
        }
    }

    /// **F12: every resolution records the file's whole location as stored on
    /// disk, read from the system and never rejoined from strings.**
    ///
    /// **F13's request, performed here until the adapter can host it** (see
    /// the module's own head), and **F14a's arrangement, which is the whole
    /// search's rather than Windows'**: exactness is decided by this reading
    /// and never by the enumeration's casing.
    pub fn the_stored_name_reading(&self, path: &[u8]) -> StoredNameAnswer {
        the_stored_name(path)
    }

    /// **Q2: on the user's computer the digest is computed by the application
    /// reading the file through.** `None` where the file cannot be read.
    ///
    /// **The pair is the content identity of Q1** — the size beside the
    /// digest, and the size is the count of bytes this read passed, never a
    /// value taken from a listing. **Q10 forbids reading a size as state**,
    /// and a size that came from the same pass as the digest is a product of
    /// the reading rather than a fact about the file's entry.
    pub fn the_local_files_content_identity(
        &self,
        file: &StoredLocation,
    ) -> Option<ContentIdentity> {
        let mut running = Running::new();
        pass_the_file_through(file.as_bytes(), u64::MAX, &mut running)?;
        Some(ContentIdentity {
            size: running.bytes_taken(),
            digest: running.so_far(),
        })
    }

    /// **A10: the digest of the source file's own prefix up to a checkpoint's
    /// offset** — the source read from the user's computer **as it now is**,
    /// for comparison against the checkpoint's recorded running digest
    /// (`Server protocol and transfers` S12, S22).
    ///
    /// **A source that cannot be read that far is a difference**, which is
    /// this reading answering `None` rather than a short digest: a digest over
    /// fewer bytes would compare unequal anyway, but saying so as an absence
    /// is what keeps the caller from reading a short read as a match.
    pub fn the_sources_prefix_digest(&self, source: &StoredLocation, up_to: u64) -> Option<Digest> {
        let mut running = Running::new();
        pass_the_file_through(source.as_bytes(), up_to, &mut running)?;
        if running.bytes_taken() < up_to {
            return None;
        }
        Some(running.so_far())
    }
}

/// Read `up_to` bytes of the file through `running`, or the whole file where
/// `up_to` is past its end. `None` where the file could not be opened or a
/// read failed part way — **a failed read is never a short answer**.
fn pass_the_file_through(path: &[u8], up_to: u64, running: &mut Running) -> Option<()> {
    let mut file = fs::File::open(a_path(path)?).ok()?;
    let mut buffer = vec![0u8; THE_READ_BUFFER];
    while running.bytes_taken() < up_to {
        let wanted = ((up_to - running.bytes_taken()) as usize).min(buffer.len());
        match file.read(&mut buffer[..wanted]) {
            Ok(0) => break,
            Ok(taken) => running.take(&buffer[..taken]),
            Err(error) if error.kind() == std::io::ErrorKind::Interrupted => continue,
            Err(_) => return None,
        }
    }
    Some(())
}

// ---------------------------------------------------------------- unix paths

#[cfg(unix)]
fn a_path(bytes: &[u8]) -> Option<std::path::PathBuf> {
    use std::os::unix::ffi::OsStrExt;
    Some(std::path::PathBuf::from(std::ffi::OsStr::from_bytes(bytes)))
}

#[cfg(unix)]
fn name_bytes(name: &std::ffi::OsStr) -> Vec<u8> {
    use std::os::unix::ffi::OsStrExt;
    name.as_bytes().to_vec()
}

/// **On Windows a path is UTF-16 and this crate carries paths as bytes**, so a
/// byte string that is not text is not a path this system can be asked about.
/// `Desktop app and delivery` C8 owns what Windows accepts in a name; nothing
/// here decides it.
#[cfg(windows)]
fn a_path(bytes: &[u8]) -> Option<std::path::PathBuf> {
    Some(std::path::PathBuf::from(std::str::from_utf8(bytes).ok()?))
}

#[cfg(windows)]
fn name_bytes(name: &std::ffi::OsStr) -> Vec<u8> {
    name.to_string_lossy().into_owned().into_bytes()
}

// ------------------------------------------------- what the filesystem is

/// APFS, the one Mac filesystem **Fact 16** observed — *"APFS stores names
/// byte for byte and normalises only its lookup hash"*. **HFS+ is not this**
/// and is one of the permanently unresolved filesystems.
const THE_ONE_MAC_FILESYSTEM_FACT_16_OBSERVED: &str = "apfs";

/// ext4, the one Linux filesystem **F14a** cites: *"The case-awareness is
/// name-preserving on the disk, meaning that the file name provided by
/// userspace is a byte-per-byte match to what is actually written in the
/// disk."*
const THE_ONE_LINUX_FILESYSTEM_F14A_CITES: &str = "ext4";

/// The filesystem holding a path, by the name this build knows it by, or
/// `None` where the system cannot say. **A name this file does not recognise
/// is `None`**: nothing is concluded from an unfamiliar filesystem.
#[cfg(target_os = "macos")]
fn the_filesystems_own_name(path: &[u8]) -> Option<&'static str> {
    let mut with_a_null = path.to_vec();
    with_a_null.push(0);
    let mut answer: libc::statfs = unsafe { std::mem::zeroed() };
    // Safety: the path is a null-terminated byte string this call reads and
    // does not keep, and the structure is this thread's own.
    let ok = unsafe { libc::statfs(with_a_null.as_ptr().cast(), &mut answer) } == 0;
    if !ok {
        return None;
    }
    let bytes: Vec<u8> = answer
        .f_fstypename
        .iter()
        .take_while(|byte| **byte != 0)
        .map(|byte| *byte as u8)
        .collect();
    (bytes == THE_ONE_MAC_FILESYSTEM_FACT_16_OBSERVED.as_bytes())
        .then_some(THE_ONE_MAC_FILESYSTEM_FACT_16_OBSERVED)
}

#[cfg(target_os = "linux")]
fn the_filesystems_own_name(path: &[u8]) -> Option<&'static str> {
    let mut with_a_null = path.to_vec();
    with_a_null.push(0);
    let mut answer: libc::statfs = unsafe { std::mem::zeroed() };
    // Safety: as above.
    let ok = unsafe { libc::statfs(with_a_null.as_ptr().cast(), &mut answer) } == 0;
    if !ok {
        return None;
    }
    (i64::from(answer.f_type) == i64::from(libc::EXT4_SUPER_MAGIC))
        .then_some(THE_ONE_LINUX_FILESYSTEM_F14A_CITES)
}

/// **Windows: not established, and never inferred.** F14 records the one
/// missing word — Microsoft states that the original long file name is
/// preserved on disk and that enumeration hands back the complete one, and
/// *"what nobody at Microsoft has written is the words *and its
/// capitalisation*"*. So the enumeration's casing is worth nothing here and
/// exactness comes from the normalized name alone.
#[cfg(windows)]
fn the_filesystems_own_name(_path: &[u8]) -> Option<&'static str> {
    None
}

#[cfg(not(any(target_os = "macos", target_os = "linux", windows)))]
fn the_filesystems_own_name(_path: &[u8]) -> Option<&'static str> {
    None
}

// ------------------------------------------------- the stored-name reading

/// **A file that will not open is not a file that is not there, and the two
/// are kept apart on every system.**
///
/// This crate has three answers for a stored-name reading and each has its own
/// consequence. **Not there** is a claim about the disk: `judge_candidate`
/// reads it as `NotAMatch` and `the_stored_location_of` as *the candidate is
/// no longer there*, and the user is told the file is not there. **The class
/// is refused** says only that Smoozy could not read the spelling, and the
/// search reports that limit instead of a false answer.
///
/// **So absence is claimed only where the system said absence** — `ENOENT`
/// alone on the two unix systems, and the standard library's *not found* on
/// Windows. A refused permission, a link loop, a lock, a name too long: every
/// one of those is a refusal, because the file is on the disk and this reading
/// could not open it. **Until 2026-09-05 all three halves answered *not there*
/// for any failure whatever**, so a texture the user could see was reported
/// missing.
///
/// The shape is `acting/machine.rs`'s for the same distinction on the command
/// channel: a refusal, a missing program and a file that is genuinely absent
/// are one shape on that surface, and the two answers have opposite
/// consequences, so the one that is a durable claim is never made from a
/// failure.
#[cfg(unix)]
fn absent_or_refused(errno: i32) -> StoredNameAnswer {
    if errno == libc::ENOENT {
        StoredNameAnswer::NotThere
    } else {
        StoredNameAnswer::TheClassIsRefused
    }
}

/// **macOS: `F_GETPATH`**, `fcntl(2)`'s own words quoted in this module's
/// head. One call, one whole location, no string rejoined.
///
/// **The file is opened without following a link** — `O_SYMLINK` — so the
/// location recorded is the one the walk enumerated rather than whatever it
/// points at. **`O_NOFOLLOW` is not the call here**: it fails on a link where
/// `O_SYMLINK` opens the link itself, and a link's own location is the answer
/// F12 wants for a name the enumeration returned.
#[cfg(target_os = "macos")]
fn the_stored_name(path: &[u8]) -> StoredNameAnswer {
    let mut with_a_null = path.to_vec();
    with_a_null.push(0);
    // Safety: a null-terminated byte string the call reads and does not keep.
    let descriptor = unsafe {
        libc::open(
            with_a_null.as_ptr().cast(),
            libc::O_RDONLY | libc::O_SYMLINK,
        )
    };
    if descriptor < 0 {
        // **A file that will not open is not a file that is not there** —
        // `errno` decides which, and only `ENOENT` is absence.
        return absent_or_refused(std::io::Error::last_os_error().raw_os_error().unwrap_or(0));
    }
    let mut buffer = [0i8; libc::PATH_MAX as usize];
    // Safety: the buffer is `MAXPATHLEN` as the manual page requires, and the
    // descriptor is this function's own.
    let answered = unsafe { libc::fcntl(descriptor, libc::F_GETPATH, buffer.as_mut_ptr()) } != -1;
    // Safety: the descriptor was opened above and is closed exactly once.
    unsafe { libc::close(descriptor) };
    if !answered {
        return StoredNameAnswer::TheClassIsRefused;
    }
    let bytes: Vec<u8> = buffer
        .iter()
        .take_while(|byte| **byte != 0)
        .map(|byte| *byte as u8)
        .collect();
    StoredNameAnswer::TheStoredName(StoredLocation::as_the_system_returned_it(&bytes))
}

/// **Linux: `realpath(3)`**, the system's own resolution of a path to an
/// absolute location.
///
/// **What this answer is worth is F14a's list and not this function's.** On a
/// case-folding ext4 directory the kernel's own documentation says the stored
/// name is the one userspace gave, byte for byte, and the casing answer above
/// already reports that; on a case-folding Linux filesystem that is not ext4
/// **there is no refusal** — the reading answers and nothing establishes that
/// what it hands back is the stored spelling. **That is the sharper of the two
/// residues and it is stated rather than worked around** (§12 items 11 and
/// 21). `Desktop app and delivery` C16 carries the ext4 citation.
///
/// # This half answers a different question from the macOS half, and it is
/// stated rather than left to be found
///
/// **`realpath` resolves every link in the path.** So where a candidate the
/// walk enumerated is a link, or sits under a linked folder, this records
/// **the target's** whole location — while the macOS half opens with
/// `O_SYMLINK` on purpose and records **the link's own** location, which is
/// the answer F12 wants for a name the enumeration returned.
///
/// **What that costs on Linux.** The mirror (A4) is then composed from the
/// shared library's own path rather than from the project's, so a file the
/// user reaches through a link lands under a location they did not point at;
/// and G11's rule that two references reaching one file by two names on disk
/// are two things is not kept — the two silently become one mirror and one
/// row.
///
/// **`realpath` cannot be told not to resolve**, and there is **no Linux
/// desktop here** to establish another call against that system's own
/// publication. §12 item 11 already carries the Linux desktop as unresolved
/// and unscheduled, and this sits under it: the honest statement is what
/// holds, not a call nobody can cite. **No space is left here for an answer.**
#[cfg(target_os = "linux")]
fn the_stored_name(path: &[u8]) -> StoredNameAnswer {
    let mut with_a_null = path.to_vec();
    with_a_null.push(0);
    // Safety: a null-terminated byte string the call reads; the returned
    // buffer is this call's own allocation and is freed below.
    let resolved = unsafe { libc::realpath(with_a_null.as_ptr().cast(), std::ptr::null_mut()) };
    if resolved.is_null() {
        // **The same distinction as the macOS half**, and the fault was the
        // same on both: `realpath` sets `errno`, and a refused permission, a
        // link loop or a name too long is a refusal rather than absence.
        return absent_or_refused(std::io::Error::last_os_error().raw_os_error().unwrap_or(0));
    }
    // Safety: the pointer is a null-terminated string this call allocated.
    let bytes = unsafe { std::ffi::CStr::from_ptr(resolved) }
        .to_bytes()
        .to_vec();
    // Safety: freeing the allocation the call made, exactly once.
    unsafe { libc::free(resolved.cast()) };
    StoredNameAnswer::TheStoredName(StoredLocation::as_the_system_returned_it(&bytes))
}

/// **Windows: `GetFileInformationByHandleEx` with `FileNormalizedNameInfo`**,
/// which `[MS-FSCC]` 2.4.35 defines as *"an absolute pathname where each short
/// name component has been replaced with the corresponding long name
/// component, and each name component uses the exact letter casing stored on
/// disk"* (F14a).
///
/// **The refusal is the specification's own product-behaviour note** — the
/// class *"is implemented on ReFS and NTFS file systems. Other file systems
/// return STATUS_INVALID_DEVICE_REQUEST."* Such a volume **says it cannot
/// answer rather than handing back a wrong spelling**, and the search reports
/// that limit instead of a false exact match (§12 item 3).
///
/// **What this returns is the whole location as the system gave it (F12)**,
/// never a component cut out of it. **F14a's last component is taken from that
/// whole value** by [`StoredLocation::final_segment`], which splits at the
/// user's own computer's separators — the backslash here — and that is the
/// only place a component is taken.
///
/// Whether the answer carries a volume prefix is not settled by the pages read
/// and is A4's question rather than this one's (F14a); nothing here adds or
/// removes one.
///
/// **Whether the casing this call hands back is the stored casing is still
/// unrun** (§12 item 21) — the Windows reading is permanently unresolved on
/// the Creator's ruling of 2026-08-31, and no space is left here for that
/// result. **What has been seen is narrower and is stated rather than
/// stretched:** on a hosted Windows runner on 2026-09-05 the call answered
/// rather than refusing, and its answer was an absolute pathname with
/// backslashes between the components and no drive letter in front of it.
/// Label: **observed in a test**. That says nothing about capitalisation,
/// which is the part item 21 is about.
///
/// **The handle is opened through the standard library rather than through
/// `CreateFileW`, and that is a Build decision.** The two open the same object
/// with the same three flags — `READ_CONTROL`, the three share modes, backup
/// semantics so a directory opens, and open-reparse-point so a link is not
/// followed — and this route closes the handle when the value is dropped, on
/// every path out including a panic, where the direct call needs a close nobody
/// may skip. **It also asks for no widening of the shared Windows binding's
/// feature list**, which `CreateFileW` does need and which is a file every
/// module shares.
#[cfg(windows)]
fn the_stored_name(path: &[u8]) -> StoredNameAnswer {
    use std::os::windows::fs::OpenOptionsExt;
    use std::os::windows::io::AsRawHandle;
    use windows_sys::Win32::Storage::FileSystem::{
        FileNormalizedNameInfo, GetFileInformationByHandleEx, FILE_FLAG_BACKUP_SEMANTICS,
        FILE_FLAG_OPEN_REPARSE_POINT, FILE_SHARE_DELETE, FILE_SHARE_READ, FILE_SHARE_WRITE,
        READ_CONTROL,
    };

    let Some(as_a_path) = a_path(path) else {
        // A byte string that is not text cannot be handed to this system's own
        // call, so the system is not asked and nothing is claimed.
        return StoredNameAnswer::TheClassIsRefused;
    };
    let handle = match std::fs::OpenOptions::new()
        .access_mode(READ_CONTROL)
        .share_mode(FILE_SHARE_READ | FILE_SHARE_WRITE | FILE_SHARE_DELETE)
        .custom_flags(FILE_FLAG_BACKUP_SEMANTICS | FILE_FLAG_OPEN_REPARSE_POINT)
        .open(&as_a_path)
    {
        Ok(handle) => handle,
        // **The same distinction as the two unix halves**, found by sweeping
        // the class rather than by the finding, which named macOS and Linux:
        // the standard library's *not found* is absence, and every other
        // failure to open — a refused permission, a sharing violation, a
        // reparse loop — is a refusal, because the file is on the disk and
        // this reading could not open it.
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            return StoredNameAnswer::NotThere
        }
        Err(_) => return StoredNameAnswer::TheClassIsRefused,
    };
    // The structure is a length followed by the name's own bytes, so the
    // buffer holds the longest path this system names plus that length.
    let mut buffer = vec![0u8; 4 + 2 * 32_768];
    // Safety: the handle is this function's own and lives across the call, and
    // the buffer is sized above and its length is handed in.
    let answered = unsafe {
        GetFileInformationByHandleEx(
            handle.as_raw_handle() as _,
            FileNormalizedNameInfo,
            buffer.as_mut_ptr().cast(),
            buffer.len() as u32,
        )
    } != 0;
    drop(handle);
    // **Every failure of the query is the one answer, and the reason is not
    // read.** The documented refusal is `STATUS_INVALID_DEVICE_REQUEST` on a
    // volume that is neither NTFS nor ReFS (F14a's product-behaviour note), and
    // this crate has exactly three answers for a stored-name reading — the
    // stored name, the class refused, and not there. **A failure of some other
    // kind has the same consequence and there is no honest fourth value for
    // it**: the reading did not supply the stored casing, so F14a's arrangement
    // takes the refusal branch, the row carries no proposal and no false exact
    // match is possible. Separating the reasons would be a distinction nothing
    // downstream can act on, and mapping an unknown error to *not there* would
    // be a claim about the disk this call did not make.
    if !answered {
        return StoredNameAnswer::TheClassIsRefused;
    }
    let bytes_of_name = u32::from_le_bytes([buffer[0], buffer[1], buffer[2], buffer[3]]) as usize;
    // The declared length is the system's and is bounded against the buffer
    // before it indexes it — a length past the end is not trimmed to fit and
    // read as a short name, which would be a spelling this call never gave.
    if bytes_of_name % 2 != 0 || 4 + bytes_of_name > buffer.len() {
        return StoredNameAnswer::TheClassIsRefused;
    }
    let units: Vec<u16> = buffer[4..4 + bytes_of_name]
        .chunks_exact(2)
        .map(|pair| u16::from_le_bytes([pair[0], pair[1]]))
        .collect();
    match String::from_utf16(&units) {
        Ok(text) => StoredNameAnswer::TheStoredName(StoredLocation::as_the_system_returned_it(
            text.as_bytes(),
        )),
        // A normalized name that is not text says nothing this crate can
        // compare, and nothing is substituted for it.
        Err(_) => StoredNameAnswer::TheClassIsRefused,
    }
}

#[cfg(not(any(target_os = "macos", target_os = "linux", windows)))]
fn the_stored_name(_path: &[u8]) -> StoredNameAnswer {
    StoredNameAnswer::TheClassIsRefused
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::search::{judge_candidate, CandidateVerdict};

    /// A scratch folder of this test's own, under the system's temporary
    /// directory. Removed by the test that made it.
    ///
    /// **The process id is in the name and the name is nothing without it.**
    /// This folder was named after the test alone until 2026-09-18, so two test
    /// runs at once on one computer used the one folder and deleted each
    /// other's files: measured here beforehand, two runs at once failed 30 of
    /// 40 while one run alone failed 0 of 20. It is the same fault that was
    /// diagnosed that day in the desktop crate's
    /// `src/presentation/the_two_hooks.rs`.
    fn a_scratch(name: &str) -> std::path::PathBuf {
        let folder = std::env::temp_dir()
            .join(format!("smoozy-dependencies-{name}-{}", std::process::id()));
        let _ = fs::remove_dir_all(&folder);
        fs::create_dir_all(&folder).expect("a scratch folder");
        folder
    }

    #[cfg(unix)]
    fn as_bytes(path: &std::path::Path) -> Vec<u8> {
        use std::os::unix::ffi::OsStrExt;
        path.as_os_str().as_bytes().to_vec()
    }

    #[cfg(windows)]
    fn as_bytes(path: &std::path::Path) -> Vec<u8> {
        path.to_string_lossy().into_owned().into_bytes()
    }

    /// **F8's enumeration, over a real folder.** Every entry's name comes back
    /// as the bytes the system returned, folders are told from files, and a
    /// folder that is not there is one answer rather than three.
    #[test]
    fn the_walk_enumerates_a_real_folder_and_an_absent_one_is_one_answer() {
        let scratch = a_scratch("enumerate");
        fs::write(scratch.join("METallic.png"), b"red").expect("a file");
        fs::create_dir(scratch.join("below")).expect("a folder");

        let FolderListing::Entries {
            mut entries,
            that_would_not_read,
        } = TheUsersOwnDisk.enumerate_folder(&as_bytes(&scratch))
        else {
            panic!("the folder listed");
        };
        assert_eq!(that_would_not_read, 0);
        entries.sort_by(|one, other| one.name.cmp(&other.name));
        assert_eq!(entries.len(), 2);
        assert_eq!(entries[0].name, b"METallic.png".to_vec());
        assert!(!entries[0].is_a_folder);
        assert_eq!(entries[1].name, b"below".to_vec());
        assert!(entries[1].is_a_folder);

        assert_eq!(
            TheUsersOwnDisk.enumerate_folder(&as_bytes(&scratch.join("nothing"))),
            FolderListing::CouldNotBeRead
        );
        let _ = fs::remove_dir_all(&scratch);
    }

    /// **A file that will not open is not a file that is not there**, run on
    /// this Mac against a real refusal.
    ///
    /// **This is the test that would have caught it**: every failure to open
    /// answered *not there*, so a texture the user could see was reported
    /// missing — `judge_candidate` reading it as no match and
    /// `the_stored_location_of` as *the candidate is no longer there*.
    ///
    /// **The refusal is made by taking the execute bit off the holding
    /// folder**, which is what stops a name inside it being looked up at all;
    /// the file itself is untouched and is still on the disk.
    #[cfg(unix)]
    #[test]
    fn a_refused_permission_is_the_class_refused_and_never_not_there() {
        use std::os::unix::fs::PermissionsExt;

        let scratch = a_scratch("refused");
        let shut = scratch.join("shut");
        fs::create_dir(&shut).expect("a folder");
        let inside = shut.join("metallic.png");
        fs::write(&inside, b"red").expect("a file");

        // A name that is genuinely absent is absence, before anything is shut.
        assert_eq!(
            TheUsersOwnDisk.the_stored_name_reading(&as_bytes(&shut.join("gone.png"))),
            StoredNameAnswer::NotThere
        );

        fs::set_permissions(&shut, fs::Permissions::from_mode(0o000)).expect("shut");
        let answer = TheUsersOwnDisk.the_stored_name_reading(&as_bytes(&inside));
        fs::set_permissions(&shut, fs::Permissions::from_mode(0o755)).expect("open again");

        // A test run as a user the permission does not bind proves nothing
        // here, and says so rather than passing quietly.
        if answer
            == StoredNameAnswer::TheStoredName(StoredLocation::as_the_system_returned_it(
                &as_bytes(&inside),
            ))
        {
            eprintln!(
                "this login is not bound by the folder's permissions; the refusal was not posed"
            );
        } else {
            assert_eq!(
                answer,
                StoredNameAnswer::TheClassIsRefused,
                "a file that would not open was reported as not there"
            );
        }
        let _ = fs::remove_dir_all(&scratch);
    }

    /// **F12 and F14a, run rather than argued, on the one system the search is
    /// established on.**
    ///
    /// A file is created spelled `METallic.png` and asked about by a different
    /// spelling. **On a case-insensitive volume the open succeeds and the
    /// stored-name reading must come back with the spelling on disk**, which
    /// is the whole of what separates F9's exact match from F10's proposal.
    /// **On a case-sensitive volume the open fails and the answer is *not
    /// there*, which is F9's match failing for the same reason** — either way
    /// no wrong file is taken, and this test asserts whichever of the two this
    /// disk gives rather than assuming one.
    #[test]
    fn the_stored_name_reading_answers_with_the_spelling_on_disk() {
        let scratch = a_scratch("stored-name");
        let stored = scratch.join("METallic.png");
        fs::write(&stored, b"red").expect("a file");

        // Asked by its own spelling, every system answers with it.
        match TheUsersOwnDisk.the_stored_name_reading(&as_bytes(&stored)) {
            StoredNameAnswer::TheStoredName(location) => {
                assert_eq!(location.final_segment(), b"METallic.png");
            }
            other => panic!("the file is there and the reading said {other:?}"),
        }

        // Asked by another spelling, the answer is the disk's own.
        let asked = scratch.join("metallic.png");
        match TheUsersOwnDisk.the_stored_name_reading(&as_bytes(&asked)) {
            // A case-insensitive volume: the reading must hand back the
            // stored spelling, not the one it was asked with.
            StoredNameAnswer::TheStoredName(location) => {
                assert_eq!(
                    location.final_segment(),
                    b"METallic.png",
                    "the reading handed back the spelling it was asked with, \
                     which is the wrong-file harm F12 exists against"
                );
            }
            // A case-sensitive volume: there is no such file.
            StoredNameAnswer::NotThere => {}
            other => panic!("the reading said {other:?}"),
        }
        let _ = fs::remove_dir_all(&scratch);
    }

    /// **F9 and F10 end to end over the real readings**, which is the pair
    /// Row 166 exists for: a capitals-only difference is **not** a match, and
    /// it surfaces as a proposal instead.
    #[test]
    fn a_capitals_only_difference_is_a_proposal_and_never_a_match() {
        let scratch = a_scratch("near-miss");
        fs::write(scratch.join("METallic.png"), b"red").expect("a file");
        let folder = as_bytes(&scratch);
        let casing = TheUsersOwnDisk.enumeration_casing_of(&folder);
        let path = as_bytes(&scratch.join("METallic.png"));

        assert_eq!(
            judge_candidate(b"METallic.png", b"METallic.png", casing, || {
                TheUsersOwnDisk.the_stored_name_reading(&path)
            }),
            CandidateVerdict::Exact
        );
        assert_eq!(
            judge_candidate(b"metallic.png", b"METallic.png", casing, || {
                TheUsersOwnDisk.the_stored_name_reading(&path)
            }),
            CandidateVerdict::AProposal
        );
        assert_eq!(
            judge_candidate(b"other.png", b"METallic.png", casing, || {
                TheUsersOwnDisk.the_stored_name_reading(&path)
            }),
            CandidateVerdict::NotAMatch
        );
        let _ = fs::remove_dir_all(&scratch);
    }

    /// **Q2 and A10 over a real file**, and the prefix digest is the prefix's
    /// own rather than the whole file's.
    #[test]
    fn the_digest_and_the_prefix_digest_are_taken_by_reading_the_file_through() {
        let scratch = a_scratch("digest");
        let file = scratch.join("big.blend");
        // Larger than one read buffer, so the loop runs more than once.
        let bytes: Vec<u8> = (0..(THE_READ_BUFFER + 1_234) as u32)
            .map(|n| (n % 251) as u8)
            .collect();
        fs::write(&file, &bytes).expect("a file");
        let at = StoredLocation::as_the_system_returned_it(&as_bytes(&file));

        let identity = TheUsersOwnDisk
            .the_local_files_content_identity(&at)
            .expect("the file reads");
        assert_eq!(identity.size, bytes.len() as u64);
        assert_eq!(identity.digest, Digest::of(&bytes));

        let prefix = TheUsersOwnDisk
            .the_sources_prefix_digest(&at, 4_096)
            .expect("the prefix reads");
        assert_eq!(prefix, Digest::of(&bytes[..4_096]));
        assert_ne!(prefix, identity.digest);

        // **A source that cannot be read that far is a difference**, not a
        // short digest.
        assert_eq!(
            TheUsersOwnDisk.the_sources_prefix_digest(&at, bytes.len() as u64 + 1),
            None
        );
        // And a file that is not there answers nothing at all.
        assert_eq!(
            TheUsersOwnDisk.the_local_files_content_identity(
                &StoredLocation::as_the_system_returned_it(&as_bytes(&scratch.join("gone")))
            ),
            None
        );
        let _ = fs::remove_dir_all(&scratch);
    }

    /// **The casing answer is a reading of the system and never of the
    /// names.** What it says on this desk is whatever this disk is; what is
    /// asserted is that it is one of the two values and that it does not
    /// change with the names in the folder.
    #[test]
    fn the_casing_answer_comes_from_the_filesystem_and_not_from_the_names() {
        let scratch = a_scratch("casing");
        fs::write(scratch.join("a.png"), b"a").expect("a file");
        let first = TheUsersOwnDisk.enumeration_casing_of(&as_bytes(&scratch));
        fs::write(scratch.join("B.PNG"), b"b").expect("a file");
        assert_eq!(
            first,
            TheUsersOwnDisk.enumeration_casing_of(&as_bytes(&scratch))
        );
        // A folder that is not there establishes nothing.
        assert_eq!(
            TheUsersOwnDisk.enumeration_casing_of(&as_bytes(&scratch.join("gone"))),
            EnumerationCasing::NotEstablished
        );
        let _ = fs::remove_dir_all(&scratch);
    }
}
