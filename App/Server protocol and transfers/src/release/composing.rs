//! The release side of B8 — composing an artifact entry from bytes that exist,
//! composing the two documents that carry it, and where the bytes are fetched
//! from.
//!
//! **What is composed here and what is not.** An artifact entry, a manifest and
//! a key-set statement are composed from values and measured bytes.
//! **Signing them is [`super::a_signature_over`] and it holds no key**, and
//! **fetching them is [`FetchesFromAnExactSourceTheManifestNamed`], whose
//! HTTPS implementation is [`super::fetching::AFetchOverHttps`] and whose one
//! missing value is the address** — those two are the Creator's, being a key
//! they hold and hosting they pay for, and every absence below says what
//! removes it.
//! *(Reworded 2026-09-07, when the fetcher landed. It said there was nowhere to
//! fetch from; there is nowhere named, which is not the same thing.)*
//!
//! **Markers used below that belong to another document, each with the
//! document it belongs to.** `Rented-server runtime` K16. `Server Setup` M1a.
//! `State, identity and recovery` §1.3 rule 6. `Desktop app and delivery` §19.
//!
//! **How a clause marker is written in this crate, and it is a rule rather
//! than a habit.** A marker with no document beside it means a clause of this
//! crate's own plan, `Preparation/Architecture/Server protocol and
//! transfers.md`. Every other document's marker is written with that
//! document — at the site, or in the file's own header list above.

use crate::transfers::integrity::Digest;

use super::manifest::{ArtifactEntry, ReleaseManifest, TargetTuple, THE_MANIFEST_FORMAT};

/// **The fixed entry-point name B8 requires the manifest to carry.**
///
/// **It is `Rented-server runtime`'s and is written twice on purpose**, that
/// crate being the one that defines what the program is and this one being the
/// one that has to put the name in a manifest — and the two crates cannot name
/// each other, the machine program having been taken off this crate's
/// dependency on 2026-09-06 precisely so that the SSH client and its C
/// cryptography stay out of its tree. **The second copy is held to the first by
/// a test that reads that crate's own source**, so the two cannot drift in
/// silence.
pub const THE_FIXED_ENTRY_POINT_NAME: &str = "smoozy-server";

/// **The one tuple this build is made for.**
///
/// `State, identity and recovery` §1.3 rule 6 makes every rented machine a
/// Linux machine, so there is one operating-system family rather than a list;
/// **x86-64 because every machine on this project's record was one**; and the
/// GNU C library because `Server Setup` M1a limits the supported families to
/// Ubuntu and Debian and both are that library's.
///
/// **The set of tuples the product supports is not this and is not Build's** —
/// B8 makes it the Creator-ruling boundary. What this constant says is which
/// artifact *this* build produces, and a manifest naming more than one entry is
/// composed by whoever holds that ruling.
///
/// **B8's detected rental tuple has a supplier, and this constant is still not
/// it.** That clause says *"`Server Setup` supplies the detected rental tuple"*
/// and **that document now does**: `Server Setup`'s `kind.rs` composes a tuple
/// from a machine's own three answers, and its `readings.rs` asks the machine
/// for them over the command channel, before any package byte exists there. A
/// machine that will not answer composes no tuple at all rather than falling
/// back to this constant, so the exact match now catches what it exists to
/// catch.
///
/// **What this constant is for is unchanged**: it names the one tuple *this*
/// build produced, so a manifest composed here carries the right words. A
/// selection is made against the machine's reading and never against this.
///
/// *(Corrected 2026-09-07. This said the clause had no supplier, that nothing
/// in the workspace read a processor architecture or a C library generation off
/// a machine, and that `Server Setup` owed it. All three were true until that
/// morning.)*
pub const THE_TUPLE_THIS_BUILD_IS_MADE_FOR: (&str, &str, &str) = ("linux", "x86_64", "gnu");

/// The tuple above, as a value.
pub fn the_tuple_this_build_is_made_for() -> TargetTuple {
    let (family, architecture, abi) = THE_TUPLE_THIS_BUILD_IS_MADE_FOR;
    TargetTuple {
        operating_system_family: family.into(),
        processor_architecture: architecture.into(),
        abi_and_capabilities: abi.into(),
    }
}

/// **The versioned-symbol mark a program built against the GNU C library
/// carries, and the one this check looks for.**
///
/// A program linked against that library records, for every symbol it imports,
/// the release of the library that introduced it — written into the file as the
/// text `GLIBC_2.39` and its like. The loader on the machine reads those and
/// refuses the program where its own library is older. **So the text being
/// present at all is the whole fault**: it means the artifact carries a floor,
/// and the manifest has nowhere to write one.
const THE_MARK_A_BUILD_AGAINST_THE_SYSTEMS_C_LIBRARY_LEAVES: &[u8] = b"GLIBC_";

/// **B8c: the machine program's artifact must need no C library from the
/// machine, and this is the check that says whether it does.**
///
/// **Why this is a rule and not a preference.** The manifest's third tuple field
/// names a C library and carries no release of it, and B8 forbids deriving one
/// — no closest-ABI, no capability fallback, no generation arithmetic. So a
/// manifest cannot say *this artifact wants release 2.39 or newer*, and a
/// desktop cannot select on it. `Server Setup` M1a names **Ubuntu and Debian**
/// and names no version of either, so the promise the artifact has to keep is
/// *every* Ubuntu and *every* Debian — which an artifact built against a
/// release of that library cannot keep, whatever release it is built against.
/// **The only artifact that keeps it is one that needs the library not at all.**
///
/// **What it cost to find this out**: release 1638's artifact was built against
/// release 2.39, Ubuntu 22.04 carries 2.35, and the placed program died five
/// times in one second on a real rented machine — `Audits/Build/Step 10c/Piece
/// 12, the fifth pass — Sesterce.md`, 2026-09-23.
///
/// **What this check is worth, stated rather than overstated.** It reads the
/// artifact's own bytes for the mark above. It catches the fault that actually
/// happened — a build made on a runner whose library is newer than the machine's
/// — and it catches it at the desk, before a release is composed, rather than on
/// a rental. **It is not a proof that the artifact runs**: it says nothing about
/// the kernel the program needs, nothing about a library some future dependency
/// might open by name at run time, and nothing about the desktop application's
/// own artifacts, which are not in this manifest at all (see
/// [`compose_the_manifest`]).
pub fn the_artifact_needs_no_c_library_from_the_machine(
    bytes: &[u8],
) -> Result<(), TheCLibraryReleasesTheArtifactAsksFor> {
    let mark = THE_MARK_A_BUILD_AGAINST_THE_SYSTEMS_C_LIBRARY_LEAVES;
    let mut asked_for: Vec<String> = Vec::new();
    let mut at = 0;
    while at + mark.len() <= bytes.len() {
        if &bytes[at..at + mark.len()] != mark {
            at += 1;
            continue;
        }
        // **The release is read only to say what the artifact asks for**, so a
        // person meeting the refusal is told the number rather than the rule
        // alone. Nothing is decided from it.
        let mut end = at + mark.len();
        while end < bytes.len() && (bytes[end].is_ascii_digit() || bytes[end] == b'.') {
            end += 1;
        }
        let named = String::from_utf8_lossy(&bytes[at..end]).into_owned();
        if !asked_for.contains(&named) {
            asked_for.push(named);
        }
        at = end.max(at + 1);
    }
    if asked_for.is_empty() {
        return Ok(());
    }
    asked_for.sort();
    Err(TheCLibraryReleasesTheArtifactAsksFor { asked_for })
}

/// What [`the_artifact_needs_no_c_library_from_the_machine`] found, by name.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TheCLibraryReleasesTheArtifactAsksFor {
    /// Every distinct release the artifact names, in order, as the artifact
    /// spells it.
    pub asked_for: Vec<String>,
}

impl std::fmt::Display for TheCLibraryReleasesTheArtifactAsksFor {
    fn fmt(&self, into: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            into,
            "the artifact asks the machine's own C library for {}, so it runs \
             only where that library is at least that new — and the manifest \
             has nowhere to say so",
            self.asked_for.join(", ")
        )
    }
}

/// **Compose one artifact entry from the artifact's own bytes.**
///
/// **The byte length and the digest are measured rather than declared**, which
/// is the whole reason this is a function and not a literal: a manifest whose
/// length or digest was typed is a manifest that can disagree with the file it
/// names, and B10's verifications would then refuse a genuine release.
///
/// **The sources are the caller's and this build has none.** See
/// [`FetchesFromAnExactSourceTheManifestNamed`].
pub fn compose_an_artifact_entry(
    artifact_identity: &str,
    tuple: TargetTuple,
    bytes: &[u8],
    entry_point_name: &str,
    sources: &[String],
) -> ArtifactEntry {
    ArtifactEntry {
        artifact_identity: artifact_identity.into(),
        tuple,
        byte_length: bytes.len() as u64,
        digest: Digest::of(bytes).bytes(),
        entry_point_name: entry_point_name.into(),
        sources: sources.to_vec(),
    }
}

/// **Compose one release manifest.**
pub fn compose_a_manifest(
    release_sequence: u64,
    package_version: &str,
    protocol_version_current: u32,
    protocol_version_previous_supported: Option<u32>,
    artifacts: Vec<ArtifactEntry>,
) -> ReleaseManifest {
    ReleaseManifest {
        manifest_format: THE_MANIFEST_FORMAT.into(),
        release_sequence,
        package_version: package_version.into(),
        protocol_version_current,
        protocol_version_previous_supported,
        artifacts,
    }
}

// ------------------------------------------------------ where the bytes are

/// **Why a fetch did not produce the artifact's bytes.**
///
/// **Every one of these is an answer rather than a stop.** B10 has the desktop
/// refuse rather than proceed on any of them, and a refusal leaves the machine's
/// active pointer and boot registration exactly as they were.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WhyTheFetchFailed {
    /// The source is not one this fetcher can reach.
    ItCouldNotBeReached { said: String },
    /// **The source is not an exact HTTPS address.** B8 requires one and B10
    /// downloads from the exact source the verified manifest named, so anything
    /// else is refused rather than repaired into an address that might work.
    TheSourceIsNotAnExactHttpsAddress { source: String },
    /// **The source answered, and there is nothing there.** A released artifact
    /// that has been withdrawn, and the state of a repository nothing has ever
    /// been published to.
    TheSourceHasNoSuchThing,
    /// The source answered with something else.
    TheSourceAnsweredWith { status: u16 },
    /// **The source offered more bytes than the fetch was willing to hold.**
    /// For an artifact the ceiling is the byte length the verified manifest
    /// carries, so this is a source disagreeing with the manifest that named
    /// it.
    ItIsLargerThanTheCeiling { ceiling: u64 },
    /// **A partial answer did not start at the byte that was asked for**, so
    /// there is nothing to join what was already held onto.
    TheResumeDidNotLineUp { said: String },
}

/// **Where a verified manifest's artifact bytes come from.**
///
/// **B10 downloads only from an exact source named by the verified manifest**,
/// so the one argument is a source the manifest named, passed whole. A
/// fetcher that took a version, a name or a pattern and composed an address
/// would be selecting bytes by something other than the manifest, which is
/// what B8 exists to forbid.
///
/// **The implementation that reaches the internet is
/// [`super::fetching::AFetchOverHttps`], and what is still missing is one value
/// rather than a decision.** B8 requires an exact HTTPS source, and **B8a rules
/// who serves one** — a separate public repository the Creator owns, and nowhere
/// else — so the shape is settled and the address itself is a value that clause
/// leaves outside itself. `Desktop app and delivery` §19 gives Build the desktop
/// application's own release pipeline and says nothing about the machine's, so
/// this is not that pipeline and must not be built as one. **What removes the
/// absence:** the address.
///
/// *(Reworded 2026-09-06. It read "because there is nowhere to reach" and
/// "Smoozy has no hosting for the server package", which stopped being true the
/// day the plan gained B8a. The sibling note in [`super`] was brought into line
/// then and this one was not.)*
/// *(Reworded again 2026-09-07, when the fetcher landed. It read "There is no
/// implementation of this that reaches the internet", and there is one.)*
pub trait FetchesFromAnExactSourceTheManifestNamed {
    fn fetch(&mut self, source: &str) -> Result<Vec<u8>, WhyTheFetchFailed>;
}

/// **A fetch that reads a local file, standing in for hosting that does not
/// exist.**
///
/// **This is not a stub and it is not a fallback.** It is the honest separation
/// this piece was built under: the mechanism around the fetch — the signed
/// manifest, the trust root, the exact-source rule, the length and the digest —
/// is Build's and is built, and the place the bytes live is the Creator's and
/// is not. Everything above this can be proved end to end against a file on
/// this computer.
///
/// **It is never reachable by accident**, because it holds an explicit table of
/// which source stands for which local file and refuses anything else. Nothing
/// constructs one but a caller that wrote the table down.
///
/// **What replaces it:** a fetcher that fetches the source over HTTPS, once
/// there is a source to fetch.
pub struct AFetchFromALocalFileStandingInForHostingThatDoesNotExist {
    what_each_source_stands_for: Vec<(String, std::path::PathBuf)>,
    /// Every source it was actually asked for, in order — so a caller can hold
    /// the exact-source rule rather than assume it.
    pub asked_for: Vec<String>,
}

impl AFetchFromALocalFileStandingInForHostingThatDoesNotExist {
    pub fn where_each_source_stands_for_a_file(
        table: Vec<(String, std::path::PathBuf)>,
    ) -> AFetchFromALocalFileStandingInForHostingThatDoesNotExist {
        AFetchFromALocalFileStandingInForHostingThatDoesNotExist {
            what_each_source_stands_for: table,
            asked_for: Vec::new(),
        }
    }
}

impl FetchesFromAnExactSourceTheManifestNamed
    for AFetchFromALocalFileStandingInForHostingThatDoesNotExist
{
    fn fetch(&mut self, source: &str) -> Result<Vec<u8>, WhyTheFetchFailed> {
        self.asked_for.push(source.to_owned());
        let Some((_, path)) = self
            .what_each_source_stands_for
            .iter()
            .find(|(named, _)| named == source)
        else {
            return Err(WhyTheFetchFailed::ItCouldNotBeReached {
                said: format!("no local file stands for {source}"),
            });
        };
        std::fs::read(path).map_err(|failed| WhyTheFetchFailed::ItCouldNotBeReached {
            said: failed.to_string(),
        })
    }
}

// ------------------------------------------------------------- the test keys

/// **Key material for tests, and it cannot be anything else.**
///
/// **Every key here is derived from a seed the calling code writes out**, so it
/// is public the moment it is written and is not a secret at all. **There is no
/// function anywhere in this workspace that generates release key material**,
/// and that is deliberate: a generated key sitting in a repository looks
/// exactly like a real one to everybody who meets it afterwards.
///
/// **Half of what replaces this arrived on 2026-09-06**: the offline release
/// root, whose public half is [`super::trust::THE_RELEASE_ROOT_THIS_BUILD_CARRIES`]
/// and **whose secret half is in the Creator's password manager and in that
/// Mac's own login keychain**.
/// **`Preparation/Groundwork/Conventions.md` puts keys in a file outside this
/// project and this one is not kept that way, deliberately** — that folder is on
/// the Mac that builds Smoozy, and a release key in a plain file there defeats
/// what the key is for, **which is the reason the keychain was chosen over a
/// file**. It is read out of the keychain for the moment it signs and nothing in
/// this workspace keeps it.
///
/// **What is still owed:** the online manifest-signing key B9 has the root vouch
/// for, and a release step that reads whichever it needs.
/// `Rented-server runtime` K16's boot registration is the only other thing this
/// project composes by value for a machine, and it carries no key at all.
pub mod test_only_keys {
    /// A pair whose secret half is a seed somebody typed.
    #[derive(Debug, Clone, Copy)]
    pub struct AKeyPair {
        pub secret: [u8; 32],
        pub public: super::super::PublicKeyBytes,
    }

    /// **The seed is the secret half, and the caller writes it down.**
    ///
    /// Taking the seed as an argument rather than making one is the guard: a
    /// key from here is only ever as private as the literal in the calling
    /// file, which is to say not private at all.
    pub fn a_key_pair_from_a_written_down_seed(seed: [u8; 32]) -> AKeyPair {
        let signing = ed25519_dalek::SigningKey::from_bytes(&seed);
        AKeyPair {
            secret: seed,
            public: signing.verifying_key().to_bytes(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// **B8c, on bytes that carry the mark.** The releases are reported as the
    /// artifact spells them, and each one once however often it appears.
    #[test]
    fn an_artifact_that_asks_the_machine_for_a_c_library_release_is_named() {
        let mut bytes = b"\x7fELF and then some symbols: ".to_vec();
        bytes.extend_from_slice(b"GLIBC_2.39\0GLIBC_2.35\0GLIBC_2.39\0GLIBC_2.2.5\0");
        let found = the_artifact_needs_no_c_library_from_the_machine(&bytes)
            .expect_err("an artifact carrying the mark is refused");
        assert_eq!(
            found.asked_for,
            vec![
                "GLIBC_2.2.5".to_owned(),
                "GLIBC_2.35".to_owned(),
                "GLIBC_2.39".to_owned()
            ]
        );
        assert!(found.to_string().contains("GLIBC_2.39"));
    }

    /// **B8c, on bytes that do not.** An artifact that needs nothing of the
    /// machine's C library carries the mark nowhere.
    #[test]
    fn an_artifact_that_needs_nothing_of_the_machines_c_library_passes() {
        let bytes = b"\x7fELF, statically linked, and no versioned import at all".to_vec();
        assert_eq!(
            the_artifact_needs_no_c_library_from_the_machine(&bytes),
            Ok(())
        );
        assert_eq!(
            the_artifact_needs_no_c_library_from_the_machine(&[]),
            Ok(())
        );
    }

    /// **The mark cut off by the end of the bytes is still the mark.** A scan
    /// that only matched a whole release string would let a truncated one
    /// through.
    #[test]
    fn the_mark_at_the_very_end_is_still_found() {
        assert!(the_artifact_needs_no_c_library_from_the_machine(b"padding GLIBC_").is_err());
        assert!(the_artifact_needs_no_c_library_from_the_machine(b"padding GLIB").is_ok());
    }

    #[test]
    fn the_entry_points_two_copies_agree() {
        // **The one place the name is defined is `Rented-server runtime`'s own
        // program.** This crate cannot name that one, so the guard is a
        // reading of its source rather than a comparison of two constants.
        let program = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../Rented-server runtime/src/the_program.rs");
        let text = std::fs::read_to_string(&program)
            .expect("the machine program's own file is readable");
        let expected = format!("\"{THE_FIXED_ENTRY_POINT_NAME}\"");
        assert!(
            text.contains(&expected),
            "the machine program no longer names {expected}, so this crate's copy has drifted"
        );
    }

    #[test]
    fn an_artifact_entry_measures_the_bytes_rather_than_taking_them_on_trust() {
        let bytes = b"a program, for the sake of argument".to_vec();
        let entry = compose_an_artifact_entry(
            "linux-x86_64-gnu",
            the_tuple_this_build_is_made_for(),
            &bytes,
            THE_FIXED_ENTRY_POINT_NAME,
            &["https://example.invalid/smoozy-server".into()],
        );
        assert_eq!(entry.byte_length, bytes.len() as u64);
        assert_eq!(entry.digest, Digest::of(&bytes).bytes());
        assert_eq!(entry.entry_point_name, THE_FIXED_ENTRY_POINT_NAME);
        // One byte different is a different artifact.
        let mut other = bytes.clone();
        other[0] ^= 1;
        let second = compose_an_artifact_entry(
            "linux-x86_64-gnu",
            the_tuple_this_build_is_made_for(),
            &other,
            THE_FIXED_ENTRY_POINT_NAME,
            &[],
        );
        assert_ne!(entry.digest, second.digest);
    }

    #[test]
    fn the_local_file_fetch_answers_only_the_sources_it_was_given() {
        let directory = std::env::temp_dir().join(format!(
            "smoozy-release-fetch-{}",
            std::process::id()
        ));
        std::fs::create_dir_all(&directory).unwrap();
        let file = directory.join("artifact");
        std::fs::write(&file, b"the bytes").unwrap();

        let mut fetch =
            AFetchFromALocalFileStandingInForHostingThatDoesNotExist::where_each_source_stands_for_a_file(
                vec![("https://example.invalid/a".into(), file.clone())],
            );
        assert_eq!(fetch.fetch("https://example.invalid/a").unwrap(), b"the bytes");
        assert!(matches!(
            fetch.fetch("https://example.invalid/b"),
            Err(WhyTheFetchFailed::ItCouldNotBeReached { .. })
        ));
        assert_eq!(fetch.asked_for.len(), 2);

        std::fs::remove_dir_all(&directory).unwrap();
    }
}
