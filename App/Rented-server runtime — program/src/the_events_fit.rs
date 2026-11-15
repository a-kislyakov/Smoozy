//! **Every event this program journals fits `State, identity and recovery`
//! E6a's per-entry budget, whatever the names** — the guard the Creator's
//! twenty-ninth notes, note 11 (piece J) asked for.
//!
//! **Why it exists.** On a live pass with the Creator's file, eleven of twelve
//! *a preview was published* events came out at 531 to 597 bytes against the
//! 512 the journal takes, and were refused. Their words had been held to a
//! figure counted before the entry was written, and a name in the entry is
//! written twice over: as the bytes of the words, and again as those words are
//! escaped inside the entry. So History never got those View Layers.
//!
//! **What it does.** It builds every event kind the program journals, with
//! names, labels and versions far longer than any real one — plain letters,
//! the bytes that take the most room, quotation marks and backslashes that are
//! written twice, and characters of several bytes — and measures each as the
//! journal would write it at the largest sequence number there is. Every one
//! must fit. **A kind added to this program and not to [`EVERY_KIND`] fails
//! [`every_kind_the_framing_names_is_here`].**
//!
//! **Markers used in this file that belong to another document, each with the
//! document it belongs to.** `State, identity and recovery` E6a. Every other
//! marker is written out with its document at the site.

use std::time::{Duration, SystemTime, UNIX_EPOCH};

use smoozy_framing::requests as asked;
use smoozy_job_files_machine::main_output::WhereThePictureCameFrom;
use smoozy_job_files_machine::reading::ArtistsText;
use smoozy_queue_history_machine::previews::ThePublishedPreview;
use smoozy_runtime::written_form::the_measure_of_an_entry::the_bytes_it_would_take;
use smoozy_state::identities::{Identity, IdentityKind};
use smoozy_state::journal::{WhatHappened, THE_PER_ENTRY_BUDGET};
use smoozy_state::observed::{Clock, MachineClock, Reading};

/// **Every kind this program journals**, by the constant it is spelled under.
const EVERY_KIND: [&str; 13] = [
    asked::THE_JOB_RECORD_WRITTEN_EVENT,
    asked::THE_HOLD_PLACED_EVENT,
    asked::THE_RENDER_ATTEMPT_STARTED_EVENT,
    asked::THE_RECORDED_READING_WRITTEN_EVENT,
    asked::THE_RECORDED_OUTCOME_WRITTEN_EVENT,
    asked::THE_DISPOSITION_EVENT,
    asked::THE_RELEASE_EVENT,
    asked::THE_JOB_RECORD_REMOVED_EVENT,
    asked::THE_PREVIEW_PUBLISHED_EVENT,
    asked::THE_PREVIEW_SET_MADE_EVENT,
    asked::THE_TELEGRAM_CHOICE_CHANGED_EVENT,
    crate::unix::THE_REFUSED_CALLER_EVENT,
    crate::unix::THE_ROLE_WORKER_FAILED_EVENT,
];

/// **The latest moment this measure allows for**: eleven digits of seconds,
/// so a clock set wildly forward or back is still inside it.
fn at_the_latest() -> SystemTime {
    UNIX_EPOCH + Duration::new(99_999_999_999, 999_999_999)
}

/// The clock the supervisor names: its installation identity's text.
fn the_machines_clock() -> Clock {
    Clock::AMachine(MachineClock(
        Identity::issue(IdentityKind::Installation).expect("randomness").as_text(),
    ))
}

fn an(kind: IdentityKind) -> Identity {
    Identity::issue(kind).expect("randomness")
}

/// One event, as the supervisor journals it.
fn the_event(kind: &str, about: Option<Identity>, says: &str) -> WhatHappened {
    WhatHappened {
        kind: kind.to_string(),
        about,
        says: says.to_string(),
        happened_at: Reading::taken((), at_the_latest(), the_machines_clock()),
    }
}

/// **The texts this guard tries in every place a user's text goes**: each
/// shape at every length up to well past anything real.
fn texts_far_longer_than_any_real_one() -> Vec<String> {
    let mut out = Vec::new();
    for length in (0..=40).chain((48..=640).step_by(16)).chain([1_000, 4_000]) {
        out.push("a".repeat(length));
        out.push("\"".repeat(length));
        out.push("\\".repeat(length));
        out.push("\u{1}".repeat(length));
        out.push("\u{20ac}".repeat(length));
    }
    out
}

/// **The byte strings a View Layer's or a File Output node's name can be**:
/// the artist's bytes, written one number per byte, so the widest byte takes
/// the most room.
fn names_far_longer_than_any_real_one() -> Vec<Vec<u8>> {
    let mut out = Vec::new();
    for length in (0..=64).chain((72..=640).step_by(8)).chain([1_000, 4_000]) {
        out.push(vec![b'a'; length]);
        out.push(vec![0xff; length]);
        out.push(vec![b'"'; length]);
    }
    out
}

/// What failed, by kind, with the bytes it would take.
fn too_long(failures: &mut Vec<String>, kind: &str, what: &WhatHappened) {
    let bytes = the_bytes_it_would_take(what);
    if bytes > THE_PER_ENTRY_BUDGET {
        failures.push(format!(
            "{kind}: {bytes} bytes, its words {} long",
            what.says.len()
        ));
    }
}

/// **`Queue & History` P3: a copy's words, and the set's that say the same**,
/// at every name, frame and place — the fault the live pass met.
#[test]
fn piece_j_every_preview_event_fits_whatever_the_names() {
    let job = an(IdentityKind::Job);
    let operation = an(IdentityKind::Operation).as_text();
    let mut failures = Vec::new();
    let mut largest = 0;
    for name in names_far_longer_than_any_real_one() {
        for (frame, place, main) in [(1, 0, true), (i64::MIN, u32::MAX, false), (i64::MAX, 11, false)] {
            for source in [
                None,
                Some(WhereThePictureCameFrom::TheOutputPath),
                Some(WhereThePictureCameFrom::AFileOutputNode(ArtistsText::of(name.clone()))),
            ] {
                let says = ThePublishedPreview {
                    frame,
                    operation: operation.clone(),
                    view_layer: Some(name.clone()),
                    main,
                    place,
                    source,
                    the_names_are_in_the_record: false,
                }
                .as_says();
                for kind in [asked::THE_PREVIEW_PUBLISHED_EVENT, asked::THE_PREVIEW_SET_MADE_EVENT] {
                    let what = the_event(kind, Some(job.clone()), &says);
                    largest = largest.max(the_bytes_it_would_take(&what));
                    too_long(&mut failures, kind, &what);
                }
            }
        }
    }
    assert!(
        failures.is_empty(),
        "{} preview events do not fit; the first: {:#?}",
        failures.len(),
        &failures[..failures.len().min(5)]
    );
    eprintln!("piece J: the largest preview event takes {largest} bytes");
}

/// **`Rented-server runtime` U20 and `Queue & History` L22: a drop copies the
/// job's label in**, since its records go in the same act, so the label is
/// what must give way — at every label and version.
#[test]
fn piece_j_every_drop_fits_whatever_the_label() {
    let hold = an(IdentityKind::Operation);
    let job = an(IdentityKind::Job);
    let mut failures = Vec::new();
    let mut largest = 0;
    for label in texts_far_longer_than_any_real_one() {
        for reason in [
            smoozy_queue_history_machine::waiting::THE_DEPENDENCY_WAS_CANCELLED.to_string(),
            "the dependency was given up".to_string(),
        ] {
            let says = crate::the_telegram_journal::the_drops_entry(
                &crate::the_telegram_journal::ADropAsJournalled {
                    the_phone_is_silent: true,
                    reason,
                    frames_held: Some(u64::MAX),
                    the_first_frame: Some(i64::MIN),
                    label: label.clone(),
                },
            );
            let what = the_event(asked::THE_RELEASE_EVENT, Some(hold.clone()), &says);
            largest = largest.max(the_bytes_it_would_take(&what));
            too_long(&mut failures, asked::THE_RELEASE_EVENT, &what);
        }
        for version in ["5.3.0".to_string(), label.clone()] {
            let says = format!(
                "Telegram off: {}",
                crate::unix::the_words_of_a_drop_for_a_removed_blender(&version, &label)
            );
            let what = the_event(asked::THE_JOB_RECORD_REMOVED_EVENT, Some(job.clone()), &says);
            largest = largest.max(the_bytes_it_would_take(&what));
            too_long(&mut failures, asked::THE_JOB_RECORD_REMOVED_EVENT, &what);
        }
    }
    assert!(
        failures.is_empty(),
        "{} drops do not fit; the first: {:#?}",
        failures.len(),
        &failures[..failures.len().min(5)]
    );
    eprintln!("piece J: the largest drop takes {largest} bytes");
}

/// **Every other kind**, at the longest words it is ever given. None of them
/// carries a name the user chose, which is why each is a fixed sentence here.
#[test]
fn piece_j_every_other_kind_fits_at_its_longest() {
    use smoozy_runtime::supervisor::what_a_journalled_disposition_says;
    use smoozy_state::machine_operations::Disposition;
    let job = an(IdentityKind::Job);
    let operation = an(IdentityKind::Operation);
    let mut longest: Vec<(&str, Option<Identity>, String)> = vec![
        (
            asked::THE_JOB_RECORD_WRITTEN_EVENT,
            Some(job.clone()),
            crate::the_telegram_journal::what_a_resubmissions_event_says(
                asked::IT_IS_RENDERING,
                &an(IdentityKind::Job),
            ),
        ),
        // The phone's *Resume* (the four questions, batch 4), at its longest
        // frame.
        (
            asked::THE_JOB_RECORD_WRITTEN_EVENT,
            Some(job.clone()),
            crate::the_telegram_journal::what_a_resumptions_event_says(
                asked::IT_IS_RENDERING,
                &an(IdentityKind::Job),
                i64::MIN,
            ),
        ),
        (asked::THE_HOLD_PLACED_EVENT, Some(operation.clone()), job.as_text()),
        (
            asked::THE_RENDER_ATTEMPT_STARTED_EVENT,
            Some(job.clone()),
            format!("attempt {}", u32::MAX),
        ),
        (
            asked::THE_RECORDED_READING_WRITTEN_EVENT,
            Some(job.clone()),
            format!("attempt {}", u32::MAX),
        ),
        (
            asked::THE_RECORDED_OUTCOME_WRITTEN_EVENT,
            Some(job.clone()),
            "Telegram off: no status was reaped".to_string(),
        ),
        (
            asked::THE_JOB_RECORD_REMOVED_EVENT,
            Some(job.clone()),
            format!("Telegram off: {}", asked::IT_WAS_CLEARED),
        ),
        (
            asked::THE_RELEASE_EVENT,
            Some(operation.clone()),
            "the hold's release outcome was written: released".to_string(),
        ),
        (
            asked::THE_TELEGRAM_CHOICE_CHANGED_EVENT,
            Some(job.clone()),
            smoozy_telegram::job_messages::TheReportingChoice::APictureEveryFewMinutes {
                minutes: u8::MAX,
            }
            .in_the_manifests_form()
            .to_string(),
        ),
        (
            crate::unix::THE_REFUSED_CALLER_EVENT,
            None,
            "a caller of another login was refused before its request was framed".to_string(),
        ),
        (
            crate::unix::THE_ROLE_WORKER_FAILED_EVENT,
            None,
            format!(
                "it has ended {} times under this run, and starting it again: the setup-in-progress \
                 state was not read",
                u64::MAX
            ),
        ),
    ];
    for disposition in [
        Disposition::Resumed,
        Disposition::ReRunOnce,
        Disposition::LeftStopped,
        Disposition::GivenUp,
    ] {
        for statement in [true, false] {
            longest.push((
                asked::THE_DISPOSITION_EVENT,
                Some(operation.clone()),
                what_a_journalled_disposition_says(disposition, statement),
            ));
        }
        longest.push((
            asked::THE_DISPOSITION_EVENT,
            Some(operation.clone()),
            format!("{disposition:?}; the second attempt is started by the line"),
        ));
    }
    for words in [
        asked::IT_ENDED_AT_ITS_BOUNDARY_NOT_COMPLETED,
        asked::IT_WAS_STOPPED,
        asked::IT_COMPLETED,
    ] {
        longest.push((asked::THE_DISPOSITION_EVENT, Some(an(IdentityKind::Run)), words.to_string()));
    }
    let mut failures = Vec::new();
    for (kind, about, says) in &longest {
        too_long(&mut failures, kind, &the_event(kind, about.clone(), says));
    }
    assert!(failures.is_empty(), "{failures:#?}");
}

/// **A kind spelled in the framing and journalled here is in [`EVERY_KIND`]**,
/// so a new kind is measured before it reaches a machine. The framing names
/// every kind the desk reads as a constant spelled `THE_…_EVENT`.
#[test]
fn every_kind_the_framing_names_is_here() {
    let framing = include_str!("../../Server protocol and transfers — framing/src/requests.rs");
    let named: Vec<&str> = framing
        .lines()
        // **Every kind is spelled `THE_…_EVENT`**; `AN_EVENT` is the word for
        // one item of a read, and no kind.
        .filter_map(|line| line.strip_prefix("pub const THE_"))
        .filter_map(|line| line.split_once("_EVENT: &str = \""))
        .filter_map(|(_, rest)| rest.strip_suffix("\";"))
        .collect();
    assert!(named.len() >= 11, "the framing's kinds were found: {named:?}");
    for kind in named {
        assert!(EVERY_KIND.contains(&kind), "{kind} is journalled and not measured here");
    }
}
