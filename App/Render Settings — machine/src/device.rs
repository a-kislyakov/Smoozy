//! What the render runs on — `Render Settings` §8: imposing the choice
//! (D1–D4) and the processor-on-a-machine-with-cards warning (D5–D7).
//!
//! **Constraint:** Row 157's first half; the second half — making the
//! machine's cards available — is `Rendering`'s Rows 8 and 9, answered there
//! and cited. Row 158: the condition is a fact about the connected machine;
//! the warning can state the setting and what it means and cannot promise the
//! outcome — the spec's own bound.
//!
//! **Markers used in this file that belong to another document, each with the
//! document it belongs to.** `Rendering` L21. `Servers` H1, H1a, H2, H5,
//! H11, H12, H12b, H15, H18, H21, K6b.

use std::time::SystemTime;

use serde::{Deserialize, Serialize};
use smoozy_servers_values::hardware::{card_count, Cards};

use crate::reading::{DeviceSetting, SceneReading};
use crate::readings::{HeldHardware, HeldState};
use crate::samples::{MissingSpelling, PairValue, SettingsPair};
use crate::spellings::{SCENE_DEVICE_SETTING, THE_PROCESSOR_VALUE};

/// The user's choice — cards or processor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DeviceChoice {
    TheCards,
    TheProcessor,
}

/// **D1: the payload's device entry is the user's choice where one exists,
/// and no entry where the file's own stands** (J4). What `Rendering` L21
/// reads is the resolved choice: the user's entry, or the reading's scene
/// value where none. **And the engine, since 2026-11-02**
/// ([`crate::engine::the_job_will_use_cycles`]): only a Cycles job names a
/// technology.
pub fn the_resolved_choice(
    the_users_entry: Option<DeviceChoice>,
    scene: &SceneReading,
) -> DeviceChoice {
    the_users_entry.unwrap_or(match scene.device {
        DeviceSetting::TheCards => DeviceChoice::TheCards,
        DeviceSetting::TheProcessor => DeviceChoice::TheProcessor,
    })
}

/// **D2: the cards side is `Rendering` L21's, cited whole** — for a Cycles job
/// the launch names the technology in the device argument after the bare double dash,
/// looked up from the card make by that document's table; the argument beats
/// the file and the preferences in both directions, and the loud refusals are
/// Rows 8 and 9's ground. **Nothing here composes the argument.**
pub const THE_CARDS_SIDE_IS_RENDERINGS: bool = true;

/// **D3: the processor side, where it is the user's override of a scene that
/// says cards, is imposed as a settings pair** — the scene's own device
/// setting assigned to the processor in the launch material, inside the
/// window. The device argument is not used for it, keeping L21 exactly as
/// written. **Where the scene's own value is already the processor and the
/// user typed nothing, nothing is imposed at all and the file renders as
/// saved.** Which scene *the scene's own device setting* names is C8's — the
/// job's scene, never another.
///
/// **The spelling landed on 2026-09-20 and §13 item 29 is closed**: it is
/// `device` on the job's scene's Cycles settings, `scene.cycles.device`, the
/// same in all four versions, **and the processor value is `CPU`** (C10, the
/// lookup of that date). Until then the pair could not be composed at all and
/// every attempt of every processor-override job failed, loudly, naming the
/// missing spelling — never a silent render on the cards. **The near miss is
/// named at C10 and matters here**: `scene.render.compositor_device` takes
/// the same two values on the same scene and decides how compositing is
/// carried out; a pair assigned against it would be accepted in silence while
/// the render went on using the cards. **And the route itself is still
/// unobserved** (§13 item 21), which the spelling landing does not change: assigning that scene setting from a script
/// handed in at launch is not among *Settings 9*'s three routes, and what
/// breaks if it does not take is the user's processor override ignored, the
/// scene rendering on the cards, the run exiting 0 with nothing said — a
/// silence not removable from here, because the one thing that would make it
/// loud is naming the technology on the launch line, which is L21's and which
/// D3 exists precisely to avoid.
pub fn the_processor_pair(
    the_users_entry: Option<DeviceChoice>,
    scene: &SceneReading,
) -> Result<Option<SettingsPair>, MissingSpelling> {
    match (the_users_entry, scene.device) {
        (Some(DeviceChoice::TheProcessor), DeviceSetting::TheCards) => Ok(Some(SettingsPair {
            property: SCENE_DEVICE_SETTING.written_out()?,
            // **The processor value is `CPU`**, matched exactly, letter case
            // included (C10, the lookup of 2026-09-20). It was the absence's
            // own placeholder until that date.
            value: PairValue::Identifier(THE_PROCESSOR_VALUE),
        })),
        _ => Ok(None),
    }
}

/// **D4: one residue is unestablished and carried** — whether a machine's
/// saved preferences can ever pull a processor-set scene onto the cards when
/// no technology is named at launch (§13 item 11, permanently unresolved).
/// The warning below never promises the outcome for exactly this reason.
pub const THE_WARNING_PROMISES_NO_OUTCOME: bool = true;

/// **D5–D6: the warning, or why it is not shown.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Warning {
    Shown { presence: Presence, tense: Tense },
    NotShown(WhyNotShown),
}

/// What the warning may state about the cards.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Presence {
    /// The cards, in full, with the count read as a count (`Servers` H5) —
    /// the Design document's tag.
    TheCardsInFull { count: usize },
    /// Cards present and cannot be read — the driver absent, or answering
    /// none: the warning states presence alone and promises nothing about
    /// what the driver would say. **No count is shown**: `Servers` H5 makes
    /// the count the rows of a per-card query and nothing else is read as
    /// one.
    PresentAndCannotBeRead,
}

/// **D6's two columns**, keyed on the connection the held reading was taken
/// on and never on the period (`Servers` H1a) — so an earlier connection's
/// reading falls to *last seen* even where a connection is live now, which is
/// H18's fourth branch (`Servers` H15, H21). **A reading's currency comes from
/// its own moment and never from the column it sits in** (`Servers` K6b).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tense {
    /// The held reading was taken on the period's primary connection and that
    /// connection is live now.
    Live,
    /// A reading is held and the *live* condition does not hold: shown as
    /// what was **last seen**, with the reading's moment, never as current
    /// (`Servers` H2). The moment is `Servers` H2a's desktop-side one, which
    /// is the one a *last seen* mark is drawn from.
    LastSeen { the_readings_moment: SystemTime },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhyNotShown {
    /// **D5: nothing about the file's insides shows before pre-load.**
    NoReadingOfTheFileStands,
    /// The device in force is the cards, so a scene rendering on the
    /// processor is not what is happening.
    TheDeviceInForceIsTheCards,
    /// **Under *never read* nothing shows in any card state**, because there
    /// is no card state — the absence of a reading Smoozy may show.
    TheHardwareWasNeverRead,
    /// **No card** — the bus scan empty and the driver absent or answering
    /// none (`Servers` H11 row 4): the warning's condition is a machine that
    /// has cards, and this reading says it has none.
    TheReadingSaysNoCard,
    /// **The cards could not be established** — on a container (`Servers`
    /// H12), or not on a container where the bus scan has no route (H11 row
    /// 5, `Servers` H12b): the reading never claims either way there, and
    /// neither does the page.
    TheCardsCouldNotBeEstablished,
}

/// **D5: the warning composes from two readings and is shown only where both
/// halves stand** — the device the scene would render on, which needs the
/// file read, and the hardware reading, which is `Servers`' remembered one.
/// **D6: it is written across both of `Servers`' axes at once** — the card
/// state the reading carries (H11–H12) and the reading's state under H1 —
/// because they are independent and the page needs an answer for every pair.
/// **No row is written in the present tense off a *last seen* reading.**
///
/// The device the warning judges is the resolved one (D1): the user's own
/// entry where it stands, else the scene's read value — either way a reading
/// of the file must stand, which is what D5's first half requires.
///
/// **The page must not call this under EEVEE.** `Preparation/Design/Render
/// Settings.md` — *"Under EEVEE the pair is not shown at all"* — so under
/// EEVEE the control is absent and the warning is never reached. That is a
/// page-visibility rule this crate does not own, so there is no engine
/// argument here and no not-shown reason for it: the closed set below is
/// written from D6's table, and a row no clause states would be an
/// unrecorded state.
pub fn the_warning(
    the_users_entry: Option<DeviceChoice>,
    scene: Option<&SceneReading>,
    hardware: &HeldHardware,
) -> Warning {
    let Some(scene) = scene else {
        return Warning::NotShown(WhyNotShown::NoReadingOfTheFileStands);
    };
    if the_resolved_choice(the_users_entry, scene) == DeviceChoice::TheCards {
        return Warning::NotShown(WhyNotShown::TheDeviceInForceIsTheCards);
    }
    let HeldHardware::Held { state, reading } = hardware else {
        return Warning::NotShown(WhyNotShown::TheHardwareWasNeverRead);
    };
    let presence = match &reading.cards {
        Cards::TheCards(_) => Presence::TheCardsInFull {
            count: card_count(&reading.cards).expect("the cards in full carry a count"),
        },
        Cards::PresentAndUnreadable(_) => Presence::PresentAndCannotBeRead,
        Cards::NoCard => return Warning::NotShown(WhyNotShown::TheReadingSaysNoCard),
        Cards::CouldNotBeEstablished => {
            return Warning::NotShown(WhyNotShown::TheCardsCouldNotBeEstablished)
        }
    };
    let tense = match state {
        HeldState::Live => Tense::Live,
        HeldState::LastSeen => Tense::LastSeen {
            the_readings_moment: reading.the_desktops_own_moment.taken_at(),
        },
    };
    Warning::Shown { presence, tense }
}

/// **D7: what the warning inherits and names** — the no-card half of Fact 46
/// is not answered and is unposable on the provider tested, so the genuinely
/// cardless reading is argued from the bus scan rather than observed; the
/// multi-card container reading is explicitly still open. Both at §13 item
/// 12, permanently unresolved; the promise merely gets weaker and closes as
/// unresolved. The per-state table above never claims what the reading
/// cannot.
pub const THE_CARDLESS_DISCRIMINATION_IS_ARGUED_AND_NOT_OBSERVED: bool = true;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::file_half::tests::{a_moment, a_scene};
    use smoozy_servers_values::hardware::{Card, CardMake, HardwareReading, MachineMemory, WhyUnreadable};

    fn held(state: HeldState, cards: Cards) -> HeldHardware {
        HeldHardware::Held {
            state,
            reading: HardwareReading::taken(
                cards,
                MachineMemory::TheWholeMachine(64 * 1024 * 1024 * 1024),
                None,
                a_moment(),
                "server-1",
                a_moment(),
                None,
            ),
        }
    }

    fn two_cards() -> Cards {
        Cards::TheCards(
            (0..2)
                .map(|at| Card {
                    unique_identifier: format!("GPU-{at}"),
                    bus_location: format!("0000:0{at}:00.0"),
                    make: CardMake::from_the_drivers_own_query("NVIDIA"),
                    total_memory_bytes: None,
                })
                .collect(),
        )
    }

    fn a_cards_scene() -> SceneReading {
        a_scene("Kitchen")
    }

    #[test]
    fn the_resolved_choice_is_the_users_entry_else_the_scenes_own() {
        let scene = a_cards_scene();
        assert_eq!(the_resolved_choice(None, &scene), DeviceChoice::TheCards);
        assert_eq!(
            the_resolved_choice(Some(DeviceChoice::TheProcessor), &scene),
            DeviceChoice::TheProcessor
        );
    }

    /// **The lookup landed on 2026-09-20, so the override composes** — the
    /// scene's own Cycles device setting, assigned to `CPU`. This test held
    /// the refusal until that date. **The near miss is checked by name**:
    /// `scene.render.compositor_device` takes the same two values on the same
    /// scene and would be accepted in silence while the render used the cards.
    #[test]
    fn a_processor_override_composes_against_the_scenes_own_cycles_device() {
        let scene = a_cards_scene();
        let pair = the_processor_pair(Some(DeviceChoice::TheProcessor), &scene)
            .expect("C10 carries D3's spelling")
            .expect("an override against a scene set to the cards is a pair");
        assert_eq!(pair.property, "scene.cycles.device");
        assert_ne!(pair.property, "scene.render.compositor_device");
        assert_eq!(pair.value, PairValue::Identifier("CPU"));
        // Where nothing is to be imposed, nothing is — whatever the spelling.
        assert_eq!(the_processor_pair(None, &scene), Ok(None));
        assert_eq!(
            the_processor_pair(Some(DeviceChoice::TheCards), &scene),
            Ok(None)
        );
        let mut processor_scene = a_cards_scene();
        processor_scene.device = DeviceSetting::TheProcessor;
        assert_eq!(
            the_processor_pair(Some(DeviceChoice::TheProcessor), &processor_scene),
            Ok(None)
        );
    }

    #[test]
    fn the_warning_needs_the_file_read_and_the_device_in_force_to_be_the_processor() {
        let scene = a_cards_scene();
        assert_eq!(
            the_warning(
                Some(DeviceChoice::TheProcessor),
                None,
                &held(HeldState::Live, two_cards())
            ),
            Warning::NotShown(WhyNotShown::NoReadingOfTheFileStands)
        );
        assert_eq!(
            the_warning(None, Some(&scene), &held(HeldState::Live, two_cards())),
            Warning::NotShown(WhyNotShown::TheDeviceInForceIsTheCards)
        );
    }

    #[test]
    fn the_table_answers_every_pair_of_card_state_and_reading_state() {
        let scene = a_cards_scene();
        let processor = Some(DeviceChoice::TheProcessor);

        assert_eq!(
            the_warning(processor, Some(&scene), &held(HeldState::Live, two_cards())),
            Warning::Shown {
                presence: Presence::TheCardsInFull { count: 2 },
                tense: Tense::Live
            }
        );
        assert_eq!(
            the_warning(
                processor,
                Some(&scene),
                &held(HeldState::LastSeen, two_cards())
            ),
            Warning::Shown {
                presence: Presence::TheCardsInFull { count: 2 },
                tense: Tense::LastSeen {
                    the_readings_moment: a_moment()
                }
            }
        );
        for why in [
            WhyUnreadable::TheDriverIsAbsent,
            WhyUnreadable::TheDriverAnsweredWithNone,
        ] {
            assert_eq!(
                the_warning(
                    processor,
                    Some(&scene),
                    &held(HeldState::Live, Cards::PresentAndUnreadable(why))
                ),
                Warning::Shown {
                    presence: Presence::PresentAndCannotBeRead,
                    tense: Tense::Live
                }
            );
        }
        assert_eq!(
            the_warning(
                processor,
                Some(&scene),
                &held(HeldState::Live, Cards::NoCard)
            ),
            Warning::NotShown(WhyNotShown::TheReadingSaysNoCard)
        );
        assert_eq!(
            the_warning(
                processor,
                Some(&scene),
                &held(HeldState::LastSeen, Cards::CouldNotBeEstablished)
            ),
            Warning::NotShown(WhyNotShown::TheCardsCouldNotBeEstablished)
        );
        assert_eq!(
            the_warning(processor, Some(&scene), &HeldHardware::NeverRead),
            Warning::NotShown(WhyNotShown::TheHardwareWasNeverRead)
        );
    }
}
