//! **`Rented-server runtime` K4 to K20, performed from the desktop** — the
//! boot-registration record, the registration itself, the two read-backs, and
//! the one act a container leaves to somebody else.
//!
//! # What crosses on which mechanism, and why it is not all one
//!
//! **The registration's own bytes go over the file-transfer subsystem**, as a
//! protocol field, because they are a file. **The three user-level commands go
//! over the command channel**, because they are not: the enable, the lingering
//! grant and the read-back are asked of the machine's own login and service
//! managers.
//!
//! **Every one of those commands is a fixed literal, and the one value they
//! need originates on the machine.** `Rented-server runtime` K6 forbids
//! Smoozy carrying the login's name: *"Lingering is requested by name and never
//! as *self*, and the name is the machine's own, derived at execution."* The
//! bare form failed on the observed box — `Could not enable linger: No such
//! device or address` — because *self* is derived from a session that does not
//! exist under a provider that creates no login session. **So the name rides
//! §1.1's fourth route, machine self-derivation**, and the command string stays
//! constant.
//!
//! **K7's runtime directory rides the same route**, as a variable-assignment
//! prefix the shell exports *"for the execution environment of the command"* and
//! no further. On the observed box the provider had turned login sessions off,
//! so no per-user runtime directory existed and the user-level control tool
//! could not reach a bus without it.
//!
//! **It is exported on every invocation rather than only where a session is
//! missing, and that is this side's own decision.** The machine's own half asks
//! its environment whether the value is already set; **the desktop has no such
//! question to ask** — it is not the process the value would be set for. What it
//! sets it to is the value a session would have set it to, composed by the same
//! derivation from the same credentials, so an invocation under a provider that
//! did create a session is handed the value it already had.
//!
//! # The container's root step, performed here since 2026-09-21
//!
//! ~~**The hook on a container is root's** (`Rented-server runtime` K11) and
//! `Server Setup` owns the elevation. What is performed here on that machine
//! kind is K16's record and nothing else, which is
//! `Rented-server runtime` K16a's own ending — *"a hand-over and not a
//! refusal"*.~~ **STRUCK 2026-09-21, step 10c: the hand-over pointed at nothing,
//! and the second pass on a real rental found every container placement failing
//! at K15 with the record's path empty.** The hook is still root's and the
//! elevation is still `Server Setup`'s — **its commands are**, at that crate's
//! `elevated.rs`, and they reach this walk as [`TheRootStepOnAContainer`]. What
//! this side does with them is the order: the hook's current content read, the
//! record written (K16b), the hook installed at the provider's path, and
//! K15's read-back. **A container placement that wrote no record would leave the
//! recycle case caught by nothing at all**, on the machine kind the recycle was
//! actually observed on, and the record is still written first.
//!
//! **Markers used in this file that belong to another document, each with the
//! document it belongs to.** `Rented-server runtime` K4, K5, K6, K7, K8, K10,
//! K10b, K11, K12, K15, K16, K16a, K16b, K17, K18, K19, K20, G13, G25.
//! `Server Setup` E7.
//!
//! **A marker this plan also defines is never listed there and is written out
//! with its document at the site**, because a header list cannot tell the two
//! apart and a bare one there would resolve to a real and wrong clause.
//! `Rented-server runtime` G4, `Rented-server runtime` G7 and
//! `Rented-server runtime` S5 are of that kind here.

use smoozy_runtime::machine_kind::MachineKind;
use smoozy_runtime::registration::{
    confirm, BootRegistrationRecord, RegistrationConfirmation, TheLingerReadBack,
    WhatTheRegistrationStepLeft,
};
use smoozy_runtime::registration_content::{
    the_container_hook_keeping, the_keepers_start_file, the_record_of, the_systemd_unit,
    ThatValueDoesNotGoIntoARegistration, THE_UNIT_NAME, THE_USER_UNIT_DIRECTORY_UNDER_A_HOME,
};
use smoozy_runtime::the_program::the_active_package_pointer_at;
use tokio::io::{AsyncRead, AsyncWrite};

use crate::placement::records::{
    publish_durably, read_a_records_bytes, write_the_boot_registration_record,
    WhetherTheDurabilityOrderingApplied,
};
use crate::subsystem::client::Subsystem;
use crate::subsystem::{StatusCode, SubsystemFailure};
use crate::transport::ssh::{CommandFailure, Connection};
use crate::transport::CommandString;

/// **K7's prefix, and it is the same value the machine's own half composes.**
///
/// The form is the observed box's — `/run/user/1001` there — completed by the
/// login's own **numeric effective user ID**, derived on the machine by §1.1's
/// fourth route. **The number never crosses from Smoozy.**
pub const THE_RUNTIME_DIRECTORY_PREFIX: &str = "XDG_RUNTIME_DIR=/run/user/\"$(id -u)\"";

/// **K6's grant: by name, and the name is the machine's own, derived at
/// execution.**
pub const THE_LINGERING_GRANT: CommandString = CommandString::constant(
    "XDG_RUNTIME_DIR=/run/user/\"$(id -u)\" loginctl enable-linger \"$(id -un)\"",
);

/// **K5's read-back, and it is what separates a registration from a report of
/// one.**
///
/// *"Enabling alone is not the registration and its failure is silent."* With
/// the unit still enabled and only lingering turned off, the program *"did not
/// come back"* and *"no log line anywhere says why"*. **Both properties are
/// asked for** because the flag is taken from the line that names it rather than
/// from a position.
pub const THE_LINGER_READ_BACK: CommandString = CommandString::constant(
    "XDG_RUNTIME_DIR=/run/user/\"$(id -u)\" loginctl show-user \"$(id -un)\" -p Linger -p State",
);

/// **K4's second act: the unit, enabled.**
///
/// **`Rented-server runtime` K12 forbids this on a container outright** —
/// `systemctl enable ssh` there *"reports success, writes the symlink, exits 0 —
/// and the unit will never run"* — so nothing on that machine kind reaches this
/// constant.
pub const THE_UNIT_ENABLE: CommandString = CommandString::constant(
    "XDG_RUNTIME_DIR=/run/user/\"$(id -u)\" systemctl --user enable smoozy-supervisor.service",
);

/// **`Rented-server runtime` K7's question, asked before the enable: is the
/// user's own service manager there to take it?**
///
/// **Why it is asked at all (K7, corrected 2026-09-21).** On a provider that
/// creates no login session, `/run/user/<uid>` does not exist until something
/// starts the user's service manager, and the enable fails with *"Failed to
/// connect to bus: No such file or directory"* — the second pass on Sesterce,
/// where the enable ran before the grant. **The grant is what starts it**:
/// `user@.service(5)`, systemd's own manual source at tag `v255`,
/// `https://raw.githubusercontent.com/systemd/systemd/v255/man/user@.service.xml`,
/// read 2026-09-21 — *"user@UID.service is accompanied by the system unit
/// user-runtime-dir@UID.service, which creates the user's runtime directory
/// /run/user/UID"* — and `method_set_user_linger` in `src/login/logind-dbus.c`
/// at `v249` and `v255`, whose enabling branch ends *"if
/// (manager_add_user_by_uid(m, uid, &u) >= 0) user_start(u);"*, `user_start`
/// being *"Start user@UID.service"* in `src/login/logind-user.c`. **That start
/// is queued rather than awaited**, so the enable is not sent until this answers.
///
/// **What is asked is the manager itself and nothing about a path.**
/// `systemctl(1)`, the same source at `v255`,
/// `https://raw.githubusercontent.com/systemd/systemd/v255/man/systemctl.xml`,
/// read 2026-09-21, of `show-environment`: *"Dump the systemd manager
/// environment block."* **It changes nothing, and only its ending is read**:
/// what it prints is the manager's environment, which is not Smoozy's to keep,
/// and `Rented-server runtime` §1.3 forbids mining a tool's output for a cause.
pub const THE_USER_MANAGER_ASKED: CommandString = CommandString::constant(
    "XDG_RUNTIME_DIR=/run/user/\"$(id -u)\" systemctl --user show-environment",
);

/// **`Rented-server runtime` G25's start, asked of the restarter that
/// document's §3.2 names for a systemd machine.**
///
/// **On a container this constant is not what the step runs.** `Rented-server
/// runtime` G25 has the commit start that document's S5 keeper itself, once,
/// and S5 makes a second start of a keeper already running harmless: the
/// keeper holds an exclusive lock on its own file in the package area and one
/// that cannot take it ends at once. ~~**That start is not built** —
/// [`start_the_supervisor_the_pointer_names`] says why.~~ **Built 2026-09-21: it
/// is [`THE_KEEPERS_START`].** *(Corrected 2026-09-19:
/// this said the container step is nothing at all and that anything started
/// from here would be a second restarter, which is what the plan said before
/// 2026-09-12 and the reverse of what it says since.)*
///
/// **It is `restart` and not `start`, and that is `Rented-server runtime` G25's
/// ending performed through the restarter itself.** Changed 2026-09-22, out of
/// step 10c's third pass: on Hyperstack and on Vast a second placement moved the
/// pointer and the supervisor already running went on running, because a start
/// of a unit that is already active was never read to replace its process and
/// this side has no route to a signal. `systemctl(1)`, systemd's own manual
/// source at `v255`,
/// `https://raw.githubusercontent.com/systemd/systemd/v255/man/systemctl.xml`,
/// read 2026-09-22, of `restart`: *"Stop and then start one or more units
/// specified on the command line. If the units are not running yet, they will
/// be started."* So on a first placement it is the start it always was, and on
/// a second it ends the running supervisor and starts the one the pointer now
/// names — G25's two steps in G25's order. **What the stop also ends is the
/// unit's whole control group** (`systemd.kill(5)`, quoted at
/// `smoozy_runtime::registration_content`), which `Rented-server runtime` G21 has already made safe:
/// the commit is not entered while work depends on the running package.
pub const THE_SUPERVISOR_START: CommandString = CommandString::constant(
    "XDG_RUNTIME_DIR=/run/user/\"$(id -u)\" systemctl --user restart smoozy-supervisor.service",
);

/// **Where a user unit sits, as the subsystem names it.**
///
/// **Relative, and that is the same Build decision [`crate::carriage`] already
/// took**: the file-transfer subsystem resolves a relative path against the
/// login's own starting directory, and the login shell expands `~` to the same
/// place — so the unit is named without anybody asking the machine where the
/// login's home is, which would be a machine-origin value with nowhere to ride.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WhereAUserUnitSitsOnTheSubsystem(pub String);

impl WhereAUserUnitSitsOnTheSubsystem {
    /// The unit's own path, composed from the two names that own them.
    pub fn the_units_own_path() -> WhereAUserUnitSitsOnTheSubsystem {
        WhereAUserUnitSitsOnTheSubsystem(format!(
            "{THE_USER_UNIT_DIRECTORY_UNDER_A_HOME}/{THE_UNIT_NAME}"
        ))
    }

    /// Each directory above it, outermost first — what has to exist before the
    /// unit can be written.
    pub fn the_directories_above(&self) -> Vec<String> {
        let mut out = Vec::new();
        let mut so_far = String::new();
        let mut segments: Vec<&str> = self.0.split('/').collect();
        segments.pop();
        for segment in segments {
            if !so_far.is_empty() {
                so_far.push('/');
            }
            so_far.push_str(segment);
            out.push(so_far.clone());
        }
        out
    }
}

/// **The registration this placement would make, composed — content and record
/// together.**
///
/// **`Rented-server runtime` G7's requirement is what makes one composition
/// enough**: the registration is written to start whatever the active-package
/// pointer names rather than to name a package version, so its text does not
/// change when the version does.
///
/// ~~**On a container the content is the hook's and is not composed here.** That
/// composition wants the machine's own name for the supervisor's login, which is
/// a machine-origin value; the elevated step that installs the hook is
/// `Server Setup`'s and the login is its to supply.~~ **STRUCK 2026-09-21: nothing
/// supplied it, and the one caller handed in no content and no path.** **On a
/// container the content is composed by [`the_container_hooks_content`]** —
/// the machine's own name for its login, and whatever the provider's file
/// already holds, read first — and handed in here, which is why the container
/// branch takes the content in. **What this function composes on that kind is
/// the record.**
pub fn the_registration_this_placement_would_make(
    kind: MachineKind,
    package_area: &str,
    the_content_a_container_hook_would_hold: Option<&[u8]>,
    the_providers_hook_path: &str,
) -> Result<BootRegistrationRecord, ThatValueDoesNotGoIntoARegistration> {
    let pointer = the_active_package_pointer_at(package_area);
    match kind {
        MachineKind::Systemd => {
            let unit = the_systemd_unit(&pointer)?;
            Ok(the_record_of(
                MachineKind::Systemd,
                &WhereAUserUnitSitsOnTheSubsystem::the_units_own_path().0,
                &unit,
            ))
        }
        MachineKind::Container => Ok(the_record_of(
            MachineKind::Container,
            the_providers_hook_path,
            the_content_a_container_hook_would_hold.unwrap_or_default(),
        )),
    }
}

/// **The registration step, and it is one act on both machine kinds:
/// `Rented-server runtime` K16's record, and then the registration's own
/// content.**
///
/// **The record goes first, and that is `Rented-server runtime` K16b.** The
/// record is written durably before the registration it describes, which is
/// `State, identity and recovery` D1's *intent before acting*. **The two orders
/// are not equally safe.** Interrupted after the record and before the
/// registration, the record holds content the machine does not — which is
/// exactly what K15's read-back compares and it fails the placement before the
/// commit. Interrupted the other way round, Smoozy's own boot bytes are live on
/// the machine with no record at all, which package inspection reads as the
/// honest never-held answer on a machine that is about to start Smoozy.
///
/// **Nothing of the previous registration is lost by writing over the record**,
/// because `Rented-server runtime` G4 has already carried the previous
/// boot-registration content forward by value.
///
/// **The second half of the answer is S20's**, and it is carried out rather than
/// swallowed: whether the durability ordering applied to the writes this step
/// made.
pub async fn place_the_registration<S>(
    subsystem: &mut Subsystem<S>,
    package_area: &str,
    record: &BootRegistrationRecord,
) -> Result<(WhatTheRegistrationStepLeft, WhetherTheDurabilityOrderingApplied), SubsystemFailure>
where
    S: AsyncRead + AsyncWrite + Unpin + Send,
{
    // **K16b: the record, durably, first.**
    let mut ordering = write_the_boot_registration_record(subsystem, package_area, record).await?;

    match record.kind {
        MachineKind::Systemd => {
            let at = WhereAUserUnitSitsOnTheSubsystem(record.at_path.clone());
            for directory in at.the_directories_above() {
                match subsystem.make_directory(directory.as_bytes()).await {
                    Ok(()) => {}
                    // A directory that is already there is the ordinary case.
                    Err(SubsystemFailure::TheServerRefused(StatusCode::Failure)) => {}
                    Err(why) => return Err(why),
                }
            }
            ordering = publish_durably(subsystem, &record.at_path, &record.content).await?;
            Ok((
                WhatTheRegistrationStepLeft::TheRecordAndTheRegistrationItself,
                ordering,
            ))
        }
        // **`Rented-server runtime` K11: installing the hook requires root, so
        // the registration step is an elevated step and `Server Setup` owns the
        // elevation.** K19 puts a second obligation on the same step — the hook
        // must be left readable by the login that performs K15's read-back.
        // **This function's own part ends at the record**, and the walk runs
        // [`install_the_hook`] next, with `Server Setup`'s commands.
        //
        // ~~**Where that elevated step lives, and what removes this hand-over.**
        // `App/Server Setup/src/elevated.rs` enumerates every elevated step and
        // **none of them installs a hook**, so the hand-over here points at
        // nothing that exists yet. … **What removes it is that elevated step
        // being written**, and until it is, a container placement's own record
        // is written and the hook on the machine is whatever the provider put
        // there.~~ **STRUCK 2026-09-21, step 10c: the step is written** — four
        // constants at `Server Setup`'s `elevated.rs`, reaching the walk as
        // [`TheRootStepOnAContainer`].
        MachineKind::Container => Ok((
            WhatTheRegistrationStepLeft::TheRecordAndTheHookLeftToTheRootStep,
            ordering,
        )),
    }
}

// ════════════════════════════════════════════════════════════════════════════
// `Rented-server runtime` K10, K11 and K19 — the container's root step
// ════════════════════════════════════════════════════════════════════════════

/// **The machine's own name for the login this connection is**, which is the
/// supervisor's (`Server Setup` M4) and the one the hook drops privilege to.
///
/// **It crosses into the hook as file content and never into a command
/// string**: the command is a constant and the name is its output. **Documented**
/// — GNU coreutils manual, *id invocation*,
/// `https://www.gnu.org/software/coreutils/manual/html_node/id-invocation.html`,
/// read 2026-09-21: *"id prints information about the given user, or the process
/// running it if no user is specified"*; of `-u`, *"Print only the user ID"*; of
/// `-n`, *"Print the user or group name instead of the ID number. Requires -u,
/// -g, or -G."* **It is the effective user**: the same page's `-r` is *"Print the
/// real, instead of effective, user or group ID"*, and it is not given. The same
/// derivation K6's grant already makes on the machine, `$(id -un)`.
pub const THE_LOGINS_OWN_NAME: CommandString = CommandString::constant("id -un");

/// One command of the root step, in the form it goes out without elevation and,
/// where elevation is to be had without a password, in that form too.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TheCommandAndItsElevatedForm {
    /// Tried first. **A login that is the container's own root needs nothing
    /// more**, and where the elevating command is not on the image at all this
    /// is the only form that could work.
    pub plain: CommandString,
    /// **`None` where elevation would need a password Smoozy does not hold**, and
    /// then a refused plain form is the step's ending.
    pub elevated: Option<CommandString>,
}

/// **`Rented-server runtime` K11's and K19's root step, as the placement is handed
/// it.** Every command is `Server Setup`'s, declared at that document's E7 —
/// *"the destination path and the mode change are constant, per K10's fixed
/// provider path; the hook's content crosses the subsystem to a
/// Smoozy-synthesised staging token, and the elevated command names the token and
/// the fixed destination"*.
///
/// **The path and the token are held beside the commands rather than taken from
/// a constant here** so that the one test that can run the whole step at a desk
/// can point it into a scratch directory; the application's own caller hands in
/// [`smoozy_runtime::registration_content::THE_PROVIDERS_HOOK_PATH`] and the
/// constants that name it, and a test at `Server Setup` holds the commands to
/// that path.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TheRootStepOnAContainer {
    /// K10's path.
    pub the_hooks_path: String,
    /// **Where the content is staged**, relative to the login's own starting
    /// directory, which the file-transfer subsystem and the login shell's `~`
    /// both name.
    pub the_staging_token_on_the_subsystem: String,
    /// **K11: the staged content copied to the provider's path.**
    pub the_copy: TheCommandAndItsElevatedForm,
    /// **K19, first half: the hook left readable.** Only where the copy needed
    /// elevation — a login that could write the hook can read it back.
    pub the_hook_made_readable: Option<CommandString>,
    /// **K19, second half, and it is the route K19 itself names first**:
    /// *"Making the containing directory traversable is enough"*. Only where
    /// the copy needed elevation, for the same reason.
    pub the_directory_made_traversable: Option<CommandString>,
    /// **The provider's hook read with elevation**, where the plain read over the
    /// file-transfer subsystem was refused — which is K19's own observation of a
    /// `0700` directory, before the root step has run.
    pub the_hook_read: Option<CommandString>,
}

/// **The hook this placement writes, composed: whatever the provider's own file
/// already holds, kept, with Smoozy's part in it.**
///
/// **Read first, from the machine, and never recomposed from a record**: the
/// provider may have written the file again since Smoozy last did — the recycle
/// case K15 exists for is exactly that. **K16 is kept**: the record is composed
/// from the result by value, and nothing about it is re-derived later.
///
/// **The two readings it takes are both of the machine and neither enters a
/// command string**: the login's name ([`THE_LOGINS_OWN_NAME`]) and the hook's
/// bytes, which cross as protocol fields or as a constant command's output.
pub async fn the_container_hooks_content<S>(
    subsystem: &mut Subsystem<S>,
    connection: &mut Connection,
    package_area: &str,
    step: &TheRootStepOnAContainer,
) -> Result<Vec<u8>, String>
where
    S: AsyncRead + AsyncWrite + Unpin + Send,
{
    let said = connection
        .run_one_command(THE_LOGINS_OWN_NAME)
        .await
        .map_err(|why| format!("the machine could not be asked its login's name: {}", why.in_words()))?;
    if !said.ended_cleanly() {
        return Err(format!(
            "the machine would not name its own login: {}",
            String::from_utf8_lossy(&said.complaint).trim()
        ));
    }
    let login = String::from_utf8_lossy(&said.output).trim().to_string();

    let held = match read_a_records_bytes(subsystem, &step.the_hooks_path).await {
        Ok(Some(bytes)) => bytes,
        // **Nothing there is the provider shipping no hook**, and the hook is
        // then Smoozy's alone.
        Ok(None) => Vec::new(),
        // **Refused is K19's `0700` directory**, before the root step has run.
        Err(refused) => match step.the_hook_read {
            Some(elevated) => {
                let said = connection.run_one_command(elevated).await.map_err(|why| {
                    format!(
                        "{}: the machine could not be asked to read it: {}",
                        step.the_hooks_path,
                        why.in_words()
                    )
                })?;
                if !said.ended_cleanly() {
                    return Err(format!(
                        "{}: it could not be read, even with elevation: {}",
                        step.the_hooks_path,
                        String::from_utf8_lossy(&said.complaint).trim()
                    ));
                }
                said.output
            }
            None => {
                return Err(format!(
                    "{}: it could not be read ({}), and reading it needs elevation \
                     that would ask for a password Smoozy does not hold",
                    step.the_hooks_path,
                    refused.in_words()
                ))
            }
        },
    };

    the_container_hook_keeping(&held, &the_active_package_pointer_at(package_area), &login)
        .map_err(|why| why.as_text())
}

/// **K11 and K19, performed: the content staged, copied to the provider's path,
/// and left readable to the login that reads it back.**
///
/// **It runs after [`place_the_registration`] has written the record**, which is
/// `Rented-server runtime` K16b's order, and **before K15's read-back**, which is
/// what then confirms it — the copy's exit is never that evidence.
///
/// **The plain form is tried first, and only a refused plain form is elevated.**
/// A login that could write the hook itself is one that can read it back, so
/// K19's two changes are made only where the copy needed elevation, and **a
/// machine whose login is its own root is left with its home directory exactly
/// as it was.**
///
/// **The staging token is taken away afterwards whatever happened**, and a token
/// that could not be taken away is replaced by the next placement's write rather
/// than obstructing it.
pub async fn install_the_hook<S>(
    subsystem: &mut Subsystem<S>,
    connection: &mut Connection,
    record: &BootRegistrationRecord,
    step: &TheRootStepOnAContainer,
) -> Result<(), String>
where
    S: AsyncRead + AsyncWrite + Unpin + Send,
{
    if record.at_path != step.the_hooks_path {
        return Err(format!(
            "the record names {} and the root step would install {}",
            record.at_path, step.the_hooks_path
        ));
    }
    if let Some((directory, _)) = step.the_staging_token_on_the_subsystem.rsplit_once('/') {
        match subsystem.make_directory(directory.as_bytes()).await {
            Ok(()) => {}
            Err(SubsystemFailure::TheServerRefused(StatusCode::Failure)) => {}
            Err(why) => {
                return Err(format!("{directory}: {}", why.in_words()))
            }
        }
    }
    publish_durably(subsystem, &step.the_staging_token_on_the_subsystem, &record.content)
        .await
        .map_err(|why| format!("{}: {}", step.the_staging_token_on_the_subsystem, why.in_words()))?;

    let installed = the_copy_and_what_k19_asks(connection, step).await;
    let _ = subsystem
        .remove(step.the_staging_token_on_the_subsystem.as_bytes())
        .await;
    installed
}

async fn the_copy_and_what_k19_asks(
    connection: &mut Connection,
    step: &TheRootStepOnAContainer,
) -> Result<(), String> {
    let plainly = what_a_command_said(connection, step.the_copy.plain).await?;
    if plainly.is_ok() {
        return Ok(());
    }
    let refused = plainly.unwrap_err();
    let Some(elevated) = step.the_copy.elevated else {
        return Err(format!(
            "{}: this login may not write it ({refused}), and doing it as root would need a \
             password Smoozy does not hold",
            step.the_hooks_path
        ));
    };
    what_a_command_said(connection, elevated)
        .await?
        .map_err(|said| format!("{}: {said}", step.the_hooks_path))?;
    for (then, what) in [
        (step.the_hook_made_readable, "left readable"),
        (step.the_directory_made_traversable, "left reachable"),
    ] {
        let Some(then) = then else {
            return Err(format!(
                "{}: it was installed and could not be {what} without a password",
                step.the_hooks_path
            ));
        };
        what_a_command_said(connection, then)
            .await?
            .map_err(|said| format!("{}: it could not be {what}: {said}", step.the_hooks_path))?;
    }
    Ok(())
}

/// One command, and whether it ended cleanly — **the outer error is the machine
/// not being reached at all**, the inner one its own words.
async fn what_a_command_said(
    connection: &mut Connection,
    command: CommandString,
) -> Result<Result<(), String>, String> {
    let said = connection
        .run_one_command(command)
        .await
        .map_err(|why| format!("the machine could not be asked: {}", why.in_words()))?;
    if said.ended_cleanly() {
        Ok(Ok(()))
    } else {
        Ok(Err(String::from_utf8_lossy(&said.complaint).trim().to_string()))
    }
}

/// **K15: a boot registration is confirmed by reading its own content back and
/// comparing it against the record Smoozy wrote.**
///
/// **What it catches and what it does not is K20's, and neither is overstated
/// here.** It refuses loudly when the registration's own bytes change — the
/// recycle case, where an 82-byte provider default replaced the hook — and it
/// confirms happily when a supplementary configuration file changes what the
/// registration does while its own bytes are untouched.
///
/// **The read is of content and never of presence**, and where it cannot be
/// taken at all that is a failure before the commit rather than a confirmation.
/// On a container that is K19's case before the root step has made the path
/// reachable, and the machine's own words come back with it.
pub async fn confirm_the_registration<S>(
    subsystem: &mut Subsystem<S>,
    record: &BootRegistrationRecord,
) -> RegistrationConfirmation
where
    S: AsyncRead + AsyncWrite + Unpin + Send,
{
    // **A record naming no path is not a path with nothing at it**, and the
    // machine's words are never shown with an empty path in front of them —
    // which the second pass on a real rental read as *": :"*, on 2026-09-21.
    // Since the root step is built no placement writes such a record; this is
    // what a reader meets if one ever does.
    if record.at_path.is_empty() {
        return confirm(
            record,
            Err("the boot-registration record names no path to read back".to_string()),
        );
    }
    match read_a_records_bytes(subsystem, &record.at_path).await {
        Ok(Some(bytes)) => confirm(record, Ok(&bytes)),
        Ok(None) => confirm(
            record,
            Err(format!("{}: there is nothing at that path", record.at_path)),
        ),
        Err(why) => confirm(record, Err(format!("{}: {}", record.at_path, why.in_words()))),
    }
}

/// **K4's second act**, over the command channel.
pub async fn enable_the_unit(connection: &mut Connection) -> Result<(), String> {
    what_a_user_level_command_said(connection, THE_UNIT_ENABLE, "enable the unit").await
}

/// **K6's grant**, by name, and the name is the machine's own.
pub async fn grant_lingering(connection: &mut Connection) -> Result<(), String> {
    what_a_user_level_command_said(connection, THE_LINGERING_GRANT, "grant lingering").await
}

/// **`Rented-server runtime` K7's question, asked once**: whether the login's
/// own service manager answers. **Nothing here waits**; the caller asks again
/// inside its own deadline (`Rented-server runtime` K17).
pub async fn the_user_manager_answers(connection: &mut Connection) -> Result<(), String> {
    what_a_user_level_command_said(
        connection,
        THE_USER_MANAGER_ASKED,
        "let this login's own service manager be reached",
    )
    .await
}

/// **`Rented-server runtime` G25's start**, asked of the restarter that
/// document's §3.2 names for this machine kind.
///
/// ~~**On a container `Rented-server runtime` G25 says *"the commit invokes the
/// keeper itself as that start step, once"*, and this starts nothing there.**
/// That is owed rather than decided, and the reason is one reading and nothing
/// in the plan. … **So a container's first placement still reaches G13's *did
/// not answer* branch at this step and rolls back**, behind the earlier failure
/// at K15's hook read-back that the unbuilt elevated hook install causes.~~
/// **STRUCK 2026-09-21, step 10c: the reading is taken and the start is built.**
///
/// **On a container this is `Rented-server runtime` G25's *"the commit invokes
/// the keeper itself as that start step, once"*.** The keeper is
/// [`smoozy_runtime::registration_content::the_keepers_loop`] — the one the hook
/// runs — written as a file at [`THE_KEEPERS_START_FILE_ON_THE_SUBSYSTEM`] and
/// started by [`THE_KEEPERS_START`]:
///
/// - **once, over the command channel, as the supervisor's own login** — the
///   channel's login is that login, so nothing drops privilege and no `su` is
///   in it;
/// - **detached the way `Rented-server runtime` S5a observed a process outliving
///   its connection on this machine kind**, *"`setsid nohup … </dev/null
///   >/dev/null 2>&1 &`"*;
/// - **harmlessly where a keeper already runs**: `Rented-server runtime` S5's
///   lock, taken by the loop itself, ends the second at once.
///
/// **What the command's exit says is only that it was sent**: a backgrounded
/// list ends at zero whatever it started. **Whether a supervisor came up is
/// `Rented-server runtime` G12's exchange and nothing else**, which is exactly where G25 puts the question.
///
/// Where the restarter produces no answering supervisor inside the caller's own
/// deadline, that is `Rented-server runtime` G13's *did not answer* branch
/// and nothing here softens it; K17 forbids budgeting a fixed wait in any case,
/// and no wait is budgeted here.
pub async fn start_the_supervisor_the_pointer_names<S>(
    subsystem: &mut Subsystem<S>,
    connection: &mut Connection,
    kind: MachineKind,
    package_area: &str,
) -> Result<(), String>
where
    S: AsyncRead + AsyncWrite + Unpin + Send,
{
    match kind {
        MachineKind::Container => {
            let keeper = the_keepers_start_file(&the_active_package_pointer_at(package_area))
                .map_err(|why| why.as_text())?;
            match subsystem
                .make_directory(crate::carriage::THE_DIRECTORY_ON_THE_SUBSYSTEM.as_bytes())
                .await
            {
                Ok(()) => {}
                Err(SubsystemFailure::TheServerRefused(StatusCode::Failure)) => {}
                Err(why) => {
                    return Err(format!(
                        "{}: {}",
                        crate::carriage::THE_DIRECTORY_ON_THE_SUBSYSTEM,
                        why.in_words()
                    ))
                }
            }
            publish_durably(subsystem, THE_KEEPERS_START_FILE_ON_THE_SUBSYSTEM, &keeper)
                .await
                .map_err(|why| format!("{THE_KEEPERS_START_FILE_ON_THE_SUBSYSTEM}: {}", why.in_words()))?;
            what_a_user_level_command_said(connection, THE_KEEPERS_START, "start the keeper").await
        }
        MachineKind::Systemd => {
            what_a_user_level_command_said(
                connection,
                THE_SUPERVISOR_START,
                "start the supervisor",
            )
            .await
        }
    }
}

/// **Where G25's start step writes the keeper it starts**, relative to the
/// login's own starting directory — the Smoozy-named directory the carriage
/// tokens sit in.
///
/// **A file rather than a planted link, and on purpose.** The command below
/// returns as soon as it has backgrounded the keeper, and the shell it starts
/// opens its file after that; a link taken away at the command's return could
/// be gone before it was opened. **The file is left where it is** and the next
/// start writes over it; a keeper already reading it is reading the file it
/// opened, which a publication by rename does not touch. **It names the package
/// area's real paths and no link.**
pub const THE_KEEPERS_START_FILE_ON_THE_SUBSYSTEM: &str = ".smoozy-carriage/keeper";

/// **G25's start step on a container**, naming the file above by the other
/// route to the same place (`~`, which the login shell expands to the directory
/// the subsystem resolves a relative path against — [`crate::carriage`]'s own
/// Build decision).
///
/// **Every outside command in it, read from its own publication on 2026-09-21:**
///
/// - `setsid(1)`, util-linux's own manual source at tag **v2.41**,
///   `https://raw.githubusercontent.com/util-linux/util-linux/v2.41/sys-utils/setsid.1.adoc`:
///   *"setsid runs a program in a new session. The command calls fork(2) if
///   already a process group leader. Otherwise, it executes a program in the
///   current process."*
/// - `nohup`, GNU coreutils manual, *nohup invocation*,
///   `https://www.gnu.org/software/coreutils/manual/html_node/nohup-invocation.html`:
///   *"nohup runs the given command with hangup signals ignored, so that the
///   command can continue running in the background after you log out"*, and
///   *"nohup does not automatically put the command it runs in the background;
///   you must do that explicitly, by ending the command line with an '&'."*
/// - `sh` with a command file, the POSIX `sh` utility,
///   `https://pubs.opengroup.org/onlinepubs/9799919799/utilities/sh.html`:
///   *"command_file — The pathname of a file containing commands. If the pathname
///   contains one or more <slash> characters, the implementation attempts to read
///   that file; the file need not be executable."*
///
/// **The shape is `Rented-server runtime` S5a's observation**, not a guess:
/// *"a process started `setsid nohup … </dev/null >/dev/null 2>&1 &` took PID
/// 855 as its own session leader … on a new connection it was still running"*.
pub const THE_KEEPERS_START: CommandString = CommandString::constant(
    "setsid nohup sh ~/.smoozy-carriage/keeper </dev/null >/dev/null 2>&1 &",
);

/// **K5's read-back**, by the name the machine derived for itself.
///
/// **A property the tool did not answer is an absence and never a no.** The
/// reading itself is
/// [`smoozy_runtime::acting::registration::the_flag_in`]'s rule, which lives on
/// the machine's own side of this crate wall; the same rule is composed here
/// from the two property names because that module is gated to a unix and the
/// desktop application runs on three systems. **The two are held together by
/// [`the_two_properties_read_back`]'s own test**, which reads the other side's
/// source rather than trusting that the two lists still agree.
pub async fn read_the_linger_flag_back(
    connection: &mut Connection,
) -> Result<TheLingerReadBack, String> {
    let said = connection
        .run_one_command(THE_LINGER_READ_BACK)
        .await
        .map_err(|why| format!("the login manager could not be asked: {}", why.in_words()))?;
    if !said.ended_cleanly() {
        return Err(format!(
            "the login manager refused the read-back: {}",
            String::from_utf8_lossy(&said.complaint).trim()
        ));
    }
    let answered = String::from_utf8_lossy(&said.output).into_owned();
    the_flag_in(&answered).ok_or_else(|| {
        format!(
            "the login manager answered without naming both {} and {}: {}",
            the_two_properties_read_back()[0],
            the_two_properties_read_back()[1],
            answered.trim()
        )
    })
}

/// **The two property names K5 reads back, and no others.**
pub fn the_two_properties_read_back() -> [&'static str; 2] {
    ["Linger", "State"]
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
/// **An answer naming one of the two is half a read-back and is not one**, and a
/// property the tool did not answer is an absence rather than a no: reading a
/// missing answer as *not granted* would be K5's own outcome by luck, and
/// reading it as granted would be the silent failure K5 exists for.
pub fn the_flag_in(what_the_tool_answered: &str) -> Option<TheLingerReadBack> {
    let granted = the_property_named(what_the_tool_answered, the_two_properties_read_back()[0])?;
    let state = the_property_named(what_the_tool_answered, the_two_properties_read_back()[1])?;
    Some(TheLingerReadBack {
        lingering_is_granted: granted.eq_ignore_ascii_case("yes"),
        the_state: state,
    })
}

/// One user-level command, with what the machine said carried whole.
///
/// **§1.3: no tool's own output is mined for a cause**, so nothing here parses
/// the complaint; it is carried as the machine's own words. **And no result is
/// classified on the exit status alone where 255 is one of the
/// possibilities** — what is asked is only whether the command ended at zero,
/// which is the one reading §1.1 says an `exec` returns and the one K5's
/// read-back rests on separately.
async fn what_a_user_level_command_said(
    connection: &mut Connection,
    command: CommandString,
    what_it_was_for: &str,
) -> Result<(), String> {
    let said = connection.run_one_command(command).await.map_err(
        |why: CommandFailure| format!("the machine could not be asked to {what_it_was_for}: {}", why.in_words()),
    )?;
    if said.ended_cleanly() {
        Ok(())
    } else {
        Err(format!(
            "the machine would not {what_it_was_for}: {}",
            String::from_utf8_lossy(&said.complaint).trim()
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// **The unit's name lives in one place and the commands name that one.**
    ///
    /// A command string may hold only a fixed literal, so the name is written
    /// out in each of the two commands that ask for it. **This is what holds
    /// them to the one name that owns it**, which is
    /// [`smoozy_runtime::registration_content::THE_UNIT_NAME`] — the same
    /// constant the unit is written under.
    #[test]
    fn every_command_that_names_the_unit_names_the_one_the_registration_writes() {
        for command in [THE_UNIT_ENABLE, THE_SUPERVISOR_START] {
            assert!(
                command.as_str().ends_with(THE_UNIT_NAME),
                "{} does not name the unit the registration writes",
                command.as_str()
            );
        }
    }

    /// **`Rented-server runtime` G25 on a systemd machine: the start step ends a
    /// running supervisor and starts the one the pointer names**, which is what
    /// `systemctl(1)`'s *restart* is documented to do and its *start* is not.
    /// Added 2026-09-22, out of step 10c's third pass on Hyperstack and Vast.
    #[test]
    fn the_start_step_on_a_systemd_machine_is_a_restart() {
        let text = THE_SUPERVISOR_START.as_str();
        assert!(text.contains(" systemctl --user restart "), "{text}");
        assert!(!text.contains(" --user start "), "{text}");
    }

    /// **K7's prefix is the same on every user-level command**, so all three are
    /// handed the same thing — which is the machine's own half's rule and is
    /// kept here by the commands sharing one prefix rather than by care at each
    /// site.
    #[test]
    fn every_user_level_command_carries_k7s_export() {
        for command in [
            THE_LINGERING_GRANT,
            THE_LINGER_READ_BACK,
            THE_USER_MANAGER_ASKED,
            THE_UNIT_ENABLE,
            THE_SUPERVISOR_START,
        ] {
            assert!(
                command.as_str().starts_with(THE_RUNTIME_DIRECTORY_PREFIX),
                "{} does not carry K7's export",
                command.as_str()
            );
        }
    }

    /// **K6: the login's name is never carried by Smoozy.** Every command that
    /// needs it derives it on the machine, and the bare *self* form — which
    /// failed on the observed box — appears nowhere.
    #[test]
    fn no_command_here_carries_a_login_name_and_none_uses_the_bare_self_form() {
        for command in [
            THE_LINGERING_GRANT,
            THE_LINGER_READ_BACK,
            THE_USER_MANAGER_ASKED,
            THE_UNIT_ENABLE,
            THE_SUPERVISOR_START,
        ] {
            let text = command.as_str();
            assert!(
                !text.contains("--user-name") && !text.split_whitespace().any(|word| word == "self"),
                "{text} names a login rather than deriving one"
            );
        }
        assert!(THE_LINGERING_GRANT.as_str().contains("$(id -un)"));
        assert!(THE_LINGER_READ_BACK.as_str().contains("$(id -un)"));
    }

    /// **The two property names are the same two the machine's own half reads
    /// back**, and this reads that file's source rather than trusting that the
    /// two lists still agree.
    ///
    /// **The copy exists because that module is gated to a unix** and the
    /// desktop application runs on three systems, so a desktop that reached for
    /// it would not build on Windows at all.
    #[test]
    fn the_two_properties_are_the_two_the_machines_own_half_reads() {
        let theirs = std::fs::read_to_string(
            std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("../Rented-server runtime/src/acting/registration.rs"),
        )
        .expect("the machine's own half is readable");
        let line = theirs
            .lines()
            .find(|line| line.contains("pub const THE_TWO_PROPERTIES_READ_BACK"))
            .expect("that file declares the two properties");
        for property in the_two_properties_read_back() {
            assert!(
                line.contains(&format!("\"{property}\"")),
                "the machine's own half does not read {property} back"
            );
        }
    }

    #[test]
    fn a_read_back_naming_one_property_is_half_a_read_back_and_is_not_one() {
        assert!(the_flag_in("Linger=yes").is_none());
        assert!(the_flag_in("State=lingering").is_none());
        let both = the_flag_in("Linger=yes\nState=lingering").expect("both are named");
        assert!(both.lingering_is_granted);
        assert_eq!(both.the_state, "lingering");
        // **The flag is taken from the line that names it and never from a
        // position**, so an answer that orders the two the other way round is
        // read correctly rather than by luck.
        let ordered_the_other_way =
            the_flag_in("State=active\nLinger=no").expect("both are named");
        assert!(!ordered_the_other_way.lingering_is_granted);
        assert_eq!(ordered_the_other_way.the_state, "active");
    }

    #[test]
    fn the_units_path_is_composed_from_the_two_names_that_own_them() {
        let at = WhereAUserUnitSitsOnTheSubsystem::the_units_own_path();
        assert_eq!(
            at.0,
            format!("{THE_USER_UNIT_DIRECTORY_UNDER_A_HOME}/{THE_UNIT_NAME}")
        );
        assert_eq!(
            at.the_directories_above(),
            vec![
                ".config".to_string(),
                ".config/systemd".to_string(),
                ".config/systemd/user".to_string()
            ]
        );
    }

    /// **A container's registration record is the hook's content by value and
    /// the provider's own path**, and the record is written on that kind all the
    /// same — `Rented-server runtime` K16a.
    #[test]
    fn a_containers_record_carries_the_hooks_content_and_the_providers_path() {
        let record = the_registration_this_placement_would_make(
            MachineKind::Container,
            "/opt/smoozy/package",
            Some(b"#!/bin/sh\n"),
            "/root/onstart.sh",
        )
        .expect("the record composes");
        assert_eq!(record.kind, MachineKind::Container);
        assert_eq!(record.at_path, "/root/onstart.sh");
        assert_eq!(record.content, b"#!/bin/sh\n".to_vec());
    }

    /// **`Rented-server runtime` G7: the unit names the pointer and no
    /// version.**
    #[test]
    fn a_systemd_registration_names_the_pointer_and_never_a_version() {
        let record = the_registration_this_placement_would_make(
            MachineKind::Systemd,
            "/opt/smoozy/package",
            None,
            "/root/onstart.sh",
        )
        .expect("the record composes");
        let text = String::from_utf8(record.content).expect("the unit is text");
        assert!(text.contains("/opt/smoozy/package/smoozy-server"));
        assert!(!text.contains("0.1.0"));
    }
}
