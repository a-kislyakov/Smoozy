//! A picture shown in Files — `File Browser` §1.8, L29.
//!
//! **Constraint:** Specs `File Browser`, *Browsing a server* — *"Opening a PNG
//! or JPEG on the server shows it over the window, as it is, read from the
//! server to be shown and never kept on the user's computer. It works on any
//! connected server, set up or not. Any other picture, and any file over 100
//! MB, is not shown."* The Creator's seventeenth notes, note 11, and their yes
//! of 2026-10-30; PNG and JPEG only and the cap ruled by the build runner on
//! 2026-10-31.
//!
//! **L29: a picture is read whole into the desktop's memory by the
//! file-transfer subsystem's byte-offset reads, on a channel of its own
//! admitted through 6.1's pool, and never touches a disk.** What is here is the
//! judgement — which names offer the act, the cap, the read with its stops,
//! and the check that the bytes are PNG or JPEG. The channel, its admission
//! and the page are `Desktop app and delivery`'s; the real end of the reading
//! is [`crate::wiring`]'s.
//!
//! **Nothing here writes anything.** There is no write route out of this file:
//! the bytes are handed back and the caller hands them to the page.
//!
//! **Markers used below that belong to another document, each with the
//! document it belongs to.** `Server protocol and transfers` R6a.

use smoozy_server_protocol::transfers::moving::THE_MOST_READS_IN_FLIGHT;

/// **L29's class** is declared at [`crate::declarations::THE_PICTURE_VIEW_CLASS`]
/// and ranked at [`crate::pool::the_declared_order_with_the_picture_view`].
pub use crate::declarations::THE_PICTURE_VIEW_CLASS;

/// **L29's cap: 100,000,000 bytes**, in the thousands Transfers writes sizes
/// in. L1's listed size is checked against it first, and the read stops at it.
pub const THE_CAP: u64 = 100_000_000;

/// **What a name's ending offers** — L29's *which files*: *"An entry whose
/// name ends `.png`, `.jpg` or `.jpeg`, case ignored, read off L1a's bytes."*
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhatTheEndingOffers {
    /// **PNG or JPEG: the act is offered, and decoding decides.**
    APictureToShow,
    /// **A picture Smoozy does not show**, named by its format — Design's
    /// *Can't preview OpenEXR files.*, or TIFF, Targa, HDR or WebP. No
    /// byte of it is read.
    APictureNotShown(TheFormat),
    /// Not a picture: nothing opens.
    NotAPicture,
}

/// The working formats Design names in the line that stands where the picture
/// would.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TheFormat {
    OpenExr,
    Tiff,
    Targa,
    Hdr,
    WebP,
}

impl TheFormat {
    /// The format's own name, as Design writes it. A value read into the line,
    /// never a sentence.
    pub fn the_name(self) -> &'static str {
        match self {
            TheFormat::OpenExr => "OpenEXR",
            TheFormat::Tiff => "TIFF",
            TheFormat::Targa => "Targa",
            TheFormat::Hdr => "HDR",
            TheFormat::WebP => "WebP",
        }
    }
}

/// **The ending, read off the name's bytes with case ignored** (L1a: the bytes
/// the machine sent, never a decoded form).
pub fn what_the_ending_offers(name: &[u8]) -> WhatTheEndingOffers {
    let lower: Vec<u8> = name.iter().map(u8::to_ascii_lowercase).collect();
    let ends = |suffix: &[u8]| lower.ends_with(suffix);
    if ends(b".png") || ends(b".jpg") || ends(b".jpeg") {
        return WhatTheEndingOffers::APictureToShow;
    }
    let not_shown = [
        (b".exr".as_slice(), TheFormat::OpenExr),
        (b".tif", TheFormat::Tiff),
        (b".tiff", TheFormat::Tiff),
        (b".tga", TheFormat::Targa),
        (b".hdr", TheFormat::Hdr),
        (b".webp", TheFormat::WebP),
    ];
    for (suffix, format) in not_shown {
        if ends(suffix) {
            return WhatTheEndingOffers::APictureNotShown(format);
        }
    }
    WhatTheEndingOffers::NotAPicture
}

/// **Whether the listed size is over the cap** — L29: *"L1's listed size is
/// checked first and a file above it is not read at all."*
pub fn the_listed_size_is_over_the_cap(listed: u64) -> bool {
    listed > THE_CAP
}

/// Which of the two the bytes open as.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TheBytesAre {
    Png,
    Jpeg,
}

impl TheBytesAre {
    /// The media type the page decodes the bytes under.
    pub fn the_media_type(self) -> &'static str {
        match self {
            TheBytesAre::Png => "image/png",
            TheBytesAre::Jpeg => "image/jpeg",
        }
    }
}

/// **The bytes' own opening: PNG's eight-byte signature, or JPEG's start of
/// image followed by a marker.** L29: *"The ending only offers the act;
/// decoding decides: bytes that do not decode as PNG or JPEG are a picture that
/// cannot be shown, never a guess at another format."* **This is the first half
/// of the decoding and the page's decode is the second**: a `.png` holding a
/// WebP would decode in a web view, so the page is never handed bytes that do
/// not open as one of the two.
pub fn what_the_bytes_are(bytes: &[u8]) -> Option<TheBytesAre> {
    const PNG: &[u8] = b"\x89PNG\r\n\x1a\n";
    if bytes.starts_with(PNG) {
        return Some(TheBytesAre::Png);
    }
    if bytes.len() >= 3 && bytes[0] == 0xFF && bytes[1] == 0xD8 && bytes[2] == 0xFF {
        return Some(TheBytesAre::Jpeg);
    }
    None
}

/// Why a read did not come back — L29: *"A read that fails — a dropped line, a
/// refusal, a cut — shows Design's failed state with the reason."*
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhyItWasNotRead {
    /// The protocol's own permission-denied status.
    TheLoginMayNot,
    /// The protocol's own no-such-file status: gone since the listing.
    ItIsNotThere,
    /// The channel went: the line dropped or was cut.
    TheLineWent,
    /// Any other refusal. Nothing is guessed from it.
    ItRefused,
}

/// **What the read came to.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ThePictureRead {
    /// The whole file, and it opens as PNG or JPEG.
    TheBytes { bytes: Vec<u8>, are: TheBytesAre },
    /// **Over the cap**: on the listed size, read nothing at all; or grown past
    /// it since the listing, and read no further than the cap.
    TooLarge,
    /// The bytes do not open as PNG or JPEG.
    CannotBeShown,
    /// The read failed, and why. **Not retried by itself.**
    NotRead(WhyItWasNotRead),
    /// **The viewer was closed or stepped on**: the read stopped and its
    /// channel is the caller's to give back. L29: *"Closing the viewer, or
    /// stepping to another entry, cancels a read in flight and frees its
    /// channel."*
    LetGo,
}

/// **The file-transfer subsystem's byte-offset reads against an open handle**,
/// as the read needs them — a seam, so the cap, the stops and the cancel are
/// testable at the desk. The real end is [`crate::wiring`]'s.
///
/// **A piece is handed over without waiting for its answer**
/// (`Server protocol and transfers` R6a's read path), so a far server costs
/// round trips by the window and not by the piece.
pub trait ReadsAFileInPieces {
    type Handle: Send + Sync;
    /// **The path crosses as the bytes the listing gave** (L1a), never a
    /// decoded form.
    fn open_the_file(
        &mut self,
        path: &[u8],
    ) -> impl std::future::Future<Output = Result<Self::Handle, WhyItWasNotRead>> + Send;
    /// Hand one read over; the answer is collected by [`Self::the_piece`].
    fn send_a_piece(
        &mut self,
        handle: &Self::Handle,
        at: u64,
        want: usize,
    ) -> impl std::future::Future<Output = Result<u32, WhyItWasNotRead>> + Send;
    /// The answer to a piece handed over. **Empty is the end of the file.**
    fn the_piece(
        &mut self,
        ticket: u32,
    ) -> impl std::future::Future<Output = Result<Vec<u8>, WhyItWasNotRead>> + Send;
    fn close_the_file(&mut self, handle: &Self::Handle) -> impl std::future::Future<Output = ()> + Send;
    /// The negotiated read size.
    fn the_piece_size(&self) -> usize;
}

/// **The whole of L29's read.**
///
/// In this order: the listed size against the cap, and nothing is opened
/// above it; the file opened; pieces handed over in order, at most
/// [`THE_MOST_READS_IN_FLIGHT`] at once, **never past the cap**; and the bytes'
/// opening checked. **`still_wanted` is asked before every piece**, and the
/// moment it says no the read stops and answers [`ThePictureRead::LetGo`] —
/// nothing more is asked of the machine, and the caller drops the channel,
/// which takes any answer still on it with it. **No read ahead**: only the
/// file asked for is read.
///
/// **A file exactly at the cap is read whole.** One that reaches the cap
/// having been listed below it has grown since the listing, and is too large
/// without a byte past the cap being asked for.
///
/// **A short answer is not the end**: where a piece comes back with fewer bytes
/// than asked and more than none, the answers behind it are collected and set
/// aside and the read goes on from where the bytes end, so no hole is left.
pub async fn read_the_picture<R: ReadsAFileInPieces>(
    path: &[u8],
    listed: u64,
    reader: &mut R,
    still_wanted: &(dyn Fn() -> bool + Sync),
) -> ThePictureRead {
    if the_listed_size_is_over_the_cap(listed) {
        return ThePictureRead::TooLarge;
    }
    if !still_wanted() {
        return ThePictureRead::LetGo;
    }
    let handle = match reader.open_the_file(path).await {
        Ok(handle) => handle,
        Err(why) => return ThePictureRead::NotRead(why),
    };
    let piece = reader.the_piece_size().max(1);
    let mut got: Vec<u8> = Vec::new();
    // What is handed over: (where, how much, ticket), in the order sent.
    let mut in_flight: std::collections::VecDeque<(u64, usize, u32)> = std::collections::VecDeque::new();
    let mut next_at: u64 = 0;
    let mut the_end_was_met = false;
    // **How many may be in flight.** One after a short answer, which is most
    // often the file's last piece, so the end is confirmed by one more ask
    // rather than by a whole window of them.
    let mut window = THE_MOST_READS_IN_FLIGHT;
    loop {
        while !the_end_was_met && in_flight.len() < window && next_at < THE_CAP {
            if !still_wanted() {
                return ThePictureRead::LetGo;
            }
            let want = usize::try_from(THE_CAP - next_at).unwrap_or(usize::MAX).min(piece);
            match reader.send_a_piece(&handle, next_at, want).await {
                Ok(ticket) => in_flight.push_back((next_at, want, ticket)),
                Err(why) => {
                    reader.close_the_file(&handle).await;
                    return ThePictureRead::NotRead(why);
                }
            }
            next_at += want as u64;
        }
        let Some((at, want, ticket)) = in_flight.pop_front() else {
            break;
        };
        if !still_wanted() {
            return ThePictureRead::LetGo;
        }
        let bytes = match reader.the_piece(ticket).await {
            Ok(bytes) => bytes,
            Err(why) => {
                reader.close_the_file(&handle).await;
                return ThePictureRead::NotRead(why);
            }
        };
        if at != got.len() as u64 {
            // **Behind a short answer**: collected and set aside.
            continue;
        }
        if bytes.is_empty() {
            the_end_was_met = true;
            continue;
        }
        let short = bytes.len() < want;
        got.extend_from_slice(&bytes);
        if short {
            // **Re-asked from where the bytes end**, once the answers already
            // handed over have been collected.
            next_at = got.len() as u64;
            window = 1;
        } else {
            window = THE_MOST_READS_IN_FLIGHT;
        }
    }
    reader.close_the_file(&handle).await;
    if got.len() as u64 >= THE_CAP && !the_end_was_met && listed < THE_CAP {
        return ThePictureRead::TooLarge;
    }
    match what_the_bytes_are(&got) {
        Some(are) => ThePictureRead::TheBytes { bytes: got, are },
        None => ThePictureRead::CannotBeShown,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

    /// A file on a machine, read in pieces of `piece` bytes.
    struct AFile {
        bytes: Vec<u8>,
        piece: usize,
        opened: usize,
        asked: Vec<(u64, usize)>,
        closed: usize,
        /// Answers with fewer bytes than asked on the piece at this offset.
        short_at: Option<u64>,
        refuses: Option<WhyItWasNotRead>,
    }

    impl AFile {
        fn holding(bytes: Vec<u8>) -> AFile {
            AFile { bytes, piece: 1000, opened: 0, asked: Vec::new(), closed: 0, short_at: None, refuses: None }
        }
    }

    impl ReadsAFileInPieces for AFile {
        type Handle = ();
        async fn open_the_file(&mut self, _path: &[u8]) -> Result<(), WhyItWasNotRead> {
            self.opened += 1;
            match self.refuses {
                Some(why) => Err(why),
                None => Ok(()),
            }
        }
        async fn send_a_piece(&mut self, _handle: &(), at: u64, want: usize) -> Result<u32, WhyItWasNotRead> {
            self.asked.push((at, want));
            Ok(self.asked.len() as u32 - 1)
        }
        async fn the_piece(&mut self, ticket: u32) -> Result<Vec<u8>, WhyItWasNotRead> {
            let (at, want) = self.asked[ticket as usize];
            let from = (at as usize).min(self.bytes.len());
            let mut to = (from + want).min(self.bytes.len());
            if self.short_at == Some(at) && to > from + 1 {
                to = from + (to - from) / 2;
                self.short_at = None;
            }
            Ok(self.bytes[from..to].to_vec())
        }
        async fn close_the_file(&mut self, _handle: &()) {
            self.closed += 1;
        }
        fn the_piece_size(&self) -> usize {
            self.piece
        }
    }

    fn a_png(len: usize) -> Vec<u8> {
        let mut bytes = b"\x89PNG\r\n\x1a\n".to_vec();
        bytes.extend((0..len.saturating_sub(8)).map(|at| (at % 251) as u8));
        bytes
    }

    #[test]
    fn only_png_and_jpeg_offer_the_act_and_the_working_formats_are_named() {
        assert_eq!(what_the_ending_offers(b"kitchen_0002.png"), WhatTheEndingOffers::APictureToShow);
        assert_eq!(what_the_ending_offers(b"FRAME.JPG"), WhatTheEndingOffers::APictureToShow);
        assert_eq!(what_the_ending_offers(b"a.JpEg"), WhatTheEndingOffers::APictureToShow);
        assert_eq!(
            what_the_ending_offers(b"studio_backplate.exr"),
            WhatTheEndingOffers::APictureNotShown(TheFormat::OpenExr)
        );
        assert_eq!(what_the_ending_offers(b"a.tif"), WhatTheEndingOffers::APictureNotShown(TheFormat::Tiff));
        assert_eq!(what_the_ending_offers(b"a.TIFF"), WhatTheEndingOffers::APictureNotShown(TheFormat::Tiff));
        assert_eq!(what_the_ending_offers(b"a.tga"), WhatTheEndingOffers::APictureNotShown(TheFormat::Targa));
        assert_eq!(what_the_ending_offers(b"a.hdr"), WhatTheEndingOffers::APictureNotShown(TheFormat::Hdr));
        assert_eq!(what_the_ending_offers(b"a.webp"), WhatTheEndingOffers::APictureNotShown(TheFormat::WebP));
        assert_eq!(what_the_ending_offers(b"scene.blend"), WhatTheEndingOffers::NotAPicture);
        assert_eq!(what_the_ending_offers(b"render.mp4"), WhatTheEndingOffers::NotAPicture);
        assert_eq!(what_the_ending_offers(b"png"), WhatTheEndingOffers::NotAPicture);
        assert_eq!(TheFormat::OpenExr.the_name(), "OpenEXR");
    }

    #[test]
    fn the_bytes_decide_and_nothing_else_is_guessed() {
        assert_eq!(what_the_bytes_are(&a_png(64)), Some(TheBytesAre::Png));
        assert_eq!(what_the_bytes_are(b"\xFF\xD8\xFF\xE0rest"), Some(TheBytesAre::Jpeg));
        // A WebP called `.png`, a GIF and nothing at all.
        assert_eq!(what_the_bytes_are(b"RIFF\0\0\0\0WEBPVP8 "), None);
        assert_eq!(what_the_bytes_are(b"GIF89a"), None);
        assert_eq!(what_the_bytes_are(b""), None);
    }

    #[tokio::test]
    async fn a_png_is_read_whole_with_the_reads_in_flight_and_closed() {
        let png = a_png(25_500);
        let mut file = AFile::holding(png.clone());
        let read = read_the_picture(b"/out/kitchen_0002.png", png.len() as u64, &mut file, &|| true).await;
        assert_eq!(read, ThePictureRead::TheBytes { bytes: png.clone(), are: TheBytesAre::Png });
        assert_eq!(file.opened, 1);
        assert_eq!(file.closed, 1);
        // Never more than the window asked for past the file's end, and the
        // window's first pieces handed over in order without waiting.
        let past_the_end = file.asked.iter().filter(|(at, _)| *at >= png.len() as u64).count();
        assert!(past_the_end <= THE_MOST_READS_IN_FLIGHT, "{past_the_end}");
        assert!(file.asked[..THE_MOST_READS_IN_FLIGHT]
            .windows(2)
            .all(|two| two[1].0 == two[0].0 + two[0].1 as u64));
    }

    #[tokio::test]
    async fn a_short_answer_leaves_no_hole() {
        let png = a_png(9_000);
        let mut file = AFile::holding(png.clone());
        file.short_at = Some(2000);
        let read = read_the_picture(b"/a.png", png.len() as u64, &mut file, &|| true).await;
        assert_eq!(read, ThePictureRead::TheBytes { bytes: png, are: TheBytesAre::Png });
    }

    #[tokio::test]
    async fn a_file_over_the_cap_on_its_listed_size_is_not_opened() {
        let mut file = AFile::holding(a_png(10));
        let read = read_the_picture(b"/studio_matte_16bit.png", 214_000_000, &mut file, &|| true).await;
        assert_eq!(read, ThePictureRead::TooLarge);
        assert_eq!(file.opened, 0, "a file above the cap is not read at all");
        assert!(file.asked.is_empty());
        assert!(!the_listed_size_is_over_the_cap(THE_CAP));
        assert!(the_listed_size_is_over_the_cap(THE_CAP + 1));
    }

    #[tokio::test]
    async fn a_file_that_grew_past_the_cap_is_never_read_past_it() {
        let mut file = AFile::holding(a_png(THE_CAP as usize + 5_000));
        file.piece = 4 * 1024 * 1024;
        let read = read_the_picture(b"/grew.png", 10_000, &mut file, &|| true).await;
        assert_eq!(read, ThePictureRead::TooLarge);
        let furthest = file.asked.iter().map(|(at, want)| at + *want as u64).max().unwrap_or(0);
        assert_eq!(furthest, THE_CAP, "nothing past the cap was asked for");
    }

    #[tokio::test]
    async fn closing_or_stepping_stops_the_read() {
        let png = a_png(200_000);
        let mut file = AFile::holding(png.clone());
        let answered = AtomicUsize::new(0);
        let wanted = AtomicBool::new(true);
        let still_wanted = || {
            // The viewer closes after a few asks.
            if answered.fetch_add(1, Ordering::SeqCst) >= 20 {
                wanted.store(false, Ordering::SeqCst);
            }
            wanted.load(Ordering::SeqCst)
        };
        let read = read_the_picture(b"/a.png", png.len() as u64, &mut file, &still_wanted).await;
        assert_eq!(read, ThePictureRead::LetGo);
        assert!(file.asked.len() < 200, "the read stopped where it was let go");
        let mut file = AFile::holding(png);
        assert_eq!(read_the_picture(b"/a.png", 200_000, &mut file, &|| false).await, ThePictureRead::LetGo);
        assert_eq!(file.opened, 0, "let go before it started, nothing is opened");
    }

    #[tokio::test]
    async fn bytes_that_are_not_png_or_jpeg_cannot_be_shown_and_a_refusal_says_why() {
        let mut file = AFile::holding(b"RIFF\0\0\0\0WEBPVP8 data".to_vec());
        assert_eq!(read_the_picture(b"/a.png", 20, &mut file, &|| true).await, ThePictureRead::CannotBeShown);
        let mut file = AFile::holding(a_png(100));
        file.refuses = Some(WhyItWasNotRead::TheLoginMayNot);
        assert_eq!(
            read_the_picture(b"/a.png", 100, &mut file, &|| true).await,
            ThePictureRead::NotRead(WhyItWasNotRead::TheLoginMayNot)
        );
    }
}
