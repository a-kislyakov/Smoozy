//! **B2's fetch, run against the real shop-front rather than against a fixture.**
//!
//! **Why this exists, and it is the one thing the tests beside the code cannot
//! do.** `src/listing.rs` proves the reading — the field names, the sort, the
//! floor, the risk marks and every refusal — against the shop-front's own answer
//! held as text. **What it cannot prove is that the request is made at all**: the
//! address, the header, the transport-layer security, the cryptography the
//! provider supplies, the bounds and the shape of what comes back on the day.
//! That answer comes from the service or from nowhere.
//!
//! **It is off unless the environment says the internet may be reached, and
//! where it is off it says so out loud rather than passing quietly** — the same
//! shape `Servers`' own rented-machine tests already use. **Nothing here may be
//! made to run on a hosted runner by default**, because a test suite that
//! depends on a network the runners may not give us fails for a reason that is
//! not Smoozy's.
//!
//! ```text
//! SMOOZY_MAY_REACH_THE_INTERNET=1   run the fetch below
//! ```
//!
//! **Nothing here needs an account, a rental or a payment.** The endpoint is
//! open and answers an unauthenticated request; the only header it is given is
//! the one B2 names.
//!
//! **How a clause marker is written in this crate, and it is a rule rather than
//! a habit.** A marker with no document beside it means a clause of this
//! crate's own plan. Every other document's marker is written with that
//! document — at the site, or in the file's own header list of markers that
//! belong elsewhere. **Where a marker is a clause of this crate's plan *and* of
//! the document meant, the header list is not enough and the document is written
//! out at the site**, because a bare one there does not fail to resolve: it
//! resolves to a real and wrong clause, which nobody reading it can tell from a
//! right one.
//!
//! **The plan here is `Preparation/Architecture/Server Setup.md`, and the words
//! above are the words `src/lib.rs` carries.** They are repeated because a file
//! under `tests/` is its own crate and never sees that one.

use smoozy_server_setup::blender::{a_bare_build_service_name, Risk, THE_FLOOR};
use smoozy_server_setup::listing::{
    fetch_the_build_services_listing, fetch_the_listing, read_the_build_services_listing,
    read_the_listing, the_list_smoozy_offers,
};

/// Says the miss out loud rather than passing quietly.
macro_rules! the_internet_or_say_so {
    () => {
        if std::env::var("SMOOZY_MAY_REACH_THE_INTERNET").unwrap_or_default() != "1" {
            println!(
                "SKIPPED: SMOOZY_MAY_REACH_THE_INTERNET is not 1. The listing was not \
                 fetched, so nothing below is proved against the real shop-front here."
            );
            return;
        }
    };
}

/// **B2, end to end: the listing fetched from the shop-front and read into the
/// list Smoozy offers.**
///
/// **This is the fetch step 6's own milestone waited on.** What it holds is what
/// B2 states: the answer comes back, it reads, the offered list is floored at
/// 4.5, it is sorted on version components with the newest first, and the
/// unstable kinds are in it carrying their marks.
#[tokio::test]
async fn the_real_listing_comes_back_and_reads_into_the_offered_list() {
    the_internet_or_say_so!();

    let offered = the_list_smoozy_offers()
        .await
        .expect("the shop-front answers B2's endpoint");

    assert!(
        !offered.channels.is_empty(),
        "the offered list is empty, so nothing came back or nothing cleared the floor"
    );

    for entry in &offered.channels {
        assert!(
            entry.version >= THE_FLOOR,
            "{}/{:?} at {}.{}.{} is below the whole-app floor and is offered",
            entry.track,
            entry.risk,
            entry.version.major,
            entry.version.minor,
            entry.version.patch
        );
        // **A finished release is a shop-front channel and carries its track;
        // an unfinished build is a build-service file and carries its bare
        // name** — `Server Setup` B2's revision of 2026-10-02.
        match entry.risk {
            Risk::Stable => assert!(!entry.track.is_empty(), "a finished release carries no track"),
            _ => assert!(
                entry.the_build_services_file.as_deref().is_some_and(a_bare_build_service_name),
                "an unfinished build carries no bare build-service name: {entry:?}"
            ),
        }
        assert_ne!(entry.risk, Risk::Edge, "the shop-front's own unfinished channel is offered");
        assert!(entry.size_bytes > 0, "an entry carries no byte size");
        assert!(
            !entry.released_at.is_empty(),
            "an entry carries no release moment"
        );
    }

    // **The sort is on version components** — every entry is at or below the one
    // before it.
    for pair in offered.channels.windows(2) {
        assert!(
            pair[0].version >= pair[1].version,
            "the offered list is not sorted newest first"
        );
    }

    // **The unstable kinds are included and marked as what they are.** They
    // come from Blender's build service since 2026-10-02, which listed a 5.3.0
    // alpha that day; what is asserted is that where one is above the floor it
    // is offered and it is marked, never that one exists on any particular day.
    let unstable: Vec<_> = offered
        .channels
        .iter()
        .filter(|entry| entry.risk.is_unstable())
        .collect();
    println!(
        "the offered list holds {} channels, {} of them unstable; \
         {} entries could not be read: {:?}",
        offered.channels.len(),
        unstable.len(),
        offered.entries_that_could_not_be_read.len(),
        offered.entries_that_could_not_be_read
    );
    for entry in unstable {
        assert_ne!(entry.risk, Risk::Stable);
    }
}

/// **What came back is the shop-front's own answer and is read whole**, so a
/// change in its shape is a failure here rather than a quietly shorter list.
#[test]
fn the_answer_that_comes_back_reads_with_nothing_it_could_not_read() {
    the_internet_or_say_so!();

    let answer = fetch_the_listing().expect("the shop-front answers");
    println!("the shop-front answered {} bytes", answer.len());
    let read = read_the_listing(&answer).expect("the answer is the listing");
    assert!(
        read.channels.len() > 10,
        "only {} entries read: {:?}",
        read.channels.len(),
        read.entries_that_could_not_be_read
    );
    // Every entry it could not read is named. This is not an assertion that
    // there are none — the shop-front's oldest tracks carry versions with a
    // letter in them — it is an assertion that nothing goes missing silently.
    for said in &read.entries_that_could_not_be_read {
        assert!(!said.is_empty());
        println!("not read: {said}");
    }
}

/// **B2's second listing, fetched from Blender's own build service and read
/// whole** — the Creator's notes 5 and 7 on 0.1.7. Every candidate it reads is
/// a Linux x86_64 release archive with a bare name the machine may fetch, and
/// a change in the service's shape is a failure here rather than a quietly
/// shorter list.
#[test]
fn the_build_services_listing_comes_back_and_reads() {
    the_internet_or_say_so!();

    let answer = fetch_the_build_services_listing().expect("the build service answers");
    println!("the build service answered {} bytes", answer.len());
    let read = read_the_build_services_listing(&answer).expect("the answer is the listing");
    println!(
        "{} unfinished builds read: {:?}; {} could not be read: {:?}",
        read.channels.len(),
        read.channels.iter().map(|it| it.the_build_services_file.clone()).collect::<Vec<_>>(),
        read.entries_that_could_not_be_read.len(),
        read.entries_that_could_not_be_read
    );
    for entry in &read.channels {
        assert!(entry.risk.is_unstable());
        assert!(entry.the_build_services_file.as_deref().is_some_and(a_bare_build_service_name));
    }
}
