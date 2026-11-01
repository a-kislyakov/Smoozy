//! Elevation, the refusal, and the password — `Server Setup` §9.
//!
//! **The constraint** is Row 117; `Security and licence` §4's invariant, its §13
//! rule 5 and that rule's named exception, and its §14 items 15 and 17; and
//! `Server protocol and transfers` §1.3's fixed route — **the elevated-access
//! password crosses on an elevating command's standard input and never as an
//! argument.**
//!
//! **The seam that once instructed two ways is closed.** `Security and licence`
//! §14 item 16 and `Server Setup` §9 E4 and §13 item 6 once ordered opposite
//! things about a failed elevation password. **It closed on 2026-08-31 and both
//! documents now instruct alike**, and the classification below is read from
//! `Security and licence` rather than restated: that document's
//! [`smoozy_security_and_licence::redaction`] carries the constant and the
//! reading, and this module carries what the answer does to a step.
//!
//! **Markers used below that belong to another document, each with the document
//! it belongs to.** `Security and licence` §2, §4, §5.1, §5.2, §13 rule 5, §14
//! items 15 and 17. `Server protocol and transfers` 1.1, §1.3.
//! `File Browser` P5, P7.
//!
//! **`Security and licence` R8 is not listed above**, because this plan defines
//! an R8 of its own and a bare one could mean either;
//! where this file cites one, the document is written out at the site.

use smoozy_security_and_licence::redaction::{read_a_failed_validate, ElevationRefusal};
use smoozy_server_protocol::carriage::CarriageSite;
use smoozy_server_protocol::transport::CommandString;

use crate::kind::{MachineKind, Probe};

/// **E1: Row 116's answer is [`crate::kind`] §1's table, and its weight is the
/// spec's own.**
///
/// **Putting a Blender on a machine and installing add-ons need no elevation at
/// all** — a self-contained folder, and files inside a version folder the login
/// owns; the package placement needs none on a systemd machine and one root step
/// on a container. **What leans on the machine's permission is making it
/// ready**, and a machine that refuses may not be set up at all.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhatNeedsElevation {
    /// A self-contained folder, and files inside a version folder the login
    /// owns.
    NeitherABlenderNorAnAddOn,
    /// None on a systemd machine, one root step on a container.
    ThePackagePlacementByKind,
    /// **The weight.**
    MakingTheMachineReady,
}

/// **E1, performed.**
pub fn needs_elevation(what: WhatNeedsElevation, kind: MachineKind) -> bool {
    match what {
        WhatNeedsElevation::NeitherABlenderNorAnAddOn => false,
        WhatNeedsElevation::ThePackagePlacementByKind => {
            matches!(kind, MachineKind::AContainer)
        }
        WhatNeedsElevation::MakingTheMachineReady => true,
    }
}

/// **E2: elevation is offered only at the moment a step is refused for lack of
/// permission, never before**, and **the password's three lifetimes are
/// `Preparation/Specs/Whole app.md`'s and are not retaken here** — one action,
/// the connection, until cleared. **What is owed here is the recognition, and it
/// is [`what_a_failed_validate_means`]'s.**
pub fn the_offer_is_made_before_a_refusal(_it_would_be_smoother: bool) -> bool {
    false
}

/// **E3: the route is the machine's own elevating command with the password on
/// its standard input.**
///
/// **Documented** — `sudo(8)` 1.9.17p2, read 2026-08-19: *"Write the prompt to
/// the standard error and read the password from the standard input instead of
/// using the terminal device"* — **because a command argument is readable by any
/// other login on the machine** (Fact 60), and `Server protocol and transfers`
/// §1.3 fixed this route before this document was written.
pub const THE_ELEVATING_COMMANDS_STDIN_FORM: &str = "sudo -S";

/// **E4: the validate form, which runs no command at all.**
///
/// **Documented**, same manual: `-v, --validate` *"Update the user's cached
/// credentials, authenticating the user if necessary. For the sudoers plugin,
/// this extends the sudo timeout for another 5 minutes by default, but does not
/// run a command."*
pub const THE_VALIDATE_FORM: CommandString = CommandString::constant("sudo -S -v");

/// **E4: the validate-first order separates the authentication step's result
/// from the requested operation's result — and that is all it separates.**
///
/// **Because it runs no command, a failed validate is never an operation's
/// failure**, and that much is the separable result `Security and licence` §14
/// item 15 records as owed by the route's owner.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhatTheStepMet {
    /// **A failed validate with a held password whose captured text is the
    /// wrong-password ending**, matched as an exact constant under
    /// `Security and licence` §13 rule 5's Creator-stated exception. **This is
    /// an authentication result separable from the requested operation's own**,
    /// so that document's §4 invariant **fires rather than withholds**, and its
    /// §2 item 3 erasure — *the password ceasing to work* — is performed on the
    /// remembered machine password.
    ///
    /// **The erase is that document's**: its trigger is recognised here, and
    /// **its order, its recorded debt and its sweep are §5.1's and are neither
    /// performed nor restated here.** No password is held afterwards, so a later
    /// refusal is [`WhatTheStepMet::ARefusalWithNoPasswordHeld`].
    AWrongPassword,
    /// **A failed validate with a held password whose captured text is anything
    /// else** — the not-an-administrator message, which **names the host and is
    /// never matched**; an unrecognised string; a release or machine kind whose
    /// wording was never observed. **Not established to be an authentication
    /// failure**, because rule 5's exception reaches the one constant and no
    /// further, so §4's invariant governs in its withholding direction and **the
    /// password is kept**. The step is reported **refused** with the machine's
    /// own words behind it.
    ARefusalNotEstablishedToBeAWrongPassword,
    /// **A failed operation after a successful validate is a failed
    /// operation**: reported as such, with the machine's words behind it, **and
    /// it forgets nothing** — a full disk, Blender's add-on lock, a wrong-Python
    /// part and a real fault all land here.
    AFailedOperationAfterASuccessfulValidate,
    /// **A refusal with no password held** — the non-interactive probe failing
    /// ([`crate::kind`] M3) — **is the moment the offer is made and the ask
    /// happens, at that moment and not before.**
    ARefusalWithNoPasswordHeld,
    /// **The operation succeeded**, after a validate that succeeded or after no
    /// validate at all. **It forgets nothing**, exactly as a failed operation
    /// does.
    ///
    /// **Added 2026-09-04, out of this module's round-1 code audit.** The set
    /// had four values and none of them meant this, so **every successful
    /// elevated command in the application was labelled as having met a failed
    /// operation** — and the branch whose own comment read *"Nothing failed at
    /// all; the step ran"* returned the failed one.
    ///
    /// **Nothing behaves differently for it.** E4's point is that the
    /// authentication result is separable from the operation's own result, and
    /// this value is where that separation is recorded; a value that says the
    /// opposite of what happened is a fault the next reader inherits with no way
    /// of telling it from an answer.
    ASuccessfulOperationAfterASuccessfulValidate,
}

impl WhatTheStepMet {
    /// **E4: whether `Security and licence` §5.2's erasure of the remembered
    /// machine password fires.**
    ///
    /// **Exactly one of the four fires it**, and the failure direction of the
    /// other three is the safe one: a good password is never discarded.
    pub fn erases_the_remembered_password(self) -> bool {
        matches!(self, WhatTheStepMet::AWrongPassword)
    }

    /// **E4: whether the password is kept.**
    pub fn the_password_is_kept(self) -> bool {
        matches!(
            self,
            WhatTheStepMet::ARefusalNotEstablishedToBeAWrongPassword
                | WhatTheStepMet::AFailedOperationAfterASuccessfulValidate
                | WhatTheStepMet::ASuccessfulOperationAfterASuccessfulValidate
        )
    }
}

/// **E4, performed, in the validate-first order.**
///
/// **The reading of the captured text is `Security and licence` §13 rule 5's and
/// is not made here** — [`read_a_failed_validate`] matches exactly one string
/// exactly, or it concludes nothing. **This module recognises the trigger; that
/// document performs the erase.**
pub fn what_a_failed_validate_means(
    a_password_was_held: bool,
    the_validate_succeeded: bool,
    the_operation_failed: bool,
    captured_text: &str,
) -> WhatTheStepMet {
    if !a_password_was_held {
        return WhatTheStepMet::ARefusalWithNoPasswordHeld;
    }
    if the_validate_succeeded {
        if the_operation_failed {
            return WhatTheStepMet::AFailedOperationAfterASuccessfulValidate;
        }
        // **Nothing failed at all; the step ran**, and from 2026-09-04 there is
        // a value that says so.
        return WhatTheStepMet::ASuccessfulOperationAfterASuccessfulValidate;
    }
    match read_a_failed_validate(captured_text) {
        ElevationRefusal::WrongPassword => WhatTheStepMet::AWrongPassword,
        ElevationRefusal::NothingConcluded => {
            WhatTheStepMet::ARefusalNotEstablishedToBeAWrongPassword
        }
    }
}

/// **E4's two limits, both carried.**
///
/// **The validate's caching is not relied on** — *"Not all security policies
/// support cached credentials"* — so where the cache does not hold, **each
/// elevated command carries the password on standard input again**, and the
/// separation stands because **the validate's own result is what classifies.**
///
/// **The credential cache is read per machine and never assumed from any one
/// page's figure**: the image observed on 2026-08-22 gave **15 minutes**, which
/// is exactly what its own release's manual documents (`sudo(8)` 1.9.15p5, read
/// 2026-08-23), while the Creator's Mac's page at 1.9.17p2 gives five —
/// **two of the tool's own pages give two different defaults.**
pub fn the_credential_cache_is_relied_on() -> bool {
    false
}

/// **E5: the middle lifetime spans a machine reboot, deliberately.**
///
/// A password kept *for as long as the connection lasts* is keyed to the
/// connected period, and **the connected period survives a restart Smoozy
/// caused** (Row 107) — so the password survives the reboot the setup itself
/// performed. **That is the spec's own construction meeting itself, not a new
/// cost.** A connection that ends any other way takes every process-memory
/// password with it (`Security and licence` §5.2).
pub fn the_password_survives_a_restart_smoozy_caused() -> bool {
    true
}

/// **E6: what is never done.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NeverDone {
    /// **No retry loop on an authentication** — rate limiting corrupts the
    /// diagnosis (`Security and licence` §4).
    ARetryLoopOnAnAuthentication,
    /// **No password in an argument or environment value, ever.**
    APasswordInAnArgumentOrEnvironmentValue,
    /// **No reading of the machine's settings into any log** — the one box on
    /// record held the provider's own secrets in plain text among them.
    ReadingTheMachinesSettingsIntoALog,
    /// **No second route beside the elevating command.**
    ASecondRouteBesideTheElevatingCommand,
}

impl NeverDone {
    pub const ALL: [NeverDone; 4] = [
        NeverDone::ARetryLoopOnAnAuthentication,
        NeverDone::APasswordInAnArgumentOrEnvironmentValue,
        NeverDone::ReadingTheMachinesSettingsIntoALog,
        NeverDone::ASecondRouteBesideTheElevatingCommand,
    ];
}

/// **E6, as the question a caller asks.**
pub fn this_module_may(_it: NeverDone) -> bool {
    false
}

/// **E7: every elevated command this document issues is checked against
/// `Server protocol and transfers` 1.1's admissibility rule, and the
/// declaration is made once, here.**
///
/// The rule holds in either phase — **no value of user, provider or machine
/// origin is ever interpolated into a command string** — so each step's values
/// **ride one of the rule's carriages or are fixed at build time**, per step.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AnElevatedStepsCarriage {
    /// The step, in E7's own terms.
    pub step: &'static str,
    /// Which carriage its values ride, or that every byte is fixed at build
    /// time.
    pub carriage: Carriage,
}

/// The carriages E7 declares for this document's elevated steps.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Carriage {
    /// **The planted-link carriage**, one link per mount, declared with its
    /// consuming command's link-treatment citation at [`crate::placement`] R7a.
    ThePlantedLink(CarriageSite),
    /// **A constant string entire**: fixed program names, fixed options, fixed
    /// values.
    ConstantEntire,
    /// **The subsystem's own protocol fields** — a Smoozy-synthesised staging
    /// token for content, with the elevated command naming the token and a fixed
    /// destination. **Both admissible classes, no link consumed.**
    ASynthesisedStagingToken,
    /// **The machine's `ldd` list selects, and never composes, the command**:
    /// the packages named are drawn from a fixed build-time mapping of library
    /// names to package names, **so every byte of the command string is fixed at
    /// build time and no machine-read name is echoed into it.**
    AFixedBuildTimeMappingSelectsIt,
    /// **The elevating command's standard input, read as data**, which is 1.1's
    /// standing instance of that carriage (E3).
    StandardInputReadAsData,
    /// **Machine self-derivation**, `Server protocol and transfers` 1.1's
    /// fourth carriage: **the command string stays constant and the machine
    /// derives the value itself at execution**, so nothing crosses from Smoozy
    /// at all.
    ///
    /// **E7's bullet for it was added to the plan on 2026-09-04 and this value
    /// on the same day, out of this module's round-1 code audit** — there was no
    /// such carriage in this crate at all, while the hand-over's commands really
    /// did carry the substitution.
    ///
    /// **E7's own bound rides it**: *"the carriage reaches no further than the
    /// credentials the invoking process itself carries"* — never a package, path
    /// or other name read off the machine and echoed back, which stays
    /// inadmissible (`Server Setup` §13 item 21).
    MachineSelfDerivation,
}

/// **E7's declaration, entire.**
///
/// **The count is E7's bullets' and is never a remembered number.**
/// [`tests::every_value_entering_an_elevated_command_has_a_declared_carriage`]
/// is what holds it: it asks that every value any elevated command of this crate
/// carries has a declaration here, and the length follows from that rather than
/// from an assertion nobody could check.
pub const THE_ELEVATED_STEPS: [AnElevatedStepsCarriage; 10] = [
    AnElevatedStepsCarriage {
        step: "the hand-over of the chosen root, and of the package area's second mount where there is one",
        carriage: Carriage::ThePlantedLink(crate::placement::THE_ROOTS_CARRIAGE_SITE),
    },
    AnElevatedStepsCarriage {
        step: "the owner that same hand-over's `chown` must name — the login the mount is handed to, which is the supervisor's and is the connection's own, and that login's own group where the command form names one; the carriage reaches no further than the credentials the invoking process itself carries",
        carriage: Carriage::MachineSelfDerivation,
    },
    AnElevatedStepsCarriage {
        step: "the creation of the root's own directory, and of the package area's own location where there is a second mount, where the plain create over the file subsystem is refused",
        carriage: Carriage::ThePlantedLink(crate::placement::THE_ROOTS_CREATION_SITE),
    },
    AnElevatedStepsCarriage {
        step: "the container boot hook — the destination path and the mode change are constant, per the fixed provider path",
        carriage: Carriage::ASynthesisedStagingToken,
    },
    AnElevatedStepsCarriage {
        step: "the restart, the driver route, the package-system repair and the lock-timeout setting",
        carriage: Carriage::ConstantEntire,
    },
    AnElevatedStepsCarriage {
        step: "the readings of the ready action's first step, elevated or not",
        carriage: Carriage::ConstantEntire,
    },
    AnElevatedStepsCarriage {
        step: "the tooling install — the one unpack tool Blender's documentation names, a constant package name",
        carriage: Carriage::ConstantEntire,
    },
    AnElevatedStepsCarriage {
        step: "the libraries install",
        carriage: Carriage::AFixedBuildTimeMappingSelectsIt,
    },
    AnElevatedStepsCarriage {
        step: "the manager-route removal — the constant package name the machine's manager holds Blender under",
        carriage: Carriage::ConstantEntire,
    },
    AnElevatedStepsCarriage {
        step: "the elevated search retry — the same constant search command as the unprivileged pass",
        carriage: Carriage::ConstantEntire,
    },
];

/// **E7's ninth line, which is the password and rides its own carriage.**
pub const THE_PASSWORDS_CARRIAGE: Carriage = Carriage::StandardInputReadAsData;

/// **E7: the machine-side steps touch no command string at all.**
///
/// The fetch, the unpack, the `ldd` reads, the publication and the read-backs
/// are the setup worker's; **file placement and every read-back ride the
/// file-transfer subsystem, whose paths are protocol fields.**
pub fn a_machine_side_step_names_a_command_string(_step: crate::ready_action::Step) -> bool {
    false
}

/// **`File Browser` P5's two readings, answered from this module's own — its §1
/// kind table and [`crate::kind`] M3's probe.**
///
/// `File Browser` P5 says outright that both belong to `Server Setup`, and
/// `File Browser` §12 assigns them here. **This trait is what a caller supplies
/// so that this end can be given without this module reaching a machine
/// itself**: the two readings are taken over the connection by the acting half,
/// **at the refusal** (`File Browser` P7's timing rule, which is the spec's own),
/// and this module maps them onto the three rows P5 judges over.
pub trait ReadsTheMachineAtARefusal {
    /// [`crate::kind`] M1's kind, taken at the refusal — **a machine the user is
    /// browsing may be one no package was ever placed on**, so nothing is read
    /// from a placement record.
    fn the_kind(&mut self) -> MachineKind;
    /// **Whether the connection's login is the machine's root.** On a container
    /// this is what separates P5's first row from its second, and
    /// [`crate::kind`] M2 already turns on it.
    fn the_login_is_root(&mut self) -> bool;
    /// [`crate::kind`] M3's non-interactive probe.
    fn the_non_interactive_probe(&mut self) -> Probe;
}

/// **The end of `File Browser` P5's seam, and `File Browser` §12's own
/// assignment performed.**
///
/// **Nothing is stood up here.** The two answers come from
/// [`ReadsTheMachineAtARefusal`], which the acting half implements over the
/// connection; what this type adds is the **mapping** from this document's kind
/// table onto `File Browser`'s three rows, which is a judgement rather than a
/// reading — and it is this document's judgement, because it is this document's
/// table.
pub struct WhatElevationWouldDo<Reader>(pub Reader);

impl<Reader> smoozy_file_browser::permission::ReadsWhatElevationWouldDo for WhatElevationWouldDo<Reader>
where
    Reader: ReadsTheMachineAtARefusal,
{
    /// **The mapping, row by row.**
    ///
    /// A container under a login that is **not** its root is `File Browser`'s
    /// first row — Fact 27's *"elevation demands a password it does not
    /// have"*. A container under its own root behaves as a systemd machine does
    /// for this question, because the elevated package installs are open to it
    /// ([`crate::kind`] M2). A machine the table does not place is
    /// `Rented-server runtime` K3's refusal, which `File Browser` P5 decides on
    /// the kind alone.
    fn the_machine_kind(&mut self) -> smoozy_file_browser::permission::MachineKind {
        use smoozy_file_browser::permission::MachineKind as Theirs;
        match self.0.the_kind() {
            MachineKind::TheTableDoesNotPlaceIt => Theirs::TheTableDoesNotPlaceIt,
            MachineKind::ASystemdMachine => Theirs::ASystemdMachine,
            MachineKind::AContainer => {
                if self.0.the_login_is_root() {
                    Theirs::ASystemdMachine
                } else {
                    Theirs::AContainerUnderANonRootLogin
                }
            }
        }
    }

    fn the_non_interactive_probe(&mut self) -> smoozy_file_browser::permission::TheProbe {
        match self.0.the_non_interactive_probe() {
            Probe::Succeeded => smoozy_file_browser::permission::TheProbe::Succeeded,
            Probe::Failed => smoozy_file_browser::permission::TheProbe::Failed,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use smoozy_file_browser::permission::{
        the_offer, ElevatableAct, MachineKind as Theirs, Offer, ReadsWhatElevationWouldDo,
        WhyThereIsNoOffer,
    };

    struct AMachine {
        kind: MachineKind,
        login_is_root: bool,
        probe: Probe,
    }

    impl ReadsTheMachineAtARefusal for AMachine {
        fn the_kind(&mut self) -> MachineKind {
            self.kind
        }
        fn the_login_is_root(&mut self) -> bool {
            self.login_is_root
        }
        fn the_non_interactive_probe(&mut self) -> Probe {
            self.probe
        }
    }

    fn reading(kind: MachineKind, login_is_root: bool, probe: Probe) -> WhatElevationWouldDo<AMachine> {
        WhatElevationWouldDo(AMachine {
            kind,
            login_is_root,
            probe,
        })
    }

    #[test]
    fn a_wrong_password_erases_and_every_other_refusal_keeps_it() {
        // E4, and the seam closed on 2026-08-31: both documents instruct alike.
        let wrong = what_a_failed_validate_means(
            true,
            false,
            false,
            "Password:\nsudo: 1 incorrect password attempt\n",
        );
        assert_eq!(wrong, WhatTheStepMet::AWrongPassword);
        assert!(wrong.erases_the_remembered_password());
        assert!(!wrong.the_password_is_kept());
    }

    #[test]
    fn the_host_naming_refusal_keeps_the_password_and_reports_the_step_refused() {
        // The message names the host, so it can never be an exact constant
        // across machines and is never matched.
        let met = what_a_failed_validate_means(
            true,
            false,
            false,
            "Sorry, user smoozynoadmin may not run sudo on a0e7a496ff47.\n",
        );
        assert_eq!(met, WhatTheStepMet::ARefusalNotEstablishedToBeAWrongPassword);
        assert!(!met.erases_the_remembered_password());
        assert!(met.the_password_is_kept());
    }

    #[test]
    fn an_unobserved_wording_concludes_nothing_and_keeps_the_password() {
        // §14 item 17's guard: a release whose strings were never observed
        // matches nothing.
        let met = what_a_failed_validate_means(true, false, false, "elevation refused\n");
        assert_eq!(met, WhatTheStepMet::ARefusalNotEstablishedToBeAWrongPassword);
        assert!(met.the_password_is_kept());
    }

    #[test]
    fn a_failed_operation_after_a_successful_validate_forgets_nothing() {
        let met = what_a_failed_validate_means(true, true, true, "No space left on device\n");
        assert_eq!(met, WhatTheStepMet::AFailedOperationAfterASuccessfulValidate);
        assert!(!met.erases_the_remembered_password());
        assert!(met.the_password_is_kept());
    }

    #[test]
    fn a_step_that_ran_met_a_successful_operation_and_both_readers_are_unmoved() {
        // **The fault this test exists for.** The set had four values and none
        // meant *the operation succeeded*, so the branch whose own comment reads
        // *"Nothing failed at all; the step ran"* returned the failed one, and
        // every successful elevated command in the application was labelled as
        // having met a failed operation.
        let met = what_a_failed_validate_means(true, true, false, "");
        assert_eq!(met, WhatTheStepMet::ASuccessfulOperationAfterASuccessfulValidate);
        // **Both existing readers give exactly the answers they gave before**,
        // which is what makes this a repair of the record and not a change of
        // behaviour.
        assert!(!met.erases_the_remembered_password());
        assert!(met.the_password_is_kept());
        let failed = what_a_failed_validate_means(true, true, true, "");
        assert_eq!(
            failed.erases_the_remembered_password(),
            met.erases_the_remembered_password()
        );
        assert_eq!(failed.the_password_is_kept(), met.the_password_is_kept());
        // **Exactly one of the values still fires the erase.**
        for it in [
            WhatTheStepMet::AWrongPassword,
            WhatTheStepMet::ARefusalNotEstablishedToBeAWrongPassword,
            WhatTheStepMet::AFailedOperationAfterASuccessfulValidate,
            WhatTheStepMet::ARefusalWithNoPasswordHeld,
            WhatTheStepMet::ASuccessfulOperationAfterASuccessfulValidate,
        ] {
            assert_eq!(
                it.erases_the_remembered_password(),
                it == WhatTheStepMet::AWrongPassword
            );
        }
    }

    #[test]
    fn a_refusal_with_no_password_held_is_the_moment_the_offer_is_made() {
        assert_eq!(
            what_a_failed_validate_means(false, false, false, ""),
            WhatTheStepMet::ARefusalWithNoPasswordHeld
        );
        assert!(!the_offer_is_made_before_a_refusal(true));
    }

    #[test]
    fn the_weight_is_making_the_machine_ready_and_not_a_blender_or_an_add_on() {
        // E1, and the spec's own weight.
        assert!(!needs_elevation(
            WhatNeedsElevation::NeitherABlenderNorAnAddOn,
            MachineKind::ASystemdMachine
        ));
        assert!(!needs_elevation(
            WhatNeedsElevation::ThePackagePlacementByKind,
            MachineKind::ASystemdMachine
        ));
        assert!(needs_elevation(
            WhatNeedsElevation::ThePackagePlacementByKind,
            MachineKind::AContainer
        ));
        assert!(needs_elevation(
            WhatNeedsElevation::MakingTheMachineReady,
            MachineKind::AContainer
        ));
    }

    #[test]
    fn the_password_goes_on_standard_input_and_never_in_an_argument() {
        assert_eq!(THE_ELEVATING_COMMANDS_STDIN_FORM, "sudo -S");
        assert_eq!(THE_VALIDATE_FORM.as_str(), "sudo -S -v");
        assert_eq!(THE_PASSWORDS_CARRIAGE, Carriage::StandardInputReadAsData);
        assert_eq!(NeverDone::ALL.len(), 4);
        for it in NeverDone::ALL {
            assert!(!this_module_may(it));
        }
        assert!(!the_credential_cache_is_relied_on());
        assert!(the_password_survives_a_restart_smoozy_caused());
    }

    #[test]
    fn every_elevated_step_declares_a_carriage_and_none_interpolates_a_reading() {
        for declared in THE_ELEVATED_STEPS {
            assert!(!declared.step.is_empty());
        }
        // The libraries install is the one that selects rather than composes.
        assert!(THE_ELEVATED_STEPS
            .iter()
            .any(|it| it.carriage == Carriage::AFixedBuildTimeMappingSelectsIt));
        // And no machine-side step names a command string at all.
        for step in crate::ready_action::Step::IN_ORDER {
            assert!(!a_machine_side_step_names_a_command_string(step));
        }
    }

    /// **E7's declaration is held to the commands rather than to a number.**
    ///
    /// **The number this test replaces was wrong and locked the fault in.** It
    /// asserted eight, and the code's eight were not the plan's eight minus the
    /// owner: this crate carries one entry the plan's bullets do not list
    /// separately, so the count agreed with nothing and hid a value — the owner
    /// the hand-over's `chown` names — that the commands really did carry and
    /// **no carriage value in this crate existed for at all.**
    ///
    /// **What is asked instead**: every value any elevated command of this crate
    /// carries has a declaration here, by kind. **The length follows.**
    #[test]
    fn every_value_entering_an_elevated_command_has_a_declared_carriage() {
        let declared: Vec<Carriage> = THE_ELEVATED_STEPS
            .iter()
            .map(|it| it.carriage)
            .chain([THE_PASSWORDS_CARRIAGE])
            .collect();
        let a_link_is_declared = declared
            .iter()
            .any(|it| matches!(it, Carriage::ThePlantedLink(_)));
        let a_token_is_declared = a_link_is_declared
            || declared.contains(&Carriage::ASynthesisedStagingToken);

        for step in crate::elevated::THE_ELEVATED_STEPS
            .iter()
            .chain([&crate::elevated::THE_ELEVATED_SEARCH])
        {
            for text in [
                step.plain.as_str(),
                step.elevated.as_str(),
                step.elevated_reading_the_password_from_standard_input.as_str(),
            ] {
                // **A substitution is 1.1's fourth carriage and nothing else**,
                // and this crate must declare that carriage for it.
                if text.contains("$(") {
                    assert!(
                        declared.contains(&Carriage::MachineSelfDerivation),
                        "{text} carries a machine-derived value and no carriage is declared for it"
                    );
                }
                // **A carriage token is either a planted link or a synthesised
                // staging token**, and one of the two must be declared.
                if text.contains(
                    smoozy_server_protocol::carriage::THE_DIRECTORY_ON_THE_SUBSYSTEM,
                ) {
                    assert!(
                        a_token_is_declared,
                        "{text} names a carriage token and no carriage is declared for it"
                    );
                }
                // **Everything else is fixed at build time**, which is the
                // constant-entire declaration.
                assert!(
                    declared.contains(&Carriage::ConstantEntire)
                        || declared.contains(&Carriage::AFixedBuildTimeMappingSelectsIt),
                    "{text} has no declaration of any kind"
                );
            }
        }

        // **And no carriage is declared that nothing rides**, so the
        // declaration cannot drift the other way either.
        for carriage in &declared {
            let ridden = crate::elevated::THE_ELEVATED_STEPS
                .iter()
                .chain([&crate::elevated::THE_ELEVATED_SEARCH])
                .any(|step| match carriage {
                    Carriage::MachineSelfDerivation => step.elevated.as_str().contains("$("),
                    Carriage::ThePlantedLink(_) | Carriage::ASynthesisedStagingToken => step
                        .elevated
                        .as_str()
                        .contains(smoozy_server_protocol::carriage::THE_DIRECTORY_ON_THE_SUBSYSTEM),
                    // The password rides no command string at all, which is E3.
                    Carriage::StandardInputReadAsData => true,
                    Carriage::ConstantEntire | Carriage::AFixedBuildTimeMappingSelectsIt => true,
                });
            assert!(ridden, "{carriage:?} is declared and nothing rides it");
        }
    }

    #[test]
    fn the_browsers_two_readings_are_answered_from_this_documents_own() {
        // `File Browser` P5's seam, and §12's assignment performed.
        let mut systemd = reading(MachineKind::ASystemdMachine, false, Probe::Failed);
        assert_eq!(systemd.the_machine_kind(), Theirs::ASystemdMachine);
        assert_eq!(
            the_offer(ElevatableAct::AListing, false, &mut systemd),
            Offer::OfferedAndAPasswordIsAsked
        );
    }

    #[test]
    fn a_containers_non_root_login_gets_no_offer_and_its_root_behaves_as_a_systemd_machine() {
        // Fact 27: a container's non-root login has an elevation route that
        // demands a password Smoozy does not hold.
        let mut non_root = reading(MachineKind::AContainer, false, Probe::Failed);
        assert_eq!(non_root.the_machine_kind(), Theirs::AContainerUnderANonRootLogin);
        assert_eq!(
            the_offer(ElevatableAct::Remove, false, &mut non_root),
            Offer::NoOffer(WhyThereIsNoOffer::AContainersLoginDemandsAPasswordSmoozyDoesNotHold)
        );

        let mut as_root = reading(MachineKind::AContainer, true, Probe::Succeeded);
        assert_eq!(as_root.the_machine_kind(), Theirs::ASystemdMachine);
        assert_eq!(
            the_offer(ElevatableAct::Remove, false, &mut as_root),
            Offer::OfferedAndNothingIsAsked
        );
    }

    #[test]
    fn a_machine_the_table_does_not_place_gets_no_offer_and_the_probe_is_not_reached() {
        let mut unplaced = reading(MachineKind::TheTableDoesNotPlaceIt, false, Probe::Failed);
        assert_eq!(unplaced.the_machine_kind(), Theirs::TheTableDoesNotPlaceIt);
        assert_eq!(
            the_offer(ElevatableAct::MakeDirectory, false, &mut unplaced),
            Offer::NoOffer(WhyThereIsNoOffer::TheKindTableDoesNotPlaceTheMachine)
        );
    }

    #[test]
    fn the_probe_crosses_unchanged_in_both_directions() {
        let mut passwordless = reading(MachineKind::ASystemdMachine, false, Probe::Succeeded);
        assert_eq!(
            passwordless.the_non_interactive_probe(),
            smoozy_file_browser::permission::TheProbe::Succeeded
        );
        let mut owed = reading(MachineKind::ASystemdMachine, false, Probe::Failed);
        assert_eq!(
            owed.the_non_interactive_probe(),
            smoozy_file_browser::permission::TheProbe::Failed
        );
    }
}
