//! **§8.2 M2, M3 and M4 performed — the manual request's own remote walk.**
//!
//! One directory reading per folder, descending from **what the user picked**,
//! sizes and kinds carried in the same reading, landing paths composed per
//! P11, names checked per P4, vouching marks per J9. **The record the walk
//! fills is [`crate::request::AManualRequest`]'s and its shape is not taken
//! again here.**
//!
//! # This walk is not §3's walk, and the difference is the whole of J9a
//!
//! §3's walk is of one job output folder and everything under it, and the
//! arrangement holds the attempt's frame order and the inventory. **This one
//! descends from what the user picked, which need not be a whole folder and
//! need not be a job's folder at all.** So nothing here computes a frontier —
//! `NO_FRONTIER_IS_COMPUTED_OVER_WHAT_THE_USER_PICKED`
//! ([`crate::vouching`]) is the rule and this module keeps it by having no
//! frame order to compute one over.
//!
//! # A refused branch ends and the enumeration is still complete
//!
//! **M4: a folder that refuses readdir ends its branch with a recorded
//! refusal, and the enumeration is still complete** — the unit is the whole
//! question, and it is complete when every branch has ended in a listing or a
//! recorded per-entry refusal. **So the totals are known and honest, the
//! refused subtrees are counted in neither total and are reported as what
//! could not be read, and no count is wrong in the quiet direction.**
//!
//! **That is the opposite handling from §3's walk and both are right.** A
//! walk feeding a judgement about what is finished must be complete or
//! discarded; an enumeration the user is about to answer a question over must
//! say what it could not reach and carry on with the rest. *Do what can be
//! done, and say what can't.*
//!
//! # Where the refusal arose is `File Browser` P3's, adopted rather than
//! adapted
//!
//! **M3: refusals separate on the route itself, at readdir and at stat alike,
//! and the walk names the component.** All three failures — *may not read it*,
//! *gone*, *the machine did not answer* — arrive as distinct protocol statuses
//! in one request (`File Browser` P1). What the component walk adds is
//! **where**: components opened in turn from the deepest listable ancestor,
//! the refusal named at the component it arises at, with that document's P4's
//! residue message
//! where the walk cannot reach that far. **It is that document's own
//! [`walk`](smoozy_file_browser::permission::walk), called, and not a second
//! wording.**
//!
//! **Markers used in this file that belong to another document, each with the
//! document it belongs to.** `File Browser` O7.
//! `Server protocol and transfers` §1.3. `Job files and outputs` N15.

use smoozy_file_browser::permission::{CanListAComponent, WalkOutcome, WhatSmoozyMet};
use smoozy_server_protocol::subsystem::SubsystemFailure;
use smoozy_state::identities::Identity;

use crate::fetching::{judge_the_landing_path, the_landing_path, TheReceivingSystemsRules};
use crate::request::{ARecordedRefusal, EnumerationEntry};
use crate::vouching::{the_mark_for, AVouchingMark, WhatTheLocalRecordSays};

use super::walking::{ADirectoryEntry, ReadsOneDirectoryOnTheMachine};

/// What one complete enumeration came to.
///
/// **M4's completeness is a property of this value rather than a claim made
/// about it**: every branch ended in a listing or in a recorded refusal, and
/// both are here.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct TheEnumeration {
    /// One entry per file, in the order the walk met them — which is P2's
    /// order for a request and `File Browser` O7's shape.
    pub entries: Vec<EnumerationEntry>,
    /// **M4's recorded refusals**, one per branch that ended in one.
    pub refusals: Vec<ARecordedRefusal>,
    /// **M9: the landing folders a picked folder puts on the user's disk.**
    ///
    /// *"A downloaded folder arrives as a folder"*, and **a folder with no
    /// files in it has no transfer to make one for it** — so a request for an
    /// empty folder would otherwise complete having put nothing anywhere. They
    /// are created through the platform file adapter
    /// ([`MakesTheLandingFolder`]), and a folder that already exists merges,
    /// which is the spec's own rule.
    pub landing_folders: Vec<Vec<u8>>,
}

/// **M9's own act: a landing folder created through the platform file
/// adapter.**
///
/// **The far side is `Desktop app and delivery`'s platform file adapter**,
/// which cannot be reached from a feature module, and **no implementation of
/// this trait exists outside this crate's own tests**
/// ([`crate::arriving_later`]).
pub trait MakesTheLandingFolder {
    /// **One landing folder, created where it is absent.** A folder that
    /// already stands is not a failure: the spec has a folder that exists
    /// merge rather than collide.
    fn create_the_landing_folder(
        &mut self,
        the_landing_folder: &[u8],
    ) -> Result<(), crate::destination::WhatTheAdapterSaid>;
}

/// **M9, performed: the folders a picked folder puts on the user's disk.**
///
/// **It is the enumeration's own answer that is acted on**, so nothing here
/// re-derives which picked things were folders. Each outcome is handed back
/// with its folder, because `Desktop app and delivery` F2's rule is that a
/// failure is reported and never read as a clean finish.
pub fn make_the_landing_folders<M: MakesTheLandingFolder + ?Sized>(
    maker: &mut M,
    enumeration: &TheEnumeration,
) -> Vec<(Vec<u8>, Result<(), crate::destination::WhatTheAdapterSaid>)> {
    enumeration
        .landing_folders
        .iter()
        .map(|folder| (folder.clone(), maker.create_the_landing_folder(folder)))
        .collect()
}

/// What the caller supplies about a path's job, so the vouching mark can be
/// made **with no read of the machine** (J9).
///
/// **It is asked per file rather than once**, because a request may reach
/// files of more than one job, and files of no job at all.
pub trait ReadsTheLocalJobRecords {
    /// What the local job records say about the job whose output folder this
    /// path lies in, if any.
    fn what_the_local_record_says(&self, source_path: &[u8]) -> WhatTheLocalRecordSays;
    /// **The file's frame**, where the inventory of that job is in hand and
    /// names it — which is what J9b's reckoning speaks about. **Absent is a
    /// value**: J9b's second bullet gives a file with no frame the
    /// whole-folder mark.
    fn the_files_frame(&self, source_path: &[u8]) -> Option<i64>;
    /// **M9 and P10: whether the thing the user picked *is* a job's own output
    /// folder, and which job.**
    ///
    /// **It is asked of the picked thing and of nothing else**, because that
    /// is the one case that brings a whole job home: H5a's own arithmetic
    /// separates a picked root at the job's output folder from a picked root
    /// anywhere below it, and only the first puts the job's folder on the
    /// user's disk.
    ///
    /// **Absent is the ordinary case** — a file, a folder inside a job's
    /// output folder, or something of no job at all — and it lands beside the
    /// destination as before.
    fn the_job_whose_output_folder_this_is(&self, picked: &[u8]) -> Option<AJobsOwnFolder>;

    /// **`Download` N5a, asked of one file a manual request reached: whether
    /// it is a copy Smoozy itself placed in a job's folder before the render
    /// and that nothing has touched since.** Such a file is not enumerated, and
    /// no count moves. Added 2026-10-02, the fixes before 0.1.8, fault 3: *Download
    /// all* on a job brought home the baked cache Smoozy had copied in — all
    /// 2.8 GB of it on the check before 0.1.8.
    ///
    /// **The answer is the caller's reading of the job's record directory**
    /// ([`super::record_directory::read_what_smoozy_placed`]); **absent is
    /// *no***, which brings the file home — N5a's own safe direction.
    fn is_a_copy_smoozy_placed(&self, _source_path: &[u8]) -> bool {
        false
    }
}

/// **The job a picked output folder belongs to**, as the local job records
/// name it.
///
/// **Both fields are the local record's** — no machine is read for either —
/// and the label is what P10 renders into the landing folder's name.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AJobsOwnFolder {
    pub job: Identity,
    /// **Absent is a value**: P10 falls back to the bare job identity, which
    /// is safe in every system's unit.
    pub label: Option<String>,
}

/// **M2, performed: anything the login can read can be requested, and the
/// enumeration is a remote walk on the subsystem route.**
///
/// One directory reading per folder, descending from each thing the user
/// picked; **sizes and kinds carried in the same reading**, so no stat per
/// entry is issued and a name holding a newline cannot split anything
/// (`Job files and outputs` N15).
///
/// **Landing paths are composed per P11** — the destination plus the path
/// relative to what was asked for, component for component, names carried as
/// the machine's bytes and never rewritten in flight. **Names are checked per
/// P4 at enumeration**, which is this half's moment, and a refused entry
/// carries its refusal rather than a repaired name (P8).
///
/// **Every branch that refuses ends with a recorded refusal and the walk
/// carries on** (M4).
pub async fn enumerate<R, N, J>(
    reader: &mut R,
    rules: &N,
    records: &J,
    what_the_user_picked: &[Vec<u8>],
    the_destination: &[u8],
) -> TheEnumeration
where
    R: ReadsOneDirectoryOnTheMachine + Send,
    N: TheReceivingSystemsRules + ?Sized,
    J: ReadsTheLocalJobRecords,
{
    let mut enumeration = TheEnumeration::default();
    // **The landing folders of the folders that held a copy N5a left behind.**
    let mut held_a_copy: Vec<Vec<u8>> = Vec::new();
    // **What the user picked always arrives as a folder** (M9), so a picked
    // folder holding only copies is still made rather than read as nothing.
    let mut picked_landings: Vec<Vec<u8>> = Vec::new();
    for picked in what_the_user_picked {
        // **P11's *relative to what was asked for* is relative to the picked
        // thing's own parent**, so a picked folder arrives as a folder under
        // the destination and a picked file arrives beside it. That is H5a's
        // own arithmetic read from the other end.
        let from = parent_of(picked);
        // **M9 and P10: where the picked thing is a job's own output folder,
        // the job's folder goes between the destination and the files, under
        // P10's composed name rather than the machine's.** The spec says a
        // whole job's outputs come home inside the job's own folder
        // *"whichever way they were asked for"*, so this route and the
        // arrangement's come through the one composition. Relative paths are
        // then taken from the picked folder itself rather than from its
        // parent, because the folder's own name is composed rather than
        // carried.
        let the_jobs_folder = records.the_job_whose_output_folder_this_is(picked);
        let from = match &the_jobs_folder {
            Some(_) => picked.clone(),
            None => from,
        };
        let mut still_to_read: Vec<Vec<u8>> = vec![picked.clone()];
        // The picked thing may be a file, in which case the reading refuses
        // and the component walk says where. **A picked file is not a refused
        // branch**: it is one entry, and it is read as one by asking its own
        // parent for it.
        while let Some(at) = still_to_read.pop() {
            match reader.read_one_directory(&at).await {
                Ok(read) => {
                    // **M9: a downloaded folder arrives as a folder.** The
                    // reading answered, so this is a folder, and its landing
                    // place is recorded for the adapter to create. **A folder
                    // holding no files has no transfer to make one for it**,
                    // so without this a request for an empty folder would
                    // complete having put nothing on the user's disk.
                    let landing = the_landing_place_of(
                        rules,
                        &at,
                        &from,
                        the_destination,
                        the_jobs_folder.as_ref(),
                    );
                    if &at == picked {
                        picked_landings.push(landing.clone());
                    }
                    if !enumeration.landing_folders.contains(&landing) {
                        enumeration.landing_folders.push(landing);
                    }
                    for entry in read {
                        if entry.name == b"." || entry.name == b".." {
                            continue;
                        }
                        // **Smoozy's own staging folder is never brought home**
                        // — the fixes before 0.1.8, fault 3, and the same step
                        // the arrangement's walk takes.
                        if super::walking::is_smoozys_own_staging_folder(&entry.name) {
                            continue;
                        }
                        let source = under(&at, &entry.name);
                        // **An entry whose kind the reply did not carry is
                        // asked rather than guessed at.** This walk's answer
                        // to an unreadable branch is M4's — *do what can be
                        // done, and say what can't* — so an entry of unknown
                        // kind joins the list of things to read: if it is a
                        // folder the reading answers, and if it is a file the
                        // reading refuses and it is found as one entry by
                        // asking its own parent. **Reading the absence as *not
                        // a folder* would enumerate a directory as a file and
                        // arrange a transfer for it.**
                        if entry.kind.is_a_folder().unwrap_or(true) {
                            still_to_read.push(source);
                            continue;
                        }
                        if records.is_a_copy_smoozy_placed(&source) {
                            held_a_copy.push(the_landing_place_of(
                                rules,
                                &at,
                                &from,
                                the_destination,
                                the_jobs_folder.as_ref(),
                            ));
                            continue;
                        }
                        enumeration.entries.push(one_entry(
                            rules,
                            records,
                            &source,
                            &from,
                            the_destination,
                            the_jobs_folder.as_ref(),
                            &entry,
                        ));
                    }
                }
                Err(failure) => match a_picked_file(reader, &at).await {
                    Some(entry) => enumeration.entries.push(one_entry(
                        rules,
                        records,
                        &at,
                        &from,
                        the_destination,
                        the_jobs_folder.as_ref(),
                        &entry,
                    )),
                    None => enumeration
                        .refusals
                        .push(the_recorded_refusal(reader, &at, &failure).await),
                },
            }
        }
    }
    // **A folder whose files were all Smoozy's own copies is not made at
    // home** — after 0.1.8, the checks' leftovers, item 4: *Download all*
    // skipped every copy in a job's `pointcache/` and still made the folder,
    // empty. **M9's empty folder is the machine's own empty one**, which still
    // arrives; one that held a copy N5a left behind, and has nothing coming
    // home in it or below it, is not.
    if !held_a_copy.is_empty() {
        let is_below = |inner: &[u8], outer: &[u8]| {
            inner.len() > outer.len() && inner.starts_with(outer) && inner[outer.len()] == b'/'
        };
        let mut folders: Vec<Vec<u8>> = enumeration.landing_folders.clone();
        // Deepest first, so a folder emptied this way empties the one above it.
        folders.sort_by_key(|folder| std::cmp::Reverse(folder.len()));
        for folder in folders {
            if !held_a_copy.contains(&folder) || picked_landings.contains(&folder) {
                continue;
            }
            let something_comes_home = enumeration
                .entries
                .iter()
                .any(|entry| is_below(&entry.landing_path, &folder))
                || enumeration
                    .landing_folders
                    .iter()
                    .any(|other| is_below(other, &folder));
            if !something_comes_home {
                enumeration.landing_folders.retain(|other| other != &folder);
                if let Some(above) = folder.iter().rposition(|byte| *byte == b'/') {
                    held_a_copy.push(folder[..above].to_vec());
                }
            }
        }
    }
    enumeration
}

/// **A picked thing that is not a directory is one file**, found by asking its
/// own parent for it rather than by reading a mode Smoozy would then have to
/// interpret.
///
/// **This is the one place a reading is retried**, and it is not a retry of a
/// failure: it is the same question asked of the right folder. Where the
/// parent does not hold it either, the branch is a refusal and M4 records it.
async fn a_picked_file<R>(reader: &mut R, at: &[u8]) -> Option<ADirectoryEntry>
where
    R: ReadsOneDirectoryOnTheMachine,
{
    let parent = parent_of(at);
    let name = at.strip_prefix(parent.as_slice())?;
    let name = name.strip_prefix(b"/".as_ref()).unwrap_or(name);
    let read = reader.read_one_directory(&parent).await.ok()?;
    read.into_iter()
        // **An entry whose kind was not carried may be this file**, and where
        // it is not the reading of it will refuse in its turn.
        .find(|entry| entry.name == name && entry.kind.is_a_folder() != Some(true))
}

/// **M3's *where*: `File Browser` P3's component walk, called.**
///
/// The refusal is named at the component it arises at, from the deepest
/// listable ancestor, with `File Browser` P4's residue where the walk cannot
/// reach that far.
/// **`File Browser` P1's separation is read off the protocol's own status and
/// nothing here parses a message.**
async fn the_recorded_refusal<R>(
    reader: &mut R,
    at: &[u8],
    failure: &SubsystemFailure,
) -> ARecordedRefusal
where
    R: ReadsOneDirectoryOnTheMachine + Send,
{
    let met = WhatSmoozyMet::from(failure);
    // The component walk takes a path as text, which is that document's own
    // surface. **A path that is not text is not walked and the refusal is
    // recorded where it arose**, which is the honest answer and never a
    // mangled component name.
    let Ok(text) = std::str::from_utf8(at) else {
        return ARecordedRefusal {
            at: at.to_vec(),
            what_smoozy_met: met,
        };
    };
    let mut lister = TheComponentWalksReader { reader };
    let outcome = smoozy_file_browser::permission::walk(text, &met, &mut lister).await;
    let at = match &outcome {
        WalkOutcome::RefusedAt(component) | WalkOutcome::GoneAt(component) => {
            component.as_bytes().to_vec()
        }
        // **`File Browser` P4's residue**: the walk could not say where, so the
        // branch itself
        // is what is named and no component is claimed.
        WalkOutcome::CouldNotSayWhere(_) => at.to_vec(),
    };
    ARecordedRefusal {
        at,
        what_smoozy_met: met,
    }
}

/// The component walk's own reader, over this module's one reading.
struct TheComponentWalksReader<'a, R> {
    reader: &'a mut R,
}

impl<R: ReadsOneDirectoryOnTheMachine + Send> CanListAComponent for TheComponentWalksReader<'_, R> {
    async fn list(&mut self, component_path: &str) -> Result<(), SubsystemFailure> {
        self.reader
            .read_one_directory(component_path.as_bytes())
            .await
            .map(|_| ())
    }
}

/// **Where one thing on the machine lands, by the same composition every
/// landing path in this crate comes through.**
///
/// The folder half of P11, and P10's job folder where the picked thing was a
/// job's own output folder. **A path that is the root the relative paths are
/// taken from lands at the destination itself**, which is the whole-job case:
/// the job's folder is composed rather than carried.
fn the_landing_place_of<N: TheReceivingSystemsRules + ?Sized>(
    rules: &N,
    at: &[u8],
    from: &[u8],
    the_destination: &[u8],
    the_jobs_folder: Option<&AJobsOwnFolder>,
) -> Vec<u8> {
    let relative = at
        .strip_prefix(from)
        .map(|rest| rest.strip_prefix(b"/".as_ref()).unwrap_or(rest))
        .unwrap_or(at);
    match the_jobs_folder {
        Some(job) => crate::fetching::the_landing_path_inside_the_jobs_folder(
            rules,
            the_destination,
            &job.job,
            job.label.as_deref(),
            relative,
        ),
        None => the_landing_path(the_destination, relative),
    }
}

/// One enumerated file: its landing path (P11), its name check (P4) and its
/// vouching mark (J9).
#[allow(
    clippy::too_many_arguments,
    reason = "one enumerated file needs the adapter, the local records, its \
              own source, the root the relative path is taken from, the \
              destination, the job whose folder it is landing inside where it \
              is landing inside one, and the reading the walk took. Gathering \
              them would make one shape whose only reader is this function"
)]
fn one_entry<N, J>(
    rules: &N,
    records: &J,
    source: &[u8],
    from: &[u8],
    the_destination: &[u8],
    the_jobs_folder: Option<&AJobsOwnFolder>,
    entry: &ADirectoryEntry,
) -> EnumerationEntry
where
    N: TheReceivingSystemsRules + ?Sized,
    J: ReadsTheLocalJobRecords,
{
    let relative = source
        .strip_prefix(from)
        .map(|rest| rest.strip_prefix(b"/".as_ref()).unwrap_or(rest))
        .unwrap_or(source);
    // **M9 and P10: a whole job lands inside the job's own folder**, composed
    // through the one place both routes come through; anything else lands
    // beside the destination as P11 composes it.
    let landing = match the_jobs_folder {
        Some(job) => crate::fetching::the_landing_path_inside_the_jobs_folder(
            rules,
            the_destination,
            &job.job,
            job.label.as_deref(),
            relative,
        ),
        None => the_landing_path(the_destination, relative),
    };
    let mut composed = EnumerationEntry::enumerated(
        source.to_vec(),
        landing.clone(),
        entry.size,
        the_mark(records, source, entry.size),
    );
    // **P4: the check is per component, over the components this document
    // composed** — the destination is the user's own and was written by
    // somebody else, so only what is added to it is judged.
    let components: Vec<&[u8]> = relative.split(|byte| *byte == b'/').collect();
    if let Err((_, refusal)) = judge_the_landing_path(rules, &components) {
        composed.refusal = Some(refusal);
    }
    composed
}

/// **J9's mark, from the local job records alone and with no read of the
/// machine.**
fn the_mark<J: ReadsTheLocalJobRecords>(
    records: &J,
    source: &[u8],
    the_size_the_walk_read: u64,
) -> AVouchingMark {
    the_mark_for(
        &records.what_the_local_record_says(source),
        records.the_files_frame(source),
        the_size_the_walk_read,
    )
}

/// **The folder a path sits in, by its last separator.**
///
/// **The parent of `/x` is `/`**, the top of the server, and never an empty
/// path. **The case this was first written for** was a picked thing below a
/// folder — `/home/ubuntu/scene.blend`, `/jobs/abc` — where the separator is
/// never the first byte. **The case it now also meets** is a file at the top
/// of the server, which on Vast is where everything goes: the login is `root`,
/// Files opens at *Root* and uploads go to `/`. **The record and
/// the world parted on the Vast pass at 12:43 on 2026-10-01**, when *Download*
/// on `/pv-4gb.bin` asked the machine to read an empty path for its parent,
/// the machine answered *no such file*, and the file was recorded as *gone*
/// (step 10c's Vast-pass fix piece 1).
fn parent_of(path: &[u8]) -> Vec<u8> {
    match path.iter().rposition(|byte| *byte == b'/') {
        None => Vec::new(),
        Some(0) => b"/".to_vec(),
        Some(at) => path[..at].to_vec(),
    }
}

fn under(parent: &[u8], child: &[u8]) -> Vec<u8> {
    if parent.is_empty() {
        return child.to_vec();
    }
    let mut out = parent.to_vec();
    if !out.ends_with(b"/") {
        out.push(b'/');
    }
    out.extend_from_slice(child);
    out
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use smoozy_queue_history::ending::Ending;
    use smoozy_server_protocol::subsystem::StatusCode;

    use crate::fetching::TheNameCheckSaid;

    use super::super::walking::TheEntrysKind;
    use super::*;

    #[derive(Default)]
    struct AMachine {
        folders: HashMap<Vec<u8>, Result<Vec<ADirectoryEntry>, SubsystemFailure>>,
    }

    impl AMachine {
        fn holding(mut self, path: &[u8], entries: Vec<ADirectoryEntry>) -> AMachine {
            self.folders.insert(path.to_vec(), Ok(entries));
            self
        }
        fn refusing(mut self, path: &[u8], failure: SubsystemFailure) -> AMachine {
            self.folders.insert(path.to_vec(), Err(failure));
            self
        }
    }

    impl ReadsOneDirectoryOnTheMachine for AMachine {
        async fn read_one_directory(
            &mut self,
            path: &[u8],
        ) -> Result<Vec<ADirectoryEntry>, SubsystemFailure> {
            self.folders
                .get(path)
                .cloned()
                .unwrap_or(Err(SubsystemFailure::TheServerRefused(
                    StatusCode::NoSuchFile,
                )))
        }
    }

    /// **The adapter's own judgement, filled by a test double and by nothing
    /// else in this crate** — the one route to it, and the wall the rules half
    /// already met.
    struct EverythingIsWritable;
    impl TheReceivingSystemsRules for EverythingIsWritable {
        fn check(&self, _component: &[u8]) -> TheNameCheckSaid {
            TheNameCheckSaid::TheNameMayBeWrittenAsGiven
        }
        fn substitute(&self, component: &[u8]) -> Vec<u8> {
            component.to_vec()
        }
        fn length_in_this_systems_own_unit(&self, component: &[u8]) -> usize {
            component.len()
        }
        fn the_longest_component(&self) -> usize {
            255
        }
    }

    struct NoJobIsKnown;
    impl ReadsTheLocalJobRecords for NoJobIsKnown {
        fn what_the_local_record_says(&self, _source_path: &[u8]) -> WhatTheLocalRecordSays {
            WhatTheLocalRecordSays {
                the_file_is_inside_the_jobs_output_folder: false,
                the_recorded_ending: None,
                the_completeness_statement: None,
            }
        }
        fn the_files_frame(&self, _source_path: &[u8]) -> Option<i64> {
            None
        }
        fn the_job_whose_output_folder_this_is(&self, _picked: &[u8]) -> Option<AJobsOwnFolder> {
            None
        }
    }

    struct AStoppedJob;
    impl ReadsTheLocalJobRecords for AStoppedJob {
        fn what_the_local_record_says(&self, _source_path: &[u8]) -> WhatTheLocalRecordSays {
            WhatTheLocalRecordSays {
                the_file_is_inside_the_jobs_output_folder: true,
                the_recorded_ending: Some(Ending::Stopped),
                the_completeness_statement: None,
            }
        }
        fn the_files_frame(&self, _source_path: &[u8]) -> Option<i64> {
            Some(1)
        }
        fn the_job_whose_output_folder_this_is(&self, _picked: &[u8]) -> Option<AJobsOwnFolder> {
            None
        }
    }

    /// A local record that knows one picked path is a job's own output folder,
    /// which is M9's whole-job case.
    struct AWholeJob {
        the_output_folder: Vec<u8>,
        job: Identity,
        label: Option<String>,
    }
    impl ReadsTheLocalJobRecords for AWholeJob {
        fn what_the_local_record_says(&self, _source_path: &[u8]) -> WhatTheLocalRecordSays {
            WhatTheLocalRecordSays {
                the_file_is_inside_the_jobs_output_folder: true,
                the_recorded_ending: Some(Ending::Finished),
                the_completeness_statement: None,
            }
        }
        fn the_files_frame(&self, _source_path: &[u8]) -> Option<i64> {
            Some(1)
        }
        fn the_job_whose_output_folder_this_is(&self, picked: &[u8]) -> Option<AJobsOwnFolder> {
            (picked == self.the_output_folder).then(|| AJobsOwnFolder {
                job: self.job.clone(),
                label: self.label.clone(),
            })
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

    fn file(name: &str, size: u64) -> ADirectoryEntry {
        ADirectoryEntry {
            name: name.as_bytes().to_vec(),
            kind: TheEntrysKind::NotAFolder,
            size,
            modification_time: 1,
        }
    }

    fn folder(name: &str) -> ADirectoryEntry {
        ADirectoryEntry {
            name: name.as_bytes().to_vec(),
            kind: TheEntrysKind::AFolder,
            size: 0,
            modification_time: 1,
        }
    }

    /// **M9 and P11: a downloaded folder arrives as a folder**, and the
    /// structure the files had on the server is the structure they have when
    /// they land.
    #[test]
    fn a_picked_folder_arrives_as_a_folder_with_its_structure_kept() {
        let mut machine = AMachine::default()
            .holding(b"/jobs/abc", vec![folder("layer"), file("a.png", 3)])
            .holding(b"/jobs/abc/layer", vec![file("0001.exr", 9)]);

        let enumeration = run(enumerate(
            &mut machine,
            &EverythingIsWritable,
            &NoJobIsKnown,
            &[b"/jobs/abc".to_vec()],
            b"/Users/a/Downloads",
        ));

        let mut landings: Vec<Vec<u8>> = enumeration
            .entries
            .iter()
            .map(|entry| entry.landing_path.clone())
            .collect();
        landings.sort();
        assert_eq!(
            landings,
            vec![
                b"/Users/a/Downloads/abc/a.png".to_vec(),
                b"/Users/a/Downloads/abc/layer/0001.exr".to_vec(),
            ]
        );
        assert!(enumeration.refusals.is_empty());
    }

    /// **Test 4 of the *Show hidden files* plan text, the download half: a
    /// folder downloaded takes its hidden files and folders** — `File Browser`
    /// `File Browser` L21: the choice changes only what the browser's list shows, and a
    /// download started there is this module's, with its own walk, which reads
    /// the machine whole.
    #[test]
    fn a_downloaded_folder_takes_its_hidden_files() {
        let mut machine = AMachine::default()
            .holding(b"/jobs/abc", vec![folder(".cache"), file(".env", 2), file("a.png", 3)])
            .holding(b"/jobs/abc/.cache", vec![file("bake.bin", 9)]);

        let enumeration = run(enumerate(
            &mut machine,
            &EverythingIsWritable,
            &NoJobIsKnown,
            &[b"/jobs/abc".to_vec()],
            b"/Users/a/Downloads",
        ));

        let mut landings: Vec<Vec<u8>> = enumeration
            .entries
            .iter()
            .map(|entry| entry.landing_path.clone())
            .collect();
        landings.sort();
        assert_eq!(
            landings,
            vec![
                b"/Users/a/Downloads/abc/.cache/bake.bin".to_vec(),
                b"/Users/a/Downloads/abc/.env".to_vec(),
                b"/Users/a/Downloads/abc/a.png".to_vec(),
            ]
        );
        assert!(enumeration.refusals.is_empty());
    }

    /// A whole job whose folder holds two copies Smoozy placed before the
    /// render, one untouched and one the render wrote over.
    struct AJobHoldingSmoozysCopies {
        untouched: Vec<u8>,
    }
    impl ReadsTheLocalJobRecords for AJobHoldingSmoozysCopies {
        fn what_the_local_record_says(&self, _source_path: &[u8]) -> WhatTheLocalRecordSays {
            NoJobIsKnown.what_the_local_record_says(_source_path)
        }
        fn the_files_frame(&self, _source_path: &[u8]) -> Option<i64> {
            None
        }
        fn the_job_whose_output_folder_this_is(&self, _picked: &[u8]) -> Option<AJobsOwnFolder> {
            None
        }
        fn is_a_copy_smoozy_placed(&self, source_path: &[u8]) -> bool {
            source_path == self.untouched
        }
    }

    /// **The fixes before 0.1.8, fault 3: *Download all* on a job brings home
    /// the job's output and nothing of Smoozy's own** — not a copy Smoozy
    /// placed and nothing touched (`Download` N5a), and not a staging folder,
    /// nor any folder for it to land in. **A placed file the render wrote over
    /// is the render's and comes home.**
    ///
    /// **The case it bites on**: the enumeration walked the job's folder whole,
    /// so the check before 0.1.8 brought home 2.8 GB of the user's own baked
    /// cache, as Smoozy had copied it in, and the empty staging folders with it.
    #[test]
    fn a_whole_jobs_download_leaves_smoozys_own_copies_and_staging_folders_behind() {
        let staging = smoozy_file_browser::partial::THE_RESERVED_STAGING_FOLDER;
        let mut machine = AMachine::default()
            .holding(
                b"/jobs/abc",
                vec![folder("pointcache"), folder(staging), file("f_0001.png", 3)],
            )
            .holding(
                b"/jobs/abc/pointcache",
                vec![folder(staging), file("c_000001.bphys", 9), file("c_000002.bphys", 9)],
            )
            .holding(b"/jobs/abc/pointcache/.smoozy-staging", vec![file("abc-1.smoozy-part", 4)])
            .holding(b"/jobs/abc/.smoozy-staging", Vec::new());

        let enumeration = run(enumerate(
            &mut machine,
            &EverythingIsWritable,
            &AJobHoldingSmoozysCopies {
                untouched: b"/jobs/abc/pointcache/c_000001.bphys".to_vec(),
            },
            &[b"/jobs/abc".to_vec()],
            b"/Users/a/Downloads",
        ));

        let mut landings: Vec<Vec<u8>> = enumeration
            .entries
            .iter()
            .map(|entry| entry.landing_path.clone())
            .collect();
        landings.sort();
        assert_eq!(
            landings,
            vec![
                b"/Users/a/Downloads/abc/f_0001.png".to_vec(),
                b"/Users/a/Downloads/abc/pointcache/c_000002.bphys".to_vec(),
            ]
        );
        assert!(
            !enumeration
                .landing_folders
                .iter()
                .any(|folder| folder.ends_with(staging.as_bytes())),
            "a staging folder would be made on the user's computer: {:?}",
            enumeration.landing_folders
        );
        assert!(enumeration.refusals.is_empty());
    }

    /// **A folder that held only Smoozy's own copies is not made at home** —
    /// after 0.1.8, the checks' leftovers, item 4. The re-check before 0.1.8
    /// found *Download all* skipping every copy in a job's `pointcache/` and
    /// still making the folder, empty, in the job's folder at home. **M9's
    /// empty folder is the machine's own empty folder**, which still arrives;
    /// a folder whose every file was a copy N5a leaves behind, with nothing
    /// else below it, is not one.
    #[test]
    fn a_folder_holding_only_smoozys_copies_is_not_made_at_home() {
        struct EveryCacheFileIsACopy;
        impl ReadsTheLocalJobRecords for EveryCacheFileIsACopy {
            fn what_the_local_record_says(&self, source_path: &[u8]) -> WhatTheLocalRecordSays {
                NoJobIsKnown.what_the_local_record_says(source_path)
            }
            fn the_files_frame(&self, _source_path: &[u8]) -> Option<i64> {
                None
            }
            fn the_job_whose_output_folder_this_is(&self, _picked: &[u8]) -> Option<AJobsOwnFolder> {
                None
            }
            fn is_a_copy_smoozy_placed(&self, source_path: &[u8]) -> bool {
                source_path.ends_with(b".bphys")
            }
        }
        let mut machine = AMachine::default()
            .holding(
                b"/jobs/abc",
                vec![folder("pointcache"), folder("empty"), file("f_0001.png", 3)],
            )
            .holding(
                b"/jobs/abc/pointcache",
                vec![folder("deeper"), file("c_000001.bphys", 9), file("c_000002.bphys", 9)],
            )
            .holding(b"/jobs/abc/pointcache/deeper", vec![file("d_000001.bphys", 9)])
            .holding(b"/jobs/abc/empty", Vec::new());

        let enumeration = run(enumerate(
            &mut machine,
            &EverythingIsWritable,
            &EveryCacheFileIsACopy,
            &[b"/jobs/abc".to_vec()],
            b"/Users/a/Downloads",
        ));

        let mut folders = enumeration.landing_folders.clone();
        folders.sort();
        assert_eq!(
            folders,
            vec![
                b"/Users/a/Downloads/abc".to_vec(),
                b"/Users/a/Downloads/abc/empty".to_vec(),
            ],
            "{:?}",
            folders
                .iter()
                .map(|it| String::from_utf8_lossy(it).to_string())
                .collect::<Vec<_>>()
        );
        assert_eq!(enumeration.entries.len(), 1);

        // **A folder the user picked still arrives as a folder** (M9), even
        // where it held only copies — it is not read as nothing picked.
        let picked_alone = run(enumerate(
            &mut machine,
            &EverythingIsWritable,
            &EveryCacheFileIsACopy,
            &[b"/jobs/abc/pointcache".to_vec()],
            b"/Users/a/Downloads",
        ));
        assert_eq!(
            picked_alone.landing_folders,
            vec![b"/Users/a/Downloads/pointcache".to_vec()],
            "{:?}",
            picked_alone
                .landing_folders
                .iter()
                .map(|it| String::from_utf8_lossy(it).to_string())
                .collect::<Vec<_>>()
        );
        assert!(picked_alone.entries.is_empty());
    }

    /// **M9, P10 and H5a: a whole-job download the user starts by hand lands
    /// on that job's own folder, under P10's composed name.**
    ///
    /// The spec's promise is *"a whole job's outputs come home inside the
    /// job's own folder, whichever way they were asked for"*, so **the
    /// destination is the folder that contains the job's folder** and the
    /// folder's name is Smoozy's composition rather than the machine's last
    /// path component.
    ///
    /// **H5a's arithmetic is asserted with it**: the landing path relative to
    /// the destination is one component longer than the source path relative
    /// to the job's output folder, which is what tells this case from a folder
    /// picked inside a job.
    #[test]
    fn a_whole_job_picked_by_hand_lands_inside_the_jobs_own_folder_under_p10s_name() {
        let mut machine = AMachine::default()
            .holding(b"/jobs/abc night", vec![folder("beauty"), file("a.png", 3)])
            .holding(b"/jobs/abc night/beauty", vec![file("0001.exr", 9)]);
        let records = AWholeJob {
            the_output_folder: b"/jobs/abc night".to_vec(),
            job: Identity::issue(smoozy_state::identities::IdentityKind::Job)
                .expect("the desk has randomness"),
            label: Some("night".into()),
        };
        let composed = crate::fetching::the_job_folders_landing_name(
            &EverythingIsWritable,
            &records.job,
            Some("night"),
        );

        let enumeration = run(enumerate(
            &mut machine,
            &EverythingIsWritable,
            &records,
            &[b"/jobs/abc night".to_vec()],
            b"/Users/a/Downloads",
        ));

        let mut landings: Vec<Vec<u8>> = enumeration
            .entries
            .iter()
            .map(|entry| entry.landing_path.clone())
            .collect();
        landings.sort();
        let under = |rest: &str| {
            let mut out = b"/Users/a/Downloads/".to_vec();
            out.extend_from_slice(&composed);
            out.push(b'/');
            out.extend_from_slice(rest.as_bytes());
            out
        };
        assert_eq!(landings, vec![under("a.png"), under("beauty/0001.exr")]);
        // **The machine's own folder name is nowhere in the landing paths.**
        assert!(!landings
            .iter()
            .any(|path| path.windows(9).any(|window| window == b"abc night")
                && !path
                    .windows(composed.len())
                    .any(|window| window == composed)));

        // H5a's count, read off one entry: two components in the source below
        // the job's output folder, three below the destination.
        let entry = enumeration
            .entries
            .iter()
            .find(|entry| entry.source_path == b"/jobs/abc night/beauty/0001.exr")
            .expect("the deep file is enumerated");
        let components = |path: &[u8], under: &[u8]| {
            path.split(|byte| *byte == b'/')
                .filter(|component| !component.is_empty())
                .count()
                - under
                    .split(|byte| *byte == b'/')
                    .filter(|component| !component.is_empty())
                    .count()
        };
        assert_eq!(
            components(&entry.landing_path, b"/Users/a/Downloads"),
            components(&entry.source_path, b"/jobs/abc night") + 1
        );
    }

    /// **A folder picked *inside* a job's output folder is not a whole job**,
    /// so no job folder is composed and it lands beside the destination as
    /// P11 composes it.
    #[test]
    fn a_folder_picked_inside_a_job_is_not_a_whole_job_and_gets_no_job_folder() {
        let mut machine =
            AMachine::default().holding(b"/jobs/abc night/beauty", vec![file("0001.exr", 9)]);
        let records = AWholeJob {
            the_output_folder: b"/jobs/abc night".to_vec(),
            job: Identity::issue(smoozy_state::identities::IdentityKind::Job)
                .expect("the desk has randomness"),
            label: Some("night".into()),
        };

        let enumeration = run(enumerate(
            &mut machine,
            &EverythingIsWritable,
            &records,
            &[b"/jobs/abc night/beauty".to_vec()],
            b"/Users/a/Downloads",
        ));

        assert_eq!(
            enumeration
                .entries
                .iter()
                .map(|entry| entry.landing_path.clone())
                .collect::<Vec<Vec<u8>>>(),
            vec![b"/Users/a/Downloads/beauty/0001.exr".to_vec()]
        );
    }

    /// **M9 and ruling 6: a picked folder with no files in it still arrives as
    /// a folder.**
    ///
    /// **An empty folder has no transfer to make one for it**, so without the
    /// landing folder the request would complete having put nothing on the
    /// user's disk — a user who asks for an empty folder gets an empty folder.
    #[test]
    fn an_empty_picked_folder_still_puts_its_folder_on_the_users_disk() {
        #[derive(Default)]
        struct ADisk {
            made: Vec<Vec<u8>>,
        }
        impl MakesTheLandingFolder for ADisk {
            fn create_the_landing_folder(
                &mut self,
                the_landing_folder: &[u8],
            ) -> Result<(), crate::destination::WhatTheAdapterSaid> {
                self.made.push(the_landing_folder.to_vec());
                Ok(())
            }
        }

        let mut machine = AMachine::default().holding(b"/jobs/empty", Vec::new());
        let enumeration = run(enumerate(
            &mut machine,
            &EverythingIsWritable,
            &NoJobIsKnown,
            &[b"/jobs/empty".to_vec()],
            b"/Users/a/Downloads",
        ));

        assert!(enumeration.entries.is_empty());
        assert!(enumeration.refusals.is_empty());
        assert_eq!(
            enumeration.landing_folders,
            vec![b"/Users/a/Downloads/empty".to_vec()]
        );

        let mut disk = ADisk::default();
        let made = make_the_landing_folders(&mut disk, &enumeration);
        assert_eq!(disk.made, vec![b"/Users/a/Downloads/empty".to_vec()]);
        assert!(made.iter().all(|(_, outcome)| outcome.is_ok()));
    }

    /// **A folder inside a picked folder arrives as a folder too**, which is
    /// M9's own rule and what keeps a job's sub-folders on the user's disk
    /// where they hold nothing.
    #[test]
    fn every_folder_the_walk_descends_into_is_a_landing_folder() {
        let mut machine = AMachine::default()
            .holding(b"/jobs/abc", vec![folder("beauty"), folder("mist")])
            .holding(b"/jobs/abc/beauty", vec![file("0001.exr", 9)])
            .holding(b"/jobs/abc/mist", Vec::new());

        let enumeration = run(enumerate(
            &mut machine,
            &EverythingIsWritable,
            &NoJobIsKnown,
            &[b"/jobs/abc".to_vec()],
            b"/Users/a/Downloads",
        ));

        let mut folders = enumeration.landing_folders.clone();
        folders.sort();
        assert_eq!(
            folders,
            vec![
                b"/Users/a/Downloads/abc".to_vec(),
                b"/Users/a/Downloads/abc/beauty".to_vec(),
                b"/Users/a/Downloads/abc/mist".to_vec(),
            ]
        );
    }

    /// **M4: a refused branch ends with a recorded refusal and the
    /// enumeration is still complete** — the readable set is enumerated, the
    /// refusal is reported, and no count is wrong in the quiet direction.
    ///
    /// **This is the opposite of §3's walk and both are right**, which the
    /// module header states.
    #[test]
    fn a_refused_branch_is_recorded_and_the_rest_of_the_folder_is_still_enumerated() {
        let mut machine = AMachine::default()
            .holding(b"/jobs/abc", vec![folder("locked"), file("a.png", 3)])
            .refusing(
                b"/jobs/abc/locked",
                SubsystemFailure::TheServerRefused(StatusCode::PermissionDenied),
            );

        let enumeration = run(enumerate(
            &mut machine,
            &EverythingIsWritable,
            &NoJobIsKnown,
            &[b"/jobs/abc".to_vec()],
            b"/Users/a/Downloads",
        ));

        assert_eq!(enumeration.entries.len(), 1);
        assert_eq!(enumeration.refusals.len(), 1);
        assert_eq!(
            enumeration.refusals[0].what_smoozy_met,
            WhatSmoozyMet::Refused
        );
        assert!(crate::request::the_enumeration_is_complete(true));
    }

    /// **A picked file is one entry** — the reading of it as a folder refuses,
    /// and the answer is its own parent's listing rather than a recorded
    /// refusal.
    #[test]
    fn a_picked_file_is_one_entry_and_not_a_refused_branch() {
        let mut machine = AMachine::default().holding(b"/jobs/abc", vec![file("a.png", 3)]);

        let enumeration = run(enumerate(
            &mut machine,
            &EverythingIsWritable,
            &NoJobIsKnown,
            &[b"/jobs/abc/a.png".to_vec()],
            b"/Users/a/Downloads",
        ));

        assert!(enumeration.refusals.is_empty());
        assert_eq!(enumeration.entries.len(), 1);
        assert_eq!(
            enumeration.entries[0].landing_path,
            b"/Users/a/Downloads/a.png".to_vec()
        );
    }

    /// **A picked file at the top of the server is one entry**, found by asking
    /// `/` for it — step 10c's Vast-pass fix piece 1. Its parent was taken as
    /// an empty path, which the machine reads as *no such file*, so the file
    /// was recorded as *gone*.
    #[test]
    fn a_picked_file_at_the_top_of_the_server_is_one_entry() {
        let mut machine = AMachine::default().holding(b"/", vec![file("pv-4gb.bin", 3)]);

        let enumeration = run(enumerate(
            &mut machine,
            &EverythingIsWritable,
            &NoJobIsKnown,
            &[b"/pv-4gb.bin".to_vec()],
            b"/Users/a/Downloads",
        ));

        assert!(enumeration.refusals.is_empty(), "{:?}", enumeration.refusals);
        assert_eq!(enumeration.entries.len(), 1);
        assert_eq!(enumeration.entries[0].source_path, b"/pv-4gb.bin");
        assert_eq!(
            enumeration.entries[0].landing_path,
            b"/Users/a/Downloads/pv-4gb.bin".to_vec()
        );
    }

    /// **J9 at the enumeration, from the local job records alone**: a job whose
    /// recorded ending is *stopped* marks its output folder's files, and no
    /// machine is read for it.
    #[test]
    fn the_vouching_mark_is_made_at_the_enumeration_from_the_local_records() {
        let mut machine = AMachine::default().holding(b"/jobs/abc", vec![file("a.png", 3)]);

        let enumeration = run(enumerate(
            &mut machine,
            &EverythingIsWritable,
            &AStoppedJob,
            &[b"/jobs/abc".to_vec()],
            b"/Users/a/Downloads",
        ));

        assert_eq!(
            enumeration.entries[0].vouching,
            AVouchingMark::CannotBeVouchedFor
        );
    }

    /// **P4 runs at enumeration for a manual request**, and a refused name is
    /// carried as *cannot land* rather than repaired.
    #[test]
    fn a_name_the_receiving_system_refuses_is_carried_as_a_refusal_and_never_repaired() {
        struct NothingMayBeWritten;
        impl TheReceivingSystemsRules for NothingMayBeWritten {
            fn check(&self, _component: &[u8]) -> TheNameCheckSaid {
                TheNameCheckSaid::NotRepresentable
            }
            fn substitute(&self, component: &[u8]) -> Vec<u8> {
                component.to_vec()
            }
            fn length_in_this_systems_own_unit(&self, component: &[u8]) -> usize {
                component.len()
            }
            fn the_longest_component(&self) -> usize {
                255
            }
        }

        let mut machine = AMachine::default().holding(b"/jobs/abc", vec![file("a.png", 3)]);
        let enumeration = run(enumerate(
            &mut machine,
            &NothingMayBeWritten,
            &NoJobIsKnown,
            &[b"/jobs/abc".to_vec()],
            b"/Users/a/Downloads",
        ));

        assert_eq!(
            enumeration.entries[0].refusal,
            Some(crate::fetching::WhichRefusalItWas::NotRepresentable)
        );
        // **The name is untouched**: the source path is still the machine's
        // own bytes and the landing path is still what P11 composed.
        assert_eq!(enumeration.entries[0].source_path, b"/jobs/abc/a.png");
    }
}
