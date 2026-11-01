//! Making a key, performed — `Keys` §6, and the probe of `Keys` T2.
//!
//! **`Keys` decides and this file acts.** Every judgement here is a call into
//! `smoozy_keys`: the filename is composed by that module, the verification is
//! `Keys` G4's own function, and the units are §11's. What is here is the tool
//! run, the readings taken afterwards, and the four writes into `State,
//! identity and recovery` §9.4's outcome record.
//!
//! **Every local write and deletion goes through the platform file adapter**
//! (`Keys` §12, `Desktop app and delivery` F10a). The tool's own write does
//! not — it is somebody else's write, and F10a's second bullet governs it: the
//! destination is composed and checked through the adapter before the run, and
//! the outcome is established by re-reading that path through the adapter
//! after.
//!
//! **How a clause marker is written in this crate, and it is a rule rather than
//! a habit.** A marker with no document beside it means a clause of this crate's
//! own plan, `Preparation/Architecture/Desktop app and delivery.md`. Every other
//! document's marker is written with that document — at the site, or in this
//! file's own header list below. **Where a marker is a clause of this crate's
//! own plan *and* of the document meant, the header list is not enough and the
//! document is written out at the site**, because a bare one there resolves to a
//! real and wrong clause.
//!
//! **Markers used below that belong to another document, each with the document
//! it belongs to.** `Keys` G11, N1, U7, U7a, U7b, U9. `Security and licence`
//! B29.
//!
//! **A marker this crate's own plan also defines is deliberately not listed
//! above, whichever document is meant, and is written out with its document
//! at every site instead** — the header list cannot tell the two apart, and a
//! bare one would resolve to a real and wrong clause.
//!
//! **`Keys` G11 is written out at its site too**, because it stands beside this
//! crate's own G-clauses on the same line.

use std::path::{Path, PathBuf};
use std::time::Duration;

use smoozy_keys::container;
use smoozy_keys::diary::{AKeyMade, Act};
use smoozy_keys::fingerprint::Reading;
use smoozy_keys::generation::{self, Asked, Verified, WhatWasMade, WhyItFailed};
use smoozy_keys::locations;
use smoozy_keys::naming;
use smoozy_keys::rows::{KeyRow, Name, PassphraseReference};
use smoozy_keys::types::{KeyType, OfferedSet};
use smoozy_keys::units::{Action, UnitRole};
use smoozy_keys::usable::{self, Readings, ToolAnswer};
use smoozy_security_and_licence::held::{HeldSecret, StoredKind, Subject};
use smoozy_security_and_licence::plaintext::Origin;
use smoozy_security_and_licence::secret_store::SecretStore;
use smoozy_state::identities::{Identity, IdentityKind};
use smoozy_state::outcomes::{Refused, UnitOutcome, Writing};

use super::askpass;
use super::readings::{self, OnThisComputer};
use crate::platform::files;

/// **`Keys` T2: the offered set is a runtime reading of the installed binary,
/// never the manual's list.**
///
/// For each supported type: attempt a generation with `-N ""` and a throwaway
/// comment into Smoozy's own temporary area, verify a key pair resulted, and
/// **delete it at once**. A type whose attempt fails is not offered.
///
/// **The sweep is the area's, not this module's** — Smoozy's own temporary area
/// is swept at application start (`Desktop app and delivery` L3a) — so the
/// deletion here is the ordinary
/// tidy and the promise is kept at the other end even where a process died
/// mid-probe.
///
/// **`Keys` G6: the probe's failure to run at all doubles as the no-generator
/// detection**, which is why that answer is a state of this value rather than
/// something a caller has to remember to ask for.
pub fn probe_the_offered_set(temporary_area: &Path) -> OfferedSet {
    let mut offered = Vec::new();
    let mut the_tool_is_there = false;

    for kind in KeyType::SUPPORTED {
        // **A name of this probe's own.** `probe_<type>` was not even per
        // process, so two launches of Smoozy collided on it: one would find the
        // other's file at the target and read the tool's refusal as *this type
        // is unavailable*.
        let path = temporary_area.join(format!(
            "probe{}-{}_{}",
            std::process::id(),
            readings::next_scratch_number(),
            kind.as_str()
        ));
        // A leftover from a run that died mid-probe would make the tool refuse
        // an existing target and read as *this type is unavailable*.
        let _ = files::remove_file(&path);
        let companions = locations::companions(&path);
        let _ = files::remove_file(&companions.public_half);

        let answer = readings::run(&[
            "-t".to_string(),
            kind.as_str().to_string(),
            "-C".to_string(),
            "smoozy probe".to_string(),
            "-N".to_string(),
            String::new(),
            "-f".to_string(),
            path.display().to_string(),
        ]);
        if !matches!(answer, ToolAnswer::NoGenerator) {
            the_tool_is_there = true;
        }
        // **A pair resulted, established by re-reading both halves** rather
        // than by the exit status alone (`Desktop app and delivery` F10a's
        // second bullet). `Keys` T2 asks for a key **pair**, so a private half
        // alone is not the answer it wants.
        let there = |path: &Path| {
            !matches!(
                files::outside_program_wrote(path),
                files::Recheck::Absent | files::Recheck::CouldNotLook { .. }
            )
        };
        if matches!(answer, ToolAnswer::Printed(_))
            && there(&path)
            && there(&companions.public_half)
        {
            offered.push(kind);
        }

        // **Deleted at once**, whatever happened.
        let _ = files::remove_file(&path);
        let _ = files::remove_file(&companions.public_half);
    }

    if !the_tool_is_there {
        return OfferedSet::NoGenerator;
    }
    OfferedSet::Read(offered)
}

/// What the act was asked for, in the words the dialog collected them in.
pub struct TheAsk<'a> {
    pub key_location: PathBuf,
    pub key_type: KeyType,
    /// **Arbitrary text, passed to `-C` as one argument.** A comment is not a
    /// secret, so an argument is a permitted place for it.
    pub comment: String,
    /// **`Security and licence` B29's origin, where the user typed one.** The
    /// act reaches it twice — `Keys` G3's delivery to the generator, and
    /// `Keys` G5's
    /// put to the store — and it is discarded when the act ends however it
    /// ends.
    pub passphrase: Option<&'a Origin>,
    /// `Keys` G11's string as at the moment the dialog opened, for N1's
    /// recorded bit.
    pub the_default_name_now: String,
    /// **`Keys` G3's route**: Smoozy's own temporary area, where the channel
    /// is opened and where the one reading that needs a scratch file writes,
    /// and the program `SSH_ASKPASS` names.
    pub the_route: askpass::TheRoute,
}

/// How the key-pair unit ended.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ThePair {
    /// The pair is there and is what was asked for. `Keys` G4 agreed.
    Made { at: PathBuf },
    /// **`Keys` G4: the act is reported failed, the files it wrote are
    /// removed, and no row is created.**
    Failed(WhyItFailed),
    /// The tool never produced a pair — it refused, or there is none installed.
    TheToolProducedNothing,
    /// **A passphrase was asked for and no channel could be arranged**, so the
    /// act is refused rather than run without one. On Windows there is no
    /// channel at all (`Keys` §13 item 2) and this is the answer there.
    ThePassphraseCouldNotBeDelivered { why: askpass::CouldNotDeliver },
}

/// What the act came to. **The act's own outcome is not a field**: it is
/// derived from the record's unit outcomes (`State, identity and recovery` P5
/// item 3, at that document's §9.4).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Made {
    /// §9.4's record for this act.
    pub outcome_record: Identity,
    pub pair: ThePair,
    /// The row, where one was written.
    pub row: Option<Identity>,
    /// **`Keys` G5: a failed put costs the key nothing.** False here means the
    /// key is whole and the passphrase is simply not held, and the first
    /// connect asks.
    pub the_passphrase_is_held: bool,
    /// **`Security and licence` S34's sentence is owed**, this act having been
    /// the first in the run to take the probe. The new-key dialog's last step
    /// says it (`Preparation/Design/Keys.md`, *On a computer with nowhere to
    /// keep a passphrase*; step 10b's first audit round, item 7).
    pub the_sentence_is_owed: bool,
}

/// Why the act did not start. **Neither is a unit failing**: both happen before
/// the enumeration, so there is no record to report against.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NotRun {
    /// `State, identity and recovery` P5 item 1, at that document's §9.4, puts
    /// the enumeration
    /// before the first unit, so an act
    /// that could not write it does not start.
    TheOutcomeRecordCouldNotBeOpened(Refused),
    /// This computer offered no randomness for the key's own identifier.
    NoIdentityCouldBeIssued,
}

/// **`Keys` G1: generation is one act carrying four things**, performed.
///
/// **The record is lent for each read and each write and is never held across
/// the tool run** — step 10a, the screens round's loose ends, L4, which is item
/// 20 of that round's Verdict: *"take the lock only around the record's reads
/// and writes, never around a tool run."* `ssh-keygen` runs at
/// [`the_key_pair_unit`] below with nothing held, and the askpass wait that a
/// passphrase adds to it is the longest thing this act does.
pub fn generate(
    ask: &TheAsk<'_>,
    record_lender: &dyn super::LendsTheRecord,
    store: &dyn SecretStore,
    deadline: Duration,
    this_run: &crate::the_store_standing::TheProbeThisRun,
) -> Result<Made, NotRun> {
    let the_directory_is_needed = !ask.key_location.is_dir();
    let act = Action::Generation {
        the_directory_is_needed,
    };
    let mut records = super::TheOutcomeRecordsLent(record_lender);
    let mut record = Writing::open(&mut records, act.as_action(), &act.roles())
        .map_err(NotRun::TheOutcomeRecordCouldNotBeOpened)?;

    // **`Keys` G9: the directory, mode 0700, before the invocation** — macOS's
    // `ssh-keygen` does not create a missing parent.
    if the_directory_is_needed {
        let made = files::ensure_directory_at_mode(&ask.key_location, generation::THE_KEY_LOCATIONS_MODE);
        let _ = record.a_unit_ended(
            UnitRole::TheDirectory.as_role(),
            if made.is_ok() {
                UnitOutcome::Done
            } else {
                UnitOutcome::Failed
            },
        );
        if made.is_err() {
            // Nothing after it is safe without it (§9.2 test 3).
            let _ = record.the_rest_were_not_attempted(UnitRole::TheDirectory.as_role());
            return Ok(Made {
                outcome_record: record.identity().clone(),
                pair: ThePair::TheToolProducedNothing,
                row: None,
                the_passphrase_is_held: false,
                the_sentence_is_owed: false,
            });
        }
    }

    let pair = the_key_pair_unit(ask);
    let _ = record.a_unit_ended(
        UnitRole::TheKeyPair.as_role(),
        match pair {
            ThePair::Made { .. } => UnitOutcome::Done,
            _ => UnitOutcome::Failed,
        },
    );
    let ThePair::Made { at } = &pair else {
        // **`Keys` G4: no row is created**, and the store put has nothing to
        // belong to. Both are units the act never tries.
        let _ = record.the_rest_were_not_attempted(UnitRole::TheKeyPair.as_role());
        return Ok(Made {
            outcome_record: record.identity().clone(),
            pair,
            row: None,
            the_passphrase_is_held: false,
            the_sentence_is_owed: false,
        });
    };

    // **The row.** N1's bit is recorded at entry and never computed later.
    let identifier =
        Identity::issue(IdentityKind::LocalKey).map_err(|_| NotRun::NoIdentityCouldBeIssued)?;
    let name = Name::what_the_user_typed(&ask.comment);
    let row = KeyRow::entering(
        identifier.clone(),
        at.clone(),
        the_fingerprint_of(at, &ask.the_route.channel_area),
        name.clone(),
        naming::was_default_at_entry(name.as_ref(), &ask.the_default_name_now),
    );
    let written = match &row {
        Some(row) => {
            let mut went_down = false;
            record_lender.lend(&mut |connection| {
                went_down = super::super::record::key_rows::add(connection, row).is_ok();
            });
            went_down
        }
        None => false,
    };
    let _ = record.a_unit_ended(
        UnitRole::TheRow.as_role(),
        if written {
            UnitOutcome::Done
        } else {
            UnitOutcome::Failed
        },
    );
    if !written {
        let _ = record.the_rest_were_not_attempted(UnitRole::TheRow.as_role());
        return Ok(Made {
            outcome_record: record.identity().clone(),
            pair,
            row: None,
            the_passphrase_is_held: false,
            the_sentence_is_owed: false,
        });
    }

    // **§12 act one: a key made.** Written at the act's own outcome, **after
    // `Keys` G4's verification and the row creation** — not at the tool's
    // exit, where a pair failing verification has its files removed and no row
    // created, so a line there would name a key that does not exist; and not
    // at the store put, which is an independent later unit whose failure stops
    // nothing. **One line per generation, never one per unit of the four §11
    // declares.**
    record_lender.write_a_diary_line(
        &Act::AKeyMade(AKeyMade {
            key_type: ask.key_type,
            // **As the yes-or-no `Keys` G4 verified, and never as anything
            // about the value.**
            a_passphrase_was_applied: ask.passphrase.is_some(),
            the_path_the_act_wrote: at.clone(),
            // **The name the act passed to `-C` where the user typed one.**
            name: name.clone(),
        }),
    );

    // **`Keys` G5: the act's last unit puts the passphrase to the store, off
    // the act's critical path.** It is an independent unit — merely later, not
    // unsafe — so its failure is reported and stops nothing.
    let mut the_sentence_is_owed = false;
    let the_passphrase_is_held = match ask.passphrase {
        None => {
            // No passphrase was asked for, so there is nothing to remember and
            // the unit is done rather than skipped: the enumeration owed it,
            // and a unit left blank would read as one whose ending was lost.
            let _ = record.a_unit_ended(UnitRole::TheStorePut.as_role(), UnitOutcome::Done);
            false
        }
        Some(origin) => {
            // **`Security and licence` S26, at §3.3: the first moment in this run
            // when a secret is actually needed.** The user asked for this key
            // to be made with a passphrase and Smoozy is about to hand it to
            // the store, so the store's standing is taken here — and never at
            // application start, where nothing is being handed over.
            the_sentence_is_owed = crate::the_store_standing::at_the_first_use_of_a_secret(
                this_run,
                record_lender,
                store,
                deadline,
            )
            .is_some_and(|write| write.say_it_once);
            // **`Keys` S1's subject is its own identity, issued at first
            // store, and never the key row's own identifier.** `State,
            // identity and recovery` §2.2 issues a secret-store subject at the
            // first store of a kind against a subject, and §2.1 rule 1 is
            // *issued once and never reused* — and a key row's identifier
            // never changes, so reusing it made every put for that key land on
            // one subject for ever. `Keys` F10 rests on the opposite: *"A
            // passphrase later entered for the replaced key is a first store
            // and takes a **new** secret-store subject… So the departed key's
            // item is never reachable by the key that took its place."*
            //
            // **What it cost, concretely.** The erase-mark register is keyed by
            // kind and subject. After an erase whose store step failed the mark
            // still stands, and the next put landed on the subject the mark
            // names — so `key_rows::all` joined the row against the register
            // and answered *owing an erase* for a passphrase that was held, U9
            // said none was available, and the sweep then erased the passphrase
            // the user had typed a minute earlier and been told was remembered.
            //
            // **The subject is recorded on the row** below, which is what
            // makes `Keys` F10 true by construction rather than by care.
            let Ok(subject_identity) = Identity::issue(IdentityKind::SecretStoreSubject) else {
                // **An independent unit: its failure is reported and stops
                // nothing** (`Keys` G5). The key is whole and the passphrase
                // is simply not held, so the first connect asks.
                let _ = record.a_unit_ended(UnitRole::TheStorePut.as_role(), UnitOutcome::Failed);
                return Ok(Made {
                    outcome_record: record.identity().clone(),
                    pair,
                    row: Some(identifier),
                    the_passphrase_is_held: false,
                    the_sentence_is_owed,
                });
            };
            let subject = Subject::issued_by_the_local_record(subject_identity.as_text());
            let item = HeldSecret::new(StoredKind::KeyPassphrase, subject.clone());
            let put = store
                .put(&item, origin.for_a_destination(), deadline)
                .is_ok();
            if put {
                if let Some(row) = &row {
                    let mut held = row.clone();
                    held.passphrase = Some(PassphraseReference {
                        subject,
                        owing_an_erase: false,
                    });
                    record_lender.lend(&mut |connection| {
                        let _ = super::super::record::key_rows::write(connection, &held);
                    });
                }
            }
            let _ = record.a_unit_ended(
                UnitRole::TheStorePut.as_role(),
                if put {
                    UnitOutcome::Done
                } else {
                    UnitOutcome::Failed
                },
            );
            put
        }
    };

    Ok(Made {
        outcome_record: record.identity().clone(),
        pair,
        row: Some(identifier),
        the_passphrase_is_held,
        the_sentence_is_owed,
    })
}

/// **The tool run plus `Keys` G4's verification as one unit** (§11, §9.2 test
/// 2): a generated-but-unverified pair could be mistaken for a finished key,
/// so the unit's outcome is the verification's, and **a failed unit removes
/// the files the act wrote**.
fn the_key_pair_unit(ask: &TheAsk<'_>) -> ThePair {
    // **`Keys` G7: the file is the name the user typed, made safe**, and
    // **`Keys` G8: the first integer whose three names are all free**, checked
    // immediately before the invocation, through the adapter.
    let path = naming::compose_a_free_name(&ask.key_location, &ask.comment, ask.key_type, &|path: &Path| {
        !matches!(files::outside_program_wrote(path), files::Recheck::Absent)
    });

    match one_run(ask, &path) {
        Ran::APairMayBeThere => verify_or_undo(ask, &path),
        // **`Keys` G8's backstop: the tool refused an existing target**, so
        // the composition runs once more with the next free name and
        // `Keys` G4 judges
        // the result as always.
        Ran::TheToolRefused => {
            let again = naming::compose_a_free_name(&ask.key_location, &ask.comment, ask.key_type, &|path: &Path| {
                !matches!(files::outside_program_wrote(path), files::Recheck::Absent)
            });
            match one_run(ask, &again) {
                Ran::APairMayBeThere => verify_or_undo(ask, &again),
                Ran::TheToolRefused | Ran::NoGenerator => ThePair::TheToolProducedNothing,
                Ran::NoChannel { why } => ThePair::ThePassphraseCouldNotBeDelivered { why },
            }
        }
        Ran::NoGenerator => ThePair::TheToolProducedNothing,
        Ran::NoChannel { why } => ThePair::ThePassphraseCouldNotBeDelivered { why },
    }
}

enum Ran {
    APairMayBeThere,
    TheToolRefused,
    NoGenerator,
    NoChannel { why: askpass::CouldNotDeliver },
}

fn one_run(ask: &TheAsk<'_>, path: &Path) -> Ran {
    let asked = Asked {
        key_type: ask.key_type,
        comment: ask.comment.clone(),
        with_a_passphrase: ask.passphrase.is_some(),
        path: path.to_path_buf(),
    };

    match ask.passphrase {
        // **`Keys` G2: with no passphrase, `-N ""`.** The helper route is
        // closed for it by the promptless construction the readings already
        // use.
        None => {
            let invocation = generation::invocation(&asked, Path::new(""));
            match readings::run(&invocation.arguments) {
                ToolAnswer::Printed(_) => Ran::APairMayBeThere,
                ToolAnswer::NoGenerator => Ran::NoGenerator,
                ToolAnswer::Refused => Ran::TheToolRefused,
            }
        }
        // **`Keys` G3: a real passphrase reaches the generator through the
        // askpass route and never any other way.**
        Some(origin) => {
            let invocation = generation::invocation(&asked, Path::new(""));
            match askpass::run_with_the_askpass_route(
                &invocation.arguments,
                origin.a_delivery(),
                &ask.the_route,
            ) {
                Ok(askpass::ToolRun::Finished { exit_was_success: true }) => Ran::APairMayBeThere,
                Ok(askpass::ToolRun::Finished { .. }) => Ran::TheToolRefused,
                Err(why) => Ran::NoChannel { why },
            }
        }
    }
}

/// `Keys` G4, and the undo it names.
fn verify_or_undo(ask: &TheAsk<'_>, path: &Path) -> ThePair {
    let readings = OnThisComputer::over(ask.the_route.channel_area.clone());
    let made = WhatWasMade {
        the_private_file_is_there: matches!(
            files::outside_program_wrote(path),
            files::Recheck::Present { .. }
        ),
        container: readings.opening_bytes(path).as_deref().and_then(container::read),
        // **U7's composite reading, which is the one `Keys` G4 takes.**
        //
        // **This took the private half's own line alone, and it destroyed
        // every key made with a passphrase.** U7b's `-y -f` derives the public
        // half from the private key itself, so it needs the key unlocked and
        // is refused for a key that has just been protected — the verification
        // then failed at *the fingerprint would not read*, the undo unlinked
        // the private half, the `.pub` and the certificate, and no row was
        // written. **The reading `Keys` G4 takes is U7's, which is
        // composite**, and the `.pub` the same tool run has just written is
        // sitting beside the key.
        //
        // **`Keys` G4's other three checks are untouched** — the file is
        // there, the container test, and the protection reading agreeing with
        // what was asked — and that last one is what still makes a system
        // ignoring the askpass route fail loudly, so `Keys` G4's whole purpose
        // survives.
        the_fingerprint_reads: matches!(
            usable::read_the_fingerprint(path, &readings),
            usable::FingerprintRead::Answered(_)
        ),
    };
    let asked = Asked {
        key_type: ask.key_type,
        comment: ask.comment.clone(),
        with_a_passphrase: ask.passphrase.is_some(),
        path: path.to_path_buf(),
    };

    match generation::verify(&asked, &made) {
        Verified::ThePairIsWhatWasAskedFor => ThePair::Made {
            at: path.to_path_buf(),
        },
        Verified::Failed(why) => {
            // **The files the act itself just wrote are removed.** Only those:
            // the act composed a name whose three were free, so nothing here
            // can reach a file that was already the user's.
            let companions = locations::companions(path);
            let _ = files::remove_file(path);
            let _ = files::remove_file(&companions.public_half);
            let _ = files::remove_file(&companions.certificate);
            ThePair::Failed(why)
        }
    }
}

/// U7's reading of what was just made, for the row's own field.
///
/// **The source is recorded with the value** (U7a), and the reading is the
/// composite one: a key made **with** a passphrase does not read from its
/// private half at all, so its row's value comes from the `.pub` the same tool
/// run wrote — recorded as the fallback value it is, which is exactly what
/// keeps it out of clause 4b's evidence later. Taking the private half's line
/// alone left such a row with no fingerprint at all.
fn the_fingerprint_of(path: &Path, temporary_area: &Path) -> Option<Reading> {
    let readings = OnThisComputer::over(temporary_area.to_path_buf());
    match usable::read_the_fingerprint(path, &readings) {
        usable::FingerprintRead::Answered(said) => Some(said.reading),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use smoozy_security_and_licence::plaintext::Plaintext;
    use smoozy_security_and_licence::secret_store::{GetFailure, ProbeOutcome, StoreFailure};
    use std::cell::RefCell;

    fn scratch(name: &str) -> PathBuf {
        crate::scratch_folders::a_folder(format!("smoozy-making-{name}-{}", std::process::id()))
    }

    fn there_is_a_generator() -> bool {
        !matches!(
            readings::run(&["-h".to_string()]),
            ToolAnswer::NoGenerator
        )
    }

    /// The local record, in memory, migrated the way a real one is.
    fn a_record() -> rusqlite::Connection {
        let connection = rusqlite::Connection::open_in_memory().expect("a record");
        for migration in crate::record::migrations::ALL {
            crate::record::migrations::run_one_for_tests(&connection, migration)
                .expect("a migration");
        }
        connection
    }

    /// A store that remembers what it was given. **The real three adapters have
    /// their own tests**; what this one is for is the unit's outcome.
    struct AStore {
        puts: RefCell<Vec<HeldSecret>>,
        accepts: bool,
    }

    impl SecretStore for AStore {
        fn put(
            &self,
            item: &HeldSecret,
            _value: Plaintext,
            _deadline: Duration,
        ) -> Result<(), StoreFailure> {
            if !self.accepts {
                return Err(StoreFailure::StoreUnavailable);
            }
            self.puts.borrow_mut().push(item.clone());
            Ok(())
        }
        fn get(&self, _item: &HeldSecret, _deadline: Duration) -> Result<Plaintext, GetFailure> {
            Err(GetFailure::NotThere)
        }
        fn erase(&self, _item: &HeldSecret, _deadline: Duration) -> Result<(), StoreFailure> {
            Ok(())
        }
        fn probe(&self, _deadline: Duration) -> ProbeOutcome {
            ProbeOutcome::RoundTripWorked
        }
        fn enumerate(&self, _deadline: Duration) -> Result<Vec<HeldSecret>, StoreFailure> {
            Ok(Vec::new())
        }
    }

    fn a_store(accepts: bool) -> AStore {
        AStore {
            puts: RefCell::new(Vec::new()),
            accepts,
        }
    }

    fn an_ask<'a>(location: &Path, area: &Path, passphrase: Option<&'a Origin>) -> TheAsk<'a> {
        an_ask_named("someone@their-mac", location, area, passphrase)
    }

    fn an_ask_named<'a>(
        typed: &str,
        location: &Path,
        area: &Path,
        passphrase: Option<&'a Origin>,
    ) -> TheAsk<'a> {
        TheAsk {
            key_location: location.to_path_buf(),
            key_type: KeyType::Ed25519,
            comment: typed.to_string(),
            passphrase,
            the_default_name_now: "someone@their-mac".to_string(),
            // **This test binary is not the helper**, and no test here drives
            // the route: the one that does is at `tests/askpass_route.rs`, with
            // the built application's own path.
            the_route: askpass::TheRoute::with_the_helper_at(
                area.to_path_buf(),
                PathBuf::from("/nonexistent/never-run-from-a-unit-test"),
            ),
        }
    }

    /// **`Keys` T2, on this computer.** The probe is a reading of the
    /// installed binary and not of any manual, and this is that reading taken.
    #[test]
    fn the_probe_reads_the_installed_binary_and_leaves_nothing_behind() {
        if !there_is_a_generator() {
            return;
        }
        let area = scratch("probe");
        let offered = probe_the_offered_set(&area);
        assert!(
            offered.offerable().contains(&KeyType::Ed25519),
            "this computer's ssh-keygen makes ed25519 keys"
        );
        assert_eq!(offered.default_type(), Some(KeyType::Ed25519));
        // **Deleted at once.**
        let left: Vec<_> = std::fs::read_dir(&area)
            .expect("the area")
            .flatten()
            .map(|entry| entry.file_name())
            .collect();
        assert!(left.is_empty(), "the probe left {left:?}");
    }

    /// **`Keys` G1, `Keys` G2, `Keys` G4 and the row, on a real computer with
    /// the real
    /// tool.**
    #[test]
    fn a_key_with_no_passphrase_is_made_verified_and_rowed() {
        if !there_is_a_generator() {
            return;
        }
        let area = scratch("plain");
        let location = area.join(".ssh");
        let connection = a_record();
        let store = a_store(true);
        let records = crate::record::outcome_records::OutcomeRecords::over(&connection);

        let made = generate(
            &an_ask(&location, &area, None),
            &crate::keys::TheRecordItself::over(&connection),
            &store,
            Duration::from_secs(5),
            &crate::the_store_standing::TheProbeThisRun::new(),
        )
        .expect("the act ran");

        let at = match &made.pair {
            ThePair::Made { at } => at.clone(),
            other => panic!("the pair was {other:?}"),
        };
        // `Keys` G7's scheme — the name as typed, here the pre-filled
        // default — and the pair is really there.
        assert_eq!(at.file_name().unwrap(), "someone@their-mac");
        assert!(at.exists() && locations::companions(&at).public_half.exists());
        // `Keys` G9: the location was made, and on this system at 0700.
        assert!(location.is_dir());
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mode = std::fs::metadata(&location).unwrap().permissions().mode();
            assert_eq!(mode & 0o777, 0o700);
        }
        // The row is in the local record, with the fingerprint read from the
        // private file itself.
        let rows = crate::record::key_rows::all(&connection).expect("the rows");
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].path, at);
        assert_eq!(
            rows[0].fingerprint.as_ref().map(|reading| reading.read_from),
            Some(smoozy_keys::fingerprint::ReadFrom::PrivateFile)
        );
        assert!(rows[0].was_default_at_entry, "N1's bit, recorded at entry");
        // Nothing was put: no passphrase was asked for.
        assert!(!made.the_passphrase_is_held);
        assert!(store.puts.borrow().is_empty());

        // §9.4: four units, all done, and the act's outcome derived.
        use smoozy_state::outcomes::Keeps;
        let record = records.read(&made.outcome_record).unwrap().unwrap();
        assert_eq!(
            record
                .units
                .iter()
                .map(|(role, _)| role.as_str())
                .collect::<Vec<_>>(),
            vec!["the directory", "the key pair", "the row", "the store put"]
        );
        assert_eq!(
            record.the_actions_outcome(),
            Some(smoozy_state::outcomes::ActionOutcome::Whole)
        );
        let _ = std::fs::remove_dir_all(&area);
    }

    /// Makes one key named `typed` in `location` with the real tool, and says
    /// where its private half went.
    fn make_one(
        typed: &str,
        location: &Path,
        area: &Path,
        connection: &rusqlite::Connection,
    ) -> PathBuf {
        let made = generate(
            &an_ask_named(typed, location, area, None),
            &crate::keys::TheRecordItself::over(connection),
            &a_store(true),
            Duration::from_secs(5),
            &crate::the_store_standing::TheProbeThisRun::new(),
        )
        .expect("the act ran");
        match &made.pair {
            ThePair::Made { at } => at.clone(),
            other => panic!("the pair was {other:?}"),
        }
    }

    /// **`Keys` G7 and G8: a key's files take its name, and a second key of
    /// the same name takes the next free one** — checked by making two keys
    /// named *hyperstack* in one throwaway location. The comment stays the
    /// typed name on both.
    #[test]
    fn a_key_named_hyperstack_is_stored_as_hyperstack_and_the_second_as_hyperstack_2() {
        if !there_is_a_generator() {
            return;
        }
        let area = scratch("collision");
        let location = area.join(".ssh");
        let connection = a_record();
        for expected in ["hyperstack", "hyperstack_2"] {
            let at = make_one("hyperstack", &location, &area, &connection);
            assert_eq!(at, location.join(expected));
            assert!(location.join(format!("{expected}.pub")).exists());
        }
        // Nothing was overwritten: both pairs stand.
        assert!(location.join("hyperstack").exists());
        assert!(location.join("hyperstack_2").exists());
        // The comment is the typed name on both, not the file's.
        for row in crate::record::key_rows::all(&connection).expect("the rows") {
            assert_eq!(row.name.as_ref().map(|name| name.as_str()), Some("hyperstack"));
            let public = std::fs::read_to_string(locations::companions(&row.path).public_half)
                .expect("the public half");
            assert!(public.trim_end().ends_with(" hyperstack"), "{public}");
        }
        let _ = std::fs::remove_dir_all(&area);
    }

    /// **`Keys` G7's making-safe and its fallback, with the real tool**: a
    /// character no file name can hold becomes `_`, and an empty name or one
    /// Windows reserves falls back to `smoozy_<type>`, then `_2`.
    #[test]
    fn an_unsafe_an_empty_and_a_reserved_name_each_land_where_g7_says() {
        if !there_is_a_generator() {
            return;
        }
        let area = scratch("unsafe");
        let location = area.join(".ssh");
        let connection = a_record();
        for (typed, expected) in [
            ("a/b:c", "a_b_c"),
            ("", "smoozy_ed25519"),
            ("CON", "smoozy_ed25519_2"),
        ] {
            assert_eq!(make_one(typed, &location, &area, &connection), location.join(expected));
        }
        let _ = std::fs::remove_dir_all(&area);
    }

    /// **Keys already made keep their files** (`Keys` G7): a key made before
    /// 2026-10-08 under `smoozy_ed25519` is neither renamed nor rewritten, and
    /// its row is untouched, when a new key named *hyperstack* is made beside
    /// it.
    #[test]
    fn a_key_already_made_keeps_its_files_and_its_row() {
        if !there_is_a_generator() {
            return;
        }
        let area = scratch("existing");
        let location = area.join(".ssh");
        std::fs::create_dir_all(&location).expect("the location");
        let old = location.join("smoozy_ed25519");
        std::fs::write(&old, b"an older private half").expect("the old key");
        std::fs::write(location.join("smoozy_ed25519.pub"), b"an older public half")
            .expect("the old public half");
        let connection = a_record();
        let old_row = KeyRow::entering(
            Identity::issue(IdentityKind::LocalKey).expect("randomness"),
            old.clone(),
            None,
            Name::what_the_user_typed("hyperstack"),
            false,
        )
        .expect("a row");
        crate::record::key_rows::add(&connection, &old_row).expect("the old row");

        let at = make_one("hyperstack", &location, &area, &connection);
        assert_eq!(at, location.join("hyperstack"));

        assert_eq!(std::fs::read(&old).unwrap(), b"an older private half");
        assert_eq!(
            std::fs::read(location.join("smoozy_ed25519.pub")).unwrap(),
            b"an older public half"
        );
        let rows = crate::record::key_rows::all(&connection).expect("the rows");
        assert_eq!(rows.len(), 2);
        let kept = rows
            .iter()
            .find(|row| row.identifier == old_row.identifier)
            .expect("the old row is still there");
        assert_eq!(kept.path, old);
        assert_eq!(kept.name, old_row.name);
        assert_eq!(kept.fingerprint, None);
        let _ = std::fs::remove_dir_all(&area);
    }

    /// **`Keys` G3 and `Keys` G5 together, with the real askpass route.** The key
    /// carries the passphrase and the same value is put to the store —
    /// `Security and licence` B29's origin serving both destinations of one
    /// act.
    ///
    /// **A generation with a real passphrase is driven by
    /// `tests/askpass_route.rs`**, which holds the built application's own
    /// path and can name it as the helper. What this one proves is the other
    /// half of `Keys` G4: **a passphrase asked for and not delivered fails the
    /// act loudly** rather than quietly making a weaker key.
    #[test]
    #[cfg(unix)]
    fn a_passphrase_asked_for_and_not_delivered_fails_the_act_loudly() {
        if !there_is_a_generator() {
            return;
        }
        let area = scratch("nochannel");
        let location = area.join(".ssh");
        let connection = a_record();
        let store = a_store(true);
        let records = crate::record::outcome_records::OutcomeRecords::over(&connection);
        let typed = Origin::what_the_user_typed("a passphrase");

        // The channel area does not exist, so no socket can be bound there.
        let made = generate(
            &TheAsk {
                the_route: askpass::TheRoute::with_the_helper_at(
                    area.join("no such area"),
                    PathBuf::from("/nonexistent/never-run-from-a-unit-test"),
                ),
                ..an_ask(&location, &area, Some(&typed))
            },
            &crate::keys::TheRecordItself::over(&connection),
            &store,
            Duration::from_secs(5),
            &crate::the_store_standing::TheProbeThisRun::new(),
        )
        .expect("the act ran");

        // **`Keys` G4's whole point**: never a key quietly weaker than was
        // asked for.
        assert!(matches!(
            made.pair,
            ThePair::ThePassphraseCouldNotBeDelivered { .. }
        ));
        assert!(made.row.is_none());
        assert!(crate::record::key_rows::all(&connection).unwrap().is_empty());
        assert!(!location.join("smoozy_ed25519").exists());

        // §9.3: the row and the store put were never tried, and say so.
        use smoozy_state::outcomes::Keeps;
        let record = records.read(&made.outcome_record).unwrap().unwrap();
        assert_eq!(
            record.what_could_not_be_done().len(),
            3,
            "the pair failed and two units were not attempted"
        );
        assert!(record.unfinished().next().is_none());
        let _ = std::fs::remove_dir_all(&area);
    }

    // **What is not tested here, said plainly rather than left to be
    // discovered.** A generation *with* a passphrase cannot be driven from a
    // unit test: the askpass helper is Smoozy's own binary re-entered, and a
    // test binary named as the helper would have `ssh-keygen` run the test
    // harness. **The route itself is proved end to end with the real binary at
    // `tests/askpass_route.rs`**; what is unproved is `generate` driving it,
    // and with it `Keys` G5's store put and the whole-key-after-a-refused-put
    // case. The nearest thing that is proved is the test above: a passphrase
    // asked for and not delivered fails the act loudly and writes no row.
}
