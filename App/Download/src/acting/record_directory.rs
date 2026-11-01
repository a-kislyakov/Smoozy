//! **§4 J2 and §6.1 N2 performed — the two desktop-initiated reads of the
//! job's record directory.**
//!
//! The attempt record's ordered frame list, and the recorded reading with the
//! inventory. **Both are readings of a machine over the connection, on the
//! route `Job files and outputs` supplies by name**, and both are readings
//! with their moment rather than state (R5).
//!
//! # The order is read and never guessed, and neither read is retried into a
//! default
//!
//! **J2: the list is read from the attempt record for every attempt, attempt 1
//! among them.** `Job files and outputs` M8 writes one attempt record before
//! each render starts, holding the frame list handed to the render in the
//! order it was written, so attempt 1 is not a special case and needs no rule
//! of its own. **The composition is the machine's and never the desktop's** —
//! the desktop composes no frame list at all, for any attempt; what it
//! composes and sends is the manifest's frame *range*, which is not an order.
//!
//! **So nothing here reads the manifest's range where the attempt record
//! cannot be read.** A range is never the order: a list written `50, 10, 20` is
//! that order, and reading a three-number range in its place would put the
//! frontier at 50 while Blender wrote 50 first, judging frames 10 and 20
//! finished before they were rendered. The value for that case is
//! [`crate::vouching::TheOrder::ItCouldNotBeRead`], **mechanism one does not
//! run for that attempt, and mechanism two alone decides.**
//!
//! # The narrowing material's two homes, and both are read
//!
//! **N2 names two ways the material reaches the desktop, and each has its own
//! file.** Where the reading ran at pre-load, it is inside the manifest `Job
//! files and outputs` M1 writes, and [`read_the_narrowing_material`] reads it
//! there. **Where the job was submitted unread, the render worker takes the
//! reading at render start** (`Rendering` P1 step 4) and writes it, with the
//! map and the inventory, into the job's record directory as one file per
//! attempt (`Job files and outputs` H9 and `Job files and outputs` R13) —
//! [`read_the_recorded_reading_the_machine_wrote`] reads that one. The
//! manifest is never rewritten, which is `Job files and outputs` M1's own
//! rule, so the two are never the same file.
//!
//! *(Corrected 2026-09-28, step 10c's ninth-pass fix piece 1. This header said
//! nothing in this workspace wrote the machine-composed reading into the
//! record directory, which stopped being true at step 10c piece 14, when the
//! render worker began writing it. The desktop went on reading the manifest
//! alone, so a job submitted unread — every job the ninth pass rendered — had
//! no inventory in hand, J3 could judge nothing, and every frame waited for
//! the ending.)*
//!
//! **Until the file lands, what that costs is exactly what §15 item 12 says it
//! costs**: there is no list, the narrowing control has nothing to offer, and
//! the arrangement covers everything — *"it walks and holds, judging
//! nothing"*, which is the honest state and the same one J6's baseline already
//! occupies. **The file lands after render start and never before it**, so a
//! read that finds nothing is asked again at the next pass rather than kept
//! (the caller's rule, `Desktop app and delivery`'s pass).
//!
//! # The two keys are not one key, and the join is here
//!
//! `Job files and outputs` I14 gives an inventory entry its predicted path
//! **whole, on the machine**; R4 keys the reckoning on the predicted path
//! **relative to the job output folder**, which is also what the walk hands
//! back (W4). **They are the same file under two spellings and the conversion
//! is one place** — [`the_entries_keyed_as_this_document_keys_them`] — so no
//! caller anywhere compares a whole path against a relative one and quietly
//! matches nothing.
//!
//! **Markers used in this file that belong to another document, each with the
//! document it belongs to.** `Job files and outputs` I14, I19, I20.
//! `Server protocol and transfers` §1.3.

use smoozy_job_files::attempts::AttemptRecord;
use smoozy_job_files::composing::TheReadingAndWhatFollows;
use smoozy_job_files::inventory::{TheInventory, WhatTheInventoryIs};
use smoozy_job_files::manifest::{
    the_recorded_readings_file_name, SubmissionManifest, TheReadingAndEverythingDerivedFromIt,
    TheRecordedReading, MANIFEST_FILE_NAME,
};
use smoozy_server_protocol::subsystem::SubsystemFailure;

use smoozy_job_files::reading::Destination;

use crate::arrangement::TheNarrowing;
use crate::ending::ANarrowedEntry;
use crate::narrowing::{
    the_whole, what_the_narrowing_says_about, TheNarrowingMaterial, WhatTheNarrowingSaysAbout,
};
use crate::vouching::{TheAttemptsOrder, TheOrder};

/// **The second reading this module takes, and the far side is a machine.**
///
/// One whole file back from the job's record directory. [`super::wiring`] is
/// the real end, on the same reading route the walk rides.
pub trait ReadsOneFileOnTheMachine {
    fn read_one_whole_file(
        &mut self,
        path: &[u8],
    ) -> impl std::future::Future<Output = Result<Vec<u8>, SubsystemFailure>> + Send;
}

/// One path under another, as bytes.
fn under(directory: &[u8], name: &str) -> Vec<u8> {
    let mut out = directory.to_vec();
    if !out.ends_with(b"/") {
        out.push(b'/');
    }
    out.extend_from_slice(name.as_bytes());
    out
}

/// **An attempt record's own file name, asked of the document that owns it.**
///
/// `Job files and outputs` M8 requires each record to be named by the attempt
/// that used it and [`AttemptRecord::file_name`] is that document's spelling of
/// it. **It is a method on a record rather than on a number**, so the name is
/// asked of a record whose only meaningful field is the number — the rest are
/// this function's own zeros and reach nothing, since the record is discarded
/// the moment the name is read. **Nothing here spells the name itself**, which
/// is what keeps the file naming in one home.
fn the_records_file_name(attempt: u32) -> String {
    AttemptRecord {
        number: attempt,
        frame_list_in_the_order_it_was_written: Vec::new(),
        map_in_force: smoozy_job_files::attempts::TheMapInForce::NoneWasComposedForThisAttempt,
        file_identity_at_this_attempt: smoozy_server_protocol::transfers::SourceReading {
            size: 0,
            inode: String::new(),
            modification_time: 0,
            change_time: 0,
        },
        the_file_changed_before_this_attempt: false,
        a_frame_an_earlier_attempt_wrote_was_kept: false,
    }
    .file_name()
}

/// **J2, performed: the attempt's own ordered frame list, read from the job's
/// record directory over the connection.**
///
/// **Every failure is one value and it is the same value**, deliberately: a
/// refusal, an absent record, a connection that went, and bytes that do not
/// read back as an attempt record all mean *this attempt's order is not in
/// hand*, and J2's rule for that is one rule. **Nothing here separates them
/// into a judgement**, because no clause makes a different decision on any of
/// them — what the separation would buy is a telling, and the telling this
/// document makes about a read it could not take is W5's, on the walk.
///
/// **Nothing is vouched for on a guessed order** — the whole reason this
/// function has no fallback.
pub async fn read_the_attempts_order<R>(
    reader: &mut R,
    the_job_record_directory: &[u8],
    attempt: u32,
) -> TheOrder
where
    R: ReadsOneFileOnTheMachine,
{
    let path = under(the_job_record_directory, &the_records_file_name(attempt));
    let Ok(bytes) = reader.read_one_whole_file(&path).await else {
        return TheOrder::ItCouldNotBeRead;
    };
    let Some(read) = AttemptRecord::read_back(&bytes) else {
        return TheOrder::ItCouldNotBeRead;
    };
    // **The record read back must be the attempt that was asked for.** The
    // file name carries the number, so a mismatch means the record directory
    // holds something other than what its own name says — which is a reason to
    // judge nothing, not a reason to use it.
    if read.number != attempt {
        return TheOrder::ItCouldNotBeRead;
    }
    TheOrder::The(TheAttemptsOrder::as_the_attempt_record_carries_it(
        read.frame_list_in_the_order_it_was_written,
    ))
}

/// What the manifest read came to.
///
/// **The inventory is carried beside the narrowing material rather than folded
/// into it**, because they answer two different questions: N1's format list is
/// the reading's, and N6's whole and R4's keys are the inventory's, and a job
/// can carry a reading whose inventory was not composed at all —
/// `Job files and outputs` I9a.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WhatTheRecordDirectoryHeld {
    pub narrowing_material: TheNarrowingMaterial,
    /// **The inventory, where one was composed.** `Job files and outputs` I9a's
    /// withholding is a state
    /// of that document's own field and reaches here as an absence, which is
    /// the same absence as a job whose reading has not landed: no count, and
    /// the arrangement holds.
    pub inventory: Option<TheInventory>,
    /// **The job output folder the manifest names**, which is the folder the
    /// walk runs over and the folder every relative key is relative to.
    ///
    /// **It is the manifest's and never composed here.** `Job files and
    /// outputs` M1 makes the folder the manifest names and the folder that is
    /// actually made one value, *"rather than two that have to be checked
    /// against each other"*, and a second composition on this side would be
    /// the second value that rule exists to prevent.
    pub job_output_folder: Option<String>,
}

impl WhatTheRecordDirectoryHeld {
    /// **N2's *until it lands there is no list and nothing to narrow with*.**
    fn nothing_landed() -> WhatTheRecordDirectoryHeld {
        WhatTheRecordDirectoryHeld {
            narrowing_material: TheNarrowingMaterial::ItHasNotLanded,
            inventory: None,
            job_output_folder: None,
        }
    }
}

/// **N2, performed: the recorded reading and the inventory, read from the
/// job's record directory over the connection, at or after render-start.**
///
/// **A reading with its moment**, and the moment is the caller's to carry: the
/// arrangement holds no durable copy of it (R5), and this is called again on
/// the next connection rather than cached across one.
///
/// **A job submitted unread comes back as *not landed*** from this read, with
/// the job output folder the manifest names, and no reading is composed for it
/// here: its reading is the machine's own file,
/// [`read_the_recorded_reading_the_machine_wrote`]'s to read.
pub async fn read_the_narrowing_material<R>(
    reader: &mut R,
    the_job_record_directory: &[u8],
) -> WhatTheRecordDirectoryHeld
where
    R: ReadsOneFileOnTheMachine,
{
    let path = under(the_job_record_directory, MANIFEST_FILE_NAME);
    let Ok(bytes) = reader.read_one_whole_file(&path).await else {
        return WhatTheRecordDirectoryHeld::nothing_landed();
    };
    let Ok(manifest) = serde_json::from_slice::<SubmissionManifest>(&bytes) else {
        return WhatTheRecordDirectoryHeld::nothing_landed();
    };
    // **The folder the manifest names, or — where it says the folder is
    // chosen at render start — the one the render worker recorded in the same
    // directory** (`Job files and outputs` P21). **Until it has, nothing has
    // landed**: there is no folder to walk, and the caller asks again at the
    // next pass rather than walking a path it composed itself.
    let job_output_folder = match manifest.job_output_folder.the_path_it_names() {
        Some(named) => named.to_string(),
        None => {
            let at = under(
                the_job_record_directory,
                smoozy_job_files::where_the_folder_sits::THE_RECORD_OF_WHERE_THE_FOLDER_SITS,
            );
            let Some(recorded) = reader
                .read_one_whole_file(&at)
                .await
                .ok()
                .and_then(|bytes| smoozy_job_files::where_the_folder_sits::WhereTheJobOutputFolderSits::read_back(&bytes))
            else {
                return WhatTheRecordDirectoryHeld::nothing_landed();
            };
            recorded.path
        }
    };
    let job_output_folder = Some(job_output_folder);
    match manifest.recorded_reading {
        TheRecordedReading::ThereIsNone => WhatTheRecordDirectoryHeld {
            narrowing_material: TheNarrowingMaterial::ItHasNotLanded,
            inventory: None,
            job_output_folder,
        },
        TheRecordedReading::ItWasReadBeforeSubmitting(derived) => {
            what_a_reading_holds(*derived, job_output_folder)
        }
    }
}

/// **N2's second home, read: the recorded reading the render worker wrote into
/// the job's record directory at render start, for one attempt.**
///
/// **Where the job was submitted unread, this is where the reading and the
/// inventory land** — `Rendering` P1 step 4 takes the reading and `Job files
/// and outputs` H9 writes it, one file per attempt (`Job files and outputs`
/// R13), under the name [`the_recorded_readings_file_name`] gives, which is the
/// one both ends read.
///
/// **It is read as the machine's own shape and never a second one**: the file
/// is the render worker's [`TheReadingAndWhatFollows`], and what the desktop
/// takes from it is exactly what a pre-loaded manifest carries —
/// [`TheReadingAndWhatFollows::as_the_manifest_carries_it`] — so both homes
/// come out through [`what_a_reading_holds`] and nothing downstream can tell
/// which one it came from.
///
/// **The attempt is the note's attempt count**, the same number J2 reads the
/// order for; **nought is not attempt one**, and asks for nothing.
///
/// **`None` where it has not landed** — no file yet, a refusal, a connection
/// that went, or bytes that do not read back. **None of those is kept by the
/// caller**: the file is written after render start and never before it, so a
/// read that found nothing is asked again at the next pass. **The job output
/// folder is the manifest's** (`Job files and outputs` M1's one value) and is
/// handed in rather than read here.
pub async fn read_the_recorded_reading_the_machine_wrote<R>(
    reader: &mut R,
    the_job_record_directory: &[u8],
    attempt: u32,
    the_job_output_folder: &str,
) -> Option<WhatTheRecordDirectoryHeld>
where
    R: ReadsOneFileOnTheMachine,
{
    if attempt == 0 {
        return None;
    }
    let path = under(
        the_job_record_directory,
        &the_recorded_readings_file_name(attempt),
    );
    let bytes = reader.read_one_whole_file(&path).await.ok()?;
    let written = serde_json::from_slice::<TheReadingAndWhatFollows>(&bytes).ok()?;
    Some(what_a_reading_holds(
        written.as_the_manifest_carries_it(),
        Some(the_job_output_folder.to_string()),
    ))
}

/// **What a reading hands the arrangement, whichever of N2's two homes it came
/// from**: the outputs half as the narrowing material, and the inventory where
/// one was composed (`Job files and outputs` I9a's withholding reaches here as
/// an absence).
fn what_a_reading_holds(
    derived: TheReadingAndEverythingDerivedFromIt,
    job_output_folder: Option<String>,
) -> WhatTheRecordDirectoryHeld {
    let inventory = match derived.inventory {
        WhatTheInventoryIs::It(inventory) => Some(inventory),
        WhatTheInventoryIs::TheFrameSetIsNotPredictable(_) => None,
    };
    WhatTheRecordDirectoryHeld {
        narrowing_material: TheNarrowingMaterial::OnTheDesktop(Box::new(derived.reading)),
        inventory,
        job_output_folder,
    }
}

/// **The one place the inventory's whole machine paths become this document's
/// own keys.**
///
/// `Job files and outputs` I14 gives an entry its predicted path whole on the
/// machine; R4 keys the reckoning on the path relative to the job output
/// folder, and W4 hands the walk back the same way. **An entry whose predicted
/// path does not lie under the job output folder is dropped**, and that is a
/// refusal rather than a repair: `Job files and outputs` L8 says nothing
/// outside the job output folder is ever counted, listed, watched or brought
/// home as a job's output, so such an entry is not this document's to fetch
/// and composing a key for it would be inventing one.
///
/// **The frame rides across with it**, absent for a video destination's entry
/// (I20), which is what keeps [`crate::ending::the_ending_walk`]'s no-frame
/// rule true of the same entries the walk sees.
pub fn the_entries_keyed_as_this_document_keys_them(
    inventory: &TheInventory,
    the_job_output_folder: &[u8],
) -> Vec<ANarrowedEntry> {
    inventory
        .entries
        .iter()
        .filter_map(|entry| {
            relative_to(the_job_output_folder, &entry.predicted_path).map(|predicted_path| {
                ANarrowedEntry {
                    predicted_path,
                    frame: entry.frame,
                }
            })
        })
        .collect()
}

/// **N4, N5 and N6 applied: the inventory cut by the arrangement's own
/// narrowing, in this document's own keys, with both halves handed back.**
///
/// **The cut has to produce both halves and not only the one that is kept.**
/// N7's third bullet has *skipped by narrowing* leave the whole and be *"out,
/// silently correct"*, and R4 re-opens such an entry to *owed* where the
/// narrowing is widened — **neither of which can happen to an entry that was
/// never written.** So the excluded half is what makes the spec's promise that
/// *"changing their mind about the narrowing costs the user nothing"* keepable
/// at all.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct WhatTheNarrowingLeaves {
    /// **The entries the arrangement is waiting for** — N6's whole, and what
    /// the walk, the count and the ending walk all read.
    pub narrowed: Vec<ANarrowedEntry>,
    /// **The entries the user's choice excludes**, which are written at
    /// *skipped by narrowing* rather than left unentered.
    ///
    /// **An entry the cut cannot speak about is not here.** N4 is emphatic
    /// that a file with no format is *outside the narrowing*, meaning not
    /// filtered out by it — it stays in the narrowed half and comes home.
    pub excluded: Vec<ANarrowedEntry>,
    /// **N6's second half: the video statement**, taken from
    /// [`crate::narrowing::the_whole`] over the same inventory and the same
    /// narrowing, so the whole a video job reads against is stated as possibly
    /// growing rather than as a number that will have to be broken (I19, I20).
    ///
    /// **The count is not taken from there.** N6's number is `narrowed`'s own
    /// length, which is the same cut carried into this document's keys; what
    /// is taken is the statement alone.
    pub it_may_grow: bool,
}

/// **N4 and N5's judgement, applied to a whole inventory.**
///
/// The match is against the inventory by predicted path, never against the
/// file's name, and the judgement itself is
/// [`crate::narrowing::what_the_narrowing_says_about`]'s — **this function
/// applies it and takes no second one.**
pub fn what_the_narrowing_leaves(
    inventory: &TheInventory,
    destinations: &[Destination],
    narrowing: &TheNarrowing,
    the_job_output_folder: &[u8],
) -> WhatTheNarrowingLeaves {
    let mut out = WhatTheNarrowingLeaves::default();
    for entry in &inventory.entries {
        let Some(predicted_path) = relative_to(the_job_output_folder, &entry.predicted_path) else {
            continue;
        };
        let keyed = ANarrowedEntry {
            predicted_path,
            frame: entry.frame,
        };
        match what_the_narrowing_says_about(
            narrowing,
            destinations,
            &entry.the_destinations_aiming_at_it,
        ) {
            WhatTheNarrowingSaysAbout::ItIsNotOneOfTheChosenFormats => out.excluded.push(keyed),
            WhatTheNarrowingSaysAbout::ItIsOneOfTheChosenFormats
            | WhatTheNarrowingSaysAbout::TheCutCannotSpeakAboutIt => out.narrowed.push(keyed),
        }
    }
    // **N6's video statement is read here and nowhere else in the acting
    // half.** It is read over the narrowed entries rather than off the raw
    // inventory, which is what `the_whole` itself does.
    out.it_may_grow = the_whole(inventory, destinations, narrowing).it_may_grow;
    out
}

/// One path under a folder, as the path relative to that folder — or nothing
/// where it does not lie under it.
///
/// **The comparison is byte-for-byte and component-wise**: a folder
/// `/jobs/abc` does not contain `/jobs/abcd`, which a plain prefix test would
/// say it did.
fn relative_to(folder: &[u8], path: &[u8]) -> Option<Vec<u8>> {
    let folder = folder.strip_suffix(b"/".as_ref()).unwrap_or(folder);
    let rest = path.strip_prefix(folder)?;
    let rest = rest.strip_prefix(b"/".as_ref())?;
    if rest.is_empty() {
        return None;
    }
    Some(rest.to_vec())
}

/// **What Smoozy itself placed in a job's folder before the render, and how
/// each placed file stood once the render had ended** — `Download` N5a's two
/// inputs, read off the job's record directory. Added 2026-10-02, the fixes
/// before 0.1.8, fault 3: until then nothing on this side read either, so the
/// ending walk was handed no list and every copy Smoozy placed came home with
/// the job's frames — a baked cache of 2.8 GB on the check before 0.1.8.
///
/// **The list is `Dependency Upload & Remap` M22f's**, as the render worker
/// writes it for the attempt. **The four values *now* are the machine's own
/// reading of the same files the moment that attempt's render ended**, because
/// the file-transfer surface carries two of the four and no more (`Server
/// protocol and transfers` S13) — and after the render has ended nothing writes
/// into the job's folder, so that reading is the one the ending walk would take.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct WhatSmoozyPlaced {
    pub list: smoozy_dependencies::remap::PlacedCopyList,
    /// **By whole machine path.** Empty where the reading after the render is
    /// not there — a render that never ended in the worker's hands — and then
    /// every placed path is one N5a could not judge, and it comes home: the
    /// clause's own safe direction.
    pub four_values_now: Vec<(Vec<u8>, smoozy_server_protocol::transfers::SourceReading)>,
}

impl WhatSmoozyPlaced {
    /// **The whole machine paths N5a skips**: placed by Smoozy, and with all
    /// four values where they were when the copy was made.
    pub fn the_copies_still_as_placed(&self) -> Vec<Vec<u8>> {
        self.four_values_now
            .iter()
            .filter(|(path, now)| {
                crate::narrowing::whether_it_is_skipped_on_the_way_home(Some(&self.list), path, now)
                    == crate::narrowing::WhetherItIsSkippedOnTheWayHome::ItIsTheCopySmoozyPlaced
            })
            .map(|(path, _)| path.clone())
            .collect()
    }
}

/// **N5a's inputs, read for one attempt.**
///
/// - **`Ok(None)`: nothing was placed** — the list is not in the record
///   directory, which is every job with no baked cache to copy.
/// - **`Err(())`: the list could not be read** — a refusal other than *no such
///   file*, a connection that went, or bytes that do not read back. It is not
///   a reading and the caller asks again rather than keeping it.
/// - **`Ok(Some(…))`**, with the reading after the render where it is there.
pub async fn read_what_smoozy_placed<R>(
    reader: &mut R,
    the_job_record_directory: &[u8],
    attempt: u32,
) -> Result<Option<WhatSmoozyPlaced>, ()>
where
    R: ReadsOneFileOnTheMachine,
{
    use smoozy_dependencies::acting::placing::{
        the_placed_copies_after_the_render_file_name, the_placed_copies_file_name, ThePlacedCopies,
    };
    if attempt == 0 {
        return Ok(None);
    }
    let as_n5a_reads_it = |file: &smoozy_dependencies::acting::placing::APlacedFile| {
        (
            file.job_folder_path.as_bytes().to_vec(),
            smoozy_server_protocol::transfers::SourceReading {
                size: file.size,
                inode: file.inode.clone(),
                modification_time: file.modification_time,
                change_time: file.change_time,
            },
        )
    };
    let listed = match reader
        .read_one_whole_file(&under(the_job_record_directory, &the_placed_copies_file_name(attempt)))
        .await
    {
        Ok(bytes) => ThePlacedCopies::read_back(&bytes).ok_or(())?,
        Err(SubsystemFailure::TheServerRefused(
            smoozy_server_protocol::subsystem::StatusCode::NoSuchFile,
        )) => return Ok(None),
        Err(_) => return Err(()),
    };
    let list = smoozy_dependencies::remap::PlacedCopyList {
        copies: listed
            .files
            .iter()
            .map(|file| {
                let (path, made) = as_n5a_reads_it(file);
                smoozy_dependencies::remap::PlacedCopy {
                    job_folder_path: smoozy_dependencies::bytes::MachineLocation::composed(path),
                    four_values_at_placement: made,
                }
            })
            .collect(),
    };
    let four_values_now = match reader
        .read_one_whole_file(&under(
            the_job_record_directory,
            &the_placed_copies_after_the_render_file_name(attempt),
        ))
        .await
    {
        Ok(bytes) => ThePlacedCopies::read_back(&bytes)
            .map(|after| after.files.iter().map(as_n5a_reads_it).collect())
            .unwrap_or_default(),
        Err(_) => Vec::new(),
    };
    Ok(Some(WhatSmoozyPlaced {
        list,
        four_values_now,
    }))
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use smoozy_job_files::attempts::TheMapInForce;
    use smoozy_job_files::inventory::{AnEntry, WhichRouteProducedTheName};
    use smoozy_server_protocol::transfers::SourceReading;

    use super::*;

    #[derive(Default)]
    struct ARecordDirectory {
        files: HashMap<Vec<u8>, Result<Vec<u8>, SubsystemFailure>>,
    }

    impl ARecordDirectory {
        fn holding(mut self, path: &[u8], bytes: Vec<u8>) -> ARecordDirectory {
            self.files.insert(path.to_vec(), Ok(bytes));
            self
        }
    }

    impl ReadsOneFileOnTheMachine for ARecordDirectory {
        async fn read_one_whole_file(&mut self, path: &[u8]) -> Result<Vec<u8>, SubsystemFailure> {
            self.files
                .get(path)
                .cloned()
                .unwrap_or(Err(SubsystemFailure::TheServerRefused(
                    smoozy_server_protocol::subsystem::StatusCode::NoSuchFile,
                )))
        }
    }

    fn run<F: std::future::Future>(f: F) -> F::Output {
        let mut f = Box::pin(f);
        let waker = std::task::Waker::noop();
        let mut context = std::task::Context::from_waker(waker);
        loop {
            if let std::task::Poll::Ready(out) = f.as_mut().poll(&mut context) {
                return out;
            }
        }
    }

    fn a_record(number: u32, frames: Vec<i64>) -> AttemptRecord {
        AttemptRecord {
            number,
            frame_list_in_the_order_it_was_written: frames,
            map_in_force: TheMapInForce::NoneWasComposedForThisAttempt,
            file_identity_at_this_attempt: SourceReading {
                size: 1,
                inode: "1".into(),
                modification_time: 1,
                change_time: 1,
            },
            the_file_changed_before_this_attempt: false,
            a_frame_an_earlier_attempt_wrote_was_kept: false,
        }
    }

    /// **J2: the order is the attempt record's own, in the order it was
    /// written**, and a typed list is not sorted anywhere on the way.
    #[test]
    fn the_order_comes_back_in_the_order_the_record_wrote_it() {
        let record = a_record(3, vec![50, 10, 20]);
        let mut machine = ARecordDirectory::default()
            .holding(b"/job-records/abc/attempt-3.json", record.as_written());

        let order = run(read_the_attempts_order(
            &mut machine,
            b"/job-records/abc",
            3,
        ));

        match order {
            TheOrder::The(order) => assert_eq!(order.frames(), &[50, 10, 20]),
            other => panic!("expected an order, got {other:?}"),
        }
    }

    /// **J2's fallback, and it has no default**: a record that cannot be read
    /// is a value, mechanism one does not run for that attempt, and nothing
    /// anywhere composes an order out of the manifest's range.
    #[test]
    fn a_record_that_cannot_be_read_is_a_value_and_never_a_range() {
        let mut machine = ARecordDirectory::default();
        let order = run(read_the_attempts_order(
            &mut machine,
            b"/job-records/abc",
            1,
        ));
        assert_eq!(order, TheOrder::ItCouldNotBeRead);
        assert!(!order.mechanism_one_runs());

        // And bytes that are not a record read back the same way rather than
        // as an empty order, which would judge every frame finished.
        let mut nonsense = ARecordDirectory::default()
            .holding(b"/job-records/abc/attempt-1.json", b"not a record".to_vec());
        assert_eq!(
            run(read_the_attempts_order(
                &mut nonsense,
                b"/job-records/abc",
                1
            )),
            TheOrder::ItCouldNotBeRead
        );
    }

    /// **A record whose own number is not the attempt asked for judges
    /// nothing.** The file name carries the number, so a disagreement means the
    /// directory holds something other than what its name says.
    #[test]
    fn a_record_naming_another_attempt_is_not_used() {
        let record = a_record(2, vec![1, 2, 3]);
        let mut machine = ARecordDirectory::default()
            .holding(b"/job-records/abc/attempt-1.json", record.as_written());
        assert_eq!(
            run(read_the_attempts_order(
                &mut machine,
                b"/job-records/abc",
                1
            )),
            TheOrder::ItCouldNotBeRead
        );
    }

    /// **N2: a manifest that cannot be read leaves the material *not
    /// landed***, which is no list and nothing to narrow with — never an empty
    /// list, which would read as a job that writes nothing.
    #[test]
    fn a_manifest_that_cannot_be_read_leaves_the_material_not_landed() {
        let mut machine = ARecordDirectory::default();
        let held = run(read_the_narrowing_material(
            &mut machine,
            b"/job-records/abc",
        ));
        assert_eq!(
            held.narrowing_material,
            TheNarrowingMaterial::ItHasNotLanded
        );
        assert!(held.inventory.is_none());
        assert!(!held.narrowing_material.a_narrowing_may_be_offered());
        assert!(held.narrowing_material.the_formats_it_offers().is_empty());
    }

    /// **`Job files and outputs` P21's second case: a manifest that says the
    /// folder is chosen at render start has landed nothing until the render
    /// worker records where it chose**, and then the folder walked is the
    /// recorded one — never a path composed on this side. The Creator's second
    /// notes, piece 4.
    #[test]
    fn a_folder_chosen_at_render_start_is_the_one_the_machine_recorded_and_nothing_before_it() {
        let manifest = serde_json::json!({
            "job": "job-1",
            "group": null,
            "blender": {"version": "5.2.2", "build_path": "/blender/5.2.2/blender"},
            "scene": {"name": "Scene", "anchor": {"ToAnchor": {"provenance": "typed"}}},
            "frame_range": "TheFilesOwn",
            "settings_payload": {},
            "remap_material": {},
            "reporting_settings": "everything",
            "blend_path_on_the_machine": "/home/ubuntu/rendering/kitchen.blend",
            "file_identity_at_submission": {"size": 1, "inode": "1", "modification_time": 1, "change_time": 1},
            "recorded_reading": "ThereIsNone",
            "card_make": "NoCard",
            "job_output_folder": {"the_folders_name": "job-1-Kitchen", "the_servers_fallback_folder_at_submission": null}
        });
        let manifest: SubmissionManifest = serde_json::from_value(manifest).expect("a manifest");
        assert_eq!(manifest.job_output_folder.the_path_it_names(), None);
        let mut machine = ARecordDirectory::default()
            .holding(b"/job-records/abc/manifest.json", serde_json::to_vec(&manifest).unwrap());
        let held = run(read_the_narrowing_material(&mut machine, b"/job-records/abc"));
        assert_eq!(held.job_output_folder, None, "nothing chosen yet, nothing to walk");

        let sits = smoozy_job_files::where_the_folder_sits::WhereTheJobOutputFolderSits {
            path: "/home/ubuntu/Render/job-1-Kitchen".to_string(),
            where_it_is: smoozy_job_files::where_the_folder_sits::WhereItIs::WhereTheFilePoints,
        };
        let mut machine = machine.holding(b"/job-records/abc/job-output-folder.json", sits.as_written());
        let held = run(read_the_narrowing_material(&mut machine, b"/job-records/abc"));
        assert_eq!(held.job_output_folder.as_deref(), Some("/home/ubuntu/Render/job-1-Kitchen"));
        assert_eq!(held.narrowing_material, TheNarrowingMaterial::ItHasNotLanded, "the reading is the worker's own file");
    }

    /// **The join between two spellings of one path**, and an entry outside
    /// the job output folder is dropped rather than given a composed key.
    #[test]
    fn the_inventorys_whole_paths_become_this_documents_relative_keys() {
        let inventory = TheInventory {
            entries: vec![
                AnEntry {
                    predicted_path: b"/jobs/abc/layer/0001.exr".to_vec(),
                    frame: Some(1),
                    the_route_that_produced_the_name:
                        WhichRouteProducedTheName::ItWasAskedOfBlender,
                    the_destinations_aiming_at_it: vec![0],
                    may_grow: false,
                },
                AnEntry {
                    predicted_path: b"/jobs/abcd/elsewhere.exr".to_vec(),
                    frame: Some(2),
                    the_route_that_produced_the_name:
                        WhichRouteProducedTheName::ItWasAskedOfBlender,
                    the_destinations_aiming_at_it: vec![0],
                    may_grow: false,
                },
            ],
            added_write_place_folders: Vec::new(),
            collision_record: Vec::new(),
            a_destination_is_video_shaped: false,
            why_it_owes_nothing: None,
        };

        let keyed = the_entries_keyed_as_this_document_keys_them(&inventory, b"/jobs/abc");

        assert_eq!(
            keyed,
            vec![ANarrowedEntry {
                predicted_path: b"layer/0001.exr".to_vec(),
                frame: Some(1),
            }]
        );
    }

    /// **N4, N5 and N6: the cut produces both halves, and the half the cut
    /// cannot speak about stays in the whole.**
    ///
    /// The user asked for PNG. The EXR has a format the choice did not pick,
    /// so it is excluded and will be written at *skipped by narrowing*. The
    /// cache file has no format at all — an added write place has no per-file
    /// entries and so nothing for the cut to select on — so **it is not
    /// filtered out**, which is N4's own correction.
    #[test]
    fn the_cut_hands_back_both_halves_and_a_file_with_no_format_stays_in_the_whole() {
        use crate::narrowing::tests::a_destination;

        let destinations = vec![
            a_destination(Some("PNG")),
            a_destination(Some("OPEN_EXR")),
            a_destination(None),
        ];
        let entry = |path: &[u8], frame: Option<i64>, destination: usize| AnEntry {
            predicted_path: path.to_vec(),
            frame,
            the_route_that_produced_the_name: WhichRouteProducedTheName::ItWasAskedOfBlender,
            the_destinations_aiming_at_it: vec![destination],
            may_grow: false,
        };
        let inventory = TheInventory {
            entries: vec![
                entry(b"/jobs/abc/f_0001.png", Some(1), 0),
                entry(b"/jobs/abc/f_0001.exr", Some(1), 1),
                entry(b"/jobs/abc/cache/fluid.bphys", None, 2),
                // Outside the job output folder: dropped rather than keyed.
                entry(b"/jobs/abcd/elsewhere.png", Some(1), 0),
            ],
            added_write_place_folders: Vec::new(),
            collision_record: Vec::new(),
            a_destination_is_video_shaped: false,
            why_it_owes_nothing: None,
        };

        let left = what_the_narrowing_leaves(
            &inventory,
            &destinations,
            &TheNarrowing::TheseFormats(vec![b"PNG".to_vec()]),
            b"/jobs/abc",
        );

        assert_eq!(
            left.narrowed
                .iter()
                .map(|entry| entry.predicted_path.clone())
                .collect::<Vec<Vec<u8>>>(),
            vec![b"f_0001.png".to_vec(), b"cache/fluid.bphys".to_vec()]
        );
        assert_eq!(
            left.excluded
                .iter()
                .map(|entry| entry.predicted_path.clone())
                .collect::<Vec<Vec<u8>>>(),
            vec![b"f_0001.exr".to_vec()]
        );

        // **Everything is unnarrowed cuts nothing out**, which is N3's
        // *everything is no filter rather than a frozen list*.
        let everything = what_the_narrowing_leaves(
            &inventory,
            &destinations,
            &TheNarrowing::Everything,
            b"/jobs/abc",
        );
        assert!(everything.excluded.is_empty());
        assert_eq!(everything.narrowed.len(), 3);
    }

    /// **A sibling folder whose name begins with the job folder's name is not
    /// inside it**, which a plain prefix test would get wrong and which is the
    /// one way a job could fetch another job's outputs.
    #[test]
    fn a_sibling_with_a_longer_name_is_not_inside_the_job_output_folder() {
        assert_eq!(relative_to(b"/jobs/abc", b"/jobs/abcd/x.png"), None);
        assert_eq!(
            relative_to(b"/jobs/abc", b"/jobs/abc/x.png"),
            Some(b"x.png".to_vec())
        );
        assert_eq!(relative_to(b"/jobs/abc", b"/jobs/abc"), None);
    }
}
