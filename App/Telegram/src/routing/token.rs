//! **The action token a button carries — `Telegram` R1.**
//!
//! **R1: a button carries one compact opaque action token and no full routing
//! context.** Telegram publishes `callback_data` as 1 to 64 bytes
//! (`Preparation/Architecture/Sources/Telegram.sources.md` entry 38,
//! documented). Smoozy's token is the unpadded base64url encoding of exactly 33
//! bytes — one format byte, the 16-byte Telegram machine-routing key and the
//! 16-byte action-context identity — so it is exactly 44 ASCII bytes. **Nothing
//! else is inside it**: no user text, no label, no job or hold identity, no
//! action and no payload. That Telegram accepts a 44-byte token of this shape
//! is observed, by live sends on the real bot whose buttons were tapped —
//! entry 78, relabelled from an assumption on 2026-09-19.
//!
//! **Both identities are copied and compared only** (`State, identity and
//! recovery` §2.2 Y4); their bytes carry no meaning. Their written form is
//! lower-case hexadecimal, the same rendering `smoozy_state::identities` uses.
//!
//! **This is the first use of `base64` in the workspace.**

use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use base64::Engine as _;

/// **`Telegram` R1 — the one format byte this build writes.** A token carrying
/// any other is refused (R1: *unknown format*).
pub const THE_FORMAT: u8 = 1;

/// **`Telegram` R1 — how many bytes the token encodes, and how long it is.**
pub const THE_TOKENS_BYTES: usize = 33;
pub const THE_TOKENS_LENGTH: usize = 44;

/// The size of each of the two identities inside the token.
pub const AN_IDENTITYS_BYTES: usize = 16;

/// **What a token names**: the machine whose store holds the record, and the
/// record there.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WhatATokenNames {
    pub routing_key: [u8; AN_IDENTITYS_BYTES],
    pub action_context: [u8; AN_IDENTITYS_BYTES],
}

/// **Why a token was refused.** Every one of these is `Telegram` R3's case: the
/// press is answered and nothing is submitted.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhyATokenIsRefused {
    NotTheRightLength,
    NotBase64url,
    AnUnknownFormat,
}

impl WhatATokenNames {
    /// **`Telegram` R1 — the 44 ASCII bytes a button carries.**
    pub fn as_the_token(&self) -> String {
        let mut bytes = [0u8; THE_TOKENS_BYTES];
        bytes[0] = THE_FORMAT;
        bytes[1..17].copy_from_slice(&self.routing_key);
        bytes[17..].copy_from_slice(&self.action_context);
        URL_SAFE_NO_PAD.encode(bytes)
    }

    /// **Read a token back, failing closed** (R1: *unknown format* is rejected).
    pub fn read(token: &str) -> Result<WhatATokenNames, WhyATokenIsRefused> {
        if token.len() != THE_TOKENS_LENGTH {
            return Err(WhyATokenIsRefused::NotTheRightLength);
        }
        let bytes = URL_SAFE_NO_PAD
            .decode(token)
            .map_err(|_| WhyATokenIsRefused::NotBase64url)?;
        if bytes.len() != THE_TOKENS_BYTES {
            return Err(WhyATokenIsRefused::NotTheRightLength);
        }
        if bytes[0] != THE_FORMAT {
            return Err(WhyATokenIsRefused::AnUnknownFormat);
        }
        let mut routing_key = [0u8; AN_IDENTITYS_BYTES];
        let mut action_context = [0u8; AN_IDENTITYS_BYTES];
        routing_key.copy_from_slice(&bytes[1..17]);
        action_context.copy_from_slice(&bytes[17..]);
        Ok(WhatATokenNames { routing_key, action_context })
    }

    /// The routing key as the store writes it (`Telegram` B8: the addressee
    /// address of a tap's context-resolution instruction).
    pub fn the_routing_key(&self) -> String {
        as_hexadecimal(&self.routing_key)
    }

    /// The action-context identity as the store writes it.
    pub fn the_action_context(&self) -> String {
        as_hexadecimal(&self.action_context)
    }
}

/// Lower-case hexadecimal, a rendering and not a meaning.
pub fn as_hexadecimal(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

/// **One 16-byte identity read back from its written form**, or nothing where
/// the text is not exactly that.
pub fn from_hexadecimal(text: &str) -> Option<[u8; AN_IDENTITYS_BYTES]> {
    if text.len() != AN_IDENTITYS_BYTES * 2 || !text.bytes().all(|b| b.is_ascii_hexdigit()) {
        return None;
    }
    let mut bytes = [0u8; AN_IDENTITYS_BYTES];
    for (byte, pair) in bytes.iter_mut().zip(text.as_bytes().chunks(2)) {
        *byte = u8::from_str_radix(std::str::from_utf8(pair).ok()?, 16).ok()?;
    }
    Some(bytes)
}

/// **A fresh 16-byte action-context identity**, random and never reused
/// (`State, identity and recovery` §2.2's row for it). `smoozy_state`'s
/// identity kinds name no Telegram action-context identity, and that crate is
/// not this piece's to change, so it is drawn here the same way.
pub fn a_fresh_action_context() -> Option<[u8; AN_IDENTITYS_BYTES]> {
    let mut bytes = [0u8; AN_IDENTITYS_BYTES];
    getrandom::fill(&mut bytes).ok()?;
    Some(bytes)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn named() -> WhatATokenNames {
        WhatATokenNames { routing_key: [0xab; 16], action_context: [0x07; 16] }
    }

    /// **`Telegram` R1: 33 bytes, unpadded base64url, exactly 44 ASCII bytes,
    /// inside Telegram's 64.**
    #[test]
    fn a_token_is_44_ascii_bytes_of_url_safe_base64() {
        let token = named().as_the_token();
        assert_eq!(token.len(), 44);
        assert!(token.len() <= 64);
        assert!(token.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'-' || b == b'_'));
        assert!(!token.contains('='));
        assert_eq!(WhatATokenNames::read(&token), Ok(named()));
        assert_eq!(named().the_routing_key(), "ab".repeat(16));
        assert_eq!(from_hexadecimal(&named().the_action_context()), Some([0x07; 16]));
    }

    /// **`Telegram` R1 and R3: resolution fails closed.**
    #[test]
    fn anything_else_is_refused() {
        let token = named().as_the_token();
        assert_eq!(WhatATokenNames::read(&token[..43]), Err(WhyATokenIsRefused::NotTheRightLength));
        assert_eq!(WhatATokenNames::read(&format!("{}=", &token[..43])), Err(WhyATokenIsRefused::NotBase64url));
        let mut other = [0u8; 33];
        other[0] = 2;
        assert_eq!(
            WhatATokenNames::read(&URL_SAFE_NO_PAD.encode(other)),
            Err(WhyATokenIsRefused::AnUnknownFormat)
        );
        assert_eq!(WhatATokenNames::read(""), Err(WhyATokenIsRefused::NotTheRightLength));
        assert_eq!(from_hexadecimal("xyz"), None);
    }

    #[test]
    fn two_fresh_identities_differ() {
        assert_ne!(a_fresh_action_context(), a_fresh_action_context());
    }
}
