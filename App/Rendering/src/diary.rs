//! This module's diary declaration — `Rendering` §8's last part, to
//! `Reporting a problem`.
//!
//! The rules are `Reporting a problem` **J2, J2a, J3, J4 and J4a**, taken as
//! they stand. **The list below is fixed and nothing in it is decided at
//! run time.** Lines are written through the diary service `Desktop app and
//! delivery` L14 (its §5.3) names, whose hosting rule — a module declares
//! or writes nothing — is `Desktop app and delivery` A7.
//!
//! **The act: a job started.** One of the spec's own five, and this
//! module's, submitting being the only way a job comes into being (S1). It
//! passes J2a's three tests: Smoozy's own act, composed on the desktop and
//! sent at the user's asking; the spec names its completion as something
//! the user meets afterwards — a job is in the line exactly when its job
//! record exists (`Queue & History` Q3); and it runs at the user's own rate
//! — **one line per scene-job**, the act's own partition unit, so a
//! twelve-scene act writes twelve lines and never more. **Never one per
//! attempt, per frame, per file the render writes, or per re-run the
//! machine makes on its own** (A2).
//!
//! **No second act qualifies**: asking for a stop (the completion is `Queue
//! & History`'s), a render attempt started (a step inside a job, and the
//! machine makes another with nobody asking), the render's own reading at
//! render start (the machine's reading), the first-frame check's answer
//! (decided elsewhere), the missing-assets warning (completes nothing).
//!
//! **The told lines this document writes, and they are two** (`Reporting a
//! problem` J4, J4a).
//! *Held for an answer*, where an attempt placed the hold — at most one per
//! job, and the submission-origin hold is not one of these. And **the
//! machine's Blender version at the version confirmation** — written where
//! the desktop learns that value and nowhere else: the refusal's reason
//! where the confirmation failed, and the recorded reading where one ran
//! and carries the version, which is **Fact 105, not established** (§9 item
//! 16). **Where neither route carries it, no line is written, and none is
//! composed from the manifest's own version.** The job's own ending is not
//! one of them: `Queue & History` declares it at its E1 and E2.
//!
//! **Markers used in this file that belong to another document, each with
//! the document it belongs to.** `Reporting a problem` J2, J2a, J3, J4,
//! J4a, J6, N2, N3, U7. `Queue & History` Q3, E1, E2, N6, N7. `Job files
//! and outputs` D7, M1, M2. `State, identity and recovery` D5.
//!
//! **`Desktop app and delivery` A7 and L14 are not listed above and are
//! written out at each site**, because this plan defines an A7 and an L14
//! of its own.

use std::time::SystemTime;

use smoozy_reporting::lines::{Line, LineKind};
use smoozy_state::identities::Identity;
use smoozy_state::observed::{Clock, MachineClock};

/// **The two kinds this module declares**, and no others.
pub const DECLARED: [LineKind; 2] = [LineKind::Act, LineKind::Told];

/// **The act: a job started — the fixed line form.** The act's name; the
/// moment on the desktop clock, stated as that clock (`State, identity and
/// recovery` D5); the machine's own name as the user labelled it; the
/// `.blend`'s path on that machine, as the act named it; and the manifest's
/// scene name where it carries one — the reading's own bytes — with L15's
/// unnamed branch stated as naming no scene rather than left blank.
///
/// **What the line does not carry**: the settings payload, the frame list
/// and the rest of the manifest (M1, M2); nothing of the launch line, the
/// launch material, the marker or the reckoning (`Reporting a problem`
/// J3's mechanism's-own-vocabulary bar); and no second line for submitting
/// again or carrying on, both of which write this same act's line.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AJobStarted {
    pub the_machines_name_as_the_user_labelled_it: String,
    pub the_blends_path_on_that_machine: String,
    /// `None` is L15's unnamed branch, stated as such on the line.
    pub the_manifests_scene_name: Option<String>,
}

/// **The identities the act line carries**: the run identity (`Reporting a
/// problem` U7); the job identity and the group identity (S3, S2); the
/// local server identifier; and the submission command's operation identity
/// (S3), which is the identity a dropped act is resolved by (S9).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Concerning {
    pub run: Identity,
    pub job: Identity,
    pub group: Option<Identity>,
    pub local_server: Identity,
    pub submission_command: Identity,
}

impl AJobStarted {
    pub const NAME: &'static str = "job started";
    pub const NO_SCENE_NAMED: &'static str = "(no scene named)";

    fn text(&self) -> String {
        [
            AJobStarted::NAME,
            &self.the_machines_name_as_the_user_labelled_it,
            &self.the_blends_path_on_that_machine,
            self.the_manifests_scene_name
                .as_deref()
                .unwrap_or(AJobStarted::NO_SCENE_NAMED),
        ]
        .join("\t")
    }

    /// The line, ready for the diary service, on the desktop's clock.
    /// **Written on the desktop, once, for a scene-job whose job record
    /// exists (Q3). A scene-job refused (S8) or left unaccounted (S10)
    /// writes no act line** — nothing that did not happen is recorded as an
    /// act.
    pub fn line(&self, concerning: Concerning, at: SystemTime) -> Line {
        let mut identities = vec![
            concerning.run.clone(),
            concerning.job,
            concerning.local_server,
            concerning.submission_command,
        ];
        if let Some(group) = concerning.group {
            identities.push(group);
        }
        Line {
            run: concerning.run,
            kind: LineKind::Act,
            at,
            clock: Clock::ThisComputer,
            text: self.text(),
            concerning: identities,
        }
    }
}

/// **The first told line: *held for an answer*, where an attempt placed
/// the hold** (P1 step 5, K14) — an ending of the attempt that came back
/// from the machine with nobody connected, on the machine's own clock. **At
/// most one per job**, S16 putting no question a second time.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HeldForAnAnswer {
    pub job: Identity,
    pub attempt: u32,
}

impl HeldForAnAnswer {
    pub const NAME: &'static str = "held for an answer";

    pub fn line(
        &self,
        run: Identity,
        the_machine: &Identity,
        at_on_the_machines_clock: SystemTime,
    ) -> Line {
        Line {
            run: run.clone(),
            kind: LineKind::Told,
            at: at_on_the_machines_clock,
            clock: Clock::AMachine(MachineClock(the_machine.as_text())),
            text: [HeldForAnAnswer::NAME, &self.attempt.to_string()].join("\t"),
            concerning: vec![run, self.job.clone(), the_machine.clone()],
        }
    }
}

/// **Where the desktop learns the machine's Blender version, and it is
/// narrower than the confirmation's own rate.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WhereTheVersionReachedTheDesktop {
    /// The refusal's reason recorded verbatim where the confirmation
    /// failed (K14's failed limb) — the version the binary named itself.
    TheConfirmationsRefusal { the_binary_named: String },
    /// The recorded reading written at P1 step 4, where it carries the
    /// version — **Fact 105, not established**, so this is a route the
    /// acting half may never have a value for.
    TheRecordedReading { it_carried: String },
    /// Neither route carried it: **no line is written, and none is
    /// composed from the manifest's own version.**
    Neither,
}

/// **The second told line: the machine's Blender version at the version
/// confirmation** — `Reporting a problem` J4's own naming of this document.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TheMachinesBlenderVersion {
    pub job: Identity,
}

impl TheMachinesBlenderVersion {
    pub const NAME: &'static str = "blender version confirmed";

    /// `None` where neither route carried the value — the manifest's own
    /// version is never dressed as a machine's reading.
    pub fn line(
        &self,
        reached_the_desktop_by: &WhereTheVersionReachedTheDesktop,
        run: Identity,
        the_machine: &Identity,
        at_on_the_machines_clock: SystemTime,
    ) -> Option<Line> {
        let version = match reached_the_desktop_by {
            WhereTheVersionReachedTheDesktop::TheConfirmationsRefusal { the_binary_named } => {
                the_binary_named
            }
            WhereTheVersionReachedTheDesktop::TheRecordedReading { it_carried } => it_carried,
            WhereTheVersionReachedTheDesktop::Neither => return None,
        };
        Some(Line {
            run: run.clone(),
            kind: LineKind::Told,
            at: at_on_the_machines_clock,
            clock: Clock::AMachine(MachineClock(the_machine.as_text())),
            text: [TheMachinesBlenderVersion::NAME, version].join("\t"),
            concerning: vec![run, self.job.clone(), the_machine.clone()],
        })
    }
}

/// **What is not a told line, each on its ground**: the worker's live
/// readings (a rate J4a exists against); the marker's observations and the
/// device line (the mechanism's own vocabulary); each journal event of a
/// hold placed, released or dropped (no journal event becomes a told line
/// by itself); and the carry-on reading of the machine (shown on the page,
/// nothing durable written from it).
pub const NOT_TOLD_LINES: [&str; 4] = [
    "the worker's live readings at the copy loop",
    "the marker's observations and the device line observed in passing",
    "each journal event of a hold placed, released or dropped",
    "the carry-on reading of the machine",
];

#[cfg(test)]
mod tests {
    use super::*;
    use crate::material::testing::a_moment;
    use smoozy_reporting::lines::{ActTests, Declarations, ErasingActsAreNotDeclared, ToldTests};
    use smoozy_state::identities::IdentityKind;

    fn identity(kind: IdentityKind) -> Identity {
        Identity::issue(kind).unwrap()
    }

    fn concerning() -> Concerning {
        Concerning {
            run: identity(IdentityKind::Run),
            job: identity(IdentityKind::Job),
            group: Some(identity(IdentityKind::Group)),
            local_server: identity(IdentityKind::LocalServer),
            submission_command: identity(IdentityKind::Operation),
        }
    }

    #[test]
    fn this_module_declares_act_and_told_lines_and_the_act_passes_the_three_tests() {
        let mut declarations: Declarations<&str> = Declarations::new();
        declarations.module_declares("Rendering", DECLARED.to_vec());
        assert!(declarations.may_write(&"Rendering", LineKind::Act));
        assert!(declarations.may_write(&"Rendering", LineKind::Told));
        assert!(!declarations.may_write(&"Rendering", LineKind::Failure));
        let tests = ActTests {
            smoozys_own: true,
            the_spec_names_its_completion_as_something_the_user_sees: true,
            at_the_users_own_rate: true,
        };
        assert!(tests.is_diary_worthy(ErasingActsAreNotDeclared {
            the_subjects_spec_promises_no_trace_afterwards: false,
        }));
        assert!(ToldTests {
            the_user_could_name_it: true,
            not_per_event_frame_derivative_file_or_byte: true,
        }
        .may_be_declared());
        assert_eq!(NOT_TOLD_LINES.len(), 4);
    }

    #[test]
    fn the_act_line_carries_the_fixed_form_and_the_identities_and_states_the_unnamed_branch() {
        let concerning = concerning();
        let act = AJobStarted {
            the_machines_name_as_the_user_labelled_it: "the big box".into(),
            the_blends_path_on_that_machine: "/ephemeral/smoozy/assets/kitchen.blend".into(),
            the_manifests_scene_name: Some("Kitchen".into()),
        };
        let line = act.line(concerning.clone(), a_moment());
        assert_eq!(line.kind, LineKind::Act);
        assert_eq!(line.clock, Clock::ThisComputer);
        assert_eq!(
            line.text,
            "job started\tthe big box\t/ephemeral/smoozy/assets/kitchen.blend\tKitchen"
        );
        assert_eq!(line.concerning.len(), 5);
        assert!(line.concerning.contains(&concerning.submission_command));
        assert!(line.concerning.contains(concerning.group.as_ref().unwrap()));
        let unnamed = AJobStarted {
            the_manifests_scene_name: None,
            ..act
        }
        .line(concerning, a_moment());
        assert!(unnamed.text.ends_with("(no scene named)"));
    }

    #[test]
    fn the_hold_told_line_is_on_the_machines_clock() {
        let run = identity(IdentityKind::Run);
        let machine = identity(IdentityKind::LocalServer);
        let held = HeldForAnAnswer {
            job: identity(IdentityKind::Job),
            attempt: 1,
        };
        let line = held.line(run.clone(), &machine, a_moment());
        assert_eq!(line.kind, LineKind::Told);
        assert_eq!(line.clock, Clock::AMachine(MachineClock(machine.as_text())));
        assert_eq!(line.text, "held for an answer\t1");
    }

    /// **Fact 105 leaves no space**: where neither route carried the
    /// version, no line — and never the manifest's own version.
    #[test]
    fn the_version_told_line_is_written_only_where_a_machine_route_carried_the_value() {
        let run = identity(IdentityKind::Run);
        let machine = identity(IdentityKind::LocalServer);
        let told = TheMachinesBlenderVersion {
            job: identity(IdentityKind::Job),
        };
        let refusal = WhereTheVersionReachedTheDesktop::TheConfirmationsRefusal {
            the_binary_named: "4.5.9".into(),
        };
        assert_eq!(
            told.line(&refusal, run.clone(), &machine, a_moment())
                .unwrap()
                .text,
            "blender version confirmed\t4.5.9"
        );
        let reading = WhereTheVersionReachedTheDesktop::TheRecordedReading {
            it_carried: "5.2.0".into(),
        };
        assert!(told
            .line(&reading, run.clone(), &machine, a_moment())
            .is_some());
        assert_eq!(
            told.line(
                &WhereTheVersionReachedTheDesktop::Neither,
                run,
                &machine,
                a_moment()
            ),
            None
        );
    }
}
