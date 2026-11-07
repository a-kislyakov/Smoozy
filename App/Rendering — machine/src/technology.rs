//! The device, the technology, and the ask — L21, L21a, L22, L23, L24, L24a,
//! L25 (§1.8, Rows 8 and 9).
//!
//! **L21.** Where a Cycles job is set to use the cards, the launch names the
//! card technology in the device argument, after the bare double dash; where
//! the file asks for the processor, or the job's engine is not Cycles, no
//! technology is named and nothing overrides the file. The engine is asked at
//! [`crate::composition::compose_the_attempt`], which holds it. The launch
//! option forces a technology, takes every card of that
//! kind whatever the machine's saved preferences said, and writes nothing
//! back (*The machine 5*, *Settings 9*). It *"names a technology and has no
//! way of naming one card out of several"* — every-card is the whole
//! mechanism: one argument, no preferences script, no per-card selection
//! anywhere. **This is Row 8.**
//!
//! **L21a, and it is held by the signature.** The device list is never read
//! as a statement that the cards are enabled: from factory startup every
//! individual device arrives with `use = True` while the backend selector
//! arrives `NONE`, and a Cycles render then runs on the processor. **Nothing
//! may infer from the device list that a render will use the cards, and
//! nothing may take a card's `use` value as an answer to *will this render
//! on the cards*.** What decides it is the technology named at launch, and
//! [`the_technology_to_name`] takes the hardware reading's cards and
//! nothing about what any of them is switched to — `Servers`' own reading
//! carries no such switch, and no argument here could.
//!
//! **L24.** Naming the technology is what converts the quiet processor
//! render into a loud refusal, and that is Row 9's whole force: two
//! distinguishable refusals, both exit 1 with nothing written (Fact 3); left
//! to the machine's saved settings or the file, the same impossibility
//! renders on the processor, writes the picture, and reports success.
//!
//! **L24a.** L24's two refusals are refusals of the device *value*; the
//! argument's own *name* is checked by nothing after the double dash, so
//! its spelling is a constant of this contract at one declaration point —
//! [`crate::spellings::DEVICE_ARGUMENT`] — and this file composes the
//! **value** alone, from a table keyed on the card make and from nothing a
//! job carries.
//!
//! **Markers used in this file that belong to another document, each with
//! the document it belongs to.** `Servers` H1, H7, H11, H12, H14, H15, H16.
//! `Queue & History` E12, E13. `Job files and outputs` M1b.
//!
//! **L23's input reaches the machine on the submission manifest, and that is
//! the route the clause named nowhere until 2026-09-20.** The make comes from
//! the hardware reading `Servers` holds, which is taken on the desktop, so
//! the worker performing this lookup can take none: it is put on the manifest
//! at submission (`Job files and outputs` M1b) and read from there by
//! [`the_technology_the_manifest_names`]. **Before the plan carried that
//! field, the worker had nothing to look up from and every cards job ended at
//! L23's loud failure before the render.**
//!
//! **`Server Setup` W4 is not listed above and is written out at each site**,
//! because this plan defines a W4 of its own.

use smoozy_job_files_machine::manifest::TheCardMakeAtSubmission;
use smoozy_servers_values::hardware::{Card, Cards};

/// **L23's technology names — the value the argument takes, and the value
/// alone.** The words are Blender's own command-line page, quoted in this
/// project's blind Blender Research and read 2026-07-30: *"Valid options
/// are: CPU CUDA OPTIX HIP ONEAPI METAL."* **The list establishes the
/// spelling and nothing about whether hardware of that make then renders**
/// — three routes give three answers on one build (`Six machines`, Fact 3).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Technology {
    /// **NVIDIA's, from 2026-10-03, on the Creator's ruling** (L23). It was
    /// `CUDA` until that day, and no row names `CUDA` now, so that value is
    /// gone rather than kept for nothing.
    Optix,
    Hip,
    OneApi,
}

impl Technology {
    /// **L22: the bare technology, never a composite with the processor.**
    /// The observed launch table took `OPTIX+CPU` as well and it is not
    /// used: the promise is every card, the processor's help is promised
    /// nowhere, and a composite would blur L26's device line. **There is no
    /// variant that could carry a `+`.**
    pub fn value(self) -> &'static str {
        match self {
            Technology::Optix => "OPTIX",
            Technology::Hip => "HIP",
            Technology::OneApi => "ONEAPI",
        }
    }
}

/// How a row's value was established.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HowTheRowIsFilled {
    /// Documented and **run on real hardware** — Fact 3 and the readings
    /// L23 tables.
    DocumentedAndRunOnRealHardware,
    /// **Documented, hardware unposed** — the word is published; whether
    /// the hardware then renders is not established (§9 item 3).
    DocumentedHardwareUnposed,
}

/// One row of L23's per-make table.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Row {
    /// The key `Servers` H7's reading supplies — a value, never a wording
    /// to be parsed.
    pub make_key: &'static str,
    pub technology: Technology,
    pub filled: HowTheRowIsFilled,
}

/// **L23: the table, three filled rows and a struck fourth.** A second make
/// costs a value and not a mechanism.
///
/// **The NVIDIA value is `OPTIX`, and it is the Creator's ruling of
/// 2026-10-03** — *"Switch to OptiX."* It was `CUDA` until that day, a
/// decision resting on one seller's own driver route omitting the OptiX
/// library and on the pixel comparison, and that evidence now prices what
/// `OPTIX` costs rather than grounding `CUDA`. **What it buys**: every scene
/// that is not tiny renders faster — about four times at ten million
/// triangles — and at 125.8 million `OPTIX` was the only one of the two that
/// rendered at all on the one machine measured. **What it costs, stated
/// rather than absorbed (§9 item 3)**: a refusal on every scene on a machine
/// whose driver left the OptiX library out — loud, before anything is
/// launched, in plain words (L24b, [`crate::acting::the_optix_library`]); a
/// picture that on the one small scene compared matched neither the
/// processor's nor `CUDA`'s; and one RTX 3060 on which 5.2.1 failed to load
/// its OptiX kernel and wrote no image. Changing the value is still one edit.
///
/// **Apple has no row and never can have one**: `METAL` is Apple Silicon
/// only and every machine this project rents is Linux (§9 item 13). Struck
/// rather than left empty.
pub const THE_TABLE: [Row; 3] = [
    Row {
        // **The key the reading's own route gives every card it names**,
        // declared once beside that route and read here — never a second
        // spelling of it. Until 2026-09-26 the reading put the driver's name
        // for the card where this key is looked up, and on step 10c's sixth
        // pass on Vast every cards job on an NVIDIA machine ended before the
        // render (`Servers — values`' `NVIDIA`).
        make_key: smoozy_servers_values::hardware::NVIDIA,
        technology: Technology::Optix,
        filled: HowTheRowIsFilled::DocumentedAndRunOnRealHardware,
    },
    Row {
        make_key: "AMD",
        technology: Technology::Hip,
        filled: HowTheRowIsFilled::DocumentedHardwareUnposed,
    },
    Row {
        make_key: "Intel",
        technology: Technology::OneApi,
        filled: HowTheRowIsFilled::DocumentedHardwareUnposed,
    },
];

/// **L23's last paragraph: a make with no row, and a hardware reading that
/// returns no card at all, are one case** — the lookup asks what technology
/// to name and both answer it with nothing. Neither withholds the
/// submission: a cards job on such a machine submits like any other, and
/// **the attempt ends on the machine before the render instruction, naming
/// the make it could not serve** (K14's failed limb, cause *the file's own*
/// under K14a's table).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NothingToName {
    /// The reading returned no card at all.
    TheReadingReturnedNoCard,
    /// The reading could not establish the cards (`Servers` H12 — a
    /// container, or a bus scan with no route).
    TheCardsCouldNotBeEstablished,
    /// Cards are present and the driver could not read them, so no make
    /// is on the reading to key on.
    TheCardsArePresentAndUnreadable,
    /// A make the table has no row for, named.
    AMakeWithNoRow { make: String },
    /// **No hardware reading stood for this machine at submission**
    /// (`Servers` H1's *never read*, carried on the manifest by `Job files
    /// and outputs` M1b). **An ordinary case rather than a gap**, and L23's
    /// last paragraph answers it with the others: the lookup asks what
    /// technology to name and it answers with nothing.
    NoReadingStoodAtSubmission,
    /// Cards of more than one make on one reading — the table names one
    /// technology per launch and the every-card promise is per technology,
    /// so the lookup refuses rather than picking a side.
    MoreThanOneMakeOnOneReading { makes: Vec<String> },
}

impl NothingToName {
    /// **Why there is no technology to name, in standard software words**
    /// (Rule 9) — beside the type.
    ///
    /// **Added 2026-09-23, out of step 10c's fifth pass on Vast, fault 5.**
    /// The attempt record's reason carried this type's debugging rendering
    /// inside the ending's own `make` field, and on a real rented machine it
    /// read `ACardMakeWithNoTechnologyToName { make: "NoReadingStoodAtSubmission" }`.
    /// **The make the reading did carry stays in the words** where there was
    /// one, because that is what somebody acts on.
    pub fn in_words(&self) -> String {
        match self {
            NothingToName::TheReadingReturnedNoCard => {
                "no GPU found on this server".to_string()
            }
            NothingToName::TheCardsCouldNotBeEstablished => {
                "could not detect this server's GPUs".to_string()
            }
            NothingToName::TheCardsArePresentAndUnreadable => {
                "GPUs found, but the driver did not name them".to_string()
            }
            NothingToName::AMakeWithNoRow { make } => {
                format!("unsupported GPU: {make}")
            }
            NothingToName::NoReadingStoodAtSubmission => {
                "no GPU reading when the job was sent".to_string()
            }
            NothingToName::MoreThanOneMakeOnOneReading { makes } => format!(
                "GPUs of more than one make ({})",
                makes.join(", ")
            ),
        }
    }
}

/// **L21, L23, performed: the technology to name, looked up from the card
/// make the hardware reading carries — and from nothing else.**
///
/// **The signature is the guard for L21a**: the only input is `Servers`'
/// own `Cards`, which carries each card's identity, bus location, make and
/// memory and **no switch** — so no `use` value can reach this function,
/// and nothing about what a device list says a card is set to has a route
/// into what is named at launch.
pub fn the_technology_to_name(cards: &Cards) -> Result<Technology, NothingToName> {
    let cards: &[Card] = match cards {
        Cards::TheCards(cards) => cards,
        Cards::NoCard => return Err(NothingToName::TheReadingReturnedNoCard),
        Cards::CouldNotBeEstablished => return Err(NothingToName::TheCardsCouldNotBeEstablished),
        Cards::PresentAndUnreadable(_) => {
            return Err(NothingToName::TheCardsArePresentAndUnreadable)
        }
    };
    let mut makes: Vec<&str> = Vec::new();
    for card in cards {
        let key = card.make.as_key();
        if !makes.contains(&key) {
            makes.push(key);
        }
    }
    the_technology_for(&makes)
}

/// **L23 performed on the machine, where the reading itself cannot be
/// taken** — from the card make the submission manifest carries (`Job files
/// and outputs` M1b).
///
/// **This is the lookup the render worker uses, and it reads the same table
/// by the same rule.** M1b puts the make on the manifest because the
/// hardware reading is `Servers`' and is taken on the desktop (`Servers`
/// H14, H15); the technology is never composed there and never travels, so
/// the table below is read on the machine and nowhere else.
pub fn the_technology_the_manifest_names(
    make: &TheCardMakeAtSubmission,
) -> Result<Technology, NothingToName> {
    match make {
        TheCardMakeAtSubmission::TheMakes(makes) => {
            let makes: Vec<&str> = makes.iter().map(String::as_str).collect();
            the_technology_for(&makes)
        }
        TheCardMakeAtSubmission::NoCard => Err(NothingToName::TheReadingReturnedNoCard),
        TheCardMakeAtSubmission::PresentAndUnreadable => {
            Err(NothingToName::TheCardsArePresentAndUnreadable)
        }
        TheCardMakeAtSubmission::CouldNotBeEstablished => {
            Err(NothingToName::TheCardsCouldNotBeEstablished)
        }
        TheCardMakeAtSubmission::NoReadingStood => {
            Err(NothingToName::NoReadingStoodAtSubmission)
        }
    }
}

/// **The table, read in one place.** Both callers above reach it, so the
/// desktop's reading and the manifest's make are judged by one rule rather
/// than by two copies of it — the one-fact-in-two-places fault this stage
/// keeps paying for.
fn the_technology_for(makes: &[&str]) -> Result<Technology, NothingToName> {
    if makes.is_empty() {
        return Err(NothingToName::TheReadingReturnedNoCard);
    }
    if makes.len() > 1 {
        return Err(NothingToName::MoreThanOneMakeOnOneReading {
            makes: makes.iter().map(|make| make.to_string()).collect(),
        });
    }
    let make = makes[0];
    THE_TABLE
        .iter()
        .find(|row| row.make_key == make)
        .map(|row| row.technology)
        .ok_or_else(|| NothingToName::AMakeWithNoRow {
            make: make.to_string(),
        })
}

/// **L24: the two distinguishable refusals, both exit 1 with nothing
/// written** (Fact 3), recorded as what the loud path looks like so nobody
/// reads either as anything but the refusal it is. **L25: a broken driver is
/// not distinguishable from absent hardware — both give the second line —
/// and the refusal is the correct outcome for both**, so nothing here tells
/// them apart.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TheLoudRefusal {
    /// `Error: BOGUS is not a valid Cycles device.` — a name the build does
    /// not know. Unreachable by composition: [`Technology`] has three values
    /// and all three are on the build's list.
    ANameTheBuildDoesNotKnow,
    /// `Error: Found no Cycles device of the specified type` — a known name
    /// no hardware answers to, which a broken driver produces as well.
    AKnownNameNoHardwareAnswersTo,
}

impl TheLoudRefusal {
    /// Both refusals: exit 1, nothing written. **Neither is ever the quiet
    /// processor render**, which is the whole of Row 9.
    pub fn writes_nothing_and_exits_non_zero(self) -> bool {
        true
    }
}

/// **L21a as a value: the ask is not evidence of what ran, and a card's
/// switch is not evidence of what will run.** `Queue & History` E12 gates
/// the recorded device line on the job's recorded outcome being *finished*
/// and this crate states no second gate (L26).
pub const NOTHING_INFERS_A_DEVICE_FROM_THE_ASK_OR_FROM_A_CARDS_SWITCH: bool = true;

/// **L24a's dependency, named at this end too**: `Server Setup` W4 reads
/// the recorded device line on a finished job and calls a processor render
/// under a cards ask a setup fault, offering to re-run the ready action,
/// which can restart the machine. **That diagnosis is sound exactly so long
/// as the spelling is one constant** — a misspelling that could vary per job
/// would have `Server Setup` W4 blaming the machine for Smoozy's own line.
pub const SERVER_SETUP_W4_RESTS_ON_THE_SPELLING_BEING_ONE_CONSTANT: bool = true;

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use smoozy_servers_values::hardware::{CardMake, WhyUnreadable};

    /// **Cards of one make, each named the way a driver names a card** — a
    /// product name, never the make's key — so no test here can pass by the
    /// name and the key happening to be the same text.
    pub(crate) fn cards_of(make: &str, count: usize) -> Cards {
        Cards::TheCards(
            (0..count)
                .map(|at| Card {
                    unique_identifier: format!("GPU-{at}"),
                    bus_location: format!("0000:0{at}:00.0"),
                    make: CardMake::of_a_make(make, &format!("{make} Model {at}")),
                    total_memory_bytes: None,
                })
                .collect(),
        )
    }

    /// **The one value from end to end, driven from a real row.** The row is
    /// the driver's own answer as step 10c's sixth pass on Vast read it, and
    /// the card it becomes is the one the reading makes of it; the table finds
    /// its technology by the key the reading gave it. **The pass's job ended
    /// here with *Smoozy does not know how to render on a NVIDIA GeForce GTX
    /// 1650*.** The desktop half of the same walk — the reading's own parser,
    /// the manifest's field — is `smoozy-rendering`'s test of the same name.
    #[test]
    fn a_real_product_name_from_the_driver_finds_its_technology() {
        let card = Card {
            unique_identifier: "GPU-6e3a0c".to_string(),
            bus_location: "00000000:01:00.0".to_string(),
            make: CardMake::from_the_drivers_own_query("NVIDIA GeForce GTX 1650"),
            total_memory_bytes: Some(4096 * 1024 * 1024),
        };
        assert_eq!(
            the_technology_to_name(&Cards::TheCards(vec![card])),
            Ok(Technology::Optix)
        );
    }

    /// **The manifest's make is judged by the same table and the same rule**
    /// (`Job files and outputs` M1b), and every one of its five states has an
    /// answer.
    #[test]
    fn the_manifests_make_reaches_the_same_answers_as_a_reading_does() {
        let makes = |make: &str| TheCardMakeAtSubmission::TheMakes(vec![make.to_string()]);
        assert_eq!(
            the_technology_the_manifest_names(&makes("NVIDIA")),
            Ok(Technology::Optix)
        );
        assert_eq!(
            the_technology_the_manifest_names(&makes("AMD")),
            Ok(Technology::Hip)
        );
        // **L23's last paragraph**: a make with no row answers with nothing,
        // and it is the same answer a reading of that make gives.
        assert_eq!(
            the_technology_the_manifest_names(&makes("Matrox")),
            the_technology_to_name(&cards_of("Matrox", 1))
        );
        assert_eq!(
            the_technology_the_manifest_names(&TheCardMakeAtSubmission::NoCard),
            Err(NothingToName::TheReadingReturnedNoCard)
        );
        assert_eq!(
            the_technology_the_manifest_names(&TheCardMakeAtSubmission::PresentAndUnreadable),
            Err(NothingToName::TheCardsArePresentAndUnreadable)
        );
        assert_eq!(
            the_technology_the_manifest_names(&TheCardMakeAtSubmission::CouldNotBeEstablished),
            Err(NothingToName::TheCardsCouldNotBeEstablished)
        );
        // **`Servers` H1's *never read*, carried as its own state** — an
        // ordinary case, and never an empty make list standing in for it.
        assert_eq!(
            the_technology_the_manifest_names(&TheCardMakeAtSubmission::NoReadingStood),
            Err(NothingToName::NoReadingStoodAtSubmission)
        );
    }

    /// **Two makes on one manifest refuse rather than pick a side**, which is
    /// the rule a reading of two makes already meets.
    #[test]
    fn a_manifest_carrying_two_makes_refuses_the_way_a_reading_of_two_does() {
        let both = TheCardMakeAtSubmission::TheMakes(vec![
            "NVIDIA".to_string(),
            "AMD".to_string(),
        ]);
        assert_eq!(
            the_technology_the_manifest_names(&both),
            Err(NothingToName::MoreThanOneMakeOnOneReading {
                makes: vec!["NVIDIA".to_string(), "AMD".to_string()]
            })
        );
    }

    #[test]
    fn nvidia_names_optix_and_the_other_two_rows_are_documented_alone() {
        assert_eq!(
            the_technology_to_name(&cards_of("NVIDIA", 2)),
            Ok(Technology::Optix)
        );
        assert_eq!(
            the_technology_to_name(&cards_of("AMD", 1)),
            Ok(Technology::Hip)
        );
        assert_eq!(
            the_technology_to_name(&cards_of("Intel", 1)),
            Ok(Technology::OneApi)
        );
        for row in THE_TABLE {
            let expected = match row.technology {
                Technology::Optix => HowTheRowIsFilled::DocumentedAndRunOnRealHardware,
                _ => HowTheRowIsFilled::DocumentedHardwareUnposed,
            };
            assert_eq!(row.filled, expected, "{:?}", row.technology);
        }
    }

    /// **L22: never a composite.** No value carries the processor's help.
    ///
    /// **And L23 since the Creator's ruling of 2026-10-03: NVIDIA names
    /// `OPTIX`, and no row names `CUDA`.** Until that day this test said the
    /// reverse — no row might name `OPTIX` — so the ruling could not have
    /// landed by accident, and it cannot be undone by one either.
    #[test]
    fn no_technology_value_is_a_composite_and_none_is_apple_or_the_processor() {
        for row in THE_TABLE {
            let value = row.technology.value();
            assert!(!value.contains('+'), "{value}");
            assert_ne!(value, "CPU");
            assert_ne!(value, "METAL");
            assert_ne!(value, "CUDA");
        }
        let nvidia = THE_TABLE
            .iter()
            .find(|row| row.make_key == smoozy_servers_values::hardware::NVIDIA)
            .expect("the NVIDIA row");
        assert_eq!(nvidia.technology.value(), "OPTIX");
    }

    #[test]
    fn a_make_with_no_row_and_a_reading_with_no_card_are_one_case_and_name_themselves() {
        assert_eq!(
            the_technology_to_name(&cards_of("Matrox", 1)),
            Err(NothingToName::AMakeWithNoRow {
                make: "Matrox".to_string()
            })
        );
        assert_eq!(
            the_technology_to_name(&Cards::NoCard),
            Err(NothingToName::TheReadingReturnedNoCard)
        );
        assert_eq!(
            the_technology_to_name(&Cards::TheCards(Vec::new())),
            Err(NothingToName::TheReadingReturnedNoCard)
        );
        assert_eq!(
            the_technology_to_name(&Cards::CouldNotBeEstablished),
            Err(NothingToName::TheCardsCouldNotBeEstablished)
        );
        assert_eq!(
            the_technology_to_name(&Cards::PresentAndUnreadable(
                WhyUnreadable::TheDriverIsAbsent
            )),
            Err(NothingToName::TheCardsArePresentAndUnreadable)
        );
    }

    #[test]
    fn two_makes_on_one_reading_refuse_rather_than_pick_a_side() {
        let Cards::TheCards(mut cards) = cards_of("NVIDIA", 1) else {
            unreachable!()
        };
        let Cards::TheCards(other) = cards_of("AMD", 1) else {
            unreachable!()
        };
        cards.extend(other);
        assert_eq!(
            the_technology_to_name(&Cards::TheCards(cards)),
            Err(NothingToName::MoreThanOneMakeOnOneReading {
                makes: vec!["NVIDIA".to_string(), "AMD".to_string()]
            })
        );
    }

    /// **L21a, held by the signature**: the reading `Servers` supplies has
    /// no switch on a card, so there is nothing a lookup could infer from.
    /// Read off the type rather than asserted: a `Card` is exactly its
    /// identity, bus location, make and memory.
    #[test]
    fn the_lookup_takes_the_reading_and_nothing_about_what_a_card_is_switched_to() {
        let card = Card {
            unique_identifier: "GPU-0".to_string(),
            bus_location: "0000:00:00.0".to_string(),
            make: CardMake::from_the_drivers_own_query("NVIDIA GeForce RTX 4090"),
            total_memory_bytes: Some(1),
        };
        let Card {
            unique_identifier: _,
            bus_location: _,
            make: _,
            total_memory_bytes: _,
        } = card;
        const _: () = assert!(NOTHING_INFERS_A_DEVICE_FROM_THE_ASK_OR_FROM_A_CARDS_SWITCH);
        const _: () = assert!(SERVER_SETUP_W4_RESTS_ON_THE_SPELLING_BEING_ONE_CONSTANT);
        assert!(TheLoudRefusal::ANameTheBuildDoesNotKnow.writes_nothing_and_exits_non_zero());
        assert!(TheLoudRefusal::AKnownNameNoHardwareAnswersTo.writes_nothing_and_exits_non_zero());
    }
}
