//! The engine — `Render Settings` §6 E1–E5.
//!
//! **Constraint:** Row 151, answered with `Rendering` Row 12 / L19 and not
//! separately. The engine name changed at 5.0 and which name a version takes
//! depends on the route; the version's own name works by every route; 4.5
//! given the newer name reports an error, writes nothing, and then ends badly
//! with no single symptom (*Settings 13*, *Images 9*).
//!
//! **Markers used in this file that belong to another document, each with the
//! document it belongs to.** `Rendering` L19, K15a. `Queue & History` R14.
//! `Server Setup` A5.

use serde::{Deserialize, Serialize};

use crate::file_half::RecordedReading;
use crate::names::{anchor_at_the_attempt, ExactName, NotFound, WhatIsAnchored};
use crate::reading::SceneReading;

/// **The two offered engines** — Smoozy offers Cycles and EEVEE (`Queue &
/// History` R14), and nothing else (J3).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum OfferedEngine {
    Cycles,
    Eevee,
}

/// **E1: the payload's engine entry is the user's pick — one of the two
/// offered — plus, where a reading of the chosen version stands, that
/// reading's own identifier for it** (Q2 item 4). The identifier is never
/// composed from a table of this document's, never carried across a version
/// boundary, and never the other version's name however forgiving one route
/// is: it is bytes the chosen version's own enumeration produced, which is
/// `Rendering` L19's requirement met by provenance.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EngineEntry {
    pub pick: OfferedEngine,
    /// The anchored identifier where one stands; **E2**: where none is
    /// anchored at submission the pick rides alone and is anchored at the
    /// attempt.
    pub identifier: Option<ExactName>,
}

/// **E1 and E3 together**: the entry where the user picked, **no entry where
/// the engine is untouched**, and no engine argument is composed for an
/// untouched engine — the file renders on its own recorded engine.
pub fn the_engine_entry(
    the_users_pick: Option<OfferedEngine>,
    standing: Option<&RecordedReading>,
) -> Option<EngineEntry> {
    let pick = the_users_pick?;
    Some(EngineEntry {
        pick,
        identifier: standing.and_then(|reading| the_readings_identifier_for(pick, reading)),
    })
}

/// The reading's own identifier for the pick, where that build registers
/// one.
pub fn the_readings_identifier_for(
    pick: OfferedEngine,
    reading: &RecordedReading,
) -> Option<ExactName> {
    let offered = &reading.settings.offered_engine_identifiers;
    match pick {
        OfferedEngine::Cycles => offered.cycles.clone(),
        OfferedEngine::Eevee => offered.eevee.clone(),
    }
}

/// **E2: a pick the reading in force cannot anchor — the engine's add-on is
/// off in that build, so its identifier is not registered — fails the
/// attempt before the render instruction, naming it.**
///
/// **What that refusal prevents is heavier than the alternative it was
/// written against** (corrected 2026-09-01): where the engine is named at
/// launch and its add-on is absent, *Add-ons 7* records no image written,
/// 5.0, 5.1 and 5.2 ending at 1, and **4.5 not ending at all — never exiting
/// on its own on a rented Linux machine** (`Rendering` K15a, §13 item 5). So
/// the alternative to this refusal is not a wrong picture; on one of the four
/// versions it is a job that never ends on a machine billed by the hour.
pub fn anchor_the_engine(
    pick: OfferedEngine,
    reading_in_force: &RecordedReading,
) -> Result<ExactName, NotFound> {
    match the_readings_identifier_for(pick, reading_in_force) {
        Some(identifier) => {
            anchor_at_the_attempt(&identifier, &WhatIsAnchored::AnEngine, reading_in_force)
        }
        None => Err(NotFound {
            what: WhatIsAnchored::AnEngine,
            name: ExactName::as_the_reading_produced_it(match pick {
                OfferedEngine::Cycles => "Cycles",
                OfferedEngine::Eevee => "EEVEE",
            }),
        }),
    }
}

/// **E4: what the page shows** — the reading's `engine_as_read` per scene, and
/// **the file's own engine name where it is neither of the two offered**,
/// which is the Design document's third-row control.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WhatThePageShows {
    /// The file's engine is one of the two offered.
    Offered(OfferedEngine),
    /// The file's own engine, by name, neither of the two offered.
    TheFilesOwn(ExactName),
}

pub fn what_the_page_shows(scene: &SceneReading, reading: &RecordedReading) -> WhatThePageShows {
    let offered = &reading.settings.offered_engine_identifiers;
    let read = scene.engine_as_read.as_bytes();
    if offered
        .cycles
        .as_ref()
        .is_some_and(|name| name.as_bytes() == read)
    {
        WhatThePageShows::Offered(OfferedEngine::Cycles)
    } else if offered
        .eevee
        .as_ref()
        .is_some_and(|name| name.as_bytes() == read)
    {
        WhatThePageShows::Offered(OfferedEngine::Eevee)
    } else {
        WhatThePageShows::TheFilesOwn(scene.engine_as_read.clone())
    }
}

/// **E5: the recorded reading supplies the engine, and nothing else about
/// add-ons is in the file to be read.** The one announcement is a field of the
/// reading ([`crate::reading::AnnouncedAddOnCase`]) and is handed to `Server
/// Setup` as that one case and never a list (that document's A5). No pass of
/// the reading goes looking for anything else about add-ons, because the
/// research says there is nothing in the file to find.
pub const NOTHING_ELSE_ABOUT_ADD_ONS_IS_READ: () = ();

#[cfg(test)]
mod tests {
    use super::*;
    use crate::file_half::tests::{a_recorded_reading, a_scene};

    #[test]
    fn an_untouched_engine_is_no_entry_and_a_pick_carries_the_readings_own_identifier() {
        let reading = a_recorded_reading();
        assert_eq!(the_engine_entry(None, Some(&reading)), None);
        let entry = the_engine_entry(Some(OfferedEngine::Cycles), Some(&reading)).unwrap();
        assert_eq!(entry.pick, OfferedEngine::Cycles);
        assert_eq!(
            entry.identifier,
            Some(ExactName::as_the_reading_produced_it("CYCLES"))
        );
        // No reading: the pick rides alone.
        let alone = the_engine_entry(Some(OfferedEngine::Eevee), None).unwrap();
        assert_eq!(alone.identifier, None);
    }

    #[test]
    fn a_pick_the_build_does_not_register_fails_the_anchor_naming_the_engine() {
        let mut reading = a_recorded_reading();
        reading.settings.offered_engine_identifiers.cycles = None;
        assert!(anchor_the_engine(OfferedEngine::Cycles, &reading).is_err());
        assert_eq!(
            anchor_the_engine(OfferedEngine::Eevee, &reading),
            Ok(ExactName::as_the_reading_produced_it("BLENDER_EEVEE_NEXT"))
        );
    }

    #[test]
    fn the_page_shows_the_files_own_engine_by_name_where_it_is_neither_offered_one() {
        let reading = a_recorded_reading();
        let mut scene = a_scene("Kitchen");
        assert_eq!(
            what_the_page_shows(&scene, &reading),
            WhatThePageShows::Offered(OfferedEngine::Cycles)
        );
        scene.engine_as_read = ExactName::as_the_reading_produced_it("HYDRA_STORM");
        assert_eq!(
            what_the_page_shows(&scene, &reading),
            WhatThePageShows::TheFilesOwn(ExactName::as_the_reading_produced_it("HYDRA_STORM"))
        );
    }
}
