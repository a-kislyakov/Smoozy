//! **`Server protocol and transfers` §1.1's request *the four values of files
//! on the machine*, answered by the SSH command adapter in its own
//! invocation.** Built 2026-11-04, out of a real folder of 480 files
//! (`Dependency Upload & Remap` Q5 route 1).
//!
//! **What it does, and only this.** For each path it makes the filesystem call
//! that follows a link, as `stat -L` does, and answers size, inode, modification
//! time and change time **in the units that command reads**: bytes, the inode
//! number and whole seconds, so a placement record written from either route
//! compares against a reading by the other. **Each path answers for itself**,
//! so one bad path never fails the rest. **It writes nothing**, so it is a pure
//! reading.
//!
//! **Why here and not in the supervisor** is the copy on the machine's reason
//! (`crate::the_copy_on_the_machine`): it reads no store the supervisor keeps,
//! and the supervisor serves its callers one after another.

use std::ffi::OsStr;
use std::os::unix::ffi::OsStrExt;
use std::os::unix::fs::MetadataExt;
use std::path::Path;

use smoozy_framing::framing::{Answered, Fields, Request, THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS};
use smoozy_framing::requests::{
    the_paths_read_back, the_values_written, OnePathAnswered, THE_PATHS_READ_FIELD,
    THE_VALUES_READ_FIELD,
};

/// **The request, answered.** A request with no paths is refused by name rather
/// than answered on nothing.
pub fn answered(request: &Request, package_version: &str) -> Answered {
    let empty = Answered::to(request, THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS, package_version);
    if !smoozy_framing::framing::DeclaredVersions::of_this_build().covers(request.protocol_version) {
        return empty.saying_it_does_not_speak_that_protocol_version(request.protocol_version);
    }
    let Some(paths) = request.fields.value(THE_PATHS_READ_FIELD) else {
        return empty.saying_it_does_not_answer_that_request(&request.name);
    };
    let answers: Vec<OnePathAnswered> = the_paths_read_back(paths)
        .into_iter()
        .map(|path| the_four_values_at(Path::new(OsStr::from_bytes(path))))
        .collect();
    empty.carrying(Fields::none().and(THE_VALUES_READ_FIELD, the_values_written(&answers)))
}

/// **One path.** `std::fs::metadata` follows a link. **A dangling link is the
/// system's own *no such file***, which is what `stat -L` meets there too.
fn the_four_values_at(path: &Path) -> OnePathAnswered {
    match std::fs::metadata(path) {
        Ok(seen) => OnePathAnswered::Values {
            size: seen.len(),
            inode: seen.ino(),
            modification_time: seen.mtime(),
            change_time: seen.ctime(),
        },
        Err(why) if why.kind() == std::io::ErrorKind::NotFound => OnePathAnswered::NothingThere,
        Err(why) => OnePathAnswered::NotRead(why.to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use smoozy_framing::requests::{the_paths_written, the_values_read_back, THE_FOUR_VALUES_OF_FILES_REQUEST};
    use smoozy_state::identities::{Identity, IdentityKind};
    use std::path::PathBuf;
    use std::process::Command;

    struct Scratch(PathBuf);

    impl Scratch {
        fn new(named: &str) -> Scratch {
            let at = std::env::temp_dir().join(format!(
                "smoozy-values-{named}-{}",
                &Identity::issue(IdentityKind::Operation).expect("randomness").as_text()[..12]
            ));
            std::fs::create_dir_all(&at).expect("made");
            Scratch(at)
        }
    }

    impl Drop for Scratch {
        fn drop(&mut self) {
            use std::os::unix::fs::PermissionsExt;
            let _ = std::fs::set_permissions(self.0.join("shut"), std::fs::Permissions::from_mode(0o755));
            let _ = std::fs::remove_dir_all(&self.0);
        }
    }

    fn a_request(paths: &[Vec<u8>]) -> Request {
        Request::speaking(
            THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS,
            Identity::issue(IdentityKind::Operation).expect("randomness"),
            THE_FOUR_VALUES_OF_FILES_REQUEST,
        )
        .carrying(Fields::none().and(THE_PATHS_READ_FIELD, the_paths_written(paths)))
    }

    /// **The one-at-a-time route, performed as it is performed**: a link planted
    /// at a fixed name, pointing at the path, and `stat -L` naming the link.
    /// **A rental's `stat` is GNU's, the command itself, and the hosted Linux job
    /// runs it.** **This Mac's is BSD's**, which takes another format and, measured
    /// here, answers a dangling link's `-L` with the link's own values at exit 0
    /// where GNU's fails; so on this Mac `test -e`, which follows the link, stands
    /// in front of it to make the same refusal.
    fn the_command_route_at(link: &Path, path: &Path) -> Option<(u64, u64, i64, i64)> {
        let _ = std::fs::remove_file(link);
        std::os::unix::fs::symlink(path, link).expect("the link is planted");
        let the_command = if cfg!(target_os = "linux") {
            "stat -L -c '%s %i %Y %Z' \"$1\""
        } else {
            "test -e \"$1\" && stat -L -f '%z %i %m %c' \"$1\""
        };
        let output = Command::new("sh").args(["-c", the_command, "sh"]).arg(link).output().expect("the command runs");
        let _ = std::fs::remove_file(link);
        if !output.status.success() {
            return None;
        }
        let text = String::from_utf8(output.stdout).expect("numbers");
        let mut fields = text.split_whitespace();
        Some((
            fields.next()?.parse().ok()?,
            fields.next()?.parse().ok()?,
            fields.next()?.parse().ok()?,
            fields.next()?.parse().ok()?,
        ))
    }

    /// **The brief's *a file a single answer can't name*: every case the
    /// one-at-a-time route covers reads the same**, against the real `stat`, on
    /// a real disk, in one ask, with the bad paths among the good so one cannot
    /// fail the rest. What the desktop keeps of *nothing there* and *not read*
    /// is the command route's own *cannot answer*, which is `None` on both sides.
    #[test]
    fn every_case_reads_as_the_one_at_a_time_route_reads_it() {
        use std::os::unix::fs::PermissionsExt;
        let scratch = Scratch::new("cases");
        let at = |name: &str| scratch.0.join(name);
        std::fs::write(at("plain.bin"), vec![7u8; 4096]).expect("written");
        std::fs::write(at("empty"), b"").expect("written");
        std::fs::write(at("a name with spaces é.png"), b"abc").expect("written");
        std::fs::write(at("line\nbreak"), b"abcd").expect("written");
        std::fs::write(at("-leading-dash"), b"abcde").expect("written");
        std::fs::create_dir_all(at("a folder")).expect("made");
        std::os::unix::fs::symlink(at("plain.bin"), at("link to plain")).expect("linked");
        std::os::unix::fs::symlink(at("gone"), at("dangling")).expect("linked");
        std::os::unix::fs::symlink(at("loop-b"), at("loop-a")).expect("linked");
        std::os::unix::fs::symlink(at("loop-a"), at("loop-b")).expect("linked");
        std::fs::create_dir_all(at("shut")).expect("made");
        std::fs::write(at("shut/inside.bin"), b"hidden").expect("written");
        std::fs::set_permissions(at("shut"), std::fs::Permissions::from_mode(0o000)).expect("shut");
        let cases = [
            "plain.bin", "never-there", "empty", "dangling", "a name with spaces é.png", "loop-a",
            "line\nbreak", "shut/inside.bin", "-leading-dash", "plain.bin/under-a-file", "a folder",
            "link to plain", "plain.bin",
        ];
        let paths: Vec<Vec<u8>> = cases.iter().map(|one| at(one).as_os_str().as_bytes().to_vec()).collect();

        let answered = answered(&a_request(&paths), "0.1.9");
        let answers = the_values_read_back(
            answered.fields.text(THE_VALUES_READ_FIELD).expect("the values"),
            paths.len(),
        )
        .expect("one answer for each path");

        let link_folder = Scratch::new("carriage");
        let link = link_folder.0.join("settings-page-blend");
        for (case, answer) in cases.iter().zip(&answers) {
            let by_the_command = the_command_route_at(&link, &at(case));
            let by_the_request = match answer {
                OnePathAnswered::Values { size, inode, modification_time, change_time } => {
                    Some((*size, *inode, *modification_time, *change_time))
                }
                OnePathAnswered::NothingThere | OnePathAnswered::NotRead(_) => None,
            };
            assert_eq!(by_the_request, by_the_command, "{case}");
        }
        // **And the three kinds of answer are the ones the plan names.**
        let of = |name: &str| &answers[cases.iter().position(|one| *one == name).expect("a case")];
        assert!(matches!(of("plain.bin"), OnePathAnswered::Values { size: 4096, .. }));
        assert_eq!(of("never-there"), &OnePathAnswered::NothingThere);
        assert_eq!(of("dangling"), &OnePathAnswered::NothingThere);
        assert!(matches!(of("loop-a"), OnePathAnswered::NotRead(_)));
        assert!(matches!(of("plain.bin/under-a-file"), OnePathAnswered::NotRead(_)));
        if std::fs::read_dir(at("shut")).is_err() {
            assert!(matches!(of("shut/inside.bin"), OnePathAnswered::NotRead(_)));
        }
    }

    /// **A request with no paths field is refused by name**, and one that names
    /// no path answers nothing, rather than a made-up reading.
    #[test]
    fn a_request_with_no_paths_is_refused_and_an_empty_one_answers_nothing() {
        let bare = Request::speaking(
            THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS,
            Identity::issue(IdentityKind::Operation).expect("randomness"),
            THE_FOUR_VALUES_OF_FILES_REQUEST,
        );
        assert!(answered(&bare, "0.1.9").the_request_name_it_says_it_does_not_answer().is_some());
        let none = answered(&a_request(&[]), "0.1.9");
        assert_eq!(none.fields.text(THE_VALUES_READ_FIELD), Some(""));
    }

    /// **What one command costs at the film's own count, measured here:** 522
    /// paths answered, which the walk took about 12 minutes over one at a time.
    #[test]
    fn five_hundred_and_twenty_two_paths_are_answered_at_once() {
        let scratch = Scratch::new("many");
        let paths: Vec<Vec<u8>> = (0..522)
            .map(|at| {
                let path = scratch.0.join(format!("critter_evolved-movement_bubbles_{at:05}.bgeo"));
                std::fs::write(&path, b"x").expect("written");
                path.as_os_str().as_bytes().to_vec()
            })
            .collect();
        let started = std::time::Instant::now();
        let answered = answered(&a_request(&paths), "0.1.9");
        let took = started.elapsed();
        let answers = the_values_read_back(answered.fields.text(THE_VALUES_READ_FIELD).expect("the values"), 522)
            .expect("522 answers");
        assert!(answers.iter().all(|one| matches!(one, OnePathAnswered::Values { size: 1, .. })));
        println!("522 paths answered in {took:?}");
        assert!(took < std::time::Duration::from_secs(5), "{took:?}");
    }
}
