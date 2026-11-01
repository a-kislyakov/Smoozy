//! The byte loop — the acting half of §7, in both directions.
//!
//! **R5: the bytes move through Smoozy, and that is observed to be possible.**
//! Driven directly against OpenSSH's own file-transfer server subsystem: writes
//! at caller-chosen byte offsets in caller-chosen chunks landed at those
//! offsets, a transfer deliberately stopped at 1,048,576 bytes left a staging
//! file of exactly that size **with the final name absent**, a second open of
//! the same path without truncation carried on from byte 1,048,576, and the
//! finished 3,145,728-byte file's digest matched the source.
//!
//! **R4: the enforcement point cannot be an external tool's flag**, settled by
//! measurement and by documentation. `sftp(1)`'s `-l limit` is *"an invocation
//! flag with no route to change it while the invocation runs"*, so the
//! external-tool route fails Row 306 by construction and not merely in
//! measurement. **S5: no tool ever chooses the staging name** — a hard kill on
//! one box left rsync's own *"randomly named temporary file"*, *"a name Smoozy
//! cannot predict"*. Because the byte loop is inside Smoozy, the name is
//! Smoozy's in every case and the hazard does not arise.
//!
//! **C23a is the shape of this file, and it was learned by watching it fail.**
//! A killed machine-side session raised the drop **out of the write call**,
//! propagated **past** the code that classifies the ending, and the process
//! exited by traceback — *"The durable record was left saying the transfer was
//! still moving rather than interrupted."* **So the classification is not a
//! step at the end of the loop**: [`carry_out`] owns every exit path of
//! [`the_loop`], and the record is written by the wrapper rather than by a line
//! an early return can jump over.
//!
//! **What is deliberately not here: §8's shared rate budget and §8.4's
//! achieved-rate reading.** R1 puts both budgets in `Desktop app and delivery`'s
//! single desktop-side scheduler and says outright that *"This document does
//! not hold a second budget, does not divide one, and does not cache one."*
//! What is here is the **per-channel enforcement point** the budget is drawn
//! at, behind [`DrawsFromTheSharedBudget`], with [`NoCapIsInForce`] standing in
//! until that scheduler exists.
//!
//! **Markers used below that belong to another document, each with the document
//! it belongs to.** `File Browser` K9, K10.
//!
//! `File Browser` K9 is the clause this plan's own S15b gate exists for. **A
//! marker with no document beside it is a clause of this crate's own plan**,
//! `Preparation/Architecture/Server protocol and transfers.md`.
//!
//! **`Desktop app and delivery` R8 is deliberately not listed above and is
//! written out with its document at each of its sites**, because **this plan
//! defines an R8 of its own** — a cap set while transfers are in flight takes
//! effect at each channel's next accounting interval — and a bare one would
//! resolve to a real and wrong clause either way round.

use std::collections::VecDeque;
use std::future::Future;
use std::time::{Duration, Instant};

use tokio::io::{AsyncRead, AsyncWrite};

use super::integrity::{compare, Digest, Running, WhatTheComparisonSaid};
use super::publication::{
    check_the_rename, judge_the_recovery, which_operation, Intent, Operation, PublicationPreState,
    RecoveryVerdict, RenameCheck, WhatTheFinalObjectReadsAs,
};
use super::resume::{
    after_verifying, compare_the_source, what_the_source_gate_does, what_to_verify, ResumePlan,
    WhatTheSourceGateDoes, WhatTheSourceSaid, WhatToVerify,
};
use super::{
    Checkpoint, Direction, Ending, Report, SourceReading, StableFileIdentity, StoppedReason,
    TransferRecord,
};
use crate::subsystem::client::Subsystem;
use crate::subsystem::{extension, SubsystemFailure};

/// **R6's time half, and it is the one that matters for R8 and G4.**
///
/// *"a chunk bounds bytes, and the delay between two chunks is bytes divided by
/// the rate the channel is actually drawing … At 261,120 bytes, a 50 KB/s cap
/// and four channels, one chunk is about 21 seconds, so a byte bound is no
/// bound on time at all — a raised cap would appear to do nothing for that long,
/// and a transfer would report an unmoving number for that long. So the loop
/// consults the budget and emits its count **at least once per a fixed short
/// period** whatever the chunk size."*
///
/// **The number is Build's to pick and it is a tenth of a second**, which is
/// R6's *many times per second per channel* met with room to spare and is well
/// under R12a's three-second window, so a window holds many tens of counts.
/// **It is one constant in one place.**
pub const THE_ACCOUNTING_PERIOD: Duration = Duration::from_millis(100);

/// How much crosses between two record writes.
///
/// **S12 step 4 bounds the waste at one checkpoint interval** — *"Bytes between
/// the checkpoint and the staged size are re-sent"* — so this number is the
/// most a resume can throw away. **The number is Build's to pick and it is four
/// mebibytes.** The endurance run wrote its durable record every two seconds,
/// which at that run's own 384 KB/s is under a mebibyte; four mebibytes is the
/// same order at the rates a real line reaches (up 0.62 MB/s, down 3.88 MB/s
/// measured) and keeps the record's write rate at a few a second at worst.
pub const THE_CHECKPOINT_INTERVAL: u64 = 4 * 1024 * 1024;

/// How many writes are handed over before their answers are collected.
///
/// **R6a requires the write path to be pipelined and says nothing about how
/// deep**, and the depth cannot be unbounded: every write's answer comes back
/// on the same channel, and a loop that never collects them fills the channel's
/// own queue until neither end can move. **So the depth is bounded here, and
/// the bound is a Build decision.**
///
/// **It is sixteen.** T7 records that the standard client keeps *"64 requests …
/// active concurrently"* by default, so a pipeline of this order is the
/// ordinary shape rather than an invention; sixteen at the negotiated
/// 261,120-byte write is about four mebibytes in flight, which is one
/// checkpoint interval and keeps the two bounds in step. **The measured
/// difference R6a is about is between one and many**, not between sixteen and
/// sixty-four: the same loop ran at **99–102 KB/s** synchronous and **384.0
/// KB/s** pipelined.
pub const THE_MOST_WRITES_IN_FLIGHT: usize = 16;

/// How many reads are handed over before the oldest one's answer is collected.
///
/// **R6a holds the read path of a download to the write path's rule** — added
/// 2026-09-30, on the build runner's ruling after step 10c's eleventh pass:
/// *"So a download keeps several reads in flight, and so does the read-back
/// that verifies a staged prefix (S12)."* At a 0.2 s round trip one 50 MB file
/// came down in **48.5 s against 7.9 s up**, one 261,120-byte read per round
/// trip, and a staged prefix was verified at **1.7 MB/s on a line carrying 11**.
///
/// **It is sixteen, the write path's own depth and for the same reasons**:
/// sixteen reads of the negotiated 261,120 bytes are about four mebibytes in
/// flight, one checkpoint interval, and the standard client keeps *"64
/// requests … active concurrently"* (T7). **The depth is bounded** because
/// every answer comes back on the same channel, and a loop that never collects
/// them fills the channel's own queue. **A frame of 665 KB is three reads, so
/// all three are in flight at once and cost one round trip rather than three.**
pub const THE_MOST_READS_IN_FLIGHT: usize = 16;

/// **R2: each transfer channel's byte loop draws from the shared budget on each
/// accounting interval and moves no more than it drew.**
///
/// **R1: there is one upload budget and one download budget for the whole
/// application, and neither is in this document.** They are `Desktop app and
/// delivery`'s. **R3: a per-channel or per-transfer limit is forbidden** — Row
/// 289's own words rule it out, and Fact 75 shows what it costs even when the
/// numbers add up: a two-way 400+400 split against an 800 KB/s budget achieved
/// **421.9 KB/s**, *"half the budget forfeited the moment the small transfer
/// finished"*.
pub trait DrawsFromTheSharedBudget: Send {
    /// How many bytes this channel may move now. **R7: a channel that drew
    /// nothing waits and does not spin.**
    fn draw(&mut self, wanted: usize) -> usize;
    /// **R7: a channel that finished returns its unused share at once.**
    /// Returning it is the half Fact 75 measured as forfeited.
    fn give_back(&mut self, unused: usize);
}

/// What stands in until `Desktop app and delivery`'s scheduler exists.
///
/// **An empty field is no cap, which is also the default** (`Desktop app and
/// delivery` R8), so this is the ordinary state rather than a stub of a
/// behaviour: with no cap set, a channel draws whatever it asked for.
pub struct NoCapIsInForce;

impl DrawsFromTheSharedBudget for NoCapIsInForce {
    fn draw(&mut self, wanted: usize) -> usize {
        wanted
    }
    fn give_back(&mut self, _unused: usize) {}
}

/// The user's own computer's end — **S18's contract, stated here and
/// implemented in `Desktop app and delivery`'s platform file adapter.**
///
/// *"What the operation is on each of Mac, Windows and Linux, and its
/// demonstration there, is that document's under Fact 79."*
pub trait TheUsersOwnComputer: Send {
    /// The staged file's size, or nothing where no staging path exists — S12
    /// step 1.
    fn staged_size(&mut self, path: &str) -> Option<u64>;
    /// Reads a range of a file on the user's computer.
    fn read_at(&mut self, path: &str, offset: u64, length: usize) -> Result<Vec<u8>, LocalFailure>;
    /// **The exclusive create.** Two attempts can never land on one object.
    fn create_staging_exclusively(&mut self, path: &str) -> Result<(), LocalFailure>;
    /// Writes at a byte offset, without truncating.
    fn write_at(&mut self, path: &str, offset: u64, bytes: &[u8]) -> Result<(), LocalFailure>;
    /// **S20's ordering on this end.** The staged bytes are made durable before
    /// publication.
    fn make_durable(&mut self, path: &str) -> Result<(), LocalFailure>;
    /// **S18's fourth requirement.** *"An adapter that cannot answer is met by
    /// S15's identity-unavailable branch rather than by a guess."*
    fn stable_identity_of(&mut self, path: &str) -> StableFileIdentity;
    /// **S15 and S18's first two requirements**: the staged file appears at the
    /// final name in **one** step, and no part-arrived bytes ever appear at the
    /// final name.
    fn publish(&mut self, staging: &str, final_path: &str, intent: Intent)
        -> Result<(), LocalFailure>;
    fn remove(&mut self, path: &str) -> Result<(), LocalFailure>;
    /// What the final object reads as, for S15's recovery and rename check.
    fn read_the_final_object(&mut self, path: &str) -> WhatTheFinalObjectReadsAs;
    /// **S15c, an upload's half: the source's modification time**, in whole
    /// seconds since 1970, as read when the transfer was arranged (`File
    /// Browser` K7's path identity), or nothing where it cannot be read.
    /// Nothing is guessed.
    fn the_sources_time(&mut self, source_path: &str) -> Option<i64>;
    /// **S15c, a download's half: the staged file takes the server file's
    /// modification time**, and the access time with it, to the same value. A
    /// refusal does not fail the transfer.
    fn keep_the_time(&mut self, staging: &str, seconds: i64) -> Result<(), LocalFailure>;
}

/// **The outcome is the first error any call returned**, which is the platform
/// adapter's own rule (`Desktop app and delivery` §6) and is not re-decided
/// here.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalFailure(pub String);

/// Who writes the transfer record down.
///
/// **S8: the record is durable before the first byte is written**, and this is
/// how the loop reaches the local record without this crate depending on it —
/// the same seam `Servers` uses for its own store.
pub trait KeepsTheRecord: Send {
    fn write_it_down(&mut self, record: &TransferRecord) -> Result<(), LocalFailure>;
}

/// Who is told how far the transfer has got — G4, G4a and G4b.
pub trait WatchesTheTransfer: Send {
    fn report(&mut self, report: &Report);
}

/// A watcher that wants nothing.
pub struct NobodyIsWatching;
impl WatchesTheTransfer for NobodyIsWatching {
    fn report(&mut self, _report: &Report) {}
}

/// **S15b: the publication gate a caller names.**
///
/// *"The caller supplies a gate with the publication. The loop asks it **once**,
/// at the moment every byte of the transfer has passed and §7.6's end-to-end
/// comparison has been made, and **before** S15's durable pre-state is written.
/// It is handed the transfer's own pass-through digest (S22) — the digest of
/// what was actually sent — and returns one of two answers."*
///
/// **What this document decides is the moment and nothing else.** What the
/// digest is compared against is the declaring document's: `File Browser` K9's
/// is a second full reading of the source, and its arithmetic is not repeated
/// here. **Nothing in this crate reads the caller's file and no gate of this
/// document's own is composed.**
///
/// **Why the caller is handed the digest rather than asked for one.** S22 puts
/// the pass-through digest in this loop, computed as the bytes pass; a caller
/// that had to keep its own copy of it would be keeping a second one of the
/// value this loop already holds.
pub trait TheCallersPublicationGate: Send {
    /// **Asked once**, with the digest of what was actually sent.
    fn may_this_be_published(&mut self, what_was_sent: &Digest) -> WhatTheGateSaid;
}

/// The gate's two answers, and there is no third.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhatTheGateSaid {
    ItMayBePublished,
    /// **S15b's refusal**, which takes S26's second row —
    /// [`Ending::Stopped`] with [`StoppedReason::TheSourceChangedUnderIt`]. The
    /// staged material is removed on that row's rule and nothing is published.
    ///
    /// **No sixth ending and no second stopped reason is added for it**: it is
    /// the ending S13's upload half already takes.
    TheSourceChangedUnderIt,
}

/// What a transfer's run came to.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WhatHappened {
    pub ending: Ending,
    pub report: Report,
}

/// Why the loop stopped, before it is turned into one of S26's five endings.
#[derive(Debug, Clone, PartialEq, Eq)]
enum WhyTheLoopStopped {
    ItFinishedAndPublished,
    /// **S26's *failed* row, first reason.** The digests disagreed at the end
    /// (S24).
    TheDigestsDisagreed,
    /// **S26's *failed* row, second reason. Added 2026-09-03.** S15's third
    /// bullet: a readable value did not match, so it is a failed publication
    /// and not a published file.
    ///
    /// **It used to be *interrupted*, which is the one word that means the work
    /// may be carried on** — so a publication that landed and was not recorded
    /// was retried for ever against a staging file the rename had already taken
    /// away, and every attempt reported *interrupted* again on a live and
    /// healthy line.
    ThePublicationCheckCameBackWrong,
    /// **S26's *failed* row, third reason. Added 2026-09-03.** A refusal from
    /// the platform file adapter on the user's own computer: the staging file
    /// that cannot be made, written to, read back or published.
    ///
    /// **It used to be *interrupted*, and the cost was the user's.** A full
    /// disk composed to **waiting for the connection** on a perfect line, the
    /// user checked their internet and restarted their router, Smoozy retried
    /// at every reconnection and got the same answer every time, and nothing
    /// anywhere said *your disk is full*.
    ALocalFailureAReconnectionCannotClear,
    /// The channel's inactivity deadline expired (C19).
    TheDeadlineExpired,
    /// The source changed under an upload (S13). **The upload direction alone**
    /// — on a download a source that ends short starts the transfer over
    /// instead, inside the loop.
    TheSourceChanged,
    /// **S15's recovery took the user's own branch.** A transfer-aware user
    /// deletion had already recorded *staging removed by user* before removing
    /// the path, so recovery takes that explicit branch without inspecting the
    /// final name.
    TheUserRemovedTheStaging,
    /// **A drop, a damaged pipe, the process ending.** Every transport failure
    /// lands here, which is S21's *"a dropped channel is never read as a clean
    /// ending"*.
    ItWasInterrupted,
}

/// Everything one transfer needs, gathered so the loop takes one argument
/// rather than eight.
pub struct Transfer<'a, S, L, B, K, W> {
    pub subsystem: &'a mut Subsystem<S>,
    pub local: &'a mut L,
    pub budget: &'a mut B,
    pub keeper: &'a mut K,
    pub watcher: &'a mut W,
    /// **C19's deadline**, set well above the longest legitimate stall observed
    /// — *"a frozen transfer sat at 17,233,920 bytes for 11.1 seconds while
    /// unfinished"*. It runs only while the channel holds an allowance it
    /// cannot spend.
    pub inactivity_deadline: Duration,
    /// **S15a.** Declared by the caller and never inferred here. *"Where the
    /// caller declares neither, publication does not begin."*
    pub intent: Intent,
}

/// **The whole of one transfer, and the wrapper that owns every exit path.**
///
/// C23a's rule: the record's ending is written by something that survives a
/// failure raised anywhere inside, **not by a line the failure can jump over**.
///
/// **S15b: this is the entry point for a caller that declares no publication
/// gate, and that is the ordinary case** — no clause of this document requires
/// one, and a download has no source on the user's own computer to read
/// through. A caller that declares one calls
/// [`carry_out_under_a_publication_gate`] instead, which is how the gate is
/// named: there is no field holding a value that stands in for a gate, and no
/// default anywhere.
pub async fn carry_out<S, L, B, K, W>(
    transfer: Transfer<'_, S, L, B, K, W>,
    record: &mut TransferRecord,
) -> WhatHappened
where
    S: AsyncRead + AsyncWrite + Unpin + Send,
    L: TheUsersOwnComputer,
    B: DrawsFromTheSharedBudget,
    K: KeepsTheRecord,
    W: WatchesTheTransfer,
{
    carry_out_with(transfer, record, None, None, TheStagingFolder::AskedForAsAlways).await
}

/// **The same transfer, handed S13's four-value reading of the file on the
/// server as it stands now** — the download direction's *"all four are
/// compared before a resume"*, taken by the caller just before it hands the
/// record back to this loop.
///
/// **Why the caller takes it and this loop does not.** The reading is a
/// command-channel reading (revision 3's attribute structure carries neither
/// the inode nor the change time), and this loop holds a file-transfer channel
/// and nothing else. So the caller reads it on the server's own point and hands
/// the value in, the same way [`carry_out_under_a_publication_gate`] is handed
/// its gate: **there is no field holding a value that stands in for one, and
/// `None` is S13's own *could not be confirmed***, which starts the file over.
/// Added 2026-10-04, the 38's piece 9 (item 18).
///
/// **It is compared only where a resume is actually in question** — a download
/// with staged material and no publication pending — and against the four
/// values the record carries from the moment the download was arranged
/// ([`TransferRecord::source_reading`]). Any difference starts the file over
/// with [`super::StartOverCause::TheSourceChangedUnderADownload`]; agreement
/// lets S12's prefix verification run.
pub async fn carry_out_with_the_source_as_it_stands<S, L, B, K, W>(
    transfer: Transfer<'_, S, L, B, K, W>,
    record: &mut TransferRecord,
    the_source_now: Option<SourceReading>,
) -> WhatHappened
where
    S: AsyncRead + AsyncWrite + Unpin + Send,
    L: TheUsersOwnComputer,
    B: DrawsFromTheSharedBudget,
    K: KeepsTheRecord,
    W: WatchesTheTransfer,
{
    carry_out_with(
        transfer,
        record,
        None,
        the_source_now,
        TheStagingFolder::AskedForAsAlways,
    )
    .await
}

/// **The same transfer, under S15b's caller-declared publication gate.**
///
/// The gate is asked **once**, after the pass-through digest is complete and
/// before S15's durable pre-state is written, and a refusal ends the transfer
/// `Stopped(TheSourceChangedUnderIt)`.
///
/// **This is the moment `File Browser` K9's no-corruption promise is kept at**,
/// and until this entry point existed that module's refusal travelled on an
/// empty upload read — a reading that means *the source ended short* being used
/// to mean *the source changed*. Both produce the right record and the right
/// ending; only one of them is a moment a reader of this loop can find.
pub async fn carry_out_under_a_publication_gate<S, L, B, K, W>(
    transfer: Transfer<'_, S, L, B, K, W>,
    record: &mut TransferRecord,
    gate: &mut dyn TheCallersPublicationGate,
) -> WhatHappened
where
    S: AsyncRead + AsyncWrite + Unpin + Send,
    L: TheUsersOwnComputer,
    B: DrawsFromTheSharedBudget,
    K: KeepsTheRecord,
    W: WatchesTheTransfer,
{
    carry_out_with(
        transfer,
        record,
        Some(gate),
        None,
        TheStagingFolder::AskedForAsAlways,
    )
    .await
}

/// **The same transfer, for a fresh file** — S12a, written 2026-10-16 on the
/// build runner's ruling on step 10c's owed piece 1.
///
/// **A fresh file is an upload whose staging folder the caller's own request
/// made — its make answered that it made it — and whose record holds no staged
/// part**: its first attempt, no attempt superseded, no checkpoint, no staging
/// object ever opened, no publication pending. **For such a file the three asks
/// this loop makes before every staging object are not made**: the staging
/// folder (S1), the staged part's size (S12 step 1) and its removal before the
/// exclusive create. Nothing can stand at a name synthesised from an identity
/// issued a moment ago (S4) inside a folder its own request made, and the
/// folder was just made.
///
/// **The case this loop was first written for** is any file, anywhere, with
/// whatever an earlier attempt left. **The case it now also meets** is a
/// thousand small files into folders the same request made, where the three
/// asks were a quarter of each file's round trips and none could find
/// anything.
///
/// **What keeps a resume whole.** The record is read here as well as declared
/// by the caller: **a record holding any trace of a staged part takes every ask
/// as always**, whatever the caller says. **And where the exclusive create is
/// refused** — the folder is gone, or something stands at the name — **the
/// three asks are made after all and the create is made again**, so a reading
/// that did not hold costs round trips and nothing else.
pub async fn carry_out_a_fresh_file_under_a_publication_gate<S, L, B, K, W>(
    transfer: Transfer<'_, S, L, B, K, W>,
    record: &mut TransferRecord,
    gate: &mut dyn TheCallersPublicationGate,
) -> WhatHappened
where
    S: AsyncRead + AsyncWrite + Unpin + Send,
    L: TheUsersOwnComputer,
    B: DrawsFromTheSharedBudget,
    K: KeepsTheRecord,
    W: WatchesTheTransfer,
{
    carry_out_with(
        transfer,
        record,
        Some(gate),
        None,
        TheStagingFolder::MadeByTheCallersRequest,
    )
    .await
}

/// **What the caller knows of the folder a file's staged part goes into** —
/// S12a. See
/// [`carry_out_a_fresh_file_under_a_publication_gate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TheStagingFolder {
    /// Asked for, and the staged part read and removed, before the file's
    /// staging object — as §7 has always done.
    AskedForAsAlways,
    /// **Made by the caller's own request**, its make having answered that it
    /// made it.
    MadeByTheCallersRequest,
}

/// **The record holds no staged part** — S12a: the
/// first attempt, none superseded, no checkpoint, no staging object ever
/// opened (an upload's identity is written down as unavailable at its first
/// opening, so `None` means none was opened), and no publication pending.
fn it_holds_no_staged_part(record: &TransferRecord) -> bool {
    record.attempt == 1
        && record.superseded_attempts.is_empty()
        && record.checkpoints.is_empty()
        && record.staging_identity.is_none()
        && record.publication_pending.is_none()
}

async fn carry_out_with<S, L, B, K, W>(
    mut transfer: Transfer<'_, S, L, B, K, W>,
    record: &mut TransferRecord,
    gate: Option<&mut dyn TheCallersPublicationGate>,
    the_source_now: Option<SourceReading>,
    the_staging_folder: TheStagingFolder,
) -> WhatHappened
where
    S: AsyncRead + AsyncWrite + Unpin + Send,
    L: TheUsersOwnComputer,
    B: DrawsFromTheSharedBudget,
    K: KeepsTheRecord,
    W: WatchesTheTransfer,
{
    let mut report = Report {
        bytes_staged: 0,
        bytes_expected: record.total_size_expected,
        started_over: None,
        verifying_a_prefix_of: None,
    };

    let why = the_loop(
        &mut transfer,
        record,
        &mut report,
        gate,
        the_source_now.as_ref(),
        the_staging_folder,
    )
    .await;

    let ending = match why {
        WhyTheLoopStopped::ItFinishedAndPublished => Ending::Published,
        // **S26's *failed* row names its reason and there are three**, widened
        // 2026-09-03: a digest mismatch, a publication check that came back
        // wrong, and a local failure a reconnection cannot clear. **None of the
        // three clears by being tried again on a fresh connection**, which is
        // why none of them may take *interrupted* — the one word that means the
        // work is waiting to be picked up (S26b).
        WhyTheLoopStopped::TheDigestsDisagreed
        | WhyTheLoopStopped::ThePublicationCheckCameBackWrong
        | WhyTheLoopStopped::ALocalFailureAReconnectionCannotClear => Ending::Failed,
        WhyTheLoopStopped::TheDeadlineExpired => Ending::Stalled,
        WhyTheLoopStopped::TheSourceChanged => {
            Ending::Stopped(StoppedReason::TheSourceChangedUnderIt)
        }
        WhyTheLoopStopped::TheUserRemovedTheStaging => {
            Ending::Stopped(StoppedReason::StagingRemovedByUser)
        }
        WhyTheLoopStopped::ItWasInterrupted => Ending::Interrupted,
    };

    // **C23: the durable record of which one Smoozy did is written before the
    // channel is closed**, on every ending whose material is removed. It is
    // written here for all five, because the wrapper is the one place that
    // cannot be jumped over.
    // **S15's paragraph of 2026-09-29: an ending that says the work is over
    // owes no publication**, so its pending mark goes in the same write as the
    // ending — unless Smoozy's own reservation may still stand at the final
    // name, where it stays for the next start to settle. See
    // [`the_pending_mark_may_go`].
    if record.publication_pending.is_some()
        && the_work_is_over(&ending)
        && the_pending_mark_may_go(&mut transfer, record)
    {
        record.publication_pending = None;
    }
    record.ending = Some(ending.clone());
    let _ = transfer.keeper.write_it_down(record);

    // **S26's table performed.** The three endings that remove the material
    // remove it; *interrupted* keeps it; *published* has none left.
    if matches!(
        ending.what_becomes_of_the_staged_material(),
        super::StagedMaterial::Removed
    ) {
        let removed = match record.direction {
            // **A channel that has just gone a whole deadline unanswered is
            // not asked for the removal**, which would be one more wait on it
            // — and until 2026-10-01 an unbounded one, so a *stalled* upload
            // never returned from here. **The removal is owed instead** (S27),
            // which is what a removal that could not be carried out is.
            // Step 10c's twelfth-pass fix piece 1.
            Direction::Up if ending == Ending::Stalled => false,
            // **C19**: the removal is a wait on the machine like any other.
            Direction::Up => matches!(
                tokio::time::timeout(
                    transfer.inactivity_deadline,
                    transfer.subsystem.remove(&record.staging_path.as_bytes()),
                )
                .await,
                Ok(Ok(()))
            ),
            Direction::Down => transfer
                .local
                .remove(&record.staging_path.as_text())
                .is_ok(),
        };
        // **S27: a stop that could not be carried out is an owed cleanup and
        // not a forgotten one.**
        record.cleanup_is_owed = !removed;
        let _ = transfer.keeper.write_it_down(record);
    }

    transfer.watcher.report(&report);
    WhatHappened { ending, report }
}

/// **The three endings no retry follows** — S26's *failed*, *stalled* and
/// *stopped*. *Published* has nothing pending, and *interrupted* is the one
/// ending whose pre-state S15's first bullet still reads.
fn the_work_is_over(ending: &Ending) -> bool {
    matches!(ending, Ending::Failed | Ending::Stalled | Ending::Stopped(_))
}

/// **Whether a pending mark may go with an ending that says the work is
/// over** — S15's paragraph of 2026-09-29, the runner's ruling under the
/// Creator's standing delegation.
///
/// **Why it goes at all.** Kept, the mark made one failed publication — the
/// user's own file saved at the name while it downloaded, which `Desktop app
/// and delivery` C12 refuses to write over — refuse every download of that
/// server, for good, with nothing said: `Download`'s admission refuses a server
/// while any of its records holds one, and nothing would ever retry it.
///
/// **Why it sometimes stays.** On a download the platform adapter makes a
/// zero-byte reservation at the final name (`Desktop app and delivery` C10) and
/// takes it back when the rename over it fails — only where the object is
/// still the one it made. **Where a zero-byte object stands at the final name
/// of a transfer that staged bytes, it may be that reservation, not taken
/// back**, and the mark is what lets the next start's C13 find and remove it.
/// Cleared here, it would be named by no record at all. **Nothing is removed
/// here on that reading**; the reading only decides whether the mark stays.
///
/// **An upload has no reservation**: `Desktop app and delivery` C10 is the
/// receiving filesystem on the user's own computer, and a create-intended
/// upload uses the base rename, whose refusal leaves the final name alone.
fn the_pending_mark_may_go<S, L, B, K, W>(
    transfer: &mut Transfer<'_, S, L, B, K, W>,
    record: &TransferRecord,
) -> bool
where
    S: AsyncRead + AsyncWrite + Unpin + Send,
    L: TheUsersOwnComputer,
    B: DrawsFromTheSharedBudget,
    K: KeepsTheRecord,
    W: WatchesTheTransfer,
{
    let Some(pending) = record.publication_pending.as_ref() else {
        return true;
    };
    match record.direction {
        Direction::Up => true,
        Direction::Down => {
            let at_the_final_name = transfer.local.read_the_final_object(&record.final_path);
            let a_reservation_may_stand = at_the_final_name.exists
                && at_the_final_name.byte_length == Some(0)
                && pending.staged_byte_length > 0;
            !a_reservation_may_stand
        }
    }
}

async fn the_loop<S, L, B, K, W>(
    transfer: &mut Transfer<'_, S, L, B, K, W>,
    record: &mut TransferRecord,
    report: &mut Report,
    mut gate: Option<&mut dyn TheCallersPublicationGate>,
    the_source_now: Option<&SourceReading>,
    the_staging_folder: TheStagingFolder,
) -> WhyTheLoopStopped
where
    S: AsyncRead + AsyncWrite + Unpin + Send,
    L: TheUsersOwnComputer,
    B: DrawsFromTheSharedBudget,
    K: KeepsTheRecord,
    W: WatchesTheTransfer,
{
    // **S8: durable before the first byte is written.** A record written at the
    // end of a transfer is a record a closing process never writes.
    if transfer.keeper.write_it_down(record).is_err() {
        return WhyTheLoopStopped::ItWasInterrupted;
    }
    // **R6's time half, and it is the one that matters for G4.** The moment the
    // last count went out, so that the gap between reports is bounded in time
    // and not only in bytes. It is set at the head of every attempt, where the
    // first report of that attempt goes out.
    let mut last_reported_at: Instant;
    // **A start-over inside one run, bounded at one for this cause.** See where
    // it is set.
    let mut a_short_source_has_already_started_it_over = false;

    // **One turn of this loop is one attempt.** There is a second turn in
    // exactly one case: a **download** whose source ends short starts over into
    // a fresh staging path rather than stopping, and a start-over is never a
    // repair in place — so the whole plan is taken again against the fresh path
    // rather than patched.
    loop {
        // **S12a: a fresh file takes none of the three asks** — the staging folder, the staged part's size and its removal.
        // Read again at each turn, so a start-over is never fresh.
        let fresh = the_staging_folder == TheStagingFolder::MadeByTheCallersRequest
            && record.direction == Direction::Up
            && it_holds_no_staged_part(record);

        // ---- S12 step 1: what is staged.
        //
        // **C19 on every wait before the bytes as well as during them**, from
        // 2026-10-01 (step 10c's twelfth-pass fix piece 1): this reading, the
        // folder, the staging object's opening and the prefix's read-back
        // each waited on the machine with no deadline.
        //
        // **A fresh file has nothing staged and is not asked** (S12a): its
        // staging name was synthesised from an identity
        // issued for it, inside a folder its own request made.
        let staged = match record.direction {
            Direction::Up if fresh => None,
            Direction::Up => match tokio::time::timeout(
                transfer.inactivity_deadline,
                transfer.subsystem.stat(&record.staging_path.as_bytes()),
            )
            .await
            {
                Err(_) => return WhyTheLoopStopped::TheDeadlineExpired,
                Ok(Ok(attributes)) => attributes.size,
                Ok(Err(failure)) if failure.is_the_path_not_being_there() => None,
                Ok(Err(SubsystemFailure::TheChannelIsGone)) => {
                    return WhyTheLoopStopped::ItWasInterrupted
                }
                Ok(Err(_)) => None,
            },
            Direction::Down => transfer.local.staged_size(&record.staging_path.as_text()),
        };

        let what = what_to_verify(record, staged);

        // ---- **S13's unchanged-source gate, asked at the head of the loop and
        // before the staged prefix is verified.**
        let mut carried_forward: Option<Running> = None;
        let plan = match the_unchanged_source_gate(record, staged, &what, the_source_now) {
            Some(WhatTheSourceGateDoes::StopAndReportThatTheSourceChanged) => {
                return WhyTheLoopStopped::TheSourceChanged
            }
            Some(WhatTheSourceGateDoes::StartOver(cause)) => ResumePlan::StartOverFromZero(cause),
            Some(WhatTheSourceGateDoes::TheResumeMayProceed) | None => match &what {
                WhatToVerify::ThePrefixUpTo(checkpoint) => {
                    // **G4b: a transfer verifying a staged prefix reports that
                    // it is verifying, and the unmoving count is never the
                    // whole report.**
                    report.verifying_a_prefix_of = Some(checkpoint.offset);
                    report.bytes_staged = checkpoint.offset;
                    transfer.watcher.report(report);
                    // **The prefix crosses once and not twice.** The read
                    // returns its running value, the comparison is made on that
                    // value, and the same value is carried into the byte loop —
                    // so a resume that carries on does not read the whole
                    // prefix a second time to rebuild a digest it has just
                    // computed. The plan prices this crossing at **one**, and
                    // measured it at about **twelve minutes** on a nearly
                    // complete 4 GiB prefix at 5.9 MB/s.
                    let running = match read_the_prefix(transfer, record, checkpoint.offset).await {
                        Ok(running) => running,
                        Err(why) => return why,
                    };
                    let said =
                        compare(&checkpoint.digest_of_everything_up_to_it, &running.so_far());
                    carried_forward = Some(running);
                    after_verifying(&what, said)
                }
                other => after_verifying(other, WhatTheComparisonSaid::TheyAgree),
            },
        };

        // ---- S12 steps 4 and 5, and S15's two publication branches.
        let mut running = Running::new();
        let mut from = match plan {
            ResumePlan::RetryThePublication => {
                report.verifying_a_prefix_of = None;
                match the_publication_recovery(transfer, record).await {
                    Err(why) => return why,
                    Ok(verdict) => match verdict {
                    // **The staging path is still there**, so publication is
                    // what is retried — S15's first bullet.
                    None => return publish(transfer, record).await,
                    // **A publication whose staging path no longer exists is
                    // not assumed to have happened**, and the recovery
                    // judgement is what says which of the four it was. Before
                    // this was wired in, the rename was retried from a path the
                    // rename itself had already taken away.
                    Some(RecoveryVerdict::ThePublicationHappened) => {
                        record.publication_pending = None;
                        let _ = transfer.keeper.write_it_down(record);
                        return WhyTheLoopStopped::ItFinishedAndPublished;
                    }
                    Some(RecoveryVerdict::TheUserRemovedTheStaging) => {
                        return WhyTheLoopStopped::TheUserRemovedTheStaging
                    }
                    Some(RecoveryVerdict::StartAFreshTransferFromZero)
                    | Some(RecoveryVerdict::TheProofCouldNotBeComposedSoStartAFresh) => {
                        // **It never converts absence into success**: the
                        // transfer starts from zero into a fresh staging path.
                        //
                        // **No G4a mark is set here, and that is deliberate.**
                        // G4a names two causes and this is neither; the mark
                        // exists so that a count *falling* is never left to
                        // stand as the whole report, and nothing has been
                        // reported for this run at all, so nothing falls.
                        record.start_over();
                        if transfer.keeper.write_it_down(record).is_err() {
                            return WhyTheLoopStopped::ItWasInterrupted;
                        }
                        0
                    }
                    },
                }
            }
            ResumePlan::CarryOnFrom(offset) => {
                // The running digest is the one the verification has just
                // built, so the end-to-end digest covers the whole file rather
                // than the tail — and the prefix is not read again for it.
                match carried_forward.take() {
                    Some(carried) => running = carried,
                    None => return WhyTheLoopStopped::ItWasInterrupted,
                }
                offset
            }
            ResumePlan::StartFromZeroWithNothingStaged => {
                // **Nothing started over**: no attempt number moves, no cause
                // is reported, and G4a's mark is not set. **The checkpoints are
                // cleared all the same** — a record carrying a checkpoint with
                // no staged file behind it names a high-water mark the disk
                // never reached, and S12 step 2 would take it at the next
                // resume.
                //
                // **Written only where there was something to clear.** A fresh
                // transfer reaches here with an empty list, and writing it
                // down again put the record down a second time before its
                // first byte with nothing in it changed — S8's write had just
                // gone down.
                if !record.checkpoints.is_empty() {
                    record.checkpoints.clear();
                    if transfer.keeper.write_it_down(record).is_err() {
                        return WhyTheLoopStopped::ItWasInterrupted;
                    }
                }
                0
            }
            ResumePlan::StartOverFromZero(cause) => {
                record.start_over();
                // **G4a's mark**, set when the fresh staging path is opened.
                report.started_over = Some(cause);
                if transfer.keeper.write_it_down(record).is_err() {
                    return WhyTheLoopStopped::ItWasInterrupted;
                }
                0
            }
        };
        // **G4b's mark is cleared when the first byte above the checkpoint is
        // written** — set to nothing here, before that byte moves.
        report.verifying_a_prefix_of = None;
        report.bytes_staged = from;
        transfer.watcher.report(report);
        last_reported_at = Instant::now();

        // ---- S1's Smoozy-named directory beside the final name.
        //
        // **It is made rather than assumed**: staging must sit on the same
        // filesystem as the final name, so the directory is a sibling of that
        // name and is created where it is not there. A directory that is
        // already there is the ordinary case and not a fault.
        //
        // **A fresh file's folder was made by its own request a moment ago**
        // and is not asked for again (S12a). Where it
        // turns out not to be there, the exclusive create says so and the
        // folder is made then ([`open_the_staging`]).
        match record.direction {
            Direction::Up if fresh => {}
            Direction::Up => {
                if let Err(why) = the_staging_folder_is_made(transfer, record).await {
                    return why;
                }
            }
            // The platform file adapter makes the directory as part of creating
            // the staging file, which is `Desktop app and delivery` §6's own
            // contract.
            Direction::Down => {}
        }

        // ---- The staging object, and its identity.
        //
        // **A download's fresh staging object has its identity on the record
        // already** — S9's moment is the creation, and [`open_the_staging`]
        // writes it there, before the machine's file is opened. What reaches
        // this block is an upload, whose identity is recorded unavailable, and
        // a download resumed into a staging object an earlier record never
        // named.
        let handle = match open_the_staging(transfer, record, from, fresh).await {
            Ok(handle) => handle,
            Err(why) => return why,
        };
        // **S15c: a download's source is read for its time on the handle just
        // opened**, handed over without waiting, so it costs no round trip of
        // its own; its answer is taken before the pre-state. An upload's call
        // is handed over beside the close, below.
        let mut the_time_asked = match record.direction {
            Direction::Down => transfer.subsystem.send_a_reading_of(&handle).await.ok(),
            Direction::Up => None,
        };
        if record.staging_identity.is_none() {
            record.staging_identity = Some(the_staging_identity(transfer, record).await);
            let _ = transfer.keeper.write_it_down(record);
        }

        // ---- The byte loop itself.
        let chunk = match record.direction {
            Direction::Up => transfer.subsystem.announcement().limits.write_chunk(),
            Direction::Down => transfer.subsystem.announcement().limits.read_chunk(),
        };
        let mut since_the_last_checkpoint = 0u64;
        let mut the_download_source_ended_short = false;

        // **R6a's read path: a download keeps several reads in flight.** It
        // has its own loop, [`the_reads_come_down`], because what it waits on
        // is the oldest read's answer rather than a pipeline of writes to
        // settle; the loop below is the upload's and is unchanged.
        if record.direction == Direction::Down {
            match the_reads_come_down(
                transfer,
                record,
                report,
                &handle,
                &mut running,
                &mut from,
                &mut last_reported_at,
                chunk,
            )
            .await
            {
                Ok(TheReadsCameTo::EveryByte) => {}
                Ok(TheReadsCameTo::TheSourceEndedShort) => the_download_source_ended_short = true,
                Err(why) => return why,
            }
        }

        while record.direction == Direction::Up && from < record.total_size_expected {
            let wanted = chunk.min((record.total_size_expected - from) as usize);
            // **R2.** The loop moves no more than it drew, and it sizes each
            // write to the allowance rather than always to the full chunk (R6).
            let allowed = transfer.budget.draw(wanted);
            if allowed == 0 {
                // **R7: a channel that drew nothing waits and does not spin.**
                tokio::time::sleep(THE_ACCOUNTING_PERIOD).await;
                // **And it still reports.** A channel holding an allowance it
                // cannot spend is exactly the case where the figure does not
                // move, and G4's bound is on time rather than on bytes.
                if last_reported_at.elapsed() >= THE_ACCOUNTING_PERIOD {
                    transfer.watcher.report(report);
                    last_reported_at = Instant::now();
                }
                continue;
            }

            // **The upload's half.** A download's bytes come down in
            // [`the_reads_come_down`] above, and never reach this loop.
            let moved = {
                let bytes = match transfer.local.read_at(&record.source_path, from, allowed) {
                    Ok(bytes) => bytes,
                    // **A refusal from the user's own disk is *failed***, not
                    // *interrupted*: a reconnection cannot clear it and
                    // retrying at every reconnection says nothing the user can
                    // act on.
                    Err(_) => {
                        let _ = close_quietly(transfer, &handle).await;
                        return WhyTheLoopStopped::ALocalFailureAReconnectionCannotClear;
                    }
                };
                if bytes.is_empty() {
                    // The source is shorter than the record said. **S13's
                    // changed-source case on the upload side stops and is
                    // reported** rather than starting over: a silent restart
                    // re-sends a whole file over the user's own line for a save
                    // the user made, says nothing about the save, and drops the
                    // offer Row 201's third clause promises.
                    let _ = close_quietly(transfer, &handle).await;
                    return WhyTheLoopStopped::TheSourceChanged;
                }
                running.take(&bytes);
                // **C19 on the write's own wait, which had none until
                // 2026-10-01** — step 10c's twelfth-pass fix piece 1. The
                // write waits for room in the channel whenever more is in
                // flight than the machine has acknowledged, and the channel
                // is woken only when the machine opens its window again. **A
                // machine that stops acknowledging leaves the write waiting
                // for ever**: on the twelfth pass a 1 GB upload stood at 916.8
                // MB for seven minutes, its request under way, with every
                // other wait in this loop bounded and this one not. **This is
                // the wait C19 is about** — the loop drew its allowance and
                // cannot spend it — so its expiry is *stalled*. A connection
                // that has ended answers it at once instead, *interrupted*,
                // through the point that lent the channel (`Desktop app and
                // delivery`'s `ALentChannel`).
                match a_wait_on_the_machine(
                    transfer.inactivity_deadline,
                    &mut *transfer.watcher,
                    report,
                    &mut last_reported_at,
                    transfer.subsystem.send_a_write(&handle, from, &bytes),
                )
                .await
                {
                    Ok(Ok(())) => {}
                    Ok(Err(_)) => return WhyTheLoopStopped::ItWasInterrupted,
                    Err(why) => return why,
                }
                bytes.len()
            };
            if moved < allowed {
                transfer.budget.give_back(allowed - moved);
            }
            from += moved as u64;
            since_the_last_checkpoint += moved as u64;
            report.bytes_staged = from;

            // **The count reaches the watcher at least once per accounting
            // period.** It used to reach nobody until a checkpoint, which is
            // four mebibytes: at a 50 KB/s cap across four transfers that is
            // five and a half minutes of a number that does not move on a
            // transfer working perfectly. G4 requires the bound in time and not
            // only in bytes, and this is the constant R6 already names.
            if last_reported_at.elapsed() >= THE_ACCOUNTING_PERIOD {
                transfer.watcher.report(report);
                last_reported_at = Instant::now();
            }

            // The pipeline is bounded rather than open — see
            // [`THE_MOST_WRITES_IN_FLIGHT`].
            if transfer.subsystem.writes_in_flight() >= THE_MOST_WRITES_IN_FLIGHT {
                match a_wait_on_the_machine(
                    transfer.inactivity_deadline,
                    &mut *transfer.watcher,
                    report,
                    &mut last_reported_at,
                    transfer.subsystem.settle_the_writes(),
                )
                .await
                {
                    Ok(Ok(())) => {}
                    Err(why) => return why,
                    Ok(Err(_)) => return WhyTheLoopStopped::ItWasInterrupted,
                }
            }

            if since_the_last_checkpoint >= THE_CHECKPOINT_INTERVAL
                || from == record.total_size_expected
            {
                // **The writes are settled before the checkpoint is recorded**,
                // so the record is not further ahead of the disk than R6a's
                // pipelining already makes it. S10a's `min` covers what is left.
                match a_wait_on_the_machine(
                    transfer.inactivity_deadline,
                    &mut *transfer.watcher,
                    report,
                    &mut last_reported_at,
                    transfer.subsystem.settle_the_writes(),
                )
                .await
                {
                    Ok(Ok(())) => {}
                    // **C19's expiry ends the channel and the record says
                    // *stalled***, which is its own ending from 2026-08-31: no
                    // drop occurred, so no replaced connection ever runs C11
                    // step 4 over it, and a row left saying *interrupted* would
                    // wait for a connection that never went away.
                    Err(why) => return why,
                    Ok(Err(_)) => return WhyTheLoopStopped::ItWasInterrupted,
                }
                record.checkpoints.push(Checkpoint {
                    offset: from,
                    digest_of_everything_up_to_it: running.so_far(),
                });
                if transfer.keeper.write_it_down(record).is_err() {
                    return WhyTheLoopStopped::ItWasInterrupted;
                }
                since_the_last_checkpoint = 0;
                transfer.watcher.report(report);
                last_reported_at = Instant::now();
            }
        }

        if the_download_source_ended_short {
            // **C19**: the start-over below opens the source again on this
            // same channel, so a close that never answered ends it here.
            if let Err(why) = close_quietly(transfer, &handle).await {
                return why;
            }
            // **The gate function already answers this exact question**, so it
            // is asked rather than reproduced: on a download a changed source
            // is a start-over into a fresh staging path, with the transfer
            // neither failed nor ended.
            match what_the_source_gate_does(record.direction, WhatTheSourceSaid::ItChanged) {
                WhatTheSourceGateDoes::StartOver(cause)
                    if !a_short_source_has_already_started_it_over =>
                {
                    a_short_source_has_already_started_it_over = true;
                    record.start_over();
                    // **G4a's mark**, and the count truthfully goes back to
                    // nothing on a transfer nothing has ended.
                    report.started_over = Some(cause);
                    report.bytes_staged = 0;
                    if transfer.keeper.write_it_down(record).is_err() {
                        return WhyTheLoopStopped::ItWasInterrupted;
                    }
                    transfer.watcher.report(report);
                    continue;
                }
                // **A second short end on a fresh attempt is the same source
                // ending at the same place**, and nothing about a third attempt
                // differs — so the run ends here rather than turning for ever.
                // The staged material is **kept** and the work may be carried
                // on, which is what *interrupted* means; a later carry-on
                // compares S13's four values before it adds a byte (the
                // caller's reading, [`carry_out_with_the_source_as_it_stands`]).
                WhatTheSourceGateDoes::StartOver(_)
                | WhatTheSourceGateDoes::TheResumeMayProceed => {
                    return WhyTheLoopStopped::ItWasInterrupted
                }
                WhatTheSourceGateDoes::StopAndReportThatTheSourceChanged => {
                    return WhyTheLoopStopped::TheSourceChanged
                }
            }
        }

        // **The final settle carries the deadline too.** It is the last place a
        // machine that has stopped answering can leave the loop waiting for
        // ever, and it was the one settle of the four that had no deadline on
        // it.
        match a_wait_on_the_machine(
            transfer.inactivity_deadline,
            &mut *transfer.watcher,
            report,
            &mut last_reported_at,
            transfer.subsystem.settle_the_writes(),
        )
        .await
        {
            Ok(Ok(())) => {}
            Err(why) => return why,
            Ok(Err(_)) => return WhyTheLoopStopped::ItWasInterrupted,
        }

        // **S20: the staged bytes are made durable before publication, where
        // the server offers it.** *"It does not close Fact 44."*
        match record.direction {
            Direction::Up => {
                if transfer
                    .subsystem
                    .announcement()
                    .announces(extension::FSYNC)
                    // **C19**: what the machine answers is not read, but a
                    // machine that never answers ends the transfer *stalled*.
                    && tokio::time::timeout(
                        transfer.inactivity_deadline,
                        transfer.subsystem.make_durable(&handle),
                    )
                    .await
                    .is_err()
                {
                    return WhyTheLoopStopped::TheDeadlineExpired;
                }
            }
            Direction::Down => {
                let _ = transfer.local.make_durable(&record.staging_path.as_text());
            }
        }
        // **S15c: the staged file on the server takes the source's time**,
        // after the last byte and the flush. The call is handed over here,
        // beside the close, so it costs no round trip of its own; its answer is
        // taken before the pre-state.
        if record.direction == Direction::Up {
            if let Some(seconds) = transfer
                .local
                .the_sources_time(&record.source_path)
                .and_then(|seconds| u32::try_from(seconds).ok())
            {
                the_time_asked = transfer
                    .subsystem
                    .send_the_times(&record.staging_path.as_bytes(), seconds)
                    .await
                    .ok();
            }
        }
        // **C19**: the last wait on the machine before publication. A close
        // that never answers ends the transfer *stalled*, and nothing is
        // published over a channel that stopped answering.
        if let Err(why) = close_quietly(transfer, &handle).await {
            return why;
        }

        // ---- S21's second stretch: the end-to-end digest.
        let staged_digest = running.so_far();
        if let Some(expected) = expected_digest(record) {
            if compare(&expected, &staged_digest) == WhatTheComparisonSaid::TheyDisagree {
                return WhyTheLoopStopped::TheDigestsDisagreed;
            }
        }

        // ---- **S15b's gate, and this is the moment the clause names.**
        //
        // **After the pass-through digest**, because the gate is handed that
        // digest and a gate asked earlier would be handed the digest of part of
        // the file. **Before the pre-state below**, because a pre-state is what
        // makes a publication recoverable and retried, so a record written past
        // a refusal would be retried past it too.
        //
        // **Asked once.** `take()` is what says so: a second turn of this loop
        // cannot reach it, and neither can the publication retry above, whose
        // pre-state is already durable — S15b's last bullet states what that
        // leaves.
        if let Some(gate) = gate.take() {
            if gate.may_this_be_published(&staged_digest) == WhatTheGateSaid::TheSourceChangedUnderIt
            {
                return WhyTheLoopStopped::TheSourceChanged;
            }
        }

        // ---- **S15c: the time is in place before S15's pre-state**, so the
        // identity the pre-state records, the rename carries and every later
        // reading takes is the identity after it was set.
        if let Err(why) = the_time_is_kept(transfer, record, the_time_asked).await {
            return why;
        }

        // ---- S15's pre-state, then publication.
        let identity = record
            .staging_identity
            .clone()
            .unwrap_or(StableFileIdentity::TheSupplierCouldNotAnswer);
        record.publication_pending = Some(PublicationPreState {
            intent: transfer.intent,
            staging_path: record.staging_path.as_text(),
            final_path: record.final_path.clone(),
            staged_identity: identity,
            staged_byte_length: from,
            staged_digest,
        });
        if transfer.keeper.write_it_down(record).is_err() {
            return WhyTheLoopStopped::ItWasInterrupted;
        }
        return publish(transfer, record).await;
    }
}

/// **S15c: a transfer keeps its source's modification time.** The answer to
/// the call handed over earlier is taken here: on an upload, the server's
/// answer to setting the staged file's time; on a download, the server file's
/// time, which the staged file on this computer then takes.
///
/// **A refusal does not fail the transfer**: the file is published as it is
/// and keeps the time it arrived with. **Only a machine that stops answering
/// ends it**, *stalled*, as every other wait on it does (C19).
async fn the_time_is_kept<S, L, B, K, W>(
    transfer: &mut Transfer<'_, S, L, B, K, W>,
    record: &TransferRecord,
    the_time_asked: Option<u32>,
) -> Result<(), WhyTheLoopStopped>
where
    S: AsyncRead + AsyncWrite + Unpin + Send,
    L: TheUsersOwnComputer,
    B: DrawsFromTheSharedBudget,
    K: KeepsTheRecord,
    W: WatchesTheTransfer,
{
    let Some(id) = the_time_asked else {
        return Ok(());
    };
    let deadline = transfer.inactivity_deadline;
    match record.direction {
        Direction::Up => {
            // **What it answered is not read**: a refusal leaves the file
            // with the time it arrived with, and nothing else moves.
            let _ = tokio::time::timeout(deadline, transfer.subsystem.the_times_answered(id))
                .await
                .map_err(|_| WhyTheLoopStopped::TheDeadlineExpired)?;
        }
        Direction::Down => {
            let read = tokio::time::timeout(deadline, transfer.subsystem.the_reading_answered(id))
                .await
                .map_err(|_| WhyTheLoopStopped::TheDeadlineExpired)?;
            if let Some(seconds) = read.ok().and_then(|read| read.modification_time) {
                let _ = transfer
                    .local
                    .keep_the_time(&record.staging_path.as_text(), i64::from(seconds));
            }
        }
    }
    Ok(())
}

/// **Ruling 1: the unchanged-source gate, asked on a download resume.**
///
/// S13: *"Resuming also requires that the source has not changed under the
/// transfer, and the two directions are answered by different means."* For a
/// file **on the server** the four values — size, inode, modification time and
/// change time — are recorded when the download is arranged and compared before
/// a resume, and **any difference starts the transfer over rather than resuming
/// it**. It was posed live, where a byte changed mid-fetch and *"the copy
/// returned 0 and delivered a different file"*, with only the timestamps
/// revealing it.
///
/// **Downloads only, and that scope limit is deliberate.** On an upload S13 puts
/// the unchanged-source question in `File Browser`'s hands rather than this
/// document's, and applying the unavailable branch there would make every upload
/// resume start over.
///
/// **It is asked only where a resume is actually in question** — where there is
/// staged material to carry on, and where the record is not already publication
/// pending, which S15's first bullet keeps out of S12 altogether.
///
/// **`read_now` is the reading the caller took just before handing the record
/// back** ([`carry_out_with_the_source_as_it_stands`]). **Nothing here is a
/// claim about the user's file**, and nothing is guessed: where either reading
/// is absent — none recorded when the download was arranged, or none taken now
/// — the answer is *could not be confirmed*, which is S13's own branch: *"Where
/// it is unavailable the resume simply does not happen and the transfer starts
/// over, which is the same outcome as any difference, so nothing above
/// changes."*
///
/// *(Until 2026-10-04 the reading taken now was answered by a function that
/// returned nothing, whose comment called the reading permanently unresolved.
/// It was not: the same command-channel reading is taken on every rental at
/// submission and by the dependency check. What was missing was a caller at
/// the moment a download is arranged and before it resumes — the 38's piece 9,
/// item 18.)*
fn the_unchanged_source_gate(
    record: &TransferRecord,
    staged: Option<u64>,
    what: &WhatToVerify,
    read_now: Option<&SourceReading>,
) -> Option<WhatTheSourceGateDoes> {
    if record.direction != Direction::Down {
        return None;
    }
    if matches!(what, WhatToVerify::NothingBecausePublicationIsPending) {
        return None;
    }
    // Nothing is staged, so there is no resume for the gate to be about.
    staged?;
    let said = compare_the_source(record.source_reading.as_ref(), read_now);
    Some(what_the_source_gate_does(record.direction, said))
}

/// **S15's recovery, asked where a publication-pending record meets a staging
/// path that is no longer there.**
///
/// Answers `None` where the staging path is still there, which is the ordinary
/// retry of S15's first bullet: *"A staged file that is already complete with
/// the final name absent is therefore finished rather than left where it sits."*
///
/// Where it is gone, the final object is read and
/// [`judge_the_recovery`] gives the verdict. **The judgement was written and
/// tested and called by nothing**, so the loop retried the rename from a path
/// the rename had already taken away — for ever, and reporting *interrupted*
/// every time.
async fn the_publication_recovery<S, L, B, K, W>(
    transfer: &mut Transfer<'_, S, L, B, K, W>,
    record: &mut TransferRecord,
) -> Result<Option<RecoveryVerdict>, WhyTheLoopStopped>
where
    S: AsyncRead + AsyncWrite + Unpin + Send,
    L: TheUsersOwnComputer,
    B: DrawsFromTheSharedBudget,
    K: KeepsTheRecord,
    W: WatchesTheTransfer,
{
    let Some(pre_state) = record.publication_pending.clone() else {
        return Ok(None);
    };
    let staging_is_still_there = match record.direction {
        // **C19**, from 2026-10-01: a reading that is never answered ends the
        // transfer *stalled* rather than being read as *not there*.
        Direction::Up => tokio::time::timeout(
            transfer.inactivity_deadline,
            transfer
                .subsystem
                .stat(record.staging_path.as_bytes().as_slice()),
        )
        .await
        .map_err(|_| WhyTheLoopStopped::TheDeadlineExpired)?
        .is_ok(),
        Direction::Down => transfer
            .local
            .staged_size(&record.staging_path.as_text())
            .is_some(),
    };
    if staging_is_still_there {
        return Ok(None);
    }
    // **A transfer-aware user deletion has already recorded *staging removed by
    // user* before removing the path**, so recovery takes that explicit branch
    // without inspecting the final name.
    let the_user_had_recorded_a_removal = matches!(
        record.ending,
        Some(Ending::Stopped(StoppedReason::StagingRemovedByUser))
    );
    let mut final_object = the_final_object(transfer, record).await?;
    // **S15: a lost rename answer needs content proof**, even when the
    // subsystem cannot supply an identity. Read the destination through the
    // same bounded read path as S12's prefix check. Presence or size alone
    // never settles an interrupted publication.
    if record.direction == Direction::Up
        && final_object.exists
        && final_object.byte_length == Some(pre_state.staged_byte_length)
    {
        let running = read_the_upload_back(
            transfer,
            record.final_path.as_bytes(),
            pre_state.staged_byte_length,
        )
        .await?;
        final_object.at_rest_digest = Some(running.so_far());
    }
    Ok(Some(judge_the_recovery(
        &pre_state,
        the_user_had_recorded_a_removal,
        &final_object,
    )))
}

/// What the final object reads as, per end. **Every field is an option because
/// a route may not answer**: on the machine the byte length always answers on
/// the subsystem and neither the identity nor the at-rest digest has a route
/// here during the ordinary rename check; recovery reads the upload back
/// above. On the user's own computer the platform adapter answers.
async fn the_final_object<S, L, B, K, W>(
    transfer: &mut Transfer<'_, S, L, B, K, W>,
    record: &TransferRecord,
) -> Result<WhatTheFinalObjectReadsAs, WhyTheLoopStopped>
where
    S: AsyncRead + AsyncWrite + Unpin + Send,
    L: TheUsersOwnComputer,
    B: DrawsFromTheSharedBudget,
    K: KeepsTheRecord,
    W: WatchesTheTransfer,
{
    // **C19**, from 2026-10-01: a reading the machine never answers is the
    // channel stalling, and is never read as the object being absent.
    match record.direction {
        Direction::Up => match tokio::time::timeout(
            transfer.inactivity_deadline,
            transfer.subsystem.stat(record.final_path.as_bytes()),
        )
        .await
        {
            Err(_) => Err(WhyTheLoopStopped::TheDeadlineExpired),
            Ok(Ok(attributes)) => Ok(WhatTheFinalObjectReadsAs {
                exists: true,
                byte_length: attributes.size,
                identity: None,
                at_rest_digest: None,
            }),
            Ok(Err(_)) => Ok(WhatTheFinalObjectReadsAs::default()),
        },
        Direction::Down => Ok(transfer.local.read_the_final_object(&record.final_path)),
    }
}

/// **S15: publication is the last step, and it is one step.**
async fn publish<S, L, B, K, W>(
    transfer: &mut Transfer<'_, S, L, B, K, W>,
    record: &mut TransferRecord,
) -> WhyTheLoopStopped
where
    S: AsyncRead + AsyncWrite + Unpin + Send,
    L: TheUsersOwnComputer,
    B: DrawsFromTheSharedBudget,
    K: KeepsTheRecord,
    W: WatchesTheTransfer,
{
    let Some(pre_state) = record.publication_pending.clone() else {
        return WhyTheLoopStopped::ItWasInterrupted;
    };

    match record.direction {
        Direction::Up => {
            let announced = transfer
                .subsystem
                .announcement()
                .announces(extension::POSIX_RENAME);
            let staging = record.staging_path.as_bytes();
            let final_path = record.final_path.as_bytes();
            // **C19 on the publication's own waits**, from 2026-10-01. The
            // pre-state is already durable, so an expiry here leaves exactly
            // what S15's recovery reads.
            let deadline = transfer.inactivity_deadline;
            let done = match which_operation(pre_state.intent, announced) {
                Operation::ThePosixRenameExtension => {
                    tokio::time::timeout(deadline, transfer.subsystem.posix_rename(&staging, final_path))
                        .await
                }
                Operation::TheBaseRename => {
                    tokio::time::timeout(deadline, transfer.subsystem.rename(&staging, final_path)).await
                }
                Operation::RemoveTheFinalNameThenTheBaseRename => {
                    // **S17's declared degradation, replace-intended only.** The
                    // removal is a deliberate recorded act — the pre-state is
                    // already durable — rather than a silent one.
                    match tokio::time::timeout(deadline, transfer.subsystem.remove(final_path)).await {
                        Err(expired) => Err(expired),
                        Ok(_) => {
                            tokio::time::timeout(deadline, transfer.subsystem.rename(&staging, final_path))
                                .await
                        }
                    }
                }
            };
            match done {
                Err(_) => return WhyTheLoopStopped::TheDeadlineExpired,
                Ok(Err(_)) => {
                    // **S15: the old connection's rename may finish between
                    // our staging read and this retry.** Its refusal can be
                    // settled only by the same whole-file proof.
                    match the_publication_recovery(transfer, record).await {
                        Ok(Some(RecoveryVerdict::ThePublicationHappened)) => {
                            record.publication_pending = None;
                            return WhyTheLoopStopped::ItFinishedAndPublished;
                        }
                        Err(why) => return why,
                        _ => return WhyTheLoopStopped::ItWasInterrupted,
                    }
                }
                Ok(Ok(())) => {}
            }
            // **S15's third bullet: the successful rename is checked before its
            // outcome is written.** On the machine the byte length always
            // answers on the subsystem; the identity and the at-rest digest are
            // read where their routes answer, and here neither does.
            let read_back = match the_final_object(transfer, record).await {
                Ok(read_back) => read_back,
                Err(why) => return why,
            };
            match check_the_rename(&pre_state, &read_back) {
                RenameCheck::ThePublicationStands(_) => {}
                // **A readable value that does not match is a failed
                // publication, not a published file** — and *failed* is what it
                // takes, from 2026-09-03. It used to take *interrupted*, so a
                // publication that landed and was not recorded was retried for
                // ever against a staging file the rename had already removed.
                RenameCheck::ThePublicationFailed => {
                    return WhyTheLoopStopped::ThePublicationCheckCameBackWrong
                }
            }
        }
        Direction::Down => {
            // **A refusal from the platform file adapter is a local failure a
            // reconnection cannot clear**, so it is *failed* rather than
            // *interrupted*.
            if transfer
                .local
                .publish(
                    &record.staging_path.as_text(),
                    &record.final_path,
                    pre_state.intent,
                )
                .is_err()
            {
                return WhyTheLoopStopped::ALocalFailureAReconnectionCannotClear;
            }
            let read_back = transfer.local.read_the_final_object(&record.final_path);
            match check_the_rename(&pre_state, &read_back) {
                RenameCheck::ThePublicationStands(_) => {}
                RenameCheck::ThePublicationFailed => {
                    return WhyTheLoopStopped::ThePublicationCheckCameBackWrong
                }
            }
        }
    }

    record.publication_pending = None;
    WhyTheLoopStopped::ItFinishedAndPublished
}

/// **One read handed over and not yet answered**, with the bytes it asked for.
struct AReadInFlight {
    id: u32,
    offset: u64,
    length: usize,
}

/// What a download's reads came to, short of an ending.
enum TheReadsCameTo {
    /// Every byte the record expects is written to the staging file.
    EveryByte,
    /// **The machine's file ended before the record's size**, which on a
    /// download is a start-over and never a stop (S13). The caller asks the
    /// gate function, exactly as it did when the reads went one at a time.
    TheSourceEndedShort,
}

/// **A download's bytes, several reads in flight — R6a's read path.** Added
/// 2026-09-30, step 10c's eleventh-pass fix piece 2.
///
/// **The case this was first written for, and the case it now also meets.**
/// The download used to send one read and wait a whole round trip for its
/// answer before sending the next, which is the synchronous shape R6a forbids
/// on the write path. That was invisible on a machine a few milliseconds away,
/// and it is what a machine across an ocean meets: at 0.2 s a round trip one
/// 50 MB file came down in **48.5 s against 7.9 s up**, and every 665 KB frame
/// paid three round trips for its three reads. **Its record and the world
/// parted company at the distance**: nothing in the loop was wrong except that
/// it assumed the far end was near.
///
/// **What pipelining costs S10a on the read side, stated as E0-1 asks.**
/// **A byte is counted when it is written to the staging file, never when its
/// read was sent**: the answers are taken oldest first, each is written at its
/// own offset, and only then do the count and the running digest move. **So a
/// checkpoint covers only bytes written without a gap from the start** — the
/// oldest outstanding read is always the one that starts where the written
/// bytes end, which the loop checks rather than assumes. What is lost to a drop
/// is the reads still in flight, which were never counted: **the record can be
/// behind the staging file and never ahead of it**, the direction S10a's `min`
/// already takes.
///
/// **A short answer is not a gap.** A read answered with fewer bytes than it
/// asked for leaves every read after it describing bytes past a hole, so those
/// are collected and set aside and the reads start again where the written
/// bytes end. **An empty answer is the file ending short**, and the reads after
/// it are collected too, so the channel is not left holding answers nobody
/// read before the source is closed and opened again.
///
/// **R2 and R7 are kept per read**: each read is sized to what it drew, a read
/// that drew nothing is not sent, and with nothing in flight and nothing drawn
/// the loop waits an accounting period and still reports. **C19's deadline**
/// bounds every wait for an answer, and its expiry is *stalled* as before.
#[allow(
    clippy::too_many_arguments,
    reason = "every one is the byte loop's own state, lent to this half of it \
              for the duration of the reads; gathering them into a shape would \
              be a second copy of the loop's state with one reader"
)]
async fn the_reads_come_down<S, L, B, K, W>(
    transfer: &mut Transfer<'_, S, L, B, K, W>,
    record: &mut TransferRecord,
    report: &mut Report,
    handle: &crate::subsystem::Handle,
    running: &mut Running,
    from: &mut u64,
    last_reported_at: &mut Instant,
    chunk: usize,
) -> Result<TheReadsCameTo, WhyTheLoopStopped>
where
    S: AsyncRead + AsyncWrite + Unpin + Send,
    L: TheUsersOwnComputer,
    B: DrawsFromTheSharedBudget,
    K: KeepsTheRecord,
    W: WatchesTheTransfer,
{
    let staging = record.staging_path.as_text();
    let total = record.total_size_expected;
    let mut in_flight: VecDeque<AReadInFlight> = VecDeque::new();
    let mut sent_to = *from;
    let mut since_the_last_checkpoint = 0u64;
    loop {
        // ---- Hand reads over while there is room, each sized to what it drew.
        while in_flight.len() < THE_MOST_READS_IN_FLIGHT && sent_to < total {
            let wanted = chunk.min((total - sent_to) as usize);
            let allowed = transfer.budget.draw(wanted);
            if allowed == 0 {
                break;
            }
            // **C19 on handing the read over**, which waits for room in the
            // channel exactly as an upload's write does — from 2026-10-01.
            match a_wait_on_the_machine(
                transfer.inactivity_deadline,
                &mut *transfer.watcher,
                report,
                last_reported_at,
                transfer.subsystem.send_a_read(handle, sent_to, allowed),
            )
            .await?
            {
                Ok(id) => in_flight.push_back(AReadInFlight {
                    id,
                    offset: sent_to,
                    length: allowed,
                }),
                Err(_) => return Err(WhyTheLoopStopped::ItWasInterrupted),
            }
            sent_to += allowed as u64;
        }

        let Some(oldest) = in_flight.pop_front() else {
            if *from >= total {
                return Ok(TheReadsCameTo::EveryByte);
            }
            // **R7: a channel that drew nothing waits and does not spin** —
            // and it still reports, since G4's bound is on time.
            tokio::time::sleep(THE_ACCOUNTING_PERIOD).await;
            if last_reported_at.elapsed() >= THE_ACCOUNTING_PERIOD {
                transfer.watcher.report(report);
                *last_reported_at = Instant::now();
            }
            continue;
        };
        // **No gap from the start**: the oldest read is the one that begins
        // where the written bytes end, or the count would be a lie.
        if oldest.offset != *from {
            return Err(WhyTheLoopStopped::ItWasInterrupted);
        }

        // **C19's deadline on the download's own read.** A machine that stops
        // answering ends the transfer *stalled* rather than holding the loop.
        // **And G4's count goes on beside the wait** (from 2026-10-01), so a
        // download whose line has gone silent reads nothing moving rather
        // than the last speed it had: the twelfth pass watched *↓ 6.1 MB/s*
        // stand for the whole minute of a cut.
        let mut bytes = match a_wait_on_the_machine(
            transfer.inactivity_deadline,
            &mut *transfer.watcher,
            report,
            last_reported_at,
            transfer.subsystem.the_read_answered(oldest.id),
        )
        .await?
        {
            Ok(bytes) => bytes,
            Err(_) => return Err(WhyTheLoopStopped::ItWasInterrupted),
        };
        if bytes.is_empty() {
            // **The download's answer is a start-over and not a stop**, which
            // is the asymmetry S13 is about. What was drawn for this read and
            // the ones after it moved nothing, so it goes back (R7).
            transfer.budget.give_back(oldest.length);
            the_rest_are_set_aside(transfer, &mut in_flight).await?;
            return Ok(TheReadsCameTo::TheSourceEndedShort);
        }
        // A far end that answers with more than was asked for is not trusted
        // with the bytes past the read's own end.
        bytes.truncate(oldest.length);

        running.take(&bytes);
        if transfer
            .local
            .write_at(&staging, oldest.offset, &bytes)
            .is_err()
        {
            return Err(WhyTheLoopStopped::ALocalFailureAReconnectionCannotClear);
        }
        let moved = bytes.len();
        if moved < oldest.length {
            transfer.budget.give_back(oldest.length - moved);
            // **A short answer**: every read after it is for bytes past a
            // hole, so they are set aside and the reads start again here.
            the_rest_are_set_aside(transfer, &mut in_flight).await?;
            sent_to = oldest.offset + moved as u64;
        }
        // **Counted when written, never when sent** (E0-1).
        *from += moved as u64;
        since_the_last_checkpoint += moved as u64;
        report.bytes_staged = *from;

        // **The count reaches the watcher at least once per accounting
        // period** (G4, R6's time half).
        if last_reported_at.elapsed() >= THE_ACCOUNTING_PERIOD {
            transfer.watcher.report(report);
            *last_reported_at = Instant::now();
        }

        // **A checkpoint covers only bytes written without a gap from the
        // start** (E0-1), which `from` is by construction. The reads still in
        // flight are ahead of it and are not in it.
        if since_the_last_checkpoint >= THE_CHECKPOINT_INTERVAL || *from == total {
            record.checkpoints.push(Checkpoint {
                offset: *from,
                digest_of_everything_up_to_it: running.so_far(),
            });
            if transfer.keeper.write_it_down(record).is_err() {
                return Err(WhyTheLoopStopped::ItWasInterrupted);
            }
            since_the_last_checkpoint = 0;
            transfer.watcher.report(report);
            *last_reported_at = Instant::now();
        }
    }
}

/// **The reads still in flight, answered and set aside** — after a short or
/// empty answer, whose later reads describe bytes past a hole or past the end.
/// What each drew and did not bring is given back (R7). Every wait carries
/// C19's deadline.
async fn the_rest_are_set_aside<S, L, B, K, W>(
    transfer: &mut Transfer<'_, S, L, B, K, W>,
    in_flight: &mut VecDeque<AReadInFlight>,
) -> Result<(), WhyTheLoopStopped>
where
    S: AsyncRead + AsyncWrite + Unpin + Send,
    L: TheUsersOwnComputer,
    B: DrawsFromTheSharedBudget,
    K: KeepsTheRecord,
    W: WatchesTheTransfer,
{
    while let Some(read) = in_flight.pop_front() {
        match tokio::time::timeout(
            transfer.inactivity_deadline,
            transfer.subsystem.the_read_answered(read.id),
        )
        .await
        {
            Ok(Ok(bytes)) => transfer
                .budget
                .give_back(read.length.saturating_sub(bytes.len())),
            Ok(Err(_)) => return Err(WhyTheLoopStopped::ItWasInterrupted),
            Err(_) => return Err(WhyTheLoopStopped::TheDeadlineExpired),
        }
    }
    Ok(())
}

/// **An upload's staged prefix, read back with several reads in flight** —
/// R6a's read path applied to S12 step 3, added 2026-09-30. It draws no rate
/// budget (G4b). The digest takes the answers oldest first, so it is the digest
/// of the prefix in order; a short answer sets the later reads aside and the
/// reads start again where the answered bytes end, and an empty one ends the
/// read-back short, which the caller reads as *did not verify*.
///
/// **Every wait carries C19's deadline, from 2026-10-01** — step 10c's
/// twelfth-pass fix piece 1. It had none, before or after the read-back was
/// pipelined, so a machine that stopped answering in the middle of a
/// verification held the transfer for ever. **G4b's sentence that C19 *does not
/// fire* on a verification is kept as it is meant**: the verification draws no
/// budget and its count does not move, and neither is read as a stall. What is
/// bounded here is one read's wait for its answer, which is the loop's own
/// attempt to move bytes and not the count.
async fn the_prefix_comes_back<S>(
    subsystem: &mut Subsystem<S>,
    handle: &crate::subsystem::Handle,
    up_to: u64,
    chunk: usize,
    running: &mut Running,
    deadline: Duration,
) -> Result<(), WhyTheLoopStopped>
where
    S: AsyncRead + AsyncWrite + Unpin + Send,
{
    let mut in_flight: VecDeque<AReadInFlight> = VecDeque::new();
    let mut sent_to = 0u64;
    let mut at = 0u64;
    loop {
        while in_flight.len() < THE_MOST_READS_IN_FLIGHT && sent_to < up_to {
            let wanted = chunk.min((up_to - sent_to) as usize);
            let id = within(deadline, subsystem.send_a_read(handle, sent_to, wanted)).await?;
            in_flight.push_back(AReadInFlight {
                id,
                offset: sent_to,
                length: wanted,
            });
            sent_to += wanted as u64;
        }
        let Some(oldest) = in_flight.pop_front() else {
            return Ok(());
        };
        if oldest.offset != at {
            return Err(WhyTheLoopStopped::ItWasInterrupted);
        }
        let mut bytes = within(deadline, subsystem.the_read_answered(oldest.id)).await?;
        bytes.truncate(oldest.length);
        let short = bytes.len() < oldest.length;
        running.take(&bytes);
        at += bytes.len() as u64;
        if short {
            while let Some(read) = in_flight.pop_front() {
                within(deadline, subsystem.the_read_answered(read.id)).await?;
            }
            if bytes.is_empty() {
                return Ok(());
            }
            sent_to = at;
        }
    }
}

/// **One wait on the machine, under C19's deadline**: its expiry is
/// *stalled*, and a failure of the channel is *interrupted* — the two endings
/// the rest of this loop gives them. Added 2026-10-01, step 10c's
/// twelfth-pass fix piece 1.
async fn within<T>(
    deadline: Duration,
    wait: impl Future<Output = Result<T, SubsystemFailure>>,
) -> Result<T, WhyTheLoopStopped> {
    match tokio::time::timeout(deadline, wait).await {
        Err(_) => Err(WhyTheLoopStopped::TheDeadlineExpired),
        Ok(Err(_)) => Err(WhyTheLoopStopped::ItWasInterrupted),
        Ok(Ok(answered)) => Ok(answered),
    }
}

/// **One wait on the machine while bytes are moving: C19's deadline on it,
/// and G4's count beside it.** Added 2026-10-01, step 10c's twelfth-pass fix
/// piece 1.
///
/// **G4's time half, kept while a wait stands.** R6 has the loop emit its
/// count at least once per [`THE_ACCOUNTING_PERIOD`] whatever the chunk size,
/// and until this the loop kept it only between waits — so a wait for the
/// machine's answer, which may last the whole deadline, left the row and the
/// speed beside it where they last stood. **The twelfth pass read *↑ 6.8
/// MB/s* over an upload that had not moved for minutes, and *↓ 6.1 MB/s*
/// through the whole minute of a cut.** So the unchanged count is reported
/// beside the wait, each period: the screen is told, and the speed it
/// composes over its trailing window falls to nothing as nothing moves.
///
/// **The expiry is *stalled*** (C19), answered as `Err`; whatever the wait
/// itself came to is the caller's to read, as `Ok`.
async fn a_wait_on_the_machine<T, W: WatchesTheTransfer>(
    deadline: Duration,
    watcher: &mut W,
    report: &Report,
    last_reported_at: &mut Instant,
    wait: impl Future<Output = T>,
) -> Result<T, WhyTheLoopStopped> {
    tokio::pin!(wait);
    let expires = tokio::time::sleep(deadline);
    tokio::pin!(expires);
    loop {
        let the_next_report = tokio::time::Instant::from_std(*last_reported_at + THE_ACCOUNTING_PERIOD);
        tokio::select! {
            biased;
            answered = &mut wait => return Ok(answered),
            () = &mut expires => return Err(WhyTheLoopStopped::TheDeadlineExpired),
            () = tokio::time::sleep_until(the_next_report) => {
                watcher.report(report);
                *last_reported_at = Instant::now();
            }
        }
    }
}

/// **S12 step 3's read-back, and the prefix crosses once.** *"Before hand-over
/// no Smoozy program is on the machine to answer a framed request, so an
/// upload's staged prefix is read back over the subsystem and the digest
/// computed in Smoozy's own loop on the desktop."* A download's staged prefix is
/// on the user's own computer and is digested there in either phase.
///
/// **It returns its running value rather than a finished digest, and the caller
/// carries that value forward.** There used to be two readers over this one
/// function — one that verified the prefix and threw its result away, and one
/// that read the whole prefix again to rebuild the running digest — so the
/// crossing the plan prices at one was paid twice.
///
/// **What the read-back costs was measured and it is linear in what is already
/// staged**: a 37,552,128-byte prefix verified in **6.6 s**, and a
/// 2,140,700,672-byte prefix in **361.4 s** — both about **5.9 MB/s**. At that
/// rate a nearly complete 4 GiB prefix takes **about twelve minutes against a
/// remaining transfer of seconds**, so the cost is highest exactly where the
/// least is left to send, and paying it twice made it twenty-four. **During the
/// verification the transfer moves no bytes and its recorded offset does not
/// change**, which is what G4b's mark is for.
async fn read_the_prefix<S, L, B, K, W>(
    transfer: &mut Transfer<'_, S, L, B, K, W>,
    record: &TransferRecord,
    up_to: u64,
) -> Result<Running, WhyTheLoopStopped>
where
    S: AsyncRead + AsyncWrite + Unpin + Send,
    L: TheUsersOwnComputer,
    B: DrawsFromTheSharedBudget,
    K: KeepsTheRecord,
    W: WatchesTheTransfer,
{
    let mut running = Running::new();
    let chunk = transfer.subsystem.announcement().limits.read_chunk();
    let staged = record.staging_path.as_text();
    match record.direction {
        Direction::Up => {
            return read_the_upload_back(transfer, staged.as_bytes(), up_to).await;
        }
        Direction::Down => {
            let mut at = 0u64;
            while at < up_to {
                let wanted = chunk.min((up_to - at) as usize);
                let bytes = transfer
                    .local
                    .read_at(&staged, at, wanted)
                    .map_err(|_| WhyTheLoopStopped::ItWasInterrupted)?;
                if bytes.is_empty() {
                    break;
                }
                running.take(&bytes);
                at += bytes.len() as u64;
            }
        }
    }
    // A prefix that is short of what was asked for did not verify, and saying
    // so here is what makes the comparison meaningful rather than a digest of
    // fewer bytes.
    if running.bytes_taken() != up_to {
        return Err(WhyTheLoopStopped::ItWasInterrupted);
    }
    Ok(running)
}

/// **S12 and S15: read a recorded byte range back once**, with R6a's bounded
/// pipeline. A short read or a lost answer is never proof of a whole file.
async fn read_the_upload_back<S, L, B, K, W>(
    transfer: &mut Transfer<'_, S, L, B, K, W>,
    path: &[u8],
    up_to: u64,
) -> Result<Running, WhyTheLoopStopped>
where
    S: AsyncRead + AsyncWrite + Unpin + Send,
    L: TheUsersOwnComputer,
    B: DrawsFromTheSharedBudget,
    K: KeepsTheRecord,
    W: WatchesTheTransfer,
{
    let deadline = transfer.inactivity_deadline;
    let chunk = transfer.subsystem.announcement().limits.read_chunk();
    let handle = within(deadline, transfer.subsystem.open_to_read(path)).await?;
    let mut running = Running::new();
    let came_back = the_prefix_comes_back(
        transfer.subsystem, &handle, up_to, chunk, &mut running, deadline,
    ).await;
    let closed = tokio::time::timeout(deadline, transfer.subsystem.close(&handle)).await;
    came_back?;
    closed.map_err(|_| WhyTheLoopStopped::TheDeadlineExpired)?
        .map_err(|_| WhyTheLoopStopped::ItWasInterrupted)?;
    if running.bytes_taken() != up_to {
        return Err(WhyTheLoopStopped::ItWasInterrupted);
    }
    Ok(running)
}

/// **S1's staging folder on the machine, made rather than assumed** — a
/// folder already there is the ordinary case and not a fault. Taken out of the
/// loop on 2026-10-16 so a fresh file whose create was refused makes the same
/// ask (S12a).
async fn the_staging_folder_is_made<S, L, B, K, W>(
    transfer: &mut Transfer<'_, S, L, B, K, W>,
    record: &TransferRecord,
) -> Result<(), WhyTheLoopStopped>
where
    S: AsyncRead + AsyncWrite + Unpin + Send,
    L: TheUsersOwnComputer,
    B: DrawsFromTheSharedBudget,
    K: KeepsTheRecord,
    W: WatchesTheTransfer,
{
    match tokio::time::timeout(
        transfer.inactivity_deadline,
        transfer
            .subsystem
            .make_directory(record.staging_path.directory().as_bytes()),
    )
    .await
    {
        Err(_) => Err(WhyTheLoopStopped::TheDeadlineExpired),
        Ok(Ok(())) | Ok(Err(SubsystemFailure::TheServerRefused(_))) => Ok(()),
        Ok(Err(_)) => Err(WhyTheLoopStopped::ItWasInterrupted),
    }
}

/// **The staging object, made or reopened, and the machine's end opened.**
///
/// **S9: the staging object's stable file identity is recorded *"once that
/// attempt's staging object was created"*, and on a download that is the moment
/// it is made** — read and written durably straight after the exclusive create
/// and **before** the machine's file is opened. The identity is a reading of
/// the user's own disk, so nothing between the create and the write needs a
/// connection, and no cut can land between the file existing and the record
/// naming it.
///
/// **The case this was first written for, and the case it now also meets.**
/// The identity used to be written after the whole opening had answered — both
/// the local create and the machine's open — which fits a cut *during* the
/// bytes. It now also meets a cut **between making the file and opening the
/// source**, which is the tenth pass's: that left a zero-byte part-file the
/// record named by path alone, and S28's judgement, which wants the path, the
/// name **and** the identity, left it in the user's own folder for good.
///
/// **Why this order and not the other one** (the machine's file opened first,
/// the local file made after). Both close the gap. This one keeps the rule in
/// S9's own words — the identity follows the creation — and leaves only local
/// acts between the two, where the other would hold a machine-side handle open
/// across a local failure that then has to close it.
///
/// **A fresh file's staging object is created without the removal before it**
/// (S12a) — nothing can stand at its name. **Where the
/// create is refused all the same**, the folder is made, the name removed and
/// the create made again, which is every ask the file would have made had it
/// not been fresh.
async fn open_the_staging<S, L, B, K, W>(
    transfer: &mut Transfer<'_, S, L, B, K, W>,
    record: &mut TransferRecord,
    from: u64,
    fresh: bool,
) -> Result<crate::subsystem::Handle, WhyTheLoopStopped>
where
    S: AsyncRead + AsyncWrite + Unpin + Send,
    L: TheUsersOwnComputer,
    B: DrawsFromTheSharedBudget,
    K: KeepsTheRecord,
    W: WatchesTheTransfer,
{
    let staged = record.staging_path.as_bytes();
    match record.direction {
        // **C19 on each of the three**, from 2026-10-01.
        Direction::Up => {
            let deadline = transfer.inactivity_deadline;
            if from == 0 && fresh {
                match tokio::time::timeout(deadline, transfer.subsystem.create_exclusively(&staged))
                    .await
                {
                    Err(_) => return Err(WhyTheLoopStopped::TheDeadlineExpired),
                    Ok(Ok(handle)) => return Ok(handle),
                    Ok(Err(SubsystemFailure::TheChannelIsGone)) => {
                        return Err(WhyTheLoopStopped::ItWasInterrupted)
                    }
                    // **The fresh reading did not hold**, so the asks are made.
                    Ok(Err(_)) => the_staging_folder_is_made(transfer, record).await?,
                }
            }
            if from == 0 {
                if tokio::time::timeout(deadline, transfer.subsystem.remove(&staged))
                    .await
                    .is_err()
                {
                    return Err(WhyTheLoopStopped::TheDeadlineExpired);
                }
                within(deadline, transfer.subsystem.create_exclusively(&staged)).await
            } else {
                within(deadline, transfer.subsystem.open_to_carry_on(&staged)).await
            }
        }
        Direction::Down => {
            let path = record.staging_path.as_text();
            if from == 0 {
                let _ = transfer.local.remove(&path);
                // **This is the call the full disk refuses**, and its refusal
                // is *failed* rather than *interrupted*: Smoozy cannot make the
                // file it needs on the user's own disk, no reconnection clears
                // that, and a row saying *waiting for the connection* on a
                // perfect line tells the user to check the one thing that is
                // not wrong.
                transfer
                    .local
                    .create_staging_exclusively(&path)
                    .map_err(|_| WhyTheLoopStopped::ALocalFailureAReconnectionCannotClear)?;
                // **S9: the identity, durably, before anything that can be
                // cut.** A record that will not take it is a file nothing would
                // ever be able to claim, so the file this call has just made —
                // empty, and made exclusively a moment ago — is taken back
                // rather than left unnamed.
                record.staging_identity = Some(the_staging_identity(transfer, record).await);
                if transfer.keeper.write_it_down(record).is_err() {
                    let _ = transfer.local.remove(&path);
                    return Err(WhyTheLoopStopped::ItWasInterrupted);
                }
            }
            // The download's bytes land through the platform adapter, so the
            // machine-side handle is one this direction never opens. The
            // subsystem handle for the *source* is opened here instead.
            //
            // **C19's deadline on the open.** It was one of the waits on a
            // transfer channel with none, so a machine that stopped answering
            // here held the channel for ever with nothing written and nothing
            // said. **Its expiry is *stalled*, exactly as a read that expires
            // is**: nothing dropped, so nothing would ever carry it on.
            match tokio::time::timeout(
                transfer.inactivity_deadline,
                transfer.subsystem.open_to_read(record.source_path.as_bytes()),
            )
            .await
            {
                Ok(Ok(handle)) => Ok(handle),
                Ok(Err(_)) => Err(WhyTheLoopStopped::ItWasInterrupted),
                Err(_) => Err(WhyTheLoopStopped::TheDeadlineExpired),
            }
        }
    }
}

/// **The close, answered quietly — and bounded by C19's deadline.**
///
/// **What the machine answers is not read**, which is what *quietly* has
/// always meant here: every caller either has its ending already or goes on to
/// the end-to-end digest, which is what says whether the bytes are right.
/// **What is new is that it cannot wait for ever.** A close the machine never
/// answers, on a channel that stays up, used to hold the loop with nothing
/// written; **its expiry is `TheDeadlineExpired`**, so a caller that goes on
/// using the channel ends *stalled* exactly as an expired read does, and a
/// caller that is already returning keeps the ending it had.
async fn close_quietly<S, L, B, K, W>(
    transfer: &mut Transfer<'_, S, L, B, K, W>,
    handle: &crate::subsystem::Handle,
) -> Result<(), WhyTheLoopStopped>
where
    S: AsyncRead + AsyncWrite + Unpin + Send,
    L: TheUsersOwnComputer,
    B: DrawsFromTheSharedBudget,
    K: KeepsTheRecord,
    W: WatchesTheTransfer,
{
    match tokio::time::timeout(transfer.inactivity_deadline, transfer.subsystem.close(handle)).await
    {
        Ok(_) => Ok(()),
        Err(_) => Err(WhyTheLoopStopped::TheDeadlineExpired),
    }
}

async fn the_staging_identity<S, L, B, K, W>(
    transfer: &mut Transfer<'_, S, L, B, K, W>,
    record: &TransferRecord,
) -> StableFileIdentity
where
    S: AsyncRead + AsyncWrite + Unpin + Send,
    L: TheUsersOwnComputer,
    B: DrawsFromTheSharedBudget,
    K: KeepsTheRecord,
    W: WatchesTheTransfer,
{
    match record.direction {
        // **On the machine it is a command-channel reading and sits on `File
        // Browser` §13 item 9's unestablished ground** (§12 item 15), and
        // revision 3's attribute structure carries **no inode and no file
        // identifier**. So the supplier cannot answer, and that is recorded as
        // a fact rather than guessed at.
        Direction::Up => StableFileIdentity::TheSupplierCouldNotAnswer,
        Direction::Down => transfer
            .local
            .stable_identity_of(&record.staging_path.as_text()),
    }
}

/// The digest the caller recorded for the whole file, where it has one.
///
/// **S25: a digest computed as the bytes pass proves the bytes read match the
/// bytes written, and no more.** It does not prove the file on the sending disk
/// was whole before Smoozy read it — that is S13's change test on the server
/// side and Row 201's on the local side.
fn expected_digest(record: &TransferRecord) -> Option<Digest> {
    record.expected_digest.clone()
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::StartOverCause;
    use smoozy_state::identities::{Identity, IdentityKind};
    use std::collections::HashMap;
    use tokio::io::{AsyncReadExt, AsyncWriteExt};

    use crate::subsystem::wire::{kind, Composed, Fields};
    use crate::subsystem::StatusCode;

    /// **A far end that serves one file, over a pipe, at the desk.**
    ///
    /// It answers exactly the four packets a download's byte loop sends — the
    /// opening `INIT`, an `OPEN` to read, byte-offset `READ`s and a `CLOSE` —
    /// and refuses everything else rather than inventing an answer. **It
    /// announces no extension**, so the negotiated limits are the draft's own
    /// recommended pair and no capability is guessed at.
    ///
    /// **This is a stand-in for a machine and not a stand-in for the loop.**
    /// The loop under test is the real one; what is faked is the wire, which is
    /// how S15b's gate is proved without a rented machine.
    fn a_far_end_serving(bytes: Vec<u8>) -> tokio::io::DuplexStream {
        a_far_end(bytes, TheFarEnd::AnswersEverything)
    }

    /// **What a far end does with the one packet it is set to mishandle.**
    /// Everything else it answers as [`a_far_end_serving`] does.
    #[derive(Clone, Copy, PartialEq, Eq)]
    enum TheFarEnd {
        AnswersEverything,
        /// S15's destination exists but its staging path is gone.
        AnswersARecoveredPublication { byte_length: u64 },
        /// **The machine's file will not open** — the tenth pass's cut landing
        /// between the staging file being made and the source being opened.
        RefusesTheOpen,
        /// **An open that never answers**, on a channel that stays up. `Server
        /// protocol and transfers` C19's case: nothing drops, so nothing ever
        /// says *interrupted*.
        NeverAnswersTheOpen,
        /// **A close that never answers**, the same case one step later.
        NeverAnswersTheClose,
    }

    fn a_far_end(bytes: Vec<u8>, behaves: TheFarEnd) -> tokio::io::DuplexStream {
        let (ours, mut theirs) = tokio::io::duplex(8 * 1024);
        tokio::spawn(async move {
            let handle: &[u8] = b"h1";
            loop {
                let mut length = [0u8; 4];
                if theirs.read_exact(&mut length).await.is_err() {
                    return;
                }
                let mut whole = vec![0u8; u32::from_be_bytes(length) as usize];
                if theirs.read_exact(&mut whole).await.is_err() {
                    return;
                }
                let (packet_kind, payload) = (whole[0], &whole[1..]);
                // **Silence is an answer never sent on a channel kept open**,
                // which is what the loop's own deadline is for.
                if (packet_kind == kind::OPEN && behaves == TheFarEnd::NeverAnswersTheOpen)
                    || (packet_kind == kind::CLOSE && behaves == TheFarEnd::NeverAnswersTheClose)
                {
                    continue;
                }
                let reply = match packet_kind {
                    kind::STAT if matches!(behaves, TheFarEnd::AnswersARecoveredPublication { .. }) => {
                        let mut fields = Fields::over(payload);
                        let id = fields.u32().unwrap_or(0);
                        let path = fields.string().unwrap_or_default();
                        if path == b"/here/frame.exr" {
                            let TheFarEnd::AnswersARecoveredPublication { byte_length } = behaves else { unreachable!() };
                            let mut composed = Composed::new();
                            composed.u32(id).u32(1).u64(byte_length);
                            composed.framed(kind::ATTRS)
                        } else {
                            a_status(id, StatusCode::Failure)
                        }
                    }
                    kind::INIT => {
                        let mut composed = Composed::new();
                        composed.u32(3);
                        composed.framed(kind::VERSION)
                    }
                    kind::OPEN if behaves == TheFarEnd::RefusesTheOpen => {
                        let mut fields = Fields::over(payload);
                        let id = fields.u32().unwrap_or(0);
                        a_status(id, StatusCode::Failure)
                    }
                    kind::OPEN => {
                        let mut fields = Fields::over(payload);
                        let id = fields.u32().unwrap_or(0);
                        let mut composed = Composed::new();
                        composed.u32(id).string(handle);
                        composed.framed(kind::HANDLE)
                    }
                    kind::READ => {
                        let mut fields = Fields::over(payload);
                        let id = fields.u32().unwrap_or(0);
                        let _handle = fields.string();
                        let offset = fields.u64().unwrap_or(0) as usize;
                        let wanted = fields.u32().unwrap_or(0) as usize;
                        if offset >= bytes.len() {
                            a_status(id, StatusCode::EndOfFile)
                        } else {
                            let end = (offset + wanted).min(bytes.len());
                            let mut composed = Composed::new();
                            composed.u32(id).string(&bytes[offset..end]);
                            composed.framed(kind::DATA)
                        }
                    }
                    kind::CLOSE => {
                        let mut fields = Fields::over(payload);
                        let id = fields.u32().unwrap_or(0);
                        a_status(id, StatusCode::Ok)
                    }
                    _ => {
                        let mut fields = Fields::over(payload);
                        let id = fields.u32().unwrap_or(0);
                        a_status(id, StatusCode::Failure)
                    }
                };
                if theirs.write_all(&reply).await.is_err() {
                    return;
                }
                if theirs.flush().await.is_err() {
                    return;
                }
            }
        });
        ours
    }

    fn a_status(id: u32, code: StatusCode) -> Vec<u8> {
        let number = match code {
            StatusCode::Ok => 0,
            StatusCode::EndOfFile => 1,
            _ => 4,
        };
        let mut composed = Composed::new();
        composed.u32(id).u32(number).string(b"").string(b"");
        composed.framed(kind::STATUS)
    }

    /// The user's own computer, in memory. **Every call the byte loop makes on
    /// this end, and no more.**
    #[derive(Default)]
    struct ADiskInMemory {
        files: HashMap<String, Vec<u8>>,
        /// **Where set, the disk answers a stable file identity**, one fresh
        /// number per file made, the way a real volume does. Unset, it answers
        /// that it cannot, which every older test here was written against.
        answers_identities: bool,
        identities: HashMap<String, u64>,
        made: u64,
        /// **Where set, a create-intended publication makes its zero-byte
        /// reservation at the final name, then fails and cannot take it
        /// back** — `Desktop app and delivery` C10's reservation left
        /// standing, which is the one case the pending mark must outlive the
        /// ending for.
        leaves_its_reservation_standing: bool,
    }

    impl ADiskInMemory {
        fn that_answers_identities() -> Self {
            ADiskInMemory {
                answers_identities: true,
                ..ADiskInMemory::default()
            }
        }
    }

    impl TheUsersOwnComputer for ADiskInMemory {
        fn staged_size(&mut self, path: &str) -> Option<u64> {
            self.files.get(path).map(|held| held.len() as u64)
        }
        fn read_at(
            &mut self,
            path: &str,
            offset: u64,
            length: usize,
        ) -> Result<Vec<u8>, LocalFailure> {
            let held = self
                .files
                .get(path)
                .ok_or_else(|| LocalFailure("no such file".into()))?;
            let from = (offset as usize).min(held.len());
            let to = (from + length).min(held.len());
            Ok(held[from..to].to_vec())
        }
        fn create_staging_exclusively(&mut self, path: &str) -> Result<(), LocalFailure> {
            if self.files.contains_key(path) {
                return Err(LocalFailure("it is already there".into()));
            }
            self.files.insert(path.to_string(), Vec::new());
            self.made += 1;
            self.identities.insert(path.to_string(), self.made);
            Ok(())
        }
        fn write_at(&mut self, path: &str, offset: u64, bytes: &[u8]) -> Result<(), LocalFailure> {
            let held = self
                .files
                .get_mut(path)
                .ok_or_else(|| LocalFailure("no such file".into()))?;
            let at = offset as usize;
            if held.len() < at + bytes.len() {
                held.resize(at + bytes.len(), 0);
            }
            held[at..at + bytes.len()].copy_from_slice(bytes);
            Ok(())
        }
        fn make_durable(&mut self, _path: &str) -> Result<(), LocalFailure> {
            Ok(())
        }
        fn the_sources_time(&mut self, _source_path: &str) -> Option<i64> {
            None
        }
        fn keep_the_time(&mut self, _staging: &str, _seconds: i64) -> Result<(), LocalFailure> {
            Ok(())
        }
        fn stable_identity_of(&mut self, path: &str) -> StableFileIdentity {
            // The identity-unavailable branch of S15, which is the honest
            // answer for a disk that has none rather than a value nobody
            // stands behind.
            match (self.answers_identities, self.identities.get(path)) {
                (true, Some(number)) => StableFileIdentity::The(format!("file-{number}")),
                _ => StableFileIdentity::TheSupplierCouldNotAnswer,
            }
        }
        fn publish(
            &mut self,
            staging: &str,
            final_path: &str,
            intent: Intent,
        ) -> Result<(), LocalFailure> {
            // **A create-intended publication never writes over a name that is
            // taken** — the platform adapter's own C12 answer, which this disk
            // keeps so the case can be met here.
            if intent == Intent::CreateIntended && self.files.contains_key(final_path) {
                return Err(LocalFailure("a file with this name already exists".into()));
            }
            if intent == Intent::CreateIntended && self.leaves_its_reservation_standing {
                self.files.insert(final_path.to_string(), Vec::new());
                return Err(LocalFailure("the rename over the reservation failed".into()));
            }
            let held = self
                .files
                .remove(staging)
                .ok_or_else(|| LocalFailure("nothing staged".into()))?;
            self.files.insert(final_path.to_string(), held);
            // A rename keeps the object, so it keeps its identity.
            if let Some(number) = self.identities.remove(staging) {
                self.identities.insert(final_path.to_string(), number);
            }
            Ok(())
        }
        fn remove(&mut self, path: &str) -> Result<(), LocalFailure> {
            self.files.remove(path);
            self.identities.remove(path);
            Ok(())
        }
        fn read_the_final_object(&mut self, path: &str) -> WhatTheFinalObjectReadsAs {
            match self.files.get(path) {
                Some(held) => WhatTheFinalObjectReadsAs {
                    exists: true,
                    byte_length: Some(held.len() as u64),
                    identity: None,
                    at_rest_digest: None,
                },
                None => WhatTheFinalObjectReadsAs::default(),
            }
        }
    }

    #[derive(Default)]
    struct WritesItDown {
        writes: usize,
    }
    impl KeepsTheRecord for WritesItDown {
        fn write_it_down(&mut self, _record: &TransferRecord) -> Result<(), LocalFailure> {
            self.writes += 1;
            Ok(())
        }
    }

    /// **A gate a caller names**, holding what it was asked and what it
    /// answered — S15b.
    struct AGateTheCallerNamed {
        answer: WhatTheGateSaid,
        asked: Vec<Digest>,
    }
    impl TheCallersPublicationGate for AGateTheCallerNamed {
        fn may_this_be_published(&mut self, what_was_sent: &Digest) -> WhatTheGateSaid {
            self.asked.push(what_was_sent.clone());
            self.answer
        }
    }

    fn a_transfer_of(bytes: &[u8]) -> TransferRecord {
        let issue = |kind| Identity::issue(kind).expect("random bytes");
        let mut record = TransferRecord::arranged(
            issue(IdentityKind::Transfer),
            Direction::Down,
            issue(IdentityKind::LocalServer),
            Some(issue(IdentityKind::Installation)),
            "/machine/frame.exr".to_string(),
            "/here/frame.exr".to_string(),
            "/here/.smoozy-staging",
            bytes.len() as u64,
            issue(IdentityKind::Run),
        );
        record.expected_digest = Some(Digest::of(bytes));
        record
    }

    /// **S15: only the whole destination with the recorded content settles
    /// a lost upload publication.** Size alone, a short read, and absence
    /// cannot claim success.
    #[tokio::test]
    async fn a_lost_upload_publication_is_proved_by_the_destinations_whole_content() {
        let expected = b"the whole uploaded file";
        for (bytes, reported, verdict) in [
            (expected.to_vec(), expected.len() as u64, RecoveryVerdict::ThePublicationHappened),
            (vec![b'x'; expected.len()], expected.len() as u64, RecoveryVerdict::StartAFreshTransferFromZero),
            (b"the part".to_vec(), 8, RecoveryVerdict::StartAFreshTransferFromZero),
            (b"the part".to_vec(), expected.len() as u64, RecoveryVerdict::StartAFreshTransferFromZero),
        ] {
            let actual = bytes.len() as u64;
            let mut subsystem = Subsystem::negotiate(a_far_end(
                bytes, TheFarEnd::AnswersARecoveredPublication { byte_length: reported },
            )).await.unwrap();
            let mut record = a_transfer_of(expected);
            record.direction = Direction::Up;
            record.publication_pending = Some(PublicationPreState {
                intent: Intent::CreateIntended,
                staging_path: record.staging_path.as_text(),
                final_path: record.final_path.clone(),
                staged_identity: StableFileIdentity::TheSupplierCouldNotAnswer,
                staged_byte_length: expected.len() as u64,
                staged_digest: Digest::of(expected),
            });
            let mut local = ADiskInMemory::default();
            let mut budget = NoCapIsInForce;
            let mut keeper = WritesItDown::default();
            let mut watcher = NobodyIsWatching;
            let mut transfer = Transfer {
                subsystem: &mut subsystem, local: &mut local, budget: &mut budget,
                keeper: &mut keeper, watcher: &mut watcher,
                inactivity_deadline: Duration::from_secs(1), intent: Intent::CreateIntended,
            };
            let recovered = the_publication_recovery(&mut transfer, &mut record).await;
            if reported == expected.len() as u64 && actual < reported {
                assert!(matches!(recovered, Err(WhyTheLoopStopped::ItWasInterrupted)));
            } else {
                assert_eq!(recovered.unwrap(), Some(verdict));
            }
            // The rename retry can also lose a race to the old connection.
            // A refusal is settled by content proof, never by the name alone.
            let retried = publish(&mut transfer, &mut record).await;
            assert_eq!(
                retried,
                if verdict == RecoveryVerdict::ThePublicationHappened {
                    WhyTheLoopStopped::ItFinishedAndPublished
                } else {
                    WhyTheLoopStopped::ItWasInterrupted
                },
            );
        }
    }

    /// **S15b, and this is the test that would have caught the fault.**
    ///
    /// Before the gate existed, a refusal could only travel on a reading that
    /// means something else — an empty upload read, which the loop treats as
    /// the changed-source case — so there was no point in the loop at which a
    /// caller's own answer entered it, and nothing to write a test against.
    #[tokio::test]
    async fn a_gate_the_caller_names_refuses_and_the_transfer_stops_with_the_source_changed() {
        let bytes = vec![7u8; 40_000];
        let mut subsystem = crate::subsystem::client::Subsystem::negotiate(a_far_end_serving(
            bytes.clone(),
        ))
        .await
        .expect("the far end announces revision 3");
        let mut record = a_transfer_of(&bytes);
        let mut local = ADiskInMemory::default();
        let mut budget = NoCapIsInForce;
        let mut keeper = WritesItDown::default();
        let mut watcher = NobodyIsWatching;
        let mut gate = AGateTheCallerNamed {
            answer: WhatTheGateSaid::TheSourceChangedUnderIt,
            asked: Vec::new(),
        };

        let happened = carry_out_under_a_publication_gate(
            Transfer {
                subsystem: &mut subsystem,
                local: &mut local,
                budget: &mut budget,
                keeper: &mut keeper,
                watcher: &mut watcher,
                inactivity_deadline: Duration::from_secs(5),
                intent: Intent::CreateIntended,
            },
            &mut record,
            &mut gate,
        )
        .await;

        // **S26's second row**, and no sixth ending was added for it.
        assert_eq!(
            happened.ending,
            Ending::Stopped(StoppedReason::TheSourceChangedUnderIt)
        );
        // **The gate was asked once**, and with the digest of what actually
        // crossed (S22).
        assert_eq!(gate.asked.len(), 1);
        assert_eq!(gate.asked[0], Digest::of(&bytes));
        // **The pre-state was never written.** That is the whole reason the
        // moment is before it: a durable *publication pending* record past a
        // refusal would be retried past it.
        assert_eq!(record.publication_pending, None);
        // §7.1 clause 1: nothing at the final name, and the staged material
        // removed on S26's own rule for this row.
        assert!(!local.files.contains_key("/here/frame.exr"));
        assert!(!local
            .files
            .contains_key(&record.staging_path.as_text()));
    }

    /// The other side of the same gate: it lets the publication through, and
    /// the transfer is in every respect the one it would have been.
    #[tokio::test]
    async fn a_gate_that_allows_it_changes_nothing_about_the_publication() {
        let bytes = vec![3u8; 40_000];
        let mut subsystem = crate::subsystem::client::Subsystem::negotiate(a_far_end_serving(
            bytes.clone(),
        ))
        .await
        .expect("the far end announces revision 3");
        let mut record = a_transfer_of(&bytes);
        let mut local = ADiskInMemory::default();
        let mut budget = NoCapIsInForce;
        let mut keeper = WritesItDown::default();
        let mut watcher = NobodyIsWatching;
        let mut gate = AGateTheCallerNamed {
            answer: WhatTheGateSaid::ItMayBePublished,
            asked: Vec::new(),
        };

        let happened = carry_out_under_a_publication_gate(
            Transfer {
                subsystem: &mut subsystem,
                local: &mut local,
                budget: &mut budget,
                keeper: &mut keeper,
                watcher: &mut watcher,
                inactivity_deadline: Duration::from_secs(5),
                intent: Intent::CreateIntended,
            },
            &mut record,
            &mut gate,
        )
        .await;

        assert_eq!(happened.ending, Ending::Published);
        assert_eq!(gate.asked.len(), 1);
        assert_eq!(local.files.get("/here/frame.exr"), Some(&bytes));
        assert!(!local
            .files
            .contains_key(&record.staging_path.as_text()));
    }

    /// **Where the caller declares none, none is asked** — and the transfer is
    /// unchanged. This is what says the gate is an addition rather than a
    /// change to the loop.
    #[tokio::test]
    async fn a_transfer_with_no_gate_declared_is_the_transfer_it_always_was() {
        let bytes = vec![9u8; 40_000];
        let mut subsystem = crate::subsystem::client::Subsystem::negotiate(a_far_end_serving(
            bytes.clone(),
        ))
        .await
        .expect("the far end announces revision 3");
        let mut record = a_transfer_of(&bytes);
        let mut local = ADiskInMemory::default();
        let mut budget = NoCapIsInForce;
        let mut keeper = WritesItDown::default();
        let mut watcher = NobodyIsWatching;

        let happened = carry_out(
            Transfer {
                subsystem: &mut subsystem,
                local: &mut local,
                budget: &mut budget,
                keeper: &mut keeper,
                watcher: &mut watcher,
                inactivity_deadline: Duration::from_secs(5),
                intent: Intent::CreateIntended,
            },
            &mut record,
        )
        .await;

        assert_eq!(happened.ending, Ending::Published);
        assert_eq!(local.files.get("/here/frame.exr"), Some(&bytes));
        // **S8: the record was durable before the first byte**, and it is
        // written again as the transfer moves.
        assert!(keeper.writes >= 2);
    }

    #[test]
    fn the_accounting_period_is_well_inside_r12as_window() {
        // R6's *many times per second per channel*, and R12a's lower bound
        // being an argument from that sample count. A three-second window holds
        // thirty of these.
        assert!(THE_ACCOUNTING_PERIOD <= Duration::from_millis(100));
        assert!(Duration::from_secs(3).as_millis() / THE_ACCOUNTING_PERIOD.as_millis() >= 30);
    }

    #[test]
    fn the_checkpoint_interval_bounds_what_a_resume_throws_away() {
        // S12 step 4: *"Bytes between the checkpoint and the staged size are
        // re-sent, which bounds the waste at one checkpoint interval."*
        assert_eq!(THE_CHECKPOINT_INTERVAL, 4 * 1024 * 1024);
    }

    #[test]
    fn the_unchanged_source_gate_is_asked_on_downloads_and_on_nothing_else() {
        // **Ruling 1's scope limit, which is deliberate.** S13 puts the
        // unchanged-source question for a file on the user's own computer in
        // `File Browser`'s hands rather than this document's, so applying the
        // unavailable branch to an upload would make every upload resume start
        // over — and upload resume is the step's own milestone, proved over a
        // real server on a second connection.
        let issue = |kind| Identity::issue(kind).expect("random bytes");
        let a_record = |direction| {
            TransferRecord::arranged(
                issue(IdentityKind::Transfer),
                direction,
                issue(IdentityKind::LocalServer),
                Some(issue(IdentityKind::Installation)),
                "/local/scene.blend".to_string(),
                "/jobs/scene.blend".to_string(),
                "/jobs/.smoozy-staging",
                4_294_967_296,
                issue(IdentityKind::Run),
            )
        };
        let a_prefix = WhatToVerify::ThePrefixUpTo(Checkpoint {
            offset: 4_194_304,
            digest_of_everything_up_to_it: Digest::of(b"the prefix"),
        });

        let read_when_arranged = SourceReading {
            size: 4_294_967_296,
            inode: "41".to_string(),
            modification_time: 1_790_000_000,
            change_time: 1_790_000_000,
        };
        let arranged_with_a_reading = |direction| {
            let mut record = a_record(direction);
            record.source_reading = Some(read_when_arranged.clone());
            record
        };

        // An upload is never asked, whatever is staged or read.
        assert_eq!(
            the_unchanged_source_gate(
                &arranged_with_a_reading(Direction::Up),
                Some(4_194_304),
                &a_prefix,
                Some(&read_when_arranged)
            ),
            None
        );
        // **A download with staged material to carry on is asked.** Where the
        // four values read now are the four recorded when it was arranged, the
        // resume goes on to S12's prefix verification — the 38's piece 9,
        // item 18: until then nothing took the reading now, and every carry-on
        // started over.
        assert_eq!(
            the_unchanged_source_gate(
                &arranged_with_a_reading(Direction::Down),
                Some(4_194_304),
                &a_prefix,
                Some(&read_when_arranged)
            ),
            Some(WhatTheSourceGateDoes::TheResumeMayProceed)
        );
        // **Any of the four different, and the file starts over**, saying the
        // file changed on the server.
        for changed in [
            SourceReading { size: 4_294_967_295, ..read_when_arranged.clone() },
            SourceReading { inode: "42".to_string(), ..read_when_arranged.clone() },
            SourceReading { modification_time: 1_790_000_001, ..read_when_arranged.clone() },
            SourceReading { change_time: 1_790_000_001, ..read_when_arranged.clone() },
        ] {
            assert_eq!(
                the_unchanged_source_gate(
                    &arranged_with_a_reading(Direction::Down),
                    Some(4_194_304),
                    &a_prefix,
                    Some(&changed)
                ),
                Some(WhatTheSourceGateDoes::StartOver(
                    StartOverCause::TheSourceChangedUnderADownload
                )),
                "{changed:?}"
            );
        }
        // **A reading that could not be taken now, or none recorded when it
        // was arranged, is *could not be confirmed*** — S13's own branch: the
        // resume does not happen and the download starts over. It is never
        // read as *the file changed*.
        for (record, now) in [
            (arranged_with_a_reading(Direction::Down), None),
            (a_record(Direction::Down), Some(&read_when_arranged)),
            (a_record(Direction::Down), None),
        ] {
            assert_eq!(
                the_unchanged_source_gate(&record, Some(4_194_304), &a_prefix, now),
                Some(WhatTheSourceGateDoes::StartOver(
                    StartOverCause::TheStagedPrefixDidNotVerify
                ))
            );
        }
        // **Nothing staged is not a resume**, so there is nothing for the gate
        // to be about and a first attempt is not turned into a start-over.
        assert_eq!(
            the_unchanged_source_gate(
                &arranged_with_a_reading(Direction::Down),
                None,
                &WhatToVerify::NothingBecauseThereIsNoStagingPathAtAll,
                Some(&read_when_arranged)
            ),
            None
        );
        // **A publication-pending record never reaches S12 at all** (S15's
        // first bullet), so the gate is not asked over it either.
        assert_eq!(
            the_unchanged_source_gate(
                &arranged_with_a_reading(Direction::Down),
                Some(4_194_304),
                &WhatToVerify::NothingBecausePublicationIsPending,
                Some(&read_when_arranged)
            ),
            None
        );
    }

    /// A keeper that holds a copy of every write, in order, so a test can say
    /// what the record said at each moment it went down.
    #[derive(Default)]
    struct KeepsEveryWrite {
        written: Vec<TransferRecord>,
    }
    impl KeepsTheRecord for KeepsEveryWrite {
        fn write_it_down(&mut self, record: &TransferRecord) -> Result<(), LocalFailure> {
            self.written.push(record.clone());
            Ok(())
        }
    }

    /// One download of `bytes` against a far end that behaves as told, with a
    /// deadline short enough for a test and an outer bound that says *for
    /// ever* in a way a test can fail on.
    async fn one_download_against(
        bytes: &[u8],
        behaves: TheFarEnd,
        local: &mut ADiskInMemory,
        keeper: &mut KeepsEveryWrite,
    ) -> Option<(WhatHappened, TransferRecord)> {
        let mut subsystem =
            crate::subsystem::client::Subsystem::negotiate(a_far_end(bytes.to_vec(), behaves))
                .await
                .expect("the far end announces revision 3");
        let mut record = a_transfer_of(bytes);
        let mut budget = NoCapIsInForce;
        let mut watcher = NobodyIsWatching;
        let happened = tokio::time::timeout(
            Duration::from_secs(10),
            carry_out(
                Transfer {
                    subsystem: &mut subsystem,
                    local,
                    budget: &mut budget,
                    keeper,
                    watcher: &mut watcher,
                    inactivity_deadline: Duration::from_millis(300),
                    intent: Intent::CreateIntended,
                },
                &mut record,
            ),
        )
        .await
        .ok()?;
        Some((happened, record))
    }

    /// **`Server protocol and transfers` S9: the staging object's identity is
    /// on the record from the moment the object exists** — and this is the
    /// tenth pass's empty part-file, made at the desk.
    ///
    /// **The case this bites on is the order the loop used to keep.** The
    /// download made its staging file, then opened the machine's file, and
    /// wrote the identity only once both had answered. A cut between the two
    /// left a zero-byte file on the user's disk that the record named by path
    /// and by nothing else, so S28's judgement — path, name **and** identity —
    /// left it where it was for good, however the record was later ended.
    #[tokio::test]
    async fn a_download_cut_before_its_source_opens_leaves_a_part_file_the_sweep_can_remove() {
        use super::super::cleanup::{what_the_sweep_derives, what_the_sweep_does, WhatTheSweepDoes};

        let bytes = vec![5u8; 40_000];
        let mut local = ADiskInMemory::that_answers_identities();
        let mut keeper = KeepsEveryWrite::default();
        let (happened, record) =
            one_download_against(&bytes, TheFarEnd::RefusesTheOpen, &mut local, &mut keeper)
                .await
                .expect("a refused open answers at once");

        // **Interrupted keeps its material** (S26), so the file is there: the
        // tenth pass's zero-byte part-file.
        assert_eq!(happened.ending, Ending::Interrupted);
        let staged = record.staging_path.as_text();
        assert_eq!(local.files.get(&staged).map(Vec::len), Some(0));

        // **The identity went down before anything that could be cut** — a
        // write carrying it with no ending yet written.
        assert!(
            keeper
                .written
                .iter()
                .any(|written| written.staging_identity.is_some() && written.ending.is_none()),
            "the staging file existed with no identity on the record before the source was opened"
        );
        assert_eq!(
            record.staging_identity,
            Some(local.stable_identity_of(&staged))
        );

        // **Given up afterwards** (S28a), the sweep's own judgement now has
        // both halves to match and removes it.
        let mut given_up = record.clone();
        given_up.ending = Some(Ending::Stopped(StoppedReason::AbandonedByItsRequest));
        let derived = what_the_sweep_derives(std::slice::from_ref(&given_up));
        assert_eq!(derived.len(), 1);
        let now = local.stable_identity_of(&staged);
        assert_eq!(
            what_the_sweep_does(&derived[0], Some(&now)),
            WhatTheSweepDoes::Remove
        );
    }

    /// **`Server protocol and transfers` C19 on the open**: a machine that
    /// never answers the open ends the transfer *stalled*, exactly as a read
    /// that expires does, and does not hold the channel for ever.
    #[tokio::test]
    async fn an_open_that_never_answers_ends_stalled_rather_than_waiting_for_ever() {
        let bytes = vec![6u8; 40_000];
        let mut local = ADiskInMemory::that_answers_identities();
        let mut keeper = KeepsEveryWrite::default();
        let (happened, record) =
            one_download_against(&bytes, TheFarEnd::NeverAnswersTheOpen, &mut local, &mut keeper)
                .await
                .expect("the loop waited for ever on an open the machine never answered");
        assert_eq!(happened.ending, Ending::Stalled);
        // **Stalled removes its material** (S26), and the record says so first.
        assert!(!local.files.contains_key(&record.staging_path.as_text()));
        assert!(!record.cleanup_is_owed);
    }

    /// **`Server protocol and transfers` C19 on the close**: the last wait
    /// before publication, bounded the same way. Nothing is published over a
    /// channel that stopped answering.
    #[tokio::test]
    async fn a_close_that_never_answers_ends_stalled_rather_than_waiting_for_ever() {
        let bytes = vec![8u8; 40_000];
        let mut local = ADiskInMemory::that_answers_identities();
        let mut keeper = KeepsEveryWrite::default();
        let (happened, record) =
            one_download_against(&bytes, TheFarEnd::NeverAnswersTheClose, &mut local, &mut keeper)
                .await
                .expect("the loop waited for ever on a close the machine never answered");
        assert_eq!(happened.ending, Ending::Stalled);
        assert!(!local.files.contains_key("/here/frame.exr"));
        assert!(!local.files.contains_key(&record.staging_path.as_text()));
    }

    /// **A fresh transfer's record goes down once before its staging object
    /// is made**, not twice. `Server protocol and transfers` S8 puts it down
    /// before the first byte; S12 step 1's clearing of a list that holds
    /// nothing wrote the same record again.
    #[tokio::test]
    async fn a_fresh_transfers_record_is_not_put_down_twice_before_its_first_byte() {
        let bytes = vec![4u8; 40_000];
        let mut local = ADiskInMemory::that_answers_identities();
        let mut keeper = KeepsEveryWrite::default();
        let (happened, _) =
            one_download_against(&bytes, TheFarEnd::AnswersEverything, &mut local, &mut keeper)
                .await
                .expect("it answers");
        assert_eq!(happened.ending, Ending::Published);
        assert!(keeper.written.len() >= 2);
        assert_ne!(
            keeper.written[0], keeper.written[1],
            "the second write before the first byte changed nothing"
        );
    }

    /// **`Server protocol and transfers` S15's paragraph of 2026-09-29: a
    /// publication that fails owes nothing afterwards, and its pending mark
    /// goes in the same write as the ending.**
    ///
    /// **The case this bites on is the mark kept.** The user saved their own
    /// file at the name while it downloaded, so the create-intended rename was
    /// refused (`Desktop app and delivery` C12) and the transfer ended
    /// *failed* — with the mark still on the record, where `Download`'s
    /// admission read it and refused every download of that server, for good.
    #[tokio::test]
    async fn a_failed_publication_owes_nothing_and_leaves_the_users_file_as_it_stands() {
        let bytes = vec![2u8; 40_000];
        let mut local = ADiskInMemory::that_answers_identities();
        local
            .files
            .insert("/here/frame.exr".into(), b"the user's own".to_vec());
        let mut keeper = KeepsEveryWrite::default();
        let (happened, record) =
            one_download_against(&bytes, TheFarEnd::AnswersEverything, &mut local, &mut keeper)
                .await
                .expect("it answers");
        assert_eq!(happened.ending, Ending::Failed);
        assert_eq!(
            record.publication_pending, None,
            "a failed publication kept its pending mark"
        );
        // **In the same write as the ending**: no write carries the ending with
        // the mark still on it.
        assert!(keeper
            .written
            .iter()
            .filter(|written| written.ending.is_some())
            .all(|written| written.publication_pending.is_none()));
        assert_eq!(
            local.files.get("/here/frame.exr").map(Vec::as_slice),
            Some(&b"the user's own"[..])
        );
    }

    /// **The other limb: where Smoozy's own reservation may still stand at
    /// the final name, the mark outlives the ending**, so the next start's
    /// `Desktop app and delivery` C13 can find and remove it. Cleared here,
    /// the reservation would be named by no record, and a zero-byte file of
    /// Smoozy's would be presented as one the user put there.
    #[tokio::test]
    async fn a_reservation_left_standing_keeps_the_mark_for_the_next_start() {
        let bytes = vec![3u8; 40_000];
        let mut local = ADiskInMemory::that_answers_identities();
        local.leaves_its_reservation_standing = true;
        let mut keeper = KeepsEveryWrite::default();
        let (happened, record) =
            one_download_against(&bytes, TheFarEnd::AnswersEverything, &mut local, &mut keeper)
                .await
                .expect("it answers");
        assert_eq!(happened.ending, Ending::Failed);
        assert_eq!(local.files.get("/here/frame.exr").map(Vec::len), Some(0));
        assert!(record.publication_pending.is_some());
    }

    /// **A far end that holds its reads for a moment before answering them**,
    /// and counts the most it held at once — R6a's read path, 2026-09-30.
    ///
    /// It answers a read only once no further packet has come for
    /// `A_MOMENT`, so a loop that waits for each answer before sending the
    /// next is seen holding one read at a time, and a loop that keeps several
    /// in flight is seen holding all of them. **Every answer carries at most
    /// `answers_at_most` bytes**, so a short answer can be served. Everything
    /// else is [`a_far_end_serving`]'s.
    fn a_far_end_that_holds_its_reads(
        bytes: Vec<u8>,
        answers_at_most: usize,
    ) -> (
        tokio::io::DuplexStream,
        std::sync::Arc<std::sync::atomic::AtomicUsize>,
    ) {
        use std::sync::atomic::Ordering;
        const A_MOMENT: Duration = Duration::from_millis(30);
        let the_most_held = std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0));
        let counted = std::sync::Arc::clone(&the_most_held);
        let (ours, mut theirs) = tokio::io::duplex(1 << 20);
        tokio::spawn(async move {
            let mut held: Vec<(u32, usize, usize)> = Vec::new();
            loop {
                let mut length = [0u8; 4];
                let arrived = if held.is_empty() {
                    theirs.read_exact(&mut length).await.is_ok()
                } else {
                    match tokio::time::timeout(A_MOMENT, theirs.read_exact(&mut length)).await {
                        Ok(read) => read.is_ok(),
                        Err(_) => {
                            // **Nothing more came**: every held read is
                            // answered, oldest first.
                            counted.fetch_max(held.len(), Ordering::SeqCst);
                            for (id, offset, wanted) in held.drain(..) {
                                let reply = if offset >= bytes.len() {
                                    a_status(id, StatusCode::EndOfFile)
                                } else {
                                    let end =
                                        (offset + wanted.min(answers_at_most)).min(bytes.len());
                                    let mut composed = Composed::new();
                                    composed.u32(id).string(&bytes[offset..end]);
                                    composed.framed(kind::DATA)
                                };
                                if theirs.write_all(&reply).await.is_err() {
                                    return;
                                }
                            }
                            let _ = theirs.flush().await;
                            continue;
                        }
                    }
                };
                if !arrived {
                    return;
                }
                let mut whole = vec![0u8; u32::from_be_bytes(length) as usize];
                if theirs.read_exact(&mut whole).await.is_err() {
                    return;
                }
                let (packet_kind, payload) = (whole[0], &whole[1..]);
                let mut fields = Fields::over(payload);
                let reply = match packet_kind {
                    kind::INIT => {
                        let mut composed = Composed::new();
                        composed.u32(3);
                        composed.framed(kind::VERSION)
                    }
                    kind::READ => {
                        let id = fields.u32().unwrap_or(0);
                        let _handle = fields.string();
                        let offset = fields.u64().unwrap_or(0) as usize;
                        let wanted = fields.u32().unwrap_or(0) as usize;
                        held.push((id, offset, wanted));
                        continue;
                    }
                    kind::OPEN => {
                        let id = fields.u32().unwrap_or(0);
                        let mut composed = Composed::new();
                        composed.u32(id).string(b"h1");
                        composed.framed(kind::HANDLE)
                    }
                    kind::CLOSE => a_status(fields.u32().unwrap_or(0), StatusCode::Ok),
                    _ => a_status(fields.u32().unwrap_or(0), StatusCode::Failure),
                };
                if theirs.write_all(&reply).await.is_err() || theirs.flush().await.is_err() {
                    return;
                }
            }
        });
        (ours, the_most_held)
    }

    /// One download of `bytes` against a far end that holds its reads.
    async fn one_download_held(
        bytes: &[u8],
        answers_at_most: usize,
    ) -> (
        WhatHappened,
        TransferRecord,
        ADiskInMemory,
        KeepsEveryWrite,
        usize,
    ) {
        let (far_end, the_most_held) =
            a_far_end_that_holds_its_reads(bytes.to_vec(), answers_at_most);
        let mut subsystem = crate::subsystem::client::Subsystem::negotiate(far_end)
            .await
            .expect("the far end announces revision 3");
        let mut record = a_transfer_of(bytes);
        let mut local = ADiskInMemory::that_answers_identities();
        let mut keeper = KeepsEveryWrite::default();
        let mut budget = NoCapIsInForce;
        let mut watcher = NobodyIsWatching;
        let happened = tokio::time::timeout(
            Duration::from_secs(30),
            carry_out(
                Transfer {
                    subsystem: &mut subsystem,
                    local: &mut local,
                    budget: &mut budget,
                    keeper: &mut keeper,
                    watcher: &mut watcher,
                    inactivity_deadline: Duration::from_secs(5),
                    intent: Intent::CreateIntended,
                },
                &mut record,
            ),
        )
        .await
        .expect("the download ends inside its bound");
        let held = the_most_held.load(std::sync::atomic::Ordering::SeqCst);
        (happened, record, local, keeper, held)
    }

    /// **R6a's read path: a download keeps several reads in flight, bounded at
    /// [`THE_MOST_READS_IN_FLIGHT`]** — step 10c's eleventh-pass fix piece 2.
    ///
    /// **The case this bites on is the loop that waited a whole round trip
    /// for each read's answer before sending the next**, which moved one
    /// chunk per round trip whatever the line carried: 48.5 s for 50 MB at
    /// 0.2 s, against 7.9 s up. The far end here holds every read until no
    /// more come, so that loop is seen holding one.
    ///
    /// **And what the pipelining costs the count (E0-1)**: every checkpoint's
    /// digest is the digest of the file's own prefix up to that offset, so no
    /// checkpoint covers a byte that was not written, in order, from the start.
    #[tokio::test]
    async fn a_download_keeps_several_reads_in_flight_and_counts_only_what_it_wrote() {
        let chunk = crate::subsystem::Limits::RECOMMENDED_WHERE_NONE_ARE_ANNOUNCED.read_chunk();
        let size = chunk * (THE_MOST_READS_IN_FLIGHT + 4) + 1_000;
        let bytes: Vec<u8> = (0..size).map(|at| (at % 251) as u8).collect();
        let (happened, record, local, keeper, held) = one_download_held(&bytes, usize::MAX).await;
        assert_eq!(happened.ending, Ending::Published);
        assert_eq!(local.files.get("/here/frame.exr"), Some(&bytes));
        assert_eq!(
            held, THE_MOST_READS_IN_FLIGHT,
            "the far end held at most {held} reads at once"
        );
        let checkpoints: Vec<&Checkpoint> = keeper
            .written
            .iter()
            .flat_map(|written| written.checkpoints.iter())
            .collect();
        assert!(!checkpoints.is_empty());
        for checkpoint in checkpoints {
            assert_eq!(
                checkpoint.digest_of_everything_up_to_it,
                Digest::of(&bytes[..checkpoint.offset as usize]),
                "a checkpoint at {} covers bytes that were not written in order",
                checkpoint.offset
            );
        }
        assert_eq!(
            record.checkpoints.last().map(|at| at.offset),
            Some(size as u64)
        );
    }

    /// **A short answer is not a gap** (E0-1): where the far end answers each
    /// read with fewer bytes than asked, the reads after it are set aside and
    /// the reads start again where the written bytes end, so the file lands
    /// whole and every checkpoint is still a prefix of it.
    #[tokio::test]
    async fn a_short_answer_sets_the_later_reads_aside_and_the_file_lands_whole() {
        let size = 150_000usize;
        let bytes: Vec<u8> = (0..size).map(|at| (at % 241) as u8).collect();
        let (happened, _record, local, keeper, held) = one_download_held(&bytes, 10_000).await;
        assert_eq!(happened.ending, Ending::Published);
        assert_eq!(local.files.get("/here/frame.exr"), Some(&bytes));
        assert!(held > 1, "the reads went one at a time");
        for checkpoint in keeper
            .written
            .iter()
            .flat_map(|written| written.checkpoints.iter())
        {
            assert_eq!(
                checkpoint.digest_of_everything_up_to_it,
                Digest::of(&bytes[..checkpoint.offset as usize])
            );
        }
    }

    /// **S12 step 3's read-back keeps several reads in flight too** (R6a's
    /// read path, 2026-09-30). **The case this bites on**: an upload cut on the
    /// eleventh pass verified its 257.5 MB staged prefix at about 1.7 MB/s on
    /// a line carrying 11, one read per round trip. The digest is still the
    /// prefix's own, in order.
    #[tokio::test]
    async fn an_uploads_staged_prefix_is_read_back_with_several_reads_in_flight() {
        let chunk = crate::subsystem::Limits::RECOMMENDED_WHERE_NONE_ARE_ANNOUNCED.read_chunk();
        let size = chunk * 6 + 77;
        let bytes: Vec<u8> = (0..size).map(|at| (at % 239) as u8).collect();
        let (far_end, the_most_held) = a_far_end_that_holds_its_reads(bytes.clone(), usize::MAX);
        let mut subsystem = crate::subsystem::client::Subsystem::negotiate(far_end)
            .await
            .expect("the far end announces revision 3");
        let issue = |kind| Identity::issue(kind).expect("random bytes");
        let record = TransferRecord::arranged(
            issue(IdentityKind::Transfer),
            Direction::Up,
            issue(IdentityKind::LocalServer),
            Some(issue(IdentityKind::Installation)),
            "/here/scene.blend".to_string(),
            "/machine/scene.blend".to_string(),
            "/machine/.smoozy-staging",
            size as u64 * 2,
            issue(IdentityKind::Run),
        );
        let mut local = ADiskInMemory::default();
        let mut keeper = WritesItDown::default();
        let mut budget = NoCapIsInForce;
        let mut watcher = NobodyIsWatching;
        let mut transfer = Transfer {
            subsystem: &mut subsystem,
            local: &mut local,
            budget: &mut budget,
            keeper: &mut keeper,
            watcher: &mut watcher,
            inactivity_deadline: Duration::from_secs(5),
            intent: Intent::CreateIntended,
        };
        let running = tokio::time::timeout(
            Duration::from_secs(30),
            read_the_prefix(&mut transfer, &record, size as u64),
        )
        .await
        .expect("the read-back ends inside its bound")
        .expect("the whole prefix came back");
        assert_eq!(running.so_far(), Digest::of(&bytes));
        let held = the_most_held.load(std::sync::atomic::Ordering::SeqCst);
        assert_eq!(held, 7, "the far end held at most {held} reads at once");
    }

    // ------------------- a machine that goes silent (twelfth-pass fix piece 1)

    /// **A far end that takes `takes` bytes of writes, answering each, and
    /// then stops reading and stops answering with its end held open** — a
    /// line gone silent, as the provider's firewall made it on step 10c's
    /// twelfth pass. What is sent after that waits for room in the pipe, as a
    /// write waits for the machine to open its window. It answers the opening
    /// and every other packet before that moment as [`a_far_end_serving`]
    /// does, and **every read it never answers**, so a prefix read back from
    /// it waits too.
    fn a_far_end_that_goes_silent(takes: usize) -> tokio::io::DuplexStream {
        let (ours, mut theirs) = tokio::io::duplex(8 * 1024);
        tokio::spawn(async move {
            let mut taken = 0usize;
            loop {
                let mut length = [0u8; 4];
                if theirs.read_exact(&mut length).await.is_err() {
                    return;
                }
                let mut whole = vec![0u8; u32::from_be_bytes(length) as usize];
                if theirs.read_exact(&mut whole).await.is_err() {
                    return;
                }
                let (packet_kind, payload) = (whole[0], &whole[1..]);
                let mut fields = Fields::over(payload);
                let reply = match packet_kind {
                    kind::INIT => {
                        let mut composed = Composed::new();
                        composed.u32(3);
                        composed.framed(kind::VERSION)
                    }
                    kind::OPEN => {
                        let id = fields.u32().unwrap_or(0);
                        let mut composed = Composed::new();
                        composed.u32(id).string(b"h1");
                        composed.framed(kind::HANDLE)
                    }
                    // **Silent from its first read**: never answered.
                    kind::READ => continue,
                    kind::WRITE => {
                        taken += whole.len();
                        a_status(fields.u32().unwrap_or(0), StatusCode::Ok)
                    }
                    _ => a_status(fields.u32().unwrap_or(0), StatusCode::Failure),
                };
                if theirs.write_all(&reply).await.is_err() || theirs.flush().await.is_err() {
                    return;
                }
                if taken >= takes {
                    // **The silence**: the end is kept open, and nothing more
                    // is read from it or said on it.
                    std::future::pending::<()>().await;
                }
            }
        });
        ours
    }

    /// A watcher that keeps every report.
    #[derive(Default)]
    struct KeepsEveryReport {
        reports: Vec<(Instant, Report)>,
    }
    impl WatchesTheTransfer for KeepsEveryReport {
        fn report(&mut self, report: &Report) {
            self.reports.push((Instant::now(), report.clone()));
        }
    }

    /// **An upload whose machine goes silent under it ends *stalled* at C19's
    /// deadline, reports its unmoving count while it waits, and owes its
    /// removal rather than waiting again for it** — step 10c's twelfth-pass
    /// fix piece 1.
    ///
    /// **The case this bites on is the twelfth pass's**: a one-minute cut at
    /// the provider's firewall under a 1 GB upload. The write waited for room
    /// in the channel, and that one wait of the loop had no deadline, so the
    /// row stood at *916.8 MB* for good with *↑ 6.8 MB/s* beside it. **And the
    /// ending would not have come back either**: *stalled* removes the staged
    /// material, and the removal was one more unbounded wait on the channel
    /// that had just stopped answering.
    #[tokio::test]
    async fn an_upload_whose_machine_goes_silent_ends_stalled_and_reports_while_it_waits() {
        let size = 2_000_000usize;
        let bytes: Vec<u8> = (0..size).map(|at| (at % 233) as u8).collect();
        let mut subsystem =
            crate::subsystem::client::Subsystem::negotiate(a_far_end_that_goes_silent(200_000))
                .await
                .expect("the far end announces revision 3");
        let issue = |kind| Identity::issue(kind).expect("random bytes");
        let mut record = TransferRecord::arranged(
            issue(IdentityKind::Transfer),
            Direction::Up,
            issue(IdentityKind::LocalServer),
            Some(issue(IdentityKind::Installation)),
            "/here/scene.blend".to_string(),
            "/machine/scene.blend".to_string(),
            "/machine/.smoozy-staging",
            size as u64,
            issue(IdentityKind::Run),
        );
        let mut local = ADiskInMemory::default();
        local.files.insert("/here/scene.blend".to_string(), bytes);
        let mut keeper = KeepsEveryWrite::default();
        let mut budget = NoCapIsInForce;
        let mut watcher = KeepsEveryReport::default();
        let deadline = Duration::from_millis(600);
        let began = Instant::now();
        let happened = tokio::time::timeout(
            Duration::from_secs(10),
            carry_out(
                Transfer {
                    subsystem: &mut subsystem,
                    local: &mut local,
                    budget: &mut budget,
                    keeper: &mut keeper,
                    watcher: &mut watcher,
                    inactivity_deadline: deadline,
                    intent: Intent::CreateIntended,
                },
                &mut record,
            ),
        )
        .await
        .expect("the upload came back inside its bound rather than waiting for ever");
        let took = began.elapsed();
        assert_eq!(happened.ending, Ending::Stalled);
        assert_eq!(record.ending, Some(Ending::Stalled));
        assert!(
            took < deadline * 3,
            "the upload took {took:?} to end against a deadline of {deadline:?}"
        );
        // **S27: the removal is owed**, and not asked of the silent channel.
        assert!(record.cleanup_is_owed, "a stalled upload's removal is not owed");
        assert_eq!(keeper.written.last().map(|last| last.cleanup_is_owed), Some(true));
        // **G4**: the unmoving count went on reaching the watcher while the
        // write waited — at least once per accounting period, less the first.
        let stood_at = happened.report.bytes_staged;
        assert!(stood_at > 0 && stood_at < size as u64, "the count stood at {stood_at}");
        let while_it_waited = watcher
            .reports
            .iter()
            .filter(|(_, report)| report.bytes_staged == stood_at)
            .count();
        let periods = (deadline.as_millis() / THE_ACCOUNTING_PERIOD.as_millis()) as usize;
        assert!(
            while_it_waited >= periods - 1,
            "the count reached the watcher {while_it_waited} times while the write waited {deadline:?}"
        );
    }

    /// **A staged prefix read back from a machine that goes silent ends
    /// *stalled* at C19's deadline** — step 10c's twelfth-pass fix piece 1.
    /// **The case this bites on**: the read-back had no deadline before or
    /// after it was pipelined, so a cut during a verification held the
    /// transfer for ever.
    #[tokio::test]
    async fn a_prefix_read_back_from_a_machine_that_goes_silent_ends_stalled() {
        let mut subsystem =
            crate::subsystem::client::Subsystem::negotiate(a_far_end_that_goes_silent(usize::MAX))
                .await
                .expect("the far end announces revision 3");
        let issue = |kind| Identity::issue(kind).expect("random bytes");
        let record = TransferRecord::arranged(
            issue(IdentityKind::Transfer),
            Direction::Up,
            issue(IdentityKind::LocalServer),
            Some(issue(IdentityKind::Installation)),
            "/here/scene.blend".to_string(),
            "/machine/scene.blend".to_string(),
            "/machine/.smoozy-staging",
            4_000_000,
            issue(IdentityKind::Run),
        );
        let mut local = ADiskInMemory::default();
        let mut keeper = WritesItDown::default();
        let mut budget = NoCapIsInForce;
        let mut watcher = NobodyIsWatching;
        let mut transfer = Transfer {
            subsystem: &mut subsystem,
            local: &mut local,
            budget: &mut budget,
            keeper: &mut keeper,
            watcher: &mut watcher,
            inactivity_deadline: Duration::from_millis(300),
            intent: Intent::CreateIntended,
        };
        let came_back = tokio::time::timeout(
            Duration::from_secs(10),
            read_the_prefix(&mut transfer, &record, 1_000_000),
        )
        .await
        .expect("the read-back came back inside its bound rather than waiting for ever");
        assert_eq!(came_back.err(), Some(WhyTheLoopStopped::TheDeadlineExpired));
    }

    #[test]
    fn no_cap_in_force_draws_what_was_asked_for_and_forfeits_nothing() {
        // `Desktop app and delivery` R8: an empty field is no cap, which is
        // also the default.
        let mut budget = NoCapIsInForce;
        assert_eq!(budget.draw(261_120), 261_120);
        budget.give_back(0);
    }
}
