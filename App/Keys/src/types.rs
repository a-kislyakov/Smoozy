//! The supported types, the offered set and the default — `Keys` §2.
//!
//! **Two sets, and confusing them is the fault this file exists to prevent.**
//! The **supported** set is fixed here, in this binary, and is what a key on
//! the user's disk is judged against (T1, T5). The **offered** set is a
//! runtime reading of the installed `ssh-keygen` (T2) and is what the make-key
//! dialog may put in front of the user. The offered set may narrow; the
//! supported set never does.

/// **T1: the supported set is fixed at three.**
///
/// Three of the four plain file-backed key types `ssh-keygen(1)` documents.
/// The fourth, `mldsa44-ed25519`, is left out because **no binary this project
/// has read offers it** — the page serves OpenBSD-current and the observed
/// 10.2p1's own installed manual renders the `-t` list without it (Fact 32,
/// observed) — so admitting it would put a type in the accepted set on the
/// manual's authority alone, which is what [`OfferedSet`] exists to refuse.
///
/// **T3: the authenticator-hosted types (`ed25519-sk`, `ecdsa-sk`) are
/// excluded by decision and never by probe.** Their private half is completed
/// by hardware Smoozy has no route to drive, generation requires an enrollment
/// interaction, and `Security and licence` §6 establishes that token-supplied
/// keys can exist with no file at all — the shape this module's file-backed
/// row does not hold. A key of an unsupported type in a key location is listed
/// and shown as *not a supported type*, which is the truth the spec's
/// *unusable* vocabulary already carries.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum KeyType {
    Ed25519,
    Ecdsa,
    Rsa,
}

impl KeyType {
    /// T1's three, in the order T4 falls back through them.
    pub const SUPPORTED: [KeyType; 3] = [KeyType::Ed25519, KeyType::Ecdsa, KeyType::Rsa];

    /// The value `-t` takes, and the name the tool prints back. **The two are
    /// one string on purpose**: a mapping between what is asked for and what is
    /// read back would be a fact this project has no source for, which is what
    /// T2b declines `ssh -Q key-plain` over.
    pub fn as_str(self) -> &'static str {
        match self {
            KeyType::Ed25519 => "ed25519",
            KeyType::Ecdsa => "ecdsa",
            KeyType::Rsa => "rsa",
        }
    }

    /// **T5: a type never leaves the supported set at runtime.** Reading a key
    /// the tool names gives back one of T1's three or nothing at all — and
    /// nothing at all is *not a supported type*, never a fourth member
    /// admitted because the tool said a word.
    pub fn from_the_tools_name(name: &str) -> Option<KeyType> {
        // The tool prints the type in upper case on the fingerprint line
        // (`(ED25519)`, `(RSA)`) and takes it in lower case at `-t`. One
        // comparison covers both rather than two spellings drifting apart.
        let name = name.trim().trim_matches(['(', ')']).to_ascii_lowercase();
        KeyType::SUPPORTED
            .into_iter()
            .find(|supported| supported.as_str() == name)
    }
}

/// **T2: the offered set for generation is a runtime reading of the installed
/// binary, never the manual's list.**
///
/// The reading itself is the acting half's: for each supported type, attempt a
/// generation with `-N ""` and a throwaway comment into Smoozy's own temporary
/// area, verify a key pair resulted, and delete it at once. A type whose
/// attempt fails is not offered. **The probe is taken once per application
/// run**, at the first moment the make-key dialog needs the list, never at
/// application start.
///
/// **Its failure doubles as the no-generator detection of G6**, which is why
/// [`OfferedSet::NoGenerator`] is a state of this value rather than a separate
/// answer somebody has to remember to ask for.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OfferedSet {
    /// **T2a: the probe has not answered yet.** The dialog draws without
    /// waiting and the type control carries no entry that has not been read —
    /// no type is offered on the manual's authority while the probe runs.
    StillReading,
    /// The types the installed binary made a key of, in T1's order.
    Read(Vec<KeyType>),
    /// **G6: the probe answered that this computer has no generator.** The
    /// make-key dialog is replaced by the statement rather than opened beside
    /// it — a dialog whose one control can offer nothing is not a screen the
    /// user asked for — and the same absence takes down every reading in §4.3
    /// that runs the tool.
    NoGenerator,
}

impl OfferedSet {
    /// What the dialog may put in the type control at this moment. **Never a
    /// type the probe has not read**, which is the same refusal T2 makes
    /// everywhere else.
    pub fn offerable(&self) -> &[KeyType] {
        match self {
            OfferedSet::Read(types) => types,
            OfferedSet::StillReading | OfferedSet::NoGenerator => &[],
        }
    }

    /// **T4: the default is `ed25519`, and it is a judgement, labelled as
    /// one.** The grounds are the manual's own no-argument behaviour — *"If
    /// invoked without any arguments, ssh-keygen will generate an Ed25519
    /// key"* — and the same page labelling SHA1 `ssh-rsa` signatures not
    /// recommended.
    ///
    /// **Where the probe finds `ed25519` absent the default falls to `ecdsa`,
    /// then `rsa`** — the same judgement applied down the list — and it is
    /// **taken from the offered set once that set exists and never ahead of
    /// it**, which is why this answers `None` while the probe is still
    /// running.
    pub fn default_type(&self) -> Option<KeyType> {
        let offerable = self.offerable();
        KeyType::SUPPORTED
            .into_iter()
            .find(|kind| offerable.contains(kind))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_supported_set_is_the_three_and_nothing_the_manual_adds() {
        assert_eq!(KeyType::SUPPORTED.len(), 3);
        assert_eq!(KeyType::from_the_tools_name("mldsa44-ed25519"), None);
        assert_eq!(KeyType::from_the_tools_name("ed25519-sk"), None);
        assert_eq!(KeyType::from_the_tools_name("dsa"), None);
    }

    #[test]
    fn a_type_is_read_back_in_either_case_the_tools_print_it_in() {
        assert_eq!(
            KeyType::from_the_tools_name("(ED25519)"),
            Some(KeyType::Ed25519)
        );
        assert_eq!(KeyType::from_the_tools_name("rsa"), Some(KeyType::Rsa));
    }

    #[test]
    fn nothing_is_offered_or_defaulted_while_the_probe_is_still_reading() {
        let offered = OfferedSet::StillReading;
        assert!(offered.offerable().is_empty());
        assert_eq!(offered.default_type(), None);
    }

    #[test]
    fn no_generator_offers_nothing_and_defaults_to_nothing() {
        let offered = OfferedSet::NoGenerator;
        assert!(offered.offerable().is_empty());
        assert_eq!(offered.default_type(), None);
    }

    #[test]
    fn the_default_falls_down_the_list_as_the_probe_narrows_it() {
        assert_eq!(
            OfferedSet::Read(vec![KeyType::Ed25519, KeyType::Ecdsa, KeyType::Rsa]).default_type(),
            Some(KeyType::Ed25519)
        );
        assert_eq!(
            OfferedSet::Read(vec![KeyType::Ecdsa, KeyType::Rsa]).default_type(),
            Some(KeyType::Ecdsa)
        );
        assert_eq!(
            OfferedSet::Read(vec![KeyType::Rsa]).default_type(),
            Some(KeyType::Rsa)
        );
    }
}
