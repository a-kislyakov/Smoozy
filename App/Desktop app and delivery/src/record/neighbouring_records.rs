//! The neighbouring records `File Browser` §8 reads and its X7a writes into —
//! `Desktop app and delivery` §5, and that document's two seams.
//!
//! **The shape is that document's and the records are three other documents'.**
//! Nothing here decides what a warning names or what a publication owes: the
//! judgements are `File Browser`'s, at its `warnings` and `publication`, and
//! what this file does is answer the two traits they read and write through.
//!
//! **Why the adapter is here.** `File Browser` cannot name the crates that hold
//! these records: `Dependency Upload & Remap` is built on `Render Settings`,
//! which is built on `Server Setup`, which is built on `File Browser` for that
//! module's own `File Browser` P5 elevation readings — so a dependency in that direction is a cycle the
//! build refuses. This crate sits above all of them and already answers that
//! module's `KeepsTheRequest` seam, and **A2 puts every module's store here
//! anyway**: no module has a durable store of its own.
//!
//! **No record shape is copied.** Every answer below is composed from
//! `Dependency Upload & Remap`'s own record trait, `Queue & History`'s own job
//! records and `Render Settings`' own reading store, through their own
//! operations. **A second copy of any of them would be the drift `State,
//! identity and recovery` §1.1 exists against.**
//!
//! **Nothing here reaches a machine**, which is `File Browser` W1's own rule:
//! the readings are of the desktop's own records and both writes land on the
//! desktop.
//!
//! **How a clause marker is written in this crate, and it is a rule rather than
//! a habit.** A marker with no document beside it means a clause of this
//! crate's own plan, `Preparation/Architecture/Desktop app and delivery.md`.
//! Every other document's marker is written with that document — at the site,
//! or in this file's own header list below.
//!
//! **Markers used below that belong to another document, each with the document
//! it belongs to.** `File Browser` W1a, W1b, W11, X7a. `Dependency Upload &
//! Remap` V13a, V2a. `Queue & History` N11, N21. `Render Settings` J10,
//! J2, Q14.
//!
//! **A marker this crate's own plan also defines is deliberately not listed
//! above, whichever document is meant, and is written out with its document at
//! every site instead** — the header list cannot tell the two apart, and a bare
//! one would resolve to a real and wrong clause. **This crate's own plan
//! defines a W1, a W5, a W6, an X5, a P5, a Q2, a V1, a V2, a V5, a V10, a K7
//! and a K8 of its own**, so `File Browser`'s warning clauses and
//! `Dependency Upload & Remap`'s record kinds are written out where they are
//! cited.

use smoozy_dependencies::bytes::MachineLocation;
use smoozy_dependencies::knowledge::what_points_at;
use smoozy_dependencies::record::{MachineKey, TheRecord};
use smoozy_dependencies::standing::{
    roots_whose_tree_resolved, CheckStanding, WhereFilesSaysItCameFrom,
};
use smoozy_file_browser::already_exists::{is_inside, paths_are_equal};
use smoozy_file_browser::publication::{
    AFixedCopyMadeFromIt, ARootWhoseTreeResolvedIt, TheNeighbouringRecords,
};
use smoozy_file_browser::warnings::{
    ALiveJob, HowTheJobStands, ReadsWhatTheLocalRecordKnows, WhatDependedOnIt,
    WhatTheLandingPathIsFor,
};
use smoozy_job_files::layout::TheLayout;
use smoozy_queue_history::line::{JobRecord, QueueState};
use smoozy_queue_history::record::LocalJobRecord;
use smoozy_render_settings::file_half::{FileHalf, FileKey, WhatDroppedIt};

/// **`File Browser` W1's readings, answered from the local record for one
/// machine.**
///
/// **One value is one machine**, which is what `File Browser` W1b's
/// installation limit rides on: [`MachineKey`] carries the server row and the
/// installation identity the records must stand under, so a record left by a
/// machine that has been recycled matches nothing here and the class comes back
/// *neither*.
pub struct WhatTheLocalRecordKnows<'a> {
    /// `Dependency Upload & Remap`'s own record, whatever holds it.
    pub record: &'a dyn TheRecord,
    /// The server row and the installation identity in force for it.
    pub machine: MachineKey,
    /// `Job files and outputs`' layout for this machine, which is where a job
    /// folder's path comes from.
    pub layout: &'a TheLayout,
    /// `Queue & History` Q2's job records for this machine — what says which
    /// jobs are live.
    pub queue: &'a [JobRecord],
    /// The local job records beside them, which carry the label, the `.blend`
    /// path (`Queue & History` N11) and the disturbance marks (its N21).
    pub entries: &'a [LocalJobRecord],
}

impl ReadsWhatTheLocalRecordKnows for WhatTheLocalRecordKnows<'_> {
    /// `Dependency Upload & Remap` V2's points-at reading, taken through that
    /// module's own function rather than re-derived.
    fn what_depended_on(&self, path: &[u8]) -> WhatDependedOnIt {
        let points = what_points_at(
            &MachineLocation::composed(path.to_vec()),
            &self.machine,
            &self.machine.installation,
            self.record,
        );
        WhatDependedOnIt {
            fixed_copies_whose_fixing_points_at_it: points
                .fixed_copies_whose_fixing_points_at_it
                .iter()
                .map(|at| at.as_bytes().to_vec())
                .collect(),
            placed_from: points
                .placed_from
                .iter()
                .map(|from| from.as_bytes().to_vec())
                .collect(),
            jobs_whose_set_names_it: points.jobs_whose_set_names_it,
        }
    }

    /// **`File Browser` W1's reading for a folder victim**: the machine
    /// locations this machine's records name **underneath** the path, each with
    /// `Dependency Upload & Remap` V2's answer for it.
    ///
    /// **The candidates are the record's own locations and nothing is
    /// enumerated from a machine** — the mirrors its placement records name,
    /// the fixed copies and derived-from copies its fixing records name, and
    /// the `.blend`, the resolved locations and the remap material its live
    /// jobs name. **Nothing is listed, nothing is connected to, and the
    /// folder's real contents are never read**, which is `File Browser` W1's
    /// one limit.
    ///
    /// **The containment test is that module's own** —
    /// [`is_inside`], per path component under its X5 rule — and the folder's
    /// own location is left out, the victim's own reading being asked
    /// separately.
    ///
    /// **A record naming a location is not evidence a file is there**, and this
    /// reading claims none: it answers what the record holds, which is what
    /// this warning names.
    fn what_the_record_holds_under(&self, folder: &[u8]) -> Vec<(Vec<u8>, WhatDependedOnIt)> {
        let mut under: Vec<Vec<u8>> = Vec::new();
        let mut consider = |at: &MachineLocation| {
            let bytes = at.as_bytes().to_vec();
            if !is_inside(&bytes, folder) || paths_are_equal(&bytes, folder) {
                return;
            }
            if !under.iter().any(|already| paths_are_equal(already, &bytes)) {
                under.push(bytes);
            }
        };
        for placement in self.record.placements_on(&self.machine) {
            consider(&placement.mirrored_location);
        }
        for fixing in self.record.fixings_on(&self.machine) {
            consider(&fixing.fixed_copy);
            consider(&fixing.derived_from);
        }
        for job in self.record.live_or_waiting_jobs_on(&self.machine) {
            consider(&job.blend);
            for at in &job.resolved_set.locations {
                consider(at);
            }
            consider(&job.resolved_set.remap_material);
        }
        under
            .into_iter()
            .filter_map(|at| {
                let what = self.what_depended_on(&at);
                what.anything_depended_on_it().then_some((at, what))
            })
            .collect()
    }

    /// **`File Browser` W1a's three-way class, read off two of that module's
    /// record kinds and off nothing `File Browser` keeps** — its placement
    /// record (`Dependency Upload & Remap` K7) and its fixing record (that
    /// module's own `Dependency Upload & Remap` K8).
    ///
    /// **The order the two are asked in decides nothing**, which is what W1a
    /// says in its own words: the class is a property of what the machine
    /// location is for, and a mirror and a fixed copy are different addresses,
    /// so no location answers both.
    ///
    /// **`File Browser` W1b's disqualification is the record's own keying and
    /// is not re-checked here.** Both questions are asked of one
    /// [`MachineKey`], which carries the server row *and* the installation
    /// identity, so a record left by a machine that has since been recycled
    /// matches no key and never reaches this code. **A second check beside it
    /// was written first and taken out**: it could not be made to fail, because
    /// there is no value it could reject that the key had not already rejected,
    /// and a guard that cannot fail reads as the guard when it is not.
    fn what_the_landing_path_is_for(&self, landing_path: &[u8]) -> WhatTheLandingPathIsFor {
        let at = MachineLocation::composed(landing_path.to_vec());
        let placed = self
            .record
            .placements_on(&self.machine)
            .into_iter()
            .any(|placement| placement.mirrored_location == at);
        if placed {
            return WhatTheLandingPathIsFor::Placed;
        }
        let fixed = self
            .record
            .fixings_on(&self.machine)
            .into_iter()
            .any(|fixing| fixing.fixed_copy == at);
        if fixed {
            return WhatTheLandingPathIsFor::Fixed;
        }
        WhatTheLandingPathIsFor::Neither
    }

    /// **`File Browser` W5's live set, composed from `Queue & History` Q2's
    /// records and `Dependency Upload & Remap` V5's per-job answer.**
    ///
    /// A job is live while its queue record is *waiting*, *held* or
    /// *rendering*; a record whose outcome is written is none of those and is
    /// left out here rather than filtered later.
    ///
    /// **A job with no dependency answer keeps its `.blend` and its folder**,
    /// which is `File Browser` W6's widest case: the resolved set comes back
    /// absent rather than empty, so the difference between *no set* and *an
    /// empty set* survives the seam.
    ///
    /// **A job whose `.blend` cannot be named, or whose folder cannot be
    /// composed, is narrowed and not dropped.** `File Browser` W6's answer to
    /// missing knowledge is to narrow the set and say so; dropping the job
    /// would narrow it to nothing and say nothing, and the job's folder is
    /// known independently of its `.blend` while its resolved set may be known
    /// too. **A job is left out only where neither the `.blend` nor the folder
    /// can be named**, which is the one case where there is nothing to say, and
    /// its resolved set alone is no basis for a warning naming a job whose own
    /// two members are both unknown.
    fn the_live_jobs(&self) -> Vec<ALiveJob> {
        let known = self.record.live_or_waiting_jobs_on(&self.machine);
        let mut out = Vec::new();
        for entry in self.entries {
            let Some(queued) = self.queue.iter().find(|record| record.job == entry.job) else {
                continue;
            };
            let stands = match queued.state {
                QueueState::Waiting => HowTheJobStands::Waiting,
                QueueState::Held { .. } => HowTheJobStands::Held,
                QueueState::Rendering { .. } => HowTheJobStands::Rendering,
                QueueState::Ended { .. } => continue,
            };
            let its_answer = known.iter().find(|job| job.job == entry.job);
            let blend = its_answer
                .map(|job| job.blend.as_bytes().to_vec())
                .or_else(|| {
                    entry
                        .blend_path_on_the_machine
                        .as_ref()
                        .map(|path| path.as_bytes().to_vec())
                });
            // **The job's folder as the local record holds it** — the one the
            // machine chose where it chose at render start, and nothing until
            // it has (`Job files and outputs` P21) — never composed a second
            // time here. The Creator's second notes, piece 4.
            let job_folder = entry.the_job_output_folder(self.layout).map(String::into_bytes);
            if blend.is_none() && job_folder.is_none() {
                continue;
            }
            // **`File Browser` W6: a job whose file was never checked has no
            // resolved set**, though every submission writes `Dependency
            // Upload & Remap` K9's set, holding only the remap material where
            // nothing was checked. So the job is W6's widest case, and its
            // warning says Smoozy has only the `.blend` and the job's own
            // folder. After 0.1.8, the checks' leftovers, item 2.
            let never_checked = entry.dependency_check
                == Some(smoozy_queue_history::record::DependencyCheckAtSubmission::NoStandingAtAll);
            let resolved_set = its_answer.filter(|_| !never_checked).map(|job| {
                let mut members: Vec<Vec<u8>> = job
                    .resolved_set
                    .locations
                    .iter()
                    .map(|at| at.as_bytes().to_vec())
                    .collect();
                members.push(job.resolved_set.remap_material.as_bytes().to_vec());
                members
            });
            let its_blend_was_disturbed = blend.as_ref().is_some_and(|blend| {
                entry
                    .disturbances
                    .iter()
                    .any(|disturbance| disturbance.the_member.as_bytes() == blend)
            });
            out.push(ALiveJob {
                job: entry.job.clone(),
                stands,
                blend,
                job_folder,
                resolved_set,
                its_blend_was_disturbed,
            });
        }
        out
    }
}

/// **What the local record holds for one machine, read once per question** —
/// the one composition every warning box in Files reads, so the upload's
/// already-exists question, the delete box, the rename and move box and the
/// dependency check's own overwrite box all read the same thing. Step 10c's
/// Files' owed acts piece 1.
///
/// **The case [`WhatTheLocalRecordKnows`] was first written for, and the case
/// it now meets.** It was written at dependency wave piece 4 and composed by
/// nothing outside its tests: every box in the running application was built
/// from nothing, so an upload's *Replace* over a texture a rendering job reads
/// warned nobody (`File Browser` W1, W1b, W5). **This is where the reader's
/// four sources are read** —
///
/// - `Dependency Upload & Remap`'s own record, through its keeper's snapshot;
/// - the local job records with no learned ending
///   ([`super::job_records::JobRecords::with_no_learned_ending_for`]);
/// - the server row's recorded layout, which is the caller's;
/// - the machine's queue reading the run already holds (`Queue & History`
///   C6), **read and never changed**.
///
/// **Nothing here reaches a machine**, which is `File Browser` W1's rule.
pub struct WhatTheLocalRecordHolds {
    pub record: smoozy_dependencies::record::InMemoryRecord,
    pub machine: MachineKey,
    pub layout: TheLayout,
    /// `Queue & History` Q2's records as this composition reads them — see
    /// [`the_line_as_the_record_reads_it`].
    pub queue: Vec<JobRecord>,
    /// The local job records for this server with no learned ending.
    pub entries: Vec<LocalJobRecord>,
}

impl WhatTheLocalRecordHolds {
    /// **Read once, for one question.** A record that will not answer is said
    /// to — the caller composes no warning rather than reading an unread
    /// record as *nothing depended on it* (`Desktop app and delivery` L8a).
    pub fn read(
        connection: &rusqlite::Connection,
        machine: &MachineKey,
        layout: TheLayout,
        the_machines_reading: Option<&smoozy_queue_history::acting::adoption::TheMachinesWorkingState>,
    ) -> Result<WhatTheLocalRecordHolds, super::RecordFailure> {
        let record = super::dependency_records::TheDependencyRecord::over(connection)
            .what_it_holds_for(machine)?;
        let entries = super::job_records::JobRecords::over(connection)
            .with_no_learned_ending_for(&machine.server_row)?;
        let queue = the_line_as_the_record_reads_it(&entries, the_machines_reading);
        Ok(WhatTheLocalRecordHolds {
            record,
            machine: machine.clone(),
            layout,
            queue,
            entries,
        })
    }

    /// The reader `File Browser`'s warnings take.
    pub fn knows(&self) -> WhatTheLocalRecordKnows<'_> {
        WhatTheLocalRecordKnows {
            record: &self.record,
            machine: self.machine.clone(),
            layout: &self.layout,
            queue: &self.queue,
            entries: &self.entries,
        }
    }

    /// **Each job's name, as Queue and History draw it** — the job identity's
    /// text beside the entry's own name, which is what the box's composition
    /// takes.
    pub fn the_labels(&self) -> Vec<(String, String)> {
        self.entries
            .iter()
            .map(|entry| {
                (
                    entry.job.as_text(),
                    smoozy_queue_history::naming::what_a_job_is_called(
                        entry.label.as_deref(),
                        entry.blend_path_on_the_machine.as_deref(),
                    ),
                )
            })
            .collect()
    }
}

/// **`Queue & History` Q2's live set, as the local record and the held reading
/// say it together.**
///
/// **Where the run holds a reading naming the job, its state is the reading's**
/// — so a job the machine says is rendering is told apart from one waiting
/// (`File Browser` W5a), and a job the machine says has ended is not live
/// although the local record has not learned its ending yet. **Where no
/// reading names it — none taken this period, or a job submitted after the
/// reading — it reads *waiting***. That is the side `Dependency Upload &
/// Remap`'s own reading of live jobs already parts on
/// ([`super::dependency_records::TheDependencyRecord::the_live_or_waiting_jobs_on`]):
/// naming one job too many costs a line on a warning, and missing one lets a
/// live job's file be touched unwarned (`File Browser` W5b). **A job with no
/// local record is not live**: nothing tracks it.
pub fn the_line_as_the_record_reads_it(
    entries: &[LocalJobRecord],
    the_machines_reading: Option<&smoozy_queue_history::acting::adoption::TheMachinesWorkingState>,
) -> Vec<JobRecord> {
    entries
        .iter()
        .map(|entry| {
            let state = the_machines_reading
                .and_then(|reading| reading.jobs.iter().find(|job| job.job == entry.job))
                .map(|job| job.state.clone())
                .unwrap_or(QueueState::Waiting);
            JobRecord {
                job: entry.job.clone(),
                group: entry.group.clone(),
                sequence_key: None,
                state,
                record_directory: String::new(),
                removed_build: None,
                reporting_choice_now: None,
            }
        })
        .collect()
}

/// **`File Browser` X7a's two outward writes and its `File Browser` W11's
/// third, performed on the records they belong to.**
///
/// **The comparison is not made here.** Every reading hands the recorded path
/// text back, and that module applies its own `File Browser` X5 rule to it
/// before any write —
/// so a match made loosely on this side is caught there rather than marking
/// the wrong root.
pub struct TheNeighbours<'a> {
    /// The server row and the installation identity in force for it.
    pub machine: MachineKey,
    /// `Dependency Upload & Remap`'s check standings for this machine, its
    /// `Dependency Upload & Remap` V10 records. **The mark is written through
    /// that module's own
    /// operation**, so nothing here knows what the mark means.
    pub standings: &'a mut Vec<CheckStanding>,
    /// `Render Settings`' recorded readings — its own store, which its J2
    /// makes process memory for the life of the run.
    pub readings: &'a mut FileHalf,
    /// `Dependency Upload & Remap`'s own record, which its V2 and V2a readings
    /// are taken from. **`File Browser` W11's write needs them and X7a's two
    /// did not**, which is why this field arrived with that clause.
    pub record: &'a dyn TheRecord,
    /// `Job files and outputs`' layout for this machine, which is how a
    /// located answer's resolution becomes a machine location.
    pub layout: &'a TheLayout,
    /// **`File Browser` N6's answer for a file on this machine**, which
    /// `Dependency Upload & Remap` V13a's walk takes first — without it the
    /// walk finds no root uploaded through Files, and `File Browser` W11's
    /// mark reaches none. Step 10c's dependency wave piece 7b.
    pub came_up_through_files: WhereFilesSaysItCameFrom<'a>,
}

impl TheNeighbours<'_> {
    /// The reading store's key for a published path.
    ///
    /// **`Render Settings`' store keys its readings by a path as text**, and
    /// `File Browser` carries paths as bytes, so a published path whose bytes
    /// are not valid text names no reading and drops none. **That is the same
    /// seam this workspace already carries at the reading traits** and it is
    /// parked at `Preparation/Groundwork/Open Threads.md` rather than settled
    /// one call at a time.
    fn the_reading_key(&self, published: &[u8]) -> Option<FileKey> {
        Some(FileKey {
            server: self.machine.server_row.clone(),
            path: String::from_utf8(published.to_vec()).ok()?,
        })
    }

    /// The roots this machine holds a check standing for —
    /// `Dependency Upload & Remap` V10's own key, and the set its walk is
    /// asked about.
    fn the_roots_with_a_standing(&self) -> Vec<MachineLocation> {
        self.standings
            .iter()
            .filter(|standing| standing.machine == self.machine)
            .map(|standing| standing.root.clone())
            .collect()
    }
}

impl TheNeighbouringRecords for TheNeighbours<'_> {
    /// **The comparison is `File Browser` X5's, per path component**, and not a
    /// raw-byte equality. A recorded root and a published path differing only
    /// by a doubled or a trailing separator are the same location, and a
    /// raw-byte test would find no standing, write no mark, and leave
    /// `Rendering`'s submission warning unfired over a file the check no longer
    /// describes.
    ///
    /// **This is `File Browser`'s own comparison rather than a second one**:
    /// [`paths_are_equal`] is the function its X5 rule lives in.
    fn the_check_standings_root_at(&self, published: &[u8]) -> Option<Vec<u8>> {
        self.standings
            .iter()
            .find(|standing| {
                standing.machine == self.machine
                    && paths_are_equal(standing.root.as_bytes(), published)
            })
            .map(|standing| standing.root.as_bytes().to_vec())
    }

    fn the_recorded_readings_path_at(&self, published: &[u8]) -> Option<Vec<u8>> {
        let key = self.the_reading_key(published)?;
        self.readings
            .standing(&key)
            .map(|_| key.path.clone().into_bytes())
    }

    /// `Dependency Upload & Remap` V13's first mark, written through that
    /// module's own operation.
    ///
    /// **The comparison is `File Browser` X5's, per path component**, on the
    /// same ground as the reading above: the root handed back here is the
    /// record's own text, and finding it again by raw bytes would fail on the
    /// spellings the reading matched.
    fn mark_the_standing_replaced_since_check(&mut self, root: &[u8]) {
        for standing in self.standings.iter_mut() {
            if standing.machine == self.machine && paths_are_equal(standing.root.as_bytes(), root) {
                *standing = standing.clone().replaced_since_check();
            }
        }
    }

    /// `Render Settings` J10's drop, at its Q14 item 1's occasion, through that
    /// document's own store.
    fn drop_the_recorded_reading(&mut self, path: &[u8]) {
        if let Some(key) = self.the_reading_key(path) {
            self.readings
                .drop_the_files_half(&key, WhatDroppedIt::ASmoozyPublicationOntoThePath);
        }
    }

    /// **`File Browser` W11's first reading**: `Dependency Upload & Remap` V2's
    /// points-at reading, taken for the roots, through that module's own walk
    /// rather than re-derived here.
    ///
    /// **The recorded text handed back is the record's own.** That walk matches
    /// a resolved location against this one by whole-path byte equality, so a
    /// root only comes back where the record's own bytes are these bytes.
    ///
    /// **So `File Browser` X5's re-check next door is inert for this reader,
    /// and that is said here rather than left to be assumed.** It compares the
    /// location handed back against the location asked about — and this reader
    /// matches on whole-path equality, so the two are necessarily the same
    /// value and the comparison cannot fail. **The re-check is live for a
    /// reader that matches more loosely**, which is what its contract is
    /// written for; it is not doing anything for this one.
    fn the_roots_whose_tree_resolved(&self, location: &[u8]) -> Vec<ARootWhoseTreeResolvedIt> {
        roots_whose_tree_resolved(
            &MachineLocation::composed(location.to_vec()),
            &self.machine,
            self.layout,
            self.record,
            &self.the_roots_with_a_standing(),
            self.came_up_through_files,
        )
        .into_iter()
        .map(|root| ARootWhoseTreeResolvedIt {
            the_location_as_the_record_holds_it: location.to_vec(),
            root: root.as_bytes().to_vec(),
        })
        .collect()
    }

    /// **`File Browser` W11's second reading**: `Dependency Upload & Remap`
    /// V2a, which is one more field of the record its V2 already reads.
    ///
    /// **`File Browser` X5's re-check next door is inert for this reader too.**
    /// That reading matches a fixing's derived-from location against this one
    /// by whole-path equality, so the location handed back is necessarily the
    /// location asked about and the comparison is a value against itself. **It
    /// is live only for a reader that matches more loosely.**
    fn the_fixed_copies_made_from(&self, location: &[u8]) -> Vec<AFixedCopyMadeFromIt> {
        what_points_at(
            &MachineLocation::composed(location.to_vec()),
            &self.machine,
            &self.machine.installation,
            self.record,
        )
        .fixed_copies_made_from_it
        .into_iter()
        .map(|copy| AFixedCopyMadeFromIt {
            the_location_as_the_record_holds_it: location.to_vec(),
            fixed_copy: copy.as_bytes().to_vec(),
        })
        .collect()
    }

    /// `Dependency Upload & Remap` V13a's disturbance mark, written through
    /// that module's own operation.
    ///
    /// **The comparison is `File Browser` X5's, per path component**, as at the
    /// two sites above. Today the root reaching this write is the standing's
    /// own text handed back through
    /// [`Self::the_roots_whose_tree_resolved`], so no spelling can differ — but
    /// one raw-byte comparison left among three per-component ones is the drift
    /// that made the other two wrong.
    fn mark_the_standing_disturbed(&mut self, root: &[u8]) {
        for standing in self.standings.iter_mut() {
            if standing.machine == self.machine && paths_are_equal(standing.root.as_bytes(), root) {
                *standing = standing.clone().disturbed();
            }
        }
    }
}

/// **`File Browser` X7a's two writes and its W11 at a publication through
/// Files, performed on the local record** — the standings read from
/// `Dependency Upload & Remap`'s keeper, marked through `File Browser`'s own
/// `at_this_publication` over [`TheNeighbours`], and every standing that
/// publication marked written back, in one save. Step 10c's dependency wave
/// piece 7b.
///
/// **The case this was first written for, and the case it now meets.**
/// `crate::surface::files` handed [`TheNeighbours`] no standings and an empty
/// record, and said so: the standings were kept nowhere in the running
/// application. Dependency piece 4 gave them a keeper and piece 7 had the band
/// read it; **from then until this, an upload through Files over a checked
/// root, or over a picture a checked root's tree resolved, marked nothing**,
/// and the band stayed gone over a file the check no longer described.
/// Piece 7b lets a Files project's standing stop the warning, so this is the
/// act that must bring it back.
///
/// **Nothing is written where a read fails**, and the caller still makes the
/// reading store's drop, which is the run's own. **A record that will not
/// answer is the one hole left**, and it is said here rather than guessed
/// round: the mark it owed is not written.
pub fn a_publication_through_files_is_marked(
    connection: &rusqlite::Connection,
    machine: &MachineKey,
    layout: &TheLayout,
    readings: &mut FileHalf,
    landing_path: &[u8],
    outcome: &smoozy_state::outcomes::UnitOutcome,
) -> Result<smoozy_file_browser::publication::WhatThePublicationWrote, super::RecordFailure> {
    use super::dependency_records::{TheDependencyRecord, WhatFilesSays};
    let keeper = TheDependencyRecord::over(connection);
    let mut standings = keeper.the_standings_on(machine)?;
    let before = standings.clone();
    let snapshot = keeper.what_it_holds_for(machine)?;
    let files_says = WhatFilesSays::read(connection, machine);
    let wrote = {
        let mut neighbours = TheNeighbours {
            machine: machine.clone(),
            standings: &mut standings,
            readings,
            record: &snapshot,
            layout,
            came_up_through_files: &|file| files_says.where_it_came_from(file),
        };
        smoozy_file_browser::publication::at_this_publication(
            landing_path,
            Some(outcome),
            &mut neighbours,
        )
    };
    the_standings_that_moved_are_saved(connection, &keeper, &standings, &before)?;
    Ok(wrote)
}

/// Every standing a write moved, saved in one save.
fn the_standings_that_moved_are_saved(
    connection: &rusqlite::Connection,
    keeper: &super::dependency_records::TheDependencyRecord<'_>,
    standings: &[CheckStanding],
    before: &[CheckStanding],
) -> Result<(), super::RecordFailure> {
    connection.execute_batch("SAVEPOINT a_publication_through_files_is_marked")?;
    let mut written = Ok(());
    for (now, was) in standings.iter().zip(before) {
        if now != was {
            written = written.and(keeper.a_standing_is_written(now));
        }
    }
    match written {
        Ok(()) => {
            connection.execute_batch("RELEASE a_publication_through_files_is_marked")?;
            Ok(())
        }
        Err(failure) => {
            let _ = connection.execute_batch(
                "ROLLBACK TO a_publication_through_files_is_marked; \
                 RELEASE a_publication_through_files_is_marked",
            );
            Err(failure)
        }
    }
}

/// **`File Browser` W11 and `File Browser` X7a at one unit of an act of that module's §7,
/// performed on the local record** — the sibling of
/// [`a_publication_through_files_is_marked`] for a delete's removal, a
/// rename's or a move's place left, and a copy's or a move's publication,
/// through that module's own `at_this_unit_of_a_managing_act`. Step 10c's
/// Files' owed acts piece 3.
///
/// **Where the unit removed a folder or moved one away, every location the
/// record names under it is marked as well** — a mirror a check placed there,
/// a fixed copy, or a place a checked tree's answer resolved to. A folder is
/// named by no answer, so asking for the folder alone would mark nothing while
/// every file the trees resolved inside it went with it. **Nothing is read from
/// the machine**: the candidates are the record's own (`File Browser` W11's
/// last limb).
pub fn a_unit_of_a_managing_act_is_marked(
    connection: &rusqlite::Connection,
    machine: &MachineKey,
    layout: &TheLayout,
    readings: &mut FileHalf,
    location: &[u8],
    did: smoozy_file_browser::managing::WhatTheUnitDid,
    outcome: &smoozy_state::outcomes::UnitOutcome,
) -> Result<(), super::RecordFailure> {
    units_of_a_managing_act_are_marked(
        connection,
        machine,
        layout,
        readings,
        &[(location.to_vec(), did, outcome.clone())],
    )
}

/// **[`a_unit_of_a_managing_act_is_marked`] for many units in one reading of
/// the record and one save** — a delete's batch, which would otherwise read
/// the whole dependency record once per entry. Step 10c's Files' owed acts
/// piece 4.
pub fn units_of_a_managing_act_are_marked(
    connection: &rusqlite::Connection,
    machine: &MachineKey,
    layout: &TheLayout,
    readings: &mut FileHalf,
    units: &[(
        Vec<u8>,
        smoozy_file_browser::managing::WhatTheUnitDid,
        smoozy_state::outcomes::UnitOutcome,
    )],
) -> Result<(), super::RecordFailure> {
    use super::dependency_records::{TheDependencyRecord, WhatFilesSays};
    use smoozy_file_browser::managing::{at_this_unit_of_a_managing_act, WhatTheUnitDid};
    if units.is_empty() {
        return Ok(());
    }
    let keeper = TheDependencyRecord::over(connection);
    let mut standings = keeper.the_standings_on(machine)?;
    let before = standings.clone();
    let snapshot = keeper.what_it_holds_for(machine)?;
    let files_says = WhatFilesSays::read(connection, machine);
    {
        let mut neighbours = TheNeighbours {
            machine: machine.clone(),
            standings: &mut standings,
            readings,
            record: &snapshot,
            layout,
            came_up_through_files: &|file| files_says.where_it_came_from(file),
        };
        for (location, did, outcome) in units {
            let mut locations = vec![location.clone()];
            if matches!(did, WhatTheUnitDid::RemovedIt | WhatTheUnitDid::MovedAwayFromIt) {
                for under in the_locations_the_record_names_under(&snapshot, machine, layout, location) {
                    if !locations.iter().any(|held| paths_are_equal(held, &under)) {
                        locations.push(under);
                    }
                }
            }
            for at in &locations {
                at_this_unit_of_a_managing_act(at, *did, Some(outcome), &mut neighbours);
            }
        }
    }
    the_standings_that_moved_are_saved(connection, &keeper, &standings, &before)
}

/// **The machine locations the record names strictly under `folder`** — the
/// mirrors its placements name, its fixed copies and the copies they were
/// derived from, and every place a recorded answer resolved to. Compared per
/// component (`File Browser` X5).
pub fn the_locations_the_record_names_under(
    record: &smoozy_dependencies::record::InMemoryRecord,
    machine: &MachineKey,
    layout: &TheLayout,
    folder: &[u8],
) -> Vec<Vec<u8>> {
    let mut under: Vec<Vec<u8>> = Vec::new();
    let mut consider = |at: &[u8]| {
        if is_inside(at, folder)
            && !paths_are_equal(at, folder)
            && !under.iter().any(|held| paths_are_equal(held, at))
        {
            under.push(at.to_vec());
        }
    };
    for placement in record.placements_on(machine) {
        consider(placement.mirrored_location.as_bytes());
    }
    for fixing in record.fixings_on(machine) {
        consider(fixing.fixed_copy.as_bytes());
        consider(fixing.derived_from.as_bytes());
    }
    for answer in &record.located_answers {
        let Some(resolution) = answer.resolution.as_ref() else {
            continue;
        };
        if let Ok(at) = smoozy_dependencies::landing::mirror_of(layout, resolution.to.location()) {
            consider(at.as_bytes());
        }
    }
    under
}

#[cfg(test)]
mod tests {
    use super::*;
    use smoozy_dependencies::bytes::{RecordedLocation, StoredLocation};
    use smoozy_dependencies::contents::ContentIdentity;
    use smoozy_dependencies::record::{
        FixingRecord, InMemoryRecord, LiveJob, PlacementRecord, ResolvedSet,
    };
    use smoozy_file_browser::already_exists::{AComparison, Comparison, TheAnswer};
    use smoozy_file_browser::publication::at_this_publication;
    use smoozy_file_browser::warnings::{
        the_delete_warning, the_live_job_warning, the_plain_upload_warning,
    };
    use smoozy_job_files::layout::{MachineRoot, WhereThePackageAreaSits};
    use smoozy_job_files::publication::AJobsPaths;
    use smoozy_queue_history::record::{ADisturbance, DisturbanceReading};
    use smoozy_server_protocol::transfers::integrity::Digest;
    use smoozy_server_protocol::transfers::SourceReading;
    use smoozy_state::identities::{Identity, IdentityKind};
    use smoozy_state::outcomes::UnitOutcome;

    fn a_machine() -> MachineKey {
        MachineKey::of(
            Identity::issue(IdentityKind::LocalServer).expect("randomness"),
            Identity::issue(IdentityKind::Installation).expect("randomness"),
        )
    }

    fn a_layout() -> TheLayout {
        TheLayout::of(
            MachineRoot::chosen("/ephemeral/smoozy").expect("a root"),
            WhereThePackageAreaSits::UnderTheRoot,
        )
    }

    fn at(path: &[u8]) -> MachineLocation {
        MachineLocation::composed(path.to_vec())
    }

    fn contents(bytes: &[u8]) -> ContentIdentity {
        ContentIdentity {
            size: bytes.len() as u64,
            digest: Digest::of(bytes),
        }
    }

    fn four_values() -> SourceReading {
        SourceReading {
            size: 3,
            inode: "77".to_string(),
            modification_time: 100,
            change_time: 100,
        }
    }

    fn a_placement(machine: &MachineKey, mirror: &[u8], source: &[u8]) -> PlacementRecord {
        PlacementRecord::at_publication(
            machine.clone(),
            smoozy_dependencies::record::Published {
                at: at(mirror),
                source: StoredLocation::as_the_system_returned_it(source),
                content: contents(b"red"),
                four_values: four_values(),
            },
        )
    }

    /// **`File Browser` W1a's class comes off the two record kinds and off
    /// nothing else**, and a recycled machine answers *neither*.
    #[test]
    fn the_class_is_placed_fixed_or_neither_and_a_recycled_machine_answers_neither() {
        let machine = a_machine();
        let layout = a_layout();
        let mirror = b"/ephemeral/smoozy/assets/files/Users/a/tex/x.png";
        let fixed = b"/ephemeral/smoozy/assets/fixed/assets/files/Users/a/lib/c.blend";
        let derived_from = b"/ephemeral/smoozy/assets/files/Users/a/lib/c.blend";
        let mut record = InMemoryRecord::default();
        record
            .placements
            .push(a_placement(&machine, mirror, b"/Users/a/tex/x.png"));
        record.fixings.push(FixingRecord {
            machine: machine.clone(),
            derived_from: at(derived_from),
            derived_from_content: contents(b"c"),
            fixed_copy: at(fixed),
            points_at: vec![],
        });

        let knows = WhatTheLocalRecordKnows {
            record: &record,
            machine: machine.clone(),
            layout: &layout,
            queue: &[],
            entries: &[],
        };
        assert_eq!(
            knows.what_the_landing_path_is_for(mirror),
            WhatTheLandingPathIsFor::Placed
        );
        assert_eq!(
            knows.what_the_landing_path_is_for(fixed),
            WhatTheLandingPathIsFor::Fixed
        );
        // **W1a's third machine location is not in the class**: the copy a
        // fixed copy was derived from answers *neither*.
        assert_eq!(
            knows.what_the_landing_path_is_for(derived_from),
            WhatTheLandingPathIsFor::Neither
        );

        // The warning composed over it names the placed file and no other.
        let answer = TheAnswer {
            comparisons: vec![
                AComparison {
                    landing_path: mirror.to_vec(),
                    what: Comparison::CollidesWithAFile,
                },
                AComparison {
                    landing_path: derived_from.to_vec(),
                    what: Comparison::CollidesWithAFile,
                },
            ],
            colliding: 2,
            arriving_files: 2,
            as_of: std::time::SystemTime::UNIX_EPOCH,
            readings_taken: 1,
        };
        let lines = the_plain_upload_warning(&answer, &knows);
        assert_eq!(lines.len(), 1);
        assert_eq!(lines[0].landing_path, mirror.to_vec());

        // A recycled machine has a different installation identity, so nothing
        // of the record answers and no line appears.
        let recycled = MachineKey::of(
            machine.server_row.clone(),
            Identity::issue(IdentityKind::Installation).expect("randomness"),
        );
        let knows = WhatTheLocalRecordKnows {
            record: &record,
            machine: recycled,
            layout: &layout,
            queue: &[],
            entries: &[],
        };
        assert_eq!(
            knows.what_the_landing_path_is_for(mirror),
            WhatTheLandingPathIsFor::Neither
        );
        assert!(the_plain_upload_warning(&answer, &knows).is_empty());
    }

    /// **`File Browser` W1's delete warning, over that module's own points-at
    /// reading.**
    #[test]
    fn the_delete_warning_reads_the_dependency_modules_own_points_at_answer() {
        let machine = a_machine();
        let layout = a_layout();
        let mirror = b"/ephemeral/smoozy/assets/files/Users/a/tex/x.png";
        let fixed = b"/ephemeral/smoozy/assets/fixed/assets/files/Users/a/lib/c.blend";
        let mut record = InMemoryRecord::default();
        record
            .placements
            .push(a_placement(&machine, mirror, b"/Users/a/tex/x.png"));
        record.fixings.push(FixingRecord {
            machine: machine.clone(),
            derived_from: at(b"/ephemeral/smoozy/assets/files/Users/a/lib/c.blend"),
            derived_from_content: contents(b"c"),
            fixed_copy: at(fixed),
            points_at: vec![(
                RecordedLocation::as_the_reading_carried_it(b"//tex/x.png"),
                at(mirror),
            )],
        });
        let knows = WhatTheLocalRecordKnows {
            record: &record,
            machine,
            layout: &layout,
            queue: &[],
            entries: &[],
        };
        let lines = the_delete_warning(&[mirror.to_vec()], &knows);
        assert_eq!(lines.len(), 1);
        assert_eq!(
            lines[0]
                .what_depended_on_it
                .fixed_copies_whose_fixing_points_at_it,
            vec![fixed.to_vec()]
        );
        assert_eq!(
            lines[0].what_depended_on_it.placed_from,
            vec![b"/Users/a/tex/x.png".to_vec()]
        );
        assert!(the_delete_warning(&[b"/ephemeral/smoozy/elsewhere".to_vec()], &knows).is_empty());
    }

    fn a_local_record(job: &Identity, blend: &str) -> LocalJobRecord {
        LocalJobRecord {
            job: job.clone(),
            group: None,
            server: smoozy_queue_history::record::TheServerAsItStood {
                local_server: Identity::issue(IdentityKind::LocalServer).expect("randomness"),
                label: None,
                hue: None,
            },
            submitted_at: None,
            label: None,
            blend_path_on_the_machine: Some(blend.to_string()),
            scene_name: None,
            submitted_with_no_scene_named: false,
            resolution_percentage_typed: None,
            picture_size_from_the_read: None,
            main_output_from_the_read: None,
            blender: smoozy_queue_history::record::TheBlenderTheEntryKeeps {
                pick: smoozy_render_settings::payload::VersionPick::of(&a_blender()),
                origin: smoozy_queue_history::record::AnswerOrigin::ResolvedForThem,
            },
            device_asked_for: smoozy_queue_history::record::TheDeviceAsk::TheFilesOwnUnread,
            the_files_values_beside_the_overrides: None,
            dependency_check: None,
            carries_on_from: None,
            disturbances: vec![],
            attempts_where_the_file_changed: vec![],
            the_frames: smoozy_queue_history::record::TheFramesTheReadFound::default(),
            standing: smoozy_queue_history::record::Standing::Provisional,
            the_render_start_has_been_learned: false,
            what_the_note_carried: smoozy_queue_history::record::WhatTheNoteCarried::default(),
            the_job_output_folder: Default::default(),
            the_driver_was_below_the_bar: None,
        }
    }

    /// **`File Browser` W5's live set, composed from `Queue & History` Q2's
    /// records** — and a job whose outcome is written is not in it.
    #[test]
    fn the_live_set_is_the_waiting_the_held_and_the_rendering_and_never_the_ended() {
        let machine = a_machine();
        let layout = a_layout();
        let blend = "/ephemeral/smoozy/assets/files/Users/a/scene.blend";
        let texture = b"/ephemeral/smoozy/assets/files/Users/a/tex/x.png";

        let waiting = Identity::issue(IdentityKind::Job).expect("randomness");
        let ended = Identity::issue(IdentityKind::Job).expect("randomness");
        let queue = vec![
            JobRecord::submitted(waiting.clone(), None, "jr/one".to_string(), &[]),
            JobRecord {
                job: ended.clone(),
                group: None,
                sequence_key: None,
                state: QueueState::Ended {
                    outcome: Identity::issue(IdentityKind::Operation).expect("randomness"),
                },
                record_directory: "jr/two".to_string(),
                removed_build: None,
                reporting_choice_now: None,
            },
        ];
        let entries = vec![
            a_local_record(&waiting, blend),
            a_local_record(&ended, blend),
        ];

        let mut record = InMemoryRecord::default();
        for job in [&waiting, &ended] {
            record.jobs.push(LiveJob {
                job: job.clone(),
                machine: machine.clone(),
                blend: at(blend.as_bytes()),
                resolved_set: ResolvedSet {
                    locations: vec![at(texture)],
                    remap_material: at(b"/ephemeral/smoozy/assets/remaps/one"),
                },
            });
        }

        let knows = WhatTheLocalRecordKnows {
            record: &record,
            machine,
            layout: &layout,
            queue: &queue,
            entries: &entries,
        };
        let live = knows.the_live_jobs();
        assert_eq!(live.len(), 1);
        assert_eq!(live[0].job, waiting);
        assert_eq!(live[0].stands, HowTheJobStands::Waiting);
        assert!(live[0].resolved_set.is_some());
        assert!(!live[0].its_blend_was_disturbed);
        // The job folder is `Job files and outputs`' own composition and not a
        // path made up here.
        assert_eq!(
            live[0].job_folder,
            Some(
                AJobsPaths::composed(&a_layout(), &waiting, None)
                    .expect("it fits")
                    .job_output_folder
                    .into_bytes()
            )
        );

        // The warning fires over the waiting job and names nothing for the
        // ended one.
        let named = the_live_job_warning(&[texture.to_vec()], &knows);
        assert_eq!(named.len(), 1);
        assert_eq!(named[0].job, waiting);
    }

    /// **`File Browser` W6: a job whose file was never checked is the widest
    /// case** — its `.blend` and its job folder, said as such — even though
    /// every submission writes `Dependency Upload & Remap` K9's set, with only
    /// the remap material in it where nothing was checked. After 0.1.8, the
    /// checks' leftovers, item 2: until then such a job read as fully
    /// checked, and its warning never said Smoozy had only those two.
    #[test]
    fn a_job_whose_file_was_never_checked_is_the_widest_case() {
        use smoozy_queue_history::record::DependencyCheckAtSubmission;
        let machine = a_machine();
        let layout = a_layout();
        let blend = "/ephemeral/smoozy/assets/files/Users/a/scene.blend";
        let never = Identity::issue(IdentityKind::Job).expect("randomness");
        let checked = Identity::issue(IdentityKind::Job).expect("randomness");
        let queue = vec![
            JobRecord::submitted(never.clone(), None, "jr/one".to_string(), &[]),
            JobRecord::submitted(checked.clone(), None, "jr/two".to_string(), &[]),
        ];
        let mut never_checked = a_local_record(&never, blend);
        never_checked.dependency_check = Some(DependencyCheckAtSubmission::NoStandingAtAll);
        let mut was_checked = a_local_record(&checked, blend);
        was_checked.dependency_check = Some(DependencyCheckAtSubmission::ACompleteStandingStood);
        let entries = vec![never_checked, was_checked];
        let mut record = InMemoryRecord::default();
        for job in [&never, &checked] {
            record.jobs.push(LiveJob {
                job: job.clone(),
                machine: machine.clone(),
                blend: at(blend.as_bytes()),
                resolved_set: ResolvedSet {
                    locations: vec![],
                    remap_material: at(b"/ephemeral/smoozy/assets/remaps/one"),
                },
            });
        }
        let knows = WhatTheLocalRecordKnows {
            record: &record,
            machine,
            layout: &layout,
            queue: &queue,
            entries: &entries,
        };
        let live = knows.the_live_jobs();
        let of = |job: &Identity| live.iter().find(|one| &one.job == job).expect("live");
        assert!(of(&never).the_set_is_the_widest_case(), "{:?}", of(&never));
        assert!(!of(&checked).the_set_is_the_widest_case(), "{:?}", of(&checked));
        // **The warning over the `.blend` says so.**
        let named = the_live_job_warning(&[blend.as_bytes().to_vec()], &knows);
        let never_named = named.iter().find(|one| one.job == never).expect("named");
        assert!(never_named.the_set_was_the_widest_case);
    }

    /// **`File Browser` W5's fall-back, read off `Queue & History` N21's own
    /// marks**: a job whose `.blend` a Smoozy act disturbed keeps the widest
    /// set for the rest of its life.
    #[test]
    fn a_job_whose_blend_was_disturbed_reads_back_with_the_widest_set() {
        let machine = a_machine();
        let layout = a_layout();
        let blend = "/ephemeral/smoozy/assets/files/Users/a/scene.blend";
        let texture = b"/ephemeral/smoozy/assets/files/Users/a/tex/x.png";
        let job = Identity::issue(IdentityKind::Job).expect("randomness");

        let queue = vec![JobRecord::submitted(
            job.clone(),
            None,
            "jr/one".to_string(),
            &[],
        )];
        let mut entry = a_local_record(&job, blend);
        entry.disturbances.push(ADisturbance {
            the_member: blend.to_string(),
            the_act_intended: "an upload request".to_string(),
            reading: DisturbanceReading::Disturbed,
            how_many: None,
        });
        let entries = vec![entry];

        let mut record = InMemoryRecord::default();
        record.jobs.push(LiveJob {
            job: job.clone(),
            machine: machine.clone(),
            blend: at(blend.as_bytes()),
            resolved_set: ResolvedSet {
                locations: vec![at(texture)],
                remap_material: at(b"/ephemeral/smoozy/assets/remaps/one"),
            },
        });

        let knows = WhatTheLocalRecordKnows {
            record: &record,
            machine,
            layout: &layout,
            queue: &queue,
            entries: &entries,
        };
        let live = knows.the_live_jobs();
        assert!(live[0].its_blend_was_disturbed);
        // The snapshot describes a file that is no longer at that path, so the
        // texture is out of the set and the `.blend` and the folder stand.
        assert!(the_live_job_warning(&[texture.to_vec()], &knows).is_empty());
        assert_eq!(
            the_live_job_warning(&[blend.as_bytes().to_vec()], &knows).len(),
            1
        );
    }

    /// **`File Browser` W6: a job Smoozy cannot fully name is narrowed and not
    /// dropped**, in all three shapes, on the real records.
    ///
    /// A dropped job is invisible to the warning — not its folder, which is
    /// known independently of its `.blend`, and not its resolved set, which may
    /// be known too — and that under-warns, which is what `File Browser` W5b's stated property
    /// forbids.
    #[test]
    fn a_job_missing_its_blend_or_its_folder_is_narrowed_and_only_a_job_missing_both_is_dropped() {
        let machine = a_machine();
        let layout = a_layout();
        let texture = b"/ephemeral/smoozy/assets/files/Users/a/tex/x.png";
        let record = InMemoryRecord::default();

        // **No `.blend` on either record**: no dependency answer, and the
        // entry's own path absent.
        let no_blend = Identity::issue(IdentityKind::Job).expect("randomness");
        // **A folder that cannot be composed**: the label pushes the path past
        // what the machine accepts, so `Job files and outputs` refuses it.
        let no_folder = Identity::issue(IdentityKind::Job).expect("randomness");
        // **Neither can be named**, which is the one case with nothing to say.
        let neither = Identity::issue(IdentityKind::Job).expect("randomness");

        let queue: Vec<JobRecord> = [&no_blend, &no_folder, &neither]
            .into_iter()
            .map(|job| JobRecord::submitted(job.clone(), None, "jr/x".to_string(), &[]))
            .collect();

        let mut entries = vec![
            a_local_record(&no_blend, ""),
            a_local_record(&no_folder, "/ephemeral/smoozy/assets/files/Users/a/scene.blend"),
            a_local_record(&neither, ""),
        ];
        entries[0].blend_path_on_the_machine = None;
        entries[2].blend_path_on_the_machine = None;

        // A layout whose own root all but fills the path budget, so a job
        // folder under it does not fit and `Job files and outputs` refuses to
        // compose one. **That is what makes the folder absent for real rather
        // than by a stub.**
        let deep = TheLayout::of(
            MachineRoot::chosen(&format!(
                "/{}/{}",
                vec!["d".repeat(250); 16].join("/"),
                "d".repeat(70)
            ))
            .expect("a root that fits the budget"),
            WhereThePackageAreaSits::UnderTheRoot,
        );
        assert!(AJobsPaths::composed(&layout, &no_folder, None).is_ok());
        assert!(AJobsPaths::composed(&deep, &no_folder, None).is_err());

        // The two jobs whose folder is absent are read under the deep layout,
        // and the one whose `.blend` is absent under the ordinary one.
        let knows_deep = WhatTheLocalRecordKnows {
            record: &record,
            machine: machine.clone(),
            layout: &deep,
            queue: &queue,
            entries: &entries,
        };
        let under_the_deep_layout = knows_deep.the_live_jobs();
        assert_eq!(
            under_the_deep_layout.len(),
            1,
            "the job whose `.blend` is on its entry keeps that member and is not dropped"
        );
        assert_eq!(under_the_deep_layout[0].job, no_folder);
        assert!(under_the_deep_layout[0].blend.is_some());
        assert!(under_the_deep_layout[0].job_folder.is_none());
        assert!(
            !under_the_deep_layout
                .iter()
                .any(|job| job.job == neither || job.job == no_blend),
            "a job neither of whose two members can be named has nothing to say"
        );

        let knows = WhatTheLocalRecordKnows {
            record: &record,
            machine,
            layout: &layout,
            queue: &queue,
            entries: &entries,
        };
        let live = knows.the_live_jobs();
        assert_eq!(
            live.len(),
            3,
            "under a layout that composes, all three jobs keep a folder"
        );
        assert_eq!(live[0].job, no_blend);
        assert!(
            live[0].blend.is_none(),
            "no `.blend` is on either record for this job"
        );
        assert!(
            live[0].job_folder.is_some(),
            "the folder is known independently of the `.blend`, so the job is narrowed and not dropped"
        );

        // The narrowed jobs still reach the warning over what they do name.
        assert_eq!(
            the_live_job_warning(
                &[b"/ephemeral/smoozy/assets/files/Users/a/scene.blend".to_vec()],
                &knows
            )
            .len(),
            1
        );
        assert!(the_live_job_warning(&[texture.to_vec()], &knows).is_empty());
    }

    /// **`File Browser` W1's delete warning over a folder victim, on the real
    /// records**: the folder itself depended on nothing and the answer is
    /// underneath it.
    #[test]
    fn a_folder_victim_names_the_placed_files_the_record_holds_under_it() {
        let machine = a_machine();
        let layout = a_layout();
        let folder = b"/ephemeral/smoozy/assets/files/Users/a/kitchen";
        let wood = b"/ephemeral/smoozy/assets/files/Users/a/kitchen/wood.png";
        let tile = b"/ephemeral/smoozy/assets/files/Users/a/kitchen/tile.png";
        let elsewhere = b"/ephemeral/smoozy/assets/files/Users/a/bath/tap.png";
        let mut record = InMemoryRecord::default();
        for (mirror, source) in [
            (&wood[..], &b"/Users/a/tex/wood.png"[..]),
            (&tile[..], &b"/Users/a/tex/tile.png"[..]),
            (&elsewhere[..], &b"/Users/a/tex/tap.png"[..]),
        ] {
            record
                .placements
                .push(a_placement(&machine, mirror, source));
        }

        let knows = WhatTheLocalRecordKnows {
            record: &record,
            machine,
            layout: &layout,
            queue: &[],
            entries: &[],
        };
        let lines = the_delete_warning(&[folder.to_vec()], &knows);
        assert_eq!(lines.len(), 1);
        assert!(
            !lines[0].what_depended_on_it.anything_depended_on_it(),
            "the records name files rather than the folders holding them"
        );
        let mut named: Vec<Vec<u8>> = lines[0]
            .what_is_under_it
            .iter()
            .map(|under| under.location.clone())
            .collect();
        named.sort();
        assert_eq!(named, vec![tile.to_vec(), wood.to_vec()]);

        // A folder the record holds nothing under produces no line, and a name
        // that merely begins with the folder's text is not inside it.
        assert!(
            the_delete_warning(&[b"/ephemeral/smoozy/assets/files/Users/a/kitchenette".to_vec()], &knows)
                .is_empty()
        );
    }

    fn a_blender() -> smoozy_render_settings::file_half::BlenderIdentity {
        smoozy_render_settings::file_half::BlenderIdentity {
            path: "/ephemeral/smoozy/blender/b".to_string(),
            version: smoozy_server_setup::blender::Version {
                major: 4,
                minor: 5,
                patch: 0,
            },
        }
    }

    fn a_standing(machine: &MachineKey, root: &str) -> CheckStanding {
        CheckStanding::at_the_sittings_end(
            at(root.as_bytes()),
            machine.clone(),
            a_blender(),
            std::time::SystemTime::UNIX_EPOCH,
            smoozy_dependencies::standing::Completion::Completed,
            &[],
            four_values(),
        )
    }

    /// **`File Browser` X7a's two writes, landing on the two records
    /// themselves.**
    #[test]
    fn a_publication_marks_the_standing_and_drops_the_reading_on_the_real_records() {
        let machine = a_machine();
        let layout = a_layout();
        let record = InMemoryRecord::default();
        let root = "/ephemeral/smoozy/assets/files/Users/a/scene.blend";
        let mut standings = vec![a_standing(&machine, root)];
        let mut readings = FileHalf::new();
        let key = FileKey {
            server: machine.server_row.clone(),
            path: root.to_string(),
        };
        readings.a_reading_landed(
            key.clone(),
            &smoozy_render_settings::file_half::StartedUnder {
                version_pick: a_blender(),
                file_identity: four_values(),
            },
            smoozy_render_settings::reading::SettingsHalf {
                scenes: vec![],
                offered_engine_identifiers:
                    smoozy_render_settings::reading::OfferedEngineIdentifiers {
                        cycles: None,
                        eevee: None,
                    },
                announced_add_on_case: None,
                the_folders_at_the_machines_top: None,
            },
            Some(&a_blender()),
            &four_values(),
            std::time::SystemTime::UNIX_EPOCH,
        );
        assert!(readings.standing(&key).is_some());
        assert!(!standings[0].replaced_since_check);

        let mut neighbours = TheNeighbours {
            machine: machine.clone(),
            standings: &mut standings,
            readings: &mut readings,
            record: &record,
            layout: &layout,
            came_up_through_files: &smoozy_dependencies::standing::nothing_came_up_through_files,
        };
        let wrote = at_this_publication(root.as_bytes(), Some(&UnitOutcome::Done), &mut neighbours);
        assert_eq!(
            wrote.the_standing_marked_replaced_since_check,
            Some(root.as_bytes().to_vec())
        );
        assert_eq!(
            wrote.the_recorded_reading_dropped,
            Some(root.as_bytes().to_vec())
        );
        assert!(standings[0].replaced_since_check);
        assert!(readings.standing(&key).is_none());

        // **A publication elsewhere writes nothing**, and the standing that
        // was already marked is not marked twice into something else.
        let mut standings = vec![a_standing(&machine, root)];
        let mut readings = FileHalf::new();
        let mut neighbours = TheNeighbours {
            machine,
            standings: &mut standings,
            readings: &mut readings,
            record: &record,
            layout: &layout,
            came_up_through_files: &smoozy_dependencies::standing::nothing_came_up_through_files,
        };
        let wrote = at_this_publication(
            b"/ephemeral/smoozy/assets/files/Users/a/other.blend",
            Some(&UnitOutcome::Done),
            &mut neighbours,
        );
        assert!(wrote.is_nothing());
        assert!(!standings[0].replaced_since_check);
    }

    /// **`File Browser` X5's per-component rule, proved where it actually
    /// runs.**
    ///
    /// The rule was proved next door against a hand-written stand-in, and the
    /// only real implementation of the reading did not behave that way: it
    /// compared the recorded root's raw bytes against the published path, so a
    /// root and a landing path differing only by a separator found no standing,
    /// wrote no mark, and left `Rendering`'s submission warning unfired over a
    /// file the check no longer describes. **This drives the real adapter and
    /// watches the mark land.**
    #[test]
    fn a_root_and_a_published_path_differing_only_by_a_separator_still_mark_the_standing() {
        let machine = a_machine();
        let layout = a_layout();
        let record = InMemoryRecord::default();
        let as_recorded = "/ephemeral/smoozy/assets//files/Users/a/scene.blend";
        let as_published = b"/ephemeral/smoozy/assets/files/Users/a/scene.blend";
        let mut standings = vec![a_standing(&machine, as_recorded)];
        let mut readings = FileHalf::new();
        let mut neighbours = TheNeighbours {
            machine: machine.clone(),
            standings: &mut standings,
            readings: &mut readings,
            record: &record,
            layout: &layout,
            came_up_through_files: &smoozy_dependencies::standing::nothing_came_up_through_files,
        };
        let wrote = at_this_publication(as_published, Some(&UnitOutcome::Done), &mut neighbours);
        assert_eq!(
            wrote.the_standing_marked_replaced_since_check,
            Some(as_recorded.as_bytes().to_vec()),
            "the reading matched by raw bytes instead of per component"
        );
        assert!(
            standings[0].replaced_since_check,
            "the write matched by raw bytes instead of per component"
        );

        // **A spelling the rule does hold apart is still held apart**: the
        // comparison is per component and never a normalisation.
        let mut standings = vec![a_standing(&machine, as_recorded)];
        let mut readings = FileHalf::new();
        let mut neighbours = TheNeighbours {
            machine,
            standings: &mut standings,
            readings: &mut readings,
            record: &record,
            layout: &layout,
            came_up_through_files: &smoozy_dependencies::standing::nothing_came_up_through_files,
        };
        let wrote = at_this_publication(
            b"/ephemeral/smoozy/assets/files/Users/a/SCENE.blend",
            Some(&UnitOutcome::Done),
            &mut neighbours,
        );
        assert!(wrote.is_nothing());
        assert!(!standings[0].replaced_since_check);
    }

    /// **`File Browser` W11's write, landing on the real check standing.**
    ///
    /// The composition behind it — which standings a location reaches, and the
    /// comparison — is that module's and is proved there. What this asks is
    /// that the write itself reaches `Dependency Upload & Remap`'s own record
    /// through that module's own operation, and that a standing on another
    /// machine is not touched by it.
    #[test]
    fn the_disturbance_mark_lands_on_the_standing_it_names() {
        let machine = a_machine();
        let layout = a_layout();
        let record = InMemoryRecord::default();
        let root = "/ephemeral/smoozy/assets/files/Users/a/scene.blend";
        let elsewhere = "/ephemeral/smoozy/assets/files/Users/a/other.blend";
        let mut standings = vec![a_standing(&machine, root), a_standing(&machine, elsewhere)];
        let mut readings = FileHalf::new();
        let mut neighbours = TheNeighbours {
            machine: machine.clone(),
            standings: &mut standings,
            readings: &mut readings,
            record: &record,
            layout: &layout,
            came_up_through_files: &smoozy_dependencies::standing::nothing_came_up_through_files,
        };
        neighbours.mark_the_standing_disturbed(root.as_bytes());
        assert!(standings[0].disturbed);
        assert!(!standings[1].disturbed);
        // The other mark is a different record state and this write does not
        // make it (`Dependency Upload & Remap` V14).
        assert!(!standings[0].replaced_since_check);
    }

    /// **`File Browser` W11 as widened 2026-10-02, on the real record: a rename
    /// or a same-filesystem move marks the standings whose tree resolved the
    /// place it left** — the file itself, or anything the record names under a
    /// folder that went (step 10c's Files' owed acts piece 3). **A rename the
    /// machine refused marks nothing**, and nothing is marked for a place no
    /// tree resolved.
    #[test]
    fn a_rename_away_from_what_a_tree_resolved_marks_the_standing_on_the_record() {
        use smoozy_dependencies::landing::mirror_of;
        use smoozy_dependencies::record::{
            HoldingFile, HowResolved, LocatedAnswer, LocatedAnswerKey, Published, Resolution,
            ResolvedTo,
        };
        use smoozy_file_browser::managing::WhatTheUnitDid;
        let folder = std::env::temp_dir().join(format!("smoozy-moved-away-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&folder);
        std::fs::create_dir_all(&folder).expect("a scratch folder");
        let record = crate::record::LocalRecord::open_at(&folder.join("local-record.sqlite3")).expect("the record opens");
        let connection = record.connection();
        let machine = a_machine();
        let layout = a_layout();
        let keeper = super::super::dependency_records::TheDependencyRecord::over(connection);
        let stored = |bytes: &[u8]| StoredLocation::as_the_system_returned_it(bytes);
        let root = mirror_of(&layout, &stored(b"/Users/a/scene.blend")).expect("fits");
        keeper
            .a_placement_is_written(&PlacementRecord::at_publication(
                machine.clone(),
                Published {
                    at: root.clone(),
                    source: stored(b"/Users/a/scene.blend"),
                    content: ContentIdentity::of(b"a scene"),
                    four_values: four_values(),
                },
            ))
            .expect("the root's placement");
        keeper
            .the_answers_are_written(&[LocatedAnswer {
                key: LocatedAnswerKey {
                    holding_file: HoldingFile::OnTheUsersComputer(stored(b"/Users/a/scene.blend")),
                    reference: RecordedLocation::as_the_reading_carried_it(b"//tex/stone.png"),
                },
                resolution: Some(Resolution {
                    to: ResolvedTo::AWholeLocation(stored(b"/Users/a/tex/stone.png")),
                    how: HowResolved::TheRecordedLocationItself,
                }),
                set_aside: false,
                rejected_guess: None,
                read_as: Vec::new(),
            }])
            .expect("the root's answer");
        let texture = mirror_of(&layout, &stored(b"/Users/a/tex/stone.png")).expect("fits");
        let the_standing = || {
            keeper
                .the_standings_on(&machine)
                .expect("the standings read")
                .into_iter()
                .next()
                .expect("one standing")
        };
        let mut texture_folder = texture.as_bytes().to_vec();
        texture_folder.truncate(texture_folder.len() - b"/stone.png".len());
        for (at, outcome, marked) in [
            (texture_folder.clone(), UnitOutcome::Failed, false),
            (b"/ephemeral/smoozy/assets/elsewhere".to_vec(), UnitOutcome::Done, false),
            (texture.as_bytes().to_vec(), UnitOutcome::Done, true),
            (texture_folder.clone(), UnitOutcome::Done, true),
        ] {
            keeper
                .a_standing_is_written(&a_standing(&machine, &String::from_utf8_lossy(&root.as_bytes())))
                .expect("the root's standing");
            assert!(!the_standing().disturbed);
            let mut readings = FileHalf::new();
            a_unit_of_a_managing_act_is_marked(
                connection,
                &machine,
                &layout,
                &mut readings,
                &at,
                WhatTheUnitDid::MovedAwayFromIt,
                &outcome,
            )
            .expect("the mark is written");
            assert_eq!(
                the_standing().disturbed,
                marked,
                "{} {outcome:?}",
                String::from_utf8_lossy(&at)
            );
        }
        // **A delete's batch, in one reading** (`File Browser` W11's
        // *removed*, step 10c's Files' owed acts piece 4): the texture among
        // other removed entries marks the standing too.
        keeper
            .a_standing_is_written(&a_standing(&machine, &String::from_utf8_lossy(&root.as_bytes())))
            .expect("the root's standing");
        assert!(!the_standing().disturbed);
        units_of_a_managing_act_are_marked(
            connection,
            &machine,
            &layout,
            &mut FileHalf::new(),
            &[
                (b"/ephemeral/smoozy/assets/elsewhere".to_vec(), WhatTheUnitDid::RemovedIt, UnitOutcome::Done),
                (texture.as_bytes().to_vec(), WhatTheUnitDid::RemovedIt, UnitOutcome::Done),
            ],
        )
        .expect("the marks are written");
        assert!(the_standing().disturbed, "a removed texture a checked tree resolved left its standing unmarked");
        drop(record);
        let _ = std::fs::remove_dir_all(&folder);
    }
}
