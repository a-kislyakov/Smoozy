//! **`Server Setup` B4's published checksum, read by the archive's own name —
//! the one part of the fetch that needs no client.**
//!
//! **Every test of the fetch itself moved on 2026-09-21** to the test module
//! `the_fetch_on_the_machines_own_line` in `src/the_setup_worker.rs`, beside
//! the program's own client.
//! **This file tested a copy of that client** — a test outside a program
//! cannot reach the program's own types — **and pointed the copy at a plain
//! `http://` server**, which never asks for a cryptography provider. So the one
//! fault that stopped every real fetch, a client built with no provider, was
//! the one fault it could not see, and it was found on the loopback against
//! the real program instead. **The tests there use the program's client itself
//! against an HTTPS server on this computer.**
//!
//! **What stays here is only what never touched the client.**
//!
//! **Markers used in this file that belong to another document, each with the
//! document it belongs to.** `Server Setup` B4.

use std::path::PathBuf;

use smoozy_server_setup_machine::acting::fetching::{
    the_archives_file_name, the_checksum_for, the_digest_of,
};
use smoozy_server_setup_machine::blender::Version;
use smoozy_state::identities::{Identity, IdentityKind};

fn a_version() -> Version {
    Version {
        major: 5,
        minor: 2,
        patch: 0,
    }
}

/// **The published checksum is read by the archive's own name**, out of a file
/// that holds an entry per artefact.
#[test]
fn the_published_checksum_is_read_by_the_archives_own_name() {
    // **A folder this call owns alone**, named by a fresh identity rather than
    // a clock reading, and removed when the test ends.
    let at = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join(format!(
        "the-fetch-checksum-{}",
        Identity::issue(IdentityKind::Operation)
            .expect("randomness")
            .as_text()
    ));
    std::fs::create_dir_all(&at).expect("the folder is made");
    std::fs::write(at.join("bytes"), b"some bytes").expect("the file is written");
    let digest = the_digest_of(&at.join("bytes")).expect("it reads");
    std::fs::remove_dir_all(&at).ok();

    let file = format!(
        "{digest}  {}\n0000000000000000000000000000000000000000000000000000000000000000  blender-5.2.0-windows-x64.zip\n",
        the_archives_file_name(a_version())
    );
    assert_eq!(
        the_checksum_for(&file, &the_archives_file_name(a_version())).as_deref(),
        Some(digest.as_str())
    );
}
