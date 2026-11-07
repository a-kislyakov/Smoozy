//! B9 — trust anchored independently of the downloaded bytes.
//!
//! **The desktop carries an offline release-root public key**, and every
//! manifest signature this module ever accepts is accepted because a statement
//! signed by that root said the signing key was allowed. **Nothing here trusts
//! a manifest to say who may sign it.**
//!
//! **The root's private half is the Creator's and is not in this workspace.**
//! What the desktop carries is the public half, and
//! [`THE_RELEASE_ROOT_THIS_BUILD_CARRIES`] is where it is carried. **It is
//! carried from 2026-09-06**, the day the pair was generated and the secret
//! half went to the Creator's password manager.
//!
//! **The durable memory is `State, identity and recovery` J6's server-package
//! trust record**, whose three fields are exactly [`TheTrustRecord`]'s. **That
//! document's J7 is why the retained statement is verified before the stored
//! numbers refuse anything**, and [`what_the_record_is_worth`] is that rule.
//!
//! **Markers used below that belong to another document, each with the
//! document it belongs to.** `State, identity and recovery` J6, J7, J8, J10.
//! `Desktop app and delivery` E19.
//!
//! **How a clause marker is written in this crate, and it is a rule rather
//! than a habit.** A marker with no document beside it means a clause of this
//! crate's own plan, `Preparation/Architecture/Server protocol and
//! transfers.md`. Every other document's marker is written with that
//! document — at the site, or in the file's own header list above.

use super::{
    a_signature_verifies, Canonical, PublicKeyBytes, Reading, SignatureBytes,
    THE_KEY_SET_SIGNING_CONTEXT,
};

/// The key-set statement format this build composes and reads.
pub const THE_KEY_SET_FORMAT: &str = "smoozy-server-package-key-set-1";

/// **The offline release-root public key this build carries, and there is one
/// from 2026-09-06.**
///
/// **This is the Creator's and not Build's.** B9 anchors every trust decision
/// in this module in a key whose private half never touches a build runner, a
/// service or this repository — `Desktop app and delivery` E19 says the same
/// thing about the desktop updater's own root, which is a different key and,
/// by `State, identity and recovery` J10, not interchangeable with this one.
///
/// **These thirty-two bytes are the public half and the public half only.** The
/// pair was generated once, on 2026-09-06, from the operating system's own
/// randomness. **The secret half is in the Creator's password manager, which
/// holds the master copy, and in that Mac's own login keychain, which is the
/// working copy a release reads.** It is read out of the keychain at the moment
/// it signs, by `App/Release/src/secret.rs`, and nothing in this crate keeps it —
/// which is what [`super::a_signature_over`] taking the secret bytes from its
/// caller is for. **It has never been in this repository, never on a build
/// runner and never in a log.**
///
/// **The Creator's Mac is the machine that builds Smoozy**, so a secret half
/// sitting in a plain file on it would defeat the thing this key exists to do.
/// **That is still right, and it is now the reason the keychain was chosen over
/// a file** rather than the reason there is no working copy on that Mac at all.
///
/// **A build that carried a root somebody in this project generated would be a
/// build whose whole trust chain anybody reading the repository could forge**,
/// and that is still the danger. What guards it now is the test named
/// `the_release_root_this_build_carries_is_not_a_key_this_project_can_make`
/// below, which derives every thirty-two-byte seed written down anywhere in
/// this workspace and fails if the root is any of them.
pub const THE_RELEASE_ROOT_THIS_BUILD_CARRIES: Option<PublicKeyBytes> = Some([
    0xf6, 0xc0, 0xed, 0xd8, 0x9a, 0xdf, 0xae, 0xf2, 0x3b, 0xfa, 0xca, 0x53, 0x0c, 0x72, 0x08, 0xb7,
    0x2d, 0xb2, 0x69, 0xc5, 0xda, 0xac, 0x3a, 0xf8, 0x86, 0xac, 0x2c, 0xf6, 0xf5, 0x84, 0xc0, 0x99,
]);

/// **The greatest key-set generation published when this build was made** —
/// half of B9's built-in floor pair, and it is raised by hand.
///
/// **B9 carries this beside the root and for the same reason.** Every other
/// floor in this module comes out of the desktop's own memory of a release it
/// has already accepted, so a copy that has never accepted one has no floor at
/// all — every install on its first day, and every install whose trust record
/// `State, identity and recovery` J7 has discarded. Such a copy would admit any
/// root-signed statement the root has ever signed, at any generation, and then
/// accept any manifest an allowed key signed below that key's revocation: **the
/// revocation is real and the desktop simply never meets the statement carrying
/// it.**
///
/// **It is the last published release's generation and never a value derived
/// from this build.** A commit count or a build number would put the floor above
/// any release that exists, and every fresh install would then refuse every
/// genuine release until the next one was published. ~~**This build has never
/// had it raised, so it is zero and refuses nothing** — which is exactly the
/// state the first release meets.~~ *(Struck 2026-09-21: it was true of every
/// build up to and including the first release's, and it stopped being true
/// the day that release was published.)*
///
/// **It is 1, raised 2026-09-21: the generation of the key-set statement
/// Smoozy 0.1.0 was published with, at release sequence 1609** — read off that
/// published release itself, from the repository's fixed latest address, by
/// `smoozy-release check`: it verifies as a desktop that has accepted
/// generation 1 and refuses as one that has accepted generation 2. **The next
/// release's runner raises it again, to that release's own generation, before
/// that release's green run** — `Preparation/Groundwork/Templates/Release
/// Runner.md` step 3a.
///
/// **Checked again 2026-09-22, before the third release, and still 1: the
/// generation of the key-set statement Smoozy 0.1.1 was published with, at
/// release sequence 1617** — read off that published release the same way. It
/// verifies as a desktop that has accepted generation 1 and refuses as one that
/// has accepted generation 2, so 0.1.1 was published under the same key-set
/// statement generation as 0.1.0 and there was nothing to raise here.
///
/// **Checked again 2026-09-22, before the fourth release, and still 1: the
/// generation of the key-set statement Smoozy 0.1.2 was published with, at
/// release sequence 1626** — read off that published release the same way. It
/// verifies as a desktop that has accepted generation 1 and refuses as one that
/// has accepted generation 2, so 0.1.2 too was published under generation 1
/// and there was nothing to raise here.
///
/// **Checked again 2026-09-22, before the fifth release, and still 1: the
/// generation of the key-set statement Smoozy 0.1.3 was published with, at
/// release sequence 1632** — read off that published release the same way. It
/// verifies as a desktop that has accepted generation 1 and refuses as one that
/// has accepted generation 2, so 0.1.3 too was published under generation 1
/// and there was nothing to raise here.
///
/// **Checked again 2026-09-26, before the sixth release, and still 1: the
/// generation of the key-set statement Smoozy 0.1.4 was published with, at
/// release sequence 1638** — read off that published release the same way. It
/// verifies as a desktop that has accepted generation 1 and refuses as one that
/// has accepted generation 2, so 0.1.4 too was published under generation 1
/// and there was nothing to raise here.
///
/// **Checked again 2026-09-26, before the seventh release, and still 1: the
/// generation of the key-set statement Smoozy 0.1.5 was published with, at
/// release sequence 1644** — read off that published release the same way. It
/// verifies as a desktop that has accepted generation 1 and refuses as one that
/// has accepted generation 2, so 0.1.5 too was published under generation 1
/// and there was nothing to raise here.
///
/// **Checked again 2026-10-02, before 0.1.7, and still 1: the generation of
/// the key-set statement Smoozy 0.1.6 was published with, at release sequence
/// 1651** — read off that published release the same way. It verifies as a
/// desktop that has accepted generation 1 and refuses as one that has accepted
/// generation 2, so 0.1.6 too was published under generation 1 and there was
/// nothing to raise here.
///
/// **Checked again 2026-10-02, before 0.1.8, and still 1: the generation of
/// the key-set statement Smoozy 0.1.7 was published with, at release sequence
/// 1766** (`v0.1.7`) — read off that published release the same way. It
/// verifies as a desktop that has accepted generation 1 and refuses as one that
/// has accepted generation 2, so 0.1.7 too was published under generation 1
/// and there was nothing to raise here.
///
/// **Checked again 2026-11-01, before 0.2.0, and still 1: the generation
/// Smoozy 0.1.8 was published under, at release sequence 1812** (`v0.1.8`).
/// **It could not be read off that release**, because the Creator deleted all
/// nine earlier releases that day, so it is taken from 0.1.8's own account
/// (`Audits/Build/Step 10c/Piece 11 — the release of 0.1.8.md`): it names no
/// generation, so it was composed at the tool's default, 1, and it verified as
/// a copy that had accepted generation 1. No release was ever composed above
/// generation 1, so there was nothing to raise.
///
/// **Checked again 2026-11-07, before 0.3.0, and still 1: the generation
/// Smoozy 0.2.0 was published under, at release sequence 2847** (`v0.2.0`) —
/// read off that published release, by its own address, with
/// `smoozy-release check`. It verifies as a desktop that has accepted
/// generation 1 and refuses as one that has accepted generation 2, so there was
/// nothing to raise here.
pub const THE_GREATEST_KEY_SET_GENERATION_PUBLISHED_WHEN_THIS_BUILD_WAS_MADE: u64 = 1;

/// **The greatest release sequence published when this build was made** — the
/// other half of B9's built-in floor pair. See the constant above; the two are
/// raised together, by hand, in the act that fixes a release's own sequence.
///
/// ~~**It is 1609, raised 2026-09-21: the release sequence Smoozy 0.1.0 was
/// published at**, read off that published release the same way — it verifies
/// as a desktop that has accepted sequence 1609 and refuses as one that has
/// accepted 1610.~~ *(Struck 2026-09-22: 1609 was the last published release
/// until Smoozy 0.1.1 was published at 1617.)*
///
/// ~~**It is 1617, raised 2026-09-22, before the third release: the release
/// sequence Smoozy 0.1.1 was published at**, read off that published release
/// from the repository's fixed latest address by `smoozy-release check` — it
/// verifies as a desktop that has accepted sequence 1617 and refuses as one
/// that has accepted 1618.~~ *(Struck 2026-09-22: 1617 was the last published
/// release until Smoozy 0.1.2 was published at 1626.)*
///
/// ~~**It is 1626, raised 2026-09-22, before the fourth release: the release
/// sequence Smoozy 0.1.2 was published at**, read off that published release
/// from the repository's fixed latest address by `smoozy-release check` — it
/// verifies as a desktop that has accepted sequence 1626 and refuses as one
/// that has accepted 1627.~~ *(Struck 2026-09-22: 1626 was the last published
/// release until Smoozy 0.1.3 was published at 1632.)*
///
/// ~~**It is 1632, raised 2026-09-22, before the fifth release: the release
/// sequence Smoozy 0.1.3 was published at**, read off that published release
/// from the repository's fixed latest address by `smoozy-release check` — it
/// verifies as a desktop that has accepted sequence 1632 and refuses as one
/// that has accepted 1633.~~ *(Struck 2026-09-26: 1632 was the last published
/// release until Smoozy 0.1.4 was published at 1638.)*
///
/// ~~**It is 1638, raised 2026-09-26, before the sixth release: the release
/// sequence Smoozy 0.1.4 was published at**, read off that published release
/// from the repository's fixed latest address by `smoozy-release check` — it
/// verifies as a desktop that has accepted sequence 1638 and refuses as one
/// that has accepted 1639.~~ *(Struck 2026-09-26: 1638 was the last published
/// release until Smoozy 0.1.5 was published at 1644.)*
///
/// ~~**It is 1644, raised 2026-09-26, before the seventh release: the release
/// sequence Smoozy 0.1.5 was published at**, read off that published release
/// from the repository's fixed latest address by `smoozy-release check` — it
/// verifies as a desktop that has accepted sequence 1644 and refuses as one
/// that has accepted 1645. Raised again, with the constant above, before the
/// next release's green run.~~ *(Struck 2026-10-02: 1644 was the last published
/// release until Smoozy 0.1.6 was published at 1651.)*
///
/// ~~**It is 1651, raised 2026-10-02, before 0.1.7: the release sequence Smoozy
/// 0.1.6 was published at** (`release-1651`), read off that published release
/// from the repository's fixed latest address by `smoozy-release check` — it
/// verifies as a desktop that has accepted sequence 1651 and refuses as one
/// that has accepted 1652. Raised again, with the constant above, before the
/// next release's green run.~~ *(Struck 2026-10-02: 1651 was the last published
/// release until Smoozy 0.1.7 was published at 1766.)*
///
/// ~~**It is 1766, raised 2026-10-02, before 0.1.8: the release sequence Smoozy
/// 0.1.7 was published at** (`v0.1.7`), read off that published release, by
/// its own address and by the repository's fixed latest address, with
/// `smoozy-release check` — it verifies as a desktop that has accepted sequence
/// 1766 and refuses as one that has accepted 1767. Raised again, with the
/// constant above, before the next release's green run.~~ *(Struck 2026-11-01:
/// 1766 was the last published release until Smoozy 0.1.8 was published at
/// 1812.)*
///
/// ~~**It is 1812, raised 2026-11-01, before 0.2.0: the release sequence Smoozy
/// 0.1.8 was published at** (`v0.1.8`, from `007491f9`). **It is the greatest
/// sequence ever published, even though no release is published today**: the
/// Creator deleted all nine earlier releases on 2026-11-01, but every one of
/// them was signed by the root this build carries and copies of their files
/// can still exist, so B9's floor still has a job — refusing them on a fresh
/// install. **It could not be read off the published release, so it was
/// established two other ways**: 0.1.8's account names 1812, and 1812 is the
/// commit count of `007491f9`, which is how a release's sequence is made
/// (`Audits/Build/Step 8a/Decisions record.md`). **Why it is safe**: a copy
/// whose record remembers 0.1.8's 1812 still accepts 0.2.0, whose own sequence
/// is its commit's count and far above 1812; and under B8d this build accepts
/// only its own release, so a floor at 1812 refuses nothing it must take.
/// Raised again, with the constant above, before the next release's green
/// run.~~ *(Struck 2026-11-07: 1812 was the greatest published release until
/// Smoozy 0.2.0 was published at 2847.)*
///
/// **It is 2847, raised 2026-11-07, before 0.3.0: the release sequence Smoozy
/// 0.2.0 was published at** (`v0.2.0`, from `e8919745e`), read off that
/// published release, by its own address, with `smoozy-release check` — it
/// verifies as a desktop that has accepted sequence 2847 and refuses as one
/// that has accepted 2848. Raised again, with the constant above, before the
/// next release's green run.
pub const THE_GREATEST_RELEASE_SEQUENCE_PUBLISHED_WHEN_THIS_BUILD_WAS_MADE: u64 = 2847;

/// **B9's built-in floor pair, as one value.**
///
/// The two numbers are raised together and are read together, so they are one
/// thing rather than two constants a caller has to remember to pass both of.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TheBuiltInFloor {
    pub greatest_key_set_generation: u64,
    pub greatest_release_sequence: u64,
}

impl TheBuiltInFloor {
    /// What this build carries.
    pub fn this_build() -> TheBuiltInFloor {
        TheBuiltInFloor {
            greatest_key_set_generation:
                THE_GREATEST_KEY_SET_GENERATION_PUBLISHED_WHEN_THIS_BUILD_WAS_MADE,
            greatest_release_sequence:
                THE_GREATEST_RELEASE_SEQUENCE_PUBLISHED_WHEN_THIS_BUILD_WAS_MADE,
        }
    }
}

/// **The context the one committed proof of the release root is signed under,
/// and it is deliberately neither of this module's two.**
///
/// A signature under [`THE_KEY_SET_SIGNING_CONTEXT`] or
/// [`super::THE_MANIFEST_SIGNING_CONTEXT`] is a signature over a document this
/// module will act on. **A proof kept in the repository must never be one of
/// those**, or the proof itself would be a root-signed document anybody could
/// offer to a desktop. Under a third context it is inert: nothing in this module
/// reads it, and no document verifies under it.
pub const THE_ROOT_PROOF_CONTEXT: &[u8] = b"smoozy.server-package.root-proof.1\0";

/// The exact bytes signed once, when the pair was generated.
pub const THE_ROOT_PROOF_MESSAGE: &[u8] =
    b"the Smoozy server-package release root, proved once on 2026-09-06";

/// **The signature the Creator's secret half made over
/// [`THE_ROOT_PROOF_MESSAGE`], and it is public.**
///
/// A signature gives away nothing about the key that made it. **What it is worth
/// is that somebody held the secret half of exactly these thirty-two bytes**, so
/// [`THE_RELEASE_ROOT_THIS_BUILD_CARRIES`] cannot be a mistyped key, a truncated
/// one, or thirty-two bytes nobody has the other half of — and the test below
/// says so through this module's own verification, not through a library call
/// written for it.
pub const THE_ROOT_PROOF_SIGNATURE: SignatureBytes = [
    0xd2, 0x4d, 0x6a, 0x00, 0xa8, 0xe3, 0x6f, 0x01, 0x72, 0xf5, 0x37, 0x03, 0x54, 0x71, 0x15, 0x0e,
    0x78, 0xb0, 0x70, 0x87, 0x39, 0xe2, 0x0d, 0xfc, 0x25, 0xb7, 0x20, 0x26, 0xc0, 0xc7, 0xcb, 0x3c,
    0x09, 0x4b, 0xaa, 0x10, 0xa3, 0xc0, 0x2b, 0x3d, 0xcb, 0x29, 0xf3, 0x91, 0x71, 0x09, 0x63, 0x8b,
    0x00, 0x4b, 0x0c, 0xbf, 0xd5, 0x1b, 0x05, 0x7e, 0xa7, 0x62, 0x34, 0x01, 0x9c, 0xa4, 0x36, 0x03,
];

/// One online manifest-signing key the root allows.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AnAllowedKey {
    pub identity: String,
    pub public_key: PublicKeyBytes,
    /// **Valid from this release sequence.** A manifest below it is not this
    /// key's to sign.
    pub valid_from_release_sequence: u64,
}

/// One revocation, with the first release sequence the key may no longer
/// authorize.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ARevocation {
    pub identity: String,
    pub first_disallowed_release_sequence: u64,
}

/// **B9's root-signed key-set statement, every field of it.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KeySetStatement {
    pub statement_format: String,
    /// **Its own increasing generation**, kept against the durable memory.
    pub generation: u64,
    pub allowed: Vec<AnAllowedKey>,
    pub revocations: Vec<ARevocation>,
}

impl KeySetStatement {
    /// Read a statement from the exact bytes the root signed.
    pub fn read(bytes: &[u8]) -> Option<KeySetStatement> {
        let statement = read_fields(bytes)?;
        (statement.statement_format == THE_KEY_SET_FORMAT).then_some(statement)
    }

    /// The canonical bytes of this statement. **The release side's function**,
    /// for the same reason the manifest's is.
    pub fn canonical_bytes(&self) -> Vec<u8> {
        let mut out = Canonical::new();
        out.text(&self.statement_format)
            .u64(self.generation)
            .u32(self.allowed.len() as u32);
        for key in &self.allowed {
            out.text(&key.identity)
                .digest(&key.public_key)
                .u64(key.valid_from_release_sequence);
        }
        out.u32(self.revocations.len() as u32);
        for revocation in &self.revocations {
            out.text(&revocation.identity)
                .u64(revocation.first_disallowed_release_sequence);
        }
        out.finish()
    }

    /// **Which keys may authorize a manifest at this release sequence.**
    ///
    /// A key is offered only where it is allowed, its validity has begun, and
    /// **no revocation of it names a first-disallowed sequence at or below the
    /// one asked about** — B9's *a revoked online key cannot authorize a
    /// manifest at or after its revocation sequence*.
    pub fn keys_that_may_authorize(&self, release_sequence: u64) -> Vec<&AnAllowedKey> {
        self.allowed
            .iter()
            .filter(|key| key.valid_from_release_sequence <= release_sequence)
            .filter(|key| {
                !self.revocations.iter().any(|revocation| {
                    revocation.identity == key.identity
                        && revocation.first_disallowed_release_sequence <= release_sequence
                })
            })
            .collect()
    }
}

fn read_fields(bytes: &[u8]) -> Option<KeySetStatement> {
    let mut reading = Reading::of(bytes);
    let statement_format = reading.text()?;
    let generation = reading.u64()?;
    let how_many = reading.u32()?;
    let mut allowed = Vec::new();
    for _ in 0..how_many {
        allowed.push(AnAllowedKey {
            identity: reading.text()?,
            public_key: reading.digest()?,
            valid_from_release_sequence: reading.u64()?,
        });
    }
    let how_many = reading.u32()?;
    let mut revocations = Vec::new();
    for _ in 0..how_many {
        revocations.push(ARevocation {
            identity: reading.text()?,
            first_disallowed_release_sequence: reading.u64()?,
        });
    }
    reading.end()?;
    Some(KeySetStatement {
        statement_format,
        generation,
        allowed,
        revocations,
    })
}

/// **`State, identity and recovery` J6's server-package trust record, and its
/// three fields are that document's three.**
///
/// The greatest accepted release sequence, the greatest accepted key-set
/// generation, and **the root-verified statement accepted at that generation,
/// retained** — which is what J7 needs and what makes the two numbers worth
/// anything.
///
/// **The statement is kept as the bytes that were signed**, with the signature
/// beside it, because verifying it again means verifying what was verified the
/// first time.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TheTrustRecord {
    pub greatest_accepted_release_sequence: u64,
    pub greatest_accepted_key_set_generation: u64,
    pub the_statement_accepted_at_that_generation: Vec<u8>,
    pub its_root_signature: SignatureBytes,
}

/// **What a stored trust record turns out to be worth — `State, identity and
/// recovery` J7.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WhatTheRecordIsWorth {
    /// The retained statement verified against the root, so the two numbers
    /// stand and refuse what is below them.
    ItsNumbersStand {
        greatest_accepted_release_sequence: u64,
        greatest_accepted_key_set_generation: u64,
        the_statement: KeySetStatement,
    },
    /// **The statement is absent, malformed, will not verify, or does not carry
    /// the generation the record claims — so the record is unsupported and is
    /// discarded whole.** It falls back to holding neither number and the next
    /// genuine statement re-establishes the record's floor on its own merits,
    /// above whatever B9's built-in pair already holds. **The floor in force
    /// does not go with it**: [`TheBuiltInFloor`] is what a copy that remembers
    /// nothing is held to.
    ///
    /// **This is a correction and not a weakening — J8.** What it gives back is
    /// a route out of a number that read too high, which otherwise refused every
    /// genuine manifest for good and in silence. Lowering a stored number was
    /// never protected: the record is unencrypted and the user's own by
    /// decision.
    ///
    /// **What the root's signature protects, said exactly rather than
    /// generously.** That signature is over the retained **statement** and it
    /// covers no field of the record. So:
    ///
    /// - **The stored generation is protected, and by a comparison rather than
    ///   by the signature.** It is written equal to the retained statement's own
    ///   generation and can only differ from it by damage, so a record where the
    ///   two disagree is discarded here.
    /// - **The stored release sequence is a bare number and nothing covers
    ///   it.** Raising it needs no signature at all: the retained statement
    ///   still verifies, the raised number stands, and every genuine release is
    ///   then refused. **What that costs is bounded by two things** — it is
    ///   reachable only by a hand on the record or a fault in the desktop's own
    ///   writes, the store being the user's own and unencrypted by decision;
    ///   and the route out is J7's discard, which is this same value. **Keeping
    ///   the accepted manifest in the record to cover it was weighed and
    ///   refused**: a second signed document to guard a number the record's own
    ///   unencrypted standing already concedes is more machinery than the threat
    ///   is worth.
    ItIsDiscardedAndTheFloorIsGone,
}

/// **J7, performed.** Called before a stored number refuses anything.
pub fn what_the_record_is_worth(
    root: &PublicKeyBytes,
    remembered: Option<&TheTrustRecord>,
) -> WhatTheRecordIsWorth {
    let Some(record) = remembered else {
        return WhatTheRecordIsWorth::ItIsDiscardedAndTheFloorIsGone;
    };
    let verified = a_signature_verifies(
        root,
        THE_KEY_SET_SIGNING_CONTEXT,
        &record.the_statement_accepted_at_that_generation,
        &record.its_root_signature,
    );
    if !verified {
        return WhatTheRecordIsWorth::ItIsDiscardedAndTheFloorIsGone;
    }
    let Some(statement) =
        KeySetStatement::read(&record.the_statement_accepted_at_that_generation)
    else {
        return WhatTheRecordIsWorth::ItIsDiscardedAndTheFloorIsGone;
    };
    // **The stored generation is the retained statement's own, and the two are
    // written equal.** They can only differ by damage, so a record where they
    // disagree is unsupported in exactly the way a statement that will not
    // verify is. **This is the check that was missing**: the root's signature is
    // over the statement and covers no field of the record, so a raised stored
    // generation used to stand on a signature that said nothing about it.
    if record.greatest_accepted_key_set_generation != statement.generation {
        return WhatTheRecordIsWorth::ItIsDiscardedAndTheFloorIsGone;
    }
    WhatTheRecordIsWorth::ItsNumbersStand {
        greatest_accepted_release_sequence: record.greatest_accepted_release_sequence,
        greatest_accepted_key_set_generation: record.greatest_accepted_key_set_generation,
        the_statement: statement,
    }
}

/// **Where the trust record is kept, which is not this document's.**
///
/// `State, identity and recovery` J6 puts it in the desktop's one local record,
/// and `Desktop app and delivery` A2 says no module has a durable store of its
/// own — so the store is that document's and the rule is this one's, and this
/// trait is the join. **The real end is `smoozy_desktop::transfers`.**
pub trait RemembersTheServerPackageTrustRecord {
    fn read_it(&self) -> Result<Option<TheTrustRecord>, String>;
    fn write_it(&mut self, record: &TheTrustRecord) -> Result<(), String>;
    /// **J7's discard**, called where the retained statement will not verify.
    fn discard_it(&mut self) -> Result<(), String>;
}

/// **Why an offered key-set statement was not admitted.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhyTheStatementWasRefused {
    /// The root's signature over it does not verify. **Trust is anchored in the
    /// root and in nothing the statement says about itself.**
    TheRootDidNotSignIt,
    /// It verified and it is not a statement, or not one of a format this build
    /// knows.
    ItIsNotAKeySetStatementThisBuildKnows,
    /// **Its generation is below the floor in force.** B9 refuses a rollback of
    /// the generation as well as of the release sequence, and the floor is the
    /// greater of what this build carries and what the desktop remembers.
    ItsGenerationIsBelowTheFloorInForce,
}

/// **What was admitted, ready for the manifest half of the check.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AnAdmittedKeySet {
    pub statement: KeySetStatement,
    /// **The floor a manifest's own release sequence is held to, and there is
    /// always one.** B9: it is the greater of the sequence this build carries
    /// and the sequence the desktop remembers, never the smaller — so a
    /// remembered floor is never lowered by a build, and a copy J7 has just
    /// discarded the record of falls back to the build's floor rather than to
    /// none.
    pub the_release_sequence_floor: u64,
}

/// **B9's admission of a key-set statement, in the order B10 states it.**
///
/// The root's signature first, the statement second, the non-rollback rule
/// third. **An online-key transition is accepted when the newer root-signed
/// statement authorizes it** — which is the whole of the mechanism: a
/// statement of a higher generation replaces the allowed set, and nothing has
/// to have observed the overlap, because the statement is retained on hosting
/// and carries the whole set rather than a difference.
/// **And the floor on each number is the greater of what this build carries and
/// what the desktop remembers** — B9's built-in pair, which is what gives a copy
/// that has never accepted a release a floor at all.
pub fn admit_a_key_set_statement(
    root: &PublicKeyBytes,
    remembered: Option<&TheTrustRecord>,
    offered_bytes: &[u8],
    offered_signature: &SignatureBytes,
) -> Result<AnAdmittedKeySet, WhyTheStatementWasRefused> {
    admit_a_key_set_statement_against(
        root,
        remembered,
        offered_bytes,
        offered_signature,
        TheBuiltInFloor::this_build(),
    )
}

/// **The same admission, against a built-in floor named by the caller.**
///
/// **The rule is held in the tests through this function rather than through
/// the constants**, because a test that states its own floor goes on meaning
/// the same thing every time a release raises the build's pair.
/// [`admit_a_key_set_statement`] is it with [`TheBuiltInFloor::this_build`]
/// passed in. *(Until 2026-09-21 this said the build's own pair was zero and
/// zero, which was the reason then; ~~the pair is now 1 and 1609~~ — struck
/// 2026-09-22: ~~the pair is now 1 and 1617, Smoozy 0.1.1's own~~ — struck
/// again 2026-09-22, before the fourth release: ~~the pair is now 1 and 1626,
/// Smoozy 0.1.2's own~~ — struck again 2026-09-22, before the fifth release:
/// ~~the pair is now 1 and 1632, Smoozy 0.1.3's own~~ — struck again
/// 2026-09-26, before the sixth release: ~~the pair is now 1 and 1638, Smoozy
/// 0.1.4's own~~ — struck again 2026-09-26, before the seventh release:
/// ~~the pair is now 1 and 1644, Smoozy 0.1.5's own~~ — struck again
/// 2026-10-02, before 0.1.7: ~~the pair is now 1 and 1651, Smoozy 0.1.6's
/// own~~ — struck again 2026-10-02, before 0.1.8: ~~the pair is now 1 and
/// 1766, Smoozy 0.1.7's own~~ — struck again 2026-11-01, before 0.2.0:
/// ~~the pair is now 1 and 1812, Smoozy 0.1.8's own~~ — struck again
/// 2026-11-07, before 0.3.0: the pair is now 1 and 2847, Smoozy 0.2.0's own.)*
pub fn admit_a_key_set_statement_against(
    root: &PublicKeyBytes,
    remembered: Option<&TheTrustRecord>,
    offered_bytes: &[u8],
    offered_signature: &SignatureBytes,
    built_in: TheBuiltInFloor,
) -> Result<AnAdmittedKeySet, WhyTheStatementWasRefused> {
    if !a_signature_verifies(
        root,
        THE_KEY_SET_SIGNING_CONTEXT,
        offered_bytes,
        offered_signature,
    ) {
        return Err(WhyTheStatementWasRefused::TheRootDidNotSignIt);
    }
    let statement = KeySetStatement::read(offered_bytes)
        .ok_or(WhyTheStatementWasRefused::ItIsNotAKeySetStatementThisBuildKnows)?;

    // **A discarded record remembers nothing, which is zero on both numbers
    // rather than an absence** — the floor that then stands is the build's own,
    // and `max` is what makes that one line instead of two branches.
    let (remembered_generation, remembered_sequence) = match what_the_record_is_worth(root, remembered)
    {
        WhatTheRecordIsWorth::ItIsDiscardedAndTheFloorIsGone => (0, 0),
        WhatTheRecordIsWorth::ItsNumbersStand {
            greatest_accepted_release_sequence,
            greatest_accepted_key_set_generation,
            ..
        } => (
            greatest_accepted_key_set_generation,
            greatest_accepted_release_sequence,
        ),
    };

    if statement.generation < remembered_generation.max(built_in.greatest_key_set_generation) {
        return Err(WhyTheStatementWasRefused::ItsGenerationIsBelowTheFloorInForce);
    }
    Ok(AnAdmittedKeySet {
        statement,
        the_release_sequence_floor: remembered_sequence.max(built_in.greatest_release_sequence),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::release::composing::test_only_keys;
    use crate::release::a_signature_over;

    fn a_root() -> test_only_keys::AKeyPair {
        test_only_keys::a_key_pair_from_a_written_down_seed(*b"the release root of these tests_")
    }

    fn an_online_key() -> test_only_keys::AKeyPair {
        test_only_keys::a_key_pair_from_a_written_down_seed(*b"the online signing key of tests_")
    }

    fn a_statement(generation: u64) -> KeySetStatement {
        KeySetStatement {
            statement_format: THE_KEY_SET_FORMAT.into(),
            generation,
            allowed: vec![AnAllowedKey {
                identity: "the first online key".into(),
                public_key: an_online_key().public,
                valid_from_release_sequence: 10,
            }],
            revocations: Vec::new(),
        }
    }

    fn signed_by(root: &test_only_keys::AKeyPair, statement: &KeySetStatement) -> (Vec<u8>, SignatureBytes) {
        let bytes = statement.canonical_bytes();
        let signature = a_signature_over(&root.secret, THE_KEY_SET_SIGNING_CONTEXT, &bytes);
        (bytes, signature)
    }

    fn a_record(root: &test_only_keys::AKeyPair, statement: &KeySetStatement, release_sequence: u64) -> TheTrustRecord {
        let (bytes, signature) = signed_by(root, statement);
        TheTrustRecord {
            greatest_accepted_release_sequence: release_sequence,
            greatest_accepted_key_set_generation: statement.generation,
            the_statement_accepted_at_that_generation: bytes,
            its_root_signature: signature,
        }
    }

    #[test]
    fn a_statement_reads_back_as_what_was_composed() {
        let statement = KeySetStatement {
            revocations: vec![ARevocation {
                identity: "the first online key".into(),
                first_disallowed_release_sequence: 40,
            }],
            ..a_statement(3)
        };
        assert_eq!(
            KeySetStatement::read(&statement.canonical_bytes()).unwrap(),
            statement
        );
    }

    #[test]
    fn a_statement_the_root_did_not_sign_is_refused_before_it_is_read() {
        let root = a_root();
        let somebody_else = test_only_keys::a_key_pair_from_a_written_down_seed(*b"not the root, and written down__");
        let statement = a_statement(1);
        let bytes = statement.canonical_bytes();
        let signature = a_signature_over(&somebody_else.secret, THE_KEY_SET_SIGNING_CONTEXT, &bytes);
        assert_eq!(
            admit_a_key_set_statement(&root.public, None, &bytes, &signature),
            Err(WhyTheStatementWasRefused::TheRootDidNotSignIt)
        );
    }

    /// **This is about the remembered floor alone, so it states a build that
    /// carries none** rather than borrowing this build's pair. *(Until
    /// 2026-09-21 it went through [`admit_a_key_set_statement`] and the build's
    /// pair, which only left the remembered floor standing alone while that pair
    /// was zero; with the pair at 1 and 1609 the release-sequence floor handed
    /// on was the build's, and the test was no longer about what it names.)*
    #[test]
    fn a_generation_below_the_one_remembered_is_refused() {
        let root = a_root();
        let a_build_that_carries_no_floor = TheBuiltInFloor {
            greatest_key_set_generation: 0,
            greatest_release_sequence: 0,
        };
        let remembered = a_record(&root, &a_statement(5), 100);
        let (bytes, signature) = signed_by(&root, &a_statement(4));
        assert_eq!(
            admit_a_key_set_statement_against(
                &root.public,
                Some(&remembered),
                &bytes,
                &signature,
                a_build_that_carries_no_floor
            ),
            Err(WhyTheStatementWasRefused::ItsGenerationIsBelowTheFloorInForce)
        );
        // The same generation is not a rollback and is admitted.
        let (bytes, signature) = signed_by(&root, &a_statement(5));
        let admitted = admit_a_key_set_statement_against(
            &root.public,
            Some(&remembered),
            &bytes,
            &signature,
            a_build_that_carries_no_floor,
        )
        .unwrap();
        assert_eq!(admitted.the_release_sequence_floor, 100);
    }

    /// **B9's built-in pair: a copy that has never accepted a release still has
    /// a floor**, so a revoked key does not go on working against every fresh
    /// install for ever.
    #[test]
    fn a_desktop_that_remembers_nothing_is_still_held_to_the_floor_this_build_carries() {
        let root = a_root();
        let built_in = TheBuiltInFloor {
            greatest_key_set_generation: 5,
            greatest_release_sequence: 100,
        };

        // A statement below the built-in generation is refused, with nothing
        // remembered at all.
        let (bytes, signature) = signed_by(&root, &a_statement(4));
        assert_eq!(
            admit_a_key_set_statement_against(&root.public, None, &bytes, &signature, built_in),
            Err(WhyTheStatementWasRefused::ItsGenerationIsBelowTheFloorInForce)
        );

        // At the built-in generation it is admitted, and the release-sequence
        // floor it hands on is the build's own.
        let (bytes, signature) = signed_by(&root, &a_statement(5));
        let admitted =
            admit_a_key_set_statement_against(&root.public, None, &bytes, &signature, built_in)
                .unwrap();
        assert_eq!(admitted.the_release_sequence_floor, 100);
    }

    /// **The floor is the greater of the two on both numbers, never the
    /// smaller** — so a build never lowers what a desktop already remembers,
    /// and a desktop's memory never lowers what a build carries.
    #[test]
    fn the_floor_is_the_greater_of_the_built_in_pair_and_the_remembered_one() {
        let root = a_root();
        let remembered = a_record(&root, &a_statement(7), 200);

        // The build is behind what is remembered: the remembered numbers stand.
        let behind = TheBuiltInFloor {
            greatest_key_set_generation: 5,
            greatest_release_sequence: 100,
        };
        let (bytes, signature) = signed_by(&root, &a_statement(6));
        assert_eq!(
            admit_a_key_set_statement_against(
                &root.public,
                Some(&remembered),
                &bytes,
                &signature,
                behind
            ),
            Err(WhyTheStatementWasRefused::ItsGenerationIsBelowTheFloorInForce)
        );
        let (bytes, signature) = signed_by(&root, &a_statement(7));
        let admitted = admit_a_key_set_statement_against(
            &root.public,
            Some(&remembered),
            &bytes,
            &signature,
            behind,
        )
        .unwrap();
        assert_eq!(admitted.the_release_sequence_floor, 200);

        // The build is ahead of what is remembered: the build's numbers stand.
        let ahead = TheBuiltInFloor {
            greatest_key_set_generation: 9,
            greatest_release_sequence: 300,
        };
        let (bytes, signature) = signed_by(&root, &a_statement(8));
        assert_eq!(
            admit_a_key_set_statement_against(
                &root.public,
                Some(&remembered),
                &bytes,
                &signature,
                ahead
            ),
            Err(WhyTheStatementWasRefused::ItsGenerationIsBelowTheFloorInForce)
        );
        let (bytes, signature) = signed_by(&root, &a_statement(9));
        let admitted = admit_a_key_set_statement_against(
            &root.public,
            Some(&remembered),
            &bytes,
            &signature,
            ahead,
        )
        .unwrap();
        assert_eq!(admitted.the_release_sequence_floor, 300);
    }

    /// **J7's discard falls back to the build's floor rather than to none.**
    /// That is what keeps J8's *this is a correction and not a weakening* worth
    /// what it claims for the whole life of an install.
    #[test]
    fn a_discarded_record_falls_back_to_the_builds_floor_rather_than_to_none() {
        let root = a_root();
        let mut damaged = a_record(&root, &a_statement(9), 300);
        damaged.its_root_signature[0] ^= 1;
        let built_in = TheBuiltInFloor {
            greatest_key_set_generation: 5,
            greatest_release_sequence: 100,
        };
        let (bytes, signature) = signed_by(&root, &a_statement(4));
        assert_eq!(
            admit_a_key_set_statement_against(
                &root.public,
                Some(&damaged),
                &bytes,
                &signature,
                built_in
            ),
            Err(WhyTheStatementWasRefused::ItsGenerationIsBelowTheFloorInForce)
        );
        let (bytes, signature) = signed_by(&root, &a_statement(5));
        let admitted = admit_a_key_set_statement_against(
            &root.public,
            Some(&damaged),
            &bytes,
            &signature,
            built_in,
        )
        .unwrap();
        assert_eq!(admitted.the_release_sequence_floor, 100);
    }

    /// **The pair this build carries is the last published release's and never
    /// a value derived from the build, and it refuses what is below it.**
    ///
    /// ~~*A build nobody has raised it on carries zero and refuses nothing on a
    /// number.*~~ *(Struck 2026-09-21, and the test renamed from
    /// `the_built_in_pair_this_build_carries_refuses_nothing_on_a_number`. That
    /// was the build's floor before the first release was published; it was
    /// raised that day to the published release's own numbers, and a test of
    /// the build's floor now has to prove the floor bites.)*
    #[test]
    fn the_built_in_pair_this_build_carries_refuses_what_is_below_it() {
        assert_eq!(
            TheBuiltInFloor::this_build(),
            TheBuiltInFloor {
                greatest_key_set_generation:
                    THE_GREATEST_KEY_SET_GENERATION_PUBLISHED_WHEN_THIS_BUILD_WAS_MADE,
                greatest_release_sequence:
                    THE_GREATEST_RELEASE_SEQUENCE_PUBLISHED_WHEN_THIS_BUILD_WAS_MADE,
            }
        );
        let root = a_root();
        // One generation below the build's, with nothing remembered: refused.
        let (bytes, signature) = signed_by(
            &root,
            &a_statement(
                THE_GREATEST_KEY_SET_GENERATION_PUBLISHED_WHEN_THIS_BUILD_WAS_MADE
                    .checked_sub(1)
                    .expect("this build carries a key-set generation above zero"),
            ),
        );
        assert_eq!(
            admit_a_key_set_statement(&root.public, None, &bytes, &signature),
            Err(WhyTheStatementWasRefused::ItsGenerationIsBelowTheFloorInForce)
        );
        // At the build's generation: admitted, and the release-sequence floor it
        // hands on is the build's own.
        let (bytes, signature) = signed_by(
            &root,
            &a_statement(THE_GREATEST_KEY_SET_GENERATION_PUBLISHED_WHEN_THIS_BUILD_WAS_MADE),
        );
        let admitted =
            admit_a_key_set_statement(&root.public, None, &bytes, &signature).unwrap();
        assert_eq!(
            admitted.the_release_sequence_floor,
            THE_GREATEST_RELEASE_SEQUENCE_PUBLISHED_WHEN_THIS_BUILD_WAS_MADE
        );
    }

    #[test]
    fn a_record_whose_retained_statement_will_not_verify_is_discarded_rather_than_obeyed() {
        let root = a_root();
        let mut damaged = a_record(&root, &a_statement(5), 100);
        damaged.its_root_signature[0] ^= 1;
        assert_eq!(
            what_the_record_is_worth(&root.public, Some(&damaged)),
            WhatTheRecordIsWorth::ItIsDiscardedAndTheFloorIsGone
        );
        // And the floor it carried refuses nothing: a statement of a lower
        // generation is now admitted on its own merits, and what stands in its
        // place is the floor this build carries.
        let (bytes, signature) = signed_by(&root, &a_statement(1));
        let admitted =
            admit_a_key_set_statement(&root.public, Some(&damaged), &bytes, &signature).unwrap();
        assert_eq!(
            admitted.the_release_sequence_floor,
            THE_GREATEST_RELEASE_SEQUENCE_PUBLISHED_WHEN_THIS_BUILD_WAS_MADE
        );
    }

    /// **A stored generation that is not the retained statement's own is
    /// damage, and the record goes with it.**
    ///
    /// **Raising it needed no signature**: the retained statement still
    /// verifies, so the raised number used to stand and refuse every genuine
    /// release for good and in silence — which is precisely the failure J7's
    /// discard exists to give a route out of.
    #[test]
    fn a_record_whose_stored_generation_is_not_the_statements_own_is_discarded() {
        let root = a_root();
        let mut raised = a_record(&root, &a_statement(5), 100);
        raised.greatest_accepted_key_set_generation = 9_000;
        assert_eq!(
            what_the_record_is_worth(&root.public, Some(&raised)),
            WhatTheRecordIsWorth::ItIsDiscardedAndTheFloorIsGone
        );
        // Lowered is damage too, and the same answer.
        let mut lowered = a_record(&root, &a_statement(5), 100);
        lowered.greatest_accepted_key_set_generation = 1;
        assert_eq!(
            what_the_record_is_worth(&root.public, Some(&lowered)),
            WhatTheRecordIsWorth::ItIsDiscardedAndTheFloorIsGone
        );
        // And an undamaged record still stands.
        let sound = a_record(&root, &a_statement(5), 100);
        assert!(matches!(
            what_the_record_is_worth(&root.public, Some(&sound)),
            WhatTheRecordIsWorth::ItsNumbersStand { .. }
        ));
    }

    /// **A raised stored generation no longer refuses a genuine release**,
    /// which is the thing the check above is for.
    #[test]
    fn a_raised_stored_generation_stops_refusing_genuine_releases() {
        let root = a_root();
        let mut raised = a_record(&root, &a_statement(5), 100);
        raised.greatest_accepted_key_set_generation = 9_000;
        let (bytes, signature) = signed_by(&root, &a_statement(6));
        let admitted =
            admit_a_key_set_statement(&root.public, Some(&raised), &bytes, &signature).unwrap();
        // The whole record went, so its release-sequence floor went with it and
        // what stands is the floor this build carries.
        assert_eq!(
            admitted.the_release_sequence_floor,
            THE_GREATEST_RELEASE_SEQUENCE_PUBLISHED_WHEN_THIS_BUILD_WAS_MADE
        );
    }

    #[test]
    fn a_record_whose_statement_is_not_a_statement_is_discarded_too() {
        let root = a_root();
        let bytes = b"not a statement".to_vec();
        let signature = a_signature_over(&root.secret, THE_KEY_SET_SIGNING_CONTEXT, &bytes);
        let record = TheTrustRecord {
            greatest_accepted_release_sequence: 100,
            greatest_accepted_key_set_generation: 5,
            the_statement_accepted_at_that_generation: bytes,
            its_root_signature: signature,
        };
        assert_eq!(
            what_the_record_is_worth(&root.public, Some(&record)),
            WhatTheRecordIsWorth::ItIsDiscardedAndTheFloorIsGone
        );
    }

    #[test]
    fn a_revoked_key_may_authorize_below_its_revocation_and_never_at_or_above_it() {
        let statement = KeySetStatement {
            revocations: vec![ARevocation {
                identity: "the first online key".into(),
                first_disallowed_release_sequence: 40,
            }],
            ..a_statement(2)
        };
        assert_eq!(statement.keys_that_may_authorize(39).len(), 1);
        assert!(statement.keys_that_may_authorize(40).is_empty());
        assert!(statement.keys_that_may_authorize(41).is_empty());
        // And below its own validity it was never a candidate.
        assert!(statement.keys_that_may_authorize(9).is_empty());
        assert_eq!(statement.keys_that_may_authorize(10).len(), 1);
    }

    /// Every thirty-two-byte byte-string literal written down anywhere in this
    /// workspace's Rust source, which is every seed
    /// [`test_only_keys::a_key_pair_from_a_written_down_seed`] could ever be
    /// handed from code in this repository.
    ///
    /// **The reading is of the source rather than of a list somebody keeps**,
    /// the same habit `composing.rs` already uses to hold the entry point's two
    /// copies together: a list would go stale the day a test wrote a new seed.
    fn every_seed_written_down_in_this_workspace() -> Vec<[u8; 32]> {
        fn walk(at: &std::path::Path, out: &mut Vec<[u8; 32]>) {
            let Ok(entries) = std::fs::read_dir(at) else {
                return;
            };
            for entry in entries.flatten() {
                let path = entry.path();
                let name = entry.file_name();
                let name = name.to_string_lossy();
                if name.starts_with('.') || name == "target" {
                    continue;
                }
                if path.is_dir() {
                    walk(&path, out);
                } else if path.extension().is_some_and(|it| it == "rs") {
                    if let Ok(text) = std::fs::read(&path) {
                        read_the_literals(&text, out);
                    }
                }
            }
        }
        fn read_the_literals(text: &[u8], out: &mut Vec<[u8; 32]>) {
            let mut at = 0;
            while at + 2 <= text.len() {
                if &text[at..at + 2] != b"b\"" {
                    at += 1;
                    continue;
                }
                let from = at + 2;
                let Some(length) = text[from..].iter().position(|byte| *byte == b'"') else {
                    return;
                };
                let literal = &text[from..from + length];
                // An escape means the written bytes are not the literal's
                // bytes, and a seed is never written that way here.
                if !literal.contains(&b'\\') {
                    if let Ok(seed) = <[u8; 32]>::try_from(literal) {
                        out.push(seed);
                    }
                }
                at = from + length + 1;
            }
        }
        let mut out = Vec::new();
        walk(
            &std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(".."),
            &mut out,
        );
        out
    }

    #[test]
    fn the_release_root_this_build_carries_is_not_a_key_this_project_can_make() {
        // **This replaced a test that asserted the root was absent**, on
        // 2026-09-06, the day the Creator's pair was generated and the public
        // half written above. The absent version could not survive a real root,
        // and deleting it would have taken the guard with it — so what it
        // guarded was kept and what it asserted was changed.
        //
        // **What it guarded, in the old test's own words:** a build carrying a
        // root made inside this project has a trust chain anybody reading the
        // repository can forge. That is still the danger and this is still the
        // test for it. The only way key material is made anywhere in this
        // workspace is a seed written down in the calling code, so the whole
        // population is readable: every thirty-two-byte literal in the source.
        // If the root is any of them, its secret half is in the repository.
        let root =
            THE_RELEASE_ROOT_THIS_BUILD_CARRIES.expect("this build carries a release root");

        let seeds = every_seed_written_down_in_this_workspace();
        assert!(
            seeds.len() >= 9,
            "only {} written-down seeds were read, so the reading has stopped working \
             rather than the workspace having lost its test keys",
            seeds.len()
        );
        for seed in seeds {
            assert_ne!(
                test_only_keys::a_key_pair_from_a_written_down_seed(seed).public,
                root,
                "the release root is the public half of a seed written down in this workspace"
            );
        }
    }

    /// Every Rust source file in this workspace, with its text.
    fn every_rust_file_in_this_workspace() -> Vec<(std::path::PathBuf, String)> {
        fn walk(at: &std::path::Path, out: &mut Vec<(std::path::PathBuf, String)>) {
            let Ok(entries) = std::fs::read_dir(at) else {
                return;
            };
            for entry in entries.flatten() {
                let path = entry.path();
                let name = entry.file_name();
                let name = name.to_string_lossy();
                if name.starts_with('.') || name == "target" {
                    continue;
                }
                if path.is_dir() {
                    walk(&path, out);
                } else if path.extension().is_some_and(|it| it == "rs") {
                    if let Ok(text) = std::fs::read_to_string(&path) {
                        out.push((path, text));
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

    /// **Wherever this workspace says where the release root's secret half is,
    /// it says both places.**
    ///
    /// **This is the test the class needed and did not have.** Four files
    /// describe that secret, and three of them went on saying it was in the
    /// Creator's password manager and nowhere else long after a working copy was
    /// put in the Mac's login keychain — so somebody reading one of them to work
    /// out what a compromise of the build machine would cost worked from a false
    /// premise. **The reading is of the source rather than of a list somebody
    /// keeps**, which is the same habit the seed guard above uses, so a fifth
    /// file written next year is caught the day it is written.
    #[test]
    fn nothing_in_this_workspace_names_the_password_manager_without_the_keychain() {
        let mut silent = Vec::new();
        let mut how_many_say_it = 0;
        for (path, text) in every_rust_file_in_this_workspace() {
            let lower = text.to_lowercase();
            if !lower.contains("password manager") {
                continue;
            }
            how_many_say_it += 1;
            if !lower.contains("keychain") {
                silent.push(path.display().to_string());
            }
        }
        assert!(
            how_many_say_it >= 4,
            "only {how_many_say_it} files were read as describing the secret, so the reading has \
             stopped working rather than the workspace having stopped describing it"
        );
        assert!(
            silent.is_empty(),
            "these files say where the release root's secret half is and do not name the \
             keychain it is actually read from: {silent:#?}"
        );
    }

    #[test]
    fn somebody_holds_the_secret_half_of_the_root_this_build_carries() {
        // **The positive half, and it is what the absent version could never
        // have.** Thirty-two bytes that verify nothing are indistinguishable
        // from a mistyped root until a release fails months later. This runs
        // the module's own verification — the same call
        // `admit_a_key_set_statement` makes — over a signature the secret half
        // made once, under a context no document of this module uses.
        let root =
            THE_RELEASE_ROOT_THIS_BUILD_CARRIES.expect("this build carries a release root");
        assert!(a_signature_verifies(
            &root,
            THE_ROOT_PROOF_CONTEXT,
            THE_ROOT_PROOF_MESSAGE,
            &THE_ROOT_PROOF_SIGNATURE
        ));
        // And the proof is inert: it is not a signature over either document.
        assert!(!a_signature_verifies(
            &root,
            THE_KEY_SET_SIGNING_CONTEXT,
            THE_ROOT_PROOF_MESSAGE,
            &THE_ROOT_PROOF_SIGNATURE
        ));
        assert!(!a_signature_verifies(
            &root,
            crate::release::THE_MANIFEST_SIGNING_CONTEXT,
            THE_ROOT_PROOF_MESSAGE,
            &THE_ROOT_PROOF_SIGNATURE
        ));
    }
}
