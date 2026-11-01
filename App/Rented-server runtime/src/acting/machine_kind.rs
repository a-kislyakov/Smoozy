//! K1's two readings, taken from the machine itself.
//!
//! **The table that turns them into an arrangement is [`crate::machine_kind`]'s
//! and is not repeated here.** What is here is only the asking.
//!
//! # The three things asked, and where each comes from
//!
//! **The container marker file.** K1 names it beside the tool, in the evidence's
//! own words: *"Tell them apart with `systemd-detect-virt` and `/.dockerenv`
//! before choosing"*. Label: **observed**, on the container of that batch.
//!
//! **The tool, asked with its own container option rather than by matching the
//! name it prints.** `systemd-detect-virt(1)`, read 2026-09-04 at
//! `https://man7.org/linux/man-pages/man1/systemd-detect-virt.1.html`:
//! *"--container: Only detects container virtualization (i.e. shared kernel
//! virtualization)."*, and *"If a virtualization technology is detected, 0 is
//! returned, a non-zero code otherwise."* **That option is what disposes of
//! K1's own near-miss** — a container inside a VMware virtual machine, where
//! the bare reading says one thing and the wider form says another — because a
//! reading limited to container virtualization cannot report the outer one.
//! `Servers` H12's own reading of this table takes the same option for the same
//! reason, and the two must stay identical.
//!
//! **Whether systemd is the machine's first process.** K1's second row asks it
//! and names no mechanism. `sd_booted(3)`, read 2026-09-04 at
//! `https://man7.org/linux/man-pages/man3/sd_booted.3.html`, is systemd's own
//! answer to exactly that question — *"sd_booted() checks whether the system was
//! booted up using the systemd init system."* — and the same page states the
//! check it performs: the directory `/run/systemd/system/` exists. **The
//! directory is read rather than the library called**, because linking a system
//! library into a package that is placed on a bare machine would make the
//! package's own placement depend on that library being there, which G8 turns
//! into a version this arrangement refuses.
//!
//! # What this asking cannot do, said rather than left to be found
//!
//! **The tool answers by its exit status and this module carries the name it
//! printed, so a name carried here is a container technology by the tool's own
//! answer.** K1's table reads it as that and matches it against no list.
//! **There is no list of technology names anywhere in this crate**, and one that
//! was here until 2026-09-04 has been taken out: K1 carries no such list, so
//! keeping one could only narrow the clause, and it placed a machine answering
//! at zero under an unlisted name as a systemd machine. `Servers` H12's own
//! reading of this table takes the same option for the same reason, and the two
//! are identical.
//!
//! **What the exit status cannot say is which technology it detected**, so a
//! tool that answers at zero and prints nothing carries an empty name. That is
//! still a container answer and K1's first row places it as one.
//!
//! **And this module reads nothing about which start-up mechanism the provider
//! offers**, which is the *What holds* paragraph of the plan's own Section 16
//! item 16 and the reason K3's refusal cannot fire on a stored-field provider.
//! See [`crate::arriving_later`].

use std::path::PathBuf;
use std::time::Duration;

use crate::acting::machine::{run, the_machine_holds};
use crate::machine_kind::{ReadsTheMachineKind, TheReadingsCouldNotBeTaken, TheTwoReadings};

/// **K1's container marker file**, under whatever root the reader is pointed at.
pub const THE_CONTAINER_MARKER_FILE: &str = ".dockerenv";

/// **`sd_booted(3)`'s own check**, under that same root.
pub const THE_DIRECTORY_SYSTEMD_MAKES_WHEN_IT_IS_THE_FIRST_PROCESS: &str = "run/systemd/system";

/// The tool K1 names.
pub const THE_TOOL: &str = "systemd-detect-virt";

/// The machine, asked directly.
///
/// **The root is a field rather than a constant**, so the two presence readings
/// can be put to a directory of the caller's own. That is what lets them be run
/// at the desk, on a machine that is neither of K1's two kinds, without the
/// readings themselves being stood in for.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TheMachineItselfAnswers {
    /// The machine's own root. `/` on a rented machine.
    pub root: PathBuf,
    /// The tool, by name.
    pub the_tool: String,
    /// **How long the tool is given to answer, carried from the caller** (K17).
    /// A tool that never returns would otherwise hold the detection for ever.
    pub how_long_the_tool_is_given: Duration,
}

impl TheMachineItselfAnswers {
    /// The rented machine, with the caller's own deadline for the tool.
    pub fn the_rented_machine(how_long_the_tool_is_given: Duration) -> Self {
        TheMachineItselfAnswers {
            root: PathBuf::from("/"),
            the_tool: THE_TOOL.to_string(),
            how_long_the_tool_is_given,
        }
    }

    fn under(&self, path: &str) -> PathBuf {
        self.root.join(path)
    }
}

impl ReadsTheMachineKind for TheMachineItselfAnswers {
    fn the_two_readings(&self) -> Result<TheTwoReadings, TheReadingsCouldNotBeTaken> {
        // **A tool that cannot be run at all is not a container answer and is
        // not a refusal either.** On the stock image read on 2026-08-22 the
        // service manager is offline and neither the recurring boot entry nor
        // its tools are installed, so an absent tool is the ordinary case there
        // — and the marker file is the limb that answers on exactly that
        // machine. K3's refusal is reached only when every limb says no.
        let what_the_tool_named = match run(
            &self.the_tool,
            &["--container"],
            &[],
            self.how_long_the_tool_is_given,
        ) {
            Ok(said) if said.ended_at_zero => Some(said.named().to_string()),
            Ok(_) => None,
            Err(_) => None,
        };
        Ok(TheTwoReadings {
            the_container_marker_file_is_present: the_machine_holds(&self.under(
                THE_CONTAINER_MARKER_FILE,
            )),
            what_the_tool_named,
            systemd_is_the_first_process: the_machine_holds(
                &self.under(THE_DIRECTORY_SYSTEMD_MAKES_WHEN_IT_IS_THE_FIRST_PROCESS),
            ),
        })
    }
}

/// A tool that answers as the real one does, written into `at` for a reading
/// this project cannot take on the machine that would answer it.
///
/// **This stands in for the tool and never for the reading.** The reading is
/// [`TheMachineItselfAnswers::the_two_readings`] above, run whole; what is
/// replaced is the program at the other end of it, which is the machine's and
/// not Smoozy's.
#[cfg(test)]
fn a_tool_that_answers(at: &std::path::Path, naming: Option<&str>) {
    use std::io::Write;
    use std::os::unix::fs::PermissionsExt;
    let mut file = std::fs::File::create(at).expect("the stand-in is written");
    match naming {
        Some(named) => write!(file, "#!/bin/sh\necho {named}\nexit 0\n"),
        None => write!(file, "#!/bin/sh\nexit 1\n"),
    }
    .expect("the stand-in is written");
    drop(file);
    let mut permission = std::fs::metadata(at).unwrap().permissions();
    permission.set_mode(0o700);
    std::fs::set_permissions(at, permission).expect("the stand-in may be run");
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;
    use crate::machine_kind::{place, MachineKind, Placement};

    fn a_root() -> PathBuf {
        let root = std::env::temp_dir().join(format!(
            "smoozy-runtime-kind-{}-{:?}",
            std::process::id(),
            {
                static NEXT: std::sync::atomic::AtomicUsize =
                    std::sync::atomic::AtomicUsize::new(0);
                NEXT.fetch_add(1, std::sync::atomic::Ordering::SeqCst)
            }
        ));
        std::fs::create_dir_all(&root).expect("the root is made");
        root
    }

    fn asking(root: &Path, the_tool: &Path) -> TheMachineItselfAnswers {
        TheMachineItselfAnswers {
            root: root.to_path_buf(),
            the_tool: the_tool.display().to_string(),
            how_long_the_tool_is_given: Duration::from_secs(30),
        }
    }

    #[test]
    fn a_machine_with_the_marker_file_and_no_tool_at_all_reads_as_a_container() {
        let root = a_root();
        std::fs::write(root.join(THE_CONTAINER_MARKER_FILE), b"").unwrap();
        let readings = asking(&root, Path::new("/no/such/tool"))
            .the_two_readings()
            .expect("the readings are taken");
        assert!(readings.the_container_marker_file_is_present);
        assert_eq!(readings.what_the_tool_named, None);
        assert_eq!(place(&readings), Placement::Use(MachineKind::Container));
        std::fs::remove_dir_all(&root).ok();
    }

    #[test]
    fn the_tool_answering_at_zero_carries_the_name_it_printed() {
        let root = a_root();
        let tool = root.join("the tool");
        a_tool_that_answers(&tool, Some("docker"));
        let readings = asking(&root, &tool)
            .the_two_readings()
            .expect("the readings are taken");
        assert_eq!(readings.what_the_tool_named.as_deref(), Some("docker"));
        assert_eq!(place(&readings), Placement::Use(MachineKind::Container));
        std::fs::remove_dir_all(&root).ok();
    }

    #[test]
    fn the_tool_answering_above_zero_names_nothing() {
        let root = a_root();
        let tool = root.join("the tool");
        a_tool_that_answers(&tool, None);
        let readings = asking(&root, &tool)
            .the_two_readings()
            .expect("the readings are taken");
        assert_eq!(readings.what_the_tool_named, None);
        std::fs::remove_dir_all(&root).ok();
    }

    #[test]
    fn a_machine_with_systemds_own_directory_and_neither_container_limb_reads_as_systemd() {
        let root = a_root();
        let tool = root.join("the tool");
        a_tool_that_answers(&tool, None);
        std::fs::create_dir_all(root.join(THE_DIRECTORY_SYSTEMD_MAKES_WHEN_IT_IS_THE_FIRST_PROCESS))
            .unwrap();
        let readings = asking(&root, &tool)
            .the_two_readings()
            .expect("the readings are taken");
        assert!(readings.systemd_is_the_first_process);
        assert_eq!(place(&readings), Placement::Use(MachineKind::Systemd));
        std::fs::remove_dir_all(&root).ok();
    }

    /// **K1's near-miss, taken through the real asking.** A container nested in
    /// a virtual machine holds the marker file and the tool's container option
    /// names the inner technology; the table places it as a container, which is
    /// the answer that keeps a service-manager registration off a box with no
    /// service manager.
    #[test]
    fn a_container_nested_in_a_virtual_machine_is_placed_as_a_container() {
        let root = a_root();
        let tool = root.join("the tool");
        a_tool_that_answers(&tool, Some("docker"));
        std::fs::write(root.join(THE_CONTAINER_MARKER_FILE), b"").unwrap();
        std::fs::create_dir_all(root.join(THE_DIRECTORY_SYSTEMD_MAKES_WHEN_IT_IS_THE_FIRST_PROCESS))
            .unwrap();
        let readings = asking(&root, &tool)
            .the_two_readings()
            .expect("the readings are taken");
        assert_eq!(place(&readings), Placement::Use(MachineKind::Container));
        std::fs::remove_dir_all(&root).ok();
    }

    /// **A machine that answers no limb reaches K3's refusal**, which is a
    /// placement that does not happen rather than a guessed arrangement.
    #[test]
    fn a_machine_that_is_neither_reaches_the_refusal() {
        let root = a_root();
        let tool = root.join("the tool");
        a_tool_that_answers(&tool, None);
        let readings = asking(&root, &tool)
            .the_two_readings()
            .expect("the readings are taken");
        assert_eq!(place(&readings), Placement::NotOneThisPackageCanRegisterOn);
        std::fs::remove_dir_all(&root).ok();
    }

    /// **A technology nobody here has met, taken through the real asking.** The
    /// tool answered at zero under its container option and printed a name this
    /// project has never seen. **K1's first row places it as a container**, and
    /// systemd's own directory being there does not change that.
    ///
    /// **This test replaces one that asserted the opposite.** Until 2026-09-04
    /// [`crate::machine_kind`] matched the printed name against a closed list of
    /// nine and this machine fell through to the second row — a systemd
    /// registration on a box with no service manager.
    #[test]
    fn a_technology_nobody_here_has_met_is_still_a_container() {
        let root = a_root();
        let tool = root.join("the tool");
        a_tool_that_answers(&tool, Some("a-technology-nobody-has-listed"));
        std::fs::create_dir_all(root.join(THE_DIRECTORY_SYSTEMD_MAKES_WHEN_IT_IS_THE_FIRST_PROCESS))
            .unwrap();
        let readings = asking(&root, &tool)
            .the_two_readings()
            .expect("the readings are taken");
        assert_eq!(
            readings.what_the_tool_named.as_deref(),
            Some("a-technology-nobody-has-listed")
        );
        assert_eq!(place(&readings), Placement::Use(MachineKind::Container));
        std::fs::remove_dir_all(&root).ok();
    }
}
