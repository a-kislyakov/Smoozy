//! The `.blend` version — `File Browser` §1.3, L7 to L10, Rows 182 and 345.
//!
//! **Constraint:** Row 182 — *"the Blender version it was saved with … Smoozy
//! reads that version straight from the file, without running Blender on it or
//! pre-loading its settings."*
//!
//! **Markers used below that belong to another document, each with the
//! document it belongs to.** `Server protocol and transfers` 1.1 item 2, T2.
//! `Servers` §15.

use crate::listing::{ApparentSize, LastChanged};

/// **L7: the reading is a bounded prefix of the file's bytes, fetched by the
/// desktop over the file-transfer subsystem's byte-offset reads on L1's one
/// long-lived reading channel, and interpreted by Smoozy itself.**
///
/// **No Blender runs, nothing opens the file, and no machine-side program is
/// asked** — which programs an arbitrary rental carries is established nowhere,
/// and the reading must work on the pre-package surface before anything is
/// installed.
///
/// **The channel is named because the alternative is expensive and silent.** A
/// byte-offset read against an open handle is the operation a transfer is made
/// of, so a reading that opened a channel of its own would offer sixty channel
/// admissions for sixty `.blend` files in one folder, against a session cap a
/// provider image may set to 1 (`Server protocol and transfers` T2) and beside
/// the four-slot pool of §6.1.
///
/// **The prefix reads ride the reading channel and are serialised behind the
/// listing**, so reading traffic stays one session per connection whatever a
/// folder holds — which is what makes *the reading never blocks the listing*
/// implementable without a second admission story: the four fields show first
/// and the version tag fills in as each prefix arrives behind them. The
/// ordering is `Server protocol and transfers` §1.1 item 2's — user-waited
/// readings before unwaited ones, no reading starved, no walk rewound —
/// **taken and consumed here rather than restated**.
///
/// **It is asynchronous because the fetch is the machine's**, and its real end
/// — [`crate::wiring::TheReadingChannel`] — is the subsystem's own
/// byte-offset read against an open handle, taken on the same channel value as
/// the listing so that *rides the reading channel and is serialised behind the
/// listing* is a property of the code rather than a caller's discipline.
pub trait ReadsAPrefix {
    /// Bytes from the front of the file, at most `want` of them. **Fewer is a
    /// short file and not a failure.**
    fn read_the_front_of(
        &mut self,
        path: &str,
        want: usize,
    ) -> impl std::future::Future<Output = Result<Vec<u8>, PrefixUnavailable>> + Send;
}

/// The prefix could not be read at all. **The tag is blank and nothing is
/// guessed** — the same ending L8 gives an unrecognised shape.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PrefixUnavailable;

/// **L7's own declaration, stated as a constant so the declaration table and a
/// test can cite it.**
///
/// *"Carrying no priority class because §11 declares one and a prefix read is
/// not a transfer."*
pub const A_PREFIX_READ_IS_NOT_A_TRANSFER_AND_CARRIES_NO_PRIORITY_CLASS: bool = true;

/// A version read out of a file's opening text.
///
/// **The whole of what establishes the digits is two observed opening texts.**
/// No version's documentation describes a `.blend`'s inside (§13 item 2,
/// permanently unresolved), so nothing here is documented and nothing here may
/// be widened by argument.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SavedWithVersion {
    pub major: u32,
    pub minor: u32,
}

impl SavedWithVersion {
    /// What the tag shows. Rule 9: the standard form, `4.5`, and never a
    /// spelling of the digits as the file holds them.
    pub fn as_a_tag(self) -> String {
        format!("{}.{}", self.major, self.minor)
    }
}

/// **L8: two opening shapes are recognised, and both are observed rather than
/// documented.**
///
/// Blender 4.5 writes `BLENDER-v405REND`; 5.0, 5.1 and 5.2 write
/// `BLENDER17-01v0500REND` with the version digits moving — label:
/// **observed**, the same in all four versions on both machines (*Reading a
/// file 1*).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OpeningShape {
    /// `BLENDER-v` + three digits + `REND`.
    TheFourFiveShape,
    /// `BLENDER17-01v` + four digits + `REND`.
    TheFiveShape,
}

/// The head of the 4.5 shape, exactly as observed.
const FOUR_FIVE_HEAD: &[u8] = b"BLENDER-v";
/// The head of the 5.x shape, exactly as observed.
const FIVE_HEAD: &[u8] = b"BLENDER17-01v";
/// The tail both observed texts carry.
const TAIL: &[u8] = b"REND";

/// **Recognise an opening text, and take the version from it.**
///
/// **The two heads are matched exactly and are not widened**, and that is a
/// decision rather than an oversight. `BLENDER-v405REND` is the whole of what
/// the research carries for the first shape; matching *any* two bytes between
/// `BLENDER` and the digits would read a version out of a header shape nobody
/// has ever observed, which is the guess L8 forbids. **An unrecognised shape
/// covers a future Blender's header as much as a damaged file** — 5.0's release
/// notes say outright that parsers have to be updated when the header changes —
/// **and a blank tag is the honest answer for both** (§13 item 2's *Holds*).
///
/// **How the digits become a version.** The first shape's three digits are one
/// major digit and two minor; the second shape's four are two and two. **That
/// rule's whole evidential basis is the two observed pairs** — `405` for 4.5
/// and `0500` for 5.0 — **and nothing publishes it.** The two pairs are a test
/// below rather than a derivation, so a change to the rule fails there first.
pub fn recognise(prefix: &[u8]) -> Option<(OpeningShape, SavedWithVersion)> {
    if let Some(version) = take(prefix, FOUR_FIVE_HEAD, 3, 1) {
        return Some((OpeningShape::TheFourFiveShape, version));
    }
    if let Some(version) = take(prefix, FIVE_HEAD, 4, 2) {
        return Some((OpeningShape::TheFiveShape, version));
    }
    None
}

fn take(
    prefix: &[u8],
    head: &[u8],
    digits: usize,
    major_digits: usize,
) -> Option<SavedWithVersion> {
    let rest = prefix.strip_prefix(head)?;
    if rest.len() < digits {
        return None;
    }
    let (version_digits, rest) = rest.split_at(digits);
    if !version_digits.iter().all(u8::is_ascii_digit) {
        return None;
    }
    if !rest.starts_with(TAIL) {
        return None;
    }
    let read =
        |bytes: &[u8]| -> Option<u32> { std::str::from_utf8(bytes).ok()?.parse::<u32>().ok() };
    Some(SavedWithVersion {
        major: read(&version_digits[..major_digits])?,
        minor: read(&version_digits[major_digits..])?,
    })
}

/// **L8's attempt order, and it is an attempt order rather than a version
/// model.**
///
/// From 5.0 an ordinary save is compressed with Zstandard, which every
/// version's manual says outside tools can undo — label: **documented** — and a
/// 4.5 save can be compressed too, while a 5.x save can be plain where the
/// preference was changed. **So: read the prefix; where it opens with a
/// recognised Blender opening text, take the version from it; otherwise hand
/// the prefix to Smoozy's own streaming Zstandard decode and read the opening
/// text out of the first decompressed bytes; where neither yields a recognised
/// shape, the listing shows no version and never a guess.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tag {
    /// The version, from a recognised opening text.
    SavedWith(SavedWithVersion),
    /// **Blank. Never a guess.**
    Blank(WhyItIsBlank),
}

/// Why a tag is blank — carried so the reason is legible rather than so
/// anything is decided from it. **Every one of these shows the same blank
/// tag.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhyItIsBlank {
    /// Neither the plain prefix nor the decompressed one opened with a
    /// recognised shape.
    NoRecognisedOpeningShape,
    /// **L9's cap was reached while the decode still wanted more input.**
    ThePrefixCapWasReached,
    /// The prefix could not be read from the machine at all.
    ThePrefixCouldNotBeRead,
}

/// Smoozy's own streaming Zstandard decode, behind a trait because it is a
/// reading: the judgement is here and the real end is [`crate::wiring`]'s.
///
/// **§13 item 10 is met here rather than worked around.** L9's growth *"assumes
/// a streaming Zstandard decode yields the first decompressed bytes from a
/// bounded compressed prefix"*, which the entry said was *"not established for
/// any particular implementation"*. **What is built is the shape that makes the
/// answer a cost rather than a correctness question**: a decode that wants more
/// input gets more, up to the cap, and past the cap the tag is blank.
///
/// **The entry is now CLOSED, and this shape is what stands.** The reading it
/// named was taken on 2026-09-04 and it settled the answer per implementation
/// rather than in general — the C library reaches the opening text from 16 to 43
/// bytes of prefix, a pure-Rust one can want the whole file — so the decoder
/// this crate pins is what decides the cost, and on the one pinned the cap is
/// never reached. **Nothing here moved for it**, which is the point of building
/// the shape rather than a number: see [`crate::wiring`] for the quotations and
/// the measurement.
///
/// *(This paragraph said the entry carries no permanence line and is a question
/// a publication can answer. Both were true, the question was answered, and the
/// entry is closed. Corrected 2026-09-04.)*
pub trait DecodesZstandard {
    /// Feed the compressed prefix; answer with the first decompressed bytes,
    /// or say more input is wanted.
    fn first_decompressed_bytes(&mut self, compressed_prefix: &[u8], want: usize) -> Decoded;
}

/// What a streaming decode answered.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Decoded {
    /// The first decompressed bytes, however many came out.
    Bytes(Vec<u8>),
    /// **The decode wants more input before it yields anything.** L9's growth
    /// is what answers this.
    MoreInputWanted,
    /// The bytes are not this format at all. **Nothing is inferred from that
    /// beyond a blank tag** — it is not evidence the file is damaged, and no
    /// message is composed from it.
    NotThisFormat,
}

/// **L9: the prefix is grown, never sized by a claim about the compression.**
///
/// *"The first fetch is small; where the streaming decode wants more input
/// before yielding the opening bytes, the next piece is fetched, up to a fixed
/// cap, past which the tag is blank."*
///
/// **The three numbers are Build's and no promise rests on any of them.** The
/// first fetch is four kilobytes, which reaches the twenty-one bytes of the
/// longer observed opening text several hundred times over; each further fetch
/// doubles; and the cap is 256 kilobytes, past which the tag is blank. **A
/// larger cap buys a tag on a file whose compression happens to be shaped
/// unusually, and costs a bounded read on every file in a folder that has
/// none.** Nothing reads these from anywhere and no setting exposes them.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PrefixGrowth {
    pub first_fetch: usize,
    pub cap: usize,
}

impl PrefixGrowth {
    pub const DECLARED: PrefixGrowth = PrefixGrowth {
        first_fetch: 4 * 1024,
        cap: 256 * 1024,
    };

    /// The next size to ask for, or `None` where the cap has been reached.
    pub fn next_after(self, already: usize) -> Option<usize> {
        if already >= self.cap {
            return None;
        }
        Some((already.saturating_mul(2)).min(self.cap))
    }
}

/// **The whole of L8 and L9, as one reading.**
///
/// Nothing here opens a connection: the two readings are the traits above.
///
/// **The reader is a type parameter because the prefix comes off a machine**,
/// so its reading is asynchronous and has no object-safe form. **The decoder is
/// one too, and may still be a trait object** (`D` is unsized-permitting), so a
/// caller holding `&mut dyn DecodesZstandard` passes it as before.
///
/// *Changed 2026-10-01, step 10c's twelfth-pass fix piece 6b.* **The case this
/// was first written for** is a reading driven where it was awaited, which is
/// every test here. **The case it now also meets** is the application's own
/// call, which runs the reading on the server's point — and the point takes
/// only a reading that can cross threads. A `&mut dyn DecodesZstandard` held
/// across the reading's waits could not, so the tag was never read in the
/// running application and every listing row went without one. A caller
/// passing [`crate::wiring::TheZstandardDecoder`] by its own type now can.
pub async fn read_the_tag<R: ReadsAPrefix, D: DecodesZstandard + ?Sized>(
    path: &str,
    reader: &mut R,
    decoder: &mut D,
    growth: PrefixGrowth,
) -> Tag {
    let mut want = growth.first_fetch;
    loop {
        let prefix = match reader.read_the_front_of(path, want).await {
            Ok(prefix) => prefix,
            Err(PrefixUnavailable) => return Tag::Blank(WhyItIsBlank::ThePrefixCouldNotBeRead),
        };

        // First attempt: the plain bytes.
        if let Some((_, version)) = recognise(&prefix) {
            return Tag::SavedWith(version);
        }

        // Second attempt: Smoozy's own streaming decode.
        match decoder.first_decompressed_bytes(&prefix, TAIL_REACHING_BYTES) {
            Decoded::Bytes(decompressed) => {
                return match recognise(&decompressed) {
                    Some((_, version)) => Tag::SavedWith(version),
                    None => Tag::Blank(WhyItIsBlank::NoRecognisedOpeningShape),
                };
            }
            Decoded::NotThisFormat => return Tag::Blank(WhyItIsBlank::NoRecognisedOpeningShape),
            Decoded::MoreInputWanted => {}
        }

        // A short file cannot be grown: asking for more of it returns the same
        // bytes for ever, so the loop would not end.
        if prefix.len() < want {
            return Tag::Blank(WhyItIsBlank::NoRecognisedOpeningShape);
        }

        match growth.next_after(want) {
            Some(more) => want = more,
            None => return Tag::Blank(WhyItIsBlank::ThePrefixCapWasReached),
        }
    }
}

/// How many decompressed bytes the recognition needs — the longer observed
/// opening text is twenty-one bytes, and this asks for a round number above it
/// rather than for exactly that, because a decode yielding *at least* the
/// opening text is what the attempt order wants.
const TAIL_REACHING_BYTES: usize = 64;

/// **L10: the reading is cached for the connected period, keyed by the path and
/// the listing's own size-and-time values, and re-taken where those moved.**
///
/// **It is a live reading**, so `Servers` §15's rule drops it wherever that rule
/// drops one: the tag rides L14's entries and ends with them (L16). **Nothing
/// durable is written from it, and a fresh period reads afresh.**
///
/// **Limits carried with the answer** (§13 item 2): no version's documentation
/// describes the file's inside, so what the opening text holds rests on the
/// tests; nothing outside Blender was ever pointed at a `.blend` in the
/// research — this reading is exactly that, built on the observed opening text
/// alone; and Windows was never tested, which does not reach this reading, it
/// being byte arithmetic on the desktop, but is carried because the research
/// names it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TagKey {
    pub path: String,
    pub size: ApparentSize,
    pub last_changed: Option<LastChanged>,
}

/// The tags read in this connected period, keyed as L10 says.
///
/// **Process memory and durable nowhere.** There is no write route out of this
/// type.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct TagsThisPeriod {
    tags: Vec<(TagKey, Tag)>,
}

impl TagsThisPeriod {
    pub fn new() -> TagsThisPeriod {
        TagsThisPeriod { tags: Vec::new() }
    }

    /// **Re-taken where the path's size or time moved**, which is what the key
    /// being all three values does: a moved value simply misses.
    pub fn tag_for(&self, key: &TagKey) -> Option<Tag> {
        self.tags
            .iter()
            .find(|(held, _)| held == key)
            .map(|(_, tag)| *tag)
    }

    pub fn remember(&mut self, key: TagKey, tag: Tag) {
        self.tags.retain(|(held, _)| held.path != key.path);
        self.tags.push((key, tag));
    }

    /// **L16: the tag rides the listing tree's entries and ends with them.**
    /// This is the whole of what a period ending does to it.
    pub fn the_period_ended(&mut self) {
        self.tags.clear();
    }

    pub fn how_many_are_held(&self) -> usize {
        self.tags.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct APrefix {
        bytes: Vec<u8>,
        reads: usize,
    }

    impl ReadsAPrefix for APrefix {
        async fn read_the_front_of(
            &mut self,
            _path: &str,
            want: usize,
        ) -> Result<Vec<u8>, PrefixUnavailable> {
            self.reads += 1;
            Ok(self.bytes.iter().copied().take(want).collect())
        }
    }

    struct NoDecode;

    impl DecodesZstandard for NoDecode {
        fn first_decompressed_bytes(&mut self, _compressed: &[u8], _want: usize) -> Decoded {
            Decoded::NotThisFormat
        }
    }

    /// A decode that wants more input until it has been handed at least
    /// `enough` bytes, and then yields the opening text.
    struct HungryDecode {
        enough: usize,
        yields: Vec<u8>,
    }

    impl DecodesZstandard for HungryDecode {
        fn first_decompressed_bytes(&mut self, compressed: &[u8], _want: usize) -> Decoded {
            if compressed.len() >= self.enough {
                Decoded::Bytes(self.yields.clone())
            } else {
                Decoded::MoreInputWanted
            }
        }
    }

    fn padded(head: &[u8], to: usize) -> Vec<u8> {
        let mut bytes = head.to_vec();
        bytes.resize(to, 0);
        bytes
    }

    #[test]
    fn the_two_observed_opening_texts_read_as_the_two_observed_versions() {
        // This is the evidential basis of the digit rule, held as a test so a
        // change to the rule fails here first.
        assert_eq!(
            recognise(b"BLENDER-v405REND"),
            Some((
                OpeningShape::TheFourFiveShape,
                SavedWithVersion { major: 4, minor: 5 }
            ))
        );
        assert_eq!(
            recognise(b"BLENDER17-01v0500REND"),
            Some((
                OpeningShape::TheFiveShape,
                SavedWithVersion { major: 5, minor: 0 }
            ))
        );
        assert_eq!(SavedWithVersion { major: 4, minor: 5 }.as_a_tag(), "4.5");
        assert_eq!(SavedWithVersion { major: 5, minor: 0 }.as_a_tag(), "5.0");
    }

    #[test]
    fn the_version_digits_move_inside_the_second_shape() {
        // L8's own words for 5.0, 5.1 and 5.2 — one text, the digits moving.
        assert_eq!(
            recognise(b"BLENDER17-01v0501REND").map(|(_, version)| version.as_a_tag()),
            Some("5.1".to_string())
        );
        assert_eq!(
            recognise(b"BLENDER17-01v0502REND").map(|(_, version)| version.as_a_tag()),
            Some("5.2".to_string())
        );
    }

    #[test]
    fn an_unobserved_head_is_not_read_as_a_version() {
        // The widening that was declined: two bytes anybody could match.
        assert_eq!(recognise(b"BLENDER_V405REND"), None);
        // A future head.
        assert_eq!(recognise(b"BLENDER18-02v0600REND"), None);
        // A damaged file, and a file that is not a `.blend` at all.
        assert_eq!(recognise(b"BLENDER-vXYZREND"), None);
        assert_eq!(recognise(b"\x89PNG\r\n\x1a\n"), None);
        assert_eq!(recognise(b""), None);
        // The tail is part of the shape.
        assert_eq!(recognise(b"BLENDER-v405"), None);
        assert_eq!(recognise(b"BLENDER-v405XXXX"), None);
    }

    #[tokio::test]
    async fn a_plain_save_is_read_without_the_decode_being_asked() {
        let mut reader = APrefix {
            bytes: padded(b"BLENDER-v405REND", 8_000),
            reads: 0,
        };
        let tag = read_the_tag(
            "/jobs/a.blend",
            &mut reader,
            &mut NoDecode,
            PrefixGrowth::DECLARED,
        )
        .await;
        assert_eq!(tag, Tag::SavedWith(SavedWithVersion { major: 4, minor: 5 }));
        assert_eq!(reader.reads, 1);
    }

    #[tokio::test]
    async fn a_compressed_save_is_read_out_of_the_decode() {
        let mut reader = APrefix {
            bytes: padded(b"\x28\xb5\x2f\xfd", 64_000),
            reads: 0,
        };
        let mut decoder = HungryDecode {
            enough: 0,
            yields: b"BLENDER17-01v0500REND".to_vec(),
        };
        let tag = read_the_tag(
            "/jobs/a.blend",
            &mut reader,
            &mut decoder,
            PrefixGrowth::DECLARED,
        )
        .await;
        assert_eq!(tag, Tag::SavedWith(SavedWithVersion { major: 5, minor: 0 }));
    }

    #[tokio::test]
    async fn a_decode_that_wants_more_gets_more_up_to_the_cap() {
        let mut reader = APrefix {
            bytes: padded(b"\x28\xb5\x2f\xfd", 512 * 1024),
            reads: 0,
        };
        let mut decoder = HungryDecode {
            // More than the first fetch, less than the cap: the growth reaches
            // it, so this is a cost and not a wrong answer.
            enough: 40 * 1024,
            yields: b"BLENDER17-01v0502REND".to_vec(),
        };
        let tag = read_the_tag(
            "/jobs/a.blend",
            &mut reader,
            &mut decoder,
            PrefixGrowth::DECLARED,
        )
        .await;
        assert_eq!(tag, Tag::SavedWith(SavedWithVersion { major: 5, minor: 2 }));
        // 4 KiB, 8, 16, 32, 64 — five reads, and the loop did not run away.
        assert_eq!(reader.reads, 5);
    }

    #[tokio::test]
    async fn past_the_cap_the_tag_is_blank_and_never_a_guess() {
        let mut reader = APrefix {
            bytes: padded(b"\x28\xb5\x2f\xfd", 4 * 1024 * 1024),
            reads: 0,
        };
        let mut decoder = HungryDecode {
            enough: 1024 * 1024,
            yields: b"BLENDER17-01v0500REND".to_vec(),
        };
        let tag = read_the_tag(
            "/jobs/a.blend",
            &mut reader,
            &mut decoder,
            PrefixGrowth::DECLARED,
        )
        .await;
        assert_eq!(tag, Tag::Blank(WhyItIsBlank::ThePrefixCapWasReached));
    }

    #[tokio::test]
    async fn a_short_file_ends_the_growth_rather_than_looping() {
        let mut reader = APrefix {
            bytes: b"not a blend".to_vec(),
            reads: 0,
        };
        let mut decoder = HungryDecode {
            enough: usize::MAX,
            yields: Vec::new(),
        };
        let tag = read_the_tag(
            "/jobs/a.txt",
            &mut reader,
            &mut decoder,
            PrefixGrowth::DECLARED,
        )
        .await;
        assert_eq!(tag, Tag::Blank(WhyItIsBlank::NoRecognisedOpeningShape));
        assert_eq!(reader.reads, 1);
    }

    #[tokio::test]
    async fn a_prefix_that_cannot_be_read_shows_a_blank_tag_and_nothing_else() {
        struct Refuses;
        impl ReadsAPrefix for Refuses {
            async fn read_the_front_of(
                &mut self,
                _path: &str,
                _want: usize,
            ) -> Result<Vec<u8>, PrefixUnavailable> {
                Err(PrefixUnavailable)
            }
        }
        let tag = read_the_tag(
            "/jobs/a.blend",
            &mut Refuses,
            &mut NoDecode,
            PrefixGrowth::DECLARED,
        )
        .await;
        assert_eq!(tag, Tag::Blank(WhyItIsBlank::ThePrefixCouldNotBeRead));
    }

    #[test]
    fn the_growth_reaches_the_cap_exactly_and_then_stops() {
        let growth = PrefixGrowth::DECLARED;
        let mut at = growth.first_fetch;
        let mut steps = 0;
        while let Some(next) = growth.next_after(at) {
            at = next;
            steps += 1;
            assert!(steps < 100, "the growth does not end");
        }
        assert_eq!(at, growth.cap);
    }

    #[test]
    fn a_tag_is_re_taken_where_the_size_or_the_time_moved() {
        let mut held = TagsThisPeriod::new();
        let key = TagKey {
            path: "/jobs/a.blend".to_string(),
            size: ApparentSize::of(1_000),
            last_changed: Some(LastChanged::at_epoch_seconds(10)),
        };
        held.remember(
            key.clone(),
            Tag::SavedWith(SavedWithVersion { major: 4, minor: 5 }),
        );
        assert!(held.tag_for(&key).is_some());

        let saved_again = TagKey {
            size: ApparentSize::of(1_200),
            ..key.clone()
        };
        assert_eq!(held.tag_for(&saved_again), None);

        let touched = TagKey {
            last_changed: Some(LastChanged::at_epoch_seconds(11)),
            ..key
        };
        assert_eq!(held.tag_for(&touched), None);
    }

    #[test]
    fn a_period_ending_takes_every_tag_with_it() {
        let mut held = TagsThisPeriod::new();
        held.remember(
            TagKey {
                path: "/jobs/a.blend".to_string(),
                size: ApparentSize::of(1),
                last_changed: None,
            },
            Tag::Blank(WhyItIsBlank::NoRecognisedOpeningShape),
        );
        assert_eq!(held.how_many_are_held(), 1);
        held.the_period_ended();
        assert_eq!(held.how_many_are_held(), 0);
    }
}
