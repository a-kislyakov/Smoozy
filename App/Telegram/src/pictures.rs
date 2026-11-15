//! **Pictures of frames — `Telegram` §11, I1 to I5.** Rules over values.
//!
//! **I1 — the bot watches no folder, reads no frame file, and judges no frame
//! finished.** Nothing in this file lists a folder or opens a render's output.
//! What it sends is handed in as [`ADerivative`]: a still copy the preview
//! worker already produced and published by rename (`Job files and outputs`
//! B22, B29).
//!
//! **I2 — a picture exists to send exactly when the derivative set holds it,
//! and the worker learns that from the journal.** *Rule kept*: the only input
//! is a derivative the set holds. *Act owed*: reading the journal forward from
//! its cursor and reading the set on a real machine, step 8b (the step plan's
//! §8 items 4 and 6).
//!
//! **I3 — the limits, in Telegram's own terms**: at most 10,485,760 bytes,
//! width plus height at most 10,000, and the ratio at most 20. **A picture
//! outside them is not sent**, and a refusal is said **once per job**, not once
//! per frame.
//!
//! **I4 — *send a frame* is answered from the set**, and a cleared derivative
//! is produced again through the same route — [`answer_send_a_frame`].
//!
//! **I2 as written 2026-10-18 (the Creator's fourteenth notes, note 7): a
//! frame of more than one copy is sent as one album**, every copy in its place
//! in the frame's set, each picture captioned with its own View Layer or
//! source; more than ten go as albums of ten, a last lone copy as one picture;
//! a frame of one copy is one picture, as before —
//! [`ThePictures::send_a_frame_with_its_stop`] and [`send_a_frames_copies`].
//!
//! **I5 — the negative cases are answered from records**, never guessed.

use crate::job_messages::{AServerLabel, TheServersName};
use crate::job_messages::{AMarkedText, a_user_message};
use std::collections::BTreeSet;
use std::sync::mpsc::Receiver;

use serde_json::Value;

use crate::actions::{accepted_id, wait_for, TheJobMessages, TheSubject, WhyAMessageWasNotSent};
use crate::media::{THE_MOST_AN_ALBUM_MAY_WEIGH, THE_MOST_PICTURES_IN_ONE_MESSAGE};
use crate::pacing::{AClassOfSend, HowASendEnded, TheSendQueue};
use crate::transport::{TheClient, WhyTheCallFailed};

/// **`Telegram` I3 — a photo's weight cap, exactly.** Observed, and
/// published as *"at most 10 MB"*.
pub const THE_MOST_A_PICTURE_MAY_WEIGH: usize = 10_485_760;
/// **`Telegram` I3** — *"The photo's width and height must not exceed 10000 in
/// total."*
pub const THE_MOST_WIDTH_AND_HEIGHT_TOGETHER: u64 = 10_000;
/// **`Telegram` I3** — *"Width and height ratio must be at most 20."*
pub const THE_MOST_RATIO: u64 = 20;

/// **One still copy from the derivative set** (`Job files and outputs` B22).
#[derive(Clone, PartialEq)]
pub struct ADerivative {
    /// The job's identity; what "once per job" counts by.
    pub job: String,
    /// What the job is called (`Telegram` R7, `Telegram` R7a): the label
    /// whole, or the `.blend`'s name where there is none.
    pub job_label: String,
    /// **The frame's own number, below zero too** (`Specs/Render Settings`:
    /// *"Nothing in Smoozy assumes the first frame is frame 1"*; Sol's S2, the
    /// four questions, batch 4: it was a number of zero or more, and a frame
    /// below zero was sent as frame 0).
    pub frame: i64,
    pub file_name: String,
    pub bytes: Vec<u8>,
    /// **Which of the frame's pictures this is**, where the frame has more
    /// than one View Layer (`Telegram` I2, added 2026-10-17, the Creator's
    /// tenth notes, note 2): its View Layer, or its source where it is no one
    /// layer's. Nothing for a frame of one View Layer, and nothing where it was
    /// not handed in.
    pub which_picture: Option<WhichPictureItIs>,
    /// **`Telegram` I2 as changed 2026-11-11 (the four questions, batch 4,
    /// Sol's S6): the View Layers of this frame whose copy could not be read
    /// and was left out**, by name, an empty name for one its record does not
    /// name. Only on the main copy; the caption of the first picture sent
    /// names them ([`the_layers_not_read`]).
    pub not_read: Vec<String>,
}

/// **`Telegram` I2 as changed 2026-11-11 — the line naming the View Layers
/// whose preview could not be read**: *Could not read the preview of
/// Reflections.*, several by commas, and those their records do not name
/// counted, *… of Reflections and 1 other View Layer.* Nothing where none.
pub fn the_layers_not_read(not_read: &[String]) -> Option<String> {
    let named: Vec<&str> = not_read.iter().map(String::as_str).filter(|name| !name.is_empty()).collect();
    let unnamed = not_read.len() - named.len();
    let others = match (named.is_empty(), unnamed) {
        (_, 0) => String::new(),
        (true, 1) => "1 View Layer".to_string(),
        (true, n) => format!("{n} View Layers"),
        (false, 1) => " and 1 other View Layer".to_string(),
        (false, n) => format!(" and {n} other View Layers"),
    };
    (!not_read.is_empty()).then(|| format!("Could not read the preview of {}{others}.", named.join(", ")))
}

/// **Which of a frame's pictures a copy is** — `Telegram` I2's caption names
/// it. Handed in from what `Queue & History` P3's event carries: the View
/// Layer's name, or the source `Job files and outputs` B31a records for a
/// main copy that is no one layer's. The names are the artist's own, said as
/// they read.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WhichPictureItIs {
    AViewLayer(String),
    /// The regular output path's file.
    TheOutputPath,
    /// A File Output node, by its name, fed by more than one View Layer.
    AFileOutputNode(String),
    /// The recorded source filename when a first-frame copy has no layer metadata.
    ASourceFile(String),
}

/// **The source's words**, the desktop's own (`Design/Queue & History`, the
/// preview's dropdown): *Output path*, and *File Output node* with the node's
/// name after it.
pub const OUTPUT_PATH: &str = "Output path";
pub const FILE_OUTPUT_NODE: &str = "File Output node";

impl WhichPictureItIs {
    /// **What the caption says of it**: the View Layer's name, *Output
    /// path*, or *File Output node Beauty*.
    pub fn as_said(&self) -> String {
        match self {
            WhichPictureItIs::AViewLayer(name) => name.clone(),
            WhichPictureItIs::TheOutputPath => OUTPUT_PATH.to_string(),
            WhichPictureItIs::AFileOutputNode(name) => format!("{FILE_OUTPUT_NODE} {name}"),
            WhichPictureItIs::ASourceFile(name) => format!("Source: {name}"),
        }
    }
}

impl std::fmt::Debug for ADerivative {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ADerivative")
            .field("job", &self.job)
            .field("frame", &self.frame)
            .field("file_name", &self.file_name)
            .field("bytes", &self.bytes.len())
            .field("which_picture", &self.which_picture)
            .finish()
    }
}

/// **The width and height a PNG or JPEG says it has**, read from its header.
/// Those are the two viewing formats a derivative is written in (`Job files
/// and outputs` B12, B28). Anything else gives nothing.
pub fn the_dimensions_of(bytes: &[u8]) -> Option<(u64, u64)> {
    const PNG: [u8; 8] = [0x89, b'P', b'N', b'G', 0x0d, 0x0a, 0x1a, 0x0a];
    if bytes.len() >= 24 && bytes[..8] == PNG && &bytes[12..16] == b"IHDR" {
        let width = u32::from_be_bytes(bytes[16..20].try_into().ok()?);
        let height = u32::from_be_bytes(bytes[20..24].try_into().ok()?);
        return Some((u64::from(width), u64::from(height)));
    }
    if bytes.len() < 4 || bytes[0] != 0xff || bytes[1] != 0xd8 {
        return None;
    }
    let mut at = 2;
    while at + 4 <= bytes.len() {
        if bytes[at] != 0xff {
            return None;
        }
        let marker = bytes[at + 1];
        if marker == 0xff {
            at += 1;
            continue;
        }
        if marker == 0xd8 || marker == 0x01 || (0xd0..=0xd7).contains(&marker) {
            at += 2;
            continue;
        }
        let length = usize::from(u16::from_be_bytes([bytes[at + 2], bytes[at + 3]]));
        let is_a_frame_header = (0xc0..=0xcf).contains(&marker) && ![0xc4, 0xc8, 0xcc].contains(&marker);
        if is_a_frame_header {
            let header = bytes.get(at + 5..at + 9)?;
            let height = u16::from_be_bytes([header[0], header[1]]);
            let width = u16::from_be_bytes([header[2], header[3]]);
            return Some((u64::from(width), u64::from(height)));
        }
        if length < 2 {
            return None;
        }
        at += 2 + length;
    }
    None
}

/// **`Telegram` I3 — whether a picture sits under all three caps.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhetherItFits {
    Fits,
    /// Over the weight, or the dimensions, or the ratio.
    OutsideTheLimits,
    /// Neither format's header could be read; **Telegram decides**, and a
    /// refusal is then said as its own (I3's *"a send Telegram nevertheless
    /// refuses"*).
    TheDimensionsCouldNotBeRead,
}

pub fn whether_it_fits(bytes: &[u8]) -> WhetherItFits {
    if bytes.len() > THE_MOST_A_PICTURE_MAY_WEIGH {
        return WhetherItFits::OutsideTheLimits;
    }
    match the_dimensions_of(bytes) {
        None => WhetherItFits::TheDimensionsCouldNotBeRead,
        Some((width, height)) => {
            let (short, long) = (width.min(height), width.max(height));
            if width + height > THE_MOST_WIDTH_AND_HEIGHT_TOGETHER || short == 0 || long > THE_MOST_RATIO * short {
                WhetherItFits::OutsideTheLimits
            } else {
                WhetherItFits::Fits
            }
        }
    }
}

/// **A picture's caption** — the design's, `#tgview`: the server, then the
/// job label, the frame, and *not colour-accurate* (`Preparation/Design/Queue &
/// History.md`: *"it is on every preview"*). **Where the frame has more than
/// one View Layer, which picture it is sits between the frame and *not
/// colour-accurate*** (`Telegram` I2, added 2026-10-17, the Creator's tenth
/// notes, note 2): *Kitchen — final · frame 140 · Reflections · not
/// colour-accurate*. Ordinary single-layer frames name none; the first-frame
/// approval path can instead name the source filename its record retained.
pub fn the_caption(server_label: &(impl TheServersName + ?Sized), job_label: &str, frame: i64, which_picture: Option<&WhichPictureItIs>) -> AMarkedText {
    the_caption_saying(server_label, job_label, frame, which_picture, &[])
}

/// **[`the_caption`], with the line naming the View Layers whose preview could
/// not be read** (`Telegram` I2 as changed 2026-11-11), where there are any.
pub fn the_caption_saying(
    server_label: &(impl TheServersName + ?Sized),
    job_label: &str,
    frame: i64,
    which_picture: Option<&WhichPictureItIs>,
    not_read: &[String],
) -> AMarkedText {
    let which = which_picture.map(|which| format!(" · {}", which.as_said())).unwrap_or_default();
    let body = the_layers_not_read(not_read).unwrap_or_default();
    within_the_caption_bound(job_label, |label| a_user_message(server_label, label, &format!(" · Frame {frame}{which} · Not colour-accurate"), &body))
}

/// **Telegram's bound on a picture's caption: 1,024 characters after entity
/// parsing** — `InputMediaPhoto.caption`, *"0-1024 characters after entities
/// parsing"* (`Preparation/Architecture/Sources/Telegram.sources.md`, entry
/// 88, read 2026-10-18), counted here in UTF-16 units as the message bound is.
pub const THE_BOUND_ON_A_CAPTION: usize = 1024;

/// **A caption with a job name too long for Telegram's bound has the name cut
/// short with an ellipsis**, so the server, the frame and the layer are still
/// said and the picture still arrives (the UI at every size, C8). The job's
/// own message carries its name whole. `compose` makes the caption around a
/// label; a caption inside the bound is left exactly as it was.
pub fn within_the_caption_bound(job_label: &str, compose: impl Fn(&str) -> AMarkedText) -> AMarkedText {
    let whole = compose(job_label);
    let units = whole.text.encode_utf16().count();
    if units <= THE_BOUND_ON_A_CAPTION {
        return whole;
    }
    let room = job_label.encode_utf16().count().saturating_sub(units - THE_BOUND_ON_A_CAPTION + 1);
    let mut kept = String::new();
    let mut used = 0;
    for letter in job_label.chars() {
        if used + letter.len_utf16() > room { break; }
        used += letter.len_utf16();
        kept.push(letter);
    }
    kept.push('\u{2026}');
    compose(&kept)
}

/// **`Telegram` I3 — the one line for all three caps**: weight, width and height
/// together, and ratio. Until 2026-11-02 it said *dimensions* for all three. The
/// drawing has no such state; Rule 9 binds.
const OVER_THE_LIMITS: &str = "Preview not sent: over Telegram's limits.";

/// **`Telegram` I3 — what the user is told, once per job, where a picture is
/// outside Telegram's limits.**
pub fn outside_the_limits(server_label: &(impl TheServersName + ?Sized), job_label: &str, frame: i64) -> AMarkedText {
    a_user_message(server_label, job_label, &format!(" · Frame {frame}"), OVER_THE_LIMITS)
}

/// **`Telegram` I3 — a picture Telegram refused although it fit**, said once
/// per job and truthfully: the refusal's own text is not read for a reason.
pub fn telegram_did_not_accept(server_label: &(impl TheServersName + ?Sized), job_label: &str, frame: i64) -> AMarkedText {
    a_user_message(server_label, job_label, &format!(" · Frame {frame}"), "Telegram rejected the preview.")
}

/// **`Telegram` I3, in answer to *send a frame*** — the same refusal, said
/// every time it is asked, since an answer to a request is always sent
/// (`Telegram` V1's floor). Added 2026-10-16, the Telegram worker's piece 6;
/// the drawing has no such state, so Rule 9 binds.
pub fn a_frame_outside_the_limits(server_label: &(impl TheServersName + ?Sized), job_label: &str, frame: i64) -> AMarkedText {
    a_user_message(server_label, job_label, &format!(" · Frame {frame}"), OVER_THE_LIMITS)
}

/// **`Telegram` I4 — a cleared picture is being made again**, said in answer
/// to *send a frame*; the picture follows once the preview route has made it.
/// Added 2026-10-16, the Telegram worker's piece 6; Rule 9 binds.
pub fn a_frame_is_being_made_again(server_label: &(impl TheServersName + ?Sized), job_label: &str, frame: i64) -> AMarkedText {
    a_user_message(server_label, job_label, &format!(" · Frame {frame}"), "Regenerating the preview.")
}

/// **`Telegram` I4 — a picture being made again did not come back in time**,
/// said once where *being made again* was said and the picture was not ready
/// ten minutes later. Added 2026-10-16, owed piece 11, on the build runner's
/// ruling of that day; the drawing has no such state, so Rule 9 binds.
pub fn a_frames_picture_could_not_be_made(server_label: &(impl TheServersName + ?Sized), job_label: &str, frame: i64) -> AMarkedText {
    a_user_message(server_label, job_label, &format!(" · Frame {frame}"), "Could not generate the preview.")
}

/// **`Telegram` I5 — the records hold no picture of this frame**, made or
/// being made: a frame the job does not cover, or one that has nothing a
/// picture can be made from. Added 2026-10-16, the Telegram worker's piece 6;
/// Rule 9 binds.
pub fn there_is_no_picture_of_the_frame(server_label: &(impl TheServersName + ?Sized), job_label: &str, frame: i64) -> AMarkedText {
    a_user_message(server_label, job_label, &format!(" · Frame {frame}"), "No preview available.")
}

/// **`Telegram` I3's *once per job*, for one kind of telling.** Held in
/// memory: a restart may say it once more, never nothing — the shape G3's crash
/// window already accepts.
#[derive(Debug, Default, Clone)]
pub struct OnceForAJob(BTreeSet<String>);

impl OnceForAJob {
    /// True the first time for a job, and never again.
    pub fn first_time_for(&mut self, job: &str) -> bool {
        self.0.insert(job.to_string())
    }
}

/// **What became of one picture handed in.**
#[derive(Debug)]
pub enum WhatBecameOfAPicture {
    /// Offered to the queue; its ending arrives here and goes to
    /// [`ThePictures::take_in_how_it_ended`].
    Offered(Receiver<HowASendEnded>),
    /// Outside the limits and not sent; the user is told on this receiver.
    NotSentAndSaid(Receiver<HowASendEnded>),
    /// Outside the limits and not sent; this job's telling was already made.
    NotSentAlreadySaid,
}

/// **What became of a frame picture sent with its stop**
/// ([`ThePictures::send_with_its_stop`]).
#[derive(Debug, PartialEq)]
pub enum APictureSent {
    /// Accepted as `message`. `stop` is how attaching *Stop this job* went,
    /// where one was owed.
    Sent { message: i64, stop: Option<Result<(), WhyAMessageWasNotSent>> },
    /// Not accepted — refused, and said once for its job, or it reached
    /// nothing. [`the_work_is_done`] reads the ending (F3).
    NotAccepted(HowASendEnded),
    /// Outside the limits and not sent; the user was told.
    NotSentAndSaid,
    /// Outside the limits and not sent; this job's telling was already made.
    NotSentAlreadySaid,
}

/// **One machine's frame pictures**, through the one queue.
#[derive(Debug)]
pub struct ThePictures {
    chat: i64,
    server_label: AServerLabel,
    outside_the_limits: OnceForAJob,
    not_accepted: OnceForAJob,
    delivered: std::collections::BTreeMap<(String, i64, usize), i64>,
}

impl ThePictures {
    pub fn new(chat: i64, server_label: &(impl TheServersName + ?Sized)) -> ThePictures {
        ThePictures {
            chat,
            server_label: server_label.named(),
            outside_the_limits: OnceForAJob::default(),
            not_accepted: OnceForAJob::default(),
            delivered: std::collections::BTreeMap::new(),
        }
    }

    /// **The pictures sent in answer to a request.** Until the second words
    /// sweep (2026-11-01) these left *Not repeated for this job.* off a
    /// refusal; no refusal says it any more, so this is [`ThePictures::new`].
    pub fn for_a_request(chat: i64, server: &(impl TheServersName + ?Sized)) -> ThePictures {
        Self::new(chat, server)
    }

    /// **Offer one derivative.** `class` is `Telegram` C6's *frame pictures
    /// and media* for a picture on the journal, and *routed answers* for a
    /// picture sent in answer to *send a frame*, which is an answer (C6: *"a
    /// picture backlog never delays … an answer"*).
    ///
    /// **`Telegram` R1: a picture is sent with no buttons.** A button's record
    /// names its message, which exists only once this send is accepted; the
    /// drawing's *Stop this job* is attached afterwards, by
    /// `crate::actions::TheJobMessages::attach_a_stop`, with the call that
    /// changes only a message's buttons. Until the round-1 code audit of the
    /// bot and the chat (fix-here item 6) buttons were handed in here, on the
    /// send, where no record could name the message.
    pub fn offer(&mut self, queue: &TheSendQueue, derivative: ADerivative, class: AClassOfSend) -> WhatBecameOfAPicture {
        let chat = self.chat;
        if whether_it_fits(&derivative.bytes) == WhetherItFits::OutsideTheLimits {
            if !self.outside_the_limits.first_time_for(&derivative.job) {
                return WhatBecameOfAPicture::NotSentAlreadySaid;
            }
            let text = outside_the_limits(&self.server_label, &derivative.job_label, derivative.frame);
            return WhatBecameOfAPicture::NotSentAndSaid(queue.offer(
                AClassOfSend::FramePicturesAndMedia,
                Box::new(move |c: &TheClient| c.send_marked_message(chat, &text, None)),
            ));
        }
        let caption = the_caption_saying(
            &self.server_label,
            &derivative.job_label,
            derivative.frame,
            derivative.which_picture.as_ref(),
            &derivative.not_read,
        );
        WhatBecameOfAPicture::Offered(queue.offer(
            class,
            Box::new(move |c: &TheClient| {
                c.send_photo_marked(chat, &derivative.file_name, derivative.bytes.clone(), Some(&caption), None)
            }),
        ))
    }

    /// I4: one picture answered; no Stop rides it (C1b as changed 2026-11-09).
    pub fn send_with_its_stop(
        &mut self,
        messages: &mut TheJobMessages,
        derivative: ADerivative,
        class: AClassOfSend,
        stop: Option<&TheSubject>,
    ) -> APictureSent {
        self.send_a_frame_with_its_stop(messages, vec![derivative], class, stop)
    }

    /// **An answer to *send a frame*** (`Telegram` I4, R14): from 2026-11-09
    /// no picture carries *Stop this job* (C1b), and nothing follows an album.
    /// `stop` is kept for an answer's refusal, which is said with the job's
    /// Stop while the job is live, as before.
    pub fn send_a_frame_with_its_stop(
        &mut self,
        messages: &mut TheJobMessages,
        copies: Vec<ADerivative>,
        class: AClassOfSend,
        stop: Option<&TheSubject>,
    ) -> APictureSent {
        self.send_a_frame_as(messages, copies, class, AFramesWay { stop, ..AFramesWay::default() })
    }

    /// **`Telegram` I2 as changed 2026-11-09 — a frame's copies, in order, at
    /// most ten to an album, as `way` says.** A frame from the journal goes as
    /// **a reply to its job's message, with no Stop and nothing after it**
    /// (the twenty-eighth notes, note 19: *"One message, Stop on job"*); an
    /// answer carries none either. Accepted batches are remembered
    /// until the whole frame has gone, so a retry repeats only the unfinished
    /// part. This memory has the journal's at-least-once life.
    ///
    /// **I3's refusal, once per job**: a line on the job's message while the
    /// job is live and that message is recorded (C1d); otherwise a message of
    /// its own — without a sound after the ending (V1e) — or, for an answer,
    /// as before.
    pub fn send_a_frame_as(
        &mut self,
        messages: &mut TheJobMessages,
        copies: Vec<ADerivative>,
        class: AClassOfSend,
        way: AFramesWay<'_>,
    ) -> APictureSent {
        let Some(first) = copies.first() else {
            return APictureSent::NotSentAlreadySaid;
        };
        let (job, label, frame) = (first.job.clone(), first.job_label.clone(), first.frame);
        let (fits, outside): (Vec<_>, Vec<_>) = copies
            .into_iter()
            .partition(|copy| whether_it_fits(&copy.bytes) != WhetherItFits::OutsideTheLimits);
        let queue = messages.queue.clone();
        let mut said = false;
        if !outside.is_empty() && self.outside_the_limits.first_time_for(&job) {
            let ended = match (way.stop, way.on_the_jobs_message()) {
                (None, Some(keeper)) => {
                    keeper.a_last_line(&job, OVER_THE_LIMITS);
                    HowASendEnded::Accepted(Value::Null)
                }
                _ => self.a_refusal_said(messages, &way, outside_the_limits(&self.server_label, &label, frame), &label, class),
            };
            if matches!(
                ended,
                HowASendEnded::HandedBack(WhyTheCallFailed::NotReached(_))
            ) {
                self.outside_the_limits.0.remove(&job);
                return APictureSent::NotAccepted(ended);
            }
            said = true;
        }
        if fits.is_empty() {
            return if said {
                APictureSent::NotSentAndSaid
            } else {
                APictureSent::NotSentAlreadySaid
            };
        }
        let how = crate::transport::HowItIsSent { silent: false, reply_to: way.reply_to };
        let mut first_message = None;
        for (part, batch) in the_sends_of_a_frame(fits).into_iter().enumerate() {
            let key = (job.clone(), frame, part);
            let message = if let Some(id) = self.delivered.get(&key) {
                *id
            } else {
                match send_a_frames_copies_each_as(&queue, self.chat, &self.server_label, batch, class, how) {
                    Ok(sent) => {
                        // R1b: each picture is about its job and its frame.
                        for (message, copy) in &sent {
                            messages.register(
                                *message,
                                crate::worker_state::WhatAMessageIsAbout::AJob {
                                    job: copy.job.clone(),
                                    generation: way.stop.map(|subject| subject.generation.clone()),
                                    // A reply naming a frame below zero is
                                    // not carried (R1a's request takes a
                                    // frame of zero or more): it asks for the
                                    // newest instead.
                                    frame: Some(copy.frame),
                                    layer: copy.which_picture.as_ref().map(|which| which.as_said()),
                                },
                            );
                        }
                        let id = sent[0].0;
                        self.delivered.insert(key, id);
                        id
                    }
                    Err(ended) => {
                        if matches!(
                            ended,
                            HowASendEnded::HandedBack(WhyTheCallFailed::Refused(_))
                        ) && self.not_accepted.first_time_for(&job)
                        {
                            let text = telegram_did_not_accept(&self.server_label, &label, frame);
                            let _ = self.a_refusal_said(messages, &way, text, &label, class);
                        }
                        return APictureSent::NotAccepted(ended);
                    }
                }
            };
            first_message.get_or_insert(message);
        }
        let message = first_message.unwrap();
        // **`Telegram` C1b as changed 2026-11-09: no picture or album carries
        // a Stop, an answer to *send a frame* included, and nothing follows
        // one** — the job's own message carries it (the twenty-eighth notes,
        // note 19; Sol's fresh look, round 1, found answers still adding it).
        self.delivered
            .retain(|(held, number, _), _| held != &job || *number != frame);
        APictureSent::Sent {
            message,
            stop: None,
        }
    }

    /// **I3's refusal as a message of its own**: an answer's as before, Stop
    /// and all while its job is live; a journal frame's with no Stop, and
    /// without a sound once the job has ended (V1e).
    fn a_refusal_said(
        &mut self,
        messages: &mut TheJobMessages,
        way: &AFramesWay<'_>,
        text: AMarkedText,
        label: &str,
        class: AClassOfSend,
    ) -> HowASendEnded {
        if let Some(subject) = way.stop {
            return crate::actions::message_result(messages.send_about_in(
                &crate::job_messages::AComposedMessage { text, buttons: vec![] },
                subject,
                &self.server_label,
                label,
                class,
            ));
        }
        let (chat, how) = (self.chat, crate::transport::HowItIsSent::silent(way.after_the_ending));
        let queue = messages.queue.clone();
        wait_for(&queue, queue.offer(class, Box::new(move |c| c.send_marked_message_as(chat, &text, None, how))))
    }

    /// **Read how a picture's send ended.** A refusal by Telegram is said
    /// once for its job, and that telling is given back; anything else needs
    /// no telling. A send that reached nothing is C5's silence.
    pub fn take_in_how_it_ended(
        &mut self,
        queue: &TheSendQueue,
        job: &str,
        job_label: &str,
        frame: i64,
        ended: &HowASendEnded,
    ) -> Option<Receiver<HowASendEnded>> {
        let HowASendEnded::HandedBack(WhyTheCallFailed::Refused(_)) = ended else {
            return None;
        };
        if !self.not_accepted.first_time_for(job) {
            return None;
        }
        let (chat, text) = (self.chat, telegram_did_not_accept(&self.server_label, job_label, frame));
        Some(queue.offer(
            AClassOfSend::FramePicturesAndMedia,
            Box::new(move |c: &TheClient| c.send_marked_message(chat, &text, None)),
        ))
    }
}

/// **`Telegram` I2 as written 2026-10-18 — the sends a frame's copies go
/// as**: in their order, in albums of ten — `sendMediaGroup` takes 2 to 10
/// (`Sources` entry 88), and eleven was refused (entry 13) — with a last lone
/// copy sent as one picture, since an album takes at least two.
///
/// **`Telegram` I3a — and no send heavier than one album may weigh**
/// ([`THE_MOST_AN_ALBUM_MAY_WEIGH`]): a copy that would take the send past it
/// starts the next send, so a frame of heavy View Layers goes as more, smaller
/// albums and every layer still arrives. The live Telegram pass of
/// 2026-10-31, fault 3. The split depends only on the copies, so a retry
/// makes the same sends.
pub fn the_sends_of_a_frame(copies: Vec<ADerivative>) -> Vec<Vec<ADerivative>> {
    let mut sends: Vec<Vec<ADerivative>> = Vec::new();
    let mut weighs = 0;
    for copy in copies {
        let full = sends.last().is_none_or(|send| {
            send.len() >= THE_MOST_PICTURES_IN_ONE_MESSAGE || weighs + copy.bytes.len() > THE_MOST_AN_ALBUM_MAY_WEIGH
        });
        if full {
            sends.push(Vec::new());
            weighs = 0;
        }
        weighs += copy.bytes.len();
        sends.last_mut().expect("a send was just started").push(copy);
    }
    sends
}

/// **How a frame's pictures go** (`Telegram` I2, C1b, C1d, V1e, as changed
/// 2026-11-09 by the twenty-eighth notes, notes 18 and 19).
#[derive(Clone, Copy, Default)]
pub struct AFramesWay<'a> {
    /// **An answer's Stop**, on a lone picture, while its job is live.
    pub stop: Option<&'a TheSubject>,
    /// **The job's message the pictures reply to** (I2), where one is
    /// recorded.
    pub reply_to: Option<i64>,
    /// **Where I3's line goes while the job is live** (C1d).
    pub the_jobs_messages: Option<&'a crate::status::TheJobsOwnMessages>,
    /// **The job has ended**: a refusal is said without a sound (V1e).
    pub after_the_ending: bool,
}

impl AFramesWay<'_> {
    fn on_the_jobs_message(&self) -> Option<&crate::status::TheJobsOwnMessages> {
        self.the_jobs_messages.filter(|_| self.reply_to.is_some() && !self.after_the_ending)
    }
}

/// **The first message a send made**: a picture's own, or an album's first
/// picture's, since `sendMediaGroup` gives back every message it sent, in
/// order.
pub fn the_first_message_of(ended: &HowASendEnded) -> Option<i64> {
    match ended {
        HowASendEnded::Accepted(Value::Array(messages)) => messages.first()?.get("message_id")?.as_i64(),
        other => accepted_id(other),
    }
}

/// **`Telegram` I2 as written 2026-10-18 — one frame's copies sent, in their
/// order, each with its own caption** ([`the_caption`]: the server, the job,
/// the frame, the copy's View Layer or source, *not colour-accurate*): albums
/// of ten, then a lone copy as one picture ([`the_sends_of_a_frame`]). **Every
/// copy handed in is sent**: leaving out one outside I3's limits is the
/// caller's. Each send is waited for in turn, in `class`; **a frame's pictures
/// stay in the serial lane** — C6b's *"frame pictures from the derivative set
/// … are small and stay in the serial lane"*, which an album of them is too —
/// as a lone frame's picture always has. Gives back the first picture's message, or
/// how the send that was not accepted ended; the sends before it went.
pub fn send_a_frames_copies(
    queue: &TheSendQueue,
    chat: i64,
    server_label: &(impl TheServersName + ?Sized),
    copies: Vec<ADerivative>,
    class: AClassOfSend,
) -> Result<i64, HowASendEnded> {
    send_a_frames_copies_each(queue, chat, server_label, copies, class)
        .and_then(|sent| sent.first().map(|(message, _)| *message).ok_or_else(|| HowASendEnded::HandedBack(WhyTheCallFailed::NotReached("there was no copy to send".into()))))
}

/// **The same, giving back every picture's message with the copy it shows**,
/// in order, so each can be registered as about its job and frame
/// (`Telegram` R1b). Added 2026-10-31.
pub fn send_a_frames_copies_each(
    queue: &TheSendQueue,
    chat: i64,
    server_label: &(impl TheServersName + ?Sized),
    copies: Vec<ADerivative>,
    class: AClassOfSend,
) -> Result<Vec<(i64, ADerivative)>, HowASendEnded> {
    send_a_frames_copies_each_as(queue, chat, server_label, copies, class, crate::transport::HowItIsSent::default())
}

/// **The same, each send as `how` says** — a reply to the job's message
/// (`Telegram` I2, the twenty-eighth notes, note 19).
pub fn send_a_frames_copies_each_as(
    queue: &TheSendQueue,
    chat: i64,
    server_label: &(impl TheServersName + ?Sized),
    copies: Vec<ADerivative>,
    class: AClassOfSend,
    how: crate::transport::HowItIsSent,
) -> Result<Vec<(i64, ADerivative)>, HowASendEnded> {
    let mut every = Vec::new();
    for one_send in the_sends_of_a_frame(copies) {
        let shown: Vec<ADerivative> = one_send.iter().map(|copy| ADerivative { bytes: Vec::new(), ..copy.clone() }).collect();
        let captioned: Vec<(String, Vec<u8>, AMarkedText)> = one_send
            .into_iter()
            .enumerate()
            .map(|(at, copy)| {
                let caption = if at == 0 { the_caption_saying(server_label, &copy.job_label, copy.frame, copy.which_picture.as_ref(), &copy.not_read) }
                    else {
                        let which = copy.which_picture.as_ref().map(|which| format!(" · {}", which.as_said())).unwrap_or_default();
                        AMarkedText::plain(format!("Frame {}{which} · Not colour-accurate", copy.frame))
                    };
                (copy.file_name, copy.bytes, caption)
            })
            .collect();
        let ended = wait_for(
            queue,
            queue.offer(
                class,
                Box::new(move |c: &TheClient| match captioned.as_slice() {
                    [(file_name, bytes, caption)] => c.send_photo_marked_as(chat, file_name, bytes.clone(), Some(caption), None, how),
                    many => c.send_a_marked_album_as(
                        chat,
                        many.iter().map(|(name, bytes, caption)| (name.clone(), bytes.clone(), Some(caption.clone()))).collect(),
                        how,
                    ),
                }),
            ),
        );
        let Some(first) = the_first_message_of(&ended) else { return Err(ended) };
        let messages: Vec<i64> = match &ended {
            HowASendEnded::Accepted(Value::Array(sent)) => sent.iter().filter_map(|one| one.get("message_id")?.as_i64()).collect(),
            _ => vec![first],
        };
        every.extend(messages.into_iter().zip(shown));
    }
    if every.is_empty() {
        return Err(HowASendEnded::HandedBack(WhyTheCallFailed::NotReached("there was no copy to send".into())));
    }
    Ok(every)
}

/// **`Telegram` F3 — whether a picture's journal event is done**, so the
/// journal cursor may pass it: sent, or refused and the refusal decided. A send
/// that reached nothing, or whose answer could not be read, is not done, and
/// its event comes back.
pub fn the_work_is_done(ended: &HowASendEnded) -> bool {
    matches!(ended, HowASendEnded::Accepted(_) | HowASendEnded::HandedBack(WhyTheCallFailed::Refused(_)))
}

/// **`Telegram` I4 and I5 — what the derivative set holds for one frame**,
/// read from records. The real set is the machine's (`Job files and outputs`
/// B22, B30) and step 8b's to read.
#[derive(Debug, Clone, PartialEq)]
pub enum WhatTheSetHolds {
    Held(ADerivative),
    /// Legitimately cleared once the fetched copy exists (`Job files and
    /// outputs` K1); it can be made again while the machine exists (`Queue &
    /// History` P7).
    Cleared,
    /// The frame has not finished.
    NotFinished,
    /// The job's record directory is gone.
    TheRecordIsGone,
}

pub trait TheDerivativeSet {
    fn what_it_holds(&self, job: &str, frame: i64) -> WhatTheSetHolds;
}

/// **The answer to *send a frame*.**
#[derive(Debug, Clone, PartialEq)]
pub enum AnAnswerToSendAFrame {
    /// Send this picture ([`ThePictures::offer`]).
    ThePicture(ADerivative),
    /// **I4**: re-run the preview route under its own declarations (`Job
    /// files and outputs` B24); the picture is then on the journal like any
    /// other. *Act owed*: step 8b.
    ProduceItAgain { job: String, frame: i64 },
    /// **I5**: say this, from the records.
    Say(AMarkedText),
}

/// **`Telegram` I4 and I5.** A machine that is gone answers nothing, and the
/// listener names it as not answering (R14) — that case never reaches here.
pub fn answer_send_a_frame(
    set: &dyn TheDerivativeSet,
    server_label: &(impl TheServersName + ?Sized),
    job: &str,
    job_label: &str,
    frame: i64,
) -> AnAnswerToSendAFrame {
    match set.what_it_holds(job, frame) {
        WhatTheSetHolds::Held(derivative) => AnAnswerToSendAFrame::ThePicture(derivative),
        WhatTheSetHolds::Cleared => AnAnswerToSendAFrame::ProduceItAgain { job: job.to_string(), frame },
        WhatTheSetHolds::NotFinished => AnAnswerToSendAFrame::Say(a_user_message(server_label, job_label, &format!(" · Frame {frame}"), "Not finished rendering.")),
        WhatTheSetHolds::TheRecordIsGone => AnAnswerToSendAFrame::Say(a_user_message(server_label, job_label, "", "No longer on the server.")),
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::pacing::OnThisComputersClock;
    use crate::transport::a_stand_in::AStandInService;
    use crate::transport::WhatACallCarries;
    use std::sync::Arc;

    /// **A PNG header of any size**, with no picture data after it — enough
    /// for the header reading, which is all these tests need.
    pub(crate) fn a_png_header(width: u32, height: u32) -> Vec<u8> {
        let mut bytes = vec![0x89, b'P', b'N', b'G', 0x0d, 0x0a, 0x1a, 0x0a, 0, 0, 0, 13];
        bytes.extend_from_slice(b"IHDR");
        bytes.extend_from_slice(&width.to_be_bytes());
        bytes.extend_from_slice(&height.to_be_bytes());
        bytes.extend_from_slice(&[8, 2, 0, 0, 0, 0, 0, 0, 0]);
        bytes
    }

    fn a_jpeg_header(width: u16, height: u16) -> Vec<u8> {
        let mut bytes = vec![0xff, 0xd8, 0xff, 0xe0, 0x00, 0x04, 0x4a, 0x46];
        bytes.extend_from_slice(&[0xff, 0xc0, 0x00, 0x11, 0x08]);
        bytes.extend_from_slice(&height.to_be_bytes());
        bytes.extend_from_slice(&width.to_be_bytes());
        bytes.extend_from_slice(&[3, 1, 0x22, 0, 2, 0x11, 1, 3, 0x11, 1]);
        bytes
    }

    fn a_derivative(job: &str, frame: i64, bytes: Vec<u8>) -> ADerivative {
        ADerivative {
            job: job.into(),
            job_label: "Kitchen \u{2014} final".into(),
            frame,
            file_name: format!("frame{frame}.png"),
            bytes,
            which_picture: None,
            not_read: Vec::new(),
        }
    }

    #[test]
    fn the_dimensions_are_read_from_both_viewing_formats() {
        assert_eq!(the_dimensions_of(&a_png_header(1920, 1080)), Some((1920, 1080)));
        assert_eq!(the_dimensions_of(&a_jpeg_header(1280, 720)), Some((1280, 720)));
        assert_eq!(the_dimensions_of(b"GIF89a....."), None);
    }

    /// **`Telegram` I3's three caps**, including the observed refusal at 6000
    /// by 6000.
    #[test]
    fn the_three_caps_are_telegrams_own() {
        assert_eq!(whether_it_fits(&a_png_header(1920, 1080)), WhetherItFits::Fits);
        assert_eq!(whether_it_fits(&a_png_header(5000, 5000)), WhetherItFits::Fits);
        assert_eq!(whether_it_fits(&a_png_header(6000, 6000)), WhetherItFits::OutsideTheLimits);
        assert_eq!(whether_it_fits(&a_png_header(2000, 100)), WhetherItFits::Fits);
        assert_eq!(whether_it_fits(&a_png_header(2100, 100)), WhetherItFits::OutsideTheLimits);
        let mut heavy = a_png_header(1920, 1080);
        heavy.resize(THE_MOST_A_PICTURE_MAY_WEIGH, 0);
        assert_eq!(whether_it_fits(&heavy), WhetherItFits::Fits);
        heavy.push(0);
        assert_eq!(whether_it_fits(&heavy), WhetherItFits::OutsideTheLimits);
        assert_eq!(whether_it_fits(b"not a picture"), WhetherItFits::TheDimensionsCouldNotBeRead);
    }

    fn texts_and_methods(service: &AStandInService) -> Vec<(String, String)> {
        service
            .heard()
            .iter()
            .map(|call| {
                let words = match &call.carries {
                    WhatACallCarries::Fields(f) => f["text"].as_str().unwrap_or("").to_string(),
                    WhatACallCarries::FieldsAndFiles { fields, .. } => fields["caption"].as_str().unwrap_or("").to_string(),
                    WhatACallCarries::Nothing => String::new(),
                };
                (call.method.clone(), words)
            })
            .collect()
    }

    /// **`Telegram` I3 — a picture outside the limits is not sent, and the
    /// refusal is said once per job**; another job's is said for that job.
    #[test]
    fn a_picture_outside_the_limits_is_said_once_per_job() {
        let service = Arc::new(AStandInService::new());
        let queue = TheSendQueue::new(TheClient::over(service.clone()), Arc::new(OnThisComputersClock));
        let mut pictures = ThePictures::new(5, "RunPod 4090");
        let class = AClassOfSend::FramePicturesAndMedia;

        assert!(matches!(
            pictures.offer(&queue, a_derivative("job-a", 1, a_png_header(1920, 1080)), class),
            WhatBecameOfAPicture::Offered(_)
        ));
        assert!(matches!(
            pictures.offer(&queue, a_derivative("job-a", 2, a_png_header(8000, 4000)), class),
            WhatBecameOfAPicture::NotSentAndSaid(_)
        ));
        assert!(matches!(
            pictures.offer(&queue, a_derivative("job-a", 3, a_png_header(8000, 4000)), class),
            WhatBecameOfAPicture::NotSentAlreadySaid
        ));
        assert!(matches!(
            pictures.offer(&queue, a_derivative("job-b", 1, a_png_header(8000, 4000)), class),
            WhatBecameOfAPicture::NotSentAndSaid(_)
        ));
        while queue.send_the_next().is_some() {}

        let heard = texts_and_methods(&service);
        assert_eq!(heard.len(), 3);
        assert_eq!(heard[0], ("sendPhoto".into(), "RunPod 4090\nKitchen — final · Frame 1 · Not colour-accurate".into()));
        assert_eq!(heard[1].0, "sendMessage");
        assert!(heard[1].1.contains("Frame 2\nPreview not sent"));
        assert!(heard[2].1.contains("Frame 1\nPreview not sent"));
    }

    /// **`Telegram` I3 — a refusal Telegram makes is said once per job, and
    /// the event is done (F3).** A send that reached nothing is neither.
    #[test]
    fn a_refusal_by_telegram_is_said_once_per_job() {
        let service = Arc::new(AStandInService::new());
        let queue = TheSendQueue::new(TheClient::over(service.clone()), Arc::new(OnThisComputersClock));
        let mut pictures = ThePictures::new(5, "RunPod 4090");
        let refused = HowASendEnded::HandedBack(WhyTheCallFailed::Refused(crate::transport::ARefusal {
            error_code: Some(400),
            description: Some("Bad Request: PHOTO_INVALID_DIMENSIONS".into()),
            retry_after: None,
        }));
        let cut = HowASendEnded::HandedBack(WhyTheCallFailed::NotReached("cut".into()));

        assert!(pictures.take_in_how_it_ended(&queue, "job-a", "Kitchen", 4, &refused).is_some());
        assert!(pictures.take_in_how_it_ended(&queue, "job-a", "Kitchen", 5, &refused).is_none());
        assert!(pictures.take_in_how_it_ended(&queue, "job-a", "Kitchen", 6, &cut).is_none());
        assert!(the_work_is_done(&refused));
        assert!(!the_work_is_done(&cut));
        assert_eq!(queue.how_many_are_waiting(), 1);
    }

    /// **B16, B24, B25 — a picture is never pinned, notifies, and carries no
    /// spoiler.**
    #[test]
    fn a_picture_is_for_the_user() {
        let service = Arc::new(AStandInService::new());
        let queue = TheSendQueue::new(TheClient::over(service.clone()), Arc::new(OnThisComputersClock));
        let mut pictures = ThePictures::new(5, "RunPod 4090");
        pictures.offer(&queue, a_derivative("job-a", 1, a_png_header(320, 180)), AClassOfSend::FramePicturesAndMedia);
        queue.send_the_next();
        let heard = service.heard();
        assert!(heard.iter().all(|c| c.method != "pinChatMessage"));
        let WhatACallCarries::FieldsAndFiles { fields, files } = &heard[0].carries else { panic!() };
        assert!(fields.get("disable_notification").is_none());
        assert_eq!(fields["caption_entities"][0]["type"], "bold");
        assert_eq!(fields["caption_entities"][1]["type"], "italic");
        assert!(fields.get("entities").is_none());
        assert_eq!(files[0].file_name, "frame1.png");
    }

    /// **`Telegram` I2, the Creator's tenth notes, note 2: the caption names
    /// which picture it is, where the frame has more than one View Layer** —
    /// the main copy's View Layer, or its source where it is no one layer's —
    /// between the frame and *not colour-accurate*. A frame of one View Layer
    /// names none. And a picture offered is sent with that caption.
    #[test]
    fn tenth_notes_2_the_caption_names_the_layer_or_the_source_and_a_one_layer_frame_none() {
        let a_layer = WhichPictureItIs::AViewLayer("Reflections".into());
        assert_eq!(
            the_caption("RunPod 4090", "Kitchen \u{2014} final", 140, Some(&a_layer)),
            "RunPod 4090\nKitchen — final · Frame 140 · Reflections · Not colour-accurate",
            "the drawing's caption"
        );
        assert_eq!(
            the_caption("S", "J", 3, Some(&WhichPictureItIs::TheOutputPath)),
            "S\nJ · Frame 3 · Output path · Not colour-accurate"
        );
        assert_eq!(
            the_caption("S", "J", 3, Some(&WhichPictureItIs::AFileOutputNode("Beauty".into()))),
            "S\nJ · Frame 3 · File Output node Beauty · Not colour-accurate"
        );
        assert_eq!(the_caption("S", "J", 3, None), "S\nJ · Frame 3 · Not colour-accurate", "one layer, none named");
        // **C8 of the UI at every size: a job name past the caption's bound is
        // cut short and the rest of the caption still said.** Before, a
        // 1,100-letter name made a caption Telegram refuses.
        let long = "Kitchen_".repeat(140);
        let cut = the_caption("S", &long, 3, Some(&a_layer));
        assert!(cut.text.encode_utf16().count() <= THE_BOUND_ON_A_CAPTION, "{}", cut.text.encode_utf16().count());
        assert!(cut.text.ends_with("\u{2026} · Frame 3 · Reflections · Not colour-accurate"), "{}", &cut.text[cut.text.len() - 80..]);
        assert!(cut.text.starts_with("S\nKitchen_Kitchen_"));

        let service = Arc::new(AStandInService::new());
        let queue = TheSendQueue::new(TheClient::over(service.clone()), Arc::new(OnThisComputersClock));
        let mut pictures = ThePictures::new(5, "RunPod 4090");
        let named = ADerivative { which_picture: Some(a_layer), ..a_derivative("job-a", 140, a_png_header(1920, 1080)) };
        pictures.offer(&queue, named, AClassOfSend::FramePicturesAndMedia);
        pictures.offer(&queue, a_derivative("job-a", 141, a_png_header(1920, 1080)), AClassOfSend::FramePicturesAndMedia);
        while queue.send_the_next().is_some() {}
        let heard = texts_and_methods(&service);
        assert_eq!(
            heard[0].1,
            "RunPod 4090\nKitchen — final · Frame 140 · Reflections · Not colour-accurate"
        );
        assert_eq!(heard[1].1, "RunPod 4090\nKitchen — final · Frame 141 · Not colour-accurate");
    }

    // ------------------------------------------------ every View Layer, named

    fn a_frames_messages(service: &Arc<AStandInService>) -> TheJobMessages {
        let store: Box<dyn crate::worker_state::TheTelegramWorkersState> =
            Box::new(crate::worker_state::InMemory::with_routing_key("0123456789abcdef0123456789abcdef"));
        TheJobMessages {
            conversation: 5,
            store,
            queue: Arc::new(TheSendQueue::new(TheClient::over(service.clone()), Arc::new(OnThisComputersClock))),
            notes: Box::new(|_| {}),
        }
    }

    fn the_job() -> TheSubject {
        TheSubject { owning_machine: "machine-a".into(), job: Some("job-a".into()), hold: None, generation: "1".into() }
    }

    /// One frame's copy, of a View Layer by name.
    fn a_copy(frame: i64, layer: &str, bytes: Vec<u8>) -> ADerivative {
        ADerivative {
            which_picture: Some(WhichPictureItIs::AViewLayer(layer.into())),
            file_name: format!("{frame}-{layer}.png"),
            ..a_derivative("job-a", frame, bytes)
        }
    }

    fn the_album_answer(first: i64, how_many: i64) -> String {
        let messages: Vec<serde_json::Value> = (first..first + how_many).map(|id| serde_json::json!({ "message_id": id })).collect();
        serde_json::json!({ "ok": true, "result": messages }).to_string()
    }

    /// Each call's method and its words — a message's text or a picture's
    /// caption — where it carries any.
    fn the_words_heard(service: &AStandInService) -> Vec<(String, String)> {
        service
            .heard()
            .iter()
            .map(|call| {
                let fields = match &call.carries {
                    WhatACallCarries::Fields(f) => Some(f),
                    WhatACallCarries::FieldsAndFiles { fields, .. } => Some(fields),
                    WhatACallCarries::Nothing => None,
                };
                let words = fields
                    .and_then(|f| f.get("text").or_else(|| f.get("caption")))
                    .and_then(|w| w.as_str())
                    .unwrap_or("")
                    .to_string();
                (call.method.clone(), words)
            })
            .collect()
    }

    /// The captions an album carried, in order.
    fn the_albums_captions(call: &crate::transport::a_stand_in::ACallItHeard) -> Vec<String> {
        let WhatACallCarries::FieldsAndFiles { fields, files } = &call.carries else { panic!("an album uploads files") };
        let media = fields["media"].as_array().unwrap();
        assert_eq!(media.len(), files.len());
        media.iter().map(|one| one["caption"].as_str().unwrap().to_string()).collect()
    }

    /// **`Telegram` I2 and C1b as changed 2026-11-09 (the twenty-eighth notes,
    /// note 19: *"One message, Stop on job"*): a two-layer frame makes one
    /// `sendMediaGroup` whose two captions name *Interior* and *Reflections* in
    /// place order, sent as a reply to the job's message, with its sound, and
    /// nothing after it** — no companion, no Stop. Until then a second message
    /// followed it holding only *Stop this job*. **An answer's album carries
    /// none either.**
    #[test]
    fn twenty_eighth_19_a_two_layer_frame_is_one_album_replying_to_the_jobs_message_and_nothing_after() {
        let service = Arc::new(AStandInService::new());
        service.then_answers(&the_album_answer(501, 2));
        let mut messages = a_frames_messages(&service);
        let mut pictures = ThePictures::new(5, "RunPod 4090");
        let copies = || vec![
            a_copy(140, "Interior", a_png_header(1920, 1080)),
            a_copy(140, "Reflections", a_png_header(1920, 1080)),
        ];
        let sent = pictures.send_a_frame_as(
            &mut messages,
            copies(),
            AClassOfSend::FramePicturesAndMedia,
            AFramesWay { reply_to: Some(40), ..AFramesWay::default() },
        );
        assert_eq!(sent, APictureSent::Sent { message: 501, stop: None });
        let heard = service.heard();
        assert_eq!(heard.iter().map(|c| c.method.as_str()).collect::<Vec<_>>(), ["sendMediaGroup"], "nothing after the album");
        assert_eq!(
            the_albums_captions(&heard[0]),
            [
                "RunPod 4090\nKitchen — final · Frame 140 · Interior · Not colour-accurate",
                "Frame 140 · Reflections · Not colour-accurate",
            ]
        );
        let WhatACallCarries::FieldsAndFiles { fields, .. } = &heard[0].carries else { panic!() };
        assert!(fields.get("reply_markup").is_none() && fields.get("disable_notification").is_none(), "no button, and it sounds");
        assert_eq!(fields["reply_parameters"], serde_json::json!({ "message_id": 40 }));
        service.then_answers(&the_album_answer(601, 2));
        let answered = pictures.send_a_frame_with_its_stop(&mut messages, copies(), AClassOfSend::RoutedAnswersAndAcknowledgements, Some(&the_job()));
        assert_eq!(answered, APictureSent::Sent { message: 601, stop: None });
        assert_eq!(service.heard().len(), 2, "an answer's album has no companion either");
    }

    /// **A part of a frame that reached nothing is owed, and the parts that
    /// went are not sent again** — the journal's at-least-once life, now that
    /// no companion follows.
    #[test]
    fn a_lost_part_of_a_frame_is_retried_without_repeating_the_album() {
        let service = Arc::new(AStandInService::new());
        service.then_answers(&the_album_answer(801, 10));
        service.then_is_not_reached("line gone for the eleventh copy");
        service.then_answers(r#"{"ok":true,"result":{"message_id":811}}"#);
        let mut messages = a_frames_messages(&service);
        let mut pictures = ThePictures::new(5, "RunPod 4090");
        let copies = || (1..=11).map(|n| a_copy(9, &format!("L{n}"), a_png_header(640, 360))).collect::<Vec<_>>();
        let way = AFramesWay { reply_to: Some(40), ..AFramesWay::default() };
        let first = pictures.send_a_frame_as(&mut messages, copies(), AClassOfSend::FramePicturesAndMedia, way);
        assert!(matches!(first, APictureSent::NotAccepted(_)), "the eleventh copy is still owed");
        let again = pictures.send_a_frame_as(&mut messages, copies(), AClassOfSend::FramePicturesAndMedia, way);
        assert_eq!(again, APictureSent::Sent { message: 801, stop: None });
        assert_eq!(service.heard().iter().filter(|call| call.method == "sendMediaGroup").count(), 1);
    }

    /// **An older release's companion**, recorded as holding nothing but its
    /// Stop (`Telegram` C1b's line of 2026-11-02).
    fn an_older_releases_companion(messages: &mut TheJobMessages) {
        let mut stop = crate::job_messages::a_stop_to_offer("RunPod 4090", "Kitchen — final", false);
        stop.payload["the_message_says_only_its_stop"] = serde_json::json!(true);
        messages
            .send_with_buttons(
                &crate::job_messages::AComposedMessage { text: AMarkedText::plain("RunPod 4090\nKitchen — final · Frame 140"), buttons: vec![stop] },
                &the_job(),
            )
            .unwrap();
    }

    /// **`Telegram` C1b's line of 2026-11-02: a companion says nothing but its
    /// Stop, so it goes when its Stop goes** — from 2026-11-09 only one an
    /// older release sent, since this one sends none. The first 5.3 live pass
    /// saw, after *B9 multilayer grade*'s album, a message holding only the
    /// server's name and *B9 multilayer grade · Frame 1*. While the job is live
    /// it stays, and a message with words of its own only loses its button.
    #[test]
    fn an_older_releases_companion_goes_with_its_stop_and_nothing_else_does() {
        let service = Arc::new(AStandInService::new());
        service.then_answers(r#"{"ok":true,"result":{"message_id":503}}"#);
        let mut messages = a_frames_messages(&service);
        an_older_releases_companion(&mut messages);
        // **An older release's picture with its Stop keeps its caption and
        // loses its button.**
        messages.attach_a_stop(700, &the_job(), "RunPod 4090", "Kitchen — final", AClassOfSend::FramePicturesAndMedia).unwrap();
        let calls = service.heard().len();

        assert_eq!(messages.remove_the_stops_where(&|_| false).unwrap(), 0, "while the job is live it stays");
        assert_eq!(service.heard().len(), calls);

        // **A line that reaches nothing leaves it owed.**
        service.then_is_not_reached("line cut");
        assert!(messages.remove_the_stops_where(&|_| true).is_err());
        let mut restarted = TheJobMessages { store: messages.store, queue: messages.queue.clone(), notes: Box::new(|_| {}), conversation: 5 };

        assert_eq!(restarted.remove_the_stops_where(&|_| true).unwrap(), 2);
        let heard = service.heard();
        let after: Vec<(&str, i64)> = heard[calls..]
            .iter()
            .map(|call| {
                let WhatACallCarries::Fields(fields) = &call.carries else { panic!("{call:?}") };
                (call.method.as_str(), fields["message_id"].as_i64().unwrap())
            })
            .collect();
        assert_eq!(
            after,
            [("deleteMessage", 503), ("deleteMessage", 503), ("editMessageReplyMarkup", 700)],
            "the companion is deleted, and the picture keeps its caption"
        );
        assert_eq!(restarted.remove_the_stops_where(&|_| true).unwrap(), 0);
        assert_eq!(service.heard().len(), heard.len(), "done once");
    }

    /// **Where Telegram refuses the deletion, its button still goes** (`Telegram`
    /// B18: a message 48 hours old or more), so no Stop stands on an ended job.
    #[test]
    fn an_older_companion_too_old_to_delete_still_loses_its_stop() {
        let service = Arc::new(AStandInService::new());
        service.then_answers(r#"{"ok":true,"result":{"message_id":503}}"#);
        let mut messages = a_frames_messages(&service);
        an_older_releases_companion(&mut messages);
        let calls = service.heard().len();
        service.then_answers(r#"{"ok":false,"error_code":400,"description":"Bad Request: message can't be deleted"}"#);
        assert_eq!(messages.remove_the_stops_where(&|_| true).unwrap(), 1);
        let heard = service.heard();
        assert_eq!(
            heard[calls..].iter().map(|call| call.method.as_str()).collect::<Vec<_>>(),
            ["deleteMessage", "editMessageReplyMarkup"]
        );
    }

    /// **A job already ended gets no companion**, as before, so nothing is ever
    /// sent with nothing to say.
    #[test]
    fn an_album_of_an_ended_job_has_no_companion() {
        let service = Arc::new(AStandInService::new());
        service.then_answers(&the_album_answer(501, 2));
        let mut messages = a_frames_messages(&service);
        let mut pictures = ThePictures::new(5, "RunPod 4090");
        let copies = vec![
            a_copy(140, "Interior", a_png_header(1280, 720)),
            a_copy(140, "FX", a_png_header(1280, 720)),
        ];
        pictures.send_a_frame_with_its_stop(&mut messages, copies, AClassOfSend::FramePicturesAndMedia, None);
        assert_eq!(service.heard().iter().map(|call| call.method.as_str()).collect::<Vec<_>>(), ["sendMediaGroup"]);
    }

    /// **A one-layer frame makes one `sendPhoto`, its caption unchanged** —
    /// ~~and its stop attached~~ and from 2026-11-09 nothing after it.
    #[test]
    fn a_one_layer_frame_is_one_picture_as_before() {
        let service = Arc::new(AStandInService::new());
        service.then_answers(r#"{"ok":true,"result":{"message_id":700}}"#);
        let mut messages = a_frames_messages(&service);
        let mut pictures = ThePictures::new(5, "RunPod 4090");
        let copies = vec![a_derivative("job-a", 3, a_png_header(1920, 1080))];
        let sent = pictures.send_a_frame_with_its_stop(&mut messages, copies, AClassOfSend::FramePicturesAndMedia, Some(&the_job()));
        assert_eq!(sent, APictureSent::Sent { message: 700, stop: None });
        let heard = the_words_heard(&service);
        assert_eq!(heard[0], ("sendPhoto".into(), "RunPod 4090\nKitchen — final · Frame 3 · Not colour-accurate".into()));
        assert_eq!(heard.len(), 1, "no Stop on a picture from 2026-11-09, an answer's included (C1b)");
    }

    /// Eleven copies make an album of ten, then a lone picture, and nothing
    /// after (no companion since 2026-11-09).
    #[test]
    fn eleven_copies_make_an_album_of_ten_then_one_picture() {
        let service = Arc::new(AStandInService::new());
        service.then_answers(&the_album_answer(801, 10));
        service.then_answers(r#"{"ok":true,"result":{"message_id":811}}"#);
        let mut messages = a_frames_messages(&service);
        let mut pictures = ThePictures::new(5, "RunPod 4090");
        let copies: Vec<ADerivative> = (1..=11)
            .map(|n| a_copy(9, &format!("Layer {n}"), a_png_header(640, 360)))
            .collect();
        let sent = pictures.send_a_frame_with_its_stop(
            &mut messages,
            copies,
            AClassOfSend::FramePicturesAndMedia,
            Some(&the_job()),
        );
        assert_eq!(
            sent,
            APictureSent::Sent {
                message: 801,
                stop: None
            }
        );
        let heard = service.heard();
        assert_eq!(
            heard.iter().map(|c| c.method.as_str()).collect::<Vec<_>>(),
            ["sendMediaGroup", "sendPhoto"]
        );
        let captions = the_albums_captions(&heard[0]);
        assert_eq!(captions.len(), 10);
        assert!(captions[0].contains("Frame 9 · Layer 1 ·") && captions[9].contains("Frame 9 · Layer 10 ·"));
        let WhatACallCarries::FieldsAndFiles { fields, .. } = &heard[1].carries else { panic!() };
        assert_eq!(fields["caption"], "RunPod 4090\nKitchen — final · Frame 9 · Layer 11 · Not colour-accurate");
        let light = |n: usize| (1..=n).map(|_| a_copy(9, "L", a_png_header(640, 360))).collect::<Vec<_>>();
        assert_eq!(the_sends_of_a_frame(light(21)).iter().map(Vec::len).collect::<Vec<_>>(), [10, 10, 1]);
        assert_eq!(the_sends_of_a_frame(light(12)).iter().map(Vec::len).collect::<Vec<_>>(), [10, 2]);
    }

    /// **The live Telegram pass, fault 3 (`Telegram` I3a): a frame of heavy
    /// View Layers goes as more, smaller albums, never one heavier than an
    /// album may weigh**, every layer in its place and named, and nothing
    /// after them. Five 3840 × 2160 copies of 6 MB: three, then two.
    #[test]
    fn live_pass_3_heavy_layers_go_as_smaller_albums_and_every_layer_arrives() {
        let heavy = |layer: &str| {
            let mut bytes = a_png_header(3840, 2160);
            bytes.resize(6_000_000, 0);
            a_copy(4, layer, bytes)
        };
        let copies: Vec<ADerivative> = ["main", "backdrop", "fog", "glass", "sky"].into_iter().map(heavy).collect();
        let sends = the_sends_of_a_frame(copies.clone());
        assert_eq!(sends.iter().map(Vec::len).collect::<Vec<_>>(), [3, 2]);
        assert!(sends.iter().all(|send| send.iter().map(|copy| copy.bytes.len()).sum::<usize>() <= THE_MOST_AN_ALBUM_MAY_WEIGH));
        let mut one = heavy("one");
        one.bytes.resize(THE_MOST_A_PICTURE_MAY_WEIGH, 0);
        assert_eq!(the_sends_of_a_frame(vec![one.clone(), one.clone(), one]).iter().map(Vec::len).collect::<Vec<_>>(), [1, 1, 1]);

        let service = Arc::new(AStandInService::new());
        service.then_answers(&the_album_answer(901, 3));
        service.then_answers(&the_album_answer(904, 2));
        let mut messages = a_frames_messages(&service);
        let mut pictures = ThePictures::new(5, "RunPod 4090");
        let sent = pictures.send_a_frame_with_its_stop(&mut messages, copies, AClassOfSend::FramePicturesAndMedia, Some(&the_job()));
        assert_eq!(sent, APictureSent::Sent { message: 901, stop: None });
        let heard = service.heard();
        assert_eq!(heard.iter().map(|c| c.method.as_str()).collect::<Vec<_>>(), ["sendMediaGroup", "sendMediaGroup"]);
        let first = the_albums_captions(&heard[0]);
        let second = the_albums_captions(&heard[1]);
        assert!(first[0].contains("Frame 4 · main ·") && first[2].contains("Frame 4 · fog ·"));
        assert!(second[0].contains("Frame 4 · glass ·") && second[1].contains("Frame 4 · sky ·"));
    }

    /// **A copy outside I3's limits is left out of the album and said once
    /// for the job**: the next frame's outsized copy is left out with nothing
    /// said; a frame whose other copies all fit still goes as an album of
    /// them, and one left with a lone copy sends it as one picture, naming its
    /// layer.
    #[test]
    fn a_copy_outside_the_limits_is_left_out_and_said_once() {
        let service = Arc::new(AStandInService::new());
        let mut messages = a_frames_messages(&service);
        let mut pictures = ThePictures::new(5, "RunPod 4090");
        let frame = |n: i64| {
            vec![
                a_copy(n, "Interior", a_png_header(1920, 1080)),
                a_copy(n, "Huge", a_png_header(8000, 4000)),
                a_copy(n, "Reflections", a_png_header(1920, 1080)),
            ]
        };
        service.then_answers(r#"{"ok":true,"result":{"message_id":900}}"#);
        service.then_answers(&the_album_answer(901, 2));
        let sent = pictures.send_a_frame_with_its_stop(&mut messages, frame(1), AClassOfSend::FramePicturesAndMedia, None);
        assert_eq!(sent, APictureSent::Sent { message: 901, stop: None });
        service.then_answers(&the_album_answer(903, 2));
        pictures.send_a_frame_with_its_stop(&mut messages, frame(2), AClassOfSend::FramePicturesAndMedia, None);
        service.then_answers(r#"{"ok":true,"result":{"message_id":905}}"#);
        let lone = vec![a_copy(3, "Interior", a_png_header(1920, 1080)), a_copy(3, "Huge", a_png_header(8000, 4000))];
        let sent = pictures.send_a_frame_with_its_stop(&mut messages, lone, AClassOfSend::FramePicturesAndMedia, None);
        assert_eq!(sent, APictureSent::Sent { message: 905, stop: None });
        let heard = service.heard();
        assert_eq!(
            heard.iter().map(|c| c.method.as_str()).collect::<Vec<_>>(),
            ["sendMessage", "sendMediaGroup", "sendMediaGroup", "sendPhoto"]
        );
        let said = the_words_heard(&service);
        assert!(said[0].1.contains("Frame 1\nPreview not sent") && said[0].1.ends_with("over Telegram's limits."));
        assert!(the_albums_captions(&heard[1]).iter().all(|c| !c.contains("Huge")));
        assert_eq!(the_albums_captions(&heard[2]).len(), 2);
        assert!(said[3].1.contains("Frame 3 · Interior ·"), "{:?}", said[3]);
    }

    /// **`Telegram` C1d and V1e: I3's refusal is a line on the job's message
    /// while the job is live**, and no message of its own; **after the ending
    /// it is a message of its own without a sound.** The twenty-eighth notes,
    /// note 18.
    #[test]
    fn twenty_eighth_18_a_refusal_is_a_line_on_the_jobs_message_while_live_and_silent_after() {
        let service = Arc::new(AStandInService::new());
        let mut messages = a_frames_messages(&service);
        let keeper = crate::status::TheJobsOwnMessages::default();
        let mut pictures = ThePictures::new(5, "RunPod 4090");
        service.then_answers(r#"{"ok":true,"result":{"message_id":900}}"#);
        let frame = vec![a_copy(1, "Interior", a_png_header(1920, 1080)), a_copy(1, "Huge", a_png_header(8000, 4000))];
        let way = AFramesWay { reply_to: Some(40), the_jobs_messages: Some(&keeper), ..AFramesWay::default() };
        pictures.send_a_frame_as(&mut messages, frame, AClassOfSend::FramePicturesAndMedia, way);
        assert_eq!(service.heard().iter().map(|c| c.method.as_str()).collect::<Vec<_>>(), ["sendPhoto"]);
        assert_eq!(keeper.the_lines_of("job-a"), ["Preview not sent: over Telegram's limits."]);
        let mut after = ThePictures::new(5, "RunPod 4090");
        let ended = AFramesWay { after_the_ending: true, the_jobs_messages: Some(&keeper), ..AFramesWay::default() };
        after.send_a_frame_as(&mut messages, vec![a_copy(2, "Huge", a_png_header(8000, 4000))], AClassOfSend::FramePicturesAndMedia, ended);
        let heard = service.heard();
        assert_eq!(heard[1].method, "sendMessage");
        let WhatACallCarries::Fields(fields) = &heard[1].carries else { panic!() };
        assert_eq!(fields["disable_notification"], serde_json::json!(true));
    }

    struct ASet;
    impl TheDerivativeSet for ASet {
        fn what_it_holds(&self, job: &str, frame: i64) -> WhatTheSetHolds {
            match (job, frame) {
                ("gone", _) => WhatTheSetHolds::TheRecordIsGone,
                (_, 1) => WhatTheSetHolds::Held(a_derivative(job, 1, a_png_header(320, 180))),
                (_, 2) => WhatTheSetHolds::Cleared,
                _ => WhatTheSetHolds::NotFinished,
            }
        }
    }

    /// **`Telegram` I4 and I5.**
    #[test]
    fn send_a_frame_is_answered_from_the_set_and_its_records() {
        assert!(matches!(answer_send_a_frame(&ASet, "S", "j", "J", 1), AnAnswerToSendAFrame::ThePicture(_)));
        assert_eq!(
            answer_send_a_frame(&ASet, "S", "j", "J", 2),
            AnAnswerToSendAFrame::ProduceItAgain { job: "j".into(), frame: 2 }
        );
        assert_eq!(
            answer_send_a_frame(&ASet, "S", "j", "J", 9),
            AnAnswerToSendAFrame::Say(a_user_message("S", "J", " · Frame 9", "Not finished rendering."))
        );
        assert!(matches!(answer_send_a_frame(&ASet, "S", "gone", "J", 1), AnAnswerToSendAFrame::Say(s) if s.contains("No longer on the server")));
    }

    // ------------------------------------------------ the four questions, batch 4

    /// **Sol's S2 (the four questions, batch 4, 2026-11-11): a frame below
    /// zero keeps its number** in every caption and every line about its
    /// picture. `Specs/Render Settings`: *"Nothing in Smoozy assumes the first
    /// frame is frame 1"*, and a range may start below zero. Before, these took
    /// a frame of zero or more, and the journal turned a frame below zero into
    /// frame 0.
    #[test]
    fn four_questions_s2_a_frame_below_zero_keeps_its_number() {
        assert_eq!(the_caption("S", "J", -5, None), "S\nJ · Frame -5 · Not colour-accurate");
        assert_eq!(outside_the_limits("S", "J", -5), a_user_message("S", "J", " · Frame -5", OVER_THE_LIMITS));
        assert!(telegram_did_not_accept("S", "J", -5).text.contains("Frame -5"));
        assert!(a_frames_picture_could_not_be_made("S", "J", -5).text.contains("Frame -5"));
        let service = Arc::new(AStandInService::new());
        service.then_answers(&the_album_answer(601, 2));
        let mut messages = a_frames_messages(&service);
        let mut pictures = ThePictures::new(5, "RunPod 4090");
        let sent = pictures.send_a_frame_as(
            &mut messages,
            vec![a_copy(-3, "Interior", a_png_header(1920, 1080)), a_copy(-3, "Reflections", a_png_header(1920, 1080))],
            AClassOfSend::FramePicturesAndMedia,
            AFramesWay::default(),
        );
        assert_eq!(sent, APictureSent::Sent { message: 601, stop: None });
        assert_eq!(
            the_albums_captions(&service.heard()[0]),
            [
                "RunPod 4090\nKitchen — final · Frame -3 · Interior · Not colour-accurate",
                "Frame -3 · Reflections · Not colour-accurate",
            ]
        );
    }

    /// **Sol's S6 (the four questions, batch 4, 2026-11-11): a View Layer whose
    /// preview could not be read is named on its album's first picture**, so
    /// the artist knows the layer rendered and only its preview is missing
    /// (`Telegram` I2 as changed that day). Before, it was left out with a line
    /// in the server's record alone. A lone picture left of a two-layer frame
    /// says it the same way.
    #[test]
    fn four_questions_s6_a_layer_whose_preview_was_not_read_is_named() {
        let service = Arc::new(AStandInService::new());
        service.then_answers(&the_album_answer(701, 2));
        let mut messages = a_frames_messages(&service);
        let mut pictures = ThePictures::new(5, "RunPod 4090");
        let main = ADerivative { not_read: vec!["Reflections".into()], ..a_copy(140, "Interior", a_png_header(1920, 1080)) };
        pictures.send_a_frame_as(
            &mut messages,
            vec![main.clone(), a_copy(140, "Shadow", a_png_header(1920, 1080))],
            AClassOfSend::FramePicturesAndMedia,
            AFramesWay::default(),
        );
        assert_eq!(
            the_albums_captions(&service.heard()[0]),
            [
                "RunPod 4090\nKitchen — final · Frame 140 · Interior · Not colour-accurate\nCould not read the preview of Reflections.",
                "Frame 140 · Shadow · Not colour-accurate",
            ]
        );

        let alone = Arc::new(AStandInService::new());
        let mut messages = a_frames_messages(&alone);
        let mut pictures = ThePictures::new(5, "RunPod 4090");
        let two = ADerivative { not_read: vec!["Reflections".into(), "Shadow".into()], ..main };
        pictures.send_a_frame_as(&mut messages, vec![two], AClassOfSend::FramePicturesAndMedia, AFramesWay::default());
        assert_eq!(
            the_words_heard(&alone)[0].1,
            "RunPod 4090\nKitchen — final · Frame 140 · Interior · Not colour-accurate\nCould not read the preview of Reflections, Shadow."
        );
    }
}
