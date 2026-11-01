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
    /// **Linux: pixels supplied in memory**, which is the only route the
    /// specification documents, with no stated size limit — **but there is no
    /// capability string for it**, so a client cannot ask in advance whether
    /// the server will render it and hints may be ignored silently.
    PixelsSuppliedInMemory,
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
        "linux" => HowThisSystemTakesIt::PixelsSuppliedInMemory,
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
/// **T17a lists five ways the picture is absent and this enumeration holds
/// four**, and the difference is not drift: two of T17a's five — the Linux
/// server dropping a picture that was sent, and the Windows contradiction — are
/// things that happen after the post has gone, which nothing here can know
/// about and nothing here composes a reason for. **The fifth of T17a's, the
/// Linux route not being built, is [`WhyThereIsNoPicture::TheLinuxRouteIsNotBuilt`]
/// below**, and it was added to that clause on 2026-09-12 (U-2) because the plan
/// carried four where the application had five.
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
    /// **Linux: the pixels-in-memory route is not built**, and the reason is
    /// written at [`THE_LINUX_ROUTE_IS_OWED`].
    TheLinuxRouteIsNotBuilt,
    /// This system takes no picture at all.
    ThisSystemTakesNone,
}

/// **The Linux picture is owed, and this is where the cost of closing it is
/// written down.**
///
/// **T16's Linux route is pixels supplied in memory**, which is the only route
/// the freedesktop specification documents. Supplying them means decoding the
/// fetched derivative, and `Job files and outputs` B12 admits **PNG or
/// JPEG** — so closing it wants a PNG decoder **and** a JPEG decoder declared
/// for the Linux artifact. Neither is in the Linux tree beneath anything this
/// crate already takes.
///
/// **What it would buy is not establishable.** §22 item 13: there is **no
/// capability string** for the picture on Linux, so a client cannot ask in
/// advance whether the server will render it, hints may be ignored silently,
/// **and nothing detects that the picture did not appear.** So the weight
/// would be spent on an artifact nobody on this project can observe, on the
/// one of the three systems where the specification itself says the client
/// cannot find out.
///
/// **What holds without it** is item 13's own answer: the notification still
/// says the job finished, which is the kind's own content; the derivative is
/// on the job record and the picture is reachable in the app; and **T17a
/// already bounds what the finished-job switch may claim**, so no promise
/// anywhere is broken by its absence.
pub const THE_LINUX_ROUTE_IS_OWED: &str =
    "a PNG and a JPEG decoder declared for the Linux artifact, feeding the specification's \
     image-data hint";

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
        HowThisSystemTakesIt::PixelsSuppliedInMemory => {
            ThePicture::None(WhyThereIsNoPicture::TheLinuxRouteIsNotBuilt)
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

/// **T17a: what the per-kind switch may claim is bounded — the kind, never the
/// picture and never delivery.**
///
/// **The picture is a common case rather than a property**, and it is absent
/// five ways that are none of them faults — four that are facts about a system
/// and one that is a fact about Smoozy, the Linux route not being built.
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
            HowThisSystemTakesIt::PixelsSuppliedInMemory
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

    /// **The Linux route is owed and says so**, rather than quietly attaching
    /// a path the specification does not document taking.
    #[test]
    fn linux_posts_without_a_picture_and_names_what_would_close_it() {
        assert_eq!(
            the_picture_for("linux", Some(&a_derivative(1024))),
            ThePicture::None(WhyThereIsNoPicture::TheLinuxRouteIsNotBuilt)
        );
        assert!(THE_LINUX_ROUTE_IS_OWED.contains("decoder"));
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

    /// **T17a's bound, stated where the agent who draws the switch will be.**
    #[test]
    fn what_a_switch_may_claim_is_bounded() {
        assert!(A_SWITCH_MAY_SAY_WHAT_THE_KIND_IS_AND_NOT_WHAT_IT_CARRIES);
        // The four reasons this application composes, none of them a fault.
        // **T17a's list has five**, and the type's own head says why two of
        // them cannot be reasons anything here composes.
        let ways = [
            WhyThereIsNoPicture::TheDerivativeIsNotOnDiskYet,
            WhyThereIsNoPicture::ItIsOverTheMacsAttachmentCeiling,
            WhyThereIsNoPicture::TheLinuxRouteIsNotBuilt,
            WhyThereIsNoPicture::ThisSystemTakesNone,
        ];
        assert_eq!(ways.len(), 4);
        assert!(as_path(&ThePicture::None(ways[0])).is_none());
    }
}
