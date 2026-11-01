//! The real ends of §7's two desktop-side seams.
//!
//! **`Server protocol and transfers` S18 states the contract on the platform
//! file adapter and this is where it is kept.** *"What the operation is on each
//! of Mac, Windows and Linux, and its demonstration there, is that document's
//! under Fact 79"* — and it is [`crate::platform::files::publish`], which
//! already carries that per-system reasoning and its Windows negative.
//!
//! **Nothing here re-decides anything.** The four requirements are the adapter's
//! and the record's shape is the protocol document's; what is here is the
//! joining.
//!
//! **How a clause marker is written in this crate, and it is a rule rather than
//! a habit.** A marker with no document beside it means a clause of this
//! crate's own plan, `Preparation/Architecture/Desktop app and delivery.md`.
//! Every other document's marker is written with that document — at the site,
//! or in this file's own header list below. **Where a marker is a clause of
//! this crate's own plan *and* of the document meant, the header list is not
//! enough and the document is written out at the site**, because a bare one
//! there does not fail to resolve: it resolves to a real and wrong clause,
//! which nobody reading it can tell from a right one.
//!
//! **Markers used below that belong to another document, each with the document
//! it belongs to.** `Server protocol and transfers` S12, S13, S15,
//! S15a, S18, S20, S26a, S28. `State, identity and recovery` D5.
//!
//! **A marker this crate's own plan also defines is deliberately not listed
//! above, whichever document is meant, and is written out with its document
//! at every site instead** — the header list cannot tell the two apart, and a
//! bare one would resolve to a real and wrong clause.
//!
//! **This crate's own plan defines an S1, an S8, an S9 and a C11 of its own** —
//! the secret store per system, the three plaintext hops, the keychain dialog
//! on macOS, and what makes a judgement independent of every folding rule — and
//! **a K7, a K8 and a K9 of its own** — the Edit menu's three items, menu
//! labels going through the typography point, and the native menu bar — so none
//! of those seven is listed above. **The list was not here at all until
//! 2026-09-03**, and
//! this file is in a crate no marker sweep has ever covered: all five runs of
//! the step 4 gate swept three crates, and this module's own desktop half lives
//! in a fourth.

use std::io::{Read, Seek, SeekFrom, Write};
use std::path::Path;
use std::time::SystemTime;

use smoozy_file_browser::continuation::{PathIdentity, ReadsAPathIdentity};
use smoozy_server_protocol::release::trust::{
    RemembersTheServerPackageTrustRecord, TheTrustRecord,
};
use smoozy_server_protocol::transfers::moving::{KeepsTheRecord, LocalFailure, TheUsersOwnComputer};
use smoozy_server_protocol::transfers::publication::{Intent, WhatTheFinalObjectReadsAs};
use smoozy_server_protocol::transfers::{StableFileIdentity, TransferRecord};

use crate::platform::files;

/// **S18's contract, over the platform file adapter.**
///
/// Four requirements: the staged file appears at the final name in **one**
/// step; **no part-arrived bytes ever appear at the final name**; staging is on
/// the same filesystem as the final name; and **the adapter supplies the staged
/// and final objects' stable file identity where the platform has one**.
pub struct ThePlatformFileAdapter;

/// **How a create-intended publication is performed on this end, and why it is
/// not one operation. Changed at step 10a piece 12, 2026-09-13.**
///
/// `Server protocol and transfers` S15a wants the publication to refuse where a
/// file already stands at the final name. **Until this date this adapter
/// refused every create-intended publication**, on the ground that no single
/// operation renaming a file and refusing an existing target is established on
/// any of the three systems — and that ground is still true.
///
/// **What answers it is this crate's own plan, and it answered it on
/// 2026-08-18 without needing that operation.** `Desktop app and delivery` C10
/// puts the question *is this name already taken* to the filesystem as an
/// **exclusive create of the final name**, whose refusal is the filesystem's
/// own; the created zero-byte entry is the reservation; and C22 has the
/// publishing rename **replace that reservation in every case**. The window
/// between the two is C13's, declared and ruled acceptable by the Creator, and
/// `Server protocol and transfers` S15a admits the reservation by name. **The
/// look this adapter once made was a test followed by a rename, with nothing
/// holding the name between them; the reservation is what holds it.** The act
/// is `crate::platform::collision::publish_create_intended`.
pub const HOW_A_CREATE_INTENDED_PUBLICATION_IS_PERFORMED_HERE: &str =
    "the final name is created exclusively, which the filesystem refuses where \
     a file already stands, and the staged file is then renamed over that \
     reservation";

/// **F10b is not kept here, and this says so at the site rather than leaving a
/// clause with nothing to find. Added 2026-09-06, the day the clause was
/// written.**
///
/// **What the clause is.** F10b settles what unit a path reaches this adapter
/// in, and settles it by citation: `Server protocol and transfers` P7 rules the
/// whole transfer seam — bytes on the machine side, the receiving system's own
/// path unit on the desktop side, text only where a name is shown — and names
/// C3's name check as the single place a machine-side path's bytes become a
/// local one. **Two things follow for §6 and neither is a new rule**: every
/// call on this adapter's surface takes and returns a local path in the
/// system's own unit and never as text, which is what makes C15's *"comparing
/// those bytes against the bytes asked for is the whole test"* and C16's *"the
/// bytes asked for **are** the stored name"* performable rather than
/// aspirational; and F10a's third bullet answers in that same unit rather than
/// in a decoded spelling of it. F10b restates none of P7 and holds no second
/// version of it.
///
/// **Why nothing here names it.** Every one of the nine methods below takes its
/// path as `&str`, because that is what
/// `Server protocol and transfers`'s local-computer surface declares, and this
/// crate cannot change the shape of another document's trait from its
/// implementing side. So the calls that reach the filesystem convert on the
/// spot — `Path::new` at the exclusive create and again at both ends of the
/// publication — and every caller and every test above them hands in a
/// `to_string_lossy`. **A path whose bytes are not valid text cannot be named
/// to this adapter at all**, which is the opposite of what F10b binds, and a
/// marker written at any of these methods would record a promise as kept where
/// the code contradicts it. **The same gap runs through the durable store**:
/// the transfer record's five path columns are `TEXT`, in the one file that
/// also says of a neighbouring module's pair *"Every path is `BLOB` and never
/// `TEXT`, and that is load-bearing rather than tidy."*
///
/// **What removes it: the carrying out of the ruling**, whose work list is
/// `Audits/Build/Seam decisions/Paths across the transfer seam — the work
/// list.md`. That list's `Desktop app and delivery` section is step 2 of its
/// own order and waits on `Server protocol and transfers`, because the trait
/// and the transfer record's path fields are what this crate implements
/// against. **On this adapter it is the cheapest change in the list**: the
/// bodies already convert, so taking a local path deletes the conversion rather
/// than adding one, and the tests stop needing their `to_string_lossy`. **With
/// it go the five `TEXT` columns and a migration**, which is a re-read and
/// re-write of one installation's own rows rather than a conversion with a
/// guess in it. **When it lands, F10b is named here.**
///
/// **One thing the carrying out settles once and this crate cannot settle from
/// here**: the durable spelling of a local path on Windows, which has no byte
/// form in the standard library. The requirement is an exact round trip with no
/// substitution, and the model is already in this crate at
/// [`crate::platform::locations`], which reads both a byte path and a wide one
/// back into the system's own unit without a decode.
pub const WHY_F10B_IS_NOT_KEPT_HERE: &str =
    "every method of this adapter takes its path as text, because the \
     local-computer surface it implements declares it that way, so a path whose \
     bytes are not valid text cannot be named to it at all; the trait, the \
     transfer record's path fields and the five TEXT columns move together in \
     the path seam's own work list, and F10b is named here when they do";

impl ThePlatformFileAdapter {
    /// **The publication, saying beside its refusal whether C10's exclusive
    /// create found the final name taken** — `true` only there. The words are
    /// the same either way; what is added is that a caller can tell `Download`
    /// P6's case from every other refusal without reading them. Added
    /// 2026-10-31, the twenty-sixth notes, note 4: an auto-download's taken
    /// name read as an ordinary failure and was fetched again at every pass.
    pub(crate) fn publish_saying_whether_the_name_was_taken(
        &mut self,
        staging: &str,
        final_path: &str,
        intent: Intent,
    ) -> Result<(), (LocalFailure, bool)> {
        use crate::platform::collision::{self, WhyNothingWasPublished};
        match intent {
            Intent::CreateIntended => {
                collision::publish_create_intended(Path::new(staging), Path::new(final_path))
                    .map_err(|why| match why {
                        WhyNothingWasPublished::TheNameIsTaken { .. } => (
                            LocalFailure("File already exists. Not overwritten.".to_string()),
                            true,
                        ),
                        WhyNothingWasPublished::ItWasRefused(failure) => {
                            (LocalFailure(failure.reported), false)
                        }
                    })
            }
            Intent::ReplaceIntended => {
                collision::publish_replace_intended(Path::new(staging), Path::new(final_path))
                    .map_err(|failure| (LocalFailure(failure.reported), false))
            }
        }
    }
}

impl TheUsersOwnComputer for ThePlatformFileAdapter {
    fn staged_size(&mut self, path: &str) -> Option<u64> {
        std::fs::metadata(path).ok().map(|read| read.len())
    }

    fn read_at(&mut self, path: &str, offset: u64, length: usize) -> Result<Vec<u8>, LocalFailure> {
        let mut file = std::fs::File::open(path).map_err(said)?;
        file.seek(SeekFrom::Start(offset)).map_err(said)?;
        let mut buffer = vec![0u8; length];
        let mut filled = 0;
        while filled < length {
            match file.read(&mut buffer[filled..]) {
                Ok(0) => break,
                Ok(read) => filled += read,
                Err(why) => return Err(said(why)),
            }
        }
        buffer.truncate(filled);
        Ok(buffer)
    }

    /// **The exclusive create**, so two attempts can never land on one object,
    /// and the Smoozy-named directory beside the final name is made where it is
    /// not there (**`Server protocol and transfers` S1**: staged material sits
    /// on the same filesystem as the final name, in a Smoozy-named directory
    /// beside it, because publication is a rename and rename is documented on
    /// both ends to require it). **That document is written out because this
    /// crate's own plan defines an S1 too** — the secret store per system — and
    /// a bare marker here resolves to it.
    fn create_staging_exclusively(&mut self, path: &str) -> Result<(), LocalFailure> {
        if let Some(parent) = Path::new(path).parent() {
            files::ensure_directory(parent).map_err(|failure| LocalFailure(failure.reported))?;
        }
        std::fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(path)
            .map(|_| ())
            .map_err(said)
    }

    fn write_at(&mut self, path: &str, offset: u64, bytes: &[u8]) -> Result<(), LocalFailure> {
        let mut file = std::fs::OpenOptions::new()
            .write(true)
            .open(path)
            .map_err(said)?;
        file.seek(SeekFrom::Start(offset)).map_err(said)?;
        file.write_all(bytes).map_err(said)
    }

    /// **S20's ordering on this end**, through the adapter's own
    /// device-level flush — which on macOS is `F_FULLFSYNC` rather than
    /// `fsync`, because Apple's own manual says the plain call does not reach
    /// the platters.
    fn make_durable(&mut self, path: &str) -> Result<(), LocalFailure> {
        let file = std::fs::File::open(path).map_err(said)?;
        files::flush_to_the_device(&file).map_err(said)
    }

    /// **F15a's own branch where the call refuses**: the identity is reported
    /// as unavailable rather than substituted for, and S15's
    /// identity-unavailable branch takes it from there.
    fn stable_identity_of(&mut self, path: &str) -> StableFileIdentity {
        match the_file_identifier_of(Path::new(path)) {
            Some(identifier) => StableFileIdentity::The(identifier),
            None => StableFileIdentity::TheSupplierCouldNotAnswer,
        }
    }

    /// **`Server protocol and transfers` S15a's two intents**, and each one is
    /// **one operation** on the final name.
    ///
    /// **Replace-intended** is C14's deliberate replacement: the platform's own
    /// replacing rename, which [`files::publish`] carries with its per-system
    /// reasoning, and **no reservation is taken**.
    ///
    /// **Create-intended requires that no file at the final name is
    /// overwritten, and the refusal has to be the filesystem's own.** It used to
    /// be a look followed by a rename, and **between those two steps anything
    /// can put a file there** — the user saving out of Blender into the same
    /// folder, another transfer — which would then be replaced silently.
    /// **C10's exclusive create is the refusal and C22's rename over the
    /// reservation is the publication**, as
    /// [`HOW_A_CREATE_INTENDED_PUBLICATION_IS_PERFORMED_HERE`] says; C12 is
    /// what a taken name answers.
    fn publish(
        &mut self,
        staging: &str,
        final_path: &str,
        intent: Intent,
    ) -> Result<(), LocalFailure> {
        self.publish_saying_whether_the_name_was_taken(staging, final_path, intent)
            .map_err(|(failure, _)| failure)
    }

    fn remove(&mut self, path: &str) -> Result<(), LocalFailure> {
        files::remove_file(Path::new(path)).map_err(|failure| LocalFailure(failure.reported))
    }

    fn read_the_final_object(&mut self, path: &str) -> WhatTheFinalObjectReadsAs {
        match std::fs::metadata(path) {
            Ok(read) => WhatTheFinalObjectReadsAs {
                exists: true,
                byte_length: Some(read.len()),
                identity: match self.stable_identity_of(path) {
                    StableFileIdentity::The(identity) => Some(identity),
                    StableFileIdentity::TheSupplierCouldNotAnswer => None,
                },
                // **The at-rest digest is not read here.** Reading a whole
                // published file back to digest it is the cost S12 step 3
                // measures at 5.9 MB/s, and S15's third bullet is explicit that
                // *"a value with no route to read it back does not fail the
                // publication"* — the pre-state already carries the staged
                // object's verified byte length and digest, and the rename is
                // one step.
                at_rest_digest: None,
            },
            Err(_) => WhatTheFinalObjectReadsAs::default(),
        }
    }

    /// **`Server protocol and transfers` S15c, an upload's half, where no
    /// watch holds `File Browser` K7's arrangement identity**: the same
    /// reading by path, taken now, in whole seconds.
    fn the_sources_time(&mut self, source_path: &str) -> Option<i64> {
        let read = self.read_by_path(source_path)?;
        i64::try_from(read.modification_time_nanoseconds.div_euclid(1_000_000_000)).ok()
    }

    /// **`Server protocol and transfers` S15c, a download's half**: the staged
    /// file takes the server file's modification time, and the access time
    /// with it, before the pre-state.
    fn keep_the_time(&mut self, staging: &str, seconds: i64) -> Result<(), LocalFailure> {
        let at = if seconds >= 0 {
            SystemTime::UNIX_EPOCH + std::time::Duration::from_secs(seconds as u64)
        } else {
            SystemTime::UNIX_EPOCH - std::time::Duration::from_secs(seconds.unsigned_abs())
        };
        let file = std::fs::OpenOptions::new()
            .write(true)
            .open(staging)
            .map_err(said)?;
        file.set_times(std::fs::FileTimes::new().set_accessed(at).set_modified(at))
            .map_err(said)
    }
}

fn said(error: std::io::Error) -> LocalFailure {
    LocalFailure(error.to_string())
}

/// **F15a's pair, rendered once.** The volume and the file, joined — *"The
/// st_ino and st_dev fields taken together uniquely identify the file within
/// the system"*, and never the file half alone, which is unique only within a
/// filesystem.
///
/// **It is one function because two callers read it**: `Server protocol and
/// transfers` S18's stable file identity and `File Browser` K7's *file
/// identifier where the system has one*. Two renderings of one pair would be
/// two spellings of one fact, and `File Browser` K7's comparison is by
/// equality.
fn the_file_identifier_of(path: &Path) -> Option<String> {
    files::file_identity(path).map(|identity| format!("{}:{}", identity.volume, identity.file))
}

/// **`File Browser` K7's arrangement identity, over this crate's platform file
/// adapter.**
///
/// *"When a transfer is arranged — and again when a quiet period ends — the
/// source's path identity is read and recorded: size, modification time at the
/// finest the system reports, and the file identifier where the system has one.
/// **Read by path, never by handle.**"*
///
/// **`File Browser` §12 assigns it here in its own words** — *"Local reads this
/// document makes — the enumeration walk, the stored-name reading of source
/// names, `File Browser` K7's path identities and `File Browser` K9's
/// read-through — all through the platform
/// file adapter"*, that document written into the quotation at each marker
/// because this crate's own plan defines a K7 and a K9 too — so this is the
/// real end of a trait that already existed, and
/// nothing about the reading is re-decided.
///
/// **Never by handle, kept structurally**: the trait takes a path and there is
/// no route into this from an open file. Fact 30 showed a held handle going on
/// delivering the pre-save file after a swap-in, so a reading taken through one
/// would be a reading of the version the save replaced.
///
/// **The modification time is `std::fs::Metadata::modified`, one call on all
/// three systems**, and it carries whatever precision the system it ran on
/// gives. That is exactly `File Browser` K7's *at the finest the system
/// reports*, with that clause's own answer for a system that reports less: the
/// low digits are zeroes rather than the value being rounded here. **What
/// precision each of the three actually reports is not established by this
/// project**, and nothing is written here that assumes one — the reading also
/// carries the size and the file identifier, and `File Browser` K9's gate is
/// contents arithmetic at publication, so a coarse clock costs earliness and
/// never correctness (that clause's own reasoning, not a new one).
///
/// **Where any part of the reading refuses, the answer is nothing**, which
/// `File Browser` K8 reads as *it could not be read* and which claims nothing
/// about the user's file. No field is substituted for and no default is
/// composed.
impl ReadsAPathIdentity for ThePlatformFileAdapter {
    fn read_by_path(&mut self, path: &str) -> Option<PathIdentity> {
        let at = Path::new(path);
        let found = std::fs::metadata(at).ok()?;
        let modified = found.modified().ok()?;
        let modification_time_nanoseconds = match modified.duration_since(SystemTime::UNIX_EPOCH) {
            Ok(since) => since.as_nanos() as i128,
            // A file whose modification time is before the epoch. It is a
            // value and not a fault, and it is carried as itself.
            Err(before) => -(before.duration().as_nanos() as i128),
        };
        Some(PathIdentity {
            size: found.len(),
            modification_time_nanoseconds,
            file_identifier: the_file_identifier_of(at),
            // **`State, identity and recovery` D5: it is a reading and
            // carries its moment**, on the desktop's
            // own clock. `File Browser` K7 compares one path's readings against
            // each other, so no clocks are mixed.
            read_at: SystemTime::now(),
        })
    }
}

/// **`Server protocol and transfers` S8's writer.** The record is put into the
/// local record, durably, before the first byte and at every checkpoint.
///
/// **That document is written out because this crate's own plan defines an S8
/// too** — the three plaintext hops — and a bare marker here resolves to it.
pub struct TheLocalRecord<'a> {
    /// **Taken by exclusive borrow**, which is what makes this seam something a
    /// transfer can be handed: a shared borrow of the engine's own connection
    /// is not something that may cross a thread, and a transfer runs on one.
    pub connection: &'a mut rusqlite::Connection,
}

impl KeepsTheRecord for TheLocalRecord<'_> {
    fn write_it_down(&mut self, record: &TransferRecord) -> Result<(), LocalFailure> {
        crate::record::transfer_records::put(&*self.connection, record)
            .map_err(|failure| LocalFailure(format!("{failure:?}")))
    }
}

/// **`Server protocol and transfers` B9's durable memory, kept in the one local
/// record** — `State, identity and recovery` J6's server-package trust record.
///
/// **The rule is that document's and the store is this one's**, which is the
/// same split every row module here draws. What crosses this seam is the row
/// and never a verdict: whether the remembered numbers stand at all is a
/// signature check under the release root, and that check is
/// `Server protocol and transfers`' own.
pub struct TheServerPackageTrustRecord<'a> {
    pub connection: &'a mut rusqlite::Connection,
}

impl RemembersTheServerPackageTrustRecord for TheServerPackageTrustRecord<'_> {
    fn read_it(&self) -> Result<Option<TheTrustRecord>, String> {
        crate::record::server_package_trust::read(&*self.connection)
            .map_err(|failure| format!("{failure:?}"))
    }

    fn write_it(&mut self, record: &TheTrustRecord) -> Result<(), String> {
        crate::record::server_package_trust::write(&*self.connection, record)
            .map_err(|failure| format!("{failure:?}"))
    }

    fn discard_it(&mut self) -> Result<(), String> {
        crate::record::server_package_trust::discard(&*self.connection)
            .map_err(|failure| format!("{failure:?}"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use smoozy_server_protocol::transfers::integrity::Digest;
    use smoozy_server_protocol::transfers::{Checkpoint, Direction, Ending, StoppedReason};
    use smoozy_state::identities::{Identity, IdentityKind};

    /// **The whole desktop-side acceptance, through the real local record.**
    ///
    /// **`Server protocol and transfers` B9's *durably remembers* proved rather
    /// than described**: a release is accepted, the record is written, and the
    /// same copy then refuses a genuine older manifest — with the record closed
    /// and opened again in between, so what refuses is what is on disk.
    ///
    /// **The keys are derived from seeds written out here**, so they are public
    /// and are not release keys. The release key is the Creator's.
    #[test]
    fn a_release_accepted_through_the_local_record_refuses_an_older_one_afterwards() {
        use smoozy_server_protocol::release::composing;
        use smoozy_server_protocol::release::trust::{
            AnAllowedKey, KeySetStatement,
            THE_GREATEST_KEY_SET_GENERATION_PUBLISHED_WHEN_THIS_BUILD_WAS_MADE,
            THE_GREATEST_RELEASE_SEQUENCE_PUBLISHED_WHEN_THIS_BUILD_WAS_MADE, THE_KEY_SET_FORMAT,
        };
        use smoozy_server_protocol::release::verifying::{
            accept_a_release, AnOfferedRelease, WhyTheReleaseWasNotAccepted,
            WhyTheReleaseWasRefused,
        };
        use smoozy_server_protocol::release::{
            a_signature_over, THE_KEY_SET_SIGNING_CONTEXT, THE_MANIFEST_SIGNING_CONTEXT,
        };

        let root = composing::test_only_keys::a_key_pair_from_a_written_down_seed(
            *b"a release root for this test onl",
        );
        let online = composing::test_only_keys::a_key_pair_from_a_written_down_seed(
            *b"an online signing key, this test",
        );
        // **Both releases are above the floor this build carries**, so what
        // refuses the older one is the record on disk and nothing else. *(Until
        // 2026-09-21 they were sequences 20 and 19 at generation 1, which only
        // said that while `Server protocol and transfers` B9's built-in pair was
        // zero; with the pair at the first published release's numbers both
        // would be refused by the build before the record was ever consulted.)*
        let above_this_builds_floor = THE_GREATEST_RELEASE_SEQUENCE_PUBLISHED_WHEN_THIS_BUILD_WAS_MADE;
        let statement = KeySetStatement {
            statement_format: THE_KEY_SET_FORMAT.into(),
            generation: THE_GREATEST_KEY_SET_GENERATION_PUBLISHED_WHEN_THIS_BUILD_WAS_MADE,
            allowed: vec![AnAllowedKey {
                identity: "the online signing key".into(),
                public_key: online.public,
                valid_from_release_sequence: 1,
            }],
            revocations: Vec::new(),
        };
        let statement_bytes = statement.canonical_bytes();
        let root_signature =
            a_signature_over(&root.secret, THE_KEY_SET_SIGNING_CONTEXT, &statement_bytes);

        let offer = |release_sequence: u64| {
            let artifact = composing::compose_an_artifact_entry(
                "linux-x86_64-gnu",
                composing::the_tuple_this_build_is_made_for(),
                b"a machine program, for the sake of the argument",
                composing::THE_FIXED_ENTRY_POINT_NAME,
                &["https://example.invalid/smoozy-server".into()],
            );
            let manifest =
                composing::compose_a_manifest(release_sequence, "0.1.0", 2, None, vec![artifact]);
            let manifest_bytes = manifest.canonical_bytes();
            AnOfferedRelease {
                the_key_set_statements_bytes: statement_bytes.clone(),
                its_root_signature: root_signature,
                its_online_signature: a_signature_over(
                    &online.secret,
                    THE_MANIFEST_SIGNING_CONTEXT,
                    &manifest_bytes,
                ),
                the_manifests_bytes: manifest_bytes,
            }
        };

        let scratch = std::env::temp_dir().join(format!(
            "smoozy-trust-record-{}",
            std::process::id()
        ));
        std::fs::create_dir_all(&scratch).expect("a temporary directory");
        let at = scratch.join("record.sqlite");

        let open = || {
            let connection = rusqlite::Connection::open(&at).expect("the record opens");
            for statement in crate::record::schema::VERSION_11 {
                let _ = connection.execute(statement, []);
            }
            connection
        };

        {
            let mut connection = open();
            let mut store = TheServerPackageTrustRecord {
                connection: &mut connection,
            };
            // **The tuple below is the constant naming what this build
            // produced, and it is the right thing here.** This test has no
            // machine to ask. `Server protocol and transfers` B8's detected
            // rental tuple is supplied by `Server Setup`, which reads it off a
            // machine over the command channel — so the reading exists and
            // there is nothing here to take it from.
            //
            // *(Reason corrected 2026-09-07. It said nothing in this workspace
            // read a processor architecture or a C library generation off a
            // machine, which stopped being true that morning. The stand-in is
            // unchanged.)*
            accept_a_release(
                &root.public,
                &mut store,
                &offer(above_this_builds_floor + 20),
                &composing::the_tuple_this_build_is_made_for(),
                None,
            )
            .expect("the release is accepted");
        }

        {
            let mut connection = open();
            let mut store = TheServerPackageTrustRecord {
                connection: &mut connection,
            };
            assert_eq!(
                accept_a_release(
                    &root.public,
                    &mut store,
                    &offer(above_this_builds_floor + 19),
                    &composing::the_tuple_this_build_is_made_for(),
                    None
                ),
                Err(WhyTheReleaseWasNotAccepted::TheRelease(
                    WhyTheReleaseWasRefused::ItsReleaseSequenceIsBelowTheFloorInForce
                ))
            );
        }

        let _ = std::fs::remove_dir_all(&scratch);
    }

    /// The transfer tables as this binary knows them — version 6 made them and
    /// version 8 added `Server protocol and transfers` S9's per-attempt
    /// identity, so a test that ran version 6 alone would be writing against a
    /// shape no record on a user's computer has.
    fn the_transfer_tables(connection: &rusqlite::Connection) {
        for statement in crate::record::schema::VERSION_6
            .iter()
            .chain(crate::record::schema::VERSION_8)
        {
            connection.execute(statement, []).expect("the tables");
        }
    }

    fn a_scratch(name: &str) -> std::path::PathBuf {
        let path = std::env::temp_dir().join(format!(
            "smoozy-transfer-adapter-{name}-{}",
            std::process::id()
        ));
        let _ = std::fs::remove_dir_all(&path);
        std::fs::create_dir_all(&path).expect("a temporary directory");
        path
    }

    #[test]
    fn a_staged_download_takes_the_servers_time_and_an_upload_reads_the_sources() {
        // `Server protocol and transfers` S15c, on this computer's own end:
        // the staged file takes the time it is handed, and the source's time
        // is read back in whole seconds.
        let scratch = a_scratch("times");
        let staged = scratch.join("staged");
        std::fs::write(&staged, b"a frame").expect("written");
        let mut adapter = ThePlatformFileAdapter;
        adapter
            .keep_the_time(&staged.to_string_lossy(), 1_767_225_600)
            .expect("the time is kept");
        assert_eq!(
            std::fs::metadata(&staged).and_then(|read| read.modified()).expect("a time"),
            SystemTime::UNIX_EPOCH + std::time::Duration::from_secs(1_767_225_600)
        );
        assert_eq!(adapter.the_sources_time(&staged.to_string_lossy()), Some(1_767_225_600));
        let _ = std::fs::remove_dir_all(&scratch);
    }

    #[test]
    fn a_create_intended_publication_never_replaces_a_file_at_the_final_name() {
        // `Server protocol and transfers` S15a. The refusal is the answer
        // rather than a fault — and it is the operation's own refusal rather
        // than a look, because between a look and a rename anything can put a
        // file at the final name.
        let scratch = a_scratch("create");
        let staged = scratch.join("staged");
        let final_name = scratch.join("final");
        std::fs::write(&staged, b"new").expect("written");
        std::fs::write(&final_name, b"old").expect("written");
        let mut adapter = ThePlatformFileAdapter;
        assert!(adapter
            .publish(
                &staged.to_string_lossy(),
                &final_name.to_string_lossy(),
                Intent::CreateIntended
            )
            .is_err());
        assert_eq!(std::fs::read(&final_name).expect("still there"), b"old");
        let _ = std::fs::remove_dir_all(&scratch);
    }

    #[test]
    fn a_create_intended_publication_lands_where_nothing_stands_and_never_by_a_look() {
        // **C10 and C22, through the adapter.** The old code looked at the
        // final name first and renamed over it, so the window between the look
        // and the rename was silent; then it refused every create-intended
        // publication rather than look. **The reservation is what holds the
        // name now**, so a free final name publishes and the staged object is
        // taken by the rename — and the test above still holds for a taken one.
        let scratch = a_scratch("create-free");
        let staged = scratch.join("staged");
        let final_name = scratch.join("nothing-stands-here");
        std::fs::write(&staged, b"new").expect("written");
        let mut adapter = ThePlatformFileAdapter;
        adapter
            .publish(
                &staged.to_string_lossy(),
                &final_name.to_string_lossy(),
                Intent::CreateIntended,
            )
            .expect("the name was free, so the reservation was made and renamed over");
        assert_eq!(std::fs::read(&final_name).expect("there"), b"new");
        assert!(!staged.exists());
        let source = include_str!("transfers.rs");
        assert!(
            !source.contains(&["WHY_A_CREATE_INTENDED", "_PUBLICATION_IS_DEGRADED_HERE"].concat()),
            "the declared degradation came back"
        );
        let _ = std::fs::remove_dir_all(&scratch);
    }

    #[test]
    fn a_replace_intended_publication_lands_in_one_step() {
        let scratch = a_scratch("replace");
        let staged = scratch.join("staged");
        let final_name = scratch.join("final");
        std::fs::write(&staged, b"new").expect("written");
        std::fs::write(&final_name, b"old").expect("written");
        let mut adapter = ThePlatformFileAdapter;
        adapter
            .publish(
                &staged.to_string_lossy(),
                &final_name.to_string_lossy(),
                Intent::ReplaceIntended,
            )
            .expect("the adapter publishes");
        assert_eq!(std::fs::read(&final_name).expect("there"), b"new");
        assert!(!staged.exists());
        let _ = std::fs::remove_dir_all(&scratch);
    }

    /// **`File Browser` K7's reading, over the real adapter and a real file.**
    ///
    /// The three fields it names are read, and the one that says *where the
    /// system has one* answers on this system.
    #[test]
    fn the_path_identity_reads_the_three_things_k7_names() {
        let scratch = a_scratch("identity");
        let file = scratch.join("scene.blend");
        std::fs::write(&file, b"twenty-one bytes here").expect("written");
        let mut adapter = ThePlatformFileAdapter;

        let read = adapter
            .read_by_path(&file.to_string_lossy())
            .expect("the reading answers on a file that is there");
        assert_eq!(read.size, 21);
        assert!(read.modification_time_nanoseconds > 0);
        // **On this system there is a file identifier**, and it is F15a's pair
        // rendered once — the same value `stable_identity_of` reports, because
        // two spellings of one pair would not compare equal.
        assert_eq!(
            read.file_identifier,
            Some(the_file_identifier_of(&file).expect("this system has one"))
        );
        assert_eq!(
            adapter.stable_identity_of(&file.to_string_lossy()),
            StableFileIdentity::The(read.file_identifier.clone().expect("there"))
        );

        // **A path that is not there answers nothing**, which `File Browser`
        // `File Browser` K8 reads as *it could not be read* and which claims
        // nothing about the
        // user's file.
        assert!(adapter
            .read_by_path(&scratch.join("nothing-here").to_string_lossy())
            .is_none());
        let _ = std::fs::remove_dir_all(&scratch);
    }

    /// **`File Browser` K8's watch, over the real adapter and a real save.**
    ///
    /// **This is the test that would have caught the fault** the wiring exists
    /// to end: with no real end for the reading, that module's watch was
    /// exercised only against a fake that swapped the file under it, and
    /// nothing said whether an actual file on an actual disk moves any of the
    /// three fields `File Browser` K7 records.
    ///
    /// The save shape here is `File Browser` K6's **in place** one — the same
    /// byte count rewritten under the same file identifier — which is the shape
    /// that defeats both the size and the identifier and leaves only the
    /// modification time.
    #[test]
    fn a_save_in_place_moves_the_reading_that_k8_compares() {
        use smoozy_file_browser::continuation::{what_the_watch_saw, WhatTheWatchSaw};

        let scratch = a_scratch("watch");
        let file = scratch.join("texture.png");
        std::fs::write(&file, b"the first version...").expect("written");
        let mut adapter = ThePlatformFileAdapter;
        let at_arrangement = adapter
            .read_by_path(&file.to_string_lossy())
            .expect("the arrangement reading");

        // The control first: nothing was saved, so nothing moved.
        let unchanged = adapter.read_by_path(&file.to_string_lossy());
        assert_eq!(
            what_the_watch_saw(&at_arrangement, unchanged.as_ref()),
            WhatTheWatchSaw::Unchanged
        );

        // **In place, and byte-count-identical** — the shape `File Browser` K6
        // observed of a
        // texture export.
        std::thread::sleep(std::time::Duration::from_millis(20));
        std::fs::write(&file, b"the second version..").expect("rewritten");
        assert_eq!(
            std::fs::metadata(&file).expect("there").len(),
            at_arrangement.size,
            "the save is byte-count-identical, which is what makes the size useless here"
        );

        let now = adapter.read_by_path(&file.to_string_lossy());
        assert_eq!(
            what_the_watch_saw(&at_arrangement, now.as_ref()),
            WhatTheWatchSaw::ItChanged
        );
        let _ = std::fs::remove_dir_all(&scratch);
    }

    #[test]
    fn the_staging_file_cannot_be_created_twice() {
        let scratch = a_scratch("exclusive");
        let staged = scratch.join("beside/staged");
        let mut adapter = ThePlatformFileAdapter;
        adapter
            .create_staging_exclusively(&staged.to_string_lossy())
            .expect("the first create makes the folder and the file");
        assert!(adapter
            .create_staging_exclusively(&staged.to_string_lossy())
            .is_err());
        let _ = std::fs::remove_dir_all(&scratch);
    }

    #[test]
    fn a_record_round_trips_through_the_local_record() {
        // `Server protocol and transfers` S9's whole record, written and read
        // back — including the two things a packed field would have made
        // unreachable: the superseded attempt numbers and the checkpoints.
        // **That document is written out because this crate's own plan defines
        // an S9 too** — the keychain dialog on macOS — and a bare marker here
        // resolves to it.
        let connection = rusqlite::Connection::open_in_memory().expect("in memory");
        the_transfer_tables(&connection);
        let issue = |kind| Identity::issue(kind).expect("random bytes");
        let mut record = TransferRecord::arranged(
            issue(IdentityKind::Transfer),
            Direction::Down,
            issue(IdentityKind::LocalServer),
            Some(issue(IdentityKind::Installation)),
            "/machine/frame.exr".to_string(),
            "/home/user/Downloads/frame.exr".to_string(),
            "/home/user/Downloads/.smoozy-staging",
            4_294_967_296,
            issue(IdentityKind::Run),
        );
        record.expected_digest = Some(Digest::of(b"the whole file"));
        record.checkpoints.push(Checkpoint {
            offset: 4_194_304,
            digest_of_everything_up_to_it: Digest::of(b"the first four mebibytes"),
        });
        // The first attempt's staging object was created and its identity read,
        // so the start-over carries that identity across to the superseded
        // attempt rather than dropping it.
        record.staging_identity = Some(StableFileIdentity::The("volume-1:file-7".to_string()));
        record.start_over();
        record.checkpoints.push(Checkpoint {
            offset: 8_388_608,
            digest_of_everything_up_to_it: Digest::of(b"the first eight"),
        });
        record.ending = Some(Ending::Stopped(StoppedReason::AbandonedByItsRequest));
        record.cleanup_is_owed = true;

        crate::record::transfer_records::put(&connection, &record).expect("written");
        let back = crate::record::transfer_records::read(&connection, &record.transfer)
            .expect("read")
            .expect("it is there");
        assert_eq!(back, record);
        assert_eq!(back.superseded_attempts.len(), 1);
        assert_eq!(back.superseded_attempts[0].attempt, 1);
        // **The identity came back beside the number** — `Server protocol and
        // transfers` S9 — which is what `Server protocol and transfers` S28's
        // removal needs and what a bare number could not carry.
        assert_eq!(
            back.superseded_attempts[0].staging_identity,
            Some(StableFileIdentity::The("volume-1:file-7".to_string()))
        );
        assert_eq!(
            back.every_staging_path_it_has_named().len(),
            record.every_staging_path_it_has_named().len()
        );
    }

    /// **A download that starts over leaves no part-file in the user's own
    /// destination folder once the sweep has run.**
    ///
    /// **This is the test that would have caught the fault.** The superseded
    /// attempt's path was derived with no identity at all, and `Server protocol
    /// and transfers` S28's only removing branch wants the path, the
    /// synthesised name **and** the identity to match — so the branch could
    /// never run and the part-file stayed in the folder the user chose, for
    /// good, on every download that starts over. On a download the user's own
    /// computer **can** answer that identity, so this was never §12 item 15's
    /// declared break: the value was available and was thrown away at the
    /// start-over.
    ///
    /// It runs the real platform file adapter over a real folder, because the
    /// identity, the removal and the record are three different seams and a
    /// test on any one of them alone would have passed.
    #[test]
    fn a_download_that_starts_over_leaves_no_part_file_in_the_users_own_folder() {
        use smoozy_server_protocol::transfers::cleanup::{
            what_the_sweep_derives, what_the_sweep_does, WhatTheSweepDoes,
        };

        let scratch = a_scratch("sweep");
        let destination = scratch.join("Downloads");
        std::fs::create_dir_all(&destination).expect("the user's own folder");
        let staging_directory = destination.join(".smoozy-staging");

        let issue = |kind| Identity::issue(kind).expect("random bytes");
        let mut record = TransferRecord::arranged(
            issue(IdentityKind::Transfer),
            Direction::Down,
            issue(IdentityKind::LocalServer),
            Some(issue(IdentityKind::Installation)),
            "/machine/frame.exr".to_string(),
            destination.join("frame.exr").to_string_lossy().into_owned(),
            &staging_directory.to_string_lossy(),
            1024,
            issue(IdentityKind::Run),
        );

        let mut adapter = ThePlatformFileAdapter;

        // The first attempt: a staging object with part of the frame in it, and
        // its identity read the way the byte loop reads it.
        let abandoned = record.staging_path.as_text();
        adapter
            .create_staging_exclusively(&abandoned)
            .expect("the staging object");
        adapter
            .write_at(&abandoned, 0, b"four fifths of a frame")
            .expect("part of the frame");
        record.staging_identity = Some(adapter.stable_identity_of(&abandoned));

        // The staged prefix does not verify, so the transfer starts over into a
        // fresh path rather than being repaired in place.
        record.start_over();
        let live = record.staging_path.as_text();
        adapter
            .create_staging_exclusively(&live)
            .expect("the fresh staging object");

        // The sweep, run over what the record derives. The live attempt has not
        // ended, so nothing derives it; the superseded one is on the list.
        for derived in what_the_sweep_derives(std::slice::from_ref(&record)) {
            let at = derived.path.as_text();
            let identity_now = std::path::Path::new(&at)
                .exists()
                .then(|| adapter.stable_identity_of(&at));
            if what_the_sweep_does(&derived, identity_now.as_ref()) == WhatTheSweepDoes::Remove {
                adapter.remove(&at).expect("the sweep removes it");
            }
        }

        assert!(
            !std::path::Path::new(&abandoned).exists(),
            "the superseded attempt's part-file is still in the user's own folder"
        );
        // **And the live attempt is untouched**, which is the other half: a
        // sweep that took the running transfer's material would be worse than
        // one that took nothing.
        assert!(std::path::Path::new(&live).exists());

        let left: Vec<String> = std::fs::read_dir(&staging_directory)
            .expect("the staging folder")
            .flatten()
            .map(|entry| entry.file_name().to_string_lossy().into_owned())
            .collect();
        assert_eq!(left.len(), 1, "{left:?}");

        let _ = std::fs::remove_dir_all(&scratch);
    }

    #[test]
    fn an_ending_this_binary_does_not_know_is_not_read_as_interrupted() {
        // Reading an unknown word as *interrupted* would tell `Server protocol
        // and transfers` C11 step 4 to carry on work that is over, which is the
        // fault that document's S26a reversed. **That document is written out
        // because this crate's own plan defines a C11 too** — what makes a
        // judgement independent of every folding rule — and a bare marker here
        // resolves to it.
        let connection = rusqlite::Connection::open_in_memory().expect("in memory");
        the_transfer_tables(&connection);
        let issue = |kind| Identity::issue(kind).expect("random bytes");
        let record = TransferRecord::arranged(
            issue(IdentityKind::Transfer),
            Direction::Up,
            issue(IdentityKind::LocalServer),
            None,
            "/a".to_string(),
            "/b".to_string(),
            "/staging",
            1,
            issue(IdentityKind::Run),
        );
        crate::record::transfer_records::put(&connection, &record).expect("written");
        connection
            .execute(
                "UPDATE transfer_record SET ending = 'something else'",
                [],
            )
            .expect("updated");
        let back = crate::record::transfer_records::read(&connection, &record.transfer)
            .expect("read")
            .expect("there");
        assert_eq!(back.ending, None);
    }

    /// **Writing a transfer record again does not move it down the write
    /// order.**
    ///
    /// `crate::record::transfer_records::all` reads this table `ORDER BY
    /// rowid`, and `Server protocol and transfers` S8 puts the record down
    /// before the first byte **and again at every checkpoint** — so the write
    /// this test makes is the ordinary case rather than an unusual one. Under
    /// the replace this store used to make, the second write deleted the row
    /// and inserted a new one, which handed it a fresh row identifier and put
    /// the busiest transfer last however early it was arranged.
    ///
    /// **Nothing today reads this table for write order**, which is written out
    /// at `put` — `Server protocol and transfers` S28's sweep judges each
    /// record on its own. What the test
    /// holds is that the row identifier means the same thing here that it means
    /// in the upload-request table one file away, so that a later reader taking
    /// the answer from either is right.
    #[test]
    fn writing_a_transfer_record_again_does_not_move_it_down_the_write_order() {
        let connection = rusqlite::Connection::open_in_memory().expect("in memory");
        the_transfer_tables(&connection);
        let issue = |kind| Identity::issue(kind).expect("random bytes");
        let a_record = |source: &str| {
            TransferRecord::arranged(
                issue(IdentityKind::Transfer),
                Direction::Up,
                issue(IdentityKind::LocalServer),
                None,
                source.to_string(),
                "/machine/landing".to_string(),
                "/machine/.smoozy-staging",
                1024,
                issue(IdentityKind::Run),
            )
        };

        let mut older = a_record("/home/older.blend");
        let newer = a_record("/home/newer.blend");
        crate::record::transfer_records::put(&connection, &older).expect("the older is written");
        crate::record::transfer_records::put(&connection, &newer).expect("the newer is written");

        // The older transfer reaches a checkpoint, which is `Server protocol
        // and transfers` S8's own second write and every write after it.
        older.checkpoints.push(Checkpoint {
            offset: 512,
            digest_of_everything_up_to_it: Digest::of(b"the first half"),
        });
        crate::record::transfer_records::put(&connection, &older).expect("the checkpoint");

        let every = crate::record::transfer_records::all(&connection).expect("they read back");
        assert_eq!(
            every
                .iter()
                .map(|record| record.source_path.as_str())
                .collect::<Vec<_>>(),
            vec!["/home/older.blend", "/home/newer.blend"],
            "a rewritten record keeps the place it was first written down in"
        );
        // And the rewrite is a rewrite: the record is one row, carrying what
        // the second write said.
        assert_eq!(every.len(), 2);
        assert_eq!(every[0].checkpoints.len(), 1);
    }
}
