//! What goes into the diary — `Reporting a problem` §3.
//!
//! Every line carries the run identity (U7), a kind from the closed set below,
//! the moment it records **and the clock that moment is on** (`State,
//! identity and recovery` D5 applied whole), and the identities of what it
//! concerns, **with the user's own labels carried beside identities and never
//! in place of them**.

use smoozy_state::identities::Identity;
use smoozy_state::observed::Clock;
use std::time::SystemTime;

/// J1: **the kinds are closed.** A line of no kind is not written, and a
/// builder that needs an eighth kind has found a hole in §3 rather than a case
/// it does not cover.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LineKind {
    /// U7: one per run, opening it. Carries the run identity, the moment on
    /// the desktop clock, and **Smoozy's own version and which operating
    /// system it runs on — exactly `Desktop app and delivery` E11's two values
    /// and nothing more**.
    Session,
    /// J2: an act a user could name. Declared per module, never decided at run
    /// time.
    Act,
    /// J4: an ending or a reading that came back from a machine, **and never
    /// on one clock** — it carries the machine's own moment on the machine's
    /// clock.
    Told,
    /// J6: the failure as Smoozy put it to the user, in the same words,
    /// captured verbatim at the first presentation per condition identity or
    /// ending record. **Nothing composes the words a second time at report
    /// time.**
    Failure,
    /// J7: a run-time arrival at a place the plan declares unreachable. The
    /// line is written and the application carries on.
    Assumption,
    /// J5: a reader had to state a gap, so the statement travels into every
    /// report — the server row, the cursor held, the oldest-retained number
    /// read.
    Gap,
    /// J8: what Smoozy manages to say on its way down. Written raw and forced
    /// to the device, never through a buffered writer.
    Last,
}

impl LineKind {
    pub const ALL: [LineKind; 7] = [
        LineKind::Session,
        LineKind::Act,
        LineKind::Told,
        LineKind::Failure,
        LineKind::Assumption,
        LineKind::Gap,
        LineKind::Last,
    ];
}

/// One line as the diary service is handed it. The service writes it; **no
/// module opens the diary, writes a line itself, or holds a diary file
/// handle** (R4).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Line {
    /// U7: every line carries the run identity of the run that wrote it. This
    /// is what the report's three slicing rules cut on.
    pub run: Identity,
    pub kind: LineKind,
    /// The moment the line records, and the clock it is on. A told line's
    /// moment is the machine's own; every other kind's is the desktop's.
    pub at: SystemTime,
    pub clock: Clock,
    /// The declared fixed form, filled with the declared values — **never free
    /// prose composed at run time, never a step the specs do not name, never a
    /// trace, a dump, or a mechanism's own vocabulary** (J3). A failure line's
    /// text is the exception in provenance rather than in shape: it is the
    /// sentence the user already met, verbatim.
    pub text: String,
    /// The identities of what the line concerns — job, group, local server
    /// identifier, operation — where there are any.
    pub concerning: Vec<Identity>,
}

/// J2a's three tests for whether an act is diary-worthy. **All three must
/// hold**, and a declaration is a fixed list: an act failing any of them, or
/// caught by [`ErasingActsAreNotDeclared`], is not declared.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ActTests {
    /// **The act is Smoozy's own**, initiated by the application, rather than a
    /// reading or an ending that came back from a machine — those are told
    /// lines.
    pub smoozys_own: bool,
    /// **The owning spec names its completion as something the user is told
    /// about or can see afterwards** — on a screen, in a row, in history, or
    /// in a folder — rather than as a step inside one.
    ///
    /// This is the test that gates an application-initiated act, and it is
    /// strict: a transition nobody sees fails it.
    pub the_spec_names_its_completion_as_something_the_user_sees: bool,
    /// **It runs at the user's own rate**: at most one line per occurrence of
    /// the act, never one per file, per frame, per derivative, per byte or per
    /// retry inside one act.
    ///
    /// This is a **rate bound and was only ever a rate bound** — naming the
    /// user's request was a description of the commonest case that hardened
    /// into a condition, and it was reworded on 2026-08-31 because it could
    /// not decide an act the application initiates, which is the very thing
    /// test one admits in terms.
    pub at_the_users_own_rate: bool,
}

/// J2a's bar, which sits beside the three tests and is **an exclusion rather
/// than a fourth test**.
///
/// **An act is not declared where the subject's own spec promises that nothing
/// afterwards shows the subject existed.** A cancel, the drop of a waiting job
/// whose Blender was removed, and the deletion of an entry, of a machine's
/// history or of all history each carry that promise in the Creator's own
/// words — and **the diary is a surface the user reads and may send to
/// somebody**, so a line naming one of them is precisely the showing the
/// promise forbids.
///
/// **Neither test two nor test three catches this**, which is why it is
/// written out: such an act can pass all three cleanly. **The cost is real and
/// is stated rather than hidden** — the diary is silent about the acts that
/// remove a job's traces, so a report cannot show what became of a job the
/// user erased. **That is the promise being kept, not a hole in it.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ErasingActsAreNotDeclared {
    pub the_subjects_spec_promises_no_trace_afterwards: bool,
}

impl ActTests {
    /// Whether an act may be declared. A builder that cannot decide a case
    /// from these has found a hole in §3 J2a rather than a case it does not
    /// cover.
    pub fn is_diary_worthy(self, bar: ErasingActsAreNotDeclared) -> bool {
        !bar.the_subjects_spec_promises_no_trace_afterwards
            && self.smoozys_own
            && self.the_spec_names_its_completion_as_something_the_user_sees
            && self.at_the_users_own_rate
    }
}

/// J4a: which of a machine's endings and readings becomes a told line, **and
/// no journal event becomes one by itself**.
///
/// Declared, not decided at run time, exactly as an act line is. **A module
/// with no declaration writes none**, which is `Desktop app and delivery` A7's
/// hosting rule and not a default.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ToldTests {
    /// **A told line records an ending or a reading the user could name** — a
    /// job's ending, a setup run's readings, a send's ending, and the
    /// machine's Blender version and Linux family at the acts that already
    /// read them.
    pub the_user_could_name_it: bool,
    /// **Never one line per journal event, per frame, per derivative, per file
    /// or per byte.**
    ///
    /// The bound is structural rather than tidiness: the journal's cap is
    /// stated in work — a job costs a small multiple of one entry per finished
    /// frame — so a told line per journal event would make the diary's forced
    /// device write a per-frame cost on the desktop and fill the 20 MB cap at
    /// the machine's rate, evicting the previous session the report's third
    /// slice depends on.
    pub not_per_event_frame_derivative_file_or_byte: bool,
}

impl ToldTests {
    pub fn may_be_declared(self) -> bool {
        self.the_user_could_name_it && self.not_per_event_frame_derivative_file_or_byte
    }
}

/// J2 and J4a's hosting rule, from the diary's side: **a module with no
/// declaration writes nothing, which is a hosting fault and not a default.**
///
/// A module writing nowhere satisfies its own spec and leaves a hole the
/// Creator cannot tell from a thing that did not happen — so until every
/// feature module's declaration exists, the record reads as complete and is
/// not.
#[derive(Debug, Default)]
pub struct Declarations<Module: Eq + std::hash::Hash> {
    declared: std::collections::HashMap<Module, Vec<LineKind>>,
}

impl<Module: Eq + std::hash::Hash> Declarations<Module> {
    pub fn new() -> Self {
        Declarations {
            declared: std::collections::HashMap::new(),
        }
    }

    pub fn module_declares(&mut self, module: Module, kinds: Vec<LineKind>) {
        self.declared.insert(module, kinds);
    }

    /// Whether this module may write a line of this kind. A module that
    /// declared nothing writes nothing — never a default set, and never every
    /// kind.
    pub fn may_write(&self, module: &Module, kind: LineKind) -> bool {
        match self.declared.get(module) {
            Some(kinds) => kinds.contains(&kind),
            None => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn a_declarable_act() -> ActTests {
        ActTests {
            smoozys_own: true,
            the_spec_names_its_completion_as_something_the_user_sees: true,
            at_the_users_own_rate: true,
        }
    }

    fn no_bar() -> ErasingActsAreNotDeclared {
        ErasingActsAreNotDeclared {
            the_subjects_spec_promises_no_trace_afterwards: false,
        }
    }

    #[test]
    fn the_kinds_are_seven_and_closed() {
        assert_eq!(LineKind::ALL.len(), 7);
    }

    #[test]
    fn an_act_passing_all_three_tests_is_declarable() {
        assert!(a_declarable_act().is_diary_worthy(no_bar()));
    }

    #[test]
    fn failing_any_one_test_is_enough_to_refuse_an_act() {
        let mut act = a_declarable_act();
        act.smoozys_own = false;
        assert!(!act.is_diary_worthy(no_bar()));

        let mut act = a_declarable_act();
        act.the_spec_names_its_completion_as_something_the_user_sees = false;
        assert!(!act.is_diary_worthy(no_bar()));

        let mut act = a_declarable_act();
        act.at_the_users_own_rate = false;
        assert!(!act.is_diary_worthy(no_bar()));
    }

    #[test]
    fn an_erasing_act_is_refused_though_it_passes_every_test() {
        let bar = ErasingActsAreNotDeclared {
            the_subjects_spec_promises_no_trace_afterwards: true,
        };
        assert!(!a_declarable_act().is_diary_worthy(bar));
    }

    #[test]
    fn a_told_line_at_the_wrong_rate_is_refused() {
        assert!(ToldTests {
            the_user_could_name_it: true,
            not_per_event_frame_derivative_file_or_byte: true,
        }
        .may_be_declared());

        assert!(!ToldTests {
            the_user_could_name_it: true,
            not_per_event_frame_derivative_file_or_byte: false,
        }
        .may_be_declared());
    }

    #[test]
    fn a_module_with_no_declaration_writes_nothing() {
        let declarations: Declarations<&str> = Declarations::new();
        for kind in LineKind::ALL {
            assert!(!declarations.may_write(&"File Browser", kind));
        }
    }

    #[test]
    fn a_module_writes_only_the_kinds_it_declared() {
        let mut declarations = Declarations::new();
        declarations.module_declares("File Browser", vec![LineKind::Act]);
        assert!(declarations.may_write(&"File Browser", LineKind::Act));
        assert!(!declarations.may_write(&"File Browser", LineKind::Told));
    }
}
