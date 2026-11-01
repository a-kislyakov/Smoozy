//! **§8.5 M13a performed — the unowned-staging reading and its one removal.**
//!
//! After `Server protocol and transfers` S28's record-driven sweep at
//! application start, `Download` asks the platform file adapter to enumerate
//! the reserved Smoozy staging directory inside each destination folder still
//! named by an auto-download arrangement or a manual request record. An entry
//! for which no transfer record supplies the exact staging path is presented
//! as **unowned download staging**, naming the destination path and offering
//! **Remove**.
//!
//! # Enumeration is never ownership evidence and removes nothing
//!
//! That is M13a's own sentence and `Server protocol and transfers` S28's
//! standing rule — *"a path found by enumerating is never ownership
//! evidence"* — and this module keeps it by shape: [`what_is_standing`]
//! returns a reading and has no removal in it, and
//! [`remove_one_named_entry`] takes one entry the user named and nothing
//! else. **There is no sweep here, no age, no cap and no automatic
//! deletion.**
//!
//! # The removal's three bounds, and each is a separate refusal
//!
//! **The user's explicit act on that named entry.** The adapter re-reads the
//! exact path. **It verifies that the entry itself is still inside the
//! reserved staging directory**, which is
//! [`crate::request::the_removal_may_run`] and is asked here before the
//! adapter is called at all. **And it removes that entry without following it
//! outside the directory** — so a link is removed as the entry it is and never
//! descended.
//!
//! **A refusal is reported as an ordinary failed removal.** The act creates no
//! transfer record, writes no transfer outcome, and infers no publication or
//! failure from the orphan's contents.
//!
//! # The reach is stated rather than inflated
//!
//! **Where no surviving arrangement or request record names the destination
//! folder, Smoozy has no bounded place to look, presents nothing, and does not
//! claim the orphan is absent.** It remains an ordinary visible entry in the
//! user's own folder until the user removes it with their file manager.
//! [`crate::request::the_folders_that_may_be_looked_in`] is that bound and
//! this module asks nothing outside what it returns.
//!
//! **Markers used in this file that belong to another document, each with the
//! document it belongs to.** `Server protocol and transfers` S4, S9, S28.
//! `Desktop app and delivery` F10, F10a.

use smoozy_server_protocol::transfers::TransferRecord;

use crate::destination::WhatTheAdapterSaid;
use crate::request::{
    the_folders_that_may_be_looked_in, the_removal_may_run, what_a_local_staging_entry_is,
    WhatALocalStagingEntryIs,
};

/// **`Desktop app and delivery` F10 and F10a — the one route for the local
/// enumeration and the explicit deletion.**
///
/// **The far side is that document's platform file adapter**, which cannot be
/// reached from a feature module, and **no implementation of this trait exists
/// outside this crate's own tests** ([`crate::arriving_later`]).
pub trait TheReservedStagingDirectory {
    /// **F10: what is standing inside one reserved staging directory**, as
    /// whole paths. A directory that is not there answers nothing, which is
    /// the ordinary case rather than a failure — most destinations have no
    /// staging directory at all between sittings.
    fn what_is_inside(&mut self, the_reserved_staging_directory: &[u8]) -> Vec<Vec<u8>>;

    /// **F10a: one named entry removed, without following it outside the
    /// directory.**
    fn remove_this_entry(&mut self, the_entry: &[u8]) -> Result<(), WhatTheAdapterSaid>;
}

/// One thing standing in a reserved staging directory that no transfer record
/// names.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnownedDownloadStaging {
    /// The entry's own whole path, which is what **Remove** acts on.
    pub entry: Vec<u8>,
    /// **The destination path, named** — which is what M13a says the existing
    /// Transfers surface presents.
    pub destination: Vec<u8>,
}

/// **M13a's reading, performed over the bounded set of folders and no
/// others.**
///
/// **It removes nothing and it is not ownership evidence.** What it produces
/// is one entry per thing standing that no transfer record supplies the exact
/// staging path for — the comparison being
/// [`what_a_local_staging_entry_is`]'s, against the records rather than
/// against the file.
pub fn what_is_standing<D: TheReservedStagingDirectory + ?Sized>(
    directories: &mut D,
    the_destinations_arrangements_still_name: &[Vec<u8>],
    the_destinations_requests_still_name: &[Vec<u8>],
    the_transfer_records: &[TransferRecord],
) -> Vec<UnownedDownloadStaging> {
    let mut unowned = Vec::new();
    for (destination, staging) in the_destinations_arrangements_still_name
        .iter()
        .chain(the_destinations_requests_still_name)
        .map(|destination| {
            (
                destination.clone(),
                crate::fetching::the_staging_directory_inside(destination),
            )
        })
        // **The bound is asked of the rules half rather than composed twice**,
        // so a folder outside it cannot be reached even by a caller that
        // handed in a longer list.
        .filter(|(_, staging)| {
            the_folders_that_may_be_looked_in(
                the_destinations_arrangements_still_name,
                the_destinations_requests_still_name,
            )
            .contains(staging)
        })
    {
        for entry in directories.what_is_inside(&staging) {
            match what_a_local_staging_entry_is(&entry, &destination, the_transfer_records) {
                WhatALocalStagingEntryIs::ATransferRecordNamesIt => continue,
                WhatALocalStagingEntryIs::UnownedDownloadStaging { destination } => {
                    unowned.push(UnownedDownloadStaging { entry, destination })
                }
            }
        }
    }
    unowned
}

/// What one **Remove** came to.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhatTheRemovalCameTo {
    ItWasRemoved,
    /// **A refusal reported as an ordinary failed removal**, with the
    /// adapter's own class and no cause composed here.
    ItWasRefused(WhatTheAdapterSaid),
    /// **The entry is not inside the reserved staging directory**, so no
    /// removal was attempted at all. The path was re-read and the bound was
    /// not met, which is the one thing that stands between this act and
    /// following an entry out of Smoozy's own area.
    ItIsNotInsideTheReservedStagingDirectory,
}

/// **M13a's Remove: the user's explicit act on one named entry.**
///
/// **The bound is asked before the adapter is called**, so an entry that is
/// not inside the reserved staging directory is refused here rather than
/// removed and regretted.
///
/// **Nothing about the entry's contents is read** and nothing is inferred from
/// them: the act creates no transfer record, writes no transfer outcome, and
/// infers no publication or failure.
pub fn remove_one_named_entry<D: TheReservedStagingDirectory + ?Sized>(
    directories: &mut D,
    the_entry: &[u8],
    the_destination: &[u8],
) -> WhatTheRemovalCameTo {
    let staging = crate::fetching::the_staging_directory_inside(the_destination);
    if !the_removal_may_run(the_entry, &staging) {
        return WhatTheRemovalCameTo::ItIsNotInsideTheReservedStagingDirectory;
    }
    match directories.remove_this_entry(the_entry) {
        Ok(()) => WhatTheRemovalCameTo::ItWasRemoved,
        Err(said) => WhatTheRemovalCameTo::ItWasRefused(said),
    }
}

#[cfg(test)]
mod tests {
    use smoozy_server_protocol::transfers::Direction;
    use smoozy_state::identities::{Identity, IdentityKind};

    use super::*;

    #[derive(Default)]
    struct ADisk {
        inside: Vec<(Vec<u8>, Vec<Vec<u8>>)>,
        asked: Vec<Vec<u8>>,
        removed: Vec<Vec<u8>>,
        the_removal_refuses: Option<WhatTheAdapterSaid>,
    }

    impl TheReservedStagingDirectory for ADisk {
        fn what_is_inside(&mut self, the_reserved_staging_directory: &[u8]) -> Vec<Vec<u8>> {
            self.asked.push(the_reserved_staging_directory.to_vec());
            self.inside
                .iter()
                .find(|(at, _)| at == the_reserved_staging_directory)
                .map(|(_, entries)| entries.clone())
                .unwrap_or_default()
        }
        fn remove_this_entry(&mut self, the_entry: &[u8]) -> Result<(), WhatTheAdapterSaid> {
            if let Some(said) = self.the_removal_refuses {
                return Err(said);
            }
            self.removed.push(the_entry.to_vec());
            Ok(())
        }
    }

    fn an_identity(kind: IdentityKind) -> Identity {
        Identity::issue(kind).expect("the desk has randomness")
    }

    fn a_record_staging_into(destination: &str) -> TransferRecord {
        TransferRecord::arranged(
            an_identity(IdentityKind::Transfer),
            Direction::Down,
            an_identity(IdentityKind::LocalServer),
            None,
            "/jobs/abc/x".into(),
            format!("{destination}/x"),
            &format!("{destination}/.smoozy-staging"),
            1,
            an_identity(IdentityKind::Run),
        )
    }

    /// **M13a: an entry no transfer record names is unowned download
    /// staging, with the destination named** — and an entry a record does name
    /// is not presented at all.
    #[test]
    fn only_what_no_record_names_is_presented_as_unowned() {
        let record = a_record_staging_into("/Users/a/Downloads/job");
        let mine = record.staging_path.as_bytes();
        let mut disk = ADisk {
            inside: vec![(
                b"/Users/a/Downloads/job/.smoozy-staging".to_vec(),
                vec![
                    mine.clone(),
                    b"/Users/a/Downloads/job/.smoozy-staging/x".to_vec(),
                ],
            )],
            ..ADisk::default()
        };

        let standing = what_is_standing(
            &mut disk,
            &[b"/Users/a/Downloads/job".to_vec()],
            &[],
            std::slice::from_ref(&record),
        );

        assert_eq!(
            standing,
            vec![UnownedDownloadStaging {
                entry: b"/Users/a/Downloads/job/.smoozy-staging/x".to_vec(),
                destination: b"/Users/a/Downloads/job".to_vec(),
            }]
        );
    }

    /// **Enumeration is never ownership evidence and removes nothing.**
    #[test]
    fn the_reading_removes_nothing() {
        let mut disk = ADisk {
            inside: vec![(
                b"/Users/a/Downloads/job/.smoozy-staging".to_vec(),
                vec![b"/Users/a/Downloads/job/.smoozy-staging/x".to_vec()],
            )],
            ..ADisk::default()
        };
        what_is_standing(&mut disk, &[b"/Users/a/Downloads/job".to_vec()], &[], &[]);
        assert!(disk.removed.is_empty());
    }

    /// **The reach is bounded**: only destinations a surviving arrangement or
    /// request record still names are looked in, and nothing anywhere else is
    /// asked or claimed absent.
    #[test]
    fn no_folder_outside_the_bounded_set_is_ever_asked() {
        let mut disk = ADisk::default();
        what_is_standing(&mut disk, &[b"/Users/a/Downloads/job".to_vec()], &[], &[]);
        assert_eq!(
            disk.asked,
            vec![b"/Users/a/Downloads/job/.smoozy-staging".to_vec()]
        );
    }

    /// **The removal never follows an entry outside the reserved staging
    /// directory**, and the bound is asked before the adapter is called at
    /// all.
    #[test]
    fn a_removal_outside_the_reserved_directory_is_refused_before_the_adapter_is_called() {
        let mut disk = ADisk::default();
        let came_to = remove_one_named_entry(
            &mut disk,
            b"/Users/a/Downloads/job/a_frame.exr",
            b"/Users/a/Downloads/job",
        );
        assert_eq!(
            came_to,
            WhatTheRemovalCameTo::ItIsNotInsideTheReservedStagingDirectory
        );
        assert!(disk.removed.is_empty());
    }

    /// **The staging directory itself is never the entry removed**, which is
    /// the fault a bare prefix test lets through.
    #[test]
    fn the_reserved_directory_itself_is_not_a_removable_entry() {
        let mut disk = ADisk::default();
        assert_eq!(
            remove_one_named_entry(
                &mut disk,
                b"/Users/a/Downloads/job/.smoozy-staging",
                b"/Users/a/Downloads/job",
            ),
            WhatTheRemovalCameTo::ItIsNotInsideTheReservedStagingDirectory
        );
        assert!(disk.removed.is_empty());
    }

    /// **A refusal is an ordinary failed removal**, carrying the adapter's own
    /// class and no cause composed here.
    #[test]
    fn a_refused_removal_carries_the_adapters_own_class() {
        let mut disk = ADisk {
            the_removal_refuses: Some(WhatTheAdapterSaid::WritingIsRefused),
            ..ADisk::default()
        };
        assert_eq!(
            remove_one_named_entry(
                &mut disk,
                b"/Users/a/Downloads/job/.smoozy-staging/x",
                b"/Users/a/Downloads/job",
            ),
            WhatTheRemovalCameTo::ItWasRefused(WhatTheAdapterSaid::WritingIsRefused)
        );
    }
}
