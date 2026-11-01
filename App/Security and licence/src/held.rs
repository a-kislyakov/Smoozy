//! The retained things Smoozy holds — `Security and licence` §2.
//!
//! H1: one arrangement covers the retained things, and what differs between them is which
//! store, what the item is keyed by, what erases it, and whether they are
//! stored at all. This file carries the two halves of that table a builder can
//! act on: the three kinds that live in the operating system's secret store,
//! and the two arrangements that are held and never stored (H3).

use crate::plaintext::Plaintext;

/// The three retained secret kinds plus the reserved store probe — `Security
/// and licence` §3.0 S1 and §3.3 S24.
///
/// These are the three retained rows of §2 whose value lives in the store. Item 1 (the
/// SSH private key half) is not Smoozy's to keep, item 6 (a machine's
/// remembered identity) is public material in the local record and not a
/// secret (H2), and the copies of the token on rented machines are not items
/// in this store (§10).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StoredKind {
    /// §2 item 2. Keyed by the local key identifier. Erased by the key being
    /// deleted or removed, the user clearing it, the fingerprint at that path
    /// changing, or the passphrase ceasing to work.
    KeyPassphrase,
    /// §2 item 3, the *remember it* choice. Keyed by the local server
    /// identifier. Erased by the server being edited and cleared, the username
    /// changing, the server being removed, or the password ceasing to work.
    MachinePassword,
    /// §2 item 5, the desktop's own copy. One item, fixed. Erased by the token
    /// being replaced (E14); switching Telegram off keeps it since 2026-10-31.
    /// The copies on rented
    /// machines are §10's and are not items in this store.
    BotToken,
    /// Reserved throwaway round trip — §3.3 S24. Never a user secret.
    StoreProbe,
}

impl StoredKind {
    /// The complete set, in the order §3.0 S1 names them. Used by the erase
    /// sweep and by anything that has to walk the kinds rather than guess at
    /// them.
    pub const ALL: [StoredKind; 4] = [
        StoredKind::KeyPassphrase,
        StoredKind::MachinePassword,
        StoredKind::BotToken,
        StoredKind::StoreProbe,
    ];

    /// The stable text a store adapter builds its item name from. It carries
    /// the kind and nothing else; the subject beside it carries no more
    /// (§3.0 rule 1).
    pub fn as_str(self) -> &'static str {
        match self {
            StoredKind::KeyPassphrase => "key-passphrase",
            StoredKind::MachinePassword => "machine-password",
            StoredKind::BotToken => "bot-token",
            StoredKind::StoreProbe => "store-probe",
        }
    }
}

/// What an item is stored against — `Security and licence` §3.0 S1 and rule 1.
///
/// **It is opaque and carries nothing about the user.** Never a host, a port, a
/// username, a file path, a key comment or a fingerprint. The reason is
/// documented and Linux-specific: the Secret Service specification says the
/// lookup attributes *"are not treated as secret material, and the service may
/// choose not to encrypt attributes when storing them to disk"*, so an
/// attribute set naming the user's servers and keys would be a plaintext
/// inventory of them beside the encrypted secrets.
///
/// The only way to make one is [`Subject::issued_by_the_local_record`], whose
/// name is the rule: the value comes from the local record's own identifier
/// for the row, and is composed from nothing the user typed or the machine
/// reported.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Subject(String);

impl Subject {
    /// Takes an identifier the local record issued. Whichever kind of row it
    /// belongs to — a key row, a server row, or the one fixed row the bot
    /// token takes — the identifier is the record's and
    /// means nothing outside it.
    pub fn issued_by_the_local_record(identifier: impl Into<String>) -> Self {
        Subject(identifier.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// A (kind, subject) pair: the whole of what one item is identified by, and
/// the whole of what a module may hold about a secret.
///
/// `Desktop app and delivery` S6's second shape — *"a handle it can present
/// again, which names a kind and a subject and carries no value"*. It is a
/// separate type from the value on purpose: a module holding one of these
/// cannot accidentally be holding a plaintext, which is §1.4 B30.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct HeldSecret {
    pub kind: StoredKind,
    pub subject: Subject,
}

impl HeldSecret {
    pub fn new(kind: StoredKind, subject: Subject) -> Self {
        HeldSecret { kind, subject }
    }
}

/// The two things that are held and never stored — `Security and licence`
/// §2 items 3b and 3c, and H3's reason for them.
///
/// Neither Windows' `CRED_PERSIST_SESSION` nor Linux's session collection
/// delivers what the spec asks — both are documented against the logon session
/// rather than the application, and macOS documents no such facility at all —
/// so the one arrangement that exists on all three systems is **process
/// memory the application itself discards**, and that is what is used.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UnstoredHold {
    /// §2 item 3b — a machine password held *while this connection lasts*.
    /// Keyed by **the connection, not the connected period**. A connection
    /// Smoozy replaces underneath a standing connected period is a new
    /// connection and inherits nothing (E12).
    ForThisConnection,
    /// §2 item 3c — a machine password held *do not remember it*, for the one
    /// action, discarded when the action ends however it ends.
    ForThisAction,
}

/// Holds an unstored value against the thing that ends it.
///
/// The key is whatever the owning module's identity for that thing is — a
/// connection identity for [`UnstoredHold::ForThisConnection`], an action's for
/// `ForThisAction`. **Ending the owner erases the
/// value**, which is §5.2 E12 for a connection and H3's arrangement for the
/// other hold.
#[derive(Debug)]
pub struct UnstoredHolder<Owner: Eq + std::hash::Hash> {
    hold: UnstoredHold,
    held: std::collections::HashMap<Owner, Plaintext>,
}

impl<Owner: Eq + std::hash::Hash> UnstoredHolder<Owner> {
    pub fn new(hold: UnstoredHold) -> Self {
        UnstoredHolder {
            hold,
            held: std::collections::HashMap::new(),
        }
    }

    pub fn which(&self) -> UnstoredHold {
        self.hold
    }

    /// Holds a value against its owner. A second hold for one owner replaces
    /// the first, and the first is erased on being dropped.
    pub fn hold(&mut self, owner: Owner, value: Plaintext) {
        self.held.insert(owner, value);
    }

    pub fn is_held_for(&self, owner: &Owner) -> bool {
        self.held.contains_key(owner)
    }

    /// Takes the value out for one use. It leaves the holder, so a use is a
    /// use and never a copy; the caller holds it again afterwards if the owner
    /// has not ended.
    pub fn take(&mut self, owner: &Owner) -> Option<Plaintext> {
        self.held.remove(owner)
    }

    /// The owner ended, however it ended. §5.2 E12: the user disconnecting,
    /// the connection dropping, or the application closing.
    pub fn owner_ended(&mut self, owner: &Owner) {
        self.held.remove(owner);
    }

    /// The run is ending. Everything held goes, whichever hold this is.
    pub fn discard_all(&mut self) {
        self.held.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_secret_kinds_and_the_reserved_probe_are_closed() {
        assert_eq!(StoredKind::ALL.len(), 4);
    }

    #[test]
    fn a_held_secret_carries_no_value() {
        let handle = HeldSecret::new(
            StoredKind::MachinePassword,
            Subject::issued_by_the_local_record("server-7"),
        );
        assert_eq!(handle.subject.as_str(), "server-7");
        // There is no field on it that could carry one.
    }

    #[test]
    fn ending_the_owner_erases_the_hold() {
        let mut holder = UnstoredHolder::new(UnstoredHold::ForThisConnection);
        holder.hold(41u32, Plaintext::new("machine password"));
        assert!(holder.is_held_for(&41));
        holder.owner_ended(&41);
        assert!(!holder.is_held_for(&41));
    }

    #[test]
    fn a_replacement_connection_inherits_nothing() {
        // E12: a connection Smoozy replaces underneath a standing connected
        // period is a new connection, so it is a different owner and holds
        // nothing of the old one's.
        let mut holder = UnstoredHolder::new(UnstoredHold::ForThisConnection);
        holder.hold(41u32, Plaintext::new("machine password"));
        holder.owner_ended(&41);
        holder.hold(42u32, Plaintext::new("machine password"));
        assert!(!holder.is_held_for(&41));
        assert!(holder.is_held_for(&42));
    }

    #[test]
    fn taking_a_value_removes_it_from_the_holder() {
        let mut holder = UnstoredHolder::new(UnstoredHold::ForThisAction);
        holder.hold("action-1".to_string(), Plaintext::new("secret"));
        let taken = holder.take(&"action-1".to_string());
        assert!(taken.is_some());
        assert!(!holder.is_held_for(&"action-1".to_string()));
    }

    #[test]
    fn the_run_closing_discards_everything() {
        let mut holder = UnstoredHolder::new(UnstoredHold::ForThisAction);
        holder.hold(1u32, Plaintext::new("machine password"));
        holder.discard_all();
        assert!(!holder.is_held_for(&1));
    }
}
