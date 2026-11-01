//! **B4's fetch, performed: the archive on the machine's own line, staged,
//! resumable, and checked against the published checksum.**
//!
//! **This is deliberately not a `Server protocol and transfers` §7 transfer**
//! and the mechanism is not borrowed. The bytes never pass through Smoozy's own
//! byte loop, and that loop is where §7's checkpoint digests come from (S22),
//! with the checkpoint record kept per transfer on the desktop side. **Neither
//! exists here, so §7's prefix verification is not available and is not
//! claimed. The checksum is what is checked instead**, which is what
//! [`crate::blender::WHAT_THE_FETCH_HONOURS`] already states as a judgement and
//! what this file performs.
//!
//! **A builder who routes this fetch through the desktop has built B7 — the
//! user's own build sent up — and not B4.** Nothing here touches a connection.
//!
//! **Markers used in this file that belong to another document, each with the
//! document it belongs to.** `Server protocol and transfers` §7.1, S9, S12,
//! S22, S24.

use std::io::{Read, Write};
use std::path::{Path, PathBuf};

use sha2::{Digest, Sha256};

use crate::acting::{NobodyIsTold, TellsHowFarItHasGot};
use crate::blender::{
    a_bare_build_service_name, the_archives_path, the_checksums_path, TheVersionAsked, Version,
    THE_BUILD_SERVICES_DOWNLOADS,
};

/// **Where Blender's own downloads are**, and every address this file composes
/// begins here.
///
/// **Observed 2026-08-19**, and recorded in
/// `Preparation/Architecture/Sources/Server Setup.sources.md` under *Shape 2,
/// publication*: `https://download.blender.org/release/` answers a plain
/// directory index (`Index of /release/`) with one folder per release line, all
/// HTTP 200. B4's derived paths sit under it.
///
/// **It is a constant of this build and never arrives from anywhere.** B4 says
/// the fetch *"stays on Blender's own downloads"*; a base that could be sent in
/// would let whatever composed a request point the machine's own line wherever
/// it liked, and no clause anywhere grants that.
pub const THE_DOWNLOADS: &str = "https://download.blender.org/release/";

/// **B4's archive address**, composed from the version and nothing else.
pub fn the_archives_address(version: Version) -> String {
    format!("{THE_DOWNLOADS}{}", the_archives_path(version))
}

/// **B4's checksum file**, beside the archive in the same folder.
pub fn the_checksums_address(version: Version) -> String {
    format!("{THE_DOWNLOADS}{}", the_checksums_path(version))
}

/// **B4's second route: an unfinished build's address on Blender's build
/// service**, composed from that service's own constant base and the bare name
/// the listing gave — **and nothing where the name is not a bare name of the one
/// archive form**, so no request is ever made to an address that arrived from
/// outside. Added 2026-10-02, the Creator's notes 5 and 7 on 0.1.7.
pub fn the_build_services_address(name: &str) -> Option<String> {
    a_bare_build_service_name(name).then(|| format!("{THE_BUILD_SERVICES_DOWNLOADS}{name}"))
}

/// **B4: the build service's checksum sits at the archive's own address with
/// `.sha256` after it** — observed 2026-10-02.
pub fn the_build_services_checksum_address(name: &str) -> Option<String> {
    the_build_services_address(name).map(|address| format!("{address}.sha256"))
}

/// **B4: the build service's checksum is a bare digest** — one line, no file
/// name, observed 2026-10-02 (64 bytes, no line ending). **Sixty-four
/// hexadecimal characters and nothing else**, with surrounding blank space
/// allowed; any other body is a checksum this reader does not understand and
/// is not guessed at.
pub fn the_bare_digest_in(text: &str) -> Option<String> {
    let digest = text.trim();
    (digest.len() == 64 && digest.bytes().all(|byte| byte.is_ascii_hexdigit()))
        .then(|| digest.to_ascii_lowercase())
}

/// **The archive's own file name**, which is what the checksum file names its
/// entries by.
pub fn the_archives_file_name(version: Version) -> String {
    format!(
        "blender-{}.{}.{}-linux-x64.tar.xz",
        version.major, version.minor, version.patch
    )
}

/// **What the far end gave**, which decides whether a resume resumed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhatTheLineGave {
    /// **The whole file follows**, whatever byte the request asked from. A
    /// resume that gets this starts the staged file over rather than appending,
    /// because appending a whole file to a prefix composes a file that is
    /// neither.
    TheWholeFile,
    /// **The byte range asked for follows** — B4's *"the downloads honour byte
    /// ranges at `http_code=206`"*.
    TheRangeAsked,
    /// **Anything else.** The status rides so the ending can name it, and
    /// nothing is written.
    Neither { status: u16 },
}

/// **Where the bytes of an answer land.**
///
/// **The answer is announced before the first byte**, which is what lets a
/// resume that was not honoured start its staged file over rather than append a
/// whole file to a prefix. **A caller of the trait below must call
/// [`TheBytesLand::the_answer_is`] once, before [`TheBytesLand::take`]**, and
/// the staged sink refuses a byte it was not told about.
pub trait TheBytesLand {
    /// **Once, before any byte.**
    fn the_answer_is(&mut self, gave: WhatTheLineGave) -> Result<(), String>;
    /// **A piece of the body, in arrival order.**
    fn take(&mut self, bytes: &[u8]) -> Result<(), String>;
    /// **How long the body is, where the answer announced it** — once, after
    /// [`TheBytesLand::the_answer_is`] and before any byte, and not at all
    /// where the answer announced no length. It is what Y7b's whole is read
    /// from. **A sink that has no use for it ignores it.** Added 2026-10-18,
    /// the Creator's fourteenth notes, piece U.
    fn the_body_is(&mut self, _bytes: u64) {}
}

/// **The one thing this crate cannot do itself: a request on the machine's own
/// line.**
///
/// **Its real end is the machine's program**, which carries the workspace's one
/// HTTPS client on the runner's R1 ruling of 2026-09-19. **At the desk it is a
/// stand-in over a local file server**, named as one — no test here reaches
/// Blender's downloads.
pub trait MakesTheRequestOnTheMachinesOwnLine {
    /// **Ask for `address` from `from_byte` on, handing what comes back to
    /// `into`.**
    ///
    /// **Nothing is buffered whole in memory**: an archive is about 380 MB
    /// (`Server Setup` R1 item 3's two timed fetches were 377 902 364 and
    /// 384 441 228 bytes), and a machine being paid for has better uses for
    /// that.
    ///
    /// **`the_answer_is` is called before any byte and the same answer is
    /// returned**, so the sink and the caller cannot disagree about what the
    /// far end gave.
    fn fetch_into(
        &self,
        address: &str,
        from_byte: u64,
        into: &mut dyn TheBytesLand,
    ) -> Result<WhatTheLineGave, String>;

    /// **The same, for something small enough to hold** — the checksum file is
    /// one short line per artefact.
    fn fetch_whole(&self, address: &str) -> Result<(WhatTheLineGave, Vec<u8>), String> {
        let mut held = TheBytesHeld::default();
        let gave = self.fetch_into(address, 0, &mut held)?;
        Ok((gave, held.bytes))
    }
}

/// **A sink that holds what it is given**, for an answer small enough.
#[derive(Debug, Default)]
pub struct TheBytesHeld {
    pub bytes: Vec<u8>,
}

impl TheBytesLand for TheBytesHeld {
    fn the_answer_is(&mut self, _gave: WhatTheLineGave) -> Result<(), String> {
        Ok(())
    }

    fn take(&mut self, bytes: &[u8]) -> Result<(), String> {
        self.bytes.extend_from_slice(bytes);
        Ok(())
    }
}

/// **The published checksum for one named file, out of the checksum file's own
/// text.**
///
/// **The format is one entry per line: the digest, spacing, then the file
/// name.** The entry is found **by the file name B4 derives** and never by
/// position, because the file holds an entry per artefact of that release and
/// their order is not this project's to rely on.
///
/// **A digest is returned only where it is 64 hexadecimal characters**, which
/// is what a SHA-256 is written as; anything else is a line this reader does not
/// understand and is not guessed at.
pub fn the_checksum_for(text: &str, file_name: &str) -> Option<String> {
    for line in text.lines() {
        let mut parts = line.split_whitespace();
        // A blank or one-word line is a line this reader does not understand,
        // and it stops nothing: the entry wanted may be below it.
        let (Some(digest), Some(named)) = (parts.next(), parts.next()) else {
            continue;
        };
        // Some checksum files mark the mode with a leading `*` on the name.
        let named = named.trim_start_matches('*');
        if named != file_name {
            continue;
        }
        if digest.len() != 64 || !digest.bytes().all(|byte| byte.is_ascii_hexdigit()) {
            return None;
        }
        return Some(digest.to_ascii_lowercase());
    }
    None
}

/// **The SHA-256 of a file on the disk, as sixty-four lower-case hexadecimal
/// characters** — the form the published checksum is written in.
///
/// **It is read in pieces**, for the reason
/// [`MakesTheRequestOnTheMachinesOwnLine::fetch_into`] gives.
pub fn the_digest_of(at: &Path) -> Result<String, String> {
    let mut file = std::fs::File::open(at).map_err(|why| format!("{}: {why}", at.display()))?;
    let mut digest = Sha256::new();
    let mut held = vec![0u8; 1 << 16];
    loop {
        let read = file
            .read(&mut held)
            .map_err(|why| format!("{}: {why}", at.display()))?;
        if read == 0 {
            break;
        }
        digest.update(&held[..read]);
    }
    Ok(digest
        .finalize()
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect())
}

/// **Where this run's archive is staged.**
///
/// **It is a name under the staging area the run was given and never a final
/// name** (§7.1: part-arrived bytes never sit at the final name). **The name is
/// the run's own**, so two runs on one machine never write into one another's
/// bytes, and a resume finds the same path a first attempt left.
pub fn where_the_archive_is_staged(staging_area: &Path, run: &str, version: Version) -> PathBuf {
    staging_area.join(format!("{run}-{}", the_archives_file_name(version)))
}

/// **What a fetch that landed leaves.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TheFetchLanded {
    /// Where the whole, checked archive sits.
    pub staged_at: PathBuf,
    /// How many bytes it is.
    pub bytes: u64,
    /// **Whether this fetch resumed an earlier one** — recorded because it is
    /// the half of B4 nothing else would show.
    pub it_resumed: bool,
}

/// **Why a fetch did not land.** Each is said rather than collapsed into one
/// failure, because the run's ending carries the reason to the user
/// (S2a).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TheFetchDidNot {
    /// **The checksum file did not answer**, or answered nothing this reader
    /// understands. **No fetch is attempted without one**: B4 verifies the whole
    /// before anything is unpacked, and a fetch with nothing to verify against
    /// could only end by being trusted.
    TheChecksumWasNotPublished { why: String },
    /// **The derived address has no file** — B5's observed case, on the
    /// unstable kinds. *"Smoozy says the build could not be fetched and names
    /// it, and never quietly substitutes a different version."*
    TheArchiveWasNotThere { status: u16 },
    /// The line, or the disk, said so in its own words.
    ItWasRefused { why: String },
    /// **The whole arrived and its digest is not the published one.** The staged
    /// material is abandoned — never repaired in place (S24) — so the step's
    /// re-run starts over into fresh staging.
    TheDigestDidNotMatch {
        published: String,
        what_arrived: String,
        the_staged_material_was_abandoned: bool,
    },
}

/// **B4, performed: the archive fetched on the machine's own line, staged,
/// resumable, and verified against the published checksum before anything is
/// unpacked.**
///
/// The order, and each part of it is one of §7.1's four contract clauses:
///
/// 1. **The published checksum is fetched first.** Without it there is nothing
///    to verify against, and a fetch that cannot be verified is not started.
/// 2. **The staged path is looked at.** Bytes already there are a prefix of this
///    same archive under this same run, so the request asks from their end —
///    *"a resume is by byte range into the same staging path"*.
/// 3. **What came back decides whether it appends.** At the range asked for it
///    appends; at the whole file it starts the staged file over, because the
///    far end ignored the range and what follows is not a continuation.
/// 4. **The whole staged file is digested and compared.** On a mismatch the
///    staged material is abandoned rather than repaired (S24).
///
/// **Nothing half-fetched is ever unpacked**, which is the whole point of
/// keeping the check here rather than at the unpack.
pub fn the_archive_is_fetched(
    line: &dyn MakesTheRequestOnTheMachinesOwnLine,
    staged_at: &Path,
    version: Version,
) -> Result<TheFetchLanded, TheFetchDidNot> {
    the_archive_is_fetched_telling(line, staged_at, version, &NobodyIsTold)
}

/// **[`the_archive_is_fetched`], telling how far it has got as it goes** —
/// Y7b: the bytes written to the staged archive, a resume counting what was
/// already staged, beside the whole where the answer announced a length.
pub fn the_archive_is_fetched_telling(
    line: &dyn MakesTheRequestOnTheMachinesOwnLine,
    staged_at: &Path,
    version: Version,
    told: &dyn TellsHowFarItHasGot,
) -> Result<TheFetchLanded, TheFetchDidNot> {
    let published = the_published_checksum(line, &the_checksums_address(version), |text| {
        the_checksum_for(text, &the_archives_file_name(version)).ok_or_else(|| {
            format!("the checksum file has no checksum for {}", the_archives_file_name(version))
        })
    })?;
    the_staged_archive_is_fetched(line, staged_at, &the_archives_address(version), published, told)
}

/// **B4, either route: the build the step asked for**, from Blender's release
/// folders where the version alone names it, and from Blender's build service
/// where the step carried that service's bare file name. **A name that is not
/// bare is refused before any request.** Added 2026-10-02, the Creator's notes
/// 5 and 7 on 0.1.7.
pub fn the_build_is_fetched(
    line: &dyn MakesTheRequestOnTheMachinesOwnLine,
    staged_at: &Path,
    asked: &TheVersionAsked,
) -> Result<TheFetchLanded, TheFetchDidNot> {
    the_build_is_fetched_telling(line, staged_at, asked, &NobodyIsTold)
}

/// **[`the_build_is_fetched`], telling how far it has got as it goes**, by
/// either route — Y7b. Added 2026-10-18, the Creator's fourteenth notes,
/// piece U.
pub fn the_build_is_fetched_telling(
    line: &dyn MakesTheRequestOnTheMachinesOwnLine,
    staged_at: &Path,
    asked: &TheVersionAsked,
    told: &dyn TellsHowFarItHasGot,
) -> Result<TheFetchLanded, TheFetchDidNot> {
    let Some(name) = asked.the_build_services_file.as_deref() else {
        return the_archive_is_fetched_telling(line, staged_at, asked.version, told);
    };
    let (Some(address), Some(checksum)) =
        (the_build_services_address(name), the_build_services_checksum_address(name))
    else {
        return Err(TheFetchDidNot::ItWasRefused {
            why: format!("{name} is not a Linux build archive name. Nothing was downloaded."),
        });
    };
    let published = the_published_checksum(line, &checksum, |text| {
        the_bare_digest_in(text).ok_or_else(|| "the checksum file is not a single checksum".to_string())
    })?;
    the_staged_archive_is_fetched(line, staged_at, &address, published, told)
}

/// **The published checksum, fetched first and read by the route's own
/// reader.** Without it there is nothing to verify against, and a fetch that
/// cannot be verified is not started.
fn the_published_checksum(
    line: &dyn MakesTheRequestOnTheMachinesOwnLine,
    address: &str,
    read: impl FnOnce(&str) -> Result<String, String>,
) -> Result<String, TheFetchDidNot> {
    let (gave, text) = line
        .fetch_whole(address)
        .map_err(|why| TheFetchDidNot::TheChecksumWasNotPublished { why })?;
    if let WhatTheLineGave::Neither { status } = gave {
        return Err(TheFetchDidNot::TheChecksumWasNotPublished {
            why: format!("the checksum file returned HTTP {status}"),
        });
    }
    read(&String::from_utf8_lossy(&text)).map_err(|why| TheFetchDidNot::TheChecksumWasNotPublished { why })
}

/// **Steps 2 to 4 of the order above, at one address against one published
/// digest**, which both routes share.
fn the_staged_archive_is_fetched(
    line: &dyn MakesTheRequestOnTheMachinesOwnLine,
    staged_at: &Path,
    archive: &str,
    published: String,
    told: &dyn TellsHowFarItHasGot,
) -> Result<TheFetchLanded, TheFetchDidNot> {
    if let Some(area) = staged_at.parent() {
        std::fs::create_dir_all(area)
            .map_err(|why| TheFetchDidNot::ItWasRefused { why: format!("{}: {why}", area.display()) })?;
    }
    let already = std::fs::metadata(staged_at).map(|it| it.len()).unwrap_or(0);
    let mut held = std::fs::OpenOptions::new()
        .create(true)
        .read(true)
        .write(true)
        .open(staged_at)
        .map_err(|why| TheFetchDidNot::ItWasRefused {
            why: format!("{}: {why}", staged_at.display()),
        })?;
    let gave = {
        // **The far end decides where the write goes**, so the sink is told
        // what it gave before it is given a byte.
        let mut into = TheStagedFile {
            file: &mut held,
            already,
            writing_from: None,
            written_to: 0,
            whole: None,
            told,
        };
        line.fetch_into(archive, already, &mut into)
            .map_err(|why| TheFetchDidNot::ItWasRefused { why })?
    };
    if let WhatTheLineGave::Neither { status } = gave {
        return Err(TheFetchDidNot::TheArchiveWasNotThere { status });
    }
    drop(held);

    let what_arrived = the_digest_of(staged_at)
        .map_err(|why| TheFetchDidNot::ItWasRefused { why })?;
    if what_arrived != published {
        let abandoned = std::fs::remove_file(staged_at).is_ok();
        return Err(TheFetchDidNot::TheDigestDidNotMatch {
            published,
            what_arrived,
            the_staged_material_was_abandoned: abandoned,
        });
    }
    let bytes = std::fs::metadata(staged_at)
        .map(|it| it.len())
        .map_err(|why| TheFetchDidNot::ItWasRefused {
            why: format!("{}: {why}", staged_at.display()),
        })?;
    Ok(TheFetchLanded {
        staged_at: staged_at.to_path_buf(),
        bytes,
        it_resumed: already > 0 && gave == WhatTheLineGave::TheRangeAsked,
    })
}

/// **The staged file as the line writes into it**, which is where the two
/// answers are told apart.
///
/// **The file is positioned when the answer arrives and not before**, because a
/// far end that answers `Neither` writes nothing and must not disturb what is
/// already staged.
///
/// **And it is where Y7b's count is kept**: the end of what the staged file
/// holds, which on a resume begins at what was already staged, beside the
/// whole — where the write began plus the body's announced length.
struct TheStagedFile<'a> {
    file: &'a mut std::fs::File,
    already: u64,
    writing_from: Option<u64>,
    /// Y7b: how many bytes the staged file holds now.
    written_to: u64,
    /// Y7b: the archive's whole size, where the answer announced a length.
    whole: Option<u64>,
    told: &'a dyn TellsHowFarItHasGot,
}

impl TheBytesLand for TheStagedFile<'_> {
    fn the_answer_is(&mut self, gave: WhatTheLineGave) -> Result<(), String> {
        use std::io::Seek;
        let from = if a_whole_file_answer_starts_the_staged_file_over(gave) {
            // The far end ignored the range. What follows is the whole file, so
            // the staged prefix is not a prefix of it.
            self.file
                .set_len(0)
                .map_err(|why| format!("Could not restart the download file: {why}"))?;
            0
        } else {
            self.already
        };
        self.file
            .seek(std::io::SeekFrom::Start(from))
            .map_err(|why| format!("Could not seek in the download file: {why}"))?;
        self.writing_from = Some(from);
        // **Y7b: a resume counts what was already staged**, so the figure
        // starts where the staged file does rather than at nothing. **It is
        // told with the first piece**, by which time the body's length has
        // been announced, so the first figure carries its whole.
        self.written_to = from;
        Ok(())
    }

    fn take(&mut self, bytes: &[u8]) -> Result<(), String> {
        if self.writing_from.is_none() {
            return Err("bytes arrived before the far end's answer was announced".to_string());
        }
        self.file
            .write_all(bytes)
            .map_err(|why| format!("Could not write the download file: {why}"))?;
        self.written_to += bytes.len() as u64;
        self.told.so_far(self.written_to, self.whole);
        Ok(())
    }

    fn the_body_is(&mut self, bytes: u64) {
        // **The whole is where the write began plus what follows**: a whole
        // file from nothing, a range from what was already staged.
        if let Some(from) = self.writing_from {
            self.whole = Some(from + bytes);
        }
    }
}

/// **The rule that tells the two answers apart**: where the far end answers
/// with the whole file, the staged prefix is not a prefix of what follows and
/// the file starts over.
///
/// **It is a named act and not folded into the sink** so that the rule can be
/// read and held to a test on its own.
pub fn a_whole_file_answer_starts_the_staged_file_over(gave: WhatTheLineGave) -> bool {
    matches!(gave, WhatTheLineGave::TheWholeFile)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn a_version() -> Version {
        Version {
            major: 5,
            minor: 2,
            patch: 0,
        }
    }

    /// **B4's two addresses, composed from the version and nothing else.**
    #[test]
    fn the_addresses_are_derived_from_the_version() {
        assert_eq!(
            the_archives_address(a_version()),
            "https://download.blender.org/release/Blender5.2/blender-5.2.0-linux-x64.tar.xz"
        );
        assert_eq!(
            the_checksums_address(a_version()),
            "https://download.blender.org/release/Blender5.2/blender-5.2.0.sha256"
        );
        assert_eq!(
            the_archives_file_name(a_version()),
            "blender-5.2.0-linux-x64.tar.xz"
        );
    }

    /// **The entry is found by name and never by position**, and a line this
    /// reader does not understand is not guessed at.
    #[test]
    fn the_published_checksum_is_read_by_the_file_name() {
        let published = "\
0000000000000000000000000000000000000000000000000000000000000000  blender-5.2.0-windows-x64.zip
1111111111111111111111111111111111111111111111111111111111111111 *blender-5.2.0-linux-x64.tar.xz
2222222222222222222222222222222222222222222222222222222222222222  blender-5.2.0-macos-arm64.dmg
";
        assert_eq!(
            the_checksum_for(published, "blender-5.2.0-linux-x64.tar.xz").as_deref(),
            Some("1111111111111111111111111111111111111111111111111111111111111111")
        );
        assert_eq!(the_checksum_for(published, "blender-5.2.0-linux-arm64.tar.xz"), None);
        assert_eq!(
            the_checksum_for("not-a-digest  blender-5.2.0-linux-x64.tar.xz", "blender-5.2.0-linux-x64.tar.xz"),
            None
        );
    }

    /// **B4's second route: the build service's address is its constant base and
    /// the bare name, its checksum is beside it with `.sha256`, and the checksum
    /// is a bare digest** — the Creator's notes 5 and 7 on 0.1.7, read
    /// 2026-10-02. **A name that is not bare composes nothing.**
    #[test]
    fn the_build_services_addresses_are_its_base_and_the_bare_name() {
        let alpha = "blender-5.3.0-alpha+main.64d181010e0d-linux.x86_64-release.tar.xz";
        assert_eq!(
            the_build_services_address(alpha).as_deref(),
            Some("https://cdn.builder.blender.org/download/daily/blender-5.3.0-alpha+main.64d181010e0d-linux.x86_64-release.tar.xz")
        );
        assert_eq!(
            the_build_services_checksum_address(alpha).as_deref(),
            Some("https://cdn.builder.blender.org/download/daily/blender-5.3.0-alpha+main.64d181010e0d-linux.x86_64-release.tar.xz.sha256")
        );
        assert_eq!(the_build_services_address("x/blender-5.3.0-linux.x86_64-release.tar.xz"), None);
        // **The body read 2026-10-02**: 64 bytes, no file name, no line ending.
        let published = "42ec085cccb6284e9c39103aa4e7ec7cc703f346ed64a381b4e89ef787dfeca3";
        assert_eq!(the_bare_digest_in(published).as_deref(), Some(published));
        assert_eq!(the_bare_digest_in(&format!("{published}\n")).as_deref(), Some(published));
        // **A release folder's per-file line is not a bare digest**, and the
        // release route's reader is not the build service's.
        assert_eq!(the_bare_digest_in(&format!("{published}  {alpha}")), None);
        assert_eq!(the_checksum_for(published, alpha), None);
    }

    /// **The digest is the published form**, sixty-four lower-case hexadecimal
    /// characters, and it is the digest of the empty file for an empty file.
    #[test]
    fn the_digest_is_sixty_four_hexadecimal_characters() {
        let its_own = crate::acting::its_own_folder("an-empty-file");
        let at = its_own.at().join("nothing-at-all");
        std::fs::write(&at, b"").expect("the file is written");
        let digest = the_digest_of(&at).expect("it reads");
        assert_eq!(digest.len(), 64);
        assert!(digest.bytes().all(|byte| byte.is_ascii_hexdigit() && !byte.is_ascii_uppercase()));
        // The published SHA-256 of nothing at all.
        assert_eq!(
            digest,
            "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
        );
    }

    /// **A whole-file answer to a range request starts the staged file over.**
    #[test]
    fn a_whole_file_answer_is_not_a_continuation() {
        assert!(a_whole_file_answer_starts_the_staged_file_over(
            WhatTheLineGave::TheWholeFile
        ));
        assert!(!a_whole_file_answer_starts_the_staged_file_over(
            WhatTheLineGave::TheRangeAsked
        ));
    }
}
