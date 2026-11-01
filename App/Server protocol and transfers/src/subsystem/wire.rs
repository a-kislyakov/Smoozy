//! The file-transfer subsystem's wire form — revision 3, encoded and decoded.
//!
//! **Nothing in this file opens a socket.** It is the whole of the protocol's
//! shape, so every field rule below is testable at the desk against bytes
//! written by hand.
//!
//! **Revision 3 is the revision in play, on two recorded grounds.**
//! *Documented*: OpenSSH's own `PROTOCOL` file — the file that opens *"This
//! documents OpenSSH's deviations and extensions to the published SSH
//! protocol."* — says *"Note that OpenSSH's sftp and sftp-server implement
//! revision 3 of the SSH filexfer protocol described in:"*. *Observed*:
//! protocol version **3** was the version announced by every rental posed —
//! four boxes across three sellers. `Server protocol and transfers` [S13]
//! carries both grounds and states the limit: that a server Smoozy has not yet
//! met announces revision 3 is an expectation rather than an observation, and
//! [`Announcement`] reads it from the server's own opening reply rather than
//! assuming it.
//!
//! **Why this mechanism carries the values that may not ride a command
//! string.** §1.1's first carriage route: *"This protocol represents file names
//! as strings"* — `draft-ietf-secsh-filexfer-02`, read 2026-08-22 —
//! length-delimited fields that no shell ever parses. **That is why every file
//! operation, attribute reading, free-space extension reading and transfer
//! byte needs no command at all, in either phase.** A string here is bytes with
//! a length in front of them, and [`Composed::string`] is the only way one is
//! written.
//!
//! [S13]: `Preparation/Architecture/Server protocol and transfers.md` §7.4
//!
//! **Markers used below that belong to another document, each with the
//! document it belongs to.** `Servers` H8.

/// The packet types revision 3 defines, and the two extension envelopes.
///
/// **Named rather than written as numbers at the call sites**, so a reader can
/// see which request a byte is without a table beside them.
pub mod kind {
    pub const INIT: u8 = 1;
    pub const VERSION: u8 = 2;
    pub const OPEN: u8 = 3;
    pub const CLOSE: u8 = 4;
    pub const READ: u8 = 5;
    pub const WRITE: u8 = 6;
    pub const LSTAT: u8 = 7;
    pub const FSTAT: u8 = 8;
    pub const SETSTAT: u8 = 9;
    pub const OPENDIR: u8 = 11;
    pub const READDIR: u8 = 12;
    pub const REMOVE: u8 = 13;
    pub const MKDIR: u8 = 14;
    /// **Observed rather than read**, on 2026-09-03, against OpenSSH's own
    /// `/usr/libexec/sftp-server` on the Creator's Mac: the server's own
    /// `-Q requests` lists `rmdir` between `mkdir` and `realpath`, and this
    /// request kind carrying `uint32 id, string path` removed an empty
    /// directory and answered status 0, answered `NoSuchFile` where the
    /// directory was not there, and answered a refusal where the directory
    /// still held a file. `App/Job files and outputs/tests/` holds that run.
    /// **Nothing here is taken from an unread specification.**
    pub const RMDIR: u8 = 15;
    pub const REALPATH: u8 = 16;
    pub const STAT: u8 = 17;
    pub const RENAME: u8 = 18;
    /// **The link's own target, read rather than resolved.**
    ///
    /// Documented, `draft-ietf-secsh-filexfer-02` — the protocol revision §1.2
    /// negotiates — read 2026-09-06 at
    /// `https://datatracker.ietf.org/doc/id/draft-ietf-secsh-filexfer-02.txt`.
    /// Section 3's packet-type list carries `#define SSH_FXP_READLINK 19`, and
    /// §6.10 says: *"The SSH_FXP_READLINK request may be used to read the target
    /// of a symbolic link"*, with a data part of `uint32 id, string path`, where
    /// *"`path' specifies the path name of the symlink to be read"*, and *"The
    /// server will respond with a SSH_FXP_NAME packet containing only one name
    /// and a dummy attributes value."*
    ///
    /// **`readlink` is in §1.1's observed request set** — that clause's own
    /// reading of `/usr/libexec/sftp-server -Q requests` lists it between
    /// `rename` and `symlink`, which is the order these three constants are in.
    ///
    /// **It is not [`REALPATH`] and the difference decides a package version.**
    /// The active-package pointer is a link and the placed artifact sits at the
    /// staging location the phase record names, so a resolving read answers with
    /// the staging directory; what B4's inspection compares against the recorded
    /// locations is the pointer's **own value**. `Rented-server runtime` G31a's
    /// machine-side half uses the same reading.
    pub const READLINK: u8 = 19;
    pub const SYMLINK: u8 = 20;

    pub const STATUS: u8 = 101;
    pub const HANDLE: u8 = 102;
    pub const DATA: u8 = 103;
    pub const NAME: u8 = 104;
    pub const ATTRS: u8 = 105;

    pub const EXTENDED: u8 = 200;
    pub const EXTENDED_REPLY: u8 = 201;
}

/// The open flags revision 3 defines.
pub mod open_flag {
    pub const READ: u32 = 0x0000_0001;
    pub const WRITE: u32 = 0x0000_0002;
    pub const CREATE: u32 = 0x0000_0008;
    pub const TRUNCATE: u32 = 0x0000_0010;
    /// **`SSH_FXF_EXCL`.** The create that refuses an existing target, which is
    /// what a staging path is opened with so that two attempts can never land
    /// on one object.
    pub const EXCLUSIVE: u32 = 0x0000_0020;
}

/// The extension names this document leans on, each named once.
///
/// **§1.2: a capability that is not announced is never attempted and never
/// guessed at.** These are the names compared against the server's own opening
/// reply; nothing here assumes one is present.
pub mod extension {
    /// **S16.** Publication over an existing name. The base rename refuses an
    /// existing target — observed, status **4 `Failure`** — so this is
    /// required for an overwrite and the base operation is enough only where
    /// the final name does not exist.
    pub const POSIX_RENAME: &str = "posix-rename@openssh.com";
    /// **F2 route 1.** The per-path filesystem-statistics reading.
    pub const STATVFS: &str = "statvfs@openssh.com";
    /// **S20.** Making the staged bytes durable before publication.
    pub const FSYNC: &str = "fsync@openssh.com";
    /// **§1.2 item 2.** The server's own declared operation limits.
    pub const LIMITS: &str = "limits@openssh.com";
    /// **`File Browser` M7's copy on the machine**, where the server announces
    /// it. OpenSSH's own `PROTOCOL` §4.10, read 2026-10-02: *"This request asks
    /// the server to copy data from one open file handle and write it to a
    /// different open file handle."* Absent from 8.9p1's announcement (this
    /// module's own head).
    pub const COPY_DATA: &str = "copy-data";
}

/// **The status codes revision 3 defines, at the path they have always been
/// reached at.**
///
/// The type sits in the other half of this module, because
/// [`crate::free_space::WhyThereIsNoReading`] names it and that shape is
/// composed on both sides of the connection. **What it is, and the draft's own
/// words for the two codes §1.3 turns on, are written at the type itself.**
pub use smoozy_framing::subsystem_answers::StatusCode;

/// **No status this surface returns is read as the disk being full**, and the
/// rule is stated here because a builder will look for one.
///
/// Observed: every write on a full filesystem was refused with **`No space left
/// on device`**, and a write with an explicit flush and `fsync` raised **errno
/// 28** — *"A full disk refuses in one way, everywhere."* **Revision 3's status
/// set has no member for it**, so a full disk arrives as
/// [`StatusCode::Failure`] with the machine's own words beside it — and **§1.3
/// forbids mining a tool's own output for a cause**, so nothing here reads the
/// message to promote a `Failure` into a disk-full.
///
/// **F7's rule is that a request whose failure carries that condition is route
/// (a) and a request whose failure carries anything else is not.** On the
/// machine, [`crate::free_space`]'s per-path reading is what says the place is
/// out of room, and it is a separate reading.
pub const NO_STATUS_HERE_IS_READ_AS_THE_DISK_BEING_FULL: bool = true;

/// One file's attributes, revision 3's structure.
///
/// **What it does not carry is what [S13] leans on hardest.** Revision 3's
/// attribute structure defines flags, size, owner and group, permissions,
/// access and modification times, and extensions — **no inode, no file
/// identifier, no change time**, and the modification time is a 32-bit count
/// of **seconds**, so no sub-second granularity either. The change time is what
/// a hand-restored modification time cannot defeat, and it is one of the two
/// this surface does not carry, which is why S13's four-value reading is a
/// command-channel reading.
///
/// [S13]: `Preparation/Architecture/Server protocol and transfers.md` §7.4
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Attributes {
    pub size: Option<u64>,
    pub permissions: Option<u32>,
    pub access_time: Option<u32>,
    pub modification_time: Option<u32>,
}

mod attribute_flag {
    pub const SIZE: u32 = 0x0000_0001;
    pub const UID_GID: u32 = 0x0000_0002;
    pub const PERMISSIONS: u32 = 0x0000_0004;
    pub const ACCESS_MODIFICATION_TIME: u32 = 0x0000_0008;
    pub const EXTENDED: u32 = 0x8000_0000;
}

impl Attributes {
    /// The attributes that ask for nothing — what an open sends where it is
    /// not setting a mode.
    pub fn none() -> Attributes {
        Attributes::default()
    }

    /// **B10's attribute-setting operation**, composed as permissions alone.
    pub fn permissions(mode: u32) -> Attributes {
        Attributes {
            permissions: Some(mode),
            ..Attributes::default()
        }
    }

    /// **S15c's attribute-setting operation**, composed as the two times
    /// alone and both the same: the source's modification time, in the whole
    /// seconds this revision carries.
    pub fn times(seconds: u32) -> Attributes {
        Attributes {
            access_time: Some(seconds),
            modification_time: Some(seconds),
            ..Attributes::default()
        }
    }
}

/// Writes revision 3's fields. **Every value that crosses this surface crosses
/// through one of these**, which is §1.1's first carriage route kept by there
/// being no other way to put a byte on the wire.
#[derive(Debug, Default)]
pub struct Composed {
    bytes: Vec<u8>,
}

impl Composed {
    pub fn new() -> Composed {
        Composed { bytes: Vec::new() }
    }

    pub fn byte(&mut self, value: u8) -> &mut Composed {
        self.bytes.push(value);
        self
    }

    pub fn u32(&mut self, value: u32) -> &mut Composed {
        self.bytes.extend_from_slice(&value.to_be_bytes());
        self
    }

    pub fn u64(&mut self, value: u64) -> &mut Composed {
        self.bytes.extend_from_slice(&value.to_be_bytes());
        self
    }

    /// **A string is a length and then bytes**, and the bytes are never
    /// examined for syntax. A path with a leading dash, a newline, a space or a
    /// non-UTF-8 name is a value here and nothing else — which is why Fact 29's
    /// hazards do not arise on this route.
    pub fn string(&mut self, value: &[u8]) -> &mut Composed {
        let length = u32::try_from(value.len()).unwrap_or(u32::MAX);
        self.u32(length);
        self.bytes.extend_from_slice(&value[..length as usize]);
        self
    }

    pub fn attributes(&mut self, attributes: &Attributes) -> &mut Composed {
        let mut flags = 0u32;
        if attributes.size.is_some() {
            flags |= attribute_flag::SIZE;
        }
        if attributes.permissions.is_some() {
            flags |= attribute_flag::PERMISSIONS;
        }
        if attributes.access_time.is_some() || attributes.modification_time.is_some() {
            flags |= attribute_flag::ACCESS_MODIFICATION_TIME;
        }
        self.u32(flags);
        if let Some(size) = attributes.size {
            self.u64(size);
        }
        if let Some(permissions) = attributes.permissions {
            self.u32(permissions);
        }
        if flags & attribute_flag::ACCESS_MODIFICATION_TIME != 0 {
            self.u32(attributes.access_time.unwrap_or(0));
            self.u32(attributes.modification_time.unwrap_or(0));
        }
        self
    }

    /// The finished packet: a four-byte length, then the type byte, then the
    /// payload. **The length covers the type byte**, which is the one place a
    /// builder composes this wrongly.
    pub fn framed(&self, packet_kind: u8) -> Vec<u8> {
        let length = u32::try_from(self.bytes.len() + 1).unwrap_or(u32::MAX);
        let mut whole = Vec::with_capacity(self.bytes.len() + 5);
        whole.extend_from_slice(&length.to_be_bytes());
        whole.push(packet_kind);
        whole.extend_from_slice(&self.bytes);
        whole
    }
}

/// Reads revision 3's fields, and **runs out rather than guessing**.
///
/// A field that is not wholly there returns nothing. Nothing here fills a short
/// read with a zero, because a zero read from a truncated packet is a value
/// nobody sent.
#[derive(Debug)]
pub struct Fields<'a> {
    bytes: &'a [u8],
    at: usize,
}

impl<'a> Fields<'a> {
    pub fn over(bytes: &'a [u8]) -> Fields<'a> {
        Fields { bytes, at: 0 }
    }

    pub fn byte(&mut self) -> Option<u8> {
        let value = *self.bytes.get(self.at)?;
        self.at += 1;
        Some(value)
    }

    pub fn u32(&mut self) -> Option<u32> {
        let slice = self.bytes.get(self.at..self.at + 4)?;
        self.at += 4;
        Some(u32::from_be_bytes(slice.try_into().ok()?))
    }

    pub fn u64(&mut self) -> Option<u64> {
        let slice = self.bytes.get(self.at..self.at + 8)?;
        self.at += 8;
        Some(u64::from_be_bytes(slice.try_into().ok()?))
    }

    /// **Bytes, never text.** A directory listing that decodes entry names as
    /// UTF-8 raises on a name the write, the read-back and the machine's own
    /// `ls` all coped with — observed, `UnicodeDecodeError` on
    /// `b'bad\xff\xfe.txt'` — so the name is one Smoozy can move and cannot
    /// list. §12 item 12's rule is that **the machine-side enumeration carries
    /// names as bytes and decodes only for display**, and that is kept here by
    /// there being no text-returning reader at all.
    pub fn string(&mut self) -> Option<&'a [u8]> {
        let length = self.u32()? as usize;
        let slice = self.bytes.get(self.at..self.at + length)?;
        self.at += length;
        Some(slice)
    }

    pub fn attributes(&mut self) -> Option<Attributes> {
        let flags = self.u32()?;
        let mut attributes = Attributes::default();
        if flags & attribute_flag::SIZE != 0 {
            attributes.size = Some(self.u64()?);
        }
        if flags & attribute_flag::UID_GID != 0 {
            let _uid = self.u32()?;
            let _gid = self.u32()?;
        }
        if flags & attribute_flag::PERMISSIONS != 0 {
            attributes.permissions = Some(self.u32()?);
        }
        if flags & attribute_flag::ACCESS_MODIFICATION_TIME != 0 {
            attributes.access_time = Some(self.u32()?);
            attributes.modification_time = Some(self.u32()?);
        }
        if flags & attribute_flag::EXTENDED != 0 {
            let count = self.u32()?;
            for _ in 0..count {
                self.string()?;
                self.string()?;
            }
        }
        Some(attributes)
    }

    /// What is left unread — the payload of a `DATA` reply, after its length.
    pub fn rest(&self) -> &'a [u8] {
        &self.bytes[self.at.min(self.bytes.len())..]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_frame_length_covers_the_type_byte() {
        // The one place a builder composes this wrongly: the four-byte length
        // is of the type byte plus the payload, not of the payload alone.
        let mut composed = Composed::new();
        composed.u32(7);
        let framed = composed.framed(kind::INIT);
        assert_eq!(&framed[..4], &5u32.to_be_bytes());
        assert_eq!(framed[4], kind::INIT);
        assert_eq!(framed.len(), 9);
    }

    #[test]
    fn a_string_is_a_length_and_then_bytes_whatever_the_bytes_are() {
        // §1.1's first carriage route. A name that breaks three tools three
        // different ways is a value here and nothing else.
        let hostile = b"a name with a space\n-and a leading dash\xff\xfe";
        let mut composed = Composed::new();
        composed.string(hostile);
        let framed = composed.framed(kind::STAT);
        let mut read = Fields::over(&framed[5..]);
        assert_eq!(read.string(), Some(&hostile[..]));
    }

    #[test]
    fn a_short_packet_runs_out_rather_than_reading_a_zero() {
        // A zero read from a truncated packet is a value nobody sent.
        let mut read = Fields::over(&[0, 0, 1]);
        assert_eq!(read.u32(), None);
        let mut read = Fields::over(&[0, 0, 0, 9, 1, 2]);
        assert_eq!(read.string(), None);
    }

    #[test]
    fn the_two_status_codes_ss13_turns_on_are_two_values() {
        // §1.3's whole consequence: at readdir and at stat alike, the
        // subsystem tells refusal from absence in one request.
        assert_eq!(StatusCode::from_number(2), StatusCode::NoSuchFile);
        assert_eq!(StatusCode::from_number(3), StatusCode::PermissionDenied);
        assert_ne!(StatusCode::NoSuchFile, StatusCode::PermissionDenied);
    }

    #[test]
    fn an_undefined_status_number_is_carried_as_itself() {
        // Nothing guesses what a number the draft does not define means.
        assert_eq!(StatusCode::from_number(31), StatusCode::Other(31));
        assert!(!StatusCode::Other(31).is_ok());
    }

    #[test]
    fn revision_three_has_no_status_for_a_full_disk() {
        // F7 and §1.3 together. Every code the draft defines is one of these,
        // and none of them means the disk is full — a `Failure` is a `Failure`.
        assert!(NO_STATUS_HERE_IS_READ_AS_THE_DISK_BEING_FULL);
        assert_eq!(StatusCode::from_number(4), StatusCode::Failure);
        assert_eq!(StatusCode::from_number(28), StatusCode::Other(28));
    }

    #[test]
    fn attributes_round_trip_and_absent_fields_stay_absent() {
        // An attribute the server did not send is an absence rather than a
        // zero — the same rule H8 already keeps for a per-card memory figure.
        let attributes = Attributes {
            size: Some(4_294_967_296),
            permissions: Some(0o600),
            access_time: None,
            modification_time: None,
        };
        let mut composed = Composed::new();
        composed.attributes(&attributes);
        let framed = composed.framed(kind::ATTRS);
        let mut read = Fields::over(&framed[5..]);
        let back = read.attributes().expect("what was written reads back");
        assert_eq!(back.size, Some(4_294_967_296));
        assert_eq!(back.permissions, Some(0o600));
        assert_eq!(back.modification_time, None);
    }

    #[test]
    fn revision_threes_attributes_carry_no_change_time_and_no_identifier() {
        // Stated as a test because S13 leans on it: the four-value unchanged
        // -source reading cannot be taken here, and a builder who thinks it can
        // has a compile error rather than a wrong answer.
        let attributes = Attributes::default();
        let rendered = format!("{attributes:?}");
        assert!(!rendered.contains("change_time"));
        assert!(!rendered.contains("inode"));
    }
}
