//! **`Dependency Upload & Remap` G17b: the check's readings, kept in the run's
//! memory.** A file unchanged since this run read it is not read again, so a
//! round costs only what is new. `crate::checking_a_file` decides what stands;
//! this file is what is held and how much.
//!
//! **Memory only.** Nothing here reaches the record, and it is gone when the app
//! quits. **What it holds**: for each file, per machine and per Blender, one
//! reading's document and the short list it stood on, so a changed file replaces
//! its own entry. **How much**: [`THE_MOST_A_RUN_KEEPS`], the oldest let go first.

use std::collections::BTreeMap;
use std::time::Duration;

use smoozy_dependencies::bytes::MachineLocation;
use smoozy_dependencies::check::{Attribution, ReadingDocument};
use smoozy_dependencies::record::MachineKey;
use smoozy_render_settings::file_half::{version_text, BlenderIdentity};
use smoozy_server_protocol::transfers::SourceReading;

/// **`Dependency Upload & Remap` G17b's bound, a Build figure.** By this file's
/// own reckoning a reading of a thousand references holds a few hundred
/// kilobytes, so this is a few hundred such files, and a reading let go is only
/// read again.
pub const THE_MOST_A_RUN_KEEPS: usize = 64 * 1024 * 1024;

/// **`Design/Dependency Upload & Remap`: *Reading the file…* stands at least this
/// long where the root's reading was kept**, so *Check again* reads as a check.
pub const THE_LEAST_A_KEPT_READING_SHOWS: Duration = Duration::from_millis(700);

/// **What one entry is kept under**: the machine, the file's location and the
/// Blender. The file's four values are what it stood on, not its key, so a
/// changed file replaces its own entry.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct WhichReading {
    machine: String,
    file: Vec<u8>,
    blender: String,
}

impl WhichReading {
    pub fn of(machine: &MachineKey, file: &MachineLocation, with: &BlenderIdentity) -> WhichReading {
        WhichReading {
            machine: format!("{}\u{1f}{}", machine.server_row.as_text(), machine.installation.as_text()),
            file: file.as_bytes().to_vec(),
            blender: format!("{}\u{1f}{}", with.path, version_text(with.version)),
        }
    }
}

/// **What a reading stood on**: the file's four values, and each library it names
/// at the location Blender looked in, with its four values or none.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WhatItStoodOn {
    pub the_file: SourceReading,
    pub the_libraries: Vec<(Vec<u8>, Option<SourceReading>)>,
}

struct AKeptReading {
    document: ReadingDocument,
    stood_on: WhatItStoodOn,
    bytes: usize,
    kept_at: u64,
}

/// **The readings this run holds.**
#[derive(Default)]
pub struct TheKeptReadings {
    readings: BTreeMap<WhichReading, AKeptReading>,
    bytes: usize,
    kept: u64,
}

/// **Roughly what an entry holds**, its text and a word or two for each piece. A
/// figure for the bound and nothing more.
fn the_bytes_of(which: &WhichReading, document: &ReadingDocument, stood_on: &WhatItStoodOn) -> usize {
    const FOR_EACH_PIECE: usize = 64;
    let the_references: usize = document
        .references
        .iter()
        .map(|one| {
            let scenes = match &one.scenes {
                Attribution::Established(names) => names.iter().map(|name| name.len() + FOR_EACH_PIECE).sum(),
                Attribution::NotEstablished => 0,
            };
            FOR_EACH_PIECE * 2
                + one.datablock.len()
                + one.setting.len()
                + one.recorded.as_bytes().len()
                + one.source.as_ref().map_or(0, String::len)
                + one.the_files_own_text.as_ref().map_or(0, |own| own.as_bytes().len())
                + scenes
        })
        .sum();
    let the_lists: usize = document.flat_list.iter().chain(&document.observed_stand_ins).map(|one| FOR_EACH_PIECE + one.as_bytes().len()).sum();
    let the_weak: usize = document.weak_references.len() * FOR_EACH_PIECE * 4;
    let the_libraries: usize = stood_on.the_libraries.iter().map(|(at, _)| FOR_EACH_PIECE * 2 + at.len()).sum();
    FOR_EACH_PIECE * 4 + which.file.len() + which.machine.len() + which.blender.len() + the_references + the_lists + the_weak + the_libraries
}

impl TheKeptReadings {
    /// **The reading kept for this file by this Blender on this machine**, with
    /// what it stood on. Whether it still stands is the caller's to compare.
    pub fn the_one_for(&self, which: &WhichReading) -> Option<(ReadingDocument, WhatItStoodOn)> {
        self.readings.get(which).map(|kept| (kept.document.clone(), kept.stood_on.clone()))
    }

    /// **A reading that no longer stands is let go.**
    pub fn let_go(&mut self, which: &WhichReading) {
        if let Some(gone) = self.readings.remove(which) {
            self.bytes -= gone.bytes;
        }
    }

    /// **One reading kept**, in place of what stood for that file. **The oldest go
    /// first where the bound is passed**, and one larger than the bound by itself
    /// is not kept.
    pub fn keep(&mut self, which: WhichReading, document: ReadingDocument, stood_on: WhatItStoodOn) {
        self.let_go(&which);
        let bytes = the_bytes_of(&which, &document, &stood_on);
        if bytes > THE_MOST_A_RUN_KEEPS {
            return;
        }
        while self.bytes + bytes > THE_MOST_A_RUN_KEEPS {
            let Some(oldest) = self.readings.iter().min_by_key(|(_, kept)| kept.kept_at).map(|(which, _)| which.clone()) else {
                break;
            };
            self.let_go(&oldest);
        }
        self.kept += 1;
        self.bytes += bytes;
        self.readings.insert(which, AKeptReading { document, stood_on, bytes, kept_at: self.kept });
    }

    /// **How much is held**, by the same reckoning as the bound.
    pub fn the_bytes_held(&self) -> usize {
        self.bytes
    }

    /// **How many readings are held.**
    pub fn how_many(&self) -> usize {
        self.readings.len()
    }
}

/// **Files going up to each machine this run**, by either route: how many began
/// and how many ended. `Dependency Upload & Remap` G17b: a reading is kept only
/// where none was on its way when it began and none began or ended before what it
/// stood on was read, since a library landing after Blender looked and before its
/// four values were read would read as there when Blender did not see it.
#[derive(Default)]
pub struct TheFilesGoingUp(BTreeMap<String, (u64, u64)>);

impl TheFilesGoingUp {
    pub fn one_begins(&mut self, machine: &MachineKey) {
        self.0.entry(machine.server_row.as_text()).or_default().0 += 1;
    }

    pub fn one_ends(&mut self, machine: &MachineKey) {
        self.0.entry(machine.server_row.as_text()).or_default().1 += 1;
    }

    /// **As they stand on this machine**: began, and ended.
    pub fn on(&self, machine: &MachineKey) -> (u64, u64) {
        self.0.get(&machine.server_row.as_text()).copied().unwrap_or((0, 0))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use smoozy_dependencies::bytes::RecordedLocation;
    use smoozy_dependencies::check::{Kind, ReadReference};
    use smoozy_server_setup::blender::Version;
    use smoozy_state::identities::{Identity, IdentityKind};

    fn a_machine() -> MachineKey {
        MachineKey::of(
            Identity::issue(IdentityKind::LocalServer).expect("randomness"),
            Identity::issue(IdentityKind::Installation).expect("randomness"),
        )
    }

    fn a_blender() -> BlenderIdentity {
        BlenderIdentity {
            path: "/root/blender".to_string(),
            version: Version { major: 5, minor: 3, patch: 0 },
        }
    }

    fn values() -> SourceReading {
        SourceReading { size: 1, inode: "7".to_string(), modification_time: 1, change_time: 1 }
    }

    /// **A reading of about the size a real shot's runs to**: a thousand
    /// references.
    fn a_document(references: usize) -> ReadingDocument {
        ReadingDocument {
            references: (0..references)
                .map(|at| ReadReference {
                    kind: Kind::Image,
                    datablock: format!("picture_{at}.png"),
                    setting: "filepath".to_string(),
                    recorded: RecordedLocation::as_the_reading_carried_it(format!("//textures/picture_{at}.png").as_bytes()),
                    scenes: Attribution::Established(vec!["Scene".to_string()]),
                    source: Some("FILE".to_string()),
                    the_files_own_text: None,
                })
                .collect(),
            flat_list: Vec::new(),
            weak_references: Vec::new(),
            observed_stand_ins: Vec::new(),
        }
    }

    fn stood_on() -> WhatItStoodOn {
        WhatItStoodOn { the_file: values(), the_libraries: Vec::new() }
    }

    /// **`Dependency Upload & Remap` G17b: a long check on a large shot holds a
    /// bounded amount**, the oldest let go first, each file one entry however often
    /// it changes.
    #[test]
    fn what_a_run_holds_is_bounded_and_the_oldest_goes_first() {
        let machine = a_machine();
        let mut kept = TheKeptReadings::default();
        let which = |at: usize| WhichReading::of(&machine, &MachineLocation::composed(format!("/m/f{at}.blend").into_bytes()), &a_blender());

        // **One file, read again and again, is one entry.**
        for _ in 0..100 {
            kept.keep(which(0), a_document(1_000), stood_on());
        }
        assert_eq!(kept.how_many(), 1);
        let one_reading = kept.the_bytes_held();
        assert!((100_000..400_000).contains(&one_reading), "a thousand references: {one_reading}");

        // **Far more files than the bound holds.**
        let how_many = THE_MOST_A_RUN_KEEPS / one_reading * 2;
        for at in 1..how_many {
            kept.keep(which(at), a_document(1_000), stood_on());
            assert!(kept.the_bytes_held() <= THE_MOST_A_RUN_KEEPS);
        }
        assert!(kept.the_one_for(&which(0)).is_none(), "the oldest went first");
        assert!(kept.the_one_for(&which(how_many - 1)).is_some(), "the newest stands");
        assert!(kept.how_many() * one_reading <= THE_MOST_A_RUN_KEEPS + one_reading);

        // **Let go is let go, in the reckoning too.**
        let before = kept.the_bytes_held();
        kept.let_go(&which(how_many - 1));
        assert!(kept.the_bytes_held() < before);
        kept.let_go(&which(how_many - 1));
    }

    /// **One reading larger than the bound by itself is not kept**, and costs what
    /// stands nothing.
    #[test]
    fn a_reading_larger_than_the_bound_is_not_kept() {
        let machine = a_machine();
        let mut kept = TheKeptReadings::default();
        let small = WhichReading::of(&machine, &MachineLocation::composed(b"/m/small.blend".to_vec()), &a_blender());
        kept.keep(small.clone(), a_document(1), stood_on());
        let huge = WhichReading::of(&machine, &MachineLocation::composed(b"/m/huge.blend".to_vec()), &a_blender());
        let mut the_document = a_document(1);
        the_document.references[0].datablock = "x".repeat(THE_MOST_A_RUN_KEEPS + 1);
        kept.keep(huge.clone(), the_document, stood_on());
        assert!(kept.the_one_for(&huge).is_none());
        assert!(kept.the_one_for(&small).is_some());
    }

    /// **Which entry a reading is**: another machine, another Blender, another file.
    #[test]
    fn a_reading_is_kept_per_machine_per_file_per_blender() {
        let (one, other) = (a_machine(), a_machine());
        let file = MachineLocation::composed(b"/m/f.blend".to_vec());
        let reading = WhichReading::of(&one, &file, &a_blender());
        assert_eq!(reading, WhichReading::of(&one, &file, &a_blender()));
        assert_ne!(reading, WhichReading::of(&other, &file, &a_blender()));
        assert_ne!(reading, WhichReading::of(&one, &MachineLocation::composed(b"/m/g.blend".to_vec()), &a_blender()));
        let newer = BlenderIdentity {
            version: Version { major: 5, minor: 3, patch: 1 },
            ..a_blender()
        };
        assert_ne!(reading, WhichReading::of(&one, &file, &newer));
    }
}
