//! §5 — the bootstrap-and-repair request, and package inspection.
//!
//! **What this is, and why it was missing.** `The shape` puts one narrow,
//! versioned bootstrap-and-repair request on the pre-package surface, and this
//! document owns its framing, its route, its idempotency, its inspection and its
//! results. Until this landed **the request existed nowhere in the workspace**:
//! [`smoozy_framing::framing::Request::for_the_bootstrap_and_repair_route`]
//! composed the frame for the route and nothing sent one, and
//! `App/Server Setup/src/arriving_later.rs` filed *"the package inspection, the
//! bootstrap-and-repair request and the plain start of the registered service"*
//! as left to this document's adapter. **Both halves of step 6 recorded the
//! request as the other's, and step 6 closed.**
//!
//! # What is here, clause by clause
//!
//! - **B1** — [`TheBootstrapAndRepairRequest`]: one request, one identity that
//!   is both the correlation identity and the idempotency key, one verified
//!   release descriptor, and no variant, sub-command or mode flag anywhere.
//! - **B2** — [`WhereTheRequestRides`] and every function below: the surface's
//!   two mechanisms and never the supervisor's machine-local endpoint.
//! - **B3** — [`what_the_phase_record_says_about`]: idempotency answered from
//!   the durable package-phase record, and from nothing else.
//! - **B4** — [`inspect`] and [`compose_the_inspection`]: a composed
//!   file-transfer-subsystem reading of the installation's own durable records.
//! - **B5** — [`inspect`] returns a [`Reading`], and [`ARememberedInspection`]
//!   is the only shape in which the answer is ever kept.
//! - **B6** — [`WhatTheRequestAnswered::TheUpgradeIsDeferred`]: carried, and not
//!   interpreted.
//! - **B7** — [`NOTHING_HERE_SENDS_THIS_REQUEST_OF_ITS_OWN_ACCORD`].
//!
//! # What is not here, and it is one thing
//!
//! **Nothing here performs the transaction's machine-side steps.** B2 divides
//! the two performers and this module is neither: what it owns is *"the request
//! they both send"* (B7) — its shape, its route, its idempotency question, the
//! reading it takes and how its results are read. **The staging, the commit, the
//! rollback and the boot registration are `Rented-server runtime`'s
//! transaction**, whose own half runs on the machine where a verified runtime is
//! alive and through the surface where none is, and [`crate::transfers`] and
//! [`crate::release`] are the two mechanisms it moves on.
//!
//! # Why the machine's own record types are read rather than copied
//!
//! **B4's values are the installation's durable records**, and those records
//! are `Rented-server runtime`'s: the phase record is that document's G2's, the
//! boot-registration record K16's, the installation identity G10's, and their
//! names are that crate's own `the_program::records`. **The desktop composes the
//! answer** — which is what this file does — **and no machine-side program
//! answers it**, which is why nothing here asks a supervisor anything. Reading
//! them through that crate's own types is what keeps one home for the form; a
//! second copy of it here would drift, and a desktop that misread a machine's
//! package version would do it in silence.
//!
//! **Markers used in this file that belong to another document, each with the
//! document it belongs to.** `Rented-server runtime` G10, G17, G26, G27,
//! G31, G31a, K16. `State, identity and recovery` D4, D5, §2.3, §3.
//!
//! **G2 is deliberately not on that list**: this plan defines a G2 of its own,
//! so the list could not tell the two apart, and the one use of it above writes
//! out whose it is.
//!
//! **A marker this plan also defines is never listed there and is written out
//! with its document at the site**, because a header list cannot tell the two
//! apart and a bare one there would resolve to a real and wrong clause.

use std::time::SystemTime;

use smoozy_framing::framing::{Fields, Request};
use smoozy_runtime::package::phase::{which_descriptor_the_pointer_names, Phase, PhaseRecord};
use smoozy_runtime::package::release::{RetainedAdapter, VerifiedReleaseDescriptor};
use smoozy_runtime::registration::BootRegistrationRecord;
use smoozy_runtime::the_program::{records, the_active_package_pointer_at, the_record_at};
use smoozy_runtime::written_form;
use smoozy_state::identities::Identity;
use smoozy_state::lifetimes::Lifetime;
use smoozy_state::observed::{Clock, Reading};

use crate::reconnection::{read_the_installation_identity, WhatTheStateAreaHeld};
use crate::release::verifying::{AVerifiedRelease, TheDescriptorsValues};
use crate::subsystem::client::Subsystem;
use crate::subsystem::{StatusCode, SubsystemFailure};

/// **The request's own name on the wire.**
///
/// **One name, because B1 makes this one request**: *"place it, repair it, or
/// replace it is one request whose answer depends on what the machine is found
/// holding"*. There is no second name, no sub-command word and no mode field —
/// see [`NO_VARIANT_SUB_COMMAND_OR_MODE_FLAG_EXISTS`].
pub const THE_BOOTSTRAP_AND_REPAIR_REQUEST: &str = "bootstrap-and-repair";

/// The field a framed bootstrap-and-repair request carries its descriptor's
/// manifest format in.
pub const THE_MANIFEST_FORMAT_FIELD: &str = "manifest-format";
/// The verified release sequence.
pub const THE_RELEASE_SEQUENCE_FIELD: &str = "release-sequence";
/// The requested package version.
pub const THE_REQUESTED_PACKAGE_VERSION_FIELD: &str = "package-version";
/// The manifest's canonical digest.
pub const THE_MANIFEST_DIGEST_FIELD: &str = "manifest-digest";
/// The selected artifact's stable identity.
pub const THE_ARTIFACT_IDENTITY_FIELD: &str = "artifact-identity";
/// The selected artifact's byte length.
pub const THE_ARTIFACT_BYTE_LENGTH_FIELD: &str = "artifact-byte-length";
/// The selected artifact's SHA-256 digest.
pub const THE_ARTIFACT_DIGEST_FIELD: &str = "artifact-digest";

/// **B1's own prohibition, kept by there being nothing to write it with.**
///
/// *"It has no variants, no sub-commands and no mode flag."* There is one
/// request name above, one type below, and no field anywhere in this module that
/// selects between two behaviours. A caller cannot ask for *place* rather than
/// *repair*, because there is nowhere to say it: **the answer depends on what
/// the machine is found holding, and the finding is `Rented-server runtime`'s.**
pub const NO_VARIANT_SUB_COMMAND_OR_MODE_FLAG_EXISTS: () = ();

/// **B1. The request is one request, and its narrowness is the point.**
///
/// It carries the protocol version (P2), the correlation identity, the
/// idempotency key — **which is that same identity, per C1** — and one verified
/// release descriptor.
///
/// **`No request can select bytes by a version string alone`, kept by the
/// constructor.** [`TheBootstrapAndRepairRequest::for_this_verified_release`]
/// takes an [`AVerifiedRelease`], which is only ever produced by
/// [`crate::release::verifying::verify_before_download`] — so a caller holding a
/// version string and nothing else cannot compose one of these at all.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TheBootstrapAndRepairRequest {
    identity: Identity,
    descriptor: TheDescriptorsValues,
}

impl TheBootstrapAndRepairRequest {
    /// **The one constructor**, and it takes a verified release rather than a
    /// version.
    ///
    /// The identity is the caller's, chosen before the request is sent (C1), and
    /// **nothing here issues one** — which is also half of what keeps B6's
    /// *does not license a second attempt under a fresh identity*.
    pub fn for_this_verified_release(
        identity: Identity,
        verified: &AVerifiedRelease,
    ) -> TheBootstrapAndRepairRequest {
        TheBootstrapAndRepairRequest {
            identity,
            descriptor: verified.the_release_descriptors_values(),
        }
    }

    /// **The correlation identity.**
    pub fn identity(&self) -> &Identity {
        &self.identity
    }

    /// **The idempotency key, which is that same operation identity and not a
    /// second value beside it** (B1, C1).
    pub fn the_idempotency_key(&self) -> &Identity {
        &self.identity
    }

    /// The verified release descriptor this request carries.
    pub fn descriptor(&self) -> &TheDescriptorsValues {
        &self.descriptor
    }

    /// **P4. The frame for the route, composed by this document rather than by
    /// the adapter it may be replacing.**
    ///
    /// That is why it is framed here at all: **a package too old to be spoken to
    /// can still be replaced.** The constructor it goes through takes no version
    /// and can only ever speak
    /// [`smoozy_framing::framing::THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS`].
    pub fn framed(&self) -> Request {
        Request::for_the_bootstrap_and_repair_route(
            self.identity.clone(),
            THE_BOOTSTRAP_AND_REPAIR_REQUEST,
        )
        .carrying(
            Fields::none()
                .and(
                    THE_MANIFEST_FORMAT_FIELD,
                    self.descriptor.manifest_format.as_bytes().to_vec(),
                )
                .and(
                    THE_RELEASE_SEQUENCE_FIELD,
                    self.descriptor.verified_release_sequence.to_string(),
                )
                .and(
                    THE_REQUESTED_PACKAGE_VERSION_FIELD,
                    self.descriptor.requested_package_version.as_bytes().to_vec(),
                )
                .and(
                    THE_MANIFEST_DIGEST_FIELD,
                    self.descriptor.manifest_digest.to_vec(),
                )
                .and(
                    THE_ARTIFACT_IDENTITY_FIELD,
                    self.descriptor.artifact_identity.as_bytes().to_vec(),
                )
                .and(
                    THE_ARTIFACT_BYTE_LENGTH_FIELD,
                    self.descriptor.artifact_byte_length.to_string(),
                )
                .and(
                    THE_ARTIFACT_DIGEST_FIELD,
                    self.descriptor.artifact_digest.to_vec(),
                ),
        )
    }
}

/// **B2. Which of §1.1's two mechanisms a step of this request rides on.**
///
/// *"It rides on the pre-package surface and never on the machine-local
/// endpoint, and no Smoozy program receives it on the machine."* The set is
/// closed by §1.1: the surface *"is made of exactly two mechanisms"*, and
/// **there is no third variant here for the endpoint** — which is the whole of
/// how this clause is kept. A reader looking for the endpoint on this route
/// finds it named nowhere in this crate.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhereTheRequestRides {
    /// **Artifact bytes and durable records move as subsystem protocol fields**,
    /// under §7's contract. Package inspection is this, in both phases (B4).
    TheFileTransferSubsystem,
    /// **The command channel carries only §1.1-admissible commands** — the boot
    /// registration and `Server Setup`'s elevated steps are the ones this
    /// transaction needs, and each is constant or carries its value by the
    /// rule's own routes.
    TheCommandChannel,
}

impl WhereTheRequestRides {
    /// Both of them, and there is no third.
    pub const BOTH: [WhereTheRequestRides; 2] = [
        WhereTheRequestRides::TheFileTransferSubsystem,
        WhereTheRequestRides::TheCommandChannel,
    ];
}

/// **B2's two performers.**
///
/// *"Where a verified runtime is alive, `Rented-server runtime` may perform its
/// own half of the transaction; where none is, the desktop performs it through
/// the surface, and the transaction's shape is that document's either way."*
///
/// **So the request reaches a machine whose supervisor is dead, whose package is
/// half-placed, or that has never had a package at all — all three by the same
/// route.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhoPerformsTheTransaction {
    /// A verified runtime is alive and may perform its own half. It takes the
    /// claim file and then `Rented-server runtime` G1a's kernel-held lock.
    ALiveVerifiedRuntime,
    /// **Every bare machine, and every machine whose supervisor is dead.** The
    /// desktop performs the transaction through the surface's two mechanisms.
    TheDesktopThroughTheSurface,
}

/// **B2.** Which performer a machine gets, and it turns on one thing.
pub fn who_performs_the_transaction(
    a_verified_runtime_is_alive: bool,
) -> WhoPerformsTheTransaction {
    if a_verified_runtime_is_alive {
        WhoPerformsTheTransaction::ALiveVerifiedRuntime
    } else {
        WhoPerformsTheTransaction::TheDesktopThroughTheSurface
    }
}

/// **B3's answer: what the durable package-phase record says about this
/// request's identity.**
///
/// **B3 is exact about which record answers and why.** *"This is the one request
/// in this document whose durable record is not the supervisor's, and the reason
/// is that the supervisor may not exist when it arrives … Its idempotency is
/// answered from the durable package-phase record, which `Rented-server runtime`
/// owns."*
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WhatThisIdentityMeans {
    /// **No record of this identity.** No phase record at all, or one carrying
    /// another attempt. C4's *record it durably before acting* has not happened
    /// for this request.
    NoRecordOfThisIdentity,
    /// **This identity is already recorded, and a request repeating a recorded
    /// identity never acts again** (C4). The phase the record carries says where
    /// the attempt reached, and what a later reading of it means about the
    /// machine is `Rented-server runtime` G3's own third column.
    ItIsAlreadyRecorded { at: Phase },
}

/// **B3.** The idempotency answer, from the phase record and from nothing else.
///
/// `the_phase_record` is [`None`] where the machine has no phase record at all,
/// which is every machine that has never held a package.
pub fn what_the_phase_record_says_about(
    request: &TheBootstrapAndRepairRequest,
    the_phase_record: Option<&PhaseRecord>,
) -> WhatThisIdentityMeans {
    match the_phase_record {
        Some(record) if &record.attempt == request.the_idempotency_key() => {
            WhatThisIdentityMeans::ItIsAlreadyRecorded { at: record.phase }
        }
        _ => WhatThisIdentityMeans::NoRecordOfThisIdentity,
    }
}

/// **B3's last sentence, kept by there being no route to one.**
///
/// *"A builder must not look for an operation record for this request."*
/// [`what_the_phase_record_says_about`] takes a phase record and takes nothing
/// else; **there is no parameter, field or call anywhere in this module for an
/// operation record**, so the wrong record cannot be reached from here even by a
/// caller that wanted to.
pub const NO_OPERATION_RECORD_IS_LOOKED_FOR_FOR_THIS_REQUEST: () = ();

/// **What a durable record on the machine held**, and the three answers are not
/// two.
///
/// **A record that is there and does not read back is never folded into
/// absence.** Absent says nothing of Smoozy's is here; a record that will not
/// read back says something is here and Smoozy cannot say what — which is B4's
/// *present but not Smoozy's* and C14's *present and different* branch, not a
/// healthy machine. This is the same three-way shape
/// [`crate::reconnection::WhatTheStateAreaHeld`] already takes for the
/// installation identity, and for the same reason.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WhatARecordHeld<T> {
    /// The record read back as what it is.
    It(T),
    /// **No record at all**, which on a machine that has never held a package is
    /// the honest answer rather than a zero.
    Nothing,
    /// **The record is there and does not read back**, with what the reader
    /// said.
    ARecordThatDoesNotReadBack(String),
}

impl<T> WhatARecordHeld<T> {
    /// Whether this record is absent.
    pub fn is_nothing(&self) -> bool {
        matches!(self, WhatARecordHeld::Nothing)
    }

    /// **Whether this record is present and is not Smoozy's** — B4's own branch.
    pub fn is_present_and_not_smoozys(&self) -> bool {
        matches!(self, WhatARecordHeld::ARecordThatDoesNotReadBack(_))
    }
}

/// **B4. What package inspection returns.**
///
/// *"So inspection returns the active-package pointer's release sequence,
/// package version, manifest digest and artifact digest; the boot-registration
/// record's own content; the phase record's phase and retained-adapter
/// intervals; the installation identity; and each adapter's declared protocol
/// interval and request set."*
///
/// **Every value is a record's content and never a path's presence.** The
/// recycle observation forbids the other reading in the plainest possible terms:
/// the boot hook *"was not deleted but replaced"* by an 82-byte provider default
/// dated at the recycle moment, so *"a check for 'does my hook exist' passes
/// while the hook does nothing Smoozy asked for"*.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AnInspection {
    /// **The active-package pointer's descriptor** — its release sequence,
    /// package version, manifest digest and artifact digest, which is
    /// `Rented-server runtime` G27's descriptor read out of the record that pins
    /// it by value.
    ///
    /// **Which of the phase record's two descriptors this is** is G31a's own
    /// rule, applied to the pointer's own value; a pointer naming neither
    /// recorded location claims no descriptor at all.
    pub the_active_packages_descriptor: Option<VerifiedReleaseDescriptor>,
    /// **The boot-registration record's own content** (`Rented-server runtime`
    /// K16), by value.
    pub the_boot_registration_record: WhatARecordHeld<BootRegistrationRecord>,
    /// **The phase record's phase.**
    pub the_phase: WhatARecordHeld<Phase>,
    /// **Each retained adapter's declared protocol interval and request set**,
    /// as `Rented-server runtime` G31a writes them into the phase record by
    /// value. **Inspection reads them as content and never asks a program**,
    /// which is what makes §1.2 item 3's declaration answerable beside a dead
    /// supervisor.
    pub the_retained_adapters: Vec<RetainedAdapter>,
    /// **The installation identity** (`Rented-server runtime` G10), read by
    /// C15's own reader.
    pub the_installation_identity: WhatTheStateAreaHeld,
}

impl AnInspection {
    /// **B4's never-held answer: every record absent.**
    ///
    /// *"That is what lets inspection answer on a machine that has never held a
    /// package — every record absent, reported honestly as that."*
    pub fn every_record_is_absent(&self) -> bool {
        self.the_active_packages_descriptor.is_none()
            && self.the_boot_registration_record.is_nothing()
            && self.the_phase.is_nothing()
            && self.the_retained_adapters.is_empty()
            && self.the_installation_identity == WhatTheStateAreaHeld::Nothing
    }

    /// **B4's last sentence: a reading in which any of those is present but not
    /// Smoozy's is C14's *present and different* branch, not a healthy
    /// machine.**
    pub fn a_record_is_present_and_not_smoozys(&self) -> bool {
        self.the_boot_registration_record.is_present_and_not_smoozys()
            || self.the_phase.is_present_and_not_smoozys()
            || self.the_installation_identity
                == WhatTheStateAreaHeld::ARecordThatDoesNotReadBack
    }

    /// **§1.2 item 3's declaration, which is inspection's to answer.**
    ///
    /// *"The command adapter's declared request set for the installed package
    /// version, returned by package inspection."* It is the union of what the
    /// retained adapters declare, taken from the record rather than from a
    /// running adapter's own answer.
    pub fn the_declared_request_set(&self) -> Vec<String> {
        let mut out: Vec<String> = Vec::new();
        for adapter in &self.the_retained_adapters {
            for request in &adapter.declared_request_set {
                if !out.contains(request) {
                    out.push(request.clone());
                }
            }
        }
        out
    }
}

/// **What the four reads over the subsystem brought back**, before anything is
/// composed from them.
///
/// **The composition is split from the reading on purpose**: B4's rule is about
/// what may be concluded from the records, and a rule that can only be exercised
/// against a machine is a rule nothing checks at the desk.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WhatTheRecordsHeld {
    pub phase_record: WhatARecordHeld<PhaseRecord>,
    pub boot_registration: WhatARecordHeld<BootRegistrationRecord>,
    pub installation_identity: WhatTheStateAreaHeld,
    /// **The active-package pointer's own value**, or [`None`] where there is no
    /// pointer. It is the link's own target and never a resolved path — see
    /// [`crate::subsystem::wire::kind::READLINK`].
    pub the_pointer_points_at: Option<String>,
}

/// **B4.** The answer, composed from the records and from nothing else.
pub fn compose_the_inspection(held: WhatTheRecordsHeld) -> AnInspection {
    let (the_phase, the_active_packages_descriptor, the_retained_adapters) = match &held.phase_record
    {
        WhatARecordHeld::It(record) => {
            let mut retained = record.staged_packages_retained_adapters.clone();
            if let Some(previous) = &record.previous {
                retained.push(previous.retained_adapter.clone());
            }
            (
                WhatARecordHeld::It(record.phase),
                which_descriptor_the_pointer_names(record, held.the_pointer_points_at.as_deref()),
                retained,
            )
        }
        WhatARecordHeld::Nothing => (WhatARecordHeld::Nothing, None, Vec::new()),
        WhatARecordHeld::ARecordThatDoesNotReadBack(why) => (
            WhatARecordHeld::ARecordThatDoesNotReadBack(why.clone()),
            None,
            Vec::new(),
        ),
    };
    AnInspection {
        the_active_packages_descriptor,
        the_boot_registration_record: held.boot_registration,
        the_phase,
        the_retained_adapters,
        the_installation_identity: held.installation_identity,
    }
}

/// **How many bytes of one of these records is read.**
///
/// **The read is bounded rather than open**, for the reason
/// [`crate::reconnection::THE_MOST_THE_RECORD_MAY_HOLD`] gives and
/// [`smoozy_framing::framing::THE_LARGEST_FRAME`] gives before it: a length
/// taken off a far end is a number the far end chose. **A record longer than
/// this reads back as one that does not read back** rather than being truncated
/// into something that parses.
pub const THE_MOST_ONE_OF_THESE_RECORDS_MAY_HOLD: usize = 64 * 1024;

/// **B4. Package inspection: a composed file-transfer-subsystem reading of the
/// installation's own durable records.**
///
/// **The paths are protocol fields**, which is §1.1's first carriage route, so a
/// leading dash, whitespace, a newline and a non-UTF-8 name are values rather
/// than syntax. **The desktop composes the answer and no machine-side program
/// answers it**, so nothing here runs a command, asks a supervisor, or needs one
/// to be alive.
///
/// **The caller hands in the package area and never a path.** Every record's own
/// path is composed here from that area and from `Rented-server runtime`'s own
/// record names, which is what keeps a provider-written path from reaching this
/// reading at all — the same rule C15 states for the installation identity.
///
/// **B5.** The answer comes back as a [`Reading`], carrying the moment it was
/// taken and the clock it was taken on. **The clock is the desktop's own**,
/// because the desktop is what composed the answer.
pub async fn inspect<S>(
    subsystem: &mut Subsystem<S>,
    the_package_area: &[u8],
) -> Result<Reading<AnInspection>, SubsystemFailure>
where
    S: tokio::io::AsyncRead + tokio::io::AsyncWrite + Unpin + Send,
{
    let area = the_package_area_as_text(the_package_area);

    let phase_record = read_a_record(
        subsystem,
        &the_record_at(&area, records::THE_PHASE_RECORD),
        written_form::read::<PhaseRecord>,
    )
    .await?;
    let boot_registration = read_a_record(
        subsystem,
        &the_record_at(&area, records::THE_BOOT_REGISTRATION_RECORD),
        written_form::read::<BootRegistrationRecord>,
    )
    .await?;
    let installation_identity =
        read_the_installation_identity(subsystem, the_package_area).await?;
    let the_pointer_points_at =
        match subsystem.read_link(the_active_package_pointer_at(&area).as_bytes()).await {
            Ok(target) => Some(String::from_utf8_lossy(&target).into_owned()),
            // **A pointer that is not there is a machine with no active
            // package**, which is the never-held answer and not a failure. A
            // path that is there and is not a link is the server's own refusal
            // and is returned as one.
            Err(SubsystemFailure::TheServerRefused(StatusCode::NoSuchFile)) => None,
            Err(why) => return Err(why),
        };

    Ok(Reading::taken(
        compose_the_inspection(WhatTheRecordsHeld {
            phase_record,
            boot_registration,
            installation_identity,
            the_pointer_points_at,
        }),
        SystemTime::now(),
        Clock::ThisComputer,
    ))
}

/// The package area as text, for composing the records' own paths.
///
/// **A trailing separator is taken off** so that one area written two ways gives
/// one set of paths.
fn the_package_area_as_text(area: &[u8]) -> String {
    let text = String::from_utf8_lossy(area).into_owned();
    text.strip_suffix('/').map(str::to_string).unwrap_or(text)
}

/// One durable record, read over the subsystem and read back through
/// `Rented-server runtime`'s own written form.
///
/// **The reader is handed in rather than reached for through a trait bound**, so
/// this crate names that crate's written form and nothing beneath it.
async fn read_a_record<T, S>(
    subsystem: &mut Subsystem<S>,
    at: &str,
    read_it_back: fn(&[u8]) -> Result<T, written_form::TheRecordCouldNotBeRead>,
) -> Result<WhatARecordHeld<T>, SubsystemFailure>
where
    S: tokio::io::AsyncRead + tokio::io::AsyncWrite + Unpin + Send,
{
    let handle = match subsystem.open_to_read(at.as_bytes()).await {
        Ok(handle) => handle,
        Err(SubsystemFailure::TheServerRefused(StatusCode::NoSuchFile)) => {
            return Ok(WhatARecordHeld::Nothing)
        }
        Err(why) => return Err(why),
    };
    let read = subsystem
        .read_at(&handle, 0, THE_MOST_ONE_OF_THESE_RECORDS_MAY_HOLD)
        .await;
    subsystem.close(&handle).await?;
    let bytes = read?;
    Ok(the_record_in(&bytes, read_it_back))
}

/// **What a record's bytes read back as**, split out so the reading rule is
/// testable without a machine.
pub fn the_record_in<T>(
    bytes: &[u8],
    read_it_back: fn(&[u8]) -> Result<T, written_form::TheRecordCouldNotBeRead>,
) -> WhatARecordHeld<T> {
    if bytes.is_empty() {
        // **An empty record is not an absent one.** The record was opened, so
        // something is there; nothing readable came back.
        return WhatARecordHeld::ARecordThatDoesNotReadBack(
            "the record is there and holds no bytes".to_string(),
        );
    }
    match read_it_back(bytes) {
        Ok(record) => WhatARecordHeld::It(record),
        Err(why) => WhatARecordHeld::ARecordThatDoesNotReadBack(why.what_the_reader_said),
    }
}

/// **B5. Inspection is a reading and its answer is never durable state on the
/// desktop side.**
///
/// `State, identity and recovery` §3's D4 and D5 bind it. **D4** keeps the
/// desktop from holding a durable copy of an observed machine state the machine
/// still holds; **D5** puts the moment and the clock on every observed reading,
/// which [`inspect`]'s [`Reading`] carries and which nothing here can drop.
///
/// **Where it is remembered at all it is remembered as this**: with the
/// installation identity it was read under (`State, identity and recovery`
/// §2.3), **subject-held on the server row**.
///
/// **Nothing in this crate writes one down.** There is no store here, no write
/// and no caller; what this type is, is the shape the one exception D4 names has
/// to take if the remembered server state ever carries it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ARememberedInspection {
    /// The reading, with its moment and its clock.
    pub reading: Reading<AnInspection>,
    /// **The installation identity it was read under.** Without it a remembered
    /// reading survives a recycle and describes a machine that is gone.
    pub read_under: Identity,
}

impl ARememberedInspection {
    /// **Subject-held on the server row** — B5's own word, and
    /// `State, identity and recovery` §11.1's class. It ends when the server row
    /// ends and no sweep reaches it.
    pub const LIFETIME: Lifetime = Lifetime::SubjectHeld;

    /// **D4, kept by there being nowhere to put one.** This crate holds no
    /// durable store for a reading and writes none; the server row is the local
    /// record's, which is `Desktop app and delivery`'s.
    pub const NOTHING_IN_THIS_CRATE_WRITES_ONE_DOWN: () = ();
}

/// **What the bootstrap-and-repair request answered.**
///
/// **The phase is `Rented-server runtime` G3's value, carried rather than
/// re-decided** — this document owns the request and that one owns everything
/// the machine does in response.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WhatTheRequestAnswered {
    /// The phase the transaction reached.
    ThePhaseTheTransactionReached(Phase),
    /// **B6. The commit is blocked while live or held work depends on the
    /// runtime, and the caller is told that the upgrade is deferred for that
    /// reason.**
    ///
    /// **It is one of the request's ordinary results**: it is not a failure, it
    /// does not expire a waiter, and it does not license a second attempt under
    /// a fresh identity.
    TheUpgradeIsDeferred {
        /// **The machine's own words, carried and not interpreted here.**
        /// Nothing in this module parses, matches on or classifies them, and
        /// there is no function that turns them into a cause.
        the_reason_the_machine_gave: Option<String>,
    },
}

/// **B6.** What the phase record says the request's result is.
pub fn what_the_result_is(record: &PhaseRecord) -> WhatTheRequestAnswered {
    match record.phase {
        Phase::Deferred => WhatTheRequestAnswered::TheUpgradeIsDeferred {
            the_reason_the_machine_gave: record.deferred_because.clone(),
        },
        phase => WhatTheRequestAnswered::ThePhaseTheTransactionReached(phase),
    }
}

impl WhatTheRequestAnswered {
    /// **B6's three prohibitions, as one answer rather than three habits.**
    ///
    /// A deferred result *"is not a failure, it does not expire a waiter, and it
    /// does not license a second attempt under a fresh identity"*.
    pub fn is_a_failure(&self) -> bool {
        false
    }

    /// **B6.** Whether this result expires a waiter. A waiter's deadline is the
    /// waiter's own (C16) and nothing a result says reaches one.
    pub fn expires_a_waiter(&self) -> bool {
        false
    }

    /// **B6.** Whether this result licenses a second attempt under a fresh
    /// identity.
    ///
    /// **Never, and nothing here could compose one**: a fresh attempt would want
    /// a fresh identity, and [`TheBootstrapAndRepairRequest`] issues none — the
    /// identity is the caller's, handed in. C5 is the rule this sits under: a
    /// re-send carries the **same** identity and is not a retry.
    pub fn licenses_a_second_attempt_under_a_fresh_identity(&self) -> bool {
        false
    }
}

/// **B7. Nothing about this request is automatic in this document.**
///
/// *"The automatic supervisor-unavailable repair trigger is `Desktop app and
/// delivery`'s and the user-facing fallback re-run is `Server Setup`'s. What is
/// owned here is the request they both send."*
///
/// **So there is no timer, no watcher, no scheduler and no condition anywhere in
/// this module**, and every function above takes its subject from a caller.
pub const NOTHING_HERE_SENDS_THIS_REQUEST_OF_ITS_OWN_ACCORD: () = ();

#[cfg(test)]
mod tests {
    use super::*;
    use smoozy_framing::framing::THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS;
    use smoozy_runtime::machine_kind::MachineKind;
    use smoozy_runtime::package::phase::{a_fresh_attempts_first_write, PreviousVersion};
    use smoozy_state::identities::IdentityKind;

    /// **The code of this file's working half** — everything above the tests,
    /// with every comment line taken out.
    ///
    /// **Both halves of that are the point.** A guard that read the whole file
    /// would find the very word it forbids inside its own assertion message and
    /// fail on itself, which is the trap `markers.rs` names when it says no
    /// example is written out there on purpose. And a guard that kept the
    /// comments would fail on the clause it is quoting: the prose above says
    /// *machine-local endpoint* in the course of saying that nothing reaches
    /// one.
    fn the_code_of_this_file() -> String {
        let whole = include_str!("bootstrap.rs");
        let working = whole
            .split_once("\n#[cfg(test)]")
            .expect("this file has a test module")
            .0;
        working
            .lines()
            .filter(|line| !line.trim_start().starts_with("//"))
            .collect::<Vec<&str>>()
            .join("\n")
    }

    fn an_operation_identity() -> Identity {
        Identity::issue(IdentityKind::Operation).expect("the desk has randomness")
    }

    fn a_descriptor(sequence: u64) -> VerifiedReleaseDescriptor {
        VerifiedReleaseDescriptor {
            manifest_format: "smoozy-server-package-manifest-1".to_string(),
            verified_release_sequence: sequence,
            requested_package_version: "0.1.0".to_string(),
            manifest_digest: [7; 32],
            artifact_identity: "linux-x86_64-gnu".to_string(),
            artifact_byte_length: 4096,
            artifact_digest: [9; 32],
        }
    }

    /// The same seven values as [`a_descriptor`], in the shape a request
    /// carries them. **The two types are deliberately not one** — the machine
    /// side's is `Rented-server runtime` G27's and this one is the desktop's —
    /// so a test that wants both writes both.
    fn the_descriptors_values() -> TheDescriptorsValues {
        TheDescriptorsValues {
            manifest_format: "smoozy-server-package-manifest-1".to_string(),
            verified_release_sequence: 7,
            requested_package_version: "0.1.0".to_string(),
            manifest_digest: [7; 32],
            artifact_identity: "linux-x86_64-gnu".to_string(),
            artifact_byte_length: 4096,
            artifact_digest: [9; 32],
        }
    }

    fn a_phase_record(attempt: Identity) -> PhaseRecord {
        a_fresh_attempts_first_write(
            attempt,
            a_descriptor(7),
            None,
            MachineKind::Systemd,
            "/ephemeral/smoozy/staging/one".to_string(),
        )
    }

    /// **B1.** The correlation identity and the idempotency key are one value.
    #[test]
    fn the_idempotency_key_is_the_correlation_identity_and_not_a_second_value() {
        let identity = an_operation_identity();
        let request = TheBootstrapAndRepairRequest {
            identity: identity.clone(),
            descriptor: the_descriptors_values(),
        };
        assert_eq!(request.identity(), request.the_idempotency_key());
        assert_eq!(request.identity(), &identity);
    }

    fn a_request() -> TheBootstrapAndRepairRequest {
        TheBootstrapAndRepairRequest {
            identity: an_operation_identity(),
            descriptor: the_descriptors_values(),
        }
    }

    /// **B1 and P4.** The frame carries the version this build speaks, the
    /// identity, the one request name, and the descriptor's seven values —
    /// **and nothing else at all.**
    #[test]
    fn the_frame_carries_the_descriptor_and_nothing_that_selects_a_behaviour() {
        let request = a_request();
        let framed = request.framed();
        assert_eq!(framed.protocol_version, THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS);
        assert_eq!(&framed.identity, request.identity());
        assert_eq!(framed.name, THE_BOOTSTRAP_AND_REPAIR_REQUEST);
        assert_eq!(framed.fields.how_many(), 7);
        for field in [
            THE_MANIFEST_FORMAT_FIELD,
            THE_RELEASE_SEQUENCE_FIELD,
            THE_REQUESTED_PACKAGE_VERSION_FIELD,
            THE_MANIFEST_DIGEST_FIELD,
            THE_ARTIFACT_IDENTITY_FIELD,
            THE_ARTIFACT_BYTE_LENGTH_FIELD,
            THE_ARTIFACT_DIGEST_FIELD,
        ] {
            assert!(framed.fields.value(field).is_some(), "{field}");
        }
        assert_eq!(
            framed.fields.value(THE_ARTIFACT_DIGEST_FIELD),
            Some(&[9u8; 32][..])
        );
    }

    /// **B1's *no request can select bytes by a version string alone*, held by
    /// the one constructor.**
    ///
    /// The only way to a request is through an [`AVerifiedRelease`], and the
    /// only producer of one of those is this crate's own verification. **The
    /// guard is a reading of this file**, because a constructor that does not
    /// exist cannot be called in a test.
    #[test]
    fn the_only_constructor_takes_a_verified_release_and_never_a_version() {
        let code = the_code_of_this_file();
        assert_eq!(
            code.matches("-> TheBootstrapAndRepairRequest").count(),
            1,
            "there is exactly one way to compose this request, and it takes a verified release"
        );
        assert!(code.contains(
            "pub fn for_this_verified_release(\n        identity: Identity,\n        verified: &AVerifiedRelease,"
        ));
    }

    /// **B2.** The surface is two mechanisms and there is no third for the
    /// machine-local endpoint.
    #[test]
    fn the_request_rides_two_mechanisms_and_the_endpoint_is_not_one_of_them() {
        assert_eq!(WhereTheRequestRides::BOTH.len(), 2);
        assert!(
            !the_code_of_this_file().contains("endpoint"),
            "B2: this request rides the pre-package surface and never the machine-local endpoint"
        );
    }

    /// **B2.** Every bare machine and every dead supervisor gets the same
    /// performer.
    #[test]
    fn the_desktop_performs_it_wherever_no_verified_runtime_is_alive() {
        assert_eq!(
            who_performs_the_transaction(false),
            WhoPerformsTheTransaction::TheDesktopThroughTheSurface
        );
        assert_eq!(
            who_performs_the_transaction(true),
            WhoPerformsTheTransaction::ALiveVerifiedRuntime
        );
    }

    /// **B3.** The idempotency answer comes off the phase record's own attempt
    /// identity.
    #[test]
    fn a_repeated_identity_is_answered_from_the_phase_record() {
        let request = a_request();
        let record = a_phase_record(request.identity().clone());
        assert_eq!(
            what_the_phase_record_says_about(&request, Some(&record)),
            WhatThisIdentityMeans::ItIsAlreadyRecorded {
                at: Phase::Inspecting
            }
        );
    }

    /// **B3.** A record carrying another attempt is no record of this identity,
    /// and neither is no record at all.
    #[test]
    fn another_attempts_record_is_not_a_record_of_this_identity() {
        let request = a_request();
        let somebody_elses = a_phase_record(an_operation_identity());
        assert_eq!(
            what_the_phase_record_says_about(&request, Some(&somebody_elses)),
            WhatThisIdentityMeans::NoRecordOfThisIdentity
        );
        assert_eq!(
            what_the_phase_record_says_about(&request, None),
            WhatThisIdentityMeans::NoRecordOfThisIdentity
        );
    }

    /// **B3's last sentence, held by a reading of this file**: no operation
    /// record is looked for.
    #[test]
    fn no_operation_record_is_reached_for_this_request() {
        assert!(
            !the_code_of_this_file().contains("OperationRecord"),
            "B3 says outright that a builder must not look for one of these for this request"
        );
    }

    /// **B4.** A machine that has never held a package answers absent
    /// everywhere, and that is a real answer.
    #[test]
    fn a_machine_that_has_never_held_a_package_answers_honestly() {
        let inspection = compose_the_inspection(WhatTheRecordsHeld {
            phase_record: WhatARecordHeld::Nothing,
            boot_registration: WhatARecordHeld::Nothing,
            installation_identity: WhatTheStateAreaHeld::Nothing,
            the_pointer_points_at: None,
        });
        assert!(inspection.every_record_is_absent());
        assert!(!inspection.a_record_is_present_and_not_smoozys());
        assert!(inspection.the_declared_request_set().is_empty());
    }

    /// **B4.** The pointer's own value decides which recorded descriptor is the
    /// active one, and a pointer naming neither claims none.
    #[test]
    fn the_pointers_own_value_decides_which_descriptor_is_the_active_one() {
        let mut record = a_phase_record(an_operation_identity());
        record.previous = Some(PreviousVersion {
            descriptor: a_descriptor(6),
            retained_adapter: RetainedAdapter {
                declared_interval: (1, 1),
                declared_request_set: vec!["readiness".to_string()],
            },
            location: "/ephemeral/smoozy/package/six".to_string(),
            previous_boot_registration_content: b"the old hook".to_vec(),
        });

        let at_the_staged = compose_the_inspection(WhatTheRecordsHeld {
            phase_record: WhatARecordHeld::It(record.clone()),
            boot_registration: WhatARecordHeld::Nothing,
            installation_identity: WhatTheStateAreaHeld::Nothing,
            the_pointer_points_at: Some("/ephemeral/smoozy/staging/one".to_string()),
        });
        assert_eq!(
            at_the_staged
                .the_active_packages_descriptor
                .as_ref()
                .map(|found| found.verified_release_sequence),
            Some(7)
        );

        let at_the_previous = compose_the_inspection(WhatTheRecordsHeld {
            phase_record: WhatARecordHeld::It(record.clone()),
            boot_registration: WhatARecordHeld::Nothing,
            installation_identity: WhatTheStateAreaHeld::Nothing,
            the_pointer_points_at: Some("/ephemeral/smoozy/package/six".to_string()),
        });
        assert_eq!(
            at_the_previous
                .the_active_packages_descriptor
                .as_ref()
                .map(|found| found.verified_release_sequence),
            Some(6)
        );

        // **A pointer naming neither is the interrupted-rename state, and no
        // descriptor is claimed for it.**
        let at_neither = compose_the_inspection(WhatTheRecordsHeld {
            phase_record: WhatARecordHeld::It(record),
            boot_registration: WhatARecordHeld::Nothing,
            installation_identity: WhatTheStateAreaHeld::Nothing,
            the_pointer_points_at: Some("/somewhere/else".to_string()),
        });
        assert!(at_neither.the_active_packages_descriptor.is_none());
    }

    /// **B4.** A record that is present and does not read back is never the
    /// absent branch, and it is not a healthy machine either.
    #[test]
    fn a_record_present_and_not_smoozys_is_never_read_as_absence() {
        let inspection = compose_the_inspection(WhatTheRecordsHeld {
            phase_record: the_record_in(b"an 82-byte provider default", written_form::read::<PhaseRecord>),
            boot_registration: WhatARecordHeld::Nothing,
            installation_identity: WhatTheStateAreaHeld::Nothing,
            the_pointer_points_at: None,
        });
        assert!(!inspection.every_record_is_absent());
        assert!(inspection.a_record_is_present_and_not_smoozys());
        assert!(inspection.the_active_packages_descriptor.is_none());
    }

    /// **An empty record is present and unreadable, never absent.**
    #[test]
    fn an_empty_record_is_not_an_absent_one() {
        assert!(the_record_in(b"", written_form::read::<PhaseRecord>).is_present_and_not_smoozys());
    }

    /// **B4 and §1.2 item 3.** The declared request set is read off the record's
    /// retained adapters rather than asked of a program.
    #[test]
    fn the_declared_request_set_comes_off_the_record() {
        let mut record = a_phase_record(an_operation_identity());
        record.staged_packages_retained_adapters = vec![RetainedAdapter {
            declared_interval: (1, 1),
            declared_request_set: vec!["readiness".to_string(), "health".to_string()],
        }];
        record.previous = Some(PreviousVersion {
            descriptor: a_descriptor(6),
            retained_adapter: RetainedAdapter {
                declared_interval: (1, 1),
                declared_request_set: vec!["readiness".to_string()],
            },
            location: "/ephemeral/smoozy/package/six".to_string(),
            previous_boot_registration_content: b"the old hook".to_vec(),
        });
        let inspection = compose_the_inspection(WhatTheRecordsHeld {
            phase_record: WhatARecordHeld::It(record),
            boot_registration: WhatARecordHeld::Nothing,
            installation_identity: WhatTheStateAreaHeld::Nothing,
            the_pointer_points_at: None,
        });
        assert_eq!(inspection.the_retained_adapters.len(), 2);
        assert_eq!(
            inspection.the_declared_request_set(),
            vec!["readiness".to_string(), "health".to_string()]
        );
    }

    /// **B4.** The boot-registration record is read as content, and the content
    /// is what comes back.
    #[test]
    fn the_boot_registration_records_own_content_reads_back() {
        let written = written_form::write(&BootRegistrationRecord {
            kind: MachineKind::Systemd,
            at_path: "/home/whoever/.config/systemd/user/smoozy.service".to_string(),
            content: b"[Unit]\n".to_vec(),
        })
        .expect("it writes");
        let held = the_record_in(&written, written_form::read::<BootRegistrationRecord>);
        let WhatARecordHeld::It(record) = held else {
            panic!("the record reads back");
        };
        assert_eq!(record.content, b"[Unit]\n".to_vec());
    }

    /// **B5.** The remembered shape is subject-held and carries the installation
    /// identity it was read under.
    #[test]
    fn a_remembered_inspection_is_subject_held_and_names_its_installation() {
        assert_eq!(ARememberedInspection::LIFETIME, Lifetime::SubjectHeld);
        let remembered = ARememberedInspection {
            reading: Reading::taken(
                compose_the_inspection(WhatTheRecordsHeld {
                    phase_record: WhatARecordHeld::Nothing,
                    boot_registration: WhatARecordHeld::Nothing,
                    installation_identity: WhatTheStateAreaHeld::Nothing,
                    the_pointer_points_at: None,
                }),
                SystemTime::UNIX_EPOCH,
                Clock::ThisComputer,
            ),
            read_under: Identity::issue(IdentityKind::Installation).expect("randomness"),
        };
        assert_eq!(remembered.reading.clock(), &Clock::ThisComputer);
        assert_eq!(remembered.reading.taken_at(), SystemTime::UNIX_EPOCH);
    }

    /// **B5 and `State, identity and recovery` D4.** Nothing in this crate
    /// writes an inspection down.
    #[test]
    fn nothing_here_writes_an_inspection_down() {
        let code = the_code_of_this_file();
        assert!(!code.contains("std::fs"));
        assert!(!code.contains("fn write"));
    }

    /// **B6.** A deferred commit is one of the request's ordinary results, and
    /// the machine's own reason is carried whole.
    #[test]
    fn a_deferred_commit_is_carried_and_not_interpreted() {
        let mut record = a_phase_record(an_operation_identity());
        record.phase = Phase::Deferred;
        record.deferred_because = Some("a render is running".to_string());
        let answered = what_the_result_is(&record);
        assert_eq!(
            answered,
            WhatTheRequestAnswered::TheUpgradeIsDeferred {
                the_reason_the_machine_gave: Some("a render is running".to_string())
            }
        );
        assert!(!answered.is_a_failure());
        assert!(!answered.expires_a_waiter());
        assert!(!answered.licenses_a_second_attempt_under_a_fresh_identity());
    }

    /// **B6.** Every other phase is carried as the phase it is.
    #[test]
    fn every_other_phase_is_carried_as_the_phase_it_is() {
        for phase in Phase::ALL {
            let mut record = a_phase_record(an_operation_identity());
            record.phase = phase;
            let answered = what_the_result_is(&record);
            if phase == Phase::Deferred {
                assert!(matches!(
                    answered,
                    WhatTheRequestAnswered::TheUpgradeIsDeferred { .. }
                ));
            } else {
                assert_eq!(
                    answered,
                    WhatTheRequestAnswered::ThePhaseTheTransactionReached(phase)
                );
            }
        }
    }

    /// **B7.** Nothing in this module starts anything of its own accord.
    #[test]
    fn nothing_here_is_automatic() {
        let code = the_code_of_this_file();
        for automatic in ["spawn", "thread", "tokio::time", "sleep", "Instant"] {
            assert!(
                !code.contains(automatic),
                "B7: nothing about this request is automatic in this document ({automatic})"
            );
        }
    }
}
