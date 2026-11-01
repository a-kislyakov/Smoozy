//! §2 — request framing and versioning, over the machine-local endpoint.
//!
//! **What this is.** One framed request, one correlated result, the two versions
//! that travel with them, what a version disagreement does, and the two rules
//! about a request's fields. `Rented-server runtime` owns the endpoint itself —
//! its existence, its lifecycle and its access boundary — and every machine-side
//! response to a request; what is here is the shape a request takes on the way
//! in and a result takes on the way out.
//!
//! **Why it is here rather than beside the endpoint.** The document's own scope
//! line puts *request framing over the machine-local endpoint* in this document,
//! and `Security and licence` §8 requirement 3 says the same from the other
//! side: *"Every request is still framed, versioned and correlated as
//! `Server protocol and transfers` defines. The endpoint is not a trusted channel
//! that skips validation; it is a channel whose peer is known."*
//!
//! # The wire form, and every part of it is this build's decision
//!
//! **The plan fixes one property and no bytes.** §1.1 says every value rides the
//! entry point's standard input *"in this document's length-delimited frame"*,
//! and nothing anywhere says what the delimiters are. **So the layout below is a
//! Build decision recorded in this module's decisions record**, and it is written
//! out here so that the two ends are built from one description rather than two.
//!
//! ```text
//! frame   := u32 length of the body            (big-endian)
//!            body
//! request := u32 the protocol version the caller speaks
//!            text  the correlation identity
//!            text  the request's name
//!            u32 how many fields
//!            field*
//! result  := u32 the protocol version the answering side speaks
//!            text  the package version it was built from
//!            text  the correlation identity
//!            u32 how many fields
//!            field*
//! field   := text  the field's name
//!            bytes the field's value
//! text    := u32 length, then that many bytes, read as UTF-8
//! bytes   := u32 length, then that many bytes, read as nothing
//! ```
//!
//! **A field's value is bytes and never text, and that is the plan's own reason
//! rather than caution.** §1.1: the entry point *"never passes a framed path
//! through a shell"*, and *"a leading dash, whitespace, a newline and a non-UTF-8
//! filename are values rather than syntax"*. A value shape that could not hold a
//! non-UTF-8 filename would make that sentence unkeepable.
//!
//! **The length is bounded and the bound is not a default.** A frame whose
//! declared length is past [`THE_LARGEST_FRAME`] is refused before a byte of it
//! is taken, because a length read off a stream is a number the far end chose.
//! Nothing in this document needs a larger one: a transfer's bytes ride a channel
//! of their own (T1) and the file-transfer subsystem's own protocol fields carry
//! every path, attribute and free-space reading (§1.1), so what a frame ever
//! holds is a request's own values.
//!
//! **Markers used in this file that belong to another document, each with the
//! document it belongs to.** `Security and licence` §8 and its requirements 2
//! and 3. `State, identity and recovery` §2.1, §2.2, §9.1.
//!
//! **A marker this plan also defines is never listed there and is written out
//! with its document at the site**, because a header list cannot tell the two
//! apart and a bare one there would resolve to a real and wrong clause.

use std::io::{Read, Write};

use smoozy_state::identities::{Identity, IdentityKind};

/// **P2. Two versions travel and they are not one field.**
///
/// The **protocol version** names the request framing, the request set, the
/// digest function (S23) and the one rate unit every budget, enforcement and
/// reading in this document is carried in (R11). The **package version** names
/// the installed material, and it is a string this code never parses.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ProtocolVersion(pub u32);

/// **The protocol version this build of Smoozy speaks.**
///
/// One, because there has never been an earlier one. **That is why
/// [`DeclaredVersions::of_this_build`] declares no immediately previous version
/// rather than declaring a placeholder** — P3's interval is *current plus the
/// single immediately previous where one exists*, and none exists.
pub const THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS: ProtocolVersion = ProtocolVersion(1);

/// **The largest frame either end reads**, a Build decision and not a plan
/// figure. One mebibyte. See the head of this file for why nothing this document
/// frames comes near it.
pub const THE_LARGEST_FRAME: usize = 1024 * 1024;

/// **P3's declared interval: the current protocol version and, where one exists,
/// the single immediately previous supported one.**
///
/// **No wider or skipped-version interval is accepted**, which is why
/// [`DeclaredVersions::declaring`] is the only way to make one and refuses
/// anything else.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DeclaredVersions {
    current: ProtocolVersion,
    the_single_immediately_previous: Option<ProtocolVersion>,
}

/// **What a manifest declared that P3 does not accept.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NotAnAcceptableInterval {
    /// A previous version that is not the one immediately below the current
    /// one — a skipped version, or a wider interval than the two P3 allows.
    ItIsNotTheSingleImmediatelyPreviousVersion,
}

impl DeclaredVersions {
    /// **P3.** A manifest's declaration, refused where the interval is wider than
    /// current-plus-one-previous or skips a version.
    pub fn declaring(
        current: ProtocolVersion,
        the_single_immediately_previous: Option<ProtocolVersion>,
    ) -> Result<Self, NotAnAcceptableInterval> {
        if let Some(previous) = the_single_immediately_previous {
            if previous.0 + 1 != current.0 {
                return Err(NotAnAcceptableInterval::ItIsNotTheSingleImmediatelyPreviousVersion);
            }
        }
        Ok(DeclaredVersions {
            current,
            the_single_immediately_previous,
        })
    }

    /// What this build declares.
    pub fn of_this_build() -> Self {
        DeclaredVersions {
            current: THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS,
            the_single_immediately_previous: None,
        }
    }

    pub fn current(&self) -> ProtocolVersion {
        self.current
    }

    pub fn the_single_immediately_previous(&self) -> Option<ProtocolVersion> {
        self.the_single_immediately_previous
    }

    /// Whether this declaration covers a version — used by P3's deferral rule
    /// rather than by negotiation, which has its own order.
    pub fn covers(&self, version: ProtocolVersion) -> bool {
        self.current == version || self.the_single_immediately_previous == Some(version)
    }
}

/// **P3's outcome.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Negotiated {
    /// The version both sides will speak.
    Speak(ProtocolVersion),
    /// **An empty intersection is incompatible, never a guessed downgrade.**
    Incompatible,
}

/// **P3. Negotiation chooses the current version where equal, otherwise that one
/// immediately previous version where it is the intersection.**
///
/// **An empty intersection is incompatible**, and there is no third answer: this
/// function never returns a version neither side declared, and it never picks a
/// lower one because it looks safer.
///
/// **Nothing calls this, and what it waits on is one thing.** The caller that
/// would is the desktop, and the answerer's declared interval reaches it through
/// **package inspection**, which §1.2 item 3 makes the route a package version
/// declares its request set and its interval by. **Package inspection is built**
/// — `smoozy_server_protocol::bootstrap::inspect` — **and what is missing is the
/// sending**: no route in that crate carries a framed request to the entry
/// point, so nothing composes a negotiation. *(Corrected 2026-09-07: this said
/// package inspection was not built, which stopped being true on 2026-09-06;
/// `smoozy_server_protocol::reconnection` had said so since and this had not.)*
/// **So a caller for this is still owed and is not built here.**
///
/// **What does not wait on it is the answering side's own check**, and that is
/// built: the machine program refuses a request whose declared version this
/// build does not cover, using [`DeclaredVersions::of_this_build`] and
/// [`DeclaredVersions::covers`], and says so with
/// [`THE_FIELD_NAME_RESERVED_FOR_A_PROTOCOL_VERSION_THIS_PACKAGE_DOES_NOT_SPEAK`].
pub fn negotiate(caller: &DeclaredVersions, answerer: &DeclaredVersions) -> Negotiated {
    if caller.current == answerer.current {
        return Negotiated::Speak(caller.current);
    }
    if let Some(previous) = caller.the_single_immediately_previous {
        if answerer.covers(previous) {
            return Negotiated::Speak(previous);
        }
    }
    if let Some(previous) = answerer.the_single_immediately_previous {
        if caller.covers(previous) {
            return Negotiated::Speak(previous);
        }
    }
    Negotiated::Incompatible
}

/// **P3's second half — a package commit may not strand accepted work — is kept
/// at the commit and not here, and this is the pointer rather than a second
/// copy.**
///
/// The rule needs the record of which accepted operations are still live, held,
/// uncollected or unacknowledged, and that record is the package-phase record,
/// which `Rented-server runtime` owns. `Rented-server runtime` G21 states the
/// condition, `Rented-server runtime` G22 makes a blocked commit *deferred*
/// rather than failed, and `Rented-server runtime` G23
/// refuses to retire the previous adapter until nothing depends on it — all
/// three citing this clause. **Writing the rule here as well would give one fact
/// two homes**, and the one that would go stale is the one with no record under
/// it. The value is `()` because what this names is where the rule lives.
pub const P3S_DEFERRAL_HALF_IS_KEPT_AT_THE_COMMIT: () = ();

/// **P3a. What a caller does when it meets a protocol interval with no overlap.**
///
/// **It does not submit a new operation and does not parse a result it cannot
/// frame**, and there is no branch here that guesses at a shape.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhatACallerDoesWithNoOverlap {
    /// **It first uses package inspection to locate any retained adapter whose
    /// interval overlaps and uses that adapter only for the inspection, stop,
    /// release and collection routes P3 preserves.**
    UseTheRetainedAdapterForInspectionStopReleaseAndCollectionOnly,
    /// **Where there is no compatible retained adapter and no live or held work,
    /// the ordinary supervisor-unavailable path applies**: the caller reports the
    /// supervisor as unavailable **without marking the server connection down**,
    /// inspects the package, and the automatic repair `Desktop app and delivery`
    /// owns invokes §5.
    ReportTheSupervisorUnavailableWithoutMarkingTheConnectionDown,
    /// **Where live or held work exists but no retained compatible adapter can
    /// control it, repair fails closed and reports the exact work identity; it
    /// does not replace the only remaining route to that work.**
    RepairFailsClosedNamingTheExactWorkIdentity,
}

/// **P3a.**
///
/// **Both answers this reads are somebody else's**: whether a retained adapter's
/// interval overlaps comes from package inspection, which is §5's, and whether
/// live or held work exists comes from the machine's own records. What is here is
/// the decision over them.
pub fn what_a_caller_does_with_no_overlap(
    a_retained_adapter_whose_interval_overlaps: bool,
    live_or_held_work_exists: bool,
) -> WhatACallerDoesWithNoOverlap {
    if a_retained_adapter_whose_interval_overlaps {
        return WhatACallerDoesWithNoOverlap::UseTheRetainedAdapterForInspectionStopReleaseAndCollectionOnly;
    }
    if live_or_held_work_exists {
        WhatACallerDoesWithNoOverlap::RepairFailsClosedNamingTheExactWorkIdentity
    } else {
        WhatACallerDoesWithNoOverlap::ReportTheSupervisorUnavailableWithoutMarkingTheConnectionDown
    }
}

/// **P6. Nothing about a request's framing is a secret.**
///
/// The machine-local endpoint's path is not a credential and is not treated as
/// one (`Security and licence` §8), **and neither is a request's shape** — which
/// is why the whole layout is written out at the head of this file rather than
/// left to be read off the code. The value is `()` because the clause is an
/// absence: nothing here obscures a name, a length or a field.
pub const NOTHING_ABOUT_A_REQUESTS_FRAMING_IS_A_SECRET: () = ();

/// **P5's second half, and A2's whole: the one field kind that is not merely
/// ignored but forbidden is a field naming the caller's own identity.**
///
/// **Ignoring it and checking it are the same thing for security and only one of
/// them is safe to write down**, so a field of this name is dropped as the frame
/// is read and there is no accessor anywhere that would return it.
/// `Rented-server runtime` Z6 establishes the caller's identity from the accepted
/// connection itself, and `Security and licence` §8 requirement 2 is the rule.
///
/// **The name is this build's**: the clause forbids a kind of field and names no
/// field, so the framing reserves one name for the kind and drops it. A caller
/// that sends it is not refused — refusing would be checking.
pub const THE_FIELD_NAME_RESERVED_FOR_A_CALLERS_OWN_IDENTITY: &str = "caller";

/// **A second reserved field name: *this package does not answer that
/// request*.**
///
/// **What it is against, because no clause names it.** C7 and C8 are about what
/// became of an operation **identity**, not about a request **name**, and P5 is
/// about **fields** — no clause in any plan names a request a package does not
/// serve. What this keeps is `Preparation/Specs/Whole app.md`'s third promise,
/// Creator-stated: Smoozy *"always tells the truth about what happened"*.
/// Without it, a supervisor asked for a name it has never heard of composes a
/// correlated, empty, healthy-looking result, and the caller cannot tell that
/// from a request that was answered and found nothing.
///
/// **A reserved name rather than a new kind of frame**, which is the shape this
/// crate has already answered the nearest question with, one name above. It is
/// also the shape P5 protects: a reader that does not know this field ignores
/// it and carries on, so an older caller is not broken by a newer package
/// filling it in.
///
/// **It is not a fourth acknowledgement state.** C8 has three closed answers
/// about an operation and this is none of them: it says the request was never a
/// request this package serves.
///
/// **The value is the request's own name**, which the result frame does not
/// otherwise carry.
pub const THE_FIELD_NAME_RESERVED_FOR_A_REQUEST_THIS_PACKAGE_DOES_NOT_ANSWER: &str =
    "no-such-request";

/// **A third reserved field name: *this package does not speak that protocol
/// version*.**
///
/// **P3a puts the duty of not submitting on the caller** — *"It does not submit
/// a new operation and does not parse a result it cannot frame"* — and that
/// decision is written above and tested. **The answering side performs no check
/// of its own**, so a supervisor that speaks one version would otherwise answer
/// a request framed at another as though it had understood it. This field is how
/// the answering side says it did not.
///
/// **It rides the same route the field above does**, for the same reasons, and
/// it is likewise not a fourth acknowledgement state.
///
/// **The value is the version the request declared**, written as a decimal
/// number in text — the version the *answering* side speaks is already on every
/// result frame (P2), so what is not otherwise carried is the one that was
/// refused.
pub const THE_FIELD_NAME_RESERVED_FOR_A_PROTOCOL_VERSION_THIS_PACKAGE_DOES_NOT_SPEAK: &str =
    "no-such-protocol-version";

/// **The readiness request, by name.**
///
/// §1.1 names it: the package's entry point *"carries the SSH command adapter,
/// and with it **readiness**, hardware readings, digest computation over a byte
/// range, and every request the second mechanism has no operation for."* **The
/// name is written here because this is where request names are carried**, and
/// the request itself is that clause's rather than §2's. It asks the supervisor
/// nothing about the machine: what it establishes is that the supervisor answered
/// at all, which is what `Rented-server runtime` V4 says its observed value is.
pub const THE_READINESS_REQUEST: &str = "readiness";

/// The fields a request or a result carries.
///
/// **P5. Every request field is either required by this protocol version or
/// ignored.** An adapter that meets a field it does not know **ignores it rather
/// than refusing the request**, and a caller that meets a result field it does not
/// know **ignores it rather than refusing the result**. That is why this is a
/// list read by name and never a fixed shape: a reader takes the fields it knows
/// and the rest sit here unread.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Fields {
    named: Vec<(String, Vec<u8>)>,
}

impl Fields {
    pub fn none() -> Self {
        Fields { named: Vec::new() }
    }

    /// Adds a field. **A field naming the caller's own identity is dropped here
    /// too**, so that the forbidden kind cannot be put on the wire by this crate
    /// any more than it can be read off it.
    pub fn and(mut self, name: &str, value: impl Into<Vec<u8>>) -> Self {
        if name == THE_FIELD_NAME_RESERVED_FOR_A_CALLERS_OWN_IDENTITY {
            return self;
        }
        self.named.push((name.to_string(), value.into()));
        self
    }

    /// A field's value, or nothing where the frame carried none of that name.
    pub fn value(&self, name: &str) -> Option<&[u8]> {
        self.named
            .iter()
            .find(|(held, _)| held == name)
            .map(|(_, value)| value.as_slice())
    }

    /// A field's value read as text, or nothing where it was absent or was not
    /// text. **A field this reader cannot make sense of is the ignored case**,
    /// not a refusal of the whole frame.
    pub fn text(&self, name: &str) -> Option<&str> {
        std::str::from_utf8(self.value(name)?).ok()
    }

    pub fn how_many(&self) -> usize {
        self.named.len()
    }
}

/// **P1. One framed request.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Request {
    /// **P2.** The protocol version the caller speaks.
    pub protocol_version: ProtocolVersion,
    /// **C1.** The operation identity, chosen by the initiating side before the
    /// request is sent, opaque and never parsed.
    pub identity: Identity,
    /// Which request this is. The request set for an installed package version is
    /// negotiated (§1.2) and each request is named by the document that defines
    /// it; the framing carries the name and reads nothing into it.
    pub name: String,
    pub fields: Fields,
}

impl Request {
    /// One request, at the negotiated version.
    pub fn speaking(protocol_version: ProtocolVersion, identity: Identity, name: &str) -> Self {
        Request {
            protocol_version,
            identity,
            name: name.to_string(),
            fields: Fields::none(),
        }
    }

    pub fn carrying(mut self, fields: Fields) -> Self {
        self.fields = fields;
        self
    }

    /// **P4. The repair route is never versioned by the thing it repairs.**
    ///
    /// The bootstrap-and-repair request rides on the pre-package surface and is
    /// framed by this document rather than by the installed adapter, **so a
    /// package too old to be spoken to can still be replaced**. That is the
    /// structural reason the request is independent of the supervisor's endpoint
    /// rather than a convenience — and it is kept here by this being the one
    /// constructor that takes no version at all: it can only ever speak
    /// [`THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS`].
    pub fn for_the_bootstrap_and_repair_route(identity: Identity, name: &str) -> Self {
        Request::speaking(THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS, identity, name)
    }
}

/// **P1. Exactly one correlated result.**
///
/// **A3. Nothing that crosses from a machine is trusted as an entitlement
/// statement**, which is `Security and licence` §1.2 and is not widened here.
/// **A result carrying anything that reads as a permission, a
/// licence state or a capability grant is read as an observation and never as
/// authority** — and the way that is kept is an absence: [`Fields`] hands back a
/// value and this crate composes no permission, no licence state and no
/// capability grant from one anywhere.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Answered {
    /// **P2.** The protocol version the answering side speaks.
    pub protocol_version: ProtocolVersion,
    /// **P2.** The package version it was built from.
    pub package_version: String,
    /// **C1 and C2.** The identity the request carried.
    pub identity: Identity,
    pub fields: Fields,
}

impl Answered {
    pub fn to(request: &Request, protocol_version: ProtocolVersion, package_version: &str) -> Self {
        Answered {
            protocol_version,
            package_version: package_version.to_string(),
            identity: request.identity.clone(),
            fields: Fields::none(),
        }
    }

    pub fn carrying(mut self, fields: Fields) -> Self {
        self.fields = fields;
        self
    }

    /// **Say that this package does not answer that request**, naming it.
    ///
    /// See [`THE_FIELD_NAME_RESERVED_FOR_A_REQUEST_THIS_PACKAGE_DOES_NOT_ANSWER`].
    /// **It carries nothing else**: a result that says this is not a result that
    /// also answers something.
    pub fn saying_it_does_not_answer_that_request(mut self, request_name: &str) -> Self {
        self.fields = Fields::none().and(
            THE_FIELD_NAME_RESERVED_FOR_A_REQUEST_THIS_PACKAGE_DOES_NOT_ANSWER,
            request_name,
        );
        self
    }

    /// **Say that this package does not speak the protocol version the request
    /// declared**, naming that version.
    ///
    /// See
    /// [`THE_FIELD_NAME_RESERVED_FOR_A_PROTOCOL_VERSION_THIS_PACKAGE_DOES_NOT_SPEAK`].
    pub fn saying_it_does_not_speak_that_protocol_version(
        mut self,
        the_version_the_request_declared: ProtocolVersion,
    ) -> Self {
        self.fields = Fields::none().and(
            THE_FIELD_NAME_RESERVED_FOR_A_PROTOCOL_VERSION_THIS_PACKAGE_DOES_NOT_SPEAK,
            the_version_the_request_declared.0.to_string(),
        );
        self
    }

    /// **The caller's side of the first reserved name**: the request name this
    /// package said it does not answer, or nothing where it said no such thing.
    pub fn the_request_name_it_says_it_does_not_answer(&self) -> Option<&str> {
        self.fields
            .text(THE_FIELD_NAME_RESERVED_FOR_A_REQUEST_THIS_PACKAGE_DOES_NOT_ANSWER)
    }

    /// **The caller's side of the second reserved name**: the protocol version
    /// this package said it does not speak, or nothing where it said no such
    /// thing. **A value that is not a number reads as nothing**, which is P5's
    /// ignore rather than a refusal of the result.
    pub fn the_protocol_version_it_says_it_does_not_speak(&self) -> Option<ProtocolVersion> {
        self.fields
            .text(THE_FIELD_NAME_RESERVED_FOR_A_PROTOCOL_VERSION_THIS_PACKAGE_DOES_NOT_SPEAK)?
            .parse()
            .ok()
            .map(ProtocolVersion)
    }

    /// **C2. A result is matched to a request by that identity and never by
    /// arrival order, never by channel, and never by the invocation that carried
    /// it.**
    pub fn answers(&self, request: &Request) -> bool {
        self.identity == request.identity
    }
}

/// Why a frame could not be read.
///
/// **None of these is a request result.** A frame that could not be read is not a
/// request that was answered badly; the caller has nothing correlated and says so.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WhyTheFrameCouldNotBeRead {
    /// The stream ended before the frame did. **On the caller's side this is the
    /// far end not answering**, which is a different thing from an answer.
    ItEndedPartWayThrough,
    /// The declared length is past [`THE_LARGEST_FRAME`], and not a byte of it
    /// was taken.
    ItDeclaredMoreThanTheLargestFrame(usize),
    /// A length inside the frame runs past the frame's own end.
    ALengthRunsPastTheEnd,
    /// A field that must be text was not text.
    SomethingThatMustBeTextWasNot,
    /// The identity did not read back as one. **It is not repaired and nothing is
    /// inferred from it** — `State, identity and recovery` §2.1 makes an identity
    /// opaque, so a value that is not one is not a value at all.
    TheIdentityDidNotReadBack,
    /// The system refused the read or the write.
    TheSystemRefusedIt(String),
}

impl WhyTheFrameCouldNotBeRead {
    pub fn as_text(&self) -> String {
        match self {
            WhyTheFrameCouldNotBeRead::ItEndedPartWayThrough => {
                "the frame ended part way through".to_string()
            }
            WhyTheFrameCouldNotBeRead::ItDeclaredMoreThanTheLargestFrame(length) => {
                format!("the frame declared {length} bytes, past the largest frame")
            }
            WhyTheFrameCouldNotBeRead::ALengthRunsPastTheEnd => {
                "a length inside the frame runs past its end".to_string()
            }
            WhyTheFrameCouldNotBeRead::SomethingThatMustBeTextWasNot => {
                "something that must be text was not".to_string()
            }
            WhyTheFrameCouldNotBeRead::TheIdentityDidNotReadBack => {
                "the identity did not read back".to_string()
            }
            WhyTheFrameCouldNotBeRead::TheSystemRefusedIt(why) => why.clone(),
        }
    }
}

fn put_length(into: &mut Vec<u8>, length: usize) {
    into.extend_from_slice(&(length as u32).to_be_bytes());
}

fn put_bytes(into: &mut Vec<u8>, value: &[u8]) {
    put_length(into, value.len());
    into.extend_from_slice(value);
}

fn put_text(into: &mut Vec<u8>, value: &str) {
    put_bytes(into, value.as_bytes());
}

fn put_fields(into: &mut Vec<u8>, fields: &Fields) {
    put_length(into, fields.named.len());
    for (name, value) in &fields.named {
        put_text(into, name);
        put_bytes(into, value);
    }
}

/// A reader over one frame's body.
struct Reading<'a> {
    body: &'a [u8],
    at: usize,
}

impl<'a> Reading<'a> {
    fn take_length(&mut self) -> Result<usize, WhyTheFrameCouldNotBeRead> {
        if self.at + 4 > self.body.len() {
            return Err(WhyTheFrameCouldNotBeRead::ALengthRunsPastTheEnd);
        }
        let mut four = [0u8; 4];
        four.copy_from_slice(&self.body[self.at..self.at + 4]);
        self.at += 4;
        Ok(u32::from_be_bytes(four) as usize)
    }

    fn take_bytes(&mut self) -> Result<&'a [u8], WhyTheFrameCouldNotBeRead> {
        let length = self.take_length()?;
        if self.at + length > self.body.len() {
            return Err(WhyTheFrameCouldNotBeRead::ALengthRunsPastTheEnd);
        }
        let value = &self.body[self.at..self.at + length];
        self.at += length;
        Ok(value)
    }

    fn take_text(&mut self) -> Result<&'a str, WhyTheFrameCouldNotBeRead> {
        std::str::from_utf8(self.take_bytes()?)
            .map_err(|_| WhyTheFrameCouldNotBeRead::SomethingThatMustBeTextWasNot)
    }

    /// **P5.** Every field is read and kept, whether or not this build knows the
    /// name — **except the one field kind A2 forbids, which is dropped here.**
    fn take_fields(&mut self) -> Result<Fields, WhyTheFrameCouldNotBeRead> {
        let how_many = self.take_length()?;
        let mut fields = Fields::none();
        for _ in 0..how_many {
            let name = self.take_text()?.to_string();
            let value = self.take_bytes()?.to_vec();
            if name == THE_FIELD_NAME_RESERVED_FOR_A_CALLERS_OWN_IDENTITY {
                continue;
            }
            fields.named.push((name, value));
        }
        Ok(fields)
    }
}

/// The bytes of one request frame.
pub fn write_request_frame(request: &Request) -> Vec<u8> {
    let mut body = Vec::new();
    body.extend_from_slice(&request.protocol_version.0.to_be_bytes());
    put_text(&mut body, &request.identity.as_text());
    put_text(&mut body, &request.name);
    put_fields(&mut body, &request.fields);
    let mut frame = Vec::with_capacity(body.len() + 4);
    put_length(&mut frame, body.len());
    frame.extend_from_slice(&body);
    frame
}

/// The bytes of one result frame.
pub fn write_result_frame(answered: &Answered) -> Vec<u8> {
    let mut body = Vec::new();
    body.extend_from_slice(&answered.protocol_version.0.to_be_bytes());
    put_text(&mut body, &answered.package_version);
    put_text(&mut body, &answered.identity.as_text());
    put_fields(&mut body, &answered.fields);
    let mut frame = Vec::with_capacity(body.len() + 4);
    put_length(&mut frame, body.len());
    frame.extend_from_slice(&body);
    frame
}

fn read_one_body(from: &mut impl Read) -> Result<Vec<u8>, WhyTheFrameCouldNotBeRead> {
    let mut four = [0u8; 4];
    read_exactly(from, &mut four)?;
    let length = u32::from_be_bytes(four) as usize;
    if length > THE_LARGEST_FRAME {
        return Err(WhyTheFrameCouldNotBeRead::ItDeclaredMoreThanTheLargestFrame(
            length,
        ));
    }
    let mut body = vec![0u8; length];
    read_exactly(from, &mut body)?;
    Ok(body)
}

fn read_exactly(
    from: &mut impl Read,
    into: &mut [u8],
) -> Result<(), WhyTheFrameCouldNotBeRead> {
    match from.read_exact(into) {
        Ok(()) => Ok(()),
        Err(why) if why.kind() == std::io::ErrorKind::UnexpectedEof => {
            Err(WhyTheFrameCouldNotBeRead::ItEndedPartWayThrough)
        }
        Err(why) => Err(WhyTheFrameCouldNotBeRead::TheSystemRefusedIt(
            why.to_string(),
        )),
    }
}

/// **P1.** Reads exactly one request frame and no more.
///
/// **No invocation carries two requests**, because a partly-delivered pair has no
/// answer either side could give — which is `State, identity and recovery` §9.1's
/// partition unit applied to the wire: one request is one unit.
pub fn read_one_request(from: &mut impl Read) -> Result<Request, WhyTheFrameCouldNotBeRead> {
    let body = read_one_body(from)?;
    let mut reading = Reading { body: &body, at: 0 };
    let protocol_version = ProtocolVersion(reading.take_length()? as u32);
    let identity = Identity::read_back(IdentityKind::Operation, reading.take_text()?)
        .ok_or(WhyTheFrameCouldNotBeRead::TheIdentityDidNotReadBack)?;
    let name = reading.take_text()?.to_string();
    let fields = reading.take_fields()?;
    Ok(Request {
        protocol_version,
        identity,
        name,
        fields,
    })
}

/// **P1.** Reads exactly one result frame and no more.
pub fn read_one_result(from: &mut impl Read) -> Result<Answered, WhyTheFrameCouldNotBeRead> {
    let body = read_one_body(from)?;
    let mut reading = Reading { body: &body, at: 0 };
    let protocol_version = ProtocolVersion(reading.take_length()? as u32);
    let package_version = reading.take_text()?.to_string();
    let identity = Identity::read_back(IdentityKind::Operation, reading.take_text()?)
        .ok_or(WhyTheFrameCouldNotBeRead::TheIdentityDidNotReadBack)?;
    let fields = reading.take_fields()?;
    Ok(Answered {
        protocol_version,
        package_version,
        identity,
        fields,
    })
}

/// Writes one frame and nothing else.
pub fn write_one_frame(
    to: &mut impl Write,
    frame: &[u8],
) -> Result<(), WhyTheFrameCouldNotBeRead> {
    to.write_all(frame)
        .and_then(|()| to.flush())
        .map_err(|why| WhyTheFrameCouldNotBeRead::TheSystemRefusedIt(why.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn an_identity() -> Identity {
        Identity::issue(IdentityKind::Operation).expect("the system supplies random bytes")
    }

    #[test]
    fn one_request_goes_out_and_comes_back_the_same() {
        let identity = an_identity();
        let request = Request::speaking(
            THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS,
            identity.clone(),
            "readiness",
        )
        .carrying(Fields::none().and("what", "the phase"));
        let frame = write_request_frame(&request);
        let read = read_one_request(&mut frame.as_slice()).expect("it reads back");
        assert_eq!(read, request);
        assert_eq!(read.fields.text("what"), Some("the phase"));
    }

    #[test]
    fn one_result_goes_out_and_comes_back_the_same_and_correlates() {
        let request = Request::speaking(ProtocolVersion(1), an_identity(), "the health answer");
        let answered = Answered::to(&request, ProtocolVersion(1), "2026.09.04")
            .carrying(Fields::none().and("phase", "verified"));
        let frame = write_result_frame(&answered);
        let read = read_one_result(&mut frame.as_slice()).expect("it reads back");
        assert_eq!(read, answered);
        assert!(read.answers(&request));
    }

    /// **C2**: a result carrying another identity is not this request's answer,
    /// whatever order it arrived in.
    #[test]
    fn a_result_carrying_another_identity_answers_nothing() {
        let asked = Request::speaking(ProtocolVersion(1), an_identity(), "readiness");
        let somebody_elses = Request::speaking(ProtocolVersion(1), an_identity(), "readiness");
        let answered = Answered::to(&somebody_elses, ProtocolVersion(1), "2026.09.04");
        assert!(!answered.answers(&asked));
    }

    /// **§1.1's own sentence**: a non-UTF-8 filename is a value rather than
    /// syntax, so a field's value has to be able to hold one.
    #[test]
    fn a_field_value_carries_bytes_that_are_not_text() {
        let name = b"bad\xff\xfe.txt".to_vec();
        let request = Request::speaking(ProtocolVersion(1), an_identity(), "a reading")
            .carrying(Fields::none().and("path", name.clone()));
        let frame = write_request_frame(&request);
        let read = read_one_request(&mut frame.as_slice()).expect("it reads back");
        assert_eq!(read.fields.value("path"), Some(name.as_slice()));
        assert_eq!(read.fields.text("path"), None);
    }

    /// **P5.** A field this build does not know is kept and ignored rather than
    /// refusing the frame.
    #[test]
    fn a_field_nobody_knows_is_ignored_rather_than_refused() {
        let request = Request::speaking(ProtocolVersion(1), an_identity(), "readiness")
            .carrying(Fields::none().and("a field from a later version", "something"));
        let frame = write_request_frame(&request);
        let read = read_one_request(&mut frame.as_slice()).expect("it is not refused");
        assert_eq!(read.fields.value("something this build reads"), None);
        assert_eq!(read.fields.how_many(), 1);
    }

    /// **A2 and P5's forbidden kind.** A caller's own identity is dropped as the
    /// frame is read, and the frame is not refused — refusing would be checking.
    #[test]
    fn a_field_naming_the_caller_is_dropped_and_never_refused() {
        // Composed by hand, because this crate's own builder will not put one
        // on the wire either.
        let identity = an_identity();
        let mut body = Vec::new();
        body.extend_from_slice(&1u32.to_be_bytes());
        put_text(&mut body, &identity.as_text());
        put_text(&mut body, "readiness");
        put_length(&mut body, 2);
        put_text(&mut body, THE_FIELD_NAME_RESERVED_FOR_A_CALLERS_OWN_IDENTITY);
        put_bytes(&mut body, b"0");
        put_text(&mut body, "kept");
        put_bytes(&mut body, b"yes");
        let mut frame = Vec::new();
        put_length(&mut frame, body.len());
        frame.extend_from_slice(&body);

        let read = read_one_request(&mut frame.as_slice()).expect("it is not refused");
        assert_eq!(read.fields.how_many(), 1);
        assert_eq!(
            read.fields
                .value(THE_FIELD_NAME_RESERVED_FOR_A_CALLERS_OWN_IDENTITY),
            None
        );
        assert_eq!(read.fields.text("kept"), Some("yes"));
    }

    #[test]
    fn this_crate_will_not_put_a_callers_identity_on_the_wire_either() {
        let fields = Fields::none()
            .and(THE_FIELD_NAME_RESERVED_FOR_A_CALLERS_OWN_IDENTITY, "0")
            .and("kept", "yes");
        assert_eq!(fields.how_many(), 1);
        assert_eq!(fields.text("kept"), Some("yes"));
    }

    /// **A result that says *this package does not answer that request* says so
    /// on the wire and reads back as that**, rather than looking like an answer
    /// that found nothing.
    #[test]
    fn a_result_can_say_it_does_not_answer_that_request_and_it_survives_the_wire() {
        let request = Request::speaking(ProtocolVersion(1), an_identity(), "the machine-local queue");
        let answered = Answered::to(&request, ProtocolVersion(1), "2026.09.06")
            .saying_it_does_not_answer_that_request(&request.name);
        let frame = write_result_frame(&answered);
        let read = read_one_result(&mut frame.as_slice()).expect("it reads back");
        assert!(read.answers(&request));
        assert_eq!(
            read.the_request_name_it_says_it_does_not_answer(),
            Some("the machine-local queue")
        );
        assert_eq!(read.the_protocol_version_it_says_it_does_not_speak(), None);
    }

    /// **And the same for the version refusal**, which carries the version the
    /// request declared rather than the one the answering side speaks.
    #[test]
    fn a_result_can_say_it_does_not_speak_that_protocol_version() {
        let request = Request::speaking(ProtocolVersion(7), an_identity(), "readiness");
        let answered = Answered::to(&request, ProtocolVersion(1), "2026.09.06")
            .saying_it_does_not_speak_that_protocol_version(request.protocol_version);
        let frame = write_result_frame(&answered);
        let read = read_one_result(&mut frame.as_slice()).expect("it reads back");
        assert_eq!(
            read.the_protocol_version_it_says_it_does_not_speak(),
            Some(ProtocolVersion(7))
        );
        assert_eq!(read.protocol_version, ProtocolVersion(1));
        assert_eq!(read.the_request_name_it_says_it_does_not_answer(), None);
    }

    /// **An ordinary result says neither**, which is what makes either one worth
    /// reading.
    #[test]
    fn an_ordinary_result_says_neither_of_the_two_refusals() {
        let request = Request::speaking(ProtocolVersion(1), an_identity(), "the health answer");
        let answered = Answered::to(&request, ProtocolVersion(1), "2026.09.06")
            .carrying(Fields::none().and("phase", "verified"));
        assert_eq!(answered.the_request_name_it_says_it_does_not_answer(), None);
        assert_eq!(
            answered.the_protocol_version_it_says_it_does_not_speak(),
            None
        );
    }

    /// **P5 holds over the new names too**: a reader that does not know one
    /// ignores it rather than refusing the frame. Proved by reading a result
    /// carrying both reserved names and an unknown one, and finding all three
    /// still there.
    #[test]
    fn the_reserved_result_names_are_ordinary_fields_to_a_reader_that_ignores_them() {
        let request = Request::speaking(ProtocolVersion(1), an_identity(), "readiness");
        let answered = Answered::to(&request, ProtocolVersion(1), "2026.09.06").carrying(
            Fields::none()
                .and(
                    THE_FIELD_NAME_RESERVED_FOR_A_REQUEST_THIS_PACKAGE_DOES_NOT_ANSWER,
                    "readiness",
                )
                .and(
                    THE_FIELD_NAME_RESERVED_FOR_A_PROTOCOL_VERSION_THIS_PACKAGE_DOES_NOT_SPEAK,
                    "9",
                )
                .and("a field from a later version", "something"),
        );
        let frame = write_result_frame(&answered);
        let read = read_one_result(&mut frame.as_slice()).expect("it is not refused");
        assert_eq!(read.fields.how_many(), 3);
    }

    /// **The three reserved names are three**, so no two of them collide and a
    /// reader of one never reads another.
    #[test]
    fn no_two_reserved_field_names_are_the_same() {
        let names = [
            THE_FIELD_NAME_RESERVED_FOR_A_CALLERS_OWN_IDENTITY,
            THE_FIELD_NAME_RESERVED_FOR_A_REQUEST_THIS_PACKAGE_DOES_NOT_ANSWER,
            THE_FIELD_NAME_RESERVED_FOR_A_PROTOCOL_VERSION_THIS_PACKAGE_DOES_NOT_SPEAK,
        ];
        let mut sorted = names.to_vec();
        sorted.sort_unstable();
        sorted.dedup();
        assert_eq!(sorted.len(), names.len());
    }

    #[test]
    fn a_frame_that_declares_more_than_the_largest_is_refused_before_a_byte_is_taken() {
        let mut frame = Vec::new();
        put_length(&mut frame, THE_LARGEST_FRAME + 1);
        let why = read_one_request(&mut frame.as_slice()).expect_err("it is refused");
        assert_eq!(
            why,
            WhyTheFrameCouldNotBeRead::ItDeclaredMoreThanTheLargestFrame(THE_LARGEST_FRAME + 1)
        );
    }

    #[test]
    fn a_frame_that_stops_part_way_is_not_a_result() {
        let request = Request::speaking(ProtocolVersion(1), an_identity(), "readiness");
        let frame = write_request_frame(&request);
        let cut = &frame[..frame.len() - 3];
        let why = read_one_request(&mut &cut[..]).expect_err("it is refused");
        assert_eq!(why, WhyTheFrameCouldNotBeRead::ItEndedPartWayThrough);
    }

    #[test]
    fn a_length_running_past_the_frames_end_is_refused() {
        let mut body = Vec::new();
        body.extend_from_slice(&1u32.to_be_bytes());
        put_length(&mut body, 4096);
        let mut frame = Vec::new();
        put_length(&mut frame, body.len());
        frame.extend_from_slice(&body);
        let why = read_one_request(&mut frame.as_slice()).expect_err("it is refused");
        assert_eq!(why, WhyTheFrameCouldNotBeRead::ALengthRunsPastTheEnd);
    }

    /// **P1.** One frame is one request: a stream holding two is read one at a
    /// time and the first read takes exactly its own bytes.
    #[test]
    fn one_frame_is_one_request_and_the_second_is_still_there() {
        let first = Request::speaking(ProtocolVersion(1), an_identity(), "first");
        let second = Request::speaking(ProtocolVersion(1), an_identity(), "second");
        let mut both = write_request_frame(&first);
        both.extend_from_slice(&write_request_frame(&second));
        let mut stream = both.as_slice();
        assert_eq!(read_one_request(&mut stream).unwrap(), first);
        assert_eq!(read_one_request(&mut stream).unwrap(), second);
    }

    #[test]
    fn a_wider_or_skipped_interval_is_not_accepted() {
        assert!(DeclaredVersions::declaring(ProtocolVersion(5), Some(ProtocolVersion(4))).is_ok());
        assert_eq!(
            DeclaredVersions::declaring(ProtocolVersion(5), Some(ProtocolVersion(3))),
            Err(NotAnAcceptableInterval::ItIsNotTheSingleImmediatelyPreviousVersion)
        );
        assert_eq!(
            DeclaredVersions::declaring(ProtocolVersion(5), Some(ProtocolVersion(5))),
            Err(NotAnAcceptableInterval::ItIsNotTheSingleImmediatelyPreviousVersion)
        );
    }

    #[test]
    fn negotiation_takes_the_current_version_where_the_two_are_equal() {
        let five = DeclaredVersions::declaring(ProtocolVersion(5), Some(ProtocolVersion(4))).unwrap();
        assert_eq!(negotiate(&five, &five), Negotiated::Speak(ProtocolVersion(5)));
    }

    #[test]
    fn negotiation_takes_the_one_immediately_previous_version_where_that_is_the_intersection() {
        let newer =
            DeclaredVersions::declaring(ProtocolVersion(5), Some(ProtocolVersion(4))).unwrap();
        let older = DeclaredVersions::declaring(ProtocolVersion(4), None).unwrap();
        assert_eq!(negotiate(&newer, &older), Negotiated::Speak(ProtocolVersion(4)));
        assert_eq!(negotiate(&older, &newer), Negotiated::Speak(ProtocolVersion(4)));
    }

    /// **An empty intersection is incompatible, never a guessed downgrade.**
    #[test]
    fn an_empty_intersection_is_incompatible_and_never_a_guessed_downgrade() {
        let newer =
            DeclaredVersions::declaring(ProtocolVersion(9), Some(ProtocolVersion(8))).unwrap();
        let older =
            DeclaredVersions::declaring(ProtocolVersion(4), Some(ProtocolVersion(3))).unwrap();
        assert_eq!(negotiate(&newer, &older), Negotiated::Incompatible);
        assert_eq!(negotiate(&older, &newer), Negotiated::Incompatible);
    }

    /// **P3's own interval, read as the commit reads it.** The deferral rule
    /// itself lives at the commit, in `Rented-server runtime`; what this crate
    /// supplies it is which versions a candidate covers.
    #[test]
    fn a_candidate_covers_its_current_version_and_the_one_below_it_and_no_other() {
        let candidate =
            DeclaredVersions::declaring(ProtocolVersion(6), Some(ProtocolVersion(5))).unwrap();
        assert!(candidate.covers(ProtocolVersion(6)));
        assert!(candidate.covers(ProtocolVersion(5)));
        assert!(!candidate.covers(ProtocolVersion(4)));
        assert!(!candidate.covers(ProtocolVersion(7)));
        let alone = DeclaredVersions::declaring(ProtocolVersion(6), None).unwrap();
        assert!(alone.covers(ProtocolVersion(6)));
        assert!(!alone.covers(ProtocolVersion(5)));
    }

    #[test]
    fn a_caller_with_no_overlap_takes_the_retained_adapter_first() {
        assert_eq!(
            what_a_caller_does_with_no_overlap(true, true),
            WhatACallerDoesWithNoOverlap::UseTheRetainedAdapterForInspectionStopReleaseAndCollectionOnly
        );
        assert_eq!(
            what_a_caller_does_with_no_overlap(false, false),
            WhatACallerDoesWithNoOverlap::ReportTheSupervisorUnavailableWithoutMarkingTheConnectionDown
        );
        assert_eq!(
            what_a_caller_does_with_no_overlap(false, true),
            WhatACallerDoesWithNoOverlap::RepairFailsClosedNamingTheExactWorkIdentity
        );
    }

    /// **P4.** The repair route's framing takes no version from anything it may
    /// be replacing.
    #[test]
    fn the_repair_route_speaks_this_builds_own_version_and_takes_none() {
        let request = Request::for_the_bootstrap_and_repair_route(an_identity(), "repair");
        assert_eq!(
            request.protocol_version,
            THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS
        );
    }
}
