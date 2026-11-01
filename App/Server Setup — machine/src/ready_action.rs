//! The ready action's steps — `Server Setup` §2.1 and §2.2, the half the
//! machine's own program may take.
//!
//! **Nothing here performs a step.** What is here is the closed, ordered list
//! of steps, the one check the driver step reads, and the libraries step's
//! mapping and its judgement over a not-found list. **The composition — which
//! steps a machine gets — stays in `smoozy-server-setup`**, because it is made
//! from the machine's kind and family, which are desktop readings; that crate
//! re-exports everything here at `ready_action`, the path it has always been
//! reached at. Split at step 10c piece 3.

use smoozy_framing::command_string::CommandString;

/// **R1: the ready action is one action to the user and a fixed, ordered
/// sequence of steps underneath, composed per machine at run start from what
/// the machine is found to need.**
///
/// **The order is fixed and the presence of each step is not**: *"each present
/// only where the machine needs it."*
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Step {
    /// **Item 1.** The machine kind, and card presence from the bus scan and
    /// the driver's own answer.
    Readings,
    /// **Item 2**, where the run installs a Blender: the archive staged on the
    /// machine, resumable, digest-checked. **Unprivileged.**
    Fetch,
    /// **Item 3.** The one unpack tool Blender's documentation names, installed
    /// where absent, and the archive unpacked with it and nothing else.
    Tooling,
    /// **Item 4.** `ldd` over the executable the machine will render with, the
    /// **whole** not-found list read at once, the named packages installed
    /// explicitly, `ldd` re-read clean.
    Libraries,
    /// **Item 5**, systemd kind only, and check-first.
    Driver,
    /// **Item 6**, only where item 5's check failed after its work.
    Restart,
    /// **Item 7**, where the run fetched one: the staged build asked its
    /// version in staging, and renamed to its final folder **only on that
    /// answer**.
    BlenderPublication,
    /// **`Server Setup` B15's removal: the plain deletion of the install's
    /// folder.** A removal run's own step (Y6b), and never part of the ready
    /// action. **Unprivileged**: the folder is Smoozy's own, under the Blender
    /// area. Added 2026-10-10, the 38's piece 14c.
    ///
    /// **B15's second route is this step too, from 2026-10-16** (step 10c's
    /// owed piece 6): for a build the machine's own software manager holds
    /// under the one fixed package, the desk sends the manager's removal,
    /// elevated (E2, E7), and the worker then reads that the build is gone and
    /// deletes nothing. **A re-run is safe the same way**: the desk sends the
    /// removal only where the build is still there, and a build already gone
    /// is nothing to do.
    BlenderRemoval,
    /// **Item 8**, where Telegram is on.
    TelegramMaterial,
    /// **Item 9.** The final verification, then the state cleared, which runs
    /// reconciliation (`Rented-server runtime` P5).
    Done,
}

impl Step {
    /// **R1's order, entire.** The composition below draws from this and never
    /// from a list written a second time.
    pub const IN_ORDER: [Step; 10] = [
        Step::Readings,
        Step::Fetch,
        Step::Tooling,
        Step::Libraries,
        Step::Driver,
        Step::Restart,
        Step::BlenderPublication,
        Step::BlenderRemoval,
        Step::TelegramMaterial,
        Step::Done,
    ];
}

/// **R1 item 5's check**, whose exit is the only reading taken of the driver.
pub const THE_DRIVER_CHECK: CommandString = CommandString::constant("nvidia-smi");

/// **R1 item 4's mapping, and E7's requirement on it: it is fixed at build time
/// rather than derived from a launch error.**
///
/// **Blender names only the first missing library — the whole list is read at
/// once, from `ldd`, never from the launch error** (Fact 47). One trap: a driver
/// metapackage incidentally supplied six of nine missing libraries on one box,
/// so **the list is installed explicitly and never left to the driver's
/// dependencies**.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MappingEntry {
    /// **Every library name `ldd` may report not found that this entry's
    /// packages answer for**, rather than the one library whose name Blender's
    /// launch error happened to carry.
    ///
    /// **This was a single name until 2026-09-22 and that was the whole of the
    /// fault**: the entry's packages already carried every library a bare
    /// `ubuntu:24.04` container is missing, and eight of the nine had no key,
    /// so [`what_to_install_for`] refused the step and installed nothing. An
    /// entry answers for the libraries its own packages publish, and the keys
    /// are now written out.
    pub libraries: &'static [&'static str],
    /// The packages that resolved that machine's whole chain.
    pub packages: &'static [&'static str],
    /// What the entry rests on, in the plan's own terms.
    pub what_it_rests_on: &'static str,
}

/// **The mapping's first entry, and R1 item 4 calls it that in its own
/// words**: *"This is one image on one seller and it is the mapping's first
/// entry rather than the mapping."*
///
/// On a stock `ubuntu:24.04` container the Blender binary failed with
/// `error while loading shared libraries: libX11.so.6: cannot open shared
/// object file: No such file or directory`, and the packages that resolved the
/// whole chain were **ten**. **Installing all ten took 17 seconds.**
///
/// **A second entry disagrees by count and neither is doubted**: *Builds 10*'s
/// bare machine could not satisfy **nine distinct libraries across ten lines,
/// the same nine in every version**, and pulled in forty-two packages when nine
/// were asked for. **The images differ, which is exactly what R1 item 4 already
/// says of its own entry**, and the whole-not-found-list rule is what makes the
/// difference between images cost nothing.
///
/// # What was wrong with it until 2026-09-22, and it stopped every Blender
///
/// **The entry was keyed on the one name Blender's launch error carried.** Step
/// 10c's fourth pass on Vast read `ldd` over a fetched 5.2.2 on a bare
/// `ubuntu:24.04` container and got **ten not-found lines, nine distinct**:
/// `libX11.so.6` twice, and `libXrender.so.1`, `libXfixes.so.3`, `libXi.so.6`,
/// `libxkbcommon.so.0`, `libSM.so.6`, `libICE.so.6`, `libXext.so.6`,
/// `libGL.so.1`. **Eight of the nine had no key**, so the step refused, said so
/// honestly and installed nothing — **while the packages beside the key were
/// the very packages those eight needed.** Every test passed `libX11.so.6`
/// alone, which is why no desk run met it.
///
/// **So an entry now names every library its own packages publish**, and the
/// library-to-package facts are read from Ubuntu's own file index rather than
/// guessed from a name. Each pair below is one search of
/// `https://packages.ubuntu.com/search?searchon=contents&mode=exactfilename&suite=noble&arch=amd64`,
/// all read **2026-09-22**, each answering with the one package under
/// `/usr/lib/x86_64-linux-gnu/`:
///
/// | the library `ldd` names | the package Ubuntu's index puts it in |
/// |---|---|
/// | `libX11.so.6` | `libx11-6` |
/// | `libXi.so.6` | `libxi6` |
/// | `libXxf86vm.so.1` | `libxxf86vm1` |
/// | `libXfixes.so.3` | `libxfixes3` |
/// | `libXrender.so.1` | `libxrender1` |
/// | `libGL.so.1` | `libgl1` |
/// | `libEGL.so.1` | `libegl1` |
/// | `libSM.so.6` | `libsm6` |
/// | `libICE.so.6` | `libice6` |
/// | `libxkbcommon.so.0` | `libxkbcommon0` |
/// | `libXext.so.6` | `libxext6` |
///
/// **`libxext6` is the one package added to the list of 2026-08-22**, because
/// `libXext.so.6` is one of the nine the fourth pass watched go not-found and
/// no package in that list publishes it. R1 item 4's own rule is that **the
/// named packages are installed explicitly** rather than arriving as some other
/// package's dependency, so it is named. Its own file list —
/// `https://packages.ubuntu.com/noble/amd64/libxext6/filelist`, read
/// 2026-09-22 — carries `/usr/lib/x86_64-linux-gnu/libXext.so.6` beside
/// `libXext.so.6.4.0`.
///
/// **The two libraries no pass has watched go missing keep their keys anyway**
/// — `libXxf86vm.so.1` and `libEGL.so.1` — because their packages are in the
/// list of 2026-08-22 and an image that reports them would otherwise meet the
/// very refusal this repair is about.
pub const THE_MAPPING: [MappingEntry; 1] = [MappingEntry {
    libraries: &[
        "libX11.so.6",
        "libXi.so.6",
        "libXxf86vm.so.1",
        "libXfixes.so.3",
        "libXrender.so.1",
        "libGL.so.1",
        "libEGL.so.1",
        "libSM.so.6",
        "libICE.so.6",
        "libxkbcommon.so.0",
        "libXext.so.6",
    ],
    packages: &[
        "libx11-6",
        "libxi6",
        "libxxf86vm1",
        "libxfixes3",
        "libxrender1",
        "libgl1",
        "libegl1",
        "libsm6",
        "libice6",
        "libxkbcommon0",
        "libxext6",
    ],
    what_it_rests_on: "observed on one stock `ubuntu:24.04` container, 2026-08-22; its keys \
                       widened to the eleven libraries its packages publish, from Ubuntu's own \
                       file index read 2026-09-22, after step 10c's fourth pass",
}];

/// **A library Blender opens by name while it runs, and the loader's lookup for
/// it.** Added 2026-10-11, `Server Setup` R1 item 4, out of the Creator's
/// eighth notes, note 5.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OpenedByName {
    /// The library, by the name the loader is asked for.
    pub library: &'static str,
    /// **The shared lookup for it**
    /// ([`crate::the_optix_library::the_loaders_own_lookup_for`]), constant
    /// entire: `0` found, `1` not found, anything else no answer.
    pub the_lookup: &'static str,
}

/// **The libraries `ldd` cannot see, because Blender does not link them.**
///
/// `ldd` lists what the executable links. Over 5.2.2 on a bare `ubuntu:24.04`
/// container it named nine libraries and **never `libEGL.so.1`** (the reading
/// of 2026-09-22, [`WHAT_A_BARE_CONTAINER_REPORTED`]; **observed**), while a
/// render's error stream on a RunPod machine said *Couldn't open libEGL.so.1*
/// (note 5; **observed**). So an image carrying every linked library but not
/// this one read clean and kept the gap. That Blender opens it by name while
/// it runs is the **inference** from the two.
///
/// **Each is asked of the machine's loader at both reads**, and one the loader
/// does not find joins the not-found list
/// ([`crate::acting::steps::the_library_chain_is_read`]) — so it is installed
/// by [`THE_MAPPING`]'s own entry, which already keys it, and nothing new is
/// fetched. **This claims only that the library is there**: whether its
/// absence stopped any render is not established.
pub const WHAT_BLENDER_OPENS_BY_NAME: [OpenedByName; 1] = [OpenedByName {
    library: "libEGL.so.1",
    the_lookup: crate::the_optix_library::the_loaders_own_lookup_for!("libEGL.so.1"),
}];

/// **What one bare machine actually reported, kept as the case every side of
/// this step is driven against.**
///
/// **The nine distinct libraries `ldd` named over a fetched Blender 5.2.2 on a
/// stock `ubuntu:24.04` container**, in the order step 10c's fourth pass on
/// Vast read them on 2026-09-22 — ten lines, `libX11.so.6` twice, which
/// [`crate::acting::steps::the_not_found_list`] already names once.
///
/// **It is here rather than written out in each test** because it was written
/// out nowhere at all: every test of this step passed `libX11.so.6` on its own,
/// the one name [`THE_MAPPING`] was keyed on, so no test could fail the way
/// every bare machine did. **A reading is one thing in one place**, and the
/// desk's own end-to-end test drives this same list.
pub const WHAT_A_BARE_CONTAINER_REPORTED: [&str; 9] = [
    "libX11.so.6",
    "libXrender.so.1",
    "libXfixes.so.3",
    "libXi.so.6",
    "libxkbcommon.so.0",
    "libSM.so.6",
    "libICE.so.6",
    "libXext.so.6",
    "libGL.so.1",
];

/// **What the libraries step does with a not-found list, and what it does with
/// a library the mapping does not name.**
///
/// **E7's rule is the shape**: *"the machine's `ldd` list **selects, and never
/// composes, the command**"*, so every byte of the install command is fixed at
/// build time and **no machine-read name is echoed into it**. A library the
/// mapping does not name **fails the step honestly, with the machine's own
/// words behind it, rather than riding the command string.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WhatTheLibrariesStepInstalls {
    /// The step has nothing to do — `ldd` came back with nothing not found.
    NothingWasMissing,
    /// The packages the mapping selects, in the mapping's own order, with no
    /// duplicate.
    ThesePackages(Vec<&'static str>),
    /// **The step fails honestly.** At least one not-found library has no entry
    /// in the mapping, so no command can be composed for it and none is.
    TheMappingDoesNotNameThese(Vec<String>),
}

/// **R1 item 4 and E7, performed over the whole not-found list at once.**
pub fn what_to_install_for(not_found: &[String]) -> WhatTheLibrariesStepInstalls {
    if not_found.is_empty() {
        return WhatTheLibrariesStepInstalls::NothingWasMissing;
    }
    let mut unnamed = Vec::new();
    let mut packages: Vec<&'static str> = Vec::new();
    for library in not_found {
        match THE_MAPPING
            .iter()
            .find(|entry| entry.libraries.contains(&library.as_str()))
        {
            Some(entry) => {
                for package in entry.packages {
                    if !packages.contains(package) {
                        packages.push(package);
                    }
                }
            }
            None => unnamed.push(library.clone()),
        }
    }
    if !unnamed.is_empty() {
        // **Never a partial install.** A step that installed what it could and
        // said nothing about the rest would leave `ldd` still unclean and the
        // action claiming a check that did not pass.
        return WhatTheLibrariesStepInstalls::TheMappingDoesNotNameThese(unnamed);
    }
    WhatTheLibrariesStepInstalls::ThesePackages(packages)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_whole_not_found_list_selects_the_command_and_never_composes_it() {
        // E7's own words. Every byte installed is fixed at build time.
        let what = what_to_install_for(&["libX11.so.6".to_string()]);
        let WhatTheLibrariesStepInstalls::ThesePackages(packages) = what else {
            panic!("the mapping names this one");
        };
        assert_eq!(packages, THE_MAPPING[0].packages.to_vec());
        assert!(packages.contains(&"libgl1"));
    }

    /// **The real case, and it is what nothing here drove until 2026-09-22.**
    ///
    /// Every desk test passed `libX11.so.6` on its own — the one name the
    /// mapping was keyed on — so the step that refuses the other eight was
    /// never reached at the desk, and a bare machine met it every time. This
    /// drives the whole list the fourth pass watched.
    #[test]
    fn the_whole_list_a_bare_container_reports_is_placed_and_selects_one_command() {
        let asked: Vec<String> = WHAT_A_BARE_CONTAINER_REPORTED
            .iter()
            .map(|library| library.to_string())
            .collect();
        let what = what_to_install_for(&asked);
        let WhatTheLibrariesStepInstalls::ThesePackages(packages) = what else {
            panic!("every one of the nine has a key: {what:?}");
        };
        // **One entry answered for all nine, so the selection is that entry's
        // own list** — one command, every byte of it fixed at build time.
        assert_eq!(packages, THE_MAPPING[0].packages.to_vec());
        for named in ["libxext6", "libxkbcommon0", "libsm6", "libice6", "libgl1"] {
            assert!(packages.contains(&named), "{named} was not selected");
        }
    }

    /// **No library is keyed twice**, so no not-found name can select two
    /// entries and leave the step with a list no one command is fixed for.
    #[test]
    fn no_library_is_named_by_two_entries() {
        let mut named: Vec<&str> = THE_MAPPING
            .iter()
            .flat_map(|entry| entry.libraries.iter().copied())
            .collect();
        let how_many = named.len();
        named.sort_unstable();
        named.dedup();
        assert_eq!(named.len(), how_many, "a library is keyed by two entries");
    }

    /// **Every library looked up by name is keyed in the mapping**, so one the
    /// loader does not find selects a command rather than failing the step,
    /// and **its lookup is the shared line with its own name in it**.
    #[test]
    fn every_library_blender_opens_by_name_is_keyed_and_looked_up_by_the_shared_line() {
        for opened in WHAT_BLENDER_OPENS_BY_NAME {
            assert!(
                THE_MAPPING.iter().any(|entry| entry.libraries.contains(&opened.library)),
                "{} has no key",
                opened.library
            );
            assert_eq!(
                opened.the_lookup,
                crate::the_optix_library::THE_LOADERS_OWN_LOOKUP
                    .replace(crate::the_optix_library::THE_LIBRARY, opened.library)
            );
        }
    }

    #[test]
    fn a_library_the_mapping_does_not_name_fails_the_step_and_installs_nothing() {
        // Never a partial install: a step that installed what it could would
        // leave `ldd` unclean and the action claiming a check that did not
        // pass. And the machine-read name never reaches a command string.
        let what = what_to_install_for(&["libX11.so.6".to_string(), "libnowhere.so.9".to_string()]);
        assert_eq!(
            what,
            WhatTheLibrariesStepInstalls::TheMappingDoesNotNameThese(vec![
                "libnowhere.so.9".to_string()
            ])
        );
    }

    #[test]
    fn an_empty_not_found_list_is_a_step_with_nothing_to_do() {
        assert_eq!(
            what_to_install_for(&[]),
            WhatTheLibrariesStepInstalls::NothingWasMissing
        );
    }
}
