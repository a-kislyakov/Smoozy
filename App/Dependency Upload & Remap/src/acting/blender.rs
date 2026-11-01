//! What this module holds about a Blender run on the machine — the desk-side
//! bound on one, the one fixed site an act still writes to, and the check's
//! reading's two marks at the path they have always been reached at.
//!
//! **No Blender is started from here any more.** The check's reading moved to
//! a file-inspection worker on 2026-09-29 (step 10c's dependency wave piece 2),
//! and **the fixing followed on 2026-10-04 (the 38's piece 7)**: it had stayed
//! a bare command through two planted links — the build and the machine file —
//! with its script and its staging path written at fixed names under the
//! carriage directory, the route `Dependency Upload & Remap` G1 and `Render
//! Settings` Q4 rule out and the one that crashed Blender at the desk. **It is
//! one request to the machine's supervisor now**
//! ([`super::fixing_worker::produce_one_fixed_copy`]), and its script and
//! launch line, with the publications they were read from, live in
//! `smoozy_dependencies_machine::fixing_script`, which the machine's own
//! program writes. **The links, the command and the four fixed sites went with
//! it.**
//!
//! **The time limit is what it was**: a Blender run this module drives carries
//! `Server Setup` B1's one bound, taken from that crate rather than chosen
//! again, so there is one number for every Blender this application starts.
//! **What it ends is the wait and not the run.**
//!
//! **Markers used in this file that belong to another document, each with the
//! document it belongs to.** `Server protocol and transfers` §1.1. `Server
//! Setup` B1. `Job files and outputs` R1. `Rented-server runtime` W1.
//! `Render Settings` W1.

use std::time::Duration;

/// **Every site on a machine that this module writes to under a fixed name**,
/// counted so nobody adds a second without meeting [`TheSitesOnOneMachine`] —
/// Q5 route 1's four-value read, which plants its token at a fixed name.
pub const THE_SITES_ONE_ACT_HOLDS: [&str; 1] =
    [super::machine::THE_FOUR_VALUE_SITE.link_path_on_the_subsystem];

/// **One machine's fixed site, and only one act on that machine may hold it.**
///
/// # What this stops
///
/// Every path on [`THE_SITES_ONE_ACT_HOLDS`] is a single fixed build-time
/// constant shared by every act of every kind. **There were five until the
/// 38's piece 7**, when the fixing's four went with its link and command; the
/// four-value read's is the one left. **Two fixings started a moment apart
/// ended with fixing A opening file B, applying B's pairs, saving to B's
/// staging path, and publishing the result at A's own address** — a fixed copy
/// of a different `.blend` standing where A's should be, with `Published`
/// returned, K8 written, V11 satisfied and nothing said anywhere. **The same
/// collision between two check readings attributed one file's references to
/// another until step 10c's dependency wave piece 2**, when the reading
/// stopped writing anything at a fixed site; it takes no holder now.
///
/// # Why serialising at the desk rather than a per-operation name
///
/// **A per-operation name is the better answer and this crate cannot write
/// it.** `Server protocol and transfers` §1.1 admits a Smoozy-synthesised
/// token in a command string as its class 2, so the *plan* allows it — but its
/// carriage is built the other way round and says so in its own words: a
/// command string holds only a build-time constant, *"the constant command
/// names the token"*, and *"one site, one token, one carriage live at a
/// time"*. Making the token per-operation is a change to that crate's
/// `CarriageSite` and `CommandString`, which is another Subject and another
/// round.
///
/// **What removes this**: `Server protocol and transfers` admitting a
/// per-operation class-2 token in a command string, which its own §1.1 rule 2
/// permits and its code does not.
///
/// # What it does not change, and what it costs
///
/// **O4's fifth declaration does not move**, and it stays *nothing declared*
/// ([`crate::declarations`]). That declaration is about **machine-level
/// exclusion** — whether the machine admits more than one operation of a kind
/// at a time — and it still declares nothing, agreeing value for value with
/// `Render Settings` §10 W1 as `Rented-server runtime` W1 requires. What
/// changes is that Smoozy does not send two at once, which is the desk's own
/// choice.
///
/// **What it costs is what O4's own cost sentence names**: a set's fixings run
/// one at a time. **What it buys** is that a fixing can no longer publish
/// another file's `.blend` under this one's address.
#[derive(Debug, Default)]
pub struct TheSitesOnOneMachine {
    held: std::sync::atomic::AtomicBool,
}

impl TheSitesOnOneMachine {
    /// One holder per machine, made where the machine is.
    pub fn for_one_machine() -> TheSitesOnOneMachine {
        TheSitesOnOneMachine::default()
    }

    /// **Take the sites for one act.** `None` where an act on this machine
    /// already holds them, which is the second act not starting.
    pub fn take(&self) -> Option<TheSitesHeld<'_>> {
        if self.held.swap(true, std::sync::atomic::Ordering::AcqRel) {
            None
        } else {
            Some(TheSitesHeld { of: self })
        }
    }
}

/// **The sites, held for the length of one act.** Every acting entry point
/// takes this by exclusive reference, so it is structural rather than a
/// comment: a second act cannot be written that borrows it while the first
/// still does, and a second act that reaches for the holder itself is refused.
#[derive(Debug)]
pub struct TheSitesHeld<'a> {
    of: &'a TheSitesOnOneMachine,
}

impl Drop for TheSitesHeld<'_> {
    fn drop(&mut self) {
        self.of
            .held
            .store(false, std::sync::atomic::Ordering::Release);
    }
}

/// **The desk-side bound on a Blender run, taken from `Server Setup` B1 rather
/// than chosen again**, so one number covers every Blender this application
/// starts.
pub const THE_TIME_LIMIT_A_BLENDER_RUN_CARRIES: Duration =
    smoozy_server_setup::blender::THE_TIME_LIMIT_EVERY_BLENDER_RUN_CARRIES;

/// **G1 and `Job files and outputs` R1: the check's reading's two marks, and
/// the document between them**, at the paths they have always been reached
/// at. **They live with the script in `smoozy-dependencies-machine`** from step
/// 10c's dependency wave piece 2, because the machine's own program writes the
/// script and reports whether the start mark was seen; the reason the marks are
/// this reading's own and not the file's reading's is written there.
pub use smoozy_dependencies_machine::check_script::{
    between_the_sentinels, THE_CLOSING_SENTINEL, THE_OPENING_SENTINEL,
};

#[cfg(test)]
mod tests {
    use super::*;

    /// **Two overlapping acts on one machine, and the second cannot start.**
    ///
    /// The fixed sites were build-time constants shared by every act of every
    /// kind, so two fixings a moment apart once ended with one publishing the
    /// other's `.blend` under its own address. The fixing writes at no fixed
    /// site since the 38's piece 7; the holder stays for the one site left.
    #[test]
    fn a_second_act_on_one_machine_cannot_start_while_the_first_holds_the_sites() {
        let machine = TheSitesOnOneMachine::for_one_machine();
        let first = machine.take().expect("the first act starts");
        assert!(
            machine.take().is_none(),
            "a second act started while the first was running"
        );
        drop(first);
        let second = machine.take().expect("the next act starts");
        assert!(machine.take().is_none());
        drop(second);
        assert!(machine.take().is_some());

        // A second machine is a second holder and is not blocked by the first.
        let other = TheSitesOnOneMachine::for_one_machine();
        let one = machine.take().expect("this machine's act");
        let two = other.take().expect("the other machine's act");
        drop((one, two));

        assert_eq!(
            THE_SITES_ONE_ACT_HOLDS,
            [crate::acting::machine::THE_FOUR_VALUE_SITE.link_path_on_the_subsystem]
        );
    }

    /// **`Dependency Upload & Remap` G1, `Render Settings` Q4 and, since the
    /// 38's piece 7, the fixing: no Blender is started through a link.** No
    /// site this module plants is a reading's or a fixing's.
    #[test]
    fn no_site_this_module_plants_is_a_readings_or_a_fixings() {
        for site in THE_SITES_ONE_ACT_HOLDS {
            assert!(!site.contains("dependency-check"), "{site} is the check's reading's");
            assert!(!site.contains("dependency-blender"), "{site} runs a Blender through a link");
            assert!(!site.contains("dependency-fix"), "{site} is the fixing's");
            assert!(!site.contains("staging"), "{site} is the fixing's");
        }
    }

    /// **G3: both marks or no document.** A stream cut short before the closing
    /// mark carries none, and a stream with neither carries none.
    #[test]
    fn a_document_needs_both_marks_and_a_short_stream_carries_none() {
        let whole = [
            b"chatter ".as_slice(),
            THE_OPENING_SENTINEL,
            b"{}",
            THE_CLOSING_SENTINEL,
            b" more chatter",
        ]
        .concat();
        assert_eq!(between_the_sentinels(&whole), Some(&b"{}"[..]));

        let cut = [b"chatter ".as_slice(), THE_OPENING_SENTINEL, b"{\"refe"].concat();
        assert_eq!(between_the_sentinels(&cut), None);
        assert_eq!(between_the_sentinels(b"nothing at all"), None);
        assert_eq!(between_the_sentinels(b""), None);
    }
}
