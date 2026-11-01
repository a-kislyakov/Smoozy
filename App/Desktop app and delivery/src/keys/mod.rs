//! The adapter side of `Keys` — the readings and the acts that touch this
//! computer.
//!
//! **`Keys` owns every judgement and this module owns none.** What is here is
//! how a reading is obtained and how an act reaches the operating system: the
//! filesystem's answers, the `ssh-keygen` runs, and the askpass helper that
//! carries a passphrase to its one endpoint.
//!
//! **It lives in this crate rather than in `Keys` because the adapters do.**
//! `Keys` §12 lists this document as a *use* rather than a supply — every local
//! write and deletion there goes through the platform file adapter, the
//! askpass helper of §9 carries both deliberate passphrase deliveries, and §10
//! is the only source of the two names the default comment is built from.

pub mod acts;
pub mod askpass;
/// **`Keys`' end of `Servers`' credential seam**, added at step 10b piece 2.
/// It is the one place in this module that another module's trait is
/// implemented, and it decides nothing: every judgement in it is
/// [`supply`]'s or `Servers`'.
pub mod for_a_connection;
pub mod making;
pub mod readings;
pub mod supply;

use std::sync::Mutex;

use smoozy_keys::diary::{Act, HowTheErasureEnded};
use smoozy_security_and_licence::erasure::{EraseMarks, Erasure};
use smoozy_state::identities::Identity;
use smoozy_state::outcomes::{Keeps, Recorded, Refused, UnitOutcome};

use crate::diary_service::ModulesDiary;
use crate::record::key_rows::KeyRowMarks;
use crate::record::outcome_records::OutcomeRecords;
use crate::start::Started;

/// The name `Keys` declares its diary lines under, and the one this module
/// writes with. **A module with no declaration writes nothing** (`Desktop app
/// and delivery` A7), so this string and `start`'s registration are one pair.
pub const THE_MODULE: &str = "Keys";

/// Hand one of `Keys`' four act lines to the diary service.
///
/// **`Keys` §12 declares the four acts and their fixed line forms; this is
/// where one is written.** No caller in this module constructed an `Act` or
/// handed one to the service at all, so every line §12 declared was a line
/// nothing wrote — and `Reporting a problem`'s own reason for the declaration
/// is that a module writing nowhere satisfies its own spec and leaves a hole
/// the Creator cannot tell from a thing that did not happen.
///
/// **The outcome goes nowhere and that is `Desktop app and delivery` L16's one
/// legitimate use of it**: a
/// refused write loses the entry and nothing else, the act it recorded proceeds
/// untouched, and no surface says anything because the user never meets the
/// diary.
pub fn write_the_line(diary: Option<&ModulesDiary<'_>>, act: &Act) {
    if let Some(diary) = diary {
        let wrote = diary.write(&act.line(diary.run.clone(), std::time::SystemTime::now()));
        debug_assert!(wrote.nothing_is_owed());
    }
}

/// How an erasure ended, as a line may state it — **confirmed, or left as
/// `Security and licence` §5.1's recorded debt for the sweep**, which is the
/// one thing about a replacement nothing afterwards would ever show (`Keys`
/// F10).
///
/// **Only where the row carried a secret-store subject reference**: where it
/// carried none there is nothing to lose, the field is absent, and an absence
/// is written as one (`Keys` F12).
pub fn how_the_erasure_ended(
    the_row_carried_a_reference: bool,
    erasure: &Erasure,
) -> Option<HowTheErasureEnded> {
    if !the_row_carried_a_reference {
        return None;
    }
    Some(match erasure {
        Erasure::Done => HowTheErasureEnded::Confirmed,
        Erasure::ReferenceGoneAndOwedToTheSweep(_)
        | Erasure::NothingDoneTheMarkWouldNotWrite => HowTheErasureEnded::LeftAsARecordedDebt,
    })
}

// ════════════════════════════════════════════════════════════════════════════
// The record, lent for one read or one write — the screens round's loose ends,
// L4, carrying out item 20 of step 10a round 1's Verdict for getting in,
// servers and keys
// ════════════════════════════════════════════════════════════════════════════

/// **The local record, lent for one read or one write and never for the length
/// of an act.**
///
/// **Why this exists.** Every act here used to be handed
/// `&rusqlite::Connection`, and on the desktop that borrow comes out of the
/// application-wide `Mutex<Started>` — so the lock was held for the whole act,
/// **`ssh-keygen` and the askpass wait included**. A key made with a passphrase
/// keeps the tool waiting on a helper the user has to answer, and every other
/// screen waits on the same lock behind it. Item 20's own words: *"take the
/// lock only around the record's reads and writes, never around a tool run."*
///
/// **So the connection is lent rather than held.** Each read and each write
/// below takes the lock, does its one thing, and gives it back; the tool runs
/// and the file work happen between them with nothing held.
///
/// **A record that is not there, or a lock that will not take, lends nothing**
/// and the act sees the read or the write refuse — the same answer it would
/// have had from a record that would not take the row.
pub trait LendsTheRecord {
    /// One read or one write against the local record.
    fn lend(&self, act: &mut dyn FnMut(&rusqlite::Connection));

    /// **One diary line, lent the same way.** The diary is reached through the
    /// spine as well, so building it is a read of the spine and not a reason to
    /// hold the lock across the act.
    fn write_a_diary_line(&self, act: &Act);

    /// **Another module's diary line, lent the same way** — added 2026-10-16,
    /// owed piece 11, for `Queue & History` §17's *untracked work taken over*,
    /// written by *Track it* through this lender. `compose` is handed the run
    /// identity, which every line carries (`Reporting a problem` U7), and the
    /// line goes through [`ModulesDiary`] under `module`, so `Desktop app and
    /// delivery` A7's hosting rule still decides. **A lender with no diary
    /// writes nothing and says so**: a test's record, which has none.
    fn write_a_modules_line(
        &self,
        module: &'static str,
        compose: &dyn Fn(Identity) -> smoozy_reporting::lines::Line,
    ) -> crate::diary_service::Wrote {
        let _ = (module, compose);
        crate::diary_service::Wrote::No { reported: "no diary is lent here".to_string() }
    }
}

/// **The run's own record, lent from the application-wide spine.**
///
/// This is the whole of L4 on the desktop: the lock is taken inside `lend` and
/// dropped before it returns, so it cannot be held across anything else.
pub struct TheRunsRecord<'a> {
    pub started: &'a Mutex<Started>,
}

impl LendsTheRecord for TheRunsRecord<'_> {
    fn lend(&self, act: &mut dyn FnMut(&rusqlite::Connection)) {
        let Ok(started) = self.started.lock() else {
            return;
        };
        let Some(record) = started.record.as_ref() else {
            return;
        };
        act(record.connection());
    }

    fn write_a_diary_line(&self, act: &Act) {
        let Ok(started) = self.started.lock() else {
            return;
        };
        let (Some(service), Some(run)) = (started.diary.as_ref(), started.run.as_ref()) else {
            return;
        };
        write_the_line(
            Some(&ModulesDiary {
                service,
                declarations: &started.declarations,
                run: run.clone(),
                module: THE_MODULE,
            }),
            act,
        );
    }

    fn write_a_modules_line(
        &self,
        module: &'static str,
        compose: &dyn Fn(Identity) -> smoozy_reporting::lines::Line,
    ) -> crate::diary_service::Wrote {
        let Ok(started) = self.started.lock() else {
            return crate::diary_service::Wrote::No { reported: "the spine's lock would not take".to_string() };
        };
        let (Some(service), Some(run)) = (started.diary.as_ref(), started.run.as_ref()) else {
            return crate::diary_service::Wrote::No { reported: "no diary or no run identity this run".to_string() };
        };
        ModulesDiary { service, declarations: &started.declarations, run: run.clone(), module }
            .write(&compose(run.clone()))
    }
}

/// **The record itself, with no lock over it** — for a caller that already
/// holds one open connection and nothing else is looking at it, which is what
/// every test in this module is.
pub struct TheRecordItself<'a> {
    pub connection: &'a rusqlite::Connection,
    pub diary: Option<&'a ModulesDiary<'a>>,
}

impl<'a> TheRecordItself<'a> {
    pub fn over(connection: &'a rusqlite::Connection) -> Self {
        TheRecordItself {
            connection,
            diary: None,
        }
    }

    pub fn with_the_diary(self, diary: &'a ModulesDiary<'a>) -> Self {
        TheRecordItself {
            diary: Some(diary),
            ..self
        }
    }
}

impl LendsTheRecord for TheRecordItself<'_> {
    fn lend(&self, act: &mut dyn FnMut(&rusqlite::Connection)) {
        act(self.connection);
    }

    fn write_a_diary_line(&self, act: &Act) {
        write_the_line(self.diary, act);
    }
}

/// **`State, identity and recovery` §9.4's keeper, over a lent record.**
///
/// P5's ordering is unmoved: the enumeration is durable before the first unit
/// and each outcome at the moment it becomes true. **What changes is only how
/// long the lock is held for each of those writes**, which is one write long.
pub struct TheOutcomeRecordsLent<'a>(pub &'a dyn LendsTheRecord);

/// What a lend that found no record says. It is the store's own refusal shape,
/// because to the act it is a write that did not go down.
fn nothing_to_write_on() -> Refused {
    Refused::TheStore("the local record is not open".to_string())
}

impl Keeps for TheOutcomeRecordsLent<'_> {
    fn enumerate(&mut self, action: &str, units: &[&str]) -> Result<Identity, Refused> {
        let mut out = Err(nothing_to_write_on());
        self.0.lend(&mut |connection| {
            out = OutcomeRecords::over(connection).enumerate(action, units);
        });
        out
    }

    fn a_unit_ended(
        &mut self,
        record: &Identity,
        unit: &str,
        outcome: UnitOutcome,
    ) -> Result<(), Refused> {
        let mut out = Err(nothing_to_write_on());
        self.0.lend(&mut |connection| {
            out = OutcomeRecords::over(connection).a_unit_ended(record, unit, outcome.clone());
        });
        out
    }

    fn read(&self, record: &Identity) -> Result<Option<Recorded>, Refused> {
        let mut out = Err(nothing_to_write_on());
        self.0.lend(&mut |connection| {
            out = OutcomeRecords::over(connection).read(record);
        });
        out
    }

    fn the_user_removed_it(&mut self, record: &Identity) -> Result<(), Refused> {
        let mut out = Err(nothing_to_write_on());
        self.0.lend(&mut |connection| {
            out = OutcomeRecords::over(connection).the_user_removed_it(record);
        });
        out
    }
}

/// **`Security and licence` §5.1's three writes, over a lent record.** The
/// ordering is untouched; each write takes the lock on its own.
pub struct TheMarksLent<'a>(pub &'a dyn LendsTheRecord);

impl EraseMarks for TheMarksLent<'_> {
    fn mark_owing_an_erase(
        &mut self,
        item: &smoozy_security_and_licence::held::HeldSecret,
    ) -> Result<(), smoozy_security_and_licence::erasure::RecordFailure> {
        let mut out = Err(smoozy_security_and_licence::erasure::RecordFailure);
        self.0.lend(&mut |connection| {
            out = KeyRowMarks(connection).mark_owing_an_erase(item);
        });
        out
    }

    fn remove_reference(
        &mut self,
        item: &smoozy_security_and_licence::held::HeldSecret,
    ) -> Result<(), smoozy_security_and_licence::erasure::RecordFailure> {
        let mut out = Err(smoozy_security_and_licence::erasure::RecordFailure);
        self.0.lend(&mut |connection| {
            out = KeyRowMarks(connection).remove_reference(item);
        });
        out
    }

    fn clear_reference_and_mark(
        &mut self,
        item: &smoozy_security_and_licence::held::HeldSecret,
    ) -> Result<(), smoozy_security_and_licence::erasure::RecordFailure> {
        let mut out = Err(smoozy_security_and_licence::erasure::RecordFailure);
        self.0.lend(&mut |connection| {
            out = KeyRowMarks(connection).clear_reference_and_mark(item);
        });
        out
    }

    fn marks(
        &self,
    ) -> Result<
        Vec<smoozy_security_and_licence::held::HeldSecret>,
        smoozy_security_and_licence::erasure::RecordFailure,
    > {
        let mut out = Err(smoozy_security_and_licence::erasure::RecordFailure);
        self.0.lend(&mut |connection| {
            out = KeyRowMarks(connection).marks();
        });
        out
    }
}

#[cfg(test)]
mod tests {
    /// One file of this module, or of the surface, as text.
    fn the_source(at: &str) -> String {
        std::fs::read_to_string(std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(at))
            .unwrap_or_else(|_| panic!("{at} is readable"))
    }

    /// The half of a file above its own tests.
    fn the_working_half(text: &str) -> String {
        text[..text.find("#[cfg(test)]").unwrap_or(text.len())].to_string()
    }

    /// **L4, and it is a shape rather than a stopwatch: no act here is handed
    /// the record for the length of the act.**
    ///
    /// **Why the test is over the shape.** Whether the lock is held across
    /// `ssh-keygen` is a question about a moment in time, and the only way to
    /// ask it of a running act is to have a second thread try for the lock
    /// while the tool runs — which is a race, and a test that passes or fails
    /// by how the scheduler felt is worse than none. **What can be asserted
    /// without a clock is that the act cannot hold it**: an act that is handed
    /// `&rusqlite::Connection` holds the borrow, and on the desktop that borrow
    /// comes out of the application-wide `Mutex<Started>` guard, so the guard
    /// lives as long as the act does. An act handed a
    /// [`LendsTheRecord`] cannot hold it at all — the lock is taken and given
    /// back inside `lend`.
    ///
    /// **It fails against the code as it stood on 2026-09-17**, where all three
    /// of these files took a connection for the whole act.
    #[test]
    fn no_act_here_is_handed_the_record_for_the_length_of_the_act() {
        for at in ["src/keys/making.rs", "src/keys/acts.rs", "src/keys/supply.rs"] {
            let working = the_working_half(&the_source(at));
            assert!(
                !working.contains("&rusqlite::Connection"),
                "{at} takes the record connection. **On the desktop that borrow \
                 comes out of the application-wide lock, so the lock is held \
                 for the whole act — `ssh-keygen` and the askpass wait \
                 included.** Take a `LendsTheRecord` instead: item 20 of step \
                 10a round 1's Verdict for getting in, servers and keys says \
                 *take the lock only around the record's reads and writes, \
                 never around a tool run*"
            );
        }
    }

    /// **And the surface's five acts lend the record rather than locking the
    /// spine around themselves.**
    ///
    /// **The screen's own reads still lock**, and rightly: they read the record
    /// and return. **These five run a tool**, and that is the whole difference.
    /// *(Six until 2026-09-18: the passphrase box's answer now opens the
    /// connect it was raised for, and the connect's own supply is what lends the
    /// record — `src/servers/a_period.rs`.)*
    #[test]
    fn every_surface_act_that_runs_a_tool_lends_the_record() {
        let text = the_source("src/surface/keys.rs");
        let working = the_working_half(&text);
        for act in [
            "fn the_key_is_made<",
            "fn the_point_is_taken<",
            "fn a_key_is_taken_away<",
            "fn a_remembered_passphrase_is_cleared<",
            "fn the_passphrase_is_asked_for<",
        ] {
            let at = working
                .find(act)
                .unwrap_or_else(|| panic!("{act} is gone from the surface"));
            let body = &working[at..];
            let ends = body[1..]
                .find("\n}\n")
                .map(|to| to + 3)
                .unwrap_or(body.len());
            let body = &body[..ends];
            assert!(
                body.contains("TheRunsRecord"),
                "{act} does not lend the record. **L4: the lock is never held \
                 around a tool run**"
            );
            assert!(
                !body.contains("started.lock()"),
                "{act} locks the spine and holds the guard across the act. \
                 **L4: lend the record for each read and each write instead**"
            );
        }
    }
}
