//! This module's diary-act declaration — `Server Setup` §12.
//!
//! **The rules are `Reporting a problem` J2a's three tests, J3's line form and
//! J4a's told lines, taken as they stand and not reinterpreted.** Lines are
//! written through the diary service `Desktop app and delivery` §5.3 L14 names,
//! whose hosting rule — **`Desktop app and delivery` A7 makes a module with no
//! declaration write nothing, silently** — is that document's.
//!
//! **A declaration is a fixed list**: the acts below are the whole of it, an act
//! failing any of J2a's three tests is not declared, and **a module writes only
//! what its declaration names.** A declaration that exists only in this file's
//! tests is a module that writes nothing, so [`DECLARED`] is what is handed to
//! `Reporting a problem`'s registration at start-up.
//!
//! **Markers used below that belong to another document, each with the document
//! it belongs to.** `Reporting a problem` J2, J2a, J3, J4, J4a, U7.
//! `Desktop app and delivery` §5.3 L14. `State, identity and recovery` D5,
//! §2.3. `Rendering` L14.

use smoozy_reporting::lines::{ActTests, ErasingActsAreNotDeclared, LineKind, ToldTests};
use smoozy_state::identities::Identity;
use smoozy_state::observed::Clock;

use crate::blender::{HowTheBuildGotHere, Version};
use crate::kind::{DistributionFamily, MachineKind};
use crate::search::RemovalRoute;

/// **The two kinds this module declares, and no others.**
pub const DECLARED: [LineKind; 2] = [LineKind::Act, LineKind::Told];

/// **The name this module is registered under**, which is the name the plans
/// use. **A module is what its declaration is keyed on** (`Desktop app and
/// delivery` A7), so the name has one home and the registration reads it here.
pub const THE_MODULE: &str = "Server Setup";

/// **§12: the identities every line below carries, once, so that no line
/// composes them at report time.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WhatEveryLineCarries {
    /// **The desktop run identity of the run that wrote it** (`Reporting a
    /// problem` U7).
    pub run: Identity,
    /// **The server row the act was performed on.**
    pub server_row: Identity,
    /// **The installation identity the machine answered under** (`State,
    /// identity and recovery` §2.3's carrying rule), **so that a line about a
    /// recycled machine can never read as a line about the machine that
    /// replaced it.**
    pub installation: Identity,
    /// **The moment on the desktop clock, stated as that clock** (D5).
    pub at: std::time::SystemTime,
    pub clock: Clock,
}

/// **§12: the lines are per act the user asked for and never per setup run.**
///
/// The quick path's two ticked options run as one run —
/// [`crate::ready_action`] R1 composes the Blender publication into the ready
/// action's own sequence — **and write one line each.**
pub const A_LINE_IS_PER_ACT_AND_NEVER_PER_RUN: bool = true;

/// **The five act lines this module declares, each a fixed form filled with
/// declared values.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ActLine {
    /// **The ready action ran.** Smoozy's own act; the spec promises *the
    /// machine made able to render* by a single action and
    /// [`crate::statements`] W1's ending is what the user is told; one per
    /// action asked for.
    ///
    /// **Written when the run reaches that ending with the desktop driving
    /// it**; a run the desktop lost is carried by [`ToldLine::ASetupRunsOwnRecordedEnding`]
    /// when its ending is read back, **and is never guessed at**. **A recovery
    /// run carrying an interrupted ready action on is that same act and writes
    /// no second line**, and nothing is written per step, per boundary, per
    /// elevated command or per retry.
    TheReadyActionRan { ending: TheEndingsName },
    /// **A Blender was installed.** One per install asked for. **Never one line
    /// per resumed byte range of the fetch.**
    ABlenderWasInstalled {
        /// The version the build answered **in staging**
        /// ([`crate::blender`] B1).
        answered: Version,
        route: HowTheBuildGotHere,
        /// Whether it was published or **declined at the version answer**.
        published: bool,
    },
    /// **A Blender was taken off.** One per removal asked for.
    ABlenderWasTakenOff {
        answered: Version,
        route: RemovalRoute,
    },
    /// **An add-on was installed.** One per add-on the user took the offer on.
    /// **Never one line per file inside the packed directory.**
    AnAddOnWasInstalled {
        /// **The add-on's own name as the offer named it.**
        name: String,
        from: crate::addons::WhereAnAddOnComesFrom,
        /// **The Blender version whose settings folder it was installed into**
        /// ([`crate::addons`] A6).
        into: Version,
        concluded: crate::addons::WhatMayBeClaimed,
    },
    /// **Telegram's piece was placed by its own action.** One per use of the
    /// dedicated entry point.
    ///
    /// **Where the ready action places the same material as its step 8, that is
    /// a step inside the act the user asked for and writes no line of its
    /// own.**
    TelegramsPieceWasPlacedByItsOwnAction { the_desired_state_was_set_to_enabled: bool },
}

impl ActLine {
    /// The act's own name, which is the first field of every fixed form.
    pub fn name(&self) -> &'static str {
        match self {
            ActLine::TheReadyActionRan { .. } => "the ready action ran",
            ActLine::ABlenderWasInstalled { .. } => "a Blender was installed",
            ActLine::ABlenderWasTakenOff { .. } => "a Blender was taken off",
            ActLine::AnAddOnWasInstalled { .. } => "an add-on was installed",
            ActLine::TelegramsPieceWasPlacedByItsOwnAction { .. } => {
                "Telegram's piece was placed"
            }
        }
    }

    /// **The fixed form, filled with the declared values** — never free prose
    /// composed at run time, never a step the specs do not name, never a trace,
    /// a dump, or a mechanism's own vocabulary (`Reporting a problem` J3).
    ///
    /// **Every value below is drawn from a closed set the cited clause already
    /// fixes**, and [`NeverInALine`] is what none of them is.
    pub fn text(&self) -> String {
        let mut fields = vec![self.name().to_string()];
        match self {
            ActLine::TheReadyActionRan { ending } => fields.push(ending.as_declared().to_string()),
            ActLine::ABlenderWasInstalled { answered, route, published } => {
                fields.push(a_version(*answered));
                fields.push(the_route(*route).to_string());
                fields.push(
                    if *published {
                        "published"
                    } else {
                        "declined at the version answer"
                    }
                    .to_string(),
                );
            }
            ActLine::ABlenderWasTakenOff { answered, route } => {
                fields.push(a_version(*answered));
                fields.push(
                    match route {
                        RemovalRoute::TheFolderDeletion => "the folder deletion",
                        RemovalRoute::TheMachinesOwnSoftwareManager => "the machine's own software manager",
                    }
                    .to_string(),
                );
            }
            ActLine::AnAddOnWasInstalled { name, from, into, concluded } => {
                fields.push(name.clone());
                fields.push(
                    match from {
                        crate::addons::WhereAnAddOnComesFrom::ByNameFromARepositoryAlreadyOnTheList => {
                            "by name from a repository already on the build's list"
                        }
                        crate::addons::WhereAnAddOnComesFrom::AFileTheUserPointedAt => {
                            "a copy from the user's own computer"
                        }
                    }
                    .to_string(),
                );
                fields.push(a_version(*into));
                fields.push(
                    match concluded {
                        crate::addons::WhatMayBeClaimed::InstalledAndSwitchedOnLoadableAtThatMoment => {
                            "installed and switched on"
                        }
                        crate::addons::WhatMayBeClaimed::NotInstalled => "not installed",
                    }
                    .to_string(),
                );
            }
            ActLine::TelegramsPieceWasPlacedByItsOwnAction {
                the_desired_state_was_set_to_enabled,
            } => fields.push(
                if *the_desired_state_was_set_to_enabled {
                    "the desired state was set to enabled"
                } else {
                    "the desired state was not set"
                }
                .to_string(),
            ),
        }
        fields.join("\t")
    }

    /// The line, ready for the diary service.
    ///
    /// **The identities are §12's own** — the server row the act was performed
    /// on and the installation identity the machine answered under, **so that a
    /// line about a recycled machine can never read as a line about the machine
    /// that replaced it** (`State, identity and recovery` §2.3). **The run
    /// identity travels in [`Line::run`]** and is not repeated among them
    /// (`Reporting a problem` U7).
    pub fn line(&self, carries: &WhatEveryLineCarries) -> smoozy_reporting::lines::Line {
        let mut concerning = vec![carries.server_row.clone()];
        concerning.push(carries.installation.clone());
        smoozy_reporting::lines::Line {
            run: carries.run.clone(),
            kind: LineKind::Act,
            at: carries.at,
            clock: carries.clock.clone(),
            text: self.text(),
            concerning,
        }
    }
}

fn a_version(version: Version) -> String {
    format!("{}.{}.{}", version.major, version.minor, version.patch)
}

fn the_route(route: HowTheBuildGotHere) -> &'static str {
    match route {
        HowTheBuildGotHere::ThePublishedListing => "the published listing",
        HowTheBuildGotHere::ABuildTheUserPointedAt => "a build the user pointed at",
        // **Not a route an install line can take** — B8a's third reading is a
        // build Smoozy did not place, and no install of this module's produced
        // it. It is named rather than left to a wildcard, so the set stays
        // closed.
        HowTheBuildGotHere::SmoozyDidNotPlaceIt => "not placed by Smoozy",
    }
}

/// **The ending's own name, from a closed set of six** — [`crate::statements`]
/// W1's five statements, **plus *stopped part-way* for a run that ended at a
/// boundary without completing.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TheEndingsName {
    Ready,
    NothingToPrepare,
    NotAKindSmoozyPrepares,
    Refused,
    CouldNotBeMadeReady,
    StoppedPartWay,
}

impl TheEndingsName {
    /// **The name as a line states it**, from the closed set and never composed
    /// at run time.
    pub fn as_declared(self) -> &'static str {
        match self {
            TheEndingsName::Ready => "ready to render",
            TheEndingsName::NothingToPrepare => "nothing to prepare",
            TheEndingsName::NotAKindSmoozyPrepares => "not a kind of machine Smoozy prepares",
            TheEndingsName::Refused => "refused",
            TheEndingsName::CouldNotBeMadeReady => "could not be made ready",
            TheEndingsName::StoppedPartWay => "stopped part-way",
        }
    }

    /// **Closed at six.**
    pub const ALL: [TheEndingsName; 6] = [
        TheEndingsName::Ready,
        TheEndingsName::NothingToPrepare,
        TheEndingsName::NotAKindSmoozyPrepares,
        TheEndingsName::Refused,
        TheEndingsName::CouldNotBeMadeReady,
        TheEndingsName::StoppedPartWay,
    ];
}

/// **§12's second told line takes the ending's own kind from the closed set the
/// record holds, which is a different list from the act line's.**
///
/// **The act line's set is [`TheEndingsName`]** — W1's five statements plus
/// *stopped part-way* — and the told line used it for both until 2026-09-04.
/// **Three of the record's five have no name in that set at all**: a step that
/// failed, a run declined at the version answer, and a run that completed a kind
/// of setup with no W1 ending of its own. **Nothing composes a told line yet, so
/// this is a type that would have been wrong when somebody did.**
///
/// **The kind and never the payload**: `Reporting a problem` J3 forbids a
/// boundary name and a step name in a line, and the record's own endings carry
/// both.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TheRecordedEndingsKind {
    /// The final verification passed and the state, where set, was cleared.
    Completed,
    /// A step failed. **The step's own name stays out of the line** (J3).
    AStepFailed,
    /// The run ended at a boundary without completing. **The boundary's own name
    /// stays out of the line** (J3).
    EndedAtBoundaryNotCompleted,
    /// Left stopped, from the user's own durable stop. **The boundary's name
    /// stays out** (J3).
    LeftStopped,
    /// [`crate::blender`] B7a: declined at the version answer — a recorded
    /// ending, and not one of [`crate::statements`] W1's five.
    DeclinedAtTheVersionAnswer,
    /// **S6's abandon**, added 2026-09-22. **What the final verification found
    /// stays out of the line** (J3), as a failed step's name does.
    Abandoned,
}

impl TheRecordedEndingsKind {
    /// **Closed at six, which is what the record holds** — five until
    /// 2026-09-22, when S6's abandon was built.
    pub const ALL: [TheRecordedEndingsKind; 6] = [
        TheRecordedEndingsKind::Completed,
        TheRecordedEndingsKind::AStepFailed,
        TheRecordedEndingsKind::EndedAtBoundaryNotCompleted,
        TheRecordedEndingsKind::LeftStopped,
        TheRecordedEndingsKind::DeclinedAtTheVersionAnswer,
        TheRecordedEndingsKind::Abandoned,
    ];

    /// **The recorded ending's kind, with its payload left behind.**
    pub fn of(ending: &crate::recovery::HowTheRunEnded) -> TheRecordedEndingsKind {
        match ending {
            crate::recovery::HowTheRunEnded::Completed => TheRecordedEndingsKind::Completed,
            crate::recovery::HowTheRunEnded::AStepFailed { .. } => {
                TheRecordedEndingsKind::AStepFailed
            }
            crate::recovery::HowTheRunEnded::EndedAtBoundaryNotCompleted { .. } => {
                TheRecordedEndingsKind::EndedAtBoundaryNotCompleted
            }
            crate::recovery::HowTheRunEnded::LeftStopped { .. } => {
                TheRecordedEndingsKind::LeftStopped
            }
            crate::recovery::HowTheRunEnded::DeclinedAtTheVersionAnswer => {
                TheRecordedEndingsKind::DeclinedAtTheVersionAnswer
            }
            crate::recovery::HowTheRunEnded::Abandoned { .. } => TheRecordedEndingsKind::Abandoned,
        }
    }
}

/// **The two told lines this module writes — declared so that the absence of
/// the rest is a decision** (`Reporting a problem` J4a).
///
/// Each carries J4's four things and **nothing computed across two clocks**: the
/// machine's own moment on the machine's clock, the journal sequence number the
/// thing arrived at, the installation identity it was read under, and the server
/// row.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ToldLine {
    /// **The machine kind and the distribution family, at every setup run**
    /// ([`crate::kind`] M1, M1a) — **the reading J4 already names as this
    /// document's**, written at the moment of the reading, **once per run**, and
    /// carrying the two answers and nothing else.
    TheMachineKindAndTheDistributionFamily {
        kind: MachineKind,
        family: Option<DistributionFamily>,
    },
    /// **A setup run's own recorded ending, where the desktop reads it back
    /// rather than driving the run to it** — [`crate::recovery`] S1 and S2's
    /// fresh reading on a later connection, or S4 item 2's take-over. **One per
    /// run**, carrying the ending's own kind from the closed set the record
    /// holds.
    ///
    /// **This is what a setup cut off part-way leaves in the diary**, the act
    /// line above never having been written for it.
    ///
    /// **The kind is the record's own and never the act line's** — see
    /// [`TheRecordedEndingsKind`].
    ASetupRunsOwnRecordedEnding { ending: TheRecordedEndingsKind },
}

/// **§12: what no line above carries, on `Reporting a problem` J3's bound.**
///
/// **No machine path; no tool's captured words** — [`crate::recovery`] S2a keeps
/// those on the machine and `Security and licence` §13 rule 5 governs what may
/// be done with them; **no password or any part of one, and no statement that
/// one was held; no boundary name, no exit status, no package name, no counted
/// duration.** Every value above is drawn from a closed set the cited clause
/// already fixes, **and nothing is composed at run time.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NeverInALine {
    AMachinePath,
    AToolsCapturedWords,
    APasswordOrAnyPartOfOne,
    AStatementThatOneWasHeld,
    ABoundaryName,
    AnExitStatus,
    APackageName,
    ACountedDuration,
}

impl NeverInALine {
    pub const ALL: [NeverInALine; 8] = [
        NeverInALine::AMachinePath,
        NeverInALine::AToolsCapturedWords,
        NeverInALine::APasswordOrAnyPartOfOne,
        NeverInALine::AStatementThatOneWasHeld,
        NeverInALine::ABoundaryName,
        NeverInALine::AnExitStatus,
        NeverInALine::APackageName,
        NeverInALine::ACountedDuration,
    ];
}

/// **J3's bound, as the question a caller asks.**
pub fn a_line_may_carry(_it: NeverInALine) -> bool {
    false
}

/// **§12: nothing else this module reads becomes a told line, and the
/// rejections are named because each is a reading a builder could mistake for
/// one.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RejectedAsAToldLine {
    /// The card-presence scan and the driver's own answer
    /// ([`crate::ready_action`] R1 items 1 and 5), **read up to three times
    /// inside one run**.
    TheCardScanAndTheDriversAnswer,
    /// `ldd`'s not-found list and its re-read (item 4).
    TheLddListAndItsReRead,
    /// The search's per-candidate version answers, **one per build the machine
    /// holds** — **and the Blender version's told line is `Rendering`'s at the
    /// launch confirmation, per `Reporting a problem` J4, never this document's
    /// search.**
    TheSearchsPerCandidateVersionAnswers,
    /// The add-on read-back listings ([`crate::addons`] A5).
    TheAddOnReadBackListings,
    /// The boundary list and its last-reached marker read over the connection
    /// ([`crate::run`] Y6), **which are per step**.
    TheBoundaryListAndItsMarker,
    /// [`crate::fallback`] F1's package inspection, **which step 4 keeps inside
    /// the module because no spec grants the user a view of it.**
    ThePackageInspection,
    /// **The desktop's read-forward of the event journal is not a source of
    /// told lines at all** (J4a).
    TheEventJournalsReadForward,
}

impl RejectedAsAToldLine {
    pub const ALL: [RejectedAsAToldLine; 7] = [
        RejectedAsAToldLine::TheCardScanAndTheDriversAnswer,
        RejectedAsAToldLine::TheLddListAndItsReRead,
        RejectedAsAToldLine::TheSearchsPerCandidateVersionAnswers,
        RejectedAsAToldLine::TheAddOnReadBackListings,
        RejectedAsAToldLine::TheBoundaryListAndItsMarker,
        RejectedAsAToldLine::ThePackageInspection,
        RejectedAsAToldLine::TheEventJournalsReadForward,
    ];
}

/// **The rejections, as the question a caller asks.**
pub fn becomes_a_told_line(_it: RejectedAsAToldLine) -> bool {
    false
}

/// **J2a's three tests, as this module answered them for its five acts.**
///
/// All five are **Smoozy's own**, all five have their completion named by the
/// owning spec as something the user is told about or can see afterwards, and
/// all five run **at the user's own rate** — one line per act asked for.
pub fn the_tests_for_this_modules_acts() -> ActTests {
    ActTests {
        smoozys_own: true,
        the_spec_names_its_completion_as_something_the_user_sees: true,
        at_the_users_own_rate: true,
    }
}

/// **J2a's bar, and none of this module's acts is caught by it.**
///
/// **No act of this module carries a promise that nothing afterwards shows the
/// subject existed.** A Blender removal is the one that could look like an
/// erasing act and is not: the spec gives the user the removal **and the
/// machine's list afterwards is where it shows**, so nothing is promised to
/// vanish.
pub fn the_bar() -> ErasingActsAreNotDeclared {
    ErasingActsAreNotDeclared {
        the_subjects_spec_promises_no_trace_afterwards: false,
    }
}

/// **J4a's two tests, as this module answered them for its two told lines.**
pub fn the_tests_for_this_modules_told_lines() -> ToldTests {
    ToldTests {
        the_user_could_name_it: true,
        not_per_event_frame_derivative_file_or_byte: true,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn this_module_declares_two_kinds_and_no_others() {
        assert_eq!(DECLARED.len(), 2);
        assert!(DECLARED.contains(&LineKind::Act));
        assert!(DECLARED.contains(&LineKind::Told));
        for kind in LineKind::ALL {
            if kind == LineKind::Act || kind == LineKind::Told {
                continue;
            }
            assert!(!DECLARED.contains(&kind));
        }
    }

    /// **The fixed form, filled with declared values and nothing composed at
    /// run time** (`Reporting a problem` J3). Added 2026-09-22 with the
    /// composer, step 10c's fourth pass on Vast, fault 7.
    #[test]
    fn an_act_lines_fixed_form_is_the_declared_values_and_no_debugging_rendering() {
        let ran = ActLine::TheReadyActionRan {
            ending: TheEndingsName::StoppedPartWay,
        };
        assert_eq!(ran.text(), "the ready action ran\tstopped part-way");

        let installed = ActLine::ABlenderWasInstalled {
            answered: Version { major: 5, minor: 2, patch: 2 },
            route: HowTheBuildGotHere::ThePublishedListing,
            published: true,
        };
        assert_eq!(
            installed.text(),
            "a Blender was installed\t5.2.2\tthe published listing\tpublished"
        );

        // **No line of this module is a debugging rendering**, and every ending
        // in the closed set has a name of its own.
        for ending in TheEndingsName::ALL {
            let said = ActLine::TheReadyActionRan { ending }.text();
            for debugging in ["Some(", "None", "{", "::", "(\""] {
                assert!(!said.contains(debugging), "{said}");
            }
        }
    }

    /// **§12: the line carries the server row and the installation identity**,
    /// so a line about a recycled machine can never read as a line about the
    /// machine that replaced it, **and the run identity is not repeated among
    /// them** (`Reporting a problem` U7 carries it in the line's own field).
    #[test]
    fn every_act_line_carries_the_server_row_and_the_installation() {
        use smoozy_state::identities::IdentityKind;
        let run = Identity::issue(IdentityKind::Run).expect("randomness");
        let server_row = Identity::issue(IdentityKind::LocalServer).expect("randomness");
        let installation = Identity::issue(IdentityKind::Installation).expect("randomness");
        let carries = WhatEveryLineCarries {
            run: run.clone(),
            server_row: server_row.clone(),
            installation: installation.clone(),
            at: std::time::SystemTime::now(),
            clock: Clock::ThisComputer,
        };
        let line = ActLine::TheReadyActionRan {
            ending: TheEndingsName::Ready,
        }
        .line(&carries);
        assert_eq!(line.kind, LineKind::Act);
        assert_eq!(line.clock, Clock::ThisComputer);
        assert_eq!(line.run, run);
        assert_eq!(line.concerning, vec![server_row, installation]);
        assert!(!line.text.contains(&run.as_text()));
    }

    #[test]
    fn a_module_that_declares_nothing_writes_nothing_and_this_one_declares() {
        // `Desktop app and delivery` A7's hosting rule, kept by handing the
        // declaration over rather than by hoping.
        let mut declarations = smoozy_reporting::lines::Declarations::new();
        assert!(!declarations.may_write(&"Server Setup", LineKind::Act));
        declarations.module_declares("Server Setup", DECLARED.to_vec());
        assert!(declarations.may_write(&"Server Setup", LineKind::Act));
        assert!(declarations.may_write(&"Server Setup", LineKind::Told));
        assert!(!declarations.may_write(&"Server Setup", LineKind::Failure));
    }

    #[test]
    fn every_declared_act_passes_all_three_of_j2as_tests_and_the_bar() {
        assert!(the_tests_for_this_modules_acts().is_diary_worthy(the_bar()));
        assert!(the_tests_for_this_modules_told_lines().may_be_declared());
    }

    #[test]
    fn the_endings_name_is_a_closed_set_of_six() {
        // W1's five statements, plus *stopped part-way* for a run that ended at
        // a boundary without completing.
        assert_eq!(TheEndingsName::ALL.len(), 6);
        assert!(TheEndingsName::ALL.contains(&TheEndingsName::StoppedPartWay));
    }

    #[test]
    fn no_line_carries_a_path_a_tools_words_a_password_or_a_duration() {
        assert_eq!(NeverInALine::ALL.len(), 8);
        for it in NeverInALine::ALL {
            assert!(!a_line_may_carry(it));
        }
    }

    #[test]
    fn the_seven_readings_named_as_rejections_become_no_told_line() {
        assert_eq!(RejectedAsAToldLine::ALL.len(), 7);
        for it in RejectedAsAToldLine::ALL {
            assert!(!becomes_a_told_line(it));
        }
    }

    #[test]
    fn a_run_the_desktop_lost_is_carried_by_the_told_line_and_never_guessed_at() {
        // §12: the act line is written when the run reaches its ending with the
        // desktop driving it, and the told line is what a cut-off setup leaves.
        let told = ToldLine::ASetupRunsOwnRecordedEnding {
            ending: TheRecordedEndingsKind::EndedAtBoundaryNotCompleted,
        };
        assert!(matches!(told, ToldLine::ASetupRunsOwnRecordedEnding { .. }));
        const { assert!(A_LINE_IS_PER_ACT_AND_NEVER_PER_RUN) };
    }

    #[test]
    fn the_told_lines_ending_comes_from_the_record_and_carries_none_of_its_payload() {
        // **The fault this test exists for.** The told line drew from the act
        // line's set, and three of the record's five have no name in that set at
        // all — a step that failed, a run declined at the version answer, and a
        // run that completed a kind of setup with no W1 ending of its own.
        // Six from 2026-09-22, when `Server Setup` S6's abandon was built.
        assert_eq!(TheRecordedEndingsKind::ALL.len(), 6);
        assert_eq!(
            TheRecordedEndingsKind::of(&crate::recovery::HowTheRunEnded::AStepFailed {
                step: crate::ready_action::Step::Libraries,
                words: crate::recovery::TheFailedStepsWords::None,
            }),
            TheRecordedEndingsKind::AStepFailed
        );
        assert_eq!(
            TheRecordedEndingsKind::of(
                &crate::recovery::HowTheRunEnded::DeclinedAtTheVersionAnswer
            ),
            TheRecordedEndingsKind::DeclinedAtTheVersionAnswer
        );
        // **J3's bound: neither the boundary's name nor the step's rides the
        // line**, so the two endings that carry one lose it here.
        assert_eq!(
            TheRecordedEndingsKind::of(
                &crate::recovery::HowTheRunEnded::EndedAtBoundaryNotCompleted {
                    at: Some(crate::run::Boundary::Tooling)
                }
            ),
            TheRecordedEndingsKind::of(
                &crate::recovery::HowTheRunEnded::EndedAtBoundaryNotCompleted { at: None }
            )
        );
        assert!(!a_line_may_carry(NeverInALine::ABoundaryName));
        // **And the act line's set stays where it belongs**, at six.
        assert_eq!(TheEndingsName::ALL.len(), 6);
    }

    #[test]
    fn the_kind_and_family_told_line_carries_the_two_answers_and_nothing_else() {
        let told = ToldLine::TheMachineKindAndTheDistributionFamily {
            kind: MachineKind::ASystemdMachine,
            family: Some(DistributionFamily::Ubuntu),
        };
        match told {
            ToldLine::TheMachineKindAndTheDistributionFamily { kind, family } => {
                assert_eq!(kind, MachineKind::ASystemdMachine);
                assert_eq!(family, Some(DistributionFamily::Ubuntu));
            }
            _ => panic!("the other told line"),
        }
    }

    #[test]
    fn the_five_act_lines_are_the_whole_declaration() {
        // One per act the user asked for, and nothing per step, per file, per
        // byte range or per retry.
        let lines = [
            ActLine::TheReadyActionRan {
                ending: TheEndingsName::Ready,
            },
            ActLine::ABlenderWasInstalled {
                answered: Version::parse("5.2.0").unwrap(),
                route: HowTheBuildGotHere::ThePublishedListing,
                published: true,
            },
            ActLine::ABlenderWasTakenOff {
                answered: Version::parse("4.5.9").unwrap(),
                route: RemovalRoute::TheFolderDeletion,
            },
            ActLine::AnAddOnWasInstalled {
                name: "magic".to_string(),
                from: crate::addons::WhereAnAddOnComesFrom::AFileTheUserPointedAt,
                into: Version::parse("5.2.0").unwrap(),
                concluded:
                    crate::addons::WhatMayBeClaimed::InstalledAndSwitchedOnLoadableAtThatMoment,
            },
            ActLine::TelegramsPieceWasPlacedByItsOwnAction {
                the_desired_state_was_set_to_enabled: true,
            },
        ];
        assert_eq!(lines.len(), 5);
    }
}
