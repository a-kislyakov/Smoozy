//! **E19's trust set: from the recovery keys a build carries to the online
//! keys a manifest may be signed by.**
//!
//! **The chain, in one paragraph.** A build carries two recovery public keys
//! (E19, from 2026-10-24) and nothing else it trusts on its own. Every online
//! key a manifest may be signed by is named by a **trust statement**: the whole
//! set of accepted online keys, with every revocation so far, at a trust
//! sequence that only goes up, signed by **either** recovery key. A copy holds
//! the newest statement it has accepted and moves to a newer one only where a
//! recovery key still in force signed it. **The manifest never names a key of
//! its own** — it names identifiers, and an identifier means a key only where
//! the statement in force says so.
//!
//! **E19's three statements.**
//!
//! - **Transition.** The next online key is added beside the current one, and
//!   manifests carry both signatures through the overlap. A copy that missed
//!   the overlap moves straight to the newest statement, because each statement
//!   carries the whole set and every one is kept on hosting.
//! - **Loss.** The current key is gone, so there is no overlap: a prepared key
//!   is put in force by the recovery signature alone.
//! - **Compromise.** The compromised identifier is revoked and a replacement put
//!   in force. **A revocation is kept for ever**: every later statement must
//!   carry it, and no later statement may put that identifier back.
//!
//! **Two recovery keys, and what they do here.** Either one alone signs a
//! statement. A statement may revoke a recovery key as well as an online key,
//! and **a statement is never admitted under a recovery key it revokes, or one
//! an accepted statement has revoked** — E19's *"a revoked key cannot
//! authorize its own recovery"*. An online key never signs a statement at all,
//! so for an online key that rule holds by there being no route. **What two
//! keys do not do** is written at E19: one key is still enough to sign, so a
//! copy that meets a thief's statement before the other key's revocation can
//! accept it.
//!
//! **The monotonic trust sequence.** A copy stores the highest it has accepted
//! and **refuses a statement at or below it, however good its signature** — at
//! or below, because two different statements at one sequence would leave the
//! copy choosing.
//!
//! **What is kept, and why it is worth anything.** `State, identity and
//! recovery` J6's desktop-update trust record holds the greatest accepted
//! release sequence, the greatest accepted trust sequence and the statement
//! accepted at it, with its recovery signature. **`State, identity and
//! recovery` J7: before either number refuses anything, the kept statement is
//! verified again against the recovery keys this build carries**, and a record
//! whose statement will not verify, or does not carry the number the record
//! claims, is discarded whole — [`what_the_record_is_worth`].
//!
//! **No recovery key is carried yet**, and [`THE_RECOVERY_KEYS_THIS_BUILD_CARRIES`]
//! says why and what the day they are made changes.
//!
//! **How a clause marker is written in this crate, and it is a rule rather than
//! a habit.** A marker with no document beside it means a clause of this
//! crate's own plan, `Preparation/Architecture/Desktop app and delivery.md`.
//! Every other document's marker is written with that document at the site.

use smoozy_server_protocol::release::{
    a_signature_verifies, Canonical, PublicKeyBytes, Reading, SignatureBytes,
};

use super::THE_STATEMENT_SIGNING_CONTEXT;

/// The trust statement format this build composes and reads.
pub const THE_STATEMENT_FORMAT: &str = "smoozy-desktop-update-trust-statement-1";

// ------------------------------------------------- what this build carries

/// **One recovery key a build carries: its public half, its name, and one
/// public proof that somebody holds its other half.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ACarriedRecoveryKey {
    pub identifier: &'static str,
    pub public_key: PublicKeyBytes,
    /// **A signature the secret half made once, over
    /// [`THE_RECOVERY_PROOF_MESSAGE`], under
    /// [`super::THE_RECOVERY_PROOF_CONTEXT`].** It gives nothing away about the
    /// key; what it is worth is that thirty-two bytes nobody holds the other
    /// half of, or a mistyped key, fails a test the day it is written here
    /// rather than at the first update months later. The server package's root
    /// carries the same kind of proof (`smoozy_server_protocol::release::trust`).
    pub proof: SignatureBytes,
}

/// The exact bytes each recovery key signs once, when it is made.
pub const THE_RECOVERY_PROOF_MESSAGE: &[u8] =
    b"a Smoozy desktop-update recovery key, proved once when it was made";

/// **The recovery keys this build carries — none.**
///
/// **E19 asks for two, from the first build that can update itself, and both
/// are the Creator's.** A pair is made from the operating system's own
/// randomness; the secret halves go where the Creator keeps them, in two
/// different places, one of them offline; and only then are the two public
/// halves and their proofs written here. **Until that day this list is empty
/// on purpose**: a recovery key made by anybody in this project and written
/// here would be a trust set anybody reading the repository could forge, and
/// that is the one way this module could go badly wrong.
///
/// **The test `this_build_carries_no_recovery_key_until_the_creator_holds_them`
/// fails the day anything is put here** — the precedent of 2026-09-06, when
/// the server package's root was kept absent the same way
/// (`Audits/Build/Step 8a/Decisions record.md`). **On the day the Creator's two
/// keys are written here, that one test is deleted and nothing else**: the
/// tests beside it already require exactly two, distinct, each with a proof
/// that verifies, none of them a key any written-down seed in this workspace
/// makes, and none of them the server package's root.
pub const THE_RECOVERY_KEYS_THIS_BUILD_CARRIES: &[ACarriedRecoveryKey] = &[];

/// **A trust statement a build carries, as the bytes that were signed.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ACarriedStatement {
    pub bytes: &'static [u8],
    pub its_recovery_signature: SignatureBytes,
}

/// **The online key set this build carries — none, because no online key
/// exists.**
///
/// E19's *"the current accepted online signing-key set"* is carried as the
/// recovery-signed statement that names it, so a build's own set is checked
/// the same way as a fetched one and a record always has a statement to keep.
/// **It is written by the release that makes the first online key** (step
/// 11's release piece), after the recovery keys above.
pub const THE_STATEMENT_THIS_BUILD_CARRIES: Option<ACarriedStatement> = None;

/// **The greatest update release sequence published when this build was made
/// — zero, because none has been.**
///
/// It is the floor a copy that remembers nothing is held to, the way the
/// server package's built-in pair is (`Server protocol and transfers` B9):
/// without it, every fresh install and every record `State, identity and
/// recovery` J7 discards would accept any genuine manifest ever signed. **It is
/// raised by hand to the last published release's own sequence**, never to a
/// value derived from the build.
pub const THE_GREATEST_RELEASE_SEQUENCE_PUBLISHED_WHEN_THIS_BUILD_WAS_MADE: u64 = 0;

/// One recovery key, as the trust set uses it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ARecoveryKey {
    pub identifier: String,
    pub public_key: PublicKeyBytes,
}

/// **Everything a build brings to the chain, as one value.**
///
/// **The rule is held in the tests through this rather than through the
/// constants**, because a test that states its own carried set goes on meaning
/// the same thing on the day the Creator's keys are written above.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TheCarriedSet {
    pub recovery_keys: Vec<ARecoveryKey>,
    /// The statement the build carries, as bytes and recovery signature.
    pub statement: Option<(Vec<u8>, SignatureBytes)>,
    pub greatest_release_sequence_published: u64,
}

impl TheCarriedSet {
    /// What this build carries.
    pub fn this_build() -> TheCarriedSet {
        TheCarriedSet {
            recovery_keys: THE_RECOVERY_KEYS_THIS_BUILD_CARRIES
                .iter()
                .map(|key| ARecoveryKey {
                    identifier: key.identifier.to_string(),
                    public_key: key.public_key,
                })
                .collect(),
            statement: THE_STATEMENT_THIS_BUILD_CARRIES
                .map(|carried| (carried.bytes.to_vec(), carried.its_recovery_signature)),
            greatest_release_sequence_published:
                THE_GREATEST_RELEASE_SEQUENCE_PUBLISHED_WHEN_THIS_BUILD_WAS_MADE,
        }
    }
}

// --------------------------------------------------------- the statement

/// **Which of E19's three statements this is.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhichStatement {
    Transition,
    Loss,
    Compromise,
}

impl WhichStatement {
    fn as_number(self) -> u32 {
        match self {
            WhichStatement::Transition => 1,
            WhichStatement::Loss => 2,
            WhichStatement::Compromise => 3,
        }
    }

    fn from_number(number: u32) -> Option<WhichStatement> {
        match number {
            1 => Some(WhichStatement::Transition),
            2 => Some(WhichStatement::Loss),
            3 => Some(WhichStatement::Compromise),
            _ => None,
        }
    }
}

/// **One online key a statement puts in force.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AnOnlineKey {
    pub identifier: String,
    pub public_key: PublicKeyBytes,
    /// **E19's activation sequence: the first release sequence this key may
    /// sign.** A manifest below it is not this key's to sign.
    pub first_release_sequence: u64,
}

/// **A trust statement, every field of it.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TrustStatement {
    pub statement_format: String,
    /// **E19's monotonically increasing trust sequence.**
    pub trust_sequence: u64,
    pub which: WhichStatement,
    /// **The whole set in force after it**, never a change to the one before.
    pub online_keys: Vec<AnOnlineKey>,
    /// Every online identifier revoked so far, this statement's own included.
    pub revoked_online_keys: Vec<String>,
    /// Every recovery identifier revoked so far, this statement's own included.
    pub revoked_recovery_keys: Vec<String>,
}

impl TrustStatement {
    /// Read a statement from the exact bytes that were signed. **Nothing is
    /// guessed at**: a short read, a trailing byte, an unknown kind, another
    /// format, or one identifier named twice in one list all give nothing.
    pub fn read(bytes: &[u8]) -> Option<TrustStatement> {
        let statement = read_fields(bytes)?;
        if statement.statement_format != THE_STATEMENT_FORMAT {
            return None;
        }
        fn distinct<'a>(names: impl Iterator<Item = &'a String>) -> bool {
            let mut seen = std::collections::HashSet::new();
            names.into_iter().all(|name| seen.insert(name))
        }
        let all_distinct = distinct(statement.online_keys.iter().map(|key| &key.identifier))
            && distinct(statement.revoked_online_keys.iter())
            && distinct(statement.revoked_recovery_keys.iter());
        all_distinct.then_some(statement)
    }

    /// The canonical bytes. **The release side's function** and the tests'.
    pub fn canonical_bytes(&self) -> Vec<u8> {
        let mut out = Canonical::new();
        out.text(&self.statement_format)
            .u64(self.trust_sequence)
            .u32(self.which.as_number())
            .u32(self.online_keys.len() as u32);
        for key in &self.online_keys {
            out.text(&key.identifier)
                .digest(&key.public_key)
                .u64(key.first_release_sequence);
        }
        out.u32(self.revoked_online_keys.len() as u32);
        for identifier in &self.revoked_online_keys {
            out.text(identifier);
        }
        out.u32(self.revoked_recovery_keys.len() as u32);
        for identifier in &self.revoked_recovery_keys {
            out.text(identifier);
        }
        out.finish()
    }

    /// **The key this statement puts in force under that identifier, where it
    /// may sign at this release sequence.** Revoked, not yet active, or not
    /// named: none.
    pub fn the_key_that_may_sign(
        &self,
        identifier: &str,
        release_sequence: u64,
    ) -> Option<&AnOnlineKey> {
        if self
            .revoked_online_keys
            .iter()
            .any(|revoked| revoked == identifier)
        {
            return None;
        }
        self.online_keys.iter().find(|key| {
            key.identifier == identifier && key.first_release_sequence <= release_sequence
        })
    }
}

fn read_fields(bytes: &[u8]) -> Option<TrustStatement> {
    let mut reading = Reading::of(bytes);
    let statement_format = reading.text()?;
    let trust_sequence = reading.u64()?;
    let which = WhichStatement::from_number(reading.u32()?)?;
    let how_many = reading.u32()?;
    let mut online_keys = Vec::new();
    for _ in 0..how_many {
        online_keys.push(AnOnlineKey {
            identifier: reading.text()?,
            public_key: reading.digest()?,
            first_release_sequence: reading.u64()?,
        });
    }
    let how_many = reading.u32()?;
    let mut revoked_online_keys = Vec::new();
    for _ in 0..how_many {
        revoked_online_keys.push(reading.text()?);
    }
    let how_many = reading.u32()?;
    let mut revoked_recovery_keys = Vec::new();
    for _ in 0..how_many {
        revoked_recovery_keys.push(reading.text()?);
    }
    reading.end()?;
    Some(TrustStatement {
        statement_format,
        trust_sequence,
        which,
        online_keys,
        revoked_online_keys,
        revoked_recovery_keys,
    })
}

/// **Which recovery key, of those given, signed these bytes**, skipping every
/// identifier in `revoked`. Each candidate is tried; nothing outside the signed
/// bytes says which one to try.
fn the_recovery_key_that_signed<'a>(
    recovery_keys: &'a [ARecoveryKey],
    revoked: &[String],
    bytes: &[u8],
    signature: &SignatureBytes,
) -> Option<&'a ARecoveryKey> {
    recovery_keys
        .iter()
        .filter(|key| !revoked.contains(&key.identifier))
        .find(|key| {
            a_signature_verifies(
                &key.public_key,
                THE_STATEMENT_SIGNING_CONTEXT,
                bytes,
                signature,
            )
        })
}

/// **A statement admitted against the carried recovery keys alone**, as it
/// would be re-verified: signed by a carried key the statement itself does not
/// revoke. Used for the build's own statement and for the kept one.
fn verified_against_the_carried_keys(
    carried: &TheCarriedSet,
    bytes: &[u8],
    signature: &SignatureBytes,
) -> Option<TrustStatement> {
    let statement = TrustStatement::read(bytes)?;
    the_recovery_key_that_signed(
        &carried.recovery_keys,
        &statement.revoked_recovery_keys,
        bytes,
        signature,
    )?;
    Some(statement)
}

// ------------------------------------------------------------ the record

/// **`State, identity and recovery` J6's desktop-update trust record, and its
/// fields are that row's.**
///
/// The statement is kept as the bytes that were signed, with the signature
/// beside it, so verifying it again verifies what was verified the first time.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TheUpdateTrustRecord {
    pub greatest_accepted_release_sequence: u64,
    pub greatest_accepted_trust_sequence: u64,
    pub the_statement_accepted_at_that_trust_sequence: Vec<u8>,
    pub its_recovery_signature: SignatureBytes,
}

/// **What a kept record turns out to be worth — `State, identity and
/// recovery` J7.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WhatTheRecordIsWorth {
    /// The kept statement verified against a carried recovery key, and the
    /// stored trust sequence is its own: both numbers stand.
    ItsNumbersStand {
        greatest_accepted_release_sequence: u64,
        the_statement: TrustStatement,
    },
    /// **Absent, malformed, will not verify, or claims a trust sequence that is
    /// not its statement's own — discarded whole.** The floor the build carries
    /// still stands; the next genuine statement re-establishes the record on its
    /// own merits. **`State, identity and recovery` J8: a correction and not a
    /// weakening**, since raising a number still needs a statement a recovery
    /// key signed.
    ///
    /// **One case is discarded that is not damage, and it is safe.** A kept
    /// statement signed by a recovery key this build no longer carries — a
    /// newer build that dropped a revoked key — will not verify here. Nothing is
    /// lost: the build's own statement is at least as new, and the next update
    /// file carries the rest.
    ItIsDiscarded,
}

/// **`State, identity and recovery` J7, performed.** Called before a stored
/// number refuses anything.
pub fn what_the_record_is_worth(
    carried: &TheCarriedSet,
    remembered: Option<&TheUpdateTrustRecord>,
) -> WhatTheRecordIsWorth {
    let Some(record) = remembered else {
        return WhatTheRecordIsWorth::ItIsDiscarded;
    };
    let Some(statement) = verified_against_the_carried_keys(
        carried,
        &record.the_statement_accepted_at_that_trust_sequence,
        &record.its_recovery_signature,
    ) else {
        return WhatTheRecordIsWorth::ItIsDiscarded;
    };
    // **The recovery signature covers the statement and no field of the
    // record**, so a stored trust sequence that is not the statement's own can
    // only be damage, and it is discarded as damage.
    if record.greatest_accepted_trust_sequence != statement.trust_sequence {
        return WhatTheRecordIsWorth::ItIsDiscarded;
    }
    WhatTheRecordIsWorth::ItsNumbersStand {
        greatest_accepted_release_sequence: record.greatest_accepted_release_sequence,
        the_statement: statement,
    }
}

// --------------------------------------------------------- the chain

/// **The trust in force: where the chain stands before an update file is
/// read, and after each statement in it is admitted.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TheTrustInForce {
    /// The recovery keys this build carries. Which of them may still sign is
    /// [`TheTrustInForce::revoked_recovery_keys`]'s to say.
    pub recovery_keys: Vec<ARecoveryKey>,
    /// **The statement in force, with the bytes and signature it arrived as**,
    /// or none where no statement this build can verify has ever been met.
    pub statement: Option<(TrustStatement, Vec<u8>, SignatureBytes)>,
    /// **The floor a manifest's release sequence is held to**: the greater of
    /// what this build carries and what the kept record stands on.
    pub the_release_sequence_floor: u64,
}

impl TheTrustInForce {
    /// The trust sequence in force; zero where no statement is.
    pub fn trust_sequence(&self) -> u64 {
        self.statement
            .as_ref()
            .map_or(0, |(statement, _, _)| statement.trust_sequence)
    }

    /// Every recovery identifier revoked so far.
    pub fn revoked_recovery_keys(&self) -> &[String] {
        self.statement.as_ref().map_or(&[], |(statement, _, _)| {
            statement.revoked_recovery_keys.as_slice()
        })
    }

    fn revoked_online_keys(&self) -> &[String] {
        self.statement.as_ref().map_or(&[], |(statement, _, _)| {
            statement.revoked_online_keys.as_slice()
        })
    }

    /// **Where the chain starts: the newer of the build's own statement and the
    /// kept one, each verified against the carried recovery keys.**
    ///
    /// The kept record is judged by `State, identity and recovery` J7 first; a
    /// discarded one contributes neither its statement nor its release
    /// sequence. **A build whose own statement will not verify contributes no
    /// statement** — a build like that is broken, and the test beside the
    /// constant fails on it.
    pub fn at_the_start(
        carried: &TheCarriedSet,
        remembered: Option<&TheUpdateTrustRecord>,
    ) -> TheTrustInForce {
        let mut statement = carried.statement.as_ref().and_then(|(bytes, signature)| {
            verified_against_the_carried_keys(carried, bytes, signature)
                .map(|statement| (statement, bytes.clone(), *signature))
        });
        let mut floor = carried.greatest_release_sequence_published;
        if let WhatTheRecordIsWorth::ItsNumbersStand {
            greatest_accepted_release_sequence,
            the_statement,
        } = what_the_record_is_worth(carried, remembered)
        {
            floor = floor.max(greatest_accepted_release_sequence);
            let kept_is_newer = statement.as_ref().is_none_or(|(the_builds, _, _)| {
                the_statement.trust_sequence > the_builds.trust_sequence
            });
            if kept_is_newer {
                let record = remembered.expect("a record that stands was remembered");
                statement = Some((
                    the_statement,
                    record.the_statement_accepted_at_that_trust_sequence.clone(),
                    record.its_recovery_signature,
                ));
            }
        }
        TheTrustInForce {
            recovery_keys: carried.recovery_keys.clone(),
            statement,
            the_release_sequence_floor: floor,
        }
    }

    /// **Admit one statement, in this order: newer, signed by a recovery key
    /// still in force, readable, not signed by a key it revokes, forgetting no
    /// revocation, putting no revoked key back, and being the statement it says
    /// it is.**
    ///
    /// The trust sequence is read before the signature is checked, and only to
    /// refuse: a statement at or below the one in force is refused without
    /// anything in it being acted on.
    pub fn admit(
        &self,
        bytes: &[u8],
        signature: &SignatureBytes,
    ) -> Result<TheTrustInForce, WhyTheStatementWasRefused> {
        let at = trust_sequence_of(bytes)
            .ok_or(WhyTheStatementWasRefused::ItIsNotAStatementThisBuildKnows)?;
        if self.statement.is_some() && at <= self.trust_sequence() {
            return Err(WhyTheStatementWasRefused::ItIsNotNewerThanTheTrustInForce);
        }
        let signer = the_recovery_key_that_signed(
            &self.recovery_keys,
            self.revoked_recovery_keys(),
            bytes,
            signature,
        )
        .ok_or(WhyTheStatementWasRefused::NoRecoveryKeyInForceSignedIt)?;
        let statement = TrustStatement::read(bytes)
            .ok_or(WhyTheStatementWasRefused::ItIsNotAStatementThisBuildKnows)?;
        if statement.revoked_recovery_keys.contains(&signer.identifier) {
            return Err(WhyTheStatementWasRefused::ItIsSignedByAKeyItRevokes);
        }
        let forgets =
            |kept: &[String], offered: &[String]| kept.iter().any(|one| !offered.contains(one));
        if forgets(self.revoked_online_keys(), &statement.revoked_online_keys)
            || forgets(
                self.revoked_recovery_keys(),
                &statement.revoked_recovery_keys,
            )
        {
            return Err(WhyTheStatementWasRefused::ItForgetsARevocation);
        }
        if statement
            .online_keys
            .iter()
            .any(|key| statement.revoked_online_keys.contains(&key.identifier))
        {
            return Err(WhyTheStatementWasRefused::ItPutsARevokedKeyInForce);
        }
        let revokes_something_new = statement.revoked_online_keys.len()
            > self.revoked_online_keys().len()
            || statement.revoked_recovery_keys.len() > self.revoked_recovery_keys().len();
        let as_it_says = match statement.which {
            WhichStatement::Compromise => revokes_something_new,
            WhichStatement::Transition | WhichStatement::Loss => !revokes_something_new,
        };
        if !as_it_says {
            return Err(WhyTheStatementWasRefused::ItIsNotTheStatementItSaysItIs);
        }
        Ok(TheTrustInForce {
            recovery_keys: self.recovery_keys.clone(),
            statement: Some((statement, bytes.to_vec(), *signature)),
            the_release_sequence_floor: self.the_release_sequence_floor,
        })
    }
}

/// A statement's trust sequence, read without trusting anything else in it.
pub(crate) fn trust_sequence_of(bytes: &[u8]) -> Option<u64> {
    let mut reading = Reading::of(bytes);
    (reading.text()? == THE_STATEMENT_FORMAT).then_some(())?;
    reading.u64()
}

/// **Why an offered statement was not admitted.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhyTheStatementWasRefused {
    /// **At or below the trust sequence in force** — E19's *"refuses an older
    /// statement even when its signature is otherwise valid"*.
    ItIsNotNewerThanTheTrustInForce,
    /// No recovery key this build carries and no accepted statement has revoked
    /// signed it: a forged signature, another root, or a revoked key.
    NoRecoveryKeyInForceSignedIt,
    /// It verified and it is not a statement of a format this build knows.
    ItIsNotAStatementThisBuildKnows,
    /// **Its only signer is a recovery key it revokes** — E19's *"a revoked key
    /// cannot authorize its own recovery"*.
    ItIsSignedByAKeyItRevokes,
    /// It leaves out a revocation the statement in force carries.
    ItForgetsARevocation,
    /// It puts in force an online identifier it, or an earlier one, revoked.
    ItPutsARevokedKeyInForce,
    /// **A compromise statement that revokes nothing new, or a transition or
    /// loss statement that does.**
    ItIsNotTheStatementItSaysItIs,
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use smoozy_server_protocol::release::a_signature_over;
    use smoozy_server_protocol::release::composing::test_only_keys::{
        a_key_pair_from_a_written_down_seed, AKeyPair,
    };

    // **Throwaway keys, made here from seeds written down here.** A key from a
    // written-down seed is public by construction and cannot be mistaken for
    // one of the Creator's.
    pub(crate) fn recovery_a() -> AKeyPair {
        a_key_pair_from_a_written_down_seed(*b"desktop update recovery A, tests")
    }
    pub(crate) fn recovery_b() -> AKeyPair {
        a_key_pair_from_a_written_down_seed(*b"desktop update recovery B, tests")
    }
    pub(crate) fn online_1() -> AKeyPair {
        a_key_pair_from_a_written_down_seed(*b"desktop update online key 1 test")
    }
    pub(crate) fn online_2() -> AKeyPair {
        a_key_pair_from_a_written_down_seed(*b"desktop update online key 2 test")
    }

    /// A build carrying the two test recovery keys and nothing else.
    pub(crate) fn a_build_with_both_recovery_keys() -> TheCarriedSet {
        TheCarriedSet {
            recovery_keys: vec![
                ARecoveryKey {
                    identifier: "recovery-a".into(),
                    public_key: recovery_a().public,
                },
                ARecoveryKey {
                    identifier: "recovery-b".into(),
                    public_key: recovery_b().public,
                },
            ],
            statement: None,
            greatest_release_sequence_published: 0,
        }
    }

    pub(crate) fn an_online_key(identifier: &str, key: &AKeyPair) -> AnOnlineKey {
        AnOnlineKey {
            identifier: identifier.into(),
            public_key: key.public,
            first_release_sequence: 1,
        }
    }

    pub(crate) fn a_statement(
        trust_sequence: u64,
        which: WhichStatement,
        online_keys: Vec<AnOnlineKey>,
    ) -> TrustStatement {
        TrustStatement {
            statement_format: THE_STATEMENT_FORMAT.into(),
            trust_sequence,
            which,
            online_keys,
            revoked_online_keys: Vec::new(),
            revoked_recovery_keys: Vec::new(),
        }
    }

    pub(crate) fn signed(by: &AKeyPair, statement: &TrustStatement) -> (Vec<u8>, SignatureBytes) {
        let bytes = statement.canonical_bytes();
        let signature = a_signature_over(&by.secret, THE_STATEMENT_SIGNING_CONTEXT, &bytes);
        (bytes, signature)
    }

    fn the_first() -> TrustStatement {
        a_statement(
            1,
            WhichStatement::Transition,
            vec![an_online_key("online-1", &online_1())],
        )
    }

    fn from_nothing() -> TheTrustInForce {
        TheTrustInForce::at_the_start(&a_build_with_both_recovery_keys(), None)
    }

    #[test]
    fn a_statement_reads_back_as_what_was_composed() {
        let statement = TrustStatement {
            revoked_online_keys: vec!["online-0".into()],
            revoked_recovery_keys: vec!["recovery-c".into()],
            ..a_statement(
                9,
                WhichStatement::Compromise,
                vec![an_online_key("online-1", &online_1())],
            )
        };
        assert_eq!(
            TrustStatement::read(&statement.canonical_bytes()).unwrap(),
            statement
        );
        assert_eq!(trust_sequence_of(&statement.canonical_bytes()), Some(9));
        let mut tail = statement.canonical_bytes();
        tail.push(0);
        assert!(TrustStatement::read(&tail).is_none());
    }

    #[test]
    fn either_recovery_key_alone_signs_a_statement_the_chain_admits() {
        for signer in [recovery_a(), recovery_b()] {
            let (bytes, signature) = signed(&signer, &the_first());
            let admitted = from_nothing().admit(&bytes, &signature).unwrap();
            assert_eq!(admitted.trust_sequence(), 1);
            let (statement, _, _) = admitted.statement.as_ref().unwrap();
            assert!(statement.the_key_that_may_sign("online-1", 1).is_some());
        }
    }

    #[test]
    fn a_bad_signature_or_another_root_is_refused() {
        let (bytes, mut signature) = signed(&recovery_a(), &the_first());
        signature[0] ^= 1;
        assert_eq!(
            from_nothing().admit(&bytes, &signature),
            Err(WhyTheStatementWasRefused::NoRecoveryKeyInForceSignedIt)
        );
        // **A key this build does not carry**, however good the signature.
        let somebody = a_key_pair_from_a_written_down_seed(*b"not a recovery key of this build");
        let (bytes, signature) = signed(&somebody, &the_first());
        assert_eq!(
            from_nothing().admit(&bytes, &signature),
            Err(WhyTheStatementWasRefused::NoRecoveryKeyInForceSignedIt)
        );
        // **An online key cannot sign a statement**, so a stolen online key
        // cannot vouch for anything.
        let (bytes, signature) = signed(&online_1(), &the_first());
        assert_eq!(
            from_nothing().admit(&bytes, &signature),
            Err(WhyTheStatementWasRefused::NoRecoveryKeyInForceSignedIt)
        );
    }

    /// **The server package's root, offered for an update, is refused** —
    /// `State, identity and recovery` J10. A server-package key-set statement
    /// signed under its own context by a root this build were to carry as a
    /// recovery key still does not verify here, because the contexts differ;
    /// and a build that carries no update recovery key does not reach for the
    /// server root in their place.
    #[test]
    fn the_server_packages_root_is_refused_for_an_update() {
        use smoozy_server_protocol::release as server_package;
        let server_root = a_key_pair_from_a_written_down_seed(*b"a server-package root, for tests");
        let (bytes, _) = signed(&recovery_a(), &the_first());
        let signed_as_a_server_document = a_signature_over(
            &server_root.secret,
            server_package::THE_KEY_SET_SIGNING_CONTEXT,
            &bytes,
        );
        let carrying_it = TheCarriedSet {
            recovery_keys: vec![ARecoveryKey {
                identifier: "the server root".into(),
                public_key: server_root.public,
            }],
            statement: None,
            greatest_release_sequence_published: 0,
        };
        assert_eq!(
            TheTrustInForce::at_the_start(&carrying_it, None)
                .admit(&bytes, &signed_as_a_server_document),
            Err(WhyTheStatementWasRefused::NoRecoveryKeyInForceSignedIt)
        );
        // And the real server-package root this build carries is no recovery
        // key of the update trust set, nor is anything in its place.
        let theirs = server_package::trust::THE_RELEASE_ROOT_THIS_BUILD_CARRIES
            .expect("the server root is carried");
        assert!(TheCarriedSet::this_build()
            .recovery_keys
            .iter()
            .all(|key| key.public_key != theirs));
        assert!(THE_RECOVERY_KEYS_THIS_BUILD_CARRIES
            .iter()
            .all(|key| key.public_key != theirs));
    }

    #[test]
    fn a_statement_at_or_below_the_trust_sequence_in_force_is_refused_however_well_signed() {
        let (bytes, signature) = signed(
            &recovery_a(),
            &a_statement(
                5,
                WhichStatement::Transition,
                vec![an_online_key("online-1", &online_1())],
            ),
        );
        let at_five = from_nothing().admit(&bytes, &signature).unwrap();
        for older in [4, 5] {
            let (bytes, signature) = signed(
                &recovery_b(),
                &a_statement(
                    older,
                    WhichStatement::Transition,
                    vec![an_online_key("online-2", &online_2())],
                ),
            );
            assert_eq!(
                at_five.admit(&bytes, &signature),
                Err(WhyTheStatementWasRefused::ItIsNotNewerThanTheTrustInForce)
            );
        }
        let (bytes, signature) = signed(
            &recovery_b(),
            &a_statement(
                6,
                WhichStatement::Transition,
                vec![an_online_key("online-2", &online_2())],
            ),
        );
        assert_eq!(
            at_five.admit(&bytes, &signature).unwrap().trust_sequence(),
            6
        );
    }

    #[test]
    fn a_transition_carries_both_keys_and_a_loss_moves_with_the_recovery_signature_alone() {
        let (bytes, signature) = signed(&recovery_a(), &the_first());
        let first = from_nothing().admit(&bytes, &signature).unwrap();
        let overlap = a_statement(
            2,
            WhichStatement::Transition,
            vec![
                an_online_key("online-1", &online_1()),
                an_online_key("online-2", &online_2()),
            ],
        );
        let (bytes, signature) = signed(&recovery_a(), &overlap);
        let both = first.admit(&bytes, &signature).unwrap();
        let (statement, _, _) = both.statement.as_ref().unwrap();
        assert!(statement.the_key_that_may_sign("online-1", 9).is_some());
        assert!(statement.the_key_that_may_sign("online-2", 9).is_some());
        // **Loss**: the current key is gone and a prepared one is put in force,
        // from the first statement straight to the third.
        let loss = a_statement(
            3,
            WhichStatement::Loss,
            vec![an_online_key("online-2", &online_2())],
        );
        let (bytes, signature) = signed(&recovery_b(), &loss);
        let after = first.admit(&bytes, &signature).unwrap();
        let (statement, _, _) = after.statement.as_ref().unwrap();
        assert!(statement.the_key_that_may_sign("online-1", 9).is_none());
        assert!(statement.the_key_that_may_sign("online-2", 9).is_some());
        // **The activation sequence**: below it the key may not sign.
        let later = TrustStatement {
            online_keys: vec![AnOnlineKey {
                first_release_sequence: 50,
                ..an_online_key("online-2", &online_2())
            }],
            ..a_statement(4, WhichStatement::Transition, Vec::new())
        };
        assert!(later.the_key_that_may_sign("online-2", 49).is_none());
        assert!(later.the_key_that_may_sign("online-2", 50).is_some());
    }

    #[test]
    fn a_compromised_keys_statement_revokes_it_for_good() {
        let (bytes, signature) = signed(&recovery_a(), &the_first());
        let first = from_nothing().admit(&bytes, &signature).unwrap();
        let compromise = TrustStatement {
            revoked_online_keys: vec!["online-1".into()],
            ..a_statement(
                2,
                WhichStatement::Compromise,
                vec![an_online_key("online-2", &online_2())],
            )
        };
        let (bytes, signature) = signed(&recovery_b(), &compromise);
        let revoked = first.admit(&bytes, &signature).unwrap();
        let (statement, _, _) = revoked.statement.as_ref().unwrap();
        assert!(statement.the_key_that_may_sign("online-1", 1).is_none());

        // **A later statement may not put it back**, by naming it in force…
        let put_back = TrustStatement {
            revoked_online_keys: vec!["online-1".into()],
            ..a_statement(
                3,
                WhichStatement::Transition,
                vec![an_online_key("online-1", &online_1())],
            )
        };
        let (bytes, signature) = signed(&recovery_a(), &put_back);
        assert_eq!(
            revoked.admit(&bytes, &signature),
            Err(WhyTheStatementWasRefused::ItPutsARevokedKeyInForce)
        );
        // …or by forgetting the revocation.
        let forgets = a_statement(
            3,
            WhichStatement::Transition,
            vec![an_online_key("online-1", &online_1())],
        );
        let (bytes, signature) = signed(&recovery_a(), &forgets);
        assert_eq!(
            revoked.admit(&bytes, &signature),
            Err(WhyTheStatementWasRefused::ItForgetsARevocation)
        );

        // **A compromise that revokes nothing new is not one, and a transition
        // that revokes something is not one either.**
        let empty_compromise = TrustStatement {
            revoked_online_keys: vec!["online-1".into()],
            ..a_statement(
                3,
                WhichStatement::Compromise,
                vec![an_online_key("online-2", &online_2())],
            )
        };
        let (bytes, signature) = signed(&recovery_a(), &empty_compromise);
        assert_eq!(
            revoked.admit(&bytes, &signature),
            Err(WhyTheStatementWasRefused::ItIsNotTheStatementItSaysItIs)
        );
        let revoking_transition = TrustStatement {
            revoked_online_keys: vec!["online-1".into()],
            ..a_statement(
                2,
                WhichStatement::Transition,
                vec![an_online_key("online-2", &online_2())],
            )
        };
        let (bytes, signature) = signed(&recovery_a(), &revoking_transition);
        assert_eq!(
            first.admit(&bytes, &signature),
            Err(WhyTheStatementWasRefused::ItIsNotTheStatementItSaysItIs)
        );
    }

    /// **A stolen recovery key is revoked by the other, and from then on
    /// signs nothing; and no key may revoke itself into a new set.**
    #[test]
    fn a_stolen_recovery_key_is_revoked_by_the_other_and_signs_nothing_after() {
        let (bytes, signature) = signed(&recovery_a(), &the_first());
        let first = from_nothing().admit(&bytes, &signature).unwrap();

        // A cannot sign the statement that revokes A.
        let a_revokes_a = TrustStatement {
            revoked_recovery_keys: vec!["recovery-a".into()],
            ..a_statement(
                2,
                WhichStatement::Compromise,
                vec![an_online_key("online-1", &online_1())],
            )
        };
        let (bytes, signature) = signed(&recovery_a(), &a_revokes_a);
        assert_eq!(
            first.admit(&bytes, &signature),
            Err(WhyTheStatementWasRefused::ItIsSignedByAKeyItRevokes)
        );

        // B can, and after it A signs nothing.
        let (bytes, signature) = signed(&recovery_b(), &a_revokes_a);
        let without_a = first.admit(&bytes, &signature).unwrap();
        assert_eq!(without_a.revoked_recovery_keys(), ["recovery-a"]);
        let thief = TrustStatement {
            revoked_recovery_keys: vec!["recovery-a".into(), "recovery-b".into()],
            ..a_statement(
                3,
                WhichStatement::Compromise,
                vec![an_online_key("online-2", &online_2())],
            )
        };
        let (bytes, signature) = signed(&recovery_a(), &thief);
        assert_eq!(
            without_a.admit(&bytes, &signature),
            Err(WhyTheStatementWasRefused::NoRecoveryKeyInForceSignedIt)
        );
        // B goes on alone.
        let next = TrustStatement {
            revoked_recovery_keys: vec!["recovery-a".into()],
            ..a_statement(
                3,
                WhichStatement::Transition,
                vec![an_online_key("online-2", &online_2())],
            )
        };
        let (bytes, signature) = signed(&recovery_b(), &next);
        assert_eq!(
            without_a
                .admit(&bytes, &signature)
                .unwrap()
                .trust_sequence(),
            3
        );
    }

    fn a_record(
        by: &AKeyPair,
        statement: &TrustStatement,
        release_sequence: u64,
    ) -> TheUpdateTrustRecord {
        let (bytes, signature) = signed(by, statement);
        TheUpdateTrustRecord {
            greatest_accepted_release_sequence: release_sequence,
            greatest_accepted_trust_sequence: statement.trust_sequence,
            the_statement_accepted_at_that_trust_sequence: bytes,
            its_recovery_signature: signature,
        }
    }

    #[test]
    fn a_kept_record_stands_only_while_its_statement_verifies_and_carries_its_number() {
        let build = a_build_with_both_recovery_keys();
        let sound = a_record(&recovery_b(), &the_first(), 40);
        assert!(matches!(
            what_the_record_is_worth(&build, Some(&sound)),
            WhatTheRecordIsWorth::ItsNumbersStand {
                greatest_accepted_release_sequence: 40,
                ..
            }
        ));
        let mut damaged = sound.clone();
        damaged.its_recovery_signature[3] ^= 1;
        assert_eq!(
            what_the_record_is_worth(&build, Some(&damaged)),
            WhatTheRecordIsWorth::ItIsDiscarded
        );
        let mut raised = sound.clone();
        raised.greatest_accepted_trust_sequence = 9_000;
        assert_eq!(
            what_the_record_is_worth(&build, Some(&raised)),
            WhatTheRecordIsWorth::ItIsDiscarded
        );
        let elsewhere = a_record(
            &a_key_pair_from_a_written_down_seed(*b"a key that signs a forged record"),
            &the_first(),
            40,
        );
        assert_eq!(
            what_the_record_is_worth(&build, Some(&elsewhere)),
            WhatTheRecordIsWorth::ItIsDiscarded
        );
        assert_eq!(
            what_the_record_is_worth(&build, None),
            WhatTheRecordIsWorth::ItIsDiscarded
        );
    }

    #[test]
    fn the_chain_starts_from_the_newer_of_the_builds_statement_and_the_kept_one() {
        let (bytes, signature) = signed(
            &recovery_a(),
            &a_statement(
                3,
                WhichStatement::Transition,
                vec![an_online_key("online-1", &online_1())],
            ),
        );
        let build = TheCarriedSet {
            statement: Some((bytes, signature)),
            greatest_release_sequence_published: 20,
            ..a_build_with_both_recovery_keys()
        };
        // Nothing kept: the build's own, and the build's floor.
        let start = TheTrustInForce::at_the_start(&build, None);
        assert_eq!(
            (start.trust_sequence(), start.the_release_sequence_floor),
            (3, 20)
        );
        // A kept record that is newer: its statement and its floor.
        let newer = a_record(
            &recovery_b(),
            &a_statement(
                5,
                WhichStatement::Transition,
                vec![an_online_key("online-2", &online_2())],
            ),
            30,
        );
        let start = TheTrustInForce::at_the_start(&build, Some(&newer));
        assert_eq!(
            (start.trust_sequence(), start.the_release_sequence_floor),
            (5, 30)
        );
        // A kept record that is older than the build: the build's statement,
        // and still the greater floor.
        let older = a_record(
            &recovery_b(),
            &a_statement(
                2,
                WhichStatement::Transition,
                vec![an_online_key("online-2", &online_2())],
            ),
            25,
        );
        let start = TheTrustInForce::at_the_start(&build, Some(&older));
        assert_eq!(
            (start.trust_sequence(), start.the_release_sequence_floor),
            (3, 25)
        );
        // A discarded record falls back to the build, floor and all.
        let mut damaged = newer.clone();
        damaged.its_recovery_signature[0] ^= 1;
        let start = TheTrustInForce::at_the_start(&build, Some(&damaged));
        assert_eq!(
            (start.trust_sequence(), start.the_release_sequence_floor),
            (3, 20)
        );
    }

    // ---------------------------------------- what this build carries

    /// **The 2026-09-06 precedent, for the update trust set: no recovery key
    /// and no statement is carried until the Creator holds the keys.** This
    /// test fails the day anything is written into either constant. **On the
    /// day the Creator's two keys are written there, this test is deleted and
    /// the ones below it are what holds** — they already ask for exactly two,
    /// each proved, none made from a written-down seed and neither the server
    /// package's root.
    #[test]
    fn this_build_carries_no_recovery_key_until_the_creator_holds_them() {
        assert!(
            THE_RECOVERY_KEYS_THIS_BUILD_CARRIES.is_empty(),
            "a recovery key is written into this build before the Creator holds it"
        );
        assert!(
            THE_STATEMENT_THIS_BUILD_CARRIES.is_none(),
            "a trust statement is written into this build before any recovery key is"
        );
        assert_eq!(
            THE_GREATEST_RELEASE_SEQUENCE_PUBLISHED_WHEN_THIS_BUILD_WAS_MADE,
            0
        );
    }

    /// Every thirty-two-byte literal written down in this workspace's Rust
    /// source — every seed a key in this repository could have been made from.
    /// The same reading `smoozy_server_protocol::release::trust`'s guard makes,
    /// over the same tree.
    fn every_seed_written_down_in_this_workspace() -> Vec<[u8; 32]> {
        fn walk(at: &std::path::Path, out: &mut Vec<[u8; 32]>) {
            let Ok(entries) = std::fs::read_dir(at) else {
                return;
            };
            for entry in entries.flatten() {
                let path = entry.path();
                let name = entry.file_name().to_string_lossy().to_string();
                if name.starts_with('.') || name.starts_with("target") {
                    continue;
                }
                if path.is_dir() {
                    walk(&path, out);
                } else if path.extension().is_some_and(|it| it == "rs") {
                    let Ok(text) = std::fs::read(&path) else {
                        continue;
                    };
                    let mut at = 0;
                    while at + 2 <= text.len() {
                        if &text[at..at + 2] != b"b\"" {
                            at += 1;
                            continue;
                        }
                        let from = at + 2;
                        let Some(length) = text[from..].iter().position(|byte| *byte == b'"')
                        else {
                            break;
                        };
                        let literal = &text[from..from + length];
                        if !literal.contains(&b'\\') {
                            if let Ok(seed) = <[u8; 32]>::try_from(literal) {
                                out.push(seed);
                            }
                        }
                        at = from + length + 1;
                    }
                }
            }
        }
        let mut out = Vec::new();
        walk(
            &std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(".."),
            &mut out,
        );
        out
    }

    /// **What holds on the day the Creator's keys are written in**, and holds
    /// now over an empty list: two or none, distinct, each proved by its own
    /// secret half, none made from a seed written down in this workspace, and
    /// neither the server package's root.
    #[test]
    fn any_recovery_key_this_build_carries_is_one_of_two_proved_keys_this_project_cannot_make() {
        let carried = THE_RECOVERY_KEYS_THIS_BUILD_CARRIES;
        assert!(
            carried.is_empty() || carried.len() == 2,
            "E19 asks for two recovery keys"
        );
        let identifiers: std::collections::HashSet<_> =
            carried.iter().map(|key| key.identifier).collect();
        let public_keys: std::collections::HashSet<_> =
            carried.iter().map(|key| key.public_key).collect();
        assert_eq!(
            (identifiers.len(), public_keys.len()),
            (carried.len(), carried.len())
        );
        let seeds = every_seed_written_down_in_this_workspace();
        assert!(
            seeds.len() >= 9,
            "only {} written-down seeds were read, so the reading has stopped working",
            seeds.len()
        );
        let server_root =
            smoozy_server_protocol::release::trust::THE_RELEASE_ROOT_THIS_BUILD_CARRIES;
        for key in carried {
            assert!(a_signature_verifies(
                &key.public_key,
                super::super::THE_RECOVERY_PROOF_CONTEXT,
                THE_RECOVERY_PROOF_MESSAGE,
                &key.proof
            ));
            assert_ne!(Some(key.public_key), server_root);
            for seed in &seeds {
                assert_ne!(
                    a_key_pair_from_a_written_down_seed(*seed).public,
                    key.public_key
                );
            }
        }
    }

    /// **The statement a build carries verifies under the keys it carries**, or
    /// the build would start every chain from nothing.
    #[test]
    fn the_statement_this_build_carries_verifies_under_the_keys_it_carries() {
        let build = TheCarriedSet::this_build();
        if let Some((bytes, signature)) = &build.statement {
            assert!(verified_against_the_carried_keys(&build, bytes, signature).is_some());
        }
    }
}
