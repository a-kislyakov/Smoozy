//! The three hops a secret's plaintext takes, and the origin beside them.
//!
//! Keeps `Security and licence` §1.4: B27 (a stored secret's plaintext exists
//! at three hops and nowhere else), B29 (a value the user is entering has an
//! origin, and the origin is not a fourth hop), and B30 (no feature module
//! receives a stored secret as a value it may keep).
//!
//! The count of three is the whole rule, and a fourth hop is a fault. What
//! makes that checkable rather than a habit is that a plaintext is a type with
//! no way out of it: [`Plaintext`] cannot be printed, cannot be compared for
//! display, cannot be written to a record, and cannot be turned into anything
//! a command line or an environment value would take — which is §13 rules 1
//! and 2 made structural instead of remembered.

use std::fmt;

/// A secret's plaintext, in the one hop that is holding it.
///
/// The three kinds of stored secret and the three values the user types are all
/// carried in this. It is deliberately awkward: the only ways out are
/// [`Plaintext::expose`], which the endpoint that consumes the value calls,
/// and dropping it, which erases the bytes.
///
/// **What it will never grow.** No `Display`, no `Serialize`, no `Deref<str>`,
/// no conversion to an argument or an environment value, and no equality
/// against a string. Each of those is a route to a screen, a record or a
/// process listing, which §3.0 rule 6 and §13 rules 1, 2 and 4 close.
pub struct Plaintext {
    bytes: Vec<u8>,
}

impl Plaintext {
    /// Takes the value into a hop. The caller is one of: the store adapter,
    /// returning what a get read; the one-use broker; or the field the user
    /// typed into, which is B29's origin.
    pub fn new(value: impl Into<Vec<u8>>) -> Self {
        Plaintext {
            bytes: value.into(),
        }
    }

    /// Hands the bytes to the endpoint that consumes them — `Security and
    /// licence` §1.4 hop 3, or §4 R2's two consumers.
    ///
    /// It takes the value by move, so a caller cannot consume it twice and
    /// cannot keep it after the use it opened the value for. What the endpoint
    /// does with the bytes is that endpoint's, and §4 R2a's four rules bound
    /// every one of them.
    pub fn expose<T>(mut self, endpoint: impl FnOnce(&[u8]) -> T) -> T {
        let answer = endpoint(&self.bytes);
        self.erase();
        answer
    }

    /// The length in bytes. Read by nothing that draws — §3.0 rule 6's second
    /// bullet makes a mask that preserves any part of the value, its length
    /// included, the value itself. It exists for the store adapters, which
    /// have per-system size limits to keep (§3.2 S17's 2560 bytes).
    pub fn len(&self) -> usize {
        self.bytes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.bytes.is_empty()
    }

    fn erase(&mut self) {
        for byte in self.bytes.iter_mut() {
            // `write_volatile` so the compiler may not drop the write as dead.
            unsafe { std::ptr::write_volatile(byte, 0) };
        }
        self.bytes.clear();
    }
}

impl Drop for Plaintext {
    fn drop(&mut self) {
        self.erase();
    }
}

/// Says the type exists and says nothing about the value — so a plaintext that
/// reaches a debug line by accident carries no secret with it.
impl fmt::Debug for Plaintext {
    fn fmt(&self, out: &mut fmt::Formatter<'_>) -> fmt::Result {
        out.write_str("Plaintext(held)")
    }
}

/// The one-use broker of `Security and licence` §1.4 hop 2: it holds the value
/// for **one** delivery and for nothing else.
///
/// Where the delivery crosses a process boundary it starts the askpass helper
/// or writes the elevating command's standard input; where it crosses none —
/// a connection's own key-loading step, §4 R2's first bullet — there is no
/// channel to open and none is invented. The broker is one component either
/// way, which is what B27 hop 2 was widened to say on 2026-09-02.
pub struct OneUseBroker {
    value: Plaintext,
}

impl OneUseBroker {
    /// Takes the value from hop 1 — the store adapter's call — or from B29's
    /// origin where the user has just typed it.
    pub fn holding(value: Plaintext) -> Self {
        OneUseBroker { value }
    }

    /// Performs the one delivery and is spent by it. Taking `self` by move is
    /// what makes *for one use only* a property of the code rather than a rule
    /// somebody has to keep: there is no second delivery to make.
    pub fn deliver<T>(self, endpoint: impl FnOnce(Plaintext) -> T) -> T {
        endpoint(self.value)
    }
}

impl fmt::Debug for OneUseBroker {
    fn fmt(&self, out: &mut fmt::Formatter<'_>) -> fmt::Result {
        out.write_str("OneUseBroker(holding)")
    }
}

/// **B29's origin: the field the user entered a secret into.** It is not a
/// fourth hop, and this type exists so that saying so is enforceable rather
/// than remembered.
///
/// **It holds the plaintext for the length of the entry and until the act that
/// raised the ask ends**, and it is discarded when that act ends however it
/// ends, put or not put.
///
/// **Why it hands out more than one value where a hop hands out none.** B29:
/// *"From the first destination onward the three hops govern in full,
/// whichever of the two the act reaches first: the one-use broker, for that
/// attempt's one delivery, and the adapter's put where the value is to be
/// remembered."* An act can genuinely need both — `Keys` G3 delivers the
/// passphrase to the generator and `Keys` G5 then puts the same value to the
/// store — and a one-use origin would make that act ask the user twice for a
/// value they are still holding.
///
/// **What it may never grow is what [`Plaintext`] may never grow**: no
/// `Display`, no serialization, no route to an argument, an environment value,
/// a record or a screen (§13 rules 1, 2 and 4). The only thing that comes out
/// of it is a [`Plaintext`], which is itself only exposable once.
pub struct Origin {
    bytes: Vec<u8>,
}

impl Origin {
    /// **What the user typed**, taken at the field.
    pub fn what_the_user_typed(value: impl Into<Vec<u8>>) -> Self {
        Origin {
            bytes: value.into(),
        }
    }

    /// A value for one destination. **Each call is one hop's worth**, and the
    /// hops themselves are still one-use each.
    pub fn for_a_destination(&self) -> Plaintext {
        Plaintext::new(self.bytes.clone())
    }

    /// The one-use broker for one delivery — hop 2, taken from here.
    pub fn a_delivery(&self) -> OneUseBroker {
        OneUseBroker::holding(self.for_a_destination())
    }

    /// **The act ended.** Called however the act ended; dropping does the same
    /// thing, and this exists so the ending can be said in the code that ends.
    pub fn the_act_ended(self) {}

    pub fn is_empty(&self) -> bool {
        self.bytes.is_empty()
    }
}

impl Drop for Origin {
    fn drop(&mut self) {
        for byte in self.bytes.iter_mut() {
            unsafe { std::ptr::write_volatile(byte, 0) };
        }
        self.bytes.clear();
    }
}

impl fmt::Debug for Origin {
    fn fmt(&self, out: &mut fmt::Formatter<'_>) -> fmt::Result {
        out.write_str("Origin(held)")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_endpoint_sees_the_bytes() {
        let secret = Plaintext::new("open sesame");
        let seen = secret.expose(|bytes| bytes.to_vec());
        assert_eq!(seen, b"open sesame");
    }

    #[test]
    fn an_origin_serves_both_destinations_of_one_act() {
        let typed = Origin::what_the_user_typed("open sesame");
        // The delivery to the tool.
        let delivered = typed.a_delivery().deliver(|value| value.expose(|bytes| bytes.to_vec()));
        // And the put to the store, of the same value.
        let put = typed.for_a_destination().expose(|bytes| bytes.to_vec());
        assert_eq!(delivered, put);
        typed.the_act_ended();
    }

    #[test]
    fn an_origin_says_nothing_about_the_value_either() {
        let typed = Origin::what_the_user_typed("open sesame");
        assert_eq!(format!("{typed:?}"), "Origin(held)");
    }

    #[test]
    fn debug_says_nothing_about_the_value() {
        let secret = Plaintext::new("open sesame");
        assert_eq!(format!("{secret:?}"), "Plaintext(held)");
        assert!(!format!("{secret:?}").contains("sesame"));
    }

    #[test]
    fn the_broker_delivers_once_and_is_spent() {
        let broker = OneUseBroker::holding(Plaintext::new("open sesame"));
        let seen = broker.deliver(|value| value.expose(|bytes| bytes.len()));
        assert_eq!(seen, 11);
        // A second delivery does not compile: `deliver` took `self` by move.
    }

    #[test]
    fn the_length_is_readable_for_a_store_limit() {
        assert_eq!(Plaintext::new(vec![0u8; 2560]).len(), 2560);
        assert!(Plaintext::new("").is_empty());
    }
}
