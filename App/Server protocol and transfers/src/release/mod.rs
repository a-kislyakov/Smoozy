//! §5's B8, B9 and B10 — the signed release manifest a server package is
//! selected from, the offline root that trust is anchored in, and verification
//! before execution at both ends.
//!
//! **What this is for.** Steps 0 to 8 of `Preparation/Build/Plan.md` built
//! twenty modules' worth of behaviour and nothing that could be placed on a
//! rented machine. The program is `Rented-server runtime`'s `smoozy-server`;
//! **what is here is the package that carries it and the evidence that says
//! the bytes are Smoozy's.**
//!
//! **Two things in this piece are not Build's, and one of the two is closed.**
//!
//! 1. **Somewhere to fetch the artifact from.** B8 requires every artifact
//!    entry to name its exact HTTPS source or sources. **B8a rules who serves
//!    them** — a separate public repository the Creator owns — **and the address
//!    itself is a value that document leaves outside itself**, so the shape is
//!    settled and one value is owed. Fetching is a trait —
//!    [`composing::FetchesFromAnExactSourceTheManifestNamed`] — and **there are
//!    now two implementations of it**: [`fetching::AFetchOverHttps`], which
//!    speaks to an exact HTTPS source over the internet and is what a real
//!    release is fetched with, and the local-file one, which stands in for the
//!    address while there is none.
//!    *(Corrected 2026-09-07. This said the one implementation in this tree
//!    read a local file, which stopped being true when the fetcher landed. **The
//!    absence it recorded has not gone**: what is owed is still the address, and
//!    it is still one value the Creator holds.)*
//! 2. **A signing key. Closed on 2026-09-06, and only for the root.** The
//!    offline release root exists: the pair was generated once, its public half
//!    is [`trust::THE_RELEASE_ROOT_THIS_BUILD_CARRIES`], and its secret half is
//!    in the Creator's password manager and in that Mac's own login keychain,
//!    and nowhere else — never in this
//!    repository, on a build runner, or in a log. [`a_signature_over`] is still
//!    the whole of the signing mechanism and it still takes the secret bytes
//!    from its caller, **which is what makes *supplied for the moment it signs*
//!    buildable at all**. **The only producer of secret bytes anywhere in this
//!    workspace is [`composing::test_only_keys`], which derives them from a seed
//!    the caller writes down** — so a key made there is public by construction
//!    and cannot be mistaken for a release key. **What is still owed is the
//!    online manifest-signing key B9 has the root vouch for, and the release
//!    step that composes and signs a real release.**
//!
//! **Everything else is built and tested end to end**: the canonical bytes,
//! the digest, the byte length, the root-signed key-set statement, the
//! non-rollback memory, the exact-tuple selection, and the three verifications
//! B10 orders.
//!
//! **The distributed form is the signed form, and that is the one decision
//! worth meeting first.** A manifest is distributed as the canonical bytes
//! themselves; nothing re-encodes one before checking a signature over it, and
//! the parsed value is derived from the bytes rather than the bytes from the
//! value. **Canonicalisation bugs are the ordinary way a signed document is
//! forged**, and there is no canonicalisation step here to get wrong.
//!
//! **Markers used below that belong to another document, each with the
//! document it belongs to.** `State, identity and recovery` J10.
//!
//! **How a clause marker is written in this crate, and it is a rule rather
//! than a habit.** A marker with no document beside it means a clause of this
//! crate's own plan, `Preparation/Architecture/Server protocol and
//! transfers.md`. Every other document's marker is written with that
//! document — at the site, or in the file's own header list above. **Where a
//! marker is a clause of this crate's own plan *and* of the document meant,
//! the header list is not enough and the document is written out at the
//! site**, because a bare one there does not fail to resolve: it resolves to a
//! real and wrong clause, which nobody reading it can tell from a right one.

pub mod composing;
pub mod fetching;
pub mod manifest;
pub mod trust;
pub mod verifying;

/// **The context every manifest signature covers, before the canonical
/// bytes.**
///
/// A signature is taken over this run followed by the document's own canonical
/// bytes, and the two documents in this module carry different runs. **Without
/// it a root-signed key-set statement and a manifest are both just a run of
/// bytes with a signature beside them**, and a signature over one could be
/// offered as a signature over the other wherever the two encodings happened
/// to agree. B9 does not name a separation and this is Build's own, chosen
/// because it costs nothing and closes a whole class of substitution.
pub const THE_MANIFEST_SIGNING_CONTEXT: &[u8] = b"smoozy.server-package.manifest.1\0";

/// **The context every key-set statement's root signature covers**, and it is
/// deliberately not [`THE_MANIFEST_SIGNING_CONTEXT`]. See that constant.
pub const THE_KEY_SET_SIGNING_CONTEXT: &[u8] = b"smoozy.server-package.key-set.1\0";

/// **The signature scheme, and it is Build's choice rather than the plan's.**
///
/// B9 names an *offline release-root public key* and a *root-signed key-set
/// statement* and names no algorithm; `State, identity and recovery` J10 says
/// only that the three roots are not interchangeable. **Ed25519 is chosen for
/// three reasons and one of them settles it**: the implementation is already
/// in this crate's dependency tree beneath the SSH transport, so it is no new
/// weight; a public key is 32 bytes and a signature 64, which is small enough
/// to embed in every build and to carry beside every manifest; and **it is not
/// in the machine program's tree at all** — `Rented-server runtime` takes the
/// framing crate and none of this one, which is what keeps the split of
/// 2026-09-06 from being undone here.
pub const THE_SIGNATURE_SCHEME: &str = "Ed25519";

/// A public key that verifies one of this module's two kinds of signature.
pub type PublicKeyBytes = [u8; 32];

/// A signature over a context and a document's canonical bytes.
pub type SignatureBytes = [u8; 64];

/// **Verify one signature over `context || canonical`.**
///
/// **The strict check rather than the permissive one.** The library offers
/// both; this takes the one that refuses a public key of small order and
/// refuses a non-canonical signature encoding, because a manifest that
/// verifies under two different keys is a manifest whose signer cannot be
/// named.
pub fn a_signature_verifies(
    public_key: &PublicKeyBytes,
    context: &[u8],
    canonical: &[u8],
    signature: &SignatureBytes,
) -> bool {
    let Ok(key) = ed25519_dalek::VerifyingKey::from_bytes(public_key) else {
        return false;
    };
    let signature = ed25519_dalek::Signature::from_bytes(signature);
    let mut over = Vec::with_capacity(context.len() + canonical.len());
    over.extend_from_slice(context);
    over.extend_from_slice(canonical);
    key.verify_strict(&over, &signature).is_ok()
}

/// **Make one signature over `context || canonical`, from secret bytes the
/// caller holds.**
///
/// **This function holds no key and makes none.** Whoever holds the release
/// root or an online signing key passes its bytes in; **the only producer of
/// secret bytes in this workspace is [`composing::test_only_keys`]**, and what
/// it makes is derived from a seed written down in the calling code.
///
/// **Taking the secret as an argument is the whole of how a release is signed
/// with a secret supplied at the moment**, and it is why nothing had to be built
/// for that on 2026-09-06 when the root was generated. The Creator's Mac is the
/// machine that builds Smoozy, so the release root's secret half is in their
/// password manager and in that Mac's own login keychain, and in no plain file
/// on it; at a release `App/Release/src/secret.rs` reads it out of the keychain,
/// passes it here, and nothing in this crate keeps it. **What is still owed is the release step that
/// calls this** — and, before it, the online signing key B9 has the root vouch
/// for.
pub fn a_signature_over(
    secret_key: &[u8; 32],
    context: &[u8],
    canonical: &[u8],
) -> SignatureBytes {
    use ed25519_dalek::Signer;
    let key = ed25519_dalek::SigningKey::from_bytes(secret_key);
    let mut over = Vec::with_capacity(context.len() + canonical.len());
    over.extend_from_slice(context);
    over.extend_from_slice(canonical);
    key.sign(&over).to_bytes()
}

// ------------------------------------------------------- the canonical bytes

/// Writes a document's canonical bytes, one field at a time.
///
/// **Every field is fixed-width or length-prefixed and the order is the
/// composing code's**, so one value has exactly one encoding. There is no
/// key ordering to get wrong, no number formatting to disagree about and no
/// whitespace.
#[derive(Debug, Default)]
pub struct Canonical {
    bytes: Vec<u8>,
}

impl Canonical {
    pub fn new() -> Canonical {
        Canonical { bytes: Vec::new() }
    }

    pub fn u32(&mut self, value: u32) -> &mut Canonical {
        self.bytes.extend_from_slice(&value.to_be_bytes());
        self
    }

    pub fn u64(&mut self, value: u64) -> &mut Canonical {
        self.bytes.extend_from_slice(&value.to_be_bytes());
        self
    }

    /// A run of bytes with its length before it.
    pub fn run(&mut self, value: &[u8]) -> &mut Canonical {
        self.u32(value.len() as u32);
        self.bytes.extend_from_slice(value);
        self
    }

    /// Text, as a run of its UTF-8 bytes.
    pub fn text(&mut self, value: &str) -> &mut Canonical {
        self.run(value.as_bytes())
    }

    /// A digest, written without a length because it has exactly one.
    pub fn digest(&mut self, value: &[u8; 32]) -> &mut Canonical {
        self.bytes.extend_from_slice(value);
        self
    }

    /// Present or absent, as one byte. **A field that may be absent is written
    /// as this byte and then, where present, the value** — never as an empty
    /// value standing in for absence.
    pub fn whether(&mut self, present: bool) -> &mut Canonical {
        self.bytes.push(u8::from(present));
        self
    }

    pub fn finish(self) -> Vec<u8> {
        self.bytes
    }
}

/// Reads a document's canonical bytes back.
///
/// **Every read is bounds-checked and returns nothing rather than a guess**, and
/// [`Reading::end`] refuses trailing bytes: a document with anything after its
/// last field is a different document from the one that was signed.
pub struct Reading<'a> {
    bytes: &'a [u8],
    at: usize,
}

impl<'a> Reading<'a> {
    pub fn of(bytes: &'a [u8]) -> Reading<'a> {
        Reading { bytes, at: 0 }
    }

    fn take(&mut self, how_many: usize) -> Option<&'a [u8]> {
        let to = self.at.checked_add(how_many)?;
        let out = self.bytes.get(self.at..to)?;
        self.at = to;
        Some(out)
    }

    pub fn u32(&mut self) -> Option<u32> {
        Some(u32::from_be_bytes(self.take(4)?.try_into().ok()?))
    }

    pub fn u64(&mut self) -> Option<u64> {
        Some(u64::from_be_bytes(self.take(8)?.try_into().ok()?))
    }

    pub fn run(&mut self) -> Option<&'a [u8]> {
        let length = self.u32()? as usize;
        self.take(length)
    }

    pub fn text(&mut self) -> Option<String> {
        std::str::from_utf8(self.run()?).ok().map(str::to_owned)
    }

    pub fn digest(&mut self) -> Option<[u8; 32]> {
        self.take(32)?.try_into().ok()
    }

    pub fn whether(&mut self) -> Option<bool> {
        match self.take(1)?[0] {
            0 => Some(false),
            1 => Some(true),
            // **Anything else is not a truth value.** A byte read as *present*
            // because it is not zero would make two different runs of bytes
            // read as the same document.
            _ => None,
        }
    }

    /// The reading is over and nothing is left. **A document with trailing
    /// bytes is refused.**
    pub fn end(self) -> Option<()> {
        (self.at == self.bytes.len()).then_some(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::release::composing::test_only_keys;

    #[test]
    fn a_signature_verifies_over_its_own_context_and_not_another() {
        let key = test_only_keys::a_key_pair_from_a_written_down_seed(*b"one seed for this test and no ot");
        let canonical = b"the document".as_slice();
        let signature = a_signature_over(&key.secret, THE_MANIFEST_SIGNING_CONTEXT, canonical);
        assert!(a_signature_verifies(
            &key.public,
            THE_MANIFEST_SIGNING_CONTEXT,
            canonical,
            &signature
        ));
        // The same bytes offered as the other kind of document.
        assert!(!a_signature_verifies(
            &key.public,
            THE_KEY_SET_SIGNING_CONTEXT,
            canonical,
            &signature
        ));
        // A different key.
        let other = test_only_keys::a_key_pair_from_a_written_down_seed(*b"a second seed, also written down");
        assert!(!a_signature_verifies(
            &other.public,
            THE_MANIFEST_SIGNING_CONTEXT,
            canonical,
            &signature
        ));
        // One byte of the document changed.
        assert!(!a_signature_verifies(
            &key.public,
            THE_MANIFEST_SIGNING_CONTEXT,
            b"the documenT",
            &signature
        ));
    }

    #[test]
    fn the_canonical_bytes_read_back_as_what_was_written_and_refuse_a_tail() {
        let mut composing = Canonical::new();
        composing
            .text("a format")
            .u64(7)
            .whether(true)
            .u32(3)
            .digest(&[9; 32])
            .run(&[1, 2, 3]);
        let bytes = composing.finish();

        let mut reading = Reading::of(&bytes);
        assert_eq!(reading.text().unwrap(), "a format");
        assert_eq!(reading.u64().unwrap(), 7);
        assert!(reading.whether().unwrap());
        assert_eq!(reading.u32().unwrap(), 3);
        assert_eq!(reading.digest().unwrap(), [9; 32]);
        assert_eq!(reading.run().unwrap(), &[1, 2, 3]);
        assert!(reading.end().is_some());

        let mut with_a_tail = bytes.clone();
        with_a_tail.push(0);
        let mut reading = Reading::of(&with_a_tail);
        reading.text().unwrap();
        reading.u64().unwrap();
        reading.whether().unwrap();
        reading.u32().unwrap();
        reading.digest().unwrap();
        reading.run().unwrap();
        assert!(reading.end().is_none());
    }

    #[test]
    fn a_short_document_is_refused_rather_than_guessed_at() {
        let mut composing = Canonical::new();
        composing.text("only this");
        let bytes = composing.finish();
        let mut reading = Reading::of(&bytes[..bytes.len() - 1]);
        assert!(reading.text().is_none());
    }

    #[test]
    fn a_byte_that_is_neither_present_nor_absent_is_not_read_as_present() {
        let mut reading = Reading::of(&[2]);
        assert!(reading.whether().is_none());
    }
}
