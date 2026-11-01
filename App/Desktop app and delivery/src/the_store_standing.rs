//! **The store's standing's one writer** — the probe, taken once in a run at
//! the first moment a secret is actually needed, and the durable value it
//! moves.
//!
//! # What was missing, and it was only this
//!
//! `Security and licence` §3.3 S32 gives the standing **three readings and one
//! writer**, and every part of it was built except the writer's caller.
//! `smoozy_security_and_licence::standing` holds the three readings and S25's
//! whole reading rule in [`StoreStanding::after`]; `store_standing` is a table
//! in [`crate::record::schema`]; every adapter has a `probe`. **Nothing in the
//! application ever took the probe**, so the value read *never probed* for
//! ever and two clauses could never fire: `Servers` N8b's permanent refusal and
//! `Keys` S10's *the passphrase cannot be held on this computer* mark. On a
//! computer with no keyring the standing repair would spin for good against an
//! answer that cannot change, which is the exact thing N8b exists to prevent.
//!
//! # The two halves of S26, and the second one is the one a later agent breaks
//!
//! **The probe is taken at the first moment in a run when a secret is actually
//! needed, and never at application start.** Reaching the store may prompt, and
//! at start nothing is being handed over. So no call to this module belongs in
//! [`crate::start`], and [`tests::the_probe_is_not_taken_at_application_start`]
//! is what says so — it walks this crate's own files and names every one that
//! may call in.
//!
//! **It is taken once in a run and not again within it**, and re-taken at the
//! next run rather than cached across runs, because a user can install a
//! keyring between two sittings. [`TheProbeThisRun`] is the whole of that: the
//! application holds [`THIS_RUN`], and a test holds its own.
//!
//! # Nothing here composes a reading
//!
//! S25's rule is [`StoreStanding::after`]'s and is not restated. This module
//! reads the standing, hands the probe's outcome to that function, and writes
//! back what it hands over. **A fourth transition cannot be invented here
//! because none is expressed here.**
//!
//! # How a clause marker is written in this crate, and it is a rule rather than
//! a habit
//!
//! A marker with no document beside it means a clause of this crate's own plan,
//! `Preparation/Architecture/Desktop app and delivery.md`. Every other
//! document's marker is written with that document — at the site, or in this
//! file's own header list below. **Where a marker is a clause of this crate's
//! own plan *and* of the document meant, the header list is not enough and the
//! document is written out at the site**, because a bare one there resolves to a
//! real and wrong clause.
//!
//! **Markers used below that belong to another document, each with the document
//! it belongs to.** `Security and licence` S24, S25, S26, S32, S33, S34, S35,
//! S36. `Servers` N8b.
//!
//! **`Keys` S10 is written out at its site instead**, because this crate's own
//! plan defines an S10 of its own — erasure, confirmed where the system
//! confirms it — so a bare one there would resolve to a real and wrong clause.
//!
//! **This crate's own plan defines an S5 of its own** — *"the Linux probe is
//! the adapter's and is taken at first use in a run, not at start … the result
//! is re-taken once per application run and not cached across runs … what is
//! durable is not the result but the standing"* — **and that clause is this
//! module's whole job.** It is written bare below, because it is this crate's.

use std::sync::Mutex;
use std::time::Duration;

use smoozy_security_and_licence::secret_store::SecretStore;
use smoozy_security_and_licence::standing::{StandingWrite, StoreStanding};

use crate::keys::LendsTheRecord;

/// **Whether this run has taken its probe, and what it came to.**
///
/// S5 and `Security and licence` §3.3 S26: *first use* means first use **in
/// this run**. The probe result itself is run-held (`State, identity and
/// recovery` §11.1) and nothing durable carries it — what is durable is the
/// standing, which this holds only so that a second caller in the same run gets
/// the same answer without a second probe.
///
/// **The application has one of these and a test has its own.** A hidden
/// process-wide value would make *taken once in a run* untestable, and a test
/// binary runs many runs' worth of work in one process.
pub struct TheProbeThisRun {
    taken: Mutex<Option<StandingWrite>>,
}

impl TheProbeThisRun {
    pub const fn new() -> Self {
        TheProbeThisRun {
            taken: Mutex::new(None),
        }
    }
}

impl Default for TheProbeThisRun {
    fn default() -> Self {
        TheProbeThisRun::new()
    }
}

/// **The application's own run.** One process is one run, which is what makes
/// *re-taken at the next run rather than cached across runs* true by
/// construction: nothing in this value survives the process.
pub static THIS_RUN: TheProbeThisRun = TheProbeThisRun::new();

/// **The one writer of the store's standing** — `Security and licence` §3.3
/// S32, and this crate's own S5.
///
/// **Call it at a moment when a secret is actually needed and nowhere else.**
/// The first such call in a run takes the probe, hands its outcome to
/// [`StoreStanding::after`], and writes back what that returns. Every later
/// call in the same run reaches no store and returns the same answer.
///
/// **A standing that will not read writes nothing and probes nothing**, and the
/// run is left untaken so the next first use tries again. `Security and
/// licence` S25's two mistaken readings are opposite lies to the user, and
/// applying a transition to a standing nobody could read would produce one of
/// them: a store that answered once, misread as absent, would be re-classified
/// as *never answered* by one failed probe and the one-sentence notice would be
/// said to a user whose keyring works.
///
/// **S34's notice is handed back to the first caller in a run and to no
/// other.** *"It says so once"* is the value's first transition — *never
/// probed* to *never answered* — **once for the life of the installation**, so
/// a second caller in the same run must not be told to say it again. What the
/// caller does with it is the caller's; nothing here draws.
pub fn at_the_first_use_of_a_secret(
    this_run: &TheProbeThisRun,
    record: &dyn LendsTheRecord,
    store: &dyn SecretStore,
    deadline: Duration,
) -> Option<StandingWrite> {
    let Ok(mut taken) = this_run.taken.lock() else {
        return None;
    };
    if let Some(already) = *taken {
        return Some(already);
    }

    // **S33: it is a note Smoozy wrote about its own history**, so the
    // transition is applied to what the record holds and never to a guess. The
    // reading rule is `crate::surface::set_up::the_store_standing`'s and is not
    // written a second time here.
    let mut standing = None;
    record.lend(&mut |connection| {
        standing = crate::surface::set_up::the_store_standing(connection);
    });
    let standing = standing?;

    // **S24: the probe, and it establishes whether a round trip worked and
    // nothing beyond that.** The lock over the record is not held across it —
    // reaching the store may prompt, and on Linux that is a wait on the user.
    let outcome = store.probe(deadline);

    // **S25's whole reading rule, called rather than restated.**
    let write = standing.after(outcome);
    if write.standing != standing {
        record.lend(&mut |connection| {
            let _ = the_standing_is_written(connection, write.standing);
        });
    }

    // The notice belongs to the first caller in the run. See above.
    *taken = Some(StandingWrite {
        say_it_once: false,
        ..write
    });
    Some(write)
}

/// **S32's one row.** *Never probed* is *"the value's absence at first
/// launch"*, so it has no spelling and writes no row — which is also why
/// nothing here ever removes one: S33 says no control removes this value, and a
/// standing that could go back to absent would offer to tell the user again
/// about a thing that has not changed.
fn the_standing_is_written(
    connection: &rusqlite::Connection,
    standing: StoreStanding,
) -> Result<(), rusqlite::Error> {
    let Some(text) = the_text_for(standing) else {
        return Ok(());
    };
    connection
        .execute(
            "INSERT INTO store_standing (only_row, standing) VALUES (1, ?1)
             ON CONFLICT(only_row) DO UPDATE SET standing = excluded.standing",
            [text],
        )
        .map(|_| ())
}

/// The spelling each reading is written under, and **the reader in
/// `crate::surface::set_up` is the one that has to agree with it** —
/// [`tests::every_reading_this_writer_writes_is_one_the_reader_reads_back`]
/// holds the two together, because a fact kept in two places is the fault this
/// build stage keeps paying for.
fn the_text_for(standing: StoreStanding) -> Option<&'static str> {
    match standing {
        StoreStanding::NeverProbed => None,
        StoreStanding::NeverAnswered => Some("never answered"),
        StoreStanding::HasAnswered => Some("has answered"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::record::LocalRecord;
    use smoozy_security_and_licence::held::HeldSecret;
    use smoozy_security_and_licence::plaintext::Plaintext;
    use smoozy_security_and_licence::secret_store::{GetFailure, ProbeOutcome, StoreFailure};
    use std::cell::Cell;
    use std::path::{Path, PathBuf};

    /// A store that answers the probe as it was told to, and counts.
    struct AStore {
        answers: ProbeOutcome,
        probes: Cell<usize>,
    }

    impl AStore {
        fn answering(answers: ProbeOutcome) -> Self {
            AStore {
                answers,
                probes: Cell::new(0),
            }
        }
    }

    impl SecretStore for AStore {
        fn put(&self, _: &HeldSecret, _: Plaintext, _: Duration) -> Result<(), StoreFailure> {
            Err(StoreFailure::StoreUnavailable)
        }
        fn get(&self, _: &HeldSecret, _: Duration) -> Result<Plaintext, GetFailure> {
            Err(GetFailure::NotThere)
        }
        fn erase(&self, _: &HeldSecret, _: Duration) -> Result<(), StoreFailure> {
            Err(StoreFailure::StoreUnavailable)
        }
        fn probe(&self, _: Duration) -> ProbeOutcome {
            self.probes.set(self.probes.get() + 1);
            self.answers
        }
        fn enumerate(&self, _: Duration) -> Result<Vec<HeldSecret>, StoreFailure> {
            Ok(Vec::new())
        }
    }

    fn scratch(name: &str) -> PathBuf {
        let at = std::env::temp_dir().join(format!("smoozy-standing-{name}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&at);
        std::fs::create_dir_all(&at).expect("the area");
        at
    }

    fn a_record(area: &Path) -> LocalRecord {
        LocalRecord::open_at(&area.join("record.sqlite3")).expect("the record opens")
    }

    /// Plants a standing the way a previous run would have left it, and the
    /// absence of a row is *never probed*.
    fn planted(record: &LocalRecord, standing: StoreStanding) {
        record
            .connection()
            .execute("DELETE FROM store_standing", [])
            .expect("the row goes");
        if let Some(text) = the_text_for(standing) {
            record
                .connection()
                .execute(
                    "INSERT INTO store_standing (only_row, standing) VALUES (1, ?1)",
                    [text],
                )
                .expect("the standing");
        }
    }

    fn read_back(record: &LocalRecord) -> Option<StoreStanding> {
        crate::surface::set_up::the_store_standing(record.connection())
    }

    /// One transition driven through this writer, from a standing a previous
    /// run left, in a fresh run.
    fn a_run(
        record: &LocalRecord,
        from: StoreStanding,
        outcome: ProbeOutcome,
    ) -> (StandingWrite, Option<StoreStanding>) {
        planted(record, from);
        let lent = crate::keys::TheRecordItself::over(record.connection());
        let store = AStore::answering(outcome);
        let this_run = TheProbeThisRun::new();
        let write = at_the_first_use_of_a_secret(&this_run, &lent, &store, Duration::from_secs(5))
            .expect("the standing read, so the probe was taken");
        assert_eq!(store.probes.get(), 1, "the probe was taken once");
        (write, read_back(record))
    }

    /// **`Security and licence` §3.3 S32's first transition, through the
    /// writer.** Never probed, and the first probe of this installation fails.
    ///
    /// **This is the one that carries S34's notice** — *never probed* to *never
    /// answered*, once for the life of the installation.
    #[test]
    fn never_probed_then_a_failure_writes_never_answered_and_says_it_once() {
        let area = scratch("first-failure");
        let record = a_record(&area);

        let (write, held) = a_run(&record, StoreStanding::NeverProbed, ProbeOutcome::RoundTripFailed);

        assert_eq!(write.standing, StoreStanding::NeverAnswered);
        assert!(write.say_it_once, "S34's notice is the first transition's");
        assert_eq!(held, Some(StoreStanding::NeverAnswered), "and it is durable");

        let _ = std::fs::remove_dir_all(&area);
    }

    /// **A later failure on a computer already known to have nowhere to
    /// remember.** The standing holds and the notice is not said again.
    #[test]
    fn never_answered_then_a_failure_holds_and_says_nothing() {
        let area = scratch("again-failure");
        let record = a_record(&area);

        let (write, held) =
            a_run(&record, StoreStanding::NeverAnswered, ProbeOutcome::RoundTripFailed);

        assert_eq!(write.standing, StoreStanding::NeverAnswered);
        assert!(!write.say_it_once, "S34: not once per run");
        assert_eq!(held, Some(StoreStanding::NeverAnswered));

        let _ = std::fs::remove_dir_all(&area);
    }

    /// **A user who installed a keyring between two sittings** — which is S26's
    /// own reason for re-taking the probe every run rather than caching it.
    #[test]
    fn never_answered_then_a_success_moves_the_standing_for_good() {
        let area = scratch("keyring-arrived");
        let record = a_record(&area);

        let (write, held) =
            a_run(&record, StoreStanding::NeverAnswered, ProbeOutcome::RoundTripWorked);

        assert_eq!(write.standing, StoreStanding::HasAnswered);
        assert!(!write.say_it_once);
        assert_eq!(held, Some(StoreStanding::HasAnswered));

        let _ = std::fs::remove_dir_all(&area);
    }

    /// **S25 and S35: a store that answered once is never re-classified as
    /// nowhere by any number of failures**, and the cost of that is named
    /// rather than repaired — a user who removes their keyring never meets the
    /// once-only notice.
    #[test]
    fn has_answered_then_a_failure_never_returns_to_never_answered() {
        let area = scratch("keyring-removed");
        let record = a_record(&area);

        for _ in 0..5 {
            let (write, held) =
                a_run(&record, StoreStanding::HasAnswered, ProbeOutcome::RoundTripFailed);
            assert_eq!(write.standing, StoreStanding::HasAnswered);
            assert!(!write.say_it_once);
            assert_eq!(held, Some(StoreStanding::HasAnswered));
        }

        let _ = std::fs::remove_dir_all(&area);
    }

    /// **S26's second half, within the run: taken once and not twice.**
    ///
    /// **It fails against the obvious wrong answer** — a writer that probes at
    /// every first use — and that answer is not merely wasteful: on Linux a
    /// probe is a write, a read and an erase against the user's keyring, and
    /// every one of them may prompt.
    #[test]
    fn the_probe_is_taken_once_in_a_run_and_not_twice() {
        let area = scratch("once");
        let record = a_record(&area);
        planted(&record, StoreStanding::NeverProbed);

        let lent = crate::keys::TheRecordItself::over(record.connection());
        let store = AStore::answering(ProbeOutcome::RoundTripFailed);
        let this_run = TheProbeThisRun::new();

        let first = at_the_first_use_of_a_secret(&this_run, &lent, &store, Duration::from_secs(5))
            .expect("the first use");
        assert!(first.say_it_once);

        for _ in 0..4 {
            let again =
                at_the_first_use_of_a_secret(&this_run, &lent, &store, Duration::from_secs(5))
                    .expect("a later use in the same run");
            assert_eq!(again.standing, StoreStanding::NeverAnswered);
            assert!(
                !again.say_it_once,
                "**S34: once for the life of the installation**, so the second \
                 caller in one run is never told to say it"
            );
        }

        assert_eq!(
            store.probes.get(),
            1,
            "**S26: the probe is not re-taken again within one run**"
        );

        let _ = std::fs::remove_dir_all(&area);
    }

    /// **A standing nobody could read is not a fresh installation.**
    ///
    /// Nothing is probed and nothing is written, and the run is left untaken so
    /// the next first use tries again. Written the other way round, one failed
    /// query on a computer whose keyring works would say *there is nowhere to
    /// remember* and say it for the life of the installation.
    #[test]
    fn a_standing_that_will_not_read_probes_nothing_and_writes_nothing() {
        let area = scratch("unreadable");
        let record = a_record(&area);
        record
            .connection()
            .execute("DELETE FROM store_standing", [])
            .expect("the row goes");
        record
            .connection()
            .execute(
                "INSERT INTO store_standing (only_row, standing) VALUES (1, ?1)",
                ["a reading this binary does not know"],
            )
            .expect("the planted value");

        let lent = crate::keys::TheRecordItself::over(record.connection());
        let store = AStore::answering(ProbeOutcome::RoundTripFailed);
        let this_run = TheProbeThisRun::new();

        assert!(
            at_the_first_use_of_a_secret(&this_run, &lent, &store, Duration::from_secs(5))
                .is_none()
        );
        assert_eq!(store.probes.get(), 0, "no probe was taken");

        // And the value it could not read is untouched.
        let held: String = record
            .connection()
            .query_row("SELECT standing FROM store_standing WHERE only_row = 1", [], |row| {
                row.get(0)
            })
            .expect("the row still stands");
        assert_eq!(held, "a reading this binary does not know");

        // The run was not marked, so the next first use tries again.
        planted(&record, StoreStanding::NeverProbed);
        let taken = at_the_first_use_of_a_secret(&this_run, &lent, &store, Duration::from_secs(5))
            .expect("the standing reads now");
        assert_eq!(taken.standing, StoreStanding::NeverAnswered);

        let _ = std::fs::remove_dir_all(&area);
    }

    /// **The reader and this writer are held together**, because the spelling
    /// of a reading is a fact and a fact kept in two places is the fault this
    /// build stage keeps paying for.
    #[test]
    fn every_reading_this_writer_writes_is_one_the_reader_reads_back() {
        for standing in [
            StoreStanding::NeverProbed,
            StoreStanding::NeverAnswered,
            StoreStanding::HasAnswered,
        ] {
            match the_text_for(standing) {
                // *Never probed* is the value's absence, so the reader meets it
                // as a row that is not there.
                None => assert_eq!(standing, StoreStanding::NeverProbed),
                Some(text) => assert_eq!(
                    crate::surface::set_up::the_standing_from(Ok(text.to_string())),
                    Some(standing),
                    "the writer writes {text:?} and the reader does not read it back"
                ),
            }
        }
    }

    /// **The other half of S26, and it is the half a later agent breaks without
    /// noticing: the probe is never taken at application start.**
    ///
    /// **Why the test is over the shape.** Application start on this computer
    /// opens the Creator's own record and reaches the Creator's own keychain,
    /// so a test that ran it would be the one thing this build step has been
    /// careful about all along. What can be asserted without running it is that
    /// no file this crate loads at start can reach this module at all: the walk
    /// below names every file that calls in, and `src/start.rs` is not among
    /// them.
    ///
    /// **It fails the moment somebody adds a call at start**, which is the
    /// whole point — reaching the store may prompt, and at start nothing is
    /// being handed over.
    #[test]
    fn the_probe_is_not_taken_at_application_start() {
        /// Every file that may take the probe, with what makes each of them a
        /// moment when a secret is actually needed.
        const THE_FIRST_USE_MOMENTS: &[&str] = &[
            // A passphrase put at a generation — `Keys` G5.
            "src/keys/making.rs",
            // A passphrase put at the box's answer — `Keys` S17.
            "src/keys/supply.rs",
            // The get for a connection's credential, and `Servers` N8b's
            // question about whether a refusal can ever change.
            "src/keys/for_a_connection.rs",
            // `Security and licence` S36: the machine-password offer is
            // composed after the probe.
            "src/surface/set_up.rs",
        ];

        let root = Path::new(env!("CARGO_MANIFEST_DIR"));
        let mut calls = Vec::new();
        for (at, text) in every_rust_file_under(root) {
            let relative = beneath_the_crate(root, &at);
            if relative == "src/the_store_standing.rs" || relative == "src/lib.rs" {
                continue;
            }
            let working = &text[..text.find("#[cfg(test)]").unwrap_or(text.len())];
            if working.contains("the_store_standing::at_the_first_use_of_a_secret")
                || working.contains("at_the_first_use_of_a_secret(")
            {
                calls.push(relative);
            }
        }
        calls.sort();

        assert!(
            !calls.iter().any(|at| at == "src/start.rs"),
            "**`Security and licence` S26: the probe is taken at the first \
             moment in a run when a secret is actually needed, and never at \
             application start.** Reaching the store may prompt, and at start \
             nothing is being handed over."
        );

        let mut expected: Vec<String> = THE_FIRST_USE_MOMENTS
            .iter()
            .map(|at| at.to_string())
            .collect();
        expected.sort();
        assert_eq!(
            calls, expected,
            "a file took the probe that this test does not name as a moment \
             when a secret is actually needed. **Add it here with what makes it \
             one, or move the call** — S26 is the clause, and `src/start.rs` is \
             the one place it can never go."
        );
    }

    /// A walked file's name as this test writes one: the part below the
    /// crate's own folder, with a forward slash between the parts on every
    /// system.
    ///
    /// **The parts are put back together rather than the front of the text cut
    /// off, and that is the whole of this function.** Cutting the text
    /// `"<folder>/"` off the front reads the same at this desk and on Linux and
    /// is wrong on Windows, where the separator between the parts is a
    /// backslash and the cut never matches. **What a cut that missed left
    /// behind was the whole path from the top of the disk**, which then loses
    /// every comparison against a name like `src/start.rs`, so the guard fired
    /// and told Windows a file had taken the probe — which was never true. The
    /// desk and the hosted Linux machine could not see it, and only the hosted
    /// Windows one could. Found on 2026-09-17 by the hosted run of that day.
    ///
    /// **Nothing can be handed in that is not below the folder**, the walk
    /// having started there, so a part that will not come off is this function
    /// being given something it was never meant to have and it stops rather
    /// than quietly handing back a path from the top of the disk. That is the
    /// half that keeps the misleading message from ever coming back: the test
    /// can no longer fail as though it had read something it did not read.
    fn beneath_the_crate(root: &Path, at: &Path) -> String {
        at.strip_prefix(root)
            .expect("the walk hands back only files below the crate's own folder")
            .components()
            .map(|part| part.as_os_str().to_string_lossy().into_owned())
            .collect::<Vec<_>>()
            .join("/")
    }

    /// Every `.rs` file of this crate, as (path, text) — the same walk
    /// `crate::markers` makes, and for the same reason: a rule that reads one
    /// level cannot fail on a file it never read.
    ///
    /// **It hands back the path itself rather than the path written out as
    /// text**, because the caller has to take the crate's folder off the front
    /// of it and that is a thing to do to a path.
    fn every_rust_file_under(at: &Path) -> Vec<(PathBuf, String)> {
        let mut out = Vec::new();
        for entry in std::fs::read_dir(at).expect("the folder is readable") {
            let path = entry.expect("the entry is readable").path();
            if path.is_dir() {
                if path.file_name().and_then(|s| s.to_str()) == Some("target") {
                    continue;
                }
                out.extend(every_rust_file_under(&path));
            } else if path.extension().and_then(|e| e.to_str()) == Some("rs") {
                let text = std::fs::read_to_string(&path).expect("the file is readable");
                out.push((path, text));
            }
        }
        out
    }
}
