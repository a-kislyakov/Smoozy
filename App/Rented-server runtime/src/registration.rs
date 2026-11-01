//! §1.2, §1.3 and §1.4 — the boot registration on each machine kind, and what
//! confirms one.
//!
//! **A boot registration is confirmed by reading its own content back and
//! comparing it against the record Smoozy wrote — never by presence.** The
//! recycle observation forbids the other reading in the plainest terms: the hook
//! *"was not deleted but replaced"* by an 82-byte provider default dated at the
//! recycle moment, so *"a check for 'does my hook exist' passes while the hook
//! does nothing Smoozy asked for"*.
//!
//! **The two read-backs are not equally strong and the difference is which side
//! wrote the value.** K5 reads back a **machine-set** value — the linger flag —
//! so a machine that will not grant the registration fails the placement
//! loudly. K15 reads back **Smoozy's own content**, so it catches the recycle
//! case and **cannot catch a provider that runs no hook at the path K10 names**.
//!
//! **Markers used in this file that belong to another document, each with the
//! document it belongs to.** `Server protocol and transfers` §1.1, C16, C17,
//! C14, B4. `Job files and outputs` L13. `State, identity and recovery` D1.
//! `Server Setup` — the elevation step it owns.
//!
//! **A marker this plan also defines is never listed here and is written out
//! with its document at the site**, because a header list cannot tell the two
//! apart and a bare one there would resolve to a real and wrong clause.

use crate::machine_kind::MachineKind;

/// **K16. The boot-registration record is a record Smoozy wrote and holds the
/// registration's content by value.**
///
/// It is in the package area, and **nothing derives it from what is on the
/// machine at the moment it is read.**
#[derive(Debug, Clone, PartialEq, Eq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct BootRegistrationRecord {
    /// Which arrangement this registration is.
    pub kind: MachineKind,
    /// **Where the registration was written.** On a container this is K10's
    /// path, chosen by the provider rather than by Smoozy; on a systemd machine
    /// it is the unit's path under the supervisor login's own configuration
    /// directory.
    pub at_path: String,
    /// **The content, by value.** This is the half K15 compares against.
    pub content: Vec<u8>,
}

/// **What a systemd registration is made of — K4, and it is two acts rather
/// than one.**
///
/// A user unit under the supervisor login's own configuration directory,
/// enabled, **plus lingering granted to that login by name**. No elevated access
/// is needed for either step: the unit, the enable, the lingering grant, K5's
/// read-back and K15's confirmation all ran unprivileged and all succeeded.
///
/// **K5. Enabling alone is not the registration and its failure is silent.**
/// With the unit still enabled and only lingering turned off, the program *"did
/// not come back"* and *"no log line anywhere says why"*.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TheSystemdRegistration {
    /// The unit's content, which the record holds by value (K16).
    pub unit_content: Vec<u8>,
    /// **K18's directive is present.** See
    /// [`TheSystemdRegistration::the_kill_behaviour_is_confined_to_the_main_process`].
    pub the_kill_behaviour_is_confined_to_the_main_process: bool,
    /// **K4's second act**: whether the unit is enabled.
    pub enabled: bool,
    /// **K5's read-back, as it came back from the machine's own login manager.**
    ///
    /// `None` is a registration whose read-back has not been taken, which is not
    /// a granted one: K5 says the registration is not complete until lingering
    /// is read back **and found granted**, so an untaken read-back and one
    /// showing *not granted* are alike incomplete here.
    pub the_linger_read_back: Option<TheLingerReadBack>,
}

impl TheSystemdRegistration {
    /// **K18. The unit is written so that ending the supervisor does not end the
    /// workers it started, and that is a requirement of the registration rather
    /// than a detail of it.**
    ///
    /// The service manager's default takes the whole control group down with the
    /// main process, so **a unit written to K4 and nothing further destroys a
    /// running render on every supervisor restart** — including S4's restart
    /// after an unexpected exit, and the one G25 performs during a package
    /// commit. Observed both ways: with the default in force, killing the
    /// supervisor under a live render took the worker and Blender with it and
    /// reconciliation wrote *re-run once*; with the unit corrected, the render
    /// **finished under a supervisor it never met**, byte-for-byte identical to
    /// the uninterrupted run.
    ///
    /// **The machine permits it two independent ways** — the unit's kill
    /// behaviour confined to the main process, or each worker started in a
    /// transient scope of its own. **This document requires the first**, because
    /// it keeps K4's arrangement exactly as it stands and adds one directive.
    ///
    /// **What the reduction must not do is exempt the supervisor from being
    /// ended**: G25's deliberate end and S4's restart both act on the
    /// supervisor's own process and are unaffected.
    ///
    /// **Three things, and the third is K5's read-back.** A registration that
    /// answered complete on the enable and the directive alone committed the
    /// placement on an image whose policy refused the permission — and K5's own
    /// observation is that the program then *"did not come back"* with *"no log
    /// line anywhere"* saying why. K8 makes the permission a per-image fact that
    /// is re-read on each provider, so the read-back is the check rather than a
    /// confirmation of something already known.
    pub fn is_complete(&self) -> bool {
        self.enabled
            && self.the_kill_behaviour_is_confined_to_the_main_process
            && self
                .the_linger_read_back
                .as_ref()
                .is_some_and(|read_back| read_back.lingering_is_granted)
    }
}

/// **Nothing equivalent to K18 is owed on a container**, because S5's keeper
/// starts the supervisor as an ordinary process with no control group of its own
/// to collapse, and the orphaning behaviour there is the machine's default
/// rather than a directive.
pub const K18_IS_OWED_ON_THE_SYSTEMD_KIND_ALONE: bool = true;

/// **K6 and K7: two values that originate on the machine and never cross from
/// Smoozy.**
///
/// **K6. Lingering is requested by name and never as *self*, and the name is the
/// machine's own, derived at execution.** The bare form failed — *"Could not
/// enable linger: No such device or address"* — because *self* is derived from a
/// session that does not exist under a provider that creates no login session.
/// By name it succeeded, unprivileged.
///
/// **The connection's configured login name is not used, and the difference is
/// deliberate rather than convenient**: what the grant must name is whatever the
/// machine's own user database calls the effective user this connection's login
/// maps to, and the derivation asks the machine exactly that question, where a
/// Smoozy-carried copy could differ from the machine's own mapping for the same
/// numeric identifier.
///
/// **K7. Where the provider creates no login session, the runtime directory
/// variable is exported by hand** — a fixed literal prefix completed by the
/// login's numeric effective user identifier, derived on the machine by the same
/// route, riding the constant command string as a variable-assignment prefix.
///
/// **What both are trusted for and what neither is.** Each is the machine's own
/// claim about itself, consumed on the machine in those commands, and **never
/// read back as a value, never recorded, never correlated, and never evidence of
/// which login Smoozy connected as.** No rule anywhere keys on either.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DerivedOnTheMachine {
    /// K6's: the effective user's **name**, for the grant and for K5's
    /// read-back.
    TheEffectiveLoginName,
    /// K7's: the effective user's **numeric identifier**, for the runtime
    /// directory prefix.
    TheEffectiveUserNumber,
}

/// **Smoozy carries neither of K6's and K7's values, and this is the rule that
/// says so.**
///
/// Both ride `Server protocol and transfers` §1.1's machine-self-derivation
/// route as part of a **constant** command string. **A value Smoozy interpolates
/// into that string would be a user- or provider-origin identifier riding a
/// command**, which that document forbids in either phase.
pub const NEITHER_DERIVED_VALUE_IS_EVER_CARRIED_BY_SMOOZY: bool = true;

/// **The composed constant form has been run on no machine, and it never will
/// be.**
///
/// The by-name grant and the read-back are **observed**, unprivileged, on a real
/// rental; the machine-self derivation is **documented**; their composition into
/// one constant command string is neither. §16 item 32 carries it as permanently
/// unresolved. **What holds is that the failure is K5's loud one and never a
/// silent registration**: a substitution naming no login the machine holds fails
/// the grant and the read-back before the commit.
pub const THE_COMPOSED_FORM_IS_PERMANENTLY_UNRESOLVED: bool = true;

/// **K5's read-back, and what it is worth stated rather than overstated.**
///
/// It is served by the machine's own login manager, so **it catches a grant the
/// image's policy refused or dropped** — K8's per-image fact, and the silent
/// failure this clause exists for — **and defends against nothing the machine
/// itself does**; no read-back on a machine can exceed that machine's honesty.
/// **K5 reads two properties for that login by name and compares both**, and
/// both are carried here.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TheLingerReadBack {
    /// Whether the login manager reports lingering granted for that login, by
    /// name.
    pub lingering_is_granted: bool,
    /// **The state the login manager reported for that same login**, which K5
    /// reads beside the flag, **as the word the machine answered with.**
    ///
    /// **It is carried rather than turned into a second condition.** The
    /// publication behind the read says the state is one of five words and does
    /// not say which of them a granted lingering implies, so nothing here
    /// decides the registration on it, and nothing folds the word into a smaller
    /// set. **What it does decide is that an answer naming only one of the two
    /// properties is a refusal**, because half a read-back is not the read-back
    /// K5 names.
    pub the_state: String,
}

/// **K8. The permission that grants lingering is a property of the image rather
/// than of Linux.**
///
/// The shipped policy that grants it to an unprivileged caller was read on one
/// machine at one release, and both machines that posed it said the behaviour
/// rather than the reading is what they agree on, with one calling it *"a
/// per-image fact rather than a Linux one"* that should be re-read on each
/// provider. **K5's read-back is what turns that from an assumption into a
/// check**, and §16 item 2 states what breaks where the permission is refused.
pub const LINGERING_IS_A_PER_IMAGE_FACT_AND_K5_IS_THE_CHECK: bool = true;

/// **K9. The one-shot boot entry is not an equal and is not used as the
/// registration.**
///
/// It launches once and forgets — no restart, no status, no supervision — and it
/// carries its own trap: **an unescaped `%` truncates the command at that point
/// and hands the remainder to it as standard input.** The registration is not
/// refused and nothing reports a fault: what runs is the truncated prefix, and
/// its own ending is what the scheduler records. **This is an additional reason
/// to refuse it and never the reason** — the supervision grounds are.
pub const THE_ONE_SHOT_BOOT_ENTRY_IS_NEVER_THE_REGISTRATION: bool = true;

/// **The container's registration — K10, and what the hook starts is S5's keeper
/// rather than the supervisor directly.**
///
/// The hook ran on both container restarts — twelve seconds after the first
/// request and five after the second — and launched a long-lived process running
/// as the non-root login, which is the whole of what the registration has to do.
///
/// **K10a. Confirmed on a second seller and a second image, and the *nothing
/// else* is now an absence rather than a preference.** On a stock container
/// image the service manager reports itself **offline**, the first process is a
/// shell waiting on a marker file, and **the scheduler and its two commands are
/// not installed at all** — so on that image the two routes K12 and K13 rule out
/// are not merely ineffective, they are absent.
///
/// **K10b. What writes the hook is the rental request's own field, on the
/// seller's control plane, and Smoozy has no route to it.** Smoozy never rents a
/// machine and never speaks to a seller's control plane, so on a machine already
/// rented the hook is a file Smoozy places over the connection at the path the
/// seller runs — which is why K15's read-back is Smoozy's own content rather
/// than a machine-set value. **The consequence: a provider that runs no hook at
/// that path is caught by nothing**, because the content reads back exactly as
/// written (§16 item 16).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TheContainerRegistration {
    /// K10's path — the provider's, not Smoozy's.
    pub the_providers_hook_path: String,
    /// The hook's content, by value (K16).
    pub content: Vec<u8>,
    /// **K19's second obligation**: whether the login that performs K15's
    /// read-back can reach that path.
    pub the_read_back_login_can_reach_it: bool,
}

impl TheContainerRegistration {
    /// **K11 and K19 together, and K19 is a second obligation rather than a
    /// restatement of the first.**
    ///
    /// **K11**: installing the hook requires root, so the placement's
    /// registration step is an elevated step and `Server Setup` owns the
    /// elevation. From the non-root account the hook's path and the traditional
    /// local start file are both refused, and the elevation route demands a
    /// password. **The clause is answerable only as *root installs the hook, and
    /// the hook drops privilege*.** Every worker the hook's supervisor starts
    /// runs as that same non-root login (`Job files and outputs` L13).
    ///
    /// **K19**: the root step must also leave the hook readable by the login
    /// that performs K15's read-back. On the provider this arrangement was
    /// designed around **the directory the hook sits in is root-owned and
    /// unreadable to the supervisor's login**. Written as K11 alone, the
    /// placement **fails at K15 every time, before the commit** — and the
    /// failure is a permanent loop, because the message sends the operator to
    /// run the root step they have just run.
    ///
    /// **Two routes satisfy it and both were established on the machine**:
    /// making the containing directory traversable, or putting the hook at the
    /// provider's second hook path, whose directory is world-readable already.
    /// **K10 fixes which path**; where that path's directory is not readable by
    /// the supervisor's login, the root step makes it so.
    pub fn the_root_step_left_it_readable(&self) -> bool {
        self.the_read_back_login_can_reach_it
    }
}

/// **K12, K12a and K13 — the three routes a builder would reach for on a
/// container, all refused.**
///
/// **K12. The service-manager enable is never the registration step on a
/// container, and this is a prohibition rather than a preference.** Observed: it
/// *"reports success, writes the symlink, exits 0 — and the unit will never
/// run."*
///
/// **K12a. And the prohibition covers the third route, which nothing forbade
/// until it was met.** The traditional local start file **exists on the
/// container image read and nothing runs it** — neither the enable K12 names nor
/// the recurring entry K13 names, so a builder meeting both prohibitions would
/// have found it untouched and taken it.
///
/// **K13. The recurring boot entry did not fire on the container image that was
/// read.** Three separate fresh starts of the daemon gave zero, while an
/// every-minute entry in the same table fired throughout. **This is a claim about
/// that image and not about containers**: the daemon's own page documents the
/// trigger as its own start-up and names no condition a container would fail.
/// The observation and the documentation disagree and neither is discarded
/// (§16 item 20).
///
/// **On a container, the boot registration is the provider's own hook at K10's
/// path and no other file is written for the purpose**, whatever the image
/// happens to carry. **Anything that appears to be a boot route on a machine
/// with no init system is treated as absent until a read-back proves it ran.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ARefusedContainerRoute {
    /// K12.
    TheServiceManagerEnable,
    /// K13.
    TheRecurringBootEntry,
    /// K12a.
    TheTraditionalLocalStartFile,
    /// K9, which is refused on both kinds.
    TheOneShotBootEntry,
}

impl ARefusedContainerRoute {
    /// Every route this document refuses as a container's registration. **The
    /// set is closed**, and a route not on it is not thereby permitted — K10
    /// says the provider's hook is the registration and no other file is written
    /// for the purpose.
    pub const ALL: [ARefusedContainerRoute; 4] = [
        ARefusedContainerRoute::TheServiceManagerEnable,
        ARefusedContainerRoute::TheRecurringBootEntry,
        ARefusedContainerRoute::TheTraditionalLocalStartFile,
        ARefusedContainerRoute::TheOneShotBootEntry,
    ];
}

/// **K14. On a container two routes a builder would try first report success and
/// do nothing** — which is why K1's detection is a step rather than a courtesy,
/// and why K5's and K15's read-backs are what separate a registration from a
/// report of one.
///
/// **But K15 does not reach as far on a container as K5 does on a systemd
/// machine, and the difference is which side wrote the value being read.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhoWroteTheValueReadBack {
    /// K5's. The machine set it, so a machine that will not grant the
    /// registration fails the placement loudly.
    TheMachine,
    /// K15's. Smoozy wrote it, so the read-back catches the recycle case and
    /// **cannot catch a provider that runs no hook at all**.
    Smoozy,
}

/// **K15 and K20 — what the content read-back catches and what it does not.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RegistrationConfirmation {
    /// The content on the machine equals the record Smoozy wrote.
    ///
    /// **K20's second half, and it is what a reader must not overstate**: this
    /// means the bytes Smoozy wrote are still there, **and not that the
    /// registration behaves as Smoozy wrote it**. A supplementary configuration
    /// file can change what the registration does while its own bytes are
    /// untouched, and the read-back answered *content matches the record*
    /// (§16 item 18).
    ContentMatchesTheRecord,
    /// **K20's first half: it refuses loudly when the registration's own bytes
    /// change** — the recycle case, where an 82-byte provider default replaced
    /// the hook. The placement fails before the commit.
    ContentDiffers {
        bytes_on_the_machine: usize,
        bytes_recorded: usize,
    },
    /// The read could not be taken at all. **On a container this is K19's case
    /// before the root step has made the path reachable**, and it is a failure
    /// before the commit rather than a confirmation.
    TheContentCouldNotBeRead { what_the_machine_said: String },
}

/// **K15. A boot registration is confirmed by reading its own content back and
/// comparing it against the boot-registration record Smoozy wrote.**
///
/// This is `Server protocol and transfers` B4's *content, never presence* applied
/// to the half of package inspection that is this document's. **K15 is performed
/// by the non-root placement on both machine kinds**, which is what makes K19 an
/// obligation on the container's root step rather than an afterthought.
pub fn confirm(
    record: &BootRegistrationRecord,
    what_the_machine_holds: Result<&[u8], String>,
) -> RegistrationConfirmation {
    match what_the_machine_holds {
        Err(what_the_machine_said) => {
            RegistrationConfirmation::TheContentCouldNotBeRead {
                what_the_machine_said,
            }
        }
        Ok(bytes) if bytes == record.content.as_slice() => {
            RegistrationConfirmation::ContentMatchesTheRecord
        }
        Ok(bytes) => RegistrationConfirmation::ContentDiffers {
            bytes_on_the_machine: bytes.len(),
            bytes_recorded: record.content.len(),
        },
    }
}

/// Reading a registration back, behind a trait: **taking the read is the acting
/// half's, and what it means is [`confirm`]'s.**
pub trait ReadsTheRegistrationBack {
    /// The bytes at the registration's own path, or what the machine said.
    ///
    /// **Never a presence answer.** A trait returning *it is there* would be the
    /// exact reading the recycle observation forbids.
    fn the_content_at(&self, path: &str) -> Result<Vec<u8>, String>;
}

/// Reading the linger flag back, behind a trait — K5.
pub trait ReadsTheLingerFlagBack {
    /// **By name**, and the name is the machine's own (K6). The trait takes no
    /// name argument for that reason: a name passed in from Smoozy is the thing
    /// K6 forbids.
    fn the_linger_read_back(&self) -> Result<TheLingerReadBack, String>;
}

/// **What the registration step left behind, and both endings leave K16's
/// record.**
///
/// **The two machine kinds differ in who installs the registration's own
/// content and in nothing else.** The record is Smoozy's own, it sits in the
/// package area, and the login that performs K15's read-back is the one that
/// writes it — which is the non-root placement on both kinds (K19).
///
/// **That is `Rented-server runtime` K16a, and both endings below are it.** The
/// record is the non-root placement's on both machine kinds, and it holds on a
/// container even though the registration itself is the root step's: K11 makes
/// installing the hook the elevated step's, which invites reading the record as
/// that step's too, and it is not. **A container placement that wrote no record
/// would leave the recycle case caught by nothing at all**, on the machine kind
/// the recycle was actually observed on, K15 having nothing to compare against.
/// **It adds no obligation to the root step**, whose two are K11's and K19's.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhatTheRegistrationStepLeft {
    /// **A systemd machine.** The record is in the package area and the unit is
    /// at the path the record names. K4's second act and K6's grant follow.
    TheRecordAndTheRegistrationItself,
    /// **A container.** The record is in the package area, holding the hook's
    /// content by value, and **installing the hook is the elevated step
    /// `Server Setup` owns** (K11), which must also leave it readable to the
    /// login that performs K15's read-back (K19).
    ///
    /// **It is an ending rather than a failure**, and that is what makes the
    /// record reachable on this kind at all: the content the root step installs
    /// and the content the record holds are the one value, composed once and
    /// written down here before either is used.
    TheRecordAndTheHookLeftToTheRootStep,
    /// **A container, and the root step ran inside the placement.** The record
    /// is in the package area and the hook is at the provider's path, installed
    /// by K11's root step and left readable by K19's. Added 2026-09-21 at step
    /// 10c, when the desktop's placement began performing that step itself;
    /// **the machine's own placement still ends at the variant above**, having no
    /// elevation of its own.
    TheRecordAndTheHookTheRootStepInstalled,
}

/// Placing the registration, behind a trait — **the three acts K4, K6, K7 and K11
/// name, with K16's record inside the first of them, and the rules above are
/// what each is for.**
///
/// **No method here takes a login name or a numeric identifier**, because K6 and
/// K7 forbid Smoozy carrying either: both ride a constant command string and are
/// derived on the machine.
pub trait PlacesTheRegistration {
    /// **The registration step, and it is one act on both machine kinds:
    /// K16's record, and then the registration's own content.**
    ///
    /// **The record is written first, and that is `Rented-server runtime`
    /// K16b** — the record written durably before the registration it
    /// describes, in both directions, so a rollback that restores the pair
    /// takes the same order rather than one of its own. **The order is D1's
    /// *intent before acting*** — `State, identity and recovery` D1, which G3
    /// already applies to every phase of this transaction. **What it buys is
    /// that the machine never carries Smoozy's own boot bytes with nothing
    /// recording them.** A
    /// step interrupted between the two leaves a record whose content is not on
    /// the machine, which is exactly what K15's read-back is for and which fails
    /// the placement before the commit; the other order would leave those bytes
    /// live on the machine with **no record at all**, which
    /// `Server protocol and transfers` B4's package inspection reads as the
    /// never-held answer on a machine that is about to start Smoozy.
    ///
    /// **Nothing of the previous registration is lost by writing over the
    /// record**, because G4 has already carried the previous boot-registration
    /// content by value into the phase record, in the same durable write that
    /// recorded this attempt — which is what G14 restores from in G9's case.
    /// **K16b binds the rollback direction too and nothing here performs a
    /// rollback**: this crate composes that decision and no performer carries
    /// it out, so the clause completing G14 with the record's own restore is
    /// deliberately not built and is recorded at [`crate::arriving_later`],
    /// which is where the marker sits.
    ///
    /// **K4's first act on a systemd machine, and K11's on a container — and
    /// the record is written on both, which is `Rented-server runtime` K16a.**
    /// On a container the hook itself is the elevated step `Server Setup` owns,
    /// which must also leave the path readable to the login that performs K15's
    /// read-back (K19) — so what comes back there is
    /// [`WhatTheRegistrationStepLeft::TheRecordAndTheHookLeftToTheRootStep`] and
    /// never a quiet success. **The record is the non-root placement's on both
    /// kinds all the same**, because K15's read-back is, and G31a makes all
    /// three durable records the placement's own.
    fn place_the_registration(
        &self,
        record: &BootRegistrationRecord,
    ) -> Result<WhatTheRegistrationStepLeft, String>;

    /// **K4's second act**: enabling the unit. Nothing on a container calls this
    /// — K12 forbids the enable there outright.
    ///
    /// **K7 as corrected on 2026-09-21 orders it last**: after the grant, K5's
    /// read-back and the user's service manager answering, because on a login
    /// with no session the grant is what starts the manager the enable needs.
    /// **No performer on the machine's side calls this yet**; the desktop's
    /// placement keeps the order at `Server protocol and transfers`'
    /// `the_user_level_registration`.
    fn enable_the_unit(&self) -> Result<(), String>;

    /// **K6's grant**, by name, and the name is the machine's own.
    fn grant_lingering(&self) -> Result<(), String>;
}

/// **K17. Nothing budgets a fixed wait for a machine to come back.**
///
/// Restart cost was measured at **9, 11 and 12 seconds** on one machine, **21,
/// 22 and 28** on a second, **85 to 90** on a third and **49 and 55** on a
/// fourth, and a container recycle returned in about 90 seconds against 15 to 20
/// for a reboot. **The same rule holds for a machine coming up for the first
/// time, and there the spread is an order of magnitude** — requested to
/// reachable was about 40 seconds on one seller, about 90 on another and about
/// seven minutes on the third. **So anything that waits a fixed period for a
/// machine to come up is wrong on at least one seller in three.**
///
/// A caller waits on its own deadline (`Server protocol and transfers` C16, C17)
/// and resumes on that document's C14 gate, **never on a timer that assumed a
/// number**. The value is `()` because there is no number to hold: this crate
/// declares no wait constant anywhere, and that absence is the clause.
pub const NOTHING_HERE_BUDGETS_A_FIXED_WAIT: () = ();

#[cfg(test)]
mod tests {
    use super::*;

    fn a_record() -> BootRegistrationRecord {
        BootRegistrationRecord {
            kind: MachineKind::Container,
            at_path: "/root/onstart.sh".into(),
            content: b"smoozy's own hook".to_vec(),
        }
    }

    #[test]
    fn matching_content_confirms() {
        let record = a_record();
        assert_eq!(
            confirm(&record, Ok(record.content.as_slice())),
            RegistrationConfirmation::ContentMatchesTheRecord
        );
    }

    /// The recycle: not deleted, replaced.
    #[test]
    fn a_replaced_hook_refuses_loudly_and_carries_both_counts() {
        let record = a_record();
        let providers_default = vec![b'x'; 82];
        assert_eq!(
            confirm(&record, Ok(&providers_default)),
            RegistrationConfirmation::ContentDiffers {
                bytes_on_the_machine: 82,
                bytes_recorded: record.content.len(),
            }
        );
    }

    /// A hook of the same length but different bytes is still a different hook.
    /// **Nothing here compares sizes.**
    #[test]
    fn equal_length_and_different_bytes_is_a_difference() {
        let record = a_record();
        let same_length: Vec<u8> = record.content.iter().map(|_| b'z').collect();
        assert!(matches!(
            confirm(&record, Ok(&same_length)),
            RegistrationConfirmation::ContentDiffers { .. }
        ));
    }

    #[test]
    fn an_unreadable_path_is_a_failure_before_the_commit_and_not_a_confirmation() {
        let record = a_record();
        let refused = confirm(
            &record,
            Err("[Errno 13] Permission denied".into()),
        );
        assert!(matches!(
            refused,
            RegistrationConfirmation::TheContentCouldNotBeRead { .. }
        ));
        assert_ne!(refused, RegistrationConfirmation::ContentMatchesTheRecord);
    }

    fn a_read_back(lingering_is_granted: bool) -> TheLingerReadBack {
        TheLingerReadBack {
            lingering_is_granted,
            the_state: if lingering_is_granted {
                "lingering".to_string()
            } else {
                "offline".to_string()
            },
        }
    }

    #[test]
    fn a_systemd_registration_needs_the_enable_k18s_directive_and_k5s_read_back() {
        let mut registration = TheSystemdRegistration {
            unit_content: b"[Service]".to_vec(),
            the_kill_behaviour_is_confined_to_the_main_process: true,
            enabled: true,
            the_linger_read_back: Some(a_read_back(true)),
        };
        assert!(registration.is_complete());
        registration.the_kill_behaviour_is_confined_to_the_main_process = false;
        assert!(!registration.is_complete());
        registration.the_kill_behaviour_is_confined_to_the_main_process = true;
        registration.enabled = false;
        assert!(!registration.is_complete());
    }

    /// **K5's own case**: with the unit written and enabled and K18's directive
    /// in place, a read-back showing lingering not granted leaves the
    /// registration incomplete — so the placement fails before the commit rather
    /// than committing on an image that refused the permission.
    #[test]
    fn a_read_back_showing_lingering_not_granted_makes_the_registration_incomplete() {
        let mut registration = TheSystemdRegistration {
            unit_content: b"[Service]".to_vec(),
            the_kill_behaviour_is_confined_to_the_main_process: true,
            enabled: true,
            the_linger_read_back: Some(a_read_back(false)),
        };
        assert!(!registration.is_complete());
        // And a read-back that was never taken is not a granted one either.
        registration.the_linger_read_back = None;
        assert!(!registration.is_complete());
        registration.the_linger_read_back = Some(a_read_back(true));
        assert!(registration.is_complete());
    }

    #[test]
    fn a_container_placement_is_incomplete_until_the_root_step_left_the_path_readable() {
        let registration = TheContainerRegistration {
            the_providers_hook_path: "/root/onstart.sh".into(),
            content: b"smoozy's own hook".to_vec(),
            the_read_back_login_can_reach_it: false,
        };
        assert!(!registration.the_root_step_left_it_readable());
    }

    #[test]
    fn every_route_a_builder_would_reach_for_is_named_and_refused() {
        // **The set is closed by the compiler rather than by a count.** The
        // match is total and carries no default arm, so a route added to the
        // enum has none, the crate stops compiling, and whoever added it is sent
        // to this list — where the length assertion this replaced could never
        // have been false.
        for (at, route) in ARefusedContainerRoute::ALL.iter().enumerate() {
            let where_the_list_puts_it = match route {
                ARefusedContainerRoute::TheServiceManagerEnable => 0,
                ARefusedContainerRoute::TheRecurringBootEntry => 1,
                ARefusedContainerRoute::TheTraditionalLocalStartFile => 2,
                ARefusedContainerRoute::TheOneShotBootEntry => 3,
            };
            assert_eq!(at, where_the_list_puts_it, "{route:?} is not where the list puts it");
        }
    }
}
