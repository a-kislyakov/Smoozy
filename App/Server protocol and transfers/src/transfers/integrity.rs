//! §7.6 — what is compared, and when.
//!
//! **S21: two different things protect two different stretches, and neither
//! covers the other.**
//!
//! - **The transport covers the pipe, for free and hard.** With a relay
//!   flipping exactly one bit and a control run first proving the relay
//!   faithful, the connection **died in both directions** — `message
//!   authentication code incorrect` downloading, `Broken pipe` uploading.
//!   **Nothing is added to detect damage in flight.** What is added is that the
//!   failure is read correctly: it presents as a dropped connection, so **a
//!   dropped channel is never read as a clean ending**.
//! - **An end-to-end digest covers everything the pipe does not**: the file at
//!   rest before it was read, the staged bytes at rest, and a resumed prefix.
//!   Four bytes overwritten in place with the modification time restored
//!   travelled through `scp` at exit 0, and a default `rsync` *"called it done:
//!   48 bytes sent, exit 0, file still corrupted"*, and *"with a change summary
//!   requested it printed nothing at all"*.
//!
//! **S22: the pass-through digest is computed in Smoozy's own byte loop, over
//! the bytes as they pass, and it is never asked of a tool.** Two reasons and
//! both are observed. The tool sets differ at the two ends and the odd end is
//! the user's — *"`sftp`'s 33 commands include **no checksum, verify, hash or
//! digest command of any kind**, and `scp` has nothing either"*. And computing
//! as the bytes pass costs no second read of the file.
//!
//! **S25: a digest computed as the bytes pass proves the bytes read match the
//! bytes written, and no more.** It does not prove the file on the sending disk
//! was whole before Smoozy read it.

use sha2::{Digest as _, Sha256};

/// **S23: one digest function per protocol version, the same at both ends, and
/// its implementation is chosen rather than inherited.**
///
/// The function is SHA-256, fixed by the protocol version. **The implementation
/// is picked for speed at each end**: measured at 2.24 s on the machine and
/// 2.22 s on the Mac for a 669,966,622-byte file, and **0.46 s on the same
/// machine with a different implementation of the same function** — *"on
/// processors with the instructions that make it fast, which both ends here had
/// and a rental may not"*.
#[derive(Clone, PartialEq, Eq)]
pub struct Digest([u8; 32]);

impl Digest {
    /// The digest of a whole slice, for the desk and for a short prefix.
    pub fn of(bytes: &[u8]) -> Digest {
        let mut running = Running::new();
        running.take(bytes);
        running.so_far()
    }

    /// The written form, which is a rendering and not a meaning.
    pub fn as_text(&self) -> String {
        let mut text = String::with_capacity(64);
        for byte in self.0 {
            text.push_str(&format!("{byte:02x}"));
        }
        text
    }

    /// The thirty-two bytes themselves.
    ///
    /// **What a record or a manifest carries is these bytes and never the
    /// written form**, which is a rendering: B8's per-artifact digest and
    /// `Rented-server runtime` G27's recorded one are both a fixed thirty-two,
    /// and a hexadecimal string would be a second encoding to agree about.
    pub fn bytes(&self) -> [u8; 32] {
        self.0
    }

    /// One read back from a record or a manifest, as the bytes it was written
    /// as.
    pub fn from_bytes(bytes: [u8; 32]) -> Digest {
        Digest(bytes)
    }

    /// Reads one back from the record. **Nothing parses it for anything but
    /// reading it back.**
    pub fn read_back(text: &str) -> Option<Digest> {
        if text.len() != 64 {
            return None;
        }
        let mut bytes = [0u8; 32];
        for (byte, pair) in bytes.iter_mut().zip(text.as_bytes().chunks(2)) {
            *byte = u8::from_str_radix(std::str::from_utf8(pair).ok()?, 16).ok()?;
        }
        Some(Digest(bytes))
    }
}

impl std::fmt::Debug for Digest {
    fn fmt(&self, out: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(out, "Digest({})", self.as_text())
    }
}

/// The running digest the byte loop keeps — **over the bytes as they pass**,
/// so no second read of the file is paid for.
///
/// **A checkpoint's digest is this value at that offset**, which is what makes
/// S12 step 3's comparison a comparison against Smoozy's own record rather than
/// against the far end.
#[derive(Clone)]
pub struct Running {
    state: Sha256,
    bytes_taken: u64,
}

impl Running {
    pub fn new() -> Running {
        Running {
            state: Sha256::new(),
            bytes_taken: 0,
        }
    }

    pub fn take(&mut self, bytes: &[u8]) {
        self.state.update(bytes);
        self.bytes_taken += bytes.len() as u64;
    }

    /// The digest of everything taken so far. **Taking it does not end the
    /// running value**, because a checkpoint is taken mid-transfer and the loop
    /// carries on from it.
    pub fn so_far(&self) -> Digest {
        let mut bytes = [0u8; 32];
        bytes.copy_from_slice(&self.state.clone().finalize());
        Digest(bytes)
    }

    pub fn bytes_taken(&self) -> u64 {
        self.bytes_taken
    }
}

impl Default for Running {
    fn default() -> Running {
        Running::new()
    }
}

/// **S24: where the digests disagree, the transfer ends *failed* and the staged
/// material is abandoned.**
///
/// It is never published, never repaired in place, never resumed, and never
/// retried inside the action. **Trying again is a new transfer with a new
/// identity.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhatTheComparisonSaid {
    TheyAgree,
    /// The transfer ends *failed*, and this value carries no route back into
    /// the transfer that produced it.
    TheyDisagree,
}

/// The comparison, as one function so that no site does it by hand.
pub fn compare(expected: &Digest, computed: &Digest) -> WhatTheComparisonSaid {
    if expected == computed {
        WhatTheComparisonSaid::TheyAgree
    } else {
        WhatTheComparisonSaid::TheyDisagree
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_running_digest_matches_a_whole_one_however_the_bytes_are_split() {
        // The whole of what makes a checkpoint's digest usable: the loop takes
        // the bytes in whatever chunks the negotiated limit gives it, and the
        // value is the file's.
        let whole = b"the quick brown fox jumps over the lazy dog, twice over";
        let mut running = Running::new();
        for chunk in whole.chunks(7) {
            running.take(chunk);
        }
        assert_eq!(running.so_far(), Digest::of(whole));
        assert_eq!(running.bytes_taken(), whole.len() as u64);
    }

    #[test]
    fn taking_the_value_does_not_end_the_running_one() {
        // A checkpoint is taken mid-transfer and the loop carries on from it.
        let mut running = Running::new();
        running.take(b"first");
        let at_the_checkpoint = running.so_far();
        running.take(b"second");
        assert_eq!(at_the_checkpoint, Digest::of(b"first"));
        assert_eq!(running.so_far(), Digest::of(b"firstsecond"));
    }

    #[test]
    fn four_bytes_changed_in_place_are_caught() {
        // Fact 72's own case: four bytes overwritten in place with the
        // modification time restored travelled through `scp` at exit 0.
        let mut original = vec![7u8; 4096];
        let sent = original.clone();
        original[2048..2052].copy_from_slice(b"oops");
        assert_eq!(
            compare(&Digest::of(&sent), &Digest::of(&original)),
            WhatTheComparisonSaid::TheyDisagree
        );
    }

    #[test]
    fn a_digest_reads_back_from_its_written_form() {
        let digest = Digest::of(b"anything");
        assert_eq!(Digest::read_back(&digest.as_text()), Some(digest));
    }

    #[test]
    fn a_written_form_that_is_not_one_reads_back_as_nothing() {
        assert_eq!(Digest::read_back(""), None);
        assert_eq!(Digest::read_back(&"z".repeat(64)), None);
    }

    #[test]
    fn the_function_is_pinned_so_a_changed_implementation_is_visible() {
        // S23 fixes the function by the protocol version and leaves the
        // implementation free — *"the implementation is picked for speed at
        // each end"*. So the value is pinned here: a swap that changed what the
        // function computes would break both ends' agreement silently, and this
        // is what makes it loud instead. **The number is what this
        // implementation produced when it was written down**, not a figure
        // taken from anywhere.
        assert_eq!(
            Digest::of(b"").as_text(),
            "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
        );
        assert_eq!(Digest::of(b"").as_text().len(), 64);
    }
}
