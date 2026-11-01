//! Making a key — `Keys` §6.
//!
//! **G1: generation is one act carrying four things**: a type from the advanced
//! control (default T4), the comment the user typed, an optional passphrase,
//! and a filename composed by [`crate::naming`].
//!
//! **The tool is `ssh-keygen`, invoked as a process with an argument array and
//! never through a shell.** A comment is not a secret, so an argument is a
//! permitted place for it; **a passphrase is, so it is never one** (`Security
//! and licence` §13 rules 1 and 2).

use std::path::{Path, PathBuf};

use crate::container;
use crate::types::KeyType;

/// **G3: a real passphrase reaches the generator through the askpass route and
/// never any other way.**
///
/// `-N` would put the user's passphrase on a command line another login can
/// read (Fact 60), and `Security and licence` §4 closes `-P` to everything but
/// the empty probe — **so the one documented flag is closed**, and the route is:
/// the invocation runs with no terminal, `SSH_ASKPASS` naming the one-use
/// helper and `SSH_ASKPASS_REQUIRE=force`, the helper being `Desktop app and
/// delivery` S7's, **fed over the channel the broker opened for that one
/// delivery**.
///
/// **The route is established by observation, not documentation** — Fact 115,
/// its Mac half answered on OpenSSH 10.2p1: the invocation with those two
/// variables and standard input closed invoked the askpass program exactly
/// twice and wrote a pair the `-P ''` probe then refused. **`ssh-keygen(1)`
/// documents no askpass behaviour at all**, so Windows and Linux are
/// unobserved and §13 item 3 carries what breaks — **permanently unresolved**,
/// there being no page left to read and no machine coming.
pub const ASKPASS_VARIABLE: &str = "SSH_ASKPASS";
pub const ASKPASS_REQUIREMENT: &str = "SSH_ASKPASS_REQUIRE";

/// The value that forces the helper route for a **delivery**.
pub const FORCE: &str = "force";
/// The value that closes the helper for every **promptless read** — §1's own
/// construction. **It closes the helper and not the terminal route**: the
/// prompt string still reaches standard error and a closed standard input is
/// consumed as an empty passphrase, ending at an exit status rather than a
/// hang.
pub const NEVER: &str = "never";

/// What the act was asked for.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Asked {
    pub key_type: KeyType,
    /// **Arbitrary text passed as one argument** — a comment with spaces,
    /// apostrophes and non-Latin characters was observed accepted and carried
    /// into the `.pub` and the fingerprint line verbatim.
    pub comment: String,
    /// **Whether a passphrase was asked for, and never the value.** The value
    /// itself never reaches this module's own types: `Desktop app and
    /// delivery` S6 means no module here holds a plaintext.
    pub with_a_passphrase: bool,
    pub path: PathBuf,
}

/// One process invocation, composed rather than assembled at the call site.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Invocation {
    pub program: &'static str,
    pub arguments: Vec<String>,
    /// Name-and-value pairs added to the child's environment. **Never a
    /// secret** — what is here is a helper's path and a requirement word.
    pub environment: Vec<(String, String)>,
    /// **Standard input is closed**, which is what turns a would-be prompt
    /// into an exit-status failure on this binary.
    pub standard_input_is_closed: bool,
}

pub const THE_TOOL: &str = "ssh-keygen";

/// G1, G2 and G3 in one composition.
///
/// **G2: with no passphrase, the invocation carries `-N ""`** — an empty
/// passphrase is not a secret, and this is Fact 39's established route.
/// **With one, no passphrase flag appears at all** and the askpass route
/// carries it.
pub fn invocation(asked: &Asked, the_askpass_helper: &Path) -> Invocation {
    let mut arguments = vec![
        "-t".to_string(),
        asked.key_type.as_str().to_string(),
        "-C".to_string(),
        asked.comment.clone(),
        "-f".to_string(),
        asked.path.display().to_string(),
    ];
    let mut environment = Vec::new();
    if asked.with_a_passphrase {
        environment.push((
            ASKPASS_VARIABLE.to_string(),
            the_askpass_helper.display().to_string(),
        ));
        environment.push((ASKPASS_REQUIREMENT.to_string(), FORCE.to_string()));
    } else {
        arguments.push("-N".to_string());
        arguments.push(String::new());
    }
    Invocation {
        program: THE_TOOL,
        arguments,
        environment,
        standard_input_is_closed: true,
    }
}

/// The reading G4 takes of what the tool left behind.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WhatWasMade {
    /// `Desktop app and delivery` F1's discipline: the private file at the
    /// composed path, **re-checked by path** after the tool exits.
    pub the_private_file_is_there: bool,
    /// U2's container test over the same bytes.
    pub container: Option<container::Container>,
    /// U7's fingerprint read.
    pub the_fingerprint_reads: bool,
}

/// **G4: the act verifies what it made, and the verification is what makes
/// G3's gap fail loudly everywhere.**
///
/// After the tool exits, and **before any row is written**: the private file is
/// re-checked by path, the container test runs, the fingerprint is read, **and
/// U8's protection reading must agree with what was asked**.
///
/// **Any disagreement, any missing file, any failed reading: the act is
/// reported failed, the files the act itself just wrote are removed, and no row
/// is created.** So on a system where the askpass route turns out not to be
/// honoured, the outcome is **a failed generation said plainly — never a key
/// quietly weaker than the user asked for**. A mistyped confirmation surfaces
/// the same way: the tool refuses, the act reports, nothing half-made survives.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Verified {
    /// The pair is what was asked for. The row may be written.
    ThePairIsWhatWasAskedFor,
    /// **The act is reported failed and the files it wrote are removed.**
    Failed(WhyItFailed),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhyItFailed {
    NoFileAtThePath,
    ItIsNotAPrivateKeyContainer,
    TheFingerprintWouldNotRead,
    /// **The one that matters**: a passphrase was asked for and the container
    /// says unprotected, or none was and it says protected.
    ItIsNotProtectedTheWayItWasAskedFor,
}

impl Verified {
    pub fn removes_what_the_act_wrote(self) -> bool {
        matches!(self, Verified::Failed(_))
    }

    pub fn a_row_may_be_written(self) -> bool {
        self == Verified::ThePairIsWhatWasAskedFor
    }
}

/// G4's judgement.
pub fn verify(asked: &Asked, made: &WhatWasMade) -> Verified {
    if !made.the_private_file_is_there {
        return Verified::Failed(WhyItFailed::NoFileAtThePath);
    }
    let Some(container) = made.container else {
        return Verified::Failed(WhyItFailed::ItIsNotAPrivateKeyContainer);
    };
    if !made.the_fingerprint_reads {
        return Verified::Failed(WhyItFailed::TheFingerprintWouldNotRead);
    }
    // **The protection reading must agree with what was asked**, and a reading
    // that is **absent** is a disagreement rather than a pass: a container this
    // reader could not make sense of says nothing about whether the key carries
    // the passphrase the user gave it, and G4's whole purpose is that a key is
    // never quietly weaker than what was asked for.
    if container.protected != Some(asked.with_a_passphrase) {
        return Verified::Failed(WhyItFailed::ItIsNotProtectedTheWayItWasAskedFor);
    }
    Verified::ThePairIsWhatWasAskedFor
}

/// **G9: where the usual key location does not exist, the act creates it, mode
/// 0700, before the invocation** — because macOS's `ssh-keygen` does not create
/// a missing parent (Fact 39, observed) and the spec's promise is a key *stored
/// where the operating system keeps SSH keys*. **The directory is what that
/// promise needs, so creating it is inside the promise rather than beside it.**
/// The creation goes through the platform file adapter like every local write.
pub const THE_KEY_LOCATIONS_MODE: u32 = 0o700;

/// **G10: the key files carry the modes the tool itself sets — observed 0600
/// private, 0644 public — and Smoozy never changes them, in either
/// direction** (`Security and licence` §6's constraint, kept).
///
/// This is a constant rather than a function because there is nothing to call:
/// **the whole rule is that no code here sets a mode on a key file.**
pub const SMOOZY_SETS_NO_MODE_ON_A_KEY_FILE: bool = true;

/// **G5: a passphrase entered at creation is remembered, the same as one
/// entered at a connect** — `Preparation/Specs/Whole app.md`'s Security
/// concern, *"as soon as it is entered"*, and `Preparation/Specs/Keys.md` since
/// 2026-08-25, *"wherever it is entered — including as the key is being
/// made"*.
///
/// **The act's last unit puts it to the store**, kind *key passphrase*, subject
/// issued by the local record at first store — **off the act's critical path**:
/// a put that fails is an ordinary store failure, said once under §3.0 rule 4's
/// classification, and **costs the key nothing**; the passphrase is simply not
/// held and the first connect asks, which is the spec's own fallback.
pub fn a_failed_store_put_costs_the_key_nothing() -> bool {
    true
}

/// **G8's backstop, as the act reads it.** A generation that failed because the
/// tool refused an existing `-f` target **re-runs the composition once with the
/// next free name**; G4's verification judges the result as always.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AfterTheToolRefusedAnExistingTarget {
    ComposeTheNextFreeNameAndRunOnce,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::container::{Armour, Container};

    fn asked(with_a_passphrase: bool) -> Asked {
        Asked {
            key_type: KeyType::Ed25519,
            comment: "someone@their-mac".into(),
            with_a_passphrase,
            path: PathBuf::from("/k/.ssh/smoozy_ed25519"),
        }
    }

    fn made(protected: bool) -> WhatWasMade {
        WhatWasMade {
            the_private_file_is_there: true,
            container: Some(Container {
                armour: Armour::OpenSshV1,
                protected: Some(protected),
            }),
            the_fingerprint_reads: true,
        }
    }

    #[test]
    fn a_passphrase_is_never_an_argument_and_never_an_environment_value() {
        let composed = invocation(&asked(true), Path::new("/Applications/Smoozy/askpass"));
        assert!(!composed.arguments.iter().any(|argument| argument == "-N"));
        assert!(!composed.arguments.iter().any(|argument| argument == "-P"));
        // What the environment carries is a path and a requirement word.
        assert_eq!(
            composed.environment,
            vec![
                (
                    ASKPASS_VARIABLE.to_string(),
                    "/Applications/Smoozy/askpass".to_string()
                ),
                (ASKPASS_REQUIREMENT.to_string(), FORCE.to_string())
            ]
        );
        assert!(composed.standard_input_is_closed);
    }

    #[test]
    fn with_no_passphrase_the_empty_one_is_given_and_no_helper_is_named() {
        let composed = invocation(&asked(false), Path::new("/Applications/Smoozy/askpass"));
        let flags = composed.arguments.windows(2).any(|pair| {
            pair[0] == "-N" && pair[1].is_empty()
        });
        assert!(flags);
        assert!(composed.environment.is_empty());
    }

    #[test]
    fn the_comment_goes_over_verbatim_as_one_argument() {
        let mut asked = asked(false);
        asked.comment = "Александр's mac, home".into();
        let composed = invocation(&asked, Path::new("/askpass"));
        assert!(composed
            .arguments
            .contains(&"Александр's mac, home".to_string()));
        assert_eq!(composed.program, "ssh-keygen");
    }

    #[test]
    fn a_key_quietly_weaker_than_it_was_asked_for_never_takes_a_row() {
        assert_eq!(
            verify(&asked(true), &made(false)),
            Verified::Failed(WhyItFailed::ItIsNotProtectedTheWayItWasAskedFor)
        );
        assert!(verify(&asked(true), &made(false)).removes_what_the_act_wrote());
        assert!(!verify(&asked(true), &made(false)).a_row_may_be_written());
    }

    #[test]
    fn a_key_protected_where_none_was_asked_for_fails_the_same_way() {
        assert_eq!(
            verify(&asked(false), &made(true)),
            Verified::Failed(WhyItFailed::ItIsNotProtectedTheWayItWasAskedFor)
        );
    }

    #[test]
    fn a_pair_that_matches_what_was_asked_for_may_take_a_row() {
        assert!(verify(&asked(true), &made(true)).a_row_may_be_written());
        assert!(verify(&asked(false), &made(false)).a_row_may_be_written());
    }

    #[test]
    fn a_missing_file_a_bad_container_and_an_unreadable_fingerprint_all_fail_the_act() {
        let mut nothing = made(true);
        nothing.the_private_file_is_there = false;
        assert_eq!(
            verify(&asked(true), &nothing),
            Verified::Failed(WhyItFailed::NoFileAtThePath)
        );

        let mut not_a_key = made(true);
        not_a_key.container = None;
        assert_eq!(
            verify(&asked(true), &not_a_key),
            Verified::Failed(WhyItFailed::ItIsNotAPrivateKeyContainer)
        );

        let mut unreadable = made(true);
        unreadable.the_fingerprint_reads = false;
        assert_eq!(
            verify(&asked(true), &unreadable),
            Verified::Failed(WhyItFailed::TheFingerprintWouldNotRead)
        );
    }

    #[test]
    fn the_location_is_made_at_the_mode_the_clause_names() {
        assert_eq!(THE_KEY_LOCATIONS_MODE, 0o700);
    }
}
