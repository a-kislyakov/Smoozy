//! A durable write, and it is the one every store in this module goes through.
//!
//! **`State, identity and recovery` §1.3 rule 6 defines the word and this is
//! that definition performed.** Its own sentence: *"A write is durable when the
//! following has completed, and not before. The bytes are written; the file is
//! flushed to the storage device; where the write is published by a name, the
//! rename follows the flush; and the directory holding that name is flushed
//! too."*
//!
//! **The last clause is the one that is ordinarily missed**, and that document
//! quotes `fsync(2)` for it — the Linux man-pages project, read 2026-08-20 at
//! `https://man7.org/linux/man-pages/man2/fsync.2.html`: *"Calling fsync() does
//! not necessarily ensure that the entry in the directory containing the file
//! has also reached disk. For that an explicit fsync() on a file descriptor for
//! the directory is also needed."*
//!
//! **Which call, and why this module takes the ordinary one.** That same rule
//! says the call differs by system and that *"On Linux — every rented machine —
//! `fsync` reaches the device"*, the full-flush form being the user's own
//! computer's. **Every write in this module happens on a rented machine**, so
//! the ordinary flush is the one rule 6 names for it, and `File::sync_all` is
//! the standard library's own name for that call.
//!
//! **The rename's atomicity is a separate guarantee and is not durability.**
//! `rename(2)`, quoted by this crate's plan at G7 and read 2026-08-20 at
//! `https://man7.org/linux/man-pages/man2/rename.2.html`: *"If newpath already
//! exists, it will be atomically replaced, so that there is no point at which
//! another process attempting to access newpath will find it missing."*
//!
//! # The temporary name, and the litter nothing sweeps
//!
//! A publication stages beside its final name and renames. **Where the machine
//! dies between the two, the staged file stays**, and this module writes no
//! sweep for it — which is this plan's own Section 16 item 3's live finding,
//! observed on all three filesystems: the temporary file an interrupted hold
//! publication leaves *"survived hours, several supervisor restarts and full
//! reconciliation on the container, and survived on the virtual machine too"*,
//! and G20's sweep is bounded to the locations the phase record names and does
//! not reach it. **The clause that would remove it is not written**, so no
//! sweep is written here either.
//!
//! **What the fixed temporary name does and does not change.** The name is
//! derived from the final name, so a retry of the same write reuses one file
//! rather than leaving a fresh one each time; nothing reads it, ever, and
//! `State, identity and recovery` §1.3 rule 5 forbids reading a file's presence
//! or name as state anywhere in this document. It does not close the finding: a
//! write that is never retried leaves its file for good.
//!
//! # The second shape of write, added 2026-09-06 with the event journal
//!
//! **Rule 6's second clause is conditional and until now nothing here had a
//! write it did not reach**: *"where the write is published by a name, the
//! rename follows the flush"*. **An append is published by no name.** Its bytes
//! go on the end of a file that already has its directory entry, so there is no
//! rename to order and no new entry to flush — **the whole of rule 6 for it is
//! the bytes written and the file flushed to the storage device**, which is the
//! first two steps and exactly what that rule's own quotation of `fsync(2)`
//! covers.
//!
//! **[`AFileToAppendTo`] is that write, and it is here rather than beside the
//! journal for the reason every other write in this module is: one writer, one
//! definition of the word *durable*, and one place a builder has to look.**
//! Publishing every journal entry by a name instead would have been a second
//! form of the same act and worse in two ways — an entry would cost a rewrite of
//! everything before it, and **the divergence this plan's own Section 16 item 26
//! records would be gone**: a rename needs a new directory entry, so a journal
//! that renamed would refuse in the same instant the record store does, where
//! the observed arrangement's appends went on landing.

use std::fs::{self, File, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};

/// What went wrong, verbatim, with the path it went wrong at.
///
/// **The machine's own words are kept.** Nothing here interprets an error into
/// a cause; a refused write on a full mount is the loud `errno 28` one and the
/// caller records the reason once (S14, M5).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TheWriteWasRefused {
    /// The path the attempt was made at.
    pub at: String,
    /// What the machine said.
    pub what_the_machine_said: String,
}

impl TheWriteWasRefused {
    fn at(path: &Path, why: std::io::Error) -> Self {
        TheWriteWasRefused {
            at: path.display().to_string(),
            what_the_machine_said: why.to_string(),
        }
    }

    /// The shape every trait in this crate carries for a failure.
    pub fn as_text(&self) -> String {
        format!("{}: {}", self.at, self.what_the_machine_said)
    }
}

/// The staging name a publication at `final_name` uses.
pub fn the_staging_name_for(final_name: &Path) -> PathBuf {
    let mut staging = final_name.as_os_str().to_os_string();
    staging.push(".writing");
    PathBuf::from(staging)
}

/// **Rule 6's four steps, in order: write, flush the file, rename, flush the
/// directory.**
///
/// This is the one write every store in this module makes. It is also §14's
/// *staging and one publication* for the two actions declared atomic there —
/// placing a hold record, and the phase record's own write.
pub fn publish(final_name: &Path, bytes: &[u8]) -> Result<(), TheWriteWasRefused> {
    let staging = the_staging_name_for(final_name);
    let mut file = File::create(&staging).map_err(|why| TheWriteWasRefused::at(&staging, why))?;
    file.write_all(bytes)
        .map_err(|why| TheWriteWasRefused::at(&staging, why))?;
    // Rule 6 step 2. Before the rename and not after it: a rename that
    // publishes unflushed bytes publishes a name whose contents may not be on
    // the device.
    file.sync_all()
        .map_err(|why| TheWriteWasRefused::at(&staging, why))?;
    drop(file);
    fs::rename(&staging, final_name).map_err(|why| TheWriteWasRefused::at(&staging, why))?;
    flush_the_directory_holding(final_name)
}

/// **Rule 6's fourth step on its own**, for a publication whose deciding write
/// is a rename somebody else performed — the pointer switch of G7, and the
/// removal of a durable state.
pub fn flush_the_directory_holding(name: &Path) -> Result<(), TheWriteWasRefused> {
    let directory = name.parent().unwrap_or_else(|| Path::new("."));
    // A directory is opened read-only for this; there is nothing to write into
    // it and the flush is of the entry the rename made.
    let handle = File::open(directory).map_err(|why| TheWriteWasRefused::at(directory, why))?;
    handle
        .sync_all()
        .map_err(|why| TheWriteWasRefused::at(directory, why))
}

/// A durable removal: the name goes and the directory holding it is flushed.
///
/// **P5's clear needs this** — a state whose file went but whose directory
/// entry did not reach the device is a state that comes back after a machine
/// that did not come back cleanly, and P4 is the clause that would then be
/// keeping a promise it had already broken.
pub fn remove_durably(name: &Path) -> Result<(), TheWriteWasRefused> {
    match fs::remove_file(name) {
        Ok(()) => flush_the_directory_holding(name),
        Err(why) if why.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Err(why) => Err(TheWriteWasRefused::at(name, why)),
    }
}

/// Read a record back, or `None` where there is none.
///
/// **An absent record is an answer rather than an error**, which is G31a's
/// never-held case — *"on a machine that has never held a package every record
/// is absent, which is the honest never-held answer"*.
pub fn read(name: &Path) -> Result<Option<Vec<u8>>, TheWriteWasRefused> {
    match fs::read(name) {
        Ok(bytes) => Ok(Some(bytes)),
        Err(why) if why.kind() == std::io::ErrorKind::NotFound => Ok(None),
        Err(why) => Err(TheWriteWasRefused::at(name, why)),
    }
}

/// Make the area a store sits in, with the permission its own clause requires.
///
/// **Z3 is the clause for the endpoint's directory** — *"created by the
/// placement, owned by the login the supervisor runs as, and permits that login
/// alone"* — and the package area's own stores are made the same way, since a
/// login that cannot reach them cannot perform the transaction at all.
pub fn make_the_area(area: &Path) -> Result<(), TheWriteWasRefused> {
    fs::create_dir_all(area).map_err(|why| TheWriteWasRefused::at(area, why))?;
    permit_that_login_alone(area)
}

/// The login the supervisor runs as, and nobody else.
///
/// **`Security and licence` §8 requirement 1 is what this keeps**, and Z11
/// states its limit in the same breath: it separates one login from another and
/// does not separate Smoozy from the machine's owner, root ignoring file
/// permissions and the login Smoozy holds being root on some rentals.
pub fn permit_that_login_alone(name: &Path) -> Result<(), TheWriteWasRefused> {
    use std::os::unix::fs::PermissionsExt;
    let mut permission = fs::metadata(name)
        .map_err(|why| TheWriteWasRefused::at(name, why))?
        .permissions();
    permission.set_mode(0o700);
    fs::set_permissions(name, permission).map_err(|why| TheWriteWasRefused::at(name, why))
}

/// **A file that is written by appending to it, held open for the life of the
/// store that appends.**
///
/// **The handle is held rather than reopened at every append**, which is the
/// arrangement Section 16 item 26 observed: *"an append to an already-open file
/// goes on succeeding until that file's last allocated block is exactly full"*.
/// Reopening at every append would put a directory lookup in front of every one
/// and would make the store's conduct on a full mount depend on that lookup
/// rather than on the append.
#[derive(Debug)]
pub struct AFileToAppendTo {
    at: PathBuf,
    handle: File,
}

impl AFileToAppendTo {
    /// **Open it, making it where it is not there** — and where the create made
    /// a new directory entry, flush the directory holding it.
    ///
    /// **That flush is rule 6's fourth step and it is taken once, at the
    /// creation, rather than at every append.** A file whose entry has not
    /// reached the device is a file that is not there after a machine that did
    /// not come back cleanly, however well its own bytes were flushed; an append
    /// to a file whose entry is already on the device adds no entry and needs no
    /// second flush.
    pub fn opened(at: &Path) -> Result<Self, TheWriteWasRefused> {
        let already_there = at.exists();
        let handle = OpenOptions::new()
            .append(true)
            .create(true)
            .open(at)
            .map_err(|why| TheWriteWasRefused::at(at, why))?;
        if !already_there {
            flush_the_directory_holding(at)?;
        }
        Ok(AFileToAppendTo {
            at: at.to_path_buf(),
            handle,
        })
    }

    /// **Rule 6 for a write published by no name: the bytes are written and the
    /// file is flushed to the storage device.**
    pub fn append(&mut self, bytes: &[u8]) -> Result<(), TheWriteWasRefused> {
        self.handle
            .write_all(bytes)
            .map_err(|why| TheWriteWasRefused::at(&self.at, why))?;
        self.handle
            .sync_all()
            .map_err(|why| TheWriteWasRefused::at(&self.at, why))
    }

    /// Cut the file back to `to`, and flush it.
    ///
    /// **This is the one repair this module performs and it removes only what
    /// never completed.** A process killed part-way through an append can leave
    /// bytes on the device with no ending behind them; appending after those
    /// bytes would join two half-records into one line nobody wrote. The caller
    /// decides where the last complete record ends — this only performs the cut.
    pub fn cut_back_to(&mut self, to: u64) -> Result<(), TheWriteWasRefused> {
        self.handle
            .set_len(to)
            .map_err(|why| TheWriteWasRefused::at(&self.at, why))?;
        self.handle
            .sync_all()
            .map_err(|why| TheWriteWasRefused::at(&self.at, why))
    }

    /// The path it appends to.
    pub fn at(&self) -> &Path {
        &self.at
    }

    /// **How long the file is now**, taken from the open handle rather than from
    /// the path.
    ///
    /// **It is what an append is put back to where the append fails part-way.**
    /// A write that stops after some of its bytes leaves a stump on the device,
    /// and appending after a stump joins two half-records into one line nobody
    /// wrote — the same hazard [`Self::cut_back_to`] exists for at a start,
    /// arriving inside one run instead.
    pub fn how_long_it_is(&self) -> Result<u64, TheWriteWasRefused> {
        self.handle
            .metadata()
            .map(|about| about.len())
            .map_err(|why| TheWriteWasRefused::at(&self.at, why))
    }
}

/// Create a file that must not already exist, and hand back the handle.
///
/// **This is the exclusive create G1b names as the claim**, performed here by
/// the machine-side holder rather than through the file-transfer subsystem:
/// *"the exclusive create of the lock file itself"*. `create_new` is the
/// standard library's own name for it.
pub fn create_only_if_absent(name: &Path) -> Result<Option<File>, TheWriteWasRefused> {
    match OpenOptions::new().write(true).create_new(true).open(name) {
        Ok(file) => Ok(Some(file)),
        Err(why) if why.kind() == std::io::ErrorKind::AlreadyExists => Ok(None),
        Err(why) => Err(TheWriteWasRefused::at(name, why)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn an_area() -> PathBuf {
        let area = std::env::temp_dir().join(format!(
            "smoozy-runtime-durable-{}-{:?}",
            std::process::id(),
            {
                static NEXT: std::sync::atomic::AtomicUsize =
                    std::sync::atomic::AtomicUsize::new(0);
                NEXT.fetch_add(1, std::sync::atomic::Ordering::SeqCst)
            }
        ));
        make_the_area(&area).expect("the area is made");
        area
    }

    #[test]
    fn a_publication_lands_at_the_final_name_and_leaves_no_staging_file() {
        let area = an_area();
        let name = area.join("phase");
        publish(&name, b"a phase record").expect("it publishes");
        assert_eq!(read(&name).unwrap().as_deref(), Some(&b"a phase record"[..]));
        assert!(!the_staging_name_for(&name).exists());
        fs::remove_dir_all(&area).ok();
    }

    /// **The finding this module refuses to repair, made visible.** A staging
    /// file left behind by an interrupted publication is not swept, and this
    /// test is what says so rather than a comment nobody runs.
    ///
    /// **Nothing else happens here on purpose.** A publication over the same
    /// final name consumes the staging file — the rename moves it — so a test
    /// that published would show the opposite of what this one is for, which is
    /// that **the residue stands**. That is the plan's own Section 16 item 3 and
    /// no sweep is written for it, here or anywhere.
    #[test]
    fn a_staging_file_left_behind_stands_and_is_swept_by_nothing_here() {
        let area = an_area();
        let name = area.join("hold");
        let staging = the_staging_name_for(&name);
        fs::write(&staging, b"half a hold").expect("the staging file is made");
        assert!(
            staging.exists(),
            "the residue this module leaves is the residue this module leaves"
        );
        assert_eq!(
            read(&staging).unwrap().as_deref(),
            Some(&b"half a hold"[..]),
            "nothing anywhere removes it or writes over it"
        );
        // And the final name it would have become is not there either: an
        // interrupted publication published nothing.
        assert_eq!(read(&name).unwrap(), None);
        fs::remove_dir_all(&area).ok();
    }

    /// **The one thing the fixed temporary name does change**: a retry of the
    /// same write reuses one staging file rather than leaving a fresh one each
    /// time. **It does not close the finding above** — a write that is never
    /// retried leaves its file for good.
    #[test]
    fn a_retry_of_the_same_write_reuses_the_one_staging_name() {
        let area = an_area();
        let name = area.join("hold");
        let staging = the_staging_name_for(&name);
        fs::write(&staging, b"half a hold").expect("the staging file is made");
        publish(&name, b"a whole hold").expect("a later publication lands");
        assert_eq!(read(&name).unwrap().as_deref(), Some(&b"a whole hold"[..]));
        assert!(!staging.exists(), "the retry reused the one staging name");
        fs::remove_dir_all(&area).ok();
    }

    #[test]
    fn an_absent_record_reads_as_none_rather_than_as_a_refusal() {
        let area = an_area();
        assert_eq!(read(&area.join("never written")).unwrap(), None);
        fs::remove_dir_all(&area).ok();
    }

    #[test]
    fn a_removal_of_something_that_is_not_there_is_not_a_refusal() {
        let area = an_area();
        remove_durably(&area.join("never written")).expect("it is not a refusal");
        fs::remove_dir_all(&area).ok();
    }

    #[test]
    fn a_removal_takes_the_name_away() {
        let area = an_area();
        let name = area.join("setup in progress");
        publish(&name, b"set").expect("it publishes");
        remove_durably(&name).expect("it is removed");
        assert_eq!(read(&name).unwrap(), None);
        fs::remove_dir_all(&area).ok();
    }

    #[test]
    fn the_exclusive_create_refuses_the_second_caller() {
        let area = an_area();
        let name = area.join("claim");
        assert!(create_only_if_absent(&name).unwrap().is_some());
        assert!(
            create_only_if_absent(&name).unwrap().is_none(),
            "the second exclusive create must not succeed"
        );
        fs::remove_file(&name).unwrap();
        assert!(
            create_only_if_absent(&name).unwrap().is_some(),
            "it succeeds again once the file is gone"
        );
        fs::remove_dir_all(&area).ok();
    }

    #[test]
    fn the_area_permits_that_login_alone() {
        use std::os::unix::fs::PermissionsExt;
        let area = an_area();
        let mode = fs::metadata(&area).unwrap().permissions().mode() & 0o777;
        assert_eq!(mode, 0o700);
        fs::remove_dir_all(&area).ok();
    }

    /// **An append lands on the end, and a later opening of the same file goes
    /// on from where the last one stopped** — which is what a supervisor restart
    /// meets.
    #[test]
    fn an_append_lands_on_the_end_and_a_later_opening_carries_on_from_it() {
        let area = an_area();
        let name = area.join("journal");
        {
            let mut file = AFileToAppendTo::opened(&name).expect("it opens");
            file.append(b"one\n").expect("it appends");
            file.append(b"two\n").expect("it appends");
        }
        {
            let mut file = AFileToAppendTo::opened(&name).expect("it opens again");
            file.append(b"three\n").expect("it appends");
        }
        assert_eq!(
            read(&name).unwrap().as_deref(),
            Some(&b"one\ntwo\nthree\n"[..])
        );
        fs::remove_dir_all(&area).ok();
    }

    /// **The one repair: bytes with no ending behind them are cut off before
    /// anything is appended after them.**
    #[test]
    fn a_part_written_tail_is_cut_back_and_the_next_append_follows_the_last_whole_one() {
        let area = an_area();
        let name = area.join("journal");
        fs::write(&name, b"one\ntwo\nthr").expect("a killed append left a part-written tail");
        let mut file = AFileToAppendTo::opened(&name).expect("it opens");
        file.cut_back_to(8).expect("it cuts back");
        file.append(b"three\n").expect("it appends");
        assert_eq!(
            read(&name).unwrap().as_deref(),
            Some(&b"one\ntwo\nthree\n"[..])
        );
        fs::remove_dir_all(&area).ok();
    }

    /// **The length before an append is what an append that failed part-way is
    /// put back to**, and the two acts together really remove the stump.
    ///
    /// This is the desk's half of the round's journal finding: the reading and
    /// the trim are real here, and the journal's own module poses the refusal
    /// that reaches them.
    #[test]
    fn the_length_before_an_append_puts_a_part_written_one_back() {
        let area = an_area();
        let name = area.join("journal");
        let mut file = AFileToAppendTo::opened(&name).expect("it opens");
        file.append(b"one\ntwo\n").expect("two whole entries land");
        let whole_before = file.how_long_it_is().expect("the length is read");
        assert_eq!(whole_before, 8);

        // The bytes an append that stopped part-way would have left.
        file.append(b"thr").expect("the part lands");
        file.cut_back_to(whole_before).expect("the stump comes off");

        file.append(b"three\n").expect("the next whole entry lands");
        assert_eq!(
            read(&name).unwrap().as_deref(),
            Some(&b"one\ntwo\nthree\n"[..]),
            "no line nobody wrote"
        );
        fs::remove_dir_all(&area).ok();
    }

    /// **An append needs no new directory entry and a publication does**, which
    /// is the mechanism this plan's own Section 16 item 26 observed on a full
    /// mount. Here the directory is closed to new entries instead of being
    /// filled, which poses that half of it at a desk.
    #[test]
    fn an_append_goes_on_landing_in_a_directory_that_takes_no_new_entry() {
        use std::os::unix::fs::PermissionsExt;
        let area = an_area();
        let name = area.join("journal");
        let mut file = AFileToAppendTo::opened(&name).expect("it opens while the area still takes one");
        file.append(b"before\n").expect("it appends");

        let mut closed = fs::metadata(&area).unwrap().permissions();
        closed.set_mode(0o500);
        fs::set_permissions(&area, closed).unwrap();

        assert!(
            publish(&area.join("a record"), b"a record").is_err(),
            "a publication needs a new name and a new directory entry"
        );
        file.append(b"after\n")
            .expect("an append needs neither and goes on landing");
        assert_eq!(
            read(&name).unwrap().as_deref(),
            Some(&b"before\nafter\n"[..])
        );

        let mut open_again = fs::metadata(&area).unwrap().permissions();
        open_again.set_mode(0o700);
        fs::set_permissions(&area, open_again).ok();
        fs::remove_dir_all(&area).ok();
    }

    #[test]
    fn a_write_into_an_area_that_is_not_there_is_refused_with_the_machines_own_words() {
        let refused = publish(Path::new("/no/such/area/at/all/phase"), b"x")
            .expect_err("there is no such area");
        assert!(refused.at.contains("/no/such/area"));
        assert!(!refused.what_the_machine_said.is_empty());
    }
}
