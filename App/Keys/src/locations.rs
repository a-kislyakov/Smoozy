//! The key locations, and what sits beside a private key — `Keys` §3 and W5.
//!
//! **L1a: the location's path is a reading of the system and never a path this
//! module composes.** The strings below name a *directory under the home
//! directory*; what resolves the home part is `Desktop app and delivery`'s
//! platform adapter, asked for its **user home directory** reading (`Desktop
//! app and delivery` L3b), and this module never assembles it from a login
//! account name (`Desktop app and delivery` I4) and never from an environment
//! value.
//!
//! **L3: the one-place premise is false and the inherited rule is kept
//! whole.** `~/.ssh` is the default and never the only place. So every row
//! records the path of the file that backs it, recorded at the moment the key
//! entered, and **no path is ever synthesised from a name, a comment or an
//! identity**. The one thing this file composes is a *new* name before a
//! generation (`crate::naming`); it never reconstructs an existing key's path.
//!
//! **L5: a key with no file never enters Keys.** Agent-only and token-supplied
//! keys have no private key file to point at. They are not listed, not
//! addable, and no promise here reaches them.

use std::path::{Path, PathBuf};

/// **L1: the usual key location, per system, is the per-user default directory
/// OpenSSH's own manuals document, and it is one directory per system.**
///
/// - **macOS:** `~/.ssh` (documented; Fact 33).
/// - **Linux:** `~/.ssh` (documented; Fact 33).
/// - **Windows:** `%USERPROFILE%\.ssh` (documented, and documentation alone —
///   Fact 33's Windows half; no Windows machine has run it).
///
/// The three are one name under the home directory on all three systems, so
/// this is one constant and not three that could drift.
pub const UNDER_THE_HOME_DIRECTORY: &str = ".ssh";

/// The usual key location, given the home directory the adapter answered with.
///
/// **The home directory is a parameter and not a reading taken here**, which is
/// L1a kept by construction: this module has no route to the system's own call
/// and cannot acquire one by accident.
pub fn usual_key_location(user_home: &Path) -> PathBuf {
    user_home.join(UNDER_THE_HOME_DIRECTORY)
}

/// **L2: `C:\ProgramData\ssh` is not a usual key location and nothing in it is
/// ever managed.**
///
/// Microsoft's own page names it for **host keys and server configuration**,
/// and its `HostKey` defaults name private key files. It is machine-wide
/// rather than the user's, and what it holds is the machine's own host keys —
/// **treating it as managed would put delete's destroying branch under private
/// key files that are not the user's identities at all.**
///
/// A key file there enters Smoozy only by pointing, as a borrowed key, exactly
/// like any other path outside L1. This constant exists so the rule is a thing
/// a test can assert rather than a sentence in a comment.
pub const THE_MACHINE_WIDE_WINDOWS_DIRECTORY: &str = r"C:\ProgramData\ssh";

/// **W5: the companion rule is Fact 36's, exactly** — the public half is the
/// private key path plus `.pub`; the certificate is the private key path plus
/// `-cert.pub`, **the `.pub` dropped rather than doubled**.
///
/// **L4: the rule follows the private file wherever it sits.** The path it is
/// composed from is the row's recorded path, which L3 fixes as the canonical
/// path of the file that actually backs the key — so the companions are looked
/// for beside the real private half and never beside a symbolic link that
/// pointed at it, whose neighbours are not this key's files at all.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Companions {
    pub public_half: PathBuf,
    pub certificate: PathBuf,
}

/// The two companion names for a private key at `path`.
pub fn companions(path: &Path) -> Companions {
    let mut public_half = path.as_os_str().to_os_string();
    public_half.push(".pub");
    let mut certificate = path.as_os_str().to_os_string();
    certificate.push("-cert.pub");
    Companions {
        public_half: PathBuf::from(public_half),
        certificate: PathBuf::from(certificate),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_location_sits_under_whatever_home_the_adapter_answered_with() {
        assert_eq!(
            usual_key_location(Path::new("/Users/someone")),
            PathBuf::from("/Users/someone/.ssh")
        );
    }

    #[test]
    fn the_certificate_drops_the_pub_rather_than_doubling_it() {
        let beside = companions(Path::new("/Users/someone/.ssh/smoozy_ed25519"));
        assert_eq!(
            beside.public_half,
            PathBuf::from("/Users/someone/.ssh/smoozy_ed25519.pub")
        );
        assert_eq!(
            beside.certificate,
            PathBuf::from("/Users/someone/.ssh/smoozy_ed25519-cert.pub")
        );
    }

    #[test]
    fn the_companions_follow_the_private_file_wherever_it_sits() {
        let beside = companions(Path::new("/Volumes/stick/borrowed_key"));
        assert_eq!(
            beside.public_half,
            PathBuf::from("/Volumes/stick/borrowed_key.pub")
        );
    }
}
