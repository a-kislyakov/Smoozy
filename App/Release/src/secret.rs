//! **Where the signing secret is, and how briefly it is held.**
//!
//! **The secret is in the Mac's keychain and in the Creator's password manager
//! and nowhere else.** It is not in this repository, it has never been on a
//! build runner, and nothing here writes it to a file, a log or an argument
//! list. It is read at the moment it signs and the buffer is overwritten
//! afterwards.
//!
//! **Why this crate reads the keychain itself rather than being handed the
//! value.** A release runner that read the secret into a shell variable to pipe
//! it in would have put it in that session's history, and a value passed as a
//! command-line argument is readable by every process on the machine while the
//! command runs. Reading it here means the release runner never holds it at all.
//!
//! **Two entries, from 2026-10-25** (step 11's D8). The first is the server
//! package's release root, below. The second is the desktop update's
//! **everyday key** — `Desktop app and delivery` E19's current online signing
//! key, which signs the desktop update file's manifest —
//! [`THE_UPDATE_KEYCHAIN_SERVICE`]. Both are read the same way, by the same
//! [`ask_the_keychain`] and judged by the same [`the_secret_in`], and neither
//! is ever the other: each is proved against what the build carries for its own
//! trust set before anything is signed (`State, identity and recovery` J10).

use smoozy_desktop::updating::trust::{TheCarriedSet, TrustStatement};
use smoozy_server_protocol::release::trust::{
    THE_RELEASE_ROOT_THIS_BUILD_CARRIES, THE_ROOT_PROOF_CONTEXT, THE_ROOT_PROOF_MESSAGE,
    THE_ROOT_PROOF_SIGNATURE,
};
use smoozy_server_protocol::release::{a_signature_over, a_signature_verifies, PublicKeyBytes};

/// **The keychain entry the release signing secret is in.** Creator-stated,
/// 2026-09-06.
///
/// # What is established about reading this entry, and what is not
///
/// **Reading it out of the keychain did not prompt. Label: observed in a test,
/// 2026-09-06.** A throwaway keychain item was stored, read back and removed,
/// and the read was silent. **So any process running as the Creator could print
/// the key that anchors every server package Smoozy will ever place**, and that
/// is a trade the Creator took with the cost stated at the time: anybody with
/// their unlocked Mac could sign a release, and the same person could change
/// Smoozy's source.
///
/// **The entry was then re-stored with its access restricted, on 2026-09-06, so
/// that a read asks for the login password rather than being silent.** **What
/// the restricted entry actually does on a read has not been read back**, and
/// nothing below depends on either answer: the code treats *the tool did not
/// hand it over* as one thing whatever caused it.
///
/// **Where this is against no clause, said so plainly.** `Server protocol and
/// transfers` B9's word *offline* is about the public half the desktop carries
/// and about replacing the root, not about where the secret sleeps between
/// releases. **None of this is a deviation from B9 and it must not be written up
/// as one.**
pub const THE_KEYCHAIN_SERVICE: &str = "Smoozy release signing key";

/// The account on that entry. See [`THE_KEYCHAIN_SERVICE`] for what is
/// established about reading it.
pub const THE_KEYCHAIN_ACCOUNT: &str = "smoozy-release";

/// **The keychain tool's own ending that means *there is no such entry*, and
/// nobody on this project has read what it is.**
///
/// **This is an unchecked fact and it is written as one rather than guessed.**
/// The tool's own manual page on this Mac, read 2026-09-06, documents its
/// options and its commands and **states no exit status at all**. So there is
/// nothing here to map an ending to a cause with, and until somebody reads a
/// publication that says which ending means which, **every refusal is reported
/// as the entry not having been handed over** — which names both possibilities
/// and asserts neither.
///
/// **What settles it:** one reading of that tool's own publication of its exit
/// statuses. No keychain needed. **What it buys:** a release runner meeting a
/// genuinely missing entry is told so in one sentence instead of two.
///
/// **Why the safe side is this one.** Saying *there is no such entry* about an
/// entry that is sitting there is a false statement about the one key everything
/// rests on, made at exactly the moment somebody is trying to make a release.
/// Saying *it was not handed over* about an entry that really is missing costs
/// the reader one extra sentence.
pub const THE_ENDING_THAT_MEANS_THERE_IS_NO_SUCH_ENTRY: Option<i32> = None;

/// **The secret, held for as long as it takes to sign and no longer.**
///
/// **It carries no way to be printed, formatted or compared**, on purpose: the
/// only thing that can be done with one is pass it to
/// [`smoozy_server_protocol::release::a_signature_over`], and its buffer is
/// overwritten when it goes out of scope.
pub struct TheSigningSecret {
    bytes: [u8; 32],
}

impl TheSigningSecret {
    pub fn bytes(&self) -> &[u8; 32] {
        &self.bytes
    }
}

impl Drop for TheSigningSecret {
    /// **Overwrite the bytes when the secret goes out of scope.**
    ///
    /// **What this is worth is stated rather than overstated.** It closes the
    /// ordinary case — the buffer being handed back to the allocator with the
    /// key still in it — and it does not defend against a copy the optimiser or
    /// the operating system made elsewhere. Nothing on this project turns on
    /// the stronger claim: the process lives for one release and exits.
    fn drop(&mut self) {
        overwrite(&mut self.bytes);
    }
}

/// Overwrite a buffer that held key material.
///
/// **The write is volatile so the compiler does not remove it**, the value
/// never being read again.
fn overwrite(bytes: &mut [u8]) {
    for byte in bytes.iter_mut() {
        unsafe { std::ptr::write_volatile(byte, 0) };
    }
}

/// **A secret made from a seed written down in a test, and nowhere else.**
///
/// **Tests only, and it cannot become anything else**: the only other way to
/// make a [`TheSigningSecret`] is
/// [`read_the_signing_secret_from_the_keychain`], which refuses anything that
/// is not the root this build carries.
#[cfg(test)]
pub fn for_tests_only(seed: [u8; 32]) -> TheSigningSecret {
    TheSigningSecret { bytes: seed }
}

/// **What the keychain tool answered, which is this crate's own seam.**
///
/// **It exists so that what an answer means can be decided and tested without a
/// keychain.** Reading the keychain is one function; deciding what its answer
/// means is another, and only the first needs a Mac with the entry in it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WhatTheKeychainAnswered {
    /// The tool's own ending, or nothing where it was ended by a signal.
    pub ending: Option<i32>,
    /// What it wrote out. **On success this is the secret**, so it is
    /// overwritten as soon as it has been read.
    pub written: Vec<u8>,
    /// What it said on its complaints stream, in its own words.
    pub said: String,
}

/// **Why the secret was not obtained**, and none of these says what it is.
#[derive(Debug)]
pub enum WhyTheSecretWasNotObtained {
    /// The keychain tool could not be run at all — this is a Mac's own command
    /// and a release is made on the Mac.
    TheKeychainCouldNotBeAsked { said: String },
    /// **The entry is not there.** The release runner's brief says what to do:
    /// stop and tell the Creator, whose password manager holds the master copy.
    ///
    /// **Nothing reaches this today**, because
    /// [`THE_ENDING_THAT_MEANS_THERE_IS_NO_SUCH_ENTRY`] is not established. See
    /// that constant.
    ThereIsNoSuchEntry,
    /// **The keychain did not hand the entry over, and this build cannot say
    /// which refusal it was.**
    ///
    /// **The entry may be there and the read may have been cancelled or
    /// refused** — it is stored with its access restricted, so a read is expected
    /// to ask for the login password — **or the entry may not be there at all.**
    /// The tool's own ending and its own words are carried so a reader can see
    /// what happened, and **nothing here asserts the key is missing.**
    ///
    /// **Until 2026-09-06 every non-zero ending was reported as the entry not
    /// being there**, so a read the Creator cancelled said the key was missing
    /// while it sat in the keychain.
    TheEntryWasNotHandedOver {
        the_tools_own_ending: Option<i32>,
        said: String,
    },
    /// The entry is there and what it holds is not thirty-two bytes written as
    /// hexadecimal.
    ItIsNotThirtyTwoBytesOfHexadecimal,
    /// **The entry is there and it is not this build's release root.** Either
    /// the key was rotated and this build was not rebuilt, or the entry is
    /// somebody else's. **Nothing is signed with it.**
    ItIsNotTheReleaseRootThisBuildCarries,
    /// The build carries no release root at all, so nothing it signed could be
    /// verified by anybody.
    ThisBuildCarriesNoReleaseRoot,
}

/// **Read the signing secret out of the keychain and prove it is the root this
/// build carries.**
///
/// **The proof is this project's own signature check and not a library call
/// written for the occasion.** The secret signs
/// [`THE_ROOT_PROOF_MESSAGE`] under [`THE_ROOT_PROOF_CONTEXT`] — a context
/// nothing in the module ever verifies a document under, so the result is inert
/// — and the signature must be the one committed beside the public key on the
/// day the pair was made. Ed25519 signing is deterministic, so that comparison
/// holds exactly when the secret is the other half of
/// [`THE_RELEASE_ROOT_THIS_BUILD_CARRIES`].
///
/// **A wrong entry is caught before a single document is signed**, which is the
/// point: a release signed with a key no shipped build carries verifies for
/// nobody, and it would look completely normal until the day somebody tried to
/// fetch it.
pub fn read_the_signing_secret_from_the_keychain(
) -> Result<(TheSigningSecret, PublicKeyBytes), WhyTheSecretWasNotObtained> {
    let Some(root) = THE_RELEASE_ROOT_THIS_BUILD_CARRIES else {
        return Err(WhyTheSecretWasNotObtained::ThisBuildCarriesNoReleaseRoot);
    };

    let mut answered = ask_the_keychain(THE_KEYCHAIN_SERVICE, THE_KEYCHAIN_ACCOUNT)?;
    let bytes = the_secret_in(&mut answered, THE_ENDING_THAT_MEANS_THERE_IS_NO_SUCH_ENTRY)?;
    let secret = TheSigningSecret { bytes };

    let proof = a_signature_over(
        secret.bytes(),
        THE_ROOT_PROOF_CONTEXT,
        THE_ROOT_PROOF_MESSAGE,
    );
    if proof != THE_ROOT_PROOF_SIGNATURE {
        return Err(WhyTheSecretWasNotObtained::ItIsNotTheReleaseRootThisBuildCarries);
    }
    Ok((secret, root))
}

/// **Run the keychain tool and carry back what it answered, whatever that is.**
///
/// **It judges nothing.** Whether an answer is the secret, an absence or a
/// refusal is [`the_secret_in`]'s, which needs no keychain and is tested without
/// one. **It names the entry it asks for** (from 2026-10-25, when the second
/// entry arrived), and the only two entries it is ever asked for are the two
/// named in this file.
fn ask_the_keychain(
    service: &str,
    account: &str,
) -> Result<WhatTheKeychainAnswered, WhyTheSecretWasNotObtained> {
    let asked = std::process::Command::new("security")
        .arg("find-generic-password")
        .arg("-w")
        .arg("-s")
        .arg(service)
        .arg("-a")
        .arg(account)
        .output()
        .map_err(|failed| WhyTheSecretWasNotObtained::TheKeychainCouldNotBeAsked {
            said: failed.to_string(),
        })?;
    Ok(WhatTheKeychainAnswered {
        ending: asked.status.code(),
        written: asked.stdout,
        said: String::from_utf8_lossy(&asked.stderr).trim().to_owned(),
    })
}

/// **What the keychain's answer means: the secret, or which failure it was.**
///
/// **The two refusals are told apart here and nowhere else**, and they are told
/// apart on the tool's own ending. `the_ending_that_means_there_is_no_such_entry`
/// is an argument rather than the constant so that both branches can be run
/// without a keychain and without knowing what the real ending is.
///
/// **Where the ending is not that one the answer is *it was not handed over***,
/// which names both possibilities and asserts neither. See
/// [`THE_ENDING_THAT_MEANS_THERE_IS_NO_SUCH_ENTRY`] for why that is the safe
/// side and what settles it.
///
/// **The tool's own output buffer held the key**, so it is overwritten here
/// rather than being left to be dropped.
fn the_secret_in(
    answered: &mut WhatTheKeychainAnswered,
    the_ending_that_means_there_is_no_such_entry: Option<i32>,
) -> Result<[u8; 32], WhyTheSecretWasNotObtained> {
    if answered.ending != Some(0) {
        if let Some(that_one) = the_ending_that_means_there_is_no_such_entry {
            if answered.ending == Some(that_one) {
                return Err(WhyTheSecretWasNotObtained::ThereIsNoSuchEntry);
            }
        }
        return Err(WhyTheSecretWasNotObtained::TheEntryWasNotHandedOver {
            the_tools_own_ending: answered.ending,
            said: answered.said.clone(),
        });
    }
    let secret = thirty_two_bytes_of_hexadecimal(&answered.written);
    overwrite(&mut answered.written);
    secret.ok_or(WhyTheSecretWasNotObtained::ItIsNotThirtyTwoBytesOfHexadecimal)
}

/// Read thirty-two bytes written as hexadecimal, and nothing else.
///
/// **Whitespace at either end is allowed and nothing in the middle is**, the
/// keychain tool ending its answer with a newline.
fn thirty_two_bytes_of_hexadecimal(written: &[u8]) -> Option<[u8; 32]> {
    let text = std::str::from_utf8(written).ok()?.trim();
    if text.len() != 64 {
        return None;
    }
    let mut out = [0u8; 32];
    for (at, pair) in text.as_bytes().chunks(2).enumerate() {
        let pair = std::str::from_utf8(pair).ok()?;
        out[at] = u8::from_str_radix(pair, 16).ok()?;
    }
    Some(out)
}

// ------------------------------------------- the desktop update's everyday key

/// **The keychain entry the desktop update's everyday key is in** — `Desktop
/// app and delivery` E19's current online signing key, the one that signs the
/// manifest inside the desktop update file. Build's name, step 11's D8,
/// 2026-10-25.
///
/// **It is named after the item the Creator keeps the same key under in their
/// password manager** (*Smoozy update everyday key*, step 11's D3 account), so
/// the two copies of one key carry one name. **It is never the release root's
/// entry** ([`THE_KEYCHAIN_SERVICE`]): the two keys belong to two trust sets
/// that share nothing (`State, identity and recovery` J10).
///
/// **What it holds**: the key's secret text, thirty-two bytes written as
/// sixty-four hexadecimal characters, exactly as the release root's entry does.
/// **Who makes it**: the Creator, with their word, on the day the update keys
/// are made — a keychain entry is theirs. Nothing in this project writes it.
pub const THE_UPDATE_KEYCHAIN_SERVICE: &str = "Smoozy update everyday key";

/// The account on that entry — the same account the release root's entry uses,
/// because both are this program's.
pub const THE_UPDATE_KEYCHAIN_ACCOUNT: &str = "smoozy-release";

/// **The context the everyday key's probe is signed under, and nothing ever
/// verifies a document under it.** It is none of the desktop update's three
/// contexts and none of the server package's, and a test says so. The probe is
/// made in memory, compared, and dropped; it is never written anywhere.
pub const THE_ONLINE_KEY_PROBE_CONTEXT: &[u8] =
    b"smoozy.release.desktop-update.online-key-probe.1\0";

/// The bytes the probe covers.
pub const THE_ONLINE_KEY_PROBE_MESSAGE: &[u8] =
    b"which online key of the desktop update trust set is this";

/// **The everyday key, held for as long as it takes to sign, with the
/// identifier the build's trust statement gives it.**
///
/// It carries no way to be printed, like [`TheSigningSecret`] inside it, whose
/// buffer is overwritten when it goes out of scope.
pub struct TheUpdateSigningKey {
    secret: TheSigningSecret,
    identifier: String,
}

impl TheUpdateSigningKey {
    pub fn secret(&self) -> &TheSigningSecret {
        &self.secret
    }

    /// **The identifier the manifest names it by** — the one the build's trust
    /// statement puts this key in force under.
    pub fn identifier(&self) -> &str {
        &self.identifier
    }
}

/// **An everyday key made from a seed written down in a test, under an
/// identifier the test names, and nothing else.** Tests only; the other way to
/// make one is [`read_the_update_signing_key_from_the_keychain`], which refuses
/// a key the build's statement does not allow.
#[cfg(test)]
pub fn update_key_for_tests_only(seed: [u8; 32], identifier: &str) -> TheUpdateSigningKey {
    TheUpdateSigningKey {
        secret: TheSigningSecret { bytes: seed },
        identifier: identifier.to_owned(),
    }
}

/// **Why the everyday key was not obtained**, and none of these says what it is.
#[derive(Debug)]
pub enum WhyTheUpdateKeyWasNotObtained {
    /// **The build carries master keys and no trust statement**, so there is no
    /// everyday key it allows. A build like that is broken, and nothing can be
    /// signed for it.
    ThisBuildCarriesNoTrustStatement,
    /// **The keychain did not hand the entry over, or what it handed over is
    /// not a key** — the release root's own reasons, read by the same rule.
    TheKeychain(WhyTheSecretWasNotObtained),
    /// **The entry holds a key and it is no everyday key the build's trust
    /// statement puts in force at this release sequence**: another key, a
    /// revoked one, or one not yet active. Nothing is signed with it — a
    /// manifest it signed would be believed by no copy of Smoozy.
    ItIsNoOnlineKeyThisBuildAllows,
}

/// **Read the everyday key out of the keychain and prove it is an online key
/// the build's own trust statement allows at this release sequence.**
///
/// **The proof is the same signature the manifest will carry, made over an
/// inert probe**: the secret signs [`THE_ONLINE_KEY_PROBE_MESSAGE`] under
/// [`THE_ONLINE_KEY_PROBE_CONTEXT`], and each online key the statement puts in
/// force at `release_sequence` is tried against it. The one it verifies under
/// is this key, and its identifier is what the manifest names. **A wrong entry
/// is caught before anything is signed**, as the release root's is.
pub fn read_the_update_signing_key_from_the_keychain(
    carried: &TheCarriedSet,
    release_sequence: u64,
) -> Result<TheUpdateSigningKey, WhyTheUpdateKeyWasNotObtained> {
    let statement = the_statement_the_build_carries(carried)?;
    let mut answered = ask_the_keychain(THE_UPDATE_KEYCHAIN_SERVICE, THE_UPDATE_KEYCHAIN_ACCOUNT)
        .map_err(WhyTheUpdateKeyWasNotObtained::TheKeychain)?;
    the_update_key_in(
        &mut answered,
        &statement,
        release_sequence,
        THE_ENDING_THAT_MEANS_THERE_IS_NO_SUCH_ENTRY,
    )
}

/// The trust statement the build carries, read. **Its signature is not judged
/// here**: the update file is verified whole, through the desktop's own
/// reading, before anything is written.
fn the_statement_the_build_carries(
    carried: &TheCarriedSet,
) -> Result<TrustStatement, WhyTheUpdateKeyWasNotObtained> {
    carried
        .statement
        .as_ref()
        .and_then(|(bytes, _)| TrustStatement::read(bytes))
        .ok_or(WhyTheUpdateKeyWasNotObtained::ThisBuildCarriesNoTrustStatement)
}

/// **What the keychain's answer means for the everyday key**: the key and its
/// identifier, or which failure it was. It needs no keychain, and is this
/// file's stand-in seam for the second entry as [`the_secret_in`] is for the
/// first.
fn the_update_key_in(
    answered: &mut WhatTheKeychainAnswered,
    statement: &TrustStatement,
    release_sequence: u64,
    the_ending_that_means_there_is_no_such_entry: Option<i32>,
) -> Result<TheUpdateSigningKey, WhyTheUpdateKeyWasNotObtained> {
    let bytes = the_secret_in(answered, the_ending_that_means_there_is_no_such_entry)
        .map_err(WhyTheUpdateKeyWasNotObtained::TheKeychain)?;
    let secret = TheSigningSecret { bytes };
    let probe = a_signature_over(
        secret.bytes(),
        THE_ONLINE_KEY_PROBE_CONTEXT,
        THE_ONLINE_KEY_PROBE_MESSAGE,
    );
    let found = statement.online_keys.iter().find(|key| {
        statement
            .the_key_that_may_sign(&key.identifier, release_sequence)
            .is_some()
            && a_signature_verifies(
                &key.public_key,
                THE_ONLINE_KEY_PROBE_CONTEXT,
                THE_ONLINE_KEY_PROBE_MESSAGE,
                &probe,
            )
    });
    match found {
        Some(key) => Ok(TheUpdateSigningKey {
            identifier: key.identifier.clone(),
            secret,
        }),
        None => Err(WhyTheUpdateKeyWasNotObtained::ItIsNoOnlineKeyThisBuildAllows),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hexadecimal_is_read_exactly_or_not_at_all() {
        let text = "00112233445566778899aabbccddeeff00112233445566778899aabbccddeeff";
        let read = thirty_two_bytes_of_hexadecimal(format!("{text}\n").as_bytes()).unwrap();
        assert_eq!(read[0], 0x00);
        assert_eq!(read[1], 0x11);
        assert_eq!(read[31], 0xff);
        // Too short, too long, and not hexadecimal at all.
        assert!(thirty_two_bytes_of_hexadecimal(b"00").is_none());
        assert!(thirty_two_bytes_of_hexadecimal(format!("{text}00").as_bytes()).is_none());
        assert!(thirty_two_bytes_of_hexadecimal(
            b"zz112233445566778899aabbccddeeff00112233445566778899aabbccddeeff"
        )
        .is_none());
        // A value with a space in the middle is not a key, however long it is.
        assert!(thirty_two_bytes_of_hexadecimal(
            b"00 12233445566778899aabbccddeeff00112233445566778899aabbccddeeff"
        )
        .is_none());
    }

    fn an_answer(ending: Option<i32>, written: &[u8], said: &str) -> WhatTheKeychainAnswered {
        WhatTheKeychainAnswered {
            ending,
            written: written.to_vec(),
            said: said.to_owned(),
        }
    }

    /// **A refusal that is not the not-there ending is never reported as the
    /// entry being missing**, which is the whole of this repair: a read the
    /// Creator cancels used to say the key was gone while it sat in the
    /// keychain.
    #[test]
    fn a_refusal_this_build_cannot_name_is_not_reported_as_a_missing_entry() {
        for ending in [Some(1), Some(51), Some(128), None] {
            let mut answered = an_answer(ending, b"", "the user cancelled the operation");
            match the_secret_in(&mut answered, THE_ENDING_THAT_MEANS_THERE_IS_NO_SUCH_ENTRY) {
                Err(WhyTheSecretWasNotObtained::TheEntryWasNotHandedOver {
                    the_tools_own_ending,
                    said,
                }) => {
                    assert_eq!(the_tools_own_ending, ending);
                    assert_eq!(said, "the user cancelled the operation");
                }
                other => panic!("{ending:?} was read as {other:?}"),
            }
        }
    }

    /// **And where the ending that means *not there* is known, that branch is
    /// the one taken.** The constant is not established, so this runs the rule
    /// against an ending named here rather than against the real one.
    #[test]
    fn the_ending_that_means_there_is_no_such_entry_is_told_apart_when_it_is_known() {
        let mut answered = an_answer(Some(44), b"", "the item could not be found");
        assert!(matches!(
            the_secret_in(&mut answered, Some(44)),
            Err(WhyTheSecretWasNotObtained::ThereIsNoSuchEntry)
        ));
        // A different ending, with the same constant in force, is still the
        // other branch.
        let mut answered = an_answer(Some(51), b"", "authorization denied");
        assert!(matches!(
            the_secret_in(&mut answered, Some(44)),
            Err(WhyTheSecretWasNotObtained::TheEntryWasNotHandedOver { .. })
        ));
    }

    /// **An ending nobody has established never matches by accident.** The
    /// constant is absent, and an answer whose ending is also absent — a tool
    /// ended by a signal — must not fall into the not-there branch on that.
    #[test]
    fn an_absent_ending_does_not_match_an_absent_constant() {
        let mut answered = an_answer(None, b"", "killed");
        assert!(matches!(
            the_secret_in(&mut answered, None),
            Err(WhyTheSecretWasNotObtained::TheEntryWasNotHandedOver {
                the_tools_own_ending: None,
                ..
            })
        ));
    }

    /// **A successful answer is the key, and the tool's buffer is overwritten
    /// where it stands** rather than being left to be dropped.
    #[test]
    fn a_successful_answer_is_the_key_and_the_buffer_that_held_it_is_overwritten() {
        let text = "00112233445566778899aabbccddeeff00112233445566778899aabbccddeeff";
        let mut answered = an_answer(Some(0), format!("{text}\n").as_bytes(), "");
        let read = the_secret_in(&mut answered, None).expect("it is the key");
        assert_eq!(read[0], 0x00);
        assert_eq!(read[31], 0xff);
        assert!(answered.written.iter().all(|byte| *byte == 0));
    }

    /// **A successful answer that is not a key says so**, and its buffer is
    /// overwritten too.
    #[test]
    fn a_successful_answer_that_is_not_a_key_is_said_rather_than_used() {
        let mut answered = an_answer(Some(0), b"not a key at all\n", "");
        assert!(matches!(
            the_secret_in(&mut answered, None),
            Err(WhyTheSecretWasNotObtained::ItIsNotThirtyTwoBytesOfHexadecimal)
        ));
        assert!(answered.written.iter().all(|byte| *byte == 0));
    }

    #[test]
    fn a_buffer_that_held_key_material_is_overwritten() {
        let mut bytes = [7u8; 32];
        overwrite(&mut bytes);
        assert_eq!(bytes, [0u8; 32]);
        let mut written = b"a run of bytes from the keychain".to_vec();
        overwrite(&mut written);
        assert!(written.iter().all(|byte| *byte == 0));
    }

    // ------------------------------------------- the everyday key's entry

    use smoozy_desktop::updating::trust::{AnOnlineKey, WhichStatement, THE_STATEMENT_FORMAT};
    use smoozy_server_protocol::release::composing::test_only_keys::a_key_pair_from_a_written_down_seed;

    const AN_EVERYDAY_SEED: [u8; 32] = *b"an everyday key, for secret.rs. ";
    const ANOTHER_EVERYDAY_SEED: [u8; 32] = *b"a second everyday key, secret.rs";

    fn as_hexadecimal(seed: &[u8; 32]) -> String {
        seed.iter().map(|byte| format!("{byte:02x}")).collect()
    }

    /// A statement putting `online-1` in force from release 10, and
    /// `online-2` from release 50, with `online-0` revoked.
    fn a_statement_naming_two_keys() -> TrustStatement {
        TrustStatement {
            statement_format: THE_STATEMENT_FORMAT.into(),
            trust_sequence: 2,
            which: WhichStatement::Compromise,
            online_keys: vec![
                AnOnlineKey {
                    identifier: "online-1".into(),
                    public_key: a_key_pair_from_a_written_down_seed(AN_EVERYDAY_SEED).public,
                    first_release_sequence: 10,
                },
                AnOnlineKey {
                    identifier: "online-2".into(),
                    public_key: a_key_pair_from_a_written_down_seed(ANOTHER_EVERYDAY_SEED).public,
                    first_release_sequence: 50,
                },
            ],
            revoked_online_keys: vec!["online-0".into()],
            revoked_recovery_keys: Vec::new(),
        }
    }

    /// **The everyday key in the keychain is found under the identifier the
    /// build's statement gives it**, and the tool's buffer is overwritten.
    #[test]
    fn the_everyday_key_is_known_by_the_identifier_the_build_gives_it() {
        let statement = a_statement_naming_two_keys();
        let mut answered = an_answer(
            Some(0),
            format!("{}\n", as_hexadecimal(&AN_EVERYDAY_SEED)).as_bytes(),
            "",
        );
        let key = the_update_key_in(&mut answered, &statement, 10, None).expect("it is online-1");
        assert_eq!(key.identifier(), "online-1");
        assert_eq!(key.secret().bytes(), &AN_EVERYDAY_SEED);
        assert!(answered.written.iter().all(|byte| *byte == 0));

        let mut answered = an_answer(
            Some(0),
            as_hexadecimal(&ANOTHER_EVERYDAY_SEED).as_bytes(),
            "",
        );
        let key = the_update_key_in(&mut answered, &statement, 50, None).expect("it is online-2");
        assert_eq!(key.identifier(), "online-2");
    }

    /// **A key the build's statement does not put in force is refused before
    /// anything is signed**: another key altogether, the release root's own
    /// kind of key, and a key not yet active at this release sequence.
    #[test]
    fn a_key_the_build_does_not_allow_is_refused_before_anything_is_signed() {
        let statement = a_statement_naming_two_keys();
        let stranger = *b"a key no trust statement names. ";
        let mut answered = an_answer(Some(0), as_hexadecimal(&stranger).as_bytes(), "");
        assert!(matches!(
            the_update_key_in(&mut answered, &statement, 10, None),
            Err(WhyTheUpdateKeyWasNotObtained::ItIsNoOnlineKeyThisBuildAllows)
        ));
        assert!(answered.written.iter().all(|byte| *byte == 0));
        // `online-2` is in the statement and may sign only from release 50.
        let mut answered = an_answer(
            Some(0),
            as_hexadecimal(&ANOTHER_EVERYDAY_SEED).as_bytes(),
            "",
        );
        assert!(matches!(
            the_update_key_in(&mut answered, &statement, 49, None),
            Err(WhyTheUpdateKeyWasNotObtained::ItIsNoOnlineKeyThisBuildAllows)
        ));
    }

    /// **A revoked everyday key is refused**, even where its own public half
    /// is still written in the statement's set.
    #[test]
    fn a_revoked_everyday_key_is_refused() {
        let mut statement = a_statement_naming_two_keys();
        statement.revoked_online_keys.push("online-1".into());
        let mut answered = an_answer(Some(0), as_hexadecimal(&AN_EVERYDAY_SEED).as_bytes(), "");
        assert!(matches!(
            the_update_key_in(&mut answered, &statement, 10, None),
            Err(WhyTheUpdateKeyWasNotObtained::ItIsNoOnlineKeyThisBuildAllows)
        ));
    }

    /// **An absent or withheld entry refuses plainly**, by the release root's
    /// own rule: a refusal this build cannot name is never called missing, and
    /// a known not-there ending is.
    #[test]
    fn an_absent_or_withheld_everyday_key_refuses_by_the_same_rule_as_the_root() {
        let statement = a_statement_naming_two_keys();
        let mut answered = an_answer(Some(44), b"", "The specified item could not be found");
        assert!(matches!(
            the_update_key_in(&mut answered, &statement, 10, THE_ENDING_THAT_MEANS_THERE_IS_NO_SUCH_ENTRY),
            Err(WhyTheUpdateKeyWasNotObtained::TheKeychain(
                WhyTheSecretWasNotObtained::TheEntryWasNotHandedOver { the_tools_own_ending: Some(44), .. }
            ))
        ));
        let mut answered = an_answer(Some(44), b"", "The specified item could not be found");
        assert!(matches!(
            the_update_key_in(&mut answered, &statement, 10, Some(44)),
            Err(WhyTheUpdateKeyWasNotObtained::TheKeychain(
                WhyTheSecretWasNotObtained::ThereIsNoSuchEntry
            ))
        ));
        let mut answered = an_answer(Some(0), b"not a key\n", "");
        assert!(matches!(
            the_update_key_in(&mut answered, &statement, 10, None),
            Err(WhyTheUpdateKeyWasNotObtained::TheKeychain(
                WhyTheSecretWasNotObtained::ItIsNotThirtyTwoBytesOfHexadecimal
            ))
        ));
    }

    /// **A build carrying master keys and no statement allows no everyday
    /// key**, and the keychain is not asked.
    #[test]
    fn a_build_with_no_statement_allows_no_everyday_key() {
        let carried = TheCarriedSet {
            recovery_keys: Vec::new(),
            statement: None,
            greatest_release_sequence_published: 0,
        };
        assert!(matches!(
            read_the_update_signing_key_from_the_keychain(&carried, 1),
            Err(WhyTheUpdateKeyWasNotObtained::ThisBuildCarriesNoTrustStatement)
        ));
    }

    /// **The two entries are two, and the probe proves nothing any document
    /// could use**: its context is none of the desktop update's three and none
    /// of the server package's.
    #[test]
    fn the_two_entries_are_two_and_the_probe_is_inert() {
        assert_ne!(THE_UPDATE_KEYCHAIN_SERVICE, THE_KEYCHAIN_SERVICE);
        use smoozy_desktop::updating as update;
        use smoozy_server_protocol::release as server_package;
        for other in [
            update::THE_STATEMENT_SIGNING_CONTEXT,
            update::THE_MANIFEST_SIGNING_CONTEXT,
            update::THE_RECOVERY_PROOF_CONTEXT,
            server_package::THE_KEY_SET_SIGNING_CONTEXT,
            server_package::THE_MANIFEST_SIGNING_CONTEXT,
            server_package::trust::THE_ROOT_PROOF_CONTEXT,
        ] {
            assert_ne!(THE_ONLINE_KEY_PROBE_CONTEXT, other);
        }
    }
}
