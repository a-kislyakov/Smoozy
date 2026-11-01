//! **A server program built at this desk, placed in place of the current
//! release** — `Server protocol and transfers` B10a, and a development build
//! only.
//!
//! **What it is.** A copy started with the throwaway setting and
//! `--desk-program=<file>` (`crate::platform::locations::throwaway`) has its
//! ready action read that file rather than fetch the current release. **It is
//! a second implementation of `crate::setting_up::ReadsTheCurrentRelease`, the
//! seam that exists so the whole order can run at a desk**, chosen at that
//! trait's one caller and nowhere else. **Everything after the fetch is
//! unchanged**: the upload, `Server protocol and transfers` B10's read-back at
//! rest against this file's own length and digest, the pointer, the boot
//! registration, the health exchange and *verified*. **This file places
//! nothing**; it only answers what the fetch would have answered.
//!
//! **What it does not do, each by construction rather than by care.**
//!
//! - **It reads no manifest and fetches nothing**, so `Server protocol and
//!   transfers` B8's selection from a signed manifest and B8b's address are
//!   not reached.
//! - **It holds no root and no key.** The program never left this computer
//!   before it is placed, so `Server protocol and transfers` B9's anchor has
//!   nothing to anchor; no second root exists anywhere.
//! - **It takes no store**, so `State, identity and recovery` J6's
//!   server-package trust record and `Server protocol and transfers` B9's
//!   floors are neither read nor written — there is no parameter through which
//!   either could be.
//!
//! **What it still refuses, before anything reaches the machine.** A program
//! that asks the machine for a release of its own C library, with `Server
//! protocol and transfers` B8c's own function; a program that is not built for
//! the one target this build makes (an x86-64 Linux program in the ELF form);
//! and a machine whose tuple is not that target's, through
//! `Server protocol and transfers` B8's own exact selection.
//!
//! **The whole file is compiled only with debug assertions** — the `mod` line
//! in `crate` carries the gate — **and no Cargo feature reaches it**. A release
//! build has no route here at all, which is B10a's *not a fallback that skips a
//! check* (`Desktop app and delivery` E15): there is nothing to fall back to.
//! The proof is three-fold and none of it is here: the test in
//! `crate::platform::locations` that asks in both builds, the release library's
//! bytes read for the argument's text, and the hosted workflow's check of each
//! release binary.
//!
//! **A pass on it proves nothing about a release** (B10a's last point), and
//! *Set up again* in a copy started without the argument places the real
//! release over whatever this placed — the reader's only caller is the ready
//! action, so that is the one way back and it is the right direction.
//!
//! **Markers used below that belong to another document, each with the
//! document it belongs to.** `Server protocol and transfers` B8, B8b, B8c, B9,
//! B10, B10a.

use smoozy_server_protocol::framing::DeclaredVersions;
use smoozy_server_protocol::release::composing::{
    compose_a_manifest, compose_an_artifact_entry,
    the_artifact_needs_no_c_library_from_the_machine, the_tuple_this_build_is_made_for,
    THE_FIXED_ENTRY_POINT_NAME,
};
use smoozy_server_protocol::release::manifest::{select, TargetTuple, WhySelectionFailed};
use smoozy_server_protocol::release::trust::TheTrustRecord;
use smoozy_server_protocol::release::verifying::AVerifiedRelease;
use smoozy_server_protocol::transfers::integrity::Digest;

use crate::setting_up::ReadsTheCurrentRelease;

/// **The release sequence a desk program is placed under: zero.** `Server
/// protocol and transfers` B9 calls a pair of zeroes *"exactly the state the
/// first release meets"*, and no published release carries it, so the
/// package-phase record on the machine says in its own values that what it
/// holds is not a release.
pub const THE_SEQUENCE_A_DESK_PROGRAM_IS_PLACED_UNDER: u64 = 0;

/// **Who signed it: nobody**, written where the real route writes the allowed
/// key that did. Nothing on the placement reads this value.
pub const SIGNED_BY_NOBODY: &str = "nobody: a program built at this desk (Server protocol and transfers B10a)";

/// The desk program, as the ready action reads it.
pub struct TheProgramBuiltAtThisDesk {
    pub file: std::path::PathBuf,
}

/// **The desk program this copy was given, where it was given one** — only
/// in a throwaway copy, which `crate::platform::locations::throwaway` asks
/// again rather than trusting the start.
pub fn the_one_this_copy_was_given() -> Option<TheProgramBuiltAtThisDesk> {
    crate::platform::locations::throwaway::the_desk_program_this_copy_was_given().map(|file| {
        TheProgramBuiltAtThisDesk {
            file: file.to_path_buf(),
        }
    })
}

impl ReadsTheCurrentRelease for TheProgramBuiltAtThisDesk {
    fn the_current_release(
        &mut self,
        detected: &TargetTuple,
    ) -> Result<(AVerifiedRelease, Vec<u8>), String> {
        let bytes = std::fs::read(&self.file).map_err(|why| {
            format!("the desk program {} could not be read: {why}", self.file.display())
        })?;
        the_desk_programs_release(&bytes, detected)
    }
}

/// **The desk program's bytes, checked and described as the fetch's answer
/// would describe a release.** Separate from the file so a test can hand it
/// bytes.
///
/// **The version it carries is this copy's own** — the workspace's, which is
/// also the program's. `Rented-server runtime` G27's health exchange requires
/// the program's answered package version to equal the one requested
/// (`smoozy_runtime::package::commit::the_health_exchange`), **so the desk
/// program and this copy must be built from the same tree**; a program from
/// another tree fails the exchange and the placement rolls back, which is the
/// placement's own answer and not one this file adds.
pub fn the_desk_programs_release(
    bytes: &[u8],
    detected: &TargetTuple,
) -> Result<(AVerifiedRelease, Vec<u8>), String> {
    // `Server protocol and transfers` B8c, by its own function.
    the_artifact_needs_no_c_library_from_the_machine(bytes)
        .map_err(|why| format!("the desk program was not placed: {why}"))?;
    it_is_built_for_the_target_this_build_makes(bytes)?;

    let declared = DeclaredVersions::of_this_build();
    let artifact = compose_an_artifact_entry(
        // **The identity the release tool gives the same artifact**, which is
        // the tuple it is for (`App/Release/src/making.rs`).
        "linux-x86_64-gnu",
        the_tuple_this_build_is_made_for(),
        bytes,
        THE_FIXED_ENTRY_POINT_NAME,
        // **No source**: nothing was fetched and nothing will be.
        &[],
    );
    let manifest = compose_a_manifest(
        THE_SEQUENCE_A_DESK_PROGRAM_IS_PLACED_UNDER,
        env!("CARGO_PKG_VERSION"),
        declared.current().0,
        declared.the_single_immediately_previous().map(|it| it.0),
        vec![artifact],
    );
    // **`Server protocol and transfers` B8's exact selection, by its own
    // function**, so an ARM machine or any other tuple is refused here exactly
    // as the real route refuses it.
    let selected = match select(&manifest, detected) {
        Ok(selected) => selected.clone(),
        Err(WhySelectionFailed::ThisMachineIsNotSupported) => {
            return Err(format!(
                "the desk program is built for {}-{}-{} and this machine is {}-{}-{}",
                manifest.artifacts[0].tuple.operating_system_family,
                manifest.artifacts[0].tuple.processor_architecture,
                manifest.artifacts[0].tuple.abi_and_capabilities,
                detected.operating_system_family,
                detected.processor_architecture,
                detected.abi_and_capabilities,
            ))
        }
        Err(other) => return Err(format!("the desk program was not placed: {other:?}")),
    };
    // **The digest of the composed, unsigned manifest** — what the package-phase
    // record keeps in the manifest digest's place.
    let manifest_digest = Digest::of(&manifest.canonical_bytes()).bytes();
    Ok((
        AVerifiedRelease {
            manifest,
            selected,
            manifest_digest,
            signed_by: SIGNED_BY_NOBODY.to_string(),
            // **Nothing on this route reads or writes it.** The type carries it
            // because the real admission hands it to the store before anything
            // is downloaded; here there is no store, and the value is the
            // empty record — no sequence, no generation, no statement.
            the_trust_record_to_keep: TheTrustRecord {
                greatest_accepted_release_sequence: 0,
                greatest_accepted_key_set_generation: 0,
                the_statement_accepted_at_that_generation: Vec::new(),
                its_root_signature: [0; 64],
            },
        },
        bytes.to_vec(),
    ))
}

/// **The desk program is an x86-64 Linux program in the ELF form, or it is not
/// placed.**
///
/// **Why a second check beside B8c's**: B8c reads for the C library's
/// versioned imports, and a program built for this Mac carries none, so it
/// would pass B8c and be placed on a machine that cannot run it. The desk's
/// own default build is exactly that program, so this is the mistake most
/// likely to be made.
///
/// **What is read, documented** (the System V ABI's *ELF Header* chapter,
/// `https://refspecs.linuxfoundation.org/elf/gabi4+/ch4.eheader.html`, read
/// 2026-09-27): the file opens with `e_ident`, whose first four bytes are
/// `ELFMAG0` to `ELFMAG3` — `0x7f`, `'E'`, `'L'`, `'F'`; `EI_CLASS` is index 4
/// and `ELFCLASS64` is 2; `EI_DATA` is index 5 and `ELFDATA2LSB` is 1; and
/// `e_machine` follows `e_ident` (16 bytes) and the two-byte `e_type`, so it is
/// the two bytes at 18, in the file's own byte order, with `EM_X86_64` being 62.
///
/// **What it is worth**: it tells a program for another system or another
/// processor from the right one. It does not run the program, and it says
/// nothing of the kernel it needs — B8c's own limits, unchanged.
pub fn it_is_built_for_the_target_this_build_makes(bytes: &[u8]) -> Result<(), String> {
    const THE_MAGIC: &[u8; 4] = b"\x7fELF";
    const ELFCLASS64: u8 = 2;
    const ELFDATA2LSB: u8 = 1;
    const EM_X86_64: u16 = 62;
    let refused = |what: &str| {
        Err(format!(
            "the desk program was not placed: it is {what}, and the machine's program \
             is built for Linux on x86-64 (the target `x86_64-unknown-linux-musl`)"
        ))
    };
    if bytes.len() < 20 || &bytes[0..4] != THE_MAGIC {
        return refused("not a Linux program");
    }
    if bytes[4] != ELFCLASS64 || bytes[5] != ELFDATA2LSB {
        return refused("not a 64-bit little-endian program");
    }
    if u16::from_le_bytes([bytes[18], bytes[19]]) != EM_X86_64 {
        return refused("built for another processor");
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The first twenty bytes of an ELF file, as the header chapter lays them
    /// out, with the processor given.
    fn an_elf_header(machine: u16) -> Vec<u8> {
        let mut bytes = vec![0x7f, b'E', b'L', b'F', 2, 1, 1, 0];
        bytes.resize(16, 0);
        bytes.extend_from_slice(&3u16.to_le_bytes());
        bytes.extend_from_slice(&machine.to_le_bytes());
        bytes.extend_from_slice(b"...the rest of a program...");
        bytes
    }

    fn the_machine() -> TargetTuple {
        the_tuple_this_build_is_made_for()
    }

    #[test]
    fn a_static_x86_64_program_is_described_as_the_fetch_would_describe_it() {
        let bytes = an_elf_header(62);
        let (verified, handed) =
            the_desk_programs_release(&bytes, &the_machine()).expect("it is placed");
        assert_eq!(handed, bytes);
        let values = verified.the_release_descriptors_values();
        // B10's read-back compares these two against the file's own bytes.
        assert_eq!(values.artifact_byte_length, bytes.len() as u64);
        assert_eq!(values.artifact_digest, Digest::of(&bytes).bytes());
        // The health exchange compares this against the program's own version.
        assert_eq!(values.requested_package_version, env!("CARGO_PKG_VERSION"));
        // Not a release, by its own values.
        assert_eq!(values.verified_release_sequence, 0);
        assert_eq!(
            values.manifest_digest,
            Digest::of(&verified.manifest.canonical_bytes()).bytes()
        );
        assert_eq!(verified.selected.entry_point_name, THE_FIXED_ENTRY_POINT_NAME);
        assert!(verified.selected.sources.is_empty());
    }

    #[test]
    fn a_program_that_asks_the_machine_for_its_c_library_is_refused() {
        let mut bytes = an_elf_header(62);
        bytes.extend_from_slice(b"\0GLIBC_2.39\0");
        let refused = the_desk_programs_release(&bytes, &the_machine()).unwrap_err();
        assert!(refused.contains("GLIBC_2.39"), "{refused}");
    }

    #[test]
    fn a_program_for_another_system_or_processor_is_refused() {
        // A program for this Mac: a Mach-O file, which carries no GLIBC_ text
        // and would pass B8c alone.
        let mach_o = [0xcf, 0xfa, 0xed, 0xfe, 0x0c, 0, 0, 0x01, 0, 0, 0, 0];
        assert!(the_desk_programs_release(&mach_o, &the_machine()).is_err());
        // An ARM Linux program (EM_AARCH64 is 183).
        assert!(the_desk_programs_release(&an_elf_header(183), &the_machine()).is_err());
        // A 32-bit one.
        let mut thirty_two = an_elf_header(62);
        thirty_two[4] = 1;
        assert!(the_desk_programs_release(&thirty_two, &the_machine()).is_err());
        assert!(the_desk_programs_release(b"", &the_machine()).is_err());
    }

    #[test]
    fn a_machine_of_another_tuple_is_refused_by_the_exact_selection() {
        let arm = TargetTuple {
            operating_system_family: "linux".into(),
            processor_architecture: "aarch64".into(),
            abi_and_capabilities: "gnu".into(),
        };
        let refused = the_desk_programs_release(&an_elf_header(62), &arm).unwrap_err();
        assert!(refused.contains("aarch64"), "{refused}");
    }

    #[test]
    fn a_file_that_is_not_there_is_refused_in_words() {
        let mut reader = TheProgramBuiltAtThisDesk {
            file: std::env::temp_dir().join("smoozy-no-such-desk-program"),
        };
        let refused = reader.the_current_release(&the_machine()).unwrap_err();
        assert!(refused.contains("could not be read"), "{refused}");
    }
}
