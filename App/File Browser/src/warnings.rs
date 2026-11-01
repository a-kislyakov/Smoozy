//! The three warnings, and the telling — `File Browser` §8, W1 to W10, Rows
//! 193, 195 and 196.
//!
//! **What counts as depended on is `Dependency Upload & Remap`'s; which jobs
//! are live is `Queue & History`'s; the moment, the composition and the
//! durability are this document's.** Wording is Design's throughout, so
//! nothing here composes a sentence: every function returns what the warning
//! names and never how it reads.
//!
//! **W1's hard rule binds every line below: the readings are answered from the
//! local record at the moment a destination is named, before any write, with
//! no read of the machine.** [`ReadsWhatTheLocalRecordKnows`] is the only way
//! into them and it carries no machine act, which is how the line is kept by
//! the types rather than by a habit.
//!
//! **Why the readings arrive behind a trait rather than by naming the two
//! crates that hold them.** `Dependency Upload & Remap` V1's readings and
//! `Queue & History` Q2's records are what §8 reads, and this crate cannot
//! name either crate: `Dependency Upload & Remap` is built on `Render
//! Settings`, which is built on `Server Setup`, which is built on this crate
//! for P5's two elevation readings — so a direct dependency is a cycle the
//! build refuses. **The judgement stays here and the reading is answered next
//! door**, at `Desktop app and delivery`, which already answers this crate's
//! other local-record seam and owns the store the readings come out of. **No
//! second record shape is invented**: the adapter there reads
//! `Dependency Upload & Remap`'s own record and `Queue & History`'s own job
//! records and hands back what this document names.
//!
//! **Markers used below that belong to another document, each with the
//! document it belongs to.** `Dependency Upload & Remap` V1, V2, V2a, V3, V5,
//! V6, V8, V9, V13, V13a. `Queue & History` Q2, N21. `Job files and outputs`
//! R10, R13, R15. `Rented-server runtime` S14. `Desktop app and delivery`
//! G3, G4, G5.
//!
//! **A marker this plan also defines is deliberately not listed above,
//! whichever document is meant, and is written out with its document at every
//! site instead** — the header list cannot tell the two apart, and a bare one
//! would resolve to a real and wrong clause. This plan defines a K7, a K8, a
//! K9, an M2, an M3, a U1 and a U4 of its own, so
//! `Dependency Upload & Remap`'s placement record, its fixing record and its
//! per-job resolved set are written out where they are cited.

use smoozy_state::identities::Identity;
use smoozy_state::outcomes::UnitOutcome;

use crate::already_exists::{is_inside, paths_are_equal, AComparison, TheAnswer};

/// **W1a: which of the colliding files were placed by the dependency check is
/// a per-file class, read off two of that module's record kinds and off
/// nothing this document keeps.**
///
/// **The class is a property of what the machine location is for, never of
/// which act wrote the file last, and that is what makes it decidable with no
/// ordering between record kinds.** A location a fixing record names is the
/// address a top file's library pair points at, so whatever sits there is what
/// a render reads through that reference; a location a placement record names
/// is the mirror a located answer resolves to. Both stay true where an earlier
/// plain upload has already published over the location — which is the case
/// the warning is needed in most — so nothing here compares this document's
/// enumeration entries against those records for recency, and **no write
/// ordering across record kinds is established anywhere to compare them by.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhatTheLandingPathIsFor {
    /// A placement record (`Dependency Upload & Remap` K7) names that machine
    /// location for this server row under the machine's current installation
    /// identity.
    Placed,
    /// A fixing record (`Dependency Upload & Remap` K8) names that machine
    /// location as where a fixed copy sits.
    Fixed,
    /// No such record names it.
    ///
    /// ***Neither* is not [`crate::uploading::WhereItCameFrom`]'s *no
    /// record*** (N7): N6's lookup answers a different question about the same
    /// path — where the file came from on the user's own computer — and
    /// neither answer decides the other, which is N9's shape applied to a
    /// second pair of readings.
    Neither,
}

impl WhatTheLandingPathIsFor {
    /// **W1b's half of the intersection**: the line names the files whose
    /// class is *placed* or *fixed*, and no others.
    pub fn the_warning_names_it(self) -> bool {
        matches!(
            self,
            WhatTheLandingPathIsFor::Placed | WhatTheLandingPathIsFor::Fixed
        )
    }
}

/// **W1a's third machine location is not in this class, and the absence is a
/// decision rather than an omission.**
///
/// *"The machine copy the fixed copy was derived from is what the fixed copy
/// was produced **from**, and the render reads the fixed copy, so publishing
/// over it puts nothing pointing at the user's own computer in front of a
/// render."* What publishing over it does instead is routed rather than
/// decided here (§13 item 15) and is `Dependency Upload & Remap` M16b's,
/// read off its V2a and marked by its V13a. **No fourth class is added here
/// and no warning of this document's is widened.**
pub const THE_COPY_A_FIXED_COPY_WAS_DERIVED_FROM_IS_NOT_IN_THE_CLASS: bool = true;

/// **W1's points-at reading, as this document takes it** —
/// `Dependency Upload & Remap` V2 and V2a, answered from the local record
/// alone.
///
/// **The delete warning names anything the points-at reading shows depended
/// on, whether or not a job is live** — a shared asset another reference was
/// pointed at, a fixed `.blend` that would be left pointing at nothing.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct WhatDependedOnIt {
    /// Which fixed copies' fixings point at it — a `.blend` that was fixed to
    /// point at assets and would be left pointing at nothing.
    pub fixed_copies_whose_fixing_points_at_it: Vec<Vec<u8>>,
    /// Which placement records name it, by the source location each was placed
    /// from — an asset shared by two projects because a later reference was
    /// pointed at the copy already there.
    pub placed_from: Vec<Vec<u8>>,
    /// Which live or waiting jobs' resolved sets name it, the remap material
    /// included.
    pub jobs_whose_set_names_it: Vec<Identity>,
}

impl WhatDependedOnIt {
    /// Whether anything the record shows depended on the location — the delete
    /// warning's whole test (`Dependency Upload & Remap` V3).
    pub fn anything_depended_on_it(&self) -> bool {
        !self.fixed_copies_whose_fixing_points_at_it.is_empty()
            || !self.placed_from.is_empty()
            || !self.jobs_whose_set_names_it.is_empty()
    }
}

/// **W5a's first bullet: per job, the warning may differentiate, and this is
/// what on** — the live set is the records with state *waiting*, *held* or
/// *rendering*, so a job that is rendering and a job that is waiting are told
/// apart.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HowTheJobStands {
    Waiting,
    Held,
    Rendering,
}

/// **W5: one live job, with the set it depends on.**
///
/// **A job is live from its record's publication until its recorded outcome is
/// written, held jobs included** — the records with state *waiting*, *held* or
/// *rendering* per machine (`Queue & History` Q2). A record whose outcome is
/// written is not one of these and never reaches this type.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ALiveJob {
    pub job: Identity,
    pub stands: HowTheJobStands,
    /// **The manifest's `.blend`** — `Job files and outputs` R15 supplies the
    /// path.
    ///
    /// **Absent where no record names one.** W6's answer to missing knowledge
    /// is to narrow the set and say so, so a job whose `.blend` cannot be named
    /// keeps the members that can be; dropping the job would narrow the set to
    /// nothing and say nothing, which is the under-warning W5b's stated
    /// property forbids.
    pub blend: Option<Vec<u8>>,
    /// **The job's own folder**, whose path is on the job's record under
    /// `Job files and outputs`' layout. The spec covers *"the folder and files
    /// a running job is writing to"*.
    ///
    /// **Absent where the layout cannot compose one**, on the same ground as
    /// the `.blend` above.
    pub job_folder: Option<Vec<u8>>,
    /// **The per-job resolved set fixed at submission, the remap material
    /// named inside it** (`Dependency Upload & Remap` V5, and that module's own
    /// per-job resolved set at its `Dependency Upload & Remap` K9).
    ///
    /// **W6: a job whose file was never checked has no resolved set**, and
    /// then the warning covers what exists — the `.blend` and the job folder.
    /// That is the spec's own smaller-set clause arriving at its widest, and
    /// it is said rather than patched.
    pub resolved_set: Option<Vec<Vec<u8>>>,
    /// **W5's fall-back trigger: whether this job's own manifest `.blend` has
    /// been disturbed by one of Smoozy's own recorded acts** — read off the
    /// durable disturb records W8 already writes, which are local, so no
    /// reading of the machine is added.
    pub its_blend_was_disturbed: bool,
}

impl ALiveJob {
    /// **What this job depends on**, composed rather than read.
    ///
    /// **W5's fall-back:** where the job's own manifest `.blend` has been
    /// disturbed by one of Smoozy's own recorded acts, the set falls back to
    /// W6's widest case for the rest of its life — the `.blend` path plus the
    /// job folder, and no resolved set. W6 makes the resolved set a snapshot
    /// never re-derived, and after such a Replace the snapshot describes a file
    /// that is no longer at that path: it names files the job may no longer
    /// touch and misses the ones it now does, which is past the spec's *the set
    /// Smoozy has may be smaller* grant, because it is not smaller but wrong.
    ///
    /// **This is composition and nothing more** — no reading of the machine is
    /// added, no re-derivation is attempted, and nothing is written into
    /// `Dependency Upload & Remap`'s own per-job set, which that document holds
    /// *"immutable for the life of the job's record"*.
    /// **An absent member is skipped and the rest of the set stands.** A job
    /// nothing at all can be named for has an empty set and reaches no
    /// warning — which is the only case where nothing can be said, and it is
    /// said here rather than left to a caller.
    pub fn the_set(&self) -> Vec<&[u8]> {
        let mut members: Vec<&[u8]> = Vec::new();
        for member in [&self.blend, &self.job_folder].into_iter().flatten() {
            members.push(member);
        }
        if self.its_blend_was_disturbed {
            return members;
        }
        if let Some(resolved) = &self.resolved_set {
            for member in resolved {
                members.push(member);
            }
        }
        members
    }

    /// **W6's widest case, and Design's sentence rests on it**: whether
    /// Smoozy's set for this job is only the `.blend` and the job folder.
    ///
    /// **Two ways reach it and both are here**: a job whose file was never
    /// checked has no resolved set (W6), and a job whose own manifest `.blend`
    /// a Smoozy act disturbed falls back to the same set for the rest of its
    /// life (W5's fall-back).
    ///
    /// **It cannot be recovered from the members reached**, which is why it
    /// travels on [`AJobNamed`]: a fully checked job whose only reached member
    /// is its `.blend` looks identical from there.
    pub fn the_set_is_the_widest_case(&self) -> bool {
        self.its_blend_was_disturbed || self.resolved_set.is_none()
    }
}

/// **W1, W1a and W5's readings, and the whole of what §8 asks of anything.**
///
/// **Every method here is answered from the local record, at the moment a
/// destination is named, before any write, with no read of the machine** (W1).
/// Nothing on this trait may reach a connection, and a reader that did would
/// be answering a question W1 forbids.
///
/// **One reader value is one machine**: the server row and its current
/// installation identity are the reader's, so every answer below is already
/// scoped to them. **W1b's second limit rides on that** — every record of
/// those kinds is disqualified under a superseded installation identity
/// (`Dependency Upload & Remap` K7, and that module's own
/// `Dependency Upload & Remap` K10), so the class is *neither* on a recycled
/// machine and no line appears.
///
/// **The comparison the reader makes is X5's and nothing weaker**, which is
/// also `Dependency Upload & Remap` K3's own keying discipline: byte for byte,
/// per path component, whole and exact. W4 states the limit that rides with
/// it.
pub trait ReadsWhatTheLocalRecordKnows {
    /// `Dependency Upload & Remap` V2's points-at reading for one machine
    /// location.
    fn what_depended_on(&self, path: &[u8]) -> WhatDependedOnIt;

    /// **W1's reading asked for a folder victim**: the machine locations the
    /// local record holds **underneath** this path, each with V2's answer for
    /// it.
    ///
    /// **Why the reading is a second question rather than a wider first one.**
    /// V2 answers for a named machine location, and the records name files
    /// rather than the folders holding them, so a folder asked about by its own
    /// path alone answers nothing at all. **The cost is per location** — a
    /// folder victim walks the local record rather than asking one question —
    /// and that is stated in W1 rather than hidden here.
    ///
    /// **It stays local, which is the only limit W1 imposes**: no machine, no
    /// connection, no listing, and the folder's real contents are never read.
    /// **A path a file is at answers nothing**, because nothing sits inside a
    /// file — which is why [`the_delete_warning`] asks it of every victim and
    /// needs no flag saying which victims are folders.
    ///
    /// **The comparison is X5's**, per path component, exactly as
    /// [`crate::already_exists::is_inside`] makes it.
    fn what_the_record_holds_under(&self, folder: &[u8]) -> Vec<(Vec<u8>, WhatDependedOnIt)>;

    /// W1a's three-way class for one landing path.
    fn what_the_landing_path_is_for(&self, landing_path: &[u8]) -> WhatTheLandingPathIsFor;

    /// W5's live set for this machine, with each job's own set.
    fn the_live_jobs(&self) -> Vec<ALiveJob>;
}

/// **One machine location the record holds under a folder victim**, with what
/// depended on it (W1).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AnythingUnderTheVictim {
    /// The location, as the record spells it.
    pub location: Vec<u8>,
    pub what_depended_on_it: WhatDependedOnIt,
}

/// **W1's delete warning, for one victim.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ADeleteWarningLine {
    /// The victim the reading was taken for, as the act named it.
    pub victim: Vec<u8>,
    pub what_depended_on_it: WhatDependedOnIt,
    /// **What the record shows underneath the victim**, which is a folder
    /// victim's whole answer and a file victim's empty one.
    pub what_is_under_it: Vec<AnythingUnderTheVictim>,
}

impl ADeleteWarningLine {
    /// Whether this victim produces a line at all — its own location, or
    /// anything the record holds under it.
    pub fn anything_depended_on_it(&self) -> bool {
        self.what_depended_on_it.anything_depended_on_it() || !self.what_is_under_it.is_empty()
    }
}

/// **W1: the delete warning, answered from the local record at the moment the
/// victim is named, before any write, with no read of the machine.**
///
/// It names anything the points-at reading shows depended on, **whether or not
/// a job is live**. A victim nothing depended on produces no line, and where no
/// victim produces one the returned list is empty and no warning is shown.
///
/// **Where the victim is a folder, the reading is asked for the folder and the
/// warning names what the record shows underneath it.** Both questions are put
/// for every victim, and the second answers nothing for a file, because nothing
/// sits inside a file — so no flag saying which victims are folders is needed
/// or invented. **The reading stays local**: no machine, no connection, and the
/// folder's real contents are never read.
pub fn the_delete_warning(
    victims: &[Vec<u8>],
    reader: &dyn ReadsWhatTheLocalRecordKnows,
) -> Vec<ADeleteWarningLine> {
    victims
        .iter()
        .filter_map(|victim| {
            let line = ADeleteWarningLine {
                victim: victim.clone(),
                what_depended_on_it: reader.what_depended_on(victim),
                what_is_under_it: reader
                    .what_the_record_holds_under(victim)
                    .into_iter()
                    .map(|(location, what_depended_on_it)| AnythingUnderTheVictim {
                        location,
                        what_depended_on_it,
                    })
                    .collect(),
            };
            line.anything_depended_on_it().then_some(line)
        })
        .collect()
}

/// **One file the plain-upload warning names** (W1b).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct APlainUploadLine {
    /// The landing path the walk found occupied.
    pub landing_path: Vec<u8>,
    /// Its class under W1a — *placed* or *fixed*, never *neither*.
    pub class: WhatTheLandingPathIsFor,
}

/// **W1's plain-upload warning: the line, composed over the intersection and
/// never over the record alone** (W1b).
///
/// **The warning names the files that both collide** — X1's walk found the
/// landing name occupied in the destination listing — **and class *placed* or
/// *fixed* under W1a.**
///
/// **A record naming a path is never evidence that a file is there** (X13's
/// discipline), and nothing removes an asset from a machine by itself, so a
/// record outlives a file the user removed with their own tools; **such a path
/// produces no line**, because there is nothing to replace and no collision to
/// warn inside.
///
/// **What the warning offers beside the line is the route through
/// `Dependency Upload & Remap`**, and what the offer then does to the request
/// is [`crate::request::hand_over`]'s (U12 to U15). Nothing of the offer is
/// composed here.
pub fn the_plain_upload_warning(
    answer: &TheAnswer,
    reader: &dyn ReadsWhatTheLocalRecordKnows,
) -> Vec<APlainUploadLine> {
    answer
        .comparisons
        .iter()
        .filter(|comparison: &&AComparison| comparison.what.is_a_collision())
        .filter_map(|comparison| {
            let class = reader.what_the_landing_path_is_for(&comparison.landing_path);
            class.the_warning_names_it().then(|| APlainUploadLine {
                landing_path: comparison.landing_path.clone(),
                class,
            })
        })
        .collect()
}

/// **W1b: the files the line names are counted in X3's headline total and
/// named separately** — the spec's own arrangement, *"counted in this
/// already-exists total, and that warning names them separately"*.
///
/// So the headline is [`TheAnswer::colliding`] unchanged, and this function
/// exists to say that in code rather than to compute anything: a warned file is
/// already in it and is never added, subtracted or moved.
pub fn the_headline_total(answer: &TheAnswer) -> usize {
    answer.colliding
}

/// **W1: the overwrite warning fires on no plain upload of this document's,
/// and its trigger is not this document's.**
///
/// *"this clause counted it among the record-answered warnings while that
/// module's `Dependency Upload & Remap` Q8 fires it from a contents test that
/// reads the machine, and a builder could keep only one of those"* — corrected
/// in the plan on 2026-08-22, out of a scenario replay.
///
/// It is `Dependency Upload & Remap` Q8's, replace-or-stop and nothing wider,
/// **scoped to that module's own row sends**, fired from that module's own
/// `Dependency Upload & Remap` Q5 establishment inside the send pre-flight — a
/// reading the send decision takes anyway, never one added at warning time, and
/// **never one this document performs**.
///
/// **A plain upload's own collision question stays X3's and U4's** — Replace,
/// Keep both, Skip existing, or Stop
/// ([`crate::already_exists::CollisionAnswer`]) — with the plain-upload warning
/// beside it where the record names the landing place.
pub const THE_OVERWRITE_WARNING_FIRES_ON_NO_PLAIN_UPLOAD_OF_THIS_DOCUMENTS: bool = true;

/// **W2: the acts whose own gates the readings run at.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TheAct {
    AnUpload,
    ADelete,
    AMove,
    ARename,
    APaste,
}

impl TheAct {
    pub const ALL: [TheAct; 5] = [
        TheAct::AnUpload,
        TheAct::ADelete,
        TheAct::AMove,
        TheAct::ARename,
        TheAct::APaste,
    ];
}

/// **W2: the moments are the acts' own gates.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TheMoment {
    /// **For an upload, the readings run against the enumeration's landing
    /// paths during the pre-flight**, and what they name is presented with the
    /// already-exists question — the warned files counted in the headline total
    /// and named separately, placement Design's.
    InTheUploadsPreFlight,
    /// **For a delete, a move, a rename or a paste, the readings run when the
    /// destination (or the victim) is named, before the warning box.**
    WhenTheDestinationOrVictimIsNamed,
}

/// W2's assignment, as a lookup rather than a judgement.
pub fn the_moment_for(act: TheAct) -> TheMoment {
    match act {
        TheAct::AnUpload => TheMoment::InTheUploadsPreFlight,
        TheAct::ADelete | TheAct::AMove | TheAct::ARename | TheAct::APaste => {
            TheMoment::WhenTheDestinationOrVictimIsNamed
        }
    }
}

/// **W3: the warnings warn and none blocks** — the machine is the user's.
///
/// The answer flows into the act's ordinary mechanics: **a Replace answered at
/// the collision question (U4) is a replace-intended publication**
/// ([`crate::already_exists::publication_intent`]), and **a delete pushed past
/// the delete warning proceeds under M2** ([`crate::managing::the_delete_order`]),
/// with §8.4's telling where live jobs are named.
pub const NO_WARNING_OF_THIS_SECTION_BLOCKS: bool = true;

/// **W4: the record's locations are compared to destinations by exact recorded
/// bytes — X5's rule, which is also `Dependency Upload & Remap` K3's own
/// keying discipline.**
///
/// This is [`crate::already_exists::paths_are_equal`] and nothing beside it, so
/// there is one comparison in this crate rather than two.
pub fn the_record_matches(recorded: &[u8], the_destination: &[u8]) -> bool {
    paths_are_equal(recorded, the_destination)
}

/// **W4's limit, declared rather than papered over** (§13 item 5).
///
/// A symbolic link or a second mount makes one location into two, and the
/// comparison reads paths, not inodes: **a file reached by an alias the record
/// does not spell is not matched, and no warning fires for it.** Modelling the
/// machine's link structure would be the same trap as modelling its folding
/// rule.
///
/// **It is not the warnings' limit alone, and it rides on three writes rather
/// than two**: X7a's two publication writes and W11's *disturbed* mark all
/// compare recorded path text under the same rule and carry the same limit, at
/// a higher cost — stated at [`crate::publication`] and in §13 item 5.
///
/// **W11's is owed at this document's removals as well as its publications**,
/// so a delete reaching a location a checked root's tree resolved through an
/// alias the record does not spell leaves that standing unmarked too, with the
/// same ending and no second route to it.
pub const AN_ALIAS_THE_RECORD_DOES_NOT_SPELL_IS_NOT_MATCHED: bool = true;

/// **One job the live-job warning names**, with the members of its set the act
/// reaches.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AJobNamed {
    pub job: Identity,
    /// W5a: the warning may differentiate per job, and this is what on.
    pub stands: HowTheJobStands,
    /// The members of this job's set the disturbance reaches, as the record
    /// spells them — which is what `Queue & History` N21's mark carries.
    pub the_members_reached: Vec<Vec<u8>>,
    /// **Whether Smoozy's set for this job was only the `.blend` and the job
    /// folder** — W6's widest case, reached by a job never checked and by a job
    /// whose own `.blend` a Smoozy act disturbed.
    ///
    /// **Design requires the warning to say so where it is true**, *"rather
    /// than implying it checked more"*. **The sentence is Design's and nothing
    /// here composes it**: this field is the value the sentence is written
    /// from, and no wording of any kind is made in this crate.
    ///
    /// **It cannot be recovered from [`Self::the_members_reached`]** — a fully
    /// checked job whose only reached member is its `.blend` looks identical —
    /// which is why it travels rather than being derived at the screen.
    pub the_set_was_the_widest_case: bool,
}

/// **W5: the live-job warning.**
///
/// **The trigger is a containment test and it runs both ways: the warning fires
/// where the destination, the victim, or any path inside a folder being
/// disturbed equals a member of any live job's set, contains a member, or sits
/// inside a member, compared per component under X5's rule, every holding job
/// named** (`Dependency Upload & Remap` V6).
///
/// **The second direction is what covers the job's own folder**, which is a
/// member precisely because a running render's frames cannot be listed at
/// submission — so a frame deleted inside a live job's output folder names that
/// job. **Nothing has to say which members are folders**: a disturbed path sits
/// inside a member only where that member is a folder, because nothing sits
/// inside a file.
///
/// **The widening is in the comparison and nowhere else.** Listing a folder's
/// contents here would be a read of the machine, which W1 forbids outright, and
/// the comparison needs none.
///
/// **The whole set is composed here and the readings are the record's**:
/// R15's staleness rule is deliberately not consumed, and the omission is a
/// decision. W5 takes from `Job files and outputs` R15 the manifest `.blend`'s
/// path alone, and not the four-value identity of its R10 or the test of its
/// R13, because applying that test would mean reading the machine at the moment
/// a destination is named — which is exactly what W1 forbids for the warnings
/// it answers.
pub fn the_live_job_warning(
    the_paths_being_disturbed: &[Vec<u8>],
    reader: &dyn ReadsWhatTheLocalRecordKnows,
) -> Vec<AJobNamed> {
    reader
        .the_live_jobs()
        .into_iter()
        .filter_map(|job| {
            let the_members_reached: Vec<Vec<u8>> = job
                .the_set()
                .into_iter()
                .filter(|member| {
                    the_paths_being_disturbed.iter().any(|disturbed| {
                        // The member sits inside the thing being touched, or
                        // the thing being touched sits inside the member.
                        is_inside(member, disturbed) || is_inside(disturbed, member)
                    })
                })
                .map(<[u8]>::to_vec)
                .collect();
            if the_members_reached.is_empty() {
                return None;
            }
            let the_set_was_the_widest_case = job.the_set_is_the_widest_case();
            Some(AJobNamed {
                job: job.job,
                stands: job.stands,
                the_members_reached,
                the_set_was_the_widest_case,
            })
        })
        .collect()
}

/// **W5a: how wide the live-job warning's statement may be, and this document
/// supplies the bound rather than the words.**
///
/// Design has it *"telling the truth about each — a render already running may
/// finish anyway, and its frames may be wrong."*
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HowWideTheStatementMayBe {
    /// **Per job, the warning may differentiate**, and W5 supplies what on: a
    /// job that is rendering and a job that is waiting are told apart.
    pub per_job_it_may_differentiate: bool,
    /// **Per file, it may not, and that is `Dependency Upload & Remap` V8's
    /// decision taken at length**: *"what a touched file does to the running
    /// render is deliberately not modelled, and the warning is uniform because
    /// the behaviour is not"*, closing *"the warning covers every touch the
    /// same way and promises nothing about what the touch does."*
    ///
    /// **So the design's clause survives as a uniform *may* and not
    /// otherwise.** Naming the picture case as the consequence is a per-kind
    /// claim `Dependency Upload & Remap` V8 forbids.
    pub per_file_it_may_not: bool,
}

/// W5a's bound, as a value.
pub const THE_BOUND_ON_THE_STATEMENT: HowWideTheStatementMayBe = HowWideTheStatementMayBe {
    per_job_it_may_differentiate: true,
    per_file_it_may_not: false,
};

/// **W5b: the warning's truth is exactly the local record's truth, and on a
/// full mount the record cannot learn that a job has ended.**
///
/// W5 composes the live set from the records with state *waiting*, *held* or
/// *rendering*, and a job is live *until its recorded outcome is written*. **On
/// a mount that has filled, the recorded outcome is exactly the write that
/// cannot land**: `Rented-server runtime` S14 holds it pending in the run's
/// memory and re-attempts it at the next request the supervisor serves, so the
/// ending exists and is readable nowhere until space returns — and the act that
/// returns space is the user's own deleting, which is the act this warning fires
/// on. **So in that window a job that has ended reads live, and the warning
/// fires over it.**
///
/// **The honest word is *unverifiable* rather than *wrong*.** A render on a
/// full mount may still be running, and the one record that would tell a
/// running job from an ended one is the write that refuses, so nothing anywhere
/// holds the answer. §13 item 18 carries the cost.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TheUnverifiableWindow;

impl TheUnverifiableWindow {
    /// **Reading the machine at warning time is forbidden by W1 and would not
    /// help**: the machine's own queue record carries the same *rendering*, its
    /// ending being the pending write.
    pub const READING_THE_MACHINE_WOULD_RETURN_THE_SAME_ANSWER: bool = true;

    /// **Suppressing the warning while the supervisor-unavailable condition
    /// stands is refused** — a genuinely rendering job would then have its files
    /// disturbed with no warning at all, which is the harm W5 exists against,
    /// and the warning is non-dismissible on the Creator's own ruling (W7).
    pub const SUPPRESSING_IT_ON_A_FULL_MOUNT_IS_REFUSED: bool = true;

    /// **What holds, and it is the direction that matters**: the warning
    /// over-warns and never under-warns, so no genuinely rendering job has its
    /// files disturbed silently.
    pub const IT_OVER_WARNS_AND_NEVER_UNDER_WARNS: bool = true;

    /// **Whether the local record should carry a pending or unknown state
    /// instead is `Queue & History` §11's question and is deliberately not
    /// opened here.** It would not change what this warning does — a job whose
    /// ending is unverified must still be warned over, because it may be
    /// rendering.
    pub const A_PENDING_STATE_IS_ROUTED_AND_NOT_OPENED_HERE: bool = true;
}

/// **W6: a live job's set is a snapshot, fixed at submission and never
/// re-derived.**
///
/// The manifest and the remap material are immutable, so *what does this job
/// depend on* has one durable answer for the job's life
/// (`Dependency Upload & Remap` V5). **The record goes stale only by Smoozy's
/// own recorded acts, never by watching the machine** (its V6); a fresh root
/// check is required only where a file is submitted as a new root, which is
/// that module's V9 and no invalidation at all.
pub const A_LIVE_JOBS_SET_IS_A_SNAPSHOT_AND_IS_NEVER_RE_DERIVED: bool = true;

/// **W7: the warning stays wide, and this document does not re-take that
/// decision.**
///
/// Creator-stated, 2026-08-12: the evidence reaches a scene's material picture
/// and a linked `.blend` and nothing else, so **no narrowing to the rendering
/// scene's files.** The warning's non-dismissibility is Creator-stated too.
pub const THE_WARNING_IS_NOT_NARROWED_TO_THE_RENDERING_SCENES_FILES: bool = true;

/// **W7: no suppression row exists for the live-job warning** (M3).
///
/// `Desktop app and delivery` G3 binds every module to record its dismissible
/// warnings' suppressions in the one table, and G4 and G5 give that table its
/// shape; **this document's whole contribution is the delete warning's one
/// row**, which [`crate::declarations::THE_SUPPRESSION_ROWS`] holds. This asks
/// that rather than asserting it, so a row added for §8 fails here.
pub fn no_suppression_row_exists_for_the_live_job_warning() -> bool {
    crate::declarations::THE_SUPPRESSION_ROWS == ["file-browser.delete"]
}

/// **W8's telling: what one durable write lands before the disturbing act's
/// first machine effect.**
///
/// *"The disturb action's §9.4 record, enumerating the affected jobs (by job
/// identity) and the act intended. **Both are fields of that same record and
/// never of a second one beside it**, because §9.4 gives one outcome record per
/// action and the disturbing act is one action; for an upload that record is
/// U1's, whose field list names them."*
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TheTellingThatStandsFirst {
    /// The affected jobs, by job identity.
    pub the_jobs: Vec<Identity>,
    /// The act intended.
    pub the_act_intended: TheAct,
}

/// W8's composition, from the warning the user pushed past.
pub fn the_telling_for(named: &[AJobNamed], act: TheAct) -> TheTellingThatStandsFirst {
    TheTellingThatStandsFirst {
        the_jobs: named.iter().map(|job| job.job.clone()).collect(),
        the_act_intended: act,
    }
}

/// **W8's second half, and it is the ordered pair §11 declares.**
///
/// *"Where that write cannot be made, the act is **not attempted**, naming the
/// telling — §9.3's ordered pair: the disturbance without its mark is exactly
/// the unexplained-failure-hours-later harm the spec names, so the act is
/// unsafe unless the telling stood first."*
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TheActIsNotAttempted {
    /// **Naming the telling** — the *stopped by* field `State, identity and
    /// recovery` §9.3 gives a unit that was never tried, carrying the declared
    /// partition unit's own words.
    pub stopped_by: String,
}

/// **W8's ordering, which is a rule the caller keeps and not a thing the types
/// carry.** This function decides whether the act may be attempted, and its
/// failing arm names the telling rather than the act — but the witness it hands
/// back is returned and discarded, nothing takes one, and **a caller that never
/// tells at all compiles.**
///
/// **What would make the ordering true rather than stated**: the act taking
/// [`TheTellingStoodFirst`] by value, the way
/// [`crate::request::UploadRequest::confirmed`] already takes
/// [`crate::request::SendingFilesUpWasOpen`] — this crate's own pattern for
/// exactly this. **It is one parameter for whoever writes the acts**, and it is
/// not written here because there are no act functions to take it: nothing in
/// the application performs a disturbing act yet, and inventing the caller to
/// make a comment true is what this project has already refused twice.
///
/// **The danger the wrong comment carried is exact**: somebody wiring the
/// delete flow reads *kept by the types*, believes the ordering cannot be got
/// wrong, and performs the delete with no durable mark — which is the
/// unexplained-failure-hours-later harm the whole ordered pair exists against.
///
/// The declaration it keeps is [`crate::declarations::THE_PARTITION_UNITS`]'s
/// *a disturb pushed past the live-job warning* row — the (telling, act) pair,
/// `State, identity and recovery` §9.2 test 3.
pub fn the_act_may_be_attempted(
    the_telling_was_written: bool,
) -> Result<TheTellingStoodFirst, TheActIsNotAttempted> {
    if !the_telling_was_written {
        return Err(TheActIsNotAttempted {
            stopped_by: THE_TELLING.to_string(),
        });
    }
    Ok(TheTellingStoodFirst(()))
}

/// **The telling's own name, in the words §11's table declares it under.**
pub const THE_TELLING: &str = "the (telling, act) pair";

/// **W8 for an upload, joined to the record it belongs on rather than kept
/// beside it.**
///
/// *"For an upload that record is U1's, whose field list names them."* So this
/// puts the jobs on [`crate::request::UploadRequest::telling`] and writes the
/// request down — **one durable write, before the act's first machine
/// effect** — and where that write cannot be made the act is not attempted,
/// naming the telling.
///
/// **A request that disturbed nothing carries neither field and writes
/// nothing**: there is no telling where no live-job warning was pushed past,
/// and the act goes ahead as any upload does.
pub fn tell_before_the_upload(
    request: &mut crate::request::UploadRequest,
    named: &[AJobNamed],
    keeper: &mut dyn crate::request::KeepsTheRequest,
) -> Result<TheTellingStoodFirst, TheActIsNotAttempted> {
    if named.is_empty() {
        return Ok(TheTellingStoodFirst(()));
    }
    request.telling = Some(crate::request::TheTelling {
        the_jobs: named.iter().map(|job| job.job.clone()).collect(),
    });
    the_act_may_be_attempted(keeper.write_it_down(request).is_ok())
}

/// The witness that W8's write landed. It carries nothing: what it says is that
/// the telling stood before the act's first machine effect.
#[derive(Debug)]
pub struct TheTellingStoodFirst(());

/// **W9: what one unit of the disturbing act did to one member.**
///
/// **Took effect means published, for an upload's Replace, and removed, for a
/// delete** — so it is [`UnitOutcome::Done`] and neither of the other two. **An
/// absent outcome is a unit whose ending was never written**, which is the
/// connection dying around the delete.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AUnitsOutcome {
    /// The member of a live job's set the unit touched, as the record spells
    /// it.
    pub the_member: Vec<u8>,
    /// The unit's own outcome, absent where its ending was never written.
    pub outcome: Option<UnitOutcome>,
}

/// **W9's composed reading, which is the field `Queue & History` carries onto
/// the entry** — its N21 disturbance mark, wording Design's.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TheComposedReading {
    /// Intent plus confirmed outcome: *a file this job depends on was
    /// disturbed*.
    Disturbed,
    /// Intent whose act's outcome was never learned — the connection died
    /// around the delete: *may have been disturbed*, which is the truth.
    MayHaveBeenDisturbed,
}

/// **W9: the act's outcome completes the mark, per job and from that job's own
/// units, and uncertainty reads as uncertainty.**
///
/// An upload with Replace and a multi-file delete are both partial across their
/// units (§11), so the act's own outcome is §9.4's *"derived from the list of
/// unit outcomes and never a separate durable field"* and **there is no single
/// outcome to write against anything.** So what is written against a job is
/// composed from the outcomes of the units that touched **that job's own
/// members** — the file it depends on, not some other job's — and **a job none
/// of whose members' units took effect carries no mark at all.**
///
/// Without that, a request whose units disagreed would mark jobs whose files
/// never moved, and the mark whose whole purpose is to explain a failure hours
/// later would point the user at a file that was never touched.
///
/// **The failure of the telling after the disturbance cannot arise**, because
/// the telling came first; the only residue is a mark for an act that then
/// never ran, and the composed reading covers it without lying — every unit
/// ended and none took effect, so no mark is written.
pub fn what_is_written_against(
    job: &AJobNamed,
    units: &[AUnitsOutcome],
) -> Option<TheComposedReading> {
    let its_own: Vec<&AUnitsOutcome> = units
        .iter()
        .filter(|unit| {
            job.the_members_reached
                .iter()
                .any(|member| paths_are_equal(member, &unit.the_member))
        })
        .collect();
    if its_own
        .iter()
        .any(|unit| unit.outcome == Some(UnitOutcome::Done))
    {
        return Some(TheComposedReading::Disturbed);
    }
    if its_own.iter().any(|unit| unit.outcome.is_none()) {
        return Some(TheComposedReading::MayHaveBeenDisturbed);
    }
    None
}

/// **W10: the mark's durability is the local record's, which is the gap it
/// exists to span.**
///
/// It survives Smoozy closing, the connection dropping and the machine being
/// destroyed, and **it is readable when the job fails hours later in a later
/// sitting** — the entry, and history after it, carry the fields unchanged
/// (`Queue & History`'s supply). **Nothing about the mark stops the user**; the
/// warning was theirs to overrule and the mark records that they did.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WhatTheMarkSurvives {
    pub smoozy_closing: bool,
    pub the_connection_dropping: bool,
    pub the_machine_being_destroyed: bool,
}

/// W10, as a value.
pub const THE_MARK_SURVIVES: WhatTheMarkSurvives = WhatTheMarkSurvives {
    smoozy_closing: true,
    the_connection_dropping: true,
    the_machine_being_destroyed: true,
};

/// **W10: nothing about the mark stops the user.**
pub const THE_MARK_STOPS_NOTHING: bool = true;

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::already_exists::Comparison;
    use smoozy_state::identities::IdentityKind;

    fn a_job() -> Identity {
        Identity::issue(IdentityKind::Job).expect("randomness")
    }

    /// A reader written by hand: **the whole of §8 is testable at the desk
    /// because W1 forbids the machine**, so this double is the shape of the
    /// real answer rather than a stand-in for a connection.
    #[derive(Default)]
    pub(crate) struct ARecord {
        pub depended_on: Vec<(Vec<u8>, WhatDependedOnIt)>,
        pub classes: Vec<(Vec<u8>, WhatTheLandingPathIsFor)>,
        pub jobs: Vec<ALiveJob>,
    }

    impl ReadsWhatTheLocalRecordKnows for ARecord {
        fn what_depended_on(&self, path: &[u8]) -> WhatDependedOnIt {
            self.depended_on
                .iter()
                .find(|(at, _)| paths_are_equal(at, path))
                .map(|(_, what)| what.clone())
                .unwrap_or_default()
        }
        /// The record's own locations that sit under the path, per component.
        /// **Nothing is listed and no machine is reached**, which is the whole
        /// of what this reading may do.
        fn what_the_record_holds_under(&self, folder: &[u8]) -> Vec<(Vec<u8>, WhatDependedOnIt)> {
            self.depended_on
                .iter()
                .filter(|(at, _)| is_inside(at, folder) && !paths_are_equal(at, folder))
                .filter(|(_, what)| what.anything_depended_on_it())
                .map(|(at, what)| (at.clone(), what.clone()))
                .collect()
        }
        fn what_the_landing_path_is_for(&self, landing_path: &[u8]) -> WhatTheLandingPathIsFor {
            self.classes
                .iter()
                .find(|(at, _)| paths_are_equal(at, landing_path))
                .map(|(_, class)| *class)
                .unwrap_or(WhatTheLandingPathIsFor::Neither)
        }
        fn the_live_jobs(&self) -> Vec<ALiveJob> {
            self.jobs.clone()
        }
    }

    fn an_answer(comparisons: Vec<(&[u8], Comparison)>) -> TheAnswer {
        let colliding = comparisons
            .iter()
            .filter(|(_, what)| what.is_a_collision())
            .count();
        TheAnswer {
            comparisons: comparisons
                .into_iter()
                .map(|(landing_path, what)| AComparison {
                    landing_path: landing_path.to_vec(),
                    what,
                })
                .collect(),
            colliding,
            arriving_files: 0,
            as_of: std::time::SystemTime::UNIX_EPOCH,
            readings_taken: 1,
        }
    }

    /// **W1: the delete warning names anything the points-at reading shows
    /// depended on, whether or not a job is live.**
    #[test]
    fn the_delete_warning_names_what_depended_on_the_victim_with_no_live_job_anywhere() {
        let victim = b"/root/assets/files/Users/a/tex/x.png".to_vec();
        let record = ARecord {
            depended_on: vec![(
                victim.clone(),
                WhatDependedOnIt {
                    fixed_copies_whose_fixing_points_at_it: vec![
                        b"/root/assets/fixed/c.blend".to_vec()
                    ],
                    placed_from: vec![b"/Users/a/tex/x.png".to_vec()],
                    jobs_whose_set_names_it: vec![],
                },
            )],
            ..ARecord::default()
        };
        let lines = the_delete_warning(std::slice::from_ref(&victim), &record);
        assert_eq!(lines.len(), 1);
        assert_eq!(lines[0].victim, victim);
        assert!(lines[0].what_depended_on_it.anything_depended_on_it());

        // A victim nothing depended on produces no line.
        assert!(the_delete_warning(&[b"/root/elsewhere".to_vec()], &record).is_empty());
    }

    /// **W1: where the victim is a folder, the warning names what the record
    /// shows underneath it.**
    ///
    /// The reading is per location and stays local — nothing is listed and no
    /// machine is reached — which is the only limit W1 imposes.
    #[test]
    fn a_folder_victim_names_what_the_record_holds_underneath_it() {
        let one = b"/root/assets/files/Users/a/kitchen/wood.png".to_vec();
        let two = b"/root/assets/files/Users/a/kitchen/tile.png".to_vec();
        let elsewhere = b"/root/assets/files/Users/a/bath/tap.png".to_vec();
        let placed = |from: &[u8]| WhatDependedOnIt {
            placed_from: vec![from.to_vec()],
            ..WhatDependedOnIt::default()
        };
        let record = ARecord {
            depended_on: vec![
                (one.clone(), placed(b"/Users/a/tex/wood.png")),
                (two.clone(), placed(b"/Users/a/tex/tile.png")),
                (elsewhere, placed(b"/Users/a/tex/tap.png")),
            ],
            ..ARecord::default()
        };

        let folder = b"/root/assets/files/Users/a/kitchen".to_vec();
        let lines = the_delete_warning(std::slice::from_ref(&folder), &record);
        assert_eq!(lines.len(), 1);
        assert_eq!(lines[0].victim, folder);
        // The folder's own location depended on nothing — the records name
        // files, not the folders holding them — and the whole answer is
        // underneath it.
        assert!(!lines[0].what_depended_on_it.anything_depended_on_it());
        let named: Vec<Vec<u8>> = lines[0]
            .what_is_under_it
            .iter()
            .map(|under| under.location.clone())
            .collect();
        assert_eq!(named, vec![one.clone(), two]);

        // A file victim answers on its own location alone, because nothing
        // sits inside a file — so no flag saying which victims are folders is
        // needed.
        let lines = the_delete_warning(std::slice::from_ref(&one), &record);
        assert_eq!(lines.len(), 1);
        assert!(lines[0].what_depended_on_it.anything_depended_on_it());
        assert!(lines[0].what_is_under_it.is_empty());

        // A folder the record holds nothing under produces no line.
        assert!(the_delete_warning(&[b"/root/assets/files/Users/a/empty".to_vec()], &record)
            .is_empty());
    }

    /// **W6 and W5b: a job Smoozy cannot fully name is narrowed and not
    /// dropped**, because a dropped job is invisible to the warning and that
    /// under-warns.
    #[test]
    fn a_job_missing_its_blend_or_its_folder_keeps_the_members_that_can_be_named() {
        let whole = a_live_job(a_job(), HowTheJobStands::Rendering);

        let no_blend = ALiveJob {
            blend: None,
            ..whole.clone()
        };
        assert_eq!(
            no_blend.the_set(),
            vec![
                &b"/root/jobs/abc"[..],
                &b"/root/assets/files/Users/a/tex/x.png"[..],
                &b"/root/assets/remaps/abc"[..],
            ]
        );

        let no_folder = ALiveJob {
            job_folder: None,
            ..whole.clone()
        };
        assert_eq!(
            no_folder.the_set(),
            vec![
                &b"/root/assets/files/Users/a/scene.blend"[..],
                &b"/root/assets/files/Users/a/tex/x.png"[..],
                &b"/root/assets/remaps/abc"[..],
            ]
        );

        // Both still reach the warning over what they do name.
        let record = ARecord {
            jobs: vec![no_blend, no_folder],
            ..ARecord::default()
        };
        assert_eq!(
            the_live_job_warning(&[b"/root/assets/files/Users/a/tex/x.png".to_vec()], &record).len(),
            2
        );

        // **And what happens to a job nothing at all can be composed for, said
        // out loud**: its set is empty, so no path reaches it and no warning
        // names it. That is the one case where there is nothing to say, and it
        // is the whole of what the two absences cost.
        let nothing_nameable = ALiveJob {
            blend: None,
            job_folder: None,
            resolved_set: None,
            its_blend_was_disturbed: false,
            ..whole
        };
        assert!(nothing_nameable.the_set().is_empty());
        let record = ARecord {
            jobs: vec![nothing_nameable],
            ..ARecord::default()
        };
        assert!(the_live_job_warning(&[b"/root".to_vec()], &record).is_empty());
    }

    /// **W6's widest case travels with the warning**, because Design's sentence
    /// cannot be written from the members reached.
    #[test]
    fn the_warning_carries_whether_the_set_was_only_the_blend_and_the_folder() {
        let disturbed = ALiveJob {
            its_blend_was_disturbed: true,
            ..a_live_job(a_job(), HowTheJobStands::Rendering)
        };
        let never_checked = ALiveJob {
            resolved_set: None,
            ..a_live_job(a_job(), HowTheJobStands::Waiting)
        };
        let checked = a_live_job(a_job(), HowTheJobStands::Waiting);
        assert!(disturbed.the_set_is_the_widest_case());
        assert!(never_checked.the_set_is_the_widest_case());
        assert!(!checked.the_set_is_the_widest_case());

        let record = ARecord {
            jobs: vec![disturbed, never_checked, checked],
            ..ARecord::default()
        };
        // All three reach the warning over the `.blend`, and the two widest
        // cases are told apart from the checked one there.
        let named = the_live_job_warning(
            &[b"/root/assets/files/Users/a/scene.blend".to_vec()],
            &record,
        );
        assert_eq!(named.len(), 3);
        assert!(named[0].the_set_was_the_widest_case);
        assert!(named[1].the_set_was_the_widest_case);
        assert!(
            !named[2].the_set_was_the_widest_case,
            "a fully checked job whose only reached member is its blend looks \
             identical from the members reached, which is why the field exists"
        );
    }

    /// **W1b: the line is composed over the intersection and never over the
    /// record alone.** A record naming a path the walk found nothing at
    /// produces no line.
    #[test]
    fn the_plain_upload_line_needs_both_a_collision_and_a_class() {
        let collided = b"/root/assets/files/Users/a/tex/x.png";
        let fresh = b"/root/assets/files/Users/a/tex/y.png";
        let plain = b"/root/other.png";
        let record = ARecord {
            classes: vec![
                (collided.to_vec(), WhatTheLandingPathIsFor::Placed),
                // A record outlives a file the user removed with their own
                // tools: this path is classed and nothing collided at it.
                (fresh.to_vec(), WhatTheLandingPathIsFor::Fixed),
            ],
            ..ARecord::default()
        };
        let answer = an_answer(vec![
            (collided, Comparison::CollidesWithAFile),
            (fresh, Comparison::LandsFresh),
            (plain, Comparison::CollidesWithAFile),
        ]);
        let lines = the_plain_upload_warning(&answer, &record);
        assert_eq!(lines.len(), 1);
        assert_eq!(lines[0].landing_path, collided.to_vec());
        assert_eq!(lines[0].class, WhatTheLandingPathIsFor::Placed);

        // **The warned file is counted in the headline and never added to
        // it**: two collisions, one of them warned.
        assert_eq!(the_headline_total(&answer), 2);
    }

    /// **W1b's second limit: a recycled machine's records are disqualified, so
    /// the class is *neither* and no line appears.** The reader is one
    /// machine's, so a disqualified record simply answers nothing.
    #[test]
    fn a_recycled_machine_answers_neither_and_no_line_appears() {
        let collided = b"/root/assets/files/Users/a/tex/x.png";
        let record = ARecord::default();
        let answer = an_answer(vec![(collided, Comparison::CollidesWithAFile)]);
        assert_eq!(
            record.what_the_landing_path_is_for(collided),
            WhatTheLandingPathIsFor::Neither
        );
        assert!(the_plain_upload_warning(&answer, &record).is_empty());
        const { assert!(THE_COPY_A_FIXED_COPY_WAS_DERIVED_FROM_IS_NOT_IN_THE_CLASS) };
        const { assert!(THE_OVERWRITE_WARNING_FIRES_ON_NO_PLAIN_UPLOAD_OF_THIS_DOCUMENTS) };
        const { assert!(NO_WARNING_OF_THIS_SECTION_BLOCKS) };
    }

    /// **W2: the moments are the acts' own gates**, and four of the five share
    /// one.
    #[test]
    fn the_upload_reads_in_its_pre_flight_and_the_other_four_when_the_target_is_named() {
        assert_eq!(
            the_moment_for(TheAct::AnUpload),
            TheMoment::InTheUploadsPreFlight
        );
        for act in [
            TheAct::ADelete,
            TheAct::AMove,
            TheAct::ARename,
            TheAct::APaste,
        ] {
            assert_eq!(
                the_moment_for(act),
                TheMoment::WhenTheDestinationOrVictimIsNamed
            );
        }
        assert_eq!(TheAct::ALL.len(), 5);
    }

    fn a_live_job(job: Identity, stands: HowTheJobStands) -> ALiveJob {
        ALiveJob {
            job,
            stands,
            blend: Some(b"/root/assets/files/Users/a/scene.blend".to_vec()),
            job_folder: Some(b"/root/jobs/abc".to_vec()),
            resolved_set: Some(vec![
                b"/root/assets/files/Users/a/tex/x.png".to_vec(),
                b"/root/assets/remaps/abc".to_vec(),
            ]),
            its_blend_was_disturbed: false,
        }
    }

    /// **W5: the trigger is a containment test that runs both ways — the path
    /// being disturbed equals a member, contains a member, or sits inside
    /// one — and every holding job is named.**
    #[test]
    fn the_live_job_warning_fires_on_equality_and_on_containment_and_names_every_holder() {
        let one = a_job();
        let other = a_job();
        let record = ARecord {
            jobs: vec![
                a_live_job(one.clone(), HowTheJobStands::Rendering),
                a_live_job(other.clone(), HowTheJobStands::Waiting),
            ],
            ..ARecord::default()
        };

        // Equality: the shared texture itself.
        let named =
            the_live_job_warning(&[b"/root/assets/files/Users/a/tex/x.png".to_vec()], &record);
        assert_eq!(named.len(), 2);
        assert_eq!(named[0].job, one);
        assert_eq!(named[1].job, other);
        // **W5a: per job the warning may differentiate**, and the state is
        // what it differentiates on.
        assert_eq!(named[0].stands, HowTheJobStands::Rendering);
        assert_eq!(named[1].stands, HowTheJobStands::Waiting);

        // Containment: a folder that holds the member.
        let named = the_live_job_warning(&[b"/root/assets/files".to_vec()], &record);
        assert_eq!(named.len(), 2);

        // The job folder is in the set, so deleting it fires.
        let named = the_live_job_warning(&[b"/root/jobs/abc".to_vec()], &record);
        assert_eq!(named.len(), 2);

        // **The other direction, which the test used to fix the wrong way
        // round**: a finished frame deleted inside a live job's own output
        // folder names that job. The frame is no member of any set — a running
        // render's frames cannot be listed at submission, which is why the
        // folder is a member — and the disturbed path sits inside it.
        let named = the_live_job_warning(&[b"/root/jobs/abc/0042.png".to_vec()], &record);
        assert_eq!(named.len(), 2);
        assert_eq!(named[0].the_members_reached, vec![b"/root/jobs/abc".to_vec()]);

        // The same direction over the `.blend`'s own folder reaches it too.
        let named = the_live_job_warning(
            &[b"/root/assets/files/Users/a/scene.blend/nothing".to_vec()],
            &record,
        );
        assert_eq!(named.len(), 2);

        // A path that merely begins with a member's text is not inside it, in
        // either direction.
        assert!(the_live_job_warning(&[b"/root/jobs/abcdef".to_vec()], &record).is_empty());
        assert!(the_live_job_warning(&[b"/root/jobs/abcdef/0042.png".to_vec()], &record).is_empty());
    }

    /// **W5's fall-back and W6's widest case, which are the same set arriving
    /// two ways.**
    #[test]
    fn a_job_whose_blend_was_disturbed_and_a_job_never_checked_both_fall_back_to_the_widest_case() {
        let disturbed = ALiveJob {
            its_blend_was_disturbed: true,
            ..a_live_job(a_job(), HowTheJobStands::Rendering)
        };
        let never_checked = ALiveJob {
            resolved_set: None,
            ..a_live_job(a_job(), HowTheJobStands::Waiting)
        };
        for job in [&disturbed, &never_checked] {
            assert_eq!(
                job.the_set(),
                vec![
                    &b"/root/assets/files/Users/a/scene.blend"[..],
                    &b"/root/jobs/abc"[..]
                ],
                "the widest case is the manifest's blend and the job folder, and no resolved set"
            );
        }
        let record = ARecord {
            jobs: vec![disturbed, never_checked],
            ..ARecord::default()
        };
        // The texture is no longer in either set, so no warning names it.
        assert!(
            the_live_job_warning(&[b"/root/assets/files/Users/a/tex/x.png".to_vec()], &record)
                .is_empty()
        );
        // The blend still is.
        assert_eq!(
            the_live_job_warning(
                &[b"/root/assets/files/Users/a/scene.blend".to_vec()],
                &record
            )
            .len(),
            2
        );
        const { assert!(A_LIVE_JOBS_SET_IS_A_SNAPSHOT_AND_IS_NEVER_RE_DERIVED) };
    }

    /// **W5a's bound, and W5b's four refusals.**
    #[test]
    fn the_statement_is_uniform_across_files_and_differentiated_across_jobs() {
        const { assert!(THE_BOUND_ON_THE_STATEMENT.per_job_it_may_differentiate) };
        const { assert!(!THE_BOUND_ON_THE_STATEMENT.per_file_it_may_not) };
        const { assert!(TheUnverifiableWindow::READING_THE_MACHINE_WOULD_RETURN_THE_SAME_ANSWER) };
        const { assert!(TheUnverifiableWindow::SUPPRESSING_IT_ON_A_FULL_MOUNT_IS_REFUSED) };
        const { assert!(TheUnverifiableWindow::IT_OVER_WARNS_AND_NEVER_UNDER_WARNS) };
        const { assert!(TheUnverifiableWindow::A_PENDING_STATE_IS_ROUTED_AND_NOT_OPENED_HERE) };
    }

    /// **W5b in code: a job whose ending could not be recorded reads live, and
    /// the warning fires over it.** The record says *rendering* because the
    /// write that would say otherwise is the one the full mount refused.
    #[test]
    fn a_job_whose_ending_could_not_be_recorded_still_reads_live_and_is_warned_over() {
        let job = a_job();
        let record = ARecord {
            jobs: vec![a_live_job(job.clone(), HowTheJobStands::Rendering)],
            ..ARecord::default()
        };
        let named = the_live_job_warning(&[b"/root/assets/files".to_vec()], &record);
        assert_eq!(named.len(), 1);
        assert_eq!(named[0].job, job);
    }

    /// **W7: the warning is not dismissible and no suppression row exists for
    /// it.**
    #[test]
    fn no_suppression_row_is_written_for_the_live_job_warning() {
        assert!(no_suppression_row_exists_for_the_live_job_warning());
        const { assert!(THE_WARNING_IS_NOT_NARROWED_TO_THE_RENDERING_SCENES_FILES) };
        const { assert!(AN_ALIAS_THE_RECORD_DOES_NOT_SPELL_IS_NOT_MATCHED) };
    }

    /// **W4: the comparison is X5's and one alias away from matching.**
    #[test]
    fn the_record_matches_by_exact_recorded_bytes_and_never_through_an_alias() {
        assert!(the_record_matches(
            b"/root/assets//x.png",
            b"/root/assets/x.png"
        ));
        assert!(!the_record_matches(
            b"/root/assets/X.png",
            b"/root/assets/x.png"
        ));
        // A second mount reaching the same file is a different path and is not
        // matched — the limit W4 declares rather than repairs.
        assert!(!the_record_matches(
            b"/mnt/two/x.png",
            b"/root/assets/x.png"
        ));
    }

    /// **W8: the telling precedes the act, and where it cannot be written the
    /// act is not attempted, naming the telling.**
    #[test]
    fn the_act_is_not_attempted_where_the_telling_could_not_be_written() {
        let one = a_job();
        let named = vec![AJobNamed {
            job: one.clone(),
            stands: HowTheJobStands::Rendering,
            the_members_reached: vec![b"/root/assets/files/Users/a/tex/x.png".to_vec()],
            the_set_was_the_widest_case: false,
        }];
        let telling = the_telling_for(&named, TheAct::ADelete);
        assert_eq!(telling.the_jobs, vec![one]);
        assert_eq!(telling.the_act_intended, TheAct::ADelete);

        assert!(the_act_may_be_attempted(true).is_ok());
        let stopped = the_act_may_be_attempted(false).expect_err("the act is not attempted");
        assert_eq!(stopped.stopped_by, THE_TELLING);
        // **The declaration this keeps is §11's own row**, and the words are
        // that row's rather than composed here.
        assert!(crate::declarations::THE_PARTITION_UNITS
            .iter()
            .any(|row| row.partition_unit == THE_TELLING && row.clause == "W8"));
    }

    /// **W9: the mark is composed per job from that job's own units.**
    #[test]
    fn a_job_none_of_whose_members_units_took_effect_carries_no_mark() {
        let mine = b"/root/assets/files/Users/a/tex/x.png";
        let somebody_elses = b"/root/assets/files/Users/b/y.png";
        let job = AJobNamed {
            job: a_job(),
            stands: HowTheJobStands::Rendering,
            the_members_reached: vec![mine.to_vec()],
            the_set_was_the_widest_case: false,
        };

        // Another job's unit took effect and mine did not: no mark.
        let units = vec![
            AUnitsOutcome {
                the_member: mine.to_vec(),
                outcome: Some(UnitOutcome::Failed),
            },
            AUnitsOutcome {
                the_member: somebody_elses.to_vec(),
                outcome: Some(UnitOutcome::Done),
            },
        ];
        assert_eq!(what_is_written_against(&job, &units), None);

        // Mine took effect: disturbed.
        let units = vec![AUnitsOutcome {
            the_member: mine.to_vec(),
            outcome: Some(UnitOutcome::Done),
        }];
        assert_eq!(
            what_is_written_against(&job, &units),
            Some(TheComposedReading::Disturbed)
        );

        // The connection died around the delete: may have been disturbed.
        let units = vec![AUnitsOutcome {
            the_member: mine.to_vec(),
            outcome: None,
        }];
        assert_eq!(
            what_is_written_against(&job, &units),
            Some(TheComposedReading::MayHaveBeenDisturbed)
        );

        // A unit not attempted is not one that took effect.
        let units = vec![AUnitsOutcome {
            the_member: mine.to_vec(),
            outcome: Some(UnitOutcome::NotAttempted {
                stopped_by: "an earlier unit".to_string(),
            }),
        }];
        assert_eq!(what_is_written_against(&job, &units), None);
    }

    /// **W10: the mark spans the gap it exists for, and stops nothing.**
    #[test]
    fn the_mark_survives_the_close_the_drop_and_the_machine() {
        const { assert!(THE_MARK_SURVIVES.smoozy_closing) };
        const { assert!(THE_MARK_SURVIVES.the_connection_dropping) };
        const { assert!(THE_MARK_SURVIVES.the_machine_being_destroyed) };
        const { assert!(THE_MARK_STOPS_NOTHING) };
    }
}
