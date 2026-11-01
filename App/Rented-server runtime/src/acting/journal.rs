//! The event journal in the package area — `State, identity and recovery` §1.2's
//! machine-side store whose content is that document's §4.
//!
//! **`smoozy_state::journal` holds the rules.** This module is the append and
//! the read.
//!
//! # Why it is written here at all
//!
//! **The rule is one document's and the store is another's**, which is the split
//! this crate already draws for §5.1's two records next door. `State, identity
//! and recovery` §1.2's *content owned by* column gives the event journal to that
//! document, and its §1.3 rule 1 makes **the supervisor the only writer of every
//! durable store in that inventory** — and the supervisor is this crate's
//! program. **So the form is that document's and the bytes are this one's**;
//! [`crate::written_form::an_event_as_written`] is where the bytes are settled
//! and says so.
//!
//! **Nobody was ever told to build this.** `Preparation/Build/Plan.md` step 2's
//! bullet names the journal, step 4's prose moves that document's machine-side
//! stores to step 6, and step 6's own two bullets name neither store — so it fell
//! between them, and its absence is why the supervisor runs no reconciliation.
//!
//! # What a write is here, and it is not the write beside it
//!
//! **Every entry is an append**, [`durable::AFileToAppendTo`], which is
//! `State, identity and recovery` §1.3 rule 6 for a write published by no name:
//! the bytes are written and the file is flushed to the storage device. **The
//! record store beside this one publishes by a name and this one does not**, and
//! the difference is load-bearing rather than a matter of taste — this plan's own
//! Section 16 item 26 observed, on a genuinely full mount, that *"`submit` and
//! `advance_attempt` refused with errno 28 while seventy-six consecutive journal
//! appends succeeded"*, because *"an append fitting inside the slack of an
//! already-allocated last block needs no new block; a record write needs a new
//! file and a new directory entry."* A journal that published every entry by a
//! name would refuse in the same instant the record store did, and that window
//! would be gone — along with the only route by which a reader learns of an
//! ending whose record could not be written.
//!
//! **The window is a hazard rather than a comfort and this module does not
//! smooth it over.** Item 26's own *Breaks* line is *"a window in which an event
//! is journalled while the record it is about cannot be written"*, and what
//! bounds the cost is S14 holding the unwritable record pending and re-attempting
//! it — which is not built here.
//!
//! # How it is laid out, and why it is not one file
//!
//! **The journal is a run of segment files, each holding a fixed number of
//! entries, named for the first sequence number in it.** That shape is a Build
//! decision and no clause fixes it; what forces it is `State, identity and
//! recovery` E6, which drops the oldest entries once the cap is reached. **A
//! single file could not do that without rewriting itself**, and a rewrite is a
//! publication by a name — so the one store on this machine that drops what it
//! holds would have needed a new directory entry to drop anything, on a mount
//! that may have none.
//!
//! **Dropping is therefore by whole segments**, which is why the journal holds
//! between [`THE_ENTRIES_IN_ONE_SEGMENT`] fewer than the cap and exactly the cap.
//! **It never holds more**, which is the direction that matters: E6a's cap is a
//! ceiling.
//!
//! # The two hazards a kill leaves, and what is done about each
//!
//! **A part-written last line.** A process killed inside an append can leave
//! bytes on the device with no ending behind them. **Those bytes never
//! completed** — `State, identity and recovery` E1 is exact, *"an event that has
//! not reached the journal has not happened as far as any reader is
//! concerned"* — so the newest segment is cut back to its last whole line before
//! anything is appended after it. Appending after the stump would have joined two
//! half-entries into one line nobody wrote.
//!
//! **The same stump can arrive inside one run, and it did not used to be met.**
//! An append that fails may still have put some of its bytes on the device — a
//! full mount is the observed case — and until 2026-09-12 the next entry was
//! written straight after them. **So every append remembers the segment's length
//! before it and puts it back where the append fails**, through the same trim.
//! **Where even the trim fails, every further append in this run is refused**
//! rather than written after a stump, and the next start's own trim clears it.
//! This is the record that is meant to go on working in exactly the moment the
//! other store fails, and one ruined segment costs a reader the whole of that
//! machine's journal rather than one entry.
//!
//! **A segment that is shorter than it should be.** Sequence numbers are
//! contiguous per machine (E5), and this store derives an entry's sequence from
//! its segment's own first number and its position in that segment. **A segment
//! that is not the newest and holds fewer entries than it should would make every
//! later number wrong**, so it is refused loudly at the read rather than papered
//! over — a reader that was quietly handed the wrong numbers could not detect the
//! gap E5 gives it, because the numbers would still be contiguous.
//!
//! **Markers used in this file that belong to another document, each with the
//! document it belongs to.** `State, identity and recovery` — E1, E2, E3, E4,
//! E5, E6, E6a, E7, D5, J11.
//!
//! **Two of that document's are deliberately not listed there** — **G1** and
//! **R1** — because this document defines a G-series and an R-series of its own,
//! so a header list could not tell them apart. Each is written out with its
//! document at the site.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use smoozy_state::identities::{Identity, IdentityKind};
use smoozy_state::journal::{
    what_retention_keeps, AnEvent, ARegisteredReader, SequenceNumber, WhatAReadOffers,
    WhatHappened, THE_ENTRY_CAP, THE_PER_ENTRY_BUDGET,
};

use crate::acting::durable::{self, AFileToAppendTo};
use crate::written_form::{self, an_event_as_written::AnEventAsWritten};

/// **Where the journal sits under the package area**, and the name is a Build
/// decision.
///
/// **No clause fixes it.** `State, identity and recovery` §1.2 puts the journal
/// in the package area and that document's J11 puts every store there **outside
/// every job folder**; neither that document nor this one names a segment.
pub const THE_AREA_THE_JOURNAL_SITS_IN: &str = "event-journal";

/// What a segment file's name ends with, so a reader of the area can tell a
/// segment from the registration record beside it without opening either.
///
/// **A publication in flight is `<name>.writing`** — [`durable::the_staging_name_for`]'s
/// own suffix — so this never matches one.
const A_SEGMENTS_SUFFIX: &str = ".segment";

/// The registration record's own name. **E2's *registration that keeps the
/// retention floor honest*, and the whole of the per-reader state the journal
/// holds.**
const THE_REGISTRATIONS_RECORD: &str = "registrations";

/// **How many entries one segment holds, and it is a Build decision no clause
/// fixes.**
///
/// **What it costs and what it buys.** Dropping is by whole segments, so the
/// journal keeps between the cap less this number and the cap; a smaller number
/// keeps more entries and makes more files, a larger one the reverse. At 1,024
/// the journal holds at least 31,745 of E6a's 32,768 and the area holds at most
/// 33 files.
pub const THE_ENTRIES_IN_ONE_SEGMENT: u64 = 1_024;

/// What went wrong with the journal, in the machine's own words or the store's.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TheJournalRefused {
    /// The machine refused a write or a read, verbatim, with the path.
    TheStore(String),
    /// **E6a: an entry longer than the per-entry budget is refused rather than
    /// trimmed.** *"An entry's event identity, sequence number, moment and kind
    /// are never abbreviated to fit; a kind whose fields would exceed the budget
    /// names the record that holds the detail rather than copying it in."* So
    /// the caller that composed it is the one that has to name a record instead,
    /// and this says how long it was.
    TheEntryIsLongerThanTheBudget { it_would_take: u64 },
    /// The machine would not supply random bytes for the event identity, so no
    /// event was journalled. **Nothing is composed from anything else instead.**
    NoRandomness,
}

impl TheJournalRefused {
    /// The shape every refusal in this crate carries.
    pub fn as_text(&self) -> String {
        match self {
            TheJournalRefused::TheStore(said) => said.clone(),
            TheJournalRefused::TheEntryIsLongerThanTheBudget { it_would_take } => format!(
                "the entry would take {it_would_take} bytes against a per-entry budget of \
                 {THE_PER_ENTRY_BUDGET}: a kind whose fields would exceed the budget names the \
                 record that holds the detail rather than copying it in"
            ),
            TheJournalRefused::NoRandomness => {
                "the machine would not supply random bytes for an event identity".to_string()
            }
        }
    }
}

/// **The words a journal that refuses every further append in this run gives.**
///
/// **It names the original failure and says what it is protecting**, because a
/// caller meeting a bare refusal at an unrelated moment would go looking for a
/// fault in its own write.
fn the_journal_is_shut_for_this_run(first: &str) -> TheJournalRefused {
    TheJournalRefused::TheStore(format!(
        "no further event is appended in this run: an append failed part-way and the bytes it \
         left could not be taken off, so anything written after them would join two half-entries \
         into one line nobody wrote and no reader could read this machine's journal at all. The \
         next supervisor start cuts the segment back to its last whole line. What failed: {first}"
    ))
}

fn the_store_said(at: &Path, why: impl std::fmt::Display) -> TheJournalRefused {
    TheJournalRefused::TheStore(format!("{}: {why}", at.display()))
}

/// The event journal, in the package area.
#[derive(Debug)]
pub struct TheEventJournalOnTheMachine {
    area: PathBuf,
    /// The first sequence number of each segment, oldest first.
    segments: Vec<SequenceNumber>,
    /// One past the newest sequence the journal holds — the next one to assign.
    next_sequence: SequenceNumber,
    /// The newest segment, held open. **Nothing until the first append**, on a
    /// machine that has never journalled anything.
    appending: Option<AFileToAppendTo>,
    /// **E2's registrations**, and nothing else per reader.
    registrations: BTreeMap<ARegisteredReader, SequenceNumber>,
    /// How many entries one segment holds. **[`THE_ENTRIES_IN_ONE_SEGMENT`] on
    /// a machine**, and a smaller figure only where a test poses the dropping.
    entries_in_one_segment: u64,
    /// **E6a's cap.** [`THE_ENTRY_CAP`] on a machine, and the same.
    cap: u64,
    /// **Set where an append failed part-way and the stump could not be taken
    /// off**, and never unset in this run.
    ///
    /// **What it stops is the second half of the hazard.** A part-written entry
    /// with bytes after it is one line nobody wrote, and the reader that meets
    /// it can read no further entry of that machine's journal at all — so where
    /// the trim itself fails, every later append in this run is refused rather
    /// than written after the stump. **The next start's own trim clears it**,
    /// which is [`Self::open_the_newest_segment`] and is a removal of something
    /// that never happened.
    no_further_append_in_this_run: Option<String>,
    /// **A part-written append, posed**, with whether the trim after it is
    /// posed as failing too — see
    /// [`Self::pose_an_append_that_lands_part_of_it_and_then_fails`].
    #[cfg(test)]
    a_posed_part_written_append: Option<(usize, bool)>,
}

impl TheEventJournalOnTheMachine {
    /// **Open it in an area, made with the permission the package area's own
    /// stores carry.**
    ///
    /// **Where a previous run was killed part-way through an append, the newest
    /// segment is cut back to its last whole line here** — see the head of this
    /// file for why that is a removal of something that never happened rather
    /// than a repair of state.
    pub fn in_the_area(area: &Path) -> Result<Self, TheJournalRefused> {
        Self::in_the_area_holding(area, THE_ENTRIES_IN_ONE_SEGMENT, THE_ENTRY_CAP)
    }

    /// The same journal with the two figures handed in.
    ///
    /// **Every rule below is the same rule and only the figures move**, which is
    /// what lets the dropping of E6 be exercised at the desk without writing
    /// 32,768 real appends to prove a rule `what_retention_keeps` already states
    /// and tests. **Nothing outside this crate's own tests calls it with
    /// anything but E6a's own two figures**, and [`Self::in_the_area`] is the
    /// only route a supervisor has.
    pub(crate) fn in_the_area_holding(
        area: &Path,
        entries_in_one_segment: u64,
        cap: u64,
    ) -> Result<Self, TheJournalRefused> {
        durable::make_the_area(area).map_err(|why| TheJournalRefused::TheStore(why.as_text()))?;
        let segments = the_segments_in(area, entries_in_one_segment)?;
        let mut journal = TheEventJournalOnTheMachine {
            area: area.to_path_buf(),
            segments,
            next_sequence: 0,
            appending: None,
            registrations: BTreeMap::new(),
            entries_in_one_segment,
            cap,
            no_further_append_in_this_run: None,
            #[cfg(test)]
            a_posed_part_written_append: None,
        };
        journal.registrations = journal.the_registrations_as_written()?;
        journal.open_the_newest_segment()?;
        Ok(journal)
    }

    fn where_a_segment_sits(&self, first: SequenceNumber) -> PathBuf {
        self.area.join(the_name_of_the_segment_starting_at(first))
    }

    /// Open the newest segment, cut off anything that never completed, and take
    /// the next sequence number from what is left.
    fn open_the_newest_segment(&mut self) -> Result<(), TheJournalRefused> {
        let Some(newest) = self.segments.last().copied() else {
            self.next_sequence = 0;
            self.appending = None;
            return Ok(());
        };
        let at = self.where_a_segment_sits(newest);
        let bytes = durable::read(&at)
            .map_err(|why| TheJournalRefused::TheStore(why.as_text()))?
            .unwrap_or_default();
        let whole = the_length_of_the_whole_lines_in(&bytes);
        let mut file = AFileToAppendTo::opened(&at)
            .map_err(|why| TheJournalRefused::TheStore(why.as_text()))?;
        if whole != bytes.len() as u64 {
            file.cut_back_to(whole)
                .map_err(|why| TheJournalRefused::TheStore(why.as_text()))?;
        }
        self.next_sequence = newest + how_many_lines_in(&bytes[..whole as usize]);
        self.appending = Some(file);
        Ok(())
    }

    /// **E1: journal one event durably, with an event identity and the next
    /// per-machine sequence number, before it is offered to any reader.**
    ///
    /// The identity is issued here because §2.2's register makes **the
    /// supervisor** its issuer, *"when the event is journalled, before it is
    /// offered to any reader"*, and this store is the supervisor's.
    ///
    /// **The sequence is assigned here and never by a caller**, which is what
    /// makes the numbers contiguous per machine and therefore what makes E5's
    /// gap detection need no extra record.
    pub fn journal(&mut self, what: WhatHappened) -> Result<AnEvent, TheJournalRefused> {
        let identity =
            Identity::issue(IdentityKind::Event).map_err(|_| TheJournalRefused::NoRandomness)?;
        let event = AnEvent::journalled(identity, self.next_sequence, what)
            .expect("an identity issued as an event's is an event's");
        let bytes = written_form::write(&AnEventAsWritten::of(&event)).map_err(|why| {
            TheJournalRefused::TheStore(why.what_the_reader_said)
        })?;
        // **E6a, before anything is written.** An entry that does not fit is
        // refused whole rather than trimmed, so nothing lands part-said.
        if bytes.len() as u64 > THE_PER_ENTRY_BUDGET {
            return Err(TheJournalRefused::TheEntryIsLongerThanTheBudget {
                it_would_take: bytes.len() as u64,
            });
        }
        if let Some(first) = &self.no_further_append_in_this_run {
            return Err(the_journal_is_shut_for_this_run(first));
        }
        #[cfg(test)]
        let posed = self.a_posed_part_written_append.take();
        #[cfg(not(test))]
        let posed: Option<(usize, bool)> = None;
        self.roll_if_the_newest_segment_is_full()?;
        let file = self
            .appending
            .as_mut()
            .expect("a segment was opened or made above");
        // **What the segment was before this append**, so a part-written entry
        // can be taken off again. **An append that fails may still have put
        // bytes on the device** — a full mount is the observed case — and the
        // next entry written straight after them makes the two one line nobody
        // wrote, after which no reader can read this machine's journal at all.
        // `State, identity and recovery` E1 is what makes the removal right
        // rather than a repair: an entry that never completed never happened.
        let whole_before = file
            .how_long_it_is()
            .map_err(|why| TheJournalRefused::TheStore(why.as_text()))?;
        let the_trim_is_posed_as_failing = posed.is_some_and(|(_, it_fails)| it_fails);
        let attempted = match posed {
            None => file.append(&bytes),
            Some((how_many, _)) => {
                let landed = &bytes[..how_many.min(bytes.len())];
                let _ = file.append(landed);
                Err(crate::acting::durable::TheWriteWasRefused {
                    at: file.at().display().to_string(),
                    what_the_machine_said: "No space left on device".to_string(),
                })
            }
        };
        if let Err(why) = attempted {
            let refused = TheJournalRefused::TheStore(why.as_text());
            let trimmed = if the_trim_is_posed_as_failing {
                Err(crate::acting::durable::TheWriteWasRefused {
                    at: file.at().display().to_string(),
                    what_the_machine_said: "No space left on device".to_string(),
                })
            } else {
                file.cut_back_to(whole_before)
            };
            if let Err(and_the_trim) = trimmed {
                // **Neither the append nor its removal landed**, so what is on
                // the device may be a stump. Every further append in this run
                // is refused rather than written after it.
                self.no_further_append_in_this_run =
                    Some(format!("{}; and the trim after it: {}", why.as_text(), and_the_trim.as_text()));
            }
            return Err(refused);
        }
        self.next_sequence += 1;
        self.drop_what_the_cap_no_longer_lets_it_keep()?;
        Ok(event)
    }

    /// **Pose the one failure this desk cannot produce: an append that lands
    /// some of its bytes and then comes back refused.**
    ///
    /// **What it poses and what it does not.** It poses the refusal at the one
    /// call a real one comes back from, with the bytes that did land really on
    /// the device — everything after it is the ordinary code running: the length
    /// remembered before the append, the trim that takes the stump off, the
    /// shutting of the journal for the run where even that fails, and what a
    /// reader then sees. **Only the source of the refusal is posed.**
    ///
    /// **Why it is posed rather than caused.** The real cause is a mount with no
    /// room, observed on a rented container; the desk is macOS, which publishes
    /// no always-full device, and the ways of making a real full mount here
    /// either change the Creator's machine or leave something mounted behind.
    /// It is the same shape as [`Self::in_the_area_holding`], which moves E6a's
    /// two figures so the dropping can be exercised without 32,768 real appends:
    /// every rule below is the same rule.
    #[cfg(test)]
    pub(crate) fn pose_an_append_that_lands_part_of_it_and_then_fails(&mut self, bytes: usize) {
        self.a_posed_part_written_append = Some((bytes, false));
    }

    /// **The same, and the trim after it fails too** — the second half of the
    /// hazard, where the stump cannot be taken off at all.
    ///
    /// **The trim is two acts and both can be refused on a full mount**: the
    /// shortening, and the flush after it that is what makes the write durable
    /// at all (`State, identity and recovery` §1.3 rule 6). The trim itself is
    /// proved really performed by the test above; what is posed here is its
    /// refusal, so that what the journal does next can be watched.
    #[cfg(test)]
    pub(crate) fn pose_an_append_and_a_trim_that_both_fail(&mut self, bytes: usize) {
        self.a_posed_part_written_append = Some((bytes, true));
    }

    /// Start a segment where there is none, or where the newest is full.
    fn roll_if_the_newest_segment_is_full(&mut self) -> Result<(), TheJournalRefused> {
        let full = match self.segments.last() {
            None => true,
            Some(newest) => self.next_sequence - newest >= self.entries_in_one_segment,
        };
        if !full {
            return Ok(());
        }
        let at = self.where_a_segment_sits(self.next_sequence);
        let file = AFileToAppendTo::opened(&at)
            .map_err(|why| TheJournalRefused::TheStore(why.as_text()))?;
        self.segments.push(self.next_sequence);
        self.appending = Some(file);
        Ok(())
    }

    /// **E6, applied: drop whole segments from the oldest end while the journal
    /// holds more than the cap.**
    ///
    /// **The newest segment is never dropped**, because it is the one being
    /// appended to; the cap is far larger than one segment, so that is not a
    /// case the rule can reach.
    fn drop_what_the_cap_no_longer_lets_it_keep(&mut self) -> Result<(), TheJournalRefused> {
        while self.segments.len() > 1 {
            let oldest = self.segments[0];
            let kept = what_retention_keeps(
                oldest,
                self.next_sequence,
                &self.every_registered_cursor(),
                self.cap,
            );
            if kept.the_first_sequence_kept <= oldest {
                break;
            }
            let at = self.where_a_segment_sits(oldest);
            durable::remove_durably(&at)
                .map_err(|why| TheJournalRefused::TheStore(why.as_text()))?;
            self.segments.remove(0);
        }
        Ok(())
    }

    fn every_registered_cursor(&self) -> Vec<SequenceNumber> {
        self.registrations.values().copied().collect()
    }

    /// **E5's number: the oldest sequence the journal still holds**, or nothing
    /// where it holds no entry at all.
    pub fn oldest_retained(&self) -> Option<SequenceNumber> {
        let first = *self.segments.first()?;
        if self.next_sequence == first {
            return None;
        }
        Some(first)
    }

    /// One past the newest sequence the journal holds.
    pub fn next_sequence(&self) -> SequenceNumber {
        self.next_sequence
    }

    /// **E3 and E5: read forward from a reader's own cursor, and report the
    /// oldest retained sequence number with it.**
    ///
    /// **Nothing here advances a cursor.** A read is a read: a reader that dies
    /// between this and its own durable acknowledgement reads the same events
    /// again, which is E4's at-least-once delivery and is why every reader
    /// applies an event idempotently by its event identity.
    ///
    /// **`at_most` bounds one read rather than the journal.** It is the caller's
    /// own figure and nothing is sized on it; a reader that asks for fewer than
    /// there are comes back for the rest.
    pub fn read_forward_from(
        &self,
        cursor: SequenceNumber,
        at_most: usize,
    ) -> Result<WhatAReadOffers, TheJournalRefused> {
        let mut events = Vec::new();
        for (at, first) in self.segments.iter().copied().enumerate() {
            let is_the_newest = at + 1 == self.segments.len();
            let one_past = if is_the_newest {
                self.next_sequence
            } else {
                first + self.entries_in_one_segment
            };
            if events.len() >= at_most || one_past <= cursor {
                continue;
            }
            let read = self.read_one_segment(first, is_the_newest)?;
            for event in read {
                if event.sequence() < cursor || events.len() >= at_most {
                    continue;
                }
                events.push(event);
            }
        }
        Ok(WhatAReadOffers {
            events,
            oldest_retained: self.oldest_retained(),
            next_sequence: self.next_sequence,
            from_the_cursor: cursor,
        })
    }

    /// One segment's entries, in order.
    ///
    /// **Two things are checked here rather than trusted.** A segment that is
    /// not the newest and does not hold exactly [`THE_ENTRIES_IN_ONE_SEGMENT`]
    /// entries would make every later sequence number wrong, and an entry whose
    /// written sequence disagrees with its position would mean the same. **Both
    /// are refused loudly**, because a reader handed the wrong numbers could not
    /// detect the gap E5 gives it — the numbers would still be contiguous.
    fn read_one_segment(
        &self,
        first: SequenceNumber,
        is_the_newest: bool,
    ) -> Result<Vec<AnEvent>, TheJournalRefused> {
        let at = self.where_a_segment_sits(first);
        let bytes = durable::read(&at)
            .map_err(|why| TheJournalRefused::TheStore(why.as_text()))?
            .ok_or_else(|| {
                the_store_said(
                    &at,
                    "a segment was listed and was not there when it was read",
                )
            })?;
        let whole = the_length_of_the_whole_lines_in(&bytes) as usize;
        let mut events = Vec::new();
        for (at_position, line) in bytes[..whole].split(|byte| *byte == b'\n').enumerate() {
            if line.is_empty() {
                continue;
            }
            let written: AnEventAsWritten =
                written_form::read(line).map_err(|why| the_store_said(&at, why.what_the_reader_said))?;
            let event = written
                .back()
                .map_err(|why| the_store_said(&at, why.what_the_reader_said))?;
            let where_it_sits = first + at_position as u64;
            if event.sequence() != where_it_sits {
                return Err(the_store_said(
                    &at,
                    format!(
                        "the entry at position {at_position} says it is sequence {} and the \
                         segment puts it at {where_it_sits}",
                        written.the_sequence_it_claims()
                    ),
                ));
            }
            events.push(event);
        }
        if !is_the_newest && events.len() as u64 != self.entries_in_one_segment {
            return Err(the_store_said(
                &at,
                format!(
                    "a segment that is not the newest holds {} entries where every sequence \
                     after it is derived from its holding {}",
                    events.len(),
                    self.entries_in_one_segment
                ),
            ));
        }
        Ok(events)
    }

    /// **E2: register a reader, so the retention floor is honest about it.**
    ///
    /// A reader that has never registered starts at the beginning of what the
    /// journal still holds, which is where a fresh installation's desktop starts
    /// under `State, identity and recovery` Y13.
    pub fn register(&mut self, reader: ARegisteredReader) -> Result<(), TheJournalRefused> {
        if self.registrations.contains_key(&reader) {
            return Ok(());
        }
        let from = self.oldest_retained().unwrap_or(self.next_sequence);
        self.registrations.insert(reader, from);
        self.write_the_registrations_back()
    }

    /// **E3: a reader has done the work every event below `cursor` asked for.**
    ///
    /// **A cursor never goes backwards here**, because a reader that reported a
    /// lower one would lower the retention floor for entries the journal may
    /// already have dropped, and E6's floor is about what is still to be done.
    ///
    /// **A reader never advances another reader's cursor** (E2), which is kept by
    /// this taking exactly one reader and writing exactly its own row.
    pub fn a_reader_has_passed(
        &mut self,
        reader: ARegisteredReader,
        cursor: SequenceNumber,
    ) -> Result<(), TheJournalRefused> {
        let standing = self.registrations.entry(reader).or_insert(0);
        if cursor <= *standing {
            return Ok(());
        }
        *standing = cursor;
        self.write_the_registrations_back()
    }

    /// What each registered reader has passed. **This is the whole of the
    /// per-reader state the journal holds** (E2).
    pub fn every_registration(&self) -> &BTreeMap<ARegisteredReader, SequenceNumber> {
        &self.registrations
    }

    fn where_the_registrations_sit(&self) -> PathBuf {
        self.area.join(THE_REGISTRATIONS_RECORD)
    }

    /// **The registrations are a record and are published by a name**, unlike an
    /// entry. They are written when a reader registers or reports, which is not
    /// on the append path — so nothing about item 26's window turns on them.
    fn write_the_registrations_back(&self) -> Result<(), TheJournalRefused> {
        let written: Vec<(String, SequenceNumber)> = self
            .registrations
            .iter()
            .map(|(reader, cursor)| (reader.as_text().to_string(), *cursor))
            .collect();
        let bytes = written_form::write(&written)
            .map_err(|why| TheJournalRefused::TheStore(why.what_the_reader_said))?;
        durable::publish(&self.where_the_registrations_sit(), &bytes)
            .map_err(|why| TheJournalRefused::TheStore(why.as_text()))
    }

    fn the_registrations_as_written(
        &self,
    ) -> Result<BTreeMap<ARegisteredReader, SequenceNumber>, TheJournalRefused> {
        let at = self.where_the_registrations_sit();
        let Some(bytes) =
            durable::read(&at).map_err(|why| TheJournalRefused::TheStore(why.as_text()))?
        else {
            return Ok(BTreeMap::new());
        };
        let written: Vec<(String, SequenceNumber)> = written_form::read(&bytes)
            .map_err(|why| the_store_said(&at, why.what_the_reader_said))?;
        let mut out = BTreeMap::new();
        for (named, cursor) in written {
            // **A word that names no reader §4 has is a refusal and never a
            // third reader made up to fill the field**, because a registration
            // nothing advances raises the retention floor for ever.
            let reader = ARegisteredReader::read_back(&named).ok_or_else(|| {
                the_store_said(&at, format!("`{named}` names no reader of this journal"))
            })?;
            out.insert(reader, cursor);
        }
        Ok(out)
    }
}

/// A segment's file name, from the first sequence number in it.
///
/// **Twenty digits, so the names sort in the order the numbers do.** Nothing
/// parses one for anything but its own listing, and the sequence a reader gets is
/// read out of the entry rather than out of the name.
fn the_name_of_the_segment_starting_at(first: SequenceNumber) -> String {
    format!("{first:020}{A_SEGMENTS_SUFFIX}")
}

/// The first sequence numbers of the segments in an area, oldest first.
///
/// **This is a listing rather than a reading of state**, which is the same
/// ground the record store beside this one states: `State, identity and
/// recovery` §1.3 rule 5 bars reading a file's presence, name or size **as
/// state**, and every state fact here comes from inside an entry. What the
/// listing supplies is only which segments the supervisor has written.
///
/// **A name that is not a segment's is skipped and never read as one** — the
/// registration record and any staging file in flight among them.
fn the_segments_in(
    area: &Path,
    entries_in_one_segment: u64,
) -> Result<Vec<SequenceNumber>, TheJournalRefused> {
    let entries = std::fs::read_dir(area).map_err(|why| the_store_said(area, why))?;
    let mut firsts = Vec::new();
    for entry in entries {
        let path = entry.map_err(|why| the_store_said(area, why))?.path();
        let Some(name) = path.file_name().and_then(|name| name.to_str()) else {
            continue;
        };
        let Some(digits) = name.strip_suffix(A_SEGMENTS_SUFFIX) else {
            continue;
        };
        let first = digits.parse::<SequenceNumber>().map_err(|why| {
            the_store_said(&path, format!("a segment's name is not a sequence: {why}"))
        })?;
        firsts.push(first);
    }
    firsts.sort_unstable();
    // **The names are checked against each other rather than trusted.** Each
    // segment but the first begins exactly one segment's worth after the one
    // before it, and a run that does not is a journal whose later sequence
    // numbers cannot be derived at all.
    for pair in firsts.windows(2) {
        if pair[1] != pair[0] + entries_in_one_segment {
            return Err(the_store_said(
                area,
                format!(
                    "the segment beginning at {} is followed by one beginning at {}, which is \
                     not one segment's worth after it",
                    pair[0], pair[1]
                ),
            ));
        }
    }
    Ok(firsts)
}

/// How many bytes of `bytes` are whole lines — everything up to and including
/// the last newline.
fn the_length_of_the_whole_lines_in(bytes: &[u8]) -> u64 {
    match bytes.iter().rposition(|byte| *byte == b'\n') {
        Some(at) => at as u64 + 1,
        None => 0,
    }
}

fn how_many_lines_in(bytes: &[u8]) -> u64 {
    bytes.iter().filter(|byte| **byte == b'\n').count() as u64
}

#[cfg(test)]
mod tests {
    use super::*;
    use smoozy_state::observed::{Clock, MachineClock, Reading};
    use std::time::{Duration, SystemTime};

    fn an_area() -> PathBuf {
        std::env::temp_dir().join(format!(
            "smoozy-runtime-journal-{}-{}",
            std::process::id(),
            {
                static NEXT: std::sync::atomic::AtomicUsize =
                    std::sync::atomic::AtomicUsize::new(0);
                NEXT.fetch_add(1, std::sync::atomic::Ordering::SeqCst)
            }
        ))
    }

    fn what_happened(says: &str) -> WhatHappened {
        WhatHappened {
            kind: "a refusal".into(),
            about: None,
            says: says.into(),
            happened_at: Reading::taken(
                (),
                SystemTime::UNIX_EPOCH + Duration::from_secs(1_777_000_000),
                Clock::AMachine(MachineClock("the machine".into())),
            ),
        }
    }

    /// **E1: an event is journalled with an identity and the next sequence
    /// number, and it comes back as it was written.**
    #[test]
    fn an_event_is_journalled_and_read_back_whole() {
        let area = an_area();
        let mut journal = TheEventJournalOnTheMachine::in_the_area(&area).unwrap();
        let journalled = journal.journal(what_happened("a caller of another login")).unwrap();
        assert_eq!(journalled.sequence(), 0);
        assert_eq!(journalled.event().kind(), IdentityKind::Event);

        let read = journal.read_forward_from(0, 100).unwrap();
        assert_eq!(read.events, vec![journalled]);
        assert_eq!(read.oldest_retained, Some(0));
        assert_eq!(read.next_sequence, 1);
        assert!(!read.it_missed_events());
        std::fs::remove_dir_all(&area).ok();
    }

    /// **E1: the sequence only increases and is contiguous**, which is what E5's
    /// gap detection rests on.
    #[test]
    fn the_sequence_numbers_are_contiguous_and_only_increase() {
        let area = an_area();
        let mut journal = TheEventJournalOnTheMachine::in_the_area(&area).unwrap();
        for at in 0..25u64 {
            assert_eq!(journal.journal(what_happened(&format!("{at}"))).unwrap().sequence(), at);
        }
        let read = journal.read_forward_from(0, 100).unwrap();
        for (at, event) in read.events.iter().enumerate() {
            assert_eq!(event.sequence(), at as u64);
        }
        assert_eq!(read.events.len(), 25);
        std::fs::remove_dir_all(&area).ok();
    }

    /// **E3: a read never advances a cursor**, so a reader that read and then
    /// died reads the same events again — E4's at-least-once delivery.
    #[test]
    fn a_read_advances_no_cursor_and_the_same_events_come_again() {
        let area = an_area();
        let mut journal = TheEventJournalOnTheMachine::in_the_area(&area).unwrap();
        journal.register(ARegisteredReader::TheDesktop).unwrap();
        for at in 0..3u64 {
            journal.journal(what_happened(&format!("{at}"))).unwrap();
        }
        let first = journal.read_forward_from(0, 100).unwrap();
        let again = journal.read_forward_from(0, 100).unwrap();
        assert_eq!(first.events, again.events);
        assert_eq!(
            journal.every_registration()[&ARegisteredReader::TheDesktop],
            0,
            "nothing about a read moves a cursor"
        );
        std::fs::remove_dir_all(&area).ok();
    }

    /// **E2 and E3: a reader's own acknowledgement moves its own row and no
    /// other, and a cursor never goes backwards.**
    #[test]
    fn a_reader_moves_its_own_cursor_and_never_another_readers_and_never_backwards() {
        let area = an_area();
        let mut journal = TheEventJournalOnTheMachine::in_the_area(&area).unwrap();
        for reader in ARegisteredReader::BOTH {
            journal.register(reader).unwrap();
        }
        for at in 0..5u64 {
            journal.journal(what_happened(&format!("{at}"))).unwrap();
        }
        journal
            .a_reader_has_passed(ARegisteredReader::TheDesktop, 4)
            .unwrap();
        assert_eq!(
            journal.every_registration()[&ARegisteredReader::TheDesktop],
            4
        );
        assert_eq!(
            journal.every_registration()[&ARegisteredReader::TheTelegramWorker],
            0,
            "a reader never advances another reader's cursor"
        );
        journal
            .a_reader_has_passed(ARegisteredReader::TheDesktop, 1)
            .unwrap();
        assert_eq!(
            journal.every_registration()[&ARegisteredReader::TheDesktop],
            4,
            "a cursor never goes backwards"
        );
        std::fs::remove_dir_all(&area).ok();
    }

    /// **The registrations survive a restart**, so the retention floor is the
    /// same floor after one.
    #[test]
    fn the_registrations_are_durable_across_a_restart() {
        let area = an_area();
        {
            let mut journal = TheEventJournalOnTheMachine::in_the_area(&area).unwrap();
            journal.register(ARegisteredReader::TheTelegramWorker).unwrap();
            for at in 0..4u64 {
                journal.journal(what_happened(&format!("{at}"))).unwrap();
            }
            journal
                .a_reader_has_passed(ARegisteredReader::TheTelegramWorker, 3)
                .unwrap();
        }
        let journal = TheEventJournalOnTheMachine::in_the_area(&area).unwrap();
        assert_eq!(
            journal.every_registration()[&ARegisteredReader::TheTelegramWorker],
            3
        );
        assert_eq!(journal.next_sequence(), 4);
        std::fs::remove_dir_all(&area).ok();
    }

    /// **A supervisor that did not write them reads them, and its own next
    /// entry carries on the sequence.** This is the store's half of the
    /// kill-and-restart proof; the real binary's half is in that member's own
    /// integration test.
    #[test]
    fn a_restart_carries_on_the_sequence_and_repeats_nothing() {
        let area = an_area();
        {
            let mut journal = TheEventJournalOnTheMachine::in_the_area(&area).unwrap();
            for at in 0..7u64 {
                journal.journal(what_happened(&format!("before {at}"))).unwrap();
            }
            // Nothing is closed, flushed or tidied on the way out: a kill
            // leaves exactly what the last durable append left.
        }
        let mut after = TheEventJournalOnTheMachine::in_the_area(&area).unwrap();
        assert_eq!(after.next_sequence(), 7);
        let carried_on = after.journal(what_happened("after")).unwrap();
        assert_eq!(carried_on.sequence(), 7);

        let read = after.read_forward_from(0, 100).unwrap();
        assert_eq!(read.events.len(), 8);
        let mut identities: Vec<String> = read
            .events
            .iter()
            .map(|event| event.event().as_text())
            .collect();
        identities.sort();
        identities.dedup();
        assert_eq!(identities.len(), 8, "nothing is repeated");
        std::fs::remove_dir_all(&area).ok();
    }

    /// **An append that lands part of itself and then fails leaves the segment
    /// exactly as it was, and the entry after it reads back whole.**
    ///
    /// **This is the fault the round found and it ruined a machine's journal for
    /// good.** The bytes that landed had no ending behind them; the next entry
    /// was written straight after them; the two became one line nobody wrote,
    /// and from that point no reader could read that machine's journal at all —
    /// the record that is meant to go on working in exactly the moment the other
    /// store fails.
    #[test]
    fn an_append_that_lands_part_of_itself_and_fails_leaves_the_segment_whole() {
        let area = an_area();
        let mut journal = TheEventJournalOnTheMachine::in_the_area(&area).unwrap();
        journal.journal(what_happened("one")).unwrap();
        let at = the_only_segment_in(&area);
        let after_one = std::fs::metadata(&at).unwrap().len();

        journal.pose_an_append_that_lands_part_of_it_and_then_fails(20);
        let refused = journal.journal(what_happened("two")).unwrap_err();
        assert!(
            format!("{refused:?}").contains("No space left on device"),
            "{refused:?}"
        );
        assert_eq!(
            std::fs::metadata(&at).unwrap().len(),
            after_one,
            "the bytes that landed are taken off again, so the segment is what it was"
        );

        // **The next entry is a real one and the whole journal reads back.**
        journal.journal(what_happened("three")).unwrap();
        let read = journal.read_forward_from(0, 100).unwrap();
        assert_eq!(read.events.len(), 2, "one line per entry and no joined line");
        assert_eq!(read.events[0].what().says, "one");
        assert_eq!(read.events[1].what().says, "three");
        // **The refused entry's number is used again**, exactly as a kill inside
        // an append leaves it: an entry that never completed never happened.
        assert_eq!(read.events[1].sequence(), 1);
        std::fs::remove_dir_all(&area).ok();
    }

    /// **Where the stump itself could not be taken off, nothing else is
    /// appended in this run.**
    ///
    /// The pose leaves bytes on the device and refuses the trim after them, which
    /// is a mount that has no room for either act. **What the journal must not do
    /// then is write after the stump**, because that is the joined line no reader
    /// can get past — so every further append in this run is refused instead, and
    /// the next start's own trim clears it.
    #[test]
    fn a_stump_that_could_not_be_taken_off_shuts_the_journal_for_this_run() {
        let area = an_area();
        let mut journal = TheEventJournalOnTheMachine::in_the_area(&area).unwrap();
        journal.journal(what_happened("one")).unwrap();

        journal.pose_an_append_and_a_trim_that_both_fail(20);
        let _ = journal.journal(what_happened("two")).unwrap_err();

        let refused = journal.journal(what_happened("three")).unwrap_err();
        assert!(
            format!("{refused:?}").contains("no further event is appended in this run"),
            "{refused:?}"
        );
        std::fs::remove_dir_all(&area).ok();
    }

    /// The one segment a short journal has, by name.
    fn the_only_segment_in(area: &Path) -> PathBuf {
        area.join(the_name_of_the_segment_starting_at(0))
    }

    /// **E1: an append that never completed never happened.** A kill inside one
    /// leaves bytes with no ending behind them, and the next start cuts them off
    /// rather than appending after the stump.
    #[test]
    fn a_part_written_entry_is_cut_off_and_its_number_is_used_again() {
        let area = an_area();
        let at = {
            let mut journal = TheEventJournalOnTheMachine::in_the_area(&area).unwrap();
            journal.journal(what_happened("one")).unwrap();
            journal.journal(what_happened("two")).unwrap();
            journal.where_a_segment_sits(0)
        };
        let mut bytes = std::fs::read(&at).unwrap();
        bytes.extend_from_slice(br#"{"event":{"kind":"Event","tex"#);
        std::fs::write(&at, &bytes).unwrap();

        let mut journal = TheEventJournalOnTheMachine::in_the_area(&area).unwrap();
        assert_eq!(journal.next_sequence(), 2, "the stump never happened");
        let next = journal.journal(what_happened("three")).unwrap();
        assert_eq!(next.sequence(), 2);
        let read = journal.read_forward_from(0, 100).unwrap();
        assert_eq!(read.events.len(), 3);
        assert_eq!(read.events[2].what().says, "three");
        std::fs::remove_dir_all(&area).ok();
    }

    /// **E5: a reader that fell behind a drop is told**, and the oldest retained
    /// number is what tells it.
    #[test]
    fn a_reader_below_the_oldest_retained_number_is_told_it_missed_events() {
        let area = an_area();
        let mut journal = TheEventJournalOnTheMachine::in_the_area_holding(&area, 2, 4).unwrap();
        journal.register(ARegisteredReader::TheDesktop).unwrap();
        for at in 0..10u64 {
            journal.journal(what_happened(&format!("{at}"))).unwrap();
        }
        // The desktop's cursor never moved, so the cap dropped past it — which
        // is E6's own named case and the one E5 makes visible.
        assert_eq!(
            journal.every_registration()[&ARegisteredReader::TheDesktop],
            0
        );
        let read = journal.read_forward_from(0, 100).unwrap();
        assert!(read.it_missed_events());
        assert!(read.oldest_retained.unwrap() > 0);
        std::fs::remove_dir_all(&area).ok();
    }

    /// **E6: whole segments go from the oldest end once the cap is passed, and
    /// the journal never holds more than the cap.**
    ///
    /// **The cap is lowered for this test and the rule is not.** Writing 32,768
    /// entries at the desk would take a minute of real appends to prove a rule
    /// `what_retention_keeps` already states; what is exercised here is the
    /// dropping itself — that a segment is really removed, that the oldest
    /// retained number really moves, and that reading forward across the hole
    /// still reads.
    #[test]
    fn the_oldest_segments_are_dropped_once_the_cap_is_passed() {
        let area = an_area();
        let a_small_cap = 4u64;
        let mut journal =
            TheEventJournalOnTheMachine::in_the_area_holding(&area, 2, a_small_cap).unwrap();
        for at in 0..12u64 {
            journal.journal(what_happened(&format!("{at}"))).unwrap();
            assert!(
                journal.next_sequence() - journal.oldest_retained().unwrap() <= a_small_cap,
                "the journal never holds more than the cap"
            );
        }
        let read = journal.read_forward_from(0, 100).unwrap();
        assert!(read.it_missed_events(), "the oldest-retained number moved");
        assert_eq!(
            read.events.first().unwrap().sequence(),
            journal.oldest_retained().unwrap()
        );
        // **The segments really went**, rather than the number moving over files
        // still on the disk.
        let left = std::fs::read_dir(&area)
            .unwrap()
            .filter(|entry| {
                entry
                    .as_ref()
                    .unwrap()
                    .file_name()
                    .to_string_lossy()
                    .ends_with(A_SEGMENTS_SUFFIX)
            })
            .count();
        assert_eq!(left, 2);
        std::fs::remove_dir_all(&area).ok();
    }

    /// **The divergence this plan's own Section 16 item 26 records, posed at a
    /// desk: a journal append outlives the moment the record store starts
    /// refusing.**
    ///
    /// **What is posed here and what is not.** Item 26's observation was on a
    /// genuinely full mount, and its mechanism has two halves: *"an append
    /// fitting inside the slack of an already-allocated last block needs no new
    /// block; a record write needs a new file and a new directory entry."* This
    /// poses the **directory-entry** half, by closing both areas to new entries
    /// rather than by filling a mount — the desk has no mount it can fill and
    /// the Creator ruled on 2026-08-31 that no machine is rented to answer a
    /// question. **The block-slack half is not posed here and is not claimed.**
    ///
    /// **The bound differs too and is said rather than implied.** On the rental
    /// the appends outlived the refusal by exactly one block — seventy-six
    /// entries — and here they outlive it for as long as the area refuses a new
    /// entry, which is until the segment is full. **What is the same is the
    /// shape, and the shape is the finding**: the journal goes on carrying
    /// events about records that cannot be written.
    #[test]
    fn a_journal_append_outlives_the_moment_the_record_store_starts_refusing() {
        use crate::acting::operation_records::TheOperationRecordsOnTheMachine;
        use smoozy_state::machine_operations::{Keeps, OperationRecord, Refused};
        use std::os::unix::fs::PermissionsExt;

        let package_area = an_area();
        let journal_area = package_area.join(THE_AREA_THE_JOURNAL_SITS_IN);
        let records_area = package_area.join("operation-records");
        let mut journal = TheEventJournalOnTheMachine::in_the_area(&journal_area).unwrap();
        let mut records = TheOperationRecordsOnTheMachine::in_the_area(&records_area).unwrap();

        // Both stores work while both areas take a new entry.
        journal.journal(what_happened("before")).unwrap();
        let a_record = OperationRecord::opened(
            Identity::issue(IdentityKind::Operation).unwrap(),
            Identity::issue(IdentityKind::Job).unwrap(),
            "render",
        );
        records.record_before_acting(&a_record).unwrap();

        for area in [&journal_area, &records_area] {
            let mut closed = std::fs::metadata(area).unwrap().permissions();
            closed.set_mode(0o500);
            std::fs::set_permissions(area, closed).unwrap();
        }

        // The record store needs a new file and a new directory entry.
        let another = OperationRecord::opened(
            Identity::issue(IdentityKind::Operation).unwrap(),
            Identity::issue(IdentityKind::Job).unwrap(),
            "render",
        );
        assert!(
            matches!(
                records.record_before_acting(&another),
                Err(Refused::TheStore(_))
            ),
            "a record write needs a new file and a new directory entry"
        );

        // The journal needs neither, and goes on.
        for at in 0..5u64 {
            let landed = journal
                .journal(what_happened(&format!("after {at}")))
                .expect("an append needs no new entry");
            assert_eq!(landed.sequence(), at + 1);
        }

        for area in [&journal_area, &records_area] {
            let mut open_again = std::fs::metadata(area).unwrap().permissions();
            open_again.set_mode(0o700);
            std::fs::set_permissions(area, open_again).ok();
        }
        std::fs::remove_dir_all(&package_area).ok();
    }

    /// **And the divergence is bounded rather than endless**: the moment the
    /// journal needs a segment of its own it needs a directory entry too, and it
    /// refuses exactly as the record store did.
    #[test]
    fn the_divergence_ends_when_the_journal_needs_a_new_segment() {
        use std::os::unix::fs::PermissionsExt;
        let area = an_area();
        let mut journal = TheEventJournalOnTheMachine::in_the_area_holding(&area, 2, 100).unwrap();
        journal.journal(what_happened("one")).unwrap();

        let mut closed = std::fs::metadata(&area).unwrap().permissions();
        closed.set_mode(0o500);
        std::fs::set_permissions(&area, closed).unwrap();

        journal
            .journal(what_happened("two"))
            .expect("the segment still has room");
        let refused = journal
            .journal(what_happened("three"))
            .expect_err("a new segment needs a new directory entry");
        assert!(matches!(refused, TheJournalRefused::TheStore(_)));

        let mut open_again = std::fs::metadata(&area).unwrap().permissions();
        open_again.set_mode(0o700);
        std::fs::set_permissions(&area, open_again).ok();
        std::fs::remove_dir_all(&area).ok();
    }

    /// **E6a: an entry longer than the budget is refused rather than trimmed**,
    /// and the caller is told to name a record instead.
    #[test]
    fn an_entry_longer_than_the_budget_is_refused_rather_than_trimmed() {
        let area = an_area();
        let mut journal = TheEventJournalOnTheMachine::in_the_area(&area).unwrap();
        let too_much = what_happened(&"a".repeat(THE_PER_ENTRY_BUDGET as usize));
        let refused = journal.journal(too_much).expect_err("it does not fit");
        assert!(matches!(
            refused,
            TheJournalRefused::TheEntryIsLongerThanTheBudget { .. }
        ));
        assert!(refused.as_text().contains("names the record"));
        assert_eq!(journal.next_sequence(), 0, "nothing landed part-said");
        assert!(journal.read_forward_from(0, 100).unwrap().events.is_empty());
        std::fs::remove_dir_all(&area).ok();
    }

    /// **E6a's four never-abbreviated fields fit the budget with room over**,
    /// which is what makes the refusal above a caller's fault rather than the
    /// form's.
    #[test]
    fn an_entry_carrying_only_what_is_never_abbreviated_fits_the_budget() {
        let area = an_area();
        let mut journal = TheEventJournalOnTheMachine::in_the_area(&area).unwrap();
        let mut what = what_happened("");
        what.about = Some(Identity::issue(IdentityKind::Job).unwrap());
        journal.journal(what).expect("the four fit");
        std::fs::remove_dir_all(&area).ok();
    }

    /// **A word that names no reader of this journal is refused rather than
    /// registered**, because a registration nothing advances raises the
    /// retention floor for ever.
    #[test]
    fn a_registration_naming_no_reader_of_this_journal_is_refused() {
        let area = an_area();
        {
            let mut journal = TheEventJournalOnTheMachine::in_the_area(&area).unwrap();
            journal.register(ARegisteredReader::TheDesktop).unwrap();
            std::fs::write(
                journal.where_the_registrations_sit(),
                br#"[["somebody else",3]]"#,
            )
            .unwrap();
        }
        let why = TheEventJournalOnTheMachine::in_the_area(&area)
            .expect_err("it names no reader of this journal");
        assert!(why.as_text().contains("names no reader"));
        std::fs::remove_dir_all(&area).ok();
    }

    /// **A segment run whose names do not step by one segment's worth is
    /// refused**, because every later sequence number is derived from that step.
    #[test]
    fn a_segment_run_with_a_hole_in_its_names_is_refused_rather_than_read() {
        let area = an_area();
        {
            let mut journal = TheEventJournalOnTheMachine::in_the_area(&area).unwrap();
            journal.journal(what_happened("one")).unwrap();
        }
        std::fs::write(
            area.join(the_name_of_the_segment_starting_at(9_999)),
            b"{}\n",
        )
        .unwrap();
        let why = TheEventJournalOnTheMachine::in_the_area(&area)
            .expect_err("the names do not step by one segment's worth");
        assert!(why.as_text().contains("not one segment's worth"));
        std::fs::remove_dir_all(&area).ok();
    }

    /// **An entry whose written sequence disagrees with where the segment puts
    /// it is refused loudly**, because a reader handed the wrong numbers cannot
    /// detect the gap E5 gives it.
    #[test]
    fn an_entry_whose_number_disagrees_with_its_position_is_refused() {
        let area = an_area();
        let mut journal = TheEventJournalOnTheMachine::in_the_area(&area).unwrap();
        journal.journal(what_happened("one")).unwrap();
        journal.journal(what_happened("two")).unwrap();
        let at = journal.where_a_segment_sits(0);
        let bytes = std::fs::read(&at).unwrap();
        let text = String::from_utf8(bytes).unwrap();
        std::fs::write(&at, text.replace("\"sequence\":1", "\"sequence\":7")).unwrap();
        let why = journal
            .read_forward_from(0, 100)
            .expect_err("the number disagrees with the position");
        assert!(why.as_text().contains("says it is sequence 7"));
        std::fs::remove_dir_all(&area).ok();
    }
}
