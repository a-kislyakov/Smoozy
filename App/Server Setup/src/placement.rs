//! What the placement is given — `Server Setup` §2.3.
//!
//! **This document supplies the candidate set and performs both hand-overs.
//! Neither choice is made here** — `Job files and outputs` 1.1 chooses the root
//! and its L4a places the package area, and both are read from that crate
//! rather than re-derived.
//!
//! **Markers used below that belong to another document, each with the document
//! it belongs to.** `Job files and outputs` L1, L2, L4a, L5, L6, L10, L11, N5,
//! N11, 1.1, 1.2, 1.3, 2.4. `State, identity and recovery` §1.3 rule 4.
//! `Server protocol and transfers` 1.1, §7.8, §9.1, S27.

use smoozy_job_files::layout::{Area, MachineRoot, TheLayout, WhereThePackageAreaSits, EVERY_AREA};
use smoozy_job_files::root_choice::{Candidate, WhatIsRecorded, THE_ROOTS_SEGMENT};
use smoozy_server_protocol::carriage::CarriageSite;
use smoozy_server_protocol::free_space::FreeSpaceReading;
use smoozy_server_protocol::transport::CommandString;

/// **R6: one writable location as this document offers it to the choice**,
/// before any reading has been taken against it.
///
/// **Taken as they arrive and with no ownership changed to become one.** A
/// volume arrives `root:root 755` and refuses the login until an elevated
/// `chown` hands it over (Fact 6; `Job files and outputs` L10) — **that
/// hand-over is [`hand_over`]'s and follows the choice rather than preceding
/// it**, because `Job files and outputs` L2 chooses on a per-path available-space
/// reading at the candidate's own path, which a `root:root 755` mount permits
/// without owning it.
///
/// # What shape a location may arrive in is not settled, and this is where a reader of this crate meets that
///
/// **Whether a path offered here can carry a relative segment, or be a link to
/// somewhere else, is established nowhere in this project.** The question was
/// parked at Build's step 4 as this module's, its Due named step 6, and step 6
/// built this module without answering it. It is recorded here so that a reader
/// of the hand-over meets it rather than finding it only in the crate that
/// receives the path.
///
/// **The half that needs no outside fact is closed, and it is closed on the
/// receiving side.** `Job files and outputs` L3 refuses a candidate path holding
/// a `.` or `..` segment — at `smoozy_job_files::layout::MachineRoot::chosen`,
/// which is Smoozy's own arithmetic against Smoozy's own rule and asks nothing
/// of any machine.
///
/// **The other half is open.** A candidate that is a link to somewhere else
/// passes the same comparison of text, and what a link does on a rented Linux
/// machine is a machine fact nobody here has read. **Nothing is guessed in its
/// place**: no shape is refused that the plans do not refuse, because refusing
/// one would be a rule nobody wrote.
///
/// **What breaks while it is open.** A root can be placed under an area the
/// machine wipes at every boot, past a test that only compares text — and after
/// one reboot the job records, the manifests and the attempt records are gone,
/// every live job re-rendering its whole range at the user's expense.
///
/// **What holds.** R6's own exclusions decide what a location *is* rather than
/// what it is named — a memory-backed filesystem, a read-only mount and a host
/// disk bind-mounted into a container are never candidates — and the receiving
/// refusal above stands whatever this side offers.
///
/// The question is parked at `Preparation/Groundwork/Open Threads.md`, in the
/// Build step 4 block, where its Due was re-pointed on 2026-09-04 to a further
/// round on this module or to the machine Build rents to watch a placement run
/// for real.
///
/// *(Added 2026-10-16, step 10c owed piece 7, on the build runner's fifth
/// ruling on the owed lines. The paragraphs above are kept as written.)*
/// **The other half is closed on the machine.** The machine program judges the
/// recorded root, and the package area's own location, by where each really
/// leads on its own disk as well as by its text —
/// `smoozy_job_files::layout::MachineRoot::chosen_where_it_really_leads` — and
/// refuses one that leads under a shared temporary area before any job is
/// given a folder under it. A path that does not exist yet is judged by its
/// nearest existing parent.
///
/// **The choice on this side still compares text**, because it is made on the
/// desktop, which cannot read the machine's disk. The login's own area it
/// offers is the file-transfer subsystem's own resolution of the folder a
/// session starts in; whether that resolution follows links is not read here.
/// **So a root chosen here that leads into a wiped area is still recorded and
/// handed over**, and the machine then refuses every job with its reason. A
/// rented machine has not yet been watched doing this.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AWritableLocation {
    pub path: String,
    /// What the location is, which is the only thing the exclusions below turn
    /// on. **Never its name**: `Job files and outputs`
    /// `NO_CANDIDATE_IS_EVER_CHOSEN_OR_REFUSED_BY_ITS_NAME` is the rule from
    /// the choosing side and this is the same rule from the offering side.
    pub what_it_is: WhatTheLocationIs,
    /// **The per-path reading taken at the candidate's own path**, through
    /// `Server protocol and transfers` §9.1's two routes, which a `root:root
    /// 755` mount permits without owning it.
    ///
    /// **R6: where a candidate's available space cannot be read at all without
    /// owning it, the reading is not obtainable, it is said so, and the
    /// candidate is dropped from the set** — rather than taken over to be
    /// measured.
    pub the_reading: Option<FreeSpaceReading>,
}

/// What a location turned out to be. **The three exclusions are the
/// 51-filesystem trap** (Fact 28): a plain reading on one container returned 51
/// filesystems, 48 memory-backed, one of them the **host's** disk bind-mounted
/// in to deliver the driver files, showing 20 GiB free the rental may not
/// touch.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhatTheLocationIs {
    /// The mount the connection login's own home area sits on.
    TheMountTheLoginsHomeAreaSitsOn,
    /// An attached volume.
    AnAttachedVolume,
    /// **Never a candidate.**
    AMemoryBackedFilesystem,
    /// **Never a candidate.**
    AReadOnlyMount,
    /// **Never a candidate.**
    AHostDiskBindMountedIntoAContainer,
}

impl WhatTheLocationIs {
    /// **R6: the set never includes a memory-backed filesystem, a read-only
    /// mount, or a host disk bind-mounted into a container.**
    pub fn may_be_a_candidate(self) -> bool {
        matches!(
            self,
            WhatTheLocationIs::TheMountTheLoginsHomeAreaSitsOn | WhatTheLocationIs::AnAttachedVolume
        )
    }
}

/// **R6: the candidate set, composed once and serving both choices.**
///
/// *"One set serves both choices and is composed once: the readings are per
/// path, the second choice draws from what the first leaves, and neither choice
/// is made here."*
///
/// **Failing to supply is a fault**, so an empty set that came out of an empty
/// input is a different thing from one that came out of every location being
/// excluded — which is why [`WhyTheSetIsEmpty`] exists rather than a bare
/// `Vec`.
pub fn compose_the_candidate_set(
    locations: &[AWritableLocation],
) -> Result<Vec<Candidate>, WhyTheSetIsEmpty> {
    if locations.is_empty() {
        return Err(WhyTheSetIsEmpty::NothingWasOffered);
    }
    let mut set = Vec::new();
    for location in locations {
        if !location.what_it_is.may_be_a_candidate() {
            continue;
        }
        // **The reading is per path and is never stood in for.** A candidate
        // whose figure could not be read is dropped, said so, and never owned
        // in order to be measured.
        let Some(reading) = location.the_reading.clone() else {
            continue;
        };
        set.push(Candidate {
            writable_location: location.path.clone(),
            reading: Some(reading),
        });
    }
    if set.is_empty() {
        return Err(WhyTheSetIsEmpty::EveryLocationWasExcludedOrUnreadable);
    }
    Ok(set)
}

/// Why the set came out empty. **The two are different faults and are never one
/// message.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhyTheSetIsEmpty {
    /// Nothing was offered at all — which is R6's *failing to supply is a
    /// fault*.
    NothingWasOffered,
    /// Every location offered was one of R6's three exclusions, or its
    /// available figure could not be read at its own path.
    EveryLocationWasExcludedOrUnreadable,
}

/// **R7: the hand-over is performed once, in R2's order, and it is completed
/// before the first job.**
///
/// *"It covers the mount `Job files and outputs` 1.1 chose for the root, and —
/// only where that document's L4a placed the package area on a second mount —
/// that mount as well."*
///
/// **No mount is ever handed over but the one or two a choice actually landed
/// on.** Handing over every attached volume merely to make it a candidate would
/// be an elevated ownership change on the user's machine for mounts that then
/// lose, with nothing recording it and nothing undoing it.
///
/// **R7: the `chown` names the root's own directory — and, where L4a placed the
/// package area apart, that location's own directory — and never the whole
/// mount.** R6's candidate set includes the mount the connection login's own
/// home area sits on, so a recursive ownership change at the mount would reach
/// files that are not Smoozy's, on the user's own rented machine, with nothing
/// recording it and nothing undoing it — **which is the same harm R6 already
/// refuses for a candidate that loses the choice.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OneHandOver {
    /// The exact path the ownership must land at.
    pub the_target_path: String,
    /// The carriage this hand-over's path rides, per R7a — **one link per
    /// mount, its own token path, its own removal**.
    pub carried_at: CarriageSite,
    /// **R7: the location this directory sits in** — the mount `Job files and
    /// outputs` 1.1 chose, which is the target's path with that document's own
    /// root segment taken off. **It is the elevated create's link target and
    /// never enters a command string.**
    ///
    /// **Absent where the path is not composed the way `Job files and outputs`
    /// composes one**, in which case no elevated create is composed either: the
    /// command's own segment is that document's build constant, and naming some
    /// other segment would be composing a path here.
    pub the_location_it_sits_in: Option<String>,
    /// **R7: the carriage the elevated create rides where the plain create is
    /// refused**, which is R7a's carriage used a second time — its own token
    /// path, its own planted link in the login's own writable area, its own
    /// removal.
    pub created_at: CarriageSite,
}

/// **R7: the location a directory of this placement sits in.**
///
/// `Job files and outputs` L1 composes the root as the chosen mount plus one
/// segment, and that segment is [`THE_ROOTS_SEGMENT`], a build constant of that
/// document. **This takes it off again and never guesses at one**, so the
/// elevated create's command can name the constant and nothing composed.
pub fn the_location_it_sits_in(path: &str) -> Option<String> {
    let rest = path.strip_suffix(THE_ROOTS_SEGMENT)?;
    let rest = rest.strip_suffix('/')?;
    Some(if rest.is_empty() {
        "/".to_string()
    } else {
        rest.to_string()
    })
}

/// **R7, performed: the hand-overs a layout actually owes.**
///
/// **On the ordinary one-mount machine there is no second choice to carry out**
/// — one candidate, one root, one `chown`, one confirming write, five areas
/// under that root.
pub fn the_hand_overs(layout: &TheLayout) -> Vec<OneHandOver> {
    let root = layout.root().as_path().to_string();
    let mut owed = vec![OneHandOver {
        the_location_it_sits_in: the_location_it_sits_in(&root),
        the_target_path: root,
        carried_at: THE_ROOTS_CARRIAGE_SITE,
        created_at: THE_ROOTS_CREATION_SITE,
    }];
    if let WhereThePackageAreaSits::ApartAt(apart) = layout.where_the_package_area_sits() {
        // **R7a: the carriage is per mount**, so the second mount is carried
        // the same way and separately — its own token path, its own planted
        // link, its own removal, and its own real-write landing check.
        let apart = apart.as_path().to_string();
        owed.push(OneHandOver {
            the_location_it_sits_in: the_location_it_sits_in(&apart),
            the_target_path: apart,
            carried_at: THE_PACKAGE_AREAS_CARRIAGE_SITE,
            created_at: THE_PACKAGE_AREAS_CREATION_SITE,
        });
    }
    owed
}

/// **R7: the five areas of `Job files and outputs` 1.2 L6 are created so no
/// deep creation is ever needed** (its 1.3, 2.4 N11). **Creating them is a
/// placement step here.**
pub fn the_areas_to_create(layout: &TheLayout) -> Vec<String> {
    EVERY_AREA
        .iter()
        .map(|area| layout.area(*area))
        .collect::<Vec<_>>()
}

/// **R7: each hand-over is confirmed by its own real write of at least one byte
/// at its exact path — never a writability test and never an empty file**
/// (`Job files and outputs` L11; `State, identity and recovery` §1.3 rule 4).
///
/// **R7a makes this the carriage's required landing check as well**: it is what
/// proves the link resolved where Smoozy planted it and the ownership landed at
/// the target. **The command's exit is never that evidence.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HowTheHandOverIsConfirmed {
    /// A real write of at least one byte, at the exact target path.
    ARealWriteOfAtLeastOneByteAtTheExactPath,
}

/// **R7 and R7a: what may be read as evidence the hand-over landed, and what
/// may not.**
pub fn the_commands_exit_confirms_the_hand_over(_it_was_zero: bool) -> bool {
    false
}

/// **R7a: the hand-over's carriage, declared against `Server protocol and
/// transfers` 1.1's admissibility rule**, because a chosen mount's path is a
/// machine-origin path and the step runs before hand-over.
///
/// **The elevated `chown` is a constant command string and no chosen path ever
/// appears in it.** The path crosses as the target of a symbolic link at a
/// Smoozy-synthesised token path — 1.1's planted-link carriage: written through
/// the file-transfer subsystem, the link in the login's own writable area and
/// **never on the mount it points at**, removed when the step ends — and the
/// command names the token.
pub const THE_ROOTS_CARRIAGE_SITE: CarriageSite = CarriageSite {
    link_path_on_the_subsystem: ".smoozy-carriage/hand-over-root",
    named_in_a_command_as: "~/.smoozy-carriage/hand-over-root",
};

/// **R7a: the second mount's own site**, because the carriage is per mount and
/// **nothing about the second mount is inferred from the first's exit or from
/// the first's landing check.**
pub const THE_PACKAGE_AREAS_CARRIAGE_SITE: CarriageSite = CarriageSite {
    link_path_on_the_subsystem: ".smoozy-carriage/hand-over-package-area",
    named_in_a_command_as: "~/.smoozy-carriage/hand-over-package-area",
};

/// **R7: the root's own directory is created as a step of this placement**,
/// before the hand-over's confirming write and before the five areas.
///
/// **Nothing anywhere in the application created it until 2026-09-04.** `Job
/// files and outputs` L1 composes the root as the chosen mount plus one
/// segment, its L6 puts four areas directly under it and its L11 puts the
/// confirming write at its exact path — and no clause told anybody to make that
/// directory, so on a machine never prepared the placement could never finish.
/// **R7 gained the creation out of this module's round-1 code audit.**
///
/// **This is the site the elevated create's link is planted at, and its target
/// is the chosen mount rather than the root** — the command then names the
/// token and the root's own segment, which is `Job files and outputs`'
/// build constant and not a machine-origin value, **so the command string stays
/// fixed at build time** and `Server protocol and transfers` 1.1 is kept by
/// construction.
pub const THE_ROOTS_CREATION_SITE: CarriageSite = CarriageSite {
    link_path_on_the_subsystem: ".smoozy-carriage/create-root",
    named_in_a_command_as: "~/.smoozy-carriage/create-root",
};

/// **R7: the package area's own location is created the same way where
/// `Job files and outputs` L4a placed it on a second mount**, and it is a second
/// site because R7a's carriage is per mount.
pub const THE_PACKAGE_AREAS_CREATION_SITE: CarriageSite = CarriageSite {
    link_path_on_the_subsystem: ".smoozy-carriage/create-package-area",
    named_in_a_command_as: "~/.smoozy-carriage/create-package-area",
};

/// **R7: the plain create over the file subsystem is tried first.**
///
/// *"It is tried unelevated first, because on a container whose login cannot
/// install packages nothing elevated runs at all"* (R3, M2) **and such a machine
/// must still be able to place a root under a location it owns.** The plain
/// create's path is a protocol field and needs no command at all, which is why
/// this is a question rather than a command string.
pub fn the_plain_create_is_tried_first() -> bool {
    true
}

/// **R7: the elevated create, where the plain one was refused.**
///
/// **Every byte is fixed at build time**: `mkdir`, the token
/// [`THE_ROOTS_CREATION_SITE`] names, and `Job files and outputs`'
/// [`THE_ROOTS_SEGMENT`]. [`tests::the_creation_commands_name_the_token_and_the_documents_own_segment`]
/// is what keeps the string and that constant in step, since a command string
/// may hold only a fixed literal.
///
/// **The make-parents form is never used**, which is `Job files and outputs`
/// N11's rule: the location the link points at is a mount that already exists,
/// so one segment is all that is made.
///
/// **What is unproved, said rather than glossed.** Whether an elevated create
/// through a planted link lands where Smoozy planted it has never been run on a
/// rented machine — nothing in this module has met one. **R7's real write of at
/// least one byte at the exact target path is what decides**, exactly as it does
/// for the hand-over itself, and the command's exit is never that evidence.
pub const THE_ROOTS_CREATION: CommandString =
    CommandString::constant("mkdir ~/.smoozy-carriage/create-root/smoozy");

/// **R7: the second location's own create**, because the carriage is per mount
/// and the two are never one command.
pub const THE_CREATION_OF_THE_PACKAGE_AREAS_LOCATION: CommandString =
    CommandString::constant("mkdir ~/.smoozy-carriage/create-package-area/smoozy");

/// **The hand-over's text, once, for a token**, so its three forms cannot
/// drift: [`crate::elevated::THE_HAND_OVER_OF_THE_ROOT`] and its neighbour put
/// each prefix in front of it.
///
/// **R7, *what already stands at the root's name*. Added 2026-11-12, out of
/// Fable's fresh look at the admin password (finding 1).** Until then the
/// `chown -R -H` was sent whatever stood at the root's name: a symbolic link
/// there was traversed, and another login's directory was taken. The text now
/// reads first, and ends with words a user can read where it changes nothing:
///
/// - `$1` is the owner, `"$(id -u):$(id -g)"`, expanded by the login's own
///   shell before the elevating command runs (1.1's fourth carriage). `$2` is
///   the token, expanded by the same shell.
/// - `readlink "$2"` is the token's target: the root's own path, read on the
///   machine and never part of the string.
/// - `[ -L "$d" ]`: that path is itself a link. Nothing is changed.
/// - `stat -c %u "$d"`: its owner. `0` is a directory R7's elevated create
///   just made; the login's own number is an earlier run's. Any other owner is
///   another login's, and nothing is changed.
/// - Only then `chown -R -H "$1" "$2"`, R7a's command on the token as before.
///
/// **Documented** — GNU coreutils manual 9.12, read 2026-11-12: `readlink`
/// *"outputs the value of the given symbolic links"*; `-L file` *"True if file
/// exists and is a symbolic link."*; `%u` *"User ID of owner"*.
macro_rules! the_guarded_hand_over {
    ($token:literal) => {
        concat!(
            "sh -c 'd=\"$(readlink \"$2\")\" && [ -n \"$d\" ] || exit 3; \
             if [ -L \"$d\" ]; then echo \"$d is a link, not a folder. Nothing was changed.\" >&2; exit 4; fi; \
             o=\"$(stat -c %u \"$d\")\" || exit 5; \
             case \"$o\" in 0|\"${1%%:*}\") ;; \
             *) echo \"$d belongs to another user. Nothing was changed.\" >&2; exit 6 ;; esac; \
             exec chown -R -H \"$1\" \"$2\"' sh \"$(id -u):$(id -g)\" ",
            $token
        )
    };
}
pub(crate) use the_guarded_hand_over;

/// **R7a: how `chown` treats a link is cited rather than assumed, because it
/// decides whether the carriage works at all.**
///
/// **Documented** — GNU coreutils manual, *chown invocation*, read 2026-08-22.
/// A symbolic link named as an argument is dereferenced by default: *"Do not
/// act on symbolic links themselves but rather on what they point to. This is
/// the default when not operating recursively."* **Under `--recursive` the
/// default reverses**, `-P` being *"Do not traverse any symbolic links. This is
/// the default if none of -H, -L, or -P is specified."* — **so the recursive
/// hand-over must carry `-H`**: *"If --recursive (-R) is specified and a command
/// line argument is a symbolic link to a directory, traverse it."*
///
/// **A recursive `chown` through the token without `-H` changes nothing on the
/// mount, and nothing in its exit status says so.**
///
/// **The owner the command names rides `Server protocol and transfers` 1.1's
/// fourth carriage — machine self-derivation — and never Smoozy's own copy of
/// the login name.** The connection's login is a user- or provider-origin
/// identifier and not a path, so none of the other three carriages fits it;
/// 1.1's fourth is written for exactly this class — *"fixed literals embedding a
/// double-quoted command substitution the login shell expands at execution"*, so
/// the value originates on the machine and never crosses from Smoozy at all.
/// Its citations are that document's: `sshd(8)`'s LOGIN PROCESS, the Shell
/// Command Language §2.6.3, and POSIX `id`, all read 2026-08-22. **Its failure
/// is loud rather than silent** — a mangled substitution names no login the
/// machine holds, and the real-write landing check below is what proves the
/// ownership landed.
///
/// **R7, *what already stands at the root's name* (added 2026-11-12): the
/// command reads what stands there before it changes anything.** See
/// [`the_guarded_hand_over`].
pub const THE_RECURSIVE_HAND_OVER: CommandString =
    CommandString::constant(the_guarded_hand_over!("~/.smoozy-carriage/hand-over-root"));

/// **The second mount's own hand-over command**, because R7a's carriage is per
/// mount and the two are never one command.
pub const THE_RECURSIVE_HAND_OVER_OF_THE_PACKAGE_AREA: CommandString = CommandString::constant(
    the_guarded_hand_over!("~/.smoozy-carriage/hand-over-package-area"),
);

/// **R7a, as the question a builder would otherwise get wrong.** A recursive
/// hand-over that does not carry `-H` traverses nothing and reports success.
pub fn a_recursive_hand_over_needs_the_traverse_option(recursive: bool) -> bool {
    recursive
}

/// **R7b: the recording is a step of this hand-over, and there is a second
/// recorded path only where there is a second mount.**
///
/// The root's path is recorded at this moment, **in the two places `Job files
/// and outputs` L1 names and in the form it names them**; what is recorded is
/// that document's and is not restated here. Where L4a placed the package area
/// on a second mount, **that location is recorded the same way and in the same
/// durable act as the root's**, so a reading that finds one finds both.
///
/// **Where L4a's second choice does not fire, nothing second is chosen,
/// recorded or handed over**, and a builder must not read the presence of the
/// root's record as evidence that a second one is owed.
pub fn what_the_hand_over_records(layout: &TheLayout) -> WhatIsRecorded {
    WhatIsRecorded {
        root_path: layout.root().as_path().to_string(),
        the_package_areas_own_location: match layout.where_the_package_area_sits() {
            WhereThePackageAreaSits::UnderTheRoot => None,
            WhereThePackageAreaSits::ApartAt(apart) => Some(apart.as_path().to_string()),
        },
    }
}

/// **R7: the Blender area is one of the four that sit directly under the root**
/// and is untouched by `Job files and outputs` L4a's placing of the package
/// area on a mount of its own — which is [`crate::blender`] B9's closure.
pub fn the_blender_area(layout: &TheLayout) -> String {
    layout.area(Area::Blender)
}

/// **The planted link is recorded staging under `Server protocol and
/// transfers` §7.8's discipline**: a removal that could not run is an owed
/// cleanup, never a forgotten one (its S27's shape). **Nothing here invents a
/// second sweep** — the link's removal rides that contract.
pub const THE_LINKS_REMOVAL_IS_THAT_DOCUMENTS_OWED_CLEANUP: bool = true;

// ═══════════════════════════════════════════════════════════════════════════
// R6a — the attached volume, found and offered
// ═══════════════════════════════════════════════════════════════════════════

/// **R6a: one mount as `/proc/self/mountinfo` names it** — the four fields the
/// clause turns on, each read from the field that holds it (Linux man-pages
/// 6.19, `proc_pid_mountinfo(5)`, read 2026-11-13).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AMountAsRead {
    /// *"major:minor: the value of st_dev for files on this filesystem"*.
    pub device: String,
    /// *"mount point: the pathname of the mount point relative to the
    /// process's root directory"*.
    pub mount_point: String,
    /// *"mount options: per-mount options"*, split at the commas.
    pub per_mount_options: Vec<String>,
    /// *"filesystem type: the filesystem type in the form "type[.subtype]""*.
    pub filesystem_type: String,
    /// *"mount source: filesystem-specific information or "none""* — for a
    /// disk, the device it is on.
    pub mount_source: String,
}

/// **R6a: the reading, line by line.** A line that is not the documented shape
/// — fewer fields than the six before the optional ones, or no single hyphen
/// after them with a type behind it — is passed over, never guessed at.
pub fn the_mounts_as_read(answer: &str) -> Vec<AMountAsRead> {
    let mut mounts = Vec::new();
    for line in answer.lines() {
        let fields: Vec<&str> = line.split_whitespace().collect();
        if fields.len() < 6 {
            continue;
        }
        let Some(separator) = fields.iter().skip(6).position(|field| *field == "-").map(|at| at + 6) else {
            continue;
        };
        let (Some(filesystem_type), Some(mount_source)) = (fields.get(separator + 1), fields.get(separator + 2)) else {
            continue;
        };
        mounts.push(AMountAsRead {
            device: fields[2].to_string(),
            mount_point: fields[4].to_string(),
            per_mount_options: fields[5].split(',').map(str::to_string).collect(),
            filesystem_type: filesystem_type.to_string(),
            mount_source: mount_source.to_string(),
        });
    }
    mounts
}

/// **R6a: the filesystem types held in memory** — `tmpfs` (`tmpfs(5)`), and
/// `ramfs` and `devtmpfs` (the kernel's `ramfs-rootfs-initramfs.txt` and
/// `DEVTMPFS`'s help text), read 2026-11-13 (`Server Setup` sources, §R6a).
pub const THE_FILESYSTEMS_HELD_IN_MEMORY: [&str; 3] = ["tmpfs", "ramfs", "devtmpfs"];

/// **R6a: the system's own folders, never a place for Smoozy's** — this
/// application's decision of 2026-11-13, so the kernel's own mounts under them
/// are never measured.
pub const THE_SYSTEMS_OWN_FOLDERS: [&str; 4] = ["/proc", "/sys", "/dev", "/run"];

/// **R6a's tests on the mounts, before anything is measured**: not under the
/// system's own folders, not held in memory, not `ro`, no escaped character,
/// not the home mount's device and not a device already kept. **Until R6a's
/// change of later on 2026-11-13 the source had to be a device under `/dev/`**,
/// which kept memory out and every network volume with it. **The home mount is the one with the longest mount point that
/// is a path prefix of the login's own area**; where none is, nothing is kept,
/// because a device that cannot be told from the home mount's is never offered.
pub fn the_mount_points_worth_measuring(mounts: &[AMountAsRead], the_logins_own_area: &str) -> Vec<String> {
    let holds_the_area = |mount_point: &str| {
        mount_point == "/"
            || the_logins_own_area == mount_point
            || the_logins_own_area.starts_with(&format!("{}/", mount_point.trim_end_matches('/')))
    };
    let Some(home) = mounts
        .iter()
        .filter(|mount| the_logins_own_area.starts_with('/') && holds_the_area(&mount.mount_point))
        .max_by_key(|mount| mount.mount_point.len())
    else {
        return Vec::new();
    };
    let mut kept_devices = vec![home.device.clone()];
    let mut worth = Vec::new();
    for mount in mounts {
        // **R6a's change of later on 2026-11-13, item 3 (Sol's look at the
        // scout's list, point 3): the source's name is no test**, so a network
        // volume counts. Memory is kept out by its type — the three the
        // kernel's own documents name — and the system's own folders are never
        // a place for Smoozy's, which is this application's decision.
        let in_memory = THE_FILESYSTEMS_HELD_IN_MEMORY.contains(&mount.filesystem_type.as_str());
        let the_systems_own = THE_SYSTEMS_OWN_FOLDERS.iter().any(|folder| {
            mount.mount_point == *folder || mount.mount_point.starts_with(&format!("{folder}/"))
        });
        let read_only = mount.per_mount_options.iter().any(|option| option == "ro");
        let escaped = mount.mount_point.contains('\\');
        if the_systems_own
            || in_memory
            || read_only
            || escaped
            || !mount.mount_point.starts_with('/')
            || kept_devices.contains(&mount.device)
        {
            continue;
        }
        kept_devices.push(mount.device.clone());
        worth.push(mount.mount_point.clone());
    }
    worth
}

/// **R6a: one volume as it was measured** — its free space, whether it is a
/// directory, and whether the login can really write there (R6a's change of
/// later on 2026-11-13, item 3: the login's own test write landed at the
/// mount point, or admin access needs no password and R7's hand-over is what
/// is tried, with the home mount alone where it does not land).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AVolumeMeasured {
    pub volume: AWritableLocation,
    pub it_is_a_directory: bool,
    pub the_login_can_write: bool,
}

/// **R6a: of the volumes measured, the one offered** — a directory, read at its
/// own path, with a larger available figure than the home mount's, the largest
/// of them winning and the first given winning a tie. **Nothing where the home
/// mount has no figure**, since *larger than* has nothing to be larger than.
pub fn the_volume_offered(
    home: &AWritableLocation,
    measured: Vec<AVolumeMeasured>,
) -> Option<AWritableLocation> {
    let home_figure = home.the_reading.as_ref()?.available_bytes;
    let mut offered: Option<(AWritableLocation, u64)> = None;
    for AVolumeMeasured { volume, it_is_a_directory, the_login_can_write } in measured {
        let Some(reading) = volume.the_reading.as_ref() else {
            continue;
        };
        if !it_is_a_directory
            || !the_login_can_write
            || reading.the_path_it_was_taken_against != volume.path
            || reading.available_bytes <= home_figure
        {
            continue;
        }
        let figure = reading.available_bytes;
        if offered.as_ref().is_none_or(|(_, so_far)| figure > *so_far) {
            offered = Some((volume, figure));
        }
    }
    offered.map(|(volume, _)| AWritableLocation {
        what_it_is: WhatTheLocationIs::AnAttachedVolume,
        ..volume
    })
}

/// **R6a: where the record of the root would be, at a location, on a one-mount
/// machine** — and so where it is on either arrangement, since L4a's apart
/// package area sits at that same path.
pub fn the_record_of_the_root_for(location: &str) -> Option<String> {
    let root = MachineRoot::chosen(&format!("{}/{}", location.trim_end_matches('/'), THE_ROOTS_SEGMENT)).ok()?;
    let layout = TheLayout::of(root, WhereThePackageAreaSits::UnderTheRoot);
    Some(smoozy_server_protocol::placement::records::the_record_at(
        &layout.area(Area::Package),
        smoozy_job_files::layout::THE_RECORD_OF_THE_ROOT,
    ))
}

/// **R6a's change of later on 2026-11-13, item 1: what the read of the record
/// of the root at the home location says** — `Ok(None)` only where the record
/// is not there, which is a first placement; the layout where it names a root
/// Smoozy may use; **and the reason in words for anything else**, which stops
/// the setup rather than choosing a disk again over a root that stands (Sol's
/// look at the scout's list, point 2).
pub fn what_the_record_of_the_root_says(
    location: &str,
    at: &str,
    read: Result<Vec<u8>, crate::readings::WhatTheMachineSaid>,
) -> Result<Option<TheLayout>, String> {
    use crate::readings::WhatTheMachineSaid;
    let bytes = match read {
        Ok(bytes) => bytes,
        Err(WhatTheMachineSaid::ThePathIsNotThere) => return Ok(None),
        Err(other) => return Err(format!("Could not read {at}: {}.", other.in_words())),
    };
    let Ok(recorded) = String::from_utf8(bytes) else {
        return Err(format!("Could not read {at}: it does not contain a folder path."));
    };
    the_layout_a_recorded_root_gives(location, &recorded).map(Some).ok_or_else(|| {
        format!("Could not read {at}: it names a folder Smoozy can't use, {}.", recorded.trim())
    })
}

/// **R6a: a machine that already holds a recorded root keeps it** (L1). The
/// layout rebuilt from the record found at `location`: the recorded root, and
/// the package area apart at that location where the recorded root is not the
/// location's own. **Nothing where the record does not name a root Smoozy may
/// use**, which is `the_layout_again`'s own refusal.
pub fn the_layout_a_recorded_root_gives(location: &str, recorded_root: &str) -> Option<TheLayout> {
    let recorded_root = recorded_root.trim();
    let own = format!("{}/{}", location.trim_end_matches('/'), THE_ROOTS_SEGMENT);
    smoozy_job_files::root_choice::the_layout_again(&WhatIsRecorded {
        root_path: recorded_root.to_string(),
        the_package_areas_own_location: (recorded_root != own).then_some(own),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use smoozy_job_files::layout::MachineRoot;

    fn a_location(path: &str, what: WhatTheLocationIs, figure: Option<u64>) -> AWritableLocation {
        AWritableLocation {
            path: path.to_string(),
            what_it_is: what,
            the_reading: figure.map(|available_bytes| FreeSpaceReading {
                available_bytes,
                the_path_it_was_taken_against: path.to_string(),
                route: smoozy_server_protocol::free_space::WhichRoute::TheStatvfsExtension,
            }),
        }
    }

    fn a_layout(root: &str, apart: Option<&str>) -> TheLayout {
        TheLayout::of(
            MachineRoot::chosen(root).expect("a root"),
            match apart {
                None => WhereThePackageAreaSits::UnderTheRoot,
                Some(path) => WhereThePackageAreaSits::ApartAt(
                    MachineRoot::chosen(path).expect("a second root"),
                ),
            },
        )
    }

    #[test]
    fn the_three_exclusions_never_reach_the_candidate_set() {
        // Fact 28's 51-filesystem trap: 48 memory-backed, one of them the
        // host's disk bind-mounted in showing 20 GiB the rental may not touch.
        let locations = [
            a_location("/home/smoozy", WhatTheLocationIs::TheMountTheLoginsHomeAreaSitsOn, Some(10)),
            a_location("/dev/shm", WhatTheLocationIs::AMemoryBackedFilesystem, Some(9_000)),
            a_location("/snap/x", WhatTheLocationIs::AReadOnlyMount, Some(9_000)),
            a_location(
                "/usr/lib/host",
                WhatTheLocationIs::AHostDiskBindMountedIntoAContainer,
                Some(20 * 1024 * 1024 * 1024),
            ),
        ];
        let set = compose_the_candidate_set(&locations).expect("one candidate stands");
        assert_eq!(set.len(), 1);
        assert_eq!(set[0].writable_location, "/home/smoozy");
    }

    #[test]
    fn a_candidate_whose_figure_cannot_be_read_is_dropped_and_never_owned_to_be_measured() {
        let locations = [
            a_location("/vol", WhatTheLocationIs::AnAttachedVolume, None),
            a_location("/home", WhatTheLocationIs::TheMountTheLoginsHomeAreaSitsOn, Some(5)),
        ];
        let set = compose_the_candidate_set(&locations).expect("one candidate stands");
        assert_eq!(set.len(), 1);
        assert_eq!(set[0].writable_location, "/home");
    }

    #[test]
    fn the_two_empty_endings_are_different_faults() {
        assert_eq!(
            compose_the_candidate_set(&[]),
            Err(WhyTheSetIsEmpty::NothingWasOffered)
        );
        assert_eq!(
            compose_the_candidate_set(&[a_location(
                "/dev/shm",
                WhatTheLocationIs::AMemoryBackedFilesystem,
                Some(1)
            )]),
            Err(WhyTheSetIsEmpty::EveryLocationWasExcludedOrUnreadable)
        );
    }

    #[test]
    fn the_ordinary_one_mount_machine_has_one_hand_over_and_five_areas() {
        let layout = a_layout("/home/smoozy/smoozy", None);
        let owed = the_hand_overs(&layout);
        assert_eq!(owed.len(), 1);
        assert_eq!(owed[0].the_target_path, "/home/smoozy/smoozy");
        assert_eq!(the_areas_to_create(&layout).len(), 5);
        assert_eq!(what_the_hand_over_records(&layout).the_package_areas_own_location, None);
    }

    #[test]
    fn a_second_mount_gets_its_own_carriage_and_its_own_record_in_the_same_act() {
        // R7a: nothing about the second mount is inferred from the first's
        // exit or from the first's landing check. R7b: a reading that finds one
        // finds both.
        let layout = a_layout("/home/smoozy/smoozy", Some("/vol/smoozy"));
        let owed = the_hand_overs(&layout);
        assert_eq!(owed.len(), 2);
        assert_ne!(
            owed[0].carried_at.link_path_on_the_subsystem,
            owed[1].carried_at.link_path_on_the_subsystem
        );
        let recorded = what_the_hand_over_records(&layout);
        assert_eq!(recorded.root_path, "/home/smoozy/smoozy");
        assert_eq!(
            recorded.the_package_areas_own_location,
            Some("/vol/smoozy".to_string())
        );
    }

    #[test]
    fn the_creation_commands_name_the_token_and_the_documents_own_segment() {
        // **R7: the segment is `Job files and outputs`' build constant and not a
        // machine-origin value**, so the command string stays fixed at build
        // time. A command string may hold only a fixed literal, so this is what
        // keeps the two in step.
        for (command, site) in [
            (THE_ROOTS_CREATION, THE_ROOTS_CREATION_SITE),
            (
                THE_CREATION_OF_THE_PACKAGE_AREAS_LOCATION,
                THE_PACKAGE_AREAS_CREATION_SITE,
            ),
        ] {
            let named = command
                .as_str()
                .strip_prefix("mkdir ")
                .expect("the plain form is the make-directory");
            let (token, segment) = named
                .rsplit_once('/')
                .expect("the command names a token and a segment");
            assert_eq!(token, site.named_in_a_command_as);
            assert_eq!(segment, THE_ROOTS_SEGMENT);
            // **Never the make-parents form** — the location the link points at
            // already exists and one segment is all that is made.
            assert!(!command.as_str().contains(" -p"));
            assert!(site.is_drawn_from_class_twos_alphabet());
        }
        assert!(the_plain_create_is_tried_first());
        // The two sites are separate, because the carriage is per mount.
        assert_ne!(
            THE_ROOTS_CREATION_SITE.link_path_on_the_subsystem,
            THE_PACKAGE_AREAS_CREATION_SITE.link_path_on_the_subsystem
        );
    }

    #[test]
    fn the_location_a_root_sits_in_is_its_path_with_that_documents_own_segment_taken_off() {
        assert_eq!(
            the_location_it_sits_in("/home/smoozy/smoozy").as_deref(),
            Some("/home/smoozy")
        );
        assert_eq!(the_location_it_sits_in("/smoozy").as_deref(), Some("/"));
        // **A path not composed the way `Job files and outputs` composes one is
        // not narrowed here**, and no elevated create is composed for it.
        assert_eq!(the_location_it_sits_in("/mnt/one/elsewhere"), None);
    }

    #[test]
    fn both_carriage_tokens_are_drawn_from_the_admissible_alphabet() {
        // `Server protocol and transfers` 1.1's class-2 test: *"A token so
        // drawn is syntax on no route."*
        assert!(THE_ROOTS_CARRIAGE_SITE.is_drawn_from_class_twos_alphabet());
        assert!(THE_PACKAGE_AREAS_CARRIAGE_SITE.is_drawn_from_class_twos_alphabet());
        // And no chosen path ever appears in the command.
        assert!(!THE_RECURSIVE_HAND_OVER.as_str().contains("/home"));
        assert!(THE_RECURSIVE_HAND_OVER
            .as_str()
            .contains(THE_ROOTS_CARRIAGE_SITE.named_in_a_command_as));
    }

    #[test]
    fn the_recursive_hand_over_carries_the_traverse_option() {
        // Without `-H` a recursive `chown` through the token changes nothing on
        // the mount, and nothing in its exit status says so.
        assert!(a_recursive_hand_over_needs_the_traverse_option(true));
        assert!(THE_RECURSIVE_HAND_OVER.as_str().contains(" -H "));
        assert!(THE_RECURSIVE_HAND_OVER_OF_THE_PACKAGE_AREA
            .as_str()
            .contains(" -H "));
    }

    #[test]
    fn the_owner_rides_the_fourth_carriage_and_no_login_name_crosses_from_smoozy() {
        // `Server protocol and transfers` 1.1's machine self-derivation: the
        // command string stays constant and the machine derives its own login
        // at execution.
        for command in [
            THE_RECURSIVE_HAND_OVER,
            THE_RECURSIVE_HAND_OVER_OF_THE_PACKAGE_AREA,
        ] {
            assert!(command.as_str().contains("\"$(id -u):$(id -g)\""));
        }
        // And the two commands are separate, because the carriage is per mount.
        assert_ne!(
            THE_RECURSIVE_HAND_OVER.as_str(),
            THE_RECURSIVE_HAND_OVER_OF_THE_PACKAGE_AREA.as_str()
        );
    }

    #[test]
    fn the_landing_check_is_a_real_write_and_never_the_commands_exit() {
        assert!(!the_commands_exit_confirms_the_hand_over(true));
        assert!(!the_commands_exit_confirms_the_hand_over(false));
        // **One way to confirm and no other**, checked by the match having to be
        // exhaustive: a second way added here stops this compiling. The
        // assertion that stood in its place compared one value with itself.
        match HowTheHandOverIsConfirmed::ARealWriteOfAtLeastOneByteAtTheExactPath {
            HowTheHandOverIsConfirmed::ARealWriteOfAtLeastOneByteAtTheExactPath => {}
        }
    }

    #[test]
    fn the_blender_area_is_a_sibling_under_the_root_even_where_the_package_area_moved() {
        // B9's closure: the Blender area is one of the four that sit directly
        // under the root.
        let layout = a_layout("/home/smoozy/smoozy", Some("/vol/smoozy"));
        assert_eq!(the_blender_area(&layout), "/home/smoozy/smoozy/blender");
        const { assert!(THE_LINKS_REMOVAL_IS_THAT_DOCUMENTS_OWED_CLEANUP) };
    }

    // ── R6a, added 2026-11-13 (the scout's list, item 1) ────────────────────

    /// **A reading composed in the documented shape** (`proc_pid_mountinfo(5)`),
    /// standing for the two disks the live passes saw on one seller's virtual
    /// machine — 97 GB at `/` and 787 GB at `/ephemeral` — with the memory
    /// mounts, a read-only mount and the boot partition beside them. The device
    /// numbers are this test's own.
    const A_VIRTUAL_MACHINE: &str = "\
22 1 252:1 / / rw,relatime shared:1 - ext4 /dev/vda1 rw,discard
23 22 0:5 / /dev rw,nosuid,relatime shared:2 - devtmpfs udev rw,size=61000000k
24 23 0:24 / /dev/shm rw,nosuid,nodev shared:4 - tmpfs tmpfs rw
25 22 0:25 / /run rw,nosuid,nodev,noexec shared:5 - tmpfs tmpfs rw,size=12000000k
26 22 0:21 / /proc rw,nosuid,nodev,noexec,relatime shared:12 - proc proc rw
27 22 252:15 / /boot/efi rw,relatime shared:30 - vfat /dev/vda15 rw
28 22 252:16 / /ephemeral rw,relatime shared:31 - ext4 /dev/vdb rw
29 22 7:1 / /snap/core22/1 ro,nodev,relatime shared:32 - squashfs /dev/loop1 ro
30 22 252:16 / /home/sesterce/again rw,relatime shared:31 - ext4 /dev/vdb rw
31 22 252:17 / /with\\040space rw,relatime shared:33 - ext4 /dev/vdc rw
garbled line
";

    #[test]
    fn r6a_the_reading_is_read_in_its_documented_fields() {
        let mounts = the_mounts_as_read(A_VIRTUAL_MACHINE);
        assert_eq!(mounts.len(), 10, "every line in the documented shape, and the garbled one passed over");
        assert_eq!(
            mounts[6],
            AMountAsRead {
                device: "252:16".into(),
                mount_point: "/ephemeral".into(),
                per_mount_options: vec!["rw".into(), "relatime".into()],
                filesystem_type: "ext4".into(),
                mount_source: "/dev/vdb".into(),
            }
        );
        // **The optional fields are any number, and the type is after the
        // hyphen whatever comes before it.**
        let none_optional = the_mounts_as_read("36 35 98:0 /mnt1 /mnt2 rw,noatime - ext3 /dev/root rw\n");
        assert_eq!(none_optional[0].filesystem_type, "ext3");
        let two_optional = the_mounts_as_read("36 35 98:0 /mnt1 /mnt2 rw shared:1 master:2 - xfs /dev/sdb rw\n");
        assert_eq!(two_optional[0].filesystem_type, "xfs");
        assert_eq!(two_optional[0].mount_point, "/mnt2");
    }

    #[test]
    fn r6a_only_a_separate_writable_disk_that_is_not_in_memory_is_worth_measuring() {
        let mounts = the_mounts_as_read(A_VIRTUAL_MACHINE);
        // `/dev`, `/dev/shm`, `/run` and `/proc` name no device — `/dev`'s
        // 61 GB is memory; the snap is `ro`; the home mount is `/`; the
        // second mount of `/dev/vdb` is the same device; the escaped one is
        // read rather than unescaped. **The boot partition stays here** and is
        // left to the measuring, where it is not larger than the home mount.
        assert_eq!(
            the_mount_points_worth_measuring(&mounts, "/home/sesterce"),
            vec!["/boot/efi".to_string(), "/ephemeral".to_string()]
        );
        // **R6a's change of later on 2026-11-13, item 3 (Sol's look at the
        // scout's list, point 3): a network volume counts, whatever its source
        // names, and memory is kept out by its type** — `ramfs` and
        // `devtmpfs` with `tmpfs` — or by sitting in the system's own folders.
        let more = the_mounts_as_read(&format!(
            "{A_VIRTUAL_MACHINE}\
40 22 0:50 / /mnt/share rw,relatime shared:40 - nfs4 10.0.0.5:/export rw
41 22 0:51 / /mnt/ram rw,relatime shared:41 - ramfs ramfs rw
42 22 0:52 / /mnt/devices rw,relatime shared:42 - devtmpfs udev rw
43 22 0:53 / /sys rw,relatime shared:43 - sysfs sysfs rw
44 22 0:54 / /workspace rw,relatime shared:44 - fuse.mfs mfs#volume rw
"
        ));
        assert_eq!(
            the_mount_points_worth_measuring(&more, "/home/sesterce"),
            vec![
                "/boot/efi".to_string(),
                "/ephemeral".to_string(),
                "/mnt/share".to_string(),
                "/workspace".to_string()
            ]
        );
        // **A home area on the big disk itself makes the big disk the home
        // mount**, and nothing on it is offered twice.
        assert!(!the_mount_points_worth_measuring(&mounts, "/ephemeral/me").contains(&"/ephemeral".to_string()));
        // **No mount that holds the home area: nothing is offered.**
        assert!(the_mount_points_worth_measuring(&mounts, "relative/home").is_empty());
    }

    #[test]
    fn r6a_the_largest_directory_bigger_than_the_home_mount_is_offered_and_nothing_else() {
        let gb = 1_000_000_000_u64;
        let home = a_location("/home/sesterce", WhatTheLocationIs::TheMountTheLoginsHomeAreaSitsOn, Some(83 * gb));
        let volume = |path: &str, figure: Option<u64>| {
            a_location(path, WhatTheLocationIs::AnAttachedVolume, figure)
        };
        let measured = |path: &str, figure: Option<u64>, it_is_a_directory: bool, the_login_can_write: bool| AVolumeMeasured {
            volume: volume(path, figure),
            it_is_a_directory,
            the_login_can_write,
        };
        let offered = the_volume_offered(
            &home,
            vec![
                measured("/boot/efi", Some(gb / 10), true, true),
                measured("/ephemeral", Some(747 * gb), true, true),
                measured("/vol2", Some(300 * gb), true, true),
                // **A single file bound in from the host** — a container's
                // `/etc/hosts` on a 7 TB disk — is not a directory.
                measured("/etc/hosts", Some(7_000 * gb), false, true),
                measured("/unread", None, true, true),
                // **R6a's change of later on 2026-11-13, item 3: a volume
                // the login cannot really write to is not offered**, however
                // large.
                measured("/readonly-share", Some(9_000 * gb), true, false),
            ],
        );
        assert_eq!(offered.map(|it| it.path), Some("/ephemeral".to_string()));

        // **Nothing bigger: nothing offered**, so a small extra mount never
        // pulls the package area onto itself.
        assert_eq!(the_volume_offered(&home, vec![measured("/boot/efi", Some(gb / 10), true, true)]), None);
        // **A figure read against another path is not this volume's figure.**
        let mut elsewhere = measured("/ephemeral", Some(747 * gb), true, true);
        elsewhere.volume.the_reading.as_mut().unwrap().the_path_it_was_taken_against = "/".into();
        assert_eq!(the_volume_offered(&home, vec![elsewhere]), None);
        // **No home figure: nothing to be larger than.**
        let unread_home = a_location("/home/sesterce", WhatTheLocationIs::TheMountTheLoginsHomeAreaSitsOn, None);
        assert_eq!(the_volume_offered(&unread_home, vec![measured("/ephemeral", Some(747 * gb), true, true)]), None);

        // **And the choice then puts the root on the volume and the package
        // area apart on the home mount, at the path it has on a one-mount
        // machine** — so the program's own place does not move.
        let set = compose_the_candidate_set(&[home.clone(), volume("/ephemeral", Some(747 * gb))]).unwrap();
        let chosen = smoozy_job_files::root_choice::choose(&set).unwrap();
        assert_eq!(chosen.layout.root().as_path(), "/ephemeral/smoozy");
        assert_eq!(chosen.layout.area(Area::Package), "/home/sesterce/smoozy/package");
        let one_mount = smoozy_job_files::root_choice::choose(&compose_the_candidate_set(&[home]).unwrap()).unwrap();
        assert_eq!(one_mount.layout.area(Area::Package), chosen.layout.area(Area::Package));
    }

    #[test]
    fn r6a_a_recorded_root_is_kept_on_either_arrangement() {
        // **The record sits at one path on either arrangement.**
        assert_eq!(
            the_record_of_the_root_for("/home/sesterce").as_deref(),
            Some("/home/sesterce/smoozy/package/root")
        );
        // **A server set up before R6a**: everything under the home folder.
        let before = the_layout_a_recorded_root_gives("/home/sesterce", "/home/sesterce/smoozy\n").unwrap();
        assert_eq!(before, a_layout("/home/sesterce/smoozy", None));
        // **A server set up with R6a**: the root on the big disk, the package
        // area apart on the home mount.
        let after = the_layout_a_recorded_root_gives("/home/sesterce", "/ephemeral/smoozy").unwrap();
        assert_eq!(after, a_layout("/ephemeral/smoozy", Some("/home/sesterce/smoozy")));
        // **A record naming a root Smoozy may not use rebuilds nothing.**
        assert_eq!(the_layout_a_recorded_root_gives("/home/sesterce", "/tmp/smoozy"), None);
        assert_eq!(the_layout_a_recorded_root_gives("/home/sesterce", ""), None);
    }

    /// **R6a's change of later on 2026-11-13, item 1 (Sol's look at the
    /// scout's list, point 2): only a record that is not there is a first
    /// placement.** Permission denied, a channel gone, bytes that are not
    /// text and text that names no root Smoozy may use each stop the setup
    /// with the reason, rather than letting a fresh choice write a second root.
    #[test]
    fn r6a_a_record_that_cannot_be_read_stops_and_only_one_not_there_is_a_first_placement() {
        use crate::readings::WhatTheMachineSaid as Said;
        let at = "/home/sesterce/smoozy/package/root";
        let says = |read| what_the_record_of_the_root_says("/home/sesterce", at, read);
        assert_eq!(says(Err(Said::ThePathIsNotThere)), Ok(None));
        assert_eq!(
            says(Ok(b"/ephemeral/smoozy\n".to_vec())),
            Ok(Some(a_layout("/ephemeral/smoozy", Some("/home/sesterce/smoozy"))))
        );
        for refused in [Said::TheLoginMayNot, Said::TheChannelIsGone, Said::ItRefused] {
            let why = says(Err(refused.clone())).expect_err("an unreadable record stops the setup");
            assert!(why.starts_with(&format!("Could not read {at}: ")), "{why}");
            assert!(why.contains(refused.in_words()), "{why}");
        }
        assert_eq!(
            says(Ok(vec![0xff, 0xfe])),
            Err(format!("Could not read {at}: it does not contain a folder path."))
        );
        assert_eq!(
            says(Ok(b"/tmp/smoozy".to_vec())),
            Err(format!("Could not read {at}: it names a folder Smoozy can't use, /tmp/smoozy."))
        );
    }
}
