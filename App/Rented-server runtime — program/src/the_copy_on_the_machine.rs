//! **`Server protocol and transfers` §1.1's request *a file is copied on the
//! machine*, answered by the SSH command adapter in its own invocation.**
//! Added 2026-10-02, step 10c's Files' owed acts piece 7.
//!
//! **What it is for.** `File Browser` M7 copies a file on the machine and never
//! across the wire — by the file service's copy-data where the connection
//! announced it, and otherwise by this request. An 8.9p1 server announces no
//! copy-data, and plain Ubuntu 22.04 images carry that line, so without this a
//! set-up server of that kind could not copy at all.
//!
//! **What it does, and only this.** It opens the source to read, creates the
//! staging path the desktop named **exclusively** with owner-only permissions
//! (the copy-data route's own create), copies the bytes, flushes them to disk,
//! and answers. **It never publishes**: the rename that makes the copy appear
//! under its name is the desktop's, on the file service, so the staging, the
//! collision answers and the writes of `File Browser` X7a and `File Browser`
//! W11 are the same on either route. **Where anything fails, what this
//! request made at the staging path is removed before it answers**, so a copy
//! that did not land leaves
//! nothing of its own behind. A path someone else had already taken is never
//! touched.
//!
//! **Why here and not in the supervisor.** The supervisor serves its callers
//! one after another (`smoozy_runtime::acting::endpoint`'s `answer_one_caller`),
//! so a copy of many gigabytes there would hold every other request — health,
//! a worker's report, the queue — until it ended. The paragraph after
//! `Rented-server runtime` W2: *"answered by the SSH command adapter in its own
//! invocation and never reaches the supervisor"*. **It reads and writes no store
//! the supervisor keeps**, which is what lets it stand outside it.
//!
//! **What it is not.** No worker, no operation record, no execution group, no
//! termination contract — that same paragraph. **A copy cut short by the
//! connection's ending** leaves a part-file at the staging path only where this
//! process itself was ended before it could remove it; that is for `File
//! Browser` M7 and `File Browser` X12 to clear, as it is on the copy-data route.

use std::io::Write;
use std::os::unix::ffi::OsStrExt;
use std::os::unix::fs::OpenOptionsExt;
use std::path::Path;

use smoozy_framing::framing::{
    Answered, Fields, Request, THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS,
};
use smoozy_framing::requests::{
    IT_WAS_COPIED, IT_WAS_NOT_COPIED, THE_COPYS_OUTCOME_FIELD, THE_COPYS_SOURCE_FIELD,
    THE_COPYS_STAGING_PATH_FIELD, THE_MACHINES_OWN_WORDS_FIELD,
};

/// **The permissions the staging file is created with** — the copy-data
/// route's own exclusive create asks the file service for the same, so a copy
/// lands the same way on either route.
const THE_STAGING_FILES_PERMISSIONS: u32 = 0o600;

/// **The request, answered.** The version is checked as every answer checks
/// it; a request missing either path is refused by name rather than answered
/// on a path made up here.
pub fn answered(request: &Request, package_version: &str) -> Answered {
    let empty = Answered::to(request, THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS, package_version);
    if !smoozy_framing::framing::DeclaredVersions::of_this_build().covers(request.protocol_version) {
        return empty.saying_it_does_not_speak_that_protocol_version(request.protocol_version);
    }
    let (Some(source), Some(staging)) = (
        request.fields.value(THE_COPYS_SOURCE_FIELD),
        request.fields.value(THE_COPYS_STAGING_PATH_FIELD),
    ) else {
        return empty.saying_it_does_not_answer_that_request(&request.name);
    };
    let source = Path::new(std::ffi::OsStr::from_bytes(source));
    let staging = Path::new(std::ffi::OsStr::from_bytes(staging));
    empty.carrying(match the_file_is_copied(source, staging) {
        Ok(()) => Fields::none().and(THE_COPYS_OUTCOME_FIELD, IT_WAS_COPIED),
        Err(words) => Fields::none()
            .and(THE_COPYS_OUTCOME_FIELD, IT_WAS_NOT_COPIED)
            .and(THE_MACHINES_OWN_WORDS_FIELD, words),
    })
}

/// **The copy itself**: the source read, the staging path created exclusively,
/// the bytes copied, flushed to disk. **What this made is removed on any
/// failure**, and the answer is the machine's own words.
fn the_file_is_copied(source: &Path, staging: &Path) -> Result<(), String> {
    let mut from = std::fs::File::open(source)
        .map_err(|why| format!("{}: could not open the file: {why}", source.display()))?;
    let mut to = std::fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .mode(THE_STAGING_FILES_PERMISSIONS)
        .open(staging)
        .map_err(|why| format!("{}: could not create the copy: {why}", staging.display()))?;
    let written = std::io::copy(&mut from, &mut to)
        .and_then(|_| to.flush())
        .and_then(|()| to.sync_all());
    drop(to);
    written.map_err(|why| {
        let _ = std::fs::remove_file(staging);
        format!("{}: the copy failed: {why}", staging.display())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use smoozy_state::identities::{Identity, IdentityKind};

    fn a_request(source: &Path, staging: &Path) -> Request {
        Request::speaking(
            THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS,
            Identity::issue(IdentityKind::Operation).expect("randomness"),
            smoozy_framing::requests::THE_FILE_IS_COPIED_ON_THE_MACHINE_REQUEST,
        )
        .carrying(
            Fields::none()
                .and(THE_COPYS_SOURCE_FIELD, source.as_os_str().as_bytes())
                .and(THE_COPYS_STAGING_PATH_FIELD, staging.as_os_str().as_bytes()),
        )
    }

    struct Scratch(std::path::PathBuf);

    impl Scratch {
        fn new(named: &str) -> Scratch {
            let at = std::env::temp_dir().join(format!(
                "smoozy-copy-{named}-{}",
                &Identity::issue(IdentityKind::Operation).expect("randomness").as_text()[..12]
            ));
            std::fs::create_dir_all(&at).expect("made");
            Scratch(at)
        }
    }

    impl Drop for Scratch {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.0);
        }
    }

    /// **A copy lands at the staging path, whole, owner-only, and nowhere
    /// else** — the source untouched, and the name it will be published under
    /// left to the desktop.
    #[test]
    fn a_file_is_copied_to_the_staging_path_and_nowhere_else() {
        use std::os::unix::fs::PermissionsExt;
        let scratch = Scratch::new("lands");
        let source = scratch.0.join("wood.png");
        let bytes: Vec<u8> = (0..200_000u32).map(|n| (n % 251) as u8).collect();
        std::fs::write(&source, &bytes).expect("written");
        let staging = scratch.0.join("staged");
        let answered = answered(&a_request(&source, &staging), "0.1.8");
        assert_eq!(answered.fields.text(THE_COPYS_OUTCOME_FIELD), Some(IT_WAS_COPIED), "{answered:?}");
        assert_eq!(std::fs::read(&staging).expect("the copy"), bytes);
        assert_eq!(
            std::fs::metadata(&staging).expect("the copy").permissions().mode() & 0o777,
            THE_STAGING_FILES_PERMISSIONS
        );
        assert_eq!(std::fs::read(&source).expect("the source"), bytes);
        assert_eq!(std::fs::read_dir(&scratch.0).expect("reads").count(), 2, "nothing else was made");
    }

    /// **A staging path already taken is written over by nothing**, and is
    /// not removed either — it is not this request's.
    #[test]
    fn a_staging_path_already_taken_is_left_as_it_was() {
        let scratch = Scratch::new("taken");
        let source = scratch.0.join("wood.png");
        std::fs::write(&source, b"the source").expect("written");
        let staging = scratch.0.join("staged");
        std::fs::write(&staging, b"someone else's").expect("written");
        let answered = answered(&a_request(&source, &staging), "0.1.8");
        assert_eq!(answered.fields.text(THE_COPYS_OUTCOME_FIELD), Some(IT_WAS_NOT_COPIED));
        assert!(answered.fields.text(THE_MACHINES_OWN_WORDS_FIELD).is_some());
        assert_eq!(std::fs::read(&staging).expect("still there"), b"someone else's");
    }

    /// **A source that cannot be read makes nothing**, and the machine's own
    /// words say why.
    #[test]
    fn a_source_that_is_not_there_makes_nothing() {
        let scratch = Scratch::new("gone");
        let staging = scratch.0.join("staged");
        let answered = answered(&a_request(&scratch.0.join("gone.png"), &staging), "0.1.8");
        assert_eq!(answered.fields.text(THE_COPYS_OUTCOME_FIELD), Some(IT_WAS_NOT_COPIED));
        assert!(!staging.exists());
    }

    /// **A request missing a path is refused by name**, never answered on a
    /// path made up here.
    #[test]
    fn a_request_missing_a_path_is_refused_by_name() {
        let request = Request::speaking(
            THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS,
            Identity::issue(IdentityKind::Operation).expect("randomness"),
            smoozy_framing::requests::THE_FILE_IS_COPIED_ON_THE_MACHINE_REQUEST,
        )
        .carrying(Fields::none().and(THE_COPYS_SOURCE_FIELD, "/somewhere"));
        assert!(answered(&request, "0.1.8").the_request_name_it_says_it_does_not_answer().is_some());
    }
}
