//! The diary — `Reporting a problem` §1.
//!
//! R1: **one line-oriented plain-text record in a diary area of its own**, in
//! the per-user application-data location the platform file adapter names,
//! beside the local record and never inside it.
//!
//! R2: **it is not state.** Nothing in the application branches on anything it
//! holds, nothing is presented from it, no promise to the user rests on its
//! contents, and its total loss costs the contents of a future report and
//! nothing else.
//!
//! R7: **the user never meets it.** No screen shows it, no setting governs it,
//! no control clears it, and nothing counts it against anything. Its lifetime
//! is the installation's.
//!
//! R8: **Smoozy never sends the diary, and never sends anything cut from it.**

/// R3: **the cap is 20 MB, fixed rather than read, and no free-space reading
/// raises or lowers it.** The same rule `State, identity and recovery` E6a, in
/// that document's §4, fixes for the journal's own cap and
/// `Rented-server runtime` R2 applies on
/// the machine in as many words — *"a constant of the server package and no
/// free-space reading raises or lowers it"*. The figure is the spec's own.
pub const CAP_BYTES: u64 = 20 * 1024 * 1024;

/// The segment size, **which R5 leaves to Build under two bounds**: the sum
/// obeys the cap, and every line is whole within one segment.
///
/// **Build's pick, and the reason.** One megabyte, which puts twenty segments
/// under the cap, so one eviction drops at most a twentieth of the record
/// rather than a large fraction of it. A segment is closed and a new one
/// opened at the first line that would take it past this size, so a line is
/// never split and the second bound holds by construction — which means a
/// segment may finish slightly over this figure, and the cap below is measured
/// against the actual sizes rather than against this one.
pub const SEGMENT_BYTES: u64 = 1024 * 1024;

/// One segment of the append-only set — R5's second half.
///
/// **Eviction is by whole segments, never by rewriting.** Row 451's own
/// constraint: a record trimmed by rewriting itself has a window in which a
/// crash loses the whole of it. So lines are appended to the newest segment,
/// nothing already written is ever modified, and **a crash at any moment loses
/// at most the line being appended**.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Segment<Name> {
    pub name: Name,
    pub bytes: u64,
}

/// Which segments to delete so the set is inside the cap again.
///
/// The set is handed in **oldest first**, which is the order it is kept in.
/// **The oldest whole segment is deleted** — and the next, and so on — until
/// the total is at or under the cap. The newest segment is never deleted,
/// because it is the one being appended to and deleting it would lose the line
/// that had just been written.
pub fn segments_to_evict<Name: Clone>(oldest_first: &[Segment<Name>]) -> Vec<Name> {
    let mut total: u64 = oldest_first.iter().map(|segment| segment.bytes).sum();
    let mut evicted = Vec::new();

    for segment in oldest_first.iter().take(oldest_first.len().saturating_sub(1)) {
        if total <= CAP_BYTES {
            break;
        }
        total -= segment.bytes;
        evicted.push(segment.name.clone());
    }
    evicted
}

/// Whether a line of this size still fits in the segment being appended to, or
/// whether that segment is closed and a fresh one opened first.
///
/// This is the rule that keeps R5's second bound — **every line is whole
/// within one segment** — so a report cut out of the record never meets half a
/// line at a segment edge.
pub fn opens_a_fresh_segment(current_segment_bytes: u64, line_bytes: u64) -> bool {
    current_segment_bytes > 0 && current_segment_bytes + line_bytes > SEGMENT_BYTES
}

/// What R5's first half requires of every write, and what it costs.
///
/// **Durable at the moment.** `State, identity and recovery` D6's second
/// sentence applied: the diary exists to survive the crash that follows the
/// line, so a line buffered against a later flush is a line the one crash that
/// matters loses. Each entry is written and **forced down** when its fact
/// becomes true — `State, identity and recovery` §1.3 rule 6's full-flush form
/// on the user's own computer.
///
/// **The cost is a real device write per entry**, and what bounds it is the
/// rate rules the kinds themselves carry ([`crate::lines`]): **no kind is
/// written per file, per frame, per derivative, per byte or per journal
/// event.**
pub const EVERY_LINE_IS_FORCED_DOWN_WHEN_IT_IS_WRITTEN: () = ();

/// R6: **the diary is protected exactly as the local record is, and by the
/// same decision.** `Security and licence` §9's argument is cited rather than
/// remade: written in the per-user application-data location, created for the
/// owning user only, protected by the operating system's own file permissions
/// and nothing further, and **not encrypted** — an encrypting key would live
/// in the store the secrets already live in, and on a Linux computer with
/// nowhere to remember it would cost the user something to type, which no spec
/// grants. It holds no secret value by construction, which is what makes that
/// protection sufficient.
pub const PROTECTED_AS_THE_LOCAL_RECORD_IS_AND_NOT_ENCRYPTED: () = ();

#[cfg(test)]
mod tests {
    use super::*;

    fn segment(name: &'static str, bytes: u64) -> Segment<&'static str> {
        Segment { name, bytes }
    }

    #[test]
    fn a_set_inside_the_cap_evicts_nothing() {
        let set = vec![segment("one", SEGMENT_BYTES), segment("two", SEGMENT_BYTES)];
        assert!(segments_to_evict(&set).is_empty());
    }

    #[test]
    fn the_oldest_whole_segment_goes_first() {
        let mut set: Vec<_> = (0..21)
            .map(|n| {
                Segment {
                    name: n,
                    bytes: SEGMENT_BYTES,
                }
            })
            .collect();
        set.sort_by_key(|segment| segment.name);
        assert_eq!(segments_to_evict(&set), vec![0]);
    }

    #[test]
    fn as_many_go_as_the_cap_needs_and_no_more() {
        let set: Vec<_> = (0..25)
            .map(|n| Segment {
                name: n,
                bytes: SEGMENT_BYTES,
            })
            .collect();
        assert_eq!(segments_to_evict(&set), vec![0, 1, 2, 3, 4]);
    }

    #[test]
    fn the_segment_being_appended_to_is_never_evicted() {
        // One segment, larger than the whole cap on its own: there is nothing
        // to drop that would not lose the line just written.
        let set = vec![segment("only", CAP_BYTES * 2)];
        assert!(segments_to_evict(&set).is_empty());
    }

    #[test]
    fn a_line_that_would_overflow_a_segment_opens_a_fresh_one() {
        assert!(opens_a_fresh_segment(SEGMENT_BYTES, 1));
        assert!(!opens_a_fresh_segment(SEGMENT_BYTES - 10, 10));
    }

    #[test]
    fn a_line_larger_than_a_segment_still_lands_whole_in_an_empty_one() {
        // Opening a fresh segment for it would leave the same problem, so an
        // empty segment takes whatever it is handed and finishes over size.
        // The cap is measured against actual sizes, which is why that is safe.
        assert!(!opens_a_fresh_segment(0, SEGMENT_BYTES * 2));
    }

    #[test]
    fn the_cap_is_twenty_megabytes_and_is_read_from_nowhere() {
        assert_eq!(CAP_BYTES, 20_971_520);
    }
}
