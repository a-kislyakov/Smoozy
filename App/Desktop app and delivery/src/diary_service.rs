//! The diary service — `Desktop app and delivery` §5.3.
//!
//! **L14: the diary service is a named service of Section 21 and the diary's
//! sole writer.** A module never opens the diary, never writes a line itself
//! and never holds a diary file handle; it hands the service an entry and the
//! service writes it. **A1 and A4 govern with no carve-out**: every byte the
//! service writes goes through the platform file adapter, and the service is
//! the only caller that writes into the diary area.
//!
//! **The diary's own rules are `Reporting a problem` §1, §3 and §4's,
//! implemented here and re-decided nowhere.** L15 names them clause by clause
//! and this module cites them where it keeps them.
//!
//! **How a clause marker is written in this crate, and it is a rule rather than
//! a habit.** A marker with no document beside it means a clause of this crate's
//! own plan, `Preparation/Architecture/Desktop app and delivery.md`. Every other
//! document's marker is written with that document — at the site, or in this
//! file's own header list below. **Where a marker is a clause of this crate's
//! own plan *and* of the document meant, the header list is not enough and the
//! document is written out at the site**, because a bare one there resolves to a
//! real and wrong clause.
//!
//! **Markers used below that belong to another document, each with the document
//! it belongs to.** `Reporting a problem` J2, J3, J4a, J8, N1, U7.
//!
//! **A marker this crate's own plan also defines is deliberately not listed
//! above, whichever document is meant, and is written out with its document
//! at every site instead** — the header list cannot tell the two apart, and a
//! bare one would resolve to a real and wrong clause.

use std::path::PathBuf;

use smoozy_reporting::diary::{opens_a_fresh_segment, segments_to_evict, Segment};
use smoozy_reporting::lines::{Declarations, Line};
use smoozy_state::identities::Identity;

use crate::platform::files::{self, Durability, Recheck};
use crate::platform::locations::{self, Location};

/// What one hand-over came to.
///
/// **L16: the service answers every hand-over with its outcome, so a builder
/// cannot half-use it** — there is no fire-and-forget entry point whose failure
/// is indistinguishable from success.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Wrote {
    /// The line is in the record and forced down to the device.
    Yes,
    /// **The entry is lost and that is the whole of the consequence.** Nothing
    /// retries, no failure record is raised and no surface says anything,
    /// because the user never meets the diary (`Reporting a problem` R7) and a
    /// failure record about the diary would put it in front of them. **The act
    /// the entry recorded proceeds untouched.**
    No { reported: String },
}

impl Wrote {
    /// **L16's second half: the caller learns the outcome and branches on
    /// nothing.** There is nothing for it to do differently, and a module that
    /// invented a fallback record of its own would be A2's fault.
    ///
    /// This exists so that the one legitimate use of the answer — putting it
    /// nowhere — is a thing the code says rather than a thing it omits.
    pub fn nothing_is_owed(&self) -> bool {
        true
    }
}

/// The diary's sole writer.
pub struct DiaryService {
    area: PathBuf,
}

impl DiaryService {
    /// Open the service on the diary area the adapter names.
    ///
    /// **L2a: the area sits in the per-user application-data location beside
    /// the local record and never inside it**, protected exactly as the record
    /// is and not encrypted — `Security and licence` §9's argument, applied
    /// there at `Reporting a problem` R6.
    pub fn open() -> Result<Self, String> {
        let area =
            locations::resolve(Location::DiaryArea).map_err(|not_named| not_named.reported)?;
        files::ensure_directory(&area).map_err(|failure| failure.reported)?;
        Ok(DiaryService { area })
    }

    /// The same, on a named area, so the rules below can be tested.
    pub fn open_at(area: PathBuf) -> Result<Self, String> {
        files::ensure_directory(&area).map_err(|failure| failure.reported)?;
        Ok(DiaryService { area })
    }

    /// Hand the service a line.
    ///
    /// **L15's first half, kept here in order.** `Reporting a problem` R5: the
    /// entry is written and **forced down at the moment its fact becomes
    /// true**; a fresh segment is opened where the line would take the current
    /// one past its size, so **every line is whole within one segment**; and
    /// the set is then brought back inside the cap by **deleting the oldest
    /// whole segment** and never by rewriting a retained byte.
    ///
    /// **The write boundary of §4 N1 is not enforced here and that is
    /// deliberate.** N1 is *"a rule about what is written, not a filter over
    /// what was written"*, and `Reporting a problem`'s own
    /// [`write_boundary`](smoozy_reporting::write_boundary) says in as many
    /// words that it is checked where the declaration is composed. **A filter
    /// at this point would be the second shape N1 forbids**, and it would also
    /// be a place a builder could believe protects them.
    pub fn write(&self, line: &Line) -> Wrote {
        let bytes = render(line);

        let mut segments = match self.segments() {
            Ok(segments) => segments,
            Err(reported) => return Wrote::No { reported },
        };

        let current = segments.last().map(|segment| segment.bytes).unwrap_or(0);
        if segments.is_empty() || opens_a_fresh_segment(current, bytes.len() as u64) {
            segments.push(Segment {
                name: self.next_segment_name(&segments),
                bytes: 0,
            });
        }

        let newest = segments.last().expect("a segment was just ensured");
        let path = self.area.join(&newest.name);
        if let Some(number) = newest.name.strip_prefix(SEGMENT_PREFIX).and_then(|n| n.parse().ok()) {
            crate::the_last_line::the_segment_is(&self.area, number);
        }

        // `Reporting a problem` R5: appended, forced down, and nothing already
        // written is modified.
        match append_and_force_down(&path, &bytes) {
            Ok(()) => {}
            Err(reported) => return Wrote::No { reported },
        }

        // `Reporting a problem` R3: the cap is 20 MB, fixed rather than read,
        // and **no free-space reading raises or lowers it** — which is also
        // F5's rule from the adapter's side, so there is no reading taken here
        // at all.
        self.evict_to_the_cap();

        Wrote::Yes
    }

    /// `Reporting a problem` J8: prepare the raw last-line route while healthy.
    pub fn prepare_the_last_line(&self, run: &Identity) -> Result<(), String> {
        let segments = self.segments()?;
        let number = segments.last().and_then(|s| s.name.strip_prefix(SEGMENT_PREFIX))
            .and_then(|n| n.parse().ok()).unwrap_or(0);
        crate::the_last_line::install(&self.area, run, number)?;
        self.evict_to_the_cap();
        crate::the_last_line::handlers_are_installed()
    }

    /// The segments, oldest first, which is the order they are kept in.
    fn segments(&self) -> Result<Vec<Segment<String>>, String> {
        let entries = match std::fs::read_dir(&self.area) {
            Ok(entries) => entries,
            Err(error) => return Err(error.to_string()),
        };
        let mut segments: Vec<Segment<String>> = entries
            .flatten()
            .filter_map(|entry| {
                let name = entry.file_name().to_string_lossy().into_owned();
                if !name.starts_with(SEGMENT_PREFIX) {
                    return None;
                }
                let bytes = entry.metadata().ok()?.len();
                Some(Segment { name, bytes })
            })
            .collect();
        // The names are fixed width and zero-padded, so their order is the
        // order they were written in. **Nothing here reads a file's presence,
        // its name or its size as a state fact** (`State, identity and
        // recovery` §1.3 rule 5): this is the record's own shape, and no state
        // fact is ever read out of the diary (`State, identity and recovery`
        // I1).
        segments.sort_by(|one, other| one.name.cmp(&other.name));
        Ok(segments)
    }

    fn next_segment_name(&self, existing: &[Segment<String>]) -> String {
        let next = existing
            .last()
            .and_then(|segment| segment.name.strip_prefix(SEGMENT_PREFIX))
            .and_then(|number| number.parse::<u64>().ok())
            .map(|number| number + 1)
            .unwrap_or(0);
        format!("{SEGMENT_PREFIX}{next:012}")
    }

    /// `Reporting a problem` R5's eviction: **whole segments, oldest first,
    /// never a rewrite.**
    ///
    /// A segment that will not delete is left where it is and the pass carries
    /// on. The cap is then over by that segment's size until it can be
    /// removed, which is the honest outcome — the alternative is rewriting a
    /// retained byte, which `Reporting a problem` R5 forbids outright.
    fn evict_to_the_cap(&self) {
        let Ok(mut segments) = self.segments() else { return; };
        // J8's handlers cannot enumerate or evict while dying. Reserve their
        // bounded allowance through the ordinary cap rule while healthy.
        let reserve = crate::the_last_line::reserve_at(&self.area);
        if reserve != 0 { segments.push(Segment { name: String::new(), bytes: reserve }); }
        for name in segments_to_evict(&segments) {
            if !name.is_empty() { let _ = files::remove_file(&self.area.join(name)); }
        }
    }

    /// The area, for the report that is cut out of it at step 11.
    pub fn area(&self) -> &std::path::Path {
        &self.area
    }

    /// **The whole diary, read back oldest first, for the report cut out of
    /// it** — `Reporting a problem` S2's slices are cut over these lines, and
    /// nothing else reads the diary (`State, identity and recovery` I1).
    ///
    /// **It reads [`render`]'s own form back and no other**, which is why it
    /// sits beside it: the two are one format, and a change to one that the
    /// other did not follow would fail the round trip below rather than a
    /// report months later.
    ///
    /// **One thing the lines cannot say for themselves comes back beside
    /// them: whether `Reporting a problem` R5's cap has ever evicted a
    /// segment.** Segments are numbered from zero ([`Self::next_segment_name`])
    /// and evicted oldest first, so an oldest survivor that is not number zero
    /// means older lines went. The report states that in its own words
    /// (`Reporting a problem` S5).
    ///
    /// **A line that will not read back is counted and left out**, never
    /// guessed at: the run identity it would be sliced on is unreadable, so no
    /// slice could hold it honestly.
    pub fn read_back(&self) -> Result<TheDiaryReadBack, String> {
        let segments = self.segments()?;
        let the_cap_has_dropped_older_lines = segments
            .first()
            .and_then(|segment| segment.name.strip_prefix(SEGMENT_PREFIX))
            .and_then(|number| number.parse::<u64>().ok())
            .is_some_and(|oldest| oldest > 0);
        let mut read = TheDiaryReadBack {
            lines: Vec::new(),
            the_cap_has_dropped_older_lines,
            lines_that_would_not_read: 0,
        };
        for segment in &segments {
            let bytes = std::fs::read(self.area.join(&segment.name)).map_err(|error| error.to_string())?;
            let text = String::from_utf8_lossy(&bytes);
            for written in text.lines() {
                if written.is_empty() {
                    continue;
                }
                match a_line_read_back(written) {
                    Some(line) => read.lines.push(line),
                    None => read.lines_that_would_not_read += 1,
                }
            }
        }
        Ok(read)
    }
}

/// **The diary as [`DiaryService::read_back`] found it.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TheDiaryReadBack {
    /// Every line, oldest first, which is the record's own order.
    pub lines: Vec<Line>,
    /// **Whether `Reporting a problem` R5's cap has ever evicted a segment** —
    /// the one fact about the record its lines cannot give.
    pub the_cap_has_dropped_older_lines: bool,
    /// Lines in the record that did not read back as [`render`] writes them.
    pub lines_that_would_not_read: usize,
}

/// **Every kind an identity on a line can be**, matched by the name its
/// written form carries. The written form is the identity's own debug form,
/// `Run(…)`, so the kind is read off that name and never guessed.
const EVERY_IDENTITY_KIND: [smoozy_state::identities::IdentityKind; 11] = {
    use smoozy_state::identities::IdentityKind::*;
    [
        LocalServer,
        LocalKey,
        SecretStoreSubject,
        Job,
        Group,
        Installation,
        Run,
        Condition,
        Transfer,
        Operation,
        Event,
    ]
};

/// One identity, read back from its written form on a line.
fn an_identity_read_back(written: &str) -> Option<Identity> {
    let (kind, rest) = written.split_once('(')?;
    let hex = rest.strip_suffix(')')?;
    let kind = EVERY_IDENTITY_KIND
        .into_iter()
        .find(|one| format!("{one:?}") == kind)?;
    Identity::read_back(kind, hex)
}

/// One line read back from [`render`]'s form — the run, the kind, the moment,
/// the clock, what it concerns, and the text, tab-separated.
///
/// **The text is the last field and takes the rest of the line**, which is
/// safe because [`render`] replaces every tab and newline inside a value.
pub fn a_line_read_back(written: &str) -> Option<Line> {
    use smoozy_reporting::lines::LineKind;
    use smoozy_state::identities::IdentityKind;
    use smoozy_state::observed::{Clock, MachineClock};

    let mut fields = written.splitn(6, '\t');
    let run = an_identity_read_back(fields.next()?)?;
    if run.kind() != IdentityKind::Run {
        return None;
    }
    let kind_written = fields.next()?;
    let kind = LineKind::ALL
        .into_iter()
        .find(|one| format!("{one:?}") == kind_written)?;
    let moment: u64 = fields.next()?.parse().ok()?;
    let clock = match fields.next()? {
        "ThisComputer" => Clock::ThisComputer,
        other => {
            // `AMachine(MachineClock("…"))`, the debug form of the machine's
            // own name — a local server identifier, which holds no quote or
            // backslash; the two escapes the form can carry are undone anyway.
            let inner = other
                .strip_prefix("AMachine(MachineClock(\"")?
                .strip_suffix("\"))")?;
            Clock::AMachine(MachineClock(inner.replace("\\\"", "\"").replace("\\\\", "\\")))
        }
    };
    let concerning_written = fields.next()?;
    let mut concerning = Vec::new();
    if !concerning_written.is_empty() {
        for one in concerning_written.split(',') {
            concerning.push(an_identity_read_back(one)?);
        }
    }
    let text = fields.next()?.to_string();
    Some(Line {
        run,
        kind,
        at: std::time::UNIX_EPOCH + std::time::Duration::from_secs(moment),
        clock,
        text,
        concerning,
    })
}

/// The name every segment carries. Fixed width and zero-padded so the set's
/// order is its names' order.
const SEGMENT_PREFIX: &str = "diary-";

/// One line as it is written: the run, the kind, the moment, the clock it is
/// on, what it concerns, and the declared text — one line of plain text, which
/// is `Reporting a problem` R1's own shape.
///
/// **Tab-separated, and a tab or a newline inside a value is not possible**:
/// J3 makes every text a declared fixed form filled with declared values, and
/// there is no route by which free prose composed at run time reaches here.
/// Where one arrived anyway it is replaced rather than escaped, because a
/// report cut out of this record is read by a person and an escape sequence is
/// a mechanism's own vocabulary.
fn render(line: &Line) -> Vec<u8> {
    let plain = |value: &str| value.replace(['\t', '\n', '\r'], " ");
    let moment = line
        .at
        .duration_since(std::time::UNIX_EPOCH)
        .map(|since| since.as_secs())
        .unwrap_or(0);
    let concerning = line
        .concerning
        .iter()
        .map(|identity| plain(&format!("{identity:?}")))
        .collect::<Vec<_>>()
        .join(",");
    format!(
        "{run:?}\t{kind:?}\t{moment}\t{clock:?}\t{concerning}\t{text}\n",
        run = line.run,
        kind = line.kind,
        clock = line.clock,
        text = plain(&line.text),
    )
    .into_bytes()
}

/// `Reporting a problem` R5's *written and forced down at the moment its fact
/// becomes true*, and **J8's shape for every line rather than only the last
/// one**.
///
/// The append is opened, written, forced to the device and closed, and then
/// **F1's re-check by path is what says whether it landed** — the adapter's own
/// rule, which is why this does not read the write call's return value as an
/// outcome.
fn append_and_force_down(path: &std::path::Path, bytes: &[u8]) -> Result<(), String> {
    use std::io::Write;

    let before = match files::outside_program_wrote(path) {
        Recheck::Present { bytes } => bytes,
        Recheck::Absent => 0,
        Recheck::CouldNotLook { reported } => return Err(reported),
        Recheck::ShortOrDifferent { bytes, .. } => bytes,
    };

    let mut first_error: Option<String> = None;
    match std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
    {
        Ok(mut file) => {
            if let Err(error) = file.write_all(bytes) {
                first_error.get_or_insert(error.to_string());
            }
            if let Err(error) = files::flush_to_the_device(&file) {
                first_error.get_or_insert(error.to_string());
            }
            drop(file);
        }
        Err(error) => {
            first_error.get_or_insert(error.to_string());
        }
    }

    if let Some(reported) = first_error {
        return Err(reported);
    }

    // F1: the re-check is what is reported.
    match files::outside_program_wrote(path) {
        Recheck::Present { bytes: after } if after == before + bytes.len() as u64 => Ok(()),
        Recheck::Present { bytes: after } => Err(format!(
            "the segment holds {} where it should hold {}",
            crate::typography::a_figure_with_its_unit(after, "bytes"),
            before + bytes.len() as u64
        )),
        Recheck::Absent => Err("the segment was not there after the line was written".into()),
        Recheck::ShortOrDifferent { bytes: after, .. } => {
            Err(format!(
                "the segment holds {}",
                crate::typography::a_figure_with_its_unit(after, "bytes")
            ))
        }
        Recheck::CouldNotLook { reported } => Err(reported),
    }
}

/// **What a feature module writes a line through — A7's hosting rule, made a
/// thing rather than a sentence.**
///
/// `Reporting a problem` J2 and J4a declare per module and never at run time,
/// and **this document's own A7 makes a module with no declaration write
/// nothing** — which is a
/// hosting fault and not a default: a module writing nowhere satisfies its own
/// spec and leaves a hole the Creator cannot tell from a thing that did not
/// happen. So the declarations are consulted here, at the one place a line
/// reaches the service, rather than trusted to each caller.
///
/// **The run identity travels with it**, because U7 puts it on every line and a
/// caller composing a line has no other route to one.
pub struct ModulesDiary<'a> {
    pub service: &'a DiaryService,
    pub declarations: &'a Declarations<&'static str>,
    /// The run identity this launch issued (`State, identity and recovery` Y5).
    pub run: Identity,
    /// Which module is writing — the name it declared under.
    pub module: &'static str,
}

impl ModulesDiary<'_> {
    /// Hand the service one line.
    ///
    /// **A kind this module did not declare is not written**, and the answer
    /// says so rather than reading as a write that happened.
    pub fn write(&self, line: &Line) -> Wrote {
        if !self.declarations.may_write(&self.module, line.kind) {
            return Wrote::No {
                reported: format!("{} declares no {:?} line", self.module, line.kind),
            };
        }
        self.service.write(line)
    }
}

/// The one thing this module states about its own cost, so nobody has to
/// rediscover it: **`Durability::ToTheDevice` is what every line is written
/// with**, and `Reporting a problem` R5's own words are that the cost is a
/// real device write per entry, bounded by the rate rules the line kinds carry
/// — no kind is written per file, per frame, per derivative, per byte or per
/// journal event.
pub const EVERY_LINE_COSTS_A_DEVICE_WRITE: Durability = Durability::ToTheDevice;

#[cfg(test)]
mod tests {
    use super::*;
    use smoozy_reporting::diary::{CAP_BYTES, SEGMENT_BYTES};
    use smoozy_reporting::lines::LineKind;
    use smoozy_state::identities::{Identity, IdentityKind};
    use smoozy_state::observed::Clock;

    fn area(name: &str) -> PathBuf {
        let mut path = std::env::temp_dir();
        path.push(format!("smoozy-diary-test-{name}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&path);
        path
    }

    fn a_line(text: &str) -> Line {
        Line {
            run: Identity::issue(IdentityKind::Run).expect("this machine has randomness"),
            kind: LineKind::Act,
            at: std::time::SystemTime::now(),
            clock: Clock::ThisComputer,
            text: text.to_string(),
            concerning: Vec::new(),
        }
    }

    /// L14 and `Reporting a problem` R5: the service writes the line, and it
    /// is in the record afterwards.
    #[test]
    fn a_line_handed_over_is_written() {
        let area = area("write");
        let service = DiaryService::open_at(area.clone()).unwrap();
        assert_eq!(
            service.write(&a_line("the user picked a folder")),
            Wrote::Yes
        );

        let segments: Vec<_> = std::fs::read_dir(&area).unwrap().flatten().collect();
        assert_eq!(segments.len(), 1);
        let held = std::fs::read_to_string(segments[0].path()).unwrap();
        assert!(held.contains("the user picked a folder"));
        assert!(held.ends_with('\n'));

        let _ = std::fs::remove_dir_all(&area);
    }

    /// `Reporting a problem` R5's second bound: **every line is whole within
    /// one segment**, so a fresh segment is opened rather than a line split at
    /// an edge.
    #[test]
    fn a_line_is_never_split_across_two_segments() {
        let area = area("segments");
        let service = DiaryService::open_at(area.clone()).unwrap();

        // Enough lines to pass the segment size at least once.
        let long = "x".repeat(4096);
        let mut written = 0u64;
        while written < SEGMENT_BYTES + 8192 {
            assert_eq!(service.write(&a_line(&long)), Wrote::Yes);
            written += long.len() as u64;
        }

        let mut names: Vec<String> = std::fs::read_dir(&area)
            .unwrap()
            .flatten()
            .map(|entry| entry.file_name().to_string_lossy().into_owned())
            .collect();
        names.sort();
        assert!(names.len() >= 2, "no fresh segment was opened: {names:?}");

        // Every segment holds whole lines: each ends with a newline and none
        // begins mid-line.
        for name in &names {
            let held = std::fs::read_to_string(area.join(name)).unwrap();
            assert!(held.ends_with('\n'), "{name} ends mid-line");
            for line in held.lines() {
                assert!(line.contains('\t'), "{name} begins mid-line");
            }
        }

        let _ = std::fs::remove_dir_all(&area);
    }

    /// `Reporting a problem` R5: eviction deletes the **oldest whole segment**
    /// and never rewrites a retained byte, and `Reporting a problem` R3's cap
    /// is what it is measured against.
    #[test]
    fn eviction_drops_whole_segments_oldest_first() {
        let area = area("evict");
        let service = DiaryService::open_at(area.clone()).unwrap();

        // Twenty-two segments' worth of names, made directly so the test does
        // not have to write twenty megabytes.
        for index in 0..22u64 {
            let name = format!("{SEGMENT_PREFIX}{index:012}");
            std::fs::create_dir_all(&area).unwrap();
            std::fs::write(area.join(&name), vec![b'x'; SEGMENT_BYTES as usize]).unwrap();
        }
        let before = service.segments().unwrap();
        assert!(before.iter().map(|s| s.bytes).sum::<u64>() > CAP_BYTES);

        service.evict_to_the_cap();

        let after = service.segments().unwrap();
        assert!(
            after.iter().map(|s| s.bytes).sum::<u64>() <= CAP_BYTES,
            "the set is still over the cap"
        );
        // Oldest first: the survivors are the tail of the original list.
        assert_eq!(after.last().unwrap().name, before.last().unwrap().name);
        assert!(after.first().unwrap().name > before.first().unwrap().name);
        // No retained byte was rewritten.
        for segment in &after {
            assert_eq!(segment.bytes, SEGMENT_BYTES);
        }

        let _ = std::fs::remove_dir_all(&area);
    }

    /// **L16: the caller learns the outcome and branches on nothing**, and a
    /// refusal loses the entry and nothing else — the act proceeds.
    #[test]
    fn a_refused_write_loses_the_entry_and_nothing_else() {
        // An area that is not there and cannot be made: the service reports it
        // and the caller has nothing owed.
        let service = DiaryService {
            area: PathBuf::from("/no-such-place/diary"),
        };
        let outcome = service.write(&a_line("an act"));
        assert!(matches!(outcome, Wrote::No { .. }));
        assert!(outcome.nothing_is_owed());
    }

    /// **The read-back is the write's own form read back** — every field of a
    /// line comes back as it was handed over, on both clocks, with what it
    /// concerns, and with the tab a value carried written as the space
    /// [`render`] made it.
    #[test]
    fn every_line_written_reads_back_as_it_was_handed_over() {
        use smoozy_state::observed::MachineClock;
        let area = area("read-back");
        let service = DiaryService::open_at(area.clone()).unwrap();
        let run = Identity::issue(IdentityKind::Run).unwrap();
        let job = Identity::issue(IdentityKind::Job).unwrap();
        let server = Identity::issue(IdentityKind::LocalServer).unwrap();
        let at = std::time::UNIX_EPOCH + std::time::Duration::from_secs(1_756_132_320);
        let lines = vec![
            Line {
                run: run.clone(),
                kind: LineKind::Session,
                at,
                clock: Clock::ThisComputer,
                text: "session\t0.1.8\tmacos".to_string(),
                concerning: vec![run.clone()],
            },
            Line {
                run: run.clone(),
                kind: LineKind::Told,
                at,
                clock: Clock::AMachine(MachineClock(server.as_text())),
                text: "the render ended".to_string(),
                concerning: vec![job.clone(), server.clone()],
            },
            Line {
                run: run.clone(),
                kind: LineKind::Failure,
                at,
                clock: Clock::ThisComputer,
                text: "The render did not finish — Smoozy does not know why.".to_string(),
                concerning: Vec::new(),
            },
        ];
        for line in &lines {
            assert_eq!(service.write(line), Wrote::Yes);
        }
        let read = service.read_back().expect("the diary reads back");
        assert_eq!(read.lines_that_would_not_read, 0);
        assert!(!read.the_cap_has_dropped_older_lines);
        let mut expected = lines.clone();
        expected[0].text = "session 0.1.8 macos".to_string();
        assert_eq!(read.lines, expected);
        let _ = std::fs::remove_dir_all(&area);
    }

    /// **`Reporting a problem` R5's eviction is read off the segment names**:
    /// an oldest survivor numbered above zero means older lines went, and a
    /// line that is not the write's own form is counted rather than guessed.
    #[test]
    fn an_evicted_segment_is_seen_and_a_line_that_will_not_read_is_counted() {
        let area = area("read-back-evicted");
        std::fs::create_dir_all(&area).unwrap();
        let one = render(&a_line("kept"));
        let mut held = one.clone();
        held.extend_from_slice(b"not a diary line at all\n");
        std::fs::write(area.join(format!("{SEGMENT_PREFIX}{:012}", 3)), &held).unwrap();
        let service = DiaryService::open_at(area.clone()).unwrap();
        let read = service.read_back().unwrap();
        assert!(read.the_cap_has_dropped_older_lines);
        assert_eq!(read.lines.len(), 1);
        assert_eq!(read.lines[0].text, "kept");
        assert_eq!(read.lines_that_would_not_read, 1);
        let _ = std::fs::remove_dir_all(&area);
    }

    /// J3: a line is one line. A value carrying a newline cannot split the
    /// record into two.
    #[test]
    fn nothing_in_a_line_can_start_a_second_one() {
        let rendered = render(&a_line("two\nlines\tand a tab"));
        assert_eq!(rendered.iter().filter(|byte| **byte == b'\n').count(), 1);
        assert_eq!(*rendered.last().unwrap(), b'\n');
    }
}

/// `Reporting a problem` J8's only exception to the ordinary writer's path:
/// prepared bytes, raw append, then the system's device flush. The new segment
/// is created only when a handler runs, so a hard kill leaves no invented line.
pub(crate) fn the_last_line_is_appended(prepared: &crate::the_last_line::Prepared, name: &[u8], bytes: &[u8]) {
    crate::the_last_line::raw_append(prepared, name, bytes);
}
