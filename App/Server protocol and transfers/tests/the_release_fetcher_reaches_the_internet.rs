//! B8a's fetcher, run against the real internet rather than argued about.
//!
//! **Every test here is ignored by default and each says why in one word: it
//! needs the internet.** Nothing else in this workspace does. A test that fails
//! because a hosted runner has no route out, or because somebody else's server
//! is having a bad morning, is a test that teaches the next agent to ignore red,
//! and that is worse than no test. **They are run by hand, at the desk, and what
//! they said is written into the module's decisions record** — which is where
//! the account of any run of them belongs.
//!
//! Run them with:
//!
//! ```text
//! cargo test -p smoozy-server-protocol --test the_release_fetcher_reaches_the_internet -- --ignored
//! ```
//!
//! **What is not here, and it is the one thing that cannot be.** There is no
//! address to fetch a real Smoozy release from: B8a leaves the repository's
//! address outside every document downstream of it, and no release has ever
//! been published, deliberately — *"The first release is the Creator's word and
//! not a builder's."* **So the empty repository is met here as the state it
//! is** rather than worked around: an address with nothing at it is answered as
//! nothing being there, which is exactly what a repository nothing has been
//! published to answers.
//!
//! **The addresses below belong to other people and are read, never written
//! to.** Blender's own download server, because this project already depends on
//! it and it serves ranged reads; and the certificate test service the client's
//! own choice was proved against on 2026-09-07.
//!
//! **Markers used below that belong to another document, each with the document
//! it belongs to.** `Desktop app and delivery` E10, E11.

use smoozy_server_protocol::release::composing::{
    self, FetchesFromAnExactSourceTheManifestNamed, WhyTheFetchFailed,
};
use smoozy_server_protocol::release::fetching::AFetchOverHttps;
use smoozy_server_protocol::release::trust::{AnAllowedKey, KeySetStatement, THE_KEY_SET_FORMAT};
use smoozy_server_protocol::release::verifying::{
    verify_before_download, verify_what_was_downloaded, AnOfferedRelease,
};
use smoozy_server_protocol::release::{
    a_signature_over, THE_KEY_SET_SIGNING_CONTEXT, THE_MANIFEST_SIGNING_CONTEXT,
};
use smoozy_server_protocol::transfers::integrity::Digest;

/// A small real file that is served over HTTPS and answers a ranged read: 576
/// bytes, checked 2026-09-07.
const A_SMALL_REAL_FILE: &str =
    "https://download.blender.org/release/Blender4.2/blender-4.2.0.sha256";

/// The same server, and nothing at that name.
const AN_ADDRESS_WITH_NOTHING_AT_IT: &str =
    "https://download.blender.org/release/Blender4.2/there-is-no-such-thing-smoozy";

fn a_fetch() -> AFetchOverHttps {
    AFetchOverHttps::reaching_the_internet(4 * 1024 * 1024)
}

/// **A whole file comes back, and it is the same bytes every time.**
#[test]
#[ignore = "it needs the internet"]
fn a_source_answers_with_its_bytes() {
    let mut fetch = a_fetch();
    let bytes = fetch.fetch(A_SMALL_REAL_FILE).expect("the source answers");
    assert!(!bytes.is_empty());
    // **The exact-source rule, held rather than assumed**: it was asked for the
    // address it was given and for nothing else.
    assert_eq!(fetch.asked_for, vec![A_SMALL_REAL_FILE.to_string()]);

    let again = fetch.fetch(A_SMALL_REAL_FILE).expect("the source answers");
    assert_eq!(Digest::of(&bytes), Digest::of(&again));
}

/// **The ranged read returns its exact bytes, and the resume splices them into
/// the file that was there all along.**
///
/// This is the proof the resume rests on. A whole copy is taken first; then the
/// first half of that copy is handed back to the fetch as bytes already held,
/// which is exactly the state a body that stopped part way leaves behind; and
/// what comes back has to be the whole file, byte for byte.
#[test]
#[ignore = "it needs the internet"]
fn a_resume_picks_up_where_it_left_off_and_the_bytes_are_the_same_file() {
    let mut fetch = a_fetch();
    let whole = fetch.fetch(A_SMALL_REAL_FILE).expect("the source answers");
    assert!(whole.len() > 32, "the file is long enough to halve");

    for held_up_to in [1, whole.len() / 2, whole.len() - 1] {
        let held = whole[..held_up_to].to_vec();
        let spliced = fetch
            .continue_a_body(A_SMALL_REAL_FILE, held, 4 * 1024 * 1024)
            .expect("the source answers the ranged read");
        assert_eq!(
            spliced.len(),
            whole.len(),
            "a resume from {held_up_to} gave a different length"
        );
        assert_eq!(
            Digest::of(&spliced),
            Digest::of(&whole),
            "a resume from {held_up_to} gave different bytes"
        );
    }
}

/// **Nothing is there, and that is an answer rather than a failure.**
///
/// **This is the empty repository met as a real state.** A repository nothing
/// has been published to answers exactly this way, and so does an artifact that
/// has been withdrawn.
#[test]
#[ignore = "it needs the internet"]
fn an_absent_artifact_is_answered_as_absent() {
    let mut fetch = a_fetch();
    assert_eq!(
        fetch.fetch(AN_ADDRESS_WITH_NOTHING_AT_IT),
        Err(WhyTheFetchFailed::TheSourceHasNoSuchThing)
    );
}

/// **A body larger than the ceiling is refused, and the refusal happens at the
/// moment the source says how large it is.**
///
/// For an artifact the ceiling is the byte length the verified manifest
/// carries, so this is a source disagreeing with the manifest that named it.
#[test]
#[ignore = "it needs the internet"]
fn a_source_offering_more_than_the_ceiling_is_refused() {
    let mut fetch = a_fetch();
    fetch.the_most_bytes_it_will_read = 8;
    assert_eq!(
        fetch.fetch(A_SMALL_REAL_FILE),
        Err(WhyTheFetchFailed::ItIsLargerThanTheCeiling { ceiling: 8 })
    );
}

/// **A certificate that should not be trusted is refused, and it is proved by
/// three refusals and one acceptance rather than by one of either.**
///
/// A fetcher that refused everything would pass a test that only checked the
/// refusals, and a fetcher that verified nothing would pass one that only
/// checked the acceptance.
#[test]
#[ignore = "it needs the internet"]
fn a_certificate_that_should_not_be_trusted_is_refused_and_a_good_one_is_not() {
    let mut fetch = a_fetch();
    for refused in [
        "https://expired.badssl.com/",
        "https://self-signed.badssl.com/",
        "https://wrong.host.badssl.com/",
    ] {
        assert!(
            matches!(
                fetch.fetch(refused),
                Err(WhyTheFetchFailed::ItCouldNotBeReached { .. })
            ),
            "{refused} was not refused"
        );
    }
    let good = fetch
        .fetch("https://sha256.badssl.com/")
        .expect("a valid certificate is accepted");
    assert!(!good.is_empty());
}

/// **The whole of B10's first two verifications, over a real HTTPS source.**
///
/// **This is the closest thing to a real release that can be run while none is
/// published.** The signing keys are derived from seeds written out here, so
/// they are public and are not release keys; everything else is the real
/// mechanism — the root-signed key-set statement, the signed manifest, the
/// exact-tuple selection, the exact source, the fetch over HTTPS, and the byte
/// length and digest the manifest carries checked against what came back.
///
/// **What it establishes that the local-file fetch cannot**: the bytes that
/// satisfy the manifest are bytes that crossed the internet.
#[test]
#[ignore = "it needs the internet"]
fn a_verified_manifest_naming_a_real_source_is_fetched_and_the_bytes_are_what_it_says() {
    let mut fetch = a_fetch();

    // What is actually at that address today. The manifest is composed over
    // these bytes, exactly as a release is composed over an artifact that
    // already exists.
    let what_is_there = fetch.fetch(A_SMALL_REAL_FILE).expect("the source answers");

    let root = composing::test_only_keys::a_key_pair_from_a_written_down_seed(
        *b"a release root for this test onl",
    );
    let online = composing::test_only_keys::a_key_pair_from_a_written_down_seed(
        *b"an online signing key, this test",
    );
    // **At this build's own generation and one past its release-sequence
    // floor**, the release a desktop built today meets. *(Generation 1 and
    // sequence 7 until 2026-09-21, which verified only while `Server protocol
    // and transfers` B9's built-in pair was zero. This test is ignored by
    // default, so it did not go red with the rest when the pair was raised.)*
    let statement = KeySetStatement {
        statement_format: THE_KEY_SET_FORMAT.into(),
        generation: smoozy_server_protocol::release::trust::THE_GREATEST_KEY_SET_GENERATION_PUBLISHED_WHEN_THIS_BUILD_WAS_MADE,
        allowed: vec![AnAllowedKey {
            identity: "the online signing key".into(),
            public_key: online.public,
            valid_from_release_sequence: 1,
        }],
        revocations: Vec::new(),
    };
    let statement_bytes = statement.canonical_bytes();
    let manifest = composing::compose_a_manifest(
        smoozy_server_protocol::release::trust::THE_GREATEST_RELEASE_SEQUENCE_PUBLISHED_WHEN_THIS_BUILD_WAS_MADE
            + 1,
        "0.1.0",
        2,
        None,
        vec![composing::compose_an_artifact_entry(
            "linux-x86_64-gnu",
            composing::the_tuple_this_build_is_made_for(),
            &what_is_there,
            composing::THE_FIXED_ENTRY_POINT_NAME,
            &[A_SMALL_REAL_FILE.to_string()],
        )],
    );
    let manifest_bytes = manifest.canonical_bytes();
    let offered = AnOfferedRelease {
        its_root_signature: a_signature_over(
            &root.secret,
            THE_KEY_SET_SIGNING_CONTEXT,
            &statement_bytes,
        ),
        the_key_set_statements_bytes: statement_bytes,
        its_online_signature: a_signature_over(
            &online.secret,
            THE_MANIFEST_SIGNING_CONTEXT,
            &manifest_bytes,
        ),
        the_manifests_bytes: manifest_bytes,
    };

    let verified = verify_before_download(
        &root.public,
        None,
        &offered,
        &composing::the_tuple_this_build_is_made_for(),
        None,
    )
    .expect("the release verifies");

    // **The source is read out of the verified manifest and is not composed
    // here**, which is the whole of B8's prohibition applied to the address.
    let source = verified.selected.sources[0].clone();
    fetch.the_most_bytes_it_will_read = verified.selected.byte_length;
    let bytes = fetch.fetch(&source).expect("the source answers");

    verify_what_was_downloaded(&verified.selected, &source, &bytes)
        .expect("the downloaded bytes are the artifact the manifest names");

    // **And a resumed fetch of the same source satisfies the same manifest**,
    // which is what says the resume is not a second, weaker route to the bytes.
    let half = bytes[..bytes.len() / 2].to_vec();
    let spliced = fetch
        .continue_a_body(&source, half, verified.selected.byte_length)
        .expect("the source answers the ranged read");
    verify_what_was_downloaded(&verified.selected, &source, &spliced)
        .expect("the resumed bytes are the artifact the manifest names");
    assert!(fetch.how_many_times_it_resumed == 0, "nothing actually broke");
}

/// **Nothing about the user goes up**, which is `Desktop app and delivery` E10
/// and E11's discipline for the desktop's other fetch, kept here.
///
/// **This is a reading of the fetch rather than of the wire**, and it is worth
/// saying which: what it establishes is that the fetch carries no credential of
/// any kind and no cookie store, because the client is built without one, so
/// there is nothing for a source to set and nothing to send back on a second
/// request. **No account is possible and none is offered.**
#[test]
#[ignore = "it needs the internet"]
fn two_fetches_of_the_same_source_carry_nothing_from_the_first_to_the_second() {
    let mut fetch = a_fetch();
    let first = fetch.fetch(A_SMALL_REAL_FILE).expect("the source answers");
    let second = fetch.fetch(A_SMALL_REAL_FILE).expect("the source answers");
    assert_eq!(Digest::of(&first), Digest::of(&second));
    assert_eq!(fetch.asked_for.len(), 2);
}
