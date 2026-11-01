//! The SSH transport Smoozy drives inside its own process.
//!
//! **`Servers` K1**: the connection is held by the SSH transport Smoozy itself
//! drives, *"and that transport is the supplier of the connection's observed
//! value"*. **K1a**: no `ssh`, `sftp` or `scp` process is run for it at any
//! moment, so no option handed to one governs it — what the connection is
//! given instead is a value on a call.
//!
//! **What is in this module and what is next door.** Here: the shapes the
//! caller works in — the endpoint, the credential, the keepalive figures, the
//! host-trust decision, the ways a connection ends, and the one command the
//! command channel carries. Next door in [`ssh`]: the driving itself.
//!
//! **Everything here is testable at the desk.** Nothing in this file opens a
//! socket.
//!
//! **Markers used below that belong to another document, each with the
//! document it belongs to.** `Servers` D9, K1, K1a, K2, K3, K5, N2, P8, Q6,
//! Q9, Q12. `Security and licence` R2a, T2a.
//!
//! **Two more belong to `Security and licence` and are written out at their
//! sites rather than only here — §6 P1 and T5** — because this plan defines a
//! P1 and a T5 of its own, and the header list is not enough for a marker that
//! is a real clause of both documents.

pub mod ssh;

use std::path::PathBuf;
use std::time::Duration;

use smoozy_security_and_licence::plaintext::Plaintext;

/// Where a connection is made to — three of `Servers` [C1]'s four fields.
///
/// The fourth is the key reference, which names a key rather than being one;
/// what reaches the transport is [`Credential`]'s path. **`Servers` C6:
/// nothing is derived from a host, a port or a username**, and nothing here
/// synthesises a path from any of them. **That document's C6 and not this
/// one's**, which is about every request being idempotent or a pure reading.
///
/// [C1]: `Preparation/Architecture/Servers.md` §13.1
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Endpoint {
    /// `Servers` C1. What the user typed, reached as typed.
    pub host: String,
    /// `Servers` C4: the field starts at 22, and a machine reached through a
    /// mapped port is reached at the mapped number.
    pub port: u16,
    /// `Servers` C1. Who logs in once the machine is reached.
    pub username: String,
}

/// What authentication is given — `Security and licence` §6 P1 and §4 R2a.
///
/// **`Security and licence` §6 P1: Smoozy holds a path and a fingerprint**,
/// never key material. **The document is written out here rather than left to
/// the line above or to the header list**, because this plan defines a P1 of
/// its own — request framing — and a bare marker does not fail to resolve: it
/// resolves to a real and wrong clause, which nobody reading it can tell from a
/// right one. The blank comment line separates this sentence from the citation
/// above, so that citation does not vouch for it. The path
/// is a value handed to the transport on a call (`Servers` K1a), and it is the
/// transport's own key-loading step that reads the file — `Security and
/// licence` §1.4's third hop, *"a hop that opens no channel and starts no
/// process"*.
///
/// **The passphrase is a [`Plaintext`] and never a `String`.** §4 R2a's four
/// rules bound what the in-process unlock costs, and three of the four are
/// structural in that type: it cannot be written to a file, a store, a record
/// or a log, it cannot become a process argument or an environment value, and
/// it erases itself when it is dropped. The fourth — *never handed to another
/// module* — is kept by this field taking the value by move at the moment of
/// the exchange and by [`Plaintext::expose`] consuming it.
pub struct Credential {
    /// The private key file's path. **Never copied, never retained, and read
    /// only at authentication** (`Servers` P8).
    pub private_key_file: PathBuf,
    /// Absent where the key has no passphrase, or where one is needed and none
    /// is available — in which case `Servers` P1 to P3 mean the attempt is
    /// never made and this transport is never reached.
    pub passphrase: Option<Plaintext>,
}

impl std::fmt::Debug for Credential {
    /// **§13 rule 4.** The path is `Keys`' own and is not a secret; the
    /// passphrase is named as present or absent and never rendered.
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("Credential")
            .field("private_key_file", &self.private_key_file)
            .field("passphrase", &self.passphrase.is_some())
            .finish()
    }
}

/// The keepalive figures, handed in rather than decided here.
///
/// **`Servers` K3 owns them and this crate owns none of them**: the interval is
/// 15 seconds, the count is 3, and the break is declared when three
/// consecutive requests have gone unanswered **and one further interval has
/// elapsed**. That third rule is what fixes the 45-to-60-second bound, and
/// `Servers`' `keepalive::KeepaliveCount` is the one type that holds it. This
/// struct carries the two figures across the boundary and states the third in
/// its own words so a reader of this crate cannot compose the bound wrongly.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Figures {
    /// **K2's second bullet.** The transport sends its own keepalive request on
    /// the connection at this interval and counts the unanswered ones.
    pub interval: Duration,
    /// How many consecutive unanswered requests stand before the break is
    /// declared **at the end of the interval after them**.
    pub unanswered_before_the_break: usize,
}

impl Figures {
    /// The whole window a break falls into, stated so a caller can assert it
    /// rather than recompute it: the declaration lands **after**
    /// `interval × unanswered` and **at or before** `interval × (unanswered +
    /// 1)`, measured from the break.
    ///
    /// **K3's own bound at K3's own figures is 45 to 60 seconds**, and this is
    /// where that arithmetic lives.
    pub fn the_declaration_window(&self) -> (Duration, Duration) {
        let unanswered = u32::try_from(self.unanswered_before_the_break).unwrap_or(u32::MAX);
        (
            self.interval.saturating_mul(unanswered),
            self.interval.saturating_mul(unanswered.saturating_add(1)),
        )
    }
}

/// What the caller decides about the host key the machine offered.
///
/// **`Security and licence` T2a**: the gap between the two reads of the host
/// key is closed by the comparison and **not by a file**. That document's T2
/// struck the throwaway known-hosts file — *"there is no client to hand one
/// to"* — so what
/// replaces it is this: the remembered entry is read from the local record at
/// the attempt's own moment and compared in Smoozy's own process, and the
/// transport stops the attempt where the two differ.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HostTrust {
    /// The comparison approved this machine. Authentication may go ahead.
    Proceed,
    /// **`Security and licence` T5's third path.** The attempt stops before
    /// anything else happens, and no credential is presented. `Servers` Q6 owns
    /// what is raised — **and Q6 is `Servers`' and vouches for nothing about
    /// the marker before it**, which is why that document is written out at
    /// this one: this plan defines a T5 of its own, the concurrent subsystem
    /// channels of §6.1, and a bare T5 here resolves to it.
    Stop,
}

/// Who takes [`HostTrust`], asked once per attempt during key exchange.
///
/// **This runs before authentication is asked for at all** — `Security and
/// licence` T6's quoted RFC 4253 §8 step 3 and §10, and `Server protocol and
/// transfers` A5's *"The offered key is obtainable and comparable before any
/// connection that would present Smoozy's own credential."* That ordering is
/// the transport's, not a setting: there is no path through this module where
/// a credential goes first.
pub trait DecideHostTrust: Send + 'static {
    /// The offered host key, in the wire encoding the comparison is made over.
    ///
    /// **`Servers` D9: it is never shown, never logged, and never handed to any
    /// other module.** What comes back is a decision and never a value.
    fn decide(&mut self, offered: &[u8]) -> HostTrust;
}

/// Why an attempt did not produce a connection.
///
/// **§1.3: no tool's own output is mined for a cause**, and Fact 37 is the
/// reason — no OpenSSH manual documents a single one of the diagnostic strings
/// the tools emit. Nothing here parses a message. These variants are the
/// transport's own knowledge of what it was doing when it stopped, which is a
/// different thing from reading a program's words.
#[derive(Debug)]
pub enum OpenFailure {
    /// No connection could be made to the address at all. **`Servers` Q9: a
    /// scan that gets no answer is not a changed identity** — it is a
    /// connection that could not be made, reported as one, with no claim about
    /// the machine.
    TheAddressDidNotAnswer,
    /// Key exchange did not complete. Nothing is claimed about the machine.
    TheExchangeDidNotComplete,
    /// **[`HostTrust::Stop`] was returned.** The attempt stopped inside key
    /// exchange, so no credential was presented.
    HostTrustStoppedIt,
    /// The private key file could not be read or could not be unlocked with
    /// the passphrase given. `Keys` owns what that means about the key.
    TheKeyCouldNotBeLoaded,
    /// The machine refused the credential. **A4: nothing here retries an
    /// authentication** — this is reported as what it is and is not sent
    /// again on the same credential.
    TheMachineRefusedTheCredential,
    /// **C16's per-waiter deadline arrived** — `Servers` N2's 20 seconds for a
    /// connect. It **changes no durable state**, and it is its own outcome
    /// rather than one of the four above: a deadline expiring says the attempt
    /// took too long and **claims nothing about whether the address answered**,
    /// which folding it into [`OpenFailure::TheAddressDidNotAnswer`] would.
    TheDeadlineExpired,
}

/// How a connection that was live stopped being live.
///
/// **`Servers` K2: two things report a break and neither requires the caller
/// to have work to do.** Both are here, and the third variant is Smoozy's own
/// act rather than a break at all.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Ending {
    /// **K2's first bullet.** The far end closing the connection, or its own
    /// service closing the socket as it shuts down, arrives on the transport
    /// as a close. Observed: a machine closing politely and a machine powered
    /// off from a provider's console were **both told to the caller in the
    /// same second**.
    Announced,
    /// **K2's second bullet and K3's declaration.** The keepalive requests went
    /// unanswered and the further interval elapsed. This is the case that
    /// matters — the caller's own line failing — which went undetected for 21
    /// minutes 45 seconds with no keepalive.
    TheKeepaliveWentUnanswered,
    /// Smoozy tore it down. Not a break, and nothing reports one.
    TornDownBySmoozy,
}

impl Ending {
    /// **K5: a break declared is a break, and the connection under it is torn
    /// down rather than kept for later.** Observed: a ten-minute silent gap
    /// left a connection unrecoverable even after the path was restored. So
    /// repair always builds a new connection, and this answers whether an
    /// ending was one Smoozy caused.
    pub fn is_a_break(self) -> bool {
        match self {
            Ending::Announced | Ending::TheKeepaliveWentUnanswered => true,
            Ending::TornDownBySmoozy => false,
        }
    }
}

/// **One command string, at the path it has always been reached at.**
///
/// The machine's own setup steps name their constant commands too, so the type
/// lives in the half of this module that opens no connection. Nothing about it
/// changed with the move: the only way to make one is still a constant.
pub use smoozy_framing::command_string::CommandString;

/// What one command on the command channel returned.
///
/// **§1.1: one SSH `exec` request carries one command string, returning
/// output, complaint output and exit status.** All three are here because the
/// callers need all three; what none of them may do is classify a result on
/// the status alone where 255 is one of the possibilities (§1.3), because a
/// transport failure and a remote command's own failure both arrive at 255 and
/// *"it is the accompanying text that separates them, not the number"*.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommandResult {
    /// The bytes the command wrote to its standard output.
    pub output: Vec<u8>,
    /// The bytes it wrote to its complaint output.
    pub complaint: Vec<u8>,
    /// The exit status, absent where the channel closed without one — which is
    /// a real case rather than a zero to be assumed.
    pub status: Option<u32>,
}

/// **§1.1's second carriage, published as the one way a value reaches a
/// command** — *"The constant command's standard input, read as data."*
///
/// **The command string stays a [`CommandString`] and the value never touches
/// it.** That is the whole of what this route is for: §1.1 admits a value on a
/// constant command's standard input *"where the receiving program's own
/// documentation says that input is data"*, and §1.3 names the standing
/// instance — *"The elevated-access password crosses whenever a step needs it,
/// on the elevating command's standard input, and is never kept on the
/// machine."* **Whether a given program reads its input as data is the calling
/// document's to cite**, not this trait's to assume; what is published here is
/// the carriage and nothing about who may use it.
///
/// **Why the seam is here rather than in the crate that calls it.** `Servers`
/// owns its own command-channel seam for its own readings because it owns the
/// connection; this route is §1.1's and is this document's, and two
/// callers already want it — the elevated password today, and §1.1's own
/// *"every value on that entry point's standard input in this document's
/// length-delimited frame"* when §2's framing lands. One seam beside the route
/// keeps both from inventing one each.
///
/// **The value is a [`Plaintext`] rather than a slice of bytes**, because the
/// only value that rides this route today is a secret and `Security and licence`
/// §1.4 names this route as a secret's third and last hop — its list of final
/// endpoints is *"the askpass helper's own channel for a key passphrase, or the
/// elevating command's standard input on the rented machine for a machine
/// password"*. A caller that later has a value that is not a secret needs its
/// own entry point rather than a secret's type bent around it.
pub trait CarriesAValueOnACommandsStandardInput {
    fn run_carrying_a_value_on_its_standard_input(
        &mut self,
        command: CommandString,
        value: Plaintext,
    ) -> impl std::future::Future<Output = Result<CommandResult, ssh::CommandFailure>> + Send;
}

impl CommandResult {
    /// Whether the command ended cleanly. **Not a classification of a cause**:
    /// §1.3 forbids that on the status alone, and this answers only the one
    /// question a status can answer on its own.
    pub fn ended_cleanly(&self) -> bool {
        self.status == Some(0)
    }

    /// The standard output as text, or nothing where the bytes are not text.
    ///
    /// **A reading is taken as a value** (`Servers` Q12), so a reader that
    /// cannot make a value of what came back has an absence rather than a
    /// guess.
    pub fn output_as_text(&self) -> Option<&str> {
        std::str::from_utf8(&self.output).ok()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn k3s_figures_give_k3s_own_window() {
        // **K3 at K3's own figures.** The declaration lands between 45 and 60
        // seconds after a break that fell anywhere in an interval. This test is
        // here because the two figures alone do not fix that bound: without the
        // third rule, a reader gets 30-to-45.
        let figures = Figures {
            interval: Duration::from_secs(15),
            unanswered_before_the_break: 3,
        };
        assert_eq!(
            figures.the_declaration_window(),
            (Duration::from_secs(45), Duration::from_secs(60))
        );
    }

    #[test]
    fn the_window_is_not_the_interval_times_the_count() {
        // The wrong composition, asserted as wrong. A reader who takes the
        // count and the interval alone gets a window closing at 45 seconds,
        // which is where K3's window opens.
        let figures = Figures {
            interval: Duration::from_secs(15),
            unanswered_before_the_break: 3,
        };
        let (opens, closes) = figures.the_declaration_window();
        assert_eq!(opens, figures.interval * 3);
        assert_ne!(closes, figures.interval * 3);
    }

    #[test]
    fn a_torn_down_connection_is_not_a_break() {
        // K5's question, and the one ending that answers it the other way.
        assert!(Ending::Announced.is_a_break());
        assert!(Ending::TheKeepaliveWentUnanswered.is_a_break());
        assert!(!Ending::TornDownBySmoozy.is_a_break());
    }

    #[test]
    fn a_command_string_carries_only_what_the_binary_holds() {
        // §1.1's prohibition, demonstrated at the type. This compiles; a
        // `CommandString::constant(&format!("uname -s {host}"))` does not, and
        // that is the whole mechanism.
        const UNAME: CommandString = CommandString::constant("uname -s");
        assert_eq!(UNAME.as_str(), "uname -s");
    }

    #[test]
    fn a_result_with_no_status_is_not_a_clean_ending() {
        // The channel closing without a status is a real case, and reading it
        // as a zero would turn a connection that went away into a command that
        // succeeded.
        let no_status = CommandResult {
            output: Vec::new(),
            complaint: Vec::new(),
            status: None,
        };
        assert!(!no_status.ended_cleanly());
    }

    #[test]
    fn output_that_is_not_text_is_an_absence_rather_than_a_guess() {
        let result = CommandResult {
            output: vec![0xff, 0xfe],
            complaint: Vec::new(),
            status: Some(0),
        };
        assert_eq!(result.output_as_text(), None);
    }

    #[test]
    fn a_credential_never_renders_its_passphrase() {
        // §13 rule 4. The rendering says present or absent and nothing else,
        // and `Plaintext` has no `Debug` of its own to leak through.
        let credential = Credential {
            private_key_file: PathBuf::from("/somewhere/id_ed25519"),
            passphrase: Some(Plaintext::new("hunter2")),
        };
        let rendered = format!("{credential:?}");
        assert!(rendered.contains("passphrase: true"));
        assert!(!rendered.contains("hunter2"));
    }
}
