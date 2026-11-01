//! The boot registration, placed and read back on the machine.
//!
//! **[`crate::registration`] holds the rules — what a registration is confirmed
//! by, what the read-back is worth, and the four routes a container refuses.**
//! This module is the three acts and the two reads.
//!
//! # What the acts are, per machine kind
//!
//! **On a systemd machine all three are this crate's and none of them needs
//! elevated access**, which was observed whole: the unit, the enable, the
//! lingering grant to the login's own name, K5's read-back and K15's
//! confirmation all ran unprivileged and all succeeded.
//!
//! **On a container the hook's own install is root's and is `Server Setup`'s
//! elevated step (K11), and the other two acts are forbidden outright (K12).**
//! So this module's container branch refuses the enable and the grant, each
//! naming the clause that refuses it, and never reports a quiet success. **That
//! is why K11's half of [`crate::registration::PlacesTheRegistration`] has no
//! end here even now**: the record at [`crate::arriving_later`] says so.
//!
//! **What the container branch does do, from 2026-09-06, is write K16's
//! record.** The registration step is one act on both kinds — the record into
//! the package area, and then the registration's own content — and only the
//! second half differs. Before that, nothing in the workspace wrote the record
//! at all, so a machine's registration was placed and nothing said what had been
//! placed: K15 had nothing to compare against and package inspection reported
//! that half absent on every real machine, for ever.
//!
//! # The publications, all read 2026-09-04
//!
//! - `loginctl(1)`, `https://man7.org/linux/man-pages/man1/loginctl.1.html`.
//!   *"Enable/disable user lingering for one or more users. If enabled for a
//!   specific user, a user manager is spawned for the user at boot and kept
//!   around after logouts. This allows users who are not logged in to run
//!   long-running services."* And, of showing a user's properties: *"To select
//!   specific properties to show, use --property=."*
//! - `org.freedesktop.login1(5)`,
//!   `https://man7.org/linux/man-pages/man5/org.freedesktop.login1.5.html`, for
//!   the two property names read back: *"The Linger property shows whether
//!   lingering is enabled for this user."*, and *"State encodes the user state
//!   and is one of 'offline', 'lingering', 'online', 'active', or 'closing'."*
//! - `systemctl(1)`,
//!   `https://man7.org/linux/man-pages/man1/systemctl.1.html`, for the enable:
//!   *"Enable one or more units or unit instances. This will create a set of
//!   symlinks, as encoded in the [Install] sections of the indicated unit
//!   files."*
//!
//! **And the observed form of both commands**, which is what fixes the
//! spelling rather than a guess. Label: **observed**, on the rentals of
//! 2026-08-15 and 2026-08-18, in the evidence documents' own transcripts:
//! `loginctl enable-linger <name>` succeeding unprivileged where the bare form
//! failed, and `loginctl show-user <name> -p Linger -p State` answering
//! `Linger=yes` and `State=lingering`.
//!
//! # One fact wanted here that could not be got, and what stands in its place
//!
//! **`systemctl(1)`'s own description of its `--user` option could not be read.**
//! The page was fetched at the address above on 2026-09-04 and its options
//! section came back truncated before that option; three separate askings
//! returned the same. **So the option's meaning is not quoted here from its own
//! publication, and this is recorded rather than filled in from memory.**
//!
//! **What stands in its place is an observation of this project's own**, which
//! Rule 2 admits as a source when it is labelled: on the systemd rental of
//! 2026-08-18 the whole boot registration ran as a **user unit** under the
//! login's own configuration directory with `loginctl enable-linger` beside it,
//! and on the container of the same batch `systemctl --user enable` *"exits 0,
//! creates the symlink, and `is-enabled` afterwards answers `enabled`"* while
//! nothing ever ran — which is K12's own prohibition and is why this module
//! never issues it there.
//!
//! **What breaks if the option means something other than what the observation
//! shows:** the enable is issued against the system manager rather than the
//! login's own, which on an unprivileged login is refused loudly and fails the
//! placement before the commit; K5's read-back is unaffected, reading a
//! machine-set value by a different tool.

use std::path::{Path, PathBuf};

use crate::acting::durable;
use crate::acting::machine::{run, TheMachinesOwnLogin};
use crate::machine_kind::MachineKind;
use crate::registration::{
    BootRegistrationRecord, PlacesTheRegistration, ReadsTheLingerFlagBack,
    ReadsTheRegistrationBack, TheLingerReadBack, WhatTheRegistrationStepLeft,
};
use crate::the_program::records;
use crate::written_form;

/// The two property names read back, and no others.
pub const THE_TWO_PROPERTIES_READ_BACK: [&str; 2] = ["Linger", "State"];

/// The environment value K7 exports by hand where the provider creates no
/// login session.
pub const THE_RUNTIME_DIRECTORY_VARIABLE: &str = "XDG_RUNTIME_DIR";

/// **K7.** Nothing where the provider made a session; the derived value where it
/// did not.
///
/// **It is a free function so that every user-level command is handed the same
/// thing.** There are three of them — the enable, the lingering grant and G25's
/// supervisor start — and the start is issued from
/// [`crate::acting::package`], which is why the derivation is not a method of
/// the registration alone. K7 is exact that on a provider creating no login
/// session the user-level control tool cannot reach a bus without it.
pub fn the_environment_a_user_level_command_needs(
    login: &TheMachinesOwnLogin,
    the_runtime_directory_is_already_set: bool,
) -> Vec<(&'static str, String)> {
    if the_runtime_directory_is_already_set {
        Vec::new()
    } else {
        vec![(THE_RUNTIME_DIRECTORY_VARIABLE, login.the_runtime_directory())]
    }
}

/// The registration as this machine performs it.
///
/// **The two tools are fields rather than constants** so that what each is
/// handed can be watched at the desk, where neither exists. The acts themselves
/// are the real ones.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TheRegistrationOnTheMachine {
    /// The kind K1's table placed this machine as, read at this placement (K2).
    pub kind: MachineKind,
    /// The machine's own name and number for the login this runs as (K6, K7).
    pub login: TheMachinesOwnLogin,
    /// **The package area K16's record goes into.**
    ///
    /// **It is carried by the placement rather than passed to the write**, so
    /// that nothing can place a registration and choose not to record it. Where
    /// the area sits is `Job files and outputs` L4a's, and `Job files and
    /// outputs` L6 is where it sits among the five areas; the path is the
    /// caller's and none is synthesised here.
    pub package_area: PathBuf,
    /// The unit K4's registration is, by name.
    pub unit_name: String,
    /// The login manager's control tool.
    pub the_login_tool: String,
    /// The service manager's control tool.
    pub the_service_tool: String,
    /// Whether this program's own environment already carries the runtime
    /// directory. Where it does not, K7's value is exported by hand.
    pub the_runtime_directory_is_already_set: bool,
    /// **How long each of these commands is given to answer, carried from the
    /// caller** (K17).
    pub how_long_a_machine_command_is_given: std::time::Duration,
}

impl TheRegistrationOnTheMachine {
    /// The rented machine, with both tools by their published names and the
    /// environment read once.
    pub fn on(
        kind: MachineKind,
        package_area: &Path,
        unit_name: &str,
        how_long_a_machine_command_is_given: std::time::Duration,
    ) -> Result<Self, String> {
        Ok(TheRegistrationOnTheMachine {
            kind,
            login: TheMachinesOwnLogin::asked()?,
            package_area: package_area.to_path_buf(),
            unit_name: unit_name.to_string(),
            the_login_tool: "loginctl".to_string(),
            the_service_tool: "systemctl".to_string(),
            the_runtime_directory_is_already_set: std::env::var_os(THE_RUNTIME_DIRECTORY_VARIABLE)
                .is_some(),
            how_long_a_machine_command_is_given,
        })
    }

    /// **K7.** Nothing where the provider made a session; the derived value
    /// where it did not.
    pub fn the_environment_a_user_level_command_needs(&self) -> Vec<(&'static str, String)> {
        the_environment_a_user_level_command_needs(
            &self.login,
            self.the_runtime_directory_is_already_set,
        )
    }

    /// **Where K16's record sits, and the name has one home.**
    ///
    /// [`crate::the_program::records::THE_BOOT_REGISTRATION_RECORD`] is that
    /// home, and it is the same name `Server protocol and transfers` B4's
    /// package inspection composes its own read from — which is what makes the
    /// two ends of this record one record rather than two agreeing habits.
    pub fn the_boot_registration_record_sits_at(&self) -> PathBuf {
        self.package_area
            .join(records::THE_BOOT_REGISTRATION_RECORD)
    }

    fn a_container_refuses(&self, act: &str, because: &str) -> Result<(), String> {
        Err(format!(
            "{act} is not performed on a container: {because}. The machine kind read at this placement is a container."
        ))
    }
}

impl ReadsTheRegistrationBack for TheRegistrationOnTheMachine {
    /// **K15's read, and it is of content.**
    ///
    /// K19 is what makes this reach the file at all on a container: the root
    /// step must leave the path readable by the login that performs the
    /// read-back, and where it has not, the observed failure is
    /// `Permission denied` before the commit — which arrives here as the
    /// machine's own words and is [`crate::registration::confirm`]'s
    /// *could not be read* branch rather than a confirmation.
    fn the_content_at(&self, path: &str) -> Result<Vec<u8>, String> {
        std::fs::read(path).map_err(|why| format!("{path}: {why}"))
    }
}

impl ReadsTheLingerFlagBack for TheRegistrationOnTheMachine {
    /// **K5's read-back**, by the name the machine derived for itself.
    ///
    /// Both properties are asked for because the observed answer carries both,
    /// and the flag is taken from the line that names it rather than from a
    /// position — a reader that took the first line would be right by luck and
    /// wrong the day the tool orders them differently.
    fn the_linger_read_back(&self) -> Result<TheLingerReadBack, String> {
        let said = run(
            &self.the_login_tool,
            &[
                "show-user",
                &self.login.name,
                "-p",
                THE_TWO_PROPERTIES_READ_BACK[0],
                "-p",
                THE_TWO_PROPERTIES_READ_BACK[1],
            ],
            &self.the_environment_a_user_level_command_needs(),
            self.how_long_a_machine_command_is_given,
        )?;
        if !said.ended_at_zero {
            return Err(format!(
                "the login manager refused the read-back for {}: {}",
                self.login.name,
                said.as_text()
            ));
        }
        the_flag_in(&said.ordinary).ok_or_else(|| {
            format!(
                "the login manager answered without naming both {} and {}: {}",
                THE_TWO_PROPERTIES_READ_BACK[0],
                THE_TWO_PROPERTIES_READ_BACK[1],
                said.ordinary.trim()
            )
        })
    }
}

/// The value of one named property, taken from the line that names it.
///
/// **From the line that names it rather than from a position** — a reader that
/// took the first line would be right by luck and wrong the day the tool orders
/// them differently.
fn the_property_named(what_the_tool_answered: &str, property: &str) -> Option<String> {
    for line in what_the_tool_answered.lines() {
        if let Some(value) = line.trim().strip_prefix(&format!("{property}=")) {
            return Some(value.trim().to_string());
        }
    }
    None
}

/// **K5's read-back: both properties, each taken from the line that names it.**
///
/// K5 says *"the state and the linger flag are read for that login by name and
/// compared"*, so an answer naming one of the two is half a read-back and is not
/// one. **A property the tool did not answer is an absence and never a no.** K5's
/// whole point is that a placement whose read-back does not show lingering
/// granted fails before the commit; reading a missing answer as *not granted*
/// would be the same outcome by luck, and reading it as granted would be the
/// silent failure K5 exists for. It is neither: the caller is told the tool did
/// not name it.
///
/// **The state is carried as the machine's own word and nothing decides on it
/// here**, for the reason written at [`TheLingerReadBack::the_state`].
pub fn the_flag_in(what_the_tool_answered: &str) -> Option<TheLingerReadBack> {
    let granted = the_property_named(what_the_tool_answered, THE_TWO_PROPERTIES_READ_BACK[0])?;
    let state = the_property_named(what_the_tool_answered, THE_TWO_PROPERTIES_READ_BACK[1])?;
    Some(TheLingerReadBack {
        lingering_is_granted: granted.eq_ignore_ascii_case("yes"),
        the_state: state,
    })
}

impl PlacesTheRegistration for TheRegistrationOnTheMachine {
    /// **The registration step, and it is one act: K16's record, and then the
    /// registration's own content.**
    ///
    /// **The record goes first, and that is `Rented-server runtime` K16b** —
    /// the record written durably before the registration it describes. The
    /// reason is written at the trait: a step interrupted between the two
    /// leaves a record whose content is not on the machine, which is what K15's
    /// read-back is for, where the other order leaves live boot bytes with no
    /// record at all. **Durably** is the whole of
    /// [`crate::acting::durable::publish`] — `State, identity and recovery`
    /// §1.3 rule 6's four steps — completing before the registration's own
    /// write begins, rather than two writes issued in an order. **K16b binds
    /// the rollback direction too and nothing here performs a rollback**; the
    /// clause that completes G14 with the record's own restore is deliberately
    /// not built and is recorded at [`crate::arriving_later`], which is where
    /// its marker sits.
    ///
    /// **The record is written on both machine kinds, which is
    /// `Rented-server runtime` K16a, and the hook's install is the only half
    /// that differs.** The write above sits before the kind is looked at, so
    /// there is no kind on which a placement records nothing, and nothing in
    /// this module elevates: K16a's *non-root placement* is this one, on both
    /// kinds. On a container the write of the hook itself
    /// is K11's elevated step and `Server Setup` owns the elevation, with K19's
    /// readability obligation on that same step — so what comes back is the
    /// record written and the hook left to the root step, never a quiet success
    /// and never a placement that recorded nothing. **Without the record there,
    /// K15 has nothing to compare the installed hook against**, and the recycle
    /// case — the hook replaced by an 82-byte provider default — is caught by
    /// nothing at all.
    ///
    /// **A record composed for the other machine kind is refused before
    /// anything is written.** K2 makes the kind a machine-derived value re-taken
    /// at every placement, so a record disagreeing with it is a caller that
    /// composed against a stale reading, and writing either half of it would put
    /// the wrong text on the machine.
    fn place_the_registration(
        &self,
        record: &BootRegistrationRecord,
    ) -> Result<WhatTheRegistrationStepLeft, String> {
        if record.kind != self.kind {
            return Err(format!(
                "the boot-registration record was composed for a {:?} machine and the kind read at \
                 this placement is {:?}",
                record.kind, self.kind
            ));
        }

        // **K16, and it is by value.** Nothing here derives the record from what
        // is on the machine at the moment it is read.
        durable::make_the_area(&self.package_area).map_err(|why| why.as_text())?;
        let bytes = written_form::write(record).map_err(|why| why.what_the_reader_said)?;
        durable::publish(&self.the_boot_registration_record_sits_at(), &bytes)
            .map_err(|why| why.as_text())?;

        if self.kind == MachineKind::Container {
            return Ok(WhatTheRegistrationStepLeft::TheRecordAndTheHookLeftToTheRootStep);
        }

        // **K4's first act.** The unit's own content, written durably at the
        // path the record names.
        let at = Path::new(&record.at_path);
        if let Some(directory) = at.parent() {
            durable::make_the_area(directory).map_err(|why| why.as_text())?;
        }
        durable::publish(at, &record.content).map_err(|why| why.as_text())?;
        Ok(WhatTheRegistrationStepLeft::TheRecordAndTheRegistrationItself)
    }

    /// **K4's second act.**
    ///
    /// **K12 forbids it on a container and this is that prohibition performed.**
    /// The observed reason is that it does not merely report success at the
    /// moment it is run — *"`is-enabled` afterwards answers `enabled`"* while
    /// the unit will never run — so a later reader is answered wrongly too, and
    /// no read-back this document has separates it.
    fn enable_the_unit(&self) -> Result<(), String> {
        if self.kind == MachineKind::Container {
            return self.a_container_refuses(
                "enabling a unit",
                "K12 makes the service-manager enable a prohibition rather than a preference, because it reports success and the unit will never run",
            );
        }
        let said = run(
            &self.the_service_tool,
            &["--user", "enable", &self.unit_name],
            &self.the_environment_a_user_level_command_needs(),
            self.how_long_a_machine_command_is_given,
        )?;
        if said.ended_at_zero {
            Ok(())
        } else {
            Err(format!(
                "the service manager refused to enable {}: {}",
                self.unit_name,
                said.as_text()
            ))
        }
    }

    /// **K6's grant, by name, and the name is the machine's own.**
    ///
    /// The bare form is never issued: it failed on the observed provider —
    /// `Could not enable linger: No such device or address` — because *self* is
    /// derived from a session that does not exist under a provider that creates
    /// no login session.
    ///
    /// **Enabling alone is not the registration and its failure is silent**, so
    /// nothing here reads a successful grant as a completed registration; K5's
    /// read-back is what completes it.
    fn grant_lingering(&self) -> Result<(), String> {
        if self.kind == MachineKind::Container {
            return self.a_container_refuses(
                "granting lingering",
                "the lingering grant belongs to the systemd arrangement of K4 and the container arrangement of K10 does not use the permission at all",
            );
        }
        let said = run(
            &self.the_login_tool,
            &["enable-linger", &self.login.name],
            &self.the_environment_a_user_level_command_needs(),
            self.how_long_a_machine_command_is_given,
        )?;
        if said.ended_at_zero {
            Ok(())
        } else {
            Err(format!(
                "the login manager refused the grant for {}: {}",
                self.login.name,
                said.as_text()
            ))
        }
    }
}

/// Where K4's unit sits, under the login's own configuration directory.
///
/// Label: **observed** — the systemd rental of 2026-08-15 placed it at
/// `~/.config/systemd/user/`, unprivileged, and the service returned 6.71
/// seconds into a new boot. **The home directory is the caller's**, because a
/// path is never synthesised here from anything but what the caller names.
pub fn where_a_user_unit_sits(home: &Path) -> PathBuf {
    home.join(".config").join("systemd").join("user")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::registration::{confirm, BootRegistrationRecord, RegistrationConfirmation};

    fn an_area() -> PathBuf {
        let area = std::env::temp_dir().join(format!(
            "smoozy-runtime-registration-{}-{:?}",
            std::process::id(),
            {
                static NEXT: std::sync::atomic::AtomicUsize =
                    std::sync::atomic::AtomicUsize::new(0);
                NEXT.fetch_add(1, std::sync::atomic::Ordering::SeqCst)
            }
        ));
        std::fs::create_dir_all(&area).expect("the area is made");
        area
    }

    /// A tool that answers as the real one does and records what it was handed.
    fn a_tool_that(at: &Path, prints: &str, ends_at: i32) {
        use std::io::Write;
        use std::os::unix::fs::PermissionsExt;
        let mut file = std::fs::File::create(at).expect("the stand-in is written");
        write!(
            file,
            "#!/bin/sh\nprintf '%s\\n' \"$@\" > \"$0.arguments\"\nprintf '%s' \"$XDG_RUNTIME_DIR\" > \"$0.runtime\"\ncat <<'THEANSWER'\n{prints}\nTHEANSWER\nexit {ends_at}\n"
        )
        .expect("the stand-in is written");
        drop(file);
        let mut permission = std::fs::metadata(at).unwrap().permissions();
        permission.set_mode(0o700);
        std::fs::set_permissions(at, permission).expect("the stand-in may be run");
    }

    fn what_it_was_handed(tool: &Path) -> Vec<String> {
        std::fs::read_to_string(format!("{}.arguments", tool.display()))
            .expect("the stand-in recorded its arguments")
            .lines()
            .map(str::to_string)
            .collect()
    }

    fn a_placement(area: &Path, kind: MachineKind, prints: &str, ends_at: i32) -> TheRegistrationOnTheMachine {
        let login_tool = area.join("the login tool");
        let service_tool = area.join("the service tool");
        a_tool_that(&login_tool, prints, ends_at);
        a_tool_that(&service_tool, prints, ends_at);
        TheRegistrationOnTheMachine {
            kind,
            login: TheMachinesOwnLogin {
                number: crate::acting::machine::THE_TEST_LOGINS_NUMBER,
                name: "the machines own name".into(),
            },
            package_area: area.join("package"),
            unit_name: "smoozy-supervisor.service".into(),
            the_login_tool: login_tool.display().to_string(),
            the_service_tool: service_tool.display().to_string(),
            the_runtime_directory_is_already_set: false,
            how_long_a_machine_command_is_given: std::time::Duration::from_secs(30),
        }
    }

    #[test]
    fn the_grant_names_the_login_the_machine_derived_and_never_a_carried_value() {
        let area = an_area();
        let placement = a_placement(&area, MachineKind::Systemd, "", 0);
        placement.grant_lingering().expect("the grant is made");
        let handed = what_it_was_handed(Path::new(&placement.the_login_tool));
        assert_eq!(handed, vec!["enable-linger", "the machines own name"]);
        assert!(
            !handed.iter().any(|word| word == "--self" || word.is_empty()),
            "the bare form is never issued"
        );
        std::fs::remove_dir_all(&area).ok();
    }

    /// **K7's value reaches the one command's environment and no further.**
    ///
    /// **The login's number is the tests' own and not a real one**
    /// ([`crate::acting::machine::THE_TEST_LOGINS_NUMBER`]), because a child
    /// started from here inherits this program's own environment: written
    /// around a number a real login could have, this test would pass on any
    /// Linux machine whose session already carried the matching directory —
    /// including the hosted runner, whose own value is `/run/user/1001` —
    /// **with the export removed altogether.** It never fired, and the same
    /// fault in the other direction, in [`crate::acting::package`], did.
    #[test]
    fn the_runtime_directory_is_exported_by_hand_where_the_provider_made_no_session() {
        let area = an_area();
        let placement = a_placement(&area, MachineKind::Systemd, "", 0);
        placement.grant_lingering().expect("the grant is made");
        let seen = std::fs::read_to_string(format!("{}.runtime", placement.the_login_tool)).unwrap();
        assert_eq!(
            seen,
            crate::acting::machine::the_test_logins_runtime_directory()
        );
        std::fs::remove_dir_all(&area).ok();
    }

    #[test]
    fn where_the_provider_made_a_session_nothing_is_exported() {
        let area = an_area();
        let mut placement = a_placement(&area, MachineKind::Systemd, "", 0);
        placement.the_runtime_directory_is_already_set = true;
        assert!(placement.the_environment_a_user_level_command_needs().is_empty());
        std::fs::remove_dir_all(&area).ok();
    }

    #[test]
    fn the_read_back_takes_both_properties_from_the_lines_that_name_them() {
        let area = an_area();
        let placement = a_placement(&area, MachineKind::Systemd, "State=lingering\nLinger=yes", 0);
        let read_back = placement.the_linger_read_back().expect("it answers");
        assert!(read_back.lingering_is_granted);
        assert_eq!(read_back.the_state, "lingering");
        std::fs::remove_dir_all(&area).ok();
    }

    /// **K5 reads both, so an answer carrying only the flag is a refusal rather
    /// than a pass.** Half a read-back is not the read-back the clause names.
    #[test]
    fn a_read_back_missing_the_state_line_is_a_refusal_rather_than_a_pass() {
        let area = an_area();
        let placement = a_placement(&area, MachineKind::Systemd, "Linger=yes", 0);
        let why = placement
            .the_linger_read_back()
            .expect_err("it named only one of the two");
        assert!(why.contains("State"));
        std::fs::remove_dir_all(&area).ok();
    }

    #[test]
    fn a_read_back_showing_lingering_off_is_not_granted() {
        let area = an_area();
        let placement = a_placement(&area, MachineKind::Systemd, "Linger=no\nState=offline", 0);
        assert!(!placement.the_linger_read_back().expect("it answers").lingering_is_granted);
        std::fs::remove_dir_all(&area).ok();
    }

    /// **An answer that names no flag is neither a yes nor a no.**
    #[test]
    fn an_answer_that_names_no_flag_is_a_refusal_rather_than_a_no() {
        let area = an_area();
        let placement = a_placement(&area, MachineKind::Systemd, "State=offline", 0);
        let why = placement.the_linger_read_back().expect_err("it names nothing");
        assert!(why.contains("Linger"));
        std::fs::remove_dir_all(&area).ok();
    }

    #[test]
    fn a_login_manager_that_refuses_is_carried_verbatim() {
        let area = an_area();
        let placement = a_placement(&area, MachineKind::Systemd, "Could not enable linger", 1);
        let why = placement.grant_lingering().expect_err("it refused");
        assert!(why.contains("ended at 1"));
        std::fs::remove_dir_all(&area).ok();
    }

    #[test]
    fn the_unit_is_written_and_reads_back_as_the_record_says() {
        let area = an_area();
        let placement = a_placement(&area, MachineKind::Systemd, "", 0);
        let at = area.join("home/.config/systemd/user/smoozy-supervisor.service");
        let record = BootRegistrationRecord {
            kind: MachineKind::Systemd,
            at_path: at.display().to_string(),
            content: b"the unit's own content".to_vec(),
        };
        assert_eq!(
            placement.place_the_registration(&record),
            Ok(WhatTheRegistrationStepLeft::TheRecordAndTheRegistrationItself)
        );
        let held = placement.the_content_at(&record.at_path).expect("it reads back");
        assert_eq!(
            confirm(&record, Ok(&held)),
            RegistrationConfirmation::ContentMatchesTheRecord
        );
        std::fs::remove_dir_all(&area).ok();
    }

    /// **K16, and it is the half nothing in the workspace used to write.** The
    /// same act that puts the unit on the machine leaves the record beside it,
    /// in the package area, holding the content by value — and it reads back as
    /// the record that was placed.
    #[test]
    fn the_same_act_leaves_the_record_in_the_package_area_holding_the_content_by_value() {
        let area = an_area();
        let placement = a_placement(&area, MachineKind::Systemd, "", 0);
        let record = BootRegistrationRecord {
            kind: MachineKind::Systemd,
            at_path: area
                .join("home/.config/systemd/user/smoozy-supervisor.service")
                .display()
                .to_string(),
            content: b"[Unit]\nDescription=Smoozy server supervisor\n".to_vec(),
        };
        placement
            .place_the_registration(&record)
            .expect("the step is performed");

        let at = placement.the_boot_registration_record_sits_at();
        assert_eq!(at, area.join("package").join("boot-registration"));
        let bytes = durable::read(&at)
            .expect("the record is readable")
            .expect("the record is there");
        let back: BootRegistrationRecord =
            written_form::read(&bytes).expect("it reads back in this crate's own form");
        assert_eq!(&back, &record);
        std::fs::remove_dir_all(&area).ok();
    }

    /// **A container leaves the record and leaves the hook to the root step**,
    /// which is K11 with K19's obligation on it — and it runs no tool.
    ///
    /// **The record on this kind is what K15 has to compare the installed hook
    /// against**, so a container placement that recorded nothing would leave the
    /// recycle case — the hook replaced by an 82-byte provider default — caught
    /// by nothing.
    #[test]
    fn a_container_leaves_the_record_and_the_hook_to_the_root_step() {
        let area = an_area();
        let placement = a_placement(&area, MachineKind::Container, "", 0);
        let record = BootRegistrationRecord {
            kind: MachineKind::Container,
            at_path: "/root/onstart.sh".into(),
            content: b"#!/bin/sh\n# Smoozy's boot registration\n".to_vec(),
        };
        assert_eq!(
            placement.place_the_registration(&record),
            Ok(WhatTheRegistrationStepLeft::TheRecordAndTheHookLeftToTheRootStep)
        );
        let bytes = durable::read(&placement.the_boot_registration_record_sits_at())
            .expect("the record is readable")
            .expect("the record is there");
        let back: BootRegistrationRecord = written_form::read(&bytes).expect("it reads back");
        assert_eq!(back.content, record.content);
        assert!(
            !Path::new(&format!("{}.arguments", placement.the_login_tool)).exists(),
            "no tool was run at all"
        );
        std::fs::remove_dir_all(&area).ok();
    }

    /// **K2: a record composed for the other kind is refused before either half
    /// is written**, so nothing lands in the package area and nothing lands on
    /// the machine.
    #[test]
    fn a_record_composed_for_the_other_machine_kind_is_refused_before_anything_is_written() {
        let area = an_area();
        let placement = a_placement(&area, MachineKind::Systemd, "", 0);
        let record = BootRegistrationRecord {
            kind: MachineKind::Container,
            at_path: area.join("hook").display().to_string(),
            content: b"the wrong kind's own text".to_vec(),
        };
        let why = placement
            .place_the_registration(&record)
            .expect_err("the kinds disagree");
        assert!(why.contains("Container") && why.contains("Systemd"), "{why}");
        assert_eq!(
            durable::read(&placement.the_boot_registration_record_sits_at())
                .expect("the read answers"),
            None
        );
        assert!(!area.join("hook").exists());
        std::fs::remove_dir_all(&area).ok();
    }

    /// **K15's own case, run rather than reasoned**: the bytes are replaced and
    /// the read-back refuses loudly, before the commit.
    #[test]
    fn a_registration_whose_bytes_were_replaced_refuses_loudly() {
        let area = an_area();
        let placement = a_placement(&area, MachineKind::Systemd, "", 0);
        let at = area.join("hook");
        let record = BootRegistrationRecord {
            kind: MachineKind::Systemd,
            at_path: at.display().to_string(),
            content: b"smoozy's own hook, which is longer".to_vec(),
        };
        placement
            .place_the_registration(&record)
            .expect("it is written");
        // The provider's default, dated at the recycle moment.
        std::fs::write(&at, b"the provider's own default").unwrap();
        let held = placement.the_content_at(&record.at_path).expect("it reads back");
        assert!(matches!(
            confirm(&record, Ok(&held)),
            RegistrationConfirmation::ContentDiffers { .. }
        ));
        std::fs::remove_dir_all(&area).ok();
    }

    /// **K19's failure, arriving as the plan says it does.** A path the
    /// read-back's login cannot reach is *could not be read* and never a
    /// confirmation.
    #[test]
    fn a_path_the_read_back_cannot_reach_is_not_a_confirmation() {
        let area = an_area();
        let placement = a_placement(&area, MachineKind::Systemd, "", 0);
        let record = BootRegistrationRecord {
            kind: MachineKind::Container,
            at_path: area.join("root/onstart.sh").display().to_string(),
            content: b"smoozy's own hook".to_vec(),
        };
        let held = placement.the_content_at(&record.at_path);
        assert!(matches!(
            confirm(&record, held.as_deref().map_err(|why| why.clone())),
            RegistrationConfirmation::TheContentCouldNotBeRead { .. }
        ));
        std::fs::remove_dir_all(&area).ok();
    }

    /// **The container's two refusals, each naming its own clause** — and the
    /// hook's own install, which is left to the root step rather than performed.
    #[test]
    fn a_container_refuses_both_user_level_acts_and_runs_nothing() {
        let area = an_area();
        let placement = a_placement(&area, MachineKind::Container, "", 0);
        assert_eq!(
            placement.place_the_registration(&BootRegistrationRecord {
                kind: MachineKind::Container,
                at_path: "/root/onstart.sh".into(),
                content: b"x".to_vec(),
            }),
            Ok(WhatTheRegistrationStepLeft::TheRecordAndTheHookLeftToTheRootStep),
            "the hook's own install is K11's elevated step and is not performed here"
        );
        let why = placement.enable_the_unit().expect_err("the enable is forbidden");
        assert!(why.contains("K12"));
        let why = placement.grant_lingering().expect_err("the grant is not the container's");
        assert!(why.contains("K10"));
        assert!(
            !Path::new(&format!("{}.arguments", placement.the_login_tool)).exists(),
            "no tool was run at all"
        );
        std::fs::remove_dir_all(&area).ok();
    }

    #[test]
    fn the_enable_names_the_unit_and_the_logins_own_service_manager() {
        let area = an_area();
        let placement = a_placement(&area, MachineKind::Systemd, "", 0);
        placement.enable_the_unit().expect("it is enabled");
        assert_eq!(
            what_it_was_handed(Path::new(&placement.the_service_tool)),
            vec!["--user", "enable", "smoozy-supervisor.service"]
        );
        std::fs::remove_dir_all(&area).ok();
    }

    #[test]
    fn a_user_units_place_is_under_the_logins_own_configuration_directory() {
        assert_eq!(
            where_a_user_unit_sits(Path::new("/home/smoozy")),
            PathBuf::from("/home/smoozy/.config/systemd/user")
        );
    }
}
