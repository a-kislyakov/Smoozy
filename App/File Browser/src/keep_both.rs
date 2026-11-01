//! Keep both — `File Browser` §4.3, X9, Rows 192 and 186.
//!
//! **Markers used below that belong to another document, each with the document
//! it belongs to.** `Server protocol and transfers` §12 item 12.

use crate::already_exists::names_are_equal;
use crate::listing::{CompleteListing, EntryName};

/// **N4's bound, in bytes, per component.** 255 passes, 256 fails.
///
/// **These are the observed *accepted* lengths and the observed refusals are the
/// next byte up** — `One box with Blender on it` §7: *"Longest single name: 255
/// characters. 256 was refused at open"* (that tester posed the length in ASCII
/// and reported characters; Fact 29 records the unit as **bytes**), and the
/// three-seller batch's *"255 accepted, 256 refused"*, with
/// `statvfs@openssh.com` reporting `f_namemax` **255** on all three.
///
/// **§13 item 3 rides with them and is permanently unresolved**: the numbers are
/// right **for Linux**, from the kernel's own header, and they are not
/// universal — POSIX.1-2024's `<limits.h>` says *"The values in the following
/// list may be constants within an implementation or may vary from one pathname
/// to another."* **What breaks:** a rental with tighter limits refuses names
/// the pre-flight passed; the transfer then fails at open with the true
/// wording, never the machine's string. **What holds:** N4's per-file
/// containment of the failure.
pub const THE_LONGEST_ACCEPTED_COMPONENT: usize = 255;

/// **N4's bound, in bytes, over the whole path.** 4095 passes, 4096 fails.
pub const THE_LONGEST_ACCEPTED_PATH: usize = 4095;

/// Whether a landing path is inside N4's two bounds. **Both inclusive.**
pub fn is_inside_the_length_bounds(landing_path: &[u8]) -> bool {
    if landing_path.len() > THE_LONGEST_ACCEPTED_PATH {
        return false;
    }
    crate::already_exists::components(landing_path)
        .iter()
        .all(|component| component.len() <= THE_LONGEST_ACCEPTED_COMPONENT)
}

/// A name split into the part a counted suffix is appended to and the part that
/// follows it.
///
/// **X9 names *the stem* as the thing that is trimmed, and that is what settles
/// where the suffix goes**: a stem is only distinct from the rest of a name if
/// the rest is kept, so `scene.blend` becomes `scene (2).blend` rather than
/// `scene.blend (2)`. **It is a decision and it has a consequence worth
/// stating:** a `.blend` renamed the other way would stop being a `.blend` to
/// L7's version reading and to N7a's toolbar rule, which is a cost no clause
/// grants.
///
/// **The split is by bytes and by one rule**: the last `.` in a **file's** name,
/// where it is not the first byte. A folder takes the suffix at the end of its
/// whole name — a folder has no extension to keep.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Split {
    pub stem: Vec<u8>,
    /// The dot and everything after it, or empty.
    pub rest: Vec<u8>,
}

pub fn split(name: &EntryName, is_a_folder: bool) -> Split {
    let bytes = name.as_bytes();
    if is_a_folder {
        return Split {
            stem: bytes.to_vec(),
            rest: Vec::new(),
        };
    }
    match bytes.iter().rposition(|byte| *byte == b'.') {
        Some(0) | None => Split {
            stem: bytes.to_vec(),
            rest: Vec::new(),
        },
        Some(at) => Split {
            stem: bytes[..at].to_vec(),
            rest: bytes[at..].to_vec(),
        },
    }
}

/// **X9: Keep both renames the whole arriving top-level thing, and the
/// generated name is checked like any other and backstopped like any other.**
///
/// The name is the original with a counted suffix — **the first of `name (2)`,
/// `name (3)`, … that collides with nothing in the destination listing and
/// nothing else arriving.** The check is X5's byte comparison; the length bound
/// is applied **in bytes, per component and over the whole landing path**,
/// **trimming the stem at a byte boundary** where the suffix would push a name
/// over; and **the publication is create-intended, so a generated name that
/// still lands on something — a folding filesystem, a race — is refused by X7
/// rather than written over anything.** Nothing inside a renamed folder is
/// merged, per the spec.
///
/// **The trim is in bytes because the name is bytes, and N2 is why.** N2 reads
/// source names as stored on disk and *"nothing rewrites, normalises or
/// re-encodes a name in flight"*, so a stem need not be valid in any encoding —
/// `Server protocol and transfers` §12 item 12 records *"one tool aborted an
/// entire transfer on a non-UTF-8 name"*, so such names are real here. **A
/// *character* boundary is a statement about an encoding**, and running the stem
/// through a decoder to find one either raises on the undecodable case, a
/// per-file failure this document does not owe, or substitutes replacement
/// characters, which is the re-encoding N2 forbids. **So the trim is at a byte
/// boundary in every case.** Where the stem's bytes do decode as UTF-8, the
/// boundary chosen is one that does not split a multi-byte sequence — **a
/// courtesy that costs nothing**.
///
/// `None` where no name inside the bounds can be composed at all — which is the
/// per-file failure N4 already describes, worded as the truth and never sent.
pub fn compose(
    original: &EntryName,
    is_a_folder: bool,
    destination: &[u8],
    destination_listing: &CompleteListing,
    also_arriving: &[EntryName],
) -> Option<EntryName> {
    let Split { stem, rest } = split(original, is_a_folder);

    for counter in 2..=u32::MAX {
        let suffix = format!(" ({counter})").into_bytes();
        let room = room_for_the_stem(destination, &suffix, &rest)?;
        let trimmed = trim_at_a_byte_boundary(&stem, room);

        let mut composed = trimmed;
        composed.extend_from_slice(&suffix);
        composed.extend_from_slice(&rest);
        let candidate = EntryName::from_the_machine(composed);

        // X5's byte comparison, against the destination listing and against
        // everything else arriving.
        let taken = destination_listing.holds(&candidate).is_some()
            || also_arriving
                .iter()
                .any(|other| names_are_equal(other, &candidate));
        if !taken {
            return Some(candidate);
        }
    }
    None
}

/// How many bytes of stem fit, given the suffix, what follows it, and both of
/// N4's bounds. `None` where the suffix alone will not fit.
fn room_for_the_stem(destination: &[u8], suffix: &[u8], rest: &[u8]) -> Option<usize> {
    // The whole landing path is the destination, a separator, and the name.
    let before_the_name = crate::already_exists::components(destination)
        .iter()
        .map(|component| component.len() + 1)
        .sum::<usize>()
        + 1;
    let fixed = suffix.len() + rest.len();
    let by_component = THE_LONGEST_ACCEPTED_COMPONENT.checked_sub(fixed)?;
    let by_path = THE_LONGEST_ACCEPTED_PATH
        .checked_sub(before_the_name)?
        .checked_sub(fixed)?;
    Some(by_component.min(by_path))
}

/// Trim to at most `room` bytes, **at a byte boundary in every case**, and
/// where the bytes decode as UTF-8 at one that does not split a multi-byte
/// sequence.
pub fn trim_at_a_byte_boundary(stem: &[u8], room: usize) -> Vec<u8> {
    if stem.len() <= room {
        return stem.to_vec();
    }
    let mut at = room;
    if std::str::from_utf8(stem).is_ok() {
        // Walk back off a continuation byte. The courtesy, and nothing more:
        // where the stem does not decode this loop is not entered at all.
        while at > 0 && (stem[at] & 0b1100_0000) == 0b1000_0000 {
            at -= 1;
        }
    }
    stem[..at].to_vec()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::listing::{ApparentSize, Kind, ListedEntry};
    use std::time::SystemTime;

    fn name(bytes: &[u8]) -> EntryName {
        EntryName::from_the_machine(bytes.to_vec())
    }

    fn listing(entries: &[&[u8]]) -> CompleteListing {
        CompleteListing::of(
            entries
                .iter()
                .map(|bytes| ListedEntry {
                    name: name(bytes),
                    size: ApparentSize::of(1),
                    last_changed: None,
                    kind: Kind::File,
                })
                .collect(),
            SystemTime::UNIX_EPOCH,
            false,
        )
    }

    #[test]
    fn the_suffix_goes_on_the_stem_and_the_extension_is_kept() {
        let composed = compose(
            &name(b"scene.blend"),
            false,
            b"/dest",
            &listing(&[b"scene.blend"]),
            &[],
        )
        .unwrap();
        assert_eq!(composed.as_bytes(), b"scene (2).blend");
    }

    #[test]
    fn the_counter_walks_past_every_name_already_there_and_every_name_arriving() {
        let composed = compose(
            &name(b"scene.blend"),
            false,
            b"/dest",
            &listing(&[b"scene.blend", b"scene (2).blend"]),
            &[name(b"scene (3).blend")],
        )
        .unwrap();
        assert_eq!(composed.as_bytes(), b"scene (4).blend");
    }

    #[test]
    fn a_folder_takes_the_suffix_at_the_end_of_its_whole_name() {
        let composed = compose(
            &name(b"assets"),
            true,
            b"/dest",
            &listing(&[b"assets"]),
            &[],
        )
        .unwrap();
        assert_eq!(composed.as_bytes(), b"assets (2)");
        // And a folder whose name holds a dot keeps the whole name as the stem.
        let composed = compose(&name(b"my.assets"), true, b"/dest", &listing(&[]), &[]).unwrap();
        assert_eq!(composed.as_bytes(), b"my.assets (2)");
    }

    #[test]
    fn a_dotfile_has_no_extension_to_keep() {
        let split = split(&name(b".hidden"), false);
        assert_eq!(split.stem, b".hidden".to_vec());
        assert!(split.rest.is_empty());
        let split = split_of(b"noextension");
        assert_eq!(split.stem, b"noextension".to_vec());
        assert!(split.rest.is_empty());
    }

    fn split_of(bytes: &[u8]) -> Split {
        split(&name(bytes), false)
    }

    #[test]
    fn the_stem_is_trimmed_where_the_suffix_would_push_the_name_over_the_byte_bound() {
        let long = vec![b'a'; 255];
        let mut with_extension = long.clone();
        with_extension.extend_from_slice(b".png");
        // The original is already too long for a component, which is N4's own
        // per-file failure — Keep both still composes a name inside the bound.
        let composed =
            compose(&name(&with_extension), false, b"/dest", &listing(&[]), &[]).unwrap();
        assert!(composed.as_bytes().len() <= THE_LONGEST_ACCEPTED_COMPONENT);
        assert!(composed.as_bytes().ends_with(b" (2).png"));
        assert!(is_inside_the_length_bounds(&{
            let mut path = b"/dest/".to_vec();
            path.extend_from_slice(composed.as_bytes());
            path
        }));
    }

    #[test]
    fn the_trim_never_splits_a_multi_byte_sequence_in_a_name_that_decodes() {
        // Four é, eight bytes. Room for five bytes must not cut the third one
        // in half.
        let stem = "\u{e9}\u{e9}\u{e9}\u{e9}".as_bytes();
        assert_eq!(stem.len(), 8);
        let trimmed = trim_at_a_byte_boundary(stem, 5);
        assert_eq!(trimmed.len(), 4);
        assert!(std::str::from_utf8(&trimmed).is_ok());
    }

    #[test]
    fn a_stem_that_does_not_decode_is_trimmed_at_a_byte_boundary_and_never_re_encoded() {
        let stem = b"bad\xff\xfe\xff\xfename";
        let trimmed = trim_at_a_byte_boundary(stem, 5);
        // Exactly five bytes, and every one of them the original's own.
        assert_eq!(trimmed, b"bad\xff\xfe".to_vec());
        assert!(!trimmed.contains(&0xef)); // no replacement character anywhere
    }

    /// A destination of `components` folders, each 200 bytes — so every
    /// component is inside its own bound and the whole path is what bites.
    fn a_deep_destination(components: usize) -> Vec<u8> {
        let mut path = Vec::new();
        for _ in 0..components {
            path.push(b'/');
            path.extend(std::iter::repeat_n(b'd', 200));
        }
        path
    }

    #[test]
    fn a_long_destination_bites_before_the_component_bound_does() {
        // The whole landing path is bounded too, and it is the tighter of the
        // two here: twenty components of 200 bytes leaves 66 for the stem.
        let deep = a_deep_destination(20);
        let composed = compose(&name(&[b'a'; 200]), false, &deep, &listing(&[]), &[]).unwrap();
        assert!(composed.as_bytes().len() < 200);
        let mut path = deep;
        path.push(b'/');
        path.extend_from_slice(composed.as_bytes());
        assert!(is_inside_the_length_bounds(&path));
    }

    #[test]
    fn where_no_name_fits_at_all_the_answer_is_a_stated_failure_and_not_a_bad_name() {
        // A destination so long that not even the suffix and the extension fit.
        let deep = a_deep_destination(21);
        assert_eq!(
            compose(&name(b"a.png"), false, &deep, &listing(&[]), &[]),
            None
        );
    }

    #[test]
    fn the_bounds_are_inclusive_at_the_accepted_length_and_fail_at_the_next_byte() {
        let mut at_the_bound = b"/".to_vec();
        at_the_bound.extend(std::iter::repeat_n(b'a', 255));
        assert!(is_inside_the_length_bounds(&at_the_bound));

        let mut a_byte_over = b"/".to_vec();
        a_byte_over.extend(std::iter::repeat_n(b'a', 256));
        assert!(!is_inside_the_length_bounds(&a_byte_over));

        // And the whole-path bound, with every component inside its own.
        let mut whole = a_deep_destination(20);
        whole.push(b'/');
        whole.extend(std::iter::repeat_n(b'a', 4095 - whole.len()));
        assert_eq!(whole.len(), 4095);
        assert!(is_inside_the_length_bounds(&whole));
        whole.push(b'a');
        assert_eq!(whole.len(), 4096);
        assert!(!is_inside_the_length_bounds(&whole));
    }
}
