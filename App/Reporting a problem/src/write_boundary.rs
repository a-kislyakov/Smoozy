//! What the diary never holds, and what it holds that is the user's —
//! `Reporting a problem` §4.
//!
//! N1: **the boundary is enforced at the write, and the rule is one sentence:
//! nothing enters the diary that may not leave the user's computer in a
//! report.** This is `Security and licence` §13's own framing — *"Redaction
//! here is never emitted, not filtered before display. A filter has to
//! recognise what it is hiding"* — applied as the diary's constitution.
//!
//! **So the report cut performs no filtering of any kind**: it slices, trims
//! and states, and every byte it may include was admissible the moment it was
//! written. A rule applied at the cut instead would be a filter, and the four
//! stored kinds have no recognisable shape for one to find.
//!
//! N5: **the diary is neither §13's *log* nor an *exported record*: it is its
//! own named surface, held to the exported standard at the write.** Calling it
//! a log would inherit four rules and lose the deliberate promise below;
//! calling it nothing would inherit none and let convenience write what it
//! liked.
//!
//! ## N2 — what never enters, each exclusion on its ground
//!
//! - **A stored secret value or any part of one** — the passphrase, any key,
//!   the bot token. **Structural twice over**: the
//!   adapter contract means no module ever holds a plaintext to write
//!   (`Desktop app and delivery` S6, on `Security and licence` B30, in that
//!   document's §1.4), and
//!   `Security and licence` §3.0 rule 6 bars a stored value from every
//!   exported record or log. In this code that is
//!   [`smoozy_security_and_licence::plaintext::Plaintext`], which has no route
//!   to a string at all.
//! - **A host key or its fingerprint** — §13 rule 3.
//! - **The local computer identity, and any part of
//!   it** — this document's own prohibition, written at the write and
//!   unconditionally, because §13 rule 4 bound *any log* and nothing wider
//!   when Row 457 was asked. Rule 4 was widened on 2026-08-25 and now names
//!   the diary in its own text.
//! - **The machine's environment** — §13 rule 2 already binds any record
//!   Smoozy keeps, and the diary is one.
//! - **The contents of the user's files** — a prohibition on a new store: no
//!   bytes and no text read out of any file of theirs. **The diary holds names
//!   of things, never their insides.**
//!
//! ## N3 — what it holds deliberately
//!
//! **File paths, folder names, scene names, machine names, usernames.** The
//! spec's own list, Creator-stated, and what keeps it from colliding with N2
//! is the **acted-on rule**: a diary line names the things the recorded act
//! was about — the path the user picked, the file the module read, the scene
//! submitted, the server's label and login the connection used — and **never
//! inventories things no act touched**. A line says *read a listing at this
//! folder*; it never carries the listing.
//!
//! ## N4 — two rules that look like they reach here and do not
//!
//! `Security and licence` §3.0 rule 1 — *"Never a host, a port, a username, a
//! file path, a key comment or a fingerprint"* — is **the secret store's own
//! attribute rule** and reaches no further than that store's attribute set.
//! Applying it here would delete exactly what N3 exists to keep. And §13 rule
//! 6's *"no path beyond what the user already sees"* binds the failure
//! **message**, which the diary holds verbatim and which already obeyed it —
//! not the act lines, whose paths are the user's own acts' subjects.

/// The acted-on rule, as the one question a builder asks of any value it is
/// about to put in a line.
///
/// It is a rule about **what is written**, not a filter over what was written:
/// N1 forbids the second shape in as many words. So this is checked where the
/// declaration is composed, and there is nothing at the cut that could catch
/// what got past it here.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MayEnterALine {
    /// The recorded act was about this thing: the path the user picked, the
    /// file the module read, the scene submitted, the server's label.
    YesTheActWasAboutIt,
    /// Nothing acted on it. An inventory, a listing, a directory's contents, a
    /// neighbouring file — none of it enters, however useful it would be to
    /// read.
    NoNothingActedOnIt,
    /// One of N2's five exclusions. It never enters, whatever the act was.
    NeverItIsOneOfTheFiveExclusions,
}

impl MayEnterALine {
    pub fn permitted(self) -> bool {
        matches!(self, MayEnterALine::YesTheActWasAboutIt)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn only_what_the_act_was_about_enters_a_line() {
        assert!(MayEnterALine::YesTheActWasAboutIt.permitted());
        assert!(!MayEnterALine::NoNothingActedOnIt.permitted());
        assert!(!MayEnterALine::NeverItIsOneOfTheFiveExclusions.permitted());
    }
}
