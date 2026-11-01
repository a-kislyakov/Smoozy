//! Driving the file-transfer subsystem — the requests, in Smoozy's own
//! process.
//!
//! **No `sftp` or `scp` program is run** (`Servers` K1a). What this drives is
//! the machine's own file-transfer server, over a subsystem session on the
//! connection [`crate::transport::ssh::Connection`] holds — which is why the
//! surface is available *"before the server package exists"* and after it is
//! destroyed: the subsystem is the machine's, and no Smoozy component is
//! needed at either phase.
//!
//! **Every value crosses as a protocol field.** §1.1's first carriage route,
//! kept by construction: there is no request on this channel that takes a
//! command string, so a path with a leading dash, a space, a newline or
//! non-UTF-8 bytes is a value and never syntax.
//!
//! **The write path is pipelined and that is a requirement rather than an
//! optimisation** (R6a, measured). The same loop against the same server ran at
//! **99–102 KB/s against a 384 KB/s cap** when each chunk waited a full round
//! trip for its reply, and at **384.0 KB/s exactly** with the writes pipelined
//! — *"A synchronous write path makes the cap not the binding constraint at
//! all"*. [`Subsystem::send_a_write`] and [`Subsystem::settle_the_writes`] are
//! that pair, and one consequence travels to S10a: a pipelined loop counts a
//! chunk when it hands it over, which is why a dropped connection leaves the
//! record **ahead** of the disk.
//!
//! **The read path is held to the same rule** (R6a, from 2026-09-30):
//! [`Subsystem::send_a_read`] and [`Subsystem::the_read_answered`] are its
//! pair, and the byte loop keeps several reads in flight with them.
//!
//! **Markers used below that belong to another document, each with the
//! document it belongs to.** `Servers` K1a. `Job files and outputs` K4.

use std::collections::HashMap;

use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};

use super::wire::{extension, kind, open_flag, Composed, Fields, StatusCode};
use super::{
    Announcement, Attributes, Entry, FilesystemStatistics, Handle, Limits, SubsystemFailure,
};

/// A file-transfer subsystem channel, driven.
///
/// **One of these is one channel.** §1.1: the reading traffic normally holds
/// one long-lived channel on the primary connection, opened at the first
/// reading and held for the connected period, and **each transfer opens its own
/// channel beside it** (T1).
pub struct Subsystem<S> {
    stream: S,
    next_id: u32,
    announcement: Announcement,
    /// Replies that arrived for a request other than the one being awaited —
    /// the ordinary case while writes are in flight, since *"There is no limit
    /// on the number of outstanding (non-acknowledged) requests that the client
    /// may send to the server"* and every request is identified by a
    /// client-chosen `id`.
    stashed: HashMap<u32, (u8, Vec<u8>)>,
    /// The identifiers of writes handed over and not yet settled.
    writes_in_flight: Vec<u32>,
}

impl<S: AsyncRead + AsyncWrite + Unpin + Send> Subsystem<S> {
    /// Opens the subsystem on a stream and negotiates — §1.2 items 1 and 2.
    ///
    /// **Three things are negotiated and none is inferred from a version
    /// number.** The version and the extension set come from the server's own
    /// opening reply; the operation limits come from the extension that reports
    /// them **where the server announces it**, and otherwise from the draft's
    /// own recommended pair.
    pub async fn negotiate(mut stream: S) -> Result<Subsystem<S>, SubsystemFailure> {
        let mut composed = Composed::new();
        composed.u32(Announcement::REVISION_THIS_DOCUMENT_USES);
        stream
            .write_all(&composed.framed(kind::INIT))
            .await
            .map_err(|_| SubsystemFailure::TheChannelIsGone)?;
        stream
            .flush()
            .await
            .map_err(|_| SubsystemFailure::TheChannelIsGone)?;

        let (packet_kind, payload) = read_one_packet(&mut stream).await?;
        if packet_kind != kind::VERSION {
            return Err(SubsystemFailure::TheAnswerCouldNotBeRead);
        }
        let mut fields = Fields::over(&payload);
        let version = fields
            .u32()
            .ok_or(SubsystemFailure::TheAnswerCouldNotBeRead)?;
        // **A revision that is not the one this code's operations are written
        // against is refused here, and the refusal names what was offered.**
        //
        // Reading on would parse the rest of the reply, and every attribute
        // structure after it, to revision 3's shape — so a server on another
        // revision would hand back wrong sizes and wrong times rather than a
        // refusal, and §1.3's *no value is read as a cause* would be broken by
        // the values themselves. **§1.2's own answer is taken**: the surface is
        // reported unavailable against this connection, which is a statement
        // about the connection rather than a failure of the work, and the
        // module that wanted it decides.
        if version != Announcement::REVISION_THIS_DOCUMENT_USES {
            return Err(SubsystemFailure::TheServerOfferedAnotherRevision(version));
        }
        let mut extensions = Vec::new();
        // The extension pairs run to the end of the packet; a server that
        // announces none simply ends here.
        while let Some(name) = fields.string() {
            if fields.string().is_none() {
                break;
            }
            if let Ok(text) = std::str::from_utf8(name) {
                extensions.push(text.to_string());
            }
        }

        let mut subsystem = Subsystem {
            stream,
            next_id: 1,
            announcement: Announcement {
                version,
                extensions,
                limits: Limits::RECOMMENDED_WHERE_NONE_ARE_ANNOUNCED,
            },
            stashed: HashMap::new(),
            writes_in_flight: Vec::new(),
        };
        if subsystem.announcement.announces(extension::LIMITS) {
            if let Some(limits) = subsystem.ask_for_the_limits().await {
                subsystem.announcement.limits = limits;
            }
        }
        Ok(subsystem)
    }

    /// What the server announced. **Held against the connected period**, never
    /// against the server row.
    pub fn announcement(&self) -> &Announcement {
        &self.announcement
    }

    /// **The channel this subsystem runs on, to be looked at and never
    /// driven** — so a caller whose transfer failed can ask the channel which
    /// connection it sat on, and whether that connection has ended. Added
    /// 2026-10-19 for `Desktop app and delivery`'s extra-connection repair.
    pub fn the_channel(&self) -> &S {
        &self.stream
    }

    /// **§1.2 item 2's own extension**, asked for only where it is announced.
    async fn ask_for_the_limits(&mut self) -> Option<Limits> {
        let id = self.take_an_id();
        let mut composed = Composed::new();
        composed.u32(id).string(extension::LIMITS.as_bytes());
        self.send(kind::EXTENDED, &composed).await.ok()?;
        let (packet_kind, payload) = self.reply_to(id).await.ok()?;
        if packet_kind != kind::EXTENDED_REPLY {
            return None;
        }
        let mut fields = after_the_identifier(&payload)?;
        Some(Limits {
            largest_packet: fields.u64()?,
            largest_read: fields.u64()?,
            largest_write: fields.u64()?,
            open_handles: fields.u64()?,
        })
    }

    // ------------------------------------------------------------- the files

    /// Opens a path. **The path is a protocol field** and the flags are the
    /// caller's.
    pub async fn open(
        &mut self,
        path: &[u8],
        flags: u32,
        attributes: Attributes,
    ) -> Result<Handle, SubsystemFailure> {
        let id = self.take_an_id();
        let mut composed = Composed::new();
        composed
            .u32(id)
            .string(path)
            .u32(flags)
            .attributes(&attributes);
        self.send(kind::OPEN, &composed).await?;
        self.expect_a_handle(id).await
    }

    /// **The exclusive create a staging path is opened with.** `SSH_FXF_EXCL`
    /// refuses an existing target, so two attempts can never land on one
    /// object — observed on all three rentals read, with a plain create
    /// succeeding as the control.
    pub async fn create_exclusively(&mut self, path: &[u8]) -> Result<Handle, SubsystemFailure> {
        self.open(
            path,
            open_flag::WRITE | open_flag::CREATE | open_flag::EXCLUSIVE,
            Attributes::permissions(0o600),
        )
        .await
    }

    /// Opens an existing staging path for writing **without truncating it** —
    /// which is what a resume needs, and what S12 step 4's *"carry on from that
    /// checkpoint"* is performed with.
    pub async fn open_to_carry_on(&mut self, path: &[u8]) -> Result<Handle, SubsystemFailure> {
        self.open(path, open_flag::WRITE, Attributes::none()).await
    }

    pub async fn open_to_read(&mut self, path: &[u8]) -> Result<Handle, SubsystemFailure> {
        self.open(path, open_flag::READ, Attributes::none()).await
    }

    pub async fn close(&mut self, handle: &Handle) -> Result<(), SubsystemFailure> {
        let id = self.take_an_id();
        let mut composed = Composed::new();
        composed.u32(id).string(handle.bytes());
        self.send(kind::CLOSE, &composed).await?;
        self.expect_ok(id).await
    }

    /// One byte-offset read against an open handle. **The end of the file
    /// arrives as `SSH_FX_EOF` and is returned as an empty answer** rather than
    /// as a failure, because a short file is not a fault.
    pub async fn read_at(
        &mut self,
        handle: &Handle,
        offset: u64,
        length: usize,
    ) -> Result<Vec<u8>, SubsystemFailure> {
        let id = self.send_a_read(handle, offset, length).await?;
        self.the_read_answered(id).await
    }

    /// **Hands one read over without waiting for its answer** — R6a's read
    /// path, added 2026-09-30 after step 10c's eleventh pass: *"a download
    /// keeps several reads in flight, and so does the read-back that verifies
    /// a staged prefix (S12)"*. The caller collects each answer with
    /// [`Subsystem::the_read_answered`], by the identifier this returns.
    ///
    /// **The case this was first written for** was one read at a time, the
    /// whole of [`Subsystem::read_at`], which is still the pair of these two.
    /// **The case it now also meets** is a far end a whole round trip away,
    /// where a read that waits for its answer before the next is sent moves
    /// one 261,120-byte chunk per round trip whatever the line carries.
    pub async fn send_a_read(
        &mut self,
        handle: &Handle,
        offset: u64,
        length: usize,
    ) -> Result<u32, SubsystemFailure> {
        let id = self.take_an_id();
        let mut composed = Composed::new();
        composed
            .u32(id)
            .string(handle.bytes())
            .u64(offset)
            .u32(u32::try_from(length).unwrap_or(u32::MAX));
        self.send(kind::READ, &composed).await?;
        Ok(id)
    }

    /// **The answer to a read handed over earlier**, matched by its
    /// identifier and never by arrival order (C2). The end of the file is an
    /// empty answer, as [`Subsystem::read_at`]'s is.
    pub async fn the_read_answered(&mut self, id: u32) -> Result<Vec<u8>, SubsystemFailure> {
        let (packet_kind, payload) = self.reply_to(id).await?;
        match packet_kind {
            kind::DATA => {
                let mut fields = after_the_identifier(&payload)
                    .ok_or(SubsystemFailure::TheAnswerCouldNotBeRead)?;
                let data = fields
                    .string()
                    .ok_or(SubsystemFailure::TheAnswerCouldNotBeRead)?;
                Ok(data.to_vec())
            }
            kind::STATUS => match status_in(&payload) {
                Some(StatusCode::EndOfFile) => Ok(Vec::new()),
                Some(code) => Err(SubsystemFailure::TheServerRefused(code)),
                None => Err(SubsystemFailure::TheAnswerCouldNotBeRead),
            },
            _ => Err(SubsystemFailure::TheAnswerCouldNotBeRead),
        }
    }

    /// **Hands one write over without waiting for its reply** — R6a's whole
    /// requirement. The caller settles them with [`Subsystem::settle_the_writes`].
    pub async fn send_a_write(
        &mut self,
        handle: &Handle,
        offset: u64,
        data: &[u8],
    ) -> Result<(), SubsystemFailure> {
        let id = self.take_an_id();
        let mut composed = Composed::new();
        composed
            .u32(id)
            .string(handle.bytes())
            .u64(offset)
            .string(data);
        self.send(kind::WRITE, &composed).await?;
        self.writes_in_flight.push(id);
        Ok(())
    }

    /// Collects every outstanding write's answer. **A refusal is the first one
    /// met and the rest are still collected**, so the channel is not left with
    /// replies nobody read.
    pub async fn settle_the_writes(&mut self) -> Result<(), SubsystemFailure> {
        let in_flight = std::mem::take(&mut self.writes_in_flight);
        let mut first_refusal = None;
        for id in in_flight {
            match self.expect_ok(id).await {
                Ok(()) => {}
                Err(failure) => {
                    if first_refusal.is_none() {
                        first_refusal = Some(failure);
                    }
                }
            }
        }
        match first_refusal {
            Some(failure) => Err(failure),
            None => Ok(()),
        }
    }

    /// How many writes are handed over and unsettled — what a loop's own
    /// accounting reads to bound its depth.
    pub fn writes_in_flight(&self) -> usize {
        self.writes_in_flight.len()
    }

    pub async fn stat(&mut self, path: &[u8]) -> Result<Attributes, SubsystemFailure> {
        self.attributes_of(kind::STAT, path).await
    }

    /// **The reading a publication's recovery takes**, and the one that tells
    /// a link from what it points at.
    pub async fn lstat(&mut self, path: &[u8]) -> Result<Attributes, SubsystemFailure> {
        self.attributes_of(kind::LSTAT, path).await
    }

    async fn attributes_of(
        &mut self,
        packet_kind: u8,
        path: &[u8],
    ) -> Result<Attributes, SubsystemFailure> {
        let id = self.take_an_id();
        let mut composed = Composed::new();
        composed.u32(id).string(path);
        self.send(packet_kind, &composed).await?;
        self.the_reading_answered(id).await
    }

    /// **B10's attribute-setting operation, composed as permissions alone.**
    ///
    /// It is the step that makes a verified staged artifact executable, and
    /// **the ordering is not this method's to keep** — B10 puts the at-rest
    /// comparison before it, and
    /// [`crate::release::verifying::make_the_artifact_executable`] is where
    /// that is held.
    ///
    /// **Permissions and nothing else.** The operation can carry a size, an
    /// owner and two times as well; sending any of them would be Smoozy
    /// deciding something about a file it was only asked to make runnable.
    pub async fn set_permissions(
        &mut self,
        path: &[u8],
        mode: u32,
    ) -> Result<(), SubsystemFailure> {
        let id = self.take_an_id();
        let mut composed = Composed::new();
        composed
            .u32(id)
            .string(path)
            .attributes(&Attributes::permissions(mode));
        self.send(kind::SETSTAT, &composed).await?;
        self.expect_ok(id).await
    }

    /// **S15c: the set-attributes call carrying the two times and nothing
    /// else**, handed over without waiting for its answer, so it rides beside
    /// the close rather than costing a round trip of its own. Never in the same
    /// call as a permission change, which stays [`Subsystem::set_permissions`].
    /// The caller collects the answer with [`Subsystem::the_times_answered`].
    pub async fn send_the_times(&mut self, path: &[u8], seconds: u32) -> Result<u32, SubsystemFailure> {
        let id = self.take_an_id();
        let mut composed = Composed::new();
        composed
            .u32(id)
            .string(path)
            .attributes(&Attributes::times(seconds));
        self.send(kind::SETSTAT, &composed).await?;
        Ok(id)
    }

    /// The answer to [`Subsystem::send_the_times`].
    pub async fn the_times_answered(&mut self, id: u32) -> Result<(), SubsystemFailure> {
        self.expect_ok(id).await
    }

    /// **S15c's reading of a download's source**, on the handle the transfer
    /// already holds, handed over without waiting so it costs no round trip of
    /// its own. Collected with [`Subsystem::the_reading_answered`].
    pub async fn send_a_reading_of(&mut self, handle: &Handle) -> Result<u32, SubsystemFailure> {
        let id = self.take_an_id();
        let mut composed = Composed::new();
        composed.u32(id).string(handle.bytes());
        self.send(kind::FSTAT, &composed).await?;
        Ok(id)
    }

    /// The answer to [`Subsystem::send_a_reading_of`].
    pub async fn the_reading_answered(&mut self, id: u32) -> Result<Attributes, SubsystemFailure> {
        let (answered, payload) = self.reply_to(id).await?;
        match answered {
            kind::ATTRS => after_the_identifier(&payload)
                .and_then(|mut fields| fields.attributes())
                .ok_or(SubsystemFailure::TheAnswerCouldNotBeRead),
            kind::STATUS => Err(refusal_in(&payload)),
            _ => Err(SubsystemFailure::TheAnswerCouldNotBeRead),
        }
    }

    pub async fn remove(&mut self, path: &[u8]) -> Result<(), SubsystemFailure> {
        let id = self.take_an_id();
        let mut composed = Composed::new();
        composed.u32(id).string(path);
        self.send(kind::REMOVE, &composed).await?;
        self.expect_ok(id).await
    }

    /// **Hands one removal over without waiting for its answer** — a file's,
    /// or an empty folder's where `a_folder` — so a delete of many entries
    /// does not wait a round trip per entry (`File Browser` M2, step 10c's
    /// Files' owed acts piece 4). The caller collects each answer with
    /// [`Subsystem::the_removal_answered`], by the identifier this returns,
    /// and **the order is the caller's to keep**: a folder is handed over only
    /// once what was inside it has answered.
    pub async fn send_a_removal(&mut self, path: &[u8], a_folder: bool) -> Result<u32, SubsystemFailure> {
        let id = self.take_an_id();
        let mut composed = Composed::new();
        composed.u32(id).string(path);
        self.send(if a_folder { kind::RMDIR } else { kind::REMOVE }, &composed)
            .await?;
        Ok(id)
    }

    /// **The answer to a removal handed over earlier**, matched by its
    /// identifier and never by arrival order.
    pub async fn the_removal_answered(&mut self, id: u32) -> Result<(), SubsystemFailure> {
        self.expect_ok(id).await
    }

    pub async fn make_directory(&mut self, path: &[u8]) -> Result<(), SubsystemFailure> {
        let id = self.take_an_id();
        let mut composed = Composed::new();
        composed
            .u32(id)
            .string(path)
            .attributes(&Attributes::permissions(0o700));
        self.send(kind::MKDIR, &composed).await?;
        self.expect_ok(id).await
    }

    /// **One empty directory removed.**
    ///
    /// `Job files and outputs` K4's abandoned-material sweep is what needs it:
    /// its two removals are a staged directory and a job output folder whose
    /// record was never published, and both are directories. **The request kind
    /// is observed rather than read** — see [`crate::subsystem::wire::kind`]'s
    /// own note on it.
    pub async fn remove_directory(&mut self, path: &[u8]) -> Result<(), SubsystemFailure> {
        let id = self.take_an_id();
        let mut composed = Composed::new();
        composed.u32(id).string(path);
        self.send(kind::RMDIR, &composed).await?;
        self.expect_ok(id).await
    }

    /// **The base rename**, which *"refuses to replace an existing target"* —
    /// observed, status **4 `Failure`**. That refusal is exactly the backstop a
    /// **create-intended** publication wants (S15a), so this is the operation
    /// that intent uses in every case, announced extension or not.
    pub async fn rename(&mut self, from: &[u8], to: &[u8]) -> Result<(), SubsystemFailure> {
        let id = self.take_an_id();
        let mut composed = Composed::new();
        composed.u32(id).string(from).string(to);
        self.send(kind::RENAME, &composed).await?;
        self.expect_ok(id).await
    }

    /// **S16's operation.** `posix-rename@openssh.com`, and the base operation
    /// will not do for a replacement.
    ///
    /// Documented by OpenSSH itself, `PROTOCOL` §4.3: *"On receiving this
    /// request the server will perform the POSIX operation rename(oldpath,
    /// newpath) and will respond with a SSH_FXP_STATUS message."* Observed: it
    /// replaced an existing target wholly, the published file held only the new
    /// content, **its inode had changed**, and the staging name was gone.
    pub async fn posix_rename(&mut self, from: &[u8], to: &[u8]) -> Result<(), SubsystemFailure> {
        if !self.announcement.announces(extension::POSIX_RENAME) {
            return Err(SubsystemFailure::ThatCapabilityWasNotAnnounced);
        }
        let id = self.take_an_id();
        let mut composed = Composed::new();
        composed
            .u32(id)
            .string(extension::POSIX_RENAME.as_bytes())
            .string(from)
            .string(to);
        self.send(kind::EXTENDED, &composed).await?;
        self.expect_ok(id).await
    }

    /// **`File Browser` M7: the bytes move on the machine and never across the
    /// wire** — OpenSSH's `copy-data`, refused before the wire where the
    /// server did not announce it (§1.2's rule, as [`Self::posix_rename`]
    /// keeps it). Its shape is OpenSSH's own `PROTOCOL` §4.10, read
    /// 2026-10-02: *"string read-from-handle, uint64 read-from-offset, uint64
    /// read-data-length, string write-to-handle, uint64 write-to-offset"*, and
    /// *"If read-data-length is 0, then the server will read data from the
    /// read-from-handle until EOF is reached."* **The whole file, from its
    /// start, to the start of the other**, answered by one status once it is
    /// written. Step 10c's Files' owed acts piece 6.
    pub async fn copy_data(&mut self, from: &Handle, to: &Handle) -> Result<(), SubsystemFailure> {
        if !self.announcement.announces(extension::COPY_DATA) {
            return Err(SubsystemFailure::ThatCapabilityWasNotAnnounced);
        }
        let id = self.take_an_id();
        let mut composed = Composed::new();
        composed
            .u32(id)
            .string(extension::COPY_DATA.as_bytes())
            .string(from.bytes())
            .u64(0)
            .u64(0)
            .string(to.bytes())
            .u64(0);
        self.send(kind::EXTENDED, &composed).await?;
        self.expect_ok(id).await
    }

    /// **S20's ordering.** `fsync@openssh.com` against an open handle, so the
    /// staged content is durable before the name that publishes it exists.
    ///
    /// **This is ordering rather than atomicity**: it makes a machine that
    /// loses power after publication unable to present a published name over
    /// undurable bytes. It does not close Fact 44.
    pub async fn make_durable(&mut self, handle: &Handle) -> Result<(), SubsystemFailure> {
        if !self.announcement.announces(extension::FSYNC) {
            return Err(SubsystemFailure::ThatCapabilityWasNotAnnounced);
        }
        let id = self.take_an_id();
        let mut composed = Composed::new();
        composed
            .u32(id)
            .string(extension::FSYNC.as_bytes())
            .string(handle.bytes());
        self.send(kind::EXTENDED, &composed).await?;
        self.expect_ok(id).await
    }

    /// **F2's first route.** `statvfs@openssh.com` **against a named path**.
    ///
    /// Documented — `sftp(1)`: `df` *"Display usage information for the
    /// filesystem holding the current directory (or path if specified) … This
    /// command is only supported on servers that implement the
    /// 'statvfs@openssh.com' extension."*
    pub async fn filesystem_statistics(
        &mut self,
        path: &[u8],
    ) -> Result<FilesystemStatistics, SubsystemFailure> {
        if !self.announcement.announces(extension::STATVFS) {
            return Err(SubsystemFailure::ThatCapabilityWasNotAnnounced);
        }
        let id = self.take_an_id();
        let mut composed = Composed::new();
        composed
            .u32(id)
            .string(extension::STATVFS.as_bytes())
            .string(path);
        self.send(kind::EXTENDED, &composed).await?;
        let (packet_kind, payload) = self.reply_to(id).await?;
        if packet_kind == kind::STATUS {
            return Err(refusal_in(&payload));
        }
        if packet_kind != kind::EXTENDED_REPLY {
            return Err(SubsystemFailure::TheAnswerCouldNotBeRead);
        }
        // OpenSSH's `PROTOCOL` §4.4 gives the reply in the order of the C
        // structure: bsize, frsize, blocks, bfree, bavail, files, ffree,
        // favail, fsid, flag, namemax.
        let mut fields = after_the_identifier(&payload)
            .ok_or(SubsystemFailure::TheAnswerCouldNotBeRead)?;
        let read = |value: Option<u64>| value.ok_or(SubsystemFailure::TheAnswerCouldNotBeRead);
        let _block_size = read(fields.u64())?;
        let fragment_size = read(fields.u64())?;
        let blocks = read(fields.u64())?;
        let free_blocks = read(fields.u64())?;
        let available_blocks = read(fields.u64())?;
        Ok(FilesystemStatistics {
            fragment_size,
            blocks,
            free_blocks,
            available_blocks,
        })
    }

    /// **§1.1's planted-link carriage, performed.**
    ///
    /// The path is written as the **target of a symbolic link** whose own name
    /// is a class-2 token, so the value crosses as a protocol field and the
    /// constant command names the token, never the value.
    ///
    /// **The argument order is OpenSSH's, which is the reverse of the
    /// specification's.** Documented, `PROTOCOL` §4.1, read 2026-08-22: *"the
    /// order of the arguments to the SSH_FXP_SYMLINK method was inadvertently
    /// reversed … For correct operation, clients should send SSH_FXP_SYMLINK as
    /// follows: uint32 id, string targetpath, string linkpath."* **A builder
    /// who sends the specification's order plants a link the other way round
    /// and points a command at nothing**, which is why the parameter names here
    /// say which is which.
    pub async fn plant_a_link(
        &mut self,
        target_path: &[u8],
        link_path: &[u8],
    ) -> Result<(), SubsystemFailure> {
        let id = self.take_an_id();
        let mut composed = Composed::new();
        composed.u32(id).string(target_path).string(link_path);
        self.send(kind::SYMLINK, &composed).await?;
        self.expect_ok(id).await
    }

    /// The absolute form of a path, as the server resolves it.
    pub async fn resolve(&mut self, path: &[u8]) -> Result<Vec<u8>, SubsystemFailure> {
        self.one_name_from(kind::REALPATH, path).await
    }

    /// **A symbolic link's own target, read and never resolved.**
    ///
    /// See [`kind::READLINK`] for the operation, its citation and why this is
    /// not [`Subsystem::resolve`]. **A path that is not a link, or is not there
    /// at all, is the server's own refusal** and is returned as one rather than
    /// turned into an absence here.
    pub async fn read_link(&mut self, path: &[u8]) -> Result<Vec<u8>, SubsystemFailure> {
        self.one_name_from(kind::READLINK, path).await
    }

    /// The one-name reply both of the two above take, read the same way once.
    ///
    /// **The payload begins with the request identifier**, which is what
    /// correlates it, and reading through it rather than past it is the fault
    /// three of this crate's six readers carried on the day they were written.
    async fn one_name_from(
        &mut self,
        request: u8,
        path: &[u8],
    ) -> Result<Vec<u8>, SubsystemFailure> {
        let id = self.take_an_id();
        let mut composed = Composed::new();
        composed.u32(id).string(path);
        self.send(request, &composed).await?;
        let (packet_kind, payload) = self.reply_to(id).await?;
        match packet_kind {
            kind::NAME => {
                let mut fields = after_the_identifier(&payload)
                    .ok_or(SubsystemFailure::TheAnswerCouldNotBeRead)?;
                let count = fields
                    .u32()
                    .ok_or(SubsystemFailure::TheAnswerCouldNotBeRead)?;
                if count == 0 {
                    return Err(SubsystemFailure::TheAnswerCouldNotBeRead);
                }
                let name = fields
                    .string()
                    .ok_or(SubsystemFailure::TheAnswerCouldNotBeRead)?;
                Ok(name.to_vec())
            }
            kind::STATUS => Err(refusal_in(&payload)),
            _ => Err(SubsystemFailure::TheAnswerCouldNotBeRead),
        }
    }

    /// A whole directory listing, read out to its end.
    ///
    /// **A re-opened enumeration re-reads that one directory from its start**,
    /// and what it cannot promise is what no re-opened enumeration can: a
    /// directory whose contents changed between the two openings may yield an
    /// entry twice or not at all.
    pub async fn list(&mut self, path: &[u8]) -> Result<Vec<Entry>, SubsystemFailure> {
        let id = self.take_an_id();
        let mut composed = Composed::new();
        composed.u32(id).string(path);
        self.send(kind::OPENDIR, &composed).await?;
        let directory = self.expect_a_handle(id).await?;

        let mut entries = Vec::new();
        loop {
            let id = self.take_an_id();
            let mut composed = Composed::new();
            composed.u32(id).string(directory.bytes());
            self.send(kind::READDIR, &composed).await?;
            let (packet_kind, payload) = self.reply_to(id).await?;
            match packet_kind {
                kind::NAME => {
                    let mut fields = after_the_identifier(&payload)
                        .ok_or(SubsystemFailure::TheAnswerCouldNotBeRead)?;
                    let count = fields
                        .u32()
                        .ok_or(SubsystemFailure::TheAnswerCouldNotBeRead)?;
                    for _ in 0..count {
                        let name = fields
                            .string()
                            .ok_or(SubsystemFailure::TheAnswerCouldNotBeRead)?
                            .to_vec();
                        // Revision 3 sends a display form beside the name; it
                        // is read past and never used, being a rendering.
                        fields
                            .string()
                            .ok_or(SubsystemFailure::TheAnswerCouldNotBeRead)?;
                        let attributes = fields
                            .attributes()
                            .ok_or(SubsystemFailure::TheAnswerCouldNotBeRead)?;
                        entries.push(Entry { name, attributes });
                    }
                }
                kind::STATUS => match status_in(&payload) {
                    Some(StatusCode::EndOfFile) => break,
                    Some(code) => {
                        let _ = self.close(&directory).await;
                        return Err(SubsystemFailure::TheServerRefused(code));
                    }
                    None => return Err(SubsystemFailure::TheAnswerCouldNotBeRead),
                },
                _ => return Err(SubsystemFailure::TheAnswerCouldNotBeRead),
            }
        }
        self.close(&directory).await?;
        Ok(entries)
    }

    // ------------------------------------------------------------ the plumbing

    fn take_an_id(&mut self) -> u32 {
        let id = self.next_id;
        self.next_id = self.next_id.wrapping_add(1).max(1);
        id
    }

    async fn send(&mut self, packet_kind: u8, composed: &Composed) -> Result<(), SubsystemFailure> {
        self.stream
            .write_all(&composed.framed(packet_kind))
            .await
            .map_err(|_| SubsystemFailure::TheChannelIsGone)?;
        self.stream
            .flush()
            .await
            .map_err(|_| SubsystemFailure::TheChannelIsGone)
    }

    /// Reads until the reply carrying this identifier arrives, stashing any
    /// other reply on the way. **Correlation is by the identifier and never by
    /// arrival order**, which is C2's rule at this layer.
    async fn reply_to(&mut self, wanted: u32) -> Result<(u8, Vec<u8>), SubsystemFailure> {
        if let Some(stashed) = self.stashed.remove(&wanted) {
            return Ok(stashed);
        }
        loop {
            let (packet_kind, payload) = read_one_packet(&mut self.stream).await?;
            let mut fields = Fields::over(&payload);
            let id = fields
                .u32()
                .ok_or(SubsystemFailure::TheAnswerCouldNotBeRead)?;
            if id == wanted {
                return Ok((packet_kind, payload));
            }
            self.stashed.insert(id, (packet_kind, payload));
        }
    }

    async fn expect_ok(&mut self, id: u32) -> Result<(), SubsystemFailure> {
        let (packet_kind, payload) = self.reply_to(id).await?;
        if packet_kind != kind::STATUS {
            return Err(SubsystemFailure::TheAnswerCouldNotBeRead);
        }
        match status_in(&payload) {
            Some(StatusCode::Ok) => Ok(()),
            Some(code) => Err(SubsystemFailure::TheServerRefused(code)),
            None => Err(SubsystemFailure::TheAnswerCouldNotBeRead),
        }
    }

    async fn expect_a_handle(&mut self, id: u32) -> Result<Handle, SubsystemFailure> {
        let (packet_kind, payload) = self.reply_to(id).await?;
        match packet_kind {
            kind::HANDLE => {
                let mut fields = after_the_identifier(&payload)
                    .ok_or(SubsystemFailure::TheAnswerCouldNotBeRead)?;
                let bytes = fields
                    .string()
                    .ok_or(SubsystemFailure::TheAnswerCouldNotBeRead)?;
                Ok(Handle::of(bytes.to_vec()))
            }
            kind::STATUS => Err(refusal_in(&payload)),
            _ => Err(SubsystemFailure::TheAnswerCouldNotBeRead),
        }
    }
}

/// **Every reply's payload begins with the request identifier**, and this is the
/// one place that is read past.
///
/// `draft-ietf-secsh-filexfer-02` gives every response packet a `uint32 id`
/// first — a `STATUS`, a `HANDLE`, a `DATA`, a `NAME`, an `ATTRS` and an
/// `EXTENDED_REPLY` alike — because that is what correlates it with the request
/// (C2: *"A result is matched to a request by that identity and never by
/// arrival order"*). **A reader that forgets it reads the identifier as the
/// first field of the answer**, which is a wrong value rather than a failure:
/// a `DATA` reply then returns as many bytes as the request's own number.
fn after_the_identifier(payload: &[u8]) -> Option<Fields<'_>> {
    let mut fields = Fields::over(payload);
    fields.u32()?;
    Some(fields)
}

/// The status code inside a `STATUS` payload, which begins with the identifier.
fn status_in(payload: &[u8]) -> Option<StatusCode> {
    let mut fields = Fields::over(payload);
    fields.u32()?;
    Some(StatusCode::from_number(fields.u32()?))
}

fn refusal_in(payload: &[u8]) -> SubsystemFailure {
    match status_in(payload) {
        Some(code) => SubsystemFailure::TheServerRefused(code),
        None => SubsystemFailure::TheAnswerCouldNotBeRead,
    }
}

/// One framed packet: a four-byte length, then that many bytes, the first of
/// which is the type.
async fn read_one_packet<S: AsyncRead + Unpin>(
    stream: &mut S,
) -> Result<(u8, Vec<u8>), SubsystemFailure> {
    let mut length = [0u8; 4];
    stream
        .read_exact(&mut length)
        .await
        .map_err(|_| SubsystemFailure::TheChannelIsGone)?;
    let length = u32::from_be_bytes(length) as usize;
    if length == 0 {
        return Err(SubsystemFailure::TheAnswerCouldNotBeRead);
    }
    let mut whole = vec![0u8; length];
    stream
        .read_exact(&mut whole)
        .await
        .map_err(|_| SubsystemFailure::TheChannelIsGone)?;
    let packet_kind = whole[0];
    Ok((packet_kind, whole[1..].to_vec()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::subsystem::wire::Composed;

    /// **The finding this test exists for, and it was found by running rather
    /// than by reading.**
    ///
    /// Every reply's payload begins with the request identifier, and three
    /// readers here read past it and three did not. **The failure is a wrong
    /// value rather than an error**: a `DATA` reply returned as many bytes as
    /// the request's own number, so a download crept forward by a couple of
    /// kilobytes a time and an upload composed one write packet larger than the
    /// server's own maximum and hung. Nothing refused; nothing said anything.
    #[test]
    fn a_reply_is_read_past_its_identifier_and_not_through_it() {
        let mut composed = Composed::new();
        composed.u32(2034).string(b"the answer");
        let framed = composed.framed(kind::DATA);
        let payload = &framed[5..];

        let mut through_it = Fields::over(payload);
        // What the wrong reader did: it took the identifier for the length.
        assert_ne!(through_it.string(), Some(&b"the answer"[..]));

        let mut past_it = after_the_identifier(payload).expect("the identifier is there");
        assert_eq!(past_it.string(), Some(&b"the answer"[..]));
    }

    #[test]
    fn a_status_reply_is_read_past_its_identifier_too() {
        // This one was right all along, and it is asserted beside the one that
        // was wrong so the pair cannot drift apart again.
        let mut composed = Composed::new();
        composed.u32(7).u32(3);
        let framed = composed.framed(kind::STATUS);
        assert_eq!(
            status_in(&framed[5..]),
            Some(crate::subsystem::StatusCode::PermissionDenied)
        );
    }

    /// A far end that reads the client's opening packet and answers with the
    /// revision named, announcing nothing.
    async fn a_server_announcing(revision: u32) -> tokio::io::DuplexStream {
        let (ours, mut theirs) = tokio::io::duplex(4096);
        tokio::spawn(async move {
            let mut length = [0u8; 4];
            if theirs.read_exact(&mut length).await.is_err() {
                return;
            }
            let mut whole = vec![0u8; u32::from_be_bytes(length) as usize];
            if theirs.read_exact(&mut whole).await.is_err() {
                return;
            }
            let mut composed = Composed::new();
            composed.u32(revision);
            let _ = theirs.write_all(&composed.framed(kind::VERSION)).await;
            let _ = theirs.flush().await;
        });
        ours
    }

    #[tokio::test]
    async fn a_server_offering_another_revision_is_refused_and_the_number_is_named() {
        // **The fault this catches was silent.** The version was read into a
        // field and nothing compared it, so a server announcing anything else
        // was parsed as revision 3 anyway — its attributes coming back as wrong
        // sizes and wrong times rather than as a refusal.
        assert_eq!(
            Subsystem::negotiate(a_server_announcing(4).await).await.err(),
            Some(SubsystemFailure::TheServerOfferedAnotherRevision(4))
        );
        assert_eq!(
            Subsystem::negotiate(a_server_announcing(6).await).await.err(),
            Some(SubsystemFailure::TheServerOfferedAnotherRevision(6))
        );
    }

    #[tokio::test]
    async fn the_revision_this_document_uses_is_the_one_that_gets_through() {
        // The control on the other side of the same gate: revision 3 is what
        // four boxes across three sellers announced, and it negotiates.
        let subsystem = Subsystem::negotiate(a_server_announcing(3).await)
            .await
            .expect("revision 3 is the one this code is written against");
        assert_eq!(
            subsystem.announcement().version,
            Announcement::REVISION_THIS_DOCUMENT_USES
        );
    }

    /// **A far end that announces revision 3 and no extension at all, and then
    /// answers every packet it is sent with a status of success** — a server
    /// that would take a request it never offered. Every packet after the
    /// opening one is counted.
    async fn a_server_that_answers_what_it_never_announced(
        heard: std::sync::Arc<std::sync::atomic::AtomicUsize>,
    ) -> tokio::io::DuplexStream {
        let (ours, mut theirs) = tokio::io::duplex(4096);
        tokio::spawn(async move {
            let mut first = true;
            loop {
                let mut length = [0u8; 4];
                if theirs.read_exact(&mut length).await.is_err() {
                    return;
                }
                let mut whole = vec![0u8; u32::from_be_bytes(length) as usize];
                if theirs.read_exact(&mut whole).await.is_err() {
                    return;
                }
                let mut composed = Composed::new();
                if first {
                    first = false;
                    composed.u32(3);
                    let _ = theirs.write_all(&composed.framed(kind::VERSION)).await;
                } else {
                    heard.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                    let id = u32::from_be_bytes([whole[1], whole[2], whole[3], whole[4]]);
                    composed.u32(id).u32(0);
                    let _ = theirs.write_all(&composed.framed(kind::STATUS)).await;
                }
                let _ = theirs.flush().await;
            }
        });
        ours
    }

    /// **§1.2's rule for copy-data, held by the client itself: a server that
    /// never announced it is never sent it.** `File Browser` M7's copy checks
    /// the announcement before it asks, so no test of the act reaches this
    /// guard (step 10c's Files' owed acts piece 6, its bite 6g). **This far
    /// end would answer the request as done if it were sent**, so without the
    /// guard the call reads as a copy that happened. Step 10c's Files' owed
    /// acts piece 7, item 2.
    #[tokio::test]
    async fn copy_data_is_never_sent_to_a_server_that_did_not_announce_it() {
        let heard = std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0));
        let mut subsystem = Subsystem::negotiate(a_server_that_answers_what_it_never_announced(heard.clone()).await)
            .await
            .expect("revision 3 negotiates");
        assert!(!subsystem.announcement().announces(extension::COPY_DATA));
        let said = subsystem
            .copy_data(&Handle(b"from".to_vec()), &Handle(b"to".to_vec()))
            .await;
        assert_eq!(said, Err(SubsystemFailure::ThatCapabilityWasNotAnnounced));
        assert_eq!(heard.load(std::sync::atomic::Ordering::SeqCst), 0, "a packet reached the server");
    }

    #[test]
    fn a_payload_with_nothing_after_the_identifier_reads_as_nothing() {
        let mut composed = Composed::new();
        composed.u32(1);
        let framed = composed.framed(kind::NAME);
        let mut fields = after_the_identifier(&framed[5..]).expect("the identifier is there");
        assert_eq!(fields.u32(), None);
    }
}
