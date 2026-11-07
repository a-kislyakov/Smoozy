//! **The frame preview, and it is three answers** — `Desktop app and
//! delivery` §13.4, T15, T16, T17 and T17a.
//!
//! **What is attached is the fetched derivative already on the user's
//! computer, and nothing is made here** (T15). `Job files and outputs` settles
//! that every viewable derivative is produced once **on the rented machine**
//! and that a fetched derivative is a viewing-format picture; its B12 fixes
//! that a viewing format is **PNG or JPEG**; and `State, identity and
//! recovery` §11.3 makes the fetched copy subject-held on its job record, so
//! it is a file on disk for as long as that record exists.
//!
//! **So there is no encoder, no resizer and no thumbnailer in this file, and
//! there is not meant to be one.** A builder who reaches for one has moved
//! where derivatives are made, which T15 closed.
//!
//! **The three systems take it three ways and one of them is unresolved.**
//! That is T16, and §22 item 13 is where what breaks is written. Nothing here
//! upgrades the Windows contradiction into an answer and nothing pretends the
//! Linux route is observed.
//!
//! **Markers used in this file that belong to another document, each with the
//! document it belongs to.** `Job files and outputs` B12. `State, identity
//! and recovery` §11.3.

use std::path::{Path, PathBuf};

/// **macOS's attachment ceiling** — T16: *"JPEG, GIF or PNG only, 10 MB
/// maximum, with no in-memory supply"*.
///
/// **Nothing establishes that a fetched derivative is by construction inside
/// it** (§22 item 25), which is why the size is checked at the moment of
/// posting rather than assumed at the moment the derivative is made.
pub const THE_MAC_ATTACHMENT_CEILING_IN_BYTES: u64 = 10 * 1024 * 1024;

/// **How each of the three takes a picture** — T16.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HowThisSystemTakesIt {
    /// **macOS: an attachment, and it must be a file on disk.** No in-memory
    /// supply exists, which is why T15's *already on disk* is what makes the
    /// route work at all.
    AnAttachmentFromDisk,
    /// **Linux: the file's address in the `image-path` hint** — the
    /// specification: *"should be either an URI (file:// is the only URI schema
    /// supported right now)"*. *(T16's Linux third, corrected 2026-11-01: it
    /// said pixels in memory were the only documented route, so no picture
    /// went.)* **There is still no capability string for it**, so a server may
    /// ignore it silently and nothing detects that it did.
    AFileAddressInAHint,
    /// **Windows: two Microsoft pages disagree and neither acknowledges the
    /// other.** One lists three image sources of which two need a package
    /// identity an unpackaged app lacks; another documents a local file path
    /// *"Supported only for desktop apps"*. **The contradiction is inherited
    /// unresolved and the local-file route is what is attempted.**
    ALocalFilePath,
    /// A system this document's three-system boundary was never written for.
    NotAtAll,
}

/// Which route a named system takes — T16.
pub fn how_this_system_takes_it(system: &str) -> HowThisSystemTakesIt {
    match system {
        "macos" => HowThisSystemTakesIt::AnAttachmentFromDisk,
        "linux" => HowThisSystemTakesIt::AFileAddressInAHint,
        "windows" => HowThisSystemTakesIt::ALocalFilePath,
        _ => HowThisSystemTakesIt::NotAtAll,
    }
}

/// **The fetched derivative as the job record holds it** — a file on disk and
/// its length.
///
/// **No format field.** `Job files and outputs` B12 fixes a viewing format as
/// PNG or JPEG and the derivative was produced on the machine, so T16's
/// *"the format holds by construction"* is exactly that — a check here would
/// be Smoozy checking its own record against itself, which Rule 8 has nothing
/// to say for.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TheFetchedDerivative {
    pub path: PathBuf,
    pub length_in_bytes: u64,
}

/// **What actually goes on the post.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ThePicture {
    /// This file, by the route this system takes.
    ThisFileOnDisk(PathBuf),
    /// **None, and the notification is posted anyway** — T16 and T17. Every
    /// way of arriving here is one of T17a's five, and **none of them is a
    /// fault.**
    None(WhyThereIsNoPicture),
}

/// **The reasons this application composes no picture, and none of them is a
/// fault.**
///
/// **T17a's ways the picture is absent are more than this enumeration holds**,
/// and the difference is not drift: the Linux server dropping a picture that
/// was sent, and the Windows contradiction, happen after the post has gone, so
/// nothing here can know of them; and macOS's held copy that cannot be made
/// read-only is decided in `macos.rs` at the attaching. *(The Linux route not
/// being built was a reason here from 2026-09-12 to 2026-11-01, when the route
/// was built — T16's Linux third.)*
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhyThereIsNoPicture {
    /// **T17: the derivative is not yet on disk at the moment of posting.**
    /// The notification is posted **without a picture** rather than delayed,
    /// suppressed, or re-posted later. The spec's own wording is *"where the
    /// job has one"*.
    TheDerivativeIsNotOnDiskYet,
    /// **macOS: it does not fit** the 10 MB attachment ceiling (T16, §22
    /// item 25).
    ItIsOverTheMacsAttachmentCeiling,
    /// This system takes no picture at all.
    ThisSystemTakesNone,
}

/// **T16 and T17, performed.**
///
/// `derivative` is what the job record holds at the moment of posting, and
/// **`None` means it is not on disk yet** — which is T17's case and produces a
/// picture-less post rather than a wait.
pub fn the_picture_for(
    system: &str,
    derivative: Option<&TheFetchedDerivative>,
) -> ThePicture {
    let Some(derivative) = derivative else {
        // **T17: a finished-job notification never waits on a fetch.**
        return ThePicture::None(WhyThereIsNoPicture::TheDerivativeIsNotOnDiskYet);
    };
    match how_this_system_takes_it(system) {
        HowThisSystemTakesIt::AnAttachmentFromDisk => {
            if derivative.length_in_bytes > THE_MAC_ATTACHMENT_CEILING_IN_BYTES {
                ThePicture::None(WhyThereIsNoPicture::ItIsOverTheMacsAttachmentCeiling)
            } else {
                ThePicture::ThisFileOnDisk(derivative.path.clone())
            }
        }
        // **The local-file route is what would be attempted** (T16), and the
        // notification still says the job finished where it does not work.
        HowThisSystemTakesIt::ALocalFilePath => {
            ThePicture::ThisFileOnDisk(derivative.path.clone())
        }
        // **T16's Linux third: the file's address, in the `image-path`
        // hint**, whatever the size — the specification states no limit.
        HowThisSystemTakesIt::AFileAddressInAHint => {
            ThePicture::ThisFileOnDisk(derivative.path.clone())
        }
        HowThisSystemTakesIt::NotAtAll => {
            ThePicture::None(WhyThereIsNoPicture::ThisSystemTakesNone)
        }
    }
}

/// The path where there is one, for a caller that only wants the file.
pub fn as_path(picture: &ThePicture) -> Option<&Path> {
    match picture {
        ThePicture::ThisFileOnDisk(path) => Some(path.as_path()),
        ThePicture::None(_) => None,
    }
}

/// **The `file://` address of a file, for the Linux `image-path` hint** — T16's
/// Linux third. The specification: *"file:// is the only URI schema supported
/// right now"*. **Every byte outside the plain set is written as `%` and two
/// hex digits**, the address form the URI standard gives, so a space or a `#`
/// in a folder name stays part of the path rather than ending it.
pub fn a_file_address(path: &Path) -> String {
    #[cfg(unix)]
    let bytes: Vec<u8> = {
        use std::os::unix::ffi::OsStrExt;
        path.as_os_str().as_bytes().to_vec()
    };
    #[cfg(not(unix))]
    let bytes: Vec<u8> = path.to_string_lossy().into_owned().into_bytes();
    let mut address = String::from("file://");
    for byte in bytes {
        match byte {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'.' | b'_' | b'~' | b'/' => {
                address.push(byte as char)
            }
            other => address.push_str(&format!("%{other:02X}")),
        }
    }
    address
}

/// **macOS takes an attachment by moving it, unless it cannot write to it** —
/// T16a, added 2026-11-01 from the Mac watch of that day.
///
/// Apple: *"Once validated, the system moves the attached files into the
/// attachment data store so that the appropriate processes can access the
/// files. The system copies attachments located inside an app's bundle."*
/// **What the Mac's own log showed** is the system copying the frame into its
/// store, the application's side deleting the original (*"File is writable.
/// Deleting file"*), and Notification Center then failing to open the original
/// path to draw the picture. **So the job record's held copy was deleted by
/// the post, and the picture never showed.**
///
/// **The held copy is made read-only before it is attached**, which is the
/// state the system leaves alone; nothing is made and nothing is moved, so T15
/// stands. Returns whether the file is now read-only; **where it is not, the
/// post goes without the picture** rather than hand the system the record's
/// only copy. Removing a read-only file is still the folder's to allow, so the
/// age sweep and a deletion remove it as before.
pub fn the_file_is_left_for_the_system_to_copy(path: &Path) -> bool {
    let Ok(read) = std::fs::metadata(path) else {
        return false;
    };
    let mut permissions = read.permissions();
    if !permissions.readonly() {
        permissions.set_readonly(true);
        if std::fs::set_permissions(path, permissions).is_err() {
            return false;
        }
    }
    std::fs::metadata(path)
        .map(|read| read.permissions().readonly())
        .unwrap_or(false)
}

/// **T17a: what the per-kind switch may claim is bounded — the kind, never the
/// picture and never delivery.**
///
/// **The picture is a common case rather than a property**, and it is absent
/// five ways that are none of them faults — four that are facts about a system
/// and one that is a fact about Smoozy, a macOS held copy that cannot be made
/// read-only. *(Until 2026-11-01 the fact about Smoozy was the Linux route not
/// being built; it is built.)*
/// *"So a label may say what the kind is and not what it carries."* **A flat claim is wrong in ordinary use
/// rather than in an edge case**, which is what makes this a bound rather than
/// a caution.
///
/// **The same bound reaches the switch's other implied claim.** A switch
/// governs whether Smoozy **attempts** the post and never whether one arrives:
/// delivery is never confirmed (T23), a Windows post fails silently (T4), a
/// computer with no notification service gets none of the four (T5), and a job
/// ending inside any reconnection window posts nothing at all (§22 item 26).
///
/// **Nothing in this crate draws that switch yet** — Settings is a screen no
/// step has built — so this constant is what the agent who draws it meets, and
/// the wording of the switch itself is Design's, bounded by this clause and by
/// nothing else (§24).
pub const A_SWITCH_MAY_SAY_WHAT_THE_KIND_IS_AND_NOT_WHAT_IT_CARRIES: bool = true;

#[cfg(test)]
mod tests {
    use super::*;

    fn a_derivative(length_in_bytes: u64) -> TheFetchedDerivative {
        TheFetchedDerivative {
            path: PathBuf::from("/tmp/smoozy-job/preview.png"),
            length_in_bytes,
        }
    }

    /// **T16: three systems, three routes.**
    #[test]
    fn the_three_systems_take_the_picture_three_ways() {
        assert_eq!(
            how_this_system_takes_it("macos"),
            HowThisSystemTakesIt::AnAttachmentFromDisk
        );
        assert_eq!(
            how_this_system_takes_it("linux"),
            HowThisSystemTakesIt::AFileAddressInAHint
        );
        assert_eq!(
            how_this_system_takes_it("windows"),
            HowThisSystemTakesIt::ALocalFilePath
        );
    }

    /// **T17: a finished-job notification never waits on a fetch.**
    ///
    /// This is the line that fails if somebody makes the post wait, retry or
    /// re-post — all three of which T17 refuses by name, and the first of
    /// which is the natural thing to write.
    #[test]
    fn a_derivative_not_yet_on_disk_posts_without_a_picture_on_every_system() {
        for system in ["macos", "windows", "linux"] {
            assert_eq!(
                the_picture_for(system, None),
                ThePicture::None(WhyThereIsNoPicture::TheDerivativeIsNotOnDiskYet),
                "{system}"
            );
        }
    }

    /// **T16 and §22 item 25: the ceiling is checked at the moment of posting,
    /// and the notification goes without the picture where it does not fit.**
    #[test]
    fn the_mac_attaches_under_the_ceiling_and_posts_without_a_picture_over_it() {
        assert_eq!(
            the_picture_for("macos", Some(&a_derivative(THE_MAC_ATTACHMENT_CEILING_IN_BYTES))),
            ThePicture::ThisFileOnDisk(PathBuf::from("/tmp/smoozy-job/preview.png"))
        );
        assert_eq!(
            the_picture_for(
                "macos",
                Some(&a_derivative(THE_MAC_ATTACHMENT_CEILING_IN_BYTES + 1))
            ),
            ThePicture::None(WhyThereIsNoPicture::ItIsOverTheMacsAttachmentCeiling)
        );
    }

    /// **Windows attempts the local-file route whatever the size**, the
    /// contradiction being about which source works rather than about a
    /// ceiling, and no ceiling of Microsoft's has been read.
    #[test]
    fn windows_attempts_the_local_file_route_and_claims_nothing_about_it() {
        assert_eq!(
            the_picture_for("windows", Some(&a_derivative(50 * 1024 * 1024))),
            ThePicture::ThisFileOnDisk(PathBuf::from("/tmp/smoozy-job/preview.png"))
        );
    }

    /// **Linux sends the held copy by its file address** — the
    /// specification's `image-path` hint, read 2026-11-01. T16's Linux third
    /// said pixels in memory were the only documented route, so no picture
    /// went; the specification documents a `file://` address beside them.
    #[test]
    fn linux_sends_the_held_copy_by_its_file_address() {
        assert_eq!(
            how_this_system_takes_it("linux"),
            HowThisSystemTakesIt::AFileAddressInAHint
        );
        assert_eq!(
            the_picture_for("linux", Some(&a_derivative(50 * 1024 * 1024))),
            ThePicture::ThisFileOnDisk(PathBuf::from("/tmp/smoozy-job/preview.png")),
            "the Linux picture is still withheld, though the specification documents \
             `image-path` with a file:// address"
        );
    }

    /// **The address is a `file://` one with every byte outside the plain set
    /// escaped**, so a space or a `#` in a folder name stays part of the path.
    #[test]
    fn a_file_address_keeps_the_whole_path() {
        assert_eq!(
            a_file_address(Path::new("/home/a b/.local/share/Smoozy/previews/7f46.png")),
            "file:///home/a%20b/.local/share/Smoozy/previews/7f46.png"
        );
        assert_eq!(a_file_address(Path::new("/x/#1/ü.png")), "file:///x/%231/%C3%BC.png");
    }

    /// **T15: nothing is made here.**
    ///
    /// Read over this module's own source, because the fault it guards against
    /// is a helpful addition rather than a wrong line: a resize, a re-encode
    /// or a thumbnail written beside the derivative would move where
    /// derivatives are made, which `Job files and outputs` settled and T15
    /// closed.
    #[test]
    fn no_picture_is_made_in_this_file() {
        let source = include_str!("picture.rs");
        let (whole, _tests) = source
            .split_once("#[cfg(test)]")
            .expect("this module has tests");
        // **Comment lines are dropped before the search**, because the clause
        // is about what the code does and the prose above it has to be free to
        // name the very thing it forbids.
        let body: String = whole
            .lines()
            .filter(|line| {
                let line = line.trim_start();
                !line.starts_with("//") && !line.starts_with("///") && !line.starts_with("//!")
            })
            .collect::<Vec<_>>()
            .join("\n");
        for forbidden in [
            "File::create",
            "fs::write",
            "fs::copy",
            "encode",
            "resize",
            "thumbnail",
        ] {
            assert!(
                !body.contains(forbidden),
                "T15: the picture is the fetched derivative already on disk and nothing is made \
                 here, and `{forbidden}` has appeared in the module that chooses it"
            );
        }
    }

    /// **The Mac watch of 2026-11-01, finding 2: the held copy is not the
    /// system's to move.** Attached as it was, macOS deleted it after copying
    /// it, and the picture never showed. Made read-only first, it stays where
    /// the record holds it — and the age sweep can still remove it.
    #[test]
    fn the_held_copy_is_left_read_only_and_can_still_be_removed() {
        let folder = std::env::temp_dir().join(format!("smoozy-picture-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&folder);
        std::fs::create_dir_all(&folder).expect("a scratch folder");
        let held = folder.join("7f46df05fbf8b8d563c3833e9b11baca.png");
        std::fs::write(&held, b"png").expect("a held copy");

        assert!(the_file_is_left_for_the_system_to_copy(&held));
        let read = std::fs::metadata(&held).expect("it is still there");
        assert!(
            read.permissions().readonly(),
            "T16a: a writable attachment is moved by macOS, and the record's copy goes with it"
        );
        assert_eq!(std::fs::read(&held).expect("it still reads"), b"png");
        // Asked twice, it is still read-only and still says so.
        assert!(the_file_is_left_for_the_system_to_copy(&held));

        std::fs::remove_file(&held).expect("the age sweep can still remove it");
        assert!(!the_file_is_left_for_the_system_to_copy(&held), "a file that is gone is not attached");
        let _ = std::fs::remove_dir_all(&folder);
    }

    /// **T17a's bound, stated where the agent who draws the switch will be.**
    #[test]
    fn what_a_switch_may_claim_is_bounded() {
        assert!(A_SWITCH_MAY_SAY_WHAT_THE_KIND_IS_AND_NOT_WHAT_IT_CARRIES);
        // The three reasons this application composes, none of them a fault.
        // The type's own head says why T17a's others are not composed here.
        let ways = [
            WhyThereIsNoPicture::TheDerivativeIsNotOnDiskYet,
            WhyThereIsNoPicture::ItIsOverTheMacsAttachmentCeiling,
            WhyThereIsNoPicture::ThisSystemTakesNone,
        ];
        assert_eq!(ways.len(), 3);
        assert!(as_path(&ThePicture::None(ways[0])).is_none());
    }
}
