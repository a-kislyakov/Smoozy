//! The container test and the protection reading — `Keys` U2 and U8.
//!
//! **Both are the same in-place header read, on the same bytes, transiently**
//! — the read `Security and licence` §6 owns. Nothing here decrypts anything
//! and nothing here holds key material: what is looked at is the armour line,
//! and for one of the three armours the two names that sit at the front of the
//! container.
//!
//! **U2: the container test is the discriminator, and it is a filename-free
//! rule because no closed filename list is possible** (Fact 34 — a key
//! location holds `config`, `known_hosts`, `authorized_keys`, `environment`,
//! `rc`, `-cert.pub` certificates, `Include` fragments and an `agent/`
//! directory of sockets). **The fingerprint read is never the
//! discriminator**: observed, `ssh-keygen -l -f` fingerprints an
//! `authorized_keys` file at exit 0 exactly as it fingerprints `known_hosts`.

/// Which armour the file opens with. **Everything else in a key location fails
/// the test and is not listed** — `known_hosts`, `authorized_keys`, `.pub` and
/// `-cert.pub` files, configuration, sockets, directories.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Armour {
    /// `-----BEGIN OPENSSH PRIVATE KEY-----` — the `openssh-key-v1`
    /// container, which is everything Smoozy generates and nearly everything a
    /// user has.
    OpenSshV1,
    /// A legacy PEM private-key armour. **Line 1 decides nothing about
    /// protection here** — both PEM keys carry the identical
    /// `-----BEGIN RSA PRIVATE KEY-----`.
    LegacyPem,
    /// A PKCS8 armour, which RFC 7468 §10 and §11 fix: unencrypted key
    /// structures *"are encoded using the \"PRIVATE KEY\" label"* and
    /// encrypted ones *"are encoded using the \"ENCRYPTED PRIVATE KEY\"
    /// label"*.
    Pkcs8 { encrypted_label: bool },
}

/// What the header read came to.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Container {
    pub armour: Armour,
    /// **U8: *is this key protected* is decided from the container**, and the
    /// `-P ''` probe confirms, never decides.
    ///
    /// **Absent where the body could not be made sense of, and that is U2 kept
    /// rather than a softening.** The discriminator for *this is a private key*
    /// is the armour line and nothing else, so a truncated or damaged
    /// `openssh-key-v1` file — a copy that failed part-way — is a key whose
    /// protection reading is missing, never a file that is not a key. Turning
    /// the missing reading into *not a key* is what kept such a file out of the
    /// list entirely, where U1 and U3 want everything found in the usual key
    /// location, usable or not: it fails U4 clause 4a instead and is listed as
    /// unusable with the reason.
    pub protected: Option<bool>,
}

const OPENSSH_ARMOUR: &str = "-----BEGIN OPENSSH PRIVATE KEY-----";
const PKCS8_PLAIN_ARMOUR: &str = "-----BEGIN PRIVATE KEY-----";
const PKCS8_ENCRYPTED_ARMOUR: &str = "-----BEGIN ENCRYPTED PRIVATE KEY-----";

/// The legacy PEM armours. **RSA, DSA and EC are the three OpenSSH's own
/// `-m PEM` writes**, and the list is a list of armour lines rather than of key
/// types — a file whose armour is here is a private key container, whatever
/// this binary can go on to do with it.
const LEGACY_PEM_ARMOURS: [&str; 3] = [
    "-----BEGIN RSA PRIVATE KEY-----",
    "-----BEGIN DSA PRIVATE KEY-----",
    "-----BEGIN EC PRIVATE KEY-----",
];

/// **RFC 1421 §4.6.1.1 and §4.6.1.3**: the `Proc-Type:` field *"required for
/// all PEM messages, identifies the type of processing performed"*, and *"the
/// field is required for all messages specified as \"ENCRYPTED\""*. Observed
/// on this Mac: an encrypted PEM key carries it on line 2, under a first line
/// identical to the plain PEM key's.
const PEM_ENCRYPTED_MARK: &str = "Proc-Type:";

/// U2 and U8 in one reading over the file's opening bytes.
///
/// **The read is ordered: the armour label on line 1 first, and where that
/// label is a PEM armour, line 2 as well.** A file that opens with no
/// private-key armour is not a key and answers `None` — which is the whole of
/// what the listing rule needs.
pub fn read(opening_bytes: &[u8]) -> Option<Container> {
    // The header is text; anything that is not is not an armour line. A file
    // whose first bytes are not valid text is a file that fails the test,
    // which is the same answer as any other non-key.
    let text = String::from_utf8_lossy(opening_bytes);
    let mut lines = text.lines();
    let first = lines.next()?.trim_end_matches('\r').trim();

    if first == OPENSSH_ARMOUR {
        // **The armour is the discriminator and the protection reading is not.**
        // A body this function cannot make sense of leaves the protection
        // answer absent; it never turns the file into something that is not a
        // key.
        return Some(Container {
            armour: Armour::OpenSshV1,
            protected: openssh_container_is_protected(&text),
        });
    }
    if first == PKCS8_PLAIN_ARMOUR || first == PKCS8_ENCRYPTED_ARMOUR {
        let encrypted_label = first == PKCS8_ENCRYPTED_ARMOUR;
        return Some(Container {
            armour: Armour::Pkcs8 { encrypted_label },
            protected: Some(encrypted_label),
        });
    }
    if LEGACY_PEM_ARMOURS.contains(&first) {
        let second = lines.next().unwrap_or_default().trim();
        return Some(Container {
            armour: Armour::LegacyPem,
            protected: Some(second.starts_with(PEM_ENCRYPTED_MARK)),
        });
    }
    None
}

/// **For the `openssh-key-v1` container: cipher and KDF names `none`/`none` is
/// unprotected, anything else is protected** (`Security and licence` §6's
/// rule, carried with its three narrowings intact — one build; four keys, one
/// encrypted; the container layout documented on no manual page).
///
/// The two names sit at the front of the container's own body, each a
/// length-prefixed string after the `openssh-key-v1\0` magic. **Nothing beyond
/// them is read** — no key material is decoded, and a body this function
/// cannot make sense of answers `None` rather than guessing a protection
/// state.
fn openssh_container_is_protected(text: &str) -> Option<bool> {
    let body: String = text
        .lines()
        .skip(1)
        .take_while(|line| !line.starts_with("-----END"))
        .collect();
    let bytes = decode_base64(&body)?;
    let rest = bytes.strip_prefix(b"openssh-key-v1\0")?;
    let (cipher, rest) = length_prefixed_string(rest)?;
    let (kdf, _) = length_prefixed_string(rest)?;
    Some(!(cipher == "none" && kdf == "none"))
}

/// One `uint32` length followed by that many bytes, read as text — the shape
/// RFC 4253 §6.6 fixes for the wire format and the one the container's own
/// header uses.
fn length_prefixed_string(bytes: &[u8]) -> Option<(String, &[u8])> {
    let (length, rest) = bytes.split_first_chunk::<4>()?;
    let length = u32::from_be_bytes(*length) as usize;
    if rest.len() < length {
        return None;
    }
    let (value, rest) = rest.split_at(length);
    Some((String::from_utf8(value.to_vec()).ok()?, rest))
}

/// The base64 alphabet RFC 4648 §4 fixes, which is what the armour body is
/// written in.
///
/// **This is a decoder and not a dependency**, because the whole of what is
/// wanted is the first few dozen bytes of a container header; padding is
/// accepted and whitespace is already stripped by the caller.
fn decode_base64(body: &str) -> Option<Vec<u8>> {
    const ALPHABET: &[u8; 64] =
        b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut out = Vec::new();
    let mut accumulator: u32 = 0;
    let mut bits = 0;
    for character in body.bytes() {
        if character == b'=' {
            break;
        }
        if character.is_ascii_whitespace() {
            continue;
        }
        let value = ALPHABET.iter().position(|&letter| letter == character)? as u32;
        accumulator = (accumulator << 6) | value;
        bits += 6;
        if bits >= 8 {
            bits -= 8;
            out.push((accumulator >> bits) as u8);
        }
    }
    Some(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// An `openssh-key-v1` header with the two names given, base64-encoded the
    /// way the armour carries it.
    fn openssh_key(cipher: &str, kdf: &str) -> String {
        let mut body = Vec::new();
        body.extend_from_slice(b"openssh-key-v1\0");
        for name in [cipher, kdf] {
            body.extend_from_slice(&(name.len() as u32).to_be_bytes());
            body.extend_from_slice(name.as_bytes());
        }
        format!(
            "{OPENSSH_ARMOUR}\n{}\n-----END OPENSSH PRIVATE KEY-----\n",
            encode_base64(&body)
        )
    }

    fn encode_base64(bytes: &[u8]) -> String {
        const ALPHABET: &[u8; 64] =
            b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
        let mut out = String::new();
        for chunk in bytes.chunks(3) {
            let mut block = [0u8; 3];
            block[..chunk.len()].copy_from_slice(chunk);
            let value = u32::from_be_bytes([0, block[0], block[1], block[2]]);
            let letters = [
                (value >> 18) & 63,
                (value >> 12) & 63,
                (value >> 6) & 63,
                value & 63,
            ];
            for (index, letter) in letters.iter().enumerate() {
                if index <= chunk.len() {
                    out.push(ALPHABET[*letter as usize] as char);
                } else {
                    out.push('=');
                }
            }
        }
        out
    }

    #[test]
    fn nothing_else_in_a_key_location_passes_the_container_test() {
        assert_eq!(read(b"ssh-ed25519 AAAAC3Nza... someone@mac\n"), None);
        assert_eq!(read(b"host github.com\n  User git\n"), None);
        assert_eq!(read(b"|1|abc=|def= ssh-ed25519 AAAA\n"), None);
        assert_eq!(read(b""), None);
        assert_eq!(read(&[0xff, 0xfe, 0x00, 0x01]), None);
    }

    #[test]
    fn an_openssh_container_with_no_cipher_and_no_kdf_is_unprotected() {
        let container = read(openssh_key("none", "none").as_bytes()).expect("a container");
        assert_eq!(container.armour, Armour::OpenSshV1);
        assert_eq!(container.protected, Some(false));
    }

    #[test]
    fn an_openssh_container_naming_a_cipher_is_protected() {
        let container =
            read(openssh_key("aes256-ctr", "bcrypt").as_bytes()).expect("a container");
        assert_eq!(container.protected, Some(true));
    }

    #[test]
    fn a_pem_key_is_decided_on_line_two_because_line_one_decides_nothing() {
        let plain = "-----BEGIN RSA PRIVATE KEY-----\nMIIEow...\n-----END RSA PRIVATE KEY-----\n";
        let encrypted = "-----BEGIN RSA PRIVATE KEY-----\nProc-Type: 4,ENCRYPTED\nDEK-Info: AES-128-CBC,0123\n\nMIIEow...\n";
        assert_eq!(
            read(plain.as_bytes()),
            Some(Container {
                armour: Armour::LegacyPem,
                protected: Some(false)
            })
        );
        assert_eq!(
            read(encrypted.as_bytes()),
            Some(Container {
                armour: Armour::LegacyPem,
                protected: Some(true)
            })
        );
    }

    #[test]
    fn a_pkcs8_key_is_decided_on_the_armour_label_on_line_one() {
        assert_eq!(
            read(b"-----BEGIN PRIVATE KEY-----\nMIGT...\n"),
            Some(Container {
                armour: Armour::Pkcs8 {
                    encrypted_label: false
                },
                protected: Some(false)
            })
        );
        assert_eq!(
            read(b"-----BEGIN ENCRYPTED PRIVATE KEY-----\nMIGT...\n"),
            Some(Container {
                armour: Armour::Pkcs8 {
                    encrypted_label: true
                },
                protected: Some(true)
            })
        );
    }

    /// **A damaged key in the usual key location is still a key.** U2 makes the
    /// armour line the discriminator and nothing else, so a body this reader
    /// cannot make sense of — a copy that failed part-way — answers with the
    /// armour and **no protection reading**, rather than with *this is not a
    /// key*. Turning the missing reading into an absent file is what kept such
    /// a file out of the list entirely, where U1 and U3 want everything found
    /// there, usable or not.
    #[test]
    fn a_damaged_body_leaves_the_protection_reading_absent_and_the_file_a_key() {
        let broken = format!("{OPENSSH_ARMOUR}\n!!!!not base64!!!!\n");
        assert_eq!(
            read(broken.as_bytes()),
            Some(Container {
                armour: Armour::OpenSshV1,
                protected: None
            })
        );

        // Truncated part-way through the header's own two names, which is what
        // a copy that stopped actually leaves.
        let mut body = Vec::new();
        body.extend_from_slice(b"openssh-key-v1\0");
        body.extend_from_slice(&(10u32).to_be_bytes());
        let cut = format!(
            "{OPENSSH_ARMOUR}\n{}\n",
            encode_base64(&body)
        );
        assert_eq!(
            read(cut.as_bytes()),
            Some(Container {
                armour: Armour::OpenSshV1,
                protected: None
            })
        );

        // And a file with no private-key armour at all is still not a key.
        assert_eq!(read(b"ssh-ed25519 AAAA someone@mac\n"), None);
    }
}
