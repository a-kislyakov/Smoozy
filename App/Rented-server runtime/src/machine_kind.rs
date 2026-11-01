//! §1.1 — the machine kind, detected before any registration is chosen.
//!
//! **The boot return is two arrangements and a detection step, never one**, and
//! this is the single easiest thing in the plan to get silently wrong. A
//! virtualization type is not a machine kind: the tool returned `docker` on a
//! container and `kvm` on a virtual machine, and on a machine that is neither it
//! names none of them.
//!
//! **The near-miss the table is closed for.** A container running inside a
//! VMware virtual machine: bare `systemd-detect-virt` said `docker`, the
//! container marker file was present, the first process was a shell — and
//! `systemd-detect-virt -v` said `vmware`. **This table places it as a container
//! and is right.** A table that expected the two readings to agree, or that
//! treated the virtual-machine form as authoritative, would choose a
//! service-manager registration on a box that has no service manager.
//!
//! **Markers used in this file that belong to another document, each with the
//! document it belongs to.** `State, identity and recovery` §2.3.

use smoozy_state::identities::Identity;

/// The two arrangements, and there is no third.
///
/// **K3: a machine the table does not place is a placement that does not
/// happen.** So this enum carries no *unknown* member — an unplaced machine is
/// [`Placement::NotOneThisPackageCanRegisterOn`] rather than a third kind that
/// something downstream would have to guess an arrangement for.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum MachineKind {
    /// §1.2. **The bare-metal rented GPU server as well as the virtual
    /// machine**, which is the machine the product's own first sentence names,
    /// so a reading that refused it would refuse the ordinary case.
    Systemd,
    /// §1.3. The provider's own boot hook and nothing else.
    Container,
}

/// **K1's two readings, and the table selects on these and on nothing else.**
///
/// The detection step the evidence names: `systemd-detect-virt` and the presence
/// of the container marker file, in that order. **Neither reading reads which
/// start-up mechanism the provider offers**, which is why K3 cannot fire on the
/// stored-field provider of [`crate::arriving_later::THE_STORED_FIELD_HOLE`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TheTwoReadings {
    /// Whether the container marker file is present.
    pub the_container_marker_file_is_present: bool,
    /// **The container technology `systemd-detect-virt` named, where it named
    /// one.** The tool is asked with its own container option, so a name here is
    /// a container technology by the tool's own answer and **nothing here
    /// matches it against a list.** `None` covers both the tool saying no and
    /// the tool not being on the machine at all.
    ///
    /// **The option is the bare reading's narrower form and not the verbose
    /// one** — the near-miss above is the case where the verbose form disagrees,
    /// and this table does not consult it.
    pub what_the_tool_named: Option<String>,
    /// Whether systemd is running as the machine's first process.
    pub systemd_is_the_first_process: bool,
}

/// What K1's table gives.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Placement {
    /// The arrangement to use.
    Use(MachineKind),
    /// **K3.** The bootstrap-and-repair request answers that the machine is not
    /// one this package version can register a boot return on, **and nothing is
    /// staged and nothing is committed.** No third arrangement is guessed at.
    NotOneThisPackageCanRegisterOn,
}

/// **K1. The closed table, and nothing but the two container readings ever
/// selects the container arrangement.**
///
/// | What the two readings give | Arrangement |
/// |---|---|
/// | the container marker file is present, **or** the tool names a container technology | §1.3 |
/// | neither of those, **and** systemd is the machine's first process | §1.2 |
/// | anything else | K3's refusal |
///
/// **The systemd arrangement is selected by the second row and needs no further
/// marker.**
///
/// **The first row's second limb is *the tool named a container technology* and
/// nothing narrower.** This crate kept a closed list of nine technology names
/// here until 2026-09-04 and selected the container arrangement only for those
/// nine. **K1 carries no such list, in that row or anywhere in the document**,
/// so the list could only narrow the clause: a machine whose tool answers at
/// zero under the container option while printing a tenth name fell through to
/// the second row and was placed as a systemd machine, which is a placement K1's
/// table does not describe. The list is gone. **The tool's own container option
/// answers the row's question**, and `Servers` H12's reading of this same table
/// decides on that answer alone — the two are identical again, which is what
/// the acting half says they must be.
pub fn place(readings: &TheTwoReadings) -> Placement {
    let names_a_container = readings.what_the_tool_named.is_some();
    if readings.the_container_marker_file_is_present || names_a_container {
        return Placement::Use(MachineKind::Container);
    }
    if readings.systemd_is_the_first_process {
        return Placement::Use(MachineKind::Systemd);
    }
    Placement::NotOneThisPackageCanRegisterOn
}

/// **K2. The detected kind is a machine-derived value, re-taken at every
/// placement and never carried forward from the local record.**
///
/// It carries the installation identity it was read under (`State, identity and
/// recovery` §2.3), and **a placement never reads a kind recorded under another
/// installation.** The recycle case is why: a recycled container is a new
/// installation on the same address, and every machine-derived value held
/// against that server row is disqualified by that document's own rule.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KindAsRead {
    pub kind: MachineKind,
    /// The installation this reading was taken under.
    pub under_installation: Identity,
}

impl KindAsRead {
    /// **Whether this recorded kind may be used at a placement running under
    /// `installation`.** A kind read under another installation is not used —
    /// it is re-taken.
    pub fn may_be_used_under(&self, installation: &Identity) -> bool {
        &self.under_installation == installation
    }
}

/// **K2 stated as a rule rather than left to a caller's discipline: a placement
/// takes the reading itself and never reads one back.**
///
/// The value is `()` because there is nothing to compute — the point is that no
/// function anywhere in this crate takes a recorded kind as its input for
/// choosing an arrangement. [`place`] takes the readings.
pub const THE_KIND_IS_RE_TAKEN_AT_EVERY_PLACEMENT: () = ();

/// The reading itself, behind a trait, because taking it is the acting half's.
///
/// **The two commands are the acting half's to run**; what is here is that there
/// are exactly two readings, that they are taken in that order, and that the
/// table above is what turns them into an arrangement.
pub trait ReadsTheMachineKind {
    /// **K1's two readings, taken at this placement.** An error here is a
    /// placement that does not happen rather than a guessed arrangement.
    fn the_two_readings(&self) -> Result<TheTwoReadings, TheReadingsCouldNotBeTaken>;
}

/// The readings could not be taken, so no arrangement is chosen.
///
/// **This is K3's outcome by another route** and is treated the same way: nothing
/// is staged and nothing is committed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TheReadingsCouldNotBeTaken {
    /// What the machine said, kept verbatim.
    pub what_the_machine_said: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn nothing() -> TheTwoReadings {
        TheTwoReadings {
            the_container_marker_file_is_present: false,
            what_the_tool_named: None,
            systemd_is_the_first_process: false,
        }
    }

    #[test]
    fn a_container_marker_file_alone_places_a_container() {
        let readings = TheTwoReadings {
            the_container_marker_file_is_present: true,
            ..nothing()
        };
        assert_eq!(place(&readings), Placement::Use(MachineKind::Container));
    }

    #[test]
    fn the_tool_naming_a_container_technology_alone_places_a_container() {
        let readings = TheTwoReadings {
            what_the_tool_named: Some("docker".into()),
            ..nothing()
        };
        assert_eq!(place(&readings), Placement::Use(MachineKind::Container));
    }

    /// The near-miss of 2026-08-22: a container inside a VMware virtual
    /// machine, with systemd nowhere.
    #[test]
    fn a_container_nested_in_a_virtual_machine_is_placed_as_a_container() {
        let readings = TheTwoReadings {
            the_container_marker_file_is_present: true,
            what_the_tool_named: Some("docker".into()),
            systemd_is_the_first_process: false,
        };
        assert_eq!(place(&readings), Placement::Use(MachineKind::Container));
    }

    /// And it stays a container even where the first process were systemd —
    /// the container row is read first and the two readings are not required
    /// to agree.
    #[test]
    fn the_container_row_wins_over_the_systemd_row() {
        let readings = TheTwoReadings {
            the_container_marker_file_is_present: true,
            what_the_tool_named: None,
            systemd_is_the_first_process: true,
        };
        assert_eq!(place(&readings), Placement::Use(MachineKind::Container));
    }

    /// **The virtual machine and the bare-metal server, which are the same row.**
    /// On either, the tool asked with its container option names nothing — a
    /// virtual machine is not container virtualization — so the first row's
    /// second limb is `None` on both.
    #[test]
    fn a_bare_metal_or_virtual_machine_with_systemd_first_is_the_systemd_kind() {
        let readings = TheTwoReadings {
            what_the_tool_named: None,
            systemd_is_the_first_process: true,
            the_container_marker_file_is_present: false,
        };
        assert_eq!(place(&readings), Placement::Use(MachineKind::Systemd));
        // The bare-metal case: the tool is not on the machine at all.
        let bare = TheTwoReadings {
            systemd_is_the_first_process: true,
            ..nothing()
        };
        assert_eq!(place(&bare), Placement::Use(MachineKind::Systemd));
    }

    #[test]
    fn anything_else_is_a_placement_that_does_not_happen() {
        assert_eq!(
            place(&nothing()),
            Placement::NotOneThisPackageCanRegisterOn
        );
    }

    /// **The narrowing this crate carried until 2026-09-04, held open so it
    /// cannot come back.**
    ///
    /// A container technology nobody here has met — the tool answered at zero
    /// under its container option and printed a name this project has never
    /// seen. **K1's first row places it as a container**, because the row asks
    /// whether the tool named a container technology and not whether the name is
    /// one somebody wrote down. The closed nine-name list placed exactly this
    /// machine as a systemd one, on a box with no service manager.
    #[test]
    fn a_container_technology_nobody_here_has_met_still_places_a_container() {
        let readings = TheTwoReadings {
            what_the_tool_named: Some("something-nobody-has-met".into()),
            ..nothing()
        };
        assert_eq!(place(&readings), Placement::Use(MachineKind::Container));

        // And with systemd's own directory there as well — a container that
        // runs systemd inside itself. The container row is still read first.
        let also_running_systemd = TheTwoReadings {
            what_the_tool_named: Some("something-nobody-has-met".into()),
            systemd_is_the_first_process: true,
            the_container_marker_file_is_present: false,
        };
        assert_eq!(
            place(&also_running_systemd),
            Placement::Use(MachineKind::Container)
        );
    }

    #[test]
    fn a_kind_read_under_another_installation_may_not_be_used() {
        let one = Identity::issue(smoozy_state::identities::IdentityKind::Installation)
            .expect("randomness");
        let other = Identity::issue(smoozy_state::identities::IdentityKind::Installation)
            .expect("randomness");
        let read = KindAsRead {
            kind: MachineKind::Container,
            under_installation: one.clone(),
        };
        assert!(read.may_be_used_under(&one));
        assert!(!read.may_be_used_under(&other));
    }
}
