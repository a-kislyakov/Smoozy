//! §7 — staging, integrity, resume and publication, **one contract in both
//! directions**.
//!
//! **The contract, stated once** (§7.1): part-arrived bytes never sit at the
//! final name; integrity and resume are measured against staged material,
//! never against the final name and never against a file's presence, name or
//! size; publication is the last step and it is one step; abandoned staging is
//! cleaned up.
//!
//! **Why clause 1 is absolute rather than a matter of degree.** The machine
//! supplies nothing this promise can read: a killed or dropped copy leaves
//! bytes at the final name with *"identical mode, permissions, extended
//! attributes, inode flags, allocation and file-type output"* to a whole file,
//! and an interrupted overwrite left a **40,000,000-byte hybrid** — *"Neither
//! version, plausible size, no marker"*. **A hybrid is the failure this clause
//! exists to make impossible**, and staging plus a single publication is the
//! only arrangement that does it — never ordering, never care.
//!
//! **Watched happening, by the route that looks safest.** A 99 GB filesystem
//! filled to leave about 41 MiB, a 209,715,200-byte file pushed in by both
//! routes: Smoozy's own byte loop into a staging name Smoozy chose left
//! **41,930,752 bytes** at the staging path and **the final name absent**;
//! `scp` writing straight at the final name exited 1 and left **261,120 bytes
//! of 209,715,200 at the final name**. *"A partial file sitting at the name a
//! user would take for a finished one, produced by a tool nobody would
//! suspect, on a machine that reported the failure loudly."*
//!
//! **This module is split the way the plans are.** What decides — the record,
//! the endings, the staging path, the resume plan, the recovery verdict, the
//! sweep's match — is here and next door in [`resume`], [`publication`] and
//! [`cleanup`], and every one of it is testable at the desk on values written
//! by hand. What acts is [`moving`], over the subsystem and the platform file
//! adapter.
//!
//! **Markers used below that belong to another document, each with the
//! document it belongs to.** `File Browser` U5. `Desktop app and delivery`
//! R15b.

pub mod cleanup;
pub mod integrity;
pub mod moving;
pub mod publication;
pub mod resume;

use smoozy_state::identities::Identity;

use integrity::Digest;

/// Which way the bytes go. **The contract does not change between them** and
/// neither does this module; what differs is only which end the staging path
/// sits on.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    /// The user's own computer to the machine.
    Up,
    /// The machine to the user's own computer.
    Down,
}

/// **S26: a transfer ends in exactly one of five ways, the record says which,
/// and the five do different things with the staged material.**
///
/// **Raised from three to five on 2026-08-31**, *failed* and *stalled* having
/// been separated out of *interrupted*. Recording an integrity failure as
/// *interrupted* put it under the one word that means *this may be carried
/// on*, and cost two things at once: C11 step 4 picked it up at the next
/// repair and re-sent the whole file with nothing said, and `File Browser` U5
/// showed it as *waiting for the connection* on a live and healthy line.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Ending {
    /// The transfer reached the final name. **The staging material is gone,
    /// taken by the rename itself** — observed: the staging name was gone.
    Published,
    /// **Stopped on a deliberate instruction, and the instruction need not be
    /// the user's.** Smoozy's own arrests write this row too, and the reason is
    /// what names who stopped it.
    Stopped(StoppedReason),
    /// **The digests disagree at the end** (S24).
    Failed,
    /// **The transfer channel's inactivity deadline expired** (C19). It is a
    /// statement about the channel and never about the connection.
    ///
    /// **G8: a stalled channel is not a broken connection and neither is
    /// inferred from the other.** A caller's own line pulled went undetected
    /// for **21 minutes 45 seconds** with keepalives off, while a machine
    /// closing politely and a provider's own stop were both announced in the
    /// same second — so a channel's silence cannot stand in for a
    /// connection's. The connection's observed value is `Servers`', and
    /// nothing in this module writes or reads one. **A stalled channel is
    /// reported as a stalled channel.**
    Stalled,
    /// **A drop, a damaged pipe, the process ending.**
    ///
    /// **S26c: *interrupted* here names a transfer's ending and nothing
    /// else.** Everything this document says about the word binds a transfer
    /// record and reaches no other record anywhere. `Queue & History` N1's
    /// *interrupted and re-run* is a mark on a render attempt, and it is not
    /// this ending, not derived from it, and not a consumer of it. **No module
    /// may resume a transfer from a render attempt's mark, mark a render
    /// attempt from a transfer's ending, or compose one presentation over
    /// both.**
    Interrupted,
}

/// Why a transfer was stopped deliberately.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StoppedReason {
    /// **`File Browser` initiated it** — the user removed the staging.
    StagingRemovedByUser,
    /// **S28a.** The module that arranged the transfer abandoned the staging
    /// rather than taking it up. Writing this is what puts the material on the
    /// sweep's list; without it *"an ordinary **Skip existing** leaves every
    /// leftover partial on the rental for the life of the installation"*.
    AbandonedByItsRequest,
    /// **S13's upload half, corrected 2026-08-31.** The source changed under
    /// the transfer, so it stops and is reported rather than starting over: a
    /// silent restart *"re-sends a whole file over the user's own line for a
    /// save the user made, says nothing about the save, and drops the offer
    /// Row 201's third clause promises"*.
    TheSourceChangedUnderIt,
}

impl Ending {
    /// **S26b: *interrupted* is the only ending that means the work is waiting
    /// to be carried on, and nothing may present any other ending as
    /// waiting.**
    ///
    /// C11 step 4 resumes *interrupted* and no other ending, and no composed
    /// presentation may show a *failed* or a *stalled* transfer as waiting for
    /// a connection — on both endings the connection is up and the work is
    /// over.
    pub fn may_the_work_be_carried_on(&self) -> bool {
        matches!(self, Ending::Interrupted)
    }

    /// What becomes of the staged material.
    ///
    /// **The two that keep it and the three that do not are not the same
    /// split as the one above**, which is why S26's table has both columns:
    /// *published* takes the material by publishing it, and *interrupted*
    /// keeps it for a resume.
    pub fn what_becomes_of_the_staged_material(&self) -> StagedMaterial {
        match self {
            Ending::Published => StagedMaterial::TakenByTheRenameItself,
            Ending::Stopped(_) | Ending::Failed | Ending::Stalled => StagedMaterial::Removed,
            Ending::Interrupted => StagedMaterial::Kept,
        }
    }

    /// **C23: a stop and a drop are never told apart from the wire, so the
    /// record is written before the stop is issued.** This says which endings
    /// that ordering binds — the ones whose material is removed, where *"a
    /// process that dies mid-stop leaves material the next connection would
    /// resume as though the user had never stopped it"*.
    pub fn the_record_is_written_before_the_channel_closes(&self) -> bool {
        matches!(
            self.what_becomes_of_the_staged_material(),
            StagedMaterial::Removed
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StagedMaterial {
    TakenByTheRenameItself,
    Removed,
    Kept,
}

/// **S4: the staging path is synthesised from the transfer identity and the
/// transfer's attempt number, and never from the file's own name.**
///
/// **The attempt number was added 2026-08-31**: the identity alone was stated
/// as the whole of the synthesis while S12 step 5 gave one identity a second,
/// fresh staging path, so *the path synthesised from this transfer identity*
/// named two objects. **One path per attempt, one attempt live at a time.**
///
/// **Deriving it from the file's name would put Fact 29's hazards inside the
/// mechanism that is meant to survive them**: 33 hostile names reached the
/// machine byte-for-byte, but the Mac collapsed `Capitals.txt` and
/// `capitals.txt` into one file *"one file's name carrying the other's
/// contents, with no error"*, one tool aborted an entire transfer on a
/// non-UTF-8 name where another skipped it, and *"one space in a path breaking
/// three tools three different ways"*.
///
/// **The alphabet is §1.1 class 2's** — lower-case letters, digits, hyphens and
/// the path separator, under an installation-owned prefix — and this clause is
/// what fixes that alphabet for every path this document synthesises. A token
/// so drawn is syntax on no route.
///
/// **S5a: the failure a staging name prevents has been watched happening, by
/// the route that looks safest.** A 99 GB filesystem was filled to leave about
/// 41 MiB and a 209,715,200-byte file pushed into it by two routes. Smoozy's
/// own byte loop into a staging name Smoozy chose raised at 42,823,680 bytes,
/// left 41,930,752 bytes at the staging name, **and the final name was
/// absent**. The standard tool writing straight at the final name exited 1 and
/// **left 261,120 bytes of 209,715,200 at the final name** — a partial file
/// sitting at the name a user would take for a finished one. **This is the
/// strongest single piece of evidence for §7.5 and for S5, and it is observed
/// rather than argued.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StagingPath {
    directory: String,
    name: String,
}

impl StagingPath {
    /// The one way to make one.
    ///
    /// **The directory is the caller's** — S1 requires it to sit on the same
    /// filesystem as the final name, in a Smoozy-named directory beside it,
    /// because *"publication is a rename, and rename is documented on both ends
    /// to require it"*.
    pub fn synthesise(directory: &str, transfer: &Identity, attempt: u32) -> StagingPath {
        StagingPath {
            directory: directory.trim_end_matches('/').to_string(),
            // The identity's written form is lower-case hexadecimal, which is
            // already inside class 2's alphabet; the hyphen and the digits of
            // the attempt number are the rest of it. Nothing of the file's own
            // name is here.
            name: format!("{}-{attempt}.smoozy-part", transfer.as_text()),
        }
    }

    /// The whole path, as it goes on the wire — a protocol field, never a
    /// command string.
    pub fn as_text(&self) -> String {
        format!("{}/{}", self.directory, self.name)
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        self.as_text().into_bytes()
    }

    /// The directory the sweep is bounded to. **Neither sweep ever reaches
    /// outside the directories S4 names.**
    pub fn directory(&self) -> &str {
        &self.directory
    }

    /// **The class-2 test, asked of the composed name.** Nothing outside the
    /// alphabet reaches a path this document synthesises, and a test asserts it
    /// rather than a reader trusting the format string.
    pub fn is_drawn_from_class_twos_alphabet(&self) -> bool {
        self.name.bytes().all(|byte| {
            byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'-' || byte == b'.'
        })
    }
}

/// One checkpoint: **a byte offset into the staged file, with the running
/// digest at that offset.**
///
/// **S31: the file body is never compressed, and this is the requirement.**
/// Sending a compressed body would make a resumed offset an offset into a
/// compressed stream, so *"a resumed offset would mean something different
/// from a fresh one"*, S12's prefix digest would cover bytes that are not the
/// file's, and S13's four-value reading would describe something other than
/// what crosses. **So the staged bytes and the wire bytes are the same
/// bytes**, in both directions, before and after hand-over.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Checkpoint {
    pub offset: u64,
    pub digest_of_everything_up_to_it: Digest,
}

/// **S18's fourth requirement, and its honest absence.**
///
/// The staged and final objects' stable file identity, where the platform or
/// the machine has one. **§12 item 15: no platform's answer has been
/// demonstrated, permanently**, and on the machine it is a command-channel
/// reading on `File Browser` §13 item 9's unestablished ground.
///
/// **Where the supplier cannot answer, the record says the identity is
/// unavailable and publication still proceeds on the byte length and digest** —
/// content evidence, which rests on no name.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StableFileIdentity {
    The(String),
    /// **Recorded as a fact rather than left as a blank.** S15's
    /// identity-unavailable branch is taken from this value, and S28's
    /// automatic removal does not run without an identity.
    TheSupplierCouldNotAnswer,
}

/// **One attempt number the record retains, with the stable file identity
/// recorded for that attempt** — `Server protocol and transfers` S9.
///
/// S9 records the identity *"beside each attempt number the record retains —
/// the live attempt's and each superseded attempt's, once that attempt's
/// staging object was created — or, per attempt, the recorded fact that its
/// supplier could not answer"*. The live attempt's sits in
/// [`TransferRecord::staging_identity`]; a superseded attempt's is this.
///
/// **Why a bare number was not enough.** `Server protocol and transfers` S28
/// removes a derived path only where the path, the synthesised name **and** the
/// stable file identity all match the record, so a superseded path derived with
/// no identity could never be removed however plain it was that the material
/// was Smoozy's — and on a download the user's own computer can answer that
/// identity, so it was thrown away rather than never known. What was left was a
/// part-file inside the user's own destination folder, for good, on every
/// download that starts over.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SupersededAttempt {
    /// **`Server protocol and transfers` S4.** The number the abandoned staging
    /// path was synthesised under.
    pub attempt: u32,
    /// The identity recorded for that attempt, carried across from
    /// [`TransferRecord::staging_identity`] when the attempt was superseded.
    ///
    /// **`None` means the attempt's staging object was never created**, which
    /// is not the same thing as a supplier that could not answer: S9 records the
    /// identity *once that attempt's staging object was created*, and there is
    /// nothing to record before that. Both leave the path unremoved, and S15 is
    /// why — *"S28's automatic removal requires the recorded identity and does
    /// not run without it"*.
    pub staging_identity: Option<StableFileIdentity>,
}

/// **The source's four values, at the path they have always been reached at.**
///
/// The machine reads them as well as the desktop, so the shape lives in the
/// half of this module that opens no connection. Nothing about it changed with
/// the move, and its comparison moved with it.
pub use smoozy_framing::source_reading::SourceReading;

/// **S9's record**, and it is on the desktop side in both directions.
///
/// **S7: what a transfer has staged, and how far, is recorded in the local
/// record on the user's computer — for an upload as well as for a download.**
/// The reason is a hard one and it is observed: **on a full disk nothing
/// further can be written**, every write refused with `No space left on device`
/// and errno 28 from a shell redirect, from `cp`, and from a flushed and
/// fsynced write. *"So a marker written on the machine is exactly what cannot
/// be written in the case it is needed for."* A record on the machine would
/// also be destroyed by a recycle in the case C14 covers, silently and on the
/// same address.
///
/// **S8: the record is durable before the first byte is written.** Row 197's
/// third clause is why — an upload cut off by *"the app closing"* keeps its
/// leftover and the user is told it is there — and *a record written at the end
/// of a transfer is a record a closing process never writes*.
///
/// **G6: a batch reports both levels, and what this record supplies is the
/// whole.** The per-transfer count of G4 gives the detail; S8's durability
/// gives a whole that exists to be read from the first moment. **What the
/// whole is counted in — files or bytes — is not settled here**: Rows 190 and
/// 299 own it, and they are `File Browser`'s and `Download`'s.
///
/// **S6: staged material is action-held and its state is not.** The material
/// ends with the action, success or failure, and its cleanup is §7.8's; the
/// durable record of the transfer is this one, in the local record on the
/// desktop side in both directions, and S7 above says why it cannot be
/// anywhere else.
///
/// **It holds no copy of any byte and no observed value the machine still
/// holds.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TransferRecord {
    /// **T4.** Issued by the initiating side when the transfer is arranged.
    pub transfer: Identity,
    /// **S4.** Starts at one; incremented only where a clause of §7 starts the
    /// transfer over.
    pub attempt: u32,
    /// **The attempts already superseded, retained so the sweep can derive
    /// their paths** (S28), **each carrying the stable file identity recorded
    /// for it** (S9).
    ///
    /// **The identity travels with the number and is not a second list.** S28's
    /// only removing branch wants the path, the synthesised name and the
    /// identity together, so a number that arrived at the sweep without its
    /// identity could not be acted on at all.
    pub superseded_attempts: Vec<SupersededAttempt>,
    pub direction: Direction,
    /// The server row this was arranged against.
    ///
    /// **T3: a transfer is bound to the server it was started against and to
    /// nothing else** — not to the browsing position, not to a screen, and
    /// **not to a connection**. This field is the whole of that binding: a
    /// resumed transfer is matched by its identity and carried on over
    /// whatever connection is live, which is what a resume on a fresh
    /// connection already drives.
    pub server: Identity,
    /// **C14.** The installation identity it was arranged under; a transfer
    /// whose installation is absent or different is disqualified rather than
    /// resumed.
    pub installation: Option<Identity>,
    /// The path at the **sending** end — the user's own file for an upload, the
    /// machine's file for a download.
    pub source_path: String,
    /// The path at the **receiving** end, and the only name publication ever
    /// touches.
    pub final_path: String,
    /// **S1: staged material sits on the same filesystem as the final name, in
    /// a Smoozy-named directory beside it.** The requirement is not a
    /// preference: publication is a rename, and rename is documented on both
    /// ends to require it — `rename(2)`, macOS 26.5, read 2026-08-17: *"Both
    /// old and new must be of the same type … and must reside on the same file
    /// system."*
    ///
    /// **S2: placing staging beside the destination closes the container's
    /// mount trap by construction.** Staging beside the destination is on the
    /// destination's mount whether or not anything reads a mount list.
    ///
    /// **S3: staging never sits under a shared temporary area.** `/tmp` *"is
    /// wiped at every boot"* and `/var/tmp` *"is not cleaned at boot and has a
    /// documented thirty-day expiry underneath"*, and it *"is … an easy place
    /// to leave one rental's data for the next tenant"*.
    pub staging_path: StagingPath,
    pub total_size_expected: u64,
    /// **S24's other side.** The whole file's digest, where the arranging
    /// module knows one — B10's verified release descriptor is the standing
    /// instance. **Where there is none, S21's transport half covers the pipe
    /// and S25 bounds what the pass-through digest is worth**: it proves the
    /// bytes read match the bytes written, and no more.
    pub expected_digest: Option<Digest>,
    /// **The checkpointed high-water offset with a running digest at each
    /// checkpoint.** The highest is what a resume verifies against.
    pub checkpoints: Vec<Checkpoint>,
    /// **C11 step 4.** Which application run last wrote the record — *"A
    /// record written by an earlier application run is not resumed here."*
    pub written_by_run: Identity,
    /// **S15.** The **live** attempt's staging object's stable file identity
    /// once created. Each superseded attempt's sits beside its own number in
    /// [`Self::superseded_attempts`], which is what S9 means by *beside each
    /// attempt number the record retains*.
    pub staging_identity: Option<StableFileIdentity>,
    /// **S13**, for a download.
    pub source_reading: Option<SourceReading>,
    /// **S15's durable pre-state**, written before the final name is touched.
    pub publication_pending: Option<publication::PublicationPreState>,
    /// **Which of S26's five endings it reached**, absent while it is running.
    pub ending: Option<Ending>,
    /// **S27's owed cleanup**, where a removal could not run.
    pub cleanup_is_owed: bool,
}

impl TransferRecord {
    /// A record for a transfer about to start, at attempt one.
    ///
    /// **Nothing here writes it down** — that is the local record's, and S8's
    /// ordering is the caller's to keep.
    #[allow(clippy::too_many_arguments)]
    pub fn arranged(
        transfer: Identity,
        direction: Direction,
        server: Identity,
        installation: Option<Identity>,
        source_path: String,
        final_path: String,
        staging_directory: &str,
        total_size_expected: u64,
        written_by_run: Identity,
    ) -> TransferRecord {
        let staging_path = StagingPath::synthesise(staging_directory, &transfer, 1);
        TransferRecord {
            transfer,
            attempt: 1,
            superseded_attempts: Vec::new(),
            direction,
            server,
            installation,
            source_path,
            final_path,
            staging_path,
            total_size_expected,
            expected_digest: None,
            checkpoints: Vec::new(),
            written_by_run,
            staging_identity: None,
            source_reading: None,
            publication_pending: None,
            ending: None,
            cleanup_is_owed: false,
        }
    }

    /// **S12 step 5 and S13's unconfirmed half**: the attempt number is
    /// incremented and the transfer starts over from zero into the staging path
    /// that number synthesises. **It is never repaired in place.**
    ///
    /// **A start-over is none of S26's five endings** — the transfer is not
    /// failed, the carry-on of Rows 199 and 209 is not broken, and the old
    /// staging path is abandoned under §7.8.
    /// **The identity goes with the number**, rather than being cleared with
    /// the rest of the live attempt's state: S9 keeps it *beside each attempt
    /// number the record retains*, and S28's removal cannot run without it.
    pub fn start_over(&mut self) {
        self.superseded_attempts.push(SupersededAttempt {
            attempt: self.attempt,
            staging_identity: self.staging_identity.take(),
        });
        self.attempt += 1;
        self.staging_path =
            StagingPath::synthesise(self.staging_path.directory(), &self.transfer, self.attempt);
        self.checkpoints.clear();
        self.publication_pending = None;
    }

    /// Every staging path this record has ever named, **each with the stable
    /// file identity recorded for its own attempt** — `Server protocol and
    /// transfers` S9 — so the sweep can reach the superseded ones and act on
    /// them. **S28: the sweep derives the exact staging path from the record**,
    /// never by enumerating a directory.
    ///
    /// The live attempt is last, and its identity is
    /// [`Self::staging_identity`].
    pub fn every_staging_path_with_the_identity_recorded_for_it(
        &self,
    ) -> Vec<(StagingPath, Option<StableFileIdentity>)> {
        let directory = self.staging_path.directory();
        self.superseded_attempts
            .iter()
            .map(|superseded| {
                (
                    StagingPath::synthesise(directory, &self.transfer, superseded.attempt),
                    superseded.staging_identity.clone(),
                )
            })
            .chain(std::iter::once((
                self.staging_path.clone(),
                self.staging_identity.clone(),
            )))
            .collect()
    }

    /// The same paths with the identities dropped, for callers that only want
    /// to know which paths this record has ever named.
    pub fn every_staging_path_it_has_named(&self) -> Vec<StagingPath> {
        self.every_staging_path_with_the_identity_recorded_for_it()
            .into_iter()
            .map(|(path, _)| path)
            .collect()
    }

    /// The highest checkpoint at or below a staged size — **S12 step 2**.
    pub fn highest_checkpoint_at_or_below(&self, size: u64) -> Option<&Checkpoint> {
        self.checkpoints
            .iter()
            .filter(|checkpoint| checkpoint.offset <= size)
            .max_by_key(|checkpoint| checkpoint.offset)
    }

    /// **S9's own high-water offset**: what was accounted for.
    pub fn accounted_for(&self) -> u64 {
        self.checkpoints
            .iter()
            .map(|checkpoint| checkpoint.offset)
            .max()
            .unwrap_or(0)
    }
}

/// **G4's transfer report, and G4a and G4b's two marks beside the count.**
///
/// **The count is the truth and is never held up.** Bytes staged is bytes
/// staged: on a fresh staging path the report is zero and climbs. Nothing
/// carries the abandoned run's figure forward and nothing smooths the fall.
///
/// **What is added is one field beside the count and never a second figure.**
/// From the count alone a transfer that has just begun, one that has begun
/// again, and one whose first chunk is still in flight are the same report, so
/// **nothing infers a start-over from the count falling**.
///
/// **Nothing durable is written from either mark** — both are period-held, like
/// the count.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Report {
    /// **G4.** Bytes staged, counted at the byte loop's own accounting
    /// interval and **never derived from asking the far end for a file's
    /// size** — a staged file's size is not a statement of progress, and a
    /// zero-byte one is *"indistinguishable between 'still being written',
    /// 'the disk filled' and 'the render wrote nothing'"*.
    ///
    /// **G5: progress is never derived from the far end's own view of the
    /// file.** The count is Smoozy's, at Smoozy's own loop, and there is no
    /// route by which the far end's answer can reach this field: the loop adds
    /// what it wrote, and nothing here reads a size back to fill it in.
    pub bytes_staged: u64,
    pub bytes_expected: u64,
    /// **G4a's mark.** Set when the fresh staging path is opened, standing
    /// until that transfer ends under S26.
    pub started_over: Option<StartOverCause>,
    /// **G4b's mark.** Set when the verification begins and cleared when the
    /// first byte above the checkpoint is written, carrying the prefix length
    /// it stands against.
    pub verifying_a_prefix_of: Option<u64>,
}

/// **G4a's two causes, and S24 is not one of them.**
///
/// A digest disagreeing at the end **ends the transfer as *failed***, and
/// *"trying again is a new transfer with a new identity"* — a fresh count from
/// zero there belongs to a different transfer and needs nothing added.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StartOverCause {
    /// **S12 step 5** — the staged prefix did not verify.
    TheStagedPrefixDidNotVerify,
    /// **S13**, and **narrowed to the download direction 2026-08-31**: on the
    /// upload side a source that has *changed* no longer starts over at all —
    /// it stops and is reported.
    TheSourceChangedUnderADownload,
}

impl Report {
    /// **G7's stall test, and G4b keeps it honest.** A verifying channel is
    /// never read as stalled: the verification moves no bytes and does not move
    /// the recorded offset, and on a nearly complete 4 GiB prefix it runs
    /// **about twelve minutes** at the measured 5.9 MB/s read-back rate.
    pub fn may_be_read_as_stalled(&self) -> bool {
        self.verifying_a_prefix_of.is_none()
    }

    /// **G4b: no completion estimate treats a resume as starting at the
    /// checkpoint.** A report carrying the verifying mark composes none, which
    /// is what `Desktop app and delivery` R15b's enumeration was missing.
    pub fn a_completion_estimate_may_be_composed_over_it(&self) -> bool {
        self.verifying_a_prefix_of.is_none()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use smoozy_state::identities::IdentityKind;

    fn an_identity(kind: IdentityKind) -> Identity {
        Identity::issue(kind).expect("this machine supplies random bytes")
    }

    fn a_record() -> TransferRecord {
        TransferRecord::arranged(
            an_identity(IdentityKind::Transfer),
            Direction::Up,
            an_identity(IdentityKind::LocalServer),
            Some(an_identity(IdentityKind::Installation)),
            "/home/user/scene.blend".to_string(),
            "/home/user/jobs/scene.blend".to_string(),
            "/home/user/jobs/.smoozy-staging",
            4_294_967_296,
            an_identity(IdentityKind::Run),
        )
    }

    #[test]
    fn interrupted_is_the_only_ending_that_may_be_carried_on() {
        // S26b. This is the finding that reversed S26a's first bullet: an
        // integrity failure recorded as *interrupted* was picked up at the next
        // repair and re-sent the whole file with nothing said.
        assert!(Ending::Interrupted.may_the_work_be_carried_on());
        for over in [
            Ending::Published,
            Ending::Stopped(StoppedReason::StagingRemovedByUser),
            Ending::Failed,
            Ending::Stalled,
        ] {
            assert!(!over.may_the_work_be_carried_on());
        }
    }

    #[test]
    fn a_transfer_is_bound_to_its_server_and_not_to_a_connection() {
        // T3. Everything a resume needs is in the record: the staging path is
        // derived from the transfer identity and the attempt number, and the
        // server is a field of its own. Nothing here names a connection, which
        // is why a transfer carries on over a fresh one.
        let record = a_record();
        let derived = StagingPath::synthesise(
            record.staging_path.directory(),
            &record.transfer,
            record.attempt,
        );
        assert_eq!(derived.as_text(), record.staging_path.as_text());
        assert!(Ending::Interrupted.may_the_work_be_carried_on());
    }

    #[test]
    fn a_stalled_channel_and_an_interrupted_one_are_two_endings() {
        // G8: a stalled channel is not a broken connection and neither is
        // inferred from the other. The deadline's ending is its own value, and
        // it does what *interrupted* does not — the material goes, and the
        // work is over rather than waiting.
        assert_ne!(Ending::Stalled, Ending::Interrupted);
        assert!(!Ending::Stalled.may_the_work_be_carried_on());
        assert!(Ending::Interrupted.may_the_work_be_carried_on());
    }

    #[test]
    fn interrupted_is_a_mark_on_a_transfer_record_and_on_nothing_else() {
        // S26c. The word names a transfer's ending here. A render attempt's
        // mark of the same name is `Queue & History` N1's and is not this
        // value, not derived from it, and not written from it — there is no
        // route between the two, and this record is the only place the ending
        // is kept.
        let mut record = a_record();
        assert_eq!(record.ending, None);
        record.ending = Some(Ending::Interrupted);
        assert_eq!(record.ending, Some(Ending::Interrupted));
    }

    #[test]
    fn the_two_columns_of_s26s_table_are_two_questions() {
        // *Published* takes the material by publishing it and may not be
        // carried on; *interrupted* keeps it and may. A builder reading one
        // column for the other gets both wrong.
        assert_eq!(
            Ending::Published.what_becomes_of_the_staged_material(),
            StagedMaterial::TakenByTheRenameItself
        );
        assert_eq!(
            Ending::Interrupted.what_becomes_of_the_staged_material(),
            StagedMaterial::Kept
        );
        assert_eq!(
            Ending::Failed.what_becomes_of_the_staged_material(),
            StagedMaterial::Removed
        );
        assert_eq!(
            Ending::Stalled.what_becomes_of_the_staged_material(),
            StagedMaterial::Removed
        );
    }

    #[test]
    fn the_record_is_written_before_the_channel_closes_on_every_removing_ending() {
        // C23: a stop and a drop are never told apart from the wire, and §7.8
        // does opposite things in the two cases.
        assert!(Ending::Failed.the_record_is_written_before_the_channel_closes());
        assert!(Ending::Stalled.the_record_is_written_before_the_channel_closes());
        assert!(Ending::Stopped(StoppedReason::AbandonedByItsRequest)
            .the_record_is_written_before_the_channel_closes());
        assert!(!Ending::Interrupted.the_record_is_written_before_the_channel_closes());
    }

    #[test]
    fn the_staging_path_carries_nothing_of_the_files_own_name() {
        // S4. The final path here holds a space, a capital and a leading dash,
        // and none of them reaches the staging name.
        let mut record = a_record();
        record.final_path = "/home/user/-Capitals And Spaces.blend".to_string();
        let staged = record.staging_path.as_text();
        assert!(!staged.contains("Capitals"));
        assert!(!staged.contains(' '));
        assert!(record.staging_path.is_drawn_from_class_twos_alphabet());
    }

    #[test]
    fn one_path_per_attempt_and_the_superseded_ones_stay_reachable() {
        // S4's own reason for the attempt number: the identity alone named two
        // objects, and a consumer taking the material up by taking the identity
        // could not tell which.
        let mut record = a_record();
        let first = record.staging_path.clone();
        record.start_over();
        let second = record.staging_path.clone();
        assert_ne!(first, second);
        assert_eq!(record.attempt, 2);
        let all = record.every_staging_path_it_has_named();
        assert!(all.contains(&first));
        assert!(all.contains(&second));
    }

    #[test]
    fn a_superseded_attempt_keeps_the_identity_that_was_recorded_for_it() {
        // `Server protocol and transfers` S9's *beside each attempt number the
        // record retains*. Before this the identity was cleared at the
        // start-over, so the superseded path reached S28 with nothing to match
        // and its only removing branch could not run.
        let mut record = a_record();
        record.staging_identity = Some(StableFileIdentity::The("volume-1:file-7".to_string()));
        let first = record.staging_path.clone();
        record.start_over();

        assert_eq!(record.superseded_attempts.len(), 1);
        assert_eq!(record.superseded_attempts[0].attempt, 1);
        assert_eq!(
            record.superseded_attempts[0].staging_identity,
            Some(StableFileIdentity::The("volume-1:file-7".to_string()))
        );
        // The live attempt starts with none of its own: its staging object does
        // not exist yet, which S9 distinguishes from a supplier that could not
        // answer.
        assert_eq!(record.staging_identity, None);

        let paths = record.every_staging_path_with_the_identity_recorded_for_it();
        assert_eq!(
            paths
                .iter()
                .find(|(path, _)| *path == first)
                .and_then(|(_, identity)| identity.clone()),
            Some(StableFileIdentity::The("volume-1:file-7".to_string()))
        );
    }

    #[test]
    fn an_attempt_whose_supplier_could_not_answer_is_kept_apart_from_one_with_no_object() {
        // S9's two per-attempt cases. A supplier that could not answer is a
        // recorded fact; an attempt whose staging object was never created has
        // nothing recorded at all. S28 removes on neither, and a builder
        // reading one for the other would report the wrong break.
        let mut record = a_record();
        record.staging_identity = Some(StableFileIdentity::TheSupplierCouldNotAnswer);
        record.start_over();
        record.start_over();

        assert_eq!(
            record.superseded_attempts[0].staging_identity,
            Some(StableFileIdentity::TheSupplierCouldNotAnswer)
        );
        assert_eq!(record.superseded_attempts[1].staging_identity, None);
    }

    #[test]
    fn a_start_over_clears_the_checkpoints_rather_than_repairing_in_place() {
        // S12 step 5: *"It is never repaired in place."*
        let mut record = a_record();
        record.checkpoints.push(Checkpoint {
            offset: 1_048_576,
            digest_of_everything_up_to_it: Digest::of(b"whatever"),
        });
        record.start_over();
        assert!(record.checkpoints.is_empty());
        assert_eq!(record.accounted_for(), 0);
    }

    #[test]
    fn the_highest_checkpoint_at_or_below_is_the_one_taken() {
        // S12 step 2, and the *at or below* is what makes a record ahead of the
        // disk harmless.
        let mut record = a_record();
        for offset in [1_000u64, 2_000, 3_000] {
            record.checkpoints.push(Checkpoint {
                offset,
                digest_of_everything_up_to_it: Digest::of(&offset.to_be_bytes()),
            });
        }
        assert_eq!(
            record
                .highest_checkpoint_at_or_below(2_500)
                .map(|checkpoint| checkpoint.offset),
            Some(2_000)
        );
        assert_eq!(record.highest_checkpoint_at_or_below(999), None);
    }

    #[test]
    fn a_verifying_report_is_neither_stalled_nor_given_an_estimate() {
        // G4b, and §12 item 18's break: on the words as they stood a resuming
        // transfer was given an estimate composed as though it had started at
        // its checkpoint, for the whole of a twelve-minute verification.
        let verifying = Report {
            bytes_staged: 4_000_000_000,
            bytes_expected: 4_294_967_296,
            started_over: None,
            verifying_a_prefix_of: Some(4_000_000_000),
        };
        assert!(!verifying.may_be_read_as_stalled());
        assert!(!verifying.a_completion_estimate_may_be_composed_over_it());
        let moving = Report {
            verifying_a_prefix_of: None,
            ..verifying
        };
        assert!(moving.may_be_read_as_stalled());
        assert!(moving.a_completion_estimate_may_be_composed_over_it());
    }
}
