//! **The told-record's rules — `Telegram` §9 L1 to L4, §19's two rows for a
//! telling reaching a machine, and §19's second diary act.** Step 10 piece 5.
//!
//! **L1: the told-record is Smoozy's own record of what it last told each
//! machine and when**, five things per machine, each with the value last
//! acknowledged, the moment of the acknowledgement, and the installation
//! identity it was told under. **Every entry is Smoozy's own act of telling and
//! never a copy of a reading** (`State, identity and recovery` D4): an entry can
//! only be made from a [`WhatSmoozyTells`], and a [`WhatSmoozyTells`] can only
//! be made from a value Smoozy itself holds. There is no constructor that takes
//! anything a machine said.
//!
//! **The store is the desktop's**, at `App/Desktop app and delivery/src/record/
//! told_record.rs`; this file holds the shapes and the rules, so they are tested
//! here with nothing stored.
//!
//! **L2: delivery is the ordinary command path on the next connection, and not
//! before.** Nothing here sends anything. The file half of the material travels
//! through `Server Setup`'s one placement act ([`crate::material`]); the command
//! half carries the role's desired state. No paid policy is told or enforced.
//!
//! **Markers in this file are all `Telegram`'s**: L1, L2, L3, L4, V2,
//! P1, P4, K1, R8. `State, identity and recovery`'s are written with their
//! document.

use std::time::SystemTime;

use crate::proof::TheConversation;

// ------------------------------------------------------------ L1: the five things

/// **L1's five told things, per machine.**
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AToldThing {
    /// The role's enabled-or-disabled desired state.
    TheRolesDesiredState,
    /// The token generation the machine holds.
    TheTokenGeneration,
    /// The proved conversation identity (P1).
    TheProvedConversation,
    /// The machine's display label (R8).
    TheLabel,
    /// The machine-level reporting settings (V2).
    TheReportingSettings,
}

impl AToldThing {
    pub const ALL: [AToldThing; 5] = [
        AToldThing::TheRolesDesiredState,
        AToldThing::TheTokenGeneration,
        AToldThing::TheProvedConversation,
        AToldThing::TheLabel,
        AToldThing::TheReportingSettings,
    ];

    /// **The one spelling a told thing is stored under.**
    pub fn stored_as(self) -> &'static str {
        match self {
            AToldThing::TheRolesDesiredState => "the role's desired state",
            AToldThing::TheTokenGeneration => "the token generation",
            AToldThing::TheProvedConversation => "the proved conversation",
            AToldThing::TheLabel => "the label",
            AToldThing::TheReportingSettings => "the reporting settings",
        }
    }

    /// The thing a stored spelling is, or nothing.
    pub fn stored_as_this(name: &str) -> Option<AToldThing> {
        AToldThing::ALL
            .into_iter()
            .find(|thing| thing.stored_as() == name)
    }
}

/// **The token generation** — which replacement of the bot's token a machine
/// holds (L4). The first token a walkthrough keeps is the first generation, and
/// each replacement accepted is the next. **A generation is a count and never a
/// token**, so it may be written in the record and in the diary (K1).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TokenGeneration(pub u64);

impl TokenGeneration {
    pub const THE_FIRST: TokenGeneration = TokenGeneration(1);

    /// The generation a replacement makes. **None where the count cannot go
    /// on**, which is refused rather than wrapped.
    pub fn the_next(self) -> Option<TokenGeneration> {
        self.0.checked_add(1).map(TokenGeneration)
    }
}

// ------------------------------------------------------------ what Smoozy tells

/// **A value Smoozy tells a machine, in the one written form a told entry keeps.**
///
/// **Each constructor takes a value Smoozy itself holds** — its own desired
/// state for the role, the generation its store holds,
/// the conversation its walkthrough proved, the label the user typed, the
/// switches the user set. **None takes anything a machine reported**, which is
/// L1's rule and `State, identity and recovery` D4's, kept by the shape rather
/// than by care.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WhatSmoozyTells {
    thing: AToldThing,
    value: String,
}

impl WhatSmoozyTells {
    pub fn the_roles_desired_state(enabled: bool) -> WhatSmoozyTells {
        WhatSmoozyTells {
            thing: AToldThing::TheRolesDesiredState,
            value: if enabled { "enabled" } else { "disabled" }.to_string(),
        }
    }

    pub fn the_token_generation(generation: TokenGeneration) -> WhatSmoozyTells {
        WhatSmoozyTells {
            thing: AToldThing::TheTokenGeneration,
            value: generation.0.to_string(),
        }
    }

    pub fn the_proved_conversation(conversation: TheConversation) -> WhatSmoozyTells {
        WhatSmoozyTells {
            thing: AToldThing::TheProvedConversation,
            value: conversation.0.to_string(),
        }
    }

    /// **R8: the label the machine is told**, which is display only. A server
    /// with no label is told that it has none, which is a value of its own.
    pub fn the_label(label: Option<&str>) -> WhatSmoozyTells {
        WhatSmoozyTells {
            thing: AToldThing::TheLabel,
            value: match label {
                Some(text) => format!("labelled {text}"),
                None => "no label".to_string(),
            },
        }
    }

    /// `Telegram` L1: one told item carries the label and hue together.
    pub fn the_label_and_colour(label: Option<&str>, colour: Option<&str>) -> WhatSmoozyTells {
        if colour.is_none() { return Self::the_label(label); }
        WhatSmoozyTells { thing: AToldThing::TheLabel, value: serde_json::json!({"label": label, "colour": colour}).to_string() }
    }

    /// **V2: the machine-level switches**, each by its stored name, sorted.
    pub fn the_reporting_settings<'a>(
        switches: impl IntoIterator<Item = (&'a str, bool)>,
    ) -> WhatSmoozyTells {
        let mut named: Vec<String> = switches
            .into_iter()
            .map(|(name, on)| format!("{name} {}", if on { "on" } else { "off" }))
            .collect();
        named.sort_unstable();
        WhatSmoozyTells {
            thing: AToldThing::TheReportingSettings,
            value: named.join(", "),
        }
    }

    pub fn thing(&self) -> AToldThing {
        self.thing
    }

    pub fn value(&self) -> &str {
        &self.value
    }
}

// ------------------------------------------------------------ L1: an entry

/// **One told entry: a value Smoozy told, acknowledged at a moment, under an
/// installation.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AToldEntry {
    pub thing: AToldThing,
    pub value: String,
    /// On the desktop's own clock — the moment the acknowledgement arrived.
    pub acknowledged_at: SystemTime,
    /// The installation identity the telling was acknowledged under.
    pub installation: String,
}

impl AToldEntry {
    /// **L1 and L2: the entry is written on the acknowledgement**, from what
    /// Smoozy told. A crash between the telling and this costs a re-send.
    pub fn acknowledged(told: &WhatSmoozyTells, at: SystemTime, installation: &str) -> AToldEntry {
        AToldEntry {
            thing: told.thing,
            value: told.value.clone(),
            acknowledged_at: at,
            installation: installation.to_string(),
        }
    }
}

// ------------------------------------------------------------ L3: the derivation

/// **One machine as the derivation reads it**: its local server identifier, its
/// label, the installation it is known to hold now, and what it was told.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AMachineAndWhatItWasTold {
    pub server: String,
    pub label: Option<String>,
    pub installation: Option<String>,
    pub told: Vec<AToldEntry>,
}

/// **L3: which of the current values a machine has not been told**, computed
/// when shown and never stored.
///
/// **An entry counts only under the installation the machine holds now.** A
/// machine whose installation was replaced was told nothing under the new one,
/// and a machine with no confirmed installation has been told nothing Smoozy
/// can stand behind.
pub fn what_has_not_reached(
    machine: &AMachineAndWhatItWasTold,
    now: &[WhatSmoozyTells],
) -> Vec<AToldThing> {
    now.iter()
        .filter(|current| {
            !machine.told.iter().any(|entry| {
                entry.thing == current.thing
                    && entry.value == current.value
                    && machine.installation.as_deref() == Some(entry.installation.as_str())
            })
        })
        .map(|current| current.thing)
        .collect()
}

/// **L3, over a fleet: every machine that has not been told something, and
/// what.** A machine told everything is left out.
pub fn the_machines_not_reached(
    fleet: &[AMachineAndWhatItWasTold],
    now: impl Fn(&AMachineAndWhatItWasTold) -> Vec<WhatSmoozyTells>,
) -> Vec<(&AMachineAndWhatItWasTold, Vec<AToldThing>)> {
    fleet
        .iter()
        .map(|machine| (machine, what_has_not_reached(machine, &now(machine))))
        .filter(|(_, things)| !things.is_empty())
        .collect()
}

// ------------------------------------------------------------ L4: a replacement

/// **L4: a replacement token accepted.** The token is checked and stored, and
/// **the record marks every machine as holding the old generation** — which is
/// the generation moving on, and L3's derivation then naming every machine whose
/// told generation is the old one. No row per machine is written for it: that
/// would be a second record of what L3 derives.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AReplacementAccepted {
    /// The generation the store now holds.
    pub generation: TokenGeneration,
    /// **Every machine now holding an older generation**, with its label beside
    /// its identifier and never in place of it. These are the machines cut off
    /// at this moment (L4's stated cost).
    pub left_on_an_old_generation: Vec<(String, Option<String>)>,
}

/// **L4: the machines a replacement leaves behind**, by L3's derivation over the
/// one told thing a replacement moves.
pub fn the_machines_left_on_an_old_generation(
    fleet: &[AMachineAndWhatItWasTold],
    generation: TokenGeneration,
) -> Vec<(String, Option<String>)> {
    let now = [WhatSmoozyTells::the_token_generation(generation)];
    fleet
        .iter()
        .filter(|machine| !what_has_not_reached(machine, &now).is_empty())
        .map(|machine| (machine.server.clone(), machine.label.clone()))
        .collect()
}

/// **§19's second act, *a replacement token accepted*: its fixed line form.**
/// The act's name; the moment on the desktop clock; the generation the store now
/// holds — a generation and never a token; and every machine left on the old
/// generation, one value inside the one line, with its label beside it.
pub const A_REPLACEMENT_TOKEN_ACCEPTED: &str =
    "a replacement token accepted; {moment}; generation {generation}; not reached: {machines}";

impl AReplacementAccepted {
    /// **The fixed form, filled.** One line whatever the fleet's size.
    pub fn the_diary_act(&self, at: SystemTime) -> String {
        let seconds = at
            .duration_since(SystemTime::UNIX_EPOCH)
            .map(|since| since.as_secs())
            .unwrap_or(0);
        let machines = if self.left_on_an_old_generation.is_empty() {
            "none".to_string()
        } else {
            self.left_on_an_old_generation
                .iter()
                .map(|(server, label)| match label {
                    Some(label) => format!("{server} ({label})"),
                    None => server.clone(),
                })
                .collect::<Vec<_>>()
                .join(", ")
        };
        A_REPLACEMENT_TOKEN_ACCEPTED
            .replace("{moment}", &seconds.to_string())
            .replace("{generation}", &self.generation.0.to_string())
            .replace("{machines}", &machines)
    }
}

// ------------------------------------------------------------ §19: one machine's telling

/// **§19's rows for a token replacement and a lock reaching a machine: one
/// ordered unit per machine** — placed on the machine, acknowledged, then the
/// told-record's mark.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AStepOfATelling {
    PlacedOnTheMachine,
    Acknowledged,
    MarkedInTheToldRecord,
}

impl AStepOfATelling {
    pub const IN_ORDER: [AStepOfATelling; 3] = [
        AStepOfATelling::PlacedOnTheMachine,
        AStepOfATelling::Acknowledged,
        AStepOfATelling::MarkedInTheToldRecord,
    ];
}

/// **How far one machine's telling got.** A failure at one step reports every
/// later step as **not attempted**, naming the step that stopped it (`State,
/// identity and recovery` P4) — never as failed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HowFarATellingGot {
    pub done: Vec<AStepOfATelling>,
    pub failed: Option<(AStepOfATelling, String)>,
    pub not_attempted: Vec<AStepOfATelling>,
}

impl HowFarATellingGot {
    pub fn all_done() -> HowFarATellingGot {
        HowFarATellingGot {
            done: AStepOfATelling::IN_ORDER.to_vec(),
            failed: None,
            not_attempted: Vec::new(),
        }
    }

    pub fn stopped_at(step: AStepOfATelling, why: impl Into<String>) -> HowFarATellingGot {
        let at = AStepOfATelling::IN_ORDER
            .iter()
            .position(|one| *one == step)
            .expect("every step is in order");
        HowFarATellingGot {
            done: AStepOfATelling::IN_ORDER[..at].to_vec(),
            failed: Some((step, why.into())),
            not_attempted: AStepOfATelling::IN_ORDER[at + 1..].to_vec(),
        }
    }

    pub fn reached_the_mark(&self) -> bool {
        self.done.contains(&AStepOfATelling::MarkedInTheToldRecord)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    fn at(seconds: u64) -> SystemTime {
        SystemTime::UNIX_EPOCH + Duration::from_secs(seconds)
    }

    fn a_machine(server: &str, installation: Option<&str>, told: Vec<AToldEntry>) -> AMachineAndWhatItWasTold {
        AMachineAndWhatItWasTold {
            server: server.into(),
            label: Some(format!("{server}'s label")),
            installation: installation.map(str::to_string),
            told,
        }
    }

    #[test]
    fn the_paid_policy_is_no_longer_a_told_thing() {
        assert_eq!(AToldThing::ALL.len(), 5);
        assert_eq!(AToldThing::stored_as_this("the lock policy"), None);
    }

    #[test]
    fn every_thing_has_one_spelling_and_reads_back() {
        for thing in AToldThing::ALL {
            assert_eq!(AToldThing::stored_as_this(thing.stored_as()), Some(thing));
        }
        assert_eq!(AToldThing::stored_as_this("anything else"), None);
    }

    /// **L1 and `State, identity and recovery` D4: an entry is what Smoozy told,
    /// and the value is the one Smoozy composed.**
    #[test]
    fn an_entry_is_made_from_what_smoozy_told_and_nothing_else() {
        let told = WhatSmoozyTells::the_token_generation(TokenGeneration(3));
        let entry = AToldEntry::acknowledged(&told, at(10), "inst-1");
        assert_eq!(entry.thing, AToldThing::TheTokenGeneration);
        assert_eq!(entry.value, "3");
        assert_eq!(entry.acknowledged_at, at(10));
        assert_eq!(entry.installation, "inst-1");
    }

    #[test]
    fn the_same_set_in_another_order_is_one_value() {
        assert_ne!(WhatSmoozyTells::the_label(None), WhatSmoozyTells::the_label(Some("")));
        assert_eq!(
            WhatSmoozyTells::the_reporting_settings([("y", true), ("x", false)]).value(),
            "x off, y on"
        );
    }

    /// **L3: computed from the current values and the told values, per thing.**
    #[test]
    fn a_machine_has_not_been_reached_by_whatever_differs() {
        let now = [
            WhatSmoozyTells::the_roles_desired_state(true),
            WhatSmoozyTells::the_token_generation(TokenGeneration(2)),
        ];
        let told = vec![
            AToldEntry::acknowledged(&WhatSmoozyTells::the_roles_desired_state(true), at(1), "i"),
            AToldEntry::acknowledged(&WhatSmoozyTells::the_token_generation(TokenGeneration(1)), at(1), "i"),
        ];
        let machine = a_machine("one", Some("i"), told);
        assert_eq!(what_has_not_reached(&machine, &now), vec![AToldThing::TheTokenGeneration]);
    }

    /// **An entry told under another installation counts for nothing**, and a
    /// machine with no confirmed installation has been told nothing.
    #[test]
    fn an_entry_counts_only_under_the_installation_held_now() {
        let now = [WhatSmoozyTells::the_roles_desired_state(true)];
        let told = vec![AToldEntry::acknowledged(&now[0], at(1), "old")];
        assert_eq!(
            what_has_not_reached(&a_machine("one", Some("new"), told.clone()), &now),
            vec![AToldThing::TheRolesDesiredState]
        );
        assert_eq!(
            what_has_not_reached(&a_machine("one", None, told.clone()), &now),
            vec![AToldThing::TheRolesDesiredState]
        );
        assert!(what_has_not_reached(&a_machine("one", Some("old"), told), &now).is_empty());
    }

    /// **L4 over two machines both told generation 1: a replacement leaves both
    /// behind, and the line names both in one line.**
    #[test]
    fn a_replacement_marks_every_machine_as_holding_the_old_generation() {
        let first = WhatSmoozyTells::the_token_generation(TokenGeneration::THE_FIRST);
        let fleet = vec![
            a_machine("one", Some("i1"), vec![AToldEntry::acknowledged(&first, at(1), "i1")]),
            a_machine("two", Some("i2"), vec![AToldEntry::acknowledged(&first, at(2), "i2")]),
        ];
        assert!(the_machines_left_on_an_old_generation(&fleet, TokenGeneration::THE_FIRST).is_empty());

        let next = TokenGeneration::THE_FIRST.the_next().unwrap();
        let left = the_machines_left_on_an_old_generation(&fleet, next);
        assert_eq!(left.len(), 2);
        let accepted = AReplacementAccepted { generation: next, left_on_an_old_generation: left };
        assert_eq!(
            accepted.the_diary_act(at(1_789_300_000)),
            "a replacement token accepted; 1789300000; generation 2; not reached: one (one's label), two (two's label)"
        );

        let fleet_reached = the_machines_not_reached(&fleet, |_| vec![first.clone()]);
        assert!(fleet_reached.is_empty(), "told the current generation, nobody is unreached");
    }

    #[test]
    fn a_generation_that_cannot_go_on_is_refused_rather_than_wrapped() {
        assert_eq!(TokenGeneration(u64::MAX).the_next(), None);
    }

    /// **§19 and `State, identity and recovery` P4: a step that failed leaves
    /// every later step not attempted, never failed.**
    #[test]
    fn a_telling_stopped_early_reports_the_later_steps_as_not_attempted() {
        let stopped = HowFarATellingGot::stopped_at(AStepOfATelling::PlacedOnTheMachine, "the path would not take");
        assert!(stopped.done.is_empty());
        assert_eq!(stopped.not_attempted, vec![AStepOfATelling::Acknowledged, AStepOfATelling::MarkedInTheToldRecord]);
        assert!(!stopped.reached_the_mark());
        assert!(HowFarATellingGot::all_done().reached_the_mark());
    }
}
