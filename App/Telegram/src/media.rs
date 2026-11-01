//! **The media worker, the moving picture and the contact sheet — `Telegram`
//! §12, M1 to M9 with M4a and M4b, and §19's *media production* row and its
//! arbitration paragraph.** Rules over values.
//!
//! **What is owed and not here**: the media worker itself, driving the Blender
//! the manifest names, staging and publishing by rename, and the admission
//! point on a real machine — step 8b (the step plan's §8 item 6). **Each
//! constant below that states a rule the worker must keep says so.**
//!
//! **M1 — the source is cited, never chosen here**: the job's frame-shaped
//! candidates at the destination the rungs chose, in the inventory's frame
//! order (`Job files and outputs` §8.3). Every function here takes that list as
//! handed in and never builds one; a job with no frame-shaped candidate hands
//! in none and gets neither a moving picture nor a contact sheet.
//!
//! **§19, a media production: atomic by staging and one publication** (M5,
//! `Job files and outputs` B29's pattern). *Act owed*: step 8b.
//!
//! **M9 — this closes `Job files and outputs` §14 item 17 from this side.**
//! What stays open is §21 item 7's observed behaviour, which nothing here
//! answers.

use crate::job_messages::{AServerLabel, TheServersName};
use crate::job_messages::{AMarkedText, a_user_message};
use crate::pacing::{HowASendEnded, TheSendQueue};
use crate::pictures::{ADerivative, WhichPictureItIs};
use crate::transport::TheClient;
use std::sync::mpsc::Receiver;

/// **`Telegram` M2 — a message holds ten pictures and no more** (observed:
/// eleven refused).
pub const THE_MOST_PICTURES_IN_ONE_MESSAGE: usize = 10;

/// **`Telegram` I3a — the most an album's pictures may weigh together, in
/// one upload.** *A Build figure, not Telegram's*: Telegram publishes a cap
/// per photo (10 MB, I3) and per file (50 MB), and none for a whole upload
/// (`Telegram` §21 item 36). Chosen from the live Telegram pass of 2026-10-31
/// (its fault 3): albums of two and four 3840 × 2160 pictures went, and one of
/// eight was refused as too large (*413*). The one copy this project has
/// weighed, an eight-bit 9000 × 3000 copy at 12.9 MB, is about half a byte a
/// pixel, which puts a 3840 × 2160 copy near 4 MB — four near 17 MB, eight near
/// 33 MB. 20,000,000 sits between what went and what was refused, and well
/// under the 50 MB file cap. A full-HD album of ten (near 1 MB a picture)
/// still goes whole. **The next live pass measures where the refusal starts,
/// and this figure moves to it.**
pub const THE_MOST_AN_ALBUM_MAY_WEIGH: usize = 20_000_000;

/// **`k` places out of `n`, evenly spread**: the first, the last, and the rest
/// spread evenly between them; all `n` where `k` is `n` or more, and the
/// first alone where `k` is one.
pub fn evenly_spread(n: usize, k: usize) -> Vec<usize> {
    if k >= n {
        return (0..n).collect();
    }
    match k {
        0 => Vec::new(),
        1 => vec![0],
        _ => {
            let last = k - 1;
            (0..=last).map(|i| (i * (n - 1) + last / 2) / last).collect()
        }
    }
}

/// **`Telegram` M2 — the contact sheet is a selection, not a composed image**:
/// at most ten of the job's derivatives, in frame order. **Which ten is the
/// design's** (`#tgview`): *"the first frame, the last, and eight spread evenly
/// between them"*. Ten or fewer frames are all shown.
pub fn the_contact_sheet(frames_in_order: &[u64]) -> Vec<u64> {
    evenly_spread(frames_in_order.len(), THE_MOST_PICTURES_IN_ONE_MESSAGE)
        .into_iter()
        .map(|at| frames_in_order[at])
        .collect()
}

/// **`Telegram` I3a, for the contact sheet — fewer pictures, evenly spread,
/// where the chosen ones together weigh more than one album may**
/// ([`THE_MOST_AN_ALBUM_MAY_WEIGH`]). The most that fit, spread evenly across
/// the chosen frames, first and last kept; one picture always goes, since one
/// is a photo and I3 already holds it under its own cap. The sheet's line then
/// counts what is shown, *4 of 8 frames, evenly spread*. The live Telegram
/// pass of 2026-10-31, fault 3; Specs `Telegram`, *Frames as they arrive*.
pub fn within_the_albums_weight(chosen: Vec<ADerivative>) -> Vec<ADerivative> {
    let weighs = |at: &[usize]| at.iter().map(|&i| chosen[i].bytes.len()).sum::<usize>();
    let n = chosen.len();
    let Some(fits) = (1..=n).rev().map(|k| evenly_spread(n, k)).find(|at| at.len() == 1 || weighs(at) <= THE_MOST_AN_ALBUM_MAY_WEIGH) else {
        return chosen;
    };
    if fits.len() == n {
        return chosen;
    }
    let mut chosen: Vec<Option<ADerivative>> = chosen.into_iter().map(Some).collect();
    fits.into_iter().filter_map(|at| chosen[at].take()).collect()
}

/// **The contact sheet's caption** — the design's words: *"Kitchen — final ·
/// 10 of 240 frames, evenly spread"*. **Where the job's chosen frames hold more
/// than one View Layer, the sheet is one of several and its line names its
/// layer** (`Telegram` M2 as changed 2026-10-31, the Creator's seventeenth
/// notes, note 10; the drawing's `tg-layers`): *Living room — final ·
/// Reflections · 10 of 240 frames, evenly spread*. `layer` is none for a job
/// whose chosen frames hold one copy each, whose one sheet names nothing.
pub fn the_contact_sheets_caption(
    server_label: &(impl TheServersName + ?Sized),
    job_label: &str,
    layer: Option<&WhichPictureItIs>,
    shown: usize,
    of: usize,
) -> AMarkedText {
    let named = layer.map(|layer| format!(" · {}", layer.as_said())).unwrap_or_default();
    if shown == of {
        a_user_message(server_label, job_label, &format!("{named} · all {of} frame{}", if of == 1 { "" } else { "s" }), "")
    } else {
        a_user_message(server_label, job_label, &format!("{named} · {shown} of {of} frames, evenly spread"), "")
    }
}

/// **`Telegram` M2 — the caption of a named sheet's every other picture**, as
/// `Telegram` I2's album writes its own (*"Each picture's caption names its
/// View Layer as I2's does"*): *Frame 140 · Reflections · Not colour-accurate*.
/// The first picture carries the sheet's line instead, which names the layer
/// too, so the name shows on a computer's Telegram, where only the first
/// caption is seen under an album (the plan and drawing pass, 2026-10-31).
fn a_named_sheets_picture_caption(frame: u64, layer: &WhichPictureItIs) -> AMarkedText {
    AMarkedText::plain(format!("Frame {frame} · {} · Not colour-accurate", layer.as_said()))
}

/// **`Telegram` M2 — the role worker sends the selection at the job's end**, as
/// one album with its caption on the first picture (`Telegram` R7). One picture
/// goes as a photo, since an album takes two to ten. **It goes in `Telegram`
/// C6b's upload lane**, one picture or ten: C6b names the album, and the sheet
/// is one act whatever its count.
///
/// **One such album per View Layer** where the chosen frames hold more than one
/// (`Telegram` M2 as changed 2026-10-31): the caller sends them one after
/// another and hands each in with its `layer`, which the sheet's line names,
/// and every other picture's caption names too
/// ([`a_named_sheets_picture_caption`]). With no `layer` the sheet is as before:
/// its caption on the first picture alone.
///
/// **Within one album's weight** ([`within_the_albums_weight`], `Telegram`
/// I3a): where the chosen pictures together weigh too much for one upload,
/// fewer go, evenly spread, and the line counts what is shown.
pub fn offer_the_contact_sheet(
    queue: &TheSendQueue,
    chat: i64,
    server_label: &(impl TheServersName + ?Sized),
    job_label: &str,
    layer: Option<WhichPictureItIs>,
    of: usize,
    chosen: Vec<ADerivative>,
) -> Option<Receiver<HowASendEnded>> {
    if chosen.is_empty() {
        return None;
    }
    let chosen = within_the_albums_weight(chosen.into_iter().take(THE_MOST_PICTURES_IN_ONE_MESSAGE).collect());
    let caption = the_contact_sheets_caption(server_label, job_label, layer.as_ref(), chosen.len(), of);
    let captions: Vec<Option<AMarkedText>> = chosen
        .iter()
        .enumerate()
        .map(|(at, one)| match (at, layer.as_ref()) {
            (0, _) => Some(caption.clone()),
            (_, Some(layer)) => Some(a_named_sheets_picture_caption(one.frame, layer)),
            (_, None) => None,
        })
        .collect();
    Some(queue.offer_an_upload(
        Box::new(move |c: &TheClient| match chosen.as_slice() {
            [one] => c.send_photo_marked(chat, &one.file_name, one.bytes.clone(), Some(&caption), None),
            many => c.send_a_marked_album(
                chat,
                many.iter()
                    .zip(captions.iter())
                    .map(|(d, caption)| (d.file_name.clone(), d.bytes.clone(), caption.clone()))
                    .collect(),
            ),
        }),
    ))
}

/// **`Telegram` M10 — the finished video's caption**: the server's line, the
/// job's name and the file's own name, as the drawing's *A finished video,
/// sent* (`tg-video`) writes it — *Kitchen — final · Kitchen_0001-0240.mp4*.
pub fn the_finished_videos_caption(server_label: &(impl TheServersName + ?Sized), job_label: &str, file_name: &str) -> AMarkedText {
    a_user_message(server_label, job_label, &format!(" · {file_name}"), "")
}

/// **`Telegram` M10 — a finished video too big to send, said on its own**,
/// where the job's finishing message did not say it: a job in a group, which
/// has no message of its own (G1), or a job that writes pictures as well,
/// whose finishing message counts frames.
pub fn the_finished_video_is_too_big(server_label: &(impl TheServersName + ?Sized), job_label: &str, file_name: &str, bytes: u64) -> AMarkedText {
    a_user_message(server_label, job_label, "", &format!(
        "{file_name} is on the server ({}), too large to send.",
        crate::job_messages::a_size_in_words(bytes)
    ))
}

/// **`Telegram` M10 — the finished video, sent at the job's end** as one
/// video message, in C6b's upload lane, with no buttons: an ending carries no
/// Stop (C1b). **One message per file**; the caller offers the next only once
/// this one has ended, so a job's files arrive in order and none twice.
/// Refusals remain the caller's to record, as the contact sheet's are.
pub fn offer_the_finished_video(
    queue: &TheSendQueue,
    chat: i64,
    caption: AMarkedText,
    file_name: String,
    bytes: Vec<u8>,
) -> Receiver<HowASendEnded> {
    queue.offer_an_upload(Box::new(move |c: &TheClient| {
        c.send_video_marked(chat, &file_name, bytes.clone(), Some(&caption), None)
    }))
}

/// **`Telegram` M3 — the moving picture is an MP4: the MPEG-4 container and
/// the H.264 codec**, produced by the Blender the manifest names, headless, in
/// the job's own view transform. *Act owed*: step 8b drives it. That the route
/// honours the view transform is not observed (§21 item 7).
pub const THE_MOVING_PICTURES_CONTAINER: &str = "MPEG-4";
pub const THE_MOVING_PICTURES_CODEC: &str = "H.264";

/// **`Telegram` M4 — a stretch's length where the job's reporting settings
/// carry none.** The number is Build's; the drawing's moving picture covers
/// *frames 101 to 140*, which is forty.
pub const THE_DEFAULT_STRETCH: usize = 40;

/// **`Telegram` M4 — when the moving picture fires.** A stretch is a run of
/// consecutive finished frames in the job's asked order; one fires when a
/// stretch completes, and at the job's ending for the frames since the last one.
/// The contact sheet fires at the ending ([`the_contact_sheet`]).
///
/// **Held in memory**: a restart may fire a stretch once more, never lose one —
/// F3's at-least-once shape.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TheStretches {
    length: usize,
    through: usize,
}

impl TheStretches {
    pub fn of(length: usize) -> TheStretches {
        TheStretches { length: length.max(1), through: 0 }
    }

    /// **Every stretch that has completed since the last**, in order: the
    /// next `length` frames of the asked order, all finished.
    pub fn what_fires(&mut self, asked_order: &[u64], finished: &dyn Fn(u64) -> bool) -> Vec<Vec<u64>> {
        let mut fired = Vec::new();
        while self.through + self.length <= asked_order.len() {
            let stretch = &asked_order[self.through..self.through + self.length];
            if !stretch.iter().all(|f| finished(*f)) {
                break;
            }
            fired.push(stretch.to_vec());
            self.through += self.length;
        }
        fired
    }

    /// **At the job's ending**: the finished frames since the last stretch,
    /// where there are any.
    pub fn at_the_ending(&mut self, asked_order: &[u64], finished: &dyn Fn(u64) -> bool) -> Option<Vec<u64>> {
        let rest: Vec<u64> = asked_order.iter().skip(self.through).copied().filter(|f| finished(*f)).collect();
        self.through = asked_order.len();
        (!rest.is_empty()).then_some(rest)
    }
}

/// **`Telegram` M4, M4a and M4b — at most one media production per machine,
/// and a further completed stretch queues at the admission point rather than
/// being refused.** *Kept at* `App/Rented-server runtime/src/arbitration.rs`
/// (`excludes_its_own_kind`, `its_document_states_that_a_further_operation_queues`,
/// and `a_second_media_production_waits_rather_than_being_refused`), and not
/// written a second time here.
pub const AT_MOST_ONE_MEDIA_PRODUCTION_RUNS_PER_MACHINE_AND_A_FURTHER_ONE_QUEUES: &str =
    "App/Rented-server runtime/src/arbitration.rs";

/// **`Telegram` M5 — the media worker's declarations against `Rented-server
/// runtime` A1.** Stated here as values because that crate is out of this one's
/// reach (the guard, `tests/the_crates_shape_holds.rs`), and `arbitration.rs`
/// says no code carries this kind's declaration yet. *Owed*: step 8b hands
/// these to the admission point.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TheMediaKindsDeclaration {
    pub processors_shared: bool,
    pub memory_shared: bool,
    /// Whether the tool it drives touches a card at all is unobserved (§21
    /// item 7).
    pub a_card_taken_exclusively: bool,
    /// Ranked last, below the preview kind.
    pub ranked_last: bool,
    pub cancellable_at_any_moment: bool,
    /// Staged and published by rename into the job record directory.
    pub published_by_rename: bool,
    /// **M4a**: machine-level exclusion of its own kind, bound one.
    pub excludes_its_own_kind: bool,
    /// **M4b**: a further production queues.
    pub a_further_one_queues: bool,
}

pub const THE_MEDIA_KINDS_DECLARATION: TheMediaKindsDeclaration = TheMediaKindsDeclaration {
    processors_shared: true,
    memory_shared: true,
    a_card_taken_exclusively: false,
    ranked_last: true,
    cancellable_at_any_moment: true,
    published_by_rename: true,
    excludes_its_own_kind: true,
    a_further_one_queues: true,
};

/// **`Telegram` M6 — the termination contract, `Rented-server runtime` X11's four things.** Delivered
/// to the recorded Blender it drives; the worker reaps it and exits, so it
/// outlives its group; the staged production path is swept, so nothing partial
/// is published; a displaced production is cancelled, never a failure of what
/// displaced it. *Act owed*: step 8b.
pub const THE_MEDIA_WORKERS_TERMINATION: [&str; 4] = [
    "delivered to the recorded Blender process, at any moment",
    "the worker reaps it and exits, outliving every member of its group",
    "the staged production path is swept, so nothing partial is published",
    "a displaced production's outcome is cancelled, never a failure",
];

/// **`Telegram` M7 — media material lives in the job record directory, is
/// never fetched, and ends with the job's machine-side record** — never by an
/// age, a cap, a sweep, a schedule or a free-space reading. *Act owed*: step 8b.
pub const MEDIA_MATERIAL_ENDS_WITH_THE_JOBS_MACHINE_SIDE_RECORD: bool = true;

// **`Telegram` M8 — the trim target is 50 MB, Telegram's published figure,
// and it is [`THE_CAP_ON_A_FILE`]**: the one cap, in one place, which the
// words file reads too. It was read here as 50 mebibytes until the round-1 code
// audit of the bot and the chat, fix-here item 14: a moving picture between the
// two readings would be sent and refused, and M8 forbids a second trim.
// Whether Telegram means the smaller is `Telegram` §21 item 32.
use crate::job_messages::THE_CAP_ON_A_FILE;

/// **`Telegram` M8 — a production over the cap is trimmed to the stretch's
/// most recent frames, never refused.** Given what the last production
/// weighed, the frames to produce next: the most recent share that the cap
/// allows, and always at least one fewer than before, so the trimming ends.
/// Gives back nothing where the production already fits.
pub fn the_frames_to_keep(frames: &[u64], produced_bytes: u64) -> Option<Vec<u64>> {
    if produced_bytes <= THE_CAP_ON_A_FILE || frames.len() <= 1 {
        return None;
    }
    let share = (frames.len() as u128 * u128::from(THE_CAP_ON_A_FILE) / u128::from(produced_bytes)) as usize;
    let keep = share.clamp(1, frames.len() - 1);
    Some(frames[frames.len() - keep..].to_vec())
}

/// **`Telegram` M8 — the message says what range it covers**: the design's
/// *"Kitchen — final · frames 101 to 140"*.
pub fn the_moving_pictures_caption(server_label: &(impl TheServersName + ?Sized), job_label: &str, frames: &[u64]) -> AMarkedText {
    match frames {
        [] => a_user_message(server_label, job_label, "", ""),
        [one] => a_user_message(server_label, job_label, &format!(" · Frame {one}"), ""),
        [first, .., last] => a_user_message(server_label, job_label, &format!(" · frames {first}–{last}"), ""),
    }
}

/// **Offer a produced moving picture**, in C6b's upload lane, with C1b's
/// durable Stop context recorded before markup is attached. No nested queue
/// wait runs from the upload closure. Refusals remain the caller's to report.
pub fn offer_the_moving_picture(
    mut messages: crate::actions::TheJobMessages,
    subject: crate::actions::TheSubject,
    server: AServerLabel,
    label: String,
    caption: AMarkedText,
    file_name: String,
    bytes: Vec<u8>,
) -> Receiver<HowASendEnded> {
    let queue = messages.queue.clone();
    queue.offer_an_upload(Box::new(move |client| {
        let result = client.send_video_marked(
            messages.conversation,
            &file_name,
            bytes.clone(),
            Some(&caption),
            None,
        )?;
        if let Some(id) = result.get("message_id").and_then(serde_json::Value::as_i64) {
            if let Some(button) = crate::actions::a_stop_button_on(
                messages.store.as_mut(),
                messages.conversation,
                id,
                &subject,
                &server,
                &label,
            ) {
                client.edit_message_reply_markup(
                    messages.conversation,
                    id,
                    serde_json::json!({"inline_keyboard":[[button]]}),
                )?;
            }
        }
        Ok(result)
    }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pacing::OnThisComputersClock;
    use crate::transport::a_stand_in::AStandInService;
    use crate::transport::WhatACallCarries;
    use std::sync::Arc;

    /// **`Telegram` M10: the finished video goes as one `sendVideo` in the
    /// upload lane, captioned with the server, the job and the file's name,
    /// carrying the file whole, with no Stop and nothing after it.** The
    /// nineteenth notes, note 4.
    #[test]
    fn video_wants_a_finished_video_is_one_upload_with_its_caption_and_no_stop() {
        let service = Arc::new(AStandInService::new());
        service.then_answers(r#"{"ok":true,"result":{"message_id":71}}"#);
        let queue = Arc::new(TheSendQueue::new(
            TheClient::over(service.clone()),
            Arc::new(OnThisComputersClock),
        ));
        let caption = the_finished_videos_caption("RunPod 4090", "Kitchen \u{2014} final", "Kitchen_0001-0240.mp4");
        assert_eq!(caption, "RunPod 4090\nKitchen \u{2014} final \u{b7} Kitchen_0001-0240.mp4");
        let told = offer_the_finished_video(&queue, 5, caption.clone(), "Kitchen_0001-0240.mp4".into(), vec![9; 4]);
        assert_eq!(queue.how_many_uploads_are_waiting(), 1);
        assert!(queue.send_the_next_upload());
        assert!(matches!(told.recv().unwrap(), HowASendEnded::Accepted(_)));
        let heard = service.heard();
        assert_eq!(heard.iter().map(|call| call.method.as_str()).collect::<Vec<_>>(), ["sendVideo"]);
        let WhatACallCarries::FieldsAndFiles { fields, files } = &heard[0].carries else {
            panic!()
        };
        assert_eq!(fields["caption"], caption.as_str());
        assert!(fields.get("reply_markup").is_none(), "an ending carries no Stop");
        assert_eq!(files[0].bytes, [9, 9, 9, 9]);
        assert_eq!(files[0].file_name, "Kitchen_0001-0240.mp4");
        assert_eq!(
            the_finished_video_is_too_big("RunPod 4090", "Kitchen", "a.mp4", 1_400_000_000),
            "RunPod 4090\nKitchen\na.mp4 is on the server (1.4 GB), too large to send."
        );
    }

    #[test]
    fn piece17_a_video_upload_records_stop_before_attaching_it() {
        use crate::worker_state::{InMemory, TheTelegramWorkersState};
        let service = Arc::new(AStandInService::new());
        service.then_answers(r#"{"ok":true,"result":{"message_id":70}}"#);
        let queue = Arc::new(TheSendQueue::new(
            TheClient::over(service.clone()),
            Arc::new(OnThisComputersClock),
        ));
        let store: Arc<std::sync::Mutex<Box<dyn TheTelegramWorkersState>>> =
            Arc::new(std::sync::Mutex::new(Box::new(InMemory::with_routing_key(
                "00112233445566778899aabbccddeeff",
            ))));
        let messages = crate::actions::TheJobMessages {
            conversation: 5,
            store: Box::new(store.clone()),
            queue: queue.clone(),
            notes: Box::new(|_| {}),
        };
        let subject = crate::actions::TheSubject {
            owning_machine: "m".into(),
            job: Some("job".into()),
            hold: None,
            generation: "op:1".into(),
        };
        let told = offer_the_moving_picture(
            messages,
            subject,
            "Server".into(),
            "Kitchen".into(),
            "Server\nKitchen · frames 1 to 3".into(),
            "clip.mp4".into(),
            vec![1, 2, 3],
        );
        assert_eq!(queue.how_many_uploads_are_waiting(), 1);
        assert!(queue.send_the_next_upload());
        assert!(matches!(told.recv().unwrap(), HowASendEnded::Accepted(_)));
        let records = store.the_action_contexts_bound_to(70).unwrap();
        assert_eq!(records.len(), 1);
        assert_eq!(records[0].action, crate::job_messages::ASK_TO_STOP_THIS_JOB);
        assert_eq!(records[0].job.as_deref(), Some("job"));
        assert_eq!(records[0].generation, "op:1");
        let heard = service.heard();
        assert_eq!(
            heard
                .iter()
                .map(|call| call.method.as_str())
                .collect::<Vec<_>>(),
            ["sendVideo", "editMessageReplyMarkup"]
        );
        let WhatACallCarries::FieldsAndFiles { fields, files } = &heard[0].carries else {
            panic!()
        };
        assert!(fields.get("reply_markup").is_none());
        assert_eq!(files[0].bytes, [1, 2, 3]);
    }

    /// **`Telegram` M2 and the design — first, last, eight evenly between.**
    #[test]
    fn the_contact_sheet_is_ten_in_frame_order_first_and_last_included() {
        let frames: Vec<u64> = (1..=240).collect();
        let sheet = the_contact_sheet(&frames);
        assert_eq!(sheet.len(), 10);
        assert_eq!(sheet.first(), Some(&1));
        assert_eq!(sheet.last(), Some(&240));
        assert!(sheet.windows(2).all(|w| w[0] < w[1]));
        assert_eq!(the_contact_sheet(&[3, 5, 9]), [3, 5, 9]);
        let eleven: Vec<u64> = (1..=11).collect();
        assert_eq!(the_contact_sheet(&eleven).len(), 10);
        assert_eq!(the_contact_sheets_caption("S", "J", None, 10, 240), "S\nJ \u{b7} 10 of 240 frames, evenly spread");
        assert_eq!(the_contact_sheets_caption("S", "J", None, 3, 3), "S\nJ \u{b7} all 3 frames");
    }

    fn a_derivative(frame: u64) -> ADerivative {
        ADerivative {
            job: "j".into(),
            job_label: "J".into(),
            frame,
            file_name: format!("{frame}.png"),
            bytes: crate::pictures::tests::a_png_header(320, 180),
            which_picture: None,
        }
    }

    /// **M2 and R7 — one album, its caption on the first picture only.**
    #[test]
    fn the_contact_sheet_goes_as_one_album_with_its_caption_on_the_first_picture() {
        let service = Arc::new(AStandInService::new());
        let queue = TheSendQueue::new(TheClient::over(service.clone()), Arc::new(OnThisComputersClock));
        offer_the_contact_sheet(&queue, 5, "S", "J", None, 240, (1..=10).map(a_derivative).collect()).unwrap();
        offer_the_contact_sheet(&queue, 5, "S", "J", None, 1, vec![a_derivative(1)]).unwrap();
        assert!(offer_the_contact_sheet(&queue, 5, "S", "J", None, 0, vec![]).is_none());
        // C6b: both wait in the upload lane, and nothing in the serial lane.
        assert_eq!((queue.how_many_uploads_are_waiting(), queue.how_many_are_waiting()), (2, 0));
        while queue.send_the_next_upload() {}
        let heard = service.heard();
        assert_eq!(heard[0].method, "sendMediaGroup");
        let WhatACallCarries::FieldsAndFiles { fields, files } = &heard[0].carries else { panic!() };
        assert_eq!(files.len(), 10);
        let media = fields["media"].as_array().unwrap();
        assert_eq!(media[0]["caption"], "S\nJ \u{b7} 10 of 240 frames, evenly spread");
        assert!(media[1..].iter().all(|m| m.get("caption").is_none()));
        assert_eq!(heard[1].method, "sendPhoto");
    }

    /// **`Telegram` M2 as changed 2026-10-31 (the seventeenth notes, note 10):
    /// a sheet of one View Layer among several names its layer** — in its
    /// line, *Living room — final · Reflections · 10 of 240 frames, evenly
    /// spread*, as the drawing's `tg-layers` writes it, and in every other
    /// picture's caption, as `Telegram` I2's album does. One lone picture goes
    /// as a photo carrying the named line.
    #[test]
    fn seventeenth_10_a_layers_sheet_names_its_layer_in_its_line_and_every_caption() {
        let layer = WhichPictureItIs::AViewLayer("Reflections".into());
        assert_eq!(
            the_contact_sheets_caption("S", "Living room \u{2014} final", Some(&layer), 10, 240),
            "S\nLiving room \u{2014} final \u{b7} Reflections \u{b7} 10 of 240 frames, evenly spread"
        );
        assert_eq!(the_contact_sheets_caption("S", "J", Some(&layer), 2, 2), "S\nJ \u{b7} Reflections \u{b7} all 2 frames");
        let service = Arc::new(AStandInService::new());
        let queue = TheSendQueue::new(TheClient::over(service.clone()), Arc::new(OnThisComputersClock));
        let nine: Vec<ADerivative> = [1, 28, 55, 81, 108, 134, 161, 187, 240].into_iter().map(a_derivative).collect();
        offer_the_contact_sheet(&queue, 5, "S", "J", Some(layer.clone()), 240, nine).unwrap();
        offer_the_contact_sheet(&queue, 5, "S", "J", Some(layer), 1, vec![a_derivative(1)]).unwrap();
        while queue.send_the_next_upload() {}
        let heard = service.heard();
        assert_eq!(heard[0].method, "sendMediaGroup");
        let WhatACallCarries::FieldsAndFiles { fields, files } = &heard[0].carries else { panic!() };
        assert_eq!(files.len(), 9, "a layer missing at one chosen frame leaves nine");
        let media = fields["media"].as_array().unwrap();
        assert_eq!(media[0]["caption"], "S\nJ \u{b7} Reflections \u{b7} 9 of 240 frames, evenly spread");
        assert_eq!(media[1]["caption"], "Frame 28 \u{b7} Reflections \u{b7} Not colour-accurate");
        assert_eq!(media[8]["caption"], "Frame 240 \u{b7} Reflections \u{b7} Not colour-accurate");
        assert_eq!(heard[1].method, "sendPhoto");
        let WhatACallCarries::FieldsAndFiles { fields, .. } = &heard[1].carries else { panic!() };
        assert_eq!(fields["caption"], "S\nJ \u{b7} Reflections \u{b7} all 1 frame");
    }

    fn a_heavy_derivative(frame: u64, bytes: usize) -> ADerivative {
        let mut one = a_derivative(frame);
        one.bytes = crate::pictures::tests::a_png_header(3840, 2160);
        one.bytes.resize(bytes, 0);
        one
    }

    /// **The live Telegram pass, fault 3 (`Telegram` I3a): a sheet of eight
    /// 3840 × 2160 pictures, refused whole as too large, now goes as fewer,
    /// evenly spread, under one album's weight**, first and last kept, its line
    /// counting what is shown. A sheet that fits is untouched; where even two
    /// are too heavy, one goes as a photo.
    #[test]
    fn live_pass_3_a_heavy_sheet_sends_fewer_evenly_spread_under_one_albums_weight() {
        let service = Arc::new(AStandInService::new());
        let queue = TheSendQueue::new(TheClient::over(service.clone()), Arc::new(OnThisComputersClock));
        let eight: Vec<ADerivative> = (1..=8).map(|frame| a_heavy_derivative(frame, 4_500_000)).collect();
        offer_the_contact_sheet(&queue, 5, "S", "J", None, 8, eight).unwrap();
        let light: Vec<ADerivative> = (1..=10).map(|frame| a_heavy_derivative(frame, 1_000_000)).collect();
        offer_the_contact_sheet(&queue, 5, "S", "J", None, 240, light).unwrap();
        let three: Vec<ADerivative> = (1..=3).map(|frame| a_heavy_derivative(frame, crate::pictures::THE_MOST_A_PICTURE_MAY_WEIGH)).collect();
        offer_the_contact_sheet(&queue, 5, "S", "J", None, 3, three).unwrap();
        while queue.send_the_next_upload() {}
        let heard = service.heard();
        assert_eq!(heard.iter().map(|call| call.method.as_str()).collect::<Vec<_>>(), ["sendMediaGroup", "sendMediaGroup", "sendPhoto"]);
        let WhatACallCarries::FieldsAndFiles { fields, files } = &heard[0].carries else { panic!() };
        assert_eq!(files.len(), 4);
        assert!(files.iter().map(|file| file.bytes.len()).sum::<usize>() <= THE_MOST_AN_ALBUM_MAY_WEIGH);
        assert_eq!(files.iter().map(|file| file.file_name.as_str()).collect::<Vec<_>>(), ["1.png", "3.png", "6.png", "8.png"]);
        assert_eq!(fields["media"][0]["caption"], "S\nJ \u{b7} 4 of 8 frames, evenly spread");
        let WhatACallCarries::FieldsAndFiles { files, .. } = &heard[1].carries else { panic!() };
        assert_eq!(files.len(), 10, "ten full-HD pictures still go whole");
        let WhatACallCarries::FieldsAndFiles { fields, .. } = &heard[2].carries else { panic!() };
        assert_eq!(fields["caption"], "S\nJ \u{b7} 1 of 3 frames, evenly spread");
        assert_eq!(evenly_spread(8, 4), [0, 2, 5, 7]);
        assert_eq!(evenly_spread(3, 5), [0, 1, 2]);
        assert_eq!(evenly_spread(9, 1), [0]);
    }

    /// **`Telegram` M4 — a stretch fires when it completes, and the ending
    /// fires the rest.**
    #[test]
    fn a_stretch_fires_when_it_completes_and_the_ending_fires_the_rest() {
        let asked: Vec<u64> = (1..=10).collect();
        let mut stretches = TheStretches::of(4);
        assert!(stretches.what_fires(&asked, &|f| f <= 3).is_empty());
        assert_eq!(stretches.what_fires(&asked, &|f| f <= 9), [vec![1, 2, 3, 4], vec![5, 6, 7, 8]]);
        assert!(stretches.what_fires(&asked, &|f| f <= 9).is_empty());
        assert_eq!(stretches.at_the_ending(&asked, &|f| f <= 9), Some(vec![9]));
        assert_eq!(stretches.at_the_ending(&asked, &|_| true), None);
    }

    /// **`Telegram` M8 — trimmed to the most recent frames, never refused, and
    /// the trimming always ends.**
    #[test]
    fn an_over_cap_production_keeps_the_most_recent_frames() {
        let stretch: Vec<u64> = (101..=140).collect();
        assert_eq!(the_frames_to_keep(&stretch, THE_CAP_ON_A_FILE), None);
        let kept = the_frames_to_keep(&stretch, THE_CAP_ON_A_FILE * 2).unwrap();
        assert_eq!(kept, (121..=140).collect::<Vec<_>>());
        let barely = the_frames_to_keep(&stretch, THE_CAP_ON_A_FILE + 1).unwrap();
        assert_eq!(barely.len(), 39);
        assert_eq!(the_frames_to_keep(&[7], u64::MAX), None);
        // **One cap, at the smaller reading (§21 item 32)**: a production one
        // byte over 50,000,000 is trimmed, which the mebibyte reading did not.
        assert_eq!(THE_CAP_ON_A_FILE, 50_000_000);
        assert!(the_frames_to_keep(&stretch, 50_000_001).is_some());
        assert_eq!(the_moving_pictures_caption("S", "J", &kept), "S\nJ · frames 121–140");
    }

    /// **`Telegram` M4a, M4b and M5 — the declaration, bound one, queuing.**
    #[test]
    fn the_media_kind_declares_one_at_a_time_and_queues() {
        let d = THE_MEDIA_KINDS_DECLARATION;
        assert!(d.excludes_its_own_kind && d.a_further_one_queues && !d.a_card_taken_exclusively);
        assert!(std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("..")
            .join(AT_MOST_ONE_MEDIA_PRODUCTION_RUNS_PER_MACHINE_AND_A_FURTHER_ONE_QUEUES.trim_start_matches("App/"))
            .exists());
    }
}
