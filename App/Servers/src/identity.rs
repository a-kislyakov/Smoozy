//! The connection identity — `Servers` §13.
//!
//! **C1: a connection identity is four fields: host, port, username, and a key
//! reference.** Nothing is added for a gateway. **C2** closed Fact 61 by
//! reading three providers' own documentation on 2026-08-19: no provider read
//! puts a server behind a gateway needing its own details, and where a proxy
//! route is offered it is an address and a port and **not a second set of
//! credentials**.
//!
//! **C6: nothing in this document is derived from a host, a port or a
//! username.** `State, identity and recovery` §2.1 rules 3 and 4 bind every
//! identity, no path is synthesised from any of them, and **the local server
//! identifier is what everything is keyed on**.
//!
//! **Markers used below that belong to another document, each with the
//! document it belongs to.** `Keys` F9, F10.

/// **C1's four fields.** All four must be filled in before the server can be
/// saved, on an add and on an edit alike (**C5**).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ConnectionIdentity {
    pub host: Host,
    pub port: Port,
    pub username: Username,
    pub key: KeyReference,
}

/// The host as the user typed it. **Nothing is derived from it** (C6).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Host(String);

impl Host {
    /// **C5: it must be filled in.** An empty host is not a host, and the save
    /// is refused rather than a row written that can never reach anything.
    pub fn what_the_user_typed(text: &str) -> Option<Host> {
        let trimmed = text.trim();
        if trimmed.is_empty() {
            return None;
        }
        Some(Host(trimmed.to_string()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// **C4: the port field starts at 22, the standard SSH port** — documented,
/// `ssh_config(5)`: *"The default is 22."* A machine reached through a mapped
/// port is reached at the mapped number, which the user reads off their
/// provider's own interface and types in.
///
/// **C2's second half is why a mapped port is the ordinary case and not an
/// oddity**: one provider's own words are that each open internal port *"is
/// mapped to a random external port on the machine's (usually shared) public IP
/// address"*, and another's that *"While SSH operates on port 22 within your
/// Pod, Runpod assigns a different external port for access."*
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Port(u16);

impl Port {
    /// C4's documented default, and the value the field starts at.
    pub const STANDARD_SSH: Port = Port(22);

    /// **Port 0 is not a port.** It is the field left empty rather than a
    /// number, and C5 refuses the save.
    pub fn what_the_user_typed(number: u16) -> Option<Port> {
        if number == 0 {
            return None;
        }
        Some(Port(number))
    }

    pub fn number(self) -> u16 {
        self.0
    }
}

impl Default for Port {
    fn default() -> Self {
        Port::STANDARD_SSH
    }
}

/// The login. **C6 forbids deriving anything from it**, and `Security and
/// licence` §2 item 3's machine password belongs to it — which is why
/// [`crate::edits`]'s D14 clears the password when it changes.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Username(String);

impl Username {
    pub fn what_the_user_typed(text: &str) -> Option<Username> {
        let trimmed = text.trim();
        if trimmed.is_empty() {
            return None;
        }
        Some(Username(trimmed.to_string()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// **P1a: the row's key reference is the pair — the local key identifier *and*
/// the key fingerprint beside it — and both values are handed over at every
/// ask.**
///
/// **The identifier alone is not the reference, and the constraint is `Keys`'
/// replacement act**: it keeps the row's identifier and changes the row's
/// fingerprint (`Keys` F9 and F10), so an identifier-only ask would go on
/// answering yes for every server set up with the key that left.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct KeyReference {
    /// `Keys`' own local key identifier, as text.
    pub local_key_identifier: String,
    /// **Row 46's match value.** The fingerprint of the key this server wants,
    /// held so a key that left Smoozy entirely can be recognised when it is
    /// pointed at again later (**P16**). What a fingerprint is, and whether two
    /// keys can share one, is `Keys`' and is not decided here.
    pub fingerprint: String,
}

/// **C3: host and port together name what Smoozy is reaching**, so the spec's
/// own duplicate check rests on them and on nothing else.
///
/// **A different port means a different server**, and C2 makes that a
/// documented arrangement rather than a labelled assumption — one rented
/// machine hands out several containers, each on its own port. The username and
/// the key settle only who logs in once it is reached, so **two rows differing
/// only in username are two logins to one machine and not two machines**.
pub fn names_the_same_machine(one: &ConnectionIdentity, other: &ConnectionIdentity) -> bool {
    one.host == other.host && one.port == other.port
}

/// **C7 and C7a: what the reading does not cover, and it is stated rather than
/// glossed.**
///
/// Three providers on one date is what was read. A provider not read, or a
/// machine at a corporate or academic host reached through a bastion, is
/// outside it — and **C8** says what breaks there: the server cannot be added
/// at all, the four fields do not reach it, the first connection fails, and
/// Smoozy reports a connection it could not make with no claim about why. **The
/// route out is a fifth field, which is Specs' and not a widening this module
/// may make** (§16 item 8).
///
/// **And one of the three pages was falsified in one respect three days later**
/// (C7a): it states the seller *"adds an inbound firewall rule allowing traffic
/// from any IP (0.0.0.0/0) on port 22 via TCP"*, and on 2026-08-22 that
/// seller's machine arrived with **no inbound connection at all**. C1 and C2
/// both survive — once the port was open the four fields reached the machine —
/// but **nothing here may be read as claiming that any provider's machine
/// arrives reachable**.
pub const NO_FIFTH_FIELD_IS_ADDED_HERE: bool = true;

#[cfg(test)]
mod tests {
    use super::*;

    fn an_identity(host: &str, port: u16, username: &str) -> ConnectionIdentity {
        ConnectionIdentity {
            host: Host::what_the_user_typed(host).expect("a host"),
            port: Port::what_the_user_typed(port).expect("a port"),
            username: Username::what_the_user_typed(username).expect("a username"),
            key: KeyReference {
                local_key_identifier: "key-1".into(),
                fingerprint: "SHA256:aaa".into(),
            },
        }
    }

    #[test]
    fn the_port_field_starts_at_the_documented_default() {
        assert_eq!(Port::default(), Port::STANDARD_SSH);
        assert_eq!(Port::STANDARD_SSH.number(), 22);
    }

    #[test]
    fn c5_refuses_a_save_with_any_of_the_four_fields_empty() {
        assert!(Host::what_the_user_typed("   ").is_none());
        assert!(Username::what_the_user_typed("").is_none());
        assert!(Port::what_the_user_typed(0).is_none());
    }

    #[test]
    fn host_and_port_together_name_the_machine_and_the_username_does_not() {
        let one = an_identity("5.180.183.77", 33510, "root");
        let same_machine_other_login = an_identity("5.180.183.77", 33510, "renderer");
        assert!(names_the_same_machine(&one, &same_machine_other_login));
    }

    #[test]
    fn a_different_port_means_a_different_server() {
        // C2's own observation: one rented machine hands out several
        // containers, each on its own external port.
        let one = an_identity("5.180.183.77", 33510, "root");
        let another_container = an_identity("5.180.183.77", 34412, "root");
        assert!(!names_the_same_machine(&one, &another_container));
    }

    #[test]
    fn the_key_reference_is_the_pair_and_not_the_identifier_alone() {
        // `Keys`' replacement keeps the identifier and changes the
        // fingerprint, so an identifier-only comparison cannot tell them apart.
        let before = KeyReference {
            local_key_identifier: "key-1".into(),
            fingerprint: "SHA256:aaa".into(),
        };
        let after_a_replacement = KeyReference {
            local_key_identifier: "key-1".into(),
            fingerprint: "SHA256:bbb".into(),
        };
        assert_eq!(
            before.local_key_identifier,
            after_a_replacement.local_key_identifier
        );
        assert_ne!(before, after_a_replacement);
    }
}
