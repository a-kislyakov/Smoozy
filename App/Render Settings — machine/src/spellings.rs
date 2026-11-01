//! Every property spelling a pair of this document's is composed from, in
//! one place — the ones *Settings 17* cites (C7, C9), and **the five C10
//! carries, read off four running Blenders on 2026-09-20.**
//!
//! **§13 item 29 was handed to Build on 2026-09-02 and closed on 2026-09-20.**
//! Five pairs had no read spelling: C1 items 2, 3 and 5, the property C1 item
//! 4's rule lives at, and D3's scene device setting. **What could not have
//! closed it: a spelling composed from an agent's memory, from a manual's
//! interface wording, or by joining names out of questions that do not carry
//! them.** What did close it is the lookup item 8(a) of `What Build inherits`
//! named as Build's own work: each of the four supported Blenders started
//! with no window and from factory settings, asked to print whole property
//! lists with every property's kind, starting value, label and description,
//! and the printed lists then matched against what was wanted. **All five came
//! back found, in all four versions, identical in all four.**
//!
//! **The source of those five is the lookup and its record, never a
//! comparison**: `Audits/Build/Step 10c/The property-name lookup — what was
//! read.md`, 2026-09-20, cited in the plan's companion sources file under its
//! own entry for this lookup. The names that
//! carry [`SETTINGS_17`] instead are *Settings 17*'s and are a different
//! source; the two are kept apart in [`Spelling::Cited`]'s own `source` field
//! and the tests below hold that split.
//!
//! **The absence shape stays in the file with nothing in it, deliberately.**
//! [`Spelling::NotYetRead`] and [`THE_FIVE_NOT_YET_READ`] remain, the list now
//! empty, because the loud-failure path they feed is `Rendering` L10a's name
//! check and that check stands whatever the names turned out to be — a pair
//! composed from a name the object does not carry still ends the attempt
//! before the render, with nothing written. A spelling that later turns out
//! to differ on Linux comes back here as an absence rather than as a guess.
//!
//! **Why a wrong name here would be the worst kind of wrong, which is why
//! none of these was taken from a near neighbour.** A wrong property name is
//! handed in, reads back correctly, and changes nothing, which is the
//! silent-wrong-render class this whole step exists to avoid. The near misses
//! the lookup turned up are named below, at [`THE_NEAR_MISSES`], for exactly
//! that reason — a setting on the same scene taking the device pair's own two
//! values, and three viewport twins whose labels and descriptions are word for
//! word the render ones'.
//!
//! **None of these names is the word the interface shows** (C9's last
//! paragraph): `resolution_percentage`, a label of *Resolution Scale*, a panel
//! reading `%` and a manual saying *"Percentage"* are four wordings for one
//! setting. Nothing here is ever composed from a label, a panel or a manual
//! page.
//!
//! **Markers used in this file that belong to another document, each with the
//! document it belongs to.** `Rendering` L10a.

/// **The two sources a name in this file may come from, and they are not the
/// same kind of thing.** *Settings 17* is a Blender-research comparison;
/// [`THE_LOOKUP`] is a reading taken at Build off the running programs
/// themselves. Nothing here is ever dressed as the other.
pub const SETTINGS_17: &str = "Settings 17";

/// **The lookup of 2026-09-20 — the four supported Blenders, started with no
/// window and from factory settings, printing their own whole property
/// lists.** Its record holds, per version and per name, the full path written
/// out from the scene, the kind, the starting value, the label and the
/// description Blender gave, and the commands that produced them.
pub const THE_LOOKUP: &str =
    "the property-name lookup, 2026-09-20 (Audits/Build/Step 10c/The property-name lookup — what was read.md)";

/// One property spelling, as a pair is composed from it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Spelling {
    /// Read out of a running Blender, cited to what it was read from —
    /// [`SETTINGS_17`] or [`THE_LOOKUP`].
    Cited {
        /// The name, identical in all four versions.
        name: &'static str,
        /// Written out from the job's scene (C8's assignment target).
        written_out_from_the_scene: &'static str,
        /// What it was read from.
        source: &'static str,
    },
    /// **An explicit absence.** No composition can make a pair from it; what
    /// it names is the reading that would close it. **Nothing in this file is
    /// one today** — [`THE_FIVE_NOT_YET_READ`] is empty — and the shape stays
    /// so a spelling found to differ somewhere Smoozy renders comes back as
    /// an absence rather than as a guess.
    NotYetRead(MissingSpelling),
}

/// **What is missing, named so a failure can say it** — the setting, the
/// clause whose pair needs it, and what closes it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MissingSpelling {
    pub the_setting: &'static str,
    pub the_clause: &'static str,
}

impl Spelling {
    /// The name a pair assigns against, or the absence that stops the pair
    /// being composed at all.
    pub fn written_out(&self) -> Result<&'static str, MissingSpelling> {
        match self {
            Spelling::Cited {
                written_out_from_the_scene,
                ..
            } => Ok(written_out_from_the_scene),
            Spelling::NotYetRead(missing) => Err(*missing),
        }
    }
}

/// **C7: the resolution percentage's spelling is `resolution_percentage`, the
/// same in all four versions; written out from the job's scene it is
/// `scene.render.resolution_percentage`** (*Settings 17*). Label:
/// **documented** from each version's own API page, and **observed** in all
/// four. The bound is item 28's: every run behind it was on the Creator's
/// Mac, and no name in it has been set on Linux.
pub const RESOLUTION_PERCENTAGE: Spelling = Spelling::Cited {
    name: "resolution_percentage",
    written_out_from_the_scene: "scene.render.resolution_percentage",
    source: SETTINGS_17,
};

/// **C9: C1 item 1 under Cycles** — `samples`, `scene.cycles.samples`.
/// **Observed** out of Cycles' own shipped files, which have no API page for
/// its render settings in any of the four.
pub const CYCLES_SAMPLES: Spelling = Spelling::Cited {
    name: "samples",
    written_out_from_the_scene: "scene.cycles.samples",
    source: SETTINGS_17,
};

/// **C9: C1 item 1 under EEVEE** — `taa_render_samples`,
/// `scene.eevee.taa_render_samples`. **Documented** from each version's own
/// API page.
pub const EEVEE_SAMPLES: Spelling = Spelling::Cited {
    name: "taa_render_samples",
    written_out_from_the_scene: "scene.eevee.taa_render_samples",
    source: SETTINGS_17,
};

/// **C9: Workbench's `render_aa`, `scene.display.render_aa` — which this page
/// never composes, and which is not a count**: a fixed list of choices
/// matched exactly, letter case included, declaring no range. The table keeps
/// the name because the name is what the table is for.
pub const WORKBENCH_RENDER_AA_NEVER_COMPOSED: Spelling = Spelling::Cited {
    name: "render_aa",
    written_out_from_the_scene: "scene.display.render_aa",
    source: SETTINGS_17,
};

/// **C9: a view layer's own override, which C1 item 4's rule governs** —
/// `samples`, `scene.view_layers["…"].samples`. **The bare word `samples`
/// names two different settings**, told apart only by what it hangs off; C8's
/// assignment-target rule is what keeps them apart.
pub const VIEW_LAYER_SAMPLES: Spelling = Spelling::Cited {
    name: "samples",
    written_out_from_the_scene: "scene.view_layers[\"…\"].samples",
    source: SETTINGS_17,
};

/// **C9, V4: the pin switch — whether a view layer renders** — `use`,
/// `scene.view_layers["…"].use`. The one that looks like it and is ignored
/// from outside is `use_single_layer`, which §4's constraint refuses.
pub const VIEW_LAYER_USE: Spelling = Spelling::Cited {
    name: "use",
    written_out_from_the_scene: "scene.view_layers[\"…\"].use",
    source: SETTINGS_17,
};

/// **C10, C1 item 2: adaptive sampling's switch, which the set turns off** —
/// `use_adaptive_sampling`, `scene.cycles.use_adaptive_sampling`, the same in
/// all four versions, starting **on**, labelled `Use Adaptive Sampling`.
/// *Settings 16* answers this displacer in the manual's interface words and
/// carries no property name; the name is the lookup's.
pub const ADAPTIVE_SAMPLING_OFF: Spelling = Spelling::Cited {
    name: "use_adaptive_sampling",
    written_out_from_the_scene: "scene.cycles.use_adaptive_sampling",
    source: THE_LOOKUP,
};

/// **C10: adaptive sampling's threshold**, `scene.cycles.adaptive_threshold`,
/// starting at 0.01. **This document composes no pair against it** — C1 item
/// 2 switches the whole thing off, which is what makes the threshold and the
/// minimum beside the point — and the name is kept because §13 item 29 named
/// all three and a later displacer clause would want it read rather than
/// guessed.
pub const ADAPTIVE_THRESHOLD_NEVER_COMPOSED: Spelling = Spelling::Cited {
    name: "adaptive_threshold",
    written_out_from_the_scene: "scene.cycles.adaptive_threshold",
    source: THE_LOOKUP,
};

/// **C10: adaptive sampling's minimum**, `scene.cycles.adaptive_min_samples`,
/// starting at 0. **Never composed**, for the same reason as the threshold.
pub const ADAPTIVE_MIN_SAMPLES_NEVER_COMPOSED: Spelling = Spelling::Cited {
    name: "adaptive_min_samples",
    written_out_from_the_scene: "scene.cycles.adaptive_min_samples",
    source: THE_LOOKUP,
};

/// **C10, C1 item 3: the time limit, set to its off value** — `time_limit`,
/// `scene.cycles.time_limit`, a decimal number starting at 0.0. **The off
/// value is zero** and that is Blender's own statement of it, its description
/// reading *"Zero disables the limit"*; it is the only property in any of the
/// four versions whose name carries these words.
pub const TIME_LIMIT_OFF: Spelling = Spelling::Cited {
    name: "time_limit",
    written_out_from_the_scene: "scene.cycles.time_limit",
    source: THE_LOOKUP,
};

/// **The time limit's off value, from the same reading** — zero. Held beside
/// the spelling because §13 item 29 owed both and a composition that had the
/// name and not the value would be no better off.
pub const THE_TIME_LIMITS_OFF_VALUE: u64 = 0;

/// **C10, C1 item 4: the property the layer-samples rule lives at** —
/// `use_layer_samples`, `scene.cycles.use_layer_samples`, taking exactly
/// `USE`, `BOUNDED` and `IGNORE` and starting at `USE`.
///
/// **C9 declined this name and was right to, and the refusal is lifted by
/// enumeration rather than by a match.** C9's ground was that nothing in
/// *Settings 17* says what the name governs, and joining two questions to
/// decide it is the assembling the Method forbids. Nothing was joined: the
/// lookup asked, in each version, which property anywhere offers exactly
/// those three choices, and in every version the answer was one property and
/// only one. The three values remain this document's own test's; they now
/// meet the property that carries them.
pub const LAYER_SAMPLES_RULE: Spelling = Spelling::Cited {
    name: "use_layer_samples",
    written_out_from_the_scene: "scene.cycles.use_layer_samples",
    source: THE_LOOKUP,
};

/// **C10, C1 item 5: the sample subset, switched off** —
/// `use_sample_subset`, `scene.cycles.use_sample_subset`, starting **off**.
/// The two numbers beside it — `sample_offset` and `sample_subset_length` —
/// are not part of this pair and nothing here composes against either.
pub const SAMPLE_SUBSET_OFF: Spelling = Spelling::Cited {
    name: "use_sample_subset",
    written_out_from_the_scene: "scene.cycles.use_sample_subset",
    source: THE_LOOKUP,
};

/// **C10, D3: the scene's own device setting, assigned to the processor** —
/// `device`, `scene.cycles.device`, taking `CPU` and `GPU` and starting at
/// `CPU`. *Settings 9* establishes the launch argument, the three routes and
/// the device values and names no path on a scene; the name is the lookup's.
/// **The route this pair takes is still unobserved** (§13 item 21), which the
/// spelling landing does not change.
pub const SCENE_DEVICE_SETTING: Spelling = Spelling::Cited {
    name: "device",
    written_out_from_the_scene: "scene.cycles.device",
    source: THE_LOOKUP,
};

/// **D3's processor value, from the same reading** — `CPU`, matched exactly,
/// letter case included.
pub const THE_PROCESSOR_VALUE: &str = "CPU";

/// **What §13 item 29 owed, as one list, now empty.** It held the five until
/// 2026-09-20; every one of them is a [`Spelling::Cited`] above. **The list
/// and its test stay** because the file's own test rests on it — nothing else
/// in this file is an absence, and nothing composes a pair from one — and
/// because a spelling later found to differ where Smoozy renders comes back
/// here rather than being guessed at.
pub const THE_FIVE_NOT_YET_READ: [&Spelling; 0] = [];

/// **Near-miss names, none of them a pair of this document's, and every one
/// of them recorded because matching a wanted thing to a near neighbour is
/// this project's own named fault.**
///
/// **C9's four**, under the same name, label, description and starting value
/// in all four versions: Cycles' `preview_samples` and EEVEE's `taa_samples`,
/// both viewport counts; `use_single_layer`, which §4's constraint refuses;
/// and Grease Pencil's `aa_samples`.
///
/// **The lookup's four, added 2026-09-20.** `compositor_device` takes the same
/// two values as D3's pair **on the same scene** and decides how compositing
/// is carried out — a pair assigned against it would be accepted in silence
/// while the render went on using the cards — and `compositor_denoise_device`
/// sits beside it. `use_preview_adaptive_sampling`, `preview_adaptive_threshold`
/// and `preview_adaptive_min_samples` are the viewport twins of C1 item 2's
/// three, **carrying labels and descriptions identical to the render ones**,
/// so those three are told apart by name alone.
///
/// **Nothing here composes against any of them**, and the test below holds it.
pub const THE_NEAR_MISSES: [&str; 9] = [
    "preview_samples",
    "taa_samples",
    "use_single_layer",
    "aa_samples",
    "compositor_device",
    "compositor_denoise_device",
    "use_preview_adaptive_sampling",
    "preview_adaptive_threshold",
    "preview_adaptive_min_samples",
];

/// **`Rendering` L10a's name check is the backstop for a spelling from the
/// wrong operating system** (item 28), and it has itself been posed on no
/// Linux machine. Recorded here so the bound on every cited name travels with
/// the names.
pub const EVERY_CITED_NAME_WAS_SET_ON_ONE_MAC_ALONE: bool = true;

#[cfg(test)]
mod tests {
    use super::*;

    /// Every spelling in this file, so a name added later cannot escape the
    /// checks below by not being listed anywhere.
    const EVERY_SPELLING: [(&str, Spelling); 13] = [
        ("C7 the percentage", RESOLUTION_PERCENTAGE),
        ("C9 Cycles' count", CYCLES_SAMPLES),
        ("C9 EEVEE's count", EEVEE_SAMPLES),
        ("C9 Workbench's", WORKBENCH_RENDER_AA_NEVER_COMPOSED),
        ("C9 a layer's own count", VIEW_LAYER_SAMPLES),
        ("C9 the pin", VIEW_LAYER_USE),
        ("C10 C1 item 2's switch", ADAPTIVE_SAMPLING_OFF),
        ("C10 its threshold", ADAPTIVE_THRESHOLD_NEVER_COMPOSED),
        ("C10 its minimum", ADAPTIVE_MIN_SAMPLES_NEVER_COMPOSED),
        ("C10 C1 item 3", TIME_LIMIT_OFF),
        ("C10 C1 item 4", LAYER_SAMPLES_RULE),
        ("C10 C1 item 5", SAMPLE_SUBSET_OFF),
        ("C10 D3", SCENE_DEVICE_SETTING),
    ];

    /// **§13 item 29 closed on 2026-09-20 and this is what closing it means
    /// here**: the list of absences is empty, and nothing in this file is an
    /// absence any more. The shape stays so a later absence has somewhere to
    /// go; what may not happen is a name arriving without a source.
    #[test]
    fn nothing_in_this_file_is_an_absence_any_more_and_the_empty_list_says_so() {
        assert!(
            THE_FIVE_NOT_YET_READ.is_empty(),
            "item 29 is closed, so nothing is owed"
        );
        for (which, spelling) in EVERY_SPELLING {
            assert!(
                spelling.written_out().is_ok(),
                "{which} carries no spelling, and item 29 is closed"
            );
        }
    }

    /// **The two sources are kept apart and neither is dressed as the other.**
    /// *Settings 17* is a comparison; the lookup is a reading taken at Build.
    #[test]
    fn every_name_carries_one_of_the_two_sources_and_none_is_a_near_miss() {
        for (which, spelling) in EVERY_SPELLING {
            let Spelling::Cited {
                name,
                written_out_from_the_scene,
                source,
            } = spelling
            else {
                panic!("{which} is not cited");
            };
            assert!(
                source == SETTINGS_17 || source == THE_LOOKUP,
                "{which} cites something that is neither"
            );
            assert!(written_out_from_the_scene.ends_with(name));
            assert!(written_out_from_the_scene.starts_with("scene."));
            assert!(!THE_NEAR_MISSES.contains(&name), "{which} is a near miss");
        }
    }

    /// **The five item 29 owed are the lookup's and not a comparison's.** A
    /// later edit that re-cited one of them to *Settings 17* would be claiming
    /// a comparison carries a name it was never posed on.
    #[test]
    fn the_five_item_29_owed_cite_the_lookup_and_never_a_comparison() {
        for spelling in [
            ADAPTIVE_SAMPLING_OFF,
            TIME_LIMIT_OFF,
            LAYER_SAMPLES_RULE,
            SAMPLE_SUBSET_OFF,
            SCENE_DEVICE_SETTING,
        ] {
            let Spelling::Cited { source, .. } = spelling else {
                panic!("{spelling:?} is not cited");
            };
            assert_eq!(source, THE_LOOKUP);
        }
    }

    /// **The device pair's sharpest near miss sits on the same scene and takes
    /// the same two values**, so it is named rather than left to be met.
    #[test]
    fn the_device_pair_is_the_cycles_one_and_not_the_compositors() {
        let Spelling::Cited {
            written_out_from_the_scene,
            ..
        } = SCENE_DEVICE_SETTING
        else {
            unreachable!()
        };
        assert_eq!(written_out_from_the_scene, "scene.cycles.device");
        assert!(THE_NEAR_MISSES.contains(&"compositor_device"));
    }

    #[test]
    fn the_word_samples_names_two_settings_told_apart_by_what_they_hang_off() {
        let (
            Spelling::Cited {
                written_out_from_the_scene: scene,
                ..
            },
            Spelling::Cited {
                written_out_from_the_scene: layer,
                ..
            },
        ) = (CYCLES_SAMPLES, VIEW_LAYER_SAMPLES)
        else {
            unreachable!()
        };
        assert_ne!(scene, layer);
        assert!(scene.contains("cycles"));
        assert!(layer.contains("view_layers"));
    }
}
