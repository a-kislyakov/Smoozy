//! **B10b: every read of the releases repository tries a second route, then
//! tries again, before it gives up** — the one helper both ways in go through.
//!
//! **Why it is here.** On 2026-11-09 the repository's plain download address
//! answered HTTP 504 during a GitHub incident and setup stopped, while GitHub's
//! REST API answered every time (the Creator's twenty-eighth notes, note 3). The
//! Creator: *"Try again before giving up is a good call"*, and on the API route,
//! *"try that before trying the first one again is even better."*
//!
//! **The order, per file, and it is B10b's.**
//!
//! 1. **The plain address.**
//! 2. **On a failure the service may get over** — no answer, a timeout, HTTP
//!    408, 429 or any 5xx, and on the API route also 403, its rate-limit
//!    refusal ([`the_service_may_get_over`]) — **the API route for the same
//!    file**: the release's list of files, then the file named in it, by the
//!    list's own address for it ([`the_api_route_for`],
//!    [`the_files_api_address_in`]).
//! 3. **Then the plain address again**, up to three more times, after the
//!    pauses in [`THE_PAUSES_BEFORE_THE_PLAIN_ADDRESS_AGAIN`].
//!
//! **The deadline: no try begins more than [`NO_TRY_BEGINS_LATER_THAN`] after
//! the first one failed**, and no pause begins that would end past it. The API
//! route's two requests count against it. A try already under way is held by
//! its own client's bounds and is not cut here.
//!
//! **What ends it at once, because it is an answer and not an outage.** A 404
//! or 410 on either route, the list not naming the file, a body past its
//! ceiling — anything [`the_service_may_get_over`] says no to.
//!
//! **Trust does not change, and nothing here decides anything about the
//! bytes.** B9, B10, `Desktop app and delivery` E15 and E16 hold whatever comes
//! back exactly as before, whichever route brought it. The API's list is
//! unsigned and only supplies an address: a wrong address delivers the right
//! bytes or is refused afterwards.
//!
//! **This narrows [`super::fetching`]'s *no rule for making* an address, once,
//! here**: B10b writes the two API forms of a plain releases address, and
//! nothing else in this crate composes one.
//!
//! **What a request says is the caller's**, unchanged on every route: each way
//! in hands its own client to [`read_by_either_route`] through
//! [`TriesOneAddress`], so B8's constant name and `Desktop app and delivery`
//! E11's version and system go on every request the route makes.
//!
//! **Markers used below that belong to another document, each with the document
//! it belongs to.** `Desktop app and delivery` E11, E15, E16.

use std::time::{Duration, Instant};

use super::composing::WhyTheFetchFailed;
use super::verifying::an_https_address;

/// **B10b: the pauses before each later try at the plain address** — a few
/// seconds apart, three of them.
pub const THE_PAUSES_BEFORE_THE_PLAIN_ADDRESS_AGAIN: [Duration; 3] = [
    Duration::from_secs(5),
    Duration::from_secs(10),
    Duration::from_secs(15),
];

/// **B10b's deadline: no try begins more than this after the first one
/// failed.** Two minutes covers the API route and every pause even when each
/// try takes a while to fail, and it stops a dead line from holding a setup
/// or an update check for long past the point where a person would try again.
pub const NO_TRY_BEGINS_LATER_THAN: Duration = Duration::from_secs(120);

/// **The most the API's list of a release's files may be.** A release's list
/// carries a few lines per file and the release's notes; a body larger than
/// this is not one.
pub const THE_MOST_THE_LIST_OF_FILES_MAY_BE: u64 = 1024 * 1024;

/// **What the list is asked for as** — the header GitHub's *REST API endpoints
/// for releases* page recommends (read 2026-11-09).
pub const THE_LIST_IS_ASKED_FOR_AS: &str = "application/vnd.github+json";

/// **What the file is asked for as on the API route** — GitHub, *REST API
/// endpoints for release assets*, read 2026-11-09: to download, *"set the
/// Accept header of the request to application/octet-stream"*.
pub const THE_FILE_IS_ASKED_FOR_AS: &str = "application/octet-stream";

/// Which of B10b's routes a try is on.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TheRoute {
    /// The plain releases address, as today.
    ThePlainAddress,
    /// The API's list of the release's files.
    TheListOfFiles,
    /// The file, by the list's own address for it.
    TheFileThroughTheApi,
}

impl TheRoute {
    /// **What the request on this route asks for**, where it asks for anything
    /// in particular. The plain address asks for nothing new.
    pub fn what_it_asks_for(self) -> Option<&'static str> {
        match self {
            TheRoute::ThePlainAddress => None,
            TheRoute::TheListOfFiles => Some(THE_LIST_IS_ASKED_FOR_AS),
            TheRoute::TheFileThroughTheApi => Some(THE_FILE_IS_ASKED_FOR_AS),
        }
    }
}

/// **One request, through the caller's own client** — so whatever that client
/// says about who is asking, and whatever budget it draws from, goes on every
/// route.
pub trait TriesOneAddress {
    fn try_once(
        &mut self,
        address: &str,
        route: TheRoute,
        ceiling: u64,
    ) -> Result<Vec<u8>, WhyTheFetchFailed>;
}

/// **The clock B10b's pauses and deadline are kept on**, as a seam so a test
/// holds the deadline without waiting for it.
pub trait KeepsTheTime {
    /// How long since this read began.
    fn so_far(&self) -> Duration;
    /// Wait this long.
    fn pause(&mut self, for_this_long: Duration);
}

/// **The real clock**: the monotonic one, and a real wait.
pub struct TheRealClock {
    began: Instant,
}

impl TheRealClock {
    pub fn from_now() -> TheRealClock {
        TheRealClock { began: Instant::now() }
    }
}

impl KeepsTheTime for TheRealClock {
    fn so_far(&self) -> Duration {
        self.began.elapsed()
    }

    fn pause(&mut self, for_this_long: Duration) {
        std::thread::sleep(for_this_long);
    }
}

/// **B10b's API route for one plain address**: the release's list, and the
/// file's name to find in it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TheApiRoute {
    pub the_list: String,
    pub the_file: String,
}

/// **The API route for a plain releases address, or nothing where it has
/// none** — B10b, from GitHub's *REST API endpoints for releases*, read
/// 2026-11-09:
///
/// - `https://github.com/<owner>/<repo>/releases/download/<tag>/<file>` is
///   *Get a release by tag name*, `https://api.github.com/repos/<owner>/<repo>/releases/tags/<tag>`;
/// - `https://github.com/<owner>/<repo>/releases/latest/download/<file>` is
///   *Get the latest release*, `https://api.github.com/repos/<owner>/<repo>/releases/latest`.
///
/// **An address of neither form has no API route** and is only tried again.
pub fn the_api_route_for(plain: &str) -> Option<TheApiRoute> {
    let rest = plain.strip_prefix("https://github.com/")?;
    let mut parts = rest.split('/');
    let owner = parts.next().filter(|part| !part.is_empty())?;
    let repository = parts.next().filter(|part| !part.is_empty())?;
    if parts.next()? != "releases" {
        return None;
    }
    let the_release = match parts.next()? {
        "download" => {
            let tag = parts.next().filter(|part| !part.is_empty())?;
            format!("tags/{tag}")
        }
        "latest" => {
            if parts.next()? != "download" {
                return None;
            }
            "latest".to_string()
        }
        _ => return None,
    };
    let file = parts.next().filter(|part| !part.is_empty())?;
    if parts.next().is_some() || file.contains(['?', '#']) {
        return None;
    }
    Some(TheApiRoute {
        the_list: format!("https://api.github.com/repos/{owner}/{repository}/releases/{the_release}"),
        the_file: file.to_string(),
    })
}

/// **The address the list gives for the file named `file`** — its `url`, the
/// one GitHub's *REST API endpoints for release assets* page reads with *Accept:
/// application/octet-stream*.
///
/// **A list that does not read** is the service's failure, since an outage can
/// garble an answer; **a list that does not name the file** is an answer.
pub fn the_files_api_address_in(list: &[u8], file: &str) -> Result<String, WhyTheFetchFailed> {
    let Ok(read) = serde_json::from_slice::<serde_json::Value>(list) else {
        return Err(WhyTheFetchFailed::ItCouldNotBeReached {
            said: "the list of the release's files could not be read".to_string(),
        });
    };
    let named = read
        .get("assets")
        .and_then(serde_json::Value::as_array)
        .into_iter()
        .flatten()
        .find(|asset| asset.get("name").and_then(serde_json::Value::as_str) == Some(file));
    let Some(named) = named else {
        return Err(WhyTheFetchFailed::TheReleaseListsNoSuchFile { file: file.to_string() });
    };
    match named.get("url").and_then(serde_json::Value::as_str) {
        Some(address) if an_https_address(address) => Ok(address.to_string()),
        Some(address) => Err(WhyTheFetchFailed::TheSourceIsNotAnExactHttpsAddress {
            source: address.to_string(),
        }),
        None => Err(WhyTheFetchFailed::ItCouldNotBeReached {
            said: "the list of the release's files gives no address for it".to_string(),
        }),
    }
}

/// **B10b: whether a failure is one the service may get over** — no answer, a
/// timeout, HTTP 408, 429 or any 5xx, a resumed body that did not line up, and
/// on the API route also 403, its rate-limit refusal. **Everything else is an
/// answer**, and ends the read at once.
pub fn the_service_may_get_over(why: &WhyTheFetchFailed, route: TheRoute) -> bool {
    match why {
        WhyTheFetchFailed::ItCouldNotBeReached { .. } | WhyTheFetchFailed::TheResumeDidNotLineUp { .. } => true,
        WhyTheFetchFailed::TheSourceAnsweredWith { status } => {
            matches!(status, 408 | 429 | 500..=599) || (*status == 403 && route != TheRoute::ThePlainAddress)
        }
        WhyTheFetchFailed::TheSourceHasNoSuchThing
        | WhyTheFetchFailed::TheReleaseListsNoSuchFile { .. }
        | WhyTheFetchFailed::TheSourceIsNotAnExactHttpsAddress { .. }
        | WhyTheFetchFailed::ItIsLargerThanTheCeiling { .. } => false,
    }
}

/// **What every try came to, where none brought the file.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EveryTryFailed {
    /// The last address asked.
    pub the_last_address: String,
    /// What it answered.
    pub the_last_answer: WhyTheFetchFailed,
    /// **Whether the last answer was one the service may get over** — every
    /// try then came to one, since any other ends the read at once. This is
    /// what lets the user be told the download got no response rather than
    /// that something was refused.
    pub nothing_answered: bool,
    /// How many times the file was tried: each plain try, and the API route as
    /// one.
    pub how_many_tries: u32,
}

/// **B10b, whole**: the plain address, the API route, then the plain address
/// again, within the deadline. See the head of this file for the order.
pub fn read_by_either_route(
    plain: &str,
    ceiling: u64,
    tries: &mut dyn TriesOneAddress,
    clock: &mut dyn KeepsTheTime,
) -> Result<Vec<u8>, EveryTryFailed> {
    let mut how_many_tries = 1;
    let ended = |address: &str, why: WhyTheFetchFailed, route: TheRoute, how_many_tries: u32| EveryTryFailed {
        the_last_address: address.to_string(),
        nothing_answered: the_service_may_get_over(&why, route),
        the_last_answer: why,
        how_many_tries,
    };

    // ── 1. The plain address.
    let mut last = match tries.try_once(plain, TheRoute::ThePlainAddress, ceiling) {
        Ok(bytes) => return Ok(bytes),
        Err(why) if !the_service_may_get_over(&why, TheRoute::ThePlainAddress) => {
            return Err(ended(plain, why, TheRoute::ThePlainAddress, how_many_tries))
        }
        Err(why) => (plain.to_string(), why, TheRoute::ThePlainAddress),
    };
    let the_deadline = clock.so_far() + NO_TRY_BEGINS_LATER_THAN;

    // ── 2. The API route, where the address has one.
    if let Some(api) = the_api_route_for(plain) {
        how_many_tries += 1;
        let listed = tries
            .try_once(&api.the_list, TheRoute::TheListOfFiles, THE_MOST_THE_LIST_OF_FILES_MAY_BE)
            .and_then(|list| the_files_api_address_in(&list, &api.the_file));
        match listed {
            Err(why) if !the_service_may_get_over(&why, TheRoute::TheListOfFiles) => {
                return Err(ended(&api.the_list, why, TheRoute::TheListOfFiles, how_many_tries))
            }
            Err(why) => last = (api.the_list.clone(), why, TheRoute::TheListOfFiles),
            Ok(address) if clock.so_far() <= the_deadline => {
                match tries.try_once(&address, TheRoute::TheFileThroughTheApi, ceiling) {
                    Ok(bytes) => return Ok(bytes),
                    Err(why) if !the_service_may_get_over(&why, TheRoute::TheFileThroughTheApi) => {
                        return Err(ended(&address, why, TheRoute::TheFileThroughTheApi, how_many_tries))
                    }
                    Err(why) => last = (address, why, TheRoute::TheFileThroughTheApi),
                }
            }
            // Found, and the deadline passed while it was being found.
            Ok(_) => {}
        }
    }

    // ── 3. The plain address again, a few seconds apart, within the deadline.
    for pause in THE_PAUSES_BEFORE_THE_PLAIN_ADDRESS_AGAIN {
        if clock.so_far() + pause > the_deadline {
            break;
        }
        clock.pause(pause);
        how_many_tries += 1;
        match tries.try_once(plain, TheRoute::ThePlainAddress, ceiling) {
            Ok(bytes) => return Ok(bytes),
            Err(why) if !the_service_may_get_over(&why, TheRoute::ThePlainAddress) => {
                return Err(ended(plain, why, TheRoute::ThePlainAddress, how_many_tries))
            }
            Err(why) => last = (plain.to_string(), why, TheRoute::ThePlainAddress),
        }
    }
    let (address, why, route) = last;
    Err(ended(&address, why, route, how_many_tries))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::VecDeque;

    const THE_PLAIN: &str = "https://github.com/owner/repo/releases/download/v0.3.1/release";
    const THE_LIST: &str = "https://api.github.com/repos/owner/repo/releases/tags/v0.3.1";
    const THE_FILE: &str = "https://api.github.com/repos/owner/repo/releases/assets/7";

    /// **A stand-in source**: it answers each request from a queue, by route,
    /// and keeps every request and the moment it began. Each request takes
    /// `each_takes` on the stand-in clock.
    struct AStandInSource {
        plain: VecDeque<Result<Vec<u8>, WhyTheFetchFailed>>,
        list: VecDeque<Result<Vec<u8>, WhyTheFetchFailed>>,
        file: VecDeque<Result<Vec<u8>, WhyTheFetchFailed>>,
        asked: Vec<(String, TheRoute, Duration)>,
        clock: std::rc::Rc<std::cell::Cell<Duration>>,
        each_takes: Duration,
    }

    impl TriesOneAddress for AStandInSource {
        fn try_once(&mut self, address: &str, route: TheRoute, _ceiling: u64) -> Result<Vec<u8>, WhyTheFetchFailed> {
            self.asked.push((address.to_string(), route, self.clock.get()));
            self.clock.set(self.clock.get() + self.each_takes);
            let queue = match route {
                TheRoute::ThePlainAddress => &mut self.plain,
                TheRoute::TheListOfFiles => &mut self.list,
                TheRoute::TheFileThroughTheApi => &mut self.file,
            };
            queue.pop_front().expect("asked more often than the test said it would be")
        }
    }

    struct AStandInClock {
        now: std::rc::Rc<std::cell::Cell<Duration>>,
        paused: Vec<Duration>,
    }

    impl KeepsTheTime for AStandInClock {
        fn so_far(&self) -> Duration {
            self.now.get()
        }
        fn pause(&mut self, for_this_long: Duration) {
            self.paused.push(for_this_long);
            self.now.set(self.now.get() + for_this_long);
        }
    }

    fn a_source_and_a_clock(each_takes: Duration) -> (AStandInSource, AStandInClock) {
        let now = std::rc::Rc::new(std::cell::Cell::new(Duration::ZERO));
        (
            AStandInSource {
                plain: VecDeque::new(),
                list: VecDeque::new(),
                file: VecDeque::new(),
                asked: Vec::new(),
                clock: now.clone(),
                each_takes,
            },
            AStandInClock { now, paused: Vec::new() },
        )
    }

    fn status(status: u16) -> Result<Vec<u8>, WhyTheFetchFailed> {
        Err(WhyTheFetchFailed::TheSourceAnsweredWith { status })
    }

    fn the_list_naming_the_file() -> Vec<u8> {
        serde_json::json!({
            "tag_name": "v0.3.1",
            "assets": [
                { "name": "desktop-update", "url": "https://api.github.com/repos/owner/repo/releases/assets/6" },
                { "name": "release", "url": THE_FILE, "browser_download_url": THE_PLAIN },
            ]
        })
        .to_string()
        .into_bytes()
    }

    fn the_routes(source: &AStandInSource) -> Vec<(String, TheRoute)> {
        source.asked.iter().map(|(address, route, _)| (address.clone(), *route)).collect()
    }

    #[test]
    fn a_plain_address_has_its_api_route_and_nothing_else_does() {
        assert_eq!(
            the_api_route_for(THE_PLAIN),
            Some(TheApiRoute { the_list: THE_LIST.into(), the_file: "release".into() })
        );
        assert_eq!(
            the_api_route_for("https://github.com/owner/repo/releases/latest/download/desktop-update"),
            Some(TheApiRoute {
                the_list: "https://api.github.com/repos/owner/repo/releases/latest".into(),
                the_file: "desktop-update".into(),
            })
        );
        for none in [
            "https://example.com/owner/repo/releases/download/v1/release",
            "http://github.com/owner/repo/releases/download/v1/release",
            "https://github.com/owner/repo/releases/download/v1",
            "https://github.com/owner/repo/releases/download/v1/a/b",
            "https://github.com/owner/repo/releases/latest/release",
            "https://github.com/owner/repo/archive/v1/release",
            "https://github.com/owner/repo/releases/download/v1/release?x=1",
            "https://github.com//repo/releases/download/v1/release",
        ] {
            assert_eq!(the_api_route_for(none), None, "{none}");
        }
    }

    #[test]
    fn the_list_gives_the_files_own_address_and_nothing_it_does_not_name() {
        assert_eq!(the_files_api_address_in(&the_list_naming_the_file(), "release"), Ok(THE_FILE.to_string()));
        assert_eq!(
            the_files_api_address_in(&the_list_naming_the_file(), "Smoozy-macos.zip"),
            Err(WhyTheFetchFailed::TheReleaseListsNoSuchFile { file: "Smoozy-macos.zip".into() })
        );
        assert!(matches!(
            the_files_api_address_in(b"<html>Unicorn</html>", "release"),
            Err(WhyTheFetchFailed::ItCouldNotBeReached { .. })
        ));
        let plain_http = serde_json::json!({ "assets": [{ "name": "release", "url": "http://x/7" }] }).to_string();
        assert!(matches!(
            the_files_api_address_in(plain_http.as_bytes(), "release"),
            Err(WhyTheFetchFailed::TheSourceIsNotAnExactHttpsAddress { .. })
        ));
    }

    /// **A 504, then the API route answers.**
    #[test]
    fn a_504_is_followed_by_the_api_route_and_the_api_route_answers() {
        let (mut source, mut clock) = a_source_and_a_clock(Duration::from_secs(1));
        source.plain.push_back(status(504));
        source.list.push_back(Ok(the_list_naming_the_file()));
        source.file.push_back(Ok(b"the release file".to_vec()));

        let read = read_by_either_route(THE_PLAIN, 4096, &mut source, &mut clock);
        assert_eq!(read, Ok(b"the release file".to_vec()));
        assert_eq!(
            the_routes(&source),
            vec![
                (THE_PLAIN.to_string(), TheRoute::ThePlainAddress),
                (THE_LIST.to_string(), TheRoute::TheListOfFiles),
                (THE_FILE.to_string(), TheRoute::TheFileThroughTheApi),
            ]
        );
        assert!(clock.paused.is_empty(), "the API route is tried before any pause");
    }

    /// **Both routes fail, and the third plain try answers.**
    #[test]
    fn both_routes_fail_and_the_third_plain_try_answers() {
        let (mut source, mut clock) = a_source_and_a_clock(Duration::from_secs(1));
        source.plain.push_back(status(504));
        source.list.push_back(status(503));
        source.plain.push_back(Err(WhyTheFetchFailed::ItCouldNotBeReached { said: "timed out".into() }));
        source.plain.push_back(Ok(b"the release file".to_vec()));

        let read = read_by_either_route(THE_PLAIN, 4096, &mut source, &mut clock);
        assert_eq!(read, Ok(b"the release file".to_vec()));
        let routes: Vec<TheRoute> = the_routes(&source).into_iter().map(|(_, route)| route).collect();
        assert_eq!(
            routes,
            vec![
                TheRoute::ThePlainAddress,
                TheRoute::TheListOfFiles,
                TheRoute::ThePlainAddress,
                TheRoute::ThePlainAddress,
            ]
        );
        assert_eq!(clock.paused, THE_PAUSES_BEFORE_THE_PLAIN_ADDRESS_AGAIN[..2].to_vec());
    }

    /// **A 404 is an answer: nothing is tried again, on either route.**
    #[test]
    fn a_404_is_not_tried_again() {
        let (mut source, mut clock) = a_source_and_a_clock(Duration::from_secs(1));
        source.plain.push_back(Err(WhyTheFetchFailed::TheSourceHasNoSuchThing));
        let read = read_by_either_route(THE_PLAIN, 4096, &mut source, &mut clock);
        assert_eq!(
            read,
            Err(EveryTryFailed {
                the_last_address: THE_PLAIN.into(),
                the_last_answer: WhyTheFetchFailed::TheSourceHasNoSuchThing,
                nothing_answered: false,
                how_many_tries: 1,
            })
        );
        assert_eq!(source.asked.len(), 1);
        assert!(clock.paused.is_empty());

        // **And on the API route**: a 504, then the list does not name it.
        let (mut source, mut clock) = a_source_and_a_clock(Duration::from_secs(1));
        source.plain.push_back(status(504));
        source.list.push_back(Ok(serde_json::json!({ "assets": [] }).to_string().into_bytes()));
        let read = read_by_either_route(THE_PLAIN, 4096, &mut source, &mut clock).unwrap_err();
        assert_eq!(read.the_last_answer, WhyTheFetchFailed::TheReleaseListsNoSuchFile { file: "release".into() });
        assert!(!read.nothing_answered);
        assert_eq!(source.asked.len(), 2);
        assert!(clock.paused.is_empty());
    }

    /// **The deadline holds**: every try fails slowly, and no try begins more
    /// than two minutes after the first one failed.
    #[test]
    fn the_deadline_holds() {
        let each_takes = Duration::from_secs(60);
        let (mut source, mut clock) = a_source_and_a_clock(each_takes);
        for _ in 0..4 {
            source.plain.push_back(status(504));
        }
        source.list.push_back(Err(WhyTheFetchFailed::ItCouldNotBeReached { said: "timed out".into() }));

        let read = read_by_either_route(THE_PLAIN, 4096, &mut source, &mut clock).unwrap_err();
        let the_first_failed_at = each_takes;
        for (address, _, began) in &source.asked {
            assert!(
                *began <= the_first_failed_at + NO_TRY_BEGINS_LATER_THAN,
                "{address} began at {began:?}, past the deadline"
            );
        }
        // Plain at 0, the list at 60, a pause to 125 and plain again to 185;
        // the next pause would end past 180.
        assert_eq!(source.asked.len(), 3, "{:?}", source.asked);
        assert_eq!(clock.paused, vec![THE_PAUSES_BEFORE_THE_PLAIN_ADDRESS_AGAIN[0]]);
        assert!(read.nothing_answered);
        assert_eq!(read.the_last_address, THE_PLAIN);
        assert_eq!(read.the_last_answer, WhyTheFetchFailed::TheSourceAnsweredWith { status: 504 });
        assert_eq!(read.how_many_tries, 3);
    }

    /// **Every try came to no answer**: the plain address, the API route and
    /// three more plain tries, and the failure says nothing answered.
    #[test]
    fn every_try_failing_says_nothing_answered() {
        let (mut source, mut clock) = a_source_and_a_clock(Duration::from_secs(1));
        for _ in 0..4 {
            source.plain.push_back(status(502));
        }
        source.list.push_back(status(403));
        let read = read_by_either_route(THE_PLAIN, 4096, &mut source, &mut clock).unwrap_err();
        assert!(read.nothing_answered);
        assert_eq!(read.how_many_tries, 5);
        assert_eq!(clock.paused, THE_PAUSES_BEFORE_THE_PLAIN_ADDRESS_AGAIN.to_vec());
    }

    /// **Which failures are the service's**, and a 403 is one only on the API.
    #[test]
    fn which_failures_the_service_may_get_over() {
        let plain = TheRoute::ThePlainAddress;
        for yes in [408, 429, 500, 502, 503, 504, 599] {
            assert!(the_service_may_get_over(&WhyTheFetchFailed::TheSourceAnsweredWith { status: yes }, plain), "{yes}");
        }
        for no in [400, 401, 403, 451] {
            assert!(!the_service_may_get_over(&WhyTheFetchFailed::TheSourceAnsweredWith { status: no }, plain), "{no}");
        }
        assert!(the_service_may_get_over(
            &WhyTheFetchFailed::TheSourceAnsweredWith { status: 403 },
            TheRoute::TheListOfFiles
        ));
        assert!(!the_service_may_get_over(&WhyTheFetchFailed::TheSourceHasNoSuchThing, plain));
        assert!(!the_service_may_get_over(&WhyTheFetchFailed::ItIsLargerThanTheCeiling { ceiling: 1 }, plain));
    }
}
