//! What an operation on the file-transfer subsystem **answered**, as values.
//!
//! **Nothing here performs an operation.** These are the two values a result
//! carries, and they sit on this side of the split because
//! [`crate::free_space::WhyThereIsNoReading`] names one of them and the machine
//! composes a free-space reading of its own. The channel that produces them,
//! and every operation on it, is next door in `smoozy-server-protocol`, which
//! re-exports both from the places they have always been reached at —
//! `subsystem::StatusCode` and `subsystem::SubsystemFailure` — so nothing that
//! reads them had to move with them.

/// The status codes revision 3 defines, kept apart because **§1.3 turns on two
/// of them being different**.
///
/// Documented, from the SSH file-transfer protocol draft version 3, read
/// 2026-08-20: `SSH_FX_NO_SUCH_FILE 2`, *"returned when a reference is made to
/// a file which should exist but doesn't"*, and `SSH_FX_PERMISSION_DENIED 3`,
/// *"returned when the authenticated user does not have sufficient permissions
/// to perform the operation."*
///
/// **That separation is the whole reason a request needing the three listing
/// failures told apart is carried on the subsystem and not on the command
/// channel**, where a transport failure and a remote command's own failure both
/// arrive at 255 and only the accompanying text separates them.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StatusCode {
    Ok,
    EndOfFile,
    /// **`SSH_FX_NO_SUCH_FILE 2`** — the path is not there.
    NoSuchFile,
    /// **`SSH_FX_PERMISSION_DENIED 3`** — the login may not.
    PermissionDenied,
    Failure,
    BadMessage,
    NoConnection,
    ConnectionLost,
    OperationUnsupported,
    /// A number the draft does not define. **Nothing guesses what it means**;
    /// it is carried as itself.
    Other(u32),
}

impl StatusCode {
    pub fn from_number(number: u32) -> StatusCode {
        match number {
            0 => StatusCode::Ok,
            1 => StatusCode::EndOfFile,
            2 => StatusCode::NoSuchFile,
            3 => StatusCode::PermissionDenied,
            4 => StatusCode::Failure,
            5 => StatusCode::BadMessage,
            6 => StatusCode::NoConnection,
            7 => StatusCode::ConnectionLost,
            8 => StatusCode::OperationUnsupported,
            other => StatusCode::Other(other),
        }
    }

    /// **F5's shape, from the other side.** Whether the operation did what was
    /// asked. Nothing else about a status is read as a cause.
    pub fn is_ok(self) -> bool {
        matches!(self, StatusCode::Ok)
    }
}

/// Why an operation did not answer as asked.
///
/// **§1.3: no tool's own output is mined for a cause**, and nothing here parses
/// a message. A [`SubsystemFailure::TheServerRefused`] carries the protocol's
/// own status code, which is a number the draft defines rather than text
/// nobody specified.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SubsystemFailure {
    /// The channel is gone. **Every handle open on it is gone with it**, and
    /// the operation that held one re-opens rather than re-using.
    TheChannelIsGone,
    /// The server answered with a status. **The code is carried as itself** so
    /// that a caller separating absence from refusal reads two values.
    TheServerRefused(StatusCode),
    /// The server answered something this revision does not define at that
    /// point. Nothing is guessed from it.
    TheAnswerCouldNotBeRead,
    /// **§1.2: a capability that is not announced is never attempted.** The
    /// caller asked for one the connection does not carry, and this is *"a
    /// statement about the connection, not a failure of the work"*.
    ThatCapabilityWasNotAnnounced,
    /// **The server offered a revision this code's operations are not written
    /// against, and the number it offered is carried.**
    ///
    /// S13's ground is that revision **3** is *"the revision in play"* — every
    /// rental posed announced it, four boxes across three sellers — but *"that
    /// a server Smoozy has not yet met announces revision 3 is an expectation
    /// rather than an observation"*. **This is what happens where the
    /// expectation does not hold.** Every operation on the channel and every
    /// field of revision 3's attribute structure would otherwise be read out of
    /// a reply composed to another revision's shape, so the sizes and the times
    /// would come back wrong rather than come back refused.
    ///
    /// **The answer is §1.2's own, applied to the revision itself**: *"A
    /// capability that is not announced is never attempted and never guessed
    /// at. Where a request has no negotiated route, the caller reports it as
    /// unavailable against that connection — which is a statement about the
    /// connection, not a failure of the work — and the module that wanted it
    /// decides."* **The refusal is in the negotiation** because that is the one
    /// place a channel is made, and nothing downstream has to remember to ask.
    /// **It is raised by the negotiation and by nothing else**, because that is
    /// the only moment a revision is read.
    TheServerOfferedAnotherRevision(u32),
}

impl SubsystemFailure {
    /// **§1.3's separation, asked as one question.** Whether the server said
    /// the path is not there, as opposed to saying the login may not.
    pub fn is_the_path_not_being_there(&self) -> bool {
        matches!(
            self,
            SubsystemFailure::TheServerRefused(StatusCode::NoSuchFile)
        )
    }

    /// The other half of the same separation.
    pub fn is_the_login_not_being_allowed(&self) -> bool {
        matches!(
            self,
            SubsystemFailure::TheServerRefused(StatusCode::PermissionDenied)
        )
    }
    /// **What the file server answered, in standard software words** (Rule 9),
    /// and nothing more than the status the protocol carried — so no cause is
    /// claimed that the answer does not carry. Added 2026-09-21, step 10c: the
    /// placement's own messages put this type's debugging rendering behind
    /// *What the server said*. The same words `Server Setup`'s own reading of a
    /// file-server answer uses.
    pub fn in_words(&self) -> String {
        match self {
            SubsystemFailure::TheChannelIsGone => "the connection to the server ended".to_string(),
            SubsystemFailure::TheServerRefused(StatusCode::NoSuchFile) => {
                "the server said the file or folder is not there".to_string()
            }
            SubsystemFailure::TheServerRefused(StatusCode::PermissionDenied) => {
                "the server said permission was denied".to_string()
            }
            SubsystemFailure::TheServerRefused(_) => "the server refused it".to_string(),
            SubsystemFailure::TheAnswerCouldNotBeRead => {
                "the server's answer could not be read".to_string()
            }
            SubsystemFailure::ThatCapabilityWasNotAnnounced => {
                "the server does not offer that operation".to_string()
            }
            SubsystemFailure::TheServerOfferedAnotherRevision(revision) => format!(
                "the server's file transfer speaks revision {revision}, which Smoozy does not"
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn refusal_and_absence_are_two_answers() {
        // §1.3's whole supply to `File Browser`: a separation rather than a
        // limit, and no message anywhere may say the two cannot be told apart.
        let gone = SubsystemFailure::TheServerRefused(StatusCode::NoSuchFile);
        let refused = SubsystemFailure::TheServerRefused(StatusCode::PermissionDenied);
        assert!(gone.is_the_path_not_being_there());
        assert!(!gone.is_the_login_not_being_allowed());
        assert!(refused.is_the_login_not_being_allowed());
        assert!(!refused.is_the_path_not_being_there());
    }

    #[test]
    fn the_two_codes_the_separation_turns_on_are_the_drafts_own_numbers() {
        // 2 and 3, quoted at `StatusCode` from the draft itself. A number the
        // draft does not define is carried as itself and nothing guesses.
        assert_eq!(StatusCode::from_number(2), StatusCode::NoSuchFile);
        assert_eq!(StatusCode::from_number(3), StatusCode::PermissionDenied);
        assert_eq!(StatusCode::from_number(99), StatusCode::Other(99));
        assert!(StatusCode::from_number(0).is_ok());
        assert!(!StatusCode::from_number(4).is_ok());
    }
}
