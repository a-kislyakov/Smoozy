//! Logging redaction — `Security and licence` §13.
//!
//! G1: **redaction here is *never emitted*, not *filtered before display*.** A
//! filter has to recognise what it is hiding, and the three stored kinds have no
//! recognisable shape. So five of the six rules are kept by there being no
//! route rather than by anything in this file:
//!
//! 1. **No secret is ever a process argument.** Kept by
//!    [`crate::plaintext::Plaintext`], which has no conversion to anything a
//!    command line takes.
//! 2. **No secret is ever placed in an environment value**, and **the
//!    machine's environment is never captured into any record Smoozy keeps or
//!    shows**. The first half is kept the same way as rule 1. The second is a
//!    rule about what is read at all: a rented machine's own settings hold the
//!    provider's secrets in the clear, so anything that read them into a log
//!    would be publishing a third party's secrets into a panel the user can
//!    open.
//! 3. **No host key, and no host key fingerprint, appears in any log, message
//!    or exported record** — §5.4 extended past the screen. A key's own
//!    fingerprint is a different matter and `Keys` shows it deliberately.
//! 4. **No computer identity, and no part of it, in any
//!    log, any message, the diary, or any exported record.** Enforceable **at
//!    the write**, which is the framing rather than a filter at the cut: a
//!    presented sentence is captured verbatim into the diary at its first
//!    presentation, so barring it at the message is what leaves the cut
//!    nothing to recognise.
//! 6. **A failure message names the thing that failed, never the credential it
//!    failed with.**
//!
//! Rule 5 is the one with anything to run, and it is here.

/// Rule 5: **a tool's own output is captured whole or not at all, and is never
/// mined for a cause** — with one named exception, and it is named on a string
/// **observed** in fixed wording rather than on a documented one.
///
/// Fact 37 establishes that no OpenSSH manual documents any of the diagnostic
/// strings the tools emit, and that at default log level five materially
/// different causes collapse into one message. Raising the log level to reach
/// where they do separate is not done on any ordinary path: those levels print
/// key paths and identity material.
pub const A_TOOLS_OUTPUT_IS_NEVER_MINED_FOR_A_CAUSE: () = ();

/// The exception, and it reaches the elevation tool alone. Creator-stated,
/// 2026-08-25.
///
/// **The bound is the whole of the exception**: it applies to the elevation
/// tool's own refusal strings **as observed**, matched as exact constants and
/// to nothing else. **Exactly one of the three refusals is such a constant** —
/// the wrong-password ending below. The not-an-administrator message **names
/// the host**, so it is never matched as a constant and nothing is concluded
/// from it.
///
/// Observed at `sudo` **1.9.15p5** under Ubuntu 24.04's stock sudoers,
/// 2026-08-22, Fact 108. **Documented nowhere this stage reads**, which §14
/// item 17 records as permanently unresolved: a second release wants a rental
/// and none is coming.
///
/// **Guard, §14 item 17's own words: the observed strings must never be
/// widened, generalised or turned into a pattern.** A later reader who matches
/// this loosely, or who assumes a second release words it the same way, would
/// fire the erase on a machine nothing observed and discard a working
/// password.
pub const WRONG_PASSWORD_ENDING: &str = "sudo: 1 incorrect password attempt";

/// What a failed elevation validate was established to be — `Security and
/// licence` §4 R8, and §13 rule 5's exception.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ElevationRefusal {
    /// The captured text ends in the observed wrong-password constant, matched
    /// exactly. **This is an authentication result separable from the
    /// requested operation's own**, so §2's *the password ceasing to work*
    /// erasure fires on it.
    WrongPassword,
    /// Anything else — the host-naming not-an-administrator message, an
    /// unrecognised string, or a release or machine kind whose wording was
    /// never observed. **Not established to be an authentication failure**, so
    /// the password is kept and the step is reported refused.
    ///
    /// That is the behaviour that stood before the exception existed, and the
    /// failure direction is the safe one: a good password is never discarded
    /// on a failure that was not established to be an authentication failure.
    NothingConcluded,
}

impl ElevationRefusal {
    /// Whether §5.2's erasure of the remembered machine password fires.
    pub fn erases_the_remembered_password(self) -> bool {
        matches!(self, ElevationRefusal::WrongPassword)
    }
}

/// Reads a failed validate's captured text. **Exactly one string exactly, or
/// it concludes nothing.**
///
/// The constant is the refusal's *ending*, so what is compared is the last
/// line that has anything on it, byte for byte. Nothing is trimmed from inside
/// it, nothing is lower-cased, nothing is matched as a substring, and no
/// second string is recognised.
pub fn read_a_failed_validate(captured_text: &str) -> ElevationRefusal {
    let ending = captured_text
        .lines()
        .rev()
        .find(|line| !line.trim().is_empty());

    match ending {
        Some(line) if line == WRONG_PASSWORD_ENDING => ElevationRefusal::WrongPassword,
        _ => ElevationRefusal::NothingConcluded,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_observed_ending_is_read_as_a_wrong_password() {
        assert_eq!(
            read_a_failed_validate("Password:\nsudo: 1 incorrect password attempt\n"),
            ElevationRefusal::WrongPassword
        );
    }

    #[test]
    fn a_wrong_password_erases_the_remembered_one() {
        assert!(ElevationRefusal::WrongPassword.erases_the_remembered_password());
        assert!(!ElevationRefusal::NothingConcluded.erases_the_remembered_password());
    }

    #[test]
    fn the_host_naming_refusal_concludes_nothing() {
        // The not-an-administrator message names the host, so it can never be
        // an exact constant across machines and is never matched.
        assert_eq!(
            read_a_failed_validate("smoozy is not in the sudoers file.\n"),
            ElevationRefusal::NothingConcluded
        );
        assert_eq!(
            read_a_failed_validate(
                "sudo: a password is required\nsmoozy is not allowed to run sudo on box-14.\n"
            ),
            ElevationRefusal::NothingConcluded
        );
    }

    #[test]
    fn the_constant_is_never_matched_loosely() {
        // Each of these is the guard §14 item 17 sets down: a near miss is a
        // string nothing observed, and firing on one would discard a working
        // password on a machine this project never ran.
        for near_miss in [
            "sudo: 2 incorrect password attempts",
            "sudo: 1 incorrect password attempt.",
            "Sudo: 1 incorrect password attempt",
            "  sudo: 1 incorrect password attempt",
            "sudo: 1 incorrect password attempt (try again)",
            "1 incorrect password attempt",
        ] {
            assert_eq!(
                read_a_failed_validate(near_miss),
                ElevationRefusal::NothingConcluded,
                "matched loosely: {near_miss}"
            );
        }
    }

    #[test]
    fn an_empty_capture_concludes_nothing() {
        assert_eq!(
            read_a_failed_validate(""),
            ElevationRefusal::NothingConcluded
        );
        assert_eq!(
            read_a_failed_validate("\n\n  \n"),
            ElevationRefusal::NothingConcluded
        );
    }

    #[test]
    fn trailing_blank_lines_do_not_hide_the_ending() {
        assert_eq!(
            read_a_failed_validate("sudo: 1 incorrect password attempt\n\n"),
            ElevationRefusal::WrongPassword
        );
    }
}
