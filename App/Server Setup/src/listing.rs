//! **B2's read of the shop-front's own listing, performed.**
//!
//! [`crate::blender`] holds every judgement B2 makes — what an entry is, what
//! the floor is, what the sort is on, what the offered list holds and what a
//! mark may claim. **This file is the one act behind them: the request that
//! fetches the listing, and the reading of what comes back.** Nothing here
//! decides anything; it turns the shop-front's answer into
//! [`crate::blender::PublishedChannel`] values and hands them next door.
//!
//! **Until 2026-09-07 there was no act here at all**, because the workspace
//! carried no client that could make an outside request. The client, the
//! transport-layer security beneath it and that layer's cryptography are now
//! chosen, measured and pinned in `App/Cargo.toml`, and the whole ruling is at
//! `Audits/Build/The one HTTPS client/Decisions record.md`. **Nothing here
//! re-opens any part of it**: no pin is loosened, no feature is added, no
//! second client appears, and no provider but the pinned one is asked for.
//!
//! **This is the fetch step 6's own milestone has been waiting on** — a machine
//! could be given Smoozy and could not be given a Blender.
//!
//! **The address and the header are B2's own and are read from
//! [`crate::blender`] rather than written again here.** A value kept in two
//! places is the fault this stage keeps meeting.
//!
//! **What is not here.** The archive fetch is B4's and runs on the machine's own
//! line, not from the desktop; the send-up of a build the user points at is B7's
//! and is a transfer. Both stay where [`crate::arriving_later`] files them.
//!
//! **Blender's own build service is read here too, since 2026-10-02** — B2's
//! revision on the Creator's notes 5 and 7 on 0.1.7: finished releases come
//! from the shop-front's `stable` channels, unfinished builds from the build
//! service's listing, newest per version and kind, and B2's narrowing runs over
//! both. *(Until that day the build service's listing was fetched by nothing,
//! and the shop-front's own unfinished channels were offered — every one of
//! them a build no release folder held.)*
//!
//! **Markers used in this file that belong to another document, each with the
//! document it belongs to.** `Server protocol and transfers` R17.

use std::sync::Arc;
use std::time::Duration;

use crate::blender::{
    PublishedChannel, Risk, Version, THE_BUILD_SERVICES_ARCHIVE_ENDING, THE_BUILD_SERVICES_LISTING,
    THE_STORES_ENDPOINT, THE_STORES_REQUIRED_HEADER,
};

// ------------------------------------------- what the processor has to carry
//
// **Nothing, since 2026-10-24.** This file used to call the workspace's one
// reading of whether the processor carried the six instruction sets the
// cryptography then needed. The cryptography is now `ring`'s, which runs on any
// 64-bit Intel or AMD processor, and the reading went with the old provider
// (the Creator's eighteenth notes, note 3).

// ------------------------------------------------------------- the two bounds

/// **The whole request's bound, and the value is decided rather than measured**
/// — exactly as B1's own time limit is, and it says so there too.
///
/// **Why there is one at all.** Nothing in B2 sets a bound, and a request with
/// none can leave the settings page waiting with no ending and no message, which
/// is a hang rather than a failure. B1 met the same question for a Blender run
/// and answered it the same way.
///
/// **Thirty seconds.** The listing was 19,847 bytes on both days it has been
/// read, and it is one small request over one connection. **What it costs if it
/// is wrong**: too short and a slow line reports that the listing could not be
/// fetched where waiting would have worked; too long and the user waits longer
/// before being told. **Neither damages anything**, and neither reaches a rented
/// machine — this request is the desktop's own and the machine is not in it.
pub const THE_LISTINGS_OWN_BOUND: Duration = Duration::from_secs(30);

/// **The ceiling on what is read back, and it is decided rather than measured
/// too.**
///
/// **Four mebibytes.** The listing answered 19,847 bytes on 2026-08-19 and again
/// on 2026-09-07 — about two hundredths of this — so the ceiling is not a limit
/// on the listing as it stands; it is what stops an answer that is not the
/// listing from being read into memory without end. **A read that reaches the
/// ceiling stops and says so** rather than reading on, which is the client's own
/// behaviour and was watched when the client was chosen.
pub const THE_MOST_OF_AN_ANSWER_THAT_IS_READ: u64 = 4 * 1024 * 1024;

// ------------------------------------------------------------- what went wrong

/// Why the offered list could not be produced.
///
/// **The service's own words ride every failure and are never mined for a
/// cause**, which is the discipline this crate already keeps for a machine's
/// words.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WhyTheListingCouldNotBeRead {
    /// The request did not reach the shop-front, or the answer did not come
    /// back. The client's own words, whole.
    TheRequestDidNotGetAnAnswer(String),
    /// The shop-front answered something other than success, with what it
    /// answered.
    TheShopFrontAnswered(u16),
    /// The answer came back and is not the shape that was read, with what is
    /// wrong.
    TheAnswerIsNotTheShapeThatWasRead(String),
}

// ------------------------------------------------------------------ the fetch

/// **B2, performed: the listing fetched from the shop-front's own endpoint.**
///
/// **The address and the header are B2's, taken from [`crate::blender`].** The
/// endpoint answered HTTP 200 on 2026-08-19, when the plan was written, and
/// again on 2026-09-07, when this was built.
///
/// **No processor is refused.** Until 2026-10-24 a processor reading came first,
/// because the cryptography then needed six instruction sets; it is now
/// `ring`'s, which runs on any 64-bit Intel or AMD processor and on Apple
/// silicon (the Creator's eighteenth notes, note 3).
///
/// **The cryptography is named at this call and never left to whatever the
/// program happens to have installed.** The client prefers a provider given here
/// over any process-wide one, so two parts of Smoozy fetching different things
/// cannot end up with different cryptography depending on which of them ran
/// first. Nothing here installs anything process-wide.
///
/// **This blocks the thread it is called on.** The client is blocking on
/// purpose — that is a third of why it was chosen, since it brings no second
/// runtime for the machine's program to carry — so a caller inside this crate's
/// asynchronous world uses [`the_list_smoozy_offers`] rather than this.
pub fn fetch_the_listing() -> Result<String, WhyTheListingCouldNotBeRead> {
    the_listing_is_fetched_from(THE_STORES_ENDPOINT, Some(THE_STORES_REQUIRED_HEADER))
}

/// **One listing fetched from `address`, on the one client both listings
/// share** — the shop-front's with its required header, the build service's
/// with none. Split out 2026-10-19 so the desk can point the same request at a
/// line that goes silent (the dead-line desk coverage); nothing about the
/// request changed.
fn the_listing_is_fetched_from(
    address: &str,
    header: Option<(&str, &str)>,
) -> Result<String, WhyTheListingCouldNotBeRead> {
    let agent = the_client_this_request_is_made_on();
    let mut asking = agent.get(address);
    if let Some((name, value)) = header {
        asking = asking.header(name, value);
    }
    let mut answer = asking.call().map_err(|said| {
        WhyTheListingCouldNotBeRead::TheRequestDidNotGetAnAnswer(said.to_string())
    })?;

    let status = answer.status().as_u16();
    if status != 200 {
        return Err(WhyTheListingCouldNotBeRead::TheShopFrontAnswered(status));
    }

    answer
        .body_mut()
        .with_config()
        .limit(THE_MOST_OF_AN_ANSWER_THAT_IS_READ)
        .read_to_string()
        .map_err(|said| WhyTheListingCouldNotBeRead::TheRequestDidNotGetAnAnswer(said.to_string()))
}

/// **B2's second listing, fetched: Blender's own build service**, on the same
/// client and the same two bounds as the shop-front's. **Read 2026-10-02**, HTTP
/// 200, 81,139 bytes — well inside [`THE_MOST_OF_AN_ANSWER_THAT_IS_READ`].
pub fn fetch_the_build_services_listing() -> Result<String, WhyTheListingCouldNotBeRead> {
    the_listing_is_fetched_from(THE_BUILD_SERVICES_LISTING, None)
}

/// **B2, the build service's half: its answer read into unfinished builds.**
///
/// **Read 2026-10-02** at [`THE_BUILD_SERVICES_LISTING`]: a top-level array, one
/// entry per file, each carrying `version`, `risk_id`, `platform`,
/// `architecture`, `file_name`, `file_mtime` (seconds) and `file_size`. Label:
/// **observed**.
///
/// **An entry is a candidate where B2 says so and nowhere else**: `platform`
/// `linux`, `architecture` `x86_64`, `file_name` ending
/// [`THE_BUILD_SERVICES_ARCHIVE_ENDING`], and `risk_id` not `stable`. Every other
/// entry — another system's, a checksum file, a finished build — is simply not
/// one, and is not called unreadable. **Of candidates with the same version and
/// the same kind, only the newest `file_mtime` is kept**, which is what disarms
/// B6's stale-entry trap. A candidate whose kind or version cannot be read is
/// named in the second list, as the shop-front's are. **The floor and the
/// narrowing are not taken here** — [`crate::blender::the_offered_list`] takes
/// them over both listings at once.
pub fn read_the_build_services_listing(answer: &str) -> Result<TheListingAsRead, WhyTheListingCouldNotBeRead> {
    let entries: Vec<serde_json::Value> = serde_json::from_str(answer).map_err(|said| {
        WhyTheListingCouldNotBeRead::TheAnswerIsNotTheShapeThatWasRead(said.to_string())
    })?;
    let text = |entry: &serde_json::Value, field: &str| entry[field].as_str().unwrap_or_default().to_string();
    let mut newest: Vec<(PublishedChannel, u64)> = Vec::new();
    let mut entries_that_could_not_be_read = Vec::new();
    for entry in &entries {
        let file_name = text(entry, "file_name");
        let risk_id = text(entry, "risk_id");
        if text(entry, "platform") != "linux"
            || text(entry, "architecture") != "x86_64"
            || !file_name.ends_with(THE_BUILD_SERVICES_ARCHIVE_ENDING)
            || risk_id == "stable"
        {
            continue;
        }
        let named = format!("{file_name} at {}", text(entry, "version"));
        let risk = match risk_id.as_str() {
            "alpha" => Risk::Alpha,
            "beta" => Risk::Beta,
            "candidate" => Risk::Candidate,
            _ => {
                entries_that_could_not_be_read.push(format!("{named} — the kind is not alpha, beta or candidate"));
                continue;
            }
        };
        let Some(version) = Version::parse(&text(entry, "version")) else {
            entries_that_could_not_be_read.push(format!("{named} — the version is not version components"));
            continue;
        };
        let moment = entry["file_mtime"].as_u64().unwrap_or(0);
        let channel = PublishedChannel {
            track: String::new(),
            risk,
            version,
            released_at: moment.to_string(),
            size_bytes: entry["file_size"].as_u64().unwrap_or(0),
            the_build_services_file: Some(file_name),
        };
        match newest
            .iter_mut()
            .find(|(held, _)| held.version == channel.version && held.risk == channel.risk)
        {
            Some(held) if held.1 < moment => *held = (channel, moment),
            Some(_) => {}
            None => newest.push((channel, moment)),
        }
    }
    Ok(TheListingAsRead {
        channels: newest.into_iter().map(|(channel, _)| channel).collect(),
        entries_that_could_not_be_read,
    })
}

/// **How many times the shop-front may hand this request on to another
/// address.**
///
/// **The client's own default is ten and HTTPS-only is off** — read from its
/// shipped source at `ureq` 3.4.1 `src/config.rs`, 2026-09-12 — so an answer
/// redirecting this request to a plain address would be followed in clear. The
/// release fetcher built the same day for the same ruling sets both and this
/// sets them the same way; **the difference between the two clients was one
/// nobody chose.**
///
/// **Two, because the listing is one small read of one published endpoint.**
/// Nothing of the user's rides the request, and the build itself is fetched
/// from Blender's own downloads against a published checksum, so what this
/// repairs is the difference rather than a harm.
pub const HOW_MANY_TIMES_THE_SHOP_FRONT_MAY_HAND_THIS_ON: u32 = 2;

/// **The client this request is made on.**
///
/// **It is a function rather than three lines inside the fetch** so that what it
/// is configured with can be read back and held to, which is how the missing
/// HTTPS-only bound was found in the first place.
fn the_client_this_request_is_made_on() -> ureq::Agent {
    ureq::Agent::config_builder()
        .timeout_global(Some(THE_LISTINGS_OWN_BOUND))
        // **No hop may leave HTTPS, a redirect included**, and the hops are
        // bounded. Neither is the client's default.
        .https_only(true)
        .max_redirects(HOW_MANY_TIMES_THE_SHOP_FRONT_MAY_HAND_THIS_ON)
        // **The status is read here rather than raised as a failure**, because
        // B5 already says what a shop-front answer that is not success means and
        // the value has to reach that judgement whole.
        .http_status_as_error(false)
        .tls_config(
            ureq::tls::TlsConfig::builder()
                .provider(ureq::tls::TlsProvider::Rustls)
                .unversioned_rustls_crypto_provider(Arc::new(rustls::crypto::ring::default_provider()))
                .build(),
        )
        .build()
        .new_agent()
}

// ----------------------------------------------------------------- the reading

/// What one read of the listing came to.
///
/// **Two lists, because an entry that cannot be read is said rather than
/// dropped quietly.** The whole of this stage's recurring fault is a fact kept
/// where nobody can see it, and an entry silently missing from an offered list
/// is exactly that.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TheListingAsRead {
    /// Every entry that could be read, in the order the shop-front gave them.
    /// **Not sorted and not floored here** — that is
    /// [`crate::blender::the_offered_list`]'s, which is where B2's rules live.
    pub channels: Vec<PublishedChannel>,
    /// Every entry that could not be read, as the shop-front's own track, risk
    /// and version text, with what is wrong with it. **Nothing acts on this**;
    /// it exists so that a shortfall is visible instead of invisible.
    pub entries_that_could_not_be_read: Vec<String>,
}

/// **B2, performed: the shop-front's answer read into entries.**
///
/// **What an entry carries is B2's own list** — the track, the risk, the
/// version, the release moment and the byte size — and each is read from the
/// field of the answer that holds it. **Read 2026-09-07** at
/// [`THE_STORES_ENDPOINT`], HTTP 200, 19,847 bytes, 40 entries: the entries sit
/// under `channel-map`; each carries `version` and a `download` holding `size`,
/// and a `channel` holding `track`, `risk` and `released-at`. Label: **observed**.
///
/// **Two things the answer also carries and nothing here reads, said rather than
/// passed over.** Each entry names an architecture, and every entry on both days
/// read named the same one; nothing Smoozy fetches comes from the shop-front, so
/// the field decides nothing, and reading it would be a rule the plan does not
/// state. And the answer carries a top-level default-track field, which was
/// empty on the day read. B3 chooses the highest stable version across tracks,
/// so that field does not decide the quick path's version.
///
/// **An entry whose version is not version components is not offered, and this
/// is a judgement rather than a reading, so it is written out.** B2's sort is on
/// components and its floor is a comparison of components, and an entry with no
/// components can be placed by neither. **Every such entry on the day read is
/// far below the floor** — the shop-front's oldest tracks carry versions ending
/// in a letter, and one carries two numbers rather than three, which does read.
/// **What it would cost if that changed**: a supported version published with a
/// letter after it would be absent from the offered list. **What holds**: it is
/// named in the second list rather than vanishing, and B5 already establishes
/// that the offered list and what can be fetched are not the same set. **Only
/// there: one below the floor is not named** ([`a_lettered_version_below_the_floor`]),
/// because the count the user reads has to mean something.
///
/// **An entry whose risk is not one of B2's four is not offered either**, for
/// the same reason written the other way round: B2 requires the unstable kinds to
/// be *"included and marked as what they are"*, and an entry whose kind is not
/// known cannot be marked as what it is. Offering it unmarked would be the one
/// thing the clause forbids.
pub fn read_the_listing(answer: &str) -> Result<TheListingAsRead, WhyTheListingCouldNotBeRead> {
    let parsed: TheShopFrontsAnswer = serde_json::from_str(answer).map_err(|said| {
        WhyTheListingCouldNotBeRead::TheAnswerIsNotTheShapeThatWasRead(said.to_string())
    })?;

    let mut channels = Vec::new();
    let mut entries_that_could_not_be_read = Vec::new();
    for value in parsed.channel_map {
        // **The entries are read one at a time, and this is why.** Until a code
        // round said otherwise the whole entry list was one shape, so **one
        // entry the shop-front published in a shape Smoozy does not read failed
        // the whole document** — a missing download block, a size that is not a
        // number, a missing release moment, a renamed field — and every Blender
        // version vanished from the user's list, on every computer, until
        // Smoozy was changed. **The file's own rule is the opposite**: an entry
        // that cannot be read is said rather than dropped quietly.
        //
        // **An answer carrying no entry list at all is still the whole-answer
        // refusal above**, and that is not softened: an answer that is not the
        // listing is not a listing with unreadable entries in it.
        let entry: TheShopFrontsEntry = match serde_json::from_value(value.clone()) {
            Ok(entry) => entry,
            Err(said) => {
                entries_that_could_not_be_read
                    .push(format!("{} — {said}", the_best_name_for(&value)));
                continue;
            }
        };
        let named = format!(
            "{}/{} at {}",
            entry.channel.track, entry.channel.risk, entry.version
        );
        let Some(risk) = the_risk(&entry.channel.risk) else {
            entries_that_could_not_be_read
                .push(format!("{named} — the risk is not one of the four"));
            continue;
        };
        let Some(version) = Version::parse(&entry.version) else {
            // **B2's line of 2026-11-02: below the floor it could never have
            // been offered, so it is not a shortfall.**
            if !a_lettered_version_below_the_floor(&entry.version) {
                entries_that_could_not_be_read
                    .push(format!("{named} — the version is not version components"));
            }
            continue;
        };
        channels.push(PublishedChannel {
            track: entry.channel.track,
            risk,
            version,
            released_at: entry.channel.released_at,
            size_bytes: entry.download.size,
            the_build_services_file: None,
        });
    }
    Ok(TheListingAsRead {
        channels,
        entries_that_could_not_be_read,
    })
}

/// **`Server Setup` B2's line of 2026-11-02: a version that is numbers then
/// letters, whose numbers are below the whole-app floor.** The first 5.3 live
/// pass read *3 entries could not be read* on both machines. Read that day from
/// the Creator's Mac, the three were `2.82a`, `2.81a` and `2.79b`, of the
/// shop-front's 41 entries. **Observed.** They could never have been offered, as
/// any entry below the floor never is, so they are not counted among what could
/// not be read. **A lettered version at or above the floor is still counted**,
/// because that one is a real shortfall, and so is anything with no numbers.
fn a_lettered_version_below_the_floor(version: &str) -> bool {
    Version::parse(version.trim_end_matches(|letter: char| letter.is_ascii_alphabetic()))
        .is_some_and(|numbers| numbers < crate::blender::THE_FLOOR)
}

/// **B2's four risk marks, read from the shop-front's own word for each.**
///
/// Observed on both days read: `stable`, `candidate`, `beta` and `edge`.
fn the_risk(word: &str) -> Option<Risk> {
    match word {
        "stable" => Some(Risk::Stable),
        "candidate" => Some(Risk::Candidate),
        "beta" => Some(Risk::Beta),
        "edge" => Some(Risk::Edge),
        _ => None,
    }
}

/// **What an entry that will not read is called in the second list.**
///
/// **Best effort and never a failure of its own.** An entry that will not read
/// is exactly an entry whose fields cannot be relied on, so each of the three
/// is taken where it is there and left out where it is not — and an entry that
/// gives none of them is named by the shop-front's own text for it, cut to a
/// length a reader can take in.
fn the_best_name_for(entry: &serde_json::Value) -> String {
    let text = |at: &serde_json::Value| at.as_str().unwrap_or_default().to_string();
    let channel = &entry["channel"];
    let track = text(&channel["track"]);
    let risk = text(&channel["risk"]);
    let version = text(&entry["version"]);
    if track.is_empty() && risk.is_empty() && version.is_empty() {
        let whole = entry.to_string();
        return whole.chars().take(120).collect();
    }
    format!("{track}/{risk} at {version}")
}

/// The shop-front's answer, in its own field names.
///
/// **Fields the answer carries and nothing here reads are ignored rather than
/// refused**, so the shop-front adding one does not stop Smoozy offering a list.
///
/// **The entries are held as values here and read one at a time below**, which
/// is what keeps one unreadable entry from taking the whole list with it.
#[derive(serde::Deserialize)]
struct TheShopFrontsAnswer {
    #[serde(rename = "channel-map")]
    channel_map: Vec<serde_json::Value>,
}

#[derive(serde::Deserialize)]
struct TheShopFrontsEntry {
    channel: TheShopFrontsChannel,
    version: String,
    download: TheShopFrontsDownload,
}

#[derive(serde::Deserialize)]
struct TheShopFrontsChannel {
    track: String,
    risk: String,
    #[serde(rename = "released-at")]
    released_at: String,
}

#[derive(serde::Deserialize)]
struct TheShopFrontsDownload {
    size: u64,
}

// ------------------------------------------------------------- the whole act

/// **B2 whole: the listing fetched, read, floored, narrowed and sorted — the
/// list Smoozy offers.**
///
/// **The floor, the narrowing and the sort are
/// [`crate::blender::the_offered_list`]'s and are not retaken here**, which is what keeps every rule B2 states in one place.
///
/// **The fetch is blocking and this is not**, so the blocking part runs on a
/// thread of its own and the caller's runtime keeps running. The listing is a
/// small request on the user's own line, and `Server protocol and transfers` R17
/// records that line at well under a megabyte a second on the reading this
/// project has of it — so it is not instant and a runtime held still for it
/// would be the whole application held still.
pub async fn the_list_smoozy_offers() -> Result<TheListingAsRead, WhyTheListingCouldNotBeRead> {
    let (shop_front, build_service) = tokio::task::spawn_blocking(|| {
        (
            once_more_if_it_fails(|| fetch_the_listing().and_then(|it| read_the_listing(&it))),
            fetch_the_build_services_listing().and_then(|it| read_the_build_services_listing(&it)),
        )
    })
    .await
    .map_err(|said| WhyTheListingCouldNotBeRead::TheRequestDidNotGetAnAnswer(said.to_string()))?;
    Ok(the_two_listings_offered(shop_front?, build_service))
}

/// **B2's shop-front is asked once more where its first answer is no use**, at
/// once. The second Sesterce pass's first *Install Blender…* box read that the list
/// could not be read, with the line answering and a second opening reading it. What
/// the first ask met was not caught, so this is a retry and not a cause.
fn once_more_if_it_fails<T, E>(ask: impl Fn() -> Result<T, E>) -> Result<T, E> {
    ask().or_else(|_| ask())
}

/// **B2 over both listings**: the shop-front's `stable` channels and the build
/// service's unfinished builds, then [`crate::blender::the_offered_list`]'s
/// floor, narrowing and sort over the two together. **The shop-front's own
/// unfinished channels are not offered** — none ever had a file Smoozy could
/// fetch (B5). **A build service that could not be read costs its builds and
/// nothing else**: the finished releases are still offered, and the shortfall
/// is said in the second list rather than hidden.
pub fn the_two_listings_offered(
    shop_front: TheListingAsRead,
    build_service: Result<TheListingAsRead, WhyTheListingCouldNotBeRead>,
) -> TheListingAsRead {
    let mut channels: Vec<PublishedChannel> = shop_front
        .channels
        .into_iter()
        .filter(|channel| channel.risk == Risk::Stable)
        .collect();
    let mut entries_that_could_not_be_read = shop_front.entries_that_could_not_be_read;
    match build_service {
        Ok(read) => {
            channels.extend(read.channels);
            entries_that_could_not_be_read.extend(read.entries_that_could_not_be_read);
        }
        Err(why) => entries_that_could_not_be_read.push(format!("Blender's build service — {why:?}")),
    }
    TheListingAsRead {
        channels: crate::blender::the_offered_list(&channels),
        entries_that_could_not_be_read,
    }
}

#[cfg(test)]
mod tests {
    /// **B2: a first answer that is no use is asked once more, and no further.**
    #[test]
    fn the_shop_front_is_asked_once_more_and_no_further() {
        let asked = std::cell::Cell::new(0);
        let answer = |at: u32| {
            asked.set(asked.get() + 1);
            if asked.get() >= at { Ok(asked.get()) } else { Err("no answer") }
        };
        assert_eq!(once_more_if_it_fails(|| answer(1)), Ok(1));
        asked.set(0);
        assert_eq!(once_more_if_it_fails(|| answer(2)), Ok(2));
        asked.set(0);
        assert_eq!(once_more_if_it_fails(|| answer(3)), Err("no answer"));
        assert_eq!(asked.get(), 2);
    }

    use super::*;

    /// **The shop-front's own answer, cut down to the entries that make each
    /// point and otherwise left exactly as it came back** — read 2026-09-07 at
    /// the endpoint B2 names, HTTP 200. **The field names, the nesting and every
    /// value below are the service's**, not this project's.
    const WHAT_THE_SHOP_FRONT_ANSWERED: &str = r#"{
      "channel-map": [
        {"channel": {"architecture": "amd64", "name": "stable", "released-at": "2026-08-25T13:56:59.398880+00:00", "risk": "stable", "track": "latest"},
         "download": {"size": 425840640}, "revision": 7740, "type": "app", "version": "5.2.1"},
        {"channel": {"architecture": "amd64", "name": "edge", "released-at": "2026-09-07T05:51:58.367116+00:00", "risk": "edge", "track": "latest"},
         "download": {"size": 433758208}, "revision": 7754, "type": "app", "version": "5.3.0"},
        {"channel": {"architecture": "amd64", "name": "4.5lts/stable", "released-at": "2026-08-25T10:07:02.077795+00:00", "risk": "stable", "track": "4.5lts"},
         "download": {"size": 420859904}, "revision": 7738, "type": "app", "version": "4.5.13"},
        {"channel": {"architecture": "amd64", "name": "4.5lts/beta", "released-at": "2025-07-08T05:48:37.078158+00:00", "risk": "beta", "track": "4.5lts"},
         "download": {"size": 420970496}, "revision": 5525, "type": "app", "version": "4.5.0"},
        {"channel": {"architecture": "amd64", "name": "4.2lts/stable", "released-at": "2026-07-21T08:17:00.528113+00:00", "risk": "stable", "track": "4.2lts"},
         "download": {"size": 398802944}, "revision": 7643, "type": "app", "version": "4.2.23"},
        {"channel": {"architecture": "amd64", "name": "4.2/stable", "released-at": "2024-01-01T00:00:00.000000+00:00", "risk": "stable", "track": "4.2"},
         "download": {"size": 398000000}, "revision": 1, "type": "app", "version": "4.2.9"},
        {"channel": {"architecture": "amd64", "name": "2.82/stable", "released-at": "2020-06-25T13:05:02.341193+00:00", "risk": "stable", "track": "2.82"},
         "download": {"size": 170643456}, "revision": 1268, "type": "app", "version": "2.82a"}
      ],
      "default-track": null,
      "name": "blender",
      "snap-id": "ZWyHJJiPBBfMKpaaKKrpSgAcupWkHWPz"
    }"#;

    fn the_answer_read() -> TheListingAsRead {
        read_the_listing(WHAT_THE_SHOP_FRONT_ANSWERED).expect("the shop-front's own answer reads")
    }

    /// **B2: the real answer is read, field for field, into entries.**
    #[test]
    fn the_shop_fronts_own_answer_reads_into_entries() {
        let read = the_answer_read();
        let latest_stable = read
            .channels
            .iter()
            .find(|entry| entry.track == "latest" && entry.risk == Risk::Stable)
            .expect("the default track's stable channel is in the answer");
        assert_eq!(
            latest_stable.version,
            Version {
                major: 5,
                minor: 2,
                patch: 1
            }
        );
        assert_eq!(latest_stable.size_bytes, 425_840_640);
        assert_eq!(
            latest_stable.released_at,
            "2026-08-25T13:56:59.398880+00:00"
        );
    }

    /// **The same shape as the answer above, with the trap moved above the
    /// floor**, so that Fact 54's case can be driven through the offered list
    /// itself rather than through a sort written out in the test.
    ///
    /// **This one is made up and says so.** The shop-front's own answer carries
    /// the 4.2.23-against-4.2.9 pair below the whole-app floor, so the offered
    /// list never sees it; the field names, the nesting and the shape here are
    /// the service's, and the two versions are this test's. **Both are finished
    /// releases on two tracks**: until 2026-09-28 the lower was a beta, which
    /// B2's narrowing now leaves out beside a finished 4.5.23 of its series.
    const THE_SAME_SHAPE_WITH_THE_TRAP_ABOVE_THE_FLOOR: &str = r#"{
      "channel-map": [
        {"channel": {"architecture": "amd64", "name": "4.5/stable", "released-at": "2026-01-01T00:00:00.000000+00:00", "risk": "stable", "track": "4.5"},
         "download": {"size": 420000000}, "version": "4.5.9"},
        {"channel": {"architecture": "amd64", "name": "4.5lts/stable", "released-at": "2026-08-25T10:07:02.077795+00:00", "risk": "stable", "track": "4.5lts"},
         "download": {"size": 420859904}, "version": "4.5.23"}
      ],
      "name": "blender"
    }"#;

    /// **B2, and it is Fact 54's own case by name: a plain alphabetical sort
    /// picks 4.2.9 as newest where 4.2.23 exists.**
    ///
    /// **Three things are held here.** The version reading orders the real pair
    /// the fact names the right way round; a plain alphabetical sort of the same
    /// two says the opposite, which is why the rule exists at all; and **the
    /// offered list itself keeps only the higher patch** when the same trap
    /// sits above the floor, as the twentieth notes' note 1 requires.
    #[test]
    fn the_sort_puts_4_2_23_above_4_2_9_where_a_string_sort_would_not() {
        let older = Version::parse("4.2.9").expect("a version");
        let newer = Version::parse("4.2.23").expect("a version");
        assert!(newer > older, "4.2.23 is newer than 4.2.9");

        // A plain alphabetical sort of the same two says the opposite.
        let mut alphabetically = ["4.2.23".to_string(), "4.2.9".to_string()];
        alphabetically.sort();
        alphabetically.reverse();
        assert_eq!(alphabetically[0], "4.2.9");

        // And the offered list, over the same trap moved above the floor,
        // keeps only the higher patch. The answer gives the older one first.
        let read = read_the_listing(THE_SAME_SHAPE_WITH_THE_TRAP_ABOVE_THE_FLOOR)
            .expect("the answer reads");
        assert_eq!(read.channels[0].version, Version::parse("4.5.9").unwrap());
        let offered = crate::blender::the_offered_list(&read.channels);
        let order: Vec<Version> = offered.iter().map(|entry| entry.version).collect();
        assert_eq!(
            order,
            vec![Version::parse("4.5.23").unwrap()],
            "the newest offered patch is chosen on version components, never on string"
        );
    }

    /// **B2: the offered list holds the channels at or above the whole-app floor
    /// of 4.5, and nothing below it.**
    #[test]
    fn the_floor_leaves_out_everything_below_4_5() {
        let read = the_answer_read();
        let offered = crate::blender::the_offered_list(&read.channels);
        assert!(
            offered
                .iter()
                .all(|entry| entry.version >= crate::blender::THE_FLOOR),
            "something below the floor is offered"
        );
        for below in ["4.2.23", "4.2.9"] {
            assert!(
                !offered.iter().any(|entry| format!(
                    "{}.{}.{}",
                    entry.version.major, entry.version.minor, entry.version.patch
                ) == below),
                "{below} is below the floor and is offered"
            );
        }
        assert!(
            read.channels.iter().any(|entry| entry.version
                == Version {
                    major: 4,
                    minor: 2,
                    patch: 23
                }),
            "the floor has to be doing the work rather than the reading"
        );
    }

    /// **B2: an unstable channel is offered and is marked as what it is** —
    /// unless B2's narrowing leaves it out. **The real answer of 2026-09-07
    /// holds one of each**: the edge channel at 5.3.0 has no finished 5.3 beside
    /// it and is offered; the beta at 4.5.0 sits beside a finished 4.5.13 and,
    /// from 2026-09-28, is not.
    #[test]
    fn an_unstable_channel_is_offered_and_carries_its_mark() {
        let read = the_answer_read();
        let offered = crate::blender::the_offered_list(&read.channels);
        let edge = offered
            .iter()
            .find(|entry| entry.risk == Risk::Edge)
            .expect("the edge channel is offered");
        assert!(edge.risk.is_unstable());
        assert_eq!(
            edge.version,
            Version {
                major: 5,
                minor: 3,
                patch: 0
            }
        );
        assert!(
            read.channels.iter().any(|entry| entry.risk == Risk::Beta),
            "the answer carries the beta, so the narrowing is doing the work"
        );
        assert!(
            !offered.iter().any(|entry| entry.risk == Risk::Beta),
            "the beta at 4.5.0 sits beside a finished 4.5.13 and is not offered"
        );
        assert!(
            offered.iter().any(|entry| entry.risk == Risk::Stable),
            "the finished releases are offered beside them"
        );
    }

    /// **The list step 10c's ninth pass drew on Hyperstack, 2026-09-27, in the
    /// shop-front's own shape.**
    ///
    /// **The versions and kinds are the ones the install box drew that day**
    /// (`Audits/Build/Step 10c/Piece 12, the ninth pass — Hyperstack, the
    /// rental's details.md`, 20:14:29): above the rule *5.2.2 Latest stable ·
    /// Long-term support*, *5.1.2 Release*, *5.0.1 Release*, *4.5.14 Long-term
    /// support*; below it *5.3.0 Unstable*, *5.2.2 Release candidate*, *4.5.14
    /// Release candidate*, *4.5.0 Beta*. **The tracks are the ones the box's own
    /// line names are read from** (`latest`, `5.2lts`, `5.1`, `5.0`, `4.5lts`),
    /// and the sizes and moments are this test's. The 2.82a entry stands for the
    /// entries that did not read that day.
    const THE_NINTH_PASSS_LIST: &str = r#"{
      "channel-map": [
        {"channel": {"architecture": "amd64", "name": "latest/edge", "released-at": "2026-09-27T05:00:00.000000+00:00", "risk": "edge", "track": "latest"},
         "download": {"size": 433758208}, "version": "5.3.0"},
        {"channel": {"architecture": "amd64", "name": "latest/stable", "released-at": "2026-09-20T00:00:00.000000+00:00", "risk": "stable", "track": "latest"},
         "download": {"size": 425840640}, "version": "5.2.2"},
        {"channel": {"architecture": "amd64", "name": "latest/candidate", "released-at": "2026-09-20T00:00:00.000000+00:00", "risk": "candidate", "track": "latest"},
         "download": {"size": 425840640}, "version": "5.2.2"},
        {"channel": {"architecture": "amd64", "name": "5.2lts/stable", "released-at": "2026-09-20T00:00:00.000000+00:00", "risk": "stable", "track": "5.2lts"},
         "download": {"size": 425840640}, "version": "5.2.2"},
        {"channel": {"architecture": "amd64", "name": "5.2lts/candidate", "released-at": "2026-09-20T00:00:00.000000+00:00", "risk": "candidate", "track": "5.2lts"},
         "download": {"size": 425840640}, "version": "5.2.2"},
        {"channel": {"architecture": "amd64", "name": "5.1/stable", "released-at": "2026-06-01T00:00:00.000000+00:00", "risk": "stable", "track": "5.1"},
         "download": {"size": 420000000}, "version": "5.1.2"},
        {"channel": {"architecture": "amd64", "name": "5.0/stable", "released-at": "2026-03-01T00:00:00.000000+00:00", "risk": "stable", "track": "5.0"},
         "download": {"size": 415000000}, "version": "5.0.1"},
        {"channel": {"architecture": "amd64", "name": "4.5lts/stable", "released-at": "2026-09-01T00:00:00.000000+00:00", "risk": "stable", "track": "4.5lts"},
         "download": {"size": 420859904}, "version": "4.5.14"},
        {"channel": {"architecture": "amd64", "name": "4.5lts/candidate", "released-at": "2026-09-01T00:00:00.000000+00:00", "risk": "candidate", "track": "4.5lts"},
         "download": {"size": 420859904}, "version": "4.5.14"},
        {"channel": {"architecture": "amd64", "name": "4.5lts/beta", "released-at": "2025-07-08T05:48:37.078158+00:00", "risk": "beta", "track": "4.5lts"},
         "download": {"size": 420970496}, "version": "4.5.0"},
        {"channel": {"architecture": "amd64", "name": "2.82/stable", "released-at": "2020-06-25T13:05:02.341193+00:00", "risk": "stable", "track": "2.82"},
         "download": {"size": 170643456}, "version": "2.82a"}
      ],
      "name": "blender"
    }"#;

    fn offered_as_seen(offered: &[PublishedChannel]) -> Vec<String> {
        offered
            .iter()
            .map(|entry| {
                format!(
                    "{}/{:?} {}.{}.{}",
                    entry.track, entry.risk, entry.version.major, entry.version.minor, entry.version.patch
                )
            })
            .collect()
    }

    /// **B2's narrowing, over the ninth pass's own list: an unfinished build is
    /// not offered where a `stable` channel in the same answer carries the same
    /// series (major and minor) at the same or a newer version.** Narrowed at
    /// the spec, Creator-stated 2026-09-27 and widened 2026-09-28.
    ///
    /// **So the two release candidates at 5.2.2 go beside a finished 5.2.2, the
    /// release candidate at 4.5.14 goes beside a finished 4.5.14, and the beta at
    /// 4.5.0 goes beside a finished 4.5.14** — an older unfinished build of a
    /// released series. **5.3.0 stays**: no `stable` channel carries 5.3. The
    /// finished releases are all offered, untouched.
    #[test]
    fn the_ninth_passs_list_loses_its_unfinished_builds_of_released_series() {
        let read = read_the_listing(THE_NINTH_PASSS_LIST).expect("the answer reads");
        let offered = crate::blender::the_offered_list(&read.channels);
        assert_eq!(
            offered_as_seen(&offered),
            vec![
                "latest/Edge 5.3.0",
                "latest/Stable 5.2.2",
                "5.2lts/Stable 5.2.2",
                "5.1/Stable 5.1.2",
                "5.0/Stable 5.0.1",
                "4.5lts/Stable 4.5.14",
            ],
            "the ninth pass's list, narrowed as B2 says"
        );
    }

    /// **Blender's own build service's answer, read 2026-10-02 at
    /// [`THE_BUILD_SERVICES_LISTING`], HTTP 200, 131 entries — cut down to the
    /// entries that make each point and otherwise left exactly as it came
    /// back.** Every field name, nesting and value of the first six entries is
    /// the service's. **The last two are this test's and say so**: a 5.3.0 alpha
    /// file two days older than the current one (the newest per version and
    /// kind is kept — B6's stale-entry trap), and an alpha of the released 5.2
    /// series from 2025, which B2's narrowing leaves out beside a finished 5.2.2.
    const WHAT_THE_BUILD_SERVICE_ANSWERED: &str = r#"[
      {"url": "https://cdn.builder.blender.org/download/daily/blender-5.3.0-alpha+main.64d181010e0d-linux.x86_64-release.tar.xz", "app": "Blender", "version": "5.3.0", "risk_id": "alpha", "branch": "main", "patch": null, "hash": "64d181010e0d", "platform": "linux", "architecture": "x86_64", "bitness": 64, "file_mtime": 1790908618, "file_name": "blender-5.3.0-alpha+main.64d181010e0d-linux.x86_64-release.tar.xz", "file_size": 399290348, "file_extension": "xz", "release_cycle": "alpha"},
      {"url": "https://cdn.builder.blender.org/download/daily/blender-5.3.0-alpha+main.64d181010e0d-linux.x86_64-release.tar.xz.sha256", "app": "Blender", "version": "5.3.0", "risk_id": "alpha", "branch": "main", "patch": null, "hash": "64d181010e0d", "platform": "linux", "architecture": "x86_64", "bitness": 64, "file_mtime": 1790908618, "file_name": "blender-5.3.0-alpha+main.64d181010e0d-linux.x86_64-release.tar.xz.sha256", "file_size": 64, "file_extension": "sha256", "release_cycle": "alpha"},
      {"url": "https://cdn.builder.blender.org/download/daily/blender-5.2.2-stable+v52.d13f752e3b9c-linux.x86_64-release.tar.xz", "app": "Blender", "version": "5.2.2", "risk_id": "stable", "branch": "v52", "patch": null, "hash": "d13f752e3b9c", "platform": "linux", "architecture": "x86_64", "bitness": 64, "file_mtime": 1790905333, "file_name": "blender-5.2.2-stable+v52.d13f752e3b9c-linux.x86_64-release.tar.xz", "file_size": 376906160, "file_extension": "xz", "release_cycle": "stable"},
      {"url": "https://cdn.builder.blender.org/download/daily/blender-4.5.14-stable+v45.62c1db4208e8-linux.x86_64-release.tar.xz", "app": "Blender", "version": "4.5.14", "risk_id": "stable", "branch": "v45", "patch": null, "hash": "62c1db4208e8", "platform": "linux", "architecture": "x86_64", "bitness": 64, "file_mtime": 1790905492, "file_name": "blender-4.5.14-stable+v45.62c1db4208e8-linux.x86_64-release.tar.xz", "file_size": 372657956, "file_extension": "xz", "release_cycle": "stable"},
      {"url": "https://cdn.builder.blender.org/download/daily/blender-5.3.0-alpha+main.64d181010e0d-windows.amd64-release.zip", "app": "Blender", "version": "5.3.0", "risk_id": "alpha", "branch": "main", "patch": null, "hash": "64d181010e0d", "platform": "windows", "architecture": "amd64", "bitness": 64, "file_mtime": 1790908618, "file_name": "blender-5.3.0-alpha+main.64d181010e0d-windows.amd64-release.zip", "file_size": 420000000, "file_extension": "zip", "release_cycle": "alpha"},
      {"url": "https://cdn.builder.blender.org/download/daily/blender-5.3.0-alpha+main.64d181010e0d-darwin.arm64-release.dmg", "app": "Blender", "version": "5.3.0", "risk_id": "alpha", "branch": "main", "patch": null, "hash": "64d181010e0d", "platform": "darwin", "architecture": "arm64", "bitness": 64, "file_mtime": 1790908618, "file_name": "blender-5.3.0-alpha+main.64d181010e0d-darwin.arm64-release.dmg", "file_size": 410000000, "file_extension": "dmg", "release_cycle": "alpha"},
      {"url": "https://cdn.builder.blender.org/download/daily/blender-5.3.0-alpha+main.0000000000aa-linux.x86_64-release.tar.xz", "app": "Blender", "version": "5.3.0", "risk_id": "alpha", "branch": "main", "patch": null, "hash": "0000000000aa", "platform": "linux", "architecture": "x86_64", "bitness": 64, "file_mtime": 1790735818, "file_name": "blender-5.3.0-alpha+main.0000000000aa-linux.x86_64-release.tar.xz", "file_size": 399000000, "file_extension": "xz", "release_cycle": "alpha"},
      {"url": "https://cdn.builder.blender.org/download/daily/blender-5.2.0-alpha+main.0000000000bb-linux.x86_64-release.tar.xz", "app": "Blender", "version": "5.2.0", "risk_id": "alpha", "branch": "main", "patch": null, "hash": "0000000000bb", "platform": "linux", "architecture": "x86_64", "bitness": 64, "file_mtime": 1735689600, "file_name": "blender-5.2.0-alpha+main.0000000000bb-linux.x86_64-release.tar.xz", "file_size": 390000000, "file_extension": "xz", "release_cycle": "alpha"}
    ]"#;

    /// **B2's revision of 2026-10-02, over the ninth pass's shop-front list and
    /// the build service's own answer** — the Creator's notes 5 and 7 on 0.1.7.
    /// **One 5.3.0 Alpha is offered, and it is the newest file of that version
    /// and kind**, carrying its bare file name for B4; the stale 5.2.0 alpha of
    /// a released series is narrowed out beside the finished 5.2.2; the build
    /// service's finished builds, its checksum files and other systems' files
    /// are not candidates; and **the shop-front's own unfinished channels are
    /// not offered at all**, the edge 5.3.0 among them.
    #[test]
    fn the_build_service_lists_the_unfinished_builds_and_the_shop_front_the_finished_ones() {
        let build_service = read_the_build_services_listing(WHAT_THE_BUILD_SERVICE_ANSWERED)
            .expect("the build service's own answer reads");
        assert!(build_service.entries_that_could_not_be_read.is_empty(), "{build_service:?}");
        assert_eq!(build_service.channels.len(), 2, "one 5.3.0 alpha and one 5.2.0 alpha: {build_service:?}");
        let shop_front = read_the_listing(THE_NINTH_PASSS_LIST).expect("the answer reads");
        let offered = the_two_listings_offered(shop_front, Ok(build_service));
        assert_eq!(
            offered_as_seen(&offered.channels),
            vec![
                "/Alpha 5.3.0",
                "latest/Stable 5.2.2",
                "5.2lts/Stable 5.2.2",
                "5.1/Stable 5.1.2",
                "5.0/Stable 5.0.1",
                "4.5lts/Stable 4.5.14",
            ]
        );
        assert_eq!(
            offered.channels[0].the_build_services_file.as_deref(),
            Some("blender-5.3.0-alpha+main.64d181010e0d-linux.x86_64-release.tar.xz"),
            "the newest file of that version and kind"
        );
        assert!(offered.channels[1..].iter().all(|it| it.the_build_services_file.is_none()));
    }

    /// **A build service that could not be read costs its builds and nothing
    /// else**: the finished releases are still offered and the shortfall is
    /// named. **A kind that is not alpha, beta or candidate is named rather
    /// than offered unmarked.**
    #[test]
    fn a_build_service_that_did_not_answer_costs_only_its_own_builds() {
        let shop_front = read_the_listing(THE_NINTH_PASSS_LIST).expect("the answer reads");
        let offered = the_two_listings_offered(
            shop_front,
            Err(WhyTheListingCouldNotBeRead::TheShopFrontAnswered(503)),
        );
        assert!(offered.channels.iter().all(|it| it.risk == Risk::Stable));
        assert!(offered.entries_that_could_not_be_read.iter().any(|it| it.contains("build service")));
        let odd = WHAT_THE_BUILD_SERVICE_ANSWERED.replacen(r#""risk_id": "alpha""#, r#""risk_id": "nightly""#, 1);
        let read = read_the_build_services_listing(&odd).expect("it reads");
        assert_eq!(read.entries_that_could_not_be_read.len(), 1, "{read:?}");
        assert!(read_the_build_services_listing("{\"not\": \"a list\"}").is_err());
    }

    /// **B2's own two limits, stated rather than guessed around, held here so
    /// the narrowing is not quietly widened past them.**
    ///
    /// **An unfinished build of a series no `stable` channel in the answer
    /// carries is still offered**, even at an older version than a newer
    /// series' finished release: the comparison is within one series, never
    /// across the list. **And a `stable` entry the answer carries but that could
    /// not be read narrows nothing**, so its series' unfinished builds stay
    /// offered and the unread entry is named.
    #[test]
    fn the_narrowing_stays_inside_one_series_and_an_unread_release_narrows_nothing() {
        // 5.1.3 is older than 5.2.2 and 5.3.0's series has no finished release.
        let answer = THE_NINTH_PASSS_LIST
            .replace(
                r#""risk": "stable", "track": "5.1"},
         "download": {"size": 420000000}, "version": "5.1.2""#,
                r#""risk": "candidate", "track": "5.1"},
         "download": {"size": 420000000}, "version": "5.1.3""#,
            )
            .replace(
                r#""risk": "stable", "track": "4.5lts"},
         "download": {"size": 420859904}, "version": "4.5.14""#,
                r#""risk": "stable", "track": "4.5lts"},
         "download": {"size": 420859904}, "version": "4.5.14a""#,
            );
        let read = read_the_listing(&answer).expect("the answer reads");
        assert!(
            read.entries_that_could_not_be_read
                .iter()
                .any(|said| said.contains("4.5lts/stable at 4.5.14a")),
            "the unread finished release is named: {:?}",
            read.entries_that_could_not_be_read
        );
        let offered = offered_as_seen(&crate::blender::the_offered_list(&read.channels));
        for kept in ["latest/Edge 5.3.0", "5.1/Candidate 5.1.3", "4.5lts/Candidate 4.5.14", "4.5lts/Beta 4.5.0"] {
            assert!(
                offered.iter().any(|it| it == kept),
                "{kept} has no finished release of its series in the answer and must be offered: {offered:?}"
            );
        }
        assert!(
            !offered.iter().any(|it| it.contains("Candidate 5.2.2")),
            "the 5.2.2 candidates still go: {offered:?}"
        );
    }

    /// **B3: the latest finished release is the highest stable version across
    /// tracks, over the real answer.**
    #[test]
    fn the_latest_finished_release_comes_off_the_real_answer() {
        let read = the_answer_read();
        let latest = crate::blender::the_latest_finished_release(&read.channels)
            .expect("the default track's stable channel is there");
        assert_eq!(
            latest.version,
            Version {
                major: 5,
                minor: 2,
                patch: 1
            }
        );
        assert_eq!(latest.risk, Risk::Stable);
    }

    /// **An entry whose version is not version components is not offered and is
    /// named**, rather than vanishing from the list.
    #[test]
    fn an_entry_with_no_version_components_is_named_rather_than_dropped() {
        let read = the_answer_read();
        assert!(
            !read.channels.iter().any(|entry| entry.track == "2.82"),
            "an entry with no version components must not be offered"
        );
        // **Named, where it could have been offered** (`Server Setup` B2's line
        // of 2026-11-02): at or above the floor, or with no numbers to place.
        for (version, named) in [("5.2.3a", true), ("4.5.14a", true), ("next", true), ("5..1", true), ("4.2.9a", false)] {
            let answer = format!(
                r#"{{"channel-map": [{{"channel": {{"released-at": "2026-08-25T13:56:59+00:00", "risk": "stable", "track": "x"}},
                   "download": {{"size": 1}}, "version": "{version}"}}]}}"#
            );
            let read = read_the_listing(&answer).expect("the answer reads");
            assert!(read.channels.is_empty(), "{version} is not offered");
            assert_eq!(read.entries_that_could_not_be_read.len(), usize::from(named), "{version}: {read:?}");
        }
    }

    /// **`Server Setup` B2's line of 2026-11-02: the first 5.3 live pass's
    /// *3 entries could not be read*.** The three, read from the shop-front that
    /// day from this Mac: `2.82a`, `2.81a` and `2.79b`, tracks far below the
    /// floor. **They could never have been offered, so they are not counted**,
    /// and nothing else moves. **What it bites on**: every install list has
    /// said *3 entries could not be read* since the list was first shown.
    #[test]
    fn a_version_with_a_letter_below_the_floor_is_not_counted_as_unread() {
        let entry = |track: &str, version: &str| {
            format!(
                r#"{{"channel": {{"released-at": "2020-06-25T13:05:02+00:00", "risk": "stable", "track": "{track}"}},
                   "download": {{"size": 170643456}}, "version": "{version}"}}"#
            )
        };
        let answer = format!(
            r#"{{"channel-map": [{}, {}, {}, {}]}}"#,
            entry("2.82", "2.82a"),
            entry("2.81", "2.81a"),
            entry("2.79", "2.79b"),
            entry("5.2lts", "5.2.3"),
        );
        let read = read_the_listing(&answer).expect("the answer reads");
        assert!(read.entries_that_could_not_be_read.is_empty(), "{read:?}");
        assert_eq!(read.channels.len(), 1, "the readable one stands");
        let offered = the_two_listings_offered(read, Ok(TheListingAsRead { channels: Vec::new(), entries_that_could_not_be_read: Vec::new() }));
        assert!(offered.entries_that_could_not_be_read.is_empty());
    }

    /// **An entry whose risk is not one of B2's four is not offered and is
    /// named**, for the reason B2 gives: an unstable kind is included **and
    /// marked as what it is**, and a kind that is not known cannot be marked.
    #[test]
    fn an_entry_with_an_unknown_risk_is_named_rather_than_offered_unmarked() {
        let answer = WHAT_THE_SHOP_FRONT_ANSWERED.replace(
            r#""risk": "edge", "track": "latest""#,
            r#""risk": "something-new", "track": "latest""#,
        );
        let read = read_the_listing(&answer).expect("the rest of the answer still reads");
        assert!(
            !read.channels.iter().any(|entry| entry.version
                == Version {
                    major: 5,
                    minor: 3,
                    patch: 0
                }),
            "an entry whose kind is not known must not be offered"
        );
        assert!(
            read.entries_that_could_not_be_read
                .iter()
                .any(|said| said.contains("something-new")),
            "it has to be named: {:?}",
            read.entries_that_could_not_be_read
        );
    }

    /// An answer that is not the listing is said rather than guessed at.
    #[test]
    fn an_answer_that_is_not_the_listing_says_so() {
        let said = read_the_listing("not the listing").expect_err("this is not the listing");
        assert!(matches!(
            said,
            WhyTheListingCouldNotBeRead::TheAnswerIsNotTheShapeThatWasRead(_)
        ));
        let no_map = read_the_listing(r#"{"name": "blender"}"#)
            .expect_err("an answer with no entries is not the listing");
        assert!(matches!(
            no_map,
            WhyTheListingCouldNotBeRead::TheAnswerIsNotTheShapeThatWasRead(_)
        ));
    }

    /// **One unreadable entry costs that entry and nothing else.**
    ///
    /// **This is the case that failed the whole document before a code round.**
    /// The download block is the required field the shortfall is posed on;
    /// every other missing or renamed required field takes the same route, and
    /// the point of the test is that the rest of the list survives and the bad
    /// entry is named.
    #[test]
    fn one_entry_that_will_not_read_costs_that_entry_and_not_the_whole_list() {
        let answer = WHAT_THE_SHOP_FRONT_ANSWERED.replace(
            r#""download": {"size": 433758208}, "revision": 7754"#,
            r#""revision": 7754"#,
        );
        let read = read_the_listing(&answer).expect("the rest of the answer still reads");
        assert!(
            read.channels.len() >= 5,
            "one unreadable entry took the whole list with it: {:?}",
            read.channels.len()
        );
        assert!(
            !read.channels.iter().any(|entry| entry.risk == Risk::Edge),
            "the entry that will not read must not be offered"
        );
        assert!(
            read.entries_that_could_not_be_read
                .iter()
                .any(|said| said.contains("latest/edge")),
            "it has to be named: {:?}",
            read.entries_that_could_not_be_read
        );
    }

    /// **An entry that gives nothing to name it by is still named**, by the
    /// shop-front's own text for it.
    #[test]
    fn an_entry_with_nothing_to_name_it_by_is_still_named() {
        let read = read_the_listing(r#"{"channel-map": [ 7 ]}"#).expect("the answer is the listing");
        assert!(read.channels.is_empty());
        assert_eq!(read.entries_that_could_not_be_read.len(), 1);
        assert!(read.entries_that_could_not_be_read[0].starts_with('7'));
    }

    /// **No hop of the listing request may leave HTTPS, and the hops are
    /// bounded.** The client's own defaults are the opposite of both — HTTPS-only
    /// off and ten redirects — so this reads the built client back rather than
    /// trusting the builder above it.
    #[test]
    fn no_hop_of_the_listing_request_may_leave_https() {
        let client = the_client_this_request_is_made_on();
        assert!(client.config().https_only());
        assert_eq!(
            client.config().max_redirects(),
            HOW_MANY_TIMES_THE_SHOP_FRONT_MAY_HAND_THIS_ON
        );
        assert_eq!(
            client.config().timeouts().global,
            Some(THE_LISTINGS_OWN_BOUND)
        );
    }

    /// **No processor check is made here, or anywhere.** There was one reading
    /// of the processor in the workspace, in `Security and licence`, and on
    /// 2026-10-24 it went with the provider that needed it (the Creator's
    /// eighteenth notes, note 3). **This is what says no copy of it is back**:
    /// no instruction name is written in this file at all.
    #[test]
    fn no_processor_check_is_made_here() {
        let whole = include_str!("listing.rs");
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

    /// **A listing request whose far end accepts the connection and then
    /// says nothing ends at the listing's own thirty seconds, says it got no
    /// answer, and leaves nothing behind it.** The dead-line desk coverage,
    /// 2026-10-19: the request is the shop-front's own, on its own client,
    /// pointed at a loopback listener that takes the connection and never
    /// answers — the shape of a line gone silent before any answer came.
    /// **The body's silence is not proved here and cannot be at this desk**:
    /// the client goes nowhere but HTTPS and trusts only the bundled roots, so
    /// no local listener can carry an answer as far as a body. The whole
    /// request is under the one bound, which the test above reads back.
    #[test]
    fn a_listing_whose_far_end_goes_silent_ends_at_its_own_bound_and_says_so() {
        let listener = std::net::TcpListener::bind(("127.0.0.1", 0)).expect("a loopback listener");
        let address = format!("https://127.0.0.1:{}/v2/snaps/info/blender", listener.local_addr().expect("an address").port());
        let held = std::thread::spawn(move || {
            // **Taken and held, and nothing said**, until the test is done.
            let taken = listener.accept().map(|(stream, _)| stream);
            std::thread::sleep(THE_LISTINGS_OWN_BOUND + Duration::from_secs(10));
            drop(taken);
        });
        let began = std::time::Instant::now();
        let fetched = the_listing_is_fetched_from(&address, Some(THE_STORES_REQUIRED_HEADER));
        let took = began.elapsed();
        println!("a silent listing came to {fetched:?} in {took:?}");
        assert!(
            matches!(fetched, Err(WhyTheListingCouldNotBeRead::TheRequestDidNotGetAnAnswer(ref said)) if !said.is_empty()),
            "{fetched:?}"
        );
        assert!(
            took + Duration::from_secs(1) >= THE_LISTINGS_OWN_BOUND && took <= THE_LISTINGS_OWN_BOUND + Duration::from_secs(5),
            "the silent listing ended after {took:?}, not at its own {THE_LISTINGS_OWN_BOUND:?}"
        );
        drop(held);
    }
}
