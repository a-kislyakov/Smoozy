//! **§9.3 D7 and D8 performed — the destination probe's two writes, in order,
//! at its cadence.**
//!
//! The arrangement's own staging directory created inside the destination
//! folder where it is absent, then one real write of at least one byte into
//! it, **both removed at once where this probe made the directory and the byte
//! alone where it already stood**, repeated at a pause of 60 seconds measured
//! from the end of the previous probe, **for as long as the condition stands,
//! indefinitely.**
//!
//! # The writes are the adapter's and the sequence is this document's
//!
//! **`Desktop app and delivery` F1 to F9 own what a local write comes to**,
//! and D1 consumes those classes rather than re-deriving them: *"nothing in
//! this crate produces one from an error number or a symbol"*. So
//! [`WritesTheProbe`] hands back
//! [`crate::destination::WhatTheAdapterSaid`] and never an error of its own,
//! and **nothing in this module classifies anything.**
//!
//! **That adapter cannot be reached from here** — its crate depends on the
//! feature modules and supplies their adapters — **and it implements this
//! seam for nothing**, so [`WritesTheProbe`] has no producer in this
//! workspace. *(Corrected 2026-09-07: this said the adapter was unbuilt for
//! these acts. F1 to F5, F7 and F9 are built in that crate at its
//! `platform/files.rs`; what is missing is the implementation of this seam
//! over them, and F6 and F8 are named by nothing there.)* What is built
//! here is the whole sequence around it: the order, the removal that runs
//! whatever the write did, the reading of the outcome, and the cadence.
//!
//! # Why the creation comes first, and it is not tidiness
//!
//! **Nothing may be assumed to have made the staging directory.** No clause of
//! `Server protocol and transfers` §7 names an actor or a moment for creating
//! one, and D6 forbids the only act this document names as making one —
//! while the condition stands nothing is arranged. **Two permitted starting
//! states reach the same dead end without the creation**: a user switches
//! auto-download on against a destination that is already full or unwritable,
//! so no transfer was ever arranged and no directory was ever made; or the
//! destination folder is deleted or its drive detached, taking the staging
//! directory inside it with it. **In either state a probe that only wrote into
//! a directory that was not there would fail for a reason unrelated to the
//! condition**, D5's withdrawal would never be met, and the arrangement would
//! probe at one byte a minute for as long as the app runs, with no ceiling,
//! no growing pause, no terminal state and no recovery.
//!
//! # The removal runs whatever happened, and it removes only what this probe
//! made
//!
//! A probe that made a directory and then could not write into it has still
//! made a directory. **So the removal is on every path out**, including the
//! failing one, and the arrangement leaves nothing behind at a destination it
//! cannot use. **A removal that itself refuses is not read as anything**: the
//! probe's answer is D7's two acts and the removal is neither of them.
//!
//! **A staging directory that was already standing is left standing**, which
//! is the clause D7 gained on 2026-09-06 out of this module's own code round.
//! Such a directory may hold the staged material of a transfer interrupted
//! before the condition was raised — `Server protocol and transfers` S26's
//! third row keeps it with its checkpoints, M10 quotes that as the whole of
//! Row 209's carry-on, and M13a reads it as owned staging. **The byte the
//! probe wrote is removed either way**, so the probe still leaves behind at
//! most what it found and D7's cost line is unchanged.
//!
//! # No amount decides anything, here least of all
//!
//! **D3: a full volume never reads zero** — 1,068 KB on the genuinely full
//! one — so no threshold exists, and **only a write attempt establishes that a
//! refusal is over** (`Desktop app and delivery` F5, F6). **No free-space
//! reading is taken anywhere in this module and no figure is consulted.**
//!
//! **Markers used in this file that belong to another document, each with the
//! document it belongs to.** `Desktop app and delivery` F1, F5, F6, F7, F8, F9,
//! Q8.
//! `Server protocol and transfers` §7, C16, C17, S1, S28. `State, identity and
//! recovery` §1.3.

use std::time::Duration;

use crate::destination::{
    what_the_probe_said, TheProbesStep, WhatTheAdapterSaid, WhatTheCreationDid, WhatTheProbeSaid,
    THE_PROBE, THE_PROBES_PAUSE, THE_PROBES_WRITE_IS_AT_LEAST_THIS_MANY_BYTES,
};

/// **D7's two acts, and the removal that follows them, on the user's own
/// disk.**
///
/// **The far side is `Desktop app and delivery`'s platform file adapter** and
/// every answer is that document's own outcome class, never an error this
/// crate would have to classify. **No implementation of this trait exists
/// outside this crate's own tests** ([`crate::arriving_later`]).
pub trait WritesTheProbe {
    /// **The arrangement's own staging directory created inside the
    /// destination folder where it is absent.** A directory that already
    /// stands is the ordinary case and is not a failure — the probe's question
    /// is whether the path the next transfer needs can be staged into.
    ///
    /// **It says which of the two it did**, because the removal turns on it:
    /// only a directory this probe made is removed with the byte.
    fn create_the_staging_directory_where_it_is_absent(
        &mut self,
        the_staging_directory: &[u8],
    ) -> Result<WhatTheCreationDid, WhatTheAdapterSaid>;

    /// **One real write of at least one byte into it**, because on a full
    /// filesystem the writability test returns writable and creating an empty
    /// file succeeds. The path is inside the staging directory and is the
    /// probe's own.
    fn write_at_least_one_byte(
        &mut self,
        the_probes_own_path: &[u8],
        bytes: &[u8],
    ) -> Result<(), WhatTheAdapterSaid>;

    /// **What the probe made, removed at once** — and a refusal here is not the
    /// probe's answer.
    ///
    /// **The byte the probe wrote is removed always.** The staging directory
    /// is named only where this probe created it; where it already stood it is
    /// absent from the call and is left standing, because it may hold the
    /// staged material of an interrupted transfer.
    fn remove_what_the_probe_made(
        &mut self,
        the_probes_own_path: &[u8],
        the_staging_directory_the_probe_created: Option<&[u8]>,
    );
}

/// **The probe's own file inside the staging directory.**
///
/// It is named from the destination's staging directory and a fixed stem of
/// Smoozy's own, so **it collides with no transfer's staging path**: those are
/// synthesised from a transfer identity (`Server protocol and transfers` S4),
/// which this name is not and can never be mistaken for.
pub fn the_probes_own_path(the_staging_directory: &[u8]) -> Vec<u8> {
    let mut out = the_staging_directory.to_vec();
    if !out.ends_with(b"/") {
        out.push(b'/');
    }
    out.extend_from_slice(b"probe.smoozy-probe");
    out
}

/// **The one byte the real write writes.**
///
/// **D7 asks for at least one and this writes exactly one**, because the point
/// is that a byte reaches the device and not how many do — and one byte is
/// what makes the cadence *priced so that paying it indefinitely is nothing*.
const THE_ONE_BYTE: [u8; THE_PROBES_WRITE_IS_AT_LEAST_THIS_MANY_BYTES] = [0];

/// **D7 and D8 performed: one probe, its three steps in order, and what it
/// said.**
///
/// **The steps are [`THE_PROBE`]'s and are walked rather than written out**,
/// so the order is that constant's and there is not a second copy of it here.
///
/// **The probe exercises the whole path the next transfer needs** — the
/// destination folder's existence, its writability, and the volume's room —
/// **and cannot pass on a state that cannot stage.**
pub fn one_probe<W: WritesTheProbe + ?Sized>(
    writer: &mut W,
    the_staging_directory: &[u8],
) -> WhatTheProbeSaid {
    let probe_path = the_probes_own_path(the_staging_directory);
    let mut what_the_creation_did = None;
    let mut the_real_write_succeeded = false;
    for step in THE_PROBE {
        match step {
            TheProbesStep::CreateTheStagingDirectoryWhereItIsAbsent => {
                what_the_creation_did = writer
                    .create_the_staging_directory_where_it_is_absent(the_staging_directory)
                    .ok();
            }
            TheProbesStep::WriteAtLeastOneByteIntoIt => {
                // **The write is not attempted where the directory is not
                // there**: it would fail for a reason unrelated to the
                // condition, which is the dead end the creation step exists to
                // prevent.
                if what_the_creation_did.is_some() {
                    the_real_write_succeeded = writer
                        .write_at_least_one_byte(&probe_path, &THE_ONE_BYTE)
                        .is_ok();
                }
            }
            // **On every path out**, so a probe that made a directory and then
            // could not write into it leaves nothing behind.
            //
            // **And it removes what this probe made and nothing else.** A
            // staging directory that was already standing is left standing:
            // it may hold the staged material of a transfer interrupted before
            // the condition was raised, which `Server protocol and transfers`
            // S26's third row keeps with its checkpoints and which M10 quotes
            // as the whole of Row 209's carry-on. The byte goes either way.
            TheProbesStep::RemoveWhatTheProbeMade => {
                let made_by_this_probe = matches!(
                    what_the_creation_did,
                    Some(WhatTheCreationDid::ItMadeTheDirectory)
                );
                writer.remove_what_the_probe_made(
                    &probe_path,
                    made_by_this_probe.then_some(the_staging_directory),
                );
            }
        }
    }
    let the_directory_was_made_or_already_stood = what_the_creation_did.is_some();
    what_the_probe_said(
        the_directory_was_made_or_already_stood,
        the_real_write_succeeded,
    )
}

/// **D7's cadence, held as the state one arrangement's probing needs between
/// two probes.**
///
/// **There is no clock, no scheduler and no registration here**, the same way
/// there is none in the watch: what this answers is how long to wait and
/// whether to probe at all, and the waiting is the caller's inside the
/// sequence it is already running.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct TheProbesCadence {
    the_condition_stands: bool,
}

/// What one turn of the cadence came to.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhatTheProbesTurnCameTo {
    /// **No condition stands, so nothing is probed.** The probe exists for a
    /// condition and for nothing else.
    NoConditionStandsSoNothingWasProbed,
    /// A probe ran, with what it said and how long to pause before the next.
    ItRan {
        said: WhatTheProbeSaid,
        pause_before_the_next: Duration,
    },
}

impl TheProbesCadence {
    pub fn fresh() -> TheProbesCadence {
        TheProbesCadence::default()
    }

    /// **D4 and D5: the condition is raised by a refusal and withdrawn by the
    /// next real write to that place succeeding, and by nothing else.**
    ///
    /// **Not a recovered free-space reading** — that is one signal where
    /// `Server protocol and transfers` F11 takes two, and the divergence is
    /// declared at D5 with its reason: on this side the free-space half is
    /// forbidden.
    pub fn a_condition_was_raised(&mut self) {
        self.the_condition_stands = true;
    }

    /// Whether the condition stands, which is what the queue's own hold reads.
    pub fn a_condition_stands(&self) -> bool {
        self.the_condition_stands
    }

    /// **One turn: a probe where the condition stands, and its outcome
    /// applied.**
    ///
    /// **D8: on a probe that succeeds the condition is withdrawn**, and the
    /// arrangement re-arranges everything written while it could not — the
    /// reckoning still names the owed units, the walk kept running, and
    /// nothing was lost in the meantime.
    ///
    /// **D9: nothing switches the arrangement off** — not the condition, not
    /// its repetition, not the suppression — **and there is no attempt
    /// ceiling, no growing pause and no terminal state.** This turn returns
    /// the same pause for ever, which is what *indefinitely* means.
    pub fn one_turn<W: WritesTheProbe + ?Sized>(
        &mut self,
        writer: &mut W,
        the_staging_directory: &[u8],
    ) -> WhatTheProbesTurnCameTo {
        if !self.the_condition_stands {
            return WhatTheProbesTurnCameTo::NoConditionStandsSoNothingWasProbed;
        }
        let said = one_probe(writer, the_staging_directory);
        if matches!(said, WhatTheProbeSaid::ItSucceededSoTheConditionIsWithdrawn) {
            self.the_condition_stands = false;
        }
        WhatTheProbesTurnCameTo::ItRan {
            said,
            pause_before_the_next: THE_PROBES_PAUSE,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Default)]
    struct ADisk {
        the_directory_refuses: Option<WhatTheAdapterSaid>,
        the_write_refuses: Option<WhatTheAdapterSaid>,
        /// **The directory was already standing when the probe asked**, which
        /// is the case a transfer interrupted before the condition was raised
        /// leaves behind.
        the_directory_already_stood: bool,
        made: Vec<Vec<u8>>,
        written: Vec<(Vec<u8>, usize)>,
        removed: Vec<Vec<u8>>,
    }

    impl WritesTheProbe for ADisk {
        fn create_the_staging_directory_where_it_is_absent(
            &mut self,
            the_staging_directory: &[u8],
        ) -> Result<WhatTheCreationDid, WhatTheAdapterSaid> {
            if let Some(said) = self.the_directory_refuses {
                return Err(said);
            }
            if self.the_directory_already_stood {
                return Ok(WhatTheCreationDid::ItAlreadyStood);
            }
            self.made.push(the_staging_directory.to_vec());
            Ok(WhatTheCreationDid::ItMadeTheDirectory)
        }
        fn write_at_least_one_byte(
            &mut self,
            the_probes_own_path: &[u8],
            bytes: &[u8],
        ) -> Result<(), WhatTheAdapterSaid> {
            if let Some(said) = self.the_write_refuses {
                return Err(said);
            }
            self.written
                .push((the_probes_own_path.to_vec(), bytes.len()));
            Ok(())
        }
        fn remove_what_the_probe_made(
            &mut self,
            the_probes_own_path: &[u8],
            the_staging_directory_the_probe_created: Option<&[u8]>,
        ) {
            self.removed.push(the_probes_own_path.to_vec());
            if let Some(directory) = the_staging_directory_the_probe_created {
                self.removed.push(directory.to_vec());
            }
        }
    }

    const STAGING: &[u8] = b"/Users/a/Downloads/job/.smoozy-staging";

    /// **D7: the creation first, then a real write of at least one byte, then
    /// both removed at once** — in that order and with nothing left behind.
    #[test]
    fn the_probe_creates_then_writes_then_removes_both() {
        let mut disk = ADisk::default();
        let said = one_probe(&mut disk, STAGING);

        assert_eq!(said, WhatTheProbeSaid::ItSucceededSoTheConditionIsWithdrawn);
        assert_eq!(disk.made, vec![STAGING.to_vec()]);
        assert_eq!(disk.written.len(), 1);
        assert!(disk.written[0].1 >= THE_PROBES_WRITE_IS_AT_LEAST_THIS_MANY_BYTES);
        assert_eq!(disk.removed.len(), 2);
    }

    /// **D7: a staging directory that was already standing is left standing,
    /// and only the byte is removed.**
    ///
    /// The failure path this closes: a 3.9 GB transfer is interrupted by a
    /// full volume, its staged material is kept with its checkpoints for the
    /// resume to carry on from, and a minute later the probe runs. A probe
    /// that removed the directory either way would remove those bytes — and
    /// the user was promised a part-sent file that can be carried on from.
    #[test]
    fn a_staging_directory_that_already_stood_is_left_standing_and_only_the_byte_goes() {
        let mut disk = ADisk {
            the_directory_already_stood: true,
            ..ADisk::default()
        };
        let said = one_probe(&mut disk, STAGING);

        // The probe still passes: the path the next transfer needs can be
        // staged into, which is the whole question it asks.
        assert_eq!(said, WhatTheProbeSaid::ItSucceededSoTheConditionIsWithdrawn);
        assert!(disk.made.is_empty());
        assert_eq!(disk.removed, vec![the_probes_own_path(STAGING)]);
        assert!(
            !disk.removed.contains(&STAGING.to_vec()),
            "the probe removed a staging directory it did not make"
        );
    }

    /// **A directory the probe made itself is removed with the byte**, so a
    /// probe leaves behind at most what it found and D7's cost line is
    /// unchanged.
    #[test]
    fn a_staging_directory_the_probe_made_is_removed_with_the_byte() {
        let mut disk = ADisk::default();
        one_probe(&mut disk, STAGING);
        assert_eq!(disk.made, vec![STAGING.to_vec()]);
        assert_eq!(
            disk.removed,
            vec![the_probes_own_path(STAGING), STAGING.to_vec()]
        );
    }

    /// **And it holds on the failing path too**: a probe that found the
    /// directory standing and then could not write into it still leaves that
    /// directory alone.
    #[test]
    fn a_refused_write_never_takes_a_standing_staging_directory_with_it() {
        let mut disk = ADisk {
            the_directory_already_stood: true,
            the_write_refuses: Some(WhatTheAdapterSaid::TheVolumeIsFull),
            ..ADisk::default()
        };
        assert_eq!(
            one_probe(&mut disk, STAGING),
            WhatTheProbeSaid::StillUnwritable
        );
        assert_eq!(disk.removed, vec![the_probes_own_path(STAGING)]);
    }

    /// **A real write and not a writability test**, because on a full
    /// filesystem the test returns writable and creating an empty file
    /// succeeds. A refused write is *still unwritable* however well the
    /// directory went.
    #[test]
    fn a_directory_that_stands_with_a_refused_write_is_still_unwritable() {
        let mut disk = ADisk {
            the_write_refuses: Some(WhatTheAdapterSaid::TheVolumeIsFull),
            ..ADisk::default()
        };
        assert_eq!(
            one_probe(&mut disk, STAGING),
            WhatTheProbeSaid::StillUnwritable
        );
    }

    /// **The removal runs on every path out**, including the one where the
    /// write refused — so the arrangement leaves nothing behind at a
    /// destination it cannot use.
    #[test]
    fn the_removal_runs_even_where_the_write_refused() {
        let mut disk = ADisk {
            the_write_refuses: Some(WhatTheAdapterSaid::TheVolumeIsFull),
            ..ADisk::default()
        };
        one_probe(&mut disk, STAGING);
        assert_eq!(disk.removed.len(), 2);
    }

    /// **Where the directory could not be made the write is not attempted**,
    /// because it would fail for a reason unrelated to the condition — which
    /// is the dead end the creation step exists to prevent.
    #[test]
    fn no_write_is_attempted_into_a_directory_that_is_not_there() {
        let mut disk = ADisk {
            the_directory_refuses: Some(WhatTheAdapterSaid::TheDestinationIsNoLongerThere),
            ..ADisk::default()
        };
        assert_eq!(
            one_probe(&mut disk, STAGING),
            WhatTheProbeSaid::StillUnwritable
        );
        assert!(disk.written.is_empty());
    }

    /// **The probe exists for a condition and for nothing else.**
    #[test]
    fn nothing_is_probed_where_no_condition_stands() {
        let mut cadence = TheProbesCadence::fresh();
        let mut disk = ADisk::default();
        assert_eq!(
            cadence.one_turn(&mut disk, STAGING),
            WhatTheProbesTurnCameTo::NoConditionStandsSoNothingWasProbed
        );
        assert!(disk.made.is_empty());
    }

    /// **D8: a probe that succeeds withdraws the condition. D9: nothing else
    /// ends the probing, there is no attempt ceiling and no growing pause**, so
    /// a destination that never recovers is probed at the same pause for ever.
    #[test]
    fn the_condition_is_withdrawn_by_a_successful_write_and_by_nothing_else() {
        let mut cadence = TheProbesCadence::fresh();
        cadence.a_condition_was_raised();

        let mut full = ADisk {
            the_write_refuses: Some(WhatTheAdapterSaid::TheVolumeIsFull),
            ..ADisk::default()
        };
        let mut pauses = Vec::new();
        for _ in 0..50 {
            match cadence.one_turn(&mut full, STAGING) {
                WhatTheProbesTurnCameTo::ItRan {
                    pause_before_the_next,
                    said,
                } => {
                    assert_eq!(said, WhatTheProbeSaid::StillUnwritable);
                    pauses.push(pause_before_the_next);
                }
                other => panic!("the probe stopped: {other:?}"),
            }
        }
        assert!(cadence.a_condition_stands());
        assert!(pauses.iter().all(|pause| *pause == THE_PROBES_PAUSE));

        let mut healthy = ADisk::default();
        match cadence.one_turn(&mut healthy, STAGING) {
            WhatTheProbesTurnCameTo::ItRan { said, .. } => {
                assert_eq!(said, WhatTheProbeSaid::ItSucceededSoTheConditionIsWithdrawn)
            }
            other => panic!("expected a probe, got {other:?}"),
        }
        assert!(!cadence.a_condition_stands());
    }

    /// **D3: no amount decides anything and no free-space reading is taken.**
    /// The rule is kept by there being no such call in this module at all.
    #[test]
    fn no_reading_of_an_amount_is_taken_anywhere_in_this_module() {
        let source = include_str!("probing.rs");
        for reading in [
            ["free", "_space"].concat(),
            ["available", "_bytes"].concat(),
        ] {
            assert!(
                !source.contains(&reading),
                "the probe reads an amount: {reading}"
            );
        }
    }

    /// **The probe's own file can never be mistaken for a transfer's staging
    /// path**, which is synthesised from a transfer identity.
    #[test]
    fn the_probes_own_file_sits_inside_the_staging_directory_and_names_no_transfer() {
        let path = the_probes_own_path(STAGING);
        assert!(path.starts_with(STAGING));
        assert!(crate::request::the_removal_may_run(&path, STAGING));
    }
}
