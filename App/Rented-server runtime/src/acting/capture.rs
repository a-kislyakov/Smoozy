//! The two sinks, made by the supervisor, and the worker's own copy into them.
//!
//! **[`crate::capture`] holds the rules** — the ceiling, what the worker does
//! when it is reached, the four collection outcomes and what each rests on.
//! This module is the two files and the write.
//!
//! # Who writes which, and why it is split that way
//!
//! **The supervisor makes the sinks and the worker copies into them.** M1 puts
//! the making before the worker is started; M3 gives the copy to the worker, for
//! two observed reasons — the supervisor may die while the render lives, so a
//! capture that ran in the supervisor would stop mid-render in exactly the case
//! the *resumed* branch exists for; and the worker is already the process that
//! remains until its external processes have exited (X12), so it is alive for
//! the whole of the render by construction.
//!
//! **The worker does not hand the render a descriptor onto a file** and does not
//! pipe the streams through the supervisor. What this module offers the worker
//! is a write, and the bytes come from the worker's own loop.
//!
//! # The count, and why it is returned rather than read
//!
//! M4: the count is Smoozy's own count and never a reading of a sink's size.
//! `State, identity and recovery` §1.3 rule 5 forbids reading a file's size as
//! state, and the reason is the one that defeats every other size reading in this
//! project — a zero-byte file *"is indistinguishable between 'still being
//! written', 'the disk filled' and 'the render wrote nothing'"*. **So the write
//! below answers with how many bytes it retained, and nothing anywhere asks the
//! sink.**
//!
//! # No probe byte, ever
//!
//! M2: no probe byte is written into a sink to confirm its placement, because a
//! byte written to confirm a placement would be a byte in the raw record that the
//! machine did not print. The whole-root real-write confirmation was performed
//! once at hand-over and is not repeated per stream.
//!
//! # These are material rather than state, and the difference is a flush
//!
//! S1 makes the supervisor the only writer of every durable store *"except the
//! staging areas and the captured streams, both of which are written under its
//! control"*, and M6 says the sinks are material in the same sense the staging
//! areas are. **So a stream write is not one of `State, identity and recovery`
//! §1.3 rule 6's durable writes and does not carry its flush**: the rule governs
//! stores, a flush per write would make the copy loop's cost the device's rather
//! than the render's, and M26 already names what a worker that died can leave —
//! bytes in a sink its count never reached — which is why *collection
//! incomplete* is its own outcome and never collapses into *collected and
//! empty*.

use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::Path;
use std::sync::Mutex;

use crate::capture::{CaptureRecord, CopiesAStreamIntoItsSink, MakesTheSinks, Stream};

/// The supervisor's making of the two sinks.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct TheSinksOnTheMachine;

impl MakesTheSinks for TheSinksOnTheMachine {
    /// **M1.** Two sinks per attempt, at the paths the capture record names.
    ///
    /// **A path that is already there is a refusal rather than an append.** M19
    /// makes a re-run a new attempt with its own two sinks and its own two
    /// outcomes, and M18 writes each outcome once and never rewrites it — so a
    /// second attempt landing in a first attempt's sink would put two attempts'
    /// bytes behind one outcome. **§14 is what the refusal keeps**: where the
    /// sinks cannot be made the worker is not attempted, naming what stopped it.
    ///
    /// **The pair is made whole or not at all.** §14 declares the start one
    /// unit, and a first sink left on disk by a failed second one cannot be
    /// recovered from: the create refuses an existing path, deliberately and for
    /// M19's reason, so a second attempt with the same capture record could
    /// never succeed. **Only what this call made is removed** — a path that was
    /// already there is somebody else's and is left alone.
    fn make(&self, record: &CaptureRecord) -> Result<(), String> {
        let mut made: Vec<&str> = Vec::new();
        for at in [&record.ordinary_sink, &record.complaints_sink] {
            let path = Path::new(at);
            let attempted = match path.parent() {
                Some(area) => crate::acting::durable::make_the_area(area)
                    .map_err(|why| why.as_text())
                    .and_then(|()| the_sink_at(path)),
                None => the_sink_at(path),
            };
            match attempted {
                Ok(()) => made.push(at),
                Err(why) => {
                    for already in made {
                        std::fs::remove_file(already).ok();
                    }
                    return Err(why);
                }
            }
        }
        Ok(())
    }
}

/// One sink, created where nothing is there.
fn the_sink_at(path: &Path) -> Result<(), String> {
    OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .map(|_| ())
        .map_err(|why| format!("{}: {why}", path.display()))
}

/// The worker's own copy into the two sinks.
///
/// **One of these is held per attempt**, opened after the supervisor made the
/// sinks and before the render starts.
#[derive(Debug)]
pub struct TheCopyIntoTheSinks {
    ordinary: Mutex<File>,
    complaints: Mutex<File>,
}

impl TheCopyIntoTheSinks {
    /// Open the two sinks the supervisor made.
    ///
    /// **Neither is created here.** A sink that is not there is a capture record
    /// whose sinks were never made, which M13 reads as *never established*
    /// rather than as something to repair by making one now.
    pub fn opening(record: &CaptureRecord) -> Result<Self, String> {
        Ok(TheCopyIntoTheSinks {
            ordinary: Mutex::new(open_the_sink(&record.ordinary_sink)?),
            complaints: Mutex::new(open_the_sink(&record.complaints_sink)?),
        })
    }
}

fn open_the_sink(at: &str) -> Result<File, String> {
    OpenOptions::new()
        .append(true)
        .open(at)
        .map_err(|why| format!("{at}: {why}"))
}

impl CopiesAStreamIntoItsSink for TheCopyIntoTheSinks {
    /// **M4's own loop.**
    ///
    /// At most `may_write` bytes are offered, which is M4a's remaining-byte
    /// counter arriving here as an argument — the two streams draw from one
    /// counter and the arithmetic is [`crate::capture::CaptureCeiling`]'s.
    ///
    /// **What comes back is what the write retained**, so a write that took
    /// fewer bytes than it was offered decrements the ceiling by what it took
    /// and no more. **A refusal is the machine's own words**, which M5 has the
    /// worker record once per stream — on a full filesystem it is the loud
    /// `errno 28` one, uniformly.
    ///
    /// **An interrupted write is a try-again and not M5's refusal.** A write can
    /// come back interrupted because a signal was delivered, which happens
    /// during the stop sequence, and reading that as M5's refusal ends durable
    /// writes for that stream for the rest of the render and makes the recorded
    /// outcome *collection incomplete* — telling the user the machine's words
    /// are a prefix and the rest was lost when nothing was lost.
    fn write(&self, stream: Stream, bytes: &[u8], may_write: u64) -> Result<u64, String> {
        let how_many = (bytes.len() as u64).min(may_write) as usize;
        if how_many == 0 {
            return Ok(0);
        }
        let mut sink = match stream {
            Stream::Ordinary => self.ordinary.lock().expect("the sink is not poisoned"),
            Stream::Complaints => self.complaints.lock().expect("the sink is not poisoned"),
        };
        match write_retrying_an_interruption(&mut *sink, &bytes[..how_many]) {
            Ok(retained) => Ok(retained as u64),
            Err(why) => Err(why.to_string()),
        }
    }
}

/// **One write, with an interruption tried again and every other refusal
/// carried.**
///
/// A signal delivered during the write is not the disk refusing, and M5's end of
/// durable writes for a stream is about the disk refusing. **Every other error is
/// M5's refusal and is carried verbatim**, including the loud `errno 28` one.
fn write_retrying_an_interruption(
    sink: &mut impl Write,
    bytes: &[u8],
) -> std::io::Result<usize> {
    loop {
        match sink.write(bytes) {
            Err(why) if why.kind() == std::io::ErrorKind::Interrupted => continue,
            other => return other,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::capture::{
        collection_outcome, CaptureCeiling, CollectionOutcome, WhatTheWorkerObserved,
    };
    use smoozy_state::identities::{Identity, IdentityKind};
    use std::path::PathBuf;

    fn an_area() -> PathBuf {
        let area = std::env::temp_dir().join(format!(
            "smoozy-runtime-capture-{}-{:?}",
            std::process::id(),
            {
                static NEXT: std::sync::atomic::AtomicUsize =
                    std::sync::atomic::AtomicUsize::new(0);
                NEXT.fetch_add(1, std::sync::atomic::Ordering::SeqCst)
            }
        ));
        std::fs::create_dir_all(&area).expect("the area is made");
        area
    }

    fn a_record(area: &Path, available: u64) -> CaptureRecord {
        CaptureRecord {
            job: Identity::issue(IdentityKind::Job).expect("the desk has randomness"),
            attempt: 1,
            ordinary_sink: area.join("attempt 1/ordinary").display().to_string(),
            complaints_sink: area.join("attempt 1/complaints").display().to_string(),
            ceiling: CaptureCeiling::from(available),
        }
    }

    #[test]
    fn the_two_sinks_are_made_empty_and_nothing_is_written_into_either() {
        let area = an_area();
        let record = a_record(&area, 128 * 1024 * 1024);
        TheSinksOnTheMachine.make(&record).expect("they are made");
        for at in [&record.ordinary_sink, &record.complaints_sink] {
            let held = std::fs::read(at).expect("the sink is there");
            assert!(held.is_empty(), "no probe byte is ever written into a sink");
        }
        std::fs::remove_dir_all(&area).ok();
    }

    #[test]
    fn a_sink_that_is_already_there_refuses_rather_than_being_appended_to() {
        let area = an_area();
        let record = a_record(&area, 128 * 1024 * 1024);
        TheSinksOnTheMachine.make(&record).expect("they are made");
        let why = TheSinksOnTheMachine
            .make(&record)
            .expect_err("a second attempt does not land in the first's sink");
        assert!(why.contains("ordinary"));
        std::fs::remove_dir_all(&area).ok();
    }

    #[test]
    fn the_two_streams_are_two_sinks_and_are_never_merged() {
        let area = an_area();
        let record = a_record(&area, 128 * 1024 * 1024);
        TheSinksOnTheMachine.make(&record).expect("they are made");
        let copy = TheCopyIntoTheSinks::opening(&record).expect("both open");
        assert_eq!(copy.write(Stream::Ordinary, b"what it printed", 1000).unwrap(), 15);
        assert_eq!(copy.write(Stream::Complaints, b"what it complained", 1000).unwrap(), 18);
        assert_eq!(
            std::fs::read(&record.ordinary_sink).unwrap(),
            b"what it printed"
        );
        assert_eq!(
            std::fs::read(&record.complaints_sink).unwrap(),
            b"what it complained"
        );
        std::fs::remove_dir_all(&area).ok();
    }

    /// **The count is what the write returned**, and the ceiling is decremented
    /// by that and by nothing read back from the sink.
    #[test]
    fn a_write_offered_less_than_it_carries_retains_only_what_it_was_offered() {
        let area = an_area();
        let record = a_record(&area, 128 * 1024 * 1024);
        TheSinksOnTheMachine.make(&record).expect("they are made");
        let copy = TheCopyIntoTheSinks::opening(&record).expect("both open");
        let retained = copy
            .write(Stream::Ordinary, b"a longer line than it may write", 6)
            .unwrap();
        assert_eq!(retained, 6);
        assert_eq!(std::fs::read(&record.ordinary_sink).unwrap(), b"a long");
        std::fs::remove_dir_all(&area).ok();
    }

    /// **A spent ceiling retains nothing**, which is M5's own end of durable
    /// writes for that stream, and the read goes on being the worker's.
    #[test]
    fn a_spent_ceiling_retains_nothing() {
        let area = an_area();
        let record = a_record(&area, 128 * 1024 * 1024);
        TheSinksOnTheMachine.make(&record).expect("they are made");
        let copy = TheCopyIntoTheSinks::opening(&record).expect("both open");
        assert_eq!(copy.write(Stream::Ordinary, b"anything at all", 0).unwrap(), 0);
        assert!(std::fs::read(&record.ordinary_sink).unwrap().is_empty());
        std::fs::remove_dir_all(&area).ok();
    }

    /// **The two streams drawing from one counter**, which is M4a, run through
    /// the real write.
    #[test]
    fn the_two_streams_draw_from_one_remaining_byte_counter() {
        let area = an_area();
        let mut record = a_record(&area, crate::capture::THE_CONTROL_STATE_RESERVE + 10);
        TheSinksOnTheMachine.make(&record).expect("they are made");
        let copy = TheCopyIntoTheSinks::opening(&record).expect("both open");
        assert_eq!(record.ceiling.remaining(), 10);

        let took = copy
            .write(Stream::Ordinary, b"1234567", record.ceiling.may_offer(7))
            .unwrap();
        record.ceiling.retained(took);
        assert_eq!(record.ceiling.remaining(), 3);

        let took = copy
            .write(
                Stream::Complaints,
                b"abcdefgh",
                record.ceiling.may_offer(8),
            )
            .unwrap();
        record.ceiling.retained(took);
        assert_eq!(took, 3);
        assert!(record.ceiling.is_spent());
        assert_eq!(std::fs::read(&record.complaints_sink).unwrap(), b"abc");
        std::fs::remove_dir_all(&area).ok();
    }

    /// **A sink the supervisor never made is not made here**, and the attempt
    /// reads as *never established* rather than being repaired into one.
    #[test]
    fn a_sink_the_supervisor_never_made_is_not_made_by_the_worker() {
        let area = an_area();
        let record = a_record(&area, 128 * 1024 * 1024);
        let why = TheCopyIntoTheSinks::opening(&record).expect_err("nothing was made");
        assert!(why.contains("ordinary"));
        assert!(!Path::new(&record.ordinary_sink).exists());
        assert_eq!(
            collection_outcome(
                false,
                false,
                &WhatTheWorkerObserved {
                    stream: Stream::Ordinary,
                    retained: 0,
                    discarded: 0,
                    first_loss_reason: None,
                    end_of_stream: true,
                    the_marker_passed: Some(true),
                    something_followed_the_marker: Some(false),
                },
                false,
            ),
            CollectionOutcome::NeverEstablished
        );
        std::fs::remove_dir_all(&area).ok();
    }

    /// **A half-made pair is not left behind.** The second create is refused
    /// because something is already at that path, and the first sink this call
    /// made is removed — otherwise no second attempt with the same capture
    /// record could ever succeed, the create refusing an existing path.
    #[test]
    fn a_pair_whose_second_sink_cannot_be_made_leaves_nothing_behind() {
        let area = an_area();
        let record = a_record(&area, 128 * 1024 * 1024);
        // Something is already at the second path, which is what the create
        // refuses.
        let complaints = Path::new(&record.complaints_sink);
        std::fs::create_dir_all(complaints.parent().unwrap()).unwrap();
        std::fs::write(complaints, b"somebody else's").unwrap();

        let why = TheSinksOnTheMachine
            .make(&record)
            .expect_err("the second sink cannot be made");
        assert!(why.contains("complaints"));
        assert!(
            !Path::new(&record.ordinary_sink).exists(),
            "the first sink of a pair that was not made whole is not left behind"
        );
        assert_eq!(
            std::fs::read(complaints).unwrap(),
            b"somebody else's",
            "only what this call made is removed"
        );
        std::fs::remove_dir_all(&area).ok();
    }

    /// **An interrupted write is a try-again**, so the bytes land and the stream
    /// is not ended for the rest of the render.
    #[test]
    fn a_sink_that_answers_interrupted_once_is_written_again_rather_than_given_up_on() {
        struct InterruptedOnceThenItLands {
            interrupted_yet: bool,
            landed: Vec<u8>,
        }
        impl Write for InterruptedOnceThenItLands {
            fn write(&mut self, bytes: &[u8]) -> std::io::Result<usize> {
                if !self.interrupted_yet {
                    self.interrupted_yet = true;
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::Interrupted,
                        "a signal was delivered",
                    ));
                }
                self.landed.extend_from_slice(bytes);
                Ok(bytes.len())
            }
            fn flush(&mut self) -> std::io::Result<()> {
                Ok(())
            }
        }

        let mut sink = InterruptedOnceThenItLands {
            interrupted_yet: false,
            landed: Vec::new(),
        };
        let retained = write_retrying_an_interruption(&mut sink, b"what it printed")
            .expect("an interruption is a try-again");
        assert_eq!(retained, 15);
        assert_eq!(sink.landed, b"what it printed");
        assert!(sink.interrupted_yet);
    }

    /// **And every other refusal is still M5's**, carried verbatim rather than
    /// tried again for ever.
    #[test]
    fn a_sink_that_answers_anything_but_interrupted_is_m5s_refusal() {
        struct ItRefuses;
        impl Write for ItRefuses {
            fn write(&mut self, _bytes: &[u8]) -> std::io::Result<usize> {
                Err(std::io::Error::new(
                    std::io::ErrorKind::StorageFull,
                    "No space left on device",
                ))
            }
            fn flush(&mut self) -> std::io::Result<()> {
                Ok(())
            }
        }
        let why = write_retrying_an_interruption(&mut ItRefuses, b"a line")
            .expect_err("the disk refused");
        assert_eq!(why.kind(), std::io::ErrorKind::StorageFull);
    }

    /// **A refused write carries the machine's own words**, which M5 has the
    /// worker record once per stream.
    #[test]
    fn a_refused_write_carries_the_machines_own_words() {
        let area = an_area();
        let record = a_record(&area, 128 * 1024 * 1024);
        TheSinksOnTheMachine.make(&record).expect("they are made");
        let copy = TheCopyIntoTheSinks::opening(&record).expect("both open");
        // The sink is taken away underneath the copy on a system where a write
        // to a closed descriptor is refused; where it is not, the write still
        // answers with a count rather than a claim about the sink.
        drop(std::fs::remove_file(&record.ordinary_sink));
        let answered = copy.write(Stream::Ordinary, b"a line", 1000);
        match answered {
            Ok(retained) => assert_eq!(retained, 6),
            Err(why) => assert!(!why.is_empty()),
        }
        std::fs::remove_dir_all(&area).ok();
    }
}
