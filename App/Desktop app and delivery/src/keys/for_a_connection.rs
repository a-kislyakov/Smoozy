//! `Keys`' end of a connection's credential — the real
//! `smoozy_servers::acting::SupplyTheCredential`.
//!
//! **This joins two halves that were both finished and had never met.**
//! `Servers` declared the seam and drove it against a stand-in; `Keys` built
//! the whole ordered supply — [`super::supply`] — and nothing called it. What
//! is here is the join and nothing else: **no judgement below is this file's**,
//! every one of them is `Keys`' or `Servers`'.
//!
//! **The three answers, and what each one keeps.**
//!
//! - `the_two_answers` — `Servers` P1 and P2. Both taken at the moment of the
//!   attempt, by running `Keys` U10's check afresh, and never from a value
//!   written earlier.
//! - `for_one_attempt` — `Servers` P4 and P4a. A connect the user started may
//!   ask; a connection Smoozy makes on its own may not, and where nobody may be
//!   asked this answers nothing.
//! - `the_refusal_is_permanent` — `Servers` N8b, which turns on `Security and
//!   licence` §3.3's store standing and `Keys` U8's protection reading.
//!
//! **What this file does not do, and each is somebody else's on purpose.** It
//! opens no connection — that is `Servers`' transport. It puts nothing on
//! screen — [`TheAsk`] is composed here and run-held, and the screen that shows
//! it is the surface's. It writes no key row of its own — every write below is
//! [`super::supply`]'s, through the lent record.
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
//! it belongs to.** `Keys` S14, S18, S19, U5, U7a, U8, U9, U10.
//! `Servers` N8b, P1a, P4a.
//! `Security and licence` §3.0, §3.3 and §5.1 are cited by section rather than
//! by marker.
//!
//! **A marker this crate's own plan also defines is deliberately not listed
//! above, whichever document is meant, and is written out with its document
//! at every site instead** — the header list cannot tell the two apart, and a
//! bare one would resolve to a real and wrong clause.

use std::time::Duration;

use smoozy_keys::fingerprint::Reading;
use smoozy_keys::passphrase::{ServerContext, TheAsk};
use smoozy_keys::rows::KeyRow;
use smoozy_keys::usable::{self, Checked, Verdict};
use smoozy_security_and_licence::erasure::Erasure;
use smoozy_security_and_licence::plaintext::{OneUseBroker, Origin, Plaintext};
use smoozy_security_and_licence::secret_store::{GetFailure, SecretStore};
use smoozy_security_and_licence::standing::StoreStanding;
use smoozy_server_protocol::transport::Credential;
use smoozy_servers::acting::{SupplyTheCredential, TheServerBeingConnectedTo};
use smoozy_servers::identity::KeyReference;
use smoozy_servers::key_moments::{self, TheTwoAnswers, WhoStartedIt};
use smoozy_servers::repair::PermanentRefusal;
use smoozy_state::identities::{Identity, IdentityKind};

use super::askpass::TheRoute;
use super::readings::OnThisComputer;
use super::supply::{self, HeldPassphrase, Supplied, WhatTheBoxTook};
use super::LendsTheRecord;
use crate::record::key_rows;

/// **Why the supply answered nothing, for the caller that has to say so.**
///
/// `Servers` P3 leaves the server *"marked with what is wrong"*, and what is
/// wrong is a `Keys` reading that this side took. `Servers`' own
/// `WhereItStopped::NoCredentialWasSupplied` says the supply answered nothing
/// and deliberately claims no reason — **this is the reason, held beside it**.
///
/// **Run-held and nothing else** (`State, identity and recovery` §11.1): it is
/// replaced at the next ask and nothing durable carries it.
#[derive(Debug)]
pub enum WhyNothingWasSupplied {
    /// The server row names a key row that is not in the record.
    TheKeyRowIsNotThere,
    /// **`Keys` U10's check refused it at this attempt's own moment.**
    TheKeyIsNotUsable { verdict: Verdict },
    /// **`Keys` U9's answer is no and `Servers` P4 says nobody may be asked**,
    /// so no attempt is made.
    NothingIsHeldAndNobodyMayBeAsked,
    /// **`Keys` S14's box is raised** and [`TheCredentialForAConnection::the_ask`]
    /// carries it. The attempt this call belonged to makes none.
    NothingIsHeldAndTheBoxIsRaised,
    /// **`Keys` S7 and `Security and licence` §3.0 rule 4: an ordinary failure
    /// and not a secret's own.** Reported on the thing that failed, the action
    /// offered again, **the user not asked and the passphrase not forgotten**.
    TheStoreWouldNotAnswer { failure: GetFailure },
    /// **`Keys` S5: the local load failed after a passing check**, so the held
    /// passphrase is attributable and is gone. Where the user started this
    /// connect, `Keys` S19's box stands beside this with its line.
    ThePassphraseStoppedWorking {
        erasure: Erasure,
        outcome_record: Identity,
    },
    /// **What the user typed did not open the key.** `Keys` S19: a second
    /// failure is a failed load reported, **never a third box**, and nothing is
    /// put.
    WhatWasTypedDidNotOpenTheKey,
    /// **The load never ran**, so nothing separated the passphrase's own
    /// failure from anything else and nothing is forgotten. On Windows there is
    /// no channel at all — `Keys` §13 item 2.
    TheLoadCouldNotBeRun { why: super::askpass::CouldNotDeliver },
}

/// The check this attempt took, kept for the length of the attempt.
///
/// **`Servers` P2 is about attempts and not about calls.** The sequence asks
/// `the_two_answers` and then `for_one_attempt` in the same moment of the same
/// attempt, and `Keys` S5 fixes **one** check per attempt — so a second
/// `ssh-keygen` run between the two would be the deviation rather than the
/// care. It is discarded the instant a different attempt asks.
struct WhatThisAttemptRead {
    key: String,
    row: KeyRow,
    checked: Checked,
}

/// **`Keys`' side of one connect**, made for that connect and dropped with it.
///
/// **It holds no secret across a call.** What the box took is the caller's own
/// [`Origin`], borrowed for the connect; everything else here is a path, a
/// reference or a reading.
pub struct TheCredentialForAConnection<'a> {
    /// The local record, lent for one read or one write and never held across a
    /// tool run — L4.
    pub record: &'a dyn LendsTheRecord,
    /// The operating system's secret store, or the one that answers nothing.
    pub store: &'a dyn SecretStore,
    /// Where the askpass channel is opened, and the program `SSH_ASKPASS`
    /// names.
    pub the_route: TheRoute,
    /// How long a store call is given.
    pub deadline: Duration,
    /// **This run's probe of the secret store** — `Security and licence` S26,
    /// at that document's §3.3, and this crate's own S5. The supply below is a moment when a secret
    /// is actually needed, and [`crate::the_store_standing`] is what makes
    /// `Servers` N8b's standing capable of changing at all.
    pub this_run: &'a crate::the_store_standing::TheProbeThisRun,
    /// **`Keys` S16's order, carried across the two calls it takes.**
    ///
    /// `Keys` S16 fixes the order at a user-started connect as *the three readings,
    /// then the box, then `Keys` S5's supply* — so the box is answered
    /// **before** the attempt this supply serves. What the user typed is the
    /// caller's own value, borrowed here for the connect it opened.
    ///
    /// **`Keys` S9 is why it exists rather than the store being read again.**
    /// On a computer with nowhere to remember, the put fails and nothing is
    /// ever held, and *"a connect the user starts works normally, asking for
    /// the passphrase each time"* — which is only true if what was typed
    /// reaches this attempt.
    pub what_the_box_took: Option<&'a Origin>,
    /// **`Keys` S14 and S19's box, where this supply raised one.** Run-held:
    /// the act that raised it is the act that shows it, and nothing durable
    /// carries it (`Keys` S18).
    pub the_ask: Option<TheAsk>,
    /// Why the last call answered nothing.
    pub why_nothing_was_supplied: Option<WhyNothingWasSupplied>,
    /// **`Security and licence` S34's sentence is owed, and this connect was
    /// the first caller in the run told to say it.** The probe this supply took
    /// moved the standing from *never probed* to *never answered*; the caller
    /// carries the sentence to the place `Preparation/Design/Keys.md` gives it —
    /// with the box where this connect raised one, and on the server's row
    /// where it did not. *(Step 10b's first audit round, item 7: the answer was
    /// thrown away here, and the moment it answers happens once for the life
    /// of an installation.)*
    pub the_sentence_is_owed: bool,
    this_attempt: Option<WhatThisAttemptRead>,
}

impl<'a> TheCredentialForAConnection<'a> {
    pub fn over(
        record: &'a dyn LendsTheRecord,
        store: &'a dyn SecretStore,
        the_route: TheRoute,
        deadline: Duration,
        this_run: &'a crate::the_store_standing::TheProbeThisRun,
    ) -> TheCredentialForAConnection<'a> {
        TheCredentialForAConnection {
            record,
            store,
            the_route,
            deadline,
            this_run,
            what_the_box_took: None,
            the_ask: None,
            why_nothing_was_supplied: None,
            the_sentence_is_owed: false,
            this_attempt: None,
        }
    }

    /// The key row a server row's reference names. **The identifier alone**;
    /// `Servers` P1a's second half — the fingerprint — is answered by the check
    /// rather than by the lookup, because a row whose fingerprint has moved is
    /// still the row that identifier names.
    fn the_row_named(&self, identifier: &str) -> Option<KeyRow> {
        let named = Identity::read_back(IdentityKind::LocalKey, identifier)?;
        let mut found = None;
        self.record.lend(&mut |connection| {
            found = key_rows::all(connection).ok().and_then(|rows| {
                rows.into_iter().find(|held| held.identifier == named)
            });
        });
        found
    }

    /// **`Servers` P2: a fresh check, and every attempt takes its own.**
    ///
    /// **This is called at the top of an attempt and nowhere else.** A repair
    /// run asks the two questions again at every attempt, and a reading kept
    /// from the last one is exactly the cached value P2 names — *"a key can go
    /// away between a draw and a connect"*. **Written the other way round, an
    /// eight-attempt run would answer from one reading taken 105 seconds
    /// earlier.**
    fn take_a_fresh_check(&mut self, key: &KeyReference) -> Option<&WhatThisAttemptRead> {
        let row = self.the_row_named(&key.local_key_identifier)?;
        let readings = OnThisComputer::over(self.the_route.channel_area.clone());
        let checked = usable::check(&row, &readings);
        self.this_attempt = Some(WhatThisAttemptRead {
            key: key.local_key_identifier.clone(),
            row,
            checked,
        });
        self.this_attempt.as_ref()
    }

    /// **The check this attempt already took, or a fresh one where it took
    /// none.**
    ///
    /// **`Keys` S5 fixes one check per attempt**, and the sequence asks
    /// `the_two_answers` and then `for_one_attempt` inside one attempt — so a
    /// second `ssh-keygen` run between the two would be the deviation rather
    /// than the care. A caller that reaches either of the other two answers
    /// without asking the two questions first still gets a reading of its own.
    fn what_this_attempt_read(&mut self, key: &KeyReference) -> Option<&WhatThisAttemptRead> {
        let already = matches!(
            &self.this_attempt,
            Some(read) if read.key == key.local_key_identifier
        );
        if already {
            return self.this_attempt.as_ref();
        }
        self.take_a_fresh_check(key)
    }

    /// **`Security and licence` §3.3's standing, read from the local record.**
    ///
    /// **A read that will not answer is not a fresh installation** — the
    /// reading rule is `crate::surface::set_up::the_store_standing`'s and is
    /// not written a second time here.
    fn the_store_standing(&self) -> Option<StoreStanding> {
        let mut standing = None;
        self.record.lend(&mut |connection| {
            standing = crate::surface::set_up::the_store_standing(connection);
        });
        standing
    }

    /// **The end of `Keys` S5's order for a value already in hand**: the check
    /// and the one local load are [`supply::supply_a_held_passphrase`]'s, and
    /// what comes back here is the credential the connection is made with.
    ///
    /// **One get, two deliveries, and that is what one attempt genuinely
    /// needs.** `Security and licence` §1.4 hop 2 is one delivery each: the
    /// askpass route carries one to `ssh-keygen`, and the transport's own
    /// key-loading step takes the other (§4 R2). **`Keys` S6's *the store is
    /// reached once per use* is kept** — the store was read once, above this.
    fn the_load_then_the_connection(
        &mut self,
        row: &KeyRow,
        value: Plaintext,
        server: ServerContext,
        who: WhoStartedIt,
    ) -> Option<Credential> {
        let (for_the_load, for_the_transport) = value.expose(|bytes| {
            (
                OneUseBroker::holding(Plaintext::new(bytes.to_vec())),
                Plaintext::new(bytes.to_vec()),
            )
        });
        let supplied = supply::supply_a_held_passphrase(
            row,
            for_the_load,
            server,
            self.record,
            self.store,
            self.deadline,
            &self.the_route,
        );
        match supplied {
            Supplied::TheKeyOpened => Some(Credential {
                private_key_file: row.path.clone(),
                passphrase: Some(for_the_transport),
            }),
            Supplied::TheCheckFailed { verdict } => {
                self.why_nothing_was_supplied =
                    Some(WhyNothingWasSupplied::TheKeyIsNotUsable { verdict });
                None
            }
            Supplied::ThePassphraseStoppedWorking {
                erasure,
                outcome_record,
                ask,
            } => {
                // **`Keys` S19: the same act raises the box with the line —
                // and only where the user started this connect.** S5's other
                // half is untouched: *"an automatic attempt finds the marker
                // gone and makes no attempt"*, so a repair erases the
                // passphrase and puts nothing on screen. This is `Keys` S13's
                // refusal, reached from the supply side.
                if who.may_ask_for_a_passphrase() {
                    self.the_ask = Some(ask);
                }
                self.why_nothing_was_supplied =
                    Some(WhyNothingWasSupplied::ThePassphraseStoppedWorking {
                        erasure,
                        outcome_record,
                    });
                None
            }
            Supplied::TheLoadCouldNotBeRun { why } => {
                self.why_nothing_was_supplied =
                    Some(WhyNothingWasSupplied::TheLoadCouldNotBeRun { why });
                None
            }
        }
    }
}

/// **`Security and licence` §5.4 and §13 rule 4**: nothing of a secret is ever
/// rendered, and the field that could hold one is named as present or absent.
impl std::fmt::Debug for TheCredentialForAConnection<'_> {
    fn fmt(&self, out: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        out.debug_struct("TheCredentialForAConnection")
            .field("the_box_was_answered", &self.what_the_box_took.is_some())
            .field("an_ask_stands", &self.the_ask.is_some())
            .field("why_nothing_was_supplied", &self.why_nothing_was_supplied)
            .finish()
    }
}

impl SupplyTheCredential for TheCredentialForAConnection<'_> {
    /// **`Servers` P1 and P2, both answered from readings taken now.**
    ///
    /// **`Servers` P1's second question carries its own condition and the condition is
    /// composed here** — *"**if it needs a passphrase**, is one available for
    /// this attempt without asking?"* `Keys` U9 answers the inner half and
    /// deliberately answers nothing about protection, because `Keys` U5 keeps
    /// the passphrase out of *usable* altogether. **So a key with no passphrase
    /// answers yes**, and a composition that handed U9's bare answer over would
    /// have withheld every automatic connection made with an unprotected key.
    ///
    /// **`Servers` P1a's pair is both halves.** The identifier finds the row; the
    /// fingerprint the server row holds must equal the key row's current one,
    /// and where they disagree the answer is no. `Keys`' replacement act keeps
    /// the identifier and changes the fingerprint, so an identifier-only answer
    /// would go on saying yes for every server set up with the key that left.
    fn the_two_answers(&mut self, key: &KeyReference) -> TheTwoAnswers {
        self.why_nothing_was_supplied = None;
        // **`Servers` P2: this is the top of an attempt, so the reading is
        // taken now** and never carried over from the attempt before it.
        let Some(read) = self.take_a_fresh_check(key) else {
            self.why_nothing_was_supplied = Some(WhyNothingWasSupplied::TheKeyRowIsNotThere);
            return TheTwoAnswers {
                the_key_is_usable: false,
                a_passphrase_is_available_without_asking: false,
            };
        };

        // **`Keys` R1 and `Keys` U7a: what the row keeps of this check's reading**,
        // through the function that decides it. The write-back belongs to the
        // supply's own check a moment later and is not made twice.
        let now = match read.checked.fingerprint_now.clone() {
            Some(fresh) => Some(Reading::what_the_row_keeps(read.row.fingerprint.as_ref(), fresh)),
            None => read.row.fingerprint.clone(),
        };
        let still_the_same_key = now.is_some_and(|reading| {
            key_moments::the_reference_still_names_the_same_key(key, reading.value.as_str())
        });

        // **`Keys` U8's protection reading**, absent where the container could
        // not be read — in which case U10's verdict is not *usable* either, so
        // the first answer is already no.
        let needs_one = read.checked.protected.unwrap_or(true);
        TheTwoAnswers {
            the_key_is_usable: read.checked.verdict == Verdict::Usable && still_the_same_key,
            a_passphrase_is_available_without_asking: !needs_one
                || usable::a_passphrase_is_available_without_asking(&read.row),
        }
    }

    /// **`Servers` P4 and P4a: one credential, good for one attempt.**
    ///
    /// **Four routes in and they are the whole set.** A key that needs no
    /// passphrase; a value the box took for this connect; a value the store
    /// holds; and nothing, which answers nothing.
    ///
    /// **`Servers` P4a: what crosses at the ask is three display-only fields and no
    /// secret.** They are handed straight to `Keys`' box and nothing is
    /// composed from them here.
    fn for_one_attempt(
        &mut self,
        key: &KeyReference,
        who: WhoStartedIt,
        naming: TheServerBeingConnectedTo<'_>,
    ) -> Option<Credential> {
        self.why_nothing_was_supplied = None;
        self.the_ask = None;
        let Some(read) = self.what_this_attempt_read(key) else {
            self.why_nothing_was_supplied = Some(WhyNothingWasSupplied::TheKeyRowIsNotThere);
            return None;
        };
        let row = read.row.clone();
        let protected = read.checked.protected;
        let verdict = read.checked.verdict.clone();
        let server = ServerContext {
            label: naming.label.map(str::to_string),
            username: Some(naming.username.to_string()),
            host: Some(naming.host.to_string()),
        };

        // **`Servers` P3, kept on this side too.** The sequence has already
        // refused an unusable key, and a caller that had not would otherwise
        // reach the load.
        if verdict != Verdict::Usable {
            self.why_nothing_was_supplied =
                Some(WhyNothingWasSupplied::TheKeyIsNotUsable { verdict });
            return None;
        }

        // **`Keys` U8: a key that needs no passphrase.** `Security and licence`
        // §6's P1 — Smoozy holds a path and hands the path over, and the
        // transport's own key-loading step reads the file.
        if protected == Some(false) {
            return Some(Credential {
                private_key_file: row.path,
                passphrase: None,
            });
        }

        // **`Security and licence` S26, at §3.3: from here down a secret is
        // actually needed** — the store is reached for a held passphrase, or
        // one the user just typed is put into it. A key needing no passphrase
        // has already returned above, and nothing was handed over for it.
        self.the_probe_is_taken();

        // **`Keys` S16's order: the box came before this.** What it took is the
        // caller's own value, and this attempt is the one it opened — `Keys`
        // S17's *a value opens the attempt*.
        if let Some(typed) = self.what_the_box_took {
            let took = supply::take_what_the_user_typed(
                &row,
                typed,
                None,
                self.record,
                self.store,
                self.deadline,
                &self.the_route,
                self.this_run,
            );
            return match took {
                // **The put is an independent unit whose failure costs the
                // attempt nothing** — `Keys` S17 — so the connection is made
                // whether or not the value was remembered, which is `Keys` S9's
                // *a connect the user starts works normally* on a computer with
                // nowhere to remember.
                WhatTheBoxTook::TheKeyOpened {
                    the_sentence_is_owed,
                    ..
                } => {
                    self.the_sentence_is_owed |= the_sentence_is_owed;
                    Some(Credential {
                        private_key_file: row.path,
                        passphrase: Some(typed.for_a_destination()),
                    })
                }
                WhatTheBoxTook::ItDidNotOpenTheKey => {
                    self.why_nothing_was_supplied =
                        Some(WhyNothingWasSupplied::WhatWasTypedDidNotOpenTheKey);
                    None
                }
                WhatTheBoxTook::TheCheckFailed { verdict } => {
                    self.why_nothing_was_supplied =
                        Some(WhyNothingWasSupplied::TheKeyIsNotUsable { verdict });
                    None
                }
                WhatTheBoxTook::TheLoadCouldNotBeRun { why } => {
                    self.why_nothing_was_supplied =
                        Some(WhyNothingWasSupplied::TheLoadCouldNotBeRun { why });
                    None
                }
            };
        }

        // **`Keys` S7: the one get, and each of its three endings costs
        // something different.**
        match supply::the_held_passphrase(&row, self.store, self.deadline) {
            HeldPassphrase::Held(value) => {
                self.the_load_then_the_connection(&row, value, server, who)
            }
            HeldPassphrase::TheStoreFailed { failure } => {
                // **`Security and licence` §3.0 rule 4: an ordinary failure.**
                // The user is not asked and the passphrase is not forgotten.
                self.why_nothing_was_supplied =
                    Some(WhyNothingWasSupplied::TheStoreWouldNotAnswer { failure });
                None
            }
            HeldPassphrase::NoneIsHeld => {
                self.nothing_is_held(&row, server, who);
                None
            }
            HeldPassphrase::GoneFromTheStore => {
                // **`Keys` U9's last clause: the marker was ahead of the
                // store, and it is corrected here, inside the attempt** —
                // whoever started it. Then `Security and licence` §3.0 rule 4:
                // *not there* asks again, as though nothing was remembered, so
                // the ask reads the row as it now stands and never the stale
                // marker. *(Added 2026-09-18: the ask read the marker, found a
                // passphrase said to be held, and raised no box.)*
                let (_erasure, row) = supply::forget_a_reference_the_store_no_longer_holds(
                    &row,
                    self.record,
                    self.store,
                    self.deadline,
                );
                if let Some(read) = self.this_attempt.as_mut() {
                    read.row = row.clone();
                }
                self.nothing_is_held(&row, server, who);
                None
            }
        }
    }

    /// **`Servers` N8b: whether a *no* to the second question is permanent.**
    ///
    /// **Both halves are read and neither is composed from the two answers.**
    /// `Security and licence` §3.3's standing is a durable note in the local
    /// record; `Keys` U8's protection is a reading of the key's own container.
    ///
    /// **A standing that will not read is not permanence.** The repair stops
    /// for good on this answer, so an unreadable standing answers *no* and the
    /// repair goes on trying — the cost of the other direction is a server that
    /// never reconnects because one query failed once.
    fn the_refusal_is_permanent(&mut self, key: &KeyReference) -> PermanentRefusal {
        // **`Security and licence` S26, at §3.3: a moment in this run when a secret
        // is actually needed.** This question is only ever asked after a
        // passphrase was wanted for a connection and was not available without
        // asking — so the standing is taken before it is read, and **not at
        // application start**, where nothing is being handed over.
        //
        // **Without this the repair would spin for good**, which is the exact
        // thing `Servers` N8b exists to prevent: the standing would read *never
        // probed* for the life of the installation and the refusal could never
        // be permanent however many times the store said no.
        self.the_probe_is_taken();
        let standing = self.the_store_standing();
        let protected = self
            .what_this_attempt_read(key)
            .and_then(|read| read.checked.protected)
            .unwrap_or(false);
        PermanentRefusal {
            the_store_standing_reads_never_answered: standing == Some(StoreStanding::NeverAnswered),
            the_rows_key_is_protected: protected,
        }
    }
}

impl TheCredentialForAConnection<'_> {
    /// **Nothing is held for this attempt.** `Servers` P4: a connect the user
    /// started may ask; a connection Smoozy makes on its own may not. `Keys`
    /// S14 decides whether the box is raised at all, from three readings this
    /// side does not re-take.
    fn nothing_is_held(&mut self, row: &KeyRow, server: ServerContext, who: WhoStartedIt) {
        let ask = supply::the_ask_for_a_key_with_nothing_held(
            row,
            server,
            who.may_ask_for_a_passphrase(),
            self.an_earlier_erasure_is_on_the_record(row),
            self.record,
            &self.the_route.channel_area,
        );
        self.why_nothing_was_supplied = Some(match ask {
            Some(ask) => {
                self.the_ask = Some(ask);
                WhyNothingWasSupplied::NothingIsHeldAndTheBoxIsRaised
            }
            None => WhyNothingWasSupplied::NothingIsHeldAndNobodyMayBeAsked,
        });
    }

    /// **`Security and licence` S26's probe, at a moment a secret is actually
    /// needed — and `Security and licence` S34's *say it once* kept rather
    /// than dropped.** The writer
    /// hands the notice to the first caller in the run and to no other, so a
    /// yes here is the one yes this installation will ever get.
    fn the_probe_is_taken(&mut self) {
        let wrote = crate::the_store_standing::at_the_first_use_of_a_secret(
            self.this_run,
            self.record,
            self.store,
            self.deadline,
        );
        if wrote.is_some_and(|write| write.say_it_once) {
            self.the_sentence_is_owed = true;
        }
    }

    /// **`Keys` S18 and §13 item 19: an ask that follows an interrupted act
    /// says no line, and this is where that is kept.**
    ///
    /// `State, identity and recovery` §9.4's outcome record is written by the
    /// erasure act and read back by [`crate::record::outcome_records`], but it
    /// enumerates its units **by role** and carries no key — no path, no
    /// identifier — so nothing can tell which key an earlier act's record
    /// belongs to. **The plan says the same since 2026-09-18**: the line is
    /// unsaid wherever the ask follows an interrupted act, whether the
    /// interruption fell inside the erasure unit or after its outcome landed.
    ///
    /// **What is lost is the explanation and never the passphrase**, which had
    /// already stopped working and was already gone. The ordinary case — the
    /// same act erasing and asking — is untouched and is
    /// [`Supplied::ThePassphraseStoppedWorking`]'s own ask above.
    fn an_earlier_erasure_is_on_the_record(&self, _row: &KeyRow) -> bool {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::record::LocalRecord;
    use smoozy_security_and_licence::held::HeldSecret;
    use smoozy_security_and_licence::secret_store::{ProbeOutcome, StoreFailure};
    use std::cell::RefCell;
    use std::path::{Path, PathBuf};

    fn scratch(name: &str) -> PathBuf {
        let mut path = std::env::temp_dir();
        path.push(format!(
            "smoozy-for-a-connection-{name}-{}",
            std::process::id()
        ));
        let _ = std::fs::remove_dir_all(&path);
        std::fs::create_dir_all(&path).expect("a scratch directory");
        path
    }

    /// A store holding whatever a test planted in it.
    #[derive(Default)]
    struct AStore {
        holds: RefCell<Vec<(HeldSecret, Vec<u8>)>>,
        get_fails_as: Option<GetFailure>,
        /// **`Security and licence` S24's round trip, as this store
        /// answers it.** The default is a store that answers, which is every
        /// computer this project has.
        the_probe_fails: bool,
    }

    impl SecretStore for AStore {
        fn put(
            &self,
            item: &HeldSecret,
            value: Plaintext,
            _deadline: Duration,
        ) -> Result<(), StoreFailure> {
            let bytes = value.expose(<[u8]>::to_vec);
            self.holds.borrow_mut().push((item.clone(), bytes));
            Ok(())
        }

        fn get(&self, item: &HeldSecret, _deadline: Duration) -> Result<Plaintext, GetFailure> {
            if let Some(failure) = self.get_fails_as {
                return Err(failure);
            }
            self.holds
                .borrow()
                .iter()
                .find(|(held, _)| held == item)
                .map(|(_, bytes)| Plaintext::new(bytes.clone()))
                .ok_or(GetFailure::NotThere)
        }

        fn erase(&self, item: &HeldSecret, _deadline: Duration) -> Result<(), StoreFailure> {
            self.holds.borrow_mut().retain(|(held, _)| held != item);
            Ok(())
        }

        fn probe(&self, _deadline: Duration) -> ProbeOutcome {
            if self.the_probe_fails {
                ProbeOutcome::RoundTripFailed
            } else {
                ProbeOutcome::RoundTripWorked
            }
        }

        fn enumerate(&self, _deadline: Duration) -> Result<Vec<HeldSecret>, StoreFailure> {
            Ok(self.holds.borrow().iter().map(|(held, _)| held.clone()).collect())
        }
    }

    fn a_record(at: &Path) -> LocalRecord {
        LocalRecord::open_at(&at.join("local-record.sqlite3")).expect("a record")
    }

    /// A real key, made by the real tool with no passphrase. **`Keys` G6: a
    /// computer with no generator has nothing to prove here**, which is what
    /// the `None` says.
    fn a_plain_key(area: &Path) -> Option<PathBuf> {
        let at = area.join("id_ed25519");
        let made = std::process::Command::new("ssh-keygen")
            .args(["-t", "ed25519", "-N", "", "-C", "smoozy-test", "-f"])
            .arg(&at)
            .stdin(std::process::Stdio::null())
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .status()
            .ok()?;
        made.success().then_some(at)
    }

    fn a_row_for(at: &Path) -> KeyRow {
        KeyRow::entering(
            Identity::issue(IdentityKind::LocalKey).expect("randomness"),
            at.to_path_buf(),
            None,
            None,
            false,
        )
        .expect("a row")
    }

    fn a_route(area: &Path) -> TheRoute {
        TheRoute {
            channel_area: area.to_path_buf(),
            helper: PathBuf::from("/nonexistent/no-load-should-run-from-this-test"),
        }
    }

    fn the_reference(row: &KeyRow, fingerprint: &str) -> KeyReference {
        KeyReference {
            local_key_identifier: row.identifier.as_text(),
            fingerprint: fingerprint.to_string(),
        }
    }

    /// The fingerprint the real tool reads off a real key.
    fn the_fingerprint_of(row: &KeyRow, area: &Path) -> String {
        let readings = OnThisComputer::over(area.to_path_buf());
        usable::check(row, &readings)
            .fingerprint_now
            .expect("the tool read a fingerprint")
            .value
            .as_str()
            .to_string()
    }

    /// **`Servers` P1's second question carries its own condition, and this is
    /// the composition the plan left to this seam.**
    ///
    /// **It fails against the obvious wrong answer** — handing `Keys` U9's bare
    /// availability answer over — which would say *no passphrase is available*
    /// for a key that needs none, and `Servers` P3 would then withhold every
    /// automatic connection made with an unprotected key.
    #[test]
    fn a_key_that_needs_no_passphrase_answers_yes_to_the_second_question() {
        let area = scratch("plain");
        let Some(at) = a_plain_key(&area) else {
            return;
        };
        let row = a_row_for(&at);
        let record = a_record(&area);
        key_rows::add(record.connection(), &row).expect("the row");
        let fingerprint = the_fingerprint_of(&row, &area);

        let lent = crate::keys::TheRecordItself::over(record.connection());
        let store = AStore::default();
        let this_run = crate::the_store_standing::TheProbeThisRun::new();
        let mut supply = TheCredentialForAConnection::over(
            &lent,
            &store,
            a_route(&area),
            Duration::from_secs(5),
            &this_run,
        );

        let answers = supply.the_two_answers(&the_reference(&row, &fingerprint));
        assert!(answers.the_key_is_usable);
        // **`Keys` U9 on its own says no here** — the row holds no reference —
        // and the composition is what makes the answer right.
        assert!(!usable::a_passphrase_is_available_without_asking(&row));
        assert!(answers.a_passphrase_is_available_without_asking);
        assert!(answers.an_attempt_may_be_made());

        let _ = std::fs::remove_dir_all(&area);
    }

    /// **`Servers` P1a: the pair, and the fingerprint half is what catches a
    /// replacement.**
    #[test]
    fn a_reference_whose_fingerprint_has_moved_is_not_usable() {
        let area = scratch("moved");
        let Some(at) = a_plain_key(&area) else {
            return;
        };
        let row = a_row_for(&at);
        let record = a_record(&area);
        key_rows::add(record.connection(), &row).expect("the row");

        let lent = crate::keys::TheRecordItself::over(record.connection());
        let store = AStore::default();
        let this_run = crate::the_store_standing::TheProbeThisRun::new();
        let mut supply = TheCredentialForAConnection::over(
            &lent,
            &store,
            a_route(&area),
            Duration::from_secs(5),
            &this_run,
        );

        let answers = supply.the_two_answers(&the_reference(&row, "SHA256:somebody-elses-key"));
        assert!(
            !answers.the_key_is_usable,
            "a server row pointing at a fingerprint this key no longer has was called usable"
        );

        let _ = std::fs::remove_dir_all(&area);
    }

    /// **`Servers` P2: every attempt takes its own check, and a key can go away
    /// between one and the next.**
    ///
    /// **This test caught a real fault in this file and is why it exists.** The
    /// reading was first kept for the whole life of the supply, keyed on the key
    /// row's identifier — so a repair run of eight attempts against one server
    /// would have answered all eight from a check taken at the first, and a key
    /// deleted halfway through would have gone on reading as usable for 105
    /// seconds. **It is the exact value P2 names.**
    #[test]
    fn a_key_that_goes_away_between_two_attempts_is_read_again() {
        let area = scratch("again");
        let Some(at) = a_plain_key(&area) else {
            return;
        };
        let row = a_row_for(&at);
        let record = a_record(&area);
        key_rows::add(record.connection(), &row).expect("the row");
        let reference = the_reference(&row, &the_fingerprint_of(&row, &area));

        let lent = crate::keys::TheRecordItself::over(record.connection());
        let store = AStore::default();
        let this_run = crate::the_store_standing::TheProbeThisRun::new();
        let mut supply = TheCredentialForAConnection::over(
            &lent,
            &store,
            a_route(&area),
            Duration::from_secs(5),
            &this_run,
        );

        assert!(supply.the_two_answers(&reference).the_key_is_usable);
        std::fs::remove_file(&at).expect("the key goes");
        assert!(
            !supply.the_two_answers(&reference).the_key_is_usable,
            "**the second attempt answered from the first attempt's reading**"
        );

        let _ = std::fs::remove_dir_all(&area);
    }

    /// **`Servers` P4 and `Keys` S13: a connection Smoozy makes on its own may
    /// not ask, and the box is not raised for it.**
    #[test]
    fn only_a_connect_the_user_started_raises_the_box() {
        let area = scratch("whoasks");
        let Some(at) = a_plain_key(&area) else {
            return;
        };
        // A row that says it is protected without being one is not what is
        // wanted; what is wanted is a real protected key, and that needs the
        // helper. **So this test asks the question one level down**, at the
        // decision `Keys` S14 makes, with the row carrying no held passphrase.
        let mut row = a_row_for(&at);
        row.passphrase = None;
        let record = a_record(&area);
        key_rows::add(record.connection(), &row).expect("the row");

        let lent = crate::keys::TheRecordItself::over(record.connection());
        let store = AStore::default();
        let this_run = crate::the_store_standing::TheProbeThisRun::new();
        let mut supply = TheCredentialForAConnection::over(
            &lent,
            &store,
            a_route(&area),
            Duration::from_secs(5),
            &this_run,
        );
        let reference = the_reference(&row, &the_fingerprint_of(&row, &area));
        let naming = TheServerBeingConnectedTo {
            label: Some("hetzner"),
            username: "root",
            host: "5.180.183.77",
        };

        // The key needs no passphrase, so both answers come back with a
        // credential and no box either way.
        for who in [WhoStartedIt::TheUser, WhoStartedIt::SmoozyOnItsOwn] {
            let credential = supply.for_one_attempt(&reference, who, naming);
            assert!(credential.is_some());
            assert!(supply.the_ask.is_none());
        }

        let _ = std::fs::remove_dir_all(&area);
    }

    /// **`Servers` N8b: both halves, and neither is derivable from the two
    /// answers.**
    ///
    /// **The standing is this run's probe applied to what the last run left**,
    /// and until step 10b piece 3a nothing in Smoozy wrote the value at all —
    /// so this answer was *never probed* for the life of an installation and
    /// the permanent refusal could never fire. **Both halves are driven here**:
    /// each of the three readings a previous run could have left, against a
    /// store that answers and a store that does not.
    /// **Step 10b's first audit round, item 7: `Security and licence` S34's
    /// sentence is owed to the first connect in the run that finds there is
    /// nowhere to keep a passphrase, and to no second one.** The supply used to
    /// throw the writer's *say it once* away, and the moment it answers
    /// happens once for the life of an installation. **A computer whose store
    /// answers owes nothing.**
    #[test]
    fn the_once_said_sentence_is_owed_to_the_first_connect_and_never_a_second() {
        let area = scratch("say-it-once");
        let record = a_record(&area);
        let reference = KeyReference {
            local_key_identifier: "key-1".into(),
            fingerprint: "SHA256:aaa".into(),
        };
        let lent = crate::keys::TheRecordItself::over(record.connection());
        let nowhere = AStore {
            the_probe_fails: true,
            ..AStore::default()
        };
        let this_run = crate::the_store_standing::TheProbeThisRun::new();
        let mut first = TheCredentialForAConnection::over(
            &lent,
            &nowhere,
            a_route(&area),
            Duration::from_secs(5),
            &this_run,
        );
        let _ = first.the_refusal_is_permanent(&reference);
        assert!(first.the_sentence_is_owed, "the first connect is told to say it");
        let mut second = TheCredentialForAConnection::over(
            &lent,
            &nowhere,
            a_route(&area),
            Duration::from_secs(5),
            &this_run,
        );
        let _ = second.the_refusal_is_permanent(&reference);
        assert!(!second.the_sentence_is_owed, "**and no second one**");

        // The next run finds the standing already moved: never said again.
        let next_run = crate::the_store_standing::TheProbeThisRun::new();
        let mut later = TheCredentialForAConnection::over(
            &lent,
            &nowhere,
            a_route(&area),
            Duration::from_secs(5),
            &next_run,
        );
        let _ = later.the_refusal_is_permanent(&reference);
        assert!(!later.the_sentence_is_owed, "once for the life of the installation");

        // A computer whose store answers owes nothing.
        let other = scratch("say-it-once-answers");
        let record = a_record(&other);
        let lent = crate::keys::TheRecordItself::over(record.connection());
        let answers = AStore::default();
        let fresh = crate::the_store_standing::TheProbeThisRun::new();
        let mut supply = TheCredentialForAConnection::over(
            &lent,
            &answers,
            a_route(&other),
            Duration::from_secs(5),
            &fresh,
        );
        let _ = supply.the_refusal_is_permanent(&reference);
        assert!(!supply.the_sentence_is_owed);
        let _ = std::fs::remove_dir_all(&area);
        let _ = std::fs::remove_dir_all(&other);
    }

    #[test]
    fn the_permanent_refusal_reads_the_standing_and_never_guesses_it() {
        let area = scratch("n8b");
        let Some(at) = a_plain_key(&area) else {
            return;
        };
        let row = a_row_for(&at);
        let record = a_record(&area);
        key_rows::add(record.connection(), &row).expect("the row");
        let reference = the_reference(&row, &the_fingerprint_of(&row, &area));

        // Each case is: what a previous run left, whether this run's probe
        // round-trips, and what N8b's first half then reads.
        for (left_behind, the_probe_fails, reads_never_answered) in [
            // **The case this whole piece exists for.** A fresh installation on
            // a computer with nowhere to remember: the probe fails, `Security and
            // licence` S32's first
            // transition lands, and the refusal becomes permanent. **Written
            // before the store standing had a writer this read *never probed*
            // and the repair spun for ever.**
            (None, true, true),
            // The same computer at the next run: the standing holds.
            (Some("never answered"), true, true),
            // A user who installed a keyring between two sittings — `Security and
            // licence` S26's own
            // reason for re-taking the probe every run.
            (Some("never answered"), false, false),
            // **`Security and licence` S25 and `Security and licence` S35: a store
            // that answered once is never re-classified
            // as nowhere**, however the probe goes.
            (Some("has answered"), true, false),
            (Some("has answered"), false, false),
            // A fresh installation whose store answers.
            (None, false, false),
        ] {
            record
                .connection()
                .execute("DELETE FROM store_standing", [])
                .expect("the row goes");
            if let Some(standing) = left_behind {
                record
                    .connection()
                    .execute(
                        "INSERT INTO store_standing (only_row, standing) VALUES (1, ?1)",
                        [standing],
                    )
                    .expect("the standing");
            }
            let lent = crate::keys::TheRecordItself::over(record.connection());
            let store = AStore {
                the_probe_fails,
                ..AStore::default()
            };
            let this_run = crate::the_store_standing::TheProbeThisRun::new();
            let mut supply = TheCredentialForAConnection::over(
                &lent,
                &store,
                a_route(&area),
                Duration::from_secs(5),
                &this_run,
            );
            let refusal = supply.the_refusal_is_permanent(&reference);
            assert_eq!(
                refusal.the_store_standing_reads_never_answered, reads_never_answered,
                "the last run left {left_behind:?} and this run's probe \
                 {}",
                if the_probe_fails { "failed" } else { "worked" }
            );
            // **The key is not protected**, so the refusal never stops the
            // repair however the standing reads — which is N8b's own *both
            // halves* and `Keys` S10's mark.
            assert!(!refusal.the_rows_key_is_protected);
            assert!(!refusal.stops_the_repair());
        }

        let _ = std::fs::remove_dir_all(&area);
    }
}
