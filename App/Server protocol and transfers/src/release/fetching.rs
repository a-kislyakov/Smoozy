//! The fetch B8a wants: an exact HTTPS source, spoken to over the internet,
//! with a ranged resume.
//!
//! **What was here before this file, and what changed.** B8 requires every
//! artifact entry to carry an exact HTTPS source, B8a rules who serves one — a
//! separate public repository the Creator owns, and nowhere else — and until
//! now nothing in this crate reached the internet at all: the fetch was
//! [`super::composing::FetchesFromAnExactSourceTheManifestNamed`] and its one
//! implementation read a local file. **This is the implementation that speaks
//! to a repository.** The local-file one stays exactly where it was and is
//! still what the release test runs against, because nothing is published yet.
//!
//! **What it does not decide, and must not be read as deciding.** B8a leaves
//! the address itself outside every document downstream of it — *"no address,
//! no name and no rule for composing one is written here or anywhere downstream
//! of here"*. **So this file contains no address, no repository name and no
//! rule for making one.** It is handed a source, whole, and it fetches that.
//! **Nothing here derives a source from a version, a name or a pattern**, which
//! is B8's prohibition for the artifact applied to the address.
//!
//! **What it refuses, and each refusal is a clause rather than a preference.**
//!
//! - **Anything that is not an exact HTTPS address.** B8 says HTTPS and B10
//!   downloads only from an exact source the verified manifest named. **The
//!   rule is asked of [`super::verifying::an_https_address`]**, which is the one
//!   test of it in this workspace, rather than written out a second time here.
//! - **A body larger than the ceiling it was given.** For an artifact the
//!   ceiling is the byte length the verified manifest carries, so a source
//!   offering more than the manifest describes is refused before the extra
//!   bytes are held rather than after.
//! - **A resumed read that does not line up.** A partial answer whose own
//!   stated first byte is not the byte that was asked for is refused rather
//!   than spliced.
//!
//! **Nothing of the user's goes up with any of it**, which is the discipline
//! `Desktop app and delivery` E10 and E11 already state for the other fetch the
//! desktop makes: no account, no registration, no credential,
//! no computer identity, no name, no count. **A request from here is a plain
//! unauthenticated read of a public address**, which is what B8a's *public* half
//! is load-bearing for: a source that will not answer an unauthenticated
//! request cannot be named in a manifest at all.
//!
//! **The bytes are still not trusted when they arrive, and that is the point.**
//! Nothing here verifies anything. What comes back goes to
//! [`super::verifying::verify_what_was_downloaded`], which holds it to the byte
//! length and the digest of the artifact the verified manifest selected, and
//! only that comparison decides whether these bytes are Smoozy's.
//!
//! **Markers used below that belong to another document, each with the document
//! it belongs to.** `Desktop app and delivery` E10, E11.
//!
//! **How a clause marker is written in this crate, and it is a rule rather than
//! a habit.** A marker with no document beside it means a clause of this
//! crate's own plan, `Preparation/Architecture/Server protocol and
//! transfers.md`. Every other document's marker is written with that
//! document — at the site, or in the file's own header list above.

use std::io::Read;
use std::sync::Arc;
use std::time::Duration;

use super::composing::{FetchesFromAnExactSourceTheManifestNamed, WhyTheFetchFailed};
use super::verifying::an_https_address;

/// **What a request says about who is asking, and it is a constant.**
///
/// `Desktop app and delivery` E11 keeps everything about the user out of a
/// request the desktop makes to Smoozy's own hosting, and this fetch is held to
/// the same. **The name is all of it**: no application version, no operating
/// system, no computer name, no identity, no count. It is here at all so that
/// whoever reads the repository's own logs can tell Smoozy's fetches from
/// anything else that reads a public address.
pub const WHAT_A_REQUEST_SAYS_ABOUT_WHO_IS_ASKING: &str = "Smoozy";

/// **How many times one fetch will pick a stopped body up where it left off.**
///
/// A resume is what happens when the body stops part way, and this bounds it so
/// a source that hangs up at the same offset for ever ends rather than loops.
pub const HOW_MANY_TIMES_ONE_FETCH_WILL_RESUME: u32 = 4;

/// **How many times a source may hand the fetch on to another address.**
///
/// Hosting that serves large files commonly answers with the address the bytes
/// actually sit at, and refusing that outright would build in a failure nobody
/// would meet until the first release. **Two things keep it inside B10's
/// exact-source rule.** Every hop is required to be HTTPS, by the client's own
/// configuration, so the chain cannot leave the protected transport; and
/// **nothing about the bytes rests on where they came from** — they are held to
/// the byte length and the digest the verified manifest carries, so an address
/// reached by a redirect can supply the right bytes or be refused, and cannot
/// supply wrong ones that pass.
pub const HOW_MANY_TIMES_A_SOURCE_MAY_HAND_THE_FETCH_ON: u32 = 5;

/// How much of a body is read in one go.
const HOW_MUCH_IS_READ_AT_A_TIME: usize = 64 * 1024;

/// **The bound on reading one answer's body, and the value is decided rather
/// than measured** — the same shape the listing fetch next door already takes
/// for its own two bounds.
///
/// **Why there is one at all.** The two bounds set on the client cover
/// connecting and the answer's headers, and **neither covers the body**. The
/// client's own shipped source says so — `ureq` 3.4.1, `src/config.rs`, read
/// 2026-09-12: the headers bound *"stops applying once the response headers
/// have been received"*, and the body is covered separately by this setting,
/// which *"Defaults to `None`"*. No whole-request bound is set either. **So a
/// source that answers its headers and then trickles or goes silent without
/// closing holds this fetch for ever**, and the resume above does not reach it:
/// a resume fires when a body **stops**, and a body that never stops never
/// fires one.
///
/// **Ten minutes, and it is a bound on one answer rather than on the fetch.**
/// The same source says the budget *"is not restarted for each read"* and
/// starts when the headers arrive, so it is a ceiling on one body. **A body cut
/// by it is a body that stopped**, which is [`WhyTheBodyStopped::ItStoppedPartWay`]
/// and is picked up where it left off — so a legitimate large artifact on a slow
/// line costs resumes rather than bytes, up to
/// [`HOW_MANY_TIMES_ONE_FETCH_WILL_RESUME`] of them.
///
/// **What it costs if it is wrong.** Too short and a very slow line spends its
/// resumes on the clock rather than on real interruptions, and past the last one
/// the fetch is reported as unreachable where waiting would have worked. Too
/// long and the user waits up to this long, at each attempt, before Smoozy
/// admits the fetch is not going to end.
///
/// **Not the whole-request bound the listing uses.** That is the right
/// instrument for a small answer and the wrong one here: an artifact is
/// hundreds of megabytes, `Server protocol and transfers` R17 has this
/// project's own reading of a real line at well under a megabyte a second, and
/// a whole-request bound would cut a download that was going perfectly well.
pub const THE_BOUND_ON_READING_A_BODY: Duration = Duration::from_secs(600);

// -------------------------------------------- what this computer's processor
// -------------------------------------------- has to carry
//
// **No processor check, since 2026-10-24.** One was written here until
// 2026-09-13, then asked of `Security and licence` until the cryptography
// became `ring`'s, which runs on any 64-bit Intel or AMD processor (the
// Creator's eighteenth notes, note 3).

// ------------------------------------------------------------- the fetch

/// **A fetch that speaks to an exact HTTPS source over the internet.**
///
/// **One of these holds a connection pool and is meant to be kept**, not made
/// per request: a release fetch reads the offered release and then the artifact
/// it names, and the second of those is the large one.
pub struct AFetchOverHttps {
    client: ureq::Agent,
    /// **The most bytes any one fetch will hold.**
    ///
    /// **For an artifact this is the byte length the verified manifest
    /// carries**, so a source offering more than the manifest describes is
    /// refused at the moment it says so rather than after the extra bytes are
    /// in memory. For the offered release itself no length is known in advance,
    /// so it is whatever the caller is willing to hold.
    ///
    /// It is a field the caller may set rather than a value fixed at
    /// construction, because one fetch reads two things whose sizes are nothing
    /// like each other.
    pub the_most_bytes_it_will_read: u64,
    /// Every source it was actually asked for, in order — so a caller can hold
    /// the exact-source rule rather than assume it. The local-file fetch keeps
    /// the same record for the same reason.
    pub asked_for: Vec<String>,
    /// How many times a stopped body was picked up where it left off, over the
    /// life of this fetch.
    pub how_many_times_it_resumed: u32,
}

impl AFetchOverHttps {
    /// **Make one.**
    ///
    /// **The cryptography provider is handed to the client rather than
    /// installed for the whole program.** The client is built with no provider
    /// of its own precisely so that this one can be named here; installing a
    /// process-wide default instead would make the choice depend on which part
    /// of Smoozy ran first, and would fail differently the second time it was
    /// installed.
    ///
    /// **The root certificates are the bundled ones rather than this
    /// computer's**, which is the client configuration's own choice at
    /// `App/Cargo.toml`: the same code runs on the user's computer and on a
    /// bare rented machine, and nothing establishes that a bare machine carries
    /// a certificate bundle at all.
    pub fn reaching_the_internet(the_most_bytes_it_will_read: u64) -> AFetchOverHttps {
        let client = ureq::Agent::config_builder()
            .tls_config(
                ureq::tls::TlsConfig::builder()
                    .provider(ureq::tls::TlsProvider::Rustls)
                    .unversioned_rustls_crypto_provider(Arc::new(
                        rustls::crypto::ring::default_provider(),
                    ))
                    .build(),
            )
            // **No hop may leave HTTPS**, a redirect included.
            .https_only(true)
            .max_redirects(HOW_MANY_TIMES_A_SOURCE_MAY_HAND_THE_FETCH_ON)
            // The status is read here rather than raised, because an absent
            // artifact is an answer this fetch has a word for.
            .http_status_as_error(false)
            .user_agent(WHAT_A_REQUEST_SAYS_ABOUT_WHO_IS_ASKING)
            // **Nothing squeezed in transit**, so a byte offset means the same
            // thing on a resumed read as on a fresh one. The client is built
            // with no compression in it either; this says so as well, because a
            // feature switched on later would otherwise change what a resume
            // means without anybody noticing.
            .accept_encoding(ureq::config::AutoHeaderValue::None)
            .timeout_connect(Some(Duration::from_secs(30)))
            .timeout_recv_response(Some(Duration::from_secs(60)))
            // **The body's own bound**, which neither of the two above covers.
            // See [`THE_BOUND_ON_READING_A_BODY`] for the value and what it
            // costs if it is wrong.
            .timeout_recv_body(Some(THE_BOUND_ON_READING_A_BODY))
            .build()
            .new_agent();

        AFetchOverHttps {
            client,
            the_most_bytes_it_will_read,
            asked_for: Vec::new(),
            how_many_times_it_resumed: 0,
        }
    }

    /// **Read a source from the beginning, or pick up bytes already held.**
    ///
    /// **This is the whole of the fetch and the resume is not a second path.**
    /// A fresh read is this with nothing held, so the ranged request is
    /// exercised by every resume and the plain one by every first read — there
    /// is no branch that only runs after a failure and is therefore never
    /// tried.
    ///
    /// **What is held is spliced rather than trusted.** The answer to a ranged
    /// request has to say for itself which byte it starts at, and that byte has
    /// to be the one asked for; anything else is refused rather than joined on.
    /// **A source that answers with the whole thing instead** — which is what a
    /// source that does not do ranged reads will do — **starts over from zero**,
    /// discarding what was held, rather than appending a second copy of the
    /// beginning onto the end of the first.
    ///
    /// **Nothing here decides whether the bytes are the right ones.** That is
    /// the manifest's byte length and digest, checked afterwards, and it is why
    /// a resume is safe at all: the standard tool's own warning about resumed
    /// transfers is that a partial that does not match is spliced into a corrupt
    /// file with no error, and the answer to that here is that every fetched
    /// artifact is held to a digest before it goes anywhere.
    pub fn continue_a_body(
        &mut self,
        source: &str,
        already_held: Vec<u8>,
        ceiling: u64,
    ) -> Result<Vec<u8>, WhyTheFetchFailed> {
        // **B8's exact-HTTPS rule, asked of the one function in this workspace
        // that answers it** rather than spelled out a second time here. A
        // second copy of a rule is how the two come to disagree.
        if !an_https_address(source) {
            return Err(WhyTheFetchFailed::TheSourceIsNotAnExactHttpsAddress {
                source: source.to_owned(),
            });
        }
        self.asked_for.push(source.to_owned());

        let mut held = already_held;
        let mut resumes_left = HOW_MANY_TIMES_ONE_FETCH_WILL_RESUME;

        loop {
            let from = held.len() as u64;
            if from > ceiling {
                return Err(WhyTheFetchFailed::ItIsLargerThanTheCeiling { ceiling });
            }

            let mut asking = self.client.get(source);
            if from > 0 {
                asking = asking.header("Range", format!("bytes={from}-"));
            }
            let mut answer = match asking.call() {
                Ok(answer) => answer,
                Err(failed) => {
                    return Err(WhyTheFetchFailed::ItCouldNotBeReached {
                        said: failed.to_string(),
                    })
                }
            };

            let status = answer.status().as_u16();
            match status {
                200 => {
                    // The whole thing, whatever was asked for. Where something
                    // was held, the source did not do the ranged read and this
                    // is a fresh copy from byte zero.
                    held.clear();
                }
                206 => {
                    let said = answer
                        .headers()
                        .get("content-range")
                        .and_then(|value| value.to_str().ok())
                        .map(str::to_owned);
                    let Some(starts_at) = said
                        .as_deref()
                        .and_then(the_first_byte_a_partial_answer_says_it_starts_at)
                    else {
                        return Err(WhyTheFetchFailed::TheResumeDidNotLineUp {
                            said: said.unwrap_or_else(|| "it said nothing".into()),
                        });
                    };
                    if starts_at != from {
                        return Err(WhyTheFetchFailed::TheResumeDidNotLineUp {
                            said: format!("it starts at {starts_at} and {from} was asked for"),
                        });
                    }
                }
                404 | 410 => return Err(WhyTheFetchFailed::TheSourceHasNoSuchThing),
                _ => return Err(WhyTheFetchFailed::TheSourceAnsweredWith { status }),
            }

            // What the answer says is left to come, where it says.
            if let Some(coming) = answer.body().content_length() {
                if held.len() as u64 + coming > ceiling {
                    return Err(WhyTheFetchFailed::ItIsLargerThanTheCeiling { ceiling });
                }
            }

            let mut reading = answer.body_mut().as_reader();
            match read_what_comes_into(&mut reading, &mut held, ceiling) {
                Ok(()) => return Ok(held),
                Err(WhyTheBodyStopped::ItPassedTheCeiling) => {
                    return Err(WhyTheFetchFailed::ItIsLargerThanTheCeiling { ceiling })
                }
                Err(WhyTheBodyStopped::ItStoppedPartWay { said }) => {
                    if resumes_left == 0 {
                        return Err(WhyTheFetchFailed::ItCouldNotBeReached { said });
                    }
                    resumes_left -= 1;
                    self.how_many_times_it_resumed += 1;
                }
            }
        }
    }
}

impl FetchesFromAnExactSourceTheManifestNamed for AFetchOverHttps {
    fn fetch(&mut self, source: &str) -> Result<Vec<u8>, WhyTheFetchFailed> {
        self.continue_a_body(source, Vec::new(), self.the_most_bytes_it_will_read)
    }
}

/// Why a body did not finish.
enum WhyTheBodyStopped {
    ItPassedTheCeiling,
    ItStoppedPartWay { said: String },
}

/// **Read a body onto the end of what is held, stopping at the ceiling.**
///
/// **What was read before the stop is kept**, which is what makes the resume
/// possible at all: a read that threw its partial away would have nothing to
/// pick up from.
fn read_what_comes_into(
    reading: &mut impl Read,
    held: &mut Vec<u8>,
    ceiling: u64,
) -> Result<(), WhyTheBodyStopped> {
    let mut chunk = vec![0u8; HOW_MUCH_IS_READ_AT_A_TIME];
    loop {
        match reading.read(&mut chunk) {
            Ok(0) => return Ok(()),
            Ok(how_many) => {
                if held.len() as u64 + how_many as u64 > ceiling {
                    return Err(WhyTheBodyStopped::ItPassedTheCeiling);
                }
                held.extend_from_slice(&chunk[..how_many]);
            }
            Err(failed) if failed.kind() == std::io::ErrorKind::Interrupted => continue,
            Err(failed) => {
                return Err(WhyTheBodyStopped::ItStoppedPartWay {
                    said: failed.to_string(),
                })
            }
        }
    }
}

/// **The first byte a partial answer says it starts at.**
///
/// The shape is `bytes 100-199/376170648` — RFC 9110 §14.4, read 2026-09-07:
/// *"The Content-Range header field is sent in a single part 206 response to
/// indicate the partial range of the selected representation enclosed as the
/// message payload"*, with the first-byte position before the dash.
///
/// **Nothing is guessed at.** A unit that is not `bytes`, a missing dash, a
/// position that is not a number, or an unsatisfied range written `bytes * /n`
/// all give nothing, and nothing is what refuses the splice.
pub fn the_first_byte_a_partial_answer_says_it_starts_at(said: &str) -> Option<u64> {
    let said = said.trim();
    let rest = said.strip_prefix("bytes")?;
    let rest = rest.strip_prefix(' ')?.trim_start();
    let (first, _) = rest.split_once('-')?;
    first.trim().parse::<u64>().ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_partial_answer_is_read_for_where_it_starts_and_nothing_is_guessed() {
        assert_eq!(
            the_first_byte_a_partial_answer_says_it_starts_at("bytes 0-99/376170648"),
            Some(0)
        );
        assert_eq!(
            the_first_byte_a_partial_answer_says_it_starts_at("bytes 4096-8191/1048576"),
            Some(4096)
        );
        // A unit that is not bytes.
        assert_eq!(
            the_first_byte_a_partial_answer_says_it_starts_at("items 0-99/100"),
            None
        );
        // The unsatisfied form, which names no first byte.
        assert_eq!(
            the_first_byte_a_partial_answer_says_it_starts_at("bytes */1048576"),
            None
        );
        // No dash at all.
        assert_eq!(
            the_first_byte_a_partial_answer_says_it_starts_at("bytes 4096"),
            None
        );
        // Something that is not a number.
        assert_eq!(
            the_first_byte_a_partial_answer_says_it_starts_at("bytes first-99/100"),
            None
        );
        // The word run together with the number, which is not the shape.
        assert_eq!(
            the_first_byte_a_partial_answer_says_it_starts_at("bytes0-99/100"),
            None
        );
    }

    #[test]
    fn a_source_that_is_not_an_exact_https_address_is_refused_before_anything_is_asked() {
        let mut fetch = AFetchOverHttps::reaching_the_internet(1024);
        for refused in [
            "http://example.invalid/artifact",
            "ftp://example.invalid/artifact",
            "example.invalid/artifact",
            "/a/local/path",
            "HTTPS://example.invalid/artifact",
            "",
        ] {
            assert!(
                matches!(
                    fetch.continue_a_body(refused, Vec::new(), 1024),
                    Err(WhyTheFetchFailed::TheSourceIsNotAnExactHttpsAddress { .. })
                ),
                "{refused} was not refused"
            );
        }
        // **And nothing was asked for**, so the refusal happened before the
        // network rather than after it.
        assert!(fetch.asked_for.is_empty());
    }

    #[test]
    fn what_is_already_held_past_the_ceiling_is_refused_rather_than_asked_for() {
        let mut fetch = AFetchOverHttps::reaching_the_internet(1024);
        let held = vec![0u8; 200];
        assert_eq!(
            fetch.continue_a_body("https://example.invalid/artifact", held, 100),
            Err(WhyTheFetchFailed::ItIsLargerThanTheCeiling { ceiling: 100 })
        );
    }

    #[test]
    fn a_body_is_read_onto_what_is_held_and_the_ceiling_stops_it() {
        let mut held = b"the first half".to_vec();
        let mut coming = b", and the second".as_slice();
        assert!(read_what_comes_into(&mut coming, &mut held, 1024).is_ok());
        assert_eq!(held, b"the first half, and the second");

        // The ceiling is counted over what is held as well as what arrives.
        let mut held = vec![0u8; 900];
        let coming = vec![1u8; 200];
        let mut coming = coming.as_slice();
        assert!(matches!(
            read_what_comes_into(&mut coming, &mut held, 1000),
            Err(WhyTheBodyStopped::ItPassedTheCeiling)
        ));
    }

    /// **Every stretch of a fetch is bounded, and the body's own stretch is the
    /// one that had none.**
    ///
    /// **The client's own configuration is read back rather than the builder's
    /// calls being trusted**, which is what makes this a test of the fetch
    /// rather than of the source above it.
    #[test]
    fn every_stretch_of_a_fetch_is_bounded_and_the_body_is_one_of_them() {
        let fetch = AFetchOverHttps::reaching_the_internet(1024);
        let bounds = fetch.client.config().timeouts();
        assert!(bounds.connect.is_some(), "connecting is unbounded");
        assert!(
            bounds.recv_response.is_some(),
            "the answer's headers are unbounded"
        );
        assert_eq!(
            bounds.recv_body,
            Some(THE_BOUND_ON_READING_A_BODY),
            "reading the body is unbounded, so a source that stalls hangs the fetch"
        );
        // **And no whole-request bound**, which is right for the small listing
        // next door and wrong here: an artifact is hundreds of megabytes and one
        // would cut a download that was going perfectly well.
        assert!(bounds.global.is_none());
        assert!(bounds.per_call.is_none());
    }

    /// **No hop may leave HTTPS, a redirect included**, and the redirects are
    /// bounded. **The client's defaults are the opposite of both** — read from
    /// its own shipped source at `ureq` 3.4.1 `src/config.rs`: HTTPS-only off
    /// and ten redirects.
    #[test]
    fn no_hop_may_leave_https_and_the_redirects_are_bounded() {
        let fetch = AFetchOverHttps::reaching_the_internet(1024);
        assert!(fetch.client.config().https_only());
        assert_eq!(
            fetch.client.config().max_redirects(),
            HOW_MANY_TIMES_A_SOURCE_MAY_HAND_THE_FETCH_ON
        );
    }

    /// **No processor check is made here.** It left this file on 2026-09-13
    /// and the workspace on 2026-10-24, with the provider that needed it (the
    /// Creator's eighteenth notes, note 3). **This is what says it has not
    /// come back**: no instruction name is written in this file's working half
    /// at all.
    #[test]
    fn no_processor_check_is_made_here() {
        let whole = include_str!("fetching.rs");
        let working = whole
            .split_once("\n#[cfg(test)]")
            .expect("this file has a test module")
            .0;
        for instruction in ["pclmulqdq", "avx2", "pmull", "is_x86_feature_detected"] {
            assert!(
                !working.contains(instruction),
                "{instruction} is named here, so a processor check has come back"
            );
        }
    }

    /// **A fetch whose source accepts the connection and then says nothing
    /// ends at the client's own bound before the answer, says it could not be
    /// reached, and hands back no bytes to admit.** The dead-line desk
    /// coverage, 2026-10-19: the fetch is the release fetch's own client,
    /// pointed at a loopback listener that takes the connection and never
    /// answers — the shape of a line gone silent before any answer came.
    /// **The body's own ten-minute bound is not run here**, and a body cannot
    /// be carried at this desk at all: the client goes nowhere but HTTPS and
    /// trusts only the bundled roots, so no local listener can answer as far
    /// as a body. The test above reads that bound back.
    #[test]
    fn a_fetch_whose_source_goes_silent_ends_at_its_own_bound_and_admits_nothing() {
        let listener = std::net::TcpListener::bind(("127.0.0.1", 0)).expect("a loopback listener");
        let source = format!("https://127.0.0.1:{}/the-release", listener.local_addr().expect("an address").port());
        let held = std::thread::spawn(move || {
            let taken = listener.accept().map(|(stream, _)| stream);
            std::thread::sleep(Duration::from_secs(75));
            drop(taken);
        });
        let mut fetch = AFetchOverHttps::reaching_the_internet(1024);
        let bounds = fetch.client.config().timeouts();
        let began = std::time::Instant::now();
        let fetched = fetch.continue_a_body(&source, Vec::new(), 1024);
        let took = began.elapsed();
        println!("a silent source came to {fetched:?} in {took:?}; the bounds are {bounds:?}");
        assert!(
            matches!(fetched, Err(WhyTheFetchFailed::ItCouldNotBeReached { ref said }) if !said.is_empty()),
            "{fetched:?}"
        );
        let (connecting, the_answer) = (
            bounds.connect.expect("a bound on connecting"),
            bounds.recv_response.expect("a bound on the answer"),
        );
        let near = |bound: Duration| took + Duration::from_secs(1) >= bound && took <= bound + Duration::from_secs(5);
        assert!(
            near(connecting) || near(the_answer),
            "the silent fetch ended after {took:?}, at neither of its own bounds ({connecting:?}, {the_answer:?})"
        );
        assert_eq!(fetch.how_many_times_it_resumed, 0, "a fetch that never had an answer resumed");
        drop(held);
    }
}
