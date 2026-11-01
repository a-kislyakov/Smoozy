//! **A job's ending, read while connected, is posted** — `Desktop app and
//! delivery` T9 to T12, T18 and T27, out of the 38's piece 1 (2026-10-03).
//!
//! **What decides *now* is not here.** It is
//! [`crate::notifications::when`]'s boundary, kept by
//! [`crate::servers::the_machines_work::the_read_forward_pass`]: an ending
//! applied at a connection's §7 step 3 is the catch-up account and posts
//! nothing (T18); one applied by a read forward inside the standing period
//! after that step completed is put out, and P10d's next pass hands it here.
//! **What each ending posts is not here either** — it is
//! [`crate::notifications::kinds::what_this_ending_posts`]. **This file reads
//! the job's own record for that function and hands what it composes to the
//! one notification service**, and nothing else.
//!
//! **Every figure is the local record's, read after the fill-in has carried
//! the note's half over** — the duration from the note's two machine-clock
//! moments, the frame a failed job was on (`Queue & History` N6), and the
//! server's label as the record copied it at submission (`Queue & History`
//! N12). **Nothing is read off a machine here**, and nothing is reckoned.
//!
//! **Markers used in this file that belong to another document, each with the
//! document it belongs to.** `Queue & History` N1, N6, N12. `State, identity
//! and recovery` D5.

use smoozy_queue_history::record::LocalJobRecord;
use smoozy_state::identities::Identity;
use tauri::{AppHandle, Manager, Runtime};

use crate::keys::LendsTheRecord;
use crate::notifications::kinds::{what_this_ending_posts, AGroupMember, AJobsRecordedEnding};
use crate::notifications::{picture, APost, TheNotificationService};
use crate::record::job_records::JobRecords;
use crate::servers::the_machines_work::with_the_record;
use crate::typography::Verbatim;

/// **What these endings post, each composed from the job's own record.**
///
/// **A job whose record is gone, is another server's, or holds no ending
/// posts nothing** — there is nothing true to say about it. **One key is
/// posted once** in the list: two members of a group ending in one read both
/// close the group, and the group's finishing is said once (T11).
pub fn what_these_endings_post(
    record: &dyn LendsTheRecord,
    server: &Identity,
    jobs: &[Identity],
) -> Vec<APost> {
    let mut posts: Vec<APost> = Vec::new();
    for job in jobs {
        let Some(Some(held)) =
            with_the_record(record, |connection| JobRecords::over(connection).read(job).ok().flatten())
        else {
            continue;
        };
        if &held.server.local_server != server {
            continue;
        }
        let Some(ending) = held.the_ending_learned() else {
            continue;
        };
        let members = match &held.group {
            Some(group) => with_the_record(record, |connection| {
                JobRecords::over(connection).in_the_group(group).ok()
            })
            .flatten()
            .unwrap_or_default()
            .iter()
            .map(a_member)
            .collect(),
            None => Vec::new(),
        };
        let ended = AJobsRecordedEnding {
            job: held.job.clone(),
            group: held.group.clone(),
            ending,
            started: held.what_the_note_carried.started.clone(),
            ended: held.what_the_note_carried.ended.clone(),
            // **The same figure History's entry draws** — `Queue & History`
            // N6's, and that document's N1's *given up, where it was* behind it.
            at_frame: held
                .what_the_note_carried
                .last_frame_figure
                .or(held.what_the_note_carried.given_up_where_it_was),
        };
        // **The name is the job's, called what Queue calls it**, and a batch
        // is called what its members are: an all-scenes submission gives
        // every member the one label and the one file.
        let name = crate::surface::the_queue::what_the_job_is_called(&held);
        let on = held.server.label.clone().unwrap_or_default();
        // **T15: the finished notification attaches the entry's own picture**
        // — the newest copy the job record holds, read now. **T17: where none
        // is held yet the post goes without one**, never waiting. The 38's
        // piece 3.
        let held_picture = with_the_record(record, |connection| {
            crate::the_previews::the_picture_held_for(connection, &held.job)
        })
        .flatten();
        for what in what_this_ending_posts(&ended, &members) {
            if posts.iter().any(|posted| posted.key == what.key) {
                continue;
            }
            let carried = match what.kind {
                crate::notifications::kinds::Kind::AJobFinished => held_picture.as_ref(),
                _ => None,
            };
            posts.push(
                APost::from(
                    what,
                    Verbatim::new(name.clone()),
                    picture::the_picture_for(std::env::consts::OS, carried),
                )
                .on_the_server(Verbatim::new(on.clone())),
            );
        }
    }
    posts
}

fn a_member(record: &LocalJobRecord) -> AGroupMember {
    AGroupMember {
        job: record.job.clone(),
        ending: record.the_ending_learned(),
        started: record.what_the_note_carried.started.clone(),
        ended: record.what_the_note_carried.ended.clone(),
    }
}

/// **Each is attempted and forgotten** (T23), through the one service and its
/// switches (T8, T17a): a kind the user switched off is not attempted.
pub fn are_handed_to(service: &TheNotificationService, posts: Vec<APost>) {
    for post in posts {
        service.post(post);
    }
}

/// **The running application's end**: composed from the record and posted
/// through the service the run manages.
pub fn are_posted<R: Runtime>(
    app: &AppHandle<R>,
    record: &dyn LendsTheRecord,
    server: &Identity,
    jobs: &[Identity],
) {
    let Some(service) = app.try_state::<std::sync::Arc<TheNotificationService>>() else {
        return;
    };
    are_handed_to(service.inner(), what_these_endings_post(record, server, jobs));
}
