//! The program the rented machine runs: its fixed entry point, the one target it
//! is built for, the three executable classes it carries, and the package area it
//! finds itself in.
//!
//! # Why this file exists
//!
//! **Until it was written, twenty modules' worth of behaviour could not be placed
//! on a rented machine.** `The shape` settles that the server package holds
//! **three executable classes or roles**, `Server protocol and transfers` B8
//! requires the signed release manifest to name **one fixed entry-point name** per
//! artifact, and every clause of this plan's §2 consumes an artifact. **No step
//! before this one produced a program at all** — the machine-side crate was an
//! ordinary library of the desktop application and of nothing else, and the only
//! binary in the workspace was the desktop's.
//!
//! # The three classes, and they are roles of one program
//!
//! `The shape`'s *Each rented server* names them: the **server supervisor**, the
//! **SSH command adapter**, and the **operation workers** — plus the **Telegram
//! role worker**, which is the one role kind of §5.3 and is a worker like any
//! other as far as being started goes. **They are roles of one program rather
//! than separate programs**, because B8 gives an artifact **one** fixed
//! entry-point name and the artifact is one executable file whose byte length and
//! digest that clause and B10 verify. Four names would want four artifacts, four
//! digests and four pointers, which is not the shape B8 describes.
//!
//! # How a role is chosen, and why the bare invocation is the adapter
//!
//! **A bare invocation is the SSH command adapter**, because `Server protocol and
//! transfers` §1.1 says exactly that of the command channel after hand-over:
//! Smoozy sends *"one constant, installation-owned command — the active package's
//! fixed entry-point name … and sends every value on that entry point's standard
//! input"*, and *"That entry carries the SSH command adapter"*. The command string
//! is the entry point's own path with nothing after it, so the program invoked
//! with no operand is that adapter and can be nothing else.
//!
//! **The other two carry a fixed word**, because both invocations are Smoozy's
//! own — the boot registration this crate composes, and the supervisor's own
//! worker start, which builds an argument vector rather than a command line.
//!
//! # Where the program finds itself
//!
//! **The package area is the directory the entry point was invoked from, and that
//! is a consequence of the arrangement rather than a convention invented here.**
//! The active-package pointer sits in the package area, it carries the fixed
//! entry-point name, and §1.1's constant command string, G7's boot registration
//! and the supervisor's own worker start all name that one path. So the directory
//! holding the invoked path is the package area on all three routes.
//!
//! **It is derived from the invoked path and never from the resolved executable.**
//! The pointer is a link and the placed artifact sits at the staging location the
//! phase record names, so a reading that followed the link would answer with the
//! staging directory and not with the package area.
//!
//! **A relative invoked path is refused loudly rather than resolved against
//! whatever directory the caller happened to be in.** All three routes name an
//! absolute path, so a relative one is a caller this program does not have.
//!
//! # What is not decided here
//!
//! **The set of tuples the product supports is not this document's and not
//! Build's.** `Server protocol and transfers` B8 says outright that it *"is the
//! Creator-ruling boundary outside this document"*. What is written below is the
//! one tuple this build is made for; a manifest naming more than one entry is the
//! third of step 8a's three things and reaches the Creator.
//!
//! **Markers used in this file that belong to another document, each with the
//! document it belongs to.** `Server protocol and transfers` — B8, B10.
//! `State, identity and recovery` §1.3 rule 6.
//!
//! **A marker this plan also defines is never listed here and is written out with
//! its document at the site**, because a header list cannot tell the two apart and
//! a bare one there would resolve to a real and wrong clause.

use crate::workers::WorkerKind;

/// **`Server protocol and transfers` B8's fixed entry-point name.**
///
/// That clause requires the signed release manifest to carry, for every artifact,
/// *"a stable artifact identity, supported operating-system family, processor
/// architecture and ABI/capability tuple, byte length, SHA-256 digest, **fixed
/// entry-point name**, and exact HTTPS source or sources."*
///
/// **It is the name the artifact is placed under, the name the active-package
/// pointer carries, and the whole of the constant command string `Server protocol
/// and transfers` §1.1 sends after hand-over.** One name for all three, because
/// all three name the same path.
///
/// **It never changes with the package version.** G7 removes the pair problem by
/// making the boot registration start whatever the pointer names rather than a
/// version, and a name that moved with the version would put the version back into
/// the registration and into the command string.
pub const THE_FIXED_ENTRY_POINT_NAME: &str = "smoozy-server";

/// **The one target tuple this build is made for.**
///
/// **One rather than several, because every rented machine is a Linux machine** —
/// `State, identity and recovery` §1.3 rule 6 says it in as many words while
/// defining what a durable write is: *"On Linux — every rented machine — `fsync`
/// reaches the device."* Nothing Smoozy rents is a Mac or a Windows box.
///
/// **The processor and the ABI half.** Every machine on this project's record was
/// an x86-64 Ubuntu or Debian box, and `Server Setup` reads the distribution
/// family at every setup run and stops a run outside the supported pair — Ubuntu
/// and Debian, Creator-stated 2026-08-22. Both are GNU C library distributions, so
/// the tuple's ABI half is that library's.
///
/// **`Server protocol and transfers` B8 forbids anything but an exact match** —
/// *"no closest-architecture, closest-ABI or generic-Linux fallback is
/// permitted"* — and G28 repeats it from this side. A machine that does not match
/// is an unsupported machine reported before any package byte is executed.
///
/// **A manifest entry does not carry this string, and it never did.** B8 has an
/// artifact entry carry the operating-system family, the processor architecture
/// and the ABI as **three separate fields**, and this is one string in the
/// compiler's own spelling. **The two are the same tuple written two ways**, and
/// this constant is the machine side's spelling of it — the target this program
/// is built for, read by nothing but its own tests.
///
/// **The other spelling is
/// `smoozy_server_protocol::release::composing::THE_TUPLE_THIS_BUILD_IS_MADE_FOR`**,
/// and the two crates cannot name each other: the machine program was taken off
/// that crate's dependency on 2026-09-06 precisely so the SSH client and its C
/// cryptography stay out of its tree. **So the two are held together by a test
/// that reads the other crate's source**, which is the same shape
/// `the_entry_points_two_copies_agree` already uses next door for the entry-point
/// name.
pub const THE_TARGET_TUPLE: &str = "x86_64-unknown-linux-gnu";

/// **What the tuple does not settle, said here rather than discovered.**
///
/// A binary built against one release of the GNU C library is not thereby known to
/// run against an older one, and **nobody on this project has read that library's
/// own publication on the question.** It is not written around and it is not
/// claimed either way: what the manifest carries is one entry for one tuple, and
/// the entry set is `Server protocol and transfers` B8's Creator-ruling boundary.
/// **What removes it is that ruling and the manifest that carries it**, which is
/// the third of step 8a's three things.
pub const THE_LIBRARY_GENERATION_IS_NOT_SETTLED_BY_THE_TUPLE: bool = true;

/// **The three executable classes `The shape` says the package holds.**
///
/// *"Server Setup installs the core Smoozy server package containing three
/// executable classes or roles, plus a separately enabled Telegram role"* — and
/// the Telegram role's worker is started as a worker like any other, which is
/// §5.3's own rule, so it is the seventh kind of the worker class rather than a
/// fourth class here.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Role {
    /// **`The shape`'s class 2.** *"A short-lived command invoked through SSH
    /// authenticates by the SSH connection, frames one versioned request, passes
    /// it to the supervisor over a machine-local endpoint, and returns the
    /// correlated result."*
    ///
    /// **It is the bare invocation**, for `Server protocol and transfers` §1.1's
    /// reason above.
    TheSshCommandAdapter,
    /// **`The shape`'s class 1.** *"One long-lived service controls the
    /// machine-local queue, durable job state, event journal, worker identities,
    /// and local command endpoint."*
    ///
    /// **This is what the boot registration starts on both machine kinds** — the
    /// user unit on a systemd machine (K4), the keeper the container's hook starts
    /// (S5).
    TheServerSupervisor,
    /// **`The shape`'s class 3.** *"The supervisor starts render, file-inspection,
    /// setup, transfer-support, preview, and media workers as separate child
    /// processes."*
    AnOperationWorker(WorkerKind),
}

/// The word that selects the supervisor.
pub const THE_SUPERVISOR_WORD: &str = "supervise";

/// The word that selects a worker, which is followed by the kind's own word.
pub const THE_WORKER_WORD: &str = "work";

/// **Each worker kind's own word.**
///
/// **These seven words are Build's own choice and the plan fixes none of them** —
/// the same standing `Job files and outputs` L6's five area segment names have,
/// which that document says outright are Build's. What the plan fixes is the
/// **set**, which is W1's seven and is closed by `The shape` rather than by this
/// crate.
pub fn the_word_for(kind: WorkerKind) -> &'static str {
    match kind {
        WorkerKind::Render => "render",
        WorkerKind::FileInspection => "file-inspection",
        WorkerKind::Setup => "setup",
        WorkerKind::TransferSupport => "transfer-support",
        WorkerKind::Preview => "preview",
        WorkerKind::Media => "media",
        WorkerKind::Telegram => "telegram",
    }
}

/// The kind that word names, or nothing where no kind of W1's seven carries it.
pub fn the_kind_named(word: &str) -> Option<WorkerKind> {
    WorkerKind::ALL
        .into_iter()
        .find(|kind| the_word_for(*kind) == word)
}

/// **Which role an invocation selects.**
///
/// `operands` is the argument vector **after** the invoked path — argv without
/// its first element.
///
/// **An operand this program does not know is refused rather than read as the
/// adapter.** A bare invocation is the adapter because §1.1 sends exactly that;
/// an invocation carrying a word nothing here defines is a caller that meant
/// something, and answering it as the adapter would answer a request nobody made.
pub fn the_role_selected_by(operands: &[String]) -> Result<Role, String> {
    match operands {
        [] => Ok(Role::TheSshCommandAdapter),
        [word] if word == THE_SUPERVISOR_WORD => Ok(Role::TheServerSupervisor),
        [word, kind] if word == THE_WORKER_WORD => the_kind_named(kind)
            .map(Role::AnOperationWorker)
            .ok_or_else(|| {
                format!(
                    "{THE_FIXED_ENTRY_POINT_NAME}: there is no worker kind called {kind}. \
                     The kinds are: {}.",
                    the_worker_kind_words().join(", ")
                )
            }),
        [word] if word == THE_WORKER_WORD => Err(format!(
            "{THE_FIXED_ENTRY_POINT_NAME}: {THE_WORKER_WORD} names a worker kind after it. \
             The kinds are: {}.",
            the_worker_kind_words().join(", ")
        )),
        _ => Err(format!(
            "{THE_FIXED_ENTRY_POINT_NAME}: this program is entered with no operand for the SSH \
             command adapter, {THE_SUPERVISOR_WORD} for the server supervisor, or \
             {THE_WORKER_WORD} and a kind for an operation worker."
        )),
    }
}

/// The seven kinds' words, in W1's own order.
pub fn the_worker_kind_words() -> Vec<&'static str> {
    WorkerKind::ALL.into_iter().map(the_word_for).collect()
}

/// **The package area, derived from the path the entry point was invoked by.**
///
/// See this file's own opening for why the invoked path names it and why the
/// resolved executable does not. **A relative path is refused**, because all three
/// routes name an absolute one.
pub fn the_package_area_the_entry_point_was_invoked_from(
    invoked_path: &str,
) -> Result<String, String> {
    if !invoked_path.starts_with('/') {
        return Err(format!(
            "{THE_FIXED_ENTRY_POINT_NAME}: this program was invoked as {invoked_path}, which is \
             not an absolute path. The boot registration, the command channel's constant command \
             and the supervisor's own worker start all name one, so nothing that reaches this \
             program legitimately arrives this way, and the package area is not guessed at."
        ));
    }
    match invoked_path.rsplit_once('/') {
        Some(("", _)) => Ok("/".to_string()),
        Some((area, _)) => Ok(area.to_string()),
        None => Err(format!(
            "{THE_FIXED_ENTRY_POINT_NAME}: {invoked_path} names no directory to take the package \
             area from."
        )),
    }
}

/// **The records this plan owns inside the package area, by name.**
///
/// **These names are Build's own and the plan fixes none of them**, exactly as
/// `Job files and outputs` L6's five area segment names are Build's. What the plan
/// fixes is that each of these is *a record Smoozy wrote*, held **by value**, in
/// the package area — K16 for the registration, G1 and G2 for the phase, Z2 for
/// the endpoint's path, G10 for the installation identity, U2 for the hold
/// records and P1 for the setup-in-progress state — and that nothing derives any
/// of them from what is on the machine at the moment it is read.
///
/// **Only this plan's own records are named here.** The machine-local queue, the
/// operation and command records, the event journal, the per-job note, the
/// captured streams and the bot token file also sit in the package area and are
/// named by the documents that own them, which `Job files and outputs` L6 lists.
pub mod records {
    /// **Z2.** The endpoint's path, *"fixed for the installation, does not vary
    /// with the package version, and is recorded in the package area at
    /// placement"*. **The record holds the path; the object at it is volatile and
    /// is not this.**
    pub const THE_ENDPOINTS_RECORDED_PATH: &str = "endpoint-path";
    /// **G1 and G2.** One phase record per machine.
    pub const THE_PHASE_RECORD: &str = "package-phase";
    /// **K16.** The boot registration's content, by value.
    pub const THE_BOOT_REGISTRATION_RECORD: &str = "boot-registration";
    /// **G10.** The installation identity, written and made durable before the
    /// pointer switch.
    pub const THE_INSTALLATION_IDENTITY: &str = "installation";
    /// **G1b.** The one package-transaction claim, whose exclusive create is the
    /// claim where no machine-side program exists to hold G1a's lock.
    pub const THE_PACKAGE_TRANSACTION_CLAIM: &str = "package-transaction";
    /// **U2.** Where the hold records sit — *"a durable store of this
    /// document's, in the package area, one per hold"*. **An area rather than a
    /// record**, because there is one per hold and each is reached by its own
    /// identity.
    pub const THE_AREA_THE_HOLD_RECORDS_SIT_IN: &str = "hold-records";
    /// **P1.** The one machine-level setup-in-progress state.
    pub const THE_SETUP_IN_PROGRESS_STATE: &str = "setup-in-progress";
    /// **W7.** The one role worker's machine-level enabled-or-disabled desired
    /// state. **Its absence is *disabled***, so a machine `Server Setup`'s
    /// second entry point has never reached holds no role.
    pub const THE_TELEGRAM_ROLE_STATE: &str = "telegram-role";
    /// **The Telegram worker's state store** — `State, identity and recovery`
    /// §1.2's row of that name, whose content `Telegram` owns and whose one
    /// writer is the supervisor, at the worker's request. **It is named here
    /// beside the role's state rather than by its owning document**, because
    /// the supervisor is its one writer and this is where the supervisor's
    /// records in the package area are named. **Its absence is *nothing
    /// recorded yet***: a machine whose worker has never asked for a write
    /// holds none. Added 2026-10-16, the Telegram worker's first piece.
    pub const THE_TELEGRAM_WORKERS_STATE: &str = "telegram-worker-state";
    /// **G15a's store version** — `State, identity and recovery` §1.2's last
    /// row, one number for every store in the state area but the package
    /// record and the installation identity. **That document owns its
    /// content and this names where it sits**, beside the records above,
    /// because the supervisor is its one writer. **Its absence is the first
    /// version** (that document's §1.3 rule 7).
    pub const THE_STORE_VERSION: &str = "store-version";
}

/// One of this plan's own records, at its path under a package area.
pub fn the_record_at(package_area: &str, record: &str) -> String {
    format!("{package_area}/{record}")
}

/// The active-package pointer's own path under a package area.
///
/// **It carries the fixed entry-point name**, which is what makes the directory
/// holding an invocation the package area.
pub fn the_active_package_pointer_at(package_area: &str) -> String {
    the_record_at(package_area, THE_FIXED_ENTRY_POINT_NAME)
}

/// **Z3's containing directory**, under the package area.
///
/// Z3: *"Its containing directory is created by the placement, owned by the
/// login the supervisor runs as, and permits that login alone."* The plan names
/// no place for it; **the package area is the one place already fixed for the
/// installation and owned by that login**, which is what Z2's *fixed for the
/// installation, does not vary with the package version* needs.
pub const THE_ENDPOINTS_OWN_DIRECTORY: &str = "endpoint";

/// The object's own name inside [`THE_ENDPOINTS_OWN_DIRECTORY`].
pub const THE_ENDPOINTS_OWN_NAME: &str = "supervisor";

/// **Z2's path, as the placement records it: `<package area>/endpoint/supervisor`.**
///
/// **Nothing wrote this record until step 10c piece 12's first repair**, and a
/// supervisor that finds none refuses to start (`the_endpoints_recorded_path`
/// in the program): every test that ran the real program wrote the record by
/// hand, and nothing that placed the program ever did. **So a machine the
/// desktop placed could not have started its supervisor even with the program
/// on it.**
///
/// **A filesystem-named socket has a length limit** — the operating system's,
/// and the placement does not measure it. The package area on a rental is
/// short (`/home/<login>/smoozy/package` on the one watched), and a package
/// area long enough to exceed it is a supervisor that fails its bind loudly
/// rather than a silent one.
pub fn the_endpoints_path_under(package_area: &str) -> String {
    format!(
        "{}/{THE_ENDPOINTS_OWN_NAME}",
        the_record_at(package_area, THE_ENDPOINTS_OWN_DIRECTORY)
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_bare_invocation_is_the_ssh_command_adapter() {
        assert_eq!(
            the_role_selected_by(&[]),
            Ok(Role::TheSshCommandAdapter),
            "`Server protocol and transfers` §1.1 sends the entry point's own path and nothing \
             after it, and says that entry carries the SSH command adapter"
        );
    }

    #[test]
    fn the_supervisor_and_every_worker_kind_are_reachable() {
        assert_eq!(
            the_role_selected_by(&[THE_SUPERVISOR_WORD.to_string()]),
            Ok(Role::TheServerSupervisor)
        );
        for kind in WorkerKind::ALL {
            assert_eq!(
                the_role_selected_by(&[
                    THE_WORKER_WORD.to_string(),
                    the_word_for(kind).to_string()
                ]),
                Ok(Role::AnOperationWorker(kind)),
                "every one of W1's seven kinds is reachable"
            );
        }
    }

    /// **The seven words are seven**, so no two kinds answer to one word and no
    /// kind is unreachable.
    #[test]
    fn no_two_kinds_share_a_word() {
        let mut words = the_worker_kind_words();
        words.sort_unstable();
        let how_many = words.len();
        words.dedup();
        assert_eq!(words.len(), how_many);
        assert_eq!(how_many, WorkerKind::ALL.len());
    }

    #[test]
    fn an_operand_this_program_does_not_know_is_refused_rather_than_read_as_the_adapter() {
        for operands in [
            vec!["supervisor".to_string()],
            vec![THE_WORKER_WORD.to_string()],
            vec![THE_WORKER_WORD.to_string(), "renderer".to_string()],
            vec![THE_SUPERVISOR_WORD.to_string(), "again".to_string()],
        ] {
            let refused = the_role_selected_by(&operands)
                .expect_err("an operand this program does not know is refused");
            assert!(refused.contains(THE_FIXED_ENTRY_POINT_NAME));
        }
    }

    #[test]
    fn the_package_area_is_the_directory_the_entry_point_was_invoked_from() {
        assert_eq!(
            the_package_area_the_entry_point_was_invoked_from(&format!(
                "/ephemeral/smoozy/package/{THE_FIXED_ENTRY_POINT_NAME}"
            )),
            Ok("/ephemeral/smoozy/package".to_string())
        );
        assert_eq!(
            the_active_package_pointer_at("/ephemeral/smoozy/package"),
            format!("/ephemeral/smoozy/package/{THE_FIXED_ENTRY_POINT_NAME}")
        );
    }

    /// **A relative invoked path is refused rather than resolved.**
    #[test]
    fn a_relative_invocation_is_refused() {
        let refused = the_package_area_the_entry_point_was_invoked_from(THE_FIXED_ENTRY_POINT_NAME)
            .expect_err("a relative path is refused");
        assert!(refused.contains("not an absolute path"));
    }

    /// **The tuple is one and it is exact.** `Server protocol and transfers` B8
    /// forbids a generic-Linux entry, so a tuple that named a family rather than a
    /// triple would be the fallback that clause refuses.
    #[test]
    fn the_target_tuple_is_an_exact_triple_on_linux() {
        let parts: Vec<&str> = THE_TARGET_TUPLE.split('-').collect();
        assert_eq!(parts.len(), 4, "an exact tuple rather than a family");
        assert!(THE_TARGET_TUPLE.contains("linux"));
    }

    /// **The tuple's two spellings are the same tuple**, and the guard is a
    /// reading of the other crate's source because the two crates cannot name
    /// each other.
    ///
    /// **This is the same shape the entry-point name already has**, in that
    /// crate's own `composing.rs`, and it is why this constant is kept rather
    /// than deleted: it is read by nothing else, and what was wrong was the
    /// sentence beside it rather than the constant.
    #[test]
    fn the_tuples_two_spellings_are_the_same_tuple() {
        let beside = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../Server protocol and transfers/src/release/composing.rs");
        let text = std::fs::read_to_string(&beside)
            .expect("the release side's own file is readable");
        let line = text
            .lines()
            .find(|line| line.contains("pub const THE_TUPLE_THIS_BUILD_IS_MADE_FOR"))
            .expect("the release side still names its tuple constant");
        let named: Vec<&str> = line.split('"').skip(1).step_by(2).collect();
        assert_eq!(
            named.len(),
            3,
            "the release side's tuple no longer reads as three quoted parts: {line}"
        );

        let parts: Vec<&str> = THE_TARGET_TUPLE.split('-').collect();
        for part in &named {
            assert!(
                parts.contains(part),
                "the release side names {part:?} and {THE_TARGET_TUPLE} does not carry it, so the \
                 two spellings of one tuple have drifted"
            );
        }
    }
}
