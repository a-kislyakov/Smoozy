//! **§2 performed — Q3 and Q5's two readings of a machine file.** The
//! four-value read the record fast path turns on, and the read-through by the
//! digest worker.
//!
//! **Neither is a reading of state** (Q6). Both are deliberate comparisons the
//! plan orders — the same shape as `Job files and outputs` 4.4's change test
//! and `Server protocol and transfers` S13's resume test — and a file found at
//! a location no record names, and not read through, is **never** *already on
//! the machine*.
//!
//! # The four-value read, and the channel that can answer it
//!
//! **Q5 route 1 names the channel and §12 item 19 records why**: the
//! file-transfer surface cannot supply it. Revision 3's attribute structure
//! carries `uint64 size`, `uint32 atime` and `uint32 mtime` and **no inode, no
//! file identifier and no change time** — and the change time is the one the
//! design leans on hardest, because a hand-restored modification time still
//! moves it and it cannot be set back. **So a builder reaching for the ordinary
//! listing would silently get two of the four and a coarser clock on one of
//! those, and the fast path would pass where it should not.** It is a
//! command-channel reading, which puts it on `File Browser` §13 item 9's
//! unestablished ground — **which programs answer such a reading on an
//! arbitrary rental** — and that is carried rather than repaired.
//!
//! **Where the channel cannot answer, the fast path simply does not fire and
//! the ordinary contents route runs**, which is the same shape as every other
//! missing capability. Nothing below claims anything from a reading that did
//! not come back.
//!
//! # The read-through, by the machine's own program — the 38's piece 7
//!
//! **Q3: on the machine the digest is computed by Smoozy's own package code,
//! running as a per-operation worker, and never by a tool found on the
//! machine.** Fact 15 measured one program at 0.46 s against another at 2.24 s
//! on the same box and the same file — but which programs an arbitrary rental
//! carries is established nowhere, and the package removes the question.
//!
//! **O2, settled 2026-10-03: the worker is a file-inspection worker whose group
//! is the worker alone.** [`the_machine_copys_contents`] asks the machine's
//! supervisor for `Server protocol and transfers` §1.1 item 1's *digest over a
//! byte range* over the whole file, asks again under the same identity until it
//! has a recorded outcome, and reads the digest out of the answer — the same
//! function, S23's, that the desktop's own digest is, so the two compare as one.
//!
//! **Until 2026-10-04 no worker of this kind existed**, and what stood in its
//! place answered *did not complete*: every row whose establishment wanted the
//! read-through was sent again — a file already at its mirror with no record
//! naming it, a placement whose four values moved, a *Confirm* past a moved
//! copy. **The answers stay three and stay honest**: *nothing there* only where
//! the machine's own system said no such file, and *did not complete* for
//! everything else, which [`crate::contents`] reads as establishing nothing.
//!
//! **Markers used in this file that belong to another document, each with the
//! document it belongs to.** `Server protocol and transfers` S13, S22, S23. `File
//! Browser` §13. `Job files and outputs` R10.

use smoozy_server_protocol::carriage::{self, a_command_naming};
use smoozy_server_protocol::framing::{Fields, Request, THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS};
use smoozy_server_protocol::requests as asked;
use smoozy_state::identities::{Identity, IdentityKind};
use smoozy_server_protocol::subsystem::client::Subsystem;
use smoozy_server_protocol::transfers::SourceReading;
use smoozy_server_protocol::transport::CommandString;
use smoozy_servers::readings::CommandChannel;
use tokio::io::{AsyncRead, AsyncWrite};

use crate::bytes::MachineLocation;
use crate::contents::ContentIdentity;
use crate::readings::{FourValuesNow, MachineContents};

/// **The carriage the four-value read names.** The location is machine-origin,
/// so it rides a link and the command names the token (A8).
pub const THE_FOUR_VALUE_SITE: smoozy_server_protocol::carriage::CarriageSite =
    smoozy_server_protocol::carriage::CarriageSite {
        link_path_on_the_subsystem: ".smoozy-carriage/dependency-four-values",
        named_in_a_command_as: "~/.smoozy-carriage/dependency-four-values",
    };

/// **Q5 route 1's four values, in one reading.**
///
/// **The link is followed rather than read**, because what the fast path is
/// about is the file the mirror location names and not the token that reaches
/// it.
///
/// **The order is size, inode, modification time, change time** — `Server
/// protocol and transfers` S13's own shape, and `Job files and outputs` R10's,
/// one shape rather than three.
///
/// # This command names one program's own flavour on nothing established
///
/// **PERMANENTLY UNRESOLVED.** `stat` with `-c` and a printf-style format is
/// **GNU coreutils'** form, and `%Z` is read as a change time on that
/// authority alone. **Nothing in this project establishes which `stat` a
/// rented machine carries, whether it takes `-c`, or what its fourth field
/// means** — that is `File Browser` §13 item 9's ground, which §12 item 19
/// rides on: *which programs answer such a reading on an arbitrary rental,
/// and whether their format holds*. This build picks one and says so; the
/// module's head says the shortfall *"is carried rather than repaired"*, and
/// this note is that carrying.
///
/// **Where the command refuses**, the reading answers
/// [`FourValuesNow::TheChannelCannotAnswer`], no fast path fires, and the
/// ordinary contents route runs (Q5 route 2). **What that costs is bandwidth
/// and nothing else** — the dependency is read through and compared, which is
/// the route the fast path exists only to skip.
///
/// **What breaks is the other shape: a `stat` that accepts these flags and
/// answers four numbers whose fourth is not a change time a hand-restored
/// modification time still moves.** The fast path then passes on a machine
/// copy that was altered after publication, the file is not sent, and the
/// render reads the altered copy — at full price, at a successful ending,
/// with nothing said.
///
/// **This is not a pending item and no space is left here or in the record
/// for an answer.** The Creator ruled on 2026-08-31 that there is no further
/// Blender research and no rental taken to answer a question, which closes
/// both routes to it. **Why the command stays**: Q5 route 1 is the plan's own
/// route, and without it every dependency would be read through on the
/// machine again on every check (route two, built at the 38's piece 7) — a
/// real cost for a harm nobody has established, and a re-decision of a plan
/// clause besides.
pub const READ_THE_FOUR_VALUES: CommandString =
    a_command_naming("stat -L -c '%s %i %Y %Z' ~/.smoozy-carriage/dependency-four-values");

/// **Q5 route 1, performed.** The values as they stand now, or the honest
/// nothing.
///
/// **Nothing here is inferred from a failure.** A reading that did not come
/// back, or came back in a shape this cannot read, is
/// [`FourValuesNow::TheChannelCannotAnswer`] — which fires no fast path and
/// claims nothing — and never *nothing there*, which is a different answer with
/// a different consequence (Q7 turns that one into *not found* and marks the
/// placement gone).
/// **The machine's six sites are held for the length of this act**
/// ([`super::blender::TheSitesHeld`]): this reading plants its own token at a
/// fixed name too, so a second act on the same machine would read the wrong
/// file's four values.
pub async fn read_the_four_values<C, S>(
    channel: &mut C,
    subsystem: &mut Subsystem<S>,
    _sites: &mut super::blender::TheSitesHeld<'_>,
    at: &MachineLocation,
) -> FourValuesNow
where
    C: CommandChannel,
    S: AsyncRead + AsyncWrite + Unpin + Send,
{
    let _ = subsystem
        .remove(THE_FOUR_VALUE_SITE.link_path_on_the_subsystem.as_bytes())
        .await;
    if carriage::plant(subsystem, THE_FOUR_VALUE_SITE, at.as_bytes())
        .await
        .is_err()
    {
        return FourValuesNow::TheChannelCannotAnswer;
    }
    let ran = channel.run(READ_THE_FOUR_VALUES).await;
    let _ = subsystem
        .remove(THE_FOUR_VALUE_SITE.link_path_on_the_subsystem.as_bytes())
        .await;

    let Ok(result) = ran else {
        return FourValuesNow::TheChannelCannotAnswer;
    };
    // **A run that did not end cleanly answers nothing at all.** It is not
    // *nothing there*: a refusal, a missing program and a file that is genuinely
    // absent are one shape on this surface, and the two answers have opposite
    // consequences — Q7 turns *nothing there* into *not found* and marks the
    // placement gone, which is a durable claim about the machine no failing
    // command may make.
    if !result.ended_cleanly() {
        return FourValuesNow::TheChannelCannotAnswer;
    }
    match result.output_as_text().and_then(the_four_values_in) {
        Some(reading) => FourValuesNow::Answered(reading),
        None => FourValuesNow::TheChannelCannotAnswer,
    }
}

/// The four values as the reading writes them, or nothing where what came back
/// is not four numbers.
///
/// **`Job files and outputs` N15's discipline: nothing here is line-oriented
/// over a listing.** This reads one reading's own four fields and refuses
/// anything else; it never splits a directory listing and never takes a field
/// count as a name.
pub fn the_four_values_in(text: &str) -> Option<SourceReading> {
    let mut fields = text.split_whitespace();
    let size = fields.next()?.parse::<u64>().ok()?;
    let inode = fields.next()?.to_string();
    let modification_time = fields.next()?.parse::<i64>().ok()?;
    let change_time = fields.next()?.parse::<i64>().ok()?;
    if fields.next().is_some() {
        return None;
    }
    Some(SourceReading {
        size,
        inode,
        modification_time,
        change_time,
    })
}

/// **Q3's contents reading, one operation on the machine** — the whole file at
/// a machine location, digested by the machine's own program.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OneDigest {
    /// **The operation's own identity**, issued before the request leaves
    /// (`Server protocol and transfers` C1).
    pub operation: Identity,
    pub at: MachineLocation,
}

impl OneDigest {
    pub fn of_the_whole(at: &MachineLocation) -> Option<OneDigest> {
        Some(OneDigest {
            operation: Identity::issue(IdentityKind::Operation).ok()?,
            at: at.clone(),
        })
    }

    /// **The one request**: the file by its path, as a protocol field and
    /// never a word of a command (A8), and the range from its start to its end.
    pub fn the_request(&self) -> Request {
        Request::speaking(
            THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS,
            self.operation.clone(),
            asked::THE_DIGEST_OVER_A_BYTE_RANGE_REQUEST,
        )
        .carrying(
            Fields::none()
                .and(asked::THE_DIGESTED_FILE_FIELD, self.at.as_bytes())
                .and(asked::THE_RANGE_STARTS_AT_FIELD, "0"),
        )
    }
}

/// **Q5 route 2's read-through, performed: the machine copy's contents, read by
/// the machine's own program.** *Nothing there* only where the machine said no
/// such file; *did not complete* for every other ending, a refusal, a program
/// placed before this request existed, or a digest that would not read back.
pub async fn the_machine_copys_contents<M>(machine: &mut M, at: &MachineLocation) -> MachineContents
where
    M: super::checking::TheMachineAReadingIsAskedOf,
{
    let Some(digest) = OneDigest::of_the_whole(at) else {
        return MachineContents::DidNotComplete;
    };
    let Ok(ended) = super::checking::asked_until_it_ends(
        machine,
        &digest.the_request(),
        super::blender::THE_TIME_LIMIT_A_BLENDER_RUN_CARRIES,
        super::checking::BETWEEN_TWO_ASKS_OF_ONE_READING,
        &mut || false,
    )
    .await
    else {
        return MachineContents::DidNotComplete;
    };
    what_the_digest_came_to(&ended.fields)
}

/// **The contents reading's answer, read** — the three outcomes and nothing
/// inferred between them.
pub fn what_the_digest_came_to(fields: &Fields) -> MachineContents {
    match fields.text(asked::THE_DIGESTS_OUTCOME_FIELD) {
        Some(asked::IT_WAS_DIGESTED) => {
            let size = fields
                .text(asked::THE_BYTES_DIGESTED_FIELD)
                .and_then(|text| text.parse::<u64>().ok());
            let digest = fields
                .text(asked::THE_DIGEST_FIELD)
                .and_then(smoozy_server_protocol::transfers::integrity::Digest::read_back);
            match (size, digest) {
                (Some(size), Some(digest)) => MachineContents::Read(ContentIdentity { size, digest }),
                _ => MachineContents::DidNotComplete,
            }
        }
        Some(asked::NOTHING_IS_THERE) => MachineContents::NothingThere,
        _ => MachineContents::DidNotComplete,
    }
}

/// **The content identity of what a transfer published, taken from the
/// transfer's own pass-through digest (Q1, `Server protocol and transfers`
/// S22).**
///
/// **No second read of a file that just crossed the wire is ever taken**, which
/// is Q1 in as many words, and this is the one function that composes a content
/// identity out of a transfer rather than out of a reading.
pub fn the_content_identity_a_transfer_published(
    size: u64,
    digest: smoozy_server_protocol::transfers::integrity::Digest,
) -> ContentIdentity {
    ContentIdentity { size, digest }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// **A8: the reading's command carries no value and names this module's own
    /// token**, and the token follows the link rather than reading it.
    #[test]
    fn the_four_value_reading_carries_no_value_and_follows_the_link() {
        let text = READ_THE_FOUR_VALUES.as_str();
        assert!(text.contains(THE_FOUR_VALUE_SITE.named_in_a_command_as));
        assert!(!text.contains('$') && !text.contains('`'));
        assert!(text.contains(" -L "), "the link is followed, not read");
    }

    /// **The four values are the four, in `Server protocol and transfers`
    /// S13's own order**, and a reading that is not four numbers is not a
    /// reading.
    #[test]
    fn four_numbers_are_a_reading_and_anything_else_is_not() {
        assert_eq!(
            the_four_values_in("4096 8391 1756000000 1756000001\n"),
            Some(SourceReading {
                size: 4096,
                inode: "8391".to_string(),
                modification_time: 1_756_000_000,
                change_time: 1_756_000_001,
            })
        );
        // Three fields, five fields, a word where a number belongs, and
        // nothing at all — none of them is a reading.
        assert_eq!(the_four_values_in("4096 8391 1756000000"), None);
        assert_eq!(the_four_values_in("4096 8391 1 2 3"), None);
        assert_eq!(the_four_values_in("stat: cannot stat"), None);
        assert_eq!(the_four_values_in(""), None);
    }

    /// **The 38's piece 7, item 27: the contents reading's three answers, read
    /// as three and nothing between them** — and the digest that comes back is
    /// the desktop's own digest of the same bytes, so Q5 compares one value
    /// with one value.
    #[test]
    fn the_digest_read_back_is_the_desktops_own_and_nothing_there_is_only_no_such_file() {
        use smoozy_server_protocol::transfers::integrity::Digest;
        let bytes = b"the same texture".to_vec();
        let digested = Fields::none()
            .and(asked::THE_DIGESTS_OUTCOME_FIELD, asked::IT_WAS_DIGESTED)
            .and(asked::THE_DIGEST_FIELD, Digest::of(&bytes).as_text())
            .and(asked::THE_BYTES_DIGESTED_FIELD, bytes.len().to_string());
        assert_eq!(
            what_the_digest_came_to(&digested),
            MachineContents::Read(ContentIdentity {
                size: bytes.len() as u64,
                digest: Digest::of(&bytes),
            })
        );
        assert_eq!(
            what_the_digest_came_to(
                &Fields::none().and(asked::THE_DIGESTS_OUTCOME_FIELD, asked::NOTHING_IS_THERE)
            ),
            MachineContents::NothingThere
        );
        for not_one in [
            Fields::none().and(asked::THE_DIGESTS_OUTCOME_FIELD, asked::IT_WAS_NOT_DIGESTED),
            Fields::none(),
            Fields::none()
                .and(asked::THE_DIGESTS_OUTCOME_FIELD, asked::IT_WAS_DIGESTED)
                .and(asked::THE_DIGEST_FIELD, "not a digest")
                .and(asked::THE_BYTES_DIGESTED_FIELD, "4"),
        ] {
            assert_eq!(what_the_digest_came_to(&not_one), MachineContents::DidNotComplete);
        }
    }

    /// **The request names the whole file, from its first byte, as the bytes
    /// its path is.**
    #[test]
    fn the_request_names_the_file_from_its_start() {
        let at = MachineLocation::composed(b"/root/assets/files/Users/a/\xfftex.png".to_vec());
        let digest = OneDigest::of_the_whole(&at).expect("randomness");
        let request = digest.the_request();
        assert_eq!(request.name, asked::THE_DIGEST_OVER_A_BYTE_RANGE_REQUEST);
        assert_eq!(request.identity, digest.operation);
        assert_eq!(request.fields.value(asked::THE_DIGESTED_FILE_FIELD), Some(at.as_bytes()));
        assert_eq!(request.fields.text(asked::THE_RANGE_STARTS_AT_FIELD), Some("0"));
        assert_eq!(request.fields.text(asked::THE_RANGE_LENGTH_FIELD), None);
    }
}
